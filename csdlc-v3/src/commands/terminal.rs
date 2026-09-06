use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
};

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};

const GITHUB_READ_ONLY_ADAPTER: &str = "github-api-read-only";

pub const TERMINAL_ROUTE_NAMES: [&str; 3] = ["finish", "clean", "cutover"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalRouteRequest {
    pub repository: String,
    pub issue: u64,
    #[serde(default)]
    pub pull_request: Option<u64>,
    #[serde(default)]
    pub expected_head_sha: Option<String>,
    #[serde(default)]
    pub mode: Option<TerminalPublicationMode>,
    #[serde(default)]
    pub public_adapter_receipt: Option<AdapterReceipt>,
    #[serde(default)]
    pub terminal_state: Option<TerminalStateWriteRequest>,
    #[serde(default)]
    pub cleanup: Option<CleanupRouteRequest>,
    #[serde(default)]
    pub cutover: Option<CutoverDecisionRequest>,
    #[serde(default)]
    pub credential_names: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalPublicationMode {
    Closing,
    PartOf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterReceipt {
    pub producer: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupRouteRequest {
    pub approved_parent: PathBuf,
    pub repository_root: PathBuf,
    pub candidate_path: PathBuf,
    #[serde(default)]
    pub remove: bool,
    #[serde(default)]
    pub terminal_receipt: bool,
    #[serde(default)]
    pub terminal_receipt_path: Option<String>,
    #[serde(default)]
    pub terminal_receipt_digest: Option<String>,
    #[serde(default)]
    pub preview_receipt_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableTerminalReceipt {
    pub schema: String,
    pub repository: String,
    pub issue: u64,
    pub pull_request: u64,
    pub head_sha: String,
    pub disposition: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStateWriteRequest {
    pub repository_root: PathBuf,
    pub state_path: PathBuf,
    pub receipt_path: PathBuf,
    #[serde(default)]
    pub expected_state_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutoverDecisionRequest {
    pub operator: String,
    pub approval: String,
    pub selected_binary_provenance: String,
    pub rollback_evidence: String,
    pub undo_boundary: String,
    #[serde(default)]
    pub operation: CutoverOperation,
    #[serde(default)]
    pub execute: bool,
    #[serde(default)]
    pub repository_root: Option<PathBuf>,
    #[serde(default)]
    pub selected_binary_path: Option<PathBuf>,
    #[serde(default)]
    pub authority_selector_path: Option<PathBuf>,
    #[serde(default)]
    pub install_destination_path: Option<PathBuf>,
    #[serde(default)]
    pub rollback_receipt_path: Option<PathBuf>,
    #[serde(default)]
    pub readiness_evidence_path: Option<PathBuf>,
    #[serde(default)]
    pub readiness_evidence_digest: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CutoverOperation {
    #[default]
    Apply,
    Rollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalRoutePlan {
    pub route: String,
    pub issue: u64,
    pub repository: String,
    pub status: TerminalRouteStatus,
    pub operational_authority: bool,
    pub findings: Vec<TerminalFinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish: Option<FinishDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup: Option<CleanupDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutover: Option<CutoverDecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalRouteStatus {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalFinding {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum FinishDecision {
    TerminalClosedOut {
        pull_request: u64,
        issue: u64,
        head_sha: String,
    },
    CheckpointDenied {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case")]
pub enum CleanupDecision {
    Absent {
        path: PathBuf,
    },
    Unregistered {
        path: PathBuf,
    },
    Dirty {
        path: PathBuf,
    },
    Live {
        path: PathBuf,
    },
    AlreadyRemoved {
        path: PathBuf,
    },
    Removable {
        path: PathBuf,
        receipt_digest: String,
    },
    Removed {
        path: PathBuf,
        receipt_digest: String,
    },
    RemovalDeniedPreCutover {
        path: PathBuf,
        receipt_digest: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CutoverDecision {
    pub operation: CutoverOperation,
    pub approved_by: String,
    pub selected_binary_provenance: String,
    pub rollback_evidence: String,
    pub undo_boundary: String,
    pub executes_cutover: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_selector_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_destination_path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_receipt_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedTerminalReadback {
    repository: String,
    issue: u64,
    pull_request: u64,
    head_sha: String,
    mode: TerminalPublicationMode,
    pr_merged: bool,
    issue_open: bool,
    closes_issue: Option<u64>,
    part_of_issue: Option<u64>,
}

impl VerifiedTerminalReadback {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_typed_adapter_receipt(
        producer: &str,
        repository: String,
        issue: u64,
        pull_request: u64,
        head_sha: String,
        mode: TerminalPublicationMode,
        pr_merged: bool,
        issue_open: bool,
        closes_issue: Option<u64>,
        part_of_issue: Option<u64>,
    ) -> Result<Self, TerminalFinding> {
        if producer != "csdlc-github-pr-state" {
            return Err(finding(
                "authenticated_adapter_required",
                "terminal readback must be produced by the typed GitHub PR-state adapter",
            ));
        }
        Ok(Self {
            repository,
            issue,
            pull_request,
            head_sha,
            mode,
            pr_merged,
            issue_open,
            closes_issue,
            part_of_issue,
        })
    }
}

pub fn prepare_terminal_route(
    route: &str,
    request: &TerminalRouteRequest,
) -> Result<TerminalRoutePlan, TerminalFinding> {
    prepare_terminal_route_with_cutover_authority(route, request, None)
}

fn prepare_terminal_route_with_cutover_authority(
    route: &str,
    request: &TerminalRouteRequest,
    cutover_comments: Option<&[GithubIssueComment]>,
) -> Result<TerminalRoutePlan, TerminalFinding> {
    if !TERMINAL_ROUTE_NAMES.contains(&route) {
        return Err(finding(
            "unknown_terminal_route",
            "route is not owned by #630",
        ));
    }
    let mut findings = Vec::new();
    let mut finish = None;
    let mut cleanup = None;
    let mut cutover = None;
    match route {
        "finish" => findings.extend(public_finish_findings(request)),
        "clean" => match request.cleanup.as_ref() {
            Some(cleanup_request) => match classify_cleanup_from_git(request, cleanup_request) {
                Ok(decision) => {
                    if matches!(decision, CleanupDecision::RemovalDeniedPreCutover { .. }) {
                        findings.push(finding(
                            "cleanup_removal_denied_pre_cutover",
                            "v3 clean cannot remove worktrees before #505 cutover authority",
                        ));
                    }
                    cleanup = Some(decision);
                }
                Err(finding) => findings.push(finding),
            },
            None => findings.push(finding(
                "missing_cleanup_request",
                "clean route requires a cleanup request",
            )),
        },
        "cutover" => match request.cutover.as_ref() {
            Some(cutover_request) => match decide_cutover(cutover_request, cutover_comments) {
                Ok(decision) => cutover = Some(decision),
                Err(route_findings) => findings.extend(route_findings),
            },
            None => findings.push(finding(
                "missing_cutover_request",
                "cutover route requires a cutover decision request",
            )),
        },
        _ => unreachable!("route checked above"),
    }
    let status = if findings.is_empty() {
        TerminalRouteStatus::Ready
    } else {
        TerminalRouteStatus::Blocked
    };
    if route == "finish" && status == TerminalRouteStatus::Blocked {
        finish = Some(FinishDecision::CheckpointDenied {
            reason: "public finish route has no authenticated adapter receipt".into(),
        });
    }
    let operational_authority = cutover
        .as_ref()
        .is_some_and(|decision| decision.executes_cutover);
    Ok(TerminalRoutePlan {
        route: route.to_owned(),
        issue: request.issue,
        repository: request.repository.clone(),
        status,
        operational_authority,
        findings,
        finish,
        cleanup,
        cutover,
    })
}

pub fn prepare_terminal_cutover_with_github_observation(
    request: &TerminalRouteRequest,
    process: &mut impl ProcessAdapter,
) -> Result<TerminalRoutePlan, TerminalFinding> {
    if request.issue != 505 {
        return Err(finding(
            "cutover_issue_mismatch",
            "authenticated cutover observation is restricted to authority issue #505",
        ));
    }
    validate_repository_name(&request.repository)?;
    let credential_name = single_credential_name(request)?;
    let value = run_github_observation(
        process,
        &credential_name,
        "issue-comments",
        &request.repository,
        request.issue,
    )?;
    let comments: Vec<GithubIssueComment> = serde_json::from_value(value).map_err(|_| {
        finding(
            "github_cutover_comments_invalid",
            "authenticated GitHub issue comments must be a typed JSON array",
        )
    })?;
    prepare_terminal_route_with_cutover_authority("cutover", request, Some(&comments))
}

pub fn prepare_terminal_finish_with_github_observation(
    request: &TerminalRouteRequest,
    process: &mut impl ProcessAdapter,
) -> Result<TerminalRoutePlan, TerminalFinding> {
    let mut findings = Vec::new();
    let finish = match observe_terminal_github_readback(request, process)
        .and_then(|readback| derive_finish_from_verified(request, readback))
        .and_then(|decision| {
            persist_terminal_finish(request, &decision)?;
            Ok(decision)
        }) {
        Ok(decision) => Some(decision),
        Err(finding) => {
            findings.push(finding);
            Some(FinishDecision::CheckpointDenied {
                reason: "finish requires authenticated terminal GitHub readback".into(),
            })
        }
    };
    let status = if findings.is_empty() {
        TerminalRouteStatus::Ready
    } else {
        TerminalRouteStatus::Blocked
    };
    Ok(TerminalRoutePlan {
        route: "finish".to_owned(),
        issue: request.issue,
        repository: request.repository.clone(),
        status,
        operational_authority: false,
        findings,
        finish,
        cleanup: None,
        cutover: None,
    })
}

pub fn derive_finish_from_verified(
    request: &TerminalRouteRequest,
    readback: VerifiedTerminalReadback,
) -> Result<FinishDecision, TerminalFinding> {
    if request.repository != readback.repository {
        return Err(finding(
            "repository_mismatch",
            "finish readback repository does not match request",
        ));
    }
    if request.issue != readback.issue {
        return Err(finding(
            "issue_mismatch",
            "finish readback issue does not match request",
        ));
    }
    if request.pull_request != Some(readback.pull_request) {
        return Err(finding(
            "pull_request_mismatch",
            "finish readback pull request does not match request",
        ));
    }
    if request.expected_head_sha.as_deref() != Some(readback.head_sha.as_str()) {
        return Err(finding(
            "head_mismatch",
            "finish readback head does not match the expected exact head",
        ));
    }
    if !readback.pr_merged {
        return Err(finding(
            "pull_request_not_merged",
            "finish requires a merged pull request",
        ));
    }
    match readback.mode {
        TerminalPublicationMode::Closing if readback.closes_issue != Some(readback.issue) => {
            Err(finding(
                "closing_linkage_missing",
                "finish requires an authenticated closing relation",
            ))
        }
        TerminalPublicationMode::Closing if readback.issue_open => Err(finding(
            "closing_issue_still_open",
            "finish requires the closed issue readback for closing publications",
        )),
        TerminalPublicationMode::Closing => Ok(FinishDecision::TerminalClosedOut {
            pull_request: readback.pull_request,
            issue: readback.issue,
            head_sha: readback.head_sha,
        }),
        TerminalPublicationMode::PartOf => Ok(FinishDecision::CheckpointDenied {
            reason: "part_of publications cannot close a terminal issue".into(),
        }),
    }
}

fn observe_terminal_github_readback(
    request: &TerminalRouteRequest,
    process: &mut impl ProcessAdapter,
) -> Result<VerifiedTerminalReadback, TerminalFinding> {
    let pull_request = request.pull_request.ok_or_else(|| {
        finding(
            "missing_pull_request",
            "authenticated terminal observation requires a concrete pull request number",
        )
    })?;
    let credential_name = single_credential_name(request)?;
    validate_repository_name(&request.repository)?;
    let pr_value = run_github_observation(
        process,
        &credential_name,
        "pull-request",
        &request.repository,
        pull_request,
    )?;
    let number = pr_value["number"].as_u64().ok_or_else(|| {
        finding(
            "github_observation_missing_pr_number",
            "GitHub pull request readback did not include a PR number",
        )
    })?;
    if number != pull_request {
        return Err(finding(
            "github_observation_pr_mismatch",
            "GitHub pull request readback number does not match the requested PR",
        ));
    }
    let head_sha = pr_value["head"]["sha"].as_str().ok_or_else(|| {
        finding(
            "github_observation_missing_head",
            "GitHub pull request readback did not include head.sha",
        )
    })?;
    let pr_merged = pr_value["merged"].as_bool().ok_or_else(|| {
        finding(
            "github_observation_missing_merged",
            "GitHub pull request readback did not include merged",
        )
    })?;
    let body = pr_value["body"].as_str().unwrap_or_default();
    let closes_issue =
        body_has_relation(Some(body), "Closes", request.issue).then_some(request.issue);
    let part_of_issue = (body_has_relation(Some(body), "Part of", request.issue)
        || body_has_relation(Some(body), "Part-Of", request.issue))
    .then_some(request.issue);

    let issue_value = run_github_observation(
        process,
        &credential_name,
        "issue",
        &request.repository,
        request.issue,
    )?;
    let issue_number = issue_value["number"].as_u64().ok_or_else(|| {
        finding(
            "github_observation_missing_issue_number",
            "GitHub issue readback did not include an issue number",
        )
    })?;
    if issue_number != request.issue {
        return Err(finding(
            "github_observation_issue_mismatch",
            "GitHub issue readback number does not match the requested issue",
        ));
    }
    let issue_state = issue_value["state"].as_str().ok_or_else(|| {
        finding(
            "github_observation_missing_issue_state",
            "GitHub issue readback did not include state",
        )
    })?;
    let issue_open = match issue_state {
        "open" => true,
        "closed" => false,
        _ => {
            return Err(finding(
                "github_observation_unknown_issue_state",
                "GitHub issue state must be open or closed",
            ))
        }
    };

    VerifiedTerminalReadback::from_typed_adapter_receipt(
        "csdlc-github-pr-state",
        request.repository.clone(),
        request.issue,
        pull_request,
        head_sha.to_owned(),
        request.mode.unwrap_or(TerminalPublicationMode::PartOf),
        pr_merged,
        issue_open,
        closes_issue,
        part_of_issue,
    )
}

fn run_github_observation(
    process: &mut impl ProcessAdapter,
    credential_name: &str,
    operation: &str,
    repository: &str,
    number: u64,
) -> Result<serde_json::Value, TerminalFinding> {
    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [
            operation.to_owned(),
            repository.to_owned(),
            number.to_string(),
        ],
    )
    .map_err(|_| {
        finding(
            "github_observation_invocation_rejected",
            "authenticated GitHub observation must use structured argv without shell strings or secrets",
        )
    })?
    .with_child_credential(credential_name)
    .map_err(|_| {
        finding(
            "github_credential_scope_invalid",
            "GitHub credential names must be explicit safe child-process environment names",
        )
    })?;
    let output = process.run(invocation);
    if output.truncated {
        return Err(finding(
            "github_observation_truncated",
            "GitHub readback output was truncated and cannot be authoritative input",
        ));
    }
    if output.status != ProcessStatus::Exit(0) {
        return Err(finding(
            "github_observation_failed",
            "GitHub readback adapter did not complete successfully",
        ));
    }
    serde_json::from_str(&output.stdout).map_err(|_| {
        finding(
            "github_observation_invalid_json",
            "GitHub readback adapter returned non-JSON output",
        )
    })
}

fn single_credential_name(request: &TerminalRouteRequest) -> Result<String, TerminalFinding> {
    match request.credential_names.as_slice() {
        [name] => Ok(name.clone()),
        [] => Err(finding(
            "github_credential_missing",
            "authenticated GitHub observation requires exactly one credential name",
        )),
        _ => Err(finding(
            "github_credential_ambiguous",
            "authenticated GitHub observation requires one unambiguous credential name",
        )),
    }
}

fn validate_repository_name(repository: &str) -> Result<(), TerminalFinding> {
    let Some((owner, name)) = repository.split_once('/') else {
        return Err(finding(
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
        return Err(finding(
            "github_repository_invalid",
            "repository contains characters that cannot be used in structured GitHub readback",
        ));
    }
    Ok(())
}

fn body_has_relation(body: Option<&str>, verb: &str, issue: u64) -> bool {
    let prefix = format!("{verb} #{issue}");
    body.unwrap_or_default()
        .lines()
        .any(|line| has_issue_relation_prefix(line.trim_start(), &prefix))
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

fn public_finish_findings(request: &TerminalRouteRequest) -> Vec<TerminalFinding> {
    let mut findings = Vec::new();
    if request.pull_request.unwrap_or_default() == 0 {
        findings.push(finding(
            "missing_pull_request",
            "finish requires a pull request identity",
        ));
    }
    if request
        .expected_head_sha
        .as_deref()
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        findings.push(finding(
            "missing_exact_head",
            "finish requires the exact reviewed/published head",
        ));
    }
    if request.mode != Some(TerminalPublicationMode::Closing) {
        findings.push(finding(
            "terminal_requires_closing_publication",
            "finish cannot terminally close a part_of checkpoint",
        ));
    }
    findings.push(finding(
        "authenticated_adapter_required",
        "public finish route cannot accept caller-authored PR or issue readback before #505 cutover",
    ));
    findings
}

fn classify_cleanup_from_git(
    terminal_request: &TerminalRouteRequest,
    request: &CleanupRouteRequest,
) -> Result<CleanupDecision, TerminalFinding> {
    let approved_parent = canonical_dir(&request.approved_parent, "approved_parent")?;
    let repository_root = canonical_dir(&request.repository_root, "repository_root")?;
    if contains_parent_component(&request.candidate_path) {
        return Err(finding(
            "path_not_normalized",
            "cleanup target must not contain parent-directory traversal",
        ));
    }
    verify_terminal_receipt(&repository_root, terminal_request, request)?;
    if !request.candidate_path.exists() {
        let existing_parent = canonical_existing_ancestor(&request.candidate_path)?;
        if !existing_parent.starts_with(&approved_parent) {
            return Err(finding(
                "path_outside_approved_parent",
                "cleanup target parent must remain beneath the approved worktree parent",
            ));
        }
    }
    let candidate = if request.candidate_path.exists() {
        canonical_dir(&request.candidate_path, "candidate_path")?
    } else {
        request.candidate_path.clone()
    };
    if candidate == repository_root || repository_root.starts_with(&candidate) {
        return Err(finding(
            "protected_path",
            "cleanup must not target the repository root or an ancestor",
        ));
    }
    if !candidate.starts_with(&approved_parent) {
        return Err(finding(
            "path_outside_approved_parent",
            "cleanup target must remain beneath the approved worktree parent",
        ));
    }
    let registrations = git_worktree_paths(&repository_root)?;
    let registered = registrations.iter().any(|path| path == &candidate);
    if !request.candidate_path.exists() {
        return Ok(if registered {
            CleanupDecision::AlreadyRemoved {
                path: request.candidate_path.clone(),
            }
        } else {
            CleanupDecision::Absent {
                path: request.candidate_path.clone(),
            }
        });
    }
    if !registered {
        return Ok(CleanupDecision::Unregistered { path: candidate });
    }
    if worktree_live(&candidate) {
        return Ok(CleanupDecision::Live { path: candidate });
    }
    if worktree_dirty(&candidate)? {
        return Ok(CleanupDecision::Dirty { path: candidate });
    }
    let candidate_head = worktree_head(&candidate)?;
    let expected_head = terminal_request
        .expected_head_sha
        .as_deref()
        .ok_or_else(|| {
            finding(
                "missing_exact_head",
                "cleanup requires the terminal expected head to match the registered worktree",
            )
        })?;
    if candidate_head != expected_head {
        return Err(finding(
            "candidate_head_mismatch",
            "cleanup target worktree HEAD must match the terminal closeout head",
        ));
    }
    let receipt_digest = cleanup_receipt_digest(&repository_root, &candidate, &candidate_head);
    if request.remove {
        if request.preview_receipt_digest.as_deref() != Some(receipt_digest.as_str()) {
            return Err(finding(
                "preview_receipt_mismatch",
                "cleanup removal requires a preview receipt for the same Git registration",
            ));
        }
        if !canonical_v3_authority(&repository_root, Some(expected_head))? {
            return Ok(CleanupDecision::RemovalDeniedPreCutover {
                path: candidate,
                receipt_digest,
                reason:
                    "v3 clean is non-authoritative before #505 cutover and must not remove worktrees"
                        .into(),
            });
        }
        remove_registered_worktree(&repository_root, &candidate)?;
        Ok(CleanupDecision::Removed {
            path: candidate,
            receipt_digest,
        })
    } else {
        Ok(CleanupDecision::Removable {
            path: candidate,
            receipt_digest,
        })
    }
}

fn persist_terminal_finish(
    request: &TerminalRouteRequest,
    decision: &FinishDecision,
) -> Result<(), TerminalFinding> {
    let Some(write_request) = request.terminal_state.as_ref() else {
        return Ok(());
    };
    let repository_root = canonical_dir(&write_request.repository_root, "repository_root")?;
    if !canonical_v3_authority(&repository_root, request.expected_head_sha.as_deref())? {
        return Err(finding(
            "terminal_persistence_denied_pre_cutover",
            "v3 finish cannot persist terminal state before the canonical selector activates v3",
        ));
    }
    let FinishDecision::TerminalClosedOut {
        pull_request,
        issue,
        head_sha,
    } = decision
    else {
        return Err(finding(
            "terminal_state_requires_closeout",
            "only a verified terminal closeout may be persisted",
        ));
    };
    let state_path = checked_repo_relative(
        &repository_root,
        &write_request.state_path,
        "terminal_state",
    )?;
    let receipt_path = checked_repo_relative(
        &repository_root,
        &write_request.receipt_path,
        "terminal_receipt",
    )?;
    if state_path != repository_root.join(format!(".csdlc/v3/issues/{issue}/terminal.json"))
        || receipt_path
            != repository_root.join(format!(".csdlc/evidence/{issue}/terminal-receipt.json"))
    {
        return Err(finding(
            "terminal_output_path_not_canonical",
            "v3 terminal state and receipt must use their canonical issue-scoped paths",
        ));
    }
    ensure_output_parent_inside_repo(&repository_root, &state_path, "terminal_state")?;
    ensure_output_parent_inside_repo(&repository_root, &receipt_path, "terminal_receipt")?;
    let state_bytes = serde_json::to_vec_pretty(&serde_json::json!({
        "schema": "csdlc.v3.terminal_state.v1",
        "repository": request.repository,
        "issue": issue,
        "pull_request": pull_request,
        "head_sha": head_sha,
        "disposition": "closed_out"
    }))
    .map_err(|error| finding("terminal_state_serialize_failed", &error.to_string()))?;
    let state_digest = blake3::hash(&state_bytes).to_hex().to_string();
    if let Ok(existing) = fs::read(&state_path) {
        let existing_digest = blake3::hash(&existing).to_hex().to_string();
        if existing != state_bytes
            && write_request.expected_state_digest.as_deref() != Some(existing_digest.as_str())
        {
            return Err(finding(
                "terminal_state_stale_digest",
                "existing v3 terminal state changed since the caller's expected digest",
            ));
        }
    }
    write_staged(&state_path, &state_bytes)?;
    let receipt = DurableTerminalReceipt {
        schema: "csdlc.v3.terminal_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: *issue,
        pull_request: *pull_request,
        head_sha: head_sha.clone(),
        disposition: "closed_out".into(),
        state_digest: Some(state_digest),
    };
    let receipt_bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| finding("terminal_receipt_serialize_failed", &error.to_string()))?;
    if let Ok(existing) = fs::read(&receipt_path) {
        if existing != receipt_bytes {
            return Err(finding(
                "terminal_receipt_conflict",
                "existing terminal receipt does not match the verified closeout state",
            ));
        }
        return Ok(());
    }
    write_staged(&receipt_path, &receipt_bytes)
}

fn canonical_v3_authority(
    repository_root: &Path,
    expected_head: Option<&str>,
) -> Result<bool, TerminalFinding> {
    let selector_path = repository_root.join("csdlc-v2/operator/generation-selector.json");
    let metadata = match selector_path.symlink_metadata() {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(finding(
                "generation_selector_unreadable",
                &format!("canonical generation selector metadata is unavailable: {error}"),
            ))
        }
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(finding(
            "generation_selector_not_regular_file",
            "canonical generation selector must be a regular file",
        ));
    }
    let selector: serde_json::Value =
        serde_json::from_slice(&fs::read(&selector_path).map_err(|error| {
            finding(
                "generation_selector_unreadable",
                &format!("canonical generation selector is unreadable: {error}"),
            )
        })?)
        .map_err(|_| {
            finding(
                "generation_selector_invalid",
                "canonical generation selector must be typed JSON",
            )
        })?;
    if selector["schema"] == "csdlc.generation_selector.v1" {
        return Ok(false);
    }
    if selector["schema"] != "csdlc.generation_selector.v2" {
        return Err(finding(
            "generation_selector_invalid",
            "canonical generation selector must use the evidence-bound v2 schema",
        ));
    }
    Ok(selector["default_generation"] == "v3"
        && selector["operational_authority"] == "csdlc-v3"
        && selector["authority_issue"] == 505
        && expected_head.is_some_and(|head| selector["exact_review_sha"] == head)
        && selector["readiness_evidence_digest"]
            .as_str()
            .is_some_and(|digest| !digest.is_empty())
        && selector["approval_evidence_digest"]
            .as_str()
            .is_some_and(|digest| !digest.is_empty()))
}

fn remove_registered_worktree(
    repository_root: &Path,
    candidate: &Path,
) -> Result<(), TerminalFinding> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(["worktree", "remove", "--"])
        .arg(candidate)
        .output()
        .map_err(|error| {
            finding(
                "cleanup_remove_failed",
                &format!("could not invoke git worktree remove: {error}"),
            )
        })?;
    if !output.status.success() {
        return Err(finding(
            "cleanup_remove_failed",
            "git refused removal of the exact clean registered terminal worktree",
        ));
    }
    if candidate.exists()
        || git_worktree_paths(repository_root)?
            .iter()
            .any(|registered| registered == candidate)
    {
        return Err(finding(
            "cleanup_remove_unreconciled",
            "worktree removal did not reconcile both filesystem and Git registration state",
        ));
    }
    Ok(())
}

fn verify_terminal_receipt(
    repository_root: &Path,
    terminal_request: &TerminalRouteRequest,
    cleanup_request: &CleanupRouteRequest,
) -> Result<(), TerminalFinding> {
    let Some(path) = cleanup_request.terminal_receipt_path.as_deref() else {
        return Err(finding(
            "missing_terminal_receipt",
            "cleanup requires a repo-contained terminal closeout receipt file",
        ));
    };
    let candidate = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        repository_root.join(path)
    };
    let canonical = candidate.canonicalize().map_err(|_| {
        finding(
            "terminal_receipt_missing",
            "declared terminal closeout receipt file is missing",
        )
    })?;
    if !is_repo_or_git_receipt_path(repository_root, &canonical) {
        return Err(finding(
            "receipt_path_escapes_repository",
            "terminal closeout receipt must canonicalize beneath the repository root or resolved Git receipt directory",
        ));
    }
    if !is_durable_terminal_receipt_path(repository_root, &canonical) {
        return Err(finding(
            "receipt_path_not_durable",
            "terminal closeout receipt must live under .csdlc/evidence or .git/csdlc-v3",
        ));
    }
    let bytes = fs::read(&canonical).map_err(|error| {
        finding(
            "terminal_receipt_unreadable",
            &format!("terminal closeout receipt could not be read: {error}"),
        )
    })?;
    let digest = blake3::hash(&bytes).to_hex().to_string();
    if cleanup_request.terminal_receipt_digest.as_deref() != Some(digest.as_str()) {
        return Err(finding(
            "terminal_receipt_digest_mismatch",
            "terminal closeout receipt digest must match the loaded receipt bytes",
        ));
    }
    let receipt: DurableTerminalReceipt = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "terminal_receipt_invalid",
            "terminal closeout receipt must be valid typed JSON",
        )
    })?;
    if receipt.schema != "csdlc.v3.terminal_receipt.v1"
        || receipt.repository != terminal_request.repository
        || receipt.issue != terminal_request.issue
        || Some(receipt.pull_request) != terminal_request.pull_request
        || Some(receipt.head_sha.as_str()) != terminal_request.expected_head_sha.as_deref()
        || receipt.disposition != "closed_out"
    {
        return Err(finding(
            "terminal_receipt_mismatch",
            "terminal closeout receipt must match repository, issue, PR, head, and closed-out disposition",
        ));
    }
    Ok(())
}

fn is_durable_terminal_receipt_path(repository_root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(repository_root.join(".csdlc/evidence"))
        || git_control_dir(repository_root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

fn is_repo_or_git_receipt_path(repository_root: &Path, canonical: &Path) -> bool {
    canonical.starts_with(repository_root)
        || git_control_dir(repository_root)
            .map(|git_dir| canonical.starts_with(git_dir.join("csdlc-v3")))
            .unwrap_or(false)
}

fn git_control_dir(repository_root: &Path) -> Option<PathBuf> {
    let dot_git = repository_root.join(".git");
    if dot_git.is_dir() {
        return dot_git.canonicalize().ok();
    }
    let contents = fs::read_to_string(&dot_git).ok()?;
    let gitdir = contents.strip_prefix("gitdir:")?.trim();
    let path = Path::new(gitdir);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        repository_root.join(path)
    };
    let git_dir = path.canonicalize().ok()?;
    if let Some(common_dir) = git_common_dir(&git_dir) {
        return Some(common_dir);
    }
    Some(git_dir)
}

fn git_common_dir(git_dir: &Path) -> Option<PathBuf> {
    let contents = fs::read_to_string(git_dir.join("commondir")).ok()?;
    let path = Path::new(contents.trim());
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        git_dir.join(path)
    };
    path.canonicalize().ok()
}

fn decide_cutover(
    request: &CutoverDecisionRequest,
    cutover_comments: Option<&[GithubIssueComment]>,
) -> Result<CutoverDecision, Vec<TerminalFinding>> {
    let mut findings = Vec::new();
    for (code, label, value) in [
        ("missing_operator", "operator", &request.operator),
        ("missing_operator_approval", "approval", &request.approval),
        (
            "missing_rollback_evidence",
            "rollback evidence",
            &request.rollback_evidence,
        ),
        (
            "missing_undo_boundary",
            "undo boundary",
            &request.undo_boundary,
        ),
    ] {
        if value.trim().is_empty() {
            findings.push(finding(
                code,
                &format!("cutover decision requires explicit {label}"),
            ));
        }
    }
    if request.operation == CutoverOperation::Apply
        && request.selected_binary_provenance.trim().is_empty()
    {
        findings.push(finding(
            "missing_binary_provenance",
            "cutover decision requires explicit selected binary provenance",
        ));
    }
    if !request.execute && !request.approval.contains("#505") {
        findings.push(finding(
            "missing_505_approval",
            "cutover approval must explicitly cite #505",
        ));
    }
    if !request.undo_boundary.to_lowercase().contains("fail-closed") {
        findings.push(finding(
            "missing_fail_closed_undo",
            "cutover undo boundary must be fail-closed",
        ));
    }
    if !findings.is_empty() {
        return Err(findings);
    }
    if !request.execute {
        return Ok(CutoverDecision {
            operation: request.operation,
            approved_by: request.operator.clone(),
            selected_binary_provenance: request.selected_binary_provenance.clone(),
            rollback_evidence: request.rollback_evidence.clone(),
            undo_boundary: request.undo_boundary.clone(),
            executes_cutover: false,
            authority_selector_path: None,
            install_destination_path: None,
            rollback_receipt_path: None,
        });
    }

    if request.operation == CutoverOperation::Apply && cutover_comments.is_none() {
        findings.push(finding(
            "authenticated_cutover_observation_required",
            "executing authority cutover requires authenticated GitHub review and operator approval readback",
        ));
        return Err(findings);
    }

    let execution = match execute_cutover(request, cutover_comments) {
        Ok(execution) => execution,
        Err(finding) => {
            findings.push(finding);
            return Err(findings);
        }
    };
    Ok(CutoverDecision {
        operation: request.operation,
        approved_by: request.operator.clone(),
        selected_binary_provenance: request.selected_binary_provenance.clone(),
        rollback_evidence: request.rollback_evidence.clone(),
        undo_boundary: request.undo_boundary.clone(),
        executes_cutover: request.operation == CutoverOperation::Apply,
        authority_selector_path: Some(execution.authority_selector_path),
        install_destination_path: Some(execution.install_destination_path),
        rollback_receipt_path: Some(execution.rollback_receipt_path),
    })
}

struct CutoverExecution {
    authority_selector_path: PathBuf,
    install_destination_path: PathBuf,
    rollback_receipt_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CutoverPhase {
    Prepared,
    BinaryInstalled,
    Committed,
    RolledBack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CutoverReceipt {
    schema: String,
    authority_issue: u64,
    phase: CutoverPhase,
    selected_revision: String,
    selected_binary_digest: String,
    readiness_evidence_path: PathBuf,
    readiness_evidence_digest: String,
    approval_evidence_path: PathBuf,
    approval_evidence_digest: String,
    canonical_selector: PathBuf,
    prior_selector: Vec<u8>,
    prior_selector_digest: String,
    cutover_selector_digest: String,
    approved_by: String,
}

#[derive(Debug, Deserialize)]
struct AuthorityReadinessEvidence {
    schema: String,
    authority_issue: u64,
    selected_binary_digest: String,
    selected_revision: String,
    route_proofs: BTreeMap<String, ImmutableProofRef>,
    canary_proof: ImmutableProofRef,
    review_proof: ImmutableProofRef,
    approval_proof: ImmutableProofRef,
    rollback_proof: ImmutableProofRef,
}

#[derive(Debug, Deserialize)]
struct ImmutableProofRef {
    path: PathBuf,
    digest: String,
    revision: String,
}

#[derive(Debug, Deserialize)]
struct V2IssueReviewRecord {
    schema: String,
    issue: u64,
    repository: String,
    phase: String,
    generation: u64,
    digest: String,
    review_assignment: Option<V2ReviewAssignment>,
    review: Option<V2ReviewEvidence>,
}

#[derive(Debug, Deserialize)]
struct V2ReviewAssignment {
    reviewer: String,
    assigned_by: String,
    revision: String,
    scope: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct V2ReviewEvidence {
    reviewer: String,
    scope: Vec<String>,
    reviewed_revision: String,
    findings: Vec<V2ReviewFinding>,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct V2ReviewFinding {
    actionable: bool,
    in_scope: bool,
    disposition: String,
}

#[derive(Debug, Deserialize)]
struct ProofCommandReceipt {
    schema: String,
    issue: u64,
    repository: String,
    manifest_id: String,
    lane: String,
    deterministic: bool,
    source_evidence_ref: PathBuf,
    source_evidence_digest: String,
    normalization: crate::commands::proof::ShadowNormalizationContract,
    command: crate::commands::proof::ShadowCommandSpec,
    request_evidence: ProofRequestEvidence,
    binary: ProofBinaryEvidence,
    exit: ProofExitEvidence,
    normalized_output: serde_json::Value,
    side_effect_boundary: Vec<ProofBoundaryEvidence>,
    provider_side_effects: bool,
}

#[derive(Debug, Deserialize)]
struct ProofRequestEvidence {
    #[serde(rename = "ref")]
    reference: PathBuf,
    digest: String,
}

#[derive(Debug, Deserialize)]
struct ProofBinaryEvidence {
    identity: PathBuf,
    digest: String,
}

#[derive(Debug, Deserialize)]
struct ProofExitEvidence {
    code: Option<i32>,
    success: bool,
}

#[derive(Debug, Deserialize)]
struct ProofBoundaryEvidence {
    changed: bool,
}

#[derive(Debug, Deserialize)]
struct AuthorityCutoverApproval {
    schema: String,
    authority_issue: u64,
    repository: String,
    decision: String,
    exact_head: String,
    selected_binary_digest: String,
    rollback_evidence_digest: String,
    review_record_digest: String,
    reviewer_github_login: String,
    review_comment_id: u64,
    approval_comment_id: u64,
    approved_by: String,
}

#[derive(Debug, Deserialize)]
struct GithubReviewAttestation {
    schema: String,
    repository: String,
    issue: u64,
    implementer: String,
    reviewer_session: String,
    exact_head: String,
    review_record_digest: String,
    result: String,
}

#[derive(Debug, Deserialize)]
struct GithubApprovalAttestation {
    schema: String,
    repository: String,
    issue: u64,
    exact_head: String,
    selected_binary_digest: String,
    rollback_evidence_digest: String,
    review_record_digest: String,
    review_comment_id: u64,
    decision: String,
}

#[derive(Debug, Deserialize)]
struct GithubIssueComment {
    id: u64,
    body: String,
    user: GithubCommentAuthor,
}

#[derive(Debug, Deserialize)]
struct GithubCommentAuthor {
    login: String,
}

struct VerifiedReviewAuthority {
    record_digest: String,
    assigned_by: String,
    reviewer_session: String,
}

struct CutoverApprovalEvidence<'a> {
    revision: &'a str,
    selected_binary_digest: &'a str,
    approval_proof: &'a ImmutableProofRef,
    rollback_proof: &'a ImmutableProofRef,
    comments: &'a [GithubIssueComment],
}

struct VerifiedCutoverReadiness {
    selected_revision: String,
    evidence_path: PathBuf,
    evidence_digest: String,
    approval_evidence_path: PathBuf,
    approval_evidence_digest: String,
}

fn execute_cutover(
    request: &CutoverDecisionRequest,
    cutover_comments: Option<&[GithubIssueComment]>,
) -> Result<CutoverExecution, TerminalFinding> {
    execute_cutover_platform(request, cutover_comments)
}

#[cfg(not(unix))]
fn execute_cutover_platform(
    _request: &CutoverDecisionRequest,
    _cutover_comments: Option<&[GithubIssueComment]>,
) -> Result<CutoverExecution, TerminalFinding> {
    Err(finding(
        "cutover_unsupported_platform",
        "authority cutover and rollback are fail-closed on unsupported platforms",
    ))
}

#[cfg(unix)]
fn execute_cutover_platform(
    request: &CutoverDecisionRequest,
    cutover_comments: Option<&[GithubIssueComment]>,
) -> Result<CutoverExecution, TerminalFinding> {
    let repository_root = request
        .repository_root
        .as_ref()
        .ok_or_else(|| {
            finding(
                "missing_repository_root",
                "executing cutover requires an explicit repository root",
            )
        })?
        .canonicalize()
        .map_err(|error| {
            finding(
                "repository_root_unavailable",
                &format!("repository root must be an existing canonical directory: {error}"),
            )
        })?;
    if !repository_root.join(".git").exists() {
        return Err(finding(
            "repository_root_not_git_checkout",
            "executing cutover requires a repository checkout root",
        ));
    }
    let selector = repo_output_path(
        &repository_root,
        request.authority_selector_path.as_ref(),
        "missing_authority_selector_path",
        "executing cutover requires an authority selector output path",
        "authority_selector",
    )?;
    let destination = repo_output_path(
        &repository_root,
        request.install_destination_path.as_ref(),
        "missing_install_destination_path",
        "executing cutover requires a stable install destination path",
        "install_destination",
    )?;
    let receipt = repo_output_path(
        &repository_root,
        request.rollback_receipt_path.as_ref(),
        "missing_rollback_receipt_path",
        "executing cutover requires a rollback receipt output path",
        "rollback_receipt",
    )?;
    if destination != repository_root.join(".adl/bin/csdlc") {
        return Err(finding(
            "install_destination_not_stable_csdlc_binary",
            "cutover installs only the stable repo-local .adl/bin/csdlc binary",
        ));
    }
    if selector != repository_root.join("csdlc-v2/operator/generation-selector.json") {
        return Err(finding(
            "authority_selector_not_canonical",
            "cutover writes only csdlc-v2/operator/generation-selector.json",
        ));
    }
    if !receipt.starts_with(repository_root.join(".csdlc/evidence/505")) {
        return Err(finding(
            "rollback_receipt_not_issue_evidence",
            "cutover rollback receipt must live under .csdlc/evidence/505",
        ));
    }
    if request.operation == CutoverOperation::Rollback {
        return execute_rollback(request, selector, destination, receipt);
    }

    let selected_binary = repo_existing_file(
        &repository_root,
        request.selected_binary_path.as_ref(),
        "missing_selected_binary_path",
        "executing cutover requires an explicit selected binary path",
        "selected_binary",
    )?;
    let binary_bytes = fs::read(&selected_binary).map_err(|error| {
        finding(
            "selected_binary_unreadable",
            &format!("selected binary must be readable: {error}"),
        )
    })?;
    let binary_digest = blake3::hash(&binary_bytes).to_hex().to_string();
    let readiness = verify_cutover_readiness(
        request,
        &repository_root,
        &binary_digest,
        cutover_comments.ok_or_else(|| {
            finding(
                "authenticated_cutover_observation_required",
                "executing authority cutover requires authenticated GitHub evidence",
            )
        })?,
    )?;
    let permissions = fs::metadata(&selected_binary)
        .map_err(|error| {
            finding(
                "selected_binary_metadata_unavailable",
                &format!("selected binary metadata must be available: {error}"),
            )
        })?
        .permissions();

    let mut journal = if receipt.exists() {
        read_cutover_receipt(&receipt)?
    } else {
        if destination.exists() {
            return Err(finding(
                "install_destination_preexists",
                "initial cutover requires the stable v3 binary destination to be absent",
            ));
        }
        let prior_selector = fs::read(&selector).map_err(|error| {
            finding(
                "canonical_selector_unreadable",
                &format!("canonical generation selector must be readable: {error}"),
            )
        })?;
        let mut selector_value: serde_json::Value = serde_json::from_slice(&prior_selector)
            .map_err(|_| {
                finding(
                    "canonical_selector_invalid",
                    "canonical generation selector must be typed JSON",
                )
            })?;
        if selector_value["schema"] != "csdlc.generation_selector.v1"
            || selector_value["default_generation"] == "v3"
        {
            return Err(finding(
                "canonical_selector_not_pre_cutover",
                "initial cutover requires the canonical v1-schema selector to be pre-v3",
            ));
        }
        apply_v3_authority_fields(
            &mut selector_value,
            &readiness.selected_revision,
            &readiness.evidence_digest,
            &readiness.approval_evidence_digest,
        );
        let selector_bytes = serde_json::to_vec_pretty(&selector_value)
            .map_err(|error| finding("selector_serialize_failed", &error.to_string()))?;
        let journal = CutoverReceipt {
            schema: "csdlc.v3.cutover_receipt.v2".into(),
            authority_issue: 505,
            phase: CutoverPhase::Prepared,
            selected_revision: readiness.selected_revision.clone(),
            selected_binary_digest: binary_digest.clone(),
            readiness_evidence_path: readiness.evidence_path.clone(),
            readiness_evidence_digest: readiness.evidence_digest.clone(),
            approval_evidence_path: readiness.approval_evidence_path.clone(),
            approval_evidence_digest: readiness.approval_evidence_digest.clone(),
            canonical_selector: PathBuf::from("csdlc-v2/operator/generation-selector.json"),
            prior_selector_digest: blake3::hash(&prior_selector).to_hex().to_string(),
            cutover_selector_digest: blake3::hash(&selector_bytes).to_hex().to_string(),
            prior_selector,
            approved_by: request.operator.clone(),
        };
        write_cutover_receipt(&receipt, &journal)?;
        journal
    };
    validate_cutover_journal(request, &journal, &binary_digest, &readiness)?;
    if journal.phase == CutoverPhase::RolledBack {
        return Err(finding(
            "cutover_already_rolled_back",
            "a rolled-back cutover receipt cannot be reused for a new authority transition",
        ));
    }
    let selector_bytes = selector_bytes_from_journal(&journal)?;
    let selector_digest = digest_file(&selector)?;
    let binary_state = optional_regular_file_digest(&destination)?;
    if selector_digest != journal.prior_selector_digest
        && selector_digest != journal.cutover_selector_digest
    {
        return Err(finding(
            "cutover_selector_stale_digest",
            "canonical selector differs from both the retained preimage and intended v3 selector",
        ));
    }
    if binary_state
        .as_deref()
        .is_some_and(|digest| digest != journal.selected_binary_digest)
    {
        return Err(finding(
            "cutover_binary_stale_digest",
            "stable v3 binary differs from the selected binary digest",
        ));
    }
    if selector_digest == journal.cutover_selector_digest && binary_state.is_none() {
        return Err(finding(
            "cutover_split_authority",
            "canonical selector activates v3 while the selected stable binary is absent",
        ));
    }
    if binary_state.is_none() {
        write_staged(&destination, &binary_bytes)?;
        journal.phase = CutoverPhase::BinaryInstalled;
        write_cutover_receipt(&receipt, &journal)?;
    }
    fs::set_permissions(&destination, permissions).map_err(|error| {
        finding(
            "installed_binary_permissions_failed",
            &format!("could not apply selected binary permissions: {error}"),
        )
    })?;
    if selector_digest == journal.prior_selector_digest {
        write_staged(&selector, &selector_bytes)?;
    }
    journal.phase = CutoverPhase::Committed;
    write_cutover_receipt(&receipt, &journal)?;
    Ok(CutoverExecution {
        authority_selector_path: selector,
        install_destination_path: destination,
        rollback_receipt_path: receipt,
    })
}

fn execute_rollback(
    request: &CutoverDecisionRequest,
    selector: PathBuf,
    destination: PathBuf,
    receipt: PathBuf,
) -> Result<CutoverExecution, TerminalFinding> {
    if !receipt.exists() {
        return Err(finding(
            "rollback_receipt_missing",
            "post-success rollback requires the durable cutover receipt",
        ));
    }
    let mut journal = read_cutover_receipt(&receipt)?;
    if journal.schema != "csdlc.v3.cutover_receipt.v2"
        || journal.authority_issue != 505
        || journal.canonical_selector != Path::new("csdlc-v2/operator/generation-selector.json")
        || (!request.selected_binary_provenance.is_empty()
            && request.selected_binary_provenance != format!("git:{}", journal.selected_revision))
    {
        return Err(finding(
            "rollback_receipt_mismatch",
            "rollback request does not match the typed #505 cutover receipt",
        ));
    }
    let selector_digest = digest_file(&selector)?;
    if selector_digest != journal.prior_selector_digest
        && selector_digest != journal.cutover_selector_digest
    {
        return Err(finding(
            "rollback_selector_stale_digest",
            "rollback refuses a canonical selector that differs from both retained states",
        ));
    }
    let binary_state = optional_regular_file_digest(&destination)?;
    if binary_state
        .as_deref()
        .is_some_and(|digest| digest != journal.selected_binary_digest)
    {
        return Err(finding(
            "rollback_binary_stale_digest",
            "rollback refuses to remove a stable binary with an unexpected digest",
        ));
    }
    if selector_digest == journal.cutover_selector_digest {
        write_staged(&selector, &journal.prior_selector)?;
    }
    if destination.exists() {
        fs::remove_file(&destination).map_err(|error| {
            finding(
                "rollback_binary_remove_failed",
                &format!("could not remove the stable v3 binary: {error}"),
            )
        })?;
    }
    journal.phase = CutoverPhase::RolledBack;
    write_cutover_receipt(&receipt, &journal)?;
    if digest_file(&selector)? != journal.prior_selector_digest || destination.exists() {
        return Err(finding(
            "rollback_reconciliation_failed",
            "rollback did not restore the exact v2 selector preimage and remove the v3 binary",
        ));
    }
    Ok(CutoverExecution {
        authority_selector_path: selector,
        install_destination_path: destination,
        rollback_receipt_path: receipt,
    })
}

fn verify_cutover_readiness(
    request: &CutoverDecisionRequest,
    repository_root: &Path,
    selected_binary_digest: &str,
    cutover_comments: &[GithubIssueComment],
) -> Result<VerifiedCutoverReadiness, TerminalFinding> {
    const REQUIRED_ROUTES: [&str; 21] = [
        "bind",
        "clean",
        "cutover",
        "doctor",
        "edit",
        "eligibility",
        "finish",
        "github",
        "github-issue",
        "github-pr",
        "install",
        "issue",
        "pr-state",
        "proof",
        "publish",
        "review",
        "schedule",
        "shadow",
        "shepherd",
        "soak",
        "validate",
    ];
    if request.readiness_evidence_path.as_deref()
        != Some(Path::new(".csdlc/evidence/505/authority-readiness.json"))
    {
        return Err(finding(
            "readiness_evidence_path_not_canonical",
            "authority readiness must use the canonical #505 evidence path",
        ));
    }
    let path = repo_existing_file(
        repository_root,
        request.readiness_evidence_path.as_ref(),
        "missing_readiness_evidence_path",
        "executing cutover requires exact operational-readiness evidence",
        "readiness_evidence",
    )?;
    let bytes = fs::read(&path).map_err(|error| {
        finding(
            "readiness_evidence_unreadable",
            &format!("could not read cutover readiness evidence: {error}"),
        )
    })?;
    let observed_digest = blake3::hash(&bytes).to_hex().to_string();
    if request.readiness_evidence_digest.as_deref() != Some(observed_digest.as_str()) {
        return Err(finding(
            "readiness_evidence_digest_mismatch",
            "cutover readiness evidence does not match its expected digest",
        ));
    }
    let evidence: AuthorityReadinessEvidence = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "readiness_evidence_invalid",
            "cutover readiness evidence must be typed JSON",
        )
    })?;
    if evidence.schema != "csdlc.v3.authority_readiness.v1"
        || evidence.authority_issue != 505
        || evidence.selected_binary_digest != selected_binary_digest
        || !is_immutable_revision(&evidence.selected_revision)
        || request.selected_binary_provenance != format!("git:{}", evidence.selected_revision)
        || evidence.route_proofs.len() != REQUIRED_ROUTES.len()
        || REQUIRED_ROUTES
            .iter()
            .any(|route| !evidence.route_proofs.contains_key(*route))
    {
        return Err(finding(
            "operational_readiness_not_proven",
            "cutover requires 21 file-backed route proofs, canary/review proofs, one immutable revision, and the selected binary digest",
        ));
    }
    for proof in evidence.route_proofs.values().chain([
        &evidence.canary_proof,
        &evidence.review_proof,
        &evidence.approval_proof,
        &evidence.rollback_proof,
    ]) {
        verify_immutable_proof(repository_root, proof, &evidence.selected_revision)?;
    }
    for (route, proof) in &evidence.route_proofs {
        verify_route_readiness(
            repository_root,
            route,
            proof,
            &evidence.selected_revision,
            selected_binary_digest,
        )?;
    }
    verify_v3_only_canary(
        repository_root,
        &evidence.canary_proof,
        &evidence.selected_revision,
        selected_binary_digest,
    )?;
    let review_authority = verify_independent_review(
        repository_root,
        &evidence.review_proof,
        &evidence.selected_revision,
        &request.operator,
    )?;
    let (approval_evidence_path, approval_evidence_digest) = verify_cutover_approval(
        request,
        repository_root,
        &review_authority,
        CutoverApprovalEvidence {
            revision: &evidence.selected_revision,
            selected_binary_digest,
            approval_proof: &evidence.approval_proof,
            rollback_proof: &evidence.rollback_proof,
            comments: cutover_comments,
        },
    )?;
    Ok(VerifiedCutoverReadiness {
        selected_revision: evidence.selected_revision,
        evidence_path: request
            .readiness_evidence_path
            .clone()
            .expect("path validated above"),
        evidence_digest: observed_digest,
        approval_evidence_path,
        approval_evidence_digest,
    })
}

fn read_proof_json<T: for<'de> Deserialize<'de>>(
    repository_root: &Path,
    proof: &ImmutableProofRef,
    code: &str,
) -> Result<T, TerminalFinding> {
    let path = repo_existing_file(
        repository_root,
        Some(&proof.path),
        code,
        "cutover proof file is missing",
        "cutover_proof",
    )?;
    serde_json::from_slice(&fs::read(path).map_err(|error| finding(code, &error.to_string()))?)
        .map_err(|_| finding(code, "cutover proof must be typed JSON evidence"))
}

fn verify_route_readiness(
    repository_root: &Path,
    route: &str,
    proof: &ImmutableProofRef,
    revision: &str,
    selected_binary_digest: &str,
) -> Result<(), TerminalFinding> {
    verify_proof_command_receipt(
        repository_root,
        proof,
        revision,
        &format!("route-{route}"),
        "route-readiness",
        selected_binary_digest,
    )
}

fn verify_v3_only_canary(
    repository_root: &Path,
    proof: &ImmutableProofRef,
    revision: &str,
    selected_binary_digest: &str,
) -> Result<(), TerminalFinding> {
    verify_proof_command_receipt(
        repository_root,
        proof,
        revision,
        "v3-only-canary",
        "v3-only-canary",
        selected_binary_digest,
    )
}

fn verify_proof_command_receipt(
    repository_root: &Path,
    proof: &ImmutableProofRef,
    revision: &str,
    manifest_id: &str,
    lane: &str,
    selected_binary_digest: &str,
) -> Result<(), TerminalFinding> {
    let expected_path = PathBuf::from(format!(".csdlc/evidence/505/v3-proof/{manifest_id}.json"));
    let evidence: ProofCommandReceipt =
        read_proof_json(repository_root, proof, "proof_command_receipt_invalid")?;
    if proof.path != expected_path || proof.revision != revision {
        return Err(finding(
            "proof_command_reference_not_exact",
            "proof readiness reference must use the canonical manifest path and exact revision",
        ));
    }
    if evidence.schema != "csdlc.v3.proof_receipt.v2"
        || evidence.issue != 505
        || evidence.repository != "agent-logic/agent-design-language"
    {
        return Err(finding(
            "proof_command_receipt_identity_invalid",
            "proof readiness receipt must use the v2 schema and canonical issue identity",
        ));
    }
    if evidence.manifest_id != manifest_id || evidence.lane != lane || !evidence.deterministic {
        return Err(finding(
            "proof_command_manifest_mismatch",
            &format!(
                "proof readiness expected manifest {manifest_id}/{lane}, observed {}/{} deterministic={}",
                evidence.manifest_id, evidence.lane, evidence.deterministic
            ),
        ));
    }
    if !valid_blake3_digest(&evidence.source_evidence_digest)
        || !evidence
            .source_evidence_ref
            .starts_with(Path::new(".csdlc/evidence/505"))
    {
        return Err(finding(
            "proof_command_source_invalid",
            "proof readiness source must be retained beneath issue #505 evidence with a digest",
        ));
    }
    let route_preview = manifest_id.strip_prefix("route-");
    let command_identity_valid = if let Some(route) = route_preview {
        evidence.normalization
            == crate::commands::proof::ShadowNormalizationContract::RoutePreviewV1
            && evidence.command.argv == [route, "--preview"]
            && evidence.normalized_output["command"] == route
            && evidence.normalized_output["mode"] == "preview"
            && evidence.normalized_output["category"].as_str().is_some()
            && evidence.normalized_output["effect"].as_str().is_some()
            && evidence.normalized_output["authority_required"]
                .as_bool()
                .is_some()
    } else {
        evidence.normalization
            == crate::commands::proof::ShadowNormalizationContract::DoctorIssuePhaseV1
            && evidence.command.argv.first().map(String::as_str) == Some("doctor")
            && evidence.normalized_output["command"] == "doctor"
    };
    if evidence.command.generation != crate::commands::proof::ShadowGeneration::V3
        || evidence.command.provider_side_effects
        || evidence.provider_side_effects
        || !command_identity_valid
    {
        return Err(finding(
            "proof_command_identity_invalid",
            "proof readiness requires the exact credential-cleared v3 route preview or declared doctor command",
        ));
    }
    if Path::new(&evidence.command.request_ref) != evidence.request_evidence.reference
        || Path::new(&evidence.command.binary_ref) != evidence.binary.identity
    {
        return Err(finding(
            "proof_command_provenance_mismatch",
            "proof command argv inputs must match the retained request and binary identities",
        ));
    }
    if evidence.binary.digest != selected_binary_digest {
        return Err(finding(
            "proof_binary_not_selected",
            "proof command must execute the exact selected cutover binary",
        ));
    }
    if evidence.exit.code != Some(0) || !evidence.exit.success {
        return Err(finding(
            "proof_command_not_successful",
            "proof readiness requires an observed successful command exit",
        ));
    }
    if evidence.normalized_output["issue"] != 505 {
        return Err(finding(
            "proof_command_output_mismatch",
            "proof readiness requires typed route output bound to issue #505",
        ));
    }
    if evidence
        .side_effect_boundary
        .iter()
        .any(|boundary| boundary.changed)
    {
        return Err(finding(
            "proof_command_side_effect_detected",
            "proof readiness command changed a declared read-only boundary",
        ));
    }
    let source = repo_existing_file(
        repository_root,
        Some(&evidence.source_evidence_ref),
        "proof_source_missing",
        "proof-command source evidence must be retained",
        "proof_source",
    )?;
    if digest_file(&source)? != evidence.source_evidence_digest {
        return Err(finding(
            "proof_source_digest_mismatch",
            "proof-command source evidence does not match its retained digest",
        ));
    }
    let binary = repo_existing_file(
        repository_root,
        Some(&evidence.binary.identity),
        "proof_binary_missing",
        "proof-command selected binary must remain available",
        "proof_binary",
    )?;
    if digest_file(&binary)? != evidence.binary.digest {
        return Err(finding(
            "proof_binary_digest_mismatch",
            "proof-command binary bytes do not match the selected binary digest",
        ));
    }
    let command_request = repo_existing_file(
        repository_root,
        Some(&evidence.request_evidence.reference),
        "proof_request_missing",
        "proof-command request evidence must remain available",
        "proof_request",
    )?;
    if digest_file(&command_request)? != evidence.request_evidence.digest {
        return Err(finding(
            "proof_request_digest_mismatch",
            "proof-command request bytes do not match the execution receipt",
        ));
    }
    Ok(())
}

fn verify_independent_review(
    repository_root: &Path,
    proof: &ImmutableProofRef,
    revision: &str,
    operator: &str,
) -> Result<VerifiedReviewAuthority, TerminalFinding> {
    if proof.path != Path::new(".csdlc/issues/505/index.json") {
        return Err(finding(
            "review_proof_not_v2_lifecycle_record",
            "cutover review proof must be the canonical typed v2 issue record",
        ));
    }
    let path = repo_existing_file(
        repository_root,
        Some(&proof.path),
        "review_proof_path_missing",
        "cutover requires retained independent review evidence",
        "review_proof",
    )?;
    let bytes =
        fs::read(path).map_err(|error| finding("review_proof_unreadable", &error.to_string()))?;
    let record: V2IssueReviewRecord = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "independent_review_invalid",
            "review proof must be the typed v2 issue record",
        )
    })?;
    let review = record.review.as_ref();
    let assignment = record.review_assignment.as_ref();
    let mut canonical: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "independent_review_invalid",
            "review proof must be canonical typed JSON",
        )
    })?;
    canonical["digest"] = serde_json::Value::String(String::new());
    let canonical_digest = blake3::hash(
        &serde_json::to_vec(&canonical)
            .map_err(|error| finding("independent_review_invalid", &error.to_string()))?,
    )
    .to_hex()
    .to_string();
    let expected_revision = format!(
        "git-blake3:{revision}:{}",
        blake3::hash(revision.as_bytes()).to_hex()
    );
    if record.schema != "csdlc.issue.index.v1"
        || record.issue != 505
        || record.repository != "agent-logic/agent-design-language"
        || !matches!(
            record.phase.as_str(),
            "reviewed" | "published" | "merge_ready"
        )
        || record.generation == 0
        || record.digest != canonical_digest
        || assignment.is_none()
        || review.is_none()
        || assignment.is_some_and(|assignment| {
            !valid_fresh_session_identity(&assignment.reviewer)
                || assignment.assigned_by != operator
                || assignment.scope.is_empty()
                || assignment.revision != expected_revision
        })
        || review.is_some_and(|review| {
            review.reviewer.trim().is_empty()
                || review.reviewer == operator
                || review.scope.is_empty()
                || review.reviewed_revision != expected_revision
                || !review.completed
                || review.findings.iter().any(|finding| {
                    finding.actionable
                        && finding.in_scope
                        && !matches!(finding.disposition.as_str(), "fixed" | "accepted_risk")
                })
        })
        || assignment.zip(review).is_some_and(|(assignment, review)| {
            assignment.reviewer != review.reviewer
                || assignment.scope != review.scope
                || assignment.revision != review.reviewed_revision
        })
    {
        return Err(finding(
            "independent_review_not_proven",
            "typed v2 lifecycle state must retain a completed independent exact-head review with resolved findings",
        ));
    }
    let assignment = assignment.expect("assignment verified above");
    let review = review.expect("review verified above");
    Ok(VerifiedReviewAuthority {
        record_digest: record.digest,
        assigned_by: assignment.assigned_by.clone(),
        reviewer_session: review.reviewer.clone(),
    })
}

fn valid_fresh_session_identity(value: &str) -> bool {
    let Some(id) = value.strip_prefix("fresh-session:") else {
        return false;
    };
    id.len() == 36
        && id.chars().enumerate().all(|(index, character)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                character == '-'
            } else {
                character.is_ascii_hexdigit()
            }
        })
}

fn verify_cutover_approval(
    request: &CutoverDecisionRequest,
    repository_root: &Path,
    review_authority: &VerifiedReviewAuthority,
    evidence: CutoverApprovalEvidence<'_>,
) -> Result<(PathBuf, String), TerminalFinding> {
    let CutoverApprovalEvidence {
        revision,
        selected_binary_digest,
        approval_proof,
        rollback_proof,
        comments: cutover_comments,
    } = evidence;
    let approval_path = PathBuf::from(request.approval.trim());
    if approval_path != Path::new(".csdlc/evidence/505/cutover-approval.json")
        || approval_path != approval_proof.path
        || rollback_proof.path.as_path() != Path::new(request.rollback_evidence.trim())
    {
        return Err(finding(
            "cutover_evidence_path_mismatch",
            "approval and rollback paths must match the digest-bound readiness references",
        ));
    }
    let path = repo_existing_file(
        repository_root,
        Some(&approval_path),
        "cutover_approval_path_missing",
        "executing cutover requires retained typed approval evidence",
        "cutover_approval",
    )?;
    let bytes = fs::read(&path)
        .map_err(|error| finding("cutover_approval_unreadable", &error.to_string()))?;
    let approval: AuthorityCutoverApproval = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "cutover_approval_invalid",
            "cutover approval must be typed JSON evidence",
        )
    })?;
    if approval.schema != "csdlc.v3.cutover_approval.v1"
        || approval.authority_issue != 505
        || approval.repository != "agent-logic/agent-design-language"
        || approval.decision != "approved"
        || approval.exact_head != revision
        || approval.selected_binary_digest != selected_binary_digest
        || approval.rollback_evidence_digest != rollback_proof.digest
        || approval.review_record_digest != review_authority.record_digest
        || approval.approved_by != review_authority.assigned_by
        || request.operator != review_authority.assigned_by
        || approval.reviewer_github_login.trim().is_empty()
        || approval.review_comment_id == approval.approval_comment_id
    {
        return Err(finding(
            "cutover_approval_not_exact",
            "typed #505 approval must bind repository, exact head, selected binary, and operator",
        ));
    }
    let review_comment = cutover_comments
        .iter()
        .find(|comment| comment.id == approval.review_comment_id)
        .ok_or_else(|| {
            finding(
                "github_review_attestation_missing",
                "authenticated GitHub review attestation comment is missing",
            )
        })?;
    let approval_comment = cutover_comments
        .iter()
        .find(|comment| comment.id == approval.approval_comment_id)
        .ok_or_else(|| {
            finding(
                "github_approval_attestation_missing",
                "authenticated GitHub operator approval comment is missing",
            )
        })?;
    let review_attestation: GithubReviewAttestation = serde_json::from_str(&review_comment.body)
        .map_err(|_| {
            finding(
                "github_review_attestation_invalid",
                "GitHub review attestation comment must be typed JSON",
            )
        })?;
    let approval_attestation: GithubApprovalAttestation =
        serde_json::from_str(&approval_comment.body).map_err(|_| {
            finding(
                "github_approval_attestation_invalid",
                "GitHub approval attestation comment must be typed JSON",
            )
        })?;
    if review_comment.user.login != approval.reviewer_github_login
        || review_comment.user.login == approval_comment.user.login
        || review_attestation.schema != "csdlc.v3.github_review_attestation.v1"
        || review_attestation.repository != "agent-logic/agent-design-language"
        || review_attestation.issue != 505
        || review_attestation.implementer != request.operator
        || review_attestation.reviewer_session != review_authority.reviewer_session
        || review_attestation.exact_head != revision
        || review_attestation.review_record_digest != review_authority.record_digest
        || review_attestation.result != "pass"
        || approval_comment.user.login != approval.approved_by
        || approval_attestation.schema != "csdlc.v3.github_cutover_approval.v1"
        || approval_attestation.repository != "agent-logic/agent-design-language"
        || approval_attestation.issue != 505
        || approval_attestation.exact_head != revision
        || approval_attestation.selected_binary_digest != selected_binary_digest
        || approval_attestation.rollback_evidence_digest != rollback_proof.digest
        || approval_attestation.review_record_digest != review_authority.record_digest
        || approval_attestation.review_comment_id != approval.review_comment_id
        || approval_attestation.decision != "approved"
    {
        return Err(finding(
            "github_cutover_authority_not_exact",
            "authenticated GitHub attestations must have distinct reviewer/operator authors and bind the exact review, head, binary, and rollback proof",
        ));
    }
    verify_proof_command_receipt(
        repository_root,
        rollback_proof,
        revision,
        "rollback-readiness",
        "rollback-readiness",
        selected_binary_digest,
    )?;
    Ok((approval_path, approval_proof.digest.clone()))
}

