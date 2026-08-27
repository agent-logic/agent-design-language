use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use super::fallback::fallback_allowed_for_policy;
use super::*;

static CIRCUIT_BREAKER_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CircuitBreakerStateKindV1 {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CircuitBreakerFinalStatusV1 {
    ClosedSuccess,
    ClosedFailure,
    OpenRejected,
    OpenFallback,
    HalfOpenProbeSuccess,
    HalfOpenProbeFailure,
    HalfOpenProbeRejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerStateV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub state: CircuitBreakerStateKindV1,
    pub consecutive_failures: u32,
    pub half_open_attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opened_at_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_failure: Option<ResilienceFaultClassificationV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub state_before: CircuitBreakerStateKindV1,
    pub state_after: CircuitBreakerStateKindV1,
    pub final_status: CircuitBreakerFinalStatusV1,
    pub operation_executed: bool,
    pub used_fallback: bool,
    pub now_ms: u64,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct CircuitBreakerExecution<T, E> {
    pub result: Result<T, E>,
    pub state: CircuitBreakerStateV1,
    pub trace: CircuitBreakerExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CircuitBreakerPolicyV1 {
    pub failure_threshold: u32,
    pub recovery_window_ms: u64,
    pub half_open_max_attempts: u32,
}

pub fn circuit_breaker_initial_state(policy: &ResiliencePolicyV1) -> CircuitBreakerStateV1 {
    CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Closed,
        consecutive_failures: 0,
        half_open_attempts: 0,
        opened_at_ms: None,
        last_failure: None,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn execute_circuit_breaker_policy<T, E, F, C, R, FB>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    current_state: &CircuitBreakerStateV1,
    now_ms: u64,
    operation: F,
    mut classify_error: C,
    mut rejection_error: R,
    mut fallback: Option<FB>,
) -> CircuitBreakerExecution<T, E>
where
    F: FnOnce() -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    R: FnMut(&CircuitBreakerStateV1, u64) -> E,
    FB: FnMut() -> T,
{
    let policy_state = circuit_breaker_state_for_policy(current_state, policy);
    let Some(breaker_policy) = policy.circuit_breaker.as_ref() else {
        let result = operation();
        let state = circuit_breaker_initial_state(policy);
        let fault = result.as_ref().err().map(&mut classify_error);
        let final_status = if result.is_ok() {
            CircuitBreakerFinalStatusV1::ClosedSuccess
        } else {
            CircuitBreakerFinalStatusV1::ClosedFailure
        };
        let decision_summary = if result.is_ok() {
            format!("{operation_ref}: breaker disabled; operation completed")
        } else {
            format!("{operation_ref}: breaker disabled; operation failed")
        };
        let telemetry_event = Some(circuit_breaker_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            fault.clone(),
        ));
        return CircuitBreakerExecution {
            result,
            state: state.clone(),
            trace: CircuitBreakerExecutionTraceV1 {
                schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                state_before: CircuitBreakerStateKindV1::Closed,
                state_after: CircuitBreakerStateKindV1::Closed,
                final_status,
                operation_executed: true,
                used_fallback: false,
                now_ms,
                decision_summary,
                fault,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    };

    let state_before = policy_state.state.clone();
    let normalized_state = circuit_breaker_state_for_now(&policy_state, breaker_policy, now_ms);
    let fallback_allowed = normalized_state
        .last_failure
        .as_ref()
        .map(|fault| fallback_allowed_for_policy(policy, fault))
        .unwrap_or(false);

    match normalized_state.state {
        CircuitBreakerStateKindV1::Open => {
            if fallback_allowed {
                if let Some(ref mut fallback_fn) = fallback {
                    let value = fallback_fn();
                    let decision_summary = format!(
                        "{operation_ref}: breaker open at {} failures; fallback executed",
                        normalized_state.consecutive_failures
                    );
                    let telemetry_event = Some(circuit_breaker_decision_event(
                        policy,
                        surface.clone(),
                        operation_ref,
                        &decision_summary,
                        normalized_state.last_failure.clone(),
                    ));
                    let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                        circuit_breaker_recovery_artifact(
                            policy,
                            surface.clone(),
                            operation_ref,
                            fault,
                            RecoveryDispositionV1::FallbackAllowed,
                            "breaker remained open; fallback path executed instead of calling the dependency",
                        )
                    });
                    return CircuitBreakerExecution {
                        result: Ok(value),
                        state: normalized_state.clone(),
                        trace: CircuitBreakerExecutionTraceV1 {
                            schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                                .to_string(),
                            policy_id: policy.policy_id.clone(),
                            surface,
                            state_before,
                            state_after: normalized_state.state.clone(),
                            final_status: CircuitBreakerFinalStatusV1::OpenFallback,
                            operation_executed: false,
                            used_fallback: true,
                            now_ms,
                            decision_summary,
                            fault: normalized_state.last_failure.clone(),
                            telemetry_event,
                            recovery_artifact,
                        },
                    };
                }
            }

            let error = rejection_error(&normalized_state, now_ms);
            let decision_summary = if fallback.is_some() && normalized_state.last_failure.is_some()
            {
                format!(
                    "{operation_ref}: breaker open at {} failures; fallback policy did not activate",
                    normalized_state.consecutive_failures
                )
            } else {
                format!(
                    "{operation_ref}: breaker open at {} failures; dependency call rejected",
                    normalized_state.consecutive_failures
                )
            };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                normalized_state.last_failure.clone(),
            ));
            let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "breaker remained open; wait for the recovery window before probing again",
                )
            });
            return CircuitBreakerExecution {
                result: Err(error),
                state: normalized_state.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: normalized_state.state.clone(),
                    final_status: CircuitBreakerFinalStatusV1::OpenRejected,
                    operation_executed: false,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: normalized_state.last_failure.clone(),
                    telemetry_event,
                    recovery_artifact,
                },
            };
        }
        CircuitBreakerStateKindV1::HalfOpen
            if normalized_state.half_open_attempts >= breaker_policy.half_open_max_attempts =>
        {
            let error = rejection_error(&normalized_state, now_ms);
            let decision_summary = format!(
                "{operation_ref}: half-open probe budget exhausted ({}/{})",
                normalized_state.half_open_attempts, breaker_policy.half_open_max_attempts
            );
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                normalized_state.last_failure.clone(),
            ));
            let recovery_artifact = normalized_state.last_failure.as_ref().map(|fault| {
                circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "half-open probe limit reached; wait for the next recovery window",
                )
            });
            return CircuitBreakerExecution {
                result: Err(error),
                state: normalized_state.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: normalized_state.state.clone(),
                    final_status: CircuitBreakerFinalStatusV1::HalfOpenProbeRejected,
                    operation_executed: false,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: normalized_state.last_failure.clone(),
                    telemetry_event,
                    recovery_artifact,
                },
            };
        }
        _ => {}
    }

    let mut state_after = normalized_state.clone();
    let half_open_probe_attempt = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
        let next_attempt = normalized_state.half_open_attempts.saturating_add(1);
        state_after.half_open_attempts = next_attempt;
        Some(next_attempt)
    } else {
        None
    };
    let result = operation();
    match result {
        Ok(value) => {
            let final_status = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
                CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess
            } else {
                CircuitBreakerFinalStatusV1::ClosedSuccess
            };
            state_after.state = CircuitBreakerStateKindV1::Closed;
            state_after.consecutive_failures = 0;
            state_after.half_open_attempts = 0;
            state_after.opened_at_ms = None;
            state_after.last_failure = None;
            let decision_summary =
                if final_status == CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess {
                    format!("{operation_ref}: half-open probe succeeded; breaker closed")
                } else {
                    format!("{operation_ref}: breaker remained closed after successful call")
                };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                None,
            ));
            CircuitBreakerExecution {
                result: Ok(value),
                state: state_after.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: state_after.state.clone(),
                    final_status,
                    operation_executed: true,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: None,
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
        Err(error) => {
            let fault = classify_error(&error);
            let final_status = if normalized_state.state == CircuitBreakerStateKindV1::HalfOpen {
                let probe_attempt = half_open_probe_attempt.unwrap_or(1);
                state_after.consecutive_failures = breaker_policy.failure_threshold;
                state_after.last_failure = Some(fault.clone());
                if probe_attempt >= breaker_policy.half_open_max_attempts {
                    state_after.state = CircuitBreakerStateKindV1::Open;
                    state_after.opened_at_ms = Some(now_ms);
                } else {
                    state_after.state = CircuitBreakerStateKindV1::HalfOpen;
                    state_after.opened_at_ms = None;
                }
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
            } else {
                state_after.consecutive_failures =
                    normalized_state.consecutive_failures.saturating_add(1);
                state_after.last_failure = Some(fault.clone());
                if state_after.consecutive_failures >= breaker_policy.failure_threshold {
                    state_after.state = CircuitBreakerStateKindV1::Open;
                    state_after.opened_at_ms = Some(now_ms);
                }
                CircuitBreakerFinalStatusV1::ClosedFailure
            };
            let decision_summary = match final_status {
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
                    if state_after.state == CircuitBreakerStateKindV1::Open =>
                {
                    format!("{operation_ref}: half-open probe failed; breaker reopened")
                }
                CircuitBreakerFinalStatusV1::HalfOpenProbeFailure => format!(
                    "{operation_ref}: half-open probe failed; {} probe attempt(s) remain before reopening",
                    breaker_policy
                        .half_open_max_attempts
                        .saturating_sub(state_after.half_open_attempts)
                ),
                _ if state_after.state == CircuitBreakerStateKindV1::Open => format!(
                    "{operation_ref}: breaker opened after {} consecutive failures",
                    state_after.consecutive_failures
                ),
                _ => format!(
                    "{operation_ref}: breaker counted failure {}/{} while remaining closed",
                    state_after.consecutive_failures, breaker_policy.failure_threshold
                ),
            };
            let telemetry_event = Some(circuit_breaker_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                Some(fault.clone()),
            ));
            let recovery_artifact = if state_after.state == CircuitBreakerStateKindV1::Open {
                Some(circuit_breaker_recovery_artifact(
                    policy,
                    surface.clone(),
                    operation_ref,
                    &fault,
                    RecoveryDispositionV1::RetryAllowed,
                    "breaker opened; defer new attempts until the recovery window allows a bounded half-open probe",
                ))
            } else {
                None
            };
            CircuitBreakerExecution {
                result: Err(error),
                state: state_after.clone(),
                trace: CircuitBreakerExecutionTraceV1 {
                    schema_version: RESILIENCE_CIRCUIT_BREAKER_EXECUTION_TRACE_SCHEMA_V1
                        .to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    state_before,
                    state_after: state_after.state.clone(),
                    final_status,
                    operation_executed: true,
                    used_fallback: false,
                    now_ms,
                    decision_summary,
                    fault: Some(fault),
                    telemetry_event,
                    recovery_artifact,
                },
            }
        }
    }
}

