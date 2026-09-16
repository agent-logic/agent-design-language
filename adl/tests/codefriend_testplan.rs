//! PVF lane: runtime. Deterministic local CPU/filesystem proof for CodeFriend
//! test planning. No provider calls, source mutation, network, or issue
//! publication.

use adl::codefriend::{
    actions::test_plan::{plan, read_plan_from_file, validate_plan, TestPlan},
    evidence::contracts::{Confidence, Severity},
    review::synthesis::{ReviewSynthesis, SynthesisSource, SynthesizedFinding, SYNTHESIS_SCHEMA},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/codefriend-testplan-tests/{name}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn source(id: &str, evidence: &[&str]) -> SynthesisSource {
    SynthesisSource {
        finding_id: id.to_string(),
        perspective: "correctness".to_string(),
        rule: "bounded-rule".to_string(),
        severity: Severity::High,
        evidence: evidence.iter().map(|value| value.to_string()).collect(),
        rationale: "rationale retained from lane".to_string(),
        confidence: Confidence::Known(90),
        inference: "observed from admitted evidence".to_string(),
        limitations: vec![],
    }
}

fn finding(id: &str, anchor: &str, evidence: &[&str]) -> SynthesizedFinding {
    SynthesizedFinding {
        id: id.to_string(),
        semantic_anchor: anchor.to_string(),
        title: format!("finding {id}"),
        severity: Severity::High,
        severity_rationale: "correctness: rationale retained".to_string(),
        evidence: evidence.iter().map(|value| value.to_string()).collect(),
        sources: vec![source(&format!("source-{id}"), evidence)],
        disagreement: None,
        scope_limits: vec!["correctness: bounded fixture only".to_string()],
    }
}

fn synthesis() -> ReviewSynthesis {
    ReviewSynthesis {
        schema: SYNTHESIS_SCHEMA.to_string(),
        review_record_digest: "review-digest".to_string(),
        run_id: "test-plan-run".to_string(),
        repository: "https://example.com/team/test-plan-target".to_string(),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        scope_digest: "scope-digest".to_string(),
        lane_count: 4,
        input_finding_count: 2,
        synthesized_findings: vec![
            finding(
                "finding-a",
                "adl/src/codefriend/review/runner.rs:85",
                &["adl/src/codefriend/review/runner.rs:85", "evidence-a"],
            ),
            finding(
                "finding-b",
                "adl/src/cli/codefriend_cmd.rs:112",
                &["adl/src/cli/codefriend_cmd.rs:112", "evidence-b"],
            ),
        ],
    }
}

#[test]
fn test_plan_maps_every_traceable_finding_to_executable_case() {
    let plan = plan(&synthesis()).unwrap();
    assert_eq!(plan.schema, "codefriend.test_plan.v1");
    assert_eq!(plan.test_cases.len(), 2);
    assert!(plan.omitted_findings.is_empty());
    assert!(plan.test_cases.iter().all(|case| {
        case.proposed_test_location.contains("test")
            && case.validation_lane.contains("PVF")
            && case.resource_profile.contains("no provider credentials")
            && case.detection_rationale.contains(&case.finding_id)
            && !case.proposed_fixture.contains("TODO")
    }));
    assert!(plan
        .test_cases
        .iter()
        .any(|case| case.proposed_test_location == "adl/tests/codefriend_cli_regression.rs"));
}

#[test]
fn test_plan_omits_findings_without_repository_path() {
    let mut synthesis = synthesis();
    synthesis.synthesized_findings.push(finding(
        "finding-c",
        "repository-wide concern without path",
        &["evidence-c"],
    ));
    let plan = plan(&synthesis).unwrap();
    assert_eq!(plan.test_cases.len(), 2);
    assert_eq!(plan.omitted_findings.len(), 1);
    assert_eq!(
        plan.omitted_findings[0].reason,
        "no_supported_repository_path_for_test_location"
    );
}

#[test]
fn test_plan_reader_rejects_placeholder_untraceable_or_non_test_cases() {
    let plan = plan(&synthesis()).unwrap();

    let mut bad_location = plan.clone();
    bad_location.test_cases[0].proposed_test_location =
        "adl/src/codefriend/review/runner.rs".to_string();
    let err = validate_plan(&bad_location).unwrap_err().to_string();
    assert!(err.contains("test_plan_location_must_be_test_surface"));

    let mut bad_rationale = plan.clone();
    bad_rationale.test_cases[0].detection_rationale = "detects behavior".to_string();
    let err = validate_plan(&bad_rationale).unwrap_err().to_string();
    assert!(err.contains("untraceable_test_detection_rationale"));

    let mut placeholder = plan;
    placeholder.test_cases[0].proposed_fixture = "TODO".to_string();
    let err = validate_plan(&placeholder).unwrap_err().to_string();
    assert!(err.contains("placeholder_test_plan"));
}

#[test]
fn installed_cli_generates_and_reads_test_plan_without_source_mutation() {
    let root = temp_dir("installed-cli");
    let input = root.join("synthesis.json");
    fs::write(&input, serde_json::to_vec_pretty(&synthesis()).unwrap()).unwrap();
    let out_dir = root.join("test-plan-out");

    let output = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "--input"])
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["schema"], "codefriend.test_plan.v1");
    assert_eq!(summary["test_case_count"], 2);
    assert!(out_dir.join("synthesis.json").exists());
    assert!(out_dir.join("manifest.json").exists());
    let plan_path = out_dir.join("test-plan.json");
    assert!(plan_path.exists());

    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "read", "--input"])
        .arg(&plan_path)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        read.status.success(),
        "{}",
        String::from_utf8_lossy(&read.stderr)
    );
    let read_plan: TestPlan = serde_json::from_slice(&read.stdout).unwrap();
    assert_eq!(read_plan.test_cases.len(), 2);
    assert_eq!(read_plan_from_file(&plan_path).unwrap(), read_plan);

    let duplicate = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "--input"])
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr)
        .contains("test_plan_output_directory_already_exists"));
}
