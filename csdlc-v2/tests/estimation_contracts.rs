use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use csdlc_v2::cards::{CardContent, CardValues};
use csdlc_v2::estimation::{
    artifact_reference, calibration_report, compare_cycle_time, forecast,
    load_observation_manifest, load_verified_json, terminal_outcome, validate_reference,
    verified_calibration, AcceptedEstimate, ArtifactReference, Availability, BacktestCase,
    CalibrationCaseArtifacts, CalibrationManifest, ComparableKey, CycleTimeComparisonStatus,
    CycleTimeEvidence, DriftState, EstimateDisposition, EstimateMethod, MetricObservation,
    Observation, ObservationManifest, ObservationSource, StaticEstimate, TerminalOutcome,
    ValidationSourceManifest, OBSERVATION_SCHEMA,
};
use csdlc_v2::{
    retain_terminal_estimation_outcome, DerivedTerminalEnvelope, FinishDisposition, IssueRecord,
    Store, TerminalEstimationStatus,
};

fn key() -> ComparableKey {
    ComparableKey {
        cohort: "danielbaustin/agent-design-language".into(),
        workflow_version: "csdlc.issue.index.v1".into(),
        model_era: "gpt-5-codex".into(),
    }
}

fn fallback() -> StaticEstimate {
    StaticEstimate {
        elapsed_seconds: 21_600,
        validation_seconds: 3_600,
        total_tokens: 80_000,
    }
}

