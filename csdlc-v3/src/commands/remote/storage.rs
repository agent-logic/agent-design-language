//! Durable remote intent, receipt and recovery storage.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;

use super::model::*;
use super::support::{
    git_control_dir, github_mutation_intent_digest, github_mutation_operation_digest,
    github_mutation_operation_marker, github_mutation_reconciliation_digest, remote_finding,
    GITHUB_OPERATIONAL_ADAPTER,
};

pub(super) fn github_mutation_receipt_path(
    repo_root: &Path,
    digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for mutation receipts",
        )
    })?;
    Ok(git_dir
        .join("csdlc-v3/remote/mutations")
        .join(format!("{digest}.json")))
}

pub(super) fn github_mutation_intent_path(
    repo_root: &Path,
    digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for mutation intents",
        )
    })?;
    Ok(git_dir
        .join("csdlc-v3/remote/intents")
        .join(format!("{digest}.json")))
}

pub(super) fn github_mutation_recovery_path(
    repo_root: &Path,
    digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for mutation recovery receipts",
        )
    })?;
    Ok(git_dir
        .join("csdlc-v3/remote/recoveries")
        .join(format!("{digest}.json")))
}

pub(super) fn github_mutation_rejected_reuse_path(
    repo_root: &Path,
    digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for rejected-recovery attempt receipts",
        )
    })?;
    Ok(git_dir
        .join("csdlc-v3/remote/rejected-recovery-attempts")
        .join(format!("{digest}.json")))
}

pub(super) fn load_mutation_intent(
    path: &Path,
    operation_digest: &str,
) -> Result<GithubMutationIntent, RemoteRouteFinding> {
    let bytes = fs::read(path).map_err(|_| {
        remote_finding(
            "github_mutation_intent_unreadable",
            "existing durable mutation intent cannot be read",
        )
    })?;
    let intent: GithubMutationIntent = serde_json::from_slice(&bytes).map_err(|_| {
        remote_finding(
            "github_mutation_intent_invalid",
            "existing durable mutation intent is not valid typed JSON",
        )
    })?;
    if !matches!(
        intent.schema.as_str(),
        "csdlc.v3.github_mutation_intent.v1" | "csdlc.v3.github_mutation_intent.v2"
    ) || intent.operation_digest != operation_digest
        || intent.operation_marker != github_mutation_operation_marker(operation_digest)
        || intent.adapter != GITHUB_OPERATIONAL_ADAPTER
        || github_mutation_operation_digest(&intent.request) != operation_digest
    {
        return Err(remote_finding(
            "github_mutation_intent_mismatch",
            "existing durable mutation intent does not bind this exact operation",
        ));
    }
    Ok(intent)
}

pub(super) fn load_mutation_receipt(
    path: &Path,
    operation_digest: &str,
) -> Result<GithubMutationReceipt, RemoteRouteFinding> {
    let bytes = fs::read(path).map_err(|_| {
        remote_finding(
            "github_mutation_receipt_unreadable",
            "existing mutation receipt cannot be read",
        )
    })?;
    let receipt: GithubMutationReceipt = serde_json::from_slice(&bytes).map_err(|_| {
        remote_finding(
            "github_mutation_receipt_invalid",
            "existing mutation receipt is not valid typed JSON",
        )
    })?;
    if receipt.schema != "csdlc.v3.github_mutation_receipt.v2"
        || receipt.operation_digest != operation_digest
        || receipt.readback_digest.is_none()
        || receipt.intent_digest.trim().is_empty()
        || receipt.reconciliation_digest.trim().is_empty()
        || !receipt.authenticated
        || receipt.adapter != GITHUB_OPERATIONAL_ADAPTER
    {
        return Err(remote_finding(
            "github_mutation_receipt_mismatch",
            "existing mutation receipt does not bind final authenticated reconciliation",
        ));
    }
    Ok(receipt)
}

