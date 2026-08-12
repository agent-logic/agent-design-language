use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{render, CardContent, CardKind, CardValues};
use crate::{ErrorCode, IssueRecord, Result, Store, V2Error};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
        backup_generation: u64,
        backup_record_digest: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FailedOperationLineage {
    pub prior_generation: u64,
    pub prior_record_digest: String,
    pub rejected_manifest_digest: String,
    pub failure_boundary: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManifestEntry {
    pub path: String,
    pub identity: NodeIdentity,
    pub size: u64,
    pub digest: Option<String>,
}

fn receipt_payload<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let name = path.file_name().and_then(|v| v.to_str()).ok_or_else(|| {
        V2Error::new(ErrorCode::CorruptRecord, "recovery receipt name is invalid")
    })?;
    let (seq, state) = name
        .strip_suffix(".json")
        .and_then(|v| v.split_once('-'))
        .ok_or_else(|| {
            V2Error::new(ErrorCode::CorruptRecord, "recovery receipt name is invalid")
        })?;
    let seq: u32 = seq.parse().map_err(|_| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt sequence is invalid",
        )
    })?;
    let value: serde_json::Value = serde_json::from_slice(&fs::read(path)?)?;
    if value.get("schema").and_then(|v| v.as_str()) != Some("csdlc.projection_recovery_receipt.v1")
        || value.get("sequence").and_then(|v| v.as_u64()) != Some(seq as u64)
        || value.get("state").and_then(|v| v.as_str()) != Some(state)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope identity is invalid",
        ));
    }
    let expected_previous = if seq == 1 {
        None
    } else {
        let prefix = format!("{:03}-", seq - 1);
        let prior: Vec<_> = fs::read_dir(path.parent().unwrap())?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
            .collect();
        if prior.len() != 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt chain predecessor is ambiguous",
            ));
        }
        Some(
            blake3::hash(&fs::read(prior[0].path())?)
                .to_hex()
                .to_string(),
        )
    };
    if value
        .get("previous_receipt_digest")
        .and_then(|v| v.as_str())
        != expected_previous.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt chain digest is invalid",
        ));
    }
    serde_json::from_value(value.get("payload").cloned().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt payload is missing",
        )
    })?)
    .map_err(Into::into)
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
    pub classification: ProjectionClassification,
    pub failed_operation_lineage: FailedOperationLineage,
    pub anchor: ProjectionCasAnchor,
    pub actor: String,
    pub reason: String,
    pub branch: String,
    pub worktree: String,
    #[serde(default)]
    pub fail_after: Option<String>,
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

fn failpoint(request: &ProjectionRecoverRequest, state: &str) -> Result<()> {
    if request.fail_after.as_deref() == Some(state) {
        Err(V2Error::new(
            ErrorCode::InterruptedTransaction,
            format!("injected projection recovery interruption after {state}"),
        ))
    } else {
        Ok(())
    }
}

fn request_authority_digest(request: &ProjectionRecoverRequest) -> Result<String> {
    let mut stable = request.clone();
    stable.fail_after = None;
    Ok(blake3::hash(&serde_json::to_vec(&stable)?)
        .to_hex()
        .to_string())
}

fn fd_identity(file: &File) -> Result<NodeIdentity> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata()?;
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
        mount_id: fd_mount_id(file)?,
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

