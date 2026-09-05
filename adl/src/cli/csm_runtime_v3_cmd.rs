use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use adl_runtime_kernel::{
    activate_config_generation, active_generation_ref, provision_config_generation,
    provision_config_generation_in_store, validate_active_config_generation,
    ConfigGenerationIdentity, RuntimeInitConfig,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DEFAULT_LABEL: &str = "com.agentlogic.adl-runtime-v3";
const RUNTIME_GENERATION_RECEIPT_SCHEMA: &str = "adl.runtime_v3.install_generation.v1";
const RUNTIME_INIT_SCHEMA: &str = "adl.runtime_v3.init.v1";

#[derive(Debug)]
struct ServiceManagerDeadlineExceeded {
    timeout: Duration,
}

impl std::fmt::Display for ServiceManagerDeadlineExceeded {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "service-manager probe exceeded {} milliseconds",
            self.timeout.as_millis()
        )
    }
}

impl std::error::Error for ServiceManagerDeadlineExceeded {}

#[derive(Debug)]
struct ConvergenceDeadlineExceeded {
    stage: &'static str,
    timeout: Duration,
}

impl std::fmt::Display for ConvergenceDeadlineExceeded {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Runtime v3 convergence stage {} did not complete within {} milliseconds",
            self.stage,
            self.timeout.as_millis()
        )
    }
}

impl std::error::Error for ConvergenceDeadlineExceeded {}

#[derive(Debug, Deserialize)]
struct RuntimeGenerationReceipt {
    schema: String,
    generation: String,
    source_revision: String,
    platform: String,
    build_profile: String,
    runtime_init_schema: String,
    artifacts: std::collections::BTreeMap<String, RuntimeGenerationArtifact>,
}

#[derive(Debug, Deserialize)]
struct RuntimeGenerationArtifact {
    file: String,
    sha256: String,
}

#[derive(Debug, Clone)]
struct RuntimeV3ServiceArgs {
    init: PathBuf,
    candidate: Option<PathBuf>,
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    plist: Option<PathBuf>,
    label: String,
    json: bool,
}

#[derive(Debug, Serialize)]
struct RuntimeV3ServiceStatus {
    schema: &'static str,
    operation: &'static str,
    service_manager: &'static str,
    label: String,
    init: String,
    config_valid: bool,
    service_loaded: bool,
    listener_ready: bool,
    listener: String,
    runtime_instance_id: Option<String>,
    runtime_process_id: Option<u32>,
    guardian_process_id: Option<u32>,
    active_init_hash: Option<String>,
    config_generation: Option<String>,
    config_receipt_digest: Option<String>,
    observability_ready: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct RuntimeReadinessProbe {
    schema: String,
    ready: bool,
    lifecycle: String,
    observability_ready: bool,
    runtime_instance_id: String,
    runtime_process_id: u32,
    guardian_process_id: u32,
    active_init_hash: String,
    config_generation: String,
    config_receipt_digest: String,
}

pub(crate) fn real_runtime_v3_service(args: &[String]) -> Result<()> {
    let Some(operation) = args.first().map(String::as_str) else {
        return Err(anyhow!(
            "csm runtime-v3 requires subcommand: start | stop | status | reload"
        ));
    };
    if matches!(operation, "--help" | "-h" | "help") {
        println!("{}", usage());
        return Ok(());
    }
    let parsed = parse_args(&args[1..])?;
    match operation {
        "start" => start(&parsed),
        "reload" => reload(&parsed),
        "stop" => stop(&parsed),
        "status" => status(&parsed, "status"),
        other => Err(anyhow!(
            "unknown csm runtime-v3 subcommand '{other}' (expected start, stop, status, or reload)"
        )),
    }
}

fn parse_args(args: &[String]) -> Result<RuntimeV3ServiceArgs> {
    let mut init = std::env::var_os("ADL_RUNTIME_V3_INIT").map(PathBuf::from);
    let mut candidate = None;
    let mut plist = std::env::var_os("ADL_RUNTIME_V3_PLIST").map(PathBuf::from);
    let mut label = DEFAULT_LABEL.to_owned();
    let mut json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--init" => {
                index += 1;
                init = Some(PathBuf::from(
                    args.get(index)
                        .ok_or_else(|| anyhow!("--init requires a value"))?,
                ));
            }
            "--plist" => {
                index += 1;
                plist = Some(PathBuf::from(
                    args.get(index)
                        .ok_or_else(|| anyhow!("--plist requires a value"))?,
                ));
            }
            "--candidate" => {
                index += 1;
                candidate = Some(PathBuf::from(
                    args.get(index)
                        .ok_or_else(|| anyhow!("--candidate requires a value"))?,
                ));
            }
            "--label" => {
                index += 1;
                label = args
                    .get(index)
                    .ok_or_else(|| anyhow!("--label requires a value"))?
                    .to_owned();
            }
            "--json" => json = true,
            other => return Err(anyhow!("unknown csm runtime-v3 option: {other}")),
        }
        index += 1;
    }
    let init = init.ok_or_else(|| {
        anyhow!("csm runtime-v3 requires --init <runtime-init.toml> or ADL_RUNTIME_V3_INIT")
    })?;
    if !init.is_absolute() {
        return Err(anyhow!("csm runtime-v3 --init must be absolute"));
    }
    if candidate.as_ref().is_some_and(|path| !path.is_absolute()) {
        return Err(anyhow!("csm runtime-v3 --candidate must be absolute"));
    }
    if label.trim().is_empty() || label.contains(char::is_whitespace) {
        return Err(anyhow!(
            "csm runtime-v3 --label must be non-empty without whitespace"
        ));
    }
    Ok(RuntimeV3ServiceArgs {
        init,
        candidate,
        plist,
        label,
        json,
    })
}

fn validated_init(path: &Path) -> Result<RuntimeInitConfig> {
    let init = RuntimeInitConfig::load(Some(path.to_path_buf()))
        .with_context(|| format!("validate Runtime v3 init {}", path.display()))?;
    if !init.binaries.kernel_path.is_file() {
        return Err(anyhow!(
            "Runtime v3 kernel does not exist: {}",
            init.binaries.kernel_path.display()
        ));
    }
    Ok(init)
}

fn run_after_preflight<T>(
    preflight: impl FnOnce() -> Result<()>,
    service_mutation: impl FnOnce() -> Result<T>,
) -> Result<T> {
    preflight()?;
    service_mutation()
}

fn validate_runtime_generation(init: &RuntimeInitConfig) -> Result<String> {
    validate_runtime_generation_with_service_binary(init, &std::env::current_exe()?)
}

fn validate_runtime_generation_with_service_binary(
    init: &RuntimeInitConfig,
    service_binary: &Path,
) -> Result<String> {
    let kernel_path = &init.binaries.kernel_path;
    let Some(bin_dir) = kernel_path.parent() else {
        return Err(anyhow!("Runtime v3 kernel path has no bin directory"));
    };
    let Some(current) = bin_dir.parent() else {
        return Err(anyhow!(
            "Runtime v3 kernel path has no installation generation"
        ));
    };
    if current.file_name().and_then(|name| name.to_str()) != Some("current") {
        return Err(anyhow!(
            "Runtime v3 service init must resolve its kernel through current/bin"
        ));
    }
    let metadata = fs::symlink_metadata(current).with_context(|| {
        format!(
            "inspect Runtime v3 current generation {}",
            current.display()
        )
    })?;
    if !metadata.file_type().is_symlink() {
        return Err(anyhow!(
            "Runtime v3 current generation is not an atomic symlink: {}",
            current.display()
        ));
    }
    let generation = current.canonicalize().with_context(|| {
        format!(
            "resolve Runtime v3 current generation {}",
            current.display()
        )
    })?;
    let generation_name = generation
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Runtime v3 generation directory name is invalid"))?;
    let generations = current
        .parent()
        .ok_or_else(|| anyhow!("Runtime v3 current generation has no install root"))?
        .join("generations")
        .canonicalize()
        .context("resolve Runtime v3 generations directory")?;
    if generation.parent() != Some(generations.as_path()) {
        return Err(anyhow!(
            "Runtime v3 current generation escapes generations directory"
        ));
    }
    let receipt_path = generation.join("receipt.json");
    let receipt: RuntimeGenerationReceipt =
        serde_json::from_slice(&fs::read(&receipt_path).with_context(|| {
            format!(
                "read Runtime v3 generation receipt {}",
                receipt_path.display()
            )
        })?)
        .with_context(|| {
            format!(
                "parse Runtime v3 generation receipt {}",
                receipt_path.display()
            )
        })?;
    if receipt.schema != RUNTIME_GENERATION_RECEIPT_SCHEMA
        || receipt.generation != generation_name
        || receipt.runtime_init_schema != RUNTIME_INIT_SCHEMA
        || receipt.source_revision.trim().is_empty()
        || receipt.platform != format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
        || receipt.build_profile.trim().is_empty()
    {
        return Err(anyhow!(
            "Runtime v3 generation receipt identity or compatibility is invalid"
        ));
    }
    let expected = [
        ("csm", "csm"),
        ("guardian", "adl-runtime-guardian"),
        ("kernel", "adl-runtime-kernel"),
    ];
    if receipt.artifacts.len() != expected.len() {
        return Err(anyhow!(
            "Runtime v3 generation receipt artifact set is incomplete"
        ));
    }
    for (key, filename) in expected {
        let artifact = receipt
            .artifacts
            .get(key)
            .ok_or_else(|| anyhow!("Runtime v3 generation receipt is missing {key}"))?;
        if artifact.file != format!("bin/{filename}") {
            return Err(anyhow!(
                "Runtime v3 generation receipt path mismatch for {key}"
            ));
        }
        let path = generation.join(&artifact.file);
        let metadata = fs::symlink_metadata(&path).with_context(|| {
            format!("inspect Runtime v3 generation artifact {}", path.display())
        })?;
        if !metadata.file_type().is_file() {
            return Err(anyhow!(
                "Runtime v3 generation artifact is missing: {}",
                path.display()
            ));
        }
        #[cfg(unix)]
        if metadata.permissions().mode() & 0o111 == 0 {
            return Err(anyhow!(
                "Runtime v3 generation artifact is not executable: {}",
                path.display()
            ));
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("hash Runtime v3 generation artifact {}", path.display()))?;
        let actual = format!("{:x}", Sha256::digest(bytes));
        if actual != artifact.sha256 {
            return Err(anyhow!(
                "Runtime v3 generation artifact hash mismatch: {filename}"
            ));
        }
    }
    let expected_kernel = generation.join("bin/adl-runtime-kernel").canonicalize()?;
    if kernel_path.canonicalize()? != expected_kernel {
        return Err(anyhow!(
            "Runtime v3 init kernel does not belong to the current generation"
        ));
    }
    let expected_csm = generation.join("bin/csm").canonicalize()?;
    if service_binary.canonicalize()? != expected_csm {
        return Err(anyhow!(
            "Runtime v3 service control does not belong to the current generation"
        ));
    }
    Ok(generation_name.to_owned())
}

