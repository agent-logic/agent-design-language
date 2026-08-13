use std::collections::BTreeSet;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use csdlc_v2::{ErrorCode, Result, V2Error};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArchivedProjectionCleanupRequest {
    pub schema: String,
    pub issue: u64,
    pub operation_id: String,
    pub repository_root: String,
    pub execution_base: String,
    pub terminal_issue: u64,
    pub terminal_envelope: String,
    pub expected_terminal_digest: String,
    pub expected_terminal_merge_sha: String,
    pub completed_recovery_receipt: String,
    pub expected_recovery_receipt_digest: String,
    pub canonical_archive_manifest: String,
    pub expected_archive_manifest_digest: String,
    pub archived_root: String,
    pub cleanup_ledger_root: String,
    pub nodes: Vec<ArchivedProjectionNode>,
    #[serde(default)]
    pub fail_after: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArchivedProjectionNode {
    pub relative_path: String,
    pub identity: CleanupNodeIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CompletedRecoveryReceipt {
    schema: String,
    issue: u64,
    state: String,
    terminal_digest: String,
    canonical_archive_manifest_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct CanonicalArchiveManifest {
    schema: String,
    issue: u64,
    terminal_digest: String,
    archived_root: String,
    nodes: Vec<ArchivedProjectionNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CleanupNodeIdentity {
    pub device: u64,
    pub inode: u64,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub node_type: CleanupNodeType,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CleanupNodeType {
    RegularFile,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArchivedProjectionCleanupResult {
    pub schema: String,
    pub issue: u64,
    pub operation_id: String,
    pub status: ArchivedProjectionCleanupStatus,
    pub terminal_digest: String,
    pub cleaned_nodes: Vec<String>,
    pub final_receipt_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ArchivedProjectionCleanupStatus {
    Completed,
    AlreadyCompleted,
}

pub fn execute_archived_projection_cleanup(
    request: &ArchivedProjectionCleanupRequest,
) -> Result<ArchivedProjectionCleanupResult> {
    validate_request(request)?;
    validate_terminal_gate(request)?;

    let archived_root =
        canonical_real_directory(Path::new(&request.archived_root), "archived root")?;
    validate_recovery_authority(request, &archived_root)?;
    let ledger_root =
        canonical_or_create_directory(Path::new(&request.cleanup_ledger_root), "cleanup ledger")?;
    if archived_root == ledger_root || ledger_root.starts_with(&archived_root) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "cleanup ledger must be outside the archived projection tree",
        ));
    }
    require_same_device(&archived_root, &ledger_root, "cleanup ledger")?;
    let operation_root = ledger_root.join(&request.operation_id);
    ensure_private_component(&request.operation_id, "operation id")?;
    failpoint(request, "operation_create_intent")?;
    fs::create_dir_all(&operation_root)?;
    sync_directory(&ledger_root)?;
    failpoint(request, "operation_created_before_receipt")?;
    receipt(
        request,
        &operation_root,
        1,
        "operation-created",
        &json!({
            "issue": request.issue,
            "operation_id": request.operation_id,
            "terminal_issue": request.terminal_issue,
            "terminal_digest": request.expected_terminal_digest,
            "recovery_receipt_digest": request.expected_recovery_receipt_digest,
            "archive_manifest_digest": request.expected_archive_manifest_digest,
            "archived_root": archived_root,
        }),
    )?;
    failpoint(request, "operation_created")?;

    let private_root = operation_root.join("private-delete");
    failpoint(request, "namespace_create_intent")?;
    fs::create_dir_all(&private_root)?;
    set_private_permissions(&private_root)?;
    sync_directory(&operation_root)?;
    failpoint(request, "namespace_created_before_receipt")?;
    receipt(
        request,
        &operation_root,
        2,
        "namespace-created",
        &json!({"private_root":"private-delete","mode":"0700"}),
    )?;
    failpoint(request, "namespace_created")?;

    let final_path = receipt_path(&operation_root, 900, "cleanup-complete");
    if final_path.is_file() {
        let _: Value = receipt_payload(&final_path)?;
        let digest = blake3::hash(&read_regular_no_symlink(&final_path)?)
            .to_hex()
            .to_string();
        return Ok(result(
            request,
            ArchivedProjectionCleanupStatus::AlreadyCompleted,
            digest,
        ));
    }

    let mut seen = BTreeSet::new();
    for (index, node) in request.nodes.iter().enumerate() {
        if !seen.insert(node.relative_path.clone()) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "cleanup node list contains duplicate relative paths",
            ));
        }
        cleanup_one_node(
            request,
            &archived_root,
            &private_root,
            &operation_root,
            index,
            node,
        )?;
    }

    receipt(
        request,
        &operation_root,
        900,
        "cleanup-complete",
        &json!({
            "issue": request.issue,
            "operation_id": request.operation_id,
            "nodes": request.nodes.iter().map(|node| node.relative_path.as_str()).collect::<Vec<_>>(),
        }),
    )?;
    failpoint(request, "cleanup_complete")?;
    let digest = blake3::hash(&read_regular_no_symlink(&final_path)?)
        .to_hex()
        .to_string();
    Ok(result(
        request,
        ArchivedProjectionCleanupStatus::Completed,
        digest,
    ))
}

fn cleanup_one_node(
    request: &ArchivedProjectionCleanupRequest,
    archived_root: &Path,
    private_root: &Path,
    operation_root: &Path,
    index: usize,
    node: &ArchivedProjectionNode,
) -> Result<()> {
    let ordinal = index as u32 + 1;
    let base = 100 + ordinal * 10;
    let source = safe_child(archived_root, &node.relative_path)?;
    let private = private_root.join(format!("node-{ordinal:03}"));
    let capture_receipt = receipt_path(operation_root, base + 3, "captured");
    let removed_receipt = receipt_path(operation_root, base + 6, "removed");
    let disposed_receipt = receipt_path(operation_root, base + 8, "placeholder-disposed");
    if disposed_receipt.is_file() {
        return Ok(());
    }

    if !capture_receipt.is_file() {
        let observed = observe_existing(&source)?;
        if observed != node.identity {
            let expected_placeholder = recorded_placeholder_identity(operation_root, base).ok();
            let captured_private = observe_existing(&private).ok();
            if expected_placeholder.as_ref() == Some(&observed)
                && captured_private.as_ref() == Some(&node.identity)
            {
                receipt(
                    request,
                    operation_root,
                    base + 3,
                    "captured",
                    &json!({"path":node.relative_path,"expected_identity":node.identity,"placeholder_identity":observed,"adopted_after_exchange":true}),
                )?;
                failpoint(request, "captured")?;
            } else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "archived node identity does not match cleanup authority",
                ));
            }
        } else {
            if node.identity.node_type == CleanupNodeType::RegularFile && observed.links != 1 {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "cleanup refuses hardlinked archived files",
                ));
            }
            if node.identity.node_type == CleanupNodeType::Directory
                && fs::read_dir(&source)?.next().is_some()
            {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "cleanup refuses non-empty archived directories",
                ));
            }
            failpoint(request, "capture_intent")?;
            let placeholder_identity = if private.exists() {
                let identity = observe_existing(&private)?;
                let expected_placeholder = recorded_placeholder_identity(operation_root, base)?;
                if identity != expected_placeholder {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "private placeholder identity drifted before exchange",
                    ));
                }
                identity
            } else {
                create_placeholder(&private, node.identity.node_type)?;
                let placeholder_identity = observe_existing(&private)?;
                receipt(
                    request,
                    operation_root,
                    base,
                    "placeholder-created",
                    &json!({"path":node.relative_path,"private":relative_to(private_root, &private)?,"identity":placeholder_identity}),
                )?;
                failpoint(request, "placeholder_created")?;
                placeholder_identity
            };
            failpoint(request, "exchange_intent")?;
            exchange_paths(&source, &private)?;
            failpoint(request, "exchanged_before_parent_fsync")?;
            sync_directory(source.parent().ok_or_else(|| {
                V2Error::new(ErrorCode::InvalidInput, "source path has no parent")
            })?)?;
            sync_directory(private_root)?;
            failpoint(request, "exchanged_parent_fsynced")?;
            receipt(
                request,
                operation_root,
                base + 3,
                "captured",
                &json!({"path":node.relative_path,"expected_identity":node.identity,"placeholder_identity":placeholder_identity}),
            )?;
            failpoint(request, "captured")?;
        }
    }

    if !removed_receipt.is_file() {
        if private.try_exists()? {
            let captured = observe_existing(&private)?;
            if captured != node.identity {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "captured private node identity does not match cleanup authority",
                ));
            }
            failpoint(request, "removal_intent")?;
            match node.identity.node_type {
                CleanupNodeType::RegularFile => fs::remove_file(&private)?,
                CleanupNodeType::Directory => fs::remove_dir(&private)?,
            }
            failpoint(request, "removed_before_parent_fsync")?;
        }
        sync_directory(private_root)?;
        failpoint(request, "removed_parent_fsynced")?;
        receipt(
            request,
            operation_root,
            base + 6,
            "removed",
            &json!({"path":node.relative_path,"removed_identity":node.identity,"adopted_after_unlink":!private.try_exists()?}),
        )?;
        failpoint(request, "removed")?;
    }

    if !disposed_receipt.is_file() {
        let expected_placeholder = recorded_placeholder_identity(operation_root, base)?;
        if source.try_exists()? {
            let placeholder = observe_existing(&source)?;
            if placeholder != expected_placeholder {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "public placeholder identity drifted before disposal",
                ));
            }
            match placeholder.node_type {
                CleanupNodeType::RegularFile => fs::remove_file(&source)?,
                CleanupNodeType::Directory => fs::remove_dir(&source)?,
            }
            failpoint(request, "placeholder_disposed_before_parent_fsync")?;
        }
        sync_directory(
            source.parent().ok_or_else(|| {
                V2Error::new(ErrorCode::InvalidInput, "source path has no parent")
            })?,
        )?;
        failpoint(request, "placeholder_disposed_parent_fsynced")?;
        receipt(
            request,
            operation_root,
            base + 8,
            "placeholder-disposed",
            &json!({"path":node.relative_path,"placeholder_identity":expected_placeholder,"adopted_after_unlink":!source.try_exists()?}),
        )?;
        failpoint(request, "placeholder_disposed")?;
    }
    Ok(())
}

