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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteCommandMode {
    Closing,
    PartOf,
}

impl From<RemoteCommandMode> for PublicationMode {
    fn from(value: RemoteCommandMode) -> Self {
        match value {
            RemoteCommandMode::Closing => PublicationMode::Closing,
            RemoteCommandMode::PartOf => PublicationMode::PartOf,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteCommandOperation {
    VerifyBridgeEvidence,
    Deliver,
    Finish,
    CleanupPreview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteCommandRequest {
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    pub head_sha: String,
    pub mode: RemoteCommandMode,
    pub operation: RemoteCommandOperation,
    pub pvf_evidence_ref: String,
    pub typed_review_ref: String,
    pub publication_intent_ref: String,
    pub pr_readback_ref: String,
    pub issue_readback_ref: String,
    pub cleanup_inspection_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteCommandStatus {
    ReadyForTypedBridge,
    DeliveryDerived,
    FinishDerived,
    CleanupPreviewDerived,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteCommandReport {
    pub schema: &'static str,
    pub issue: u64,
    pub pull_request: u64,
    pub operation: RemoteCommandOperation,
    pub operational_authority: bool,
    pub trusted_authority: bool,
    pub status: RemoteCommandStatus,
    pub blockers: Vec<String>,
    pub evidence_refs: BTreeMap<&'static str, String>,
    pub evidence_digest: String,
    pub result: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteCommandRejectReason {
    InvalidIdentity,
    CallerForgedAuthority { field: &'static str },
    EvidenceRefEscapesRepo { field: &'static str },
    EvidenceRefMissing { field: &'static str },
    EvidenceRefInvalidJson { field: &'static str },
    EvidenceRefSchemaMissing { field: &'static str },
    EvidenceSchemaMismatch { field: &'static str },
    EvidenceIdentityMismatch { field: &'static str },
    EvidenceDerivationFailed(String),
}

pub trait VerifiableSubject {
    fn subject_digest(&self) -> String;
}

pub fn verify_remote_bridge_request(
    root: &Path,
    request: RemoteCommandRequest,
) -> Result<RemoteCommandReport, RemoteCommandRejectReason> {
    if request.repository.trim().is_empty()
        || request.issue == 0
        || request.pull_request == 0
        || !is_full_sha(&request.head_sha)
    {
        return Err(RemoteCommandRejectReason::InvalidIdentity);
    }
    let refs = [
        ("pvf_evidence_ref", request.pvf_evidence_ref.as_str()),
        ("typed_review_ref", request.typed_review_ref.as_str()),
        (
            "publication_intent_ref",
            request.publication_intent_ref.as_str(),
        ),
        ("pr_readback_ref", request.pr_readback_ref.as_str()),
        ("issue_readback_ref", request.issue_readback_ref.as_str()),
        (
            "cleanup_inspection_ref",
            request.cleanup_inspection_ref.as_str(),
        ),
    ];
    let issue = request.issue.to_string();
    let pull_request = request.pull_request.to_string();
    let mut digest_inputs = vec![
        request.repository.clone(),
        issue,
        pull_request,
        request.head_sha.clone(),
    ];
    let mut evidence_refs = BTreeMap::new();
    let mut evidence = BTreeMap::new();
    let mut evidence_digests = Vec::new();
    for (field, evidence_ref) in refs {
        let verified = validate_evidence_ref(root, field, evidence_ref)?;
        evidence_refs.insert(field, evidence_ref.to_owned());
        digest_inputs.push(evidence_ref.to_owned());
        evidence_digests.push(verified.digest);
        evidence.insert(field, verified.value);
    }
    digest_inputs.extend(evidence_digests);
    let evidence_digest =
        stable_digest(&digest_inputs.iter().map(String::as_str).collect::<Vec<_>>());
    let blockers = vec![
        "v3 remote command is pre-cutover verification only; typed C-SDLC v2 remains operational authority until #505 explicitly switches authority".to_owned(),
    ];
    let (status, result) = derive_remote_command_result(&request, &evidence)?;
    Ok(RemoteCommandReport {
        schema: "csdlc.v3.remote_delivery.v1",
        issue: request.issue,
        pull_request: request.pull_request,
        operation: request.operation,
        operational_authority: false,
        trusted_authority: false,
        status,
        blockers,
        evidence_refs,
        evidence_digest,
        result: Some(result),
    })
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

fn is_full_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Clone)]
struct JsonEvidence {
    value: Value,
    digest: String,
}

fn validate_evidence_ref(
    root: &Path,
    field: &'static str,
    evidence_ref: &str,
) -> Result<JsonEvidence, RemoteCommandRejectReason> {
    if evidence_ref.trim().is_empty()
        || evidence_ref.contains('\0')
        || evidence_ref.contains("://")
        || evidence_ref.starts_with("caller:")
        || evidence_ref.starts_with("inline:")
    {
        return Err(RemoteCommandRejectReason::CallerForgedAuthority { field });
    }
    let path = Path::new(evidence_ref);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::RootDir))
    {
        return Err(RemoteCommandRejectReason::EvidenceRefEscapesRepo { field });
    }
    let Ok(root) = root.canonicalize() else {
        return Err(RemoteCommandRejectReason::EvidenceRefMissing { field });
    };
    let Ok(target) = root.join(path).canonicalize() else {
        return Err(RemoteCommandRejectReason::EvidenceRefMissing { field });
    };
    if !target.starts_with(&root) {
        return Err(RemoteCommandRejectReason::EvidenceRefEscapesRepo { field });
    }
    if !target.is_file() {
        return Err(RemoteCommandRejectReason::EvidenceRefMissing { field });
    }
    let bytes =
        fs::read(&target).map_err(|_| RemoteCommandRejectReason::EvidenceRefMissing { field })?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|_| RemoteCommandRejectReason::EvidenceRefInvalidJson { field })?;
    let Some(schema) = value.get("schema").and_then(Value::as_str) else {
        return Err(RemoteCommandRejectReason::EvidenceRefSchemaMissing { field });
    };
    if schema.trim().is_empty() {
        return Err(RemoteCommandRejectReason::EvidenceRefSchemaMissing { field });
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc-v3.remote-bridge-evidence.v1");
    hasher.update(b"\0");
    hasher.update(field.as_bytes());
    hasher.update(b"\0");
    hasher.update(evidence_ref.as_bytes());
    hasher.update(b"\0");
    hasher.update(schema.as_bytes());
    hasher.update(b"\0");
    hasher.update(&bytes);
    Ok(JsonEvidence {
        value,
        digest: hasher.finalize().to_hex().to_string(),
    })
}

fn derive_remote_command_result(
    request: &RemoteCommandRequest,
    evidence: &BTreeMap<&'static str, Value>,
) -> Result<(RemoteCommandStatus, Value), RemoteCommandRejectReason> {
    let pvf = verified(
        parse_pvf(request, required(evidence, "pvf_evidence_ref")?)?,
        AuthoritySource::Pvf,
    )?;
    let review = verified(
        parse_review(request, required(evidence, "typed_review_ref")?)?,
        AuthoritySource::Review,
    )?;
    let publication = verified(
        parse_publication(request, required(evidence, "publication_intent_ref")?)?,
        AuthoritySource::PublicationIntent,
    )?;
    let pr = verified(
        parse_pr_readback(request, required(evidence, "pr_readback_ref")?)?,
        AuthoritySource::GithubReadback,
    )?;
    let issue = verified(
        parse_issue_readback(request, required(evidence, "issue_readback_ref")?)?,
        AuthoritySource::GithubReadback,
    )?;
    let cleanup = verified(
        parse_cleanup(required(evidence, "cleanup_inspection_ref")?)?,
        AuthoritySource::WorktreeInspection,
    )?;
    let input = RemoteDeliveryInput::new(pvf, review, publication, pr, issue, cleanup);
    match request.operation {
        RemoteCommandOperation::VerifyBridgeEvidence => Ok((
            RemoteCommandStatus::ReadyForTypedBridge,
            serde_json::json!({
                "authority": "typed evidence accepted for v3 derivation",
                "mutation_allowed": false
            }),
        )),
        RemoteCommandOperation::Deliver => {
            let result = deliver(input).map_err(|error| {
                RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
            })?;
            Ok((
                RemoteCommandStatus::DeliveryDerived,
                serde_json::json!({
                    "finish": format!("{:?}", result.finish),
                    "cleanup": result.cleanup.map(|cleanup| format!("{cleanup:?}")),
                    "authorization_issue": result.authorization.issue,
                    "publication_pull_request": result.publication.pull_request,
                    "mutation_allowed": false
                }),
            ))
        }
        RemoteCommandOperation::Finish => {
            let RemoteDeliveryInput {
                publication,
                pull_request,
                issue,
                ..
            } = input;
            let publication_request = publication
                .require_source(AuthoritySource::PublicationIntent)
                .map_err(|error| {
                    RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
                })?;
            let review = parse_review(request, required(evidence, "typed_review_ref")?)?;
            let target = ReviewTarget {
                repository: publication_request.repository.clone(),
                issue: publication_request.issue,
                mode: publication_request.mode,
            };
            let authorization = authorize_publication(
                &review,
                &publication_request.head_sha,
                &publication_request.publisher,
                target,
            )
            .map_err(|error| {
                RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
            })?;
            let publication = publish(publication_request, &authorization).map_err(|error| {
                RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
            })?;
            let pr = pull_request
                .require_source(AuthoritySource::GithubReadback)
                .map_err(|error| {
                    RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
                })?;
            let issue = issue
                .require_source(AuthoritySource::GithubReadback)
                .map_err(|error| {
                    RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
                })?;
            let finish = derive_finish(&publication, &pr, &issue);
            Ok((
                RemoteCommandStatus::FinishDerived,
                serde_json::json!({
                    "finish": format!("{finish:?}"),
                    "mutation_allowed": false
                }),
            ))
        }
        RemoteCommandOperation::CleanupPreview => {
            let RemoteDeliveryInput { cleanup, .. } = input;
            let cleanup = cleanup
                .require_source(AuthoritySource::WorktreeInspection)
                .map_err(|error| {
                    RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
                })?;
            let preview = classify_cleanup(&cleanup).map_err(|error| {
                RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}"))
            })?;
            Ok((
                RemoteCommandStatus::CleanupPreviewDerived,
                serde_json::json!({
                    "cleanup": format!("{preview:?}"),
                    "mutation_allowed": false
                }),
            ))
        }
    }
}

fn required<'a>(
    evidence: &'a BTreeMap<&'static str, Value>,
    field: &'static str,
) -> Result<&'a Value, RemoteCommandRejectReason> {
    evidence
        .get(field)
        .ok_or(RemoteCommandRejectReason::EvidenceRefMissing { field })
}

fn verified<T: VerifiableSubject>(
    value: T,
    source: AuthoritySource,
) -> Result<Verified<T>, RemoteCommandRejectReason> {
    let subject_digest = value.subject_digest();
    Verified::new(value, receipt(source, &subject_digest))
        .map_err(|error| RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}")))
}

fn parse_pvf(
    request: &RemoteCommandRequest,
    value: &Value,
) -> Result<AcceptedPvfResult, RemoteCommandRejectReason> {
    require_schema(value, "pvf_evidence_ref", "csdlc.v3.pvf_result.v1")?;
    let issue = u64_field(value, "pvf_evidence_ref", "issue")?;
    let revision = string_field(value, "pvf_evidence_ref", "revision")?;
    let evidence_digest = string_field(value, "pvf_evidence_ref", "evidence_digest")?;
    if issue != request.issue || revision != request.head_sha || evidence_digest.trim().is_empty() {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "pvf_evidence_ref",
        });
    }
    Ok(AcceptedPvfResult {
        issue,
        revision,
        evidence_digest,
    })
}

fn parse_review(
    request: &RemoteCommandRequest,
    value: &Value,
) -> Result<ReviewRecord, RemoteCommandRejectReason> {
    require_schema(value, "typed_review_ref", "csdlc.v3.accepted_review.v1")?;
    let issue = u64_field(value, "typed_review_ref", "issue")?;
    let reviewed_revision = string_field(value, "typed_review_ref", "reviewed_revision")?;
    let scope_paths = string_array_field(value, "typed_review_ref", "scope_paths")?;
    let implementer = string_field(value, "typed_review_ref", "implementer")?;
    let reviewer = string_field(value, "typed_review_ref", "reviewer")?;
    let typed_review_evidence_digest =
        string_field(value, "typed_review_ref", "typed_review_evidence_digest")?;
    let target = object_field(value, "typed_review_ref", "target")?;
    let target_repository = string_field(target, "typed_review_ref", "repository")?;
    let target_issue = u64_field(target, "typed_review_ref", "issue")?;
    let target_mode = mode_from_string(
        "typed_review_ref",
        &string_field(target, "typed_review_ref", "mode")?,
    )?;
    let findings = value
        .get("findings")
        .and_then(Value::as_array)
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "typed_review_ref",
        })?
        .iter()
        .map(parse_review_finding)
        .collect::<Result<Vec<_>, _>>()?;
    if issue != request.issue
        || reviewed_revision != request.head_sha
        || target_repository != request.repository
        || target_issue != request.issue
        || target_mode != request.mode.into()
    {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "typed_review_ref",
        });
    }
    review_from_accepted_evidence(AcceptedReviewEvidence {
        issue,
        reviewed_revision,
        scope_paths,
        implementer,
        reviewer,
        findings,
        target: ReviewTarget {
            repository: target_repository,
            issue: target_issue,
            mode: target_mode,
        },
        typed_review_evidence_digest,
    })
    .map_err(|error| RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}")))
}

fn parse_publication(
    request: &RemoteCommandRequest,
    value: &Value,
) -> Result<PublicationRequest, RemoteCommandRejectReason> {
    require_schema(
        value,
        "publication_intent_ref",
        "csdlc.v3.publication_intent.v1",
    )?;
    let repository = string_field(value, "publication_intent_ref", "repository")?;
    let issue = u64_field(value, "publication_intent_ref", "issue")?;
    let pull_request = u64_field(value, "publication_intent_ref", "pull_request")?;
    let mode = mode_from_string(
        "publication_intent_ref",
        &string_field(value, "publication_intent_ref", "mode")?,
    )?;
    let publisher = string_field(value, "publication_intent_ref", "publisher")?;
    let body = string_field(value, "publication_intent_ref", "body")?;
    let head_sha = string_field(value, "publication_intent_ref", "head_sha")?;
    if repository != request.repository
        || issue != request.issue
        || pull_request != request.pull_request
        || head_sha != request.head_sha
        || mode != request.mode.into()
    {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "publication_intent_ref",
        });
    }
    Ok(PublicationRequest {
        repository,
        issue,
        pull_request,
        mode,
        publisher,
        body,
        head_sha,
    })
}

