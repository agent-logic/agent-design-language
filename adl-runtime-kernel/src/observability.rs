use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::{Duration, Instant, SystemTime},
};

use adl_runtime_kernel::{
    ObservabilityHealth, ObservabilityPipelineSnapshot, RUNTIME_MASTER_LOG_AUDIT_SCHEMA,
    RUNTIME_MASTER_LOG_RECORD_SCHEMA, RUNTIME_OBSERVABILITY_PIPELINE_SCHEMA,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{
    field::{Field, Visit},
    span, Event, Id, Metadata, Subscriber,
};

pub const VECTOR_COMPONENT_VERSION: &str = "0.56.0";
pub const VECTOR_COMPONENT_BINARY_REF: &str = ".adl/bin/vector";
#[cfg(windows)]
pub const VECTOR_COMPONENT_WINDOWS_BINARY_REF: &str = ".adl/bin/vector.exe";

const STARTUP_OBSERVATION: Duration = Duration::from_millis(350);
const DEFAULT_DRAIN_TIMEOUT: Duration = Duration::from_millis(5_000);
const DEFAULT_OTLP_TIMEOUT_MILLIS: u64 = 5_000;

#[derive(Clone, Debug)]
pub struct RuntimeVectorConfig {
    pub vector_binary: PathBuf,
    pub state_root: PathBuf,
    pub runtime_instance_id: String,
    pub guardian_id: String,
    pub process_id: u32,
    pub revision: String,
    pub service_name: String,
    pub lifecycle_suite: String,
    pub lifecycle_run: String,
    pub lifecycle_cycle: String,
    pub otlp_endpoint: Option<String>,
    pub otlp_timeout_millis: u64,
    pub drain_timeout: Duration,
}

impl RuntimeVectorConfig {
    pub fn from_runtime_environment(
        state_root: PathBuf,
        runtime_instance_id: String,
        revision: String,
    ) -> Result<Self, String> {
        if !state_root.is_absolute() {
            return Err("runtime_vector_state_root_must_be_absolute".to_owned());
        }
        let vector_binary = std::env::var("ADL_RUNTIME_V3_VECTOR_BINARY")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .map(PathBuf::from)
            .or_else(resolve_vector_binary)
            .unwrap_or_else(|| PathBuf::from(VECTOR_COMPONENT_BINARY_REF));
        let otlp_endpoint = first_env([
            "ADL_RUNTIME_V3_OTLP_ENDPOINT",
            "ADL_CSM_OTLP_ENDPOINT",
            "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT",
            "OTEL_EXPORTER_OTLP_ENDPOINT",
        ]);
        validate_otlp_endpoint(otlp_endpoint.as_deref())?;
        let otlp_timeout_millis = first_env([
            "ADL_RUNTIME_V3_OTLP_TIMEOUT_MS",
            "ADL_CSM_OTLP_TIMEOUT_MS",
            "OTEL_EXPORTER_OTLP_TIMEOUT_MS",
            "OTEL_EXPORTER_OTLP_TIMEOUT",
        ])
        .and_then(|value| parse_duration_millis(&value))
        .unwrap_or(DEFAULT_OTLP_TIMEOUT_MILLIS);
        let drain_timeout = first_env(["ADL_RUNTIME_V3_VECTOR_DRAIN_TIMEOUT_MS"])
            .and_then(|value| parse_duration_millis(&value))
            .map(Duration::from_millis)
            .unwrap_or(DEFAULT_DRAIN_TIMEOUT);
        Ok(Self {
            vector_binary,
            state_root,
            runtime_instance_id,
            guardian_id: std::env::var("ADL_RUNTIME_GUARDIAN_ID")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| format!("process:{}", std::process::id())),
            process_id: std::process::id(),
            revision,
            service_name: std::env::var("ADL_RUNTIME_SERVICE_NAME")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "adl-runtime-v3".to_owned()),
            lifecycle_suite: std::env::var("ADL_RUNTIME_LIFECYCLE_SUITE")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "runtime".to_owned()),
            lifecycle_run: std::env::var("ADL_RUNTIME_LIFECYCLE_RUN")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "runtime-run".to_owned()),
            lifecycle_cycle: std::env::var("ADL_RUNTIME_LIFECYCLE_CYCLE")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "runtime-cycle".to_owned()),
            otlp_endpoint,
            otlp_timeout_millis,
            drain_timeout,
        })
    }
}