fn prepare_active_config_generation(
    init_path: &Path,
    binary_generation: &str,
) -> Result<ConfigGenerationIdentity> {
    let identity =
        provision_config_generation(init_path, binary_generation).map_err(anyhow::Error::msg)?;
    let active_ref = active_generation_ref(init_path).map_err(anyhow::Error::msg)?;
    if !active_ref.exists() {
        activate_config_generation(init_path, &identity).map_err(anyhow::Error::msg)?;
    }
    validate_active_config_generation(init_path, binary_generation).map_err(anyhow::Error::msg)
}

#[cfg(target_os = "macos")]
fn validate_runtime_service_definition(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
) -> Result<()> {
    let kernel = &init.binaries.kernel_path;
    let Some(current) = kernel.parent().and_then(Path::parent) else {
        return Err(anyhow!("Runtime v3 launchd init has no current generation"));
    };
    if current.file_name().and_then(|name| name.to_str()) != Some("current") {
        return Err(anyhow!(
            "Runtime v3 launchd init must resolve through current/bin"
        ));
    }
    let plist = match args.plist.as_ref() {
        Some(source) => source.clone(),
        None => installed_launchd_plist(args)?,
    };
    let expected_guardian = current.join("bin/adl-runtime-guardian");
    let executable = launchd_program_executable(&plist)?;
    if executable != expected_guardian {
        return Err(anyhow!(
            "Runtime v3 launchd Guardian does not resolve through the current generation"
        ));
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn launchd_program_executable(plist: &Path) -> Result<PathBuf> {
    let output = Command::new("/usr/bin/plutil")
        .args(["-extract", "ProgramArguments.0", "raw", "-o", "-"])
        .arg(plist)
        .output()
        .with_context(|| format!("parse Runtime v3 launchd definition {}", plist.display()))?;
    if !output.status.success() {
        return Err(anyhow!("Runtime v3 launchd ProgramArguments is invalid"));
    }
    let executable = String::from_utf8(output.stdout)
        .context("decode Runtime v3 launchd executable")?
        .trim()
        .to_owned();
    if executable.is_empty() || !Path::new(&executable).is_absolute() {
        return Err(anyhow!("Runtime v3 launchd executable is invalid"));
    }
    Ok(PathBuf::from(executable))
}

#[cfg(target_os = "linux")]
fn validate_runtime_service_definition(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
) -> Result<()> {
    let kernel = &init.binaries.kernel_path;
    let Some(current) = kernel.parent().and_then(Path::parent) else {
        return Err(anyhow!("Runtime v3 systemd init has no current generation"));
    };
    if current.file_name().and_then(|name| name.to_str()) != Some("current") {
        return Err(anyhow!(
            "Runtime v3 systemd init must resolve through current/bin"
        ));
    }
    let output = Command::new("systemctl")
        .args([
            "show",
            &systemd_unit(args),
            "--property=ExecStart",
            "--value",
        ])
        .output()
        .context("inspect Runtime v3 systemd definition")?;
    if !output.status.success() {
        return Err(anyhow!("Runtime v3 systemd definition is unavailable"));
    }
    let definition =
        String::from_utf8(output.stdout).context("decode Runtime v3 systemd definition")?;
    let expected_guardian = current.join("bin/adl-runtime-guardian");
    let executable = systemd_exec_start_executable(&definition)?;
    if executable != expected_guardian {
        return Err(anyhow!(
            "Runtime v3 systemd Guardian does not resolve through the current generation"
        ));
    }
    Ok(())
}

#[cfg(any(target_os = "linux", test))]
fn systemd_exec_start_executable(definition: &str) -> Result<PathBuf> {
    let trimmed = definition.trim();
    let executable = if let Some(after_path) = trimmed.strip_prefix("{ path=") {
        after_path
            .split_once(" ;")
            .map(|(value, _)| value.trim())
            .unwrap_or_else(|| after_path.split_whitespace().next().unwrap_or_default())
    } else {
        trimmed
            .trim_start_matches('{')
            .split_whitespace()
            .next()
            .unwrap_or_default()
    };
    if executable.is_empty() || !Path::new(executable).is_absolute() {
        return Err(anyhow!(
            "Runtime v3 systemd ExecStart executable is invalid"
        ));
    }
    Ok(PathBuf::from(executable))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn validate_runtime_service_definition(
    _args: &RuntimeV3ServiceArgs,
    _init: &RuntimeInitConfig,
) -> Result<()> {
    Err(anyhow!(
        "Runtime v3 service control supports launchd and systemd"
    ))
}

fn start(args: &RuntimeV3ServiceArgs) -> Result<()> {
    if args.candidate.is_some() {
        return Err(anyhow!("--candidate is valid only with runtime-v3 reload"));
    }
    reconcile_interrupted_reload(args)?;
    let init = validated_init(&args.init)?;
    run_after_preflight(
        || {
            let binary_generation = validate_runtime_generation(&init)?;
            prepare_active_config_generation(&args.init, &binary_generation)?;
            validate_runtime_service_definition(args, &init)
        },
        || {
            if owned_runtime_readiness(args, &init).is_ok() {
                return emit_status(args, &init, "start", true);
            }
            start_clean(args, &init)?;
            emit_status(args, &init, "start", true)
        },
    )
}

fn reload(args: &RuntimeV3ServiceArgs) -> Result<()> {
    reconcile_interrupted_reload(args)?;
    let current = validated_init(&args.init)?;
    let candidate = args
        .candidate
        .as_ref()
        .map(|path| validated_init(path))
        .transpose()?;
    let binary_generation = validate_runtime_generation(&current)?;
    prepare_active_config_generation(&args.init, &binary_generation)?;
    let candidate_identity = if let Some(candidate) = candidate.as_ref() {
        let candidate_binary_generation = validate_runtime_generation(candidate)?;
        Some(
            provision_config_generation_in_store(
                args.candidate.as_ref().expect("candidate path"),
                &args.init,
                &candidate_binary_generation,
            )
            .map_err(anyhow::Error::msg)?,
        )
    } else {
        None
    };
    run_after_preflight(
        || validate_runtime_service_definition(args, &current),
        || {
            let Some((candidate_path, candidate)) = args.candidate.as_ref().zip(candidate.as_ref())
            else {
                start_clean(args, &current)?;
                return emit_status(args, &current, "reload", true);
            };
            reload_candidate_transaction(
                &args.init,
                candidate_path,
                candidate_identity.as_ref().expect("candidate identity"),
                || stop_and_wait(args, &current),
                || start_and_wait(args, candidate),
                || stop_and_wait(args, candidate),
                || start_and_wait(args, &current),
            )?;
            emit_status(args, candidate, "reload", true)
        },
    )
}

fn reload_candidate_transaction(
    active: &Path,
    candidate_path: &Path,
    candidate_identity: &ConfigGenerationIdentity,
    stop_current: impl FnOnce() -> Result<()>,
    start_candidate: impl FnOnce() -> Result<()>,
    stop_candidate: impl FnOnce() -> Result<()>,
    start_current: impl FnOnce() -> Result<()>,
) -> Result<()> {
    stop_current()?;
    let backup = match replace_config_with_candidate(active, candidate_path, candidate_identity) {
        Ok(backup) => backup,
        Err(error) => {
            start_current().context("Runtime v3 did not recover after candidate install failed")?;
            return Err(error);
        }
    };
    if let Err(reload_error) = start_candidate() {
        stop_candidate().context("stop failed Runtime v3 candidate before config rollback")?;
        restore_last_known_good(active, &backup).with_context(|| {
            format!(
                "restore last-known-good Runtime v3 init {}",
                active.display()
            )
        })?;
        start_current().context("Runtime v3 did not recover after config rollback")?;
        return Err(anyhow!(
            "Runtime v3 candidate reload failed and last-known-good configuration was restored: {reload_error}"
        ));
    }
    commit_candidate(active, &backup)
}

fn replace_config_with_candidate(
    active: &Path,
    candidate: &Path,
    candidate_identity: &ConfigGenerationIdentity,
) -> Result<PathBuf> {
    if active == candidate {
        return Err(anyhow!(
            "Runtime v3 reload candidate must differ from the active init path"
        ));
    }
    let parent = active
        .parent()
        .ok_or_else(|| anyhow!("Runtime v3 active init has no parent directory"))?;
    let file_name = active
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Runtime v3 active init filename is invalid"))?;
    let backup = parent.join(format!(".{file_name}.last-known-good"));
    let staged = parent.join(format!(".{file_name}.candidate"));
    let active_ref = active_generation_ref(active).map_err(anyhow::Error::msg)?;
    let backup_ref = active_ref.with_extension("active-generation.last-known-good");
    if backup.exists() || staged.exists() || backup_ref.exists() {
        return Err(anyhow!(
            "Runtime v3 reload transaction already exists; run start to reconcile it"
        ));
    }
    copy_create_new(active, &backup).with_context(|| {
        format!(
            "retain last-known-good Runtime v3 init {}",
            active.display()
        )
    })?;
    copy_create_new(&active_ref, &backup_ref)
        .context("retain last-known-good Runtime configuration generation reference")?;
    sync_parent(active)?;
    if let Err(error) = copy_create_new(candidate, &staged)
        .and_then(|_| fs::rename(&staged, active))
        .map_err(anyhow::Error::from)
        .and_then(|_| sync_parent(active))
        .and_then(|_| {
            activate_config_generation(active, candidate_identity).map_err(anyhow::Error::msg)
        })
    {
        let _ = restore_last_known_good(active, &backup);
        return Err(error).with_context(|| {
            format!(
                "atomically install Runtime v3 candidate {}",
                candidate.display()
            )
        });
    }
    Ok(backup)
}

fn copy_create_new(source: &Path, destination: &Path) -> io::Result<()> {
    let mut created = false;
    let result = (|| {
        let mut input = fs::File::open(source)?;
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)?;
        created = true;
        io::copy(&mut input, &mut output)?;
        output.sync_all()
    })();
    if result.is_err() && created {
        let _ = fs::remove_file(destination);
    }
    result
}

fn reload_transaction_paths(active: &Path) -> Result<(PathBuf, PathBuf)> {
    let parent = active
        .parent()
        .ok_or_else(|| anyhow!("Runtime v3 active init has no parent directory"))?;
    let file_name = active
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Runtime v3 active init filename is invalid"))?;
    Ok((
        parent.join(format!(".{file_name}.last-known-good")),
        parent.join(format!(".{file_name}.candidate")),
    ))
}

fn reconcile_interrupted_reload(args: &RuntimeV3ServiceArgs) -> Result<()> {
    let active = &args.init;
    let (backup, _) = reload_transaction_paths(active)?;
    let recovery_convergence = if backup.exists() {
        validated_init(&backup)?.service_convergence
    } else {
        Default::default()
    };
    reconcile_interrupted_reload_with(
        active,
        |backup| validated_init(backup).map(|_| ()),
        || {
            validated_init(active)
                .and_then(|init| owned_runtime_readiness(args, &init).map(|_| ()))
                .is_ok()
        },
        || {
            if let Ok(init) = validated_init(active) {
                stop_and_wait_with_timeout(
                    args,
                    &init,
                    Duration::from_millis(recovery_convergence.stop_timeout_millis),
                )?;
            } else {
                let timeout = Duration::from_millis(recovery_convergence.unload_timeout_millis);
                let deadline = std::time::Instant::now() + timeout;
                platform_stop_with_timeout(args, timeout)
                    .map_err(|error| normalize_stage_timeout(error, "unload", timeout))?;
                let remaining = deadline.saturating_duration_since(std::time::Instant::now());
                if remaining.is_zero() {
                    return convergence_timeout("unload", timeout);
                }
                wait_for_service_unloaded(args, remaining)
                    .map_err(|error| normalize_stage_timeout(error, "unload", timeout))?;
            }
            Ok(())
        },
    )
}

fn reconcile_interrupted_reload_with(
    active: &Path,
    validate_backup: impl FnOnce(&Path) -> Result<()>,
    candidate_is_running: impl FnOnce() -> bool,
    stop_running: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let (backup, staged) = reload_transaction_paths(active)?;
    if !backup.exists() {
        if staged.exists() {
            return Err(anyhow!(
                "Runtime v3 reload has an ambiguous staged candidate without a last-known-good configuration: {}",
                staged.display()
            ));
        }
        return Ok(());
    }
    validate_backup(&backup).context("validate interrupted Runtime v3 last-known-good config")?;
    if candidate_is_running() {
        return commit_candidate(active, &backup);
    }
    stop_running().context("stop interrupted Runtime v3 candidate before rollback")?;
    restore_last_known_good(active, &backup)?;
    if staged.exists() {
        fs::remove_file(&staged).with_context(|| {
            format!(
                "remove interrupted Runtime v3 candidate {}",
                staged.display()
            )
        })?;
    }
    sync_parent(active)
}

fn commit_candidate(active: &Path, backup: &Path) -> Result<()> {
    let (_, staged) = reload_transaction_paths(active)?;
    fs::remove_file(backup)
        .with_context(|| format!("remove Runtime v3 reload backup {}", backup.display()))?;
    let active_ref = active_generation_ref(active).map_err(anyhow::Error::msg)?;
    let backup_ref = active_ref.with_extension("active-generation.last-known-good");
    if backup_ref.exists() {
        fs::remove_file(&backup_ref)
            .context("remove Runtime v3 committed configuration generation reference backup")?;
    }
    if staged.exists() {
        fs::remove_file(&staged).with_context(|| {
            format!("remove Runtime v3 committed candidate {}", staged.display())
        })?;
    }
    sync_parent(active)
}

fn restore_last_known_good(active: &Path, backup: &Path) -> Result<()> {
    fs::rename(backup, active).with_context(|| {
        format!(
            "restore Runtime v3 last-known-good config {}",
            backup.display()
        )
    })?;
    let active_ref = active_generation_ref(active).map_err(anyhow::Error::msg)?;
    let backup_ref = active_ref.with_extension("active-generation.last-known-good");
    fs::rename(&backup_ref, &active_ref)
        .context("restore last-known-good Runtime configuration generation reference")?;
    sync_parent(active)
}

fn sync_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Runtime v3 config has no parent directory"))?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .with_context(|| format!("sync Runtime v3 config directory {}", parent.display()))
}

fn stop(args: &RuntimeV3ServiceArgs) -> Result<()> {
    let init = validated_init(&args.init)?;
    stop_and_wait_with_timeout(
        args,
        &init,
        Duration::from_millis(init.service_convergence.stop_timeout_millis),
    )?;
    emit_status(args, &init, "stop", false)
}

fn start_clean(args: &RuntimeV3ServiceArgs, init: &RuntimeInitConfig) -> Result<()> {
    stop_and_wait(args, init)?;
    start_and_wait(args, init)
}

fn stop_and_wait(args: &RuntimeV3ServiceArgs, current: &RuntimeInitConfig) -> Result<()> {
    stop_and_wait_with_timeout(
        args,
        current,
        Duration::from_millis(current.service_convergence.stop_timeout_millis),
    )
}

fn stop_and_wait_with_timeout(
    args: &RuntimeV3ServiceArgs,
    current: &RuntimeInitConfig,
    timeout: Duration,
) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    let guardian_process_id = platform_process_id_with_timeout(args, timeout)
        .map_err(|error| normalize_stage_timeout(error, "stop", timeout))?;
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return convergence_timeout("stop", timeout);
    }
    platform_stop_with_timeout(args, remaining)
        .map_err(|error| normalize_stage_timeout(error, "stop", timeout))?;
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return convergence_timeout("stop", timeout);
    }
    wait_for_stopped(args, current, guardian_process_id, remaining)
        .map_err(|error| normalize_stage_timeout(error, "stop", timeout))
}

