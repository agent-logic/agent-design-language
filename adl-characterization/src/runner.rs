use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use base64::Engine;
use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};

use crate::model::{Case, CommandObservation, PreAction, RawObservation, OBSERVATION_SCHEMA};

pub fn binary_sha256(binary: &Path) -> Result<String> {
    let bytes = fs::read(binary).with_context(|| format!("read binary {}", binary.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn run_case(
    binary: &Path,
    binary_digest: &str,
    revision: &str,
    corpus_root: &Path,
    case: &Case,
    repetition: u32,
) -> Result<RawObservation> {
    let temp = tempfile::Builder::new()
        .prefix("adl-characterization-")
        .tempdir()?;
    let workdir = temp.path().canonicalize()?;
    let corpus_root = corpus_root.canonicalize()?;
    let mut commands = Vec::new();
    for step in &case.steps {
        for action in &step.pre_actions {
            apply_pre_action(action, &workdir)?;
        }
        let expanded_args = step
            .args
            .iter()
            .map(|arg| expand(arg, &corpus_root, &workdir))
            .collect::<Result<Vec<_>>>()?;
        let output = Command::new(binary)
            .args(&expanded_args)
            .current_dir(&workdir)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", &workdir)
            .env("TMPDIR", &workdir)
            .env("ADL_OBSERVABILITY", "0")
            .env("NO_PROXY", "*")
            .output()
            .with_context(|| format!("execute case {} step {}", case.id, step.id))?;
        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8(output.stdout).context("v1 stdout is not UTF-8")?;
        let stderr = String::from_utf8(output.stderr).context("v1 stderr is not UTF-8")?;
        if exit_code != step.expected_exit {
            bail!(
                "case {} step {} exit {exit_code}, expected {}: {stderr}",
                case.id,
                step.id,
                step.expected_exit
            );
        }
        for expected in &step.stdout_contains {
            if !stdout.contains(expected) {
                bail!(
                    "case {} step {} stdout missing {expected:?}",
                    case.id,
                    step.id
                );
            }
        }
        for expected in &step.stderr_contains {
            if !stderr.contains(expected) {
                bail!(
                    "case {} step {} stderr missing {expected:?}",
                    case.id,
                    step.id
                );
            }
        }
        commands.push(CommandObservation {
            step_id: step.id.clone(),
            declared_args: step.args.clone(),
            expanded_args,
            exit_code,
            stdout,
            stderr,
        });
    }
    Ok(RawObservation {
        schema: OBSERVATION_SCHEMA.into(),
        case_id: case.id.clone(),
        repetition,
        incumbent_revision: revision.into(),
        binary_sha256: binary_digest.into(),
        corpus_root: corpus_root.display().to_string(),
        workdir: workdir.display().to_string(),
        commands,
    })
}

fn apply_pre_action(action: &PreAction, workdir: &Path) -> Result<()> {
    match action {
        PreAction::FixedEd25519Keypair {
            private_path,
            public_path,
            seed_byte,
        } => {
            let private = resolve_work_path(private_path, workdir)?;
            let public = resolve_work_path(public_path, workdir)?;
            ensure_parent(&private)?;
            ensure_parent(&public)?;
            let key = SigningKey::from_bytes(&[*seed_byte; 32]);
            fs::write(
                private,
                base64::engine::general_purpose::STANDARD.encode(key.to_bytes()),
            )?;
            fs::write(
                public,
                base64::engine::general_purpose::STANDARD.encode(key.verifying_key().to_bytes()),
            )?;
        }
        PreAction::ReplaceText { path, from, to } => {
            let path = resolve_work_path(path, workdir)?;
            let current =
                fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
            if !current.contains(from) {
                bail!("replace_text source not found in {}", path.display());
            }
            fs::write(path, current.replacen(from, to, 1))?;
        }
    }
    Ok(())
}

fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn resolve_work_path(value: &str, workdir: &Path) -> Result<PathBuf> {
    let workdir = workdir
        .canonicalize()
        .with_context(|| format!("resolve workdir {}", workdir.display()))?;
    let relative = value
        .strip_prefix("{WORK}/")
        .ok_or_else(|| anyhow::anyhow!("pre-action paths must be rooted under {{WORK}}"))?;
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        bail!("pre-action path must be a clean relative path under {{WORK}}");
    }

    let path = workdir.join(relative);
    let mut existing = path.as_path();
    while fs::symlink_metadata(existing).is_err() {
        existing = existing
            .parent()
            .ok_or_else(|| anyhow::anyhow!("pre-action path has no existing ancestor"))?;
    }
    let canonical = existing
        .canonicalize()
        .with_context(|| format!("resolve pre-action path {}", path.display()))?;
    if !canonical.starts_with(&workdir) {
        bail!("pre-action path escapes {{WORK}}");
    }
    Ok(path)
}

fn expand(value: &str, root: &Path, workdir: &Path) -> Result<String> {
    let expanded = value
        .replace("{ROOT}", &root.display().to_string())
        .replace("{WORK}", &workdir.display().to_string());
    if expanded.contains('{') || expanded.contains('}') {
        bail!("unknown placeholder in argument {value}");
    }
    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_actions_reject_paths_outside_the_workdir() {
        let work = tempfile::tempdir().unwrap();
        assert!(resolve_work_path("/tmp/outside", work.path()).is_err());
        assert!(resolve_work_path("{ROOT}/fixture", work.path()).is_err());
        assert!(resolve_work_path("{WORK}/../outside", work.path()).is_err());
        assert_eq!(
            resolve_work_path("{WORK}/keys/private", work.path()).unwrap(),
            work.path().canonicalize().unwrap().join("keys/private")
        );
    }

    #[cfg(unix)]
    #[test]
    fn pre_actions_reject_symlink_escape() {
        use std::os::unix::fs::symlink;

        let work = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), work.path().join("escape")).unwrap();
        let error = resolve_work_path("{WORK}/escape/file", work.path()).unwrap_err();
        assert!(error.to_string().contains("escapes"));
    }
}
