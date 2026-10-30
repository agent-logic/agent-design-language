use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{render, CardContent, CardKind, CardValues};
use crate::projection_cleanup::{
    ArchivedProjectionCleanupRequest, ArchivedProjectionNode, CleanupNodeIdentity, CleanupNodeType,
};
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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
    let bytes = read_receipt_file(path)?;
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
    let value: serde_json::Value = serde_json::from_slice(&bytes)?;
    let object = value.as_object().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope is not an object",
        )
    })?;
    for key in object.keys() {
        if !matches!(
            key.as_str(),
            "schema" | "sequence" | "state" | "previous_receipt_digest" | "payload"
        ) {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt envelope contains an unexpected field",
            ));
        }
    }
    if !object.contains_key("previous_receipt_digest") || !object.contains_key("payload") {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope is incomplete",
        ));
    }
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
            blake3::hash(&read_receipt_file(&prior[0].path())?)
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionRecoveryCleanupBridgeRequest {
    pub schema: String,
    pub issue: u64,
    pub recovery_operation_id: String,
    pub cleanup_issue: u64,
    pub cleanup_operation_id: String,
    pub repository_root: String,
    pub execution_base: String,
    pub terminal_issue: u64,
    pub terminal_envelope: String,
    pub expected_terminal_digest: String,
    pub expected_terminal_merge_sha: String,
    pub cleanup_ledger_root: String,
    pub branch: String,
    pub worktree: String,
    #[serde(default)]
    pub fail_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ProjectionRecoveryCleanupBridgeResult {
    pub schema: String,
    pub issue: u64,
    pub recovery_operation_id: String,
    pub cleanup_issue: u64,
    pub cleanup_operation_id: String,
    pub completed_recovery_receipt: String,
    pub expected_recovery_receipt_digest: String,
    pub canonical_archive_manifest: String,
    pub expected_archive_manifest_digest: String,
    pub archived_root: String,
    pub cleanup_ledger_root: String,
    pub nodes: Vec<ArchivedProjectionNode>,
    pub cleanup_request: ArchivedProjectionCleanupRequest,
}

fn issue_parent(store: &Store) -> PathBuf {
    store.root().join(".csdlc/issues")
}
fn recovery_root(store: &Store, issue: u64) -> PathBuf {
    issue_parent(store).join(format!(".{issue}.recovery"))
}

pub(crate) struct PrivateRecoveryDir {
    pub(crate) file: File,
}

impl PrivateRecoveryDir {
    pub(crate) fn names(&self) -> Result<Vec<std::ffi::CString>> {
        directory_names(&self.file)
    }

    pub(crate) fn open_child(
        &self,
        name: &std::ffi::CStr,
        label: &str,
    ) -> Result<PrivateRecoveryDir> {
        let file = open_child_no_follow(&self.file, name, true).map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} cannot be opened descriptor-relative without following links"),
            )
        })?;
        private_recovery_dir_from_file(file, label, Some(&self.file))
    }

    fn open_directory_child(&self, name: &std::ffi::CStr, label: &str) -> Result<File> {
        let file = open_child_no_follow(&self.file, name, true).map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} cannot be opened descriptor-relative without following links"),
            )
        })?;
        let identity = fd_identity(&file)?;
        let parent = fd_identity(&self.file)?;
        if identity.node_type != "directory"
            || identity.device != parent.device
            || identity.mount_id != parent.mount_id
            || identity.uid != parent.uid
            || identity.gid != parent.gid
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} identity is unsafe"),
            ));
        }
        Ok(file)
    }

    fn create_private_child(&self, name: &std::ffi::CStr) -> Result<()> {
        use std::os::fd::AsRawFd;
        if unsafe { libc::mkdirat(self.file.as_raw_fd(), name.as_ptr(), 0o700) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        self.file.sync_all()?;
        Ok(())
    }

    fn open_regular_child(&self, name: &std::ffi::CStr) -> Result<File> {
        let file = open_child_no_follow(&self.file, name, false).map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt cannot be opened descriptor-relative without following links",
            )
        })?;
        validate_receipt_metadata(&file.metadata()?, &self.file.metadata()?)?;
        Ok(file)
    }

    fn read_regular_child(&self, name: &std::ffi::CStr) -> Result<Vec<u8>> {
        let mut file = self.open_regular_child(name)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn create_regular_child(&self, name: &std::ffi::CStr, bytes: &[u8]) -> Result<()> {
        use std::os::fd::{AsRawFd, FromRawFd};
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let mut file = unsafe { File::from_raw_fd(fd) };
        file.write_all(bytes)?;
        file.sync_all()?;
        self.file.sync_all()?;
        Ok(())
    }

    fn open_writable_regular_child(&self, name: &std::ffi::CStr) -> Result<File> {
        use std::os::fd::{AsRawFd, FromRawFd};
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDWR | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        validate_receipt_metadata(&file.metadata()?, &self.file.metadata()?)?;
        Ok(file)
    }

    fn create_empty_regular_child(&self, name: &std::ffi::CStr) -> Result<File> {
        use std::os::fd::{AsRawFd, FromRawFd};
        let fd = unsafe {
            libc::openat(
                self.file.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        validate_receipt_metadata(&file.metadata()?, &self.file.metadata()?)?;
        Ok(file)
    }

    fn validate_binding(&self, path: &Path, label: &str) -> Result<()> {
        let current = open_private_recovery_dir(path, label, None)?;
        let retained = fd_identity(&self.file)?;
        let named = fd_identity(&current.file)?;
        if retained.device != named.device
            || retained.mount_id != named.mount_id
            || retained.inode != named.inode
            || retained.uid != named.uid
            || retained.gid != named.gid
            || retained.mode != named.mode
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("{label} pathname no longer names the retained directory"),
            ));
        }
        Ok(())
    }
}

fn open_or_create_private_child(
    parent: &PrivateRecoveryDir,
    name: &str,
    label: &str,
) -> Result<PrivateRecoveryDir> {
    let name = std::ffi::CString::new(name)
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "directory name contains NUL"))?;
    match parent.open_child(&name, label) {
        Ok(child) => Ok(child),
        Err(error)
            if error
                .message
                .contains("cannot be opened descriptor-relative") =>
        {
            use std::os::fd::AsRawFd;
            let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
            let result = unsafe {
                libc::fstatat(
                    parent.file.as_raw_fd(),
                    name.as_ptr(),
                    stat.as_mut_ptr(),
                    libc::AT_SYMLINK_NOFOLLOW,
                )
            };
            if result != 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::NotFound
            {
                parent.create_private_child(&name)?;
                parent.open_child(&name, label)
            } else {
                Err(error)
            }
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn open_private_recovery_dir(
    path: &Path,
    label: &str,
    expected_parent: Option<&File>,
) -> Result<PrivateRecoveryDir> {
    let file = open_root_no_follow(path).map_err(|_| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            format!("{label} cannot be opened without following links"),
        )
    })?;
    private_recovery_dir_from_file(file, label, expected_parent)
}

fn private_recovery_dir_from_file(
    file: File,
    label: &str,
    expected_parent: Option<&File>,
) -> Result<PrivateRecoveryDir> {
    let metadata = file.metadata()?;
    if !metadata.is_dir() || metadata.mode() & 0o777 != 0o700 {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            format!("{label} is not a private non-symlink directory"),
        ));
    }
    if let Some(parent) = expected_parent {
        let parent_metadata = parent.metadata()?;
        if metadata.uid() != parent_metadata.uid()
            || metadata.gid() != parent_metadata.gid()
            || metadata.dev() != parent_metadata.dev()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} owner or device does not match its parent"),
            ));
        }
    }
    Ok(PrivateRecoveryDir { file })
}

fn validate_receipt_metadata(
    metadata: &fs::Metadata,
    parent_metadata: &fs::Metadata,
) -> Result<()> {
    if !metadata.is_file() || metadata.nlink() != 1 || metadata.mode() & 0o777 != 0o600 {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt is not a private regular file",
        ));
    }
    if metadata.uid() != parent_metadata.uid()
        || metadata.gid() != parent_metadata.gid()
        || metadata.dev() != parent_metadata.dev()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt owner or device does not match its ledger",
        ));
    }
    Ok(())
}

fn read_receipt_file(path: &Path) -> Result<Vec<u8>> {
    read_receipt_file_after_open(path, || Ok(()))
}

fn read_receipt_file_after_open<F>(path: &Path, after_open: F) -> Result<Vec<u8>>
where
    F: FnOnce() -> Result<()>,
{
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let parent_path = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "recovery receipt has no ledger"))?;
    let parent = open_root_no_follow(parent_path).map_err(|_| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt ledger cannot be opened without following links",
        )
    })?;
    let name = path.file_name().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt has no file name",
        )
    })?;
    let name = CString::new(name.as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::CorruptRecord, "receipt name contains NUL"))?;
    let mut file = open_child_no_follow(&parent, &name, false).map_err(|_| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt cannot be opened without following links",
        )
    })?;
    after_open()?;
    let metadata = file.metadata()?;
    let parent_metadata = parent.metadata()?;
    validate_receipt_metadata(&metadata, &parent_metadata)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
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

fn inject_post_validation_directory_swap(
    request: &ProjectionRecoverRequest,
    state: &str,
    path: &Path,
) -> Result<()> {
    if request.fail_after.as_deref() != Some(state) {
        return Ok(());
    }
    let displaced = path.with_extension(format!("{state}.displaced"));
    fs::rename(path, &displaced)?;
    fs::create_dir(path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    Ok(())
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
    use std::os::fd::AsRawFd;
    #[cfg(target_os = "linux")]
    {
        use std::mem::MaybeUninit;
        let mut statx = MaybeUninit::<libc::statx>::zeroed();
        let empty = b"\0";
        if unsafe {
            libc::statx(
                file.as_raw_fd(),
                empty.as_ptr().cast(),
                libc::AT_EMPTY_PATH | libc::AT_NO_AUTOMOUNT,
                libc::STATX_MNT_ID,
                statx.as_mut_ptr(),
            )
        } != 0
        {
            return Err(std::io::Error::last_os_error().into());
        }
        let statx = unsafe { statx.assume_init() };
        linux_mount_id_from_statx(statx.stx_mask, statx.stx_mnt_id)
    }
    #[cfg(not(target_os = "linux"))]
    use std::mem::MaybeUninit;
    #[cfg(not(target_os = "linux"))]
    let mut stat = MaybeUninit::<libc::statfs>::uninit();
    #[cfg(not(target_os = "linux"))]
    if unsafe { libc::fstatfs(file.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    #[cfg(not(target_os = "linux"))]
    let stat = unsafe { stat.assume_init() };
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        let words: [i32; 2] = unsafe { std::mem::transmute_copy(&stat.f_fsid) };
        Ok(format!("{}:{}", words[0], words[1]))
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd"
    )))]
    {
        Ok(format!("dev:{}", file.metadata()?.dev()))
    }
}

#[cfg(target_os = "linux")]
fn linux_mount_id_from_statx(mask: u32, mount_id: u64) -> Result<String> {
    if mask & libc::STATX_MNT_ID == 0 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "retained descriptor does not expose a Linux mount identity",
        ));
    }
    Ok(format!("mnt:{mount_id}"))
}

fn open_root_no_follow(path: &Path) -> Result<File> {
    use std::ffi::CString;
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    let path_cstr = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "path contains NUL"))?;
    let fd = unsafe {
        libc::open(
            path_cstr.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
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
        | if directory { 0 } else { libc::O_NONBLOCK }
        | if directory { libc::O_DIRECTORY } else { 0 };
    let fd = unsafe { libc::openat(parent.as_raw_fd(), name.as_ptr(), flags) };
    if fd < 0 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn directory_names(directory: &File) -> Result<Vec<std::ffi::CString>> {
    use std::os::fd::AsRawFd;
    let dup = unsafe { libc::dup(directory.as_raw_fd()) };
    if dup < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    if unsafe { libc::lseek(dup, 0, libc::SEEK_SET) } < 0 {
        unsafe { libc::close(dup) };
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
    walk_file(root)
}

fn walk_file(root: File) -> Result<Vec<ManifestEntry>> {
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

fn open_relative_no_follow(root: &File, relative: &str, directory: bool) -> Result<File> {
    let mut current = root.try_clone()?;
    let components: Vec<_> = Path::new(relative).components().collect();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "descriptor-relative path is not a safe relative path",
        ));
    }
    for (index, component) in components.iter().enumerate() {
        let std::path::Component::Normal(name) = component else {
            unreachable!()
        };
        let name = std::ffi::CString::new(name.as_bytes())
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "relative path contains NUL"))?;
        current =
            open_child_no_follow(&current, &name, index + 1 != components.len() || directory)?;
    }
    Ok(current)
}

