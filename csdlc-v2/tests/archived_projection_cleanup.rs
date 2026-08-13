use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use csdlc_v2::{
    execute_archived_projection_cleanup, ArchivedProjectionCleanupRequest,
    ArchivedProjectionCleanupStatus, ArchivedProjectionNode, CleanupNodeIdentity, CleanupNodeType,
};

fn git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("utf8")
        .trim()
        .to_owned()
}

fn repo() -> (tempfile::TempDir, PathBuf, String) {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("repo");
    fs::create_dir(&root).expect("repo dir");
    git(&root, &["init"]);
    git(&root, &["config", "user.email", "test@example.invalid"]);
    git(&root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "base\n").expect("readme");
    git(&root, &["add", "README.md"]);
    git(&root, &["commit", "-m", "base"]);
    let merge_sha = git(&root, &["rev-parse", "HEAD"]);
    (temp, root, merge_sha)
}

fn terminal(root: &Path, merge_sha: &str) -> (PathBuf, String) {
    let path = root.join("derived-terminal-298.json");
    let mut value = serde_json::json!({
        "schema": "csdlc.derived_terminal.v1",
        "issue": 298,
        "repository": "agent-logic/agent-design-language",
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
    .expect("write terminal");
    (path, digest)
}

fn identity(path: &Path, node_type: CleanupNodeType) -> CleanupNodeIdentity {
    let metadata = fs::symlink_metadata(path).expect("metadata");
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

fn authority(
    root: &Path,
    terminal_digest: &str,
    archived: &Path,
    nodes: &[ArchivedProjectionNode],
) -> (PathBuf, String, PathBuf, String) {
    let manifest_path = root.join("canonical-archive-manifest-298.json");
    let manifest = serde_json::json!({
        "schema": "csdlc.canonical_archive_manifest.v1",
        "issue": 298,
        "terminal_digest": terminal_digest,
        "archived_root": fs::canonicalize(archived).expect("canonical archived"),
        "nodes": nodes,
    });
    let manifest_bytes = serde_json::to_vec_pretty(&manifest).expect("manifest json");
    fs::write(&manifest_path, &manifest_bytes).expect("manifest");
    let manifest_digest = blake3::hash(&manifest_bytes).to_hex().to_string();

    let receipt_path = root.join("completed-recovery-receipt-298.json");
    let receipt = serde_json::json!({
        "schema": "csdlc.completed_recovery_receipt.v1",
        "issue": 298,
        "state": "completed",
        "terminal_digest": terminal_digest,
        "canonical_archive_manifest_digest": manifest_digest,
    });
    let receipt_bytes = serde_json::to_vec_pretty(&receipt).expect("receipt json");
    fs::write(&receipt_path, &receipt_bytes).expect("receipt");
    let receipt_digest = blake3::hash(&receipt_bytes).to_hex().to_string();

    (receipt_path, receipt_digest, manifest_path, manifest_digest)
}

fn request(
    root: &Path,
    merge_sha: &str,
    terminal_path: &Path,
    terminal_digest: &str,
    nodes: Vec<ArchivedProjectionNode>,
) -> ArchivedProjectionCleanupRequest {
    let archived = root.join("archived");
    let (receipt_path, receipt_digest, manifest_path, manifest_digest) =
        authority(root, terminal_digest, &archived, &nodes);
    ArchivedProjectionCleanupRequest {
        schema: "csdlc.archived_projection_cleanup_request.v1".into(),
        issue: 299,
        operation_id: "op-299".into(),
        repository_root: root.to_string_lossy().into_owned(),
        execution_base: merge_sha.into(),
        terminal_issue: 298,
        terminal_envelope: terminal_path.to_string_lossy().into_owned(),
        expected_terminal_digest: terminal_digest.into(),
        expected_terminal_merge_sha: merge_sha.into(),
        completed_recovery_receipt: receipt_path.to_string_lossy().into_owned(),
        expected_recovery_receipt_digest: receipt_digest,
        canonical_archive_manifest: manifest_path.to_string_lossy().into_owned(),
        expected_archive_manifest_digest: manifest_digest,
        archived_root: archived.to_string_lossy().into_owned(),
        cleanup_ledger_root: root.join("cleanup-ledger").to_string_lossy().into_owned(),
        nodes,
        fail_after: None,
    }
}

#[test]
fn cleanup_requires_terminal_gate_before_mutation() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let mut req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );
    req.expected_terminal_digest = "wrong".into();
    let error = execute_archived_projection_cleanup(&req).expect_err("wrong terminal rejected");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert!(
        node.exists(),
        "terminal failure must not mutate archived node"
    );
    assert!(!root.join("cleanup-ledger").exists());
}

#[test]
fn cleanup_requires_completed_recovery_and_manifest_before_mutation() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );

    let mut bad_manifest = req.clone();
    fs::write(
        &bad_manifest.canonical_archive_manifest,
        b"{\"schema\":\"csdlc.canonical_archive_manifest.v1\",\"issue\":298,\"terminal_digest\":\"wrong\",\"archived_root\":\"/nope\",\"nodes\":[]}",
    )
    .expect("bad manifest");
    bad_manifest.expected_archive_manifest_digest =
        blake3::hash(&fs::read(&bad_manifest.canonical_archive_manifest).expect("read bad"))
            .to_hex()
            .to_string();
    assert_eq!(
        execute_archived_projection_cleanup(&bad_manifest)
            .expect_err("bad canonical manifest rejected")
            .code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    assert!(node.exists());
    assert!(
        !root.join("cleanup-ledger").exists(),
        "manifest failure must not create cleanup namespace or receipts"
    );

    let missing_receipt = req.clone();
    fs::remove_file(&missing_receipt.completed_recovery_receipt).expect("remove receipt");
    assert_eq!(
        execute_archived_projection_cleanup(&missing_receipt)
            .expect_err("missing completed receipt rejected")
            .code,
        csdlc_v2::ErrorCode::Io
    );
    assert!(node.exists());
    assert!(
        !root.join("cleanup-ledger").exists(),
        "authority failure must not create cleanup namespace or receipts"
    );
}

