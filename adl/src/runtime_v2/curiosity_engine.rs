//! Runtime-v2 Curiosity Engine contract for v0.91.7 WP-10.
//!
//! This module owns the governed curiosity core that later CSM runtime
//! components can host. It produces bounded discovery proposals from explicit
//! signals, routes them through governance handoffs, and rejects unbounded or
//! ungated curiosity before activation.

use super::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

pub const RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA: &str = "runtime_v2.curiosity_engine.v1";
pub const RUNTIME_V2_CURIOSITY_ENGINE_PATH: &str =
    "runtime_v2/curiosity_engine/curiosity_engine.json";
pub const RUNTIME_V2_CURIOSITY_ENGINE_FEATURE_DOC: &str =
    "docs/milestones/v0.91.7/features/CURIOSITY_ENGINE_DISCOVERY_SUBSTRATE_v0.91.7.md";
pub const RUNTIME_V2_CURIOSITY_ENGINE_TEST_MARKER: &str = "runtime_v2_curiosity_engine";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriosityEnginePacket {
    pub schema_version: String,
    pub engine_id: String,
    pub milestone: String,
    pub wp: String,
    pub artifact_path: String,
    pub source_feature_doc: String,
    pub runtime_module_ref: String,
    pub future_component_refs: Vec<String>,
    pub budget: RuntimeV2CuriosityBudget,
    pub signals: Vec<RuntimeV2CuriositySignal>,
    pub proposals: Vec<RuntimeV2CuriosityProposal>,
    pub governance: RuntimeV2CuriosityGovernance,
    pub handoff: RuntimeV2CuriosityHandoff,
    pub validation_commands: Vec<String>,
    pub claim_boundary: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriosityBudget {
    pub max_open_questions: u32,
    pub max_proposals_per_cycle: u32,
    pub max_experiment_steps: u32,
    pub max_external_actions: u32,
    pub exhaustion_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriositySignal {
    pub signal_id: String,
    pub signal_kind: RuntimeV2CuriositySignalKind,
    pub source_ref: String,
    pub novelty_score: u8,
    pub surprise_score: u8,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2CuriositySignalKind {
    RuntimeAnomaly,
    EvidenceGap,
    CapabilityDelta,
    OperatorQuestion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriosityProposal {
    pub proposal_id: String,
    pub source_signal_id: String,
    pub question: String,
    pub hypothesis: String,
    pub experiment_plan: Vec<String>,
    pub expected_artifacts: Vec<String>,
    pub gated_by: Vec<String>,
    pub status: RuntimeV2CuriosityProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeV2CuriosityProposalStatus {
    Proposed,
    BlockedByBudget,
    BlockedByGovernance,
    ReadyForReview,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriosityGovernance {
    pub freedom_gate_required: bool,
    pub constructability_gate_required: bool,
    pub cav_review_required: bool,
    pub operator_review_required_for_external_action: bool,
    pub allowed_runtime_actions: Vec<String>,
    pub prohibited_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeV2CuriosityHandoff {
    pub reasoning_graph_refs: Vec<String>,
    pub obsmem_refs: Vec<String>,
    pub trace_refs: Vec<String>,
    pub constructability_refs: Vec<String>,
    pub csm_component_followups: Vec<String>,
    pub replay_guarantees: Vec<String>,
}

impl RuntimeV2CuriosityEnginePacket {
    pub fn prototype() -> Result<Self> {
        let packet = Self {
            schema_version: RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA.to_string(),
            engine_id: "curiosity-engine-v0-91-7-wp-10".to_string(),
            milestone: "v0.91.7".to_string(),
            wp: "WP-10".to_string(),
            artifact_path: RUNTIME_V2_CURIOSITY_ENGINE_PATH.to_string(),
            source_feature_doc: RUNTIME_V2_CURIOSITY_ENGINE_FEATURE_DOC.to_string(),
            runtime_module_ref: "adl/src/runtime_v2/curiosity_engine.rs".to_string(),
            future_component_refs: vec![
                "issue-5124-curiosity-engine-csm-runtime-component".to_string(),
                "issue-5125-constructability-gate-csm-runtime-component".to_string(),
            ],
            budget: RuntimeV2CuriosityBudget {
                max_open_questions: 4,
                max_proposals_per_cycle: 2,
                max_experiment_steps: 3,
                max_external_actions: 0,
                exhaustion_policy: "defer_new_questions_and_emit_budget_exhausted_event"
                    .to_string(),
            },
            signals: prototype_signals(),
            proposals: prototype_proposals(),
            governance: RuntimeV2CuriosityGovernance {
                freedom_gate_required: true,
                constructability_gate_required: true,
                cav_review_required: true,
                operator_review_required_for_external_action: true,
                allowed_runtime_actions: vec![
                    "record_question".to_string(),
                    "propose_bounded_experiment".to_string(),
                    "write_reviewable_trace".to_string(),
                    "handoff_to_reasoning_graph".to_string(),
                ],
                prohibited_actions: vec![
                    "autonomous_external_execution".to_string(),
                    "private_reasoning_disclosure".to_string(),
                    "constructability_promotion_without_anchor".to_string(),
                    "freedom_gate_bypass".to_string(),
                ],
            },
            handoff: RuntimeV2CuriosityHandoff {
                reasoning_graph_refs: vec![
                    "runtime_v2/reasoning_graph/reasoning_graph.json".to_string(),
                ],
                obsmem_refs: vec!["obsmem/curiosity/curiosity-cycle-0001.json".to_string()],
                trace_refs: vec![
                    "trace://runtime_v2/curiosity/signal-gap-wp10".to_string(),
                    "trace://runtime_v2/curiosity/proposal-bounded-proof".to_string(),
                ],
                constructability_refs: vec![
                    "runtime_v2/constructability/anchor-validator.json".to_string(),
                ],
                csm_component_followups: vec![
                    "issue-5124 consumes this curiosity core as a supervised CSM component"
                        .to_string(),
                    "issue-5125 hosts the constructability gate needed before promotion"
                        .to_string(),
                ],
                replay_guarantees: vec![
                    "signals and proposals are sorted deterministically before serialization"
                        .to_string(),
                    "every proposal must cite an existing signal".to_string(),
                    "every executable proposal must be budget bounded".to_string(),
                    "every proposal must retain Freedom Gate and Constructability gates"
                        .to_string(),
                ],
            },
            validation_commands: vec![
                format!(
                    "cargo test --manifest-path adl/Cargo.toml {RUNTIME_V2_CURIOSITY_ENGINE_TEST_MARKER} -- --nocapture"
                ),
                "cargo test --manifest-path adl/Cargo.toml trace_runtime_v2_curiosity_engine -- --nocapture".to_string(),
                "adl/target/debug/adl runtime-v2 curiosity-engine --out .adl/local-artifacts/wp10-curiosity/curiosity-engine.json".to_string(),
                "git diff --check".to_string(),
            ],
            claim_boundary:
                "WP-10 #4692 proves a bounded Runtime v2 Curiosity Engine core for governed discovery-cycle proposals. It is ready for later CSM runtime-component hosting, but does not claim autonomous exploration, live external action, or WP-07A supervisor integration."
                    .to_string(),
            non_claims: vec![
                "does not perform autonomous external exploration".to_string(),
                "does not bypass Freedom Gate, CAV, operator review, or Constructability"
                    .to_string(),
                "does not claim WP-07A CSM component supervisor integration".to_string(),
                "does not promote hypotheses into shared reality without constructability anchors"
                    .to_string(),
                "does not expose private reasoning".to_string(),
            ],
        };
        packet.validate()?;
        Ok(packet)
    }

    pub fn validate(&self) -> Result<()> {
        require_exact(
            &self.schema_version,
            RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA,
            "curiosity.schema_version",
        )?;
        normalize_id(self.engine_id.clone(), "curiosity.engine_id")?;
        require_exact(&self.milestone, "v0.91.7", "curiosity.milestone")?;
        require_exact(&self.wp, "WP-10", "curiosity.wp")?;
        require_exact(
            &self.artifact_path,
            RUNTIME_V2_CURIOSITY_ENGINE_PATH,
            "curiosity.artifact_path",
        )?;
        require_exact(
            &self.source_feature_doc,
            RUNTIME_V2_CURIOSITY_ENGINE_FEATURE_DOC,
            "curiosity.source_feature_doc",
        )?;
        validate_relative_path(&self.artifact_path, "curiosity.artifact_path")?;
        validate_relative_path(&self.source_feature_doc, "curiosity.source_feature_doc")?;
        validate_relative_path(&self.runtime_module_ref, "curiosity.runtime_module_ref")?;
        validate_future_component_refs(&self.future_component_refs)?;
        validate_budget(&self.budget)?;
        validate_signals(&self.signals)?;
        validate_proposals(&self.proposals, &self.signals, &self.budget)?;
        validate_governance(&self.governance)?;
        validate_handoff(&self.handoff)?;
        validate_command_list(&self.validation_commands)?;
        ensure_contains_in_list(
            &self.non_claims,
            "does not claim WP-07A CSM component supervisor integration",
            "curiosity non-claims must preserve WP-07A boundary",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "bounded Runtime v2 Curiosity Engine core",
            "curiosity claim boundary must stay bounded to this core",
        )?;
        ensure_contains(
            &self.claim_boundary,
            "later CSM runtime-component hosting",
            "curiosity claim boundary must name future CSM hosting",
        )
    }

    pub fn canonicalized(&self) -> Result<Self> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .signals
            .sort_by(|a, b| a.signal_id.cmp(&b.signal_id));
        canonical
            .proposals
            .sort_by(|a, b| a.proposal_id.cmp(&b.proposal_id));
        canonical.future_component_refs.sort();
        canonical.governance.allowed_runtime_actions.sort();
        canonical.governance.prohibited_actions.sort();
        canonical.handoff.reasoning_graph_refs.sort();
        canonical.handoff.obsmem_refs.sort();
        canonical.handoff.trace_refs.sort();
        canonical.handoff.constructability_refs.sort();
        canonical.handoff.csm_component_followups.sort();
        canonical.handoff.replay_guarantees.sort();
        canonical.validation_commands.sort();
        canonical.non_claims.sort();
        canonical.validate()?;
        Ok(canonical)
    }

    pub fn pretty_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self.canonicalized()?)
            .context("serialize Runtime v2 Curiosity Engine packet")
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "create Runtime v2 Curiosity Engine output directory {}",
                    parent.display()
                )
            })?;
        }
        fs::write(path, self.pretty_json_bytes()?).with_context(|| {
            format!(
                "write Runtime v2 Curiosity Engine packet to {}",
                path.display()
            )
        })
    }
}

