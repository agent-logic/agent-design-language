use csdlc_v2::cards::{digest, FindingDisposition, FindingSeverity, PublicationState};
use csdlc_v2::model::TransitionEvent;
use csdlc_v2::{
    approve_design, assign_review, bind_issue,
    build_archived_projection_cleanup_request_from_recovery, classify_preserved_projection,
    edit_issue, evaluate_publication_review, evaluate_publication_review_in_repo,
    execute_archived_projection_cleanup, record_review, ArchivedProjectionCleanupStatus,
    BindRequest, BootstrapRequest, CardKind, CleanupNodeIdentity, CleanupNodeType, EditRequest,
    ErrorCode, FailedOperationLineage, InitialCardInput, LifecyclePhase, NonSubstantiveProof,
    PlanningProfile, ProjectionCasAnchor, ProjectionClassifyRequest, ProjectionRecoverRequest,
    ProjectionRecoveryCleanupBridgeRequest, RecoverDesignReviewRequest, ReviewAssignmentRequest,
    ReviewEvidence, ReviewFindingEvidence, ReviewRecordRequest, ReviewRecoveryRequest,
    SemanticOperation, Store,
};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

fn copy_tree(source: &std::path::Path, destination: &std::path::Path) {
    std::fs::create_dir(destination).expect("create copied projection root");
    for entry in std::fs::read_dir(source).expect("read copied projection") {
        let entry = entry.expect("projection entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("entry type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).expect("copy projection file");
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

fn receipt_path(attempt: &std::path::Path, seq: u32, state: &str) -> std::path::PathBuf {
    attempt.join(format!("{seq:03}-{state}.json"))
}

fn rewrite_receipt_payload_and_rechain(
    attempt: &std::path::Path,
    seq: u32,
    state: &str,
    payload: serde_json::Value,
) {
    let path = receipt_path(attempt, seq, state);
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("receipt")).expect("receipt json");
    envelope["payload"] = payload;
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("receipt bytes");
    bytes.push(b'\n');
    std::fs::write(&path, bytes).expect("write forged receipt");
    let states = [
        "prepared",
        "archive-intent",
        "rejected-archived",
        "candidate-plan",
        "candidate-created",
        "candidate-verified",
        "install-intent",
        "canonical-installed",
        "displace-intent",
        "prior-displaced",
        "canonical-verified",
        "recovery-complete-intent",
        "recovered",
    ];
    for next in (seq + 1)..=13 {
        let prior = receipt_path(attempt, next - 1, states[next as usize - 2]);
        let next_path = receipt_path(attempt, next, states[next as usize - 1]);
        let digest = blake3::hash(&std::fs::read(prior).expect("prior receipt"))
            .to_hex()
            .to_string();
        let mut next_envelope: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&next_path).expect("next receipt"))
                .expect("next receipt json");
        next_envelope["previous_receipt_digest"] = serde_json::Value::String(digest);
        let mut next_bytes = serde_json::to_vec_pretty(&next_envelope).expect("next bytes");
        next_bytes.push(b'\n');
        std::fs::write(next_path, next_bytes).expect("write rechained receipt");
    }
}

fn add_receipt_envelope_field_and_rechain(
    attempt: &std::path::Path,
    seq: u32,
    state: &str,
    key: &str,
    value: serde_json::Value,
) {
    let path = receipt_path(attempt, seq, state);
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("receipt")).expect("receipt json");
    envelope[key] = value;
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("receipt bytes");
    bytes.push(b'\n');
    std::fs::write(&path, bytes).expect("write forged receipt envelope");
    let states = [
        "prepared",
        "archive-intent",
        "rejected-archived",
        "candidate-plan",
        "candidate-created",
        "candidate-verified",
        "install-intent",
        "canonical-installed",
        "displace-intent",
        "prior-displaced",
        "canonical-verified",
        "recovery-complete-intent",
        "recovered",
    ];
    for next in (seq + 1)..=13 {
        let prior = receipt_path(attempt, next - 1, states[next as usize - 2]);
        let next_path = receipt_path(attempt, next, states[next as usize - 1]);
        let digest = blake3::hash(&std::fs::read(prior).expect("prior receipt"))
            .to_hex()
            .to_string();
        let mut next_envelope: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&next_path).expect("next receipt"))
                .expect("next receipt json");
        next_envelope["previous_receipt_digest"] = serde_json::Value::String(digest);
        let mut next_bytes = serde_json::to_vec_pretty(&next_envelope).expect("next bytes");
        next_bytes.push(b'\n');
        std::fs::write(next_path, next_bytes).expect("write rechained receipt");
    }
}

fn rewrite_single_receipt_payload(
    attempt: &std::path::Path,
    seq: u32,
    state: &str,
    payload: serde_json::Value,
) {
    let path = receipt_path(attempt, seq, state);
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("receipt")).expect("receipt json");
    envelope["payload"] = payload;
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("receipt bytes");
    bytes.push(b'\n');
    std::fs::write(path, bytes).expect("write forged receipt");
}

fn append_extra_terminal_receipt(attempt: &std::path::Path) {
    let previous = receipt_path(attempt, 13, "recovered");
    let previous_digest = blake3::hash(&std::fs::read(previous).expect("terminal receipt"))
        .to_hex()
        .to_string();
    let envelope = serde_json::json!({
        "schema":"csdlc.projection_recovery_receipt.v1",
        "sequence":14,
        "state":"post-terminal",
        "previous_receipt_digest":previous_digest,
        "payload":{"forged":true}
    });
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("extra receipt");
    bytes.push(b'\n');
    std::fs::write(receipt_path(attempt, 14, "post-terminal"), bytes)
        .expect("write extra terminal receipt");
}

#[cfg(unix)]
fn append_extra_terminal_symlink(attempt: &std::path::Path) {
    std::os::unix::fs::symlink(
        receipt_path(attempt, 13, "recovered"),
        receipt_path(attempt, 14, "post-terminal"),
    )
    .expect("create extra terminal symlink");
}

fn append_extra_terminal_directory(attempt: &std::path::Path) {
    std::fs::create_dir(receipt_path(attempt, 14, "post-terminal"))
        .expect("create extra terminal directory");
}

#[cfg(unix)]
fn append_extra_terminal_fifo(attempt: &std::path::Path) {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let path = receipt_path(attempt, 14, "post-terminal");
    let path = CString::new(path.as_os_str().as_bytes()).expect("FIFO path has no NUL");
    // SAFETY: `path` is a live NUL-terminated pathname and `mkfifo` does not retain it.
    let result = unsafe { libc::mkfifo(path.as_ptr(), 0o600) };
    assert_eq!(
        result,
        0,
        "create extra terminal FIFO: {}",
        std::io::Error::last_os_error()
    );
}

fn completed_recovery_attempt(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    operation_id: &str,
) -> (ProjectionRecoverRequest, std::path::PathBuf) {
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
    let request = recovery_request(store, record, &classify, operation_id);
    csdlc_v2::recover_preserved_projection(store, request.clone()).expect("recover");
    let attempt = store
        .root()
        .join(format!(".csdlc/issues/.7.recovery/{operation_id}"));
    (request, attempt)
}

fn terminal_envelope(
    root: &std::path::Path,
    issue: u64,
    merge_sha: &str,
) -> (std::path::PathBuf, String) {
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
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&value).expect("terminal json"),
    )
    .expect("write terminal");
    (path, digest)
}

fn cleanup_identity(path: &std::path::Path) -> CleanupNodeIdentity {
    let metadata = std::fs::symlink_metadata(path).expect("metadata");
    let node_type = if metadata.is_file() {
        CleanupNodeType::RegularFile
    } else if metadata.is_dir() {
        CleanupNodeType::Directory
    } else {
        panic!("unsupported node type");
    };
    CleanupNodeIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
        links: metadata.nlink(),
        uid: metadata.uid(),
        gid: metadata.gid(),
        mode: metadata.mode(),
        node_type,
    }
}

#[test]
fn recovery_bridge_emits_cleanup_authority_consumed_by_cleanup() {
    let (_temp, store, record) = implemented_fixture();
    let (_recovery_request, attempt) =
        completed_recovery_attempt(&store, &record, "bridge-cleanup");
    let merge_sha = git_out(store.root(), &["rev-parse", "HEAD"]);
    let (terminal_path, terminal_digest) = terminal_envelope(store.root(), 7, &merge_sha);
    let cleanup_ledger = store
        .root()
        .join(".csdlc/issues/.7.recovery/bridge-cleanup-cleanup");

    let request = ProjectionRecoveryCleanupBridgeRequest {
        schema: "csdlc.projection_recovery_cleanup_bridge_request.v1".into(),
        issue: 7,
        recovery_operation_id: "bridge-cleanup".into(),
        cleanup_issue: 7,
        cleanup_operation_id: "bridge-cleanup-cleanup".into(),
        repository_root: store.root().to_string_lossy().into_owned(),
        execution_base: merge_sha.clone(),
        terminal_issue: 7,
        terminal_envelope: terminal_path.to_string_lossy().into_owned(),
        expected_terminal_digest: terminal_digest.clone(),
        expected_terminal_merge_sha: merge_sha,
        cleanup_ledger_root: cleanup_ledger.to_string_lossy().into_owned(),
        branch: "issue-7".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: None,
    };

    let bridge = build_archived_projection_cleanup_request_from_recovery(&store, request.clone())
        .expect("build cleanup authority from recovery");
    let replay = build_archived_projection_cleanup_request_from_recovery(&store, request.clone())
        .expect("same cleanup bridge operation replays idempotently");
    assert_eq!(
        replay.expected_recovery_receipt_digest,
        bridge.expected_recovery_receipt_digest
    );
    assert_eq!(
        replay.expected_archive_manifest_digest,
        bridge.expected_archive_manifest_digest
    );
    assert_eq!(
        replay.completed_recovery_receipt,
        bridge.completed_recovery_receipt
    );
    assert_eq!(
        replay.canonical_archive_manifest,
        bridge.canonical_archive_manifest
    );

    let mut conflicting = request.clone();
    conflicting.cleanup_operation_id = "bridge-cleanup-other".into();
    assert_eq!(
        build_archived_projection_cleanup_request_from_recovery(&store, conflicting)
            .expect_err("different cleanup operation is not co-authorized")
            .code,
        ErrorCode::ReconciliationRequired
    );

    assert!(bridge
        .completed_recovery_receipt
        .starts_with(&attempt.to_string_lossy().to_string()));
    assert!(bridge
        .canonical_archive_manifest
        .starts_with(&attempt.to_string_lossy().to_string()));
    assert_eq!(
        bridge.cleanup_request.completed_recovery_receipt,
        bridge.completed_recovery_receipt
    );
    assert_eq!(
        bridge.cleanup_request.expected_archive_manifest_digest,
        bridge.expected_archive_manifest_digest
    );
    assert!(!bridge.nodes.iter().any(|node| node.relative_path == "."));
    assert!(bridge
        .nodes
        .iter()
        .any(|node| node.relative_path == "index.json"));
    for node in &bridge.nodes {
        let observed = cleanup_identity(
            &std::path::PathBuf::from(&bridge.archived_root).join(&node.relative_path),
        );
        assert_eq!(observed, node.identity, "{}", node.relative_path);
    }

    let cleanup = execute_archived_projection_cleanup(&bridge.cleanup_request)
        .expect("cleanup consumes production bridge authority");
    assert_eq!(cleanup.status, ArchivedProjectionCleanupStatus::Completed);
    assert_eq!(cleanup.terminal_digest, terminal_digest);
    assert!(!store
        .root()
        .join(".csdlc/issues/.7.recovery/bridge-cleanup/rejected/index.json")
        .exists());
}

fn node_receipt_path(
    attempt: &std::path::Path,
    ordinal: usize,
    seq: u32,
    state: &str,
) -> std::path::PathBuf {
    receipt_path(&attempt.join(format!("node-{ordinal:03}")), seq, state)
}

fn append_extra_node_receipt(attempt: &std::path::Path, ordinal: usize) {
    let previous = node_receipt_path(attempt, ordinal, 10, "node-created");
    let previous_digest = blake3::hash(&std::fs::read(previous).expect("terminal node receipt"))
        .to_hex()
        .to_string();
    let envelope = serde_json::json!({
        "schema":"csdlc.projection_recovery_receipt.v1",
        "sequence":11,
        "state":"post-node",
        "previous_receipt_digest":previous_digest,
        "payload":{"forged":true}
    });
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("extra node receipt");
    bytes.push(b'\n');
    std::fs::write(node_receipt_path(attempt, ordinal, 11, "post-node"), bytes)
        .expect("write extra node receipt");
}

#[cfg(unix)]
fn append_extra_node_symlink(attempt: &std::path::Path, ordinal: usize) {
    std::os::unix::fs::symlink(
        node_receipt_path(attempt, ordinal, 10, "node-created"),
        node_receipt_path(attempt, ordinal, 11, "post-node"),
    )
    .expect("create extra node symlink");
}

