use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

static RATE_LIMIT_EXECUTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RateLimitPolicyV1 {
    pub max_requests: u32,
    pub window_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitFinalStatusV1 {
    Allowed,
    Throttled,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RateLimitStateV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub window_started_at_ms: u64,
    pub requests_in_window: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RateLimitExecutionTraceV1 {
    pub schema_version: String,
    pub policy_id: String,
    pub surface: ResilienceSurfaceV1,
    pub final_status: RateLimitFinalStatusV1,
    pub window_started_at_ms: u64,
    pub requests_in_window_before: u32,
    pub requests_in_window_after: u32,
    pub max_requests: u32,
    pub window_ms: u64,
    pub operation_executed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_ms: Option<u64>,
    pub decision_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<ResilienceFaultClassificationV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_event: Option<ResilienceTelemetryEventV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_artifact: Option<RecoveryArtifactV1>,
}

#[derive(Debug)]
pub struct RateLimitExecution<T, E> {
    pub result: Result<T, E>,
    pub state: RateLimitStateV1,
    pub trace: RateLimitExecutionTraceV1,
}

pub fn rate_limit_initial_state(policy: &ResiliencePolicyV1, now_ms: u64) -> RateLimitStateV1 {
    RateLimitStateV1 {
        schema_version: RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        window_started_at_ms: now_ms,
        requests_in_window: 0,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn execute_rate_limit_policy<T, E, F, R, C>(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    current_state: &RateLimitStateV1,
    now_ms: u64,
    operation: F,
    mut rejection_error: R,
    mut classify_error: C,
) -> RateLimitExecution<T, E>
where
    F: FnOnce() -> Result<T, E>,
    R: FnMut(&RateLimitStateV1, u64) -> E,
    C: FnMut(&E) -> ResilienceFaultClassificationV1,
{
    let state = rate_limit_state_for_policy(current_state, policy, now_ms);
    let Some(rate_limit_policy) = policy.rate_limit.as_ref() else {
        let result = operation();
        let state = rate_limit_initial_state(policy, now_ms);
        let decision_summary = format!("{operation_ref}: rate limit disabled; operation completed");
        let telemetry_event = Some(rate_limit_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            None,
        ));
        return RateLimitExecution {
            result,
            state: state.clone(),
            trace: RateLimitExecutionTraceV1 {
                schema_version: RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                final_status: RateLimitFinalStatusV1::Allowed,
                window_started_at_ms: state.window_started_at_ms,
                requests_in_window_before: 0,
                requests_in_window_after: 0,
                max_requests: 0,
                window_ms: 0,
                operation_executed: true,
                wait_ms: None,
                decision_summary,
                fault: None,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    };

    let mut normalized_state = rate_limit_state_for_now(&state, rate_limit_policy, now_ms);
    let requests_before = normalized_state.requests_in_window;
    if normalized_state.requests_in_window < rate_limit_policy.max_requests {
        normalized_state.requests_in_window = normalized_state.requests_in_window.saturating_add(1);
        let result = operation();
        let decision_summary = format!(
            "{operation_ref}: rate limit allowed {}/{} in {}ms window",
            normalized_state.requests_in_window,
            rate_limit_policy.max_requests,
            rate_limit_policy.window_ms
        );
        let telemetry_event = Some(rate_limit_decision_event(
            policy,
            surface.clone(),
            operation_ref,
            &decision_summary,
            None,
        ));
        return RateLimitExecution {
            result,
            state: normalized_state.clone(),
            trace: RateLimitExecutionTraceV1 {
                schema_version: RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
                policy_id: policy.policy_id.clone(),
                surface,
                final_status: RateLimitFinalStatusV1::Allowed,
                window_started_at_ms: normalized_state.window_started_at_ms,
                requests_in_window_before: requests_before,
                requests_in_window_after: normalized_state.requests_in_window,
                max_requests: rate_limit_policy.max_requests,
                window_ms: rate_limit_policy.window_ms,
                operation_executed: true,
                wait_ms: None,
                decision_summary,
                fault: None,
                telemetry_event,
                recovery_artifact: None,
            },
        };
    }

    let wait_ms = rate_limit_wait_ms(&normalized_state, rate_limit_policy, now_ms);
    let error = rejection_error(&normalized_state, wait_ms);
    let fault = classify_error(&error);
    let decision_summary = format!(
        "{operation_ref}: rate limited at {}/{} requests; wait {}ms for window refill",
        normalized_state.requests_in_window, rate_limit_policy.max_requests, wait_ms
    );
    let telemetry_event = Some(rate_limit_decision_event(
        policy,
        surface.clone(),
        operation_ref,
        &decision_summary,
        Some(fault.clone()),
    ));
    let recovery_artifact = Some(rate_limit_recovery_artifact(
        policy,
        surface.clone(),
        operation_ref,
        &fault,
        wait_ms,
    ));
    RateLimitExecution {
        result: Err(error),
        state: normalized_state.clone(),
        trace: RateLimitExecutionTraceV1 {
            schema_version: RESILIENCE_RATE_LIMIT_EXECUTION_TRACE_SCHEMA_V1.to_string(),
            policy_id: policy.policy_id.clone(),
            surface,
            final_status: RateLimitFinalStatusV1::Throttled,
            window_started_at_ms: normalized_state.window_started_at_ms,
            requests_in_window_before: requests_before,
            requests_in_window_after: normalized_state.requests_in_window,
            max_requests: rate_limit_policy.max_requests,
            window_ms: rate_limit_policy.window_ms,
            operation_executed: false,
            wait_ms: Some(wait_ms),
            decision_summary,
            fault: Some(fault),
            telemetry_event,
            recovery_artifact,
        },
    }
}

fn rate_limit_state_for_policy(
    state: &RateLimitStateV1,
    policy: &ResiliencePolicyV1,
    now_ms: u64,
) -> RateLimitStateV1 {
    if state.policy_id == policy.policy_id {
        state.clone()
    } else {
        rate_limit_initial_state(policy, now_ms)
    }
}

fn rate_limit_state_for_now(
    state: &RateLimitStateV1,
    policy: &RateLimitPolicyV1,
    now_ms: u64,
) -> RateLimitStateV1 {
    if now_ms.saturating_sub(state.window_started_at_ms) < policy.window_ms {
        return state.clone();
    }
    RateLimitStateV1 {
        schema_version: state.schema_version.clone(),
        policy_id: state.policy_id.clone(),
        window_started_at_ms: now_ms,
        requests_in_window: 0,
    }
}

fn rate_limit_wait_ms(state: &RateLimitStateV1, policy: &RateLimitPolicyV1, now_ms: u64) -> u64 {
    policy
        .window_ms
        .saturating_sub(now_ms.saturating_sub(state.window_started_at_ms))
}

fn rate_limit_decision_event(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    decision_summary: &str,
    fault: Option<ResilienceFaultClassificationV1>,
) -> ResilienceTelemetryEventV1 {
    let correlation_suffix = rate_limit_execution_correlation_suffix();
    ResilienceTelemetryEventV1 {
        schema_version: RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1.to_string(),
        event_id: format!(
            "{}:rate-limit:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        event_kind: TelemetryEventKindV1::RateLimitDecision,
        surface,
        decision_summary: decision_summary.to_string(),
        run_id: None,
        request_id: None,
        policy_ref: Some(policy.policy_id.clone()),
        fault,
        artifact_ref: None,
    }
}

fn rate_limit_recovery_artifact(
    policy: &ResiliencePolicyV1,
    surface: ResilienceSurfaceV1,
    operation_ref: &str,
    fault: &ResilienceFaultClassificationV1,
    wait_ms: u64,
) -> RecoveryArtifactV1 {
    let correlation_suffix = rate_limit_execution_correlation_suffix();
    RecoveryArtifactV1 {
        schema_version: RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1.to_string(),
        artifact_id: format!(
            "{}:rate-limit:{operation_ref}:{correlation_suffix}",
            policy.policy_id
        ),
        surface,
        triggering_fault: fault.clone(),
        disposition: RecoveryDispositionV1::RetryAllowed,
        next_action: format!(
            "respect rate limit throttle by waiting at least {wait_ms}ms before retrying"
        ),
        source_run_id: None,
        checkpoint_ref: None,
        evidence_refs: vec![policy.policy_id.clone()],
    }
}

fn rate_limit_execution_correlation_suffix() -> String {
    RATE_LIMIT_EXECUTION_COUNTER
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1)
        .to_string()
}
