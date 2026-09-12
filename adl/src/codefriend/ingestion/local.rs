//! Local acquisition reads immutable Git objects, never working files or filters.
use super::{unsafe_content, validate_repository, GitObjectFormat, Object, Packet, Scope, SCHEMA};
use anyhow::{ensure, Result};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

fn git(root: &Path, args: &[&str], limit: u64) -> Result<Vec<u8>> {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_LITERAL_PATHSPECS", "1")
        .env("GIT_NO_REPLACE_OBJECTS", "1")
        .env("GIT_NO_LAZY_FETCH", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_ALLOW_PROTOCOL", "")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    for key in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_CONFIG_COUNT",
    ] {
        command.env_remove(key);
    }
    let mut child = command
        .spawn()
        .map_err(|_| anyhow::anyhow!("git_start_failed"))?;
    let mut bytes = Vec::new();
    let read = child
        .stdout
        .take()
        .expect("piped stdout")
        .take(limit + 1)
        .read_to_end(&mut bytes);
    if read.is_err() || bytes.len() as u64 > limit {
        let _ = child.kill();
        let _ = child.wait();
        anyhow::bail!("git_output_limit_exceeded");
    }
    ensure!(
        child
            .wait()
            .map_err(|_| anyhow::anyhow!("git_wait_failed"))?
            .success(),
        "git_read_failed"
    );
    Ok(bytes)
}
fn text(root: &Path, args: &[&str], limit: u64) -> Result<String> {
    String::from_utf8(git(root, args, limit)?)
        .map(|s| s.trim_end().to_string())
        .map_err(|_| anyhow::anyhow!("git_non_utf8_metadata"))
}

pub fn acquire(root: &Path, repository: &str, revision: &str, scope: Scope) -> Result<Packet> {
    acquire_checked(root, repository, revision, scope, || {})
}
fn acquire_checked(
    root: &Path,
    repository: &str,
    revision: &str,
    scope: Scope,
    after_capture: impl FnOnce(),
) -> Result<Packet> {
    scope.validate()?;
    validate_repository(repository)?;
    ensure!(super::object_id(revision), "exact_revision_required");
    let root = root
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("checkout_not_found"))?;
    let top = text(&root, &["rev-parse", "--show-toplevel"], 4096)?;
    ensure!(
        Path::new(&top).canonicalize().ok().as_ref() == Some(&root),
        "repository_root_required"
    );
    let object_format = match text(&root, &["rev-parse", "--show-object-format"], 32)?.as_str() {
        "sha1" => GitObjectFormat::Sha1,
        "sha256" => GitObjectFormat::Sha256,
        _ => anyhow::bail!("unsupported_git_object_format"),
    };
    ensure!(
        object_format.accepts(revision),
        "revision_object_format_mismatch"
    );
    let origin = text(&root, &["config", "--get", "remote.origin.url"], 2048)?;
    ensure!(
        origin.trim_end_matches(".git") == repository,
        "repository_origin_mismatch"
    );
    ensure!(
        text(&root, &["cat-file", "-t", revision], 20)? == "commit",
        "commit_revision_required"
    );
    let head = text(&root, &["rev-parse", "--verify", "HEAD"], 100)?;
    let mut objects = Vec::new();
    let mut total = 0u64;
    for path in scope.paths() {
        let metadata = git(&root, &["ls-tree", "-z", revision, "--", path], 4096)?;
        let support = if scope.analysis.iter().any(|p| p == path) && path.ends_with(".rs") {
            "rust_source_not_yet_analyzed"
        } else {
            "context_or_unsupported_analysis"
        };
        let mut object = Object {
            path: path.into(),
            source_object: None,
            source_bytes: 0,
            content_digest: None,
            content: None,
            disposition: "missing".into(),
            analysis_support: support.into(),
        };
        if !metadata.is_empty() {
            let entry =
                std::str::from_utf8(&metadata).map_err(|_| anyhow::anyhow!("invalid_git_entry"))?;
            let (fields, name) = entry
                .strip_suffix('\0')
                .and_then(|s| s.split_once('\t'))
                .ok_or_else(|| anyhow::anyhow!("invalid_git_entry"))?;
            ensure!(name == path, "scope_entry_mismatch");
            let fields: Vec<_> = fields.split(' ').collect();
            ensure!(fields.len() == 3, "invalid_git_entry");
            ensure!(fields[0] != "120000", "symlink_input_rejected");
            object.source_object = Some(fields[2].into());
            if fields[1] != "blob" || !matches!(fields[0], "100644" | "100755") {
                object.disposition = "omitted_unsupported_object".into();
            } else {
                let size: u64 = text(&root, &["cat-file", "-s", fields[2]], 30)?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("invalid_object_size"))?;
                total = total
                    .checked_add(size)
                    .ok_or_else(|| anyhow::anyhow!("byte_limit_exceeded"))?;
                ensure!(
                    size <= scope.max_file_bytes && total <= scope.max_bytes,
                    "byte_limit_exceeded"
                );
                object.source_bytes = size;
                let bytes = git(
                    &root,
                    &["cat-file", "blob", fields[2]],
                    scope.max_file_bytes,
                )?;
                ensure!(bytes.len() as u64 == size, "source_changed_during_capture");
                match String::from_utf8(bytes) {
                    Ok(content) if !content.contains('\0') => {
                        if unsafe_content(path, &content) {
                            object.disposition = "omitted_unsafe".into();
                        } else {
                            object.disposition = "included".into();
                            object.content_digest = Some(super::digest(content.as_bytes()));
                            object.content = Some(content);
                        }
                    }
                    _ => object.disposition = "omitted_binary".into(),
                }
            }
        }
        objects.push(object);
    }
    after_capture();
    ensure!(
        text(&root, &["rev-parse", "--verify", "HEAD"], 100)? == head
            && text(&root, &["config", "--get", "remote.origin.url"], 2048)? == origin,
        "source_changed_during_capture"
    );
    let mut packet = Packet {
        schema: SCHEMA.into(),
        repository: repository.into(),
        revision: revision.into(),
        object_format,
        scope,
        scope_digest: String::new(),
        completeness: if objects.iter().all(|o| o.disposition == "included") {
            "complete_scoped_acquisition"
        } else {
            "partial"
        }
        .into(),
        objects,
        excluded_surfaces: "all_paths_outside_declared_scope".into(),
        checkout_policy: "committed_blobs_only_dirty_and_untracked_excluded".into(),
        review_state: "not_reviewed".into(),
        packet_id: String::new(),
    };
    packet.seal()?;
    Ok(packet)
}