#[test]
fn cleanup_rejects_coherent_caller_identity_forgery_without_manifest_binding() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let authorized = archived.join("authorized.json");
    fs::write(&authorized, "{}\n").expect("authorized");
    let forged = archived.join("forged.json");
    fs::write(&forged, "{\"forged\":true}\n").expect("forged");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let authorized_node = ArchivedProjectionNode {
        relative_path: "authorized.json".into(),
        identity: identity(&authorized, CleanupNodeType::RegularFile),
    };
    let (receipt_path, receipt_digest, manifest_path, manifest_digest) = authority(
        &root,
        &terminal_digest,
        &archived,
        std::slice::from_ref(&authorized_node),
    );

    let forged_req = ArchivedProjectionCleanupRequest {
        schema: "csdlc.archived_projection_cleanup_request.v1".into(),
        issue: 299,
        operation_id: "op-299".into(),
        repository_root: root.to_string_lossy().into_owned(),
        execution_base: merge_sha.clone(),
        terminal_issue: 298,
        terminal_envelope: terminal_path.to_string_lossy().into_owned(),
        expected_terminal_digest: terminal_digest,
        expected_terminal_merge_sha: merge_sha,
        completed_recovery_receipt: receipt_path.to_string_lossy().into_owned(),
        expected_recovery_receipt_digest: receipt_digest,
        canonical_archive_manifest: manifest_path.to_string_lossy().into_owned(),
        expected_archive_manifest_digest: manifest_digest,
        archived_root: archived.to_string_lossy().into_owned(),
        cleanup_ledger_root: root.join("cleanup-ledger").to_string_lossy().into_owned(),
        nodes: vec![ArchivedProjectionNode {
            relative_path: "forged.json".into(),
            identity: identity(&forged, CleanupNodeType::RegularFile),
        }],
        fail_after: None,
    };
    assert_eq!(
        execute_archived_projection_cleanup(&forged_req)
            .expect_err("caller-supplied coherent identity cannot replace manifest authority")
            .code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    assert_eq!(
        fs::read_to_string(&forged).expect("forged preserved"),
        "{\"forged\":true}\n"
    );
    assert!(authorized.exists());
    assert!(
        !root.join("cleanup-ledger").exists(),
        "forgery must fail before cleanup namespace or receipt mutation"
    );
}