fn start_and_wait(args: &RuntimeV3ServiceArgs, next: &RuntimeInitConfig) -> Result<()> {
    let timeout = Duration::from_millis(next.service_convergence.listener_timeout_millis);
    let deadline = std::time::Instant::now() + timeout;
    platform_start_with_timeout(args, timeout)
        .map_err(|error| normalize_stage_timeout(error, "listener", timeout))?;
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return convergence_timeout("listener", timeout);
    }
    wait_for_listener_open(next, remaining)
        .map_err(|error| normalize_stage_timeout(error, "listener", timeout))?;
    wait_for_readiness(
        args,
        next,
        Duration::from_millis(next.service_convergence.readiness_timeout_millis),
    )
}

fn status(args: &RuntimeV3ServiceArgs, operation: &'static str) -> Result<()> {
    let (backup, staged) = reload_transaction_paths(&args.init)?;
    if backup.exists() || staged.exists() {
        return Err(anyhow!(
            "Runtime v3 reload transaction is incomplete; run start to restore last-known-good configuration"
        ));
    }
    let init = validated_init(&args.init)?;
    validate_runtime_generation(&init)?;
    validate_runtime_service_definition(args, &init)?;
    let loaded = platform_loaded_with_timeout(
        args,
        Duration::from_millis(init.service_convergence.listener_timeout_millis),
    )
    .map_err(|error| {
        normalize_stage_timeout(
            error,
            "listener",
            Duration::from_millis(init.service_convergence.listener_timeout_millis),
        )
    })?;
    let ready = owned_runtime_readiness(args, &init).is_ok();
    emit_status(args, &init, operation, loaded && ready)?;
    if !loaded || !ready {
        return Err(anyhow!(
            "Runtime v3 is not ready: service_loaded={loaded} listener_ready={ready}"
        ));
    }
    Ok(())
}

fn emit_status(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    operation: &'static str,
    expected_ready: bool,
) -> Result<()> {
    let timeout = if operation == "stop" {
        Duration::from_millis(init.service_convergence.stop_timeout_millis)
    } else {
        Duration::from_millis(init.service_convergence.listener_timeout_millis)
    };
    let stage = if operation == "stop" {
        "stop"
    } else {
        "listener"
    };
    let loaded = platform_loaded_with_timeout(args, timeout)
        .map_err(|error| normalize_stage_timeout(error, stage, timeout))?;
    let readiness = owned_runtime_readiness(args, init).ok();
    let ready = readiness.is_some();
    let report = RuntimeV3ServiceStatus {
        schema: "adl.csm.runtime_v3_service_status.v1",
        operation,
        service_manager: platform_name(),
        label: args.label.clone(),
        init: args.init.display().to_string(),
        config_valid: true,
        service_loaded: loaded,
        listener_ready: ready,
        listener: init.api.address.clone(),
        runtime_instance_id: readiness
            .as_ref()
            .map(|health| health.runtime_instance_id.clone()),
        runtime_process_id: readiness.as_ref().map(|health| health.runtime_process_id),
        guardian_process_id: readiness.as_ref().map(|health| health.guardian_process_id),
        active_init_hash: readiness
            .as_ref()
            .map(|health| health.active_init_hash.clone()),
        config_generation: readiness
            .as_ref()
            .map(|health| health.config_generation.clone()),
        config_receipt_digest: readiness
            .as_ref()
            .map(|health| health.config_receipt_digest.clone()),
        observability_ready: readiness
            .as_ref()
            .is_some_and(|health| health.observability_ready),
    };
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "CSM_RUNTIME_V3 operation={} loaded={} ready={} listener={}",
            operation, loaded, ready, report.listener
        );
    }
    if expected_ready && (!loaded || !ready) {
        return Err(anyhow!("Runtime v3 service did not converge to ready"));
    }
    Ok(())
}

fn owned_runtime_readiness(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
) -> Result<RuntimeReadinessProbe> {
    owned_runtime_readiness_with_timeout(args, init, Duration::from_millis(750))
}

fn owned_runtime_readiness_with_timeout(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    request_timeout: Duration,
) -> Result<RuntimeReadinessProbe> {
    let deadline = std::time::Instant::now() + request_timeout;
    let service_process_id = platform_process_id_with_timeout(args, request_timeout)?
        .ok_or_else(|| anyhow!("Runtime v3 service manager has no live process identity"))?;
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return Err(anyhow!(
            "Runtime v3 service-manager identity probe exhausted readiness budget"
        ));
    }
    let readiness = runtime_readiness_with_timeout(init, remaining)?;
    let active_init_hash = file_hash(&args.init)?;
    let binary_generation = validate_runtime_generation(init)?;
    let active_config_generation =
        validate_active_config_generation(&args.init, &binary_generation)
            .map_err(anyhow::Error::msg)?;
    validate_owned_readiness(
        service_process_id,
        &active_init_hash,
        &active_config_generation,
        &readiness,
    )?;
    Ok(readiness)
}

fn validate_owned_readiness(
    service_process_id: u32,
    active_init_hash: &str,
    active_config_generation: &ConfigGenerationIdentity,
    readiness: &RuntimeReadinessProbe,
) -> Result<()> {
    if readiness.guardian_process_id != service_process_id {
        return Err(anyhow!(
            "Runtime v3 readiness belongs to Guardian process {} but service manager owns {}",
            readiness.guardian_process_id,
            service_process_id
        ));
    }
    if readiness.active_init_hash != active_init_hash {
        return Err(anyhow!(
            "Runtime v3 readiness config identity does not match active init"
        ));
    }
    if readiness.config_generation != active_config_generation.generation
        || readiness.config_receipt_digest != active_config_generation.receipt_digest
    {
        return Err(anyhow!(
            "Runtime v3 readiness config generation does not match active receipt"
        ));
    }
    Ok(())
}

fn runtime_readiness_with_timeout(
    init: &RuntimeInitConfig,
    request_timeout: Duration,
) -> Result<RuntimeReadinessProbe> {
    let addresses = init
        .socket_addrs()
        .context("resolve Runtime v3 API address")?;
    let address = addresses
        .first()
        .copied()
        .ok_or_else(|| anyhow!("Runtime v3 API address resolved to no endpoints"))?;
    let roots = fs::read(&init.api.tls.trust_roots_path).with_context(|| {
        format!(
            "read Runtime v3 trust roots {}",
            init.api.tls.trust_roots_path.display()
        )
    })?;
    let certificates =
        reqwest::Certificate::from_pem_bundle(&roots).context("parse Runtime v3 trust roots")?;
    let mut builder = reqwest::blocking::Client::builder()
        .timeout(request_timeout)
        .tls_built_in_root_certs(false)
        .resolve(&init.api.tls.server_name, address);
    for certificate in certificates {
        builder = builder.add_root_certificate(certificate);
    }
    let client = builder
        .build()
        .context("build Runtime v3 readiness client")?;
    let endpoint = format!(
        "https://{}:{}/v1/ready",
        init.api.tls.server_name,
        address.port()
    );
    let response = client
        .get(&endpoint)
        .send()
        .with_context(|| format!("query Runtime v3 readiness at {endpoint}"))?
        .error_for_status()
        .context("Runtime v3 readiness endpoint rejected the request")?;
    let readiness: RuntimeReadinessProbe = response
        .json()
        .context("decode Runtime v3 readiness response")?;
    validate_readiness_probe(&readiness)?;
    Ok(readiness)
}