fn read_relative_no_follow(root: &File, relative: &str) -> Result<Vec<u8>> {
    let mut file = open_relative_no_follow(root, relative, false)?;
    let identity = fd_identity(&file)?;
    if identity.node_type != "regular" || identity.links != 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "projection artifact is not a unique regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn projection_source_files(root: &File) -> Result<BTreeMap<String, Vec<u8>>> {
    use strum::IntoEnumIterator;
    let mut files = BTreeMap::new();
    files.insert(
        "index.json".into(),
        read_relative_no_follow(root, "index.json")?,
    );
    files.insert(
        "audit.jsonl".into(),
        read_relative_no_follow(root, "audit.jsonl")?,
    );
    for kind in CardKind::iter() {
        for suffix in ["values.json", "md"] {
            let relative = format!("cards/{kind}.{suffix}");
            files.insert(relative.clone(), read_relative_no_follow(root, &relative)?);
        }
    }
    Ok(files)
}

fn read_file_bytes(file: &File) -> Result<Vec<u8>> {
    use std::os::unix::fs::FileExt;
    let size: usize = file
        .metadata()?
        .len()
        .try_into()
        .map_err(|_| V2Error::new(ErrorCode::CorruptRecord, "file is too large"))?;
    let mut bytes = vec![0; size];
    let mut offset = 0;
    while offset < size {
        let count = file.read_at(&mut bytes[offset..], offset as u64)?;
        if count == 0 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "file changed while being read",
            ));
        }
        offset += count;
    }
    Ok(bytes)
}

fn validate_projection(path: &Path, issue: u64) -> Result<IssueRecord> {
    let root = open_root_no_follow(path)?;
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
    validate_projection_file(&root, repo_root, issue)
}

fn validate_projection_file(root: &File, repo_root: &Path, issue: u64) -> Result<IssueRecord> {
    use strum::IntoEnumIterator;
    let record: IssueRecord =
        serde_json::from_slice(&read_relative_no_follow(root, "index.json")?)?;
    if record.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "projection namespace mismatch",
        ));
    }
    crate::store::verify_record(&record)?;
    let mut cards: BTreeMap<CardKind, CardValues> = BTreeMap::new();
    for kind in CardKind::iter() {
        let values: CardValues = serde_json::from_slice(&read_relative_no_follow(
            root,
            &format!("cards/{kind}.values.json"),
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
            || read_relative_no_follow(root, &format!("cards/{kind}.md"))?
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
    let audit = read_relative_no_follow(root, "audit.jsonl")?;
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

fn observe_file(file: &File, repo_root: &Path, name: &str, issue: u64) -> CandidateObservation {
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
    let entries_result: Result<Vec<ManifestEntry>> =
        file.try_clone().map_err(Into::into).and_then(walk_file);
    let entries = match entries_result {
        Ok(entries) => entries,
        Err(error) => {
            return CandidateObservation {
                state: "unsafe".into(),
                error: Some(error.message),
                ..absent
            }
        }
    };
    let manifest_digest = blake3::hash(&serde_json::to_vec(&entries).unwrap_or_default())
        .to_hex()
        .to_string();
    let validation = validate_projection_file(file, repo_root, issue);
    let record = validation.as_ref().ok();
    let valid = record.is_some();
    CandidateObservation {
        name: name.into(),
        state: if valid { "verified" } else { "invalid" }.into(),
        valid_projection: valid,
        generation: record.as_ref().map(|record| record.generation),
        record_digest: record.as_ref().map(|record| record.digest.clone()),
        manifest_digest: Some(manifest_digest),
        entries,
        error: validation.err().map(|error| error.message),
    }
}

fn observe_child(
    parent: &PrivateRecoveryDir,
    child: &str,
    repo_root: &Path,
    name: &str,
    issue: u64,
) -> Result<CandidateObservation> {
    let child = parent.open_directory_child(
        &std::ffi::CString::new(child)
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "child name contains NUL"))?,
        name,
    )?;
    Ok(observe_file(&child, repo_root, name, issue))
}

fn retained_matching_projection(
    attempt: &PrivateRecoveryDir,
    current_path: &Path,
    expected: &CandidateObservation,
    repo_root: &Path,
    issue: u64,
) -> Result<File> {
    if let Ok(file) = open_root_no_follow(current_path) {
        let observed = observe_file(&file, repo_root, "prior", issue);
        if observations_match_after_move(&observed, expected) {
            return Ok(file);
        }
    }
    for name in ["displaced", "candidate"] {
        let component = std::ffi::CString::new(name).unwrap();
        if let Ok(file) = attempt.open_directory_child(&component, "retained prior source") {
            let observed = observe_file(&file, repo_root, "prior", issue);
            if observations_match_after_move(&observed, expected) {
                return Ok(file);
            }
        }
    }
    Err(V2Error::new(
        ErrorCode::ReconciliationRequired,
        "authorized retained recovery prior source is unavailable",
    ))
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
    let recovery = match recovery.symlink_metadata() {
        Ok(_) => {
            let issues = open_root_no_follow(&issue_parent(store))?;
            Some(open_private_recovery_dir(
                &recovery,
                "recovery root",
                Some(&issues),
            )?)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    if let Some(recovery) = recovery {
        for name in recovery.names()? {
            let name_text = name.to_str().map_err(|_| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery namespace contains a non-UTF8 entry",
                )
            })?;
            validate_operation_id(name_text)?;
            let attempt = match recovery.open_child(&name, "recovery attempt") {
                Ok(attempt) => attempt,
                Err(error) => {
                    if is_authorized_cleanup_ledger_entry(
                        store,
                        request.issue,
                        &recovery,
                        name_text,
                    )? {
                        continue;
                    }
                    return Err(error);
                }
            };
            let recovered: Vec<_> = directory_names(&attempt.file)?
                .into_iter()
                .filter(|v| v.to_string_lossy().ends_with("-recovered.json"))
                .collect();
            if recovered.len() > 1 {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery attempt has ambiguous terminal receipts",
                ));
            }
            if !recovered.is_empty() {
                let result = validate_completed_recovery_attempt_retained(
                    store,
                    request.issue,
                    &attempt,
                    name_text,
                )?;
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
#[allow(dead_code)] // Removed with the remaining pathname-based completed-ledger validator.
fn receipt(dir: &Path, seq: u32, state: &str, value: &serde_json::Value) -> Result<String> {
    use std::ffi::CString;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
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
            blake3::hash(&read_receipt_file(&prior[0].path())?)
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
    if receipt_exists(dir, seq, state)? {
        let existing = read_receipt_file(&p)?;
        if existing == bytes {
            return Ok(blake3::hash(&bytes).to_hex().to_string());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "immutable recovery receipt collision",
        ));
    }
    let parent = open_root_no_follow(dir)?;
    let name = CString::new(p.file_name().unwrap().as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0o600,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut f = unsafe { File::from_raw_fd(fd) };
    f.write_all(&bytes)?;
    f.sync_all()?;
    sync(dir)?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn receipt_path(dir: &Path, seq: u32, state: &str) -> PathBuf {
    dir.join(format!("{seq:03}-{state}.json"))
}

fn receipt_name(seq: u32, state: &str) -> Result<std::ffi::CString> {
    std::ffi::CString::new(format!("{seq:03}-{state}.json"))
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))
}

fn receipt_exists_at(dir: &PrivateRecoveryDir, seq: u32, state: &str) -> Result<bool> {
    let name = receipt_name(seq, state)?;
    match dir.open_regular_child(&name) {
        Ok(_) => Ok(true),
        Err(error)
            if error
                .message
                .contains("cannot be opened descriptor-relative") =>
        {
            use std::os::fd::AsRawFd;
            let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
            let result = unsafe {
                libc::fstatat(
                    dir.file.as_raw_fd(),
                    name.as_ptr(),
                    stat.as_mut_ptr(),
                    libc::AT_SYMLINK_NOFOLLOW,
                )
            };
            if result != 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::NotFound
            {
                Ok(false)
            } else {
                Err(error)
            }
        }
        Err(error) => Err(error),
    }
}

fn receipt_payload_at<T: serde::de::DeserializeOwned>(
    dir: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
) -> Result<T> {
    let name = receipt_name(seq, state)?;
    let bytes = dir.read_regular_child(&name)?;
    let envelope: serde_json::Value = serde_json::from_slice(&bytes)?;
    let expected_previous = if seq == 1 {
        None
    } else {
        let prefix = format!("{:03}-", seq - 1);
        let names: Vec<_> = dir
            .names()?
            .into_iter()
            .filter(|name| name.to_bytes().starts_with(prefix.as_bytes()))
            .collect();
        if names.len() != 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt chain predecessor is ambiguous",
            ));
        }
        Some(
            blake3::hash(&dir.read_regular_child(&names[0])?)
                .to_hex()
                .to_string(),
        )
    };
    if envelope
        .get("previous_receipt_digest")
        .and_then(|value| value.as_str())
        != expected_previous.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt chain digest is invalid",
        ));
    }
    receipt_payload_bytes(&name.to_string_lossy(), &bytes)
}

fn receipt_payload_bytes<T: serde::de::DeserializeOwned>(name: &str, bytes: &[u8]) -> Result<T> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let object = value.as_object().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope is not an object",
        )
    })?;
    if object.keys().any(|key| {
        !matches!(
            key.as_str(),
            "schema" | "sequence" | "state" | "previous_receipt_digest" | "payload"
        )
    }) || !object.contains_key("previous_receipt_digest")
        || !object.contains_key("payload")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope is invalid",
        ));
    }
    let expected_name = format!(
        "{:03}-{}.json",
        value["sequence"].as_u64().unwrap_or(0),
        value["state"].as_str().unwrap_or("")
    );
    if name != expected_name
        || value.get("schema").and_then(|value| value.as_str())
            != Some("csdlc.projection_recovery_receipt.v1")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt envelope identity is invalid",
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

fn receipt_at(
    dir: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
    value: &serde_json::Value,
) -> Result<String> {
    let previous_digest = if seq == 1 {
        None
    } else {
        let prefix = format!("{:03}-", seq - 1);
        let names: Vec<_> = dir
            .names()?
            .into_iter()
            .filter(|name| name.to_bytes().starts_with(prefix.as_bytes()))
            .collect();
        if names.len() != 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt chain predecessor is missing or ambiguous",
            ));
        }
        let bytes = dir.read_regular_child(&names[0])?;
        Some(blake3::hash(&bytes).to_hex().to_string())
    };
    let envelope = serde_json::json!({"schema":"csdlc.projection_recovery_receipt.v1","sequence":seq,"state":state,"previous_receipt_digest":previous_digest,"payload":value});
    let mut bytes = serde_json::to_vec_pretty(&envelope)?;
    bytes.push(b'\n');
    let name = receipt_name(seq, state)?;
    if receipt_exists_at(dir, seq, state)? {
        if dir.read_regular_child(&name)? == bytes {
            return Ok(blake3::hash(&bytes).to_hex().to_string());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "immutable recovery receipt collision",
        ));
    }
    dir.create_regular_child(&name, &bytes)?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

#[allow(dead_code)] // Removed with the remaining pathname-based completed-ledger validator.
fn receipt_exists(dir: &Path, seq: u32, state: &str) -> Result<bool> {
    use std::ffi::CString;
    let parent = open_root_no_follow(dir)?;
    let name = CString::new(format!("{seq:03}-{state}.json"))
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))?;
    match open_child_no_follow(&parent, &name, false) {
        Ok(_) => Ok(true),
        Err(error) if error.to_string().contains("No such file") => Ok(false),
        Err(_) => Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt cannot be inspected without following links",
        )),
    }
}

