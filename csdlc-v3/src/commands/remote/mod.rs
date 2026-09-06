#![allow(dead_code)]

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};
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
use std::fs;
use std::path::{Path, PathBuf};

const GITHUB_READ_ONLY_ADAPTER: &str = "github-api-read-only";
const GITHUB_OPERATIONAL_ADAPTER: &str = "github-api-operational";
const CANONICAL_AUTHORITY_SELECTOR_PATH: &str = "csdlc-v2/operator/generation-selector.json";
const GITHUB_OPERATION_MARKER_PREFIX: &str = "csdlc-v3-operation";

pub const REMOTE_PUBLICATION_ROUTE_NAMES: [&str; 6] = [
    "github",
    "github-issue",
    "github-pr",
    "pr-state",
    "publish",
    "review",
];

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
#[serde(tag = "action", rename_all = "snake_case")]
pub enum GithubMutation {
    IssueComment {
        body: String,
    },
    IssueEdit {
        title: Option<String>,
        body: Option<String>,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationRequest {
    pub repository: String,
    pub issue: u64,
    #[serde(default)]
    pub pull_request: Option<u64>,
    #[serde(default)]
    pub cutover_issue: Option<u64>,
    #[serde(default)]
    pub operator_approval: Option<String>,
    pub expected_head_sha: String,
    pub credential_names: Vec<String>,
    pub mutation: GithubMutation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubMutationIntent {
    pub schema: String,
    pub operation_digest: String,
    pub operation_marker: String,
    pub authority_selector_digest: String,
    pub request: GithubMutationRequest,
    pub adapter: String,
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
    pub receipt: GithubMutationReceipt,
    pub reconciliation: GithubMutationReconciliationReceipt,
    pub invocation: CommandInvocation,
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

pub fn prepare_remote_publication_route(
    route: &str,
    request: &RemoteRouteRequest,
) -> Result<RemoteRoutePlan, RemoteRouteFinding> {
    prepare_remote_publication_route_with_receipts(route, request, &RemoteRouteReceipts::default())
}

pub fn prepare_remote_publication_route_with_receipts(
    route: &str,
    request: &RemoteRouteRequest,
    receipts: &RemoteRouteReceipts,
) -> Result<RemoteRoutePlan, RemoteRouteFinding> {
    if !REMOTE_PUBLICATION_ROUTE_NAMES.contains(&route) {
        return Err(remote_finding(
            "unknown_remote_publication_route",
            "route is not owned by #629",
        ));
    }
    let findings = match route {
        "publish" => publication_findings(request, receipts),
        "github" | "github-issue" | "pr-state" | "github-pr" => {
            pr_state_findings(request, receipts)
        }
        "review" => review_findings(request),
        _ => unreachable!("route checked above"),
    };
    let status = if findings.is_empty() {
        RemoteRouteStatus::Ready
    } else {
        RemoteRouteStatus::Blocked
    };
    Ok(RemoteRoutePlan {
        route: route.to_owned(),
        issue: request.issue,
        repository: request.repository.clone(),
        status,
        findings,
        redacted_credentials: request
            .credential_names
            .iter()
            .map(|name| format!("{name}=<redacted>"))
            .collect(),
    })
}

fn publication_findings(
    request: &RemoteRouteRequest,
    receipts: &RemoteRouteReceipts,
) -> Vec<RemoteRouteFinding> {
    let mut findings = Vec::new();
    if !request.review_present {
        findings.push(remote_finding(
            "missing_review_truth",
            "publication requires current typed review truth",
        ));
    }
    if !typed_review_receipt_matches(request, receipts.typed_review.as_ref()) {
        findings.push(remote_finding(
            "authenticated_review_receipt_missing",
            "publication requires a repo-contained typed review receipt matching the exact issue, principals, and head",
        ));
    }
    let expected = request
        .expected_head_sha
        .as_deref()
        .unwrap_or_default()
        .trim();
    let actual = request.head_sha.as_deref().unwrap_or_default().trim();
    if expected.is_empty() || actual.is_empty() {
        findings.push(remote_finding(
            "missing_review_revision",
            "publication requires exact reviewed and current head revisions",
        ));
    } else if expected != actual {
        findings.push(remote_finding(
            "stale_review_truth",
            "publication head must match the reviewed exact head",
        ));
    }
    match request.mode {
        Some(RemotePublicationMode::Closing)
            if !body_has_relation(request.body.as_deref(), "Closes", request.issue) =>
        {
            findings.push(remote_finding(
                "missing_closing_relation",
                "closing publication must visibly include Closes #<issue>",
            ));
        }
        Some(RemotePublicationMode::PartOf)
            if !body_has_relation(request.body.as_deref(), "Part of", request.issue)
                && !body_has_relation(request.body.as_deref(), "Part-Of", request.issue) =>
        {
            findings.push(remote_finding(
                "missing_part_of_relation",
                "checkpoint publication must visibly include Part of #<issue>",
            ));
        }
        None => findings.push(remote_finding(
            "missing_publication_mode",
            "publication mode is required",
        )),
        _ => {}
    }
    if matches!(request.mode, Some(RemotePublicationMode::Closing)) {
        let body_closing_issues = body_closing_issue_references(request.body.as_deref());
        if body_closing_issues
            .iter()
            .any(|issue| *issue != request.issue)
        {
            findings.push(remote_finding(
                "unexpected_closing_relation",
                "closing publication body must not include GitHub closing-keyword references for issues other than the tracked issue",
            ));
        }
    }
    findings.extend(pr_state_findings(request, receipts));
    findings
}

fn pr_state_findings(
    request: &RemoteRouteRequest,
    receipts: &RemoteRouteReceipts,
) -> Vec<RemoteRouteFinding> {
    let mut findings = Vec::new();
    if request.readback_source != Some(RemoteReadbackSource::Github) {
        findings.push(remote_finding(
            "caller_forged_readback",
            "PR state must come from authenticated GitHub readback",
        ));
    }
    if !github_readback_receipt_matches(request, receipts.github_readback.as_ref()) {
        findings.push(remote_finding(
            "github_readback_receipt_missing",
            "PR state requires a repo-contained GitHub readback receipt over the observed PR fields",
        ));
    }
    if !github_adapter_receipt_matches(request, receipts) {
        findings.push(remote_finding(
            "authenticated_github_adapter_missing",
            "PR state requires a repo-contained authenticated adapter receipt bound to the readback receipt",
        ));
    }
    match request.mode {
        Some(RemotePublicationMode::Closing) if request.closes_issue != Some(request.issue) => {
            findings.push(remote_finding(
                "missing_closing_readback",
                "GitHub readback must expose the closing issue relation",
            ));
        }
        Some(RemotePublicationMode::Closing)
            if request
                .closing_issues
                .iter()
                .any(|issue| *issue != request.issue) =>
        {
            findings.push(remote_finding(
                "unexpected_closing_readback",
                "GitHub readback must not expose closing relations for issues other than the tracked issue",
            ));
        }
        Some(RemotePublicationMode::PartOf) if request.part_of_issue != Some(request.issue) => {
            findings.push(remote_finding(
                "missing_part_of_readback",
                "GitHub readback must expose the part-of relation",
            ));
        }
        _ => {}
    }
    findings
}

fn review_findings(request: &RemoteRouteRequest) -> Vec<RemoteRouteFinding> {
    let mut findings = Vec::new();
    if same_principal(request.implementer.as_deref(), request.reviewer.as_deref()) {
        findings.push(remote_finding(
            "self_review_denied",
            "implementer and reviewer must be distinct principals",
        ));
    }
    if request
        .review_revision
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        findings.push(remote_finding(
            "missing_exact_review_revision",
            "review route requires an exact reviewed revision",
        ));
    }
    findings
}

fn same_principal(left: Option<&str>, right: Option<&str>) -> bool {
    let left = left.unwrap_or_default().trim();
    let right = right.unwrap_or_default().trim();
    !left.is_empty() && left.eq_ignore_ascii_case(right)
}

fn body_has_relation(body: Option<&str>, verb: &str, issue: u64) -> bool {
    let prefix = format!("{verb} #{issue}");
    body.unwrap_or_default()
        .lines()
        .any(|line| has_issue_relation_prefix(line.trim_start(), &prefix))
}

fn body_closing_issue_references(body: Option<&str>) -> Vec<u64> {
    let Some(body) = body else {
        return Vec::new();
    };
    let lower = body.to_ascii_lowercase();
    let keywords = [
        "close", "closes", "closed", "fix", "fixes", "fixed", "resolve", "resolves", "resolved",
    ];
    let mut issues = Vec::new();
    for keyword in keywords {
        let mut search_start = 0;
        while let Some(relative) = lower[search_start..].find(keyword) {
            let keyword_start = search_start + relative;
            let after_keyword = keyword_start + keyword.len();
            search_start = after_keyword;
            if !word_boundary_before(&lower, keyword_start)
                || !word_boundary_after(&lower, after_keyword)
            {
                continue;
            }
            if let Some(issue) = parse_issue_after_closing_keyword(&lower[after_keyword..]) {
                issues.push(issue);
            }
        }
    }
    issues.sort_unstable();
    issues.dedup();
    issues
}

fn word_boundary_before(text: &str, index: usize) -> bool {
    index == 0
        || text[..index]
            .chars()
            .next_back()
            .is_none_or(|ch| !ch.is_ascii_alphanumeric() && ch != '_')
}

fn word_boundary_after(text: &str, index: usize) -> bool {
    index == text.len()
        || text[index..]
            .chars()
            .next()
            .is_none_or(|ch| !ch.is_ascii_alphanumeric() && ch != '_')
}

fn parse_issue_after_closing_keyword(rest: &str) -> Option<u64> {
    let rest = rest.trim_start();
    let digits = rest.strip_prefix('#')?;
    let digits = digits
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn has_issue_relation_prefix(line: &str, prefix: &str) -> bool {
    let Some(rest) = line.strip_prefix(prefix) else {
        return false;
    };
    rest.is_empty()
        || rest
            .chars()
            .next()
            .is_some_and(|ch| ch.is_whitespace() || matches!(ch, ',' | '.' | ';' | ':' | ')' | ']'))
}

fn remote_finding(code: &str, message: &str) -> RemoteRouteFinding {
    RemoteRouteFinding {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

pub fn typed_review_receipt_payload_digest(receipt: &TypedReviewReceipt) -> String {
    stable_digest(&[
        &receipt.schema,
        &receipt.repository,
        &receipt.issue.to_string(),
        &receipt.implementer,
        &receipt.reviewer,
        &receipt.reviewed_revision,
        &receipt.expected_head_sha,
        &receipt.evidence_digest,
    ])
}

pub fn github_readback_receipt_payload_digest(receipt: &GithubReadbackReceipt) -> String {
    stable_digest(&[
        &receipt.schema,
        &receipt.repository,
        &receipt.issue.to_string(),
        &receipt.pull_request.to_string(),
        receipt.title.as_deref().unwrap_or_default(),
        &receipt.head_sha,
        &receipt.closes_issue.unwrap_or_default().to_string(),
        &receipt
            .closing_issues
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(","),
        &receipt.part_of_issue.unwrap_or_default().to_string(),
        match receipt.source {
            RemoteReadbackSource::Github => "github",
            RemoteReadbackSource::Caller => "caller",
            RemoteReadbackSource::Fixture => "fixture",
        },
        &receipt.observed_by,
    ])
}

pub fn github_adapter_receipt_payload_digest(receipt: &GithubAdapterReceipt) -> String {
    stable_digest(&[
        &receipt.schema,
        &receipt.repository,
        &receipt.issue.to_string(),
        &receipt.pull_request.to_string(),
        &receipt.head_sha,
        &receipt.readback_receipt_digest,
        &receipt.credential_names.join(","),
        &receipt.adapter,
        if receipt.authenticated {
            "authenticated"
        } else {
            "unauthenticated"
        },
    ])
}

pub fn observe_github_pr_readback(
    request: &RemoteRouteRequest,
    process: &mut impl ProcessAdapter,
) -> Result<ObservedRemoteRouteRequest, RemoteRouteFinding> {
    let pull_request = request.pull_request.ok_or_else(|| {
        remote_finding(
            "missing_pull_request",
            "authenticated GitHub observation requires a concrete pull request number",
        )
    })?;
    let credential_name = single_credential_name(request)?;
    validate_repository_name(&request.repository)?;

    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [
            "pull-request".to_owned(),
            request.repository.clone(),
            pull_request.to_string(),
        ],
    )
    .map_err(|_| {
        remote_finding(
            "github_observation_invocation_rejected",
            "authenticated GitHub observation must use structured argv without shell strings or secrets",
        )
    })?
    .with_child_credential(credential_name)
    .map_err(|_| {
        remote_finding(
            "github_credential_scope_invalid",
            "GitHub credential names must be explicit safe child-process environment names",
        )
    })?;
    let output = process.run(invocation.clone());
    if output.truncated {
        return Err(remote_finding(
            "github_observation_truncated",
            "GitHub readback output was truncated and cannot be authoritative input",
        ));
    }
    if output.status != ProcessStatus::Exit(0) {
        return Err(remote_finding(
            "github_observation_failed",
            "GitHub readback adapter did not complete successfully",
        ));
    }
    let value: serde_json::Value = serde_json::from_str(&output.stdout).map_err(|_| {
        remote_finding(
            "github_observation_invalid_json",
            "GitHub readback adapter returned non-JSON output",
        )
    })?;
    let number = value["number"].as_u64().ok_or_else(|| {
        remote_finding(
            "github_observation_missing_pr_number",
            "GitHub pull request readback did not include a PR number",
        )
    })?;
    if number != pull_request {
        return Err(remote_finding(
            "github_observation_pr_mismatch",
            "GitHub pull request readback number does not match the requested PR",
        ));
    }
    let head_sha = value["head"]["sha"].as_str().ok_or_else(|| {
        remote_finding(
            "github_observation_missing_head",
            "GitHub pull request readback did not include head.sha",
        )
    })?;
    let title = value["title"].as_str().ok_or_else(|| {
        remote_finding(
            "github_observation_missing_title",
            "GitHub pull request readback did not include title",
        )
    })?;
    if title.trim().is_empty() {
        return Err(remote_finding(
            "github_observation_missing_title",
            "GitHub pull request readback title was empty",
        ));
    }
    let body = value["body"].as_str().unwrap_or_default();
    let closes_issue =
        body_has_relation(Some(body), "Closes", request.issue).then_some(request.issue);
    let closing_issues = body_closing_issue_references(Some(body));
    let part_of_issue = (body_has_relation(Some(body), "Part of", request.issue)
        || body_has_relation(Some(body), "Part-Of", request.issue))
    .then_some(request.issue);
    let readback = GithubReadbackReceipt {
        schema: "csdlc.v3.github_readback_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request,
        title: Some(title.to_owned()),
        head_sha: head_sha.to_owned(),
        closes_issue,
        closing_issues: closing_issues.clone(),
        part_of_issue,
        source: RemoteReadbackSource::Github,
        observed_by: GITHUB_READ_ONLY_ADAPTER.into(),
    };
    let readback_digest = github_readback_receipt_payload_digest(&readback);
    let adapter = GithubAdapterReceipt {
        schema: "csdlc.v3.github_adapter_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request,
        head_sha: head_sha.to_owned(),
        readback_receipt_digest: readback_digest.clone(),
        credential_names: request.credential_names.clone(),
        adapter: GITHUB_READ_ONLY_ADAPTER.into(),
        authenticated: true,
    };
    let adapter_digest = github_adapter_receipt_payload_digest(&adapter);
    let mut observed = request.clone();
    observed.title = Some(title.to_owned());
    observed.head_sha = Some(head_sha.to_owned());
    observed.readback_source = Some(RemoteReadbackSource::Github);
    observed.readback_receipt_digest = Some(readback_digest);
    observed.adapter_receipt_digest = Some(adapter_digest);
    observed.closes_issue = closes_issue;
    observed.closing_issues = closing_issues;
    observed.part_of_issue = part_of_issue;
    Ok(ObservedRemoteRouteRequest {
        request: observed,
        receipts: RemoteRouteReceipts {
            typed_review: None,
            github_readback: Some(readback),
            adapter: Some(adapter),
        },
        invocation,
    })
}

#[cfg(unix)]
pub fn dispatch_operational_remote(
    repo_root: &Path,
    dispatch: &OperationalRemoteDispatchRequest,
    process: &mut impl ProcessAdapter,
) -> Result<OperationalRemoteDispatchResult, RemoteRouteFinding> {
    let authority = verify_canonical_v3_authority(
        repo_root,
        Some(&dispatch.expected_lifecycle_digest),
        &dispatch.exact_review_sha,
    )?;
    let outcome = match &dispatch.operation {
        OperationalRemoteOperation::Review(request) => {
            verify_transition_revision("review", request, &dispatch.exact_review_sha)?;
            let receipts = load_remote_route_receipts(repo_root, request)?;
            OperationalRemoteOutcome::Review(prepare_remote_publication_route_with_receipts(
                "review", request, &receipts,
            )?)
        }
        OperationalRemoteOperation::Publish(request) => {
            verify_transition_revision("publish", request, &dispatch.exact_review_sha)?;
            let receipts = load_remote_route_receipts(repo_root, request)?;
            OperationalRemoteOutcome::Publish(prepare_remote_publication_route_with_receipts(
                "publish", request, &receipts,
            )?)
        }
        OperationalRemoteOperation::GithubMutation(request) => {
            if request.expected_head_sha != dispatch.exact_review_sha {
                return Err(remote_finding(
                    "operational_remote_exact_review_mismatch",
                    "GitHub mutation must bind the canonical exact review SHA",
                ));
            }
            let result = execute_github_mutation(repo_root, request, process)?;
            OperationalRemoteOutcome::GithubMutation(Box::new(OperationalGithubMutationResult {
                receipt: result.receipt,
                reconciliation: result.reconciliation,
            }))
        }
    };
    Ok(OperationalRemoteDispatchResult {
        schema: "csdlc.v3.operational_remote_dispatch.v1".into(),
        authority,
        outcome,
    })
}

#[cfg(not(unix))]
pub fn dispatch_operational_remote(
    _repo_root: &Path,
    _dispatch: &OperationalRemoteDispatchRequest,
    _process: &mut impl ProcessAdapter,
) -> Result<OperationalRemoteDispatchResult, RemoteRouteFinding> {
    Err(remote_finding(
        "operational_remote_unsupported_platform",
        "native v3 operational remote mutation is supported only on Unix platforms",
    ))
}

pub fn canonical_authority_selector_digest(repo_root: &Path) -> Result<String, RemoteRouteFinding> {
    let bytes = read_canonical_authority_selector(repo_root)?;
    Ok(stable_digest(&[std::str::from_utf8(&bytes).map_err(
        |_| {
            remote_finding(
                "canonical_authority_selector_invalid",
                "canonical authority selector must be UTF-8 JSON",
            )
        },
    )?]))
}

pub fn execute_github_mutation(
    repo_root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    #[cfg(not(unix))]
    return Err(remote_finding(
        "operational_remote_unsupported_platform",
        "native v3 operational remote mutation is supported only on Unix platforms",
    ));

    validate_repository_name(&request.repository)?;
    let credential_name = mutation_credential_name(request)?;
    validate_mutation(request)?;
    let authority = verify_canonical_v3_authority(repo_root, None, &request.expected_head_sha)?;

    let operation_digest = github_mutation_operation_digest(request);
    let operation_marker = github_mutation_operation_marker(&operation_digest);
    let intent = GithubMutationIntent {
        schema: "csdlc.v3.github_mutation_intent.v1".into(),
        operation_digest: operation_digest.clone(),
        operation_marker: operation_marker.clone(),
        authority_selector_digest: authority.selector_digest,
        request: request.clone(),
        adapter: GITHUB_OPERATIONAL_ADAPTER.into(),
    };
    let intent_digest = github_mutation_intent_digest(&intent);
    let intent_path = github_mutation_intent_path(repo_root, &operation_digest)?;
    let receipt_path = github_mutation_receipt_path(repo_root, &operation_digest)?;

    if receipt_path.exists() {
        let mut receipt = load_mutation_receipt(&receipt_path, &operation_digest)?;
        let (reconciliation, invocation) =
            reconcile_github_mutation(request, &operation_digest, &operation_marker, process)?;
        if receipt.intent_digest != intent_digest
            || receipt.reconciliation_digest
                != github_mutation_reconciliation_digest(&reconciliation)
        {
            return Err(remote_finding(
                "github_mutation_receipt_mismatch",
                "existing mutation receipt does not match current authenticated reconciliation",
            ));
        }
        receipt.idempotent_replay = true;
        return Ok(GithubMutationResult {
            receipt,
            reconciliation,
            invocation,
        });
    }

    if intent_path.exists() {
        let existing = load_mutation_intent(&intent_path, &operation_digest)?;
        if existing != intent {
            return Err(remote_finding(
                "github_mutation_intent_mismatch",
                "existing durable intent does not match this exact operation",
            ));
        }
        let (reconciliation, invocation) =
            reconcile_github_mutation(request, &operation_digest, &operation_marker, process)?;
        let receipt = finalize_mutation_receipt(
            request,
            &operation_digest,
            &intent_digest,
            None,
            &reconciliation,
            true,
        );
        persist_json_create_new(&receipt_path, &receipt)?;
        return Ok(GithubMutationResult {
            receipt,
            reconciliation,
            invocation,
        });
    }

    persist_json_create_new(&intent_path, &intent)?;
    let input_path =
        write_mutation_input(repo_root, &operation_digest, &operation_marker, request)?;
    let invocation = github_mutation_invocation(request, &input_path)?
        .with_child_credential(credential_name)
        .map_err(|_| {
            remote_finding(
                "github_credential_scope_invalid",
                "GitHub credential name is not safe for child-process injection",
            )
        })?;
    let output = process.run(invocation.clone());
    let _ = fs::remove_file(&input_path);
    let response_digest = (!output.stdout.is_empty()).then(|| stable_digest(&[&output.stdout]));

    let (reconciliation, _) = reconcile_github_mutation(
        request,
        &operation_digest,
        &operation_marker,
        process,
    )
    .map_err(|finding| {
        remote_finding(
            "github_mutation_reconciliation_pending",
            &format!(
                "mutation outcome is uncertain; durable intent forbids replay until authenticated reconciliation succeeds: {}",
                finding.code
            ),
        )
    })?;
    let receipt = finalize_mutation_receipt(
        request,
        &operation_digest,
        &intent_digest,
        response_digest,
        &reconciliation,
        false,
    );
    persist_json_create_new(&receipt_path, &receipt)?;
    Ok(GithubMutationResult {
        receipt,
        reconciliation,
        invocation,
    })
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

fn verify_transition_revision(
    route: &str,
    request: &RemoteRouteRequest,
    exact_review_sha: &str,
) -> Result<(), RemoteRouteFinding> {
    if exact_review_sha.trim().is_empty()
        || request.review_revision.as_deref() != Some(exact_review_sha)
        || request.expected_head_sha.as_deref() != Some(exact_review_sha)
        || (route == "publish" && request.head_sha.as_deref() != Some(exact_review_sha))
    {
        return Err(remote_finding(
            "operational_remote_exact_review_mismatch",
            "operational review and publication transitions require the canonical exact review SHA",
        ));
    }
    Ok(())
}

fn read_canonical_authority_selector(repo_root: &Path) -> Result<Vec<u8>, RemoteRouteFinding> {
    fs::read(repo_root.join(CANONICAL_AUTHORITY_SELECTOR_PATH)).map_err(|_| {
        remote_finding(
            "canonical_v3_authority_missing",
            "canonical generation selector is unavailable; v3 remote operations remain denied",
        )
    })
}

fn verify_canonical_v3_authority(
    repo_root: &Path,
    expected_lifecycle_digest: Option<&str>,
    exact_review_sha: &str,
) -> Result<CanonicalV3AuthorityEvidence, RemoteRouteFinding> {
    let bytes = read_canonical_authority_selector(repo_root)?;
    let selector_digest = stable_digest(&[std::str::from_utf8(&bytes).map_err(|_| {
        remote_finding(
            "canonical_authority_selector_invalid",
            "canonical authority selector must be UTF-8 JSON",
        )
    })?]);
    if expected_lifecycle_digest
        .is_some_and(|expected| expected.trim().is_empty() || expected != selector_digest)
    {
        return Err(remote_finding(
            "canonical_authority_selector_digest_mismatch",
            "expected lifecycle digest does not match the canonical authority selector",
        ));
    }
    let selector: CanonicalAuthoritySelector = serde_json::from_slice(&bytes).map_err(|_| {
        remote_finding(
            "canonical_authority_selector_invalid",
            "canonical authority selector is not valid typed JSON",
        )
    })?;
    let authority_issue = selector.authority_issue.unwrap_or_default();
    let approved_sha = selector.exact_review_sha.as_deref().unwrap_or_default();
    let readiness_digest = selector
        .readiness_evidence_digest
        .as_deref()
        .unwrap_or_default();
    let approval_digest = selector
        .approval_evidence_digest
        .as_deref()
        .unwrap_or_default();
    if selector.schema.trim().is_empty()
        || selector.default_generation != "v3"
        || selector.operational_authority.as_deref() != Some("csdlc-v3")
        || authority_issue != 505
        || exact_review_sha.trim().is_empty()
        || approved_sha != exact_review_sha
        || readiness_digest.trim().is_empty()
        || approval_digest.trim().is_empty()
    {
        return Err(remote_finding(
            "canonical_v3_authority_inactive",
            "canonical selector does not bind v3 authority to #505, exact review SHA, readiness evidence, and approval evidence",
        ));
    }
    Ok(CanonicalV3AuthorityEvidence {
        schema: "csdlc.v3.canonical_authority_evidence.v1".into(),
        selector_path: CANONICAL_AUTHORITY_SELECTOR_PATH.into(),
        selector_digest,
        authority_issue,
        exact_review_sha: approved_sha.to_owned(),
        readiness_evidence_digest: readiness_digest.to_owned(),
        approval_evidence_digest: approval_digest.to_owned(),
    })
}

fn mutation_credential_name(request: &GithubMutationRequest) -> Result<String, RemoteRouteFinding> {
    match request.credential_names.as_slice() {
        [name] if !name.trim().is_empty() => Ok(name.clone()),
        [] => Err(remote_finding(
            "github_credential_missing",
            "authenticated GitHub mutation requires exactly one credential name",
        )),
        _ => Err(remote_finding(
            "github_credential_ambiguous",
            "authenticated GitHub mutation requires one unambiguous credential name",
        )),
    }
}

fn github_mutation_intent_digest(intent: &GithubMutationIntent) -> String {
    stable_digest(&[
        &intent.schema,
        &intent.operation_digest,
        &intent.operation_marker,
        &intent.authority_selector_digest,
        &intent.adapter,
    ])
}

fn github_mutation_reconciliation_digest(
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

fn validate_mutation(request: &GithubMutationRequest) -> Result<(), RemoteRouteFinding> {
    if request.issue == 0 {
        return Err(remote_finding(
            "github_issue_invalid",
            "issue number must be non-zero",
        ));
    }
    if request.expected_head_sha.trim().is_empty() {
        return Err(remote_finding(
            "github_expected_head_missing",
            "GitHub mutation requires the canonical exact review SHA",
        ));
    }
    match &request.mutation {
        GithubMutation::IssueComment { body } if body.trim().is_empty() => Err(remote_finding(
            "github_body_missing",
            "issue comment body must not be empty",
        )),
        GithubMutation::IssueEdit { title, body }
            if (title.is_none() && body.is_none()) || body.is_none() =>
        {
            Err(remote_finding(
                "github_issue_edit_marker_body_missing",
                "issue edit requires a body so authenticated readback can bind the operation marker",
            ))
        }
        GithubMutation::PullRequestCreate {
            base, head, title, ..
        } if base.trim().is_empty()
            || head.trim().is_empty()
            || title.trim().is_empty()
            || request.pull_request.is_some() =>
        {
            Err(remote_finding(
                "github_pr_create_invalid",
                "PR create requires non-empty base/head/title and no existing PR number",
            ))
        }
        GithubMutation::PullRequestUpdate { title, body }
            if request.pull_request.is_none() || (title.is_none() && body.is_none()) =>
        {
            Err(remote_finding(
                "github_pr_update_invalid",
                "PR update requires a PR number and title or body",
            ))
        }
        GithubMutation::PullRequestReady if request.pull_request.is_none() => Err(remote_finding(
            "github_pr_ready_invalid",
            "PR ready requires a PR number",
        )),
        _ => Ok(()),
    }
}

fn github_mutation_invocation(
    request: &GithubMutationRequest,
    input_path: &Path,
) -> Result<CommandInvocation, RemoteRouteFinding> {
    let endpoint = match request.mutation {
        GithubMutation::IssueComment { .. } => format!(
            "repos/{}/issues/{}/comments",
            request.repository, request.issue
        ),
        GithubMutation::IssueEdit { .. } => {
            format!("repos/{}/issues/{}", request.repository, request.issue)
        }
        GithubMutation::PullRequestCreate { .. } => format!("repos/{}/pulls", request.repository),
        GithubMutation::PullRequestUpdate { .. } => format!(
            "repos/{}/pulls/{}",
            request.repository,
            request.pull_request.unwrap_or_default()
        ),
        GithubMutation::PullRequestReady => {
            return CommandInvocation::new(
                GITHUB_OPERATIONAL_ADAPTER,
                [
                    "POST".into(),
                    format!(
                        "repos/{}/pulls/{}/ready_for_review",
                        request.repository,
                        request.pull_request.unwrap_or_default()
                    ),
                    input_path.to_string_lossy().into_owned(),
                ],
            )
            .map_err(|_| {
                remote_finding(
                    "github_mutation_invocation_rejected",
                    "GitHub mutation must use structured argv",
                )
            })
        }
    };
    let method = if matches!(
        request.mutation,
        GithubMutation::IssueEdit { .. } | GithubMutation::PullRequestUpdate { .. }
    ) {
        "PATCH"
    } else {
        "POST"
    };
    CommandInvocation::new(
        GITHUB_OPERATIONAL_ADAPTER,
        [
            method.into(),
            endpoint,
            input_path.to_string_lossy().into_owned(),
        ],
    )
    .map_err(|_| {
        remote_finding(
            "github_mutation_invocation_rejected",
            "GitHub mutation must use structured argv and a private JSON input file",
        )
    })
}

fn write_mutation_input(
    repo_root: &Path,
    digest: &str,
    operation_marker: &str,
    request: &GithubMutationRequest,
) -> Result<PathBuf, RemoteRouteFinding> {
    let dir = git_control_dir(repo_root)
        .ok_or_else(|| {
            remote_finding(
                "git_control_dir_unavailable",
                "Git control directory is required for private mutation input",
            )
        })?
        .join("csdlc-v3/runtime");
    fs::create_dir_all(&dir).map_err(|_| {
        remote_finding(
            "github_mutation_input_failed",
            "private mutation directory could not be created",
        )
    })?;
    let path = dir.join(format!("github-mutation-{digest}.json"));
    let value = match &request.mutation {
        GithubMutation::IssueComment { body } => {
            serde_json::json!({"body": body_with_operation_marker(body, operation_marker)})
        }
        GithubMutation::IssueEdit { title, body }
        | GithubMutation::PullRequestUpdate { title, body } => {
            serde_json::json!({
                "title": title,
                "body": body.as_ref().map(|body| body_with_operation_marker(body, operation_marker))
            })
        }
        GithubMutation::PullRequestCreate {
            base,
            head,
            title,
            body,
            draft,
        } => {
            serde_json::json!({
                "base": base,
                "head": head,
                "title": title,
                "body": body_with_operation_marker(body, operation_marker),
                "draft": draft
            })
        }
        GithubMutation::PullRequestReady => serde_json::json!({}),
    };
    let bytes = serde_json::to_vec(&value).map_err(|_| {
        remote_finding(
            "github_mutation_input_failed",
            "mutation input could not be serialized",
        )
    })?;
    write_private_create_new(&path, &bytes)?;
    Ok(path)
}

fn body_with_operation_marker(body: &str, operation_marker: &str) -> String {
    if body.contains(operation_marker) {
        body.to_owned()
    } else if body.is_empty() {
        operation_marker.to_owned()
    } else {
        format!("{body}\n\n{operation_marker}")
    }
}

fn validate_mutation_response(
    request: &GithubMutationRequest,
    stdout: &str,
) -> Result<(), RemoteRouteFinding> {
    if matches!(request.mutation, GithubMutation::PullRequestReady) {
        return Ok(());
    }
    let value: serde_json::Value = serde_json::from_str(stdout).map_err(|_| {
        remote_finding(
            "github_mutation_invalid_json",
            "GitHub mutation returned non-JSON output",
        )
    })?;
    match request.mutation {
        GithubMutation::IssueComment { .. } if value["id"].as_u64().is_none() => {
            Err(remote_finding(
                "github_comment_readback_missing",
                "created comment response did not include its immutable id",
            ))
        }
        GithubMutation::IssueEdit { .. } if value["number"].as_u64() != Some(request.issue) => {
            Err(remote_finding(
                "github_issue_readback_mismatch",
                "edited issue response did not match the requested issue",
            ))
        }
        GithubMutation::PullRequestCreate { .. } if value["number"].as_u64().is_none() => {
            Err(remote_finding(
                "github_pr_readback_missing",
                "created PR response did not include its number",
            ))
        }
        GithubMutation::PullRequestUpdate { .. }
            if value["number"].as_u64() != request.pull_request =>
        {
            Err(remote_finding(
                "github_pr_readback_mismatch",
                "updated PR response did not match the requested PR",
            ))
        }
        _ => Ok(()),
    }
}

fn reconcile_github_mutation(
    request: &GithubMutationRequest,
    operation_digest: &str,
    operation_marker: &str,
    process: &mut impl ProcessAdapter,
) -> Result<(GithubMutationReconciliationReceipt, CommandInvocation), RemoteRouteFinding> {
    let credential_name = mutation_credential_name(request)?;
    let invocation = github_mutation_reconciliation_invocation(request)?
        .with_child_credential(credential_name)
        .map_err(|_| {
            remote_finding(
                "github_credential_scope_invalid",
                "GitHub credential name is not safe for child-process injection",
            )
        })?;
    let output = process.run(invocation.clone());
    if output.truncated || output.status != ProcessStatus::Exit(0) {
        return Err(remote_finding(
            "github_mutation_reconciliation_unavailable",
            "authenticated GitHub readback did not complete; durable intent prevents mutation replay",
        ));
    }
    let value: serde_json::Value = serde_json::from_str(&output.stdout).map_err(|_| {
        remote_finding(
            "github_mutation_reconciliation_invalid_json",
            "authenticated GitHub reconciliation returned non-JSON output",
        )
    })?;
    let (pull_request, remote_object_id) =
        match_reconciled_mutation(request, operation_marker, &value)?;
    let canonical = serde_json::to_string(&value).map_err(|_| {
        remote_finding(
            "github_mutation_reconciliation_invalid_json",
            "authenticated GitHub reconciliation could not be canonicalized",
        )
    })?;
    Ok((
        GithubMutationReconciliationReceipt {
            schema: "csdlc.v3.github_mutation_reconciliation.v1".into(),
            operation_digest: operation_digest.to_owned(),
            operation_marker: operation_marker.to_owned(),
            repository: request.repository.clone(),
            issue: request.issue,
            pull_request,
            remote_object_id,
            expected_head_sha: request.expected_head_sha.clone(),
            readback_digest: stable_digest(&[&canonical]),
            observed_by: GITHUB_READ_ONLY_ADAPTER.into(),
            authenticated: true,
        },
        invocation,
    ))
}

fn github_mutation_reconciliation_invocation(
    request: &GithubMutationRequest,
) -> Result<CommandInvocation, RemoteRouteFinding> {
    let argv = match &request.mutation {
        GithubMutation::IssueComment { .. } => vec![
            "issue-comments".into(),
            request.repository.clone(),
            request.issue.to_string(),
        ],
        GithubMutation::IssueEdit { .. } => vec![
            "issue".into(),
            request.repository.clone(),
            request.issue.to_string(),
        ],
        GithubMutation::PullRequestCreate { head, .. } => vec![
            "pull-requests-by-head".into(),
            request.repository.clone(),
            head.clone(),
        ],
        GithubMutation::PullRequestUpdate { .. } | GithubMutation::PullRequestReady => vec![
            "pull-request".into(),
            request.repository.clone(),
            request.pull_request.unwrap_or_default().to_string(),
        ],
    };
    CommandInvocation::new(GITHUB_READ_ONLY_ADAPTER, argv).map_err(|_| {
        remote_finding(
            "github_reconciliation_invocation_rejected",
            "GitHub reconciliation must use structured argv",
        )
    })
}

fn match_reconciled_mutation(
    request: &GithubMutationRequest,
    operation_marker: &str,
    value: &serde_json::Value,
) -> Result<(Option<u64>, Option<u64>), RemoteRouteFinding> {
    let candidates = github_readback_candidates(value);
    let matched = candidates
        .into_iter()
        .find(|candidate| match &request.mutation {
            GithubMutation::IssueComment { body } => {
                candidate["id"].as_u64().is_some()
                    && candidate["body"].as_str()
                        == Some(body_with_operation_marker(body, operation_marker).as_str())
            }
            GithubMutation::IssueEdit { title, body } => {
                candidate["number"].as_u64() == Some(request.issue)
                    && title
                        .as_ref()
                        .is_none_or(|title| candidate["title"].as_str() == Some(title))
                    && body.as_ref().is_some_and(|body| {
                        candidate["body"].as_str()
                            == Some(body_with_operation_marker(body, operation_marker).as_str())
                    })
            }
            GithubMutation::PullRequestCreate {
                base,
                head,
                title,
                body,
                draft,
            } => {
                candidate["number"].as_u64().is_some()
                    && candidate["head"]["sha"].as_str() == Some(request.expected_head_sha.as_str())
                    && candidate["head"]["ref"].as_str() == Some(head.as_str())
                    && candidate["base"]["ref"].as_str() == Some(base.as_str())
                    && candidate["title"].as_str() == Some(title.as_str())
                    && candidate["body"].as_str()
                        == Some(body_with_operation_marker(body, operation_marker).as_str())
                    && candidate["draft"].as_bool() == Some(*draft)
            }
            GithubMutation::PullRequestUpdate { title, body } => {
                candidate["number"].as_u64() == request.pull_request
                    && candidate["head"]["sha"].as_str() == Some(request.expected_head_sha.as_str())
                    && title
                        .as_ref()
                        .is_none_or(|title| candidate["title"].as_str() == Some(title))
                    && body.as_ref().is_none_or(|body| {
                        candidate["body"].as_str()
                            == Some(body_with_operation_marker(body, operation_marker).as_str())
                    })
            }
            GithubMutation::PullRequestReady => {
                candidate["number"].as_u64() == request.pull_request
                    && candidate["head"]["sha"].as_str() == Some(request.expected_head_sha.as_str())
                    && candidate["draft"].as_bool() == Some(false)
            }
        });
    let Some(matched) = matched else {
        return Err(remote_finding(
            "github_mutation_not_reconciled",
            "authenticated readback did not contain the exact operation marker and expected state",
        ));
    };
    let pull_request = match request.mutation {
        GithubMutation::PullRequestCreate { .. }
        | GithubMutation::PullRequestUpdate { .. }
        | GithubMutation::PullRequestReady => matched["number"].as_u64(),
        _ => None,
    };
    Ok((pull_request, matched["id"].as_u64().or(pull_request)))
}

fn github_readback_candidates(value: &serde_json::Value) -> Vec<&serde_json::Value> {
    if let Some(values) = value.as_array() {
        return values.iter().collect();
    }
    for key in ["items", "comments", "pull_requests"] {
        if let Some(values) = value[key].as_array() {
            return values.iter().collect();
        }
    }
    vec![value]
}

fn github_mutation_receipt_path(
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

fn github_mutation_intent_path(
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

fn load_mutation_intent(
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
    if intent.schema != "csdlc.v3.github_mutation_intent.v1"
        || intent.operation_digest != operation_digest
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

fn load_mutation_receipt(
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

fn finalize_mutation_receipt(
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
        issue: request.issue,
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

fn persist_json_create_new(path: &Path, value: &impl Serialize) -> Result<(), RemoteRouteFinding> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|_| {
        remote_finding(
            "receipt_serialization_failed",
            "typed receipt could not be serialized",
        )
    })?;
    write_private_create_new(path, &bytes)
}

fn write_private_create_new(path: &Path, bytes: &[u8]) -> Result<(), RemoteRouteFinding> {
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

fn typed_review_receipt_matches(
    request: &RemoteRouteRequest,
    receipt: Option<&TypedReviewReceipt>,
) -> bool {
    let Some(receipt) = receipt else {
        return false;
    };
    receipt.schema == "csdlc.v3.typed_review_receipt.v1"
        && receipt.repository == request.repository
        && receipt.issue == request.issue
        && Some(receipt.implementer.as_str()) == request.implementer.as_deref()
        && Some(receipt.reviewer.as_str()) == request.reviewer.as_deref()
        && Some(receipt.reviewed_revision.as_str()) == request.review_revision.as_deref()
        && Some(receipt.expected_head_sha.as_str()) == request.expected_head_sha.as_deref()
        && !receipt.evidence_digest.trim().is_empty()
        && request.typed_review_receipt_digest.as_deref()
            == Some(typed_review_receipt_payload_digest(receipt).as_str())
}

fn github_readback_receipt_matches(
    request: &RemoteRouteRequest,
    receipt: Option<&GithubReadbackReceipt>,
) -> bool {
    let Some(receipt) = receipt else {
        return false;
    };
    receipt.schema == "csdlc.v3.github_readback_receipt.v1"
        && receipt.repository == request.repository
        && receipt.issue == request.issue
        && Some(receipt.pull_request) == request.pull_request
        && title_matches(receipt.title.as_deref(), request.title.as_deref())
        && Some(receipt.head_sha.as_str()) == request.head_sha.as_deref()
        && receipt.closes_issue == request.closes_issue
        && receipt.closing_issues == request.closing_issues
        && receipt.part_of_issue == request.part_of_issue
        && receipt.source == RemoteReadbackSource::Github
        && receipt.observed_by == GITHUB_READ_ONLY_ADAPTER
        && request.readback_receipt_digest.as_deref()
            == Some(github_readback_receipt_payload_digest(receipt).as_str())
}

fn title_matches(receipt_title: Option<&str>, request_title: Option<&str>) -> bool {
    let Some(receipt_title) = receipt_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
    else {
        return false;
    };
    let Some(request_title) = request_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
    else {
        return false;
    };
    receipt_title == request_title
}

fn github_adapter_receipt_matches(
    request: &RemoteRouteRequest,
    receipts: &RemoteRouteReceipts,
) -> bool {
    let Some(adapter) = receipts.adapter.as_ref() else {
        return false;
    };
    let Some(readback) = receipts.github_readback.as_ref() else {
        return false;
    };
    let readback_digest = github_readback_receipt_payload_digest(readback);
    adapter.schema == "csdlc.v3.github_adapter_receipt.v1"
        && adapter.repository == request.repository
        && adapter.issue == request.issue
        && Some(adapter.pull_request) == request.pull_request
        && Some(adapter.head_sha.as_str()) == request.head_sha.as_deref()
        && adapter.readback_receipt_digest == readback_digest
        && adapter.credential_names == request.credential_names
        && !adapter.credential_names.is_empty()
        && adapter.adapter == GITHUB_READ_ONLY_ADAPTER
        && adapter.authenticated
        && request.adapter_receipt_digest.as_deref()
            == Some(github_adapter_receipt_payload_digest(adapter).as_str())
}

fn single_credential_name(request: &RemoteRouteRequest) -> Result<String, RemoteRouteFinding> {
    match request.credential_names.as_slice() {
        [name] => Ok(name.clone()),
        [] => Err(remote_finding(
            "github_credential_missing",
            "authenticated GitHub observation requires exactly one credential name",
        )),
        _ => Err(remote_finding(
            "github_credential_ambiguous",
            "authenticated GitHub observation requires one unambiguous credential name",
        )),
    }
}

fn validate_repository_name(repository: &str) -> Result<(), RemoteRouteFinding> {
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

pub fn load_remote_route_receipts(
    repo_root: &Path,
    request: &RemoteRouteRequest,
) -> Result<RemoteRouteReceipts, RemoteRouteFinding> {
    Ok(RemoteRouteReceipts {
        typed_review: load_optional_receipt(
            repo_root,
            request.typed_review_receipt_path.as_deref(),
            "typed_review_receipt_missing",
        )?,
        github_readback: load_optional_receipt(
            repo_root,
            request.readback_receipt_path.as_deref(),
            "github_readback_receipt_missing",
        )?,
        adapter: load_optional_receipt(
            repo_root,
            request.adapter_receipt_path.as_deref(),
            "github_adapter_receipt_missing",
        )?,
    })
}

fn load_optional_receipt<T: for<'de> Deserialize<'de>>(
    repo_root: &Path,
    path: Option<&str>,
    code: &str,
) -> Result<Option<T>, RemoteRouteFinding> {
    let Some(path) = path else {
        return Ok(None);
    };
    let root = repo_root.canonicalize().map_err(|_| {
        remote_finding(
            "repository_root_unavailable",
            "repository root must be canonical before loading receipts",
        )
    })?;
    let candidate = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        root.join(path)
    };
    let canonical = candidate
        .canonicalize()
        .map_err(|_| remote_finding(code, "declared receipt file is missing"))?;
    if !is_repo_or_git_receipt_path(&root, &canonical) {
        return Err(remote_finding(
            "receipt_path_escapes_repository",
            "receipt paths must canonicalize beneath the repository root or resolved Git receipt directory",
        ));
    }
    if !is_durable_receipt_path(&root, &canonical) {
        return Err(remote_finding(
            "receipt_path_not_durable",
            "receipt paths must live under .csdlc/evidence or .git/csdlc-v3",
        ));
    }
    let bytes = std::fs::read(&canonical)
        .map_err(|_| remote_finding(code, "declared receipt file is not readable"))?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|_| remote_finding(code, "declared receipt file is not valid typed JSON"))
}

fn is_durable_receipt_path(root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(root.join(".csdlc/evidence"))
        || git_control_dir(root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

fn is_repo_or_git_receipt_path(root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(root)
        || git_control_dir(root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

fn git_control_dir(root: &Path) -> Option<PathBuf> {
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
    if let Some(common_dir) = git_common_dir(&git_dir) {
        return Some(common_dir);
    }
    Some(git_dir)
}

fn git_common_dir(git_dir: &Path) -> Option<PathBuf> {
    let contents = std::fs::read_to_string(git_dir.join("commondir")).ok()?;
    let path = Path::new(contents.trim());
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        git_dir.join(path)
    };
    path.canonicalize().ok()
}

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