fn verify_immutable_proof(
    repository_root: &Path,
    proof: &ImmutableProofRef,
    revision: &str,
) -> Result<(), TerminalFinding> {
    if proof.revision != revision || !is_immutable_revision(&proof.revision) {
        return Err(finding(
            "proof_revision_mismatch",
            "every readiness proof must bind the selected immutable revision",
        ));
    }
    let path = repo_existing_file(
        repository_root,
        Some(&proof.path),
        "proof_path_missing",
        "readiness proof requires an existing repository-contained file",
        "readiness_proof",
    )?;
    if digest_file(&path)? != proof.digest {
        return Err(finding(
            "proof_digest_mismatch",
            "readiness proof bytes do not match the retained digest",
        ));
    }
    Ok(())
}

fn is_immutable_revision(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn valid_blake3_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn validate_cutover_journal(
    request: &CutoverDecisionRequest,
    journal: &CutoverReceipt,
    binary_digest: &str,
    readiness: &VerifiedCutoverReadiness,
) -> Result<(), TerminalFinding> {
    if journal.schema != "csdlc.v3.cutover_receipt.v2"
        || journal.authority_issue != 505
        || journal.selected_binary_digest != binary_digest
        || journal.selected_revision != readiness.selected_revision
        || journal.readiness_evidence_path != readiness.evidence_path
        || journal.readiness_evidence_digest != readiness.evidence_digest
        || journal.approval_evidence_path != readiness.approval_evidence_path
        || journal.approval_evidence_digest != readiness.approval_evidence_digest
        || request.selected_binary_provenance != format!("git:{}", journal.selected_revision)
        || blake3::hash(&journal.prior_selector).to_hex().to_string()
            != journal.prior_selector_digest
    {
        return Err(finding(
            "cutover_receipt_mismatch",
            "existing cutover journal does not match the selected binary, revision, readiness proof, or selector preimage",
        ));
    }
    Ok(())
}

fn selector_bytes_from_journal(journal: &CutoverReceipt) -> Result<Vec<u8>, TerminalFinding> {
    let mut value: serde_json::Value =
        serde_json::from_slice(&journal.prior_selector).map_err(|_| {
            finding(
                "cutover_receipt_invalid",
                "retained selector preimage is not typed JSON",
            )
        })?;
    apply_v3_authority_fields(
        &mut value,
        &journal.selected_revision,
        &journal.readiness_evidence_digest,
        &journal.approval_evidence_digest,
    );
    let bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| finding("selector_serialize_failed", &error.to_string()))?;
    if blake3::hash(&bytes).to_hex().to_string() != journal.cutover_selector_digest {
        return Err(finding(
            "cutover_receipt_invalid",
            "retained selector preimage does not reconstruct the journaled v3 selector",
        ));
    }
    Ok(bytes)
}

