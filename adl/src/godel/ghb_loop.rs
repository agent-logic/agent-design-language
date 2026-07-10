//! Godel-Hadamard-Bayes recursive self-improvement proof loop.
//!
//! This module composes the existing bounded Godel stage loop, Runtime v2
//! reasoning graph and loop-runtime contracts, ObsMem index artifacts, agent
//! standing/admission policy, and the per-agent Godel snapshot/diff protocol
//! into one reviewable proof surface.

use super::{GodelStageLoopExecutor, StageLoopConfig, StageLoopInput, StageLoopPersistenceResult};
use crate::csm_godel_snapshot::{prove_godel_snapshot_diff, GodelSnapshotProofOptions};
use crate::runtime_v2::{
    runtime_v2_loop_runtime_contract, runtime_v2_reasoning_graph_contract, RuntimeV2StandingPolicy,
};
use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub const GHB_PROOF_SCHEMA: &str = "adl.godel.ghb_recursive_self_improvement_proof.v1";
pub const GHB_CYCLE_SCHEMA: &str = "adl.godel.ghb_cycle.v1";
pub const GHB_REPLAY_SCHEMA: &str = "adl.godel.ghb_replay_validation.v1";

#[derive(Debug, Clone)]
pub struct GhbProofOptions {
    pub out_dir: PathBuf,
    pub run_id: String,
    pub admitted_task: String,
    pub local_provider_route: String,
    pub remote_provider_route: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbProofReport {
    pub schema: String,
    pub run_id: String,
    pub generated_at: DateTime<Utc>,
    pub local_cycle_ref: String,
    pub remote_cycle_ref: String,
    pub replay_validation_ref: String,
    pub snapshot_proof_ref: String,
    pub local_cycle: GhbCycleSummary,
    pub remote_cycle: GhbCycleSummary,
    pub replay: GhbReplayValidation,
    pub negative_cases: Vec<GhbNegativeCase>,
    pub validation_commands: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbCyclePacket {
    pub schema: String,
    pub cycle_id: String,
    pub agent_instance_id: String,
    pub agent_standing_before: String,
    pub agent_standing_after: String,
    pub execution_class: String,
    pub provider_route: String,
    pub provider_execution_status: String,
    pub reasoning_graph_id: String,
    pub loop_runtime_id: String,
    pub standing_policy_id: String,
    pub snapshot_chain_ref: String,
    pub admitted_task: String,
    pub admission: GhbAdmissionDecision,
    pub hypothesis_classification: String,
    pub phases: Vec<GhbPhaseRecord>,
    pub state_space_compression: GhbStateCompression,
    pub selected_update: GhbSelectedUpdate,
    pub rejected_alternatives: Vec<GhbRejectedAlternative>,
    pub governance: GhbGovernanceDecision,
    pub artifact_refs: GhbArtifactRefs,
    pub observability: GhbObservability,
    pub deterministic_replay_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbCycleSummary {
    pub cycle_id: String,
    pub execution_class: String,
    pub provider_route: String,
    pub provider_execution_status: String,
    pub artifact_ref: String,
    pub selected_update_id: String,
    pub governance_status: String,
    pub replay_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbAdmissionDecision {
    pub status: String,
    pub intent_classification: String,
    pub standing_class: String,
    pub admitted: bool,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbPhaseRecord {
    pub phase: String,
    pub temporal_anchor: String,
    pub input_refs: Vec<String>,
    pub output_refs: Vec<String>,
    pub decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbStateCompression {
    pub input_microstate_ref: String,
    pub compressed_macrostate_ref: String,
    pub macrostate_transformation: String,
    pub observable_projection_ref: String,
    pub selected_durable_state_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbSelectedUpdate {
    pub update_id: String,
    pub update_kind: String,
    pub changed_state_refs: Vec<String>,
    pub accepted_because: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbRejectedAlternative {
    pub alternative_id: String,
    pub rejection_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbGovernanceDecision {
    pub status: String,
    pub source_code_mutation_allowed: bool,
    pub prompt_persistence_allowed: bool,
    pub credential_capture_allowed: bool,
    pub review_required_for_promotion: bool,
    pub denial_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbArtifactRefs {
    pub reasoning_graph_ref: String,
    pub loop_runtime_ref: String,
    pub godel_hypothesis_ref: String,
    pub godel_mutation_ref: String,
    pub godel_evaluation_ref: String,
    pub godel_experiment_record_ref: String,
    pub obsmem_index_ref: String,
    pub snapshot_chain_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbObservability {
    pub trace_id: String,
    pub span_id: String,
    pub phase_timing_refs: Vec<String>,
    pub provider_route: String,
    pub failure_class: String,
    pub redacted_artifact_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbReplayValidation {
    pub schema: String,
    pub status: String,
    pub local_replay_hash: String,
    pub remote_replay_hash: String,
    pub deterministic_inputs: Vec<String>,
    pub replay_guarantees: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GhbNegativeCase {
    pub case_id: String,
    pub status: String,
    pub expected_error_contains: String,
    pub observed_error: String,
}

pub fn prove_ghb_recursive_self_improvement(options: GhbProofOptions) -> Result<GhbProofReport> {
    validate_proof_options(&options)?;
    prepare_proof_dir(&options.out_dir)?;

    let reasoning_graph = runtime_v2_reasoning_graph_contract()?;
    let loop_runtime = runtime_v2_loop_runtime_contract()?;
    let standing = RuntimeV2StandingPolicy::prototype()?;
    let snapshot_proof_dir = options.out_dir.join("snapshot-proof");
    let snapshot_proof = prove_godel_snapshot_diff(GodelSnapshotProofOptions {
        out_dir: snapshot_proof_dir,
        spec_path: None,
        run_id: format!("{}-snapshot", options.run_id),
    })?;

    let local = build_cycle(
        &options,
        &reasoning_graph.graph_id,
        &loop_runtime.runtime_id,
        &standing.policy_id,
        &snapshot_proof.positive_case.chain_ref,
        "local",
        &options.local_provider_route,
    )?;
    let remote = build_cycle(
        &options,
        &reasoning_graph.graph_id,
        &loop_runtime.runtime_id,
        &standing.policy_id,
        &snapshot_proof.positive_case.chain_ref,
        "remote",
        &options.remote_provider_route,
    )?;

    let local_ref = "ghb/local_cycle.v1.json".to_string();
    let remote_ref = "ghb/remote_cycle.v1.json".to_string();
    write_json(&options.out_dir.join(&local_ref), &local)?;
    write_json(&options.out_dir.join(&remote_ref), &remote)?;

    let replay = GhbReplayValidation {
        schema: GHB_REPLAY_SCHEMA.to_string(),
        status: if replay_signature_without_provider(&local)
            == replay_signature_without_provider(&remote)
        {
            "deterministic_comparable_replay".to_string()
        } else {
            "failed".to_string()
        },
        local_replay_hash: local.deterministic_replay_hash.clone(),
        remote_replay_hash: remote.deterministic_replay_hash.clone(),
        deterministic_inputs: vec![
            "admitted_task".to_string(),
            "reasoning_graph_id".to_string(),
            "loop_runtime_id".to_string(),
            "standing_policy_id".to_string(),
            "godel_stage_order".to_string(),
            "snapshot_chain_ref".to_string(),
        ],
        replay_guarantees: vec![
            "Godel/Hadamard/Bayes/persistence/governance phases are ordered deterministically"
                .to_string(),
            "speculative alternatives remain rejected until governance admits a selected update"
                .to_string(),
            "provider route changes execution class metadata without changing phase graph"
                .to_string(),
            "snapshot last-known-good pointer is validated before durable state is selected"
                .to_string(),
        ],
    };
    if replay.status != "deterministic_comparable_replay" {
        bail!("GHB replay validation failed");
    }
    let replay_ref = "ghb/replay_validation.v1.json".to_string();
    write_json(&options.out_dir.join(&replay_ref), &replay)?;

    let negative_cases = negative_cases(&options, &local)?;
    let report = GhbProofReport {
        schema: GHB_PROOF_SCHEMA.to_string(),
        run_id: options.run_id.clone(),
        generated_at: Utc::now(),
        local_cycle_ref: local_ref.clone(),
        remote_cycle_ref: remote_ref.clone(),
        replay_validation_ref: replay_ref.clone(),
        snapshot_proof_ref: "snapshot-proof/godel_snapshot_diff_proof.json".to_string(),
        local_cycle: summarize_cycle(&local, local_ref),
        remote_cycle: summarize_cycle(&remote, remote_ref),
        replay,
        negative_cases,
        validation_commands: vec![
            "cargo test --manifest-path adl/Cargo.toml ghb_loop -- --nocapture".to_string(),
            "cargo test --manifest-path adl/Cargo.toml --test cli_smoke godel::godel_ghb_proof_executes_local_and_remote_cycles -- --exact --nocapture".to_string(),
            "adl godel ghb-proof --out <proof-dir> --json".to_string(),
            "git diff --check".to_string(),
        ],
        non_claims: vec![
            "not_unbounded_recursive_self_improvement".to_string(),
            "not_source_code_mutation_without_review".to_string(),
            "not_credential_capture".to_string(),
            "not_private_prompt_persistence".to_string(),
            "not_live_hosted_provider_invocation".to_string(),
            "not_v092_birthday_completion_by_itself".to_string(),
        ],
    };
    write_json(
        &options.out_dir.join("ghb/ghb_proof_report.v1.json"),
        &report,
    )?;
    Ok(report)
}

fn build_cycle(
    options: &GhbProofOptions,
    reasoning_graph_id: &str,
    loop_runtime_id: &str,
    standing_policy_id: &str,
    snapshot_chain_ref: &str,
    execution_class: &str,
    provider_route: &str,
) -> Result<GhbCyclePacket> {
    let cycle_id = format!("{}-{execution_class}", options.run_id);
    let runs_root = options.out_dir.join(format!("{execution_class}-runs"));
    let stage = GodelStageLoopExecutor::new(StageLoopConfig::default()).execute_and_persist(
        &StageLoopInput {
            run_id: cycle_id.clone(),
            workflow_id: "wf-ghb-recursive-self-improvement".to_string(),
            failure_code: "bounded_self_improvement_request".to_string(),
            failure_summary: options.admitted_task.clone(),
            evidence_refs: vec![
                "runtime_v2/reasoning_graph/reasoning_graph.json".to_string(),
                "runtime_v2/loop_runtime/loop_runtime.json".to_string(),
                snapshot_chain_ref.to_string(),
            ],
        },
        &runs_root,
    )?;
    let update_id = format!("ghb-update-{execution_class}-0001");
    let phases = phase_records(&cycle_id, &stage, reasoning_graph_id, loop_runtime_id);
    let packet = GhbCyclePacket {
        schema: GHB_CYCLE_SCHEMA.to_string(),
        cycle_id: cycle_id.clone(),
        agent_instance_id: "godel-agent-5096".to_string(),
        agent_standing_before: "candidate".to_string(),
        agent_standing_after: "provisional".to_string(),
        execution_class: execution_class.to_string(),
        provider_route: provider_route.to_string(),
        provider_execution_status: provider_execution_status(execution_class).to_string(),
        reasoning_graph_id: reasoning_graph_id.to_string(),
        loop_runtime_id: loop_runtime_id.to_string(),
        standing_policy_id: standing_policy_id.to_string(),
        snapshot_chain_ref: snapshot_chain_ref.to_string(),
        admitted_task: options.admitted_task.clone(),
        admission: GhbAdmissionDecision {
            status: "admitted".to_string(),
            intent_classification: "planning".to_string(),
            standing_class: "candidate".to_string(),
            admitted: true,
            rationale: format!(
                "standing policy {standing_policy_id} admits bounded planning self-improvement with governance"
            ),
        },
        hypothesis_classification: "planning".to_string(),
        phases,
        state_space_compression: GhbStateCompression {
            input_microstate_ref: format!("ghb/{execution_class}/microstate.input.v1.json"),
            compressed_macrostate_ref: format!("ghb/{execution_class}/macrostate.compressed.v1.json"),
            macrostate_transformation:
                "Godel expands current truth, Hadamard explores alternatives, Bayes selects the durable compressed state"
                    .to_string(),
            observable_projection_ref: stage.canonical_evidence_rel_path.display().to_string(),
            selected_durable_state_ref: snapshot_chain_ref.to_string(),
        },
        selected_update: GhbSelectedUpdate {
            update_id: update_id.clone(),
            update_kind: "durable_cognitive_macrostate".to_string(),
            changed_state_refs: vec![
                stage.canonical_mutation_rel_path.display().to_string(),
                stage.canonical_experiment_record_rel_path.display().to_string(),
                snapshot_chain_ref.to_string(),
            ],
            accepted_because:
                "Bayes phase selected the bounded mutation after evidence scoring and governance review"
                    .to_string(),
        },
        rejected_alternatives: vec![
            GhbRejectedAlternative {
                alternative_id: "unsafe-source-mutation".to_string(),
                rejection_reason: "source code mutation requires separate human review".to_string(),
            },
            GhbRejectedAlternative {
                alternative_id: "raw-private-prompt-persistence".to_string(),
                rejection_reason: "private prompt material cannot be persisted in GHB artifacts".to_string(),
            },
        ],
        governance: GhbGovernanceDecision {
            status: "accepted_with_review_boundary".to_string(),
            source_code_mutation_allowed: false,
            prompt_persistence_allowed: false,
            credential_capture_allowed: false,
            review_required_for_promotion: true,
            denial_refs: vec![
                "unsafe-source-mutation".to_string(),
                "raw-private-prompt-persistence".to_string(),
            ],
        },
        artifact_refs: GhbArtifactRefs {
            reasoning_graph_ref: "runtime_v2/reasoning_graph/reasoning_graph.json".to_string(),
            loop_runtime_ref: "runtime_v2/loop_runtime/loop_runtime.json".to_string(),
            godel_hypothesis_ref: stage.hypothesis_rel_path.display().to_string(),
            godel_mutation_ref: stage.canonical_mutation_rel_path.display().to_string(),
            godel_evaluation_ref: stage.canonical_evaluation_plan_rel_path.display().to_string(),
            godel_experiment_record_ref: stage
                .canonical_experiment_record_rel_path
                .display()
                .to_string(),
            obsmem_index_ref: stage.obsmem_index_rel_path.display().to_string(),
            snapshot_chain_ref: snapshot_chain_ref.to_string(),
        },
        observability: GhbObservability {
            trace_id: format!("trace-ghb-{execution_class}-5096"),
            span_id: format!("span-ghb-{execution_class}-0001"),
            phase_timing_refs: vec![
                format!("ghb/{execution_class}/timing/godel.json"),
                format!("ghb/{execution_class}/timing/hadamard.json"),
                format!("ghb/{execution_class}/timing/bayes.json"),
                format!("ghb/{execution_class}/timing/persistence.json"),
                format!("ghb/{execution_class}/timing/governance.json"),
            ],
            provider_route: provider_route.to_string(),
            failure_class: if execution_class == "remote" {
                "provider_route_classified_not_invoked".to_string()
            } else {
                "none".to_string()
            },
            redacted_artifact_refs: vec![
                stage.canonical_evidence_rel_path.display().to_string(),
                stage.obsmem_index_rel_path.display().to_string(),
            ],
        },
        deterministic_replay_hash: String::new(),
    };
    let mut packet = packet;
    packet.deterministic_replay_hash = replay_hash(&packet);
    validate_cycle(&packet)?;
    Ok(packet)
}

fn phase_records(
    cycle_id: &str,
    stage: &StageLoopPersistenceResult,
    reasoning_graph_id: &str,
    loop_runtime_id: &str,
) -> Vec<GhbPhaseRecord> {
    vec![
        phase(
            "godel",
            cycle_id,
            &["current_truth", reasoning_graph_id],
            &[stage.hypothesis_rel_path.display().to_string()],
            "represented current truth, constraints, evidence, and goal context",
        ),
        phase(
            "hadamard",
            cycle_id,
            &[loop_runtime_id, "bounded_alternative_generation"],
            &[stage.policy_comparison_rel_path.display().to_string()],
            "generated bounded replayable alternatives without acting on them",
        ),
        phase(
            "bayes",
            cycle_id,
            &[stage
                .canonical_evaluation_plan_rel_path
                .display()
                .to_string()],
            &[stage.promotion_decision_rel_path.display().to_string()],
            "scored evidence and selected the safe durable update",
        ),
        phase(
            "persistence",
            cycle_id,
            &[stage.canonical_mutation_rel_path.display().to_string()],
            &[
                stage
                    .canonical_experiment_record_rel_path
                    .display()
                    .to_string(),
                stage.obsmem_index_rel_path.display().to_string(),
            ],
            "persisted trace, ObsMem, experiment, and snapshot-linked state",
        ),
        phase(
            "governance",
            cycle_id,
            &["standing_policy", "mutation_boundary"],
            &["ghb/governance_decision.v1.json"],
            "blocked uncontrolled mutation, credentials, and prompt persistence",
        ),
    ]
}

fn phase(
    phase: &str,
    cycle_id: &str,
    input_refs: &[impl ToString],
    output_refs: &[impl ToString],
    decision: &str,
) -> GhbPhaseRecord {
    GhbPhaseRecord {
        phase: phase.to_string(),
        temporal_anchor: format!("{cycle_id}:{phase}:0001"),
        input_refs: input_refs.iter().map(ToString::to_string).collect(),
        output_refs: output_refs.iter().map(ToString::to_string).collect(),
        decision: decision.to_string(),
    }
}

fn summarize_cycle(packet: &GhbCyclePacket, artifact_ref: String) -> GhbCycleSummary {
    GhbCycleSummary {
        cycle_id: packet.cycle_id.clone(),
        execution_class: packet.execution_class.clone(),
        provider_route: packet.provider_route.clone(),
        provider_execution_status: packet.provider_execution_status.clone(),
        artifact_ref,
        selected_update_id: packet.selected_update.update_id.clone(),
        governance_status: packet.governance.status.clone(),
        replay_hash: packet.deterministic_replay_hash.clone(),
    }
}

fn validate_proof_options(options: &GhbProofOptions) -> Result<()> {
    validate_id(&options.run_id, "run_id")?;
    if options.admitted_task.trim().is_empty() {
        bail!("GHB admitted task must be non-empty");
    }
    if !options.local_provider_route.starts_with("local:") {
        bail!("local provider route must start with local:");
    }
    if !options.remote_provider_route.starts_with("hosted:") {
        bail!("remote provider route must start with hosted:");
    }
    Ok(())
}

fn validate_cycle(packet: &GhbCyclePacket) -> Result<()> {
    if packet.schema != GHB_CYCLE_SCHEMA {
        bail!("unsupported GHB cycle schema '{}'", packet.schema);
    }
    if packet.admission.status != "admitted" || !packet.admission.admitted {
        bail!("GHB cycle must be admitted before execution");
    }
    let expected = ["godel", "hadamard", "bayes", "persistence", "governance"];
    let actual: Vec<&str> = packet
        .phases
        .iter()
        .map(|phase| phase.phase.as_str())
        .collect();
    if actual != expected {
        bail!("GHB phases must be ordered godel->hadamard->bayes->persistence->governance");
    }
    if packet
        .phases
        .iter()
        .any(|phase| phase.temporal_anchor.trim().is_empty())
    {
        bail!("GHB every phase requires a deterministic temporal anchor");
    }
    if packet.governance.source_code_mutation_allowed
        || packet.governance.prompt_persistence_allowed
        || packet.governance.credential_capture_allowed
    {
        bail!(
            "GHB governance must deny unsafe mutation, prompt persistence, and credential capture"
        );
    }
    if packet
        .state_space_compression
        .selected_durable_state_ref
        .trim()
        .is_empty()
    {
        bail!("GHB selected durable state ref is required");
    }
    if packet.governance.denial_refs.is_empty() {
        bail!("GHB governance denial refs are required for unsafe alternatives");
    }
    if packet.rejected_alternatives.is_empty() {
        bail!("GHB speculative alternatives must remain rejected until governance admits a selected update");
    }
    if packet.agent_standing_before.trim().is_empty()
        || packet.agent_standing_after.trim().is_empty()
    {
        bail!("GHB agent standing before/after must remain explicit across persistence");
    }
    if packet.provider_execution_status.trim().is_empty() {
        bail!("GHB provider execution status is required");
    }
    if packet.execution_class == "remote"
        && packet.provider_execution_status != "classified_hosted_route_not_invoked"
    {
        bail!("GHB remote proof must not claim live hosted provider invocation");
    }
    if packet.reasoning_graph_id.trim().is_empty()
        || packet.loop_runtime_id.trim().is_empty()
        || packet.standing_policy_id.trim().is_empty()
        || packet.snapshot_chain_ref.trim().is_empty()
    {
        bail!("GHB deterministic bindings are required");
    }
    Ok(())
}

fn replay_hash(packet: &GhbCyclePacket) -> String {
    let signature = replay_signature_without_provider(packet);
    let mut hasher = Sha256::new();
    hasher.update(signature.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn replay_signature_without_provider(packet: &GhbCyclePacket) -> String {
    let phases = packet
        .phases
        .iter()
        .map(|phase| format!("{}:{}", phase.phase, phase.decision))
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        packet.admitted_task,
        packet.admission.intent_classification,
        packet.admission.rationale,
        packet.reasoning_graph_id,
        packet.loop_runtime_id,
        packet.standing_policy_id,
        packet.snapshot_chain_ref,
        phases,
        packet.state_space_compression.macrostate_transformation,
        packet.governance.status
    )
}

fn provider_execution_status(execution_class: &str) -> &'static str {
    if execution_class == "remote" {
        "classified_hosted_route_not_invoked"
    } else {
        "local_stage_executor_invoked"
    }
}

fn negative_cases(
    options: &GhbProofOptions,
    valid_cycle: &GhbCyclePacket,
) -> Result<Vec<GhbNegativeCase>> {
    let mut cases = Vec::new();
    cases.push(expect_error(
        "unadmitted_task",
        "admitted task must be non-empty",
        || {
            let mut bad = options.clone();
            bad.admitted_task.clear();
            validate_proof_options(&bad)
        },
    ));
    cases.push(expect_error(
        "unsafe_run_id",
        "run_id must be a safe id",
        || {
            let mut bad = options.clone();
            bad.run_id = "../bad".to_string();
            validate_proof_options(&bad)
        },
    ));
    cases.push(expect_error(
        "local_provider_mismatch",
        "local provider route",
        || {
            let mut bad = options.clone();
            bad.local_provider_route = "hosted:bedrock/nova-pro".to_string();
            validate_proof_options(&bad)
        },
    ));
    cases.push(expect_error(
        "provider_mismatch",
        "remote provider route",
        || {
            let mut bad = options.clone();
            bad.remote_provider_route = "local:not-remote".to_string();
            validate_proof_options(&bad)
        },
    ));
    cases.push(expect_error(
        "unsafe_mutation",
        "governance must deny unsafe mutation",
        || {
            let mut bad = valid_cycle.clone();
            bad.governance.source_code_mutation_allowed = true;
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "malformed_alternative",
        "phases must be ordered",
        || {
            let mut bad = valid_cycle.clone();
            bad.phases.swap(0, 1);
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "missing_temporal_anchor",
        "temporal anchor",
        || {
            let mut bad = valid_cycle.clone();
            bad.phases[0].temporal_anchor.clear();
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "missing_durable_state",
        "durable state ref",
        || {
            let mut bad = valid_cycle.clone();
            bad.state_space_compression
                .selected_durable_state_ref
                .clear();
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "speculation_to_execution_collapse",
        "speculative alternatives",
        || {
            let mut bad = valid_cycle.clone();
            bad.rejected_alternatives.clear();
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error("identity_discontinuity", "standing", || {
        let mut bad = valid_cycle.clone();
        bad.agent_standing_after.clear();
        validate_cycle(&bad)
    }));
    cases.push(expect_error(
        "constraint_underspecification",
        "governance denial refs",
        || {
            let mut bad = valid_cycle.clone();
            bad.governance.denial_refs.clear();
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "hosted_execution_overclaim",
        "must not claim live hosted provider invocation",
        || {
            let mut bad = valid_cycle.clone();
            bad.execution_class = "remote".to_string();
            bad.provider_execution_status = "live_hosted_provider_invoked".to_string();
            validate_cycle(&bad)
        },
    ));
    cases.push(expect_error(
        "missing_deterministic_binding",
        "deterministic bindings",
        || {
            let mut bad = valid_cycle.clone();
            bad.reasoning_graph_id.clear();
            validate_cycle(&bad)
        },
    ));

    let failed: Vec<String> = cases
        .iter()
        .filter(|case| case.status != "passed")
        .map(|case| format!("{}: {}", case.case_id, case.observed_error))
        .collect();
    if !failed.is_empty() {
        bail!("GHB negative-case validation failed: {}", failed.join("; "));
    }
    Ok(cases)
}

fn expect_error<F>(case_id: &str, expected: &str, f: F) -> GhbNegativeCase
where
    F: FnOnce() -> Result<()>,
{
    match f() {
        Ok(()) => GhbNegativeCase {
            case_id: case_id.to_string(),
            status: "failed".to_string(),
            expected_error_contains: expected.to_string(),
            observed_error: "validator unexpectedly accepted invalid input".to_string(),
        },
        Err(err) => {
            let observed = err.to_string();
            let status = if observed.contains(expected) {
                "passed"
            } else {
                "failed"
            };
            GhbNegativeCase {
                case_id: case_id.to_string(),
                status: status.to_string(),
                expected_error_contains: expected.to_string(),
                observed_error: observed,
            }
        }
    }
}

fn validate_id(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('-')
        || value.contains('/')
        || value.contains('\\')
        || value.contains(':')
        || value.contains("..")
    {
        return Err(anyhow!("{field} must be a safe id"));
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create parent '{}'", parent.display()))?;
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(value).context("serialize GHB proof JSON")?,
    )
    .with_context(|| format!("write '{}'", path.display()))
}

fn prepare_proof_dir(out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("create GHB proof dir '{}'", out_dir.display()))?;
    for owned_child in ["ghb", "snapshot-proof", "local-runs", "remote-runs"] {
        let path = out_dir.join(owned_child);
        if path.exists() {
            fs::remove_dir_all(&path)
                .with_context(|| format!("clean stale GHB proof child '{}'", path.display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("adl-ghb-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn ghb_loop_proves_local_and_remote_comparable_cycles() {
        let out = temp_dir("proof");
        let report = prove_ghb_recursive_self_improvement(GhbProofOptions {
            out_dir: out.clone(),
            run_id: "ghb-proof-5096".to_string(),
            admitted_task: "Improve a bounded review plan without source mutation".to_string(),
            local_provider_route: "local:ollama/qwen".to_string(),
            remote_provider_route: "hosted:bedrock/nova-pro".to_string(),
        })
        .expect("GHB proof");
        assert_eq!(report.schema, GHB_PROOF_SCHEMA);
        assert_eq!(report.local_cycle.execution_class, "local");
        assert_eq!(report.remote_cycle.execution_class, "remote");
        assert_eq!(report.replay.status, "deterministic_comparable_replay");
        assert!(out.join("ghb/local_cycle.v1.json").is_file());
        assert!(out.join("ghb/remote_cycle.v1.json").is_file());
        assert!(out
            .join("snapshot-proof/godel_snapshot_diff_proof.json")
            .is_file());
    }

    #[test]
    fn ghb_loop_rejects_remote_provider_mismatch() {
        let err = prove_ghb_recursive_self_improvement(GhbProofOptions {
            out_dir: temp_dir("bad-provider"),
            run_id: "ghb-proof-5096".to_string(),
            admitted_task: "Improve a bounded review plan".to_string(),
            local_provider_route: "local:ollama/qwen".to_string(),
            remote_provider_route: "local:not-remote".to_string(),
        })
        .expect_err("remote route must be hosted");
        assert!(err
            .to_string()
            .contains("remote provider route must start with hosted:"));
    }
}
