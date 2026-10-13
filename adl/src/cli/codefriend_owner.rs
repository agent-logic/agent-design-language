//! Lightweight selection after CodeFriend's product_distribution.py qualification.
//! The installer authenticates external producer pins and all resources once. The
//! selected installation must remain trusted; this is not a second verifier.
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) struct SelectedCodeFriend {
    executable: PathBuf,
}

fn hex_identity(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

fn metadata(path: &Path) -> Result<serde_json::Value> {
    let info = std::fs::symlink_metadata(path)?;
    if !info.is_file() || info.len() > 8 * 1024 * 1024 {
        bail!("CodeFriend installation metadata must be a bounded regular file");
    }
    serde_json::from_slice(&std::fs::read(path)?)
        .context("invalid CodeFriend installation metadata")
}

impl SelectedCodeFriend {
    pub(super) fn from_environment() -> Result<Self> {
        let prefix = std::env::var_os("ADL_CODEFRIEND_INSTALL_PREFIX")
            .filter(|value| !value.is_empty())
            .context("set ADL_CODEFRIEND_INSTALL_PREFIX to a qualified CodeFriend installation")?;
        let archive = std::env::var("ADL_CODEFRIEND_ARCHIVE_SHA256")
            .context("set ADL_CODEFRIEND_ARCHIVE_SHA256 to its authenticated archive identity")?;
        let candidate = std::env::var("ADL_CODEFRIEND_CANDIDATE")
            .context("set ADL_CODEFRIEND_CANDIDATE to its authenticated source revision")?;
        Self::from_installation(Path::new(&prefix), &archive, &candidate)
    }

    pub(super) fn from_installation(prefix: &Path, archive: &str, candidate: &str) -> Result<Self> {
        if !hex_identity(archive, 64) || !hex_identity(candidate, 40) {
            bail!("CodeFriend requires exact lowercase archive and candidate identities");
        }
        if !prefix.is_absolute() || prefix.canonicalize()? != prefix {
            bail!("CodeFriend installation must be absolute and unaliased");
        }
        let receipt = metadata(&prefix.join("INSTALLATION.json"))?;
        let manifest = metadata(&prefix.join("manifest.json"))?;
        if receipt["schema"] != "codefriend.product_installation.v1"
            || receipt["authentication"] != "external_owner_pins_not_self_authorization"
            || receipt["archive_sha256"] != archive
            || receipt["candidate"] != candidate
            || manifest["schema"] != "codefriend.product_distribution.v1"
            || manifest["source_revision"] != candidate
            || manifest["source_clean"] != true
        {
            bail!("CodeFriend installation does not match the selected owner identity");
        }
        let context = &manifest["builder_context"];
        if context["GITHUB_REPOSITORY"] != "agent-logic/codefriend" {
            bail!("CodeFriend producer repository mismatch");
        }
        for (receipt_key, context_key) in [
            ("run_id", "GITHUB_RUN_ID"),
            ("run_attempt", "GITHUB_RUN_ATTEMPT"),
            ("workflow_ref", "GITHUB_WORKFLOW_REF"),
            ("workflow_sha", "GITHUB_WORKFLOW_SHA"),
            ("job_name", "GITHUB_JOB"),
        ] {
            if !receipt[receipt_key]
                .as_str()
                .is_some_and(|value| !value.is_empty())
                || receipt[receipt_key] != context[context_key]
            {
                bail!("CodeFriend installed producer metadata mismatch");
            }
        }
        let resources = manifest["resources"]
            .as_array()
            .context("CodeFriend resource list missing")?;
        let executables: Vec<_> = resources
            .iter()
            .filter(|row| row["path"] == "bin/codefriend")
            .collect();
        if executables.len() != 1
            || executables[0]["mode"].as_u64() != Some(0o755)
            || !executables[0]["sha256"]
                .as_str()
                .is_some_and(|value| hex_identity(value, 64))
        {
            bail!("CodeFriend executable is absent from the qualified manifest");
        }
        let executable = prefix.join("bin/codefriend");
        let info = std::fs::symlink_metadata(&executable)?;
        if !info.is_file() || executable.canonicalize()? != executable {
            bail!("CodeFriend executable must be a regular unaliased file");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if info.permissions().mode() & 0o7777 != 0o755 {
                bail!("CodeFriend executable mode mismatch");
            }
        }
        Ok(Self { executable })
    }

    pub(super) fn command(&self, args: &[String]) -> Command {
        let mut command = Command::new(&self.executable);
        command.args(args);
        command
    }

    #[cfg(unix)]
    pub(super) fn exec(&self, args: &[String]) -> Result<()> {
        use std::os::unix::process::CommandExt;
        Err(self.command(args).exec()).context("selected CodeFriend invocation failed")
    }

    #[cfg(not(unix))]
    pub(super) fn exec(&self, _args: &[String]) -> Result<()> {
        bail!("CodeFriend package execution is supported only on Unix hosts")
    }
}
