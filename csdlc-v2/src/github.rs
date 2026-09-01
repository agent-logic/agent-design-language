use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::params::repos::Commitish;
use octocrab::params::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

use adl_resilience::{execute_retry_policy_async_with_classifier, RetryPolicyError, RetryPolicyV1};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrStateRequest {
    pub repository: String,
    pub pull_request: u64,
    pub required_checks: Vec<String>,
    pub require_review: bool,
    pub token_file: Option<String>,
    pub linked_issue: Option<u64>,
    #[serde(default)]
    pub linked_issue_repository: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrCheck {
    pub name: String,
    pub required: bool,
    pub conclusion: String,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrStatePacket {
    pub schema: String,
    pub repository: String,
    pub pull_request: u64,
    pub linked_issue: Option<u64>,
    #[serde(default)]
    pub linkage_source: Option<String>,
    #[serde(default = "unknown_pr_state")]
    pub state: String,
    pub draft: bool,
    pub merge_state: String,
    pub review_decision: String,
    pub base_ref: Option<String>,
    #[serde(default)]
    pub head_ref: Option<String>,
    pub head_sha: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub merged: bool,
    #[serde(default)]
    pub merge_commit_sha: Option<String>,
    pub checks: Vec<PrCheck>,
    pub required_check_names: Vec<String>,
    pub classification: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct ClosingPullRequestIdentity {
    pub repository: String,
    pub pull_request: u64,
    pub state: String,
    pub merged: bool,
    #[serde(default)]
    pub merged_at: Option<String>,
}

fn unknown_pr_state() -> String {
    "unknown".into()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GithubAction {
    IssueCreate,
    IssueUpdate,
    IssueComment,
    IssueClose,
    IssueRead,
    PrState,
    PrCreate,
    PrUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GithubActionRequest {
    pub repository: String,
    pub action: GithubAction,
    pub operation_key: Option<String>,
    pub token_file: Option<String>,
    pub issue: Option<u64>,
    pub pull_request: Option<u64>,
    pub title: Option<String>,
    pub body: Option<String>,
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default)]
    pub head: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub state: Option<String>,
    pub comment_body: Option<String>,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub linked_issue: Option<u64>,
}

impl TryFrom<&GithubActionRequest> for PrStateRequest {
    type Error = crate::V2Error;

    fn try_from(request: &GithubActionRequest) -> crate::Result<Self> {
        Ok(Self {
            repository: request.repository.clone(),
            pull_request: request.pull_request.ok_or_else(|| {
                crate::V2Error::new(crate::ErrorCode::InvalidInput, "pull_request is required")
            })?,
            required_checks: request.required_checks.clone(),
            require_review: request.require_review,
            token_file: request.token_file.clone(),
            linked_issue: request.linked_issue,
            linked_issue_repository: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GithubIssuePacket {
    pub schema: String,
    pub repository: String,
    pub number: u64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub created_at: Option<String>,
    pub closed_at: Option<String>,
    pub labels: Vec<String>,
    pub assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub marker_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GithubActionResult {
    pub schema: String,
    pub repository: String,
    pub action: GithubAction,
    pub operation_key: Option<String>,
    pub issue: Option<GithubIssuePacket>,
    pub comment_id: Option<u64>,
    pub pr_state: Option<PrStatePacket>,
    pub reconciled: bool,
    #[serde(skip)]
    #[schemars(skip)]
    pub(crate) producer_digest: Option<String>,
}

impl GithubActionResult {
    pub fn is_producer_verified(&self) -> bool {
        self.producer_digest
            .as_deref()
            .zip(self.content_digest().ok().as_deref())
            .is_some_and(|(sealed, current)| sealed == current)
    }

    fn content_digest(&self) -> crate::Result<String> {
        let bytes = serde_json::to_vec(&(
            &self.schema,
            &self.repository,
            &self.action,
            &self.operation_key,
            &self.issue,
            &self.comment_id,
            &self.pr_state,
            self.reconciled,
        ))?;
        Ok(blake3::hash(&bytes).to_hex().to_string())
    }

    fn seal_producer(mut self) -> crate::Result<Self> {
        self.producer_digest = Some(self.content_digest()?);
        Ok(self)
    }
}

pub async fn execute_github_action(
    request: &GithubActionRequest,
) -> crate::Result<GithubActionResult> {
    validate_request(request)?;
    if matches!(request.action, GithubAction::PrState) {
        let pr_request = PrStateRequest::try_from(request)?;
        let pr_state = collect_pr_state(&pr_request).await?;
        return GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: request.repository.clone(),
            action: request.action.clone(),
            operation_key: request.operation_key.clone(),
            issue: None,
            comment_id: None,
            pr_state: Some(pr_state),
            reconciled: true,
            producer_digest: None,
        }
        .seal_producer();
    }
    if matches!(request.action, GithubAction::PrUpdate) {
        let pr_state = update_pull_request(request).await?;
        return GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: request.repository.clone(),
            action: request.action.clone(),
            operation_key: request.operation_key.clone(),
            issue: None,
            comment_id: None,
            pr_state: Some(pr_state),
            reconciled: true,
            producer_digest: None,
        }
        .seal_producer();
    }
    if matches!(request.action, GithubAction::PrCreate) {
        let pr_state = create_or_reconcile_pull_request(request).await?;
        return GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: request.repository.clone(),
            action: request.action.clone(),
            operation_key: request.operation_key.clone(),
            issue: None,
            comment_id: None,
            pr_state: Some(pr_state),
            reconciled: true,
            producer_digest: None,
        }
        .seal_producer();
    }

    let (owner, repo) = split_repository(&request.repository)?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = github_client(token)?;
    let issue = match request.action {
        GithubAction::IssueCreate => reconcile_issue_create(&crab, owner, repo, request).await?,
        GithubAction::IssueUpdate => {
            let number = required_issue(request)?;
            if is_title_only_issue_update(request) {
                let (packet, comment_id) =
                    reconcile_bodyless_issue_update(&crab, owner, repo, number, request).await?;
                return GithubActionResult {
                    schema: "csdlc.github_action_result.v1".into(),
                    repository: request.repository.clone(),
                    action: request.action.clone(),
                    operation_key: request.operation_key.clone(),
                    issue: Some(packet),
                    comment_id: Some(comment_id),
                    pr_state: None,
                    reconciled: true,
                    producer_digest: None,
                }
                .seal_producer();
            }
            update_issue(&crab, owner, repo, number, request).await?;
            let packet =
                read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                    .await?;
            verify_issue_update_readback(&packet, request)?;
            packet
        }
        GithubAction::IssueComment => {
            let number = required_issue(request)?;
            let body = request.comment_body.as_deref().ok_or_else(|| {
                crate::V2Error::new(crate::ErrorCode::InvalidInput, "comment_body is required")
            })?;
            let marked = append_marker(body, required_marker(request)?);
            let before =
                find_marked_comments(&crab, owner, repo, number, required_marker(request)?).await?;
            if before.len() > 1 {
                return Err(crate::V2Error::new(
                    crate::ErrorCode::ReconciliationRequired,
                    "multiple comments match operation marker",
                ));
            }
            let comment_id = if let Some(id) = before.first().copied() {
                id
            } else {
                let value: Value = crab
                    .post(
                        format!("/repos/{owner}/{repo}/issues/{number}/comments"),
                        Some(&json!({ "body": marked })),
                    )
                    .await
                    .map_err(remote)?;
                value.get("id").and_then(Value::as_u64).ok_or_else(|| {
                    crate::V2Error::new(
                        crate::ErrorCode::ReconciliationRequired,
                        "created comment has no id",
                    )
                })?
            };
            let after =
                find_marked_comments(&crab, owner, repo, number, required_marker(request)?).await?;
            if after != vec![comment_id] {
                return Err(crate::V2Error::new(
                    crate::ErrorCode::ReconciliationRequired,
                    "comment marker readback is ambiguous",
                ));
            }
            return GithubActionResult {
                schema: "csdlc.github_action_result.v1".into(),
                repository: request.repository.clone(),
                action: request.action.clone(),
                operation_key: request.operation_key.clone(),
                issue: Some(
                    read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                        .await?,
                ),
                comment_id: Some(comment_id),
                pr_state: None,
                reconciled: true,
                producer_digest: None,
            }
            .seal_producer();
        }
        GithubAction::IssueClose => {
            let number = required_issue(request)?;
            patch_issue(
                &crab,
                owner,
                repo,
                number,
                json!({"state": "closed", "state_reason": "completed"}),
            )
            .await?;
            let packet =
                read_issue_packet(&crab, owner, repo, number, request.operation_key.as_deref())
                    .await?;
            verify_issue_closed(&packet)?;
            packet
        }
        GithubAction::IssueRead => {
            let number = required_issue(request)?;
            let value = fetch_issue_value(&crab, owner, repo, number)
                .await
                .map_err(|error| classify_issue_read_error(error, &request.repository, number))?;
            normalize_issue(
                &request.repository,
                &value,
                request.operation_key.as_deref(),
            )?
        }
        GithubAction::PrState | GithubAction::PrCreate | GithubAction::PrUpdate => {
            unreachable!("handled above")
        }
    };
    GithubActionResult {
        schema: "csdlc.github_action_result.v1".into(),
        repository: request.repository.clone(),
        action: request.action.clone(),
        operation_key: request.operation_key.clone(),
        issue: Some(issue),
        comment_id: None,
        pr_state: None,
        reconciled: true,
        producer_digest: None,
    }
    .seal_producer()
}

fn is_title_only_issue_update(request: &GithubActionRequest) -> bool {
    request.title.is_some()
        && request.body.is_none()
        && request.state.is_none()
        && request.labels.is_empty()
        && request.assignees.is_empty()
        && request.milestone.is_none()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct IssueUpdateReceipt {
    schema: String,
    operation_key: String,
    request_fingerprint: String,
    pre_body_digest: String,
    post_body_digest: String,
    pre_updated_at: Option<String>,
    post_updated_at: Option<String>,
}

#[derive(Debug, Clone)]
struct MarkedComment {
    id: u64,
    body: String,
}

async fn reconcile_bodyless_issue_update(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    request: &GithubActionRequest,
) -> crate::Result<(GithubIssuePacket, u64)> {
    let operation_key = required_marker(request)?;
    let fingerprint = issue_update_fingerprint(request)?;
    let existing = find_marked_comment_values(crab, owner, repo, number, operation_key).await?;
    if existing.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "multiple provenance receipts match operation key",
        ));
    }
    if let Some(comment) = existing.first() {
        let receipt = parse_issue_update_receipt(&comment.body, operation_key)?;
        if receipt.request_fingerprint != fingerprint {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "operation key is already bound to a different issue update fingerprint",
            ));
        }
        let value = fetch_issue_value(crab, owner, repo, number)
            .await
            .map_err(remote)?;
        let packet = normalize_issue(&request.repository, &value, None)?;
        verify_issue_update_readback(&packet, request)?;
        let body_digest = digest_text(&packet.body);
        if receipt.pre_body_digest != receipt.post_body_digest
            || receipt.post_body_digest != body_digest
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "issue body no longer agrees with the recorded update provenance",
            ));
        }
        return Ok((packet, comment.id));
    }

    let before_value = fetch_issue_value(crab, owner, repo, number)
        .await
        .map_err(remote)?;
    let before = normalize_issue(&request.repository, &before_value, None)?;
    let pre_updated_at = issue_updated_at(&before_value);
    let confirmed_value = fetch_issue_value(crab, owner, repo, number)
        .await
        .map_err(remote)?;
    let confirmed = normalize_issue(&request.repository, &confirmed_value, None)?;
    if confirmed.body != before.body || issue_updated_at(&confirmed_value) != pre_updated_at {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue drifted before body-preserving update",
        ));
    }
    let mutation_already_observed = request
        .title
        .as_ref()
        .is_some_and(|title| &confirmed.title == title);
    if mutation_already_observed {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "requested issue title is already present without a matching durable provenance receipt",
        ));
    }
    update_issue(crab, owner, repo, number, request).await?;

    let after_value = fetch_issue_value(crab, owner, repo, number)
        .await
        .map_err(remote)?;
    let after = normalize_issue(&request.repository, &after_value, None)?;
    verify_issue_update_readback(&after, request)?;
    if after.body != before.body {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue body drifted during body-preserving update",
        ));
    }
    let receipt = IssueUpdateReceipt {
        schema: "csdlc.github_issue_update_receipt.v1".into(),
        operation_key: operation_key.into(),
        request_fingerprint: fingerprint,
        pre_body_digest: digest_text(&before.body),
        post_body_digest: digest_text(&after.body),
        pre_updated_at: issue_updated_at(&confirmed_value),
        post_updated_at: issue_updated_at(&after_value),
    };
    let receipt_body = format!(
        "{}\n```json\n{}\n```\n",
        marker_line(operation_key),
        serde_json::to_string(&receipt)?
    );
    let created: Value = crab
        .post(
            format!("/repos/{owner}/{repo}/issues/{number}/comments"),
            Some(&json!({ "body": receipt_body })),
        )
        .await
        .map_err(remote)?;
    let comment_id = created.get("id").and_then(Value::as_u64).ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created provenance receipt has no id",
        )
    })?;

    let comments = find_marked_comment_values(crab, owner, repo, number, operation_key).await?;
    if comments.len() != 1 || comments[0].id != comment_id {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "provenance receipt readback is ambiguous",
        ));
    }
    let observed_receipt = parse_issue_update_receipt(&comments[0].body, operation_key)?;
    if observed_receipt != receipt {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "provenance receipt readback differs from governed receipt",
        ));
    }
    let final_value = fetch_issue_value(crab, owner, repo, number)
        .await
        .map_err(remote)?;
    let final_packet = normalize_issue(&request.repository, &final_value, None)?;
    verify_issue_update_readback(&final_packet, request)?;
    if final_packet.body != before.body {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue body drifted before final provenance reconciliation",
        ));
    }
    Ok((final_packet, comment_id))
}

