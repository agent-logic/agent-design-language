use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ErrorCode, IssueRecord, Result, Store, V2Error};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum ProjectionCasAnchor {
    VerifiedCanonical {
        generation: u64,
        record_digest: String,
    },
    ExpectedCanonicalAbsent {
        backup_generation: u64,
        backup_record_digest: String,
    },
    ExactObservedInvalid {
        canonical_identity: NodeIdentity,
        manifest_digest: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NodeIdentity {
    pub device: u64,
    pub mount_id: String,
    pub inode: u64,
    pub ctime_seconds: i64,
    pub ctime_nanoseconds: i64,
    pub links: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub node_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    pub path: String,
    pub identity: NodeIdentity,
    pub size: u64,
    pub digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CandidateObservation {
    pub name: String,
    pub state: String,
    pub valid_projection: bool,
    pub generation: Option<u64>,
    pub record_digest: Option<String>,
    pub manifest_digest: Option<String>,
    pub entries: Vec<ManifestEntry>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionClassifyRequest {
    pub issue: u64,
    pub anchor: ProjectionCasAnchor,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionClassification {
    pub schema: String,
    pub issue: u64,
    pub actor: String,
    pub reason: String,
    pub canonical: CandidateObservation,
    pub backup: CandidateObservation,
    pub preserved: CandidateObservation,
    pub disposition: String,
    pub receipt_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionRecoverRequest {
    pub issue: u64,
    pub operation_id: String,
    pub classify_receipt_digest: String,
    pub anchor: ProjectionCasAnchor,
    pub actor: String,
    pub reason: String,
    pub branch: String,
    pub worktree: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionRecoveryResult {
    pub schema: String,
    pub issue: u64,
    pub operation_id: String,
    pub disposition: String,
    pub archived_manifest_digest: String,
    pub canonical_generation: u64,
    pub canonical_digest: String,
    pub receipt_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionCleanupRequest {
    pub issue: u64,
    pub recovery_operation_id: String,
    pub cleanup_operation_id: String,
    pub recovery_receipt_digest: String,
    pub archived_manifest_digest: String,
    pub anchor: ProjectionCasAnchor,
    pub actor: String,
    pub reason: String,
    pub branch: String,
    pub worktree: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionCleanupResult {
    pub schema: String,
    pub issue: u64,
    pub cleanup_operation_id: String,
    pub disposition: String,
    pub receipt_digest: String,
}

fn issue_parent(store: &Store) -> PathBuf {
    store.root().join(".csdlc/issues")
}
fn recovery_root(store: &Store, issue: u64) -> PathBuf {
    issue_parent(store).join(format!(".{issue}.recovery"))
}

fn validate_operation_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 96
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "operation id must be a bounded safe component",
        ));
    }
    Ok(())
}

fn mount_id(path: &Path, metadata: &fs::Metadata) -> Result<String> {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    unsafe {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        let c = CString::new(bytes)
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path contains NUL"))?;
        let mut stat: libc::statfs = std::mem::zeroed();
        if libc::statfs(c.as_ptr(), &mut stat) != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let words: [i32; 2] = std::mem::transmute_copy(&stat.f_fsid);
        Ok(format!("{}:{}:{}", metadata.dev(), words[0], words[1]))
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    {
        Ok(format!("dev:{}", metadata.dev()))
    }
}

fn identity(path: &Path, metadata: &fs::Metadata) -> Result<NodeIdentity> {
    let ft = metadata.file_type();
    let node_type = if ft.is_dir() {
        "directory"
    } else if ft.is_file() {
        "regular"
    } else if ft.is_symlink() {
        "symlink"
    } else {
        "special"
    };
    Ok(NodeIdentity {
        device: metadata.dev(),
        mount_id: mount_id(path, metadata)?,
        inode: metadata.ino(),
        ctime_seconds: metadata.ctime(),
        ctime_nanoseconds: metadata.ctime_nsec(),
        links: metadata.nlink(),
        uid: metadata.uid(),
        gid: metadata.gid(),
        mode: metadata.mode(),
        node_type: node_type.into(),
    })
}

fn walk(path: &Path) -> Result<Vec<ManifestEntry>> {
    let root_meta = fs::symlink_metadata(path)?;
    if !root_meta.is_dir() || root_meta.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "candidate root is not a no-follow directory",
        ));
    }
    let root_id = identity(path, &root_meta)?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    fn visit(
        root: &Path,
        path: &Path,
        root_id: &NodeIdentity,
        seen: &mut BTreeSet<(String, u64, u64)>,
        out: &mut Vec<ManifestEntry>,
    ) -> Result<()> {
        let meta = fs::symlink_metadata(path)?;
        let id = identity(path, &meta)?;
        if id.node_type == "symlink" || id.node_type == "special" {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "projection contains symlink or special node",
            ));
        }
        if id.device != root_id.device || id.mount_id != root_id.mount_id {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "projection crosses filesystem mount",
            ));
        }
        if id.uid != root_id.uid || id.gid != root_id.gid || id.mode & 0o022 != 0 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "projection node ownership or mode is unsafe",
            ));
        }
        if !seen.insert((id.mount_id.clone(), id.device, id.inode)) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "projection contains repeated inode identity",
            ));
        }
        if id.node_type == "regular" && id.links != 1 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "projection regular file is hardlinked",
            ));
        }
        let rel = path.strip_prefix(root).unwrap_or(Path::new(""));
        let rel = if rel.as_os_str().is_empty() {
            ".".into()
        } else {
            rel.to_str()
                .ok_or_else(|| {
                    V2Error::new(ErrorCode::InvalidInput, "projection path must be UTF-8")
                })?
                .into()
        };
        let digest = if meta.is_file() {
            let mut b = Vec::new();
            File::open(path)?.read_to_end(&mut b)?;
            Some(blake3::hash(&b).to_hex().to_string())
        } else {
            None
        };
        out.push(ManifestEntry {
            path: rel,
            identity: id.clone(),
            size: meta.len(),
            digest,
        });
        if meta.is_dir() {
            let mut children = fs::read_dir(path)?.collect::<std::io::Result<Vec<_>>>()?;
            children.sort_by_key(|e| e.file_name());
            for child in children {
                visit(root, &child.path(), root_id, seen, out)?;
            }
        }
        Ok(())
    }
    visit(path, path, &root_id, &mut seen, &mut out)?;
    Ok(out)
}

