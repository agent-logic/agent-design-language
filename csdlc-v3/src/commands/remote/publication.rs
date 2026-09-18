//! Review, publication and authenticated PR-readback admission.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};
use serde::Deserialize;

use super::model::*;
use super::storage::{load_mutation_intent, load_mutation_receipt};
use super::support::{
    git_control_dir, github_mutation_intent_digest, github_mutation_operation_digest,
    github_mutation_operation_marker, github_mutation_reconciliation_digest, remote_finding,
    stable_digest, validate_repository_name, GITHUB_READ_ONLY_ADAPTER,
};

/// Observe retained remote uncertainty without reconciling, retrying, or
/// creating receipts. A successful PR readback does not settle a pending write.
pub fn pending_mutation_finding(
    repo_root: &Path,
    request: &RemoteRouteRequest,
) -> Result<(), RemoteRouteFinding> {
    let control = git_control_dir(repo_root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git metadata is required to inspect retained remote operations",
        )
    })?;
    let remote = control.join("csdlc-v3/remote");
    for path in [&remote, &remote.join("mutations")] {
        if path
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.is_dir())
        {
            return Err(remote_finding(
                "remote_intent_inventory_invalid",
                "remote evidence directories must not redirect observation",
            ));
        }
    }
    for (directory, suffix) in [("intents", ".json"), ("merges", ".intent.json")] {
        let dir = remote.join(directory);
        let metadata = match dir.symlink_metadata() {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => {
                return Err(remote_finding(
                    "remote_intent_inventory_unreadable",
                    "retained remote inventory cannot be inspected",
                ))
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(remote_finding(
                "remote_intent_inventory_invalid",
                "retained remote inventory must be a real directory",
            ));
        }
        let mut paths = fs::read_dir(&dir)
            .map_err(|_| {
                remote_finding(
                    "remote_intent_inventory_unreadable",
                    "retained remote inventory cannot be inspected",
                )
            })?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                remote_finding(
                    "remote_intent_inventory_unreadable",
                    "retained remote inventory cannot be inspected",
                )
            })?;
        paths.sort();
        for path in paths {
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(digest) = name.strip_suffix(suffix) else {
                continue;
            };
            if path
                .symlink_metadata()
                .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
            {
                return Err(remote_finding(
                    "remote_intent_file_invalid",
                    "retained remote intent must be a regular file",
                ));
            }
            let (operation, intent_digest) = if directory == "intents" {
                let intent = load_mutation_intent(&path, digest)?;
                let hash = github_mutation_intent_digest(&intent);
                (intent.request, hash)
            } else {
                let value: serde_json::Value =
                    serde_json::from_slice(&fs::read(&path).map_err(|_| {
                        remote_finding(
                            "remote_merge_intent_unreadable",
                            "retained merge intent cannot be read",
                        )
                    })?)
                    .map_err(|_| {
                        remote_finding(
                            "remote_merge_intent_invalid",
                            "retained merge intent is invalid",
                        )
                    })?;
                let operation: GithubMutationRequest =
                    serde_json::from_value(value["request"].clone()).map_err(|_| {
                        remote_finding(
                            "remote_merge_intent_invalid",
                            "retained merge request is invalid",
                        )
                    })?;
                if value["schema"] != "csdlc.v3.merge_intent.v1"
                    || github_mutation_operation_digest(&operation) != digest
                    || !matches!(operation.mutation, GithubMutation::PullRequestMerge { .. })
                {
                    return Err(remote_finding(
                        "remote_merge_intent_mismatch",
                        "retained merge identity does not match its operation",
                    ));
                }
                let hash = stable_digest(&[&serde_json::to_string(&value).map_err(|_| {
                    remote_finding(
                        "remote_merge_intent_invalid",
                        "retained merge intent cannot be encoded",
                    )
                })?]);
                (operation, hash)
            };
            if operation.repository != request.repository || operation.issue != request.issue {
                continue;
            }
            let receipt_path = remote.join("mutations").join(format!("{digest}.json"));
            if !receipt_path.exists() {
                return Err(remote_finding("remote_mutation_recovery_required",
                    "a retained remote operation lacks final reconciliation; use its exact native mutation owner to reconcile it explicitly; PR observation never retries the effect or writes a receipt"));
            }
            if receipt_path
                .symlink_metadata()
                .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
            {
                return Err(remote_finding(
                    "remote_mutation_receipt_invalid",
                    "retained reconciliation must be a regular file",
                ));
            }
            let receipt = load_mutation_receipt(&receipt_path, digest)?;
            if receipt.intent_digest != intent_digest
                || receipt.repository != operation.repository
                || receipt.issue != operation.issue
                || receipt.expected_head_sha != operation.expected_head_sha
                || operation
                    .pull_request
                    .is_some_and(|pr| receipt.pull_request != Some(pr))
                || (matches!(operation.mutation, GithubMutation::PullRequestCreate { .. })
                    && receipt.pull_request.is_none_or(|pr| pr == 0))
            {
                return Err(remote_finding(
                    "remote_mutation_receipt_mismatch",
                    "retained reconciliation does not settle the exact remote intent",
                ));
            }
            if directory == "merges" {
                let path = dir.join(format!("{digest}.reconciliation.json"));
                let invalid = || {
                    remote_finding(
                    "remote_merge_reconciliation_invalid",
                    "retained merge reconciliation must authenticate the exact operation and merge identity",
                )
                };
                let metadata = path.symlink_metadata().map_err(|_| invalid())?;
                if !metadata.is_file() || metadata.file_type().is_symlink() {
                    return Err(invalid());
                }
                let reconciliation: GithubMutationReconciliationReceipt =
                    serde_json::from_slice(&fs::read(&path).map_err(|_| invalid())?)
                        .map_err(|_| invalid())?;
                let identity = reconciliation.merge.as_ref().ok_or_else(invalid)?;
                let identity_digest =
                    stable_digest(&[&serde_json::to_string(identity).map_err(|_| invalid())?]);
                if reconciliation.schema != "csdlc.v3.github_mutation_reconciliation.v1"
                    || reconciliation.operation_digest != digest
                    || reconciliation.operation_marker != github_mutation_operation_marker(digest)
                    || reconciliation.repository != operation.repository
                    || reconciliation.issue != operation.issue
                    || reconciliation.pull_request != operation.pull_request
                    || reconciliation.remote_object_id != operation.pull_request
                    || reconciliation.expected_head_sha != operation.expected_head_sha
                    || reconciliation.observed_by != GITHUB_READ_ONLY_ADAPTER
                    || !reconciliation.authenticated
                    || reconciliation.readback_digest != identity_digest
                    || receipt.readback_digest.as_deref() != Some(identity_digest.as_str())
                    || receipt.reconciliation_digest
                        != github_mutation_reconciliation_digest(&reconciliation)
                    || identity.repository != operation.repository
                    || Some(identity.pull_request) != operation.pull_request
                    || identity.head_sha != operation.expected_head_sha
                {
                    return Err(invalid());
                }
            }
        }
    }
    Ok(())
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

