use std::fs;
use std::path::PathBuf;
use std::process::Command;

use csdlc_v2::cards::{
    apply, CardContent, CardIdentity, CardStatus, CardValues, EvidenceOutcome, ExecutionEstimates,
    PlanStep, SemanticOperation, SppValues, StepStatus, ValidationResult,
};
use csdlc_v2::estimation::{
    adapt_github, adapt_lifecycle, adapt_operator, adapt_session, adapt_validation,
    calibration_report, canonical_digest, compare_cycle_time, estimation_schema_bundle, forecast,
    join_observations, load_verified_json, terminal_outcome, validate_accepted_estimate,
    validate_observation, validate_reference, verified_calibration, AcceptedEstimate,
    AdapterContext, ArtifactReference, BacktestCase, CalibrationReport, ComparableKey,
    CycleTimeCohort, CycleTimeComparisonStatus, EstimateDisposition, EstimateMethod, GithubTiming,
    LifecycleTiming, MetricObservation, Observation, ObservationSource, OperatorTiming,
    SessionTiming, StaticEstimate, ValidationTiming, VerifiedCalibration, OBSERVATION_SCHEMA,
};
use csdlc_v2::{
    retain_terminal_estimation_outcome, DerivedTerminalEnvelope, FinishDisposition, PrStatePacket,
    Store, TerminalEstimationStatus,
};

fn key() -> ComparableKey {
    ComparableKey {
        cohort: "csdlc-v2-rust-medium".into(),
        workflow_version: "v2-gate-10d2".into(),
        model_era: "codex-2026-q3".into(),
    }
}

