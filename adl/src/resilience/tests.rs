use super::circuit_breaker::{
    circuit_breaker_decision_event, circuit_breaker_recovery_artifact,
    circuit_breaker_state_for_now,
};
use super::fault::sanitize_resilience_summary;
use super::timeout::{
    classification_represents_timeout, timeout_breach, timeout_breach_label,
    timeout_deadline_fault, timeout_decision_event, timeout_recovery_artifact,
};
use super::*;
use serde_json::Value;
use std::cell::{Cell, RefCell};

fn clone_fault_classification(
    error: &ResilienceFaultClassificationV1,
) -> ResilienceFaultClassificationV1 {
    error.clone()
}

fn workflow_timeout_fault(
    breach_kind: TimeoutBreachKindV1,
    elapsed_ms: u64,
    budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Workflow,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{} exceeded at {elapsed_ms}/{budget_ms}",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn provider_timeout_fault(
    breach_kind: TimeoutBreachKindV1,
    elapsed_ms: u64,
    budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::ProviderTimeout,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{} exceeded at {elapsed_ms}/{budget_ms}",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn tool_timeout_fault(
    breach_kind: TimeoutBreachKindV1,
    elapsed_ms: u64,
    budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Tool,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{} exceeded at {elapsed_ms}/{budget_ms}",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn runtime_timeout_fault(
    breach_kind: TimeoutBreachKindV1,
    elapsed_ms: u64,
    budget_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Runtime,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "{} exceeded at {elapsed_ms}/{budget_ms}",
            timeout_breach_label(&breach_kind)
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn tool_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Tool,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("cancelled at {elapsed_ms}"),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn workflow_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Workflow,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("cancelled at {elapsed_ms}"),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn provider_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("cancelled at {elapsed_ms}"),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn runtime_cancelled_fault(elapsed_ms: u64) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Runtime,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Terminal,
        retryable: false,
        summary: format!("cancelled at {elapsed_ms}"),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn provider_breaker_rejection(
    state: &CircuitBreakerStateV1,
    now_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::ProviderTimeout,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "breaker open at {} after {}ms",
            state.consecutive_failures, now_ms
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn provider_breaker_probe_rejection(
    state: &CircuitBreakerStateV1,
    now_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::ProviderTimeout,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "breaker probe rejected at {} after {}ms",
            state.half_open_attempts, now_ms
        ),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    }
}

fn provider_rate_limit_rejection(
    state: &RateLimitStateV1,
    wait_ms: u64,
) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::ProviderRateLimited,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "rate limited after {} request(s); wait {}ms",
            state.requests_in_window, wait_ms
        ),
        component_ref: None,
        http_status: Some(429),
        retry_after_ms: Some(wait_ms),
    }
}

fn provider_bulkhead_rejection(state: &BulkheadStateV1) -> ResilienceFaultClassificationV1 {
    ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Provider,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: format!(
            "bulkhead saturated for fault domain '{}' at {} in-flight",
            state.fault_domain, state.in_flight
        ),
        component_ref: Some(state.fault_domain.clone()),
        http_status: None,
        retry_after_ms: None,
    }
}

fn test_circuit_breaker_policy() -> ResiliencePolicyV1 {
    ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "breaker.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 3,
            backoff_ms: Some(25),
            jitter_ms: Some(5),
            max_elapsed_ms: None,
            retryable_fault_classes: vec![
                ResilienceFaultClassV1::ProviderTimeout,
                ResilienceFaultClassV1::ProviderTransientHttp,
            ],
        }),
        timeout: Some(TimeoutPolicyV1 {
            timeout_ms: 100,
            hard_deadline_ms: Some(150),
        }),
        circuit_breaker: Some(CircuitBreakerPolicyV1 {
            failure_threshold: 2,
            recovery_window_ms: 30,
            half_open_max_attempts: 1,
        }),
        rate_limit: None,
        bulkhead: None,
        fallback: Some(FallbackPolicyV1 {
            fallback_ref: "test.fallback".to_string(),
            activation_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
            marks_output_degraded: true,
        }),
        checkpoint_required: false,
        telemetry_required: true,
    }
}

fn test_bulkhead_policy(fault_domain: &str, max_concurrency: u32) -> ResiliencePolicyV1 {
    ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: format!("bulkhead.{fault_domain}"),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: Some(BulkheadPolicyV1 {
            fault_domain: fault_domain.to_string(),
            max_concurrency,
            max_queue_depth: None,
        }),
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    }
}

fn test_degraded_fallback_policy() -> ResiliencePolicyV1 {
    ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "fallback.policy".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(10),
            jitter_ms: Some(0),
            max_elapsed_ms: Some(20),
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
        }),
        timeout: Some(TimeoutPolicyV1 {
            timeout_ms: 50,
            hard_deadline_ms: Some(75),
        }),
        circuit_breaker: Some(CircuitBreakerPolicyV1 {
            failure_threshold: 2,
            recovery_window_ms: 30,
            half_open_max_attempts: 1,
        }),
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 100,
        }),
        bulkhead: None,
        fallback: Some(FallbackPolicyV1 {
            fallback_ref: "test.degraded".to_string(),
            activation_fault_classes: vec![
                ResilienceFaultClassV1::ProviderTimeout,
                ResilienceFaultClassV1::ProviderTransientHttp,
            ],
            marks_output_degraded: true,
        }),
        checkpoint_required: false,
        telemetry_required: true,
    }
}

fn test_alternate_route_policy() -> ResiliencePolicyV1 {
    let mut policy = test_degraded_fallback_policy();
    policy.policy_id = "alternate.route.policy".to_string();
    policy.fallback = Some(FallbackPolicyV1 {
        fallback_ref: "test.alternate".to_string(),
        activation_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
        marks_output_degraded: false,
    });
    policy
}

#[test]
fn provider_fault_classifier_emits_retryable_timeout() {
    let fault = ResilienceFaultClassificationV1::provider("provider timeout", None);
    assert_eq!(fault.fault_class, ResilienceFaultClassV1::ProviderTimeout);
    assert_eq!(fault.disposition, ResilienceFaultDispositionV1::Retryable);
    assert!(fault.retryable);
}

#[test]
fn provider_fault_classifier_emits_operator_gated_auth_missing() {
    let fault = ResilienceFaultClassificationV1::provider(
        "missing required environment variable OPENAI_API_KEY",
        None,
    );
    assert_eq!(
        fault.fault_class,
        ResilienceFaultClassV1::ProviderAuthMissing
    );
    assert_eq!(
        fault.disposition,
        ResilienceFaultDispositionV1::OperatorGated
    );
    assert!(!fault.retryable);
}

