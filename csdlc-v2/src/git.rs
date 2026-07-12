use std::path::Path;
use std::process::Command;

use crate::error::{ErrorCode, Result, V2Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitOutput {
    pub stdout: String,
    pub stderr: String,
}

pub fn run(root: &Path, args: &[&str]) -> Result<GitOutput> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            format!(
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    Ok(GitOutput {
        stdout: String::from_utf8_lossy(&output.stdout).trim().into(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().into(),
    })
}

pub fn current_branch(root: &Path) -> Result<String> {
    Ok(run(root, &["branch", "--show-current"])?.stdout)
}

pub fn worktrees(root: &Path) -> Result<Vec<(String, String)>> {
    let text = run(root, &["worktree", "list", "--porcelain"])?.stdout;
    let mut result = Vec::new();
    let mut path = None;
    for line in text.lines().chain(std::iter::once("")) {
        if let Some(value) = line.strip_prefix("worktree ") {
            path = Some(value.to_owned());
        }
        if let (Some(branch), Some(path)) = (line.strip_prefix("branch refs/heads/"), path.as_ref())
        {
            result.push((branch.to_owned(), path.clone()));
        }
    }
    Ok(result)
}
