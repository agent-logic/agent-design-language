//! Verified terminal archival for generated issue residue.
//! This is cleanup evidence, never a second lifecycle state authority.
use super::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Archive {
    pub digest: String,
    entries: BTreeMap<String, Value>,
}
fn admitted(path: &str, issue: u64) -> bool {
    [
        format!(".csdlc/issues/{issue}/"),
        format!(".csdlc/evidence/{issue}/"),
        format!(".csdlc/transactions/completed/{issue}/"),
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
        || path == format!(".csdlc/locks/{issue}.lock")
        || path.starts_with(&format!(".csdlc/v3/issues/{issue}/"))
}
fn git_bytes(root: &Path, args: &[&str]) -> Result<Vec<u8>, TerminalFinding> {
    let out = std::process::Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|_| {
            finding(
                "cleanup_archive_git_failed",
                "cannot inspect archive candidate",
            )
        })?;
    if !out.status.success() {
        return Err(finding(
            "cleanup_archive_git_failed",
            "cannot inspect archive candidate",
        ));
    }
    Ok(out.stdout)
}
fn strings(bytes: &[u8]) -> Result<Vec<&str>, TerminalFinding> {
    std::str::from_utf8(bytes)
        .map(|text| text.split('\0').filter(|part| !part.is_empty()).collect())
        .map_err(|_| {
            finding(
                "cleanup_archive_path_encoding",
                "archive paths must be UTF-8",
            )
        })
}
fn fingerprint(path: &Path) -> Result<Value, TerminalFinding> {
    for ancestor in path.parent().into_iter().flat_map(Path::ancestors) {
        if ancestor
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err(finding(
                "cleanup_archive_parent_symlink_denied",
                "archive input parent cannot redirect through symlinks",
            ));
        }
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| finding("cleanup_archive_input_missing", "archive input disappeared"))?;
    if metadata.file_type().is_symlink() {
        let target = fs::read_link(path).map_err(|_| {
            finding(
                "cleanup_archive_symlink_read_failed",
                "cannot retain symlink text",
            )
        })?;
        let target = target.to_str().ok_or_else(|| {
            finding(
                "cleanup_archive_path_encoding",
                "symlink target must be UTF-8",
            )
        })?;
        Ok(
            json!({"kind":"symlink","target":target,"digest":blake3::hash(target.as_bytes()).to_hex().to_string()}),
        )
    } else if metadata.is_file() {
        let bytes = fs::read(path).map_err(|_| {
            finding(
                "cleanup_archive_input_unreadable",
                "archive input unreadable",
            )
        })?;
        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let permissions = 0;
        Ok(
            json!({"kind":"file","bytes":bytes.len(),"permissions":permissions,"digest":blake3::hash(&bytes).to_hex().to_string()}),
        )
    } else {
        Err(finding(
            "cleanup_archive_special_file_denied",
            "only regular files and symlinks may be archived",
        ))
    }
}

