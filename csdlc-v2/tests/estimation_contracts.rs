use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use csdlc_v2::cards::{CardContent, CardValues};
use csdlc_v2::estimation::{
    artifact_reference, calibration_report, compare_cycle_time, forecast, load_cycle_time_evidence,
    load_observation_manifest, load_verified_json, terminal_outcome, validate_reference,
    verified_calibration, AcceptedEstimate, ArtifactReference, BacktestCase,
    CalibrationCaseArtifacts, CalibrationManifest, ComparableKey, CycleIssueTiming,
    CycleTimeComparisonStatus, CycleTimeEvidence, EstimateDisposition, EstimateMethod,
    MetricObservation, Observation, ObservationManifest, ObservationSource, StaticEstimate,
    OBSERVATION_SCHEMA,
};
use csdlc_v2::{
    retain_terminal_estimation_outcome, DerivedTerminalEnvelope, FinishDisposition, Store,
    TerminalEstimationStatus,
};

fn key() -> ComparableKey {
    ComparableKey {
        cohort: "csdlc-v2-rust-medium".into(),
        workflow_version: "v2-gate-10d2".into(),
        model_era: "codex-2026-q3".into(),
    }
}

fn fallback() -> StaticEstimate {
    StaticEstimate {
        elapsed_seconds: 21_600,
        validation_seconds: 3_600,
        total_tokens: 80_000,
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

fn write_json(root: &Path, reference: &str, value: &serde_json::Value) -> ArtifactReference {
    let path = root.join(reference);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
    artifact_reference(root, reference).unwrap()
}

fn observation(issue: u64, elapsed: u64, validation: u64, tokens: u64) -> Observation {
    Observation {
        schema: OBSERVATION_SCHEMA.into(),
        issue,
        key: key(),
        elapsed_seconds: MetricObservation::known(
            elapsed,
            ObservationSource::Session,
            format!(".csdlc/evidence/{issue}/session.json"),
        ),
        active_work_seconds: MetricObservation::known(
            elapsed.saturating_sub(validation),
            ObservationSource::Session,
            format!(".csdlc/evidence/{issue}/session.json"),
        ),
        validation_seconds: MetricObservation::known(
            validation,
            ObservationSource::Validation,
            format!(".csdlc/evidence/{issue}/pvf.json"),
        ),
        pr_wait_seconds: MetricObservation::known(
            120,
            ObservationSource::Github,
            format!(".csdlc/evidence/{issue}/github.json"),
        ),
        ci_wait_seconds: MetricObservation::known(
            60,
            ObservationSource::Github,
            format!(".csdlc/evidence/{issue}/github.json"),
        ),
        total_tokens: MetricObservation::known(
            tokens,
            ObservationSource::Session,
            format!(".csdlc/evidence/{issue}/session.json"),
        ),
    }
}

fn source_manifest(root: &Path, issue: u64) -> ArtifactReference {
    let base = format!(".csdlc/evidence/{issue}");
    let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let lifecycle_value: serde_json::Value = serde_json::from_slice(
        &fs::read(repository_root.join(".csdlc/issues/5822/index.json")).unwrap(),
    )
    .unwrap();
    let lifecycle = write_json(root, &format!("{base}/lifecycle.json"), &lifecycle_value);
    let github = write_json(
        root,
        &format!("{base}/github.json"),
        &serde_json::json!({
            "number":6000, "body":format!("Closes #{issue}"),
            "created_at":"2026-08-06T00:00:00Z", "closed_at":"2026-08-06T00:02:00Z",
            "merged_at":"2026-08-06T00:02:00Z",
            "base":{"repo":{"full_name":"agent-logic/agent-design-language"}}
        }),
    );
    let validation = write_json(
        root,
        &format!("{base}/pvf.json"),
        &serde_json::json!({
            "schema":"csdlc.pvf_execution_report.v1", "disposition":"local_pass",
            "selected_waves":[["focused"]], "evidence":[{
                "lane":"focused", "command":["cargo","nextest"], "purpose":"focused proof",
                "status":"passed", "duration_ms":110000, "log_ref":null, "redaction_ok":true,
                "redactions_applied":0, "path_hygiene_ok":true
            }]
        }),
    );
    let session = write_json(
        root,
        &format!("{base}/session.json"),
        &serde_json::json!({
            "schema_version":"issue_goal_metrics.v1", "issue_number":issue,
            "data_source":"codex_goal_tool", "started_at":"2026-08-06T00:00:00Z",
            "completed_at":"2026-08-06T00:16:40Z", "elapsed_seconds":1000,
            "active_work_seconds":890, "elapsed_availability":"known",
            "active_work_availability":"known", "token_usage":{
                "availability":"known", "total_tokens":22000
            }
        }),
    );
    let operator = write_json(
        root,
        &format!("{base}/operator.json"),
        &serde_json::json!({
            "schema":"csdlc.operator_timing_source.v1", "issue":issue,
            "source":"operator_annotation", "wait_intervals":[], "interrupted":false
        }),
    );
    let manifest = ObservationManifest {
        schema: "csdlc.estimation_observation_manifest.v1".into(),
        issue,
        key: key(),
        lifecycle: Some(lifecycle),
        github: Some(github),
        validation: Some(validation),
        session: Some(session),
        operator: Some(operator),
    };
    write_json(
        root,
        &format!("{base}/terminal-observation-manifest.json"),
        &serde_json::to_value(manifest).unwrap(),
    )
}

fn verified_fixture_calibration(root: &Path) -> csdlc_v2::VerifiedCalibration {
    let mut cases = Vec::new();
    let mut entries = Vec::new();
    for issue in [71_u64, 72, 73] {
        let predicted = forecast(issue, key(), &[], fallback(), None).unwrap();
        let forecast_ref = write_json(
            root,
            &format!(".csdlc/evidence/99/{issue}-forecast.json"),
            &serde_json::to_value(&predicted).unwrap(),
        );
        let actual = observation(issue, 21_600, 3_600, 80_000);
        let outcome = terminal_outcome(&predicted, forecast_ref.clone(), &actual).unwrap();
        let outcome_ref = write_json(
            root,
            &format!(".csdlc/evidence/99/{issue}-outcome.json"),
            &serde_json::to_value(&outcome).unwrap(),
        );
        entries.push(CalibrationCaseArtifacts {
            issue,
            forecast: forecast_ref,
            outcome: outcome_ref,
        });
        cases.push(BacktestCase {
            issue,
            forecast: predicted,
            outcome,
        });
    }
    let manifest = CalibrationManifest {
        schema: "csdlc.estimation_calibration_manifest.v1".into(),
        cases: entries,
        context_sources: vec![],
        tolerance_basis_points: 500,
    };
    let manifest_ref = write_json(
        root,
        ".csdlc/evidence/99/calibration-manifest.json",
        &serde_json::to_value(&manifest).unwrap(),
    );
    let report = calibration_report(&cases, 500, manifest_ref).unwrap();
    let report_ref = write_json(
        root,
        ".csdlc/evidence/99/calibration.json",
        &serde_json::to_value(report).unwrap(),
    );
    verified_calibration(root, report_ref).unwrap()
}

#[test]
fn adapters_derive_identity_and_measurements_from_verified_retained_sources() {
    let root = fastwork_root("sources");
    let manifest = source_manifest(&root, 5822);
    let joined = load_observation_manifest(&root, &manifest).unwrap();
    assert_eq!(joined.issue, 5822);
    assert_eq!(joined.elapsed_seconds.value, Some(1000));
    assert_eq!(joined.validation_seconds.value, Some(110));
    assert_eq!(joined.pr_wait_seconds.value, Some(120));
    assert_eq!(joined.total_tokens.value, Some(22_000));

    fs::write(root.join(".csdlc/evidence/5822/session.json"), b"{}\n").unwrap();
    assert!(load_observation_manifest(&root, &manifest).is_err());
}

#[test]
fn source_identity_mismatch_and_missing_sources_fail_closed() {
    let root = fastwork_root("source-identity");
    let manifest = source_manifest(&root, 5822);
    let manifest_value: ObservationManifest = load_verified_json(&root, &manifest).unwrap();
    let session = manifest_value.session.unwrap();
    fs::write(root.join(&session.reference), serde_json::to_vec(&serde_json::json!({
        "schema_version":"issue_goal_metrics.v1", "issue_number":9999,
        "data_source":"codex_goal_tool", "started_at":"2026-08-06T00:00:00Z",
        "completed_at":"2026-08-06T00:00:01Z", "elapsed_seconds":1,
        "active_work_seconds":1, "elapsed_availability":"known",
        "active_work_availability":"known", "token_usage":{"availability":"known","total_tokens":1}
    })).unwrap()).unwrap();
    let rewritten = artifact_reference(&root, session.reference.clone()).unwrap();
    let mut changed: ObservationManifest = load_verified_json(&root, &manifest).unwrap();
    changed.session = Some(rewritten);
    let changed_ref = write_json(
        &root,
        ".csdlc/evidence/5822/changed-manifest.json",
        &serde_json::to_value(changed).unwrap(),
    );
    assert!(load_observation_manifest(&root, &changed_ref).is_err());

    fs::write(
        root.join(&session.reference),
        serde_json::to_vec(&serde_json::json!({
            "schema_version":"issue_goal_metrics.v1", "issue_number":5822,
            "data_source":"codex_goal_tool", "started_at":"2026-08-06T00:00:00Z",
            "completed_at":"2026-08-06T00:00:02Z", "elapsed_seconds":999,
            "active_work_seconds":1, "elapsed_availability":"known",
            "active_work_availability":"known", "token_usage":{"availability":"known","total_tokens":1}
        }))
        .unwrap(),
    )
    .unwrap();
    let mut mismatch: ObservationManifest = load_verified_json(&root, &manifest).unwrap();
    mismatch.session = Some(artifact_reference(&root, &session.reference).unwrap());
    let mismatch_ref = write_json(
        &root,
        ".csdlc/evidence/5822/mismatch-manifest.json",
        &serde_json::to_value(mismatch).unwrap(),
    );
    assert!(load_observation_manifest(&root, &mismatch_ref).is_err());
    assert!(artifact_reference(&root, ".csdlc/evidence/5822/missing.json").is_err());
}

#[test]
fn verified_calibration_is_loader_only_reproducible_and_tamper_evident() {
    let root = fastwork_root("calibration");
    let calibration = verified_fixture_calibration(&root);
    assert!(calibration.report().calibrated);
    let candidates = vec![
        observation(21, 1000, 100, 20_000),
        observation(22, 1100, 110, 22_000),
        observation(23, 1200, 120, 24_000),
    ];
    assert_eq!(
        forecast(99, key(), &candidates, fallback(), Some(&calibration))
            .unwrap()
            .method,
        EstimateMethod::ComparableMedian
    );

    let manifest: CalibrationManifest =
        load_verified_json(&root, &calibration.report().source_manifest).unwrap();
    fs::write(root.join(&manifest.cases[0].forecast.reference), b"{}\n").unwrap();
    assert!(verified_calibration(&root, calibration.artifact().clone()).is_err());

    let calibration = verified_fixture_calibration(&root);
    fs::write(root.join(&calibration.artifact().reference), b"{}\n").unwrap();
    assert!(verified_calibration(&root, calibration.artifact().clone()).is_err());
}

#[test]
fn retained_issue_calibration_replays_real_sources_and_fails_closed() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let report = artifact_reference(&root, ".csdlc/evidence/5822/calibration-report.json").unwrap();
    let calibration = verified_calibration(&root, report).unwrap();
    assert_eq!(calibration.report().case_count, 0);
    assert!(!calibration.report().calibrated);
}

#[test]
fn missing_or_failed_calibration_forces_static_fallback() {
    let candidates = vec![
        observation(21, 1000, 100, 20_000),
        observation(22, 1100, 110, 22_000),
        observation(23, 1200, 120, 24_000),
    ];
    assert_eq!(
        forecast(99, key(), &candidates, fallback(), None)
            .unwrap()
            .method,
        EstimateMethod::StaticPlanningProfileFallback
    );
    let root = fastwork_root("failed-calibration");
    let context = write_json(
        &root,
        ".csdlc/evidence/99/real-history.json",
        &serde_json::json!({"source":"retained_history","actual_comparison":"not_available"}),
    );
    let manifest = CalibrationManifest {
        schema: "csdlc.estimation_calibration_manifest.v1".into(),
        cases: vec![],
        context_sources: vec![context],
        tolerance_basis_points: 500,
    };
    let manifest_ref = write_json(
        &root,
        ".csdlc/evidence/99/manifest.json",
        &serde_json::to_value(&manifest).unwrap(),
    );
    let report = calibration_report(&[], 500, manifest_ref).unwrap();
    let report_ref = write_json(
        &root,
        ".csdlc/evidence/99/report.json",
        &serde_json::to_value(report).unwrap(),
    );
    let failed = verified_calibration(&root, report_ref).unwrap();
    assert!(!failed.report().calibrated);
    assert_eq!(
        forecast(99, key(), &candidates, fallback(), Some(&failed))
            .unwrap()
            .method,
        EstimateMethod::StaticPlanningProfileFallback
    );
}

#[test]
fn cycle_comparison_derives_basis_gates_and_totals_from_verified_artifacts() {
    let root = fastwork_root("cycle");
    let source = write_json(
        &root,
        ".csdlc/evidence/99/session.json",
        &serde_json::json!({"actual":"retained"}),
    );
    let cohort = |id: &str, active: u64| CycleTimeEvidence {
        schema: "csdlc.cycle_time_evidence.v1".into(),
        id: id.into(),
        comparison_key: key(),
        preserved_gates: vec!["validation".into(), "review".into()],
        issues: vec![CycleIssueTiming {
            issue: 1,
            active_work_seconds: Some(active),
            validation_seconds: Some(10),
            review_seconds: Some(10),
            ci_seconds: Some(10),
            wait_seconds: Some(0),
            reconnect_actions: Some(1),
        }],
        provenance: vec![source.clone()],
    };
    let baseline = write_json(
        &root,
        ".csdlc/evidence/99/baseline.json",
        &serde_json::to_value(cohort("baseline", 100)).unwrap(),
    );
    let candidate = write_json(
        &root,
        ".csdlc/evidence/99/candidate.json",
        &serde_json::to_value(cohort("candidate", 80)).unwrap(),
    );
    let comparison = compare_cycle_time(&root, &baseline, &candidate).unwrap();
    assert_eq!(comparison.status, CycleTimeComparisonStatus::Comparable);
    assert_eq!(comparison.elapsed_reduction_seconds, 20);

    fs::write(root.join(&source.reference), b"tampered\n").unwrap();
    assert!(compare_cycle_time(&root, &baseline, &candidate).is_err());
    let missing = ArtifactReference {
        reference: ".csdlc/evidence/99/missing.json".into(),
        digest: "a".repeat(64),
    };
    assert!(compare_cycle_time(&root, &baseline, &missing).is_err());
}

#[test]
fn retained_cycle_baseline_is_byte_verified_and_derived_from_real_evidence() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let artifact =
        artifact_reference(&root, ".csdlc/evidence/5822/cycle-time-baseline.json").unwrap();
    let baseline = load_cycle_time_evidence(&root, &artifact).unwrap();
    assert_eq!(baseline.issue_count, 1);
    assert_eq!(baseline.active_work_seconds, 321);
    assert!(baseline
        .unknown_components
        .contains(&"validation_seconds".into()));
}

