use csdlc_v2::cards::{
    apply, CardContent, CardIdentity, CardStatus, CardValues, ExecutionEstimates, PlanStep,
    SemanticOperation, SppValues, StepStatus,
};
use csdlc_v2::estimation::{
    calibration_report, canonical_digest, compare_cycle_time, estimation_schema_bundle, forecast,
    join_observations, terminal_outcome, validate_accepted_estimate, validate_observation,
    AcceptedEstimate, Availability, BacktestCase, ComparableKey, CycleTimeCohort,
    EstimateDisposition, EstimateMethod, MetricObservation, Observation, ObservationSource,
    StaticEstimate, OBSERVATION_SCHEMA,
};

fn key() -> ComparableKey {
    ComparableKey {
        cohort: "csdlc-v2-rust-medium".into(),
        workflow_version: "v2-gate-10d2".into(),
        model_era: "codex-2026-q3".into(),
    }
}

fn unknown(source: ObservationSource, reference: &str) -> MetricObservation {
    MetricObservation::unavailable(Availability::Unknown, source, reference)
}

fn observation(issue: u64, elapsed: u64, validation: u64, tokens: u64) -> Observation {
    Observation {
        schema: OBSERVATION_SCHEMA.into(),
        issue,
        key: key(),
        elapsed_seconds: MetricObservation::known(
            elapsed,
            ObservationSource::Lifecycle,
            format!(".csdlc/evidence/{issue}/lifecycle.json"),
        ),
        active_work_seconds: MetricObservation::known(
            elapsed.saturating_sub(validation),
            ObservationSource::Session,
            format!(".csdlc/evidence/{issue}/session-metrics.json"),
        ),
        validation_seconds: MetricObservation::known(
            validation,
            ObservationSource::Validation,
            format!(".csdlc/evidence/{issue}/validation.json"),
        ),
        pr_wait_seconds: MetricObservation::known(
            120,
            ObservationSource::Github,
            format!("github:agent-logic/agent-design-language#{issue}"),
        ),
        ci_wait_seconds: MetricObservation::known(
            180,
            ObservationSource::Github,
            format!("github:agent-logic/agent-design-language/actions/{issue}"),
        ),
        total_tokens: MetricObservation::known(
            tokens,
            ObservationSource::Session,
            format!(".csdlc/evidence/{issue}/token-summary.json"),
        ),
    }
}

fn fallback() -> StaticEstimate {
    StaticEstimate {
        elapsed_seconds: 21_600,
        validation_seconds: 3_600,
        total_tokens: 80_000,
    }
}

#[test]
fn observation_schema_round_trips_with_explicit_unknowns_and_provenance() {
    let mut packet = observation(10, 1_000, 100, 20_000);
    packet.ci_wait_seconds = MetricObservation::unavailable(
        Availability::Interrupted,
        ObservationSource::OperatorAnnotation,
        ".csdlc/evidence/10/operator-pause.json",
    );
    validate_observation(&packet).unwrap();
    let encoded = serde_json::to_vec(&packet).unwrap();
    let decoded: Observation = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded, packet);
    assert_eq!(decoded.ci_wait_seconds.value, None);
    assert_eq!(
        decoded.ci_wait_seconds.availability,
        Availability::Interrupted
    );
    let schemas = estimation_schema_bundle();
    assert_eq!(schemas["schema"], "csdlc.estimation_schema_bundle.v1");
    assert!(schemas["forecast"]["definitions"].is_object());
}

