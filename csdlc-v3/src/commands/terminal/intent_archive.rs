//! Verified terminal archival for generated, untracked issue residue only.
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

pub(super) fn preview(candidate: &Path, issue: u64) -> Result<Archive, TerminalFinding> {
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
        if !entry.starts_with("?? ") || !admitted(&entry[3..], issue) {
            return Err(finding(
                "cleanup_archive_foreign_or_tracked_dirty",
                "cleanup refuses tracked changes and unrelated untracked files",
            ));
        }
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
    let tracked = git_bytes(candidate, &["ls-files", "-z"])?;
    let tracked: BTreeSet<_> = strings(&tracked)?.into_iter().collect();
    let mut entries = BTreeMap::new();
    fn visit(
        root: &Path,
        path: &Path,
        tracked: &BTreeSet<&str>,
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
                    tracked,
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
            if !tracked.contains(relative) {
                entries.insert(relative.into(), fingerprint(path)?);
            }
        }
        Ok(())
    }
    for relative in [
        format!(".csdlc/issues/{issue}"),
        format!(".csdlc/evidence/{issue}"),
        format!(".csdlc/transactions/completed/{issue}"),
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
        visit(candidate, &path, &tracked, &mut entries)?;
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
pub(super) fn retained_index(
    primary: &Path,
    candidate: &Path,
    issue: u64,
) -> Result<Option<Value>, TerminalFinding> {
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
        if !files.contains_key(&index_ref) {
            continue;
        }
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
        if files[&index_ref]["kind"] != "file" {
            return Err(finding(
                "cleanup_archive_index_invalid",
                "archived index must be a regular file",
            ));
        }
        let index: Value = serde_json::from_slice(
            &fs::read(directory.join("files").join(&index_ref)).map_err(|_| {
                finding(
                    "cleanup_archive_index_invalid",
                    "archived index unavailable",
                )
            })?,
        )
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
        if accepted.as_ref().is_some_and(|previous| previous != &index) {
            return Err(finding(
                "cleanup_archive_index_ambiguous",
                "multiple archived issue versions require an explicit operator disposition",
            ));
        }
        accepted = Some(index);
    }
    Ok(accepted)
}
