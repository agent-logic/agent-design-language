use super::super::*;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

pub(crate) fn sync_completed_output_surfaces(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
    completed_output_path: &Path,
) -> Result<PathBuf> {
    let normalized_output_path = if completed_output_path.is_absolute() {
        completed_output_path.to_path_buf()
    } else {
        repo_root.join(completed_output_path)
    };
    let canonical_root_output = issue_ref.task_bundle_output_path(primary_root);
    let copied_to_root =
        !(same_filesystem_target(&normalized_output_path, &canonical_root_output)?);
    if copied_to_root {
        super::reconciliation::ensure_canonical_output_is_local_only(
            primary_root,
            &canonical_root_output,
            "finish: canonical .adl output surfaces must remain local-only during output sync",
        )?;
        if let Some(parent) = canonical_root_output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&normalized_output_path, &canonical_root_output).with_context(|| {
            format!(
                "finish: failed to sync completed output card '{}' to canonical local task bundle '{}'",
                normalized_output_path.display(),
                canonical_root_output.display()
            )
        })?;
        super::super::validate_completed_sor(repo_root, &canonical_root_output)?;
    }

    let cards_root = resolve_cards_root(primary_root, None);
    let review_output = card_output_path(&cards_root, issue_ref.issue_number());
    ensure_symlink(&review_output, &canonical_root_output)?;
    Ok(canonical_root_output)
}

pub(crate) fn same_filesystem_target(left: &Path, right: &Path) -> Result<bool> {
    if left == right {
        return Ok(true);
    }
    if left.exists() && right.exists() {
        let left_canonical = fs::canonicalize(left)?;
        let right_canonical = fs::canonicalize(right)?;
        return Ok(left_canonical == right_canonical);
    }
    Ok(false)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IssueWorktreePruneResult {
    Missing(String),
    Pruned(String),
    RetainedWithReason(String),
}

impl IssueWorktreePruneResult {
    pub(crate) fn card_value(&self) -> String {
        match self {
            Self::Missing(name) => format!("skipped_missing: {name}"),
            Self::Pruned(name) => format!("pruned: {name}"),
            Self::RetainedWithReason(name) => {
                format!("retained_with_reason: dirty stale worktree retained: {name}")
            }
        }
    }
}

pub(crate) fn worktree_display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "issue worktree".to_string())
}

pub(crate) fn record_worktree_prune_result(path: &Path, result: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    super::reconciliation::replace_or_insert_after_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        "- Worktree prune result:",
        &format!("- Worktree prune result: {result}"),
    );
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn replace_worktree_only_paths_remaining(path: &Path, value: &str) -> Result<()> {
    let mut text = fs::read_to_string(path)?;
    super::reconciliation::replace_first_prefixed_line(
        &mut text,
        "- Worktree-only paths remaining:",
        &format!("- Worktree-only paths remaining: {value}"),
    );
    fs::write(path, text)?;
    Ok(())
}

pub(crate) fn scrub_noncanonical_issue_bundle_residue(
    worktree_root: &Path,
    issue_ref: &IssueRef,
) -> Result<()> {
    let adl_root = worktree_root.join(".adl");
    if !adl_root.is_dir() {
        return Ok(());
    }

    let canonical_body = issue_ref.issue_prompt_path(worktree_root);
    let canonical_bundle = issue_ref.task_bundle_dir_path(worktree_root);
    let body_prefix = format!("issue-{:04}-", issue_ref.issue_number());
    let task_prefix = format!("issue-{:04}__", issue_ref.issue_number());

    for scope_entry in fs::read_dir(&adl_root)? {
        let scope_entry = scope_entry?;
        let scope_path = scope_entry.path();

        let bodies = scope_path.join("bodies");
        if bodies.is_dir() {
            for entry in fs::read_dir(&bodies)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("issue-")
                    && name.starts_with(&body_prefix)
                    && path != canonical_body
                {
                    if path_has_tracked_entries(worktree_root, &path)? {
                        continue;
                    }
                    fs::remove_file(&path).with_context(|| {
                        format!(
                            "closeout: failed to scrub noncanonical local issue prompt residue '{}'",
                            path.display()
                        )
                    })?;
                }
            }
        }

        let tasks = scope_path.join("tasks");
        if tasks.is_dir() {
            for entry in fs::read_dir(&tasks)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("issue-")
                    && name.starts_with(&task_prefix)
                    && path != canonical_bundle
                {
                    if path_has_tracked_entries(worktree_root, &path)? {
                        scrub_untracked_entries(worktree_root, &path)?;
                        continue;
                    }
                    fs::remove_dir_all(&path).with_context(|| {
                        format!(
                            "closeout: failed to scrub noncanonical local task-bundle residue '{}'",
                            path.display()
                        )
                    })?;
                }
            }
        }
    }

    Ok(())
}