fn fd_mount_id(file: &File) -> Result<String> {
    use std::mem::MaybeUninit;
    use std::os::fd::AsRawFd;
    let mut stat = MaybeUninit::<libc::statfs>::uninit();
    if unsafe { libc::fstatfs(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let stat = unsafe { stat.assume_init() };
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        let words: [i32; 2] = unsafe { std::mem::transmute_copy(&stat.f_fsid) };
        Ok(format!("{}:{}", words[0], words[1]))
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    {
        Ok(format!("dev:{}", stat.f_fsid.__val[0]))
    }
}

fn open_root_no_follow(path: &Path) -> Result<File> {
    use std::ffi::CString;
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn open_child_no_follow(parent: &File, name: &std::ffi::CStr, directory: bool) -> Result<File> {
    use std::os::fd::{AsRawFd, FromRawFd};
    let flags = libc::O_RDONLY
        | libc::O_NOFOLLOW
        | libc::O_CLOEXEC
        | if directory { libc::O_DIRECTORY } else { 0 };
    let fd = unsafe { libc::openat(parent.as_raw_fd(), name.as_ptr(), flags) };
    if fd < 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn identity(path: &Path, _metadata: &fs::Metadata) -> Result<NodeIdentity> {
    use std::ffi::CString;
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    fd_identity(&unsafe { File::from_raw_fd(fd) })
}

fn directory_names(directory: &File) -> Result<Vec<std::ffi::CString>> {
    use std::os::fd::AsRawFd;
    let dup = unsafe { libc::dup(directory.as_raw_fd()) };
    if dup < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let stream = unsafe { libc::fdopendir(dup) };
    if stream.is_null() {
        unsafe { libc::close(dup) };
        return Err(std::io::Error::last_os_error().into());
    }
    let mut names = Vec::new();
    loop {
        let entry = unsafe { libc::readdir(stream) };
        if entry.is_null() {
            break;
        }
        let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
        if name.to_bytes() != b"." && name.to_bytes() != b".." {
            names.push(std::ffi::CString::new(name.to_bytes()).map_err(|_| {
                V2Error::new(ErrorCode::InvalidInput, "directory entry contains NUL")
            })?);
        }
    }
    unsafe { libc::closedir(stream) };
    names.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    Ok(names)
}

fn walk(path: &Path) -> Result<Vec<ManifestEntry>> {
    let root = open_root_no_follow(path)?;
    let root_id = fd_identity(&root)?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let mut retained = Vec::<File>::new();
    fn visit(
        file: File,
        relative: String,
        root_id: &NodeIdentity,
        seen: &mut BTreeSet<(String, u64, u64)>,
        out: &mut Vec<ManifestEntry>,
        retained: &mut Vec<File>,
    ) -> Result<()> {
        use std::os::fd::{AsRawFd, FromRawFd};
        let id = fd_identity(&file)?;
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
        let digest = if id.node_type == "regular" {
            let dup = unsafe { libc::dup(file.as_raw_fd()) };
            if dup < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            let mut copy = unsafe { File::from_raw_fd(dup) };
            let mut bytes = Vec::new();
            copy.read_to_end(&mut bytes)?;
            Some(blake3::hash(&bytes).to_hex().to_string())
        } else {
            None
        };
        let size = file.metadata()?.len();
        out.push(ManifestEntry {
            path: relative.clone(),
            identity: id.clone(),
            size,
            digest,
        });
        if id.node_type == "directory" {
            for name in directory_names(&file)? {
                let text = std::str::from_utf8(name.as_bytes()).map_err(|_| {
                    V2Error::new(ErrorCode::InvalidInput, "projection path must be UTF-8")
                })?;
                let child_rel = if relative == "." {
                    text.into()
                } else {
                    format!("{relative}/{text}")
                };
                let child = open_child_no_follow(&file, &name, false)?;
                visit(child, child_rel, root_id, seen, out, retained)?;
            }
        }
        retained.push(file);
        Ok(())
    }
    visit(
        root,
        ".".into(),
        &root_id,
        &mut seen,
        &mut out,
        &mut retained,
    )?;
    Ok(out)
}

fn validate_projection(path: &Path, issue: u64) -> Result<IssueRecord> {
    use strum::IntoEnumIterator;
    let record: IssueRecord = serde_json::from_slice(&fs::read(path.join("index.json"))?)?;
    if record.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "projection namespace mismatch",
        ));
    }
    crate::store::verify_record(&record)?;
    let mut cards: BTreeMap<CardKind, CardValues> = BTreeMap::new();
    for kind in CardKind::iter() {
        let values: CardValues = serde_json::from_slice(&fs::read(
            path.join("cards").join(format!("{kind}.values.json")),
        )?)?;
        if values.identity.issue != issue
            || values.identity.generation != record.generation
            || values.kind() != kind
        {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "projection card identity is inconsistent",
            ));
        }
        let rendered = render(&values)?;
        let projection = record.cards.get(&kind).ok_or_else(|| {
            V2Error::new(ErrorCode::CardInvalid, "projection card digest is missing")
        })?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
            || fs::read(path.join("cards").join(format!("{kind}.md")))?
                != rendered.markdown.as_bytes()
        {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "projection card artifact or digest drift",
            ));
        }
        cards.insert(kind, values);
    }
    let spp = cards
        .get(&CardKind::Spp)
        .and_then(|v| match &v.content {
            CardContent::Spp(v) => Some(v),
            _ => None,
        })
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "SPP card is missing"))?;
    crate::cards::validate_cross_card(
        &cards,
        &spp.design_ref,
        &spp.design_digest,
        &spp.diagram_ref,
        &spp.diagram_digest,
    )?;
    let mut cursor = path;
    let repo_root = loop {
        if cursor.file_name().and_then(|v| v.to_str()) == Some(".csdlc") {
            break cursor.parent().unwrap();
        }
        cursor = cursor.parent().ok_or_else(|| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "projection is outside a repository .csdlc namespace",
            )
        })?;
    };
    for (relative, digest) in [
        (&spp.design_ref, &spp.design_digest),
        (&spp.diagram_ref, &spp.diagram_digest),
    ] {
        let bytes = fs::read(repo_root.join(relative))?;
        if blake3::hash(&bytes).to_hex().as_str() != digest {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "projection authored artifact digest drift",
            ));
        }
    }
    let audit = fs::read(path.join("audit.jsonl"))?;
    let mut expected = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut expected, event)?;
        expected.push(b'\n');
    }
    if audit != expected {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "projection audit artifact drift",
        ));
    }
    Ok(record)
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
    let validation = validate_projection(path, issue);
    let record = validation.as_ref().ok();
    let valid = record.is_some();
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
            Some(
                validation
                    .err()
                    .map(|e| e.message)
                    .unwrap_or_else(|| "candidate projection invalid".into()),
            )
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
            backup_generation,
            backup_record_digest,
        } => {
            !canonical.valid_projection
                && canonical
                    .entries
                    .first()
                    .is_some_and(|e| &e.identity == canonical_identity)
                && canonical.manifest_digest.as_deref() == Some(manifest_digest)
                && backup.valid_projection
                && backup.generation == Some(*backup_generation)
                && backup.record_digest.as_deref() == Some(backup_record_digest)
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
    let recovery = recovery_root(store, request.issue);
    let mut completed_match = false;
    if recovery.is_dir() {
        for entry in fs::read_dir(&recovery)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_str().ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery namespace contains a non-UTF8 entry",
                )
            })?;
            validate_operation_id(name)?;
            if !entry.file_type()?.is_dir() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "recovery namespace contains a non-directory attempt",
                ));
            }
            let recovered: Vec<_> = fs::read_dir(entry.path())?
                .filter_map(std::result::Result::ok)
                .filter(|v| v.file_name().to_string_lossy().ends_with("-recovered.json"))
                .collect();
            if recovered.len() > 1 {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery attempt has ambiguous terminal receipts",
                ));
            }
            if !recovered.is_empty() {
                let result =
                    validate_completed_recovery_attempt(store, request.issue, &entry.path())?;
                completed_match |= result.issue == request.issue
                    && canonical.valid_projection
                    && canonical.generation == Some(result.canonical_generation)
                    && canonical.record_digest.as_deref() == Some(&result.canonical_digest);
            }
        }
    }
    let unsafe_any = [&canonical, &backup, &preserved]
        .iter()
        .any(|c| c.state == "unsafe");
    let disposition = if unsafe_any {
        "unsafe"
    } else if completed_match {
        "already_recovered"
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
    let previous_digest = if seq == 1 {
        None
    } else {
        let prefix = format!("{:03}-", seq - 1);
        let prior: Vec<_> = fs::read_dir(dir)?
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
            .collect();
        if prior.len() != 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt chain predecessor is missing or ambiguous",
            ));
        }
        Some(
            blake3::hash(&fs::read(prior[0].path())?)
                .to_hex()
                .to_string(),
        )
    };
    let envelope = serde_json::json!({
        "schema":"csdlc.projection_recovery_receipt.v1",
        "sequence":seq,
        "state":state,
        "previous_receipt_digest":previous_digest,
        "payload":value
    });
    let mut bytes = serde_json::to_vec_pretty(&envelope)?;
    bytes.push(b'\n');
    if p.symlink_metadata().is_ok() {
        let existing = fs::read(&p)?;
        if existing == bytes {
            return Ok(blake3::hash(&bytes).to_hex().to_string());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "immutable recovery receipt collision",
        ));
    }
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

fn receipt_path(dir: &Path, seq: u32, state: &str) -> PathBuf {
    dir.join(format!("{seq:03}-{state}.json"))
}

fn validate_receipt_inventory(dir: &Path) -> Result<()> {
    let mut receipts: Vec<_> = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|t| t.is_file()))
        .collect();
    receipts.sort_by_key(|entry| entry.file_name());
    for (index, entry) in receipts.iter().enumerate() {
        let name = entry.file_name();
        let name = name.to_str().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt name is not UTF-8",
            )
        })?;
        let seq: usize = name.get(..3).and_then(|v| v.parse().ok()).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt name has no sequence",
            )
        })?;
        if seq != index + 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt inventory is not a unique exact prefix",
            ));
        }
        let _: serde_json::Value = receipt_payload(&entry.path())?;
    }
    Ok(())
}