fn parse_pr_readback(
    request: &RemoteCommandRequest,
    value: &Value,
) -> Result<PullRequestReadback, RemoteCommandRejectReason> {
    require_schema(value, "pr_readback_ref", "csdlc.v3.pr_readback.v1")?;
    let repository = string_field(value, "pr_readback_ref", "repository")?;
    let number = u64_field(value, "pr_readback_ref", "number")?;
    let head_sha = string_field(value, "pr_readback_ref", "head_sha")?;
    if repository != request.repository
        || number != request.pull_request
        || head_sha != request.head_sha
    {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "pr_readback_ref",
        });
    }
    Ok(PullRequestReadback {
        repository,
        number,
        head_sha,
        merged: bool_field(value, "pr_readback_ref", "merged")?,
        closes_issue: optional_u64_field(value, "closes_issue"),
        part_of_issue: optional_u64_field(value, "part_of_issue"),
    })
}

fn parse_issue_readback(
    request: &RemoteCommandRequest,
    value: &Value,
) -> Result<IssueReadback, RemoteCommandRejectReason> {
    require_schema(value, "issue_readback_ref", "csdlc.v3.issue_readback.v1")?;
    let repository = string_field(value, "issue_readback_ref", "repository")?;
    let issue = u64_field(value, "issue_readback_ref", "issue")?;
    if repository != request.repository || issue != request.issue {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
            field: "issue_readback_ref",
        });
    }
    Ok(IssueReadback {
        repository,
        issue,
        open: bool_field(value, "issue_readback_ref", "open")?,
    })
}

