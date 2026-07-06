use super::*;
use std::io::Write;

fn spawn_loopback_otlp_collector() -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::thread::JoinHandle<()>,
) {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback otlp collector");
    listener
        .set_nonblocking(true)
        .expect("set collector nonblocking");
    let addr = listener.local_addr().expect("collector addr");
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let started = std::time::Instant::now();
        let mut last_request_at: Option<std::time::Instant> = None;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_nonblocking(false)
                        .expect("set collector stream blocking");
                    stream
                        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                        .expect("set collector stream read timeout");
                    let body = read_http_body(&mut stream);
                    tx.send(body).expect("send otlp body");
                    stream
                        .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\n\r\nOK")
                        .expect("write collector response");
                    last_request_at = Some(std::time::Instant::now());
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    let idle_done = last_request_at
                        .map(|instant| instant.elapsed() > std::time::Duration::from_secs(3))
                        .unwrap_or(false);
                    if idle_done || started.elapsed() > std::time::Duration::from_secs(12) {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                Err(err) => panic!("collector accept failed: {err}"),
            }
        }
    });
    (format!("http://{addr}/v1/traces"), rx, handle)
}

fn read_http_body(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    let mut temp = [0_u8; 4096];
    loop {
        let n = stream.read(&mut temp).expect("read request");
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(header_end) = find_header_end(&buf) {
            let content_length = content_length(&buf[..header_end]).unwrap_or(0);
            if buf.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    if let Some(header_end) = find_header_end(&buf) {
        String::from_utf8_lossy(&buf[header_end + 4..]).to_string()
    } else {
        String::new()
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn content_length(headers: &[u8]) -> Option<usize> {
    String::from_utf8_lossy(headers).lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then(|| value.trim().parse::<usize>().ok())
            .flatten()
    })
}

fn read_http_response_body(stream: &mut std::net::TcpStream) -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    let mut temp = [0_u8; 4096];
    loop {
        let n = match stream.read(&mut temp) {
            Ok(n) => n,
            Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => break,
            Err(err) => panic!("read response: {err}"),
        };
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(header_end) = find_header_end(&buf) {
            let content_length = content_length(&buf[..header_end]).unwrap_or(0);
            if buf.len() >= header_end + 4 + content_length {
                break;
            }
        }
    }
    if let Some(header_end) = find_header_end(&buf) {
        String::from_utf8_lossy(&buf[header_end + 4..]).to_string()
    } else {
        String::new()
    }
}

fn read_text_or_missing(path: &std::path::Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| format!("<missing {}: {err}>", path.display()))
}

fn assert_text_contains(haystack: &str, needle: &str, label: &str) {
    assert!(
        haystack.contains(needle),
        "expected {label} to contain {needle:?}, actual:\n{haystack}"
    );
}

fn http_get_json(addr: &str, path: &str) -> serde_json::Value {
    let started = std::time::Instant::now();
    loop {
        match std::net::TcpStream::connect(addr) {
            Ok(mut stream) => {
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                    .expect("set API read timeout");
                write!(
                    stream,
                    "GET {path} HTTP/1.1\r\nhost: {addr}\r\nconnection: close\r\n\r\n"
                )
                .expect("write API request");
                stream.flush().expect("flush API request");
                let body = read_http_response_body(&mut stream);
                if body.trim().is_empty() && started.elapsed() < std::time::Duration::from_secs(5) {
                    std::thread::sleep(std::time::Duration::from_millis(25));
                    continue;
                }
                return serde_json::from_str(&body).unwrap_or_else(|err| {
                    panic!("parse API response for {path}: {err}; body:\n{body}")
                });
            }
            Err(err) if started.elapsed() < std::time::Duration::from_secs(5) => {
                let _ = err;
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            Err(err) => panic!("connect to CSM API {addr} for {path}: {err}"),
        }
    }
}

fn otlp_attr_string<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    attrs.as_array()?.iter().find_map(|attr| {
        (attr.get("key")?.as_str()? == key)
            .then(|| attr.get("value")?.get("stringValue")?.as_str())
            .flatten()
    })
}

fn copy_dir_all(source: &std::path::Path, dest: &std::path::Path) {
    fs::create_dir_all(dest).expect("create copied bundle dir");
    for entry in fs::read_dir(source).expect("read source dir") {
        let entry = entry.expect("read source entry");
        let child_source = entry.path();
        let child_dest = dest.join(entry.file_name());
        if child_source.is_dir() {
            copy_dir_all(&child_source, &child_dest);
        } else {
            fs::copy(&child_source, &child_dest).expect("copy bundle file");
        }
    }
}

