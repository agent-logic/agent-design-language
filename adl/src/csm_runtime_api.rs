//! Local CSM runtime observability API.
use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::long_lived_agent::{load_spec, AgentStatusState, LoadedAgentSpec, StatusRecord};

pub const CSM_RUNTIME_API_SCHEMA: &str = "adl.csm.runtime_api.v1";
pub const CSM_RUNTIME_API_STATUS_SCHEMA: &str = "adl.csm.runtime_api.status.v1";
pub const CSM_RUNTIME_API_HEALTH_SCHEMA: &str = "adl.csm.runtime_api.health.v1";
pub const CSM_RUNTIME_API_READY_SCHEMA: &str = "adl.csm.runtime_api.ready.v1";
pub const CSM_RUNTIME_API_METRICS_SCHEMA: &str = "adl.csm.runtime_api.metrics.v1";
pub const CSM_RUNTIME_API_EVENTS_SCHEMA: &str = "adl.csm.runtime_api.events.v1";

#[derive(Debug, Clone)]
pub struct CsmRuntimeApiOptions {
    pub spec_path: PathBuf,
    pub bind: String,
    pub test_max_requests: Option<usize>,
    pub idle_timeout_ms: Option<u64>,
    pub otel_status_path: Option<PathBuf>,
    pub otel_log_path: Option<PathBuf>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CsmRuntimeApiServeResult {
    pub schema: String,
    pub status: String,
    pub bind_addr: String,
    pub served_requests: usize,
}

pub fn serve_runtime_api(options: CsmRuntimeApiOptions) -> Result<CsmRuntimeApiServeResult> {
    if options.test_max_requests == Some(0) {
        bail!("CSM runtime API test request limit must be greater than zero");
    }
    let listener = TcpListener::bind(&options.bind)
        .with_context(|| format!("failed binding CSM runtime API to {}", options.bind))?;
    let addr = listener
        .local_addr()
        .context("read CSM API local address")?;
    validate_loopback_bind(&addr)?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "listening",
            "bind_addr": addr.to_string(),
            "runtime_owner": "csm"
        }))?
    );
    std::io::stdout().flush().ok();

    let mut served = 0usize;
    if let Some(idle_timeout_ms) = options.idle_timeout_ms {
        listener
            .set_nonblocking(true)
            .context("set CSM runtime API listener nonblocking")?;
        let idle_timeout = Duration::from_millis(idle_timeout_ms);
        let mut last_activity = Instant::now();
        while options
            .test_max_requests
            .is_none_or(|test_max_requests| served < test_max_requests)
        {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(5)))
                        .context("set CSM runtime API read timeout")?;
                    let request = read_request_path(&mut stream)?;
                    let response = runtime_api_response(&options, &request)?;
                    write_http_json(&mut stream, &response)?;
                    served += 1;
                    last_activity = Instant::now();
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    if last_activity.elapsed() >= idle_timeout {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(err) => return Err(err).context("accept CSM runtime API connection"),
            }
        }
    } else {
        for stream in listener.incoming() {
            let mut stream = stream.context("accept CSM runtime API connection")?;
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .context("set CSM runtime API read timeout")?;
            let request = read_request_path(&mut stream)?;
            let response = runtime_api_response(&options, &request)?;
            write_http_json(&mut stream, &response)?;
            served += 1;
            if options
                .test_max_requests
                .is_some_and(|test_max_requests| served >= test_max_requests)
            {
                break;
            }
        }
    }
    Ok(CsmRuntimeApiServeResult {
        schema: CSM_RUNTIME_API_SCHEMA.to_string(),
        status: "completed".to_string(),
        bind_addr: addr.to_string(),
        served_requests: served,
    })
}

pub fn runtime_api_response(options: &CsmRuntimeApiOptions, path: &str) -> Result<Value> {
    let loaded = load_spec(&options.spec_path)?;
    let endpoint = path.split('?').next().unwrap_or(path);
    match endpoint {
        "/" | "/status" => status_response(&loaded, options),
        "/health" => health_response(&loaded, options),
        "/ready" => ready_response(&loaded, options),
        "/metrics" => metrics_response(&loaded, options),
        "/events" => events_response(&loaded),
        other => Ok(json!({
            "schema": CSM_RUNTIME_API_SCHEMA,
            "status": "not_found",
            "endpoint": other,
            "supported_endpoints": ["/status", "/health", "/ready", "/metrics", "/events"]
        })),
    }
}