pub(super) fn publication_findings(
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

pub(super) fn pr_state_findings(
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

pub(super) fn review_findings(request: &RemoteRouteRequest) -> Vec<RemoteRouteFinding> {
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

pub(super) fn same_principal(left: Option<&str>, right: Option<&str>) -> bool {
    let left = left.unwrap_or_default().trim();
    let right = right.unwrap_or_default().trim();
    !left.is_empty() && left.eq_ignore_ascii_case(right)
}

/// Shared preparation/amendment/publication closing-relation admission.
pub fn publication_body_is_valid(body: &str, issue: u64) -> bool {
    body_has_relation(Some(body), "Closes", issue)
        && body_closing_issue_references(Some(body))
            .iter()
            .all(|other| *other == issue)
}

pub(super) fn body_has_relation(body: Option<&str>, verb: &str, issue: u64) -> bool {
    let prefix = format!("{verb} #{issue}");
    body.unwrap_or_default()
        .lines()
        .any(|line| has_issue_relation_prefix(line.trim_start(), &prefix))
}

pub(super) fn body_closing_issue_references(body: Option<&str>) -> Vec<u64> {
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

pub(super) fn word_boundary_before(text: &str, index: usize) -> bool {
    index == 0
        || text[..index]
            .chars()
            .next_back()
            .is_none_or(|ch| !ch.is_ascii_alphanumeric() && ch != '_')
}

pub(super) fn word_boundary_after(text: &str, index: usize) -> bool {
    index == text.len()
        || text[index..]
            .chars()
            .next()
            .is_none_or(|ch| !ch.is_ascii_alphanumeric() && ch != '_')
}

pub(super) fn parse_issue_after_closing_keyword(rest: &str) -> Option<u64> {
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

pub(super) fn has_issue_relation_prefix(line: &str, prefix: &str) -> bool {
    let Some(rest) = line.strip_prefix(prefix) else {
        return false;
    };
    rest.is_empty()
        || rest
            .chars()
            .next()
            .is_some_and(|ch| ch.is_whitespace() || matches!(ch, ',' | '.' | ';' | ':' | ')' | ']'))
}

pub fn typed_review_receipt_payload_digest(receipt: &TypedReviewReceipt) -> String {
    let legacy = stable_digest(&[
        &receipt.schema,
        &receipt.repository,
        &receipt.issue.to_string(),
        &receipt.implementer,
        &receipt.reviewer,
        &receipt.reviewed_revision,
        &receipt.expected_head_sha,
        &receipt.evidence_digest,
    ]);
    match &receipt.publication_linkage {
        Some(linkage) => stable_digest(&[
            &legacy,
            &linkage.repository,
            &linkage.issue.to_string(),
            match linkage.mode {
                RemotePublicationMode::Closing => "closing",
                RemotePublicationMode::PartOf => "part_of",
            },
        ]),
        None => legacy,
    }
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

pub(super) fn typed_review_receipt_matches(
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

pub(super) fn github_readback_receipt_matches(
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

pub(super) fn title_matches(receipt_title: Option<&str>, request_title: Option<&str>) -> bool {
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

pub(super) fn github_adapter_receipt_matches(
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

pub(super) fn single_credential_name(
    request: &RemoteRouteRequest,
) -> Result<String, RemoteRouteFinding> {
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

pub(super) fn load_optional_receipt<T: for<'de> Deserialize<'de>>(
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

pub(super) fn is_durable_receipt_path(root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(root.join(".csdlc/evidence"))
        || git_control_dir(root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

pub(super) fn is_repo_or_git_receipt_path(root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(root)
        || git_control_dir(root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

pub fn validate_publication_metadata(
    issue: u64,
    branch: &str,
    base: &str,
    title: &str,
    body: &str,
) -> Result<(), RemoteRouteFinding> {
    if !publication_body_is_valid(body, issue) {
        return Err(remote_finding(
            "intent_publication_body_invalid",
            "publication body must preserve the canonical closing issue",
        ));
    }
    let valid_base = !base.is_empty()
        && !base.starts_with('-')
        && !base.ends_with('.')
        && !base.contains("..")
        && base
            .split('/')
            .all(|part| !part.is_empty() && !part.starts_with('.') && !part.ends_with(".lock"))
        && base
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/_-.".contains(&b));
    if issue == 0
        || !valid_base
        || base == branch
        || title.trim().is_empty()
        || title.contains(['\n', '\r', '\0'])
        || body.contains('\0')
        || !body_has_relation(Some(body), "Closes", issue)
        || body_closing_issue_references(Some(body))
            .iter()
            .any(|other| *other != issue)
    {
        return Err(remote_finding("intent_publication_metadata_invalid",
            "publication requires a safe distinct base, nonempty title and only the canonical closing issue on its own line"));
    }
    Ok(())
}