#[test]
fn provider_fault_classifier_covers_remaining_provider_fault_branches() {
    let cases = [
        (
            "provider rate limit exceeded",
            None,
            ResilienceFaultClassV1::ProviderRateLimited,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "provider timeout while waiting for upstream",
            None,
            ResilienceFaultClassV1::ProviderTimeout,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "billing blocked due to credit balance",
            None,
            ResilienceFaultClassV1::ProviderBillingBlocked,
            ResilienceFaultDispositionV1::OperatorGated,
            false,
        ),
        (
            "local_runtime_busy because this is a non-target model",
            None,
            ResilienceFaultClassV1::LocalRuntimeBusy,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "local_runtime_hung while stopping...",
            None,
            ResilienceFaultClassV1::LocalRuntimeHung,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "ollama not running: connection refused",
            None,
            ResilienceFaultClassV1::LocalRuntimeUnavailable,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "provider model not found",
            None,
            ResilienceFaultClassV1::ProviderModelUnavailable,
            ResilienceFaultDispositionV1::Terminal,
            false,
        ),
        (
            "empty provider response output",
            None,
            ResilienceFaultClassV1::ProviderEmptyTextOutput,
            ResilienceFaultDispositionV1::Terminal,
            false,
        ),
        (
            "upstream exploded",
            Some(503),
            ResilienceFaultClassV1::ProviderTransientHttp,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
        (
            "provider_internal_error",
            Some(418),
            ResilienceFaultClassV1::ProviderError,
            ResilienceFaultDispositionV1::Terminal,
            false,
        ),
        (
            "something ambiguous happened",
            None,
            ResilienceFaultClassV1::Unknown,
            ResilienceFaultDispositionV1::Retryable,
            true,
        ),
    ];

    for (note, http_status, expected_class, expected_disposition, expected_retryable) in cases {
        let fault = ResilienceFaultClassificationV1::provider(note, http_status);
        assert_eq!(fault.fault_class, expected_class, "{note}");
        assert_eq!(fault.disposition, expected_disposition, "{note}");
        assert_eq!(fault.retryable, expected_retryable, "{note}");
    }
}

#[test]
fn resilience_foundation_defaults_stay_wired_to_phase1_contract() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("provider_attempt_default", 3, 30_000);
    let retry = policy.retry.as_ref().expect("retry policy");
    let timeout = policy.timeout.as_ref().expect("timeout policy");
    assert_eq!(policy.schema_version, RESILIENCE_POLICY_SCHEMA_V1);
    assert_eq!(retry.max_attempts, 3);
    assert_eq!(retry.backoff_ms, None);
    assert_eq!(retry.jitter_ms, None);
    assert!(retry
        .retryable_fault_classes
        .contains(&ResilienceFaultClassV1::ProviderRateLimited));
    assert!(retry
        .retryable_fault_classes
        .contains(&ResilienceFaultClassV1::LocalRuntimeHung));
    assert_eq!(timeout.timeout_ms, 30_000);
    assert_eq!(timeout.hard_deadline_ms, None);
    assert!(policy.circuit_breaker.is_none());
    assert!(policy.rate_limit.is_none());
    assert!(policy.bulkhead.is_none());
    assert!(policy.fallback.is_none());
    assert!(!policy.checkpoint_required);
    assert!(policy.telemetry_required);

    let manifest = ResilienceSubstrateManifestV1::phase1_foundation();
    assert_eq!(manifest.schema_version, RESILIENCE_SUBSTRATE_SCHEMA_V1);
    assert_eq!(
        manifest.supported_surfaces,
        vec![
            ResilienceSurfaceV1::Provider,
            ResilienceSurfaceV1::Tool,
            ResilienceSurfaceV1::Workflow,
            ResilienceSurfaceV1::CitizenRuntime,
        ]
    );
    assert_eq!(
        manifest.policy,
        ResiliencePolicyV1::provider_attempt_policy("provider_attempt_default", 3, 30_000)
    );
}

#[test]
fn phase1_manifest_references_all_required_schema_surfaces() {
    let manifest = ResilienceSubstrateManifestV1::phase1_foundation();
    assert_eq!(manifest.schema_version, RESILIENCE_SUBSTRATE_SCHEMA_V1);
    assert_eq!(
        manifest.fault_schema_ref,
        RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1
    );
    assert_eq!(
        manifest.citizen_health_schema_ref,
        RESILIENCE_CITIZEN_HEALTH_SCHEMA_V1
    );
    assert_eq!(
        manifest.recovery_artifact_schema_ref,
        RESILIENCE_RECOVERY_ARTIFACT_SCHEMA_V1
    );
    assert_eq!(
        manifest.checkpoint_schema_ref,
        RESILIENCE_CHECKPOINT_SCHEMA_V1
    );
    assert_eq!(
        manifest.telemetry_schema_ref,
        RESILIENCE_TELEMETRY_EVENT_SCHEMA_V1
    );
    assert!(manifest
        .supported_surfaces
        .contains(&ResilienceSurfaceV1::CitizenRuntime));
}

#[test]
fn schema_smoke_contains_manifest_title() {
    let schema = resilience_schema_smoke();
    let title = schema
        .get("title")
        .and_then(Value::as_str)
        .unwrap_or_default();
    assert_eq!(title, "ResilienceSubstrateManifestV1");
}

#[test]
fn provider_fault_summary_redacts_secret_like_content() {
    let classification = ResilienceFaultClassificationV1::provider(
        "request failed with key=super-secret-token prompt: send money",
        None,
    );
    assert_eq!(classification.summary, "redacted provider diagnostic");
    assert!(!classification.summary.contains("super-secret-token"));
}

#[test]
fn provider_fault_summary_normalizes_whitespace_and_truncates_long_messages() {
    let note = format!("{}\n  {}", "word ".repeat(40), "tail");
    let summary = sanitize_resilience_summary(&note);
    assert!(!summary.contains('\n'));
    assert!(!summary.contains("  "));
    assert!(summary.ends_with("..."));
    assert_eq!(summary.chars().count(), 180);
}

#[test]
fn fault_classification_round_trips_with_snake_case_schema_values() {
    let classification = ResilienceFaultClassificationV1::provider("provider timeout", Some(504));
    let json = serde_json::to_value(&classification).expect("serialize classification");
    assert_eq!(json["surface"], "provider");
    assert_eq!(json["fault_class"], "provider_timeout");
    assert_eq!(json["disposition"], "retryable");
    let reparsed: ResilienceFaultClassificationV1 =
        serde_json::from_value(json).expect("round trip classification");
    assert_eq!(reparsed, classification);
}

#[test]
fn execute_timeout_policy_succeeds_before_deadline() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.success", 1, 100);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.timeout.success",
        || TimeoutObservation {
            result: Ok("ok"),
            elapsed_ms: 40,
            cancelled: false,
        },
        clone_fault_classification,
        tool_timeout_fault,
        tool_cancelled_fault,
    );

    assert_eq!(execution.result.expect("success"), "ok");
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::Succeeded
    );
    assert_eq!(execution.trace.timeout_ms, Some(100));
    assert_eq!(execution.trace.hard_deadline_ms, None);
    assert!(execution.trace.recovery_artifact.is_none());
}

#[test]
fn execute_timeout_policy_emits_timeout_artifact_when_timeout_budget_is_exceeded() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.deadline", 1, 50);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.deadline",
        || TimeoutObservation {
            result: Ok::<(), ResilienceFaultClassificationV1>(()),
            elapsed_ms: 75,
            cancelled: false,
        },
        clone_fault_classification,
        workflow_timeout_fault,
        workflow_cancelled_fault,
    );

    let failure = execution.result.expect_err("timeout failure");
    assert!(failure.retryable);
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::TimedOut
    );
    assert_eq!(
        execution.trace.breach_kind,
        Some(TimeoutBreachKindV1::Timeout)
    );
    assert_eq!(
        execution.trace.schema_version,
        RESILIENCE_TIMEOUT_EXECUTION_TRACE_SCHEMA_V1
    );
    assert_eq!(
        execution
            .trace
            .telemetry_event
            .as_ref()
            .map(|event| event.event_kind.clone()),
        Some(TelemetryEventKindV1::TimeoutDecision)
    );
    assert_eq!(
        execution
            .trace
            .recovery_artifact
            .as_ref()
            .map(|artifact| artifact.disposition.clone()),
        Some(RecoveryDispositionV1::RetryAllowed)
    );
}

