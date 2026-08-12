use csdlc_v2::cards::{digest, FindingDisposition, FindingSeverity, PublicationState};
use csdlc_v2::model::TransitionEvent;
use csdlc_v2::{
    assign_review, bind_issue, edit_issue, evaluate_publication_review,
    evaluate_publication_review_in_repo, record_review, BindRequest, BootstrapRequest, CardKind,
    EditRequest, ErrorCode, InitialCardInput, LifecyclePhase, NonSubstantiveProof, PlanningProfile,
    ReviewAssignmentRequest, ReviewEvidence, ReviewFindingEvidence, ReviewRecordRequest,
    ReviewRecoveryRequest, SemanticOperation, Store,
};

fn install_native_authority(root: &std::path::Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    std::fs::create_dir_all(registry.parent().unwrap()).unwrap();
    std::fs::create_dir_all(manifest.parent().unwrap()).unwrap();
    std::fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .unwrap();
    std::fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .unwrap();
}

fn bootstrap_issue(
    store: &Store,
    request: BootstrapRequest,
) -> csdlc_v2::Result<csdlc_v2::IssueRecord> {
    csdlc_v2::initialize_native_json(store, &serde_json::to_vec(&request).unwrap())
}

fn finding(id: &str) -> ReviewFindingEvidence {
    ReviewFindingEvidence {
        id: id.into(),
        severity: FindingSeverity::P1,
        summary: "fix correctness".into(),
        actionable: true,
        in_scope: true,
        disposition: FindingDisposition::Fixed,
        fix_revision: Some("rev-2".into()),
        route: None,
    }
}

#[test]
fn implemented_authored_design_refresh_end_to_end_is_atomic_and_assignment_gated() {
    let (_temp, store, implemented) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "first-reviewer".into(),
            assigned_by: "agent".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("initial assignment");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "agent".into(),
            reason: "authored design changed after implementation".into(),
        },
    )
    .expect("typed recovery");
    std::fs::write(store.root().join("docs/design.md"), "# refreshed design\n").unwrap();
    std::fs::write(
        store.root().join("docs/diagram.mmd"),
        "flowchart LR\n B-->C\n",
    )
    .unwrap();
    let stale = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation - 1,
            expected_digest: recovered.digest.clone(),
            actor: "agent".into(),
            reason: "stale refresh".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect_err("stale CAS must fail");
    assert_eq!(stale.code, ErrorCode::StaleGeneration);
    let refreshed = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "agent".into(),
            reason: "refresh paired authored tuple".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect("atomic authored refresh");
    assert_eq!(refreshed.phase, LifecyclePhase::Implemented);
    assert!(refreshed.review_assignment.is_none());
    assert!(refreshed
        .audit
        .last()
        .unwrap()
        .operation
        .contains("refresh_authored_design_after_recovery"));
    let spp = store.load_cards(7).unwrap().remove(&CardKind::Spp).unwrap();
    let csdlc_v2::cards::CardContent::Spp(values) = spp.content else {
        panic!("SPP")
    };
    assert_eq!(values.design_digest, digest(b"# refreshed design\n"));
    assert_eq!(values.diagram_digest, digest(b"flowchart LR\n B-->C\n"));

    git(store.root(), &["add", "."]);
    git(store.root(), &["commit", "-m", "refresh authored tuple"]);
    let approved = csdlc_v2::approve_design(
        &store,
        csdlc_v2::ApproveDesignRequest {
            issue: 7,
            expected_generation: refreshed.generation,
            expected_digest: refreshed.digest,
            reviewer: "fresh-session:11111111-1111-1111-1111-111111111111".into(),
        },
    )
    .expect("approve refreshed tuple");
    assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: approved.generation,
            expected_digest: approved.digest,
            reviewer: "replacement-reviewer".into(),
            assigned_by: "agent".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("assignment after exact tuple approval");
}

fn implemented_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::write(temp.path().join("docs/design.md"), "# reviewed design\n").expect("design");
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .expect("diagram");
    std::fs::create_dir_all(temp.path().join("src")).expect("source directory");
    std::fs::write(temp.path().join("src/lib.rs"), "// fixture\n").expect("source fixture");
    std::fs::write(
        temp.path().join("src/validate.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\ntest -f src/lib.rs\n",
    )
    .expect("validator fixture");
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);
    let store = Store::new(temp.path());
    let record = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue: 7,
            repository: "example/repo".into(),
            actor: "agent".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            initial: InitialCardInput {
                title: "review fixture".into(),
                slug: "review-fixture".into(),
                version: "v0.91.7".into(),
                goal: "prove review".into(),
                required_outcome: "review truth".into(),
                declared_scope: vec!["src".into()],
                authority_boundary: vec!["no network".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "review only".into(),
                deliverables: vec!["src/validate.sh".into()],
                acceptance_criteria: vec!["review current".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["src".into()],
                non_goals: vec!["publish".into()],
                plan_summary: "implement then review".into(),
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "one".into(),
                    action: "review".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
                affected_areas: vec!["src".into(), "src/validate.sh".into()],
                invariants: vec!["exact revision".into()],
                risks: vec!["stale".into()],
                planning_profile: PlanningProfile::Small,
                stop_conditions: vec!["stale".into()],
                validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                    lane: "focused".into(),
                    proof_role: "review".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                    budget_seconds: 60,
                    budget_tokens: 100,
                    argv: vec!["bash".into(), "src/validate.sh".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "fail closed".into(),
                review_prompts: vec!["review correctness".into()],
                review_scope: "fixture".into(),
            },
        },
    )
    .expect("init");
    let _ready = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "agent".into(),
            reason: "fixture is execution-ready".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .expect("ready");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "initialize issue"]);
    let worktree = temp.path().join("worktrees/issue-7");
    bind_issue(
        &store,
        BindRequest {
            issue: 7,
            base_branch: "main".into(),
            branch: "issue-7".into(),
            worktree: worktree.to_string_lossy().into_owned(),
            code_repository: None,
        },
    )
    .expect("bind");
    let store = Store::new(worktree);
    let mut record = store.load_record(7).expect("bound record");
    for operation in [
        SemanticOperation::RecordExecution {
            summary: "implemented".into(),
            changes: vec!["src".into()],
            artifacts: vec!["artifact".into()],
        },
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
    ] {
        let card = if matches!(operation, SemanticOperation::RecordExecution { .. }) {
            CardKind::Sor
        } else {
            CardKind::Sip
        };
        record = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card,
                expected_generation: record.generation,
                expected_digest: record.digest.clone(),
                actor: "agent".into(),
                reason: "fixture transition".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .expect("transition");
    }
    (temp, store, record)
}

