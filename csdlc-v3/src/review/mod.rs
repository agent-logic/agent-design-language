use crate::publication::PublicationMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRecord {
    pub issue: u64,
    pub reviewed_revision: String,
    pub scope_digest: String,
    pub evidence_digest: String,
    pub implementer: String,
    pub reviewer: String,
    pub findings: Vec<ReviewFinding>,
    pub target: ReviewTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewFinding {
    pub id: String,
    pub disposition: FindingDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingDisposition {
    Resolved,
    NonActionable,
    Actionable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewTarget {
    pub repository: String,
    pub issue: u64,
    pub mode: PublicationMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationAuthorization {
    pub(crate) issue: u64,
    pub(crate) reviewed_revision: String,
    pub(crate) review_scope_digest: String,
    pub(crate) reviewer: String,
    pub(crate) publisher: String,
    pub(crate) target: ReviewTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewRejectReason {
    MissingExactScope,
    StaleReview,
    ActionableFinding,
    SamePrincipal,
    WrongTarget,
}

pub fn authorize_publication(
    review: &ReviewRecord,
    current_revision: &str,
    publisher: &str,
    target: ReviewTarget,
) -> Result<PublicationAuthorization, ReviewRejectReason> {
    if review.scope_digest.trim().is_empty() || review.reviewed_revision.trim().is_empty() {
        return Err(ReviewRejectReason::MissingExactScope);
    }
    if review.reviewed_revision != current_revision {
        return Err(ReviewRejectReason::StaleReview);
    }
    if review
        .findings
        .iter()
        .any(|finding| finding.disposition == FindingDisposition::Actionable)
    {
        return Err(ReviewRejectReason::ActionableFinding);
    }
    if same_principal(&review.implementer, &review.reviewer)
        || same_principal(&review.implementer, publisher)
    {
        return Err(ReviewRejectReason::SamePrincipal);
    }
    if review.target != target {
        return Err(ReviewRejectReason::WrongTarget);
    }
    Ok(PublicationAuthorization {
        issue: review.issue,
        reviewed_revision: review.reviewed_revision.clone(),
        review_scope_digest: review.scope_digest.clone(),
        reviewer: review.reviewer.clone(),
        publisher: publisher.to_owned(),
        target,
    })
}

fn same_principal(left: &str, right: &str) -> bool {
    let left = left.trim();
    let right = right.trim();
    !left.is_empty() && left.eq_ignore_ascii_case(right)
}
