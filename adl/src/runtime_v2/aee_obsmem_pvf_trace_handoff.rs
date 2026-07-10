//! Runtime-v2 AEE / ObsMem / PVF trace handoff evidence.
//!
//! This issue-local surface turns a governed runtime trace into a concrete
//! ObsMem write through the adapter boundary and retains the PVF proof refs
//! that make the handoff reviewable.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

use super::*;
use crate::{
    artifacts, governed_executor, instrumentation,
    obsmem_adapter::ObsMemAdapter,
    obsmem_contract::{
        MemoryCitation, MemoryQueryResult, MemoryTraceRef, MemoryWriteAck, MemoryWriteRequest,
        OBSMEM_CONTRACT_VERSION,
    },
    obsmem_store::FileObsMemClient,
    trace,
};

pub const RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_SCHEMA: &str =
    "runtime_v2.aee_obsmem_pvf_trace_handoff.v1";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_PACKET: &str =
    "issue_4697/aee_obsmem_pvf_trace_handoff_packet.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST: &str =
    "issue_4697/aee_obsmem_pvf_trace_manifest.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE: &str =
    "issue_4697/obsmem_memory_write_request.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK: &str =
    "issue_4697/obsmem_memory_write_ack.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL: &str =
    "issue_4697/obsmem_retrieval_result.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_STORE: &str = "issue_4697/obsmem_store.v1.json";
pub const RUNTIME_V2_AEE_OBSMEM_PVF_ACTIVATION_LOG: &str =
    "artifacts/runtime-v2-aee-obsmem-pvf-handoff/logs/activation_log.json";

