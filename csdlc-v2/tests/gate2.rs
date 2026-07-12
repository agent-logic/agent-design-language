use std::fs;

use csdlc_v2::{
    bootstrap_issue, diagnose, edit_issue, BootstrapRequest, CardKind, Claim, EditRequest,
    ErrorCode, SemanticOperation, Store,
};
use tempfile::TempDir;

fn fixture() -> (TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# Design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram");
    let store = Store::new(temp.path());
    let record = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue: 42,
            repository: "example/repo".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "reviewer".into(),
            claim: Claim {
                id: "claim-1".into(),
                owner: "agent".into(),
                generation: 0,
                acquired_unix_seconds: 1,
                expires_unix_seconds: u64::MAX,
                heartbeat_unix_seconds: 1,
                protected_paths: vec!["src".into()],
                purpose: "test".into(),
            },
            initial: csdlc_v2::InitialCardInput {
                title: "Gate 2 fixture".into(),
                slug: "gate-2-fixture".into(),
                version: "v0.92".into(),
                goal: "Prove Gate 2.".into(),
                required_outcome: "Construct and validate six typed cards.".into(),
                declared_scope: vec!["fixture record".into()],
                authority_boundary: vec!["no network".into()],
                task_boundary: "Implement only the fixture.".into(),
                deliverables: vec!["record".into()],
                acceptance_criteria: vec!["six cards exist".into(), "doctor is ready".into()],
                dependencies: vec!["none".into()],
                repo_inputs: vec!["docs/design.md".into()],
                non_goals: vec!["GitHub".into()],
                plan_summary: "Build then diagnose.".into(),
                steps: vec![csdlc_v2::cards::PlanStep {
                    id: "step-1".into(),
                    action: "construct and diagnose".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    status: csdlc_v2::cards::StepStatus::Pending,
                }],
                invariants: vec!["atomic record".into()],
                risks: vec!["interruption".into()],
                planning_profile: csdlc_v2::PlanningProfile::Small,
                stop_conditions: vec!["invariant failure".into()],
                validation_lanes: vec![csdlc_v2::cards::ValidationLane {
                    lane: "focused".into(),
                    proof_role: "Gate 2 behavior".into(),
                    acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                    deterministic: true,
                    resource_profile: csdlc_v2::cards::ResourceProfile::Small,
                    budget_seconds: 120,
                    budget_tokens: 1_000,
                    argv: vec!["cargo".into(), "test".into()],
                    parallel_group: "local".into(),
                    defer_reason: None,
                }],
                failure_policy: "Fail closed.".into(),
                review_prompts: vec!["Review correctness.".into()],
            },
        },
    )
    .expect("bootstrap");
    (temp, store, record)
}

fn edit(record: &csdlc_v2::IssueRecord, operation: SemanticOperation) -> EditRequest {
    EditRequest {
        issue: 42,
        card: CardKind::Sip,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        claim_id: "claim-1".into(),
        actor: "agent".into(),
        reason: "test edit".into(),
        operation,
        fail_after_backup: false,
    }
}

#[test]
fn bootstrap_constructs_all_six_cards_and_ready_doctor() {
    let (_temp, store, record) = fixture();
    assert_eq!(record.cards.len(), 6);
    assert_eq!(store.load_cards(42).expect("cards").len(), 6);
    assert_eq!(
        sip_goal(&store.load_cards(42).expect("cards")),
        "Prove Gate 2."
    );
    let report = diagnose(&store, 42);
    assert!(report.ready, "{report:?}");
    assert!(report.findings.is_empty());
}

#[test]
fn semantic_edit_updates_one_owned_projection_atomically() {
    let (_temp, store, record) = fixture();
    let next = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::Goal,
                value: "Ship the small state engine.".into(),
            },
        ),
    )
    .expect("edit");
    assert_eq!(next.generation, 1);
    let cards = store.load_cards(42).expect("cards");
    assert_eq!(sip_goal(&cards), "Ship the small state engine.");
    let audit = fs::read_to_string(store.issue_dir(42).join("audit.jsonl")).expect("audit");
    let events: Vec<serde_json::Value> = audit
        .lines()
        .map(|line| serde_json::from_str(line).expect("audit event"))
        .collect();
    assert_eq!(events.len(), 2);
    assert_eq!(events[1]["generation"], 1);
    assert!(events[1]["operation"]
        .as_str()
        .expect("operation")
        .contains("set_field"));
    assert!(diagnose(&store, 42).findings.is_empty());
}