fn context(issue: u64, reference: &str) -> AdapterContext {
    AdapterContext {
        issue,
        key: key(),
        reference: reference.into(),
    }
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

fn calibration(calibrated: bool) -> VerifiedCalibration {
    let report = CalibrationReport {
        schema: "csdlc.estimation_calibration.v1".into(),
        case_count: 3,
        elapsed_median_error_basis_points: Some(100),
        validation_median_error_basis_points: Some(100),
        token_median_error_basis_points: Some(100),
        calibrated,
    };
    VerifiedCalibration {
        artifact: ArtifactReference {
            reference: ".csdlc/evidence/99/calibration.json".into(),
            digest: canonical_digest(&report).unwrap(),
        },
        report,
    }
}

fn fastwork_root(name: &str) -> PathBuf {
    let target = PathBuf::from(std::env::var("CARGO_TARGET_DIR").expect("FastWork target"));
    assert!(target.starts_with("/Volumes/FastWork"));
    let root = target.join("estimation-contract-fixtures").join(name);
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn concrete_adapters_join_all_authoritative_observation_sources() {
    let issue = 5822;
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let record = Store::new(&repository_root).load_record(issue).unwrap();
    let packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "agent-logic/agent-design-language".into(),
        pull_request: 6000,
        linked_issue: Some(issue),
        linkage_source: Some("closing_keyword".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "clean".into(),
        review_decision: "approved".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5822".into()),
        head_sha: "a".repeat(40),
        url: Some("github:agent-logic/agent-design-language/pull/6000".into()),
        body: None,
        merged: true,
        merge_commit_sha: Some("b".repeat(40)),
        checks: Vec::new(),
        required_check_names: Vec::new(),
        classification: "ready".into(),
    };
    let validation_result = ValidationResult {
        command: vec!["cargo".into(), "nextest".into()],
        purpose: "focused PVF lane".into(),
        outcome: EvidenceOutcome::Passed,
        evidence_ref: ".csdlc/evidence/5822/validation-summary.json".into(),
    };
    let fragments = vec![
        adapt_lifecycle(
            context(issue, ".csdlc/issues/5822/index.json#audit"),
            &record,
            LifecycleTiming {
                bound_unix_seconds: 100,
                observed_unix_seconds: 1_100,
            },
        )
        .unwrap(),
        adapt_github(
            context(issue, "github:agent-logic/agent-design-language#5822"),
            &packet,
            GithubTiming {
                pr_opened_unix_seconds: 1_000,
                terminal_unix_seconds: 1_120,
                ci_started_unix_seconds: Some(1_010),
                ci_completed_unix_seconds: Some(1_080),
            },
        )
        .unwrap(),
        adapt_validation(
            context(issue, ".csdlc/evidence/5822/validation-summary.json"),
            &validation_result,
            ValidationTiming {
                started_unix_seconds: 500,
                completed_unix_seconds: 610,
            },
        )
        .unwrap(),
        adapt_session(
            context(issue, "codex-goal:5822"),
            SessionTiming {
                started_unix_seconds: 100,
                observed_unix_seconds: 1_100,
                active_work_seconds: Some(890),
                total_tokens: Some(22_000),
                interrupted: false,
            },
        )
        .unwrap(),
        adapt_operator(
            context(issue, ".csdlc/evidence/5822/operator-annotation.json"),
            OperatorTiming {
                wait_seconds: None,
                interrupted: false,
            },
        )
        .unwrap(),
    ];
    let joined = join_observations(fragments).unwrap();
    validate_observation(&joined).unwrap();
    let sources: std::collections::BTreeSet<_> = [
        &joined.elapsed_seconds,
        &joined.active_work_seconds,
        &joined.validation_seconds,
        &joined.pr_wait_seconds,
        &joined.ci_wait_seconds,
        &joined.total_tokens,
    ]
    .into_iter()
    .flat_map(|metric| metric.provenance.iter().map(|item| item.source))
    .collect();
    assert_eq!(sources.len(), 5);
    assert_eq!(joined.elapsed_seconds.value, Some(1_000));
    assert_eq!(joined.validation_seconds.value, Some(110));
    assert_eq!(joined.total_tokens.value, Some(22_000));
    assert!(estimation_schema_bundle()["forecast"]["definitions"].is_object());
}

#[test]
fn duplicate_issue_ids_are_consolidated_or_rejected() {
    let calibrated = calibration(true);
    let one = observation(21, 1_000, 100, 20_000);
    let duplicate_only = forecast(
        99,
        key(),
        &[one.clone(), one.clone(), one.clone()],
        fallback(),
        Some(&calibrated),
    )
    .unwrap();
    assert_eq!(
        duplicate_only.method,
        EstimateMethod::StaticPlanningProfileFallback
    );
    assert_eq!(duplicate_only.comparable_issues, vec![21]);

    let mut conflicting = one.clone();
    conflicting.elapsed_seconds.value = Some(2_000);
    assert!(forecast(
        99,
        key(),
        &[one, conflicting],
        fallback(),
        Some(&calibrated)
    )
    .is_err());
}

#[test]
fn calibration_is_mandatory_for_comparable_forecasts() {
    let candidates = vec![
        observation(21, 1_000, 100, 20_000),
        observation(22, 1_200, 120, 24_000),
        observation(23, 1_100, 110, 22_000),
    ];
    let missing = forecast(99, key(), &candidates, fallback(), None).unwrap();
    assert_eq!(
        missing.method,
        EstimateMethod::StaticPlanningProfileFallback
    );
    let failed = calibration(false);
    let failed = forecast(99, key(), &candidates, fallback(), Some(&failed)).unwrap();
    assert_eq!(failed.method, EstimateMethod::StaticPlanningProfileFallback);

    let calibrated = calibration(true);
    let first = forecast(99, key(), &candidates, fallback(), Some(&calibrated)).unwrap();
    let mut reversed = candidates;
    reversed.reverse();
    let second = forecast(99, key(), &reversed, fallback(), Some(&calibrated)).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.method, EstimateMethod::ComparableMedian);
    assert_eq!(first.elapsed_seconds.point, 1_100);
    assert_eq!(first.comparable_issues, vec![21, 22, 23]);
}

#[test]
fn verified_artifacts_reject_digest_tampering() {
    let root = fastwork_root("digest-tamper");
    let directory = root.join(".csdlc/evidence/99");
    fs::create_dir_all(&directory).unwrap();
    let report = calibration(true).report;
    let path = directory.join("calibration.json");
    fs::write(&path, serde_json::to_vec(&report).unwrap()).unwrap();
    let artifact = ArtifactReference {
        reference: ".csdlc/evidence/99/calibration.json".into(),
        digest: canonical_digest(&report).unwrap(),
    };
    assert!(
        verified_calibration(&root, artifact.clone())
            .unwrap()
            .report
            .calibrated
    );
    let mut changed = report;
    changed.calibrated = false;
    fs::write(path, serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(load_verified_json::<CalibrationReport>(&root, &artifact).is_err());
}

#[test]
fn traversal_windows_absolute_and_sensitive_references_fail_closed() {
    for reference in [
        "../escape.json",
        "evidence/../../escape.json",
        "C:\\Users\\operator\\artifact.json",
        "D:/artifact.json",
        "\\\\server\\share\\artifact.json",
        "/Users/operator/artifact.json",
        ".csdlc/session-transcript.json",
    ] {
        assert!(
            validate_reference(reference).is_err(),
            "accepted {reference}"
        );
    }
    assert!(validate_reference(".csdlc/evidence/99/artifact.json").is_ok());
}

#[test]
fn advisory_disposition_and_terminal_outcome_are_reproducible() {
    let calibrated = calibration(true);
    let forecast = forecast(
        40,
        key(),
        &[
            observation(41, 1_000, 100, 20_000),
            observation(42, 1_100, 110, 22_000),
            observation(43, 1_200, 120, 24_000),
        ],
        fallback(),
        Some(&calibrated),
    )
    .unwrap();
    let accepted = AcceptedEstimate {
        forecast_ref: ".csdlc/evidence/40/forecast.json".into(),
        forecast_digest: canonical_digest(&forecast).unwrap(),
        disposition: EstimateDisposition::Accepted,
        advisory_only: true,
        adjusted: None,
        operator_rationale: "Calibrated comparable cohort accepted as advisory guidance.".into(),
    };
    validate_accepted_estimate(&accepted).unwrap();
    let outcome = terminal_outcome(
        &forecast,
        accepted.forecast_ref.clone(),
        &observation(40, 1_100, 110, 22_000),
    )
    .unwrap();
    assert!(outcome.unknown_actuals.is_empty());
    assert_eq!(outcome.forecast_digest, accepted.forecast_digest);

    let mut card = spp_card();
    assert_eq!(
        apply(
            &mut card,
            &SemanticOperation::RecordAdvisoryEstimate { estimate: accepted }
        )
        .unwrap(),
        None
    );
    assert_eq!(card.status, CardStatus::Ready);
}

#[test]
fn finish_reconciliation_records_verified_outcome_and_reports_tampering() {
    let root = fastwork_root("finish-reconciliation");
    assert!(Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(&root)
        .status()
        .unwrap()
        .success());
    let source_cards = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".csdlc/issues/5822/cards");
    let target_cards = root.join(".csdlc/issues/5822/cards");
    fs::create_dir_all(&target_cards).unwrap();
    for entry in fs::read_dir(source_cards).unwrap() {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_string_lossy()
            .ends_with(".values.json")
        {
            fs::copy(entry.path(), target_cards.join(entry.file_name())).unwrap();
        }
    }

    let report = calibration(true).report;
    let calibration_path = root.join(".csdlc/evidence/5822/calibration.json");
    fs::create_dir_all(calibration_path.parent().unwrap()).unwrap();
    fs::write(&calibration_path, serde_json::to_vec(&report).unwrap()).unwrap();
    let calibration = VerifiedCalibration {
        artifact: ArtifactReference {
            reference: ".csdlc/evidence/5822/calibration.json".into(),
            digest: canonical_digest(&report).unwrap(),
        },
        report,
    };
    let forecast = forecast(
        5822,
        key(),
        &[
            observation(70, 1_000, 100, 20_000),
            observation(71, 1_100, 110, 22_000),
            observation(72, 1_200, 120, 24_000),
        ],
        fallback(),
        Some(&calibration),
    )
    .unwrap();
    let forecast_path = root.join(".csdlc/evidence/5822/forecast.json");
    fs::write(&forecast_path, serde_json::to_vec(&forecast).unwrap()).unwrap();
    let accepted = AcceptedEstimate {
        forecast_ref: ".csdlc/evidence/5822/forecast.json".into(),
        forecast_digest: canonical_digest(&forecast).unwrap(),
        disposition: EstimateDisposition::Accepted,
        advisory_only: true,
        adjusted: None,
        operator_rationale: "verified finish fixture".into(),
    };
    let spp_path = target_cards.join("spp.values.json");
    let mut spp: CardValues = serde_json::from_slice(&fs::read(&spp_path).unwrap()).unwrap();
    let CardContent::Spp(values) = &mut spp.content else {
        panic!("SPP fixture")
    };
    values.execution_estimates.advisory = Some(Box::new(accepted));
    fs::write(&spp_path, serde_json::to_vec(&spp).unwrap()).unwrap();

    let terminal = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: 5822,
        repository: "agent-logic/agent-design-language".into(),
        initialization_digest: "a".repeat(64),
        canonical_generation: 1,
        canonical_digest: "b".repeat(64),
        pull_request: Some(1),
        disposition: FinishDisposition::Merged,
        head_sha: Some("c".repeat(40)),
        merge_sha: Some("d".repeat(40)),
        issue_state: "closed".into(),
        pr_state: Some("merged".into()),
        approved_reason: None,
        observed_unix_seconds: 1,
        mutable_fresh_until_unix_seconds: None,
        source: "github".into(),
        digest: "e".repeat(64),
    };
    let store = Store::new(&root);
    let recorded = retain_terminal_estimation_outcome(&store, &terminal);
    assert_eq!(recorded.status, TerminalEstimationStatus::Recorded);
    assert!(recorded.outcome_digest.is_some());

    fs::write(forecast_path, b"{}\n").unwrap();
    let tampered = retain_terminal_estimation_outcome(&store, &terminal);
    assert_eq!(tampered.status, TerminalEstimationStatus::Invalid);
}