fn observe(path: &Path, name: &str, issue: u64) -> CandidateObservation {
    let absent = CandidateObservation {
        name: name.into(),
        state: "absent".into(),
        valid_projection: false,
        generation: None,
        record_digest: None,
        manifest_digest: None,
        entries: vec![],
        error: None,
    };
    let meta = match fs::symlink_metadata(path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return absent,
        Err(e) => {
            return CandidateObservation {
                error: Some(e.to_string()),
                state: "unsafe".into(),
                ..absent
            }
        }
    };
    if meta.file_type().is_symlink() || !meta.is_dir() {
        return CandidateObservation {
            state: "unsafe".into(),
            error: Some("candidate is symlink or non-directory".into()),
            ..absent
        };
    }
    let entries = match walk(path) {
        Ok(v) => v,
        Err(e) => {
            return CandidateObservation {
                state: "unsafe".into(),
                error: Some(e.message),
                ..absent
            }
        }
    };
    let manifest_digest = blake3::hash(&serde_json::to_vec(&entries).unwrap_or_default())
        .to_hex()
        .to_string();
    let record = fs::read(path.join("index.json"))
        .ok()
        .and_then(|b| serde_json::from_slice::<IssueRecord>(&b).ok());
    let valid = record.as_ref().is_some_and(|r| r.issue == issue);
    CandidateObservation {
        name: name.into(),
        state: if valid { "verified" } else { "invalid" }.into(),
        valid_projection: valid,
        generation: record.as_ref().map(|r| r.generation),
        record_digest: record.as_ref().map(|r| r.digest.clone()),
        manifest_digest: Some(manifest_digest),
        entries,
        error: if valid {
            None
        } else {
            Some("candidate record absent, corrupt, or namespace-mismatched".into())
        },
    }
}

