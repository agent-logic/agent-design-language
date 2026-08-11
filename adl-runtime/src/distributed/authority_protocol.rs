//! Deterministic committed authority protocol.
//!
//! This module turns one exact committed intent plus endorsements from the
//! current voter set into an opaque verified operation. It deliberately owns
//! no membership, certificate, lease, fencing, migration, or recovery effect.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    lease::{AuthorityMembership, ControlCertificatePurpose},
    membership::{MemberRole, MembershipState},
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
};

const INTENT_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-INTENT-V1\0";
const ENDORSEMENT_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-ENDORSEMENT-V1\0";
const CONFIGURATION_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-CONFIGURATION-V1\0";
const MAX_IDENTITY_BYTES: usize = 128;
const MAX_ARTIFACT_BYTES: usize = 1024 * 1024;
const MAX_PUBLISHED_OPERATIONS: usize = 4096;
const PROTOCOL_INSTANCE_VERSION: &str = "adl.committed-authority-protocol.v1";

pub type AuthorityProtocolResult<T> = Result<T, AuthorityProtocolError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthorityProtocolError {
    InvalidIntent,
    InvalidMembership,
    WrongMembership,
    WrongTrustDomain,
    WrongVoter,
    WrongVoterPurpose,
    StaleVoter,
    DuplicateVoter,
    MissingQuorum,
    InvalidEndorsement,
    TimeOutsideIntent,
    ArtifactMismatch,
    Serialization,
    Storage,
    StateRegression,
    RetryConflict,
    CapacityExceeded,
}

impl AuthorityProtocolError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidIntent => "invalid_intent",
            Self::InvalidMembership => "invalid_membership",
            Self::WrongMembership => "wrong_membership",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::WrongVoter => "wrong_voter",
            Self::WrongVoterPurpose => "wrong_voter_purpose",
            Self::StaleVoter => "stale_voter",
            Self::DuplicateVoter => "duplicate_voter",
            Self::MissingQuorum => "missing_quorum",
            Self::InvalidEndorsement => "invalid_endorsement",
            Self::TimeOutsideIntent => "time_outside_intent",
            Self::ArtifactMismatch => "artifact_mismatch",
            Self::Serialization => "serialization",
            Self::Storage => "storage",
            Self::StateRegression => "state_regression",
            Self::RetryConflict => "retry_conflict",
            Self::CapacityExceeded => "capacity_exceeded",
        }
    }
}

impl std::fmt::Display for AuthorityProtocolError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for AuthorityProtocolError {}

