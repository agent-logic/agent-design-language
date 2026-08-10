use crate::{profile_digest, CognitiveProfile, ReasoningGraphDefinition, ValidatedReasoningGraph};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const ADAPTIVE_LEARNING_INPUT_SCHEMA: &str = "adl.adaptive_learning.input.v1";
pub const ADAPTIVE_LEARNING_POLICY_SCHEMA: &str = "adl.adaptive_learning.policy.v1";
pub const ADAPTIVE_LEARNING_HISTORY_SCHEMA: &str = "adl.adaptive_learning.history.v1";

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
    pub previous_history_sha256: Option<String>,
    pub profile_sha256: String,
    pub capability_envelope_sha256: String,
    pub before_graph_sha256: String,
    pub resulting_graph_sha256: String,
    pub resulting_state_sha256: String,
    pub evaluation: LearningEvaluation,
    pub adaptation: AdaptationDelta,
    pub proposal: GraphProposal,
    pub decision: LearningDecision,
    pub policy_sha256: String,
    pub canonical_input_sha256: String,
    pub history_sha256: String,
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
    EncodingFailure,
}

pub fn build_adaptive_learning_history(
    graph: &ValidatedReasoningGraph,
    profile: &CognitiveProfile,
    input: &AdaptiveLearningInput,
    policy: &AdaptiveLearningPolicy,
    previous: Option<&AdaptiveLearningHistory>,
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
    if refs.len() != p.evidence.len()
        || i.evaluation.evidence_ids.is_empty()
        || i.evaluation
            .evidence_ids
            .iter()
            .chain(&i.proposal.evidence_ids)
            .any(|id| !refs.contains_key(id.as_str()))
        || p.evidence.iter().any(|v| {
            !safe_id(&v.id)
                || !safe_path(&v.path)
                || !is_sha(&v.sha256)
                || !is_sha(&v.revision_sha256)
        })
    {
        e.insert(AdaptiveLearningRejection::InvalidEvidence);
    }
    if !is_sha(&i.evaluation.loop_event_sha256)
        || i.evaluation.confidence_bps == 0
        || i.evaluation.confidence_bps > 10_000
        || !p
            .allowed_feedback_sources
            .contains(&i.evaluation.feedback_source)
    {
        e.insert(AdaptiveLearningRejection::InvalidEvaluation);
    }
    if i.recurrence == 0 || i.recurrence > p.max_recurrence {
        e.insert(AdaptiveLearningRejection::RecurrenceExceeded);
    }
    if !is_sha(&i.adaptation.before_state_sha256)
        || !is_sha(&i.adaptation.after_state_sha256)
        || i.adaptation.rollback_state_sha256 != i.adaptation.before_state_sha256
        || i.adaptation.rationale.len() < 8
    {
        e.insert(AdaptiveLearningRejection::InvalidDelta);
    }
    let proposed = ValidatedReasoningGraph::validate(i.proposal.proposed_graph.clone()).ok();
    if i.proposal.before_graph_sha256 != graph.hash()
        || proposed.is_none()
        || !safe_id(&i.proposal.proposal_id)
    {
        e.insert(AdaptiveLearningRejection::InvalidGraph);
    }
    if i.decision.policy_sha256 != psha
        || i.decision.authority_sha256 != p.authority_sha256
        || !is_sha(&i.decision.authority_sha256)
        || !safe_id(&i.decision.reason_code)
    {
        e.insert(AdaptiveLearningRejection::InvalidDecision);
    }
    match previous {
        None if i.sequence != 1 || i.previous_history_sha256.is_some() => {
            e.insert(AdaptiveLearningRejection::InvalidHistoryPrefix);
        }
        Some(v)
            if i.sequence != v.sequence + 1
                || i.previous_history_sha256.as_deref() != Some(v.history_sha256.as_str())
                || history_digest(v).ok().as_deref() != Some(v.history_sha256.as_str())
                || v.resulting_graph_sha256 != graph.hash() =>
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
        previous_history_sha256: i.previous_history_sha256,
        profile_sha256: i.profile_sha256,
        capability_envelope_sha256: i.capability_envelope_sha256,
        before_graph_sha256: graph.hash().into(),
        resulting_graph_sha256: result_graph,
        resulting_state_sha256: result_state,
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
pub fn validate_adaptive_learning_history(
    h: &AdaptiveLearningHistory,
    g: &ValidatedReasoningGraph,
    p: &CognitiveProfile,
    policy: &AdaptiveLearningPolicy,
    prev: Option<&AdaptiveLearningHistory>,
) -> Result<(), Vec<AdaptiveLearningRejection>> {
    let i = AdaptiveLearningInput {
        schema: ADAPTIVE_LEARNING_INPUT_SCHEMA.into(),
        history_id: h.history_id.clone(),
        sequence: h.sequence,
        previous_history_sha256: h.previous_history_sha256.clone(),
        profile_sha256: h.profile_sha256.clone(),
        capability_envelope_sha256: h.capability_envelope_sha256.clone(),
        recurrence: h.sequence as u32,
        evaluation: h.evaluation.clone(),
        adaptation: h.adaptation.clone(),
        proposal: h.proposal.clone(),
        decision: h.decision.clone(),
    };
    match build_adaptive_learning_history(g, p, &i, policy, prev) {
        Ok(v) if &v == h => Ok(()),
        Ok(_) => Err(vec![AdaptiveLearningRejection::NonCanonicalHistory]),
        Err(e) => Err(e),
    }
}
pub fn rollback_adaptive_learning(
    h: &AdaptiveLearningHistory,
    graph: &str,
    state: &str,
) -> Result<(String, String), AdaptiveLearningRejection> {
    if h.decision.disposition != LearningDisposition::Accepted
        || graph != h.resulting_graph_sha256
        || state != h.resulting_state_sha256
    {
        return Err(AdaptiveLearningRejection::RollbackMismatch);
    }
    Ok((
        h.before_graph_sha256.clone(),
        h.adaptation.rollback_state_sha256.clone(),
    ))
}
pub fn history_digest(
    v: &AdaptiveLearningHistory,
) -> Result<String, Vec<AdaptiveLearningRejection>> {
    let mut x = v.clone();
    x.history_sha256.clear();
    digest(&x)
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