fn append_extra_node_directory(attempt: &std::path::Path, ordinal: usize) {
    std::fs::create_dir(node_receipt_path(attempt, ordinal, 11, "post-node"))
        .expect("create extra node directory");
}

fn rewrite_node_receipt_payload_and_rechain(
    attempt: &std::path::Path,
    ordinal: usize,
    seq: u32,
    state: &str,
    payload: serde_json::Value,
) {
    let ledger = attempt.join(format!("node-{ordinal:03}"));
    let path = receipt_path(&ledger, seq, state);
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("node receipt"))
            .expect("node receipt json");
    envelope["payload"] = payload;
    let mut bytes = serde_json::to_vec_pretty(&envelope).expect("node receipt bytes");
    bytes.push(b'\n');
    std::fs::write(&path, bytes).expect("write forged node receipt");
    let states = [
        "create-intent",
        "created-identity",
        "write-intent",
        "write-completed",
        "node-fsync-intent",
        "node-fsync-completed",
        "parent-fsync-intent",
        "parent-fsync-completed",
        "publish-intent",
        "node-created",
    ];
    for next in (seq + 1)..=10 {
        let prior = receipt_path(&ledger, next - 1, states[next as usize - 2]);
        let next_path = receipt_path(&ledger, next, states[next as usize - 1]);
        let digest = blake3::hash(&std::fs::read(prior).expect("prior node receipt"))
            .to_hex()
            .to_string();
        let mut next_envelope: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&next_path).expect("next node receipt"))
                .expect("next node receipt json");
        next_envelope["previous_receipt_digest"] = serde_json::Value::String(digest);
        let mut next_bytes = serde_json::to_vec_pretty(&next_envelope).expect("next node bytes");
        next_bytes.push(b'\n');
        std::fs::write(next_path, next_bytes).expect("write rechained node receipt");
    }
}

fn rewrite_observation_index(observation: &mut serde_json::Value, index_bytes: &[u8]) {
    let entries = observation["entries"]
        .as_array_mut()
        .expect("observation entries");
    let index = entries
        .iter_mut()
        .find(|entry| entry["path"] == "index.json")
        .expect("index observation");
    index["size"] = serde_json::json!(index_bytes.len());
    index["digest"] = serde_json::json!(blake3::hash(index_bytes).to_hex().to_string());
    observation["manifest_digest"] = serde_json::json!(blake3::hash(
        &serde_json::to_vec(entries).expect("manifest bytes")
    )
    .to_hex()
    .to_string());
}

#[test]
fn preserved_projection_recovery_archives_builds_installs_and_is_idempotent() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
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
    .expect("classify recoverable projection");
    assert_eq!(classify.disposition, "recoverable");
    let rejected_manifest = classify
        .preserved
        .manifest_digest
        .clone()
        .expect("manifest");
    let worktree = store.root().to_string_lossy().into_owned();
    let request = ProjectionRecoverRequest {
        issue: 7,
        operation_id: "fixture-recovery".into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest,
            rejected_manifest_digest: rejected_manifest,
            failure_boundary: "verifier_rejected_after_install".into(),
        },
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: classify.canonical.generation.expect("generation"),
            record_digest: classify.canonical.record_digest.clone().expect("digest"),
        },
        actor: "test".into(),
        reason: "recover retained failed projection".into(),
        branch: "issue-7".into(),
        worktree,
        fail_after: None,
    };
    let first = csdlc_v2::recover_preserved_projection(&store, request.clone())
        .expect("recover projection");
    let second =
        csdlc_v2::recover_preserved_projection(&store, request).expect("repeat same recovery");
    assert_eq!(first.receipt_digest, second.receipt_digest);
    assert!(store
        .root()
        .join(".csdlc/issues/.7.recovery/fixture-recovery/rejected")
        .is_dir());
    assert!(store
        .root()
        .join(".csdlc/issues/.7.recovery/fixture-recovery/displaced")
        .is_dir());
    assert!(!store.rollback_preserved(7).exists());
    assert_eq!(
        store.load_record(7).expect("recovered record").generation,
        first.canonical_generation
    );
    let recovered_record = store.load_record(7).expect("record for later commit");
    let after_first_commit = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_record.generation,
            expected_digest: recovered_record.digest,
            actor: "test".into(),
            reason: "ordinary commit after recovery".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "post recovery".into(),
                changes: vec!["none".into()],
                artifacts: vec!["none".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("ordinary typed commit after complete recovery");
    edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: after_first_commit.generation,
            expected_digest: after_first_commit.digest,
            actor: "test".into(),
            reason: "second ordinary commit after recovery".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "second post recovery".into(),
                changes: vec!["none".into()],
                artifacts: vec!["none".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("second ordinary typed commit after complete recovery");
}

#[test]
fn preserved_projection_recovery_rejects_lineage_and_replacement_without_mutation() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "classify negative".into(),
        },
    )
    .expect("classification");
    let mut request = ProjectionRecoverRequest {
        issue: 7,
        operation_id: "negative".into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest.clone(),
            rejected_manifest_digest: "wrong".into(),
            failure_boundary: "verifier".into(),
        },
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: record.generation,
            record_digest: record.digest,
        },
        actor: "test".into(),
        reason: "negative".into(),
        branch: "issue-7".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: None,
    };
    assert_eq!(
        csdlc_v2::recover_preserved_projection(&store, request.clone())
            .expect_err("lineage mismatch")
            .code,
        ErrorCode::ReconciliationRequired
    );
    assert!(preserved.is_dir());
    request.failed_operation_lineage.rejected_manifest_digest = classify
        .preserved
        .manifest_digest
        .expect("rejected manifest");
    let replacement = store.root().join("replacement");
    copy_tree(&preserved, &replacement);
    std::fs::remove_dir_all(&preserved).expect("remove classified inode");
    std::fs::rename(&replacement, &preserved).expect("replace after classify");
    assert_eq!(
        csdlc_v2::recover_preserved_projection(&store, request)
            .expect_err("replacement race")
            .code,
        ErrorCode::ReconciliationRequired
    );
    assert!(preserved.is_dir());
}

#[test]
fn preserved_projection_recovery_classifies_hardlink_as_unsafe() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    std::fs::hard_link(preserved.join("index.json"), preserved.join("index.alias"))
        .expect("hardlink alias");
    let classified = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest,
            },
            actor: "test".into(),
            reason: "unsafe alias".into(),
        },
    )
    .expect("classification reports unsafe");
    assert_eq!(classified.disposition, "unsafe");
    assert!(preserved.is_dir());
}

#[test]
fn preserved_projection_recovery_rejects_wrong_topology_and_unsafe_mode() {
    use std::os::unix::fs::PermissionsExt;
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "topology negative".into(),
        },
    )
    .expect("classification");
    let request = ProjectionRecoverRequest {
        issue: 7,
        operation_id: "wrong-topology".into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest,
            rejected_manifest_digest: classify.preserved.manifest_digest.expect("manifest"),
            failure_boundary: "verifier".into(),
        },
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: classify.canonical.generation.expect("generation"),
            record_digest: classify.canonical.record_digest.clone().expect("digest"),
        },
        actor: "test".into(),
        reason: "wrong topology".into(),
        branch: "not-the-bound-branch".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: None,
    };
    assert_eq!(
        csdlc_v2::recover_preserved_projection(&store, request)
            .expect_err("wrong branch")
            .code,
        ErrorCode::UnsafeCheckout
    );
    std::fs::set_permissions(
        preserved.join("index.json"),
        std::fs::Permissions::from_mode(0o666),
    )
    .expect("unsafe mode");
    let classified = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: classify.canonical.generation.expect("generation"),
                record_digest: classify.canonical.record_digest.clone().expect("digest"),
            },
            actor: "test".into(),
            reason: "unsafe mode".into(),
        },
    )
    .expect("classification reports unsafe mode");
    assert_eq!(classified.disposition, "unsafe");
}

#[test]
fn preserved_projection_recovery_keeps_initialized_and_ready_and_291_semantics_unchanged() {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join("docs")).expect("docs");
    std::fs::write(temp.path().join("docs/design.md"), "# design\n").expect("design");
    std::fs::write(
        temp.path().join("docs/diagram.mmd"),
        "flowchart LR\n A-->B\n",
    )
    .expect("diagram");
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
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
    let initialized = bootstrap_issue(
        &store,
        BootstrapRequest {
            issue: 291,
            repository: "example/repo".into(),
            actor: "agent".into(),
            design_path: "docs/design.md".into(),
            diagram_path: "docs/diagram.mmd".into(),
            design_reviewer: "architect".into(),
            design_approved: true,
            initial: fixture_initial_input(),
        },
    )
    .expect("initialized #291-compatible fixture");
    assert_eq!(initialized.phase, LifecyclePhase::Initialized);
    let ready = edit_issue(
        &store,
        EditRequest {
            issue: 291,
            card: CardKind::Sip,
            expected_generation: initialized.generation,
            expected_digest: initialized.digest,
            actor: "agent".into(),
            reason: "ready regression".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .expect("ready behavior remains available");
    assert_eq!(ready.phase, LifecyclePhase::Ready);
    assert!(!store.rollback_preserved(291).exists());
    assert!(!store.root().join(".csdlc/issues/.291.recovery").exists());
}

#[test]
fn preserved_projection_recovery_resumes_every_recovery_boundary() {
    for state in [
        "prepared",
        "archive_intent",
        "archive_renamed",
        "rejected_archived",
        "candidate_plan",
        "node_create_intent",
        "node_created_identity",
        "node_write_completed",
        "node_fsynced",
        "node_parent_fsynced",
        "node_published",
        "candidate_created",
        "candidate_verified",
        "install_intent",
        "install_exchanged",
        "canonical_installed",
        "displace_intent",
        "prior_displaced_renamed",
        "prior_displaced",
        "canonical_verified",
        "recovery_complete_intent",
    ] {
        let (_temp, store, record) = implemented_fixture();
        let preserved = store.rollback_preserved(7);
        copy_tree(&store.issue_dir(7), &preserved);
        let classify = classify_preserved_projection(
            &store,
            ProjectionClassifyRequest {
                issue: 7,
                anchor: ProjectionCasAnchor::VerifiedCanonical {
                    generation: record.generation,
                    record_digest: record.digest.clone(),
                },
                actor: "test".into(),
                reason: "classify failpoint fixture".into(),
            },
        )
        .expect("classify failpoint fixture");
        let mut request = ProjectionRecoverRequest {
            issue: 7,
            operation_id: format!("fail-{state}"),
            classify_receipt_digest: classify.receipt_digest.clone(),
            classification: classify.clone(),
            failed_operation_lineage: FailedOperationLineage {
                prior_generation: record.generation,
                prior_record_digest: record.digest,
                rejected_manifest_digest: classify
                    .preserved
                    .manifest_digest
                    .clone()
                    .expect("rejected manifest"),
                failure_boundary: "verifier_rejected_after_install".into(),
            },
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: classify.canonical.generation.expect("generation"),
                record_digest: classify.canonical.record_digest.clone().expect("digest"),
            },
            actor: "test".into(),
            reason: "recover after deterministic failpoint".into(),
            branch: "issue-7".into(),
            worktree: store.root().to_string_lossy().into_owned(),
            fail_after: Some(state.into()),
        };
        let interrupted = csdlc_v2::recover_preserved_projection(&store, request.clone())
            .expect_err("failpoint must interrupt");
        assert_eq!(
            interrupted.code,
            ErrorCode::InterruptedTransaction,
            "{state}"
        );
        request.fail_after = None;
        let recovered = csdlc_v2::recover_preserved_projection(&store, request)
            .unwrap_or_else(|error| panic!("restart after {state}: {error:?}"));
        assert_eq!(
            store
                .load_record(7)
                .expect("canonical after restart")
                .digest,
            recovered.canonical_digest,
            "{state}"
        );
    }
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_symlinked_partial_receipt_before_resume_mutation() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "partial symlink fixture".into(),
        },
    )
    .expect("classify");
    let mut request = recovery_request(&store, &record, &classify, "partial-symlink-receipt");
    request.fail_after = Some("prepared".into());
    csdlc_v2::recover_preserved_projection(&store, request.clone())
        .expect_err("prepared failpoint interrupts");
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/partial-symlink-receipt");
    let prepared = receipt_path(&attempt, 1, "prepared");
    let copy = attempt.join("001-prepared-copy.json");
    std::fs::copy(&prepared, &copy).expect("copy prepared receipt");
    std::fs::remove_file(&prepared).expect("remove regular prepared receipt");
    std::os::unix::fs::symlink(&copy, &prepared).expect("symlink prepared receipt");
    request.fail_after = None;
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("symlinked partial receipt must fail before resume");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert!(
        preserved.is_dir(),
        "resume must fail before archiving preserved evidence"
    );
}