fn recorded_placeholder_identity(operation_root: &Path, base: u32) -> Result<CleanupNodeIdentity> {
    let payload: Value =
        receipt_payload(&receipt_path(operation_root, base, "placeholder-created"))?;
    serde_json::from_value(payload.get("identity").cloned().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "placeholder receipt identity is missing",
        )
    })?)
    .map_err(Into::into)
}

fn result(
    request: &ArchivedProjectionCleanupRequest,
    status: ArchivedProjectionCleanupStatus,
    final_receipt_digest: String,
) -> ArchivedProjectionCleanupResult {
    ArchivedProjectionCleanupResult {
        schema: "csdlc.archived_projection_cleanup_result.v1".into(),
        issue: request.issue,
        operation_id: request.operation_id.clone(),
        status,
        terminal_digest: request.expected_terminal_digest.clone(),
        cleaned_nodes: request
            .nodes
            .iter()
            .map(|node| node.relative_path.clone())
            .collect(),
        final_receipt_digest,
    }
}

fn validate_request(request: &ArchivedProjectionCleanupRequest) -> Result<()> {
    if request.schema != "csdlc.archived_projection_cleanup_request.v1"
        || request.issue == 0
        || request.operation_id.trim().is_empty()
        || request.terminal_issue == 0
        || request.expected_terminal_digest.trim().is_empty()
        || request.expected_terminal_merge_sha.trim().is_empty()
        || request.completed_recovery_receipt.trim().is_empty()
        || request.expected_recovery_receipt_digest.trim().is_empty()
        || request.canonical_archive_manifest.trim().is_empty()
        || request.expected_archive_manifest_digest.trim().is_empty()
        || request.nodes.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "archived projection cleanup request identity is incomplete",
        ));
    }
    ensure_private_component(&request.operation_id, "operation id")?;
    Ok(())
}

