use super::manifest::{read_json, reject_symlink_components, verify_artifacts};
use crate::codefriend::{
    evidence::{
        contracts::{Approval, Completion, Publication, PublicationState, ReviewRecord},
        hash, valid_digest,
    },
    ingestion::unsafe_content,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const DECISION_SCHEMA: &str = "codefriend.publication_decision.v1";
pub const ADMISSION_SCHEMA: &str = "codefriend.publication_admission.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionKind {
    Approved,
    Withheld,
    Invalidated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DecisionRecord {
    pub schema: String,
    pub publication: Publication,
    pub decision: DecisionKind,
    pub actor: String,
    pub reason: String,
    pub channel: String,
    pub decided_at: u64,
    pub previous_decision_digest: Option<String>,
    pub digest: String,
}

impl DecisionRecord {
    pub fn read(path: &Path, review: &ReviewRecord) -> Result<Self> {
        let record: Self = read_json(path, "publication_decision")?;
        record.validate(review)?;
        Ok(record)
    }

    pub fn new(
        review: &ReviewRecord,
        publication: &Publication,
        decision: DecisionKind,
        actor: &str,
        reason: &str,
        decided_at: u64,
        previous: Option<&DecisionRecord>,
    ) -> Result<Self> {
        publication.validate(review)?;
        if let Some(previous) = previous {
            previous.validate(review)?;
            ensure!(
                previous.publication.binding_digest()? == publication.binding_digest()?,
                "decision_binding_changed"
            );
        }
        ensure!(
            !matches!(decision, DecisionKind::Invalidated) || previous.is_some(),
            "invalidation_requires_prior_decision"
        );
        let mut governed = publication.clone();
        match decision {
            DecisionKind::Approved => {
                ensure!(
                    review.run.completion == Completion::Complete,
                    "approval_requires_complete_run"
                );
                governed.state = PublicationState::Approved;
                governed.approval = Some(Approval {
                    binding_digest: governed.binding_digest()?,
                    actor: actor.trim().to_string(),
                });
            }
            DecisionKind::Withheld => {
                governed.state = PublicationState::Withheld;
                governed.approval = None;
            }
            DecisionKind::Invalidated => {
                governed.state = PublicationState::Invalidated;
                governed.approval = None;
            }
        }
        let mut record = Self {
            schema: DECISION_SCHEMA.to_string(),
            publication: governed,
            decision,
            actor: actor.trim().to_string(),
            reason: reason.trim().to_string(),
            channel: "explicit_cli".to_string(),
            decided_at,
            previous_decision_digest: previous.map(|record| record.digest.clone()),
            digest: String::new(),
        };
        record.digest = record.expected_digest()?;
        record.validate(review)?;
        Ok(record)
    }

    pub fn validate(&self, review: &ReviewRecord) -> Result<()> {
        ensure!(
            self.schema == DECISION_SCHEMA,
            "unsupported_decision_version"
        );
        self.publication.validate(review)?;
        safe_text(&self.actor, "decision_actor")?;
        safe_text(&self.reason, "decision_reason")?;
        ensure!(self.channel == "explicit_cli", "invalid_decision_channel");
        ensure!(self.decided_at > 0, "invalid_decision_time");
        if let Some(previous) = &self.previous_decision_digest {
            ensure!(valid_digest(previous), "invalid_previous_decision_digest");
        }
        let state_matches = matches!(
            (&self.decision, &self.publication.state),
            (DecisionKind::Approved, PublicationState::Approved)
                | (DecisionKind::Withheld, PublicationState::Withheld)
                | (DecisionKind::Invalidated, PublicationState::Invalidated)
        );
        ensure!(state_matches, "decision_state_mismatch");
        ensure!(
            self.digest == self.expected_digest()?,
            "decision_digest_mismatch"
        );
        if self.decision == DecisionKind::Approved {
            ensure!(
                review.run.completion == Completion::Complete,
                "approval_requires_complete_run"
            );
        }
        Ok(())
    }

    fn expected_digest(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.digest.clear();
        hash(&copy)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AdmissionReceipt {
    pub schema: String,
    pub decision_digest: String,
    pub binding_digest: String,
    pub target: String,
    pub admitted_at: u64,
    pub artifact_manifest_digest: String,
    pub digest: String,
}

impl AdmissionReceipt {
    fn expected_digest(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.digest.clear();
        hash(&copy)
    }
}

pub fn admit_local(
    review: &ReviewRecord,
    decision: &DecisionRecord,
    artifact_root: &Path,
    destination_root: &Path,
    admitted_at: u64,
) -> Result<AdmissionReceipt> {
    decision.validate(review)?;
    ensure!(
        decision.decision == DecisionKind::Approved,
        "publication_not_approved"
    );
    ensure!(
        admitted_at >= decision.decided_at,
        "admission_precedes_approval"
    );
    verify_artifacts(artifact_root, &decision.publication.artifact_manifest)?;
    reject_symlink_components(destination_root)?;
    ensure!(
        fs::symlink_metadata(destination_root)?.file_type().is_dir(),
        "invalid_destination_root"
    );
    let target = destination_root.join(&decision.publication.target);
    reject_symlink_components(&target)?;
    ensure!(!target.exists(), "publication_target_exists");
    let mut receipt = AdmissionReceipt {
        schema: ADMISSION_SCHEMA.to_string(),
        decision_digest: decision.digest.clone(),
        binding_digest: decision.publication.binding_digest()?,
        target: decision.publication.target.clone(),
        admitted_at,
        artifact_manifest_digest: decision.publication.manifest_digest.clone(),
        digest: String::new(),
    };
    receipt.digest = receipt.expected_digest()?;
    publish_atomically(artifact_root, &target, decision, &receipt)?;
    Ok(receipt)
}

fn publish_atomically(
    artifact_root: &Path,
    target: &Path,
    decision: &DecisionRecord,
    receipt: &AdmissionReceipt,
) -> Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid_publication_target"))?;
    fs::create_dir_all(parent)?;
    reject_symlink_components(parent)?;
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let stage = loop {
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            ".codefriend-publication-{}-{serial}",
            std::process::id()
        ));
        match fs::create_dir(&candidate) {
            Ok(()) => break candidate,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    };
    let result = (|| -> Result<()> {
        for artifact in &decision.publication.artifact_manifest {
            let source = artifact_root.join(&artifact.path);
            let output = stage.join(&artifact.path);
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
            let bytes = fs::read(source)?;
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(output)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        write_json_create_only(&stage.join("publication-control.json"), decision)?;
        write_json_create_only(&stage.join("publication-admission.json"), receipt)?;
        sync_directories(&stage)?;
        fs::rename(&stage, target)?;
        File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    result
}

fn sync_directories(root: &Path) -> Result<()> {
    let mut directories = vec![root.to_path_buf()];
    for entry in walkdir(root)? {
        if entry.is_dir() {
            directories.push(entry);
        }
    }
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for directory in directories {
        File::open(directory)?.sync_all()?;
    }
    Ok(())
}

fn walkdir(root: &Path) -> Result<Vec<PathBuf>> {
    fn visit(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                out.push(entry.path());
                visit(&entry.path(), out)?;
            }
        }
        Ok(())
    }
    let mut paths = Vec::new();
    visit(root, &mut paths)?;
    Ok(paths)
}

pub fn write_json_create_only<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure!(
            parent.exists() && fs::symlink_metadata(parent)?.is_dir(),
            "output_parent_missing"
        );
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}

fn safe_text(value: &str, label: &str) -> Result<()> {
    ensure!(
        !value.trim().is_empty() && value.len() <= 8192 && !unsafe_content("", value),
        "invalid_{label}"
    );
    Ok(())
}
