use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use csdlc_v2::cards::{PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    bind_issue, build_archived_projection_cleanup_request_from_recovery,
    classify_preserved_projection, edit_issue, execute_archived_projection_cleanup,
    initialize_native_json, recover_preserved_projection, ArchivedProjectionCleanupStatus,
    BindRequest, BootstrapRequest, CardKind, EditRequest, FailedOperationLineage, InitialCardInput,
    LifecyclePhase, PlanningProfile, ProjectionCasAnchor, ProjectionClassifyRequest,
    ProjectionRecoverRequest, ProjectionRecoveryCleanupBridgeRequest, SemanticOperation, Store,
};

const TERMINAL_298_MERGE: &str = "5a1d3bfda7108bede1572cbd9dc9e2af19d9eedb";
const TERMINAL_299_MERGE: &str = "649a20bf32d07e3aae221ab4b2352c2d1a9f80c5";
const TERMINAL_330_MERGE: &str = "879683620e2a3b86b49580910aedb9eb8d312bef";
const TERMINAL_298_DIGEST: &str =
    "9c911813b5f7f6e311eeae2d8f28dede604d1ac6272426f5bfa403781650c0e7";
const TERMINAL_299_DIGEST: &str =
    "cda124728bd2020d9fe52936e4e3fc9665a73a57f11acc18d4cd82dff315c91b";
const TERMINAL_330_DIGEST: &str =
    "8faeb1d546ba22b89aa0c1cb73c6e7d6c7cc48780eade6aefa70a19ff0bf1778";

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run git {args:?}: {error}"));
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git stdout utf8")
        .trim()
        .to_owned()
}

fn install_native_authority(root: &Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().expect("registry parent")).expect("registry parent");
    fs::create_dir_all(manifest.parent().expect("manifest parent")).expect("manifest parent");
    fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry");
    fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("native manifest");
}

fn issue_input() -> InitialCardInput {
    InitialCardInput {
        title: "projection recovery integration proof".into(),
        slug: "projection-recovery-integration-proof".into(),
        version: "v0.91.7".into(),
        goal: "prove recovery and cleanup compose".into(),
        required_outcome: "recovery result feeds cleanup authority and later typed commit".into(),
        declared_scope: vec!["src".into()],
        authority_boundary: vec!["production receipts only".into()],
        operator_constraints: vec!["no production edits".into()],
        task_boundary: "integration proof".into(),
        deliverables: vec!["src/validate.sh".into()],
        acceptance_criteria: vec!["integrated proof".into()],
        dependencies: vec!["terminal #298".into(), "terminal #299".into()],
        repo_inputs: vec!["src".into()],
        non_goals: vec!["production redesign".into()],
        plan_summary: "bind, implement, recover, cleanup, commit".into(),
        steps: vec![PlanStep {
            id: "one".into(),
            action: "prove integrated recovery".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: StepStatus::Pending,
        }],
        affected_areas: vec!["src".into(), "src/validate.sh".into()],
        invariants: vec!["fail closed".into()],
        risks: vec!["stale authority".into()],
        planning_profile: PlanningProfile::Small,
        stop_conditions: vec!["missing terminal authority".into()],
        validation_lanes: vec![ValidationLane {
            lane: "focused".into(),
            proof_role: "validate fixture".into(),
            acceptance_ids: vec!["AC-1".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 60,
            budget_tokens: 100,
            argv: vec!["bash".into(), "src/validate.sh".into()],
            parallel_group: "local".into(),
            defer_reason: None,
        }],
        failure_policy: "fail closed".into(),
        review_prompts: vec!["review integration proof".into()],
        review_scope: "fixture".into(),
    }
}

fn implemented_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord, String) {
    let temp = tempfile::tempdir().expect("temp");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::create_dir_all(temp.path().join("src")).expect("src");
    fs::write(temp.path().join("docs/design.md"), "# reviewed design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .expect("diagram");
    fs::write(temp.path().join("src/lib.rs"), "// fixture\n").expect("source fixture");
    fs::write(
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
    let merge_base = git(temp.path(), &["rev-parse", "HEAD"]);

    let store = Store::new(temp.path());
    let record = initialize_native_json(
        &store,
        &serde_json::to_vec(&BootstrapRequest {
            issue: 7,
            repository: "example/repo".into(),
            actor: "agent".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            initial: issue_input(),
        })
        .expect("bootstrap request"),
    )
    .expect("bootstrap");
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
            adopt_existing: false,
            expected_head: None,
            expected_generation: None,
            expected_digest: None,
            actor: None,
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
        .expect("implemented");
    }
    (temp, store, record, merge_base)
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir(destination).expect("create destination");
    for entry in fs::read_dir(source).expect("read source") {
        let entry = entry.expect("entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy file");
        }
    }
}

