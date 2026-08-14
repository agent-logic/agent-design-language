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

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    identity::{GuardianControlSignerCustody, LocalNodeGuardianIdentity},
    lease::{AuthorityMembership, ControlCertificatePurpose},
    membership::{MemberRole, MembershipState},
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
};

const INTENT_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-INTENT-V1\0";
#[cfg(test)]
const ENDORSEMENT_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-ENDORSEMENT-V1\0";
const REPLICATED_ENDORSEMENT_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-REPLICATED-ENDORSEMENT-V1\0";
const CONFIGURATION_DOMAIN: &[u8] = b"ADL-COMMITTED-AUTHORITY-CONFIGURATION-V1\0";
const QUORUM_CONFIG_DOMAIN: &[u8] = b"ADL-SEALED-QUORUM-CONFIG-V1\0";
const MAX_IDENTITY_BYTES: usize = 128;
const MAX_ARTIFACT_BYTES: usize = 1024 * 1024;
const MAX_PUBLISHED_OPERATIONS: usize = 4096;
const PROTOCOL_INSTANCE_VERSION: &str = "adl.committed-authority-protocol.v1";
pub const CONTINUITY_TRANSFER_ADAPTER_210: &str = "adl.runtime.continuity-transfer.adapter.210.v1";

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
    ObservatoryServing,
}