#[test]
fn cleanup_rejects_ledger_inside_archive_before_namespace_mutation() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let mut req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );
    req.cleanup_ledger_root = archived
        .join("cleanup-ledger")
        .to_string_lossy()
        .into_owned();
    assert_eq!(
        execute_archived_projection_cleanup(&req)
            .expect_err("archive-local ledger rejected before create")
            .code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    );
    assert!(node.exists());
    assert!(
        !archived.join("cleanup-ledger").exists(),
        "unsafe cleanup ledger path must not be created inside archive"
    );
}

#[test]
fn cleanup_removes_only_exact_file_and_empty_directory_then_repeats_idempotently() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    fs::write(archived.join("stale.json"), "{}\n").expect("file");
    fs::create_dir(archived.join("empty-card")).expect("empty dir");
    fs::write(archived.join("sentinel.txt"), "keep\n").expect("sentinel");
    let file_identity = identity(&archived.join("stale.json"), CleanupNodeType::RegularFile);
    let dir_identity = identity(&archived.join("empty-card"), CleanupNodeType::Directory);
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![
            ArchivedProjectionNode {
                relative_path: "stale.json".into(),
                identity: file_identity,
            },
            ArchivedProjectionNode {
                relative_path: "empty-card".into(),
                identity: dir_identity,
            },
        ],
    );
    let result = execute_archived_projection_cleanup(&req).expect("cleanup");
    assert_eq!(result.status, ArchivedProjectionCleanupStatus::Completed);
    assert!(!archived.join("stale.json").exists());
    assert!(!archived.join("empty-card").exists());
    assert_eq!(
        fs::read_to_string(archived.join("sentinel.txt")).expect("sentinel"),
        "keep\n"
    );
    assert!(root
        .join("cleanup-ledger/op-299/900-cleanup-complete.json")
        .is_file());

    let repeated = execute_archived_projection_cleanup(&req).expect("repeat");
    assert_eq!(
        repeated.status,
        ArchivedProjectionCleanupStatus::AlreadyCompleted
    );
    assert_eq!(repeated.final_receipt_digest, result.final_receipt_digest);
}

#[test]
fn cleanup_rejects_replacement_inode_before_capture() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let original = identity(&node, CleanupNodeType::RegularFile);
    fs::remove_file(&node).expect("remove original");
    fs::write(&node, "{\"replacement\":true}\n").expect("replacement");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: original,
        }],
    );
    let error = execute_archived_projection_cleanup(&req).expect_err("replacement rejected");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read_to_string(&node).expect("replacement retained"),
        "{\"replacement\":true}\n"
    );
}

#[test]
fn cleanup_rejects_symlink_and_non_empty_directory() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    std::os::unix::fs::symlink("target", archived.join("link")).expect("symlink");
    fs::create_dir(archived.join("non-empty")).expect("dir");
    fs::write(archived.join("non-empty/child"), "x").expect("child");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let symlink_req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "link".into(),
            identity: CleanupNodeIdentity {
                device: 0,
                inode: 0,
                links: 0,
                uid: 0,
                gid: 0,
                mode: 0,
                node_type: CleanupNodeType::RegularFile,
            },
        }],
    );
    assert_eq!(
        execute_archived_projection_cleanup(&symlink_req)
            .expect_err("symlink rejected")
            .code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    );

    let dir_identity = identity(&archived.join("non-empty"), CleanupNodeType::Directory);
    let non_empty_req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "non-empty".into(),
            identity: dir_identity,
        }],
    );
    execute_archived_projection_cleanup(&non_empty_req).expect_err("non-empty dir rejected");
    assert!(archived.join("non-empty/child").exists());
}

#[test]
fn cleanup_resumes_after_capture_receipt_before_removal() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let mut req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );
    req.fail_after = Some("captured".into());
    execute_archived_projection_cleanup(&req).expect_err("injected capture failure");
    assert!(
        node.exists(),
        "placeholder occupies public path after capture"
    );
    req.fail_after = None;
    let result = execute_archived_projection_cleanup(&req).expect("resume");
    assert_eq!(result.status, ArchivedProjectionCleanupStatus::Completed);
    assert!(!node.exists());
}