#[allow(dead_code)] // Removed with the remaining pathname-based node-ledger validator.
fn validate_receipt_inventory(dir: &Path) -> Result<usize> {
    let mut receipts = Vec::new();
    let mut node_ledgers = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry.map_err(|error| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("recovery receipt inventory cannot be enumerated: {error}"),
            )
        })?;
        let file_type = entry.file_type().map_err(|error| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("recovery receipt type cannot be read: {error}"),
            )
        })?;
        if file_type.is_file() {
            let _ = read_receipt_file(&entry.path())?;
            receipts.push(entry);
            continue;
        }
        let name = entry.file_name();
        let name = name.to_str().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery workspace entry name is not UTF-8",
            )
        })?;
        if file_type.is_dir() && matches!(name, "candidate" | "displaced" | "rejected") {
            continue;
        }
        let node_ordinal = file_type.is_dir().then(|| {
            name.strip_prefix("node-")
                .filter(|suffix| suffix.len() == 3)
                .and_then(|suffix| suffix.parse::<usize>().ok())
        });
        if let Some(Some(ordinal)) = node_ordinal {
            node_ledgers.push(ordinal);
        } else {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt inventory contains an unexpected workspace entry",
            ));
        }
    }
    receipts.sort_by_key(|entry| entry.file_name());
    if receipts.len() != RECOVERY_STATES.len() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt inventory is not the exact terminal sequence",
        ));
    }
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
        let expected_name = format!("{seq:03}-{}.json", RECOVERY_STATES[index]);
        if name != expected_name {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt inventory contains an unexpected state",
            ));
        }
        let _: serde_json::Value = receipt_payload(&entry.path())?;
    }
    node_ledgers.sort_unstable();
    if node_ledgers
        .iter()
        .enumerate()
        .any(|(expected, actual)| expected != *actual)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery node ledger inventory is not an exact prefix",
        ));
    }
    Ok(node_ledgers.len())
}

fn validate_receipt_inventory_at(dir: &PrivateRecoveryDir) -> Result<usize> {
    let mut receipt_names = Vec::new();
    let mut node_ledgers = Vec::new();
    for name in dir.names()? {
        let text = name.to_str().map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery workspace entry name is not UTF-8",
            )
        })?;
        if text.ends_with(".json") {
            let _ = dir.open_regular_child(&name)?;
            receipt_names.push(text.to_owned());
            continue;
        }
        if matches!(text, "candidate" | "displaced" | "rejected") {
            let child = open_child_no_follow(&dir.file, &name, true).map_err(|_| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery workspace directory cannot be opened descriptor-relative",
                )
            })?;
            let identity = fd_identity(&child)?;
            let parent = fd_identity(&dir.file)?;
            if identity.node_type != "directory"
                || identity.device != parent.device
                || identity.mount_id != parent.mount_id
                || identity.uid != parent.uid
                || identity.gid != parent.gid
            {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recovery workspace directory identity is unsafe",
                ));
            }
            continue;
        }
        if text == "cleanup-authority" {
            validate_cleanup_authority_namespace(dir, &name)?;
            continue;
        }
        if let Some(ordinal) = text
            .strip_prefix("node-")
            .filter(|suffix| suffix.len() == 3)
            .and_then(|suffix| suffix.parse::<usize>().ok())
        {
            let _ = dir.open_child(&name, "recovery node ledger")?;
            node_ledgers.push(ordinal);
            continue;
        }
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt inventory contains an unexpected workspace entry",
        ));
    }
    receipt_names.sort();
    if receipt_names.len() != RECOVERY_STATES.len() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery receipt inventory is not the exact terminal sequence",
        ));
    }
    for (index, state) in RECOVERY_STATES.iter().enumerate() {
        let seq = index as u32 + 1;
        if receipt_names[index] != format!("{seq:03}-{state}.json") {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery receipt inventory contains an unexpected state",
            ));
        }
        let _: serde_json::Value = receipt_payload_at(dir, seq, state)?;
    }
    node_ledgers.sort_unstable();
    if node_ledgers
        .iter()
        .enumerate()
        .any(|(expected, actual)| expected != *actual)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery node ledger inventory is not an exact prefix",
        ));
    }
    Ok(node_ledgers.len())
}

#[allow(dead_code)] // Removed with the remaining pathname validator helpers.
fn validate_exact_receipt_files(dir: &Path, states: &[&str], label: &str) -> Result<()> {
    let mut receipts = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry.map_err(|error| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt inventory cannot be enumerated: {error}"),
            )
        })?;
        let file_type = entry.file_type().map_err(|error| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt type cannot be read: {error}"),
            )
        })?;
        if !file_type.is_file() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt inventory contains a non-regular entry"),
            ));
        }
        let _ = read_receipt_file(&entry.path())?;
        receipts.push(entry);
    }
    receipts.sort_by_key(|entry| entry.file_name());
    if receipts.len() != states.len() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            format!("{label} receipt inventory is not the exact terminal sequence"),
        ));
    }
    for (index, entry) in receipts.iter().enumerate() {
        let name = entry.file_name();
        let name = name.to_str().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt name is not UTF-8"),
            )
        })?;
        let seq: usize = name.get(..3).and_then(|v| v.parse().ok()).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt name has no sequence"),
            )
        })?;
        if seq != index + 1 {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt inventory is not a unique exact prefix"),
            ));
        }
        let expected_name = format!("{seq:03}-{}.json", states[index]);
        if name != expected_name {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt inventory contains an unexpected state"),
            ));
        }
        let _: serde_json::Value = receipt_payload(&entry.path())?;
    }
    Ok(())
}

fn validate_exact_receipt_files_at(
    dir: &PrivateRecoveryDir,
    states: &[&str],
    label: &str,
) -> Result<()> {
    let mut names = dir.names()?;
    names.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    if names.len() != states.len() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            format!("{label} receipt inventory is not the exact terminal sequence"),
        ));
    }
    for (index, state) in states.iter().enumerate() {
        let seq = index as u32 + 1;
        if names[index].to_bytes() != format!("{seq:03}-{state}.json").as_bytes() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{label} receipt inventory contains an unexpected state"),
            ));
        }
        let _: serde_json::Value = receipt_payload_at(dir, seq, state)?;
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

const NODE_STATES: [&str; 10] = [
    "create-intent",
    "created-identity",
    "write-intent",
    "write-completed",
    "node-fsync-intent",
    "node-fsync-completed",
    "parent-fsync-intent",
    "parent-fsync-completed",
    "publish-intent",
    "node-created",
];

#[allow(dead_code)] // Removed with the remaining pathname validator helpers.
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

fn require_receipt_payload_at(
    attempt: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
    expected: serde_json::Value,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload_at(attempt, seq, state)?;
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

fn observed_entry<'a>(
    observation: &'a CandidateObservation,
    relative: &str,
) -> Result<&'a ManifestEntry> {
    observation
        .entries
        .iter()
        .find(|entry| entry.path == relative)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "candidate observation is missing an authorized node",
            )
        })
}

#[allow(dead_code)] // Removed with the remaining pathname validator helpers.
fn require_node_receipt_payload(
    ledger: &Path,
    seq: u32,
    state: &str,
    expected: serde_json::Value,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload(&receipt_path(ledger, seq, state))?;
    if actual == expected {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn stable_directory_identity_matches(actual: &NodeIdentity, expected: &NodeIdentity) -> bool {
    actual.device == expected.device
        && actual.mount_id == expected.mount_id
        && actual.inode == expected.inode
        && actual.uid == expected.uid
        && actual.gid == expected.gid
        && actual.mode == expected.mode
        && actual.node_type == expected.node_type
}

#[allow(dead_code)] // Removed with the remaining pathname validator helpers.
fn require_directory_identity_receipt(
    ledger: &Path,
    seq: u32,
    state: &str,
    expected: &NodeIdentity,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload(&receipt_path(ledger, seq, state))?;
    let identity = if let Some(identity) = actual.get("identity") {
        serde_json::from_value(identity.clone())?
    } else {
        serde_json::from_value(actual)?
    };
    // Directory ctime and link count legitimately evolve while the exact authorized child
    // manifest is constructed. The caller separately binds the complete candidate manifest,
    // exact node-ledger count, contiguous ordinals, and every per-node receipt sequence.
    if stable_directory_identity_matches(&identity, expected) {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn stable_file_identity_matches(actual: &NodeIdentity, expected: &NodeIdentity) -> bool {
    actual.device == expected.device
        && actual.mount_id == expected.mount_id
        && actual.inode == expected.inode
        && actual.links == expected.links
        && actual.uid == expected.uid
        && actual.gid == expected.gid
        && actual.mode == expected.mode
        && actual.node_type == expected.node_type
}

#[allow(dead_code)] // Removed with the remaining pathname validator helpers.
fn require_file_identity_receipt(
    ledger: &Path,
    seq: u32,
    state: &str,
    expected: &NodeIdentity,
    digest: &str,
    size: Option<usize>,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload(&receipt_path(ledger, seq, state))?;
    let identity: NodeIdentity =
        serde_json::from_value(actual.get("identity").cloned().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "candidate file identity receipt is missing identity",
            )
        })?)?;
    if stable_file_identity_matches(&identity, expected)
        && actual.get("digest").and_then(|v| v.as_str()) == Some(digest)
        && size.is_none_or(|size| actual.get("size").and_then(|v| v.as_u64()) == Some(size as u64))
    {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn require_node_receipt_payload_at(
    ledger: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
    expected: serde_json::Value,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload_at(ledger, seq, state)?;
    if actual == expected {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn require_directory_identity_receipt_at(
    ledger: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
    expected: &NodeIdentity,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload_at(ledger, seq, state)?;
    let identity: NodeIdentity =
        serde_json::from_value(actual.get("identity").cloned().unwrap_or(actual))?;
    if stable_directory_identity_matches(&identity, expected) {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn require_file_identity_receipt_at(
    ledger: &PrivateRecoveryDir,
    seq: u32,
    state: &str,
    expected: &NodeIdentity,
    digest: &str,
    size: Option<usize>,
    message: &str,
) -> Result<()> {
    let actual: serde_json::Value = receipt_payload_at(ledger, seq, state)?;
    let identity: NodeIdentity =
        serde_json::from_value(actual.get("identity").cloned().ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "candidate file identity receipt is missing identity",
            )
        })?)?;
    if stable_file_identity_matches(&identity, expected)
        && actual.get("digest").and_then(|v| v.as_str()) == Some(digest)
        && size.is_none_or(|size| actual.get("size").and_then(|v| v.as_u64()) == Some(size as u64))
    {
        Ok(())
    } else {
        Err(V2Error::new(ErrorCode::CorruptRecord, message))
    }
}

fn validate_directory_node_ledger(
    attempt_authority: &PrivateRecoveryDir,
    candidate_observation: &CandidateObservation,
    relative: &str,
    ordinal: usize,
) -> Result<()> {
    let ledger_authority = attempt_authority.open_child(
        &std::ffi::CString::new(format!("node-{ordinal:03}")).unwrap(),
        "recovery node ledger",
    )?;
    validate_exact_receipt_files_at(&ledger_authority, &NODE_STATES, "recovery node")?;
    let entry = observed_entry(candidate_observation, relative)?;
    if entry.identity.node_type != "directory" {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate directory node observation is not a directory",
        ));
    }
    require_node_receipt_payload_at(
        &ledger_authority,
        1,
        "create-intent",
        serde_json::json!({"path":relative,"type":"directory","mode":"0700"}),
        "candidate directory create intent does not match authorized node",
    )?;
    require_directory_identity_receipt_at(
        &ledger_authority,
        2,
        "created-identity",
        &entry.identity,
        "candidate directory created identity does not match observed node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        3,
        "write-intent",
        serde_json::json!({"directory":relative,"metadata_only":true}),
        "candidate directory write intent does not match authorized node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        4,
        "write-completed",
        serde_json::json!({"directory":relative}),
        "candidate directory write completion does not match authorized node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        5,
        "node-fsync-intent",
        serde_json::json!({"directory":relative}),
        "candidate directory fsync intent does not match authorized node",
    )?;
    require_directory_identity_receipt_at(
        &ledger_authority,
        6,
        "node-fsync-completed",
        &entry.identity,
        "candidate directory fsync completion does not match observed node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        7,
        "parent-fsync-intent",
        serde_json::json!({"directory":relative}),
        "candidate directory parent fsync intent does not match authorized node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        8,
        "parent-fsync-completed",
        serde_json::json!({"directory":relative}),
        "candidate directory parent fsync completion does not match authorized node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        9,
        "publish-intent",
        serde_json::json!({"directory":relative,"already_at_final_name":true}),
        "candidate directory publish intent does not match authorized node",
    )?;
    require_directory_identity_receipt_at(
        &ledger_authority,
        10,
        "node-created",
        &entry.identity,
        "candidate directory terminal receipt does not match observed node",
    )
}

fn validate_file_node_ledger(
    attempt_authority: &PrivateRecoveryDir,
    attempt: &Path,
    candidate_observation: &CandidateObservation,
    relative: &str,
    contents: &[u8],
    ordinal: usize,
) -> Result<()> {
    let ledger_authority = attempt_authority.open_child(
        &std::ffi::CString::new(format!("node-{ordinal:03}")).unwrap(),
        "recovery node ledger",
    )?;
    validate_exact_receipt_files_at(&ledger_authority, &NODE_STATES, "recovery node")?;
    let entry = observed_entry(candidate_observation, relative)?;
    let digest = blake3::hash(contents).to_hex().to_string();
    let size = contents.len();
    let parent = attempt
        .join("candidate")
        .join(relative)
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "candidate file has no parent"))?
        .to_path_buf();
    if entry.identity.node_type != "regular"
        || entry.digest.as_deref() != Some(digest.as_str())
        || entry.size != size as u64
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate file node observation does not match authorized contents",
        ));
    }
    let create_intent: serde_json::Value =
        receipt_payload_at(&ledger_authority, 1, "create-intent")?;
    if create_intent.get("temporary").is_none()
        || create_intent.get("final").and_then(|v| v.as_str()) != Some(relative)
        || create_intent.get("type").and_then(|v| v.as_str()) != Some("regular")
        || create_intent.get("mode").and_then(|v| v.as_str()) != Some("0600")
        || create_intent.get("digest").and_then(|v| v.as_str()) != Some(digest.as_str())
        || create_intent.get("size").and_then(|v| v.as_u64()) != Some(size as u64)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate file create intent does not match authorized node",
        ));
    }
    let created_identity: NodeIdentity =
        receipt_payload_at(&ledger_authority, 2, "created-identity")?;
    if created_identity.node_type != "regular"
        || created_identity.links != 1
        || created_identity.mode & 0o777 != 0o600
        || !stable_file_identity_matches(&created_identity, &entry.identity)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate file created identity is not a safe regular file",
        ));
    }
    require_node_receipt_payload_at(
        &ledger_authority,
        3,
        "write-intent",
        serde_json::json!({"digest":digest,"size":size}),
        "candidate file write intent does not match authorized contents",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        4,
        "write-completed",
        serde_json::json!({"digest":digest,"size":size}),
        "candidate file write completion does not match authorized contents",
    )?;
    let fsync_intent: serde_json::Value =
        receipt_payload_at(&ledger_authority, 5, "node-fsync-intent")?;
    if fsync_intent.get("temporary").is_none() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate file fsync intent does not match authorized node",
        ));
    }
    require_file_identity_receipt_at(
        &ledger_authority,
        6,
        "node-fsync-completed",
        &entry.identity,
        &digest,
        None,
        "candidate file fsync completion does not match observed node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        7,
        "parent-fsync-intent",
        serde_json::json!({"parent":parent}),
        "candidate file parent fsync intent does not match authorized node",
    )?;
    require_node_receipt_payload_at(
        &ledger_authority,
        8,
        "parent-fsync-completed",
        serde_json::json!({"parent":parent}),
        "candidate file parent fsync completion does not match authorized node",
    )?;
    let publish_intent: serde_json::Value =
        receipt_payload_at(&ledger_authority, 9, "publish-intent")?;
    if publish_intent.get("temporary").is_none()
        || publish_intent.get("final").and_then(|v| v.as_str()) != Some(relative)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate file publish intent does not match authorized node",
        ));
    }
    require_file_identity_receipt_at(
        &ledger_authority,
        10,
        "node-created",
        &entry.identity,
        &digest,
        Some(size),
        "candidate file terminal receipt does not match observed node",
    )
}

