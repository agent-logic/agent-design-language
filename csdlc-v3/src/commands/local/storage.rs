//! Storage for the native local owner.

use std::{fs, path::Path};

use serde_json::Value;

use super::filesystem::{atomic_write_json, io_finding};
use super::{
    finding, DoctorFinding, LocalPreparationRequest, PlanStatus, PromptRegistry,
    REQUIRED_CARD_KINDS,
};

pub(super) fn persist_index(
    issue_root: &Path,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    phase: &str,
    generation: u64,
) -> Result<(u64, String), Vec<DoctorFinding>> {
    let mut index = serde_json::json!({
        "schema": "csdlc.v3.local_state.v1",
        "issue": request.issue,
        "phase": phase,
        "generation": generation,
        "repository": request.repository,
        "branch": request.branch,
        "worktree": request.worktree,
        "template_registry_version": registry.version,
        "operational_authority": true
    });
    let digest = lifecycle_digest(issue_root, &index)?;
    index["digest"] = Value::String(digest.clone());
    atomic_write_json(&issue_root.join("index.json"), &index)?;
    Ok((generation, digest))
}
pub(super) fn lifecycle_digest(
    issue_root: &Path,
    index: &Value,
) -> Result<String, Vec<DoctorFinding>> {
    let mut canonical_index = index.clone();
    canonical_index
        .as_object_mut()
        .map(|object| object.remove("digest"));
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serde_json::to_vec(&canonical_index).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "index_serialize_failed",
            &error.to_string(),
        )]
    })?);
    for kind in REQUIRED_CARD_KINDS {
        for suffix in ["values.json", "md"] {
            let path = issue_root.join(format!("cards/{kind}.{suffix}"));
            hasher.update(&fs::read(path).map_err(io_finding("card_read_failed"))?);
        }
    }
    let binding = issue_root.join("binding.json");
    if binding.is_file() {
        hasher.update(&fs::read(binding).map_err(io_finding("binding_read_failed"))?);
    }
    let intent_plan = issue_root.join("intent-plan.json");
    if intent_plan.is_file() {
        hasher.update(b"csdlc.v3.intent_plan.v1\0");
        hasher.update(&fs::read(intent_plan).map_err(io_finding("intent_plan_read_failed"))?);
    }
    Ok(hasher.finalize().to_hex().to_string())
}
pub(super) fn read_index_value(issue_root: &Path) -> Result<Value, Vec<DoctorFinding>> {
    serde_json::from_slice(
        &fs::read(issue_root.join("index.json")).map_err(io_finding("index_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "index_invalid",
            &error.to_string(),
        )]
    })
}
