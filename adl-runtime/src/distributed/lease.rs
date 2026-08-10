use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use openraft::Membership;
use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const AUTHORITY_CERTIFICATE_SCHEMA_VERSION: u32 = 1;
pub const SIGNING_ALGORITHM_ED25519: u32 = 1;
pub const BODY_DOMAIN: &[u8] = b"ADL-AUTHORITY-CERTIFICATE-BODY-V1\0";
pub const ENDORSEMENT_DOMAIN: &[u8] = b"ADL-AUTHORITY-ENDORSEMENT-V1\0";
pub const ACTIVATION_DOMAIN: &[u8] = b"ADL-AUTHORITY-ACTIVATION-V1\0";
pub const POLICY_DOMAIN: &[u8] = b"ADL-AUTHORITY-LEASE-POLICY-V1\0";
pub const AUTHORITY_SNAPSHOT_SCHEMA: &str = "adl.distributed.authority_ledger_snapshot.v1";

const MAX_IDENTITY_BYTES: usize = 128;
const MAX_VOTERS: usize = 4096;
const MAX_CERTIFICATE_BYTES: usize = 1024 * 1024;
const SHA256_BYTES: usize = 32;
const SIGNATURE_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum OperationClass {
    LeaseGrant = 1,
    LeaseRenewal = 2,
    Fence = 3,
    Activate = 4,
    OwnerCommit = 5,
    Revoke = 6,
}