fn validate_classification_authority(
    classification: &ProjectionClassification,
    expected_digest: &str,
    issue: u64,
) -> Result<()> {
    let mut digest_input = classification.clone();
    digest_input.receipt_digest.clear();
    let digest = blake3::hash(&serde_json::to_vec(&digest_input)?)
        .to_hex()
        .to_string();
    if classification.schema != "csdlc.projection_classification.v1"
        || classification.issue != issue
        || classification.receipt_digest != expected_digest
        || classification.receipt_digest != digest
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "classification authority digest invalid",
        ));
    }
    Ok(())
}

pub(crate) fn validate_completed_recovery_attempt(
    store: &Store,
    issue: u64,
    attempt: &Path,
) -> Result<ProjectionRecoveryResult> {
    let parent = open_root_no_follow(attempt.parent().ok_or_else(|| {
        V2Error::new(ErrorCode::CorruptRecord, "recovery attempt has no parent")
    })?)?;
    let retained_attempt = open_private_recovery_dir(attempt, "recovery attempt", Some(&parent))?;
    let operation = attempt
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "attempt operation identity invalid",
            )
        })?
        .to_owned();
    validate_completed_recovery_attempt_retained(store, issue, &retained_attempt, &operation)
}

fn validate_completed_recovery_attempt_retained(
    store: &Store,
    issue: u64,
    retained_attempt: &PrivateRecoveryDir,
    operation: &str,
) -> Result<ProjectionRecoveryResult> {
    let operation = operation.to_owned();
    let attempt_path = recovery_root(store, issue).join(&operation);
    let node_ledger_count = validate_receipt_inventory_at(retained_attempt)?;
    let prepared: serde_json::Value = receipt_payload_at(retained_attempt, 1, "prepared")?;
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
        || prepared_request.operation_id != operation
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
    validate_classification_authority(
        &prepared_classification,
        &prepared_request.classify_receipt_digest,
        issue,
    )?;
    if prepared_request.classification != prepared_classification
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
    require_receipt_payload_at(
        retained_attempt,
        2,
        "archive-intent",
        serde_json::json!({"source":rejected_observation,"destination":"rejected"}),
        "archive intent receipt does not match typed recovery sources",
    )?;
    let archived: CandidateObservation =
        receipt_payload_at(retained_attempt, 3, "rejected-archived")?;
    match observe_child(retained_attempt, "rejected", store.root(), "archive", issue) {
        Ok(live_archived) => {
            if live_archived != archived {
                validate_completed_recovery_cleanup_authority(
                    store,
                    issue,
                    retained_attempt,
                    &operation,
                    &attempt_path,
                    &archived,
                )?;
            }
        }
        Err(_) => validate_completed_recovery_cleanup_authority(
            store,
            issue,
            retained_attempt,
            &operation,
            &attempt_path,
            &archived,
        )?,
    }
    require_receipt_payload_at(
        retained_attempt,
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
    require_receipt_payload_at(
        retained_attempt,
        4,
        "candidate-plan",
        serde_json::json!({"source":prior_observation,"destination":"candidate","audit":audit_commitment}),
        "candidate plan receipt does not match typed recovery audit",
    )?;
    let candidate_created: serde_json::Value =
        receipt_payload_at(retained_attempt, 5, "candidate-created")?;
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
    let prior_digest = prior_observation
        .record_digest
        .as_deref()
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "PREPARED prior digest missing"))?;
    let displaced_authority = retained_attempt.open_directory_child(
        &std::ffi::CString::new("displaced").unwrap(),
        "displaced recovery source",
    )?;
    let displaced_source = projection_source_files(&displaced_authority)?;
    let (derived_record, derived_files) = store
        .projection_recovery_candidate_files_from_bytes_locked(
            issue,
            &displaced_source,
            prior_digest,
            prepared_request.actor.clone(),
            prepared_request.reason.clone(),
            audit_commitment.to_string(),
        )?;
    if node_ledger_count != derived_files.len() + 2 {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery node ledger inventory does not match the authorized candidate",
        ));
    }
    validate_directory_node_ledger(retained_attempt, &candidate_observation, ".", 0)?;
    validate_directory_node_ledger(retained_attempt, &candidate_observation, "cards", 1)?;
    for (index, (relative, contents)) in derived_files.iter().enumerate() {
        validate_file_node_ledger(
            retained_attempt,
            &attempt_path,
            &candidate_observation,
            relative,
            contents,
            index + 2,
        )?;
    }
    if candidate_record != derived_record {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate-created receipt does not match authorized recovery transform",
        ));
    }
    require_receipt_payload_at(
        retained_attempt,
        6,
        "candidate-verified",
        serde_json::json!({"candidate":candidate_observation,"record_digest":candidate_record.digest,"generation":candidate_record.generation}),
        "candidate-verified receipt does not match created candidate",
    )?;
    require_receipt_payload_at(
        retained_attempt,
        7,
        "install-intent",
        serde_json::json!({"exchange":install_exchange,"candidate":candidate_observation,"canonical":prepared_classification.canonical}),
        "install intent receipt does not match candidate or anchor mode",
    )?;
    let canonical_installed: serde_json::Value =
        receipt_payload_at(retained_attempt, 8, "canonical-installed")?;
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
    require_receipt_payload_at(
        retained_attempt,
        9,
        "displace-intent",
        if install_exchange {
            serde_json::json!({"source":prior_observation,"destination":"displaced"})
        } else {
            serde_json::json!({"source":prior_observation,"destination":"displaced","completed_before_install":true})
        },
        "displace intent receipt does not match typed prior source",
    )?;
    let displaced = observe_child(
        retained_attempt,
        "displaced",
        store.root(),
        "displaced",
        issue,
    )?;
    require_receipt_payload_at(
        retained_attempt,
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
    let prior_digest = prior_observation
        .record_digest
        .as_deref()
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "verified prior digest missing"))?;
    let (expected_candidate_record, expected_candidate_files) = store
        .projection_recovery_candidate_files_from_bytes_locked(
            issue,
            &displaced_source,
            prior_digest,
            prepared_request.actor.clone(),
            prepared_request.reason.clone(),
            audit_commitment.to_string(),
        )?;
    let exact_candidate_files = candidate_observation
        .entries
        .iter()
        .filter(|entry| entry.identity.node_type == "regular")
        .count()
        == expected_candidate_files.len()
        && expected_candidate_files.iter().all(|(path, bytes)| {
            let digest = blake3::hash(bytes).to_hex().to_string();
            candidate_observation.entries.iter().any(|entry| {
                entry.path == *path
                    && entry.identity.node_type == "regular"
                    && entry.size == bytes.len() as u64
                    && entry.digest.as_deref() == Some(digest.as_str())
            })
        });
    if candidate_record != expected_candidate_record || !exact_candidate_files {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate does not equal the authorized recovery transformation",
        ));
    }
    require_receipt_payload_at(
        retained_attempt,
        11,
        "canonical-verified",
        serde_json::json!({"canonical":installed_observation,"archive":archived,"displaced":displaced}),
        "canonical-verified receipt does not match installed recovery state",
    )?;
    require_receipt_payload_at(
        retained_attempt,
        12,
        "recovery-complete-intent",
        serde_json::json!({"canonical_digest":candidate_record.digest,"generation":candidate_record.generation}),
        "recovery complete intent does not match candidate record",
    )?;
    let result: ProjectionRecoveryResult = receipt_payload_at(retained_attempt, 13, "recovered")?;
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
    if canonical_record.generation == result.canonical_generation {
        for (relative, expected) in derived_files {
            if fs::read(store.issue_dir(issue).join(&relative))? != expected {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "installed canonical artifact does not match authorized recovery transform",
                ));
            }
        }
    }
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