pub struct RuntimeVectorPipeline {
    child: Option<Child>,
    ingress: Arc<Mutex<File>>,
    sequence: Arc<AtomicU64>,
    config: RuntimeVectorConfig,
    master_log_path: PathBuf,
    audit_path: PathBuf,
    last_failure: Option<String>,
    drain_complete: bool,
}

impl RuntimeVectorPipeline {
    pub fn start(config: RuntimeVectorConfig) -> Result<Self, String> {
        verify_vector_binary(&config.vector_binary)?;
        let root = config.state_root.join("observability");
        for relative in [
            "config",
            "ingress",
            "durable",
            "audit",
            "vector-data",
            "logs",
        ] {
            fs::create_dir_all(root.join(relative)).map_err(|_| "state_root_unavailable")?;
        }
        let root = root.canonicalize().map_err(|_| "state_root_unavailable")?;
        let ingress_path = root.join("ingress/runtime-v3.jsonl");
        let master_log_path = root.join("durable/master.log.jsonl");
        let audit_path = root.join("audit/master-log-audit.json");
        let config_path = root.join("config/runtime-v3-vector.json");
        let rendered = render_vector_config(&root, &config);
        write_json_atomic(&config_path, &rendered).map_err(|_| "vector_config_write_failed")?;
        validate_vector_config(&config.vector_binary, &config_path, &root)?;
        let next_sequence = next_sequence_from_log(&ingress_path).unwrap_or(0);
        let ingress = Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&ingress_path)
                .map_err(|_| "runtime_vector_ingress_unavailable")?,
        ));
        let sequence = Arc::new(AtomicU64::new(next_sequence));
        tracing::subscriber::set_global_default(RuntimeTracingSubscriber::new(
            ingress.clone(),
            sequence.clone(),
            config.clone(),
        ))
        .map_err(|_| "runtime_tracing_subscriber_already_installed")?;
        let stdout = append_file(&root.join("logs/vector.stdout.log"))
            .map_err(|_| "runtime_vector_log_unavailable")?;
        let stderr = append_file(&root.join("logs/vector.stderr.log"))
            .map_err(|_| "runtime_vector_log_unavailable")?;
        let mut child = Command::new(&config.vector_binary)
            .arg("--config-json")
            .arg(&config_path)
            .arg("--require-healthy")
            .arg("true")
            .arg("--log-format")
            .arg("json")
            .arg("--graceful-shutdown-limit-secs")
            .arg(config.drain_timeout.as_secs().max(1).to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .map_err(|_| "vector_binary_missing_or_not_executable")?;
        let child_pid = child.id();
        tracing::info!(
            target: "adl_runtime_kernel",
            component = "observability",
            operation = "vector_pipeline_started",
            reason = "vector_child_supervised",
            vector_pid = child_pid,
            vector_version = VECTOR_COMPONENT_VERSION,
            "Runtime v3 Vector observability pipeline started"
        );
        sleep(STARTUP_OBSERVATION);
        if let Some(status) = child
            .try_wait()
            .map_err(|_| "vector_child_health_unknown")?
        {
            return Err(format!("vector_startup_failed:{status}"));
        }
        Ok(Self {
            child: Some(child),
            ingress,
            sequence,
            config,
            master_log_path,
            audit_path,
            last_failure: None,
            drain_complete: false,
        })
    }

    pub fn snapshot(&self) -> ObservabilityPipelineSnapshot {
        ObservabilityPipelineSnapshot {
            schema: RUNTIME_OBSERVABILITY_PIPELINE_SCHEMA.to_owned(),
            health: if self.last_failure.is_some() {
                ObservabilityHealth::Degraded {
                    reason: adl_runtime_kernel::ObservabilityDegradation::ExporterUnavailable,
                }
            } else {
                ObservabilityHealth::Ready
            },
            runtime_instance_id: self.config.runtime_instance_id.clone(),
            vector_pid: self.child.as_ref().map(Child::id),
            vector_version: VECTOR_COMPONENT_VERSION.to_owned(),
            master_log_ref: self.master_log_path.to_string_lossy().into_owned(),
            log_audit_ref: self.audit_path.to_string_lossy().into_owned(),
            otlp_endpoint: self.config.otlp_endpoint.clone(),
            otlp_timeout_millis: self.config.otlp_timeout_millis,
            sequence_next: self.sequence.load(Ordering::SeqCst),
            drain_complete: self.drain_complete,
            last_failure: self.last_failure.clone(),
        }
    }

    pub fn audit_master_log(
        &self,
        platform: &str,
        suite: &str,
    ) -> std::io::Result<MasterLogAuditReport> {
        audit_master_log_file(
            &self.master_log_path,
            platform,
            suite,
            &self.config.revision,
        )
    }

    pub fn shutdown(&mut self) {
        if self.drain_complete {
            return;
        }
        let drain_sequence = self.sequence.load(Ordering::SeqCst);
        tracing::info!(
            target: "adl_runtime_kernel",
            component = "observability",
            operation = "observability_drain_complete",
            reason = "shutdown_drained",
            drain = true,
            "Runtime v3 observability drain complete"
        );
        if let Ok(mut ingress) = self.ingress.lock() {
            let _ = ingress.flush();
            let _ = ingress.sync_data();
        }
        self.drain_complete = self.wait_for_master_sequence(drain_sequence);
        if !self.drain_complete {
            self.last_failure = Some("master_log_drain_incomplete".to_owned());
        }
        let _ = self
            .audit_master_log(&current_platform(), &self.config.lifecycle_suite)
            .and_then(|report| write_json_atomic(&self.audit_path, &report));
        self.terminate_vector();
    }

    fn wait_for_master_sequence(&self, sequence: u64) -> bool {
        let deadline = Instant::now() + self.config.drain_timeout;
        while Instant::now() < deadline {
            if master_log_contains_sequence(&self.master_log_path, sequence) {
                return true;
            }
            sleep(Duration::from_millis(50));
        }
        false
    }

    fn terminate_vector(&mut self) {
        let Some(mut child) = self.child.take() else {
            return;
        };
        #[cfg(unix)]
        unsafe {
            let _ = libc::kill(child.id() as i32, libc::SIGTERM);
        }
        #[cfg(not(unix))]
        {
            let _ = child.kill();
        }
        let deadline = Instant::now() + self.config.drain_timeout;
        while Instant::now() < deadline {
            match child.try_wait() {
                Ok(Some(_)) => return,
                Ok(None) => sleep(Duration::from_millis(50)),
                Err(_) => break,
            }
        }
        let _ = child.kill();
        let _ = child.wait();
    }
}