#[test]
fn cleanup_preserves_public_replacement_before_placeholder_disposal() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let mut req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );
    req.fail_after = Some("removed".into());
    execute_archived_projection_cleanup(&req).expect_err("injected removed failure");
    fs::remove_file(&node).expect("remove operation placeholder");
    fs::write(&node, "{\"third_party\":true}\n").expect("replacement");
    req.fail_after = None;
    let error =
        execute_archived_projection_cleanup(&req).expect_err("replacement must be preserved");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert_eq!(
        fs::read_to_string(&node).expect("replacement retained"),
        "{\"third_party\":true}\n"
    );
}

fn single_file_request() -> (
    tempfile::TempDir,
    PathBuf,
    ArchivedProjectionCleanupRequest,
    PathBuf,
) {
    let (temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: identity(&node, CleanupNodeType::RegularFile),
        }],
    );
    (temp, root, req, node)
}

#[test]
fn cleanup_restarts_across_intent_receipt_fsync_and_disposal_boundaries() {
    let failpoints = [
        "operation_create_intent",
        "operation_created_before_receipt",
        "receipt_operation_created_temp_created",
        "receipt_operation_created_written",
        "receipt_operation_created_fsynced",
        "receipt_operation_created_renamed",
        "receipt_operation_created_parent_fsynced",
        "namespace_create_intent",
        "namespace_created_before_receipt",
        "receipt_namespace_created_temp_created",
        "receipt_namespace_created_written",
        "receipt_namespace_created_fsynced",
        "receipt_namespace_created_renamed",
        "receipt_namespace_created_parent_fsynced",
        "capture_intent",
        "placeholder_created",
        "exchange_intent",
        "exchanged_before_parent_fsync",
        "exchanged_parent_fsynced",
        "receipt_captured_temp_created",
        "receipt_captured_written",
        "receipt_captured_fsynced",
        "receipt_captured_renamed",
        "receipt_captured_parent_fsynced",
        "removal_intent",
        "removed_before_parent_fsync",
        "removed_parent_fsynced",
        "receipt_removed_temp_created",
        "receipt_removed_written",
        "receipt_removed_fsynced",
        "receipt_removed_renamed",
        "receipt_removed_parent_fsynced",
        "placeholder_disposed_before_parent_fsync",
        "placeholder_disposed_parent_fsynced",
        "receipt_placeholder_disposed_temp_created",
        "receipt_placeholder_disposed_written",
        "receipt_placeholder_disposed_fsynced",
        "receipt_placeholder_disposed_renamed",
        "receipt_placeholder_disposed_parent_fsynced",
        "receipt_cleanup_complete_temp_created",
        "receipt_cleanup_complete_written",
        "receipt_cleanup_complete_fsynced",
        "receipt_cleanup_complete_renamed",
        "receipt_cleanup_complete_parent_fsynced",
        "cleanup_complete",
    ];
    for failpoint in failpoints {
        let (_temp, _root, mut req, node) = single_file_request();
        req.fail_after = Some(failpoint.into());
        assert!(
            execute_archived_projection_cleanup(&req).is_err(),
            "injected failpoint {failpoint} should fail"
        );
        req.fail_after = None;
        let result = execute_archived_projection_cleanup(&req)
            .unwrap_or_else(|error| panic!("resume after {failpoint}: {error:?}"));
        assert!(
            matches!(
                result.status,
                ArchivedProjectionCleanupStatus::Completed
                    | ArchivedProjectionCleanupStatus::AlreadyCompleted
            ),
            "resume after {failpoint} returned {:?}",
            result.status
        );
        assert!(!node.exists(), "node should be absent after {failpoint}");
    }
}