fn status_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let agent_status = read_agent_status_snapshot(loaded)?;
    let daemon_status = read_json_artifact(&artifact_path(loaded, "daemon_status.json"));
    let continuity_checkpoint =
        read_json_artifact(&artifact_path(loaded, "continuity_checkpoint.json"));
    let replay_manifest =
        read_json_artifact(&artifact_path(loaded, "continuity_replay_manifest.json"));
    let safe_fail_bundle = read_json_artifact(&artifact_path(loaded, "safe_fail_bundle.json"));
    let backpressure_state =
        read_json_artifact(&artifact_path(loaded, "csm_backpressure_state.json"));
    let otel_status_path = resolve_otel_status_path(loaded, options);
    let otel_log_path = resolve_otel_log_path(loaded, options);
    let otel_status = otel_status_path
        .as_deref()
        .map(read_json_artifact)
        .unwrap_or_else(|| json!({"status": "missing", "ref": "ADL_OTEL_STATUS"}));
    let otel_log = otel_log_path
        .as_deref()
        .map(|path| file_ref_status(path, "ADL_OTEL_LOG"))
        .unwrap_or_else(|| json!({"status": "missing", "ref": "ADL_OTEL_LOG"}));
    let checkpoint_freshness = checkpoint_freshness(&daemon_status, &continuity_checkpoint);
    let daemon_state = daemon_lifecycle_state(&daemon_status);
    let daemon_pid_liveness = daemon_supervisor_pid_liveness(&daemon_status);
    let health = classify_health(
        &agent_status.state,
        &daemon_status,
        &checkpoint_freshness,
        daemon_state,
        daemon_pid_liveness.as_deref(),
    );
    let ready = classify_ready(
        &agent_status.state,
        &daemon_status,
        &continuity_checkpoint,
        daemon_state,
        daemon_pid_liveness.as_deref(),
    );
    let runtime_capabilities = daemon_status
        .get("value")
        .and_then(|value| value.get("runtime_capabilities"))
        .cloned()
        .unwrap_or_else(|| json!({"status": "missing"}));
    let response = json!({
        "schema": CSM_RUNTIME_API_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "adl_role": "tooling_control_plane",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": health,
        "ready": ready,
        "daemon_liveness": artifact_liveness(&daemon_status),
        "agent_status": {
            "state": agent_status.state,
            "last_cycle_id": agent_status.last_cycle_id,
            "last_cycle_status": agent_status.last_cycle_status,
            "completed_cycle_count": agent_status.completed_cycle_count,
            "consecutive_failure_count": agent_status.consecutive_failure_count,
            "stop_requested": agent_status.stop_requested,
            "updated_at": agent_status.updated_at
        },
        "scheduler": runtime_capabilities.get("scheduler_watcher").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "chronosense": runtime_capabilities.get("chronosense").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "aee_resilience": runtime_capabilities.get("aee").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "resilience_middleware": runtime_capabilities.get("resilience_middleware").cloned().unwrap_or_else(|| json!({"status": "missing"})),
        "checkpoint": checkpoint_freshness,
        "continuity": {
            "checkpoint": compact_artifact_status(&continuity_checkpoint, "continuity_checkpoint.json"),
            "replay_manifest": compact_artifact_status(&replay_manifest, "continuity_replay_manifest.json"),
            "safe_fail_bundle": compact_artifact_status(&safe_fail_bundle, "safe_fail_bundle.json")
        },
        "backpressure": compact_artifact_status(&backpressure_state, "csm_backpressure_state.json"),
        "otel": {
            "status": compact_artifact_status(&otel_status, "ADL_OTEL_STATUS"),
            "log": otel_log
        },
        "events": file_ref_status(&artifact_path(loaded, "operator_events.jsonl"), "operator_events.jsonl"),
        "redaction": {
            "absolute_host_paths": "not_returned",
            "secret_material": "not_returned",
            "cloud_account_identifiers": "not_returned"
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn health_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_HEALTH_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "status": status["status"],
        "daemon_liveness": status["daemon_liveness"],
        "checkpoint": status["checkpoint"],
        "otel": status["otel"]
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn ready_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let response = json!({
        "schema": CSM_RUNTIME_API_READY_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "ready": status["ready"],
        "blocking_reasons": readiness_blockers(&status)
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn metrics_response(loaded: &LoadedAgentSpec, options: &CsmRuntimeApiOptions) -> Result<Value> {
    let status = status_response(loaded, options)?;
    let daemon = read_json_artifact(&artifact_path(loaded, "daemon_status.json"));
    let backpressure_state =
        read_json_artifact(&artifact_path(loaded, "csm_backpressure_state.json"));
    let event_count = read_jsonl_tail(&artifact_path(loaded, "operator_events.jsonl"), usize::MAX)
        .get("entries")
        .and_then(Value::as_array)
        .map(|entries| entries.len())
        .unwrap_or(0);
    let response = json!({
        "schema": CSM_RUNTIME_API_METRICS_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "gauges": {
            "completed_cycle_count": status["agent_status"]["completed_cycle_count"],
            "consecutive_failure_count": status["agent_status"]["consecutive_failure_count"],
            "restart_count": daemon.pointer("/value/restart_count").cloned().unwrap_or(Value::Null),
            "checkpoint_interval_secs": daemon.pointer("/value/checkpoint_interval_secs").cloned().unwrap_or(Value::Null),
            "operator_event_count_observed": event_count,
            "backpressure_queue_depth": backpressure_state.pointer("/value/summary/max_queue_depth").cloned().unwrap_or(Value::Null),
            "backpressure_lag_ms": backpressure_state.pointer("/value/summary/max_lag_ms").cloned().unwrap_or(Value::Null),
            "backpressure_deferred_count": backpressure_state.pointer("/value/summary/deferred_count").cloned().unwrap_or(Value::Null),
            "backpressure_shed_count": backpressure_state.pointer("/value/summary/shed_count").cloned().unwrap_or(Value::Null),
            "backpressure_retry_capacity_remaining": backpressure_state.pointer("/value/summary/retry_budget_remaining").cloned().unwrap_or(Value::Null)
        },
        "states": {
            "health": status["status"],
            "ready": status["ready"],
            "agent_state": status["agent_status"]["state"],
            "backpressure_health": backpressure_state.pointer("/value/summary/health").cloned().unwrap_or(Value::Null),
            "backpressure_safe_fail_action": backpressure_state.pointer("/value/safe_fail_action/action").cloned().unwrap_or(Value::Null)
        }
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn events_response(loaded: &LoadedAgentSpec) -> Result<Value> {
    let events = read_jsonl_tail(&artifact_path(loaded, "operator_events.jsonl"), 40);
    let response = json!({
        "schema": CSM_RUNTIME_API_EVENTS_SCHEMA,
        "runtime_owner": "csm",
        "agent_instance_id": loaded.spec.agent_instance_id,
        "events": events
    });
    assert_api_response_redacted(&response)?;
    Ok(response)
}

fn artifact_path(loaded: &LoadedAgentSpec, name: &str) -> PathBuf {
    loaded.state_root.join(name)
}

fn resolve_otel_status_path(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Option<PathBuf> {
    options
        .otel_status_path
        .clone()
        .or_else(|| env_path("ADL_OTEL_STATUS"))
        .or_else(|| env_path("OTEL_STATUS"))
        .or_else(|| sibling_service_log_path(loaded, "otel_status.json"))
        .filter(|path| path.exists())
}

fn resolve_otel_log_path(
    loaded: &LoadedAgentSpec,
    options: &CsmRuntimeApiOptions,
) -> Option<PathBuf> {
    options
        .otel_log_path
        .clone()
        .or_else(|| env_path("ADL_OTEL_LOG"))
        .or_else(|| sibling_service_log_path(loaded, "otel.jsonl"))
        .filter(|path| path.exists())
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn sibling_service_log_path(loaded: &LoadedAgentSpec, file_name: &str) -> Option<PathBuf> {
    loaded
        .state_root
        .parent()
        .map(|runtime_root| runtime_root.join("service").join("logs").join(file_name))
}

fn read_agent_status_snapshot(loaded: &LoadedAgentSpec) -> Result<StatusRecord> {
    let path = artifact_path(loaded, "status.json");
    if !path.exists() {
        return Ok(StatusRecord {
            schema: "adl.long_lived_agent_status.v1".to_string(),
            agent_instance_id: loaded.spec.agent_instance_id.clone(),
            state: AgentStatusState::NotStarted,
            last_cycle_id: None,
            last_cycle_status: None,
            completed_cycle_count: 0,
            consecutive_failure_count: 0,
            active_lease: None,
            stop_requested: false,
            last_error: None,
            safety_policy: Value::Null,
            updated_at: Utc::now(),
        });
    }
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("read CSM runtime API status snapshot {}", path.display()))?;
    serde_json::from_str::<StatusRecord>(&raw)
        .with_context(|| format!("parse CSM runtime API status snapshot {}", path.display()))
}

fn read_json_artifact(path: &Path) -> Value {
    if !path.exists() {
        return json!({"status": "missing"});
    }
    match fs::read_to_string(path) {
        Ok(raw) => match serde_json::from_str::<Value>(&raw) {
            Ok(value) => json!({"status": "serialized", "value": sanitize_json(value)}),
            Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
        },
        Err(err) => json!({"status": "unreadable", "reason": err.to_string()}),
    }
}

fn read_jsonl_tail(path: &Path, limit: usize) -> Value {
    if !path.exists() {
        return json!({"status": "missing", "entries": []});
    }
    let Ok(raw) = fs::read_to_string(path) else {
        return json!({"status": "unreadable", "entries": []});
    };
    let lines = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();
    let start = lines.len().saturating_sub(limit);
    let mut entries = Vec::new();
    let mut unreadable = 0usize;
    for line in &lines[start..] {
        match serde_json::from_str::<Value>(line) {
            Ok(value) => entries.push(sanitize_json(value)),
            Err(_) => unreadable += 1,
        }
    }
    json!({
        "status": if unreadable == 0 { "serialized" } else { "partial" },
        "tail_limit": limit,
        "unreadable_lines": unreadable,
        "entries": entries
    })
}

fn sanitize_json(value: Value) -> Value {
    match value {
        Value::String(raw) => sanitize_string(&raw).into(),
        Value::Array(values) => Value::Array(values.into_iter().map(sanitize_json).collect()),
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| {
                    let lowered = key.to_ascii_lowercase();
                    if lowered.contains("secret")
                        || lowered.contains("token")
                        || lowered.contains("authorization")
                        || lowered.contains("credential")
                    {
                        (key, Value::String("[redacted]".to_string()))
                    } else {
                        (key, sanitize_json(value))
                    }
                })
                .collect(),
        ),
        other => other,
    }
}

fn sanitize_string(raw: &str) -> String {
    if looks_like_host_private_path(raw)
        || raw.contains("Authorization:")
        || raw.to_ascii_lowercase().contains("bearer ")
        || raw.to_ascii_lowercase().contains("aws_secret_access_key")
        || raw.contains("arn:aws:")
        || contains_cloud_account_identifier(raw)
    {
        "[redacted]".to_string()
    } else {
        raw.to_string()
    }
}

fn contains_cloud_account_identifier(raw: &str) -> bool {
    let mut digits = 0usize;
    for byte in raw.bytes() {
        if byte.is_ascii_digit() {
            digits += 1;
        } else {
            if digits == 12 {
                return true;
            }
            digits = 0;
        }
    }
    digits == 12
}

fn looks_like_host_private_path(raw: &str) -> bool {
    raw.contains("/Users/")
        || raw.contains("/home/")
        || raw.contains("/private/")
        || raw.contains("/var/folders/")
}

fn file_ref_status(path: &Path, reference: &str) -> Value {
    if path.exists() {
        json!({"status": "retained", "ref": reference, "bytes": fs::metadata(path).map(|m| m.len()).ok()})
    } else {
        json!({"status": "missing", "ref": reference})
    }
}

fn compact_artifact_status(artifact: &Value, reference: &str) -> Value {
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("unknown")),
        "ref": reference,
        "schema": artifact.pointer("/value/schema").cloned().unwrap_or(Value::Null)
    })
}

fn artifact_liveness(daemon_status: &Value) -> Value {
    let status = daemon_status
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    let pid_liveness = daemon_supervisor_pid_liveness(daemon_status);
    json!({
        "status": if status == "serialized" { "observed" } else { status },
        "state": daemon_status.pointer("/value/state").cloned().unwrap_or(Value::Null),
        "supervisor_pid_liveness": pid_liveness.unwrap_or_else(|| "missing_pid_metadata".to_string()),
        "last_event": daemon_status.pointer("/value/last_event").cloned().unwrap_or(Value::Null),
        "updated_at": daemon_status.pointer("/value/updated_at").cloned().unwrap_or(Value::Null)
    })
}

fn checkpoint_freshness(daemon_status: &Value, continuity_checkpoint: &Value) -> Value {
    let daemon_checkpoint = daemon_status.pointer("/value/last_checkpoint_at");
    let checkpoint_status = continuity_checkpoint.get("status").and_then(Value::as_str);
    let age_secs = daemon_checkpoint
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|time| {
            Utc::now()
                .signed_duration_since(time.with_timezone(&Utc))
                .num_seconds()
                .max(0)
        });
    let freshness = match (checkpoint_status, age_secs) {
        (Some("serialized"), Some(age)) if age <= 300 => "fresh",
        (Some("serialized"), Some(_)) => "stale",
        (Some("serialized"), None) => "unknown",
        (Some(other), _) => other,
        (None, _) => "missing",
    };
    json!({
        "status": freshness,
        "last_checkpoint_at": daemon_checkpoint.cloned().unwrap_or(Value::Null),
        "age_secs": age_secs,
        "checkpoint_ref": "continuity_checkpoint.json"
    })
}

fn classify_health(
    state: &AgentStatusState,
    daemon_status: &Value,
    checkpoint_freshness: &Value,
    daemon_state: Option<&str>,
    daemon_pid_liveness: Option<&str>,
) -> &'static str {
    let degraded = matches!(state, AgentStatusState::Failed)
        || daemon_status.get("status").and_then(Value::as_str) != Some("serialized")
        || checkpoint_freshness.get("status").and_then(Value::as_str) == Some("stale")
        || is_terminal_daemon_state(daemon_state)
        || daemon_pid_liveness == Some("stale_pid");
    if degraded {
        "degraded"
    } else {
        "healthy"
    }
}

fn classify_ready(
    state: &AgentStatusState,
    daemon_status: &Value,
    continuity_checkpoint: &Value,
    daemon_state: Option<&str>,
    daemon_pid_liveness: Option<&str>,
) -> &'static str {
    let not_ready = matches!(
        state,
        AgentStatusState::Failed | AgentStatusState::Leased | AgentStatusState::Stopped
    ) || daemon_status.get("status").and_then(Value::as_str) != Some("serialized")
        || continuity_checkpoint.get("status").and_then(Value::as_str) != Some("serialized")
        || is_terminal_daemon_state(daemon_state)
        || daemon_pid_liveness == Some("stale_pid");
    if not_ready {
        "not_ready"
    } else {
        "ready"
    }
}

fn daemon_lifecycle_state(daemon_status: &Value) -> Option<&str> {
    daemon_status
        .pointer("/value/state")
        .and_then(Value::as_str)
}

fn is_terminal_daemon_state(state: Option<&str>) -> bool {
    matches!(
        state,
        Some("governed_stopped" | "stopped" | "stop_requested" | "startup_failed")
    )
}

fn daemon_supervisor_pid_liveness(daemon_status: &Value) -> Option<String> {
    let pid = daemon_status
        .pointer("/value/supervisor_pid")
        .and_then(Value::as_u64)
        .and_then(|pid| u32::try_from(pid).ok())?;
    Some(match exact_pid_is_live(pid) {
        Some(true) => "live_pid".to_string(),
        Some(false) => "stale_pid".to_string(),
        None => "unknown".to_string(),
    })
}

#[cfg(unix)]
fn exact_pid_is_live(pid: u32) -> Option<bool> {
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
fn exact_pid_is_live(_pid: u32) -> Option<bool> {
    None
}

fn readiness_blockers(status: &Value) -> Vec<String> {
    let mut blockers = Vec::new();
    if status
        .pointer("/daemon_liveness/status")
        .and_then(Value::as_str)
        != Some("observed")
    {
        blockers.push("daemon_status_missing".to_string());
    }
    if status
        .pointer("/continuity/checkpoint/status")
        .and_then(Value::as_str)
        != Some("serialized")
    {
        blockers.push("continuity_checkpoint_missing".to_string());
    }
    if status
        .pointer("/daemon_liveness/supervisor_pid_liveness")
        .and_then(Value::as_str)
        == Some("stale_pid")
    {
        blockers.push("daemon_supervisor_pid_stale".to_string());
    }
    if status
        .pointer("/agent_status/state")
        .and_then(Value::as_str)
        == Some("failed")
    {
        blockers.push("agent_state_failed".to_string());
    }
    if let Some("leased") = status
        .pointer("/agent_status/state")
        .and_then(Value::as_str)
    {
        blockers.push("agent_state_leased".to_string());
    }
    match status
        .pointer("/daemon_liveness/state")
        .and_then(Value::as_str)
    {
        Some("governed_stopped") => blockers.push("daemon_state_governed_stopped".to_string()),
        Some("stopped") => blockers.push("daemon_state_stopped".to_string()),
        Some("stop_requested") => blockers.push("daemon_state_stop_requested".to_string()),
        Some("startup_failed") => blockers.push("daemon_state_startup_failed".to_string()),
        _ => {}
    }
    blockers
}

fn validate_loopback_bind(addr: &SocketAddr) -> Result<()> {
    if !addr.ip().is_loopback() {
        bail!("CSM runtime API requires a loopback bind address unless remote auth is implemented");
    }
    Ok(())
}

fn read_request_path(stream: &mut TcpStream) -> Result<String> {
    let mut buf = Vec::new();
    let mut chunk = [0_u8; 256];
    loop {
        let n = stream
            .read(&mut chunk)
            .context("read CSM runtime API request")?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.windows(2).any(|window| window == b"\r\n")
            || buf.windows(1).any(|window| window == b"\n")
            || buf.len() >= 4096
        {
            break;
        }
    }
    let request = String::from_utf8_lossy(&buf);
    let first_line = request.lines().next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let path = parts.next().unwrap_or("/__bad_request");
    if method != "GET" {
        return Ok("/__method_not_allowed".to_string());
    }
    Ok(path.to_string())
}

fn write_http_json(stream: &mut TcpStream, body: &Value) -> Result<()> {
    let status = if body.get("status").and_then(Value::as_str) == Some("not_found") {
        "404 Not Found"
    } else {
        "200 OK"
    };
    let raw = serde_json::to_vec_pretty(body)?;
    write!(
        stream,
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
        raw.len()
    )?;
    stream.write_all(&raw)?;
    Ok(())
}

fn assert_api_response_redacted(value: &Value) -> Result<()> {
    let raw = serde_json::to_string(value)?;
    for forbidden in [
        "/Users/",
        "/home/",
        "/private/",
        "/var/folders/",
        "Authorization:",
        "Bearer ",
        "arn:aws:",
    ] {
        if raw.contains(forbidden) {
            return Err(anyhow!(
                "CSM runtime API response leaked forbidden token: {forbidden}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_root(name: &str) -> PathBuf {
        let id = SEQ.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!("adl-csm-runtime-api-{name}-{id}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn write_spec(root: &Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: api-agent
display_name: API Agent
state_root: state
workflow:
  kind: demo_adapter
heartbeat:
  interval_secs: 1
safety: {}
memory: {}
"#,
        )
        .unwrap();
        spec
    }

    fn test_api_bind(offset: u64) -> String {
        let port = 19950 + (offset % 47);
        format!("127.0.0.1:{port}")
    }

    #[test]
    fn runtime_api_reports_not_ready_when_runtime_artifacts_are_missing() {
        let root = temp_root("missing");
        let spec = write_spec(&root);
        let state = root.join("state");
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(response["schema"], CSM_RUNTIME_API_READY_SCHEMA);
        assert_eq!(response["ready"], "not_ready");
        assert!(response["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_status_missing")));
        assert!(
            !state.exists(),
            "read-only runtime API status must not initialize state"
        );
    }

    #[test]
    fn runtime_api_redacts_secret_and_host_path_event_payloads() {
        let root = temp_root("redaction");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("operator_events.jsonl"),
            r#"{"schema":"adl.long_lived_agent_operator_event.v1","event":"probe","details":{"token":"abc","message":"failed opening /Users/example/secret from account 123456789012","arn":"arn:aws:iam::123456789012:role/example"}}"#,
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/events").unwrap();
        let raw = serde_json::to_string(&response).unwrap();
        assert!(!raw.contains("abc"));
        assert!(!raw.contains("/Users/"));
        assert!(!raw.contains("123456789012"));
        assert!(!raw.contains("arn:aws:"));
        assert!(raw.contains("[redacted]"));
    }

    #[test]
    fn runtime_api_reports_ready_during_active_agent_cycle() {
        let root = temp_root("active-state");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "running_cycle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": null,
                "completed_cycle_count": 0,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.daemon_status.v1",
                "state": "running",
                "last_event": "daemon_started",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.csm.continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(response["ready"], "ready");
        assert!(response["blocking_reasons"].as_array().unwrap().is_empty());
    }

    #[test]
    fn runtime_api_discovers_sibling_service_otel_artifacts() {
        let root = temp_root("service-otel");
        let spec = write_spec(&root);
        let state = root.join("state");
        let service_logs = root.join("service/logs");
        fs::create_dir_all(&state).unwrap();
        fs::create_dir_all(&service_logs).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "idle",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": false,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "running",
                "last_event": "child_exit",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            service_logs.join("otel_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.otel.monitor_status.v1",
                "event_count": 3,
                "last_event": "csm.startup_runtime_ready"
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            service_logs.join("otel.jsonl"),
            "{\"schema\":\"adl.otel.event.v1\"}\n",
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let response = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(response["status"], "healthy");
        assert_eq!(response["otel"]["status"]["status"], "serialized");
        assert_eq!(
            response["otel"]["status"]["schema"],
            "adl.otel.monitor_status.v1"
        );
        assert_eq!(response["otel"]["log"]["status"], "retained");
    }

    #[test]
    fn runtime_api_marks_governed_stopped_runtime_not_ready() {
        let root = temp_root("governed-stopped");
        let spec = write_spec(&root);
        let state = root.join("state");
        fs::create_dir_all(&state).unwrap();
        fs::write(
            state.join("status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_status.v1",
                "agent_instance_id": "api-agent",
                "state": "stopped",
                "last_cycle_id": "cycle-000001",
                "last_cycle_status": "success",
                "completed_cycle_count": 1,
                "consecutive_failure_count": 0,
                "active_lease": null,
                "stop_requested": true,
                "last_error": null,
                "safety_policy": null,
                "updated_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("daemon_status.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_daemon_status.v1",
                "state": "governed_stopped",
                "supervisor_pid": u32::MAX,
                "last_event": "governed_stop",
                "updated_at": Utc::now(),
                "last_checkpoint_at": Utc::now()
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            state.join("continuity_checkpoint.json"),
            serde_json::to_string_pretty(&json!({
                "schema": "adl.long_lived_agent_continuity_checkpoint.v1"
            }))
            .unwrap(),
        )
        .unwrap();
        let options = CsmRuntimeApiOptions {
            spec_path: spec,
            bind: test_api_bind(SEQ.load(Ordering::SeqCst)),
            test_max_requests: Some(1),
            idle_timeout_ms: None,
            otel_status_path: None,
            otel_log_path: None,
        };
        let status = runtime_api_response(&options, "/status").unwrap();
        assert_eq!(status["status"], "degraded");
        assert_eq!(status["ready"], "not_ready");
        let ready = runtime_api_response(&options, "/ready").unwrap();
        assert_eq!(ready["ready"], "not_ready");
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_state_governed_stopped")));
        assert!(ready["blocking_reasons"]
            .as_array()
            .unwrap()
            .contains(&json!("daemon_supervisor_pid_stale")));
    }
}
