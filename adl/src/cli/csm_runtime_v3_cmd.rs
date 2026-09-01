use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use adl_runtime_kernel::RuntimeInitConfig;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_LABEL: &str = "com.agentlogic.adl-runtime-v3";

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

fn start(args: &RuntimeV3ServiceArgs) -> Result<()> {
    if args.candidate.is_some() {
        return Err(anyhow!("--candidate is valid only with runtime-v3 reload"));
    }
    reconcile_interrupted_reload(args)?;
    let init = validated_init(&args.init)?;
    if owned_runtime_readiness(args, &init).is_ok() {
        return emit_status(args, &init, "start", true);
    }
    start_clean(args, &init)?;
    emit_status(args, &init, "start", true)
}

fn reload(args: &RuntimeV3ServiceArgs) -> Result<()> {
    reconcile_interrupted_reload(args)?;
    let current = validated_init(&args.init)?;
    let Some(candidate_path) = args.candidate.as_ref() else {
        start_clean(args, &current)?;
        return emit_status(args, &current, "reload", true);
    };
    let candidate = validated_init(candidate_path)?;
    stop_and_wait(args, &current)?;
    let backup = match replace_config_with_candidate(&args.init, candidate_path) {
        Ok(backup) => backup,
        Err(error) => {
            start_and_wait(args, &current)
                .context("Runtime v3 did not recover after candidate install failed")?;
            return Err(error);
        }
    };
    let reload_result = start_and_wait(args, &candidate);
    if let Err(reload_error) = reload_result {
        stop_and_wait(args, &candidate)
            .context("stop failed Runtime v3 candidate before config rollback")?;
        restore_last_known_good(&args.init, &backup).with_context(|| {
            format!(
                "restore last-known-good Runtime v3 init {}",
                args.init.display()
            )
        })?;
        start_and_wait(args, &current)
            .context("Runtime v3 did not recover after config rollback")?;
        return Err(anyhow!(
            "Runtime v3 candidate reload failed and last-known-good configuration was restored: {reload_error}"
        ));
    }
    commit_candidate(&args.init, &backup)?;
    emit_status(args, &candidate, "reload", true)
}

