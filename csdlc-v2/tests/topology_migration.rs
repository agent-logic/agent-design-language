use std::fs;
use std::process::Command;

use csdlc_v2::cards::{digest, PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    initialize_native_json, migrate_bound_issue_identity, migrate_bound_topology,
    migrate_bound_topology_with_crash_for_test, migrate_bound_topology_with_failure_for_test,
    BootstrapRequest, BoundIssueIdentityMigrationRequest, BoundTopologyDisposition,
    BoundTopologyMigrationItem, BoundTopologyMigrationRequest, ClosedIssueEvidence,
    InitialCardInput, LifecyclePhase, MigrationIssueState, PlanningProfile, PublicationEvidence,
    PublicationLinkageMode, Store, TerminalDisposition,
};

fn git(root: &std::path::Path, args: &[&str]) {
    assert!(Command::new("git")
        .current_dir(root)
        .args(args)
        .status()
        .unwrap()
        .success());
}

fn fixture(issue: u64) -> (tempfile::TempDir, Store) {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    fs::create_dir_all(&repo).unwrap();
    let root = repo.as_path();
    fs::create_dir_all(root.join("docs/templates/prompts")).unwrap();
    fs::create_dir_all(root.join("csdlc-v2/operator")).unwrap();
    fs::create_dir_all(root.join("design")).unwrap();
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
    fs::write(root.join("design/design.md"), "# Approved design\n").unwrap();
    fs::write(root.join("design/diagram.mmd"), "flowchart LR\n A --> B\n").unwrap();
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    let request = bootstrap_request(issue);
    let store = Store::new(root);
    initialize_native_json(&store, &serde_json::to_vec(&request).unwrap()).unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "fixture"]);
    make_pre_topology_bound(&store, issue);
    (temp, store)
}

