//! Governed membership-transition artifacts and crash-reconciliation records.
//!
//! The coordinator consumes only replicated #201 membership results and opaque
//! #202 authority receipts. It never constructs transport admission/exclusion
//! state and never treats a journaled receipt projection as live authority.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    authority_protocol::{
        AuthorityNodeIdentity, AuthorityOperationKind, CommittedAuthorityArtifact,
        PublishedAuthorityResult,
    },
    learner_transport::{LearnerIdentity, MembershipDiscriminator, VerifiedMembershipArtifact},
    polis_runtime::GovernedMembershipAuthorityReceipt,
    polis_runtime::{
        AppliedMembershipEntry, CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRaft, PolisStateMachineStore,
    },
};

const PROMOTE_ARTIFACT_SCHEMA: &str = "adl.membership-coordinator.promote-voter.v1";
const MEMBERSHIP_ARTIFACT_DOMAIN: &str = "adl.authority-artifact.membership.v1";
const COORDINATOR_OBJECT: &str = "governed-membership-coordinator";
const COORDINATOR_FILE: &str = "governed-membership-coordinator.json";

pub type MembershipCoordinatorResult<T> = Result<T, MembershipCoordinatorError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MembershipCoordinatorError {
    InvalidArtifact,
    WrongOperationKind,
    WrongIdentity,
    WrongStableMap,
    Expired,
    ReceiptMismatch,
    StateRegression,
    Storage,
}

