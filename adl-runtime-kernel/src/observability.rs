use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread::sleep,
    time::{Duration, Instant},
};

use adl_runtime_kernel::{
    ObservabilityDegradation, ObservabilityHealth, ObservabilityPipelineSnapshot,
    RUNTIME_MASTER_LOG_RECORD_SCHEMA, RUNTIME_OBSERVABILITY_PIPELINE_SCHEMA,
};
use serde_json::{json, Value};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::{
    field::{Field, Visit},
    Event, Metadata, Subscriber,
};
use tracing_subscriber::{
    filter::Targets,
    layer::{Context, Layer},
    prelude::*,
    Registry,
};

#[path = "observability/master_log.rs"]
mod master_log;
#[path = "observability/redaction.rs"]
mod redaction;
#[path = "observability/vector.rs"]
mod vector;
pub use master_log::{audit_master_log_file, MasterLogAuditReport};
use master_log::{
    create_parent_dir, current_platform, master_log_contains_sequence, master_log_highest_sequence,
    recover_next_sequence, recover_run_start_sequence, rotate_file_if_large, write_json_atomic,
    write_sequence_checkpoint, SequenceCheckpoint,
};
use redaction::redact_field;
pub use vector::{render_vector_config, RuntimeVectorConfig};

#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
use windows_sys::Win32::System::{
    Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
    Threading::CREATE_NEW_PROCESS_GROUP,
};

pub const VECTOR_COMPONENT_VERSION: &str = "0.56.0";
const MASTER_LOG_LIVENESS_TIMEOUT: Duration = Duration::from_secs(30);
const VECTOR_RETRY_BACKOFF_LIMIT: Duration = Duration::from_secs(30);

#[derive(Debug)]
struct MasterLogLiveness {
    last_durable_sequence: Option<u64>,
    stalled_since: Option<Instant>,
}

#[derive(Debug, Default)]
struct RecoveryRetry {
    failures: u32,
    retry_at: Option<Instant>,
}

impl RecoveryRetry {
    fn ready(&self, now: Instant) -> bool {
        self.retry_at.is_none_or(|retry_at| now >= retry_at)
    }

    fn record_failure(&mut self, now: Instant, base: Duration) -> Duration {
        self.failures = self.failures.saturating_add(1);
        let delay = vector_retry_backoff(base, self.failures, VECTOR_RETRY_BACKOFF_LIMIT);
        self.retry_at = Some(now + delay);
        delay
    }

    fn reset(&mut self) {
        self.failures = 0;
        self.retry_at = None;
    }
}

impl MasterLogLiveness {
    fn new(last_durable_sequence: Option<u64>) -> Self {
        Self {
            last_durable_sequence,
            stalled_since: None,
        }
    }

    fn observe(
        &mut self,
        latest_sequence: Option<u64>,
        durable_sequence: Option<u64>,
        now: Instant,
        timeout: Duration,
    ) -> bool {
        let Some(latest_sequence) = latest_sequence else {
            self.stalled_since = None;
            return false;
        };

        if durable_sequence.is_some_and(|sequence| sequence >= latest_sequence) {
            self.last_durable_sequence = durable_sequence;
            self.stalled_since = None;
            return false;
        }

        if durable_sequence > self.last_durable_sequence {
            self.last_durable_sequence = durable_sequence;
            self.stalled_since = Some(now);
            return false;
        }

        let stalled_since = self.stalled_since.get_or_insert(now);
        now.saturating_duration_since(*stalled_since) >= timeout
    }
}

pub struct RuntimeVectorPipeline {
    child: Option<Child>,
    ingress: Arc<Mutex<File>>,
    sequence: Arc<AtomicU64>,
    accepting: Arc<AtomicBool>,
    config: RuntimeVectorConfig,
    master_log_path: PathBuf,
    audit_path: PathBuf,
    sequence_path: PathBuf,
    run_start_sequence: u64,
    last_failure: Option<String>,
    master_log_liveness: MasterLogLiveness,
    recovery_retry: RecoveryRetry,
    drain_complete: bool,
}

