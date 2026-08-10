use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    project_private_state, verify_binding, IdentityBinding, IdentityMemoryError, MemoryCheckpoint,
    MemoryLedger, PrivateStateError, PrivateStateLineage, PrivateStateRecord, ProjectionRequest,
    SanctuaryPolicy,
};

pub const BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA: &str = "adl.birthday.identity_candidate.v2";
pub const BIRTHDAY_IDENTITY_RECORD_SCHEMA: &str = "adl.birthday.identity_record.v2";
pub const VERIFIED_PROJECTION_RECEIPT_SCHEMA: &str = "adl.birthday.verified_projection_receipt.v1";
const IDENTITY_ROOT_MATERIAL_SCHEMA: &str = "adl.birthday.identity_root_material.v2";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityBasis {
    OriginEvidence,
    DisplayName,
    BootAdmission,
    WakeState,
    Snapshot,
    CopiedState,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityReference {
    pub id: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AliasBinding {
    pub name: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginBinding {
    pub event_id: String,
    pub provenance_id: String,
    pub reference: IdentityReference,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityBinding {
    pub identity_root: String,
    pub head_sha256: String,
    pub reference: IdentityReference,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthdayIdentityCandidate {
    pub schema: String,
    pub basis: IdentityBasis,
    pub stable_name: String,
    pub identity_root: String,
    #[serde(default)]
    pub aliases: Vec<AliasBinding>,
    pub origin: OriginBinding,
    pub continuity: ContinuityBinding,
    pub provenance: Vec<IdentityReference>,
    pub witnesses: Vec<IdentityReference>,
    pub governed_projection: IdentityReference,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedProjectionReceipt {
    pub schema: String,
    pub subject_id: String,
    pub lineage_id: String,
    pub generation: u64,
    pub signing_key_id: String,
    pub accepted_record_hash: String,
    pub projection_sha256: String,
    pub visible_fields: BTreeMap<String, String>,
    pub redacted_fields: Vec<String>,
}

/// Opaque proof that the canonical identity-memory and private-state authorities
/// accepted the exact lineage and governed projection used for construction.
/// Callers cannot deserialize or construct this token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedBirthdayEvidence {
    citizen_id: String,
    runtime_id: String,
    continuity_id: String,
    binding_sha256: String,
    checkpoint_sha256: String,
    checkpoint_head: String,
    checkpoint_facts: BTreeMap<String, String>,
    private_record_sha256: String,
    projection_receipt: VerifiedProjectionReceipt,
}

impl VerifiedBirthdayEvidence {
    pub fn binding_sha256(&self) -> &str {
        &self.binding_sha256
    }

    pub fn checkpoint_sha256(&self) -> &str {
        &self.checkpoint_sha256
    }

    pub fn checkpoint_head(&self) -> &str {
        &self.checkpoint_head
    }

    pub fn private_record_sha256(&self) -> &str {
        &self.private_record_sha256
    }

    pub fn projection_receipt(&self) -> &VerifiedProjectionReceipt {
        &self.projection_receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BirthdayEvidenceRequirements {
    pub identity_signing_key_id: String,
    pub private_state_signing_key_id: String,
    pub identity_generation: u64,
    pub continuity_generation: u64,
    pub projection_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BirthdayEvidenceError {
    IdentitySignature,
    IdentityUnauthorized,
    IdentityContinuity,
    IdentitySignerMismatch,
    IdentityGenerationMismatch,
    ContinuityGenerationMismatch,
    PrivateSignature,
    PrivateUnauthorized,
    PrivateLineage,
    PrivateProjection,
    PrivateSignerMismatch,
    ProjectionGenerationMismatch,
    AuthoritySubjectMismatch,
    RawPrivateProjection,
    EncodingFailure,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum IdentityRejection {
    UnsupportedSchema,
    UnsupportedBasis { basis: IdentityBasis },
    InvalidStableName,
    EmptyIdentityRoot,
    MalformedIdentityRoot,
    IdentityRootMismatch,
    InvalidAlias { name: String },
    AliasCollision { name: String },
    InvalidOriginEvent,
    OriginAuthorityMismatch,
    MissingProvenance { id: String },
    UnverifiedProvenance { id: String },
    DuplicateProvenance { id: String },
    DuplicateWitness { id: String },
    MissingWitnesses,
    MissingGovernedWitness,
    InvalidReference { id: String },
    UnsafeReferencePath { id: String },
    MalformedReferenceDigest { id: String },
    ContinuityIdentitySubstitution,
    ContinuityHeadMismatch,
    ContinuityAuthorityMismatch,
    ProjectionAuthorityMismatch,
    InvalidAliasAuthority { id: String },
    RecordDigestMismatch,
    NonCanonicalRecord,
    EncodingFailure,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthdayIdentityRecord {
    pub schema: String,
    pub stable_name: String,
    pub identity_root: String,
    pub aliases: Vec<AliasBinding>,
    pub origin: OriginBinding,
    pub continuity: ContinuityBinding,
    pub provenance: Vec<IdentityReference>,
    pub witnesses: Vec<IdentityReference>,
    pub governed_projection: IdentityReference,
    pub projection_receipt: VerifiedProjectionReceipt,
    pub record_sha256: String,
}

#[derive(Serialize)]
struct RootMaterial<'a> {
    schema: &'static str,
    stable_name: &'a str,
    origin_event_id: &'a str,
    citizen_id: &'a str,
    runtime_id: &'a str,
    continuity_id: &'a str,
    binding_sha256: &'a str,
    checkpoint_head: &'a str,
}

#[allow(clippy::too_many_arguments)]
pub fn verify_birthday_evidence(
    binding: &IdentityBinding,
    checkpoint: &MemoryCheckpoint,
    identity_keys: &BTreeMap<String, VerifyingKey>,
    private_record: &PrivateStateRecord,
    private_lineage: &mut PrivateStateLineage,
    private_keys: &BTreeMap<String, VerifyingKey>,
    available_projection: &BTreeMap<String, String>,
    sanctuary_policy: &SanctuaryPolicy,
    projection_request: &ProjectionRequest,
    requirements: &BirthdayEvidenceRequirements,
) -> Result<VerifiedBirthdayEvidence, BirthdayEvidenceError> {
    verify_binding(binding, identity_keys).map_err(map_identity_error)?;
    MemoryLedger::restore(checkpoint, binding, identity_keys).map_err(map_identity_error)?;
    if binding.signing_key_id != requirements.identity_signing_key_id
        || checkpoint.signing_key_id != requirements.identity_signing_key_id
    {
        return Err(BirthdayEvidenceError::IdentitySignerMismatch);
    }
    if binding.issued_at_tick != requirements.identity_generation {
        return Err(BirthdayEvidenceError::IdentityGenerationMismatch);
    }
    if checkpoint.accepted_through != requirements.continuity_generation {
        return Err(BirthdayEvidenceError::ContinuityGenerationMismatch);
    }

    let accepted_record_hash = private_lineage
        .append(private_record, private_keys)
        .map_err(map_private_error)?;
    if private_record.signing_key_id != requirements.private_state_signing_key_id {
        return Err(BirthdayEvidenceError::PrivateSignerMismatch);
    }
    if private_record.sequence != requirements.projection_generation {
        return Err(BirthdayEvidenceError::ProjectionGenerationMismatch);
    }
    if private_record.subject_id != binding.citizen_id
        || private_record.lineage_id != binding.continuity_id
    {
        return Err(BirthdayEvidenceError::AuthoritySubjectMismatch);
    }
    let projection = project_private_state(
        private_lineage,
        private_keys,
        private_record,
        available_projection,
        sanctuary_policy,
        projection_request,
    )
    .map_err(map_private_error)?;
    if projection.subject_id != binding.citizen_id
        || projection.lineage_id != binding.continuity_id
        || projection.sequence != requirements.projection_generation
        || projection.record_hash != accepted_record_hash
    {
        return Err(BirthdayEvidenceError::AuthoritySubjectMismatch);
    }
    if projection
        .visible_fields
        .keys()
        .any(|field| is_raw_private_field(field))
    {
        return Err(BirthdayEvidenceError::RawPrivateProjection);
    }

    let projection_sha256 = digest_jcs(&projection)?;
    Ok(VerifiedBirthdayEvidence {
        citizen_id: binding.citizen_id.clone(),
        runtime_id: binding.runtime_id.clone(),
        continuity_id: binding.continuity_id.clone(),
        binding_sha256: digest_jcs(binding)?,
        checkpoint_sha256: digest_jcs(checkpoint)?,
        checkpoint_head: checkpoint.head_hash.clone(),
        checkpoint_facts: checkpoint.facts.clone(),
        private_record_sha256: digest_jcs(private_record)?,
        projection_receipt: VerifiedProjectionReceipt {
            schema: VERIFIED_PROJECTION_RECEIPT_SCHEMA.to_owned(),
            subject_id: projection.subject_id,
            lineage_id: projection.lineage_id,
            generation: projection.sequence,
            signing_key_id: private_record.signing_key_id.clone(),
            accepted_record_hash,
            projection_sha256,
            visible_fields: projection.visible_fields,
            redacted_fields: projection.redacted_fields,
        },
    })
}

pub fn derive_identity_root(
    candidate: &BirthdayIdentityCandidate,
    evidence: &VerifiedBirthdayEvidence,
) -> Result<String, serde_json::Error> {
    let material = RootMaterial {
        schema: IDENTITY_ROOT_MATERIAL_SCHEMA,
        stable_name: &candidate.stable_name,
        origin_event_id: &candidate.origin.event_id,
        citizen_id: &evidence.citizen_id,
        runtime_id: &evidence.runtime_id,
        continuity_id: &evidence.continuity_id,
        binding_sha256: &evidence.binding_sha256,
        checkpoint_head: &evidence.checkpoint_head,
    };
    Ok(sha256(&serde_jcs::to_vec(&material)?))
}

pub fn build_birthday_identity(
    candidate: &BirthdayIdentityCandidate,
    evidence: &VerifiedBirthdayEvidence,
) -> Result<BirthdayIdentityRecord, Vec<IdentityRejection>> {
    let mut rejections = BTreeSet::new();
    if candidate.schema != BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA {
        rejections.insert(IdentityRejection::UnsupportedSchema);
    }
    if candidate.basis != IdentityBasis::OriginEvidence {
        rejections.insert(IdentityRejection::UnsupportedBasis {
            basis: candidate.basis,
        });
    }
    if !valid_name(&candidate.stable_name) {
        rejections.insert(IdentityRejection::InvalidStableName);
    }
    if candidate.identity_root.is_empty() {
        rejections.insert(IdentityRejection::EmptyIdentityRoot);
    } else if !is_sha256(&candidate.identity_root) {
        rejections.insert(IdentityRejection::MalformedIdentityRoot);
    }
    if derive_identity_root(candidate, evidence).ok().as_deref()
        != Some(candidate.identity_root.as_str())
    {
        rejections.insert(IdentityRejection::IdentityRootMismatch);
    }
    if !safe_id(&candidate.origin.event_id) {
        rejections.insert(IdentityRejection::InvalidOriginEvent);
    }
    if evidence.checkpoint_facts.get("birthday.origin_event") != Some(&candidate.origin.event_id)
        || evidence.checkpoint_facts.get("birthday.stable_name") != Some(&candidate.stable_name)
        || candidate.origin.reference.sha256 != evidence.binding_sha256
    {
        rejections.insert(IdentityRejection::OriginAuthorityMismatch);
    }

    let mut provenance_by_id = BTreeMap::new();
    for reference in &candidate.provenance {
        if provenance_by_id
            .insert(reference.id.clone(), reference)
            .is_some()
        {
            rejections.insert(IdentityRejection::DuplicateProvenance {
                id: reference.id.clone(),
            });
        }
        validate_reference(reference, &mut rejections);
    }
    match provenance_by_id.get(&candidate.origin.provenance_id) {
        None => {
            rejections.insert(IdentityRejection::MissingProvenance {
                id: candidate.origin.provenance_id.clone(),
            });
        }
        Some(provenance) if provenance.sha256 != evidence.binding_sha256 => {
            rejections.insert(IdentityRejection::UnverifiedProvenance {
                id: provenance.id.clone(),
            });
        }
        Some(_) => {}
    }
    validate_reference(&candidate.origin.reference, &mut rejections);

    let stable_key = name_key(&candidate.stable_name);
    let mut alias_keys = BTreeSet::new();
    for alias in &candidate.aliases {
        let key = name_key(&alias.name);
        if !valid_name(&alias.name) {
            rejections.insert(IdentityRejection::InvalidAlias {
                name: alias.name.clone(),
            });
        }
        if key == stable_key || !alias_keys.insert(key) {
            rejections.insert(IdentityRejection::AliasCollision {
                name: alias.name.clone(),
            });
        }
        match provenance_by_id.get(&alias.provenance_id) {
            None => {
                rejections.insert(IdentityRejection::MissingProvenance {
                    id: alias.provenance_id.clone(),
                });
            }
            Some(reference)
                if reference.sha256 != evidence.checkpoint_sha256
                    || evidence
                        .checkpoint_facts
                        .get(&format!("birthday.alias.{}", alias.provenance_id))
                        != Some(&alias.name) =>
            {
                rejections.insert(IdentityRejection::InvalidAliasAuthority {
                    id: alias.provenance_id.clone(),
                });
            }
            Some(_) => {}
        }
    }

    if candidate.continuity.identity_root != candidate.identity_root {
        rejections.insert(IdentityRejection::ContinuityIdentitySubstitution);
    }
    if candidate.continuity.head_sha256 != evidence.checkpoint_head {
        rejections.insert(IdentityRejection::ContinuityHeadMismatch);
    }
    if candidate.continuity.reference.sha256 != evidence.checkpoint_sha256 {
        rejections.insert(IdentityRejection::ContinuityAuthorityMismatch);
    }
    validate_reference(&candidate.continuity.reference, &mut rejections);

    let mut witness_ids = BTreeSet::new();
    let mut witness_digests = BTreeSet::new();
    for witness in &candidate.witnesses {
        if !witness_ids.insert(witness.id.clone()) {
            rejections.insert(IdentityRejection::DuplicateWitness {
                id: witness.id.clone(),
            });
        }
        witness_digests.insert(witness.sha256.clone());
        validate_reference(witness, &mut rejections);
    }
    if candidate.witnesses.is_empty() {
        rejections.insert(IdentityRejection::MissingWitnesses);
    }
    if !witness_digests.contains(&evidence.private_record_sha256)
        || !witness_digests.contains(&evidence.projection_receipt.projection_sha256)
    {
        rejections.insert(IdentityRejection::MissingGovernedWitness);
    }
    if candidate.governed_projection.sha256 != evidence.projection_receipt.projection_sha256 {
        rejections.insert(IdentityRejection::ProjectionAuthorityMismatch);
    }
    validate_reference(&candidate.governed_projection, &mut rejections);

    if !rejections.is_empty() {
        return Err(rejections.into_iter().collect());
    }

    let mut record = BirthdayIdentityRecord {
        schema: BIRTHDAY_IDENTITY_RECORD_SCHEMA.to_owned(),
        stable_name: candidate.stable_name.clone(),
        identity_root: candidate.identity_root.clone(),
        aliases: candidate.aliases.clone(),
        origin: candidate.origin.clone(),
        continuity: candidate.continuity.clone(),
        provenance: candidate.provenance.clone(),
        witnesses: candidate.witnesses.clone(),
        governed_projection: candidate.governed_projection.clone(),
        projection_receipt: evidence.projection_receipt.clone(),
        record_sha256: String::new(),
    };
    canonicalize_record(&mut record);
    record.record_sha256 =
        record_digest(&record).map_err(|_| vec![IdentityRejection::EncodingFailure])?;
    Ok(record)
}

pub fn record_digest(record: &BirthdayIdentityRecord) -> Result<String, serde_json::Error> {
    let mut unsigned = record.clone();
    unsigned.record_sha256.clear();
    serde_jcs::to_vec(&unsigned).map(|bytes| sha256(&bytes))
}

pub fn validate_birthday_identity_record(
    record: &BirthdayIdentityRecord,
    evidence: &VerifiedBirthdayEvidence,
) -> Result<(), Vec<IdentityRejection>> {
    let candidate = BirthdayIdentityCandidate {
        schema: BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA.to_owned(),
        basis: IdentityBasis::OriginEvidence,
        stable_name: record.stable_name.clone(),
        identity_root: record.identity_root.clone(),
        aliases: record.aliases.clone(),
        origin: record.origin.clone(),
        continuity: record.continuity.clone(),
        provenance: record.provenance.clone(),
        witnesses: record.witnesses.clone(),
        governed_projection: record.governed_projection.clone(),
    };
    let mut rejections = BTreeSet::new();
    let expected = match build_birthday_identity(&candidate, evidence) {
        Ok(expected) => Some(expected),
        Err(errors) => {
            rejections.extend(errors);
            None
        }
    };
    if record.schema != BIRTHDAY_IDENTITY_RECORD_SCHEMA {
        rejections.insert(IdentityRejection::UnsupportedSchema);
    }
    if record.projection_receipt != evidence.projection_receipt {
        rejections.insert(IdentityRejection::ProjectionAuthorityMismatch);
    }
    if record_digest(record).ok().as_deref() != Some(record.record_sha256.as_str()) {
        rejections.insert(IdentityRejection::RecordDigestMismatch);
    }
    if expected.as_ref().is_some_and(|expected| record != expected) {
        rejections.insert(IdentityRejection::NonCanonicalRecord);
    }
    if rejections.is_empty() {
        Ok(())
    } else {
        Err(rejections.into_iter().collect())
    }
}

fn canonicalize_record(record: &mut BirthdayIdentityRecord) {
    record.aliases.sort_by(|left, right| {
        name_key(&left.name)
            .cmp(&name_key(&right.name))
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| left.provenance_id.cmp(&right.provenance_id))
    });
    record.provenance.sort();
    record.witnesses.sort();
    record.projection_receipt.redacted_fields.sort();
}

fn validate_reference(reference: &IdentityReference, rejections: &mut BTreeSet<IdentityRejection>) {
    if !safe_id(&reference.id) {
        rejections.insert(IdentityRejection::InvalidReference {
            id: reference.id.clone(),
        });
    }
    if !safe_repository_path(&reference.path) {
        rejections.insert(IdentityRejection::UnsafeReferencePath {
            id: reference.id.clone(),
        });
    }
    if !is_sha256(&reference.sha256) {
        rejections.insert(IdentityRejection::MalformedReferenceDigest {
            id: reference.id.clone(),
        });
    }
}

fn map_identity_error(error: IdentityMemoryError) -> BirthdayEvidenceError {
    match error {
        IdentityMemoryError::Signature => BirthdayEvidenceError::IdentitySignature,
        IdentityMemoryError::UnauthorizedIdentity => BirthdayEvidenceError::IdentityUnauthorized,
        _ => BirthdayEvidenceError::IdentityContinuity,
    }
}

fn map_private_error(error: PrivateStateError) -> BirthdayEvidenceError {
    match error {
        PrivateStateError::Signature => BirthdayEvidenceError::PrivateSignature,
        PrivateStateError::Unauthorized => BirthdayEvidenceError::PrivateUnauthorized,
        PrivateStateError::Projection => BirthdayEvidenceError::PrivateProjection,
        PrivateStateError::Encoding(_) => BirthdayEvidenceError::EncodingFailure,
        _ => BirthdayEvidenceError::PrivateLineage,
    }
}

fn digest_jcs<T: Serialize>(value: &T) -> Result<String, BirthdayEvidenceError> {
    serde_jcs::to_vec(value)
        .map(|bytes| sha256(&bytes))
        .map_err(|_| BirthdayEvidenceError::EncodingFailure)
}

fn is_raw_private_field(field: &str) -> bool {
    matches!(
        field,
        "raw" | "raw_private_state" | "private_state" | "sealed_payload"
    )
}

fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.trim() == value
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b' ' | b'-' | b'_' | b'.'))
}

fn name_key(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn safe_repository_path(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.as_bytes().get(1).copied() == Some(b':')
    {
        return false;
    }
    value
        .split('/')
        .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