impl From<PolisRuntimeError> for AuthorityProtocolError {
    fn from(error: PolisRuntimeError) -> Self {
        match error {
            PolisRuntimeError::StateRegression | PolisRuntimeError::Replay => Self::StateRegression,
            PolisRuntimeError::FrameTooLarge => Self::CapacityExceeded,
            PolisRuntimeError::Serialization => Self::Serialization,
            _ => Self::Storage,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityOperationKind {
    Membership,
    Reconciliation,
    ExistingStore,
    ContinuityTransfer,
}

impl AuthorityOperationKind {
    fn artifact_domain(self) -> &'static str {
        match self {
            Self::Membership => "adl.authority-artifact.membership.v1",
            Self::Reconciliation => "adl.authority-artifact.reconciliation.v1",
            Self::ExistingStore => "adl.authority-artifact.existing-store.v1",
            Self::ContinuityTransfer => "adl.authority-artifact.continuity-transfer.v1",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalAuthorityTime {
    pub unix_seconds: i64,
    pub nanos: u32,
    pub uncertainty_millis: u64,
}

impl CanonicalAuthorityTime {
    fn validate(&self) -> AuthorityProtocolResult<()> {
        if self.unix_seconds <= 0 || self.nanos >= 1_000_000_000 {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        Ok(())
    }

    fn order_key(&self) -> (i64, u32) {
        (self.unix_seconds, self.nanos)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommittedAuthorityArtifact {
    pub domain: String,
    pub bytes: Vec<u8>,
    pub sha256: [u8; 32],
}

impl CommittedAuthorityArtifact {
    pub fn new(kind: AuthorityOperationKind, bytes: Vec<u8>) -> AuthorityProtocolResult<Self> {
        if bytes.is_empty() || bytes.len() > MAX_ARTIFACT_BYTES {
            return Err(AuthorityProtocolError::ArtifactMismatch);
        }
        Ok(Self {
            domain: kind.artifact_domain().to_owned(),
            sha256: Sha256::digest(&bytes).into(),
            bytes,
        })
    }

    fn validate(&self, kind: AuthorityOperationKind) -> AuthorityProtocolResult<()> {
        if self.domain != kind.artifact_domain()
            || self.bytes.is_empty()
            || self.bytes.len() > MAX_ARTIFACT_BYTES
            || self.sha256 != <[u8; 32]>::from(Sha256::digest(&self.bytes))
        {
            return Err(AuthorityProtocolError::ArtifactMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareAuthorityIntent {
    pub polis_id: String,
    pub trust_domain: String,
    pub membership_epoch: u64,
    pub membership_log_index: u64,
    pub voter_set_generation: u64,
    pub configuration_sha256: [u8; 32],
    pub operation_kind: AuthorityOperationKind,
    pub expected_protocol_checkpoint_sha256: [u8; 32],
    pub payload_sha256: [u8; 32],
    pub prepare_time: CanonicalAuthorityTime,
    pub inclusive_deadline: CanonicalAuthorityTime,
    pub operation_id: String,
    pub artifact: CommittedAuthorityArtifact,
}

impl PrepareAuthorityIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        polis_id: impl Into<String>,
        membership: &MembershipState,
        authority: &AuthorityMembership,
        operation_kind: AuthorityOperationKind,
        expected_protocol_checkpoint_sha256: [u8; 32],
        prepare_time: CanonicalAuthorityTime,
        inclusive_deadline: CanonicalAuthorityTime,
        operation_id: impl Into<String>,
        artifact: CommittedAuthorityArtifact,
    ) -> AuthorityProtocolResult<Self> {
        validate_membership_pair(membership, authority)?;
        let intent = Self {
            polis_id: polis_id.into(),
            trust_domain: membership.trust_domain().to_owned(),
            membership_epoch: membership.epoch(),
            membership_log_index: membership.committed_log_index(),
            voter_set_generation: authority.voter_set_generation,
            configuration_sha256: configuration_digest(authority)?,
            operation_kind,
            expected_protocol_checkpoint_sha256,
            payload_sha256: artifact.sha256,
            prepare_time,
            inclusive_deadline,
            operation_id: operation_id.into(),
            artifact,
        };
        intent.validate_against(membership, authority)?;
        Ok(intent)
    }

    pub fn validate_against(
        &self,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<()> {
        validate_identifier(&self.polis_id)?;
        validate_identifier(&self.trust_domain)?;
        validate_identifier(&self.operation_id)?;
        self.prepare_time.validate()?;
        self.inclusive_deadline.validate()?;
        self.artifact.validate(self.operation_kind)?;
        validate_membership_pair(membership, authority)?;
        if self.trust_domain != membership.trust_domain()
            || self.membership_epoch != membership.epoch()
            || self.membership_log_index != membership.committed_log_index()
            || self.membership_log_index != authority.committed_log_index
            || self.voter_set_generation != authority.voter_set_generation
            || self.configuration_sha256 != configuration_digest(authority)?
            || self.prepare_time.order_key() > self.inclusive_deadline.order_key()
            || self.expected_protocol_checkpoint_sha256 == [0; 32]
            || self.payload_sha256 == [0; 32]
            || self.payload_sha256 != self.artifact.sha256
        {
            return Err(AuthorityProtocolError::WrongMembership);
        }
        Ok(())
    }

    pub fn digest(&self) -> AuthorityProtocolResult<[u8; 32]> {
        canonical_domain_digest(INTENT_DOMAIN, self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeAuthorityIntent {
    pub operation_id: String,
    pub intent_sha256: [u8; 32],
    pub finalization_time: CanonicalAuthorityTime,
    pub endorsements: Vec<AuthorityIntentEndorsement>,
}

impl FinalizeAuthorityIntent {
    pub fn new(
        intent: &PrepareAuthorityIntent,
        finalization_time: CanonicalAuthorityTime,
        endorsements: Vec<AuthorityIntentEndorsement>,
    ) -> AuthorityProtocolResult<Self> {
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
        {
            return Err(AuthorityProtocolError::TimeOutsideIntent);
        }
        Ok(Self {
            operation_id: intent.operation_id.clone(),
            intent_sha256: intent.digest()?,
            finalization_time,
            endorsements,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityIntentEndorsement {
    guardian_id: Vec<u8>,
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    signature: Vec<u8>,
}

pub struct VoterEndorsementAuthority {
    node_id: String,
    guardian_id: Vec<u8>,
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    signing_key: SigningKey,
}

impl VoterEndorsementAuthority {
    pub fn restore_configured(
        node_id: impl Into<String>,
        guardian_id: Vec<u8>,
        certificate_generation: u64,
        boot_generation: u64,
        membership_log_index: u64,
        signing_key: SigningKey,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<Self> {
        validate_membership_pair(membership, authority)?;
        let node_id = node_id.into();
        validate_identifier(&node_id)?;
        let member = membership
            .member(&node_id)
            .ok_or(AuthorityProtocolError::WrongVoter)?;
        let voter = authority
            .voters
            .get(&guardian_id)
            .ok_or(AuthorityProtocolError::WrongVoter)?;
        if certificate_generation == 0
            || boot_generation == 0
            || membership_log_index != authority.committed_log_index
            || voter.certificate_generation != certificate_generation
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || voter.revoked
            || voter.control_public_key != signing_key.verifying_key().to_bytes()
            || member.role != MemberRole::Voter
            || member.guardian_id.as_bytes() != guardian_id
        {
            return Err(AuthorityProtocolError::WrongVoter);
        }
        Ok(Self {
            node_id,
            guardian_id,
            certificate_generation,
            boot_generation,
            membership_log_index,
            signing_key,
        })
    }

    pub fn endorse(
        &self,
        intent: &PrepareAuthorityIntent,
        finalization_time: &CanonicalAuthorityTime,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<AuthorityIntentEndorsement> {
        intent.validate_against(membership, authority)?;
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
        {
            return Err(AuthorityProtocolError::TimeOutsideIntent);
        }
        let voter = authority
            .voters
            .get(&self.guardian_id)
            .ok_or(AuthorityProtocolError::WrongVoter)?;
        if voter.revoked
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || voter.certificate_generation != self.certificate_generation
            || authority.committed_log_index != self.membership_log_index
            || membership
                .member(&self.node_id)
                .is_none_or(|member| member.guardian_id.as_bytes() != self.guardian_id)
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        let payload = endorsement_payload(intent.digest()?, finalization_time)?;
        Ok(AuthorityIntentEndorsement {
            guardian_id: self.guardian_id.clone(),
            certificate_generation: self.certificate_generation,
            boot_generation: self.boot_generation,
            membership_log_index: self.membership_log_index,
            signature: self.signing_key.sign(&payload).to_bytes().to_vec(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAuthorityOperation {
    operation_id: String,
    intent_sha256: [u8; 32],
    finalization_time: CanonicalAuthorityTime,
    artifact: CommittedAuthorityArtifact,
    signer_guardian_ids: BTreeSet<Vec<u8>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityNodeIdentity {
    pub trust_domain: String,
    pub polis_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub boot_generation: u64,
}

impl AuthorityNodeIdentity {
    fn validate(&self) -> AuthorityProtocolResult<()> {
        validate_identifier(&self.trust_domain)?;
        validate_identifier(&self.polis_id)?;
        validate_identifier(&self.node_id)?;
        validate_identifier(&self.guardian_id)?;
        if self.boot_generation == 0 {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        Ok(())
    }

    fn checkpoint_object(&self) -> AuthorityProtocolResult<String> {
        Ok(format!(
            "authority-protocol-{}",
            hex::encode(canonical_domain_digest(
                b"ADL-COMMITTED-AUTHORITY-CHECKPOINT-OBJECT-V1\0",
                &(PROTOCOL_INSTANCE_VERSION, self),
            )?)
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedAuthorityResult {
    operation_id: String,
    intent_sha256: [u8; 32],
    result_sha256: [u8; 32],
    retry_sha256: [u8; 32],
    committed_log_index: u64,
    operation: VerifiedAuthorityOperation,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurablePublishedAuthorityResult {
    operation_id: String,
    intent_sha256: [u8; 32],
    result_sha256: [u8; 32],
    retry_sha256: [u8; 32],
    committed_log_index: u64,
    finalization_time: CanonicalAuthorityTime,
    artifact: CommittedAuthorityArtifact,
    signer_guardian_ids: BTreeSet<Vec<u8>>,
}

impl DurablePublishedAuthorityResult {
    fn public_result(&self) -> PublishedAuthorityResult {
        PublishedAuthorityResult {
            operation_id: self.operation_id.clone(),
            intent_sha256: self.intent_sha256,
            result_sha256: self.result_sha256,
            retry_sha256: self.retry_sha256,
            committed_log_index: self.committed_log_index,
            operation: VerifiedAuthorityOperation {
                operation_id: self.operation_id.clone(),
                intent_sha256: self.intent_sha256,
                finalization_time: self.finalization_time.clone(),
                artifact: self.artifact.clone(),
                signer_guardian_ids: self.signer_guardian_ids.clone(),
            },
        }
    }
}

impl PublishedAuthorityResult {
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn result_sha256(&self) -> [u8; 32] {
        self.result_sha256
    }

    pub fn retry_sha256(&self) -> [u8; 32] {
        self.retry_sha256
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn operation(&self) -> &VerifiedAuthorityOperation {
        &self.operation
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityProtocolState {
    committed_log_index: u64,
    published: BTreeMap<String, DurablePublishedAuthorityResult>,
}

impl CheckpointMetadataSource for AuthorityProtocolState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        let state_sha256 = canonical_domain_digest(b"ADL-COMMITTED-AUTHORITY-STATE-V1\0", self)
            .map(hex::encode)
            .map_err(|_| PolisRuntimeError::Serialization)?;
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.committed_log_index),
            state_sha256: Some(state_sha256),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}

/// One node's durable authority-publication barrier.
///
/// Instances are deliberately node-local: their canonical checkpoint object
/// binds the node and Guardian identity plus boot generation. Publication does
/// not return until the local result/retry bytes and that exact external CAS
/// agree.
pub struct DurableAuthorityProtocol {
    store: CheckpointedJson<AuthorityProtocolState>,
    envelope: DurableEnvelope<AuthorityProtocolState>,
    identity: AuthorityNodeIdentity,
    capacity: usize,
}

impl DurableAuthorityProtocol {
    pub fn open(
        root: &Path,
        identity: AuthorityNodeIdentity,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> AuthorityProtocolResult<Self> {
        Self::open_with_capacity(root, identity, authority, MAX_PUBLISHED_OPERATIONS)
    }

    pub fn open_with_capacity(
        root: &Path,
        identity: AuthorityNodeIdentity,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> AuthorityProtocolResult<Self> {
        identity.validate()?;
        if capacity == 0 || capacity > MAX_PUBLISHED_OPERATIONS {
            return Err(AuthorityProtocolError::CapacityExceeded);
        }
        let object = identity.checkpoint_object()?;
        let (store, envelope) = CheckpointedJson::open(
            root,
            &object,
            "authority-protocol.json",
            AuthorityProtocolState::default(),
            authority,
        )?;
        Ok(Self {
            store,
            envelope,
            identity,
            capacity,
        })
    }

    pub fn checkpoint_sha256(&self) -> AuthorityProtocolResult<[u8; 32]> {
        parse_sha256(self.envelope.payload_sha256())
    }

    pub fn generation(&self) -> u64 {
        self.envelope.generation()
    }

    pub fn published(&self, operation_id: &str) -> Option<PublishedAuthorityResult> {
        self.envelope
            .payload()
            .published
            .get(operation_id)
            .map(DurablePublishedAuthorityResult::public_result)
    }

    pub fn publish(
        &mut self,
        intent: &PrepareAuthorityIntent,
        verified: VerifiedAuthorityOperation,
    ) -> AuthorityProtocolResult<PublishedAuthorityResult> {
        if intent.trust_domain != self.identity.trust_domain
            || intent.polis_id != self.identity.polis_id
            || verified.operation_id != intent.operation_id
            || verified.intent_sha256 != intent.digest()?
            || verified.artifact != intent.artifact
        {
            return Err(AuthorityProtocolError::WrongTrustDomain);
        }
        if let Some(existing) = self.envelope.payload().published.get(&intent.operation_id) {
            if existing.intent_sha256 == verified.intent_sha256
                && existing.public_result().operation == verified
                && existing.retry_sha256 == retry_digest(existing)?
                && existing.result_sha256
                    == result_digest(
                        &existing.public_result().operation,
                        intent.membership_log_index,
                    )?
            {
                return Ok(existing.public_result());
            }
            return Err(AuthorityProtocolError::RetryConflict);
        }
        if intent.expected_protocol_checkpoint_sha256 != self.checkpoint_sha256()? {
            return Err(AuthorityProtocolError::StateRegression);
        }
        if self.envelope.payload().published.len() >= self.capacity {
            return Err(AuthorityProtocolError::CapacityExceeded);
        }
        let result_sha256 = result_digest(&verified, intent.membership_log_index)?;
        let mut result = DurablePublishedAuthorityResult {
            operation_id: intent.operation_id.clone(),
            intent_sha256: verified.intent_sha256,
            result_sha256,
            retry_sha256: [0; 32],
            committed_log_index: intent.membership_log_index,
            finalization_time: verified.finalization_time.clone(),
            artifact: verified.artifact.clone(),
            signer_guardian_ids: verified.signer_guardian_ids.clone(),
        };
        result.retry_sha256 = retry_digest(&result)?;
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = next.committed_log_index.max(intent.membership_log_index);
        next.published
            .insert(intent.operation_id.clone(), result.clone());
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(result.public_result())
    }
}

impl VerifiedAuthorityOperation {
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn intent_sha256(&self) -> [u8; 32] {
        self.intent_sha256
    }

    pub fn finalization_time(&self) -> &CanonicalAuthorityTime {
        &self.finalization_time
    }

    // The sealed downstream adapters land in their owning issues. Keep this
    // crate-private accessor non-public without weakening the issue-local
    // warning gate while those consumers remain absent.
    #[allow(dead_code)]
    pub(crate) fn artifact_for_sealed_consumer(&self) -> &CommittedAuthorityArtifact {
        &self.artifact
    }
}

pub fn verify_finalization(
    intent: &PrepareAuthorityIntent,
    finalize: &FinalizeAuthorityIntent,
    membership: &MembershipState,
    authority: &AuthorityMembership,
) -> AuthorityProtocolResult<VerifiedAuthorityOperation> {
    intent.validate_against(membership, authority)?;
    finalize.finalization_time.validate()?;
    let intent_sha256 = intent.digest()?;
    if finalize.operation_id != intent.operation_id
        || finalize.intent_sha256 != intent_sha256
        || finalize.finalization_time.order_key() < intent.prepare_time.order_key()
        || finalize.finalization_time.order_key() > intent.inclusive_deadline.order_key()
    {
        return Err(AuthorityProtocolError::InvalidIntent);
    }
    let payload = endorsement_payload(intent_sha256, &finalize.finalization_time)?;
    let mut signers = BTreeSet::new();
    for endorsement in &finalize.endorsements {
        if !signers.insert(endorsement.guardian_id.clone()) {
            return Err(AuthorityProtocolError::DuplicateVoter);
        }
        let voter = authority
            .voters
            .get(&endorsement.guardian_id)
            .ok_or(AuthorityProtocolError::WrongVoter)?;
        if voter.revoked
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || endorsement.certificate_generation != voter.certificate_generation
            || endorsement.membership_log_index != authority.committed_log_index
            || endorsement.boot_generation == 0
            || finalize.finalization_time.unix_seconds < voter.not_before_unix_seconds
            || finalize.finalization_time.unix_seconds > voter.not_after_unix_seconds
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        let verifying_key = VerifyingKey::from_bytes(&voter.control_public_key)
            .map_err(|_| AuthorityProtocolError::InvalidEndorsement)?;
        let signature = Signature::from_slice(&endorsement.signature)
            .map_err(|_| AuthorityProtocolError::InvalidEndorsement)?;
        verifying_key
            .verify(&payload, &signature)
            .map_err(|_| AuthorityProtocolError::InvalidEndorsement)?;
    }
    if !has_joint_quorum(authority, &signers) {
        return Err(AuthorityProtocolError::MissingQuorum);
    }
    Ok(VerifiedAuthorityOperation {
        operation_id: intent.operation_id.clone(),
        intent_sha256,
        finalization_time: finalize.finalization_time.clone(),
        artifact: intent.artifact.clone(),
        signer_guardian_ids: signers,
    })
}

fn validate_membership_pair(
    membership: &MembershipState,
    authority: &AuthorityMembership,
) -> AuthorityProtocolResult<()> {
    if membership.trust_domain().as_bytes() != authority.trust_domain_id
        || membership.committed_log_index() != authority.committed_log_index
    {
        return Err(AuthorityProtocolError::WrongMembership);
    }
    let membership_voters = membership
        .members()
        .filter(|member| member.role == MemberRole::Voter)
        .map(|member| {
            (
                member.guardian_id.as_bytes().to_vec(),
                member.guardian_control_public_key,
            )
        })
        .collect::<BTreeMap<_, _>>();
    if membership_voters.len() != authority.voters.len()
        || authority.voters.iter().any(|(guardian_id, voter)| {
            membership_voters.get(guardian_id) != Some(&voter.control_public_key)
                || voter.guardian_id != *guardian_id
                || voter.trust_domain_id != authority.trust_domain_id
                || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
                || voter.revoked
        })
    {
        return Err(AuthorityProtocolError::InvalidMembership);
    }
    Ok(())
}

fn configuration_digest(authority: &AuthorityMembership) -> AuthorityProtocolResult<[u8; 32]> {
    let guardian_by_raft = authority
        .raft_ids
        .iter()
        .map(|(guardian, raft_id)| (*raft_id, guardian.clone()))
        .collect::<BTreeMap<_, _>>();
    let configs = authority
        .raft_membership
        .get_joint_config()
        .iter()
        .map(|config| {
            config
                .iter()
                .map(|raft_id| {
                    guardian_by_raft
                        .get(raft_id)
                        .cloned()
                        .ok_or(AuthorityProtocolError::InvalidMembership)
                })
                .collect::<AuthorityProtocolResult<Vec<_>>>()
        })
        .collect::<AuthorityProtocolResult<Vec<_>>>()?;
    canonical_domain_digest(CONFIGURATION_DOMAIN, &configs)
}

fn has_joint_quorum(authority: &AuthorityMembership, signers: &BTreeSet<Vec<u8>>) -> bool {
    let signer_raft_ids = signers
        .iter()
        .filter_map(|guardian| authority.raft_ids.get(guardian).copied())
        .collect::<BTreeSet<_>>();
    authority
        .raft_membership
        .get_joint_config()
        .iter()
        .all(|config| {
            config
                .iter()
                .filter(|raft_id| signer_raft_ids.contains(raft_id))
                .count()
                > config.len() / 2
        })
}

fn endorsement_payload(
    intent_sha256: [u8; 32],
    finalization_time: &CanonicalAuthorityTime,
) -> AuthorityProtocolResult<Vec<u8>> {
    let body = serde_jcs::to_vec(&(intent_sha256, finalization_time))
        .map_err(|_| AuthorityProtocolError::Serialization)?;
    let mut payload = Vec::with_capacity(ENDORSEMENT_DOMAIN.len() + body.len());
    payload.extend_from_slice(ENDORSEMENT_DOMAIN);
    payload.extend_from_slice(&body);
    Ok(payload)
}

fn canonical_domain_digest<T: Serialize>(
    domain: &[u8],
    value: &T,
) -> AuthorityProtocolResult<[u8; 32]> {
    let bytes = serde_jcs::to_vec(value).map_err(|_| AuthorityProtocolError::Serialization)?;
    let mut hasher = Sha256::new();
    hasher.update(domain);
    hasher.update(bytes);
    Ok(hasher.finalize().into())
}

fn result_digest(
    operation: &VerifiedAuthorityOperation,
    committed_log_index: u64,
) -> AuthorityProtocolResult<[u8; 32]> {
    canonical_domain_digest(
        b"ADL-COMMITTED-AUTHORITY-RESULT-V1\0",
        &(
            committed_log_index,
            &operation.operation_id,
            operation.intent_sha256,
            &operation.finalization_time,
            &operation.artifact,
            &operation.signer_guardian_ids,
        ),
    )
}

fn retry_digest(result: &DurablePublishedAuthorityResult) -> AuthorityProtocolResult<[u8; 32]> {
    canonical_domain_digest(
        b"ADL-COMMITTED-AUTHORITY-RETRY-V1\0",
        &(
            &result.operation_id,
            result.intent_sha256,
            result.result_sha256,
            result.committed_log_index,
            &result.finalization_time,
            &result.artifact,
            &result.signer_guardian_ids,
        ),
    )
}

fn parse_sha256(value: &str) -> AuthorityProtocolResult<[u8; 32]> {
    let bytes = hex::decode(value).map_err(|_| AuthorityProtocolError::Serialization)?;
    bytes
        .try_into()
        .map_err(|_| AuthorityProtocolError::Serialization)
}

fn validate_identifier(value: &str) -> AuthorityProtocolResult<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTITY_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':'))
    {
        return Err(AuthorityProtocolError::InvalidIntent);
    }
    Ok(())
}