pub(crate) fn repository_scoped_issue_creation_receipt(
    remote: &Path,
    receipt_path: &Path,
    repository: &str,
    assigned_issue: u64,
) -> Result<bool, RemoteRouteFinding> {
    let Some(filename_digest) = receipt_path
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".json"))
    else {
        return Ok(false);
    };
    if filename_digest.is_empty()
        || !filename_digest
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Ok(false);
    }
    let receipt_value: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).map_err(|_| {
            remote_finding(
                "github_mutation_receipt_unreadable",
                "existing mutation receipt cannot be read",
            )
        })?)
        .map_err(|_| {
            remote_finding(
                "github_mutation_receipt_invalid",
                "existing mutation receipt is not valid typed JSON",
            )
        })?;
    let Some(operation_digest) = receipt_value["operation_digest"].as_str() else {
        return Ok(false);
    };
    if operation_digest.is_empty()
        || !operation_digest
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Ok(false);
    }
    let filename_intent_path = remote
        .join("intents")
        .join(format!("{filename_digest}.json"));
    let recorded_intent_path = remote
        .join("intents")
        .join(format!("{operation_digest}.json"));
    let (candidate_digest, intent_path) = if filename_intent_path.is_file() {
        (filename_digest, filename_intent_path)
    } else if filename_digest != operation_digest && recorded_intent_path.is_file() {
        (operation_digest, recorded_intent_path)
    } else {
        return Ok(false);
    };
    let intent = load_mutation_intent(&intent_path, candidate_digest)?;
    if intent.request.repository != repository
        || intent.request.issue != 0
        || !matches!(intent.request.mutation, GithubMutation::IssueCreate { .. })
    {
        return Ok(false);
    }
    if filename_digest != operation_digest {
        return Err(remote_finding(
            "github_mutation_receipt_mismatch",
            "repository-scoped issue creation receipt filename does not bind its operation",
        ));
    }
    let receipt = load_mutation_receipt(receipt_path, operation_digest)?;
    if receipt.intent_digest != github_mutation_intent_digest(&intent)
        || receipt.repository != intent.request.repository
        || receipt.issue == 0
        || receipt.pull_request.is_some()
        || receipt.expected_head_sha != intent.request.expected_head_sha
    {
        return Err(remote_finding(
            "github_mutation_receipt_mismatch",
            "repository-scoped issue creation receipt does not bind its exact native intent",
        ));
    }
    Ok(receipt.issue == assigned_issue)
}

pub(super) fn persist_recovery_receipt(
    repo_root: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
    ready_target: Option<&GithubReadyTarget>,
) -> Result<(), RemoteRouteFinding> {
    let path = ensure_recovery_available(repo_root, operation_digest)?;
    let receipt = GithubMutationRecoveryReceipt {
        schema: "csdlc.v3.github_mutation_recovery.v1".into(),
        operation_digest: operation_digest.into(),
        intent_digest: intent_digest.into(),
        recovery: GithubMutationRecovery::RetryAfterAuthenticatedAbsence,
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request,
        expected_head_sha: request.expected_head_sha.clone(),
        resolved_ready_target: ready_target.cloned(),
    };
    persist_json_create_new(&path, &receipt)
}

pub(super) fn ensure_recovery_available(
    repo_root: &Path,
    operation_digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let path = github_mutation_recovery_path(repo_root, operation_digest)?;
    if path.exists() {
        return Err(remote_finding(
            "github_mutation_recovery_already_consumed",
            "the single authenticated-absence recovery was already consumed",
        ));
    }
    Ok(path)
}

pub(super) fn verify_rejected_recovery_receipt(
    repo_root: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
) -> Result<(), RemoteRouteFinding> {
    let reuse_path = github_mutation_rejected_reuse_path(repo_root, operation_digest)?;
    if reuse_path.exists() {
        let reuse: GithubMutationRejectedReuseReceipt =
            serde_json::from_slice(&fs::read(&reuse_path).map_err(|_| {
                remote_finding(
                    "github_mutation_rejected_reuse_unreadable",
                    "definite-rejection retry reservation cannot be read",
                )
            })?)
            .map_err(|_| {
                remote_finding(
                    "github_mutation_rejected_reuse_invalid",
                    "definite-rejection retry reservation is invalid",
                )
            })?;
        if reuse.schema != "csdlc.v3.github_mutation_rejected_reuse.v1"
            || reuse.operation_digest != operation_digest
            || reuse.intent_digest != intent_digest
            || reuse.repository != request.repository
            || reuse.issue != request.issue
            || reuse.expected_head_sha != request.expected_head_sha
        {
            return Err(remote_finding(
                "github_mutation_rejected_reuse_mismatch",
                "definite-rejection retry reservation does not match the retained operation",
            ));
        }
        return Err(remote_finding(
            "github_mutation_recovery_already_consumed",
            "the definite-rejection compatibility retry was already attempted",
        ));
    }
    let path = github_mutation_recovery_path(repo_root, operation_digest)?;
    let receipt: GithubMutationRecoveryReceipt =
        serde_json::from_slice(&fs::read(&path).map_err(|_| {
            remote_finding(
                "github_mutation_rejected_recovery_missing",
                "definitely rejected recovery requires its retained recovery receipt",
            )
        })?)
        .map_err(|_| {
            remote_finding(
                "github_mutation_rejected_recovery_invalid",
                "definitely rejected recovery receipt is invalid",
            )
        })?;
    if receipt.schema != "csdlc.v3.github_mutation_recovery.v1"
        || receipt.operation_digest != operation_digest
        || receipt.intent_digest != intent_digest
        || receipt.recovery != GithubMutationRecovery::RetryAfterAuthenticatedAbsence
        || receipt.repository != request.repository
        || receipt.issue != request.issue
        || receipt.pull_request != request.pull_request
        || receipt.expected_head_sha != request.expected_head_sha
    {
        return Err(remote_finding(
            "github_mutation_rejected_recovery_mismatch",
            "definitely rejected recovery receipt does not match the retained operation",
        ));
    }
    Ok(())
}

