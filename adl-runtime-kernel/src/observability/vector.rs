use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use adl_runtime_kernel::{RuntimeCloudWatchInitConfig, RuntimeObservabilityInitConfig};
use serde_json::{json, Value};

const VECTOR_DISK_BUFFER_MIN_BYTES: u64 = 256 * 1024 * 1024 + 32;

use super::{
    master_log::{absolute_path, vector_path},
    redaction::validate_otlp_endpoint,
};

#[derive(Clone, Debug)]
pub struct RuntimeVectorConfig {
    pub vector_binary: PathBuf,
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
    pub vector_startup_attempts: u32,
    pub vector_startup_backoff: Duration,
    pub vector_shutdown_limit: Duration,
    pub drain_timeout: Duration,
    pub filter_directive: String,
    pub vector_config_path: PathBuf,
    pub ingress_spool_path: PathBuf,
    pub master_log_path: PathBuf,
    pub audit_path: PathBuf,
    pub sequence_checkpoint_path: PathBuf,
    pub vector_data_dir: PathBuf,
    pub spool_max_bytes: u64,
    pub spool_retained_files: usize,
    pub cloudwatch: Option<RuntimeCloudWatchInitConfig>,
}

impl RuntimeVectorConfig {
    pub fn from_init_config(
        init: &RuntimeObservabilityInitConfig,
        observability_root: PathBuf,
        runtime_instance_id: String,
    ) -> Result<Self, String> {
        if !observability_root.is_absolute() {
            return Err("runtime_vector_observability_root_must_be_absolute".to_owned());
        }
        validate_otlp_endpoint(init.otlp_endpoint.as_deref())?;
        let state_path = |path: &Path| -> Result<PathBuf, String> {
            absolute_path(&observability_root.join(path))
                .map_err(|_| "runtime_vector_state_path_invalid".to_owned())
        };
        Ok(Self {
            vector_binary: init.vector_binary_path.clone(),
            runtime_instance_id,
            guardian_id: init.guardian_id.clone(),
            process_id: std::process::id(),
            revision: init.revision.clone(),
            service_name: init.service_name.clone(),
            lifecycle_suite: init.lifecycle_suite.clone(),
            lifecycle_run: init.lifecycle_run.clone(),
            lifecycle_cycle: init.lifecycle_cycle.clone(),
            otlp_endpoint: init.otlp_endpoint.clone(),
            otlp_timeout_millis: init.otlp_timeout_millis,
            vector_startup_attempts: init.vector_startup_attempts,
            vector_startup_backoff: Duration::from_millis(init.vector_startup_backoff_millis),
            vector_shutdown_limit: Duration::from_millis(init.vector_shutdown_limit_millis),
            drain_timeout: Duration::from_millis(init.drain_timeout_millis),
            filter_directive: init.trace_filter.clone(),
            vector_config_path: state_path(&init.vector_config_path)?,
            ingress_spool_path: state_path(&init.ingress_spool_path)?,
            master_log_path: state_path(&init.master_log_path)?,
            audit_path: state_path(&init.audit_path)?,
            sequence_checkpoint_path: state_path(&init.sequence_checkpoint_path)?,
            vector_data_dir: state_path(&init.vector_data_dir)?,
            spool_max_bytes: init.spool_max_bytes,
            spool_retained_files: init.spool_retained_files,
            cloudwatch: init.cloudwatch.clone(),
        })
    }
}

pub fn render_vector_config(config: &RuntimeVectorConfig) -> Value {
    let path = |path: &Path| vector_path(path);
    let buffer_events = config.spool_max_bytes.saturating_div(1024).max(1);
    let mut sinks = serde_json::Map::new();
    let mut transforms = serde_json::Map::new();
    sinks.insert(
        "runtime_v3_master_log".to_owned(),
        json!({
            "type": "file",
            "inputs": ["runtime_v3_redacted"],
            "path": path(&config.master_log_path),
            "encoding": {"codec": "json"},
            "acknowledgements": {"enabled": true},
            "buffer": {"type": "memory", "max_events": buffer_events, "when_full": "block"}
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
                ".otel.service_name = .service_name\n",
                ".otel.resource.revision = .revision\n",
                ".otel.resource.runtime_instance_id = .runtime_instance_id\n",
                ".otel.resource.guardian_id = .guardian_id"
            )
        }),
    );
    if let Some(uri) = config.otlp_endpoint.as_deref() {
        sinks.insert(
            "runtime_v3_otlp".to_owned(),
            json!({
                "type": "opentelemetry",
                "inputs": ["runtime_v3_redacted"],
                "protocol": {
                    "type": "http",
                    "uri": uri,
                    "encoding": {"codec": "json"}
                },
                "healthcheck": {"enabled": true},
                "acknowledgements": {"enabled": true},
                "buffer": {"type": "memory", "max_events": buffer_events, "when_full": "block"},
                "request": {"timeout_secs": config.otlp_timeout_millis.div_ceil(1_000).max(1)}
            }),
        );
    }
    if let Some(cloudwatch) = config.cloudwatch.as_ref() {
        transforms.insert(
            "runtime_v3_cloud_health".to_owned(),
            json!({
                "type": "remap",
                "inputs": ["runtime_v3_redacted"],
                "source": concat!(
                    "if .fields.schema != \"adl.runtime_v3.cloud_health.v1\" || .fields.event != \"runtime_health_heartbeat\" { abort }\n",
                    ". = {\n",
                    "  \"schema\": .fields.schema,\n",
                    "  \"event\": .fields.event,\n",
                    "  \"polis_id\": .fields.polis_id,\n",
                    "  \"runtime_instance_id\": .fields.runtime_instance_id,\n",
                    "  \"topology_generation\": .fields.topology_generation,\n",
                    "  \"continuity_generation\": .fields.continuity_generation,\n",
                    "  \"config_hash\": .fields.config_hash,\n",
                    "  \"ready\": .fields.ready,\n",
                    "  \"live\": .fields.live,\n",
                    "  \"ready_metric\": .fields.ready_metric,\n",
                    "  \"live_metric\": .fields.live_metric,\n",
                    "  \"degraded_components\": .fields.degraded_components,\n",
                    "  \"failed_components\": .fields.failed_components,\n",
                    "  \"restarting_components\": .fields.restarting_components,\n",
                    "  \"saturated_queues\": .fields.saturated_queues\n",
                    "}"
                )
            }),
        );
        sinks.insert(
            "runtime_v3_cloudwatch".to_owned(),
            json!({
                "type": "aws_cloudwatch_logs",
                "inputs": ["runtime_v3_cloud_health"],
                "region": cloudwatch.region,
                "group_name": cloudwatch.log_group,
                "stream_name": cloudwatch.log_stream,
                "create_missing_group": false,
                "create_missing_stream": true,
                "encoding": {"codec": "json"},
                "healthcheck": {"enabled": true},
                "acknowledgements": {"enabled": true},
                "buffer": {
                    "type": "disk",
                    "max_size": config.spool_max_bytes.max(VECTOR_DISK_BUFFER_MIN_BYTES),
                    "when_full": "block"
                }
            }),
        );
    }
    json!({
        "data_dir": path(&config.vector_data_dir),
        "healthchecks": {"enabled": true},
        "sources": {
            "runtime_v3_ingress": {
                "type": "file",
                "include": [path(&config.ingress_spool_path)],
                "read_from": "beginning",
                "fingerprint": {"strategy": "checksum", "lines": 1, "ignored_header_bytes": 0},
                "ignore_not_found": false
            }
        },
        "transforms": transforms,
        "sinks": sinks
    })
}