fn fixture_root(name: &str) -> PathBuf {
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
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

fn seal_issue_record(value: serde_json::Value) -> serde_json::Value {
    let mut record: IssueRecord = serde_json::from_value(value).unwrap();
    record.digest.clear();
    record.digest = blake3::hash(&serde_json::to_vec(&record).unwrap())
        .to_hex()
        .to_string();
    serde_json::to_value(record).unwrap()
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
        operator_wait_seconds: MetricObservation::known(
            0,
            ObservationSource::OperatorAnnotation,
            format!(".csdlc/evidence/{issue}/operator.json"),
        ),
        reconnect_actions: MetricObservation::known(
            0,
            ObservationSource::OperatorAnnotation,
            format!(".csdlc/evidence/{issue}/operator.json"),
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
    let mut lifecycle_value: serde_json::Value = serde_json::from_slice(
        &fs::read(repository_root.join(".csdlc/issues/5822/index.json")).unwrap(),
    )
    .unwrap();
    lifecycle_value["issue"] = issue.into();
    lifecycle_value["phase"] = "closed_out".into();
    lifecycle_value["review"] = serde_json::json!({
        "reviewer":"independent", "scope":["focused"], "reviewed_revision":"a",
        "findings":[], "residual_risks":[], "completed":true,
        "non_substantive_proof":null
    });
    lifecycle_value["publication"] = serde_json::json!({
        "repository":"agent-logic/agent-design-language", "issue":issue,
        "pull_request":6000, "url":"https://example.invalid/pr/6000", "base":"main",
        "head":"codex/test", "revision":"a", "draft":false, "observed_state":"merged"
    });
    lifecycle_value["terminal"] = serde_json::json!({
        "pull_request":6000, "disposition":"merged", "observed_sha":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "observed_state":"merged", "receipt_path":".csdlc/receipts/test.json",
        "branch":"codex/test", "worktree":null
    });
    let lifecycle_value = seal_issue_record(lifecycle_value);
    let lifecycle = write_json(root, &format!("{base}/lifecycle.json"), &lifecycle_value);
    let github = write_json(
        root,
        &format!("{base}/github.json"),
        &serde_json::json!({
            "number":6000,
            "body":format!("Closes danielbaustin/agent-design-language#{issue}"),
            "created_at":"2026-08-06T00:00:00Z", "closed_at":"2026-08-06T00:02:00Z",
            "merged_at":"2026-08-06T00:02:00Z",
            "ci_started_at":"2026-08-06T00:00:00Z",
            "ci_completed_at":"2026-08-06T00:01:00Z",
            "base":{"repo":{"full_name":"agent-logic/agent-design-language"}}
        }),
    );
    let validation_report = write_json(
        root,
        &format!("{base}/pvf.json"),
        &serde_json::json!({
            "schema":"csdlc.pvf_execution_report.v1", "disposition":"local_pass",
            "selected_waves":[["focused"]], "evidence":[{
                "lane":"focused", "command":["cargo","nextest"], "purpose":"focused proof",
                "status":"passed", "duration_ms":3600000, "log_ref":null, "redaction_ok":true,
                "redactions_applied":0, "path_hygiene_ok":true
            }]
        }),
    );
    let validation = write_json(
        root,
        &format!("{base}/validation-source.json"),
        &serde_json::to_value(ValidationSourceManifest {
            schema: "csdlc.validation_source_manifest.v1".into(),
            issue_record: lifecycle.clone(),
            execution_report: validation_report,
        })
        .unwrap(),
    );
    let session = write_json(
        root,
        &format!("{base}/session.json"),
        &serde_json::json!({
            "schema_version":"issue_goal_metrics.v1", "issue_number":issue,
            "data_source":"codex_goal_tool", "model_ref":"gpt-5-codex",
            "started_at":"2026-08-06T00:00:00Z",
            "completed_at":"2026-08-06T06:00:00Z", "elapsed_seconds":21600,
            "active_work_seconds":18000, "elapsed_availability":"known",
            "active_work_availability":"known", "token_usage":{
                "availability":"known", "total_tokens":80000
            }
        }),
    );
    let operator = write_json(
        root,
        &format!("{base}/operator.json"),
        &serde_json::json!({
            "schema":"csdlc.operator_timing_source.v1", "issue":issue,
            "source":"operator_annotation", "wait_intervals":[{
                "started_unix_seconds":100, "completed_unix_seconds":110
            }], "reconnect_actions":2, "interrupted":false
        }),
    );
    let manifest = ObservationManifest {
        schema: "csdlc.estimation_observation_manifest.v1".into(),
        issue,
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

fn rewrite_session_measurements(
    root: &Path,
    manifest_ref: &ArtifactReference,
    completed_at: &str,
    elapsed_seconds: u64,
    active_work_seconds: u64,
    total_tokens: u64,
) -> ArtifactReference {
    let mut manifest: ObservationManifest = load_verified_json(root, manifest_ref).unwrap();
    let session_ref = manifest.session.clone().unwrap();
    let mut session: serde_json::Value = load_verified_json(root, &session_ref).unwrap();
    session["completed_at"] = completed_at.into();
    session["elapsed_seconds"] = elapsed_seconds.into();
    session["active_work_seconds"] = active_work_seconds.into();
    session["token_usage"]["total_tokens"] = total_tokens.into();
    manifest.session = Some(write_json(root, &session_ref.reference, &session));
    write_json(
        root,
        &manifest_ref.reference,
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
        let actual_ref = source_manifest(root, issue);
        let actual_ref = rewrite_session_measurements(
            root,
            &actual_ref,
            "2026-08-06T05:55:00Z",
            21_300,
            17_700,
            79_000,
        );
        let actual = load_observation_manifest(root, &actual_ref).unwrap();
        let outcome = terminal_outcome(&predicted, forecast_ref.clone(), &actual).unwrap();
        let outcome_ref = write_json(
            root,
            &format!(".csdlc/evidence/99/{issue}-outcome.json"),
            &serde_json::to_value(&outcome).unwrap(),
        );
        entries.push(CalibrationCaseArtifacts {
            issue,
            forecast: forecast_ref,
            actual_observation: actual_ref,
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
    let root = fixture_root("sources");
    let manifest = source_manifest(&root, 5822);
    let joined = load_observation_manifest(&root, &manifest).unwrap();
    assert_eq!(joined.issue, 5822);
    assert_eq!(joined.elapsed_seconds.value, Some(21_600));
    assert_eq!(joined.validation_seconds.value, Some(3_600));
    assert_eq!(joined.pr_wait_seconds.value, Some(120));
    assert_eq!(joined.operator_wait_seconds.value, Some(10));
    assert_eq!(joined.reconnect_actions.value, Some(2));
    assert_eq!(joined.total_tokens.value, Some(80_000));

    fs::write(root.join(".csdlc/evidence/5822/session.json"), b"{}\n").unwrap();
    assert!(load_observation_manifest(&root, &manifest).is_err());
}

#[test]
fn github_adapter_requires_qualified_linkage_and_verified_publication_repository() {
    let root = fixture_root("github-authority");
    let manifest_ref = source_manifest(&root, 5822);
    let mut manifest: ObservationManifest = load_verified_json(&root, &manifest_ref).unwrap();
    let github_ref = manifest.github.clone().unwrap();
    let mut github: serde_json::Value = load_verified_json(&root, &github_ref).unwrap();
    github["body"] = "Closes #5822".into();
    manifest.github = Some(write_json(&root, &github_ref.reference, &github));
    let unqualified = write_json(
        &root,
        ".csdlc/evidence/5822/unqualified-github-manifest.json",
        &serde_json::to_value(&manifest).unwrap(),
    );
    assert!(load_observation_manifest(&root, &unqualified).is_err());

    let manifest_ref = source_manifest(&root, 5822);
    let mut manifest: ObservationManifest = load_verified_json(&root, &manifest_ref).unwrap();
    let github_ref = manifest.github.clone().unwrap();
    let mut github: serde_json::Value = load_verified_json(&root, &github_ref).unwrap();
    github["base"]["repo"]["full_name"] = "wrong/repository".into();
    manifest.github = Some(write_json(&root, &github_ref.reference, &github));
    let wrong_repository = write_json(
        &root,
        ".csdlc/evidence/5822/wrong-repository-github-manifest.json",
        &serde_json::to_value(&manifest).unwrap(),
    );
    assert!(load_observation_manifest(&root, &wrong_repository).is_err());
}

#[test]
fn session_adapter_emits_schema_drift_and_forces_static_fallback() {
    let root = fixture_root("session-schema-drift");
    let manifest_ref = source_manifest(&root, 60);
    let mut manifest: ObservationManifest = load_verified_json(&root, &manifest_ref).unwrap();
    let session_ref = manifest.session.clone().unwrap();
    let mut session: serde_json::Value = load_verified_json(&root, &session_ref).unwrap();
    session["elapsed_availability"] = "future_availability".into();
    manifest.session = Some(write_json(&root, &session_ref.reference, &session));
    let unexpected_availability = write_json(
        &root,
        ".csdlc/evidence/60/unexpected-availability-manifest.json",
        &serde_json::to_value(&manifest).unwrap(),
    );
    let unexpected = load_observation_manifest(&root, &unexpected_availability).unwrap();
    assert_eq!(
        unexpected.elapsed_seconds.availability,
        Availability::SchemaDrift
    );
    assert_eq!(unexpected.elapsed_seconds.value, None);

    let mut drifted = Vec::new();
    for issue in [61_u64, 62, 63] {
        let manifest_ref = source_manifest(&root, issue);
        let mut manifest: ObservationManifest = load_verified_json(&root, &manifest_ref).unwrap();
        let session_ref = manifest.session.clone().unwrap();
        let mut session: serde_json::Value = load_verified_json(&root, &session_ref).unwrap();
        session["schema_version"] = "issue_goal_metrics.v2".into();
        manifest.session = Some(write_json(&root, &session_ref.reference, &session));
        let drifted_ref = write_json(
            &root,
            &format!(".csdlc/evidence/{issue}/drifted-manifest.json"),
            &serde_json::to_value(&manifest).unwrap(),
        );
        let observation = load_observation_manifest(&root, &drifted_ref).unwrap();
        assert_eq!(
            observation.elapsed_seconds.availability,
            Availability::SchemaDrift
        );
        assert_eq!(
            observation.active_work_seconds.availability,
            Availability::SchemaDrift
        );
        assert_eq!(
            observation.total_tokens.availability,
            Availability::SchemaDrift
        );
        drifted.push(observation);
    }
    let calibration = verified_fixture_calibration(&root);
    let forecast = forecast(99, key(), &drifted, fallback(), Some(&calibration)).unwrap();
    assert_eq!(
        forecast.method,
        EstimateMethod::StaticPlanningProfileFallback
    );
    assert_eq!(forecast.drift, DriftState::SchemaDrift);
}

#[test]
fn source_identity_mismatch_and_missing_sources_fail_closed() {
    let root = fixture_root("source-identity");
    let manifest = source_manifest(&root, 5822);
    let manifest_value: ObservationManifest = load_verified_json(&root, &manifest).unwrap();
    let session = manifest_value.session.unwrap();
    fs::write(root.join(&session.reference), serde_json::to_vec(&serde_json::json!({
        "schema_version":"issue_goal_metrics.v1", "issue_number":9999,
        "data_source":"codex_goal_tool", "model_ref":"gpt-5-codex",
        "started_at":"2026-08-06T00:00:00Z",
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
            "data_source":"codex_goal_tool", "model_ref":"gpt-5-codex",
            "started_at":"2026-08-06T00:00:00Z",
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

    let clean = source_manifest(&root, 5822);
    let mut clean_manifest: ObservationManifest = load_verified_json(&root, &clean).unwrap();
    let validation_ref = clean_manifest.validation.clone().unwrap();
    let mut validation: ValidationSourceManifest =
        load_verified_json(&root, &validation_ref).unwrap();
    let other = source_manifest(&root, 9999);
    let other_manifest: ObservationManifest = load_verified_json(&root, &other).unwrap();
    validation.issue_record = other_manifest.lifecycle.unwrap();
    clean_manifest.validation = Some(write_json(
        &root,
        ".csdlc/evidence/5822/forged-validation-source.json",
        &serde_json::to_value(validation).unwrap(),
    ));
    let forged_validation = write_json(
        &root,
        ".csdlc/evidence/5822/forged-validation-observation.json",
        &serde_json::to_value(clean_manifest).unwrap(),
    );
    assert!(load_observation_manifest(&root, &forged_validation).is_err());
    assert!(artifact_reference(&root, ".csdlc/evidence/5822/missing.json").is_err());
}

#[test]
fn verified_calibration_is_loader_only_reproducible_and_tamper_evident() {
    let root = fixture_root("calibration");
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

    let calibration = verified_fixture_calibration(&root);
    let mut manifest: CalibrationManifest =
        load_verified_json(&root, &calibration.report().source_manifest).unwrap();
    let first = &mut manifest.cases[0];
    let mut favorable: TerminalOutcome = load_verified_json(&root, &first.outcome).unwrap();
    for metric in [
        &mut favorable.elapsed_seconds,
        &mut favorable.validation_seconds,
        &mut favorable.total_tokens,
    ] {
        metric.actual = Some(metric.forecast);
        metric.absolute_error = Some(0);
        metric.error_basis_points = Some(0);
    }
    first.outcome = write_json(
        &root,
        &first.outcome.reference,
        &serde_json::to_value(favorable).unwrap(),
    );
    let manifest_ref = write_json(
        &root,
        &calibration.report().source_manifest.reference,
        &serde_json::to_value(&manifest).unwrap(),
    );
    let mut forged_cases = Vec::new();
    for entry in &manifest.cases {
        forged_cases.push(BacktestCase {
            issue: entry.issue,
            forecast: load_verified_json(&root, &entry.forecast).unwrap(),
            outcome: load_verified_json(&root, &entry.outcome).unwrap(),
        });
    }
    let forged_report = calibration_report(&forged_cases, 500, manifest_ref).unwrap();
    let forged_report_ref = write_json(
        &root,
        ".csdlc/evidence/99/forged-calibration.json",
        &serde_json::to_value(forged_report).unwrap(),
    );
    assert!(verified_calibration(&root, forged_report_ref).is_err());
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
    let root = fixture_root("failed-calibration");
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
    let root = fixture_root("cycle");
    let baseline_observation = source_manifest(&root, 101);
    let candidate_observation = source_manifest(&root, 102);
    let mut candidate_manifest: ObservationManifest =
        load_verified_json(&root, &candidate_observation).unwrap();
    let candidate_session = candidate_manifest.session.clone().unwrap();
    let mut session: serde_json::Value = load_verified_json(&root, &candidate_session).unwrap();
    session["completed_at"] = "2026-08-06T05:00:00Z".into();
    session["elapsed_seconds"] = 18_000.into();
    session["active_work_seconds"] = 14_400.into();
    let rewritten_session = write_json(&root, &candidate_session.reference, &session);
    candidate_manifest.session = Some(rewritten_session);
    let candidate_observation = write_json(
        &root,
        &candidate_observation.reference,
        &serde_json::to_value(candidate_manifest).unwrap(),
    );
    let cohort = |observation: ArtifactReference| CycleTimeEvidence {
        schema: "csdlc.cycle_time_evidence.v2".into(),
        observations: vec![observation],
    };
    let baseline = write_json(
        &root,
        ".csdlc/evidence/99/baseline.json",
        &serde_json::to_value(cohort(baseline_observation)).unwrap(),
    );
    let candidate = write_json(
        &root,
        ".csdlc/evidence/99/candidate.json",
        &serde_json::to_value(cohort(candidate_observation.clone())).unwrap(),
    );
    let comparison = compare_cycle_time(&root, &baseline, &candidate).unwrap();
    assert_eq!(comparison.status, CycleTimeComparisonStatus::Comparable);
    assert_eq!(comparison.elapsed_reduction_seconds, 3_600);

    let same_elapsed_observation = rewrite_session_measurements(
        &root,
        &source_manifest(&root, 103),
        "2026-08-06T06:00:00Z",
        21_600,
        1_000,
        80_000,
    );
    let same_elapsed = write_json(
        &root,
        ".csdlc/evidence/99/same-elapsed.json",
        &serde_json::to_value(cohort(same_elapsed_observation)).unwrap(),
    );
    let same_elapsed_comparison = compare_cycle_time(&root, &baseline, &same_elapsed).unwrap();
    assert_eq!(
        same_elapsed_comparison.status,
        CycleTimeComparisonStatus::Comparable
    );
    assert_eq!(same_elapsed_comparison.elapsed_reduction_seconds, 0);

    fs::write(root.join(&candidate_session.reference), b"tampered\n").unwrap();
    assert!(compare_cycle_time(&root, &baseline, &candidate).is_err());
    let missing = ArtifactReference {
        reference: ".csdlc/evidence/99/missing.json".into(),
        digest: "a".repeat(64),
    };
    assert!(compare_cycle_time(&root, &baseline, &missing).is_err());
}

#[test]
fn retained_cycle_boundary_records_one_terminal_baseline_without_a_reduction_claim() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let baseline: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/5822/cycle-time-baseline.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(baseline["issue"], 5778);
    assert_eq!(baseline["measured"], true);
    assert_eq!(baseline["terminal"], true);
    assert_eq!(baseline["unknown_components"], serde_json::json!([]));
    assert_eq!(baseline["candidate_ref"], serde_json::Value::Null);
    assert_eq!(
        baseline["elapsed_reduction_seconds"],
        serde_json::Value::Null
    );
    assert_eq!(
        baseline["reconnect_action_reduction"],
        serde_json::Value::Null
    );
    assert_eq!(baseline["reduction_claimed"], false);
    let boundary: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/5822/cycle-time-evidence-boundary.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        boundary["status"],
        "terminal_baseline_measured_candidate_absent"
    );
    assert_eq!(boundary["ac7_satisfied"], true);
    assert_eq!(
        boundary["baseline_artifact"],
        ".csdlc/evidence/5822/cycle-time-baseline.json"
    );
    assert_eq!(boundary["candidate_ref"], serde_json::Value::Null);
    assert_eq!(
        boundary["elapsed_reduction_seconds"],
        serde_json::Value::Null
    );
    assert_eq!(
        boundary["reconnect_action_reduction"],
        serde_json::Value::Null
    );
}

#[test]
fn finish_records_available_actuals_even_when_the_estimate_is_deferred() {
    let root = fixture_root("finish");
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
        disposition: EstimateDisposition::Deferred,
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
        TerminalEstimationStatus::Deferred,
        "{}",
        result.detail
    );
    let outcome: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".git/csdlc-v2/derived-terminal/5822.estimation.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(outcome["elapsed_seconds"]["actual"], 21600);
    assert_eq!(outcome["validation_seconds"]["actual"], 3600);
    assert_eq!(outcome["total_tokens"]["actual"], 80000);
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