fn parse_cleanup(value: &Value) -> Result<CleanupCandidate, RemoteCommandRejectReason> {
    require_schema(
        value,
        "cleanup_inspection_ref",
        "csdlc.v3.cleanup_inspection.v1",
    )?;
    let registration = object_field(value, "cleanup_inspection_ref", "registration")?;
    let registration = crate::publication::GitWorktreeRegistration {
        repository_root: Path::new(&string_field(
            registration,
            "cleanup_inspection_ref",
            "repository_root",
        )?)
        .to_path_buf(),
        worktree_path: Path::new(&string_field(
            registration,
            "cleanup_inspection_ref",
            "worktree_path",
        )?)
        .to_path_buf(),
        git_common_dir: Path::new(&string_field(
            registration,
            "cleanup_inspection_ref",
            "git_common_dir",
        )?)
        .to_path_buf(),
    };
    crate::publication::cleanup_candidate_from_git_registration(
        Path::new(&string_field(
            value,
            "cleanup_inspection_ref",
            "approved_worktree_parent",
        )?),
        Path::new(&string_field(
            value,
            "cleanup_inspection_ref",
            "candidate_path",
        )?),
        registration,
        bool_field(value, "cleanup_inspection_ref", "preview")?,
        bool_field(value, "cleanup_inspection_ref", "preview_receipt")?,
        bool_field(value, "cleanup_inspection_ref", "committed_closed_out")?,
        bool_field(value, "cleanup_inspection_ref", "terminal_receipt")?,
        value
            .get("preview_identity_digest")
            .and_then(Value::as_str)
            .map(str::to_owned),
        bool_field(value, "cleanup_inspection_ref", "dirty")?,
        bool_field(value, "cleanup_inspection_ref", "live")?,
    )
    .map_err(|error| RemoteCommandRejectReason::EvidenceDerivationFailed(format!("{error:?}")))
}