fn verify_topology(store: &Store, branch: &str, worktree: &str) -> Result<()> {
    let root = fs::canonicalize(store.root())?;
    if root != fs::canonicalize(worktree)? || crate::git::current_branch(store.root())? != branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "recovery topology does not match registered worktree and branch",
        ));
    }
    Ok(())
}

fn check_anchor(
    anchor: &ProjectionCasAnchor,
    canonical: &CandidateObservation,
    backup: &CandidateObservation,
) -> Result<()> {
    let ok = match anchor {
        ProjectionCasAnchor::VerifiedCanonical {
            generation,
            record_digest,
        } => {
            canonical.valid_projection
                && canonical.generation == Some(*generation)
                && canonical.record_digest.as_deref() == Some(record_digest)
        }
        ProjectionCasAnchor::ExpectedCanonicalAbsent {
            backup_generation,
            backup_record_digest,
        } => {
            canonical.state == "absent"
                && backup.valid_projection
                && backup.generation == Some(*backup_generation)
                && backup.record_digest.as_deref() == Some(backup_record_digest)
        }
        ProjectionCasAnchor::ExactObservedInvalid {
            canonical_identity,
            manifest_digest,
        } => {
            !canonical.valid_projection
                && canonical
                    .entries
                    .first()
                    .is_some_and(|e| &e.identity == canonical_identity)
                && canonical.manifest_digest.as_deref() == Some(manifest_digest)
        }
    };
    if ok {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "projection recovery CAS anchor is stale",
        ))
    }
}

pub fn classify_preserved_projection(
    store: &Store,
    request: ProjectionClassifyRequest,
) -> Result<ProjectionClassification> {
    let _lock = store.authority_projection_lock(request.issue)?;
    let canonical = observe(&store.issue_dir(request.issue), "canonical", request.issue);
    let backup = observe(
        &store.interrupted_backup(request.issue),
        "backup",
        request.issue,
    );
    let preserved = observe(
        &store.rollback_preserved(request.issue),
        "preserved",
        request.issue,
    );
    check_anchor(&request.anchor, &canonical, &backup)?;
    let unsafe_any = [&canonical, &backup, &preserved]
        .iter()
        .any(|c| c.state == "unsafe");
    let disposition = if unsafe_any {
        "unsafe"
    } else if preserved.state == "absent" && backup.state == "absent" {
        "clean"
    } else if preserved.state != "absent" && (canonical.valid_projection || backup.valid_projection)
    {
        "recoverable"
    } else {
        "ambiguous"
    };
    let mut result = ProjectionClassification {
        schema: "csdlc.projection_classification.v1".into(),
        issue: request.issue,
        actor: request.actor,
        reason: request.reason,
        canonical,
        backup,
        preserved,
        disposition: disposition.into(),
        receipt_digest: String::new(),
    };
    result.receipt_digest = blake3::hash(&serde_json::to_vec(&result)?)
        .to_hex()
        .to_string();
    Ok(result)
}

fn private_dir(path: &Path) -> Result<()> {
    fs::create_dir(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    sync(path.parent().unwrap())
}
fn sync(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}
fn receipt(dir: &Path, seq: u32, state: &str, value: &serde_json::Value) -> Result<String> {
    let p = dir.join(format!("{seq:03}-{state}.json"));
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&p)?;
    f.write_all(&bytes)?;
    f.sync_all()?;
    sync(dir)?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn same_identity(path: &Path, expected: &NodeIdentity) -> Result<bool> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    Ok(identity(path, &metadata)? == *expected)
}

