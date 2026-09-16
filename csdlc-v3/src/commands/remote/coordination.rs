//! Explicit coordination-only completion; administrative closure remains separate.
use super::*;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::io::Read;

const CONTRACT_PREFIX: &str = "<!-- csdlc-coordination:v1 ";
const MAX_EVIDENCE_BYTES: u64 = 4 * 1024 * 1024;

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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Contract {
    repository: String,
    issue: u64,
    kind: String,
    children: Vec<Child>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Child {
    issue: u64,
    pull_request: u64,
    head_sha: String,
}
fn reject(message: &str) -> RemoteRouteFinding {
    remote_finding("github_coordination_completion_denied", message)
}
fn ensure(condition: bool, message: &str) -> Result<(), RemoteRouteFinding> {
    if condition {
        Ok(())
    } else {
        Err(reject(message))
    }
}
fn hex(value: &str, len: usize) -> bool {
    value.len() == len && value.bytes().all(|b| b.is_ascii_hexdigit())
}
fn contract(
    request: &GithubMutationRequest,
    completion: &CoordinationCompletion,
) -> Result<Contract, RemoteRouteFinding> {
    let lines: Vec<_> = completion
        .current_body
        .lines()
        .filter(|line| line.contains("csdlc-coordination:"))
        .collect();
    ensure(
        lines.len() == 1,
        "one explicit coordination contract is required",
    )?;
    let encoded = lines[0]
        .strip_prefix(CONTRACT_PREFIX)
        .and_then(|line| line.strip_suffix(" -->"))
        .ok_or_else(|| reject("coordination contract marker is malformed"))?;
    let contract: Contract =
        serde_json::from_str(encoded).map_err(|_| reject("coordination contract is invalid"))?;
    ensure(
        contract.repository == request.repository
            && contract.issue == request.issue
            && contract.kind == "coordination_only",
        "coordination contract identity or classification mismatch",
    )?;
    ensure(
        !contract.children.is_empty() && contract.children.len() <= 100,
        "coordination child set must contain 1 to 100 children",
    )?;
    let mut issues = BTreeSet::new();
    let mut prs = BTreeSet::new();
    for child in &contract.children {
        ensure(
            child.issue > 0
                && child.issue != request.issue
                && child.pull_request > 0
                && hex(&child.head_sha, 40)
                && issues.insert(child.issue)
                && prs.insert(child.pull_request),
            "coordination child identities must be distinct and exact",
        )?;
    }
    Ok(contract)
}

pub(super) fn validate(request: &GithubMutationRequest) -> Result<(), RemoteRouteFinding> {
    let GithubMutation::IssueCompleteCoordination { completion } = &request.mutation else {
        return Ok(());
    };
    ensure(
        request.pull_request.is_none()
            && request.issue > 0
            && hex(&request.expected_head_sha, 40)
            && request
                .operator_approval
                .as_ref()
                .is_some_and(|v| !v.trim().is_empty())
            && !completion.rationale.trim().is_empty()
            && !completion.expected_updated_at.trim().is_empty()
            && completion.current_body.len() <= 65536,
        "completion requires explicit operator approval, exact issue snapshot and rationale",
    )?;
    ensure(
        !completion.evidence.is_empty() && completion.evidence.len() <= 32,
        "bounded durable completion evidence is required",
    )?;
    let mut paths = BTreeSet::new();
    for evidence in &completion.evidence {
        ensure(
            !evidence.path.trim().is_empty()
                && hex(&evidence.digest, 64)
                && paths.insert(&evidence.path),
            "evidence paths and digests must be distinct and valid",
        )?;
    }
    contract(request, completion)?;
    Ok(())
}
fn observe(
    request: &GithubMutationRequest,
    operation: &str,
    target: String,
    process: &mut impl ProcessAdapter,
) -> Result<Value, RemoteRouteFinding> {
    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [operation.into(), request.repository.clone(), target],
    )
    .and_then(|i| i.with_child_credential(mutation_credential_name(request).unwrap_or_default()))
    .map_err(|_| reject("authenticated observation invocation invalid"))?;
    let value = read_mutation_reconciliation_page(invocation, process)?;
    ensure(
        value.get("errors").is_none(),
        "partial authenticated observation rejected",
    )?;
    Ok(value)
}
fn verify_evidence(
    root: &Path,
    evidence: &[CoordinationEvidence],
) -> Result<(), RemoteRouteFinding> {
    let root = root
        .canonicalize()
        .map_err(|_| reject("evidence root unavailable"))?;
    for reference in evidence {
        let candidate = root.join(&reference.path);
        let path = candidate
            .canonicalize()
            .map_err(|_| reject("completion evidence missing"))?;
        ensure(
            is_repo_or_git_receipt_path(&root, &path) && is_durable_receipt_path(&root, &path),
            "completion evidence must be durable and repository scoped",
        )?;
        let file = fs::File::open(&path).map_err(|_| reject("completion evidence unreadable"))?;
        let metadata = file
            .metadata()
            .map_err(|_| reject("completion evidence metadata unavailable"))?;
        ensure(
            metadata.is_file() && metadata.len() > 0 && metadata.len() <= MAX_EVIDENCE_BYTES,
            "completion evidence size invalid",
        )?;
        let mut bytes = Vec::new();
        file.take(MAX_EVIDENCE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| reject("completion evidence unreadable"))?;
        ensure(
            bytes.len() as u64 <= MAX_EVIDENCE_BYTES
                && blake3::hash(&bytes).to_hex().as_str() == reference.digest,
            "completion evidence digest mismatch",
        )?;
    }
    Ok(())
}

