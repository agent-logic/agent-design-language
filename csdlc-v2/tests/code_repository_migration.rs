use std::fs;
use std::process::Command;

use csdlc_v2::cards::{CardKind, PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    bind_issue, edit_issue, migrate_code_repository, prepare_publication, record_review,
    BindRequest, BootstrapRequest, CodeRepositoryMigrationRequest, EditRequest, ErrorCode,
    InitialCardInput, LifecyclePhase, PlanningProfile, PublicationLinkageMode, PublicationRequest,
    ReviewEvidence, ReviewRecordRequest, SemanticOperation, Store,
};

const ISSUE: u64 = 90;
const CODE_REPOSITORY: &str = "agent-logic/agent-design-language";
const ISSUE_REPOSITORY: &str = "legacy-owner/agent-design-language";

fn git(root: &std::path::Path, args: &[&str]) {
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

fn install_native_authority(root: &std::path::Path) {
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
}

fn bootstrap_request() -> BootstrapRequest {
    BootstrapRequest {
        issue: ISSUE,
        repository: ISSUE_REPOSITORY.into(),
        actor: "test".into(),
        design_path: "docs/design.md".into(),
        diagram_path: "docs/diagram.mmd".into(),
        design_reviewer: "architect".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "Code repository migration fixture".into(),
            slug: "code-repository-migration-fixture".into(),
            version: "v0.92".into(),
            goal: "Migrate one legacy repository identity.".into(),
            required_outcome: "Retain review truth and publish safely.".into(),
            declared_scope: vec!["src".into()],
            authority_boundary: vec!["exact origin only".into()],
            operator_constraints: vec!["no network mutation".into()],
            task_boundary: "Focused migration fixture.".into(),
            deliverables: vec!["src/validate.sh".into()],
            acceptance_criteria: vec!["migration is fail closed".into()],
            dependencies: vec!["git".into()],
            repo_inputs: vec!["src/lib.rs".into()],
            non_goals: vec!["remote publication".into()],
            plan_summary: "Bind, migrate, and prove publication preflight.".into(),
            steps: vec![PlanStep {
                id: "S1".into(),
                action: "migrate".into(),
                acceptance_ids: vec!["AC-1".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec!["src".into(), "src/validate.sh".into()],
            invariants: vec!["review remains current".into()],
            risks: vec!["wrong origin".into()],
            planning_profile: PlanningProfile::Migration,
            stop_conditions: vec!["identity mismatch".into()],
            validation_lanes: vec![ValidationLane {
                lane: "focused".into(),
                proof_role: "migration fixture".into(),
                acceptance_ids: vec!["AC-1".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 60,
                budget_tokens: 100,
                argv: vec!["bash".into(), "src/validate.sh".into()],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed.".into(),
            review_prompts: vec!["Review migration truth.".into()],
            review_scope: "src".into(),
        },
    }
}

fn ready_fixture() -> (tempfile::TempDir, Store) {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("docs")).unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("docs/design.md"), "# Approved design\n").unwrap();
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A --> B\n",
    )
    .unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub fn fixture() {}\n").unwrap();
    fs::write(
        temp.path().join("src/validate.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\ntest -f src/lib.rs\n",
    )
    .unwrap();
    install_native_authority(temp.path());
    git(temp.path(), &["init", "-b", "main"]);
    git(
        temp.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
    git(
        temp.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/legacy-owner/agent-design-language.git",
        ],
    );
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "fixture"]);

    let store = Store::new(temp.path());
    let initialized = csdlc_v2::initialize_native_json(
        &store,
        &serde_json::to_vec(&bootstrap_request()).unwrap(),
    )
    .unwrap();
    edit_issue(
        &store,
        EditRequest {
            issue: ISSUE,
            card: CardKind::Sip,
            expected_generation: initialized.generation,
            expected_digest: initialized.digest,
            actor: "test".into(),
            reason: "fixture ready".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .unwrap();
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "initialize issue"]);
    (temp, store)
}

fn fixture() -> (tempfile::TempDir, Store) {
    let (temp, store) = ready_fixture();
    git(temp.path(), &["switch", "-c", "codex/90"]);
    bind_issue(
        &store,
        BindRequest {
            issue: ISSUE,
            base_branch: "main".into(),
            branch: "codex/90".into(),
            worktree: ".".into(),
            code_repository: None,
        },
    )
    .unwrap();
    git(
        temp.path(),
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "bind issue"]);
    (temp, store)
}

fn request(store: &Store) -> CodeRepositoryMigrationRequest {
    let record = store.load_record(ISSUE).unwrap();
    CodeRepositoryMigrationRequest {
        schema: "csdlc.code_repository_migration_request.v1".into(),
        issue: ISSUE,
        code_repository: CODE_REPOSITORY.into(),
        expected_generation: record.generation,
        expected_digest: record.digest,
        actor: "test-migrator".into(),
        reason: "recover pre-field record".into(),
    }
}

fn implement(root: &std::path::Path, store: &Store) {
    let mut record = store.load_record(ISSUE).unwrap();
    for (card, operation) in [
        (
            CardKind::Sor,
            SemanticOperation::RecordExecution {
                summary: "implemented".into(),
                changes: vec!["src".into()],
                artifacts: vec!["fixture".into()],
            },
        ),
        (
            CardKind::Sip,
            SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Implemented,
            },
        ),
    ] {
        record = edit_issue(
            store,
            EditRequest {
                issue: ISSUE,
                card,
                expected_generation: record.generation,
                expected_digest: record.digest,
                actor: "test".into(),
                reason: "fixture transition".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .unwrap();
    }
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "implement fixture"]);
}

