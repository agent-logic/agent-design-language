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
pub const MUTATION_DOMAIN: &[u8] = b"ADL-AUTHORITY-MUTATION-V1\0";
pub const AUTHORITY_SNAPSHOT_SCHEMA: &str = "adl.distributed.authority_ledger_snapshot.v1";

const MAX_IDENTITY_BYTES: usize = 128;
const MAX_VOTERS: usize = 4096;
const MAX_LINEAGES: usize = 4096;
const MAX_CERTIFICATE_BYTES: usize = 1024 * 1024;
const SHA256_BYTES: usize = 32;
const SIGNATURE_BYTES: usize = 64;

mod raw_access {
    const LEASE_STORE_ACCESS_MAGIC: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x4c, 0x45, 0x41, 0x53, 0x45, 0x2d, 0x53, 0x54, 0x4f, 0x52, 0x45,
        0x2d, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x56, 0x31, 0x2d, 0x53, 0x45, 0x41, 0x4c,
        0x03, 0x59,
    ];

    #[derive(Debug)]
    struct LeaseStoreAccessSeal {
        magic: [u8; 32],
    }

    static AUTHORITY_BOUND_SEAL: LeaseStoreAccessSeal = LeaseStoreAccessSeal {
        magic: LEASE_STORE_ACCESS_MAGIC,
    };

    #[cfg(any(test, feature = "internal-test-fixtures"))]
    static TEST_FIXTURE_SEAL: LeaseStoreAccessSeal = LeaseStoreAccessSeal {
        magic: LEASE_STORE_ACCESS_MAGIC,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct LeaseStoreAccess {
        seal: &'static LeaseStoreAccessSeal,
    }

    pub(crate) const AUTHORITY_BOUND: LeaseStoreAccess = LeaseStoreAccess {
        seal: &AUTHORITY_BOUND_SEAL,
    };

    #[cfg(test)]
    pub(crate) const TEST_FIXTURE: LeaseStoreAccess = LeaseStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    #[cfg(all(not(test), feature = "internal-test-fixtures"))]
    #[doc(hidden)]
    pub const TEST_FIXTURE: LeaseStoreAccess = LeaseStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    pub(super) fn validate(access: &LeaseStoreAccess) -> bool {
        #[cfg(any(test, feature = "internal-test-fixtures"))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)
            || std::ptr::eq(access.seal, &TEST_FIXTURE_SEAL);
        #[cfg(not(any(test, feature = "internal-test-fixtures")))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL);
        known_seal && access.seal.magic == LEASE_STORE_ACCESS_MAGIC
    }
}

pub use raw_access::LeaseStoreAccess;
#[allow(unused_imports)]
pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_LEASE_ACCESS;
#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use raw_access::TEST_FIXTURE as TEST_LEASE_STORE_ACCESS;
#[cfg(all(not(test), feature = "internal-test-fixtures"))]
#[doc(hidden)]
#[allow(unused_imports)]
pub use raw_access::TEST_FIXTURE as TEST_LEASE_STORE_ACCESS;

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

#[derive(Clone, PartialEq, Message)]
pub struct MutationAuthorizationPayloadV1 {
    #[prost(bytes = "vec", tag = "1")]
    pub lineage_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub holder_guardian_id: Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub epoch: u64,
    #[prost(uint64, tag = "4")]
    pub applied_log_index: u64,
    #[prost(uint64, tag = "5")]
    pub sequence: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub mutation_sha256: Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub certificate_sha256: Vec<u8>,
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
        Self::new_with_stable_ids(
            trust_domain_id,
            voter_set_generation,
            committed_log_index,
            configs,
            voters.into_values().collect(),
            raft_ids,
        )
    }

    pub fn new_with_stable_ids(
        trust_domain_id: Vec<u8>,
        voter_set_generation: u64,
        committed_log_index: u64,
        configs: Vec<BTreeSet<Vec<u8>>>,
        voters: Vec<VoterAuthority>,
        raft_ids: BTreeMap<Vec<u8>, u64>,
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
            || raft_ids.len() != all_ids.len()
            || raft_ids.keys().any(|id| !all_ids.contains(id))
            || raft_ids.values().any(|id| *id == 0)
            || raft_ids.values().copied().collect::<BTreeSet<_>>().len() != raft_ids.len()
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
    pub max_lineages: usize,
    pub max_snapshot_bytes: usize,
}

