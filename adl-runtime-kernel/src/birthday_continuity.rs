use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    birthday_identity::{
        record_digest as identity_record_digest, validate_birthday_identity_record,
    },
    BirthdayIdentityRecord, CheckpointManifest, IdentityReference, VerifiedBirthdayEvidence,
    CHECKPOINT_SCHEMA,
};

pub const BIRTHDAY_CONTINUITY_RECORD_SCHEMA: &str = "adl.birthday.continuity_record.v1";
const BIRTHDAY_CONTINUITY_HEAD_SCHEMA: &str = "adl.birthday.continuity_head.v1";
const BIRTHDAY_CONTINUITY_AUTHORITY_CONTEXT_SCHEMA: &str =
    "adl.birthday.continuity_authority_context.v1";
const LIVE_RUNTIME_PROVENANCE: &str = "runtime-v3-live-shutdown";
const LIVE_RUNTIME_SNAPSHOT_FILE: &str = "0000-live_kernel.bin";

/// Runtime-provisioned authority for accepting Birthday continuity evidence.
///
/// Construction is crate-private so a candidate cannot nominate a key and a
/// self-consistent identity record as its own trust root.
///
/// ```compile_fail
/// use adl_runtime_kernel::BirthdayContinuityAuthorityPolicy;
/// let _attacker_policy_factory = BirthdayContinuityAuthorityPolicy::establish;
/// ```
#[derive(Clone)]
pub struct BirthdayContinuityAuthorityPolicy {
    trusted_keys: BTreeMap<String, VerifyingKey>,
    signing_key_id: String,
    identity_record_sha256: String,
    topology_hash: String,
    config_hash: String,
    service_schema: String,
    first_generation: u64,
    first_previous_integrity: Option<String>,
    authority_context_sha256: String,
}

impl BirthdayContinuityAuthorityPolicy {
    #[cfg_attr(not(test), allow(dead_code))]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn establish(
        trusted_keys: BTreeMap<String, VerifyingKey>,
        signing_key_id: impl Into<String>,
        identity: &BirthdayIdentityRecord,
        identity_evidence: &VerifiedBirthdayEvidence,
        topology_hash: impl Into<String>,
        config_hash: impl Into<String>,
        service_schema: impl Into<String>,
        first_generation: u64,
        first_previous_integrity: Option<String>,
    ) -> Result<Self, ContinuityRejection> {
        let mut policy = Self {
            trusted_keys,
            signing_key_id: signing_key_id.into(),
            identity_record_sha256: identity.record_sha256.clone(),
            topology_hash: topology_hash.into(),
            config_hash: config_hash.into(),
            service_schema: service_schema.into(),
            first_generation,
            first_previous_integrity,
            authority_context_sha256: String::new(),
        };
        if !policy.trusted_keys.contains_key(&policy.signing_key_id)
            || validate_birthday_identity_record(identity, identity_evidence).is_err()
            || !is_sha256(&policy.topology_hash)
            || !is_sha256(&policy.config_hash)
            || policy.service_schema.is_empty()
            || policy.first_generation == 0
            || (policy.first_generation == 1 && policy.first_previous_integrity.is_some())
            || (policy.first_generation > 1
                && !policy
                    .first_previous_integrity
                    .as_deref()
                    .is_some_and(is_canonical_digest))
        {
            return Err(ContinuityRejection::PolicyInvalid);
        }
        policy.authority_context_sha256 =
            authority_context_digest(&policy).map_err(|_| ContinuityRejection::PolicyInvalid)?;
        Ok(policy)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BirthdayCycleEvidence<'a> {
    pub manifest: &'a CheckpointManifest,
}