fn implement_and_review(root: &std::path::Path, store: &Store) {
    implement(root, store);
    let revision = csdlc_v2::git::substantive_revision(root, &["src".into()]).unwrap();
    let record = store.load_record(ISSUE).unwrap();
    record_review(
        store,
        ReviewRecordRequest {
            issue: ISSUE,
            expected_generation: record.generation,
            expected_digest: record.digest,
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
    .unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "record review"]);
}

#[test]
fn bound_record_adopts_exact_origin_and_retry_is_stale_digest() {
    let (_temp, store) = fixture();
    let before = store.load_record(ISSUE).unwrap();
    let cards_before = store.load_cards(ISSUE).unwrap();
    let initial_request = request(&store);
    let report = migrate_code_repository(&store, initial_request.clone()).unwrap();
    let after = store.load_record(ISSUE).unwrap();
    assert!(report.changed);
    assert_eq!(after.code_repository.as_deref(), Some(CODE_REPOSITORY));
    assert_eq!(after.phase, before.phase);
    assert_eq!(after.review, before.review);
    assert!(!after.audit.last().unwrap().operation.contains("https://"));
    let mut cards_after = store.load_cards(ISSUE).unwrap();
    for card in cards_after.values_mut() {
        card.identity.generation = before.generation;
    }
    assert_eq!(cards_after, cards_before);
    let audit_len = after.audit.len();
    assert_eq!(
        migrate_code_repository(&store, initial_request)
            .unwrap_err()
            .code,
        ErrorCode::StaleDigest
    );
    let retried = store.load_record(ISSUE).unwrap();
    assert_eq!(retried.generation, after.generation);
    assert_eq!(retried.audit.len(), audit_len);
    let mut current = request(&store);
    current.expected_generation = retried.generation;
    current.expected_digest = retried.digest;
    assert_eq!(
        migrate_code_repository(&store, current).unwrap_err().code,
        ErrorCode::InvalidTransition
    );
}

#[test]
fn implemented_record_migrates_without_changing_phase() {
    let (temp, store) = fixture();
    implement(temp.path(), &store);
    let before = store.load_record(ISSUE).unwrap();
    let report = migrate_code_repository(&store, request(&store)).unwrap();
    assert_eq!(report.phase, LifecyclePhase::Implemented);
    let after = store.load_record(ISSUE).unwrap();
    assert_eq!(after.phase, before.phase);
    assert_eq!(after.review, before.review);
}