fn validate_terminal_gate(request: &ArchivedProjectionCleanupRequest) -> Result<()> {
    let terminal_path = Path::new(&request.terminal_envelope);
    let terminal: Value = serde_json::from_slice(&read_regular_no_symlink(terminal_path)?)?;
    let digest = terminal
        .get("digest")
        .and_then(Value::as_str)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "terminal digest is missing"))?;
    let merge_sha = terminal
        .get("merge_sha")
        .and_then(Value::as_str)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "terminal merge SHA is missing"))?;
    if terminal.get("schema").and_then(Value::as_str) != Some("csdlc.derived_terminal.v1")
        || terminal.get("issue").and_then(Value::as_u64) != Some(request.terminal_issue)
        || terminal.get("disposition").and_then(Value::as_str) != Some("merged")
        || terminal.get("issue_state").and_then(Value::as_str) != Some("closed_by_merged_pr")
        || digest != request.expected_terminal_digest
        || merge_sha != request.expected_terminal_merge_sha
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "terminal envelope does not grant exact cleanup authority",
        ));
    }
    let output = Command::new("git")
        .current_dir(&request.repository_root)
        .args([
            "merge-base",
            "--is-ancestor",
            merge_sha,
            &request.execution_base,
        ])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "terminal merge SHA is not ancestral to cleanup execution base",
        ));
    }
    Ok(())
}

