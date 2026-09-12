//! Operator-local store: OS lock, create-only atomic admission, content-free tombstones.
use super::{hash, valid_digest, Admission, Retention, VERSION};
use crate::codefriend::ingestion::Packet;
use anyhow::{ensure, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
const LIMIT: u64 = 10 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Tombstone {
    schema: String,
    packet_id: String,
    admission_digest: String,
    deleted_at: u64,
    reason: String,
    digest: String,
}
impl Tombstone {
    fn seal(&mut self) -> Result<()> {
        self.digest.clear();
        self.digest = hash(self)?;
        Ok(())
    }
    fn validate(&self) -> Result<()> {
        let mut t = self.clone();
        t.seal()?;
        ensure!(
            self.schema == VERSION
                && self.digest == t.digest
                && valid_digest(&self.packet_id)
                && valid_digest(&self.admission_digest)
                && ["deleted", "expired"].contains(&self.reason.as_str()),
            "invalid_tombstone"
        );
        Ok(())
    }
}
pub struct Store {
    root: PathBuf,
    _lock: File,
    clock: Box<dyn Fn() -> u64>,
}
fn safe_path(path: &Path) -> Result<()> {
    ensure!(
        !path
            .components()
            .any(|p| matches!(p, std::path::Component::ParentDir)),
        "store_parent_traversal_rejected"
    );
    let absolute = std::path::absolute(path)?;
    let mut current = PathBuf::new();
    for part in absolute.components() {
        current.push(part);
        if let Ok(meta) = fs::symlink_metadata(&current) {
            ensure!(!meta.file_type().is_symlink(), "store_symlink_rejected");
        }
    }
    Ok(())
}
fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    ensure!(
        fs::symlink_metadata(path)?.file_type().is_file(),
        "invalid_store_file"
    );
    let mut bytes = Vec::new();
    File::open(path)?.take(LIMIT + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= LIMIT, "store_record_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_store_record"))
}
fn owner_file(path: &Path) -> Result<File> {
    let mut opts = OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    Ok(opts.open(path)?)
}
// Only a fully synced ownership directory becomes visible at the requested root.
// Interrupted staging directories contain no admitted content and are never trusted.
fn bootstrap(root: &Path) -> Result<()> {
    let permissions = if root.exists() {
        ensure!(fs::symlink_metadata(root)?.is_dir(), "invalid_store_root");
        ensure!(
            fs::read_dir(root)?.next().is_none(),
            "unowned_store_directory"
        );
        Some(fs::metadata(root)?.permissions())
    } else {
        None
    };
    let absolute = std::path::absolute(root)?;
    let parent = absolute
        .parent()
        .ok_or_else(|| anyhow::anyhow!("invalid_store_root"))?;
    fs::create_dir_all(parent)?;
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let stage = loop {
        let serial = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = parent.join(format!(
            ".codefriend-bootstrap-{}-{serial}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => break path,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.into()),
        }
    };
    let result = (|| -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&stage, fs::Permissions::from_mode(0o700))?;
        }
        let lock = owner_file(&stage.join(".lock"))?;
        lock.sync_all()?;
        let mut marker = owner_file(&stage.join(".codefriend-store-v1"))?;
        marker.write_all(b"codefriend-store-v1")?;
        marker.sync_all()?;
        if let Some(permissions) = permissions {
            fs::set_permissions(&stage, permissions)?;
        }
        File::open(&stage)?.sync_all()?;
        // rename cannot replace a nonempty directory: concurrent publication or
        // unrelated contents fail closed instead of acquiring false ownership.
        fs::rename(&stage, &absolute)?;
        File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    result
}