fn validate_readiness_probe(readiness: &RuntimeReadinessProbe) -> Result<()> {
    if readiness.schema != "adl.runtime_v3.readiness.v1"
        || !readiness.ready
        || readiness.lifecycle != "running"
        || !readiness.observability_ready
        || readiness.runtime_instance_id.trim().is_empty()
        || readiness.runtime_process_id == 0
        || readiness.guardian_process_id == 0
        || !is_blake3_hex(&readiness.active_init_hash)
        || !is_blake3_hex(&readiness.config_generation)
        || !is_blake3_hex(&readiness.config_receipt_digest)
    {
        return Err(anyhow!("Runtime v3 readiness response is not healthy"));
    }
    Ok(())
}

fn wait_for_listener_open(init: &RuntimeInitConfig, timeout: Duration) -> Result<()> {
    let addresses = init
        .socket_addrs()
        .context("resolve Runtime v3 listener address")?;
    wait_for_convergence("listener", timeout, |remaining| {
        let attempt_deadline = std::time::Instant::now() + remaining;
        for address in &addresses {
            let remaining = attempt_deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                return Ok(false);
            }
            if TcpStream::connect_timeout(address, remaining.min(Duration::from_millis(750)))
                .is_ok()
            {
                return Ok(true);
            }
        }
        Ok(false)
    })
    .with_context(|| format!("connect to Runtime v3 listener {}", init.api.address))
}

fn wait_for_readiness(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    timeout: Duration,
) -> Result<()> {
    wait_for_convergence("readiness", timeout, |remaining| {
        Ok(owned_runtime_readiness_with_timeout(
            args,
            init,
            remaining.min(Duration::from_millis(750)),
        )
        .is_ok())
    })
}

fn wait_for_stopped(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    stopped_guardian_process_id: Option<u32>,
    timeout: Duration,
) -> Result<()> {
    wait_for_convergence("stop", timeout, |remaining| {
        let attempt_deadline = std::time::Instant::now() + remaining;
        let stopped_listener_gone = runtime_readiness_with_timeout(
            init,
            attempt_deadline
                .saturating_duration_since(std::time::Instant::now())
                .min(Duration::from_millis(750)),
        )
        .map_or(true, |readiness| {
            stopped_guardian_process_id
                .is_some_and(|process_id| process_id != readiness.guardian_process_id)
        });
        let remaining = attempt_deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return Ok(false);
        }
        Ok(platform_stopped_with_timeout(args, remaining)? && stopped_listener_gone)
    })
}

fn wait_for_service_unloaded(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    wait_for_convergence("unload", timeout, |remaining| {
        platform_stopped_with_timeout(args, remaining)
    })
}

fn wait_for_convergence(
    stage: &'static str,
    timeout: Duration,
    mut converged: impl FnMut(Duration) -> Result<bool>,
) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        let now = std::time::Instant::now();
        let remaining = deadline.saturating_duration_since(now);
        if remaining.is_zero() {
            break;
        }
        let success = converged(remaining)?;
        if std::time::Instant::now() >= deadline {
            break;
        }
        if success {
            return Ok(());
        }
        let now = std::time::Instant::now();
        if now >= deadline {
            break;
        }
        std::thread::sleep(
            deadline
                .saturating_duration_since(now)
                .min(Duration::from_millis(200)),
        );
    }
    convergence_timeout(stage, timeout)
}

fn convergence_timeout<T>(stage: &'static str, timeout: Duration) -> Result<T> {
    Err(convergence_error(stage, timeout))
}

fn convergence_error(stage: &'static str, timeout: Duration) -> anyhow::Error {
    ConvergenceDeadlineExceeded { stage, timeout }.into()
}

fn normalize_stage_timeout(
    error: anyhow::Error,
    stage: &'static str,
    timeout: Duration,
) -> anyhow::Error {
    if error
        .downcast_ref::<ServiceManagerDeadlineExceeded>()
        .is_some()
        || error
            .downcast_ref::<ConvergenceDeadlineExceeded>()
            .is_some()
    {
        convergence_error(stage, timeout)
    } else {
        error
    }
}

fn file_hash(path: &Path) -> Result<String> {
    Ok(blake3::hash(
        &fs::read(path).with_context(|| format!("read Runtime v3 init {}", path.display()))?,
    )
    .to_hex()
    .to_string())
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(target_os = "macos")]
fn launchd_target(args: &RuntimeV3ServiceArgs) -> String {
    format!("gui/{}/{}", unsafe { libc::geteuid() }, args.label)
}

#[cfg(target_os = "macos")]
fn launchd_domain() -> String {
    format!("gui/{}", unsafe { libc::geteuid() })
}

#[cfg(target_os = "macos")]
fn installed_launchd_plist(args: &RuntimeV3ServiceArgs) -> Result<PathBuf> {
    let home = std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| {
        anyhow!("HOME is not set; cannot resolve the user LaunchAgents directory")
    })?;
    Ok(home
        .join("Library/LaunchAgents")
        .join(format!("{}.plist", args.label)))
}

#[cfg(target_os = "macos")]
fn install_launchd_plist(args: &RuntimeV3ServiceArgs, source: &Path) -> Result<PathBuf> {
    if !source.is_absolute() || !source.is_file() {
        return Err(anyhow!(
            "Runtime v3 --plist must be an absolute existing file"
        ));
    }
    let destination = installed_launchd_plist(args)?;
    if source == destination {
        return Ok(destination);
    }
    let parent = destination
        .parent()
        .ok_or_else(|| anyhow!("installed launchd plist has no parent directory"))?;
    fs::create_dir_all(parent)
        .with_context(|| format!("create launchd directory {}", parent.display()))?;
    let temporary = parent.join(format!(".{}.plist.installing", args.label));
    fs::copy(source, &temporary).with_context(|| {
        format!(
            "copy launchd plist {} to {}",
            source.display(),
            temporary.display()
        )
    })?;
    fs::rename(&temporary, &destination).with_context(|| {
        format!(
            "install launchd plist {} at {}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(destination)
}

#[cfg(target_os = "macos")]
fn platform_start_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    if let Some(source) = args.plist.as_ref() {
        let plist = install_launchd_plist(args, source)?;
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return Err(ServiceManagerDeadlineExceeded { timeout }.into());
        }
        run_start_command_with_timeout(
            Command::new("launchctl")
                .args(["bootstrap", &launchd_domain()])
                .arg(plist),
            remaining,
        )?;
    } else if !platform_loaded_with_timeout(args, timeout)? {
        let plist = installed_launchd_plist(args)?;
        if !plist.is_file() {
            return Err(anyhow!(
                "Runtime v3 service is not loaded and installed plist is missing; --plist is required"
            ));
        }
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return Err(ServiceManagerDeadlineExceeded { timeout }.into());
        }
        run_start_command_with_timeout(
            Command::new("launchctl")
                .args(["bootstrap", &launchd_domain()])
                .arg(plist),
            remaining,
        )?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn platform_stop_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    if platform_stopped_with_timeout(args, timeout)? {
        return Ok(());
    }
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return convergence_timeout("stop", timeout);
    }
    run_with_timeout(
        Command::new("launchctl").args(["bootout", &launchd_target(args)]),
        remaining,
    )
}

#[cfg(target_os = "macos")]
fn platform_loaded_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<bool> {
    service_manager_command_succeeds_with_timeout(
        Command::new("launchctl").args(["print", &launchd_target(args)]),
        timeout,
    )
}

#[cfg(target_os = "macos")]
fn platform_stopped_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<bool> {
    let target = launchd_target(args);
    let output =
        command_output_with_timeout(Command::new("launchctl").args(["print", &target]), timeout)
            .with_context(|| format!("inspect launchd service {target}"))?;
    launchctl_print_is_stopped(output.status.success(), output.status.code())
        .with_context(|| format!("inspect launchd service {target}"))
}

#[cfg(any(target_os = "macos", test))]
fn launchctl_print_is_stopped(success: bool, exit_code: Option<i32>) -> Result<bool> {
    if success {
        return Ok(false);
    }
    if exit_code == Some(113) {
        return Ok(true);
    }
    Err(anyhow!(
        "launchctl print failed ambiguously with exit code {exit_code:?}"
    ))
}

#[cfg(target_os = "macos")]
fn platform_process_id_with_timeout(
    args: &RuntimeV3ServiceArgs,
    timeout: Duration,
) -> Result<Option<u32>> {
    let output = command_output_with_timeout(
        Command::new("launchctl").args(["print", &launchd_target(args)]),
        timeout,
    )?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(parse_launchctl_process_id(std::str::from_utf8(
        &output.stdout,
    )?))
}

#[cfg(any(target_os = "macos", test))]
fn parse_launchctl_process_id(output: &str) -> Option<u32> {
    output.lines().find_map(|line| {
        let (name, value) = line.trim().split_once('=')?;
        (name.trim() == "pid")
            .then(|| value.trim().parse::<u32>().ok())
            .flatten()
            .filter(|process_id| *process_id > 0)
    })
}

#[cfg(target_os = "linux")]
fn systemd_unit(args: &RuntimeV3ServiceArgs) -> String {
    if args.label.ends_with(".service") {
        args.label.clone()
    } else {
        format!("{}.service", args.label)
    }
}

#[cfg(target_os = "linux")]
fn platform_start_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    run_start_command_with_timeout(
        Command::new("systemctl").args(["start", &systemd_unit(args)]),
        timeout,
    )
}

#[cfg(target_os = "linux")]
fn platform_stop_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    if systemd_service_state_with_timeout(args, timeout)?.is_stopped() {
        return Ok(());
    }
    let remaining = deadline.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return convergence_timeout("stop", timeout);
    }
    run_with_timeout(
        Command::new("systemctl").args(["stop", &systemd_unit(args)]),
        remaining,
    )
}

#[cfg(any(target_os = "linux", test))]
#[derive(Debug, Eq, PartialEq)]
struct SystemdServiceState {
    load: String,
    active: String,
}

