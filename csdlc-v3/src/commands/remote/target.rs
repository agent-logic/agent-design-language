//! Publication-target inventory for ordinary publication and terminal reconciliation.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use super::model::{
    GithubMutation, GithubMutationRecovery, GithubMutationRecoveryReceipt, RemoteRouteFinding,
};
use super::storage::{
    github_mutation_receipt_path, github_mutation_recovery_path, load_mutation_intent,
    load_mutation_receipt,
};
use super::support::{git_control_dir, github_mutation_intent_digest, remote_finding};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct SettledPublicationIdentity {
    pub pull_request: u64,
    pub branch: String,
    pub head: String,
}

pub(crate) fn settled_publication_identities(
    root: &Path,
    repository: &str,
    issue: u64,
) -> Result<Vec<SettledPublicationIdentity>, RemoteRouteFinding> {
    let control = git_control_dir(root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required",
        )
    })?;
    let directory = control.join("csdlc-v3/remote/intents");
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut identities: BTreeMap<u64, (u8, SettledPublicationIdentity)> = BTreeMap::new();
    for entry in fs::read_dir(directory).map_err(|_| {
        remote_finding(
            "intent_publication_inventory_unreadable",
            "native intent inventory cannot be read",
        )
    })? {
        let path = entry
            .map_err(|_| {
                remote_finding(
                    "intent_publication_inventory_unreadable",
                    "native intent entry cannot be read",
                )
            })?
            .path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let digest = path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                remote_finding(
                    "intent_publication_inventory_invalid",
                    "intent filename is invalid",
                )
            })?;
        let intent = load_mutation_intent(&path, digest)?;
        if intent.request.repository != repository || intent.request.issue != issue {
            continue;
        }
        let (rank, published_branch) = match &intent.request.mutation {
            GithubMutation::PullRequestCreate { head, .. } => (1, head.clone()),
            GithubMutation::PullRequestUpdate { .. } => (2, String::new()),
            GithubMutation::PullRequestReady => (3, String::new()),
            GithubMutation::PullRequestMerge { .. } => (4, String::new()),
            _ => continue,
        };
        let receipt_path = github_mutation_receipt_path(root, digest)?;
        if !receipt_path.exists() {
            continue;
        }
        let receipt = load_mutation_receipt(&receipt_path, digest)?;
        if receipt.intent_digest != github_mutation_intent_digest(&intent)
            || receipt.repository != repository
            || receipt.issue != issue
            || receipt.expected_head_sha != intent.request.expected_head_sha
        {
            return Err(remote_finding(
                "intent_publication_receipt_mismatch",
                "publication receipt does not bind its native intent",
            ));
        }
        let pull_request = receipt
            .pull_request
            .filter(|number| *number > 0)
            .ok_or_else(|| {
                remote_finding(
                    "intent_publication_target_missing",
                    "authenticated publication receipt lacks PR identity",
                )
            })?;
        let identity = SettledPublicationIdentity {
            pull_request,
            branch: published_branch,
            head: intent.request.expected_head_sha.clone(),
        };
        match identities.get(&pull_request) {
            Some((selected_rank, selected)) if *selected_rank == rank && selected >= &identity => {}
            Some((selected_rank, _)) if *selected_rank > rank => {}
            _ => {
                identities.insert(pull_request, (rank, identity));
            }
        }
    }
    Ok(identities
        .into_values()
        .map(|(_, identity)| identity)
        .collect())
}

pub(super) fn publication_target_inventory(
    root: &Path,
    repository: &str,
    issue: u64,
    branch: &str,
    head: &str,
    admit_authenticated_absence: bool,
) -> Result<Vec<u64>, RemoteRouteFinding> {
    let control = git_control_dir(root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required",
        )
    })?;
    let directory = control.join("csdlc-v3/remote/intents");
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let mut targets = BTreeSet::new();
    let mut stale_unresolved_create = false;
    for entry in fs::read_dir(directory).map_err(|_| {
        remote_finding(
            "intent_publication_inventory_unreadable",
            "native intent inventory cannot be read",
        )
    })? {
        let path = entry
            .map_err(|_| {
                remote_finding(
                    "intent_publication_inventory_unreadable",
                    "native intent entry cannot be read",
                )
            })?
            .path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let digest = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
            remote_finding(
                "intent_publication_inventory_invalid",
                "intent filename is invalid",
            )
        })?;
        let intent = load_mutation_intent(&path, digest)?;
        if intent.request.repository != repository || intent.request.issue != issue {
            continue;
        }
        let receipt_path = github_mutation_receipt_path(root, digest)?;
        match &intent.request.mutation {
            GithubMutation::PullRequestCreate {
                head: published_branch,
                ..
            } => {
                if published_branch != branch {
                    return Err(remote_finding(
                        "intent_publication_branch_conflict",
                        "issue has publication intent on another branch",
                    ));
                }
                if !receipt_path.exists() {
                    if intent.request.expected_head_sha != head {
                        let recovery_path = github_mutation_recovery_path(root, digest)?;
                        let reconciled_absence = if admit_authenticated_absence
                            && recovery_path.exists()
                        {
                            let recovery: GithubMutationRecoveryReceipt = serde_json::from_slice(
                                &fs::read(&recovery_path).map_err(|_| {
                                    remote_finding(
                                        "intent_publication_recovery_unreadable",
                                        "retained publication recovery cannot be read",
                                    )
                                })?,
                            )
                            .map_err(|_| {
                                remote_finding(
                                    "intent_publication_recovery_invalid",
                                    "retained publication recovery is invalid",
                                )
                            })?;
                            recovery.schema == "csdlc.v3.github_mutation_recovery.v1"
                                && recovery.operation_digest == digest
                                && recovery.intent_digest == github_mutation_intent_digest(&intent)
                                && recovery.recovery
                                    == GithubMutationRecovery::RetryAfterAuthenticatedAbsence
                                && recovery.repository == repository
                                && recovery.issue == issue
                                && recovery.pull_request.is_none()
                                && recovery.expected_head_sha == intent.request.expected_head_sha
                        } else {
                            false
                        };
                        stale_unresolved_create |= !reconciled_absence;
                    }
                    continue;
                }
            }
            GithubMutation::PullRequestUpdate { .. }
            | GithubMutation::PullRequestReady
            | GithubMutation::PullRequestMerge { .. } => {
                if !receipt_path.exists() {
                    continue;
                }
            }
            _ => continue,
        }
        let receipt = load_mutation_receipt(&receipt_path, digest)?;
        if receipt.intent_digest != github_mutation_intent_digest(&intent)
            || receipt.repository != repository
            || receipt.issue != issue
            || receipt.expected_head_sha != intent.request.expected_head_sha
        {
            return Err(remote_finding(
                "intent_publication_receipt_mismatch",
                "publication receipt does not bind its native intent",
            ));
        }
        let number = receipt.pull_request.filter(|n| *n > 0).ok_or_else(|| {
            remote_finding(
                "intent_publication_target_missing",
                "authenticated publication receipt lacks PR identity",
            )
        })?;
        targets.insert(number);
    }
    if stale_unresolved_create && (admit_authenticated_absence || targets.is_empty()) {
        return Err(remote_finding(
            "intent_publication_uncertain_head",
            "reconcile the retained publication intent before changing its candidate",
        ));
    }
    Ok(targets.into_iter().collect())
}
