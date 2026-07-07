use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use ::adl::long_lived_agent;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cli::observability;

const SERVICE_MANIFEST_SCHEMA: &str = "adl.csm.service_manifest.v1";
const SERVICE_STATUS_SCHEMA: &str = "adl.csm.service_status.v1";
const DEFAULT_LABEL: &str = "com.agentlogic.csm.runtime";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ServiceManager {
    Launchd,
    Local,
}

impl ServiceManager {
    fn parse(raw: &str) -> Result<Self> {
        match raw {
            "launchd" => Ok(Self::Launchd),
            "local" => Ok(Self::Local),
            other => Err(anyhow!(
                "unknown CSM service manager '{other}' (expected launchd or local)"
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Launchd => "launchd",
            Self::Local => "local",
        }
    }
}

#[derive(Debug, Clone)]
struct ServiceArgs {
    service_root: PathBuf,
    spec: Option<PathBuf>,
    label: String,
    csm_bin: Option<PathBuf>,
    manager: ServiceManager,
    checkpoint_interval_secs: u64,
    interval_secs: Option<u64>,
    recover_stale_lease: bool,
    no_sleep: bool,
    otlp_endpoint: Option<String>,
    otlp_timeout_ms: Option<u64>,
    json: bool,
}

impl Default for ServiceArgs {
    fn default() -> Self {
        Self {
            service_root: PathBuf::from("out/csm-service"),
            spec: None,
            label: DEFAULT_LABEL.to_string(),
            csm_bin: None,
            manager: ServiceManager::Launchd,
            checkpoint_interval_secs: 3,
            interval_secs: None,
            recover_stale_lease: true,
            no_sleep: false,
            otlp_endpoint: None,
            otlp_timeout_ms: env::var("ADL_OTEL_EXPORTER_TIMEOUT_MS")
                .or_else(|_| env::var("OTEL_EXPORTER_OTLP_TIMEOUT_MS"))
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
                .filter(|value| *value > 0),
            json: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceManifest {
    schema: String,
    label: String,
    manager: ServiceManager,
    runtime_owner: String,
    #[serde(default)]
    restart_policy: String,
    #[serde(default)]
    service_mode: String,
    csm_bin: PathBuf,
    spec: PathBuf,
    service_root: PathBuf,
    plist: PathBuf,
    pid_file: PathBuf,
    stdout_log: PathBuf,
    stderr_log: PathBuf,
    observability_log: PathBuf,
    otel_log: PathBuf,
    otel_status: PathBuf,
    startup_ledger: PathBuf,
    daemon_status: PathBuf,
    continuity_checkpoint: PathBuf,
    continuity_replay_manifest: PathBuf,
    operator_events: PathBuf,
    checkpoint_interval_secs: u64,
    interval_secs: Option<u64>,
    recover_stale_lease: bool,
    no_sleep: bool,
    otlp_endpoint: Option<String>,
    otlp_timeout_ms: Option<u64>,
    launchd_domain: String,
    unsupported_permanence_claims: Vec<String>,
    created_at: String,
}

#[derive(Debug, Clone, Serialize)]
struct ServiceStatus {
    schema: &'static str,
    label: String,
    manager: String,
    runtime_owner: &'static str,
    restart_policy: String,
    service_mode: String,
    service_state: String,
    pid: Option<u32>,
    pid_liveness: String,
    broad_process_scan: bool,
    uses_ps: bool,
    manifest_ref: String,
    plist_ref: String,
    daemon_status_ref: String,
    continuity_checkpoint_ref: String,
    observability_log_ref: String,
    otel_log_ref: String,
    otel_status_ref: String,
    startup_ledger_ref: String,
    startup_classification: String,
    first_daemon_record_observed: bool,
    continuity_checkpoint_observed: bool,
    cycle_ledger_observed: bool,
    otlp_exporter_configured: bool,
    otlp_endpoint_ref: Option<&'static str>,
    last_action: String,
    last_error: Option<String>,
    unsupported_permanence_claims: Vec<String>,
    updated_at: String,
}

#[derive(Debug, Clone, Copy)]
struct LocalStartOutcome {
    pid: u32,
    reused_existing: bool,
}

pub(crate) fn real_service(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(anyhow!(
            "csm service requires subcommand: install | start | stop | status | remove"
        ));
    };

    match subcommand {
        "install" => install(&args[1..]),
        "start" => start(&args[1..]),
        "stop" => stop(&args[1..]),
        "status" => status(&args[1..]),
        "remove" => remove(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", service_usage());
            Ok(())
        }
        other => Err(anyhow!(
            "unknown csm service subcommand '{other}' (expected install, start, stop, status, remove)"
        )),
    }
}

fn install(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, true)?;
    let spec = parsed
        .spec
        .clone()
        .ok_or_else(|| anyhow!("csm service install requires --spec <agent-spec.yaml>"))?;
    let spec = absolutize(&spec)?;
    let service_root = absolutize_create(&parsed.service_root)?;
    let csm_bin = parsed
        .csm_bin
        .clone()
        .map(|path| absolutize(&path))
        .transpose()?
        .unwrap_or_else(current_exe_or_csm);
    let manifest = build_manifest(parsed, service_root, spec, csm_bin)?;
    fs::create_dir_all(manifest.service_root.join("logs"))?;
    fs::create_dir_all(manifest.service_root.join("state"))?;
    write_launchd_plist(&manifest)?;
    write_json_pretty(&manifest_path(&manifest.service_root), &manifest)?;
    let status = service_status(&manifest, "installed", "install", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn start(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    let start_requested_at = Utc::now();
    record_startup_event(&manifest, "start_requested", "started", None, None)?;
    let mut local_start = None;
    match manifest.manager {
        ServiceManager::Local => {
            let outcome = start_local(&manifest)?;
            local_start = Some(outcome);
            record_startup_event(
                &manifest,
                if outcome.reused_existing {
                    "local_already_running"
                } else {
                    "local_spawn"
                },
                "started",
                Some(outcome.pid),
                None,
            )?;
        }
        ServiceManager::Launchd => {
            let args = [
                "bootstrap",
                &manifest.launchd_domain,
                path_str(&manifest.plist)?,
            ];
            if let Err(err) = run_launchctl(&args) {
                let classification = "launchd_bootstrap_failed";
                record_startup_event(&manifest, classification, "failed", None, Some(&err))?;
                let status =
                    service_status(&manifest, "startup_failed", "start", Some(err.to_string()))?;
                write_status(&manifest, &status)?;
                return Err(err);
            }
            record_startup_event(&manifest, "launchd_bootstrap", "requested", None, None)?;
        }
    }
    let observation = observe_startup(&manifest, local_start, start_requested_at)?;
    let state = if observation.healthy {
        "running"
    } else {
        "startup_failed"
    };
    record_startup_event(
        &manifest,
        observation.classification,
        if observation.healthy {
            "completed"
        } else {
            "failed"
        },
        observation.pid,
        None,
    )?;
    let status = service_status(&manifest, state, "start", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    if !observation.healthy {
        return Err(anyhow!(
            "csm service startup failed before first daemon record: {}",
            observation.classification
        ));
    }
    Ok(())
}

fn stop(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    match manifest.manager {
        ServiceManager::Local => stop_local(&manifest)?,
        ServiceManager::Launchd => run_launchctl(&[
            "bootout",
            &manifest.launchd_domain,
            path_str(&manifest.plist)?,
        ])
        .or_else(|err| {
            let status = service_status(&manifest, "blocked", "stop", Some(err.to_string()))?;
            write_status(&manifest, &status)?;
            Err(err)
        })?,
    }
    let status = service_status(&manifest, "stopped_or_requested", "stop", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn status(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    let status = service_status(&manifest, "observed", "status", None)?;
    write_status(&manifest, &status)?;
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn remove(args: &[String]) -> Result<()> {
    let parsed = parse_service_args(args, false)?;
    let service_root = absolutize(&parsed.service_root)?;
    let manifest = read_manifest(&service_root)?;
    if manifest.manager == ServiceManager::Local {
        let _ = stop_local(&manifest);
    }
    let status = service_status(&manifest, "removed", "remove", None)?;
    write_status(&manifest, &status)?;
    for path in [&manifest.plist, &manifest.pid_file] {
        if path.exists() {
            fs::remove_file(path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    print_status(&status, manifest.service_root.as_path(), args);
    Ok(())
}

fn parse_service_args(args: &[String], require_spec: bool) -> Result<ServiceArgs> {
    let mut parsed = ServiceArgs::default();
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--service-root" => {
                parsed.service_root = PathBuf::from(required_value(args, i, "--service-root")?);
                i += 1;
            }
            "--spec" => {
                parsed.spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--label" => {
                parsed.label = required_value(args, i, "--label")?.to_string();
                validate_label(&parsed.label)?;
                i += 1;
            }
            "--csm-bin" => {
                parsed.csm_bin = Some(PathBuf::from(required_value(args, i, "--csm-bin")?));
                i += 1;
            }
            "--manager" => {
                parsed.manager = ServiceManager::parse(required_value(args, i, "--manager")?)?;
                i += 1;
            }
            "--checkpoint-interval-secs" => {
                parsed.checkpoint_interval_secs = parse_positive_u64(
                    required_value(args, i, "--checkpoint-interval-secs")?,
                    "--checkpoint-interval-secs",
                )?;
                i += 1;
            }
            "--interval-secs" => {
                parsed.interval_secs = Some(parse_positive_u64(
                    required_value(args, i, "--interval-secs")?,
                    "--interval-secs",
                )?);
                i += 1;
            }
            "--no-recover-stale-lease" => parsed.recover_stale_lease = false,
            "--no-sleep" => parsed.no_sleep = true,
            "--otlp-endpoint" => {
                let endpoint = required_value(args, i, "--otlp-endpoint")?
                    .trim()
                    .to_string();
                validate_otlp_endpoint(&endpoint)?;
                parsed.otlp_endpoint = Some(endpoint);
                i += 1;
            }
            "--otlp-timeout-ms" => {
                parsed.otlp_timeout_ms = Some(parse_positive_u64(
                    required_value(args, i, "--otlp-timeout-ms")?,
                    "--otlp-timeout-ms",
                )?);
                i += 1;
            }
            "--json" => parsed.json = true,
            "--help" | "-h" => {
                println!("{}", service_usage());
                std::process::exit(0);
            }
            other => return Err(anyhow!("unknown csm service arg: {other}")),
        }
        i += 1;
    }
    if require_spec && parsed.spec.is_none() {
        return Err(anyhow!(
            "csm service install requires --spec <agent-spec.yaml>"
        ));
    }
    if parsed.otlp_endpoint.is_none() {
        parsed.otlp_endpoint = env_otlp_endpoint()?;
    }
    Ok(parsed)
}

fn build_manifest(
    parsed: ServiceArgs,
    service_root: PathBuf,
    spec: PathBuf,
    csm_bin: PathBuf,
) -> Result<ServiceManifest> {
    let loaded = long_lived_agent::load_spec(&spec)?;
    let state_root = loaded.state_root;
    let launchd_domain = format!("gui/{}", current_uid());
    Ok(ServiceManifest {
        schema: SERVICE_MANIFEST_SCHEMA.to_string(),
        label: parsed.label,
        manager: parsed.manager,
        runtime_owner: "csm".to_string(),
        restart_policy: service_restart_policy(parsed.manager, parsed.no_sleep),
        service_mode: service_mode(parsed.manager, parsed.no_sleep),
        csm_bin,
        spec,
        service_root: service_root.clone(),
        plist: service_root.join("csm.launchd.plist"),
        pid_file: service_root.join("csm-service.pid"),
        stdout_log: service_root.join("logs/csm.stdout.log"),
        stderr_log: service_root.join("logs/csm.stderr.log"),
        observability_log: service_root.join("logs/observability.log"),
        otel_log: service_root.join("logs/otel.jsonl"),
        otel_status: service_root.join("logs/otel_status.json"),
        startup_ledger: service_root.join("logs/startup_ledger.jsonl"),
        daemon_status: state_root.join("daemon_status.json"),
        continuity_checkpoint: state_root.join("continuity_checkpoint.json"),
        continuity_replay_manifest: state_root.join("continuity_replay_manifest.json"),
        operator_events: state_root.join("operator_events.jsonl"),
        checkpoint_interval_secs: parsed.checkpoint_interval_secs,
        interval_secs: parsed.interval_secs,
        recover_stale_lease: parsed.recover_stale_lease,
        no_sleep: parsed.no_sleep,
        otlp_endpoint: parsed.otlp_endpoint,
        otlp_timeout_ms: parsed.otlp_timeout_ms,
        launchd_domain,
        unsupported_permanence_claims: unsupported_permanence_claims(),
        created_at: Utc::now().to_rfc3339(),
    })
}

fn write_launchd_plist(manifest: &ServiceManifest) -> Result<()> {
    let mut args = vec![
        manifest.csm_bin.display().to_string(),
        "daemon".to_string(),
        "--spec".to_string(),
        manifest.spec.display().to_string(),
        "--checkpoint-interval-secs".to_string(),
        manifest.checkpoint_interval_secs.to_string(),
        "--json".to_string(),
    ];
    if let Some(interval_secs) = manifest.interval_secs {
        args.push("--interval-secs".to_string());
        args.push(interval_secs.to_string());
    }
    if manifest.recover_stale_lease {
        args.push("--recover-stale-lease".to_string());
    }
    if manifest.no_sleep {
        args.push("--no-sleep".to_string());
    }
    let program_args = args
        .iter()
        .map(|arg| format!("    <string>{}</string>", xml_escape(arg)))
        .collect::<Vec<_>>()
        .join("\n");
    let otlp_env = match manifest.otlp_endpoint.as_deref() {
        Some(endpoint) => {
            let mut block = format!(
                "    <key>ADL_OTEL_EXPORTER_OTLP_ENDPOINT</key>\n    <string>{}</string>\n",
                xml_escape(endpoint)
            );
            if let Some(timeout_ms) = manifest.otlp_timeout_ms {
                block.push_str(&format!(
                    "    <key>ADL_OTEL_EXPORTER_TIMEOUT_MS</key>\n    <string>{timeout_ms}</string>\n"
                ));
            }
            block
        }
        None => String::new(),
    };
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>{label}</string>
  <key>ProgramArguments</key>
  <array>
{program_args}
  </array>
  <key>KeepAlive</key>
  <true/>
  <key>RunAtLoad</key>
  <true/>
  <key>StandardOutPath</key>
  <string>{stdout}</string>
  <key>StandardErrorPath</key>
  <string>{stderr}</string>
  <key>EnvironmentVariables</key>
  <dict>
    <key>ADL_OBSERVABILITY_LOG</key>
    <string>{observability}</string>
    <key>ADL_OBSERVABILITY_STDERR</key>
    <string>0</string>
    <key>ADL_OTEL_LOG</key>
    <string>{otel_log}</string>
    <key>ADL_OTEL_STATUS</key>
    <string>{otel_status}</string>
{otlp_env}  </dict>
</dict>
</plist>
"#,
        label = xml_escape(&manifest.label),
        stdout = xml_escape(&manifest.stdout_log.display().to_string()),
        stderr = xml_escape(&manifest.stderr_log.display().to_string()),
        observability = xml_escape(&manifest.observability_log.display().to_string()),
        otel_log = xml_escape(&manifest.otel_log.display().to_string()),
        otel_status = xml_escape(&manifest.otel_status.display().to_string()),
        otlp_env = otlp_env,
    );
    fs::write(&manifest.plist, plist).with_context(|| format!("write {}", manifest.plist.display()))
}

fn start_local(manifest: &ServiceManifest) -> Result<LocalStartOutcome> {
    if manifest.pid_file.exists() {
        let pid = read_pid_file(&manifest.pid_file)?;
        if pid_liveness(pid) != "live_pid" {
            let _ = fs::remove_file(&manifest.pid_file);
        } else if daemon_status_matches_pid_and_spec(manifest, pid)? {
            return Ok(LocalStartOutcome {
                pid,
                reused_existing: true,
            });
        } else {
            return Err(anyhow!(
                "csm service start refused live but unverified pid metadata for pid {pid}; remove stale metadata only after confirming ownership"
            ));
        }
    }
    let stdout = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stdout_log)?;
    let stderr = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.stderr_log)?;
    let mut command = Command::new(&manifest.csm_bin);
    command
        .arg("daemon")
        .arg("--spec")
        .arg(&manifest.spec)
        .arg("--checkpoint-interval-secs")
        .arg(manifest.checkpoint_interval_secs.to_string())
        .arg("--json")
        .env("ADL_OBSERVABILITY_LOG", &manifest.observability_log)
        .env("ADL_OBSERVABILITY_STDERR", "0")
        .env("ADL_OTEL_LOG", &manifest.otel_log)
        .env("ADL_OTEL_STATUS", &manifest.otel_status)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr));
    if let Some(endpoint) = manifest.otlp_endpoint.as_deref() {
        command.env("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint);
    }
    if let Some(timeout_ms) = manifest.otlp_timeout_ms {
        command.env("ADL_OTEL_EXPORTER_TIMEOUT_MS", timeout_ms.to_string());
    }
    if let Some(interval_secs) = manifest.interval_secs {
        command
            .arg("--interval-secs")
            .arg(interval_secs.to_string());
    }
    if manifest.recover_stale_lease {
        command.arg("--recover-stale-lease");
    }
    if manifest.no_sleep {
        command.arg("--no-sleep");
    }
    let child = command.spawn().context("spawn local csm daemon service")?;
    let pid = child.id();
    fs::write(&manifest.pid_file, pid.to_string())?;
    Ok(LocalStartOutcome {
        pid,
        reused_existing: false,
    })
}

fn stop_local(manifest: &ServiceManifest) -> Result<()> {
    let _ = long_lived_agent::stop(&manifest.spec, "csm service stop requested");
    if !manifest.pid_file.exists() {
        return Ok(());
    }
    let pid = read_pid_file(&manifest.pid_file)?;
    if daemon_status_matches_pid_and_spec(manifest, pid)? {
        for _ in 0..30 {
            if pid_liveness(pid) != "live_pid" {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
    if pid_liveness(pid) != "live_pid" {
        let _ = fs::remove_file(&manifest.pid_file);
    }
    Ok(())
}

fn service_status(
    manifest: &ServiceManifest,
    state: &str,
    action: &str,
    err: Option<String>,
) -> Result<ServiceStatus> {
    let pid = if manifest.pid_file.exists() {
        read_pid_file(&manifest.pid_file).ok()
    } else {
        read_daemon_pid(&manifest.daemon_status).ok().flatten()
    };
    let pid_liveness = pid
        .map(pid_liveness)
        .unwrap_or_else(|| "missing_pid_metadata".to_string());
    let daemon_record_not_before =
        last_failed_startup_at(manifest).unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
    let first_daemon_record_observed =
        verified_daemon_record_observed(manifest, pid, daemon_record_not_before);
    Ok(ServiceStatus {
        schema: SERVICE_STATUS_SCHEMA,
        label: manifest.label.clone(),
        manager: manifest.manager.as_str().to_string(),
        runtime_owner: "csm",
        restart_policy: manifest.restart_policy.clone(),
        service_mode: manifest.service_mode.clone(),
        service_state: state.to_string(),
        pid,
        pid_liveness: pid_liveness.clone(),
        broad_process_scan: false,
        uses_ps: false,
        manifest_ref: ref_for(
            &manifest.service_root,
            &manifest_path(&manifest.service_root),
        ),
        plist_ref: ref_for(&manifest.service_root, &manifest.plist),
        daemon_status_ref: ref_for(&manifest.service_root, &manifest.daemon_status),
        continuity_checkpoint_ref: ref_for(&manifest.service_root, &manifest.continuity_checkpoint),
        observability_log_ref: ref_for(&manifest.service_root, &manifest.observability_log),
        otel_log_ref: ref_for(&manifest.service_root, &manifest.otel_log),
        otel_status_ref: ref_for(&manifest.service_root, &manifest.otel_status),
        startup_ledger_ref: ref_for(&manifest.service_root, &manifest.startup_ledger),
        startup_classification: startup_classification(
            manifest,
            pid.as_ref().copied(),
            &pid_liveness,
            first_daemon_record_observed,
        ),
        first_daemon_record_observed,
        continuity_checkpoint_observed: manifest.continuity_checkpoint.exists(),
        cycle_ledger_observed: cycle_ledger_path(manifest).exists(),
        otlp_exporter_configured: manifest.otlp_endpoint.is_some(),
        otlp_endpoint_ref: manifest.otlp_endpoint.as_ref().map(|_| "<configured>"),
        last_action: action.to_string(),
        last_error: err,
        unsupported_permanence_claims: manifest.unsupported_permanence_claims.clone(),
        updated_at: Utc::now().to_rfc3339(),
    })
}

fn read_manifest(service_root: &Path) -> Result<ServiceManifest> {
    let path = manifest_path(service_root);
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let manifest: ServiceManifest =
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    Ok(normalize_service_manifest_metadata(manifest))
}

fn write_status(manifest: &ServiceManifest, status: &ServiceStatus) -> Result<()> {
    write_json_pretty(&manifest.service_root.join("service_status.json"), status)
}

fn print_status(status: &ServiceStatus, service_root: &Path, args: &[String]) {
    let json = args.iter().any(|arg| arg == "--json");
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(status).expect("serialize service status")
        );
    } else {
        println!(
            "csm service {} manager={} pid_liveness={} root={}",
            status.service_state,
            status.manager,
            status.pid_liveness,
            service_root.display()
        );
    }
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let rendered = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{rendered}\n")).with_context(|| format!("write {}", path.display()))
}

#[derive(Debug, Clone, Copy)]
struct StartupObservation {
    pid: Option<u32>,
    classification: &'static str,
    healthy: bool,
}

fn observe_startup(
    manifest: &ServiceManifest,
    local_start: Option<LocalStartOutcome>,
    start_requested_at: DateTime<Utc>,
) -> Result<StartupObservation> {
    let deadline = startup_observation_attempts();
    let not_before = if local_start
        .map(|outcome| outcome.reused_existing)
        .unwrap_or(false)
    {
        DateTime::<Utc>::UNIX_EPOCH
    } else {
        start_requested_at
    };
    let mut last = StartupObservation {
        pid: local_start.map(|outcome| outcome.pid),
        classification: "startup_missing_daemon_record",
        healthy: false,
    };
    for attempt in 0..deadline {
        let pid = local_start
            .map(|outcome| outcome.pid)
            .or_else(|| fresh_daemon_pid_after(manifest, start_requested_at));
        let pid_liveness = pid
            .map(pid_liveness)
            .unwrap_or_else(|| "missing_pid_metadata".to_string());
        let first_daemon_record_observed =
            verified_daemon_record_observed(manifest, pid, not_before);
        let cycle_ledger_observed = cycle_ledger_path(manifest).exists();
        let continuity_checkpoint_observed = manifest.continuity_checkpoint.exists();
        let classification = classify_startup(
            pid.as_ref().copied(),
            &pid_liveness,
            first_daemon_record_observed,
            cycle_ledger_observed,
            continuity_checkpoint_observed,
        );
        last = StartupObservation {
            pid,
            classification,
            healthy: classification == "startup_first_daemon_record_observed",
        };
        record_startup_probe(
            manifest,
            StartupProbeRecord {
                attempt: attempt + 1,
                pid,
                pid_liveness: &pid_liveness,
                first_daemon_record_observed,
                cycle_ledger_observed,
                continuity_checkpoint_observed,
                classification,
            },
        )?;
        if last.healthy || classification == "startup_stale_before_first_daemon_record" {
            return Ok(last);
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(last)
}

fn startup_observation_attempts() -> u32 {
    env::var("ADL_CSM_SERVICE_STARTUP_ATTEMPTS")
        .ok()
        .and_then(|raw| raw.parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(20)
}

fn cycle_ledger_path(manifest: &ServiceManifest) -> PathBuf {
    manifest
        .continuity_checkpoint
        .parent()
        .unwrap_or(manifest.service_root.as_path())
        .join("cycle_ledger.jsonl")
}

fn startup_classification(
    manifest: &ServiceManifest,
    pid: Option<u32>,
    pid_liveness: &str,
    first_daemon_record_observed: bool,
) -> String {
    let current = classify_startup(
        pid,
        pid_liveness,
        first_daemon_record_observed,
        cycle_ledger_path(manifest).exists(),
        manifest.continuity_checkpoint.exists(),
    );
    if current == "startup_first_daemon_record_observed" {
        return current.to_string();
    }
    if let Some(classification) = last_startup_classification(manifest) {
        return classification;
    }
    current.to_string()
}

fn verified_daemon_record_observed(
    manifest: &ServiceManifest,
    pid: Option<u32>,
    not_before: DateTime<Utc>,
) -> bool {
    pid.and_then(|pid| daemon_status_matches_pid_spec_and_time(manifest, pid, not_before).ok())
        .unwrap_or(false)
}

fn fresh_daemon_pid_after(manifest: &ServiceManifest, not_before: DateTime<Utc>) -> Option<u32> {
    let raw = fs::read_to_string(&manifest.daemon_status).ok()?;
    let value: Value = serde_json::from_str(&raw).ok()?;
    let loaded = long_lived_agent::load_spec(&manifest.spec).ok()?;
    let agent_id = value.get("agent_instance_id").and_then(Value::as_str)?;
    if agent_id != loaded.spec.agent_instance_id {
        return None;
    }
    let updated_at = value
        .get("updated_at")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|value| value.with_timezone(&Utc))?;
    if updated_at < not_before {
        return None;
    }
    value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok())
}

fn last_startup_classification(manifest: &ServiceManifest) -> Option<String> {
    let raw = fs::read_to_string(&manifest.startup_ledger).ok()?;
    raw.lines()
        .rev()
        .find_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            let result = value
                .get("result")
                .and_then(Value::as_str)
                .filter(|result| *result == "failed" || *result == "completed")?;
            if result == "completed" {
                return Some(None);
            }
            Some(
                value
                    .get("event")
                    .and_then(Value::as_str)
                    .map(str::to_string),
            )
        })
        .flatten()
}

fn last_failed_startup_at(manifest: &ServiceManifest) -> Option<DateTime<Utc>> {
    let raw = fs::read_to_string(&manifest.startup_ledger).ok()?;
    raw.lines()
        .rev()
        .find_map(|line| {
            let value: Value = serde_json::from_str(line).ok()?;
            let result = value
                .get("result")
                .and_then(Value::as_str)
                .filter(|result| *result == "failed" || *result == "completed")?;
            if result == "completed" {
                return Some(None);
            }
            Some(
                value
                    .get("updated_at")
                    .and_then(Value::as_str)
                    .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
                    .map(|value| value.with_timezone(&Utc)),
            )
        })
        .flatten()
}

fn classify_startup(
    pid: Option<u32>,
    pid_liveness: &str,
    first_daemon_record_observed: bool,
    _cycle_ledger_observed: bool,
    _continuity_checkpoint_observed: bool,
) -> &'static str {
    if first_daemon_record_observed && pid_liveness == "live_pid" {
        "startup_first_daemon_record_observed"
    } else if first_daemon_record_observed {
        "startup_daemon_record_without_live_pid"
    } else if matches!(pid_liveness, "stale_pid") {
        "startup_stale_before_first_daemon_record"
    } else if pid.is_none() {
        "startup_missing_pid_metadata"
    } else {
        "startup_waiting_for_first_daemon_record"
    }
}

struct StartupProbeRecord<'a> {
    attempt: u32,
    pid: Option<u32>,
    pid_liveness: &'a str,
    first_daemon_record_observed: bool,
    cycle_ledger_observed: bool,
    continuity_checkpoint_observed: bool,
    classification: &'a str,
}

fn record_startup_probe(manifest: &ServiceManifest, probe: StartupProbeRecord<'_>) -> Result<()> {
    let attempt_s = probe.attempt.to_string();
    let pid_s = probe.pid.map(|pid| pid.to_string()).unwrap_or_default();
    let first_s = probe.first_daemon_record_observed.to_string();
    let cycle_s = probe.cycle_ledger_observed.to_string();
    let checkpoint_s = probe.continuity_checkpoint_observed.to_string();
    append_startup_ledger(
        manifest,
        "startup_probe",
        probe.classification,
        probe.pid,
        Some(json!({
            "attempt": probe.attempt,
            "pid_liveness": probe.pid_liveness,
            "first_daemon_record_observed": probe.first_daemon_record_observed,
            "cycle_ledger_observed": probe.cycle_ledger_observed,
            "continuity_checkpoint_observed": probe.continuity_checkpoint_observed
        })),
    )?;
    emit_service_event(
        manifest,
        "startup_probe",
        probe.classification,
        &[
            ("attempt", &attempt_s),
            ("pid", &pid_s),
            ("pid_liveness", probe.pid_liveness),
            ("first_daemon_record_observed", &first_s),
            ("cycle_ledger_observed", &cycle_s),
            ("continuity_checkpoint_observed", &checkpoint_s),
        ],
    )?;
    Ok(())
}

fn record_startup_event(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    pid: Option<u32>,
    err: Option<&anyhow::Error>,
) -> Result<()> {
    let error = err.map(|err| redact_manifest_path(manifest, &err.to_string()));
    append_startup_ledger(
        manifest,
        stage,
        result,
        pid,
        error.as_ref().map(|message| json!({"error": message})),
    )?;
    let pid_s = pid.map(|pid| pid.to_string()).unwrap_or_default();
    let err_s = error.unwrap_or_default();
    emit_service_event(
        manifest,
        stage,
        result,
        &[("pid", &pid_s), ("error", &err_s)],
    )?;
    Ok(())
}

fn append_startup_ledger(
    manifest: &ServiceManifest,
    event: &str,
    result: &str,
    pid: Option<u32>,
    details: Option<Value>,
) -> Result<()> {
    if let Some(parent) = manifest.startup_ledger.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.startup_ledger)
        .with_context(|| format!("open {}", manifest.startup_ledger.display()))?;
    let record = json!({
        "schema": "adl.csm.service_startup_event.v1",
        "label": manifest.label,
        "manager": manifest.manager.as_str(),
        "runtime_owner": "csm",
        "event": event,
        "result": result,
        "pid": pid,
        "daemon_status_ref": ref_for(&manifest.service_root, &manifest.daemon_status),
        "continuity_checkpoint_ref": ref_for(&manifest.service_root, &manifest.continuity_checkpoint),
        "cycle_ledger_ref": ref_for(&manifest.service_root, &cycle_ledger_path(manifest)),
        "details": details.unwrap_or_else(|| json!({})),
        "updated_at": Utc::now().to_rfc3339()
    });
    writeln!(file, "{}", serde_json::to_string(&record)?)?;
    Ok(())
}

fn emit_service_event(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    extra: &[(&str, &str)],
) -> Result<()> {
    let manager = manifest.manager.as_str();
    let label = manifest.label.as_str();
    let mut fields = vec![
        ("process_class", "csm_service"),
        ("runtime_owner", "csm"),
        ("manager", manager),
        ("label", label),
        ("otel_service_name", "csm-runtime-service"),
    ];
    fields.extend_from_slice(extra);
    observability::emit_event("csm", stage, result, &fields);
    append_service_observability_log(manifest, stage, result, &fields)?;
    append_service_otel_log(manifest, stage, result, &fields)?;
    Ok(())
}

fn append_service_observability_log(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<()> {
    if let Some(parent) = manifest.observability_log.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut line = format!(
        "adl_event schema=adl.observability.event.v1 command=csm stage={stage} result={result}"
    );
    for (key, value) in fields {
        line.push(' ');
        line.push_str(key);
        line.push('=');
        line.push_str(&service_log_token(manifest, value));
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.observability_log)
        .with_context(|| format!("open {}", manifest.observability_log.display()))?;
    writeln!(file, "{line}")?;
    Ok(())
}

fn append_service_otel_log(
    manifest: &ServiceManifest,
    stage: &str,
    result: &str,
    fields: &[(&str, &str)],
) -> Result<()> {
    if let Some(parent) = manifest.otel_log.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut attributes = serde_json::Map::new();
    attributes.insert("adl.command".to_string(), json!("csm"));
    attributes.insert("adl.stage".to_string(), json!(stage));
    attributes.insert("adl.result".to_string(), json!(result));
    for (key, value) in fields {
        attributes.insert(
            format!("adl.{key}"),
            json!(redact_manifest_path(manifest, value)),
        );
    }
    let event = json!({
        "schema": "adl.otel.event.v1",
        "timestamp": Utc::now().to_rfc3339(),
        "name": format!("csm.{stage}"),
        "severity_text": if result == "failed" { "ERROR" } else { "INFO" },
        "resource": {"service.name": "csm-runtime-service"},
        "attributes": attributes
    });
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&manifest.otel_log)
        .with_context(|| format!("open {}", manifest.otel_log.display()))?;
    writeln!(file, "{}", serde_json::to_string(&event)?)?;
    write_json_pretty(
        &manifest.otel_status,
        &json!({
            "schema": "adl.otel.monitor_status.v1",
            "event_count": count_jsonl_lines(&manifest.otel_log),
            "last_event": format!("csm.{stage}"),
            "last_result": result,
            "updated_at": Utc::now().to_rfc3339()
        }),
    )?;
    Ok(())
}

fn count_jsonl_lines(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|raw| raw.lines().filter(|line| !line.trim().is_empty()).count())
        .unwrap_or(0)
}

fn service_log_token(manifest: &ServiceManifest, value: &str) -> String {
    redact_manifest_path(manifest, value)
        .chars()
        .map(|ch| if ch.is_whitespace() { '_' } else { ch })
        .collect()
}

fn redact_manifest_path(manifest: &ServiceManifest, value: &str) -> String {
    value.replace(
        &manifest.service_root.display().to_string(),
        "<service_root>",
    )
}

fn run_launchctl(args: &[&str]) -> Result<()> {
    let status = Command::new("launchctl")
        .args(args)
        .status()
        .context("run launchctl")?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("launchctl {:?} exited with {status}", args))
    }
}

