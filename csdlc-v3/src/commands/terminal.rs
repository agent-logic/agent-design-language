use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
    pub cleanup: Option<CleanupRouteRequest>,
    #[serde(default)]
    pub cutover: Option<CutoverDecisionRequest>,
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
    pub preview_receipt_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutoverDecisionRequest {
    pub operator: String,
    pub approval: String,
    pub selected_binary_provenance: String,
    pub rollback_evidence: String,
    pub undo_boundary: String,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CutoverDecision {
    pub approved_by: String,
    pub selected_binary_provenance: String,
    pub rollback_evidence: String,
    pub undo_boundary: String,
    pub executes_cutover: bool,
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
    pub fn from_typed_adapter_receipt(
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
            Some(cleanup_request) => match classify_cleanup_from_git(cleanup_request) {
                Ok(decision) => cleanup = Some(decision),
                Err(finding) => findings.push(finding),
            },
            None => findings.push(finding(
                "missing_cleanup_request",
                "clean route requires a cleanup request",
            )),
        },
        "cutover" => match request.cutover.as_ref() {
            Some(cutover_request) => match decide_cutover(cutover_request) {
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
    Ok(TerminalRoutePlan {
        route: route.to_owned(),
        issue: request.issue,
        repository: request.repository.clone(),
        status,
        operational_authority: false,
        findings,
        finish,
        cleanup,
        cutover,
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
    request: &CleanupRouteRequest,
) -> Result<CleanupDecision, TerminalFinding> {
    if !request.terminal_receipt {
        return Err(finding(
            "missing_terminal_receipt",
            "cleanup requires terminal closeout receipt truth",
        ));
    }
    let approved_parent = canonical_dir(&request.approved_parent, "approved_parent")?;
    let repository_root = canonical_dir(&request.repository_root, "repository_root")?;
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
    let receipt_digest = cleanup_receipt_digest(&repository_root, &candidate);
    if request.remove {
        if request.preview_receipt_digest.as_deref() != Some(receipt_digest.as_str()) {
            return Err(finding(
                "preview_receipt_mismatch",
                "cleanup removal requires a preview receipt for the same Git registration",
            ));
        }
        fs::remove_dir_all(&candidate).map_err(|error| {
            finding(
                "cleanup_remove_failed",
                &format!("failed to remove registered worktree: {error}"),
            )
        })?;
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

fn decide_cutover(
    request: &CutoverDecisionRequest,
) -> Result<CutoverDecision, Vec<TerminalFinding>> {
    let mut findings = Vec::new();
    for (code, label, value) in [
        ("missing_operator", "operator", &request.operator),
        ("missing_operator_approval", "approval", &request.approval),
        (
            "missing_binary_provenance",
            "selected binary provenance",
            &request.selected_binary_provenance,
        ),
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
    if !request.approval.contains("#505") {
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
    if findings.is_empty() {
        Ok(CutoverDecision {
            approved_by: request.operator.clone(),
            selected_binary_provenance: request.selected_binary_provenance.clone(),
            rollback_evidence: request.rollback_evidence.clone(),
            undo_boundary: request.undo_boundary.clone(),
            executes_cutover: false,
        })
    } else {
        Err(findings)
    }
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

fn cleanup_receipt_digest(repository_root: &Path, candidate: &Path) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc.v3.cleanup.git-registration.v1");
    hasher.update(b"\0");
    hasher.update(repository_root.to_string_lossy().as_bytes());
    hasher.update(b"\0");
    hasher.update(candidate.to_string_lossy().as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn finding(code: &str, message: &str) -> TerminalFinding {
    TerminalFinding {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