fn preview_with_partial_removal(
    candidate: &Path,
    issue: u64,
    allow_partial_removal: bool,
) -> Result<Archive, TerminalFinding> {
    let lock_path = candidate.join(format!(".csdlc/locks/{issue}.lock"));
    if lock_path.symlink_metadata().is_ok() {
        let lock = fingerprint(&lock_path)?;
        if lock["kind"] != "file" || lock["bytes"] != 0 {
            return Err(finding(
                "cleanup_archive_lock_invalid",
                "native issue lock must be an empty regular control file",
            ));
        }
    }
    let status = git_bytes(
        candidate,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    for entry in strings(&status)? {
        let path = entry.get(3..).unwrap_or_default();
        let untracked = entry.starts_with("?? ");
        if untracked && admitted(path, issue) {
            continue;
        }
        // The archive retains filesystem bytes, not the worktree index. Even
        // an admitted issue file may have distinct staged evidence; never let
        // forced worktree removal discard it (including on cleanup recovery).
        if !untracked
            && entry
                .as_bytes()
                .first()
                .is_some_and(|status| *status != b' ')
        {
            return Err(finding(
                "cleanup_archive_staged_changes",
                "cleanup refuses staged changes; preserve or unstage them before archival",
            ));
        }
        let tracked_issue_record = !untracked
            && entry.len() >= 4
            && (allow_partial_removal
                || !entry.as_bytes()[..2]
                    .iter()
                    .any(|status| matches!(*status, b'D' | b'R' | b'C' | b'U' | b'?')))
            && admitted(path, issue);
        if tracked_issue_record {
            continue;
        }
        return Err(finding(
            "cleanup_archive_foreign_or_tracked_dirty",
            "cleanup refuses tracked changes outside the exact generated issue namespace, destructive tracked changes, and unrelated untracked files",
        ));
    }
    let ignored = git_bytes(
        candidate,
        &[
            "ls-files",
            "--others",
            "--ignored",
            "--exclude-standard",
            "-z",
        ],
    )?;
    for path in strings(&ignored)? {
        if !admitted(path, issue)
            && !["target/", "csdlc-v3/target/", "adl/target/"]
                .iter()
                .any(|prefix| path.starts_with(prefix))
        {
            return Err(finding("cleanup_archive_unclassified_ignored_file","ignored residue outside exact issue evidence or declared disposable Rust targets requires explicit preservation"));
        }
    }
    let mut entries = BTreeMap::new();
    fn visit(
        root: &Path,
        path: &Path,
        entries: &mut BTreeMap<String, Value>,
    ) -> Result<(), TerminalFinding> {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(_) => {
                return Err(finding(
                    "cleanup_archive_input_unreadable",
                    "cannot inspect generated residue",
                ))
            }
        };
        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            for entry in fs::read_dir(path).map_err(|_| {
                finding(
                    "cleanup_archive_directory_unreadable",
                    "cannot inspect generated residue directory",
                )
            })? {
                visit(
                    root,
                    &entry
                        .map_err(|_| {
                            finding(
                                "cleanup_archive_directory_unreadable",
                                "cannot inspect directory entry",
                            )
                        })?
                        .path(),
                    entries,
                )?;
            }
        } else {
            let relative = path
                .strip_prefix(root)
                .ok()
                .and_then(Path::to_str)
                .ok_or_else(|| {
                    finding(
                        "cleanup_archive_path_encoding",
                        "generated path must remain within worktree",
                    )
                })?;
            entries.insert(relative.into(), fingerprint(path)?);
        }
        Ok(())
    }
    for relative in [
        format!(".csdlc/issues/{issue}"),
        format!(".csdlc/evidence/{issue}"),
        format!(".csdlc/transactions/completed/{issue}"),
        format!(".csdlc/v3/issues/{issue}"),
    ] {
        let path = candidate.join(relative);
        // Intermediate symlinks cannot redirect traversal into another tree.
        for ancestor in path
            .parent()
            .into_iter()
            .flat_map(Path::ancestors)
            .take_while(|ancestor| *ancestor != candidate)
        {
            if ancestor
                .symlink_metadata()
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
            {
                return Err(finding(
                    "cleanup_archive_parent_symlink_denied",
                    "generated residue parent may not be a symlink",
                ));
            }
        }
        visit(candidate, &path, &mut entries)?;
    }
    if entries.keys().any(|path| !admitted(path, issue)) {
        return Err(finding(
            "cleanup_archive_path_not_admitted",
            "generated archive inventory escaped its exact issue namespace",
        ));
    }
    let bytes = serde_json::to_vec(&entries).map_err(|_| {
        finding(
            "cleanup_archive_manifest_invalid",
            "cannot serialize generated inventory",
        )
    })?;
    Ok(Archive {
        digest: blake3::hash(&bytes).to_hex().to_string(),
        entries,
    })
}

pub(super) fn preview(candidate: &Path, issue: u64) -> Result<Archive, TerminalFinding> {
    preview_with_partial_removal(candidate, issue, false)
}

