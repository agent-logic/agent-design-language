#[allow(dead_code)]
#[path = "../src/observability.rs"]
mod runtime_observability;

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use adl_runtime_kernel::{RUNTIME_MASTER_LOG_AUDIT_SCHEMA, RUNTIME_MASTER_LOG_RECORD_SCHEMA};
use runtime_observability::{
    audit_master_log_file, render_vector_config, RuntimeVectorConfig, RuntimeVectorPipeline,
};
use serde_json::{json, Value};

#[test]
fn master_log_auditor_accepts_wp12_clean_report_shape() {
    let root = test_root("clean-audit");
    let log_path = root.join("master.log.jsonl");
    write_records(
        &log_path,
        &[
            record(0, "info", "kernel_starting", "boot", "wp-12"),
            record(
                1,
                "info",
                "observability_drain_complete",
                "shutdown_drained",
                "wp-12",
            ),
        ],
    );

    let report = audit_master_log_file(&log_path, "macos", "wp-12", "rev-a").unwrap();

    assert_eq!(report.schema, RUNTIME_MASTER_LOG_AUDIT_SCHEMA);
    assert_eq!(report.status, "pass");
    assert_eq!(report.platform, "macos");
    assert_eq!(report.suite, "wp-12");
    assert_eq!(report.revision, "rev-a");
    assert_eq!(report.record_count, 2);
    assert_eq!(report.master_log_sha256.len(), 64);
    assert_eq!(report.malformed_records, 0);
    assert_eq!(report.missing_required_fields, 0);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.error_events, 0);
    assert_eq!(report.degraded_events, 0);
    assert_eq!(report.unexplained_restarts, 0);
    assert_eq!(report.incomplete_drains, 0);
}

#[test]
fn master_log_auditor_rejects_bad_sequences_errors_and_missing_drain() {
    let root = test_root("bad-audit");
    let log_path = root.join("master.log.jsonl");
    let mut lines = String::new();
    lines.push_str("{not-json}\n");
    lines.push_str(
        &serde_json::to_string(&record(0, "info", "kernel_starting", "boot", "wp-12")).unwrap(),
    );
    lines.push('\n');
    lines.push_str(
        &serde_json::to_string(&record(
            2,
            "error",
            "pipeline_failure",
            "exporter_unavailable",
            "wp-12",
        ))
        .unwrap(),
    );
    lines.push('\n');
    fs::write(&log_path, lines).unwrap();

    let report = audit_master_log_file(&log_path, "linux", "wp-12", "rev-b").unwrap();

    assert_eq!(report.status, "fail");
    assert_eq!(report.malformed_records, 1);
    assert_eq!(report.sequence_gaps, 1);
    assert_eq!(report.error_events, 1);
    assert_eq!(report.degraded_events, 1);
    assert_eq!(report.incomplete_drains, 1);
}

#[test]
fn vector_config_declares_durable_master_otlp_and_bounded_buffers() {
    let root = test_root("config-shape");
    let config = vector_config(root.clone(), Some("http://127.0.0.1:4318".to_owned()));
    let rendered = render_vector_config(&root, &config);

    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["path"],
        json!(root.join("durable/master.log.jsonl").to_string_lossy())
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_master_log"]["buffer"]["type"],
        "disk"
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["type"],
        "opentelemetry"
    );
    assert_eq!(
        rendered["sinks"]["runtime_v3_otlp"]["buffer"]["when_full"],
        "block"
    );
    let rendered_text = serde_json::to_string(&rendered).unwrap();
    assert!(rendered_text.contains("adl_runtime_v3"));
    assert!(!rendered_text.contains("aws_secret_access_key"));
    assert!(!rendered_text.contains("aws_access_key_id"));
}

