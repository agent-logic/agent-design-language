use crate::publication::{
    classify_cleanup, derive_finish, publish, CleanupCandidate, CleanupClassification,
    FinishClassification, IssueReadback, PublicationEvidence, PublicationMode, PublicationRequest,
    PullRequestReadback,
};
use crate::review::{
    authorize_publication, FindingDisposition, PublicationAuthorization, ReviewFinding,
    ReviewRecord, ReviewRejectReason, ReviewTarget,
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
    pub source: AuthoritySource,
    pub digest: String,
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

impl<T> Verified<T> {
    pub fn new(value: T, receipt: AuthorityReceipt) -> Result<Self, VerificationRejectReason> {
        if receipt.digest.trim().is_empty() {
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
    pub pvf: Verified<AcceptedPvfResult>,
    pub review: Verified<ReviewRecord>,
    pub publication: Verified<PublicationRequest>,
    pub pull_request: Verified<PullRequestReadback>,
    pub issue: Verified<IssueReadback>,
    pub cleanup: Verified<CleanupCandidate>,
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

pub fn deliver(
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

pub fn receipt(source: AuthoritySource, digest: &str) -> AuthorityReceipt {
    AuthorityReceipt {
        source,
        digest: digest.to_owned(),
    }
}

pub fn accepted_review(
    issue: u64,
    revision: &str,
    implementer: &str,
    reviewer: &str,
    mode: PublicationMode,
) -> ReviewRecord {
    ReviewRecord {
        issue,
        reviewed_revision: revision.to_owned(),
        scope_digest: stable_digest(&[
            "csdlc-v3/src/commands/remote",
            "csdlc-v3/src/review",
            "csdlc-v3/src/publication",
            "csdlc-v3/tests/remote_commands",
        ]),
        implementer: implementer.to_owned(),
        reviewer: reviewer.to_owned(),
        findings: vec![ReviewFinding {
            id: "review-clean".to_owned(),
            disposition: FindingDisposition::Resolved,
        }],
        target: ReviewTarget {
            repository: "agent-logic/agent-design-language".to_owned(),
            issue,
            mode,
        },
    }
}

fn stable_digest(values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().to_hex().to_string()
}