fn issue_update_fingerprint(request: &GithubActionRequest) -> crate::Result<String> {
    let bytes = serde_json::to_vec(&(
        &request.repository,
        request.issue,
        &request.operation_key,
        &request.title,
        &request.body,
        &request.labels,
        &request.assignees,
        request.milestone,
        &request.state,
    ))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn digest_text(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}

fn issue_updated_at(value: &Value) -> Option<String> {
    value
        .get("updated_at")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn parse_issue_update_receipt(
    body: &str,
    operation_key: &str,
) -> crate::Result<IssueUpdateReceipt> {
    if !body.contains(&marker_line(operation_key)) {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "provenance receipt marker is missing",
        ));
    }
    let json = body
        .split_once("```json\n")
        .and_then(|(_, tail)| tail.split_once("\n```").map(|(json, _)| json))
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "provenance receipt payload is malformed",
            )
        })?;
    let receipt: IssueUpdateReceipt = serde_json::from_str(json).map_err(|_| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "provenance receipt payload is invalid",
        )
    })?;
    if receipt.schema != "csdlc.github_issue_update_receipt.v1"
        || receipt.operation_key != operation_key
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "provenance receipt identity is invalid",
        ));
    }
    Ok(receipt)
}

fn validate_request(request: &GithubActionRequest) -> crate::Result<()> {
    split_repository(&request.repository)?;
    if matches!(
        request.action,
        GithubAction::IssueCreate
            | GithubAction::IssueUpdate
            | GithubAction::IssueComment
            | GithubAction::IssueClose
            | GithubAction::PrCreate
            | GithubAction::PrUpdate
    ) {
        required_marker(request)?;
    }
    if let Some(key) = &request.operation_key {
        if key.trim() != key
            || key.len() < 8
            || key.len() > 128
            || !key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':'))
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "operation_key must be 8..128 chars of ascii alnum, dash, underscore, dot, or colon",
            ));
        }
    }
    if matches!(request.action, GithubAction::IssueCreate)
        && (request.title.as_deref().is_none_or(|v| v.trim().is_empty())
            || request.body.as_deref().is_none_or(|v| v.trim().is_empty()))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "title and body are required for issue_create",
        ));
    }
    if matches!(request.action, GithubAction::PrUpdate) {
        if request.pull_request.is_none() {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "pull_request is required for pr_update",
            ));
        }
        if request.body.as_deref().is_none_or(|v| v.trim().is_empty()) {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "body is required for pr_update",
            ));
        }
        if request.issue.is_some()
            || request.title.is_some()
            || request.state.is_some()
            || request.comment_body.is_some()
            || request.base.is_some()
            || request.head.is_some()
            || !request.labels.is_empty()
            || !request.assignees.is_empty()
            || request.milestone.is_some()
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "pr_update accepts only pull_request, body, required_checks, require_review, linked_issue, token_file, and operation_key",
            ));
        }
    }
    if matches!(request.action, GithubAction::PrCreate) {
        if request.title.as_deref().is_none_or(|v| v.trim().is_empty())
            || request.body.as_deref().is_none_or(|v| v.trim().is_empty())
            || request.base.as_deref().is_none_or(|v| v.trim().is_empty())
            || request.head.as_deref().is_none_or(|v| v.trim().is_empty())
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "title, body, base, and head are required for pr_create",
            ));
        }
        if request.issue.is_some()
            || request.pull_request.is_some()
            || request.state.is_some()
            || request.comment_body.is_some()
            || !request.labels.is_empty()
            || !request.assignees.is_empty()
            || request.milestone.is_some()
        {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "pr_create accepts only title, body, base, head, required_checks, require_review, linked_issue, token_file, and operation_key",
            ));
        }
    }
    if let Some(state) = &request.state {
        if !matches!(state.as_str(), "open" | "closed") {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "state must be open or closed",
            ));
        }
    }
    Ok(())
}