fn parse_review_finding(value: &Value) -> Result<ReviewFinding, RemoteCommandRejectReason> {
    let id = string_field(value, "typed_review_ref", "id")?;
    let disposition = match string_field(value, "typed_review_ref", "disposition")?.as_str() {
        "resolved" => crate::review::FindingDisposition::Resolved,
        "non_actionable" => crate::review::FindingDisposition::NonActionable,
        "actionable" => crate::review::FindingDisposition::Actionable,
        _ => {
            return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch {
                field: "typed_review_ref",
            })
        }
    };
    Ok(ReviewFinding { id, disposition })
}

fn require_schema(
    value: &Value,
    field: &'static str,
    expected: &str,
) -> Result<(), RemoteCommandRejectReason> {
    let schema = value
        .get("schema")
        .and_then(Value::as_str)
        .ok_or(RemoteCommandRejectReason::EvidenceRefSchemaMissing { field })?;
    if schema != expected {
        return Err(RemoteCommandRejectReason::EvidenceSchemaMismatch { field });
    }
    Ok(())
}

fn object_field<'a>(
    value: &'a Value,
    field: &'static str,
    key: &str,
) -> Result<&'a Value, RemoteCommandRejectReason> {
    value
        .get(key)
        .filter(|value| value.is_object())
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })
}