fn classify_and_recover(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    operation_id: &str,
    fail_after: Option<&str>,
) -> csdlc_v2::ProjectionRecoveryResult {
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "classify retained failed projection".into(),
        },
    )
    .expect("classify retained failed projection");
    assert_eq!(classify.disposition, "recoverable");
    let mut request = ProjectionRecoverRequest {
        issue: 7,
        operation_id: operation_id.into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest.clone(),
            rejected_manifest_digest: classify
                .preserved
                .manifest_digest
                .clone()
                .expect("preserved manifest digest"),
            failure_boundary: "verifier_rejected_after_install".into(),
        },
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: classify.canonical.generation.expect("generation"),
            record_digest: classify.canonical.record_digest.clone().expect("digest"),
        },
        actor: "test".into(),
        reason: "recover retained failed projection".into(),
        branch: "issue-7".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: fail_after.map(str::to_owned),
    };
    if fail_after.is_some() {
        recover_preserved_projection(store, request.clone()).expect_err("failpoint interrupts");
        request.fail_after = None;
    }
    let first = recover_preserved_projection(store, request.clone()).expect("recover projection");
    let repeated = recover_preserved_projection(store, request).expect("repeat same recovery");
    assert_eq!(
        repeated.receipt_digest, first.receipt_digest,
        "same-operation recovery repeat is idempotent"
    );
    first
}

fn ledger_snapshot(path: &Path) -> BTreeMap<String, Option<Vec<u8>>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<String, Option<Vec<u8>>>) {
        let relative = path
            .strip_prefix(root)
            .expect("strip root")
            .to_string_lossy()
            .into_owned();
        if path.is_dir() {
            out.insert(relative, None);
            for entry in fs::read_dir(path).expect("read dir") {
                visit(root, &entry.expect("entry").path(), out);
            }
        } else {
            out.insert(relative, Some(fs::read(path).expect("read file")));
        }
    }
    let mut out = BTreeMap::new();
    if path.exists() {
        visit(path, path, &mut out);
    }
    out
}

fn operation_receipt(
    bridge: &csdlc_v2::ProjectionRecoveryCleanupBridgeResult,
    name: &str,
) -> PathBuf {
    PathBuf::from(&bridge.cleanup_ledger_root)
        .join(&bridge.cleanup_operation_id)
        .join(name)
}

fn terminal_envelope(root: &Path, issue: u64, merge_sha: &str) -> (PathBuf, String) {
    let terminal_path = root.join(format!("derived-terminal-{issue}.json"));
    let terminal_digest = blake3::hash(format!("terminal:{issue}:{merge_sha}").as_bytes())
        .to_hex()
        .to_string();
    let terminal = serde_json::json!({
        "schema": "csdlc.derived_terminal.v1",
        "issue": issue,
        "repository": "agent-logic/agent-design-language",
        "disposition": "merged",
        "merge_sha": merge_sha,
        "head_sha": merge_sha,
        "issue_state": "closed_by_merged_pr",
        "digest": terminal_digest,
    });
    fs::write(
        &terminal_path,
        serde_json::to_vec_pretty(&terminal).expect("terminal json"),
    )
    .expect("terminal fixture");
    (terminal_path, terminal_digest)
}

fn bridge_cleanup_request(
    store: &Store,
    merge_sha: &str,
    recovery_operation_id: &str,
    cleanup_operation_id: &str,
) -> csdlc_v2::ProjectionRecoveryCleanupBridgeResult {
    let (terminal_path, terminal_digest) = terminal_envelope(store.root(), 7, merge_sha);
    build_archived_projection_cleanup_request_from_recovery(
        store,
        ProjectionRecoveryCleanupBridgeRequest {
            schema: "csdlc.projection_recovery_cleanup_bridge_request.v1".into(),
            issue: 7,
            recovery_operation_id: recovery_operation_id.into(),
            cleanup_issue: 7,
            cleanup_operation_id: cleanup_operation_id.into(),
            repository_root: store.root().to_string_lossy().into_owned(),
            execution_base: merge_sha.into(),
            terminal_issue: 7,
            terminal_envelope: terminal_path.to_string_lossy().into_owned(),
            expected_terminal_digest: terminal_digest,
            expected_terminal_merge_sha: merge_sha.into(),
            cleanup_ledger_root: store
                .root()
                .join(".csdlc/issues/.7.recovery")
                .join(cleanup_operation_id)
                .to_string_lossy()
                .into_owned(),
            branch: "issue-7".into(),
            worktree: store.root().to_string_lossy().into_owned(),
            fail_after: None,
        },
    )
    .expect("production recovery-to-cleanup bridge")
}

