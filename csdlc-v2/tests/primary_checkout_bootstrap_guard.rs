use std::fs;
use std::path::Path;
use std::process::Command;

use csdlc_v2::cards::{PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    initialize_native_json, BootstrapRequest, ErrorCode, InitialCardInput, PlanningProfile, Store,
};

const ISSUE: u64 = 544;

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn install_native_authority(root: &Path) {
    fs::create_dir_all(root.join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(root.join("csdlc-v2/operator")).unwrap();
    fs::write(
        root.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        root.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

fn init_repo(root: &Path) {
    install_native_authority(root);
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(
        root.join("docs/validate.sh"),
        "#!/usr/bin/env bash\nset -e\n",
    )
    .unwrap();
    fs::write(
        root.join("docs/onboarding.md"),
        "primary checkout isolated staging checkout\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("csdlc-v2")).unwrap();
    fs::write(
        root.join("csdlc-v2/README.md"),
        "primary checkout isolated staging checkout\n",
    )
    .unwrap();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "fixture"]);
}

fn bootstrap_request() -> BootstrapRequest {
    BootstrapRequest {
        issue: ISSUE,
        repository: "agent-logic/agent-design-language".into(),
        actor: "test".into(),
        design_path: ".csdlc/prepared/issues/544/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/544/diagram.mmd".into(),
        design_reviewer: "fresh-session:00000000-0000-4000-8000-000000000001".into(),
        design_approved: false,
        initial: InitialCardInput {
            title: "Primary checkout bootstrap guard".into(),
            slug: "primary-checkout-bootstrap-guard".into(),
            version: "v0.92.1".into(),
            goal: "Reject primary checkout bootstrap.".into(),
            required_outcome:
                "Bootstrap state is created only from isolated non-primary checkouts.".into(),
            declared_scope: vec!["csdlc-v2/src/lifecycle.rs".into()],
            authority_boundary: vec!["primary checkout is inspection-only".into()],
            operator_constraints: vec!["bootstrap from isolated staging checkout".into()],
            task_boundary: "Focused lifecycle guard.".into(),
            deliverables: vec![
                "csdlc-v2/src/lifecycle.rs".into(),
                "docs/validate.sh".into(),
            ],
            acceptance_criteria: vec!["AC-1: primary checkout is rejected".into()],
            dependencies: vec!["git worktree topology".into()],
            repo_inputs: vec!["csdlc-v2/src/lifecycle.rs".into()],
            non_goals: vec!["bind policy change".into()],
            plan_summary: "Guard then bind.".into(),
            steps: vec![PlanStep {
                id: "S1".into(),
                action: "guard bootstrap".into(),
                acceptance_ids: vec!["AC-1".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec![
                "csdlc-v2/src/lifecycle.rs".into(),
                "docs/validate.sh".into(),
            ],
            invariants: vec!["primary checkout stays clean".into()],
            risks: vec!["ambiguous topology".into()],
            planning_profile: PlanningProfile::Small,
            stop_conditions: vec!["topology cannot be proven".into()],
            validation_lanes: vec![ValidationLane {
                lane: "docs-validate".into(),
                proof_role: "fixture".into(),
                acceptance_ids: vec!["AC-1".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 30,
                budget_tokens: 100,
                argv: vec!["bash".into(), "docs/validate.sh".into()],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed on primary checkout or ambiguous topology.".into(),
            review_prompts: vec!["Review topology guard.".into()],
            review_scope: "csdlc-v2/src/lifecycle.rs".into(),
        },
    }
}

fn assert_no_issue_residue(root: &Path) {
    assert!(!root.join(".csdlc/issues/544").exists());
    assert!(!root.join(".csdlc/prepared/issues/544").exists());
    assert!(!root.join(".csdlc/locks/544.lock").exists());
}

#[test]
fn primary_checkout_bootstrap_rejects_before_issue_residue() {
    let temp = tempfile::tempdir().unwrap();
    init_repo(temp.path());

    let error = initialize_native_json(
        &Store::new(temp.path()),
        &serde_json::to_vec(&bootstrap_request()).unwrap(),
    )
    .expect_err("primary checkout bootstrap should fail");

    assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    assert!(error.message.contains("primary checkout"));
    assert_no_issue_residue(temp.path());
}

#[test]
fn non_primary_checkout_bootstrap_succeeds_and_is_idempotent() {
    let temp = tempfile::tempdir().unwrap();
    let primary = temp.path().join("primary");
    let staging = temp.path().join("staging");
    fs::create_dir_all(&primary).unwrap();
    init_repo(&primary);
    git(
        &primary,
        &[
            "worktree",
            "add",
            "--detach",
            staging.to_str().unwrap(),
            "HEAD",
        ],
    );

    let store = Store::new(&staging);
    let request = serde_json::to_vec(&bootstrap_request()).unwrap();
    let first = initialize_native_json(&store, &request).expect("staging bootstrap");
    let second = initialize_native_json(&store, &request).expect("idempotent staging bootstrap");

    assert_eq!(first.issue, ISSUE);
    assert_eq!(first.digest, second.digest);
    assert!(staging.join(".csdlc/issues/544/index.json").is_file());
    assert_no_issue_residue(&primary);
}

#[test]
fn operator_docs_name_isolated_staging_bootstrap() {
    let onboarding = include_str!("../../docs/onboarding.md");
    let readme = include_str!("../README.md");
    for text in [onboarding, readme] {
        assert!(text.contains("primary checkout"));
        assert!(text.contains("inspection-only"));
        assert!(text.contains("isolated"));
        assert!(text.contains("staging checkout"));
    }
}
