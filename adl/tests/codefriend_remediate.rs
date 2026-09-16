//! PVF lane: runtime. Deterministic local CPU/filesystem proof for CodeFriend
//! remediation planning. No provider calls, source mutation, network, or issue
//! publication.

use adl::codefriend::{
    actions::remediation::{plan, read_plan_from_file, validate_plan, RemediationPlan},
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
        "target/codefriend-remediate-tests/{name}-{}-{}",
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
        scope_limits: vec![],
    }
}

fn synthesis() -> ReviewSynthesis {
    ReviewSynthesis {
        schema: SYNTHESIS_SCHEMA.to_string(),
        review_record_digest: "review-digest".to_string(),
        run_id: "remediation-test-run".to_string(),
        repository: "https://example.com/team/remediation-target".to_string(),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        scope_digest: "scope-digest".to_string(),
        lane_count: 4,
        input_finding_count: 2,
        synthesized_findings: vec![
            finding(
                "finding-a",
                "adl/src/codefriend/review/runner.rs:85",
                &["evidence-a"],
            ),
            finding(
                "finding-b",
                "adl/src/codefriend/review/runner.rs:501",
                &["evidence-b"],
            ),
        ],
    }
}

#[test]
fn remediation_plan_orders_traceable_bounded_actions() {
    let plan = plan(&synthesis()).unwrap();
    assert_eq!(plan.schema, "codefriend.remediation_plan.v1");
    assert_eq!(plan.actions.len(), 2);
    assert!(plan.omitted_findings.is_empty());
    assert_eq!(plan.action_order.len(), 2);
    assert!(plan
        .actions
        .iter()
        .all(|action| action.assignment_status == "unassigned"
            && action.owner_role == "codefriend-owner"
            && action
                .acceptance_criteria
                .iter()
                .any(|criterion| criterion.contains(&action.finding_id))));
    assert!(plan
        .actions
        .iter()
        .any(|action| !action.dependencies.is_empty()));
}

#[test]
fn remediation_plan_omits_untraceable_repository_paths() {
    let mut synthesis = synthesis();
    synthesis.synthesized_findings.push(finding(
        "finding-c",
        "repository-wide concern without path",
        &["evidence-c"],
    ));
    let plan = plan(&synthesis).unwrap();
    assert_eq!(plan.actions.len(), 2);
    assert_eq!(plan.omitted_findings.len(), 1);
    assert_eq!(
        plan.omitted_findings[0].reason,
        "no_supported_repository_path_in_synthesized_finding"
    );
}

#[test]
fn remediation_reader_rejects_tampered_paths_acceptance_and_cycles() {
    let plan = plan(&synthesis()).unwrap();

    let mut bad_path = plan.clone();
    bad_path.actions[0].relevant_paths = vec!["../secret".to_string()];
    let err = validate_plan(&bad_path).unwrap_err().to_string();
    assert!(err.contains("unsupported_remediation_path"));

    let mut bad_acceptance = plan.clone();
    bad_acceptance.actions[0].acceptance_criteria = vec!["fix something".to_string()];
    let err = validate_plan(&bad_acceptance).unwrap_err().to_string();
    assert!(err.contains("untraceable_remediation_acceptance"));

    let mut bad_order = plan.clone();
    bad_order.action_order.reverse();
    let err = validate_plan(&bad_order).unwrap_err().to_string();
    assert!(err.contains("remediation_action_order_not_topological"));

    let mut cycle = plan;
    let first = cycle.actions[0].id.clone();
    let second = cycle.actions[1].id.clone();
    cycle.actions[0].dependencies = vec![second];
    cycle.actions[1].dependencies = vec![first];
    let err = validate_plan(&cycle).unwrap_err().to_string();
    assert!(err.contains("remediation_dependency_cycle"));
}

#[test]
fn installed_cli_generates_and_reads_remediation_plan() {
    let root = temp_dir("installed-cli");
    let input = root.join("synthesis.json");
    fs::write(&input, serde_json::to_vec_pretty(&synthesis()).unwrap()).unwrap();
    let out_dir = root.join("remediation-out");

    let output = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "remediation", "--input"])
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
    assert_eq!(summary["schema"], "codefriend.remediation_plan.v1");
    assert_eq!(summary["action_count"], 2);
    assert!(out_dir.join("synthesis.json").exists());
    assert!(out_dir.join("manifest.json").exists());
    let plan_path = out_dir.join("remediation-plan.json");
    assert!(plan_path.exists());

    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "remediation", "read", "--input"])
        .arg(&plan_path)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        read.status.success(),
        "{}",
        String::from_utf8_lossy(&read.stderr)
    );
    let read_plan: RemediationPlan = serde_json::from_slice(&read.stdout).unwrap();
    assert_eq!(read_plan.actions.len(), 2);
    assert_eq!(read_plan_from_file(&plan_path).unwrap(), read_plan);

    let duplicate = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "remediation", "--input"])
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr)
        .contains("remediation_output_directory_already_exists"));
}
