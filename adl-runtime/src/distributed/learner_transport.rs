//! Authority-bound, replication-only learner transport.
//!
//! This module deliberately keeps learner admission separate from the exact
//! three-voter route cut.  Admission is possible only through the sealed
//! artifact carried by a durably published membership operation.

use std::{collections::BTreeMap, net::SocketAddr, path::Path, sync::Arc};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    authority_protocol::{
        AuthorityEligibilityExclusion, AuthorityOperationKind, CommittedAuthorityArtifact,
        PublishedAuthorityResult,
    },
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
    transport::{AuthenticatedConnection, OrdinarySessionExclusion, VerifiedPolisRouteCut},
};

const MEMBERSHIP_ARTIFACT_SCHEMA: &str = "adl.distributed.learner_membership.v1";
const MEMBERSHIP_ARTIFACT_DOMAIN: &str = "adl.authority-artifact.membership.v1";
const EXCLUSION_OBJECT: &str = "pending-membership-exclusion-v1";
const EXCLUSION_FILE: &str = "pending-membership-exclusion.json";
const ADMISSION_OBJECT: &str = "learner-admission-v1";
const ADMISSION_FILE: &str = "learner-admission.json";
const ROLE: &str = "replication_only_learner";
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
enum MembershipDiscriminator {
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
    fn validate(&self) -> Result<(), LearnerTransportError> {
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
struct VerifiedMembershipArtifact {
    payload: CanonicalMembershipArtifact,
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
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
        operation_sha256: result.result_sha256(),
        operation_id: result.operation_id().to_owned(),
        committed_log_index: result.committed_log_index(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedLearnerAdmission {
    identity: LearnerIdentity,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
}

impl VerifiedLearnerAdmission {
    pub fn from_published_membership(
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<Self, LearnerTransportError> {
        let verified =
            consume_published_membership(result, MembershipDiscriminator::EnrollNonVoting)?;
        if &verified.payload.identity != expected_identity
            || verified.payload.voter_cut_sha256 != expected_voter_cut_sha256
            || now_unix_seconds <= 0
            || now_unix_seconds >= verified.payload.deadline_unix_seconds
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(Self {
            identity: verified.payload.identity,
            voter_cut_sha256: verified.payload.voter_cut_sha256,
            operation_sha256: verified.operation_sha256,
            operation_id: verified.operation_id,
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

    pub fn establish_learner_session(
        &self,
        now_unix_seconds: i64,
    ) -> Result<EstablishedLearnerSession, LearnerTransportError> {
        let admission = self
            .admission
            .as_ref()
            .ok_or(LearnerTransportError::AuthorityDenied)?;
        if now_unix_seconds >= admission.deadline_unix_seconds {
            return Err(LearnerTransportError::Expired);
        }
        Ok(EstablishedLearnerSession {
            binding: LearnerSessionBinding::from_admission(admission, self.voter_cut_sha256),
            highest_sequence: 0,
            closed: false,
        })
    }
}

fn route_cut_digest(cut: &VerifiedPolisRouteCut) -> Result<[u8; 32], LearnerTransportError> {
    let bytes = serde_jcs::to_vec(&(
        cut.polis_id(),
        cut.trust_domain(),
        cut.membership_epoch(),
        cut.committed_membership_index(),
        cut.routes(),
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
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
    role: &'static str,
    protocol_version: u32,
}

impl LearnerSessionBinding {
    fn from_admission(admission: &VerifiedLearnerAdmission, voter_cut_sha256: [u8; 32]) -> Self {
        Self {
            identity: admission.identity.clone(),
            voter_cut_sha256,
            operation_sha256: admission.operation_sha256,
            operation_id: admission.operation_id.clone(),
            committed_log_index: admission.committed_log_index,
            deadline_unix_seconds: admission.deadline_unix_seconds,
            overlap_end_unix_seconds: admission.overlap_end_unix_seconds,
            role: ROLE,
            protocol_version: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EstablishedLearnerSession {
    binding: LearnerSessionBinding,
    highest_sequence: u64,
    closed: bool,
}

#[derive(Serialize)]
struct LearnerRpcBinding<'a> {
    trust_domain: &'a str,
    polis_id: &'a str,
    node_id: &'a str,
    guardian_id: &'a str,
    stable_raft_id: u64,
    certificate_generation: u64,
    boot_generation: u64,
    address: String,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: &'a str,
    committed_log_index: u64,
    role: &'a str,
    protocol_version: u32,
    sequence: u64,
    message_kind: &'a str,
    payload_sha256: [u8; 32],
    deadline_unix_seconds: i64,
}

impl EstablishedLearnerSession {
    pub fn authorize(
        &mut self,
        kind: LearnerRpcKind,
        sequence: u64,
        payload: &[u8],
        now_unix_seconds: i64,
    ) -> Result<[u8; 32], LearnerTransportError> {
        if self.closed {
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
            stable_raft_id: self.binding.identity.stable_raft_id,
            certificate_generation: self.binding.identity.certificate_generation,
            boot_generation: self.binding.identity.boot_generation,
            address: self.binding.identity.address.to_string(),
            voter_cut_sha256: self.binding.voter_cut_sha256,
            operation_sha256: self.binding.operation_sha256,
            operation_id: &self.binding.operation_id,
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
        if connection.matches_learner_endpoint(
            &identity.trust_domain,
            &identity.node_id,
            &identity.guardian_id,
            identity.certificate_generation,
        ) {
            Ok(())
        } else {
            Err(LearnerTransportError::InvalidBinding)
        }
    }
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
    ) -> Result<[u8; 32], LearnerTransportError> {
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
        connection
            .send(sequence, bytes)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        Ok(authorization_sha256)
    }

    pub async fn send_append_entries(
        &mut self,
        connection: &AuthenticatedConnection,
        sequence: u64,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<[u8; 32], LearnerTransportError> {
        self.send_replication(
            connection,
            LearnerRpcKind::AppendEntries,
            sequence,
            payload,
            now_unix_seconds,
        )
        .await
    }

    pub async fn send_install_snapshot(
        &mut self,
        connection: &AuthenticatedConnection,
        sequence: u64,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<[u8; 32], LearnerTransportError> {
        self.send_replication(
            connection,
            LearnerRpcKind::InstallSnapshot,
            sequence,
            payload,
            now_unix_seconds,
        )
        .await
    }

    pub async fn receive_replication(
        &mut self,
        connection: &AuthenticatedConnection,
        now_unix_seconds: i64,
    ) -> Result<LearnerReplicationFrame, LearnerTransportError> {
        self.require_connection(connection)?;
        let envelope = connection
            .receive()
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let frame: LearnerReplicationFrame = serde_json::from_slice(&envelope.payload)
            .map_err(|_| LearnerTransportError::InvalidBinding)?;
        if serde_jcs::to_vec(&frame).map_err(|_| LearnerTransportError::InvalidBinding)?
            != envelope.payload
            || frame.schema != LEARNER_REPLICATION_FRAME_SCHEMA
            || frame.sequence != envelope.sequence
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
        Ok(frame)
    }

    pub(crate) async fn send_response(
        &mut self,
        connection: &AuthenticatedConnection,
        request: &LearnerReplicationFrame,
        payload: Vec<u8>,
        now_unix_seconds: i64,
    ) -> Result<(), LearnerTransportError> {
        self.require_connection(connection)?;
        let kind = match request.message_kind.as_str() {
            "append_entries" => LearnerRpcKind::AppendEntries,
            "install_snapshot" => LearnerRpcKind::InstallSnapshot,
            _ => return Err(LearnerTransportError::AuthorityDenied),
        };
        let response_authorization_sha256 =
            self.authorize(kind, request.sequence, &payload, now_unix_seconds)?;
        let response = LearnerReplicationResponse {
            schema: LEARNER_REPLICATION_RESPONSE_SCHEMA.to_owned(),
            sequence: request.sequence,
            request_authorization_sha256: request.authorization_sha256,
            response_authorization_sha256,
            payload_sha256: Sha256::digest(&payload).into(),
            payload,
        };
        let bytes =
            serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)?;
        connection
            .send(request.sequence, bytes)
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)
    }

    pub(crate) async fn receive_response(
        &mut self,
        connection: &AuthenticatedConnection,
        kind: LearnerRpcKind,
        request_sequence: u64,
        request_authorization_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<Vec<u8>, LearnerTransportError> {
        self.require_connection(connection)?;
        let envelope = connection
            .receive()
            .await
            .map_err(|_| LearnerTransportError::AuthorityDenied)?;
        let response: LearnerReplicationResponse = serde_json::from_slice(&envelope.payload)
            .map_err(|_| LearnerTransportError::InvalidBinding)?;
        if serde_jcs::to_vec(&response).map_err(|_| LearnerTransportError::InvalidBinding)?
            != envelope.payload
            || response.schema != LEARNER_REPLICATION_RESPONSE_SCHEMA
            || response.sequence != request_sequence
            || envelope.sequence != request_sequence
            || response.request_authorization_sha256 != request_authorization_sha256
            || response.payload_sha256 != <[u8; 32]>::from(Sha256::digest(&response.payload))
            || self.authorize(kind, request_sequence, &response.payload, now_unix_seconds)?
                != response.response_authorization_sha256
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        Ok(response.payload)
    }

    pub(crate) fn learner_id(&self) -> u64 {
        self.binding.identity.stable_raft_id
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
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    operation_id: String,
    committed_log_index: u64,
    deadline_unix_seconds: i64,
    overlap_end_unix_seconds: Option<i64>,
}

impl From<&VerifiedLearnerAdmission> for DurableAdmission {
    fn from(value: &VerifiedLearnerAdmission) -> Self {
        Self {
            identity: value.identity.clone(),
            voter_cut_sha256: value.voter_cut_sha256,
            operation_sha256: value.operation_sha256,
            operation_id: value.operation_id.clone(),
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
            voter_cut_sha256: value.voter_cut_sha256,
            operation_sha256: value.operation_sha256,
            operation_id: value.operation_id,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LearnerAdmissionSnapshot {
    generation: u64,
    current: Option<VerifiedLearnerAdmission>,
}

impl LearnerAdmissionSnapshot {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn current(&self) -> Option<&VerifiedLearnerAdmission> {
        self.current.as_ref()
    }
}

/// Durable single-lineage admission publication and successor flip.
pub struct LearnerAdmissionAuthority {
    store: CheckpointedJson<AdmissionState>,
    envelope: DurableEnvelope<AdmissionState>,
}

impl LearnerAdmissionAuthority {
    pub fn open(
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

    pub fn activate(
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

    pub fn stage_successor(
        &mut self,
        successor: &VerifiedLearnerAdmission,
        previous_operation_sha256: [u8; 32],
    ) -> Result<(), LearnerTransportError> {
        let state = self.envelope.payload();
        let current = state
            .current
            .as_ref()
            .ok_or(LearnerTransportError::AuthorityDenied)?;
        if current.operation_sha256 != previous_operation_sha256
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

    pub fn flip_successor(
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

    pub fn expire(
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

    pub fn snapshot(&self) -> LearnerAdmissionSnapshot {
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
                && admission.committed_log_index > excluded.committed_log_index
        })
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

pub struct PendingMembershipExclusionAuthority {
    store: CheckpointedJson<ExclusionState>,
    envelope: DurableEnvelope<ExclusionState>,
    capacity: usize,
}

impl PendingMembershipExclusionAuthority {
    pub fn open(
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

    pub fn activate(
        &mut self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
    ) -> Result<PendingExclusionSnapshot, LearnerTransportError> {
        let verified = consume_published_membership(result, MembershipDiscriminator::RemoveVoter)?;
        if &verified.payload.identity != expected_identity
            || verified.payload.voter_cut_sha256 != expected_voter_cut_sha256
        {
            return Err(LearnerTransportError::InvalidBinding);
        }
        if let Some(current) = self.envelope.payload().published.as_ref() {
            if current.operation_sha256 == verified.operation_sha256 {
                return Ok(self.snapshot());
            }
            return Err(LearnerTransportError::CapacityExceeded);
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
        });
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(self.snapshot())
    }

    pub fn snapshot(&self) -> PendingExclusionSnapshot {
        PendingExclusionSnapshot {
            generation: self.envelope.generation(),
            published: self.envelope.payload().published.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
