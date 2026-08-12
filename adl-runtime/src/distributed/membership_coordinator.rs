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
    learner_transport::{
        LearnerIdentity, MembershipDiscriminator, VerifiedLearnerAdmission,
        VerifiedMembershipArtifact,
    },
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipState,
    },
    polis_runtime::GovernedMembershipAuthorityReceipt,
    polis_runtime::{
        AppliedMembershipEntry, CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRaft, PolisStateMachineStore,
        SecurePolisNetworkFactory,
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
    final_membership_log_index: Option<u64>,
    reconciled_membership_sha256: Option<[u8; 32]>,
    reconciled_authority_sha256: Option<[u8; 32]>,
    #[serde(default)]
    membership_change_submitted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableEnrollment {
    operation_sha256: [u8; 32],
    membership_event_log_index: u64,
    candidate_stable_raft_id: u64,
    old_stable_map_sha256: [u8; 32],
    target_stable_map_sha256: [u8; 32],
    target_stable_ids: Vec<(Vec<u8>, u64)>,
    external_receipt: Option<DurableAuthorityReceiptProjection>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurablePublishedMembershipResult {
    operation_sha256: [u8; 32],
    result_sha256: [u8; 32],
    committed_log_index: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct PreparedEnrollmentStableIds {
    target: BTreeMap<Vec<u8>, u64>,
    old_sha256: [u8; 32],
    target_sha256: [u8; 32],
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MembershipCoordinatorState {
    committed_log_index: u64,
    published_generation: u64,
    active: Option<DurableMembershipTransition>,
    #[serde(default)]
    active_enrollment: Option<DurableEnrollment>,
    #[serde(default)]
    stable_id_registry: Vec<(Vec<u8>, u64)>,
    #[serde(default)]
    pending_visibility_operation_sha256: Option<[u8; 32]>,
    published_operation_sha256: Option<[u8; 32]>,
    published_result_sha256: Option<[u8; 32]>,
    #[serde(default)]
    published_results: Vec<DurablePublishedMembershipResult>,
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
    #[cfg(test)]
    crash_after_membership_change: bool,
    #[cfg(test)]
    fail_membership_change_before_submit: bool,
    #[cfg(test)]
    crash_boundary: Option<MembershipCrashBoundary>,
    #[cfg(test)]
    crash_boundary_hit: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MembershipCrashBoundary {
    BeforeEnrollmentJournal,
    AfterEnrollmentJournal,
    BeforeExternalAuthorityCall,
    AfterExternalAuthorityCall,
    AfterExternalAuthorityObservation,
    BeforeLearnerEffect,
    AfterLearnerEffect,
    AfterJointHistory,
    AfterFinalHistory,
    AfterJointFinalObservation,
    AfterStableMapPreparation,
    AfterLocalProjectionPrepared,
    AfterParityReconciliation,
    BeforeCheckpoint,
    AfterCheckpoint,
    AfterDurablePublicationBeforeVisibility,
}

/// Runtime-owned production aggregate. Callers submit governed operations to
/// this owner rather than composing coordinator phases or publication pieces.
pub struct GovernedMembershipRuntime {
    coordinator: Option<MembershipCoordinator>,
    factory: SecurePolisNetworkFactory,
    raft: PolisRaft,
    state_machine: PolisStateMachineStore,
    membership: MembershipState,
    authority: AuthorityMembership,
}

impl GovernedMembershipRuntime {
    pub fn new(
        coordinator: MembershipCoordinator,
        factory: SecurePolisNetworkFactory,
        raft: PolisRaft,
        state_machine: PolisStateMachineStore,
        membership: MembershipState,
        authority: AuthorityMembership,
    ) -> Self {
        Self {
            coordinator: Some(coordinator),
            factory,
            raft,
            state_machine,
            membership,
            authority,
        }
    }

    pub async fn promote(
        &mut self,
        promotion: &VerifiedPromoteVoter,
        transition: &AuthorizedMembershipTransition,
        candidate: VoterAuthority,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        self.coordinator
            .as_mut()
            .expect("governed coordinator is installed")
            .promote_voter_to_published(
                promotion,
                &self.factory,
                &self.raft,
                &self.state_machine,
                transition,
                &mut self.membership,
                &mut self.authority,
                candidate,
            )
            .await
    }

    /// Resume the same durable owner after a leader change. Operation identity
    /// remains in the coordinator journal; only the live consensus handles are
    /// replaced.
    pub fn resume_consensus(&mut self, raft: PolisRaft, state_machine: PolisStateMachineStore) {
        self.raft = raft;
        self.state_machine = state_machine;
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn remove(
        &mut self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        now_unix_seconds: i64,
        transition: &AuthorizedMembershipTransition,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        let current = self.authority.clone();
        self.coordinator
            .as_mut()
            .expect("governed coordinator is installed")
            .remove_voter_to_published(
                result,
                expected_identity,
                expected_voter_cut_sha256,
                now_unix_seconds,
                &self.factory,
                &self.raft,
                &self.state_machine,
                transition,
                &current,
                &mut self.membership,
                &mut self.authority,
            )
            .await
    }

    pub async fn enroll_non_voting(
        &mut self,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
        membership_event_log_index: u64,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        self.coordinator
            .as_mut()
            .expect("governed coordinator is installed")
            .enroll_non_voting_to_published(
                admission,
                &self.factory,
                now_unix_seconds,
                membership_event_log_index,
                &self.authority,
                &self.raft,
                &mut self.membership,
            )
            .await
    }

    pub fn membership(&self) -> &MembershipState {
        &self.membership
    }

    pub fn authority(&self) -> &AuthorityMembership {
        &self.authority
    }

    #[cfg(test)]
    pub(crate) fn coordinator(&self) -> &MembershipCoordinator {
        self.coordinator
            .as_ref()
            .expect("governed coordinator is installed")
    }

    #[cfg(test)]
    pub(crate) fn has_published_result(&self, operation_sha256: [u8; 32]) -> bool {
        published_result(
            self.coordinator
                .as_ref()
                .expect("governed coordinator is installed")
                .envelope
                .payload(),
            operation_sha256,
        )
        .is_some()
    }

    #[cfg(test)]
    pub(crate) fn inject_crash_after_membership_change(&mut self) {
        self.coordinator
            .as_mut()
            .expect("governed coordinator is installed")
            .inject_crash_after_membership_change();
    }

    #[cfg(test)]
    pub(crate) fn inject_membership_change_no_effect_failure(&mut self) {
        self.coordinator
            .as_mut()
            .expect("governed coordinator is installed")
            .inject_membership_change_no_effect_failure();
    }

    #[cfg(test)]
    pub(crate) fn inject_crash_boundary(&mut self, boundary: MembershipCrashBoundary) {
        let coordinator = self
            .coordinator
            .as_mut()
            .expect("governed coordinator is installed");
        coordinator.crash_boundary = Some(boundary);
        coordinator.crash_boundary_hit = false;
    }

    #[cfg(test)]
    pub(crate) fn crash_boundary_hit(&self) -> bool {
        self.coordinator
            .as_ref()
            .expect("governed coordinator is installed")
            .crash_boundary_hit
    }

    #[cfg(test)]
    pub(crate) fn reopen_coordinator(
        &mut self,
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) {
        drop(self.coordinator.take());
        self.coordinator = Some(MembershipCoordinator::open(root, checkpoint).unwrap());
    }

    #[cfg(test)]
    pub(crate) fn replace_authority_for_test(
        &mut self,
        authority: AuthorityMembership,
    ) -> AuthorityMembership {
        std::mem::replace(&mut self.authority, authority)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedMembershipTransition {
    pub old_stable_ids: BTreeMap<Vec<u8>, u64>,
    pub target_stable_ids: BTreeMap<Vec<u8>, u64>,
    pub old_membership: BTreeSet<u64>,
    pub target_membership: BTreeSet<u64>,
}

/// Canonical, validated observation of the local membership and authority
/// projections.  Its fields are intentionally private: completion APIs accept
/// this evidence rather than caller-selected digests.
struct ObservedMembershipParity {
    membership_sha256: [u8; 32],
    authority_sha256: [u8; 32],
    committed_log_index: u64,
}

impl ObservedMembershipParity {
    fn observe(
        membership: &MembershipState,
        authority: &AuthorityMembership,
        transition: &AuthorizedMembershipTransition,
    ) -> MembershipCoordinatorResult<Self> {
        if authority.raft_ids != transition.target_stable_ids
            || authority.raft_membership.get_joint_config().len() != 1
            || authority
                .raft_membership
                .voter_ids()
                .collect::<BTreeSet<_>>()
                != transition.target_membership
            || membership.trust_domain().as_bytes() != authority.trust_domain_id
            || membership.committed_log_index() != authority.committed_log_index
        {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        let members = membership
            .members()
            .filter(|member| member.role == MemberRole::Voter)
            .map(|member| (member.guardian_id.as_bytes().to_vec(), member))
            .collect::<BTreeMap<_, _>>();
        if members.len() != authority.voters.len()
            || authority.voters.iter().any(|(guardian, voter)| {
                members.get(guardian).is_none_or(|member| {
                    member.guardian_control_public_key != voter.control_public_key
                        || member.identity_generation != voter.certificate_generation
                })
            })
        {
            return Err(MembershipCoordinatorError::WrongIdentity);
        }
        let membership_bytes = membership
            .snapshot()
            .map_err(|_| MembershipCoordinatorError::StateRegression)?;
        let authority_projection = (
            &authority.trust_domain_id,
            authority.voter_set_generation,
            authority.committed_log_index,
            authority.raft_membership.get_joint_config(),
            authority.raft_ids.iter().collect::<Vec<_>>(),
            authority.voters.iter().collect::<Vec<_>>(),
        );
        let authority_bytes = serde_jcs::to_vec(&authority_projection)
            .map_err(|_| MembershipCoordinatorError::StateRegression)?;
        Ok(Self {
            membership_sha256: <[u8; 32]>::from(Sha256::digest(membership_bytes)),
            authority_sha256: <[u8; 32]>::from(Sha256::digest(authority_bytes)),
            committed_log_index: membership.committed_log_index(),
        })
    }
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
        Ok(Self {
            store,
            envelope,
            #[cfg(test)]
            crash_after_membership_change: false,
            #[cfg(test)]
            fail_membership_change_before_submit: false,
            #[cfg(test)]
            crash_boundary: None,
            #[cfg(test)]
            crash_boundary_hit: false,
        })
    }

    fn begin_promotion(
        &mut self,
        promotion: &VerifiedPromoteVoter,
    ) -> MembershipCoordinatorResult<()> {
        if self.envelope.payload().active_enrollment.is_some()
            || self
                .envelope
                .payload()
                .pending_visibility_operation_sha256
                .is_some()
        {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        if let Some(active) = self.envelope.payload().active.as_ref() {
            return if active.operation_sha256 == promotion.operation_sha256 {
                Ok(())
            } else {
                Err(MembershipCoordinatorError::StateRegression)
            };
        }
        if published_result(self.envelope.payload(), promotion.operation_sha256).is_some() {
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
            final_membership_log_index: None,
            reconciled_membership_sha256: None,
            reconciled_authority_sha256: None,
            membership_change_submitted: false,
        });
        self.commit(next)
    }

    fn begin_removal(
        &mut self,
        removal: &VerifiedMembershipArtifact,
        transition: &AuthorizedMembershipTransition,
    ) -> MembershipCoordinatorResult<()> {
        if self.envelope.payload().active_enrollment.is_some()
            || self
                .envelope
                .payload()
                .pending_visibility_operation_sha256
                .is_some()
        {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        if let Some(active) = self.envelope.payload().active.as_ref() {
            return if active.operation_sha256 == removal.operation_sha256() {
                Ok(())
            } else {
                Err(MembershipCoordinatorError::StateRegression)
            };
        }
        if published_result(self.envelope.payload(), removal.operation_sha256()).is_some() {
            return Ok(());
        }
        verify_authorized_removal_inputs(removal, transition)?;
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = removal.committed_log_index();
        next.active = Some(DurableMembershipTransition {
            operation_sha256: removal.operation_sha256(),
            authority_log_index: removal.committed_log_index(),
            candidate_stable_raft_id: removal.identity().stable_raft_id,
            old_stable_map_sha256: stable_map_sha256(&transition.old_stable_ids)?,
            target_stable_map_sha256: stable_map_sha256(&transition.target_stable_ids)?,
            target_membership_sha256: removal.target_membership_sha256(),
            phase: MembershipCoordinatorPhase::AuthorizedOld,
            external_receipt: None,
            joint_membership_sha256: None,
            final_membership_sha256: None,
            final_membership_log_index: None,
            reconciled_membership_sha256: None,
            reconciled_authority_sha256: None,
            membership_change_submitted: false,
        });
        self.commit(next)
    }

    fn observe_removal_authority(
        &mut self,
        removal: &VerifiedMembershipArtifact,
        receipt: &GovernedMembershipAuthorityReceipt,
    ) -> MembershipCoordinatorResult<()> {
        if receipt.operation_sha256() != removal.operation_sha256()
            || receipt.generation() == 0
            || receipt.published_state_sha256() == [0; 32]
        {
            return Err(MembershipCoordinatorError::ReceiptMismatch);
        }
        let projection = DurableAuthorityReceiptProjection::from(receipt);
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, removal.operation_sha256())?;
        set_receipt_once(&mut active.external_receipt, projection)?;
        if active.phase != MembershipCoordinatorPhase::AuthorizedOld {
            return Ok(());
        }
        advance_phase(
            active,
            MembershipCoordinatorPhase::ExternalAuthorityObserved,
        )?;
        self.commit(next)
    }

    fn observe_external_authority(
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
        if active.phase != MembershipCoordinatorPhase::AuthorizedOld {
            return Ok(());
        }
        advance_phase(
            active,
            MembershipCoordinatorPhase::ExternalAuthorityObserved,
        )?;
        self.commit(next)
    }

    fn record_learner_caught_up(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        self.advance(
            operation_sha256,
            MembershipCoordinatorPhase::LearnerCaughtUp,
        )
    }

    fn record_joint_membership(
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

    fn record_final_membership(
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

    fn record_committed_membership_history(
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
        if self.active_phase() == Some(MembershipCoordinatorPhase::LearnerCaughtUp) {
            self.record_joint_membership(operation_sha256, joint_sha256)?;
        }
        self.crash_at(MembershipCrashBoundary::AfterJointHistory)?;
        if self.active_phase() == Some(MembershipCoordinatorPhase::JointCommitted) {
            self.record_final_membership(operation_sha256, final_sha256)?;
        }
        self.crash_at(MembershipCrashBoundary::AfterFinalHistory)?;
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        match active.final_membership_log_index {
            Some(index) if index != final_entry.log_id.index => {
                return Err(MembershipCoordinatorError::StateRegression);
            }
            Some(_) => {}
            None => active.final_membership_log_index = Some(final_entry.log_id.index),
        }
        next.committed_log_index = final_entry.log_id.index;
        self.commit(next)
    }

    fn mark_membership_change_submitted(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        if active.phase != MembershipCoordinatorPhase::LearnerCaughtUp {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        active.membership_change_submitted = true;
        self.commit(next)
    }

    fn clear_no_effect_membership_change_submission(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<()> {
        let mut next = self.envelope.payload().clone();
        let active = exact_active_mut(&mut next, operation_sha256)?;
        if active.phase != MembershipCoordinatorPhase::LearnerCaughtUp {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        active.membership_change_submitted = false;
        self.commit(next)
    }

    async fn await_committed_membership_history(
        &mut self,
        operation_sha256: [u8; 32],
        state_machine: &PolisStateMachineStore,
        expected_old: &BTreeSet<u64>,
        expected_target: &BTreeSet<u64>,
    ) -> MembershipCoordinatorResult<()> {
        for _ in 0..500 {
            let history = state_machine.applied_membership_history().await;
            let recorded = self.record_committed_membership_history(
                operation_sha256,
                &history,
                expected_old,
                expected_target,
            );
            if recorded.is_ok() {
                return Ok(());
            }
            #[cfg(test)]
            if self.crash_boundary_hit {
                return Err(MembershipCoordinatorError::StateRegression);
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        Err(MembershipCoordinatorError::StateRegression)
    }

    fn reconcile_authority_parity(
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

    fn checkpoint(&mut self, operation_sha256: [u8; 32]) -> MembershipCoordinatorResult<()> {
        self.advance(operation_sha256, MembershipCoordinatorPhase::Checkpointed)
    }

    fn publish(
        &mut self,
        operation_sha256: [u8; 32],
        target_stable_ids: &BTreeMap<Vec<u8>, u64>,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        let mut next = self.envelope.payload().clone();
        if let Some(published) = published_result(&next, operation_sha256) {
            return Ok(published.result_sha256);
        }
        let committed_log_index = next.committed_log_index;
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
        let final_membership_log_index = active
            .final_membership_log_index
            .unwrap_or(committed_log_index);
        next.published_generation = next
            .published_generation
            .checked_add(1)
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        next.published_operation_sha256 = Some(operation_sha256);
        next.published_result_sha256 = Some(result_sha256);
        next.pending_visibility_operation_sha256 = Some(operation_sha256);
        next.stable_id_registry = target_stable_ids
            .iter()
            .map(|(guardian, raft_id)| (guardian.clone(), *raft_id))
            .collect();
        record_published_result(
            &mut next,
            operation_sha256,
            DurablePublishedMembershipResult {
                operation_sha256,
                result_sha256,
                committed_log_index: final_membership_log_index,
            },
        );
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

    async fn promote_voter_with_raft(
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
        self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityObservation)?;
        let final_history_complete =
            self.envelope
                .payload()
                .active
                .as_ref()
                .is_some_and(|active| {
                    active.phase == MembershipCoordinatorPhase::FinalCommitted
                        && active.final_membership_log_index.is_some()
                });
        if final_history_complete
            || matches!(
                self.active_phase(),
                Some(
                    MembershipCoordinatorPhase::AuthorityParityReconciled
                        | MembershipCoordinatorPhase::Checkpointed
                )
            )
        {
            return Ok(());
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::ExternalAuthorityObserved) {
            self.crash_at(MembershipCrashBoundary::BeforeLearnerEffect)?;
            raft.add_learner(
                promotion.identity.stable_raft_id,
                openraft::BasicNode::new(promotion.identity.address.to_string()),
                true,
            )
            .await
            .map_err(|_| MembershipCoordinatorError::StateRegression)?;
            self.record_learner_caught_up(promotion.operation_sha256)?;
            self.crash_at(MembershipCrashBoundary::AfterLearnerEffect)?;
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::LearnerCaughtUp) {
            let history = state_machine.applied_membership_history().await;
            if self
                .record_committed_membership_history(
                    promotion.operation_sha256,
                    &history,
                    &transition.old_membership,
                    &transition.target_membership,
                )
                .is_ok()
            {
                return Ok(());
            }
            let already_submitted = self
                .envelope
                .payload()
                .active
                .as_ref()
                .is_some_and(|active| active.membership_change_submitted);
            if !already_submitted {
                self.mark_membership_change_submitted(promotion.operation_sha256)?;
                #[cfg(test)]
                if self.fail_membership_change_before_submit {
                    self.fail_membership_change_before_submit = false;
                    if raft_membership_is_exact_old(raft, &transition.old_membership) {
                        self.clear_no_effect_membership_change_submission(
                            promotion.operation_sha256,
                        )?;
                    }
                    return Err(MembershipCoordinatorError::StateRegression);
                }
                if raft
                    .change_membership(transition.target_membership.clone(), false)
                    .await
                    .is_err()
                {
                    if raft_membership_is_exact_old(raft, &transition.old_membership) {
                        self.clear_no_effect_membership_change_submission(
                            promotion.operation_sha256,
                        )?;
                    }
                    return Err(MembershipCoordinatorError::StateRegression);
                }
            }
            #[cfg(test)]
            if self.crash_after_membership_change {
                self.crash_after_membership_change = false;
                return Err(MembershipCoordinatorError::StateRegression);
            }
            return self
                .await_committed_membership_history(
                    promotion.operation_sha256,
                    state_machine,
                    &transition.old_membership,
                    &transition.target_membership,
                )
                .await;
        }
        let history = state_machine.applied_membership_history().await;
        self.record_committed_membership_history(
            promotion.operation_sha256,
            &history,
            &transition.old_membership,
            &transition.target_membership,
        )
    }

    /// Governed non-voting enrollment and rejoin entrypoint. Exact retries
    /// re-observe the #202 generation, idempotently repair the local Join
    /// event, and return the same durable published result.
    #[allow(clippy::too_many_arguments)]
    async fn enroll_non_voting_to_published(
        &mut self,
        admission: &VerifiedLearnerAdmission,
        factory: &SecurePolisNetworkFactory,
        now_unix_seconds: i64,
        membership_event_log_index: u64,
        authority: &AuthorityMembership,
        raft: &PolisRaft,
        membership: &mut MembershipState,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        if let Some(published) =
            published_result(self.envelope.payload(), admission.operation_sha256())
        {
            if self.envelope.payload().pending_visibility_operation_sha256
                != Some(admission.operation_sha256())
            {
                return Ok(published.result_sha256);
            }
            factory
                .observe_learner_admission_receipt(admission.operation_sha256())
                .await
                .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
                .ok_or(MembershipCoordinatorError::ReceiptMismatch)?;
            self.crash_at(MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility)?;
            let identity = admission.identity();
            match membership.member(&identity.node_id) {
                Some(member)
                    if member.guardian_id == identity.guardian_id
                        && member.identity_generation == identity.certificate_generation
                        && member.guardian_control_public_key
                            == identity.guardian_control_public_key
                        && member.role == MemberRole::NonVoting => {}
                Some(_) => return Err(MembershipCoordinatorError::WrongIdentity),
                None => apply_local_membership_event(
                    membership,
                    admission.operation_sha256(),
                    published.committed_log_index,
                    enrollment_operation(admission),
                )?,
            }
            self.clear_pending_visibility(admission.operation_sha256())?;
            return Ok(published.result_sha256);
        }
        if self.envelope.payload().active.is_some()
            || self
                .envelope
                .payload()
                .pending_visibility_operation_sha256
                .is_some()
        {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        validate_enrollment_old_parity(membership, authority, raft)?;
        let identity = admission.identity();
        validate_enrollment_candidate(membership, identity)?;
        validate_registry_preserves_authority(
            &self.envelope.payload().stable_id_registry,
            authority,
        )?;
        let old_stable_ids = if self.envelope.payload().stable_id_registry.is_empty() {
            authority.raft_ids.clone()
        } else {
            self.envelope
                .payload()
                .stable_id_registry
                .iter()
                .cloned()
                .collect()
        };
        let prepared_stable_ids = prepare_enrollment_stable_ids(&old_stable_ids, identity)?;
        let target_stable_ids = prepared_stable_ids.target;
        let old_stable_map_sha256 = prepared_stable_ids.old_sha256;
        let target_stable_map_sha256 = prepared_stable_ids.target_sha256;
        self.crash_at(MembershipCrashBoundary::AfterStableMapPreparation)?;
        let mut staged_membership = membership.clone();
        apply_local_membership_event(
            &mut staged_membership,
            admission.operation_sha256(),
            membership_event_log_index,
            enrollment_operation(admission),
        )?;
        self.crash_at(MembershipCrashBoundary::AfterLocalProjectionPrepared)?;
        self.crash_at(MembershipCrashBoundary::BeforeEnrollmentJournal)?;
        match self.envelope.payload().active_enrollment.as_ref() {
            Some(active)
                if active.operation_sha256 == admission.operation_sha256()
                    && active.membership_event_log_index == membership_event_log_index
                    && active.candidate_stable_raft_id == identity.stable_raft_id
                    && active.old_stable_map_sha256 == old_stable_map_sha256
                    && active.target_stable_map_sha256 == target_stable_map_sha256
                    && active.target_stable_ids
                        == target_stable_ids
                            .iter()
                            .map(|(guardian, raft_id)| (guardian.clone(), *raft_id))
                            .collect::<Vec<_>>() => {}
            Some(_) => return Err(MembershipCoordinatorError::StateRegression),
            None => {
                let mut next = self.envelope.payload().clone();
                next.active_enrollment = Some(DurableEnrollment {
                    operation_sha256: admission.operation_sha256(),
                    membership_event_log_index,
                    candidate_stable_raft_id: identity.stable_raft_id,
                    old_stable_map_sha256,
                    target_stable_map_sha256,
                    target_stable_ids: target_stable_ids
                        .iter()
                        .map(|(guardian, raft_id)| (guardian.clone(), *raft_id))
                        .collect(),
                    external_receipt: None,
                });
                self.commit(next)?;
            }
        }
        self.crash_at(MembershipCrashBoundary::AfterEnrollmentJournal)?;
        self.crash_at(MembershipCrashBoundary::BeforeExternalAuthorityCall)?;
        let activated = if let Some(existing) = factory
            .observe_learner_admission_receipt(admission.operation_sha256())
            .await
            .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
        {
            existing
        } else {
            match factory
                .activate_learner_admission(admission, now_unix_seconds)
                .await
            {
                Ok(receipt) => receipt,
                Err(_) => {
                    factory
                        .stage_learner_successor(admission)
                        .await
                        .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?;
                    factory
                        .flip_learner_successor(admission.operation_sha256())
                        .await
                        .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?;
                    factory
                        .observe_learner_admission_receipt(admission.operation_sha256())
                        .await
                        .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
                        .ok_or(MembershipCoordinatorError::ReceiptMismatch)?
                }
            }
        };
        self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityCall)?;
        let observed = factory
            .observe_learner_admission_receipt(admission.operation_sha256())
            .await
            .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
            .ok_or(MembershipCoordinatorError::ReceiptMismatch)?;
        if activated != observed {
            return Err(MembershipCoordinatorError::ReceiptMismatch);
        }
        let projection = DurableAuthorityReceiptProjection::from(&observed);
        let mut observed_state = self.envelope.payload().clone();
        let active_enrollment = observed_state
            .active_enrollment
            .as_mut()
            .filter(|active| active.operation_sha256 == admission.operation_sha256())
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        match active_enrollment.external_receipt.as_ref() {
            Some(current) if current != &projection => {
                return Err(MembershipCoordinatorError::ReceiptMismatch);
            }
            Some(_) => {}
            None => active_enrollment.external_receipt = Some(projection),
        }
        self.commit(observed_state)?;
        self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityObservation)?;
        let membership_bytes = staged_membership
            .snapshot()
            .map_err(|_| MembershipCoordinatorError::StateRegression)?;
        let result_sha256 = <[u8; 32]>::from(Sha256::digest(
            [
                admission.operation_sha256().as_slice(),
                observed.published_state_sha256().as_slice(),
                Sha256::digest(membership_bytes).as_slice(),
                target_stable_map_sha256.as_slice(),
            ]
            .concat(),
        ));
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = membership_event_log_index;
        next.published_generation = next
            .published_generation
            .checked_add(1)
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        next.published_operation_sha256 = Some(admission.operation_sha256());
        next.published_result_sha256 = Some(result_sha256);
        next.stable_id_registry = target_stable_ids.into_iter().collect();
        record_published_result(
            &mut next,
            admission.operation_sha256(),
            DurablePublishedMembershipResult {
                operation_sha256: admission.operation_sha256(),
                result_sha256,
                committed_log_index: membership_event_log_index,
            },
        );
        next.active_enrollment = None;
        next.pending_visibility_operation_sha256 = Some(admission.operation_sha256());
        self.crash_at(MembershipCrashBoundary::BeforeCheckpoint)?;
        self.commit(next)?;
        self.crash_at(MembershipCrashBoundary::AfterCheckpoint)?;
        *membership = staged_membership;
        self.clear_pending_visibility(admission.operation_sha256())?;
        Ok(result_sha256)
    }

    fn clear_pending_visibility(&mut self, operation: [u8; 32]) -> MembershipCoordinatorResult<()> {
        if self.envelope.payload().pending_visibility_operation_sha256 != Some(operation) {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        let mut next = self.envelope.payload().clone();
        next.pending_visibility_operation_sha256 = None;
        self.commit(next)
    }

    /// Production promotion entrypoint. The coordinator obtains both external
    /// observations from the #202 factory itself, executes the standard Raft
    /// path, validates concrete local parity, checkpoints, and publishes.
    #[allow(clippy::too_many_arguments)]
    async fn promote_voter_to_published(
        &mut self,
        promotion: &VerifiedPromoteVoter,
        factory: &SecurePolisNetworkFactory,
        raft: &PolisRaft,
        state_machine: &PolisStateMachineStore,
        transition: &AuthorizedMembershipTransition,
        membership: &mut MembershipState,
        authority: &mut AuthorityMembership,
        candidate_authority: VoterAuthority,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        let retained = published_result(self.envelope.payload(), promotion.operation_sha256);
        if self.envelope.payload().published_operation_sha256 != Some(promotion.operation_sha256) {
            if let Some(published) = retained.as_ref() {
                return Ok(published.result_sha256);
            }
        }
        verify_authorized_transition_inputs(
            promotion,
            &transition.old_stable_ids,
            &transition.target_stable_ids,
            &transition.old_membership,
            &transition.target_membership,
        )?;
        if let Some(published) = retained {
            if ObservedMembershipParity::observe(membership, authority, transition).is_ok() {
                if self.envelope.payload().pending_visibility_operation_sha256
                    == Some(promotion.operation_sha256)
                {
                    self.clear_pending_visibility(promotion.operation_sha256)?;
                }
                return Ok(published.result_sha256);
            }
            observe_old_parity(membership, authority, transition)?;
            let mut staged_membership = membership.clone();
            let mut staged_authority = authority.clone();
            self.reconcile_promotion_states(
                promotion,
                transition,
                &mut staged_membership,
                &mut staged_authority,
                candidate_authority,
            )?;
            ObservedMembershipParity::observe(&staged_membership, &staged_authority, transition)?;
            *membership = staged_membership;
            *authority = staged_authority;
            self.clear_pending_visibility(promotion.operation_sha256)?;
            return Ok(published.result_sha256);
        }
        observe_old_parity(membership, authority, transition)?;
        self.crash_at(MembershipCrashBoundary::BeforeExternalAuthorityCall)?;
        let receipt = factory
            .observe_learner_admission_receipt(promotion.enrollment_operation_sha256)
            .await
            .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
            .ok_or(MembershipCoordinatorError::ReceiptMismatch)?;
        self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityCall)?;
        self.promote_voter_with_raft(promotion, &receipt, raft, state_machine, transition)
            .await?;
        self.crash_at(MembershipCrashBoundary::AfterJointFinalObservation)?;
        let mut staged_membership = membership.clone();
        let mut staged_authority = authority.clone();
        self.reconcile_promotion_states(
            promotion,
            transition,
            &mut staged_membership,
            &mut staged_authority,
            candidate_authority,
        )?;
        self.crash_at(MembershipCrashBoundary::AfterLocalProjectionPrepared)?;
        let current = factory
            .observe_learner_admission_receipt(promotion.enrollment_operation_sha256)
            .await
            .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
            .ok_or(MembershipCoordinatorError::ReceiptMismatch)?;
        let parity =
            ObservedMembershipParity::observe(&staged_membership, &staged_authority, transition)?;
        let result = self.publish_observed_parity(
            promotion.operation_sha256,
            &current,
            &transition.target_stable_ids,
            parity,
        )?;
        self.crash_at(MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility)?;
        *membership = staged_membership;
        *authority = staged_authority;
        self.clear_pending_visibility(promotion.operation_sha256)?;
        Ok(result)
    }

    /// Production removal entrypoint. Pending exclusion is activated through
    /// the #202 factory before `retain=false` membership change, and final
    /// publication requires the exact still-current exclusion receipt and
    /// concrete local parity.
    #[allow(clippy::too_many_arguments)]
    async fn remove_voter_to_published(
        &mut self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        now_unix_seconds: i64,
        factory: &SecurePolisNetworkFactory,
        raft: &PolisRaft,
        state_machine: &PolisStateMachineStore,
        transition: &AuthorizedMembershipTransition,
        current_authority: &AuthorityMembership,
        published_membership: &mut MembershipState,
        published_authority: &mut AuthorityMembership,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        let removal = VerifiedMembershipArtifact::from_published(
            result,
            MembershipDiscriminator::RemoveVoter,
        )
        .map_err(|_| MembershipCoordinatorError::InvalidArtifact)?;
        if removal.identity() != expected_identity
            || removal.voter_cut_sha256() != expected_voter_cut_sha256
        {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        let retained = published_result(self.envelope.payload(), removal.operation_sha256());
        if self.envelope.payload().published_operation_sha256 != Some(removal.operation_sha256()) {
            if let Some(published) = retained.as_ref() {
                return Ok(published.result_sha256);
            }
        }
        if current_authority.raft_ids != transition.old_stable_ids
            || current_authority
                .raft_membership
                .voter_ids()
                .collect::<BTreeSet<_>>()
                != transition.old_membership
        {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        if let Some(published) = retained {
            if ObservedMembershipParity::observe(
                published_membership,
                published_authority,
                transition,
            )
            .is_ok()
            {
                if self.envelope.payload().pending_visibility_operation_sha256
                    == Some(removal.operation_sha256())
                {
                    self.clear_pending_visibility(removal.operation_sha256())?;
                }
                return Ok(published.result_sha256);
            }
            observe_old_parity(published_membership, current_authority, transition)?;
            let mut staged_membership = published_membership.clone();
            let mut staged_authority = published_authority.clone();
            self.reconcile_removal_states(
                &removal,
                transition,
                &mut staged_membership,
                &mut staged_authority,
            )?;
            ObservedMembershipParity::observe(&staged_membership, &staged_authority, transition)?;
            *published_membership = staged_membership;
            *published_authority = staged_authority;
            self.clear_pending_visibility(removal.operation_sha256())?;
            return Ok(published.result_sha256);
        }
        observe_old_parity(published_membership, current_authority, transition)?;
        self.begin_removal(&removal, transition)?;
        if self.active_phase() == Some(MembershipCoordinatorPhase::AuthorizedOld) {
            self.crash_at(MembershipCrashBoundary::BeforeExternalAuthorityCall)?;
            let receipt = factory
                .activate_pending_exclusion(
                    result,
                    expected_identity,
                    expected_voter_cut_sha256,
                    removal.target_membership_sha256(),
                    now_unix_seconds,
                )
                .await
                .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?;
            self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityCall)?;
            self.observe_removal_authority(&removal, &receipt)?;
            self.crash_at(MembershipCrashBoundary::AfterExternalAuthorityObservation)?;
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::ExternalAuthorityObserved) {
            self.record_learner_caught_up(removal.operation_sha256())?;
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::LearnerCaughtUp) {
            let history = state_machine.applied_membership_history().await;
            if self
                .record_committed_membership_history(
                    removal.operation_sha256(),
                    &history,
                    &transition.old_membership,
                    &transition.target_membership,
                )
                .is_err()
            {
                let already_submitted = self
                    .envelope
                    .payload()
                    .active
                    .as_ref()
                    .is_some_and(|active| active.membership_change_submitted);
                if !already_submitted {
                    self.mark_membership_change_submitted(removal.operation_sha256())?;
                    #[cfg(test)]
                    if self.fail_membership_change_before_submit {
                        self.fail_membership_change_before_submit = false;
                        if raft_membership_is_exact_old(raft, &transition.old_membership) {
                            self.clear_no_effect_membership_change_submission(
                                removal.operation_sha256(),
                            )?;
                        }
                        return Err(MembershipCoordinatorError::StateRegression);
                    }
                    if raft
                        .change_membership(transition.target_membership.clone(), false)
                        .await
                        .is_err()
                    {
                        if raft_membership_is_exact_old(raft, &transition.old_membership) {
                            self.clear_no_effect_membership_change_submission(
                                removal.operation_sha256(),
                            )?;
                        }
                        return Err(MembershipCoordinatorError::StateRegression);
                    }
                }
                self.await_committed_membership_history(
                    removal.operation_sha256(),
                    state_machine,
                    &transition.old_membership,
                    &transition.target_membership,
                )
                .await?;
            }
        }
        let final_history_complete =
            self.envelope
                .payload()
                .active
                .as_ref()
                .is_some_and(|active| {
                    active.phase == MembershipCoordinatorPhase::FinalCommitted
                        && active.final_membership_log_index.is_some()
                });
        if !final_history_complete {
            let history = state_machine.applied_membership_history().await;
            self.record_committed_membership_history(
                removal.operation_sha256(),
                &history,
                &transition.old_membership,
                &transition.target_membership,
            )?;
        }
        self.crash_at(MembershipCrashBoundary::AfterJointFinalObservation)?;
        let mut staged_membership = published_membership.clone();
        let mut staged_authority = published_authority.clone();
        self.reconcile_removal_states(
            &removal,
            transition,
            &mut staged_membership,
            &mut staged_authority,
        )?;
        self.crash_at(MembershipCrashBoundary::AfterLocalProjectionPrepared)?;
        let current = factory
            .observe_pending_exclusion_receipt(removal.operation_sha256())
            .await
            .map_err(|_| MembershipCoordinatorError::ReceiptMismatch)?
            .ok_or(MembershipCoordinatorError::ReceiptMismatch)?;
        let parity =
            ObservedMembershipParity::observe(&staged_membership, &staged_authority, transition)?;
        let result = self.publish_observed_parity(
            removal.operation_sha256(),
            &current,
            &transition.target_stable_ids,
            parity,
        )?;
        self.crash_at(MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility)?;
        *published_membership = staged_membership;
        *published_authority = staged_authority;
        self.clear_pending_visibility(removal.operation_sha256())?;
        Ok(result)
    }

    fn final_membership_log_index(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> MembershipCoordinatorResult<u64> {
        let state = self.envelope.payload();
        state
            .active
            .as_ref()
            .filter(|active| active.operation_sha256 == operation_sha256)
            .and_then(|active| active.final_membership_log_index)
            .or_else(|| {
                published_result(state, operation_sha256).map(|entry| entry.committed_log_index)
            })
            .ok_or(MembershipCoordinatorError::StateRegression)
    }

    fn reconcile_promotion_states(
        &mut self,
        promotion: &VerifiedPromoteVoter,
        transition: &AuthorizedMembershipTransition,
        membership: &mut MembershipState,
        authority: &mut AuthorityMembership,
        candidate: VoterAuthority,
    ) -> MembershipCoordinatorResult<()> {
        if authority.raft_ids != transition.old_stable_ids
            || candidate.guardian_id != promotion.identity.guardian_id.as_bytes()
            || candidate.trust_domain_id != promotion.identity.trust_domain.as_bytes()
            || candidate.certificate_generation != promotion.identity.certificate_generation
            || candidate.control_public_key != promotion.identity.guardian_control_public_key
            || candidate.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || candidate.revoked
            || membership
                .member(&promotion.identity.node_id)
                .is_none_or(|member| member.role != MemberRole::NonVoting)
        {
            return Err(MembershipCoordinatorError::WrongIdentity);
        }
        let final_index = self.final_membership_log_index(promotion.operation_sha256)?;
        let mut voters = authority.voters.values().cloned().collect::<Vec<_>>();
        voters.push(candidate);
        let next_authority = build_uniform_authority(authority, transition, voters, final_index)?;
        self.crash_at(MembershipCrashBoundary::AfterStableMapPreparation)?;
        apply_local_membership_event(
            membership,
            promotion.operation_sha256,
            final_index,
            MembershipOperation::Promote {
                node_id: promotion.identity.node_id.clone(),
            },
        )?;
        *authority = next_authority;
        Ok(())
    }

    fn reconcile_removal_states(
        &mut self,
        removal: &VerifiedMembershipArtifact,
        transition: &AuthorizedMembershipTransition,
        membership: &mut MembershipState,
        authority: &mut AuthorityMembership,
    ) -> MembershipCoordinatorResult<()> {
        if authority.raft_ids != transition.old_stable_ids {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        let final_index = self.final_membership_log_index(removal.operation_sha256())?;
        let voters = authority
            .voters
            .values()
            .filter(|voter| voter.guardian_id != removal.identity().guardian_id.as_bytes())
            .cloned()
            .collect();
        let next_authority = build_uniform_authority(authority, transition, voters, final_index)?;
        self.crash_at(MembershipCrashBoundary::AfterStableMapPreparation)?;
        apply_local_membership_event(
            membership,
            removal.operation_sha256(),
            final_index,
            MembershipOperation::Remove {
                node_id: removal.identity().node_id.clone(),
            },
        )?;
        *authority = next_authority;
        Ok(())
    }

    /// Complete publication only from concrete local projections that have
    /// already been checked for exact voter, stable-id, key, certificate,
    /// configuration, and committed-index parity.
    fn publish_observed_parity(
        &mut self,
        operation_sha256: [u8; 32],
        observed_receipt: &GovernedMembershipAuthorityReceipt,
        target_stable_ids: &BTreeMap<Vec<u8>, u64>,
        parity: ObservedMembershipParity,
    ) -> MembershipCoordinatorResult<[u8; 32]> {
        let final_index = self
            .envelope
            .payload()
            .active
            .as_ref()
            .filter(|active| active.operation_sha256 == operation_sha256)
            .and_then(|active| active.final_membership_log_index)
            .ok_or(MembershipCoordinatorError::StateRegression)?;
        if parity.committed_log_index != final_index {
            return Err(MembershipCoordinatorError::StateRegression);
        }
        if self.active_phase() == Some(MembershipCoordinatorPhase::FinalCommitted) {
            self.reconcile_authority_parity(
                operation_sha256,
                observed_receipt,
                parity.membership_sha256,
                parity.authority_sha256,
            )?;
        }
        self.crash_at(MembershipCrashBoundary::AfterParityReconciliation)?;
        if self.active_phase() == Some(MembershipCoordinatorPhase::AuthorityParityReconciled) {
            self.crash_at(MembershipCrashBoundary::BeforeCheckpoint)?;
            self.checkpoint(operation_sha256)?;
        }
        self.crash_at(MembershipCrashBoundary::AfterCheckpoint)?;
        self.publish(operation_sha256, target_stable_ids)
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

    #[cfg(test)]
    fn inject_crash_after_membership_change(&mut self) {
        self.crash_after_membership_change = true;
    }

    #[cfg(test)]
    fn inject_membership_change_no_effect_failure(&mut self) {
        self.fail_membership_change_before_submit = true;
    }

    fn crash_at(&mut self, boundary: MembershipCrashBoundary) -> MembershipCoordinatorResult<()> {
        #[cfg(test)]
        if self.crash_boundary == Some(boundary) {
            self.crash_boundary_hit = true;
            return Err(MembershipCoordinatorError::StateRegression);
        }
        #[cfg(not(test))]
        let _ = boundary;
        Ok(())
    }
}

fn apply_local_membership_event(
    membership: &mut MembershipState,
    event_id: [u8; 32],
    committed_log_index: u64,
    operation: MembershipOperation,
) -> MembershipCoordinatorResult<()> {
    let epoch = membership
        .epoch()
        .checked_add(1)
        .ok_or(MembershipCoordinatorError::StateRegression)?;
    let event = CommittedMembershipEvent::new(
        membership.trust_domain(),
        event_id,
        epoch,
        committed_log_index,
        operation,
    );
    membership
        .apply(&event)
        .map(|_| ())
        .map_err(|_| MembershipCoordinatorError::StateRegression)
}

fn enrollment_operation(admission: &VerifiedLearnerAdmission) -> MembershipOperation {
    let identity = admission.identity();
    MembershipOperation::Join {
        member: Member {
            node_id: identity.node_id.clone(),
            guardian_id: identity.guardian_id.clone(),
            identity_generation: identity.certificate_generation,
            guardian_control_public_key: identity.guardian_control_public_key,
            role: MemberRole::NonVoting,
        },
    }
}

fn prepare_enrollment_stable_ids(
    old_stable_ids: &BTreeMap<Vec<u8>, u64>,
    identity: &LearnerIdentity,
) -> MembershipCoordinatorResult<PreparedEnrollmentStableIds> {
    let mut target_stable_ids = old_stable_ids.clone();
    match target_stable_ids.get(identity.guardian_id.as_bytes()) {
        Some(existing) if *existing != identity.stable_raft_id => {
            return Err(MembershipCoordinatorError::WrongStableMap);
        }
        Some(_) => {}
        None => {
            if target_stable_ids
                .values()
                .any(|raft_id| *raft_id == identity.stable_raft_id)
            {
                return Err(MembershipCoordinatorError::WrongStableMap);
            }
            target_stable_ids.insert(
                identity.guardian_id.as_bytes().to_vec(),
                identity.stable_raft_id,
            );
        }
    }
    Ok(PreparedEnrollmentStableIds {
        old_sha256: stable_map_sha256(old_stable_ids)?,
        target_sha256: stable_map_sha256(&target_stable_ids)?,
        target: target_stable_ids,
    })
}

fn validate_enrollment_old_parity(
    membership: &MembershipState,
    authority: &AuthorityMembership,
    raft: &PolisRaft,
) -> MembershipCoordinatorResult<()> {
    let expected_voters = authority
        .raft_membership
        .voter_ids()
        .collect::<BTreeSet<_>>();
    if authority.raft_membership.get_joint_config().len() != 1
        || membership.trust_domain().as_bytes() != authority.trust_domain_id
        || !raft_membership_is_exact_old(raft, &expected_voters)
        || authority
            .raft_ids
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            != expected_voters
    {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    let voters = membership
        .members()
        .filter(|member| member.role == MemberRole::Voter)
        .map(|member| (member.guardian_id.as_bytes(), member))
        .collect::<BTreeMap<_, _>>();
    if voters.len() != authority.voters.len()
        || authority.voters.iter().any(|(guardian, voter)| {
            voters.get(guardian.as_slice()).is_none_or(|member| {
                authority
                    .raft_ids
                    .get(guardian)
                    .copied()
                    .unwrap_or_default()
                    == 0
                    || member.guardian_control_public_key != voter.control_public_key
                    || member.identity_generation != voter.certificate_generation
            })
        })
    {
        return Err(MembershipCoordinatorError::WrongIdentity);
    }
    Ok(())
}

fn validate_enrollment_candidate(
    membership: &MembershipState,
    identity: &LearnerIdentity,
) -> MembershipCoordinatorResult<()> {
    if membership.member(&identity.node_id).is_some()
        || membership.members().any(|member| {
            member.guardian_id == identity.guardian_id
                || member.guardian_control_public_key == identity.guardian_control_public_key
        })
    {
        return Err(MembershipCoordinatorError::WrongIdentity);
    }
    Ok(())
}

fn validate_registry_preserves_authority(
    registry: &[(Vec<u8>, u64)],
    authority: &AuthorityMembership,
) -> MembershipCoordinatorResult<()> {
    if registry.is_empty() {
        return Ok(());
    }
    let entry_count = registry.len();
    let registry = registry.iter().cloned().collect::<BTreeMap<_, _>>();
    if registry.len() != entry_count {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    stable_map_sha256(&registry)?;
    if registry != authority.raft_ids {
        return Err(MembershipCoordinatorError::WrongStableMap);
    }
    Ok(())
}

fn observe_old_parity(
    membership: &MembershipState,
    authority: &AuthorityMembership,
    transition: &AuthorizedMembershipTransition,
) -> MembershipCoordinatorResult<ObservedMembershipParity> {
    if authority.committed_log_index > membership.committed_log_index() {
        return Err(MembershipCoordinatorError::StateRegression);
    }
    let mut comparable_authority = authority.clone();
    comparable_authority.committed_log_index = membership.committed_log_index();
    ObservedMembershipParity::observe(
        membership,
        &comparable_authority,
        &AuthorizedMembershipTransition {
            old_stable_ids: transition.old_stable_ids.clone(),
            target_stable_ids: transition.old_stable_ids.clone(),
            old_membership: transition.old_membership.clone(),
            target_membership: transition.old_membership.clone(),
        },
    )
}

fn build_uniform_authority(
    current: &AuthorityMembership,
    transition: &AuthorizedMembershipTransition,
    voters: Vec<VoterAuthority>,
    committed_log_index: u64,
) -> MembershipCoordinatorResult<AuthorityMembership> {
    let guardian_config = transition
        .target_stable_ids
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    AuthorityMembership::new_with_stable_ids(
        current.trust_domain_id.clone(),
        current
            .voter_set_generation
            .checked_add(1)
            .ok_or(MembershipCoordinatorError::StateRegression)?,
        committed_log_index,
        vec![guardian_config],
        voters,
        transition.target_stable_ids.clone(),
    )
    .map_err(|_| MembershipCoordinatorError::StateRegression)
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

fn published_result(
    state: &MembershipCoordinatorState,
    operation_sha256: [u8; 32],
) -> Option<DurablePublishedMembershipResult> {
    if let Some(result) = state
        .published_results
        .iter()
        .find(|result| result.operation_sha256 == operation_sha256)
    {
        return Some(result.clone());
    }
    if state.published_operation_sha256 != Some(operation_sha256) {
        return None;
    }
    Some(DurablePublishedMembershipResult {
        operation_sha256,
        result_sha256: state.published_result_sha256?,
        committed_log_index: state.committed_log_index,
    })
}

fn raft_membership_is_exact_old(raft: &PolisRaft, expected_old: &BTreeSet<u64>) -> bool {
    let metrics = raft.metrics();
    let observed = metrics.borrow();
    let configs = observed.membership_config.membership().get_joint_config();
    membership_configs_are_exact_old(configs, expected_old)
}

fn membership_configs_are_exact_old(
    configs: &[BTreeSet<u64>],
    expected_old: &BTreeSet<u64>,
) -> bool {
    configs.len() == 1 && configs[0] == *expected_old
}

fn record_published_result(
    state: &mut MembershipCoordinatorState,
    operation_sha256: [u8; 32],
    result: DurablePublishedMembershipResult,
) {
    if let Some(existing) = state
        .published_results
        .iter_mut()
        .find(|existing| existing.operation_sha256 == operation_sha256)
    {
        *existing = result;
        return;
    }
    state.published_results.push(result);
    if state.published_results.len() > 64 {
        state.published_results.remove(0);
    }
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

fn set_receipt_once(
    slot: &mut Option<DurableAuthorityReceiptProjection>,
    value: DurableAuthorityReceiptProjection,
) -> MembershipCoordinatorResult<()> {
    match slot {
        Some(current) if current == &value => Ok(()),
        Some(_) => Err(MembershipCoordinatorError::ReceiptMismatch),
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

pub fn verify_authorized_removal_inputs(
    removal: &VerifiedMembershipArtifact,
    transition: &AuthorizedMembershipTransition,
) -> MembershipCoordinatorResult<()> {
    let candidate = removal.identity().guardian_id.as_bytes();
    if removal.discriminator() != MembershipDiscriminator::RemoveVoter
        || membership_set_sha256(&transition.target_membership)?
            != removal.target_membership_sha256()
        || transition
            .old_stable_ids
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            != transition.old_membership
        || transition
            .target_stable_ids
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            != transition.target_membership
        || transition.old_stable_ids.get(candidate) != Some(&removal.identity().stable_raft_id)
        || transition.target_stable_ids.contains_key(candidate)
        || transition
            .target_membership
            .contains(&removal.identity().stable_raft_id)
        || transition
            .target_stable_ids
            .iter()
            .any(|(guardian, raft_id)| transition.old_stable_ids.get(guardian) != Some(raft_id))
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
