use schemars::schema_for;
use serde_json::Value;

use super::ResilienceSubstrateManifestV1;

pub const RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1: &str =
    "adl.resilience.fault_classification.v1";
pub const RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1: &str = "adl.resilience.citizen_health.v1";
pub const RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1: &str = "adl.resilience.recovery_artifact.v1";
pub const RESILIENCE_CHECKPOINT_SCHEMA_V1: &str = "adl.resilience.checkpoint.v1";
pub const RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1: &str = "adl.resilience.telemetry_event.v1";
pub const RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1: &str = "adl.resilience.retry_attempt.v1";
pub const RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.retry_execution_trace.v1";
pub const RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.timeout_execution_trace.v1";
pub const RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.circuit_breaker_execution_trace.v1";
pub const RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1: &str =
    "adl.resilience.circuit_breaker_state.v1";
pub const RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.rate_limit_execution_trace.v1";
pub const RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1: &str = "adl.resilience.rate_limit_state.v1";
pub const RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.bulkhead_execution_trace.v1";
pub const RESILIENCE_BULKHEAD_STATE_SCHEMA_V1: &str = "adl.resilience.bulkhead_state.v1";
pub const RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1: &str =
    "adl.resilience.fallback_execution_trace.v1";
pub const RESILIENCE_POLICY_SCHEMA_V1: &str = "adl.resilience.policy.v1";
pub const RESILIENCE_SUBSTRATE_SCHEMA_V1: &str = "adl.resilience.substrate_manifest.v1";
pub const RUNTIME_RESILIENCE_TRACE_SCHEMA_V1: &str = "adl.runtime.resilience_trace.v1";
pub const RUNTIME_CORRELATION_FIELDS_SCHEMA_V1: &str = "adl.runtime.correlation_fields.v1";
pub const RUNTIME_HEALTH_STATUS_SCHEMA_V1: &str = "adl.runtime.health_status.v1";

pub fn resilience_schema_smoke() -> Value {
    serde_json::to_value(schema_for!(ResilienceSubstrateManifestV1))
        .expect("resilience substrate schema should serialize")
}
