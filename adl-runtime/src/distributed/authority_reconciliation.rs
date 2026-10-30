//! Crash-safe visibility barrier between finalized authority operations and
//! concrete, independently durable authority stores.
//!
//! This module deliberately contains no production store adapter. The sealed
//! registry admits only the deterministic `cfg(test)` adapter until the owning
//! certificate/lease/fence and migration issues add their concrete adapters.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    authority_protocol::{
        AuthorityNodeIdentity, AuthorityOperationKind, CanonicalAuthorityTime,
        CommittedAuthorityArtifact, PublishedAuthorityResult, ReconciliationTokenProjection,
    },
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
};

const ARTIFACT_SCHEMA: &str = "adl.authority-reconciliation.artifact.v1";
const PROTOCOL_INSTANCE: &str = "adl.authority-reconciliation.v1";
#[cfg(any(test, debug_assertions))]
const TEST_ADAPTER_KIND: &str = "adl.test.deterministic-authority";
#[cfg(any(test, debug_assertions))]
const TEST_ADAPTER_VERSION: u32 = 1;
const MAX_OPERATIONS: usize = 4_096;
const MAX_STEPS: usize = 64;
const MAX_STEP_BYTES: usize = 64 * 1024;
const MAX_RESULT_BYTES: usize = 1024 * 1024;
const MAX_IDENTIFIER_BYTES: usize = 128;

pub type AuthorityReconciliationResult<T> = Result<T, AuthorityReconciliationError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityReconciliationError {
    InvalidArtifact,
    UntrustedAuthority,
    WrongTrustDomain,
    WrongPolis,
    WrongNode,
    WrongGuardian,
    WrongBootGeneration,
    WrongProtocolInstance,
    WrongMembership,
    WrongOperationKind,
    UnknownAdapter,
    WrongTimeEvidence,
    RetryConflict,
    ReceiptMismatch,
    ReconciliationRequired,
    PermitDenied,
    ClockNotReady,
    ClockUnsafe,
    Interrupted,
    CapacityExceeded,
    CheckpointConflict,
    Serialization,
    Storage,
    StateRegression,
}

impl AuthorityReconciliationError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidArtifact => "invalid_artifact",
            Self::UntrustedAuthority => "untrusted_authority",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::WrongPolis => "wrong_polis",
            Self::WrongNode => "wrong_node",
            Self::WrongGuardian => "wrong_guardian",
            Self::WrongBootGeneration => "wrong_boot_generation",
            Self::WrongProtocolInstance => "wrong_protocol_instance",
            Self::WrongMembership => "wrong_membership",
            Self::WrongOperationKind => "wrong_operation_kind",
            Self::UnknownAdapter => "unknown_adapter",
            Self::WrongTimeEvidence => "wrong_time_evidence",
            Self::RetryConflict => "retry_conflict",
            Self::ReceiptMismatch => "receipt_mismatch",
            Self::ReconciliationRequired => "reconciliation_required",
            Self::PermitDenied => "permit_denied",
            Self::ClockNotReady => "clock_not_ready",
            Self::ClockUnsafe => "clock_unsafe",
            Self::Interrupted => "interrupted",
            Self::CapacityExceeded => "capacity_exceeded",
            Self::CheckpointConflict => "checkpoint_conflict",
            Self::Serialization => "serialization",
            Self::Storage => "storage",
            Self::StateRegression => "state_regression",
        }
    }
}

impl std::fmt::Display for AuthorityReconciliationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for AuthorityReconciliationError {}

