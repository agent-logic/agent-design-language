use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

static FALLBACK_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackExecutionFinalStatusV1 {
    PrimarySuccess,
    PrimaryFailure,
    AlternateRouteSuccess,
    DegradedSuccess,
    FallbackUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackOutcomeKindV1 {
    Primary,
    AlternateRoute,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FallbackExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: FallbackExecutionFinalStatusV1,
    pub outcome_kind: FallbackOutcomeKindV1,
    pub fallback_executed: bool,
    pub output_degraded: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_ref: Option<String>,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct FallbackExecution<T, E> {
    pub result: Result<T, E>,
    pub outcome_kind: FallbackOutcomeKindV1,
    pub trace: FallbackExecutionTraceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FallbackPolicyV1 {
    pub fallback_ref: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_fault_classes: Vec<ResilienceFaultClassV1>,
    pub marks_output_degraded: bool,
}

pub(super) fn fallback_allowed_for_policy(
    policy: &ResiliencePolicyV1,
    fault: &ResilienceFaultClassificationV1,
) -> bool {
    let Some(fallback_policy) = policy.fallback.as_ref() else {
        return false;
    };
    fallback_policy.activation_fault_classes.is_empty()
        || fallback_policy
            .activation_fault_classes
            .contains(&fault.fault_class)
}

pub fn execute_fallback_policy<T, E, F, C, FB>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    operation: F,
    mut classify_error: C,
    mut fallback: Option<FB>,
) -> FallbackExecution<T, E>
where
    F: FnOnce() -> Result<T, E>,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
    FB: FnMut() -> T,
{
    let fallback_policy = policy.fallback.as_ref();
    match operation() {
        Ok(value) => {
            let decision_summary = format!(
                "{operation_ref}: primary path completed without fallback or degraded execution"
            );
            let telemetry_event = Some(fallback_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                None,
            ));
            FallbackExecution {
                result: Ok(value),
                outcome_kind: FallbackOutcomeKindV1::Primary,
                trace: FallbackExecutionTraceV1 {
                    schema_version: RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status: FallbackExecutionFinalStatusV1::PrimarySuccess,
                    outcome_kind: FallbackOutcomeKindV1::Primary,
                    fallback_executed: false,
                    output_degraded: false,
                    fallback_ref: fallback_policy.map(|fallback| fallback.fallback_ref.clone()),
                    decision_summary,
                    fault: None,
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
        Err(error) => {
            let classification = classify_error(&error);
            let fallback_allowed = fallback_allowed_for_policy(policy, &classification);
            let fallback_ref = fallback_policy.map(|fallback| fallback.fallback_ref.clone());
            let output_degraded = fallback_policy
                .map(|fallback| fallback.marks_output_degraded)
                .unwrap_or(false);

            if fallback_allowed {
                if let Some(ref mut fallback_fn) = fallback {
                    let value = fallback_fn();
                    let (final_status, outcome_kind, decision_summary) = if output_degraded {
                        (
                            FallbackExecutionFinalStatusV1::DegradedSuccess,
                            FallbackOutcomeKindV1::Degraded,
                            format!(
                                "{operation_ref}: primary path failed; degraded fallback '{}' executed",
                                fallback_ref.as_deref().unwrap_or("unnamed.fallback")
                            ),
                        )
                    } else {
                        (
                            FallbackExecutionFinalStatusV1::AlternateRouteSuccess,
                            FallbackOutcomeKindV1::AlternateRoute,
                            format!(
                                "{operation_ref}: primary path failed; alternate route '{}' executed",
                                fallback_ref.as_deref().unwrap_or("unnamed.fallback")
                            ),
                        )
                    };
                    let telemetry_event = Some(fallback_decision_event(
                        policy,
                        surface.clone(),
                        operation_ref,
                        &decision_summary,
                        Some(classification.clone()),
                    ));
                    let recovery_artifact = Some(fallback_recovery_artifact(
                        policy,
                        surface.clone(),
                        operation_ref,
                        &classification,
                        fallback_ref.as_deref().unwrap_or("unnamed.fallback"),
                        output_degraded,
                    ));
                    return FallbackExecution {
                        result: Ok(value),
                        outcome_kind: outcome_kind.clone(),
                        trace: FallbackExecutionTraceV1 {
                            schema_version: RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1
                                .to_string(),
                            policy_id: policy.policy_id.clone(),
                            surface,
                            final_status,
                            outcome_kind,
                            fallback_executed: true,
                            output_degraded,
                            fallback_ref,
                            decision_summary,
                            fault: Some(classification),
                            telemetry_event,
                            recovery_artifact,
                        },
                    };
                }
            }

            let final_status = if fallback_allowed {
                FallbackExecutionFinalStatusV1::FallbackUnavailable
            } else {
                FallbackExecutionFinalStatusV1::PrimaryFailure
            };
            let decision_summary = if fallback_allowed {
                format!(
                    "{operation_ref}: primary path failed; fallback policy allowed recovery but no fallback hook was available"
                )
            } else {
                format!("{operation_ref}: primary path failed; fallback policy did not activate")
            };
            let telemetry_event = Some(fallback_decision_event(
                policy,
                surface.clone(),
                operation_ref,
                &decision_summary,
                Some(classification.clone()),
            ));
            FallbackExecution {
                result: Err(error),
                outcome_kind: FallbackOutcomeKindV1::Primary,
                trace: FallbackExecutionTraceV1 {
                    schema_version: RESILIENCE_FALLBACK_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                    policy_id: policy.policy_id.clone(),
                    surface,
                    final_status,
                    outcome_kind: FallbackOutcomeKindV1::Primary,
                    fallback_executed: false,
                    output_degraded: false,
                    fallback_ref,
                    decision_summary,
                    fault: Some(classification),
                    telemetry_event,
                    recovery_artifact: None,
                },
            }
        }
    }
}

fn fallback_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = fallback_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:fallback:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::FallbackDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn fallback_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    fallback_ref: &str,
    output_degraded: bool,
) -> RecoveryArtifactV1 {
    let correlation_suffix = fallback_execution_correlation_suffix();
    let next_action = if output_degraded {
        format!(
            "surface degraded output explicitly for fallback '{fallback_ref}' and preserve the original primary failure for downstream handling"
        )
    } else {
        format!(
            "route to explicit alternate path '{fallback_ref}' while recording the original primary failure"
        )
    };
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:fallback:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition: RecoveryDispositionV1::FallbackAllowed,
        next_action,
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone(), fallback_ref.to_string()],
    }
}

fn fallback_execution_correlation_suffix() -> String {
    FALLBACK_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}