impl Drop for RuntimeVectorPipeline {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MasterLogAuditReport {
    pub schema: String,
    pub status: String,
    pub platform: String,
    pub suite: String,
    pub revision: String,
    pub master_log_sha256: String,
    pub record_count: u64,
    pub malformed_records: u64,
    pub missing_required_fields: u64,
    pub sequence_gaps: u64,
    pub error_events: u64,
    pub degraded_events: u64,
    pub unexplained_restarts: u64,
    pub incomplete_drains: u64,
}

pub fn audit_master_log_file(
    path: &Path,
    platform: &str,
    suite: &str,
    revision: &str,
) -> std::io::Result<MasterLogAuditReport> {
    let bytes = fs::read(path)?;
    let master_log_sha256 = sha256_hex(&bytes);
    let reader = BufReader::new(bytes.as_slice());
    let mut report = MasterLogAuditReport {
        schema: RUNTIME_MASTER_LOG_AUDIT_SCHEMA.to_owned(),
        status: "pass".to_owned(),
        platform: platform.to_owned(),
        suite: suite.to_owned(),
        revision: revision.to_owned(),
        master_log_sha256,
        record_count: 0,
        malformed_records: 0,
        missing_required_fields: 0,
        sequence_gaps: 0,
        error_events: 0,
        degraded_events: 0,
        unexplained_restarts: 0,
        incomplete_drains: 1,
    };
    let mut expected_sequence = None;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(record) = serde_json::from_str::<Value>(&line) else {
            report.malformed_records = report.malformed_records.saturating_add(1);
            continue;
        };
        report.record_count = report.record_count.saturating_add(1);
        if !has_required_master_log_fields(&record) {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        if record["schema"] != RUNTIME_MASTER_LOG_RECORD_SCHEMA {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        if record["lifecycle_suite"].as_str() != Some(suite) {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        match record["sequence"].as_u64() {
            Some(sequence) => {
                if let Some(expected) = expected_sequence {
                    if sequence != expected {
                        report.sequence_gaps = report.sequence_gaps.saturating_add(1);
                    }
                }
                expected_sequence = Some(sequence.saturating_add(1));
            }
            None => {
                report.missing_required_fields = report.missing_required_fields.saturating_add(1)
            }
        }
        let severity = record["severity"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let operation = record["operation"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let reason = record["reason"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let error_chain = record["error_chain"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if matches!(severity.as_str(), "error" | "fatal")
            || ["panic", "fatal", "error"].iter().any(|needle| {
                operation.contains(needle)
                    || reason.contains(needle)
                    || error_chain.contains(needle)
            })
        {
            report.error_events = report.error_events.saturating_add(1);
        }
        if ["degraded", "unavailable", "pipeline_failure"]
            .iter()
            .any(|needle| {
                operation.contains(needle)
                    || reason.contains(needle)
                    || error_chain.contains(needle)
            })
        {
            report.degraded_events = report.degraded_events.saturating_add(1);
        }
        if operation.contains("restart") && !reason.contains("explained") {
            report.unexplained_restarts = report.unexplained_restarts.saturating_add(1);
        }
        if operation == "observability_drain_complete" && reason == "shutdown_drained" {
            report.incomplete_drains = 0;
        }
    }
    if report.record_count == 0
        || report.malformed_records != 0
        || report.missing_required_fields != 0
        || report.sequence_gaps != 0
        || report.error_events != 0
        || report.degraded_events != 0
        || report.unexplained_restarts != 0
        || report.incomplete_drains != 0
    {
        report.status = "fail".to_owned();
    }
    Ok(report)
}

pub fn render_vector_config(root: &Path, config: &RuntimeVectorConfig) -> Value {
    let path = |relative: &str| root.join(relative).to_string_lossy().into_owned();
    let mut sinks = serde_json::Map::new();
    let mut transforms = serde_json::Map::new();
    sinks.insert(
        "runtime_v3_master_log".to_owned(),
        json!({
            "type": "file",
            "inputs": ["runtime_v3_redacted"],
            "path": path("durable/master.log.jsonl"),
            "encoding": {"codec": "json"},
            "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"}
        }),
    );
    transforms.insert(
        "runtime_v3_parse".to_owned(),
        json!({
            "type": "remap",
            "inputs": ["runtime_v3_ingress"],
            "source": ". = parse_json!(.message)"
        }),
    );
    transforms.insert(
        "runtime_v3_redacted".to_owned(),
        json!({
            "type": "remap",
            "inputs": ["runtime_v3_parse"],
            "source": concat!(
                "if exists(.fields.secret) { .fields.secret = \"<redacted>\" }\n",
                "if exists(.fields.token) { .fields.token = \"<redacted>\" }\n",
                "if exists(.fields.authorization) { .fields.authorization = \"<redacted>\" }\n",
                "if exists(.fields.password) { .fields.password = \"<redacted>\" }\n",
                "if exists(.fields.api_key) { .fields.api_key = \"<redacted>\" }\n",
                ".otel.service_name = \"", "adl-runtime-v3", "\"\n",
                ".otel.resource.revision = .revision\n",
                ".otel.resource.runtime_instance_id = .runtime_instance_id\n",
                ".otel.resource.guardian_id = .guardian_id"
            )
        }),
    );
    if let Some(uri) = config.otlp_endpoint.as_deref() {
        transforms.insert(
            "runtime_v3_metrics".to_owned(),
            json!({
                "type": "log_to_metric",
                "inputs": ["runtime_v3_redacted"],
                "metrics": [{
                    "type": "counter",
                    "field": "runtime_event_count",
                    "name": "runtime_event_count",
                    "namespace": "adl_runtime_v3",
                    "tags": {
                        "service": "{{service_name}}",
                        "runtime_instance_id": "{{runtime_instance_id}}",
                        "component": "{{component}}",
                        "severity": "{{severity}}"
                    }
                }]
            }),
        );
        sinks.insert(
            "runtime_v3_otlp".to_owned(),
            json!({
                "type": "opentelemetry",
                "inputs": ["runtime_v3_redacted", "runtime_v3_metrics"],
                "protocol": {
                    "type": "http",
                    "uri": uri,
                    "encoding": {"codec": "otlp"}
                },
                "healthcheck": {"enabled": true},
                "acknowledgements": {"enabled": true},
                "buffer": {"type": "disk", "max_size": 268435488, "when_full": "block"},
                "request": {"timeout_secs": config.otlp_timeout_millis.div_ceil(1_000).max(1)}
            }),
        );
    }
    json!({
        "data_dir": path("vector-data"),
        "healthchecks": {"enabled": true, "require_healthy": true},
        "sources": {
            "runtime_v3_ingress": {
                "type": "file",
                "include": [path("ingress/runtime-v3.jsonl")],
                "read_from": "beginning",
            "fingerprint": {"strategy": "device_and_inode"}
            }
        },
        "transforms": transforms,
        "sinks": sinks
    })
}

#[derive(Clone)]
struct RuntimeTracingSubscriber {
    ingress: Arc<Mutex<File>>,
    sequence: Arc<AtomicU64>,
    config: RuntimeVectorConfig,
}

impl RuntimeTracingSubscriber {
    fn new(
        ingress: Arc<Mutex<File>>,
        sequence: Arc<AtomicU64>,
        config: RuntimeVectorConfig,
    ) -> Self {
        Self {
            ingress,
            sequence,
            config,
        }
    }

    fn write_record(
        &self,
        metadata: &Metadata<'_>,
        mut fields: BTreeMap<String, Value>,
        span_id: Option<u64>,
        parent_span_id: Option<u64>,
    ) {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let operation = string_field(&fields, "operation")
            .or_else(|| string_field(&fields, "event"))
            .unwrap_or_else(|| metadata.name().to_owned());
        let component =
            string_field(&fields, "component").unwrap_or_else(|| metadata.target().to_owned());
        let reason = string_field(&fields, "reason").unwrap_or_else(|| operation.clone());
        let error_chain = string_field(&fields, "error")
            .or_else(|| string_field(&fields, "error_chain"))
            .unwrap_or_default();
        fields.insert(
            "runtime_event_count".to_owned(),
            Value::Number(serde_json::Number::from(1)),
        );
        let record = json!({
            "schema": RUNTIME_MASTER_LOG_RECORD_SCHEMA,
            "timestamp": format!("unix:{}", epoch_millis_now()),
            "timestamp_unix_millis": epoch_millis_now(),
            "severity": metadata.level().as_str(),
            "level": metadata.level().as_str(),
            "target": metadata.target(),
            "runtime_instance_id": self.config.runtime_instance_id,
            "guardian_id": self.config.guardian_id,
            "process_id": self.config.process_id,
            "process_kind": "runtime_kernel",
            "service_name": self.config.service_name,
            "lifecycle_suite": self.config.lifecycle_suite,
            "lifecycle_run": self.config.lifecycle_run,
            "lifecycle_cycle": self.config.lifecycle_cycle,
            "component": component,
            "operation": operation,
            "reason": reason,
            "error_chain": error_chain,
            "revision": self.config.revision,
            "sequence": sequence,
            "runtime_event_count": 1,
            "trace_id": string_field(&fields, "correlation_id").unwrap_or_else(|| format!("runtime-trace-{}", self.config.runtime_instance_id)),
            "span_id": span_id.unwrap_or(sequence),
            "parent_span_id": parent_span_id,
            "fields": fields
        });
        if let Ok(mut file) = self.ingress.lock() {
            let _ = serde_json::to_writer(&mut *file, &record);
            let _ = file.write_all(b"\n");
            let _ = file.flush();
        }
    }
}

impl Subscriber for RuntimeTracingSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attrs: &span::Attributes<'_>) -> Id {
        let span_id = self.sequence.load(Ordering::SeqCst);
        let mut visitor = JsonFieldVisitor::default();
        attrs.record(&mut visitor);
        self.write_record(
            attrs.metadata(),
            visitor.fields,
            Some(span_id),
            attrs.parent().map(Id::into_u64),
        );
        Id::from_u64(span_id)
    }

    fn record(&self, _span: &Id, _values: &span::Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut visitor = JsonFieldVisitor::default();
        event.record(&mut visitor);
        self.write_record(event.metadata(), visitor.fields, None, None);
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

#[derive(Default)]
struct JsonFieldVisitor {
    fields: BTreeMap<String, Value>,
}

impl Visit for JsonFieldVisitor {
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .insert(field.name().to_owned(), Value::Number(value.into()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .insert(field.name().to_owned(), Value::Number(value.into()));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .insert(field.name().to_owned(), Value::Bool(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.fields.insert(
            field.name().to_owned(),
            Value::String(redact_field(field.name(), value)),
        );
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.fields.insert(
            field.name().to_owned(),
            Value::String(redact_field(field.name(), &format!("{value:?}"))),
        );
    }
}

fn has_required_master_log_fields(record: &Value) -> bool {
    [
        "timestamp",
        "severity",
        "level",
        "target",
        "runtime_instance_id",
        "guardian_id",
        "process_id",
        "lifecycle_suite",
        "lifecycle_run",
        "lifecycle_cycle",
        "component",
        "operation",
        "reason",
        "error_chain",
        "revision",
        "sequence",
    ]
    .iter()
    .all(|field| !record[*field].is_null())
}

fn validate_vector_config(binary: &Path, config_path: &Path, root: &Path) -> Result<(), String> {
    let _ = fs::create_dir_all(root.join("logs"));
    let output = Command::new(binary)
        .arg("validate")
        .arg("--no-environment")
        .arg("--deny-warnings")
        .arg("--config-json")
        .arg(config_path)
        .output()
        .map_err(|_| "vector_binary_missing_or_not_executable")?;
    if output.status.success() {
        return Ok(());
    }
    let diagnostic = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let _ = fs::write(
        root.join("logs/vector.validate.stderr.log"),
        redact_diagnostic(&diagnostic),
    );
    Err("vector_config_validation_failed".to_owned())
}

fn verify_vector_binary(binary: &Path) -> Result<(), String> {
    let output = Command::new(binary)
        .arg("--version")
        .output()
        .map_err(|_| "vector_binary_missing_or_not_executable")?;
    let expected = format!("vector {VECTOR_COMPONENT_VERSION} ");
    if !output.status.success() || !String::from_utf8_lossy(&output.stdout).starts_with(&expected) {
        return Err("vector_binary_version_mismatch".to_owned());
    }
    Ok(())
}

fn resolve_vector_binary() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(parent) = executable.parent() {
            #[cfg(windows)]
            candidates.push(parent.join("vector.exe"));
            candidates.push(parent.join("vector"));
        }
    }
    if let Ok(current_dir) = std::env::current_dir() {
        for ancestor in current_dir.ancestors() {
            #[cfg(windows)]
            candidates.push(ancestor.join(VECTOR_COMPONENT_WINDOWS_BINARY_REF));
            candidates.push(ancestor.join(VECTOR_COMPONENT_BINARY_REF));
        }
    }
    candidates.into_iter().find(|path| path.is_file())
}

fn append_file(path: &Path) -> std::io::Result<File> {
    OpenOptions::new().create(true).append(true).open(path)
}

fn write_json_atomic(path: &Path, value: &impl Serialize) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension("tmp");
    {
        let mut file = File::create(&temp)?;
        serde_json::to_writer_pretty(&mut file, value)?;
        file.write_all(b"\n")?;
        file.sync_data()?;
    }
    fs::rename(temp, path)
}

fn next_sequence_from_log(path: &Path) -> std::io::Result<u64> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error),
    };
    let mut next = 0_u64;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let Ok(record) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        if let Some(sequence) = record["sequence"].as_u64() {
            next = next.max(sequence.saturating_add(1));
        }
    }
    Ok(next)
}

fn master_log_contains_sequence(path: &Path, sequence: u64) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .any(|line| {
            serde_json::from_str::<Value>(&line)
                .ok()
                .and_then(|record| record["sequence"].as_u64())
                == Some(sequence)
        })
}

fn string_field(fields: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    fields
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn first_env<const N: usize>(keys: [&str; N]) -> Option<String> {
    keys.into_iter()
        .find_map(|key| std::env::var(key).ok())
        .filter(|value| !value.trim().is_empty())
}

fn parse_duration_millis(value: &str) -> Option<u64> {
    let trimmed = value.trim();
    if let Some(seconds) = trimmed.strip_suffix('s') {
        return seconds
            .parse::<u64>()
            .ok()
            .map(|seconds| seconds.saturating_mul(1_000));
    }
    trimmed.parse::<u64>().ok()
}

fn validate_otlp_endpoint(endpoint: Option<&str>) -> Result<(), String> {
    let Some(endpoint) = endpoint else {
        return Ok(());
    };
    if endpoint.starts_with("https://")
        || endpoint.starts_with("http://127.0.0.1:")
        || endpoint.starts_with("http://localhost:")
        || endpoint.starts_with("http://[::1]:")
    {
        return Ok(());
    }
    Err("otlp_endpoint_requires_https_or_loopback".to_owned())
}

fn redact_field(key: &str, value: &str) -> String {
    if sensitive_key(key) || sensitive_text(value) {
        "<redacted>".to_owned()
    } else {
        value.to_owned()
    }
}

fn sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    [
        "secret",
        "token",
        "authorization",
        "password",
        "api_key",
        "access_key",
        "private_key",
        "credential",
    ]
    .iter()
    .any(|sensitive| normalized.contains(sensitive))
}

fn sensitive_text(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    value.contains("-----BEGIN PRIVATE KEY-----")
        || [
            "authorization:",
            "api_key=",
            "password=",
            "token=",
            "secret=",
        ]
        .iter()
        .any(|needle| lowered.contains(needle))
}

fn redact_diagnostic(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            let lowered = line.to_ascii_lowercase();
            if ["authorization", "secret", "token", "password", "api_key"]
                .iter()
                .any(|needle| lowered.contains(needle))
            {
                "<redacted-vector-diagnostic>"
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn epoch_millis_now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn current_platform() -> String {
    std::env::consts::OS.to_owned()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = sha256(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h = [
        0x6a09e667_u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in padded.chunks_exact(64) {
        let mut w = [0_u32; 64];
        for (index, word) in w.iter_mut().take(16).enumerate() {
            let start = index * 4;
            *word = u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut output = [0_u8; 32];
    for (index, word) in h.iter().enumerate() {
        output[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