impl OperationClass {
    fn parse(value: u32) -> AuthorityResult<Self> {
        match value {
            1 => Ok(Self::LeaseGrant),
            2 => Ok(Self::LeaseRenewal),
            3 => Ok(Self::Fence),
            4 => Ok(Self::Activate),
            5 => Ok(Self::OwnerCommit),
            6 => Ok(Self::Revoke),
            _ => Err(AuthorityError::InvalidOperationClass),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Message, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityCertificateBodyV1 {
    #[prost(uint32, tag = "1")]
    pub schema_version: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub trust_domain_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub lineage_id: Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub voter_set_generation: u64,
    #[prost(uint64, tag = "5")]
    pub raft_term: u64,
    #[prost(uint64, tag = "6")]
    pub committed_log_index: u64,
    #[prost(uint64, tag = "7")]
    pub epoch: u64,
    #[prost(bytes = "vec", tag = "8")]
    pub holder_node_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub holder_guardian_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "10")]
    pub activation_key_sha256: Vec<u8>,
    #[prost(uint32, tag = "11")]
    pub operation_class: u32,
    #[prost(int64, tag = "12")]
    pub issued_unix_seconds: i64,
    #[prost(uint32, tag = "13")]
    pub issued_nanos: u32,
    #[prost(uint64, tag = "14")]
    pub lease_duration_millis: u64,
    #[prost(bytes = "vec", tag = "15")]
    pub policy_sha256: Vec<u8>,
    #[prost(uint32, tag = "16")]
    pub signing_algorithm: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityEndorsementV1 {
    #[prost(bytes = "vec", tag = "1")]
    pub signer_guardian_id: Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub certificate_generation: u64,
    #[prost(uint32, tag = "3")]
    pub signing_algorithm: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct AuthorityEndorsementPayloadV1 {
    #[prost(bytes = "vec", tag = "1")]
    pub certificate_body_sha256: Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub signer_guardian_id: Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub certificate_generation: u64,
    #[prost(uint32, tag = "4")]
    pub signing_algorithm: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityCertificateV1 {
    #[prost(message, optional, tag = "1")]
    pub body: Option<AuthorityCertificateBodyV1>,
    #[prost(message, repeated, tag = "2")]
    pub endorsements: Vec<AuthorityEndorsementV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ControlCertificatePurpose {
    AuthorityEndorsement,
    Transport,
    Discovery,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoterAuthority {
    pub guardian_id: Vec<u8>,
    pub trust_domain_id: Vec<u8>,
    pub certificate_generation: u64,
    pub purpose: ControlCertificatePurpose,
    pub not_before_unix_seconds: i64,
    pub not_after_unix_seconds: i64,
    pub revoked: bool,
    pub control_public_key: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityMembership {
    pub trust_domain_id: Vec<u8>,
    pub voter_set_generation: u64,
    pub committed_log_index: u64,
    pub raft_membership: Membership<u64, ()>,
    pub raft_ids: BTreeMap<Vec<u8>, u64>,
    pub voters: BTreeMap<Vec<u8>, VoterAuthority>,
}

impl AuthorityMembership {
    pub fn new(
        trust_domain_id: Vec<u8>,
        voter_set_generation: u64,
        committed_log_index: u64,
        configs: Vec<BTreeSet<Vec<u8>>>,
        voters: Vec<VoterAuthority>,
    ) -> AuthorityResult<Self> {
        if !valid_identity(&trust_domain_id)
            || voter_set_generation == 0
            || committed_log_index == 0
            || configs.is_empty()
            || configs.len() > 2
            || configs
                .iter()
                .any(|config| config.len() < 3 || config.len() > MAX_VOTERS)
        {
            return Err(AuthorityError::InvalidMembership);
        }
        let voters = voters
            .into_iter()
            .map(|voter| (voter.guardian_id.clone(), voter))
            .collect::<BTreeMap<_, _>>();
        let all_ids = configs.iter().flatten().cloned().collect::<BTreeSet<_>>();
        if voters.len() != all_ids.len()
            || voters.keys().any(|id| !all_ids.contains(id))
            || voters.values().any(|voter| {
                !valid_identity(&voter.guardian_id)
                    || voter.trust_domain_id != trust_domain_id
                    || voter.certificate_generation == 0
                    || voter.not_before_unix_seconds <= 0
                    || voter.not_after_unix_seconds <= voter.not_before_unix_seconds
                    || voter.control_public_key == [0; 32]
                    || VerifyingKey::from_bytes(&voter.control_public_key).is_err()
            })
        {
            return Err(AuthorityError::InvalidMembership);
        }
        let unique_keys = voters
            .values()
            .map(|voter| voter.control_public_key)
            .collect::<BTreeSet<_>>();
        if unique_keys.len() != voters.len() {
            return Err(AuthorityError::DuplicateControlKey);
        }
        let raft_ids = all_ids
            .iter()
            .enumerate()
            .map(|(index, id)| (id.clone(), index as u64 + 1))
            .collect::<BTreeMap<_, _>>();
        let raft_configs = configs
            .iter()
            .map(|config| {
                config
                    .iter()
                    .map(|id| raft_ids[id])
                    .collect::<BTreeSet<_>>()
            })
            .collect();
        Ok(Self {
            trust_domain_id,
            voter_set_generation,
            committed_log_index,
            raft_membership: Membership::new(raft_configs, ()),
            raft_ids,
            voters,
        })
    }

    fn has_quorum(&self, signers: &BTreeSet<Vec<u8>>) -> bool {
        let signer_raft_ids = signers
            .iter()
            .filter_map(|id| self.raft_ids.get(id).copied())
            .collect::<BTreeSet<_>>();
        self.raft_membership
            .get_joint_config()
            .iter()
            .all(|config| {
                config
                    .iter()
                    .filter(|id| signer_raft_ids.contains(id))
                    .count()
                    > config.len() / 2
            })
    }
}

#[derive(Clone, Debug)]
pub struct LeasePolicy {
    pub max_lease_duration_millis: u64,
    pub max_clock_uncertainty_millis: u64,
    pub message_delay_margin_millis: u64,
    pub max_snapshot_bytes: usize,
}

impl LeasePolicy {
    pub fn validate(&self) -> AuthorityResult<()> {
        if self.max_lease_duration_millis == 0
            || self.max_lease_duration_millis > 86_400_000
            || self.max_clock_uncertainty_millis > 60_000
            || self.message_delay_margin_millis > 60_000
            || !(1024..=16 * 1024 * 1024).contains(&self.max_snapshot_bytes)
        {
            return Err(AuthorityError::InvalidPolicy);
        }
        Ok(())
    }

    pub fn sha256(&self) -> AuthorityResult<[u8; 32]> {
        self.validate()?;
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&self.max_lease_duration_millis.to_be_bytes());
        bytes.extend_from_slice(&self.max_clock_uncertainty_millis.to_be_bytes());
        bytes.extend_from_slice(&self.message_delay_margin_millis.to_be_bytes());
        bytes.extend_from_slice(&(self.max_snapshot_bytes as u64).to_be_bytes());
        Ok(domain_digest(POLICY_DOMAIN, &bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeaseState {
    pub lineage_id: Vec<u8>,
    pub holder_node_id: Vec<u8>,
    pub holder_guardian_id: Vec<u8>,
    pub activation_public_key: [u8; 32],
    pub raft_term: u64,
    pub committed_log_index: u64,
    pub epoch: u64,
    pub certificate_generation: u64,
    pub activated_elapsed_millis: u64,
    pub deadline_elapsed_millis: u64,
    pub certificate_bytes: Vec<u8>,
    pub revoked: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAuthority {
    pub body: AuthorityCertificateBodyV1,
    pub body_sha256: [u8; 32],
    pub signer_guardian_ids: BTreeSet<Vec<u8>>,
}

#[derive(Clone, Copy, Debug)]
pub struct AuthorityApplication<'a> {
    pub now_unix_seconds: i64,
    pub now_elapsed_millis: u64,
    pub clock_uncertainty_millis: u64,
    pub activation_public_key: [u8; 32],
    pub activation_proof: &'a [u8],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityError {
    InvalidPolicy,
    InvalidMembership,
    DuplicateControlKey,
    ResourceExhausted,
    MalformedEncoding,
    NonCanonicalEncoding,
    InvalidCertificate,
    InvalidOperationClass,
    WrongTrustDomain,
    StaleMembership,
    StaleAppliedIndex,
    DuplicateSigner,
    InvalidEndorsement,
    CertificateUnauthorized,
    PolicyMismatch,
    QuorumNotReached,
    ActivationPossession,
    ClockUncertain,
    StaleTerm,
    StaleEpoch,
    EpochGap,
    HolderMismatch,
    LeaseExpired,
    LeaseRevoked,
    Replay,
    SnapshotCorrupt,
}

impl AuthorityError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidMembership => "invalid_membership",
            Self::DuplicateControlKey => "duplicate_control_key",
            Self::ResourceExhausted => "resource_exhausted",
            Self::MalformedEncoding => "malformed_encoding",
            Self::NonCanonicalEncoding => "non_canonical_encoding",
            Self::InvalidCertificate => "invalid_certificate",
            Self::InvalidOperationClass => "invalid_operation_class",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::StaleMembership => "stale_membership",
            Self::StaleAppliedIndex => "stale_applied_index",
            Self::DuplicateSigner => "duplicate_signer",
            Self::InvalidEndorsement => "invalid_endorsement",
            Self::CertificateUnauthorized => "certificate_unauthorized",
            Self::PolicyMismatch => "policy_mismatch",
            Self::QuorumNotReached => "quorum_not_reached",
            Self::ActivationPossession => "activation_possession_failed",
            Self::ClockUncertain => "clock_uncertain",
            Self::StaleTerm => "stale_term",
            Self::StaleEpoch => "stale_epoch",
            Self::EpochGap => "epoch_gap",
            Self::HolderMismatch => "holder_mismatch",
            Self::LeaseExpired => "lease_expired",
            Self::LeaseRevoked => "lease_revoked",
            Self::Replay => "replay",
            Self::SnapshotCorrupt => "snapshot_corrupt",
        }
    }
}

impl fmt::Display for AuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for AuthorityError {}
pub type AuthorityResult<T> = Result<T, AuthorityError>;

pub fn encode_certificate(certificate: &AuthorityCertificateV1) -> AuthorityResult<Vec<u8>> {
    let bytes = certificate.encode_to_vec();
    if bytes.len() > MAX_CERTIFICATE_BYTES {
        return Err(AuthorityError::ResourceExhausted);
    }
    validate_certificate_wire(&bytes)?;
    Ok(bytes)
}

pub fn decode_certificate(bytes: &[u8]) -> AuthorityResult<AuthorityCertificateV1> {
    if bytes.is_empty() || bytes.len() > MAX_CERTIFICATE_BYTES {
        return Err(AuthorityError::ResourceExhausted);
    }
    validate_certificate_wire(bytes)?;
    let certificate =
        AuthorityCertificateV1::decode(bytes).map_err(|_| AuthorityError::MalformedEncoding)?;
    if certificate.encode_to_vec() != bytes {
        return Err(AuthorityError::NonCanonicalEncoding);
    }
    Ok(certificate)
}

pub fn certificate_body_sha256(body: &AuthorityCertificateBodyV1) -> [u8; 32] {
    domain_digest(BODY_DOMAIN, &body.encode_to_vec())
}

pub fn endorsement_payload_digest(
    body_sha256: [u8; 32],
    signer_guardian_id: &[u8],
    certificate_generation: u64,
) -> [u8; 32] {
    let payload = AuthorityEndorsementPayloadV1 {
        certificate_body_sha256: body_sha256.to_vec(),
        signer_guardian_id: signer_guardian_id.to_vec(),
        certificate_generation,
        signing_algorithm: SIGNING_ALGORITHM_ED25519,
    };
    domain_digest(ENDORSEMENT_DOMAIN, &payload.encode_to_vec())
}

pub fn endorse(
    body: &AuthorityCertificateBodyV1,
    signer_guardian_id: Vec<u8>,
    certificate_generation: u64,
    signing_key: &SigningKey,
) -> AuthorityEndorsementV1 {
    let digest = endorsement_payload_digest(
        certificate_body_sha256(body),
        &signer_guardian_id,
        certificate_generation,
    );
    AuthorityEndorsementV1 {
        signer_guardian_id,
        certificate_generation,
        signing_algorithm: SIGNING_ALGORITHM_ED25519,
        signature: signing_key.sign(&digest).to_bytes().to_vec(),
    }
}

pub fn activation_signature(body: &AuthorityCertificateBodyV1, key: &SigningKey) -> [u8; 64] {
    key.sign(&domain_digest(
        ACTIVATION_DOMAIN,
        &certificate_body_sha256(body),
    ))
    .to_bytes()
}

pub fn verify_certificate(
    bytes: &[u8],
    membership: &AuthorityMembership,
    now_unix_seconds: i64,
) -> AuthorityResult<VerifiedAuthority> {
    if now_unix_seconds <= 0 {
        return Err(AuthorityError::InvalidCertificate);
    }
    let certificate = decode_certificate(bytes)?;
    let body = certificate.body.ok_or(AuthorityError::InvalidCertificate)?;
    validate_body(&body)?;
    if body.trust_domain_id != membership.trust_domain_id {
        return Err(AuthorityError::WrongTrustDomain);
    }
    if body.voter_set_generation != membership.voter_set_generation {
        return Err(AuthorityError::StaleMembership);
    }
    if body.committed_log_index > membership.committed_log_index {
        return Err(AuthorityError::StaleAppliedIndex);
    }
    let digest = certificate_body_sha256(&body);
    let mut signer_ids = BTreeSet::new();
    let mut signer_keys = BTreeSet::new();
    let mut previous: Option<&[u8]> = None;
    for endorsement in &certificate.endorsements {
        if previous.is_some_and(|prior| prior >= endorsement.signer_guardian_id.as_slice()) {
            return Err(AuthorityError::NonCanonicalEncoding);
        }
        previous = Some(&endorsement.signer_guardian_id);
        if endorsement.signing_algorithm != SIGNING_ALGORITHM_ED25519
            || endorsement.signature.len() != SIGNATURE_BYTES
        {
            return Err(AuthorityError::InvalidEndorsement);
        }
        let voter = membership
            .voters
            .get(&endorsement.signer_guardian_id)
            .ok_or(AuthorityError::InvalidEndorsement)?;
        if voter.trust_domain_id != body.trust_domain_id
            || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
            || voter.revoked
            || now_unix_seconds < voter.not_before_unix_seconds
            || now_unix_seconds >= voter.not_after_unix_seconds
        {
            return Err(AuthorityError::CertificateUnauthorized);
        }
        if voter.certificate_generation != endorsement.certificate_generation {
            return Err(AuthorityError::StaleMembership);
        }
        if !signer_ids.insert(endorsement.signer_guardian_id.clone())
            || !signer_keys.insert(voter.control_public_key)
        {
            return Err(AuthorityError::DuplicateSigner);
        }
        let key = VerifyingKey::from_bytes(&voter.control_public_key)
            .map_err(|_| AuthorityError::InvalidEndorsement)?;
        let signature = Signature::from_slice(&endorsement.signature)
            .map_err(|_| AuthorityError::InvalidEndorsement)?;
        let signed = endorsement_payload_digest(
            digest,
            &endorsement.signer_guardian_id,
            endorsement.certificate_generation,
        );
        key.verify_strict(&signed, &signature)
            .map_err(|_| AuthorityError::InvalidEndorsement)?;
    }
    if !membership.has_quorum(&signer_ids) {
        return Err(AuthorityError::QuorumNotReached);
    }
    Ok(VerifiedAuthority {
        body,
        body_sha256: digest,
        signer_guardian_ids: signer_ids,
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotBody {
    schema: String,
    applied_log_index: u64,
    last_raft_term: u64,
    leases: Vec<LeaseState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEnvelope {
    body: SnapshotBody,
    digest: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct AuthorityLedger {
    policy: LeasePolicy,
    applied_log_index: u64,
    last_raft_term: u64,
    leases: BTreeMap<Vec<u8>, LeaseState>,
}

impl AuthorityLedger {
    pub fn new(policy: LeasePolicy) -> AuthorityResult<Self> {
        policy.validate()?;
        Ok(Self {
            policy,
            applied_log_index: 0,
            last_raft_term: 0,
            leases: BTreeMap::new(),
        })
    }

    pub fn applied_log_index(&self) -> u64 {
        self.applied_log_index
    }

    pub fn lease(&self, lineage_id: &[u8]) -> Option<&LeaseState> {
        self.leases.get(lineage_id)
    }

    pub fn apply(
        &mut self,
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        application: AuthorityApplication<'_>,
    ) -> AuthorityResult<&LeaseState> {
        if application.clock_uncertainty_millis > self.policy.max_clock_uncertainty_millis {
            return Err(AuthorityError::ClockUncertain);
        }
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)?;
        let body = &verified.body;
        let operation = OperationClass::parse(body.operation_class)?;
        if body.committed_log_index <= self.applied_log_index {
            return Err(AuthorityError::Replay);
        }
        if body.raft_term < self.last_raft_term {
            return Err(AuthorityError::StaleTerm);
        }
        if body.lease_duration_millis > self.policy.max_lease_duration_millis {
            return Err(AuthorityError::InvalidPolicy);
        }
        if body.policy_sha256 != self.policy.sha256()?.as_slice() {
            return Err(AuthorityError::PolicyMismatch);
        }
        if body.issued_unix_seconds > application.now_unix_seconds {
            return Err(AuthorityError::InvalidCertificate);
        }
        verify_activation(
            body,
            application.activation_public_key,
            application.activation_proof,
        )?;

        let current = self.leases.get(&body.lineage_id).cloned();
        match operation {
            OperationClass::LeaseGrant | OperationClass::Activate => {
                let expected = current
                    .as_ref()
                    .map_or(1, |lease| lease.epoch.saturating_add(1));
                if body.epoch < expected {
                    return Err(AuthorityError::StaleEpoch);
                }
                if body.epoch != expected {
                    return Err(AuthorityError::EpochGap);
                }
                if let Some(previous) = current.as_ref() {
                    let safety_deadline = previous
                        .deadline_elapsed_millis
                        .checked_add(self.policy.max_clock_uncertainty_millis)
                        .and_then(|value| {
                            value.checked_add(self.policy.message_delay_margin_millis)
                        })
                        .ok_or(AuthorityError::ResourceExhausted)?;
                    if application.now_elapsed_millis < safety_deadline {
                        return Err(AuthorityError::LeaseExpired);
                    }
                }
            }
            OperationClass::LeaseRenewal => {
                let previous = current.as_ref().ok_or(AuthorityError::HolderMismatch)?;
                if previous.revoked
                    || previous.epoch != body.epoch
                    || previous.holder_node_id != body.holder_node_id
                    || previous.holder_guardian_id != body.holder_guardian_id
                    || previous.activation_public_key != application.activation_public_key
                {
                    return Err(AuthorityError::HolderMismatch);
                }
                if application.now_elapsed_millis >= previous.deadline_elapsed_millis {
                    return Err(AuthorityError::LeaseExpired);
                }
            }
            OperationClass::Revoke | OperationClass::Fence => {
                let previous = self
                    .leases
                    .get_mut(&body.lineage_id)
                    .ok_or(AuthorityError::HolderMismatch)?;
                if body.epoch != previous.epoch
                    || body.holder_node_id != previous.holder_node_id
                    || body.holder_guardian_id != previous.holder_guardian_id
                {
                    return Err(AuthorityError::HolderMismatch);
                }
                previous.revoked = true;
                previous.committed_log_index = body.committed_log_index;
                previous.raft_term = body.raft_term;
                previous.certificate_bytes = certificate_bytes.to_vec();
                self.applied_log_index = body.committed_log_index;
                self.last_raft_term = body.raft_term;
                return Ok(previous);
            }
            OperationClass::OwnerCommit => {
                let previous = current.as_ref().ok_or(AuthorityError::HolderMismatch)?;
                if previous.revoked
                    || previous.epoch != body.epoch
                    || previous.holder_node_id != body.holder_node_id
                    || previous.holder_guardian_id != body.holder_guardian_id
                    || previous.activation_public_key != application.activation_public_key
                {
                    return Err(AuthorityError::HolderMismatch);
                }
                if application.now_elapsed_millis >= previous.deadline_elapsed_millis {
                    return Err(AuthorityError::LeaseExpired);
                }
                let previous = self
                    .leases
                    .get_mut(&body.lineage_id)
                    .ok_or(AuthorityError::HolderMismatch)?;
                previous.committed_log_index = body.committed_log_index;
                previous.raft_term = body.raft_term;
                previous.certificate_bytes = certificate_bytes.to_vec();
                self.applied_log_index = body.committed_log_index;
                self.last_raft_term = body.raft_term;
                return Ok(previous);
            }
        }
        let deadline = application
            .now_elapsed_millis
            .checked_add(body.lease_duration_millis)
            .ok_or(AuthorityError::ResourceExhausted)?;
        let state = LeaseState {
            lineage_id: body.lineage_id.clone(),
            holder_node_id: body.holder_node_id.clone(),
            holder_guardian_id: body.holder_guardian_id.clone(),
            activation_public_key: application.activation_public_key,
            raft_term: body.raft_term,
            committed_log_index: body.committed_log_index,
            epoch: body.epoch,
            certificate_generation: body.voter_set_generation,
            activated_elapsed_millis: application.now_elapsed_millis,
            deadline_elapsed_millis: deadline,
            certificate_bytes: certificate_bytes.to_vec(),
            revoked: false,
        };
        self.applied_log_index = body.committed_log_index;
        self.last_raft_term = body.raft_term;
        self.leases.insert(body.lineage_id.clone(), state);
        self.leases
            .get(&body.lineage_id)
            .ok_or(AuthorityError::SnapshotCorrupt)
    }

    pub fn authorize_mutation(
        &self,
        lineage_id: &[u8],
        holder_guardian_id: &[u8],
        epoch: u64,
        now_elapsed_millis: u64,
        applied_log_index: u64,
    ) -> AuthorityResult<()> {
        let lease = self
            .leases
            .get(lineage_id)
            .ok_or(AuthorityError::LeaseExpired)?;
        if lease.revoked {
            return Err(AuthorityError::LeaseRevoked);
        }
        if lease.holder_guardian_id != holder_guardian_id || lease.epoch != epoch {
            return Err(AuthorityError::HolderMismatch);
        }
        if applied_log_index < lease.committed_log_index {
            return Err(AuthorityError::StaleAppliedIndex);
        }
        if now_elapsed_millis >= lease.deadline_elapsed_millis {
            return Err(AuthorityError::LeaseExpired);
        }
        Ok(())
    }

    pub fn snapshot(&self) -> AuthorityResult<Vec<u8>> {
        let body = SnapshotBody {
            schema: AUTHORITY_SNAPSHOT_SCHEMA.to_owned(),
            applied_log_index: self.applied_log_index,
            last_raft_term: self.last_raft_term,
            leases: self.leases.values().cloned().collect(),
        };
        let body_bytes = serde_jcs::to_vec(&body).map_err(|_| AuthorityError::SnapshotCorrupt)?;
        let envelope = SnapshotEnvelope {
            body,
            digest: Sha256::digest(body_bytes).into(),
        };
        let bytes = serde_jcs::to_vec(&envelope).map_err(|_| AuthorityError::SnapshotCorrupt)?;
        if bytes.len() > self.policy.max_snapshot_bytes {
            return Err(AuthorityError::ResourceExhausted);
        }
        Ok(bytes)
    }

    pub fn restore(policy: LeasePolicy, bytes: &[u8]) -> AuthorityResult<Self> {
        policy.validate()?;
        if bytes.is_empty() || bytes.len() > policy.max_snapshot_bytes {
            return Err(AuthorityError::ResourceExhausted);
        }
        let envelope: SnapshotEnvelope =
            serde_json::from_slice(bytes).map_err(|_| AuthorityError::SnapshotCorrupt)?;
        let mut leases = BTreeMap::new();
        let leases_valid = envelope.body.leases.iter().all(|lease| {
            validate_snapshot_lease(
                lease,
                envelope.body.applied_log_index,
                envelope.body.last_raft_term,
            ) && leases
                .insert(lease.lineage_id.clone(), lease.clone())
                .is_none()
        });
        if envelope.body.schema != AUTHORITY_SNAPSHOT_SCHEMA
            || envelope.digest
                != <[u8; 32]>::from(Sha256::digest(
                    serde_jcs::to_vec(&envelope.body)
                        .map_err(|_| AuthorityError::SnapshotCorrupt)?,
                ))
            || serde_jcs::to_vec(&envelope).map_err(|_| AuthorityError::SnapshotCorrupt)? != bytes
            || !leases_valid
            || leases
                .values()
                .map(|lease| lease.committed_log_index)
                .max()
                .unwrap_or(0)
                != envelope.body.applied_log_index
        {
            return Err(AuthorityError::SnapshotCorrupt);
        }
        for lease in leases.values_mut() {
            // A restored process does not possess the prior activation private
            // key. It retains the committed prefix but starts fenced until a
            // quorum commits a safe higher-epoch activation.
            lease.revoked = true;
        }
        Ok(Self {
            policy,
            applied_log_index: envelope.body.applied_log_index,
            last_raft_term: envelope.body.last_raft_term,
            leases,
        })
    }
}

fn validate_snapshot_lease(
    lease: &LeaseState,
    applied_log_index: u64,
    last_raft_term: u64,
) -> bool {
    let Ok(certificate) = decode_certificate(&lease.certificate_bytes) else {
        return false;
    };
    let Some(body) = certificate.body else {
        return false;
    };
    validate_body(&body).is_ok()
        && valid_identity(&lease.lineage_id)
        && valid_identity(&lease.holder_node_id)
        && valid_identity(&lease.holder_guardian_id)
        && VerifyingKey::from_bytes(&lease.activation_public_key).is_ok()
        && lease.raft_term > 0
        && lease.raft_term <= last_raft_term
        && lease.committed_log_index > 0
        && lease.committed_log_index <= applied_log_index
        && lease.epoch > 0
        && lease.certificate_generation > 0
        && lease.deadline_elapsed_millis >= lease.activated_elapsed_millis
        && body.lineage_id == lease.lineage_id
        && body.holder_node_id == lease.holder_node_id
        && body.holder_guardian_id == lease.holder_guardian_id
        && body.raft_term == lease.raft_term
        && body.committed_log_index == lease.committed_log_index
        && body.epoch == lease.epoch
        && body.voter_set_generation == lease.certificate_generation
        && body.activation_key_sha256
            == <[u8; 32]>::from(Sha256::digest(lease.activation_public_key)).as_slice()
}

fn validate_body(body: &AuthorityCertificateBodyV1) -> AuthorityResult<()> {
    OperationClass::parse(body.operation_class)?;
    if body.schema_version != AUTHORITY_CERTIFICATE_SCHEMA_VERSION
        || !valid_identity(&body.trust_domain_id)
        || !valid_identity(&body.lineage_id)
        || !valid_identity(&body.holder_node_id)
        || !valid_identity(&body.holder_guardian_id)
        || body.voter_set_generation == 0
        || body.raft_term == 0
        || body.committed_log_index == 0
        || body.epoch == 0
        || body.activation_key_sha256.len() != SHA256_BYTES
        || body.policy_sha256.len() != SHA256_BYTES
        || body.issued_unix_seconds <= 0
        || body.issued_nanos >= 1_000_000_000
        || body.lease_duration_millis == 0
        || body.signing_algorithm != SIGNING_ALGORITHM_ED25519
    {
        return Err(AuthorityError::InvalidCertificate);
    }
    Ok(())
}

fn verify_activation(
    body: &AuthorityCertificateBodyV1,
    activation_public_key: [u8; 32],
    activation_proof: &[u8],
) -> AuthorityResult<()> {
    if <[u8; 32]>::from(Sha256::digest(activation_public_key))
        != body.activation_key_sha256.as_slice()
    {
        return Err(AuthorityError::ActivationPossession);
    }
    let key = VerifyingKey::from_bytes(&activation_public_key)
        .map_err(|_| AuthorityError::ActivationPossession)?;
    let signature = Signature::from_slice(activation_proof)
        .map_err(|_| AuthorityError::ActivationPossession)?;
    key.verify_strict(
        &domain_digest(ACTIVATION_DOMAIN, &certificate_body_sha256(body)),
        &signature,
    )
    .map_err(|_| AuthorityError::ActivationPossession)
}

fn valid_identity(value: &[u8]) -> bool {
    !value.is_empty() && value.len() <= MAX_IDENTITY_BYTES
}

fn domain_digest(domain: &[u8], message: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(message);
    digest.finalize().into()
}

fn validate_certificate_wire(bytes: &[u8]) -> AuthorityResult<()> {
    let fields = parse_wire(bytes, &[(1, 2), (2, 2)], &[1])?;
    let body = fields
        .iter()
        .find(|field| field.tag == 1)
        .ok_or(AuthorityError::MalformedEncoding)?;
    parse_wire(
        body.value,
        &[
            (1, 0),
            (2, 2),
            (3, 2),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 2),
            (9, 2),
            (10, 2),
            (11, 0),
            (12, 0),
            (13, 0),
            (14, 0),
            (15, 2),
            (16, 0),
        ],
        &(1_u32..=16).collect::<Vec<_>>(),
    )?;
    for endorsement in fields.iter().filter(|field| field.tag == 2) {
        parse_wire(
            endorsement.value,
            &[(1, 2), (2, 0), (3, 0), (4, 2)],
            &[1, 2, 3, 4],
        )?;
    }
    Ok(())
}

struct WireField<'a> {
    tag: u32,
    value: &'a [u8],
}

fn parse_wire<'a>(
    bytes: &'a [u8],
    allowed: &[(u32, u8)],
    singular_required: &[u32],
) -> AuthorityResult<Vec<WireField<'a>>> {
    let mut cursor = 0;
    let mut fields = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous_tag = 0;
    while cursor < bytes.len() {
        let (key, key_len) = decode_minimal_varint(&bytes[cursor..])?;
        cursor += key_len;
        let tag = u32::try_from(key >> 3).map_err(|_| AuthorityError::MalformedEncoding)?;
        let wire = (key & 7) as u8;
        if tag == 0 || !allowed.contains(&(tag, wire)) || tag < previous_tag {
            return Err(AuthorityError::NonCanonicalEncoding);
        }
        previous_tag = tag;
        if singular_required.contains(&tag) && !seen.insert(tag) {
            return Err(AuthorityError::NonCanonicalEncoding);
        }
        let value = match wire {
            0 => {
                let (_, len) = decode_minimal_varint(&bytes[cursor..])?;
                let value = &bytes[cursor..cursor + len];
                cursor += len;
                value
            }
            2 => {
                let (length, len_len) = decode_minimal_varint(&bytes[cursor..])?;
                cursor += len_len;
                let length =
                    usize::try_from(length).map_err(|_| AuthorityError::MalformedEncoding)?;
                let end = cursor
                    .checked_add(length)
                    .filter(|end| *end <= bytes.len())
                    .ok_or(AuthorityError::MalformedEncoding)?;
                let value = &bytes[cursor..end];
                cursor = end;
                value
            }
            _ => return Err(AuthorityError::NonCanonicalEncoding),
        };
        fields.push(WireField { tag, value });
    }
    if singular_required.iter().any(|tag| !seen.contains(tag)) {
        return Err(AuthorityError::MalformedEncoding);
    }
    Ok(fields)
}

fn decode_minimal_varint(bytes: &[u8]) -> AuthorityResult<(u64, usize)> {
    let mut value = 0_u64;
    for (index, byte) in bytes.iter().take(10).enumerate() {
        let payload = u64::from(byte & 0x7f);
        if index == 9 && payload > 1 {
            return Err(AuthorityError::MalformedEncoding);
        }
        value |= payload << (index * 7);
        if byte & 0x80 == 0 {
            if index > 0 && payload == 0 {
                return Err(AuthorityError::NonCanonicalEncoding);
            }
            return Ok((value, index + 1));
        }
    }
    Err(AuthorityError::MalformedEncoding)
}