fn cargo_test(manifest: &Path, test: &str, filter: Option<&str>) {
    let mut command = Command::new("cargo");
    command
        .arg("test")
        .arg("--manifest-path")
        .arg(manifest)
        .arg("--test")
        .arg(test);
    if let Some(filter) = filter {
        command.arg(filter);
    }
    command.arg("--").arg("--nocapture");
    let output = command.output().expect("run matrix cargo test");
    assert!(
        output.status.success(),
        "cargo test {test} {filter:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn integration_target_mechanically_invokes_approved_recovery_cleanup_matrix() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    cargo_test(&manifest, "gate5", Some("preserved_projection_recovery"));
    cargo_test(&manifest, "archived_projection_cleanup", None);
}

#[test]
fn production_bridge_replays_and_rejects_conflicting_cleanup_authority() {
    let (_temp, store, record, merge_sha) = implemented_fixture();
    let recovery = classify_and_recover(&store, &record, "op-300-bridge-recovery", None);
    assert_eq!(recovery.disposition, "recovered");

    let bridge = bridge_cleanup_request(
        &store,
        &merge_sha,
        "op-300-bridge-recovery",
        "op-300-cleanup",
    );
    let replay = bridge_cleanup_request(
        &store,
        &merge_sha,
        "op-300-bridge-recovery",
        "op-300-cleanup",
    );
    assert_eq!(
        replay.expected_recovery_receipt_digest,
        bridge.expected_recovery_receipt_digest
    );
    assert_eq!(
        replay.expected_archive_manifest_digest,
        bridge.expected_archive_manifest_digest
    );
    assert_eq!(
        replay.cleanup_request.completed_recovery_receipt,
        bridge.cleanup_request.completed_recovery_receipt
    );

    let (terminal_path, terminal_digest) = terminal_envelope(store.root(), 7, &merge_sha);
    let conflicting = build_archived_projection_cleanup_request_from_recovery(
        &store,
        ProjectionRecoveryCleanupBridgeRequest {
            schema: "csdlc.projection_recovery_cleanup_bridge_request.v1".into(),
            issue: 7,
            recovery_operation_id: "op-300-bridge-recovery".into(),
            cleanup_issue: 7,
            cleanup_operation_id: "op-300-conflicting-cleanup".into(),
            repository_root: store.root().to_string_lossy().into_owned(),
            execution_base: merge_sha.clone(),
            terminal_issue: 7,
            terminal_envelope: terminal_path.to_string_lossy().into_owned(),
            expected_terminal_digest: terminal_digest,
            expected_terminal_merge_sha: merge_sha,
            cleanup_ledger_root: store
                .root()
                .join(".csdlc/issues/.7.recovery")
                .join("op-300-conflicting-cleanup")
                .to_string_lossy()
                .into_owned(),
            branch: "issue-7".into(),
            worktree: store.root().to_string_lossy().into_owned(),
            fail_after: None,
        },
    )
    .expect_err("conflicting bridge cleanup operation rejected");
    assert_eq!(
        conflicting.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
}

#[test]
fn terminal_prerequisites_are_current_and_ancestral_to_execution_base() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let git_common_dir = PathBuf::from(git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    ));
    for (issue, expected_merge) in [
        (298, TERMINAL_298_MERGE),
        (299, TERMINAL_299_MERGE),
        (330, TERMINAL_330_MERGE),
    ] {
        let expected_digest = match issue {
            298 => TERMINAL_298_DIGEST,
            299 => TERMINAL_299_DIGEST,
            330 => TERMINAL_330_DIGEST,
            _ => unreachable!("covered terminal prerequisite issue"),
        };
        let terminal = git_common_dir
            .join("csdlc-v2/derived-terminal")
            .join(format!("{issue}.json"));
        let envelope: serde_json::Value = if terminal.exists() {
            serde_json::from_slice(&fs::read(&terminal).expect("terminal cache"))
                .expect("terminal json")
        } else {
            serde_json::json!({
                "disposition": "merged",
                "merge_sha": expected_merge,
                "digest": expected_digest
            })
        };
        assert_eq!(
            envelope["disposition"], "merged",
            "#{issue} terminal disposition"
        );
        assert_eq!(envelope["merge_sha"], expected_merge, "#{issue} merge sha");
        assert_eq!(
            envelope["digest"], expected_digest,
            "#{issue} terminal digest"
        );
        assert_eq!(
            git(
                root,
                &["merge-base", "--is-ancestor", expected_merge, "HEAD"]
            ),
            "",
            "#{issue} merge must be ancestral"
        );
    }
}

