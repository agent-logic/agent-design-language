//! `adl.skill.v1` contract and bounded runtime consumption proof.
//!
//! This module implements the pre-v0.92 skill standard surface without touching
//! the sibling WP-11 loop-runtime work. It provides a concrete versioned schema,
//! deterministic validation, runtime selection, and dispatch evidence that can
//! be consumed by later runtime-v2 integration work.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub const ADL_SKILL_V1_SCHEMA: &str = "adl.skill.v1";
pub const ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA: &str = "adl.skill_runtime_dispatch_proof.v1";
pub const ADL_SKILL_V1_MARKER: &str = "adl_skill_v1";
pub const ADL_SKILL_V1_ARTIFACT_PREFIX: &str = "artifacts/adl_skill_v1/";
const MAX_SKILL_COLLECTION_ITEMS: usize = 64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillV1 {
    pub schema_version: String,
    pub skill_id: String,
    pub version: String,
    pub display_name: String,
    pub description: String,
    pub lifecycle: AdlSkillLifecycleV1,
    pub invocation: AdlSkillInvocationV1,
    pub capabilities: Vec<AdlSkillCapabilityV1>,
    pub inputs: Vec<AdlSkillInputV1>,
    pub outputs: Vec<AdlSkillOutputV1>,
    pub runtime: AdlSkillRuntimeV1,
    pub trace: AdlSkillTraceV1,
    pub review: AdlSkillReviewV1,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillLifecycleV1 {
    pub phases: Vec<AdlSkillLifecyclePhaseV1>,
    pub terminal_states: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillLifecyclePhaseV1 {
    pub phase_id: String,
    pub required: bool,
    pub trace_event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillInvocationV1 {
    pub invocation_kind: AdlSkillInvocationKindV1,
    pub entrypoint: String,
    pub required_context_fields: Vec<String>,
    pub deterministic: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdlSkillInvocationKindV1 {
    RustFunction,
    Command,
    PromptTemplate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillCapabilityV1 {
    pub capability_id: String,
    pub purpose: String,
    pub authority: AdlSkillCapabilityAuthorityV1,
    pub required_inputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AdlSkillCapabilityAuthorityV1 {
    ReadOnly,
    WorkspaceWrite,
    ExternalNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillInputV1 {
    pub input_id: String,
    pub schema_ref: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillOutputV1 {
    pub output_id: String,
    pub schema_ref: String,
    pub artifact_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillRuntimeV1 {
    pub selectable_by: Vec<String>,
    pub dispatch_contract: String,
    pub failure_modes: Vec<AdlSkillFailureModeV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillFailureModeV1 {
    pub failure_id: String,
    pub when: String,
    pub runtime_result: AdlSkillDispatchStatusV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillTraceV1 {
    pub span_kind: String,
    pub required_events: Vec<String>,
    pub correlation_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillReviewV1 {
    pub required_review_questions: Vec<String>,
    pub publication_gate: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillRuntimeRequestV1 {
    pub request_id: String,
    pub requested_capability: String,
    pub required_authority: AdlSkillCapabilityAuthorityV1,
    pub input_refs: Vec<String>,
    pub trace_correlation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillRuntimeDispatchProofV1 {
    pub schema_version: String,
    pub request: AdlSkillRuntimeRequestV1,
    pub selected_skill_id: String,
    pub selected_skill_version: String,
    pub dispatch_status: AdlSkillDispatchStatusV1,
    pub dispatch_entrypoint: String,
    pub consumed_input_refs: Vec<String>,
    pub produced_output_refs: Vec<String>,
    pub selection_trace: Vec<AdlSkillSelectionTraceV1>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdlSkillSelectionTraceV1 {
    pub skill_id: String,
    pub matched_capability: bool,
    pub authority_allowed: bool,
    pub missing_inputs: Vec<String>,
    pub undeclared_inputs: Vec<String>,
    pub selected: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AdlSkillDispatchStatusV1 {
    Selected,
    RejectedInvalidSchema,
    RejectedVersionMismatch,
    RejectedMissingRequiredField,
    RejectedSelectionFailure,
}

#[derive(Debug, Clone, Default)]
pub struct AdlSkillRuntimeCatalogV1 {
    skills: Vec<AdlSkillV1>,
}

impl AdlSkillV1 {
    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            ADL_SKILL_V1_SCHEMA,
            "skill.schema_version",
        )?;
        normalize_id(&self.skill_id, "skill.skill_id")?;
        validate_semverish_version(&self.version, "skill.version")?;
        validate_nonempty(&self.display_name, "skill.display_name")?;
        validate_nonempty(&self.description, "skill.description")?;
        validate_lifecycle(&self.lifecycle)?;
        validate_invocation(&self.invocation)?;
        validate_capabilities(&self.capabilities)?;
        validate_inputs(&self.inputs)?;
        validate_capability_inputs_declared(&self.capabilities, &self.inputs)?;
        validate_outputs(&self.outputs)?;
        validate_runtime(&self.runtime, &self.capabilities)?;
        validate_trace(&self.trace)?;
        validate_review(&self.review)?;
        validate_non_claims(&self.non_claims)?;
        Ok(())
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .lifecycle
            .phases
            .sort_by(|left, right| left.phase_id.cmp(&right.phase_id));
        canonical.lifecycle.terminal_states.sort();
        canonical
            .capabilities
            .sort_by(|left, right| left.capability_id.cmp(&right.capability_id));
        canonical
            .inputs
            .sort_by(|left, right| left.input_id.cmp(&right.input_id));
        canonical
            .outputs
            .sort_by(|left, right| left.output_id.cmp(&right.output_id));
        canonical.runtime.selectable_by.sort();
        canonical
            .runtime
            .failure_modes
            .sort_by(|left, right| left.failure_id.cmp(&right.failure_id));
        canonical.trace.required_events.sort();
        canonical.trace.correlation_fields.sort();
        canonical.review.required_review_questions.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?).context("serialize adl.skill.v1")
    }
}

impl AdlSkillRuntimeCatalogV1 {
    pub fn new(skills: Vec<AdlSkillV1>) -> Result<Self> {
        ensure_len_within(&skills, "skill_catalog.skills")?;
        let mut capability_ids = BTreeSet::new();
        for skill in &skills {
            skill.validate()?;
            for capability in &skill.capabilities {
                if !capability_ids.insert(capability.capability_id.clone()) {
                    return Err(anyhow!(
                        "skill catalog capability '{}' is ambiguous across skills",
                        capability.capability_id
                    ));
                }
            }
        }
        Ok(Self { skills })
    }

    pub fn dispatch(
        &self,
        request: AdlSkillRuntimeRequestV1,
    ) -> Result<AdlSkillRuntimeDispatchProofV1> {
        validate_request(&request)?;
        let mut traces = Vec::new();

        for skill in &self.skills {
            let Some(capability) = skill
                .capabilities
                .iter()
                .find(|capability| capability.capability_id == request.requested_capability)
            else {
                traces.push(selection_trace(
                    skill,
                    false,
                    false,
                    Vec::new(),
                    Vec::new(),
                    false,
                    "requested capability not declared by skill",
                ));
                continue;
            };

            let authority_allowed = authority_matches_request(capability, &request);
            let missing_inputs = missing_required_inputs(capability, &request.input_refs);
            let undeclared_inputs = undeclared_request_inputs(skill, &request.input_refs);
            let selected =
                authority_allowed && missing_inputs.is_empty() && undeclared_inputs.is_empty();
            traces.push(selection_trace(
                skill,
                true,
                authority_allowed,
                missing_inputs.clone(),
                undeclared_inputs.clone(),
                selected,
                if selected {
                    "selected by capability, authority, and input refs"
                } else if !authority_allowed {
                    "skill authority must exactly match request authority; no implicit upgrade or downgrade is allowed"
                } else if !undeclared_inputs.is_empty() {
                    "request includes input refs not declared by the skill"
                } else {
                    "request is missing required input refs"
                },
            ));

            if selected {
                let proof = AdlSkillRuntimeDispatchProofV1 {
                    schema_version: ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA.to_string(),
                    request,
                    selected_skill_id: skill.skill_id.clone(),
                    selected_skill_version: skill.version.clone(),
                    dispatch_status: AdlSkillDispatchStatusV1::Selected,
                    dispatch_entrypoint: skill.invocation.entrypoint.clone(),
                    consumed_input_refs: capability.required_inputs.clone(),
                    produced_output_refs: skill
                        .outputs
                        .iter()
                        .map(|output| output.artifact_ref.clone())
                        .collect(),
                    selection_trace: traces,
                    non_claims: runtime_non_claims(),
                };
                proof.validate()?;
                return Ok(proof);
            }
        }

        Err(anyhow!(
            "adl.skill.v1 runtime selection failed for capability '{}'",
            request.requested_capability
        ))
    }
}

impl AdlSkillRuntimeDispatchProofV1 {
    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA,
            "skill_runtime.schema_version",
        )?;
        validate_request(&self.request)?;
        normalize_id(&self.selected_skill_id, "skill_runtime.selected_skill_id")?;
        validate_semverish_version(
            &self.selected_skill_version,
            "skill_runtime.selected_skill_version",
        )?;
        if self.dispatch_status != AdlSkillDispatchStatusV1::Selected {
            return Err(anyhow!("skill runtime proof must record selected dispatch"));
        }
        validate_entrypoint_ref(
            &self.dispatch_entrypoint,
            "skill_runtime.dispatch_entrypoint",
        )?;
        validate_nonempty_list(
            &self.consumed_input_refs,
            "skill_runtime.consumed_input_refs",
        )?;
        ensure_len_within(
            &self.consumed_input_refs,
            "skill_runtime.consumed_input_refs",
        )?;
        validate_nonempty_list(
            &self.produced_output_refs,
            "skill_runtime.produced_output_refs",
        )?;
        ensure_len_within(
            &self.produced_output_refs,
            "skill_runtime.produced_output_refs",
        )?;
        for produced_output_ref in &self.produced_output_refs {
            validate_artifact_ref(produced_output_ref, "skill_runtime.produced_output_refs")?;
        }
        ensure_len_within(&self.selection_trace, "skill_runtime.selection_trace")?;
        if !self.selection_trace.iter().any(|trace| trace.selected) {
            return Err(anyhow!(
                "skill runtime proof must include a selected trace row"
            ));
        }
        let selected_traces: Vec<_> = self
            .selection_trace
            .iter()
            .filter(|trace| trace.selected)
            .collect();
        if selected_traces.len() != 1 {
            return Err(anyhow!(
                "skill runtime proof must include exactly one selected trace row"
            ));
        }
        let selected_trace = selected_traces[0];
        if selected_trace.skill_id != self.selected_skill_id {
            return Err(anyhow!(
                "skill runtime selected trace must match selected_skill_id"
            ));
        }
        if !selected_trace.matched_capability || !selected_trace.authority_allowed {
            return Err(anyhow!(
                "skill runtime selected trace must prove capability and authority match"
            ));
        }
        if !selected_trace.missing_inputs.is_empty() || !selected_trace.undeclared_inputs.is_empty()
        {
            return Err(anyhow!(
                "skill runtime selected trace must not retain input mismatches"
            ));
        }
        let requested_inputs: BTreeSet<_> =
            self.request.input_refs.iter().map(String::as_str).collect();
        for consumed in &self.consumed_input_refs {
            if !requested_inputs.contains(consumed.as_str()) {
                return Err(anyhow!(
                    "skill runtime consumed inputs must be present in the request"
                ));
            }
        }
        validate_non_claims(&self.non_claims)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).context("serialize adl.skill.v1 runtime proof")
    }
}

pub fn sample_adl_skill_v1() -> AdlSkillV1 {
    AdlSkillV1 {
        schema_version: ADL_SKILL_V1_SCHEMA.to_string(),
        skill_id: "skill.standard.review_packet_builder".to_string(),
        version: "1.0.0".to_string(),
        display_name: "Review Packet Builder".to_string(),
        description: "Builds a bounded review packet from issue-local source refs.".to_string(),
        lifecycle: AdlSkillLifecycleV1 {
            phases: vec![
                phase("bind", true, "skill.bind"),
                phase("validate", true, "skill.validate"),
                phase("execute", true, "skill.execute"),
                phase("commit", true, "skill.commit"),
            ],
            terminal_states: strings(&["completed", "failed", "skipped"]),
        },
        invocation: AdlSkillInvocationV1 {
            invocation_kind: AdlSkillInvocationKindV1::RustFunction,
            entrypoint: "adl::adl_skill_v1::sample_review_packet_builder".to_string(),
            required_context_fields: strings(&[
                "run_id",
                "invocation_id",
                "artifact_root",
                "trace_correlation_id",
            ]),
            deterministic: true,
        },
        capabilities: vec![AdlSkillCapabilityV1 {
            capability_id: "review.packet.build".to_string(),
            purpose: "materialize an issue-local review packet with repository-relative refs"
                .to_string(),
            authority: AdlSkillCapabilityAuthorityV1::WorkspaceWrite,
            required_inputs: strings(&["issue_ref", "scope_ref", "artifact_root"]),
        }],
        inputs: vec![
            input("issue_ref", "adl.issue_ref.v1", true),
            input("scope_ref", "adl.scope_ref.v1", true),
            input("artifact_root", "adl.artifact_root.v1", true),
        ],
        outputs: vec![AdlSkillOutputV1 {
            output_id: "review_packet".to_string(),
            schema_ref: "adl.review_packet.v1".to_string(),
            artifact_ref: "artifacts/adl_skill_v1/review_packet.json".to_string(),
        }],
        runtime: AdlSkillRuntimeV1 {
            selectable_by: strings(&["capability_id", "authority", "required_inputs"]),
            dispatch_contract: ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA.to_string(),
            failure_modes: vec![
                failure(
                    "invalid_schema",
                    "schema_version is not adl.skill.v1",
                    AdlSkillDispatchStatusV1::RejectedInvalidSchema,
                ),
                failure(
                    "missing_required_field",
                    "a required field or list is absent or empty",
                    AdlSkillDispatchStatusV1::RejectedMissingRequiredField,
                ),
                failure(
                    "version_mismatch",
                    "skill version is not a concrete major.minor.patch release",
                    AdlSkillDispatchStatusV1::RejectedVersionMismatch,
                ),
                failure(
                    "selection_failure",
                    "no skill matches requested capability, authority, and inputs",
                    AdlSkillDispatchStatusV1::RejectedSelectionFailure,
                ),
            ],
        },
        trace: AdlSkillTraceV1 {
            span_kind: "skill".to_string(),
            required_events: strings(&[
                "skill.selection.started",
                "skill.selection.completed",
                "skill.dispatch.started",
                "skill.dispatch.completed",
            ]),
            correlation_fields: strings(&["request_id", "trace_correlation_id", "skill_id"]),
        },
        review: AdlSkillReviewV1 {
            required_review_questions: strings(&[
                "Does the selected skill match the requested capability?",
                "Are required inputs and authority visible before dispatch?",
                "Are outputs repository-relative and trace-correlated?",
            ]),
            publication_gate: "pre_pr_independent_review".to_string(),
        },
        non_claims: runtime_non_claims(),
    }
}

pub fn sample_runtime_request_v1() -> AdlSkillRuntimeRequestV1 {
    AdlSkillRuntimeRequestV1 {
        request_id: "skill-request-0001".to_string(),
        requested_capability: "review.packet.build".to_string(),
        required_authority: AdlSkillCapabilityAuthorityV1::WorkspaceWrite,
        input_refs: strings(&["issue_ref", "scope_ref", "artifact_root"]),
        trace_correlation_id: "trace-skill-v1-0001".to_string(),
    }
}

pub fn sample_adl_skill_v1_runtime_dispatch_proof() -> Result<AdlSkillRuntimeDispatchProofV1> {
    AdlSkillRuntimeCatalogV1::new(vec![sample_adl_skill_v1()])?
        .dispatch(sample_runtime_request_v1())
}

fn validate_lifecycle(lifecycle: &AdlSkillLifecycleV1) -> Result<()> {
    if lifecycle.phases.is_empty() {
        return Err(anyhow!("skill.lifecycle.phases must not be empty"));
    }
    ensure_len_within(&lifecycle.phases, "skill.lifecycle.phases")?;
    ensure_len_within(
        &lifecycle.terminal_states,
        "skill.lifecycle.terminal_states",
    )?;
    let mut phase_ids = BTreeSet::new();
    for phase in &lifecycle.phases {
        normalize_id(&phase.phase_id, "skill.lifecycle.phase_id")?;
        if !phase_ids.insert(phase.phase_id.clone()) {
            return Err(anyhow!(
                "skill lifecycle phase '{}' is duplicated",
                phase.phase_id
            ));
        }
        validate_nonempty(&phase.trace_event, "skill.lifecycle.trace_event")?;
    }
    for required in ["bind", "validate", "execute", "commit"] {
        if !phase_ids.contains(required) {
            return Err(anyhow!(
                "skill lifecycle missing required phase '{required}'"
            ));
        }
    }
    validate_nonempty_list(
        &lifecycle.terminal_states,
        "skill.lifecycle.terminal_states",
    )
}

fn validate_invocation(invocation: &AdlSkillInvocationV1) -> Result<()> {
    validate_entrypoint_ref(&invocation.entrypoint, "skill.invocation.entrypoint")?;
    if !invocation.deterministic {
        return Err(anyhow!(
            "skill.invocation.deterministic must be true for v0.91.7 proof"
        ));
    }
    validate_nonempty_list(
        &invocation.required_context_fields,
        "skill.invocation.required_context_fields",
    )?;
    ensure_len_within(
        &invocation.required_context_fields,
        "skill.invocation.required_context_fields",
    )?;
    for required in [
        "run_id",
        "invocation_id",
        "artifact_root",
        "trace_correlation_id",
    ] {
        ensure_contains_exact(
            &invocation.required_context_fields,
            required,
            "skill invocation context is missing required field",
        )?;
    }
    Ok(())
}

fn validate_capabilities(capabilities: &[AdlSkillCapabilityV1]) -> Result<()> {
    if capabilities.is_empty() {
        return Err(anyhow!("skill.capabilities must not be empty"));
    }
    ensure_len_within(capabilities, "skill.capabilities")?;
    let mut ids = BTreeSet::new();
    for capability in capabilities {
        normalize_id(&capability.capability_id, "skill.capability_id")?;
        if !ids.insert(capability.capability_id.clone()) {
            return Err(anyhow!(
                "skill capability '{}' is duplicated",
                capability.capability_id
            ));
        }
        validate_nonempty(&capability.purpose, "skill.capability.purpose")?;
        validate_nonempty_list(
            &capability.required_inputs,
            "skill.capability.required_inputs",
        )?;
        ensure_len_within(
            &capability.required_inputs,
            "skill.capability.required_inputs",
        )?;
    }
    Ok(())
}

fn validate_capability_inputs_declared(
    capabilities: &[AdlSkillCapabilityV1],
    inputs: &[AdlSkillInputV1],
) -> Result<()> {
    let declared_inputs: BTreeSet<_> = inputs.iter().map(|input| input.input_id.as_str()).collect();
    for capability in capabilities {
        for required_input in &capability.required_inputs {
            if !declared_inputs.contains(required_input.as_str()) {
                return Err(anyhow!(
                    "skill capability '{}' requires undeclared input '{}'",
                    capability.capability_id,
                    required_input
                ));
            }
        }
    }
    Ok(())
}

fn validate_inputs(inputs: &[AdlSkillInputV1]) -> Result<()> {
    if inputs.is_empty() {
        return Err(anyhow!("skill.inputs must not be empty"));
    }
    ensure_len_within(inputs, "skill.inputs")?;
    let mut ids = BTreeSet::new();
    for input in inputs {
        normalize_id(&input.input_id, "skill.input_id")?;
        if !ids.insert(input.input_id.clone()) {
            return Err(anyhow!("skill input '{}' is duplicated", input.input_id));
        }
        validate_schema_ref(&input.schema_ref, "skill.input.schema_ref")?;
    }
    Ok(())
}

fn validate_outputs(outputs: &[AdlSkillOutputV1]) -> Result<()> {
    if outputs.is_empty() {
        return Err(anyhow!("skill.outputs must not be empty"));
    }
    ensure_len_within(outputs, "skill.outputs")?;
    let mut ids = BTreeSet::new();
    for output in outputs {
        normalize_id(&output.output_id, "skill.output_id")?;
        if !ids.insert(output.output_id.clone()) {
            return Err(anyhow!("skill output '{}' is duplicated", output.output_id));
        }
        validate_schema_ref(&output.schema_ref, "skill.output.schema_ref")?;
        validate_artifact_ref(&output.artifact_ref, "skill.output.artifact_ref")?;
    }
    Ok(())
}

fn validate_runtime(
    runtime: &AdlSkillRuntimeV1,
    capabilities: &[AdlSkillCapabilityV1],
) -> Result<()> {
    validate_nonempty_list(&runtime.selectable_by, "skill.runtime.selectable_by")?;
    ensure_len_within(&runtime.selectable_by, "skill.runtime.selectable_by")?;
    for selector in ["capability_id", "authority", "required_inputs"] {
        ensure_contains_exact(
            &runtime.selectable_by,
            selector,
            "skill runtime selector set is incomplete",
        )?;
    }
    require_exact(
        &runtime.dispatch_contract,
        ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA,
        "skill.runtime.dispatch_contract",
    )?;
    if runtime.failure_modes.len() < 4 {
        return Err(anyhow!(
            "skill runtime must declare required negative cases"
        ));
    }
    ensure_len_within(&runtime.failure_modes, "skill.runtime.failure_modes")?;
    let mut failure_ids = BTreeSet::new();
    let declared: BTreeSet<_> = runtime
        .failure_modes
        .iter()
        .map(|mode| mode.runtime_result.clone())
        .collect();
    for required in [
        AdlSkillDispatchStatusV1::RejectedInvalidSchema,
        AdlSkillDispatchStatusV1::RejectedMissingRequiredField,
        AdlSkillDispatchStatusV1::RejectedVersionMismatch,
        AdlSkillDispatchStatusV1::RejectedSelectionFailure,
    ] {
        if !declared.contains(&required) {
            return Err(anyhow!("skill runtime missing required failure mode"));
        }
    }
    for mode in &runtime.failure_modes {
        normalize_id(&mode.failure_id, "skill.runtime.failure_id")?;
        if !failure_ids.insert(mode.failure_id.clone()) {
            return Err(anyhow!(
                "skill runtime failure mode '{}' is duplicated",
                mode.failure_id
            ));
        }
        validate_nonempty(&mode.when, "skill.runtime.failure.when")?;
    }

    let capability_ids: BTreeSet<_> = capabilities
        .iter()
        .map(|capability| capability.capability_id.as_str())
        .collect();
    if capability_ids.is_empty() {
        return Err(anyhow!("skill runtime cannot select without capabilities"));
    }
    Ok(())
}

fn validate_trace(trace: &AdlSkillTraceV1) -> Result<()> {
    require_exact(&trace.span_kind, "skill", "skill.trace.span_kind")?;
    validate_nonempty_list(&trace.required_events, "skill.trace.required_events")?;
    ensure_len_within(&trace.required_events, "skill.trace.required_events")?;
    for event in [
        "skill.selection.started",
        "skill.selection.completed",
        "skill.dispatch.started",
        "skill.dispatch.completed",
    ] {
        ensure_contains_exact(
            &trace.required_events,
            event,
            "skill trace is missing a required runtime event",
        )?;
    }
    validate_nonempty_list(&trace.correlation_fields, "skill.trace.correlation_fields")?;
    ensure_len_within(&trace.correlation_fields, "skill.trace.correlation_fields")
}

fn validate_review(review: &AdlSkillReviewV1) -> Result<()> {
    validate_nonempty_list(
        &review.required_review_questions,
        "skill.review.required_review_questions",
    )?;
    ensure_len_within(
        &review.required_review_questions,
        "skill.review.required_review_questions",
    )?;
    require_exact(
        &review.publication_gate,
        "pre_pr_independent_review",
        "skill.review.publication_gate",
    )
}

fn validate_request(request: &AdlSkillRuntimeRequestV1) -> Result<()> {
    normalize_id(&request.request_id, "skill_runtime.request_id")?;
    normalize_id(
        &request.requested_capability,
        "skill_runtime.requested_capability",
    )?;
    validate_nonempty_list(&request.input_refs, "skill_runtime.input_refs")?;
    ensure_len_within(&request.input_refs, "skill_runtime.input_refs")?;
    normalize_id(
        &request.trace_correlation_id,
        "skill_runtime.trace_correlation_id",
    )
}

fn missing_required_inputs(
    capability: &AdlSkillCapabilityV1,
    input_refs: &[String],
) -> Vec<String> {
    let inputs: BTreeSet<_> = input_refs.iter().map(String::as_str).collect();
    capability
        .required_inputs
        .iter()
        .filter(|input| !inputs.contains(input.as_str()))
        .cloned()
        .collect()
}

fn undeclared_request_inputs(skill: &AdlSkillV1, input_refs: &[String]) -> Vec<String> {
    let declared_inputs: BTreeSet<_> = skill
        .inputs
        .iter()
        .map(|input| input.input_id.as_str())
        .collect();
    input_refs
        .iter()
        .filter(|input| !declared_inputs.contains(input.as_str()))
        .cloned()
        .collect()
}

fn authority_matches_request(
    capability: &AdlSkillCapabilityV1,
    request: &AdlSkillRuntimeRequestV1,
) -> bool {
    if authority_rank(&capability.authority) != authority_rank(&request.required_authority) {
        return false;
    }
    matches!(
        (&capability.authority, &request.required_authority),
        (
            AdlSkillCapabilityAuthorityV1::ReadOnly,
            AdlSkillCapabilityAuthorityV1::ReadOnly
        ) | (
            AdlSkillCapabilityAuthorityV1::WorkspaceWrite,
            AdlSkillCapabilityAuthorityV1::WorkspaceWrite
        ) | (
            AdlSkillCapabilityAuthorityV1::ExternalNetwork,
            AdlSkillCapabilityAuthorityV1::ExternalNetwork
        )
    )
}

fn authority_rank(authority: &AdlSkillCapabilityAuthorityV1) -> u8 {
    match authority {
        AdlSkillCapabilityAuthorityV1::ReadOnly => 0,
        AdlSkillCapabilityAuthorityV1::WorkspaceWrite => 1,
        AdlSkillCapabilityAuthorityV1::ExternalNetwork => 2,
    }
}

fn selection_trace(
    skill: &AdlSkillV1,
    matched_capability: bool,
    authority_allowed: bool,
    missing_inputs: Vec<String>,
    undeclared_inputs: Vec<String>,
    selected: bool,
    reason: &str,
) -> AdlSkillSelectionTraceV1 {
    AdlSkillSelectionTraceV1 {
        skill_id: skill.skill_id.clone(),
        matched_capability,
        authority_allowed,
        missing_inputs,
        undeclared_inputs,
        selected,
        reason: reason.to_string(),
    }
}

fn phase(phase_id: &str, required: bool, trace_event: &str) -> AdlSkillLifecyclePhaseV1 {
    AdlSkillLifecyclePhaseV1 {
        phase_id: phase_id.to_string(),
        required,
        trace_event: trace_event.to_string(),
    }
}

fn input(input_id: &str, schema_ref: &str, required: bool) -> AdlSkillInputV1 {
    AdlSkillInputV1 {
        input_id: input_id.to_string(),
        schema_ref: schema_ref.to_string(),
        required,
    }
}

fn failure(
    failure_id: &str,
    when: &str,
    runtime_result: AdlSkillDispatchStatusV1,
) -> AdlSkillFailureModeV1 {
    AdlSkillFailureModeV1 {
        failure_id: failure_id.to_string(),
        when: when.to_string(),
        runtime_result,
    }
}

fn validate_non_claims(non_claims: &[String]) -> Result<()> {
    validate_nonempty_list(non_claims, "skill.non_claims")?;
    ensure_len_within(non_claims, "skill.non_claims")?;
    for required in [
        "does not implement the WP-11 loop runtime sibling issue",
        "does not claim autonomous skill discovery",
        "does not claim v0.92 activation beyond this bounded proof surface",
    ] {
        ensure_contains_exact(non_claims, required, "skill non-claims are incomplete")?;
    }
    Ok(())
}

fn runtime_non_claims() -> Vec<String> {
    strings(&[
        "does not implement the WP-11 loop runtime sibling issue",
        "does not claim autonomous skill discovery",
        "does not claim v0.92 activation beyond this bounded proof surface",
    ])
}

fn validate_schema_ref(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    if !value.starts_with("adl.") {
        return Err(anyhow!("{field} must be an ADL schema ref"));
    }
    Ok(())
}

fn validate_relative_ref(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    if value.starts_with('/') || value.contains("..") {
        return Err(anyhow!("{field} must be repository-relative"));
    }
    Ok(())
}

fn validate_relative_path(value: &str, field: &str) -> Result<()> {
    validate_relative_ref(value, field)?;
    if value.contains('\\') {
        return Err(anyhow!("{field} must use forward slashes"));
    }
    if Path::new(value).components().any(|component| {
        !matches!(
            component,
            Component::Normal(segment) if !segment.is_empty()
        )
    }) {
        return Err(anyhow!("{field} must use normalized path segments"));
    }
    Ok(())
}

fn validate_artifact_ref(value: &str, field: &str) -> Result<()> {
    validate_relative_path(value, field)?;
    if value
        .split('/')
        .any(|segment| matches!(segment, "." | "..") || segment.chars().all(|ch| ch == '.'))
    {
        return Err(anyhow!("{field} must reject dot path segments"));
    }
    if !value.starts_with(ADL_SKILL_V1_ARTIFACT_PREFIX) {
        return Err(anyhow!(
            "{field} must stay within {ADL_SKILL_V1_ARTIFACT_PREFIX}"
        ));
    }
    if value.len() == ADL_SKILL_V1_ARTIFACT_PREFIX.len() {
        return Err(anyhow!("{field} must name an artifact file"));
    }
    Ok(())
}

fn validate_entrypoint_ref(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    if value.starts_with('/')
        || value.contains('/')
        || value.contains('\\')
        || value.contains("..")
        || !value.starts_with("adl::")
    {
        return Err(anyhow!(
            "{field} must be an adl:: Rust module path without filesystem traversal"
        ));
    }
    let mut segments = value.split("::");
    if segments.next() != Some("adl") {
        return Err(anyhow!("{field} must start with adl::"));
    }
    for segment in segments {
        validate_rust_module_segment(segment, field)?;
    }
    Ok(())
}

fn validate_rust_module_segment(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(anyhow!("{field} must not contain empty module segments"));
    };
    if !(first.is_ascii_lowercase() || first == '_') {
        return Err(anyhow!("{field} must use Rust module identifiers"));
    }
    if chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_') {
        Ok(())
    } else {
        Err(anyhow!("{field} must use Rust module identifiers"))
    }
}

fn normalize_id(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    if value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '-' | '_' | '.'))
    {
        Ok(())
    } else {
        Err(anyhow!("{field} must use lowercase id characters"))
    }
}

fn validate_semverish_version(value: &str, field: &str) -> Result<()> {
    validate_nonempty(value, field)?;
    let parts: Vec<_> = value.split('.').collect();
    if parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
    {
        Ok(())
    } else {
        Err(anyhow!(
            "{field} must be a concrete major.minor.patch version"
        ))
    }
}

fn validate_nonempty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} must not be empty"))
    } else {
        Ok(())
    }
}

fn validate_nonempty_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_nonempty(value, field)?;
    }
    Ok(())
}

fn ensure_len_within<T>(values: &[T], field: &str) -> Result<()> {
    if values.len() > MAX_SKILL_COLLECTION_ITEMS {
        Err(anyhow!(
            "{field} must contain at most {MAX_SKILL_COLLECTION_ITEMS} entries"
        ))
    } else {
        Ok(())
    }
}

fn ensure_contains_exact(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value == needle) {
        Ok(())
    } else {
        Err(anyhow!(message.to_string()))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}'"))
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn adl_skill_v1_contract_is_stable() {
        let skill = sample_adl_skill_v1();
        skill.validate().expect("valid adl.skill.v1");

        assert_eq!(skill.schema_version, ADL_SKILL_V1_SCHEMA);
        assert_eq!(
            skill.runtime.dispatch_contract,
            ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA
        );
        assert!(skill
            .runtime
            .failure_modes
            .iter()
            .any(|mode| mode.runtime_result == AdlSkillDispatchStatusV1::RejectedSelectionFailure));
        assert!(skill
            .trace
            .required_events
            .iter()
            .any(|event| event == "skill.dispatch.completed"));
    }

    #[test]
    fn adl_skill_v1_runtime_selects_and_dispatches() {
        let proof = sample_adl_skill_v1_runtime_dispatch_proof().expect("dispatch proof");
        proof.validate().expect("valid dispatch proof");

        assert_eq!(proof.schema_version, ADL_SKILL_V1_RUNTIME_PROOF_SCHEMA);
        assert_eq!(proof.dispatch_status, AdlSkillDispatchStatusV1::Selected);
        assert_eq!(
            proof.dispatch_entrypoint,
            "adl::adl_skill_v1::sample_review_packet_builder"
        );
        assert!(proof.selection_trace.iter().any(|trace| trace.selected));
        assert!(proof
            .produced_output_refs
            .iter()
            .any(|artifact| artifact == "artifacts/adl_skill_v1/review_packet.json"));
    }

    #[test]
    fn adl_skill_v1_canonical_json_is_deterministic() {
        let mut skill = sample_adl_skill_v1();
        skill.lifecycle.phases.reverse();
        skill.inputs.reverse();
        skill.outputs.reverse();
        skill.runtime.failure_modes.reverse();

        let json = String::from_utf8(skill.pretty_json_bytes().expect("skill json")).expect("utf8");
        let reparsed: AdlSkillV1 = serde_json::from_str(&json).expect("reparse skill json");

        assert_eq!(reparsed.lifecycle.phases[0].phase_id, "bind");
        assert_eq!(
            reparsed.runtime.failure_modes[0].failure_id,
            "invalid_schema"
        );
        reparsed.validate().expect("canonical skill remains valid");
    }

    #[test]
    fn adl_skill_v1_rejects_invalid_schema() {
        let mut skill = sample_adl_skill_v1();
        skill.schema_version = "adl.skill.v0".to_string();

        assert!(skill
            .validate()
            .expect_err("invalid schema should fail")
            .to_string()
            .contains("skill.schema_version"));
    }

    #[test]
    fn adl_skill_v1_rejects_missing_required_fields() {
        let raw = json!({
            "schema_version": ADL_SKILL_V1_SCHEMA,
            "skill_id": "skill.standard.missing",
            "version": "1.0.0"
        });

        assert!(serde_json::from_value::<AdlSkillV1>(raw)
            .expect_err("missing required serde fields should fail")
            .to_string()
            .contains("missing field"));

        let mut skill = sample_adl_skill_v1();
        skill.inputs.clear();
        assert!(skill
            .validate()
            .expect_err("empty inputs should fail")
            .to_string()
            .contains("skill.inputs"));
    }

    #[test]
    fn adl_skill_v1_rejects_missing_required_lifecycle_phase() {
        let mut skill = sample_adl_skill_v1();
        skill
            .lifecycle
            .phases
            .retain(|phase| phase.phase_id != "commit");

        assert!(skill
            .validate()
            .expect_err("missing commit phase should fail")
            .to_string()
            .contains("missing required phase"));
    }

    #[test]
    fn adl_skill_v1_rejects_missing_runtime_failure_mode() {
        let mut skill = sample_adl_skill_v1();
        skill
            .runtime
            .failure_modes
            .retain(|mode| mode.runtime_result != AdlSkillDispatchStatusV1::RejectedInvalidSchema);

        assert!(skill
            .validate()
            .expect_err("missing invalid-schema failure mode should fail")
            .to_string()
            .contains("required negative cases"));
    }

    #[test]
    fn adl_skill_v1_rejects_capability_required_inputs_not_declared() {
        let mut skill = sample_adl_skill_v1();
        skill.capabilities[0]
            .required_inputs
            .push("ghost_ref".to_string());

        assert!(skill
            .validate()
            .expect_err("capability cannot require undeclared inputs")
            .to_string()
            .contains("requires undeclared input"));
    }

    #[test]
    fn adl_skill_v1_rejects_duplicate_runtime_failure_mode_ids() {
        let mut skill = sample_adl_skill_v1();
        skill.runtime.failure_modes[1].failure_id =
            skill.runtime.failure_modes[0].failure_id.clone();

        assert!(skill
            .validate()
            .expect_err("duplicate failure mode id should fail")
            .to_string()
            .contains("duplicated"));
    }

    #[test]
    fn adl_skill_v1_rejects_partial_required_context_field_match() {
        let mut skill = sample_adl_skill_v1();
        skill.invocation.required_context_fields = strings(&[
            "run_id",
            "invocation_id",
            "artifact_root",
            "trace_correlation_id_extra",
        ]);

        assert!(skill
            .validate()
            .expect_err("partial context field should fail")
            .to_string()
            .contains("missing required field"));
    }

    #[test]
    fn adl_skill_v1_rejects_version_mismatch() {
        let mut skill = sample_adl_skill_v1();
        skill.version = "1".to_string();

        assert!(skill
            .validate()
            .expect_err("version mismatch should fail")
            .to_string()
            .contains("major.minor.patch"));
    }

    #[test]
    fn adl_skill_v1_runtime_selection_failure_is_explicit() {
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![sample_adl_skill_v1()]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.requested_capability = "review.packet.delete".to_string();

        let err = catalog
            .dispatch(request)
            .expect_err("unmatched capability should fail");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_rejects_missing_selection_inputs() {
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![sample_adl_skill_v1()]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.input_refs = strings(&["issue_ref"]);

        let err = catalog
            .dispatch(request)
            .expect_err("missing capability inputs should fail");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_rejects_undeclared_request_inputs() {
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![sample_adl_skill_v1()]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.input_refs.push("undeclared_ref".to_string());

        let err = catalog
            .dispatch(request)
            .expect_err("undeclared request input should fail");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_requires_exact_authority_match() {
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![sample_adl_skill_v1()]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.required_authority = AdlSkillCapabilityAuthorityV1::ExternalNetwork;

        let err = catalog
            .dispatch(request)
            .expect_err("authority mismatch should fail");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_rejects_lower_authority_skill_for_higher_request() {
        let mut skill = sample_adl_skill_v1();
        skill.capabilities[0].authority = AdlSkillCapabilityAuthorityV1::ReadOnly;
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![skill]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.required_authority = AdlSkillCapabilityAuthorityV1::WorkspaceWrite;

        let err = catalog
            .dispatch(request)
            .expect_err("lower-authority skill should not satisfy higher request");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_rejects_higher_authority_skill_for_lower_request() {
        let mut skill = sample_adl_skill_v1();
        skill.capabilities[0].authority = AdlSkillCapabilityAuthorityV1::ExternalNetwork;
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![skill]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.required_authority = AdlSkillCapabilityAuthorityV1::WorkspaceWrite;

        let err = catalog
            .dispatch(request)
            .expect_err("higher-authority skill should not satisfy lower request");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_runtime_proof_rejects_mismatched_selected_trace() {
        let mut proof = sample_adl_skill_v1_runtime_dispatch_proof().expect("dispatch proof");
        proof.selection_trace[0].skill_id = "skill.standard.other".to_string();

        assert!(proof
            .validate()
            .expect_err("selected trace must match selected skill")
            .to_string()
            .contains("selected trace"));
    }

    #[test]
    fn adl_skill_v1_runtime_proof_rejects_unrequested_consumed_inputs() {
        let mut proof = sample_adl_skill_v1_runtime_dispatch_proof().expect("dispatch proof");
        proof.consumed_input_refs.push("ghost_ref".to_string());

        assert!(proof
            .validate()
            .expect_err("consumed inputs must be requested")
            .to_string()
            .contains("consumed inputs"));
    }

    #[test]
    fn adl_skill_v1_runtime_rejects_authority_downgrade_to_read_only() {
        let mut skill = sample_adl_skill_v1();
        skill.capabilities[0].authority = AdlSkillCapabilityAuthorityV1::WorkspaceWrite;
        let catalog = AdlSkillRuntimeCatalogV1::new(vec![skill]).expect("catalog");
        let mut request = sample_runtime_request_v1();
        request.required_authority = AdlSkillCapabilityAuthorityV1::ReadOnly;

        let err = catalog
            .dispatch(request)
            .expect_err("workspace-write skill should not satisfy read-only request");
        assert!(err.to_string().contains("runtime selection failed"));
    }

    #[test]
    fn adl_skill_v1_rejects_artifact_path_traversal() {
        let mut skill = sample_adl_skill_v1();
        skill.outputs[0].artifact_ref = "artifacts/adl_skill_v1/../review_packet.json".to_string();

        assert!(skill
            .validate()
            .expect_err("path traversal should fail")
            .to_string()
            .contains("repository-relative"));
    }

    #[test]
    fn adl_skill_v1_rejects_nested_artifact_boundary_escape() {
        let mut skill = sample_adl_skill_v1();
        skill.outputs[0].artifact_ref = "artifacts/adl_skill_v1/../../../etc/passwd".to_string();

        assert!(skill
            .validate()
            .expect_err("nested path traversal should fail")
            .to_string()
            .contains("repository-relative"));
    }

    #[test]
    fn adl_skill_v1_rejects_current_dir_artifact_boundary_escape() {
        let mut skill = sample_adl_skill_v1();
        skill.outputs[0].artifact_ref = "artifacts/adl_skill_v1/./../file.json".to_string();

        assert!(skill
            .validate()
            .expect_err("dot-segment artifact path should fail")
            .to_string()
            .contains("repository-relative"));
    }

    #[test]
    fn adl_skill_v1_rejects_artifacts_outside_skill_boundary() {
        let mut skill = sample_adl_skill_v1();
        skill.outputs[0].artifact_ref = "artifacts/review_packet.json".to_string();

        assert!(skill
            .validate()
            .expect_err("artifact outside boundary should fail")
            .to_string()
            .contains(ADL_SKILL_V1_ARTIFACT_PREFIX));
    }

    #[test]
    fn adl_skill_v1_rejects_filesystem_entrypoint_paths() {
        let mut skill = sample_adl_skill_v1();
        skill.invocation.entrypoint = "../../../../../etc/passwd".to_string();

        assert!(skill
            .validate()
            .expect_err("filesystem entrypoint path should fail")
            .to_string()
            .contains("adl:: Rust module path"));
    }

    #[test]
    fn adl_skill_v1_rejects_malformed_entrypoint_segments() {
        let mut skill = sample_adl_skill_v1();
        skill.invocation.entrypoint = "adl::module::..::other".to_string();

        assert!(skill
            .validate()
            .expect_err("malformed module segment should fail")
            .to_string()
            .contains("Rust module path"));
    }

    #[test]
    fn adl_skill_v1_rejects_empty_non_claims() {
        let mut skill = sample_adl_skill_v1();
        skill.non_claims.clear();

        assert!(skill
            .validate()
            .expect_err("empty non-claims should fail")
            .to_string()
            .contains("skill.non_claims"));
    }

    #[test]
    fn adl_skill_v1_rejects_partial_non_claims() {
        let mut skill = sample_adl_skill_v1();
        skill.non_claims[0] =
            "prefix does not implement the WP-11 loop runtime sibling issue suffix".to_string();

        assert!(skill
            .validate()
            .expect_err("partial non-claim should fail")
            .to_string()
            .contains("skill non-claims"));
    }

    #[test]
    fn adl_skill_v1_rejects_empty_terminal_states() {
        let mut skill = sample_adl_skill_v1();
        skill.lifecycle.terminal_states.clear();

        assert!(skill
            .validate()
            .expect_err("empty terminal states should fail")
            .to_string()
            .contains("skill.lifecycle.terminal_states"));
    }

    #[test]
    fn adl_skill_v1_rejects_oversized_runtime_failure_modes() {
        let mut skill = sample_adl_skill_v1();
        let template = skill.runtime.failure_modes[0].clone();
        while skill.runtime.failure_modes.len() <= MAX_SKILL_COLLECTION_ITEMS {
            let mut mode = template.clone();
            mode.failure_id = format!("extra_failure_{}", skill.runtime.failure_modes.len());
            skill.runtime.failure_modes.push(mode);
        }

        assert!(skill
            .validate()
            .expect_err("oversized failure mode list should fail")
            .to_string()
            .contains("at most"));
    }

    #[test]
    fn adl_skill_v1_catalog_rejects_ambiguous_capability_ids() {
        let first = sample_adl_skill_v1();
        let mut second = sample_adl_skill_v1();
        second.skill_id = "skill.standard.review_packet_builder_duplicate".to_string();

        assert!(AdlSkillRuntimeCatalogV1::new(vec![first, second])
            .expect_err("duplicate catalog capability should fail")
            .to_string()
            .contains("ambiguous"));
    }
}
