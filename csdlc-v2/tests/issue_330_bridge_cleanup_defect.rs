use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use csdlc_v2::cards::{PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    bind_issue, build_archived_projection_cleanup_request_from_recovery,
    classify_preserved_projection, edit_issue, execute_archived_projection_cleanup,
    initialize_native_json, recover_preserved_projection, ArchivedProjectionCleanupStatus,
    BindRequest, BootstrapRequest, CardKind, EditRequest, FailedOperationLineage, InitialCardInput,
    LifecyclePhase, PlanningProfile, ProjectionCasAnchor, ProjectionClassifyRequest,
    ProjectionRecoverRequest, ProjectionRecoveryCleanupBridgeRequest, SemanticOperation, Store,
};

fn install_native_authority(root: &Path) {
    let registry = root.join("docs/templates/prompts/current.json");
    let manifest = root.join("csdlc-v2/operator/native-card-shape.json");
    fs::create_dir_all(registry.parent().unwrap()).expect("registry parent");
    fs::create_dir_all(manifest.parent().unwrap()).expect("manifest parent");
    fs::write(
        registry,
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry");
    fs::write(
        manifest,
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("manifest");
}

fn git(root: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .expect("git");
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_out(root: &Path, args: &[&str]) -> String {
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

fn initial_input() -> InitialCardInput {
    InitialCardInput {
        title: "bridge cleanup defect fixture".into(),
        slug: "bridge-cleanup-defect-fixture".into(),
        version: "v0.92".into(),
        goal: "prove bridge cleanup defect".into(),
        required_outcome: "bridge cleanup is coherent".into(),
        declared_scope: vec!["src".into()],
        authority_boundary: vec!["test fixture only".into()],
        operator_constraints: vec!["none".into()],
        task_boundary: "fixture only".into(),
        deliverables: vec!["src/validate.sh".into()],
        acceptance_criteria: vec!["AC-1".into()],
        dependencies: vec!["none".into()],
        repo_inputs: vec!["src".into()],
        non_goals: vec!["publish".into()],
        plan_summary: "bind and implement fixture".into(),
        steps: vec![PlanStep {
            id: "S1".into(),
            action: "fixture".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: StepStatus::Pending,
        }],
        affected_areas: vec!["src".into(), "src/validate.sh".into()],
        invariants: vec!["fail closed".into()],
        risks: vec!["none".into()],
        planning_profile: PlanningProfile::Small,
        stop_conditions: vec!["stale".into()],
        validation_lanes: vec![ValidationLane {
            lane: "fixture".into(),
            proof_role: "fixture".into(),
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
        review_prompts: vec!["fixture".into()],
        review_scope: "fixture".into(),
    }
}

fn implemented_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("temp");
    fs::create_dir_all(temp.path().join("docs")).expect("docs");
    fs::write(temp.path().join("docs/design.md"), "# design\n").expect("design");
    fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .expect("diagram");
    fs::create_dir_all(temp.path().join("src")).expect("src");
    fs::write(temp.path().join("src/lib.rs"), "// fixture\n").expect("src");
    fs::write(
        temp.path().join("src/validate.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\ntest -f src/lib.rs\n",
    )
    .expect("validator");
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
            initial: initial_input(),
        })
        .unwrap(),
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
            reason: "ready".into(),
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
                reason: "implemented".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .expect("transition");
    }
    (temp, store, record)
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir(destination).expect("copy root");
    for entry in fs::read_dir(source).expect("read source") {
        let entry = entry.expect("entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("entry type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy file");
        }
    }
}

fn recovery_request(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    classify: &csdlc_v2::ProjectionClassification,
    operation_id: &str,
) -> ProjectionRecoverRequest {
    ProjectionRecoverRequest {
        issue: 7,
        operation_id: operation_id.into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest.clone(),
            rejected_manifest_digest: classify.preserved.manifest_digest.clone().unwrap(),
            failure_boundary: "verifier_rejected_after_install".into(),
        },
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: classify.canonical.generation.unwrap(),
            record_digest: classify.canonical.record_digest.clone().unwrap(),
        },
        actor: "test".into(),
        reason: "recover receipt fixture".into(),
        branch: "issue-7".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: None,
    }
}

fn completed_recovery_attempt(store: &Store, record: &csdlc_v2::IssueRecord, operation_id: &str) {
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
            reason: format!("{operation_id} fixture"),
        },
    )
    .expect("classify");
    recover_preserved_projection(
        store,
        recovery_request(store, record, &classify, operation_id),
    )
    .expect("recover");
}

