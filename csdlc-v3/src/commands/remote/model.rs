//! Stable remote request, receipt, result and dispatch contracts.

use crate::adapters::CommandInvocation;
use serde::{Deserialize, Serialize};

use serde_json::Value;

pub const REMOTE_PUBLICATION_ROUTE_NAMES: [&str; 6] = [
    "github",
    "github-issue",
    "github-pr",
    "pr-state",
    "publish",
    "review",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinationEvidence {
    pub path: String,
    /// BLAKE3 of the exact durable evidence bytes approved by the operator.
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoordinationCompletion {
    pub current_body: String,
    pub expected_updated_at: String,
    pub rationale: String,
    pub evidence: Vec<CoordinationEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationLinkage {
    /// Issue repository; it may differ from the code/PR repository.
    pub repository: String,
    pub issue: u64,
    pub mode: RemotePublicationMode,
}

/// Only two-parent merge commits are supported: replay can prove their identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeMethod {
    Merge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergedIdentity {
    pub repository: String,
    pub pull_request: u64,
    pub head_sha: String,
    pub base: String,
    pub base_sha: String,
    pub method: MergeMethod,
    pub merge_commit: String,
    pub publication_linkage: PublicationLinkage,
    pub issue_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct MergeIntent {
    pub(super) schema: String,
    pub(super) request: GithubMutationRequest,
    pub(super) selector_digest: String,
    pub(super) publication_linkage: PublicationLinkage,
    pub(super) pre_state: Value,
    pub(super) rules: Value,
    pub(super) base_sha: String,
}

#[derive(Debug, Clone)]
pub(super) struct StagedMerge {
    pub(super) request: GithubMutationRequest,
    pub(super) intent: MergeIntent,
    pub(super) intent_digest: String,
    pub(super) operation_digest: String,
    pub(super) preexisting: bool,
}

impl StagedMerge {
    pub(super) fn request(&self) -> &GithubMutationRequest {
        &self.request
    }

    pub(super) fn intent_digest(&self) -> &str {
        &self.intent_digest
    }

    pub(super) fn operation_digest(&self) -> &str {
        &self.operation_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteRouteRequest {
    pub repository: String,
    pub issue: u64,
    #[serde(default)]
    pub pull_request: Option<u64>,
    #[serde(default)]
    pub actor: Option<String>,
    #[serde(default)]
    pub implementer: Option<String>,
    #[serde(default)]
    pub reviewer: Option<String>,
    #[serde(default)]
    pub review_revision: Option<String>,
    #[serde(default)]
    pub expected_head_sha: Option<String>,
    #[serde(default)]
    pub head_sha: Option<String>,
    #[serde(default)]
    pub mode: Option<RemotePublicationMode>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub review_present: bool,
    #[serde(default)]
    pub typed_review_receipt_path: Option<String>,
    #[serde(default)]
    pub typed_review_receipt_digest: Option<String>,
    #[serde(default)]
    pub readback_source: Option<RemoteReadbackSource>,
    #[serde(default)]
    pub readback_receipt_path: Option<String>,
    #[serde(default)]
    pub readback_receipt_digest: Option<String>,
    #[serde(default)]
    pub adapter_receipt_path: Option<String>,
    #[serde(default)]
    pub adapter_receipt_digest: Option<String>,
    #[serde(default)]
    pub closes_issue: Option<u64>,
    #[serde(default)]
    pub closing_issues: Vec<u64>,
    #[serde(default)]
    pub part_of_issue: Option<u64>,
    #[serde(default)]
    pub credential_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RemoteRouteReceipts {
    pub typed_review: Option<TypedReviewReceipt>,
    pub github_readback: Option<GithubReadbackReceipt>,
    pub adapter: Option<GithubAdapterReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedReviewReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub implementer: String,
    pub reviewer: String,
    pub reviewed_revision: String,
    pub expected_head_sha: String,
    pub evidence_digest: String,
    /// Required by merge admission; older publication-only receipts remain readable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_linkage: Option<PublicationLinkage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubReadbackReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    #[serde(default)]
    pub title: Option<String>,
    pub head_sha: String,
    #[serde(default)]
    pub closes_issue: Option<u64>,
    #[serde(default)]
    pub closing_issues: Vec<u64>,
    #[serde(default)]
    pub part_of_issue: Option<u64>,
    pub source: RemoteReadbackSource,
    pub observed_by: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubAdapterReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    pub head_sha: String,
    pub readback_receipt_digest: String,
    pub credential_names: Vec<String>,
    pub adapter: String,
    pub authenticated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedRemoteRouteRequest {
    pub request: RemoteRouteRequest,
    pub receipts: RemoteRouteReceipts,
    pub invocation: CommandInvocation,
}

/// A bounded GitHub mutation owned by the v3 remote route.  Arbitrary URLs,
/// shell strings and credential values are deliberately not representable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum GithubMutation {
    IssueCreate {
        title: String,
        body: String,
        #[serde(default)]
        labels: Vec<String>,
        #[serde(default)]
        assignees: Vec<String>,
        #[serde(default)]
        milestone: Option<u64>,
    },
    IssueComment {
        body: String,
    },
    IssueEdit {
        title: Option<String>,
        body: Option<String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "explicit_metadata"
        )]
        labels: Option<IssueLabelsUpdate>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "explicit_metadata"
        )]
        assignees: Option<Vec<String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "explicit_metadata"
        )]
        milestone: Option<IssueMilestoneUpdate>,
    },
    IssueClose {
        rationale: String,
        current_body: String,
        disposition: IssueCloseDisposition,
        #[serde(default)]
        duplicate_of: Option<u64>,
        #[serde(default)]
        github_state_reason: Option<IssueCloseStateReason>,
    },
    IssueCompleteCoordination {
        completion: CoordinationCompletion,
    },
    PullRequestCreate {
        base: String,
        head: String,
        title: String,
        body: String,
        #[serde(default)]
        draft: bool,
    },
    PullRequestUpdate {
        title: Option<String>,
        body: Option<String>,
    },
    PullRequestReady,
    PullRequestMerge {
        base: String,
        method: MergeMethod,
        review_receipt_path: String,
        review_receipt_digest: String,
    },
}

fn explicit_metadata<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

/// Omission preserves metadata; clear operations are explicit, never JSON null.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum IssueLabelsUpdate {
    Replace { names: Vec<String> },
    Add { names: Vec<String> },
    Remove { names: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum IssueMilestoneUpdate {
    Set { number: u64 },
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueCloseDisposition {
    Duplicate,
    Superseded,
    NoOp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueCloseStateReason {
    Completed,
    NotPlanned,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationRequest {
    pub repository: String,
    #[serde(default)]
    pub issue: u64,
    #[serde(default)]
    pub pull_request: Option<u64>,
    #[serde(default)]
    pub cutover_issue: Option<u64>,
    #[serde(default)]
    pub operator_approval: Option<String>,
    pub expected_head_sha: String,
    pub credential_names: Vec<String>,
    #[serde(default)]
    pub recovery: Option<GithubMutationRecovery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_non_effect_disposition: Option<GithubMutationLegacyNonEffectDisposition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_non_effect_disposition_source: Option<CoordinationEvidence>,
    pub mutation: GithubMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GithubMutationRecovery {
    RetryAfterAuthenticatedAbsence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GithubMutationLegacyNonEffectDisposition {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub operation_digest: String,
    pub intent_digest: String,
    pub request_digest: String,
    pub authority_selector_digest: String,
    pub expected_head_sha: String,
    pub definitive_non_effect_reason: GithubMutationDefinitiveNonEffectReason,
    pub evidence_path: String,
    pub evidence_digest: String,
    pub operator: String,
    pub authorization_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GithubMutationDefinitiveNonEffectReason {
    TransportFailedBeforeDispatch,
    ProviderRejectedBeforeAcceptance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationIntent {
    pub schema: String,
    pub operation_digest: String,
    pub operation_marker: String,
    pub authority_selector_digest: String,
    pub request: GithubMutationRequest,
    pub adapter: String,
    /// First authenticated resolution, retained unchanged across retries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_edit: Option<GithubMutation>,
    /// First authenticated PR identity used by the narrow ready mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_ready_target: Option<GithubReadyTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubReadyTarget {
    pub number: u64,
    pub node_id: String,
    pub head_sha: String,
    pub draft: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct GithubMutationRecoveryReceipt {
    pub(super) schema: String,
    pub(super) operation_digest: String,
    pub(super) intent_digest: String,
    pub(super) recovery: GithubMutationRecovery,
    pub(super) repository: String,
    pub(super) issue: u64,
    pub(super) pull_request: Option<u64>,
    pub(super) expected_head_sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) resolved_ready_target: Option<GithubReadyTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GithubMutationHeadAvailableRecoveryReceipt {
    pub(super) schema: String,
    pub(super) operation_digest: String,
    pub(super) intent_digest: String,
    pub(super) repository: String,
    pub(super) issue: u64,
    pub(super) head: String,
    pub(super) expected_head_sha: String,
}

pub(super) struct GithubMutationDispatchContext<'a> {
    pub(super) operation_digest: &'a str,
    pub(super) operation_marker: &'a str,
    pub(super) credential_name: &'a str,
    pub(super) ready_target: Option<&'a GithubReadyTarget>,
    pub(super) recovery_intent_digest: Option<&'a str>,
    pub(super) head_available_recovery: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationReconciliationReceipt {
    pub schema: String,
    pub operation_digest: String,
    pub operation_marker: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: Option<u64>,
    pub remote_object_id: Option<u64>,
    pub expected_head_sha: String,
    pub readback_digest: String,
    pub observed_by: String,
    pub authenticated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge: Option<MergedIdentity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: Option<u64>,
    pub expected_head_sha: String,
    pub operation_digest: String,
    pub response_digest: Option<String>,
    pub readback_digest: Option<String>,
    pub intent_digest: String,
    pub reconciliation_digest: String,
    pub adapter: String,
    pub authenticated: bool,
    pub idempotent_replay: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubMutationResult {
    /// Per-invocation effects, not whether a historical receipt exists.
    /// None preserves uncertainty for owners without effect instrumentation.
    pub performed_mutation: Option<bool>,
    pub receipt: GithubMutationReceipt,
    pub reconciliation: GithubMutationReconciliationReceipt,
    pub invocation: CommandInvocation,
}

/// Exact native mutation intent prepared and durably retained before the
/// semantic transaction reserves the corresponding effect.
#[derive(Debug, Clone)]
pub struct StagedGithubMutation {
    pub(super) request: GithubMutationRequest,
    pub(super) native_identity: crate::storage::semantic::protocol::NativeIdentity,
    pub(super) request_bytes: Vec<u8>,
    pub(super) operation_digest: String,
    pub(super) operation_marker: String,
    pub(super) intent_digest: String,
    pub(super) credential_name: String,
    pub(super) resolved_ready_target: Option<GithubReadyTarget>,
    pub(super) merge: Option<StagedMerge>,
    pub(super) preexisting: bool,
    pub(super) recovery: Option<GithubMutationRecovery>,
    pub(super) legacy_non_effect_disposition: Option<GithubMutationLegacyNonEffectDisposition>,
    pub(super) legacy_non_effect_disposition_source: Option<CoordinationEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalRemoteDispatchRequest {
    pub expected_lifecycle_digest: String,
    pub exact_review_sha: String,
    pub operation: OperationalRemoteOperation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "request", rename_all = "snake_case")]
pub enum OperationalRemoteOperation {
    Review(RemoteRouteRequest),
    Publish(RemoteRouteRequest),
    GithubMutation(GithubMutationRequest),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanonicalV3AuthorityEvidence {
    pub schema: String,
    pub selector_path: String,
    pub selector_digest: String,
    pub authority_issue: u64,
    pub exact_review_sha: String,
    pub readiness_evidence_digest: String,
    pub approval_evidence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperationalGithubMutationResult {
    pub performed_mutation: Option<bool>,
    pub receipt: GithubMutationReceipt,
    pub reconciliation: GithubMutationReconciliationReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "result", rename_all = "snake_case")]
pub enum OperationalRemoteOutcome {
    Review(RemoteRoutePlan),
    Publish(RemoteRoutePlan),
    GithubMutation(Box<OperationalGithubMutationResult>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperationalRemoteDispatchResult {
    pub schema: String,
    pub authority: CanonicalV3AuthorityEvidence,
    pub outcome: OperationalRemoteOutcome,
}

#[derive(Debug, Deserialize)]
struct CanonicalAuthoritySelector {
    schema: String,
    default_generation: String,
    operational_authority: Option<String>,
    #[serde(default, alias = "cutover_issue")]
    authority_issue: Option<u64>,
    #[serde(
        default,
        alias = "approved_sha",
        alias = "approved_revision",
        alias = "reviewed_revision"
    )]
    exact_review_sha: Option<String>,
    #[serde(default)]
    readiness_evidence_digest: Option<String>,
    #[serde(default)]
    approval_evidence_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationStateReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    pub head_sha: String,
    pub mode: RemotePublicationMode,
    pub review_receipt_digest: String,
    pub mutation_operation_digest: String,
    pub github_readback_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemotePublicationMode {
    Closing,
    PartOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteReadbackSource {
    Github,
    Caller,
    Fixture,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteRoutePlan {
    pub route: String,
    pub issue: u64,
    pub repository: String,
    pub status: RemoteRouteStatus,
    pub findings: Vec<RemoteRouteFinding>,
    pub redacted_credentials: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteRouteStatus {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteRouteFinding {
    pub code: String,
    pub message: String,
}
