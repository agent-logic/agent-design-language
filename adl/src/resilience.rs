mod bulkhead;
mod circuit_breaker;
mod fallback;
mod fault;
mod policy;
mod rate_limit;
mod retry;
mod runtime;
mod schema;
mod timeout;

pub use bulkhead::{
    bulkhead_initial_state, execute_bulkhead_policy, BulkheadExecution, BulkheadExecutionTraceV1,
    BulkheadFinalStatusV1, BulkheadPolicyV1, BulkheadStateV1,
};
pub use circuit_breaker::{
    circuit_breaker_initial_state, execute_circuit_breaker_policy, CircuitBreakerExecution,
    CircuitBreakerExecutionTraceV1, CircuitBreakerFinalStatusV1, CircuitBreakerPolicyV1,
    CircuitBreakerStateKindV1, CircuitBreakerStateV1,
};
pub use fallback::{
    execute_fallback_policy, FallbackExecution, FallbackExecutionFinalStatusV1,
    FallbackExecutionTraceV1, FallbackOutcomeKindV1, FallbackPolicyV1,
};
pub(crate) use fault::sanitize_resilience_summary;
pub use fault::{
    CheckpointKindV1, CheckpointRecordV1, CitizenHealthRecordV1, CitizenHealthStateV1,
    RecoveryArtifactV1, RecoveryDispositionV1, ResilienceFaultClassV1,
    ResilienceFaultClassificationV1, ResilienceFaultDispositionV1, ResilienceSurfaceV1,
    ResilienceTelemetryEventV1, TelemetryEventKindV1,
};
pub use policy::{ResiliencePolicyV1, ResilienceSubstrateManifestV1};
pub use rate_limit::{
    execute_rate_limit_policy, rate_limit_initial_state, RateLimitExecution,
    RateLimitExecutionTraceV1, RateLimitFinalStatusV1, RateLimitPolicyV1, RateLimitStateV1,
};
pub use retry::{
    execute_retry_policy, RetryAttemptRecordV1, RetryExecution, RetryExecutionFinalStatusV1,
    RetryExecutionTraceV1, RetryPolicyV1, RetryTerminalReasonV1,
};
pub use runtime::{
    remote_exec_health_payload, RuntimeCorrelationFieldsV1, RuntimeHealthStateV1,
    RuntimeHealthStatusV1, RuntimeResilienceDispositionV1, RuntimeResilienceTraceV1,
};
pub use schema::{
    resilience_schema_smoke, RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1,
    RESILIENCE_BULKHEAD_STATE_SCHEMA_V1, RESILIENCE_CHECKPOINT_SCHEMA_V1,
    RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1,
    RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1, RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1,
    RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1, RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1,
    RESILIENCE_POLICY_SCHEMA_V1, RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1,
    RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1, RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1,
    RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1, RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1,
    RESILIENCE_SUBSTRATE_SCHEMA_V1, RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1,
    RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1, RUNTIME_CORRELATION_FIELDS_SCHEMA_V1,
    RUNTIME_HEALTH_STATUS_SCHEMA_V1, RUNTIME_RESILIENCE_TRACE_SCHEMA_V1,
};
pub use timeout::{
    execute_timeout_policy, TimeoutBreachKindV1, TimeoutExecution, TimeoutExecutionFinalStatusV1,
    TimeoutExecutionTraceV1, TimeoutObservation, TimeoutPolicyV1,
};

#[cfg(test)]
mod tests;
