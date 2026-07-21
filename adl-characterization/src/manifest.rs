use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::model::{Corpus, CORPUS_SCHEMA};

pub fn load_corpus(path: &Path) -> Result<Corpus> {
    let bytes = fs::read(path).with_context(|| format!("read corpus {}", path.display()))?;
    let yaml: serde_yaml::Value = serde_yaml::from_slice(&bytes).context("parse corpus YAML")?;
    let json = serde_json::to_value(yaml).context("convert corpus to JSON")?;
    let schema_path = resolve_sibling(path, schema_path(&json)?)?;
    validate_schema(&json, &schema_path)?;
    let corpus: Corpus = serde_json::from_value(json).context("decode typed corpus")?;
    validate_semantics(&corpus, path.parent().unwrap_or_else(|| Path::new(".")))?;
    Ok(corpus)
}

pub fn corpus_bundle_sha256(corpus_path: &Path) -> Result<String> {
    let root = corpus_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()
        .with_context(|| format!("resolve corpus root for {}", corpus_path.display()))?;
    let mut files = Vec::new();
    for entry in WalkDir::new(&root).follow_links(false) {
        let entry = entry.with_context(|| format!("walk corpus root {}", root.display()))?;
        if entry.file_type().is_symlink() {
            bail!("corpus bundle rejects symlink {}", entry.path().display());
        }
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(&root)
                .expect("walk entry is under corpus root")
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("corpus path is not UTF-8"))?
                .replace(std::path::MAIN_SEPARATOR, "/");
            files.push((relative, entry.path().to_path_buf()));
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    let mut digest = Sha256::new();
    digest.update(b"adl.characterization.corpus-bundle.v1\0");
    for (relative, path) in files {
        let bytes =
            fs::read(&path).with_context(|| format!("read corpus file {}", path.display()))?;
        digest.update(relative.as_bytes());
        digest.update([0]);
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
    }
    Ok(format!("{:x}", digest.finalize()))
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

fn validate_semantics(corpus: &Corpus, corpus_root: &Path) -> Result<()> {
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
    if corpus.command_timeout_ms == 0 || corpus.command_timeout_ms > 300_000 {
        bail!("command_timeout_ms must be between 1 and 300000");
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
    if !corpus.equivalence_groups.is_empty() || !corpus.difference_groups.is_empty() {
        unique_nonempty(
            corpus
                .equivalence_groups
                .iter()
                .chain(&corpus.difference_groups)
                .map(|group| group.id.as_str()),
            "comparison group id",
        )?;
    }
    validate_execution_policy(corpus, corpus_root)?;
    Ok(())
}

fn validate_execution_policy(corpus: &Corpus, corpus_root: &Path) -> Result<()> {
    for case in &corpus.cases {
        for step in &case.steps {
            let args = step.args.iter().map(String::as_str).collect::<Vec<_>>();
            let allowed = match args.as_slice() {
                ["--help"] | ["--version"] | ["--definitely-invalid"] => true,
                [fixture, "--print-plan"] | [fixture, "--print-prompts"] => {
                    is_clean_fixture(fixture)
                }
                ["instrument", "graph", fixture, "--format", "json"] => is_clean_fixture(fixture),
                ["sign", fixture, "--key", "{WORK}/keys/private.b64", "--key-id", "characterization-fixed", "--out", "{WORK}/signed.adl.yaml"] => {
                    is_clean_fixture(fixture)
                }
                ["verify", "{WORK}/signed.adl.yaml", "--key", "{WORK}/keys/public.b64"] => true,
                [fixture, "--run", "--allow-unsigned", "--out", "{WORK}/run", "--quiet"]
                    if is_clean_fixture(fixture) =>
                {
                    validate_local_mock_run(case, step, corpus_root)?;
                    true
                }
                _ => false,
            };
            if !allowed {
                bail!(
                    "case {} step {} is outside the local-only command policy",
                    case.id,
                    step.id
                );
            }
        }
    }
    Ok(())
}

fn is_clean_fixture(value: &str) -> bool {
    let Some(relative) = value.strip_prefix("{ROOT}/fixtures/") else {
        return false;
    };
    !relative.is_empty()
        && relative.ends_with(".adl.yaml")
        && Path::new(relative)
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn validate_local_mock_run(
    case: &crate::model::Case,
    step: &crate::model::Step,
    corpus_root: &Path,
) -> Result<()> {
    if !case
        .behaviors
        .iter()
        .any(|behavior| behavior == "local-mock-run")
    {
        bail!(
            "case {} executes --run without local-mock-run behavior",
            case.id
        );
    }
    let fixture = step
        .args
        .first()
        .and_then(|arg| arg.strip_prefix("{ROOT}/"))
        .ok_or_else(|| anyhow::anyhow!("local mock run must use a corpus-root fixture"))?;
    let fixture = resolve_sibling(&corpus_root.join("corpus.yaml"), fixture)?;
    let yaml: serde_yaml::Value = serde_yaml::from_slice(
        &fs::read(&fixture)
            .with_context(|| format!("read local mock fixture {}", fixture.display()))?,
    )?;
    let json = serde_json::to_value(yaml)?;
    let providers = json
        .get("providers")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow::anyhow!("local mock fixture requires providers"))?;
    if providers.is_empty()
        || providers.values().any(|provider| {
            provider.as_object().is_none_or(|object| object.len() != 1)
                || !provider
                    .get("profile")
                    .and_then(Value::as_str)
                    .is_some_and(|profile| profile.starts_with("mock:"))
        })
        || json.pointer("/run/remote").is_some()
    {
        bail!(
            "case {} --run fixture is not exclusively local_mock",
            case.id
        );
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