async fn create_or_reconcile_pull_request(
    request: &GithubActionRequest,
) -> crate::Result<PrStatePacket> {
    let (owner, repo) = split_repository(&request.repository)?;
    let title = request.title.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "title is required for pr_create",
        )
    })?;
    let body = request.body.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "body is required for pr_create",
        )
    })?;
    let base = request.base.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "base is required for pr_create",
        )
    })?;
    let head = request.head.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "head is required for pr_create",
        )
    })?;
    let marker = required_marker(request)?;
    let marked_body = append_marker(body, marker);
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = github_client(token)?;
    let observed = find_open_pull_request(&crab, owner, repo, head, base).await?;
    let pr_number = if let Some(pr) = observed {
        verify_pull_request_create_readback(&pr, title, &marked_body)?;
        pull_request_number(&pr)?
    } else {
        let created = crab
            .pulls(owner, repo)
            .create(title, head, base)
            .body(&marked_body)
            .send()
            .await
            .map_err(remote)?;
        verify_pull_request_create_readback(&created, title, &marked_body)?;
        pull_request_number(&created)?
    };
    let packet = collect_pr_state(&PrStateRequest {
        repository: request.repository.clone(),
        pull_request: pr_number,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: request.linked_issue,
        linked_issue_repository: None,
    })
    .await?;
    if packet.base_ref.as_deref() == Some(base)
        && packet.head_ref.as_deref() == Some(head)
        && packet.body.as_deref() == Some(marked_body.as_str())
    {
        Ok(packet)
    } else {
        Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR create readback differs from governed request",
        ))
    }
}

async fn find_open_pull_request(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    head: &str,
    base: &str,
) -> crate::Result<Option<octocrab::models::pulls::PullRequest>> {
    let page = crab
        .pulls(owner, repo)
        .list()
        .state(State::Open)
        .head(format!("{owner}:{head}"))
        .base(base)
        .per_page(100)
        .send()
        .await
        .map_err(remote)?;
    let items = crab.all_pages(page).await.map_err(remote)?;
    select_unique_pull_request(items)
}

fn select_unique_pull_request(
    mut items: Vec<octocrab::models::pulls::PullRequest>,
) -> crate::Result<Option<octocrab::models::pulls::PullRequest>> {
    if items.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "multiple matching PRs observed",
        ));
    }
    Ok(items.pop())
}

fn verify_pull_request_create_readback(
    pr: &octocrab::models::pulls::PullRequest,
    title: &str,
    body: &str,
) -> crate::Result<()> {
    if pr.title.as_deref() != Some(title) || pr.body.as_deref() != Some(body) {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR create observation differs from governed request",
        ));
    }
    Ok(())
}

fn pull_request_number(pr: &octocrab::models::pulls::PullRequest) -> crate::Result<u64> {
    pr.number.ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR observation is missing number",
        )
    })
}

fn plan_pull_request_body_update(
    observed_body: Option<&str>,
    requested_body: &str,
    operation_key: &str,
) -> crate::Result<Option<String>> {
    let marker = marker_line(operation_key);
    let governed_body = append_marker(requested_body, operation_key);
    match observed_body {
        Some(body) if body == governed_body => Ok(None),
        Some(body) if body.contains(&marker) => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR update operation key already applied to a different body",
        )),
        _ => Ok(Some(governed_body)),
    }
}

async fn update_pull_request(request: &GithubActionRequest) -> crate::Result<PrStatePacket> {
    let (owner, repo) = split_repository(&request.repository)?;
    let number = request.pull_request.ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "pull_request is required for pr_update",
        )
    })?;
    let body = request.body.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "body is required for pr_update",
        )
    })?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = github_client(token)?;
    let state_request = PrStateRequest {
        repository: request.repository.clone(),
        pull_request: number,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: request.linked_issue,
        linked_issue_repository: None,
    };
    let before = collect_pr_state(&state_request).await?;
    let governed_body = append_marker(body, required_marker(request)?);
    if let Some(next_body) =
        plan_pull_request_body_update(before.body.as_deref(), body, required_marker(request)?)?
    {
        let _: Value = crab
            .patch(
                format!("/repos/{owner}/{repo}/pulls/{number}"),
                Some(&json!({ "body": next_body })),
            )
            .await
            .map_err(remote)?;
    }
    let packet = collect_pr_state(&state_request).await?;
    if packet.body.as_deref() != Some(governed_body.as_str()) {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR update readback differs from governed request",
        ));
    }
    Ok(packet)
}