impl RuntimeVectorPipeline {
    pub fn start(config: RuntimeVectorConfig) -> Result<Self, String> {
        Self::start_inner(config, true)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn start_without_subscriber_for_test(config: RuntimeVectorConfig) -> Result<Self, String> {
        Self::start_inner(config, false)
    }

    fn start_inner(config: RuntimeVectorConfig, install_subscriber: bool) -> Result<Self, String> {
        create_parent_dir(&config.vector_config_path).map_err(|_| "state_root_unavailable")?;
        create_parent_dir(&config.ingress_spool_path).map_err(|_| "state_root_unavailable")?;
        create_parent_dir(&config.master_log_path).map_err(|_| "state_root_unavailable")?;
        create_parent_dir(&config.audit_path).map_err(|_| "state_root_unavailable")?;
        create_parent_dir(&config.sequence_checkpoint_path)
            .map_err(|_| "state_root_unavailable")?;
        fs::create_dir_all(&config.vector_data_dir).map_err(|_| "state_root_unavailable")?;
        rotate_file_if_large(
            &config.ingress_spool_path,
            config.spool_max_bytes,
            config.spool_retained_files,
        )
        .map_err(|_| "runtime_vector_spool_rotation_failed")?;
        truncate_incomplete_jsonl_tail(&config.ingress_spool_path)
            .map_err(|_| "runtime_vector_spool_tail_recovery_failed")?;
        let rendered = render_vector_config(&config);
        write_json_atomic(&config.vector_config_path, &rendered)
            .map_err(|_| "vector_config_write_failed")?;
        validate_vector_config(&config.vector_binary, &config.vector_config_path)?;
        let next_sequence = recover_next_sequence(
            &config.sequence_checkpoint_path,
            &config.master_log_path,
            &config.ingress_spool_path,
        )
        .map_err(|error| format!("runtime_vector_sequence_recovery_failed:{error}"))?;
        let run_start_sequence = recover_run_start_sequence(
            &config.master_log_path,
            &config.lifecycle_run,
            &config.lifecycle_cycle,
            next_sequence,
        )
        .map_err(|error| format!("runtime_vector_run_window_recovery_failed:{error}"))?;
        let ingress = Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&config.ingress_spool_path)
                .map_err(|_| "runtime_vector_ingress_unavailable")?,
        ));
        let sequence = Arc::new(AtomicU64::new(next_sequence));
        let accepting = Arc::new(AtomicBool::new(true));
        if install_subscriber {
            let filter = config
                .filter_directive
                .parse::<Targets>()
                .map_err(|_| "runtime_vector_trace_filter_invalid")?;
            tracing::subscriber::set_global_default(
                Registry::default().with(
                    RuntimeTracingLayer::new(
                        ingress.clone(),
                        sequence.clone(),
                        accepting.clone(),
                        config.clone(),
                    )
                    .with_filter(filter),
                ),
            )
            .map_err(|_| "runtime_tracing_subscriber_already_installed")?;
        }
        let child = start_vector_with_retries(&config, &ingress, &sequence, &accepting)?;
        let last_durable_sequence = master_log_highest_sequence(&config.master_log_path);
        Ok(Self {
            child: Some(child),
            ingress,
            sequence,
            accepting,
            config: config.clone(),
            master_log_path: config.master_log_path.clone(),
            audit_path: config.audit_path.clone(),
            sequence_path: config.sequence_checkpoint_path.clone(),
            run_start_sequence,
            last_failure: None,
            master_log_liveness: MasterLogLiveness::new(last_durable_sequence),
            recovery_retry: RecoveryRetry::default(),
            drain_complete: false,
        })
    }

    pub fn snapshot(&self) -> ObservabilityPipelineSnapshot {
        ObservabilityPipelineSnapshot {
            schema: RUNTIME_OBSERVABILITY_PIPELINE_SCHEMA.to_owned(),
            health: if self.last_failure.is_none() {
                ObservabilityHealth::Ready
            } else {
                ObservabilityHealth::Degraded {
                    reason: ObservabilityDegradation::ExporterUnavailable,
                }
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

    pub fn poll_health(&mut self) -> Result<(), String> {
        if self.drain_complete {
            return Ok(());
        }
        let now = Instant::now();
        if self.child.is_none() && !self.recovery_retry.ready(now) {
            return Ok(());
        }
        let child_failure = match self.child.as_mut() {
            None => Some(
                self.last_failure
                    .clone()
                    .unwrap_or_else(|| "vector_child_missing".to_owned()),
            ),
            Some(child) => match child.try_wait() {
                Ok(Some(status)) => Some(format!("vector_child_exited:{status}")),
                Ok(None) => None,
                Err(_) => Some("vector_child_health_unknown".to_owned()),
            },
        };
        if let Some(failure) = child_failure {
            return self.restart_vector_after_failure(failure);
        }
        let latest_sequence = self.sequence.load(Ordering::SeqCst).saturating_sub(1);
        let latest_sequence =
            (latest_sequence >= self.run_start_sequence).then_some(latest_sequence);
        let durable_sequence = master_log_highest_sequence(&self.master_log_path);
        if self.master_log_liveness.observe(
            latest_sequence,
            durable_sequence,
            now,
            MASTER_LOG_LIVENESS_TIMEOUT,
        ) {
            let failure = "master_log_liveness_stalled".to_owned();
            return self.restart_vector_after_failure(failure);
        }
        Ok(())
    }

    fn restart_vector_after_failure(&mut self, failure: String) -> Result<(), String> {
        self.last_failure = Some(failure.clone());
        if let Some(mut child) = self.child.take() {
            if child.try_wait().ok().flatten().is_none() {
                let _ = child.kill();
            }
            let _ = child.wait();
        }
        let restart_sequence = self.sequence.load(Ordering::SeqCst);
        write_observability_record(
            &self.ingress,
            &self.sequence,
            &self.accepting,
            &self.config,
            StructuredEvent {
                // A detected child exit is a recoverable pipeline event: the
                // Runtime retains service authority, records the restart, and
                // immediately attempts bounded recovery. Reserve ERROR for a
                // restart that cannot be persisted or completed.
                severity: "WARN",
                target: "adl_runtime_kernel",
                component: "observability",
                operation: "vector_pipeline_restarting",
                reason: &failure,
                error_chain: &failure,
                trace_id: format!("runtime-trace-{}", self.config.runtime_instance_id),
                fields: BTreeMap::from([("service_continues".to_owned(), Value::Bool(true))]),
            },
        );
        if self.sequence.load(Ordering::SeqCst) != restart_sequence.saturating_add(1) {
            let recovery_failure = format!("{failure}:restart_record_not_persisted");
            return Err(self.defer_recovery_failure(recovery_failure));
        }
        let child = match start_vector_with_retries(
            &self.config,
            &self.ingress,
            &self.sequence,
            &self.accepting,
        ) {
            Ok(child) => child,
            Err(error) => {
                let recovery_failure = format!("{failure}:vector_restart_failed:{error}");
                return Err(self.defer_recovery_failure(recovery_failure));
            }
        };
        self.child = Some(child);
        self.master_log_liveness =
            MasterLogLiveness::new(master_log_highest_sequence(&self.master_log_path));
        self.recovery_retry.reset();
        self.last_failure = None;
        write_observability_record(
            &self.ingress,
            &self.sequence,
            &self.accepting,
            &self.config,
            StructuredEvent {
                severity: "INFO",
                target: "adl_runtime_kernel",
                component: "observability",
                operation: "vector_pipeline_recovered",
                reason: "child_restarted",
                error_chain: "",
                trace_id: format!("runtime-trace-{}", self.config.runtime_instance_id),
                fields: BTreeMap::from([("service_continued".to_owned(), Value::Bool(true))]),
            },
        );
        Ok(())
    }

    fn defer_recovery_failure(&mut self, failure: String) -> String {
        self.recovery_retry
            .record_failure(Instant::now(), self.config.vector_startup_backoff);
        self.last_failure = Some(failure.clone());
        failure
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
            self.run_start_sequence,
            self.sequence.load(Ordering::SeqCst).saturating_sub(1),
        )
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn master_log_path_for_test(&self) -> &Path {
        &self.master_log_path
    }

    pub async fn shutdown(&mut self) -> Result<(), String> {
        self.shutdown_blocking()
    }

    fn shutdown_blocking(&mut self) -> Result<(), String> {
        if self.drain_complete {
            return match self.last_failure.clone() {
                Some(failure) => Err(failure),
                None => Ok(()),
            };
        }
        if let Ok(mut ingress) = self.ingress.lock() {
            self.accepting.store(false, Ordering::SeqCst);
            let _ = write_observability_record_locked(
                &mut ingress,
                &self.sequence,
                &self.config,
                StructuredEvent {
                    severity: "INFO",
                    target: "adl_runtime_kernel",
                    component: "observability",
                    operation: "observability_drain_complete",
                    reason: "shutdown_drained",
                    error_chain: "",
                    trace_id: format!("runtime-trace-{}", self.config.runtime_instance_id),
                    fields: BTreeMap::from([("drain".to_owned(), Value::Bool(true))]),
                },
            );
            let _ = ingress.flush();
            let _ = ingress.sync_data();
        }
        let drain_sequence = self.sequence.load(Ordering::SeqCst).saturating_sub(1);
        let master_drained = wait_for_master_sequence(
            &self.master_log_path,
            drain_sequence,
            self.config.drain_timeout,
        );
        if master_drained {
            let _ = write_sequence_checkpoint(
                &self.sequence_path,
                SequenceCheckpoint::new(self.sequence.load(Ordering::SeqCst), drain_sequence),
            );
        }
        let vector_stopped_cleanly = self.terminate_vector();
        self.drain_complete = vector_stopped_cleanly && master_drained;
        if !self.drain_complete {
            self.last_failure = Some(
                match (master_drained, vector_stopped_cleanly) {
                    (false, false) => "master_log_and_vector_shutdown_incomplete",
                    (false, true) => "master_log_drain_incomplete",
                    (true, false) => "vector_shutdown_incomplete",
                    (true, true) => unreachable!("drain_complete covers this state"),
                }
                .to_owned(),
            );
        }
        if let Err(error) = self
            .audit_master_log(&current_platform(), &self.config.lifecycle_suite)
            .and_then(|report| write_json_atomic(&self.audit_path, &report))
        {
            let failure = format!("master_log_audit_persistence_failed:{error}");
            self.last_failure = Some(failure.clone());
            return Err(failure);
        }
        if self.drain_complete {
            Ok(())
        } else {
            Err(self
                .last_failure
                .clone()
                .unwrap_or_else(|| "runtime_observability_shutdown_incomplete".to_owned()))
        }
    }

    fn terminate_vector(&mut self) -> bool {
        let Some(mut child) = self.child.take() else {
            return false;
        };
        terminate_vector_child(&mut child, self.config.drain_timeout)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn terminate_vector_child_for_test(mut child: Child, drain_timeout: Duration) -> bool {
        terminate_vector_child(&mut child, drain_timeout)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn stop_vector_for_test(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn stop_accepting_for_test(&self) {
        self.accepting.store(false, Ordering::SeqCst);
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn configure_master_log_watchdog_for_test(
        &mut self,
        master_log_path: PathBuf,
        latest_sequence: u64,
        last_durable_sequence: u64,
        stalled_for: Duration,
    ) {
        self.master_log_path = master_log_path;
        self.sequence
            .store(latest_sequence.saturating_add(1), Ordering::SeqCst);
        self.master_log_liveness = MasterLogLiveness {
            last_durable_sequence: Some(last_durable_sequence),
            stalled_since: Some(Instant::now() - stalled_for),
        };
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn restore_master_log_path_for_test(&mut self, master_log_path: PathBuf) {
        self.master_log_path = master_log_path;
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn terminate_vector_for_test(&mut self) -> bool {
        self.terminate_vector()
    }
}

fn truncate_incomplete_jsonl_tail(path: &Path) -> io::Result<()> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if bytes.is_empty() || bytes.last() == Some(&b'\n') {
        return Ok(());
    }
    let retained_len = bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map(|index| index + 1)
        .unwrap_or(0);
    OpenOptions::new()
        .write(true)
        .open(path)?
        .set_len(retained_len as u64)
}

fn terminate_vector_child(child: &mut Child, drain_timeout: Duration) -> bool {
    if child.try_wait().is_ok_and(|status| status.is_some()) {
        return true;
    }
    if !signal_vector_shutdown(child) {
        if child.try_wait().is_ok_and(|status| status.is_some()) {
            return true;
        }
        let _ = child.kill();
        let _ = child.wait();
        return false;
    }
    let deadline = Instant::now() + drain_timeout;
    while Instant::now() < deadline {
        match child.try_wait() {
            Ok(Some(_status)) => return true,
            Ok(None) => sleep(Duration::from_millis(50)),
            Err(_) => break,
        }
    }
    let _ = child.kill();
    let _ = child.wait();
    false
}

#[cfg(unix)]
fn signal_vector_shutdown(child: &Child) -> bool {
    unsafe { libc::kill(child.id() as i32, libc::SIGTERM) == 0 }
}

#[cfg(windows)]
fn signal_vector_shutdown(child: &Child) -> bool {
    unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, child.id()) != 0 }
}

#[cfg(not(any(unix, windows)))]
fn signal_vector_shutdown(_child: &Child) -> bool {
    false
}

impl Drop for RuntimeVectorPipeline {
    fn drop(&mut self) {
        self.accepting.store(false, Ordering::SeqCst);
        if !self.drain_complete {
            if let Some(child) = self.child.as_mut() {
                let _ = child.kill();
            }
        }
    }
}

#[derive(Clone)]
struct RuntimeTracingLayer {
    ingress: Arc<Mutex<File>>,
    sequence: Arc<AtomicU64>,
    accepting: Arc<AtomicBool>,
    config: RuntimeVectorConfig,
}

impl RuntimeTracingLayer {
    fn new(
        ingress: Arc<Mutex<File>>,
        sequence: Arc<AtomicU64>,
        accepting: Arc<AtomicBool>,
        config: RuntimeVectorConfig,
    ) -> Self {
        Self {
            ingress,
            sequence,
            accepting,
            config,
        }
    }

    fn write_record(&self, metadata: &Metadata<'_>, mut fields: BTreeMap<String, Value>) {
        let trace_id = string_field(&fields, "correlation_id")
            .unwrap_or_else(|| format!("runtime-trace-{}", self.config.runtime_instance_id));
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
        write_observability_record(
            &self.ingress,
            &self.sequence,
            &self.accepting,
            &self.config,
            StructuredEvent {
                severity: metadata.level().as_str(),
                target: metadata.target(),
                component: &component,
                operation: &operation,
                reason: &reason,
                error_chain: &error_chain,
                trace_id,
                fields,
            },
        );
    }
}

impl<S> Layer<S> for RuntimeTracingLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = JsonFieldVisitor::default();
        event.record(&mut visitor);
        self.write_record(event.metadata(), visitor.fields);
    }
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

fn validate_vector_config(binary: &Path, config_path: &Path) -> Result<(), String> {
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
    Err("vector_config_validation_failed".to_owned())
}

#[cfg(test)]
mod master_log_liveness_tests {
    use super::MasterLogLiveness;
    use std::time::{Duration, Instant};

    #[test]
    fn forward_progress_while_behind_does_not_trigger_restart() {
        let start = Instant::now();
        let timeout = Duration::from_secs(30);
        let mut liveness = MasterLogLiveness::new(Some(10));

        assert!(!liveness.observe(Some(20), Some(10), start, timeout));
        assert!(!liveness.observe(Some(20), Some(11), start + Duration::from_secs(29), timeout,));
        assert!(!liveness.observe(Some(20), Some(12), start + Duration::from_secs(58), timeout,));
    }

    #[test]
    fn sustained_pending_backlog_without_progress_triggers_restart() {
        let start = Instant::now();
        let timeout = Duration::from_secs(30);
        let mut liveness = MasterLogLiveness::new(Some(10));

        assert!(!liveness.observe(Some(20), Some(10), start, timeout));
        assert!(liveness.observe(Some(20), Some(10), start + timeout, timeout,));
    }

    #[test]
    fn caught_up_master_log_clears_a_previous_stall_window() {
        let start = Instant::now();
        let timeout = Duration::from_secs(30);
        let mut liveness = MasterLogLiveness::new(Some(10));

        assert!(!liveness.observe(Some(20), Some(10), start, timeout));
        assert!(!liveness.observe(Some(20), Some(20), start + Duration::from_secs(29), timeout,));
        assert!(!liveness.observe(Some(21), Some(20), start + Duration::from_secs(31), timeout,));
    }
}

struct StructuredEvent<'a> {
    severity: &'a str,
    target: &'a str,
    component: &'a str,
    operation: &'a str,
    reason: &'a str,
    error_chain: &'a str,
    trace_id: String,
    fields: BTreeMap<String, Value>,
}

fn write_observability_record(
    ingress: &Arc<Mutex<File>>,
    sequence: &Arc<AtomicU64>,
    accepting: &Arc<AtomicBool>,
    config: &RuntimeVectorConfig,
    event: StructuredEvent<'_>,
) {
    if !accepting.load(Ordering::SeqCst) {
        return;
    }
    if let Ok(mut file) = ingress.lock() {
        if !accepting.load(Ordering::SeqCst) {
            return;
        }
        let _ = write_observability_record_locked(&mut file, sequence, config, event);
    }
}

fn write_observability_record_locked(
    file: &mut File,
    sequence: &AtomicU64,
    config: &RuntimeVectorConfig,
    event: StructuredEvent<'_>,
) -> std::io::Result<()> {
    let current_sequence = sequence.load(Ordering::SeqCst);
    let now = OffsetDateTime::now_utc();
    let timestamp = now
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned());
    let timestamp_unix_millis = now.unix_timestamp_nanos().max(0) as u64 / 1_000_000;
    let record = json!({
        "schema": RUNTIME_MASTER_LOG_RECORD_SCHEMA,
        "timestamp": timestamp,
        "timestamp_unix_millis": timestamp_unix_millis,
        "severity": event.severity,
        "level": event.severity,
        "target": event.target,
        "runtime_instance_id": config.runtime_instance_id,
        "guardian_id": config.guardian_id,
        "process_id": config.process_id,
        "process_kind": "runtime_kernel",
        "service_name": config.service_name,
        "lifecycle_suite": config.lifecycle_suite,
        "lifecycle_run": config.lifecycle_run,
        "lifecycle_cycle": config.lifecycle_cycle,
        "component": event.component,
        "operation": event.operation,
        "reason": event.reason,
        "error_chain": event.error_chain,
        "revision": config.revision,
        "sequence": current_sequence,
        "runtime_event_count": 1,
        "trace_id": event.trace_id,
        "span_id": current_sequence,
        "parent_span_id": null,
        "fields": event.fields
    });
    serde_json::to_writer(&mut *file, &record)?;
    file.write_all(b"\n")?;
    file.flush()?;
    sequence.store(current_sequence.saturating_add(1), Ordering::SeqCst);
    Ok(())
}

fn start_vector_with_retries(
    config: &RuntimeVectorConfig,
    ingress: &Arc<Mutex<File>>,
    sequence: &Arc<AtomicU64>,
    accepting: &Arc<AtomicBool>,
) -> Result<Child, String> {
    let mut last_failure = "vector_startup_not_attempted".to_owned();
    for attempt in 1..=config.vector_startup_attempts {
        let mut child = match spawn_vector_child(config) {
            Ok(child) => child,
            Err(failure) => {
                last_failure = failure;
                if attempt < config.vector_startup_attempts {
                    record_vector_start_retry(
                        config,
                        ingress,
                        sequence,
                        accepting,
                        attempt,
                        &last_failure,
                    );
                    sleep(vector_retry_backoff(
                        config.vector_startup_backoff,
                        attempt,
                        VECTOR_RETRY_BACKOFF_LIMIT,
                    ));
                    continue;
                }
                break;
            }
        };
        let child_pid = child.id();
        let probe_sequence = sequence.load(Ordering::SeqCst);
        write_observability_record(
            ingress,
            sequence,
            accepting,
            config,
            StructuredEvent {
                severity: "INFO",
                target: "adl_runtime_kernel",
                component: "observability",
                operation: "vector_pipeline_started",
                reason: "vector_child_supervised",
                error_chain: "",
                trace_id: format!("runtime-trace-{}", config.runtime_instance_id),
                fields: BTreeMap::from([
                    (
                        "startup_attempt".to_owned(),
                        Value::Number(serde_json::Number::from(attempt)),
                    ),
                    (
                        "vector_pid".to_owned(),
                        Value::Number(serde_json::Number::from(child_pid)),
                    ),
                    (
                        "vector_version".to_owned(),
                        Value::String(VECTOR_COMPONENT_VERSION.to_owned()),
                    ),
                ]),
            },
        );
        if sequence.load(Ordering::SeqCst) != probe_sequence.saturating_add(1) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("vector_startup_record_not_persisted".to_owned());
        }

        last_failure = match child.try_wait() {
            Ok(Some(status)) => format!("vector_startup_failed:{status}"),
            Err(_) => "vector_child_health_unknown".to_owned(),
            Ok(None)
                if wait_for_master_sequence(
                    &config.master_log_path,
                    probe_sequence,
                    config.drain_timeout,
                ) =>
            {
                return Ok(child);
            }
            Ok(None) => "vector_startup_readiness_not_observed".to_owned(),
        };
        if child.try_wait().ok().flatten().is_none() {
            let _ = child.kill();
        }
        let _ = child.wait();
        if attempt < config.vector_startup_attempts {
            record_vector_start_retry(config, ingress, sequence, accepting, attempt, &last_failure);
            sleep(vector_retry_backoff(
                config.vector_startup_backoff,
                attempt,
                VECTOR_RETRY_BACKOFF_LIMIT,
            ));
        }
    }
    Err(format!(
        "{last_failure}:attempts_exhausted:{}",
        config.vector_startup_attempts
    ))
}

fn vector_retry_backoff(base: Duration, failed_attempt: u32, limit: Duration) -> Duration {
    let exponent = failed_attempt.saturating_sub(1).min(31);
    let multiplier = 1_u32.checked_shl(exponent).unwrap_or(u32::MAX);
    base.saturating_mul(multiplier).min(limit.max(base))
}

#[cfg(test)]
mod vector_retry_backoff_tests {
    use super::{truncate_incomplete_jsonl_tail, vector_retry_backoff, RecoveryRetry};
    use std::fs;
    use std::time::Duration;

    #[test]
    fn interrupted_jsonl_tail_is_removed_without_touching_complete_records() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("runtime-v3.current.jsonl");
        fs::write(&path, b"{\"sequence\":1}\n{\"sequence\":2").unwrap();

        truncate_incomplete_jsonl_tail(&path).unwrap();

        assert_eq!(fs::read(&path).unwrap(), b"{\"sequence\":1}\n");
    }

    #[test]
    fn backoff_is_exponential_and_bounded() {
        let base = Duration::from_millis(100);
        let limit = Duration::from_secs(1);

        assert_eq!(
            vector_retry_backoff(base, 1, limit),
            Duration::from_millis(100)
        );
        assert_eq!(
            vector_retry_backoff(base, 2, limit),
            Duration::from_millis(200)
        );
        assert_eq!(
            vector_retry_backoff(base, 3, limit),
            Duration::from_millis(400)
        );
        assert_eq!(vector_retry_backoff(base, 8, limit), limit);
    }

    #[test]
    fn backoff_limit_never_shortens_the_configured_base() {
        let base = Duration::from_secs(5);
        let lower_limit = Duration::from_secs(1);

        assert_eq!(vector_retry_backoff(base, 1, lower_limit), base);
        assert_eq!(vector_retry_backoff(base, 8, lower_limit), base);
    }

    #[test]
    fn recovery_backoff_persists_across_health_polls_until_reset() {
        let start = std::time::Instant::now();
        let base = Duration::from_millis(100);
        let mut retry = RecoveryRetry::default();

        assert!(retry.ready(start));
        assert_eq!(
            retry.record_failure(start, base),
            Duration::from_millis(100)
        );
        assert!(!retry.ready(start + Duration::from_millis(99)));
        assert!(retry.ready(start + Duration::from_millis(100)));

        assert_eq!(
            retry.record_failure(start + Duration::from_millis(100), base),
            Duration::from_millis(200)
        );
        assert!(!retry.ready(start + Duration::from_millis(299)));
        assert!(retry.ready(start + Duration::from_millis(300)));

        retry.reset();
        assert!(retry.ready(start));
        assert_eq!(retry.failures, 0);
    }
}