impl From<PolisRuntimeError> for AuthorityReconciliationError {
    fn from(error: PolisRuntimeError) -> Self {
        match error {
            PolisRuntimeError::FrameTooLarge => Self::CapacityExceeded,
            PolisRuntimeError::Serialization => Self::Serialization,
            PolisRuntimeError::StateRegression | PolisRuntimeError::Replay => Self::StateRegression,
            _ => Self::Storage,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityReconciliationIdentity {
    pub trust_domain: String,
    pub polis_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub boot_generation: u64,
    pub protocol_instance: String,
}

impl AuthorityReconciliationIdentity {
    pub fn from_authority_node(identity: &AuthorityNodeIdentity) -> Self {
        Self {
            trust_domain: identity.trust_domain.clone(),
            polis_id: identity.polis_id.clone(),
            node_id: identity.node_id.clone(),
            guardian_id: identity.guardian_id.clone(),
            boot_generation: identity.boot_generation,
            protocol_instance: PROTOCOL_INSTANCE.to_owned(),
        }
    }

    fn validate(&self) -> AuthorityReconciliationResult<()> {
        for value in [
            &self.trust_domain,
            &self.polis_id,
            &self.node_id,
            &self.guardian_id,
            &self.protocol_instance,
        ] {
            validate_identifier(value)?;
        }
        if self.boot_generation == 0 || self.protocol_instance != PROTOCOL_INSTANCE {
            return Err(AuthorityReconciliationError::WrongProtocolInstance);
        }
        Ok(())
    }

    fn checkpoint_object(&self) -> AuthorityReconciliationResult<String> {
        Ok(format!(
            "authority-reconciliation-{}",
            hex::encode(domain_digest(
                b"ADL-AUTHORITY-RECONCILIATION-CHECKPOINT-OBJECT-V1\0",
                self,
            )?)
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityReconciliationArtifact {
    schema: String,
    lineage_id: String,
    adapter_kind: String,
    adapter_version: u32,
    mutation_kind: String,
    steps: Vec<Vec<u8>>,
    result: Vec<u8>,
    inclusive_deadline_unix_seconds: i64,
}

impl AuthorityReconciliationArtifact {
    pub fn new(
        lineage_id: String,
        adapter_kind: String,
        adapter_version: u32,
        mutation_kind: String,
        steps: Vec<Vec<u8>>,
        result: Vec<u8>,
        inclusive_deadline_unix_seconds: i64,
    ) -> AuthorityReconciliationResult<Self> {
        let artifact = Self {
            schema: ARTIFACT_SCHEMA.to_owned(),
            lineage_id,
            adapter_kind,
            adapter_version,
            mutation_kind,
            steps,
            result,
            inclusive_deadline_unix_seconds,
        };
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn committed_artifact(&self) -> AuthorityReconciliationResult<CommittedAuthorityArtifact> {
        self.validate()?;
        let bytes =
            serde_jcs::to_vec(self).map_err(|_| AuthorityReconciliationError::Serialization)?;
        CommittedAuthorityArtifact::new(AuthorityOperationKind::Reconciliation, bytes)
            .map_err(|_| AuthorityReconciliationError::InvalidArtifact)
    }

    fn validate(&self) -> AuthorityReconciliationResult<()> {
        if self.schema != ARTIFACT_SCHEMA
            || self.adapter_version == 0
            || self.inclusive_deadline_unix_seconds <= 0
            || self.steps.is_empty()
            || self.steps.len() > MAX_STEPS
            || self.result.is_empty()
            || self.result.len() > MAX_RESULT_BYTES
        {
            return Err(AuthorityReconciliationError::InvalidArtifact);
        }
        for value in [&self.lineage_id, &self.adapter_kind, &self.mutation_kind] {
            validate_identifier(value)?;
        }
        if self
            .steps
            .iter()
            .any(|step| step.is_empty() || step.len() > MAX_STEP_BYTES)
        {
            return Err(AuthorityReconciliationError::InvalidArtifact);
        }
        let step_digests = self
            .steps
            .iter()
            .map(Sha256::digest)
            .collect::<BTreeSet<_>>();
        if step_digests.len() != self.steps.len() {
            return Err(AuthorityReconciliationError::InvalidArtifact);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityReconciliationPhase {
    Pending,
    Reconciling,
    Checkpointed,
    Published,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedReconciliationResult {
    operation_id: String,
    lineage_id: String,
    adapter_kind: String,
    adapter_version: u32,
    mutation_kind: String,
    generation: u64,
    result: Vec<u8>,
    receipts_sha256: [u8; 32],
    result_sha256: [u8; 32],
}

impl PublishedReconciliationResult {
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn lineage_id(&self) -> &str {
        &self.lineage_id
    }

    pub fn adapter_kind(&self) -> &str {
        &self.adapter_kind
    }

    pub fn adapter_version(&self) -> u32 {
        self.adapter_version
    }

    pub fn mutation_kind(&self) -> &str {
        &self.mutation_kind
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn result(&self) -> &[u8] {
        &self.result
    }

    pub fn result_sha256(&self) -> [u8; 32] {
        self.result_sha256
    }

    pub fn receipts_sha256(&self) -> [u8; 32] {
        self.receipts_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityStepPlan {
    index: u32,
    input_sha256: [u8; 32],
    expected_output_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityStepReceipt {
    index: u32,
    input_sha256: [u8; 32],
    output_sha256: [u8; 32],
    receipt_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableReconciliationOperation {
    operation_id: String,
    lineage_id: String,
    adapter_kind: String,
    adapter_version: u32,
    mutation_kind: String,
    token_sha256: [u8; 32],
    intent_sha256: [u8; 32],
    authority_result_sha256: [u8; 32],
    authority_retry_sha256: [u8; 32],
    committed_log_index: u64,
    finalization_time: CanonicalAuthorityTime,
    time_sha256: [u8; 32],
    signer_set_sha256: [u8; 32],
    artifact_sha256: [u8; 32],
    plan_sha256: [u8; 32],
    steps: Vec<Vec<u8>>,
    plan: Vec<AuthorityStepPlan>,
    receipts: Vec<AuthorityStepReceipt>,
    result: Vec<u8>,
    result_sha256: [u8; 32],
    retry_sha256: [u8; 32],
    target_generation: u64,
    phase: AuthorityReconciliationPhase,
    result_cached: bool,
    marker_written: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedView {
    operation_id: String,
    lineage_id: String,
    adapter_kind: String,
    adapter_version: u32,
    mutation_kind: String,
    generation: u64,
    token_sha256: [u8; 32],
    plan_sha256: [u8; 32],
    receipts_sha256: [u8; 32],
    result_sha256: [u8; 32],
    retry_sha256: [u8; 32],
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReconciliationState {
    revision: u64,
    committed_log_index: u64,
    operations: BTreeMap<String, DurableReconciliationOperation>,
    published: BTreeMap<String, PublishedView>,
}

impl CheckpointMetadataSource for ReconciliationState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.revision),
            state_sha256: Some(hex::encode(
                domain_digest(b"ADL-AUTHORITY-RECONCILIATION-STATE-V1\0", self)
                    .map_err(|_| PolisRuntimeError::Serialization)?,
            )),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) enum AuthorityPermitAction {
    Read,
    Mutation(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct AuthorityReconciliationPermit {
    lineage_id: String,
    adapter_kind: String,
    adapter_version: u32,
    generation: u64,
    action: AuthorityPermitAction,
    operation_sha256: [u8; 32],
}

pub struct AuthorityReconciliationBarrier {
    store: CheckpointedJson<ReconciliationState>,
    envelope: DurableEnvelope<ReconciliationState>,
    identity: AuthorityReconciliationIdentity,
    capacity: usize,
}

impl AuthorityReconciliationBarrier {
    pub fn open(
        root: &Path,
        identity: AuthorityReconciliationIdentity,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> AuthorityReconciliationResult<Self> {
        Self::open_with_capacity(root, identity, authority, MAX_OPERATIONS)
    }

    pub fn open_with_capacity(
        root: &Path,
        identity: AuthorityReconciliationIdentity,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> AuthorityReconciliationResult<Self> {
        identity.validate()?;
        if capacity == 0 || capacity > MAX_OPERATIONS {
            return Err(AuthorityReconciliationError::CapacityExceeded);
        }
        let object = identity.checkpoint_object()?;
        let (store, envelope) = CheckpointedJson::open(
            root,
            &object,
            "authority-reconciliation.json",
            ReconciliationState::default(),
            authority,
        )?;
        validate_state(envelope.payload(), capacity)?;
        Ok(Self {
            store,
            envelope,
            identity,
            capacity,
        })
    }

    pub fn phase(&self, operation_id: &str) -> Option<AuthorityReconciliationPhase> {
        self.envelope
            .payload()
            .operations
            .get(operation_id)
            .map(|operation| operation.phase)
    }

    pub fn published_result(&self, lineage_id: &str) -> Option<PublishedReconciliationResult> {
        let view = self.envelope.payload().published.get(lineage_id)?;
        let operation = self.envelope.payload().operations.get(&view.operation_id)?;
        (operation.phase == AuthorityReconciliationPhase::Published
            && operation.marker_written
            && exact_view(operation, view).is_ok())
        .then(|| published_result(operation))
    }

    /// Publishes deterministic reconciliation authority for integration tests.
    ///
    /// This does not expose a raw store or manufacture an authority-bound store;
    /// callers must still pass through the authority store adapter registry after
    /// publishing the same reconciliation artifact used by production.
    #[cfg(feature = "internal-test-fixtures")]
    #[doc(hidden)]
    pub fn publish_internal_test_fixture(
        &mut self,
        operation_id: &str,
        artifact: AuthorityReconciliationArtifact,
        committed_log_index: u64,
        finalization_time: CanonicalAuthorityTime,
    ) -> AuthorityReconciliationResult<PublishedReconciliationResult> {
        use super::authority_protocol::test_published_reconciliation_token;

        let identity = AuthorityNodeIdentity {
            trust_domain: self.identity.trust_domain.clone(),
            polis_id: self.identity.polis_id.clone(),
            node_id: self.identity.node_id.clone(),
            guardian_id: self.identity.guardian_id.clone(),
            boot_generation: self.identity.boot_generation,
        };
        let token = test_published_reconciliation_token(
            identity,
            operation_id,
            artifact.committed_artifact()?,
            committed_log_index,
            finalization_time,
        );
        self.reconcile(&token)
    }

    /// Denial-only compatibility surface. Raw bytes can never be promoted to
    /// reconciliation authority; callers must supply the opaque #201 result.
    pub fn reject_untrusted_reconciliation(
        &self,
        _untrusted: &[u8],
    ) -> AuthorityReconciliationResult<()> {
        Err(AuthorityReconciliationError::UntrustedAuthority)
    }

    pub fn reconcile(
        &mut self,
        token: &PublishedAuthorityResult,
    ) -> AuthorityReconciliationResult<PublishedReconciliationResult> {
        let projection = token
            .reconciliation_projection()
            .map_err(|_| AuthorityReconciliationError::UntrustedAuthority)?;
        self.validate_projection(&projection)?;
        let artifact = decode_artifact(&projection)?;
        let token_sha256 = token_digest(&projection)?;

        if let Some(existing) = self
            .envelope
            .payload()
            .operations
            .get(&projection.operation_id)
        {
            if existing.token_sha256 != token_sha256 {
                return Err(AuthorityReconciliationError::RetryConflict);
            }
        } else {
            if self.envelope.payload().operations.len() >= self.capacity
                || self
                    .envelope
                    .payload()
                    .operations
                    .values()
                    .any(|operation| {
                        operation.lineage_id == artifact.lineage_id
                            && operation.phase != AuthorityReconciliationPhase::Published
                    })
            {
                return Err(AuthorityReconciliationError::CapacityExceeded);
            }
            let operation = build_operation(
                &projection,
                &artifact,
                token_sha256,
                self.next_generation(&artifact.lineage_id)?,
            )?;
            let mut next = self.envelope.payload().clone();
            next.committed_log_index = next.committed_log_index.max(projection.committed_log_index);
            next.operations
                .insert(projection.operation_id.clone(), operation);
            self.commit(next)?;
            test_fault(&projection.operation_id, "after_journal")?;
        }

        loop {
            let operation = self
                .envelope
                .payload()
                .operations
                .get(&projection.operation_id)
                .cloned()
                .ok_or(AuthorityReconciliationError::StateRegression)?;
            if operation.token_sha256 != token_sha256 {
                return Err(AuthorityReconciliationError::RetryConflict);
            }
            match operation.phase {
                AuthorityReconciliationPhase::Pending => {
                    let mut next = self.envelope.payload().clone();
                    next.operations
                        .get_mut(&projection.operation_id)
                        .ok_or(AuthorityReconciliationError::StateRegression)?
                        .phase = AuthorityReconciliationPhase::Reconciling;
                    self.commit(next)?;
                }
                AuthorityReconciliationPhase::Reconciling => {
                    if operation.receipts.len() < operation.plan.len() {
                        let index = operation.receipts.len();
                        test_fault(&projection.operation_id, &format!("before_step_{index}"))?;
                        let receipt = execute_registered_step(&operation, index)?;
                        validate_receipt(&operation, &receipt, index)?;
                        if operation
                            .receipts
                            .iter()
                            .any(|existing| existing.index == receipt.index)
                        {
                            return Err(AuthorityReconciliationError::ReceiptMismatch);
                        }
                        test_fault(&projection.operation_id, &format!("after_effect_{index}"))?;
                        let mut next = self.envelope.payload().clone();
                        next.operations
                            .get_mut(&projection.operation_id)
                            .ok_or(AuthorityReconciliationError::StateRegression)?
                            .receipts
                            .push(receipt);
                        self.commit(next)?;
                        test_fault(&projection.operation_id, &format!("after_receipt_{index}"))?;
                        continue;
                    }
                    if !operation.result_cached {
                        let mut next = self.envelope.payload().clone();
                        next.operations
                            .get_mut(&projection.operation_id)
                            .ok_or(AuthorityReconciliationError::StateRegression)?
                            .result_cached = true;
                        self.commit(next)?;
                        test_fault(&projection.operation_id, "after_result")?;
                        continue;
                    }
                    test_fault(&projection.operation_id, "before_checkpoint")?;
                    let mut next = self.envelope.payload().clone();
                    next.operations
                        .get_mut(&projection.operation_id)
                        .ok_or(AuthorityReconciliationError::StateRegression)?
                        .phase = AuthorityReconciliationPhase::Checkpointed;
                    self.commit(next)?;
                    test_fault(&projection.operation_id, "after_checkpoint")?;
                }
                AuthorityReconciliationPhase::Checkpointed => {
                    if !operation.marker_written {
                        let mut next = self.envelope.payload().clone();
                        next.operations
                            .get_mut(&projection.operation_id)
                            .ok_or(AuthorityReconciliationError::StateRegression)?
                            .marker_written = true;
                        self.commit(next)?;
                        test_fault(&projection.operation_id, "after_marker")?;
                        continue;
                    }
                    let view = view_for(&operation)?;
                    let mut next = self.envelope.payload().clone();
                    next.operations
                        .get_mut(&projection.operation_id)
                        .ok_or(AuthorityReconciliationError::StateRegression)?
                        .phase = AuthorityReconciliationPhase::Published;
                    next.published.insert(operation.lineage_id.clone(), view);
                    self.commit(next)?;
                    test_fault(&projection.operation_id, "after_view")?;
                }
                AuthorityReconciliationPhase::Published => {
                    let view = self
                        .envelope
                        .payload()
                        .published
                        .get(&operation.lineage_id)
                        .ok_or(AuthorityReconciliationError::StateRegression)?;
                    exact_view(&operation, view)?;
                    return Ok(published_result(&operation));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn read_permit(
        &self,
        lineage_id: &str,
    ) -> AuthorityReconciliationResult<AuthorityReconciliationPermit> {
        self.permit(lineage_id, AuthorityPermitAction::Read)
    }

    #[allow(dead_code)]
    pub(crate) fn mutation_permit(
        &self,
        lineage_id: &str,
        mutation_kind: &str,
    ) -> AuthorityReconciliationResult<AuthorityReconciliationPermit> {
        validate_identifier(mutation_kind)?;
        self.permit(
            lineage_id,
            AuthorityPermitAction::Mutation(mutation_kind.to_owned()),
        )
    }

    #[allow(dead_code)]
    pub(crate) fn validate_permit(
        &self,
        permit: &AuthorityReconciliationPermit,
        required: &AuthorityPermitAction,
    ) -> AuthorityReconciliationResult<()> {
        if &permit.action != required {
            return Err(AuthorityReconciliationError::PermitDenied);
        }
        let view = self
            .envelope
            .payload()
            .published
            .get(&permit.lineage_id)
            .ok_or(AuthorityReconciliationError::ReconciliationRequired)?;
        if self.lineage_has_unpublished(&permit.lineage_id)
            || permit.generation != view.generation
            || permit.adapter_kind != view.adapter_kind
            || permit.adapter_version != view.adapter_version
            || permit.operation_sha256 != view.token_sha256
            || matches!(
                (&permit.action, required),
                (AuthorityPermitAction::Mutation(actual), AuthorityPermitAction::Mutation(expected))
                    if actual != expected || actual != &view.mutation_kind
            )
        {
            return Err(AuthorityReconciliationError::PermitDenied);
        }
        Ok(())
    }

    fn permit(
        &self,
        lineage_id: &str,
        action: AuthorityPermitAction,
    ) -> AuthorityReconciliationResult<AuthorityReconciliationPermit> {
        validate_identifier(lineage_id)?;
        if self.lineage_has_unpublished(lineage_id) {
            return Err(AuthorityReconciliationError::ReconciliationRequired);
        }
        let view = self
            .envelope
            .payload()
            .published
            .get(lineage_id)
            .ok_or(AuthorityReconciliationError::ReconciliationRequired)?;
        if matches!(&action, AuthorityPermitAction::Mutation(kind) if kind != &view.mutation_kind) {
            return Err(AuthorityReconciliationError::PermitDenied);
        }
        Ok(AuthorityReconciliationPermit {
            lineage_id: lineage_id.to_owned(),
            adapter_kind: view.adapter_kind.clone(),
            adapter_version: view.adapter_version,
            generation: view.generation,
            action,
            operation_sha256: view.token_sha256,
        })
    }

    fn lineage_has_unpublished(&self, lineage_id: &str) -> bool {
        self.envelope
            .payload()
            .operations
            .values()
            .any(|operation| {
                operation.lineage_id == lineage_id
                    && operation.phase != AuthorityReconciliationPhase::Published
            })
    }

    fn validate_projection(
        &self,
        projection: &ReconciliationTokenProjection,
    ) -> AuthorityReconciliationResult<()> {
        if projection.identity.trust_domain != self.identity.trust_domain {
            return Err(AuthorityReconciliationError::WrongTrustDomain);
        }
        if projection.identity.polis_id != self.identity.polis_id {
            return Err(AuthorityReconciliationError::WrongPolis);
        }
        if projection.identity.node_id != self.identity.node_id {
            return Err(AuthorityReconciliationError::WrongNode);
        }
        if projection.identity.guardian_id != self.identity.guardian_id {
            return Err(AuthorityReconciliationError::WrongGuardian);
        }
        if projection.identity.boot_generation != self.identity.boot_generation {
            return Err(AuthorityReconciliationError::WrongBootGeneration);
        }
        if self.identity.protocol_instance != PROTOCOL_INSTANCE {
            return Err(AuthorityReconciliationError::WrongProtocolInstance);
        }
        if projection.signer_set_sha256 == [0; 32] || projection.signer_count == 0 {
            return Err(AuthorityReconciliationError::WrongMembership);
        }
        if projection.committed_log_index == 0
            || projection.intent_sha256 == [0; 32]
            || projection.result_sha256 == [0; 32]
            || projection.retry_sha256 == [0; 32]
        {
            return Err(AuthorityReconciliationError::UntrustedAuthority);
        }
        Ok(())
    }

    fn next_generation(&self, lineage_id: &str) -> AuthorityReconciliationResult<u64> {
        self.envelope
            .payload()
            .published
            .get(lineage_id)
            .map_or(Ok(1), |view| {
                view.generation
                    .checked_add(1)
                    .ok_or(AuthorityReconciliationError::StateRegression)
            })
    }

    fn commit(&mut self, mut next: ReconciliationState) -> AuthorityReconciliationResult<()> {
        next.revision = self
            .envelope
            .payload()
            .revision
            .checked_add(1)
            .ok_or(AuthorityReconciliationError::StateRegression)?;
        validate_state(&next, self.capacity)?;
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(())
    }
}

fn decode_artifact(
    projection: &ReconciliationTokenProjection,
) -> AuthorityReconciliationResult<AuthorityReconciliationArtifact> {
    if projection.artifact.domain != "adl.authority-artifact.reconciliation.v1"
        || projection.artifact.sha256
            != <[u8; 32]>::from(Sha256::digest(&projection.artifact.bytes))
    {
        return Err(AuthorityReconciliationError::WrongOperationKind);
    }
    let artifact: AuthorityReconciliationArtifact =
        serde_json::from_slice(&projection.artifact.bytes)
            .map_err(|_| AuthorityReconciliationError::InvalidArtifact)?;
    artifact.validate()?;
    if serde_jcs::to_vec(&artifact).map_err(|_| AuthorityReconciliationError::Serialization)?
        != projection.artifact.bytes
    {
        return Err(AuthorityReconciliationError::InvalidArtifact);
    }
    if projection.finalization_time.unix_seconds > artifact.inclusive_deadline_unix_seconds
        || projection.finalization_time.unix_seconds <= 0
        || projection.finalization_time.nanos >= 1_000_000_000
    {
        return Err(AuthorityReconciliationError::WrongTimeEvidence);
    }
    Ok(artifact)
}

fn build_operation(
    projection: &ReconciliationTokenProjection,
    artifact: &AuthorityReconciliationArtifact,
    token_sha256: [u8; 32],
    target_generation: u64,
) -> AuthorityReconciliationResult<DurableReconciliationOperation> {
    let plan = artifact
        .steps
        .iter()
        .enumerate()
        .map(|(index, input)| {
            let input_sha256 = Sha256::digest(input).into();
            Ok(AuthorityStepPlan {
                index: u32::try_from(index)
                    .map_err(|_| AuthorityReconciliationError::CapacityExceeded)?,
                input_sha256,
                expected_output_sha256: step_output_digest(
                    &artifact.adapter_kind,
                    artifact.adapter_version,
                    index,
                    input_sha256,
                ),
            })
        })
        .collect::<AuthorityReconciliationResult<Vec<_>>>()?;
    let plan_sha256 = domain_digest(b"ADL-AUTHORITY-RECONCILIATION-PLAN-V1\0", &plan)?;
    let result_sha256 = Sha256::digest(&artifact.result).into();
    let retry_sha256 = domain_digest(
        b"ADL-AUTHORITY-RECONCILIATION-RETRY-V1\0",
        &(token_sha256, plan_sha256, result_sha256, target_generation),
    )?;
    Ok(DurableReconciliationOperation {
        operation_id: projection.operation_id.clone(),
        lineage_id: artifact.lineage_id.clone(),
        adapter_kind: artifact.adapter_kind.clone(),
        adapter_version: artifact.adapter_version,
        mutation_kind: artifact.mutation_kind.clone(),
        token_sha256,
        intent_sha256: projection.intent_sha256,
        authority_result_sha256: projection.result_sha256,
        authority_retry_sha256: projection.retry_sha256,
        committed_log_index: projection.committed_log_index,
        finalization_time: projection.finalization_time.clone(),
        time_sha256: domain_digest(
            b"ADL-AUTHORITY-RECONCILIATION-TIME-V1\0",
            &(
                &projection.finalization_time,
                artifact.inclusive_deadline_unix_seconds,
            ),
        )?,
        signer_set_sha256: projection.signer_set_sha256,
        artifact_sha256: projection.artifact.sha256,
        plan_sha256,
        steps: artifact.steps.clone(),
        plan,
        receipts: Vec::new(),
        result: artifact.result.clone(),
        result_sha256,
        retry_sha256,
        target_generation,
        phase: AuthorityReconciliationPhase::Pending,
        result_cached: false,
        marker_written: false,
    })
}

fn execute_registered_step(
    operation: &DurableReconciliationOperation,
    index: usize,
) -> AuthorityReconciliationResult<AuthorityStepReceipt> {
    #[cfg(test)]
    if operation.adapter_kind == TEST_ADAPTER_KIND
        && operation.adapter_version == TEST_ADAPTER_VERSION
    {
        return execute_test_step(operation, index);
    }
    #[cfg(all(debug_assertions, not(test)))]
    if operation.adapter_kind == TEST_ADAPTER_KIND
        && operation.adapter_version == TEST_ADAPTER_VERSION
    {
        return execute_debug_step(operation, index);
    }
    #[cfg(not(any(test, debug_assertions)))]
    let _ = operation;
    let _ = index;
    Err(AuthorityReconciliationError::UnknownAdapter)
}

#[cfg(all(debug_assertions, not(test)))]
fn execute_debug_step(
    operation: &DurableReconciliationOperation,
    index: usize,
) -> AuthorityReconciliationResult<AuthorityStepReceipt> {
    let plan = operation
        .plan
        .get(index)
        .ok_or(AuthorityReconciliationError::ReceiptMismatch)?;
    Ok(AuthorityStepReceipt {
        index: plan.index,
        input_sha256: plan.input_sha256,
        output_sha256: plan.expected_output_sha256,
        receipt_sha256: domain_digest(
            b"ADL-AUTHORITY-RECONCILIATION-STEP-RECEIPT-V1\0",
            &(
                operation.token_sha256,
                operation.plan_sha256,
                plan.index,
                plan.input_sha256,
                plan.expected_output_sha256,
            ),
        )?,
    })
}

fn validate_receipt(
    operation: &DurableReconciliationOperation,
    receipt: &AuthorityStepReceipt,
    index: usize,
) -> AuthorityReconciliationResult<()> {
    let plan = operation
        .plan
        .get(index)
        .ok_or(AuthorityReconciliationError::ReceiptMismatch)?;
    let expected_receipt = domain_digest(
        b"ADL-AUTHORITY-RECONCILIATION-STEP-RECEIPT-V1\0",
        &(
            operation.token_sha256,
            operation.plan_sha256,
            plan.index,
            plan.input_sha256,
            plan.expected_output_sha256,
        ),
    )?;
    if receipt.index != plan.index
        || receipt.input_sha256 != plan.input_sha256
        || receipt.output_sha256 != plan.expected_output_sha256
        || receipt.receipt_sha256 != expected_receipt
    {
        return Err(AuthorityReconciliationError::ReceiptMismatch);
    }
    Ok(())
}

fn view_for(
    operation: &DurableReconciliationOperation,
) -> AuthorityReconciliationResult<PublishedView> {
    if !operation.result_cached
        || !operation.marker_written
        || operation.receipts.len() != operation.plan.len()
    {
        return Err(AuthorityReconciliationError::StateRegression);
    }
    Ok(PublishedView {
        operation_id: operation.operation_id.clone(),
        lineage_id: operation.lineage_id.clone(),
        adapter_kind: operation.adapter_kind.clone(),
        adapter_version: operation.adapter_version,
        mutation_kind: operation.mutation_kind.clone(),
        generation: operation.target_generation,
        token_sha256: operation.token_sha256,
        plan_sha256: operation.plan_sha256,
        receipts_sha256: domain_digest(
            b"ADL-AUTHORITY-RECONCILIATION-RECEIPTS-V1\0",
            &operation.receipts,
        )?,
        result_sha256: operation.result_sha256,
        retry_sha256: operation.retry_sha256,
    })
}

fn exact_view(
    operation: &DurableReconciliationOperation,
    view: &PublishedView,
) -> AuthorityReconciliationResult<()> {
    if view != &view_for(operation)? {
        return Err(AuthorityReconciliationError::StateRegression);
    }
    Ok(())
}

fn published_result(operation: &DurableReconciliationOperation) -> PublishedReconciliationResult {
    let receipts_sha256 = domain_digest(
        b"ADL-AUTHORITY-RECONCILIATION-RECEIPTS-V1\0",
        &operation.receipts,
    )
    .unwrap_or([0; 32]);
    PublishedReconciliationResult {
        operation_id: operation.operation_id.clone(),
        lineage_id: operation.lineage_id.clone(),
        adapter_kind: operation.adapter_kind.clone(),
        adapter_version: operation.adapter_version,
        mutation_kind: operation.mutation_kind.clone(),
        generation: operation.target_generation,
        result: operation.result.clone(),
        receipts_sha256,
        result_sha256: operation.result_sha256,
    }
}

fn token_digest(
    projection: &ReconciliationTokenProjection,
) -> AuthorityReconciliationResult<[u8; 32]> {
    domain_digest(
        b"ADL-AUTHORITY-RECONCILIATION-TOKEN-V1\0",
        &(
            &projection.identity,
            &projection.operation_id,
            projection.intent_sha256,
            projection.result_sha256,
            projection.retry_sha256,
            projection.committed_log_index,
            &projection.finalization_time,
            projection.artifact.sha256,
            projection.signer_set_sha256,
            projection.signer_count,
        ),
    )
}

fn validate_state(
    state: &ReconciliationState,
    capacity: usize,
) -> AuthorityReconciliationResult<()> {
    if state.operations.len() > capacity || state.operations.len() > MAX_OPERATIONS {
        return Err(AuthorityReconciliationError::CapacityExceeded);
    }
    let mut active_lineages = BTreeSet::new();
    for (operation_id, operation) in &state.operations {
        if operation_id != &operation.operation_id
            || operation.operation_id.is_empty()
            || operation.committed_log_index == 0
            || operation.committed_log_index > state.committed_log_index
            || operation.steps.is_empty()
            || operation.steps.len() != operation.plan.len()
            || operation.receipts.len() > operation.plan.len()
            || operation.result.is_empty()
            || operation.result.len() > MAX_RESULT_BYTES
            || operation.result_sha256 != <[u8; 32]>::from(Sha256::digest(&operation.result))
            || operation.plan_sha256
                != domain_digest(b"ADL-AUTHORITY-RECONCILIATION-PLAN-V1\0", &operation.plan)?
        {
            return Err(AuthorityReconciliationError::StateRegression);
        }
        let mut receipt_indexes = BTreeSet::new();
        for (index, receipt) in operation.receipts.iter().enumerate() {
            validate_receipt(operation, receipt, index)?;
            if !receipt_indexes.insert(receipt.index) {
                return Err(AuthorityReconciliationError::ReceiptMismatch);
            }
        }
        if operation.phase != AuthorityReconciliationPhase::Published
            && !active_lineages.insert(operation.lineage_id.clone())
        {
            return Err(AuthorityReconciliationError::StateRegression);
        }
        match operation.phase {
            AuthorityReconciliationPhase::Pending if !operation.receipts.is_empty() => {
                return Err(AuthorityReconciliationError::StateRegression)
            }
            AuthorityReconciliationPhase::Checkpointed
            | AuthorityReconciliationPhase::Published
                if !operation.result_cached || operation.receipts.len() != operation.plan.len() =>
            {
                return Err(AuthorityReconciliationError::StateRegression)
            }
            AuthorityReconciliationPhase::Published if !operation.marker_written => {
                return Err(AuthorityReconciliationError::StateRegression)
            }
            _ => {}
        }
    }
    for (lineage_id, view) in &state.published {
        if lineage_id != &view.lineage_id {
            return Err(AuthorityReconciliationError::StateRegression);
        }
        let operation = state
            .operations
            .get(&view.operation_id)
            .ok_or(AuthorityReconciliationError::StateRegression)?;
        if operation.phase != AuthorityReconciliationPhase::Published {
            return Err(AuthorityReconciliationError::StateRegression);
        }
        exact_view(operation, view)?;
    }
    Ok(())
}

fn step_output_digest(
    adapter_kind: &str,
    adapter_version: u32,
    index: usize,
    input_sha256: [u8; 32],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"ADL-AUTHORITY-RECONCILIATION-STEP-OUTPUT-V1\0");
    digest.update(adapter_kind.as_bytes());
    digest.update(adapter_version.to_be_bytes());
    digest.update(index.to_be_bytes());
    digest.update(input_sha256);
    digest.finalize().into()
}

fn domain_digest<T: Serialize>(
    domain: &[u8],
    value: &T,
) -> AuthorityReconciliationResult<[u8; 32]> {
    let encoded =
        serde_jcs::to_vec(value).map_err(|_| AuthorityReconciliationError::Serialization)?;
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(encoded);
    Ok(digest.finalize().into())
}

fn validate_identifier(value: &str) -> AuthorityReconciliationResult<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
    {
        return Err(AuthorityReconciliationError::InvalidArtifact);
    }
    Ok(())
}

#[cfg(not(test))]
fn test_fault(_operation_id: &str, _point: &str) -> AuthorityReconciliationResult<()> {
    Ok(())
}

#[cfg(test)]
use tests::{execute_test_step, test_fault};

#[cfg(test)]
mod tests;