fn validate_recovery_authority(
    request: &ArchivedProjectionCleanupRequest,
    archived_root: &Path,
) -> Result<()> {
    let receipt_bytes = read_regular_no_symlink(Path::new(&request.completed_recovery_receipt))?;
    let receipt_digest = blake3::hash(&receipt_bytes).to_hex().to_string();
    if receipt_digest != request.expected_recovery_receipt_digest {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed recovery receipt digest does not match cleanup authority",
        ));
    }
    let receipt: CompletedRecoveryReceipt = serde_json::from_slice(&receipt_bytes)?;
    if receipt.schema != "csdlc.completed_recovery_receipt.v1"
        || receipt.issue != request.terminal_issue
        || receipt.state != "completed"
        || receipt.terminal_digest != request.expected_terminal_digest
        || receipt.canonical_archive_manifest_digest != request.expected_archive_manifest_digest
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed recovery receipt does not bind terminal and archive authority",
        ));
    }

    let manifest_bytes = read_regular_no_symlink(Path::new(&request.canonical_archive_manifest))?;
    let manifest_digest = blake3::hash(&manifest_bytes).to_hex().to_string();
    if manifest_digest != request.expected_archive_manifest_digest {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical archive manifest digest does not match cleanup authority",
        ));
    }
    let manifest: CanonicalArchiveManifest = serde_json::from_slice(&manifest_bytes)?;
    if manifest.schema != "csdlc.canonical_archive_manifest.v1"
        || manifest.issue != request.terminal_issue
        || manifest.terminal_digest != request.expected_terminal_digest
        || Path::new(&manifest.archived_root) != archived_root
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical archive manifest does not bind cleanup root and recovery authority",
        ));
    }
    let mut request_nodes = BTreeSet::new();
    for node in &request.nodes {
        ensure_safe_relative(&node.relative_path)?;
        if node.identity.node_type == CleanupNodeType::RegularFile && node.identity.links != 1 {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "cleanup authority refuses hardlinked archived files",
            ));
        }
        if !request_nodes.insert((&node.relative_path, &node.identity)) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "cleanup request contains duplicate authority nodes",
            ));
        }
    }
    let mut manifest_nodes = BTreeSet::new();
    for node in &manifest.nodes {
        ensure_safe_relative(&node.relative_path)?;
        if node.identity.node_type == CleanupNodeType::RegularFile && node.identity.links != 1 {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "canonical archive manifest refuses hardlinked archived files",
            ));
        }
        if !manifest_nodes.insert((&node.relative_path, &node.identity)) {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "canonical archive manifest contains duplicate authority nodes",
            ));
        }
    }
    if request_nodes != manifest_nodes {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "cleanup nodes do not match completed recovery archive manifest",
        ));
    }
    Ok(())
}

