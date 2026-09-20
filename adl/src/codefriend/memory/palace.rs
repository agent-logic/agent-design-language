//! Bounded references in the production Runtime Memory Palace, with live admission on every read.
use super::{
    baseline::{self, AdmittedBaselines, BaselineAccess, BaselineRef},
    comparison::{self, DeltaReport},
};
use crate::codefriend::evidence::valid_digest;
use adl_runtime::memory_palace::RuntimeMemoryPalaceService;
use adl_runtime_kernel::{
    MemoryPalaceInput, MemoryReference, MemoryTemporalAnchor, MemoryVisibility,
    ObsMemContextRecord, VerifiedMemoryPalaceAuthority,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};
pub const VERSION: &str = "codefriend.palace.v1";
const WORKFLOW: &str = "codefriend-review";
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexRequest {
    pub schema: String,
    pub references: Vec<BaselineRef>,
    pub observed_epoch_ms: u64,
    pub stale_after_ms: u64,
    pub max_working_set_items: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetrieveRequest {
    pub schema: String,
    pub baseline: BaselineRef,
    pub current: BaselineRef,
    pub expected_identity_root: String,
    pub expected_continuity_head: String,
    pub packet_observed_epoch_ms: u64,
    pub observed_epoch_ms: u64,
    pub stale_after_ms: u64,
    pub max_working_set_items: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PalaceReceipt {
    pub schema: String,
    pub backend: String,
    pub generation: u64,
    pub packet_sha256: String,
    pub checkpoint_sha256: String,
    pub continuity_head: String,
    pub identity_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RetrievedComparison {
    pub schema: String,
    pub provenance: PalaceReceipt,
    pub observed_epoch_ms: u64,
    pub selected_references: Vec<BaselineRef>,
    pub delta: DeltaReport,
}
fn bounds(schema: &str, max: usize, stale: u64) -> Result<()> {
    ensure!(schema == VERSION, "palace_schema_mismatch");
    ensure!(
        (1..=64).contains(&max) && stale > 0,
        "palace_invalid_bounds"
    );
    Ok(())
}
fn sha256<T: Serialize>(value: &T) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(serde_json::to_vec(value)?)))
}
fn reference_citation(r: &BaselineRef) -> Result<MemoryReference> {
    Ok(MemoryReference {
        id: format!("baseline:{}", r.record_digest),
        path: format!("baselines/{}.json", r.record_digest),
        sha256: sha256(r)?,
    })
}
fn receipt(c: &adl_runtime::memory_palace::RuntimeMemoryPalaceCommit) -> PalaceReceipt {
    PalaceReceipt {
        schema: VERSION.into(),
        backend: "RuntimeMemoryPalaceService".into(),
        generation: c.checkpoint.memory_palace_generation,
        packet_sha256: c.packet.packet_sha256.clone(),
        checkpoint_sha256: c.checkpoint.checkpoint_sha256.clone(),
        continuity_head: c.packet.continuity_head.clone(),
        identity_root: c.packet.identity_root.clone(),
    }
}
/// Only shared digest references enter durable memory; source and finding text stay in admission.
pub fn index(
    backend: &AdmittedBaselines<'_>,
    root: &Path,
    authority: &VerifiedMemoryPalaceAuthority,
    request: &IndexRequest,
) -> Result<PalaceReceipt> {
    index_with_baselines(backend, root, authority, request)
}