fn write_consistent_record(root: &std::path::Path, record: &mut csdlc_v2::IssueRecord) {
    record.digest.clear();
    record.digest = csdlc_v2::cards::digest(
        &serde_json::to_vec(&*record).expect("record digest serialization"),
    );
    let mut bytes = serde_json::to_vec_pretty(&*record).expect("record projection serialization");
    bytes.push(b'\n');
    std::fs::write(
        root.join(format!(".csdlc/issues/{}/index.json", record.issue)),
        bytes,
    )
    .expect("write consistent record projection");
}

#[test]
fn substantive_revision_honors_review_scope_pathspecs() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
    std::fs::write(temp.path().join("docs/review.md"), "reviewed\n").expect("doc");
    std::fs::write(temp.path().join("src/outside.rs"), "outside\n").expect("src");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs", "src"]);
    git(temp.path(), &["commit", "-m", "fixture"]);

    let clean = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("clean scoped revision");
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    assert_eq!(clean, csdlc_v2::git::clean_commit_revision(&head));

    std::fs::write(temp.path().join("src/outside.rs"), "outside dirty\n").expect("dirty src");
    std::fs::write(temp.path().join("src/untracked.rs"), "new outside\n").expect("outside new");
    let outside_dirty = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("outside dirty scoped revision");
    assert_eq!(outside_dirty, clean);

    std::fs::write(temp.path().join("docs/new.md"), "new reviewed file\n").expect("new doc");
    let inside_untracked = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("inside untracked scoped revision");
    assert_ne!(inside_untracked, clean);

    std::fs::write(temp.path().join("docs/review.md"), "reviewed dirty\n").expect("dirty doc");
    let inside_dirty = csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()])
        .expect("inside dirty scoped revision");
    assert_ne!(inside_dirty, clean);
}

#[test]
fn assignment_and_recording_update_index_and_srp_without_publication_side_effect() {
    let (temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("assign");
    let cards = store.load_cards(7).expect("assigned cards");
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP");
    };
    assert_eq!(srp.review_scope, "src");
    assert!(assigned.review.is_none());
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let mut fixed = finding("F-1");
    fixed.fix_revision = Some(revision.clone());
    let value = ReviewEvidence {
        reviewer: "subagent".into(),
        scope: vec!["src".into()],
        reviewed_revision: revision.clone(),
        findings: vec![fixed],
        residual_risks: vec!["none".into()],
        completed: true,
        non_substantive_proof: None,
    };
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "agent".into(),
            evidence: value,
        },
    )
    .expect("record");
    assert!(evaluate_publication_review(reviewed.review.as_ref(), &revision).ready);
    let cards = store.load_cards(7).expect("cards");
    match &cards[&CardKind::Srp].content {
        csdlc_v2::cards::CardContent::Srp(srp) => {
            assert_eq!(srp.reviewer.as_deref(), Some("subagent"));
            assert_eq!(srp.findings.len(), 1);
        }
        _ => unreachable!(),
    };
    assert_eq!(
        git_out(store.root(), &["branch", "--show-current"]),
        "issue-7"
    );
    assert!(
        !temp.path().join(".git/refs/remotes").exists(),
        "review created remote state"
    );
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
}

#[test]
fn direct_exact_review_records_and_advances_without_assignment() {
    let (_temp, store, record) = implemented_fixture();
    assert!(record.review_assignment.is_none());
    let revision = csdlc_v2::git::substantive_revision(store.root(), &["src".into()])
        .expect("exact scoped revision");
    let before = std::fs::read(store.issue_dir(7).join("index.json")).expect("before");
    let mut stale = ReviewRecordRequest {
        issue: 7,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        actor: "reviewer".into(),
        evidence: ReviewEvidence {
            reviewer: "reviewer".into(),
            scope: vec!["src".into()],
            reviewed_revision: "git-blake3:stale:stale".into(),
            findings: vec![],
            residual_risks: vec![],
            completed: true,
            non_substantive_proof: None,
        },
    };
    assert_eq!(
        record_review(&store, stale.clone()).unwrap_err().code,
        ErrorCode::UnsafeCheckout
    );
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).expect("unchanged"),
        before
    );
    stale.evidence.reviewed_revision = revision;
    let reviewed = record_review(&store, stale).expect("direct exact review");
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
    assert!(reviewed.review_assignment.is_none());
    assert_eq!(
        reviewed.audit.last().expect("audit").operation,
        "record_review"
    );
}