pub(super) fn persist_rejected_recovery_attempt(
    repo_root: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
) -> Result<(), RemoteRouteFinding> {
    verify_rejected_recovery_receipt(repo_root, request, operation_digest, intent_digest)?;
    let receipt = GithubMutationRejectedReuseReceipt {
        schema: "csdlc.v3.github_mutation_rejected_reuse.v1".into(),
        operation_digest: operation_digest.into(),
        intent_digest: intent_digest.into(),
        repository: request.repository.clone(),
        issue: request.issue,
        expected_head_sha: request.expected_head_sha.clone(),
    };
    persist_json_create_new(
        &github_mutation_rejected_reuse_path(repo_root, operation_digest)?,
        &receipt,
    )
}

/// Verify that a retained ordinary issue mutation is the settled result of the
/// exact native intent stored beside it. Legacy semantic activation may retain
/// these records, but must never infer authority from an orphan receipt.
pub(crate) fn settled_issue_mutation_receipt(
    remote: &Path,
    receipt_path: &Path,
    repository: &str,
    issue: u64,
) -> Result<bool, RemoteRouteFinding> {
    let Some(operation_digest) = receipt_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
    else {
        return Ok(false);
    };
    let intent_path = remote
        .join("intents")
        .join(format!("{operation_digest}.json"));
    if !intent_path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        return Ok(false);
    }
    let intent = load_mutation_intent(&intent_path, operation_digest)?;
    let receipt = load_mutation_receipt(receipt_path, operation_digest)?;
    if intent.request.repository != repository
        || intent.request.issue != issue
        || intent.request.pull_request.is_some()
        || !matches!(intent.request.mutation, GithubMutation::IssueEdit { .. })
        || receipt.repository != intent.request.repository
        || receipt.issue != intent.request.issue
        || receipt.pull_request != intent.request.pull_request
        || receipt.expected_head_sha != intent.request.expected_head_sha
        || receipt.intent_digest != github_mutation_intent_digest(&intent)
    {
        return Ok(false);
    }
    Ok(true)
}

/// Verify any retained issue-scoped mutation receipt against its exact intent.
/// Legacy semantic activation uses this for historical publication and comment
/// effects as well as ordinary issue edits; orphan or cross-issue receipts are
/// never admitted.
pub(crate) fn settled_issue_scoped_mutation_receipt(
    remote: &Path,
    receipt_path: &Path,
    repository: &str,
    issue: u64,
) -> Result<bool, RemoteRouteFinding> {
    let Some(operation_digest) = receipt_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
    else {
        return Ok(false);
    };
    let intent_path = remote
        .join("intents")
        .join(format!("{operation_digest}.json"));
    if !intent_path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        return Ok(false);
    }
    let intent = load_mutation_intent(&intent_path, operation_digest)?;
    let receipt = load_mutation_receipt(receipt_path, operation_digest)?;
    let pull_request_matches = match &intent.request.mutation {
        GithubMutation::PullRequestCreate { .. } => {
            intent.request.pull_request.is_none()
                && receipt.pull_request.is_some_and(|number| number > 0)
        }
        _ => receipt.pull_request == intent.request.pull_request,
    };
    if intent.request.repository != repository
        || intent.request.issue != issue
        || receipt.repository != intent.request.repository
        || receipt.issue != intent.request.issue
        || !pull_request_matches
        || receipt.expected_head_sha != intent.request.expected_head_sha
        || receipt.intent_digest != github_mutation_intent_digest(&intent)
    {
        return Ok(false);
    }
    Ok(true)
}