/// Rechecked directly before every dispatch, including an explicitly requested retry.
/// Already reconciled operations use their immutable receipt and never dispatch again.
pub(super) fn verify(
    root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<(), RemoteRouteFinding> {
    let GithubMutation::IssueCompleteCoordination { completion } = &request.mutation else {
        return Ok(());
    };
    validate(request)?;
    let contract = contract(request, completion)?;
    verify_evidence(root, &completion.evidence)?;
    let parent = observe(request, "issue", request.issue.to_string(), process)?;
    ensure(
        parent["number"] == request.issue
            && parent.get("pull_request").is_none()
            && parent["html_url"]
                == format!(
                    "https://github.com/{}/issues/{}",
                    request.repository, request.issue
                )
            && parent["state"] == "open"
            && parent["body"] == completion.current_body
            && parent["updated_at"] == completion.expected_updated_at,
        "approved coordination issue snapshot is stale or mismatched",
    )?;
    let mut observed_children = Vec::new();
    for child in contract.children {
        let issue = observe(request, "issue", child.issue.to_string(), process)?;
        ensure(
            issue["number"] == child.issue
                && issue.get("pull_request").is_none()
                && issue["html_url"]
                    == format!(
                        "https://github.com/{}/issues/{}",
                        request.repository, child.issue
                    )
                && issue["state"] == "closed"
                && issue["state_reason"] == "completed",
            "coordination child is not authentically completed",
        )?;
        let mut child_request = request.clone();
        child_request.issue = child.issue;
        child_request.pull_request = Some(child.pull_request);
        child_request.expected_head_sha = child.head_sha.clone();
        let linkage = PublicationLinkage {
            repository: request.repository.clone(),
            issue: child.issue,
            mode: RemotePublicationMode::Closing,
        };
        let merged = observe(
            request,
            "pull-request-merge-linkage",
            linkage.observation_target(&child_request),
            process,
        )?;
        let repo = &merged["data"]["repository"];
        let pr = &repo["pullRequest"];
        ensure(
            repo["nameWithOwner"] == request.repository
                && pr["number"] == child.pull_request
                && pr["url"]
                    == format!(
                        "https://github.com/{}/pull/{}",
                        request.repository, child.pull_request
                    )
                && pr["headRefOid"] == child.head_sha
                && pr["merged"] == true
                && pr["state"] == "MERGED"
                && pr["mergeCommit"]["oid"]
                    .as_str()
                    .is_some_and(|v| hex(v, 40)),
            "coordination child merged PR identity mismatch",
        )?;
        linkage.validate_completed_child(&merged, &child_request)?;
        observed_children.push(json!({"issue":child.issue,"pull_request":child.pull_request,"head_sha":child.head_sha,
            "issue_observation_digest":blake3::hash(&serde_json::to_vec(&issue).map_err(|_| reject("observation encoding failed"))?).to_hex().to_string(),
            "merge_observation_digest":blake3::hash(&serde_json::to_vec(&merged).map_err(|_| reject("observation encoding failed"))?).to_hex().to_string()}));
    }
    let latest_parent = observe(request, "issue", request.issue.to_string(), process)?;
    ensure(
        latest_parent == parent,
        "coordination parent changed during readiness verification",
    )?;
    verify_evidence(root, &completion.evidence)?;
    // Store only bounded identity/digest facts, never issue bodies or evidence payloads.
    let receipt = json!({"schema":"csdlc-coordination-readiness/v1","repository":request.repository,"issue":request.issue,
        "expected_head_sha":request.expected_head_sha,"operator_approval":request.operator_approval,
        "operation_digest":github_mutation_operation_digest(request),
        "parent_body_digest":blake3::hash(completion.current_body.as_bytes()).to_hex().to_string(),
        "expected_updated_at":completion.expected_updated_at,"evidence":completion.evidence,"children":observed_children});
    let digest = blake3::hash(
        &serde_json::to_vec(&receipt).map_err(|_| reject("readiness encoding failed"))?,
    )
    .to_hex()
    .to_string();
    let dir = git_control_dir(root)
        .ok_or_else(|| reject("Git control directory unavailable"))?
        .join("csdlc-v3/coordination-readiness");
    fs::create_dir_all(&dir).map_err(|_| reject("readiness directory unavailable"))?;
    let path = dir.join(format!("{digest}.json"));
    if path.exists() {
        let existing: Value = serde_json::from_slice(
            &fs::read(&path).map_err(|_| reject("readiness receipt unreadable"))?,
        )
        .map_err(|_| reject("readiness receipt invalid"))?;
        ensure(existing == receipt, "readiness receipt mismatch")?;
    } else {
        persist_json_create_new(&path, &receipt)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