#[test]
fn dirty_substantive_tree_is_rejected_before_review_assignment() {
    let (_temp, store, record) = implemented_fixture();
    std::fs::write(store.root().join("docs/design.md"), "# changed design\n").expect("dirty");
    let error = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect_err("dirty review assignment must fail closed");
    assert!(matches!(error.code, ErrorCode::UnsafeCheckout));
}

#[test]
fn metadata_only_changes_do_not_stale_a_clean_review() {
    let (_temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("clean assignment");
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "agent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision.clone(),
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record review");
    std::fs::create_dir_all(store.root().join(".csdlc/review")).expect("metadata dir");
    std::fs::write(store.root().join(".csdlc/review/observation.json"), "{}\n").expect("metadata");
    let current = csdlc_v2::git::substantive_revision(store.root(), &["docs".into()])
        .expect("current revision");
    assert_eq!(current, revision);
    assert!(
        evaluate_publication_review_in_repo(store.root(), reviewed.review.as_ref(), &current).ready
    );
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
}

#[test]
fn reviewed_dirty_state_is_diagnosed_and_recoverable_for_clean_rereview() {
    let (_temp, store, implemented) = implemented_fixture();
    let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
    let premature = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Srp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "not actually recovered".into(),
            operation: SemanticOperation::CorrectReviewPromptsAfterRecovery {
                values: vec!["truthful prompt".into()],
            },
            fail_after_backup: false,
        },
    )
    .unwrap_err();
    assert_eq!(premature.code, ErrorCode::InvalidTransition);
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
        before
    );
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assign clean review");
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "agent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision,
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record review");
    std::fs::write(store.root().join("docs/new-proof.md"), "proof\n").expect("dirty change");
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Block
    ));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
    assert_eq!(report.next_operation.as_deref(), Some("recover_review"));

    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "re-review after finalizing substantive changes".into(),
        },
    )
    .expect("recover reviewed state");
    assert_eq!(recovered.phase, LifecyclePhase::Implemented);
    assert!(recovered.review.is_none());
    assert!(recovered.review_assignment.is_none());
    assert!(recovered
        .audit
        .iter()
        .any(|event| event.operation == "recover_review"));

    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Srp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "correct stale review question after recovery".into(),
            operation: SemanticOperation::CorrectReviewPromptsAfterRecovery {
                values: vec!["Does the final hosted mode match current truth?".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("correct prompts after recovery");
    let cards = store.load_cards(7).unwrap();
    let csdlc_v2::cards::CardContent::Srp(srp) = &cards[&CardKind::Srp].content else {
        panic!("SRP")
    };
    assert_eq!(
        srp.review_prompts,
        vec!["Does the final hosted mode match current truth?"]
    );

    git(store.root(), &["add", "docs/new-proof.md"]);
    git(store.root(), &["commit", "-m", "finalize reviewed changes"]);
    let reassigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: corrected.generation,
            expected_digest: corrected.digest,
            reviewer: "reviewer".into(),
            assigned_by: "operator".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("reassign after clean finalize");
    assert!(reassigned.review_assignment.is_some());
}

#[test]
fn implemented_review_recovery_clears_truth() {
    let (_temp, clean_store, clean) = implemented_fixture();
    let clean_error = csdlc_v2::recover_review(
        &clean_store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: clean.generation,
            expected_digest: clean.digest,
            actor: "operator".into(),
            reason: "clean implemented records have nothing to recover".into(),
        },
    )
    .expect_err("clean implemented recovery must fail closed");
    assert_eq!(clean_error.code, ErrorCode::InvalidTransition);

    let (_temp, assigned_store, implemented) = implemented_fixture();
    let assigned = assign_review(
        &assigned_store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "operator".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assign review");
    assert_eq!(assigned.phase, LifecyclePhase::Implemented);
    let correction_error = edit_issue(
        &assigned_store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest.clone(),
            actor: "operator".into(),
            reason: "correct scope".into(),
            operation: SemanticOperation::CorrectDeclaredScopeBeforePublication {
                values: vec!["src/lib.rs".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("scope correction must wait for typed review recovery");
    assert_eq!(correction_error.code, ErrorCode::InvalidTransition);
    let transition_count = assigned.transitions.len();
    let recovered_assignment = csdlc_v2::recover_review(
        &assigned_store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "correct declared scope before review".into(),
        },
    )
    .expect("recover assignment-only implemented state");
    assert_eq!(recovered_assignment.phase, LifecyclePhase::Implemented);
    assert_eq!(recovered_assignment.transitions.len(), transition_count);
    assert!(recovered_assignment.review_assignment.is_none());
    assert!(recovered_assignment.review.is_none());
    assert!(recovered_assignment.publication.is_none());
    assert!(recovered_assignment.readiness.is_none());
    assert!(recovered_assignment
        .audit
        .last()
        .is_some_and(|event| event.operation == "recover_review"));
    let corrected = edit_issue(
        &assigned_store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered_assignment.generation,
            expected_digest: recovered_assignment.digest,
            actor: "operator".into(),
            reason: "correct scope after typed recovery".into(),
            operation: SemanticOperation::CorrectDeclaredScopeBeforePublication {
                values: vec!["src/lib.rs".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("correct scope after recovery");
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);

    let (_temp, reviewed_store, implemented) = implemented_fixture();
    let assigned = assign_review(
        &reviewed_store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "operator".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assign review");
    let revision = assigned
        .review_assignment
        .as_ref()
        .expect("assignment")
        .revision
        .clone();
    let changes_required = record_review(
        &reviewed_store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision,
                findings: vec![ReviewFindingEvidence {
                    id: "scope-path".into(),
                    severity: FindingSeverity::P1,
                    summary: "declared scope names a stale path".into(),
                    actionable: true,
                    in_scope: true,
                    disposition: FindingDisposition::Open,
                    fix_revision: None,
                    route: None,
                }],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record changes-required review");
    assert_eq!(changes_required.phase, LifecyclePhase::Implemented);
    assert!(changes_required.review.is_some());
    let transition_count = changes_required.transitions.len();
    let recovered_review = csdlc_v2::recover_review(
        &reviewed_store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: changes_required.generation,
            expected_digest: changes_required.digest,
            actor: "operator".into(),
            reason: "repair the declared scope finding".into(),
        },
    )
    .expect("recover changes-required implemented state");
    assert_eq!(recovered_review.phase, LifecyclePhase::Implemented);
    assert_eq!(recovered_review.transitions.len(), transition_count);
    assert!(recovered_review.review_assignment.is_none());
    assert!(recovered_review.review.is_none());
}

#[test]
fn recovered_issue_can_correct_only_the_spp_plan_summary() {
    for recovery_phase in [
        LifecyclePhase::Reviewed,
        LifecyclePhase::Published,
        LifecyclePhase::MergeReady,
    ] {
        let (_temp, store, implemented) = implemented_fixture();
        let revision = csdlc_v2::git::substantive_revision(store.root(), &["src".into()])
            .expect("review revision");
        let mut record = record_review(
            &store,
            ReviewRecordRequest {
                issue: 7,
                expected_generation: implemented.generation,
                expected_digest: implemented.digest,
                actor: "reviewer".into(),
                evidence: ReviewEvidence {
                    reviewer: "reviewer".into(),
                    scope: vec!["src".into()],
                    reviewed_revision: revision,
                    findings: vec![],
                    residual_risks: vec![],
                    completed: true,
                    non_substantive_proof: None,
                },
            },
        )
        .expect("record review");
        if matches!(
            recovery_phase,
            LifecyclePhase::Published | LifecyclePhase::MergeReady
        ) {
            record = edit_issue(
                &store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Sor,
                    expected_generation: record.generation,
                    expected_digest: record.digest,
                    actor: "publisher".into(),
                    reason: "record ready publication".into(),
                    operation: SemanticOperation::RecordPublication {
                        state: PublicationState::Ready,
                    },
                    fail_after_backup: false,
                },
            )
            .expect("record publication readiness");
            record = edit_issue(
                &store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Sor,
                    expected_generation: record.generation,
                    expected_digest: record.digest,
                    actor: "publisher".into(),
                    reason: "advance published".into(),
                    operation: SemanticOperation::AdvancePhase {
                        phase: LifecyclePhase::Published,
                    },
                    fail_after_backup: false,
                },
            )
            .expect("advance published");
        }
        if recovery_phase == LifecyclePhase::MergeReady {
            record.phase = LifecyclePhase::MergeReady;
            record.transitions.push(TransitionEvent {
                sequence: record.transitions.len() as u64 + 1,
                from: LifecyclePhase::Published,
                to: LifecyclePhase::MergeReady,
                actor: "legacy-readiness".into(),
                reason: "retained merge-ready compatibility state".into(),
            });
            write_consistent_record(store.root(), &mut record);
        }
        assert_eq!(record.phase, recovery_phase);

        let recovery_actor = format!("recover-{recovery_phase}");
        let recovery_reason = format!("correct {recovery_phase} plan summary");
        if recovery_phase == LifecyclePhase::Reviewed {
            let before_invalid_recovery = std::fs::read(store.issue_dir(7).join("index.json"))
                .expect("before invalid recovery");
            let error = csdlc_v2::recover_review(
                &store,
                ReviewRecoveryRequest {
                    issue: 7,
                    expected_generation: record.generation,
                    expected_digest: record.digest.clone(),
                    actor: " ".into(),
                    reason: recovery_reason.clone(),
                },
            )
            .expect_err("blank recovery actor must fail");
            assert_eq!(error.code, ErrorCode::InvalidInput);
            assert_eq!(
                std::fs::read(store.issue_dir(7).join("index.json"))
                    .expect("after invalid recovery"),
                before_invalid_recovery
            );
        }
        let recovered = csdlc_v2::recover_review(
            &store,
            ReviewRecoveryRequest {
                issue: 7,
                expected_generation: record.generation,
                expected_digest: record.digest,
                actor: recovery_actor,
                reason: recovery_reason,
            },
        )
        .expect("recover review");
        let before_cards = store
            .load_cards(7)
            .expect("cards before summary correction");
        let replacement = format!("corrected after {recovery_phase}");
        if recovery_phase == LifecyclePhase::Published {
            let recovered_snapshot = recovered.clone();
            let mut retained = recovered.clone();
            retained.publication = Some(csdlc_v2::model::PublicationEvidence {
                repository: "example/repo".into(),
                issue: 7,
                pull_request: 7,
                url: "https://example.invalid/pr/7".into(),
                base: "main".into(),
                head: "issue-7".into(),
                revision: "retained".into(),
                linkage_mode: None,
                draft: true,
                observed_state: "open".into(),
            });
            write_consistent_record(store.root(), &mut retained);
            assert_eq!(
                edit_issue(
                    &store,
                    EditRequest {
                        issue: 7,
                        card: CardKind::Spp,
                        expected_generation: retained.generation,
                        expected_digest: retained.digest,
                        actor: "operator".into(),
                        reason: "reject retained publication".into(),
                        operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                            value: replacement.clone(),
                        },
                        fail_after_backup: false,
                    },
                )
                .expect_err("retained publication must fail")
                .code,
                ErrorCode::InvalidTransition
            );
            let mut restored = recovered_snapshot.clone();
            write_consistent_record(store.root(), &mut restored);

            let mut retained = recovered_snapshot.clone();
            retained.readiness = Some(csdlc_v2::model::ReadinessEvidence {
                pull_request: 7,
                head_sha: "retained".into(),
                checks: vec![],
                review_state: csdlc_v2::readiness::RemoteReviewState::Pending,
                conflict_state: csdlc_v2::readiness::ConflictState::Pending,
                post_publication_findings: vec![],
                ready: false,
                blockers: vec!["retained".into()],
            });
            write_consistent_record(store.root(), &mut retained);
            assert_eq!(
                edit_issue(
                    &store,
                    EditRequest {
                        issue: 7,
                        card: CardKind::Spp,
                        expected_generation: retained.generation,
                        expected_digest: retained.digest,
                        actor: "operator".into(),
                        reason: "reject retained readiness".into(),
                        operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                            value: replacement.clone(),
                        },
                        fail_after_backup: false,
                    },
                )
                .expect_err("retained readiness must fail")
                .code,
                ErrorCode::InvalidTransition
            );
            let mut restored = recovered_snapshot;
            write_consistent_record(store.root(), &mut restored);
        }
        if recovery_phase == LifecyclePhase::Reviewed {
            for (card, value, generation, digest) in [
                (
                    CardKind::Sip,
                    replacement.clone(),
                    recovered.generation,
                    recovered.digest.clone(),
                ),
                (
                    CardKind::Spp,
                    " ".into(),
                    recovered.generation,
                    recovered.digest.clone(),
                ),
                (
                    CardKind::Spp,
                    replacement.clone(),
                    recovered.generation - 1,
                    recovered.digest.clone(),
                ),
                (
                    CardKind::Spp,
                    replacement.clone(),
                    recovered.generation,
                    "0".repeat(64),
                ),
            ] {
                let before_rejection = std::fs::read(store.issue_dir(7).join("index.json"))
                    .expect("before correction rejection");
                let error = edit_issue(
                    &store,
                    EditRequest {
                        issue: 7,
                        card,
                        expected_generation: generation,
                        expected_digest: digest,
                        actor: "operator".into(),
                        reason: "prove correction rejection".into(),
                        operation: SemanticOperation::CorrectPlanSummaryAfterRecovery { value },
                        fail_after_backup: false,
                    },
                )
                .expect_err("invalid correction must fail");
                assert!(matches!(
                    error.code,
                    ErrorCode::InvalidTransition
                        | ErrorCode::CardInvalid
                        | ErrorCode::StaleGeneration
                        | ErrorCode::StaleDigest
                ));
                assert_eq!(
                    std::fs::read(store.issue_dir(7).join("index.json"))
                        .expect("after correction rejection"),
                    before_rejection
                );
            }
            let interrupted = edit_issue(
                &store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Spp,
                    expected_generation: recovered.generation,
                    expected_digest: recovered.digest.clone(),
                    actor: "operator".into(),
                    reason: "prove interrupted correction recovery".into(),
                    operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                        value: replacement.clone(),
                    },
                    fail_after_backup: true,
                },
            )
            .expect_err("injected interruption must fail");
            assert_eq!(interrupted.code, ErrorCode::InterruptedTransaction);
        }
        let corrected = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest,
                actor: "operator".into(),
                reason: "align recovered summary".into(),
                operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                    value: replacement.clone(),
                },
                fail_after_backup: false,
            },
        )
        .expect("correct recovered summary");
        let after_cards = store.load_cards(7).expect("cards after summary correction");
        let csdlc_v2::cards::CardContent::Spp(after_spp) = &after_cards[&CardKind::Spp].content
        else {
            panic!("SPP")
        };
        assert_eq!(after_spp.summary, replacement);
        for kind in [
            CardKind::Sip,
            CardKind::Stp,
            CardKind::Vpp,
            CardKind::Srp,
            CardKind::Sor,
        ] {
            assert_eq!(
                after_cards[&kind].content, before_cards[&kind].content,
                "{kind} changed during SPP-only correction"
            );
        }
        let audit: serde_json::Value =
            serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
                .expect("structured summary audit");
        assert_eq!(audit["operation"], "correct_plan_summary_after_recovery");
        let csdlc_v2::cards::CardContent::Spp(before_spp) = &before_cards[&CardKind::Spp].content
        else {
            panic!("SPP")
        };
        assert_eq!(audit["previous_value"], before_spp.summary);
        assert_eq!(audit["new_value"], replacement);
        if recovery_phase == LifecyclePhase::Reviewed {
            let assigned = assign_review(
                &store,
                ReviewAssignmentRequest {
                    issue: 7,
                    expected_generation: corrected.generation,
                    expected_digest: corrected.digest,
                    reviewer: "later-reviewer".into(),
                    assigned_by: "operator".into(),
                    scope: vec!["src".into()],
                },
            )
            .expect("assign retained review truth");
            let error = edit_issue(
                &store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Spp,
                    expected_generation: assigned.generation,
                    expected_digest: assigned.digest,
                    actor: "operator".into(),
                    reason: "reject stale transition provenance".into(),
                    operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                        value: "must remain blocked".into(),
                    },
                    fail_after_backup: false,
                },
            )
            .expect_err("retained review truth and stale provenance must fail");
            assert_eq!(error.code, ErrorCode::InvalidTransition);
        }
    }

    let (_temp, store, implemented) = implemented_fixture();
    let clean_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "reject clean implemented state".into(),
            operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                value: "must not apply".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("clean implemented issue must fail");
    assert_eq!(clean_error.code, ErrorCode::InvalidTransition);

    let mut transition_only = implemented.clone();
    transition_only.transitions.push(TransitionEvent {
        sequence: transition_only.transitions.len() as u64 + 1,
        from: LifecyclePhase::Implemented,
        to: LifecyclePhase::Reviewed,
        actor: "synthetic-review".into(),
        reason: "prove transition-only rejection".into(),
    });
    transition_only.transitions.push(TransitionEvent {
        sequence: transition_only.transitions.len() as u64 + 1,
        from: LifecyclePhase::Reviewed,
        to: LifecyclePhase::Implemented,
        actor: "synthetic-recovery".into(),
        reason: "prove transition-only rejection".into(),
    });
    write_consistent_record(store.root(), &mut transition_only);
    let transition_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: transition_only.generation,
            expected_digest: transition_only.digest,
            actor: "operator".into(),
            reason: "reject transition-only provenance".into(),
            operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                value: "must not apply".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("transition-only provenance must fail");
    assert_eq!(transition_error.code, ErrorCode::InvalidTransition);
    let mut restored = implemented.clone();
    write_consistent_record(store.root(), &mut restored);
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "operator".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("assign review on implemented issue");
    let audit_only = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "clear implemented review truth".into(),
        },
    )
    .expect("audit-only recovery");
    let before = std::fs::read(store.issue_dir(7).join("index.json")).expect("before rejection");
    for (actor, reason) in [
        (
            "operator",
            "audit-only recovery must not authorize correction",
        ),
        ("", "missing actor"),
        ("operator", " "),
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: audit_only.generation,
                expected_digest: audit_only.digest.clone(),
                actor: actor.into(),
                reason: reason.into(),
                operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                    value: "must not apply".into(),
                },
                fail_after_backup: false,
            },
        )
        .expect_err("invalid provenance/input must fail");
        assert!(matches!(
            error.code,
            ErrorCode::InvalidTransition | ErrorCode::InvalidInput
        ));
        assert_eq!(
            std::fs::read(store.issue_dir(7).join("index.json")).expect("after rejection"),
            before
        );
    }
}