#[test]
fn preserved_projection_recovery_rejects_post_validation_root_and_attempt_swaps_before_mutation() {
    for (operation, swap) in [
        (
            "post-validation-root-swap",
            "swap_recovery_root_after_validation",
        ),
        (
            "post-validation-attempt-swap",
            "swap_recovery_attempt_after_validation",
        ),
        (
            "pre-archive-root-swap",
            "swap_recovery_root_before_archive_mutation",
        ),
        (
            "pre-archive-attempt-swap",
            "swap_recovery_attempt_before_archive_mutation",
        ),
    ] {
        let (_temp, store, record) = implemented_fixture();
        let preserved = store.rollback_preserved(7);
        copy_tree(&store.issue_dir(7), &preserved);
        let classify = classify_preserved_projection(
            &store,
            ProjectionClassifyRequest {
                issue: 7,
                anchor: ProjectionCasAnchor::VerifiedCanonical {
                    generation: record.generation,
                    record_digest: record.digest.clone(),
                },
                actor: "test".into(),
                reason: swap.into(),
            },
        )
        .expect("classify");
        let mut request = recovery_request(&store, &record, &classify, operation);
        request.fail_after = Some(swap.into());
        let error = csdlc_v2::recover_preserved_projection(&store, request)
            .expect_err("post-validation substitution must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
        assert!(preserved.is_dir(), "rejected evidence must not be archived");
        let recovery = store.root().join(".csdlc/issues/.7.recovery");
        let displaced_root = recovery.with_extension(format!("{swap}.displaced"));
        let attempts = [recovery.join(operation), displaced_root.join(operation)];
        for attempt in attempts {
            if !swap.contains("before_archive") {
                assert!(!receipt_path(&attempt, 1, "prepared").exists());
            }
            assert!(!receipt_path(&attempt, 2, "archive-intent").exists());
            assert!(!attempt.join("candidate").exists());
            assert!(!attempt.join("rejected").exists());
        }
    }
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_symlinked_recovery_root() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let external = store.root().join("external-recovery-root");
    std::fs::create_dir(&external).expect("external recovery root");
    std::os::unix::fs::symlink(&external, store.root().join(".csdlc/issues/.7.recovery"))
        .expect("symlink recovery root");
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "symlinked recovery root fixture".into(),
        },
    )
    .expect_err("classification must reject symlinked recovery root before reading it");
    assert_eq!(classify.code, ErrorCode::CorruptRecord);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "symlinked recovery root fixture after removal".into(),
        },
    )
    .expect_err("recover also requires fail-closed classification authority");
    assert_eq!(classify.code, ErrorCode::CorruptRecord);
    assert!(
        preserved.is_dir(),
        "symlinked root must fail before archiving preserved evidence"
    );
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_blocks_ordinary_commit_on_symlinked_recovery_root() {
    let (_temp, store, record) = implemented_fixture();
    let external = store
        .root()
        .join("external-recovery-root-for-ordinary-commit");
    std::fs::create_dir(&external).expect("external recovery root");
    std::os::unix::fs::symlink(&external, store.root().join(".csdlc/issues/.7.recovery"))
        .expect("symlink recovery root");
    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test".into(),
            reason: "must block symlinked recovery root".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "blocked".into(),
                changes: vec!["none".into()],
                artifacts: vec!["none".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("ordinary commit must reject symlinked recovery root");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_symlinked_recovery_root_before_recover_mutation() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "clean classification before symlinked root".into(),
        },
    )
    .expect("classify before symlinked root");
    let external = store.root().join("external-recovery-root");
    std::fs::create_dir(&external).expect("external recovery root");
    std::os::unix::fs::symlink(&external, store.root().join(".csdlc/issues/.7.recovery"))
        .expect("symlink recovery root");
    let request = recovery_request(&store, &record, &classify, "symlinked-root");
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("symlinked recovery root must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert!(
        preserved.is_dir(),
        "symlinked root must fail before archiving preserved evidence"
    );
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_hardlinked_terminal_receipt() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "hardlinked-receipt");
    std::fs::hard_link(
        receipt_path(&attempt, 13, "recovered"),
        store.root().join("hardlinked-recovered-receipt.json"),
    )
    .expect("hardlink terminal receipt");
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("hardlinked receipt must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_permissive_terminal_receipt() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "permissive-receipt");
    let receipt = receipt_path(&attempt, 13, "recovered");
    let mut permissions = std::fs::metadata(&receipt)
        .expect("receipt metadata")
        .permissions();
    permissions.set_mode(0o644);
    std::fs::set_permissions(&receipt, permissions).expect("chmod receipt");
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("permissive receipt must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_swapped_post_exchange_candidate() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "classify swapped post-exchange fixture".into(),
        },
    )
    .expect("classify swapped post-exchange fixture");
    let mut request = recovery_request(&store, &record, &classify, "swap-after-exchange");
    request.fail_after = Some("canonical_installed".into());
    let interrupted = csdlc_v2::recover_preserved_projection(&store, request.clone())
        .expect_err("failpoint must interrupt after canonical install");
    assert_eq!(interrupted.code, ErrorCode::InterruptedTransaction);

    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/swap-after-exchange");
    let candidate = attempt.join("candidate");
    let swapped_aside = attempt.join("candidate.real");
    std::fs::rename(&candidate, &swapped_aside).expect("move real post-exchange prior aside");
    std::fs::create_dir(&candidate).expect("replacement candidate");
    std::fs::write(candidate.join("index.json"), b"{}").expect("replacement marker");

    request.fail_after = None;
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("swapped post-exchange prior must fail closed");
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    assert!(swapped_aside.is_dir());
    assert!(candidate.is_dir());
    assert!(!attempt.join("displaced").is_dir());
}

#[test]
fn preserved_projection_recovery_classifies_without_mutation_and_rejects_symlink() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    std::fs::rename(store.issue_dir(7), &preserved).expect("preserve canonical fixture");
    std::fs::create_dir_all(store.issue_dir(7)).expect("replacement canonical");
    std::fs::write(store.issue_dir(7).join("index.json"), b"{}\n").expect("invalid canonical");
    let canonical_meta = std::fs::symlink_metadata(store.issue_dir(7)).expect("canonical meta");
    use std::os::unix::fs::MetadataExt;
    let request = ProjectionClassifyRequest {
        issue: 7,
        anchor: ProjectionCasAnchor::ExactObservedInvalid {
            canonical_identity: csdlc_v2::NodeIdentity {
                device: canonical_meta.dev(),
                mount_id: format!("dev:{}", canonical_meta.dev()),
                inode: canonical_meta.ino(),
                ctime_seconds: canonical_meta.ctime(),
                ctime_nanoseconds: canonical_meta.ctime_nsec(),
                links: canonical_meta.nlink(),
                uid: canonical_meta.uid(),
                gid: canonical_meta.gid(),
                mode: canonical_meta.mode(),
                node_type: "directory".into(),
            },
            manifest_digest: String::new(),
            backup_generation: record.generation,
            backup_record_digest: record.digest.clone(),
        },
        actor: "test".into(),
        reason: "classify".into(),
    };
    let err = classify_preserved_projection(&store, request).expect_err("empty manifest CAS stale");
    assert_eq!(err.code, ErrorCode::StaleGeneration);
    assert!(
        preserved.is_dir(),
        "classification mutated preserved evidence"
    );
    assert!(
        store.issue_dir(7).is_dir(),
        "classification mutated canonical"
    );

    std::fs::remove_dir_all(store.issue_dir(7)).expect("remove invalid fixture");
    std::os::unix::fs::symlink(&preserved, store.issue_dir(7)).expect("symlink canonical");
    let request = ProjectionClassifyRequest {
        issue: 7,
        anchor: ProjectionCasAnchor::VerifiedCanonical {
            generation: record.generation,
            record_digest: record.digest,
        },
        actor: "test".into(),
        reason: "reject symlink".into(),
    };
    assert!(classify_preserved_projection(&store, request).is_err());
}

#[test]
fn preserved_projection_recovery_blocks_ordinary_commit_until_typed_recovery() {
    let (_temp, store, record) = implemented_fixture();
    std::fs::create_dir_all(store.rollback_preserved(7)).expect("preserved marker");
    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test".into(),
            reason: "must block".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "blocked".into(),
                changes: vec!["none".into()],
                artifacts: vec!["none".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("ordinary commit must fail closed");
    assert_eq!(error.code, ErrorCode::ReconciliationRequired);
}

#[test]
fn preserved_projection_recovery_validates_terminal_receipt_chain_and_classifies_completed_attempt()
{
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "terminal receipt fixture".into(),
        },
    )
    .expect("classify");
    let request = recovery_request(&store, &record, &classify, "receipt-chain");
    let recovered =
        csdlc_v2::recover_preserved_projection(&store, request.clone()).expect("recover");
    let completed = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: recovered.canonical_generation,
                record_digest: recovered.canonical_digest,
            },
            actor: "test".into(),
            reason: "classify completed".into(),
        },
    )
    .expect("classify completed");
    assert_eq!(completed.disposition, "already_recovered");

    let terminal = store
        .root()
        .join(".csdlc/issues/.7.recovery/receipt-chain/013-recovered.json");
    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&terminal).unwrap()).unwrap();
    envelope["previous_receipt_digest"] = serde_json::Value::String("tampered".into());
    std::fs::write(&terminal, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("tampered chain rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_extra_post_terminal_receipt() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "extra terminal receipt fixture".into(),
        },
    )
    .expect("classify");
    let request = recovery_request(&store, &record, &classify, "extra-terminal");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).expect("recover");
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/extra-terminal");
    append_extra_terminal_receipt(&attempt);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra post-terminal receipt must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_extra_post_terminal_symlink() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "extra terminal symlink fixture".into(),
        },
    )
    .expect("classify");
    let request = recovery_request(&store, &record, &classify, "extra-terminal-symlink");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).expect("recover");
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/extra-terminal-symlink");
    append_extra_terminal_symlink(&attempt);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra post-terminal symlink must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_extra_post_terminal_directory() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "extra terminal directory fixture".into(),
        },
    )
    .expect("classify");
    let request = recovery_request(&store, &record, &classify, "extra-terminal-directory");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).expect("recover");
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/extra-terminal-directory");
    append_extra_terminal_directory(&attempt);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra post-terminal directory must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_extra_post_terminal_fifo() {
    let (_temp, store, record) = implemented_fixture();
    let preserved = store.rollback_preserved(7);
    copy_tree(&store.issue_dir(7), &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "extra terminal FIFO fixture".into(),
        },
    )
    .expect("classify");
    let request = recovery_request(&store, &record, &classify, "extra-terminal-fifo");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).expect("recover");
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/extra-terminal-fifo");
    append_extra_terminal_fifo(&attempt);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra post-terminal FIFO must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_extra_node_receipt() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "extra-node-receipt");
    append_extra_node_receipt(&attempt, 0);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra node receipt must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[cfg(unix)]