/// Supports authenticated routing to multiple original admission owners while
/// preserving the same Runtime authority, bounds and reference validation.
pub fn index_with_baselines(
    backend: &impl BaselineAccess,
    root: &Path,
    authority: &VerifiedMemoryPalaceAuthority,
    request: &IndexRequest,
) -> Result<PalaceReceipt> {
    bounds(
        &request.schema,
        request.max_working_set_items,
        request.stale_after_ms,
    )?;
    ensure!(
        !request.references.is_empty() && request.references.len() <= 64,
        "palace_reference_limit"
    );
    baseline::safe_path(root)?;
    let mut refs = request.references.clone();
    refs.sort_by(|a, b| (&a.run_id, &a.record_digest).cmp(&(&b.run_id, &b.record_digest)));
    let mut seen = BTreeSet::new();
    for r in &refs {
        r.validate()?;
        backend.load(r)?;
        ensure!(seen.insert(r.run_id.clone()), "palace_duplicate_run");
    }
    let trace_digest = sha256(&(VERSION, &refs, request.observed_epoch_ms))?;
    let trace = MemoryReference {
        id: format!("trace:{trace_digest}"),
        path: format!(".adl/runtime-v3/observability/codefriend/{trace_digest}.json"),
        sha256: trace_digest,
    };
    let continuity = authority.continuity().record();
    let records = refs
        .iter()
        .map(|r| {
            Ok(ObsMemContextRecord {
                id: format!("baseline:{}", r.record_digest),
                run_id: r.run_id.clone(),
                workflow_id: WORKFLOW.into(),
                payload: serde_json::to_string(r)?,
                visibility: MemoryVisibility::Public,
                identity_root: authority.identity().identity_root.clone(),
                continuity_head: continuity.continuity_head.clone(),
                trace_id: trace.id.clone(),
                citations: vec![trace.clone(), reference_citation(r)?],
                temporal_anchor: MemoryTemporalAnchor {
                    created_epoch_ms: request.observed_epoch_ms,
                    observed_epoch_ms: request.observed_epoch_ms,
                    effective_epoch_ms: request.observed_epoch_ms,
                    continuity_head: continuity.continuity_head.clone(),
                    event_sequence: 1,
                },
            })
        })
        .collect::<Result<Vec<_>>>()?;
    // The citation names real bounded durable bytes, not a synthetic existence claim.
    let trace_path = root.join(&trace.path);
    baseline::safe_path(&trace_path)?;
    std::fs::create_dir_all(trace_path.parent().unwrap())?;
    let trace_bytes = serde_json::to_vec(&(VERSION, &refs, request.observed_epoch_ms))?;
    if trace_path.exists() {
        ensure!(
            read_trace(&trace_path)? == trace_bytes,
            "palace_trace_collision"
        );
    } else {
        baseline::write_json(&trace_path, &(VERSION, &refs, request.observed_epoch_ms))?;
    }
    let input = MemoryPalaceInput {
        schema: adl_runtime_kernel::MEMORY_PALACE_INPUT_SCHEMA.into(),
        identity_record_sha256: authority.identity().record_sha256.clone(),
        continuity_record_sha256: continuity.record_sha256.clone(),
        trace_reference: trace,
        redaction_policy_sha256: sha256(&(
            VERSION,
            "digest-references-only;live-admission-required",
        ))?,
        observed_epoch_ms: request.observed_epoch_ms,
        stale_after_ms: request.stale_after_ms,
        max_working_set_items: request.max_working_set_items,
        records,
    };
    let commit = RuntimeMemoryPalaceService::new(root)
        .commit(authority, &input)
        .map_err(|_| anyhow::anyhow!("palace_commit_denied"))?;
    Ok(receipt(&commit))
}
fn read_trace(path: &Path) -> Result<Vec<u8>> {
    use std::io::Read;
    baseline::safe_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)
            .map_err(|_| anyhow::anyhow!("palace_trace_unavailable"))?
            .is_file(),
        "palace_trace_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .map_err(|_| anyhow::anyhow!("palace_trace_unavailable"))?
        .take(65537)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow::anyhow!("palace_trace_unavailable"))?;
    ensure!(bytes.len() <= 65536, "palace_trace_bounds");
    Ok(bytes)
}
/// Every selected reference is re-admitted before comparison; a saved packet cannot resurrect data.
pub fn retrieve(
    backend: &AdmittedBaselines<'_>,
    root: &Path,
    request: &RetrieveRequest,
) -> Result<RetrievedComparison> {
    retrieve_with_baselines(backend, root, request)
}