#[test]
fn recovered_issue_can_correct_only_the_sip_required_outcome() {
    let (_temp, store, implemented) = implemented_fixture();
    let operation = SemanticOperation::CorrectRequiredOutcomeAfterRecovery {
        value: "a corrected four-child outcome".into(),
    };
    let unrecovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "must recover first".into(),
            operation: operation.clone(),
            fail_after_backup: false,
        },
    )
    .expect_err("unrecovered required-outcome correction must fail");
    assert_eq!(unrecovered.code, ErrorCode::InvalidTransition);
    let revision = csdlc_v2::git::substantive_revision(store.root(), &["src".into()])
        .expect("review revision");
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["src".into()],
                reviewed_revision: revision,
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record review");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "correct required outcome".into(),
        },
    )
    .expect("recover review");
    let before_cards = store
        .load_cards(7)
        .expect("cards before outcome correction");
    let replacement = "a corrected four-child outcome".to_string();
    for (card, value) in [
        (CardKind::Spp, replacement.clone()),
        (CardKind::Sip, " ".into()),
    ] {
        let rejected = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "prove correction rejection".into(),
                operation: SemanticOperation::CorrectRequiredOutcomeAfterRecovery { value },
                fail_after_backup: false,
            },
        )
        .expect_err("wrong-card or blank correction must fail");
        assert!(matches!(
            rejected.code,
            ErrorCode::InvalidTransition | ErrorCode::CardInvalid
        ));
    }
    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "align recovered required outcome".into(),
            operation: SemanticOperation::CorrectRequiredOutcomeAfterRecovery {
                value: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct recovered required outcome");
    let after_cards = store.load_cards(7).expect("cards after outcome correction");
    let csdlc_v2::cards::CardContent::Sip(after_sip) = &after_cards[&CardKind::Sip].content else {
        panic!("SIP")
    };
    assert_eq!(after_sip.required_outcome, replacement);
    for kind in [
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_eq!(
            after_cards[&kind].content, before_cards[&kind].content,
            "{kind} changed during SIP-only correction"
        );
    }
    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured required-outcome audit");
    assert_eq!(
        audit["operation"],
        "correct_required_outcome_after_recovery"
    );
    assert_eq!(audit["new_value"], replacement);
}