pub(crate) fn validate_completed_recovery_attempt_from_dir(
    store: &Store,
    issue: u64,
    attempt: &PrivateRecoveryDir,
    operation: &str,
) -> Result<ProjectionRecoveryResult> {
    validate_completed_recovery_attempt_retained(store, issue, attempt, operation)
}

pub fn build_archived_projection_cleanup_request_from_recovery(
    store: &Store,
    request: ProjectionRecoveryCleanupBridgeRequest,
) -> Result<ProjectionRecoveryCleanupBridgeResult> {
    if request.schema != "csdlc.projection_recovery_cleanup_bridge_request.v1"
        || request.issue == 0
        || request.cleanup_issue == 0
        || request.terminal_issue == 0
        || request.recovery_operation_id.trim().is_empty()
        || request.cleanup_operation_id.trim().is_empty()
        || request.repository_root.trim().is_empty()
        || request.execution_base.trim().is_empty()
        || request.terminal_envelope.trim().is_empty()
        || request.expected_terminal_digest.trim().is_empty()
        || request.expected_terminal_merge_sha.trim().is_empty()
        || request.cleanup_ledger_root.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "projection recovery cleanup bridge request is incomplete",
        ));
    }
    if request.terminal_issue != request.issue {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "cleanup bridge terminal issue must match the recovered issue",
        ));
    }
    validate_operation_id(&request.recovery_operation_id)?;
    validate_operation_id(&request.cleanup_operation_id)?;
    verify_topology(store, &request.branch, &request.worktree)?;
    let _lock = store.authority_projection_lock(request.issue)?;

    let recovery_path = recovery_root(store, request.issue);
    let attempt_path = recovery_path.join(&request.recovery_operation_id);
    let result = validate_completed_recovery_attempt(store, request.issue, &attempt_path)?;
    if result.operation_id != request.recovery_operation_id || result.issue != request.issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "completed recovery result does not match bridge request",
        ));
    }

    let issues = open_root_no_follow(&issue_parent(store))?;
    let root_authority = open_private_recovery_dir(&recovery_path, "recovery root", Some(&issues))?;
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    let operation_name = std::ffi::CString::new(request.recovery_operation_id.as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "operation id contains NUL"))?;
    let attempt_authority = root_authority.open_child(&operation_name, "recovery attempt")?;
    attempt_authority.validate_binding(&attempt_path, "recovery attempt")?;

    let archived_observation = observe_child(
        &attempt_authority,
        "rejected",
        store.root(),
        "archive",
        request.issue,
    )?;
    if archived_observation.manifest_digest.as_deref()
        != Some(result.archived_manifest_digest.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recovery archive observation does not match terminal recovery result",
        ));
    }
    let archived_root = fs::canonicalize(attempt_path.join("rejected"))?;
    let nodes = cleanup_nodes_from_archive_observation(&archived_observation)?;
    if nodes.is_empty() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed recovery archive has no cleanup-authorized nodes",
        ));
    }

    let manifest = serde_json::json!({
        "schema": "csdlc.canonical_archive_manifest.v1",
        "issue": request.terminal_issue,
        "terminal_digest": request.expected_terminal_digest,
        "archived_root": archived_root,
        "nodes": nodes,
    });
    let cleanup_authority =
        bridge_cleanup_authority_dir(&attempt_authority, &request.cleanup_operation_id)?;
    let authority_path = attempt_path
        .join("cleanup-authority")
        .join(&request.cleanup_operation_id);
    let manifest_name = std::ffi::CString::new("canonical-archive-manifest.json")
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "manifest name contains NUL"))?;
    let manifest_digest = write_bridge_json_child(&cleanup_authority, &manifest_name, &manifest)?;
    let manifest_path = authority_path.join(manifest_name.to_string_lossy().as_ref());

    let receipt = serde_json::json!({
        "schema": "csdlc.completed_recovery_receipt.v1",
        "issue": request.terminal_issue,
        "state": "completed",
        "terminal_digest": request.expected_terminal_digest,
        "canonical_archive_manifest_digest": manifest_digest,
    });
    let receipt_name = std::ffi::CString::new("completed-recovery-receipt.json")
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))?;
    let receipt_digest = write_bridge_json_child(&cleanup_authority, &receipt_name, &receipt)?;
    let receipt_path = authority_path.join(receipt_name.to_string_lossy().as_ref());

    let cleanup_request = ArchivedProjectionCleanupRequest {
        schema: "csdlc.archived_projection_cleanup_request.v1".into(),
        issue: request.cleanup_issue,
        operation_id: request.cleanup_operation_id.clone(),
        repository_root: request.repository_root,
        execution_base: request.execution_base,
        terminal_issue: request.terminal_issue,
        terminal_envelope: request.terminal_envelope,
        expected_terminal_digest: request.expected_terminal_digest,
        expected_terminal_merge_sha: request.expected_terminal_merge_sha,
        completed_recovery_receipt: receipt_path.to_string_lossy().into_owned(),
        expected_recovery_receipt_digest: receipt_digest.clone(),
        canonical_archive_manifest: manifest_path.to_string_lossy().into_owned(),
        expected_archive_manifest_digest: manifest_digest.clone(),
        archived_root: archived_root.to_string_lossy().into_owned(),
        cleanup_ledger_root: request.cleanup_ledger_root.clone(),
        nodes: nodes.clone(),
        fail_after: request.fail_after,
    };

    Ok(ProjectionRecoveryCleanupBridgeResult {
        schema: "csdlc.projection_recovery_cleanup_bridge_result.v1".into(),
        issue: request.issue,
        recovery_operation_id: request.recovery_operation_id,
        cleanup_issue: request.cleanup_issue,
        cleanup_operation_id: request.cleanup_operation_id,
        completed_recovery_receipt: cleanup_request.completed_recovery_receipt.clone(),
        expected_recovery_receipt_digest: receipt_digest,
        canonical_archive_manifest: cleanup_request.canonical_archive_manifest.clone(),
        expected_archive_manifest_digest: manifest_digest,
        archived_root: cleanup_request.archived_root.clone(),
        cleanup_ledger_root: request.cleanup_ledger_root,
        nodes,
        cleanup_request,
    })
}

fn cleanup_nodes_from_archive_observation(
    observation: &CandidateObservation,
) -> Result<Vec<ArchivedProjectionNode>> {
    if matches!(observation.state.as_str(), "absent" | "unsafe") {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed recovery archive is not cleanup-authorized",
        ));
    }
    let mut nodes = Vec::new();
    for entry in &observation.entries {
        if entry.path == "." {
            continue;
        }
        let node_type = match entry.identity.node_type.as_str() {
            "regular" => CleanupNodeType::RegularFile,
            "directory" => CleanupNodeType::Directory,
            _ => {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "completed recovery archive contains an unsupported node type",
                ));
            }
        };
        nodes.push(ArchivedProjectionNode {
            relative_path: entry.path.clone(),
            identity: CleanupNodeIdentity {
                device: entry.identity.device,
                inode: entry.identity.inode,
                links: entry.identity.links,
                uid: entry.identity.uid,
                gid: entry.identity.gid,
                mode: entry.identity.mode,
                node_type,
            },
        });
    }
    nodes.sort_by(|left, right| {
        let left_depth = left.relative_path.matches('/').count();
        let right_depth = right.relative_path.matches('/').count();
        right_depth
            .cmp(&left_depth)
            .then_with(|| left.relative_path.cmp(&right.relative_path))
    });
    Ok(nodes)
}

fn validate_cleanup_authority_namespace(
    attempt: &PrivateRecoveryDir,
    name: &std::ffi::CStr,
) -> Result<()> {
    let namespace = attempt.open_child(name, "cleanup authority namespace")?;
    for operation_name in namespace.names()? {
        let operation = namespace.open_child(&operation_name, "cleanup authority operation")?;
        let mut names = operation
            .names()?
            .into_iter()
            .map(|entry| {
                entry.to_str().map(str::to_owned).map_err(|_| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        "cleanup authority artifact name is not UTF-8",
                    )
                })
            })
            .collect::<Result<Vec<_>>>()?;
        names.sort();
        if names
            != [
                "canonical-archive-manifest.json",
                "completed-recovery-receipt.json",
            ]
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup authority namespace contains unexpected artifacts",
            ));
        }
        let manifest_name = std::ffi::CString::new("canonical-archive-manifest.json")
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "manifest name contains NUL"))?;
        let receipt_name = std::ffi::CString::new("completed-recovery-receipt.json")
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))?;
        let _: serde_json::Value =
            serde_json::from_slice(&operation.read_regular_child(&manifest_name)?)?;
        let _: serde_json::Value =
            serde_json::from_slice(&operation.read_regular_child(&receipt_name)?)?;
    }
    Ok(())
}

fn validate_completed_recovery_cleanup_authority(
    store: &Store,
    issue: u64,
    attempt: &PrivateRecoveryDir,
    recovery_operation: &str,
    attempt_path: &Path,
    archived: &CandidateObservation,
) -> Result<()> {
    let namespace_name = std::ffi::CString::new("cleanup-authority").map_err(|_| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "cleanup authority name contains NUL",
        )
    })?;
    let namespace = attempt.open_child(&namespace_name, "cleanup authority namespace")?;
    let operation_names = namespace.names()?;
    if operation_names.len() != 1 {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup authority namespace is missing or ambiguous",
        ));
    }
    let cleanup_operation_name = &operation_names[0];
    let cleanup_operation = cleanup_operation_name.to_str().map_err(|_| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup authority operation name is not UTF-8",
        )
    })?;
    validate_operation_id(cleanup_operation)?;
    if cleanup_operation == recovery_operation {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup authority operation must be distinct from recovery operation",
        ));
    }
    let authority = namespace.open_child(cleanup_operation_name, "cleanup authority operation")?;
    let manifest_name = std::ffi::CString::new("canonical-archive-manifest.json")
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "manifest name contains NUL"))?;
    let receipt_name = std::ffi::CString::new("completed-recovery-receipt.json")
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "receipt name contains NUL"))?;
    let manifest_bytes = authority.read_regular_child(&manifest_name)?;
    let manifest_digest = blake3::hash(&manifest_bytes).to_hex().to_string();
    let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes)?;
    let archived_root = fs::canonicalize(attempt_path)?.join("rejected");
    let expected_nodes = cleanup_nodes_from_archive_observation(archived)?;
    if manifest.get("schema").and_then(|value| value.as_str())
        != Some("csdlc.canonical_archive_manifest.v1")
        || manifest.get("issue").and_then(|value| value.as_u64()) != Some(issue)
        || manifest.get("archived_root") != Some(&serde_json::to_value(&archived_root)?)
        || manifest.get("nodes") != Some(&serde_json::to_value(&expected_nodes)?)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup authority manifest does not bind retained archive receipt",
        ));
    }
    let terminal_digest = manifest
        .get("terminal_digest")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup authority manifest terminal digest is missing",
            )
        })?;
    let receipt_bytes = authority.read_regular_child(&receipt_name)?;
    let receipt_digest = blake3::hash(&receipt_bytes).to_hex().to_string();
    let receipt: serde_json::Value = serde_json::from_slice(&receipt_bytes)?;
    if receipt.get("schema").and_then(|value| value.as_str())
        != Some("csdlc.completed_recovery_receipt.v1")
        || receipt.get("issue").and_then(|value| value.as_u64()) != Some(issue)
        || receipt.get("state").and_then(|value| value.as_str()) != Some("completed")
        || receipt
            .get("terminal_digest")
            .and_then(|value| value.as_str())
            != Some(terminal_digest)
        || receipt
            .get("canonical_archive_manifest_digest")
            .and_then(|value| value.as_str())
            != Some(manifest_digest.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup authority receipt does not bind terminal and archive manifest",
        ));
    }

    let cleanup_operation_root = recovery_root(store, issue)
        .join(cleanup_operation)
        .join(cleanup_operation);
    let operation_payload =
        cleanup_receipt_payload(&cleanup_operation_root.join("001-operation-created.json"))?;
    if operation_payload
        != serde_json::json!({
            "issue": issue,
            "operation_id": cleanup_operation,
            "terminal_issue": issue,
            "terminal_digest": terminal_digest,
            "recovery_receipt_digest": receipt_digest,
            "archive_manifest_digest": manifest_digest,
            "archived_root": archived_root,
        })
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup ledger does not bind completed recovery authority",
        ));
    }
    let final_previous =
        validate_completed_cleanup_ledger_chain(&cleanup_operation_root, &expected_nodes)?;
    let final_receipt_path = cleanup_operation_root.join("900-cleanup-complete.json");
    let final_receipt = cleanup_receipt_envelope(&final_receipt_path)?;
    if final_receipt.get("schema").and_then(|value| value.as_str())
        != Some("csdlc.archived_projection_cleanup_receipt.v1")
        || final_receipt
            .get("sequence")
            .and_then(|value| value.as_u64())
            != Some(900)
        || final_receipt.get("state").and_then(|value| value.as_str()) != Some("cleanup-complete")
        || final_receipt
            .get("previous_receipt_digest")
            .and_then(|value| value.as_str())
            != Some(final_previous.as_str())
        || final_receipt.get("payload")
            != Some(&serde_json::json!({
                "issue": issue,
                "operation_id": cleanup_operation,
                "nodes": expected_nodes.iter().map(|node| node.relative_path.as_str()).collect::<Vec<_>>(),
            }))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup final receipt does not bind retained recovery authority",
        ));
    }
    Ok(())
}

