use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub const SELECTOR_PATH: &str = "csdlc-v3/operator/authority-selector.json";

#[derive(Debug, Clone, Deserialize)]
pub struct CanonicalV3Authority {
    pub schema: String,
    pub generation: String,
    pub operational_authority: String,
    pub authority_issue: u64,
    pub authority_pull_request: u64,
    pub review_authority: String,
    pub approval_authority: String,
    pub receipt_path: PathBuf,
    pub receipt_digest: String,
}

#[derive(Debug, Clone, Deserialize)]
struct NativeAuthorityReceipt {
    schema: String,
    authority_issue: u64,
    authority_pull_request: u64,
    reviewed_head: String,
    merge_commit: String,
    pr_observation_path: PathBuf,
    pr_observation_digest: String,
    terminal_receipt_path: PathBuf,
    terminal_receipt_digest: String,
    source_selector_schema: String,
    source_selector_digest: String,
    operational_authority: String,
    review_authority: String,
    approval_authority: String,
    remote_reconciliation: String,
}

#[derive(Debug, Deserialize)]
struct PullRequestObservation {
    schema: String,
    repository: String,
    pull_request: u64,
    base_ref: String,
    head_sha: String,
    merge_commit_sha: String,
    state: String,
    merged: bool,
    linked_issue: u64,
    linkage_source: String,
}

#[derive(Debug, Deserialize)]
struct TerminalReceipt {
    schema: String,
    repository: String,
    issue: u64,
    pull_request: u64,
    head_sha: String,
    disposition: String,
}

/// Returns authority only when the tracked selector is identical to the blob on
/// canonical `origin/main`. A feature checkout can carry the future selector,
/// but cannot activate it before that blob lands on the remote default branch.
pub fn canonical_v3_authority(root: &Path) -> Result<Option<CanonicalV3Authority>, String> {
    let local = match canonical_tracked_bytes(root, Path::new(SELECTOR_PATH))? {
        Some(bytes) => bytes,
        None => return Ok(None),
    };
    let value: serde_json::Value =
        serde_json::from_slice(&local).map_err(|error| error.to_string())?;
    if value["schema"] != "csdlc.v3.authority_selector.v1" || value["generation"] != "v3" {
        return Ok(None);
    }
    let selector: CanonicalV3Authority =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    if selector.schema != "csdlc.v3.authority_selector.v1"
        || selector.generation != "v3"
        || selector.operational_authority != "csdlc-v3"
        || selector.authority_issue != 505
        || selector.authority_pull_request != 591
        || selector.review_authority != "typed-exact-head"
        || selector.approval_authority != "merged-pr-591-closed-issue-505"
        || selector.receipt_path != Path::new("csdlc-v3/operator/native-authority-receipt.json")
    {
        return Ok(None);
    }
    let receipt_bytes = match canonical_tracked_bytes(root, &selector.receipt_path)? {
        Some(bytes) => bytes,
        None => return Ok(None),
    };
    if blake3::hash(&receipt_bytes).to_hex().as_str() != selector.receipt_digest {
        return Ok(None);
    }
    let receipt: NativeAuthorityReceipt =
        serde_json::from_slice(&receipt_bytes).map_err(|error| error.to_string())?;
    if receipt.schema != "csdlc.v3.native_authority_receipt.v1"
        || receipt.authority_issue != selector.authority_issue
        || receipt.authority_pull_request != selector.authority_pull_request
        || !is_lower_hex(&receipt.reviewed_head, 40)
        || !is_lower_hex(&receipt.merge_commit, 40)
        || receipt.source_selector_schema != "csdlc.generation_selector.v2"
        || !receipt
            .source_selector_digest
            .strip_prefix("sha256:")
            .is_some_and(|digest| is_lower_hex(digest, 64))
        || receipt.operational_authority != selector.operational_authority
        || receipt.review_authority != selector.review_authority
        || receipt.approval_authority != selector.approval_authority
        || receipt.terminal_receipt_path != Path::new(".csdlc/evidence/505/terminal-receipt.json")
        || !is_lower_hex(&receipt.terminal_receipt_digest, 64)
        || receipt.pr_observation_path
            != Path::new("csdlc-v3/operator/native-authority-pr-observation.json")
        || !is_lower_hex(&receipt.pr_observation_digest, 64)
        || receipt.remote_reconciliation != "canonical-terminal-receipt-and-git-objects"
    {
        return Ok(None);
    }
    let terminal_bytes = match canonical_tracked_bytes(root, &receipt.terminal_receipt_path)? {
        Some(bytes) => bytes,
        None => return Ok(None),
    };
    if blake3::hash(&terminal_bytes).to_hex().as_str() != receipt.terminal_receipt_digest {
        return Ok(None);
    }
    let terminal: TerminalReceipt =
        serde_json::from_slice(&terminal_bytes).map_err(|error| error.to_string())?;
    let observation_bytes = match canonical_tracked_bytes(root, &receipt.pr_observation_path)? {
        Some(bytes) => bytes,
        None => return Ok(None),
    };
    if blake3::hash(&observation_bytes).to_hex().as_str() != receipt.pr_observation_digest {
        return Ok(None);
    }
    let observation: PullRequestObservation =
        serde_json::from_slice(&observation_bytes).map_err(|error| error.to_string())?;
    if terminal.schema != "csdlc.v3.terminal_receipt.v1"
        || terminal.repository != "agent-logic/agent-design-language"
        || terminal.issue != receipt.authority_issue
        || terminal.pull_request != receipt.authority_pull_request
        || terminal.head_sha != receipt.reviewed_head
        || terminal.disposition != "closed_out"
        || observation.schema != "csdlc.github_pr_state.v1"
        || observation.repository != terminal.repository
        || observation.pull_request != receipt.authority_pull_request
        || observation.base_ref != "main"
        || observation.head_sha != receipt.reviewed_head
        || observation.merge_commit_sha != receipt.merge_commit
        || observation.state != "closed"
        || !observation.merged
        || observation.linked_issue != receipt.authority_issue
        || observation.linkage_source != "github_closing_issues_references"
        || !git_commit_exists(root, &receipt.merge_commit)?
        || !git_is_ancestor(root, &receipt.merge_commit, "refs/remotes/origin/main")?
        || !git_commit_subject(root, &receipt.merge_commit)?.contains("(#591)")
    {
        return Ok(None);
    }
    Ok(Some(selector))
}