#[test]
fn wrong_identity_dirty_tree_and_divergent_push_fail_without_mutation() {
    let (temp, store) = fixture();
    let before = fs::read(store.issue_dir(ISSUE).join("index.json")).unwrap();
    let mut wrong = request(&store);
    wrong.code_repository = "other/repo".into();
    assert_eq!(
        migrate_code_repository(&store, wrong).unwrap_err().code,
        ErrorCode::ReconciliationRequired
    );
    assert_eq!(
        fs::read(store.issue_dir(ISSUE).join("index.json")).unwrap(),
        before
    );

    fs::write(temp.path().join("untracked.txt"), "dirty\n").unwrap();
    assert_eq!(
        migrate_code_repository(&store, request(&store))
            .unwrap_err()
            .code,
        ErrorCode::UnsafeCheckout
    );
    fs::remove_file(temp.path().join("untracked.txt")).unwrap();

    git(
        temp.path(),
        &[
            "remote",
            "set-url",
            "--add",
            "--push",
            "origin",
            "https://github.com/other/repo.git",
        ],
    );
    let missing_topology = migrate_code_repository(&store, request(&store)).unwrap_err();
    assert_eq!(
        missing_topology.code,
        ErrorCode::ReconciliationRequired,
        "{missing_topology:?}"
    );
    assert_eq!(
        fs::read(store.issue_dir(ISSUE).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn wrong_branch_missing_topology_and_unsupported_phase_fail_closed() {
    let (temp, store) = fixture();
    let before = fs::read(store.issue_dir(ISSUE).join("index.json")).unwrap();
    git(temp.path(), &["switch", "-c", "other-branch"]);
    assert_eq!(
        migrate_code_repository(&store, request(&store))
            .unwrap_err()
            .code,
        ErrorCode::UnsafeCheckout
    );
    git(temp.path(), &["switch", "codex/90"]);
    assert_eq!(
        fs::read(store.issue_dir(ISSUE).join("index.json")).unwrap(),
        before
    );

    let path = store.issue_dir(ISSUE).join("index.json");
    let mut record = store.load_record(ISSUE).unwrap();
    record.worktree = None;
    record.digest.clear();
    record.digest = csdlc_v2::cards::digest(&serde_json::to_vec(&record).unwrap());
    fs::write(&path, serde_json::to_vec_pretty(&record).unwrap()).unwrap();
    let missing_topology = migrate_code_repository(&store, request(&store)).unwrap_err();
    assert_eq!(
        missing_topology.code,
        ErrorCode::CorruptRecord,
        "{missing_topology:?}"
    );

    let (_ready_temp, ready_store) = ready_fixture();
    assert_eq!(
        migrate_code_repository(&ready_store, request(&ready_store))
            .unwrap_err()
            .code,
        ErrorCode::InvalidTransition
    );
}

#[test]
fn reviewed_record_preserves_review_and_passes_split_publication_preflight() {
    let (temp, store) = fixture();
    implement_and_review(temp.path(), &store);
    let reviewed = store.load_record(ISSUE).unwrap();
    let review = reviewed.review.clone();
    migrate_code_repository(&store, request(&store)).unwrap();
    let migrated = store.load_record(ISSUE).unwrap();
    assert_eq!(migrated.phase, LifecyclePhase::Reviewed);
    assert_eq!(migrated.review, review);
    git(temp.path(), &["add", "."]);
    git(temp.path(), &["commit", "-m", "migrate code repository"]);
    let migrated = store.load_record(ISSUE).unwrap();
    let intent = prepare_publication(
        &store,
        &PublicationRequest {
            schema: "csdlc.publication_request.v1".into(),
            issue: ISSUE,
            repository: ISSUE_REPOSITORY.into(),
            code_repository: Some(CODE_REPOSITORY.into()),
            expected_generation: migrated.generation,
            expected_digest: migrated.digest,
            actor: "test-publisher".into(),
            base: "main".into(),
            head: "codex/90".into(),
            title: "Migration fixture".into(),
            body: format!("Closes {ISSUE_REPOSITORY}#{ISSUE}"),
            linkage_mode: PublicationLinkageMode::Closing,
            draft: true,
            remote: "origin".into(),
            token_file: None,
        },
    )
    .unwrap();
    assert_eq!(intent.repository, CODE_REPOSITORY);
    assert_eq!(intent.issue_repository, ISSUE_REPOSITORY);
}

#[test]
fn schema_and_cli_expose_typed_migration_contract() {
    let schema = csdlc_v2::public_schema_bundle();
    assert!(schema.get("code_repository_migration_request").is_some());
    assert!(schema.get("code_repository_migration_report").is_some());
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc-issue"))
        .arg("--help")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(stdout.contains("migrate-code-repository"));
}