fn validate_completed_cleanup_ledger_chain(
    operation_root: &Path,
    expected_nodes: &[ArchivedProjectionNode],
) -> Result<String> {
    reject_unexpected_cleanup_ledger_entries(operation_root, expected_nodes)?;
    let (mut previous, operation_payload) = cleanup_receipt_envelope_digest(
        &operation_root.join("001-operation-created.json"),
        1,
        "operation-created",
        None,
    )?;
    if !operation_payload.is_object() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup operation-created receipt payload is invalid",
        ));
    }
    let (namespace_digest, namespace_payload) = cleanup_receipt_envelope_digest(
        &operation_root.join("002-namespace-created.json"),
        2,
        "namespace-created",
        Some(&previous),
    )?;
    if namespace_payload
        .get("private_root")
        .and_then(|value| value.as_str())
        != Some("private-delete")
        || namespace_payload
            .get("mode")
            .and_then(|value| value.as_str())
            != Some("0700")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup namespace-created receipt payload is invalid",
        ));
    }
    previous = namespace_digest;
    for (index, node) in expected_nodes.iter().enumerate() {
        let ordinal = index as u32 + 1;
        let base = 100 + ordinal * 10;
        let (placeholder_digest, placeholder_payload) = cleanup_receipt_envelope_digest(
            &operation_root.join(format!("{base:03}-placeholder-created.json")),
            base,
            "placeholder-created",
            Some(&previous),
        )?;
        if placeholder_payload
            .get("path")
            .and_then(|value| value.as_str())
            != Some(node.relative_path.as_str())
            || placeholder_payload
                .get("private")
                .and_then(|value| value.as_str())
                != Some(format!("node-{ordinal:03}").as_str())
            || placeholder_payload.get("identity").is_none()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup placeholder receipt does not match retained recovery authority",
            ));
        }
        let placeholder_identity =
            placeholder_payload
                .get("identity")
                .cloned()
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        "cleanup placeholder identity is missing",
                    )
                })?;
        let (captured_digest, captured_payload) = cleanup_receipt_envelope_digest(
            &operation_root.join(format!("{:03}-captured.json", base + 3)),
            base + 3,
            "captured",
            Some(&placeholder_digest),
        )?;
        if captured_payload
            .get("path")
            .and_then(|value| value.as_str())
            != Some(node.relative_path.as_str())
            || captured_payload.get("expected_identity")
                != Some(&serde_json::to_value(&node.identity)?)
            || captured_payload.get("placeholder_identity") != Some(&placeholder_identity)
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup captured receipt does not match retained recovery authority",
            ));
        }
        let (removed_digest, removed_payload) = cleanup_receipt_envelope_digest(
            &operation_root.join(format!("{:03}-removed.json", base + 6)),
            base + 6,
            "removed",
            Some(&captured_digest),
        )?;
        if removed_payload.get("path").and_then(|value| value.as_str())
            != Some(node.relative_path.as_str())
            || removed_payload.get("removed_identity")
                != Some(&serde_json::to_value(&node.identity)?)
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup removed receipt does not match retained recovery authority",
            ));
        }
        let (disposed_digest, disposed_payload) = cleanup_receipt_envelope_digest(
            &operation_root.join(format!("{:03}-placeholder-disposed.json", base + 8)),
            base + 8,
            "placeholder-disposed",
            Some(&removed_digest),
        )?;
        if disposed_payload
            .get("path")
            .and_then(|value| value.as_str())
            != Some(node.relative_path.as_str())
            || disposed_payload.get("placeholder_identity") != Some(&placeholder_identity)
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup placeholder disposal receipt does not match retained recovery authority",
            ));
        }
        previous = disposed_digest;
    }
    Ok(previous)
}

fn reject_unexpected_cleanup_ledger_entries(
    operation_root: &Path,
    expected_nodes: &[ArchivedProjectionNode],
) -> Result<()> {
    let mut expected = BTreeSet::from([
        "001-operation-created.json".to_string(),
        "002-namespace-created.json".to_string(),
        "900-cleanup-complete.json".to_string(),
        "private-delete".to_string(),
    ]);
    for (index, _) in expected_nodes.iter().enumerate() {
        let ordinal = index as u32 + 1;
        let base = 100 + ordinal * 10;
        expected.insert(format!("{base:03}-placeholder-created.json"));
        expected.insert(format!("{:03}-captured.json", base + 3));
        expected.insert(format!("{:03}-removed.json", base + 6));
        expected.insert(format!("{:03}-placeholder-disposed.json", base + 8));
    }
    for entry in fs::read_dir(operation_root)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if !expected.contains(&name) {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup operation ledger contains unexpected entries",
            ));
        }
    }
    let private_root = operation_root.join("private-delete");
    let metadata = fs::symlink_metadata(&private_root)?;
    if !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup private namespace is missing",
        ));
    }
    if fs::read_dir(&private_root)?.next().is_some() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup private namespace contains unexpected entries",
        ));
    }
    Ok(())
}

fn cleanup_receipt_envelope_digest(
    path: &Path,
    sequence: u32,
    state: &str,
    previous: Option<&String>,
) -> Result<(String, serde_json::Value)> {
    let bytes = read_cleanup_receipt_file(path)?;
    let envelope: serde_json::Value = serde_json::from_slice(&bytes)?;
    if envelope.get("schema").and_then(|value| value.as_str())
        != Some("csdlc.archived_projection_cleanup_receipt.v1")
        || envelope.get("sequence").and_then(|value| value.as_u64()) != Some(sequence as u64)
        || envelope.get("state").and_then(|value| value.as_str()) != Some(state)
        || envelope.get("previous_receipt_digest") != Some(&serde_json::to_value(previous)?)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt predecessor chain is invalid",
        ));
    }
    let payload = envelope.get("payload").cloned().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt payload is missing",
        )
    })?;
    Ok((blake3::hash(&bytes).to_hex().to_string(), payload))
}

pub(crate) fn is_authorized_cleanup_ledger_entry(
    store: &Store,
    issue: u64,
    recovery: &PrivateRecoveryDir,
    cleanup_operation: &str,
) -> Result<bool> {
    for attempt_name in recovery.names()? {
        let recovery_operation = attempt_name.to_str().map_err(|_| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery namespace contains a non-UTF8 entry",
            )
        })?;
        if recovery_operation == cleanup_operation {
            continue;
        }
        let Ok(attempt) = recovery.open_child(&attempt_name, "recovery attempt") else {
            continue;
        };
        let namespace_name = std::ffi::CString::new("cleanup-authority").map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "cleanup authority name contains NUL",
            )
        })?;
        let Ok(namespace) = attempt.open_child(&namespace_name, "cleanup authority namespace")
        else {
            continue;
        };
        let operation_name =
            std::ffi::CString::new(cleanup_operation.as_bytes()).map_err(|_| {
                V2Error::new(ErrorCode::InvalidInput, "cleanup operation id contains NUL")
            })?;
        if namespace
            .open_child(&operation_name, "cleanup authority operation")
            .is_err()
        {
            continue;
        }
        let archived: CandidateObservation =
            match receipt_payload_at(&attempt, 3, "rejected-archived") {
                Ok(archived) => archived,
                Err(_) => continue,
            };
        validate_completed_recovery_cleanup_authority(
            store,
            issue,
            &attempt,
            recovery_operation,
            &recovery_root(store, issue).join(recovery_operation),
            &archived,
        )?;
        return Ok(true);
    }
    Ok(false)
}

fn cleanup_receipt_envelope(path: &Path) -> Result<serde_json::Value> {
    let bytes = read_cleanup_receipt_file(path)?;
    serde_json::from_slice(&bytes).map_err(Into::into)
}

fn read_cleanup_receipt_file(path: &Path) -> Result<Vec<u8>> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt is not a regular non-symlink file",
        ));
    }
    fs::read(path).map_err(Into::into)
}

fn cleanup_receipt_payload(path: &Path) -> Result<serde_json::Value> {
    let envelope = cleanup_receipt_envelope(path)?;
    if envelope.get("schema").and_then(|value| value.as_str())
        != Some("csdlc.archived_projection_cleanup_receipt.v1")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt schema is invalid",
        ));
    }
    envelope.get("payload").cloned().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "cleanup receipt payload is missing",
        )
    })
}

fn bridge_cleanup_authority_dir(
    attempt: &PrivateRecoveryDir,
    cleanup_operation_id: &str,
) -> Result<PrivateRecoveryDir> {
    let namespace_name = std::ffi::CString::new("cleanup-authority").map_err(|_| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "cleanup authority name contains NUL",
        )
    })?;
    if attempt
        .open_child(&namespace_name, "cleanup authority namespace")
        .is_err()
    {
        attempt.create_private_child(&namespace_name)?;
    }
    let namespace = attempt.open_child(&namespace_name, "cleanup authority namespace")?;
    let operation_name = std::ffi::CString::new(cleanup_operation_id.as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "cleanup operation id contains NUL"))?;
    for existing in namespace.names()? {
        if existing.as_c_str() != operation_name.as_c_str() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "cleanup bridge authority already exists for a different operation",
            ));
        }
    }
    if namespace
        .open_child(&operation_name, "cleanup authority operation")
        .is_err()
    {
        namespace.create_private_child(&operation_name)?;
    }
    namespace.open_child(&operation_name, "cleanup authority operation")
}

fn write_bridge_json_child(
    dir: &PrivateRecoveryDir,
    name: &std::ffi::CStr,
    value: &serde_json::Value,
) -> Result<String> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    match dir.read_regular_child(name) {
        Ok(existing) => {
            if existing == bytes {
                return Ok(blake3::hash(&existing).to_hex().to_string());
            }
            Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "cleanup bridge authority artifact already exists with different contents",
            ))
        }
        Err(_) => {
            dir.create_regular_child(name, &bytes)?;
            Ok(blake3::hash(&bytes).to_hex().to_string())
        }
    }
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