fn apply_v3_authority_fields(
    value: &mut serde_json::Value,
    exact_review_sha: &str,
    readiness_evidence_digest: &str,
    approval_evidence_digest: &str,
) {
    value["schema"] = serde_json::Value::String("csdlc.generation_selector.v2".into());
    value["default_generation"] = serde_json::Value::String("v3".into());
    value["operational_authority"] = serde_json::Value::String("csdlc-v3".into());
    value["authority_issue"] = serde_json::Value::from(505);
    value["exact_review_sha"] = serde_json::Value::String(exact_review_sha.into());
    value["readiness_evidence_digest"] =
        serde_json::Value::String(readiness_evidence_digest.into());
    value["approval_evidence_digest"] = serde_json::Value::String(approval_evidence_digest.into());
}

fn read_cutover_receipt(path: &Path) -> Result<CutoverReceipt, TerminalFinding> {
    serde_json::from_slice(&fs::read(path).map_err(|error| {
        finding(
            "cutover_receipt_unreadable",
            &format!("cutover receipt is unreadable: {error}"),
        )
    })?)
    .map_err(|_| {
        finding(
            "cutover_receipt_invalid",
            "cutover receipt must be typed JSON",
        )
    })
}

fn write_cutover_receipt(path: &Path, receipt: &CutoverReceipt) -> Result<(), TerminalFinding> {
    let bytes = serde_json::to_vec_pretty(receipt)
        .map_err(|error| finding("receipt_serialize_failed", &error.to_string()))?;
    write_staged(path, &bytes)
}