#[test]
fn preserved_projection_recovery_rejects_extra_node_symlink() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "extra-node-symlink");
    append_extra_node_symlink(&attempt, 0);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra node symlink must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_extra_node_directory() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "extra-node-directory");
    append_extra_node_directory(&attempt, 0);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("extra node directory must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_node_payload_forgery() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "node-payload-forgery");
    rewrite_node_receipt_payload_and_rechain(
        &attempt,
        2,
        3,
        "write-intent",
        serde_json::json!({"digest":"0".repeat(64),"size":1}),
    );
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("rehashed node payload must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_node_final_path_forgery() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "node-final-path-forgery");
    let ordinal = 2;
    let forged = "cards/attacker-controlled.values.json";
    let mutations = [(1, "create-intent"), (9, "publish-intent")];
    for (seq, state) in mutations {
        let path = node_receipt_path(&attempt, ordinal, seq, state);
        let envelope: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).expect("node receipt"))
                .expect("node receipt json");
        let mut payload = envelope["payload"].clone();
        payload["final"] = serde_json::Value::String(forged.into());
        rewrite_node_receipt_payload_and_rechain(&attempt, ordinal, seq, state, payload);
    }
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("coherently rehashed final paths must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_node_created_identity_forgery() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) =
        completed_recovery_attempt(&store, &record, "node-created-identity-forgery");
    let ordinal = 2;
    let path = node_receipt_path(&attempt, ordinal, 2, "created-identity");
    let envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("node receipt"))
            .expect("node receipt json");
    let mut payload = envelope["payload"].clone();
    payload["inode"] = serde_json::json!(payload["inode"].as_u64().expect("inode") + 1);
    rewrite_node_receipt_payload_and_rechain(&attempt, ordinal, 2, "created-identity", payload);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("coherently rehashed created identity must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_node_terminal_identity_forgery() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) =
        completed_recovery_attempt(&store, &record, "node-terminal-identity-forgery");
    let ordinal = 2;
    let path = node_receipt_path(&attempt, ordinal, 10, "node-created");
    let envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("node receipt"))
            .expect("node receipt json");
    let mut payload = envelope["payload"].clone();
    payload["identity"]["inode"] =
        serde_json::json!(payload["identity"]["inode"].as_u64().expect("inode") + 1);
    rewrite_node_receipt_payload_and_rechain(&attempt, ordinal, 10, "node-created", payload);
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("coherently rehashed terminal identity must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_intermediate_payload_forgery() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "intermediate forgery fixture".into(),
        },
    )
    .unwrap();
    let request = recovery_request(&store, &record, &classify, "intermediate-forgery");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).unwrap();
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/intermediate-forgery");
    let mut candidate_created: serde_json::Value = serde_json::from_slice(
        &std::fs::read(receipt_path(&attempt, 5, "candidate-created")).unwrap(),
    )
    .unwrap();
    candidate_created["payload"]["record"]["repository"] =
        serde_json::Value::String("forged/repo".into());
    rewrite_receipt_payload_and_rechain(
        &attempt,
        5,
        "candidate-created",
        candidate_created["payload"].clone(),
    );
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("rehashed intermediate payload must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_envelope_extension() {
    let (_temp, store, record) = implemented_fixture();
    let (request, attempt) = completed_recovery_attempt(&store, &record, "envelope-extension");
    add_receipt_envelope_field_and_rechain(
        &attempt,
        5,
        "candidate-created",
        "forged_authority",
        serde_json::json!({"accepted":true}),
    );
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("top-level receipt envelope extension must fail closed");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_uses_backup_source_when_canonical_absent() {
    let (_temp, store, record) = implemented_fixture();
    let backup = store.interrupted_backup(7);
    let preserved = store.rollback_preserved(7);
    std::fs::rename(store.issue_dir(7), &backup).expect("move canonical to backup");
    copy_tree(&backup, &preserved);
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::ExpectedCanonicalAbsent {
                backup_generation: record.generation,
                backup_record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "expected absent canonical fixture".into(),
        },
    )
    .expect("classify absent canonical");
    assert_eq!(classify.disposition, "recoverable");
    assert_eq!(
        classify.backup.record_digest.as_deref(),
        Some(record.digest.as_str())
    );
    assert_eq!(classify.canonical.state, "absent");
    let request = ProjectionRecoverRequest {
        issue: 7,
        operation_id: "expected-absent-canonical".into(),
        classify_receipt_digest: classify.receipt_digest.clone(),
        classification: classify.clone(),
        failed_operation_lineage: FailedOperationLineage {
            prior_generation: record.generation,
            prior_record_digest: record.digest.clone(),
            rejected_manifest_digest: classify.preserved.manifest_digest.clone().unwrap(),
            failure_boundary: "canonical_absent_after_backup_preserved".into(),
        },
        anchor: ProjectionCasAnchor::ExpectedCanonicalAbsent {
            backup_generation: record.generation,
            backup_record_digest: record.digest.clone(),
        },
        actor: "test".into(),
        reason: "recover from backup while canonical absent".into(),
        branch: "issue-7".into(),
        worktree: store.root().to_string_lossy().into_owned(),
        fail_after: None,
    };
    let recovered =
        csdlc_v2::recover_preserved_projection(&store, request).expect("recover from backup");
    assert_eq!(recovered.canonical_generation, record.generation + 1);
    assert_eq!(
        store.load_record(7).expect("recovered canonical").digest,
        recovered.canonical_digest
    );
}

#[test]
fn preserved_projection_recovery_restarts_nonexchange_anchors_after_displacement() {
    for anchor_kind in ["expected-absent", "exact-invalid"] {
        for state in [
            "candidate_installed",
            "canonical_installed",
            "canonical_verified",
        ] {
            let (_temp, store, record) = implemented_fixture();
            let backup = store.interrupted_backup(7);
            let preserved = store.rollback_preserved(7);
            copy_tree(&store.issue_dir(7), &backup);

            let (classify, anchor, rejected_manifest_digest) = if anchor_kind == "expected-absent" {
                std::fs::rename(store.issue_dir(7), &preserved)
                    .expect("preserve canonical while making canonical absent");
                let classify = classify_preserved_projection(
                    &store,
                    ProjectionClassifyRequest {
                        issue: 7,
                        anchor: ProjectionCasAnchor::ExpectedCanonicalAbsent {
                            backup_generation: record.generation,
                            backup_record_digest: record.digest.clone(),
                        },
                        actor: "test".into(),
                        reason: "nonexchange restart absent fixture".into(),
                    },
                )
                .expect("classify absent canonical");
                (
                    classify.clone(),
                    ProjectionCasAnchor::ExpectedCanonicalAbsent {
                        backup_generation: record.generation,
                        backup_record_digest: record.digest.clone(),
                    },
                    classify.preserved.manifest_digest.clone().unwrap(),
                )
            } else {
                std::fs::create_dir(&preserved).expect("create invalid observed projection");
                std::fs::write(preserved.join("index.json"), b"{}\n")
                    .expect("write invalid observed projection");
                let probe = classify_preserved_projection(
                    &store,
                    ProjectionClassifyRequest {
                        issue: 7,
                        anchor: ProjectionCasAnchor::VerifiedCanonical {
                            generation: record.generation,
                            record_digest: record.digest.clone(),
                        },
                        actor: "test".into(),
                        reason: "capture exact invalid observation".into(),
                    },
                )
                .expect("observe invalid preserved projection");
                let mut invalid = probe.preserved;
                std::fs::remove_dir_all(store.issue_dir(7)).expect("remove valid canonical");
                std::fs::rename(&preserved, store.issue_dir(7))
                    .expect("install exact invalid canonical");
                copy_tree(&store.issue_dir(7), &preserved);
                use std::os::unix::fs::MetadataExt;
                let canonical_meta = std::fs::symlink_metadata(store.issue_dir(7))
                    .expect("invalid canonical metadata after rename");
                let mount_id = invalid.entries.first().unwrap().identity.mount_id.clone();
                invalid.entries.first_mut().unwrap().identity = csdlc_v2::NodeIdentity {
                    device: canonical_meta.dev(),
                    mount_id,
                    inode: canonical_meta.ino(),
                    ctime_seconds: canonical_meta.ctime(),
                    ctime_nanoseconds: canonical_meta.ctime_nsec(),
                    links: canonical_meta.nlink(),
                    uid: canonical_meta.uid(),
                    gid: canonical_meta.gid(),
                    mode: canonical_meta.mode(),
                    node_type: "directory".into(),
                };
                invalid.manifest_digest = Some(
                    blake3::hash(&serde_json::to_vec(&invalid.entries).unwrap())
                        .to_hex()
                        .to_string(),
                );
                let anchor = ProjectionCasAnchor::ExactObservedInvalid {
                    canonical_identity: invalid.entries.first().unwrap().identity.clone(),
                    manifest_digest: invalid.manifest_digest.clone().unwrap(),
                    backup_generation: record.generation,
                    backup_record_digest: record.digest.clone(),
                };
                let classify = classify_preserved_projection(
                    &store,
                    ProjectionClassifyRequest {
                        issue: 7,
                        anchor: anchor.clone(),
                        actor: "test".into(),
                        reason: "nonexchange restart invalid fixture".into(),
                    },
                )
                .expect("classify exact invalid canonical");
                (
                    classify.clone(),
                    anchor,
                    classify.preserved.manifest_digest.clone().unwrap(),
                )
            };

            let mut request = ProjectionRecoverRequest {
                issue: 7,
                operation_id: format!("restart-{anchor_kind}-{state}"),
                classify_receipt_digest: classify.receipt_digest.clone(),
                classification: classify,
                failed_operation_lineage: FailedOperationLineage {
                    prior_generation: record.generation,
                    prior_record_digest: record.digest.clone(),
                    rejected_manifest_digest,
                    failure_boundary: "nonexchange_prior_displaced".into(),
                },
                anchor,
                actor: "test".into(),
                reason: "restart nonexchange recovery after displacement".into(),
                branch: "issue-7".into(),
                worktree: store.root().to_string_lossy().into_owned(),
                fail_after: Some(state.into()),
            };
            let interrupted = csdlc_v2::recover_preserved_projection(&store, request.clone())
                .expect_err("failpoint must interrupt nonexchange recovery");
            assert_eq!(
                interrupted.code,
                ErrorCode::InterruptedTransaction,
                "{anchor_kind} at {state}: {interrupted:?}"
            );
            request.fail_after = None;
            let recovered = csdlc_v2::recover_preserved_projection(&store, request)
                .unwrap_or_else(|error| panic!("restart {anchor_kind} after {state}: {error:?}"));
            assert_eq!(
                store
                    .load_record(7)
                    .expect("canonical after restart")
                    .digest,
                recovered.canonical_digest
            );
        }
    }
}

#[test]
fn preserved_projection_recovery_rejects_rehashed_prepared_classification_forgery() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "prepared forgery fixture".into(),
        },
    )
    .unwrap();
    let request = recovery_request(&store, &record, &classify, "prepared-forgery");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).unwrap();
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/prepared-forgery");
    let mut prepared: serde_json::Value =
        serde_json::from_slice(&std::fs::read(receipt_path(&attempt, 1, "prepared")).unwrap())
            .unwrap();
    prepared["payload"]["classification"]["actor"] =
        serde_json::Value::String("forged-actor".into());
    rewrite_receipt_payload_and_rechain(&attempt, 1, "prepared", prepared["payload"].clone());
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("rehashed PREPARED classification must be rejected");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
}

#[test]
fn preserved_projection_recovery_rejects_coherent_candidate_chain_forgery() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "coherent candidate forgery fixture".into(),
        },
    )
    .unwrap();
    let request = recovery_request(&store, &record, &classify, "coherent-candidate-forgery");
    csdlc_v2::recover_preserved_projection(&store, request.clone()).unwrap();
    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/coherent-candidate-forgery");

    let candidate_path = receipt_path(&attempt, 5, "candidate-created");
    let mut candidate: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&candidate_path).unwrap()).unwrap();
    let mut forged_record: csdlc_v2::IssueRecord =
        serde_json::from_value(candidate["payload"]["record"].clone()).unwrap();
    forged_record.repository = "forged/repository".into();
    forged_record.digest.clear();
    forged_record.digest = csdlc_v2::cards::digest(&serde_json::to_vec(&forged_record).unwrap());
    let mut forged_index = serde_json::to_vec_pretty(&forged_record).unwrap();
    forged_index.push(b'\n');
    std::fs::write(store.issue_dir(7).join("index.json"), &forged_index).unwrap();

    let mut candidate_observation = candidate["payload"]["candidate"].clone();
    candidate_observation["record_digest"] = serde_json::json!(forged_record.digest);
    rewrite_observation_index(&mut candidate_observation, &forged_index);
    candidate["payload"]["record"] = serde_json::to_value(&forged_record).unwrap();
    candidate["payload"]["candidate"] = candidate_observation.clone();
    rewrite_receipt_payload_and_rechain(
        &attempt,
        5,
        "candidate-created",
        candidate["payload"].clone(),
    );

    rewrite_receipt_payload_and_rechain(
        &attempt,
        6,
        "candidate-verified",
        serde_json::json!({"candidate":candidate_observation,"record_digest":forged_record.digest,"generation":forged_record.generation}),
    );
    let mut installed = candidate_observation.clone();
    installed["name"] = serde_json::json!("canonical");
    rewrite_receipt_payload_and_rechain(
        &attempt,
        7,
        "install-intent",
        serde_json::json!({"exchange":true,"candidate":candidate_observation,"canonical":classify.canonical}),
    );
    rewrite_receipt_payload_and_rechain(
        &attempt,
        8,
        "canonical-installed",
        serde_json::json!({"canonical":installed}),
    );
    let archived: serde_json::Value = serde_json::from_value::<serde_json::Value>(
        serde_json::from_slice::<serde_json::Value>(
            &std::fs::read(receipt_path(&attempt, 11, "canonical-verified")).unwrap(),
        )
        .unwrap()["payload"]["archive"]
            .clone(),
    )
    .unwrap();
    let displaced = serde_json::from_slice::<serde_json::Value>(
        &std::fs::read(receipt_path(&attempt, 10, "prior-displaced")).unwrap(),
    )
    .unwrap()["payload"]
        .clone();
    rewrite_receipt_payload_and_rechain(
        &attempt,
        11,
        "canonical-verified",
        serde_json::json!({"canonical":installed,"archive":archived,"displaced":displaced}),
    );
    rewrite_receipt_payload_and_rechain(
        &attempt,
        12,
        "recovery-complete-intent",
        serde_json::json!({"canonical_digest":forged_record.digest,"generation":forged_record.generation}),
    );
    let mut terminal: csdlc_v2::ProjectionRecoveryResult = serde_json::from_value(
        serde_json::from_slice::<serde_json::Value>(
            &std::fs::read(receipt_path(&attempt, 13, "recovered")).unwrap(),
        )
        .unwrap()["payload"]
            .clone(),
    )
    .unwrap();
    terminal.canonical_digest = forged_record.digest;
    terminal.receipt_digest.clear();
    terminal.receipt_digest = blake3::hash(&serde_json::to_vec(&terminal).unwrap())
        .to_hex()
        .to_string();
    rewrite_receipt_payload_and_rechain(
        &attempt,
        13,
        "recovered",
        serde_json::to_value(terminal).unwrap(),
    );

    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("coherent candidate chain forgery must fail closed");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert_eq!(
        error.message,
        "candidate file node observation does not match authorized contents"
    );
}

