//! Shared remote constants, findings and byte-stable digests.

use super::model::{
    GithubMutationIntent, GithubMutationReconciliationReceipt, GithubMutationRequest,
    RemoteRouteFinding,
};
use std::path::{Path, PathBuf};

pub(super) const GITHUB_READ_ONLY_ADAPTER: &str = "github-api-read-only";
pub(super) const GITHUB_OPERATIONAL_ADAPTER: &str = "github-api-operational";
pub(super) const CANONICAL_AUTHORITY_SELECTOR_PATH: &str = crate::authority::SELECTOR_PATH;
pub(super) const GITHUB_OPERATION_MARKER_PREFIX: &str = "csdlc-v3-operation";

pub(super) fn remote_finding(code: &str, message: &str) -> RemoteRouteFinding {
    RemoteRouteFinding {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

pub(super) fn stable_digest(values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().to_hex().to_string()
}

pub fn github_mutation_operation_digest(request: &GithubMutationRequest) -> String {
    let mutation = serde_json::to_string(&request.mutation).unwrap_or_default();
    stable_digest(&[
        "csdlc.v3.github_mutation.v1",
        &request.repository,
        &request.issue.to_string(),
        &request.pull_request.unwrap_or_default().to_string(),
        &request.expected_head_sha,
        &mutation,
    ])
}

pub fn github_mutation_operation_marker(operation_digest: &str) -> String {
    format!("<!-- {GITHUB_OPERATION_MARKER_PREFIX}:{operation_digest} -->")
}

pub(super) fn validate_repository_name(repository: &str) -> Result<(), RemoteRouteFinding> {
    let Some((owner, name)) = repository.split_once('/') else {
        return Err(remote_finding(
            "github_repository_invalid",
            "repository must be owner/name for GitHub readback",
        ));
    };
    if owner.is_empty()
        || name.is_empty()
        || repository
            .chars()
            .any(|ch| !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/')))
    {
        return Err(remote_finding(
            "github_repository_invalid",
            "repository contains characters that cannot be used in structured GitHub readback",
        ));
    }
    Ok(())
}

pub(super) fn is_full_git_sha(value: &str) -> bool {
    value.len() == 40 && value.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

pub(super) fn same_names(actual: &[String], expected: &[String]) -> bool {
    let canonical = |names: &[String]| {
        names
            .iter()
            .map(|name| name.to_lowercase())
            .collect::<std::collections::BTreeSet<_>>()
    };
    canonical(actual) == canonical(expected)
}

pub(super) fn exact_issue_names(value: &serde_json::Value, field: &str) -> Option<Vec<String>> {
    value
        .as_array()?
        .iter()
        .map(|v| v.as_str().or_else(|| v[field].as_str()).map(str::to_owned))
        .collect()
}

pub(super) fn git_control_dir(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return dot_git.canonicalize().ok();
    }
    let contents = std::fs::read_to_string(&dot_git).ok()?;
    let gitdir = contents.strip_prefix("gitdir:")?.trim();
    let path = Path::new(gitdir);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let git_dir = path.canonicalize().ok()?;
    git_common_dir(&git_dir).or(Some(git_dir))
}

pub(super) fn git_common_dir(git_dir: &Path) -> Option<PathBuf> {
    let contents = std::fs::read_to_string(git_dir.join("commondir")).ok()?;
    let path = Path::new(contents.trim());
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        git_dir.join(path)
    };
    path.canonicalize().ok()
}

pub(super) fn github_mutation_intent_digest(intent: &GithubMutationIntent) -> String {
    let mut digest = stable_digest(&[
        &intent.schema,
        &intent.operation_digest,
        &intent.operation_marker,
        &intent.authority_selector_digest,
        &intent.adapter,
    ]);
    if let Some(edit) = &intent.resolved_edit {
        digest = stable_digest(&[&digest, &serde_json::to_string(edit).unwrap_or_default()]);
    }
    if intent.schema == "csdlc.v3.github_mutation_intent.v2" {
        if let Some(target) = &intent.resolved_ready_target {
            digest = stable_digest(&[&digest, &serde_json::to_string(target).unwrap_or_default()]);
        }
    }
    digest
}

pub(super) fn github_mutation_reconciliation_digest(
    reconciliation: &GithubMutationReconciliationReceipt,
) -> String {
    stable_digest(&[
        &reconciliation.schema,
        &reconciliation.operation_digest,
        &reconciliation.operation_marker,
        &reconciliation.repository,
        &reconciliation.issue.to_string(),
        &reconciliation.pull_request.unwrap_or_default().to_string(),
        &reconciliation
            .remote_object_id
            .unwrap_or_default()
            .to_string(),
        &reconciliation.expected_head_sha,
        &reconciliation.readback_digest,
        &reconciliation.observed_by,
        if reconciliation.authenticated {
            "authenticated"
        } else {
            "unauthenticated"
        },
    ])
}
