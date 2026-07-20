use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};

use serde::Deserialize;

use crate::cards::{compiled_headings, CardKind};
use crate::{ErrorCode, Result, V2Error};

const NATIVE_GENERATION: &str = "csdlc_v2_native";
const NATIVE_TEMPLATE_SET: &str = "1.0.0";
const NATIVE_FAMILY: &str = "compact_native";

#[derive(Deserialize)]
struct Registry {
    schema: String,
    csdlc_prompt_template_set: String,
    semver: String,
    status: String,
    object_kind: String,
    lifecycle: Vec<String>,
    generations: BTreeMap<String, GenerationEntry>,
}

#[derive(Deserialize)]
struct GenerationEntry {
    template_set: String,
    projection_family: String,
    #[serde(default)]
    shape_manifest_path: Option<String>,
    #[serde(default)]
    path: Option<String>,
}

#[derive(Deserialize)]
struct ShapeManifest {
    schema: String,
    generation: String,
    template_set: String,
    projection_family: String,
    cards: BTreeMap<String, Vec<String>>,
}

pub fn validate_native_registry(root: &Path) -> Result<()> {
    let registry: Registry = read_json(&root.join("docs/templates/prompts/current.json"))?;
    if registry.schema != "adl.csdlc.prompt_template_registry.v1"
        || registry.csdlc_prompt_template_set != "1.0.3"
        || registry.semver != "1.0.3"
        || registry.status != "active"
        || registry.object_kind != "csdlc_prompt_template_set"
        || registry.lifecycle != ["SIP", "STP", "SPP", "VPP", "SRP", "SOR"]
    {
        return invalid("prompt registry top-level authority is incompatible");
    }
    let legacy = registry.generations.get("legacy_import").ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            "legacy import registry entry is missing",
        )
    })?;
    if legacy.template_set != "1.0.3"
        || legacy.projection_family != "legacy_full"
        || legacy.path.as_deref() != Some("docs/templates/prompts/1.0.3")
    {
        return invalid("legacy import registry identity is incompatible");
    }
    let entry = registry.generations.get(NATIVE_GENERATION).ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            "native v2 registry entry is missing",
        )
    })?;
    if entry.template_set != NATIVE_TEMPLATE_SET || entry.projection_family != NATIVE_FAMILY {
        return invalid("native v2 registry identity is incompatible");
    }
    let relative = Path::new(entry.shape_manifest_path.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            "native v2 shape manifest path is missing",
        )
    })?);
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return invalid("native v2 shape manifest path is not repository-relative");
    }
    let manifest: ShapeManifest = read_json(&root.join(relative))?;
    if manifest.schema != "csdlc.native_card_shape.v1"
        || manifest.generation != NATIVE_GENERATION
        || manifest.template_set != NATIVE_TEMPLATE_SET
        || manifest.projection_family != NATIVE_FAMILY
        || manifest.cards.len() != CARD_KINDS.len()
    {
        return invalid("native v2 shape manifest identity is incompatible");
    }
    for kind in CARD_KINDS {
        let key = kind.to_string();
        let declared = manifest.cards.get(&key).ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidManifest,
                format!("native v2 shape manifest is missing {key}"),
            )
        })?;
        if declared.iter().map(String::as_str).collect::<Vec<_>>() != compiled_headings(kind) {
            return invalid(&format!(
                "native v2 {key} shape does not match compiled contract"
            ));
        }
    }
    Ok(())
}

const CARD_KINDS: [CardKind; 6] = [
    CardKind::Sip,
    CardKind::Stp,
    CardKind::Spp,
    CardKind::Vpp,
    CardKind::Srp,
    CardKind::Sor,
];

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let bytes = fs::read(path).map_err(|error| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            format!("cannot read native v2 registry contract: {error}"),
        )
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            format!("native v2 registry contract is malformed: {error}"),
        )
    })
}

fn invalid<T>(message: &str) -> Result<T> {
    Err(V2Error::new(ErrorCode::InvalidManifest, message))
}