#[test]
fn pinned_vector_validates_generated_master_log_config() {
    let root = test_root("vector-validate");
    let config = vector_config(root.clone(), None);
    let rendered = render_vector_config(&root, &config);
    let config_path = root.join("runtime-v3-vector.json");
    fs::write(&config_path, serde_json::to_vec_pretty(&rendered).unwrap()).unwrap();

    let output = Command::new(&config.vector_binary)
        .arg("validate")
        .arg("--no-environment")
        .arg("--deny-warnings")
        .arg("--config-json")
        .arg(&config_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn runtime_vector_pipeline_writes_auditable_master_log() {
    let root = test_root("runtime-vector-e2e");
    let mut pipeline = RuntimeVectorPipeline::start(vector_config(root, None)).unwrap();
    tracing::info!(
        target: "adl_runtime_kernel",
        component = "test",
        operation = "test_observation",
        reason = "ok",
        "observability test event"
    );
    pipeline.shutdown();

    let report = pipeline.audit_master_log("macos", "wp-12").unwrap();

    assert_eq!(report.schema, RUNTIME_MASTER_LOG_AUDIT_SCHEMA);
    assert_eq!(report.status, "pass");
    assert!(report.record_count >= 2);
    assert_eq!(report.malformed_records, 0);
    assert_eq!(report.sequence_gaps, 0);
    assert_eq!(report.error_events, 0);
    assert_eq!(report.degraded_events, 0);
    assert_eq!(report.unexplained_restarts, 0);
    assert_eq!(report.incomplete_drains, 0);
}

#[test]
fn windows_vector_installer_contract_is_pinned_and_verified() {
    let script =
        fs::read_to_string(repo_root().join("adl/tools/install_vector_component.ps1")).unwrap();

    assert!(script.contains("vector-0.56.0-x86_64-pc-windows-msvc.zip"));
    assert!(script.contains("67611f6b18c3b267ab26402c0dddc59e59bbccd762c7c0ea5f654f4ec4e6bf42"));
    assert!(script.contains("Get-FileHash -Algorithm SHA256"));
    assert!(script.contains("Expand-Archive"));
    assert!(script.contains(".adl/bin/vector.exe"));
    assert!(script.contains("adl.component.provenance.v1"));
}

#[test]
fn shell_installer_routes_native_windows_to_powershell_installer() {
    let script =
        fs::read_to_string(repo_root().join("adl/tools/install_vector_component.sh")).unwrap();

    assert!(script.contains("install_vector_component.ps1"));
    assert!(script.contains("MINGW*|MSYS*|CYGWIN*"));
}

fn vector_config(root: PathBuf, otlp_endpoint: Option<String>) -> RuntimeVectorConfig {
    let vector_binary = std::env::var("ADL_RUNTIME_V3_VECTOR_BINARY")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join(".adl/bin/vector"));
    RuntimeVectorConfig {
        vector_binary,
        state_root: root.clone(),
        runtime_instance_id: "runtime-test-instance".to_owned(),
        guardian_id: "guardian-test".to_owned(),
        process_id: std::process::id(),
        revision: "test-revision".to_owned(),
        service_name: "adl-runtime-v3".to_owned(),
        lifecycle_suite: "wp-12".to_owned(),
        lifecycle_run: "run-1".to_owned(),
        lifecycle_cycle: "cycle-1".to_owned(),
        otlp_endpoint,
        otlp_timeout_millis: 5_000,
        drain_timeout: std::time::Duration::from_millis(1_000),
    }
}

fn record(sequence: u64, severity: &str, operation: &str, reason: &str, suite: &str) -> Value {
    json!({
        "schema": RUNTIME_MASTER_LOG_RECORD_SCHEMA,
        "timestamp": format!("unix:{}", 1_700_000_000_000_u64 + sequence),
        "timestamp_unix_millis": 1_700_000_000_000_u64 + sequence,
        "severity": severity,
        "level": severity,
        "target": "adl_runtime_kernel",
        "runtime_instance_id": "runtime-test-instance",
        "guardian_id": "guardian-test",
        "process_id": std::process::id(),
        "process_kind": "runtime_kernel",
        "service_name": "adl-runtime-v3",
        "lifecycle_suite": suite,
        "lifecycle_run": "run-1",
        "lifecycle_cycle": "cycle-1",
        "component": "observability",
        "operation": operation,
        "reason": reason,
        "error_chain": "",
        "revision": "test-revision",
        "sequence": sequence,
        "runtime_event_count": 1,
        "trace_id": "trace-test",
        "span_id": sequence,
        "parent_span_id": null,
        "fields": {}
    })
}

fn write_records(path: &Path, records: &[Value]) {
    let mut lines = String::new();
    for record in records {
        lines.push_str(&serde_json::to_string(record).unwrap());
        lines.push('\n');
    }
    fs::write(path, lines).unwrap();
}

fn test_root(name: &str) -> PathBuf {
    let root = std::env::current_dir()
        .unwrap()
        .join("target/observability-tests")
        .join(format!("{}-{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
