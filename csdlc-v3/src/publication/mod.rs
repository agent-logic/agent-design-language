use crate::review::PublicationAuthorization;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationMode {
    Closing,
    PartOf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationRequest {
    pub repository: String,
    pub issue: u64,
    pub mode: PublicationMode,
    pub publisher: String,
    pub body: String,
    pub head_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationEvidence {
    pub repository: String,
    pub issue: u64,
    pub mode: PublicationMode,
    pub head_sha: String,
    pub authorization_digest: String,
    pub relation: PublicationRelation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationRelation {
    Closes,
    PartOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationRejectReason {
    AuthorizationMismatch,
    MissingClosingRelation,
    ClosingModeHasNonClosingRelation,
    PartOfModeHasClosingRelation,
    MissingPartOfRelation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PullRequestReadback {
    pub repository: String,
    pub number: u64,
    pub head_sha: String,
    pub merged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueReadback {
    pub repository: String,
    pub issue: u64,
    pub open: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishClassification {
    TerminalClosedOut {
        pull_request: u64,
        issue: u64,
        head_sha: String,
    },
    CheckpointCompleted {
        pull_request: u64,
        issue: u64,
        invalidates_review_and_publication: bool,
    },
    OperatorRequired {
        reason: FinishRejectReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishRejectReason {
    PullRequestNotMerged,
    HeadMismatch,
    RepositoryMismatch,
    IssueMismatch,
    PartOfParentClosed,
    ClosingParentStillOpen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupCandidate {
    pub preview: bool,
    pub preview_receipt: bool,
    pub committed_closed_out: bool,
    pub terminal_receipt: bool,
    pub registered_worktree: PathBuf,
    pub candidate_path: PathBuf,
    pub dirty: bool,
    pub live: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanupClassification {
    PreviewEligible { path: PathBuf },
    RemoveEligible { path: PathBuf },
    Removed { path: PathBuf },
    AlreadyRemoved { path: PathBuf },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupRejectReason {
    NotTerminal,
    MissingReceipt,
    MissingPreviewReceipt,
    NonCanonicalPath,
    PathMismatch,
    DirtyWorktree,
    LiveWorktree,
    UnregisteredWorktree,
}

pub fn publish(
    request: PublicationRequest,
    authorization: &PublicationAuthorization,
) -> Result<PublicationEvidence, PublicationRejectReason> {
    if request.repository != authorization.target.repository
        || request.issue != authorization.target.issue
        || request.mode != authorization.target.mode
        || request.head_sha != authorization.reviewed_revision
    {
        return Err(PublicationRejectReason::AuthorizationMismatch);
    }
    let closes = has_closing_relation(&request.body, request.issue);
    let part_of = has_part_of_relation(&request.body, request.issue);
    match request.mode {
        PublicationMode::Closing => {
            if !closes {
                return Err(PublicationRejectReason::MissingClosingRelation);
            }
            if part_of {
                return Err(PublicationRejectReason::ClosingModeHasNonClosingRelation);
            }
            Ok(evidence(
                request,
                authorization,
                PublicationRelation::Closes,
            ))
        }
        PublicationMode::PartOf => {
            if closes {
                return Err(PublicationRejectReason::PartOfModeHasClosingRelation);
            }
            if !part_of {
                return Err(PublicationRejectReason::MissingPartOfRelation);
            }
            Ok(evidence(
                request,
                authorization,
                PublicationRelation::PartOf,
            ))
        }
    }
}

pub fn derive_finish(
    publication: &PublicationEvidence,
    pull_request: &PullRequestReadback,
    issue: &IssueReadback,
) -> FinishClassification {
    if pull_request.repository != publication.repository
        || issue.repository != publication.repository
    {
        return operator_required(FinishRejectReason::RepositoryMismatch);
    }
    if issue.issue != publication.issue {
        return operator_required(FinishRejectReason::IssueMismatch);
    }
    if pull_request.head_sha != publication.head_sha {
        return operator_required(FinishRejectReason::HeadMismatch);
    }
    if !pull_request.merged {
        return operator_required(FinishRejectReason::PullRequestNotMerged);
    }
    match publication.mode {
        PublicationMode::Closing if issue.open => {
            operator_required(FinishRejectReason::ClosingParentStillOpen)
        }
        PublicationMode::Closing => FinishClassification::TerminalClosedOut {
            pull_request: pull_request.number,
            issue: issue.issue,
            head_sha: pull_request.head_sha.clone(),
        },
        PublicationMode::PartOf if issue.open => FinishClassification::CheckpointCompleted {
            pull_request: pull_request.number,
            issue: issue.issue,
            invalidates_review_and_publication: true,
        },
        PublicationMode::PartOf => operator_required(FinishRejectReason::PartOfParentClosed),
    }
}

pub fn classify_cleanup(
    candidate: &CleanupCandidate,
) -> Result<CleanupClassification, CleanupRejectReason> {
    if !candidate.committed_closed_out {
        return Err(CleanupRejectReason::NotTerminal);
    }
    if !candidate.terminal_receipt {
        return Err(CleanupRejectReason::MissingReceipt);
    }
    if !is_canonical_absolute(&candidate.registered_worktree)
        || !is_canonical_absolute(&candidate.candidate_path)
    {
        return Err(CleanupRejectReason::NonCanonicalPath);
    }
    if !same_canonical_path(&candidate.registered_worktree, &candidate.candidate_path) {
        return Err(CleanupRejectReason::PathMismatch);
    }
    if candidate.dirty {
        return Err(CleanupRejectReason::DirtyWorktree);
    }
    if candidate.live {
        return Err(CleanupRejectReason::LiveWorktree);
    }
    if candidate.preview {
        Ok(CleanupClassification::PreviewEligible {
            path: candidate.candidate_path.clone(),
        })
    } else if !candidate.preview_receipt {
        Err(CleanupRejectReason::MissingPreviewReceipt)
    } else {
        Ok(CleanupClassification::RemoveEligible {
            path: candidate.candidate_path.clone(),
        })
    }
}

pub fn execute_cleanup_removal(
    candidate: &CleanupCandidate,
) -> Result<CleanupClassification, CleanupRejectReason> {
    if !candidate.committed_closed_out {
        return Err(CleanupRejectReason::NotTerminal);
    }
    if !candidate.terminal_receipt {
        return Err(CleanupRejectReason::MissingReceipt);
    }
    if !is_canonical_absolute(&candidate.registered_worktree)
        || !is_canonical_absolute(&candidate.candidate_path)
    {
        return Err(CleanupRejectReason::NonCanonicalPath);
    }
    if candidate.dirty {
        return Err(CleanupRejectReason::DirtyWorktree);
    }
    if candidate.live {
        return Err(CleanupRejectReason::LiveWorktree);
    }
    if !candidate.preview && !candidate.preview_receipt {
        return Err(CleanupRejectReason::MissingPreviewReceipt);
    }
    if !candidate.registered_worktree.exists() && !candidate.candidate_path.exists() {
        if !candidate.preview && candidate.registered_worktree == candidate.candidate_path {
            return Ok(CleanupClassification::AlreadyRemoved {
                path: candidate.candidate_path.clone(),
            });
        }
        return Err(CleanupRejectReason::UnregisteredWorktree);
    }
    if !candidate.registered_worktree.exists() || !candidate.candidate_path.exists() {
        return Err(CleanupRejectReason::UnregisteredWorktree);
    }
    let classification = classify_cleanup(candidate)?;
    let CleanupClassification::RemoveEligible { path } = classification else {
        return Ok(classification);
    };
    fs::remove_dir_all(&path).map_err(|_| CleanupRejectReason::UnregisteredWorktree)?;
    Ok(CleanupClassification::Removed { path })
}

fn evidence(
    request: PublicationRequest,
    authorization: &PublicationAuthorization,
    relation: PublicationRelation,
) -> PublicationEvidence {
    PublicationEvidence {
        repository: request.repository,
        issue: request.issue,
        mode: request.mode,
        head_sha: request.head_sha,
        authorization_digest: authorization_digest(authorization),
        relation,
    }
}

fn operator_required(reason: FinishRejectReason) -> FinishClassification {
    FinishClassification::OperatorRequired { reason }
}

fn has_closing_relation(body: &str, issue: u64) -> bool {
    body.lines().any(|line| {
        let line = line.trim();
        line == format!("Closes #{issue}") || line == format!("closes #{issue}")
    })
}

fn has_part_of_relation(body: &str, issue: u64) -> bool {
    body.lines()
        .any(|line| line.trim() == format!("Part-Of #{issue}"))
}

fn authorization_digest(authorization: &PublicationAuthorization) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(authorization.issue.to_string().as_bytes());
    hasher.update(b"\0");
    hasher.update(authorization.reviewed_revision.as_bytes());
    hasher.update(b"\0");
    hasher.update(authorization.review_scope_digest.as_bytes());
    hasher.update(b"\0");
    hasher.update(authorization.reviewer.as_bytes());
    hasher.update(b"\0");
    hasher.update(authorization.publisher.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn same_canonical_path(left: &Path, right: &Path) -> bool {
    let Ok(left) = fs::canonicalize(left) else {
        return false;
    };
    let Ok(right) = fs::canonicalize(right) else {
        return false;
    };
    left == right
}

fn is_canonical_absolute(path: &Path) -> bool {
    path.is_absolute()
        && path.components().all(|component| {
            !matches!(
                component,
                std::path::Component::CurDir | std::path::Component::ParentDir
            )
        })
}
