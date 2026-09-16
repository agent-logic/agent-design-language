//! Canonical v3 authority selector loading and verification.

use std::{fs, path::Path, process::Command};

use super::model::*;
use super::support::{
    is_full_git_sha, remote_finding, stable_digest, CANONICAL_AUTHORITY_SELECTOR_PATH,
};

pub fn canonical_authority_selector_digest(repo_root: &Path) -> Result<String, RemoteRouteFinding> {
    let bytes = read_canonical_authority_selector(repo_root)?;
    Ok(stable_digest(&[std::str::from_utf8(&bytes).map_err(
        |_| {
            remote_finding(
                "canonical_authority_selector_invalid",
                "canonical authority selector must be UTF-8 JSON",
            )
        },
    )?]))
}

pub(super) fn read_canonical_authority_selector(
    repo_root: &Path,
) -> Result<Vec<u8>, RemoteRouteFinding> {
    fs::read(repo_root.join(CANONICAL_AUTHORITY_SELECTOR_PATH)).map_err(|_| {
        remote_finding(
            "canonical_v3_authority_missing",
            "canonical generation selector is unavailable; v3 remote operations remain denied",
        )
    })
}

pub(super) fn verify_canonical_v3_authority(
    repo_root: &Path,
    expected_lifecycle_digest: Option<&str>,
    exact_review_sha: &str,
) -> Result<CanonicalV3AuthorityEvidence, RemoteRouteFinding> {
    let exact_review_sha = exact_review_sha.trim();
    if !is_full_git_sha(exact_review_sha) {
        return Err(remote_finding(
            "canonical_exact_review_sha_invalid",
            "operational v3 authority requires a full 40-character exact review SHA",
        ));
    }
    let bytes = read_canonical_authority_selector(repo_root)?;
    let selector_digest = stable_digest(&[std::str::from_utf8(&bytes).map_err(|_| {
        remote_finding(
            "canonical_authority_selector_invalid",
            "canonical authority selector must be UTF-8 JSON",
        )
    })?]);
    if expected_lifecycle_digest
        .is_some_and(|expected| expected.trim().is_empty() || expected != selector_digest)
    {
        return Err(remote_finding(
            "canonical_authority_selector_digest_mismatch",
            "expected lifecycle digest does not match the canonical authority selector",
        ));
    }
    let selector = crate::authority::canonical_v3_authority(repo_root)
        .map_err(|error| remote_finding("canonical_authority_selector_invalid", &error))?
        .ok_or_else(|| {
            remote_finding(
                "canonical_v3_authority_inactive",
                "canonical selector is not active on origin/main",
            )
        })?;
    let current_head = git_head(repo_root)?;
    if current_head != exact_review_sha {
        return Err(remote_finding(
            "canonical_exact_review_sha_mismatch",
            "operational v3 authority requires exact_review_sha to match the checked-out Git HEAD",
        ));
    }
    Ok(CanonicalV3AuthorityEvidence {
        schema: "csdlc.v3.canonical_authority_evidence.v1".into(),
        selector_path: CANONICAL_AUTHORITY_SELECTOR_PATH.into(),
        selector_digest: selector_digest.clone(),
        authority_issue: selector.authority_issue,
        exact_review_sha: exact_review_sha.to_owned(),
        readiness_evidence_digest: selector_digest.clone(),
        approval_evidence_digest: selector_digest,
    })
}

pub(super) fn git_head(repo_root: &Path) -> Result<String, RemoteRouteFinding> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .map_err(|_| {
            remote_finding(
                "canonical_exact_head_unavailable",
                "unable to read the checked-out Git HEAD for operational v3 authority",
            )
        })?;
    if !output.status.success() {
        return Err(remote_finding(
            "canonical_exact_head_unavailable",
            "unable to read the checked-out Git HEAD for operational v3 authority",
        ));
    }
    let head = String::from_utf8(output.stdout).map_err(|_| {
        remote_finding(
            "canonical_exact_head_unavailable",
            "checked-out Git HEAD output was not valid UTF-8",
        )
    })?;
    let head = head.trim();
    if !is_full_git_sha(head) {
        return Err(remote_finding(
            "canonical_exact_head_unavailable",
            "checked-out Git HEAD did not resolve to a full 40-character SHA",
        ));
    }
    Ok(head.to_owned())
}
