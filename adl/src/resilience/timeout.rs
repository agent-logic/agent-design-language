use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

static TIMEOUT_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TimeoutPolicyV1 {
    pub timeout_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_deadline_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutBreachKindV1 {
    Timeout,
    HardDeadline,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeoutExecutionFinalStatusV1 {
    Succeeded,
    Failed,
    TimedOut,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutObservation<T, E> {
    pub result: Result<T, E>,
    pub elapsed_ms: u64,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TimeoutExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: TimeoutExecutionFinalStatusV1,
    pub elapsed_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hard_deadline_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breach_kind: Option<TimeoutBreachKindV1>,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct TimeoutExecution<T, E> {
    pub result: Result<T, E>,
    pub trace: TimeoutExecutionTraceV1,
}

pub fn execute_timeout_policy<T, E, F, C, TO, CO>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    operation: F,
    mut classify_error: C,
    mut timeout_error: TO,
    mut cancellation_error: CO,
) -> TimeoutExecution<T, E>
where
    F: FnOnce() -> TimeoutObservation<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    TO: FnMut(TimeoutBreachKindV1, u64, u64) -> E,
    CO: FnMut(u64) -> E,
{
    let observation = operation();
    let timeout_ms = policy.timeout.as_ref().map(|timeout| timeout.timeout_ms);
    let hard_deadline_ms = policy
        .timeout
        .as_ref()
        .and_then(|timeout| timeout.hard_deadline_ms);
    let breach = timeout_breach(timeout_ms, hard_deadline_ms, observation.elapsed_ms);

    if observation.cancelled {
        let error = cancellation_error(observation.elapsed_ms);
        let fault = timeout_cancellation_fault(surface.clone(), operation_ref);
        let decision_summary = format!(
            "{operation_ref}: operation cancelled after {}ms",
            observation.elapsed_ms
        );
        let telemetry_event = Some(timeout_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            Some(fault.clone()),
        ));
        let recovery_artifact = Some(timeout_recovery_artifact(
            policy,
            surface.clone(),
            operation_ref,
            &fault,
            RecoveryDispositionV1::ResumeAllowed,
            "handle explicit cancellation before retrying or rescheduling",
        ));
        return TimeoutExecution {
            result: Err(error),
            trace: TimeoutExecutionTraceV1 {
                schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                final_status: TimeoutExecutionFinalStatusV1::Cancelled,
                elapsed_ms: observation.elapsed_ms,
                timeout_ms,
                hard_deadline_ms,
                breach_kind: None,
                decision_summary,
                fault: Some(fault),
                telemetry_event,
                recovery_artifact,
            },
        };
    }

    match observation.result {
        Ok(value) => {
            if let Some((breach_kind, breached_budget_ms)) = breach.clone() {
                let error = timeout_error(
                    breach_kind.clone(),
                    observation.elapsed_ms,
                    breached_budget_ms,
                );
                let fault = timeout_deadline_fault(
                    surface.clone(),
                    operation_ref,
                    observation.elapsed_ms,
                    breach_kind.clone(),
                    breached_budget_ms,
                );
                let decision_summary = format!(
                    "{operation_ref}: {} exceeded after {}ms (budget {}ms)",
                    timeout_breach_label(&breach_kind),
                    observation.elapsed_ms,
                    breached_budget_ms
                );
                let telemetry_event = Some(timeout_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &decision_summary,
                    Some(fault.clone()),
                ));
                let recovery_artifact = Some(timeout_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "operation exceeded deadline; retry only through the caller's bounded policy",
                ));
                return TimeoutExecution {
                    result: Err(error),
                    trace: TimeoutExecutionTraceV1 {
                        schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: TimeoutExecutionFinalStatusV1::TimedOut,
                        elapsed_ms: observation.elapsed_ms,
                        timeout_ms,
                        hard_deadline_ms,
                        breach_kind: Some(breach_kind),
                        decision_summary,
                        fault: Some(fault),
                        telemetry_event,
                        recovery_artifact,
                    },
                };
            }

            let decision_summary =
                format!("{operation_ref}: completed before timeout/deadline budget");
            let telemetry_event = timeout_ms.map(|_| {
                timeout_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &decision_summary,
                    None,
                )
            });
            TimeoutExecution {
                result: Ok(value),
                trace: TimeoutExecutionTraceV1 {
                    schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status: TimeoutExecutionFinalStatusV1::Succeeded,
                    elapsed_ms: observation.elapsed_ms,
                    timeout_ms,
                    hard_deadline_ms,
                    breach_kind: None,
                    decision_summary,
                    fault: None,
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
        Err(error) => {
            let classification = classify_error(&error);
            let timed_out = classification_represents_timeout(&classification);
            let final_status = if timed_out {
                TimeoutExecutionFinalStatusV1::TimedOut
            } else {
                TimeoutExecutionFinalStatusV1::Failed
            };
            let decision_summary = if timed_out {
                let budget_summary = breach
                    .as_ref()
                    .map(|(kind, ms)| format!(" ({}, {}ms)", timeout_breach_label(kind), ms))
                    .unwrap_or_default();
                format!(
                    "{operation_ref}: timeout failure after {}ms{}",
                    observation.elapsed_ms, budget_summary
                )
            } else if let Some((kind, budget_ms)) = breach.as_ref() {
                format!(
                    "{operation_ref}: failed after {} exceeded ({}ms budget) with {:?}",
                    timeout_breach_label(kind),
                    budget_ms,
                    classification.fault_class
                )
            } else {
                format!(
                    "{operation_ref}: failed before deadline with {:?}",
                    classification.fault_class
                )
            };
            let telemetry_event = Some(timeout_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                Some(classification.clone()),
            ));
            let recovery_artifact = if timed_out {
                Some(timeout_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &classification,
                    RecoveryDispositionV1::RetryAllowed,
                    "timeout classified distinctly from business failure; retry only through the caller's bounded policy",
                ))
            } else {
                None
            };
            TimeoutExecution {
                result: Err(error),
                trace: TimeoutExecutionTraceV1 {
                    schema_version: RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status,
                    elapsed_ms: observation.elapsed_ms,
                    timeout_ms,
                    hard_deadline_ms,
                    breach_kind: if timed_out {
                        breach.as_ref().map(|(kind, _)| kind.clone())
                    } else {
                        None
                    },
                    decision_summary,
                    fault: Some(classification),
                    telemetry_event,
                    recovery_artifact,
                },
            }
        }
    }
}

