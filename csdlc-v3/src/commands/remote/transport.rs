//! Credential-scoped GitHub transport, response validation and authenticated reconciliation.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};

use super::model::*;
use super::storage::*;
use super::support::{
    exact_issue_names, git_control_dir, remote_finding, same_names, stable_digest,
    GITHUB_OPERATIONAL_ADAPTER, GITHUB_READ_ONLY_ADAPTER,
};

pub(super) fn mutation_credential_name(
    request: &GithubMutationRequest,
) -> Result<String, RemoteRouteFinding> {
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

pub(super) fn preflight_github_credential(
    credential_name: &str,
    process: &mut impl ProcessAdapter,
) -> Result<(), RemoteRouteFinding> {
    let invocation = CommandInvocation::new(GITHUB_OPERATIONAL_ADAPTER, ["credential-preflight"])
        .and_then(|invocation| invocation.with_child_credential(credential_name.to_owned()))
        .map_err(|_| {
            remote_finding(
                "github_credential_scope_invalid",
                "GitHub credential name is not safe for child-process injection",
            )
        })?;
    process
        .preflight_child_credential(&invocation)
        .map_err(|_| {
            remote_finding(
                "github_credential_unavailable",
                "GitHub credential must resolve before a durable mutation intent is created",
            )
        })
}

pub(super) fn validate_mutation(request: &GithubMutationRequest) -> Result<(), RemoteRouteFinding> {
    if request.issue == 0 && !matches!(request.mutation, GithubMutation::IssueCreate { .. }) {
        return Err(remote_finding(
            "github_issue_invalid",
            "issue number must be non-zero except for issue creation",
        ));
    }
    if request.expected_head_sha.trim().is_empty() {
        return Err(remote_finding(
            "github_expected_head_missing",
            "GitHub mutation requires the canonical exact review SHA",
        ));
    }
    match &request.mutation {
        GithubMutation::IssueCreate { title, body, .. }
            if title.trim().is_empty() || body.trim().is_empty() || request.pull_request.is_some() =>
        {
            Err(remote_finding(
                "github_issue_create_invalid",
                "issue create requires non-empty title/body, issue 0, and no PR number",
            ))
        }
        GithubMutation::IssueCreate { .. } if request.issue != 0 => Err(remote_finding(
            "github_issue_create_target_invalid",
            "issue create must use issue 0 because GitHub assigns the issue number",
        )),
        GithubMutation::IssueComment { body } if body.trim().is_empty() => Err(remote_finding(
            "github_body_missing",
            "issue comment body must not be empty",
        )),
        GithubMutation::IssueEdit { title, body, labels, assignees, milestone } => {
            let names = labels.as_ref().map(|update| match update {
                IssueLabelsUpdate::Replace { names } | IssueLabelsUpdate::Add { names }
                | IssueLabelsUpdate::Remove { names } => names,
            });
            if request.pull_request.is_some()
                || (title.is_none() && body.is_none() && labels.is_none() && assignees.is_none() && milestone.is_none())
                || title.as_ref().is_some_and(|title| title.trim().is_empty())
                || matches!(milestone, Some(IssueMilestoneUpdate::Set { number: 0 }))
                || names.into_iter().chain(assignees.as_ref()).any(|names| names.iter().any(|name| name.trim().is_empty()))
            {
                Err(remote_finding("github_issue_edit_invalid", "issue edit requires a selected field, valid names/milestone and no PR target"))
            } else { Ok(()) }
        }
        GithubMutation::IssueClose {
            rationale,
            current_body,
            disposition,
            duplicate_of,
            github_state_reason,
        } if rationale.trim().is_empty()
            || current_body.contains("<!-- csdlc-v3-operation:")
            || request.pull_request.is_some()
            || matches!(disposition, IssueCloseDisposition::Duplicate) && duplicate_of.is_none()
            || github_state_reason == &Some(IssueCloseStateReason::Completed) =>
        {
            Err(remote_finding(
                "github_issue_close_invalid",
                "issue close requires a non-empty rationale, an unmarked current body, no PR number, duplicate owner for duplicate disposition, and a non-completion state reason",
            ))
        }
        GithubMutation::PullRequestCreate {
            base, head, title, ..
        } if !crate::adapters::supported_pr_branch(base)
            || !crate::adapters::supported_pr_branch(head)
            || title.trim().is_empty()
            || request.pull_request.is_some() =>
        {
            Err(remote_finding(
                "github_pr_create_invalid",
                "PR create requires supported Git branch base/head, non-empty title, and no existing PR number",
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

pub(super) fn github_mutation_invocation(
    request: &GithubMutationRequest,
    input_path: &Path,
) -> Result<CommandInvocation, RemoteRouteFinding> {
    let endpoint = match request.mutation {
        GithubMutation::IssueCreate { .. } => format!("repos/{}/issues", request.repository),
        GithubMutation::IssueComment { .. } => format!(
            "repos/{}/issues/{}/comments",
            request.repository, request.issue
        ),
        GithubMutation::IssueEdit { .. } => {
            format!("repos/{}/issues/{}", request.repository, request.issue)
        }
        GithubMutation::IssueClose { .. } | GithubMutation::IssueCompleteCoordination { .. } => {
            format!("repos/{}/issues/{}", request.repository, request.issue)
        }
        GithubMutation::PullRequestCreate { .. } => format!("repos/{}/pulls", request.repository),
        GithubMutation::PullRequestUpdate { .. } => format!(
            "repos/{}/pulls/{}",
            request.repository,
            request.pull_request.unwrap_or_default()
        ),
        GithubMutation::PullRequestMerge { .. } => {
            return Err(remote_finding(
                "github_merge_route_required",
                "merge requires its guarded owner",
            ))
        }
        GithubMutation::PullRequestReady => {
            return CommandInvocation::new(
                GITHUB_OPERATIONAL_ADAPTER,
                [
                    "GRAPHQL".into(),
                    "mark-pull-request-ready".into(),
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
        GithubMutation::IssueEdit { .. }
            | GithubMutation::IssueClose { .. }
            | GithubMutation::IssueCompleteCoordination { .. }
            | GithubMutation::PullRequestUpdate { .. }
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

pub(super) fn write_mutation_input(
    repo_root: &Path,
    digest: &str,
    operation_marker: &str,
    request: &GithubMutationRequest,
    ready_target: Option<&GithubReadyTarget>,
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
        GithubMutation::IssueCompleteCoordination { completion } => serde_json::json!({
            "state":"closed", "state_reason":"completed",
            "body":body_with_operation_marker(&completion.current_body, operation_marker)
        }),
        GithubMutation::IssueCreate {
            title,
            body,
            labels,
            assignees,
            milestone,
        } => {
            serde_json::json!({
                "title": title,
                "body": body_with_operation_marker(body, operation_marker),
                "labels": labels,
                "assignees": assignees,
                "milestone": milestone
            })
        }
        GithubMutation::IssueComment { body } => {
            serde_json::json!({"body": body_with_operation_marker(body, operation_marker)})
        }
        GithubMutation::IssueEdit {
            title,
            body,
            labels,
            assignees,
            milestone,
        } => {
            let mut value = serde_json::Map::new();
            if let Some(title) = title {
                value.insert("title".into(), serde_json::json!(title));
            }
            if let Some(body) = body {
                value.insert(
                    "body".into(),
                    serde_json::json!(body_with_operation_marker(body, operation_marker)),
                );
            }
            if let Some(IssueLabelsUpdate::Replace { names }) = labels {
                value.insert("labels".into(), serde_json::json!(names));
            }
            if let Some(assignees) = assignees {
                value.insert("assignees".into(), serde_json::json!(assignees));
            }
            if let Some(milestone) = milestone {
                value.insert(
                    "milestone".into(),
                    match milestone {
                        IssueMilestoneUpdate::Set { number } => serde_json::json!(number),
                        IssueMilestoneUpdate::Clear => serde_json::Value::Null,
                    },
                );
            }
            serde_json::Value::Object(value)
        }
        GithubMutation::PullRequestUpdate { title, body } => {
            serde_json::json!({
                "title": title,
                "body": body.as_ref().map(|body| body_with_operation_marker(body, operation_marker))
            })
        }
        GithubMutation::IssueClose {
            rationale,
            current_body,
            disposition,
            duplicate_of,
            github_state_reason,
        } => {
            let state_reason = github_state_reason.unwrap_or(IssueCloseStateReason::NotPlanned);
            serde_json::json!({
                "state": "closed",
                "state_reason": state_reason.as_github_value(),
                "body": body_with_operation_marker(
                    &issue_close_readback_body(current_body, rationale, *disposition, *duplicate_of),
                    operation_marker,
                )
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
        GithubMutation::PullRequestMerge { .. } => {
            return Err(remote_finding(
                "github_merge_route_required",
                "merge requires its guarded owner",
            ))
        }
        GithubMutation::PullRequestReady => {
            let target = ready_target.ok_or_else(|| {
                remote_finding(
                    "github_pr_ready_target_missing",
                    "ready mutation requires an authenticated retained PR node identity",
                )
            })?;
            serde_json::json!({
                "query": "mutation MarkPullRequestReady($pullRequestId: ID!) { markPullRequestReadyForReview(input: {pullRequestId: $pullRequestId}) { pullRequest { number headRefOid isDraft } } }",
                "variables": {"pullRequestId": target.node_id}
            })
        }
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

pub(super) fn issue_close_readback_body(
    current_body: &str,
    rationale: &str,
    disposition: IssueCloseDisposition,
    duplicate_of: Option<u64>,
) -> String {
    let duplicate_line = duplicate_of
        .map(|issue| format!("\nDuplicate owner: #{issue}"))
        .unwrap_or_default();
    let close_section = format!(
        "Closed by native C-SDLC v3 issue-close.\nDisposition: {}{duplicate_line}\nRationale: {}",
        disposition.as_str(),
        rationale.trim()
    );
    if current_body.trim().is_empty() {
        close_section
    } else {
        format!("{}\n\n{}", current_body.trim_end(), close_section)
    }
}

impl IssueCloseDisposition {
    fn as_str(self) -> &'static str {
        match self {
            IssueCloseDisposition::Duplicate => "duplicate",
            IssueCloseDisposition::Superseded => "superseded",
            IssueCloseDisposition::NoOp => "no_op",
        }
    }
}

impl IssueCloseStateReason {
    fn as_github_value(self) -> &'static str {
        match self {
            IssueCloseStateReason::Completed => "completed",
            IssueCloseStateReason::NotPlanned => "not_planned",
        }
    }

    fn matches_readback(self, value: &serde_json::Value) -> bool {
        value
            .as_str()
            .is_none_or(|observed| observed == self.as_github_value())
    }
}

pub(super) fn body_with_operation_marker(body: &str, operation_marker: &str) -> String {
    if body.contains(operation_marker) {
        body.to_owned()
    } else if body.is_empty() {
        operation_marker.to_owned()
    } else {
        format!("{body}\n\n{operation_marker}")
    }
}

pub(super) fn validate_mutation_response(
    request: &GithubMutationRequest,
    stdout: &str,
) -> Result<(), RemoteRouteFinding> {
    let value: serde_json::Value = serde_json::from_str(stdout).map_err(|_| {
        remote_finding(
            "github_mutation_invalid_json",
            "GitHub mutation returned non-JSON output",
        )
    })?;
    if matches!(request.mutation, GithubMutation::PullRequestReady)
        && value.get("errors").is_some_and(|errors| !errors.is_null())
    {
        return Err(remote_finding(
            "github_mutation_rejected",
            "GitHub GraphQL rejected the authenticated ready mutation",
        ));
    }
    match request.mutation {
        GithubMutation::IssueCompleteCoordination { .. }
            if value["number"] != request.issue
                || value["state"] != "closed"
                || value["state_reason"] != "completed" =>
        {
            Err(remote_finding(
                "github_coordination_readback_mismatch",
                "coordination completion response identity or completed state mismatch",
            ))
        }
        GithubMutation::IssueCreate { .. } if value["number"].as_u64().is_none() => {
            Err(remote_finding(
                "github_issue_readback_missing",
                "created issue response did not include its assigned number",
            ))
        }
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
        GithubMutation::IssueClose { .. } if value["number"].as_u64() != Some(request.issue) => {
            Err(remote_finding(
                "github_issue_readback_mismatch",
                "closed issue response did not match the requested issue",
            ))
        }
        GithubMutation::IssueClose { .. } if value["state"].as_str() != Some("closed") => {
            Err(remote_finding(
                "github_issue_close_readback_missing",
                "closed issue response did not report closed state",
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
        GithubMutation::PullRequestReady
            if value["data"]["markPullRequestReadyForReview"]["pullRequest"]["number"].as_u64()
                != request.pull_request
                || value["data"]["markPullRequestReadyForReview"]["pullRequest"]["headRefOid"]
                    .as_str()
                    != Some(request.expected_head_sha.as_str())
                || value["data"]["markPullRequestReadyForReview"]["pullRequest"]["isDraft"]
                    .as_bool()
                    != Some(false) =>
        {
            Err(remote_finding(
                "github_pr_ready_response_mismatch",
                "ready mutation response must bind the exact PR, head, and ready state",
            ))
        }
        _ => Ok(()),
    }
}

pub(super) fn persist_recovery_receipt(
    repo_root: &Path,
    request: &GithubMutationRequest,
    operation_digest: &str,
    intent_digest: &str,
    ready_target: Option<&GithubReadyTarget>,
) -> Result<(), RemoteRouteFinding> {
    let path = ensure_recovery_available(repo_root, operation_digest)?;
    let receipt = GithubMutationRecoveryReceipt {
        schema: "csdlc.v3.github_mutation_recovery.v1".into(),
        operation_digest: operation_digest.into(),
        intent_digest: intent_digest.into(),
        recovery: GithubMutationRecovery::RetryAfterAuthenticatedAbsence,
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request,
        expected_head_sha: request.expected_head_sha.clone(),
        resolved_ready_target: ready_target.cloned(),
    };
    persist_json_create_new(&path, &receipt)
}

pub(super) fn ensure_recovery_available(
    repo_root: &Path,
    operation_digest: &str,
) -> Result<PathBuf, RemoteRouteFinding> {
    let path = github_mutation_recovery_path(repo_root, operation_digest)?;
    if path.exists() {
        return Err(remote_finding(
            "github_mutation_recovery_already_consumed",
            "the single authenticated-absence recovery was already consumed",
        ));
    }
    Ok(path)
}

pub(super) fn reconcile_github_mutation(
    request: &GithubMutationRequest,
    operation_digest: &str,
    operation_marker: &str,
    process: &mut impl ProcessAdapter,
) -> Result<(GithubMutationReconciliationReceipt, CommandInvocation), RemoteRouteFinding> {
    let credential_name = mutation_credential_name(request)?;
    let invocation = github_mutation_reconciliation_invocation(request, operation_digest)?
        .with_child_credential(credential_name)
        .map_err(|_| {
            remote_finding(
                "github_credential_scope_invalid",
                "GitHub credential name is not safe for child-process injection",
            )
        })?;
    let value = if matches!(request.mutation, GithubMutation::IssueComment { .. }) {
        // A full page is not authenticated absence. Scan the whole bounded
        // collection even after a match so duplicate operation markers fail closed.
        let mut comments = Vec::new();
        let mut complete = false;
        for page in 1..=100 {
            let mut argv = invocation.argv().to_vec();
            argv.push(page.to_string());
            let mut page_invocation = CommandInvocation::new(GITHUB_READ_ONLY_ADAPTER, argv)
                .map_err(|_| {
                    remote_finding(
                        "github_reconciliation_invocation_rejected",
                        "invalid comment page invocation",
                    )
                })?;
            page_invocation.credential_scope = invocation.credential_scope.clone();
            let value = read_mutation_reconciliation_page(page_invocation, process)?;
            let page_comments = value
                .as_array()
                .filter(|items| items.len() <= 100)
                .ok_or_else(|| {
                    remote_finding(
                        "github_mutation_reconciliation_invalid_json",
                        "authenticated comment page is not a bounded array",
                    )
                })?;
            comments.extend(page_comments.iter().cloned());
            if page_comments.len() < 100 {
                complete = true;
                break;
            }
        }
        if !complete {
            return Err(remote_finding(
                "github_mutation_reconciliation_incomplete",
                "comment scan reached its page bound; absence and replay remain unauthorized",
            ));
        }
        serde_json::Value::Array(comments)
    } else {
        read_mutation_reconciliation_page(invocation.clone(), process)?
    };
    let (issue, pull_request, remote_object_id) =
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
            issue,
            pull_request,
            remote_object_id,
            expected_head_sha: request.expected_head_sha.clone(),
            readback_digest: stable_digest(&[&canonical]),
            observed_by: GITHUB_READ_ONLY_ADAPTER.into(),
            authenticated: true,
            merge: None,
        },
        invocation,
    ))
}

pub(super) fn read_mutation_reconciliation_page(
    invocation: CommandInvocation,
    process: &mut impl ProcessAdapter,
) -> Result<serde_json::Value, RemoteRouteFinding> {
    let output = process.run(invocation);
    if output.truncated || output.status != ProcessStatus::Exit(0) {
        return Err(remote_finding(
            "github_mutation_reconciliation_unavailable",
            "authenticated GitHub readback did not complete; durable intent prevents mutation replay",
        ));
    }
    serde_json::from_str(&output.stdout).map_err(|_| {
        remote_finding(
            "github_mutation_reconciliation_invalid_json",
            "authenticated GitHub reconciliation returned non-JSON output",
        )
    })
}

pub(super) fn github_mutation_reconciliation_invocation(
    request: &GithubMutationRequest,
    operation_digest: &str,
) -> Result<CommandInvocation, RemoteRouteFinding> {
    let argv = match &request.mutation {
        GithubMutation::IssueCreate { .. } => vec![
            "issues-by-marker".into(),
            request.repository.clone(),
            operation_digest.to_owned(),
        ],
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
        GithubMutation::IssueClose { .. } | GithubMutation::IssueCompleteCoordination { .. } => {
            vec![
                "issue".into(),
                request.repository.clone(),
                request.issue.to_string(),
            ]
        }
        GithubMutation::PullRequestCreate { head, .. } => vec![
            "pull-requests-by-head".into(),
            request.repository.clone(),
            head.clone(),
        ],
        GithubMutation::PullRequestMerge { .. } => {
            return Err(remote_finding(
                "github_merge_route_required",
                "merge requires its guarded owner",
            ))
        }
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

pub(super) fn match_reconciled_mutation(
    request: &GithubMutationRequest,
    operation_marker: &str,
    value: &serde_json::Value,
) -> Result<(u64, Option<u64>, Option<u64>), RemoteRouteFinding> {
    let candidates = github_readback_candidates(value);
    let mut matches = candidates
        .into_iter()
        .filter(|candidate| match &request.mutation {
            GithubMutation::PullRequestMerge { .. } => false,
            GithubMutation::IssueCreate {
                title,
                body,
                labels,
                assignees,
                milestone,
            } => {
                candidate["number"].as_u64().is_some()
                    && candidate["title"].as_str() == Some(title.as_str())
                    && candidate["body"].as_str()
                        == Some(body_with_operation_marker(body, operation_marker).as_str())
                    && json_string_array_contains_all(&candidate["labels"], labels)
                    && json_string_array_contains_all(&candidate["assignees"], assignees)
                    && milestone.is_none_or(|milestone| {
                        candidate["milestone"]["number"].as_u64() == Some(milestone)
                    })
            }
            GithubMutation::IssueComment { body } => {
                candidate["id"].as_u64().is_some()
                    && candidate["body"].as_str()
                        == Some(body_with_operation_marker(body, operation_marker).as_str())
            }
            GithubMutation::IssueEdit {
                title,
                body,
                labels,
                assignees,
                milestone,
            } => {
                candidate["number"].as_u64() == Some(request.issue)
                    && title
                        .as_ref()
                        .is_none_or(|title| candidate["title"].as_str() == Some(title))
                    && body.as_ref().is_some_and(|body| {
                        candidate["body"].as_str()
                            == Some(body_with_operation_marker(body, operation_marker).as_str())
                    })
                    && labels.as_ref().is_none_or(|labels| match labels {
                        IssueLabelsUpdate::Replace { names } => {
                            exact_issue_names(&candidate["labels"], "name")
                                .is_some_and(|actual| same_names(&actual, names))
                        }
                        _ => false,
                    })
                    && assignees.as_ref().is_none_or(|names| {
                        exact_issue_names(&candidate["assignees"], "login")
                            .is_some_and(|actual| same_names(&actual, names))
                    })
                    && milestone.as_ref().is_none_or(|milestone| match milestone {
                        IssueMilestoneUpdate::Set { number } => {
                            candidate["milestone"]["number"].as_u64() == Some(*number)
                        }
                        IssueMilestoneUpdate::Clear => candidate
                            .get("milestone")
                            .is_some_and(serde_json::Value::is_null),
                    })
            }
            GithubMutation::IssueCompleteCoordination { completion } => {
                candidate["number"].as_u64() == Some(request.issue)
                    && candidate["html_url"]
                        == format!(
                            "https://github.com/{}/issues/{}",
                            request.repository, request.issue
                        )
                    && candidate["state"] == "closed"
                    && candidate["state_reason"] == "completed"
                    && candidate["body"].as_str()
                        == Some(
                            body_with_operation_marker(&completion.current_body, operation_marker)
                                .as_str(),
                        )
            }
            GithubMutation::IssueClose {
                rationale,
                current_body,
                disposition,
                duplicate_of,
                github_state_reason,
            } => {
                candidate["number"].as_u64() == Some(request.issue)
                    && candidate["state"].as_str() == Some("closed")
                    && github_state_reason
                        .unwrap_or(IssueCloseStateReason::NotPlanned)
                        .matches_readback(&candidate["state_reason"])
                    && candidate["body"].as_str()
                        == Some(
                            body_with_operation_marker(
                                &issue_close_readback_body(
                                    current_body,
                                    rationale,
                                    *disposition,
                                    *duplicate_of,
                                ),
                                operation_marker,
                            )
                            .as_str(),
                        )
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
    let Some(matched) = matches.next() else {
        return Err(remote_finding(
            "github_mutation_not_reconciled",
            "authenticated readback did not contain the exact operation marker and expected state",
        ));
    };
    if matches!(request.mutation, GithubMutation::IssueComment { .. }) && matches.next().is_some() {
        return Err(remote_finding(
            "github_mutation_reconciliation_ambiguous",
            "multiple comments match the exact operation; replay remains unauthorized",
        ));
    }
    let pull_request = match request.mutation {
        GithubMutation::PullRequestCreate { .. }
        | GithubMutation::PullRequestUpdate { .. }
        | GithubMutation::PullRequestReady => matched["number"].as_u64(),
        _ => None,
    };
    let issue = if matches!(request.mutation, GithubMutation::IssueCreate { .. }) {
        matched["number"].as_u64().unwrap_or(request.issue)
    } else {
        request.issue
    };
    Ok((
        issue,
        pull_request,
        matched["id"].as_u64().or(pull_request).or(Some(issue)),
    ))
}

pub(super) fn json_string_array_contains_all(
    value: &serde_json::Value,
    expected: &[String],
) -> bool {
    expected.iter().all(|expected| {
        github_readback_candidates(value)
            .into_iter()
            .any(|candidate| {
                candidate.as_str() == Some(expected.as_str())
                    || candidate["name"].as_str() == Some(expected.as_str())
                    || candidate["login"].as_str() == Some(expected.as_str())
            })
    })
}

pub(super) fn github_readback_candidates(value: &serde_json::Value) -> Vec<&serde_json::Value> {
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