#[test]
fn execute_timeout_policy_distinguishes_timeout_budget_from_hard_deadline() {
    let mut policy = ResiliencePolicyV1::provider_attempt_policy("timeout.budgets", 1, 50);
    policy
        .timeout
        .as_mut()
        .expect("timeout policy")
        .hard_deadline_ms = Some(90);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.budgets",
        || TimeoutObservation {
            result: Ok::<(), ResilienceFaultClassificationV1>(()),
            elapsed_ms: 60,
            cancelled: false,
        },
        clone_fault_classification,
        workflow_timeout_fault,
        workflow_cancelled_fault,
    );

    assert!(execution.result.is_err());
    assert_eq!(
        execution.trace.breach_kind,
        Some(TimeoutBreachKindV1::Timeout)
    );
    assert_eq!(execution.trace.timeout_ms, Some(50));
    assert_eq!(execution.trace.hard_deadline_ms, Some(90));
}

#[test]
fn execute_timeout_policy_emits_hard_deadline_breach_when_timeout_budget_is_absent() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "timeout.deadline-only".to_string(),
        retry: None,
        timeout: Some(TimeoutPolicyV1 {
            timeout_ms: 120,
            hard_deadline_ms: Some(90),
        }),
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Runtime,
        "test.timeout.deadline-only",
        || TimeoutObservation {
            result: Ok::<(), ResilienceFaultClassificationV1>(()),
            elapsed_ms: 100,
            cancelled: false,
        },
        clone_fault_classification,
        runtime_timeout_fault,
        runtime_cancelled_fault,
    );

    assert!(execution.result.is_err());
    assert_eq!(
        execution.trace.breach_kind,
        Some(TimeoutBreachKindV1::HardDeadline)
    );
    assert!(execution
        .trace
        .decision_summary
        .contains("hard deadline exceeded"));
}

#[test]
fn execute_timeout_policy_distinguishes_timeout_from_terminal_business_failure() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.failure", 1, 100);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.timeout.failure",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1::provider(
                "provider invalid api key",
                Some(401),
            )),
            elapsed_ms: 20,
            cancelled: false,
        },
        clone_fault_classification,
        provider_timeout_fault,
        provider_cancelled_fault,
    );

    let failure = execution.result.expect_err("terminal failure");
    assert_eq!(
        failure.fault_class,
        ResilienceFaultClassV1::ProviderAuthError
    );
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::Failed
    );
    assert!(execution.trace.recovery_artifact.is_none());
}

#[test]
fn execute_timeout_policy_keeps_late_terminal_errors_terminal() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.late-failure", 1, 50);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.timeout.late-failure",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1::provider(
                "provider invalid api key",
                Some(401),
            )),
            elapsed_ms: 80,
            cancelled: false,
        },
        clone_fault_classification,
        provider_timeout_fault,
        provider_cancelled_fault,
    );

    let failure = execution.result.expect_err("terminal failure");
    assert_eq!(
        failure.fault_class,
        ResilienceFaultClassV1::ProviderAuthError
    );
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::Failed
    );
    assert!(execution.trace.decision_summary.contains("failed after"));
    assert!(execution.trace.recovery_artifact.is_none());
}

#[test]
fn execute_timeout_policy_recognizes_generic_timeout_failures() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.generic", 1, 100);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.timeout.generic",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1 {
                schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                surface: ResilienceSurfaceV1::Tool,
                fault_class: ResilienceFaultClassV1::RuntimeFailure,
                disposition: ResilienceFaultDispositionV1::Retryable,
                retryable: true,
                summary: "tool timeout while waiting for child process".to_string(),
                component_ref: None,
                http_status: None,
                retry_after_ms: None,
            }),
            elapsed_ms: 105,
            cancelled: false,
        },
        clone_fault_classification,
        tool_timeout_fault,
        tool_cancelled_fault,
    );

    let failure = execution.result.expect_err("generic timeout failure");
    assert!(failure.retryable);
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::TimedOut
    );
    assert_eq!(
        execution.trace.breach_kind,
        Some(TimeoutBreachKindV1::Timeout)
    );
    assert!(execution.trace.recovery_artifact.is_some());
}

#[test]
fn execute_timeout_policy_handles_timeout_classification_without_budget_breach() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "timeout.classified-only".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.timeout.classified-only",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1 {
                schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                surface: ResilienceSurfaceV1::Tool,
                fault_class: ResilienceFaultClassV1::Unknown,
                disposition: ResilienceFaultDispositionV1::Retryable,
                retryable: true,
                summary: "operation timed out without explicit timeout budget".to_string(),
                component_ref: None,
                http_status: None,
                retry_after_ms: None,
            }),
            elapsed_ms: 45,
            cancelled: false,
        },
        clone_fault_classification,
        tool_timeout_fault,
        tool_cancelled_fault,
    );

    assert!(execution.result.is_err());
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::TimedOut
    );
    assert_eq!(execution.trace.breach_kind, None);
    assert!(execution
        .trace
        .decision_summary
        .contains("timeout failure after 45ms"));
    assert!(execution.trace.recovery_artifact.is_some());
}

#[test]
fn execute_timeout_policy_marks_cancellation_as_cancelled_not_success() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.cancel", 1, 100);
    let execution = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.cancel",
        || TimeoutObservation::<(), ResilienceFaultClassificationV1> {
            result: Err(ResilienceFaultClassificationV1 {
                schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
                surface: ResilienceSurfaceV1::Workflow,
                fault_class: ResilienceFaultClassV1::WorkflowFailure,
                disposition: ResilienceFaultDispositionV1::Terminal,
                retryable: false,
                summary: "cancelled".to_string(),
                component_ref: None,
                http_status: None,
                retry_after_ms: None,
            }),
            elapsed_ms: 15,
            cancelled: true,
        },
        clone_fault_classification,
        workflow_timeout_fault,
        workflow_cancelled_fault,
    );

    let failure = execution.result.expect_err("cancelled result");
    assert_eq!(failure.summary, "cancelled at 15");
    assert_eq!(
        execution.trace.final_status,
        TimeoutExecutionFinalStatusV1::Cancelled
    );
    assert_eq!(
        execution
            .trace
            .recovery_artifact
            .as_ref()
            .map(|artifact| artifact.disposition.clone()),
        Some(RecoveryDispositionV1::ResumeAllowed)
    );
}

#[test]
fn timeout_event_and_artifact_ids_remain_unique_across_repeated_emissions() {
    let policy = ResiliencePolicyV1::provider_attempt_policy("timeout.ids", 1, 10);
    let fault = timeout_deadline_fault(
        ResilienceSurfaceV1::Workflow,
        "test.timeout.ids",
        12,
        TimeoutBreachKindV1::Timeout,
        10,
    );
    let first_event = timeout_decision_event(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.ids",
        "first timeout",
        Some(fault.clone()),
    );
    let second_event = timeout_decision_event(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.ids",
        "second timeout",
        Some(fault.clone()),
    );
    let first_artifact = timeout_recovery_artifact(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.ids",
        &fault,
        RecoveryDispositionV1::RetryAllowed,
        "retry",
    );
    let second_artifact = timeout_recovery_artifact(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.timeout.ids",
        &fault,
        RecoveryDispositionV1::RetryAllowed,
        "retry",
    );

    assert_ne!(first_event.event_id, second_event.event_id);
    assert_ne!(first_artifact.artifact_id, second_artifact.artifact_id);
}