fn digest_file(path: &Path) -> Result<String, TerminalFinding> {
    fs::read(path)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| {
            finding(
                "cutover_file_unreadable",
                &format!("cutover file is unreadable: {error}"),
            )
        })
}

fn optional_regular_file_digest(path: &Path) -> Result<Option<String>, TerminalFinding> {
    match path.symlink_metadata() {
        Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
            digest_file(path).map(Some)
        }
        Ok(_) => Err(finding(
            "cutover_output_not_regular_file",
            "cutover output target must be a regular file",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(finding(
            "cutover_file_unreadable",
            &format!("cutover file metadata is unavailable: {error}"),
        )),
    }
}

fn repo_existing_file(
    repository_root: &Path,
    path: Option<&PathBuf>,
    missing_code: &str,
    missing_message: &str,
    label: &str,
) -> Result<PathBuf, TerminalFinding> {
    let path = path.ok_or_else(|| finding(missing_code, missing_message))?;
    let joined = checked_repo_relative(repository_root, path, label)?;
    let canonical = fs::canonicalize(&joined).map_err(|error| {
        finding(
            "cutover_path_unavailable",
            &format!("{label} path must exist and be canonical: {error}"),
        )
    })?;
    if !canonical.starts_with(repository_root) || !canonical.is_file() {
        return Err(finding(
            "cutover_path_escapes_repository",
            &format!("{label} must be a file inside the repository"),
        ));
    }
    Ok(canonical)
}

fn repo_output_path(
    repository_root: &Path,
    path: Option<&PathBuf>,
    missing_code: &str,
    missing_message: &str,
    label: &str,
) -> Result<PathBuf, TerminalFinding> {
    let path = path.ok_or_else(|| finding(missing_code, missing_message))?;
    let output = checked_repo_relative(repository_root, path, label)?;
    ensure_output_parent_inside_repo(repository_root, &output, label)?;
    Ok(output)
}

fn checked_repo_relative(
    repository_root: &Path,
    path: &Path,
    label: &str,
) -> Result<PathBuf, TerminalFinding> {
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(finding(
            "cutover_path_not_repo_relative",
            &format!("{label} must be a normalized repository-relative path"),
        ));
    }
    Ok(repository_root.join(path))
}