async fn reconcile_issue_create(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    request: &GithubActionRequest,
) -> crate::Result<GithubIssuePacket> {
    let marker = required_marker(request)?;
    let matches = find_marked_issues(crab, owner, repo, marker).await?;
    if matches.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "multiple issues match operation marker",
        ));
    }
    if let Some(number) = matches.first().copied() {
        let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
        verify_issue_identity(&packet, request)?;
        return Ok(packet);
    }
    let body = append_marker(request.body.as_deref().unwrap_or_default(), marker);
    let mut payload = json!({
        "title": request.title.as_deref().unwrap_or_default(),
        "body": body,
    });
    if !request.labels.is_empty() {
        payload["labels"] = json!(request.labels);
    }
    if !request.assignees.is_empty() {
        payload["assignees"] = json!(request.assignees);
    }
    if let Some(milestone) = request.milestone {
        payload["milestone"] = json!(milestone);
    }
    let created: Value = crab
        .post(format!("/repos/{owner}/{repo}/issues"), Some(&payload))
        .await
        .map_err(remote)?;
    let number = created
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "created issue has no number",
            )
        })?;
    let packet = read_created_issue_packet(crab, owner, repo, number, marker).await?;
    verify_issue_identity(&packet, request)?;
    Ok(packet)
}

async fn read_created_issue_packet(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<GithubIssuePacket> {
    let policy = RetryPolicyV1::new(4, Some(250));
    let execution = execute_retry_policy_async_with_classifier(
        &policy,
        |_| async {
            let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
            if packet.marker_present {
                Ok(packet)
            } else {
                reconcile_created_issue_by_marker_search(crab, owner, repo, number, marker).await
            }
        },
        is_retryable_created_issue_readback,
        tokio::time::sleep,
    )
    .await
    .map_err(retry_policy_error)?;
    execution.result
}

async fn reconcile_created_issue_by_marker_search(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<GithubIssuePacket> {
    let matches = find_marked_issue_packets(crab, owner, repo, marker).await?;
    match matches.as_slice() {
        [packet] if packet.number == number => Ok(packet.clone()),
        [] => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created issue marker search found no matching issue",
        )),
        [packet] => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            format!(
                "created issue marker search found different issue {} instead of {}",
                packet.number, number
            ),
        )),
        _ => Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "created issue marker search found multiple matching issues",
        )),
    }
}

fn is_retryable_created_issue_readback(error: &crate::V2Error) -> bool {
    error.code == crate::ErrorCode::ReconciliationRequired
        && error
            .message
            .contains("created issue marker search found no matching issue")
}

fn retry_policy_error(error: RetryPolicyError) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::ValidationFailed,
        format!("GitHub readback retry policy failed: {error:?}"),
    )
}

async fn update_issue(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    let mut payload = serde_json::Map::new();
    if let Some(title) = &request.title {
        payload.insert("title".into(), json!(title));
    }
    if let Some(body) = &request.body {
        payload.insert(
            "body".into(),
            json!(if let Some(marker) = request.operation_key.as_deref() {
                append_marker(body, marker)
            } else {
                body.clone()
            }),
        );
    }
    if let Some(state) = &request.state {
        payload.insert("state".into(), json!(state));
    }
    if !request.labels.is_empty() {
        payload.insert("labels".into(), json!(request.labels));
    }
    if !request.assignees.is_empty() {
        payload.insert("assignees".into(), json!(request.assignees));
    }
    if let Some(milestone) = request.milestone {
        payload.insert("milestone".into(), json!(milestone));
    }
    if payload.is_empty() {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "issue_update has no fields to update",
        ));
    }
    patch_issue(crab, owner, repo, number, Value::Object(payload)).await
}

async fn patch_issue(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    payload: Value,
) -> crate::Result<()> {
    let _: Value = crab
        .patch(
            format!("/repos/{owner}/{repo}/issues/{number}"),
            Some(&payload),
        )
        .await
        .map_err(remote)?;
    Ok(())
}

async fn read_issue_packet(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: Option<&str>,
) -> crate::Result<GithubIssuePacket> {
    let value = fetch_issue_value(crab, owner, repo, number)
        .await
        .map_err(remote)?;
    normalize_issue(&format!("{owner}/{repo}"), &value, marker)
}

async fn fetch_issue_value(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
) -> Result<Value, octocrab::Error> {
    crab.get(
        format!("/repos/{owner}/{repo}/issues/{number}"),
        None::<&()>,
    )
    .await
}

fn classify_issue_read_error(
    error: octocrab::Error,
    repository: &str,
    number: u64,
) -> crate::V2Error {
    use crate::ErrorCode;

    let (code, message) = match error {
        octocrab::Error::GitHub { source, .. } => {
            return classify_github_issue_read_status(
                source.status_code.as_u16(),
                &source.message,
                source.documentation_url.as_deref(),
                repository,
                number,
            );
        }
        octocrab::Error::Http { .. }
        | octocrab::Error::Hyper { .. }
        | octocrab::Error::Service { .. }
        | octocrab::Error::Uri { .. }
        | octocrab::Error::UriParse { .. } => (
            ErrorCode::RemoteTransport,
            format!("Transport failure prevented reading {repository}#{number}"),
        ),
        _ => (
            ErrorCode::RemoteFailure,
            format!("GitHub observation failed while reading {repository}#{number}"),
        ),
    };
    crate::V2Error::new(code, message)
}

fn classify_github_issue_read_status(
    status: u16,
    remote_message: &str,
    documentation_url: Option<&str>,
    repository: &str,
    number: u64,
) -> crate::V2Error {
    use crate::ErrorCode;
    let (code, message) = match status {
        404 => (
            ErrorCode::RemoteNotFound,
            format!("GitHub issue {repository}#{number} was not found or is inaccessible; verify the repository, issue number, and token access"),
        ),
        401 => (
            ErrorCode::RemoteAuthentication,
            format!("Authentication failed while reading {repository}#{number}"),
        ),
        403 if github_rate_limit_signal(remote_message, documentation_url) => (
            ErrorCode::RemoteRateLimited,
            format!("GitHub rate limit prevented reading {repository}#{number}"),
        ),
        403 => (
            ErrorCode::RemoteAuthorization,
            format!("Authorization failed while reading {repository}#{number}"),
        ),
        429 => (
            ErrorCode::RemoteRateLimited,
            format!("GitHub rate limit prevented reading {repository}#{number}"),
        ),
        500..=599 => (
            ErrorCode::RemoteServer,
            format!("GitHub server failure prevented reading {repository}#{number}"),
        ),
        _ => (
            ErrorCode::RemoteFailure,
            format!("GitHub observation failed while reading {repository}#{number}"),
        ),
    };
    crate::V2Error::new(code, message)
}