const RECOVERY_STATES: [&str; 13] = [
    "prepared",
    "archive-intent",
    "rejected-archived",
    "candidate-plan",
    "candidate-created",
    "candidate-verified",
    "install-intent",
    "canonical-installed",
    "displace-intent",
    "prior-displaced",
    "canonical-verified",
    "recovery-complete-intent",
    "recovered",
];

fn require_receipt_payload(
    attempt: &Path,
    seq: u32,
    state: &str,
    expected: serde_json::Value,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload(&receipt_path(attempt, seq, state))?;
    if actual == expected {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn observations_match_after_move(
    actual: &CandidateObservation,
    expected: &CandidateObservation,
) -> bool {
    actual.state == expected.state
        && actual.valid_projection == expected.valid_projection
        && actual.generation == expected.generation
        && actual.record_digest == expected.record_digest
        && actual.entries.len() == expected.entries.len()
        && actual.entries.iter().zip(&expected.entries).all(|(a, e)| {
            a.path == e.path
                && a.size == e.size
                && a.digest == e.digest
                && a.identity.device == e.identity.device
                && a.identity.mount_id == e.identity.mount_id
                && a.identity.inode == e.identity.inode
                && a.identity.links == e.identity.links
                && a.identity.uid == e.identity.uid
                && a.identity.gid == e.identity.gid
                && a.identity.mode == e.identity.mode
                && a.identity.node_type == e.identity.node_type
        })
}

pub(crate) fn validate_completed_recovery_attempt(
    store: &Store,
    issue: u64,
    attempt: &Path,
) -> Result<ProjectionRecoveryResult> {
    validate_receipt_inventory(attempt)?;
    for (offset, state) in RECOVERY_STATES.iter().enumerate() {
        let path = receipt_path(attempt, offset as u32 + 1, state);
        if !path.is_file() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recovery attempt is not an exact completed state sequence",
            ));
        }
        let _: serde_json::Value = receipt_payload(&path)?;
    }
    let prepared: serde_json::Value = receipt_payload(&receipt_path(attempt, 1, "prepared"))?;
    let request_digest = prepared
        .get("request_digest")
        .and_then(|v| v.as_str())
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "PREPARED request digest missing"))?;
    if request_digest.len() != 64 || !request_digest.bytes().all(|v| v.is_ascii_hexdigit()) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "PREPARED request digest invalid",
        ));
    }
    let prepared_request: ProjectionRecoverRequest =
        serde_json::from_value(prepared.get("request").cloned().ok_or_else(|| {
            V2Error::new(ErrorCode::CorruptRecord, "PREPARED typed request missing")
        })?)?;
    if prepared_request.issue != issue
        || prepared_request.operation_id
            != attempt
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or_default()
        || request_authority_digest(&prepared_request)? != request_digest
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "PREPARED typed request authority invalid",
        ));
    }
    let prepared_classification: ProjectionClassification =
        serde_json::from_value(prepared.get("classification").cloned().ok_or_else(|| {
            V2Error::new(ErrorCode::CorruptRecord, "PREPARED classification missing")
        })?)?;
    if prepared_classification.receipt_digest != prepared_request.classify_receipt_digest
        || prepared_classification.issue != issue
        || prepared.get("lineage")
            != serde_json::to_value(&prepared_request.failed_operation_lineage)
                .ok()
                .as_ref()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "PREPARED classification or lineage authority invalid",
        ));
    }
    let prior_observation = if prepared_classification.canonical.valid_projection {
        prepared_classification.canonical.clone()
    } else {
        prepared_classification.backup.clone()
    };
    let rejected_observation = if matches!(
        prepared_request.anchor,
        ProjectionCasAnchor::ExactObservedInvalid { .. }
    ) {
        prepared_classification.canonical.clone()
    } else {
        prepared_classification.preserved.clone()
    };
    let install_exchange = matches!(
        prepared_request.anchor,
        ProjectionCasAnchor::VerifiedCanonical { .. }
    );
    require_receipt_payload(
        attempt,
        2,
        "archive-intent",
        serde_json::json!({"source":rejected_observation,"destination":"rejected"}),
        "archive intent receipt does not match typed recovery sources",
    )?;
    let archived = observe(&attempt.join("rejected"), "archive", issue);
    require_receipt_payload(
        attempt,
        3,
        "rejected-archived",
        serde_json::to_value(&archived)?,
        "archived receipt does not match retained rejected evidence",
    )?;
    let audit_commitment = serde_json::json!({
        "operation":"recover_preserved_projection",
        "operation_id":prepared_request.operation_id,
        "classify_receipt":prepared_request.classify_receipt_digest,
        "anchor":prepared_request.anchor,
        "lineage":prepared_request.failed_operation_lineage,
        "archived_manifest":archived.manifest_digest,
        "prior_generation":prior_observation.generation,
        "prior_digest":prior_observation.record_digest,
        "canonical_generation":prior_observation.generation.map(|v| v + 1)
    });
    require_receipt_payload(
        attempt,
        4,
        "candidate-plan",
        serde_json::json!({"source":prior_observation,"destination":"candidate","audit":audit_commitment}),
        "candidate plan receipt does not match typed recovery audit",
    )?;
    let candidate_created: serde_json::Value =
        receipt_payload(&receipt_path(attempt, 5, "candidate-created"))?;
    let candidate_record: IssueRecord =
        serde_json::from_value(candidate_created.get("record").cloned().ok_or_else(|| {
            V2Error::new(ErrorCode::CorruptRecord, "candidate-created record missing")
        })?)?;
    let candidate_observation: CandidateObservation =
        serde_json::from_value(candidate_created.get("candidate").cloned().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "candidate-created observation missing",
            )
        })?)?;
    if crate::store::record_digest(&candidate_record)? != candidate_record.digest
        || !candidate_observation.valid_projection
        || candidate_observation.generation != Some(candidate_record.generation)
        || candidate_observation.record_digest.as_deref() != Some(candidate_record.digest.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate-created receipt is not self-consistent",
        ));
    }
    require_receipt_payload(
        attempt,
        6,
        "candidate-verified",
        serde_json::json!({"candidate":candidate_observation,"record_digest":candidate_record.digest,"generation":candidate_record.generation}),
        "candidate-verified receipt does not match created candidate",
    )?;
    require_receipt_payload(
        attempt,
        7,
        "install-intent",
        serde_json::json!({"exchange":install_exchange,"candidate":candidate_observation,"canonical":prepared_classification.canonical}),
        "install intent receipt does not match candidate or anchor mode",
    )?;
    let canonical_installed: serde_json::Value =
        receipt_payload(&receipt_path(attempt, 8, "canonical-installed"))?;
    let installed_observation: CandidateObservation =
        serde_json::from_value(canonical_installed.get("canonical").cloned().ok_or_else(
            || {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "canonical-installed observation missing",
                )
            },
        )?)?;
    if !observations_match_after_move(&installed_observation, &candidate_observation) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "canonical-installed receipt does not match created candidate",
        ));
    }
    require_receipt_payload(
        attempt,
        9,
        "displace-intent",
        if install_exchange {
            serde_json::json!({"source":prior_observation,"destination":"displaced"})
        } else {
            serde_json::json!({"source":prior_observation,"destination":"displaced","completed_before_install":true})
        },
        "displace intent receipt does not match typed prior source",
    )?;
    let displaced = observe(&attempt.join("displaced"), "displaced", issue);
    require_receipt_payload(
        attempt,
        10,
        "prior-displaced",
        serde_json::to_value(&displaced)?,
        "prior-displaced receipt does not match retained displaced evidence",
    )?;
    if !observations_match_after_move(&displaced, &prior_observation) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "displaced receipt does not match typed prior source",
        ));
    }
    require_receipt_payload(
        attempt,
        11,
        "canonical-verified",
        serde_json::json!({"canonical":installed_observation,"archive":archived,"displaced":displaced}),
        "canonical-verified receipt does not match installed recovery state",
    )?;
    require_receipt_payload(
        attempt,
        12,
        "recovery-complete-intent",
        serde_json::json!({"canonical_digest":candidate_record.digest,"generation":candidate_record.generation}),
        "recovery complete intent does not match candidate record",
    )?;
    let result: ProjectionRecoveryResult =
        receipt_payload(&receipt_path(attempt, 13, "recovered"))?;
    let operation = attempt
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "attempt operation identity invalid",
            )
        })?;
    if result.schema != "csdlc.projection_recovery_result.v1"
        || result.issue != issue
        || result.operation_id != operation
        || result.disposition != "recovered"
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal recovery result identity invalid",
        ));
    }
    let mut expected = result.clone();
    expected.receipt_digest.clear();
    if result.receipt_digest
        != blake3::hash(&serde_json::to_vec(&expected)?)
            .to_hex()
            .to_string()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal recovery result self-digest invalid",
        ));
    }
    if result.canonical_generation != candidate_record.generation
        || result.canonical_digest != candidate_record.digest
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal recovery result does not match verified candidate record",
        ));
    }
    let canonical_record = validate_projection(&store.issue_dir(issue), issue)?;
    let audit_bound = canonical_record.audit.iter().any(|event| {
        serde_json::from_str::<serde_json::Value>(&event.operation)
            .ok()
            .is_some_and(|audit| {
                audit == audit_commitment
                    && audit.get("operation").and_then(|v| v.as_str())
                        == Some("recover_preserved_projection")
                    && audit.get("operation_id").and_then(|v| v.as_str())
                        == Some(result.operation_id.as_str())
                    && audit.get("classify_receipt").and_then(|v| v.as_str())
                        == Some(prepared_request.classify_receipt_digest.as_str())
                    && audit.get("anchor")
                        == serde_json::to_value(&prepared_request.anchor).ok().as_ref()
                    && audit.get("lineage")
                        == serde_json::to_value(&prepared_request.failed_operation_lineage)
                            .ok()
                            .as_ref()
                    && audit.get("canonical_generation").and_then(|v| v.as_u64())
                        == Some(result.canonical_generation)
            })
    });
    if canonical_record.generation < result.canonical_generation || !audit_bound {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed recovery is not durably bound to canonical recovery audit",
        ));
    }
    Ok(result)
}

