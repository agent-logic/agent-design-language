use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::{
    candidate_digest, decide_birthday, BirthdayCandidate, BirthdayDecision, EvidenceKind,
    EvidenceReference, EvidenceVisibility,
};

pub const BIRTH_WITNESS_ATTESTATION_SCHEMA: &str = "adl.birth_witness.attestation.v1";
pub const BIRTH_WITNESS_SET_SCHEMA: &str = "adl.birth_witness.set.v1";
pub const CITIZEN_BIRTH_RECEIPT_SCHEMA: &str = "adl.birth_witness.citizen_receipt.v1";
pub const BIRTH_WITNESS_PACKET_SCHEMA: &str = "adl.birth_witness.packet.v1";

const MAX_ATTESTATIONS: usize = 16;
const MAX_PUBLIC_EVIDENCE: usize = 32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthWitnessRole {
    IdentityContinuity,
    MemoryCapability,
    NegativeCaseGuard,
    HandoffConsumer,
}

impl BirthWitnessRole {
    pub const REQUIRED: [Self; 4] = [
        Self::IdentityContinuity,
        Self::MemoryCapability,
        Self::NegativeCaseGuard,
        Self::HandoffConsumer,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WitnessDecision {
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthEventStatus {
    NotClaimed,
    Claimed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptDisposition {
    WitnessesAccepted,
    WitnessesRejected,
}

#[derive(Clone, Debug)]
pub(crate) struct TrustedBirthWitness {
    pub witness_id: String,
    pub role: BirthWitnessRole,
    pub signing_key_id: String,
    pub verifying_key: VerifyingKey,
}

/// Opaque runtime-established witness authority and freshness policy.
///
/// Neither the trusted witness roster nor its verifying keys can be nominated
/// by an external caller:
///
/// ```compile_fail
/// use adl_runtime_kernel::BirthWitnessPolicy;
/// let _attacker_policy_factory = BirthWitnessPolicy::establish;
/// ```
///
/// The policy is intentionally not serializable and is never copied into the
/// public receipt.
#[derive(Clone, Debug)]
pub struct BirthWitnessPolicy {
    candidate_sha256: String,
    current_generation: u64,
    trusted: BTreeMap<String, TrustedBirthWitness>,
    roster_sha256: String,
    policy_sha256: String,
}

/// Trusted Runtime configuration for one birth-witness authority.
///
/// This is an initialization input, not an attestation or receipt. Runtime
/// owners are responsible for loading it from their trusted configuration
/// boundary; request payloads must never be promoted into this type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeBirthWitnessAuthority {
    pub witness_id: String,
    pub role: BirthWitnessRole,
    pub signing_key_id: String,
    pub verifying_key: [u8; 32],
}

/// Runtime-owned production boundary for governed birth-witness receipts.
///
/// The service retains the opaque policy and never exposes its trusted roster.
/// Its only execution path builds and revalidates the packet before emitting
/// the canonical receipt bytes.
#[derive(Clone, Debug)]
pub struct RuntimeBirthWitnessService {
    policy: BirthWitnessPolicy,
}

impl BirthWitnessPolicy {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn establish(
        authority_context: impl Into<String>,
        candidate_sha256: impl Into<String>,
        current_generation: u64,
        trusted: Vec<TrustedBirthWitness>,
    ) -> Result<Self, BirthWitnessError> {
        let authority_context = authority_context.into();
        let candidate_sha256 = candidate_sha256.into();
        if !safe_identifier(&authority_context)
            || !is_sha256(&candidate_sha256)
            || current_generation == 0
            || trusted.len() != BirthWitnessRole::REQUIRED.len()
        {
            return Err(BirthWitnessError::InvalidPolicy);
        }

        let mut by_id = BTreeMap::new();
        let mut roles = BTreeSet::new();
        let mut key_ids = BTreeSet::new();
        for entry in trusted {
            if !safe_identifier(&entry.witness_id)
                || !safe_identifier(&entry.signing_key_id)
                || !roles.insert(entry.role)
                || !key_ids.insert(entry.signing_key_id.to_ascii_lowercase())
                || by_id
                    .insert(entry.witness_id.to_ascii_lowercase(), entry)
                    .is_some()
            {
                return Err(BirthWitnessError::InvalidPolicy);
            }
        }
        if roles != BirthWitnessRole::REQUIRED.into_iter().collect() {
            return Err(BirthWitnessError::InvalidPolicy);
        }

        let roster_sha256 = trusted_roster_digest(&by_id)?;
        let policy_sha256 = policy_digest(
            &authority_context,
            &candidate_sha256,
            current_generation,
            &roster_sha256,
        )?;
        Ok(Self {
            candidate_sha256,
            current_generation,
            trusted: by_id,
            roster_sha256,
            policy_sha256,
        })
    }

    pub fn roster_sha256(&self) -> &str {
        &self.roster_sha256
    }

    pub fn policy_sha256(&self) -> &str {
        &self.policy_sha256
    }
}

impl RuntimeBirthWitnessService {
    pub(crate) fn provision(
        authority_context: impl Into<String>,
        candidate_sha256: impl Into<String>,
        current_generation: u64,
        authorities: Vec<RuntimeBirthWitnessAuthority>,
    ) -> Result<Self, BirthWitnessError> {
        let trusted = authorities
            .into_iter()
            .map(|authority| {
                let verifying_key = VerifyingKey::from_bytes(&authority.verifying_key)
                    .map_err(|_| BirthWitnessError::InvalidPolicy)?;
                Ok(TrustedBirthWitness {
                    witness_id: authority.witness_id,
                    role: authority.role,
                    signing_key_id: authority.signing_key_id,
                    verifying_key,
                })
            })
            .collect::<Result<Vec<_>, BirthWitnessError>>()?;
        let policy = BirthWitnessPolicy::establish(
            authority_context,
            candidate_sha256,
            current_generation,
            trusted,
        )?;
        Ok(Self { policy })
    }

    pub fn roster_sha256(&self) -> &str {
        self.policy.roster_sha256()
    }

    pub fn policy_sha256(&self) -> &str {
        self.policy.policy_sha256()
    }

    pub fn build_validate_and_emit<F, C>(
        &self,
        candidate: &BirthdayCandidate,
        decision: &BirthdayDecision,
        attestations: &[BirthWitnessAttestation],
        prepare_receipt: F,
    ) -> Result<BirthWitnessPacket, BirthWitnessError>
    where
        F: FnOnce(&[u8]) -> Result<C, ()>,
        C: FnOnce(),
    {
        let packet = build_birth_witness_packet(candidate, decision, &self.policy, attestations)?;
        validate_birth_witness_packet(&packet, candidate, decision, &self.policy, attestations)?;
        let receipt =
            serde_jcs::to_vec(&packet.receipt).map_err(|_| BirthWitnessError::Encoding)?;
        let commit = prepare_receipt(&receipt).map_err(|()| BirthWitnessError::ReceiptEmission)?;
        commit();
        Ok(packet)
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn birth_witness_roster_digest(
    trusted: &[TrustedBirthWitness],
) -> Result<String, BirthWitnessError> {
    let mut by_id = BTreeMap::new();
    for entry in trusted {
        if by_id
            .insert(entry.witness_id.to_ascii_lowercase(), entry.clone())
            .is_some()
        {
            return Err(BirthWitnessError::InvalidPolicy);
        }
    }
    trusted_roster_digest(&by_id)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthWitnessAttestation {
    pub schema: String,
    pub witness_id: String,
    pub role: BirthWitnessRole,
    pub candidate_sha256: String,
    pub evidence_set_sha256: String,
    pub observed_generation: u64,
    pub decision: WitnessDecision,
    pub signing_key_id: String,
    pub signature: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicWitnessSummary {
    pub witness_id: String,
    pub role: BirthWitnessRole,
    pub decision: WitnessDecision,
    pub observed_generation: u64,
    pub attestation_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedBirthWitnessSet {
    pub schema: String,
    pub candidate_sha256: String,
    pub evidence_set_sha256: String,
    pub roster_sha256: String,
    pub policy_sha256: String,
    pub witnesses: Vec<PublicWitnessSummary>,
    pub witness_set_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitizenBirthReceipt {
    pub schema: String,
    pub candidate_id: String,
    pub candidate_sha256: String,
    pub witness_set_sha256: String,
    pub disposition: ReceiptDisposition,
    pub birth_event_status: BirthEventStatus,
    pub public_evidence: Vec<EvidenceReference>,
    pub caveats: Vec<String>,
    pub receipt_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthWitnessPacket {
    pub schema: String,
    pub witness_set: ValidatedBirthWitnessSet,
    pub receipt: CitizenBirthReceipt,
    pub packet_sha256: String,
}

/// Opaque proof that a packet was emitted by the Runtime-provisioned witness
/// owner after signature, roster, freshness, policy, and receipt validation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedBirthWitnessBinding {
    pub(crate) packet: BirthWitnessPacket,
    pub(crate) observed_generation: u64,
}

impl VerifiedBirthWitnessBinding {
    pub fn packet(&self) -> &BirthWitnessPacket {
        &self.packet
    }

    pub fn observed_generation(&self) -> u64 {
        self.observed_generation
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum BirthWitnessError {
    #[error("invalid provisioned witness policy")]
    InvalidPolicy,
    #[error("birthday candidate is not canonically accepted")]
    CandidateRejected,
    #[error("birthday decision does not match the candidate")]
    DecisionMismatch,
    #[error("candidate digest is not bound to the provisioned policy")]
    CandidateDigestMismatch,
    #[error("candidate witness-roster reference is not policy-bound")]
    RosterDigestMismatch,
    #[error("witness set is missing a required role")]
    MissingRequiredRole,
    #[error("witness identity, role, or signing key is duplicated")]
    DuplicateWitness,
    #[error("witness is not authorized by the provisioned policy")]
    UnauthorizedWitness,
    #[error("witness attestation shape is invalid")]
    InvalidAttestation,
    #[error("witness attestation is stale or generation-mismatched")]
    StaleWitness,
    #[error("witness attestation is bound to different candidate or evidence")]
    AttestationBindingMismatch,
    #[error("witness signature is invalid")]
    InvalidSignature,
    #[error("public receipt evidence is unsafe")]
    UnsafePublicEvidence,
    #[error("packet encoding failed")]
    Encoding,
    #[error("packet does not reconstruct canonically")]
    PacketMismatch,
    #[error("validated receipt emission could not be prepared")]
    ReceiptEmission,
}

#[derive(Serialize)]
struct UnsignedAttestation<'a> {
    schema: &'a str,
    witness_id: &'a str,
    role: BirthWitnessRole,
    candidate_sha256: &'a str,
    evidence_set_sha256: &'a str,
    observed_generation: u64,
    decision: WitnessDecision,
    signing_key_id: &'a str,
}

pub fn witness_signing_bytes(
    attestation: &BirthWitnessAttestation,
) -> Result<Vec<u8>, BirthWitnessError> {
    serde_jcs::to_vec(&UnsignedAttestation {
        schema: &attestation.schema,
        witness_id: &attestation.witness_id,
        role: attestation.role,
        candidate_sha256: &attestation.candidate_sha256,
        evidence_set_sha256: &attestation.evidence_set_sha256,
        observed_generation: attestation.observed_generation,
        decision: attestation.decision,
        signing_key_id: &attestation.signing_key_id,
    })
    .map_err(|_| BirthWitnessError::Encoding)
}

pub fn reviewed_evidence_set_digest(
    candidate: &BirthdayCandidate,
) -> Result<String, BirthWitnessError> {
    let mut evidence = candidate
        .evidence
        .iter()
        .filter(|entry| !matches!(entry.kind, EvidenceKind::WitnessSet | EvidenceKind::Receipt))
        .cloned()
        .collect::<Vec<_>>();
    evidence.sort_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.sha256.cmp(&right.sha256))
            .then_with(|| left.visibility.cmp(&right.visibility))
    });
    digest_jcs(&evidence)
}

pub fn build_birth_witness_packet(
    candidate: &BirthdayCandidate,
    decision: &BirthdayDecision,
    policy: &BirthWitnessPolicy,
    attestations: &[BirthWitnessAttestation],
) -> Result<BirthWitnessPacket, BirthWitnessError> {
    let canonical_decision = decide_birthday(candidate);
    if decision != &canonical_decision {
        return Err(BirthWitnessError::DecisionMismatch);
    }
    if !decision.accepted || !decision.rejections.is_empty() {
        return Err(BirthWitnessError::CandidateRejected);
    }
    let candidate_sha256 = candidate_digest(candidate).map_err(|_| BirthWitnessError::Encoding)?;
    if candidate.packet_sha256 != candidate_sha256 || policy.candidate_sha256 != candidate_sha256 {
        return Err(BirthWitnessError::CandidateDigestMismatch);
    }
    let roster_ref = candidate
        .evidence
        .iter()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .ok_or(BirthWitnessError::RosterDigestMismatch)?;
    if roster_ref.sha256 != policy.roster_sha256 {
        return Err(BirthWitnessError::RosterDigestMismatch);
    }
    if attestations.len() != BirthWitnessRole::REQUIRED.len()
        || attestations.len() > MAX_ATTESTATIONS
    {
        return Err(BirthWitnessError::MissingRequiredRole);
    }

    let evidence_set_sha256 = reviewed_evidence_set_digest(candidate)?;
    let mut ids = BTreeSet::new();
    let mut roles = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut summaries = Vec::with_capacity(attestations.len());
    for attestation in attestations {
        validate_attestation_shape(attestation)?;
        if !ids.insert(attestation.witness_id.to_ascii_lowercase())
            || !roles.insert(attestation.role)
            || !keys.insert(attestation.signing_key_id.to_ascii_lowercase())
        {
            return Err(BirthWitnessError::DuplicateWitness);
        }
        let trusted = policy
            .trusted
            .get(&attestation.witness_id.to_ascii_lowercase())
            .ok_or(BirthWitnessError::UnauthorizedWitness)?;
        if trusted.witness_id != attestation.witness_id
            || trusted.role != attestation.role
            || trusted.signing_key_id != attestation.signing_key_id
        {
            return Err(BirthWitnessError::UnauthorizedWitness);
        }
        if attestation.observed_generation != policy.current_generation {
            return Err(BirthWitnessError::StaleWitness);
        }
        if attestation.candidate_sha256 != candidate_sha256
            || attestation.evidence_set_sha256 != evidence_set_sha256
        {
            return Err(BirthWitnessError::AttestationBindingMismatch);
        }
        let signature_bytes =
            hex::decode(&attestation.signature).map_err(|_| BirthWitnessError::InvalidSignature)?;
        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|_| BirthWitnessError::InvalidSignature)?;
        trusted
            .verifying_key
            .verify(&witness_signing_bytes(attestation)?, &signature)
            .map_err(|_| BirthWitnessError::InvalidSignature)?;
        summaries.push(PublicWitnessSummary {
            witness_id: attestation.witness_id.clone(),
            role: attestation.role,
            decision: attestation.decision,
            observed_generation: attestation.observed_generation,
            attestation_sha256: digest_jcs(attestation)?,
        });
    }
    if roles != BirthWitnessRole::REQUIRED.into_iter().collect() {
        return Err(BirthWitnessError::MissingRequiredRole);
    }
    summaries.sort_by(|left, right| {
        left.role
            .cmp(&right.role)
            .then_with(|| left.witness_id.cmp(&right.witness_id))
    });

    let mut witness_set = ValidatedBirthWitnessSet {
        schema: BIRTH_WITNESS_SET_SCHEMA.to_owned(),
        candidate_sha256: candidate_sha256.clone(),
        evidence_set_sha256,
        roster_sha256: policy.roster_sha256.clone(),
        policy_sha256: policy.policy_sha256.clone(),
        witnesses: summaries,
        witness_set_sha256: String::new(),
    };
    witness_set.witness_set_sha256 = witness_set_digest(&witness_set)?;

    let disposition = if witness_set
        .witnesses
        .iter()
        .all(|witness| witness.decision == WitnessDecision::Accept)
    {
        ReceiptDisposition::WitnessesAccepted
    } else {
        ReceiptDisposition::WitnessesRejected
    };
    validate_public_evidence(&candidate.evidence)?;
    let mut public_evidence = candidate
        .evidence
        .iter()
        .map(redacted_public_evidence)
        .collect::<Vec<_>>();
    public_evidence.sort_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then_with(|| left.path.cmp(&right.path))
    });
    validate_public_evidence(&public_evidence)?;
    let mut receipt = CitizenBirthReceipt {
        schema: CITIZEN_BIRTH_RECEIPT_SCHEMA.to_owned(),
        candidate_id: candidate.candidate_id.clone(),
        candidate_sha256,
        witness_set_sha256: witness_set.witness_set_sha256.clone(),
        disposition,
        birth_event_status: BirthEventStatus::NotClaimed,
        public_evidence,
        caveats: canonical_caveats(),
        receipt_sha256: String::new(),
    };
    receipt.receipt_sha256 = receipt_digest(&receipt)?;

    let mut packet = BirthWitnessPacket {
        schema: BIRTH_WITNESS_PACKET_SCHEMA.to_owned(),
        witness_set,
        receipt,
        packet_sha256: String::new(),
    };
    packet.packet_sha256 = packet_digest(&packet)?;
    Ok(packet)
}

pub fn validate_birth_witness_packet(
    packet: &BirthWitnessPacket,
    candidate: &BirthdayCandidate,
    decision: &BirthdayDecision,
    policy: &BirthWitnessPolicy,
    attestations: &[BirthWitnessAttestation],
) -> Result<(), BirthWitnessError> {
    let expected = build_birth_witness_packet(candidate, decision, policy, attestations)?;
    if packet != &expected
        || packet.schema != BIRTH_WITNESS_PACKET_SCHEMA
        || packet.packet_sha256 != packet_digest(packet)?
        || packet.witness_set.witness_set_sha256 != witness_set_digest(&packet.witness_set)?
        || packet.receipt.receipt_sha256 != receipt_digest(&packet.receipt)?
        || packet.receipt.birth_event_status != BirthEventStatus::NotClaimed
        || packet.receipt.caveats != canonical_caveats()
    {
        return Err(BirthWitnessError::PacketMismatch);
    }
    Ok(())
}

fn validate_attestation_shape(
    attestation: &BirthWitnessAttestation,
) -> Result<(), BirthWitnessError> {
    if attestation.schema != BIRTH_WITNESS_ATTESTATION_SCHEMA
        || !safe_identifier(&attestation.witness_id)
        || !safe_identifier(&attestation.signing_key_id)
        || !is_sha256(&attestation.candidate_sha256)
        || !is_sha256(&attestation.evidence_set_sha256)
        || attestation.observed_generation == 0
        || attestation.signature.len() != 128
        || !attestation
            .signature
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(BirthWitnessError::InvalidAttestation);
    }
    Ok(())
}

fn validate_public_evidence(evidence: &[EvidenceReference]) -> Result<(), BirthWitnessError> {
    if evidence.len() > MAX_PUBLIC_EVIDENCE
        || evidence.iter().any(|entry| {
            entry.visibility != EvidenceVisibility::ReviewerVisible
                || !safe_repository_path(&entry.path)
                || unsafe_text(&entry.path)
                || !is_sha256(&entry.sha256)
        })
    {
        return Err(BirthWitnessError::UnsafePublicEvidence);
    }
    Ok(())
}

fn redacted_public_evidence(entry: &EvidenceReference) -> EvidenceReference {
    EvidenceReference {
        kind: entry.kind,
        path: format!(
            "evidence-ref/{}-{}.json",
            evidence_kind_label(entry.kind),
            entry.sha256
        ),
        sha256: entry.sha256.clone(),
        visibility: EvidenceVisibility::ReviewerVisible,
    }
}

fn evidence_kind_label(kind: EvidenceKind) -> &'static str {
    match kind {
        EvidenceKind::StableName => "stable-name",
        EvidenceKind::IdentityRoot => "identity-root",
        EvidenceKind::ContinuityHead => "continuity-head",
        EvidenceKind::MemoryGrounding => "memory-grounding",
        EvidenceKind::CapabilityEnvelope => "capability-envelope",
        EvidenceKind::CognitiveProfile => "cognitive-profile",
        EvidenceKind::InheritedMoralContext => "inherited-moral-context",
        EvidenceKind::WitnessSet => "witness-set",
        EvidenceKind::Receipt => "receipt",
        EvidenceKind::ReviewerValidation => "reviewer-validation",
    }
}

fn canonical_caveats() -> Vec<String> {
    [
        "receipt_is_review_surface_not_birth_authority",
        "birth_event_status_not_claimed",
        "no_legal_personhood_or_citizenship_claim",
        "no_governance_or_public_launch_claim",
        "no_raw_private_state_disclosed",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[derive(Serialize)]
struct RosterEntry<'a> {
    witness_id: &'a str,
    role: BirthWitnessRole,
    signing_key_id: &'a str,
    verifying_key: String,
}

fn trusted_roster_digest(
    trusted: &BTreeMap<String, TrustedBirthWitness>,
) -> Result<String, BirthWitnessError> {
    let entries = trusted
        .values()
        .map(|entry| RosterEntry {
            witness_id: &entry.witness_id,
            role: entry.role,
            signing_key_id: &entry.signing_key_id,
            verifying_key: hex::encode(entry.verifying_key.as_bytes()),
        })
        .collect::<Vec<_>>();
    digest_jcs(&entries)
}

#[derive(Serialize)]
struct PolicyMaterial<'a> {
    schema: &'static str,
    authority_context: &'a str,
    candidate_sha256: &'a str,
    current_generation: u64,
    roster_sha256: &'a str,
}

fn policy_digest(
    authority_context: &str,
    candidate_sha256: &str,
    current_generation: u64,
    roster_sha256: &str,
) -> Result<String, BirthWitnessError> {
    digest_jcs(&PolicyMaterial {
        schema: "adl.birth_witness.policy.v1",
        authority_context,
        candidate_sha256,
        current_generation,
        roster_sha256,
    })
}

fn witness_set_digest(set: &ValidatedBirthWitnessSet) -> Result<String, BirthWitnessError> {
    #[derive(Serialize)]
    struct Material<'a> {
        schema: &'a str,
        candidate_sha256: &'a str,
        evidence_set_sha256: &'a str,
        roster_sha256: &'a str,
        policy_sha256: &'a str,
        witnesses: &'a [PublicWitnessSummary],
    }
    digest_jcs(&Material {
        schema: &set.schema,
        candidate_sha256: &set.candidate_sha256,
        evidence_set_sha256: &set.evidence_set_sha256,
        roster_sha256: &set.roster_sha256,
        policy_sha256: &set.policy_sha256,
        witnesses: &set.witnesses,
    })
}

fn receipt_digest(receipt: &CitizenBirthReceipt) -> Result<String, BirthWitnessError> {
    #[derive(Serialize)]
    struct Material<'a> {
        schema: &'a str,
        candidate_id: &'a str,
        candidate_sha256: &'a str,
        witness_set_sha256: &'a str,
        disposition: ReceiptDisposition,
        birth_event_status: BirthEventStatus,
        public_evidence: &'a [EvidenceReference],
        caveats: &'a [String],
    }
    digest_jcs(&Material {
        schema: &receipt.schema,
        candidate_id: &receipt.candidate_id,
        candidate_sha256: &receipt.candidate_sha256,
        witness_set_sha256: &receipt.witness_set_sha256,
        disposition: receipt.disposition,
        birth_event_status: receipt.birth_event_status,
        public_evidence: &receipt.public_evidence,
        caveats: &receipt.caveats,
    })
}

