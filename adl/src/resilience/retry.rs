use crate::model_identity::observed_at_now_v1;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryPolicyV1 {
    pub max_attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jitter_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_elapsed_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retryable_fault_classes: Vec<ResilienceFaultClassV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetryTerminalReasonV1 {
    Succeeded,
    NonRetryableFault,
    RetryBudgetExhausted,
    RetryTimeBudgetExhausted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryAttemptRecordV1 {
    pub schema_version: String,
    pub attempt_index: u32,
    pub started_at: String,
    pub duration_ms: u64,
    pub retry_allowed: bool,
    pub scheduled_backoff_ms: u64,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_reason: Option<RetryTerminalReasonV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetryExecutionFinalStatusV1 {
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetryExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: RetryExecutionFinalStatusV1,
    pub total_duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attempts: Vec<RetryAttemptRecordV1>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telemetry_events: Vec<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct RetryExecution<T, E> {
    pub result: Result<T, E>,
    pub trace: RetryExecutionTraceV1,
}

impl RetryPolicyV1 {
    pub(super) fn permits_fault(&self, classification: &ResilienceFaultClassificationV1) -> bool {
        if !classification.retryable {
            return false;
        }
        self.retryable_fault_classes.is_empty()
            || self
                .retryable_fault_classes
                .contains(&classification.fault_class)
    }

    pub(super) fn next_delay_ms(&self, policy_id: &str, attempt_index: u32) -> u64 {
        let base = self.backoff_ms.unwrap_or(0);
        let multiplier_shift = attempt_index.saturating_sub(1).min(20);
        let multiplier = 1_u64 << multiplier_shift;
        let backoff = base.saturating_mul(multiplier);
        backoff.saturating_add(deterministic_jitter_ms(
            policy_id,
            attempt_index,
            self.jitter_ms.unwrap_or(0),
        ))
    }
}

pub fn execute_retry_policy<T, E, F, C, S, O>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    mut operation: F,
    mut classify_error: C,
    mut sleep_fn: S,
    mut observe_attempt: O,
) -> RetryExecution<T, E>
where
    E: Clone,
    F: FnMut(u32) -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    S: FnMut(u64),
    O: FnMut(&RetryAttemptRecordV1),
{
    let retry = policy.retry.clone().unwrap_or(RetryPolicyV1 {
        max_attempts: 1,
        backoff_ms: None,
        jitter_ms: None,
        max_elapsed_ms: None,
        retryable_fault_classes: Vec::new(),
    });
    let started = Instant::now();
    let mut attempts = Vec::new();
    let mut telemetry_events = Vec::new();

    for attempt_index in 1..=retry.max_attempts.max(1) {
        let attempt_started_at = observed_at_now_v1();
        let attempt_started = Instant::now();
        match operation(attempt_index) {
            Ok(value) => {
                let record = RetryAttemptRecordV1 {
                    schema_version: RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1.to_string(),
                    attempt_index,
                    started_at: attempt_started_at.clone(),
                    duration_ms: attempt_started.elapsed().as_millis() as u64,
                    retry_allowed: false,
                    scheduled_backoff_ms: 0,
                    decision_summary: format!("attempt {attempt_index} succeeded"),
                    terminal_reason: Some(RetryTerminalReasonV1::Succeeded),
                    fault: None,
                };
                observe_attempt(&record);
                telemetry_events.push(retry_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    attempt_index,
                    &record.decision_summary,
                    None,
                ));
                attempts.push(record);
                return RetryExecution {
                    result: Ok(value),
                    trace: RetryExecutionTraceV1 {
                        schema_version: RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: RetryExecutionFinalStatusV1::Succeeded,
                        total_duration_ms: started.elapsed().as_millis() as u64,
                        attempts,
                        telemetry_events,
                        recovery_artifact: None,
                    },
                };
            }
            Err(error) => {
                let classification = classify_error(&error);
                let delay_ms = retry
                    .next_delay_ms(&policy.policy_id, attempt_index)
                    .max(classification.retry_after_ms.unwrap_or(0));
                let elapsed_ms = started.elapsed().as_millis() as u64;
                let retryable_fault = retry.permits_fault(&classification);
                let within_attempt_budget = attempt_index < retry.max_attempts.max(1);
                let within_time_budget = retry
                    .max_elapsed_ms
                    .map(|max_elapsed_ms| elapsed_ms.saturating_add(delay_ms) <= max_elapsed_ms)
                    .unwrap_or(true);
                let retry_allowed = retryable_fault && within_attempt_budget && within_time_budget;
                let terminal_reason = if retry_allowed {
                    None
                } else if !retryable_fault {
                    Some(RetryTerminalReasonV1::NonRetryableFault)
                } else if !within_attempt_budget {
                    Some(RetryTerminalReasonV1::RetryBudgetExhausted)
                } else {
                    Some(RetryTerminalReasonV1::RetryTimeBudgetExhausted)
                };
                let decision_summary = retry_decision_summary(
                    attempt_index,
                    &classification,
                    retry_allowed,
                    delay_ms,
                    terminal_reason.as_ref(),
                );
                let record = RetryAttemptRecordV1 {
                    schema_version: RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1.to_string(),
                    attempt_index,
                    started_at: attempt_started_at,
                    duration_ms: attempt_started.elapsed().as_millis() as u64,
                    retry_allowed,
                    scheduled_backoff_ms: if retry_allowed { delay_ms } else { 0 },
                    decision_summary: decision_summary.clone(),
                    terminal_reason: terminal_reason.clone(),
                    fault: Some(classification.clone()),
                };
                observe_attempt(&record);
                telemetry_events.push(retry_decision_event(
                    policy,
                    surface.clone(),
                    operation_ref,
                    attempt_index,
                    &decision_summary,
                    Some(classification.clone()),
                ));
                attempts.push(record);
                if retry_allowed {
                    sleep_fn(delay_ms);
                    continue;
                }

                let recovery_artifact = Some(recovery_artifact_for_failure(
                    policy,
                    surface.clone(),
                    attempt_index,
                    &classification,
                    terminal_reason.unwrap_or(RetryTerminalReasonV1::NonRetryableFault),
                ));
                return RetryExecution {
                    result: Err(error),
                    trace: RetryExecutionTraceV1 {
                        schema_version: RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                        policy_id: policy.policy_id.clone(),
                        surface,
                        final_status: RetryExecutionFinalStatusV1::Failed,
                        total_duration_ms: started.elapsed().as_millis() as u64,
                        attempts,
                        telemetry_events,
                        recovery_artifact,
                    },
                };
            }
        }
    }

    unreachable!("retry execution should return inside the attempt loop")
}

fn deterministic_jitter_ms(policy_id: &str, attempt_index: u32, max_jitter_ms: u64) -> u64 {
    if max_jitter_ms == 0 {
        return 0;
    }
    let mut hash = 1469598103934665603_u64;
    for byte in policy_id.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    hash ^= u64::from(attempt_index);
    hash = hash.wrapping_mul(1099511628211);
    hash % max_jitter_ms.saturating_add(1)
}

fn retry_decision_summary(
    attempt_index: u32,
    classification: &ResilienceFaultClassificationV1,
    retry_allowed: bool,
    delay_ms: u64,
    terminal_reason: Option<&RetryTerminalReasonV1>,
) -> String {
    if retry_allowed {
        format!(
            "attempt {attempt_index} classified {:?}; retry scheduled after {delay_ms}ms",
            classification.fault_class
        )
    } else {
        match terminal_reason {
            Some(reason) => format!(
                "attempt {attempt_index} classified {:?}; terminal reason {:?}",
                classification.fault_class, reason
            ),
            None => format!(
                "attempt {attempt_index} classified {:?}; retry not allowed",
                classification.fault_class
            ),
        }
    }
}

fn retry_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    attempt_index: u32,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!("{}:retry:{attempt_index}", policy.policy_id),
        event_kind: TelemetryEventKindV1::RetryDecision,
        surface,
        decision_summary: format!("{operation_ref}: {decision_summary}"),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn recovery_artifact_for_failure(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    attempt_index: u32,
    triggering_fault: &ResilienceFaultClassificationV1,
    terminal_reason: RetryTerminalReasonV1,
) -> RecoveryArtifactV1 {
    let (disposition, next_action) = match terminal_reason {
        RetryTerminalReasonV1::RetryBudgetExhausted => (
            RecoveryDispositionV1::OperatorInterventionRequired,
            format!(
                "retry budget exhausted after attempt {attempt_index}; operator must decide whether to widen the retry budget"
            ),
        ),
        RetryTerminalReasonV1::RetryTimeBudgetExhausted => (
            RecoveryDispositionV1::OperatorInterventionRequired,
            format!(
                "retry time budget exhausted after attempt {attempt_index}; operator must decide whether to widen the elapsed budget"
            ),
        ),
        RetryTerminalReasonV1::NonRetryableFault => match triggering_fault.disposition {
            ResilienceFaultDispositionV1::DegradedAllowed => (
                RecoveryDispositionV1::FallbackAllowed,
                "fault is non-retryable here; route to a degraded or fallback path".to_string(),
            ),
            ResilienceFaultDispositionV1::QuarantineRequired => (
                RecoveryDispositionV1::QuarantineRequired,
                "fault requires quarantine before retrying the surface".to_string(),
            ),
            _ => (
                RecoveryDispositionV1::OperatorInterventionRequired,
                "fault is non-retryable; inspect the failure before attempting recovery".to_string(),
            ),
        },
        RetryTerminalReasonV1::Succeeded => (
            RecoveryDispositionV1::ResumeAllowed,
            "operation completed successfully".to_string(),
        ),
    };
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!("{}:recovery:{attempt_index}", policy.policy_id),
        surface,
        triggering_fault: triggering_fault.clone(),
        disposition,
        next_action,
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
    }
}
