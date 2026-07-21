use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::manifest::corpus_bundle_sha256;
use crate::model::{Corpus, NormalizedObservation, RawObservation, OBSERVATION_SCHEMA};
use crate::normalize::normalize;
use crate::runner::{binary_sha256, run_case, CaptureIdentity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerificationReport {
    pub schema: String,
    pub incumbent_revision: String,
    pub binary_sha256: String,
    pub case_count: usize,
    pub observation_count: usize,
    pub behavior_count: usize,
    pub equivalence_group_count: usize,
    pub difference_group_count: usize,
    pub status: String,
}

pub fn capture_corpus(
    binary: &Path,
    corpus_path: &Path,
    corpus: &Corpus,
    output: &Path,
) -> Result<VerificationReport> {
    let digest = binary_sha256(binary)?;
    let corpus_bundle_digest = corpus_bundle_sha256(corpus_path)?;
    if digest != corpus.binary_sha256 {
        bail!(
            "binary digest {digest} does not match corpus pin {}",
            corpus.binary_sha256
        );
    }
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let staging = tempfile::Builder::new()
        .prefix("adl-characterization-capture-")
        .tempdir_in(parent)?;
    let staged_output = staging.path();
    let root = corpus_path.parent().unwrap_or_else(|| Path::new("."));
    let identity = CaptureIdentity {
        binary_sha256: &digest,
        incumbent_revision: &corpus.incumbent_revision,
        corpus_bundle_sha256: &corpus_bundle_digest,
    };
    for case in &corpus.cases {
        let case_dir = staged_output.join(&case.id);
        fs::create_dir_all(&case_dir)?;
        for repetition in 1..=corpus.repetitions {
            let raw = run_case(
                binary,
                &identity,
                root,
                case,
                repetition,
                corpus.command_timeout_ms,
            )?;
            write_json(&case_dir.join(format!("{repetition:02}.raw.json")), &raw)?;
            let normalized = normalize(&raw, &case.normalization)?;
            write_json(
                &case_dir.join(format!("{repetition:02}.normalized.json")),
                &normalized,
            )?;
        }
    }
    let report = verify_corpus(corpus_path, corpus, staged_output)?;
    replace_capture(staging, output)?;
    Ok(report)
}

fn replace_capture(staging: tempfile::TempDir, output: &Path) -> Result<()> {
    let staged = staging.keep();
    if !output.exists() {
        fs::rename(&staged, output)
            .with_context(|| format!("install capture at {}", output.display()))?;
        return Ok(());
    }
    let name = output
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("observations");
    let backup = output
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(".{name}.backup-{}", std::process::id()));
    if backup.exists() {
        bail!("capture backup path already exists at {}", backup.display());
    }
    fs::rename(output, &backup).context("move prior capture to rollback path")?;
    if let Err(error) = fs::rename(&staged, output) {
        fs::rename(&backup, output).context("restore prior capture after install failure")?;
        return Err(error).context("install staged capture");
    }
    fs::remove_dir_all(&backup).context("remove replaced capture backup")?;
    Ok(())
}