#[cfg(any(target_os = "linux", test))]
impl SystemdServiceState {
    fn is_stopped(&self) -> bool {
        self.active == "inactive"
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_systemd_service_state(output: &str) -> Result<SystemdServiceState> {
    let property = |name: &str| {
        output.lines().find_map(|line| {
            let (key, value) = line.split_once('=')?;
            (key == name).then(|| value.trim().to_owned())
        })
    };
    Ok(SystemdServiceState {
        load: property("LoadState")
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow!("systemctl show omitted LoadState"))?,
        active: property("ActiveState")
            .filter(|value| !value.is_empty())
            .ok_or_else(|| anyhow!("systemctl show omitted ActiveState"))?,
    })
}

#[cfg(target_os = "linux")]
fn systemd_service_state_with_timeout(
    args: &RuntimeV3ServiceArgs,
    timeout: Duration,
) -> Result<SystemdServiceState> {
    let unit = systemd_unit(args);
    let output = command_output_with_timeout(
        Command::new("systemctl").args([
            "show",
            "--property=LoadState",
            "--property=ActiveState",
            &unit,
        ]),
        timeout,
    )
    .with_context(|| format!("inspect systemd unit {unit}"))?;
    if !output.status.success() {
        return Err(anyhow!(
            "systemctl show failed for {unit} with status {}",
            output.status
        ));
    }
    parse_systemd_service_state(
        std::str::from_utf8(&output.stdout)
            .with_context(|| format!("systemctl show returned non-UTF-8 state for {unit}"))?,
    )
}

#[cfg(target_os = "linux")]
fn platform_stopped_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<bool> {
    Ok(systemd_service_state_with_timeout(args, timeout)?.is_stopped())
}

#[cfg(target_os = "linux")]
fn platform_loaded_with_timeout(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<bool> {
    service_manager_command_succeeds_with_timeout(
        Command::new("systemctl").args(["is-active", "--quiet", &systemd_unit(args)]),
        timeout,
    )
}

#[cfg(target_os = "linux")]
fn platform_process_id_with_timeout(
    args: &RuntimeV3ServiceArgs,
    timeout: Duration,
) -> Result<Option<u32>> {
    let output = command_output_with_timeout(
        Command::new("systemctl").args([
            "show",
            "--property",
            "MainPID",
            "--value",
            &systemd_unit(args),
        ]),
        timeout,
    )?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(std::str::from_utf8(&output.stdout)?
        .trim()
        .parse::<u32>()
        .ok()
        .filter(|process_id| *process_id > 0))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_start_with_timeout(_args: &RuntimeV3ServiceArgs, _timeout: Duration) -> Result<()> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_stop_with_timeout(_args: &RuntimeV3ServiceArgs, _timeout: Duration) -> Result<()> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_loaded_with_timeout(_args: &RuntimeV3ServiceArgs, _timeout: Duration) -> Result<bool> {
    Ok(false)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_stopped_with_timeout(_args: &RuntimeV3ServiceArgs, _timeout: Duration) -> Result<bool> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_process_id_with_timeout(
    _args: &RuntimeV3ServiceArgs,
    _timeout: Duration,
) -> Result<Option<u32>> {
    Ok(None)
}

fn command_output_with_timeout(command: &mut Command, timeout: Duration) -> Result<Output> {
    let rendered = format!("{command:?}");
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .with_context(|| format!("start service-manager command {rendered}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("capture stdout for service-manager command {rendered}"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("capture stderr for service-manager command {rendered}"))?;
    let stdout_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut stdout = stdout;
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut stderr = stderr;
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    let deadline = std::time::Instant::now() + timeout;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .with_context(|| format!("poll service-manager command {rendered}"))?
        {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
            return Err(ServiceManagerDeadlineExceeded { timeout }.into());
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let stdout = stdout_reader
        .join()
        .map_err(|_| anyhow!("stdout reader panicked for service-manager command {rendered}"))??;
    let stderr = stderr_reader
        .join()
        .map_err(|_| anyhow!("stderr reader panicked for service-manager command {rendered}"))??;
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

fn run_with_timeout(command: &mut Command, timeout: Duration) -> Result<()> {
    let rendered = format!("{command:?}");
    let output = command_output_with_timeout(command, timeout)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr = stderr.trim();
        return Err(if stderr.is_empty() {
            anyhow!(
                "service-manager command failed: {rendered}: {}",
                output.status
            )
        } else {
            anyhow!(
                "service-manager command failed: {rendered}: {}: {stderr}",
                output.status
            )
        });
    }
    Ok(())
}

fn run_start_command_with_timeout(command: &mut Command, timeout: Duration) -> Result<()> {
    run_with_timeout(command, timeout)
}

fn service_manager_command_succeeds_with_timeout(
    command: &mut Command,
    timeout: Duration,
) -> Result<bool> {
    Ok(command_output_with_timeout(command, timeout)?
        .status
        .success())
}

fn platform_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "launchd"
    } else if cfg!(target_os = "linux") {
        "systemd"
    } else {
        "unsupported"
    }
}

pub(crate) fn usage() -> &'static str {
    "csm runtime-v3 start|stop|status|reload --init <absolute-runtime-init.toml> [--candidate <absolute-candidate-init.toml>] [--plist <absolute-launchd-plist>] [--label <service-label>] [--json]"
}

#[cfg(test)]
mod tests {
    use super::*;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct EnvRestore {
        key: &'static str,
        previous: Option<std::ffi::OsString>,
    }

    impl EnvRestore {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(key);
            std::env::set_var(key, value);
            Self { key, previous }
        }

        fn remove(key: &'static str) -> Self {
            let previous = std::env::var_os(key);
            std::env::remove_var(key);
            Self { key, previous }
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }

    #[cfg(unix)]
    fn write_generation_init(root: &Path) -> (PathBuf, RuntimeInitConfig, PathBuf) {
        use std::os::unix::fs::symlink;

        let generation = root.join("generations/test-generation");
        let bin = generation.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let mut artifacts = serde_json::Map::new();
        for (key, filename) in [
            ("csm", "csm"),
            ("guardian", "adl-runtime-guardian"),
            ("kernel", "adl-runtime-kernel"),
        ] {
            let path = bin.join(filename);
            fs::write(&path, format!("{filename}-test-artifact")).unwrap();
            let mut permissions = fs::metadata(&path).unwrap().permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).unwrap();
            let hash = format!("{:x}", Sha256::digest(fs::read(&path).unwrap()));
            artifacts.insert(
                key.into(),
                serde_json::json!({"file": format!("bin/{filename}"), "sha256": hash}),
            );
        }
        fs::write(
            generation.join("receipt.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema": RUNTIME_GENERATION_RECEIPT_SCHEMA,
                "generation": "test-generation",
                "source_revision": "test-revision",
                "platform": format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
                "build_profile": "debug",
                "runtime_init_schema": RUNTIME_INIT_SCHEMA,
                "artifacts": artifacts,
            }))
            .unwrap(),
        )
        .unwrap();
        symlink("generations/test-generation", root.join("current")).unwrap();
        let state_root = root.join("state");
        let kernel = root.join("current/bin/adl-runtime-kernel");
        let text = include_str!("../../../infra/runtime-v3/runtime-init.toml")
            .replace("/var/lib/adl/runtime-v3", &state_root.display().to_string())
            .replace(
                "/opt/adl/bin/adl-runtime-kernel",
                &kernel.display().to_string(),
            );
        let init_path = root.join("runtime-init.toml");
        fs::write(&init_path, text).unwrap();
        let init = RuntimeInitConfig::load(Some(init_path.clone())).unwrap();
        (init_path, init, root.join("current/bin/csm"))
    }

    #[cfg(unix)]
    fn update_generation_receipt(root: &Path, update: impl FnOnce(&mut serde_json::Value)) {
        let path = root.join("current/receipt.json");
        let mut receipt = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        update(&mut receipt);
        fs::write(path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    }

    fn write_valid_init(root: &Path) -> (PathBuf, RuntimeInitConfig) {
        let state_root = root.join("state");
        let kernel = std::env::current_exe().unwrap();
        let text = include_str!("../../../infra/runtime-v3/runtime-init.toml")
            .replace("/var/lib/adl/runtime-v3", &state_root.display().to_string())
            .replace(
                "/opt/adl/bin/adl-runtime-kernel",
                &kernel.display().to_string(),
            );
        let path = root.join("runtime-init.toml");
        fs::write(&path, text).unwrap();
        let init = validated_init(&path).unwrap();
        (path, init)
    }

    fn service_args(init: PathBuf) -> RuntimeV3ServiceArgs {
        RuntimeV3ServiceArgs {
            init,
            candidate: None,
            plist: None,
            label: "com.agentlogic.adl-runtime-v3-test-missing".into(),
            json: false,
        }
    }

    fn test_config_generation_identity(generation: &str) -> ConfigGenerationIdentity {
        ConfigGenerationIdentity {
            generation: generation.to_owned(),
            receipt_digest: "a".repeat(64),
        }
    }

    fn write_active_config_generation_ref(init: &Path, generation: &str) {
        let active_ref = active_generation_ref(init).unwrap();
        fs::write(active_ref, format!("{generation} {}\n", "b".repeat(64))).unwrap();
    }

    #[test]
    fn parser_requires_absolute_init() {
        let error = parse_args(&["--init".into(), "relative.toml".into()]).unwrap_err();
        assert!(error.to_string().contains("must be absolute"));
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_mixed_artifacts_before_mutation() {
        use std::cell::Cell;

        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        validate_runtime_generation_with_service_binary(&init, &csm).unwrap();

        fs::write(
            root.path().join("current/bin/adl-runtime-guardian"),
            "mixed",
        )
        .unwrap();
        let service_mutated = Cell::new(false);
        let error = run_after_preflight(
            || validate_runtime_generation_with_service_binary(&init, &csm).map(|_| ()),
            || {
                service_mutated.set(true);
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error.to_string().contains("hash mismatch"));
        assert!(
            !service_mutated.get(),
            "preflight failure must precede service mutation"
        );
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_non_executable_artifacts_before_mutation() {
        use std::cell::Cell;

        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        let guardian = root.path().join("current/bin/adl-runtime-guardian");
        let mut permissions = fs::metadata(&guardian).unwrap().permissions();
        permissions.set_mode(0o644);
        fs::set_permissions(&guardian, permissions).unwrap();

        let service_mutated = Cell::new(false);
        let error = run_after_preflight(
            || validate_runtime_generation_with_service_binary(&init, &csm).map(|_| ()),
            || {
                service_mutated.set(true);
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error.to_string().contains("not executable"));
        assert!(
            !service_mutated.get(),
            "executable preflight failure must precede service mutation"
        );
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_direct_generation_kernel_path() {
        let root = tempfile::tempdir().unwrap();
        let (_path, mut init, csm) = write_generation_init(root.path());
        init.binaries.kernel_path = root
            .path()
            .join("generations/test-generation/bin/adl-runtime-kernel");

        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error.to_string().contains("through current/bin"));
    }

    #[test]
    fn deadline_errors_render_exact_stage_and_timeout() {
        let service = ServiceManagerDeadlineExceeded {
            timeout: Duration::from_millis(17),
        };
        assert_eq!(
            service.to_string(),
            "service-manager probe exceeded 17 milliseconds"
        );

        let convergence = ConvergenceDeadlineExceeded {
            stage: "readiness",
            timeout: Duration::from_millis(23),
        };
        assert_eq!(
            convergence.to_string(),
            "Runtime v3 convergence stage readiness did not complete within 23 milliseconds"
        );
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_missing_broken_and_escaping_current_links() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        fs::remove_file(root.path().join("current")).unwrap();
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error
            .to_string()
            .contains("inspect Runtime v3 current generation"));

        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        fs::remove_file(root.path().join("current")).unwrap();
        symlink("generations/missing", root.path().join("current")).unwrap();
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error
            .to_string()
            .contains("resolve Runtime v3 current generation"));

        let root = tempfile::tempdir().unwrap();
        let external = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("generations")).unwrap();
        fs::create_dir_all(external.path().join("bin")).unwrap();
        fs::write(
            external.path().join("bin/adl-runtime-kernel"),
            "external-kernel",
        )
        .unwrap();
        fs::write(external.path().join("bin/csm"), "external-csm").unwrap();
        symlink(external.path(), root.path().join("current")).unwrap();
        let mut init = write_valid_init(root.path()).1;
        init.binaries.kernel_path = root.path().join("current/bin/adl-runtime-kernel");
        let error = validate_runtime_generation_with_service_binary(
            &init,
            &root.path().join("current/bin/csm"),
        )
        .unwrap_err();
        assert!(error
            .to_string()
            .contains("current generation escapes generations directory"));
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_invalid_receipt_contracts() {
        for (field, invalid, expected_error) in [
            (
                "source_revision",
                serde_json::json!(""),
                "identity or compatibility",
            ),
            (
                "artifacts",
                serde_json::json!({}),
                "artifact set is incomplete",
            ),
        ] {
            let root = tempfile::tempdir().unwrap();
            let (_path, init, csm) = write_generation_init(root.path());
            update_generation_receipt(root.path(), |receipt| receipt[field] = invalid);

            let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
            assert!(error.to_string().contains(expected_error));
        }
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_unreadable_receipt_and_non_file_artifact() {
        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        fs::write(root.path().join("current/receipt.json"), "not json").unwrap();
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error
            .to_string()
            .contains("parse Runtime v3 generation receipt"));

        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        fs::remove_file(root.path().join("current/bin/csm")).unwrap();
        fs::create_dir(root.path().join("current/bin/csm")).unwrap();
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error.to_string().contains("generation artifact is missing"));
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_rejects_receipt_paths_and_binary_identity() {
        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        update_generation_receipt(root.path(), |receipt| {
            receipt["artifacts"]["guardian"]["file"] = serde_json::json!("bin/not-guardian");
        });
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error.to_string().contains("path mismatch"));

        let root = tempfile::tempdir().unwrap();
        let (_path, mut init, csm) = write_generation_init(root.path());
        init.binaries.kernel_path = root.path().join("current/bin/csm");
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error.to_string().contains("kernel does not belong"));

        let root = tempfile::tempdir().unwrap();
        let (_path, init, _csm) = write_generation_init(root.path());
        let guardian = root.path().join("current/bin/adl-runtime-guardian");
        let error = validate_runtime_generation_with_service_binary(&init, &guardian).unwrap_err();
        assert!(error
            .to_string()
            .contains("service control does not belong"));
    }

    #[cfg(unix)]
    #[test]
    fn generation_preflight_requires_atomic_current_symlink() {
        let root = tempfile::tempdir().unwrap();
        let (_path, init, csm) = write_generation_init(root.path());
        fs::remove_file(root.path().join("current")).unwrap();
        fs::create_dir(root.path().join("current")).unwrap();
        let error = validate_runtime_generation_with_service_binary(&init, &csm).unwrap_err();
        assert!(error.to_string().contains("not an atomic symlink"));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn launchd_preflight_requires_guardian_from_current_generation() {
        let root = tempfile::tempdir().unwrap();
        let (init_path, init, _csm) = write_generation_init(root.path());
        let plist = root.path().join("runtime.plist");
        let expected = root.path().join("current/bin/adl-runtime-guardian");
        fs::write(
            &plist,
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><plist version=\"1.0\"><dict><key>ProgramArguments</key><array><string>{}</string></array></dict></plist>",
                expected.display()
            ),
        )
        .unwrap();
        let mut args = service_args(init_path);
        args.plist = Some(plist.clone());

        validate_runtime_service_definition(&args, &init).unwrap();
        fs::write(
            &plist,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><plist version=\"1.0\"><dict><key>ProgramArguments</key><array><string>/old/bin/adl-runtime-guardian</string></array></dict></plist>",
        )
        .unwrap();
        let error = validate_runtime_service_definition(&args, &init).unwrap_err();
        assert!(error.to_string().contains("does not resolve through"));
    }

    #[test]
    fn service_definition_preflight_parsers_require_exact_executable_position() {
        let expected = "/runtime/current/bin/adl-runtime-guardian";
        let systemd = format!("{{ path=/old/guardian ; argv[]=/old/guardian {expected} ; }}");
        assert_eq!(
            systemd_exec_start_executable(&systemd).unwrap(),
            Path::new("/old/guardian")
        );
        let ambiguous_systemd = format!("ENV_path={expected} {{ path=/old/guardian ; }}");
        assert!(systemd_exec_start_executable(&ambiguous_systemd).is_err());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn launchd_semantic_preflight_ignores_commented_program_arguments() {
        let root = tempfile::tempdir().unwrap();
        let plist = root.path().join("commented.plist");
        fs::write(
            &plist,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><plist version=\"1.0\"><dict><!-- <key>ProgramArguments</key><array><string>/runtime/current/bin/adl-runtime-guardian</string></array> --><key>ProgramArguments</key><array><string>/old/guardian</string></array></dict></plist>",
        )
        .unwrap();
        assert_eq!(
            launchd_program_executable(&plist).unwrap(),
            Path::new("/old/guardian")
        );
    }

    #[test]
    fn parser_accepts_explicit_service_identity() {
        let args = parse_args(&[
            "--init".into(),
            "/opt/agent-logic/runtime-init.toml".into(),
            "--plist".into(),
            "/Library/LaunchDaemons/com.agentlogic.adl-runtime-v3.plist".into(),
            "--label".into(),
            "com.agentlogic.adl-runtime-v3".into(),
            "--json".into(),
        ])
        .unwrap();
        assert_eq!(args.label, DEFAULT_LABEL);
        assert!(args.json);
        assert!(args.plist.is_some());
    }

    #[test]
    fn parser_accepts_environment_defaults_and_help_aliases() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _init = EnvRestore::set("ADL_RUNTIME_V3_INIT", "/opt/agent-logic/runtime-init.toml");
        let _plist = EnvRestore::set(
            "ADL_RUNTIME_V3_PLIST",
            "/Library/LaunchDaemons/com.agentlogic.adl-runtime-v3.plist",
        );

        let args = parse_args(&["--json".into()]).unwrap();
        assert_eq!(args.init, Path::new("/opt/agent-logic/runtime-init.toml"));
        assert_eq!(
            args.plist.as_deref(),
            Some(Path::new(
                "/Library/LaunchDaemons/com.agentlogic.adl-runtime-v3.plist"
            ))
        );
        assert!(args.json);

        assert!(real_runtime_v3_service(&["-h".into()]).is_ok());
        assert!(real_runtime_v3_service(&["--help".into()]).is_ok());
    }

    #[test]
    fn parser_requires_init_when_environment_default_is_absent() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _init = EnvRestore::remove("ADL_RUNTIME_V3_INIT");

        let error = parse_args(&[]).unwrap_err();

        assert!(error.to_string().contains("requires --init"));
    }

    #[test]
    fn parser_accepts_absolute_reload_candidate() {
        let args = parse_args(&[
            "--init".into(),
            "/opt/agent-logic/runtime-init.toml".into(),
            "--candidate".into(),
            "/opt/agent-logic/runtime-init.next.toml".into(),
        ])
        .unwrap();
        assert_eq!(
            args.candidate.as_deref(),
            Some(Path::new("/opt/agent-logic/runtime-init.next.toml"))
        );
    }

    #[test]
    fn parser_rejects_missing_values_unknown_options_and_invalid_identity() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _init = EnvRestore::remove("ADL_RUNTIME_V3_INIT");

        for args in [
            vec!["--init".into()],
            vec!["--init".into(), "/tmp/init".into(), "--plist".into()],
            vec!["--init".into(), "/tmp/init".into(), "--candidate".into()],
            vec!["--init".into(), "/tmp/init".into(), "--label".into()],
            vec!["--unknown".into()],
            Vec::new(),
            vec![
                "--init".into(),
                "/tmp/init".into(),
                "--candidate".into(),
                "relative".into(),
            ],
            vec![
                "--init".into(),
                "/tmp/init".into(),
                "--label".into(),
                "bad label".into(),
            ],
        ] {
            assert!(parse_args(&args).is_err(), "unexpectedly accepted {args:?}");
        }
    }

