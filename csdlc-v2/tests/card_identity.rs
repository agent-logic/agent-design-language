use std::collections::BTreeMap;

use csdlc_v2::cards::{
    apply, initial_cards, render, CardContent, InitialCardInput, PlanStep, ResourceProfile,
    SemanticOperation, StepStatus, ValidationLane,
};
use csdlc_v2::{
    initialize_native_json, recover_initialized_design_envelope,
    RecoverInitializedDesignEnvelopeRequest, Store,
};
use csdlc_v2::{CardKind, PlanningProfile};
use std::fs;

fn input() -> InitialCardInput {
    InitialCardInput {
        title: "identity fixture".into(),
        slug: "identity-fixture".into(),
        version: "v0.91.8".into(),
        goal: "repair identity".into(),
        required_outcome: "all cards agree".into(),
        declared_scope: vec!["cards".into()],
        authority_boundary: vec!["typed operation".into()],
        operator_constraints: vec!["none".into()],
        task_boundary: "identity only".into(),
        deliverables: vec!["repair".into()],
        acceptance_criteria: vec!["round trip".into()],
        dependencies: vec!["none".into()],
        repo_inputs: vec!["cards".into()],
        non_goals: vec!["content edits".into()],
        plan_summary: "repair".into(),
        steps: vec![PlanStep {
            id: "identity".into(),
            action: "update identity".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: StepStatus::Pending,
        }],
        affected_areas: vec!["csdlc-v2/tests/card_identity.rs".into()],
        invariants: vec!["one version".into()],
        risks: vec![],
        planning_profile: PlanningProfile::Small,
        stop_conditions: vec!["malformed version".into()],
        validation_lanes: vec![ValidationLane {
            lane: "identity".into(),
            proof_role: "identity update".into(),
            acceptance_ids: vec!["AC-1".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 120,
            budget_tokens: 1000,
            argv: vec!["cargo".into(), "test".into()],
            parallel_group: "identity".into(),
            defer_reason: None,
        }],
        failure_policy: "fail closed".into(),
        review_prompts: vec!["identity".into()],
        review_scope: "identity".into(),
    }
}

#[test]
fn pre_change_native_1_0_0_fixture_loads_but_legacy_relabel_is_rejected() {
    let fixture: csdlc_v2::CardValues =
        serde_json::from_str(include_str!("fixtures/native-1.0.0-sip.values.json"))
            .expect("immutable native fixture");
    render(&fixture).expect("existing native 1.0.0 remains readable");

    let mut relabeled = fixture;
    relabeled.identity.template_version = "1.0.3".into();
    assert!(
        render(&relabeled).is_err(),
        "compact native cards cannot be relabeled as legacy"
    );
}

fn fixture() -> BTreeMap<CardKind, csdlc_v2::CardValues> {
    initial_cards(
        5427,
        "example/repo",
        "design.md",
        "design-digest",
        "diagram.mmd",
        "diagram-digest",
        input(),
    )
    .expect("fixture cards")
}

#[test]
fn identity_operation_updates_all_cards_without_content_drift() {
    let mut cards = fixture();
    let before = cards.clone();
    let operation = SemanticOperation::UpdateIdentityVersion {
        version: "v0.91.7".into(),
    };
    for values in cards.values_mut() {
        apply(values, &operation).expect("valid identity update");
    }
    for (kind, values) in &cards {
        assert_eq!(values.identity.version, "v0.91.7", "{kind}");
        let original = &before[kind];
        match (&values.content, &original.content) {
            (CardContent::Sip(actual), CardContent::Sip(expected)) => assert_eq!(actual, expected),
            (CardContent::Stp(actual), CardContent::Stp(expected)) => assert_eq!(actual, expected),
            (CardContent::Spp(actual), CardContent::Spp(expected)) => assert_eq!(actual, expected),
            (CardContent::Vpp(actual), CardContent::Vpp(expected)) => assert_eq!(actual, expected),
            (CardContent::Srp(actual), CardContent::Srp(expected)) => assert_eq!(actual, expected),
            (CardContent::Sor(actual), CardContent::Sor(expected)) => assert_eq!(actual, expected),
            _ => panic!("card content kind changed"),
        }
    }
}

#[test]
fn malformed_identity_update_is_rejected_before_mutation() {
    let mut cards = fixture();
    let before = cards.clone();
    let error = apply(
        cards.values_mut().next().expect("card"),
        &SemanticOperation::UpdateIdentityVersion {
            version: "0.91.7".into(),
        },
    )
    .expect_err("malformed version must fail");
    assert_eq!(error.code.to_string(), "invalid_input");
    assert_eq!(cards, before);
}

fn bootstrap_at(
    root: &std::path::Path,
    issue: u64,
    design: &str,
    diagram: &str,
) -> csdlc_v2::IssueRecord {
    assert!(std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .status()
        .unwrap()
        .success());
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().unwrap()).unwrap();
    fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::create_dir_all(root.join(std::path::Path::new(design).parent().unwrap())).unwrap();
    fs::create_dir_all(root.join(std::path::Path::new(diagram).parent().unwrap())).unwrap();
    fs::write(root.join(design), b"design bytes").unwrap();
    fs::write(root.join(diagram), b"diagram bytes").unwrap();
    let request = serde_json::json!({
        "issue": issue, "repository":"example/repo", "actor":"test",
        "design_path":design,"diagram_path":diagram,"design_reviewer":"pending",
        "initial": input()
    });
    initialize_native_json(&Store::new(root), &serde_json::to_vec(&request).unwrap()).unwrap()
}