/// Opaque result of verifying one signed cycle against runtime trust policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedBirthdayCycle {
    identity_root: String,
    identity_record_sha256: String,
    generation: u64,
    accepted_through: u64,
    previous_integrity: String,
    integrity: String,
    signing_key_id: String,
    reference: IdentityReference,
    authority_context_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityGrade {
    EvidenceBacked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthdayCycleRecord {
    pub generation: u64,
    pub accepted_through: u64,
    pub previous_integrity: String,
    pub integrity: String,
    pub signing_key_id: String,
    pub reference: IdentityReference,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthdayContinuityRecord {
    pub schema: String,
    pub identity_root: String,
    pub identity_record_sha256: String,
    pub predecessor_head: String,
    pub authority_context_sha256: String,
    pub cycles: Vec<BirthdayCycleRecord>,
    pub grade: ContinuityGrade,
    pub continuity_head: String,
    pub record_sha256: String,
}

/// Opaque proof that a Birthday continuity record was rebuilt from trusted,
/// signed Runtime cycles and the bound identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedBirthdayContinuity {
    record: BirthdayContinuityRecord,
    identity_checkpoint_head: String,
}

impl VerifiedBirthdayContinuity {
    pub fn record(&self) -> &BirthdayContinuityRecord {
        &self.record
    }
    pub fn continuity_head(&self) -> &str {
        &self.record.continuity_head
    }
    pub fn identity_checkpoint_head(&self) -> &str {
        &self.identity_checkpoint_head
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ContinuityRejection {
    PolicyInvalid,
    IdentityRecordMismatch,
    AuthorityContextMismatch { generation: u64 },
    InsufficientCycles,
    UnsupportedCheckpointSchema { generation: u64 },
    WrongSigner { generation: u64 },
    InvalidSignature { generation: u64 },
    InvalidIntegrity { generation: u64 },
    WrongGeneration { expected: u64, actual: u64 },
    MissingPredecessor { generation: u64 },
    DiscontinuousPredecessor { generation: u64 },
    NonMonotonicEvidence { generation: u64 },
    RuntimeIdentityMismatch { generation: u64 },
    RuntimeProvenanceMismatch { generation: u64 },
    MissingRuntimeWitness { generation: u64 },
    DuplicateRuntimeWitness { generation: u64 },
    UnsafeWitnessPath { generation: u64 },
    WitnessDigestMismatch { generation: u64 },
    DuplicateCycle { generation: u64 },
    GenerationOverflow,
    RecordDigestMismatch,
    ContinuityHeadMismatch,
    NonCanonicalRecord,
    EncodingFailure,
}

pub fn verify_birthday_cycles(
    policy: &BirthdayContinuityAuthorityPolicy,
    identity: &BirthdayIdentityRecord,
    evidence: &[BirthdayCycleEvidence<'_>],
) -> Result<Vec<VerifiedBirthdayCycle>, Vec<ContinuityRejection>> {
    let mut rejections = BTreeSet::new();
    if identity_record_digest(identity).ok().as_deref() != Some(&identity.record_sha256)
        || identity.record_sha256 != policy.identity_record_sha256
    {
        rejections.insert(ContinuityRejection::IdentityRecordMismatch);
    }
    if evidence.len() < 2 {
        rejections.insert(ContinuityRejection::InsufficientCycles);
    }

    let mut expected_generation = policy.first_generation;
    let mut expected_runtime_predecessor = policy.first_previous_integrity.as_deref();
    let mut previous_accepted = 0;
    let mut seen_integrities = BTreeSet::new();
    let mut verified = Vec::with_capacity(evidence.len());

    for (index, item) in evidence.iter().enumerate() {
        let manifest = item.manifest;
        let generation = manifest.generation;
        if manifest.schema != CHECKPOINT_SCHEMA {
            rejections.insert(ContinuityRejection::UnsupportedCheckpointSchema { generation });
        }
        if manifest.signing_key_id != policy.signing_key_id {
            rejections.insert(ContinuityRejection::WrongSigner { generation });
        }
        if verify_manifest_signature(manifest, &policy.trusted_keys).is_err() {
            rejections.insert(ContinuityRejection::InvalidSignature { generation });
        }
        if verify_manifest_integrity(manifest).is_err() {
            rejections.insert(ContinuityRejection::InvalidIntegrity { generation });
        }
        if generation != expected_generation {
            rejections.insert(ContinuityRejection::WrongGeneration {
                expected: expected_generation,
                actual: generation,
            });
        }
        if manifest.previous_integrity.as_deref() != expected_runtime_predecessor {
            if manifest.previous_integrity.is_none() && expected_runtime_predecessor.is_some() {
                rejections.insert(ContinuityRejection::MissingPredecessor { generation });
            } else {
                rejections.insert(ContinuityRejection::DiscontinuousPredecessor { generation });
            }
        }
        if manifest.accepted_through <= previous_accepted {
            rejections.insert(ContinuityRejection::NonMonotonicEvidence { generation });
        }
        if manifest.topology_hash != policy.topology_hash
            || manifest.config_hash != policy.config_hash
        {
            rejections.insert(ContinuityRejection::RuntimeIdentityMismatch { generation });
        }
        if manifest.provenance != LIVE_RUNTIME_PROVENANCE {
            rejections.insert(ContinuityRejection::RuntimeProvenanceMismatch { generation });
        }
        let matching_services = manifest
            .snapshots
            .iter()
            .filter(|entry| {
                entry.service == "live_kernel" && entry.service_schema == policy.service_schema
            })
            .count();
        if matching_services == 0 {
            rejections.insert(ContinuityRejection::MissingRuntimeWitness { generation });
        } else if matching_services > 1 {
            rejections.insert(ContinuityRejection::DuplicateRuntimeWitness { generation });
        }
        if manifest.snapshots.iter().any(|entry| {
            entry.service != "live_kernel"
                || entry.service_schema != policy.service_schema
                || !governed_continuity_path(&entry.file)
        }) {
            rejections.insert(ContinuityRejection::UnsafeWitnessPath { generation });
        }
        if !seen_integrities.insert(manifest.integrity.clone()) {
            rejections.insert(ContinuityRejection::DuplicateCycle { generation });
        }

        verified.push(VerifiedBirthdayCycle {
            identity_root: identity.identity_root.clone(),
            identity_record_sha256: identity.record_sha256.clone(),
            generation,
            accepted_through: manifest.accepted_through,
            previous_integrity: if index == 0 {
                identity.continuity.head_sha256.clone()
            } else {
                manifest.previous_integrity.clone().unwrap_or_default()
            },
            integrity: manifest.integrity.clone(),
            signing_key_id: manifest.signing_key_id.clone(),
            reference: canonical_cycle_reference(manifest)
                .map_err(|_| vec![ContinuityRejection::EncodingFailure])?,
            authority_context_sha256: policy.authority_context_sha256.clone(),
        });
        match expected_generation.checked_add(1) {
            Some(next) => expected_generation = next,
            None if index + 1 < evidence.len() => {
                rejections.insert(ContinuityRejection::GenerationOverflow);
            }
            None => {}
        }
        expected_runtime_predecessor = Some(&manifest.integrity);
        previous_accepted = manifest.accepted_through;
    }
    if rejections.is_empty() {
        Ok(verified)
    } else {
        Err(rejections.into_iter().collect())
    }
}

pub fn build_birthday_continuity(
    identity: &BirthdayIdentityRecord,
    cycles: &[VerifiedBirthdayCycle],
) -> Result<BirthdayContinuityRecord, Vec<ContinuityRejection>> {
    let mut rejections = BTreeSet::new();
    if identity_record_digest(identity).ok().as_deref() != Some(&identity.record_sha256) {
        rejections.insert(ContinuityRejection::IdentityRecordMismatch);
    }
    if cycles.len() < 2 {
        rejections.insert(ContinuityRejection::InsufficientCycles);
        return Err(rejections.into_iter().collect());
    }
    let authority_context_sha256 = cycles[0].authority_context_sha256.clone();
    if !is_sha256(&authority_context_sha256) {
        rejections.insert(ContinuityRejection::AuthorityContextMismatch {
            generation: cycles[0].generation,
        });
    }
    let mut expected_generation = cycles[0].generation;
    let mut expected_predecessor = identity.continuity.head_sha256.as_str();
    let mut previous_accepted = 0;
    let mut seen_integrities = BTreeSet::new();
    for (index, cycle) in cycles.iter().enumerate() {
        if cycle.identity_root != identity.identity_root
            || cycle.identity_record_sha256 != identity.record_sha256
        {
            rejections.insert(ContinuityRejection::IdentityRecordMismatch);
        }
        if cycle.authority_context_sha256 != authority_context_sha256 {
            rejections.insert(ContinuityRejection::AuthorityContextMismatch {
                generation: cycle.generation,
            });
        }
        if cycle.generation != expected_generation {
            rejections.insert(ContinuityRejection::WrongGeneration {
                expected: expected_generation,
                actual: cycle.generation,
            });
        }
        if cycle.previous_integrity != expected_predecessor {
            rejections.insert(ContinuityRejection::DiscontinuousPredecessor {
                generation: cycle.generation,
            });
        }
        if cycle.accepted_through <= previous_accepted {
            rejections.insert(ContinuityRejection::NonMonotonicEvidence {
                generation: cycle.generation,
            });
        }
        if !seen_integrities.insert(cycle.integrity.clone()) {
            rejections.insert(ContinuityRejection::DuplicateCycle {
                generation: cycle.generation,
            });
        }
        expected_predecessor = &cycle.integrity;
        previous_accepted = cycle.accepted_through;
        match expected_generation.checked_add(1) {
            Some(next) => expected_generation = next,
            None if index + 1 < cycles.len() => {
                rejections.insert(ContinuityRejection::GenerationOverflow);
            }
            None => {}
        }
    }
    if !rejections.is_empty() {
        return Err(rejections.into_iter().collect());
    }
    let cycle_records = cycles
        .iter()
        .map(|cycle| BirthdayCycleRecord {
            generation: cycle.generation,
            accepted_through: cycle.accepted_through,
            previous_integrity: cycle.previous_integrity.clone(),
            integrity: cycle.integrity.clone(),
            signing_key_id: cycle.signing_key_id.clone(),
            reference: cycle.reference.clone(),
        })
        .collect::<Vec<_>>();
    let continuity_head = derive_continuity_head(
        &identity.identity_root,
        &identity.record_sha256,
        &identity.continuity.head_sha256,
        &authority_context_sha256,
        &cycle_records,
    )
    .map_err(|_| vec![ContinuityRejection::EncodingFailure])?;
    let mut record = BirthdayContinuityRecord {
        schema: BIRTHDAY_CONTINUITY_RECORD_SCHEMA.to_owned(),
        identity_root: identity.identity_root.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        predecessor_head: identity.continuity.head_sha256.clone(),
        authority_context_sha256,
        cycles: cycle_records,
        grade: ContinuityGrade::EvidenceBacked,
        continuity_head,
        record_sha256: String::new(),
    };
    record.record_sha256 = continuity_record_digest(&record)
        .map_err(|_| vec![ContinuityRejection::EncodingFailure])?;
    Ok(record)
}

pub fn validate_birthday_continuity_record(
    record: &BirthdayContinuityRecord,
    identity: &BirthdayIdentityRecord,
    cycles: &[VerifiedBirthdayCycle],
) -> Result<(), Vec<ContinuityRejection>> {
    let mut rejections = BTreeSet::new();
    if continuity_record_digest(record).ok().as_deref() != Some(&record.record_sha256) {
        rejections.insert(ContinuityRejection::RecordDigestMismatch);
    }
    let expected = build_birthday_continuity(identity, cycles);
    match expected {
        Ok(expected) => {
            if expected.continuity_head != record.continuity_head {
                rejections.insert(ContinuityRejection::ContinuityHeadMismatch);
            }
            if &expected != record {
                rejections.insert(ContinuityRejection::NonCanonicalRecord);
            }
        }
        Err(errors) => rejections.extend(errors),
    }
    if rejections.is_empty() {
        Ok(())
    } else {
        Err(rejections.into_iter().collect())
    }
}

pub fn verify_birthday_continuity_record(
    record: &BirthdayContinuityRecord,
    identity: &BirthdayIdentityRecord,
    cycles: &[VerifiedBirthdayCycle],
) -> Result<VerifiedBirthdayContinuity, Vec<ContinuityRejection>> {
    validate_birthday_continuity_record(record, identity, cycles)?;
    Ok(VerifiedBirthdayContinuity {
        record: record.clone(),
        identity_checkpoint_head: identity.continuity.head_sha256.clone(),
    })
}

pub fn continuity_record_digest(
    record: &BirthdayContinuityRecord,
) -> Result<String, serde_json::Error> {
    let mut unsigned = record.clone();
    unsigned.record_sha256.clear();
    digest_jcs(&unsigned)
}

fn derive_continuity_head(
    identity_root: &str,
    identity_record_sha256: &str,
    predecessor_head: &str,
    authority_context_sha256: &str,
    cycles: &[BirthdayCycleRecord],
) -> Result<String, serde_json::Error> {
    #[derive(Serialize)]
    struct HeadMaterial<'a> {
        schema: &'static str,
        identity_root: &'a str,
        identity_record_sha256: &'a str,
        predecessor_head: &'a str,
        authority_context_sha256: &'a str,
        cycles: &'a [BirthdayCycleRecord],
    }
    digest_jcs(&HeadMaterial {
        schema: BIRTHDAY_CONTINUITY_HEAD_SCHEMA,
        identity_root,
        identity_record_sha256,
        predecessor_head,
        authority_context_sha256,
        cycles,
    })
}

fn verify_manifest_signature(
    manifest: &CheckpointManifest,
    trusted_keys: &BTreeMap<String, VerifyingKey>,
) -> Result<(), ()> {
    if manifest.signing_algorithm != "ed25519" {
        return Err(());
    }
    let key = trusted_keys.get(&manifest.signing_key_id).ok_or(())?;
    let signature = Signature::from_slice(&hex::decode(&manifest.signature).map_err(|_| ())?)
        .map_err(|_| ())?;
    let mut unsigned = manifest.clone();
    unsigned.signature.clear();
    let bytes = serde_json::to_vec(&unsigned).map_err(|_| ())?;
    key.verify(&bytes, &signature).map_err(|_| ())
}

fn verify_manifest_integrity(manifest: &CheckpointManifest) -> Result<(), ()> {
    let mut unsigned = manifest.clone();
    unsigned.integrity.clear();
    unsigned.signature.clear();
    let bytes = serde_json::to_vec(&unsigned).map_err(|_| ())?;
    if blake3::hash(&bytes).to_hex().as_str() == manifest.integrity {
        Ok(())
    } else {
        Err(())
    }
}

fn manifest_digest(manifest: &CheckpointManifest) -> Result<String, serde_json::Error> {
    digest_jcs(manifest)
}

fn canonical_cycle_reference(
    manifest: &CheckpointManifest,
) -> Result<IdentityReference, serde_json::Error> {
    Ok(IdentityReference {
        id: format!("bounded-cycle-{}", manifest.generation),
        path: format!("evidence/continuity/cycle-{}.json", manifest.generation),
        sha256: manifest_digest(manifest)?,
    })
}

fn authority_context_digest(
    policy: &BirthdayContinuityAuthorityPolicy,
) -> Result<String, serde_json::Error> {
    #[derive(Serialize)]
    struct TrustedKey<'a> {
        key_id: &'a str,
        ed25519_public_key: String,
    }

    #[derive(Serialize)]
    struct AuthorityContext<'a> {
        schema: &'static str,
        trusted_keys: Vec<TrustedKey<'a>>,
        signing_key_id: &'a str,
        identity_record_sha256: &'a str,
        topology_hash: &'a str,
        config_hash: &'a str,
        service_schema: &'a str,
        first_generation: u64,
        first_previous_integrity: Option<&'a str>,
    }

    let trusted_keys = policy
        .trusted_keys
        .iter()
        .map(|(key_id, key)| TrustedKey {
            key_id,
            ed25519_public_key: hex::encode(key.as_bytes()),
        })
        .collect();
    digest_jcs(&AuthorityContext {
        schema: BIRTHDAY_CONTINUITY_AUTHORITY_CONTEXT_SCHEMA,
        trusted_keys,
        signing_key_id: &policy.signing_key_id,
        identity_record_sha256: &policy.identity_record_sha256,
        topology_hash: &policy.topology_hash,
        config_hash: &policy.config_hash,
        service_schema: &policy.service_schema,
        first_generation: policy.first_generation,
        first_previous_integrity: policy.first_previous_integrity.as_deref(),
    })
}

fn digest_jcs<T: Serialize + ?Sized>(value: &T) -> Result<String, serde_json::Error> {
    serde_jcs::to_vec(value).map(|bytes| sha256(&bytes))
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn is_canonical_digest(value: &str) -> bool {
    is_sha256(value)
}

#[cfg(any(test, feature = "test-support"))]
#[path = "../tests/fixtures/birthday_continuity/authority_tests.rs"]
#[cfg_attr(feature = "test-support", allow(dead_code, unused_imports))]
pub(crate) mod authority_tests;

fn safe_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.starts_with('\\')
        && !value.contains('\\')
        && value.as_bytes().get(1).copied() != Some(b':')
        && value
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}

fn governed_continuity_path(value: &str) -> bool {
    safe_path(value) && value == LIVE_RUNTIME_SNAPSHOT_FILE
}
