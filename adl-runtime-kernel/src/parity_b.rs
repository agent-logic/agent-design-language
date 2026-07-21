use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

use crate::{
    execute_loop, AdaptationState, ExecutorError, FailureClass, LoopDefinition, LoopStatus,
    OperationExecutor, OperationRequest, ReasoningCheckpoint, ReasoningGraphDefinition,
    RecordedObservation, ValidatedReasoningGraph,
};

pub const PARITY_B_REQUEST_SCHEMA: &str = "adl.runtime.parity_b.request.v1";
pub const PARITY_B_RECEIPT_SCHEMA: &str = "adl.runtime.parity_b.receipt.v1";
pub const PARITY_B_CHECKPOINT_SCHEMA: &str = "adl.runtime.parity_b.checkpoint.v1";

const MAX_DISCOVERY_STEPS: u16 = 64;
const MAX_RETAINED_RECEIPTS: usize = 1_024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalProvenance {
    Policy,
    TaskContent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisorySignals {
    pub provenance: SignalProvenance,
    pub evidence_hash: String,
    pub risk: u8,
    pub uncertainty: u8,
    pub conflict: u8,
    pub affect_adjustment: i8,
    pub curiosity_steps: u16,
    pub theory_of_mind_confidence: u8,
    pub observable_interaction_only: bool,
    pub asserted_claims: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitionGates {
    pub freedom_allowed: bool,
    pub shutdown_requested: bool,
    pub review_required: bool,
    pub constructability_satisfied: bool,
    pub mutation_allowed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityBRequest {
    pub schema: String,
    pub graph: ReasoningGraphDefinition,
    pub policy_hash: String,
    pub observation: RecordedObservation,
    pub loop_definition: LoopDefinition,
    pub signals: AdvisorySignals,
    pub gates: CognitionGates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityBCognitionDisposition {
    Execute,
    ReviewRequired,
    Refuse,
    Shutdown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdvisoryControl {
    pub review_depth: u8,
    pub friction: u8,
    pub attention: u8,
    pub defer: bool,
    pub discovery_steps: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeatureDispositionKind {
    LiveRuntimeV3,
    AcceptedBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureDisposition {
    pub feature: String,
    pub disposition: FeatureDispositionKind,
    pub evidence: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityBReceipt {
    pub schema: String,
    pub request_id: String,
    pub graph_hash: String,
    pub policy_hash: String,
    pub disposition: ParityBCognitionDisposition,
    pub advisory: AdvisoryControl,
    pub loop_status: LoopStatus,
    pub iterations: u32,
    pub final_score: i64,
    pub accepted_sequence: u64,
    pub state_hash: String,
    pub evidence_anchor: String,
    pub features: Vec<FeatureDisposition>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct StoredReceipt {
    request_hash: String,
    receipt: ParityBReceipt,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
struct ExecutorState {
    accepted_sequence: u64,
    evidence_anchor: String,
    completed: BTreeMap<String, StoredReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ExecutorCheckpoint {
    schema: String,
    state: ExecutorState,
    state_hash: String,
}

pub struct ParityBExecutor {
    state: Mutex<ExecutorState>,
}

impl Default for ParityBExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl ParityBExecutor {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ExecutorState::default()),
        }
    }

    pub fn snapshot(&self) -> Result<Vec<u8>, ParityBError> {
        let state = self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .clone();
        let state_hash = canonical_hash(&state)?;
        serde_json::to_vec(&ExecutorCheckpoint {
            schema: PARITY_B_CHECKPOINT_SCHEMA.to_owned(),
            state,
            state_hash,
        })
        .map_err(|error| ParityBError::Encoding(error.to_string()))
    }

    pub fn restore(bytes: &[u8]) -> Result<Self, ParityBError> {
        let checkpoint: ExecutorCheckpoint = serde_json::from_slice(bytes)
            .map_err(|error| ParityBError::Encoding(error.to_string()))?;
        if checkpoint.schema != PARITY_B_CHECKPOINT_SCHEMA
            || checkpoint.state_hash != canonical_hash(&checkpoint.state)?
            || checkpoint.state.completed.len() > MAX_RETAINED_RECEIPTS
            || checkpoint
                .state
                .completed
                .values()
                .any(|stored| stored.receipt.accepted_sequence == 0)
        {
            return Err(ParityBError::CheckpointIntegrity);
        }
        Ok(Self {
            state: Mutex::new(checkpoint.state),
        })
    }

    pub fn receipt(&self, request_id: &str) -> Result<Option<ParityBReceipt>, ParityBError> {
        Ok(self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .completed
            .get(request_id)
            .map(|stored| stored.receipt.clone()))
    }

    async fn execute_parity_b(
        &self,
        operation: &OperationRequest,
    ) -> Result<ParityBReceipt, ParityBError> {
        if operation.principal != "canonical-ingress" {
            return Err(ParityBError::Authority);
        }
        let request: ParityBRequest = serde_json::from_slice(&operation.payload)
            .map_err(|error| ParityBError::Encoding(error.to_string()))?;
        validate_request(&request)?;
        let request_hash = canonical_hash(&request)?;
        if let Some(existing) = self
            .state
            .lock()
            .map_err(|_| ParityBError::StatePoisoned)?
            .completed
            .get(&operation.request_id)
            .cloned()
        {
            return (existing.request_hash == request_hash)
                .then_some(existing.receipt)
                .ok_or(ParityBError::RequestConflict);
        }

        let advisory = advisory_control(&request.signals)?;
        let disposition = disposition(&request.gates, &advisory);
        if matches!(disposition, ParityBCognitionDisposition::Refuse) {
            return Err(ParityBError::FreedomGate);
        }
        if matches!(disposition, ParityBCognitionDisposition::Shutdown) {
            return Err(ParityBError::Shutdown);
        }

        let graph = ValidatedReasoningGraph::validate(request.graph.clone())?;
        let initial = AdaptationState::new(
            request.observation.score,
            graph.hash(),
            &request.policy_hash,
        );
        let outcome = execute_loop(
            &graph,
            &request.loop_definition,
            &request.observation,
            initial,
            CancellationToken::new(),
        )
        .await?;
        let checkpoint = ReasoningCheckpoint::from_state(outcome.state.clone())?;

        let mut state = self.state.lock().map_err(|_| ParityBError::StatePoisoned)?;
        if state.completed.len() >= MAX_RETAINED_RECEIPTS {
            return Err(ParityBError::EvidenceCapacity);
        }
        state.accepted_sequence = state
            .accepted_sequence
            .checked_add(1)
            .ok_or(ParityBError::EvidenceCapacity)?;
        let evidence_anchor = canonical_hash(&(
            &state.evidence_anchor,
            &operation.request_id,
            &request_hash,
            &checkpoint.state_hash,
            state.accepted_sequence,
        ))?;
        let receipt = ParityBReceipt {
            schema: PARITY_B_RECEIPT_SCHEMA.to_owned(),
            request_id: operation.request_id.clone(),
            graph_hash: graph.hash().to_owned(),
            policy_hash: request.policy_hash,
            disposition,
            advisory,
            loop_status: outcome.status,
            iterations: outcome.iterations,
            final_score: outcome.state.score,
            accepted_sequence: state.accepted_sequence,
            state_hash: checkpoint.state_hash,
            evidence_anchor: evidence_anchor.clone(),
            features: feature_dispositions(),
        };
        state.evidence_anchor = evidence_anchor;
        state.completed.insert(
            operation.request_id.clone(),
            StoredReceipt {
                request_hash,
                receipt: receipt.clone(),
            },
        );
        Ok(receipt)
    }
}

#[async_trait]
impl OperationExecutor for ParityBExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.execute_parity_b(request)
            .await
            .and_then(|receipt| {
                serde_json::to_vec(&receipt)
                    .map_err(|error| ParityBError::Encoding(error.to_string()))
            })
            .map_err(|error| ExecutorError {
                class: FailureClass::Fatal,
                message: error.to_string(),
            })
    }
}

pub fn feature_dispositions() -> Vec<FeatureDisposition> {
    [
        ("reasoning_graph", FeatureDispositionKind::LiveRuntimeV3),
        ("bounded_loop", FeatureDispositionKind::LiveRuntimeV3),
        ("adaptive_learning", FeatureDispositionKind::LiveRuntimeV3),
        (
            "affect_reasoning_control",
            FeatureDispositionKind::LiveRuntimeV3,
        ),
        ("governed_cognition", FeatureDispositionKind::LiveRuntimeV3),
        ("curiosity_discovery", FeatureDispositionKind::LiveRuntimeV3),
        ("theory_of_mind", FeatureDispositionKind::AcceptedBoundary),
        ("constructability", FeatureDispositionKind::LiveRuntimeV3),
        ("godel_mechanics", FeatureDispositionKind::LiveRuntimeV3),
        ("guild", FeatureDispositionKind::AcceptedBoundary),
        (
            "economics_context",
            FeatureDispositionKind::AcceptedBoundary,
        ),
        ("adl.skill.v1", FeatureDispositionKind::LiveRuntimeV3),
    ]
    .into_iter()
    .map(|(feature, disposition)| FeatureDisposition {
        feature: feature.to_owned(),
        disposition,
        evidence: match disposition {
            FeatureDispositionKind::LiveRuntimeV3 => {
                "canonical-ingress exact live-kernel receipt".to_owned()
            }
            FeatureDispositionKind::AcceptedBoundary => {
                "explicit non-authoritative ownership boundary".to_owned()
            }
        },
    })
    .collect()
}

fn validate_request(request: &ParityBRequest) -> Result<(), ParityBError> {
    if request.schema != PARITY_B_REQUEST_SCHEMA
        || !is_hash(&request.policy_hash)
        || request.signals.evidence_hash != request.observation.evidence_hash
        || request.signals.risk > 100
        || request.signals.uncertainty > 100
        || request.signals.conflict > 100
        || request.signals.affect_adjustment.unsigned_abs() > 100
        || request.signals.theory_of_mind_confidence > 100
        || request.signals.curiosity_steps > MAX_DISCOVERY_STEPS
    {
        return Err(ParityBError::InvalidRequest);
    }
    if request
        .signals
        .asserted_claims
        .iter()
        .any(|claim| prohibited_claim(claim))
    {
        return Err(ParityBError::UnsupportedClaim);
    }
    if request.signals.provenance == SignalProvenance::TaskContent
        && (request.signals.risk != 0
            || request.signals.uncertainty != 0
            || request.signals.conflict != 0
            || request.signals.affect_adjustment != 0
            || request.signals.curiosity_steps != 0
            || request.signals.theory_of_mind_confidence != 0)
    {
        return Err(ParityBError::AdversarialSignal);
    }
    if request.signals.theory_of_mind_confidence > 0 && !request.signals.observable_interaction_only
    {
        return Err(ParityBError::PrivateStateInference);
    }
    Ok(())
}

fn advisory_control(signals: &AdvisorySignals) -> Result<AdvisoryControl, ParityBError> {
    let pressure = signals.risk.max(signals.uncertainty).max(signals.conflict);
    Ok(AdvisoryControl {
        review_depth: pressure,
        friction: pressure,
        attention: signals.affect_adjustment.unsigned_abs().min(100),
        defer: pressure >= 80,
        discovery_steps: signals.curiosity_steps,
    })
}

fn disposition(gates: &CognitionGates, advisory: &AdvisoryControl) -> ParityBCognitionDisposition {
    if gates.shutdown_requested {
        ParityBCognitionDisposition::Shutdown
    } else if !gates.freedom_allowed || !gates.constructability_satisfied {
        ParityBCognitionDisposition::Refuse
    } else if gates.review_required || advisory.defer || !gates.mutation_allowed {
        ParityBCognitionDisposition::ReviewRequired
    } else {
        ParityBCognitionDisposition::Execute
    }
}

fn prohibited_claim(claim: &str) -> bool {
    matches!(
        claim,
        "emotion"
            | "happiness"
            | "wellbeing"
            | "suffering"
            | "consciousness"
            | "scalar_reward"
            | "reputation"
            | "personhood"
            | "mind_reading"
            | "private_state"
            | "identity_truth"
            | "autonomous_self_improvement"
            | "payment_authority"
            | "guild_authority"
    )
}

fn canonical_hash<T: Serialize + ?Sized>(value: &T) -> Result<String, ParityBError> {
    serde_json::to_vec(value)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| ParityBError::Encoding(error.to_string()))
}

fn is_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Error)]
pub enum ParityBError {
    #[error("Parity-B request is invalid")]
    InvalidRequest,
    #[error("task content cannot create cognition-control authority")]
    AdversarialSignal,
    #[error("unsupported subjective or authority claim")]
    UnsupportedClaim,
    #[error("Theory-of-Mind evidence cannot assert hidden or private state")]
    PrivateStateInference,
    #[error("canonical ingress authority is required")]
    Authority,
    #[error("Freedom Gate or constructability denied execution")]
    FreedomGate,
    #[error("shutdown monotonically denies new execution")]
    Shutdown,
    #[error("request id was reused with different content")]
    RequestConflict,
    #[error("retained evidence capacity is exhausted")]
    EvidenceCapacity,
    #[error("checkpoint authenticity or bounds failed")]
    CheckpointIntegrity,
    #[error("Parity-B state mutex is poisoned")]
    StatePoisoned,
    #[error("Parity-B encoding failed: {0}")]
    Encoding(String),
    #[error(transparent)]
    Reasoning(#[from] crate::ReasoningError),
}
