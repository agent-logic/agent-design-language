use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde_json::Value;

use crate::model::{Corpus, CORPUS_SCHEMA};

pub fn load_corpus(path: &Path) -> Result<Corpus> {
    let bytes = fs::read(path).with_context(|| format!("read corpus {}", path.display()))?;
    let yaml: serde_yaml::Value = serde_yaml::from_slice(&bytes).context("parse corpus YAML")?;
    let json = serde_json::to_value(yaml).context("convert corpus to JSON")?;
    let schema_path = resolve_sibling(path, schema_path(&json)?)?;
    validate_schema(&json, &schema_path)?;
    let corpus: Corpus = serde_json::from_value(json).context("decode typed corpus")?;
    validate_semantics(&corpus)?;
    Ok(corpus)
}

fn schema_path(value: &Value) -> Result<&str> {
    value
        .get("schema_path")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("corpus schema_path is required"))
}

fn resolve_sibling(corpus_path: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        bail!("schema_path must be a clean relative path");
    }
    Ok(corpus_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(path))
}

fn validate_schema(instance: &Value, schema_path: &Path) -> Result<()> {
    let schema: Value = serde_json::from_slice(
        &fs::read(schema_path).with_context(|| format!("read schema {}", schema_path.display()))?,
    )
    .context("parse corpus JSON Schema")?;
    let compiled = jsonschema::JSONSchema::compile(&schema)
        .map_err(|error| anyhow::anyhow!("compile corpus schema: {error}"))?;
    if let Err(errors) = compiled.validate(instance) {
        let messages = errors.map(|error| error.to_string()).collect::<Vec<_>>();
        bail!("corpus schema validation failed: {}", messages.join("; "));
    }
    Ok(())
}

fn validate_semantics(corpus: &Corpus) -> Result<()> {
    if corpus.schema != CORPUS_SCHEMA {
        bail!("unsupported corpus schema {}", corpus.schema);
    }
    if corpus.incumbent_revision.len() != 40
        || !corpus
            .incumbent_revision
            .chars()
            .all(|c| c.is_ascii_hexdigit())
    {
        bail!("incumbent_revision must be a full hexadecimal Git revision");
    }
    if corpus.binary_sha256.len() != 64
        || !corpus.binary_sha256.chars().all(|c| c.is_ascii_hexdigit())
    {
        bail!("binary_sha256 must be a full SHA-256 digest");
    }
    if corpus.repetitions < 3 {
        bail!("corpus requires at least three repetitions");
    }
    unique_nonempty(
        corpus.required_behaviors.iter().map(String::as_str),
        "required behavior",
    )?;
    unique_nonempty(corpus.cases.iter().map(|case| case.id.as_str()), "case id")?;
    let case_ids = corpus
        .cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<BTreeSet<_>>();
    let required = corpus
        .required_behaviors
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for case in &corpus.cases {
        if case.steps.is_empty() || case.behaviors.is_empty() {
            bail!("case {} requires steps and behaviors", case.id);
        }
        unique_nonempty(case.steps.iter().map(|step| step.id.as_str()), "step id")?;
        for behavior in &case.behaviors {
            if !required.contains(behavior.as_str()) {
                bail!("case {} names unknown behavior {}", case.id, behavior);
            }
        }
    }
    let mut coverage = BTreeMap::new();
    for entry in &corpus.coverage {
        if coverage
            .insert(entry.behavior.as_str(), &entry.cases)
            .is_some()
        {
            bail!("duplicate coverage entry for {}", entry.behavior);
        }
        if !required.contains(entry.behavior.as_str()) || entry.cases.is_empty() {
            bail!("invalid coverage entry for {}", entry.behavior);
        }
        for case in &entry.cases {
            if !case_ids.contains(case.as_str()) {
                bail!("coverage {} names unknown case {}", entry.behavior, case);
            }
            let owns = corpus
                .cases
                .iter()
                .find(|candidate| candidate.id == *case)
                .is_some_and(|candidate| candidate.behaviors.contains(&entry.behavior));
            if !owns {
                bail!(
                    "coverage {} is not declared by case {}",
                    entry.behavior,
                    case
                );
            }
        }
    }
    if coverage.keys().copied().collect::<BTreeSet<_>>() != required {
        bail!("coverage map does not exactly cover required behaviors");
    }
    for group in corpus
        .equivalence_groups
        .iter()
        .chain(&corpus.difference_groups)
    {
        if group.cases.len() < 2
            || group
                .cases
                .iter()
                .any(|case| !case_ids.contains(case.as_str()))
        {
            bail!(
                "comparison group {} must name at least two known cases",
                group.id
            );
        }
    }
    Ok(())
}

fn unique_nonempty<'a>(values: impl Iterator<Item = &'a str>, label: &str) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() || !seen.insert(value) {
            bail!("{label}s must be non-empty and unique");
        }
    }
    if seen.is_empty() {
        bail!("at least one {label} is required");
    }
    Ok(())
}