fn manifest_path(service_root: &Path) -> PathBuf {
    service_root.join("service_manifest.json")
}

fn current_exe_or_csm() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("csm"))
}

fn absolutize_create(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(path)?;
    absolutize(path)
}

fn absolutize(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn required_value<'a>(args: &'a [String], index: usize, flag: &str) -> Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| anyhow!("{flag} requires a value"))
}

fn parse_u64(raw: &str) -> Result<u64> {
    raw.parse::<u64>()
        .map_err(|_| anyhow!("expected unsigned integer, got '{raw}'"))
}

fn parse_positive_u64(raw: &str, flag: &str) -> Result<u64> {
    let value = parse_u64(raw)?;
    if value == 0 {
        return Err(anyhow!("{flag} must be greater than zero"));
    }
    Ok(value)
}

fn validate_label(label: &str) -> Result<()> {
    if label.trim().is_empty() {
        return Err(anyhow!("--label cannot be empty"));
    }
    if label.contains('/') || label.contains(char::is_whitespace) {
        return Err(anyhow!(
            "--label must not contain path separators or whitespace"
        ));
    }
    Ok(())
}

fn validate_otlp_endpoint(endpoint: &str) -> Result<()> {
    if endpoint.is_empty() {
        return Err(anyhow!("--otlp-endpoint requires a non-empty endpoint"));
    }
    if !(endpoint.starts_with("http://") || endpoint.starts_with("https://")) {
        return Err(anyhow!(
            "--otlp-endpoint must start with http:// or https://"
        ));
    }
    let lower = endpoint.to_ascii_lowercase();
    if lower.contains("token")
        || lower.contains("secret")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || endpoint.contains('@')
    {
        return Err(anyhow!(
            "--otlp-endpoint must not contain credentials, userinfo, or secret markers"
        ));
    }
    Ok(())
}

