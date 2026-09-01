#![allow(dead_code)]

use crate::publication::{
    classify_cleanup, derive_finish, publish, CleanupCandidate, CleanupClassification,
    FinishClassification, IssueReadback, PublicationEvidence, PublicationMode, PublicationRequest,
    PullRequestReadback,
};
use crate::review::{
    authorize_publication, PublicationAuthorization, ReviewFinding, ReviewRecord,
    ReviewRejectReason, ReviewTarget,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedPvfResult {
    pub issue: u64,
    pub revision: String,
    pub evidence_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritySource {
    Pvf,
    Review,
    PublicationIntent,
    GithubReadback,
    WorktreeInspection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityReceipt {
    source: AuthoritySource,
    digest: String,
    subject_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verified<T> {
    value: T,
    receipt: AuthorityReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationRejectReason {
    MissingReceipt,
    WrongSource,
}

pub trait VerifiableSubject {
    fn subject_digest(&self) -> String;
}

impl<T: VerifiableSubject> Verified<T> {
    pub(crate) fn new(
        value: T,
        receipt: AuthorityReceipt,
    ) -> Result<Self, VerificationRejectReason> {
        let subject_digest = value.subject_digest();
        if receipt.digest != authority_receipt_digest(receipt.source, &subject_digest)
            || receipt.subject_digest != subject_digest
            || receipt.subject_digest.trim().is_empty()
        {
            return Err(VerificationRejectReason::MissingReceipt);
        }
        Ok(Self { value, receipt })
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    fn require_source(self, source: AuthoritySource) -> Result<T, VerificationRejectReason> {
        if self.receipt.source != source {
            return Err(VerificationRejectReason::WrongSource);
        }
        Ok(self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDeliveryInput {
    pvf: Verified<AcceptedPvfResult>,
    review: Verified<ReviewRecord>,
    publication: Verified<PublicationRequest>,
    pull_request: Verified<PullRequestReadback>,
    issue: Verified<IssueReadback>,
    cleanup: Verified<CleanupCandidate>,
}

impl RemoteDeliveryInput {
    pub(crate) fn new(
        pvf: Verified<AcceptedPvfResult>,
        review: Verified<ReviewRecord>,
        publication: Verified<PublicationRequest>,
        pull_request: Verified<PullRequestReadback>,
        issue: Verified<IssueReadback>,
        cleanup: Verified<CleanupCandidate>,
    ) -> Self {
        Self {
            pvf,
            review,
            publication,
            pull_request,
            issue,
            cleanup,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDeliveryResult {
    pub authorization: PublicationAuthorization,
    pub publication: PublicationEvidence,
    pub finish: FinishClassification,
    pub cleanup: Option<CleanupClassification>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteDeliveryRejectReason {
    Verification(VerificationRejectReason),
    PvfEvidenceMissing,
    PvfRevisionMismatch,
    Review(ReviewRejectReason),
    Publication(crate::publication::PublicationRejectReason),
    FinishNotTerminal(FinishClassification),
    Cleanup(crate::publication::CleanupRejectReason),
}

pub(crate) fn deliver(
    input: RemoteDeliveryInput,
) -> Result<RemoteDeliveryResult, RemoteDeliveryRejectReason> {
    let pvf = input
        .pvf
        .require_source(AuthoritySource::Pvf)
        .map_err(RemoteDeliveryRejectReason::Verification)?;
    let review = input
        .review
        .require_source(AuthoritySource::Review)
        .map_err(RemoteDeliveryRejectReason::Verification)?;
    let publication_request = input
        .publication
        .require_source(AuthoritySource::PublicationIntent)
        .map_err(RemoteDeliveryRejectReason::Verification)?;
    let pull_request = input
        .pull_request
        .require_source(AuthoritySource::GithubReadback)
        .map_err(RemoteDeliveryRejectReason::Verification)?;
    let issue = input
        .issue
        .require_source(AuthoritySource::GithubReadback)
        .map_err(RemoteDeliveryRejectReason::Verification)?;
    let cleanup_candidate = input
        .cleanup
        .require_source(AuthoritySource::WorktreeInspection)
        .map_err(RemoteDeliveryRejectReason::Verification)?;

    if pvf.evidence_digest.trim().is_empty() {
        return Err(RemoteDeliveryRejectReason::PvfEvidenceMissing);
    }
    if pvf.revision != review.reviewed_revision {
        return Err(RemoteDeliveryRejectReason::PvfRevisionMismatch);
    }
    let target = ReviewTarget {
        repository: publication_request.repository.clone(),
        issue: pvf.issue,
        mode: publication_request.mode,
    };
    let authorization = authorize_publication(
        &review,
        &pvf.revision,
        &publication_request.publisher,
        target,
    )
    .map_err(RemoteDeliveryRejectReason::Review)?;
    let publication = publish(publication_request, &authorization)
        .map_err(RemoteDeliveryRejectReason::Publication)?;
    let finish = derive_finish(&publication, &pull_request, &issue);
    let cleanup = match finish {
        FinishClassification::TerminalClosedOut { .. } => Some(
            classify_cleanup(&cleanup_candidate).map_err(RemoteDeliveryRejectReason::Cleanup)?,
        ),
        FinishClassification::CheckpointCompleted { .. } => None,
        FinishClassification::OperatorRequired { .. } => {
            return Err(RemoteDeliveryRejectReason::FinishNotTerminal(finish));
        }
    };
    Ok(RemoteDeliveryResult {
        authorization,
        publication,
        finish,
        cleanup,
    })
}

pub(crate) fn receipt(source: AuthoritySource, subject_digest: &str) -> AuthorityReceipt {
    AuthorityReceipt {
        source,
        digest: authority_receipt_digest(source, subject_digest),
        subject_digest: subject_digest.to_owned(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedReviewEvidence {
    pub issue: u64,
    pub reviewed_revision: String,
    pub scope_paths: Vec<String>,
    pub implementer: String,
    pub reviewer: String,
    pub findings: Vec<ReviewFinding>,
    pub target: ReviewTarget,
    pub typed_review_evidence_digest: String,
}

pub(crate) fn review_from_accepted_evidence(
    evidence: AcceptedReviewEvidence,
) -> Result<ReviewRecord, VerificationRejectReason> {
    if evidence.typed_review_evidence_digest.trim().is_empty()
        || evidence.scope_paths.is_empty()
        || evidence.reviewed_revision.trim().is_empty()
    {
        return Err(VerificationRejectReason::MissingReceipt);
    }
    Ok(ReviewRecord {
        issue: evidence.issue,
        reviewed_revision: evidence.reviewed_revision,
        scope_digest: stable_digest(
            &evidence
                .scope_paths
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        ),
        implementer: evidence.implementer,
        reviewer: evidence.reviewer,
        findings: evidence.findings,
        target: evidence.target,
        evidence_digest: evidence.typed_review_evidence_digest,
    })
}

#[cfg(test)]
pub(crate) fn accepted_review(
    issue: u64,
    revision: &str,
    implementer: &str,
    reviewer: &str,
    mode: PublicationMode,
) -> ReviewRecord {
    review_from_accepted_evidence(AcceptedReviewEvidence {
        issue,
        reviewed_revision: revision.to_owned(),
        scope_paths: vec![
            "csdlc-v3/src/commands/remote".to_owned(),
            "csdlc-v3/src/review".to_owned(),
            "csdlc-v3/src/publication".to_owned(),
            "csdlc-v3/tests/remote_commands".to_owned(),
        ],
        implementer: implementer.to_owned(),
        reviewer: reviewer.to_owned(),
        findings: vec![crate::review::ReviewFinding {
            id: "review-clean".to_owned(),
            disposition: crate::review::FindingDisposition::Resolved,
        }],
        target: ReviewTarget {
            repository: "agent-logic/agent-design-language".to_owned(),
            issue,
            mode,
        },
        typed_review_evidence_digest: stable_digest(&["typed-review-evidence", revision]),
    })
    .expect("accepted typed review evidence")
}

fn stable_digest(values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().to_hex().to_string()
}

fn authority_receipt_digest(source: AuthoritySource, subject_digest: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(format!("{source:?}").as_bytes());
    hasher.update(b"\0");
    hasher.update(subject_digest.as_bytes());
    hasher.finalize().to_hex().to_string()
}

impl VerifiableSubject for AcceptedPvfResult {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            &self.issue.to_string(),
            &self.revision,
            &self.evidence_digest,
        ])
    }
}

impl VerifiableSubject for ReviewRecord {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            &self.issue.to_string(),
            &self.reviewed_revision,
            &self.scope_digest,
            &self.implementer,
            &self.reviewer,
            &self.evidence_digest,
        ])
    }
}

impl VerifiableSubject for PublicationRequest {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            &self.repository,
            &self.issue.to_string(),
            &self.pull_request.to_string(),
            match self.mode {
                PublicationMode::Closing => "closing",
                PublicationMode::PartOf => "part_of",
            },
            &self.publisher,
            &self.body,
            &self.head_sha,
        ])
    }
}

impl VerifiableSubject for PullRequestReadback {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            &self.repository,
            &self.number.to_string(),
            &self.head_sha,
            if self.merged { "merged" } else { "open" },
            &self.closes_issue.map(|v| v.to_string()).unwrap_or_default(),
            &self
                .part_of_issue
                .map(|v| v.to_string())
                .unwrap_or_default(),
        ])
    }
}

impl VerifiableSubject for IssueReadback {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            &self.repository,
            &self.issue.to_string(),
            if self.open { "open" } else { "closed" },
        ])
    }
}

impl VerifiableSubject for CleanupCandidate {
    fn subject_digest(&self) -> String {
        stable_digest(&[
            if self.preview { "preview" } else { "remove" },
            if self.preview_receipt {
                "preview_receipt"
            } else {
                "no_preview_receipt"
            },
            if self.committed_closed_out {
                "closed_out"
            } else {
                "not_closed_out"
            },
            if self.terminal_receipt {
                "terminal_receipt"
            } else {
                "no_terminal_receipt"
            },
            &self.approved_worktree_parent.to_string_lossy(),
            &self.registration.repository_root.to_string_lossy(),
            &self.registration.worktree_path.to_string_lossy(),
            &self.registration.git_common_dir.to_string_lossy(),
            &self.candidate_path.to_string_lossy(),
            &self.registration_digest,
            self.preview_identity_digest.as_deref().unwrap_or_default(),
            if self.dirty { "dirty" } else { "clean" },
            if self.live { "live" } else { "not_live" },
        ])
    }
}

#[cfg(test)]
mod tests;