pub fn verify_corpus(
    corpus_path: &Path,
    corpus: &Corpus,
    observations: &Path,
) -> Result<VerificationReport> {
    let corpus_bundle_digest = corpus_bundle_sha256(corpus_path)?;
    let mut normalized_by_case = BTreeMap::<String, Vec<NormalizedObservation>>::new();
    for case in &corpus.cases {
        let mut values = Vec::new();
        for repetition in 1..=corpus.repetitions {
            let raw_path = observations
                .join(&case.id)
                .join(format!("{repetition:02}.raw.json"));
            let normalized_path = observations
                .join(&case.id)
                .join(format!("{repetition:02}.normalized.json"));
            let raw: RawObservation = read_json(&raw_path)?;
            if raw.schema != OBSERVATION_SCHEMA
                || raw.case_id != case.id
                || raw.repetition != repetition
                || raw.incumbent_revision != corpus.incumbent_revision
                || raw.binary_sha256 != corpus.binary_sha256
                || raw.corpus_bundle_sha256 != corpus_bundle_digest
            {
                bail!("observation identity mismatch at {}", raw_path.display());
            }
            verify_observation_envelope(case, &raw)?;
            verify_command_contract(case, &raw)?;
            let derived = normalize(&raw, &case.normalization)?;
            let retained: NormalizedObservation = read_json(&normalized_path)?;
            if derived != retained {
                bail!(
                    "retained normalized evidence is stale at {}",
                    normalized_path.display()
                );
            }
            values.push(derived);
        }
        let first = semantic(&values[0]);
        if values.iter().skip(1).any(|value| semantic(value) != first) {
            bail!("unexplained repeated-run divergence in case {}", case.id);
        }
        normalized_by_case.insert(case.id.clone(), values);
    }
    for group in &corpus.equivalence_groups {
        let first = semantic(first_case(&normalized_by_case, &group.cases[0])?);
        for case in group.cases.iter().skip(1) {
            if semantic(first_case(&normalized_by_case, case)?) != first {
                bail!("equivalence group {} differs at case {}", group.id, case);
            }
        }
    }
    for group in &corpus.difference_groups {
        let first = semantic(first_case(&normalized_by_case, &group.cases[0])?);
        if group.cases.iter().skip(1).all(|case| {
            semantic(first_case(&normalized_by_case, case).expect("validated case")) == first
        }) {
            bail!("difference group {} has no semantic difference", group.id);
        }
    }
    Ok(VerificationReport {
        schema: "adl.characterization.verification.v1".into(),
        incumbent_revision: corpus.incumbent_revision.clone(),
        binary_sha256: corpus.binary_sha256.clone(),
        case_count: corpus.cases.len(),
        observation_count: corpus.cases.len() * corpus.repetitions as usize,
        behavior_count: corpus.required_behaviors.len(),
        equivalence_group_count: corpus.equivalence_groups.len(),
        difference_group_count: corpus.difference_groups.len(),
        status: "pass".into(),
    })
}

fn verify_observation_envelope(case: &crate::model::Case, raw: &RawObservation) -> Result<()> {
    verify_digest_shape(
        &case.id,
        "observation",
        "evidence envelope",
        &raw.evidence_envelope_sha256,
    )?;
    let derived = raw.compute_evidence_envelope_sha256()?;
    if raw.evidence_envelope_sha256 != derived {
        bail!("case {} evidence envelope digest mismatch", case.id);
    }
    Ok(())
}

fn verify_command_contract(case: &crate::model::Case, raw: &RawObservation) -> Result<()> {
    if raw.commands.len() != case.steps.len() {
        bail!("case {} command count does not match corpus steps", case.id);
    }
    for (step, command) in case.steps.iter().zip(&raw.commands) {
        if command.step_id != step.id {
            bail!(
                "case {} command order does not match step {}",
                case.id,
                step.id
            );
        }
        if command.declared_args != step.args {
            bail!(
                "case {} step {} declared args do not match corpus",
                case.id,
                step.id
            );
        }
        let portable_args = step
            .args
            .iter()
            .map(|arg| arg.replace("{ROOT}", "<ROOT>").replace("{WORK}", "<WORK>"))
            .collect::<Vec<_>>();
        if command.expanded_args != portable_args {
            bail!(
                "case {} step {} expanded args are not portable corpus arguments",
                case.id,
                step.id
            );
        }
        for (label, digest) in [
            ("captured stdout", &command.captured_stdout_sha256),
            ("captured stderr", &command.captured_stderr_sha256),
            ("portable stdout", &command.portable_stdout_sha256),
            ("portable stderr", &command.portable_stderr_sha256),
        ] {
            verify_digest_shape(&case.id, &step.id, label, digest)?;
        }
        let portable_stdout = format!("{:x}", Sha256::digest(command.stdout.as_bytes()));
        let portable_stderr = format!("{:x}", Sha256::digest(command.stderr.as_bytes()));
        if command.portable_stdout_sha256 != portable_stdout
            || command.portable_stderr_sha256 != portable_stderr
        {
            bail!(
                "case {} step {} portable stream digest mismatch",
                case.id,
                step.id
            );
        }
        if command.stdout.contains("/Users/")
            || command.stderr.contains("/Users/")
            || command.stdout.contains("/private/var/")
            || command.stderr.contains("/private/var/")
        {
            bail!(
                "case {} step {} retains a machine-local path",
                case.id,
                step.id
            );
        }
        if command.exit_code != step.expected_exit {
            bail!(
                "case {} step {} exit does not match corpus",
                case.id,
                step.id
            );
        }
        for fragment in &step.stdout_contains {
            if !command.stdout.contains(fragment) {
                bail!(
                    "case {} step {} stdout misses required fragment",
                    case.id,
                    step.id
                );
            }
        }
        for fragment in &step.stderr_contains {
            if !command.stderr.contains(fragment) {
                bail!(
                    "case {} step {} stderr misses required fragment",
                    case.id,
                    step.id
                );
            }
        }
    }
    Ok(())
}