pub fn runtime_v2_curiosity_engine_contract() -> Result<RuntimeV2CuriosityEnginePacket> {
    RuntimeV2CuriosityEnginePacket::prototype()?.canonicalized()
}

fn prototype_signals() -> Vec<RuntimeV2CuriositySignal> {
    vec![
        RuntimeV2CuriositySignal {
            signal_id: "signal-capability-delta".to_string(),
            signal_kind: RuntimeV2CuriositySignalKind::CapabilityDelta,
            source_ref: "docs/milestones/v0.91.7/FEATURE_DOCS_v0.91.7.md".to_string(),
            novelty_score: 4,
            surprise_score: 3,
            summary:
                "v0.92 activation asks for curiosity behavior that was previously only documented."
                    .to_string(),
        },
        RuntimeV2CuriositySignal {
            signal_id: "signal-evidence-gap".to_string(),
            signal_kind: RuntimeV2CuriositySignalKind::EvidenceGap,
            source_ref: RUNTIME_V2_CURIOSITY_ENGINE_FEATURE_DOC.to_string(),
            novelty_score: 3,
            surprise_score: 4,
            summary:
                "The feature doc requires a governed discovery-cycle proof before consumption."
                    .to_string(),
        },
    ]
}

fn prototype_proposals() -> Vec<RuntimeV2CuriosityProposal> {
    vec![
        RuntimeV2CuriosityProposal {
            proposal_id: "proposal-bounded-discovery-proof".to_string(),
            source_signal_id: "signal-evidence-gap".to_string(),
            question: "Which bounded discovery cycle can prove curiosity without autonomous exploration?".to_string(),
            hypothesis: "A deterministic proposal packet can prove curiosity admission, governance, handoff, and non-claims before CSM component hosting.".to_string(),
            experiment_plan: vec![
                "derive one reviewable question from an evidence gap".to_string(),
                "bind the question to budget and governance gates".to_string(),
                "emit trace, ObsMem, reasoning-graph, and Constructability handoff refs".to_string(),
            ],
            expected_artifacts: vec![
                RUNTIME_V2_CURIOSITY_ENGINE_PATH.to_string(),
                "runtime_v2/curiosity_engine/curiosity_trace.jsonl".to_string(),
            ],
            gated_by: vec![
                "freedom_gate".to_string(),
                "cav_review".to_string(),
                "constructability_anchor".to_string(),
                "operator_review_for_external_action".to_string(),
            ],
            status: RuntimeV2CuriosityProposalStatus::ReadyForReview,
        },
        RuntimeV2CuriosityProposal {
            proposal_id: "proposal-csm-component-handoff".to_string(),
            source_signal_id: "signal-capability-delta".to_string(),
            question: "How should the curiosity core become a CSM runtime module later?".to_string(),
            hypothesis: "Keep the core deterministic and host-agnostic now, then let WP-07A wrap it with supervision, channels, lifecycle, and provider access.".to_string(),
            experiment_plan: vec![
                "export a stable runtime_v2 contract function".to_string(),
                "record WP-07A component follow-up refs".to_string(),
            ],
            expected_artifacts: vec![RUNTIME_V2_CURIOSITY_ENGINE_PATH.to_string()],
            gated_by: vec![
                "freedom_gate".to_string(),
                "constructability_anchor".to_string(),
            ],
            status: RuntimeV2CuriosityProposalStatus::Proposed,
        },
    ]
}

