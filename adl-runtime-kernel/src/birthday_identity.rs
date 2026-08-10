use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA: &str = "adl.birthday.identity_candidate.v1";
pub const BIRTHDAY_IDENTITY_RECORD_SCHEMA: &str = "adl.birthday.identity_record.v1";
const IDENTITY_ROOT_MATERIAL_SCHEMA: &str = "adl.birthday.identity_root_material.v1";

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

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityReferenceVisibility {
    ReviewerVisible,
    RawPrivate,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityReference {
    pub id: String,
    pub path: String,
    pub sha256: String,
    pub visibility: IdentityReferenceVisibility,
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
pub struct RedactionPolicy {
    pub projection_only: bool,
    pub raw_private_state: bool,
    #[serde(default)]
    pub redacted_fields: Vec<String>,
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
    pub redaction_policy: RedactionPolicy,
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
    MissingProvenance { id: String },
    DuplicateProvenance { id: String },
    DuplicateWitness { id: String },
    MissingWitnesses,
    InvalidReference { id: String },
    UnsafeReferencePath { id: String },
    PrivateReference { id: String },
    MalformedReferenceDigest { id: String },
    ContinuityIdentitySubstitution,
    MalformedContinuityHead,
    UnsafeRedactionPolicy,
    InvalidRedactedField { field: String },
    DuplicateRedactedField { field: String },
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
    pub redaction_policy: RedactionPolicy,
    pub record_sha256: String,
}

#[derive(Serialize)]
struct RootMaterial<'a> {
    schema: &'static str,
    stable_name: &'a str,
    origin_event_id: &'a str,
    origin_provenance_id: &'a str,
    origin_reference_sha256: &'a str,
}

pub fn derive_identity_root(
    candidate: &BirthdayIdentityCandidate,
) -> Result<String, serde_json::Error> {
    let material = RootMaterial {
        schema: IDENTITY_ROOT_MATERIAL_SCHEMA,
        stable_name: &candidate.stable_name,
        origin_event_id: &candidate.origin.event_id,
        origin_provenance_id: &candidate.origin.provenance_id,
        origin_reference_sha256: &candidate.origin.reference.sha256,
    };
    Ok(sha256(&serde_jcs::to_vec(&material)?))
}

pub fn build_birthday_identity(
    candidate: &BirthdayIdentityCandidate,
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
    if derive_identity_root(candidate).ok().as_deref() != Some(candidate.identity_root.as_str()) {
        rejections.insert(IdentityRejection::IdentityRootMismatch);
    }
    if !safe_id(&candidate.origin.event_id) {
        rejections.insert(IdentityRejection::InvalidOriginEvent);
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
    if !provenance_by_id.contains_key(&candidate.origin.provenance_id) {
        rejections.insert(IdentityRejection::MissingProvenance {
            id: candidate.origin.provenance_id.clone(),
        });
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
        if !provenance_by_id.contains_key(&alias.provenance_id) {
            rejections.insert(IdentityRejection::MissingProvenance {
                id: alias.provenance_id.clone(),
            });
        }
    }

    if candidate.continuity.identity_root != candidate.identity_root {
        rejections.insert(IdentityRejection::ContinuityIdentitySubstitution);
    }
    if !is_sha256(&candidate.continuity.head_sha256) {
        rejections.insert(IdentityRejection::MalformedContinuityHead);
    }
    validate_reference(&candidate.continuity.reference, &mut rejections);

    let mut witness_ids = BTreeSet::new();
    for witness in &candidate.witnesses {
        if !witness_ids.insert(witness.id.clone()) {
            rejections.insert(IdentityRejection::DuplicateWitness {
                id: witness.id.clone(),
            });
        }
        validate_reference(witness, &mut rejections);
    }
    if candidate.witnesses.is_empty() {
        rejections.insert(IdentityRejection::MissingWitnesses);
    }

    if !candidate.redaction_policy.projection_only || candidate.redaction_policy.raw_private_state {
        rejections.insert(IdentityRejection::UnsafeRedactionPolicy);
    }
    let mut redacted_fields = BTreeSet::new();
    for field in &candidate.redaction_policy.redacted_fields {
        if !safe_id(field) {
            rejections.insert(IdentityRejection::InvalidRedactedField {
                field: field.clone(),
            });
        } else if !redacted_fields.insert(field.clone()) {
            rejections.insert(IdentityRejection::DuplicateRedactedField {
                field: field.clone(),
            });
        }
    }

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
        redaction_policy: candidate.redaction_policy.clone(),
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
        redaction_policy: record.redaction_policy.clone(),
    };
    let mut rejections = BTreeSet::new();
    let expected = match build_birthday_identity(&candidate) {
        Ok(expected) => Some(expected),
        Err(errors) => {
            rejections.extend(errors);
            None
        }
    };
    if record.schema != BIRTHDAY_IDENTITY_RECORD_SCHEMA {
        rejections.insert(IdentityRejection::UnsupportedSchema);
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
    record.redaction_policy.redacted_fields.sort();
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
    if reference.visibility == IdentityReferenceVisibility::RawPrivate {
        rejections.insert(IdentityRejection::PrivateReference {
            id: reference.id.clone(),
        });
    }
    if !is_sha256(&reference.sha256) {
        rejections.insert(IdentityRejection::MalformedReferenceDigest {
            id: reference.id.clone(),
        });
    }
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