fn remove_manifest_tree(root: &Path, entries: &[ManifestEntry], ledger: &Path) -> Result<()> {
    let mut ordered = entries.to_vec();
    ordered.sort_by_key(|entry| std::cmp::Reverse(entry.path.matches('/').count()));
    for (index, entry) in ordered.iter().enumerate() {
        let path = if entry.path == "." {
            root.to_path_buf()
        } else {
            root.join(&entry.path)
        };
        receipt(
            ledger,
            100 + (index as u32 * 2),
            "node-delete-intent",
            &serde_json::to_value(entry)?,
        )?;
        if !same_identity(&path, &entry.identity)? {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "cleanup node identity changed before removal",
            ));
        }
        if entry.identity.node_type == "regular" {
            fs::remove_file(&path)?;
        } else {
            fs::remove_dir(&path)?;
        }
        sync(
            path.parent().ok_or_else(|| {
                V2Error::new(ErrorCode::InvalidInput, "cleanup node has no parent")
            })?,
        )?;
        receipt(
            ledger,
            101 + (index as u32 * 2),
            "node-removed",
            &serde_json::json!({"path":entry.path,"identity":entry.identity}),
        )?;
    }
    Ok(())
}

pub fn recover_preserved_projection(
    store: &Store,
    request: ProjectionRecoverRequest,
) -> Result<ProjectionRecoveryResult> {
    validate_operation_id(&request.operation_id)?;
    verify_topology(store, &request.branch, &request.worktree)?;
    let _lock = store.authority_projection_lock(request.issue)?;
    let classification = classify_preserved_projection_unlocked(store, &request)?;
    if classification.receipt_digest != request.classify_receipt_digest
        || classification.disposition != "recoverable"
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "classification receipt is stale or not recoverable",
        ));
    }
    let root = recovery_root(store, request.issue);
    if !root.exists() {
        private_dir(&root)?;
    }
    let attempt = root.join(&request.operation_id);
    if attempt.exists() {
        let done = attempt.join("009-recovered.json");
        if done.is_file() {
            return Ok(serde_json::from_slice(&fs::read(done)?)?);
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "incomplete recovery attempt requires deterministic resume",
        ));
    }
    private_dir(&attempt)?;
    receipt(
        &attempt,
        1,
        "prepared",
        &serde_json::to_value(&classification)?,
    )?;
    let preserved = store.rollback_preserved(request.issue);
    let archive = attempt.join("rejected-projection");
    receipt(
        &attempt,
        2,
        "archive-intent",
        &serde_json::json!({"source":classification.preserved,"destination":"rejected-projection"}),
    )?;
    if archive.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "archive collision",
        ));
    }
    fs::rename(&preserved, &archive)?;
    sync(&root)?;
    sync(&attempt)?;
    let archived = observe(&archive, "archive", request.issue);
    receipt(
        &attempt,
        3,
        "rejected-archived",
        &serde_json::to_value(&archived)?,
    )?;
    let before = store.load_record(request.issue)?;
    let prior_generation = before.generation;
    let prior_digest = before.digest.clone();
    let record=store.append_projection_recovery_audit_locked(request.issue,&prior_digest,request.actor.clone(),request.reason.clone(),serde_json::json!({"operation":"recover_preserved_projection","operation_id":request.operation_id,"classify_receipt":request.classify_receipt_digest,"archived_manifest":archived.manifest_digest,"prior_generation":prior_generation,"prior_digest":prior_digest}).to_string())?;
    let canonical = observe(&store.issue_dir(request.issue), "canonical", request.issue);
    receipt(
        &attempt,
        8,
        "canonical-verified",
        &serde_json::to_value(&canonical)?,
    )?;
    let mut out = ProjectionRecoveryResult {
        schema: "csdlc.projection_recovery_result.v1".into(),
        issue: request.issue,
        operation_id: request.operation_id,
        disposition: "recovered".into(),
        archived_manifest_digest: archived.manifest_digest.unwrap_or_default(),
        canonical_generation: record.generation,
        canonical_digest: record.digest,
        receipt_digest: String::new(),
    };
    out.receipt_digest = blake3::hash(&serde_json::to_vec(&out)?)
        .to_hex()
        .to_string();
    receipt(&attempt, 9, "recovered", &serde_json::to_value(&out)?)?;
    Ok(out)
}

