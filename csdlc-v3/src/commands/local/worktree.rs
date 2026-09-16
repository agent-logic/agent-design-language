//! Worktree for the native local owner.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::failpoints::local_transaction_failpoint;
use super::filesystem::io_finding;
use super::{finding, DoctorFinding, PlanStatus};

pub(super) fn ensure_bind_registration(
    repository_root: &Path,
    branch: &str,
    target: &Path,
    expected_head: &str,
    require_clean: bool,
) -> Result<(), Vec<DoctorFinding>> {
    if git_worktree_registration(repository_root, branch, target)? {
        return verify_bound_worktree(branch, target, expected_head, require_clean);
    }
    if target.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "unregistered_worktree_exists",
            "target path exists but is not registered for the requested branch",
        )]);
    }
    let branch_ref = format!("refs/heads/{branch}");
    let branch_exists = Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(["show-ref", "--verify", "--quiet", &branch_ref])
        .status()
        .map_err(io_finding("git_branch_probe_failed"))?
        .success();
    if !branch_exists {
        let branch_output = Command::new("git")
            .arg("-C")
            .arg(repository_root)
            .args(["branch", "--"])
            .arg(branch)
            .arg(expected_head)
            .output()
            .map_err(io_finding("git_branch_create_failed"))?;
        if !branch_output.status.success() {
            return Err(vec![finding(
                PlanStatus::Failed,
                "git_branch_create_failed",
                String::from_utf8_lossy(&branch_output.stderr).trim(),
            )]);
        }
        local_transaction_failpoint("bind_after_branch_creation");
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(["worktree", "add", "--"])
        .arg(target)
        .arg(branch)
        .output()
        .map_err(io_finding("git_worktree_add_failed"))?;
    if !output.status.success() || !git_worktree_registration(repository_root, branch, target)? {
        return Err(vec![finding(
            PlanStatus::Failed,
            "git_worktree_add_failed",
            String::from_utf8_lossy(&output.stderr).trim(),
        )]);
    }
    verify_bound_worktree(branch, target, expected_head, require_clean)
}
pub(super) fn verify_bound_worktree(
    branch: &str,
    target: &Path,
    expected_head: &str,
    require_clean: bool,
) -> Result<(), Vec<DoctorFinding>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(target)
        .args(["rev-parse", "--is-inside-work-tree", "HEAD"])
        .output()
        .map_err(io_finding("bound_worktree_health_check_failed"))?;
    let lines: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    if !output.status.success()
        || lines.first().map(String::as_str) != Some("true")
        || lines.get(1).map(String::as_str) != Some(expected_head)
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bound_worktree_head_mismatch",
            "registered worktree must be a healthy checkout at the exact expected head",
        )]);
    }
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(target)
        .args(["symbolic-ref", "--quiet", "--short", "HEAD"])
        .output()
        .map_err(io_finding("bound_worktree_branch_check_failed"))?;
    if !branch_output.status.success()
        || String::from_utf8_lossy(&branch_output.stdout).trim() != branch
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bound_worktree_branch_mismatch",
            "registered worktree branch does not match the requested branch",
        )]);
    }
    if require_clean {
        let status = Command::new("git")
            .arg("-C")
            .arg(target)
            .args(["status", "--porcelain"])
            .output()
            .map_err(io_finding("bound_worktree_status_failed"))?;
        if !status.status.success() || !status.stdout.is_empty() {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "bound_worktree_not_clean",
                "newly bound worktree must be clean before lifecycle completion is published",
            )]);
        }
    }
    Ok(())
}
pub(super) fn canonical_existing_ancestor_local(path: &Path) -> Option<PathBuf> {
    let mut candidate = path;
    loop {
        if let Ok(canonical) = candidate.canonicalize() {
            return Some(canonical);
        }
        candidate = candidate.parent()?;
    }
}
pub(super) fn has_canonical_existing_ancestor(path: &Path) -> bool {
    let mut cursor = path;
    loop {
        if cursor.symlink_metadata().is_ok() {
            return cursor
                .canonicalize()
                .is_ok_and(|canonical| canonical == cursor);
        }
        let Some(parent) = cursor.parent() else {
            return false;
        };
        cursor = parent;
    }
}
pub(super) fn git_worktree_registration(
    repo: &Path,
    branch: &str,
    target: &Path,
) -> Result<bool, Vec<DoctorFinding>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(io_finding("git_worktree_list_failed"))?;
    if !output.status.success() {
        return Err(vec![finding(
            PlanStatus::Failed,
            "git_worktree_list_failed",
            String::from_utf8_lossy(&output.stderr).trim(),
        )]);
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let branch_ref = format!("refs/heads/{branch}");
    Ok(text.split("\n\n").any(|record| {
        record
            .lines()
            .any(|line| line == format!("worktree {}", target.display()))
            && record
                .lines()
                .any(|line| line == format!("branch {branch_ref}"))
    }))
}