#[test]
fn preserved_projection_recovery_rejects_forged_candidate_created_resume_before_install() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "active candidate forgery fixture".into(),
        },
    )
    .unwrap();
    let mut request = recovery_request(&store, &record, &classify, "active-candidate-forgery");
    request.fail_after = Some("candidate_created".into());
    let interrupted = csdlc_v2::recover_preserved_projection(&store, request.clone())
        .expect_err("candidate-created failpoint interrupts");
    assert_eq!(interrupted.code, ErrorCode::InterruptedTransaction);

    let attempt = store
        .root()
        .join(".csdlc/issues/.7.recovery/active-candidate-forgery");
    let candidate_path = receipt_path(&attempt, 5, "candidate-created");
    let mut candidate: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&candidate_path).unwrap()).unwrap();
    let mut forged_record: csdlc_v2::IssueRecord =
        serde_json::from_value(candidate["payload"]["record"].clone()).unwrap();
    forged_record.repository = "forged/repository".into();
    forged_record.digest.clear();
    forged_record.digest = csdlc_v2::cards::digest(&serde_json::to_vec(&forged_record).unwrap());
    let mut forged_index = serde_json::to_vec_pretty(&forged_record).unwrap();
    forged_index.push(b'\n');
    std::fs::write(attempt.join("candidate/index.json"), &forged_index).unwrap();
    let mut candidate_observation = candidate["payload"]["candidate"].clone();
    candidate_observation["record_digest"] = serde_json::json!(forged_record.digest);
    rewrite_observation_index(&mut candidate_observation, &forged_index);
    candidate["payload"]["record"] = serde_json::to_value(&forged_record).unwrap();
    candidate["payload"]["candidate"] = candidate_observation;
    rewrite_single_receipt_payload(
        &attempt,
        5,
        "candidate-created",
        candidate["payload"].clone(),
    );

    request.fail_after = None;
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("forged active candidate receipt must fail before install");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert!(!receipt_path(&attempt, 8, "canonical-installed").exists());
}

#[test]
fn preserved_projection_recovery_rejects_malformed_request_classification_before_mutation() {
    let (_temp, store, record) = implemented_fixture();
    copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
    let classify = classify_preserved_projection(
        &store,
        ProjectionClassifyRequest {
            issue: 7,
            anchor: ProjectionCasAnchor::VerifiedCanonical {
                generation: record.generation,
                record_digest: record.digest.clone(),
            },
            actor: "test".into(),
            reason: "malformed request fixture".into(),
        },
    )
    .unwrap();
    let mut request = recovery_request(&store, &record, &classify, "malformed-request");
    request.classification.actor = "forged-actor".into();
    let error = csdlc_v2::recover_preserved_projection(&store, request)
        .expect_err("malformed request classification must fail before mutation");
    assert_eq!(error.code, ErrorCode::CorruptRecord);
    assert!(store.rollback_preserved(7).is_dir());
    assert!(store.issue_dir(7).is_dir());
    assert!(!store
        .root()
        .join(".csdlc/issues/.7.recovery/malformed-request")
        .exists());
}

#[test]
fn preserved_projection_recovery_rejects_forged_terminal_and_broken_earlier_chain() {
    for mutation in [
        "terminal-self-digest",
        "operation-mismatch",
        "broken-earlier-link",
    ] {
        let (_temp, store, record) = implemented_fixture();
        copy_tree(&store.issue_dir(7), &store.rollback_preserved(7));
        let classify = classify_preserved_projection(
            &store,
            ProjectionClassifyRequest {
                issue: 7,
                anchor: ProjectionCasAnchor::VerifiedCanonical {
                    generation: record.generation,
                    record_digest: record.digest.clone(),
                },
                actor: "test".into(),
                reason: "negative receipt fixture".into(),
            },
        )
        .unwrap();
        let operation = format!("negative-{mutation}");
        let request = recovery_request(&store, &record, &classify, &operation);
        csdlc_v2::recover_preserved_projection(&store, request.clone()).unwrap();
        let attempt = store
            .root()
            .join(format!(".csdlc/issues/.7.recovery/{operation}"));
        let path = if mutation == "broken-earlier-link" {
            attempt.join("006-candidate-verified.json")
        } else {
            attempt.join("013-recovered.json")
        };
        let mut envelope: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        if mutation == "broken-earlier-link" {
            envelope["previous_receipt_digest"] = serde_json::Value::String("0".repeat(64));
        } else if mutation == "operation-mismatch" {
            envelope["payload"]["operation_id"] =
                serde_json::Value::String("other-operation".into());
        } else {
            envelope["payload"]["receipt_digest"] = serde_json::Value::String("0".repeat(64));
        }
        std::fs::write(&path, serde_json::to_vec_pretty(&envelope).unwrap()).unwrap();
        let error = csdlc_v2::recover_preserved_projection(&store, request).expect_err(mutation);
        assert!(
            matches!(
                error.code,
                ErrorCode::CorruptRecord | ErrorCode::ReconciliationRequired
            ),
            "{mutation}: {error:?}"
        );
    }
}

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

fn fixture_initial_input() -> InitialCardInput {
    InitialCardInput {
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
    }
}

#[test]
fn implemented_design_refresh_and_assignment_support_issue_local_artifacts() {
    let (_temp, store, implemented) = implemented_fixture_with_authored_paths(
        ".csdlc/issues/7/authored/design.md",
        ".csdlc/issues/7/authored/diagram.mmd",
    );
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
    .expect("assignment preserves valid issue-local authored copies");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "agent".into(),
            reason: "refresh issue-local authored artifacts".into(),
        },
    )
    .expect("typed recovery");
    std::fs::write(
        store.root().join(".csdlc/issues/7/authored/design.md"),
        "# refreshed issue-local design\n",
    )
    .unwrap();
    std::fs::write(
        store.root().join(".csdlc/issues/7/authored/diagram.mmd"),
        "flowchart LR\n C-->D\n",
    )
    .unwrap();

    let refreshed = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "agent".into(),
            reason: "refresh issue-local authored tuple".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect("issue-local authored refresh commits through expected inode replacement");
    let cards = store.load_cards(7).unwrap();
    let csdlc_v2::cards::CardContent::Spp(spp) = &cards[&CardKind::Spp].content else {
        panic!("SPP")
    };
    assert_eq!(
        spp.design_digest,
        digest(b"# refreshed issue-local design\n")
    );
    assert_eq!(spp.diagram_digest, digest(b"flowchart LR\n C-->D\n"));
    assert!(matches!(
        refreshed.design_review,
        csdlc_v2::model::DesignReview::Pending
    ));
    assert!(!store
        .root()
        .join(".csdlc/issues/.7.rollback-preserved")
        .exists());

    std::fs::write(
        store.root().join(".csdlc/issues/7/authored/design.md"),
        "# refreshed issue-local design r2\n",
    )
    .unwrap();
    let iterative = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: refreshed.generation,
            expected_digest: refreshed.digest,
            actor: "agent".into(),
            reason: "iterate the assignment-only authored tuple".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect("assignment-only iterative authored refresh remains supported");
    let advanced = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: iterative.generation,
            expected_digest: iterative.digest,
            actor: "agent".into(),
            reason: "record non-refresh suffix event".into(),
            operation: SemanticOperation::UpdatePlanStep {
                step_id: "one".into(),
                status: csdlc_v2::cards::StepStatus::InProgress,
            },
            fail_after_backup: false,
        },
    )
    .expect("ordinary plan update remains independently authorized");
    std::fs::write(
        store.root().join(".csdlc/issues/7/authored/design.md"),
        "# must not refresh after non-refresh suffix\n",
    )
    .unwrap();
    let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
    let blocked = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: advanced.generation,
            expected_digest: advanced.digest,
            actor: "agent".into(),
            reason: "reject assignment-only non-refresh suffix".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect_err("assignment-only compatibility must admit authored refreshes only");
    assert_eq!(blocked.code, ErrorCode::InvalidTransition);
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn implemented_design_refresh_survives_repairs_and_design_review_recovery() {
    let (_temp, store, implemented) = implemented_fixture();
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
            reason: "repair plan and authored design".into(),
        },
    )
    .expect("recover review");
    let recovery = recovered.audit.last().expect("recovery event").clone();
    let repaired = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "repair deliverable parity".into(),
            operation: SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: vec!["src/lib.rs".into(), "src/validate.sh".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("repair STP deliverables");
    let csdlc_v2::DesignReview::Approved { reviewer, revision } = repaired.design_review.clone()
    else {
        panic!("fixture design approval")
    };
    let design_recovered = csdlc_v2::recover_design_review(
        &store,
        RecoverDesignReviewRequest {
            issue: 7,
            expected_phase: LifecyclePhase::Implemented,
            expected_generation: repaired.generation,
            expected_digest: repaired.digest,
            previous_reviewer: reviewer.clone(),
            previous_revision: revision,
            false_reviewer: reviewer,
            actor: "operator".into(),
            reason: "clear stale design approval after repairs".into(),
            disposition: "fresh design review required".into(),
        },
    )
    .expect("recover design review");

    let shared_repair = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: design_recovered.generation,
            expected_digest: design_recovered.digest.clone(),
            actor: "operator".into(),
            reason: "shared predicate must remain closed".into(),
            operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                value: "must not apply after design recovery".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("design recovery must not widen ordinary repair authority");
    assert_eq!(shared_repair.code, ErrorCode::InvalidTransition);

    std::fs::write(store.root().join("docs/design.md"), "# repaired design\n").unwrap();
    std::fs::write(
        store.root().join("docs/diagram.mmd"),
        "flowchart LR\n Recovered-->Refreshed\n",
    )
    .unwrap();
    let refreshed = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: design_recovered.generation,
            expected_digest: design_recovered.digest,
            actor: "operator".into(),
            reason: "refresh authored tuple in the same recovery epoch".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect("refresh authored design after repairs and design recovery");
    assert!(matches!(
        refreshed.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    assert!(refreshed.review_assignment.is_none());
    assert!(refreshed.review.is_none());
    assert!(refreshed.publication.is_none());
    assert!(refreshed.readiness.is_none());
    assert!(refreshed.terminal.is_none());
    let refresh_audit: serde_json::Value =
        serde_json::from_str(&refreshed.audit.last().expect("refresh audit").operation)
            .expect("structured refresh audit");
    assert_eq!(refresh_audit["recovery_sequence"], recovery.sequence);
    assert_eq!(refresh_audit["recovery_generation"], recovery.generation);

    std::fs::write(
        store.root().join("docs/design.md"),
        "# repaired design r2\n",
    )
    .unwrap();
    let iterative = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: refreshed.generation,
            expected_digest: refreshed.digest,
            actor: "operator".into(),
            reason: "iterate authored tuple in the same recovery epoch".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect("iterative refresh remains supported");
    let iterative_audit: serde_json::Value =
        serde_json::from_str(&iterative.audit.last().expect("iterative audit").operation)
            .expect("structured iterative audit");
    assert_eq!(iterative_audit["recovery_sequence"], recovery.sequence);
    assert_eq!(iterative_audit["recovery_generation"], recovery.generation);
}

#[test]
fn implemented_design_refresh_rejects_unlisted_recovery_epoch_operation() {
    let (_temp, store, implemented) = implemented_fixture();
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
            reason: "start recovery epoch".into(),
        },
    )
    .expect("recover review");
    let advanced = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "record an unrelated execution-plan update".into(),
            operation: SemanticOperation::UpdatePlanStep {
                step_id: "one".into(),
                status: csdlc_v2::cards::StepStatus::InProgress,
            },
            fail_after_backup: false,
        },
    )
    .expect("authorized but refresh-unlisted update");
    std::fs::write(store.root().join("docs/design.md"), "# unrelated design\n").unwrap();
    let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
    let error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: advanced.generation,
            expected_digest: advanced.digest,
            actor: "operator".into(),
            reason: "reject unrelated recovery epoch".into(),
            operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
            fail_after_backup: false,
        },
    )
    .expect_err("unlisted recovery operation must block authored refresh");
    assert_eq!(error.code, ErrorCode::InvalidTransition);
    assert_eq!(
        std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
        before
    );
}

#[test]
fn implemented_design_refresh_rejects_absent_and_stale_recovery_epoch() {
    let (_temp, store, implemented) = implemented_fixture();
    std::fs::write(store.root().join("docs/design.md"), "# unanchored design\n").unwrap();
    let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
    for (generation, digest) in [
        (implemented.generation, implemented.digest.clone()),
        (implemented.generation + 1, implemented.digest.clone()),
        (implemented.generation, "0".repeat(64)),
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: generation,
                expected_digest: digest,
                actor: "operator".into(),
                reason: "reject absent or stale recovery epoch".into(),
                operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
                fail_after_backup: false,
            },
        )
        .expect_err("absent or stale recovery epoch must fail closed");
        assert!(matches!(
            error.code,
            ErrorCode::InvalidTransition | ErrorCode::StaleGeneration | ErrorCode::StaleDigest
        ));
        assert_eq!(
            std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
            before
        );
    }
}