fn packet_digest(packet: &BirthWitnessPacket) -> Result<String, BirthWitnessError> {
    #[derive(Serialize)]
    struct Material<'a> {
        schema: &'a str,
        witness_set: &'a ValidatedBirthWitnessSet,
        receipt: &'a CitizenBirthReceipt,
    }
    digest_jcs(&Material {
        schema: &packet.schema,
        witness_set: &packet.witness_set,
        receipt: &packet.receipt,
    })
}

fn digest_jcs<T: Serialize>(value: &T) -> Result<String, BirthWitnessError> {
    serde_jcs::to_vec(value)
        .map(|bytes| hex::encode(Sha256::digest(bytes)))
        .map_err(|_| BirthWitnessError::Encoding)
}

fn safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.trim() == value
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
        && !unsafe_text(value)
}

fn safe_repository_path(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 512
        && !value.starts_with('/')
        && !value.starts_with('\\')
        && !value.contains('\\')
        && value.as_bytes().get(1).copied() != Some(b':')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
        && value
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

fn unsafe_text(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "gho_",
        "github_pat_",
        "bearer ",
        "api_key",
        "api-key",
        "sk-",
        "/users/",
        "/home/",
        "/private/",
        "private_state",
        "raw_state",
        "sealed_payload",
        "password",
        "passwd",
        "secret",
        "token",
        "credential",
        "access_key",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
        || lower.starts_with("users/")
        || lower.starts_with("home/")
        || lower.starts_with("private/")
}

#[cfg(test)]
#[path = "../tests/fixtures/birth_witness/authority_tests.rs"]
mod authority_tests;

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
