//! Public JSON artifact checks only; not native lifecycle validation authority.
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use std::{fs, path::Path};

fn json(path: &Path) -> Result<Value> {
    serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?,
    )
    .with_context(|| format!("parse {}", path.display()))
}

pub fn check(root: &Path, version: Option<&str>, active_registry: bool) -> Result<String> {
    let registry = json(&root.join("docs/templates/prompts/current.json"))?;
    let set = version
        .or_else(|| registry["csdlc_prompt_template_set"].as_str())
        .context("registry must declare csdlc_prompt_template_set")?;
    let parts: Vec<_> = set.split('.').collect();
    ensure!(
        parts.len() == 3
            && parts
                .iter()
                .all(|s| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())),
        "invalid template set semver"
    );
    let numbers = parts
        .iter()
        .map(|p| p.parse::<u64>())
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let kinds: &[&str] = if (numbers[0], numbers[1], numbers[2]) >= (1, 0, 3) {
        &["sip", "stp", "spp", "vpp", "srp", "sor"]
    } else {
        &["sip", "stp", "spp", "srp", "sor"]
    };
    if active_registry {
        ensure!(
            version.is_none(),
            "active registry check cannot select a historical version"
        );
        ensure!(registry["status"] == "active", "registry must be active");
        ensure!(
            registry["semver"].as_str() == Some(set),
            "registry semver mismatch"
        );
        let entries = registry["templates"]
            .as_object()
            .context("registry templates must be an object")?;
        ensure!(
            entries.len() == 6
                && ["sip", "stp", "spp", "vpp", "srp", "sor"]
                    .iter()
                    .all(|k| entries.contains_key(*k)),
            "active registry must declare exactly six lifecycle cards"
        );
        for entry in entries.values() {
            for field in ["path", "structure_schema_path"] {
                let path = entry[field]
                    .as_str()
                    .context("registry path must be a string")?;
                ensure!(root.join(path).is_file(), "registry file missing: {path}");
            }
        }
    }
    if version.is_none() {
        ensure!(
            registry["templates"].is_object(),
            "registry templates must be an object"
        );
    }
    for kind in kinds {
        let (schema_path, template_path) = if version.is_some() {
            (
                format!("docs/templates/prompts/{set}/schemas/{kind}.structure.json"),
                format!("docs/templates/prompts/{set}/{kind}.md"),
            )
        } else {
            let entry = &registry["templates"][kind];
            let schema = entry["structure_schema_path"]
                .as_str()
                .context("structure_schema_path must be a string")?;
            ensure!(
                schema.ends_with(".json"),
                "structure_schema_path must point at a JSON artifact"
            );
            let template = entry["path"]
                .as_str()
                .filter(|s| !s.is_empty())
                .context("template path must be present")?;
            (schema.to_owned(), template.to_owned())
        };
        let schema = json(&root.join(schema_path))?;
        for (field, expected) in [
            ("schema", "adl.csdlc.prompt_card_structure.v1"),
            ("template_set", set),
            ("card_kind", *kind),
            ("template_path", template_path.as_str()),
        ] {
            ensure!(
                schema[field].as_str() == Some(expected),
                "{kind} {field} mismatch"
            );
        }
        ensure!(
            schema["parser"]
                .as_str()
                .is_some_and(|s| s.contains("markdown-rs")),
            "{kind} parser must record markdown-rs extraction"
        );
        for field in [
            "editable_sections",
            "frontmatter_keys",
            "headings",
            "fenced_blocks",
            "locked_lines",
        ] {
            ensure!(schema[field].is_array(), "{kind} {field} must be a list");
        }
        ensure!(
            !schema["headings"].as_array().unwrap().is_empty(),
            "{kind} schema must record Markdown headings"
        );
    }
    Ok(set.to_owned())
}

pub fn cli(args: &[String]) -> Result<()> {
    let (version, active) = match args {
        [] => (None, false),
        [flag] if flag == "--active-registry" => (None, true),
        [flag, version] if flag == "--template-set" => (Some(version.as_str()), false),
        _ => anyhow::bail!("usage: prompt-schemas [--template-set <semver> | --active-registry]"),
    };
    let set = check(&crate::root(), version, active)?;
    println!("PASS: prompt-card structure JSON artifacts validated for {set}");
    Ok(())
}
