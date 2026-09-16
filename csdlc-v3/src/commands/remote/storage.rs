//! Durable remote intent, receipt and recovery storage.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde::Serialize;

use super::model::*;
use super::support::{
    git_control_dir, github_mutation_intent_digest, github_mutation_operation_digest,
    github_mutation_operation_marker, github_mutation_reconciliation_digest,
    github_mutation_request_digest, remote_finding, GITHUB_OPERATIONAL_ADAPTER,
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

pub(super) fn github_mutation_head_available_recovery_path(
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
        .join("csdlc-v3/remote/head-available-recoveries")
        .join(format!("{digest}.json")))
}

pub(super) fn github_mutation_legacy_non_effect_disposition_path(
    repo_root: &Path,
    digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let git_dir = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required for mutation recovery dispositions",
        )
    })?;
    Ok(git_dir
        .join("csdlc-v3/remote/legacy-non-effect-dispositions")
        .join(format!("{digest}.json")))
}

pub(super) fn admit_legacy_non_effect_disposition(
    repo_root: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
    disposition: Option<&GithubMutationLegacyNonEffectDisposition>,
    source: Option<&CoordinationEvidence>,
) -> Result<(), RemoteRouteFinding> {
    let disposition = disposition.ok_or_else(|| {
        remote_finding(
            "github_mutation_recovery_disposition_missing",
            "the consumed recovery requires a typed definitive non-effect disposition",
        )
    })?;
    let source = source.ok_or_else(|| {
        remote_finding(
            "github_mutation_recovery_disposition_source_missing",
            "the consumed recovery requires a canonical disposition source",
        )
    })?;
    let expected_path = format!("docs/csdlc-v3/recovery-dispositions/{operation_digest}.json");
    if source.path != expected_path
        || source.digest.len() != 64
        || !source.digest.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_source_invalid",
            "the disposition source must be the exact operation-bound canonical path and digest",
        ));
    }
    let object = format!("origin/main:{}", source.path);
    let output = Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(repo_root)
        .args(["show", &object])
        .output()
        .map_err(|_| {
            remote_finding(
                "github_mutation_recovery_disposition_source_unavailable",
                "the canonical disposition source could not be read",
            )
        })?;
    if !output.status.success() || blake3::hash(&output.stdout).to_hex().as_str() != source.digest {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_source_mismatch",
            "the canonical disposition source bytes do not match the approved digest",
        ));
    }
    let approved: GithubMutationLegacyNonEffectDisposition = serde_json::from_slice(&output.stdout)
        .map_err(|_| {
            remote_finding(
                "github_mutation_recovery_disposition_source_invalid",
                "the canonical disposition source is not a typed disposition",
            )
        })?;
    if &approved != disposition {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_source_mismatch",
            "the recovery disposition differs from canonical origin/main bytes",
        ));
    }
    let expected_evidence_prefix = format!(".csdlc/evidence/{}/", disposition.issue);
    if !disposition
        .evidence_path
        .starts_with(&expected_evidence_prefix)
        || disposition.evidence_path.contains("..")
        || disposition.evidence_path.starts_with('/')
    {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_evidence_invalid",
            "the definitive non-effect evidence path is outside the issue evidence boundary",
        ));
    }
    let evidence_object = format!("origin/main:{}", disposition.evidence_path);
    let evidence = Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(repo_root)
        .args(["show", &evidence_object])
        .output()
        .map_err(|_| {
            remote_finding(
                "github_mutation_recovery_disposition_evidence_unavailable",
                "the definitive non-effect evidence could not be read",
            )
        })?;
    if !evidence.status.success()
        || blake3::hash(&evidence.stdout).to_hex().as_str() != disposition.evidence_digest
    {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_evidence_mismatch",
            "the definitive non-effect evidence bytes do not match the approved digest",
        ));
    }
    let retained = load_mutation_intent(
        &github_mutation_intent_path(repo_root, operation_digest)?,
        operation_digest,
    )?;
    let digest =
        |value: &str| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
    if disposition.schema != "csdlc.v3.github_mutation_legacy_non_effect_disposition.v1"
        || disposition.repository != request.repository
        || disposition.issue != request.issue
        || disposition.operation_digest != operation_digest
        || disposition.intent_digest != intent_digest
        || disposition.request_digest != github_mutation_request_digest(request)
        || disposition.authority_selector_digest != retained.authority_selector_digest
        || disposition.expected_head_sha != request.expected_head_sha
        || !digest(&disposition.evidence_digest)
        || disposition.operator.trim().is_empty()
        || disposition.authorization_ref.trim().is_empty()
    {
        return Err(remote_finding(
            "github_mutation_recovery_disposition_mismatch",
            "the typed definitive non-effect disposition does not bind the retained operation",
        ));
    }
    let path = github_mutation_legacy_non_effect_disposition_path(repo_root, operation_digest)?;
    if path.exists() {
        let bytes = fs::read(&path).map_err(|_| {
            remote_finding(
                "github_mutation_recovery_disposition_unreadable",
                "the retained definitive non-effect disposition is unreadable",
            )
        })?;
        let retained: GithubMutationLegacyNonEffectDisposition = serde_json::from_slice(&bytes)
            .map_err(|_| {
                remote_finding(
                    "github_mutation_recovery_disposition_invalid",
                    "the retained definitive non-effect disposition is invalid",
                )
            })?;
        if &retained != disposition {
            return Err(remote_finding(
                "github_mutation_recovery_disposition_mismatch",
                "the retained definitive non-effect disposition differs from this recovery",
            ));
        }
        Ok(())
    } else {
        persist_json_create_new(&path, disposition)
    }
}

pub(super) fn load_mutation_recovery_receipt(
    path: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
) -> Result<GithubMutationRecoveryReceipt, RemoteRouteFinding> {
    let bytes = fs::read(path).map_err(|_| {
        remote_finding(
            "github_mutation_recovery_unreadable",
            "existing recovery receipt cannot be read",
        )
    })?;
    let receipt: GithubMutationRecoveryReceipt = serde_json::from_slice(&bytes).map_err(|_| {
        remote_finding(
            "github_mutation_recovery_invalid",
            "existing recovery receipt is not valid typed JSON",
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
        || receipt.resolved_ready_target.is_some()
    {
        return Err(remote_finding(
            "github_mutation_recovery_mismatch",
            "existing recovery receipt does not bind this exact PR-create operation",
        ));
    }
    Ok(receipt)
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