#[test]
fn finish_records_available_actuals_instead_of_unconditional_unknowns() {
    let root = fastwork_root("finish");
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
    let predicted = forecast(5822, key(), &[], fallback(), None).unwrap();
    let forecast_ref = write_json(
        &root,
        ".csdlc/evidence/5822/forecast.json",
        &serde_json::to_value(&predicted).unwrap(),
    );
    let accepted = AcceptedEstimate {
        forecast_ref: forecast_ref.reference.clone(),
        forecast_digest: forecast_ref.digest.clone(),
        disposition: EstimateDisposition::Accepted,
        advisory_only: true,
        adjusted: None,
        operator_rationale: "verified fixture".into(),
    };
    let spp_path = target_cards.join("spp.values.json");
    let mut spp: CardValues = serde_json::from_slice(&fs::read(&spp_path).unwrap()).unwrap();
    let CardContent::Spp(values) = &mut spp.content else {
        panic!("SPP")
    };
    values.execution_estimates.advisory = Some(Box::new(accepted));
    fs::write(&spp_path, serde_json::to_vec(&spp).unwrap()).unwrap();
    source_manifest(&root, 5822);
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
    let result = retain_terminal_estimation_outcome(&store, &terminal);
    assert_eq!(
        result.status,
        TerminalEstimationStatus::Recorded,
        "{}",
        result.detail
    );
    let outcome: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".git/csdlc-v2/derived-terminal/5822.estimation.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(outcome["elapsed_seconds"]["actual"], 1000);
    assert_eq!(outcome["validation_seconds"]["actual"], 110);
    assert_eq!(outcome["total_tokens"]["actual"], 22000);
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
}