fn bootstrap_request(issue: u64) -> BootstrapRequest {
    BootstrapRequest {
        issue,
        repository: "example/repo".into(),
        actor: "test".into(),
        design_path: "design/design.md".into(),
        diagram_path: "design/diagram.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "Topology migration fixture".into(),
            slug: format!("topology-migration-{issue}"),
            version: "v0.92".into(),
            goal: "Migrate one record.".into(),
            required_outcome: "Produce truthful topology state.".into(),
            declared_scope: vec!["fixture".into()],
            authority_boundary: vec!["temporary repository".into()],
            operator_constraints: vec!["no network".into()],
            task_boundary: "Focused migration fixture.".into(),
            deliverables: vec!["migrated record".into()],
            acceptance_criteria: vec!["migration is idempotent".into()],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["design/design.md".into()],
            non_goals: vec!["publication".into()],
            plan_summary: "Migrate and verify.".into(),
            steps: vec![PlanStep {
                id: "S1".into(),
                action: "migrate".into(),
                acceptance_ids: vec!["AC-1".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec!["design/design.md".into()],
            invariants: vec!["no invented topology".into()],
            risks: vec!["invalid digest".into()],
            planning_profile: PlanningProfile::Migration,
            stop_conditions: vec!["ambiguous topology".into()],
            validation_lanes: vec![ValidationLane {
                lane: "focused".into(),
                proof_role: "migration regression".into(),
                acceptance_ids: vec!["AC-1".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 60,
                budget_tokens: 100,
                argv: vec!["cargo".into(), "test".into()],
                parallel_group: "test".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed.".into(),
            review_prompts: vec!["Review migration truth.".into()],
            review_scope: "fixture".into(),
        },
    }
}

fn make_pre_topology_bound(store: &Store, issue: u64) {
    let mut record = store.load_record(issue).unwrap();
    record.transitions.push(csdlc_v2::model::TransitionEvent {
        sequence: 1,
        from: LifecyclePhase::Initialized,
        to: LifecyclePhase::Ready,
        actor: "legacy".into(),
        reason: "automatic readiness verification".into(),
    });
    record.transitions.push(csdlc_v2::model::TransitionEvent {
        sequence: 2,
        from: LifecyclePhase::Ready,
        to: LifecyclePhase::Bound,
        actor: "legacy".into(),
        reason: "verified Git worktree binding".into(),
    });
    record.phase = LifecyclePhase::Bound;
    record.digest.clear();
    record.digest = digest(&serde_json::to_vec(&record).unwrap());
    fs::write(
        store.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
}

fn add_topology_candidate(
    root: &std::path::Path,
    issue: u64,
    branch: &str,
    suffix: &str,
) -> std::path::PathBuf {
    let path = root
        .parent()
        .unwrap()
        .join(format!("candidate-{issue}-{suffix}"));
    git(
        root,
        &[
            "worktree",
            "add",
            "-b",
            branch,
            path.to_str().unwrap(),
            "main",
        ],
    );
    let candidate = Store::new(&path);
    make_pre_topology_bound(&candidate, issue);
    let mut record = candidate.load_record(issue).unwrap();
    record.branch = Some(branch.into());
    record.worktree = Some(path.canonicalize().unwrap().to_string_lossy().into_owned());
    record.digest.clear();
    record.digest = digest(&serde_json::to_vec(&record).unwrap());
    fs::write(
        candidate.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    path
}

fn request(
    store: &Store,
    issue: u64,
    state: MigrationIssueState,
    apply: bool,
) -> BoundTopologyMigrationRequest {
    let evidence_dir = store.root().join("evidence");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        evidence_dir.join("states.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.bound_topology_issue_states.v1",
            "issues":[{
                "issue":issue,
                "state":state,
                "closed_at": if state == MigrationIssueState::Closed {
                    Some("2026-08-06T00:00:00Z")
                } else {
                    None
                }
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    if state == MigrationIssueState::Closed {
        fs::write(
            evidence_dir.join("closed.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "schema":"csdlc.bound_topology_terminal_observation.v1",
                "issue":issue,
                "issue_state":"closed",
                "closed_at":"2026-08-06T00:00:00Z",
                "disposition":"closed_no_pr",
                "pull_request":null,
                "observed_sha":null
            }))
            .unwrap(),
        )
        .unwrap();
    }
    BoundTopologyMigrationRequest {
        schema: "csdlc.bound_topology_migration_request.v1".into(),
        apply,
        actor: "test-migrator".into(),
        issue_state_evidence: "evidence/states.json".into(),
        issues: vec![BoundTopologyMigrationItem {
            issue,
            state,
            terminal: (state == MigrationIssueState::Closed).then(|| ClosedIssueEvidence {
                pull_request: None,
                disposition: TerminalDisposition::ClosedNoPr,
                observed_sha: None,
                observed_state: "closed".into(),
                receipt_path: "evidence/closed.json".into(),
            }),
        }],
    }
}

fn commit_all(root: &std::path::Path, message: &str) {
    git(root, &["add", "."]);
    git(root, &["commit", "-m", message]);
}

fn identity_target_evidence(store: &Store, target_issue: u64) -> String {
    let evidence_dir = store.root().join("evidence");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        evidence_dir.join("target-issue.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "csdlc.bound_issue_identity_target_evidence.v1",
            "repository": "example/repo",
            "issue": target_issue,
            "state": "open",
            "title": "Canonical current issue",
            "operation_key": "typed-create-current-issue"
        }))
        .unwrap(),
    )
    .unwrap();
    "evidence/target-issue.json".into()
}

fn identity_request(
    store: &Store,
    source_issue: u64,
    target_issue: u64,
) -> BoundIssueIdentityMigrationRequest {
    let record = store.load_record(source_issue).unwrap();
    BoundIssueIdentityMigrationRequest {
        schema: "csdlc.bound_issue_identity_migration_request.v1".into(),
        source_issue,
        target_issue,
        source_repository: "example/repo".into(),
        target_repository: "example/repo".into(),
        expected_generation: record.generation,
        expected_digest: record.digest,
        actor: "test-identity-migrator".into(),
        reason: "recover wrong issue identity".into(),
        target_issue_evidence: identity_target_evidence(store, target_issue),
    }
}

fn make_published_source(store: &Store, issue: u64) {
    let mut record = store.load_record(issue).unwrap();
    record.transitions.push(csdlc_v2::model::TransitionEvent {
        sequence: record.transitions.len() as u64 + 1,
        from: LifecyclePhase::Bound,
        to: LifecyclePhase::Implemented,
        actor: "fixture".into(),
        reason: "implemented fixture".into(),
    });
    record.transitions.push(csdlc_v2::model::TransitionEvent {
        sequence: record.transitions.len() as u64 + 1,
        from: LifecyclePhase::Implemented,
        to: LifecyclePhase::Reviewed,
        actor: "fixture".into(),
        reason: "reviewed fixture".into(),
    });
    record.transitions.push(csdlc_v2::model::TransitionEvent {
        sequence: record.transitions.len() as u64 + 1,
        from: LifecyclePhase::Reviewed,
        to: LifecyclePhase::Published,
        actor: "fixture".into(),
        reason: "published fixture".into(),
    });
    record.phase = LifecyclePhase::Published;
    record.publication = Some(PublicationEvidence {
        repository: "example/repo".into(),
        issue,
        pull_request: 320,
        url: "https://github.com/example/repo/pull/320".into(),
        base: "main".into(),
        head: "codex/source".into(),
        revision: "git-blake3:0123456789012345678901234567890123456789:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    record.digest.clear();
    record.digest = digest(&serde_json::to_vec(&record).unwrap());
    fs::write(
        store.issue_dir(issue).join("index.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
}

#[test]
fn open_record_dry_run_apply_and_second_run_are_truthful() {
    let (_temp, store) = fixture(42);
    let dry_run = migrate_bound_topology(
        &store,
        request(&store, 42, MigrationIssueState::Open, false),
    )
    .unwrap();
    assert_eq!(dry_run.changed, 0);
    assert_eq!(
        dry_run.results[0].disposition,
        BoundTopologyDisposition::ResetInitialized
    );

    let applied =
        migrate_bound_topology(&store, request(&store, 42, MigrationIssueState::Open, true))
            .unwrap();
    assert_eq!(applied.changed, 1);
    assert_eq!(
        store.load_record(42).unwrap().phase,
        LifecyclePhase::Initialized
    );

    let second =
        migrate_bound_topology(&store, request(&store, 42, MigrationIssueState::Open, true))
            .unwrap();
    assert_eq!(second.changed, 0);
    assert_eq!(
        second.results[0].disposition,
        BoundTopologyDisposition::AlreadyCurrent
    );
}

#[test]
fn bound_issue_identity_migration_moves_published_source_to_canonical_issue() {
    let (_temp, store) = fixture(5913);
    make_published_source(&store, 5913);
    let request = identity_request(&store, 5913, 322);
    commit_all(store.root(), "published wrong issue identity");

    let report = migrate_bound_issue_identity(&store, request).unwrap();
    assert!(report.changed);
    assert_eq!(report.source_issue, 5913);
    assert_eq!(report.target_issue, 322);
    assert_eq!(report.evidence.previous_phase, LifecyclePhase::Published);
    assert_eq!(report.evidence.resulting_phase, LifecyclePhase::Published);
    assert!(report.evidence.cleared_publication);
    assert!(!store.issue_dir(5913).exists());

    let migrated = store.load_record(322).unwrap();
    assert_eq!(migrated.issue, 322);
    assert_eq!(migrated.repository, "example/repo");
    assert_eq!(migrated.phase, LifecyclePhase::Published);
    assert!(migrated.publication.is_none());
    assert!(migrated.readiness.is_none());
    assert!(migrated
        .audit
        .last()
        .unwrap()
        .operation
        .contains("\"source_issue\":5913"));
    for card in store.load_cards(322).unwrap().values() {
        assert_eq!(card.identity.issue, 322);
        assert_eq!(card.identity.repository, "example/repo");
        assert_eq!(card.identity.generation, migrated.generation);
    }
}

#[test]
fn bound_issue_identity_migration_fails_closed_on_stale_conflict_dirty_and_terminal() {
    let (_temp, store) = fixture(5913);
    commit_all(store.root(), "bound wrong issue identity");
    let mut stale = identity_request(&store, 5913, 322);
    stale.expected_generation += 1;
    assert_eq!(
        migrate_bound_issue_identity(&store, stale)
            .unwrap_err()
            .code,
        csdlc_v2::ErrorCode::StaleDigest
    );

    fs::create_dir_all(store.issue_dir(322)).unwrap();
    assert_eq!(
        migrate_bound_issue_identity(&store, identity_request(&store, 5913, 322))
            .unwrap_err()
            .code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    fs::remove_dir_all(store.issue_dir(322)).unwrap();

    fs::write(store.root().join("untracked.txt"), "dirty\n").unwrap();
    assert_eq!(
        migrate_bound_issue_identity(&store, identity_request(&store, 5913, 322))
            .unwrap_err()
            .code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    );
    fs::remove_file(store.root().join("untracked.txt")).unwrap();

    let mut terminal = store.load_record(5913).unwrap();
    terminal.phase = LifecyclePhase::ClosedOut;
    terminal.terminal = Some(csdlc_v2::TerminalEvidence {
        pull_request: None,
        disposition: TerminalDisposition::ClosedNoPr,
        observed_sha: None,
        observed_state: "closed".into(),
        receipt_path: "evidence/terminal.json".into(),
        branch: None,
        worktree: None,
    });
    terminal.digest.clear();
    terminal.digest = digest(&serde_json::to_vec(&terminal).unwrap());
    fs::write(
        store.issue_dir(5913).join("index.json"),
        serde_json::to_vec_pretty(&terminal).unwrap(),
    )
    .unwrap();
    commit_all(store.root(), "terminal wrong issue identity");
    assert_eq!(
        migrate_bound_issue_identity(&store, identity_request(&store, 5913, 322))
            .unwrap_err()
            .code,
        csdlc_v2::ErrorCode::InvalidTransition
    );
}

#[test]
fn closed_record_becomes_terminal_without_topology() {
    let (_temp, store) = fixture(43);
    let report = migrate_bound_topology(
        &store,
        request(&store, 43, MigrationIssueState::Closed, true),
    )
    .unwrap();
    assert_eq!(
        report.results[0].disposition,
        BoundTopologyDisposition::ClosedOut
    );
    let record = store.load_record(43).unwrap();
    assert_eq!(record.phase, LifecyclePhase::ClosedOut);
    assert_eq!(
        record.terminal.unwrap().disposition,
        TerminalDisposition::ClosedNoPr
    );
}

#[test]
fn digest_tampering_fails_before_migration() {
    let (_temp, store) = fixture(44);
    let path = store.issue_dir(44).join("index.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["digest"] = serde_json::Value::String("tampered".into());
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let error =
        migrate_bound_topology(&store, request(&store, 44, MigrationIssueState::Open, true))
            .unwrap_err();
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
}

#[test]
fn verified_existing_topology_is_adopted() {
    let (_temp, store) = fixture(45);
    let candidate = add_topology_candidate(store.root(), 45, "issue-45", "one");
    let report =
        migrate_bound_topology(&store, request(&store, 45, MigrationIssueState::Open, true))
            .unwrap();
    assert_eq!(
        report.results[0].disposition,
        BoundTopologyDisposition::AdoptedVerifiedTopology
    );
    let record = store.load_record(45).unwrap();
    assert_eq!(record.branch.as_deref(), Some("issue-45"));
    assert_eq!(
        record.worktree.as_deref(),
        Some(candidate.canonicalize().unwrap().to_string_lossy().as_ref())
    );
}

#[test]
fn ambiguous_existing_topology_fails_without_source_mutation() {
    let (_temp, store) = fixture(46);
    add_topology_candidate(store.root(), 46, "issue-46-a", "one");
    add_topology_candidate(store.root(), 46, "issue-46-b", "two");
    let before = fs::read(store.issue_dir(46).join("index.json")).unwrap();
    let error =
        migrate_bound_topology(&store, request(&store, 46, MigrationIssueState::Open, true))
            .unwrap_err();
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read(store.issue_dir(46).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn topology_owned_by_another_issue_is_rejected() {
    let (_temp, store) = fixture(47);
    let candidate = add_topology_candidate(store.root(), 47, "issue-47", "one");
    let conflicting = candidate.join(".csdlc/issues/999");
    fs::create_dir_all(&conflicting).unwrap();
    fs::write(
        conflicting.join("index.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue":999,
            "branch":"different-branch",
            "worktree":"../candidate-47-one"
        }))
        .unwrap(),
    )
    .unwrap();
    let before = fs::read(store.issue_dir(47).join("index.json")).unwrap();
    let error =
        migrate_bound_topology(&store, request(&store, 47, MigrationIssueState::Open, true))
            .unwrap_err();
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read(store.issue_dir(47).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn mid_batch_failure_restores_every_issue_byte_for_byte() {
    let (_temp, store) = fixture(48);
    initialize_native_json(&store, &serde_json::to_vec(&bootstrap_request(49)).unwrap()).unwrap();
    make_pre_topology_bound(&store, 49);
    let evidence_dir = store.root().join("evidence");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        evidence_dir.join("states.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.bound_topology_issue_states.v1",
            "issues":[
                {"issue":48,"state":"open","closed_at":null},
                {"issue":49,"state":"open","closed_at":null}
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    let request = BoundTopologyMigrationRequest {
        schema: "csdlc.bound_topology_migration_request.v1".into(),
        apply: true,
        actor: "test-migrator".into(),
        issue_state_evidence: "evidence/states.json".into(),
        issues: vec![48, 49]
            .into_iter()
            .map(|issue| BoundTopologyMigrationItem {
                issue,
                state: MigrationIssueState::Open,
                terminal: None,
            })
            .collect(),
    };
    let before = [48, 49].map(|issue| {
        (
            fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
            fs::read(store.issue_dir(issue).join("audit.jsonl")).unwrap(),
        )
    });
    let error = migrate_bound_topology_with_failure_for_test(&store, request, 1).unwrap_err();
    assert_eq!(error.code, csdlc_v2::ErrorCode::InterruptedTransaction);
    for (offset, issue) in [48, 49].into_iter().enumerate() {
        assert_eq!(
            fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
            before[offset].0
        );
        assert_eq!(
            fs::read(store.issue_dir(issue).join("audit.jsonl")).unwrap(),
            before[offset].1
        );
    }
}

#[test]
fn next_run_recovers_a_durable_interrupted_batch() {
    let (_temp, store) = fixture(50);
    initialize_native_json(&store, &serde_json::to_vec(&bootstrap_request(51)).unwrap()).unwrap();
    make_pre_topology_bound(&store, 51);
    let evidence_dir = store.root().join("evidence");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        evidence_dir.join("states.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.bound_topology_issue_states.v1",
            "issues":[
                {"issue":50,"state":"open","closed_at":null},
                {"issue":51,"state":"open","closed_at":null}
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    let request = BoundTopologyMigrationRequest {
        schema: "csdlc.bound_topology_migration_request.v1".into(),
        apply: true,
        actor: "test-migrator".into(),
        issue_state_evidence: "evidence/states.json".into(),
        issues: vec![50, 51]
            .into_iter()
            .map(|issue| BoundTopologyMigrationItem {
                issue,
                state: MigrationIssueState::Open,
                terminal: None,
            })
            .collect(),
    };
    let before = [50, 51].map(|issue| fs::read(store.issue_dir(issue).join("index.json")).unwrap());
    migrate_bound_topology_with_crash_for_test(&store, request.clone(), 1).unwrap_err();
    assert_eq!(
        store.load_record(50).unwrap().phase,
        LifecyclePhase::Initialized
    );

    let report = migrate_bound_topology(&store, request).unwrap();
    assert_eq!(report.changed, 2);
    for (offset, issue) in [50, 51].into_iter().enumerate() {
        assert_ne!(
            fs::read(store.issue_dir(issue).join("index.json")).unwrap(),
            before[offset]
        );
        assert_eq!(
            store.load_record(issue).unwrap().phase,
            LifecyclePhase::Initialized
        );
    }
    assert!(!store
        .root()
        .join(".csdlc/issues/.bound-topology-migration-backup")
        .exists());
}

#[test]
fn concurrent_runs_are_serialized_by_the_repository_lock() {
    let (_temp, store) = fixture(52);
    let request = request(&store, 52, MigrationIssueState::Open, true);
    let root = store.root().to_path_buf();
    let mut changed = std::thread::scope(|scope| {
        let first_request = request.clone();
        let first_root = root.clone();
        let first = scope.spawn(move || {
            migrate_bound_topology(&Store::new(first_root), first_request)
                .unwrap()
                .changed
        });
        let second = scope.spawn(move || {
            migrate_bound_topology(&Store::new(root), request)
                .unwrap()
                .changed
        });
        vec![first.join().unwrap(), second.join().unwrap()]
    });
    changed.sort_unstable();
    assert_eq!(changed, vec![0, 1]);
    assert_eq!(
        store.load_record(52).unwrap().phase,
        LifecyclePhase::Initialized
    );
}
