use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Component, Path, PathBuf},
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
    RemovalDeniedPreCutover {
        path: PathBuf,
        receipt_digest: String,
        reason: String,
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
    #[cfg(test)]
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
    let receipt_digest = cleanup_receipt_digest(&repository_root, &candidate);
    if request.remove {
        if request.preview_receipt_digest.as_deref() != Some(receipt_digest.as_str()) {
            return Err(finding(
                "preview_receipt_mismatch",
                "cleanup removal requires a preview receipt for the same Git registration",
            ));
        }
        Ok(CleanupDecision::RemovalDeniedPreCutover {
            path: candidate,
            receipt_digest,
            reason:
                "v3 clean is non-authoritative before #505 cutover and must not remove worktrees"
                    .into(),
        })
    } else {
        Ok(CleanupDecision::Removable {
            path: candidate,
            receipt_digest,
        })
    }
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
            cleanup: None,
            cutover: None,
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
