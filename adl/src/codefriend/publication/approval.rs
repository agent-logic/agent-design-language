use super::manifest::{
    destination_digest, read_json, reject_symlink_components, snapshot_artifacts, VerifiedArtifact,
};
use crate::codefriend::{
    evidence::{
        contracts::{Approval, Completion, Publication, PublicationState, ReviewRecord},
        hash, valid_digest,
    },
    ingestion::unsafe_content,
};
use anyhow::{ensure, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const DECISION_SCHEMA: &str = "codefriend.publication_decision.v1";
pub const ADMISSION_SCHEMA: &str = "codefriend.publication_admission.v1";
const STORE_SCHEMA: &str = "codefriend.publication_store.v1";
const HEAD_SCHEMA: &str = "codefriend.publication_head.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct StoreMarker {
    schema: String,
    canonical_root: String,
    digest: String,
}

impl StoreMarker {
    fn new(root: &Path) -> Result<Self> {
        let canonical_root = root
            .canonicalize()?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("publication_store_path_not_utf8"))?
            .to_string();
        let mut marker = Self {
            schema: STORE_SCHEMA.into(),
            canonical_root,
            digest: String::new(),
        };
        marker.digest = marker.expected_digest()?;
        Ok(marker)
    }

    fn expected_digest(&self) -> Result<String> {
        hash(&(STORE_SCHEMA, &self.canonical_root))
    }

    fn validate(&self, root: &Path) -> Result<()> {
        ensure!(self.schema == STORE_SCHEMA, "unsupported_publication_store");
        ensure!(
            self == &Self::new(root)?,
            "publication_store_identity_mismatch"
        );
        ensure!(
            self.digest == self.expected_digest()?,
            "publication_store_digest_mismatch"
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct HeadCommitment {
    schema: String,
    store_digest: String,
    binding_digest: String,
    decision_digest: String,
    generation: u64,
    digest: String,
}

impl HeadCommitment {
    fn new(
        marker: &StoreMarker,
        binding_digest: String,
        decision_digest: String,
        generation: u64,
    ) -> Result<Self> {
        let mut head = Self {
            schema: HEAD_SCHEMA.into(),
            store_digest: marker.digest.clone(),
            binding_digest,
            decision_digest,
            generation,
            digest: String::new(),
        };
        head.digest = head.expected_digest()?;
        Ok(head)
    }

    fn expected_digest(&self) -> Result<String> {
        hash(&(
            HEAD_SCHEMA,
            &self.store_digest,
            &self.binding_digest,
            &self.decision_digest,
            self.generation,
        ))
    }

    fn validate(&self, marker: &StoreMarker, binding: &str) -> Result<()> {
        ensure!(self.schema == HEAD_SCHEMA, "unsupported_publication_head");
        ensure!(
            self.store_digest == marker.digest,
            "publication_head_store_mismatch"
        );
        ensure!(
            self.binding_digest == binding,
            "publication_head_binding_mismatch"
        );
        ensure!(self.generation > 0, "invalid_publication_head_generation");
        ensure!(
            valid_digest(&self.decision_digest),
            "invalid_publication_head_decision"
        );
        ensure!(
            self.digest == self.expected_digest()?,
            "publication_head_digest_mismatch"
        );
        Ok(())
    }
}

struct DecisionStore {
    root: PathBuf,
    marker: StoreMarker,
    _lock: File,
}

impl DecisionStore {
    fn open(root: &Path) -> Result<Self> {
        reject_symlink_components(root)?;
        if !root.exists() {
            fs::create_dir_all(root)?;
        }
        ensure!(
            fs::symlink_metadata(root)?.is_dir(),
            "invalid_publication_store_root"
        );
        let marker_path = root.join(".codefriend-publication-store-v1.json");
        if !marker_path.exists() {
            ensure!(
                fs::read_dir(root)?.next().is_none(),
                "unowned_publication_store"
            );
            let marker = StoreMarker::new(root)?;
            write_json_create_only(&marker_path, &marker)?;
            File::open(root)?.sync_all()?;
        }
        let root = root.canonicalize()?;
        let marker: StoreMarker = read_json(&marker_path, "publication_store")?;
        marker.validate(&root)?;
        for directory in ["decisions", "heads"] {
            let path = root.join(directory);
            if !path.exists() {
                fs::create_dir(&path)?;
                File::open(&root)?.sync_all()?;
            }
            ensure!(
                fs::symlink_metadata(&path)?.is_dir(),
                "invalid_publication_store_layout"
            );
            reject_symlink_components(&path)?;
        }
        let lock_path = root.join(".lock");
        let mut options = OpenOptions::new();
        options.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let lock = options.open(lock_path)?;
        lock.try_lock_exclusive()
            .map_err(|_| anyhow::anyhow!("publication_store_busy"))?;
        Ok(Self {
            root,
            marker,
            _lock: lock,
        })
    }

    fn decision_directory(&self, binding: &str) -> Result<PathBuf> {
        ensure!(valid_digest(binding), "invalid_publication_binding");
        Ok(self.root.join("decisions").join(binding))
    }

    fn head_path(&self, binding: &str) -> Result<PathBuf> {
        ensure!(valid_digest(binding), "invalid_publication_binding");
        Ok(self.root.join("heads").join(format!("{binding}.json")))
    }

    fn external_head_path(&self, binding: &str) -> Result<PathBuf> {
        ensure!(valid_digest(binding), "invalid_publication_binding");
        let parent = self
            .root
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid_publication_store_root"))?;
        let anchors = parent
            .join(".codefriend-publication-anchors")
            .join(&self.marker.digest);
        ensure!(
            !anchors.starts_with(&self.root) && !self.root.starts_with(&anchors),
            "publication_anchor_not_external"
        );
        reject_symlink_components(&anchors)?;
        if !anchors.exists() {
            fs::create_dir_all(&anchors)?;
            File::open(parent)?.sync_all()?;
        }
        reject_symlink_components(&anchors)?;
        let anchors = anchors.canonicalize()?;
        ensure!(
            !anchors.starts_with(&self.root) && !self.root.starts_with(&anchors),
            "publication_anchor_not_external"
        );
        ensure!(
            fs::symlink_metadata(&anchors)?.is_dir(),
            "invalid_publication_anchor_root"
        );
        Ok(anchors.join(format!("{binding}.json")))
    }

    fn head(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
    ) -> Result<Option<DecisionRecord>> {
        publication.validate(review)?;
        let binding = publication.binding_digest()?;
        let directory = self.decision_directory(&binding)?;
        let head_path = self.head_path(&binding)?;
        let external_head_path = self.external_head_path(&binding)?;
        if !directory.exists() && !head_path.exists() && !external_head_path.exists() {
            return Ok(None);
        }
        ensure!(
            directory.exists() && head_path.exists() && external_head_path.exists(),
            "publication_store_incomplete"
        );
        let head: HeadCommitment = read_json(&head_path, "publication_head")?;
        head.validate(&self.marker, &binding)?;
        let external_head: HeadCommitment =
            read_json(&external_head_path, "publication_external_head")?;
        external_head.validate(&self.marker, &binding)?;
        ensure!(head == external_head, "publication_external_head_mismatch");
        let record = read_chain_head(&directory, review)?
            .ok_or_else(|| anyhow::anyhow!("publication_decision_missing"))?;
        ensure!(
            record.digest == head.decision_digest,
            "publication_head_replayed"
        );
        ensure!(
            record.publication.binding_digest()? == binding,
            "decision_binding_changed"
        );
        let count = fs::read_dir(&directory)?.count() as u64;
        ensure!(
            count == head.generation,
            "publication_head_generation_mismatch"
        );
        Ok(Some(record))
    }

    fn append(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
        decision: DecisionKind,
        actor: &str,
        reason: &str,
        decided_at: u64,
    ) -> Result<DecisionRecord> {
        publication.validate(review)?;
        let binding = publication.binding_digest()?;
        let directory = self.decision_directory(&binding)?;
        let previous = self.head(review, publication)?;
        if !directory.exists() {
            fs::create_dir(&directory)?;
            File::open(self.root.join("decisions"))?.sync_all()?;
        }
        let record = DecisionRecord::new(
            review,
            publication,
            decision,
            actor,
            reason,
            decided_at,
            previous.as_ref(),
        )?;
        write_json_create_only(&directory.join(format!("{}.json", record.digest)), &record)?;
        File::open(&directory)?.sync_all()?;
        let generation = fs::read_dir(&directory)?.count() as u64;
        let head = HeadCommitment::new(&self.marker, binding, record.digest.clone(), generation)?;
        replace_json_atomically(&self.head_path(&head.binding_digest)?, &head)?;
        replace_json_atomically(&self.external_head_path(&head.binding_digest)?, &head)?;
        ensure!(
            self.head(review, publication)?.as_ref() == Some(&record),
            "decision_head_readback_failed"
        );
        Ok(record)
    }

    fn admit_local<F>(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
        artifact_root: &Path,
        destination_root: &Path,
        admitted_at: u64,
        before_publish: F,
    ) -> Result<AdmissionReceipt>
    where
        F: FnOnce() -> Result<()>,
    {
        let decision = self
            .head(review, publication)?
            .ok_or_else(|| anyhow::anyhow!("publication_decision_missing"))?;
        ensure!(
            decision.decision == DecisionKind::Approved,
            "publication_not_approved"
        );
        ensure!(
            admitted_at >= decision.decided_at,
            "admission_precedes_approval"
        );
        let artifacts = snapshot_artifacts(artifact_root, &decision.publication.artifact_manifest)?;
        reject_symlink_components(destination_root)?;
        ensure!(
            fs::symlink_metadata(destination_root)?.file_type().is_dir(),
            "invalid_destination_root"
        );
        ensure!(
            destination_digest(destination_root)? == decision.publication.destination_digest,
            "publication_destination_mismatch"
        );
        let destination_root = destination_root.canonicalize()?;
        crate::codefriend::ingestion::validate_path(&decision.publication.target)?;
        let target = destination_root.join(&decision.publication.target);
        ensure!(
            target.starts_with(&destination_root),
            "publication_target_outside_destination"
        );
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
        before_publish()?;
        publish_atomically(&artifacts, &target, &decision, &receipt)?;
        Ok(receipt)
    }
}

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

pub fn read_decision_head(
    store_root: &Path,
    review: &ReviewRecord,
    publication: &Publication,
) -> Result<Option<DecisionRecord>> {
    DecisionStore::open(store_root)?.head(review, publication)
}

fn read_chain_head(directory: &Path, review: &ReviewRecord) -> Result<Option<DecisionRecord>> {
    reject_symlink_components(directory)?;
    ensure!(
        fs::symlink_metadata(directory)?.file_type().is_dir(),
        "invalid_decision_directory"
    );
    let mut records = BTreeMap::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        ensure!(records.len() < 1000, "too_many_decisions");
        ensure!(
            entry.file_type()?.is_file() && !entry.file_type()?.is_symlink(),
            "invalid_decision_entry"
        );
        ensure!(
            entry.path().extension().and_then(|value| value.to_str()) == Some("json"),
            "invalid_decision_entry"
        );
        let record = DecisionRecord::read(&entry.path(), review)?;
        ensure!(
            entry.file_name().to_string_lossy() == format!("{}.json", record.digest),
            "decision_filename_mismatch"
        );
        ensure!(
            records.insert(record.digest.clone(), record).is_none(),
            "duplicate_decision"
        );
    }
    if records.is_empty() {
        return Ok(None);
    }
    let binding = records
        .values()
        .next()
        .expect("nonempty")
        .publication
        .binding_digest()?;
    let mut referenced = BTreeSet::new();
    let mut roots = 0_usize;
    for record in records.values() {
        ensure!(
            record.publication.binding_digest()? == binding,
            "decision_directory_mixed_bindings"
        );
        match &record.previous_decision_digest {
            Some(previous) => {
                let prior = records
                    .get(previous)
                    .ok_or_else(|| anyhow::anyhow!("decision_predecessor_missing"))?;
                ensure!(
                    prior.decided_at <= record.decided_at,
                    "decision_time_regressed"
                );
                ensure!(referenced.insert(previous.clone()), "decision_chain_forked");
            }
            None => roots += 1,
        }
    }
    ensure!(roots == 1, "decision_chain_root_invalid");
    let heads = records
        .keys()
        .filter(|digest| !referenced.contains(*digest))
        .collect::<Vec<_>>();
    ensure!(heads.len() == 1, "decision_chain_head_invalid");
    let mut cursor = records.get(heads[0]).expect("known head");
    let mut visited = BTreeSet::new();
    loop {
        ensure!(
            visited.insert(cursor.digest.clone()),
            "decision_chain_cycle"
        );
        let Some(previous) = &cursor.previous_decision_digest else {
            break;
        };
        cursor = records.get(previous).expect("predecessor checked");
    }
    ensure!(
        visited.len() == records.len(),
        "decision_chain_disconnected"
    );
    Ok(Some(records.get(heads[0]).expect("known head").clone()))
}

pub fn append_decision(
    store_root: &Path,
    review: &ReviewRecord,
    publication: &Publication,
    decision: DecisionKind,
    actor: &str,
    reason: &str,
    decided_at: u64,
) -> Result<DecisionRecord> {
    DecisionStore::open(store_root)?.append(
        review,
        publication,
        decision,
        actor,
        reason,
        decided_at,
    )
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
    publication: &Publication,
    store_root: &Path,
    artifact_root: &Path,
    destination_root: &Path,
    admitted_at: u64,
) -> Result<AdmissionReceipt> {
    DecisionStore::open(store_root)?.admit_local(
        review,
        publication,
        artifact_root,
        destination_root,
        admitted_at,
        || Ok(()),
    )
}

fn publish_atomically(
    artifacts: &[VerifiedArtifact],
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
        for artifact in artifacts {
            let output = stage.join(&artifact.path);
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(output)?;
            file.write_all(&artifact.bytes)?;
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

fn replace_json_atomically<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("output_parent_missing"))?;
    ensure!(
        fs::symlink_metadata(parent)?.is_dir(),
        "output_parent_missing"
    );
    static NEXT_HEAD: AtomicU64 = AtomicU64::new(0);
    let pending = parent.join(format!(
        ".codefriend-head-{}-{}",
        std::process::id(),
        NEXT_HEAD.fetch_add(1, Ordering::Relaxed)
    ));
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&pending)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    fs::rename(&pending, path)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn safe_text(value: &str, label: &str) -> Result<()> {
    ensure!(
        !value.trim().is_empty() && value.len() <= 8192 && !unsafe_content("", value),
        "invalid_{label}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codefriend::{
        evidence::contracts::Artifact,
        ingestion::digest,
        publication::manifest::{ManifestInput, MANIFEST_INPUT_SCHEMA},
    };

    #[test]
    fn atomic_publication_uses_the_verified_snapshot_not_a_second_source_read() {
        let target_tmp = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/test-tmp");
        fs::create_dir_all(&target_tmp).unwrap();
        let directory = tempfile::tempdir_in(target_tmp).unwrap();
        let artifact_root = directory.path().join("artifacts");
        let destination = directory.path().join("destination");
        fs::create_dir(&artifact_root).unwrap();
        fs::create_dir(&destination).unwrap();
        let path = artifact_root.join("report.md");
        let approved_bytes = b"approved bytes\n";
        fs::write(&path, approved_bytes).unwrap();
        let review: ReviewRecord = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/codefriend/evidence/review-v1.json"
        ))
        .unwrap();
        let input = ManifestInput {
            schema: MANIFEST_INPUT_SCHEMA.into(),
            artifact_manifest: vec![Artifact {
                path: "report.md".into(),
                digest: digest(approved_bytes),
            }],
            renderer_versions: BTreeMap::from([("markdown".into(), "v1".into())]),
            target: "published".into(),
            claims: vec!["Verified snapshot".into()],
            nonclaims: vec!["No remote publication".into()],
        };
        let publication = input.publication(&review, &destination).unwrap();
        let decision = DecisionRecord::new(
            &review,
            &publication,
            DecisionKind::Approved,
            "operator-fixture",
            "Exact snapshot approved",
            10,
            None,
        )
        .unwrap();
        let snapshot = snapshot_artifacts(&artifact_root, &publication.artifact_manifest).unwrap();
        fs::write(&path, b"mutated after verification\n").unwrap();
        let mut receipt = AdmissionReceipt {
            schema: ADMISSION_SCHEMA.into(),
            decision_digest: decision.digest.clone(),
            binding_digest: publication.binding_digest().unwrap(),
            target: publication.target.clone(),
            admitted_at: 11,
            artifact_manifest_digest: publication.manifest_digest.clone(),
            digest: String::new(),
        };
        receipt.digest = receipt.expected_digest().unwrap();
        publish_atomically(
            &snapshot,
            &destination.join("published"),
            &decision,
            &receipt,
        )
        .unwrap();
        assert_eq!(
            fs::read(destination.join("published/report.md")).unwrap(),
            approved_bytes
        );
    }

    #[test]
    fn admission_holds_the_store_lock_until_publication_is_visible() {
        let target_tmp = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/test-tmp");
        fs::create_dir_all(&target_tmp).unwrap();
        let directory = tempfile::tempdir_in(target_tmp).unwrap();
        let artifact_root = directory.path().join("artifacts");
        fs::create_dir(&artifact_root).unwrap();
        let bytes = b"approved bytes\n";
        fs::write(artifact_root.join("report.md"), bytes).unwrap();
        let review: ReviewRecord = serde_json::from_slice(include_bytes!(
            "../../../tests/fixtures/codefriend/evidence/review-v1.json"
        ))
        .unwrap();
        let input = ManifestInput {
            schema: MANIFEST_INPUT_SCHEMA.into(),
            artifact_manifest: vec![Artifact {
                path: "report.md".into(),
                digest: digest(bytes),
            }],
            renderer_versions: BTreeMap::from([("markdown".into(), "v1".into())]),
            target: "published".into(),
            claims: vec!["Serialized admission".into()],
            nonclaims: vec!["No remote publication".into()],
        };
        for (index, revocation) in [DecisionKind::Invalidated, DecisionKind::Withheld]
            .into_iter()
            .enumerate()
        {
            let store_root = directory.path().join(format!("store-{index}"));
            fs::create_dir(&store_root).unwrap();
            let destination = directory.path().join(format!("destination-{index}"));
            fs::create_dir(&destination).unwrap();
            let publication = input.publication(&review, &destination).unwrap();
            append_decision(
                &store_root,
                &review,
                &publication,
                DecisionKind::Approved,
                "operator-fixture",
                "Exact artifacts accepted",
                10,
            )
            .unwrap();
            let store = DecisionStore::open(&store_root).unwrap();
            store
                .admit_local(
                    &review,
                    &publication,
                    &artifact_root,
                    &destination,
                    11,
                    || {
                        let error = append_decision(
                            &store_root,
                            &review,
                            &publication,
                            revocation.clone(),
                            "operator-fixture",
                            "Revoke during admission",
                            11,
                        )
                        .unwrap_err()
                        .to_string();
                        ensure!(error == "publication_store_busy", "unexpected_lock_result");
                        ensure!(
                            !destination.join("published").exists(),
                            "publication_visible_before_commit"
                        );
                        Ok(())
                    },
                )
                .unwrap();
            assert!(
                destination.join("published").is_dir(),
                "publication_not_visible_after_commit"
            );
            drop(store);

            append_decision(
                &store_root,
                &review,
                &publication,
                revocation,
                "operator-fixture",
                "Revoke after completed admission",
                12,
            )
            .unwrap();
            fs::remove_dir_all(destination.join("published")).unwrap();
            assert!(admit_local(
                &review,
                &publication,
                &store_root,
                &artifact_root,
                &destination,
                13,
            )
            .is_err());
            assert!(!destination.join("published").exists());
        }
    }
}