/// Authenticate the consumed recovery identity, then require fresh absence.
/// A retry reservation alone says nothing about whether its dispatch succeeded.
/// A retained mutation receipt must be reconciled by the normal recovery owner.
pub(crate) fn authenticated_absence_recovery(
    repo_root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl crate::adapters::ProcessAdapter,
) -> Result<bool, RemoteRouteFinding> {
    let operation_digest = github_mutation_operation_digest(request);
    let intent_path = github_mutation_intent_path(repo_root, &operation_digest)?;
    let recovery_path = github_mutation_recovery_path(repo_root, &operation_digest)?;
    if !intent_path.is_file() || !recovery_path.is_file() {
        return Ok(false);
    }
    let intent = load_mutation_intent(&intent_path, &operation_digest)?;
    let mut original_request = request.clone();
    original_request.recovery = None;
    if intent.request != original_request {
        return Ok(false);
    }
    let recovery: GithubMutationRecoveryReceipt =
        serde_json::from_slice(&fs::read(&recovery_path).map_err(|_| {
            remote_finding(
                "github_mutation_recovery_unreadable",
                "retained recovery receipt cannot be read",
            )
        })?)
        .map_err(|_| {
            remote_finding(
                "github_mutation_recovery_invalid",
                "retained recovery receipt is not valid typed JSON",
            )
        })?;
    let identity_matches = recovery.schema == "csdlc.v3.github_mutation_recovery.v1"
        && recovery.operation_digest == operation_digest
        && recovery.intent_digest == github_mutation_intent_digest(&intent)
        && recovery.recovery == GithubMutationRecovery::RetryAfterAuthenticatedAbsence
        && recovery.repository == request.repository
        && recovery.issue == request.issue
        && recovery.pull_request == request.pull_request
        && recovery.expected_head_sha == request.expected_head_sha;
    if !identity_matches || github_mutation_receipt_path(repo_root, &operation_digest)?.exists() {
        return Ok(false);
    }
    super::transport::observe_publication_absence(request, &operation_digest, process)
}

/// Read-only transition admission. Every retained remote intent must have its
/// exact authenticated completion; repository-scoped creation is included.
pub(crate) fn require_settled_transition_remote(remote: &Path) -> Result<(), RemoteRouteFinding> {
    use super::support::stable_digest;
    let fail = || {
        remote_finding(
            "transition_remote_unsettled",
            "remote intent lacks exact authenticated completion; reconcile before transition",
        )
    };
    let mut admitted = std::collections::BTreeSet::new();
    for namespace in ["intents", "merges"] {
        let directory = remote.join(namespace);
        if !directory.exists() {
            continue;
        }
        for entry in fs::read_dir(&directory).map_err(|_| fail())? {
            let path = entry.map_err(|_| fail())?.path();
            let name = path.file_name().and_then(|s| s.to_str()).ok_or_else(fail)?;
            let digest = if namespace == "merges" {
                let Some(d) = name.strip_suffix(".intent.json") else {
                    continue;
                };
                d
            } else {
                if name.ends_with(".lock") {
                    continue;
                }
                name.strip_suffix(".json").ok_or_else(fail)?
            };
            if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_hexdigit()) {
                return Err(fail());
            }
            let (request, intent_digest) = if namespace == "intents" {
                let intent = load_mutation_intent(&path, digest)?;
                let identity = github_mutation_intent_digest(&intent);
                (intent.request, identity)
            } else {
                let intent: MergeIntent =
                    serde_json::from_slice(&fs::read(&path).map_err(|_| fail())?)
                        .map_err(|_| fail())?;
                if intent.schema != "csdlc.v3.merge_intent.v1"
                    || github_mutation_operation_digest(&intent.request) != digest
                {
                    return Err(fail());
                }
                let identity =
                    stable_digest(&[&serde_json::to_string(&intent).map_err(|_| fail())?]);
                (intent.request, identity)
            };
            let receipt = load_mutation_receipt(
                &remote.join("mutations").join(format!("{digest}.json")),
                digest,
            )?;
            if receipt.intent_digest != intent_digest
                || receipt.repository != request.repository
                || receipt.expected_head_sha != request.expected_head_sha
            {
                return Err(fail());
            }
            admitted.insert(digest.to_owned());
        }
    }
    for namespace in ["mutations", "recoveries"] {
        let directory = remote.join(namespace);
        if !directory.exists() {
            continue;
        }
        for entry in fs::read_dir(directory).map_err(|_| fail())? {
            let path = entry.map_err(|_| fail())?.path();
            let name = path.file_name().and_then(|s| s.to_str()).ok_or_else(fail)?;
            if name.ends_with(".lock") {
                continue;
            }
            if !admitted.contains(name.split('.').next().ok_or_else(fail)?) {
                return Err(fail());
            }
        }
    }
    Ok(())
}

