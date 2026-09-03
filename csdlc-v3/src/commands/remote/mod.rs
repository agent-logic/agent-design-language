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
use std::path::{Path, PathBuf};

const GITHUB_READ_ONLY_ADAPTER: &str = "github-api-read-only";

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
        && Some(receipt.head_sha.as_str()) == request.head_sha.as_deref()
        && receipt.closes_issue == request.closes_issue
        && receipt.closing_issues == request.closing_issues
        && receipt.part_of_issue == request.part_of_issue
        && receipt.source == RemoteReadbackSource::Github
        && receipt.observed_by == GITHUB_READ_ONLY_ADAPTER
        && request.readback_receipt_digest.as_deref()
            == Some(github_readback_receipt_payload_digest(receipt).as_str())
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