fn exact_observation(path: &Path, expected: &CandidateObservation, issue: u64) -> Result<bool> {
    let actual = observe(path, &expected.name, issue);
    Ok(actual.state == expected.state
        && actual.manifest_digest == expected.manifest_digest
        && actual.entries == expected.entries
        && actual.generation == expected.generation
        && actual.record_digest == expected.record_digest)
}

fn same_moved_observation(
    path: &Path,
    expected: &CandidateObservation,
    issue: u64,
) -> Result<bool> {
    let actual = observe(path, &expected.name, issue);
    if actual.state != expected.state
        || actual.generation != expected.generation
        || actual.record_digest != expected.record_digest
        || actual.entries.len() != expected.entries.len()
    {
        return Ok(false);
    }
    Ok(actual.entries.iter().zip(&expected.entries).all(|(a, e)| {
        a.path == e.path
            && a.size == e.size
            && a.digest == e.digest
            && a.identity.device == e.identity.device
            && a.identity.mount_id == e.identity.mount_id
            && a.identity.inode == e.identity.inode
            && a.identity.links == e.identity.links
            && a.identity.uid == e.identity.uid
            && a.identity.gid == e.identity.gid
            && a.identity.mode == e.identity.mode
            && a.identity.node_type == e.identity.node_type
    }))
}

struct AnchoredName {
    parent: File,
    name: std::ffi::CString,
    child: Option<File>,
}

fn anchored_root_identity_matches(
    anchor: &AnchoredName,
    expected: &CandidateObservation,
) -> Result<bool> {
    anchored_root_identity_matches_inner(anchor, expected, true)
}

fn anchored_moved_root_identity_matches(
    anchor: &AnchoredName,
    expected: &CandidateObservation,
) -> Result<bool> {
    anchored_root_identity_matches_inner(anchor, expected, false)
}

fn anchored_root_identity_matches_inner(
    anchor: &AnchoredName,
    expected: &CandidateObservation,
    require_ctime: bool,
) -> Result<bool> {
    let Some(root) = expected.entries.iter().find(|entry| entry.path == ".") else {
        return Ok(false);
    };
    let actual = fd_identity(anchor.child.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "anchored source child is absent",
        )
    })?)?;
    Ok(actual.device == root.identity.device
        && actual.mount_id == root.identity.mount_id
        && actual.inode == root.identity.inode
        && (!require_ctime
            || (actual.ctime_seconds == root.identity.ctime_seconds
                && actual.ctime_nanoseconds == root.identity.ctime_nanoseconds)))
}

fn anchored_name(path: &Path) -> Result<AnchoredName> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let parent =
        open_root_no_follow(path.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "anchored path has no parent")
        })?)?;
    let name = CString::new(
        path.file_name()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "anchored path has no name"))?
            .as_bytes(),
    )
    .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "anchored path contains NUL"))?;
    let child = if path.symlink_metadata().is_ok() {
        Some(open_child_no_follow(&parent, &name, false)?)
    } else {
        None
    };
    Ok(AnchoredName {
        parent,
        name,
        child,
    })
}