#[test]
fn deterministic_join_covers_all_sources_and_rejects_conflicting_known_values() {
    let base = observation(11, 1_100, 110, 22_000);
    let mut annotation = base.clone();
    annotation.elapsed_seconds = unknown(
        ObservationSource::OperatorAnnotation,
        ".csdlc/evidence/11/operator.json",
    );
    let left = join_observations(vec![base.clone(), annotation.clone()]).unwrap();
    let right = join_observations(vec![annotation, base.clone()]).unwrap();
    assert_eq!(left, right);
    let sources: std::collections::BTreeSet<_> = [
        &left.elapsed_seconds,
        &left.active_work_seconds,
        &left.validation_seconds,
        &left.pr_wait_seconds,
        &left.ci_wait_seconds,
        &left.total_tokens,
    ]
    .into_iter()
    .flat_map(|metric| metric.provenance.iter().map(|item| item.source))
    .collect();
    assert_eq!(sources.len(), 5);

    let mut conflicting = base.clone();
    conflicting.elapsed_seconds.value = Some(9_999);
    assert!(join_observations(vec![base, conflicting]).is_err());
}

#[test]
fn privacy_and_schema_drift_fail_closed_without_treating_unknown_as_zero() {
    let mut packet = observation(12, 1_200, 120, 24_000);
    packet.elapsed_seconds.provenance[0].reference =
        "/Users/operator/session-transcript.json".into();
    assert!(validate_observation(&packet).is_err());

    let mut drifted = observation(13, 1_300, 130, 26_000);
    drifted.validation_seconds = MetricObservation::unavailable(
        Availability::SchemaDrift,
        ObservationSource::Validation,
        ".csdlc/evidence/13/validation-v0.json",
    );
    let result = forecast(99, key(), &[drifted], fallback()).unwrap();
    assert_eq!(result.method, EstimateMethod::StaticPlanningProfileFallback);
    assert_eq!(result.validation_seconds.point, 3_600);
    assert_eq!(result.validation_seconds.cohort_size, 0);
}

#[test]
fn comparable_cohort_is_deterministic_robust_and_excludes_target_actuals() {
    let candidates = vec![
        observation(21, 1_000, 100, 20_000),
        observation(22, 1_200, 120, 24_000),
        observation(23, 1_100, 110, 22_000),
        observation(99, 9_999_999, 999_999, 9_999_999),
    ];
    let first = forecast(99, key(), &candidates, fallback()).unwrap();
    let mut reversed = candidates;
    reversed.reverse();
    let second = forecast(99, key(), &reversed, fallback()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.method, EstimateMethod::ComparableMedian);
    assert_eq!(first.elapsed_seconds.point, 1_100);
    assert_eq!(first.comparable_issues, vec![21, 22, 23]);
    assert!(!first.comparable_issues.contains(&99));
    assert_eq!(first.elapsed_seconds.cohort_size, 3);
    assert_eq!(first.elapsed_seconds.source_provenance.len(), 3);
}

#[test]
fn sparse_or_incomparable_data_uses_the_static_planning_profile_fallback() {
    let mut other_era = observation(31, 900, 90, 18_000);
    other_era.key.model_era = "older-model-era".into();
    let result = forecast(
        32,
        key(),
        &[observation(30, 1_000, 100, 20_000), other_era],
        fallback(),
    )
    .unwrap();
    assert_eq!(result.method, EstimateMethod::StaticPlanningProfileFallback);
    assert_eq!(result.elapsed_seconds.point, 21_600);
    assert_eq!(result.comparable_issues, vec![30]);
    assert!(result.fallback_reason.is_some());
}

#[test]
fn typed_spp_advisory_disposition_cannot_enforce_execution_or_phase() {
    let forecast = forecast(
        40,
        key(),
        &[
            observation(41, 1_000, 100, 20_000),
            observation(42, 1_100, 110, 22_000),
            observation(43, 1_200, 120, 24_000),
        ],
        fallback(),
    )
    .unwrap();
    let accepted = AcceptedEstimate {
        forecast_ref: ".csdlc/evidence/40/forecast.json".into(),
        forecast_digest: canonical_digest(&forecast).unwrap(),
        disposition: EstimateDisposition::Accepted,
        advisory_only: true,
        adjusted: None,
        operator_rationale: "Comparable v2 cohort is sufficient for planning guidance.".into(),
    };
    validate_accepted_estimate(&accepted).unwrap();
    let mut card = spp_card();
    let phase = apply(
        &mut card,
        &SemanticOperation::RecordAdvisoryEstimate {
            estimate: accepted.clone(),
        },
    )
    .unwrap();
    assert_eq!(phase, None);
    assert_eq!(card.status, CardStatus::Ready);
    let CardContent::Spp(spp) = card.content else {
        panic!("SPP")
    };
    assert_eq!(spp.execution_estimates.advisory, Some(accepted));

    let mut invalid = spp.execution_estimates.advisory.unwrap();
    invalid.advisory_only = false;
    assert!(validate_accepted_estimate(&invalid).is_err());
}