fn github_rate_limit_signal(message: &str, documentation_url: Option<&str>) -> bool {
    let message = message.trim().to_ascii_lowercase();
    if message.starts_with("api rate limit exceeded")
        || message
            == "you have exceeded a secondary rate limit. please wait a few minutes before you try again."
    {
        return true;
    }
    let Some(documentation_url) = documentation_url else {
        return false;
    };
    let Ok(url) = url::Url::parse(documentation_url) else {
        return false;
    };
    if url.scheme() != "https" || url.host_str() != Some("docs.github.com") {
        return false;
    }
    url.path() == "/rest/using-the-rest-api/rate-limits-for-the-rest-api"
        || (url.path() == "/rest/overview/resources-in-the-rest-api"
            && matches!(
                url.fragment(),
                Some("rate-limiting" | "secondary-rate-limits")
            ))
}

fn normalize_issue(
    repository: &str,
    value: &Value,
    marker: Option<&str>,
) -> crate::Result<GithubIssuePacket> {
    let body = value
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    Ok(GithubIssuePacket {
        schema: "csdlc.github_issue.v1".into(),
        repository: repository.into(),
        number: value.get("number").and_then(Value::as_u64).ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "issue number missing",
            )
        })?,
        title: value
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        body: body.clone(),
        state: value
            .get("state")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_owned(),
        created_at: value
            .get("created_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        closed_at: value
            .get("closed_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
        labels: value
            .get("labels")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|label| label.get("name").and_then(Value::as_str).map(str::to_owned))
            .collect(),
        assignees: value
            .get("assignees")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|user| user.get("login").and_then(Value::as_str).map(str::to_owned))
            .collect(),
        milestone: value
            .get("milestone")
            .and_then(|m| m.get("number"))
            .and_then(Value::as_u64),
        marker_present: marker.is_some_and(|m| body.contains(&marker_line(m))),
    })
}

fn verify_issue_identity(
    packet: &GithubIssuePacket,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    let wanted_labels = request.labels.iter().cloned().collect::<BTreeSet<_>>();
    let got_labels = packet.labels.iter().cloned().collect::<BTreeSet<_>>();
    let wanted_assignees = request.assignees.iter().cloned().collect::<BTreeSet<_>>();
    let got_assignees = packet.assignees.iter().cloned().collect::<BTreeSet<_>>();
    if packet.title != request.title.clone().unwrap_or_default()
        || !packet.marker_present
        || !wanted_labels.is_subset(&got_labels)
        || !wanted_assignees.is_subset(&got_assignees)
        || request
            .milestone
            .is_some_and(|m| packet.milestone != Some(m))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue readback differs from governed request identity",
        ));
    }
    Ok(())
}

fn verify_issue_update_readback(
    packet: &GithubIssuePacket,
    request: &GithubActionRequest,
) -> crate::Result<()> {
    if request
        .title
        .as_ref()
        .is_some_and(|title| &packet.title != title)
        || request.body.as_ref().is_some_and(|body| {
            packet.body != append_marker(body, request.operation_key.as_deref().unwrap_or_default())
        })
        || request
            .state
            .as_ref()
            .is_some_and(|state| &packet.state != state)
        || (!request.labels.is_empty() && !requested_values_match(&request.labels, &packet.labels))
        || (!request.assignees.is_empty()
            && !requested_values_match(&request.assignees, &packet.assignees))
        || request
            .milestone
            .is_some_and(|milestone| packet.milestone != Some(milestone))
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue update readback differs from governed request",
        ));
    }
    Ok(())
}

fn verify_issue_closed(packet: &GithubIssuePacket) -> crate::Result<()> {
    if packet.state != "closed" {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "issue close readback did not observe closed state",
        ));
    }
    Ok(())
}

fn requested_values_match(requested: &[String], observed: &[String]) -> bool {
    let requested = requested.iter().cloned().collect::<BTreeSet<_>>();
    let observed = observed.iter().cloned().collect::<BTreeSet<_>>();
    requested == observed
}

async fn find_marked_issues(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    marker: &str,
) -> crate::Result<Vec<u64>> {
    Ok(find_marked_issue_packets(crab, owner, repo, marker)
        .await?
        .into_iter()
        .map(|packet| packet.number)
        .collect())
}

async fn find_marked_issue_packets(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    marker: &str,
) -> crate::Result<Vec<GithubIssuePacket>> {
    let query = format!(
        "repo:{owner}/{repo} type:issue in:body {}",
        marker_line(marker)
    );
    let value: Value = crab
        .get(
            "/search/issues",
            Some(&[("q", query.as_str()), ("per_page", "10")]),
        )
        .await
        .map_err(remote)?;
    let candidates = value
        .get("items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("number").and_then(Value::as_u64))
        .collect::<BTreeSet<_>>();
    let mut exact_matches = Vec::new();
    for number in candidates {
        let packet = read_issue_packet(crab, owner, repo, number, Some(marker)).await?;
        if packet.marker_present {
            exact_matches.push(packet);
        }
    }
    Ok(exact_matches)
}

async fn find_marked_comments(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<Vec<u64>> {
    Ok(
        find_marked_comment_values(crab, owner, repo, number, marker)
            .await?
            .into_iter()
            .map(|comment| comment.id)
            .collect(),
    )
}

async fn find_marked_comment_values(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    number: u64,
    marker: &str,
) -> crate::Result<Vec<MarkedComment>> {
    let mut matches = Vec::new();
    let mut page = 1_u32;
    loop {
        let value: Vec<Value> = crab
            .get(
                format!("/repos/{owner}/{repo}/issues/{number}/comments"),
                Some(&[("per_page", "100".to_owned()), ("page", page.to_string())]),
            )
            .await
            .map_err(remote)?;
        let value_len = value.len();
        matches.extend(value.into_iter().filter_map(|comment| {
            let id = comment.get("id").and_then(Value::as_u64)?;
            let body = comment.get("body").and_then(Value::as_str)?.to_owned();
            body.contains(&marker_line(marker))
                .then_some(MarkedComment { id, body })
        }));
        if value_len < 100 {
            break;
        }
        page = page.checked_add(1).ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "issue comment pagination exceeded supported range",
            )
        })?;
    }
    Ok(matches)
}

fn split_repository(repository: &str) -> crate::Result<(&str, &str)> {
    repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })
}

fn required_issue(request: &GithubActionRequest) -> crate::Result<u64> {
    request
        .issue
        .ok_or_else(|| crate::V2Error::new(crate::ErrorCode::InvalidInput, "issue is required"))
}

fn required_marker(request: &GithubActionRequest) -> crate::Result<&str> {
    request.operation_key.as_deref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "operation_key is required for idempotent mutation",
        )
    })
}

pub fn marker_line(operation_key: &str) -> String {
    format!("<!-- csdlc-github-operation:{operation_key} -->")
}

pub fn append_marker(body: &str, operation_key: &str) -> String {
    let marker = marker_line(operation_key);
    if body.contains(&marker) {
        body.to_owned()
    } else if body.ends_with('\n') {
        format!("{body}{marker}\n")
    } else {
        format!("{body}\n\n{marker}\n")
    }
}

fn github_client(token: String) -> crate::Result<octocrab::Octocrab> {
    let mut builder = octocrab::Octocrab::builder().personal_token(token);
    #[cfg(debug_assertions)]
    if let Some(base) = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE") {
        let base = base.to_string_lossy();
        let parsed = url::Url::parse(&base).map_err(|_| {
            crate::V2Error::new(crate::ErrorCode::InvalidInput, "test API base is invalid")
        })?;
        let loopback = match parsed.host() {
            Some(url::Host::Domain("localhost")) => true,
            Some(url::Host::Ipv4(address)) => address.is_loopback(),
            Some(url::Host::Ipv6(address)) => address.is_loopback(),
            _ => false,
        };
        if parsed.scheme() != "http" || !loopback || parsed.path() != "/" {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "test API base must be an HTTP loopback origin",
            ));
        }
        builder = builder.base_uri(base.as_ref()).map_err(remote)?;
    }
    builder.build().map_err(remote)
}

