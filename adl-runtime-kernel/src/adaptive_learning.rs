use crate::{
    profile_digest, validate_replay, CognitiveProfile, GraphPatch, KernelDurableState, LoopOutcome,
    LoopStatus, MutationAuthority, MutationEvidence, MutationGate, MutationGrant,
    ReasoningGraphDefinition, ValidatedReasoningGraph, GOVERNED_LIFELOG_SCHEMA,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use tokio_util::sync::CancellationToken;

pub const ADAPTIVE_LEARNING_INPUT_SCHEMA: &str = "adl.adaptive_learning.input.v1";
pub const ADAPTIVE_LEARNING_POLICY_SCHEMA: &str = "adl.adaptive_learning.policy.v1";
pub const ADAPTIVE_LEARNING_HISTORY_SCHEMA: &str = "adl.adaptive_learning.history.v1";
pub const ADAPTIVE_LEARNING_DURABLE_DOMAIN: &str = "runtime-v3-adaptive-learning";
const ADAPTIVE_LEARNING_HISTORY_DOMAIN_PREFIX: &str = "runtime-v3-adaptive-learning-history";
const ADAPTIVE_LEARNING_PENDING_DOMAIN: &str = "runtime-v3-adaptive-learning-pending";
const ADAPTIVE_LEARNING_PENDING_SCHEMA: &str = "adl.adaptive_learning.pending.v1";
const MAX_POLICY_EVIDENCE: usize = 256;
const MAX_FEEDBACK_SOURCES: usize = 64;
const MAX_RATIONALE_BYTES: usize = 512;
const MAX_PENDING_INTENT_BYTES: usize = 4 * 1024 * 1024;
const MAX_GATE_SNAPSHOT_BYTES: usize = 2 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PendingStatus {
    Reserved,
    Aborted,
    Committed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveLearningPendingIntent {
    schema: String,
    status: PendingStatus,
    history: AdaptiveLearningHistory,
    before_gate_snapshot_hex: String,
    before_gate_snapshot_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearningEvidence {
    pub id: String,
    pub path: String,
    pub sha256: String,
    pub revision_sha256: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearningEvaluation {
    pub loop_event_sha256: String,
    pub feedback_source: String,
    pub confidence_bps: u16,
    pub evidence_ids: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptationDelta {
    pub before_state_sha256: String,
    pub after_state_sha256: String,
    pub rationale: String,
    pub rollback_state_sha256: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphProposal {
    pub proposal_id: String,
    pub before_graph_sha256: String,
    pub proposed_graph: ReasoningGraphDefinition,
    pub evidence_ids: Vec<String>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningDisposition {
    Accepted,
    Rejected,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearningDecision {
    pub disposition: LearningDisposition,
    pub authority_sha256: String,
    pub policy_sha256: String,
    pub reason_code: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveLearningPolicy {
    pub schema: String,
    pub profile_sha256: String,
    pub capability_envelope_sha256: String,
    pub authority_sha256: String,
    pub evidence: Vec<LearningEvidence>,
    pub max_recurrence: u32,
    pub allowed_feedback_sources: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveLearningInput {
    pub schema: String,
    pub history_id: String,
    pub sequence: u64,
    pub previous_history_sha256: Option<String>,
    pub profile_sha256: String,
    pub capability_envelope_sha256: String,
    pub recurrence: u32,
    pub evaluation: LearningEvaluation,
    pub adaptation: AdaptationDelta,
    pub proposal: GraphProposal,
    pub decision: LearningDecision,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveLearningHistory {
    pub schema: String,
    pub history_id: String,
    pub sequence: u64,
    pub recurrence: u32,
    pub previous_history_sha256: Option<String>,
    pub profile_sha256: String,
    pub capability_envelope_sha256: String,
    pub before_graph_sha256: String,
    pub resulting_graph_sha256: String,
    pub resulting_state_sha256: String,
    pub loop_binding: LearningLoopBinding,
    pub mutation_evidence: Option<MutationEvidence>,
    pub evaluation: LearningEvaluation,
    pub adaptation: AdaptationDelta,
    pub proposal: GraphProposal,
    pub decision: LearningDecision,
    pub policy_sha256: String,
    pub canonical_input_sha256: String,
    pub history_sha256: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearningLoopBinding {
    pub status: LoopStatus,
    pub iterations: u32,
    pub state_sha256: String,
    pub replay_head_sha256: String,
    pub cancellation_observed: bool,
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveLearningRejection {
    UnsupportedSchema,
    InvalidAuthority,
    InvalidEvidence,
    InvalidEvaluation,
    InvalidDelta,
    InvalidGraph,
    InvalidDecision,
    InvalidHistoryPrefix,
    RecurrenceExceeded,
    RollbackMismatch,
    NonCanonicalHistory,
    MutationFailed,
    DurableWriteFailed,
    EncodingFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidentAdaptiveLearningStatus {
    Accepted,
    Rejected,
    Cancelled,
    Restored,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentAdaptiveLearningReceipt {
    pub schema: String,
    pub resident_id: String,
    pub continuity_head_sha256: String,
    pub status: ResidentAdaptiveLearningStatus,
    pub history_id: String,
    pub sequence: u64,
    pub history_sha256: String,
    pub profile_sha256: String,
    pub capability_envelope_sha256: String,
    pub before_graph_sha256: String,
    pub resulting_graph_sha256: String,
    pub resulting_state_sha256: String,
    pub policy_sha256: String,
    pub cancellation_observed: bool,
    pub mutation_evidence_retained: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn execute_resident_adaptive_learning_cycle(
    resident_id: &str,
    continuity_head_sha256: &str,
    gate: &MutationGate,
    durable: &KernelDurableState,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
    loop_outcome: &LoopOutcome,
    cancellation: &CancellationToken,
    mutation: Option<(&MutationGrant, &[GraphPatch])>,
) -> Result<ResidentAdaptiveLearningReceipt, Vec<AdaptiveLearningRejection>> {
    validate_resident_adaptive_learning_bindings(
        resident_id,
        continuity_head_sha256,
        profile,
        input,
        policy,
        previous,
    )?;
    let history = execute_governed_adaptive_learning(
        gate,
        durable,
        profile,
        input,
        policy,
        previous,
        loop_outcome,
        cancellation,
        mutation,
    )?;
    resident_adaptive_learning_receipt(
        resident_id,
        continuity_head_sha256,
        &history,
        resident_status_from_history(&history),
    )
}

pub fn reconcile_resident_adaptive_learning_startup(
    resident_id: &str,
    continuity_head_sha256: &str,
    durable: &KernelDurableState,
    gate: &mut MutationGate,
    profile: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    authority: &MutationAuthority,
) -> Result<Option<ResidentAdaptiveLearningReceipt>, AdaptiveLearningRejection> {
    validate_resident_identity(resident_id, continuity_head_sha256, profile)
        .map_err(|_| AdaptiveLearningRejection::InvalidAuthority)?;
    reconcile_adaptive_learning_startup(durable, gate, profile, policy, authority)?
        .map(|history| {
            resident_adaptive_learning_receipt(
                resident_id,
                continuity_head_sha256,
                &history,
                ResidentAdaptiveLearningStatus::Restored,
            )
            .map_err(|_| AdaptiveLearningRejection::InvalidAuthority)
        })
        .transpose()
}

fn validate_resident_adaptive_learning_bindings(
    resident_id: &str,
    continuity_head_sha256: &str,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    validate_resident_identity(resident_id, continuity_head_sha256, profile)?;
    let mut canonical_policy_value = policy.clone();
    canonical_policy(&mut canonical_policy_value);
    let policy_sha256 = digest(&canonical_policy_value)?;
    if input.profile_sha256 != profile.profile_sha256
        || policy.profile_sha256 != profile.profile_sha256
        || input.capability_envelope_sha256 != profile.capability_envelope_sha256
        || policy.capability_envelope_sha256 != profile.capability_envelope_sha256
        || input.decision.policy_sha256 != policy_sha256
        || previous.is_some_and(|prior| {
            input.previous_history_sha256.as_deref() != Some(prior.history_sha256.as_str())
                || prior.profile_sha256 != profile.profile_sha256
                || prior.capability_envelope_sha256 != profile.capability_envelope_sha256
                || prior.history_id != input.history_id
        })
    {
        return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
    }
    Ok(())
}

fn validate_resident_identity(
    resident_id: &str,
    continuity_head_sha256: &str,
    profile: &CognitiveProfile,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    if !safe_id(resident_id)
        || !safe_text(resident_id)
        || !is_sha(continuity_head_sha256)
        || continuity_head_sha256 != profile.continuity_head
    {
        return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
    }
    Ok(())
}

fn resident_status_from_history(
    history: &AdaptiveLearningHistory,
) -> ResidentAdaptiveLearningStatus {
    if history.loop_binding.cancellation_observed {
        ResidentAdaptiveLearningStatus::Cancelled
    } else {
        match history.decision.disposition {
            LearningDisposition::Accepted => ResidentAdaptiveLearningStatus::Accepted,
            LearningDisposition::Rejected => ResidentAdaptiveLearningStatus::Rejected,
        }
    }
}

fn resident_adaptive_learning_receipt(
    resident_id: &str,
    continuity_head_sha256: &str,
    history: &AdaptiveLearningHistory,
    status: ResidentAdaptiveLearningStatus,
) -> Result<ResidentAdaptiveLearningReceipt, Vec<AdaptiveLearningRejection>> {
    if !safe_id(resident_id) || !safe_text(resident_id) || !is_sha(continuity_head_sha256) {
        return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
    }
    Ok(ResidentAdaptiveLearningReceipt {
        schema: "adl.resident_adaptive_learning.receipt.v1".into(),
        resident_id: resident_id.into(),
        continuity_head_sha256: continuity_head_sha256.into(),
        status,
        history_id: history.history_id.clone(),
        sequence: history.sequence,
        history_sha256: history.history_sha256.clone(),
        profile_sha256: history.profile_sha256.clone(),
        capability_envelope_sha256: history.capability_envelope_sha256.clone(),
        before_graph_sha256: history.before_graph_sha256.clone(),
        resulting_graph_sha256: history.resulting_graph_sha256.clone(),
        resulting_state_sha256: history.resulting_state_sha256.clone(),
        policy_sha256: history.policy_sha256.clone(),
        cancellation_observed: history.loop_binding.cancellation_observed,
        mutation_evidence_retained: history.mutation_evidence.is_some(),
    })
}

pub fn build_adaptive_learning_history(
    graph: &ValidatedReasoningGraph,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
) -> Result<AdaptiveLearningHistory, Vec<AdaptiveLearningRejection>> {
    if input.decision.disposition == LearningDisposition::Accepted {
        return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
    }
    let loop_binding = LearningLoopBinding {
        status: LoopStatus::Exhausted,
        iterations: 0,
        state_sha256: input.adaptation.before_state_sha256.clone(),
        replay_head_sha256: input.evaluation.loop_event_sha256.clone(),
        cancellation_observed: false,
    };
    build_history_internal(graph, profile, input, policy, previous, loop_binding, None)
}

fn build_history_internal(
    graph: &ValidatedReasoningGraph,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
    loop_binding: LearningLoopBinding,
    mutation_evidence: Option<MutationEvidence>,
) -> Result<AdaptiveLearningHistory, Vec<AdaptiveLearningRejection>> {
    let mut e = BTreeSet::new();
    let mut p = policy.clone();
    canonical_policy(&mut p);
    let mut i = input.clone();
    canonical_input(&mut i);
    let psha = digest(&p)?;
    if i.schema != ADAPTIVE_LEARNING_INPUT_SCHEMA || p.schema != ADAPTIVE_LEARNING_POLICY_SCHEMA {
        e.insert(AdaptiveLearningRejection::UnsupportedSchema);
    }
    if profile_digest(profile).ok().as_deref() != Some(profile.profile_sha256.as_str())
        || i.profile_sha256 != profile.profile_sha256
        || p.profile_sha256 != profile.profile_sha256
        || i.capability_envelope_sha256 != profile.capability_envelope_sha256
        || p.capability_envelope_sha256 != profile.capability_envelope_sha256
    {
        e.insert(AdaptiveLearningRejection::InvalidAuthority);
    }
    let refs: BTreeMap<_, _> = p.evidence.iter().map(|v| (v.id.as_str(), v)).collect();
    let folded_refs: BTreeSet<_> = p
        .evidence
        .iter()
        .map(|v| v.id.to_ascii_lowercase())
        .collect();
    if p.evidence.is_empty()
        || p.evidence.len() > MAX_POLICY_EVIDENCE
        || refs.len() != p.evidence.len()
        || folded_refs.len() != p.evidence.len()
        || p.allowed_feedback_sources.is_empty()
        || p.allowed_feedback_sources.len() > MAX_FEEDBACK_SOURCES
        || p.allowed_feedback_sources
            .iter()
            .any(|source| !safe_id(source) || !safe_text(source))
        || i.evaluation.evidence_ids.is_empty()
        || i.evaluation
            .evidence_ids
            .iter()
            .chain(&i.proposal.evidence_ids)
            .any(|id| !refs.contains_key(id.as_str()))
        || p.evidence.iter().any(|v| {
            !safe_id(&v.id)
                || !safe_text(&v.id)
                || !safe_path(&v.path)
                || !safe_text(&v.path)
                || !is_sha(&v.sha256)
                || !is_sha(&v.revision_sha256)
        })
    {
        e.insert(AdaptiveLearningRejection::InvalidEvidence);
    }
    if !safe_id(&i.history_id)
        || !safe_text(&i.history_id)
        || !is_sha(&i.evaluation.loop_event_sha256)
        || i.evaluation.confidence_bps == 0
        || i.evaluation.confidence_bps > 10_000
        || !p
            .allowed_feedback_sources
            .contains(&i.evaluation.feedback_source)
    {
        e.insert(AdaptiveLearningRejection::InvalidEvaluation);
    }
    if i.recurrence == 0
        || i.recurrence > p.max_recurrence
        || p.max_recurrence == 0
        || p.max_recurrence > 10_000
    {
        e.insert(AdaptiveLearningRejection::RecurrenceExceeded);
    }
    if !is_sha(&i.adaptation.before_state_sha256)
        || !is_sha(&i.adaptation.after_state_sha256)
        || i.adaptation.rollback_state_sha256 != i.adaptation.before_state_sha256
        || i.adaptation.rationale.len() < 8
        || i.adaptation.rationale.len() > MAX_RATIONALE_BYTES
        || !safe_text(&i.adaptation.rationale)
    {
        e.insert(AdaptiveLearningRejection::InvalidDelta);
    }
    let proposed = ValidatedReasoningGraph::validate(i.proposal.proposed_graph.clone()).ok();
    if i.proposal.before_graph_sha256 != graph.hash()
        || proposed.is_none()
        || !safe_id(&i.proposal.proposal_id)
        || !safe_text(&i.proposal.proposal_id)
    {
        e.insert(AdaptiveLearningRejection::InvalidGraph);
    }
    if i.decision.policy_sha256 != psha
        || !is_sha(&i.decision.authority_sha256)
        || !safe_id(&i.decision.reason_code)
        || !safe_text(&i.decision.reason_code)
    {
        e.insert(AdaptiveLearningRejection::InvalidDecision);
    }
    if !is_sha(&loop_binding.state_sha256)
        || !is_sha(&loop_binding.replay_head_sha256)
        || loop_binding.replay_head_sha256 != i.evaluation.loop_event_sha256
        || loop_binding.state_sha256 != i.adaptation.before_state_sha256
        || loop_binding.iterations > i.recurrence
    {
        e.insert(AdaptiveLearningRejection::InvalidEvaluation);
    }
    match (&i.decision.disposition, &mutation_evidence) {
        (LearningDisposition::Accepted, Some(mutation))
            if mutation.validate().is_ok()
                && mutation.policy_hash == psha
                && mutation.grant.policy_hash == psha
                && mutation.before_hash == graph.hash()
                && proposed
                    .as_ref()
                    .is_some_and(|candidate| candidate.hash() == mutation.after_hash)
                && i.decision.authority_sha256 == mutation.grant_hash
                && !loop_binding.cancellation_observed
                && loop_binding.status != LoopStatus::Cancelled => {}
        (LearningDisposition::Rejected, None)
            if i.decision.authority_sha256 == p.authority_sha256 => {}
        _ => {
            e.insert(AdaptiveLearningRejection::InvalidAuthority);
        }
    }
    match previous {
        None if i.sequence != 1 || i.previous_history_sha256.is_some() => {
            e.insert(AdaptiveLearningRejection::InvalidHistoryPrefix);
        }
        Some(v)
            if v.sequence.checked_add(1) != Some(i.sequence)
                || i.previous_history_sha256.as_deref() != Some(v.history_sha256.as_str())
                || history_digest(v).ok().as_deref() != Some(v.history_sha256.as_str())
                || !valid_retained_history(v, &p)
                || v.resulting_graph_sha256 != graph.hash()
                || v.history_id != i.history_id
                || v.profile_sha256 != i.profile_sha256
                || v.capability_envelope_sha256 != i.capability_envelope_sha256
                || v.policy_sha256 != psha
                || i.adaptation.before_state_sha256 != v.resulting_state_sha256 =>
        {
            e.insert(AdaptiveLearningRejection::InvalidHistoryPrefix);
        }
        _ => {}
    }
    if !e.is_empty() {
        return Err(e.into_iter().collect());
    }
    let ph = proposed.as_ref().unwrap().hash();
    let (result_graph, result_state) = match i.decision.disposition {
        LearningDisposition::Accepted => (ph.to_owned(), i.adaptation.after_state_sha256.clone()),
        LearningDisposition::Rejected => (
            graph.hash().to_owned(),
            i.adaptation.before_state_sha256.clone(),
        ),
    };
    let canonical_input_sha256 = digest(&i)?;
    let mut o = AdaptiveLearningHistory {
        schema: ADAPTIVE_LEARNING_HISTORY_SCHEMA.into(),
        history_id: i.history_id,
        sequence: i.sequence,
        recurrence: i.recurrence,
        previous_history_sha256: i.previous_history_sha256,
        profile_sha256: i.profile_sha256,
        capability_envelope_sha256: i.capability_envelope_sha256,
        before_graph_sha256: graph.hash().into(),
        resulting_graph_sha256: result_graph,
        resulting_state_sha256: result_state,
        loop_binding,
        mutation_evidence,
        evaluation: i.evaluation,
        adaptation: i.adaptation,
        proposal: i.proposal,
        decision: i.decision,
        policy_sha256: psha,
        canonical_input_sha256,
        history_sha256: String::new(),
    };
    o.history_sha256 = history_digest(&o)?;
    Ok(o)
}

#[allow(clippy::too_many_arguments)]
pub fn execute_governed_adaptive_learning(
    gate: &MutationGate,
    durable: &KernelDurableState,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
    loop_outcome: &LoopOutcome,
    cancellation: &CancellationToken,
    mutation: Option<(&MutationGrant, &[GraphPatch])>,
) -> Result<AdaptiveLearningHistory, Vec<AdaptiveLearningRejection>> {
    let mut canonical_policy_value = policy.clone();
    canonical_policy(&mut canonical_policy_value);
    let policy_sha256 = digest(&canonical_policy_value)?;
    let gate_snapshot = gate
        .snapshot_bytes()
        .map_err(|_| vec![AdaptiveLearningRejection::InvalidAuthority])?;
    let gate_snapshot_value: serde_json::Value = serde_json::from_slice(&gate_snapshot)
        .map_err(|_| vec![AdaptiveLearningRejection::InvalidAuthority])?;
    if gate_snapshot_value["policy_hash"].as_str() != Some(policy_sha256.as_str()) {
        return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
    }
    let before_graph = gate.graph();
    let before_state = gate.adaptation().state();
    let before_state_sha = before_state
        .hash()
        .map_err(|_| vec![AdaptiveLearningRejection::InvalidEvaluation])?;
    let replay_head = loop_outcome
        .replay
        .last()
        .map(|event| event.hash.clone())
        .ok_or_else(|| vec![AdaptiveLearningRejection::InvalidEvaluation])?;
    let replay_start = loop_outcome
        .replay
        .first()
        .and_then(|event| event.sequence.checked_sub(1))
        .ok_or_else(|| vec![AdaptiveLearningRejection::InvalidEvaluation])?;
    let replay_anchor = loop_outcome.replay[0].previous_hash.clone();
    if before_state != loop_outcome.state
        || loop_outcome.iterations == 0
        || loop_outcome.iterations as usize != loop_outcome.replay.len()
        || loop_outcome.iterations > input.recurrence
        || validate_replay(&loop_outcome.replay, replay_start, &replay_anchor)
            .ok()
            .as_deref()
            != Some(replay_head.as_str())
        || loop_outcome.state.accepted_sequence
            != loop_outcome
                .replay
                .last()
                .map(|event| event.sequence)
                .unwrap_or_default()
        || input.evaluation.loop_event_sha256 != replay_head
        || input.adaptation.before_state_sha256 != before_state_sha
        || input.proposal.before_graph_sha256 != before_graph.hash()
        || mutation
            .as_ref()
            .is_some_and(|(_, patches)| patches.is_empty())
    {
        return Err(vec![AdaptiveLearningRejection::InvalidEvaluation]);
    }
    let cancelled = cancellation.is_cancelled() || loop_outcome.status == LoopStatus::Cancelled;
    let mut accepted = mutation.is_some() && !cancelled;
    let mut canonical = input.clone();
    canonical.decision.disposition = if accepted {
        LearningDisposition::Accepted
    } else {
        LearningDisposition::Rejected
    };
    if !accepted {
        canonical.decision.authority_sha256 = policy.authority_sha256.clone();
        canonical.adaptation.after_state_sha256 = before_state_sha.clone();
    }
    let mut loop_binding = LearningLoopBinding {
        status: loop_outcome.status,
        iterations: loop_outcome.iterations,
        state_sha256: before_state_sha.clone(),
        replay_head_sha256: replay_head,
        cancellation_observed: cancelled,
    };

    // Validate every caller-controlled field before the mutation gate is invoked.
    let mut preflight = canonical.clone();
    preflight.decision.disposition = LearningDisposition::Rejected;
    preflight.decision.authority_sha256 = policy.authority_sha256.clone();
    preflight.adaptation.after_state_sha256 = before_state_sha.clone();
    build_history_internal(
        &before_graph,
        profile,
        &preflight,
        policy,
        previous,
        loop_binding.clone(),
        None,
    )?;
    validate_durable_predecessor(durable, &canonical, previous)?;

    if accepted && cancellation.is_cancelled() {
        accepted = false;
        canonical.decision.disposition = LearningDisposition::Rejected;
        canonical.decision.authority_sha256 = policy.authority_sha256.clone();
        canonical.adaptation.after_state_sha256 = before_state_sha.clone();
        loop_binding.cancellation_observed = true;
    }
    // Apply the exact signed mutation to an isolated snapshot first. This both proves that
    // the patches produce the declared proposal and lets all durable writes complete before
    // the live gate is changed.
    let evidence = if let Some((grant, patches)) = mutation.filter(|_| accepted) {
        if grant.policy_hash != policy_sha256 {
            return Err(vec![AdaptiveLearningRejection::InvalidAuthority]);
        }
        let preview = gate
            .restore_from_snapshot(&gate_snapshot)
            .map_err(|_| vec![AdaptiveLearningRejection::MutationFailed])?;
        let evidence = preview
            .apply_and_migrate(grant, patches)
            .map_err(|_| vec![AdaptiveLearningRejection::MutationFailed])?;
        let after_graph = preview.graph();
        if after_graph.definition() != &canonical.proposal.proposed_graph
            || after_graph.hash() != evidence.after_hash
            || evidence.policy_hash != policy_sha256
        {
            return Err(vec![AdaptiveLearningRejection::InvalidGraph]);
        }
        canonical.decision.authority_sha256 = evidence.grant_hash.clone();
        canonical.adaptation.after_state_sha256 = preview
            .adaptation()
            .state()
            .hash()
            .map_err(|_| vec![AdaptiveLearningRejection::MutationFailed])?;
        Some(evidence)
    } else {
        None
    };
    let history = build_history_internal(
        &before_graph,
        profile,
        &canonical,
        policy,
        previous,
        loop_binding,
        evidence,
    )?;
    reserve_pending_intent(durable, &history, &gate_snapshot)?;
    if let Some((grant, patches)) = mutation.filter(|_| accepted) {
        let mut commit_error = None;
        let transaction = gate.apply_and_migrate_transactional(
            grant,
            patches,
            |live_evidence, live_graph, live_adaptation| {
                if history.mutation_evidence.as_ref() != Some(live_evidence)
                    || live_graph.hash() != history.resulting_graph_sha256
                    || live_adaptation.hash().ok().as_deref()
                        != Some(history.resulting_state_sha256.as_str())
                {
                    commit_error = Some(vec![AdaptiveLearningRejection::MutationFailed]);
                    return false;
                }
                match complete_pending_intent(durable, &history) {
                    Ok(()) => true,
                    Err(error) => {
                        commit_error = Some(error);
                        false
                    }
                }
            },
        );
        if transaction.is_err() {
            abort_pending_intent(durable, &history);
            return Err(
                commit_error.unwrap_or_else(|| vec![AdaptiveLearningRejection::MutationFailed])
            );
        }
    } else if let Err(error) = complete_pending_intent(durable, &history) {
        abort_pending_intent(durable, &history);
        return Err(error);
    }
    Ok(history)
}

pub fn adaptive_learning_history_domain(history_id: &str, sequence: u64) -> String {
    format!("{ADAPTIVE_LEARNING_HISTORY_DOMAIN_PREFIX}:{history_id}:{sequence:020}")
}

pub fn adaptive_learning_pending_domain(history_id: &str, sequence: u64) -> String {
    let _ = (history_id, sequence);
    ADAPTIVE_LEARNING_PENDING_DOMAIN.into()
}

fn pending_intent(
    history: &AdaptiveLearningHistory,
    status: PendingStatus,
    before_gate_snapshot: &[u8],
) -> AdaptiveLearningPendingIntent {
    AdaptiveLearningPendingIntent {
        schema: ADAPTIVE_LEARNING_PENDING_SCHEMA.into(),
        status,
        history: history.clone(),
        before_gate_snapshot_hex: hex::encode(before_gate_snapshot),
        before_gate_snapshot_sha256: format!("{:x}", Sha256::digest(before_gate_snapshot)),
    }
}

fn pending_bytes(
    intent: &AdaptiveLearningPendingIntent,
) -> Result<Vec<u8>, Vec<AdaptiveLearningRejection>> {
    serde_jcs::to_vec(intent).map_err(|_| vec![AdaptiveLearningRejection::EncodingFailure])
}

fn reserve_pending_intent(
    durable: &KernelDurableState,
    history: &AdaptiveLearningHistory,
    before_gate_snapshot: &[u8],
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    if before_gate_snapshot.is_empty() || before_gate_snapshot.len() > MAX_GATE_SNAPSHOT_BYTES {
        return Err(vec![AdaptiveLearningRejection::EncodingFailure]);
    }
    let domain = adaptive_learning_pending_domain(&history.history_id, history.sequence);
    let reserved = pending_bytes(&pending_intent(
        history,
        PendingStatus::Reserved,
        before_gate_snapshot,
    ))?;
    let applied = durable
        .compare_and_set_governed_state(&domain, None, &reserved)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    if applied {
        let sequence_domain =
            adaptive_learning_history_domain(&history.history_id, history.sequence);
        if durable
            .load_governed_state(&sequence_domain)
            .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
            .is_none()
        {
            return Ok(());
        }
        abort_pending_intent(durable, history);
        return Err(vec![AdaptiveLearningRejection::DurableWriteFailed]);
    }
    let current = durable
        .load_governed_state(&domain)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
        .ok_or_else(|| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    let intent: AdaptiveLearningPendingIntent = serde_json::from_slice(&current)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    if intent.schema != ADAPTIVE_LEARNING_PENDING_SCHEMA {
        return Err(vec![AdaptiveLearningRejection::DurableWriteFailed]);
    }
    match intent.status {
        PendingStatus::Reserved => Err(vec![AdaptiveLearningRejection::DurableWriteFailed]),
        PendingStatus::Committed | PendingStatus::Aborted => durable
            .compare_and_set_governed_state(&domain, Some(&current), &reserved)
            .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
            .then_some(())
            .ok_or_else(|| vec![AdaptiveLearningRejection::DurableWriteFailed]),
    }
}

fn abort_pending_intent(durable: &KernelDurableState, history: &AdaptiveLearningHistory) {
    let domain = adaptive_learning_pending_domain(&history.history_id, history.sequence);
    if let Ok(Some(current)) = durable.load_governed_state(&domain) {
        if let Ok(mut intent) = serde_json::from_slice::<AdaptiveLearningPendingIntent>(&current) {
            if intent.status == PendingStatus::Reserved && intent.history == *history {
                intent.status = PendingStatus::Aborted;
                if let Ok(aborted) = pending_bytes(&intent) {
                    let _ =
                        durable.compare_and_set_governed_state(&domain, Some(&current), &aborted);
                }
            }
        }
    }
}

fn gate_matches_history(gate: &MutationGate, history: &AdaptiveLearningHistory) -> bool {
    match (&history.decision.disposition, &history.mutation_evidence) {
        (LearningDisposition::Accepted, Some(evidence)) => {
            gate.evidence().contains(evidence)
                && gate.graph().hash() == history.resulting_graph_sha256
                && gate.adaptation().state().hash().ok().as_deref()
                    == Some(history.resulting_state_sha256.as_str())
        }
        (LearningDisposition::Rejected, None) => {
            gate.graph().hash() == history.resulting_graph_sha256
                && gate.adaptation().state().hash().ok().as_deref()
                    == Some(history.resulting_state_sha256.as_str())
        }
        _ => false,
    }
}

fn complete_pending_intent(
    durable: &KernelDurableState,
    history: &AdaptiveLearningHistory,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    let encoded =
        serde_jcs::to_vec(history).map_err(|_| vec![AdaptiveLearningRejection::EncodingFailure])?;
    let sequence_domain = adaptive_learning_history_domain(&history.history_id, history.sequence);
    let expected_head = if history.sequence == 1 {
        None
    } else {
        let prior =
            load_adaptive_learning_history(durable, &history.history_id, history.sequence - 1)
                .map_err(|error| vec![error])?
                .ok_or_else(|| vec![AdaptiveLearningRejection::InvalidHistoryPrefix])?;
        Some(
            serde_jcs::to_vec(&prior)
                .map_err(|_| vec![AdaptiveLearningRejection::EncodingFailure])?,
        )
    };
    let current_head = durable
        .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    let domain = adaptive_learning_pending_domain(&history.history_id, history.sequence);
    let reserved = durable
        .load_governed_state(&domain)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
        .ok_or_else(|| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    let mut intent: AdaptiveLearningPendingIntent = serde_json::from_slice(&reserved)
        .map_err(|_| vec![AdaptiveLearningRejection::NonCanonicalHistory])?;
    if intent.status != PendingStatus::Reserved || intent.history != *history {
        return Err(vec![AdaptiveLearningRejection::DurableWriteFailed]);
    }
    intent.status = PendingStatus::Committed;
    let committed = pending_bytes(&intent)?;
    if current_head.as_deref() == Some(encoded.as_slice())
        && durable
            .load_governed_state(&sequence_domain)
            .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
            .as_deref()
            == Some(encoded.as_slice())
    {
        return Ok(());
    }
    durable
        .append_governed_lifelog(&json!({
            "schema": GOVERNED_LIFELOG_SCHEMA,
            "event": "adaptive_learning_completion_intent",
            "history_id": history.history_id,
            "sequence": history.sequence,
            "history_sha256": history.history_sha256,
            "disposition": history.decision.disposition,
            "history": history,
        }))
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    if !durable
        .compare_and_set_governed_states(&[
            (&sequence_domain, None, encoded.as_slice()),
            (
                ADAPTIVE_LEARNING_DURABLE_DOMAIN,
                expected_head.as_deref(),
                encoded.as_slice(),
            ),
            (&domain, Some(reserved.as_slice()), committed.as_slice()),
        ])
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?
    {
        return Err(vec![AdaptiveLearningRejection::DurableWriteFailed]);
    }
    Ok(())
}

pub fn reconcile_adaptive_learning_startup(
    durable: &KernelDurableState,
    gate: &mut MutationGate,
    profile: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    authority: &MutationAuthority,
) -> Result<Option<AdaptiveLearningHistory>, AdaptiveLearningRejection> {
    let domain = ADAPTIVE_LEARNING_PENDING_DOMAIN;
    let Some(bytes) = durable
        .load_governed_state(domain)
        .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?
    else {
        return Ok(None);
    };
    if bytes.len() > MAX_PENDING_INTENT_BYTES {
        return Err(AdaptiveLearningRejection::NonCanonicalHistory);
    }
    let intent: AdaptiveLearningPendingIntent = serde_json::from_slice(&bytes)
        .map_err(|_| AdaptiveLearningRejection::NonCanonicalHistory)?;
    let before_snapshot = hex::decode(&intent.before_gate_snapshot_hex)
        .map_err(|_| AdaptiveLearningRejection::NonCanonicalHistory)?;
    if intent.schema != ADAPTIVE_LEARNING_PENDING_SCHEMA
        || before_snapshot.is_empty()
        || before_snapshot.len() > MAX_GATE_SNAPSHOT_BYTES
        || serde_jcs::to_vec(&intent).ok().as_deref() != Some(bytes.as_slice())
        || format!("{:x}", Sha256::digest(&before_snapshot)) != intent.before_gate_snapshot_sha256
        || history_digest(&intent.history).ok().as_deref()
            != Some(intent.history.history_sha256.as_str())
    {
        return Err(AdaptiveLearningRejection::NonCanonicalHistory);
    }
    let before_gate = gate
        .restore_from_snapshot(&before_snapshot)
        .map_err(|_| AdaptiveLearningRejection::InvalidAuthority)?;
    let before_graph = before_gate.graph();
    let previous = if intent.history.sequence == 1 {
        None
    } else {
        let previous = load_adaptive_learning_history(
            durable,
            &intent.history.history_id,
            intent.history.sequence - 1,
        )?
        .ok_or(AdaptiveLearningRejection::InvalidHistoryPrefix)?;
        validate_durable_history_chain(durable, &previous, authority, &before_gate, policy)?;
        Some(previous)
    };
    validate_adaptive_learning_history(
        &intent.history,
        &before_graph,
        profile,
        policy,
        previous.as_ref(),
    )
    .map_err(|_| AdaptiveLearningRejection::NonCanonicalHistory)?;
    match (
        &intent.history.decision.disposition,
        &intent.history.mutation_evidence,
    ) {
        (LearningDisposition::Accepted, Some(evidence))
            if authority.verify_evidence(evidence).is_ok()
                && evidence.rollback == *before_graph.definition()
                && evidence.before_hash == before_graph.hash()
                && evidence.after_hash == intent.history.resulting_graph_sha256 => {}
        (LearningDisposition::Rejected, None)
            if intent.history.resulting_graph_sha256 == before_graph.hash() => {}
        _ => return Err(AdaptiveLearningRejection::InvalidAuthority),
    }
    let gate_before = gate.snapshot_bytes().ok().as_deref() == Some(before_snapshot.as_slice());
    let gate_after = gate_matches_history(gate, &intent.history);
    let encoded_history = serde_jcs::to_vec(&intent.history)
        .map_err(|_| AdaptiveLearningRejection::EncodingFailure)?;
    let durable_committed = durable
        .load_governed_state(&adaptive_learning_history_domain(
            &intent.history.history_id,
            intent.history.sequence,
        ))
        .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?
        .as_deref()
        == Some(encoded_history.as_slice())
        && durable
            .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
            .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?
            .as_deref()
            == Some(encoded_history.as_slice());
    match intent.status {
        PendingStatus::Committed if !durable_committed => {
            Err(AdaptiveLearningRejection::DurableWriteFailed)
        }
        PendingStatus::Committed if gate_after => Ok(Some(intent.history)),
        PendingStatus::Committed if gate_before => {
            if let Some(evidence) = &intent.history.mutation_evidence {
                let recovered = gate
                    .apply_and_migrate(&evidence.grant, &evidence.patches)
                    .map_err(|_| AdaptiveLearningRejection::MutationFailed)?;
                if recovered != *evidence || !gate_matches_history(gate, &intent.history) {
                    *gate = before_gate;
                    return Err(AdaptiveLearningRejection::MutationFailed);
                }
            }
            Ok(Some(intent.history))
        }
        PendingStatus::Committed => Err(AdaptiveLearningRejection::MutationFailed),
        PendingStatus::Aborted if gate_before => Ok(None),
        PendingStatus::Aborted if gate_after => {
            *gate = before_gate;
            Ok(None)
        }
        PendingStatus::Aborted => Err(AdaptiveLearningRejection::MutationFailed),
        PendingStatus::Reserved if gate_after => {
            complete_pending_intent(durable, &intent.history)
                .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?;
            Ok(Some(intent.history))
        }
        PendingStatus::Reserved if gate_before => {
            abort_pending_intent(durable, &intent.history);
            Ok(None)
        }
        PendingStatus::Reserved => Err(AdaptiveLearningRejection::MutationFailed),
    }
}

pub fn load_adaptive_learning_history(
    durable: &KernelDurableState,
    history_id: &str,
    sequence: u64,
) -> Result<Option<AdaptiveLearningHistory>, AdaptiveLearningRejection> {
    if !safe_id(history_id) || sequence == 0 {
        return Err(AdaptiveLearningRejection::InvalidHistoryPrefix);
    }
    durable
        .load_governed_state(&adaptive_learning_history_domain(history_id, sequence))
        .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?
        .map(|bytes| {
            serde_json::from_slice(&bytes)
                .map_err(|_| AdaptiveLearningRejection::NonCanonicalHistory)
        })
        .transpose()
}

fn validate_durable_predecessor(
    durable: &KernelDurableState,
    input: &AdaptiveLearningInput,
    previous: Option<&AdaptiveLearningHistory>,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    let head = durable
        .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    match (previous, head) {
        (None, None) => Ok(()),
        (Some(expected), Some(bytes))
            if serde_json::from_slice::<AdaptiveLearningHistory>(&bytes)
                .ok()
                .as_ref()
                == Some(expected)
                && load_adaptive_learning_history(
                    durable,
                    &expected.history_id,
                    expected.sequence,
                )
                .ok()
                .flatten()
                .as_ref()
                    == Some(expected)
                && input.history_id == expected.history_id =>
        {
            Ok(())
        }
        _ => Err(vec![AdaptiveLearningRejection::InvalidHistoryPrefix]),
    }
}
pub fn validate_adaptive_learning_history(
    h: &AdaptiveLearningHistory,
    g: &ValidatedReasoningGraph,
    p: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    prev: Option<&AdaptiveLearningHistory>,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    let i = input_from_history(h);
    match build_history_internal(
        g,
        p,
        &i,
        policy,
        prev,
        h.loop_binding.clone(),
        h.mutation_evidence.clone(),
    ) {
        Ok(v) if &v == h => Ok(()),
        Ok(_) => Err(vec![AdaptiveLearningRejection::NonCanonicalHistory]),
        Err(e) => Err(e),
    }
}
pub fn validate_governed_adaptive_learning_history(
    history: &AdaptiveLearningHistory,
    graph: &ValidatedReasoningGraph,
    profile: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
    authority: &MutationAuthority,
    gate: &MutationGate,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    validate_adaptive_learning_history(history, graph, profile, policy, previous)?;
    if previous.is_some_and(|prior| !valid_governed_evidence(prior, authority, gate, policy)) {
        return Err(vec![AdaptiveLearningRejection::InvalidHistoryPrefix]);
    }
    match (&history.decision.disposition, &history.mutation_evidence) {
        (LearningDisposition::Accepted, Some(evidence))
            if authority.verify_evidence(evidence).is_ok()
                && gate.evidence().contains(evidence)
                && evidence.policy_hash == history.policy_sha256
                && evidence.grant.policy_hash == history.policy_sha256
                && evidence.before_hash == history.before_graph_sha256
                && evidence.after_hash == history.resulting_graph_sha256 =>
        {
            Ok(())
        }
        (LearningDisposition::Rejected, None)
            if history.resulting_graph_sha256 == history.before_graph_sha256
                && history.resulting_state_sha256 == history.adaptation.before_state_sha256 =>
        {
            Ok(())
        }
        _ => Err(vec![AdaptiveLearningRejection::InvalidAuthority]),
    }
}
fn valid_governed_evidence(
    history: &AdaptiveLearningHistory,
    authority: &MutationAuthority,
    gate: &MutationGate,
    policy: &AdaptiveLearningPolicy,
) -> bool {
    match (&history.decision.disposition, &history.mutation_evidence) {
        (LearningDisposition::Accepted, Some(evidence)) => {
            authority.verify_evidence(evidence).is_ok()
                && gate.evidence().contains(evidence)
                && evidence.policy_hash == history.policy_sha256
                && evidence.grant.policy_hash == history.policy_sha256
                && evidence.grant_hash == history.decision.authority_sha256
        }
        (LearningDisposition::Rejected, None) => {
            history.decision.authority_sha256 == policy.authority_sha256
        }
        _ => false,
    }
}
pub fn rollback_adaptive_learning(
    h: &AdaptiveLearningHistory,
    graph: &str,
    state: &str,
) -> Result<(String, String), AdaptiveLearningRejection> {
    let _ = (h, graph, state);
    Err(AdaptiveLearningRejection::InvalidAuthority)
}
#[allow(clippy::too_many_arguments)]
pub fn rollback_governed_adaptive_learning(
    history: &AdaptiveLearningHistory,
    current_graph_sha256: &str,
    current_state_sha256: &str,
    original_graph: &ValidatedReasoningGraph,
    profile: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
    authority: &MutationAuthority,
    gate: &MutationGate,
    durable: &KernelDurableState,
) -> Result<(ReasoningGraphDefinition, String), AdaptiveLearningRejection> {
    validate_governed_adaptive_learning_history(
        history,
        original_graph,
        profile,
        policy,
        previous,
        authority,
        gate,
    )
    .map_err(|_| AdaptiveLearningRejection::InvalidAuthority)?;
    let evidence = history
        .mutation_evidence
        .as_ref()
        .ok_or(AdaptiveLearningRejection::RollbackMismatch)?;
    let retained = durable
        .load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN)
        .map_err(|_| AdaptiveLearningRejection::DurableWriteFailed)?
        .and_then(|bytes| serde_json::from_slice::<AdaptiveLearningHistory>(&bytes).ok());
    validate_durable_history_chain(durable, history, authority, gate, policy)?;
    if history.decision.disposition != LearningDisposition::Accepted
        || current_graph_sha256 != history.resulting_graph_sha256
        || current_state_sha256 != history.resulting_state_sha256
        || evidence.rollback != *original_graph.definition()
        || evidence.before_hash != original_graph.hash()
        || history.adaptation.rollback_state_sha256 != history.adaptation.before_state_sha256
        || retained.as_ref() != Some(history)
    {
        return Err(AdaptiveLearningRejection::RollbackMismatch);
    }
    Ok((
        evidence.rollback.clone(),
        history.adaptation.before_state_sha256.clone(),
    ))
}

fn validate_durable_history_chain(
    durable: &KernelDurableState,
    expected_head: &AdaptiveLearningHistory,
    authority: &MutationAuthority,
    gate: &MutationGate,
    policy: &AdaptiveLearningPolicy,
) -> Result<(), AdaptiveLearningRejection> {
    let mut previous: Option<AdaptiveLearningHistory> = None;
    for sequence in 1..=expected_head.sequence {
        let current = load_adaptive_learning_history(durable, &expected_head.history_id, sequence)?
            .ok_or(AdaptiveLearningRejection::RollbackMismatch)?;
        if current.sequence != sequence
            || current.history_id != expected_head.history_id
            || !valid_retained_history(&current, policy)
            || !valid_governed_evidence(&current, authority, gate, policy)
            || match &previous {
                None => current.previous_history_sha256.is_some(),
                Some(prior) => {
                    current.previous_history_sha256.as_deref()
                        != Some(prior.history_sha256.as_str())
                        || current.before_graph_sha256 != prior.resulting_graph_sha256
                        || current.adaptation.before_state_sha256 != prior.resulting_state_sha256
                }
            }
        {
            return Err(AdaptiveLearningRejection::RollbackMismatch);
        }
        previous = Some(current);
    }
    if previous.as_ref() != Some(expected_head) {
        return Err(AdaptiveLearningRejection::RollbackMismatch);
    }
    Ok(())
}
pub fn history_digest(
    v: &AdaptiveLearningHistory,
) -> Result<String, Vec<AdaptiveLearningRejection>> {
    let mut x = v.clone();
    x.history_sha256.clear();
    digest(&x)
}
fn valid_retained_history(
    history: &AdaptiveLearningHistory,
    policy: &AdaptiveLearningPolicy,
) -> bool {
    let mut canonical_policy_value = policy.clone();
    canonical_policy(&mut canonical_policy_value);
    let policy_sha = digest(&canonical_policy_value).ok();
    let input = input_from_history(history);
    let mut canonicalized_input = input.clone();
    canonical_input(&mut canonicalized_input);
    let references: BTreeSet<_> = policy
        .evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect();
    history.schema == ADAPTIVE_LEARNING_HISTORY_SCHEMA
        && safe_id(&history.history_id)
        && history.sequence > 0
        && history.recurrence > 0
        && history.recurrence <= policy.max_recurrence
        && history.policy_sha256 == policy_sha.unwrap_or_default()
        && history.profile_sha256 == policy.profile_sha256
        && history.capability_envelope_sha256 == policy.capability_envelope_sha256
        && input == canonicalized_input
        && digest(&canonicalized_input).ok().as_deref()
            == Some(history.canonical_input_sha256.as_str())
        && !history.evaluation.evidence_ids.is_empty()
        && history
            .evaluation
            .evidence_ids
            .iter()
            .chain(&history.proposal.evidence_ids)
            .all(|id| references.contains(id.as_str()))
        && history_digest(history).ok().as_deref() == Some(history.history_sha256.as_str())
        && is_sha(&history.canonical_input_sha256)
        && is_sha(&history.loop_binding.state_sha256)
        && is_sha(&history.loop_binding.replay_head_sha256)
        && history.loop_binding.replay_head_sha256 == history.evaluation.loop_event_sha256
        && history.loop_binding.state_sha256 == history.adaptation.before_state_sha256
        && history.loop_binding.iterations <= history.recurrence
        && match (&history.decision.disposition, &history.mutation_evidence) {
            (LearningDisposition::Accepted, Some(evidence)) => {
                evidence.validate().is_ok()
                    && evidence.policy_hash == history.policy_sha256
                    && evidence.grant.policy_hash == history.policy_sha256
                    && evidence.before_hash == history.before_graph_sha256
                    && evidence.after_hash == history.resulting_graph_sha256
                    && evidence.grant_hash == history.decision.authority_sha256
                    && !history.loop_binding.cancellation_observed
                    && history.loop_binding.status != LoopStatus::Cancelled
            }
            (LearningDisposition::Rejected, None) => {
                history.decision.authority_sha256 == policy.authority_sha256
                    && history.resulting_graph_sha256 == history.before_graph_sha256
                    && history.resulting_state_sha256 == history.adaptation.before_state_sha256
            }
            _ => false,
        }
}
fn input_from_history(history: &AdaptiveLearningHistory) -> AdaptiveLearningInput {
    AdaptiveLearningInput {
        schema: ADAPTIVE_LEARNING_INPUT_SCHEMA.into(),
        history_id: history.history_id.clone(),
        sequence: history.sequence,
        previous_history_sha256: history.previous_history_sha256.clone(),
        profile_sha256: history.profile_sha256.clone(),
        capability_envelope_sha256: history.capability_envelope_sha256.clone(),
        recurrence: history.recurrence,
        evaluation: history.evaluation.clone(),
        adaptation: history.adaptation.clone(),
        proposal: history.proposal.clone(),
        decision: history.decision.clone(),
    }
}
fn canonical_input(v: &mut AdaptiveLearningInput) {
    v.evaluation.evidence_ids.sort();
    v.evaluation.evidence_ids.dedup();
    v.proposal.evidence_ids.sort();
    v.proposal.evidence_ids.dedup();
}
fn canonical_policy(v: &mut AdaptiveLearningPolicy) {
    v.evidence.sort();
    v.allowed_feedback_sources.sort();
    v.allowed_feedback_sources.dedup();
}
fn digest<T: Serialize>(v: &T) -> Result<String, Vec<AdaptiveLearningRejection>> {
    serde_jcs::to_vec(v)
        .map(|b| format!("{:x}", Sha256::digest(b)))
        .map_err(|_| vec![AdaptiveLearningRejection::EncodingFailure])
}
fn is_sha(v: &str) -> bool {
    v.len() == 64
        && v.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn safe_id(v: &str) -> bool {
    !v.is_empty()
        && v.len() < 128
        && v.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
}
fn safe_path(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 512
        && !v.starts_with('/')
        && !v.contains(['\\', ':'])
        && !v.split('/').any(|s| {
            s.is_empty()
                || s == "."
                || s == ".."
                || matches!(
                    s.to_ascii_lowercase().as_str(),
                    "private" | "home" | "users" | "user"
                )
        })
}
fn safe_text(v: &str) -> bool {
    let l = v.to_ascii_lowercase();
    ![
        "bearer ",
        "api_key",
        "gho_",
        "sk-",
        "private_state",
        "raw_state",
        "/users/",
        "/home/",
        "/private/",
    ]
    .iter()
    .any(|x| l.contains(x))
}