fn anchored_child_name(parent: &File, name: &str, directory: bool) -> Result<AnchoredName> {
    use std::os::fd::AsRawFd;
    let name = std::ffi::CString::new(name)
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "anchored child name contains NUL"))?;
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    let exists = unsafe {
        libc::fstatat(
            parent.as_raw_fd(),
            name.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    } == 0;
    let child = if exists {
        Some(open_child_no_follow(parent, &name, directory)?)
    } else {
        None
    };
    Ok(AnchoredName {
        parent: parent.try_clone()?,
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
    attempt_authority: &PrivateRecoveryDir,
    candidate_authority: &PrivateRecoveryDir,
    relative: &str,
    ordinal: usize,
) -> Result<()> {
    let ledger_authority = open_or_create_private_child(
        attempt_authority,
        &format!("node-{ordinal:03}"),
        "recovery node ledger",
    )?;
    let node_authority = if relative == "." {
        PrivateRecoveryDir {
            file: candidate_authority.file.try_clone()?,
        }
    } else {
        open_or_create_private_child(candidate_authority, relative, "candidate directory")?
    };
    if receipt_exists_at(&ledger_authority, 10, "node-created")? {
        if fd_identity(&node_authority.file)?.node_type == "directory" {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed candidate directory was replaced",
        ));
    }
    receipt_at(
        &ledger_authority,
        1,
        "create-intent",
        &serde_json::json!({"path":relative,"type":"directory","mode":"0700"}),
    )?;
    failpoint(request, "node_create_intent")?;
    if !receipt_exists_at(&ledger_authority, 2, "created-identity")? {
        let node = fd_identity(&node_authority.file)?;
        if node.node_type != "directory" || node.mode & 0o777 != 0o700 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate directory collision or mode drift",
            ));
        }
        receipt_at(
            &ledger_authority,
            2,
            "created-identity",
            &serde_json::to_value(&node)?,
        )?;
        failpoint(request, "node_created_identity")?;
    }
    receipt_at(
        &ledger_authority,
        3,
        "write-intent",
        &serde_json::json!({"directory":relative,"metadata_only":true}),
    )?;
    receipt_at(
        &ledger_authority,
        4,
        "write-completed",
        &serde_json::json!({"directory":relative}),
    )?;
    failpoint(request, "node_write_completed")?;
    receipt_at(
        &ledger_authority,
        5,
        "node-fsync-intent",
        &serde_json::json!({"directory":relative}),
    )?;
    node_authority.file.sync_all()?;
    failpoint(request, "node_fsynced")?;
    receipt_at(
        &ledger_authority,
        6,
        "node-fsync-completed",
        &serde_json::json!({"identity":fd_identity(&node_authority.file)?}),
    )?;
    receipt_at(
        &ledger_authority,
        7,
        "parent-fsync-intent",
        &serde_json::json!({"directory":relative}),
    )?;
    if relative == "." {
        attempt_authority.file.sync_all()?;
    } else {
        candidate_authority.file.sync_all()?;
    }
    failpoint(request, "node_parent_fsynced")?;
    receipt_at(
        &ledger_authority,
        8,
        "parent-fsync-completed",
        &serde_json::json!({"directory":relative}),
    )?;
    receipt_at(
        &ledger_authority,
        9,
        "publish-intent",
        &serde_json::json!({"directory":relative,"already_at_final_name":true}),
    )?;
    receipt_at(
        &ledger_authority,
        10,
        "node-created",
        &serde_json::json!({"identity":fd_identity(&node_authority.file)?}),
    )?;
    Ok(())
}