fn validate_budget(budget: &RuntimeV2CuriosityBudget) -> Result<()> {
    if budget.max_open_questions == 0 {
        return Err(anyhow!(
            "curiosity budget must allow at least one open question"
        ));
    }
    if budget.max_proposals_per_cycle == 0 {
        return Err(anyhow!("curiosity budget must allow at least one proposal"));
    }
    if budget.max_experiment_steps == 0 {
        return Err(anyhow!(
            "curiosity budget must allow at least one experiment step"
        ));
    }
    if budget.max_external_actions != 0 {
        return Err(anyhow!(
            "WP-10 curiosity proof must not allow external actions before later CSM hosting"
        ));
    }
    ensure_contains(
        &budget.exhaustion_policy,
        "defer",
        "curiosity budget exhaustion must defer rather than silently continue",
    )
}

fn validate_signals(signals: &[RuntimeV2CuriositySignal]) -> Result<()> {
    if signals.is_empty() {
        return Err(anyhow!("curiosity signals must not be empty"));
    }
    let mut seen = BTreeSet::new();
    for signal in signals {
        normalize_id(signal.signal_id.clone(), "curiosity.signal_id")?;
        if !seen.insert(signal.signal_id.clone()) {
            return Err(anyhow!("duplicate curiosity signal '{}'", signal.signal_id));
        }
        validate_relative_path(&signal.source_ref, "curiosity.signal.source_ref")?;
        validate_score(signal.novelty_score, "curiosity.signal.novelty_score")?;
        validate_score(signal.surprise_score, "curiosity.signal.surprise_score")?;
        validate_nonempty_text(&signal.summary, "curiosity.signal.summary")?;
    }
    Ok(())
}