pub fn retrieve_with_baselines(
    backend: &impl BaselineAccess,
    root: &Path,
    request: &RetrieveRequest,
) -> Result<RetrievedComparison> {
    bounds(
        &request.schema,
        request.max_working_set_items,
        request.stale_after_ms,
    )?;
    request.baseline.validate()?;
    request.current.validate()?;
    ensure!(
        valid_digest(&request.expected_continuity_head)
            && !request.expected_identity_root.is_empty(),
        "palace_invalid_identity"
    );
    ensure!(
        request.observed_epoch_ms >= request.packet_observed_epoch_ms
            && request.observed_epoch_ms - request.packet_observed_epoch_ms
                <= request.stale_after_ms,
        "palace_stale_observation"
    );
    baseline::safe_path(root)?;
    let config = serde_json::json!({"memory_palace":{"input_ref":"latest.json","max_working_set_items":request.max_working_set_items,"stale_after_ms":request.stale_after_ms,"required_continuity_id":request.expected_continuity_head,"observed_epoch_ms":request.packet_observed_epoch_ms}});
    let (context, checkpoint) = crate::memory_palace::build_context_from_agent_memory_strict(
        &config,
        root,
        &request.current.run_id,
        u128::from(request.packet_observed_epoch_ms),
    )?;
    let trace_bytes = read_trace(&root.join(&checkpoint.trace_reference.path))?;
    ensure!(
        format!("{:x}", Sha256::digest(&trace_bytes)) == checkpoint.trace_reference.sha256,
        "palace_trace_digest_mismatch"
    );
    ensure!(
        checkpoint.identity_root == request.expected_identity_root
            && checkpoint.continuity_head == request.expected_continuity_head,
        "palace_identity_mismatch"
    );
    ensure!(
        context.working_set.selected.len() <= 64,
        "palace_working_set_limit"
    );
    let mut selected = Vec::new();
    let mut seen = BTreeSet::new();
    for item in &context.working_set.selected {
        ensure!(item.payload.len() <= 1024, "palace_payload_limit");
        let r: BaselineRef = serde_json::from_str(&item.payload)
            .map_err(|_| anyhow::anyhow!("palace_reference_invalid"))?;
        r.validate()?;
        let anchor = context
            .topology
            .anchors
            .iter()
            .find(|a| a.record_id == item.record_id)
            .ok_or_else(|| anyhow::anyhow!("palace_anchor_missing"))?;
        let citation = reference_citation(&r)?;
        ensure!(
            item.record_id == format!("baseline:{}", r.record_digest)
                && anchor.run_id == r.run_id
                && anchor.continuity_id.as_deref()
                    == Some(request.expected_continuity_head.as_str()),
            "palace_run_identity_mismatch"
        );
        ensure!(
            item.provenance
                .iter()
                .any(|c| c.path == citation.path && c.hash == format!("sha256:{}", citation.sha256)),
            "palace_citation_mismatch"
        );
        ensure!(
            u128::from(request.observed_epoch_ms).saturating_sub(anchor.effective_epoch_ms)
                <= u128::from(request.stale_after_ms),
            "palace_stale_reference"
        );
        ensure!(seen.insert(r.run_id.clone()), "palace_ambiguous_run");
        backend.load(&r)?;
        selected.push(r);
    }
    selected.sort_by(|a, b| (&a.run_id, &a.record_digest).cmp(&(&b.run_id, &b.record_digest)));
    ensure!(
        selected.contains(&request.baseline),
        "palace_selected_baseline_missing"
    );
    let delta = comparison::compare(backend, &request.baseline, &request.current)?;
    Ok(RetrievedComparison {
        schema: VERSION.into(),
        provenance: PalaceReceipt {
            schema: VERSION.into(),
            backend: "RuntimeMemoryPalaceService".into(),
            generation: checkpoint.memory_palace_generation,
            packet_sha256: checkpoint.packet_sha256,
            checkpoint_sha256: checkpoint.checkpoint_sha256,
            continuity_head: checkpoint.continuity_head,
            identity_root: checkpoint.identity_root,
        },
        observed_epoch_ms: request.observed_epoch_ms,
        selected_references: selected,
        delta,
    })
}