#[test]
fn timeout_helper_functions_cover_remaining_branch_cases() {
    assert_eq!(timeout_breach(None, None, 10), None);
    assert_eq!(
        timeout_breach(Some(50), Some(90), 95),
        Some((TimeoutBreachKindV1::Timeout, 50))
    );
    assert_eq!(
        timeout_breach(Some(120), Some(90), 100),
        Some((TimeoutBreachKindV1::HardDeadline, 90))
    );
    assert_eq!(
        timeout_breach_label(&TimeoutBreachKindV1::Timeout),
        "timeout budget"
    );
    assert_eq!(
        timeout_breach_label(&TimeoutBreachKindV1::HardDeadline),
        "hard deadline"
    );

    let provider_timeout = ResilienceFaultClassificationV1::provider("provider timeout", None);
    assert!(classification_represents_timeout(&provider_timeout));

    let runtime_deadline = ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Runtime,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: "deadline elapsed while waiting".to_string(),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    };
    assert!(classification_represents_timeout(&runtime_deadline));

    let runtime_non_timeout = ResilienceFaultClassificationV1 {
        schema_version: RESILIENCE_FAULT_CLASSIFICATION_SCHEMA_V1.to_string(),
        surface: ResilienceSurfaceV1::Runtime,
        fault_class: ResilienceFaultClassV1::RuntimeFailure,
        disposition: ResilienceFaultDispositionV1::Retryable,
        retryable: true,
        summary: "worker exited with code 2".to_string(),
        component_ref: None,
        http_status: None,
        retry_after_ms: None,
    };
    assert!(!classification_represents_timeout(&runtime_non_timeout));

    let provider_error =
        ResilienceFaultClassificationV1::provider("provider_internal_error", Some(500));
    assert!(!classification_represents_timeout(&provider_error));
}

#[test]
fn retry_policy_delay_is_deterministic_and_bounded_by_jitter() {
    let retry = RetryPolicyV1 {
        max_attempts: 3,
        backoff_ms: Some(100),
        jitter_ms: Some(25),
        max_elapsed_ms: None,
        retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
    };
    let first = retry.next_delay_ms("policy.retry", 1);
    let second = retry.next_delay_ms("policy.retry", 1);
    let third_attempt = retry.next_delay_ms("policy.retry", 3);
    assert_eq!(first, second);
    assert!((100..=125).contains(&first));
    assert!((400..=425).contains(&third_attempt));
}

#[test]
fn execute_retry_policy_retries_and_emits_trace() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "retry.trace".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 3,
            backoff_ms: Some(5),
            jitter_ms: Some(0),
            max_elapsed_ms: None,
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
        }),
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let mut attempts = Vec::new();
    let mut sleeps = Vec::new();
    let mut observed = Vec::new();
    let execution = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.retry",
        |attempt_index| {
            attempts.push(attempt_index);
            if attempt_index < 3 {
                Err(ResilienceFaultClassificationV1::provider(
                    "provider timeout",
                    Some(504),
                ))
            } else {
                Ok("ok")
            }
        },
        |error| error.clone(),
        |delay_ms| sleeps.push(delay_ms),
        |record| observed.push(record.clone()),
    );
    assert_eq!(execution.result.expect("final success"), "ok");
    assert_eq!(attempts, vec![1, 2, 3]);
    assert_eq!(sleeps, vec![5, 10]);
    assert_eq!(observed.len(), 3);
    assert_eq!(
        execution.trace.schema_version,
        RESILIENCE_RETRY_EXECUTION_TRACE_SCHEMA_V1
    );
    assert!(execution
        .trace
        .attempts
        .iter()
        .all(|attempt| attempt.schema_version == RESILIENCE_RETRY_ATTEMPT_SCHEMA_V1));
    assert_eq!(execution.trace.telemetry_events.len(), 3);
    assert_eq!(
        execution.trace.final_status,
        RetryExecutionFinalStatusV1::Succeeded
    );
    assert!(execution.trace.recovery_artifact.is_none());
}

#[test]
fn execute_retry_policy_emits_recovery_artifact_when_budget_exhausts() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "retry.exhausted".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(1),
            jitter_ms: Some(0),
            max_elapsed_ms: None,
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderTransientHttp],
        }),
        timeout: None,
        circuit_breaker: None,
        rate_limit: None,
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let execution: RetryExecution<(), ResilienceFaultClassificationV1> = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.retry.exhausted",
        |_| {
            Err(ResilienceFaultClassificationV1::provider(
                "server 503",
                Some(503),
            ))
        },
        |error| error.clone(),
        |_| {},
        |_| {},
    );
    let failure = execution.result.expect_err("final failure");
    assert_eq!(
        failure.fault_class,
        ResilienceFaultClassV1::ProviderTransientHttp
    );
    assert_eq!(execution.trace.attempts.len(), 2);
    let recovery = execution
        .trace
        .recovery_artifact
        .expect("recovery artifact");
    assert_eq!(
        recovery.disposition,
        RecoveryDispositionV1::OperatorInterventionRequired
    );
    assert!(recovery.next_action.contains("retry budget exhausted"));
}

#[test]
fn timeout_fault_builder_helpers_cover_all_remaining_surfaces() {
    let tool_timeout = tool_timeout_fault(TimeoutBreachKindV1::Timeout, 12, 10);
    assert_eq!(tool_timeout.surface, ResilienceSurfaceV1::Tool);
    assert!(tool_timeout.retryable);

    let provider_timeout = provider_timeout_fault(TimeoutBreachKindV1::HardDeadline, 22, 20);
    assert_eq!(provider_timeout.surface, ResilienceSurfaceV1::Provider);
    assert!(provider_timeout.summary.contains("hard deadline"));

    let tool_cancel = tool_cancelled_fault(13);
    assert_eq!(tool_cancel.surface, ResilienceSurfaceV1::Tool);
    assert!(!tool_cancel.retryable);

    let provider_cancel = provider_cancelled_fault(14);
    assert_eq!(provider_cancel.surface, ResilienceSurfaceV1::Provider);
    assert!(provider_cancel.summary.contains("cancelled"));

    let runtime_cancel = runtime_cancelled_fault(15);
    assert_eq!(runtime_cancel.surface, ResilienceSurfaceV1::Runtime);
    assert!(runtime_cancel.summary.contains("15"));
}

#[test]
fn circuit_breaker_trips_open_and_rejects_follow_up_calls() {
    let policy = test_circuit_breaker_policy();
    let first = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.trip",
        &circuit_breaker_initial_state(&policy),
        10,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(first.result.is_err());
    assert_eq!(first.state.state, CircuitBreakerStateKindV1::Closed);
    assert_eq!(first.state.consecutive_failures, 1);

    let second = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.trip",
        &first.state,
        20,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(second.result.is_err());
    assert_eq!(second.state.state, CircuitBreakerStateKindV1::Open);
    assert_eq!(second.state.consecutive_failures, 2);
    assert!(second.trace.recovery_artifact.is_some());

    let called = Cell::new(0);
    let third = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.trip",
        &second.state,
        25,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert_eq!(called.get(), 0);
    assert!(third.result.is_err());
    assert_eq!(
        third.trace.final_status,
        CircuitBreakerFinalStatusV1::OpenRejected
    );
    assert!(!third.trace.operation_executed);
}