#[test]
fn calibration_backtests_reject_duplicate_cases() {
    let failed = calibration(false);
    let forecast = forecast(61, key(), &[], fallback(), Some(&failed)).unwrap();
    let actual = observation(61, 21_600, 3_600, 80_000);
    let outcome = terminal_outcome(&forecast, ".csdlc/evidence/61/forecast.json", &actual).unwrap();
    let case = BacktestCase {
        issue: 61,
        forecast,
        outcome,
    };
    assert!(calibration_report(&[case.clone(), case], 500).is_err());
}

#[test]
fn measured_cycle_evidence_refuses_incomparable_speedup_claims() {
    let provenance = ArtifactReference {
        reference: ".csdlc/evidence/5822/operator-cycle-inputs.json".into(),
        digest: "a".repeat(64),
    };
    let baseline = CycleTimeCohort {
        id: "prior-execution-goal".into(),
        issue_count: 1,
        active_work_seconds: 1_436,
        validation_seconds: 0,
        review_seconds: 0,
        ci_seconds: 0,
        wait_seconds: 0,
        reconnect_actions: 2,
        measured: true,
        unknown_components: vec!["validation_seconds".into(), "review_seconds".into()],
        provenance: vec![provenance.clone()],
    };
    let candidate = CycleTimeCohort {
        id: "repair-goal-in-progress".into(),
        issue_count: 1,
        active_work_seconds: 900,
        validation_seconds: 0,
        review_seconds: 0,
        ci_seconds: 0,
        wait_seconds: 0,
        reconnect_actions: 0,
        measured: true,
        unknown_components: vec!["terminal_elapsed".into(), "review_seconds".into()],
        provenance: vec![provenance],
    };
    let report = compare_cycle_time(baseline, candidate, false, true).unwrap();
    assert_eq!(report.status, CycleTimeComparisonStatus::NotComparable);
    assert_eq!(report.elapsed_reduction_seconds, 0);
    assert_eq!(report.reconnect_action_reduction, 0);
    assert!(report.gates_preserved);
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