pub fn classify_pr_state(packet: &PrStatePacket, require_review: bool) -> &'static str {
    if packet.draft {
        return "waiting";
    }
    if packet.merge_state == "behind" {
        return "stale_base";
    }
    if packet.merge_state == "dirty" {
        return "conflicted";
    }
    if matches!(packet.merge_state.as_str(), "blocked" | "draft" | "unknown") {
        return "waiting";
    }
    if packet.merge_state == "unstable" && packet.required_check_names.is_empty() {
        return "waiting";
    }
    for name in &packet.required_check_names {
        let Some(check) = packet.checks.iter().find(|check| &check.name == name) else {
            return "waiting";
        };
        match check.conclusion.as_str() {
            "success" => {}
            "failure" | "cancelled" => return "failed",
            _ => return "waiting",
        }
    }
    if require_review && packet.review_decision != "approved" {
        return "operator_review";
    }
    "ready"
}

fn remotely_linked_issue(
    response: &Value,
    repository: &str,
    expected: Option<u64>,
) -> crate::Result<Option<u64>> {
    let nodes = response
        // Octocrab's `graphql::<Value>` returns the decoded `data` payload,
        // while direct/raw fixtures may retain the outer `data` envelope.
        .pointer("/repository/pullRequest/closingIssuesReferences/nodes")
        .or_else(|| response.pointer("/data/repository/pullRequest/closingIssuesReferences/nodes"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub closing-issue relation is absent",
            )
        })?;
    let mut issues = nodes
        .iter()
        .filter(|node| {
            node.pointer("/repository/nameWithOwner")
                .and_then(Value::as_str)
                == Some(repository)
        })
        .filter_map(|node| node.get("number").and_then(Value::as_u64))
        .collect::<BTreeSet<_>>();
    if let Some(expected) = expected {
        if !issues.remove(&expected) {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "caller-linked issue is not a remote GitHub closing relation",
            ));
        }
        return Ok(Some(expected));
    }
    if issues.len() > 1 {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR closes multiple issues; linked issue must be selected explicitly",
        ));
    }
    Ok(issues.into_iter().next())
}

fn closing_pull_request_identities(
    response: &Value,
) -> crate::Result<Vec<ClosingPullRequestIdentity>> {
    let connection = response
        .pointer("/repository/issue/closedByPullRequestsReferences")
        .or_else(|| response.pointer("/data/repository/issue/closedByPullRequestsReferences"))
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub issue closing-PR references are absent",
            )
        })?;
    if connection
        .pointer("/pageInfo/hasNextPage")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "GitHub issue closing-PR inventory exceeds the bounded page",
        ));
    }
    let nodes = connection
        .get("nodes")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub issue closing-PR references are absent",
            )
        })?;
    let identities = nodes
        .iter()
        .map(|node| {
            let repository = node
                .pointer("/repository/nameWithOwner")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| {
                    crate::V2Error::new(
                        crate::ErrorCode::ReconciliationRequired,
                        "GitHub closing PR repository identity is absent",
                    )
                })?;
            let pull_request = node
                .get("number")
                .and_then(Value::as_u64)
                .filter(|n| *n > 0)
                .ok_or_else(|| {
                    crate::V2Error::new(
                        crate::ErrorCode::ReconciliationRequired,
                        "GitHub closing PR number is absent",
                    )
                })?;
            Ok(ClosingPullRequestIdentity {
                repository: repository.to_owned(),
                pull_request,
                state: node
                    .get("state")
                    .and_then(Value::as_str)
                    .unwrap_or("UNKNOWN")
                    .to_owned(),
                merged: node.get("merged").and_then(Value::as_bool).unwrap_or(false),
                merged_at: node
                    .get("mergedAt")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect::<crate::Result<BTreeSet<_>>>()?;
    Ok(identities.into_iter().collect())
}

pub async fn collect_issue_closing_pull_requests(
    repository: &str,
    issue: u64,
    token_file: Option<String>,
) -> crate::Result<Vec<ClosingPullRequestIdentity>> {
    let (owner, repo) = repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    let token = resolve_token(token_file.as_deref())?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote)?;
    let response: Value = crab
        .graphql(&json!({
            "query": "query ClosingPullRequests($owner: String!, $repo: String!, $number: Int!) { repository(owner: $owner, name: $repo) { issue(number: $number) { closedByPullRequestsReferences(first: 100, includeClosedPrs: true) { nodes { number state merged mergedAt repository { nameWithOwner } } pageInfo { hasNextPage } } } } }",
            "variables": {"owner": owner, "repo": repo, "number": issue}
        }))
        .await
        .map_err(remote)?;
    closing_pull_request_identities(&response)
}

pub async fn collect_pr_state(request: &PrStateRequest) -> crate::Result<PrStatePacket> {
    let (owner, repo) = request.repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    let token = resolve_token(request.token_file.as_deref())?;
    let crab = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote)?;
    let pr = crab
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(remote)?;
    let linkage: Value = crab
        .graphql(&json!({
            "query": "query ClosingIssues($owner: String!, $repo: String!, $number: Int!) { repository(owner: $owner, name: $repo) { pullRequest(number: $number) { closingIssuesReferences(first: 100) { nodes { number repository { nameWithOwner } } } } } }",
            "variables": {"owner": owner, "repo": repo, "number": request.pull_request}
        }))
        .await
        .map_err(remote)?;
    let linked_issue = remotely_linked_issue(
        &linkage,
        request
            .linked_issue_repository
            .as_deref()
            .unwrap_or(&request.repository),
        request.linked_issue,
    )?;
    let head = pr.head.as_ref().ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::ReconciliationRequired,
            "PR head is absent",
        )
    })?;
    let mut page_number = 1_u32;
    let first_page = crab
        .checks(owner, repo)
        .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
        .per_page(100)
        .page(page_number)
        .send()
        .await
        .map_err(remote)?;
    let total = first_page.total_count as usize;
    let mut check_runs = first_page.check_runs;
    while check_runs.len() < total {
        page_number += 1;
        let next_page = crab
            .checks(owner, repo)
            .list_check_runs_for_git_ref(Commitish(head.sha.clone()))
            .per_page(100)
            .page(page_number)
            .send()
            .await
            .map_err(remote)?;
        if next_page.check_runs.is_empty() {
            return Err(crate::V2Error::new(
                crate::ErrorCode::ReconciliationRequired,
                "GitHub check-run pagination ended before total_count",
            ));
        }
        check_runs.extend(next_page.check_runs);
    }
    let mut latest = BTreeMap::new();
    for run in check_runs {
        let replace =
            latest
                .get(&run.name)
                .is_none_or(|prior: &octocrab::models::checks::CheckRun| {
                    run_is_newer(
                        run.started_at.map(|time| time.timestamp_millis()),
                        run.id.0,
                        prior.started_at.map(|time| time.timestamp_millis()),
                        prior.id.0,
                    )
                });
        if replace {
            latest.insert(run.name.clone(), run);
        }
    }
    let checks = latest
        .into_values()
        .map(|run| PrCheck {
            required: request.required_checks.contains(&run.name),
            name: run.name,
            conclusion: conclusion(run.conclusion.as_deref()).into(),
            details_url: run.details_url,
        })
        .collect::<Vec<_>>();
    let reviews = crab
        .all_pages(
            crab.pulls(owner, repo)
                .list_reviews(request.pull_request)
                .per_page(100)
                .send()
                .await
                .map_err(remote)?,
        )
        .await
        .map_err(remote)?;
    let review_decision = exact_head_review_decision(
        reviews.iter().map(|review| {
            (
                review
                    .user
                    .as_ref()
                    .map(|user| user.login.as_str())
                    .unwrap_or_default(),
                review.commit_id.as_deref(),
                review.state,
                (
                    review
                        .submitted_at
                        .map(|submitted| submitted.timestamp_millis())
                        .unwrap_or_default(),
                    review.id.0,
                ),
            )
        }),
        &head.sha,
    );
    let merge_state = normalize_mergeable_state(pr.mergeable_state);
    let mut packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        linked_issue,
        linkage_source: linked_issue.map(|_| "github_closing_issues_references".into()),
        state: match pr.state {
            Some(octocrab::models::IssueState::Open) => "open",
            Some(octocrab::models::IssueState::Closed) => "closed",
            _ => "unknown",
        }
        .into(),
        draft: pr.draft.unwrap_or(false),
        merge_state: merge_state.into(),
        review_decision: review_decision.into(),
        base_ref: pr.base.as_ref().map(|b| b.ref_field.clone()),
        head_ref: Some(head.ref_field.clone()),
        head_sha: head.sha.clone(),
        url: pr.html_url.map(|url| url.to_string()),
        body: pr.body.clone(),
        merged: pr.merged_at.is_some(),
        merge_commit_sha: pr.merge_commit_sha.clone(),
        checks,
        required_check_names: request.required_checks.clone(),
        classification: String::new(),
    };
    packet.classification = classify_pr_state(&packet, request.require_review).into();
    Ok(packet)
}