fn env_otlp_endpoint() -> Result<Option<String>> {
    let Some(endpoint) = env::var("ADL_OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| env::var("OTEL_EXPORTER_OTLP_ENDPOINT"))
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };
    validate_otlp_endpoint(&endpoint)?;
    Ok(Some(endpoint))
}

fn xml_escape(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn ref_for(root: &Path, path: &Path) -> String {
    if let Ok(rel) = path.strip_prefix(root) {
        return rel.display().to_string();
    }
    if let Some(parent) = root.parent() {
        if let Ok(rel) = path.strip_prefix(parent) {
            return format!("../{}", rel.display());
        }
    }
    path.display().to_string()
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("path is not valid UTF-8: {}", path.display()))
}

fn read_pid_file(path: &Path) -> Result<u32> {
    let raw = fs::read_to_string(path)?;
    raw.trim()
        .parse::<u32>()
        .with_context(|| format!("parse pid from {}", path.display()))
}

fn read_daemon_pid(path: &Path) -> Result<Option<u32>> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&raw)?;
    Ok(value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok()))
}

fn daemon_status_matches_pid_and_spec(manifest: &ServiceManifest, pid: u32) -> Result<bool> {
    if !manifest.daemon_status.exists() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest.daemon_status)?;
    let value: Value = serde_json::from_str(&raw)?;
    let loaded = long_lived_agent::load_spec(&manifest.spec)?;
    let daemon_pid = value
        .get("supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok());
    let agent_id = value.get("agent_instance_id").and_then(Value::as_str);
    Ok(daemon_pid == Some(pid) && agent_id == Some(loaded.spec.agent_instance_id.as_str()))
}