const HANDOFF_RUN_ID: &str = "issue-4697-aee-obsmem-pvf-handoff";
const TRACE_RUN_ID: &str = "runtime-v2-aee-obsmem-pvf-handoff";
const HANDOFF_WORKFLOW_ID: &str = "runtime_v2.aee_obsmem_pvf_trace_handoff";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AeeObsMemPvfTraceManifest {
    pub schema_version: String,
    pub issue: u32,
    pub run_id: String,
    pub workflow_id: String,
    pub activation_log_ref: String,
    pub aee_boundary: String,
    pub obsmem_boundary: String,
    pub pvf_lane: String,
    pub pvf_trace_handoff_refs: Vec<String>,
    pub retained_trace_event_refs: Vec<MemoryTraceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2AeeObsMemPvfTraceHandoffPacket {
    pub schema_version: String,
    pub issue: u32,
    pub milestone: String,
    pub proof_id: String,
    pub runtime_boundary: String,
    pub trace_manifest_ref: String,
    pub activation_log_ref: String,
    pub obsmem_store_ref: String,
    pub obsmem_write_ref: String,
    pub obsmem_ack_ref: String,
    pub obsmem_retrieval_ref: String,
    pub pvf_lane: String,
    pub pvf_trace_handoff_refs: Vec<String>,
    pub retained_evidence_refs: Vec<String>,
    pub validation_commands: Vec<String>,
    pub integration_summary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeV2AeeObsMemPvfTraceHandoffArtifacts {
    pub packet: RuntimeV2AeeObsMemPvfTraceHandoffPacket,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuntimeV2AeeObsMemPvfMaterializedSummary {
    packet: RuntimeV2AeeObsMemPvfTraceHandoffPacket,
    memory_write: MemoryWriteRequest,
    ack: MemoryWriteAck,
    retrieval: MemoryQueryResult,
}

impl RuntimeV2AeeObsMemPvfTraceHandoffArtifacts {
    pub fn prototype() -> Result<Self> {
        let packet = RuntimeV2AeeObsMemPvfTraceHandoffPacket {
            schema_version: RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_SCHEMA.to_string(),
            issue: 4697,
            milestone: "v0.91.7".to_string(),
            proof_id: "issue-4697-aee-obsmem-pvf-trace-handoff-0001".to_string(),
            runtime_boundary:
                "Runtime v2 governed trace -> AEE observation summary -> ObsMem adapter write -> PVF runtime lane handoff"
                    .to_string(),
            trace_manifest_ref: RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST.to_string(),
            activation_log_ref: RUNTIME_V2_AEE_OBSMEM_PVF_ACTIVATION_LOG.to_string(),
            obsmem_store_ref: RUNTIME_V2_AEE_OBSMEM_PVF_STORE.to_string(),
            obsmem_write_ref: RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE.to_string(),
            obsmem_ack_ref: RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK.to_string(),
            obsmem_retrieval_ref: RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL.to_string(),
            pvf_lane: "runtime".to_string(),
            pvf_trace_handoff_refs: vec![
                "pvf://v0.91.7/issue-4697/runtime/aee-obsmem-trace-handoff".to_string(),
                "pvf://v0.91.7/issue-4697/runtime/obsmem-retrieval-proof".to_string(),
            ],
            retained_evidence_refs: vec![
                RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_PACKET.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_ACTIVATION_LOG.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL.to_string(),
                RUNTIME_V2_AEE_OBSMEM_PVF_STORE.to_string(),
            ],
            validation_commands: vec![
                "cargo test --manifest-path adl/Cargo.toml runtime_v2_aee_obsmem_pvf_trace_handoff -- --nocapture".to_string(),
                "cargo test --manifest-path adl/Cargo.toml --lib --features slow-proof-runtime runtime_v2_aee_obsmem_pvf_trace_handoff_writes_retained_evidence -- --nocapture".to_string(),
                "cargo run --manifest-path adl/Cargo.toml -- runtime-v2 aee-obsmem-pvf-handoff --out artifacts/v0917/issue-4697-aee-obsmem-pvf-handoff".to_string(),
                "git diff --check".to_string(),
            ],
            integration_summary:
                "Issue #4697 materializes the AEE/ObsMem/PVF boundary by emitting a governed Runtime v2 trace, deriving an AEE observation handoff manifest, writing it through the ObsMem adapter into a file-backed store, and querying that store with runtime PVF tags while retaining every trace and memory artifact."
                    .to_string(),
            non_claims: vec![
                "does not claim sibling WP-11 loop runtime or adl.skill.v1 completion".to_string(),
                "does not claim complete v0.92 birthday readiness".to_string(),
                "does not replace the broader Soak #2 runtime proof owned outside #4697".to_string(),
            ],
        };
        let artifacts = Self { packet };
        artifacts.validate()?;
        Ok(artifacts)
    }

    pub fn validate(&self) -> Result<()> {
        self.packet.validate()
    }

    pub fn write_to_root(&self, root: impl AsRef<Path>) -> Result<()> {
        let root = root.as_ref();
        self.validate()?;
        let manifest = write_runtime_trace_and_manifest(root)?;
        let memory_write = build_memory_write(root, &manifest)?;
        write_relative(
            root,
            RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE,
            serde_json::to_vec_pretty(&memory_write)
                .context("serialize AEE ObsMem memory write")?,
        )?;

        let adapter = ObsMemAdapter::new(FileObsMemClient::new(
            root.join(RUNTIME_V2_AEE_OBSMEM_PVF_STORE),
        ));
        let ack = adapter
            .index_prebuilt_write_request(&memory_write)
            .map_err(|err| anyhow!("write AEE ObsMem handoff through adapter: {err}"))?;
        write_relative(
            root,
            RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK,
            serde_json::to_vec_pretty(&ack).context("serialize AEE ObsMem write ack")?,
        )?;

        let query_tags = vec!["issue:4697".to_string(), "pvf:runtime".to_string()];
        let retrieval = adapter
            .query(Some(HANDOFF_WORKFLOW_ID), None, &query_tags, 10)
            .map_err(|err| anyhow!("query AEE ObsMem handoff through adapter: {err}"))?;
        if retrieval.hits.len() != 1 {
            return Err(anyhow!(
                "AEE ObsMem handoff retrieval expected exactly one hit, got {}",
                retrieval.hits.len()
            ));
        }
        write_relative(
            root,
            RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL,
            serde_json::to_vec_pretty(&retrieval)
                .context("serialize AEE ObsMem retrieval result")?,
        )?;

        let materialized = RuntimeV2AeeObsMemPvfMaterializedSummary {
            packet: self.packet.clone(),
            memory_write,
            ack,
            retrieval,
        };
        materialized.validate()?;
        write_relative(
            root,
            RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_PACKET,
            serde_json::to_vec_pretty(&self.packet)
                .context("serialize AEE ObsMem PVF handoff packet")?,
        )
    }
}

impl RuntimeV2AeeObsMemPvfTraceHandoffPacket {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_SCHEMA {
            return Err(anyhow!(
                "unsupported AEE ObsMem PVF handoff schema '{}'",
                self.schema_version
            ));
        }
        if self.issue != 4697 {
            return Err(anyhow!(
                "AEE ObsMem PVF handoff packet must remain bound to #4697"
            ));
        }
        if self.milestone != "v0.91.7" || self.pvf_lane != "runtime" {
            return Err(anyhow!(
                "AEE ObsMem PVF handoff packet must stay in v0.91.7 runtime PVF lane"
            ));
        }
        for (field, value) in [
            ("trace_manifest_ref", &self.trace_manifest_ref),
            ("activation_log_ref", &self.activation_log_ref),
            ("obsmem_store_ref", &self.obsmem_store_ref),
            ("obsmem_write_ref", &self.obsmem_write_ref),
            ("obsmem_ack_ref", &self.obsmem_ack_ref),
            ("obsmem_retrieval_ref", &self.obsmem_retrieval_ref),
        ] {
            validate_relative_path(value, &format!("aee_obsmem_pvf.{field}"))?;
        }
        validate_relative_path_list(
            &self.retained_evidence_refs,
            "aee_obsmem_pvf.retained_evidence_refs",
        )?;
        for required in [
            RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_PACKET,
            RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST,
            RUNTIME_V2_AEE_OBSMEM_PVF_ACTIVATION_LOG,
            RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE,
            RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK,
            RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL,
            RUNTIME_V2_AEE_OBSMEM_PVF_STORE,
        ] {
            if !self.retained_evidence_refs.iter().any(|value| value == required) {
                return Err(anyhow!(
                    "AEE ObsMem PVF handoff missing retained evidence ref '{required}'"
                ));
            }
        }
        if self.pvf_trace_handoff_refs.is_empty()
            || !self
                .pvf_trace_handoff_refs
                .iter()
                .all(|value| value.starts_with("pvf://"))
        {
            return Err(anyhow!(
                "AEE ObsMem PVF handoff refs must use pvf:// identifiers"
            ));
        }
        if !self
            .validation_commands
            .iter()
            .any(|command| command.contains("aee-obsmem-pvf-handoff"))
        {
            return Err(anyhow!(
                "AEE ObsMem PVF handoff must include the runnable CLI proof command"
            ));
        }
        validate_nonempty_text(&self.runtime_boundary, "aee_obsmem_pvf.runtime_boundary")?;
        validate_nonempty_text(
            &self.integration_summary,
            "aee_obsmem_pvf.integration_summary",
        )?;
        if !self
            .non_claims
            .iter()
            .any(|claim| claim.contains("sibling WP-11 loop runtime"))
        {
            return Err(anyhow!(
                "AEE ObsMem PVF handoff must preserve sibling WP-11 non-claim"
            ));
        }
        Ok(())
    }
}

impl RuntimeV2AeeObsMemPvfTraceManifest {
    fn validate(&self) -> Result<()> {
        if self.schema_version != "runtime_v2.aee_obsmem_pvf_trace_manifest.v1" {
            return Err(anyhow!(
                "unsupported AEE ObsMem PVF trace manifest schema '{}'",
                self.schema_version
            ));
        }
        if self.issue != 4697 || self.pvf_lane != "runtime" {
            return Err(anyhow!(
                "AEE ObsMem PVF trace manifest must remain bound to #4697 runtime lane"
            ));
        }
        validate_relative_path(
            &self.activation_log_ref,
            "aee_obsmem_pvf_trace_manifest.activation_log_ref",
        )?;
        if self.retained_trace_event_refs.len() < 2 {
            return Err(anyhow!(
                "AEE ObsMem PVF trace manifest must retain at least two trace event refs"
            ));
        }
        for value in &self.pvf_trace_handoff_refs {
            if !value.starts_with("pvf://") {
                return Err(anyhow!(
                    "AEE ObsMem PVF trace manifest handoff refs must use pvf:// identifiers"
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeV2AeeObsMemPvfMaterializedSummary {
    fn validate(&self) -> Result<()> {
        self.packet.validate()?;
        self.memory_write
            .validate()
            .map_err(|err| anyhow!("AEE ObsMem memory write failed validation: {err}"))?;
        if !self.ack.accepted {
            return Err(anyhow!("AEE ObsMem memory write was not accepted"));
        }
        if self.retrieval.hits.len() != 1 {
            return Err(anyhow!("AEE ObsMem retrieval must produce one retained hit"));
        }
        Ok(())
    }
}

pub fn write_runtime_trace_and_manifest(
    root: &Path,
) -> Result<RuntimeV2AeeObsMemPvfTraceManifest> {
    let mut governed_trace = trace::Trace::new(
        TRACE_RUN_ID.to_string(),
        HANDOFF_WORKFLOW_ID.to_string(),
        "0.91.7".to_string(),
    );
    let outcome = governed_executor::emit_fixture_safe_read_trace_v1(&mut governed_trace);
    if outcome.selected_actions.is_empty() {
        return Err(anyhow!(
            "AEE ObsMem PVF handoff trace must emit one selected governed action"
        ));
    }
    let run_paths =
        artifacts::RunArtifactPaths::for_run_in_root(TRACE_RUN_ID, root.join("artifacts"))?;
    run_paths.ensure_layout()?;
    run_paths.write_model_marker()?;
    instrumentation::write_trace_artifact(&run_paths.activation_log_json(), &governed_trace.events)?;

    let trace_refs = retained_trace_refs(&run_paths.activation_log_json())?;
    let manifest = RuntimeV2AeeObsMemPvfTraceManifest {
        schema_version: "runtime_v2.aee_obsmem_pvf_trace_manifest.v1".to_string(),
        issue: 4697,
        run_id: HANDOFF_RUN_ID.to_string(),
        workflow_id: HANDOFF_WORKFLOW_ID.to_string(),
        activation_log_ref: RUNTIME_V2_AEE_OBSMEM_PVF_ACTIVATION_LOG.to_string(),
        aee_boundary: "AEE observes governed runtime outcome and records bounded adaptation context"
            .to_string(),
        obsmem_boundary: "ObsMem adapter receives a validated MemoryWriteRequest with trace refs"
            .to_string(),
        pvf_lane: "runtime".to_string(),
        pvf_trace_handoff_refs: vec![
            "pvf://v0.91.7/issue-4697/runtime/aee-obsmem-trace-handoff".to_string(),
            "pvf://v0.91.7/issue-4697/runtime/obsmem-retrieval-proof".to_string(),
        ],
        retained_trace_event_refs: trace_refs,
    };
    manifest.validate()?;
    write_relative(
        root,
        RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST,
        serde_json::to_vec_pretty(&manifest).context("serialize AEE ObsMem trace manifest")?,
    )?;
    Ok(manifest)
}

fn build_memory_write(
    root: &Path,
    manifest: &RuntimeV2AeeObsMemPvfTraceManifest,
) -> Result<MemoryWriteRequest> {
    let mut request = MemoryWriteRequest {
        contract_version: OBSMEM_CONTRACT_VERSION,
        run_id: HANDOFF_RUN_ID.to_string(),
        workflow_id: HANDOFF_WORKFLOW_ID.to_string(),
        trace_bundle_rel_path: RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST.to_string(),
        activation_log_rel_path: manifest.activation_log_ref.clone(),
        failure_code: None,
        summary: "AEE observed the governed Runtime v2 action, handed retained trace refs to ObsMem, and exposed runtime PVF handoff refs for validation consumption.".to_string(),
        tags: vec![
            "aee:observation".to_string(),
            "issue:4697".to_string(),
            "obsmem:write-through-adapter".to_string(),
            "pvf:runtime".to_string(),
            "runtime-v2:aee-obsmem-pvf".to_string(),
        ],
        citations: vec![
            citation_for_path(root, RUNTIME_V2_AEE_OBSMEM_PVF_TRACE_MANIFEST)?,
            citation_for_path(root, &manifest.activation_log_ref)?,
        ],
        trace_event_refs: manifest.retained_trace_event_refs.clone(),
        temporal_anchor: None,
        review_findings: Vec::new(),
        residual_risks: vec![
            "Broader Soak #2 memory consumption remains outside issue #4697.".to_string(),
        ],
        follow_on_refs: Vec::new(),
    };
    request.normalize();
    request
        .validate()
        .map_err(|err| anyhow!("build AEE ObsMem memory write request: {err}"))?;
    Ok(request)
}

fn retained_trace_refs(path: &Path) -> Result<Vec<MemoryTraceRef>> {
    let trace = instrumentation::load_trace_artifact(path)
        .with_context(|| format!("load AEE ObsMem trace artifact {}", path.display()))?;
    let mut refs = Vec::new();
    for (sequence, event) in trace.iter().enumerate() {
        match event {
            instrumentation::TraceEventNormalized::GovernedActionSelected { .. } => {
                refs.push(MemoryTraceRef {
                    event_sequence: sequence,
                    event_kind: "governed_action_selected".to_string(),
                    step_id: Some("aee_obsmem_pvf_handoff".to_string()),
                    delegation_id: None,
                });
            }
            instrumentation::TraceEventNormalized::GovernedExecutionResultRecorded { .. } => {
                refs.push(MemoryTraceRef {
                    event_sequence: sequence,
                    event_kind: "governed_execution_result_recorded".to_string(),
                    step_id: Some("aee_obsmem_pvf_handoff".to_string()),
                    delegation_id: None,
                });
            }
            _ => {}
        }
    }
    if refs.len() < 2 {
        return Err(anyhow!(
            "AEE ObsMem PVF handoff trace did not retain selected action and execution result refs"
        ));
    }
    Ok(refs)
}

fn citation_for_path(root: &Path, rel_path: &str) -> Result<MemoryCitation> {
    validate_relative_path(rel_path, "aee_obsmem_pvf.citation")?;
    let bytes = fs::read(root.join(rel_path))
        .with_context(|| format!("read AEE ObsMem citation source {rel_path}"))?;
    Ok(MemoryCitation {
        path: rel_path.to_string(),
        hash: format!("sha256:{:x}", Sha256::digest(bytes)),
    })
}

fn validate_relative_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let mut seen = std::collections::BTreeSet::new();
    for value in values {
        validate_relative_path(value, field)?;
        if !seen.insert(value.clone()) {
            return Err(anyhow!("{field} contains duplicate artifact ref"));
        }
    }
    Ok(())
}