fn terminal_envelope(root: &Path, issue: u64, merge_sha: &str) -> (PathBuf, String) {
    let path = root.join(format!("derived-terminal-{issue}.json"));
    let mut value = serde_json::json!({
        "schema": "csdlc.derived_terminal.v1",
        "issue": issue,
        "repository": "example/repo",
        "initialization_digest": "init",
        "canonical_generation": 70,
        "canonical_digest": "canonical",
        "pull_request": 305,
        "disposition": "merged",
        "head_sha": merge_sha,
        "merge_sha": merge_sha,
        "issue_state": "closed_by_merged_pr",
        "pr_state": "closed",
        "approved_reason": null,
        "observed_unix_seconds": 1,
        "mutable_fresh_until_unix_seconds": null,
        "source": "test",
        "digest": ""
    });
    let digest = blake3::hash(&serde_json::to_vec(&value).expect("terminal vec"))
        .to_hex()
        .to_string();
    value["digest"] = serde_json::Value::String(digest.clone());
    fs::write(
        &path,
        serde_json::to_vec_pretty(&value).expect("terminal json"),
    )
    .expect("terminal");
    (path, digest)
}

fn bridge_cleanup_request(
    store: &Store,
    recovery_operation_id: &str,
    cleanup_operation_id: &str,
) -> csdlc_v2::ProjectionRecoveryCleanupBridgeResult {
    let merge_sha = git_out(store.root(), &["rev-parse", "HEAD"]);
    let (terminal_path, terminal_digest) = terminal_envelope(store.root(), 7, &merge_sha);
    let cleanup_ledger = store
        .root()
        .join(format!(".csdlc/issues/.7.recovery/{cleanup_operation_id}"));
    build_archived_projection_cleanup_request_from_recovery(
        store,
        ProjectionRecoveryCleanupBridgeRequest {
            schema: "csdlc.projection_recovery_cleanup_bridge_request.v1".into(),
            issue: 7,
            recovery_operation_id: recovery_operation_id.into(),
            cleanup_issue: 7,
            cleanup_operation_id: cleanup_operation_id.into(),
            repository_root: store.root().to_string_lossy().into_owned(),
            execution_base: merge_sha.clone(),
            terminal_issue: 7,
            terminal_envelope: terminal_path.to_string_lossy().into_owned(),
            expected_terminal_digest: terminal_digest,
            expected_terminal_merge_sha: merge_sha,
            cleanup_ledger_root: cleanup_ledger.to_string_lossy().into_owned(),
            branch: "issue-7".into(),
            worktree: store.root().to_string_lossy().into_owned(),
            fail_after: None,
        },
    )
    .expect("production bridge")
}

fn ledger_snapshot(path: &Path) -> BTreeMap<String, Option<Vec<u8>>> {
    fn visit(root: &Path, path: &Path, out: &mut BTreeMap<String, Option<Vec<u8>>>) {
        let relative = path
            .strip_prefix(root)
            .expect("strip")
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

#[test]
fn bridge_cleanup_does_not_poison_later_recovery_validation() {
    let (_temp, store, record) = implemented_fixture();
    completed_recovery_attempt(&store, &record, "op-330-recovery");
    let bridge = bridge_cleanup_request(&store, "op-330-recovery", "op-330-cleanup");
    let archived_index = PathBuf::from(&bridge.archived_root).join("index.json");
    assert!(archived_index.exists());

    let cleanup = execute_archived_projection_cleanup(&bridge.cleanup_request).expect("cleanup");
    assert_eq!(cleanup.status, ArchivedProjectionCleanupStatus::Completed);
    assert!(!archived_index.exists());

    let recovered_record = store.load_record(7).expect("record after recovery");
    let after = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_record.generation,
            expected_digest: recovered_record.digest,
            actor: "test".into(),
            reason: "ordinary commit after exact-authority cleanup".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "post cleanup commit".into(),
                changes: vec!["src".into()],
                artifacts: vec![cleanup.final_receipt_digest],
            },
            fail_after_backup: false,
        },
    )
    .expect("ordinary commit after exact-authority cleanup");
    assert!(after.generation > recovered_record.generation);
}

#[test]
fn forged_cleanup_final_chain_does_not_authorize_recovery_skip() {
    let (_temp, store, record) = implemented_fixture();
    completed_recovery_attempt(&store, &record, "op-330-forged-chain-recovery");
    let bridge = bridge_cleanup_request(
        &store,
        "op-330-forged-chain-recovery",
        "op-330-forged-chain-cleanup",
    );
    let cleanup = execute_archived_projection_cleanup(&bridge.cleanup_request).expect("cleanup");
    assert_eq!(cleanup.status, ArchivedProjectionCleanupStatus::Completed);

    let final_receipt = PathBuf::from(&bridge.cleanup_ledger_root)
        .join(&bridge.cleanup_operation_id)
        .join("900-cleanup-complete.json");
    let mut forged: serde_json::Value =
        serde_json::from_slice(&fs::read(&final_receipt).expect("final receipt"))
            .expect("final json");
    forged["previous_receipt_digest"] = serde_json::Value::String("forged-predecessor".into());
    let mut bytes = serde_json::to_vec_pretty(&forged).expect("forged bytes");
    bytes.push(b'\n');
    fs::write(&final_receipt, bytes).expect("write forged final");

    let recovered_record = store.load_record(7).expect("record after recovery");
    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_record.generation,
            expected_digest: recovered_record.digest,
            actor: "test".into(),
            reason: "ordinary commit after forged cleanup chain".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "post forged cleanup commit".into(),
                changes: vec!["src".into()],
                artifacts: vec![cleanup.final_receipt_digest],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("forged cleanup chain must not authorize recovery skip");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
    assert!(
        error.message.contains("cleanup") || error.message.contains("recovery"),
        "unexpected error: {}",
        error.message
    );
}