#[test]
fn circuit_breaker_uses_fallback_when_open() {
    let policy = test_circuit_breaker_policy();
    let open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Open,
        consecutive_failures: 2,
        half_open_attempts: 0,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let called = Cell::new(0);
    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.breaker.fallback",
        &open_state,
        20,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("primary")
        },
        clone_fault_classification,
        provider_breaker_rejection,
        Some(|| "fallback"),
    );

    assert_eq!(called.get(), 0);
    assert_eq!(execution.result.expect("fallback result"), "fallback");
    assert_eq!(
        execution.trace.final_status,
        CircuitBreakerFinalStatusV1::OpenFallback
    );
    assert!(execution.trace.used_fallback);
    assert!(execution.trace.recovery_artifact.is_some());
}

#[test]
fn fallback_policy_returns_primary_success_without_degraded_marking() {
    let policy = test_degraded_fallback_policy();
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.fallback.primary",
        || Ok::<_, ResilienceFaultClassificationV1>("primary"),
        clone_fault_classification,
        Some(|| "fallback"),
    );

    assert_eq!(execution.result.expect("primary result"), "primary");
    assert_eq!(execution.outcome_kind, FallbackOutcomeKindV1::Primary);
    assert_eq!(
        execution.trace.final_status,
        FallbackExecutionFinalStatusV1::PrimarySuccess
    );
    assert!(!execution.trace.fallback_executed);
    assert!(!execution.trace.output_degraded);
}

#[test]
fn fallback_policy_marks_degraded_output_explicitly() {
    let policy = test_degraded_fallback_policy();
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.fallback.degraded",
        || {
            Err::<&'static str, _>(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        Some(|| "degraded-result"),
    );

    assert_eq!(
        execution.result.expect("degraded fallback result"),
        "degraded-result"
    );
    assert_eq!(execution.outcome_kind, FallbackOutcomeKindV1::Degraded);
    assert_eq!(
        execution.trace.final_status,
        FallbackExecutionFinalStatusV1::DegradedSuccess
    );
    assert!(execution.trace.fallback_executed);
    assert!(execution.trace.output_degraded);
    assert_eq!(
        execution.trace.fallback_ref.as_deref(),
        Some("test.degraded")
    );
    assert!(execution.trace.recovery_artifact.is_some());
}

#[test]
fn fallback_policy_supports_explicit_alternate_route_without_degraded_flag() {
    let policy = test_alternate_route_policy();
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.fallback.alternate",
        || {
            Err::<&'static str, _>(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        Some(|| "alternate-result"),
    );

    assert_eq!(
        execution.result.expect("alternate fallback result"),
        "alternate-result"
    );
    assert_eq!(
        execution.outcome_kind,
        FallbackOutcomeKindV1::AlternateRoute
    );
    assert_eq!(
        execution.trace.final_status,
        FallbackExecutionFinalStatusV1::AlternateRouteSuccess
    );
    assert!(execution.trace.fallback_executed);
    assert!(!execution.trace.output_degraded);
}

#[test]
fn fallback_policy_preserves_primary_failure_when_no_hook_exists() {
    let policy = test_degraded_fallback_policy();
    let execution = execute_fallback_policy::<&'static str, _, _, _, fn() -> &'static str>(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.fallback.unavailable",
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        None,
    );

    assert!(execution.result.is_err());
    assert_eq!(execution.outcome_kind, FallbackOutcomeKindV1::Primary);
    assert_eq!(
        execution.trace.final_status,
        FallbackExecutionFinalStatusV1::FallbackUnavailable
    );
    assert!(!execution.trace.fallback_executed);
}

#[test]
fn fallback_policy_does_not_activate_for_non_matching_faults() {
    let policy = test_degraded_fallback_policy();
    let execution = execute_fallback_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.fallback.miss",
        || {
            Err::<&'static str, _>(ResilienceFaultClassificationV1::provider(
                "billing blocked",
                None,
            ))
        },
        clone_fault_classification,
        Some(|| "should-not-run"),
    );

    assert!(execution.result.is_err());
    assert_eq!(
        execution.trace.final_status,
        FallbackExecutionFinalStatusV1::PrimaryFailure
    );
    assert!(!execution.trace.fallback_executed);
    assert_eq!(execution.outcome_kind, FallbackOutcomeKindV1::Primary);
}

#[test]
fn representative_provider_flow_composes_retry_rate_limit_timeout_breaker_and_fallback() {
    let mut policy = test_degraded_fallback_policy();
    policy.policy_id = "phase1.representative.provider".to_string();
    policy.retry = Some(RetryPolicyV1 {
        max_attempts: 4,
        backoff_ms: Some(10),
        jitter_ms: Some(0),
        max_elapsed_ms: Some(1_000),
        retryable_fault_classes: vec![
            ResilienceFaultClassV1::ProviderTimeout,
            ResilienceFaultClassV1::ProviderRateLimited,
        ],
    });
    policy.fallback = Some(FallbackPolicyV1 {
        fallback_ref: "test.phase1.degraded".to_string(),
        activation_fault_classes: vec![ResilienceFaultClassV1::ProviderTimeout],
        marks_output_degraded: true,
    });
    if let Some(circuit_breaker) = policy.circuit_breaker.as_mut() {
        circuit_breaker.recovery_window_ms = 500;
    }

    let rate_state = RefCell::new(rate_limit_initial_state(&policy, 0));
    let breaker_state = RefCell::new(circuit_breaker_initial_state(&policy));
    let now_ms = Cell::new(0_u64);
    let sleep_count = Cell::new(0_u32);
    let mut sleeps = Vec::new();
    let final_breaker_trace = RefCell::new(None);

    let execution = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.phase1.provider-flow",
        |attempt_index| {
            let limited = execute_rate_limit_policy(
                &policy,
                ResilienceSurfaceV1::Provider,
                "test.phase1.provider-flow.rate-limit",
                &rate_state.borrow().clone(),
                now_ms.get(),
                || Ok::<_, ResilienceFaultClassificationV1>(()),
                provider_rate_limit_rejection,
                clone_fault_classification,
            );
            *rate_state.borrow_mut() = limited.state.clone();
            limited.result?;

            let breaker = execute_circuit_breaker_policy(
                &policy,
                ResilienceSurfaceV1::Provider,
                "test.phase1.provider-flow.breaker",
                &breaker_state.borrow().clone(),
                now_ms.get(),
                || {
                    let timeout = execute_timeout_policy(
                        &policy,
                        ResilienceSurfaceV1::Provider,
                        "test.phase1.provider-flow.timeout",
                        || match attempt_index {
                            1 | 3 => TimeoutObservation {
                                result: Ok::<_, ResilienceFaultClassificationV1>("late"),
                                elapsed_ms: 125,
                                cancelled: false,
                            },
                            _ => TimeoutObservation {
                                result: Ok::<_, ResilienceFaultClassificationV1>("fast"),
                                elapsed_ms: 5,
                                cancelled: false,
                            },
                        },
                        clone_fault_classification,
                        provider_timeout_fault,
                        provider_cancelled_fault,
                    );
                    timeout.result
                },
                clone_fault_classification,
                provider_breaker_rejection,
                Some(|| "degraded-answer"),
            );
            *breaker_state.borrow_mut() = breaker.state.clone();
            *final_breaker_trace.borrow_mut() = Some(breaker.trace.clone());
            breaker.result
        },
        clone_fault_classification,
        |delay_ms| {
            sleeps.push(delay_ms);
            let next_count = sleep_count.get().saturating_add(1);
            sleep_count.set(next_count);
            let advance = if next_count == 1 {
                delay_ms
            } else {
                delay_ms.max(110)
            };
            now_ms.set(now_ms.get().saturating_add(advance));
        },
        |_| {},
    );

    assert_eq!(
        execution.result.expect("degraded fallback"),
        "degraded-answer"
    );
    assert_eq!(execution.trace.attempts.len(), 4);
    assert_eq!(
        execution.trace.attempts[0]
            .fault
            .as_ref()
            .map(|f| f.fault_class.clone()),
        Some(ResilienceFaultClassV1::ProviderTimeout)
    );
    assert_eq!(
        execution.trace.attempts[1]
            .fault
            .as_ref()
            .map(|f| f.fault_class.clone()),
        Some(ResilienceFaultClassV1::ProviderRateLimited)
    );
    assert_eq!(
        execution.trace.attempts[2]
            .fault
            .as_ref()
            .map(|f| f.fault_class.clone()),
        Some(ResilienceFaultClassV1::ProviderTimeout)
    );
    assert!(execution.trace.attempts[3].fault.is_none());
    assert_eq!(sleeps.len(), 3);
    assert_eq!(sleeps[0], 10);
    assert!(sleeps[1] >= 40);
    assert!(sleeps[2] >= 40);
    assert_eq!(
        breaker_state.borrow().state,
        CircuitBreakerStateKindV1::Open
    );

    let breaker_trace = final_breaker_trace
        .borrow()
        .clone()
        .expect("final breaker trace");
    assert_eq!(
        breaker_trace.final_status,
        CircuitBreakerFinalStatusV1::OpenFallback
    );
    assert!(breaker_trace.used_fallback);
    assert!(breaker_trace.recovery_artifact.is_some());
}