fn daemon_status_matches_pid_spec_and_time(
    manifest: &ServiceManifest,
    pid: u32,
    not_before: DateTime<Utc>,
) -> Result<bool> {
    if !manifest.daemon_status.exists() {
        return Ok(false);
    }
    let raw = fs::read_to_string(&manifest.daemon_status)?;
    let value: Value = serde_json::from_str(&raw)?;
    let updated_at = value
        .get("updated_at")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|value| value.with_timezone(&Utc));
    Ok(daemon_status_matches_pid_and_spec(manifest, pid)?
        && updated_at
            .map(|updated_at| updated_at >= not_before)
            .unwrap_or(false))
}

fn pid_liveness(pid: u32) -> String {
    match pid_is_live(pid) {
        Some(true) => "live_pid".to_string(),
        Some(false) => "stale_pid".to_string(),
        None => "unknown".to_string(),
    }
}

#[cfg(unix)]
fn pid_is_live(pid: u32) -> Option<bool> {
    const EPERM: i32 = 1;
    const ESRCH: i32 = 3;
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    if pid > i32::MAX as u32 {
        return Some(false);
    }
    let result = unsafe { kill(pid as i32, 0) };
    if result == 0 {
        return Some(true);
    }
    match std::io::Error::last_os_error().raw_os_error() {
        Some(EPERM) => Some(true),
        Some(ESRCH) => Some(false),
        _ => None,
    }
}