fn receipt(
    request: &ArchivedProjectionCleanupRequest,
    dir: &Path,
    seq: u32,
    state: &str,
    payload: &Value,
) -> Result<String> {
    let path = receipt_path(dir, seq, state);
    let previous = if seq == 1 {
        None
    } else {
        previous_receipt_digest(dir, seq)?
    };
    let envelope = json!({
        "schema":"csdlc.archived_projection_cleanup_receipt.v1",
        "sequence":seq,
        "state":state,
        "previous_receipt_digest":previous,
        "payload":payload,
    });
    let mut bytes = serde_json::to_vec_pretty(&envelope)?;
    bytes.push(b'\n');
    let failpoint_prefix = state.replace('-', "_");
    failpoint(request, &format!("receipt_{failpoint_prefix}_before_temp"))?;
    if path.exists() {
        let existing = read_regular_no_symlink(&path)?;
        if existing == bytes {
            return Ok(blake3::hash(&existing).to_hex().to_string());
        }
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt already exists with different contents",
        ));
    }
    let tmp = path.with_extension("json.tmp");
    if tmp.exists() {
        let existing = read_regular_no_symlink(&tmp)?;
        if existing.is_empty() {
            fs::remove_file(&tmp)?;
            sync_directory(dir)?;
        } else if existing == bytes || temp_receipt_matches(&existing, seq, state, payload)? {
            fs::rename(&tmp, &path)?;
            sync_directory(dir)?;
            return Ok(blake3::hash(&existing).to_hex().to_string());
        } else {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup receipt temporary file is incomplete or corrupt",
            ));
        }
    }
    {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        let mut file = options.open(&tmp)?;
        failpoint(request, &format!("receipt_{failpoint_prefix}_temp_created"))?;
        file.write_all(&bytes)?;
        file.flush()?;
        failpoint(request, &format!("receipt_{failpoint_prefix}_written"))?;
        file.sync_all()?;
        failpoint(request, &format!("receipt_{failpoint_prefix}_fsynced"))?;
    }
    fs::rename(&tmp, &path)?;
    failpoint(request, &format!("receipt_{failpoint_prefix}_renamed"))?;
    sync_directory(dir)?;
    failpoint(
        request,
        &format!("receipt_{failpoint_prefix}_parent_fsynced"),
    )?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn temp_receipt_matches(bytes: &[u8], seq: u32, state: &str, payload: &Value) -> Result<bool> {
    let value: Value = serde_json::from_slice(bytes)?;
    Ok(value.get("schema").and_then(Value::as_str)
        == Some("csdlc.archived_projection_cleanup_receipt.v1")
        && value.get("sequence").and_then(Value::as_u64) == Some(seq as u64)
        && value.get("state").and_then(Value::as_str) == Some(state)
        && receipt_payload_matches(state, value.get("payload"), payload))
}

fn receipt_payload_matches(state: &str, existing: Option<&Value>, expected: &Value) -> bool {
    if existing == Some(expected) {
        return true;
    }
    let Some(existing) = existing else {
        return false;
    };
    match state {
        "captured" => {
            existing.get("path") == expected.get("path")
                && existing.get("expected_identity") == expected.get("expected_identity")
        }
        "removed" => {
            existing.get("path") == expected.get("path")
                && existing.get("removed_identity") == expected.get("removed_identity")
        }
        "placeholder-disposed" => {
            existing.get("path") == expected.get("path")
                && existing.get("placeholder_identity") == expected.get("placeholder_identity")
        }
        _ => false,
    }
}

fn previous_receipt_digest(dir: &Path, seq: u32) -> Result<Option<String>> {
    let mut entries = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let number = name
                .split_once('-')
                .and_then(|(prefix, _)| prefix.parse::<u32>().ok())?;
            (number < seq).then_some((number, entry.path()))
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|(number, _)| *number);
    let Some((_, path)) = entries.pop() else {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt predecessor is missing or ambiguous",
        ));
    };
    Ok(Some(
        blake3::hash(&read_regular_no_symlink(&path)?)
            .to_hex()
            .to_string(),
    ))
}

fn receipt_payload<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let bytes = read_regular_no_symlink(path)?;
    let value: Value = serde_json::from_slice(&bytes)?;
    if value.get("schema").and_then(Value::as_str)
        != Some("csdlc.archived_projection_cleanup_receipt.v1")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt schema is invalid",
        ));
    }
    serde_json::from_value(value.get("payload").cloned().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt payload is missing",
        )
    })?)
    .map_err(Into::into)
}

fn receipt_path(dir: &Path, seq: u32, state: &str) -> PathBuf {
    dir.join(format!("{seq:03}-{state}.json"))
}

fn failpoint(request: &ArchivedProjectionCleanupRequest, state: &str) -> Result<()> {
    if request.fail_after.as_deref() == Some(state) {
        Err(V2Error::new(
            ErrorCode::ValidationFailed,
            format!("injected cleanup failpoint after {state}"),
        ))
    } else {
        Ok(())
    }
}

fn observe_existing(path: &Path) -> Result<CleanupNodeIdentity> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "cleanup refuses symlink nodes",
        ));
    }
    let node_type = if metadata.is_file() {
        CleanupNodeType::RegularFile
    } else if metadata.is_dir() {
        CleanupNodeType::Directory
    } else {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "cleanup refuses unsupported node type",
        ));
    };
    Ok(CleanupNodeIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
        links: metadata.nlink(),
        uid: metadata.uid(),
        gid: metadata.gid(),
        mode: metadata.permissions().mode(),
        node_type,
    })
}