#[test]
fn circuit_breaker_allows_half_open_probe_and_closes_on_success() {
    let policy = test_circuit_breaker_policy();
    let open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Open,
        consecutive_failures: 2,
        half_open_attempts: 0,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.breaker.half-open-success",
        &open_state,
        50,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );

    assert_eq!(execution.result.expect("success"), "ok");
    assert_eq!(
        execution.trace.state_before,
        CircuitBreakerStateKindV1::Open
    );
    assert_eq!(
        execution.trace.state_after,
        CircuitBreakerStateKindV1::Closed
    );
    assert_eq!(
        execution.trace.final_status,
        CircuitBreakerFinalStatusV1::HalfOpenProbeSuccess
    );
    assert_eq!(execution.state.consecutive_failures, 0);
    assert!(execution.state.last_failure.is_none());
}

#[test]
fn circuit_breaker_reopens_after_failed_half_open_probe() {
    let policy = test_circuit_breaker_policy();
    let half_open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::HalfOpen,
        consecutive_failures: 2,
        half_open_attempts: 0,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.half-open-failure",
        &half_open_state,
        60,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );

    assert!(execution.result.is_err());
    assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Open);
    assert_eq!(
        execution.trace.final_status,
        CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
    );
    assert!(execution.trace.recovery_artifact.is_some());
}

#[test]
fn circuit_breaker_bounds_half_open_probe_attempts() {
    let policy = test_circuit_breaker_policy();
    let half_open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::HalfOpen,
        consecutive_failures: 2,
        half_open_attempts: 1,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let called = Cell::new(0);
    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.half-open-limit",
        &half_open_state,
        60,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
        },
        clone_fault_classification,
        provider_breaker_probe_rejection,
        None::<fn() -> &'static str>,
    );

    assert_eq!(called.get(), 0);
    assert!(execution.result.is_err());
    assert_eq!(
        execution.trace.final_status,
        CircuitBreakerFinalStatusV1::HalfOpenProbeRejected
    );
    assert_eq!(execution.state.state, CircuitBreakerStateKindV1::HalfOpen);
}

#[test]
fn circuit_breaker_honors_multi_probe_budget_before_reopening() {
    let mut policy = test_circuit_breaker_policy();
    policy
        .circuit_breaker
        .as_mut()
        .expect("breaker policy")
        .half_open_max_attempts = 2;
    let open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Open,
        consecutive_failures: 2,
        half_open_attempts: 0,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };

    let first_failure = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.multi-probe.first",
        &open_state,
        50,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(first_failure.result.is_err());
    assert_eq!(
        first_failure.trace.final_status,
        CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
    );
    assert_eq!(
        first_failure.state.state,
        CircuitBreakerStateKindV1::HalfOpen
    );
    assert_eq!(first_failure.state.half_open_attempts, 1);
    assert!(first_failure.state.opened_at_ms.is_none());

    let second_failure = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.multi-probe.second",
        &first_failure.state,
        55,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(second_failure.result.is_err());
    assert_eq!(
        second_failure.trace.final_status,
        CircuitBreakerFinalStatusV1::HalfOpenProbeFailure
    );
    assert_eq!(second_failure.state.state, CircuitBreakerStateKindV1::Open);
    assert_eq!(second_failure.state.half_open_attempts, 2);
    assert_eq!(second_failure.state.opened_at_ms, Some(55));
}

#[test]
fn circuit_breaker_resets_mismatched_policy_state() {
    let policy = test_circuit_breaker_policy();
    let stale_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: "stale.policy".to_string(),
        state: CircuitBreakerStateKindV1::Open,
        consecutive_failures: 7,
        half_open_attempts: 1,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };

    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.policy-reset",
        &stale_state,
        15,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert_eq!(execution.result.expect("success"), "ok");
    assert_eq!(
        execution.trace.state_before,
        CircuitBreakerStateKindV1::Closed
    );
    assert_eq!(execution.state.policy_id, policy.policy_id);
    assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Closed);
    assert_eq!(execution.state.consecutive_failures, 0);
}

#[test]
fn circuit_breaker_composes_timeout_faults_without_retry_storms() {
    let policy = test_circuit_breaker_policy();
    let first_timeout = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.compose.timeout",
        || TimeoutObservation {
            result: Ok::<_, ResilienceFaultClassificationV1>("late"),
            elapsed_ms: 125,
            cancelled: false,
        },
        clone_fault_classification,
        provider_timeout_fault,
        provider_cancelled_fault,
    );
    let first_fault = first_timeout.trace.fault.clone().expect("timeout fault");
    assert_eq!(
        first_fault.fault_class,
        ResilienceFaultClassV1::RuntimeFailure
    );

    let first_breaker = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.compose.first",
        &circuit_breaker_initial_state(&policy),
        10,
        || Err(first_fault.clone()),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(first_breaker.result.is_err());
    assert_eq!(first_breaker.state.state, CircuitBreakerStateKindV1::Closed);

    let second_timeout = execute_timeout_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.compose.timeout",
        || TimeoutObservation {
            result: Ok::<_, ResilienceFaultClassificationV1>("late"),
            elapsed_ms: 130,
            cancelled: false,
        },
        clone_fault_classification,
        provider_timeout_fault,
        provider_cancelled_fault,
    );
    let second_fault = second_timeout.trace.fault.clone().expect("timeout fault");

    let second_breaker = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.compose.second",
        &first_breaker.state,
        20,
        || Err(second_fault),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(second_breaker.result.is_err());
    assert_eq!(second_breaker.state.state, CircuitBreakerStateKindV1::Open);

    let called = Cell::new(0);
    let rejected = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.compose.third",
        &second_breaker.state,
        25,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert_eq!(called.get(), 0);
    assert!(rejected.result.is_err());
    assert_eq!(
        rejected.trace.final_status,
        CircuitBreakerFinalStatusV1::OpenRejected
    );
}