fn git_is_ancestor(root: &Path, ancestor: &str, descendant: &str) -> Result<bool, String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .status()
        .map_err(|error| error.to_string())?;
    Ok(status.success())
}

fn git_commit_exists(root: &Path, revision: &str) -> Result<bool, String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["cat-file", "-e", &format!("{revision}^{{commit}}")])
        .status()
        .map_err(|error| error.to_string())?;
    Ok(status.success())
}

fn git_commit_subject(root: &Path, revision: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["show", "-s", "--format=%s", revision])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Ok(String::new());
    }
    String::from_utf8(output.stdout)
        .map(|subject| subject.trim_end().to_owned())
        .map_err(|error| error.to_string())
}

fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn canonical_tracked_bytes(root: &Path, relative: &Path) -> Result<Option<Vec<u8>>, String> {
    let local = match std::fs::read(root.join(relative)) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "show",
            &format!("refs/remotes/origin/main:{}", relative.display()),
        ])
        .output()
        .map_err(|error| error.to_string())?;
    Ok((output.status.success() && output.stdout == local).then_some(local))
}

pub fn canonical_v2_rollback(root: &Path) -> Result<bool, String> {
    let Some(bytes) = canonical_tracked_bytes(root, Path::new(SELECTOR_PATH))? else {
        return Ok(false);
    };
    let selector: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    Ok(selector["schema"] == "csdlc.v3.authority_selector.v1"
        && selector["generation"] == "rollback"
        && selector["operational_authority"] == "suspended"
        && selector["authority_issue"] == 505
        && selector["authority_pull_request"] == 591)
}

#[cfg(test)]
mod tests {
    use super::{canonical_v3_authority, is_lower_hex, SELECTOR_PATH};
    use std::{fs, path::PathBuf, process::Command};

    #[test]
    fn immutable_identifiers_require_exact_lower_hex() {
        assert!(is_lower_hex(&"a1".repeat(20), 40));
        assert!(is_lower_hex(&"ab".repeat(32), 64));
        assert!(!is_lower_hex(&"g".repeat(40), 40));
        assert!(!is_lower_hex(&"A".repeat(40), 40));
        assert!(!is_lower_hex("", 64));
        assert!(!is_lower_hex(&"a".repeat(63), 64));
    }

