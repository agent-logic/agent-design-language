//! Lightweight selection of a Runtime-owner-qualified installation.
//!
//! Qualification belongs to Runtime's install_runtime.py. This consumer reads its
//! managed `current -> versions/<archive>` selection and metadata; it does not
//! rehash payloads, install packages, or turn an arbitrary binary path into an owner.
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

pub(super) struct SelectedRuntime {
    executable: PathBuf,
}

fn regular_file(path: &Path) -> Result<std::fs::Metadata> {
    let metadata = std::fs::symlink_metadata(path)
        .with_context(|| format!("Runtime package file unavailable: {}", path.display()))?;
    if !metadata.is_file() {
        bail!(
            "Runtime package member is not a regular file: {}",
            path.display()
        );
    }
    Ok(metadata)
}

fn metadata_text(path: &Path) -> Result<String> {
    if regular_file(path)?.len() > 8 * 1024 * 1024 {
        bail!("Runtime package metadata is too large");
    }
    std::fs::read_to_string(path).context("Runtime package metadata is not UTF-8")
}

fn host_target() -> Result<String> {
    let suffix = match std::env::consts::OS {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        _ => bail!("Runtime package host is unsupported"),
    };
    Ok(format!("{}-{suffix}", std::env::consts::ARCH))
}

fn archive_identity(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

impl SelectedRuntime {
    /// Select only an explicitly pinned, previously qualified owner installation.
    /// The owner-managed installation directory must remain trusted after qualification.
    pub(super) fn from_installation(prefix: &Path, expected_archive: &str) -> Result<Self> {
        if !archive_identity(expected_archive) {
            bail!("Runtime package requires an exact lowercase archive SHA-256");
        }
        if !prefix.is_absolute() || prefix.canonicalize()? != prefix {
            bail!("Runtime package prefix must be absolute and unaliased");
        }
        let relative = Path::new("versions").join(expected_archive);
        if std::fs::read_link(prefix.join("current"))? != relative {
            bail!("Runtime package selection does not match the pinned archive");
        }
        let package = prefix.join(relative);
        if package.canonicalize()? != package {
            bail!("Runtime package version is aliased");
        }
        if metadata_text(&package.join("archive.sha256"))?.trim() != expected_archive {
            bail!("Runtime installed archive identity mismatch");
        }
        // Retain the owner's archive receipt without reading or hashing the payload again.
        regular_file(&package.join("archive.tar"))?;
        let manifest: serde_json::Value =
            serde_json::from_str(&metadata_text(&package.join("manifest.json"))?)?;
        if manifest["schema"] != "runtime.local-package.v1" || manifest["target"] != host_target()?
        {
            bail!("Runtime package schema or host target mismatch");
        }
        let member = &manifest["files"]["bin/adl-runtime"];
        if member["mode"].as_u64() != Some(0o755)
            || !member["sha256"].as_str().is_some_and(archive_identity)
        {
            bail!("Runtime executable is absent from the qualified manifest");
        }
        let executable = package.join("bin/adl-runtime");
        if executable.canonicalize()? != executable {
            bail!("Runtime executable path is aliased");
        }
        let metadata = regular_file(&executable)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o7777 != 0o755 {
                bail!("Runtime executable mode mismatch");
            }
        }
        Ok(Self { executable })
    }

    /// Preserve argument boundaries, working directory, environment and standard streams.
    pub(super) fn command(&self, args: &[String]) -> Command {
        let mut command = Command::new(&self.executable);
        command.args(args);
        command
    }

    /// Replace the CLI process on Unix, preserving signals as well as exit status.
    #[cfg(unix)]
    pub(super) fn exec(&self, args: &[String]) -> Result<()> {
        use std::os::unix::process::CommandExt;
        Err(self.command(args).exec()).context("selected Runtime invocation failed")
    }

    pub(super) fn invoke(&self, args: &[String]) -> Result<ExitStatus> {
        self.command(args)
            .status()
            .context("selected Runtime invocation failed")
    }
}