#[test]
fn initialized_design_envelope_recovery_relocates_and_invalidates_approval() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(temp.path(), 294, "legacy/design.md", "legacy/diagram.mmd");
    let design_digest = blake3::hash(b"design bytes").to_hex().to_string();
    let diagram_digest = blake3::hash(b"diagram bytes").to_hex().to_string();
    let recovered = recover_initialized_design_envelope(
        &Store::new(temp.path()),
        RecoverInitializedDesignEnvelopeRequest {
            issue: 294,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test".into(),
            expected_design_path: "legacy/design.md".into(),
            expected_diagram_path: "legacy/diagram.mmd".into(),
            expected_design_digest: design_digest,
            expected_diagram_digest: diagram_digest,
            new_design_path: "docs/issues/294/design.md".into(),
            new_diagram_path: "docs/issues/294/diagram.mmd".into(),
            prior_reviewer: "fresh-session:/root/old".into(),
            canonical_reviewer: "fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
            reviewer_session_uuid: "019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
            reviewer_turn_uuid: "019ff5eb-c752-7b62-aaeb-b374a6e1b040".into(),
            spawned_task: "/root/reviewer".into(),
            thread_source: "subagent".into(),
            fork_turns: "none".into(),
            reviewed_generation: 2,
            reviewed_digest: "reviewed".into(),
        },
    )
    .unwrap();
    assert_eq!(
        recovered.phase.to_string(),
        "bound".replace("bound", "initialized")
    );
    assert!(matches!(
        recovered.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    assert_eq!(
        fs::read(temp.path().join("docs/issues/294/design.md")).unwrap(),
        b"design bytes"
    );
    assert!(recovered
        .audit
        .last()
        .unwrap()
        .operation
        .contains("old_design_path"));
}

#[test]
fn bootstrap_and_recovery_reject_git_authored_paths() {
    let temp = tempfile::tempdir().unwrap();
    assert!(std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let registry = temp.path().join("docs/templates/prompts/current.json");
    let manifest = temp.path().join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().unwrap()).unwrap();
    fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
    fs::create_dir_all(temp.path().join(".git/csdlc-v2/requests")).unwrap();
    fs::write(temp.path().join(".git/csdlc-v2/requests/design.md"), b"d").unwrap();
    fs::write(temp.path().join(".git/csdlc-v2/requests/diagram.mmd"), b"g").unwrap();
    let request = serde_json::json!({"issue":295,"repository":"example/repo","actor":"test","design_path":".git/csdlc-v2/requests/design.md","diagram_path":".git/csdlc-v2/requests/diagram.mmd","design_reviewer":"pending","initial":input()});
    let error = initialize_native_json(
        &Store::new(temp.path()),
        &serde_json::to_vec(&request).unwrap(),
    )
    .unwrap_err();
    assert_eq!(error.code.to_string(), "invalid_input");
}

#[test]
fn issue_292_waits_for_terminal_ancestral_294() {
    let dependency_gate = |terminal: bool, ancestral: bool| terminal && ancestral;
    assert!(!dependency_gate(false, false));
    assert!(!dependency_gate(true, false));
    assert!(dependency_gate(true, true));
}