/// Require the exact authenticated native completion that authorizes the
/// narrow legacy coordination terminal compatibility path.
pub(crate) fn settled_coordination_completion_receipt(
    repo_root: &Path,
    repository: &str,
    issue: u64,
    expected_head_sha: &str,
) -> Result<bool, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for mutation receipts",
        )
    })?;
    let remote = git_dir.join("csdlc-v3/remote");
    let receipts = remote.join("mutations");
    let entries = match fs::read_dir(&receipts) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(_) => {
            return Err(remote_finding(
                "github_mutation_receipt_unreadable",
                "existing mutation receipts cannot be inspected",
            ))
        }
    };
    for entry in entries {
        let path = entry
            .map_err(|_| {
                remote_finding(
                    "github_mutation_receipt_unreadable",
                    "existing mutation receipt cannot be inspected",
                )
            })?
            .path();
        let Some(operation_digest) =
            path.file_stem()
                .and_then(|value| value.to_str())
                .filter(|value| {
                    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
                })
        else {
            continue;
        };
        let intent_path = remote
            .join("intents")
            .join(format!("{operation_digest}.json"));
        if !intent_path.is_file() {
            continue;
        }
        let intent = load_mutation_intent(&intent_path, operation_digest)?;
        if intent.request.repository != repository
            || intent.request.issue != issue
            || intent.request.expected_head_sha != expected_head_sha
            || intent.request.pull_request.is_some()
            || !matches!(
                intent.request.mutation,
                GithubMutation::IssueCompleteCoordination { .. }
            )
        {
            continue;
        }
        let receipt = load_mutation_receipt(&path, operation_digest)?;
        if receipt.repository == repository
            && receipt.issue == issue
            && receipt.pull_request.is_none()
            && receipt.expected_head_sha == expected_head_sha
            && receipt.intent_digest == github_mutation_intent_digest(&intent)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(super) fn finalize_mutation_receipt(
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
    response_digest: Option<String>,
    reconciliation: &GithubMutationReconciliationReceipt,
    idempotent_replay: bool,
) -> GithubMutationReceipt {
    GithubMutationReceipt {
        schema: "csdlc.v3.github_mutation_receipt.v2".into(),
        repository: request.repository.clone(),
        issue: reconciliation.issue,
        pull_request: reconciliation.pull_request.or(request.pull_request),
        expected_head_sha: request.expected_head_sha.clone(),
        operation_digest: operation_digest.to_owned(),
        response_digest,
        readback_digest: Some(reconciliation.readback_digest.clone()),
        intent_digest: intent_digest.to_owned(),
        reconciliation_digest: github_mutation_reconciliation_digest(reconciliation),
        adapter: GITHUB_OPERATIONAL_ADAPTER.into(),
        authenticated: true,
        idempotent_replay,
    }
}

pub(super) fn persist_json_create_new(
    path: &Path,
    value: &impl Serialize,
) -> Result<(), RemoteRouteFinding> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| {
        remote_finding(
            "receipt_serialization_failed",
            "typed receipt could not be serialized",
        )
    })?;
    write_private_create_new(path, &bytes)
}

pub(super) fn write_private_create_new(
    path: &Path,
    bytes: &[u8],
) -> Result<(), RemoteRouteFinding> {
    use std::io::Write;
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| remote_finding("receipt_path_invalid", "receipt path has no parent"))?,
    )
    .map_err(|_| {
        remote_finding(
            "receipt_write_failed",
            "receipt parent could not be created",
        )
    })?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|_| remote_finding("receipt_write_failed", "create-only receipt write failed"))?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| {
            remote_finding(
                "receipt_write_failed",
                "receipt could not be durably written",
            )
        })?;
    fs::File::open(path.parent().expect("validated receipt parent"))
        .and_then(|parent| parent.sync_all())
        .map_err(|_| {
            remote_finding(
                "receipt_write_failed",
                "receipt parent directory could not be durably synchronized",
            )
        })
}