#[test]
fn recovered_implemented_issue_can_correct_only_stp_deliverables() {
    let (_temp, store, implemented) = implemented_fixture();
    let before_cards = store.load_cards(7).expect("load cards before correction");
    let csdlc_v2::cards::CardContent::Stp(before_stp) =
        before_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let replacement = vec!["src/lib.rs".into(), "src/validate.sh".into()];

    let unrecovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "correct reviewed denominator".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("ordinary implemented issue must not imply recovery");
    assert_eq!(unrecovered.code, ErrorCode::InvalidTransition);

    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "operator".into(),
            scope: vec!["csdlc-v2".into()],
        },
    )
    .expect("assign review");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "repair contradictory STP deliverables".into(),
        },
    )
    .expect("recover review");

    let stale = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation - 1,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "stale request".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("stale generation must fail closed");
    assert_eq!(stale.code, ErrorCode::StaleGeneration);

    let durable_before_stale_digest =
        std::fs::read(store.issue_dir(7).join("index.json")).expect("read durable record");
    let stale_digest = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation,
            expected_digest: "0".repeat(64),
            actor: "operator".into(),
            reason: "stale digest request".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("stale digest must fail closed");
    assert_eq!(stale_digest.code, ErrorCode::StaleDigest);
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).expect("reread durable record"),
        durable_before_stale_digest
    );

    for invalid in [
        Vec::<String>::new(),
        vec![" ".into()],
        vec!["src/lib.rs".into(), " src/lib.rs ".into()],
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Stp,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "reject malformed replacement".into(),
                operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                    values: invalid,
                },
                fail_after_backup: false,
            },
        )
        .expect_err("malformed replacement must fail closed");
        assert_eq!(error.code, ErrorCode::CardInvalid);
    }

    let wrong_card = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject wrong card".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("non-STP card must fail closed");
    assert_eq!(wrong_card.code, ErrorCode::InvalidTransition);

    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "align deliverables with reviewed plan".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct STP deliverables after recovery");
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
    let after_cards = store.load_cards(7).expect("load corrected cards");
    let csdlc_v2::cards::CardContent::Stp(after_stp) = after_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let mut expected_stp = before_stp.clone();
    expected_stp.deliverables = replacement.clone();
    assert_eq!(after_stp, expected_stp);

    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured audit operation");
    assert_eq!(
        audit["operation"],
        "correct_stp_deliverables_after_recovery"
    );
    assert_eq!(
        audit["previous_values"],
        serde_json::json!(before_stp.deliverables)
    );
    assert_eq!(audit["new_values"], serde_json::json!(replacement));
}

