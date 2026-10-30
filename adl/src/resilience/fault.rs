use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceSurfaceV1 {
    Provider,
    Tool,
    Workflow,
    CitizenRuntime,
    Runtime,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceFaultClassV1 {
    ProviderAuthMissing,
    ProviderAuthError,
    ProviderRateLimited,
    ProviderTimeout,
    ProviderTransientHttp,
    ProviderEmptyTextOutput,
    ProviderModelUnavailable,
    ProviderBillingBlocked,
    LocalRuntimeUnavailable,
    LocalRuntimeBusy,
    LocalRuntimeHung,
    ProviderError,
    ToolFailure,
    WorkflowFailure,
    RuntimeFailure,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResilienceFaultDispositionV1 {
    Retryable,
    Terminal,
    OperatorGated,
    DegradedAllowed,
    QuarantineRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceFaultClassificationV1 {
    pub schema_version: String,
    pub surface: ResilienceSurfaceV1,
    pub fault_class: ResilienceFaultClassV1,
    pub disposition: ResilienceFaultDispositionV1,
    pub retryable: bool,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CitizenHealthStateV1 {
    Healthy,
    Degraded,
    Recovering,
    Blocked,
    Quarantined,
    Sleeping,
    Hibernating,
    Migrating,
    Restoring,
    Replaying,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CitizenHealthRecordV1 {
    pub schema_version: String,
    pub citizen_id: String,
    pub state: CitizenHealthStateV1,
    pub observed_at: String,
    pub continuity_claim: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryDispositionV1 {
    ResumeAllowed,
    RetryAllowed,
    QuarantineRequired,
    OperatorInterventionRequired,
    FallbackAllowed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RecoveryArtifactV1 {
    pub schema_version: String,
    pub artifact_id: String,
    pub surface: ResilienceSurfaceV1,
    pub triggering_fault: ResilienceFaultClassificationV1,
    pub disposition: RecoveryDispositionV1,
    pub next_action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKindV1 {
    Provisional,
    Durable,
    SleepWake,
    Migration,
    ReplayAnchor,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckpointRecordV1 {
    pub schema_version: String,
    pub checkpoint_id: String,
    pub kind: CheckpointKindV1,
    pub state_ref: String,
    pub created_at: String,
    pub replayable: bool,
    pub claim_boundary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citizen_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TelemetryEventKindV1 {
    RetryDecision,
    TimeoutDecision,
    CircuitBreakerDecision,
    RateLimitDecision,
    BulkheadDecision,
    FallbackDecision,
    RecoveryDecision,
    CheckpointCreated,
    CitizenHealthTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResilienceTelemetryEventV1 {
    pub schema_version: String,
    pub event_id: String,
    pub event_kind: TelemetryEventKindV1,
    pub surface: ResilienceSurfaceV1,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
}

impl ResilienceFaultClassificationV1 {
    pub fn provider(note: &str, http_status: Option<u16>) -> Self {
        let lower = note.to_ascii_lowercase();
        let (fault_class, disposition) = if lower.contains("unauthorized")
            || lower.contains("forbidden")
            || lower.contains("invalid api key")
            || lower.contains("invalid_api_key")
            || http_status == Some(401)
            || http_status == Some(403)
        {
            (
                ResilienceFaultClassV1::ProviderAuthError,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("missing required environment variable")
            || lower.contains("missing api_key")
            || lower.contains("missing api key")
        {
            (
                ResilienceFaultClassV1::ProviderAuthMissing,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("rate limit")
            || lower.contains("rate_limited")
            || http_status == Some(429)
        {
            (
                ResilienceFaultClassV1::ProviderRateLimited,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("timed out") || lower.contains("timeout") {
            (
                ResilienceFaultClassV1::ProviderTimeout,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("credit balance") || lower.contains("billing") {
            (
                ResilienceFaultClassV1::ProviderBillingBlocked,
                ResilienceFaultDispositionV1::OperatorGated,
            )
        } else if lower.contains("local_runtime_busy") || lower.contains("non-target model") {
            (
                ResilienceFaultClassV1::LocalRuntimeBusy,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("local_runtime_hung") || lower.contains("stopping...") {
            (
                ResilienceFaultClassV1::LocalRuntimeHung,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("connection refused")
            || lower.contains("ollama") && lower.contains("not running")
            || lower.contains("local_runtime_unavailable")
        {
            (
                ResilienceFaultClassV1::LocalRuntimeUnavailable,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if lower.contains("model")
            && (lower.contains("not found") || lower.contains("does not exist"))
        {
            (
                ResilienceFaultClassV1::ProviderModelUnavailable,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else if lower.contains("empty")
            && (lower.contains("response") || lower.contains("output"))
        {
            (
                ResilienceFaultClassV1::ProviderEmptyTextOutput,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else if matches!(http_status, Some(500..=599)) {
            (
                ResilienceFaultClassV1::ProviderTransientHttp,
                ResilienceFaultDispositionV1::Retryable,
            )
        } else if http_status.is_some() || lower.contains("provider_") {
            (
                ResilienceFaultClassV1::ProviderError,
                ResilienceFaultDispositionV1::Terminal,
            )
        } else {
            (
                ResilienceFaultClassV1::Unknown,
                ResilienceFaultDispositionV1::Retryable,
            )
        };

        let retryable = matches!(disposition, ResilienceFaultDispositionV1::Retryable);
        Self {
            schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
            surface: ResilienceSurfaceV1::Provider,
            fault_class,
            disposition,
            retryable,
            summary: sanitize_resilience_summary(note),
            component_ref: None,
            http_status,
            retry_after_ms: None,
        }
    }
}

pub(crate) fn sanitize_resilience_summary(note: &str) -> String {
    let text = note.split_whitespace().collect::<Vec<_>>().join(" ");
    let lowered = text.to_ascii_lowercase();
    let sensitive = [
        "authorization",
        "bearer ",
        "x-api-key",
        "key=",
        "api_key=",
        "api key",
        ".key",
        "prompt:",
        "raw prompt",
        "user said",
        "messages",
        "tool arguments",
        "tool_args",
        "request body",
        "request_body",
    ];
    if sensitive.iter().any(|marker| lowered.contains(marker)) {
        return "redacted provider diagnostic".to_string();
    }
    truncate_chars(&text, 180)
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut iter = text.chars();
    let mut out: String = iter.by_ref().take(max_chars).collect();
    if iter.next().is_some() {
        let keep = out.chars().count().saturating_sub(3);
        out = out.chars().take(keep).collect();
        out.push_str("...");
    }
    out
}