fn exact_head_review_decision<'a>(
    reviews: impl IntoIterator<Item = (&'a str, Option<&'a str>, Option<ReviewState>, (i64, u64))>,
    head_sha: &str,
) -> &'static str {
    let mut latest = BTreeMap::<String, ((i64, u64), Option<ReviewState>)>::new();
    for (reviewer, commit_id, state, order) in reviews {
        if commit_id != Some(head_sha)
            || !matches!(
                state,
                Some(
                    ReviewState::Approved | ReviewState::ChangesRequested | ReviewState::Dismissed
                )
            )
        {
            continue;
        }
        let key = if reviewer.is_empty() {
            format!("anonymous-review-{}", order.1)
        } else {
            reviewer.into()
        };
        if latest
            .get(&key)
            .is_none_or(|(current_order, _)| order > *current_order)
        {
            latest.insert(key, (order, state));
        }
    }
    let states = latest
        .values()
        .filter_map(|(_, state)| *state)
        .filter(|state| *state != ReviewState::Dismissed)
        .collect::<Vec<_>>();
    if states.contains(&ReviewState::ChangesRequested) {
        "changes_requested"
    } else if states.contains(&ReviewState::Approved) {
        "approved"
    } else {
        "pending"
    }
}

pub(crate) fn normalize_mergeable_state(state: Option<MergeableState>) -> &'static str {
    match state {
        Some(MergeableState::Behind) => "behind",
        Some(MergeableState::Blocked) => "blocked",
        Some(MergeableState::Clean) => "clean",
        Some(MergeableState::Dirty) => "dirty",
        Some(MergeableState::Draft) => "draft",
        Some(MergeableState::HasHooks) => "clean",
        Some(MergeableState::Unstable) => "unstable",
        Some(MergeableState::Unknown) | None => "unknown",
        _ => "unknown",
    }
}

fn resolve_token(path: Option<&str>) -> crate::Result<String> {
    crate::github_token::resolve(path)
}
fn conclusion(value: Option<&str>) -> &'static str {
    match value {
        Some("success") => "success",
        Some("failure" | "timed_out" | "action_required" | "startup_failure") => "failure",
        Some("cancelled") => "cancelled",
        Some("skipped") => "skipped",
        Some("neutral") => "neutral",
        None => "pending",
        _ => "unknown",
    }
}

fn run_is_newer(
    candidate_started_millis: Option<i64>,
    candidate_id: u64,
    prior_started_millis: Option<i64>,
    prior_id: u64,
) -> bool {
    candidate_started_millis.zip(prior_started_millis).map_or(
        candidate_id >= prior_id,
        |(candidate_started, prior_started)| {
            (candidate_started, candidate_id) >= (prior_started, prior_id)
        },
    )
}

