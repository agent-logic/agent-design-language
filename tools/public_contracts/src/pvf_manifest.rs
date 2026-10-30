//! Retained v0.91.4 manifest contract. Never executes lane commands.
use anyhow::{ensure, Context, Result};
use serde_json::{Map, Value};
use std::collections::BTreeSet;
pub const CLASSES: &[&str] = &[
    "fast_unit",
    "contract_schema_card",
    "docs",
    "cli_workflow",
    "integration_worktree",
    "provider_live",
    "release_gate",
];
const ENUMS: &[(&str, &[&str])] = &[
    ("lane_class", CLASSES),
    ("resource_profile", &["low", "medium", "high"]),
    ("determinism", &["strict", "fixture_bound", "live"]),
    ("cache_strategy", &["none", "local_reuse", "artifact_reuse"]),
    (
        "release_gate_class",
        &[
            "required_on_pr",
            "release_candidate",
            "manual_release_gate",
            "optional",
        ],
    ),
    (
        "default_trigger",
        &["always", "changed_paths", "manual", "release_only"],
    ),
];
pub const REQUIRED: &[&str] = &[
    "lane_class",
    "owner_surface",
    "command",
    "resource_profile",
    "determinism",
    "cache_strategy",
    "release_gate_class",
    "default_trigger",
    "changed_path_hints",
    "evidence_outputs",
];
fn object<'a>(value: &'a Value, label: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .with_context(|| format!("{label} must be an object"))
}
fn string<'a>(value: &'a Value, label: &str) -> Result<&'a str> {
    value
        .as_str()
        .filter(|s| !s.is_empty())
        .with_context(|| format!("{label} must be a non-empty string"))
}
fn array<'a>(value: &'a Value, label: &str) -> Result<Vec<&'a str>> {
    value
        .as_array()
        .with_context(|| format!("{label} must be an array"))?
        .iter()
        .map(|v| string(v, label))
        .collect()
}
pub fn validate(manifest: &Value) -> Result<()> {
    let top = object(manifest, "manifest")?;
    ensure!(
        top.keys()
            .all(|k| ["manifest_version", "lane_classes", "lanes"].contains(&k.as_str())),
        "unknown top-level key"
    );
    ensure!(
        string(&manifest["manifest_version"], "manifest_version")? == "v0.91.4",
        "manifest_version must be v0.91.4"
    );
    let classes = array(&manifest["lane_classes"], "lane_classes")?;
    let declared: BTreeSet<_> = classes.iter().copied().collect();
    ensure!(
        !classes.is_empty() && classes.len() == declared.len(),
        "lane_classes must be nonempty and unique"
    );
    ensure!(
        declared.iter().all(|c| CLASSES.contains(c)),
        "unsupported lane class"
    );
    let lanes = object(&manifest["lanes"], "lanes")?;
    ensure!(!lanes.is_empty(), "lanes must not be empty");
    let mut used = BTreeSet::new();
    for (id, lane) in lanes {
        ensure!(
            !id.is_empty()
                && id
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
            "lane id must match ^[a-z0-9_]+$"
        );
        let fields = object(lane, id)?;
        ensure!(
            fields.keys().all(|k| REQUIRED.contains(&k.as_str())
                || [
                    "timeout_minutes",
                    "requires_credentials",
                    "requires_worktree",
                    "notes"
                ]
                .contains(&k.as_str())),
            "unknown lane key in {id}"
        );
        for key in REQUIRED {
            ensure!(
                fields.contains_key(*key),
                "lane {id} missing required key: {key}"
            );
        }
        for (key, values) in ENUMS {
            ensure!(
                values.contains(&string(&lane[key], key)?),
                "lane {id}: invalid {key}"
            );
        }
        for key in ["owner_surface", "command"] {
            string(&lane[key], key)?;
        }
        for key in ["changed_path_hints", "evidence_outputs"] {
            array(&lane[key], key)?;
        }
        if let Some(value) = fields.get("timeout_minutes") {
            ensure!(
                value.as_u64().is_some_and(|v| v >= 1),
                "timeout_minutes must be a positive integer"
            );
        }
        for key in ["requires_credentials", "requires_worktree"] {
            if let Some(value) = fields.get(key) {
                ensure!(value.is_boolean(), "{key} must be Boolean");
            }
        }
        if let Some(value) = fields.get("notes") {
            string(value, "notes")?;
        }
        used.insert(string(&lane["lane_class"], "lane_class")?);
    }
    ensure!(
        declared == used,
        "lane_classes must equal the subset actually used by lanes"
    );
    Ok(())
}