fn path_has_tracked_entries(repo_root: &Path, path: &Path) -> Result<bool> {
    let Ok(relative) = path.strip_prefix(repo_root) else {
        return Ok(false);
    };
    let output = Command::new("git")
        .args([
            "-C",
            path_str(repo_root)?,
            "ls-files",
            "--",
            path_str(relative)?,
        ])
        .output()
        .context("closeout: failed to inspect tracked noncanonical issue residue")?;
    if !output.status.success() {
        bail!(
            "closeout: failed to inspect tracked noncanonical issue residue '{}'",
            path.display()
        );
    }
    Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
}

fn scrub_untracked_entries(repo_root: &Path, path: &Path) -> Result<()> {
    let Ok(relative) = path.strip_prefix(repo_root) else {
        return Ok(());
    };
    let status = Command::new("git")
        .args([
            "-C",
            path_str(repo_root)?,
            "clean",
            "-fd",
            "--",
            path_str(relative)?,
        ])
        .status()
        .context("closeout: failed to scrub untracked noncanonical issue residue")?;
    if !status.success() {
        bail!(
            "closeout: failed to scrub untracked noncanonical issue residue '{}'",
            path.display()
        );
    }
    Ok(())
}

pub(crate) fn prune_issue_worktree(
    repo_root: &Path,
    primary_root: &Path,
    issue_ref: &IssueRef,
) -> Result<IssueWorktreePruneResult> {
    let worktree_path = issue_ref.default_worktree_path(
        primary_root,
        std::env::var_os("ADL_WORKTREE_ROOT")
            .map(PathBuf::from)
            .as_deref(),
    );
    if !worktree_path.is_dir() {
        println!(
            "closeout: issue worktree already absent; prune not needed: {}",
            worktree_path.display()
        );
        return Ok(IssueWorktreePruneResult::Missing(worktree_display_name(
            &worktree_path,
        )));
    }
    ensure_no_active_runtime_pidfiles_before_prune(issue_ref, &worktree_path)?;
    if has_uncommitted_or_untracked_changes(&worktree_path)? {
        bail!(
            "closeout: refusing to prune dirty worktree '{}' because it contains staged, unstaged, or untracked changes",
            worktree_path.display()
        );
    }
    let current_dir = env::current_dir().context("closeout: determine current directory")?;
    if current_dir.starts_with(&worktree_path) {
        env::set_current_dir(primary_root).with_context(|| {
            format!(
                "closeout: failed to leave worktree '{}' before pruning",
                worktree_path.display()
            )
        })?;
    }
    run_status(
        "git",
        &[
            "-C",
            path_str(repo_root)?,
            "worktree",
            "remove",
            path_str(&worktree_path)?,
        ],
    )?;
    println!(
        "closeout: pruned issue worktree: {}",
        worktree_path.display()
    );
    Ok(IssueWorktreePruneResult::Pruned(worktree_display_name(
        &worktree_path,
    )))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RuntimePidfileStatus {
    pub(super) role: String,
    pub(super) pid_file: PathBuf,
    pub(super) status: String,
}

#[derive(Debug, Deserialize)]
struct ProcessStatusHelperReport {
    status: String,
    broad_process_scan: bool,
    uses_ps: bool,
}

fn ensure_no_active_runtime_pidfiles_before_prune(
    issue_ref: &IssueRef,
    worktree_path: &Path,
) -> Result<()> {
    let statuses =
        active_runtime_pidfile_statuses(worktree_path, check_pidfile_with_process_status_helper)?;
    fail_if_active_runtime_pidfiles(issue_ref, worktree_path, &statuses)
}

pub(super) fn active_runtime_pidfile_statuses<F>(
    worktree_path: &Path,
    mut checker: F,
) -> Result<Vec<RuntimePidfileStatus>>
where
    F: FnMut(&Path) -> Result<String>,
{
    let mut statuses = Vec::new();
    for pid_file in collect_runtime_pidfiles(worktree_path)? {
        let status = checker(&pid_file).with_context(|| {
            format!(
                "closeout: failed to classify runtime pidfile '{}' with permission-safe process status helper",
                pid_file.display()
            )
        })?;
        statuses.push(RuntimePidfileStatus {
            role: runtime_pidfile_role(worktree_path, &pid_file),
            pid_file,
            status,
        });
    }
    Ok(statuses)
}

pub(super) fn fail_if_active_runtime_pidfiles(
    issue_ref: &IssueRef,
    worktree_path: &Path,
    statuses: &[RuntimePidfileStatus],
) -> Result<()> {
    let blockers: Vec<&RuntimePidfileStatus> = statuses
        .iter()
        .filter(|status| !matches!(status.status.as_str(), "stale_pid" | "missing_metadata"))
        .collect();
    if blockers.is_empty() {
        return Ok(());
    }

    let details = blockers
        .iter()
        .map(|status| {
            format!(
                "{} status={} pid_file={}",
                status.role,
                status.status,
                runtime_pidfile_display_path(worktree_path, &status.pid_file)
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    bail!(
        "closeout: refusing to prune issue worktree '{}' because active or unclassifiable runtime pidfiles were detected: {}. Settle the runtime first through the CSM owner stop/control-plane path, retain final process/checkpoint/safe-fail/OTel evidence, then rerun `bash adl/tools/pr.sh closeout {}` or the applicable `bash adl/tools/pr.sh finish {} ...` path. Detection used `adl process status --pid-file <path> --json` and no broad process scan.",
        worktree_path.display(),
        details,
        issue_ref.issue_number(),
        issue_ref.issue_number()
    );
}

fn check_pidfile_with_process_status_helper(pid_file: &Path) -> Result<String> {
    let helper = std::env::var_os("ADL_PROCESS_STATUS_BIN")
        .map(PathBuf::from)
        .map(Ok)
        .unwrap_or_else(std::env::current_exe)
        .context("closeout: failed to resolve process status helper binary")?;
    let helper_name = helper.file_stem().and_then(|name| name.to_str());
    let mut command = Command::new(&helper);
    if helper_name == Some("adl-process") {
        command.arg("status");
    } else {
        command.args(["process", "status"]);
    }
    let output = command
        .args(["--pid-file", path_str(pid_file)?, "--json"])
        .output()
        .with_context(|| {
            format!(
                "closeout: failed to run permission-safe process status helper for '{}'",
                pid_file.display()
            )
        })?;
    if !output.status.success() {
        bail!(
            "closeout: process status helper failed for runtime pidfile '{}'",
            pid_file.display()
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json_start = stdout
        .find('{')
        .ok_or_else(|| anyhow!("closeout: process status helper emitted no JSON object"))?;
    let report: ProcessStatusHelperReport = serde_json::from_str(&stdout[json_start..])
        .with_context(|| {
            format!(
                "closeout: failed to parse process status helper JSON for '{}'",
                pid_file.display()
            )
        })?;
    if report.broad_process_scan || report.uses_ps {
        bail!(
            "closeout: process status helper violated permission-safe contract for '{}'",
            pid_file.display()
        );
    }
    Ok(report.status)
}

fn collect_runtime_pidfiles(worktree_path: &Path) -> Result<Vec<PathBuf>> {
    let mut pidfiles = BTreeSet::new();
    for relative in known_runtime_pidfile_paths() {
        let candidate = worktree_path.join(relative);
        if candidate.is_file() {
            pidfiles.insert(candidate);
        }
    }
    for relative in runtime_pidfile_scan_roots() {
        let root = worktree_path.join(relative);
        if root.is_dir() {
            collect_pidfiles_under(&root, &mut pidfiles)?;
        }
    }
    Ok(pidfiles.into_iter().collect())
}

fn known_runtime_pidfile_paths() -> &'static [&'static str] {
    &[
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/logs/csm.pid",
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/collector/collector.pid",
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/api/csm-api.pid",
    ]
}

fn runtime_pidfile_scan_roots() -> &'static [&'static str] {
    &[
        ".adl/runs",
        ".adl/runtime",
        ".adl/csm",
        "docs/milestones/v0.91.7/review/runtime",
    ]
}

fn collect_pidfiles_under(root: &Path, pidfiles: &mut BTreeSet<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if should_skip_runtime_pidfile_dir(&path) {
                continue;
            }
            collect_pidfiles_under(&path, pidfiles)?;
        } else if file_type.is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("pid")
        {
            pidfiles.insert(path);
        }
    }
    Ok(())
}

fn should_skip_runtime_pidfile_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | "target" | "node_modules" | ".worktrees")
    )
}

fn runtime_pidfile_role(worktree_path: &Path, pid_file: &Path) -> String {
    let relative = runtime_pidfile_display_path(worktree_path, pid_file);
    match relative.as_str() {
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/logs/csm.pid" => {
            "csm_daemon".to_string()
        }
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/collector/collector.pid" => {
            "otlp_loopback_collector".to_string()
        }
        "docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/full/api/csm-api.pid" => {
            "csm_api".to_string()
        }
        _ => relative
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".pid"))
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("runtime_child")
            .to_string(),
    }
}

fn runtime_pidfile_display_path(worktree_path: &Path, pid_file: &Path) -> String {
    pid_file
        .strip_prefix(worktree_path)
        .unwrap_or(pid_file)
        .to_string_lossy()
        .replace('\\', "/")
}