fn record_vector_start_retry(
    config: &RuntimeVectorConfig,
    ingress: &Arc<Mutex<File>>,
    sequence: &Arc<AtomicU64>,
    accepting: &Arc<AtomicBool>,
    attempt: u32,
    failure: &str,
) {
    write_observability_record(
        ingress,
        sequence,
        accepting,
        config,
        StructuredEvent {
            severity: "WARN",
            target: "adl_runtime_kernel",
            component: "observability",
            operation: "vector_pipeline_start_retry",
            reason: failure,
            error_chain: failure,
            trace_id: format!("runtime-trace-{}", config.runtime_instance_id),
            fields: BTreeMap::from([
                (
                    "completed_attempt".to_owned(),
                    Value::Number(serde_json::Number::from(attempt)),
                ),
                ("service_continues".to_owned(), Value::Bool(true)),
            ]),
        },
    );
}

fn spawn_vector_child(config: &RuntimeVectorConfig) -> Result<Child, String> {
    let mut command = Command::new(&config.vector_binary);
    command
        .arg("--config-json")
        .arg(&config.vector_config_path)
        .arg("--require-healthy")
        .arg("true")
        .arg("--log-format")
        .arg("json")
        .arg("--graceful-shutdown-limit-secs")
        .arg(config.vector_shutdown_limit.as_secs().max(1).to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    command
        .spawn()
        .map_err(|_| "vector_binary_missing_or_not_executable".to_owned())
}

fn wait_for_master_sequence(path: &Path, sequence: u64, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if master_log_contains_sequence(path, sequence) {
            return true;
        }
        sleep(Duration::from_millis(25));
    }
    false
}

fn string_field(fields: &BTreeMap<String, Value>, key: &str) -> Option<String> {
    fields
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}