fn create_placeholder(path: &Path, node_type: CleanupNodeType) -> Result<()> {
    if path.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "private placeholder path already exists",
        ));
    }
    match node_type {
        CleanupNodeType::RegularFile => {
            let file = OpenOptions::new().write(true).create_new(true).open(path)?;
            file.sync_all()?;
        }
        CleanupNodeType::Directory => {
            fs::create_dir(path)?;
            set_private_permissions(path)?;
        }
    }
    sync_directory(
        path.parent()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "placeholder has no parent"))?,
    )?;
    Ok(())
}

fn exchange_paths(source: &Path, destination: &Path) -> Result<()> {
    let source_parent = open_directory(
        source
            .parent()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "source path has no parent"))?,
    )?;
    let destination_parent =
        open_directory(destination.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "destination path has no parent")
        })?)?;
    let source_name = c_name(source)?;
    let destination_name = c_name(destination)?;
    exchange_at(
        &source_parent,
        &source_name,
        &destination_parent,
        &destination_name,
    )
}

#[cfg(target_os = "macos")]
fn exchange_at(
    source_parent: &File,
    source_name: &CString,
    destination_parent: &File,
    destination_name: &CString,
) -> Result<()> {
    let result = unsafe {
        libc::renameatx_np(
            source_parent.as_raw_fd(),
            source_name.as_ptr(),
            destination_parent.as_raw_fd(),
            destination_name.as_ptr(),
            libc::RENAME_SWAP,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(target_os = "linux")]
fn exchange_at(
    source_parent: &File,
    source_name: &CString,
    destination_parent: &File,
    destination_name: &CString,
) -> Result<()> {
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source_parent.as_raw_fd(),
            source_name.as_ptr(),
            destination_parent.as_raw_fd(),
            destination_name.as_ptr(),
            libc::RENAME_EXCHANGE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn exchange_at(
    _source_parent: &File,
    _source_name: &CString,
    _destination_parent: &File,
    _destination_name: &CString,
) -> Result<()> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "no proven atomic exchange primitive on this platform",
    ))
}

fn c_name(path: &Path) -> Result<CString> {
    CString::new(
        path.file_name()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "path has no final component"))?
            .as_bytes(),
    )
    .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path contains NUL"))
}

fn open_directory(path: &Path) -> Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW);
    }
    Ok(options.open(path)?)
}

fn require_same_device(left: &Path, right: &Path, label: &str) -> Result<()> {
    let left_device = fs::symlink_metadata(left)?.dev();
    let right_device = fs::symlink_metadata(right)?.dev();
    if left_device != right_device {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} must be on the archived projection device for atomic exchange"),
        ));
    }
    Ok(())
}

fn sync_directory(path: &Path) -> Result<()> {
    open_directory(path)?.sync_all()?;
    Ok(())
}

fn set_private_permissions(path: &Path) -> Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
}

fn canonical_real_directory(path: &Path, label: &str) -> Result<PathBuf> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("{label} must be a real directory"),
        ));
    }
    Ok(fs::canonicalize(path)?)
}

fn canonical_or_create_directory(path: &Path, label: &str) -> Result<PathBuf> {
    fs::create_dir_all(path)?;
    canonical_real_directory(path, label)
}

fn safe_child(root: &Path, relative: &str) -> Result<PathBuf> {
    ensure_safe_relative(relative)?;
    Ok(root.join(relative))
}

fn ensure_safe_relative(relative: &str) -> Result<()> {
    if relative.trim().is_empty()
        || Path::new(relative).is_absolute()
        || !Path::new(relative)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "cleanup path must be a safe relative path",
        ));
    }
    Ok(())
}

fn ensure_private_component(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.contains('/')
        || value.contains('\\')
        || value == "."
        || value == ".."
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("{label} must be one private path component"),
        ));
    }
    Ok(())
}

fn relative_to(root: &Path, path: &Path) -> Result<String> {
    path.strip_prefix(root)
        .map(|value| value.to_string_lossy().into_owned())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path is outside root"))
}

fn read_regular_no_symlink(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "expected regular non-symlink file",
        ));
    }
    Ok(fs::read(path)?)
}