impl LeasePolicy {
    pub fn validate(&self) -> AuthorityResult<()> {
        if self.max_lease_duration_millis == 0
            || self.max_lease_duration_millis > 86_400_000
            || self.max_clock_uncertainty_millis > 60_000
            || self.message_delay_margin_millis > 60_000
            || !(1..=MAX_LINEAGES).contains(&self.max_lineages)
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
        bytes.extend_from_slice(&(self.max_lineages as u64).to_be_bytes());
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
    pub deadline_unix_millis: u64,
    pub certificate_bytes: Vec<u8>,
    pub revoked: bool,
    pub last_mutation_sequence: u64,
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
    pub now_unix_nanos: u32,
    pub now_elapsed_millis: u64,
    pub clock_uncertainty_millis: u64,
    pub activation_public_key: [u8; 32],
    pub activation_proof: &'a [u8],
}

#[derive(Clone, Copy, Debug)]
pub struct MutationAuthorization<'a> {
    pub lineage_id: &'a [u8],
    pub holder_guardian_id: &'a [u8],
    pub epoch: u64,
    pub now_elapsed_millis: u64,
    pub applied_log_index: u64,
    pub sequence: u64,
    pub mutation_sha256: [u8; 32],
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
    AuthorityAlreadyExists,
    AuthorityRequired,
    HolderMismatch,
    LeaseExpired,
    LeaseRevoked,
    Replay,
    SnapshotCorrupt,
    RevisionDrift,
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
            Self::AuthorityAlreadyExists => "authority_already_exists",
            Self::AuthorityRequired => "authority_required",
            Self::HolderMismatch => "holder_mismatch",
            Self::LeaseExpired => "lease_expired",
            Self::LeaseRevoked => "lease_revoked",
            Self::Replay => "replay",
            Self::SnapshotCorrupt => "snapshot_corrupt",
            Self::RevisionDrift => "revision_drift",
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

fn validate_raw_access(access: &LeaseStoreAccess) -> AuthorityResult<()> {
    raw_access::validate(access)
        .then_some(())
        .ok_or(AuthorityError::CertificateUnauthorized)
}

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

pub fn mutation_signature(
    lease: &LeaseState,
    applied_log_index: u64,
    sequence: u64,
    mutation_sha256: [u8; 32],
    key: &SigningKey,
) -> [u8; 64] {
    key.sign(&mutation_authorization_digest(
        lease,
        applied_log_index,
        sequence,
        mutation_sha256,
    ))
    .to_bytes()
}

fn mutation_authorization_digest(
    lease: &LeaseState,
    applied_log_index: u64,
    sequence: u64,
    mutation_sha256: [u8; 32],
) -> [u8; 32] {
    let payload = MutationAuthorizationPayloadV1 {
        lineage_id: lease.lineage_id.clone(),
        holder_guardian_id: lease.holder_guardian_id.clone(),
        epoch: lease.epoch,
        applied_log_index,
        sequence,
        mutation_sha256: mutation_sha256.to_vec(),
        certificate_sha256: Sha256::digest(&lease.certificate_bytes).to_vec(),
    };
    domain_digest(MUTATION_DOMAIN, &payload.encode_to_vec())
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
    recovery_fences_unix_millis: BTreeMap<Vec<u8>, u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeaseAuthorityRevision {
    applied_log_index: u64,
    state_sha256: [u8; 32],
}

impl LeaseAuthorityRevision {
    pub fn applied_log_index(&self) -> u64 {
        self.applied_log_index
    }

    pub fn state_sha256(&self) -> [u8; 32] {
        self.state_sha256
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RedactedLeaseHealth {
    Active,
    Expired,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedLeaseRow {
    lineage_ref: String,
    holder_node_ref: Option<String>,
    holder_guardian_ref: Option<String>,
    epoch: u64,
    committed_log_index: u64,
    certificate_generation: u64,
    health: RedactedLeaseHealth,
}

impl RedactedLeaseRow {
    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }

    pub fn holder_node_ref(&self) -> Option<&str> {
        self.holder_node_ref.as_deref()
    }

    pub fn holder_guardian_ref(&self) -> Option<&str> {
        self.holder_guardian_ref.as_deref()
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn certificate_generation(&self) -> u64 {
        self.certificate_generation
    }

    pub fn health(&self) -> RedactedLeaseHealth {
        self.health
    }

    pub fn revoked(&self) -> bool {
        self.health == RedactedLeaseHealth::Revoked
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedLeaseSnapshot {
    trust_domain: String,
    revision: LeaseAuthorityRevision,
    rows: Vec<RedactedLeaseRow>,
}

impl RedactedLeaseSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn revision(&self) -> LeaseAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedLeaseRow> {
        self.rows.iter()
    }
}

impl AuthorityLedger {
    pub fn new(access: &LeaseStoreAccess, policy: LeasePolicy) -> AuthorityResult<Self> {
        validate_raw_access(access)?;
        policy.validate()?;
        Ok(Self {
            policy,
            applied_log_index: 0,
            last_raft_term: 0,
            leases: BTreeMap::new(),
            recovery_fences_unix_millis: BTreeMap::new(),
        })
    }

    pub fn applied_log_index(&self) -> u64 {
        self.applied_log_index
    }

    pub fn lease(&self, lineage_id: &[u8]) -> Option<&LeaseState> {
        self.leases.get(lineage_id)
    }

    pub fn authority_revision(&self) -> AuthorityResult<LeaseAuthorityRevision> {
        let snapshot = self.snapshot()?;
        Ok(LeaseAuthorityRevision {
            applied_log_index: self.applied_log_index,
            state_sha256: Sha256::digest(snapshot).into(),
        })
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: LeaseAuthorityRevision,
        membership: &AuthorityMembership,
        now_elapsed_millis: u64,
    ) -> AuthorityResult<RedactedLeaseSnapshot> {
        let revision = self.authority_revision()?;
        if revision != expected_revision {
            return Err(AuthorityError::RevisionDrift);
        }
        if self.leases.len() > self.policy.max_lineages
            || membership.committed_log_index < self.applied_log_index
        {
            return Err(AuthorityError::ResourceExhausted);
        }
        let trust_domain = std::str::from_utf8(&membership.trust_domain_id)
            .map_err(|_| AuthorityError::InvalidMembership)?
            .to_owned();
        let rows = self
            .leases
            .values()
            .map(|lease| {
                let health = if lease.revoked {
                    RedactedLeaseHealth::Revoked
                } else if now_elapsed_millis >= lease.deadline_elapsed_millis {
                    RedactedLeaseHealth::Expired
                } else {
                    RedactedLeaseHealth::Active
                };
                Ok(RedactedLeaseRow {
                    lineage_ref: projection_ref(b"lineage", &lease.lineage_id),
                    holder_node_ref: (!lease.holder_node_id.is_empty())
                        .then(|| projection_ref(b"node", &lease.holder_node_id)),
                    holder_guardian_ref: (!lease.holder_guardian_id.is_empty())
                        .then(|| projection_ref(b"guardian", &lease.holder_guardian_id)),
                    epoch: lease.epoch,
                    committed_log_index: lease.committed_log_index,
                    certificate_generation: lease.certificate_generation,
                    health,
                })
            })
            .collect::<AuthorityResult<Vec<_>>>()?;
        if self.authority_revision()? != revision {
            return Err(AuthorityError::RevisionDrift);
        }
        Ok(RedactedLeaseSnapshot {
            trust_domain,
            revision,
            rows,
        })
    }

    #[cfg(test)]
    pub(crate) fn seed_lease_for_snapshot_test(
        &mut self,
        lease: LeaseState,
    ) -> AuthorityResult<()> {
        if self.leases.len() >= self.policy.max_lineages || lease.lineage_id.is_empty() {
            return Err(AuthorityError::ResourceExhausted);
        }
        self.applied_log_index = self.applied_log_index.max(lease.committed_log_index);
        self.leases.insert(lease.lineage_id.clone(), lease);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn set_counters_for_test(
        &mut self,
        lineage_id: &[u8],
        epoch: u64,
        last_mutation_sequence: u64,
    ) -> AuthorityResult<()> {
        let lease = self
            .leases
            .get_mut(lineage_id)
            .ok_or(AuthorityError::AuthorityRequired)?;
        lease.epoch = epoch;
        lease.last_mutation_sequence = last_mutation_sequence;
        Ok(())
    }

    pub fn apply(
        &mut self,
        access: &LeaseStoreAccess,
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        application: AuthorityApplication<'_>,
    ) -> AuthorityResult<&LeaseState> {
        validate_raw_access(access)?;
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
        let certificate_deadline_unix_millis =
            certificate_deadline_unix_millis(body).ok_or(AuthorityError::ResourceExhausted)?;
        if application.now_unix_nanos >= 1_000_000_000 {
            return Err(AuthorityError::ClockUncertain);
        }
        let now_unix_millis = u64::try_from(application.now_unix_seconds)
            .ok()
            .and_then(|seconds| seconds.checked_mul(1_000))
            .and_then(|millis| {
                millis.checked_add(u64::from(application.now_unix_nanos) / 1_000_000)
            })
            .ok_or(AuthorityError::ClockUncertain)?;
        let issued_unix_nanos = timestamp_unix_nanos(body.issued_unix_seconds, body.issued_nanos)
            .ok_or(AuthorityError::InvalidCertificate)?;
        let now_unix_nanos =
            timestamp_unix_nanos(application.now_unix_seconds, application.now_unix_nanos)
                .ok_or(AuthorityError::ClockUncertain)?;
        if issued_unix_nanos > now_unix_nanos {
            return Err(AuthorityError::InvalidCertificate);
        }
        if now_unix_millis >= certificate_deadline_unix_millis {
            return Err(AuthorityError::LeaseExpired);
        }
        if matches!(
            operation,
            OperationClass::LeaseGrant
                | OperationClass::LeaseRenewal
                | OperationClass::Activate
                | OperationClass::OwnerCommit
        ) {
            verify_activation(
                body,
                application.activation_public_key,
                application.activation_proof,
            )?;
        }

        let current = self.leases.get(&body.lineage_id).cloned();
        match operation {
            OperationClass::LeaseGrant => {
                if current.is_some() {
                    return Err(AuthorityError::AuthorityAlreadyExists);
                }
                if body.epoch != 1 {
                    return Err(if body.epoch < 1 {
                        AuthorityError::StaleEpoch
                    } else {
                        AuthorityError::EpochGap
                    });
                }
            }
            OperationClass::Activate => {
                let previous = current.as_ref().ok_or(AuthorityError::AuthorityRequired)?;
                let previous_operation = lease_operation(previous)?;
                let expected = if previous.revoked && previous_operation == OperationClass::Fence {
                    previous.epoch
                } else {
                    previous
                        .epoch
                        .checked_add(1)
                        .ok_or(AuthorityError::ResourceExhausted)?
                };
                if body.epoch < expected {
                    return Err(AuthorityError::StaleEpoch);
                }
                if body.epoch != expected {
                    return Err(AuthorityError::EpochGap);
                }
                if let Some(safety_deadline) = self
                    .recovery_fences_unix_millis
                    .get(&body.lineage_id)
                    .copied()
                {
                    if now_unix_millis < safety_deadline {
                        return Err(AuthorityError::LeaseExpired);
                    }
                } else {
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
            OperationClass::Revoke => {
                let previous = current.as_ref().ok_or(AuthorityError::HolderMismatch)?;
                if body.epoch != previous.epoch
                    || body.holder_node_id != previous.holder_node_id
                    || body.holder_guardian_id != previous.holder_guardian_id
                    || body.activation_key_sha256
                        != <[u8; 32]>::from(Sha256::digest(previous.activation_public_key))
                            .as_slice()
                    || certificate_deadline_unix_millis != previous.deadline_unix_millis
                {
                    return Err(AuthorityError::HolderMismatch);
                }
            }
            OperationClass::Fence => {
                let previous = current.as_ref().ok_or(AuthorityError::HolderMismatch)?;
                let expected = previous
                    .epoch
                    .checked_add(1)
                    .ok_or(AuthorityError::ResourceExhausted)?;
                if body.epoch < expected {
                    return Err(AuthorityError::StaleEpoch);
                }
                if body.epoch != expected {
                    return Err(AuthorityError::EpochGap);
                }
                if body.holder_node_id != previous.holder_node_id
                    || body.holder_guardian_id != previous.holder_guardian_id
                    || body.activation_key_sha256
                        != <[u8; 32]>::from(Sha256::digest(previous.activation_public_key))
                            .as_slice()
                    || certificate_deadline_unix_millis != previous.deadline_unix_millis
                {
                    return Err(AuthorityError::HolderMismatch);
                }
            }
            OperationClass::OwnerCommit => {
                let previous = current.as_ref().ok_or(AuthorityError::HolderMismatch)?;
                if previous.revoked
                    || previous.epoch != body.epoch
                    || previous.holder_node_id != body.holder_node_id
                    || previous.holder_guardian_id != body.holder_guardian_id
                    || previous.activation_public_key != application.activation_public_key
                    || certificate_deadline_unix_millis != previous.deadline_unix_millis
                {
                    return Err(AuthorityError::HolderMismatch);
                }
                if application.now_elapsed_millis >= previous.deadline_elapsed_millis {
                    return Err(AuthorityError::LeaseExpired);
                }
            }
        }
        if current.is_none() && self.leases.len() >= self.policy.max_lineages {
            return Err(AuthorityError::ResourceExhausted);
        }
        let state = match operation {
            OperationClass::Revoke | OperationClass::Fence => {
                let mut state = current.ok_or(AuthorityError::HolderMismatch)?;
                state.revoked = true;
                state.committed_log_index = body.committed_log_index;
                state.raft_term = body.raft_term;
                state.epoch = body.epoch;
                state.certificate_generation = body.voter_set_generation;
                state.certificate_bytes = certificate_bytes.to_vec();
                if operation == OperationClass::Fence {
                    state.last_mutation_sequence = 0;
                }
                state
            }
            OperationClass::OwnerCommit => {
                let mut state = current.ok_or(AuthorityError::HolderMismatch)?;
                state.committed_log_index = body.committed_log_index;
                state.raft_term = body.raft_term;
                state.certificate_generation = body.voter_set_generation;
                state.certificate_bytes = certificate_bytes.to_vec();
                state
            }
            OperationClass::LeaseGrant
            | OperationClass::Activate
            | OperationClass::LeaseRenewal => {
                let remaining_millis = certificate_deadline_unix_millis
                    .checked_sub(now_unix_millis)
                    .ok_or(AuthorityError::LeaseExpired)?;
                let deadline = application
                    .now_elapsed_millis
                    .checked_add(remaining_millis)
                    .ok_or(AuthorityError::ResourceExhausted)?;
                LeaseState {
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
                    deadline_unix_millis: certificate_deadline_unix_millis,
                    certificate_bytes: certificate_bytes.to_vec(),
                    revoked: false,
                    last_mutation_sequence: current
                        .as_ref()
                        .filter(|previous| previous.epoch == body.epoch)
                        .map_or(0, |previous| previous.last_mutation_sequence),
                }
            }
        };
        let mut prospective_leases = self.leases.clone();
        prospective_leases.insert(body.lineage_id.clone(), state);
        encode_snapshot(
            body.committed_log_index,
            body.raft_term,
            &prospective_leases,
            self.policy.max_snapshot_bytes,
        )?;
        let mut prospective_recovery_fences = self.recovery_fences_unix_millis.clone();
        match operation {
            OperationClass::Fence | OperationClass::Revoke => {
                let safety_deadline = certificate_deadline_unix_millis
                    .checked_add(self.policy.max_clock_uncertainty_millis)
                    .and_then(|value| value.checked_add(self.policy.message_delay_margin_millis))
                    .ok_or(AuthorityError::ResourceExhausted)?;
                prospective_recovery_fences.insert(body.lineage_id.clone(), safety_deadline);
            }
            OperationClass::Activate => {
                prospective_recovery_fences.remove(&body.lineage_id);
            }
            OperationClass::LeaseGrant
            | OperationClass::LeaseRenewal
            | OperationClass::OwnerCommit => {}
        }
        self.applied_log_index = body.committed_log_index;
        self.last_raft_term = body.raft_term;
        self.leases = prospective_leases;
        self.recovery_fences_unix_millis = prospective_recovery_fences;
        self.leases
            .get(&body.lineage_id)
            .ok_or(AuthorityError::SnapshotCorrupt)
    }

    pub fn authorize_mutation(
        &mut self,
        access: &LeaseStoreAccess,
        authorization: MutationAuthorization<'_>,
    ) -> AuthorityResult<()> {
        validate_raw_access(access)?;
        let lease = self
            .leases
            .get(authorization.lineage_id)
            .ok_or(AuthorityError::LeaseExpired)?;
        if lease.revoked {
            return Err(AuthorityError::LeaseRevoked);
        }
        if lease.holder_guardian_id != authorization.holder_guardian_id
            || lease.epoch != authorization.epoch
        {
            return Err(AuthorityError::HolderMismatch);
        }
        if authorization.applied_log_index != self.applied_log_index
            || lease.committed_log_index > self.applied_log_index
        {
            return Err(AuthorityError::StaleAppliedIndex);
        }
        if authorization.now_elapsed_millis >= lease.deadline_elapsed_millis {
            return Err(AuthorityError::LeaseExpired);
        }
        let expected_sequence = lease
            .last_mutation_sequence
            .checked_add(1)
            .ok_or(AuthorityError::ResourceExhausted)?;
        if authorization.sequence != expected_sequence {
            return Err(AuthorityError::Replay);
        }
        let key = VerifyingKey::from_bytes(&lease.activation_public_key)
            .map_err(|_| AuthorityError::ActivationPossession)?;
        let signature = Signature::from_slice(authorization.activation_proof)
            .map_err(|_| AuthorityError::ActivationPossession)?;
        key.verify_strict(
            &mutation_authorization_digest(
                lease,
                authorization.applied_log_index,
                authorization.sequence,
                authorization.mutation_sha256,
            ),
            &signature,
        )
        .map_err(|_| AuthorityError::ActivationPossession)?;
        self.leases
            .get_mut(authorization.lineage_id)
            .ok_or(AuthorityError::LeaseExpired)?
            .last_mutation_sequence = authorization.sequence;
        Ok(())
    }

    pub fn snapshot(&self) -> AuthorityResult<Vec<u8>> {
        encode_snapshot(
            self.applied_log_index,
            self.last_raft_term,
            &self.leases,
            self.policy.max_snapshot_bytes,
        )
    }

    pub fn restore(
        policy: LeasePolicy,
        bytes: &[u8],
        membership: &AuthorityMembership,
        now_unix_seconds: i64,
    ) -> AuthorityResult<Self> {
        policy.validate()?;
        if bytes.is_empty() || bytes.len() > policy.max_snapshot_bytes {
            return Err(AuthorityError::ResourceExhausted);
        }
        let envelope: SnapshotEnvelope =
            serde_json::from_slice(bytes).map_err(|_| AuthorityError::SnapshotCorrupt)?;
        let mut leases = BTreeMap::new();
        let mut recovery_fences_unix_millis = BTreeMap::new();
        let leases_valid = envelope.body.leases.len() <= policy.max_lineages
            && envelope.body.leases.iter().all(|lease| {
                let Some(_body) = validate_snapshot_lease(
                    lease,
                    envelope.body.applied_log_index,
                    envelope.body.last_raft_term,
                    membership,
                    now_unix_seconds,
                    &policy,
                ) else {
                    return false;
                };
                let Some(fence) = restart_safety_deadline_unix_millis(&policy, lease) else {
                    return false;
                };
                recovery_fences_unix_millis.insert(lease.lineage_id.clone(), fence);
                leases
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
            || envelope.body.applied_log_index != membership.committed_log_index
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
            recovery_fences_unix_millis,
        })
    }
}

fn projection_ref(kind: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"adl-projection-ref-v1");
    digest.update((kind.len() as u64).to_be_bytes());
    digest.update(kind);
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
    format!("id_{}", hex::encode(digest.finalize()))
}

fn encode_snapshot(
    applied_log_index: u64,
    last_raft_term: u64,
    leases: &BTreeMap<Vec<u8>, LeaseState>,
    max_snapshot_bytes: usize,
) -> AuthorityResult<Vec<u8>> {
    let body = SnapshotBody {
        schema: AUTHORITY_SNAPSHOT_SCHEMA.to_owned(),
        applied_log_index,
        last_raft_term,
        leases: leases.values().cloned().collect(),
    };
    let body_bytes = serde_jcs::to_vec(&body).map_err(|_| AuthorityError::SnapshotCorrupt)?;
    let envelope = SnapshotEnvelope {
        body,
        digest: Sha256::digest(body_bytes).into(),
    };
    let bytes = serde_jcs::to_vec(&envelope).map_err(|_| AuthorityError::SnapshotCorrupt)?;
    if bytes.len() > max_snapshot_bytes {
        return Err(AuthorityError::ResourceExhausted);
    }
    Ok(bytes)
}

fn validate_snapshot_lease(
    lease: &LeaseState,
    applied_log_index: u64,
    last_raft_term: u64,
    membership: &AuthorityMembership,
    now_unix_seconds: i64,
    policy: &LeasePolicy,
) -> Option<AuthorityCertificateBodyV1> {
    let verified =
        verify_certificate(&lease.certificate_bytes, membership, now_unix_seconds).ok()?;
    let body = verified.body;
    let valid = validate_body(&body).is_ok()
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
        && lease.deadline_unix_millis == certificate_deadline_unix_millis(&body)?
        && body.policy_sha256 == policy.sha256().ok()?.as_slice()
        && body.lineage_id == lease.lineage_id
        && body.holder_node_id == lease.holder_node_id
        && body.holder_guardian_id == lease.holder_guardian_id
        && body.raft_term == lease.raft_term
        && body.committed_log_index == lease.committed_log_index
        && body.epoch == lease.epoch
        && body.voter_set_generation == lease.certificate_generation
        && body.activation_key_sha256
            == <[u8; 32]>::from(Sha256::digest(lease.activation_public_key)).as_slice();
    valid.then_some(body)
}

fn restart_safety_deadline_unix_millis(policy: &LeasePolicy, lease: &LeaseState) -> Option<u64> {
    lease
        .deadline_unix_millis
        .checked_add(policy.max_clock_uncertainty_millis)?
        .checked_add(policy.message_delay_margin_millis)
}

fn lease_operation(lease: &LeaseState) -> AuthorityResult<OperationClass> {
    let certificate = decode_certificate(&lease.certificate_bytes)?;
    let body = certificate.body.ok_or(AuthorityError::InvalidCertificate)?;
    OperationClass::parse(body.operation_class)
}

fn certificate_deadline_unix_millis(body: &AuthorityCertificateBodyV1) -> Option<u64> {
    let issued_seconds = u64::try_from(body.issued_unix_seconds).ok()?;
    issued_seconds
        .checked_mul(1_000)?
        .checked_add(u64::from(body.issued_nanos) / 1_000_000)?
        .checked_add(body.lease_duration_millis)
}

fn timestamp_unix_nanos(seconds: i64, nanos: u32) -> Option<u128> {
    if nanos >= 1_000_000_000 {
        return None;
    }
    u128::try_from(seconds)
        .ok()?
        .checked_mul(1_000_000_000)?
        .checked_add(u128::from(nanos))
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
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16],
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