    #[test]
    fn checked_in_authority_artifacts_activate_in_post_merge_equivalent_clone() {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/authority-post-merge")
            .join(std::process::id().to_string());
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        let run = |cwd: &std::path::Path, args: &[&str]| {
            let output = Command::new("git")
                .arg("-C")
                .arg(cwd)
                .args(args)
                .env("GIT_AUTHOR_NAME", "C-SDLC v3 authority test")
                .env("GIT_AUTHOR_EMAIL", "csdlc-v3@example.invalid")
                .env("GIT_COMMITTER_NAME", "C-SDLC v3 authority test")
                .env("GIT_COMMITTER_EMAIL", "csdlc-v3@example.invalid")
                .output()
                .unwrap();
            assert!(output.status.success(), "git {args:?}: {output:?}");
        };
        let output = Command::new("git")
            .args([
                "clone",
                "--quiet",
                source.to_str().unwrap(),
                root.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "clone source repository: {output:?}"
        );
        for relative in [
            SELECTOR_PATH,
            "csdlc-v3/operator/native-authority-receipt.json",
            "csdlc-v3/operator/native-authority-pr-observation.json",
            ".csdlc/evidence/505/terminal-receipt.json",
        ] {
            let destination = root.join(relative);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(source.join(relative), destination).unwrap();
        }
        run(
            &root,
            &[
                "add",
                SELECTOR_PATH,
                "csdlc-v3/operator/native-authority-receipt.json",
                "csdlc-v3/operator/native-authority-pr-observation.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        run(
            &root,
            &[
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "post-merge native authority",
            ],
        );
        run(&root, &["update-ref", "refs/remotes/origin/main", "HEAD"]);

        let authority = canonical_v3_authority(&root).unwrap();
        assert_eq!(authority.unwrap().operational_authority, "csdlc-v3");

        let unrelated = Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["rev-parse", "HEAD^"])
            .output()
            .unwrap();
        assert!(unrelated.status.success());
        let unrelated = String::from_utf8(unrelated.stdout)
            .unwrap()
            .trim()
            .to_owned();
        assert_ne!(unrelated, "74ddb31702482172eea4ba3d74700536eab32e49");
        let terminal_path = root.join(".csdlc/evidence/505/terminal-receipt.json");
        let mut terminal: serde_json::Value =
            serde_json::from_slice(&fs::read(&terminal_path).unwrap()).unwrap();
        terminal["head_sha"] = serde_json::Value::String(unrelated.clone());
        let terminal_bytes = serde_json::to_vec_pretty(&terminal).unwrap();
        fs::write(&terminal_path, &terminal_bytes).unwrap();
        let receipt_path = root.join("csdlc-v3/operator/native-authority-receipt.json");
        let mut receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
        receipt["reviewed_head"] = serde_json::Value::String(unrelated);
        receipt["terminal_receipt_digest"] =
            serde_json::Value::String(blake3::hash(&terminal_bytes).to_hex().to_string());
        let receipt_bytes = serde_json::to_vec_pretty(&receipt).unwrap();
        fs::write(&receipt_path, &receipt_bytes).unwrap();
        let selector_path = root.join(SELECTOR_PATH);
        let mut selector: serde_json::Value =
            serde_json::from_slice(&fs::read(&selector_path).unwrap()).unwrap();
        selector["receipt_digest"] =
            serde_json::Value::String(blake3::hash(&receipt_bytes).to_hex().to_string());
        fs::write(
            &selector_path,
            serde_json::to_vec_pretty(&selector).unwrap(),
        )
        .unwrap();
        run(
            &root,
            &[
                "add",
                SELECTOR_PATH,
                "csdlc-v3/operator/native-authority-receipt.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        run(
            &root,
            &["commit", "--quiet", "-m", "tamper authority receipt"],
        );
        run(&root, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        assert!(canonical_v3_authority(&root).unwrap().is_none());

        for relative in [
            SELECTOR_PATH,
            "csdlc-v3/operator/native-authority-receipt.json",
            "csdlc-v3/operator/native-authority-pr-observation.json",
            ".csdlc/evidence/505/terminal-receipt.json",
        ] {
            fs::copy(source.join(relative), root.join(relative)).unwrap();
        }
        let observation_path = root.join("csdlc-v3/operator/native-authority-pr-observation.json");
        let mut observation: serde_json::Value =
            serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
        observation["merge_commit_sha"] = serde_json::Value::String("1".repeat(40));
        let observation_bytes = serde_json::to_vec_pretty(&observation).unwrap();
        fs::write(&observation_path, &observation_bytes).unwrap();
        let receipt_path = root.join("csdlc-v3/operator/native-authority-receipt.json");
        let mut receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
        receipt["merge_commit"] = serde_json::Value::String("1".repeat(40));
        receipt["pr_observation_digest"] =
            serde_json::Value::String(blake3::hash(&observation_bytes).to_hex().to_string());
        let receipt_bytes = serde_json::to_vec_pretty(&receipt).unwrap();
        fs::write(&receipt_path, &receipt_bytes).unwrap();
        let selector_path = root.join(SELECTOR_PATH);
        let mut selector: serde_json::Value =
            serde_json::from_slice(&fs::read(&selector_path).unwrap()).unwrap();
        selector["receipt_digest"] =
            serde_json::Value::String(blake3::hash(&receipt_bytes).to_hex().to_string());
        fs::write(
            &selector_path,
            serde_json::to_vec_pretty(&selector).unwrap(),
        )
        .unwrap();
        run(
            &root,
            &[
                "add",
                SELECTOR_PATH,
                "csdlc-v3/operator/native-authority-receipt.json",
                "csdlc-v3/operator/native-authority-pr-observation.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        run(
            &root,
            &["commit", "--quiet", "-m", "substitute missing merge object"],
        );
        run(&root, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        assert!(canonical_v3_authority(&root).unwrap().is_none());
        fs::remove_dir_all(root).unwrap();
    }
}
