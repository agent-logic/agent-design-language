//! Authority-bound, replication-only learner transport.
//!
//! This module deliberately keeps learner admission separate from the exact
//! three-voter route cut.  Admission is possible only through the sealed
//! artifact carried by a durably published membership operation.

use std::{
    collections::BTreeMap,
    net::SocketAddr,
    path::Path,
    sync::{Arc, Mutex},
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::distributed::{
    authority_protocol::{
        endorse_committed_authority_prepare_with_exclusion, AuthorityEligibilityExclusion,
        AuthorityIntentEndorsement, AuthorityNodeIdentity, AuthorityOperationKind,
        AuthorityProtocolError, CanonicalAuthorityTime, CommittedAuthorityArtifact,
        PrepareAuthorityIntent, PublishedAuthorityResult,
    },
    identity::{GuardianControlSignerCustody, LocalNodeGuardianIdentity},
    lease::AuthorityMembership,
    membership::MembershipState,
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
        SecureBootGenerationCustody,
    },
    transport::{
        transport_peer_identity_key, AuthenticatedConnection, LearnerEndpointRole,
        LearnerPendingResponse, LearnerReceivedEnvelope, LearnerWireSession,
        OrdinarySessionExclusion, ProductionTransportAuthority, TransportAuthorityOwner,
        TransportAuthorityWriteLease, VerifiedPolisRouteCut,
    },
};

#[cfg(test)]
use super::super::TransportDispatchTestHook;

const MEMBERSHIP_ARTIFACT_SCHEMA: &str = "adl.distributed.learner_membership.v1";
const MEMBERSHIP_ARTIFACT_DOMAIN: &str = "adl.authority-artifact.membership.v1";
const EXCLUSION_OBJECT: &str = "pending-membership-exclusion-v1";
const EXCLUSION_FILE: &str = "pending-membership-exclusion.json";
const ADMISSION_OBJECT: &str = "learner-admission-v1";
const ADMISSION_FILE: &str = "learner-admission.json";
const TRANSPORT_INSTANCE_OBJECT: &str = "transport-authority-instance-v1";
const TRANSPORT_INSTANCE_FILE: &str = "transport-authority-instance.json";
const ROLE: &str = "replication_only_learner";
const LIVE_BINDING_SCHEMA: &str = "adl.distributed.learner_live_binding.v1";
pub const MAX_LEARNER_RPC_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearnerTransportError {
    AuthorityDenied,
    ArtifactMismatch,
    InvalidBinding,
    Expired,
    Replay,
    FrameTooLarge,
    CapacityExceeded,
    Storage,
}

impl std::fmt::Display for LearnerTransportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::AuthorityDenied => "learner_authority_denied",
            Self::ArtifactMismatch => "learner_artifact_mismatch",
            Self::InvalidBinding => "learner_invalid_binding",
            Self::Expired => "learner_admission_expired",
            Self::Replay => "learner_replay_rejected",
            Self::FrameTooLarge => "learner_frame_too_large",
            Self::CapacityExceeded => "learner_capacity_exceeded",
            Self::Storage => "learner_storage_failed",
        })
    }
}

impl std::error::Error for LearnerTransportError {}

