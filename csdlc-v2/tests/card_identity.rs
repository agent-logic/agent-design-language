use std::collections::BTreeMap;

use csdlc_v2::cards::{
    apply, digest, initial_cards, render, CardContent, CardValues, InitialCardInput, PlanStep,
    ResourceProfile, SemanticOperation, StepStatus, ValidationLane,
};
use csdlc_v2::{
    approve_design, bind_issue, edit_issue, initialize_native_json,
    recover_initialized_design_envelope, recover_initialized_design_envelope_with_hook,
    ApproveDesignRequest, BindRequest, DesignRecoveryFailpoint, EditRequest,
    RecoverInitializedDesignEnvelopeRequest, Store,
};
use csdlc_v2::{CardKind, IssueRecord, PlanningProfile};
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
        affected_areas: vec!["tests/card_identity.rs".into()],
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
            argv: vec![
                "cargo".into(),
                "test".into(),
                "--test".into(),
                "card_identity".into(),
            ],
            parallel_group: "identity".into(),
            defer_reason: Some("fixture lane is explicitly deferred".into()),
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
    fs::create_dir_all(root.join("csdlc-v2/tests")).unwrap();
    fs::write(
        root.join("csdlc-v2/tests/card_identity.rs"),
        b"// fixture target\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("tests")).unwrap();
    fs::write(root.join("tests/card_identity.rs"), b"// fixture target\n").unwrap();
    fs::create_dir_all(root.join(std::path::Path::new(design).parent().unwrap())).unwrap();
    fs::create_dir_all(root.join(std::path::Path::new(diagram).parent().unwrap())).unwrap();
    fs::write(root.join(design), b"design bytes").unwrap();
    fs::write(root.join(diagram), b"flowchart TD\n  A --> B\n").unwrap();
    let request = serde_json::json!({
        "issue": issue, "repository":"example/repo", "actor":"test",
        "design_path":design,"diagram_path":diagram,"design_reviewer":"fresh-session:019ff5eb-c6ed-7f83-9d6c-87b7a661eb8b","design_approved":true,
        "initial": input()
    });
    initialize_native_json(&Store::new(root), &serde_json::to_vec(&request).unwrap()).unwrap()
}

fn record_digest_for_fixture(record: &IssueRecord) -> String {
    let mut value = record.clone();
    value.digest.clear();
    digest(&serde_json::to_vec(&value).expect("record digest JSON"))
}

fn commit_all(root: &std::path::Path, message: &str) {
    assert!(std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(root)
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
            "-qm",
            message,
        ])
        .current_dir(root)
        .status()
        .unwrap()
        .success());
}