fn remote(error: octocrab::Error) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::RemoteFailure,
        format!("GitHub observation failed: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_signals_are_classified_without_network_retry_behavior() {
        assert!(github_rate_limit_signal(
            "API rate limit exceeded for fixture",
            None
        ));
        assert!(github_rate_limit_signal(
            "You have exceeded a secondary rate limit. Please wait a few minutes before you try again.",
            None
        ));
        assert!(github_rate_limit_signal(
            "ordinary message",
            Some("https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api")
        ));
        assert!(github_rate_limit_signal(
            "ordinary message",
            Some("https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits")
        ));
        assert!(!github_rate_limit_signal(
            "rate limit almost matched",
            Some("https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api/extra")
        ));
        for (status, expected) in [
            (404, crate::ErrorCode::RemoteNotFound),
            (401, crate::ErrorCode::RemoteAuthentication),
            (403, crate::ErrorCode::RemoteAuthorization),
            (429, crate::ErrorCode::RemoteRateLimited),
            (500, crate::ErrorCode::RemoteServer),
            (418, crate::ErrorCode::RemoteFailure),
        ] {
            assert_eq!(
                classify_github_issue_read_status(
                    status,
                    "ordinary message",
                    None,
                    "owner/repo",
                    77,
                )
                .code,
                expected
            );
        }
    }

    fn packet(state: &str) -> PrStatePacket {
        PrStatePacket {
            schema: "x".into(),
            repository: "o/r".into(),
            pull_request: 1,
            linked_issue: Some(2),
            linkage_source: Some("github_closing_issues_references".into()),
            state: "open".into(),
            draft: false,
            merge_state: "clean".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_ref: Some("codex/2".into()),
            head_sha: "abc".into(),
            url: Some("https://github.com/o/r/pull/1".into()),
            body: Some("Closes #2".into()),
            merged: false,
            merge_commit_sha: None,
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: state.into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: String::new(),
        }
    }

    #[test]
    fn pr_update_operation_key_is_idempotent_and_conflict_detecting() {
        let governed = append_marker("Closes #596\n", "worker-6-pr-update");
        assert_eq!(
            plan_pull_request_body_update(None, "Closes #596\n", "worker-6-pr-update").unwrap(),
            Some(governed.clone())
        );
        assert_eq!(
            plan_pull_request_body_update(
                Some("Closes #596\n"),
                "Closes #596\n",
                "worker-6-pr-update"
            )
            .unwrap(),
            Some(governed.clone())
        );
        assert_eq!(
            plan_pull_request_body_update(Some(&governed), "Closes #596\n", "worker-6-pr-update")
                .unwrap(),
            None
        );

        let conflicting = append_marker("Closes #597\n", "worker-6-pr-update");
        let error = plan_pull_request_body_update(
            Some(&conflicting),
            "Closes #596\n",
            "worker-6-pr-update",
        )
        .unwrap_err();
        assert_eq!(error.code, crate::ErrorCode::ReconciliationRequired);
    }

    #[test]
    fn review_decision_ignores_approval_from_a_superseded_head() {
        let decision = exact_head_review_decision(
            [
                ("reviewer", Some("old"), Some(ReviewState::Approved), (1, 1)),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "pending");
    }

    #[test]
    fn exact_head_changes_requested_wins_over_exact_head_approval() {
        let decision = exact_head_review_decision(
            [
                ("a", Some("current"), Some(ReviewState::Approved), (1, 1)),
                (
                    "b",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "changes_requested");
    }

    #[test]
    fn later_approval_supersedes_same_reviewer_changes_request() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Approved),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "approved");
    }

    #[test]
    fn later_comment_does_not_revoke_same_reviewer_approval() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Approved),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "approved");
    }

    #[test]
    fn later_comment_does_not_revoke_same_reviewer_changes_request() {
        let decision = exact_head_review_decision(
            [
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::ChangesRequested),
                    (1, 1),
                ),
                (
                    "reviewer",
                    Some("current"),
                    Some(ReviewState::Commented),
                    (2, 2),
                ),
            ],
            "current",
        );
        assert_eq!(decision, "changes_requested");
    }
    #[test]
    fn classifies_common_tail_states() {
        assert_eq!(classify_pr_state(&packet("success"), true), "ready");
        assert_eq!(classify_pr_state(&packet("pending"), false), "waiting");
        assert_eq!(classify_pr_state(&packet("failure"), false), "failed");
    }

    #[test]
    fn newer_check_run_identity_replaces_stale_duplicate_name() {
        assert!(run_is_newer(Some(20), 20, Some(10), 10));
        assert!(run_is_newer(Some(20), 30, Some(20), 20));
        assert!(run_is_newer(None, 30, Some(20), 20));
        assert!(run_is_newer(Some(20), 30, None, 20));
        assert!(!run_is_newer(Some(10), 30, Some(20), 20));
        assert!(!run_is_newer(None, 10, Some(20), 20));
    }

    #[test]
    fn producer_accepts_only_remote_closing_issue_linkage() {
        let response = json!({"data":{"repository":{"pullRequest":{"closingIssuesReferences":{"nodes":[
            {"number": 7, "repository":{"nameWithOwner":"o/r"}},
            {"number": 9, "repository":{"nameWithOwner":"other/r"}}
        ]}}}}});
        assert_eq!(
            remotely_linked_issue(&response, "o/r", Some(7)).unwrap(),
            Some(7)
        );
        let error = remotely_linked_issue(&response, "o/r", Some(8)).unwrap_err();
        assert_eq!(error.code, crate::ErrorCode::ReconciliationRequired);
        let decoded_data = response.get("data").unwrap();
        assert_eq!(
            remotely_linked_issue(decoded_data, "o/r", Some(7)).unwrap(),
            Some(7)
        );
    }

    #[test]
    fn issue_closing_pr_inventory_preserves_multiple_candidates() {
        let response = json!({"data":{"repository":{"issue":{"closedByPullRequestsReferences":{
            "nodes":[
                {"number": 9, "state":"MERGED", "merged":true, "mergedAt":"2026-08-06T00:22:16Z", "repository":{"nameWithOwner":"canonical/repo"}},
                {"number": 10, "state":"CLOSED", "merged":false, "repository":{"nameWithOwner":"other/repo"}},
                {"number": 9, "state":"MERGED", "merged":true, "mergedAt":"2026-08-06T00:22:16Z", "repository":{"nameWithOwner":"canonical/repo"}}
            ],
            "pageInfo":{"hasNextPage":false}
        }}}}});
        assert_eq!(
            closing_pull_request_identities(&response).unwrap(),
            vec![
                ClosingPullRequestIdentity {
                    repository: "canonical/repo".into(),
                    pull_request: 9,
                    state: "MERGED".into(),
                    merged: true,
                    merged_at: Some("2026-08-06T00:22:16Z".into()),
                },
                ClosingPullRequestIdentity {
                    repository: "other/repo".into(),
                    pull_request: 10,
                    state: "CLOSED".into(),
                    merged: false,
                    merged_at: None,
                },
            ]
        );

        let paginated = json!({"repository":{"issue":{"closedByPullRequestsReferences":{
            "nodes":[], "pageInfo":{"hasNextPage":true}
        }}}});
        assert!(closing_pull_request_identities(&paginated).is_err());
    }

    #[test]
    fn classifies_mergeability_states_without_treating_pending_as_stale() {
        for state in ["blocked", "draft", "unknown"] {
            let mut value = packet("success");
            value.merge_state = state.into();
            assert_eq!(classify_pr_state(&value, true), "waiting", "{state}");
        }

        let mut behind = packet("success");
        behind.merge_state = "behind".into();
        assert_eq!(classify_pr_state(&behind, false), "stale_base");

        let mut dirty = packet("success");
        dirty.merge_state = "dirty".into();
        assert_eq!(classify_pr_state(&dirty, false), "conflicted");
    }

    #[test]
    fn unstable_merge_state_ignores_optional_cancelled_and_unknown_checks() {
        let mut value = packet("success");
        value.merge_state = "unstable".into();
        value.checks.extend([
            PrCheck {
                name: "optional-cancelled".into(),
                required: false,
                conclusion: "cancelled".into(),
                details_url: None,
            },
            PrCheck {
                name: "optional-unknown".into(),
                required: false,
                conclusion: "unknown".into(),
                details_url: None,
            },
        ]);
        assert_eq!(classify_pr_state(&value, true), "ready");
    }

    #[test]
    fn unstable_merge_state_still_fails_closed_on_declared_required_checks() {
        let mut failed = packet("failure");
        failed.merge_state = "unstable".into();
        assert_eq!(classify_pr_state(&failed, false), "failed");

        let mut missing = packet("success");
        missing.merge_state = "unstable".into();
        missing.required_check_names.push("coverage".into());
        assert_eq!(classify_pr_state(&missing, false), "waiting");
    }

    #[test]
    fn unstable_merge_state_requires_at_least_one_declared_required_check() {
        let mut value = packet("success");
        value.merge_state = "unstable".into();
        value.required_check_names.clear();
        assert_eq!(classify_pr_state(&value, false), "waiting");
    }

    #[test]
    fn normalizes_every_supported_mergeability_variant_explicitly() {
        let cases = [
            (Some(MergeableState::Behind), "behind"),
            (Some(MergeableState::Blocked), "blocked"),
            (Some(MergeableState::Clean), "clean"),
            (Some(MergeableState::Dirty), "dirty"),
            (Some(MergeableState::Draft), "draft"),
            (Some(MergeableState::HasHooks), "clean"),
            (Some(MergeableState::Unstable), "unstable"),
            (Some(MergeableState::Unknown), "unknown"),
            (None, "unknown"),
        ];
        for (state, expected) in cases {
            assert_eq!(normalize_mergeable_state(state), expected);
        }
    }
}