#[cfg(target_os = "macos")]
fn rename_no_replace(source: &AnchoredName, destination: &AnchoredName) -> Result<()> {
    use std::os::fd::AsRawFd;
    let result = unsafe {
        libc::renameatx_np(
            source.parent.as_raw_fd(),
            source.name.as_ptr(),
            destination.parent.as_raw_fd(),
            destination.name.as_ptr(),
            libc::RENAME_EXCL,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(target_os = "linux")]
fn rename_no_replace(source: &AnchoredName, destination: &AnchoredName) -> Result<()> {
    use std::os::fd::AsRawFd;
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source.parent.as_raw_fd(),
            source.name.as_ptr(),
            destination.parent.as_raw_fd(),
            destination.name.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn rename_no_replace(_source: &AnchoredName, _destination: &AnchoredName) -> Result<()> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "no proven no-replace rename primitive on this platform",
    ))
}

#[cfg(target_os = "macos")]
fn exchange(source: &AnchoredName, destination: &AnchoredName) -> Result<()> {
    use std::os::fd::AsRawFd;
    let result = unsafe {
        libc::renameatx_np(
            source.parent.as_raw_fd(),
            source.name.as_ptr(),
            destination.parent.as_raw_fd(),
            destination.name.as_ptr(),
            libc::RENAME_SWAP,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(target_os = "linux")]
fn exchange(source: &AnchoredName, destination: &AnchoredName) -> Result<()> {
    use std::os::fd::AsRawFd;
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source.parent.as_raw_fd(),
            source.name.as_ptr(),
            destination.parent.as_raw_fd(),
            destination.name.as_ptr(),
            libc::RENAME_EXCHANGE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error().into())
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn exchange(_source: &AnchoredName, _destination: &AnchoredName) -> Result<()> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "no proven atomic exchange primitive on this platform",
    ))
}

fn ensure_candidate_directory(
    request: &ProjectionRecoverRequest,
    attempt: &Path,
    candidate: &Path,
    relative: &str,
    ordinal: usize,
) -> Result<()> {
    let ledger = attempt.join(format!("node-{ordinal:03}"));
    if ledger.symlink_metadata().is_err() {
        private_dir(&ledger)?;
    }
    let path = if relative == "." {
        candidate.to_path_buf()
    } else {
        candidate.join(relative)
    };
    if receipt_path(&ledger, 10, "node-created").is_file() {
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed candidate directory was replaced",
        ));
    }
    receipt(
        &ledger,
        1,
        "create-intent",
        &serde_json::json!({"path":relative,"type":"directory","mode":"0700"}),
    )?;
    failpoint(request, "node_create_intent")?;
    if !receipt_path(&ledger, 2, "created-identity").is_file() {
        if path.symlink_metadata().is_err() {
            fs::create_dir(&path)?;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
        }
        let metadata = fs::symlink_metadata(&path)?;
        let node = identity(&path, &metadata)?;
        if node.node_type != "directory" || node.mode & 0o777 != 0o700 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate directory collision or mode drift",
            ));
        }
        receipt(
            &ledger,
            2,
            "created-identity",
            &serde_json::to_value(&node)?,
        )?;
        failpoint(request, "node_created_identity")?;
    }
    receipt(
        &ledger,
        3,
        "write-intent",
        &serde_json::json!({"directory":relative,"metadata_only":true}),
    )?;
    receipt(
        &ledger,
        4,
        "write-completed",
        &serde_json::json!({"directory":relative}),
    )?;
    failpoint(request, "node_write_completed")?;
    receipt(
        &ledger,
        5,
        "node-fsync-intent",
        &serde_json::json!({"directory":relative}),
    )?;
    sync(&path)?;
    failpoint(request, "node_fsynced")?;
    receipt(
        &ledger,
        6,
        "node-fsync-completed",
        &serde_json::json!({"identity":identity(&path,&fs::symlink_metadata(&path)?)?}),
    )?;
    receipt(
        &ledger,
        7,
        "parent-fsync-intent",
        &serde_json::json!({"directory":relative}),
    )?;
    sync(path.parent().ok_or_else(|| {
        V2Error::new(ErrorCode::InvalidInput, "candidate directory has no parent")
    })?)?;
    failpoint(request, "node_parent_fsynced")?;
    receipt(
        &ledger,
        8,
        "parent-fsync-completed",
        &serde_json::json!({"directory":relative}),
    )?;
    receipt(
        &ledger,
        9,
        "publish-intent",
        &serde_json::json!({"directory":relative,"already_at_final_name":true}),
    )?;
    receipt(
        &ledger,
        10,
        "node-created",
        &serde_json::json!({"identity":identity(&path,&fs::symlink_metadata(&path)?)?}),
    )?;
    Ok(())
}