pub(super) fn circuit_breaker_state_for_now(
    state: &CircuitBreakerStateV1,
    policy: &CircuitBreakerPolicyV1,
    now_ms: u64,
) -> CircuitBreakerStateV1 {
    if state.state != CircuitBreakerStateKindV1::Open {
        return state.clone();
    }
    let ready_for_probe = state
        .opened_at_ms
        .map(|opened_at_ms| now_ms.saturating_sub(opened_at_ms) >= policy.recovery_window_ms)
        .unwrap_or(false);
    if !ready_for_probe {
        return state.clone();
    }

    let mut next = state.clone();
    next.state = CircuitBreakerStateKindV1::HalfOpen;
    next.half_open_attempts = 0;
    next
}

fn circuit_breaker_state_for_policy(
    state: &CircuitBreakerStateV1,
    policy: &ResiliencePolicyV1,
) -> CircuitBreakerStateV1 {
    if state.policy_id == policy.policy_id {
        state.clone()
    } else {
        circuit_breaker_initial_state(policy)
    }
}

pub(super) fn circuit_breaker_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = circuit_breaker_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:circuit-breaker:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::CircuitBreakerDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

pub(super) fn circuit_breaker_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    disposition: RecoveryDispositionV1,
    next_action: &str,
) -> RecoveryArtifactV1 {
    let correlation_suffix = circuit_breaker_execution_correlation_suffix();
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:circuit-breaker:{operation_ref}:{correlation_suffix}",
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

fn circuit_breaker_execution_correlation_suffix() -> String {
    CIRCUIT_BREAKER_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}
