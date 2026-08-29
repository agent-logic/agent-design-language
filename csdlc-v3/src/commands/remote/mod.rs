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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDeliveryInput {
    pub pvf: AcceptedPvfResult,
    pub review: ReviewRecord,
    pub publication: PublicationRequest,
    pub pull_request: PullRequestReadback,
    pub issue: IssueReadback,
    pub cleanup: CleanupCandidate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteDeliveryResult {
    pub authorization: PublicationAuthorization,
    pub publication: PublicationEvidence,
    pub finish: FinishClassification,
    pub cleanup: CleanupClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteDeliveryRejectReason {
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
    if input.pvf.evidence_digest.trim().is_empty() {
        return Err(RemoteDeliveryRejectReason::PvfEvidenceMissing);
    }
    if input.pvf.revision != input.review.reviewed_revision {
        return Err(RemoteDeliveryRejectReason::PvfRevisionMismatch);
    }
    let target = ReviewTarget {
        repository: input.publication.repository.clone(),
        issue: input.pvf.issue,
        mode: input.publication.mode,
    };
    let authorization = authorize_publication(
        &input.review,
        &input.pvf.revision,
        &input.publication.publisher,
        target,
    )
    .map_err(RemoteDeliveryRejectReason::Review)?;
    let publication = publish(input.publication, &authorization)
        .map_err(RemoteDeliveryRejectReason::Publication)?;
    let finish = derive_finish(&publication, &input.pull_request, &input.issue);
    if !matches!(finish, FinishClassification::TerminalClosedOut { .. }) {
        return Err(RemoteDeliveryRejectReason::FinishNotTerminal(finish));
    }
    let cleanup = classify_cleanup(&input.cleanup).map_err(RemoteDeliveryRejectReason::Cleanup)?;
    Ok(RemoteDeliveryResult {
        authorization,
        publication,
        finish,
        cleanup,
    })
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