fn ensure_candidate_file(
    request: &ProjectionRecoverRequest,
    attempt: &Path,
    candidate: &Path,
    relative: &str,
    contents: &[u8],
    ordinal: usize,
) -> Result<()> {
    let ledger = attempt.join(format!("node-{ordinal:03}"));
    if ledger.symlink_metadata().is_err() {
        private_dir(&ledger)?;
    }
    let final_path = candidate.join(relative);
    let parent = final_path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "candidate file has no parent"))?;
    let temporary = parent.join(format!(".recovery-node-{ordinal:03}.tmp"));
    let digest = blake3::hash(contents).to_hex().to_string();
    if receipt_path(&ledger, 10, "node-created").is_file() {
        let metadata = fs::symlink_metadata(&final_path)?;
        if metadata.is_file()
            && !metadata.file_type().is_symlink()
            && metadata.nlink() == 1
            && fs::read(&final_path)? == contents
        {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed candidate file was replaced",
        ));
    }
    if receipt_path(&ledger, 9, "publish-intent").is_file() {
        let temp_meta = temporary.symlink_metadata();
        let final_meta = final_path.symlink_metadata();
        if temp_meta.is_ok() && final_meta.is_err() {
            rename_no_replace(&anchored_name(&temporary)?, &anchored_name(&final_path)?)?;
        } else if !(temp_meta.is_err() && final_meta.is_ok() && fs::read(&final_path)? == contents)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate node publish restart is ambiguous",
            ));
        }
        sync(parent)?;
        let node = identity(&final_path, &fs::symlink_metadata(&final_path)?)?;
        receipt(
            &ledger,
            10,
            "node-created",
            &serde_json::json!({"identity":node,"digest":digest,"size":contents.len()}),
        )?;
        return Ok(());
    }
    receipt(
        &ledger,
        1,
        "create-intent",
        &serde_json::json!({"temporary":temporary.file_name(),"final":relative,"type":"regular","mode":"0600","digest":digest,"size":contents.len()}),
    )?;
    failpoint(request, "node_create_intent")?;
    if !receipt_path(&ledger, 2, "created-identity").is_file() {
        if temporary.symlink_metadata().is_err() {
            OpenOptions::new()
                .create_new(true)
                .read(true)
                .write(true)
                .mode(0o600)
                .open(&temporary)?;
        }
        let node = identity(&temporary, &fs::symlink_metadata(&temporary)?)?;
        if node.node_type != "regular" || node.links != 1 || node.mode & 0o777 != 0o600 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary identity is unsafe",
            ));
        }
        receipt(
            &ledger,
            2,
            "created-identity",
            &serde_json::to_value(&node)?,
        )?;
        failpoint(request, "node_created_identity")?;
    }
    receipt(
        &ledger,
        3,
        "write-intent",
        &serde_json::json!({"digest":digest,"size":contents.len()}),
    )?;
    if !receipt_path(&ledger, 4, "write-completed").is_file() {
        let existing = fs::read(&temporary)?;
        if existing.len() > contents.len() || existing != contents[..existing.len()] {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary content is not the committed prefix",
            ));
        }
        if existing.len() < contents.len() {
            let mut file = OpenOptions::new().append(true).open(&temporary)?;
            file.write_all(&contents[existing.len()..])?;
        }
        if fs::read(&temporary)? != contents {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary content completion failed",
            ));
        }
        receipt(
            &ledger,
            4,
            "write-completed",
            &serde_json::json!({"digest":digest,"size":contents.len()}),
        )?;
        failpoint(request, "node_write_completed")?;
    }
    receipt(
        &ledger,
        5,
        "node-fsync-intent",
        &serde_json::json!({"temporary":temporary.file_name()}),
    )?;
    File::open(&temporary)?.sync_all()?;
    failpoint(request, "node_fsynced")?;
    receipt(
        &ledger,
        6,
        "node-fsync-completed",
        &serde_json::json!({"identity":identity(&temporary,&fs::symlink_metadata(&temporary)?)?,"digest":digest}),
    )?;
    receipt(
        &ledger,
        7,
        "parent-fsync-intent",
        &serde_json::json!({"parent":parent}),
    )?;
    sync(parent)?;
    failpoint(request, "node_parent_fsynced")?;
    receipt(
        &ledger,
        8,
        "parent-fsync-completed",
        &serde_json::json!({"parent":parent}),
    )?;
    receipt(
        &ledger,
        9,
        "publish-intent",
        &serde_json::json!({"temporary":temporary.file_name(),"final":relative}),
    )?;
    if !receipt_path(&ledger, 10, "node-created").is_file() {
        let temp_meta = temporary.symlink_metadata();
        let final_meta = final_path.symlink_metadata();
        if temp_meta.is_ok() && final_meta.is_err() {
            rename_no_replace(&anchored_name(&temporary)?, &anchored_name(&final_path)?)?;
        } else if !(temp_meta.is_err() && final_meta.is_ok() && fs::read(&final_path)? == contents)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate node publish state is ambiguous",
            ));
        }
        failpoint(request, "node_published")?;
        sync(parent)?;
        let node = identity(&final_path, &fs::symlink_metadata(&final_path)?)?;
        if node.node_type != "regular" || node.links != 1 || fs::read(&final_path)? != contents {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "published candidate node drifted",
            ));
        }
        receipt(
            &ledger,
            10,
            "node-created",
            &serde_json::json!({"identity":node,"digest":digest,"size":contents.len()}),
        )?;
    }
    Ok(())
}

fn build_candidate_tree(
    store: &Store,
    request: &ProjectionRecoverRequest,
    attempt: &Path,
    source: &Path,
    candidate: &Path,
    expected_digest: &str,
    audit_payload: &serde_json::Value,
) -> Result<IssueRecord> {
    let (record, files) = store.projection_recovery_candidate_files_locked(
        request.issue,
        source,
        expected_digest,
        request.actor.clone(),
        request.reason.clone(),
        audit_payload.to_string(),
    )?;
    ensure_candidate_directory(request, attempt, candidate, ".", 0)?;
    ensure_candidate_directory(request, attempt, candidate, "cards", 1)?;
    for (ordinal, (relative, contents)) in files.iter().enumerate() {
        ensure_candidate_file(request, attempt, candidate, relative, contents, ordinal + 2)?;
    }
    Ok(record)
}