#[test]
fn stp_deliverable_correction_rejects_projection_drift() {
    let (_temp, store, implemented) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest,
            reviewer: "subagent".into(),
            assigned_by: "operator".into(),
            scope: vec!["csdlc-v2".into()],
        },
    )
    .expect("assign review");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "repair contradictory STP deliverables".into(),
        },
    )
    .expect("recover review");
    std::fs::write(
        store.issue_dir(7).join("cards/stp.md"),
        "# drifted projection\n",
    )
    .expect("drift STP projection");

    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "must reject drift".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: vec!["src/lib.rs".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("projection drift must fail closed");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}
fn git(root: &std::path::Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
fn evidence() -> ReviewEvidence {
    ReviewEvidence {
        reviewer: "bounded-subagent".into(),
        scope: vec!["csdlc-v2/".into()],
        reviewed_revision: "rev-2".into(),
        findings: vec![finding("F-1")],
        residual_risks: vec!["none known".into()],
        completed: true,
        non_substantive_proof: None,
    }
}

#[test]
fn exact_completed_review_with_resolved_findings_is_publishable() {
    let report = evaluate_publication_review(Some(&evidence()), "rev-2");
    assert!(report.ready);
    assert!(report.blocker_codes.is_empty());
}

#[test]
fn missing_incomplete_stale_and_unresolved_review_fail_closed() {
    assert_eq!(
        evaluate_publication_review(None, "rev").blocker_codes,
        vec!["review_missing"]
    );
    let mut value = evidence();
    value.completed = false;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_incomplete".into()));
    value.completed = true;
    assert!(evaluate_publication_review(Some(&value), "rev-3")
        .blocker_codes
        .contains(&"review_stale".into()));
    value.findings[0].disposition = FindingDisposition::Open;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"actionable_finding_unresolved".into()));
}

