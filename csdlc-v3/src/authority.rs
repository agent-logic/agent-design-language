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
    source_selector_schema: String,
    source_selector_digest: String,
    operational_authority: String,
    review_authority: String,
    approval_authority: String,
    remote_reconciliation: String,
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
        || receipt.reviewed_head.len() != 40
        || receipt.merge_commit.len() != 40
        || receipt.source_selector_schema != "csdlc.generation_selector.v2"
        || !receipt.source_selector_digest.starts_with("sha256:")
        || receipt.operational_authority != selector.operational_authority
        || receipt.review_authority != selector.review_authority
        || receipt.approval_authority != selector.approval_authority
        || receipt.remote_reconciliation != "authenticated-readback-required"
    {
        return Ok(None);
    }
    Ok(Some(selector))
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
