use crate::codefriend::{
    evidence::{
        contracts::{Artifact, Publication, PublicationState, ReviewRecord, CONTRACT},
        hash, valid_digest,
    },
    ingestion::{unsafe_content, validate_path},
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    path::{Component, Path, PathBuf},
};

pub const MANIFEST_INPUT_SCHEMA: &str = "codefriend.publication_manifest_input.v1";
const MAX_INPUT_BYTES: u64 = 2 * 1024 * 1024;
const MAX_ARTIFACT_BYTES: u64 = 10 * 1024 * 1024;
const MAX_TOTAL_ARTIFACT_BYTES: u64 = 100 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManifestInput {
    pub schema: String,
    pub artifact_manifest: Vec<Artifact>,
    pub renderer_versions: BTreeMap<String, String>,
    pub target: String,
    pub claims: Vec<String>,
    pub nonclaims: Vec<String>,
}

impl ManifestInput {
    pub fn read(path: &Path) -> Result<Self> {
        read_json(path, "publication_manifest_input")
    }

    pub fn publication(
        &self,
        review: &ReviewRecord,
        destination_root: &Path,
    ) -> Result<Publication> {
        ensure!(
            self.schema == MANIFEST_INPUT_SCHEMA,
            "unsupported_manifest_input_version"
        );
        validate_path(&self.target)?;
        let publication = Publication {
            schema: CONTRACT.to_string(),
            run_digest: hash(&review.run)?,
            finding_set_digest: review.finding_digest()?,
            manifest_digest: hash(&self.artifact_manifest)?,
            artifact_manifest: self.artifact_manifest.clone(),
            renderer_versions: self.renderer_versions.clone(),
            scope_digest: review.run.scope_digest.clone(),
            target: self.target.clone(),
            destination_digest: destination_digest(destination_root)?,
            claims: self.claims.clone(),
            nonclaims: self.nonclaims.clone(),
            state: PublicationState::Withheld,
            approval: None,
        };
        publication.validate(review)?;
        Ok(publication)
    }
}

pub(crate) fn destination_digest(root: &Path) -> Result<String> {
    ensure!(root.exists(), "destination_root_missing");
    reject_symlink_components(root)?;
    ensure!(
        fs::symlink_metadata(root)?.file_type().is_dir(),
        "invalid_destination_root"
    );
    let canonical = root.canonicalize()?;
    let value = canonical
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("destination_root_not_utf8"))?;
    hash(&("codefriend.publication_destination.v1", value))
}

pub fn read_review(path: &Path) -> Result<ReviewRecord> {
    let review: ReviewRecord = read_json(path, "review_record")?;
    review.validate()?;
    Ok(review)
}

pub fn read_publication(path: &Path) -> Result<Publication> {
    read_json(path, "publication")
}

pub fn verify_artifacts(root: &Path, artifacts: &[Artifact]) -> Result<()> {
    snapshot_artifacts(root, artifacts).map(|_| ())
}

#[derive(Debug)]
pub(crate) struct VerifiedArtifact {
    pub(crate) path: String,
    pub(crate) bytes: Vec<u8>,
}

/// Open, validate and retain the exact bytes that may be published. Admission
/// consumes this snapshot instead of reopening attacker-mutable source paths.
pub(crate) fn snapshot_artifacts(
    root: &Path,
    artifacts: &[Artifact],
) -> Result<Vec<VerifiedArtifact>> {
    ensure!(!artifacts.is_empty(), "empty_artifact_manifest");
    ensure!(root.exists(), "artifact_root_missing");
    ensure!(
        fs::symlink_metadata(root)?.file_type().is_dir(),
        "invalid_artifact_root"
    );
    reject_symlink_components(root)?;

    let declared: Vec<_> = artifacts
        .iter()
        .map(|artifact| artifact.path.clone())
        .collect();
    ensure!(
        declared.windows(2).all(|pair| pair[0] < pair[1]),
        "artifact_manifest_not_canonical"
    );
    let actual = inventory(root)?;
    ensure!(actual == declared, "artifact_manifest_incomplete");

    let mut total = 0_u64;
    let mut verified = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        validate_path(&artifact.path)?;
        ensure!(valid_digest(&artifact.digest), "invalid_artifact_digest");
        let path = root.join(&artifact.path);
        reject_symlink_components(&path)?;
        let file = File::open(&path)?;
        let metadata = file.metadata()?;
        ensure!(metadata.file_type().is_file(), "invalid_artifact_file");
        ensure!(metadata.len() <= MAX_ARTIFACT_BYTES, "artifact_too_large");
        total = total
            .checked_add(metadata.len())
            .ok_or_else(|| anyhow::anyhow!("artifact_size_overflow"))?;
        ensure!(total <= MAX_TOTAL_ARTIFACT_BYTES, "artifact_set_too_large");
        let mut bytes = Vec::new();
        file.take(MAX_ARTIFACT_BYTES + 1).read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() as u64 <= MAX_ARTIFACT_BYTES,
            "artifact_too_large"
        );
        ensure!(
            crate::codefriend::ingestion::digest(&bytes) == artifact.digest,
            "artifact_digest_mismatch"
        );
        if let Ok(text) = std::str::from_utf8(&bytes) {
            ensure!(
                !unsafe_content(&artifact.path, text),
                "artifact_redaction_failed"
            );
        }
        verified.push(VerifiedArtifact {
            path: artifact.path.clone(),
            bytes,
        });
    }
    Ok(verified)
}

fn inventory(root: &Path) -> Result<Vec<String>> {
    fn visit(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            ensure!(!file_type.is_symlink(), "artifact_symlink_rejected");
            if file_type.is_dir() {
                visit(root, &entry.path(), out)?;
            } else {
                ensure!(file_type.is_file(), "invalid_artifact_file");
                let relative = entry
                    .path()
                    .strip_prefix(root)
                    .context("artifact_outside_root")?
                    .to_string_lossy()
                    .replace('\\', "/");
                validate_path(&relative)?;
                out.push(relative);
                ensure!(out.len() <= 1000, "too_many_artifacts");
            }
        }
        Ok(())
    }

    let mut out = Vec::new();
    visit(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

pub(crate) fn reject_symlink_components(path: &Path) -> Result<()> {
    ensure!(
        !path
            .components()
            .any(|component| matches!(component, Component::ParentDir)),
        "artifact_parent_traversal_rejected"
    );
    let absolute = std::path::absolute(path)?;
    let mut current = PathBuf::new();
    for component in absolute.components() {
        current.push(component);
        if let Ok(metadata) = fs::symlink_metadata(&current) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "artifact_symlink_rejected"
            );
        }
    }
    Ok(())
}

pub(crate) fn read_json<T: serde::de::DeserializeOwned>(path: &Path, label: &str) -> Result<T> {
    ensure!(
        fs::symlink_metadata(path)?.file_type().is_file(),
        "invalid_{label}_file"
    );
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= MAX_INPUT_BYTES, "{label}_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_{label}_json"))
}