#[test]
fn guard_rejects_malformed_fixed_and_accepted_risk_evidence() {
    let mut value = evidence();
    value.findings[0].fix_revision = Some("wrong".into());
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_evidence_invalid".into()));
    value.findings[0].disposition = FindingDisposition::AcceptedRisk;
    value.findings[0].fix_revision = None;
    value.residual_risks.clear();
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"review_evidence_invalid".into()));
}

#[test]
fn out_of_scope_finding_must_remain_visible_and_routed() {
    let mut value = evidence();
    value.findings[0].in_scope = false;
    value.findings[0].disposition = FindingDisposition::OutOfScope;
    value.findings[0].fix_revision = None;
    value.findings[0].route = None;
    assert!(evaluate_publication_review(Some(&value), "rev-2")
        .blocker_codes
        .contains(&"out_of_scope_finding_unrouted".into()));
    value.findings[0].route = Some("follow-up:#999".into());
    assert!(evaluate_publication_review(Some(&value), "rev-2").ready);
}

#[test]
fn non_substantive_exception_is_narrow_and_machine_proven() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join(".csdlc/review")).expect("dir");
    std::fs::write(temp.path().join(".csdlc/review/result.json"), "one").expect("one");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "one"]);
    let from = git_out(temp.path(), &["rev-parse", "HEAD"]);
    std::fs::write(temp.path().join(".csdlc/review/result.json"), "two").expect("two");
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "two"]);
    let to = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let from_revision = csdlc_v2::git::clean_commit_revision(&from);
    let to_revision = csdlc_v2::git::clean_commit_revision(&to);
    let mut value = evidence();
    value.reviewed_revision = from_revision.clone();
    value.findings[0].fix_revision = Some(from_revision.clone());
    value.non_substantive_proof = Some(NonSubstantiveProof {
        policy: "review_metadata_only_v1".into(),
        from_revision,
        to_revision: to_revision.clone(),
        from_commit: from,
        to_commit: to,
        changed_paths: vec![".csdlc/review/result.json".into()],
    });
    assert!(evaluate_publication_review_in_repo(temp.path(), Some(&value), &to_revision).ready);
    value
        .non_substantive_proof
        .as_mut()
        .expect("proof")
        .changed_paths = vec!["src/lib.rs".into()];
    assert!(!evaluate_publication_review_in_repo(temp.path(), Some(&value), &to_revision).ready);
}

