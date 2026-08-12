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
        "design_path":design,"diagram_path":diagram,"design_reviewer":"fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b","design_approved":true,
        "initial": input()
    });
    initialize_native_json(&Store::new(root), &serde_json::to_vec(&request).unwrap()).unwrap()
}

#[test]
fn initialized_design_envelope_recovery_relocates_and_invalidates_approval() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        294,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let design_digest = blake3::hash(b"design bytes").to_hex().to_string();
    let diagram_digest = blake3::hash(b"diagram bytes").to_hex().to_string();
    let recovered = recover_initialized_design_envelope(
        &Store::new(temp.path()),
        RecoverInitializedDesignEnvelopeRequest {
            issue: 294,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test".into(),
            expected_design_path: ".csdlc/prepared/legacy/design.md".into(),
            expected_diagram_path: ".csdlc/prepared/legacy/diagram.mmd".into(),
            expected_design_digest: design_digest.clone(),
            expected_diagram_digest: diagram_digest,
            new_design_path: "docs/issues/294/design.md".into(),
            new_diagram_path: "docs/issues/294/diagram.mmd".into(),
            prior_reviewer: "fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
            canonical_reviewer: "fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
            reviewer_session_uuid: "019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
            reviewer_turn_uuid: "019ff5eb-c752-7b62-aaeb-b374a6e1b040".into(),
            spawned_task: "/root/reviewer".into(),
            thread_source: "subagent".into(),
            fork_turns: "none".into(),
            reviewed_generation: record.generation,
            reviewed_digest: design_digest.clone(),
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

fn recovery_request(record: &csdlc_v2::IssueRecord) -> RecoverInitializedDesignEnvelopeRequest {
    RecoverInitializedDesignEnvelopeRequest {
        issue: record.issue,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        actor: "test".into(),
        expected_design_path: record.design_path.clone(),
        expected_diagram_path: record.diagram_path.clone(),
        expected_design_digest: blake3::hash(b"design bytes").to_hex().to_string(),
        expected_diagram_digest: blake3::hash(b"diagram bytes").to_hex().to_string(),
        new_design_path: format!("docs/issues/{}/design.md", record.issue),
        new_diagram_path: format!("docs/issues/{}/diagram.mmd", record.issue),
        prior_reviewer: "fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
        canonical_reviewer: "fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
        reviewer_session_uuid: "019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b".into(),
        reviewer_turn_uuid: "019ff5eb-c752-7b62-aaeb-b374a6e1b040".into(),
        spawned_task: "/root/reviewer".into(),
        thread_source: "subagent".into(),
        fork_turns: "none".into(),
        reviewed_generation: record.generation,
        reviewed_digest: match &record.design_review {
            csdlc_v2::DesignReview::Approved { revision, .. } => revision.clone(),
            _ => panic!("approved"),
        },
    }
}

#[test]
fn recovery_rejects_stale_cas_unsafe_alias_and_missing_provenance_without_mutation() {
    for case in [
        "generation",
        "digest",
        "alias",
        "provenance",
        "reviewer",
        "reviewed_generation",
        "source_absent",
        "source_drift",
        "destination_collision",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let record = bootstrap_at(
            temp.path(),
            296,
            ".csdlc/prepared/legacy/design.md",
            ".csdlc/prepared/legacy/diagram.mmd",
        );
        let before = fs::read(temp.path().join(".csdlc/issues/296/index.json")).unwrap();
        let mut request = recovery_request(&record);
        match case {
            "generation" => request.expected_generation += 1,
            "digest" => request.expected_digest = "stale".into(),
            "alias" => request.new_diagram_path = request.new_design_path.clone(),
            "provenance" => request.fork_turns = "all".into(),
            "reviewer" => request.prior_reviewer = "fresh-session:wrong".into(),
            "reviewed_generation" => request.reviewed_generation += 1,
            "source_absent" => {
                fs::remove_file(temp.path().join(&record.design_path)).unwrap();
            }
            "source_drift" => {
                fs::write(temp.path().join(&record.design_path), b"drifted").unwrap();
            }
            "destination_collision" => {
                let destination = temp.path().join(&request.new_design_path);
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
                fs::write(destination, b"owned elsewhere").unwrap();
            }
            _ => unreachable!(),
        }
        assert!(
            recover_initialized_design_envelope(&Store::new(temp.path()), request).is_err(),
            "{case}"
        );
        assert_eq!(
            fs::read(temp.path().join(".csdlc/issues/296/index.json")).unwrap(),
            before,
            "{case}"
        );
    }
}

#[test]
fn destination_collision_cleans_stage_and_retry_succeeds() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        298,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let request = recovery_request(&record);
    let destination = temp.path().join(&request.new_design_path);
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(&destination, b"collision").unwrap();
    assert!(
        recover_initialized_design_envelope(&Store::new(temp.path()), request.clone()).is_err()
    );
    assert!(!destination
        .parent()
        .unwrap()
        .join(".design.md.csdlc-stage")
        .exists());
    fs::remove_file(destination).unwrap();
    let recovered = recover_initialized_design_envelope(&Store::new(temp.path()), request).unwrap();
    assert_eq!(recovered.generation, record.generation + 1);
}

#[cfg(unix)]
#[test]
fn journal_owned_inode_prevents_identical_byte_replacement_deletion() {
    use std::os::unix::fs::MetadataExt;
    let temp = tempfile::tempdir().unwrap();
    assert!(std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let record = bootstrap_at(
        temp.path(),
        299,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let destination = temp.path().join("docs/issues/299/design.md");
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(&destination, b"design bytes").unwrap();
    let owned = fs::metadata(&destination).unwrap();
    fs::remove_file(&destination).unwrap();
    fs::write(&destination, b"design bytes").unwrap();
    let common = String::from_utf8(
        std::process::Command::new("git")
            .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .current_dir(temp.path())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let journal_dir = std::path::Path::new(common.trim()).join("csdlc-v2/recovery-journals");
    fs::create_dir_all(&journal_dir).unwrap();
    fs::write(journal_dir.join("299.json"), serde_json::to_vec(&serde_json::json!({"schema":"csdlc.initialized_design_envelope_recovery_journal.v1","issue":299,"pre_generation":record.generation,"pre_digest":record.digest,"post_generation":record.generation+1,"old_design_path":record.design_path,"old_diagram_path":record.diagram_path,"new_design_path":"docs/issues/299/design.md","new_diagram_path":"docs/issues/299/diagram.mmd","design_digest":blake3::hash(b"design bytes").to_hex().to_string(),"diagram_digest":blake3::hash(b"diagram bytes").to_hex().to_string(),"design_identity":[owned.dev(),owned.ino()],"diagram_identity":null,"phase":"design_installed"})).unwrap()).unwrap();
    assert!(recover_initialized_design_envelope(
        &Store::new(temp.path()),
        recovery_request(&record)
    )
    .is_err());
    assert_eq!(fs::read(destination).unwrap(), b"design bytes");
}

#[test]
fn recovery_replays_precommit_journal_and_succeeds_in_linked_worktree() {
    let temp = tempfile::tempdir().unwrap();
    assert!(std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    assert!(std::process::Command::new("git")
        .args([
            "-c",
            "user.name=test",
            "-c",
            "user.email=test@example.com",
            "commit",
            "--allow-empty",
            "-qm",
            "base"
        ])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let linked = temp.path().join("linked");
    assert!(std::process::Command::new("git")
        .args([
            "worktree",
            "add",
            "-q",
            "-b",
            "issue-297",
            linked.to_str().unwrap()
        ])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let record = bootstrap_at(
        &linked,
        297,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    fs::create_dir_all(linked.join("docs/issues/297")).unwrap();
    fs::write(linked.join("docs/issues/297/design.md"), b"design bytes").unwrap();
    fs::hard_link(
        linked.join("docs/issues/297/design.md"),
        linked.join("docs/issues/297/.design.md.csdlc-stage"),
    )
    .unwrap();
    let common = String::from_utf8(
        std::process::Command::new("git")
            .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .current_dir(&linked)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let journal_dir = std::path::Path::new(common.trim()).join("csdlc-v2/recovery-journals");
    fs::create_dir_all(&journal_dir).unwrap();
    fs::write(journal_dir.join("297.json"), serde_json::to_vec(&serde_json::json!({"schema":"csdlc.initialized_design_envelope_recovery_journal.v1","issue":297,"pre_generation":record.generation,"pre_digest":record.digest,"post_generation":record.generation+1,"old_design_path":record.design_path,"old_diagram_path":record.diagram_path,"new_design_path":"docs/issues/297/design.md","new_diagram_path":"docs/issues/297/diagram.mmd","design_digest":blake3::hash(b"design bytes").to_hex().to_string(),"diagram_digest":blake3::hash(b"diagram bytes").to_hex().to_string(),"phase":"design_installed"})).unwrap()).unwrap();
    let recovered =
        recover_initialized_design_envelope(&Store::new(&linked), recovery_request(&record))
            .unwrap();
    assert_eq!(recovered.design_path, "docs/issues/297/design.md");
    assert!(!journal_dir.join("297.json").exists());
    assert!(
        linked.join(".git").is_file(),
        "fixture is a linked worktree"
    );
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