fn validate_proposals(
    proposals: &[RuntimeV2CuriosityProposal],
    signals: &[RuntimeV2CuriositySignal],
    budget: &RuntimeV2CuriosityBudget,
) -> Result<()> {
    if proposals.is_empty() {
        return Err(anyhow!("curiosity proposals must not be empty"));
    }
    if proposals.len() > budget.max_proposals_per_cycle as usize {
        return Err(anyhow!(
            "curiosity proposals exceed max_proposals_per_cycle budget"
        ));
    }
    let signal_ids: BTreeSet<_> = signals
        .iter()
        .map(|signal| signal.signal_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for proposal in proposals {
        normalize_id(proposal.proposal_id.clone(), "curiosity.proposal_id")?;
        if !seen.insert(proposal.proposal_id.clone()) {
            return Err(anyhow!(
                "duplicate curiosity proposal '{}'",
                proposal.proposal_id
            ));
        }
        if !signal_ids.contains(proposal.source_signal_id.as_str()) {
            return Err(anyhow!(
                "curiosity proposal '{}' cites missing source signal '{}'",
                proposal.proposal_id,
                proposal.source_signal_id
            ));
        }
        validate_nonempty_text(&proposal.question, "curiosity.proposal.question")?;
        validate_nonempty_text(&proposal.hypothesis, "curiosity.proposal.hypothesis")?;
        if proposal.experiment_plan.is_empty()
            || proposal.experiment_plan.len() > budget.max_experiment_steps as usize
        {
            return Err(anyhow!(
                "curiosity proposal '{}' must stay within max_experiment_steps",
                proposal.proposal_id
            ));
        }
        for artifact in &proposal.expected_artifacts {
            validate_relative_path(artifact, "curiosity.proposal.expected_artifacts")?;
        }
        require_gate(&proposal.gated_by, "freedom_gate")?;
        require_gate(&proposal.gated_by, "constructability_anchor")?;
    }
    Ok(())
}

fn validate_governance(governance: &RuntimeV2CuriosityGovernance) -> Result<()> {
    if !governance.freedom_gate_required {
        return Err(anyhow!("curiosity governance must require Freedom Gate"));
    }
    if !governance.constructability_gate_required {
        return Err(anyhow!(
            "curiosity governance must require Constructability before promotion"
        ));
    }
    if !governance.cav_review_required {
        return Err(anyhow!("curiosity governance must require CAV review"));
    }
    if !governance.operator_review_required_for_external_action {
        return Err(anyhow!(
            "curiosity governance must require operator review for external action"
        ));
    }
    ensure_contains_in_list(
        &governance.allowed_runtime_actions,
        "propose_bounded_experiment",
        "curiosity governance must allow bounded proposal generation",
    )?;
    ensure_contains_in_list(
        &governance.prohibited_actions,
        "autonomous_external_execution",
        "curiosity governance must prohibit autonomous external execution",
    )?;
    ensure_contains_in_list(
        &governance.prohibited_actions,
        "freedom_gate_bypass",
        "curiosity governance must prohibit Freedom Gate bypass",
    )
}

fn validate_handoff(handoff: &RuntimeV2CuriosityHandoff) -> Result<()> {
    validate_path_list(
        &handoff.reasoning_graph_refs,
        "curiosity.handoff.reasoning_graph_refs",
    )?;
    validate_path_list(&handoff.obsmem_refs, "curiosity.handoff.obsmem_refs")?;
    validate_trace_refs(&handoff.trace_refs)?;
    validate_path_list(
        &handoff.constructability_refs,
        "curiosity.handoff.constructability_refs",
    )?;
    ensure_contains_in_list(
        &handoff.csm_component_followups,
        "issue-5124 consumes this curiosity core as a supervised CSM component",
        "curiosity handoff must name the future CSM Curiosity component",
    )?;
    ensure_contains_in_list(
        &handoff.csm_component_followups,
        "issue-5125 hosts the constructability gate needed before promotion",
        "curiosity handoff must name the Constructability component dependency",
    )?;
    ensure_contains_in_list(
        &handoff.replay_guarantees,
        "every proposal must cite an existing signal",
        "curiosity replay guarantees must bind proposals to signals",
    )
}

fn validate_future_component_refs(refs: &[String]) -> Result<()> {
    ensure_contains_in_list(
        refs,
        "issue-5124-curiosity-engine-csm-runtime-component",
        "curiosity future component refs must include #5124",
    )?;
    ensure_contains_in_list(
        refs,
        "issue-5125-constructability-gate-csm-runtime-component",
        "curiosity future component refs must include #5125",
    )
}

fn validate_path_list(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        validate_relative_path(value, field)?;
    }
    Ok(())
}