fn classify_preserved_projection_unlocked(
    store: &Store,
    request: &ProjectionRecoverRequest,
) -> Result<ProjectionClassification> {
    let canonical = observe(&store.issue_dir(request.issue), "canonical", request.issue);
    let backup = observe(
        &store.interrupted_backup(request.issue),
        "backup",
        request.issue,
    );
    let preserved = observe(
        &store.rollback_preserved(request.issue),
        "preserved",
        request.issue,
    );
    check_anchor(&request.anchor, &canonical, &backup)?;
    let disposition =
        if preserved.state != "absent" && (canonical.valid_projection || backup.valid_projection) {
            "recoverable"
        } else {
            "ambiguous"
        };
    let mut v = ProjectionClassification {
        schema: "csdlc.projection_classification.v1".into(),
        issue: request.issue,
        actor: request.actor.clone(),
        reason: request.reason.clone(),
        canonical,
        backup,
        preserved,
        disposition: disposition.into(),
        receipt_digest: String::new(),
    };
    v.receipt_digest = blake3::hash(&serde_json::to_vec(&v)?).to_hex().to_string();
    Ok(v)
}

pub fn cleanup_preserved_projection(
    store: &Store,
    request: ProjectionCleanupRequest,
) -> Result<ProjectionCleanupResult> {
    validate_operation_id(&request.cleanup_operation_id)?;
    verify_topology(store, &request.branch, &request.worktree)?;
    let _lock = store.authority_projection_lock(request.issue)?;
    let canonical = observe(&store.issue_dir(request.issue), "canonical", request.issue);
    check_anchor(
        &request.anchor,
        &canonical,
        &observe(
            &store.interrupted_backup(request.issue),
            "backup",
            request.issue,
        ),
    )?;
    let attempt = recovery_root(store, request.issue).join(&request.recovery_operation_id);
    let recovered: ProjectionRecoveryResult =
        serde_json::from_slice(&fs::read(attempt.join("009-recovered.json"))?)?;
    if recovered.receipt_digest != request.recovery_receipt_digest {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "recovery receipt is stale",
        ));
    }
    let archive = attempt.join("rejected-projection");
    let observed = observe(&archive, "archive", request.issue);
    if observed.manifest_digest.as_deref() != Some(&request.archived_manifest_digest) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "archived projection manifest changed",
        ));
    }
    let cleanup = attempt.join(format!("cleanup-{}", request.cleanup_operation_id));
    private_dir(&cleanup)?;
    receipt(
        &cleanup,
        1,
        "cleanup-prepared",
        &serde_json::to_value(&observed)?,
    )?;
    let captured = cleanup.join("captured");
    fs::rename(&archive, &captured)?;
    sync(&attempt)?;
    sync(&cleanup)?;
    let after = observe(&captured, "captured", request.issue);
    if after.manifest_digest != observed.manifest_digest {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "captured projection identity changed",
        ));
    }
    receipt(
        &cleanup,
        2,
        "cleanup-captured",
        &serde_json::to_value(&after)?,
    )?;
    remove_manifest_tree(&captured, &after.entries, &cleanup)?;
    sync(&cleanup)?;
    receipt(
        &cleanup,
        3,
        "cleanup-removed",
        &serde_json::json!({"manifest":request.archived_manifest_digest}),
    )?;
    let mut out = ProjectionCleanupResult {
        schema: "csdlc.projection_cleanup_result.v1".into(),
        issue: request.issue,
        cleanup_operation_id: request.cleanup_operation_id,
        disposition: "cleaned".into(),
        receipt_digest: String::new(),
    };
    out.receipt_digest = blake3::hash(&serde_json::to_vec(&out)?)
        .to_hex()
        .to_string();
    receipt(
        &cleanup,
        4,
        "cleanup-complete",
        &serde_json::to_value(&out)?,
    )?;
    Ok(out)
}