#[test]
fn circuit_breaker_disabled_path_reports_success_and_failure() {
    let policy = ResiliencePolicyV1 {
        circuit_breaker: None,
        ..test_circuit_breaker_policy()
    };
    let state = circuit_breaker_initial_state(&policy);

    let success = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.breaker.disabled.success",
        &state,
        5,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert_eq!(success.result.expect("success"), "ok");
    assert_eq!(
        success.trace.final_status,
        CircuitBreakerFinalStatusV1::ClosedSuccess
    );
    assert!(success.trace.decision_summary.contains("breaker disabled"));

    let failure = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.breaker.disabled.failure",
        &state,
        6,
        || {
            Err(ResilienceFaultClassificationV1::provider(
                "provider timeout",
                None,
            ))
        },
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );
    assert!(failure.result.is_err());
    assert_eq!(
        failure.trace.final_status,
        CircuitBreakerFinalStatusV1::ClosedFailure
    );
    assert!(failure.trace.fault.is_some());
}

#[test]
fn circuit_breaker_closed_success_resets_prior_failure_state() {
    let policy = test_circuit_breaker_policy();
    let prior_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Closed,
        consecutive_failures: 1,
        half_open_attempts: 0,
        opened_at_ms: None,
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let execution = execute_circuit_breaker_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.breaker.closed-success",
        &prior_state,
        40,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_breaker_rejection,
        None::<fn() -> &'static str>,
    );

    assert_eq!(execution.result.expect("success"), "ok");
    assert_eq!(
        execution.trace.final_status,
        CircuitBreakerFinalStatusV1::ClosedSuccess
    );
    assert_eq!(execution.state.consecutive_failures, 0);
    assert_eq!(execution.state.state, CircuitBreakerStateKindV1::Closed);
    assert!(execution.state.last_failure.is_none());
}

#[test]
fn circuit_breaker_helper_functions_cover_state_window_and_id_generation() {
    let policy = test_circuit_breaker_policy();
    let open_state = CircuitBreakerStateV1 {
        schema_version: RESILIENCE_CIRCUIT_BREAKER_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        state: CircuitBreakerStateKindV1::Open,
        consecutive_failures: 2,
        half_open_attempts: 0,
        opened_at_ms: Some(10),
        last_failure: Some(ResilienceFaultClassificationV1::provider(
            "provider timeout",
            None,
        )),
    };
    let still_open = circuit_breaker_state_for_now(
        &open_state,
        policy.circuit_breaker.as_ref().expect("breaker policy"),
        20,
    );
    assert_eq!(still_open.state, CircuitBreakerStateKindV1::Open);
    let half_open = circuit_breaker_state_for_now(
        &open_state,
        policy.circuit_breaker.as_ref().expect("breaker policy"),
        45,
    );
    assert_eq!(half_open.state, CircuitBreakerStateKindV1::HalfOpen);
    assert_eq!(half_open.half_open_attempts, 0);
    let unchanged = circuit_breaker_state_for_now(
        &circuit_breaker_initial_state(&policy),
        policy.circuit_breaker.as_ref().expect("breaker policy"),
        45,
    );
    assert_eq!(unchanged.state, CircuitBreakerStateKindV1::Closed);

    let fault = ResilienceFaultClassificationV1::provider("provider timeout", None);
    let first_event = circuit_breaker_decision_event(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.ids",
        "first",
        Some(fault.clone()),
    );
    let second_event = circuit_breaker_decision_event(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.ids",
        "second",
        Some(fault.clone()),
    );
    let first_artifact = circuit_breaker_recovery_artifact(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.ids",
        &fault,
        RecoveryDispositionV1::RetryAllowed,
        "retry later",
    );
    let second_artifact = circuit_breaker_recovery_artifact(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.breaker.ids",
        &fault,
        RecoveryDispositionV1::RetryAllowed,
        "retry later",
    );
    assert_ne!(first_event.event_id, second_event.event_id);
    assert_ne!(first_artifact.artifact_id, second_artifact.artifact_id);
}

#[test]
fn rate_limit_allows_calls_within_window_budget() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "rate-limit.allow".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 2,
            window_ms: 100,
        }),
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let state = rate_limit_initial_state(&policy, 0);
    let first = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.rate-limit.allow",
        &state,
        10,
        || Ok::<_, ResilienceFaultClassificationV1>("first"),
        provider_rate_limit_rejection,
        clone_fault_classification,
    );
    assert_eq!(first.result.expect("allowed"), "first");
    assert_eq!(first.state.requests_in_window, 1);
    assert_eq!(first.trace.final_status, RateLimitFinalStatusV1::Allowed);
    assert_eq!(
        first
            .trace
            .telemetry_event
            .as_ref()
            .map(|event| event.event_kind.clone()),
        Some(TelemetryEventKindV1::RateLimitDecision)
    );

    let second = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.rate-limit.allow",
        &first.state,
        20,
        || Ok::<_, ResilienceFaultClassificationV1>("second"),
        provider_rate_limit_rejection,
        clone_fault_classification,
    );
    assert_eq!(second.result.expect("allowed"), "second");
    assert_eq!(second.state.requests_in_window, 2);
    assert_eq!(second.trace.requests_in_window_before, 1);
    assert_eq!(second.trace.requests_in_window_after, 2);
}

#[test]
fn rate_limit_throttles_calls_after_budget_is_exhausted() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "rate-limit.throttle".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 100,
        }),
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let state = RateLimitStateV1 {
        schema_version: RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        window_started_at_ms: 10,
        requests_in_window: 1,
    };
    let called = Cell::new(0);
    let execution = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.rate-limit.throttle",
        &state,
        40,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
        },
        provider_rate_limit_rejection,
        clone_fault_classification,
    );

    assert_eq!(called.get(), 0);
    let failure = execution.result.expect_err("throttled");
    assert_eq!(
        failure.fault_class,
        ResilienceFaultClassV1::ProviderRateLimited
    );
    assert_eq!(
        execution.trace.final_status,
        RateLimitFinalStatusV1::Throttled
    );
    assert_eq!(execution.trace.wait_ms, Some(70));
    assert!(execution.trace.recovery_artifact.is_some());
    assert!(execution
        .trace
        .decision_summary
        .contains("wait 70ms for window refill"));
}

#[test]
fn rate_limit_resets_window_after_budget_refills() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "rate-limit.reset".to_string(),
        retry: None,
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 50,
        }),
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };
    let stale_state = RateLimitStateV1 {
        schema_version: RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        window_started_at_ms: 10,
        requests_in_window: 1,
    };
    let execution = execute_rate_limit_policy(
        &policy,
        ResilienceSurfaceV1::Workflow,
        "test.rate-limit.reset",
        &stale_state,
        70,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        provider_rate_limit_rejection,
        clone_fault_classification,
    );
    assert_eq!(execution.result.expect("allowed"), "ok");
    assert_eq!(execution.state.window_started_at_ms, 70);
    assert_eq!(execution.state.requests_in_window, 1);
    assert_eq!(execution.trace.requests_in_window_before, 0);
}