fn ensure_candidate_file(
    request: &ProjectionRecoverRequest,
    attempt_authority: &PrivateRecoveryDir,
    candidate_authority: &PrivateRecoveryDir,
    candidate: &Path,
    relative: &str,
    contents: &[u8],
    ordinal: usize,
) -> Result<()> {
    let ledger_authority = open_or_create_private_child(
        attempt_authority,
        &format!("node-{ordinal:03}"),
        "recovery node ledger",
    )?;
    let final_path = candidate.join(relative);
    let parent = final_path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "candidate file has no parent"))?;
    let temporary = parent.join(format!(".recovery-node-{ordinal:03}.tmp"));
    let relative_path = Path::new(relative);
    let parent_authority = match relative_path.parent().and_then(|path| path.to_str()) {
        Some("") | None => PrivateRecoveryDir {
            file: candidate_authority.file.try_clone()?,
        },
        Some("cards") => candidate_authority.open_child(
            &std::ffi::CString::new("cards").unwrap(),
            "candidate file parent",
        )?,
        _ => {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "candidate file parent is unsupported",
            ))
        }
    };
    let final_name = std::ffi::CString::new(
        relative_path
            .file_name()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "candidate file has no name"))?
            .as_bytes(),
    )
    .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "candidate file name contains NUL"))?;
    let temporary_name =
        std::ffi::CString::new(format!(".recovery-node-{ordinal:03}.tmp")).unwrap();
    let digest = blake3::hash(contents).to_hex().to_string();
    if receipt_exists_at(&ledger_authority, 10, "node-created")? {
        let final_file = parent_authority.open_regular_child(&final_name)?;
        if read_file_bytes(&final_file)? == contents {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "completed candidate file was replaced",
        ));
    }
    if receipt_exists_at(&ledger_authority, 9, "publish-intent")? {
        let temp_file = parent_authority.open_regular_child(&temporary_name);
        let final_file = parent_authority.open_regular_child(&final_name);
        if temp_file.is_ok() && final_file.is_err() {
            rename_no_replace(
                &AnchoredName {
                    parent: parent_authority.file.try_clone()?,
                    name: temporary_name.clone(),
                    child: temp_file.ok(),
                },
                &AnchoredName {
                    parent: parent_authority.file.try_clone()?,
                    name: final_name.clone(),
                    child: None,
                },
            )?;
        } else if !(temp_file.is_err()
            && final_file
                .as_ref()
                .is_ok_and(|file| read_file_bytes(file).is_ok_and(|bytes| bytes == contents)))
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate node publish restart is ambiguous",
            ));
        }
        parent_authority.file.sync_all()?;
        let node = fd_identity(&parent_authority.open_regular_child(&final_name)?)?;
        receipt_at(
            &ledger_authority,
            10,
            "node-created",
            &serde_json::json!({"identity":node,"digest":digest,"size":contents.len()}),
        )?;
        return Ok(());
    }
    receipt_at(
        &ledger_authority,
        1,
        "create-intent",
        &serde_json::json!({"temporary":temporary.file_name(),"final":relative,"type":"regular","mode":"0600","digest":digest,"size":contents.len()}),
    )?;
    failpoint(request, "node_create_intent")?;
    if !receipt_exists_at(&ledger_authority, 2, "created-identity")? {
        let file = match parent_authority.open_writable_regular_child(&temporary_name) {
            Ok(file) => file,
            Err(_) => parent_authority.create_empty_regular_child(&temporary_name)?,
        };
        let node = fd_identity(&file)?;
        if node.node_type != "regular" || node.links != 1 || node.mode & 0o777 != 0o600 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary identity is unsafe",
            ));
        }
        receipt_at(
            &ledger_authority,
            2,
            "created-identity",
            &serde_json::to_value(&node)?,
        )?;
        failpoint(request, "node_created_identity")?;
    }
    receipt_at(
        &ledger_authority,
        3,
        "write-intent",
        &serde_json::json!({"digest":digest,"size":contents.len()}),
    )?;
    if !receipt_exists_at(&ledger_authority, 4, "write-completed")? {
        let mut file = parent_authority.open_writable_regular_child(&temporary_name)?;
        let existing = read_file_bytes(&file)?;
        if existing.len() > contents.len() || existing != contents[..existing.len()] {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary content is not the committed prefix",
            ));
        }
        if existing.len() < contents.len() {
            use std::io::{Seek, SeekFrom};
            file.seek(SeekFrom::End(0))?;
            file.write_all(&contents[existing.len()..])?;
        }
        if read_file_bytes(&file)? != contents {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate temporary content completion failed",
            ));
        }
        receipt_at(
            &ledger_authority,
            4,
            "write-completed",
            &serde_json::json!({"digest":digest,"size":contents.len()}),
        )?;
        failpoint(request, "node_write_completed")?;
    }
    receipt_at(
        &ledger_authority,
        5,
        "node-fsync-intent",
        &serde_json::json!({"temporary":temporary.file_name()}),
    )?;
    let temporary_file = parent_authority.open_writable_regular_child(&temporary_name)?;
    temporary_file.sync_all()?;
    failpoint(request, "node_fsynced")?;
    receipt_at(
        &ledger_authority,
        6,
        "node-fsync-completed",
        &serde_json::json!({"identity":fd_identity(&temporary_file)?,"digest":digest}),
    )?;
    receipt_at(
        &ledger_authority,
        7,
        "parent-fsync-intent",
        &serde_json::json!({"parent":parent}),
    )?;
    parent_authority.file.sync_all()?;
    failpoint(request, "node_parent_fsynced")?;
    receipt_at(
        &ledger_authority,
        8,
        "parent-fsync-completed",
        &serde_json::json!({"parent":parent}),
    )?;
    receipt_at(
        &ledger_authority,
        9,
        "publish-intent",
        &serde_json::json!({"temporary":temporary.file_name(),"final":relative}),
    )?;
    if !receipt_exists_at(&ledger_authority, 10, "node-created")? {
        let temp_file = parent_authority.open_regular_child(&temporary_name);
        let final_file = parent_authority.open_regular_child(&final_name);
        if temp_file.is_ok() && final_file.is_err() {
            rename_no_replace(
                &AnchoredName {
                    parent: parent_authority.file.try_clone()?,
                    name: temporary_name.clone(),
                    child: temp_file.ok(),
                },
                &AnchoredName {
                    parent: parent_authority.file.try_clone()?,
                    name: final_name.clone(),
                    child: None,
                },
            )?;
        } else if !(temp_file.is_err()
            && final_file
                .as_ref()
                .is_ok_and(|file| read_file_bytes(file).is_ok_and(|bytes| bytes == contents)))
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "candidate node publish state is ambiguous",
            ));
        }
        failpoint(request, "node_published")?;
        parent_authority.file.sync_all()?;
        let final_file = parent_authority.open_regular_child(&final_name)?;
        let node = fd_identity(&final_file)?;
        if node.node_type != "regular"
            || node.links != 1
            || read_file_bytes(&final_file)? != contents
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "published candidate node drifted",
            ));
        }
        receipt_at(
            &ledger_authority,
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
    attempt_authority: &PrivateRecoveryDir,
    source_authority: &File,
    candidate: &Path,
    expected_digest: &str,
    audit_payload: &serde_json::Value,
) -> Result<IssueRecord> {
    let source_files = projection_source_files(source_authority)?;
    let (record, files) = store.projection_recovery_candidate_files_from_bytes_locked(
        request.issue,
        &source_files,
        expected_digest,
        request.actor.clone(),
        request.reason.clone(),
        audit_payload.to_string(),
    )?;
    let candidate_authority =
        open_or_create_private_child(attempt_authority, "candidate", "recovery candidate")?;
    ensure_candidate_directory(request, attempt_authority, &candidate_authority, ".", 0)?;
    ensure_candidate_directory(request, attempt_authority, &candidate_authority, "cards", 1)?;
    for (ordinal, (relative, contents)) in files.iter().enumerate() {
        ensure_candidate_file(
            request,
            attempt_authority,
            &candidate_authority,
            candidate,
            relative,
            contents,
            ordinal + 2,
        )?;
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
    let recovery_path = recovery_root(store, request.issue);
    if recovery_path.symlink_metadata().is_err() {
        private_dir(&recovery_path)?;
    }
    let issues = open_root_no_follow(&issue_parent(store))?;
    let root_authority = open_private_recovery_dir(&recovery_path, "recovery root", Some(&issues))?;
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    for name in directory_names(&root_authority.file)? {
        if name.to_bytes() != request.operation_id.as_bytes() {
            let other = root_authority.open_child(&name, "recovery attempt")?;
            if !directory_names(&other.file)?
                .iter()
                .any(|item| item.to_bytes().ends_with(b"-recovered.json"))
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "a different incomplete recovery attempt already exists",
                ));
            }
        }
    }
    if request.classification.receipt_digest != request.classify_receipt_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "classification payload and receipt digest disagree",
        ));
    }
    validate_classification_authority(
        &request.classification,
        &request.classify_receipt_digest,
        request.issue,
    )?;
    let operation_name = std::ffi::CString::new(request.operation_id.as_bytes())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "operation id contains NUL"))?;
    if !directory_names(&root_authority.file)?
        .iter()
        .any(|name| name.as_bytes() == operation_name.as_bytes())
    {
        root_authority.create_private_child(&operation_name)?;
    }
    let attempt_authority = root_authority.open_child(&operation_name, "recovery attempt")?;
    let original_attempt_path = recovery_path.join(&request.operation_id);
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
    inject_post_validation_directory_swap(
        &request,
        "swap_recovery_root_after_validation",
        &recovery_path,
    )?;
    inject_post_validation_directory_swap(
        &request,
        "swap_recovery_attempt_after_validation",
        &original_attempt_path,
    )?;
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
    let attempt = original_attempt_path.clone();
    if receipt_exists_at(&attempt_authority, 13, "recovered")? {
        return validate_completed_recovery_attempt(store, request.issue, &attempt);
    }
    let classification = if receipt_exists_at(&attempt_authority, 1, "prepared")? {
        let prepared: serde_json::Value = receipt_payload_at(&attempt_authority, 1, "prepared")?;
        let classification =
            serde_json::from_value(prepared.get("classification").cloned().ok_or_else(|| {
                V2Error::new(ErrorCode::CorruptRecord, "PREPARED classification missing")
            })?)?;
        let prepared_request: ProjectionRecoverRequest =
            serde_json::from_value(prepared.get("request").cloned().ok_or_else(|| {
                V2Error::new(ErrorCode::CorruptRecord, "PREPARED typed request missing")
            })?)?;
        validate_classification_authority(
            &classification,
            &request.classify_receipt_digest,
            request.issue,
        )?;
        if prepared_request.classification != classification {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "active recovery PREPARED authority does not match request",
            ));
        }
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
            || request.classification != classification
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
        let request_digest = request_authority_digest(&request)?;
        receipt_at(
            &attempt_authority,
            1,
            "prepared",
            &serde_json::json!({"request_digest":request_digest,"request":request,"classification":classification,"lineage":request.failed_operation_lineage}),
        )?;
        failpoint(&request, "prepared")?;
        classification
    };

    root_authority.validate_binding(&recovery_path, "recovery root")?;
    attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;

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

    inject_post_validation_directory_swap(
        &request,
        "swap_recovery_root_before_archive_mutation",
        &recovery_path,
    )?;
    inject_post_validation_directory_swap(
        &request,
        "swap_recovery_attempt_before_archive_mutation",
        &original_attempt_path,
    )?;
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
    receipt_at(
        &attempt_authority,
        2,
        "archive-intent",
        &serde_json::json!({"source":rejected_observation,"destination":"rejected"}),
    )?;
    failpoint(&request, "archive_intent")?;
    if !receipt_exists_at(&attempt_authority, 3, "rejected-archived")? {
        root_authority.validate_binding(&recovery_path, "recovery root")?;
        attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
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
            let archive_anchor = anchored_child_name(&attempt_authority.file, "rejected", true)?;
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
        attempt_authority.file.sync_all()?;
        let archived_now = observe_child(
            &attempt_authority,
            "rejected",
            store.root(),
            "archive",
            request.issue,
        )?;
        receipt_at(
            &attempt_authority,
            3,
            "rejected-archived",
            &serde_json::to_value(&archived_now)?,
        )?;
        failpoint(&request, "rejected_archived")?;
    }
    let archived = observe_child(
        &attempt_authority,
        "rejected",
        store.root(),
        "archive",
        request.issue,
    )?;

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
    root_authority.validate_binding(&recovery_path, "recovery root")?;
    attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
    receipt_at(
        &attempt_authority,
        4,
        "candidate-plan",
        &serde_json::json!({"source":prior_observation,"destination":"candidate","audit":audit_payload}),
    )?;
    failpoint(&request, "candidate_plan")?;
    let (candidate_record, candidate_observation) =
        if receipt_exists_at(&attempt_authority, 5, "candidate-created")? {
            let payload: serde_json::Value =
                receipt_payload_at(&attempt_authority, 5, "candidate-created")?;
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
            root_authority.validate_binding(&recovery_path, "recovery root")?;
            attempt_authority.validate_binding(&original_attempt_path, "recovery attempt")?;
            let build_prior_authority = retained_matching_projection(
                &attempt_authority,
                prior_path,
                &prior_observation,
                store.root(),
                request.issue,
            )?;
            let record = build_candidate_tree(
                store,
                &request,
                &attempt_authority,
                &build_prior_authority,
                &candidate,
                prior_observation.record_digest.as_deref().ok_or_else(|| {
                    V2Error::new(ErrorCode::CorruptRecord, "verified prior digest missing")
                })?,
                &audit_payload,
            )?;
            let candidate_authority = attempt_authority.open_child(
                &std::ffi::CString::new("candidate").unwrap(),
                "recovery candidate",
            )?;
            candidate_authority.file.sync_all()?;
            attempt_authority.file.sync_all()?;
            let observation = observe_file(
                &candidate_authority.file,
                store.root(),
                "candidate",
                request.issue,
            );
            receipt_at(
                &attempt_authority,
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
    let retained_prior_authority = retained_matching_projection(
        &attempt_authority,
        prior_path,
        &prior_observation,
        store.root(),
        request.issue,
    )?;
    let retained_prior_files = projection_source_files(&retained_prior_authority)?;
    let (derived_record, derived_files) = store
        .projection_recovery_candidate_files_from_bytes_locked(
            request.issue,
            &retained_prior_files,
            prior_observation.record_digest.as_deref().ok_or_else(|| {
                V2Error::new(ErrorCode::CorruptRecord, "verified prior digest missing")
            })?,
            request.actor.clone(),
            request.reason.clone(),
            audit_payload.to_string(),
        )?;
    if candidate_record != derived_record {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "candidate-created receipt does not match authorized recovery transform",
        ));
    }
    if let Ok(candidate_authority) = attempt_authority.open_child(
        &std::ffi::CString::new("candidate").unwrap(),
        "recovery candidate",
    ) {
        let observed = observe_file(
            &candidate_authority.file,
            store.root(),
            "candidate",
            request.issue,
        );
        if !observations_match_after_move(&observed, &candidate_observation) {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "candidate descriptor observation drifted from its receipt",
            ));
        }
        for (relative, expected) in derived_files {
            if read_relative_no_follow(&candidate_authority.file, &relative)? != expected {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "candidate artifact does not match authorized recovery transform",
                ));
            }
        }
    }

    receipt_at(
        &attempt_authority,
        6,
        "candidate-verified",
        &serde_json::json!({"candidate":candidate_observation,"record_digest":candidate_record.digest,"generation":candidate_record.generation}),
    )?;
    failpoint(&request, "candidate_verified")?;

    if !receipt_exists_at(&attempt_authority, 8, "canonical-installed")? {
        receipt_at(
            &attempt_authority,
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
                let candidate_anchor =
                    anchored_child_name(&attempt_authority.file, "candidate", true)?;
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
                let displaced_anchor =
                    anchored_child_name(&attempt_authority.file, "displaced", true)?;
                rename_no_replace(&prior_anchor, &displaced_anchor)?;
                sync(&issue_parent(store))?;
                attempt_authority.file.sync_all()?;
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
                let candidate_anchor =
                    anchored_child_name(&attempt_authority.file, "candidate", true)?;
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
        attempt_authority.file.sync_all()?;
        receipt_at(
            &attempt_authority,
            8,
            "canonical-installed",
            &serde_json::json!({"canonical":observe(&canonical_path,"canonical",request.issue)}),
        )?;
        failpoint(&request, "canonical_installed")?;
    }

    if install_exchange && !receipt_exists_at(&attempt_authority, 10, "prior-displaced")? {
        receipt_at(
            &attempt_authority,
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
            let candidate_anchor = anchored_child_name(&attempt_authority.file, "candidate", true)?;
            if !anchored_moved_root_identity_matches(&candidate_anchor, &prior_observation)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "post-exchange prior identity drifted at displacement boundary",
                ));
            }
            let displaced_anchor = anchored_child_name(&attempt_authority.file, "displaced", true)?;
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
        attempt_authority.file.sync_all()?;
        let displaced_now = observe_child(
            &attempt_authority,
            "displaced",
            store.root(),
            "displaced",
            request.issue,
        )?;
        receipt_at(
            &attempt_authority,
            10,
            "prior-displaced",
            &serde_json::to_value(&displaced_now)?,
        )?;
        failpoint(&request, "prior_displaced")?;
    } else if !install_exchange {
        receipt_at(
            &attempt_authority,
            9,
            "displace-intent",
            &serde_json::json!({"source":prior_observation,"destination":"displaced","completed_before_install":true}),
        )?;
        receipt_at(
            &attempt_authority,
            10,
            "prior-displaced",
            &serde_json::to_value(observe_child(
                &attempt_authority,
                "displaced",
                store.root(),
                "displaced",
                request.issue,
            )?)?,
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
    receipt_at(
        &attempt_authority,
        11,
        "canonical-verified",
        &serde_json::json!({"canonical":canonical,"archive":archived,"displaced":observe_child(&attempt_authority,"displaced",store.root(),"displaced",request.issue)?}),
    )?;
    failpoint(&request, "canonical_verified")?;
    receipt_at(
        &attempt_authority,
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
    receipt_at(
        &attempt_authority,
        13,
        "recovered",
        &serde_json::to_value(&out)?,
    )?;
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

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_mount_identity_requires_statx_mount_id_bit() {
        let error = linux_mount_id_from_statx(0, 41)
            .expect_err("missing mount-id authority must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_mount_identity_uses_returned_statx_mount_id() {
        assert_eq!(
            linux_mount_id_from_statx(libc::STATX_MNT_ID, 41).expect("mount-id authority"),
            "mnt:41"
        );
    }

    #[cfg(unix)]
    #[test]
    fn receipt_read_keeps_opened_regular_file_authority_after_path_swap() {
        let temp = tempfile::tempdir().expect("tempdir");
        let receipt = temp.path().join("001-prepared.json");
        let displaced = temp.path().join("001-prepared.original.json");
        let replacement = temp.path().join("replacement.json");
        let expected = b"opened receipt authority\n";
        fs::write(&receipt, expected).expect("receipt");
        fs::set_permissions(&receipt, fs::Permissions::from_mode(0o600)).expect("receipt mode");
        fs::write(&replacement, b"replacement authority\n").expect("replacement");
        fs::set_permissions(&replacement, fs::Permissions::from_mode(0o600))
            .expect("replacement mode");

        let bytes = read_receipt_file_after_open(&receipt, || {
            fs::rename(&receipt, &displaced)?;
            std::os::unix::fs::symlink(&replacement, &receipt)?;
            Ok(())
        })
        .expect("read retained receipt handle");

        assert_eq!(bytes, expected);
        assert!(receipt
            .symlink_metadata()
            .expect("replacement metadata")
            .file_type()
            .is_symlink());
    }

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

    #[test]
    fn retained_recovery_root_rejects_post_open_path_swap() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join(".298.recovery");
        fs::create_dir(&root).expect("root");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("root mode");
        let retained = open_private_recovery_dir(&root, "recovery root", None).expect("open root");
        let displaced = temp.path().join(".298.recovery.original");
        fs::rename(&root, &displaced).expect("displace root");
        fs::create_dir(&root).expect("replacement root");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("replacement mode");
        let error = retained
            .validate_binding(&root, "recovery root")
            .expect_err("root substitution must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
        assert!(directory_names(&retained.file)
            .expect("retained inventory")
            .is_empty());
    }

    #[test]
    fn retained_recovery_attempt_rejects_post_open_path_swap() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join(".298.recovery");
        let attempt = root.join("attempt");
        fs::create_dir(&root).expect("root");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("root mode");
        fs::create_dir(&attempt).expect("attempt");
        fs::set_permissions(&attempt, fs::Permissions::from_mode(0o700)).expect("attempt mode");
        let root_fd = open_private_recovery_dir(&root, "recovery root", None).expect("open root");
        let name = std::ffi::CString::new("attempt").unwrap();
        let retained = root_fd
            .open_child(&name, "recovery attempt")
            .expect("open attempt");
        let displaced = root.join("attempt.original");
        fs::rename(&attempt, &displaced).expect("displace attempt");
        fs::create_dir(&attempt).expect("replacement attempt");
        fs::set_permissions(&attempt, fs::Permissions::from_mode(0o700)).expect("replacement mode");
        let error = retained
            .validate_binding(&attempt, "recovery attempt")
            .expect_err("attempt substitution must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
        assert!(directory_names(&retained.file)
            .expect("retained inventory")
            .is_empty());
    }
}