#[test]
fn field_ownership_violation_fails_without_generation_change() {
    let (_temp, store, record) = fixture();
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::PlanSummary,
                value: "wrong owner".into(),
            },
        ),
    )
    .expect_err("ownership failure");
    assert!(matches!(error.code, ErrorCode::FieldOwnership));
    assert_eq!(store.load_record(42).expect("record").generation, 0);
}

#[test]
fn stale_generation_and_digest_fail_closed() {
    let (_temp, store, record) = fixture();
    let mut stale_generation = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "x".into(),
        },
    );
    stale_generation.expected_generation = 9;
    assert!(matches!(
        edit_issue(&store, stale_generation)
            .expect_err("stale generation")
            .code,
        ErrorCode::StaleGeneration
    ));

    let mut stale_digest = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "x".into(),
        },
    );
    stale_digest.expected_digest = "bad".into();
    assert!(matches!(
        edit_issue(&store, stale_digest)
            .expect_err("stale digest")
            .code,
        ErrorCode::StaleDigest
    ));
}

#[test]
fn illegal_transition_fails_closed() {
    let (_temp, store, record) = fixture();
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect_err("skip ready");
    assert!(matches!(error.code, ErrorCode::InvalidTransition));
    assert_eq!(
        store.load_record(42).expect("record").phase,
        csdlc_v2::LifecyclePhase::Initialized
    );
}

#[test]
fn direct_markdown_drift_is_corruption() {
    let (_temp, store, _record) = fixture();
    fs::write(
        store.issue_dir(42).join("cards/sip.md"),
        "# edited by hand\n",
    )
    .expect("drift");
    let report = diagnose(&store, 42);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Corrupt
    ));
    assert_eq!(report.findings[0].code, "corrupt_record");
}

#[test]
fn missing_design_or_diagram_blocks_readiness() {
    let (temp, store, _record) = fixture();
    fs::remove_file(temp.path().join("docs/diagram.mmd")).expect("remove diagram");
    let report = diagnose(&store, 42);
    assert!(!report.ready);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.code == "diagram_missing"));
}

#[test]
fn interrupted_commit_keeps_complete_backup_and_next_writer_recovers() {
    let (_temp, store, record) = fixture();
    let mut interrupted = edit(
        &record,
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: "interrupted".into(),
        },
    );
    interrupted.fail_after_backup = true;
    assert!(matches!(
        edit_issue(&store, interrupted)
            .expect_err("injected failure")
            .code,
        ErrorCode::InterruptedTransaction
    ));
    assert!(!store.issue_dir(42).exists());
    assert!(store.interrupted_backup(42).exists());
    assert!(matches!(
        diagnose(&store, 42).status,
        csdlc_v2::doctor::DoctorStatus::Interrupted
    ));

    let recovered = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::Goal,
                value: "recovered".into(),
            },
        ),
    )
    .expect("recover and edit");
    assert_eq!(recovered.generation, 1);
    assert!(!store.interrupted_backup(42).exists());
    assert!(diagnose(&store, 42).findings.is_empty());
}

fn sip_goal(cards: &std::collections::BTreeMap<CardKind, csdlc_v2::CardValues>) -> &str {
    match &cards[&CardKind::Sip].content {
        csdlc_v2::cards::CardContent::Sip(values) => &values.goal,
        _ => unreachable!("SIP content"),
    }
}

#[test]
fn public_schema_bundle_covers_requests_state_and_doctor_output() {
    let schema = csdlc_v2::public_schema_bundle();
    assert_eq!(schema["schema"], "csdlc.public_schema_bundle.v1");
    for key in [
        "bootstrap_request",
        "edit_request",
        "issue_record",
        "doctor_report",
    ] {
        assert!(schema[key].is_object(), "missing schema for {key}");
        assert!(
            schema[key]["properties"].is_object(),
            "missing root properties for {key}"
        );
    }
}

#[test]
fn doctor_rejects_index_digest_tampering() {
    let (_temp, store, _record) = fixture();
    let path = store.issue_dir(42).join("index.json");
    let mut index: serde_json::Value =
        serde_json::from_slice(&fs::read(&path).expect("index")).expect("index JSON");
    index["generation"] = 99.into();
    fs::write(&path, serde_json::to_vec_pretty(&index).expect("serialize")).expect("tamper");
    let report = diagnose(&store, 42);
    assert!(matches!(
        report.status,
        csdlc_v2::doctor::DoctorStatus::Corrupt
    ));
    assert!(report.findings[0].message.contains("digest"));
}

