use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

static BULKHEAD_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BulkheadFinalStatusV1 {
    Allowed,
    Saturated,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BulkheadStateV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub fault_domain: String,
    pub in_flight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BulkheadExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub fault_domain: String,
    pub final_status: BulkheadFinalStatusV1,
    pub in_flight_before: u32,
    pub in_flight_during_execution: u32,
    pub in_flight_after: u32,
    pub max_concurrency: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_queue_depth: Option<u32>,
    pub operation_executed: bool,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct BulkheadExecution<T, E> {
    pub result: Result<T, E>,
    pub state: BulkheadStateV1,
    pub trace: BulkheadExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BulkheadPolicyV1 {
    pub fault_domain: String,
    pub max_concurrency: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_queue_depth: Option<u32>,
}

pub fn bulkhead_initial_state(policy: &ResiliencePolicyV1) -> BulkheadStateV1 {
    BulkheadStateV1 {
        schema_version: RESILIENCE_BULKHEAD_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        fault_domain: bulkhead_fault_domain(policy),
        in_flight: 0,
    }
}

pub fn execute_bulkhead_policy<T, E, F, C, R>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    current_state: &BulkheadStateV1,
    operation: F,
    mut classify_error: C,
    mut rejection_error: R,
) -> BulkheadExecution<T, E>
where
    F: FnOnce() -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    R: FnMut(&BulkheadStateV1) -> E,
{
    let state = bulkhead_state_for_policy(current_state, policy);
    let Some(bulkhead_policy) = policy.bulkhead.as_ref() else {
        let result = operation();
        let fault = result.as_ref().err().map(&mut classify_error);
        let decision_summary = format!("{operation_ref}: bulkhead disabled; operation completed");
        let telemetry_event = Some(bulkhead_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            fault.clone(),
        ));
        let state = bulkhead_initial_state(policy);
        return BulkheadExecution {
            result,
            state: state.clone(),
            trace: BulkheadExecutionTraceV1 {
                schema_version: RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                fault_domain: state.fault_domain.clone(),
                final_status: BulkheadFinalStatusV1::Allowed,
                in_flight_before: 0,
                in_flight_during_execution: 0,
                in_flight_after: 0,
                max_concurrency: 0,
                max_queue_depth: None,
                operation_executed: true,
                decision_summary,
                fault,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    };

    let state_before = state.in_flight;
    if state_before < bulkhead_policy.max_concurrency {
        let result = operation();
        let fault = result.as_ref().err().map(&mut classify_error);
        let decision_summary = format!(
            "{operation_ref}: bulkhead admitted fault domain '{}' at {}/{} in-flight",
            bulkhead_policy.fault_domain,
            state_before.saturating_add(1),
            bulkhead_policy.max_concurrency
        );
        let telemetry_event = Some(bulkhead_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            fault.clone(),
        ));
        return BulkheadExecution {
            result,
            state: state.clone(),
            trace: BulkheadExecutionTraceV1 {
                schema_version: RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                fault_domain: bulkhead_policy.fault_domain.clone(),
                final_status: BulkheadFinalStatusV1::Allowed,
                in_flight_before: state_before,
                in_flight_during_execution: state_before.saturating_add(1),
                in_flight_after: state_before,
                max_concurrency: bulkhead_policy.max_concurrency,
                max_queue_depth: bulkhead_policy.max_queue_depth,
                operation_executed: true,
                decision_summary,
                fault,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    }

    let error = rejection_error(&state);
    let fault = classify_error(&error);
    let decision_summary = format!(
        "{operation_ref}: bulkhead saturated for fault domain '{}' at {}/{} in-flight",
        bulkhead_policy.fault_domain, state_before, bulkhead_policy.max_concurrency
    );
    let telemetry_event = Some(bulkhead_decision_event(
        policy,
        surface.clone(),
        operation_ref,
        &decision_summary,
        Some(fault.clone()),
    ));
    let recovery_artifact = Some(bulkhead_recovery_artifact(
        policy,
        surface.clone(),
        operation_ref,
        &fault,
        &bulkhead_policy.fault_domain,
    ));
    BulkheadExecution {
        result: Err(error),
        state: state.clone(),
        trace: BulkheadExecutionTraceV1 {
            schema_version: RESILIENCE_BULKHEAD_EXECUTION_TRACE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            surface,
            fault_domain: bulkhead_policy.fault_domain.clone(),
            final_status: BulkheadFinalStatusV1::Saturated,
            in_flight_before: state_before,
            in_flight_during_execution: state_before,
            in_flight_after: state_before,
            max_concurrency: bulkhead_policy.max_concurrency,
            max_queue_depth: bulkhead_policy.max_queue_depth,
            operation_executed: false,
            decision_summary,
            fault: Some(fault),
            telemetry_event,
            recovery_artifact,
        },
    }
}

fn bulkhead_fault_domain(policy: &ResiliencePolicyV1) -> String {
    policy
        .bulkhead
        .as_ref()
        .map(|bulkhead| bulkhead.fault_domain.clone())
        .unwrap_or_else(|| "unbounded".to_string())
}

fn bulkhead_state_for_policy(
    state: &BulkheadStateV1,
    policy: &ResiliencePolicyV1,
) -> BulkheadStateV1 {
    let expected_fault_domain = bulkhead_fault_domain(policy);
    if state.policy_id == policy.policy_id && state.fault_domain == expected_fault_domain {
        state.clone()
    } else {
        bulkhead_initial_state(policy)
    }
}

fn bulkhead_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = bulkhead_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:bulkhead:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::BulkheadDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn bulkhead_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    fault_domain: &str,
) -> RecoveryArtifactV1 {
    let correlation_suffix = bulkhead_execution_correlation_suffix();
    let disposition = match fault.disposition {
        ResilienceFaultDispositionV1::Retryable => RecoveryDispositionV1::RetryAllowed,
        ResilienceFaultDispositionV1::DegradedAllowed => RecoveryDispositionV1::FallbackAllowed,
        ResilienceFaultDispositionV1::QuarantineRequired => {
            RecoveryDispositionV1::QuarantineRequired
        }
        ResilienceFaultDispositionV1::Terminal | ResilienceFaultDispositionV1::OperatorGated => {
            RecoveryDispositionV1::OperatorInterventionRequired
        }
    };
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:bulkhead:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition,
        next_action: format!(
            "preserve isolation for fault domain '{fault_domain}' and retry only after in-flight work drains or operator policy changes"
        ),
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone(), fault_domain.to_string()],
    }
}

fn bulkhead_execution_correlation_suffix() -> String {
    BULKHEAD_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}