#[test]
fn implemented_design_refresh_rejects_superseded_recovery_epoch() {
    for superseding_recorded_review in [false, true] {
        let (_temp, store, implemented) = implemented_fixture();
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
        .expect("record first review");
        let recovered = csdlc_v2::recover_review(
            &store,
            ReviewRecoveryRequest {
                issue: 7,
                expected_generation: reviewed.generation,
                expected_digest: reviewed.digest,
                actor: "operator".into(),
                reason: "start recovery epoch that will be superseded".into(),
            },
        )
        .expect("recover first review");
        let assigned = assign_review(
            &store,
            ReviewAssignmentRequest {
                issue: 7,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest,
                reviewer: "fresh-session:superseding".into(),
                assigned_by: "operator".into(),
                scope: vec!["src".into()],
            },
        )
        .expect("assign superseding review");
        let superseded = if superseding_recorded_review {
            record_review(
                &store,
                ReviewRecordRequest {
                    issue: 7,
                    expected_generation: assigned.generation,
                    expected_digest: assigned.digest,
                    actor: "fresh-session:superseding".into(),
                    evidence: ReviewEvidence {
                        reviewer: "fresh-session:superseding".into(),
                        scope: vec!["src".into()],
                        reviewed_revision: csdlc_v2::git::substantive_revision(
                            store.root(),
                            &["src".into()],
                        )
                        .expect("superseding revision"),
                        findings: vec![],
                        residual_risks: vec![],
                        completed: true,
                        non_substantive_proof: None,
                    },
                },
            )
            .expect("record superseding review")
        } else {
            assigned
        };
        std::fs::write(
            store.root().join("docs/design.md"),
            "# superseded recovery design\n",
        )
        .unwrap();
        let before = std::fs::read(store.issue_dir(7).join("index.json")).unwrap();
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: superseded.generation,
                expected_digest: superseded.digest,
                actor: "operator".into(),
                reason: "reject superseded recovery epoch".into(),
                operation: SemanticOperation::RefreshAuthoredDesignAfterRecovery,
                fail_after_backup: false,
            },
        )
        .expect_err("later assignment or review must supersede the old recovery epoch");
        assert_eq!(error.code, ErrorCode::InvalidTransition);
        assert_eq!(
            std::fs::read(store.issue_dir(7).join("index.json")).unwrap(),
            before
        );
    }
}

fn implemented_fixture() -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    implemented_fixture_with_authored_paths("docs/design.md", "docs/diagram.mmd")
}

fn implemented_fixture_with_authored_paths(
    design_path: &str,
    diagram_path: &str,
) -> (tempfile::TempDir, Store, csdlc_v2::IssueRecord) {
    let temp = tempfile::tempdir().expect("temp");
    std::fs::create_dir_all(temp.path().join(design_path).parent().unwrap())
        .expect("design parent");
    std::fs::create_dir_all(temp.path().join(diagram_path).parent().unwrap())
        .expect("diagram parent");
    std::fs::write(temp.path().join(design_path), "# reviewed design\n").expect("design");
    std::fs::write(temp.path().join(diagram_path), "flowchart LR\n A-->B\n").expect("diagram");
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
            design_path: design_path.into(),
            diagram_path: diagram_path.into(),
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
fn review_assignment_rejects_self_staling_lifecycle_scope() {
    let (_temp, store, record) = implemented_fixture();
    let error = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            reviewer: "subagent".into(),
            assigned_by: "agent".into(),
            scope: vec![".csdlc/issues/7/**".into()],
        },
    )
    .expect_err("generated issue lifecycle scope would stale itself");
    assert!(matches!(error.code, ErrorCode::InvalidInput));

    let after = store.load_record(7).expect("record remains readable");
    assert_eq!(after.digest, record.digest);
    assert!(after.review_assignment.is_none());
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
fn implemented_phase_card_truth_repair_unblocks_fresh_review_assignment() {
    let (_temp, store, implemented) = implemented_fixture();
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
    .expect("assign review");

    let active_assignment_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest.clone(),
            actor: "operator".into(),
            reason: "repair stale task boundary".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::TaskBoundary,
                value: "implemented and ready for fresh review".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("active review assignment must block implemented card repair");
    assert_eq!(active_assignment_error.code, ErrorCode::InvalidTransition);

    let assignment_only_recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "repair stale card truth before publication".into(),
        },
    )
    .expect("recover assignment-only implemented state");
    assert_eq!(assignment_only_recovered.phase, LifecyclePhase::Implemented);
    assert!(assignment_only_recovered.review_assignment.is_none());
    assert!(assignment_only_recovered.review.is_none());
    assert!(assignment_only_recovered.publication.is_none());
    assert!(assignment_only_recovered.readiness.is_none());
    assert!(assignment_only_recovered.terminal.is_none());
    let assignment_only_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: assignment_only_recovered.generation,
            expected_digest: assignment_only_recovered.digest.clone(),
            actor: "operator".into(),
            reason: "assignment-only recovery must not authorize implemented repair".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::TaskBoundary,
                value: "implemented and ready for fresh review".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("assignment-only recovery is not enough for card truth repair");
    assert_eq!(assignment_only_error.code, ErrorCode::InvalidTransition);

    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assignment_only_recovered.generation,
            expected_digest: assignment_only_recovered.digest.clone(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["src".into()],
                reviewed_revision: csdlc_v2::git::substantive_revision(
                    store.root(),
                    &["src".into()],
                )
                .expect("review revision"),
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record exact review before repair recovery");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair stale card truth before publication".into(),
        },
    )
    .expect("recover recorded-review implemented state");
    assert_eq!(recovered.phase, LifecyclePhase::Implemented);
    assert!(recovered.review_assignment.is_none());
    assert!(recovered.review.is_none());
    assert!(recovered.publication.is_none());
    assert!(recovered.readiness.is_none());
    assert!(recovered.terminal.is_none());

    let stale_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: recovered.generation,
            expected_digest: "stale-digest".into(),
            actor: "operator".into(),
            reason: "stale repair".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::TaskBoundary,
                value: "implemented and ready for fresh review".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("stale CAS must fail before mutation");
    assert_eq!(stale_error.code, ErrorCode::StaleDigest);

    let assert_task_boundary_rejected =
        |store: &Store, record: &csdlc_v2::IssueRecord, message: &str| {
            let error = edit_issue(
                store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Stp,
                    expected_generation: record.generation,
                    expected_digest: record.digest.clone(),
                    actor: "operator".into(),
                    reason: message.into(),
                    operation: SemanticOperation::SetField {
                        field: csdlc_v2::cards::TextField::TaskBoundary,
                        value: "implemented and ready for fresh review".into(),
                    },
                    fail_after_backup: false,
                },
            )
            .expect_err(message);
            assert_eq!(error.code, ErrorCode::InvalidTransition);
        };
    let assert_task_boundary_fails_closed =
        |store: &Store, record: &csdlc_v2::IssueRecord, message: &str| {
            let error = edit_issue(
                store,
                EditRequest {
                    issue: 7,
                    card: CardKind::Stp,
                    expected_generation: record.generation,
                    expected_digest: record.digest.clone(),
                    actor: "operator".into(),
                    reason: message.into(),
                    operation: SemanticOperation::SetField {
                        field: csdlc_v2::cards::TextField::TaskBoundary,
                        value: "implemented and ready for fresh review".into(),
                    },
                    fail_after_backup: false,
                },
            )
            .expect_err(message);
            assert!(
                matches!(
                    error.code,
                    ErrorCode::InvalidTransition | ErrorCode::CorruptRecord
                ),
                "unexpected fail-closed code: {:?}",
                error.code
            );
        };

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
    assert_task_boundary_rejected(&store, &retained, "retained publication must block repair");

    let mut retained = recovered.clone();
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
    assert_task_boundary_rejected(&store, &retained, "retained readiness must block repair");

    let mut retained = recovered.clone();
    retained.terminal = Some(csdlc_v2::model::TerminalEvidence {
        pull_request: Some(7),
        disposition: csdlc_v2::readiness::TerminalDisposition::Merged,
        observed_sha: Some("retained".into()),
        observed_state: "closed".into(),
        receipt_path: "terminal.json".into(),
        branch: Some("issue-7".into()),
        worktree: Some(store.root().to_string_lossy().into_owned()),
    });
    write_consistent_record(store.root(), &mut retained);
    assert_task_boundary_rejected(&store, &retained, "retained terminal must block repair");

    for phase in [
        LifecyclePhase::Reviewed,
        LifecyclePhase::Published,
        LifecyclePhase::MergeReady,
    ] {
        let mut retained = recovered.clone();
        retained.phase = phase;
        write_consistent_record(store.root(), &mut retained);
        assert_task_boundary_fails_closed(
            &store,
            &retained,
            "reviewed/published phase must block repair",
        );
    }

    let mut restored = recovered.clone();
    write_consistent_record(store.root(), &mut restored);
    let required_outcome = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "repair stale required outcome".into(),
            operation: SemanticOperation::CorrectRequiredOutcomeAfterRecovery {
                value: "Implemented parent proof is complete; fresh review and publication remain pending.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair SIP required outcome immediately after recorded-review recovery");

    let task_boundary = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: required_outcome.generation,
            expected_digest: required_outcome.digest,
            actor: "operator".into(),
            reason: "repair stale task boundary".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::TaskBoundary,
                value: "implemented and ready for fresh review".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair task boundary after typed recovery");

    let non_goals = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: task_boundary.generation,
            expected_digest: task_boundary.digest,
            actor: "operator".into(),
            reason: "repair stale non-goals".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: csdlc_v2::cards::PlanningCollectionField::NonGoals,
                values: vec![
                    "Do not widen lifecycle repair beyond pre-publication card truth".into(),
                ],
            },
            fail_after_backup: false,
        },
    )
    .expect("repair non-goals after typed recovery");

    let summary = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: non_goals.generation,
            expected_digest: non_goals.digest,
            actor: "operator".into(),
            reason: "repair stale plan summary".into(),
            operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                value: "Implemented work is complete; fresh exact review and publication remain pending.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair summary after assignment-only recovery");

    let vpp_summary = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: summary.generation,
            expected_digest: summary.digest,
            actor: "operator".into(),
            reason: "repair stale validation summary".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::PlanSummary,
                value: "Implemented validation covers the parent proof; fresh review and hosted publication checks remain pending.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair VPP summary after assignment-only recovery");

    let vpp_failure_policy = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: vpp_summary.generation,
            expected_digest: vpp_summary.digest,
            actor: "operator".into(),
            reason: "repair stale validation failure policy".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::FailurePolicy,
                value: "Fail closed on validation, review, publication, readiness, terminal, or stale authority drift.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair VPP failure policy after assignment-only recovery");

    let srp_prompts = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Srp,
            expected_generation: vpp_failure_policy.generation,
            expected_digest: vpp_failure_policy.digest,
            actor: "operator".into(),
            reason: "repair stale review prompts".into(),
            operation: SemanticOperation::CorrectReviewPromptsAfterRecovery {
                values: vec![
                    "Review the implemented exact head and lifecycle truth before publication."
                        .into(),
                    "Verify the repaired card truth does not widen product scope.".into(),
                ],
            },
            fail_after_backup: false,
        },
    )
    .expect("repair SRP prompts after assignment-only recovery");

    let sor_summary = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: srp_prompts.generation,
            expected_digest: srp_prompts.digest,
            actor: "operator".into(),
            reason: "repair stale SOR summary".into(),
            operation: SemanticOperation::SetField {
                field: csdlc_v2::cards::TextField::SorSummary,
                value: "Implemented evidence is recorded; publication and terminal closeout remain pending.".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("repair SOR summary after assignment-only recovery");

    let sor_follow_ups = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: sor_summary.generation,
            expected_digest: sor_summary.digest,
            actor: "operator".into(),
            reason: "repair stale SOR follow-ups".into(),
            operation: SemanticOperation::ReplaceSorFollowUpsAfterRecovery {
                values: vec![
                    "Obtain fresh exact-head review before typed publication.".into(),
                    "Finish only after typed publication, required CI, and merge authority are green.".into(),
                ],
            },
            fail_after_backup: false,
        },
    )
    .expect("repair SOR follow-ups after assignment-only recovery");

    let pre_review_publication_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: sor_follow_ups.generation,
            expected_digest: sor_follow_ups.digest.clone(),
            actor: "operator".into(),
            reason: "publication still requires review".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Published,
            },
            fail_after_backup: false,
        },
    )
    .expect_err("publication must still require fresh review evidence");
    assert_eq!(
        pre_review_publication_error.code,
        ErrorCode::InvalidTransition
    );

    let sor_ready = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: sor_follow_ups.generation,
            expected_digest: sor_follow_ups.digest,
            actor: "operator".into(),
            reason: "normalize SOR status after execution evidence".into(),
            operation: SemanticOperation::AdvanceStatus {
                status: csdlc_v2::cards::CardStatus::Ready,
            },
            fail_after_backup: false,
        },
    )
    .expect("normalize SOR status with execution evidence");

    let fresh_assignment = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: sor_ready.generation,
            expected_digest: sor_ready.digest,
            reviewer: "fresh-session:387".into(),
            assigned_by: "operator".into(),
            scope: vec!["csdlc-v2".into()],
        },
    )
    .expect("fresh review assignment after repair");
    assert!(fresh_assignment.review_assignment.is_some());

    assert_task_boundary_rejected(
        &store,
        &fresh_assignment,
        "fresh active assignment must block further card repair",
    );
}