    #[test]
    fn command_dispatch_covers_help_missing_unknown_and_start_candidate_rejection() {
        assert!(real_runtime_v3_service(&[]).is_err());
        assert!(real_runtime_v3_service(&["help".into()]).is_ok());
        assert!(real_runtime_v3_service(&[
            "unknown".into(),
            "--init".into(),
            "/tmp/runtime-init.toml".into(),
        ])
        .is_err());

        let root = tempfile::tempdir().unwrap();
        let invalid = root.path().join("invalid.toml");
        fs::write(&invalid, "invalid = [").unwrap();
        for operation in ["start", "reload", "stop", "status"] {
            assert!(real_runtime_v3_service(&[
                operation.into(),
                "--init".into(),
                invalid.display().to_string(),
            ])
            .is_err());
        }
        assert!(real_runtime_v3_service(&[
            "start".into(),
            "--init".into(),
            "/tmp/runtime-init.toml".into(),
            "--candidate".into(),
            "/tmp/runtime-init.next.toml".into(),
        ])
        .is_err());
    }

    #[test]
    fn candidate_install_retains_exact_last_known_good_config() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::write(&candidate, "candidate").unwrap();
        write_active_config_generation_ref(&active, "current");

        let identity = test_config_generation_identity("candidate");
        let backup = replace_config_with_candidate(&active, &candidate, &identity).unwrap();