fn assert_stage_failure<F>(
    bundle: &std::path::Path,
    bad_bundle: &std::path::Path,
    out_dir: &std::path::Path,
    mutate: F,
    expected_stderr: &str,
) where
    F: FnOnce(&std::path::Path),
{
    copy_dir_all(bundle, bad_bundle);
    mutate(bad_bundle);
    let out = run_csm(&[
        "continuity",
        "stage",
        "--bundle",
        bad_bundle.to_str().expect("utf8 bad bundle"),
        "--out",
        out_dir.to_str().expect("utf8 stage dir"),
        "--target-host",
        "ec2-staging",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected stage failure containing {expected_stderr}, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains(expected_stderr),
        "expected stderr to contain {expected_stderr}, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn agent_run_writes_bounded_cycles_and_status() {
    let root = unique_test_temp_dir("agent-smoke");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: smoke-agent
display_name: Smoke Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "3",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        out.status.success(),
        "expected agent run success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"completed_cycle_count\": 3"),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/status.json").exists());
    assert!(root.join("state/agent_spec.locked.json").exists());
    assert!(root.join("state/continuity.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());
    assert!(root.join("state/cycle_ledger.jsonl").exists());
    assert!(root.join("state/provider_binding_history.jsonl").exists());
    assert!(root.join("state/memory_index.json").exists());
    for cycle_id in ["cycle-000001", "cycle-000002", "cycle-000003"] {
        let cycle_dir = root.join("state/cycles").join(cycle_id);
        for artifact in [
            "cycle_manifest.json",
            "observations.json",
            "decision_request.json",
            "decision_result.json",
            "run_ref.json",
            "memory_writes.jsonl",
            "guardrail_report.json",
            "cycle_summary.md",
        ] {
            assert!(
                cycle_dir.join(artifact).exists(),
                "missing {artifact} for {cycle_id}"
            );
        }
    }
    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3);
    let continuity =
        fs::read_to_string(root.join("state/continuity.json")).expect("read continuity");
    assert!(continuity.contains(r#""continuity_kind": "pre_v0_92_handle""#));
    assert!(continuity.contains(r#""latest_cycle_id": "cycle-000003""#));

    let human_status = run_adl(&["agent", "status", "--spec", spec_str]);
    assert!(
        human_status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&human_status.stderr)
    );
    let human_stdout = String::from_utf8_lossy(&human_status.stdout);
    assert!(human_stdout.contains("agent: smoke-agent"));
    assert!(human_stdout.contains("state: completed"));

    let status = run_adl(&["agent", "status", "--spec", spec_str, "--json"]);
    assert!(
        status.status.success(),
        "expected agent status success, stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let status_stdout = String::from_utf8_lossy(&status.stdout);
    assert!(
        status_stdout.contains("\"state\": \"completed\""),
        "stdout:\n{status_stdout}"
    );

    let inspect = run_adl(&["agent", "inspect", "--spec", spec_str, "--json"]);
    assert!(
        inspect.status.success(),
        "expected agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    let inspect_stdout = String::from_utf8_lossy(&inspect.stdout);
    assert!(
        inspect_stdout.contains("\"schema\": \"adl.long_lived_agent_inspection_packet.v1\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"manifest\": \"cycles/cycle-000003/cycle_manifest.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout
            .contains("\"guardrail_report\": \"cycles/cycle-000003/guardrail_report.json\""),
        "stdout:\n{inspect_stdout}"
    );
    assert!(
        inspect_stdout.contains("\"path\": \"continuity_checkpoint.json\""),
        "stdout:\n{inspect_stdout}"
    );

    let human_inspect = run_adl(&["agent", "inspect", "--spec", spec_str]);
    assert!(
        human_inspect.status.success(),
        "expected human agent inspect success, stderr:\n{}",
        String::from_utf8_lossy(&human_inspect.stderr)
    );
    let human_inspect_stdout = String::from_utf8_lossy(&human_inspect.stdout);
    assert!(human_inspect_stdout.contains("agent: smoke-agent"));
    assert!(human_inspect_stdout.contains("cycle: cycle-000003 success"));
    assert!(human_inspect_stdout.contains("proof: pass"));
}

#[test]
fn agent_restart_restores_checkpoint_and_reuses_next_cycle_id_without_duplicates() {
    let root = unique_test_temp_dir("agent-restart");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: restart-agent
display_name: Restart Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let first = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "2",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        first.status.success(),
        "expected first run success, stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );

    fs::remove_file(root.join("state/status.json")).expect("remove status to force restore");

    let restored = run_adl(&["agent", "status", "--spec", spec_str, "--json"]);
    assert!(
        restored.status.success(),
        "expected restored status success, stderr:\n{}",
        String::from_utf8_lossy(&restored.stderr)
    );
    let restored_stdout = String::from_utf8_lossy(&restored.stdout);
    assert!(
        restored_stdout.contains("\"last_cycle_id\": \"cycle-000002\""),
        "stdout:\n{restored_stdout}"
    );

    let replay_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_replay_manifest.json"))
            .expect("read continuity replay manifest"),
    )
    .expect("parse continuity replay manifest");
    assert_eq!(
        replay_manifest["expected_resume"]["next_cycle_id"],
        "cycle-000003"
    );

    let second = run_adl(&[
        "agent",
        "run",
        "--spec",
        spec_str,
        "--max-cycles",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        second.status.success(),
        "expected resumed run success, stderr:\n{}",
        String::from_utf8_lossy(&second.stderr)
    );

    let ledger =
        fs::read_to_string(root.join("state/cycle_ledger.jsonl")).expect("read cycle ledger");
    assert_eq!(ledger.lines().count(), 3, "ledger:\n{ledger}");
    assert!(ledger.contains("\"cycle_id\":\"cycle-000001\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000002\""));
    assert!(ledger.contains("\"cycle_id\":\"cycle-000003\""));

    let checkpoint: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/continuity_checkpoint.json"))
            .expect("read continuity checkpoint"),
    )
    .expect("parse continuity checkpoint");
    assert_eq!(checkpoint["latest_cycle_id"], "cycle-000003");
}

#[test]
fn csm_daemon_writes_status_checkpoints_and_otel_observability() {
    let root = unique_test_temp_dir("csm-daemon");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-agent
display_name: Daemon Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp02_smoke_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/daemon-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let spec_str = spec.to_str().expect("utf8 path");
    let log_str = observability_log.to_str().expect("utf8 log path");
    let otel_log_str = otel_log.to_str().expect("utf8 otel log path");
    let otel_status_str = otel_status.to_str().expect("utf8 otel status path");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            ("ADL_OBSERVABILITY_LOG", log_str),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
            ("ADL_OTEL_LOG", otel_log_str),
            ("ADL_OTEL_STATUS", otel_status_str),
        ],
    );
    assert!(
        out.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"schema\": \"adl.long_lived_agent_daemon_status.v1\""),
        "stdout:\n{stdout}"
    );
    assert!(
        stdout.contains("\"state\": \"completed\""),
        "stdout:\n{stdout}"
    );
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["unsupported_permanence_claims"][0],
        "not_os_boot_persistent"
    );
    assert_eq!(daemon_status["trace_id"], "agent.daemon-agent.daemon");

    let operator_events = read_text_or_missing(&root.join("state/operator_events.jsonl"));
    assert_text_contains(
        &operator_events,
        "\"event\":\"daemon_started\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"event\":\"child_spawn\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"event\":\"checkpoint_write\"",
        "operator events",
    );
    assert_text_contains(
        &operator_events,
        "\"trace_id\":\"agent.daemon-agent.daemon\"",
        "operator events",
    );
    assert_text_contains(&operator_events, "\"otel\"", "operator events");

    let observability = read_text_or_missing(&observability_log);
    assert_text_contains(&observability, "command=csm", "observability log");
    assert_text_contains(&observability, "stage=csm_daemon", "observability log");
    assert_text_contains(&observability, "stage=daemon_started", "observability log");
    assert_text_contains(
        &observability,
        "stage=checkpoint_write",
        "observability log",
    );
    assert_text_contains(
        &observability,
        "otel_service_name=csm-runtime-daemon",
        "observability log",
    );
    assert_text_contains(
        &observability,
        "trace_id=agent.daemon-agent.daemon",
        "observability log",
    );

    let otel_events = read_text_or_missing(&otel_log);
    let otel_event_lines = otel_events
        .lines()
        .filter(|line| line.contains("\"schema\":\"adl.otel.event.v1\""))
        .count();
    assert!(
        otel_event_lines >= 4,
        "expected at least four retained OTel JSONL events, got {otel_event_lines}; events:\n{otel_events}\nobservability:\n{observability}\noperator_events:\n{operator_events}"
    );
    assert_text_contains(
        &otel_events,
        "\"schema\":\"adl.otel.event.v1\"",
        "OTel JSONL",
    );
    assert_text_contains(
        &otel_events,
        "\"name\":\"csm.daemon_started\"",
        "OTel JSONL",
    );
    assert_text_contains(&otel_events, "\"name\":\"csm.csm_daemon\"", "OTel JSONL");
    assert_text_contains(
        &otel_events,
        "\"trace_id\":\"agent.daemon-agent.daemon\"",
        "OTel JSONL",
    );
    assert_text_contains(
        &otel_events,
        "\"service.name\":\"csm-runtime-daemon\"",
        "OTel JSONL",
    );

    let otel_status: serde_json::Value =
        serde_json::from_str(&read_text_or_missing(&otel_status)).expect("parse otel status");
    assert_eq!(otel_status["schema"], "adl.otel.monitor_status.v1");
    let status_event_count = otel_status["event_count"].as_u64().expect("event count");
    assert!(
        status_event_count >= 4,
        "expected OTel monitor status to observe at least four events, got {status_event_count}; status:\n{}\nevents:\n{otel_events}",
        serde_json::to_string_pretty(&otel_status).expect("render otel status")
    );
    assert_eq!(
        otel_status["last_trace_id"],
        "agent.daemon-agent.daemon",
        "status:\n{}\nevents:\n{otel_events}",
        serde_json::to_string_pretty(&otel_status).expect("render otel status")
    );
}

#[test]
fn csm_runtime_api_serves_status_health_ready_metrics_and_events() {
    let root = unique_test_temp_dir("csm-runtime-api");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: api-agent
display_name: API Agent
state_root: state
workflow:
  kind: demo_adapter
  name: wp07_api_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/api-agent
  write_policy: append_only
"#,
    )
    .expect("write API agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let spec_str = spec.to_str().expect("utf8 spec path");
    let daemon = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OBSERVABILITY_HEARTBEAT_MS", "25"),
            (
                "ADL_OTEL_LOG",
                otel_log.to_str().expect("utf8 otel log path"),
            ),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
        ],
    );
    assert!(
        daemon.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&daemon.stderr)
    );
    let backpressure_dir = root.join("backpressure-proof");
    let backpressure = run_csm_with_env(
        &[
            "backpressure",
            "prove",
            "--spec",
            spec_str,
            "--out",
            backpressure_dir.to_str().expect("utf8 backpressure dir"),
            "--profile",
            "soak2",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
        ],
    );
    assert!(
        backpressure.status.success(),
        "expected backpressure proof success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&backpressure.stdout),
        String::from_utf8_lossy(&backpressure.stderr)
    );
    let backpressure_stdout: serde_json::Value =
        serde_json::from_slice(&backpressure.stdout).expect("parse backpressure stdout");
    assert_eq!(
        backpressure_stdout["schema"],
        "adl.csm.backpressure_command_result.v1"
    );
    assert_eq!(backpressure_stdout["status"], "passed");
    let backpressure_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(backpressure_dir.join("backpressure_report.json"))
            .expect("read backpressure report"),
    )
    .expect("parse backpressure report");
    assert_eq!(
        backpressure_report["schema"],
        "adl.csm.backpressure_report.v1"
    );
    assert_eq!(
        backpressure_report["summary"]["required_state_silently_dropped"],
        false
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["action"],
        "safe_fail_serialize"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["status"],
        "verified"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["artifact_schema"],
        "adl.csm.safe_fail_bundle.v1"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["agent_outcome_state"],
        "sleeping"
    );
    assert_eq!(
        backpressure_report["safe_fail_action"]["recoverability_class"],
        "recoverable_sleeping"
    );
    let proved_surfaces: std::collections::BTreeSet<_> = backpressure_report["proof_cases"]
        .as_array()
        .expect("proof cases")
        .iter()
        .map(|case| case["surface"].as_str().expect("proof surface"))
        .collect();
    for expected_surface in [
        "runtime_loop",
        "event_export",
        "checkpoint_write",
        "snapshot_diff",
        "dag_execution",
        "provider_call",
        "cloud_hook",
        "continuity_serialization",
    ] {
        assert!(
            proved_surfaces.contains(expected_surface),
            "missing proof case for {expected_surface}"
        );
    }
    assert!(root.join("state/csm_backpressure_state.json").exists());
    let safe_fail_bundle = root.join("state/safe_fail_bundle.json");
    let safe_fail_bundle_backup = root.join("state/safe_fail_bundle.json.bak");
    fs::rename(&safe_fail_bundle, &safe_fail_bundle_backup).expect("hide safe-fail bundle");
    let missing_bundle = run_csm(&[
        "backpressure",
        "prove",
        "--spec",
        spec_str,
        "--out",
        root.join("missing-bundle-backpressure")
            .to_str()
            .expect("utf8 missing bundle backpressure"),
        "--profile",
        "local",
        "--json",
    ]);
    fs::rename(&safe_fail_bundle_backup, &safe_fail_bundle).expect("restore safe-fail bundle");
    assert!(
        !missing_bundle.status.success(),
        "expected missing safe-fail bundle rejection"
    );
    assert!(String::from_utf8_lossy(&missing_bundle.stderr)
        .contains("missing required safe-fail bundle"));
    let bad_profile = run_csm(&[
        "backpressure",
        "prove",
        "--spec",
        spec_str,
        "--out",
        root.join("bad-backpressure")
            .to_str()
            .expect("utf8 bad backpressure"),
        "--profile",
        "unbounded",
        "--json",
    ]);
    assert!(
        !bad_profile.status.success(),
        "expected unsupported backpressure profile rejection"
    );
    assert!(String::from_utf8_lossy(&bad_profile.stderr)
        .contains("unsupported csm backpressure profile"));

    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("bind API probe port");
    let addr = probe.local_addr().expect("API probe addr");
    drop(probe);
    let mut child = std::process::Command::new(resolve_csm_exe())
        .args([
            "api",
            "serve",
            "--spec",
            spec_str,
            "--bind",
            &addr.to_string(),
            "--max-requests",
            "100",
            "--idle-timeout-ms",
            "1000",
            "--otel-status",
            otel_status.to_str().expect("utf8 otel status path"),
            "--otel-log",
            otel_log.to_str().expect("utf8 otel log path"),
            "--json",
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn csm api");
    let stdout = child.stdout.take().expect("api stdout pipe");
    let mut stdout = std::io::BufReader::new(stdout);
    let mut startup_line = String::new();
    std::io::BufRead::read_line(&mut stdout, &mut startup_line).expect("read API startup");
    assert!(
        startup_line.contains("\"status\":\"listening\""),
        "unexpected API startup line: {startup_line}"
    );

    let status = http_get_json(&addr.to_string(), "/status");
    let health = http_get_json(&addr.to_string(), "/health");
    let ready = http_get_json(&addr.to_string(), "/ready");
    let metrics = http_get_json(&addr.to_string(), "/metrics");
    let events = http_get_json(&addr.to_string(), "/events");

    let mut stderr = String::new();
    if let Some(mut pipe) = child.stderr.take() {
        use std::io::Read;
        pipe.read_to_string(&mut stderr).expect("read API stderr");
    }
    let status_code = child.wait().expect("wait csm api");
    let mut remaining_stdout = String::new();
    use std::io::Read;
    stdout
        .read_to_string(&mut remaining_stdout)
        .expect("read remaining API stdout");
    assert!(
        status_code.success(),
        "api stdout:\n{startup_line}{remaining_stdout}\nstderr:\n{stderr}"
    );

    assert_eq!(status["schema"], "adl.csm.runtime_api.status.v1");
    assert_eq!(status["runtime_owner"], "csm");
    assert_eq!(status["agent_instance_id"], "api-agent");
    assert_eq!(status["status"], "healthy");
    assert_eq!(status["ready"], "ready");
    assert_eq!(status["daemon_liveness"]["state"], "completed");
    assert_eq!(
        status["backpressure"]["schema"],
        "adl.csm.backpressure_state.v1"
    );
    assert_eq!(status["scheduler"]["status"], "integrated");
    assert_eq!(status["chronosense"]["status"], "integrated");
    assert_eq!(status["aee_resilience"]["status"], "integrated");
    assert_eq!(
        status["otel"]["status"]["schema"],
        "adl.otel.monitor_status.v1"
    );
    assert_eq!(health["schema"], "adl.csm.runtime_api.health.v1");
    assert_eq!(health["status"], "healthy");
    assert_eq!(ready["schema"], "adl.csm.runtime_api.ready.v1");
    assert_eq!(ready["ready"], "ready");
    assert_eq!(metrics["schema"], "adl.csm.runtime_api.metrics.v1");
    assert_eq!(metrics["gauges"]["backpressure_queue_depth"], 12);
    assert_eq!(metrics["gauges"]["backpressure_lag_ms"], 3100);
    assert_eq!(metrics["gauges"]["backpressure_deferred_count"], 23);
    assert_eq!(metrics["gauges"]["backpressure_shed_count"], 7);
    assert_eq!(
        metrics["states"]["backpressure_health"],
        "capacity_degraded"
    );
    assert_eq!(
        metrics["states"]["backpressure_safe_fail_action"],
        "safe_fail_serialize"
    );
    assert!(
        matches!(
            metrics["states"]["agent_state"].as_str(),
            Some("idle" | "completed")
        ),
        "unexpected metrics state: {}",
        metrics["states"]["agent_state"]
    );
    assert_eq!(events["schema"], "adl.csm.runtime_api.events.v1");
    assert!(events["events"]["entries"]
        .as_array()
        .expect("events array")
        .iter()
        .any(|event| event["event"] == "daemon_started"));
    let observability = fs::read_to_string(&observability_log).expect("read observability log");
    assert!(observability.contains("stage=backpressure_policy"));
    assert!(observability.contains("safe_fail_action=safe_fail_serialize"));

    for response in [&status, &health, &ready, &metrics, &events] {
        let raw = serde_json::to_string(response).expect("serialize API response");
        assert!(!raw.contains("/Users/"), "leaked host path:\n{raw}");
        assert!(
            !raw.contains("Authorization:"),
            "leaked auth header:\n{raw}"
        );
        assert!(!raw.contains("Bearer "), "leaked bearer token:\n{raw}");
    }
}

#[test]
fn csm_continuity_capsule_captures_stages_and_rejects_unsafe_bundles() {
    let root = unique_test_temp_dir("csm-continuity-capsule");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: continuity-agent
display_name: Continuity Agent
state_root: state
workflow:
  kind: demo_adapter
  name: continuity_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/continuity-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let daemon = run_csm(&[
        "daemon",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--max-restarts",
        "1",
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        daemon.status.success(),
        "expected daemon success, stderr:\n{}",
        String::from_utf8_lossy(&daemon.stderr)
    );

    let observability_log = root.join("continuity-observability.log");
    let bundle = root.join("continuity-capsule");
    let capture = run_csm_with_env(
        &[
            "continuity",
            "capture",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--out",
            bundle.to_str().expect("utf8 bundle"),
            "--source-host",
            "wuji",
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
        ],
    );
    assert!(
        capture.status.success(),
        "expected continuity capture success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&capture.stdout),
        String::from_utf8_lossy(&capture.stderr)
    );
    let capture_stdout: serde_json::Value =
        serde_json::from_slice(&capture.stdout).expect("parse capture stdout");
    assert_eq!(
        capture_stdout["schema"],
        "adl.csm.continuity_capsule_command_result.v1"
    );
    assert_eq!(capture_stdout["operation"], "capture");
    assert_eq!(capture_stdout["status"], "captured");

    let manifest_path = bundle.join("continuity_capsule_manifest.json");
    let manifest_text = fs::read_to_string(&manifest_path).expect("read continuity manifest");
    assert!(
        !manifest_text.contains(root.to_str().expect("root utf8")),
        "continuity capsule manifest leaked host path:\n{manifest_text}"
    );
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_text).expect("parse continuity manifest");
    assert_eq!(manifest["schema"], "adl.csm.continuity_capsule.v1");
    assert_eq!(manifest["format_version"], "csm.continuity-capsule.v1");
    assert_eq!(manifest["runtime_owner"], "csm");
    assert_eq!(manifest["source_host"], "wuji");
    assert_eq!(manifest["target_host"], "ec2-staging");
    assert_eq!(
        manifest["rebind_policy"]["aws"]["default_profile"],
        "agent-logic-admin"
    );
    assert!(manifest["artifacts"]
        .as_array()
        .expect("artifact array")
        .iter()
        .any(|artifact| artifact["role"] == "continuity_checkpoint"));
    let binary_segments = manifest["binary_segments"]
        .as_array()
        .expect("binary segment array");
    let checkpoint_segment = binary_segments
        .iter()
        .find(|segment| segment["role"] == "continuity_checkpoint_snapshot")
        .expect("continuity checkpoint binary segment");
    assert_eq!(checkpoint_segment["schema"], "adl.csm.snapshot_segment.v1");
    assert_eq!(
        checkpoint_segment["format_version"],
        "csm.snapshot-segment.v1"
    );
    assert_eq!(
        checkpoint_segment["source_ref"],
        "continuity_checkpoint.json"
    );
    assert!(checkpoint_segment["hash_address"]
        .as_str()
        .expect("hash address")
        .starts_with("sha256:"));
    assert!(bundle
        .join(
            checkpoint_segment["segment_ref"]
                .as_str()
                .expect("segment ref")
        )
        .exists());
    assert!(bundle.join("state/continuity_checkpoint.json").exists());
    assert!(bundle.join("state/operator_events.jsonl").exists());

    let staged = root.join("ec2-staged");
    let stage = run_csm_with_env(
        &[
            "continuity",
            "stage",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            staged.to_str().expect("utf8 staged"),
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
        ],
    );
    assert!(
        stage.status.success(),
        "expected continuity stage success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&stage.stdout),
        String::from_utf8_lossy(&stage.stderr)
    );
    let stage_stdout: serde_json::Value =
        serde_json::from_slice(&stage.stdout).expect("parse stage stdout");
    assert_eq!(stage_stdout["operation"], "stage");
    assert_eq!(stage_stdout["status"], "staged");
    let stage_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(staged.join("stage_report.json")).expect("read stage report"),
    )
    .expect("parse stage report");
    assert_eq!(
        stage_report["schema"],
        "adl.csm.continuity_capsule_stage_report.v1"
    );
    assert_eq!(stage_report["status"], "staged");
    assert!(staged
        .join("staged_state/continuity_checkpoint.json")
        .exists());

    let restored = root.join("restored-runtime");
    let restore = run_csm_with_env(
        &[
            "continuity",
            "restore",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            restored.to_str().expect("utf8 restored"),
            "--target-host",
            "ec2-staging",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
        ],
    );
    assert!(
        restore.status.success(),
        "expected continuity restore success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restore.stdout),
        String::from_utf8_lossy(&restore.stderr)
    );
    let restore_stdout: serde_json::Value =
        serde_json::from_slice(&restore.stdout).expect("parse restore stdout");
    assert_eq!(restore_stdout["operation"], "restore");
    assert_eq!(restore_stdout["status"], "restored");
    let restore_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(restored.join("restore_report.json")).expect("read restore report"),
    )
    .expect("parse restore report");
    assert_eq!(
        restore_report["schema"],
        "adl.csm.continuity_capsule_restore_report.v1"
    );
    assert_eq!(restore_report["status"], "restored");
    assert!(restored.join("agent.yaml").exists());
    assert!(restored.join("state/continuity_checkpoint.json").exists());

    let restored_daemon = run_csm(&[
        "daemon",
        "--spec",
        restored
            .join("agent.yaml")
            .to_str()
            .expect("utf8 restored spec"),
        "--max-restarts",
        "1",
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        restored_daemon.status.success(),
        "expected restored daemon fire-up success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restored_daemon.stdout),
        String::from_utf8_lossy(&restored_daemon.stderr)
    );

    let ec2_blocked = root.join("ec2-blocked");
    let ec2_stage = run_csm(&[
        "continuity",
        "stage",
        "--bundle",
        bundle.to_str().expect("utf8 bundle"),
        "--out",
        ec2_blocked.to_str().expect("utf8 ec2 blocked"),
        "--target-host",
        "ec2",
        "--json",
    ]);
    assert!(
        ec2_stage.status.success(),
        "expected bounded EC2 stage packet success, stderr:\n{}",
        String::from_utf8_lossy(&ec2_stage.stderr)
    );
    let ec2_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(ec2_blocked.join("stage_report.json")).expect("read ec2 report"),
    )
    .expect("parse ec2 report");
    assert_eq!(ec2_report["status"], "blocked");
    assert_eq!(ec2_report["rebind_policy"]["target_host"], "ec2");
    assert_eq!(
        ec2_report["blockers"][0]["required_profile"],
        "agent-logic-admin"
    );

    let drill_dir = root.join("fire-drill");
    let drill = run_csm_with_env(
        &[
            "continuity",
            "drill",
            "--bundle",
            bundle.to_str().expect("utf8 bundle"),
            "--out",
            drill_dir.to_str().expect("utf8 drill"),
            "--target-host",
            "local",
            "--cadence",
            "daily",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability"),
            ),
        ],
    );
    assert!(
        drill.status.success(),
        "expected continuity drill success, stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&drill.stdout),
        String::from_utf8_lossy(&drill.stderr)
    );
    let drill_stdout: serde_json::Value =
        serde_json::from_slice(&drill.stdout).expect("parse drill stdout");
    assert_eq!(drill_stdout["operation"], "fire_drill");
    assert_eq!(drill_stdout["status"], "passed");
    assert_eq!(drill_stdout["report_ref"], "fire_drill_report.json");
    let drill_report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(drill_dir.join("fire_drill_report.json")).expect("read drill report"),
    )
    .expect("parse drill report");
    assert_eq!(
        drill_report["schema"],
        "adl.csm.continuity_fire_drill_report.v1"
    );
    assert_eq!(drill_report["status"], "passed");
    assert_eq!(drill_report["source_bundle_ref"], "../continuity-capsule");
    assert_eq!(
        drill_report["manifest_ref"],
        "../continuity-capsule/continuity_capsule_manifest.json"
    );
    assert_eq!(drill_report["cadence_policy"]["selected"], "daily");
    assert_eq!(drill_report["safety"]["mutates_live_runtime_state"], false);
    assert_eq!(drill_report["stage"]["status"], "staged");
    assert_eq!(drill_report["restore"]["status"], "restored");
    assert!(drill_dir
        .join("restored-runtime/state/continuity_checkpoint.json")
        .exists());
    let negative_cases = drill_report["negative_cases"]
        .as_array()
        .expect("negative cases");
    assert_eq!(negative_cases.len(), 2);
    assert!(negative_cases
        .iter()
        .all(|case| case["status"] == "failed_as_expected"));
    assert_eq!(
        drill_report["rto_rpo_measurement"]["rpo_scope"],
        "selected_continuity_capsule_point_in_time"
    );
    let overlapping_drill = run_csm(&[
        "continuity",
        "drill",
        "--bundle",
        bundle.to_str().expect("utf8 bundle"),
        "--out",
        bundle.join("nested-drill").to_str().expect("utf8 nested"),
        "--target-host",
        "local",
        "--json",
    ]);
    assert!(
        !overlapping_drill.status.success(),
        "expected overlapping drill output rejection"
    );
    assert!(String::from_utf8_lossy(&overlapping_drill.stderr)
        .contains("must be disjoint from --bundle"));
    assert!(
        bundle.join("continuity_capsule_manifest.json").exists(),
        "overlapping drill rejection must preserve source bundle"
    );

    let observability =
        fs::read_to_string(&observability_log).expect("read continuity observability");
    assert!(observability.contains("stage=continuity_capsule_capture"));
    assert!(observability.contains("stage=continuity_capsule_stage"));
    assert!(observability.contains("stage=continuity_capsule_restore"));
    assert!(observability.contains("stage=continuity_fire_drill"));
    assert!(observability.contains("otel_service_name=csm-runtime-daemon"));

    assert_stage_failure(
        &bundle,
        &root.join("bad-version"),
        &root.join("bad-version-stage"),
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read bad manifest"))
                    .expect("parse bad manifest");
            value["format_version"] = serde_json::json!("csm.continuity-capsule.v0");
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize bad manifest"),
            )
            .expect("write bad manifest");
        },
        "unsupported continuity capsule format version",
    );
    assert_stage_failure(
        &bundle,
        &root.join("missing-file"),
        &root.join("missing-file-stage"),
        |bad| {
            fs::remove_file(bad.join("state/status.json")).expect("remove staged status");
        },
        "missing bundle artifact",
    );
    assert_stage_failure(
        &bundle,
        &root.join("truncated-manifest"),
        &root.join("truncated-manifest-stage"),
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["artifacts"] = serde_json::json!([{
                "role": "recoverable_status",
                "source_ref": "status.json",
                "bundle_ref": "state/status.json",
                "sha256": value["artifacts"][1]["sha256"].clone(),
                "bytes": value["artifacts"][1]["bytes"].clone(),
                "required": true
            }]);
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "missing required artifact role",
    );
    assert_stage_failure(
        &bundle,
        &root.join("path-leak"),
        &root.join("path-leak-stage"),
        |bad| {
            fs::write(
                bad.join("state/status.json"),
                format!("{{\"path\":\"{}\"}}", bad.display()),
            )
            .expect("write path leak");
        },
        "host-private absolute path",
    );
    assert_stage_failure(
        &bundle,
        &root.join("linux-path-leak"),
        &root.join("linux-path-leak-stage"),
        |bad| {
            fs::write(
                bad.join("state/status.json"),
                "{\"path\":\"/home/runner/work/agent-design-language\"}\n",
            )
            .expect("write linux path leak");
        },
        "host-private absolute path",
    );
    assert_stage_failure(
        &bundle,
        &root.join("credential-leak"),
        &root.join("credential-leak-stage"),
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["api_key"] = serde_json::json!("not-exportable");
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "credential-like key",
    );
    assert_stage_failure(
        &bundle,
        &root.join("missing-binary-segment-manifest-entry"),
        &root.join("missing-binary-segment-manifest-entry-stage"),
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            value["binary_segments"] = serde_json::json!([]);
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "missing required binary segment role",
    );
    assert_stage_failure(
        &bundle,
        &root.join("divergent-binary-segment-payload"),
        &root.join("divergent-binary-segment-payload-stage"),
        |bad| {
            let path = bad.join("continuity_capsule_manifest.json");
            let mut value: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).expect("read manifest"))
                    .expect("parse manifest");
            let segment = value["binary_segments"][0]
                .as_object_mut()
                .expect("binary segment object");
            segment.insert(
                "payload_sha256".to_string(),
                serde_json::json!(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                ),
            );
            fs::write(
                &path,
                serde_json::to_vec_pretty(&value).expect("serialize manifest"),
            )
            .expect("write manifest");
        },
        "payload hash does not match retained artifact",
    );
    assert_stage_failure(
        &bundle,
        &root.join("corrupted-segment"),
        &root.join("corrupted-segment-stage"),
        |bad| {
            fs::write(
                bad.join("segments/continuity_checkpoint.snapshot.segment"),
                b"not-a-valid-segment",
            )
            .expect("corrupt binary segment");
        },
        "hash mismatch",
    );
    assert_stage_failure(
        &bundle,
        &root.join("corrupted-manifest"),
        &root.join("corrupted-manifest-stage"),
        |bad| {
            fs::write(bad.join("continuity_capsule_manifest.json"), b"{").expect("corrupt");
        },
        "failed parsing",
    );

    let unsupported_stage = root.join("unsupported-stage");
    let unsupported = run_csm(&[
        "continuity",
        "stage",
        "--bundle",
        bundle.to_str().expect("utf8 bundle"),
        "--out",
        unsupported_stage.to_str().expect("utf8 stage"),
        "--target-host",
        "mars",
        "--json",
    ]);
    assert!(
        !unsupported.status.success(),
        "expected unsupported target failure"
    );
    assert!(String::from_utf8_lossy(&unsupported.stderr)
        .contains("unsupported continuity capsule target host"));
}