fn write_staged(path: &Path, bytes: &[u8]) -> Result<(), TerminalFinding> {
    let parent = path.parent().ok_or_else(|| {
        finding(
            "cutover_output_without_parent",
            "cutover output path must have a parent directory",
        )
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        finding(
            "cutover_output_parent_failed",
            &format!("could not create cutover output parent: {error}"),
        )
    })?;
    if path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || metadata.is_dir())
    {
        return Err(finding(
            "cutover_output_not_regular_file",
            "cutover output target must not be a symlink or directory",
        ));
    }
    let stage = parent.join(format!(
        ".{}.stage",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("cutover")
    ));
    if stage.exists() {
        let staged = fs::read(&stage).map_err(|error| {
            finding(
                "cutover_stage_unreadable",
                &format!("staged cutover file is unreadable: {error}"),
            )
        })?;
        if staged != bytes {
            return Err(finding(
                "cutover_stage_conflict",
                "staged cutover bytes differ from the deterministic retry",
            ));
        }
        return fs::rename(&stage, path).map_err(|error| {
            finding(
                "cutover_atomic_replace_failed",
                &format!("could not reconcile staged cutover output: {error}"),
            )
        });
    }
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&stage)
            .map_err(|error| {
                finding(
                    "cutover_stage_create_failed",
                    &format!("could not create cutover stage file: {error}"),
                )
            })?;
        file.write_all(bytes).map_err(|error| {
            finding(
                "cutover_stage_write_failed",
                &format!("could not write cutover stage file: {error}"),
            )
        })?;
        file.sync_all().map_err(|error| {
            finding(
                "cutover_stage_sync_failed",
                &format!("could not sync cutover stage file: {error}"),
            )
        })?;
    }
    fs::rename(&stage, path).map_err(|error| {
        let _ = fs::remove_file(&stage);
        finding(
            "cutover_atomic_replace_failed",
            &format!("could not atomically replace cutover output: {error}"),
        )
    })
}