impl std::fmt::Display for MembershipCoordinatorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidArtifact => "invalid_artifact",
            Self::WrongOperationKind => "wrong_operation_kind",
            Self::WrongIdentity => "wrong_identity",
            Self::WrongStableMap => "wrong_stable_map",
            Self::Expired => "expired",
            Self::ReceiptMismatch => "receipt_mismatch",
            Self::StateRegression => "state_regression",
            Self::Storage => "storage",
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipCoordinatorPhase {
    AuthorizedOld,
    ExternalAuthorityObserved,
    LearnerCaughtUp,
    JointCommitted,
    FinalCommitted,
    AuthorityParityReconciled,
    Checkpointed,
    Published,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableAuthorityReceiptProjection {
    operation_sha256: [u8; 32],
    generation: u64,
    published_state_sha256: [u8; 32],
}

impl From<&GovernedMembershipAuthorityReceipt> for DurableAuthorityReceiptProjection {
    fn from(receipt: &GovernedMembershipAuthorityReceipt) -> Self {
        Self {
            operation_sha256: receipt.operation_sha256(),
            generation: receipt.generation(),
            published_state_sha256: receipt.published_state_sha256(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableMembershipTransition {
    operation_sha256: [u8; 32],
    authority_log_index: u64,
    candidate_stable_raft_id: u64,
    old_stable_map_sha256: [u8; 32],
    target_stable_map_sha256: [u8; 32],
    target_membership_sha256: [u8; 32],
    phase: MembershipCoordinatorPhase,
    external_receipt: Option<DurableAuthorityReceiptProjection>,
    joint_membership_sha256: Option<[u8; 32]>,
    final_membership_sha256: Option<[u8; 32]>,
    reconciled_membership_sha256: Option<[u8; 32]>,
    reconciled_authority_sha256: Option<[u8; 32]>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MembershipCoordinatorState {
    committed_log_index: u64,
    published_generation: u64,
    active: Option<DurableMembershipTransition>,
    published_operation_sha256: Option<[u8; 32]>,
    published_result_sha256: Option<[u8; 32]>,
}

impl CheckpointMetadataSource for MembershipCoordinatorState {
    fn checkpoint_metadata(
        &self,
    ) -> Result<CheckpointMetadata, super::polis_runtime::PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: (self.committed_log_index > 0).then_some(self.committed_log_index),
            ..Default::default()
        })
    }
}

/// Durable local half of the cross-authority membership saga.
pub struct MembershipCoordinator {
    store: CheckpointedJson<MembershipCoordinatorState>,
    envelope: DurableEnvelope<MembershipCoordinatorState>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedMembershipTransition {
    pub old_stable_ids: BTreeMap<Vec<u8>, u64>,
    pub target_stable_ids: BTreeMap<Vec<u8>, u64>,
    pub old_membership: BTreeSet<u64>,
    pub target_membership: BTreeSet<u64>,
}

impl MembershipCoordinator {
    pub fn open(
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> MembershipCoordinatorResult<Self> {
        let (store, envelope) = CheckpointedJson::open(
            root,
            COORDINATOR_OBJECT,
            COORDINATOR_FILE,
            MembershipCoordinatorState::default(),
            checkpoint,
        )
        .map_err(|_| MembershipCoordinatorError::Storage)?;
        Ok(Self { store, envelope })
    }

    pub fn begin_promotion(
        &mut self,
        promotion: &VerifiedPromoteVoter,
    ) -> MembershipCoordinatorResult<()> {
        if let Some(active) = self.envelope.payload().active.as_ref() {
            return if active.operation_sha256 == promotion.operation_sha256 {
                Ok(())
            } else {
                Err(MembershipCoordinatorError::StateRegression)
            };
        }
        if self
            .envelope
            .payload()
            .published_operation_sha256
            .is_some_and(|operation| operation == promotion.operation_sha256)
        {
            return Ok(());
        }
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = promotion.committed_log_index;
        next.active = Some(DurableMembershipTransition {
            operation_sha256: promotion.operation_sha256,
            authority_log_index: promotion.committed_log_index,
            candidate_stable_raft_id: promotion.identity.stable_raft_id,
            old_stable_map_sha256: promotion.old_stable_map_sha256,
            target_stable_map_sha256: promotion.target_stable_map_sha256,
            target_membership_sha256: promotion.target_membership_sha256,
            phase: MembershipCoordinatorPhase::AuthorizedOld,
            external_receipt: None,
            joint_membership_sha256: None,
            final_membership_sha256: None,
            reconciled_membership_sha256: None,
            reconciled_authority_sha256: None,
        });
        self.commit(next)
    }

    pub fn observe_external_authority(
        &mut self,
        promotion: &VerifiedPromoteVoter,
        receipt: &GovernedMembershipAuthorityReceipt,
    ) -> MembershipCoordinatorResult<()> {
        promotion.require_current_enrollment_receipt(receipt)?;
        let projection = DurableAuthorityReceiptProjection::from(receipt);
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, promotion.operation_sha256)?;
        if let Some(current) = active.external_receipt.as_ref() {
            if current != &projection {
                return Err(MembershipCoordinatorError::ReceiptMismatch);
            }
        } else {
            active.external_receipt = Some(projection);
        }
        advance_phase(
            active,
            MembershipCoordinatorPhase::ExternalAuthorityObserved,
        )?;
        self.commit(next)
    }

    pub fn record_learner_caught_up(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        self.advance(
            operation_sha256,
            MembershipCoordinatorPhase::LearnerCaughtUp,
        )
    }

    pub fn record_joint_membership(
        &mut self,
        operation_sha256: [u8; 32],
        joint_membership_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        if joint_membership_sha256 == [0; 32] {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        set_once(&mut active.joint_membership_sha256, joint_membership_sha256)?;
        advance_phase(active, MembershipCoordinatorPhase::JointCommitted)?;
        self.commit(next)
    }

    pub fn record_final_membership(
        &mut self,
        operation_sha256: [u8; 32],
        final_membership_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        if final_membership_sha256 == [0; 32] {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        if active.joint_membership_sha256.is_none() {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        set_once(&mut active.final_membership_sha256, final_membership_sha256)?;
        advance_phase(active, MembershipCoordinatorPhase::FinalCommitted)?;
        self.commit(next)
    }

    pub fn record_committed_membership_history(
        &mut self,
        operation_sha256: [u8; 32],
        history: &[AppliedMembershipEntry],
        expected_old: &BTreeSet<u64>,
        expected_target: &BTreeSet<u64>,
    ) -> MembershipCoordinatorResult<()> {
        let authority_log_index = self
            .envelope
            .payload()
            .active
            .as_ref()
            .filter(|active| active.operation_sha256 == operation_sha256)
            .map(|active| active.authority_log_index)
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        let joint_index = history.iter().position(|entry| {
            entry.log_id.index > authority_log_index
                && entry.joint_configs.len() == 2
                && entry.joint_configs[0] == *expected_old
                && entry.joint_configs[1] == *expected_target
        });
        let Some(joint_index) = joint_index else {
            return Err(MembershipCoordinatorError::StateRegression);
        };
        let final_entry = history[joint_index + 1..]
            .iter()
            .find(|entry| {
                entry.log_id.index > history[joint_index].log_id.index
                    && entry.joint_configs.len() == 1
                    && entry.joint_configs[0] == *expected_target
            })
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        let joint_sha256 = membership_history_entry_sha256(&history[joint_index])?;
        let final_sha256 = membership_history_entry_sha256(final_entry)?;
        self.record_joint_membership(operation_sha256, joint_sha256)?;
        self.record_final_membership(operation_sha256, final_sha256)
    }

    pub fn reconcile_authority_parity(
        &mut self,
        operation_sha256: [u8; 32],
        observed_receipt: &GovernedMembershipAuthorityReceipt,
        membership_sha256: [u8; 32],
        authority_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        if membership_sha256 == [0; 32] || authority_sha256 == [0; 32] {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        if active.phase != MembershipCoordinatorPhase::FinalCommitted
            || active.external_receipt.as_ref()
                != Some(&DurableAuthorityReceiptProjection::from(observed_receipt))
        {
            return Err(MembershipCoordinatorError::ReceiptMismatch);
        }
        set_once(&mut active.reconciled_membership_sha256, membership_sha256)?;
        set_once(&mut active.reconciled_authority_sha256, authority_sha256)?;
        advance_phase(
            active,
            MembershipCoordinatorPhase::AuthorityParityReconciled,
        )?;
        self.commit(next)
    }

    pub fn checkpoint(&mut self, operation_sha256: [u8; 32]) -> MembershipCoordinatorResult<()> {
        self.advance(operation_sha256, MembershipCoordinatorPhase::Checkpointed)
    }

    pub fn publish(&mut self, operation_sha256: [u8; 32]) -> MembershipCoordinatorResult<[u8; 32]> {
        let mut next = self.envelope.payload().clone();
        if next
            .published_operation_sha256
            .is_some_and(|published| published == operation_sha256)
        {
            return next
                .published_result_sha256
                .ok_or(MembershipCoordinatorError::StateRegression);
        }
        let active = exact_active_mut(&mut next, operation_sha256)?;
        if active.phase != MembershipCoordinatorPhase::Checkpointed
            || active.reconciled_membership_sha256.is_none()
            || active.reconciled_authority_sha256.is_none()
        {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        active.phase = MembershipCoordinatorPhase::Published;
        let result_sha256 = <[u8; 32]>::from(Sha256::digest(
            serde_jcs::to_vec(active).map_err(|_| MembershipCoordinatorError::Storage)?,
        ));
        next.published_generation = next
            .published_generation
            .checked_add(1)
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        next.published_operation_sha256 = Some(operation_sha256);
        next.published_result_sha256 = Some(result_sha256);
        next.active = None;
        self.commit(next)?;
        Ok(result_sha256)
    }

    pub fn published_generation(&self) -> u64 {
        self.envelope.payload().published_generation
    }

    pub fn active_phase(&self) -> Option<MembershipCoordinatorPhase> {
        self.envelope
            .payload()
            .active
            .as_ref()
            .map(|active| active.phase)
    }

    pub async fn promote_voter_with_raft(
        &mut self,
        promotion: &VerifiedPromoteVoter,
        current_receipt: &GovernedMembershipAuthorityReceipt,
        raft: &PolisRaft,
        state_machine: &PolisStateMachineStore,
        transition: &AuthorizedMembershipTransition,
    ) -> MembershipCoordinatorResult<()> {
        verify_authorized_transition_inputs(
            promotion,
            &transition.old_stable_ids,
            &transition.target_stable_ids,
            &transition.old_membership,
            &transition.target_membership,
        )?;
        self.begin_promotion(promotion)?;
        self.observe_external_authority(promotion, current_receipt)?;
        if self.active_phase() == Some(MembershipCoordinatorPhase::ExternalAuthorityObserved) {
            raft.add_learner(
                promotion.identity.stable_raft_id,
                openraft::BasicNode::new(promotion.identity.address.to_string()),
                true,
            )
            .await
            .map_err(|_| MembershipCoordinatorError::StateRegression)?;
            self.record_learner_caught_up(promotion.operation_sha256)?;
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::LearnerCaughtUp) {
            raft.change_membership(transition.target_membership.clone(), false)
                .await
                .map_err(|_| MembershipCoordinatorError::StateRegression)?;
        }
        let history = state_machine.applied_membership_history().await;
        self.record_committed_membership_history(
            promotion.operation_sha256,
            &history,
            &transition.old_membership,
            &transition.target_membership,
        )
    }

    fn advance(
        &mut self,
        operation_sha256: [u8; 32],
        phase: MembershipCoordinatorPhase,
    ) -> MembershipCoordinatorResult<()> {
        let mut next = self.envelope.payload().clone();
        advance_phase(exact_active_mut(&mut next, operation_sha256)?, phase)?;
        self.commit(next)
    }

    fn commit(&mut self, next: MembershipCoordinatorState) -> MembershipCoordinatorResult<()> {
        self.envelope = self
            .store
            .commit(&self.envelope, next)
            .map_err(|_| MembershipCoordinatorError::Storage)?;
        Ok(())
    }
}

pub fn membership_history_entry_sha256(
    entry: &AppliedMembershipEntry,
) -> MembershipCoordinatorResult<[u8; 32]> {
    serde_jcs::to_vec(entry)
        .map(|bytes| <[u8; 32]>::from(Sha256::digest(bytes)))
        .map_err(|_| MembershipCoordinatorError::StateRegression)
}

fn exact_active_mut(
    state: &mut MembershipCoordinatorState,
    operation_sha256: [u8; 32],
) -> MembershipCoordinatorResult<&mut DurableMembershipTransition> {
    state
        .active
        .as_mut()
        .filter(|active| active.operation_sha256 == operation_sha256)
        .ok_or(MembershipCoordinatorError::StateRegression)
}

fn set_once(slot: &mut Option<[u8; 32]>, value: [u8; 32]) -> MembershipCoordinatorResult<()> {
    match slot {
        Some(current) if current == &value => Ok(()),
        Some(_) => Err(MembershipCoordinatorError::StateRegression),
        None => {
            *slot = Some(value);
            Ok(())
        }
    }
}

fn advance_phase(
    active: &mut DurableMembershipTransition,
    target: MembershipCoordinatorPhase,
) -> MembershipCoordinatorResult<()> {
    let current = active.phase as u8;
    let target = target as u8;
    if target < current || target > current.saturating_add(1) {
        return Err(MembershipCoordinatorError::StateRegression);
    }
    if target > current {
        active.phase = match target {
            0 => MembershipCoordinatorPhase::AuthorizedOld,
            1 => MembershipCoordinatorPhase::ExternalAuthorityObserved,
            2 => MembershipCoordinatorPhase::LearnerCaughtUp,
            3 => MembershipCoordinatorPhase::JointCommitted,
            4 => MembershipCoordinatorPhase::FinalCommitted,
            5 => MembershipCoordinatorPhase::AuthorityParityReconciled,
            6 => MembershipCoordinatorPhase::Checkpointed,
            7 => MembershipCoordinatorPhase::Published,
            _ => return Err(MembershipCoordinatorError::StateRegression),
        };
    }
    Ok(())
}

impl std::error::Error for MembershipCoordinatorError {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalPromoteVoterArtifact {
    schema: String,
    discriminator: String,
    identity: LearnerIdentity,
    voter_cut_sha256: [u8; 32],
    enrollment_operation_sha256: [u8; 32],
    enrollment_generation: u64,
    old_stable_map_sha256: [u8; 32],
    target_stable_map_sha256: [u8; 32],
    target_membership_sha256: [u8; 32],
    deadline_unix_seconds: i64,
}

/// Builder for the #199-owned discriminator under coarse Membership authority.
pub struct PromoteVoterArtifact;

impl PromoteVoterArtifact {
    #[allow(clippy::too_many_arguments)]
    pub fn committed(
        identity: LearnerIdentity,
        voter_cut_sha256: [u8; 32],
        enrollment_operation_sha256: [u8; 32],
        enrollment_generation: u64,
        old_stable_map_sha256: [u8; 32],
        target_stable_map_sha256: [u8; 32],
        target_membership_sha256: [u8; 32],
        deadline_unix_seconds: i64,
    ) -> MembershipCoordinatorResult<CommittedAuthorityArtifact> {
        identity
            .validate()
            .map_err(|_| MembershipCoordinatorError::WrongIdentity)?;
        if identity.stable_raft_id == 0
            || voter_cut_sha256 == [0; 32]
            || enrollment_operation_sha256 == [0; 32]
            || enrollment_generation == 0
            || old_stable_map_sha256 == [0; 32]
            || target_stable_map_sha256 == [0; 32]
            || old_stable_map_sha256 == target_stable_map_sha256
            || target_membership_sha256 == [0; 32]
            || deadline_unix_seconds <= 0
        {
            return Err(MembershipCoordinatorError::InvalidArtifact);
        }
        let bytes = serde_jcs::to_vec(&CanonicalPromoteVoterArtifact {
            schema: PROMOTE_ARTIFACT_SCHEMA.to_owned(),
            discriminator: "promote_voter".to_owned(),
            identity,
            voter_cut_sha256,
            enrollment_operation_sha256,
            enrollment_generation,
            old_stable_map_sha256,
            target_stable_map_sha256,
            target_membership_sha256,
            deadline_unix_seconds,
        })
        .map_err(|_| MembershipCoordinatorError::InvalidArtifact)?;
        CommittedAuthorityArtifact::new(AuthorityOperationKind::Membership, bytes)
            .map_err(|_| MembershipCoordinatorError::InvalidArtifact)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedPromoteVoter {
    identity: LearnerIdentity,
    publication_identity: AuthorityNodeIdentity,
    operation_sha256: [u8; 32],
    committed_log_index: u64,
    enrollment_operation_sha256: [u8; 32],
    enrollment_generation: u64,
    old_stable_map_sha256: [u8; 32],
    target_stable_map_sha256: [u8; 32],
    target_membership_sha256: [u8; 32],
}

impl VerifiedPromoteVoter {
    pub fn from_published(
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        expected_old_stable_map_sha256: [u8; 32],
        expected_target_stable_map_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> MembershipCoordinatorResult<Self> {
        let artifact = result
            .operation()
            .artifact_for_sealed_consumer()
            .map_err(|_| MembershipCoordinatorError::WrongOperationKind)?;
        if artifact.domain != MEMBERSHIP_ARTIFACT_DOMAIN
            || artifact.sha256 != <[u8; 32]>::from(Sha256::digest(&artifact.bytes))
        {
            return Err(MembershipCoordinatorError::WrongOperationKind);
        }
        let payload: CanonicalPromoteVoterArtifact = serde_json::from_slice(&artifact.bytes)
            .map_err(|_| MembershipCoordinatorError::InvalidArtifact)?;
        if serde_jcs::to_vec(&payload).map_err(|_| MembershipCoordinatorError::InvalidArtifact)?
            != artifact.bytes
            || payload.schema != PROMOTE_ARTIFACT_SCHEMA
            || payload.discriminator != "promote_voter"
        {
            return Err(MembershipCoordinatorError::InvalidArtifact);
        }
        if &payload.identity != expected_identity {
            return Err(MembershipCoordinatorError::WrongIdentity);
        }
        let publication_identity = result.authority_identity_for_sealed_consumer();
        if publication_identity.trust_domain != payload.identity.trust_domain
            || publication_identity.polis_id != payload.identity.polis_id
        {
            return Err(MembershipCoordinatorError::WrongIdentity);
        }
        if payload.voter_cut_sha256 != expected_voter_cut_sha256
            || payload.old_stable_map_sha256 != expected_old_stable_map_sha256
            || payload.target_stable_map_sha256 != expected_target_stable_map_sha256
        {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        if now_unix_seconds <= 0 || now_unix_seconds >= payload.deadline_unix_seconds {
            return Err(MembershipCoordinatorError::Expired);
        }
        Ok(Self {
            identity: payload.identity,
            publication_identity: publication_identity.clone(),
            operation_sha256: result.result_sha256(),
            committed_log_index: result.committed_log_index(),
            enrollment_operation_sha256: payload.enrollment_operation_sha256,
            enrollment_generation: payload.enrollment_generation,
            old_stable_map_sha256: payload.old_stable_map_sha256,
            target_stable_map_sha256: payload.target_stable_map_sha256,
            target_membership_sha256: payload.target_membership_sha256,
        })
    }

    pub fn require_current_enrollment_receipt(
        &self,
        receipt: &GovernedMembershipAuthorityReceipt,
    ) -> MembershipCoordinatorResult<()> {
        if receipt.operation_sha256() != self.enrollment_operation_sha256
            || receipt.generation() != self.enrollment_generation
            || receipt.published_state_sha256() == [0; 32]
        {
            return Err(MembershipCoordinatorError::ReceiptMismatch);
        }
        Ok(())
    }

    pub fn identity(&self) -> &LearnerIdentity {
        &self.identity
    }

    pub fn publication_identity(&self) -> &AuthorityNodeIdentity {
        &self.publication_identity
    }

    pub fn operation_sha256(&self) -> [u8; 32] {
        self.operation_sha256
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn old_stable_map_sha256(&self) -> [u8; 32] {
        self.old_stable_map_sha256
    }

    pub fn target_stable_map_sha256(&self) -> [u8; 32] {
        self.target_stable_map_sha256
    }

    pub fn target_membership_sha256(&self) -> [u8; 32] {
        self.target_membership_sha256
    }
}

pub fn stable_map_sha256(
    stable_ids: &BTreeMap<Vec<u8>, u64>,
) -> MembershipCoordinatorResult<[u8; 32]> {
    if stable_ids.is_empty()
        || stable_ids.values().any(|id| *id == 0)
        || stable_ids
            .values()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != stable_ids.len()
    {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    let ordered = stable_ids.iter().collect::<Vec<_>>();
    serde_jcs::to_vec(&ordered)
        .map(|bytes| <[u8; 32]>::from(Sha256::digest(bytes)))
        .map_err(|_| MembershipCoordinatorError::WrongStableMap)
}

pub fn membership_set_sha256(membership: &BTreeSet<u64>) -> MembershipCoordinatorResult<[u8; 32]> {
    if membership.is_empty() || membership.contains(&0) {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    serde_jcs::to_vec(membership)
        .map(|bytes| <[u8; 32]>::from(Sha256::digest(bytes)))
        .map_err(|_| MembershipCoordinatorError::WrongStableMap)
}

pub fn verify_authorized_transition_inputs(
    promotion: &VerifiedPromoteVoter,
    old_stable_ids: &BTreeMap<Vec<u8>, u64>,
    target_stable_ids: &BTreeMap<Vec<u8>, u64>,
    expected_old: &BTreeSet<u64>,
    expected_target: &BTreeSet<u64>,
) -> MembershipCoordinatorResult<()> {
    if stable_map_sha256(old_stable_ids)? != promotion.old_stable_map_sha256
        || stable_map_sha256(target_stable_ids)? != promotion.target_stable_map_sha256
        || membership_set_sha256(expected_target)? != promotion.target_membership_sha256
        || old_stable_ids.values().copied().collect::<BTreeSet<_>>() != *expected_old
        || target_stable_ids.values().copied().collect::<BTreeSet<_>>() != *expected_target
        || old_stable_ids
            .iter()
            .any(|(guardian, raft_id)| target_stable_ids.get(guardian) != Some(raft_id))
        || target_stable_ids.get(promotion.identity.guardian_id.as_bytes())
            != Some(&promotion.identity.stable_raft_id)
    {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    Ok(())
}

pub fn verify_external_membership_receipt(
    result: &PublishedAuthorityResult,
    discriminator: MembershipDiscriminator,
    receipt: &GovernedMembershipAuthorityReceipt,
    expected_identity: &LearnerIdentity,
    expected_voter_cut_sha256: [u8; 32],
    expected_target_membership_sha256: [u8; 32],
    now_unix_seconds: i64,
) -> MembershipCoordinatorResult<()> {
    let artifact = VerifiedMembershipArtifact::from_published(result, discriminator)
        .map_err(|_| MembershipCoordinatorError::InvalidArtifact)?;
    if artifact.identity() != expected_identity {
        return Err(MembershipCoordinatorError::WrongIdentity);
    }
    if artifact.voter_cut_sha256() != expected_voter_cut_sha256
        || artifact.target_membership_sha256() != expected_target_membership_sha256
    {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    if now_unix_seconds <= 0 || now_unix_seconds >= artifact.deadline_unix_seconds() {
        return Err(MembershipCoordinatorError::Expired);
    }
    if receipt.operation_sha256() != artifact.operation_sha256()
        || receipt.generation() == 0
        || receipt.published_state_sha256() == [0; 32]
    {
        return Err(MembershipCoordinatorError::ReceiptMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