#[test]
fn csm_daemon_exports_otlp_http_json_to_loopback_collector() {
    let root = unique_test_temp_dir("csm-daemon-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-otlp-agent
display_name: Daemon OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: otlp_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 2
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/daemon-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let (endpoint, captured, collector) = spawn_loopback_otlp_collector();
    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("utf8 otel path")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_OTEL_EXPORTER_OTLP_ENDPOINT", endpoint.as_str()),
            ("ADL_OTEL_EXPORTER_TIMEOUT_MS", "2000"),
        ],
    );
    assert!(
        out.status.success(),
        "expected daemon OTLP success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    collector.join().expect("collector joined");
    let exported = captured.try_iter().collect::<Vec<_>>();
    let mut span_names = std::collections::BTreeSet::new();
    let mut service_names = std::collections::BTreeSet::new();
    let mut trace_id_lengths = std::collections::BTreeSet::new();
    for body in &exported {
        let payload: serde_json::Value = serde_json::from_str(body).expect("parse otlp payload");
        for resource_span in payload["resourceSpans"].as_array().expect("resource spans") {
            if let Some(service_name) =
                otlp_attr_string(&resource_span["resource"]["attributes"], "service.name")
            {
                service_names.insert(service_name.to_string());
            }
            for scope_span in resource_span["scopeSpans"].as_array().expect("scope spans") {
                for span in scope_span["spans"].as_array().expect("spans") {
                    span_names.insert(span["name"].as_str().expect("span name").to_string());
                    trace_id_lengths.insert(span["traceId"].as_str().expect("trace id").len());
                    assert_eq!(span["kind"], 1);
                    assert!(span["startTimeUnixNano"].as_str().is_some());
                    assert!(span["endTimeUnixNano"].as_str().is_some());
                    assert_eq!(span["spanId"].as_str().expect("span id").len(), 16);
                }
            }
        }
    }
    assert!(service_names.contains("csm-runtime-daemon"));
    assert!(span_names.contains("csm.daemon_started"));
    assert!(span_names.contains("csm.checkpoint_write"));
    assert!(trace_id_lengths.contains(&32));
    let exported_text = exported.join("\n");
    assert!(!exported_text.contains("adl.otlp_http_json.export.v1"));
    assert!(!exported_text.contains(root.to_str().expect("root utf8")));

    let status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&otel_status).expect("read otel status"))
            .expect("parse otel status");
    assert_eq!(status["schema"], "adl.otel.monitor_status.v1");
    assert_eq!(status["exporter"]["schema"], "adl.otel.exporter_status.v1");
    assert_eq!(status["exporter"]["protocol"], "otlp_http_json");
    assert_eq!(status["exporter"]["status"], "success");
    assert_eq!(status["exporter"]["endpoint"], "<configured>");
}