#[test]
fn retry_policy_can_respect_rate_limit_waits_without_retry_storms() {
    let policy = ResiliencePolicyV1 {
        schema_version: RESILIENCE_POLICY_SCHEMA_V1.to_string(),
        policy_id: "rate-limit.retry".to_string(),
        retry: Some(RetryPolicyV1 {
            max_attempts: 2,
            backoff_ms: Some(5),
            jitter_ms: Some(0),
            max_elapsed_ms: None,
            retryable_fault_classes: vec![ResilienceFaultClassV1::ProviderRateLimited],
        }),
        timeout: None,
        circuit_breaker: None,
        rate_limit: Some(RateLimitPolicyV1 {
            max_requests: 1,
            window_ms: 50,
        }),
        bulkhead: None,
        fallback: None,
        checkpoint_required: false,
        telemetry_required: true,
    };

    let rate_state = RefCell::new(RateLimitStateV1 {
        schema_version: RESILIENCE_RATE_LIMIT_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        window_started_at_ms: 0,
        requests_in_window: 1,
    });
    let now_ms = Cell::new(10_u64);
    let mut sleeps = Vec::new();

    let execution = execute_retry_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.rate-limit.retry",
        |_| {
            let current_state = rate_state.borrow().clone();
            let limited = execute_rate_limit_policy(
                &policy,
                ResilienceSurfaceV1::Provider,
                "test.rate-limit.retry",
                &current_state,
                now_ms.get(),
                || Ok::<_, ResilienceFaultClassificationV1>("ok"),
                provider_rate_limit_rejection,
                clone_fault_classification,
            );
            *rate_state.borrow_mut() = limited.state.clone();
            match limited.result {
                Ok(value) => Ok(value),
                Err(error) => Err(error),
            }
        },
        clone_fault_classification,
        |delay_ms| {
            sleeps.push(delay_ms);
            now_ms.set(now_ms.get().saturating_add(delay_ms.max(50)));
        },
        |_| {},
    );

    assert_eq!(execution.result.expect("second attempt succeeds"), "ok");
    assert_eq!(execution.trace.attempts.len(), 2);
    assert_eq!(sleeps, vec![40]);
    assert_eq!(rate_state.borrow().requests_in_window, 1);
    assert_eq!(rate_state.borrow().window_started_at_ms, 60);
}

#[test]
fn bulkhead_allows_calls_when_domain_has_capacity() {
    let policy = test_bulkhead_policy("provider.openrouter", 2);
    let state = bulkhead_initial_state(&policy);

    let execution = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.bulkhead.allow",
        &state,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );

    assert_eq!(execution.result.expect("allowed"), "ok");
    assert_eq!(execution.state.in_flight, 0);
    assert_eq!(execution.trace.final_status, BulkheadFinalStatusV1::Allowed);
    assert_eq!(execution.trace.fault_domain, "provider.openrouter");
    assert_eq!(execution.trace.in_flight_before, 0);
    assert_eq!(execution.trace.in_flight_during_execution, 1);
    assert_eq!(execution.trace.in_flight_after, 0);
    assert_eq!(
        execution
            .trace
            .telemetry_event
            .as_ref()
            .map(|event| event.event_kind.clone()),
        Some(TelemetryEventKindV1::BulkheadDecision)
    );
}

#[test]
fn bulkhead_rejects_when_domain_is_saturated() {
    let policy = test_bulkhead_policy("provider.ollama", 1);
    let state = BulkheadStateV1 {
        schema_version: RESILIENCE_BULKHEAD_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        fault_domain: "provider.ollama".to_string(),
        in_flight: 1,
    };
    let called = Cell::new(0);

    let execution = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::Provider,
        "test.bulkhead.saturated",
        &state,
        || {
            called.set(called.get() + 1);
            Ok::<_, ResilienceFaultClassificationV1>("should-not-run")
        },
        clone_fault_classification,
        provider_bulkhead_rejection,
    );

    assert_eq!(called.get(), 0);
    let failure = execution.result.expect_err("saturated");
    assert_eq!(failure.fault_class, ResilienceFaultClassV1::RuntimeFailure);
    assert_eq!(
        execution.trace.final_status,
        BulkheadFinalStatusV1::Saturated
    );
    assert_eq!(execution.trace.in_flight_before, 1);
    assert_eq!(execution.trace.in_flight_during_execution, 1);
    assert_eq!(execution.trace.in_flight_after, 1);
    assert!(execution.trace.recovery_artifact.is_some());
    assert!(execution
        .trace
        .decision_summary
        .contains("fault domain 'provider.ollama'"));
}

#[test]
fn bulkhead_domains_are_isolated_from_each_other() {
    let saturated_policy = test_bulkhead_policy("provider.primary", 1);
    let saturated_state = BulkheadStateV1 {
        schema_version: RESILIENCE_BULKHEAD_STATE_SCHEMA_V1.to_string(),
        policy_id: saturated_policy.policy_id.clone(),
        fault_domain: "provider.primary".to_string(),
        in_flight: 1,
    };
    let saturated = execute_bulkhead_policy(
        &saturated_policy,
        ResilienceSurfaceV1::Provider,
        "test.bulkhead.primary",
        &saturated_state,
        || Ok::<_, ResilienceFaultClassificationV1>("should-not-run"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );
    assert!(saturated.result.is_err());

    let independent_policy = test_bulkhead_policy("workflow.review", 1);
    let independent_state = bulkhead_initial_state(&independent_policy);
    let independent = execute_bulkhead_policy(
        &independent_policy,
        ResilienceSurfaceV1::Workflow,
        "test.bulkhead.workflow",
        &independent_state,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );

    assert_eq!(independent.result.expect("independent domain"), "ok");
    assert_eq!(independent.trace.fault_domain, "workflow.review");
    assert_eq!(
        independent.trace.final_status,
        BulkheadFinalStatusV1::Allowed
    );
}

#[test]
fn bulkhead_resets_stale_state_when_policy_or_domain_changes() {
    let policy = test_bulkhead_policy("tool.validation", 2);
    let stale_state = BulkheadStateV1 {
        schema_version: RESILIENCE_BULKHEAD_STATE_SCHEMA_V1.to_string(),
        policy_id: "bulkhead.old".to_string(),
        fault_domain: "provider.legacy".to_string(),
        in_flight: 5,
    };

    let execution = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::Tool,
        "test.bulkhead.reset",
        &stale_state,
        || Ok::<_, ResilienceFaultClassificationV1>("ok"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );

    assert_eq!(execution.result.expect("reset then allow"), "ok");
    assert_eq!(execution.state.policy_id, policy.policy_id);
    assert_eq!(execution.state.fault_domain, "tool.validation");
    assert_eq!(execution.state.in_flight, 0);
    assert_eq!(execution.trace.in_flight_before, 0);
}

#[test]
fn bulkhead_decision_artifacts_keep_bounded_unique_ids() {
    let policy = test_bulkhead_policy("citizen.runtime", 1);
    let state = BulkheadStateV1 {
        schema_version: RESILIENCE_BULKHEAD_STATE_SCHEMA_V1.to_string(),
        policy_id: policy.policy_id.clone(),
        fault_domain: "citizen.runtime".to_string(),
        in_flight: 1,
    };
    let first = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::CitizenRuntime,
        "test.bulkhead.ids",
        &state,
        || Ok::<_, ResilienceFaultClassificationV1>("should-not-run"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );
    let second = execute_bulkhead_policy(
        &policy,
        ResilienceSurfaceV1::CitizenRuntime,
        "test.bulkhead.ids",
        &state,
        || Ok::<_, ResilienceFaultClassificationV1>("should-not-run"),
        clone_fault_classification,
        provider_bulkhead_rejection,
    );

    let first_event = first
        .trace
        .telemetry_event
        .as_ref()
        .expect("first telemetry event");
    let second_event = second
        .trace
        .telemetry_event
        .as_ref()
        .expect("second telemetry event");
    let first_artifact = first
        .trace
        .recovery_artifact
        .as_ref()
        .expect("first artifact");
    let second_artifact = second
        .trace
        .recovery_artifact
        .as_ref()
        .expect("second artifact");

    assert_ne!(first_event.event_id, second_event.event_id);
    assert_ne!(first_artifact.artifact_id, second_artifact.artifact_id);
}