#[cfg(not(unix))]
fn pid_is_live(_pid: u32) -> Option<bool> {
    None
}

#[cfg(unix)]
fn current_uid() -> u32 {
    unsafe extern "C" {
        fn getuid() -> u32;
    }
    unsafe { getuid() }
}

#[cfg(not(unix))]
fn current_uid() -> u32 {
    0
}

fn unsupported_permanence_claims() -> Vec<String> {
    vec![
        "host_reboot_survival_not_proven".to_string(),
        "kill_9_recovery_not_proven".to_string(),
        "disk_full_recovery_not_proven".to_string(),
        "resource_exhaustion_recovery_not_proven".to_string(),
        "cloud_orchestration_not_claimed".to_string(),
    ]
}

fn normalize_service_manifest_metadata(mut manifest: ServiceManifest) -> ServiceManifest {
    manifest.restart_policy = service_restart_policy(manifest.manager, manifest.no_sleep);
    manifest.service_mode = service_mode(manifest.manager, manifest.no_sleep);
    manifest
}

fn service_restart_policy(manager: ServiceManager, no_sleep: bool) -> String {
    if no_sleep {
        "bounded_test_only".to_string()
    } else if manager == ServiceManager::Launchd {
        "always".to_string()
    } else {
        "external_supervisor_required".to_string()
    }
}