pub(super) fn verify_partial_source(
    candidate: &Path,
    issue: u64,
    identity: &Value,
) -> Result<Archive, TerminalFinding> {
    let digest = identity["inventory_digest"].as_str().ok_or_else(|| {
        finding(
            "cleanup_archive_manifest_invalid",
            "retained cleanup archive digest is missing",
        )
    })?;
    let entries: BTreeMap<String, Value> = serde_json::from_value(identity["files"].clone())
        .map_err(|_| {
            finding(
                "cleanup_archive_manifest_invalid",
                "retained cleanup archive inventory is invalid",
            )
        })?;
    let remaining = preview_with_partial_removal(candidate, issue, true)?;
    let tracked = git_bytes(candidate, &["ls-files", "-z"])?;
    let tracked: BTreeSet<_> = strings(&tracked)?.into_iter().map(str::to_owned).collect();
    let status = git_bytes(
        candidate,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let dirty: BTreeSet<_> = strings(&status)?
        .into_iter()
        .map(|entry| entry.get(3..).unwrap_or_default().to_owned())
        .collect();
    if remaining
        .entries
        .iter()
        .any(|(path, fingerprint)| match entries.get(path) {
            Some(expected) => expected != fingerprint,
            None => !tracked.contains(path) || dirty.contains(path),
        })
    {
        return Err(finding(
            "cleanup_changed_after_archive",
            "remaining generated residue differs from the verified retained archive",
        ));
    }
    Ok(Archive {
        digest: digest.to_owned(),
        entries,
    })
}

pub(super) fn verify_archived_source_removed(
    candidate: &Path,
    issue: u64,
) -> Result<(), TerminalFinding> {
    let remaining = preview_with_partial_removal(candidate, issue, true)?;
    if !remaining.entries.is_empty() {
        return Err(finding(
            "cleanup_changed_after_archive",
            "generated residue remained after verified archival",
        ));
    }
    Ok(())
}

fn safe_directory(path: &Path) -> Result<(), TerminalFinding> {
    for ancestor in path.ancestors() {
        if ancestor
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err(finding(
                "cleanup_archive_destination_symlink_denied",
                "archive destination cannot contain a symlink",
            ));
        }
    }
    fs::create_dir_all(path).map_err(|_| {
        finding(
            "cleanup_archive_destination_unavailable",
            "archive destination is not writable",
        )
    })
}
pub(super) fn execute(
    primary: &Path,
    candidate: &Path,
    issue: u64,
    archive: &Archive,
) -> Result<(), TerminalFinding> {
    if preview(candidate, issue)?.digest != archive.digest {
        return Err(finding(
            "cleanup_archive_preview_stale",
            "generated residue changed after preview",
        ));
    }
    let lock_path = candidate.join(format!(".csdlc/locks/{issue}.lock"));
    safe_directory(lock_path.parent().unwrap())?;
    let issue_lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| {
            finding(
                "cleanup_archive_lock_unavailable",
                "cannot acquire native issue lock",
            )
        })?;
    issue_lock.try_lock_exclusive().map_err(|_| {
        finding(
            "cleanup_archive_issue_busy",
            "another native writer owns the issue",
        )
    })?;
    if preview(candidate, issue)?.digest != archive.digest {
        return Err(finding(
            "cleanup_archive_preview_stale",
            "generated residue changed before issue-lock admission",
        ));
    }
    let state = crate::commands::local::operational_state_root(primary).map_err(|_| {
        finding(
            "cleanup_archive_state_root_invalid",
            "cannot resolve Git metadata archive",
        )
    })?;
    let destination = state.join(format!("archives/{issue}-intent-{}", archive.digest));
    safe_directory(&destination)?;
    let manifest = json!({"schema":"csdlc.v3.intent_cleanup_archive.v1","issue":issue,"inventory_digest":archive.digest,"files":archive.entries,"native_lock":{"path":format!(".csdlc/locks/{issue}.lock"),"kind":"empty_regular_control_file"},"disposable_cache_roots":["target/","csdlc-v3/target/","adl/target/"]});
    for (relative, expected) in &archive.entries {
        let source = candidate.join(relative);
        let target = destination.join("files").join(relative);
        safe_directory(target.parent().unwrap())?;
        if target.symlink_metadata().is_ok() {
            if fingerprint(&target)? != *expected {
                return Err(finding(
                    "cleanup_archive_existing_conflict",
                    "existing archive bytes conflict; originals retained",
                ));
            }
            continue;
        }
        if expected["kind"] == "symlink" {
            #[cfg(unix)]
            std::os::unix::fs::symlink(
                fs::read_link(&source).map_err(|_| {
                    finding(
                        "cleanup_archive_symlink_read_failed",
                        "cannot retain symlink",
                    )
                })?,
                &target,
            )
            .map_err(|_| {
                finding(
                    "cleanup_archive_copy_failed",
                    "cannot create archived symlink",
                )
            })?;
            #[cfg(not(unix))]
            return Err(finding(
                "cleanup_archive_platform_not_supported",
                "symlink archival requires Unix",
            ));
        } else {
            let mut output = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&target)
                .map_err(|_| {
                    finding("cleanup_archive_copy_failed", "cannot create archive file")
                })?;
            let mut input = fs::File::open(&source).map_err(|_| {
                finding(
                    "cleanup_archive_input_unreadable",
                    "cannot copy original bytes",
                )
            })?;
            std::io::copy(&mut input, &mut output).map_err(|_| {
                finding(
                    "cleanup_archive_copy_failed",
                    "copy failed; originals retained",
                )
            })?;
            fs::set_permissions(
                &target,
                fs::metadata(&source)
                    .map_err(|_| {
                        finding(
                            "cleanup_archive_permissions_failed",
                            "cannot read original permissions",
                        )
                    })?
                    .permissions(),
            )
            .map_err(|_| {
                finding(
                    "cleanup_archive_permissions_failed",
                    "cannot retain original permissions",
                )
            })?;
            output.sync_all().map_err(|_| {
                finding(
                    "cleanup_archive_sync_failed",
                    "archive data durability failed",
                )
            })?;
        }
        if fingerprint(&source)? != *expected || fingerprint(&target)? != *expected {
            return Err(finding(
                "cleanup_archive_verification_failed",
                "copied bytes differ; originals retained",
            ));
        }
        fs::File::open(target.parent().unwrap())
            .and_then(|directory| directory.sync_all())
            .map_err(|_| {
                finding(
                    "cleanup_archive_sync_failed",
                    "archive directory durability failed",
                )
            })?;
    }
    write_staged(
        &destination.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest).map_err(|_| {
            finding(
                "cleanup_archive_manifest_invalid",
                "cannot serialize archive manifest",
            )
        })?,
    )?;
    fs::File::open(&destination)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| {
            finding(
                "cleanup_archive_sync_failed",
                "archive manifest durability failed",
            )
        })?;
    // File fsync does not persist intermediate directory entries. Establish
    // reachability of every archived byte, including resumed archive paths,
    // bottom-up through the existing native state parent before any removal.
    let boundary = state.parent().ok_or_else(|| {
        finding(
            "cleanup_archive_state_root_invalid",
            "native archive parent is missing",
        )
    })?;
    let mut directories = BTreeSet::new();
    for relative in archive.entries.keys() {
        let parent = destination
            .join("files")
            .join(relative)
            .parent()
            .unwrap()
            .to_path_buf();
        for directory in parent.ancestors() {
            if !directory.starts_with(boundary) {
                break;
            }
            directories.insert(directory.to_path_buf());
        }
    }
    let mut directories: Vec<_> = directories.into_iter().collect();
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for directory in directories {
        fs::File::open(&directory)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| {
                finding(
                    "cleanup_archive_sync_failed",
                    "archive ancestor durability failed; all originals retained",
                )
            })?;
    }
    if preview(candidate, issue)?.digest != archive.digest {
        return Err(finding(
            "cleanup_archive_preview_stale",
            "source changed during archive; all originals retained",
        ));
    }
    // Every byte is durably archived before the first removal. Keep the index
    // until last so interrupted removal remains tied to its original issue.
    let index = format!(".csdlc/issues/{issue}/index.json");
    let mut paths: Vec<_> = archive.entries.keys().collect();
    paths.sort_by_key(|path| *path == &index);
    let mut removed_non_index = false;
    for relative in paths {
        let source = candidate.join(relative);
        if fingerprint(&source)? != archive.entries[relative] {
            return Err(finding(
                "cleanup_archive_source_changed",
                "source changed during removal; verified archive retained",
            ));
        }
        fs::remove_file(&source).map_err(|_| {
            finding(
                "cleanup_archive_removal_partial",
                "removal interrupted; all original bytes remain in the verified archive",
            )
        })?;
        if relative != &index && !removed_non_index {
            removed_non_index = true;
            #[cfg(debug_assertions)]
            if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
                == Ok("cleanup_after_first_source_removal")
            {
                std::process::exit(91);
            }
        }
        let mut parent = source.parent();
        while let Some(directory) = parent {
            if directory == candidate || directory == candidate.join(".csdlc") {
                break;
            }
            if fs::remove_dir(directory).is_err() {
                break;
            }
            parent = directory.parent();
        }
    }
    // The index is gone before this empty control file is unlinked, so a new
    // writer cannot regain issue admission by reopening a different lock inode.
    fs::remove_file(&lock_path).map_err(|_| {
        finding(
            "cleanup_archive_lock_cleanup_failed",
            "verified archive retained; empty control-file cleanup failed",
        )
    })?;
    let _ = fs::remove_dir(lock_path.parent().unwrap());
    Ok(())
}

