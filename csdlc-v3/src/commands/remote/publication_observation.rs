//! Authenticated publication identity and read-only readiness observations.

use crate::adapters::{CommandInvocation, ProcessAdapter};

use super::model::{GithubMutation, GithubMutationRequest, RemoteRouteFinding};
use super::publication::{body_closing_issue_references, body_has_relation};
use super::support::{remote_finding, validate_repository_name, GITHUB_READ_ONLY_ADAPTER};
use super::transport::{mutation_credential_name, read_mutation_reconciliation_page};

/// Authenticate mutable target identity immediately before the production owner
/// can create an intent or dispatch a write. Local publication receipts identify
/// the target; they do not establish its current remote candidate.
pub fn verify_publication_target(
    request: &GithubMutationRequest,
    base: &str,
    branch: &str,
    process: &mut impl ProcessAdapter,
) -> Result<(), RemoteRouteFinding> {
    publication_target_observation(request, base, branch, process).map(|_| ())
}

/// Authenticate readiness without dispatching or changing the retained native operation.
pub fn observe_ready_publication(
    request: &GithubMutationRequest,
    base: &str,
    branch: &str,
    process: &mut impl ProcessAdapter,
) -> Result<serde_json::Value, RemoteRouteFinding> {
    if !matches!(request.mutation, GithubMutation::PullRequestReady) {
        return Err(remote_finding(
            "intent_ready_reconciliation_invalid",
            "readiness observation requires a ready request",
        ));
    }
    let value = publication_target_observation(request, base, branch, process)?;
    if value["draft"] != false {
        return Err(remote_finding(
            "intent_ready_reconciliation_not_ready",
            "authenticated publication is not ready",
        ));
    }
    // Keep only the facts used by this transition, not arbitrary remote body content.
    Ok(
        serde_json::json!({"repository":request.repository,"number":value["number"],
        "head":value["head"]["sha"],"branch":value["head"]["ref"],
        "base":value["base"]["ref"],"state":value["state"],"merged":value["merged"],
        "draft":value["draft"],"closing_issue":request.issue,
        "authenticated":true,"observed_by":"github-api-read-only"}),
    )
}

fn publication_target_observation(
    request: &GithubMutationRequest,
    base: &str,
    branch: &str,
    process: &mut impl ProcessAdapter,
) -> Result<serde_json::Value, RemoteRouteFinding> {
    let number = request
        .pull_request
        .filter(|number| *number > 0)
        .ok_or_else(|| {
            remote_finding(
                "intent_publication_target_missing",
                "existing publication requires a numeric target",
            )
        })?;
    validate_repository_name(&request.repository)?;
    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [
            "pull-request".to_owned(),
            request.repository.clone(),
            number.to_string(),
        ],
    )
    .and_then(|invocation| {
        invocation.with_child_credential(mutation_credential_name(request).unwrap_or_default())
    })
    .map_err(|_| {
        remote_finding(
            "intent_publication_observation_invalid",
            "publication readback invocation is invalid",
        )
    })?;
    let value = read_mutation_reconciliation_page(invocation, process)?;
    if value["number"].as_u64() != Some(number)
        || value["head"]["sha"] != request.expected_head_sha
        || value["head"]["ref"] != branch
        || value["base"]["ref"] != base
        || value["state"] != "open"
        || value["merged"] != false
        || !body_has_relation(value["body"].as_str(), "Closes", request.issue)
        || body_closing_issue_references(value["body"].as_str())
            .iter()
            .any(|issue| *issue != request.issue)
    {
        return Err(remote_finding("intent_publication_remote_identity_mismatch","authenticated PR must match current head, branch, base, open state and canonical closing linkage before mutation"));
    }
    Ok(value)
}