fn string_field(
    value: &Value,
    field: &'static str,
    key: &str,
) -> Result<String, RemoteCommandRejectReason> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .filter(|value| !value.trim().is_empty())
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })
}

fn string_array_field(
    value: &Value,
    field: &'static str,
    key: &str,
) -> Result<Vec<String>, RemoteCommandRejectReason> {
    let values = value
        .get(key)
        .and_then(Value::as_array)
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })?;
    let values = values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .filter(|value| !value.trim().is_empty())
                .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if values.is_empty() {
        return Err(RemoteCommandRejectReason::EvidenceIdentityMismatch { field });
    }
    Ok(values)
}

fn u64_field(
    value: &Value,
    field: &'static str,
    key: &str,
) -> Result<u64, RemoteCommandRejectReason> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .filter(|value| *value != 0)
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })
}

fn optional_u64_field(value: &Value, key: &str) -> Option<u64> {
    value.get(key).and_then(Value::as_u64)
}

fn bool_field(
    value: &Value,
    field: &'static str,
    key: &str,
) -> Result<bool, RemoteCommandRejectReason> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or(RemoteCommandRejectReason::EvidenceIdentityMismatch { field })
}

fn mode_from_string(
    field: &'static str,
    value: &str,
) -> Result<PublicationMode, RemoteCommandRejectReason> {
    match value {
        "closing" => Ok(PublicationMode::Closing),
        "part_of" => Ok(PublicationMode::PartOf),
        _ => Err(RemoteCommandRejectReason::EvidenceIdentityMismatch { field }),
    }
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
