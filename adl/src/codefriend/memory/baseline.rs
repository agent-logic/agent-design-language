//! Immutable bounded review references backed by live evidence admission.
use crate::codefriend::evidence::{
    contracts::{Finding, ReviewRecord, Run},
    hash,
    store::Store,
    valid_digest,
};
use anyhow::{ensure, Result};
use fs2::FileExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};
const VERSION: &str = "codefriend.baseline.v1";
const LIMIT: u64 = 16 * 1024 * 1024;
const MARKER: &str = ".codefriend-baselines-v1";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BaselineRef {
    pub run_id: String,
    pub packet_id: String,
    pub record_digest: String,
}
impl BaselineRef {
    pub fn from_record(record: &ReviewRecord) -> Result<Self> {
        record.validate()?;
        let mut findings = record.findings.clone();
        findings.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self {
            run_id: record.run.id.clone(),
            packet_id: record.run.packet_id.clone(),
            record_digest: hash(&(VERSION, &record.run, &findings))?,
        })
    }
    /// Stable retained-result identity; existing reference wire bytes are unchanged.
    pub fn storage_identity(&self) -> Result<String> {
        self.validate()?;
        hash(&("codefriend.baseline.storage.v2", self))
    }
    pub fn validate(&self) -> Result<()> {
        ensure!(
            valid_digest(&self.run_id)
                && valid_digest(&self.packet_id)
                && valid_digest(&self.record_digest),
            "invalid_baseline_reference"
        );
        Ok(())
    }
}
/// Trusted adapter contract: implementations must enforce live retention and deletion.
/// The matcher independently validates identity, not the backend's external state.
pub trait BaselineAccess {
    fn load(&self, reference: &BaselineRef) -> Result<ReviewRecord>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Retained {
    schema: String,
    run: Run,
    findings: Vec<Finding>,
    digest: String,
}
impl Retained {
    fn expected_digest(&self) -> Result<String> {
        hash(&(&self.schema, &self.run, &self.findings))
    }
}
/// The adapter stores no copy of source content. Every load requires live admission.
pub struct AdmittedBaselines<'a> {
    store: &'a Store,
    root: PathBuf,
    _lock: File,
}
pub fn safe_path(path: &Path) -> Result<()> {
    let absolute = std::path::absolute(path)?;
    ensure!(
        !absolute
            .components()
            .any(|c| matches!(c, Component::ParentDir)),
        "parent_path_rejected"
    );
    let mut p = PathBuf::new();
    for c in absolute.components() {
        p.push(c);
        if let Ok(m) = fs::symlink_metadata(&p) {
            ensure!(!m.file_type().is_symlink(), "symlink_path_rejected");
        }
    }
    Ok(())
}
pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    safe_path(path)?;
    ensure!(
        fs::symlink_metadata(path)?.file_type().is_file(),
        "regular_file_required"
    );
    let mut bytes = Vec::new();
    File::open(path)?.take(LIMIT + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= LIMIT, "memory_artifact_byte_limit");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_memory_artifact"))
}
pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    safe_path(path)?;
    let bytes = serde_json::to_vec(value)?;
    ensure!(bytes.len() as u64 <= LIMIT, "memory_artifact_byte_limit");
    let mut o = OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    let mut f = o.open(path)?;
    f.write_all(&bytes)?;
    f.sync_all()?;
    File::open(
        path.parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new(".")),
    )?
    .sync_all()?;
    Ok(())
}
impl<'a> AdmittedBaselines<'a> {
    pub fn open(store: &'a Store, root: &Path, create: bool) -> Result<Self> {
        safe_path(root)?;
        let absolute = std::path::absolute(root)?;
        ensure!(
            !absolute.starts_with(store.root_path()) && !store.root_path().starts_with(&absolute),
            "baseline_and_admission_roots_must_be_separate"
        );
        if !root.exists() {
            ensure!(create, "baseline_store_missing");
            let mut builder = fs::DirBuilder::new();
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                builder.mode(0o700);
            }
            builder.create(root)?;
            write_json(&root.join(MARKER), &VERSION)?;
        }
        ensure!(
            fs::symlink_metadata(root)?.is_dir(),
            "invalid_baseline_root"
        );
        let marker: String = read_json(&root.join(MARKER))?;
        ensure!(marker == VERSION, "invalid_baseline_marker");
        let lockpath = root.join(".lock");
        safe_path(&lockpath)?;
        let mut o = OpenOptions::new();
        o.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            o.mode(0o600);
        }
        let lock = o.open(lockpath)?;
        lock.try_lock_exclusive()
            .map_err(|_| anyhow::anyhow!("baseline_store_busy"))?;
        let result = Self {
            store,
            root: root.to_path_buf(),
            _lock: lock,
        };
        result.count()?;
        Ok(result)
    }
    fn count(&self) -> Result<usize> {
        let mut ids = std::collections::BTreeSet::new();
        for entry in fs::read_dir(&self.root)? {
            let e = entry?;
            ensure!(e.file_type()?.is_file(), "invalid_baseline_entry");
            let n = e.file_name();
            let n = n
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("invalid_baseline_name"))?;
            if n == MARKER || n == ".lock" {
                continue;
            }
            let id = n
                .strip_suffix(".json")
                .or_else(|| n.strip_suffix(".deleted"));
            ensure!(
                id.is_some_and(|id| valid_digest(id.strip_prefix("v2-").unwrap_or(id))),
                "invalid_baseline_name"
            );
            ids.insert(id.unwrap().to_string());
            ensure!(ids.len() <= 128, "baseline_count_limit");
        }
        Ok(ids.len())
    }
    pub fn retain(&self, record: &ReviewRecord) -> Result<BaselineRef> {
        record.validate()?;
        ensure!(
            self.store.get(&record.run.packet_id)? == record.admission,
            "baseline_admission_mismatch"
        );
        let mut r = Retained {
            schema: VERSION.into(),
            run: record.run.clone(),
            findings: record.findings.clone(),
            digest: String::new(),
        };
        r.findings.sort_by(|a, b| a.id.cmp(&b.id));
        r.digest = r.expected_digest()?;
        let reference = BaselineRef {
            run_id: r.run.id.clone(),
            packet_id: r.run.packet_id.clone(),
            record_digest: r.digest.clone(),
        };
        self.ensure_not_deleted(&reference)?;
        if let Some(old) = self.legacy(&reference)? {
            if old.digest == reference.record_digest {
                self.load(&reference)?;
                return Ok(reference);
            }
        }
        let path = self.path(&reference)?;
        if path.exists() {
            self.load(&reference)?;
            return Ok(reference);
        }
        ensure!(self.count()? < 128, "baseline_count_limit");
        write_json(&path, &r)?;
        self.load(&reference)?;
        Ok(reference)
    }
    fn path(&self, r: &BaselineRef) -> Result<PathBuf> {
        r.validate()?;
        Ok(self.root.join(format!("v2-{}.json", r.storage_identity()?)))
    }
    fn deleted(&self, r: &BaselineRef) -> Result<PathBuf> {
        r.validate()?;
        Ok(self
            .root
            .join(format!("v2-{}.deleted", r.storage_identity()?)))
    }
    fn ensure_not_deleted(&self, r: &BaselineRef) -> Result<()> {
        r.validate()?;
        ensure!(
            !self.root.join(format!("{}.deleted", r.run_id)).exists() && !self.deleted(r)?.exists(),
            "baseline_deleted"
        );
        Ok(())
    }
    // Authenticate any legacy record before considering a different tuple. Corrupt
    // old storage must never become a silent fallback to a newer namespace.
    fn legacy(&self, reference: &BaselineRef) -> Result<Option<Retained>> {
        reference.validate()?;
        let path = self.root.join(format!("{}.json", reference.run_id));
        if !path.exists() {
            return Ok(None);
        }
        let r: Retained = read_json(&path)?;
        ensure!(
            r.schema == VERSION
                && r.digest == r.expected_digest()?
                && r.run.id == reference.run_id
                && r.run.packet_id == reference.packet_id,
            "baseline_identity_or_digest_mismatch"
        );
        let record = ReviewRecord {
            admission: self.store.get(&reference.packet_id)?,
            run: r.run.clone(),
            findings: r.findings.clone(),
        };
        record.validate()?;
        Ok(Some(r))
    }
    fn retained(&self, reference: &BaselineRef) -> Result<Retained> {
        self.ensure_not_deleted(reference)?;
        let legacy = self.legacy(reference)?;
        let path = self.path(reference)?;
        let r: Retained = if path.exists() {
            read_json(&path)?
        } else {
            legacy.ok_or_else(|| anyhow::anyhow!("baseline_missing_or_deleted"))?
        };
        ensure!(
            r.schema == VERSION
                && r.digest == r.expected_digest()?
                && r.digest == reference.record_digest
                && r.run.id == reference.run_id
                && r.run.packet_id == reference.packet_id,
            "baseline_identity_or_digest_mismatch"
        );
        Ok(r)
    }
    pub fn delete(&self, r: &BaselineRef) -> Result<()> {
        r.validate()?;
        let legacy_path = self.root.join(format!("{}.json", r.run_id));
        let legacy_tombstone = self.root.join(format!("{}.deleted", r.run_id));
        let legacy_selected = !self.path(r)?.exists()
            && (legacy_tombstone.exists()
                || self
                    .legacy(r)?
                    .is_some_and(|old| old.digest == r.record_digest));
        let tombstone = if legacy_selected {
            legacy_tombstone
        } else {
            self.deleted(r)?
        };
        if tombstone.exists() {
            let old: BaselineRef = read_json(&tombstone)?;
            ensure!(&old == r, "baseline_deletion_mismatch");
        } else {
            self.retained(r)?;
            write_json(&tombstone, r)?;
        }
        let path = if legacy_selected {
            legacy_path
        } else {
            self.path(r)?
        };
        if path.exists() {
            safe_path(&path)?;
            fs::remove_file(path)?;
        }
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
}
impl BaselineAccess for AdmittedBaselines<'_> {
    fn load(&self, reference: &BaselineRef) -> Result<ReviewRecord> {
        let r = self.retained(reference)?;
        let record = ReviewRecord {
            admission: self.store.get(&reference.packet_id)?,
            run: r.run,
            findings: r.findings,
        };
        record.validate()?;
        Ok(record)
    }
}