/// Read-only terminal continuation evidence for a registered target whose index
/// was already archived before interrupted Git removal. Never restores cards or
/// authorizes implementation from archived state.
fn retained_archive(
    primary: &Path,
    candidate: &Path,
    issue: u64,
    expected_digest: Option<&str>,
) -> Result<Option<(Value, Value)>, TerminalFinding> {
    let state = crate::commands::local::operational_state_root(primary).map_err(|_| {
        finding(
            "cleanup_archive_state_root_invalid",
            "cannot locate archive evidence",
        )
    })?;
    let archives = state.join("archives");
    let dirs = match fs::read_dir(&archives) {
        Ok(dirs) => dirs,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => {
            return Err(finding(
                "cleanup_archive_unreadable",
                "cannot inspect retained archive evidence",
            ))
        }
    };
    let mut accepted = None;
    for entry in dirs {
        let entry = entry
            .map_err(|_| finding("cleanup_archive_unreadable", "cannot inspect archive entry"))?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with(&format!("{issue}-intent-")) {
            continue;
        }
        if expected_digest.is_some_and(|digest| name != format!("{issue}-intent-{digest}")) {
            continue;
        }
        let directory = entry.path();
        let manifest_path = directory.join("manifest.json");
        if !manifest_path.exists() {
            continue;
        }
        if fingerprint(&manifest_path)?["kind"] != "file" {
            return Err(finding(
                "cleanup_archive_manifest_invalid",
                "archive manifest must be regular",
            ));
        }
        let manifest: Value =
            serde_json::from_slice(&fs::read(&manifest_path).map_err(|_| {
                finding("cleanup_archive_unreadable", "cannot read archive manifest")
            })?)
            .map_err(|_| {
                finding(
                    "cleanup_archive_manifest_invalid",
                    "retained archive manifest is invalid",
                )
            })?;
        let files: BTreeMap<String, Value> = serde_json::from_value(manifest["files"].clone())
            .map_err(|_| {
                finding(
                    "cleanup_archive_manifest_invalid",
                    "archive files are invalid",
                )
            })?;
        let digest = blake3::hash(&serde_json::to_vec(&files).map_err(|_| {
            finding(
                "cleanup_archive_manifest_invalid",
                "archive inventory cannot serialize",
            )
        })?)
        .to_hex()
        .to_string();
        if manifest["schema"] != "csdlc.v3.intent_cleanup_archive.v1"
            || manifest["issue"] != issue
            || manifest["inventory_digest"] != digest
            || name != format!("{issue}-intent-{digest}")
        {
            return Err(finding(
                "cleanup_archive_manifest_mismatch",
                "archive identity or inventory changed",
            ));
        }
        let index_ref = format!(".csdlc/issues/{issue}/index.json");
        for (relative, expected) in &files {
            if !admitted(relative, issue)
                || Path::new(relative).components().any(|part| {
                    matches!(
                        part,
                        Component::ParentDir | Component::RootDir | Component::Prefix(_)
                    )
                })
            {
                return Err(finding(
                    "cleanup_archive_path_not_admitted",
                    "archive path is outside exact issue namespace",
                ));
            }
            if fingerprint(&directory.join("files").join(relative))? != *expected {
                return Err(finding(
                    "cleanup_archive_verification_failed",
                    "retained archive bytes changed",
                ));
            }
        }
        let archived_index = directory.join("files").join(&index_ref);
        let live_index = candidate.join(&index_ref);
        let index_path = if files.contains_key(&index_ref) {
            archived_index.as_path()
        } else if live_index.is_file() {
            live_index.as_path()
        } else {
            continue;
        };
        if fingerprint(index_path)?["kind"] != "file" {
            return Err(finding(
                "cleanup_archive_index_invalid",
                "archived index must be a regular file",
            ));
        }
        let index: Value = serde_json::from_slice(&fs::read(index_path).map_err(|_| {
            finding(
                "cleanup_archive_index_invalid",
                "archived index unavailable",
            )
        })?)
        .map_err(|_| finding("cleanup_archive_index_invalid", "archived index is invalid"))?;
        if index["schema"] != "csdlc.v3.local_state.v1"
            || index["issue"] != issue
            || index["worktree"].as_str() != candidate.to_str()
        {
            return Err(finding(
                "cleanup_archive_index_invalid",
                "archived index has another target",
            ));
        }
        let identity = json!({"schema":"csdlc.v3.semantic_cleanup_archive_identity.v1",
            "issue":issue,"candidate":candidate,"inventory_digest":digest,"files":files});
        let record = (index, identity);
        if accepted
            .as_ref()
            .is_some_and(|previous| previous != &record)
        {
            return Err(finding(
                "cleanup_archive_index_ambiguous",
                "multiple archived issue versions require an explicit operator disposition",
            ));
        }
        accepted = Some(record);
    }
    Ok(accepted)
}

