use serde::Deserialize;
use std::{path::Path, process::Command};

pub const SELECTOR_PATH: &str = "csdlc-v2/operator/generation-selector.json";

#[derive(Debug, Clone, Deserialize)]
pub struct CanonicalV3Authority {
    pub schema: String,
    pub default_generation: String,
    pub operational_authority: String,
    pub authority_issue: u64,
    pub authority_pull_request: u64,
    pub review_authority: String,
    pub approval_authority: String,
}

/// Returns authority only when the tracked selector is identical to the blob on
/// canonical `origin/main`. A feature checkout can carry the future selector,
/// but cannot activate it before that blob lands on the remote default branch.
pub fn canonical_v3_authority(root: &Path) -> Result<Option<CanonicalV3Authority>, String> {
    let local = match std::fs::read(root.join(SELECTOR_PATH)) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["show", &format!("refs/remotes/origin/main:{SELECTOR_PATH}")])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() || output.stdout != local {
        return Ok(None);
    }
    let value: serde_json::Value =
        serde_json::from_slice(&local).map_err(|error| error.to_string())?;
    if value["schema"] != "csdlc.generation_selector.v2" || value["default_generation"] != "v3" {
        return Ok(None);
    }
    let selector: CanonicalV3Authority =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    if selector.schema != "csdlc.generation_selector.v2"
        || selector.default_generation != "v3"
        || selector.operational_authority != "csdlc-v3"
        || selector.authority_issue != 505
        || selector.authority_pull_request != 591
        || selector.review_authority != "typed-v2-exact-head"
        || selector.approval_authority != "merged-pr-591-closed-issue-505"
    {
        return Ok(None);
    }
    Ok(Some(selector))
}
