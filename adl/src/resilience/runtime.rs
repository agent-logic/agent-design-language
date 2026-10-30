use crate::trace_schema_v1::{TraceEventTypeV1, TraceEventV1, TraceScopeLevelV1};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeHealthStateV1 {
    Healthy,
    Degraded,
    Blocked,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeResilienceDispositionV1 {
    Admitted,
    QueuedBackpressure,
    Succeeded,
    Timeout,
    Cancelled,
    DegradedContinue,
    TerminalFailure,
}

impl RuntimeResilienceDispositionV1 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::QueuedBackpressure => "queued_backpressure",
            Self::Succeeded => "succeeded",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::DegradedContinue => "degraded_continue",
            Self::TerminalFailure => "terminal_failure",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeResilienceTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub component: String,
    pub step_id: String,
    pub provider_id: String,
    pub task_id: String,
    pub watcher_disposition: RuntimeResilienceDispositionV1,
    pub middleware_disposition: RuntimeResilienceDispositionV1,
    pub terminal: bool,
    pub attempt_count: u32,
    pub elapsed_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_depth: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    pub decision_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCorrelationFieldsV1 {
    pub schema_version: String,
    pub surface: ResilienceSurfaceV1,
    pub component: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault_code: Option<String>,
}

impl RuntimeCorrelationFieldsV1 {
    pub fn new(surface: ResilienceSurfaceV1, component: impl Into<String>) -> Self {
        Self {
            schema_version: RUNTIME_CORRELATION_FIELDS_SCHEMA_V1.to_string(),
            surface,
            component: component.into(),
            run_id: None,
            trace_id: None,
            span_id: None,
            parent_span_id: None,
            task_id: None,
            fault_code: None,
        }
    }

    pub fn from_trace_event(
        event: &TraceEventV1,
        surface: ResilienceSurfaceV1,
        component: impl Into<String>,
    ) -> Self {
        let task_id = match event.scope.level {
            TraceScopeLevelV1::Step => Some(event.scope.name.clone()),
            _ => None,
        };
        let fault_code = match event.event_type {
            TraceEventTypeV1::Error => event.error.as_ref().map(|error| error.code.clone()),
            _ => None,
        };
        Self {
            schema_version: RUNTIME_CORRELATION_FIELDS_SCHEMA_V1.to_string(),
            surface,
            component: component.into(),
            run_id: Some(event.run_id.clone()),
            trace_id: Some(event.trace_id.clone()),
            span_id: Some(event.span_id.clone()),
            parent_span_id: event.parent_span_id.clone(),
            task_id,
            fault_code,
        }
    }

    pub fn field_contract() -> &'static [&'static str] {
        &[
            "run_id",
            "trace_id",
            "span_id",
            "parent_span_id",
            "task_id",
            "fault_code",
            "component",
            "surface",
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeHealthStatusV1 {
    pub schema_version: String,
    pub state: RuntimeHealthStateV1,
    pub summary: String,
    pub correlation: RuntimeCorrelationFieldsV1,
    pub field_contract: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl RuntimeHealthStatusV1 {
    pub fn healthy_runtime_component(
        component: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: RUNTIME_HEALTH_STATUS_SCHEMA_V1.to_string(),
            state: RuntimeHealthStateV1::Healthy,
            summary: summary.into(),
            correlation: RuntimeCorrelationFieldsV1::new(ResilienceSurfaceV1::Runtime, component),
            field_contract: RuntimeCorrelationFieldsV1::field_contract()
                .iter()
                .map(|field| (*field).to_string())
                .collect(),
            detail: None,
        }
    }

    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(self).expect("runtime health status is serializable")
    }
}

pub fn remote_exec_health_payload() -> Value {
    RuntimeHealthStatusV1::healthy_runtime_component(
        "remote_exec",
        "remote execution server ready for bounded request handling",
    )
    .to_json_value()
}