#[test]
fn cleanup_private_namespace_residue_does_not_authorize_recovery_skip() {
    let (_temp, store, record) = implemented_fixture();
    completed_recovery_attempt(&store, &record, "op-330-residue-recovery");
    let bridge =
        bridge_cleanup_request(&store, "op-330-residue-recovery", "op-330-residue-cleanup");
    let cleanup = execute_archived_projection_cleanup(&bridge.cleanup_request).expect("cleanup");
    assert_eq!(cleanup.status, ArchivedProjectionCleanupStatus::Completed);

    let recovery_root = store.root().join(".csdlc/issues/.7.recovery");
    let operation_root =
        PathBuf::from(&bridge.cleanup_ledger_root).join(&bridge.cleanup_operation_id);
    let residue = operation_root.join("private-delete/residue");
    fs::write(&residue, b"unexpected retained namespace bytes").expect("residue");

    let before_recovery = ledger_snapshot(&recovery_root);
    let before_issue = ledger_snapshot(&store.issue_dir(7));
    let recovered_record = store.load_record(7).expect("record after recovery");
    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_record.generation,
            expected_digest: recovered_record.digest,
            actor: "test".into(),
            reason: "ordinary commit after residue cleanup namespace".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "post residue cleanup commit".into(),
                changes: vec!["src".into()],
                artifacts: vec![cleanup.final_receipt_digest],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("residue in cleanup private namespace must fail closed");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
    assert_eq!(
        ledger_snapshot(&recovery_root),
        before_recovery,
        "rejection must not create or rewrite recovery ledger bytes"
    );
    assert_eq!(
        ledger_snapshot(&store.issue_dir(7)),
        before_issue,
        "rejection must not mutate issue projection bytes"
    );
}

#[test]
fn raced_final_receipt_before_prefinal_validation_is_zero_mutation() {
    let (_temp, store, record) = implemented_fixture();
    completed_recovery_attempt(&store, &record, "op-330-race-recovery");
    let bridge = bridge_cleanup_request(&store, "op-330-race-recovery", "op-330-race-cleanup");
    let archived_index = PathBuf::from(&bridge.archived_root).join("index.json");
    assert!(archived_index.exists());

    let mut cleanup = bridge.cleanup_request;
    cleanup.fail_after = Some("before_cleanup_node_mutation".into());
    execute_archived_projection_cleanup(&cleanup).expect_err("stop before prefinal validation");
    cleanup.fail_after = None;

    let ledger_root = PathBuf::from(&cleanup.cleanup_ledger_root);
    let before_ledger = ledger_snapshot(&ledger_root);
    let before_archive = ledger_snapshot(Path::new(&cleanup.archived_root));
    let operation_root = ledger_root.join(&cleanup.operation_id);
    let raced_final = serde_json::json!({
        "schema": "csdlc.archived_projection_cleanup_receipt.v1",
        "sequence": 900,
        "state": "cleanup-complete",
        "previous_receipt_digest": "raced",
        "payload": {
            "issue": cleanup.issue,
            "operation_id": cleanup.operation_id,
            "nodes": cleanup.nodes.iter().map(|node| node.relative_path.as_str()).collect::<Vec<_>>(),
        }
    });
    let mut bytes = serde_json::to_vec_pretty(&raced_final).expect("raced final");
    bytes.push(b'\n');
    fs::write(operation_root.join("900-cleanup-complete.json"), bytes).expect("raced final");
    let before_with_race = ledger_snapshot(&ledger_root);

    let error = execute_archived_projection_cleanup(&cleanup).expect_err("raced final rejected");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
    assert_eq!(
        ledger_snapshot(&ledger_root),
        before_with_race,
        "rejection must not create or rewrite ledger, namespace, or receipts"
    );
    assert_eq!(
        ledger_snapshot(Path::new(&cleanup.archived_root)),
        before_archive,
        "rejection must not mutate archived nodes"
    );
    assert_ne!(
        before_ledger, before_with_race,
        "test must inject a raced final receipt"
    );
    assert!(
        archived_index.exists(),
        "archive remains intact on rejection"
    );
}
