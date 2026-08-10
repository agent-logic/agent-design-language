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
const MAX_POLICY_EVIDENCE: usize = 256;
const MAX_FEEDBACK_SOURCES: usize = 64;
const MAX_RATIONALE_BYTES: usize = 512;

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
    match (
        previous,
        durable.load_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN),
    ) {
        (None, Ok(None)) => {}
        (Some(expected), Ok(Some(bytes)))
            if serde_json::from_slice::<AdaptiveLearningHistory>(&bytes)
                .ok()
                .as_ref()
                == Some(expected) => {}
        _ => return Err(vec![AdaptiveLearningRejection::InvalidHistoryPrefix]),
    }

    let pending = json!({
        "schema": GOVERNED_LIFELOG_SCHEMA,
        "event": "adaptive_learning_preflight",
        "history_id": canonical.history_id,
        "sequence": canonical.sequence,
        "disposition": if accepted { "accepted_pending_gate" } else if cancelled { "cancelled" } else { "rejected" },
        "graph_sha256": before_graph.hash(),
        "state_sha256": before_state_sha,
    });
    durable
        .append_governed_lifelog(&pending)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;

    if accepted && cancellation.is_cancelled() {
        accepted = false;
        canonical.decision.disposition = LearningDisposition::Rejected;
        canonical.decision.authority_sha256 = policy.authority_sha256.clone();
        canonical.adaptation.after_state_sha256 = before_state_sha.clone();
        loop_binding.cancellation_observed = true;
    }
    let evidence = if let Some((grant, patches)) = mutation.filter(|_| accepted) {
        let evidence = gate
            .apply_and_migrate(grant, patches)
            .map_err(|_| vec![AdaptiveLearningRejection::MutationFailed])?;
        let after_graph = gate.graph();
        if after_graph.definition() != &canonical.proposal.proposed_graph
            || after_graph.hash() != evidence.after_hash
        {
            return Err(vec![AdaptiveLearningRejection::InvalidGraph]);
        }
        canonical.decision.authority_sha256 = evidence.grant_hash.clone();
        canonical.adaptation.after_state_sha256 = gate
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
    let encoded = serde_jcs::to_vec(&history)
        .map_err(|_| vec![AdaptiveLearningRejection::EncodingFailure])?;
    durable
        .store_governed_state(ADAPTIVE_LEARNING_DURABLE_DOMAIN, &encoded)
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    durable
        .append_governed_lifelog(&json!({
            "schema": GOVERNED_LIFELOG_SCHEMA,
            "event": "adaptive_learning_recorded",
            "history_id": history.history_id,
            "sequence": history.sequence,
            "history_sha256": history.history_sha256,
            "disposition": history.decision.disposition,
        }))
        .map_err(|_| vec![AdaptiveLearningRejection::DurableWriteFailed])?;
    Ok(history)
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
    let policy_sha = digest(policy).ok();
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