#[test]
fn cleanup_rejects_temp_receipts_without_predecessor_and_placeholder_binding() {
    let (_temp, root, mut req, node) = single_file_request();
    req.fail_after = Some("receipt_captured_fsynced".into());
    execute_archived_projection_cleanup(&req).expect_err("leave captured temp receipt");
    let captured_tmp = root.join("cleanup-ledger/op-299/113-captured.json.tmp");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&captured_tmp).expect("read captured tmp"))
            .expect("captured tmp json");
    receipt["previous_receipt_digest"] = serde_json::Value::String("forged".into());
    fs::write(
        &captured_tmp,
        serde_json::to_vec_pretty(&receipt).expect("rewrite captured tmp"),
    )
    .expect("write forged predecessor");
    req.fail_after = None;
    assert_eq!(
        execute_archived_projection_cleanup(&req)
            .expect_err("forged predecessor digest rejected")
            .code,
        csdlc_v2::ErrorCode::CorruptRecord
    );
    assert!(node.exists());

    let (_temp, root, mut req, node) = single_file_request();
    req.fail_after = Some("receipt_captured_fsynced".into());
    execute_archived_projection_cleanup(&req).expect_err("leave captured temp receipt");
    let captured_tmp = root.join("cleanup-ledger/op-299/113-captured.json.tmp");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&captured_tmp).expect("read captured tmp"))
            .expect("captured tmp json");
    receipt["payload"]["placeholder_identity"]["inode"] = serde_json::Value::from(0_u64);
    fs::write(
        &captured_tmp,
        serde_json::to_vec_pretty(&receipt).expect("rewrite captured tmp"),
    )
    .expect("write forged placeholder");
    req.fail_after = None;
    assert_eq!(
        execute_archived_projection_cleanup(&req)
            .expect_err("forged placeholder identity rejected")
            .code,
        csdlc_v2::ErrorCode::CorruptRecord
    );
    assert!(node.exists());
}

#[test]
fn cleanup_rejects_hardlink_and_mode_drift_before_mutation() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    fs::create_dir(&archived).expect("archived");
    let node = archived.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    fs::hard_link(&node, archived.join("linked.json")).expect("hardlink");
    let hardlink_identity = identity(&node, CleanupNodeType::RegularFile);
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let hardlink_req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: hardlink_identity,
        }],
    );
    assert_eq!(
        execute_archived_projection_cleanup(&hardlink_req)
            .expect_err("hardlink rejected")
            .code,
        csdlc_v2::ErrorCode::UnsafeCheckout
    );

    fs::remove_file(archived.join("linked.json")).expect("remove hardlink");
    let mode_identity = identity(&node, CleanupNodeType::RegularFile);
    fs::set_permissions(&node, fs::Permissions::from_mode(0o600)).expect("mode drift");
    let mode_req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "stale.json".into(),
            identity: mode_identity,
        }],
    );
    assert_eq!(
        execute_archived_projection_cleanup(&mode_req)
            .expect_err("mode drift rejected")
            .code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    assert!(node.exists());
}

#[test]
fn cleanup_rejects_parent_replacement_and_corrupt_receipt() {
    let (_temp, root, merge_sha) = repo();
    let archived = root.join("archived");
    let nested = archived.join("nested");
    fs::create_dir_all(&nested).expect("nested");
    let node = nested.join("stale.json");
    fs::write(&node, "{}\n").expect("node");
    let original = identity(&node, CleanupNodeType::RegularFile);
    fs::rename(&nested, archived.join("nested.old")).expect("replace parent");
    fs::create_dir(&nested).expect("new nested");
    fs::write(&node, "{\"replacement\":true}\n").expect("replacement");
    let (terminal_path, terminal_digest) = terminal(&root, &merge_sha);
    let parent_req = request(
        &root,
        &merge_sha,
        &terminal_path,
        &terminal_digest,
        vec![ArchivedProjectionNode {
            relative_path: "nested/stale.json".into(),
            identity: original,
        }],
    );
    assert_eq!(
        execute_archived_projection_cleanup(&parent_req)
            .expect_err("parent replacement rejected")
            .code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    assert_eq!(
        fs::read_to_string(&node).expect("replacement retained"),
        "{\"replacement\":true}\n"
    );

    let (_temp, root, mut req, _node) = single_file_request();
    execute_archived_projection_cleanup(&req).expect("initial cleanup");
    let final_receipt = root.join("cleanup-ledger/op-299/900-cleanup-complete.json");
    fs::write(&final_receipt, b"{\"schema\":\"wrong\"}\n").expect("corrupt receipt");
    req.fail_after = None;
    assert_eq!(
        execute_archived_projection_cleanup(&req)
            .expect_err("corrupt receipt rejected")
            .code,
        csdlc_v2::ErrorCode::CorruptRecord
    );
}