impl From<PolisRuntimeError> for LearnerTransportError {
    fn from(error: PolisRuntimeError) -> Self {
        match error {
            PolisRuntimeError::FrameTooLarge => Self::FrameTooLarge,
            PolisRuntimeError::Replay | PolisRuntimeError::StateRegression => Self::Replay,
            _ => Self::Storage,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipDiscriminator {
    EnrollNonVoting,
    RemoveVoter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnerIdentity {
    pub trust_domain: String,
    pub polis_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub guardian_control_public_key: [u8; 32],
    pub stable_raft_id: u64,
    pub certificate_generation: u64,
    pub boot_generation: u64,
    pub address: SocketAddr,
}

impl LearnerIdentity {
    pub(crate) fn validate(&self) -> Result<(), LearnerTransportError> {
        if [
            self.trust_domain.as_str(),
            self.polis_id.as_str(),
            self.node_id.as_str(),
            self.guardian_id.as_str(),
        ]
        .iter()
        .any(|value| value.is_empty() || value.len() > 128 || value.contains('\0'))
            || self.stable_raft_id == 0
            || self.guardian_control_public_key == [0; 32]
            || ed25519_dalek::VerifyingKey::from_bytes(&self.guardian_control_public_key).is_err()
            || self.certificate_generation == 0
            || self.boot_generation == 0
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CanonicalMembershipArtifact {
    schema: String,
    discriminator: MembershipDiscriminator,
    identity: LearnerIdentity,
    voter_cut_sha256: [u8; 32],
    previous_operation_sha256: Option<[u8; 32]>,
    target_membership_sha256: [u8; 32],
    authority_overlap_end_unix_seconds: Option<i64>,
    deadline_unix_seconds: i64,
    reason_code: String,
}

/// Canonical issue-local artifact builder. It does not grant authority; only a
/// matching durably published #201 result can be consumed by the adapter.
pub struct LearnerMembershipArtifact;

impl LearnerMembershipArtifact {
    #[allow(clippy::too_many_arguments)]
    pub fn enroll_non_voting(
        identity: LearnerIdentity,
        voter_cut_sha256: [u8; 32],
        previous_operation_sha256: Option<[u8; 32]>,
        target_membership_sha256: [u8; 32],
        authority_overlap_end_unix_seconds: Option<i64>,
        deadline_unix_seconds: i64,
    ) -> Result<CommittedAuthorityArtifact, LearnerTransportError> {
        Self::build(
            MembershipDiscriminator::EnrollNonVoting,
            identity,
            voter_cut_sha256,
            previous_operation_sha256,
            target_membership_sha256,
            authority_overlap_end_unix_seconds,
            deadline_unix_seconds,
            "governed_learner_admission",
        )
    }

    pub fn remove_voter(
        identity: LearnerIdentity,
        voter_cut_sha256: [u8; 32],
        target_membership_sha256: [u8; 32],
        deadline_unix_seconds: i64,
        reason_code: &str,
    ) -> Result<CommittedAuthorityArtifact, LearnerTransportError> {
        Self::build(
            MembershipDiscriminator::RemoveVoter,
            identity,
            voter_cut_sha256,
            None,
            target_membership_sha256,
            None,
            deadline_unix_seconds,
            reason_code,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn build(
        discriminator: MembershipDiscriminator,
        identity: LearnerIdentity,
        voter_cut_sha256: [u8; 32],
        previous_operation_sha256: Option<[u8; 32]>,
        target_membership_sha256: [u8; 32],
        authority_overlap_end_unix_seconds: Option<i64>,
        deadline_unix_seconds: i64,
        reason_code: &str,
    ) -> Result<CommittedAuthorityArtifact, LearnerTransportError> {
        identity.validate()?;
        if voter_cut_sha256 == [0; 32]
            || target_membership_sha256 == [0; 32]
            || deadline_unix_seconds <= 0
            || reason_code.is_empty()
            || reason_code.len() > 128
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        let bytes = serde_jcs::to_vec(&CanonicalMembershipArtifact {
            schema: MEMBERSHIP_ARTIFACT_SCHEMA.to_owned(),
            discriminator,
            identity,
            voter_cut_sha256,
            previous_operation_sha256,
            target_membership_sha256,
            authority_overlap_end_unix_seconds,
            deadline_unix_seconds,
            reason_code: reason_code.to_owned(),
        })
        .map_err(|_| LearnerTransportError::ArtifactMismatch)?;
        CommittedAuthorityArtifact::new(AuthorityOperationKind::Membership, bytes)
            .map_err(|_| LearnerTransportError::ArtifactMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedMembershipArtifact {
    payload: CanonicalMembershipArtifact,
    publication_identity: AuthorityNodeIdentity,
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
}

impl VerifiedMembershipArtifact {
    pub fn from_published(
        result: &PublishedAuthorityResult,
        discriminator: MembershipDiscriminator,
    ) -> Result<Self, LearnerTransportError> {
        consume_published_membership(result, discriminator)
    }

    pub fn discriminator(&self) -> MembershipDiscriminator {
        self.payload.discriminator
    }

    pub fn identity(&self) -> &LearnerIdentity {
        &self.payload.identity
    }

    pub fn voter_cut_sha256(&self) -> [u8; 32] {
        self.payload.voter_cut_sha256
    }

    pub fn target_membership_sha256(&self) -> [u8; 32] {
        self.payload.target_membership_sha256
    }

    pub fn deadline_unix_seconds(&self) -> i64 {
        self.payload.deadline_unix_seconds
    }

    pub fn operation_sha256(&self) -> [u8; 32] {
        self.operation_sha256
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn publication_identity(&self) -> &AuthorityNodeIdentity {
        &self.publication_identity
    }
}

fn consume_published_membership(
    result: &PublishedAuthorityResult,
    discriminator: MembershipDiscriminator,
) -> Result<VerifiedMembershipArtifact, LearnerTransportError> {
    let operation = result.operation();
    let artifact = operation
        .artifact_for_sealed_consumer()
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    if artifact.domain != MEMBERSHIP_ARTIFACT_DOMAIN
        || artifact.sha256 != <[u8; 32]>::from(Sha256::digest(&artifact.bytes))
    {
        return Err(LearnerTransportError::ArtifactMismatch);
    }
    let payload: CanonicalMembershipArtifact = serde_json::from_slice(&artifact.bytes)
        .map_err(|_| LearnerTransportError::ArtifactMismatch)?;
    if serde_jcs::to_vec(&payload).map_err(|_| LearnerTransportError::ArtifactMismatch)?
        != artifact.bytes
        || payload.schema != MEMBERSHIP_ARTIFACT_SCHEMA
        || payload.discriminator != discriminator
        || operation.committed_log_index() != result.committed_log_index()
    {
        return Err(LearnerTransportError::ArtifactMismatch);
    }
    payload.identity.validate()?;
    Ok(VerifiedMembershipArtifact {
        payload,
        publication_identity: result.authority_identity_for_sealed_consumer().clone(),
        operation_sha256: result.result_sha256(),
        operation_id: result.operation_id().to_owned(),
        committed_log_index: result.committed_log_index(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLearnerAdmission {
    identity: LearnerIdentity,
    publication_identity: AuthorityNodeIdentity,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    previous_operation_sha256: Option<[u8; 32]>,
    target_membership_sha256: [u8; 32],
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
}

impl VerifiedLearnerAdmission {
    pub fn from_published_membership(
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        voter_cut: &VerifiedPolisRouteCut,
        now_unix_seconds: i64,
    ) -> Result<Self, LearnerTransportError> {
        let verified =
            consume_published_membership(result, MembershipDiscriminator::EnrollNonVoting)?;
        let expected_voter_cut_sha256 = route_cut_digest(voter_cut)?;
        if &verified.payload.identity != expected_identity
            || verified.payload.voter_cut_sha256 != expected_voter_cut_sha256
            || !published_identity_matches_cut(
                &verified.publication_identity,
                &verified.payload.identity,
                voter_cut,
            )
            || verified.committed_log_index < voter_cut.committed_membership_index()
            || now_unix_seconds <= 0
            || now_unix_seconds >= verified.payload.deadline_unix_seconds
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(Self {
            identity: verified.payload.identity,
            publication_identity: verified.publication_identity,
            voter_cut_sha256: verified.payload.voter_cut_sha256,
            operation_sha256: verified.operation_sha256,
            operation_id: verified.operation_id,
            previous_operation_sha256: verified.payload.previous_operation_sha256,
            target_membership_sha256: verified.payload.target_membership_sha256,
            committed_log_index: verified.committed_log_index,
            deadline_unix_seconds: verified.payload.deadline_unix_seconds,
            overlap_end_unix_seconds: verified.payload.authority_overlap_end_unix_seconds,
        })
    }

    #[cfg(test)]
    fn from_published_membership_for_test(
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<Self, LearnerTransportError> {
        let verified =
            consume_published_membership(result, MembershipDiscriminator::EnrollNonVoting)?;
        if &verified.payload.identity != expected_identity
            || verified.payload.voter_cut_sha256 != expected_voter_cut_sha256
            || verified.publication_identity.trust_domain != verified.payload.identity.trust_domain
            || verified.publication_identity.polis_id != verified.payload.identity.polis_id
            || now_unix_seconds <= 0
            || now_unix_seconds >= verified.payload.deadline_unix_seconds
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(Self {
            identity: verified.payload.identity,
            publication_identity: verified.publication_identity,
            voter_cut_sha256: verified.payload.voter_cut_sha256,
            operation_sha256: verified.operation_sha256,
            operation_id: verified.operation_id,
            previous_operation_sha256: verified.payload.previous_operation_sha256,
            target_membership_sha256: verified.payload.target_membership_sha256,
            committed_log_index: verified.committed_log_index,
            deadline_unix_seconds: verified.payload.deadline_unix_seconds,
            overlap_end_unix_seconds: verified.payload.authority_overlap_end_unix_seconds,
        })
    }

    pub fn identity(&self) -> &LearnerIdentity {
        &self.identity
    }
    pub fn operation_sha256(&self) -> [u8; 32] {
        self.operation_sha256
    }

    pub(in crate::distributed::transport) fn voter_cut_sha256(&self) -> [u8; 32] {
        self.voter_cut_sha256
    }

    pub fn previous_operation_sha256(&self) -> Option<[u8; 32]> {
        self.previous_operation_sha256
    }

    pub(in crate::distributed::transport) fn target_membership_sha256(&self) -> [u8; 32] {
        self.target_membership_sha256
    }

    pub(in crate::distributed::transport) fn is_live_at(&self, now_unix_seconds: i64) -> bool {
        now_unix_seconds > 0 && now_unix_seconds < self.deadline_unix_seconds
    }

    pub(in crate::distributed::transport) fn matches_route_cut(
        &self,
        voter_cut: &VerifiedPolisRouteCut,
    ) -> bool {
        route_cut_digest(voter_cut).is_ok_and(|digest| digest == self.voter_cut_sha256)
            && published_identity_matches_cut(&self.publication_identity, &self.identity, voter_cut)
            && self.committed_log_index >= voter_cut.committed_membership_index()
    }

    pub(in crate::distributed::transport) fn publication_identity_matches(
        &self,
        polis_id: &str,
        trust_domain: &str,
        authority: &AuthorityMembership,
        node_identities: &BTreeMap<Vec<u8>, (String, u64)>,
    ) -> bool {
        authority.trust_domain_id.as_slice() == trust_domain.as_bytes()
            && self.publication_identity.polis_id == polis_id
            && self.publication_identity.trust_domain == trust_domain
            && authority
                .voters
                .get(self.publication_identity.guardian_id.as_bytes())
                .is_some_and(|voter| {
                    !voter.revoked
                        && node_identities.get(self.publication_identity.guardian_id.as_bytes())
                            == Some(&(
                                self.publication_identity.node_id.clone(),
                                self.publication_identity.boot_generation,
                            ))
                })
    }
}

fn published_identity_matches_cut(
    publication: &AuthorityNodeIdentity,
    learner: &LearnerIdentity,
    cut: &VerifiedPolisRouteCut,
) -> bool {
    publication.trust_domain == learner.trust_domain
        && publication.polis_id == learner.polis_id
        && publication.trust_domain == cut.trust_domain()
        && publication.polis_id == cut.polis_id()
        && cut.routes().keys().any(|raft_id| {
            cut.authority_node_identity(*raft_id).is_some_and(
                |(node_id, guardian_id, boot_generation)| {
                    node_id == publication.node_id
                        && guardian_id == publication.guardian_id
                        && boot_generation == publication.boot_generation
                },
            )
        })
}

#[derive(Clone, Debug)]
pub struct VerifiedPolisLearnerTopology {
    voter_cut: VerifiedPolisRouteCut,
    voter_cut_sha256: [u8; 32],
    admission: Option<VerifiedLearnerAdmission>,
}

impl VerifiedPolisLearnerTopology {
    pub fn voter_only(voter_cut: VerifiedPolisRouteCut) -> Result<Self, LearnerTransportError> {
        let voter_cut_sha256 = route_cut_digest(&voter_cut)?;
        Ok(Self {
            voter_cut,
            voter_cut_sha256,
            admission: None,
        })
    }

    pub fn admit(
        voter_cut: VerifiedPolisRouteCut,
        admission: VerifiedLearnerAdmission,
    ) -> Result<Self, LearnerTransportError> {
        let voter_cut_sha256 = route_cut_digest(&voter_cut)?;
        if admission.voter_cut_sha256 != voter_cut_sha256
            || !admission.matches_route_cut(&voter_cut)
            || voter_cut.contains(admission.identity.stable_raft_id)
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(Self {
            voter_cut,
            voter_cut_sha256,
            admission: Some(admission),
        })
    }

    pub fn voter_routes(&self) -> BTreeMap<u64, SocketAddr> {
        self.voter_cut.routes()
    }
    pub fn voter_cut_sha256(&self) -> [u8; 32] {
        self.voter_cut_sha256
    }
    pub fn learner_route(&self) -> Option<(u64, SocketAddr)> {
        self.admission.as_ref().map(|admission| {
            (
                admission.identity.stable_raft_id,
                admission.identity.address,
            )
        })
    }
}

pub(in crate::distributed::transport) fn route_cut_digest(
    cut: &VerifiedPolisRouteCut,
) -> Result<[u8; 32], LearnerTransportError> {
    let authority_bindings = cut
        .routes()
        .keys()
        .map(|node| {
            let (node_id, guardian_id, boot_generation) = cut
                .authority_node_identity(*node)
                .ok_or(LearnerTransportError::InvalidBinding)?;
            let voter = cut
                .authority_membership()
                .voters
                .get(guardian_id.as_bytes())
                .cloned()
                .ok_or(LearnerTransportError::InvalidBinding)?;
            Ok((*node, node_id, guardian_id, boot_generation, voter))
        })
        .collect::<Result<Vec<_>, LearnerTransportError>>()?;
    let bytes = serde_jcs::to_vec(&(
        cut.polis_id(),
        cut.trust_domain(),
        cut.membership_epoch(),
        cut.committed_membership_index(),
        cut.routes(),
        authority_bindings,
    ))
    .map_err(|_| LearnerTransportError::InvalidBinding)?;
    Ok(Sha256::digest(bytes).into())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearnerRpcKind {
    AppendEntries,
    InstallSnapshot,
}

impl LearnerRpcKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::AppendEntries => "append_entries",
            Self::InstallSnapshot => "install_snapshot",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LearnerSessionBinding {
    identity: LearnerIdentity,
    voter: LearnerVoterBinding,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    target_membership_sha256: [u8; 32],
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
    role: &'static str,
    protocol_version: u32,
}

impl LearnerSessionBinding {
    fn from_admission(
        admission: &VerifiedLearnerAdmission,
        voter_cut_sha256: [u8; 32],
        voter: LearnerVoterBinding,
    ) -> Self {
        Self {
            identity: admission.identity.clone(),
            voter,
            voter_cut_sha256,
            operation_sha256: admission.operation_sha256,
            operation_id: admission.operation_id.clone(),
            target_membership_sha256: admission.target_membership_sha256,
            committed_log_index: admission.committed_log_index,
            deadline_unix_seconds: admission.deadline_unix_seconds,
            overlap_end_unix_seconds: admission.overlap_end_unix_seconds,
            role: ROLE,
            protocol_version: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::distributed::transport) struct LearnerVoterBinding {
    pub(in crate::distributed::transport) stable_raft_id: u64,
    pub(in crate::distributed::transport) node_id: String,
    pub(in crate::distributed::transport) guardian_id: String,
    pub(in crate::distributed::transport) certificate_generation: u64,
    pub(in crate::distributed::transport) boot_generation: u64,
    pub(in crate::distributed::transport) control_public_key: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct EstablishedLearnerSession {
    binding: LearnerSessionBinding,
    endpoint_role: LearnerEndpointRole,
    authority: ProductionLearnerAuthority,
    highest_sequence: u64,
    closed: bool,
    live_binding_sha256: Option<[u8; 32]>,
    wire_session: LearnerWireSession,
    peer_authority_instance_id: Option<[u8; 32]>,
}

pub struct LearnerBootAttestationCustody {
    source: LearnerBootAttestationSource,
}

enum LearnerBootAttestationSource {
    Production {
        boot: SecureBootGenerationCustody,
        signer: GuardianControlSignerCustody,
    },
    #[cfg(test)]
    Test {
        generation: u64,
        signing_key: SigningKey,
    },
}

impl LearnerBootAttestationCustody {
    pub(in crate::distributed::transport) fn establish(
        boot: SecureBootGenerationCustody,
        identity: &LocalNodeGuardianIdentity,
        expected: &LearnerIdentity,
    ) -> Result<Self, LearnerTransportError> {
        boot.require_current()
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let public = identity.public_identity();
        if boot.node_id() != expected.stable_raft_id
            || boot.generation() != expected.boot_generation
            || public.node_id != expected.node_id
            || public.guardian_id != expected.guardian_id
            || public.guardian_control_public_key != expected.guardian_control_public_key
            || public.identity_generation != expected.certificate_generation
        {
            return Err(LearnerTransportError::AuthorityDenied);
        }
        Ok(Self {
            source: LearnerBootAttestationSource::Production {
                boot,
                signer: identity.authority_signer_custody(),
            },
        })
    }

    #[cfg(test)]
    pub(crate) fn for_test(generation: u64, signing_key: &SigningKey) -> Self {
        Self {
            source: LearnerBootAttestationSource::Test {
                generation,
                signing_key: signing_key.clone(),
            },
        }
    }

    fn generation(&self) -> u64 {
        match &self.source {
            LearnerBootAttestationSource::Production { boot, .. } => boot.generation(),
            #[cfg(test)]
            LearnerBootAttestationSource::Test { generation, .. } => *generation,
        }
    }

    fn verifying_key(&self) -> VerifyingKey {
        match &self.source {
            LearnerBootAttestationSource::Production { signer, .. } => signer.verifying_key(),
            #[cfg(test)]
            LearnerBootAttestationSource::Test { signing_key, .. } => signing_key.verifying_key(),
        }
    }

    fn sign(&self, payload: &[u8]) -> Result<Signature, LearnerTransportError> {
        match &self.source {
            LearnerBootAttestationSource::Production { boot, signer } => boot
                .with_current(|| signer.sign(payload))
                .map_err(|_| LearnerTransportError::AuthorityDenied),
            #[cfg(test)]
            LearnerBootAttestationSource::Test { signing_key, .. } => Ok(signing_key.sign(payload)),
        }
    }

    pub(crate) fn require_current(&self) -> Result<(), LearnerTransportError> {
        match &self.source {
            LearnerBootAttestationSource::Production { boot, .. } => boot
                .require_current()
                .map_err(|_| LearnerTransportError::AuthorityDenied),
            #[cfg(test)]
            LearnerBootAttestationSource::Test { .. } => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LearnerLiveBinding {
    schema: String,
    phase: String,
    binding_sha256: [u8; 32],
    challenge: [u8; 32],
    node_id: String,
    guardian_id: String,
    boot_generation: u64,
    sender_authority_instance_id: [u8; 32],
    receiver_authority_instance_id: [u8; 32],
    signature: Vec<u8>,
}

#[derive(Serialize)]
struct LearnerLiveBindingPayload<'a> {
    schema: &'a str,
    phase: &'a str,
    binding_sha256: [u8; 32],
    challenge: [u8; 32],
    node_id: &'a str,
    guardian_id: &'a str,
    boot_generation: u64,
    sender_authority_instance_id: [u8; 32],
    receiver_authority_instance_id: [u8; 32],
}

fn learner_binding_sha256(
    binding: &LearnerSessionBinding,
) -> Result<[u8; 32], LearnerTransportError> {
    let bytes = serde_jcs::to_vec(&(
        &binding.identity,
        binding.voter.stable_raft_id,
        &binding.voter.node_id,
        &binding.voter.guardian_id,
        binding.voter.certificate_generation,
        binding.voter.boot_generation,
        binding.voter.control_public_key,
        binding.voter_cut_sha256,
        binding.operation_sha256,
        &binding.operation_id,
        binding.target_membership_sha256,
        binding.committed_log_index,
        binding.deadline_unix_seconds,
        binding.overlap_end_unix_seconds,
        binding.role,
        binding.protocol_version,
    ))
    .map_err(|_| LearnerTransportError::InvalidBinding)?;
    Ok(Sha256::digest(bytes).into())
}

#[allow(clippy::too_many_arguments)]
fn signed_live_binding(
    phase: &str,
    binding_sha256: [u8; 32],
    challenge: [u8; 32],
    node_id: &str,
    guardian_id: &str,
    boot_generation: u64,
    sender_authority_instance_id: [u8; 32],
    receiver_authority_instance_id: Option<[u8; 32]>,
    signing_key: &SigningKey,
) -> Result<LearnerLiveBinding, LearnerTransportError> {
    let bytes = serde_jcs::to_vec(&LearnerLiveBindingPayload {
        schema: LIVE_BINDING_SCHEMA,
        phase,
        binding_sha256,
        challenge,
        node_id,
        guardian_id,
        boot_generation,
        sender_authority_instance_id,
        receiver_authority_instance_id: receiver_authority_instance_id.unwrap_or([0; 32]),
    })
    .map_err(|_| LearnerTransportError::InvalidBinding)?;
    Ok(LearnerLiveBinding {
        schema: LIVE_BINDING_SCHEMA.to_owned(),
        phase: phase.to_owned(),
        binding_sha256,
        challenge,
        node_id: node_id.to_owned(),
        guardian_id: guardian_id.to_owned(),
        boot_generation,
        sender_authority_instance_id,
        receiver_authority_instance_id: receiver_authority_instance_id.unwrap_or([0; 32]),
        signature: signing_key.sign(&bytes).to_bytes().to_vec(),
    })
}

#[allow(clippy::too_many_arguments)]
fn attested_live_binding(
    custody: &LearnerBootAttestationCustody,
    phase: &str,
    binding_sha256: [u8; 32],
    challenge: [u8; 32],
    node_id: &str,
    guardian_id: &str,
    sender_authority_instance_id: [u8; 32],
    receiver_authority_instance_id: [u8; 32],
) -> Result<LearnerLiveBinding, LearnerTransportError> {
    custody.require_current()?;
    let boot_generation = custody.generation();
    let payload = LearnerLiveBindingPayload {
        schema: LIVE_BINDING_SCHEMA,
        phase,
        binding_sha256,
        challenge,
        node_id,
        guardian_id,
        boot_generation,
        sender_authority_instance_id,
        receiver_authority_instance_id,
    };
    let bytes = serde_jcs::to_vec(&payload).map_err(|_| LearnerTransportError::InvalidBinding)?;
    custody.require_current()?;
    Ok(LearnerLiveBinding {
        schema: LIVE_BINDING_SCHEMA.to_owned(),
        phase: phase.to_owned(),
        binding_sha256,
        challenge,
        node_id: node_id.to_owned(),
        guardian_id: guardian_id.to_owned(),
        boot_generation,
        sender_authority_instance_id,
        receiver_authority_instance_id,
        signature: custody.sign(&bytes)?.to_bytes().to_vec(),
    })
}

#[allow(clippy::too_many_arguments)]
fn verify_live_binding(
    value: &LearnerLiveBinding,
    phase: &str,
    binding_sha256: [u8; 32],
    challenge: [u8; 32],
    node_id: &str,
    guardian_id: &str,
    boot_generation: u64,
    expected_sender_authority_instance_id: Option<[u8; 32]>,
    receiver_authority_instance_id: [u8; 32],
    control_public_key: [u8; 32],
) -> Result<(), LearnerTransportError> {
    if value.schema != LIVE_BINDING_SCHEMA
        || value.phase != phase
        || value.binding_sha256 != binding_sha256
        || value.challenge != challenge
        || value.node_id != node_id
        || value.guardian_id != guardian_id
        || value.boot_generation != boot_generation
        || value.sender_authority_instance_id == [0; 32]
        || expected_sender_authority_instance_id
            .is_some_and(|expected| expected != value.sender_authority_instance_id)
        || (value.receiver_authority_instance_id != [0; 32]
            && value.receiver_authority_instance_id != receiver_authority_instance_id)
    {
        return Err(LearnerTransportError::InvalidBinding);
    }
    let bytes = serde_jcs::to_vec(&LearnerLiveBindingPayload {
        schema: LIVE_BINDING_SCHEMA,
        phase,
        binding_sha256,
        challenge,
        node_id,
        guardian_id,
        boot_generation,
        sender_authority_instance_id: value.sender_authority_instance_id,
        receiver_authority_instance_id: value.receiver_authority_instance_id,
    })
    .map_err(|_| LearnerTransportError::InvalidBinding)?;
    let key = VerifyingKey::from_bytes(&control_public_key)
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
    let signature = Signature::from_slice(&value.signature)
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
    key.verify(&bytes, &signature)
        .map_err(|_| LearnerTransportError::AuthorityDenied)
}

#[derive(Serialize)]
struct LearnerRpcBinding<'a> {
    trust_domain: &'a str,
    polis_id: &'a str,
    node_id: &'a str,
    guardian_id: &'a str,
    voter_node_id: &'a str,
    voter_guardian_id: &'a str,
    voter_stable_raft_id: u64,
    voter_certificate_generation: u64,
    voter_boot_generation: u64,
    stable_raft_id: u64,
    certificate_generation: u64,
    boot_generation: u64,
    address: String,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: &'a str,
    target_membership_sha256: [u8; 32],
    committed_log_index: u64,
    role: &'a str,
    protocol_version: u32,
    sequence: u64,
    message_kind: &'a str,
    payload_sha256: [u8; 32],
    deadline_unix_seconds: i64,
}

impl EstablishedLearnerSession {
    #[cfg(test)]
    pub(in crate::distributed::transport) async fn pause_after_revalidation_for_test(
        &self,
        phase: &'static str,
    ) {
        self.authority
            .pause_after_revalidation_for_test(phase)
            .await;
    }

    pub(in crate::distributed::transport) fn peer_transport_instance(
        &self,
    ) -> Option<(&str, [u8; 32])> {
        self.peer_authority_instance_id
            .map(|instance| (self.wire_session.peer_identity_key(), instance))
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::distributed::transport) fn new(
        admission: &VerifiedLearnerAdmission,
        voter_cut_sha256: [u8; 32],
        voter: LearnerVoterBinding,
        endpoint_role: LearnerEndpointRole,
        authority: ProductionLearnerAuthority,
        now_unix_seconds: i64,
    ) -> Result<Self, LearnerTransportError> {
        if !authority.admission_is_current(admission)?
            || now_unix_seconds <= 0
            || now_unix_seconds >= admission.deadline_unix_seconds
        {
            return Err(LearnerTransportError::AuthorityDenied);
        }
        let (peer_role, peer_raft_id, peer_node_id, peer_guardian_id) = match endpoint_role {
            LearnerEndpointRole::Voter => (
                LearnerEndpointRole::Learner,
                admission.identity.stable_raft_id,
                admission.identity.node_id.as_str(),
                admission.identity.guardian_id.as_str(),
            ),
            LearnerEndpointRole::Learner => (
                LearnerEndpointRole::Voter,
                voter.stable_raft_id,
                voter.node_id.as_str(),
                voter.guardian_id.as_str(),
            ),
        };
        let peer_identity_key =
            transport_peer_identity_key(peer_role, peer_raft_id, peer_node_id, peer_guardian_id)
                .map_err(|_| LearnerTransportError::InvalidBinding)?;
        let wire_session = authority
            .transport_authority()
            .learner_wire_session(
                admission.voter_cut_sha256(),
                admission.operation_sha256(),
                peer_identity_key,
            )
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let session = Self {
            binding: LearnerSessionBinding::from_admission(admission, voter_cut_sha256, voter),
            endpoint_role,
            authority,
            highest_sequence: 0,
            closed: false,
            live_binding_sha256: None,
            wire_session,
            peer_authority_instance_id: None,
        };
        #[cfg(test)]
        {
            let mut session = session;
            session.establish_live_binding_for_test();
            Ok(session)
        }
        #[cfg(not(test))]
        {
            Ok(session)
        }
    }

    fn establish_live_binding(
        &mut self,
        binding_sha256: [u8; 32],
    ) -> Result<(), LearnerTransportError> {
        if learner_binding_sha256(&self.binding)? != binding_sha256 {
            return Err(LearnerTransportError::InvalidBinding);
        }
        self.live_binding_sha256 = Some(binding_sha256);
        Ok(())
    }

    #[cfg(test)]
    fn establish_live_binding_for_test(&mut self) {
        let digest = learner_binding_sha256(&self.binding).expect("test learner binding digest");
        self.establish_live_binding(digest)
            .expect("test learner live binding");
    }

    pub fn authorize(
        &mut self,
        kind: LearnerRpcKind,
        sequence: u64,
        payload: &[u8],
        now_unix_seconds: i64,
    ) -> Result<[u8; 32], LearnerTransportError> {
        if self.closed || self.live_binding_sha256 != learner_binding_sha256(&self.binding).ok() {
            return Err(LearnerTransportError::AuthorityDenied);
        }
        if !self.authority.binding_is_current(&self.binding)? {
            self.closed = true;
            return Err(LearnerTransportError::AuthorityDenied);
        }
        let deadline = self
            .binding
            .overlap_end_unix_seconds
            .map_or(self.binding.deadline_unix_seconds, |overlap| {
                overlap.min(self.binding.deadline_unix_seconds)
            });
        if now_unix_seconds >= deadline {
            self.closed = true;
            return Err(LearnerTransportError::Expired);
        }
        if payload.len() > MAX_LEARNER_RPC_BYTES {
            return Err(LearnerTransportError::FrameTooLarge);
        }
        if sequence == 0 || sequence <= self.highest_sequence {
            return Err(LearnerTransportError::Replay);
        }
        self.highest_sequence = sequence;
        let kind = kind.as_str();
        let bytes = serde_jcs::to_vec(&LearnerRpcBinding {
            trust_domain: &self.binding.identity.trust_domain,
            polis_id: &self.binding.identity.polis_id,
            node_id: &self.binding.identity.node_id,
            guardian_id: &self.binding.identity.guardian_id,
            voter_node_id: &self.binding.voter.node_id,
            voter_guardian_id: &self.binding.voter.guardian_id,
            voter_stable_raft_id: self.binding.voter.stable_raft_id,
            voter_certificate_generation: self.binding.voter.certificate_generation,
            voter_boot_generation: self.binding.voter.boot_generation,
            stable_raft_id: self.binding.identity.stable_raft_id,
            certificate_generation: self.binding.identity.certificate_generation,
            boot_generation: self.binding.identity.boot_generation,
            address: self.binding.identity.address.to_string(),
            voter_cut_sha256: self.binding.voter_cut_sha256,
            operation_sha256: self.binding.operation_sha256,
            operation_id: &self.binding.operation_id,
            target_membership_sha256: self.binding.target_membership_sha256,
            committed_log_index: self.binding.committed_log_index,
            role: self.binding.role,
            protocol_version: self.binding.protocol_version,
            sequence,
            message_kind: kind,
            payload_sha256: Sha256::digest(payload).into(),
            deadline_unix_seconds: deadline,
        })
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
        Ok(Sha256::digest(bytes).into())
    }

    pub fn close(&mut self) {
        self.closed = true;
    }
    pub fn vote(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn client_write(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn authority_endorse(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn authority_finalize(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn mutation(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn renewal(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn shepherd(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }
    pub fn observatory(&self) -> Result<(), LearnerTransportError> {
        Err(LearnerTransportError::AuthorityDenied)
    }

    fn require_connection(
        &self,
        connection: &AuthenticatedConnection,
    ) -> Result<(), LearnerTransportError> {
        let identity = &self.binding.identity;
        if connection.matches_learner_route(
            self.endpoint_role,
            &identity.trust_domain,
            &self.binding.voter.node_id,
            &self.binding.voter.guardian_id,
            self.binding.voter.certificate_generation,
            &identity.node_id,
            &identity.guardian_id,
            identity.certificate_generation,
            identity.address,
        ) {
            Ok(())
        } else {
            Err(LearnerTransportError::InvalidBinding)
        }
    }

    pub(in crate::distributed::transport) fn validate_connection(
        &self,
        connection: &AuthenticatedConnection,
    ) -> Result<(), LearnerTransportError> {
        self.require_connection(connection)
    }
}

pub(in crate::distributed::transport) async fn establish_voter_learner_sessions(
    connection: &AuthenticatedConnection,
    outbound: &mut EstablishedLearnerSession,
    responses: &mut EstablishedLearnerSession,
    signing_key: &SigningKey,
) -> Result<(), LearnerTransportError> {
    if outbound.binding != responses.binding
        || signing_key.verifying_key().to_bytes() != outbound.binding.voter.control_public_key
    {
        return Err(LearnerTransportError::InvalidBinding);
    }
    outbound.require_connection(connection)?;
    responses.require_connection(connection)?;
    if !outbound.authority.binding_is_current(&outbound.binding)? {
        return Err(LearnerTransportError::AuthorityDenied);
    }
    let binding_sha256 = learner_binding_sha256(&outbound.binding)?;
    let mut challenge = [0_u8; 32];
    OsRng.fill_bytes(&mut challenge);
    let request = signed_live_binding(
        "voter_request",
        binding_sha256,
        challenge,
        &outbound.binding.voter.node_id,
        &outbound.binding.voter.guardian_id,
        outbound.binding.voter.boot_generation,
        outbound.wire_session.instance_id(),
        outbound.wire_session.expected_peer_instance_id(),
        signing_key,
    )?;
    let request = serde_jcs::to_vec(&request).map_err(|_| LearnerTransportError::InvalidBinding)?;
    let permit = outbound
        .wire_session
        .initiate_handshake_permit(request)
        .await
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    let response_bytes = connection
        .initiate_learner_handshake(permit)
        .await
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    let response: LearnerLiveBinding = serde_json::from_slice(&response_bytes)
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
    if serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)?
        != response_bytes
    {
        return Err(LearnerTransportError::InvalidBinding);
    }
    verify_live_binding(
        &response,
        "learner_response",
        binding_sha256,
        challenge,
        &outbound.binding.identity.node_id,
        &outbound.binding.identity.guardian_id,
        outbound.binding.identity.boot_generation,
        outbound.wire_session.expected_peer_instance_id(),
        outbound.wire_session.instance_id(),
        outbound.binding.identity.guardian_control_public_key,
    )?;
    outbound.peer_authority_instance_id = Some(response.sender_authority_instance_id);
    responses.peer_authority_instance_id = Some(response.sender_authority_instance_id);
    outbound.establish_live_binding(binding_sha256)?;
    responses.establish_live_binding(binding_sha256)
}

pub(in crate::distributed::transport) async fn establish_learner_voter_sessions(
    connection: &AuthenticatedConnection,
    inbound: &mut EstablishedLearnerSession,
    outbound: &mut EstablishedLearnerSession,
    custody: &LearnerBootAttestationCustody,
) -> Result<(), LearnerTransportError> {
    custody.require_current()?;
    if custody.generation() != inbound.binding.identity.boot_generation {
        return Err(LearnerTransportError::AuthorityDenied);
    }
    if inbound.binding != outbound.binding
        || custody.verifying_key().to_bytes()
            != inbound.binding.identity.guardian_control_public_key
    {
        return Err(LearnerTransportError::InvalidBinding);
    }
    inbound.require_connection(connection)?;
    outbound.require_connection(connection)?;
    if !inbound.authority.binding_is_current(&inbound.binding)? {
        return Err(LearnerTransportError::AuthorityDenied);
    }
    let binding_sha256 = learner_binding_sha256(&inbound.binding)?;
    let binding = inbound.binding.clone();
    let permit = inbound
        .wire_session
        .accept_handshake_permit()
        .await
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    let pending = connection
        .accept_learner_handshake(permit)
        .await
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    let response: Result<Vec<u8>, LearnerTransportError> = {
        let request_bytes = pending.request();
        let request: LearnerLiveBinding = serde_json::from_slice(request_bytes)
            .map_err(|_| LearnerTransportError::InvalidBinding)?;
        if serde_jcs::to_vec(&request).map_err(|_| LearnerTransportError::InvalidBinding)?
            != request_bytes
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        verify_live_binding(
            &request,
            "voter_request",
            binding_sha256,
            request.challenge,
            &binding.voter.node_id,
            &binding.voter.guardian_id,
            binding.voter.boot_generation,
            inbound.wire_session.expected_peer_instance_id(),
            inbound.wire_session.instance_id(),
            binding.voter.control_public_key,
        )
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
        let response = attested_live_binding(
            custody,
            "learner_response",
            binding_sha256,
            request.challenge,
            &binding.identity.node_id,
            &binding.identity.guardian_id,
            inbound.wire_session.instance_id(),
            request.sender_authority_instance_id,
        )
        .map_err(|_| LearnerTransportError::InvalidBinding)?;
        serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)
    };
    let peer_instance_id = serde_json::from_slice::<LearnerLiveBinding>(pending.request())
        .map_err(|_| LearnerTransportError::InvalidBinding)?
        .sender_authority_instance_id;
    let response = response?;
    connection
        .respond_learner_handshake(pending, response)
        .await
        .map_err(|_| LearnerTransportError::AuthorityDenied)?;
    inbound.peer_authority_instance_id = Some(peer_instance_id);
    outbound.peer_authority_instance_id = Some(peer_instance_id);
    inbound.establish_live_binding(binding_sha256)?;
    outbound.establish_live_binding(binding_sha256)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnerReplicationFrame {
    schema: String,
    sequence: u64,
    message_kind: String,
    authorization_sha256: [u8; 32],
    payload_sha256: [u8; 32],
    payload: Vec<u8>,
}

pub struct AuthorizedLearnerRequest {
    frame: LearnerReplicationFrame,
    received: LearnerReceivedEnvelope,
}

impl AuthorizedLearnerRequest {
    pub fn sequence(&self) -> u64 {
        self.frame.sequence()
    }

    pub fn message_kind(&self) -> &str {
        self.frame.message_kind()
    }

    pub fn payload(&self) -> &[u8] {
        self.frame.payload()
    }
}

impl LearnerReplicationFrame {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn message_kind(&self) -> &str {
        &self.message_kind
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LearnerReplicationResponse {
    schema: String,
    sequence: u64,
    request_authorization_sha256: [u8; 32],
    response_authorization_sha256: [u8; 32],
    payload_sha256: [u8; 32],
    payload: Vec<u8>,
}

pub(in crate::distributed::transport) struct PendingLearnerRpcResponse {
    transport: LearnerPendingResponse,
    kind: LearnerRpcKind,
    sequence: u64,
    request_authorization_sha256: [u8; 32],
}

const LEARNER_REPLICATION_FRAME_SCHEMA: &str = "adl.distributed.learner_replication_frame.v1";
const LEARNER_REPLICATION_RESPONSE_SCHEMA: &str = "adl.distributed.learner_replication_response.v1";

impl EstablishedLearnerSession {
    async fn send_replication(
        &mut self,
        connection: &AuthenticatedConnection,
        kind: LearnerRpcKind,
        sequence: u64,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<PendingLearnerRpcResponse, LearnerTransportError> {
        self.require_connection(connection)?;
        let authorization_sha256 = self.authorize(kind, sequence, &payload, now_unix_seconds)?;
        let frame = LearnerReplicationFrame {
            schema: LEARNER_REPLICATION_FRAME_SCHEMA.to_owned(),
            sequence,
            message_kind: kind.as_str().to_owned(),
            authorization_sha256,
            payload_sha256: Sha256::digest(&payload).into(),
            payload,
        };
        let bytes = serde_jcs::to_vec(&frame).map_err(|_| LearnerTransportError::InvalidBinding)?;
        let permit = self
            .wire_session
            .send_permit(sequence, bytes)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let transport = connection
            .dispatch_learner(permit)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        Ok(PendingLearnerRpcResponse {
            transport,
            kind,
            sequence,
            request_authorization_sha256: authorization_sha256,
        })
    }

    pub(in crate::distributed::transport) async fn send_append_entries(
        &mut self,
        connection: &AuthenticatedConnection,
        sequence: u64,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<PendingLearnerRpcResponse, LearnerTransportError> {
        self.send_replication(
            connection,
            LearnerRpcKind::AppendEntries,
            sequence,
            payload,
            now_unix_seconds,
        )
        .await
    }

    pub(in crate::distributed::transport) async fn send_install_snapshot(
        &mut self,
        connection: &AuthenticatedConnection,
        sequence: u64,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<PendingLearnerRpcResponse, LearnerTransportError> {
        self.send_replication(
            connection,
            LearnerRpcKind::InstallSnapshot,
            sequence,
            payload,
            now_unix_seconds,
        )
        .await
    }

    pub(in crate::distributed::transport) async fn receive_replication(
        &mut self,
        connection: &AuthenticatedConnection,
        now_unix_seconds: i64,
    ) -> Result<AuthorizedLearnerRequest, LearnerTransportError> {
        self.require_connection(connection)?;
        let permit = self
            .wire_session
            .receive_permit()
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let envelope = connection
            .receive_learner(permit)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let frame: LearnerReplicationFrame = serde_json::from_slice(envelope.payload())
            .map_err(|_| LearnerTransportError::InvalidBinding)?;
        if serde_jcs::to_vec(&frame).map_err(|_| LearnerTransportError::InvalidBinding)?
            != envelope.payload()
            || frame.schema != LEARNER_REPLICATION_FRAME_SCHEMA
            || frame.sequence != envelope.sequence()
            || frame.payload_sha256 != <[u8; 32]>::from(Sha256::digest(&frame.payload))
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        let kind = match frame.message_kind.as_str() {
            "append_entries" => LearnerRpcKind::AppendEntries,
            "install_snapshot" => LearnerRpcKind::InstallSnapshot,
            _ => return Err(LearnerTransportError::AuthorityDenied),
        };
        if self.authorize(kind, frame.sequence, &frame.payload, now_unix_seconds)?
            != frame.authorization_sha256
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(AuthorizedLearnerRequest {
            frame,
            received: envelope,
        })
    }

    pub(in crate::distributed::transport) async fn send_response(
        &mut self,
        connection: &AuthenticatedConnection,
        request: AuthorizedLearnerRequest,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<(), LearnerTransportError> {
        self.require_connection(connection)?;
        let kind = match request.frame.message_kind.as_str() {
            "append_entries" => LearnerRpcKind::AppendEntries,
            "install_snapshot" => LearnerRpcKind::InstallSnapshot,
            _ => return Err(LearnerTransportError::AuthorityDenied),
        };
        let response_authorization_sha256 =
            self.authorize(kind, request.frame.sequence, &payload, now_unix_seconds)?;
        let response = LearnerReplicationResponse {
            schema: LEARNER_REPLICATION_RESPONSE_SCHEMA.to_owned(),
            sequence: request.frame.sequence,
            request_authorization_sha256: request.frame.authorization_sha256,
            response_authorization_sha256,
            payload_sha256: Sha256::digest(&payload).into(),
            payload,
        };
        let bytes =
            serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)?;
        let permit = request
            .received
            .response_permit(request.frame.sequence, bytes);
        connection
            .dispatch_learner(permit)
            .await
            .map(drop)
            .map_err(|_| LearnerTransportError::AuthorityDenied)
    }

    pub(in crate::distributed::transport) async fn receive_response(
        &mut self,
        connection: &AuthenticatedConnection,
        pending: PendingLearnerRpcResponse,
        now_unix_seconds: i64,
    ) -> Result<Vec<u8>, LearnerTransportError> {
        self.require_connection(connection)?;
        let envelope = connection
            .receive_learner_response(pending.transport)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let response: LearnerReplicationResponse = serde_json::from_slice(envelope.payload())
            .map_err(|_| LearnerTransportError::InvalidBinding)?;
        if serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)?
            != envelope.payload()
            || response.schema != LEARNER_REPLICATION_RESPONSE_SCHEMA
            || response.sequence != pending.sequence
            || envelope.sequence() != pending.sequence
            || response.request_authorization_sha256 != pending.request_authorization_sha256
            || response.payload_sha256 != <[u8; 32]>::from(Sha256::digest(&response.payload))
            || self.authorize(
                pending.kind,
                pending.sequence,
                &response.payload,
                now_unix_seconds,
            )? != response.response_authorization_sha256
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(response.payload)
    }

    pub fn deny_message_kind(&self, message_kind: &str) -> Result<(), LearnerTransportError> {
        match message_kind {
            "append_entries" | "install_snapshot" => Ok(()),
            _ => Err(LearnerTransportError::AuthorityDenied),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableAdmission {
    identity: LearnerIdentity,
    publication_identity: AuthorityNodeIdentity,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    previous_operation_sha256: Option<[u8; 32]>,
    target_membership_sha256: [u8; 32],
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
}

impl From<&VerifiedLearnerAdmission> for DurableAdmission {
    fn from(value: &VerifiedLearnerAdmission) -> Self {
        Self {
            identity: value.identity.clone(),
            publication_identity: value.publication_identity.clone(),
            voter_cut_sha256: value.voter_cut_sha256,
            operation_sha256: value.operation_sha256,
            operation_id: value.operation_id.clone(),
            previous_operation_sha256: value.previous_operation_sha256,
            target_membership_sha256: value.target_membership_sha256,
            committed_log_index: value.committed_log_index,
            deadline_unix_seconds: value.deadline_unix_seconds,
            overlap_end_unix_seconds: value.overlap_end_unix_seconds,
        }
    }
}

impl From<DurableAdmission> for VerifiedLearnerAdmission {
    fn from(value: DurableAdmission) -> Self {
        Self {
            identity: value.identity,
            publication_identity: value.publication_identity,
            voter_cut_sha256: value.voter_cut_sha256,
            operation_sha256: value.operation_sha256,
            operation_id: value.operation_id,
            previous_operation_sha256: value.previous_operation_sha256,
            target_membership_sha256: value.target_membership_sha256,
            committed_log_index: value.committed_log_index,
            deadline_unix_seconds: value.deadline_unix_seconds,
            overlap_end_unix_seconds: value.overlap_end_unix_seconds,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmissionState {
    committed_log_index: u64,
    current: Option<DurableAdmission>,
    staged_successor: Option<DurableAdmission>,
}

impl CheckpointMetadataSource for AdmissionState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.committed_log_index),
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TransportInstanceState {
    instance_id: [u8; 32],
    peer_instances: BTreeMap<String, [u8; 32]>,
}

impl CheckpointMetadataSource for TransportInstanceState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata::default())
    }
}

struct TransportInstanceAuthority {
    store: CheckpointedJson<TransportInstanceState>,
    envelope: DurableEnvelope<TransportInstanceState>,
}

impl TransportInstanceAuthority {
    fn open(
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, LearnerTransportError> {
        let mut instance_id = [0; 32];
        while instance_id == [0; 32] {
            OsRng.fill_bytes(&mut instance_id);
        }
        let (store, envelope) = CheckpointedJson::open(
            root,
            TRANSPORT_INSTANCE_OBJECT,
            TRANSPORT_INSTANCE_FILE,
            TransportInstanceState {
                instance_id,
                peer_instances: BTreeMap::new(),
            },
            checkpoint,
        )?;
        if envelope.payload().instance_id == [0; 32] {
            return Err(LearnerTransportError::Storage);
        }
        Ok(Self { store, envelope })
    }

    fn instance_id(&self) -> [u8; 32] {
        self.envelope.payload().instance_id
    }

    fn peer_instances(&self) -> BTreeMap<String, [u8; 32]> {
        self.envelope.payload().peer_instances.clone()
    }

    fn pin_peer(
        &mut self,
        guardian_id: &str,
        instance_id: [u8; 32],
    ) -> Result<(), LearnerTransportError> {
        match self.envelope.payload().peer_instances.get(guardian_id) {
            Some(current) if current == &instance_id => return Ok(()),
            Some(_) => return Err(LearnerTransportError::AuthorityDenied),
            None => {}
        }
        let mut next = self.envelope.payload().clone();
        next.peer_instances
            .insert(guardian_id.to_owned(), instance_id);
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearnerAdmissionSnapshot {
    generation: u64,
    current: Option<VerifiedLearnerAdmission>,
}

pub(in crate::distributed::transport) struct MembershipReceiptParts {
    pub operation_sha256: [u8; 32],
    pub generation: u64,
    pub published_state_sha256: [u8; 32],
}

impl LearnerAdmissionSnapshot {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn current(&self) -> Option<&VerifiedLearnerAdmission> {
        self.current.as_ref()
    }

    pub(in crate::distributed::transport) fn membership_receipt_parts(
        &self,
    ) -> Result<Option<MembershipReceiptParts>, LearnerTransportError> {
        self.current
            .as_ref()
            .map(|current| {
                let published_state_sha256 = <[u8; 32]>::from(Sha256::digest(
                    serde_jcs::to_vec(&(
                        "adl.governed-membership-authority.admission.v1",
                        self.generation,
                        current.operation_sha256,
                        current.target_membership_sha256,
                        current.committed_log_index,
                    ))
                    .map_err(|_| LearnerTransportError::Storage)?,
                ));
                Ok(MembershipReceiptParts {
                    operation_sha256: current.operation_sha256,
                    generation: self.generation,
                    published_state_sha256,
                })
            })
            .transpose()
    }
}

/// Durable single-lineage admission publication and successor flip.
pub(crate) struct LearnerAdmissionAuthority {
    store: CheckpointedJson<AdmissionState>,
    envelope: DurableEnvelope<AdmissionState>,
}

impl LearnerAdmissionAuthority {
    pub(crate) fn open(
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, LearnerTransportError> {
        let (store, envelope) = CheckpointedJson::open(
            root,
            ADMISSION_OBJECT,
            ADMISSION_FILE,
            AdmissionState::default(),
            checkpoint,
        )?;
        Ok(Self { store, envelope })
    }

    pub(crate) fn activate(
        &mut self,
        admission: &VerifiedLearnerAdmission,
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        let state = self.envelope.payload();
        if let Some(current) = state.current.as_ref() {
            if current.operation_sha256 == admission.operation_sha256 {
                return Ok(self.snapshot());
            }
            return Err(LearnerTransportError::CapacityExceeded);
        }
        let mut next = state.clone();
        next.committed_log_index = admission.committed_log_index;
        next.current = Some(admission.into());
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(self.snapshot())
    }

    pub(crate) fn stage_successor(
        &mut self,
        successor: &VerifiedLearnerAdmission,
    ) -> Result<(), LearnerTransportError> {
        let state = self.envelope.payload();
        let current = state
            .current
            .as_ref()
            .ok_or(LearnerTransportError::AuthorityDenied)?;
        if successor.previous_operation_sha256 != Some(current.operation_sha256)
            || successor.identity.stable_raft_id != current.identity.stable_raft_id
            || successor.identity.certificate_generation <= current.identity.certificate_generation
            || successor.committed_log_index <= current.committed_log_index
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        if let Some(staged) = state.staged_successor.as_ref() {
            return if staged.operation_sha256 == successor.operation_sha256 {
                Ok(())
            } else {
                Err(LearnerTransportError::CapacityExceeded)
            };
        }
        let mut next = state.clone();
        next.committed_log_index = successor.committed_log_index;
        next.staged_successor = Some(successor.into());
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(())
    }

    pub(crate) fn flip_successor(
        &mut self,
        operation_sha256: [u8; 32],
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        let state = self.envelope.payload();
        if state
            .current
            .as_ref()
            .is_some_and(|current| current.operation_sha256 == operation_sha256)
            && state.staged_successor.is_none()
        {
            return Ok(self.snapshot());
        }
        let staged = state
            .staged_successor
            .as_ref()
            .ok_or(LearnerTransportError::AuthorityDenied)?;
        if staged.operation_sha256 != operation_sha256 {
            return Err(LearnerTransportError::InvalidBinding);
        }
        let mut next = state.clone();
        next.current = next.staged_successor.take();
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(self.snapshot())
    }

    pub(crate) fn expire(
        &mut self,
        now_unix_seconds: i64,
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        let state = self.envelope.payload();
        let Some(current) = state.current.as_ref() else {
            return Ok(self.snapshot());
        };
        if now_unix_seconds < current.deadline_unix_seconds {
            return Err(LearnerTransportError::InvalidBinding);
        }
        let mut next = state.clone();
        next.current = None;
        next.staged_successor = None;
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(self.snapshot())
    }

    pub(crate) fn snapshot(&self) -> LearnerAdmissionSnapshot {
        LearnerAdmissionSnapshot {
            generation: self.envelope.generation(),
            current: self.envelope.payload().current.clone().map(Into::into),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExclusionState {
    committed_log_index: u64,
    published: Option<PublishedExclusion>,
}

impl CheckpointMetadataSource for ExclusionState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.committed_log_index),
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedExclusion {
    identity: LearnerIdentity,
    voter_cut_sha256: [u8; 32],
    target_membership_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
    deadline_unix_seconds: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingExclusionSnapshot {
    generation: u64,
    published: Option<PublishedExclusion>,
}

impl PendingExclusionSnapshot {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub(in crate::distributed::transport) fn membership_receipt_parts(
        &self,
    ) -> Result<Option<MembershipReceiptParts>, LearnerTransportError> {
        self.published
            .as_ref()
            .map(|published| {
                let published_state_sha256 = <[u8; 32]>::from(Sha256::digest(
                    serde_jcs::to_vec(&(
                        "adl.governed-membership-authority.exclusion.v1",
                        self.generation,
                        published.operation_sha256,
                        published.target_membership_sha256,
                        published.committed_log_index,
                    ))
                    .map_err(|_| LearnerTransportError::Storage)?,
                ));
                Ok(MembershipReceiptParts {
                    operation_sha256: published.operation_sha256,
                    generation: self.generation,
                    published_state_sha256,
                })
            })
            .transpose()
    }
    pub(crate) fn transport_identity(&self) -> Option<(String, String)> {
        self.published.as_ref().map(|published| {
            (
                published.identity.node_id.clone(),
                published.identity.guardian_id.clone(),
            )
        })
    }
    pub fn ordinary_authority_allowed(&self, node_id: &str, guardian_id: &str) -> bool {
        self.published.as_ref().is_none_or(|excluded| {
            excluded.identity.node_id != node_id && excluded.identity.guardian_id != guardian_id
        })
    }
    pub fn recovery_learner_allowed(&self, admission: &VerifiedLearnerAdmission) -> bool {
        self.published.as_ref().is_some_and(|excluded| {
            excluded.identity.stable_raft_id == admission.identity.stable_raft_id
                && excluded.identity.node_id != admission.identity.node_id
                && excluded.identity.guardian_id != admission.identity.guardian_id
                && excluded.target_membership_sha256 == admission.target_membership_sha256()
                && admission.committed_log_index > excluded.committed_log_index
        })
    }

    pub(in crate::distributed::transport) fn learner_admission_allowed(
        &self,
        admission: &VerifiedLearnerAdmission,
    ) -> bool {
        self.published.is_none() || self.recovery_learner_allowed(admission)
    }

    pub(in crate::distributed::transport) fn learner_route_allowed(
        &self,
        admission: &VerifiedLearnerAdmission,
        stable_id_is_in_voter_cut: bool,
    ) -> bool {
        self.learner_admission_allowed(admission)
            && (!stable_id_is_in_voter_cut || self.recovery_learner_allowed(admission))
    }
}

impl AuthorityEligibilityExclusion for PendingExclusionSnapshot {
    fn ordinary_authority_allowed(&self, node_id: &str, guardian_id: &[u8]) -> bool {
        std::str::from_utf8(guardian_id)
            .is_ok_and(|guardian| self.ordinary_authority_allowed(node_id, guardian))
    }
}

impl OrdinarySessionExclusion for PendingExclusionSnapshot {
    fn ordinary_session_allowed(&self, node_id: &str, guardian_id: &str) -> bool {
        self.ordinary_authority_allowed(node_id, guardian_id)
    }
}

pub(crate) struct PendingMembershipExclusionAuthority {
    store: CheckpointedJson<ExclusionState>,
    envelope: DurableEnvelope<ExclusionState>,
    capacity: usize,
}

impl PendingMembershipExclusionAuthority {
    pub(crate) fn open(
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, LearnerTransportError> {
        let (store, envelope) = CheckpointedJson::open(
            root,
            EXCLUSION_OBJECT,
            EXCLUSION_FILE,
            ExclusionState::default(),
            checkpoint,
        )?;
        Ok(Self {
            store,
            envelope,
            capacity: 1,
        })
    }

    pub(crate) fn activate(
        &mut self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        expected_target_membership_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<PendingExclusionSnapshot, LearnerTransportError> {
        // Exact retries are served from the durable published view before the
        // caller-provided result is decoded again. This preserves idempotent
        // recovery after the original authorization deadline while still
        // binding every caller-supplied target field to the cached result.
        if let Some(current) = self.envelope.payload().published.as_ref() {
            if current.operation_sha256 == result.result_sha256()
                && current.operation_id == result.operation_id()
            {
                if &current.identity != expected_identity
                    || current.voter_cut_sha256 != expected_voter_cut_sha256
                    || current.target_membership_sha256 != expected_target_membership_sha256
                {
                    return Err(LearnerTransportError::InvalidBinding);
                }
                return Ok(self.snapshot());
            }
        }
        let verified = consume_published_membership(result, MembershipDiscriminator::RemoveVoter)?;
        if &verified.payload.identity != expected_identity
            || verified.payload.voter_cut_sha256 != expected_voter_cut_sha256
            || verified.payload.target_membership_sha256 != expected_target_membership_sha256
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        if self.envelope.payload().published.is_some() {
            return Err(LearnerTransportError::CapacityExceeded);
        }
        if now_unix_seconds <= 0 || now_unix_seconds >= verified.payload.deadline_unix_seconds {
            return Err(LearnerTransportError::Expired);
        }
        if self.capacity == 0 {
            return Err(LearnerTransportError::CapacityExceeded);
        }
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = verified.committed_log_index;
        next.published = Some(PublishedExclusion {
            identity: verified.payload.identity,
            voter_cut_sha256: verified.payload.voter_cut_sha256,
            target_membership_sha256: verified.payload.target_membership_sha256,
            operation_sha256: verified.operation_sha256,
            operation_id: verified.operation_id,
            committed_log_index: verified.committed_log_index,
            deadline_unix_seconds: verified.payload.deadline_unix_seconds,
        });
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(self.snapshot())
    }

    pub(crate) fn snapshot(&self) -> PendingExclusionSnapshot {
        PendingExclusionSnapshot {
            generation: self.envelope.generation(),
            published: self.envelope.payload().published.clone(),
        }
    }
}

/// The mandatory production authority shared by every voter factory and every
/// established learner session. Opening this value reconstructs both durable
/// admission and pending-exclusion state before any route can be exposed.
#[derive(Clone)]
pub struct ProductionLearnerAuthority {
    transport_instance: Arc<Mutex<TransportInstanceAuthority>>,
    admissions: Arc<Mutex<LearnerAdmissionAuthority>>,
    exclusions: Arc<Mutex<PendingMembershipExclusionAuthority>>,
    /// One Runtime-wide fence makes exclusion publication atomic with every
    /// production dispatch. Dispatchers take a shared guard before authority
    /// revalidation and retain it through QUIC stream creation; exclusion
    /// activation takes the exclusive guard before changing durable truth.
    transport_authority: ProductionTransportAuthority,
    transport_owner: Arc<Mutex<Option<TransportAuthorityOwner>>>,
}

impl std::fmt::Debug for ProductionLearnerAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProductionLearnerAuthority")
            .finish_non_exhaustive()
    }
}

#[allow(private_interfaces)]
impl ProductionLearnerAuthority {
    pub fn open(
        root: &Path,
        checkpoint: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, LearnerTransportError> {
        let transport_instance = TransportInstanceAuthority::open(root, Arc::clone(&checkpoint))?;
        let instance_id = transport_instance.instance_id();
        let peer_instances = transport_instance.peer_instances();
        let admissions = LearnerAdmissionAuthority::open(root, Arc::clone(&checkpoint))?;
        let exclusions = PendingMembershipExclusionAuthority::open(root, checkpoint)?;
        // Both constructors validate the node-local file against its durable
        // checkpoint. Do not expose the shared handle until both succeed.
        let transport_owner = crate::distributed::transport::TransportAuthorityOwner::bootstrap(
            instance_id,
            peer_instances,
        );
        Ok(Self {
            transport_instance: Arc::new(Mutex::new(transport_instance)),
            admissions: Arc::new(Mutex::new(admissions)),
            exclusions: Arc::new(Mutex::new(exclusions)),
            transport_authority: transport_owner.authority(),
            transport_owner: Arc::new(Mutex::new(Some(transport_owner))),
        })
    }

    #[cfg(test)]
    async fn dispatch_guard(&self) -> tokio::sync::OwnedRwLockReadGuard<()> {
        self.transport_authority.read_lease().await
    }

    pub(in crate::distributed::transport) fn transport_authority(
        &self,
    ) -> ProductionTransportAuthority {
        self.transport_authority.clone()
    }

    pub(in crate::distributed::transport) fn take_transport_owner(
        &self,
    ) -> Result<TransportAuthorityOwner, LearnerTransportError> {
        self.transport_owner
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .take()
            .ok_or(LearnerTransportError::AuthorityDenied)
    }

    pub(in crate::distributed::transport) fn pin_peer_instance(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        guardian_id: &str,
        instance_id: [u8; 32],
    ) -> Result<(), LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        self.transport_instance
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .pin_peer(guardian_id, instance_id)?;
        lease
            .commit_peer_instance(guardian_id, instance_id)
            .map_err(|_| LearnerTransportError::AuthorityDenied)
    }

    #[cfg(test)]
    fn install_dispatch_pause_for_test(&self, phase: &'static str) -> TransportDispatchTestHook {
        self.transport_authority
            .install_dispatch_pause_for_test(phase)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) async fn pause_after_revalidation_for_test(&self, phase: &'static str) {
        self.transport_authority
            .pause_after_revalidation_for_test(phase)
            .await;
    }

    pub(in crate::distributed::transport) fn governed_activate_admission(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        admission: &VerifiedLearnerAdmission,
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        if !self
            .exclusion_snapshot()?
            .learner_admission_allowed(admission)
        {
            return Err(LearnerTransportError::AuthorityDenied);
        }
        let snapshot = self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .activate(admission)?;
        lease
            .replace_learner_operation(Some(admission.operation_sha256()))
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        Ok(snapshot)
    }

    pub(in crate::distributed::transport) fn governed_stage_successor(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        successor: &VerifiedLearnerAdmission,
    ) -> Result<(), LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        self.admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .stage_successor(successor)
    }

    pub(in crate::distributed::transport) fn governed_flip_successor(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        operation_sha256: [u8; 32],
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let snapshot = self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .flip_successor(operation_sha256)?;
        lease
            .replace_learner_operation(
                snapshot
                    .current()
                    .map(VerifiedLearnerAdmission::operation_sha256),
            )
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        Ok(snapshot)
    }

    pub(in crate::distributed::transport) fn governed_expire_admission(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        now_unix_seconds: i64,
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let snapshot = self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .expire(now_unix_seconds)?;
        lease
            .replace_learner_operation(None)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        Ok(snapshot)
    }

    pub(in crate::distributed::transport) fn governed_activate_exclusion(
        &self,
        lease: &mut TransportAuthorityWriteLease,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        expected_target_membership_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<PendingExclusionSnapshot, LearnerTransportError> {
        lease
            .require_authority(&self.transport_authority)
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        self.exclusions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .activate(
                result,
                expected_identity,
                expected_voter_cut_sha256,
                expected_target_membership_sha256,
                now_unix_seconds,
            )
    }

    #[cfg(test)]
    pub(crate) fn activate_admission(
        &self,
        admission: &VerifiedLearnerAdmission,
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        let snapshot = self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .activate(admission)?;
        self.transport_authority.set_learner_operation_for_test(
            admission.voter_cut_sha256(),
            Some(admission.operation_sha256()),
        );
        Ok(snapshot)
    }

    #[cfg(test)]
    pub(crate) fn stage_successor(
        &self,
        successor: &VerifiedLearnerAdmission,
    ) -> Result<(), LearnerTransportError> {
        self.admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .stage_successor(successor)
    }

    #[cfg(test)]
    pub(crate) fn flip_successor(
        &self,
        operation_sha256: [u8; 32],
    ) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        let snapshot = self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .flip_successor(operation_sha256)?;
        if let Some(current) = snapshot.current() {
            self.transport_authority.set_learner_operation_for_test(
                current.voter_cut_sha256(),
                Some(current.operation_sha256()),
            );
        }
        Ok(snapshot)
    }

    #[cfg(test)]
    pub(crate) fn activate_exclusion(
        &self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        expected_target_membership_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<PendingExclusionSnapshot, LearnerTransportError> {
        let snapshot = self
            .exclusions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .activate(
                result,
                expected_identity,
                expected_voter_cut_sha256,
                expected_target_membership_sha256,
                now_unix_seconds,
            )?;
        self.transport_authority.set_exclusion_for_test(
            &expected_identity.node_id,
            &expected_identity.guardian_id,
            snapshot.generation(),
        );
        Ok(snapshot)
    }

    pub fn admission_snapshot(&self) -> Result<LearnerAdmissionSnapshot, LearnerTransportError> {
        Ok(self
            .admissions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .snapshot())
    }

    pub fn exclusion_snapshot(&self) -> Result<PendingExclusionSnapshot, LearnerTransportError> {
        Ok(self
            .exclusions
            .lock()
            .map_err(|_| LearnerTransportError::Storage)?
            .snapshot())
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::distributed::transport) fn endorse_committed_prepare(
        &self,
        identity: &LocalNodeGuardianIdentity,
        certificate_generation: u64,
        boot_generation: u64,
        membership_log_index: u64,
        authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
        intent: &PrepareAuthorityIntent,
        finalization_time: &CanonicalAuthorityTime,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> Result<AuthorityIntentEndorsement, AuthorityProtocolError> {
        let exclusion = self
            .exclusion_snapshot()
            .map_err(|_| AuthorityProtocolError::Storage)?;
        endorse_committed_authority_prepare_with_exclusion(
            identity,
            certificate_generation,
            boot_generation,
            membership_log_index,
            authoritative_boot_generations,
            intent,
            finalization_time,
            membership,
            authority,
            &exclusion,
        )
    }

    pub(in crate::distributed::transport) fn admission_is_current(
        &self,
        admission: &VerifiedLearnerAdmission,
    ) -> Result<bool, LearnerTransportError> {
        Ok(self
            .admission_snapshot()?
            .current()
            .is_some_and(|current| current == admission)
            && self
                .exclusion_snapshot()?
                .learner_admission_allowed(admission))
    }

    fn binding_is_current(
        &self,
        binding: &LearnerSessionBinding,
    ) -> Result<bool, LearnerTransportError> {
        let Some(current) = self.admission_snapshot()?.current().cloned() else {
            return Ok(false);
        };
        if current.operation_sha256 != binding.operation_sha256
            || current.operation_id != binding.operation_id
            || current.target_membership_sha256 != binding.target_membership_sha256
            || current.committed_log_index != binding.committed_log_index
            || current.identity != binding.identity
            || current.voter_cut_sha256 != binding.voter_cut_sha256
        {
            return Ok(false);
        }
        Ok(self
            .exclusion_snapshot()?
            .learner_admission_allowed(&current))
    }

    pub(in crate::distributed::transport) fn session_is_current(
        &self,
        session: &EstablishedLearnerSession,
    ) -> Result<bool, LearnerTransportError> {
        self.binding_is_current(&session.binding)
    }
}

#[cfg(test)]
mod tests;