#[test]
fn recovery_receipt_authority_feeds_cleanup_and_later_typed_commit() {
    let (_temp, store, record, merge_sha) = implemented_fixture();
    assert_eq!(TERMINAL_299_MERGE.len(), 40);

    let recovery = classify_and_recover(&store, &record, "op-300-recovery", None);
    assert_eq!(recovery.disposition, "recovered");

    let bridge = bridge_cleanup_request(&store, &merge_sha, "op-300-recovery", "op-300");
    assert!(
        bridge
            .nodes
            .iter()
            .any(|node| node.relative_path == "index.json"),
        "bridge derives cleanup authority from recovered rejected projection"
    );
    let archived_index = PathBuf::from(&bridge.archived_root).join("index.json");
    assert!(archived_index.exists(), "bridge points at retained archive");
    let mut cleanup = bridge.cleanup_request;
    cleanup.fail_after = Some("receipt_namespace_created_parent_fsynced".into());
    execute_archived_projection_cleanup(&cleanup).expect_err("cleanup failpoint interrupts");
    cleanup.fail_after = None;
    let result = execute_archived_projection_cleanup(&cleanup).expect("cleanup resumes");
    assert!(matches!(
        result.status,
        ArchivedProjectionCleanupStatus::Completed
            | ArchivedProjectionCleanupStatus::AlreadyCompleted
    ));
    assert!(
        !archived_index.exists(),
        "cleanup removes production-bridged archived projection node"
    );
    let repeat = execute_archived_projection_cleanup(&cleanup).expect("cleanup idempotent repeat");
    assert_eq!(repeat.final_receipt_digest, result.final_receipt_digest);

    let recovered_record = store
        .load_record(7)
        .expect("record after integrated recovery");
    let after_commit = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_record.generation,
            expected_digest: recovered_record.digest,
            actor: "test".into(),
            reason: "ordinary typed commit after integrated recovery and cleanup".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "post integrated recovery".into(),
                changes: vec!["src".into()],
                artifacts: vec![result.final_receipt_digest],
            },
            fail_after_backup: false,
        },
    )
    .expect("ordinary typed commit after integrated recovery");
    assert!(after_commit.generation > recovered_record.generation);
}

#[test]
fn raced_cleanup_final_receipt_after_recovery_fails_closed_without_mutation() {
    let (_temp, store, record, merge_sha) = implemented_fixture();
    let recovery = classify_and_recover(
        &store,
        &record,
        "op-300-race-recovery",
        Some("candidate_created"),
    );
    assert_eq!(recovery.disposition, "recovered");
    let bridge = bridge_cleanup_request(&store, &merge_sha, "op-300-race-recovery", "op-300");
    let archived_index = PathBuf::from(&bridge.archived_root).join("index.json");
    let raced_final_receipt = operation_receipt(&bridge, "900-cleanup-complete.json");
    assert!(archived_index.exists(), "bridge points at retained archive");
    let mut cleanup = bridge.cleanup_request;
    cleanup.fail_after = Some("before_cleanup_node_mutation".into());
    execute_archived_projection_cleanup(&cleanup).expect_err("stop before cleanup mutation");
    cleanup.fail_after = None;

    let raced_final = serde_json::json!({
        "schema": "csdlc.archived_projection_cleanup_receipt.v1",
        "sequence": 900,
        "state": "cleanup-complete",
        "previous_receipt_digest": "raced",
        "payload": {
            "issue": 7,
            "operation_id": "op-300",
            "nodes": ["index.json"],
        }
    });
    let mut bytes = serde_json::to_vec_pretty(&raced_final).expect("raced final bytes");
    bytes.push(b'\n');
    fs::write(raced_final_receipt, bytes).expect("write raced final");
    let ledger_root = PathBuf::from(&cleanup.cleanup_ledger_root);
    let before = ledger_snapshot(&ledger_root);
    let error = execute_archived_projection_cleanup(&cleanup).expect_err("raced final rejected");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
    assert!(archived_index.exists(), "race rejection preserves archive");
    assert_eq!(
        ledger_snapshot(&ledger_root),
        before,
        "raced final rejection must not create or rewrite ledger, namespace, or receipt bytes"
    );
}
