use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use base64::Engine;
use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};
use wait_timeout::ChildExt;

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
    timeout_ms: u64,
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
        let mut child = Command::new(binary)
            .args(&expanded_args)
            .current_dir(&workdir)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", &workdir)
            .env("TMPDIR", &workdir)
            .env("ADL_OBSERVABILITY", "0")
            .env("NO_PROXY", "*")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("execute case {} step {}", case.id, step.id))?;
        let status = match child.wait_timeout(Duration::from_millis(timeout_ms))? {
            Some(status) => status,
            None => {
                child.kill().context("kill timed-out incumbent child")?;
                child.wait().context("reap timed-out incumbent child")?;
                bail!(
                    "case {} step {} timed out after {} ms",
                    case.id,
                    step.id,
                    timeout_ms
                );
            }
        };
        let mut stdout_bytes = Vec::new();
        let mut stderr_bytes = Vec::new();
        child
            .stdout
            .take()
            .expect("piped stdout")
            .read_to_end(&mut stdout_bytes)?;
        child
            .stderr
            .take()
            .expect("piped stderr")
            .read_to_end(&mut stderr_bytes)?;
        let exit_code = status.code().unwrap_or(-1);
        let stdout_sha256 = format!("{:x}", Sha256::digest(&stdout_bytes));
        let stderr_sha256 = format!("{:x}", Sha256::digest(&stderr_bytes));
        let stdout = portable_text(
            String::from_utf8(stdout_bytes).context("v1 stdout is not UTF-8")?,
            &corpus_root,
            &workdir,
        );
        let stderr = portable_text(
            String::from_utf8(stderr_bytes).context("v1 stderr is not UTF-8")?,
            &corpus_root,
            &workdir,
        );
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
        let portable_args = step
            .args
            .iter()
            .map(|arg| arg.replace("{ROOT}", "<ROOT>").replace("{WORK}", "<WORK>"))
            .collect();
        commands.push(CommandObservation {
            step_id: step.id.clone(),
            declared_args: step.args.clone(),
            expanded_args: portable_args,
            exit_code,
            stdout_sha256,
            stderr_sha256,
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

fn portable_text(value: String, root: &Path, workdir: &Path) -> String {
    value
        .replace(&root.display().to_string(), "<ROOT>")
        .replace(&workdir.display().to_string(), "<WORK>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Case;

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

    #[cfg(unix)]
    #[test]
    fn hung_child_is_killed_and_reported_as_a_bounded_timeout() {
        use std::os::unix::fs::PermissionsExt;

        let root = tempfile::tempdir().unwrap();
        let script = root.path().join("hang.sh");
        fs::write(&script, "#!/bin/sh\nexec sleep 5\n").unwrap();
        let mut permissions = fs::metadata(&script).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&script, permissions).unwrap();
        let case = Case {
            id: "hang".into(),
            behaviors: vec!["timeout".into()],
            steps: vec![crate::model::Step {
                id: "wait".into(),
                args: vec![],
                expected_exit: 0,
                stdout_contains: vec![],
                stderr_contains: vec![],
                pre_actions: vec![],
            }],
            normalization: vec![],
        };
        let error = run_case(
            &script,
            &"a".repeat(64),
            &"b".repeat(40),
            root.path(),
            &case,
            1,
            25,
        )
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "case hang step wait timed out after 25 ms"
        );
    }
}