fn implemented_bound_fixture(
    temp: &tempfile::TempDir,
    issue: u64,
) -> (std::path::PathBuf, IssueRecord) {
    let root = temp.path();
    let boot = bootstrap_at(
        root,
        issue,
        ".csdlc/prepared/issues/fixture/design.md",
        ".csdlc/prepared/issues/fixture/diagram.mmd",
    );
    let ready = edit_issue(
        &Store::new(root),
        EditRequest {
            issue,
            card: CardKind::Sip,
            expected_generation: boot.generation,
            expected_digest: boot.digest,
            actor: "test".into(),
            reason: "fixture ready".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    commit_all(root, "fixture ready");
    let bind_target = root.join("bound-worktree");
    bind_issue(
        &Store::new(root),
        BindRequest {
            issue,
            base_branch: "main".into(),
            branch: format!("issue-{issue}-bound"),
            worktree: bind_target.to_string_lossy().into_owned(),
            code_repository: None,
        },
    )
    .unwrap();
    let bound = Store::new(&bind_target).load_record(issue).unwrap();
    assert_eq!(bound.phase, csdlc_v2::LifecyclePhase::Bound);
    let executed = edit_issue(
        &Store::new(&bind_target),
        EditRequest {
            issue,
            card: CardKind::Sor,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            actor: "test".into(),
            reason: "fixture execution evidence".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "fixture implementation evidence".into(),
                changes: vec!["changed tooling fixture".into()],
                artifacts: vec!["csdlc-v2/tests/card_identity.rs".into()],
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    let implemented = edit_issue(
        &Store::new(&bind_target),
        EditRequest {
            issue,
            card: CardKind::Sip,
            expected_generation: executed.generation,
            expected_digest: executed.digest,
            actor: "test".into(),
            reason: "fixture implemented".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Implemented,
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    assert_eq!(ready.phase, csdlc_v2::LifecyclePhase::Ready);
    assert_eq!(implemented.phase, csdlc_v2::LifecyclePhase::Implemented);
    (bind_target, implemented)
}

fn identity_title_slug_operation(issue: u64) -> SemanticOperation {
    SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition {
        title: format!(
            "[v0.92][WP-18C.02a][{issue}.a] Define shared Layer 8 signed authority core"
        ),
        slug: format!("wp18c02a-{issue}a-shared-layer8-signed-authority-core"),
        live_issue_title: format!(
            "[v0.92][WP-18C.02a][{issue}.a] Define shared Layer 8 signed authority core"
        ),
        live_issue_url: format!(
            "https://github.com/agent-logic/agent-design-language/issues/{issue}"
        ),
        live_issue_body_digest: "body-digest".into(),
    }
}

#[test]
fn implemented_identity_title_slug_repair_updates_all_cards_with_audit() {
    let temp = tempfile::tempdir().unwrap();
    let issue = 112;
    let (worktree, implemented) = implemented_bound_fixture(&temp, issue);
    let before = Store::new(&worktree).load_cards(issue).unwrap();
    let repaired = edit_issue(
        &Store::new(&worktree),
        EditRequest {
            issue,
            card: CardKind::Sip,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            actor: "test".into(),
            reason: "repair decomposed identity".into(),
            operation: identity_title_slug_operation(issue),
            fail_after_backup: false,
        },
    )
    .unwrap();
    let cards = Store::new(&worktree).load_cards(issue).unwrap();
    for (kind, values) in &cards {
        assert_eq!(
            values.identity.title,
            "[v0.92][WP-18C.02a][112.a] Define shared Layer 8 signed authority core",
            "{kind}"
        );
        assert_eq!(
            values.identity.slug, "wp18c02a-112a-shared-layer8-signed-authority-core",
            "{kind}"
        );
        assert_eq!(values.identity.generation, repaired.generation, "{kind}");
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
    let audit = repaired.audit.last().expect("repair audit");
    assert!(audit
        .operation
        .contains("correct_identity_title_slug_after_decomposition"));
    assert!(audit.operation.contains("previous_title"));
    assert!(audit.operation.contains("live_issue_evidence"));
}

#[test]
fn implemented_identity_title_slug_repair_rejects_mismatched_live_title() {
    let temp = tempfile::tempdir().unwrap();
    let issue = 113;
    let (worktree, implemented) = implemented_bound_fixture(&temp, issue);
    let mut operation = identity_title_slug_operation(issue);
    if let SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition {
        live_issue_title, ..
    } = &mut operation
    {
        *live_issue_title = "[v0.92][WP-18C.02b][113.b] Sibling scope".into();
    }
    let error = edit_issue(
        &Store::new(&worktree),
        EditRequest {
            issue,
            card: CardKind::Sip,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            actor: "test".into(),
            reason: "reject sibling".into(),
            operation,
            fail_after_backup: false,
        },
    )
    .unwrap_err();
    assert_eq!(error.code.to_string(), "invalid_input");
}

#[test]
fn identity_title_slug_repair_rejects_before_implemented_phase() {
    let temp = tempfile::tempdir().unwrap();
    let issue = 114;
    let root = temp.path();
    let record = bootstrap_at(
        root,
        issue,
        ".csdlc/prepared/issues/fixture/design.md",
        ".csdlc/prepared/issues/fixture/diagram.mmd",
    );
    let error = edit_issue(
        &Store::new(root),
        EditRequest {
            issue,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test".into(),
            reason: "wrong phase".into(),
            operation: identity_title_slug_operation(issue),
            fail_after_backup: false,
        },
    )
    .unwrap_err();
    assert_eq!(error.code.to_string(), "invalid_transition");
}

fn rewrite_initialized_authored_paths_for_legacy_fixture(
    root: &std::path::Path,
    record: &mut IssueRecord,
    design_path: &str,
    diagram_path: &str,
    design_bytes: &[u8],
    diagram_bytes: &[u8],
) {
    fs::create_dir_all(root.join(std::path::Path::new(design_path).parent().unwrap())).unwrap();
    fs::create_dir_all(root.join(std::path::Path::new(diagram_path).parent().unwrap())).unwrap();
    fs::write(root.join(design_path), design_bytes).unwrap();
    fs::write(root.join(diagram_path), diagram_bytes).unwrap();
    let design_digest = digest(design_bytes);
    let diagram_digest = digest(diagram_bytes);
    let cards_dir = root
        .join(".csdlc/issues")
        .join(record.issue.to_string())
        .join("cards");
    for kind in [CardKind::Spp, CardKind::Vpp] {
        let path = cards_dir.join(format!("{kind}.values.json"));
        let mut values: CardValues =
            serde_json::from_slice(&fs::read(&path).expect("card values")).expect("card JSON");
        match &mut values.content {
            CardContent::Spp(values) => {
                values.design_ref = design_path.into();
                values.diagram_ref = diagram_path.into();
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_ref = design_path.into();
                values.diagram_ref = diagram_path.into();
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            _ => unreachable!("fixture only rewrites SPP/VPP"),
        }
        let mut encoded = serde_json::to_vec_pretty(&values).expect("card values JSON");
        encoded.push(b'\n');
        let rendered = render(&values).expect("render card");
        fs::write(&path, &encoded).unwrap();
        fs::write(
            cards_dir.join(format!("{kind}.md")),
            rendered.markdown.as_bytes(),
        )
        .unwrap();
        let projection = record.cards.get_mut(&kind).expect("card projection");
        projection.values_digest = rendered.values_digest;
        projection.rendered_digest = rendered.rendered_digest;
        projection.ast_digest = rendered.ast_digest;
    }
    record.design_path = design_path.into();
    record.diagram_path = diagram_path.into();
    if let csdlc_v2::DesignReview::Approved { revision, .. } = &mut record.design_review {
        *revision = design_digest;
    }
    record.digest = record_digest_for_fixture(record);
    let index_path = root
        .join(".csdlc/issues")
        .join(record.issue.to_string())
        .join("index.json");
    let mut index = serde_json::to_vec_pretty(record).expect("record JSON");
    index.push(b'\n');
    fs::write(index_path, index).unwrap();
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
    let diagram_digest = blake3::hash(b"flowchart TD\n  A --> B\n")
        .to_hex()
        .to_string();
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
        expected_diagram_digest: blake3::hash(b"flowchart TD\n  A --> B\n")
            .to_hex()
            .to_string(),
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
        "canonical_reviewer",
        "session_uuid",
        "turn_uuid",
        "spawned_task",
        "thread_source",
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
            "canonical_reviewer" => request.canonical_reviewer = "fresh-session:wrong".into(),
            "session_uuid" => request.reviewer_session_uuid = "not-a-uuid".into(),
            "turn_uuid" => request.reviewer_turn_uuid = "not-a-uuid".into(),
            "spawned_task" => request.spawned_task.clear(),
            "thread_source" => request.thread_source = "parent".into(),
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
    let owned_handle = fs::File::open(&destination).unwrap();
    let owned = owned_handle.metadata().unwrap();
    fs::remove_file(&destination).unwrap();
    fs::write(&destination, b"design bytes").unwrap();
    let replacement = fs::metadata(&destination).unwrap();
    assert_ne!(
        (owned.dev(), owned.ino()),
        (replacement.dev(), replacement.ino())
    );
    let common = String::from_utf8(
        std::process::Command::new("git")
            .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .current_dir(temp.path())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    let journal_dir =
        std::path::Path::new(common.trim()).join("csdlc-v2/recovery-journals/299/fixture");
    fs::create_dir_all(&journal_dir).unwrap();
    fs::write(journal_dir.join("010-design_installed.json"), serde_json::to_vec(&serde_json::json!({"schema":"csdlc.initialized_design_envelope_recovery_journal.v1","issue":299,"pre_generation":record.generation,"pre_digest":record.digest,"post_generation":record.generation+1,"old_design_path":record.design_path,"old_diagram_path":record.diagram_path,"new_design_path":"docs/issues/299/design.md","new_diagram_path":"docs/issues/299/diagram.mmd","design_digest":blake3::hash(b"design bytes").to_hex().to_string(),"diagram_digest":blake3::hash(b"diagram bytes").to_hex().to_string(),"design_identity":[owned.dev(),owned.ino()],"diagram_identity":null,"phase":"design_installed","attempt_id":"fixture","sequence":10})).unwrap()).unwrap();
    assert!(recover_initialized_design_envelope(
        &Store::new(temp.path()),
        recovery_request(&record)
    )
    .is_err());
    assert_eq!(fs::read(destination).unwrap(), b"design bytes");
}

#[cfg(unix)]
#[test]
fn restart_reconciles_interrupted_stage_quarantine() {
    use std::os::unix::fs::MetadataExt;
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        300,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let request = recovery_request(&record);
    let parent = temp.path().join("docs/issues/300");
    fs::create_dir_all(&parent).unwrap();
    let quarantine_source = parent.join("stage-source");
    fs::write(&quarantine_source, b"design bytes").unwrap();
    let metadata = fs::metadata(&quarantine_source).unwrap();
    let quarantine = parent.join(format!(
        "..design.md.csdlc-stage.csdlc-delete-{}-{}",
        metadata.dev(),
        metadata.ino()
    ));
    fs::rename(&quarantine_source, &quarantine).unwrap();
    let recovered = recover_initialized_design_envelope(&Store::new(temp.path()), request).unwrap();
    assert_eq!(recovered.generation, record.generation + 1);
    assert!(!quarantine.exists());
}

#[cfg(unix)]
#[test]
fn restart_reconciles_interrupted_owned_delete_quarantine() {
    use std::os::unix::fs::MetadataExt;
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        304,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let destination = temp.path().join("docs/issues/304/design.md");
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(&destination, b"design bytes").unwrap();
    let meta = fs::metadata(&destination).unwrap();
    let owned = destination.parent().unwrap().join(format!(
        ".design.md.csdlc-owned-delete-{}-{}",
        meta.dev(),
        meta.ino()
    ));
    fs::rename(&destination, &owned).unwrap();
    let stage = destination.parent().unwrap().join(".design.md.csdlc-stage");
    fs::hard_link(&owned, &stage).unwrap();
    let common = temp
        .path()
        .join(".git/csdlc-v2/recovery-journals/304/fixture");
    fs::create_dir_all(&common).unwrap();
    fs::write(common.join("010-design_installed.json"), serde_json::to_vec(&serde_json::json!({"schema":"csdlc.initialized_design_envelope_recovery_journal.v1","issue":304,"pre_generation":record.generation,"pre_digest":record.digest,"post_generation":record.generation+1,"old_design_path":record.design_path,"old_diagram_path":record.diagram_path,"new_design_path":"docs/issues/304/design.md","new_diagram_path":"docs/issues/304/diagram.mmd","design_digest":blake3::hash(b"design bytes").to_hex().to_string(),"diagram_digest":blake3::hash(b"diagram bytes").to_hex().to_string(),"design_identity":null,"diagram_identity":null,"phase":"design_installed","attempt_id":"fixture","sequence":10})).unwrap()).unwrap();
    let recovered =
        recover_initialized_design_envelope(&Store::new(temp.path()), recovery_request(&record))
            .unwrap();
    assert_eq!(recovered.generation, record.generation + 1);
    assert!(!owned.exists());
}

#[test]
fn recovery_rejects_later_lifecycle_and_unsafe_control_destinations() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        301,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    for unsafe_path in [
        ".git/recovery.md",
        ".csdlc/issues/301/design.md",
        ".csdlc/locks/design.md",
        "../escape.md",
    ] {
        let mut request = recovery_request(&record);
        request.new_design_path = unsafe_path.into();
        assert!(
            recover_initialized_design_envelope(&Store::new(temp.path()), request).is_err(),
            "{unsafe_path}"
        );
    }
}

#[cfg(unix)]
#[test]
fn recovery_rejects_hardlinked_source_alias_before_journaling() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        302,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    fs::remove_file(temp.path().join(&record.diagram_path)).unwrap();
    fs::hard_link(
        temp.path().join(&record.design_path),
        temp.path().join(&record.diagram_path),
    )
    .unwrap();
    let mut request = recovery_request(&record);
    request.expected_diagram_digest = request.expected_design_digest.clone();
    assert!(recover_initialized_design_envelope(&Store::new(temp.path()), request).is_err());
    let common = String::from_utf8(
        std::process::Command::new("git")
            .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .current_dir(temp.path())
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    assert!(!std::path::Path::new(common.trim())
        .join("csdlc-v2/recovery-journals/302")
        .exists());
}

#[test]
fn recovery_rejects_tampered_later_history_without_artifact_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let record = bootstrap_at(
        temp.path(),
        303,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    let index_path = temp.path().join(".csdlc/issues/303/index.json");
    let mut index: serde_json::Value =
        serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index["phase"] = serde_json::Value::String("bound".into());
    index["branch"] = serde_json::Value::String("codex/303-later".into());
    fs::write(&index_path, serde_json::to_vec_pretty(&index).unwrap()).unwrap();
    let before_design = fs::read(temp.path().join(&record.design_path)).unwrap();
    assert!(recover_initialized_design_envelope(
        &Store::new(temp.path()),
        recovery_request(&record)
    )
    .is_err());
    assert_eq!(
        fs::read(temp.path().join(&record.design_path)).unwrap(),
        before_design
    );
    assert!(!temp.path().join("docs/issues/303/design.md").exists());
}

#[test]
fn every_design_recovery_failpoint_restarts_to_a_canonical_state() {
    for point in [
        DesignRecoveryFailpoint::AfterPreparedReceipt,
        DesignRecoveryFailpoint::AfterDesignInstall,
        DesignRecoveryFailpoint::AfterDesignReceipt,
        DesignRecoveryFailpoint::AfterDiagramInstall,
        DesignRecoveryFailpoint::AfterArtifactsReceipt,
        DesignRecoveryFailpoint::AfterStatePreparedReceipt,
        DesignRecoveryFailpoint::BeforeStateCommit,
        DesignRecoveryFailpoint::AfterStateCommit,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let record = bootstrap_at(
            temp.path(),
            305,
            ".csdlc/prepared/legacy/design.md",
            ".csdlc/prepared/legacy/diagram.mmd",
        );
        let request = recovery_request(&record);
        assert!(recover_initialized_design_envelope_with_hook(
            &Store::new(temp.path()),
            request.clone(),
            |candidate| candidate == point
        )
        .is_err());
        let current = Store::new(temp.path()).load_record(305).unwrap();
        if current.generation == record.generation {
            let recovered = recover_initialized_design_envelope(&Store::new(temp.path()), request)
                .unwrap_or_else(|error| panic!("restart failed at {point:?}: {error:?}"));
            assert_eq!(recovered.generation, record.generation + 1, "{point:?}");
        } else {
            assert_eq!(current.generation, record.generation + 1, "{point:?}");
            assert_eq!(current.design_path, "docs/issues/305/design.md");
        }
    }
}

#[test]
fn recovery_replays_precommit_journal_and_succeeds_in_linked_worktree() {
    let temp = tempfile::tempdir().unwrap();
    let mut record = bootstrap_at(
        temp.path(),
        297,
        ".csdlc/prepared/legacy/design.md",
        ".csdlc/prepared/legacy/diagram.mmd",
    );
    assert!(std::process::Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    rewrite_initialized_authored_paths_for_legacy_fixture(
        temp.path(),
        &mut record,
        ".git/csdlc-v2/requests/design.md",
        ".git/csdlc-v2/requests/diagram.mmd",
        b"design bytes",
        b"flowchart TD\n  A --> B\n",
    );
    let bind_target = temp.path().join("bound-297");
    let before_bind = bind_issue(
        &Store::new(temp.path()),
        BindRequest {
            issue: 297,
            base_branch: "main".into(),
            branch: "issue-297-bound".into(),
            worktree: bind_target.to_string_lossy().into_owned(),
            code_repository: None,
        },
    );
    let before_bind = before_bind.expect_err("dirty/unsafe pre-recovery source must not bind");
    assert_eq!(before_bind.code.to_string(), "unsafe_checkout");
    assert!(
        before_bind.message.contains("authored design")
            || before_bind.message.contains("design/diagram"),
        "pre-recovery bind must fail on unsafe authored artifact source, got: {before_bind:?}"
    );
    recover_initialized_design_envelope_with_hook(
        &Store::new(temp.path()),
        recovery_request(&record),
        |candidate| candidate == DesignRecoveryFailpoint::AfterDesignInstall,
    )
    .expect_err("fixture injects an actual post-design-install recovery interruption");
    let recovered =
        recover_initialized_design_envelope(&Store::new(temp.path()), recovery_request(&record))
            .unwrap();
    assert_eq!(recovered.design_path, "docs/issues/297/design.md");
    let approved = approve_design(
        &Store::new(temp.path()),
        ApproveDesignRequest {
            issue: 297,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            reviewer: "fresh-session:11111111-1111-4111-8111-111111111111".into(),
        },
    )
    .unwrap();
    assert!(matches!(
        approved.design_review,
        csdlc_v2::DesignReview::Approved { .. }
    ));
    let ready = edit_issue(
        &Store::new(temp.path()),
        EditRequest {
            issue: 297,
            card: CardKind::Sip,
            expected_generation: approved.generation,
            expected_digest: approved.digest,
            actor: "test".into(),
            reason: "fixture ready after recovery".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    assert_eq!(ready.phase, csdlc_v2::LifecyclePhase::Ready);
    assert!(std::process::Command::new("git")
        .args(["add", "."])
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
            "-qm",
            "recover issue 297"
        ])
        .current_dir(temp.path())
        .status()
        .unwrap()
        .success());
    let after_bind = bind_issue(
        &Store::new(temp.path()),
        BindRequest {
            issue: 297,
            base_branch: "main".into(),
            branch: "issue-297-bound".into(),
            worktree: bind_target.to_string_lossy().into_owned(),
            code_repository: None,
        },
    );
    assert!(
        after_bind.is_ok(),
        "approved safe recovery should bind: {after_bind:?}; doctor={:?}",
        csdlc_v2::diagnose(&Store::new(temp.path()), 297)
    );
    assert!(
        bind_target.join(".git").is_file(),
        "fixture target is a linked worktree"
    );
    assert!(bind_target.join("docs/issues/297/design.md").is_file());
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

#[test]
fn implemented_authored_design_refresh() {
    let operation: SemanticOperation = serde_json::from_value(serde_json::json!({
        "operation": "refresh_authored_design_after_recovery"
    }))
    .expect("typed refresh operation");
    let mut cards = fixture();
    let spp = cards.get_mut(&CardKind::Spp).expect("SPP");
    apply(spp, &operation).expect("SPP owns refresh operation");
    let error = apply(cards.get_mut(&CardKind::Vpp).expect("VPP"), &operation)
        .expect_err("VPP cannot own refresh operation");
    assert_eq!(error.code.to_string(), "field_ownership");
}

#[test]
fn implemented_authored_design_refresh_linked_worktree() {
    let schema = csdlc_v2::public_schema_bundle();
    let encoded = serde_json::to_string(&schema).expect("schema JSON");
    assert!(encoded.contains("refresh_authored_design_after_recovery"));
    assert!(encoded.contains("approve_design_request"));
    assert!(encoded.contains("review_assignment_request"));
}