#[test]
fn typed_publication_metadata_commit_does_not_stale_review_but_source_drift_does() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/7/cards")).expect("cards");
    std::fs::create_dir_all(temp.path().join(".csdlc/requests")).expect("requests");
    std::fs::create_dir_all(temp.path().join(".csdlc/publication")).expect("publication");
    std::fs::write(temp.path().join("docs/design.md"), "reviewed\n").expect("design");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "reviewed source"]);
    let from = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let from_revision = csdlc_v2::git::clean_commit_revision(&from);
    let evidence = ReviewEvidence {
        reviewer: "subagent".into(),
        scope: vec!["docs".into()],
        reviewed_revision: from_revision,
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    };
    for (path, body) in [
        (".csdlc/issues/7/index.json", "{}\n"),
        (".csdlc/issues/7/audit.jsonl", "{}\n"),
        (".csdlc/issues/7/cards/sor.md", "card\n"),
        (".csdlc/issues/7/cards/sor.values.json", "{}\n"),
        (".csdlc/publication/7.intent.json", "{}\n"),
    ] {
        let target = temp.path().join(path);
        std::fs::write(target, body).expect("metadata");
    }
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "typed publication metadata"]);
    let to = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let current = csdlc_v2::git::clean_commit_revision(&to);
    assert!(evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &current).ready);

    std::fs::write(temp.path().join(".csdlc/requests/7-publish.json"), "{}\n")
        .expect("obsolete tracked request");
    git(temp.path(), &["add", ".csdlc/requests/7-publish.json"]);
    git(
        temp.path(),
        &["commit", "-m", "obsolete tracked request drift"],
    );
    let request_drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let request_drift_revision = csdlc_v2::git::clean_commit_revision(&request_drift);
    let request_report =
        evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &request_drift_revision);
    assert!(request_report
        .blocker_codes
        .contains(&"review_stale".into()));

    std::fs::write(
        temp.path().join(".csdlc/issues/7/cards/sor.md"),
        "hand-edited substantive card\n",
    )
    .expect("card drift");
    git(temp.path(), &["add", ".csdlc/issues/7/cards/sor.md"]);
    git(temp.path(), &["commit", "-m", "substantive card drift"]);
    let card_drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let card_drift_revision = csdlc_v2::git::clean_commit_revision(&card_drift);
    let card_report =
        evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &card_drift_revision);
    assert!(card_report.blocker_codes.contains(&"review_stale".into()));

    std::fs::write(temp.path().join("docs/new-source.md"), "substantive\n").expect("source");
    git(temp.path(), &["add", "docs/new-source.md"]);
    git(temp.path(), &["commit", "-m", "substantive drift"]);
    let drift = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let drift_revision = csdlc_v2::git::clean_commit_revision(&drift);
    let report = evaluate_publication_review_in_repo(temp.path(), Some(&evidence), &drift_revision);
    assert!(report.blocker_codes.contains(&"review_stale".into()));
}

#[test]
fn doctor_accepts_committed_typed_metadata_after_review() {
    let (temp, store, record) = implemented_fixture();
    let assigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest,
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec!["docs".into()],
        },
    )
    .expect("assignment");
    let revision = assigned
        .review_assignment
        .as_ref()
        .unwrap()
        .revision
        .clone();
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "subagent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["docs".into()],
                reviewed_revision: revision,
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("review");
    assert_eq!(reviewed.phase, LifecyclePhase::Reviewed);
    std::fs::create_dir_all(temp.path().join(".csdlc/publication")).expect("publication");
    std::fs::write(temp.path().join(".csdlc/publication/7.intent.json"), "{}\n").expect("intent");
    git(temp.path(), &["add", ".csdlc/publication/7.intent.json"]);
    git(temp.path(), &["commit", "-m", "typed publication metadata"]);
    let report = csdlc_v2::diagnose(&store, 7);
    assert!(!report
        .findings
        .iter()
        .any(|finding| finding.code == "review_publication_dead_end"));
}

#[test]
fn guard_cli_is_read_only_and_returns_typed_truth() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::write(temp.path().join("docs/review.md"), "review").expect("doc");
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(temp.path(), &["add", "docs"]);
    git(temp.path(), &["commit", "-m", "review"]);
    let revision =
        csdlc_v2::git::substantive_revision(temp.path(), &["docs".into()]).expect("revision");
    let mut reviewed = evidence();
    reviewed.reviewed_revision = revision.clone();
    reviewed.findings[0].fix_revision = Some(revision);
    let request_dir = tempfile::tempdir().expect("request dir");
    let path = request_dir.path().join("guard.json");
    std::fs::write(
        &path,
        serde_json::json!({"evidence":reviewed,"scope":["docs"]}).to_string(),
    )
    .expect("request");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_csdlc-review"))
        .args([
            "--root",
            temp.path().to_str().expect("root"),
            "guard",
            "--request",
            path.to_str().expect("request"),
        ])
        .output()
        .expect("CLI");
    assert!(output.status.success());
    let report: String = String::from_utf8(output.stdout).expect("UTF-8");
    assert!(report.contains("\"ready\":true"));
    assert!(
        !temp.path().join(".csdlc").exists(),
        "guard mutated repository"
    );
}
fn git_out(root: &std::path::Path, args: &[&str]) -> String {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("UTF-8")
        .trim()
        .into()
}