#[test]
fn csm_daemon_executes_adl_workflow_dag_with_aee_runtime_trace() {
    let root = unique_test_temp_dir("csm-adl-workflow");
    let spec = root.join("agent.yaml");
    let workflow = fixture_path("examples/v0-3-scheduler-max-concurrency.adl.yaml");
    fs::write(
        &spec,
        format!(
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-dag-agent
display_name: CSM DAG Agent
state_root: state
workflow:
  kind: adl_workflow
  name: scheduler_max_concurrency
  path: {}
  run_args: {{}}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/csm-dag-agent
  write_policy: append_only
"#,
            workflow.display()
        ),
    )
    .expect("write agent spec");

    let observability_log = root.join("observability.log");
    let otel_log = root.join("otel.jsonl");
    let otel_status = root.join("otel-status.json");
    let mock = fixture_path("tools/mock_ollama_v0_4.sh");
    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_csm_with_env(
        &[
            "daemon",
            "--spec",
            spec_str,
            "--max-restarts",
            "1",
            "--checkpoint-interval-secs",
            "1",
            "--no-sleep",
            "--json",
        ],
        &[
            ("ADL_OBSERVABILITY_STDERR", "0"),
            (
                "ADL_OBSERVABILITY_LOG",
                observability_log.to_str().expect("utf8 observability path"),
            ),
            ("ADL_OTEL_LOG", otel_log.to_str().expect("utf8 otel path")),
            (
                "ADL_OTEL_STATUS",
                otel_status.to_str().expect("utf8 otel status path"),
            ),
            ("ADL_OLLAMA_BIN", mock.to_str().expect("utf8 mock path")),
        ],
    );
    assert!(
        out.status.success(),
        "expected csm DAG runtime success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let cycle_root = root.join("state/cycles/cycle-000001");
    let run_status_path = cycle_root.join("csm_adl_run_status.json");
    assert!(run_status_path.exists());
    assert!(cycle_root.join("adl_runtime").exists());
    let run_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&run_status_path).expect("read run status"))
            .expect("parse run status");
    assert_eq!(run_status["schema"], "adl.csm.adl_workflow_run_status.v1");
    assert_eq!(run_status["runtime_owner"], "csm");
    assert_eq!(run_status["adl_role"], "tooling_control_plane");
    assert_eq!(run_status["status"], "success");
    assert_eq!(run_status["step_count"], 4);
    assert_eq!(run_status["scheduler_policy"]["max_concurrency"], 2);
    assert_eq!(run_status["scheduler_policy"]["source"], "run_default");
    assert_eq!(run_status["records"][0]["step_id"], "fork.a");
    assert_eq!(run_status["records"][0]["status"], "success");
    assert!(run_status["trace_events"]
        .as_array()
        .expect("trace events array")
        .iter()
        .any(|event| event
            .as_str()
            .expect("trace event")
            .contains("SchedulerPolicy max_concurrency=2 source=run_default")));
    assert!(
        run_status["trace_events"]
            .as_array()
            .expect("trace events array")
            .iter()
            .any(|event| event
                .as_str()
                .expect("trace event")
                .contains("RuntimeResilienceDecision")),
        "expected retained AEE/runtime resilience trace: {run_status}"
    );
    assert_eq!(
        run_status["aee_resilience_trace"],
        "retained_in_trace_events"
    );

    let run_ref: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(cycle_root.join("run_ref.json")).unwrap())
            .expect("parse run ref");
    assert_eq!(run_ref["run_status_ref"], "csm_adl_run_status.json");
    assert!(run_ref["execution_note"]
        .as_str()
        .expect("execution note")
        .contains("CSM executed the configured ADL DAG"));

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(
        daemon_status["runtime_capabilities"]["chronosense"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["aee"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["scheduler_watcher"]["status"],
        "integrated"
    );
    assert_eq!(
        daemon_status["runtime_capabilities"]["resilience_middleware"]["status"],
        "integrated"
    );
}

#[test]
fn csm_owns_daemon_and_adl_agent_daemon_is_removed() {
    let help = run_csm(&["--help"]);
    assert!(
        help.status.success(),
        "expected csm help success, stderr:\n{}",
        String::from_utf8_lossy(&help.stderr)
    );
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(help_stdout.contains("csm daemon --spec"));
    assert!(help_stdout.contains("dedicated runtime owner binary"));

    let removed_agent = run_adl(&["agent", "daemon", "--help"]);
    assert!(
        !removed_agent.status.success(),
        "expected adl agent daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_agent.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_agent.stderr);
    assert!(
        stderr.contains("unknown agent subcommand 'daemon'"),
        "stderr:\n{stderr}"
    );

    let removed_adl_csm = run_adl(&["csm", "daemon", "--help"]);
    assert!(
        !removed_adl_csm.status.success(),
        "expected adl csm daemon removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm.stderr);
    assert!(
        stderr.contains("csm daemon is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );

    let removed_adl_csm_service = run_adl(&["csm", "service", "install", "--help"]);
    assert!(
        !removed_adl_csm_service.status.success(),
        "expected adl csm service removal, stdout:\n{}",
        String::from_utf8_lossy(&removed_adl_csm_service.stdout)
    );
    let stderr = String::from_utf8_lossy(&removed_adl_csm_service.stderr);
    assert!(
        stderr.contains("csm service is owned by the standalone csm runtime binary"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_service_install_writes_launchd_envelope_without_adl_runtime_owner() {
    let root = unique_test_temp_dir("csm-service-install");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-install-agent
display_name: Service Install Agent
state_root: runtime-state
workflow:
  kind: demo_adapter
  name: service_install_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-install-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "launchd",
        "--label",
        "com.agentlogic.csm.test-install",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--otlp-endpoint",
        "http://127.0.0.1:4318/v1/traces",
        "--otlp-timeout-ms",
        "750",
        "--json",
    ]);
    assert!(
        out.status.success(),
        "expected service install success, stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_manifest.json")).expect("manifest"),
    )
    .expect("parse manifest");
    assert_eq!(manifest["schema"], "adl.csm.service_manifest.v1");
    assert_eq!(manifest["runtime_owner"], "csm");
    assert_eq!(manifest["manager"], "launchd");
    assert_eq!(manifest["checkpoint_interval_secs"], 1);
    assert_eq!(manifest["otlp_endpoint"], "http://127.0.0.1:4318/v1/traces");
    assert_eq!(manifest["otlp_timeout_ms"], 750);
    assert!(manifest["daemon_status"]
        .as_str()
        .expect("daemon status path")
        .ends_with("runtime-state/daemon_status.json"));
    assert!(manifest["continuity_checkpoint"]
        .as_str()
        .expect("checkpoint path")
        .ends_with("runtime-state/continuity_checkpoint.json"));
    assert!(manifest["unsupported_permanence_claims"]
        .as_array()
        .expect("nonclaims")
        .iter()
        .any(|value| value == "host_reboot_survival_not_proven"));

    let plist = fs::read_to_string(service_root.join("csm.launchd.plist")).expect("plist");
    assert!(plist.contains("<key>KeepAlive</key>"));
    assert!(plist.contains("<string>daemon</string>"));
    assert!(plist.contains("ADL_OTEL_STATUS"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_OTLP_ENDPOINT"));
    assert!(plist.contains("http://127.0.0.1:4318/v1/traces"));
    assert!(plist.contains("ADL_OTEL_EXPORTER_TIMEOUT_MS"));
    assert!(!plist.contains("adl agent daemon"));

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(service_root.join("service_status.json")).expect("service status"),
    )
    .expect("parse service status");
    assert_eq!(status["schema"], "adl.csm.service_status.v1");
    assert_eq!(status["service_state"], "installed");
    assert_eq!(status["broad_process_scan"], false);
    assert_eq!(status["uses_ps"], false);
    assert_eq!(status["otlp_exporter_configured"], true);
    assert_eq!(status["otlp_endpoint_ref"], "<configured>");
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint() {
    let root = unique_test_temp_dir("csm-service-secret-otlp");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-agent
display_name: Service Secret OTLP Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let out = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        root.join("service").to_str().expect("utf8 service root"),
        "--otlp-endpoint",
        "https://collector.example.invalid/v1/traces?token=secret",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected secret-bearing endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_service_install_rejects_secret_bearing_otlp_endpoint_from_env() {
    let root = unique_test_temp_dir("csm-service-secret-otlp-env");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-secret-otlp-env-agent
display_name: Service Secret OTLP Env Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_secret_otlp_env_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-secret-otlp-env-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let out = run_csm_with_env(
        &[
            "service",
            "install",
            "--spec",
            spec.to_str().expect("utf8 spec"),
            "--service-root",
            service_root.to_str().expect("utf8 service root"),
            "--json",
        ],
        &[(
            "ADL_OTEL_EXPORTER_OTLP_ENDPOINT",
            "https://user:secret@collector.example.invalid/v1/traces",
        )],
    );
    assert!(
        !out.status.success(),
        "expected env endpoint rejection, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("--otlp-endpoint must not contain credentials"),
        "stderr:\n{stderr}"
    );
    assert!(!service_root.join("service_manifest.json").exists());
    assert!(!service_root.join("csm.launchd.plist").exists());
}

#[test]
fn csm_service_local_start_stop_retains_status_checkpoint_and_observability() {
    let root = unique_test_temp_dir("csm-service-local");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-local-agent
display_name: Service Local Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_local_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-local-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--label",
        "com.agentlogic.csm.test-local",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        start.status.success(),
        "start stderr:\n{}",
        String::from_utf8_lossy(&start.stderr)
    );
    std::thread::sleep(std::time::Duration::from_millis(1500));

    let status = run_csm(&[
        "service",
        "status",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        status.status.success(),
        "status stderr:\n{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&status.stdout).expect("parse service status stdout");
    assert_eq!(service_status["runtime_owner"], "csm");
    assert_eq!(service_status["broad_process_scan"], false);
    assert_eq!(service_status["uses_ps"], false);
    assert!(root.join("state/daemon_status.json").exists());
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(service_root.join("logs/observability.log").exists());
    assert!(service_root.join("logs/otel_status.json").exists());

    let stop = run_csm(&[
        "service",
        "stop",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        stop.status.success(),
        "stop stderr:\n{}",
        String::from_utf8_lossy(&stop.stderr)
    );
    let service_status: serde_json::Value =
        serde_json::from_slice(&stop.stdout).expect("parse stop status stdout");
    assert_eq!(service_status["service_state"], "stopped_or_requested");
    let agent_status: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(root.join("state/status.json")).unwrap())
            .expect("parse agent status");
    assert_eq!(agent_status["state"], "stopped");
    assert_eq!(
        agent_status["last_error"]["class"],
        "operator_stop_requested"
    );
}

#[test]
fn csm_service_local_start_refuses_unverified_live_pid_metadata() {
    let root = unique_test_temp_dir("csm-service-unverified-pid");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: service-unverified-pid-agent
display_name: Service Unverified PID Agent
state_root: state
workflow:
  kind: demo_adapter
  name: service_unverified_pid_probe
  run_args: {}
heartbeat:
  interval_secs: 10
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
memory:
  namespace: smoke/service-unverified-pid-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");
    let service_root = root.join("service");
    let csm_bin = resolve_csm_exe();
    let install = run_csm(&[
        "service",
        "install",
        "--spec",
        spec.to_str().expect("utf8 spec"),
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--manager",
        "local",
        "--label",
        "com.agentlogic.csm.test-unverified-pid",
        "--csm-bin",
        csm_bin.to_str().expect("utf8 csm bin"),
        "--checkpoint-interval-secs",
        "1",
        "--json",
    ]);
    assert!(
        install.status.success(),
        "install stderr:\n{}",
        String::from_utf8_lossy(&install.stderr)
    );
    fs::write(
        service_root.join("csm-service.pid"),
        std::process::id().to_string(),
    )
    .expect("write live but unowned pid");

    let start = run_csm(&[
        "service",
        "start",
        "--service-root",
        service_root.to_str().expect("utf8 service root"),
        "--json",
    ]);
    assert!(
        !start.status.success(),
        "expected unverified live pid refusal, stdout:\n{}",
        String::from_utf8_lossy(&start.stdout)
    );
    let stderr = String::from_utf8_lossy(&start.stderr);
    assert!(
        stderr.contains("refused live but unverified pid metadata"),
        "stderr:\n{stderr}"
    );
}

#[test]
fn csm_daemon_restart_budget_failure_leaves_recoverable_checkpoint() {
    let root = unique_test_temp_dir("csm-daemon-failure");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: daemon-failure-agent
display_name: Daemon Failure Agent
state_root: state
workflow:
  kind: unsupported_adapter
  name: failing_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 5
memory:
  namespace: smoke/daemon-failure-agent
  write_policy: append_only
"#,
    )
    .expect("write agent spec");

    let spec_str = spec.to_str().expect("utf8 path");
    let out = run_csm(&[
        "daemon",
        "--spec",
        spec_str,
        "--max-restarts",
        "1",
        "--checkpoint-interval-secs",
        "1",
        "--no-sleep",
        "--json",
    ]);
    assert!(
        !out.status.success(),
        "expected daemon failure, stdout:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let daemon_status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/daemon_status.json")).expect("read daemon status"),
    )
    .expect("parse daemon status");
    assert_eq!(daemon_status["state"], "failed");
    assert_eq!(daemon_status["restart_count"], 1);
    assert_eq!(daemon_status["last_event"], "restart_budget_exhausted");
    assert!(root.join("state/continuity_checkpoint.json").exists());
    assert!(root.join("state/continuity_replay_manifest.json").exists());
    assert!(root.join("state/safe_fail_bundle.json").exists());
    assert!(root
        .join("state/safe_fail_artifacts/safe-fail-000001.json")
        .exists());

    let status: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/status.json")).expect("read status"),
    )
    .expect("parse status");
    assert_eq!(status["state"], "failed");
    assert_eq!(status["last_error"]["class"], "daemon_child_failed");

    let operator_events =
        fs::read_to_string(root.join("state/operator_events.jsonl")).expect("operator events");
    assert!(operator_events.contains("\"event\":\"restart_scheduled\""));
    assert!(operator_events.contains("\"event\":\"restart_attempted\""));
    assert!(operator_events.contains("\"event\":\"restart_budget_exhausted\""));
    assert!(operator_events.contains("\"event\":\"safe_fail_serialization\""));
    assert!(operator_events.contains("\"checkpoint_ref\":\"continuity_checkpoint.json\""));

    let safe_fail: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("state/safe_fail_bundle.json")).expect("safe fail bundle"),
    )
    .expect("parse safe fail bundle");
    assert_eq!(safe_fail["schema"], "adl.csm.safe_fail_bundle.v1");
    assert_eq!(safe_fail["runtime_owner"], "csm");
    assert_eq!(safe_fail["trigger"], "restart_budget_exhausted");
    assert_eq!(safe_fail["agent_outcome"]["state"], "recoverable");
    assert_eq!(
        safe_fail["recoverability"]["class"],
        "recoverable_checkpointed"
    );
    assert_eq!(
        safe_fail["monotonicity"]["does_not_rewrite_continuity_checkpoint"],
        true
    );
    assert_eq!(
        safe_fail["observability"]["otel_service_name"],
        "csm-runtime-daemon"
    );
    assert!(safe_fail["serialized_refs"]
        .as_array()
        .expect("serialized refs")
        .iter()
        .any(|artifact| artifact["role"] == "continuity_checkpoint"
            && artifact["status"] == "retained"));
    assert_eq!(
        safe_fail["serialized_state"]["status"]["value"]["last_error"]["class"],
        "daemon_child_failed"
    );
}