fn service_mode(manager: ServiceManager, no_sleep: bool) -> String {
    if no_sleep {
        "bounded_test_only".to_string()
    } else if manager == ServiceManager::Launchd {
        "permanent".to_string()
    } else {
        "local_proof_only".to_string()
    }
}

pub(crate) fn service_usage() -> &'static str {
    "Usage:
  csm service install --spec <agent-spec.yaml> [--service-root <dir>] [--manager launchd|local] [--label <label>] [--csm-bin <path>] [--checkpoint-interval-secs <n>] [--interval-secs <n>] [--otlp-endpoint <url>] [--otlp-timeout-ms <n>] [--no-recover-stale-lease] [--no-sleep] [--json]
  csm service start [--service-root <dir>] [--json]
  csm service status [--service-root <dir>] [--json]
  csm service stop [--service-root <dir>] [--json]
  csm service remove [--service-root <dir>] [--json]

Semantics:
  - csm service is the host-service envelope for the standalone csm runtime owner.
  - launchd service mode records restart_policy=always and service_mode=permanent; launchd KeepAlive is the primary macOS service-manager target and systemd Restart=always compatible metadata is retained.
  - local mode is a bounded proof fallback and records external_supervisor_required/local_proof_only.
  - the managed command is always csm daemon, never adl agent daemon.
  - --no-sleep is an explicit test-only bounded harness boundary, not production service mode.
  - service artifacts include service_manifest.json, service_status.json, csm.launchd.plist, logs, OTel status/export paths, daemon_status.json, continuity checkpoints, and operator events.
  - status uses metadata or exact PID liveness probes only; no broad process scan or ps output is used.
  - unsupported permanence claims remain explicit for reboot, kill -9, disk-full, resource exhaustion, and cloud orchestration."
}