#[test]
fn ready_transition_requires_current_design_and_automatic_budgets() {
    let (temp, store, record) = fixture();
    let cards = store.load_cards(42).expect("cards");
    match (
        &cards[&CardKind::Spp].content,
        &cards[&CardKind::Vpp].content,
    ) {
        (csdlc_v2::cards::CardContent::Spp(spp), csdlc_v2::cards::CardContent::Vpp(vpp)) => {
            assert_eq!(spp.execution_estimates.elapsed_seconds, 7_200);
            assert_eq!(spp.execution_estimates.total_tokens, 40_000);
            assert_eq!(vpp.planned_validation_seconds, 1_200);
            assert_eq!(vpp.planned_validation_tokens, 10_000);
        }
        _ => unreachable!("planning cards"),
    }
    let mut over_budget = cards.clone();
    if let csdlc_v2::cards::CardContent::Vpp(vpp) =
        &mut over_budget.get_mut(&CardKind::Vpp).expect("VPP").content
    {
        vpp.lanes[0].budget_tokens = vpp.planned_validation_tokens + 1;
    }
    let design_digest =
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/design.md")).expect("design"));
    let diagram_digest =
        csdlc_v2::cards::digest(&fs::read(temp.path().join("docs/diagram.mmd")).expect("diagram"));
    assert!(csdlc_v2::cards::validate_cross_card(
        &over_budget,
        "docs/design.md",
        &design_digest,
        "docs/diagram.mmd",
        &diagram_digest,
    )
    .is_err());
    fs::remove_file(temp.path().join("docs/design.md")).expect("remove design");
    let error = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect_err("stale design must block ready");
    assert!(matches!(error.code, ErrorCode::Io | ErrorCode::CardInvalid));

    let (_other_temp, other_store, other_record) = fixture();
    let ready = edit_issue(
        &other_store,
        edit(
            &other_record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("valid readiness transition");
    assert_eq!(ready.phase, csdlc_v2::LifecyclePhase::Ready);
}

#[test]
fn sor_status_cannot_fabricate_terminal_truth() {
    let (_temp, store, record) = fixture();
    let mut sor = store
        .load_cards(42)
        .expect("cards")
        .remove(&CardKind::Sor)
        .expect("SOR");
    csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::AdvanceStatus {
            status: csdlc_v2::CardStatus::Ready,
        },
    )
    .expect("activate SOR values");
    let error = csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::AdvanceStatus {
            status: csdlc_v2::CardStatus::Complete,
        },
    )
    .expect_err("premature SOR completion");
    assert!(matches!(error.code, ErrorCode::InvalidTransition));

    let empty_evidence = csdlc_v2::cards::apply(
        &mut sor,
        &SemanticOperation::RecordValidation {
            result: csdlc_v2::cards::ValidationResult {
                command: Vec::new(),
                purpose: String::new(),
                outcome: csdlc_v2::cards::EvidenceOutcome::Passed,
                evidence_ref: String::new(),
            },
        },
    )
    .expect_err("empty evidence");
    assert!(matches!(empty_evidence.code, ErrorCode::CardInvalid));

    let ready = edit_issue(
        &store,
        edit(
            &record,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Ready,
            },
        ),
    )
    .expect("ready");
    let bound = edit_issue(
        &store,
        edit(
            &ready,
            SemanticOperation::AdvancePhase {
                phase: csdlc_v2::LifecyclePhase::Bound,
            },
        ),
    )
    .expect("bound");
    let mut premature_closeout = edit(
        &bound,
        SemanticOperation::RecordCloseout {
            integration_state: csdlc_v2::cards::IntegrationState::Merged,
            publication_state: csdlc_v2::cards::PublicationState::Closed,
            merge_state: csdlc_v2::cards::MergeState::Merged,
            closeout_state: csdlc_v2::cards::CloseoutState::Complete,
        },
    );
    premature_closeout.card = CardKind::Sor;
    assert!(matches!(
        edit_issue(&store, premature_closeout)
            .expect_err("closeout while bound")
            .code,
        ErrorCode::InvalidTransition
    ));
}