fn replace_config_with_candidate(active: &Path, candidate: &Path) -> Result<PathBuf> {
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
    if backup.exists() || staged.exists() {
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
    sync_parent(active)?;
    if let Err(error) = copy_create_new(candidate, &staged)
        .and_then(|_| fs::rename(&staged, active))
        .map_err(anyhow::Error::from)
        .and_then(|_| sync_parent(active))
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
    reconcile_interrupted_reload_with(
        active,
        |backup| validated_init(backup).map(|_| ()),
        || {
            validated_init(active)
                .and_then(|init| owned_runtime_readiness(args, &init).map(|_| ()))
                .is_ok()
        },
        || {
            if platform_loaded(args) {
                let guardian_process_id = platform_process_id(args);
                platform_stop(args)?;
                if let Ok(init) = validated_init(active) {
                    wait_for_stopped(args, &init, guardian_process_id, Duration::from_secs(15))?;
                } else {
                    wait_for_service_unloaded(args, Duration::from_secs(15))?;
                }
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
    let guardian_process_id = platform_process_id(args);
    platform_stop(args)?;
    wait_for_stopped(args, &init, guardian_process_id, Duration::from_secs(15))?;
    emit_status(args, &init, "stop", false)
}

fn start_clean(args: &RuntimeV3ServiceArgs, init: &RuntimeInitConfig) -> Result<()> {
    stop_and_wait(args, init)?;
    start_and_wait(args, init)
}

fn stop_and_wait(args: &RuntimeV3ServiceArgs, current: &RuntimeInitConfig) -> Result<()> {
    let guardian_process_id = platform_process_id(args);
    platform_stop(args)?;
    wait_for_stopped(args, current, guardian_process_id, Duration::from_secs(15))
}

fn start_and_wait(args: &RuntimeV3ServiceArgs, next: &RuntimeInitConfig) -> Result<()> {
    platform_start(args)?;
    wait_for_listener(args, next, Duration::from_secs(15))
}

fn status(args: &RuntimeV3ServiceArgs, operation: &'static str) -> Result<()> {
    let (backup, staged) = reload_transaction_paths(&args.init)?;
    if backup.exists() || staged.exists() {
        return Err(anyhow!(
            "Runtime v3 reload transaction is incomplete; run start to restore last-known-good configuration"
        ));
    }
    let init = validated_init(&args.init)?;
    let loaded = platform_loaded(args);
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
    let loaded = platform_loaded(args);
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
    let service_process_id = platform_process_id(args)
        .ok_or_else(|| anyhow!("Runtime v3 service manager has no live process identity"))?;
    let readiness = runtime_readiness(init)?;
    let active_init_hash = file_hash(&args.init)?;
    validate_owned_readiness(service_process_id, &active_init_hash, &readiness)?;
    Ok(readiness)
}

fn validate_owned_readiness(
    service_process_id: u32,
    active_init_hash: &str,
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
    Ok(())
}

fn runtime_readiness(init: &RuntimeInitConfig) -> Result<RuntimeReadinessProbe> {
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
        .timeout(Duration::from_millis(750))
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
    {
        return Err(anyhow!("Runtime v3 readiness response is not healthy"));
    }
    Ok(())
}

fn wait_for_listener(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    timeout: Duration,
) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if owned_runtime_readiness(args, init).is_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(anyhow!(
        "Runtime v3 listener {} did not become ready within {} seconds",
        init.api.address,
        timeout.as_secs()
    ))
}

fn wait_for_stopped(
    args: &RuntimeV3ServiceArgs,
    init: &RuntimeInitConfig,
    stopped_guardian_process_id: Option<u32>,
    timeout: Duration,
) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        let stopped_listener_gone = runtime_readiness(init).map_or(true, |readiness| {
            stopped_guardian_process_id != Some(readiness.guardian_process_id)
        });
        if platform_stopped(args)? && stopped_listener_gone {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(anyhow!(
        "Runtime v3 service did not stop within {} seconds",
        timeout.as_secs()
    ))
}

fn wait_for_service_unloaded(args: &RuntimeV3ServiceArgs, timeout: Duration) -> Result<()> {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if platform_stopped(args)? {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(anyhow!(
        "Runtime v3 service manager did not unload within {} seconds",
        timeout.as_secs()
    ))
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
fn platform_start(args: &RuntimeV3ServiceArgs) -> Result<()> {
    if let Some(source) = args.plist.as_ref() {
        let plist = install_launchd_plist(args, source)?;
        run(Command::new("launchctl")
            .args(["bootstrap", &launchd_domain()])
            .arg(plist))?;
    } else if !platform_loaded(args) {
        let plist = installed_launchd_plist(args)?;
        if !plist.is_file() {
            return Err(anyhow!(
                "Runtime v3 service is not loaded and installed plist is missing; --plist is required"
            ));
        }
        run(Command::new("launchctl")
            .args(["bootstrap", &launchd_domain()])
            .arg(plist))?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn platform_stop(args: &RuntimeV3ServiceArgs) -> Result<()> {
    if !platform_loaded(args) {
        return Ok(());
    }
    run(Command::new("launchctl").args(["bootout", &launchd_target(args)]))
}

#[cfg(target_os = "macos")]
fn platform_loaded(args: &RuntimeV3ServiceArgs) -> bool {
    Command::new("launchctl")
        .args(["print", &launchd_target(args)])
        .output()
        .is_ok_and(|output| output.status.success())
}

#[cfg(target_os = "macos")]
fn platform_stopped(args: &RuntimeV3ServiceArgs) -> Result<bool> {
    Ok(!platform_loaded(args))
}

#[cfg(target_os = "macos")]
fn platform_process_id(args: &RuntimeV3ServiceArgs) -> Option<u32> {
    let output = Command::new("launchctl")
        .args(["print", &launchd_target(args)])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_launchctl_process_id(std::str::from_utf8(&output.stdout).ok()?)
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
fn platform_start(args: &RuntimeV3ServiceArgs) -> Result<()> {
    run(Command::new("systemctl").args(["start", &systemd_unit(args)]))
}

#[cfg(target_os = "linux")]
fn platform_stop(args: &RuntimeV3ServiceArgs) -> Result<()> {
    if systemd_service_state(args)?.is_stopped() {
        return Ok(());
    }
    run(Command::new("systemctl").args(["stop", &systemd_unit(args)]))
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
fn systemd_service_state(args: &RuntimeV3ServiceArgs) -> Result<SystemdServiceState> {
    let unit = systemd_unit(args);
    let output = Command::new("systemctl")
        .args([
            "show",
            "--property=LoadState",
            "--property=ActiveState",
            &unit,
        ])
        .output()
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
fn platform_stopped(args: &RuntimeV3ServiceArgs) -> Result<bool> {
    Ok(systemd_service_state(args)?.is_stopped())
}

#[cfg(target_os = "linux")]
fn platform_loaded(args: &RuntimeV3ServiceArgs) -> bool {
    Command::new("systemctl")
        .args(["is-active", "--quiet", &systemd_unit(args)])
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(target_os = "linux")]
fn platform_process_id(args: &RuntimeV3ServiceArgs) -> Option<u32> {
    let output = Command::new("systemctl")
        .args([
            "show",
            "--property",
            "MainPID",
            "--value",
            &systemd_unit(args),
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    std::str::from_utf8(&output.stdout)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
        .filter(|process_id| *process_id > 0)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_start(_args: &RuntimeV3ServiceArgs) -> Result<()> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_stop(_args: &RuntimeV3ServiceArgs) -> Result<()> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_loaded(_args: &RuntimeV3ServiceArgs) -> bool {
    false
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_stopped(_args: &RuntimeV3ServiceArgs) -> Result<bool> {
    Err(anyhow!(
        "csm runtime-v3 service control supports launchd and systemd"
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn platform_process_id(_args: &RuntimeV3ServiceArgs) -> Option<u32> {
    None
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

fn run(command: &mut Command) -> Result<()> {
    let rendered = format!("{command:?}");
    let status = command
        .status()
        .with_context(|| format!("run service-manager command {rendered}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "service-manager command failed: {rendered}: {status}"
        ))
    }
}

pub(crate) fn usage() -> &'static str {
    "csm runtime-v3 start|stop|status|reload --init <absolute-runtime-init.toml> [--candidate <absolute-candidate-init.toml>] [--plist <absolute-launchd-plist>] [--label <service-label>] [--json]"
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn parser_requires_absolute_init() {
        let error = parse_args(&["--init".into(), "relative.toml".into()]).unwrap_err();
        assert!(error.to_string().contains("must be absolute"));
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

        let backup = replace_config_with_candidate(&active, &candidate).unwrap();

        assert_eq!(fs::read_to_string(&active).unwrap(), "candidate");
        assert_eq!(fs::read_to_string(backup).unwrap(), "current");
    }

    #[test]
    fn interrupted_reload_restores_last_known_good_before_next_start() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::write(&candidate, "candidate").unwrap();

        let backup = replace_config_with_candidate(&active, &candidate).unwrap();
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
        assert!(!backup.exists());
    }

    #[test]
    fn interrupted_reload_commits_candidate_when_owned_readiness_matches_active_hash() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        let candidate = root.path().join("runtime-init.next.toml");
        fs::write(&active, "current").unwrap();
        fs::write(&candidate, "candidate").unwrap();
        let backup = replace_config_with_candidate(&active, &candidate).unwrap();
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

        commit_candidate(&active, &backup).unwrap();

        assert!(!backup.exists());
        assert!(!staged.exists());
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

        let error = replace_config_with_candidate(&active, &candidate).unwrap_err();

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

        assert!(replace_config_with_candidate(&active, &candidate).is_err());
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

        assert!(replace_config_with_candidate(&active, &candidate).is_err());
        assert_eq!(fs::read_to_string(&active).unwrap(), "current");
        assert!(!backup.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn transaction_helpers_reject_ambiguous_or_invalid_paths() {
        let root = tempfile::tempdir().unwrap();
        let active = root.path().join("runtime-init.toml");
        fs::write(&active, "current").unwrap();
        assert!(replace_config_with_candidate(&active, &active).is_err());
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
        reconcile_interrupted_reload_with(&active, |_| Ok(()), || false, || Ok(())).unwrap();
        assert_eq!(fs::read_to_string(&active).unwrap(), "known-good");
        assert!(!backup.exists());
        assert!(!staged.exists());
    }

    #[test]
    fn valid_init_status_and_readiness_fail_closed_without_a_service() {
        let root = tempfile::tempdir().unwrap();
        let (path, init) = write_valid_init(root.path());
        let mut args = service_args(path.clone());

        assert!(runtime_readiness(&init).is_err());
        assert!(owned_runtime_readiness(&args, &init).is_err());
        assert!(emit_status(&args, &init, "status", false).is_ok());
        args.json = true;
        assert!(emit_status(&args, &init, "status", true).is_err());
        assert!(status(&args, "status").is_err());
        assert!(wait_for_listener(&args, &init, Duration::ZERO).is_err());
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

        let error = runtime_readiness(&init).unwrap_err().to_string();
        assert!(error.contains("query Runtime v3 readiness"));
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
        assert!(run(&mut Command::new("true")).is_ok());
        assert!(run(&mut Command::new("false")).is_err());
        assert!(run(&mut Command::new("adl-command-that-does-not-exist")).is_err());
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
        };
        assert!(validate_readiness_probe(&healthy).is_ok());
        assert!(validate_owned_readiness(41, &"a".repeat(64), &healthy).is_ok());
        assert!(validate_owned_readiness(99, &"a".repeat(64), &healthy).is_err());
        assert!(validate_owned_readiness(41, &"b".repeat(64), &healthy).is_err());
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
            },
            RuntimeReadinessProbe {
                guardian_process_id: 0,
                ..healthy.clone()
            },
            RuntimeReadinessProbe {
                active_init_hash: "invalid".into(),
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

        assert!(!platform_loaded(&args));
        assert_eq!(platform_process_id(&args), None);
        assert!(platform_stop(&args).is_ok());
        assert!(wait_for_service_unloaded(&args, Duration::from_millis(5)).is_ok());
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