/// Create-only atomic publication of an already validated, redacted packet.
/// Output must be outside the source checkout. Never replaces an existing artifact.
pub fn write_packet(root: &Path, output: &Path, packet: &Packet) -> Result<()> {
    packet.validate()?;
    let root = root
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("checkout_not_found"))?;
    let parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let parent = parent
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("output_parent_not_found"))?;
    ensure!(!parent.starts_with(&root), "output_inside_source_rejected");
    let filename = output
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("invalid_output_name"))?;
    let destination = parent.join(filename);
    let bytes = serde_json::to_vec_pretty(packet)?;
    ensure!(
        bytes.len() as u64 <= super::MAX_PACKET_BYTES,
        "packet_byte_limit_exceeded"
    );
    let staging = parent.join(format!(
        ".codefriend-{:032x}.pending",
        rand::random::<u128>()
    ));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&staging)
        .map_err(|_| anyhow::anyhow!("packet_stage_failed"))?;
    let result = (|| -> Result<()> {
        file.write_all(&bytes)
            .map_err(|_| anyhow::anyhow!("packet_write_failed"))?;
        file.sync_all()
            .map_err(|_| anyhow::anyhow!("packet_sync_failed"))?;
        fs::hard_link(&staging, &destination)
            .map_err(|_| anyhow::anyhow!("packet_publish_failed_or_exists"))?;
        Ok(())
    })();
    let cleanup = fs::remove_file(&staging);
    result?;
    cleanup.map_err(|_| anyhow::anyhow!("packet_staging_cleanup_failed"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // PVF: runtime; production acquisition mutation boundary; deterministic local
    // Git/temp fixture, no network/model; required #878 gate.
    #[test]
    fn rejects_checkout_identity_drift_during_capture() {
        let dir = tempfile::tempdir().unwrap();
        let run = |args: &[&str]| {
            assert!(Command::new("git")
                .arg("-C")
                .arg(dir.path())
                .args(args)
                .output()
                .unwrap()
                .status
                .success());
        };
        run(&["init"]);
        run(&[
            "remote",
            "add",
            "origin",
            "https://example.com/team/repo.git",
        ]);
        fs::write(dir.path().join("main.rs"), "fn main() {}\n").unwrap();
        run(&["add", "main.rs"]);
        run(&[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ]);
        let revision = text(dir.path(), &["rev-parse", "HEAD"], 100).unwrap();
        let result = acquire_checked(
            dir.path(),
            "https://example.com/team/repo",
            &revision,
            Scope {
                analysis: vec!["main.rs".into()],
                context: vec![],
                max_files: 1,
                max_bytes: 1024,
                max_file_bytes: 1024,
            },
            || {
                run(&[
                    "remote",
                    "set-url",
                    "origin",
                    "https://example.com/other/repo.git",
                ])
            },
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "source_changed_during_capture"
        );
    }
}