#[test]
fn terminal_outcomes_and_backtests_are_reproducible_and_calibrated() {
    let candidates = [
        observation(51, 1_000, 100, 20_000),
        observation(52, 1_100, 110, 22_000),
        observation(53, 1_200, 120, 24_000),
    ];
    let mut cases = Vec::new();
    for issue in [61, 62, 63] {
        let forecast = forecast(issue, key(), &candidates, fallback()).unwrap();
        let actual = observation(issue, 1_100, 110, 22_000);
        let outcome = terminal_outcome(
            &forecast,
            format!(".csdlc/evidence/{issue}/forecast.json"),
            &actual,
        )
        .unwrap();
        assert!(outcome.unknown_actuals.is_empty());
        assert!(!outcome.elapsed_seconds.actual_provenance.is_empty());
        cases.push(BacktestCase {
            issue,
            forecast,
            outcome,
        });
    }
    let first = calibration_report(&cases, 500).unwrap();
    let second = calibration_report(&cases, 500).unwrap();
    assert_eq!(first, second);
    assert!(first.calibrated);
    assert_eq!(first.elapsed_median_error_basis_points, Some(0));
}

#[test]
fn equivalent_cycle_time_cohorts_measure_reconnection_without_gate_weakening() {
    let baseline = CycleTimeCohort {
        id: "baseline-v2-bind".into(),
        issue_count: 3,
        active_work_seconds: 1_800,
        validation_seconds: 600,
        review_seconds: 300,
        ci_seconds: 450,
        wait_seconds: 900,
        reconnect_actions: 12,
    };
    let candidate = CycleTimeCohort {
        id: "candidate-v2-bind".into(),
        issue_count: 3,
        active_work_seconds: 1_800,
        validation_seconds: 600,
        review_seconds: 300,
        ci_seconds: 450,
        wait_seconds: 300,
        reconnect_actions: 3,
    };
    let report = compare_cycle_time(baseline.clone(), candidate.clone(), true, true).unwrap();
    assert_eq!(report.elapsed_reduction_seconds, 600);
    assert_eq!(report.reconnect_action_reduction, 9);
    assert!(compare_cycle_time(baseline, candidate, false, true).is_err());
}

fn spp_card() -> CardValues {
    CardValues {
        identity: CardIdentity {
            schema_version: "1.0.0".into(),
            template_version: "1.0.0".into(),
            issue: 40,
            repository: "agent-logic/agent-design-language".into(),
            title: "estimate fixture".into(),
            slug: "estimate-fixture".into(),
            version: "v0.92".into(),
            generation: 1,
        },
        status: CardStatus::Ready,
        content: CardContent::Spp(SppValues {
            plan_revision: 1,
            summary: "Use an advisory estimate.".into(),
            steps: vec![PlanStep {
                id: "S1".into(),
                action: "record advisory".into(),
                acceptance_ids: vec!["AC-1".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec!["csdlc-v2/src/estimation.rs".into()],
            invariants: vec!["advisory only".into()],
            risks: vec!["sparse data".into()],
            execution_estimates: ExecutionEstimates {
                elapsed_seconds: 21_600,
                validation_seconds: 3_600,
                total_tokens: 80_000,
                advisory: None,
            },
            stop_conditions: vec!["schema drift".into()],
            replan_triggers: vec!["model era changes".into()],
            design_ref: "docs/design.md".into(),
            design_digest: "d".repeat(64),
            diagram_ref: "docs/diagram.mmd".into(),
            diagram_digest: "e".repeat(64),
        }),
    }
}