impl Store {
    pub fn open(root: &Path, clock: impl Fn() -> u64 + 'static) -> Result<Self> {
        safe_path(root)?;
        let marker = root.join(".codefriend-store-v1");
        if !marker.exists() {
            bootstrap(root)?;
        }
        ensure!(fs::symlink_metadata(root)?.is_dir(), "invalid_store_root");
        safe_path(&marker)?;
        ensure!(
            fs::read(&marker)? == b"codefriend-store-v1",
            "invalid_store_marker"
        );
        let lockpath = root.join(".lock");
        safe_path(&lockpath)?;
        let mut opts = OpenOptions::new();
        opts.read(true).write(true).create(true).truncate(false);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            opts.mode(0o600);
        }
        let lock = opts.open(lockpath)?;
        lock.try_lock_exclusive()
            .map_err(|_| anyhow::anyhow!("store_busy"))?;
        let store = Self {
            root: root.canonicalize()?,
            _lock: lock,
            clock: Box::new(clock),
        };
        store.recover()?;
        Ok(store)
    }
    fn sync(&self) -> Result<()> {
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
    fn path(&self, id: &str, suffix: &str) -> Result<PathBuf> {
        ensure!(valid_digest(id), "invalid_evidence_packet_id");
        Ok(self.root.join(format!("{id}.{suffix}")))
    }
    fn write<T: Serialize>(&self, id: &str, suffix: &str, value: &T) -> Result<()> {
        let target = self.path(id, suffix)?;
        ensure!(!target.exists(), "immutable_record_collision");
        let bytes = serde_json::to_vec(value)?;
        ensure!(bytes.len() as u64 <= LIMIT, "store_record_too_large");
        let pending = self.root.join(format!("{id}.{suffix}.pending"));
        let mut file = owner_file(&pending)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        fs::hard_link(&pending, &target)?;
        self.sync()?;
        fs::remove_file(pending)?;
        self.sync()
    }
    fn recover(&self) -> Result<()> {
        let mut active = Vec::new();
        for item in fs::read_dir(&self.root)? {
            let item = item?;
            let name = item
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("invalid_store_name"))?;
            ensure!(item.file_type()?.is_file(), "invalid_store_file");
            if name == ".lock" || name == ".codefriend-store-v1" {
                continue;
            }
            if name.ends_with(".pending") {
                let stem = name.split('.').next().unwrap_or("");
                ensure!(valid_digest(stem), "invalid_store_name");
                fs::remove_file(item.path())?;
                continue;
            }
            if let Some(id) = name.strip_suffix(".tombstone") {
                let t: Tombstone = read(&item.path())?;
                t.validate()?;
                ensure!(t.packet_id == id, "tombstone_identity_mismatch");
                let record = self.path(id, "json")?;
                if record.exists() {
                    ensure!(
                        fs::symlink_metadata(&record)?.file_type().is_file(),
                        "invalid_store_file"
                    );
                    fs::remove_file(record)?;
                }
            } else if let Some(id) = name.strip_suffix(".anchor") {
                ensure!(valid_digest(id), "invalid_store_name");
                let anchor: String = read(&item.path())?;
                ensure!(valid_digest(&anchor), "invalid_admission_anchor");
                if !self.path(id, "json")?.exists() && !self.path(id, "tombstone")?.exists() {
                    fs::remove_file(item.path())?;
                }
            } else if let Some(id) = name.strip_suffix(".json") {
                ensure!(valid_digest(id), "invalid_store_name");
                active.push(id.to_string());
            } else {
                anyhow::bail!("invalid_store_name");
            }
        }
        self.sync()?;
        for id in active {
            if self.path(&id, "tombstone")?.exists() {
                continue;
            }
            let a: Admission = read(&self.path(&id, "json")?)?;
            a.validate()?;
            let anchor: String = read(&self.path(&a.packet.packet_id, "anchor")?)?;
            ensure!(anchor == a.digest, "admission_anchor_mismatch");
            ensure!(a.packet.packet_id == id, "admission_identity_mismatch");
            if (self.clock)() >= a.expires_at {
                self.remove(&id, "expired")?;
            }
        }
        Ok(())
    }
    pub fn admit(&self, packet: Packet, retention: Retention) -> Result<Admission> {
        // All validation, including credential scanning, precedes even temporary content writes.
        let candidate = Admission::new(packet, retention, (self.clock)())?;
        let id = &candidate.packet.packet_id;
        ensure!(!self.path(id, "tombstone")?.exists(), "evidence_deleted");
        if self.path(id, "json")?.exists() {
            let old = self.get(id)?;
            ensure!(
                old.packet == candidate.packet && old.retention == candidate.retention,
                "immutable_admission_collision"
            );
            return Ok(old);
        }
        self.write(id, "anchor", &candidate.digest)?;
        self.write(id, "json", &candidate)?;
        self.get(id)
    }
    pub fn get(&self, id: &str) -> Result<Admission> {
        ensure!(!self.path(id, "tombstone")?.exists(), "evidence_deleted");
        let a: Admission = read(&self.path(id, "json")?)?;
        a.validate()?;
        let anchor: String = read(&self.path(&a.packet.packet_id, "anchor")?)?;
        ensure!(anchor == a.digest, "admission_anchor_mismatch");
        ensure!(a.packet.packet_id == id, "admission_identity_mismatch");
        if (self.clock)() >= a.expires_at {
            self.remove(id, "expired")?;
            anyhow::bail!("evidence_expired");
        }
        Ok(a)
    }
    fn remove(&self, id: &str, reason: &str) -> Result<()> {
        if self.path(id, "tombstone")?.exists() {
            return Ok(());
        }
        let a: Admission = read(&self.path(id, "json")?)?;
        a.validate()?;
        let anchor: String = read(&self.path(&a.packet.packet_id, "anchor")?)?;
        ensure!(anchor == a.digest, "admission_anchor_mismatch");
        ensure!(a.packet.packet_id == id, "admission_identity_mismatch");
        let mut t = Tombstone {
            schema: VERSION.into(),
            packet_id: id.into(),
            admission_digest: a.digest,
            deleted_at: (self.clock)(),
            reason: reason.into(),
            digest: String::new(),
        };
        t.seal()?;
        self.write(id, "tombstone", &t)?;
        fs::remove_file(self.path(id, "json")?)?;
        self.sync()
    }
    pub fn delete(&self, id: &str) -> Result<()> {
        self.remove(id, "deleted")
    }
}
