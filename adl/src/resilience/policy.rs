use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResiliencePolicyV1 {
    pub schema_version: String,
    pub policy_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeoutPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_breaker: Option<CircuitBreakerPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<RateLimitPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulkhead: Option<BulkheadPolicyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<FallbackPolicyV1>,
    pub checkpoint_required: bool,
    pub telemetry_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceSubstrateManifestV1 {
    pub schema_version: String,
    pub manifest_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_surfaces: Vec<ResilienceSurfaceV1>,
    pub fault_schema_ref: String,
    pub citizen_health_schema_ref: String,
    pub recovery_artifact_schema_ref: String,
    pub checkpoint_schema_ref: String,
    pub telemetry_schema_ref: String,
    pub policy: ResiliencePolicyV1,
}

impl ResiliencePolicyV1 {
    pub fn provider_attempt_policy(
        policy_id: impl Into<String>,
        max_attempts: u32,
        timeout_ms: u64,
    ) -> Self {
        Self {
            schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
            policy_id: policy_id.into(),
            retry: Some(RetryPolicyV1 {
                max_attempts,
                backoff_ms: None,
                jitter_ms: None,
                max_elapsed_ms: None,
                retryable_fault_classes: vec![
                    ResilienceFaultClassV1::ProviderRateLimited,
                    ResilienceFaultClassV1::ProviderTimeout,
                    ResilienceFaultClassV1::ProviderTransientHttp,
                    ResilienceFaultClassV1::LocalRuntimeUnavailable,
                    ResilienceFaultClassV1::LocalRuntimeBusy,
                    ResilienceFaultClassV1::LocalRuntimeHung,
                    ResilienceFaultClassV1::Unknown,
                ],
            }),
            timeout: Some(TimeoutPolicyV1 {
                timeout_ms,
                hard_deadline_ms: None,
            }),
            circuit_breaker: None,
            rate_limit: None,
            bulkhead: None,
            fallback: None,
            checkpoint_required: false,
            telemetry_required: true,
        }
    }
}

impl ResilienceSubstrateManifestV1 {
    pub fn phase1_foundation() -> Self {
        Self {
            schema_version: RESILIENCE_SUBSTRATE_SCHEMA_V1.to_string(),
            manifest_id: "phase1_resilience_substrate_foundation".to_string(),
            supported_surfaces: vec![
                ResilienceSurfaceV1::Provider,
                ResilienceSurfaceV1::Tool,
                ResilienceSurfaceV1::Workflow,
                ResilienceSurfaceV1::CitizenRuntime,
            ],
            fault_schema_ref: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            citizen_health_schema_ref: RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1.to_string(),
            recovery_artifact_schema_ref: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
            checkpoint_schema_ref: RESILIENCE_CHECKPOINT_SCHEMA_V1.to_string(),
            telemetry_schema_ref: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
            policy: ResiliencePolicyV1::provider_attempt_policy(
                "provider_attempt_default",
                3,
                30_000,
            ),
        }
    }
}
