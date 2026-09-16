//! Storage for the native local owner.

use std::{
    fs,
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::Value;

use super::{
    finding, DoctorFinding, LocalPreparationRequest, OperationalLocalResult, PlanStatus,
    PromptRegistry, REQUIRED_CARD_KINDS,
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
pub(super) fn atomic_write_json(path: &Path, value: &Value) -> Result<(), Vec<DoctorFinding>> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "json_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write(path, &bytes)
}
pub(super) fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), Vec<DoctorFinding>> {
    let parent = path.parent().ok_or_else(|| {
        vec![finding(
            PlanStatus::Failed,
            "write_parent_missing",
            "target has no parent",
        )]
    })?;
    fs::create_dir_all(parent).map_err(io_finding("write_parent_create_failed"))?;
    let temporary = parent.join(format!(
        ".{}.tmp-{}-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("state"),
        std::process::id(),
        next_local_temp_sequence()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(io_finding("temporary_create_failed"))?;
    file.write_all(bytes)
        .map_err(io_finding("temporary_write_failed"))?;
    file.sync_all()
        .map_err(io_finding("temporary_sync_failed"))?;
    fs::rename(&temporary, path).map_err(io_finding("atomic_rename_failed"))
}
pub(super) fn next_local_temp_sequence() -> u64 {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    SEQUENCE.fetch_add(1, Ordering::Relaxed)
}
pub(super) fn io_finding(code: &'static str) -> impl FnOnce(std::io::Error) -> Vec<DoctorFinding> {
    move |error| vec![finding(PlanStatus::Failed, code, &error.to_string())]
}

pub(super) fn append_directory_rollback_finding(path: &Path, findings: &mut Vec<DoctorFinding>) {
    if let Err(error) = fs::remove_dir_all(path) {
        findings.push(finding(
            PlanStatus::Failed,
            "issue_state_rollback_failed",
            &error.to_string(),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn operational_result(
    route: &str,
    issue: u64,
    mutated: bool,
    phase: &str,
    generation: u64,
    digest: String,
    next_route: Option<&str>,
    findings: Vec<DoctorFinding>,
) -> OperationalLocalResult {
    OperationalLocalResult {
        route: route.into(),
        issue,
        mutated,
        phase: Some(phase.into()),
        generation: Some(generation),
        digest: Some(digest),
        next_route: next_route.map(str::to_string),
        routing: None,
        findings,
    }
}

pub(super) fn local_transaction_failpoint(name: &str) {
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref() == Ok(name) {
        std::process::exit(91);
    }
}