fn ensure_output_parent_inside_repo(
    repository_root: &Path,
    path: &Path,
    label: &str,
) -> Result<(), TerminalFinding> {
    let parent = path
        .parent()
        .ok_or_else(|| finding("cutover_output_without_parent", "output path has no parent"))?;
    let existing = canonical_existing_ancestor(parent)?;
    if !existing.starts_with(repository_root) {
        return Err(finding(
            "cutover_path_escapes_repository",
            &format!("{label} parent escapes the repository"),
        ));
    }
    Ok(())
}

fn canonical_dir(path: &Path, label: &str) -> Result<PathBuf, TerminalFinding> {
    let canonical = fs::canonicalize(path).map_err(|error| {
        finding(
            "non_canonical_path",
            &format!("{label} must be an existing canonical directory: {error}"),
        )
    })?;
    if !canonical.is_dir() {
        return Err(finding(
            "non_directory_path",
            &format!("{label} must be a directory"),
        ));
    }
    Ok(canonical)
}

fn git_worktree_paths(repository_root: &Path) -> Result<Vec<PathBuf>, TerminalFinding> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()
        .map_err(|error| finding("git_worktree_list_failed", &error.to_string()))?;
    if !output.status.success() {
        return Err(finding(
            "git_worktree_list_failed",
            &String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .map(|path| fs::canonicalize(&path).unwrap_or(path))
        .collect())
}

fn worktree_dirty(path: &Path) -> Result<bool, TerminalFinding> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|error| finding("git_status_failed", &error.to_string()))?;
    if !output.status.success() {
        return Err(finding(
            "git_status_failed",
            &String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(!output.stdout.is_empty())
}

fn worktree_live(path: &Path) -> bool {
    path.join(".csdlc/live-worktree.marker").exists()
}

fn worktree_head(path: &Path) -> Result<String, TerminalFinding> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .map_err(|error| finding("git_rev_parse_failed", &error.to_string()))?;
    if !output.status.success() {
        return Err(finding(
            "git_rev_parse_failed",
            &String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn cleanup_receipt_digest(
    repository_root: &Path,
    candidate: &Path,
    candidate_head: &str,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc.v3.cleanup.git-registration.v1");
    hasher.update(b"\0");
    hasher.update(repository_root.to_string_lossy().as_bytes());
    hasher.update(b"\0");
    hasher.update(candidate.to_string_lossy().as_bytes());
    hasher.update(b"\0");
    hasher.update(candidate_head.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn contains_parent_component(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::ParentDir))
}

fn canonical_existing_ancestor(path: &Path) -> Result<PathBuf, TerminalFinding> {
    let mut cursor = path;
    loop {
        if cursor.exists() {
            return fs::canonicalize(cursor).map_err(|error| {
                finding(
                    "non_canonical_path",
                    &format!("cleanup target parent must be canonicalizable: {error}"),
                )
            });
        }
        cursor = cursor.parent().ok_or_else(|| {
            finding(
                "non_canonical_path",
                "cleanup target must have an existing canonical parent",
            )
        })?;
    }
}

fn finding(code: &str, message: &str) -> TerminalFinding {
    TerminalFinding {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_request() -> TerminalRouteRequest {
        TerminalRouteRequest {
            repository: "agent-logic/agent-design-language".into(),
            issue: 630,
            pull_request: Some(641),
            expected_head_sha: Some("0123456789012345678901234567890123456789".into()),
            mode: Some(TerminalPublicationMode::Closing),
            public_adapter_receipt: None,
            terminal_state: None,
            cleanup: None,
            cutover: None,
            credential_names: Vec::new(),
        }
    }

    #[test]
    fn terminal_verified_readback_can_derive_closeout_inside_adapter_boundary() {
        let request = base_request();
        let readback = VerifiedTerminalReadback::from_typed_adapter_receipt(
            "csdlc-github-pr-state",
            request.repository.clone(),
            request.issue,
            request.pull_request.unwrap(),
            request.expected_head_sha.clone().unwrap(),
            TerminalPublicationMode::Closing,
            true,
            false,
            Some(request.issue),
            None,
        )
        .expect("typed receipt");
        let decision = derive_finish_from_verified(&request, readback).expect("finish decision");
        assert_eq!(
            decision,
            FinishDecision::TerminalClosedOut {
                pull_request: 641,
                issue: 630,
                head_sha: "0123456789012345678901234567890123456789".into()
            }
        );
    }

    #[test]
    fn terminal_verified_readback_denies_stale_nonmerged_and_open_issue() {
        let request = base_request();
        let stale_head = VerifiedTerminalReadback::from_typed_adapter_receipt(
            "csdlc-github-pr-state",
            request.repository.clone(),
            request.issue,
            request.pull_request.unwrap(),
            "fedcba9876543210012345678901234567890123".into(),
            TerminalPublicationMode::Closing,
            true,
            false,
            Some(request.issue),
            None,
        )
        .expect("typed receipt");
        assert_eq!(
            derive_finish_from_verified(&request, stale_head)
                .unwrap_err()
                .code,
            "head_mismatch"
        );

        let nonmerged = VerifiedTerminalReadback::from_typed_adapter_receipt(
            "csdlc-github-pr-state",
            request.repository.clone(),
            request.issue,
            request.pull_request.unwrap(),
            request.expected_head_sha.clone().unwrap(),
            TerminalPublicationMode::Closing,
            false,
            false,
            Some(request.issue),
            None,
        )
        .expect("typed receipt");
        assert_eq!(
            derive_finish_from_verified(&request, nonmerged)
                .unwrap_err()
                .code,
            "pull_request_not_merged"
        );

        let open_issue = VerifiedTerminalReadback::from_typed_adapter_receipt(
            "csdlc-github-pr-state",
            request.repository.clone(),
            request.issue,
            request.pull_request.unwrap(),
            request.expected_head_sha.clone().unwrap(),
            TerminalPublicationMode::Closing,
            true,
            true,
            Some(request.issue),
            None,
        )
        .expect("typed receipt");
        assert_eq!(
            derive_finish_from_verified(&request, open_issue)
                .unwrap_err()
                .code,
            "closing_issue_still_open"
        );
    }
}
