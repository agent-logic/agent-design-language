use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::model::{Corpus, NormalizedObservation, RawObservation, OBSERVATION_SCHEMA};
use crate::normalize::normalize;
use crate::runner::{binary_sha256, run_case};

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
    if digest != corpus.binary_sha256 {
        bail!(
            "binary digest {digest} does not match corpus pin {}",
            corpus.binary_sha256
        );
    }
    fs::create_dir_all(output)?;
    let root = corpus_path.parent().unwrap_or_else(|| Path::new("."));
    for case in &corpus.cases {
        let case_dir = output.join(&case.id);
        fs::create_dir_all(&case_dir)?;
        for repetition in 1..=corpus.repetitions {
            let raw = run_case(
                binary,
                &digest,
                &corpus.incumbent_revision,
                root,
                case,
                repetition,
            )?;
            write_json(&case_dir.join(format!("{repetition:02}.raw.json")), &raw)?;
            let normalized = normalize(&raw, &case.normalization)?;
            write_json(
                &case_dir.join(format!("{repetition:02}.normalized.json")),
                &normalized,
            )?;
        }
    }
    verify_corpus(corpus, output)
}

pub fn verify_corpus(corpus: &Corpus, observations: &Path) -> Result<VerificationReport> {
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
            {
                bail!("observation identity mismatch at {}", raw_path.display());
            }
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

fn semantic(value: &NormalizedObservation) -> Vec<crate::model::CommandObservation> {
    value.commands.clone()
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