impl AuthorityOperationKind {
    fn artifact_domain(self) -> &'static str {
        match self {
            Self::Membership => "adl.authority-artifact.membership.v1",
            Self::Reconciliation => "adl.authority-artifact.reconciliation.v1",
            Self::ExistingStore => "adl.authority-artifact.existing-store.v1",
            Self::ContinuityTransfer => "adl.authority-artifact.continuity-transfer.v1",
            Self::ObservatoryServing => "adl.observatory-serving-authority-binding.v1",
        }
    }

    fn from_artifact_domain(domain: &str) -> AuthorityProtocolResult<Self> {
        [
            Self::Membership,
            Self::Reconciliation,
            Self::ExistingStore,
            Self::ContinuityTransfer,
            Self::ObservatoryServing,
        ]
        .into_iter()
        .find(|kind| kind.artifact_domain() == domain)
        .ok_or(AuthorityProtocolError::ArtifactMismatch)
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

    pub fn continuity_transfer(
        grant: &ContinuityTransferGrantArtifact,
    ) -> AuthorityProtocolResult<Self> {
        grant.validate()?;
        Self::new(
            AuthorityOperationKind::ContinuityTransfer,
            serde_jcs::to_vec(grant).map_err(|_| AuthorityProtocolError::Serialization)?,
        )
    }

    #[cfg(feature = "internal-test-fixtures")]
    pub fn observatory_serving_fixture(bytes: Vec<u8>) -> AuthorityProtocolResult<Self> {
        Self::new(AuthorityOperationKind::ObservatoryServing, bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferEntry {
    pub schema: String,
    pub absolute_start: u64,
    pub length: u64,
    pub sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferChunk {
    pub index: u64,
    pub absolute_start: u64,
    pub length: u64,
    pub sha256: [u8; 32],
    pub predecessor_sha256: Option<[u8; 32]>,
}

/// Store-native, signed continuity authorization bytes retained by the
/// committed artifact. This is data, not a transfer or source-access handle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferGrantArtifact {
    pub trust_domain: String,
    pub polis_id: String,
    pub source_guardian_id: String,
    pub target_guardian_id: String,
    pub route_id: String,
    pub membership_epoch: u64,
    pub membership_log_index: u64,
    pub source_certificate_generation: u64,
    pub target_certificate_generation: u64,
    pub source_boot_generation: u64,
    pub target_boot_generation: u64,
    pub transfer_id: String,
    pub lineage_id: Vec<u8>,
    pub source_checkpoint_handle_identity: Vec<u8>,
    pub bundle_handle_identity: Vec<u8>,
    pub signed_manifest_bytes: Vec<u8>,
    pub signed_manifest_sha256: [u8; 32],
    pub signed_catalog_bytes: Vec<u8>,
    pub signed_catalog_sha256: [u8; 32],
    pub trusted_key_generation: u64,
    pub entries: Vec<ContinuityTransferEntry>,
    pub chunks: Vec<ContinuityTransferChunk>,
    pub total_bytes: u64,
    pub inclusive_deadline: CanonicalAuthorityTime,
    pub cleanup_identity: String,
}

impl ContinuityTransferGrantArtifact {
    fn validate(&self) -> AuthorityProtocolResult<()> {
        for value in [
            &self.trust_domain,
            &self.polis_id,
            &self.source_guardian_id,
            &self.target_guardian_id,
            &self.route_id,
            &self.transfer_id,
            &self.cleanup_identity,
        ] {
            validate_identifier(value)?;
        }
        self.inclusive_deadline.validate()?;
        if self.source_guardian_id == self.target_guardian_id
            || self.membership_epoch == 0
            || self.membership_log_index == 0
            || self.source_certificate_generation == 0
            || self.target_certificate_generation == 0
            || self.source_boot_generation == 0
            || self.target_boot_generation == 0
            || self.trusted_key_generation == 0
            || self.lineage_id.is_empty()
            || self.source_checkpoint_handle_identity.is_empty()
            || self.bundle_handle_identity.is_empty()
            || self.signed_manifest_bytes.is_empty()
            || self.signed_catalog_bytes.is_empty()
            || self.total_bytes == 0
            || self.entries.is_empty()
            || self.chunks.is_empty()
            || self.signed_manifest_sha256
                != <[u8; 32]>::from(Sha256::digest(&self.signed_manifest_bytes))
            || self.signed_catalog_sha256
                != <[u8; 32]>::from(Sha256::digest(&self.signed_catalog_bytes))
        {
            return Err(AuthorityProtocolError::ArtifactMismatch);
        }
        let mut next = 0_u64;
        for (index, entry) in self.entries.iter().enumerate() {
            validate_identifier(&entry.schema)?;
            if entry.length == 0
                || entry.sha256 == [0; 32]
                || entry.absolute_start != next
                || (index > 0 && entry.absolute_start == 0)
            {
                return Err(AuthorityProtocolError::ArtifactMismatch);
            }
            next = next
                .checked_add(entry.length)
                .ok_or(AuthorityProtocolError::ArtifactMismatch)?;
        }
        if next != self.total_bytes {
            return Err(AuthorityProtocolError::ArtifactMismatch);
        }
        let mut chunk_next = 0_u64;
        let mut predecessor = None;
        for (index, chunk) in self.chunks.iter().enumerate() {
            if chunk.index != index as u64
                || chunk.absolute_start != chunk_next
                || chunk.length == 0
                || chunk.sha256 == [0; 32]
                || chunk.predecessor_sha256 != predecessor
            {
                return Err(AuthorityProtocolError::ArtifactMismatch);
            }
            chunk_next = chunk_next
                .checked_add(chunk.length)
                .ok_or(AuthorityProtocolError::ArtifactMismatch)?;
            predecessor = Some(chunk.sha256);
        }
        if chunk_next != self.total_bytes {
            return Err(AuthorityProtocolError::ArtifactMismatch);
        }
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContinuityProjectionConsumer {
    TransferAdapter210,
    #[cfg(test)]
    Other,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct ContinuityProjectionExpectation<'a> {
    consumer: ContinuityProjectionConsumer,
    lineage_id: &'a [u8],
    source_checkpoint_handle_identity: &'a [u8],
    bundle_handle_identity: &'a [u8],
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct ContinuityTransferGrantProjection<'a> {
    artifact: &'a CommittedAuthorityArtifact,
}

/// Checks an exact committed continuity binding but returns no authority,
/// source handle, bytes, or transfer capability to the caller.
pub fn validate_continuity_transfer_binding(
    artifact: &CommittedAuthorityArtifact,
    consumer_id: &str,
    lineage_id: &[u8],
    source_checkpoint_handle_identity: &[u8],
    bundle_handle_identity: &[u8],
) -> AuthorityProtocolResult<()> {
    if consumer_id != CONTINUITY_TRANSFER_ADAPTER_210
        || artifact.domain != AuthorityOperationKind::ContinuityTransfer.artifact_domain()
    {
        return Err(AuthorityProtocolError::WrongVoterPurpose);
    }
    artifact.validate(AuthorityOperationKind::ContinuityTransfer)?;
    let grant: ContinuityTransferGrantArtifact = serde_json::from_slice(&artifact.bytes)
        .map_err(|_| AuthorityProtocolError::ArtifactMismatch)?;
    grant.validate()?;
    if serde_jcs::to_vec(&grant).map_err(|_| AuthorityProtocolError::Serialization)?
        != artifact.bytes
        || grant.lineage_id != lineage_id
        || grant.source_checkpoint_handle_identity != source_checkpoint_handle_identity
        || grant.bundle_handle_identity != bundle_handle_identity
    {
        return Err(AuthorityProtocolError::ArtifactMismatch);
    }
    Ok(())
}

impl ContinuityTransferGrantProjection<'_> {
    #[allow(dead_code)]
    fn decode(&self) -> AuthorityProtocolResult<ContinuityTransferGrantArtifact> {
        serde_json::from_slice(&self.artifact.bytes)
            .map_err(|_| AuthorityProtocolError::ArtifactMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareAuthorityIntent {
    pub polis_id: String,
    pub trust_domain: String,
    pub membership_epoch: u64,
    pub membership_log_index: u64,
    pub prepare_log_index: u64,
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
        prepare_log_index: u64,
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
            prepare_log_index,
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
            || self.prepare_log_index <= self.membership_log_index
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

    pub(crate) fn validate_replicated_shape(&self) -> AuthorityProtocolResult<()> {
        validate_identifier(&self.polis_id)?;
        validate_identifier(&self.trust_domain)?;
        validate_identifier(&self.operation_id)?;
        self.prepare_time.validate()?;
        self.inclusive_deadline.validate()?;
        self.artifact.validate(self.operation_kind)?;
        if self.membership_epoch == 0
            || self.membership_log_index == 0
            || self.prepare_log_index <= self.membership_log_index
            || self.voter_set_generation == 0
            || self.configuration_sha256 == [0; 32]
            || self.expected_protocol_checkpoint_sha256 == [0; 32]
            || self.payload_sha256 == [0; 32]
            || self.payload_sha256 != self.artifact.sha256
            || self.prepare_time.order_key() > self.inclusive_deadline.order_key()
        {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        Ok(())
    }

    pub fn digest(&self) -> AuthorityProtocolResult<[u8; 32]> {
        canonical_domain_digest(INTENT_DOMAIN, self)
    }

    pub(crate) fn validate_against_authority(
        &self,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<()> {
        self.validate_replicated_shape()?;
        if self.trust_domain.as_bytes() != authority.trust_domain_id
            || self.membership_log_index != authority.committed_log_index
            || self.voter_set_generation != authority.voter_set_generation
            || self.configuration_sha256 != configuration_digest(authority)?
        {
            return Err(AuthorityProtocolError::WrongMembership);
        }
        Ok(())
    }
}

/// Caller-index-free proposal that becomes an authority intent only at
/// deterministic OpenRaft state-machine apply.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityPrepareProposal {
    intent_template: PrepareAuthorityIntent,
}

impl AuthorityPrepareProposal {
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
        let mut intent_template = PrepareAuthorityIntent::new(
            polis_id,
            membership,
            authority,
            operation_kind,
            membership
                .committed_log_index()
                .checked_add(1)
                .ok_or(AuthorityProtocolError::StateRegression)?,
            expected_protocol_checkpoint_sha256,
            prepare_time,
            inclusive_deadline,
            operation_id,
            artifact,
        )?;
        intent_template.prepare_log_index = 0;
        Ok(Self { intent_template })
    }

    pub(crate) fn commit_at(
        &self,
        committed_log_index: u64,
        polis_id: &str,
        membership_epoch: u64,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<PrepareAuthorityIntent> {
        if self.intent_template.prepare_log_index != 0
            || self.intent_template.polis_id != polis_id
            || self.intent_template.membership_epoch != membership_epoch
            || committed_log_index <= self.intent_template.membership_log_index
        {
            return Err(AuthorityProtocolError::StateRegression);
        }
        let mut committed = self.intent_template.clone();
        committed.prepare_log_index = committed_log_index;
        committed.validate_against_authority(authority)?;
        Ok(committed)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeAuthorityIntent {
    pub operation_id: String,
    pub intent_sha256: [u8; 32],
    pub finalize_log_index: u64,
    pub finalization_time: CanonicalAuthorityTime,
    pub endorsements: Vec<AuthorityIntentEndorsement>,
}

impl FinalizeAuthorityIntent {
    pub fn new(
        intent: &PrepareAuthorityIntent,
        finalize_log_index: u64,
        finalization_time: CanonicalAuthorityTime,
        endorsements: Vec<AuthorityIntentEndorsement>,
    ) -> AuthorityProtocolResult<Self> {
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
            || finalize_log_index <= intent.prepare_log_index
        {
            return Err(AuthorityProtocolError::TimeOutsideIntent);
        }
        Ok(Self {
            operation_id: intent.operation_id.clone(),
            intent_sha256: intent.digest()?,
            finalize_log_index,
            finalization_time: finalization_time.clone(),
            endorsements,
        })
    }
}

/// Finalization proposal signed after the prepare entry is committed. Its
/// serialized bytes contain no caller-nominated Raft index; apply supplies the
/// only final committed index.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityFinalizeProposal {
    pub operation_id: String,
    pub intent_sha256: [u8; 32],
    pub finalization_time: CanonicalAuthorityTime,
    pub endorsements: Vec<AuthorityIntentEndorsement>,
}

impl AuthorityFinalizeProposal {
    pub fn new(
        intent: &PrepareAuthorityIntent,
        finalization_time: CanonicalAuthorityTime,
        endorsements: Vec<AuthorityIntentEndorsement>,
    ) -> AuthorityProtocolResult<Self> {
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
            || endorsements.is_empty()
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

/// Opaque eligibility boundary used by the pending-exclusion authority.  The
/// protocol owns the call site so a caller cannot self-attest eligibility.
pub(crate) trait AuthorityEligibilityExclusion {
    fn ordinary_authority_allowed(&self, node_id: &str, guardian_id: &[u8]) -> bool;
}

struct VoterEndorsementAuthority {
    node_id: String,
    guardian_id: Vec<u8>,
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    custody: GuardianControlSignerCustody,
}

/// Produces one endorsement through the configured local Guardian identity
/// and the durable pending-membership exclusion view, without exposing raw
/// signing-key material or accepting caller-supplied eligibility booleans.
#[allow(clippy::too_many_arguments)]
pub(crate) fn endorse_committed_authority_prepare_with_exclusion(
    identity: &LocalNodeGuardianIdentity,
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
    intent: &PrepareAuthorityIntent,
    finalization_time: &CanonicalAuthorityTime,
    membership: &MembershipState,
    authority: &AuthorityMembership,
    exclusion: &dyn AuthorityEligibilityExclusion,
) -> AuthorityProtocolResult<AuthorityIntentEndorsement> {
    let voter = VoterEndorsementAuthority::restore_configured(
        identity.authority_signer_custody(),
        certificate_generation,
        boot_generation,
        membership_log_index,
        authoritative_boot_generations,
        membership,
        authority,
    )?;
    if !exclusion.ordinary_authority_allowed(&voter.node_id, &voter.guardian_id) {
        return Err(AuthorityProtocolError::WrongVoter);
    }
    voter.endorse_committed_prepare(
        intent,
        finalization_time,
        membership,
        authority,
        authoritative_boot_generations,
    )
}

impl VoterEndorsementAuthority {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn restore_configured(
        custody: GuardianControlSignerCustody,
        certificate_generation: u64,
        boot_generation: u64,
        membership_log_index: u64,
        authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<Self> {
        validate_membership_pair(membership, authority)?;
        let public = custody.public_identity();
        let node_id = public.node_id.clone();
        let guardian_id = public.guardian_id.as_bytes().to_vec();
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
            || authoritative_boot_generations.get(&guardian_id) != Some(&boot_generation)
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || voter.revoked
            || voter.control_public_key != custody.verifying_key().to_bytes()
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
            custody,
        })
    }

    #[cfg(test)]
    pub(crate) fn endorse(
        &self,
        intent: &PrepareAuthorityIntent,
        finalize_log_index: u64,
        finalization_time: &CanonicalAuthorityTime,
        membership: &MembershipState,
        authority: &AuthorityMembership,
    ) -> AuthorityProtocolResult<AuthorityIntentEndorsement> {
        intent.validate_against(membership, authority)?;
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
            || finalize_log_index <= intent.prepare_log_index
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
            || finalization_time.unix_seconds < voter.not_before_unix_seconds
            || finalization_time.unix_seconds >= voter.not_after_unix_seconds
            || membership
                .member(&self.node_id)
                .is_none_or(|member| member.guardian_id.as_bytes() != self.guardian_id)
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        let payload = endorsement_payload(
            &self.guardian_id,
            self.certificate_generation,
            self.boot_generation,
            self.membership_log_index,
            intent.digest()?,
            intent.prepare_log_index,
            finalize_log_index,
            finalization_time,
        )?;
        Ok(AuthorityIntentEndorsement {
            guardian_id: self.guardian_id.clone(),
            certificate_generation: self.certificate_generation,
            boot_generation: self.boot_generation,
            membership_log_index: self.membership_log_index,
            signature: self.custody.sign(&payload).to_bytes().to_vec(),
        })
    }

    pub(crate) fn endorse_committed_prepare(
        &self,
        intent: &PrepareAuthorityIntent,
        finalization_time: &CanonicalAuthorityTime,
        membership: &MembershipState,
        authority: &AuthorityMembership,
        authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
    ) -> AuthorityProtocolResult<AuthorityIntentEndorsement> {
        intent.validate_against(membership, authority)?;
        finalization_time.validate()?;
        if finalization_time.order_key() < intent.prepare_time.order_key()
            || finalization_time.order_key() > intent.inclusive_deadline.order_key()
        {
            return Err(AuthorityProtocolError::TimeOutsideIntent);
        }
        self.validate_current_custody(
            finalization_time,
            membership,
            authority,
            authoritative_boot_generations,
        )?;
        let payload = replicated_endorsement_payload(
            &self.guardian_id,
            self.certificate_generation,
            self.boot_generation,
            self.membership_log_index,
            intent.digest()?,
            intent.prepare_log_index,
            finalization_time,
        )?;
        Ok(AuthorityIntentEndorsement {
            guardian_id: self.guardian_id.clone(),
            certificate_generation: self.certificate_generation,
            boot_generation: self.boot_generation,
            membership_log_index: self.membership_log_index,
            signature: self.custody.sign(&payload).to_bytes().to_vec(),
        })
    }

    fn validate_current_custody(
        &self,
        finalization_time: &CanonicalAuthorityTime,
        membership: &MembershipState,
        authority: &AuthorityMembership,
        authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
    ) -> AuthorityProtocolResult<()> {
        let voter = authority
            .voters
            .get(&self.guardian_id)
            .ok_or(AuthorityProtocolError::WrongVoter)?;
        if voter.revoked
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || voter.certificate_generation != self.certificate_generation
            || authoritative_boot_generations.get(&self.guardian_id) != Some(&self.boot_generation)
            || authority.committed_log_index != self.membership_log_index
            || finalization_time.unix_seconds < voter.not_before_unix_seconds
            || finalization_time.unix_seconds >= voter.not_after_unix_seconds
            || membership
                .member(&self.node_id)
                .is_none_or(|member| member.guardian_id.as_bytes() != self.guardian_id)
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAuthorityOperation {
    operation_id: String,
    intent_sha256: [u8; 32],
    committed_log_index: u64,
    finalization_time: CanonicalAuthorityTime,
    artifact: CommittedAuthorityArtifact,
    signer_guardian_ids: BTreeSet<Vec<u8>>,
    signer_eligibility: Vec<QuorumEligibilityEntry>,
    inclusive_deadline: CanonicalAuthorityTime,
    quorum_basis: Option<QuorumBasisSnapshot>,
    source: AuthorityVerificationSource,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QuorumEligibilityEntry {
    guardian_id: Vec<u8>,
    certificate_generation: u64,
    boot_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QuorumConfigurationSnapshot {
    entries: Vec<QuorumEligibilityEntry>,
    threshold: usize,
    digest: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QuorumBasisSnapshot {
    configuration_sha256: [u8; 32],
    voter_set_generation: u64,
    committed_membership_log_index: u64,
    configurations: Vec<QuorumConfigurationSnapshot>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthorityVerificationSource {
    #[cfg(test)]
    LegacyDirect,
    ReplicatedApply,
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
    identity: AuthorityNodeIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReconciliationTokenProjection {
    pub(crate) identity: AuthorityNodeIdentity,
    pub(crate) operation_id: String,
    pub(crate) intent_sha256: [u8; 32],
    pub(crate) result_sha256: [u8; 32],
    pub(crate) retry_sha256: [u8; 32],
    pub(crate) committed_log_index: u64,
    pub(crate) finalization_time: CanonicalAuthorityTime,
    pub(crate) artifact: CommittedAuthorityArtifact,
    pub(crate) signer_set_sha256: [u8; 32],
    pub(crate) signer_count: usize,
}

pub(crate) struct ObservatoryAuthoritySourceProjection<'a> {
    pub(crate) trust_domain: &'a str,
    pub(crate) polis_id: &'a str,
    pub(crate) operation_id: &'a str,
    pub(crate) committed_log_index: u64,
    pub(crate) result_sha256: [u8; 32],
    pub(crate) artifact_bytes: &'a [u8],
    pub(crate) artifact_sha256: [u8; 32],
    pub(crate) signer_set_sha256: [u8; 32],
    pub(crate) signer_count: usize,
    pub(crate) inclusive_deadline: &'a CanonicalAuthorityTime,
    pub(crate) finalization_time: &'a CanonicalAuthorityTime,
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
    signer_eligibility: Vec<QuorumEligibilityEntry>,
    inclusive_deadline: CanonicalAuthorityTime,
    quorum_basis: QuorumBasisSnapshot,
}

impl DurablePublishedAuthorityResult {
    fn verified_operation(&self) -> VerifiedAuthorityOperation {
        VerifiedAuthorityOperation {
            operation_id: self.operation_id.clone(),
            intent_sha256: self.intent_sha256,
            committed_log_index: self.committed_log_index,
            finalization_time: self.finalization_time.clone(),
            artifact: self.artifact.clone(),
            signer_guardian_ids: self.signer_guardian_ids.clone(),
            signer_eligibility: self.signer_eligibility.clone(),
            inclusive_deadline: self.inclusive_deadline.clone(),
            quorum_basis: Some(self.quorum_basis.clone()),
            source: AuthorityVerificationSource::ReplicatedApply,
        }
    }

    fn public_result(&self, identity: &AuthorityNodeIdentity) -> PublishedAuthorityResult {
        PublishedAuthorityResult {
            operation_id: self.operation_id.clone(),
            intent_sha256: self.intent_sha256,
            result_sha256: self.result_sha256,
            retry_sha256: self.retry_sha256,
            committed_log_index: self.committed_log_index,
            operation: self.verified_operation(),
            identity: identity.clone(),
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

    /// Exact publishing-node identity for sealed in-crate authority consumers.
    ///
    /// Downstream callers cannot substitute this identity when adapting the
    /// retained committed artifact into a narrower runtime authority.
    pub(crate) fn authority_identity_for_sealed_consumer(&self) -> &AuthorityNodeIdentity {
        &self.identity
    }

    pub(crate) fn reconciliation_projection(
        &self,
    ) -> AuthorityProtocolResult<ReconciliationTokenProjection> {
        let artifact = self.operation.artifact_for_sealed_consumer()?.clone();
        Ok(ReconciliationTokenProjection {
            identity: self.identity.clone(),
            operation_id: self.operation_id.clone(),
            intent_sha256: self.intent_sha256,
            result_sha256: self.result_sha256,
            retry_sha256: self.retry_sha256,
            committed_log_index: self.committed_log_index,
            finalization_time: self.operation.finalization_time.clone(),
            artifact,
            signer_set_sha256: canonical_domain_digest(
                b"ADL-COMMITTED-AUTHORITY-SIGNER-SET-V1\0",
                &self.operation.signer_guardian_ids,
            )?,
            signer_count: self.operation.signer_guardian_ids.len(),
        })
    }

    pub(crate) fn observatory_projection(
        &self,
    ) -> AuthorityProtocolResult<ObservatoryAuthoritySourceProjection<'_>> {
        if self.operation.source != AuthorityVerificationSource::ReplicatedApply
            || self.operation.artifact.domain
                != AuthorityOperationKind::ObservatoryServing.artifact_domain()
        {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        validate_quorum_basis(
            self.operation
                .quorum_basis
                .as_ref()
                .ok_or(AuthorityProtocolError::InvalidMembership)?,
            &self.operation.signer_eligibility,
            true,
        )?;
        if self.operation.finalization_time.order_key()
            > self.operation.inclusive_deadline.order_key()
        {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        Ok(ObservatoryAuthoritySourceProjection {
            trust_domain: &self.identity.trust_domain,
            polis_id: &self.identity.polis_id,
            operation_id: &self.operation_id,
            committed_log_index: self.committed_log_index,
            result_sha256: self.result_sha256,
            artifact_bytes: &self.operation.artifact.bytes,
            artifact_sha256: self.operation.artifact.sha256,
            signer_set_sha256: canonical_domain_digest(
                b"ADL-COMMITTED-AUTHORITY-SIGNER-SET-V1\0",
                &self.operation.signer_eligibility,
            )?,
            signer_count: self.operation.signer_eligibility.len(),
            inclusive_deadline: &self.operation.inclusive_deadline,
            finalization_time: &self.operation.finalization_time,
        })
    }
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_published_authority(bytes: Vec<u8>) -> PublishedAuthorityResult {
    test_observatory_published_authority_for_operation(bytes, "operation", 2)
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_published_authority_for_operation(
    bytes: Vec<u8>,
    operation_id: &str,
    committed_log_index: u64,
) -> PublishedAuthorityResult {
    test_published_reconciliation_token(
        AuthorityNodeIdentity {
            trust_domain: "trust-domain".into(),
            polis_id: "polis".into(),
            node_id: "node".into(),
            guardian_id: "guardian".into(),
            boot_generation: 1,
        },
        operation_id,
        CommittedAuthorityArtifact::observatory_serving_fixture(bytes)
            .expect("observatory fixture artifact"),
        committed_log_index,
        CanonicalAuthorityTime {
            unix_seconds: 1_700_000_000,
            nanos: 123_456_789,
            uncertainty_millis: 1,
        },
    )
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_durable_round_trip(bytes: Vec<u8>) -> PublishedAuthorityResult {
    let published = test_observatory_published_authority(bytes);
    let mut durable = DurablePublishedAuthorityResult {
        operation_id: published.operation_id.clone(),
        intent_sha256: published.intent_sha256,
        result_sha256: published.result_sha256,
        retry_sha256: published.retry_sha256,
        committed_log_index: published.committed_log_index,
        finalization_time: published.operation.finalization_time.clone(),
        artifact: published.operation.artifact.clone(),
        signer_guardian_ids: published.operation.signer_guardian_ids.clone(),
        signer_eligibility: published.operation.signer_eligibility.clone(),
        inclusive_deadline: published.operation.inclusive_deadline.clone(),
        quorum_basis: published.operation.quorum_basis.clone().unwrap(),
    };
    durable.result_sha256 =
        result_digest(&durable.verified_operation(), durable.committed_log_index).unwrap();
    durable.retry_sha256 = retry_digest(&durable).unwrap();
    let state = AuthorityProtocolState {
        committed_log_index: durable.committed_log_index,
        published: BTreeMap::from([(durable.operation_id.clone(), durable)]),
    };
    let reopened: AuthorityProtocolState =
        serde_json::from_slice(&serde_jcs::to_vec(&state).unwrap()).unwrap();
    validate_protocol_state(&reopened).unwrap();
    reopened
        .published
        .get("operation")
        .unwrap()
        .public_result(&published.identity)
}

#[cfg(feature = "internal-test-fixtures")]
#[derive(Clone, Copy, Debug)]
pub enum ObservatoryAuthorityMutation {
    MissingQuorumBasis,
    NonMajorityThreshold,
    OversizedGuardianId,
    EmptySigners,
    SignerGeneration,
    DeadlineBeforeFinalization,
    ExtraSigner,
    ResultDigest,
    RetryDigest,
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_authority_mutation_rejected(
    bytes: Vec<u8>,
    mutation: ObservatoryAuthorityMutation,
) -> bool {
    let mut published = test_observatory_published_authority(bytes);
    match mutation {
        ObservatoryAuthorityMutation::MissingQuorumBasis => {
            published.operation.quorum_basis = None;
        }
        ObservatoryAuthorityMutation::NonMajorityThreshold => {
            let basis = published.operation.quorum_basis.as_mut().unwrap();
            basis.configurations[0].threshold = basis.configurations[0].entries.len();
            if basis.configurations[0].threshold == 1 {
                basis.configurations[0].threshold = 2;
            }
            basis.configurations[0].digest = canonical_domain_digest(
                QUORUM_CONFIG_DOMAIN,
                &(
                    basis.configurations[0].threshold,
                    &basis.configurations[0].entries,
                ),
            )
            .unwrap();
        }
        ObservatoryAuthorityMutation::OversizedGuardianId => {
            let basis = published.operation.quorum_basis.as_mut().unwrap();
            basis.configurations[0].entries[0].guardian_id = vec![b'x'; MAX_IDENTITY_BYTES + 1];
            basis.configurations[0].digest = canonical_domain_digest(
                QUORUM_CONFIG_DOMAIN,
                &(
                    basis.configurations[0].threshold,
                    &basis.configurations[0].entries,
                ),
            )
            .unwrap();
        }
        ObservatoryAuthorityMutation::EmptySigners => {
            published.operation.signer_eligibility.clear();
        }
        ObservatoryAuthorityMutation::SignerGeneration => {
            published.operation.signer_eligibility[0].boot_generation += 1;
        }
        ObservatoryAuthorityMutation::DeadlineBeforeFinalization => {
            published.operation.inclusive_deadline.unix_seconds =
                published.operation.finalization_time.unix_seconds - 1;
        }
        ObservatoryAuthorityMutation::ExtraSigner => {
            published
                .operation
                .signer_eligibility
                .push(QuorumEligibilityEntry {
                    guardian_id: b"extra-guardian".to_vec(),
                    certificate_generation: 1,
                    boot_generation: 1,
                });
        }
        ObservatoryAuthorityMutation::ResultDigest => published.result_sha256[0] ^= 1,
        ObservatoryAuthorityMutation::RetryDigest => published.retry_sha256[0] ^= 1,
    }
    published.observatory_projection().is_err()
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_durable_restore_mutation_rejected(
    bytes: Vec<u8>,
    mutation: ObservatoryAuthorityMutation,
) -> bool {
    let published = test_observatory_published_authority(bytes);
    let mut durable = DurablePublishedAuthorityResult {
        operation_id: published.operation_id.clone(),
        intent_sha256: published.intent_sha256,
        result_sha256: published.result_sha256,
        retry_sha256: published.retry_sha256,
        committed_log_index: published.committed_log_index,
        finalization_time: published.operation.finalization_time.clone(),
        artifact: published.operation.artifact.clone(),
        signer_guardian_ids: published.operation.signer_guardian_ids.clone(),
        signer_eligibility: published.operation.signer_eligibility.clone(),
        inclusive_deadline: published.operation.inclusive_deadline.clone(),
        quorum_basis: published.operation.quorum_basis.clone().unwrap(),
    };
    durable.result_sha256 =
        result_digest(&durable.verified_operation(), durable.committed_log_index).unwrap();
    durable.retry_sha256 = retry_digest(&durable).unwrap();
    match mutation {
        ObservatoryAuthorityMutation::MissingQuorumBasis => return false,
        ObservatoryAuthorityMutation::NonMajorityThreshold => {
            durable.quorum_basis.configurations[0].threshold = 2;
            durable.quorum_basis.configurations[0].digest = canonical_domain_digest(
                QUORUM_CONFIG_DOMAIN,
                &(
                    durable.quorum_basis.configurations[0].threshold,
                    &durable.quorum_basis.configurations[0].entries,
                ),
            )
            .unwrap();
        }
        ObservatoryAuthorityMutation::OversizedGuardianId => {
            durable.quorum_basis.configurations[0].entries[0].guardian_id =
                vec![b'x'; MAX_IDENTITY_BYTES + 1];
        }
        ObservatoryAuthorityMutation::EmptySigners => durable.signer_eligibility.clear(),
        ObservatoryAuthorityMutation::SignerGeneration => {
            durable.signer_eligibility[0].boot_generation += 1;
        }
        ObservatoryAuthorityMutation::DeadlineBeforeFinalization => {
            durable.inclusive_deadline.unix_seconds = durable.finalization_time.unix_seconds - 1;
        }
        ObservatoryAuthorityMutation::ExtraSigner => {
            durable.signer_eligibility.push(QuorumEligibilityEntry {
                guardian_id: b"extra-guardian".to_vec(),
                certificate_generation: 1,
                boot_generation: 1,
            });
        }
        ObservatoryAuthorityMutation::ResultDigest => durable.result_sha256[0] ^= 1,
        ObservatoryAuthorityMutation::RetryDigest => durable.retry_sha256[0] ^= 1,
    }
    let state = AuthorityProtocolState {
        committed_log_index: durable.committed_log_index,
        published: BTreeMap::from([(durable.operation_id.clone(), durable)]),
    };
    let restored: AuthorityProtocolState =
        serde_json::from_slice(&serde_jcs::to_vec(&state).unwrap()).unwrap();
    validate_protocol_state(&restored).is_err()
}

#[cfg(feature = "internal-test-fixtures")]
pub fn test_observatory_legacy_durable_state_rejected(bytes: Vec<u8>) -> bool {
    let published = test_observatory_published_authority(bytes);
    let mut value = serde_json::to_value(AuthorityProtocolState {
        committed_log_index: published.committed_log_index,
        published: BTreeMap::new(),
    })
    .unwrap();
    value["published"] = serde_json::json!({"operation": {
        "operation_id": published.operation_id,
        "intent_sha256": published.intent_sha256,
        "result_sha256": published.result_sha256,
        "retry_sha256": published.retry_sha256,
        "committed_log_index": published.committed_log_index,
        "finalization_time": published.operation.finalization_time,
        "artifact": published.operation.artifact,
        "signer_guardian_ids": published.operation.signer_guardian_ids
    }});
    serde_json::from_value::<AuthorityProtocolState>(value).is_err()
}

#[cfg(any(test, feature = "internal-test-fixtures"))]
pub(crate) fn test_published_reconciliation_token(
    identity: AuthorityNodeIdentity,
    operation_id: &str,
    artifact: CommittedAuthorityArtifact,
    committed_log_index: u64,
    finalization_time: CanonicalAuthorityTime,
) -> PublishedAuthorityResult {
    let signer_eligibility = vec![QuorumEligibilityEntry {
        guardian_id: b"test-guardian-a".to_vec(),
        certificate_generation: 1,
        boot_generation: 1,
    }];
    let entries = signer_eligibility.clone();
    let threshold = 1;
    let configuration_entries = vec![vec![b"test-guardian-a".to_vec()]];
    let quorum_basis = QuorumBasisSnapshot {
        configuration_sha256: canonical_domain_digest(CONFIGURATION_DOMAIN, &configuration_entries)
            .expect("fixture configuration digest"),
        voter_set_generation: 1,
        committed_membership_log_index: 1,
        configurations: vec![QuorumConfigurationSnapshot {
            digest: canonical_domain_digest(QUORUM_CONFIG_DOMAIN, &(threshold, &entries))
                .expect("fixture digest"),
            entries,
            threshold,
        }],
    };
    let intent_sha256: [u8; 32] = Sha256::digest(
        [
            b"ADL-TEST-PUBLISHED-RECONCILIATION-TOKEN-V1\0".as_slice(),
            operation_id.as_bytes(),
            artifact.sha256.as_slice(),
            &committed_log_index.to_be_bytes(),
        ]
        .concat(),
    )
    .into();
    let result_sha256: [u8; 32] =
        Sha256::digest([intent_sha256.as_slice(), b"result".as_slice()].concat()).into();
    let retry_sha256: [u8; 32] =
        Sha256::digest([intent_sha256.as_slice(), b"retry".as_slice()].concat()).into();
    PublishedAuthorityResult {
        operation_id: operation_id.to_owned(),
        intent_sha256,
        result_sha256,
        retry_sha256,
        committed_log_index,
        operation: VerifiedAuthorityOperation {
            operation_id: operation_id.to_owned(),
            intent_sha256,
            committed_log_index,
            finalization_time: finalization_time.clone(),
            artifact,
            signer_guardian_ids: BTreeSet::from([b"test-guardian-a".to_vec()]),
            signer_eligibility,
            inclusive_deadline: CanonicalAuthorityTime {
                unix_seconds: finalization_time.unix_seconds + 60,
                nanos: finalization_time.nanos,
                uncertainty_millis: finalization_time.uncertainty_millis,
            },
            quorum_basis: Some(quorum_basis),
            source: AuthorityVerificationSource::ReplicatedApply,
        },
        identity,
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TestReconciliationTokenMutation {
    TrustDomain,
    Polis,
    Node,
    Guardian,
    BootGeneration,
    Membership,
}

#[cfg(test)]
pub(crate) fn mutate_test_reconciliation_token(
    token: &mut PublishedAuthorityResult,
    mutation: TestReconciliationTokenMutation,
) {
    match mutation {
        TestReconciliationTokenMutation::TrustDomain => token.identity.trust_domain.push('x'),
        TestReconciliationTokenMutation::Polis => token.identity.polis_id.push('x'),
        TestReconciliationTokenMutation::Node => token.identity.node_id.push('x'),
        TestReconciliationTokenMutation::Guardian => token.identity.guardian_id.push('x'),
        TestReconciliationTokenMutation::BootGeneration => {
            token.identity.boot_generation = token.identity.boot_generation.saturating_add(1)
        }
        TestReconciliationTokenMutation::Membership => token.operation.signer_guardian_ids.clear(),
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
        validate_protocol_state(envelope.payload())?;
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
            .map(|result| result.public_result(&self.identity))
    }

    pub(crate) fn publish(
        &mut self,
        intent: &PrepareAuthorityIntent,
        verified: VerifiedAuthorityOperation,
    ) -> AuthorityProtocolResult<PublishedAuthorityResult> {
        if verified.source != AuthorityVerificationSource::ReplicatedApply {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        self.publish_verified(intent, verified)
    }

    #[cfg(test)]
    pub(crate) fn publish_test_only(
        &mut self,
        intent: &PrepareAuthorityIntent,
        verified: VerifiedAuthorityOperation,
    ) -> AuthorityProtocolResult<PublishedAuthorityResult> {
        self.publish(intent, verified)
    }

    fn publish_verified(
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
                && existing.public_result(&self.identity).operation == verified
                && existing.retry_sha256 == retry_digest(existing)?
                && existing.result_sha256
                    == result_digest(
                        &existing.public_result(&self.identity).operation,
                        existing.committed_log_index,
                    )?
            {
                return Ok(existing.public_result(&self.identity));
            }
            return Err(AuthorityProtocolError::RetryConflict);
        }
        if intent.expected_protocol_checkpoint_sha256 != self.checkpoint_sha256()? {
            return Err(AuthorityProtocolError::StateRegression);
        }
        if intent.prepare_log_index <= self.envelope.payload().committed_log_index
            || verified.committed_log_index <= self.envelope.payload().committed_log_index
        {
            return Err(AuthorityProtocolError::StateRegression);
        }
        if self.envelope.payload().published.len() >= self.capacity {
            return Err(AuthorityProtocolError::CapacityExceeded);
        }
        let quorum_basis = verified
            .quorum_basis
            .clone()
            .ok_or(AuthorityProtocolError::InvalidMembership)?;
        let result_sha256 = result_digest(&verified, verified.committed_log_index)?;
        let mut result = DurablePublishedAuthorityResult {
            operation_id: intent.operation_id.clone(),
            intent_sha256: verified.intent_sha256,
            result_sha256,
            retry_sha256: [0; 32],
            committed_log_index: verified.committed_log_index,
            finalization_time: verified.finalization_time.clone(),
            artifact: verified.artifact.clone(),
            signer_guardian_ids: verified.signer_guardian_ids.clone(),
            signer_eligibility: verified.signer_eligibility.clone(),
            inclusive_deadline: verified.inclusive_deadline.clone(),
            quorum_basis,
        };
        result.retry_sha256 = retry_digest(&result)?;
        let mut next = self.envelope.payload().clone();
        next.committed_log_index = next.committed_log_index.max(verified.committed_log_index);
        next.published
            .insert(intent.operation_id.clone(), result.clone());
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(result.public_result(&self.identity))
    }
}

fn validate_protocol_state(state: &AuthorityProtocolState) -> AuthorityProtocolResult<()> {
    if state.published.len() > MAX_PUBLISHED_OPERATIONS {
        return Err(AuthorityProtocolError::CapacityExceeded);
    }
    for (operation_id, durable) in &state.published {
        if operation_id != &durable.operation_id
            || durable.operation_id.is_empty()
            || durable.intent_sha256 == [0; 32]
            || durable.committed_log_index == 0
            || durable.committed_log_index > state.committed_log_index
        {
            return Err(AuthorityProtocolError::StateRegression);
        }
        durable
            .artifact
            .validate(AuthorityOperationKind::from_artifact_domain(
                &durable.artifact.domain,
            )?)?;
        durable.inclusive_deadline.validate()?;
        if durable.finalization_time.order_key() > durable.inclusive_deadline.order_key()
            || durable.signer_guardian_ids
                != durable
                    .signer_eligibility
                    .iter()
                    .map(|entry| entry.guardian_id.clone())
                    .collect()
        {
            return Err(AuthorityProtocolError::StateRegression);
        }
        validate_quorum_basis(&durable.quorum_basis, &durable.signer_eligibility, true)?;
        if durable.result_sha256
            != result_digest(&durable.verified_operation(), durable.committed_log_index)?
            || durable.retry_sha256 != retry_digest(durable)?
        {
            return Err(AuthorityProtocolError::StateRegression);
        }
    }
    Ok(())
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

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    // The sealed downstream adapters land in their owning issues. Keep this
    // crate-private accessor non-public without weakening the issue-local
    // warning gate while those consumers remain absent.
    #[allow(dead_code)]
    pub(crate) fn artifact_for_sealed_consumer(
        &self,
    ) -> AuthorityProtocolResult<&CommittedAuthorityArtifact> {
        if self.source != AuthorityVerificationSource::ReplicatedApply {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        Ok(&self.artifact)
    }

    #[allow(dead_code)]
    fn continuity_projection<'a>(
        &'a self,
        expected: ContinuityProjectionExpectation<'_>,
    ) -> AuthorityProtocolResult<ContinuityTransferGrantProjection<'a>> {
        if self.source != AuthorityVerificationSource::ReplicatedApply {
            return Err(AuthorityProtocolError::InvalidIntent);
        }
        if expected.consumer != ContinuityProjectionConsumer::TransferAdapter210 {
            return Err(AuthorityProtocolError::WrongVoterPurpose);
        }
        validate_continuity_transfer_binding(
            &self.artifact,
            CONTINUITY_TRANSFER_ADAPTER_210,
            expected.lineage_id,
            expected.source_checkpoint_handle_identity,
            expected.bundle_handle_identity,
        )?;
        Ok(ContinuityTransferGrantProjection {
            artifact: &self.artifact,
        })
    }
}

#[cfg(test)]
pub(crate) fn verify_finalization(
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
        || finalize.finalize_log_index <= intent.prepare_log_index
        || finalize.finalization_time.order_key() < intent.prepare_time.order_key()
        || finalize.finalization_time.order_key() > intent.inclusive_deadline.order_key()
    {
        return Err(AuthorityProtocolError::InvalidIntent);
    }
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
            || finalize.finalization_time.unix_seconds < voter.not_before_unix_seconds
            || finalize.finalization_time.unix_seconds >= voter.not_after_unix_seconds
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        let payload = endorsement_payload(
            &endorsement.guardian_id,
            endorsement.certificate_generation,
            endorsement.boot_generation,
            endorsement.membership_log_index,
            intent_sha256,
            intent.prepare_log_index,
            finalize.finalize_log_index,
            &finalize.finalization_time,
        )?;
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
        committed_log_index: finalize.finalize_log_index,
        finalization_time: finalize.finalization_time.clone(),
        artifact: intent.artifact.clone(),
        signer_guardian_ids: signers,
        signer_eligibility: signer_eligibility(&finalize.endorsements),
        inclusive_deadline: intent.inclusive_deadline.clone(),
        quorum_basis: None,
        source: AuthorityVerificationSource::LegacyDirect,
    })
}

pub(crate) fn verify_replicated_finalization(
    intent: &PrepareAuthorityIntent,
    finalize: &AuthorityFinalizeProposal,
    committed_log_index: u64,
    authority: &AuthorityMembership,
    authoritative_boot_generations: &BTreeMap<Vec<u8>, u64>,
) -> AuthorityProtocolResult<VerifiedAuthorityOperation> {
    intent.validate_against_authority(authority)?;
    finalize.finalization_time.validate()?;
    let intent_sha256 = intent.digest()?;
    if committed_log_index <= intent.prepare_log_index
        || finalize.operation_id != intent.operation_id
        || finalize.intent_sha256 != intent_sha256
        || finalize.finalization_time.order_key() < intent.prepare_time.order_key()
        || finalize.finalization_time.order_key() > intent.inclusive_deadline.order_key()
    {
        return Err(AuthorityProtocolError::InvalidIntent);
    }
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
            || authoritative_boot_generations.get(&endorsement.guardian_id)
                != Some(&endorsement.boot_generation)
            || endorsement.membership_log_index != authority.committed_log_index
            || finalize.finalization_time.unix_seconds < voter.not_before_unix_seconds
            || finalize.finalization_time.unix_seconds >= voter.not_after_unix_seconds
        {
            return Err(AuthorityProtocolError::StaleVoter);
        }
        let payload = replicated_endorsement_payload(
            &endorsement.guardian_id,
            endorsement.certificate_generation,
            endorsement.boot_generation,
            endorsement.membership_log_index,
            intent_sha256,
            intent.prepare_log_index,
            &finalize.finalization_time,
        )?;
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
        committed_log_index,
        finalization_time: finalize.finalization_time.clone(),
        artifact: intent.artifact.clone(),
        signer_guardian_ids: signers,
        signer_eligibility: signer_eligibility(&finalize.endorsements),
        inclusive_deadline: intent.inclusive_deadline.clone(),
        quorum_basis: Some(quorum_basis_snapshot(
            authority,
            authoritative_boot_generations,
        )?),
        source: AuthorityVerificationSource::ReplicatedApply,
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

fn quorum_basis_snapshot(
    authority: &AuthorityMembership,
    boot_generations: &BTreeMap<Vec<u8>, u64>,
) -> AuthorityProtocolResult<QuorumBasisSnapshot> {
    let guardian_by_raft = authority
        .raft_ids
        .iter()
        .map(|(guardian, raft_id)| (*raft_id, guardian.clone()))
        .collect::<BTreeMap<_, _>>();
    let configurations = authority
        .raft_membership
        .get_joint_config()
        .iter()
        .map(|config| {
            let mut entries = config
                .iter()
                .map(|raft_id| {
                    let guardian_id = guardian_by_raft
                        .get(raft_id)
                        .ok_or(AuthorityProtocolError::InvalidMembership)?;
                    let voter = authority
                        .voters
                        .get(guardian_id)
                        .ok_or(AuthorityProtocolError::InvalidMembership)?;
                    let boot_generation = *boot_generations
                        .get(guardian_id)
                        .ok_or(AuthorityProtocolError::StaleVoter)?;
                    if voter.certificate_generation == 0 || boot_generation == 0 {
                        return Err(AuthorityProtocolError::StaleVoter);
                    }
                    Ok(QuorumEligibilityEntry {
                        guardian_id: guardian_id.clone(),
                        certificate_generation: voter.certificate_generation,
                        boot_generation,
                    })
                })
                .collect::<AuthorityProtocolResult<Vec<_>>>()?;
            entries.sort_by(|left, right| left.guardian_id.cmp(&right.guardian_id));
            let threshold = entries.len() / 2 + 1;
            let digest = canonical_domain_digest(QUORUM_CONFIG_DOMAIN, &(threshold, &entries))?;
            Ok(QuorumConfigurationSnapshot {
                entries,
                threshold,
                digest,
            })
        })
        .collect::<AuthorityProtocolResult<Vec<_>>>()?;
    let snapshot = QuorumBasisSnapshot {
        configuration_sha256: configuration_digest(authority)?,
        voter_set_generation: authority.voter_set_generation,
        committed_membership_log_index: authority.committed_log_index,
        configurations,
    };
    validate_quorum_basis(&snapshot, &[], false)?;
    Ok(snapshot)
}

fn signer_eligibility(endorsements: &[AuthorityIntentEndorsement]) -> Vec<QuorumEligibilityEntry> {
    let mut entries = endorsements
        .iter()
        .map(|entry| QuorumEligibilityEntry {
            guardian_id: entry.guardian_id.clone(),
            certificate_generation: entry.certificate_generation,
            boot_generation: entry.boot_generation,
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.guardian_id.cmp(&right.guardian_id));
    entries
}

fn validate_quorum_basis(
    snapshot: &QuorumBasisSnapshot,
    signers: &[QuorumEligibilityEntry],
    require_quorum: bool,
) -> AuthorityProtocolResult<()> {
    if snapshot.configuration_sha256 == [0; 32]
        || snapshot.voter_set_generation == 0
        || snapshot.committed_membership_log_index == 0
        || snapshot.configurations.is_empty()
        || snapshot.configurations.len() > 2
    {
        return Err(AuthorityProtocolError::InvalidMembership);
    }
    let configuration_entries = snapshot
        .configurations
        .iter()
        .map(|config| {
            config
                .entries
                .iter()
                .map(|entry| entry.guardian_id.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if snapshot.configuration_sha256
        != canonical_domain_digest(CONFIGURATION_DOMAIN, &configuration_entries)?
    {
        return Err(AuthorityProtocolError::InvalidMembership);
    }
    for config in &snapshot.configurations {
        if config.entries.is_empty()
            || config.entries.len() > 4096
            || config.threshold != config.entries.len() / 2 + 1
            || config
                .entries
                .windows(2)
                .any(|pair| pair[0].guardian_id >= pair[1].guardian_id)
            || config.entries.iter().any(|entry| {
                entry.guardian_id.is_empty()
                    || entry.guardian_id.len() > MAX_IDENTITY_BYTES
                    || entry.certificate_generation == 0
                    || entry.boot_generation == 0
            })
            || config.digest
                != canonical_domain_digest(
                    QUORUM_CONFIG_DOMAIN,
                    &(config.threshold, &config.entries),
                )?
            || (require_quorum
                && signers
                    .iter()
                    .filter(|signer| config.entries.contains(signer))
                    .count()
                    < config.threshold)
        {
            return Err(AuthorityProtocolError::MissingQuorum);
        }
    }
    if require_quorum
        && (signers.is_empty()
            || signers
                .windows(2)
                .any(|pair| pair[0].guardian_id >= pair[1].guardian_id)
            || signers.iter().any(|signer| {
                !snapshot
                    .configurations
                    .iter()
                    .any(|config| config.entries.contains(signer))
            }))
    {
        return Err(AuthorityProtocolError::MissingQuorum);
    }
    Ok(())
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

#[cfg(test)]
fn endorsement_payload(
    guardian_id: &[u8],
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    intent_sha256: [u8; 32],
    prepare_log_index: u64,
    finalize_log_index: u64,
    finalization_time: &CanonicalAuthorityTime,
) -> AuthorityProtocolResult<Vec<u8>> {
    let body = serde_jcs::to_vec(&(
        guardian_id,
        certificate_generation,
        boot_generation,
        membership_log_index,
        intent_sha256,
        prepare_log_index,
        finalize_log_index,
        finalization_time,
    ))
    .map_err(|_| AuthorityProtocolError::Serialization)?;
    let mut payload = Vec::with_capacity(ENDORSEMENT_DOMAIN.len() + body.len());
    payload.extend_from_slice(ENDORSEMENT_DOMAIN);
    payload.extend_from_slice(&body);
    Ok(payload)
}

fn replicated_endorsement_payload(
    guardian_id: &[u8],
    certificate_generation: u64,
    boot_generation: u64,
    membership_log_index: u64,
    intent_sha256: [u8; 32],
    prepare_log_index: u64,
    finalization_time: &CanonicalAuthorityTime,
) -> AuthorityProtocolResult<Vec<u8>> {
    let body = serde_jcs::to_vec(&(
        guardian_id,
        certificate_generation,
        boot_generation,
        membership_log_index,
        intent_sha256,
        prepare_log_index,
        finalization_time,
    ))
    .map_err(|_| AuthorityProtocolError::Serialization)?;
    let mut payload = Vec::with_capacity(REPLICATED_ENDORSEMENT_DOMAIN.len() + body.len());
    payload.extend_from_slice(REPLICATED_ENDORSEMENT_DOMAIN);
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
            operation.committed_log_index,
            &operation.finalization_time,
            &operation.artifact,
            &operation.signer_guardian_ids,
            &operation.signer_eligibility,
            &operation.inclusive_deadline,
            &operation.quorum_basis,
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
            &result.signer_eligibility,
            &result.inclusive_deadline,
            &result.quorum_basis,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn marker(case: &str, result: &str) {
        println!("ADL_ISSUE_201_CASE_V1 {case} {result}");
    }

    fn grant() -> ContinuityTransferGrantArtifact {
        let manifest = b"signed-manifest".to_vec();
        let catalog = b"signed-catalog".to_vec();
        ContinuityTransferGrantArtifact {
            trust_domain: "trust-domain".into(),
            polis_id: "polis-a".into(),
            source_guardian_id: "guardian-a".into(),
            target_guardian_id: "guardian-b".into(),
            route_id: "route-a".into(),
            membership_epoch: 7,
            membership_log_index: 19,
            source_certificate_generation: 3,
            target_certificate_generation: 4,
            source_boot_generation: 5,
            target_boot_generation: 6,
            transfer_id: "transfer-a".into(),
            lineage_id: b"lineage-a".to_vec(),
            source_checkpoint_handle_identity: b"source-handle-a".to_vec(),
            bundle_handle_identity: b"bundle-handle-a".to_vec(),
            signed_manifest_sha256: Sha256::digest(&manifest).into(),
            signed_manifest_bytes: manifest,
            signed_catalog_sha256: Sha256::digest(&catalog).into(),
            signed_catalog_bytes: catalog,
            trusted_key_generation: 8,
            entries: vec![ContinuityTransferEntry {
                schema: "kernel.page.v1".into(),
                absolute_start: 0,
                length: 4,
                sha256: [9; 32],
            }],
            chunks: vec![ContinuityTransferChunk {
                index: 0,
                absolute_start: 0,
                length: 4,
                sha256: [10; 32],
                predecessor_sha256: None,
            }],
            total_bytes: 4,
            inclusive_deadline: CanonicalAuthorityTime {
                unix_seconds: 1_800_000_000,
                nanos: 0,
                uncertainty_millis: 2,
            },
            cleanup_identity: "cleanup-a".into(),
        }
    }

    fn operation() -> VerifiedAuthorityOperation {
        let signer_eligibility = vec![
            QuorumEligibilityEntry {
                guardian_id: b"guardian-a".to_vec(),
                certificate_generation: 1,
                boot_generation: 1,
            },
            QuorumEligibilityEntry {
                guardian_id: b"guardian-b".to_vec(),
                certificate_generation: 1,
                boot_generation: 1,
            },
        ];
        let threshold = 2;
        let configuration_sha256 = canonical_domain_digest(
            CONFIGURATION_DOMAIN,
            &vec![vec![b"guardian-a".to_vec(), b"guardian-b".to_vec()]],
        )
        .unwrap();
        VerifiedAuthorityOperation {
            operation_id: "continuity-a".into(),
            intent_sha256: [7; 32],
            committed_log_index: 21,
            finalization_time: CanonicalAuthorityTime {
                unix_seconds: 1_799_999_999,
                nanos: 0,
                uncertainty_millis: 2,
            },
            artifact: CommittedAuthorityArtifact::continuity_transfer(&grant()).unwrap(),
            signer_guardian_ids: [b"guardian-a".to_vec(), b"guardian-b".to_vec()]
                .into_iter()
                .collect(),
            signer_eligibility: signer_eligibility.clone(),
            inclusive_deadline: grant().inclusive_deadline,
            quorum_basis: Some(QuorumBasisSnapshot {
                configuration_sha256,
                voter_set_generation: 1,
                committed_membership_log_index: 1,
                configurations: vec![QuorumConfigurationSnapshot {
                    digest: canonical_domain_digest(
                        QUORUM_CONFIG_DOMAIN,
                        &(threshold, &signer_eligibility),
                    )
                    .unwrap(),
                    entries: signer_eligibility,
                    threshold,
                }],
            }),
            source: AuthorityVerificationSource::ReplicatedApply,
        }
    }

    fn expectation<'a>(
        lineage: &'a [u8],
        source: &'a [u8],
        bundle: &'a [u8],
    ) -> ContinuityProjectionExpectation<'a> {
        ContinuityProjectionExpectation {
            consumer: ContinuityProjectionConsumer::TransferAdapter210,
            lineage_id: lineage,
            source_checkpoint_handle_identity: source,
            bundle_handle_identity: bundle,
        }
    }

    #[test]
    fn sealed_continuity_transfer_projection() {
        let operation = operation();
        let projection = operation
            .continuity_projection(expectation(
                b"lineage-a",
                b"source-handle-a",
                b"bundle-handle-a",
            ))
            .unwrap();
        assert_eq!(projection.decode().unwrap(), grant());
        marker("sealed_continuity_transfer_projection", "passed");
    }

    #[test]
    fn continuity_projection_consumer_confusion_rejected() {
        let verified = operation();
        let mut expected = expectation(b"lineage-a", b"source-handle-a", b"bundle-handle-a");
        expected.consumer = ContinuityProjectionConsumer::Other;
        assert_eq!(
            verified.continuity_projection(expected).err(),
            Some(AuthorityProtocolError::WrongVoterPurpose)
        );
        let mut legacy = operation();
        legacy.source = AuthorityVerificationSource::LegacyDirect;
        assert_eq!(
            legacy
                .continuity_projection(expectation(
                    b"lineage-a",
                    b"source-handle-a",
                    b"bundle-handle-a",
                ))
                .err(),
            Some(AuthorityProtocolError::InvalidIntent)
        );
        marker(
            "continuity_projection_consumer_confusion_rejected",
            "rejected",
        );
    }

    #[test]
    fn continuity_projection_wrong_lineage_rejected() {
        assert_eq!(
            operation()
                .continuity_projection(expectation(
                    b"wrong-lineage",
                    b"source-handle-a",
                    b"bundle-handle-a"
                ))
                .err(),
            Some(AuthorityProtocolError::ArtifactMismatch)
        );
        marker("continuity_projection_wrong_lineage_rejected", "rejected");
    }

    #[test]
    fn continuity_projection_wrong_source_checkpoint_handle_rejected() {
        assert_eq!(
            operation()
                .continuity_projection(expectation(
                    b"lineage-a",
                    b"wrong-source",
                    b"bundle-handle-a"
                ))
                .err(),
            Some(AuthorityProtocolError::ArtifactMismatch)
        );
        marker(
            "continuity_projection_wrong_source_checkpoint_handle_rejected",
            "rejected",
        );
    }

    #[test]
    fn continuity_projection_wrong_bundle_handle_rejected() {
        assert_eq!(
            operation()
                .continuity_projection(expectation(
                    b"lineage-a",
                    b"source-handle-a",
                    b"wrong-bundle"
                ))
                .err(),
            Some(AuthorityProtocolError::ArtifactMismatch)
        );
        marker(
            "continuity_projection_wrong_bundle_handle_rejected",
            "rejected",
        );
    }
}

#[cfg(test)]
#[path = "authority_protocol_contract_tests.rs"]
mod contract_tests;