fn validate_trace_refs(values: &[String]) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("curiosity trace refs must not be empty"));
    }
    for value in values {
        if !value.starts_with("trace://") {
            return Err(anyhow!("curiosity trace ref must start with trace://"));
        }
    }
    Ok(())
}

fn validate_score(value: u8, field: &str) -> Result<()> {
    if value > 5 {
        return Err(anyhow!("{field} must be in the range 0..=5"));
    }
    Ok(())
}

fn require_gate(gates: &[String], expected: &str) -> Result<()> {
    if gates.iter().any(|gate| gate == expected) {
        Ok(())
    } else {
        Err(anyhow!(
            "curiosity proposal missing required gate '{expected}'"
        ))
    }
}

fn validate_command_list(commands: &[String]) -> Result<()> {
    if commands.is_empty() {
        return Err(anyhow!("curiosity validation commands must not be empty"));
    }
    ensure_contains_in_list(
        commands,
        RUNTIME_V2_CURIOSITY_ENGINE_TEST_MARKER,
        "curiosity validation commands must include the focused Rust test marker",
    )?;
    ensure_contains_in_list(
        commands,
        "git diff --check",
        "curiosity validation commands must include whitespace/path hygiene",
    )
}

fn ensure_contains_in_list(values: &[String], needle: &str, message: &str) -> Result<()> {
    if values.iter().any(|value| value.contains(needle)) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn ensure_contains(value: &str, needle: &str, message: &str) -> Result<()> {
    if value.contains(needle) {
        Ok(())
    } else {
        Err(anyhow!("{message}"))
    }
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(anyhow!("{field} must be '{expected}', got '{actual}'"))
    }
}