pub(super) fn timeout_deadline_fault(
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    elapsed_ms: u64,
    breach_kind: TimeoutBreachKindV1,
    breached_budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{operation_ref} exceeded {} after {elapsed_ms}ms (budget {breached_budget_ms}ms)",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: Some(operation_ref.to_string()),
        http_status: None,
        retry_after_ms: None,
    }
}

pub(super) fn timeout_cancellation_fault(
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("{operation_ref} cancelled before completion"),
        component_ref: Some(operation_ref.to_string()),
        http_status: None,
        retry_after_ms: None,
    }
}

pub(super) fn timeout_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = timeout_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:timeout:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::TimeoutDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

pub(super) fn timeout_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    disposition: RecoveryDispositionV1,
    next_action: &str,
) -> RecoveryArtifactV1 {
    let correlation_suffix = timeout_execution_correlation_suffix();
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:timeout:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition,
        next_action: next_action.to_string(),
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
    }
}

fn timeout_execution_correlation_suffix() -> String {
    TIMEOUT_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}

pub(super) fn timeout_breach(
    timeout_ms: Option<u64>,
    hard_deadline_ms: Option<u64>,
    elapsed_ms: u64,
) -> Option<(TimeoutBreachKindV1, u64)> {
    let mut budgets = Vec::new();
    if let Some(timeout_ms) = timeout_ms {
        budgets.push((timeout_ms, TimeoutBreachKindV1::Timeout));
    }
    if let Some(hard_deadline_ms) = hard_deadline_ms {
        budgets.push((hard_deadline_ms, TimeoutBreachKindV1::HardDeadline));
    }
    budgets.sort_by_key(|(budget_ms, _)| *budget_ms);
    budgets
        .into_iter()
        .find(|(budget_ms, _)| elapsed_ms > *budget_ms)
        .map(|(budget_ms, kind)| (kind, budget_ms))
}

pub(super) fn timeout_breach_label(kind: &TimeoutBreachKindV1) -> &'static str {
    match kind {
        TimeoutBreachKindV1::Timeout => "timeout budget",
        TimeoutBreachKindV1::HardDeadline => "hard deadline",
    }
}

pub(super) fn classification_represents_timeout(
    classification: &ResilienceFaultClassificationV1,
) -> bool {
    if classification.fault_class == ResilienceFaultClassV1::ProviderTimeout
        || classification.fault_class == ResilienceFaultClassV1::LocalRuntimeHung
    {
        return true;
    }
    if matches!(
        classification.fault_class,
        ResilienceFaultClassV1::RuntimeFailure
            | ResilienceFaultClassV1::WorkflowFailure
            | ResilienceFaultClassV1::ToolFailure
            | ResilienceFaultClassV1::Unknown
    ) {
        let summary = classification.summary.to_ascii_lowercase();
        return summary.contains("timeout")
            || summary.contains("timed out")
            || summary.contains("deadline");
    }
    false
}