pub(super) fn retained_index(
    primary: &Path,
    candidate: &Path,
    issue: u64,
) -> Result<Option<Value>, TerminalFinding> {
    retained_archive(primary, candidate, issue, None).map(|record| record.map(|(index, _)| index))
}

pub(super) fn matching_retained_index(
    primary: &Path,
    candidate: &Path,
    issue: u64,
    digest: &str,
) -> Result<Option<Value>, TerminalFinding> {
    retained_archive(primary, candidate, issue, Some(digest))
        .map(|record| record.map(|(index, _)| index))
}

pub(super) fn matching_retained_semantic_identity(
    primary: &Path,
    candidate: &Path,
    issue: u64,
    expected: &[u8],
) -> Result<Option<Vec<u8>>, TerminalFinding> {
    let expected: Value = serde_json::from_slice(expected).map_err(|_| {
        finding(
            "cleanup_archive_manifest_invalid",
            "retained semantic archive identity is invalid",
        )
    })?;
    let digest = expected["inventory_digest"].as_str().ok_or_else(|| {
        finding(
            "cleanup_archive_manifest_invalid",
            "retained semantic archive digest is missing",
        )
    })?;
    let Some((_, identity)) = retained_archive(primary, candidate, issue, Some(digest))? else {
        return Ok(None);
    };
    if identity != expected {
        return Err(finding(
            "cleanup_archive_verification_failed",
            "retained archive does not match the pending semantic identity",
        ));
    }
    serde_json::to_vec(&identity).map(Some).map_err(|_| {
        finding(
            "cleanup_archive_manifest_invalid",
            "cannot serialize retained archive identity",
        )
    })
}