pub fn recover_preserved_projection(
    store: &Store,
    request: ProjectionRecoverRequest,
) -> Result<ProjectionRecoveryResult> {
    validate_operation_id(&request.operation_id)?;
    verify_topology(store, &request.branch, &request.worktree)?;
    let _lock = store.authority_projection_lock(request.issue)?;
    let root = recovery_root(store, request.issue);
    if root.symlink_metadata().is_err() {
        private_dir(&root)?;
    }
    let attempt = root.join(&request.operation_id);
    if root.is_dir() {
        for entry in fs::read_dir(&root)? {
            let entry = entry?;
            if entry.file_name() != request.operation_id.as_str()
                && entry.file_type()?.is_dir()
                && !fs::read_dir(entry.path())?.any(|item| {
                    item.ok().is_some_and(|item| {
                        item.file_name()
                            .to_string_lossy()
                            .ends_with("-recovered.json")
                    })
                })
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "a different incomplete recovery attempt already exists",
                ));
            }
        }
    }
    let prepared_path = receipt_path(&attempt, 1, "prepared");
    let done = receipt_path(&attempt, 13, "recovered");
    if done.is_file() {
        return validate_completed_recovery_attempt(store, request.issue, &attempt);
    }

    if request.classification.receipt_digest != request.classify_receipt_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "classification payload and receipt digest disagree",
        ));
    }
    let classification = if attempt.exists() {
        if !prepared_path.is_file() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recovery attempt exists without durable PREPARED receipt",
            ));
        }
        let prepared: serde_json::Value = receipt_payload(&prepared_path)?;
        let classification =
            serde_json::from_value(prepared.get("classification").cloned().ok_or_else(|| {
                V2Error::new(ErrorCode::CorruptRecord, "PREPARED classification missing")
            })?)?;
        if prepared.get("request_digest").and_then(|v| v.as_str())
            != Some(request_authority_digest(&request)?.as_str())
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "active recovery attempt belongs to a different request",
            ));
        }
        classification
    } else {
        let classification = classify_preserved_projection_unlocked(
            store,
            &request,
            &request.classification.actor,
            &request.classification.reason,
        )?;
        if classification.receipt_digest != request.classify_receipt_digest
            || classification.disposition != "recoverable"
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "classification receipt is stale or not recoverable",
            ));
        }
        let prior = if classification.canonical.valid_projection {
            &classification.canonical
        } else {
            &classification.backup
        };
        if prior.generation != Some(request.failed_operation_lineage.prior_generation)
            || prior.record_digest.as_deref()
                != Some(&request.failed_operation_lineage.prior_record_digest)
            || classification.preserved.manifest_digest.as_deref()
                != Some(&request.failed_operation_lineage.rejected_manifest_digest)
            || request.failed_operation_lineage.failure_boundary.is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "failed-operation lineage does not bind the classified sources",
            ));
        }
        private_dir(&attempt)?;
        let request_digest = request_authority_digest(&request)?;
        receipt(
            &attempt,
            1,
            "prepared",
            &serde_json::json!({"request_digest":request_digest,"request":request,"classification":classification,"lineage":request.failed_operation_lineage}),
        )?;
        failpoint(&request, "prepared")?;
        classification
    };

    let canonical_path = store.issue_dir(request.issue);
    let backup_path = store.interrupted_backup(request.issue);
    let preserved_path = store.rollback_preserved(request.issue);
    let archive = attempt.join("rejected");
    let candidate = attempt.join("candidate");
    let displaced = attempt.join("displaced");
    let (prior_path, rejected_path, install_exchange) = match &request.anchor {
        ProjectionCasAnchor::VerifiedCanonical { .. } => (&canonical_path, &preserved_path, true),
        ProjectionCasAnchor::ExpectedCanonicalAbsent { .. } => {
            (&backup_path, &preserved_path, false)
        }
        ProjectionCasAnchor::ExactObservedInvalid { .. } => (&backup_path, &canonical_path, false),
    };
    let prior_observation = if classification.canonical.valid_projection {
        classification.canonical.clone()
    } else {
        classification.backup.clone()
    };
    let rejected_observation = if matches!(
        request.anchor,
        ProjectionCasAnchor::ExactObservedInvalid { .. }
    ) {
        classification.canonical.clone()
    } else {
        classification.preserved.clone()
    };

    receipt(
        &attempt,
        2,
        "archive-intent",
        &serde_json::json!({"source":rejected_observation,"destination":"rejected"}),
    )?;
    failpoint(&request, "archive_intent")?;
    if !receipt_path(&attempt, 3, "rejected-archived").is_file() {
        if rejected_path.symlink_metadata().is_ok() && archive.symlink_metadata().is_err() {
            if !exact_observation(rejected_path, &rejected_observation, request.issue)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "rejected source identity drifted before archive",
                ));
            }
            let rejected_anchor = anchored_name(rejected_path)?;
            if !anchored_root_identity_matches(&rejected_anchor, &rejected_observation)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "rejected source identity drifted at archive mutation boundary",
                ));
            }
            let archive_anchor = anchored_name(&archive)?;
            rename_no_replace(&rejected_anchor, &archive_anchor)?;
            failpoint(&request, "archive_renamed")?;
        } else if rejected_path.symlink_metadata().is_err() && archive.symlink_metadata().is_ok() {
            if !same_moved_observation(&archive, &rejected_observation, request.issue)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "archive identity does not match rejected source",
                ));
            }
        } else {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "archive transition is ambiguous",
            ));
        }
        sync(&issue_parent(store))?;
        sync(&attempt)?;
        receipt(
            &attempt,
            3,
            "rejected-archived",
            &serde_json::to_value(observe(&archive, "archive", request.issue))?,
        )?;
        failpoint(&request, "rejected_archived")?;
    }
    let archived = observe(&archive, "archive", request.issue);

    let audit_payload = serde_json::json!({
        "operation":"recover_preserved_projection",
        "operation_id":request.operation_id,
        "classify_receipt":request.classify_receipt_digest,
        "anchor":request.anchor,
        "lineage":request.failed_operation_lineage,
        "archived_manifest":archived.manifest_digest,
        "prior_generation":prior_observation.generation,
        "prior_digest":prior_observation.record_digest,
        "canonical_generation":prior_observation.generation.map(|v| v + 1)
    });
    receipt(
        &attempt,
        4,
        "candidate-plan",
        &serde_json::json!({"source":prior_observation,"destination":"candidate","audit":audit_payload}),
    )?;
    failpoint(&request, "candidate_plan")?;
    let (candidate_record, candidate_observation) =
        if receipt_path(&attempt, 5, "candidate-created").is_file() {
            let payload: serde_json::Value =
                receipt_payload(&receipt_path(&attempt, 5, "candidate-created"))?;
            (
                serde_json::from_value(payload.get("record").cloned().ok_or_else(|| {
                    V2Error::new(ErrorCode::CorruptRecord, "candidate record receipt missing")
                })?)?,
                serde_json::from_value(payload.get("candidate").cloned().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        "candidate observation receipt missing",
                    )
                })?)?,
            )
        } else {
            if !exact_observation(prior_path, &prior_observation, request.issue)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "verified prior source drifted before candidate construction",
                ));
            }
            let record = build_candidate_tree(
                store,
                &request,
                &attempt,
                prior_path,
                &candidate,
                prior_observation.record_digest.as_deref().ok_or_else(|| {
                    V2Error::new(ErrorCode::CorruptRecord, "verified prior digest missing")
                })?,
                &audit_payload,
            )?;
            sync(&candidate)?;
            sync(&attempt)?;
            let observation = observe(&candidate, "candidate", request.issue);
            receipt(
                &attempt,
                5,
                "candidate-created",
                &serde_json::json!({"record":record,"candidate":observation}),
            )?;
            failpoint(&request, "candidate_created")?;
            (record, observation)
        };
    if !candidate_observation.valid_projection
        || candidate_observation.generation != Some(candidate_record.generation)
        || candidate_observation.record_digest.as_deref() != Some(&candidate_record.digest)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recovery candidate verification failed",
        ));
    }

    receipt(
        &attempt,
        6,
        "candidate-verified",
        &serde_json::json!({"candidate":candidate_observation,"record_digest":candidate_record.digest,"generation":candidate_record.generation}),
    )?;
    failpoint(&request, "candidate_verified")?;

    if !receipt_path(&attempt, 8, "canonical-installed").is_file() {
        receipt(
            &attempt,
            7,
            "install-intent",
            &serde_json::json!({"exchange":install_exchange,"candidate":candidate_observation,"canonical":classification.canonical}),
        )?;
        failpoint(&request, "install_intent")?;
        if install_exchange {
            let candidate_here = candidate.symlink_metadata().is_ok();
            let canonical_here = canonical_path.symlink_metadata().is_ok();
            if candidate_here
                && canonical_here
                && exact_observation(&candidate, &candidate_observation, request.issue)?
                && exact_observation(&canonical_path, &prior_observation, request.issue)?
            {
                let candidate_anchor = anchored_name(&candidate)?;
                let canonical_anchor = anchored_name(&canonical_path)?;
                if !anchored_root_identity_matches(&candidate_anchor, &candidate_observation)?
                    || !anchored_root_identity_matches(&canonical_anchor, &prior_observation)?
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "canonical exchange identity drifted at mutation boundary",
                    ));
                }
                exchange(&candidate_anchor, &canonical_anchor)?;
                failpoint(&request, "install_exchanged")?;
            } else if candidate_here
                && canonical_here
                && same_moved_observation(&canonical_path, &candidate_observation, request.issue)?
                && same_moved_observation(&candidate, &prior_observation, request.issue)?
            {
            } else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "canonical exchange state is ambiguous",
                ));
            }
        } else {
            if prior_path.symlink_metadata().is_ok() && displaced.symlink_metadata().is_err() {
                if !exact_observation(prior_path, &prior_observation, request.issue)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "prior source identity drifted before displacement",
                    ));
                }
                let prior_anchor = anchored_name(prior_path)?;
                if !anchored_root_identity_matches(&prior_anchor, &prior_observation)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "prior source identity drifted at displacement boundary",
                    ));
                }
                let displaced_anchor = anchored_name(&displaced)?;
                rename_no_replace(&prior_anchor, &displaced_anchor)?;
                sync(&issue_parent(store))?;
                sync(&attempt)?;
            } else if !(prior_path.symlink_metadata().is_err()
                && displaced.symlink_metadata().is_ok()
                && same_moved_observation(&displaced, &prior_observation, request.issue)?)
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "prior displacement state is ambiguous",
                ));
            }
            if candidate.symlink_metadata().is_ok() && canonical_path.symlink_metadata().is_err() {
                if !exact_observation(&candidate, &candidate_observation, request.issue)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "candidate identity drifted before install",
                    ));
                }
                let candidate_anchor = anchored_name(&candidate)?;
                if !anchored_root_identity_matches(&candidate_anchor, &candidate_observation)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "candidate identity drifted at install boundary",
                    ));
                }
                let canonical_anchor = anchored_name(&canonical_path)?;
                rename_no_replace(&candidate_anchor, &canonical_anchor)?;
                failpoint(&request, "candidate_installed")?;
            } else if !(candidate.symlink_metadata().is_err()
                && canonical_path.symlink_metadata().is_ok()
                && same_moved_observation(&canonical_path, &candidate_observation, request.issue)?)
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "canonical install state is ambiguous",
                ));
            }
        }
        sync(&issue_parent(store))?;
        sync(&attempt)?;
        receipt(
            &attempt,
            8,
            "canonical-installed",
            &serde_json::json!({"canonical":observe(&canonical_path,"canonical",request.issue)}),
        )?;
        failpoint(&request, "canonical_installed")?;
    }

    if install_exchange && !receipt_path(&attempt, 10, "prior-displaced").is_file() {
        receipt(
            &attempt,
            9,
            "displace-intent",
            &serde_json::json!({"source":prior_observation,"destination":"displaced"}),
        )?;
        failpoint(&request, "displace_intent")?;
        if candidate.symlink_metadata().is_ok() && displaced.symlink_metadata().is_err() {
            if !same_moved_observation(&candidate, &prior_observation, request.issue)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "post-exchange prior identity drifted before displacement",
                ));
            }
            let candidate_anchor = anchored_name(&candidate)?;
            if !anchored_moved_root_identity_matches(&candidate_anchor, &prior_observation)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "post-exchange prior identity drifted at displacement boundary",
                ));
            }
            let displaced_anchor = anchored_name(&displaced)?;
            rename_no_replace(&candidate_anchor, &displaced_anchor)?;
            if !same_moved_observation(&displaced, &prior_observation, request.issue)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "post-exchange displaced prior identity verification failed",
                ));
            }
            failpoint(&request, "prior_displaced_renamed")?;
        } else if !(candidate.symlink_metadata().is_err()
            && displaced.symlink_metadata().is_ok()
            && same_moved_observation(&displaced, &prior_observation, request.issue)?)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "displaced prior state is ambiguous",
            ));
        }
        sync(&attempt)?;
        receipt(
            &attempt,
            10,
            "prior-displaced",
            &serde_json::to_value(observe(&displaced, "displaced", request.issue))?,
        )?;
        failpoint(&request, "prior_displaced")?;
    } else if !install_exchange {
        receipt(
            &attempt,
            9,
            "displace-intent",
            &serde_json::json!({"source":prior_observation,"destination":"displaced","completed_before_install":true}),
        )?;
        receipt(
            &attempt,
            10,
            "prior-displaced",
            &serde_json::to_value(observe(&displaced, "displaced", request.issue))?,
        )?;
    }

    let canonical = observe(&canonical_path, "canonical", request.issue);
    if !canonical.valid_projection
        || canonical.generation != Some(candidate_record.generation)
        || canonical.record_digest.as_deref() != Some(&candidate_record.digest)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "installed canonical does not match verified candidate",
        ));
    }
    receipt(
        &attempt,
        11,
        "canonical-verified",
        &serde_json::json!({"canonical":canonical,"archive":archived,"displaced":observe(&displaced,"displaced",request.issue)}),
    )?;
    failpoint(&request, "canonical_verified")?;
    receipt(
        &attempt,
        12,
        "recovery-complete-intent",
        &serde_json::json!({"canonical_digest":candidate_record.digest,"generation":candidate_record.generation}),
    )?;
    failpoint(&request, "recovery_complete_intent")?;
    let mut out = ProjectionRecoveryResult {
        schema: "csdlc.projection_recovery_result.v1".into(),
        issue: request.issue,
        operation_id: request.operation_id,
        disposition: "recovered".into(),
        archived_manifest_digest: archived.manifest_digest.unwrap_or_default(),
        canonical_generation: candidate_record.generation,
        canonical_digest: candidate_record.digest,
        receipt_digest: String::new(),
    };
    out.receipt_digest = blake3::hash(&serde_json::to_vec(&out)?)
        .to_hex()
        .to_string();
    receipt(&attempt, 13, "recovered", &serde_json::to_value(&out)?)?;
    Ok(out)
}

fn classify_preserved_projection_unlocked(
    store: &Store,
    request: &ProjectionRecoverRequest,
    actor: &str,
    reason: &str,
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
        actor: actor.into(),
        reason: reason.into(),
        canonical,
        backup,
        preserved,
        disposition: disposition.into(),
        receipt_digest: String::new(),
    };
    v.receipt_digest = blake3::hash(&serde_json::to_vec(&v)?).to_hex().to_string();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anchored_identity_check_rejects_name_swap_after_anchor_creation() {
        let temp = tempfile::tempdir().expect("tempdir");
        let child = temp.path().join("candidate");
        fs::create_dir(&child).expect("candidate dir");
        fs::write(child.join("index.json"), "{}").expect("candidate file");
        let expected = observe(&child, "candidate", 298);
        let anchor = anchored_name(&child).expect("anchor");
        assert!(anchored_root_identity_matches(&anchor, &expected).expect("initial identity"));
        fs::rename(&child, temp.path().join("candidate.old")).expect("displace candidate");
        fs::create_dir(&child).expect("replacement candidate dir");
        assert!(!anchored_root_identity_matches(&anchor, &expected).expect("swapped identity"));
    }
}