#[test]
fn implemented_card_truth_repair_can_correct_vpp_and_sor_semantic_fields() {
    let (_temp, store, implemented) = implemented_fixture();
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
    .expect("record exact review before semantic repair recovery");
    let mut recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair stale VPP/SOR semantic fields before publication".into(),
        },
    )
    .expect("recover recorded-review implemented state");

    for (card, operation) in [
        (
            CardKind::Spp,
            SemanticOperation::CorrectValidationSummaryAfterRecovery {
                value: "wrong card".into(),
            },
        ),
        (
            CardKind::Vpp,
            SemanticOperation::CorrectValidationSummaryAfterRecovery { value: " ".into() },
        ),
        (
            CardKind::Vpp,
            SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { value: " ".into() },
        ),
        (
            CardKind::Sor,
            SemanticOperation::CorrectSorFollowUpsAfterRecovery {
                values: vec![" ".into()],
            },
        ),
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "reject malformed VPP/SOR correction".into(),
                operation,
                fail_after_backup: false,
            },
        )
        .expect_err("malformed VPP/SOR correction must fail");
        assert!(matches!(
            error.code,
            ErrorCode::InvalidTransition | ErrorCode::CardInvalid
        ));
    }

    recovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "correct VPP summary".into(),
            operation: SemanticOperation::CorrectValidationSummaryAfterRecovery {
                value: "implemented validation truth".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct VPP summary");
    let duplicate_summary = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject duplicate VPP summary correction".into(),
            operation: SemanticOperation::CorrectValidationSummaryAfterRecovery {
                value: "duplicate".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("duplicate VPP summary correction must fail");
    assert_eq!(duplicate_summary.code, ErrorCode::InvalidTransition);

    recovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "correct VPP failure policy".into(),
            operation: SemanticOperation::CorrectValidationFailurePolicyAfterRecovery {
                value: "fail closed on stale publication truth".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct VPP failure policy");
    recovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "correct SOR follow-ups".into(),
            operation: SemanticOperation::CorrectSorFollowUpsAfterRecovery {
                values: vec!["remaining typed follow-up".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("correct SOR follow-ups");

    let reassigned = assign_review(
        &store,
        ReviewAssignmentRequest {
            issue: 7,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            reviewer: "fresh-session:semantic-r2".into(),
            assigned_by: "operator".into(),
            scope: vec!["src".into()],
        },
    )
    .expect("assign second exact review");
    let reviewed_again = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: reassigned.generation,
            expected_digest: reassigned.digest,
            actor: "fresh-session:semantic-r2".into(),
            evidence: ReviewEvidence {
                reviewer: "fresh-session:semantic-r2".into(),
                scope: vec!["src".into()],
                reviewed_revision: csdlc_v2::git::substantive_revision(
                    store.root(),
                    &["src".into()],
                )
                .expect("second review revision"),
                findings: vec![],
                residual_risks: vec![],
                completed: true,
                non_substantive_proof: None,
            },
        },
    )
    .expect("record second exact review");
    let recovered_again = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed_again.generation,
            expected_digest: reviewed_again.digest,
            actor: "operator".into(),
            reason: "clear second review truth for SOR removal".into(),
        },
    )
    .expect("recover second review");
    let removed = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sor,
            expected_generation: recovered_again.generation,
            expected_digest: recovered_again.digest,
            actor: "operator".into(),
            reason: "remove stale SOR follow-ups".into(),
            operation: SemanticOperation::CorrectSorFollowUpsAfterRecovery { values: vec![] },
            fail_after_backup: false,
        },
    )
    .expect("empty vector removes SOR follow-ups");
    let after_cards = store.load_cards(7).expect("cards after VPP/SOR correction");
    let csdlc_v2::cards::CardContent::Vpp(after_vpp) = &after_cards[&CardKind::Vpp].content else {
        panic!("VPP")
    };
    let csdlc_v2::cards::CardContent::Sor(after_sor) = &after_cards[&CardKind::Sor].content else {
        panic!("SOR")
    };
    assert_eq!(after_vpp.summary, "implemented validation truth");
    assert_eq!(
        after_vpp.failure_policy,
        "fail closed on stale publication truth"
    );
    assert!(after_sor.follow_ups.is_empty());
    let audit: serde_json::Value = serde_json::from_str(
        &removed
            .audit
            .last()
            .expect("SOR correction audit")
            .operation,
    )
    .expect("structured SOR audit");
    assert_eq!(audit["operation"], "correct_sor_follow_ups_after_recovery");
    assert_eq!(audit["new_values"], serde_json::json!([]));
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
        assert!(audit["recovery_sequence"].as_u64().is_some());
        assert!(audit["recovery_generation"].as_u64().is_some());
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
fn implemented_plan_summary_recovery_survives_allowed_intervening_repairs() {
    let (_temp, store, implemented) = implemented_fixture();
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
    .expect("review");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair review finding".into(),
        },
    )
    .expect("recover");
    let recovery_event = recovered.audit.last().expect("recovery event").clone();
    let mut repaired = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "repair affected areas".into(),
            operation: SemanticOperation::ReplacePlanningCollection {
                field: csdlc_v2::cards::PlanningCollectionField::AffectedAreas,
                values: vec!["src".into(), "tests".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("allowed intervening repair");
    assert_eq!(
        edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Sip,
                expected_generation: repaired.generation,
                expected_digest: repaired.digest.clone(),
                actor: "operator".into(),
                reason: "required outcome must remain immediate-only".into(),
                operation: SemanticOperation::CorrectRequiredOutcomeAfterRecovery {
                    value: "must remain blocked".into(),
                },
                fail_after_backup: false,
            }
        )
        .expect_err("intervening repairs must not widen SIP recovery")
        .code,
        ErrorCode::InvalidTransition
    );
    repaired = approve_design(
        &store,
        csdlc_v2::store::ApproveDesignRequest {
            issue: 7,
            expected_generation: repaired.generation,
            expected_digest: repaired.digest,
            reviewer: "fresh-session:11111111-1111-4111-8111-111111111111".into(),
        },
    )
    .expect("approve repaired design");
    let cards = store.load_cards(7).expect("cards for exact repair chain");
    let csdlc_v2::cards::CardContent::Spp(spp) = &cards[&CardKind::Spp].content else {
        panic!("SPP")
    };
    repaired = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: repaired.generation,
            expected_digest: repaired.digest,
            actor: "operator".into(),
            reason: "replace plan steps".into(),
            operation: SemanticOperation::ReplacePlanSteps {
                steps: spp.steps.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("replace plan steps");
    let cards = store.load_cards(7).expect("cards for VPP repair");
    let csdlc_v2::cards::CardContent::Vpp(vpp) = &cards[&CardKind::Vpp].content else {
        panic!("VPP")
    };
    repaired = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Vpp,
            expected_generation: repaired.generation,
            expected_digest: repaired.digest,
            actor: "operator".into(),
            reason: "replace validation lanes".into(),
            operation: SemanticOperation::ReplaceValidationLanes {
                lanes: vpp.lanes.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("replace validation lanes");
    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: repaired.generation,
            expected_digest: repaired.digest.clone(),
            actor: "operator".into(),
            reason: "align recovered summary".into(),
            operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                value: "post-recovery plan".into(),
            },
            fail_after_backup: false,
        },
    )
    .expect("summary correction after allowed repair");
    assert_eq!(corrected.audit.last().unwrap().actor, "operator");
    let correction: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured correction audit");
    assert_eq!(correction["recovery_sequence"], recovery_event.sequence);
    assert_eq!(correction["recovery_generation"], recovery_event.generation);
    assert!(matches!(
        edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: corrected.generation,
                expected_digest: corrected.digest,
                actor: "operator".into(),
                reason: "reject second correction".into(),
                operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                    value: "second correction".into(),
                },
                fail_after_backup: false,
            }
        )
        .expect_err("second correction must end epoch")
        .code,
        ErrorCode::InvalidTransition
    ));

    let mut forged = repaired;
    forged.audit.push(csdlc_v2::model::AuditEvent {
        sequence: forged.audit.len() as u64 + 1,
        generation: forged.generation,
        actor: "forger".into(),
        reason: "unknown".into(),
        operation: "unknown_recovery_operation".into(),
    });
    write_consistent_record(store.root(), &mut forged);
    assert!(matches!(
        edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Spp,
                expected_generation: forged.generation,
                expected_digest: forged.digest,
                actor: "operator".into(),
                reason: "reject unknown".into(),
                operation: SemanticOperation::CorrectPlanSummaryAfterRecovery {
                    value: "blocked".into()
                },
                fail_after_backup: false,
            }
        )
        .expect_err("unknown operation must end epoch")
        .code,
        ErrorCode::InvalidTransition | ErrorCode::CorruptRecord
    ));
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
fn recovered_implemented_issue_can_correct_only_the_sip_goal() {
    let (_temp, store, implemented) = implemented_fixture();
    let operation = SemanticOperation::CorrectGoalAfterRecovery {
        value: "corrected issue-local ADR evidence goal".into(),
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
    .expect_err("unrecovered goal correction must fail");
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
            reason: "correct SIP goal".into(),
        },
    )
    .expect("recover review");
    let before_cards = store.load_cards(7).expect("cards before goal correction");
    let replacement = "corrected issue-local ADR evidence goal".to_string();
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
                operation: SemanticOperation::CorrectGoalAfterRecovery { value },
                fail_after_backup: false,
            },
        )
        .expect_err("wrong-card or blank correction must fail");
        assert!(matches!(
            rejected.code,
            ErrorCode::InvalidTransition | ErrorCode::CardInvalid
        ));
    }

    let stale = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered.generation + 1,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject stale generation".into(),
            operation: operation.clone(),
            fail_after_backup: false,
        },
    )
    .expect_err("stale generation must fail");
    assert_eq!(stale.code, ErrorCode::StaleGeneration);

    let mut retained = recovered.clone();
    retained.terminal = Some(csdlc_v2::model::TerminalEvidence {
        pull_request: Some(7),
        disposition: csdlc_v2::readiness::TerminalDisposition::Merged,
        observed_sha: Some("retained".into()),
        observed_state: "closed".into(),
        receipt_path: "terminal.json".into(),
        branch: Some("issue-7".into()),
        worktree: Some(store.root().to_string_lossy().into_owned()),
    });
    write_consistent_record(store.root(), &mut retained);
    let terminal_rejected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: retained.generation,
            expected_digest: retained.digest,
            actor: "operator".into(),
            reason: "retained terminal must block goal repair".into(),
            operation: operation.clone(),
            fail_after_backup: false,
        },
    )
    .expect_err("retained terminal truth must block goal repair");
    assert_eq!(terminal_rejected.code, ErrorCode::InvalidTransition);
    let mut restored = recovered.clone();
    write_consistent_record(store.root(), &mut restored);

    for broad_operation in [
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::Goal,
            value: replacement.clone(),
        },
        SemanticOperation::SetField {
            field: csdlc_v2::cards::TextField::RequiredOutcome,
            value: replacement.clone(),
        },
        SemanticOperation::ReplacePlanningCollection {
            field: csdlc_v2::cards::PlanningCollectionField::DeclaredScope,
            values: vec![replacement.clone()],
        },
    ] {
        let rejected = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Sip,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "reject broad SIP mutation".into(),
                operation: broad_operation,
                fail_after_backup: false,
            },
        )
        .expect_err("broad SIP mutation must remain rejected");
        assert_eq!(rejected.code, ErrorCode::InvalidTransition);
    }

    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Sip,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "align recovered SIP goal".into(),
            operation: SemanticOperation::CorrectGoalAfterRecovery {
                value: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct recovered SIP goal");
    let after_cards = store.load_cards(7).expect("cards after goal correction");
    let csdlc_v2::cards::CardContent::Sip(after_sip) = &after_cards[&CardKind::Sip].content else {
        panic!("SIP")
    };
    assert_eq!(after_sip.goal, replacement);
    for kind in [
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_eq!(
            after_cards[&kind].content, before_cards[&kind].content,
            "{kind} changed during SIP-only goal correction"
        );
    }
    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured goal audit");
    assert_eq!(audit["operation"], "correct_goal_after_recovery");
    assert_eq!(audit["new_value"], replacement);
    assert!(audit["recovery_sequence"].is_number());
    assert!(audit["recovery_generation"].is_number());
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

#[test]
fn recovered_implemented_issue_can_correct_stp_dependencies_after_recorded_review() {
    let (_temp, store, implemented) = implemented_fixture();
    let before_cards = store.load_cards(7).expect("load cards before correction");
    let csdlc_v2::cards::CardContent::Stp(before_stp) =
        before_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let replacement = vec![
        "#271 terminal and canonical".into(),
        "#114 terminal and canonical".into(),
        "#115 terminal and canonical".into(),
        "#116 terminal and canonical".into(),
        "#279 terminal and canonical".into(),
        "#280 terminal and canonical".into(),
        "#281 terminal and canonical".into(),
        "#282 terminal and canonical".into(),
        "#110 umbrella remains open until parent truth is reconciled".into(),
    ];

    let unrecovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "correct reviewed dependency denominator".into(),
            operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("ordinary implemented issue must not imply dependency recovery");
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
    let assignment_recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "assignment-only recovery".into(),
        },
    )
    .expect("recover assignment-only implemented state");
    let assignment_only_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: assignment_recovered.generation,
            expected_digest: assignment_recovered.digest.clone(),
            actor: "operator".into(),
            reason: "assignment-only recovery must not authorize dependency repair".into(),
            operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("assignment-only recovery must fail closed for dependencies");
    assert_eq!(assignment_only_error.code, ErrorCode::InvalidTransition);

    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assignment_recovered.generation,
            expected_digest: assignment_recovered.digest.clone(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["csdlc-v2".into()],
                reviewed_revision: csdlc_v2::git::substantive_revision(
                    store.root(),
                    &["csdlc-v2".into()],
                )
                .expect("review revision"),
                findings: vec![ReviewFindingEvidence {
                    id: "fresh-session:dependencies".into(),
                    severity: FindingSeverity::P1,
                    summary: "STP dependencies omit consumed terminal inputs".into(),
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
    .expect("record dependency review finding");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair dependency review finding".into(),
        },
    )
    .expect("recover recorded-review implemented state");

    for invalid in [
        Vec::<String>::new(),
        vec![" ".into()],
        vec![
            "#271 terminal and canonical".into(),
            " #271 terminal and canonical ".into(),
        ],
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Stp,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "reject malformed dependency replacement".into(),
                operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
                    values: invalid,
                },
                fail_after_backup: false,
            },
        )
        .expect_err("malformed dependencies must fail closed");
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
            operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
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
            reason: "align dependencies with reviewed terminal denominator".into(),
            operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct STP dependencies after recorded-review recovery");
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
    let after_cards = store.load_cards(7).expect("load corrected cards");
    let csdlc_v2::cards::CardContent::Stp(after_stp) = after_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let mut expected_stp = before_stp.clone();
    expected_stp.dependencies = replacement.clone();
    assert_eq!(after_stp, expected_stp);

    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured audit operation");
    assert_eq!(
        audit["operation"],
        "correct_stp_dependencies_after_recovery"
    );
    assert_eq!(
        audit["previous_values"],
        serde_json::json!(before_stp.dependencies)
    );
    assert_eq!(audit["new_values"], serde_json::json!(replacement));
    assert!(audit["recovery_sequence"].as_u64().is_some());
    assert!(audit["recovery_generation"].as_u64().is_some());

    let repeat = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: corrected.generation,
            expected_digest: corrected.digest,
            actor: "operator".into(),
            reason: "same recovery epoch must not authorize a second dependency repair".into(),
            operation: SemanticOperation::CorrectStpDependenciesAfterRecovery {
                values: vec!["#271 terminal and canonical".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("same recovery epoch must not authorize repeated dependency repair");
    assert_eq!(repeat.code, ErrorCode::InvalidTransition);
}

#[test]
fn recovered_implemented_issue_can_correct_stp_repo_inputs_after_recorded_review() {
    let (_temp, store, implemented) = implemented_fixture();
    let before_cards = store.load_cards(7).expect("load cards before correction");
    let csdlc_v2::cards::CardContent::Stp(before_stp) =
        before_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let replacement = vec![
        ".csdlc/prepared/issues/117/design.md".into(),
        ".csdlc/prepared/issues/117/diagram.mmd".into(),
        ".csdlc/prepared/issues/117/validate_preparation_bundle.py".into(),
        ".csdlc/issues/271".into(),
        "/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-parent-integration-proof/.csdlc/issues/114".into(),
    ];

    let unrecovered = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "correct reviewed repo input denominator".into(),
            operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("ordinary implemented issue must not imply repo input recovery");
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
    let assignment_recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest,
            actor: "operator".into(),
            reason: "assignment-only recovery".into(),
        },
    )
    .expect("recover assignment-only implemented state");
    let assignment_only_error = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: assignment_recovered.generation,
            expected_digest: assignment_recovered.digest.clone(),
            actor: "operator".into(),
            reason: "assignment-only recovery must not authorize repo input repair".into(),
            operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("assignment-only recovery must fail closed for repo inputs");
    assert_eq!(assignment_only_error.code, ErrorCode::InvalidTransition);

    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assignment_recovered.generation,
            expected_digest: assignment_recovered.digest.clone(),
            actor: "reviewer".into(),
            evidence: ReviewEvidence {
                reviewer: "reviewer".into(),
                scope: vec!["csdlc-v2".into()],
                reviewed_revision: csdlc_v2::git::substantive_revision(
                    store.root(),
                    &["csdlc-v2".into()],
                )
                .expect("review revision"),
                findings: vec![ReviewFindingEvidence {
                    id: "fresh-session:repo-inputs".into(),
                    severity: FindingSeverity::P1,
                    summary: "STP repo inputs omit consumed terminal inputs".into(),
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
    .expect("record repo input review finding");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair repo input review finding".into(),
        },
    )
    .expect("recover recorded-review implemented state");

    for invalid in [
        Vec::<String>::new(),
        vec![" ".into()],
        vec![".csdlc/issues/271".into(), " .csdlc/issues/271 ".into()],
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 7,
                card: CardKind::Stp,
                expected_generation: recovered.generation,
                expected_digest: recovered.digest.clone(),
                actor: "operator".into(),
                reason: "reject malformed repo input replacement".into(),
                operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery { values: invalid },
                fail_after_backup: false,
            },
        )
        .expect_err("malformed repo inputs must fail closed");
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
            operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery {
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
            reason: "align repo inputs with reviewed terminal denominator".into(),
            operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery {
                values: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct STP repo inputs after recorded-review recovery");
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
    let after_cards = store.load_cards(7).expect("load corrected cards");
    let csdlc_v2::cards::CardContent::Stp(after_stp) = after_cards[&CardKind::Stp].content.clone()
    else {
        panic!("STP")
    };
    let mut expected_stp = before_stp.clone();
    expected_stp.repo_inputs = replacement.clone();
    assert_eq!(after_stp, expected_stp);

    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured audit operation");
    assert_eq!(audit["operation"], "correct_stp_repo_inputs_after_recovery");
    assert_eq!(
        audit["previous_values"],
        serde_json::json!(before_stp.repo_inputs)
    );
    assert_eq!(audit["new_values"], serde_json::json!(replacement));
    assert!(audit["recovery_sequence"].as_u64().is_some());
    assert!(audit["recovery_generation"].as_u64().is_some());

    let repeat = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Stp,
            expected_generation: corrected.generation,
            expected_digest: corrected.digest,
            actor: "operator".into(),
            reason: "same recovery epoch must not authorize a second repo input repair".into(),
            operation: SemanticOperation::CorrectStpRepoInputsAfterRecovery {
                values: vec![".csdlc/issues/271".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect_err("same recovery epoch must not authorize repeated repo input repair");
    assert_eq!(repeat.code, ErrorCode::InvalidTransition);
}

#[test]
fn recovered_implemented_issue_can_correct_spp_step_status_after_recorded_review() {
    let (_temp, store, implemented) = implemented_fixture();
    let before_cards = store.load_cards(7).expect("load cards before correction");
    let csdlc_v2::cards::CardContent::Spp(before_spp) =
        before_cards[&CardKind::Spp].content.clone()
    else {
        panic!("SPP")
    };
    let mut replacement = before_spp.steps.clone();
    replacement[0].status = csdlc_v2::cards::StepStatus::Completed;

    let generic_replacement = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: implemented.generation,
            expected_digest: implemented.digest.clone(),
            actor: "operator".into(),
            reason: "generic implemented replace_plan_steps must remain blocked".into(),
            operation: SemanticOperation::ReplacePlanSteps {
                steps: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("generic implemented plan replacement must not accept completed status");
    assert_eq!(generic_replacement.code, ErrorCode::CardInvalid);

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
    let reviewed = record_review(
        &store,
        ReviewRecordRequest {
            issue: 7,
            expected_generation: assigned.generation,
            expected_digest: assigned.digest.clone(),
            actor: "subagent".into(),
            evidence: ReviewEvidence {
                reviewer: "subagent".into(),
                scope: vec!["csdlc-v2".into()],
                reviewed_revision: csdlc_v2::git::substantive_revision(
                    store.root(),
                    &["csdlc-v2".into()],
                )
                .expect("review revision"),
                findings: vec![ReviewFindingEvidence {
                    id: "fresh-session:spp-step-status".into(),
                    severity: FindingSeverity::P1,
                    summary: "SPP step status no longer reflects implemented execution truth"
                        .into(),
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
    .expect("record SPP step review finding");
    let recovered = csdlc_v2::recover_review(
        &store,
        ReviewRecoveryRequest {
            issue: 7,
            expected_generation: reviewed.generation,
            expected_digest: reviewed.digest,
            actor: "operator".into(),
            reason: "repair SPP step status review finding".into(),
        },
    )
    .expect("recover recorded-review implemented state");

    let mut rewritten_action = replacement.clone();
    rewritten_action[0].action = "rewrite substantive plan action".into();
    let substantive_rewrite = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject substantive plan action rewrite".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: rewritten_action,
            },
            fail_after_backup: false,
        },
    )
    .expect_err("substantive SPP plan rewrites must fail closed");
    assert_eq!(substantive_rewrite.code, ErrorCode::CardInvalid);

    let mut rewritten_id = replacement.clone();
    rewritten_id[0].id = "rewritten-step-id".into();
    let id_rewrite = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject SPP plan step id rewrite".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: rewritten_id,
            },
            fail_after_backup: false,
        },
    )
    .expect_err("SPP plan step id rewrites must fail closed");
    assert_eq!(id_rewrite.code, ErrorCode::CardInvalid);

    let mut rewritten_acceptance_ids = replacement.clone();
    rewritten_acceptance_ids[0].acceptance_ids = vec!["AC-rewritten".into()];
    let acceptance_ids_rewrite = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject SPP plan acceptance id rewrite".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: rewritten_acceptance_ids,
            },
            fail_after_backup: false,
        },
    )
    .expect_err("SPP plan acceptance id rewrites must fail closed");
    assert_eq!(acceptance_ids_rewrite.code, ErrorCode::CardInvalid);

    let mut dropped_step = replacement.clone();
    dropped_step.pop();
    let cardinality_rewrite = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            actor: "operator".into(),
            reason: "reject SPP plan cardinality rewrite".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: dropped_step,
            },
            fail_after_backup: false,
        },
    )
    .expect_err("SPP plan cardinality rewrites must fail closed");
    assert_eq!(cardinality_rewrite.code, ErrorCode::CardInvalid);

    let corrected = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest,
            actor: "operator".into(),
            reason: "align SPP step status with implemented execution truth".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect("correct SPP steps after recorded-review recovery");
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
    let after_cards = store.load_cards(7).expect("load corrected cards");
    let csdlc_v2::cards::CardContent::Spp(after_spp) = after_cards[&CardKind::Spp].content.clone()
    else {
        panic!("SPP")
    };
    let mut expected_spp = before_spp.clone();
    expected_spp.plan_revision += 1;
    expected_spp.steps = replacement.clone();
    assert_eq!(after_spp, expected_spp);

    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured audit operation");
    assert_eq!(audit["operation"], "correct_plan_steps_after_recovery");
    assert_eq!(audit["previous_steps"], serde_json::json!(before_spp.steps));
    assert_eq!(audit["new_steps"], serde_json::json!(replacement));
    assert!(audit["recovery_sequence"].as_u64().is_some());
    assert!(audit["recovery_generation"].as_u64().is_some());

    let repeat = edit_issue(
        &store,
        EditRequest {
            issue: 7,
            card: CardKind::Spp,
            expected_generation: corrected.generation,
            expected_digest: corrected.digest,
            actor: "operator".into(),
            reason: "same recovery epoch must not authorize a second plan-step repair".into(),
            operation: SemanticOperation::CorrectPlanStepsAfterRecovery {
                steps: replacement.clone(),
            },
            fail_after_backup: false,
        },
    )
    .expect_err("same recovery epoch must not authorize repeated plan-step repair");
    assert_eq!(repeat.code, ErrorCode::InvalidTransition);
}

#[test]
fn public_edit_schema_exposes_implemented_recovery_card_repairs() {
    let schema = csdlc_v2::public_schema_bundle()["edit_request"].to_string();
    for operation in [
        "correct_stp_dependencies_after_recovery",
        "correct_stp_repo_inputs_after_recovery",
        "correct_plan_steps_after_recovery",
    ] {
        assert!(
            schema.contains(operation),
            "edit_request schema omits {operation}"
        );
    }
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