fn verify_digest_shape(case: &str, step: &str, label: &str, digest: &str) -> Result<()> {
    if digest.len() != 64 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        bail!("case {case} step {step} has invalid {label} SHA-256");
    }
    Ok(())
}

fn semantic(value: &NormalizedObservation) -> Vec<(&str, i32, &str, &str)> {
    value
        .commands
        .iter()
        .map(|command| {
            (
                command.step_id.as_str(),
                command.exit_code,
                command.stdout.as_str(),
                command.stderr.as_str(),
            )
        })
        .collect()
}

fn first_case<'a>(
    values: &'a BTreeMap<String, Vec<NormalizedObservation>>,
    case: &str,
) -> Result<&'a NormalizedObservation> {
    values
        .get(case)
        .and_then(|values| values.first())
        .ok_or_else(|| anyhow::anyhow!("missing normalized case {case}"))
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path).with_context(|| format!("read {}", path.display()))?)
        .with_context(|| format!("parse {}", path.display()))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Case, CoverageEntry, Step, CORPUS_SCHEMA};

    #[cfg(unix)]
    #[test]
    fn timed_out_capture_preserves_the_prior_complete_output() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let binary = temp.path().join("hang.sh");
        fs::write(&binary, "#!/bin/sh\nexec sleep 5\n").unwrap();
        let mut permissions = fs::metadata(&binary).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&binary, permissions).unwrap();
        let output = temp.path().join("observations");
        fs::create_dir(&output).unwrap();
        fs::write(output.join("complete.marker"), "prior").unwrap();
        let digest = binary_sha256(&binary).unwrap();
        let corpus = Corpus {
            schema: CORPUS_SCHEMA.into(),
            incumbent_revision: "a".repeat(40),
            binary_sha256: digest,
            repetitions: 3,
            command_timeout_ms: 25,
            schema_path: "schema.json".into(),
            required_behaviors: vec!["timeout".into()],
            cases: vec![Case {
                id: "hang".into(),
                behaviors: vec!["timeout".into()],
                steps: vec![Step {
                    id: "wait".into(),
                    args: vec![],
                    expected_exit: 0,
                    stdout_contains: vec![],
                    stderr_contains: vec![],
                    pre_actions: vec![],
                }],
                normalization: vec![],
            }],
            equivalence_groups: vec![],
            difference_groups: vec![],
            coverage: vec![CoverageEntry {
                behavior: "timeout".into(),
                cases: vec!["hang".into()],
            }],
        };
        let error = capture_corpus(&binary, &temp.path().join("corpus.yaml"), &corpus, &output)
            .unwrap_err();
        assert!(error.to_string().contains("timed out"));
        assert_eq!(
            fs::read_to_string(output.join("complete.marker")).unwrap(),
            "prior"
        );
    }
}