/// Exact verified archive input identity, reproducible after source removal.
/// Preview performs no writes. Retained evidence is accepted only after every
/// archived byte and the original checkout identity have been revalidated.
pub(super) fn semantic_identity(
    primary: &Path,
    candidate: &Path,
    issue: u64,
) -> Result<Vec<u8>, TerminalFinding> {
    let live_index = candidate.join(format!(".csdlc/issues/{issue}/index.json"));
    let identity = if live_index.exists() {
        let archive = preview(candidate, issue)?;
        json!({"schema":"csdlc.v3.semantic_cleanup_archive_identity.v1",
            "issue":issue,"candidate":candidate,"inventory_digest":archive.digest,"files":archive.entries})
    } else if let Some((_, identity)) = retained_archive(primary, candidate, issue, None)? {
        identity
    } else {
        let archive = preview(candidate, issue)?;
        json!({"schema":"csdlc.v3.semantic_cleanup_archive_identity.v1",
            "issue":issue,"candidate":candidate,"inventory_digest":archive.digest,"files":archive.entries})
    };
    serde_json::to_vec(&identity).map_err(|_| {
        finding(
            "cleanup_archive_manifest_invalid",
            "cannot serialize exact archive identity",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn git(root: &Path, args: &[&str]) {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "{args:?}: {output:?}");
    }

    #[test]
    fn issue_1092_archive_preserves_modified_tracked_issue_projection_only() {
        let root = std::env::current_dir()
            .unwrap()
            .join("target/intent-archive-tests")
            .join(format!("issue-1092-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(root.join(".csdlc/v3/issues/505")).unwrap();
        fs::create_dir_all(root.join(".csdlc/evidence/505")).unwrap();
        fs::write(root.join(".csdlc/v3/issues/505/state.json"), b"old\n").unwrap();
        fs::write(root.join("tracked.txt"), b"old\n").unwrap();
        git(&root, &["init", "-q"]);
        git(&root, &["config", "user.email", "fixture@example.com"]);
        git(&root, &["config", "user.name", "Fixture"]);
        git(&root, &["add", "."]);
        git(&root, &["commit", "-q", "-m", "fixture"]);

        fs::write(
            root.join(".csdlc/v3/issues/505/state.json"),
            b"closed out\n",
        )
        .unwrap();
        fs::write(root.join(".csdlc/evidence/505/terminal.json"), b"{}\n").unwrap();
        let archive = preview(&root, 505).unwrap();
        assert!(archive
            .entries
            .contains_key(".csdlc/v3/issues/505/state.json"));
        assert!(archive
            .entries
            .contains_key(".csdlc/evidence/505/terminal.json"));

        fs::write(root.join("tracked.txt"), b"foreign change\n").unwrap();
        let finding = match preview(&root, 505) {
            Ok(_) => panic!("foreign tracked change was admitted"),
            Err(finding) => finding,
        };
        assert_eq!(finding.code, "cleanup_archive_foreign_or_tracked_dirty");
        fs::remove_dir_all(&root).unwrap();
    }

    // PVF #1098: required deterministic local archive guard proof; small Git
    // fixture, no network. Covers initial and interrupted-cleanup admission.
    #[test]
    fn issue_1098_archive_refuses_staged_evidence_during_recovery() {
        let root = std::env::current_dir()
            .unwrap()
            .join("target/intent-archive-tests")
            .join(format!("issue-1098-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(root.join(".csdlc/evidence/505")).unwrap();
        let relative = ".csdlc/evidence/505/proof.txt";
        let file = root.join(relative);
        fs::write(&file, b"committed\n").unwrap();
        git(&root, &["init", "-q"]);
        git(&root, &["config", "user.email", "fixture@example.com"]);
        git(&root, &["config", "user.name", "Fixture"]);
        git(&root, &["add", "."]);
        git(&root, &["commit", "-q", "-m", "fixture"]);
        fs::write(&file, b"staged evidence\n").unwrap();
        git(&root, &["add", relative]);
        fs::write(&file, b"working evidence\n").unwrap();
        for partial in [false, true] {
            let finding = match preview_with_partial_removal(&root, 505, partial) {
                Ok(_) => panic!("distinct staged evidence admitted, partial={partial}"),
                Err(finding) => finding,
            };
            assert_eq!(finding.code, "cleanup_archive_staged_changes");
            assert_eq!(fs::read(&file).unwrap(), b"working evidence\n");
        }
        git(&root, &["reset", "--quiet", "HEAD", "--", relative]);
        assert!(
            preview(&root, 505).is_ok(),
            "unstaged evidence should remain archivable"
        );
        fs::remove_dir_all(&root).unwrap();
    }
}