        assert_eq!(fs::read_to_string(&active).unwrap(), "candidate");
        assert_eq!(fs::read_to_string(backup).unwrap(), "current");
        assert_eq!(
            fs::read_to_string(active_generation_ref(&active).unwrap()).unwrap(),
            format!("candidate {}\n", "a".repeat(64))
        );
    }

    #[test]
    fn interrupted_reload_restores_last_known_good_before_next_start() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::write(&candidate, "candidate").unwrap();
        write_active_config_generation_ref(&active, "current");

        let identity = test_config_generation_identity("candidate");
        let backup = replace_config_with_candidate(&active, &candidate, &identity).unwrap();
        assert_eq!(fs::read_to_string(&active).unwrap(), "candidate");

        let mut stopped = false;
        reconcile_interrupted_reload_with(
            &active,
            |path| {
                assert_eq!(path, backup);
                assert_eq!(fs::read_to_string(path).unwrap(), "current");
                Ok(())
            },
            || false,
            || {
                stopped = true;
                Ok(())
            },
        )
        .unwrap();

        assert!(stopped);
        assert_eq!(fs::read_to_string(&active).unwrap(), "current");
        assert_eq!(
            fs::read_to_string(active_generation_ref(&active).unwrap()).unwrap(),
            format!("current {}\n", "b".repeat(64))
        );
        assert!(!backup.exists());
    }

    #[cfg(unix)]
    #[test]
    fn interrupted_reload_reconciles_before_config_generation_preflight() {
        let root = tempfile::tempdir().unwrap();
        let (active, _, _) = write_generation_init(root.path());
        let candidate = root.path().join("runtime-init.next.toml");
        let candidate_text = fs::read_to_string(&active)
            .unwrap()
            .replace("/state", "/candidate-state");
        fs::write(&candidate, candidate_text).unwrap();

        let active_identity =
            provision_config_generation(&active, "test-generation").expect("provision active");
        activate_config_generation(&active, &active_identity).expect("activate active");
        let candidate_identity =
            provision_config_generation_in_store(&candidate, &active, "test-generation")
                .expect("provision candidate in active store");
        let backup = replace_config_with_candidate(&active, &candidate, &candidate_identity)
            .expect("install interrupted candidate");
        let active_ref = active_generation_ref(&active).unwrap();
        fs::write(
            &active_ref,
            format!(
                "{} {}\n",
                active_identity.generation, active_identity.receipt_digest
            ),
        )
        .unwrap();

        let pre_reconcile_error =
            prepare_active_config_generation(&active, "test-generation").unwrap_err();
        assert!(pre_reconcile_error
            .to_string()
            .contains("active reference does not match init content"));

        reconcile_interrupted_reload_with(
            &active,
            |path| {
                assert_eq!(path, backup);
                validated_init(path).map(|_| ())
            },
            || false,
            || Ok(()),
        )
        .unwrap();

        assert_eq!(
            prepare_active_config_generation(&active, "test-generation")
                .expect("preflight after reconcile"),
            active_identity
        );
    }

    #[test]
    fn interrupted_reload_commits_candidate_when_owned_readiness_matches_active_hash() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::write(&candidate, "candidate").unwrap();
        write_active_config_generation_ref(&active, "current");
        let identity = test_config_generation_identity("candidate");
        let backup = replace_config_with_candidate(&active, &candidate, &identity).unwrap();
        let mut stopped = false;

        reconcile_interrupted_reload_with(
            &active,
            |_| Ok(()),
            || true,
            || {
                stopped = true;
                Ok(())
            },
        )
        .unwrap();

        assert!(!stopped);
        assert_eq!(fs::read_to_string(&active).unwrap(), "candidate");
        assert_eq!(
            fs::read_to_string(active_generation_ref(&active).unwrap()).unwrap(),
            format!("candidate {}\n", "a".repeat(64))
        );
        assert!(!backup.exists());
    }

    #[test]
    fn committed_candidate_cleanup_removes_backup_and_stale_staging() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        fs::write(&active, "candidate").unwrap();
        let (backup, staged) = reload_transaction_paths(&active).unwrap();
        fs::write(&backup, "known-good").unwrap();
        fs::write(&staged, "stale-stage").unwrap();
        let active_ref = active_generation_ref(&active).unwrap();
        let backup_ref = active_ref.with_extension("active-generation.last-known-good");
        fs::write(&active_ref, format!("candidate {}\n", "a".repeat(64))).unwrap();
        fs::write(&backup_ref, format!("known-good {}\n", "b".repeat(64))).unwrap();

        commit_candidate(&active, &backup).unwrap();

        assert!(!backup.exists());
        assert!(!staged.exists());
        assert!(!backup_ref.exists());
        assert_eq!(fs::read_to_string(active).unwrap(), "candidate");
    }

    #[test]
    fn empty_interrupted_transaction_is_a_noop() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        fs::write(&active, "active").unwrap();
        let mut stopped = false;

        reconcile_interrupted_reload_with(
            &active,
            |_| panic!("no backup should be validated"),
            || false,
            || {
                stopped = true;
                Ok(())
            },
        )
        .unwrap();

        assert!(!stopped);
        assert_eq!(fs::read_to_string(active).unwrap(), "active");
    }

    #[test]
    fn reload_never_overwrites_an_existing_last_known_good_config() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "unproven-active").unwrap();
        fs::write(&candidate, "next-candidate").unwrap();
        let (backup, _) = reload_transaction_paths(&active).unwrap();
        fs::write(&backup, "known-good").unwrap();

        let identity = test_config_generation_identity("candidate");
        let error = replace_config_with_candidate(&active, &candidate, &identity).unwrap_err();

        assert!(error.to_string().contains("transaction already exists"));
        assert_eq!(fs::read_to_string(backup).unwrap(), "known-good");
    }

    #[test]
    fn failed_backup_copy_removes_its_partial_transaction_file() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::create_dir(&active).unwrap();
        fs::write(&candidate, "candidate").unwrap();
        let (backup, staged) = reload_transaction_paths(&active).unwrap();

        let identity = test_config_generation_identity("candidate");
        assert!(replace_config_with_candidate(&active, &candidate, &identity).is_err());
        assert!(!backup.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn failed_candidate_copy_removes_staging_and_restores_active() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::create_dir(&candidate).unwrap();
        let (backup, staged) = reload_transaction_paths(&active).unwrap();
        write_active_config_generation_ref(&active, "current");

        let identity = test_config_generation_identity("candidate");
        assert!(replace_config_with_candidate(&active, &candidate, &identity).is_err());
        assert_eq!(fs::read_to_string(&active).unwrap(), "current");
        assert_eq!(
            fs::read_to_string(active_generation_ref(&active).unwrap()).unwrap(),
            format!("current {}\n", "b".repeat(64))
        );
        assert!(!backup.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn transaction_helpers_reject_ambiguous_or_invalid_paths() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        fs::write(&active, "current").unwrap();
        let identity = test_config_generation_identity("candidate");
        assert!(replace_config_with_candidate(&active, &active, &identity).is_err());
        assert!(reload_transaction_paths(Path::new("/")).is_err());
        assert!(sync_parent(Path::new("/")).is_err());

        let (_, staged) = reload_transaction_paths(&active).unwrap();
        fs::write(&staged, "candidate").unwrap();
        let error = reconcile_interrupted_reload_with(&active, |_| Ok(()), || false, || Ok(()))
            .unwrap_err();
        assert!(error.to_string().contains("ambiguous staged candidate"));
    }

    #[test]
    fn interrupted_reload_validation_failure_preserves_last_known_good() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let (backup, staged) = reload_transaction_paths(&active).unwrap();
        fs::write(&active, "candidate").unwrap();
        fs::write(&backup, "known-good").unwrap();
        fs::write(&staged, "partial").unwrap();

        assert!(reconcile_interrupted_reload_with(
            &active,
            |_| Err(anyhow!("invalid backup")),
            || false,
            || Ok(())
        )
        .is_err());
        assert_eq!(fs::read_to_string(&backup).unwrap(), "known-good");
        assert!(staged.exists());
    }

    #[test]
    fn interrupted_reload_wrapper_rejects_invalid_backup_and_removes_valid_staging() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let (backup, staged) = reload_transaction_paths(&active).unwrap();
        fs::write(&active, "candidate").unwrap();
        fs::write(&backup, "invalid = [").unwrap();
        assert!(reconcile_interrupted_reload_with(
            &active,
            |path| validated_init(path).map(|_| ()),
            || false,
            || Ok(())
        )
        .is_err());

        fs::write(&backup, "known-good").unwrap();
        fs::write(&staged, "partial").unwrap();
        let active_ref = active_generation_ref(&active).unwrap();
        let backup_ref = active_ref.with_extension("active-generation.last-known-good");
        fs::write(&active_ref, format!("candidate {}\n", "a".repeat(64))).unwrap();
        fs::write(&backup_ref, format!("known-good {}\n", "b".repeat(64))).unwrap();
        reconcile_interrupted_reload_with(&active, |_| Ok(()), || false, || Ok(())).unwrap();
        assert_eq!(fs::read_to_string(&active).unwrap(), "known-good");
        assert_eq!(
            fs::read_to_string(active_ref).unwrap(),
            format!("known-good {}\n", "b".repeat(64))
        );
        assert!(!backup.exists());
        assert!(!backup_ref.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn valid_init_status_and_readiness_fail_closed_without_a_service() {
        let root = tempfile::tempdir().unwrap();
        let (path, init) = write_valid_init(root.path());
        let mut args = service_args(path.clone());

        assert!(runtime_readiness_with_timeout(&init, Duration::from_millis(750)).is_err());
        assert!(owned_runtime_readiness(&args, &init).is_err());
        assert!(emit_status(&args, &init, "status", false).is_ok());
        args.json = true;
        assert!(emit_status(&args, &init, "status", true).is_err());
        assert!(status(&args, "status").is_err());
        assert!(wait_for_listener_open(&init, Duration::ZERO).is_err());
        assert!(wait_for_readiness(&args, &init, Duration::ZERO).is_err());
        assert!(wait_for_stopped(&args, &init, None, Duration::ZERO).is_err());

        let (backup, _) = reload_transaction_paths(&path).unwrap();
        fs::write(backup, "known-good").unwrap();
        assert!(status(&args, "status")
            .unwrap_err()
            .to_string()
            .contains("incomplete"));
    }

    #[test]
    fn readiness_client_reaches_connection_failure_with_valid_trust_root() {
        let root = tempfile::tempdir().unwrap();
        let (_, mut init) = write_valid_init(root.path());
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let trust_roots = root.path().join("trust-roots.pem");
        fs::write(&trust_roots, certified.cert.pem()).unwrap();
        init.api.tls.trust_roots_path = trust_roots;

        let error = runtime_readiness_with_timeout(&init, Duration::from_millis(750))
            .unwrap_err()
            .to_string();
        assert!(error.contains("query Runtime v3 readiness"));
    }

    #[test]
    fn convergence_slow_success_completes_within_configured_deadline() {
        let mut probes = 0_u8;
        wait_for_convergence("readiness", Duration::from_millis(500), |_| {
            probes += 1;
            Ok(probes == 3)
        })
        .unwrap();
        assert_eq!(probes, 3);
    }

    #[test]
    fn convergence_true_timeout_reports_exact_stage_and_deadline() {
        let error = wait_for_convergence("unload", Duration::ZERO, |_| Ok(false)).unwrap_err();
        assert_eq!(
            error.to_string(),
            "Runtime v3 convergence stage unload did not complete within 0 milliseconds"
        );
    }

    #[test]
    fn convergence_rejects_success_observed_after_deadline() {
        let error = wait_for_convergence("readiness", Duration::from_millis(5), |_| {
            std::thread::sleep(Duration::from_millis(10));
            Ok(true)
        })
        .unwrap_err();
        assert!(error.to_string().contains("stage readiness"));
        assert!(error.to_string().contains("5 milliseconds"));
    }

    #[test]
    fn convergence_rejects_overrun_for_every_blocking_stage() {
        for stage in ["stop", "unload", "readiness"] {
            let error = wait_for_convergence(stage, Duration::from_millis(5), |_| {
                std::thread::sleep(Duration::from_millis(10));
                Ok(true)
            })
            .unwrap_err();
            assert!(error.to_string().contains(&format!("stage {stage}")));
        }
    }

    #[test]
    fn convergence_hanging_start_and_loaded_commands_are_killed_at_stage_deadline() {
        let timeout = Duration::from_millis(250);

        let started = std::time::Instant::now();
        let start_error = run_start_command_with_timeout(Command::new("sleep").arg("2"), timeout)
            .map_err(|error| normalize_stage_timeout(error, "listener", timeout))
            .unwrap_err();
        assert!(start_error.to_string().contains("stage listener"));
        assert!(start_error.to_string().contains("250 milliseconds"));
        assert!(started.elapsed() < Duration::from_millis(1500));

        let started = std::time::Instant::now();
        let loaded_error =
            service_manager_command_succeeds_with_timeout(Command::new("sleep").arg("2"), timeout)
                .map_err(|error| normalize_stage_timeout(error, "listener", timeout))
                .unwrap_err();
        assert!(loaded_error.to_string().contains("stage listener"));
        assert!(loaded_error.to_string().contains("250 milliseconds"));
        assert!(started.elapsed() < Duration::from_millis(1500));
    }

    #[test]
    fn service_manager_timeout_preserves_non_timeout_command_diagnostics() {
        let started = std::time::Instant::now();
        let error =
            run_with_timeout(&mut Command::new("false"), Duration::from_secs(1)).unwrap_err();
        assert!(error.to_string().contains("service-manager command failed"));
        assert!(error.to_string().contains("\"false\""));
        assert!(error.to_string().contains("exit status"));
        assert!(started.elapsed() < Duration::from_millis(500));
    }

    #[test]
    fn service_manager_output_is_drained_while_command_runs() {
        let output = command_output_with_timeout(
            Command::new("/bin/sh").args(["-c", "dd if=/dev/zero bs=131072 count=1 2>/dev/null"]),
            Duration::from_secs(2),
        )
        .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout.len(), 131_072);
    }

    #[test]
    fn convergence_stage_normalization_preserves_non_deadline_errors() {
        let error = normalize_stage_timeout(
            anyhow!("malformed service-manager state"),
            "stop",
            Duration::from_secs(300),
        );
        assert_eq!(error.to_string(), "malformed service-manager state");
    }

    #[test]
    fn convergence_listener_open_is_distinct_from_full_readiness() {
        let root = tempfile::tempdir().unwrap();
        let (path, mut init) = write_valid_init(root.path());
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        init.api.address = listener.local_addr().unwrap().to_string();
        let args = service_args(path);

        wait_for_listener_open(&init, Duration::from_secs(1)).unwrap();
        let error = wait_for_readiness(&args, &init, Duration::ZERO).unwrap_err();
        assert!(error.to_string().contains("stage readiness"));
    }

    #[test]
    fn convergence_timeout_rolls_back_candidate_and_restarts_last_known_good() {
        use std::cell::Cell;

        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "last-known-good").unwrap();
        fs::write(&candidate, "candidate").unwrap();
        write_active_config_generation_ref(&active, "last-known-good");
        let identity = test_config_generation_identity("candidate");
        let stopped_current = Cell::new(false);
        let stopped_candidate = Cell::new(false);
        let restarted_current = Cell::new(false);

        let error = reload_candidate_transaction(
            &active,
            &candidate,
            &identity,
            || {
                stopped_current.set(true);
                Ok(())
            },
            || wait_for_convergence("readiness", Duration::ZERO, |_| Ok(false)),
            || {
                stopped_candidate.set(true);
                Ok(())
            },
            || {
                restarted_current.set(true);
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error.to_string().contains("stage readiness"));
        assert!(stopped_current.get());
        assert!(stopped_candidate.get());
        assert!(restarted_current.get());
        assert_eq!(fs::read_to_string(&active).unwrap(), "last-known-good");
        let (backup, staged) = reload_transaction_paths(&active).unwrap();
        assert!(!backup.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn validated_init_rejects_missing_kernel_and_invalid_toml() {
        let root = tempfile::tempdir().unwrap();
        let invalid = root.path().join("invalid.toml");
        fs::write(&invalid, "not toml = [").unwrap();
        assert!(validated_init(&invalid).is_err());

        let (path, _) = write_valid_init(root.path());
        let text = fs::read_to_string(&path).unwrap().replace(
            &std::env::current_exe().unwrap().display().to_string(),
            "/missing/kernel",
        );
        fs::write(&path, text).unwrap();
        assert!(validated_init(&path)
            .unwrap_err()
            .to_string()
            .contains("kernel does not exist"));
    }

    #[test]
    fn command_runner_reports_success_failure_and_missing_binary() {
        assert!(run_with_timeout(&mut Command::new("true"), Duration::from_secs(1)).is_ok());
        assert!(run_with_timeout(&mut Command::new("false"), Duration::from_secs(1)).is_err());
        assert!(run_with_timeout(
            &mut Command::new("adl-command-that-does-not-exist"),
            Duration::from_secs(1)
        )
        .is_err());
        assert!(!platform_name().is_empty());

        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        let destination = root.path().join("destination");
        fs::write(&source, "source").unwrap();
        fs::write(&destination, "existing").unwrap();
        assert!(copy_create_new(&source, &destination).is_err());
        assert_eq!(fs::read_to_string(destination).unwrap(), "existing");

        let missing_backup = root.path().join("missing-backup");
        assert!(restore_last_known_good(&source, &missing_backup).is_err());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn systemd_unit_adds_suffix_once() {
        let mut args = service_args(PathBuf::from("/tmp/runtime-init.toml"));
        assert_eq!(
            systemd_unit(&args),
            "com.agentlogic.adl-runtime-v3-test-missing.service"
        );
        args.label.push_str(".service");
        assert_eq!(systemd_unit(&args), args.label);
    }

    #[test]
    fn readiness_requires_runtime_identity_and_full_health() {
        let healthy = RuntimeReadinessProbe {
            schema: "adl.runtime_v3.readiness.v1".into(),
            ready: true,
            lifecycle: "running".into(),
            observability_ready: true,
            runtime_instance_id: "runtime-wuji".into(),
            runtime_process_id: 42,
            guardian_process_id: 41,
            active_init_hash: "a".repeat(64),
            config_generation: "b".repeat(64),
            config_receipt_digest: "c".repeat(64),
        };
        let active_config_generation = ConfigGenerationIdentity {
            generation: "b".repeat(64),
            receipt_digest: "c".repeat(64),
        };
        assert!(validate_readiness_probe(&healthy).is_ok());
        assert!(
            validate_owned_readiness(41, &"a".repeat(64), &active_config_generation, &healthy)
                .is_ok()
        );
        assert!(
            validate_owned_readiness(99, &"a".repeat(64), &active_config_generation, &healthy)
                .is_err()
        );
        assert!(
            validate_owned_readiness(41, &"b".repeat(64), &active_config_generation, &healthy)
                .is_err()
        );
        assert!(validate_owned_readiness(
            41,
            &"a".repeat(64),
            &ConfigGenerationIdentity {
                generation: "d".repeat(64),
                receipt_digest: "c".repeat(64),
            },
            &healthy
        )
        .is_err());
        assert!(validate_owned_readiness(
            41,
            &"a".repeat(64),
            &ConfigGenerationIdentity {
                generation: "b".repeat(64),
                receipt_digest: "d".repeat(64),
            },
            &healthy
        )
        .is_err());
        assert!(!is_blake3_hex("short"));
        assert!(!is_blake3_hex(&"z".repeat(64)));

        for unhealthy in [
            RuntimeReadinessProbe {
                ready: false,
                ..healthy.clone()
            },
            RuntimeReadinessProbe {
                schema: "unrelated.listener.v1".into(),
                ready: true,
                lifecycle: "running".into(),
                observability_ready: true,
                runtime_instance_id: "runtime-wuji".into(),
                runtime_process_id: 42,
                guardian_process_id: 41,
                active_init_hash: "a".repeat(64),
                config_generation: "b".repeat(64),
                config_receipt_digest: "c".repeat(64),
            },
            RuntimeReadinessProbe {
                guardian_process_id: 0,
                ..healthy.clone()
            },
            RuntimeReadinessProbe {
                active_init_hash: "invalid".into(),
                ..healthy.clone()
            },
            RuntimeReadinessProbe {
                config_generation: "invalid".into(),
                ..healthy.clone()
            },
            RuntimeReadinessProbe {
                config_receipt_digest: "invalid".into(),
                ..healthy.clone()
            },
        ] {
            assert!(validate_readiness_probe(&unhealthy).is_err());
        }
    }

    #[test]
    fn launchctl_process_identity_parser_is_exact_and_rejects_zero() {
        assert_eq!(
            parse_launchctl_process_id("state = running\n\tpid = 12345\n"),
            Some(12345)
        );
        assert_eq!(parse_launchctl_process_id("pid = 0\n"), None);
        assert_eq!(parse_launchctl_process_id("parent-pid = 12345\n"), None);
    }

    #[test]
    fn launchctl_state_classifier_accepts_only_running_or_service_not_found() {
        assert!(!launchctl_print_is_stopped(true, Some(0)).unwrap());
        assert!(launchctl_print_is_stopped(false, Some(113)).unwrap());
        for exit_code in [Some(1), Some(3), None] {
            assert!(launchctl_print_is_stopped(false, exit_code).is_err());
        }
    }

    #[test]
    fn systemd_state_parser_distinguishes_stopped_transitional_and_failed_units() {
        let stopped =
            parse_systemd_service_state("LoadState=not-found\nActiveState=inactive\n").unwrap();
        assert!(stopped.is_stopped());

        for active in ["active", "activating", "deactivating", "failed"] {
            let state =
                parse_systemd_service_state(&format!("LoadState=loaded\nActiveState={active}\n"))
                    .unwrap();
            assert!(
                !state.is_stopped(),
                "{active} must not be treated as stopped"
            );
        }

        assert!(parse_systemd_service_state("LoadState=loaded\n").is_err());
        assert!(parse_systemd_service_state("ActiveState=inactive\n").is_err());
    }

    #[test]
    fn active_init_hash_is_exact_file_content_identity() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("runtime-init.toml");
        fs::write(&path, "candidate-config").unwrap();

        assert_eq!(
            file_hash(&path).unwrap(),
            blake3::hash(b"candidate-config").to_hex().to_string()
        );
        assert!(file_hash(&root.path().join("missing.toml")).is_err());
    }

    #[test]
    fn missing_service_is_unloaded_and_has_no_owned_process() {
        let root = tempfile::tempdir().unwrap();
        let args = service_args(root.path().join("runtime-init.toml"));

        assert!(!platform_loaded_with_timeout(&args, Duration::from_millis(750)).unwrap());
        assert_eq!(
            platform_process_id_with_timeout(&args, Duration::from_millis(750)).unwrap(),
            None
        );
        assert!(platform_stop_with_timeout(&args, Duration::from_secs(30)).is_ok());
        assert!(wait_for_service_unloaded(&args, Duration::from_millis(750)).is_ok());
    }

    #[test]
    fn usage_exposes_all_governed_lifecycle_operations() {
        let text = usage();
        for operation in ["start", "stop", "status", "reload"] {
            assert!(text.contains(operation));
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn installed_plist_uses_service_label() {
        let args = RuntimeV3ServiceArgs {
            init: PathBuf::from("/tmp/runtime-init.toml"),
            candidate: None,
            plist: None,
            label: "com.agentlogic.test-runtime".into(),
            json: false,
        };
        let path = installed_launchd_plist(&args).unwrap();
        assert!(path.ends_with("Library/LaunchAgents/com.agentlogic.test-runtime.plist"));
    }
}
