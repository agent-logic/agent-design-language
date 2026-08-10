use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    birthday_identity::record_digest as identity_digest, candidate_digest,
    continuity_record_digest, decide_birthday, validate_capability_envelope, BirthdayCandidate,
    BirthdayContinuityRecord, BirthdayIdentityRecord, CapabilityEnvelope, CapabilityEnvelopePolicy,
    BIRTHDAY_IDENTITY_RECORD_SCHEMA,
};

pub const COGNITIVE_PROFILE_INPUT_SCHEMA: &str = "adl.cognitive_profile.input.v1";
pub const COGNITIVE_PROFILE_POLICY_SCHEMA: &str = "adl.cognitive_profile.policy.v1";
pub const COGNITIVE_PROFILE_SCHEMA: &str = "adl.cognitive_profile.v1";
pub const COGNITIVE_PROFILE_PUBLIC_SCHEMA: &str = "adl.cognitive_profile.public.v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveEvidenceCategory {
    Identity,
    Continuity,
    Memory,
    Capability,
    TheoryOfMind,
    Intelligence,
    GovernedLearning,
}

impl CognitiveEvidenceCategory {
    pub const REQUIRED: [Self; 7] = [
        Self::Identity,
        Self::Continuity,
        Self::Memory,
        Self::Capability,
        Self::TheoryOfMind,
        Self::Intelligence,
        Self::GovernedLearning,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveEvidenceVisibility {
    InternalRedacted,
    Public,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveEvidenceReference {
    pub id: String,
    pub category: CognitiveEvidenceCategory,
    pub path: String,
    pub sha256: String,
    pub revision_sha256: String,
    pub visibility: CognitiveEvidenceVisibility,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveProfileField {
    pub key: String,
    pub value: String,
    pub evidence_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllowedCognitiveField {
    pub key: String,
    pub allowed_values: Vec<String>,
    pub public: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveProfilePolicy {
    pub schema: String,
    pub evidence: Vec<CognitiveEvidenceReference>,
    pub allowed_fields: Vec<AllowedCognitiveField>,
    pub required_nonclaims: Vec<String>,
    pub redaction_policy_sha256: String,
    pub capability_policy: CapabilityEnvelopePolicy,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveProfileInput {
    pub schema: String,
    pub profile_id: String,
    pub revision: u64,
    pub previous_profile_sha256: Option<String>,
    pub birthday_candidate_sha256: String,
    pub identity_record_sha256: String,
    pub continuity_record_sha256: String,
    pub capability_envelope_sha256: String,
    pub update_actor: String,
    pub update_reason: String,
    pub added_fields: Vec<String>,
    pub removed_fields: Vec<String>,
    pub evidence: Vec<CognitiveEvidenceReference>,
    pub fields: Vec<CognitiveProfileField>,
    pub nonclaims: Vec<String>,
    pub redaction_policy_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCognitiveField {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCognitiveProfile {
    pub schema: String,
    pub profile_id: String,
    pub revision: u64,
    pub identity_root: String,
    pub fields: Vec<PublicCognitiveField>,
    pub nonclaims: Vec<String>,
    pub source_profile_sha256: String,
    pub projection_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveProfile {
    pub schema: String,
    pub profile_id: String,
    pub revision: u64,
    pub previous_profile_sha256: Option<String>,
    pub identity_root: String,
    pub continuity_head: String,
    pub birthday_candidate_sha256: String,
    pub identity_record_sha256: String,
    pub continuity_record_sha256: String,
    pub capability_envelope_sha256: String,
    pub update_actor: String,
    pub update_reason: String,
    pub added_fields: Vec<String>,
    pub removed_fields: Vec<String>,
    pub evidence: Vec<CognitiveEvidenceReference>,
    pub fields: Vec<CognitiveProfileField>,
    pub nonclaims: Vec<String>,
    pub redaction_policy_sha256: String,
    pub policy_sha256: String,
    pub canonical_input_sha256: String,
    pub profile_sha256: String,
    pub public_projection: PublicCognitiveProfile,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum CognitiveProfileRejection {
    UnsupportedSchema,
    InvalidAuthority,
    IdentityMismatch,
    ContinuityMismatch,
    CapabilityMismatch,
    InvalidProfileId,
    InvalidRevisionLink,
    UnexplainedMutation,
    MissingEvidence { category: CognitiveEvidenceCategory },
    EvidenceMismatch,
    InvalidEvidence,
    DuplicateEvidence,
    UnsupportedField,
    MissingFieldEvidence,
    ForbiddenInference,
    PrivacyViolation,
    MissingNonclaim,
    InvalidUpdate,
    NonCanonicalProfile,
    DigestMismatch,
    EncodingFailure,
}

pub fn build_cognitive_profile(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
) -> Result<CognitiveProfile, Vec<CognitiveProfileRejection>> {
    let mut errors = BTreeSet::new();
    validate_authorities(
        birthday,
        identity,
        continuity,
        capability,
        input,
        policy,
        &mut errors,
    );
    let mut canonical = input.clone();
    canonicalize_input(&mut canonical);
    let mut canonical_policy = policy.clone();
    canonicalize_policy(&mut canonical_policy);
    let expected_policy_sha256 = digest(&canonical_policy).ok();
    validate_policy_and_input(
        &canonical,
        &canonical_policy,
        previous,
        &identity.identity_root,
        expected_policy_sha256.as_deref(),
        &mut errors,
    );
    if !errors.is_empty() {
        return Err(errors.into_iter().collect());
    }

    let policy_sha256 = digest(&canonical_policy)?;
    let canonical_input_sha256 = digest(&canonical)?;
    let mut profile = CognitiveProfile {
        schema: COGNITIVE_PROFILE_SCHEMA.into(),
        profile_id: canonical.profile_id,
        revision: canonical.revision,
        previous_profile_sha256: canonical.previous_profile_sha256,
        identity_root: identity.identity_root.clone(),
        continuity_head: continuity.continuity_head.clone(),
        birthday_candidate_sha256: canonical.birthday_candidate_sha256,
        identity_record_sha256: canonical.identity_record_sha256,
        continuity_record_sha256: canonical.continuity_record_sha256,
        capability_envelope_sha256: canonical.capability_envelope_sha256,
        update_actor: canonical.update_actor,
        update_reason: canonical.update_reason,
        added_fields: canonical.added_fields,
        removed_fields: canonical.removed_fields,
        evidence: canonical.evidence,
        fields: canonical.fields,
        nonclaims: canonical.nonclaims,
        redaction_policy_sha256: canonical.redaction_policy_sha256,
        policy_sha256,
        canonical_input_sha256,
        profile_sha256: String::new(),
        public_projection: PublicCognitiveProfile {
            schema: COGNITIVE_PROFILE_PUBLIC_SCHEMA.into(),
            profile_id: String::new(),
            revision: 0,
            identity_root: String::new(),
            fields: vec![],
            nonclaims: vec![],
            source_profile_sha256: String::new(),
            projection_sha256: String::new(),
        },
    };
    profile.profile_sha256 = profile_digest(&profile)?;
    let allowed: BTreeMap<_, _> = canonical_policy
        .allowed_fields
        .iter()
        .map(|f| (f.key.as_str(), f))
        .collect();
    let public_fields = profile
        .fields
        .iter()
        .filter(|field| {
            allowed
                .get(field.key.as_str())
                .is_some_and(|rule| rule.public)
        })
        .map(|field| PublicCognitiveField {
            key: field.key.clone(),
            value: field.value.clone(),
        })
        .collect::<Vec<_>>();
    if public_fields.len() >= profile.fields.len() {
        return Err(vec![CognitiveProfileRejection::PrivacyViolation]);
    }
    let mut projection = PublicCognitiveProfile {
        schema: COGNITIVE_PROFILE_PUBLIC_SCHEMA.into(),
        profile_id: profile.profile_id.clone(),
        revision: profile.revision,
        identity_root: profile.identity_root.clone(),
        fields: public_fields,
        nonclaims: profile.nonclaims.clone(),
        source_profile_sha256: profile.profile_sha256.clone(),
        projection_sha256: String::new(),
    };
    projection.projection_sha256 = public_projection_digest(&projection)?;
    profile.public_projection = projection;
    Ok(profile)
}

pub fn validate_cognitive_profile(
    profile: &CognitiveProfile,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    let input = CognitiveProfileInput {
        schema: COGNITIVE_PROFILE_INPUT_SCHEMA.into(),
        profile_id: profile.profile_id.clone(),
        revision: profile.revision,
        previous_profile_sha256: profile.previous_profile_sha256.clone(),
        birthday_candidate_sha256: profile.birthday_candidate_sha256.clone(),
        identity_record_sha256: profile.identity_record_sha256.clone(),
        continuity_record_sha256: profile.continuity_record_sha256.clone(),
        capability_envelope_sha256: profile.capability_envelope_sha256.clone(),
        update_actor: profile.update_actor.clone(),
        update_reason: profile.update_reason.clone(),
        added_fields: profile.added_fields.clone(),
        removed_fields: profile.removed_fields.clone(),
        evidence: profile.evidence.clone(),
        fields: profile.fields.clone(),
        nonclaims: profile.nonclaims.clone(),
        redaction_policy_sha256: profile.redaction_policy_sha256.clone(),
    };
    match build_cognitive_profile(
        birthday, identity, continuity, capability, &input, policy, previous,
    ) {
        Ok(expected) if &expected == profile => Ok(()),
        Ok(_) => Err(vec![CognitiveProfileRejection::NonCanonicalProfile]),
        Err(errors) => Err(errors),
    }
}

pub fn profile_digest(
    profile: &CognitiveProfile,
) -> Result<String, Vec<CognitiveProfileRejection>> {
    let mut value = profile.clone();
    value.profile_sha256.clear();
    value.public_projection = PublicCognitiveProfile {
        schema: COGNITIVE_PROFILE_PUBLIC_SCHEMA.into(),
        profile_id: String::new(),
        revision: 0,
        identity_root: String::new(),
        fields: vec![],
        nonclaims: vec![],
        source_profile_sha256: String::new(),
        projection_sha256: String::new(),
    };
    digest(&value)
}
pub fn public_projection_digest(
    profile: &PublicCognitiveProfile,
) -> Result<String, Vec<CognitiveProfileRejection>> {
    let mut value = profile.clone();
    value.projection_sha256.clear();
    digest(&value)
}

fn validate_authorities(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    errors: &mut BTreeSet<CognitiveProfileRejection>,
) {
    if input.schema != COGNITIVE_PROFILE_INPUT_SCHEMA
        || policy.schema != COGNITIVE_PROFILE_POLICY_SCHEMA
    {
        errors.insert(CognitiveProfileRejection::UnsupportedSchema);
    }
    if !decide_birthday(b).accepted
        || candidate_digest(b).ok().as_deref() != Some(&b.packet_sha256)
        || input.birthday_candidate_sha256 != b.packet_sha256
    {
        errors.insert(CognitiveProfileRejection::InvalidAuthority);
    }
    if i.schema != BIRTHDAY_IDENTITY_RECORD_SCHEMA
        || identity_digest(i).ok().as_deref() != Some(&i.record_sha256)
        || input.identity_record_sha256 != i.record_sha256
        || i.identity_root != b.identity_root
        || i.stable_name != b.stable_name
    {
        errors.insert(CognitiveProfileRejection::IdentityMismatch);
    }
    if continuity_record_digest(c).ok().as_deref() != Some(&c.record_sha256)
        || input.continuity_record_sha256 != c.record_sha256
        || c.identity_root != i.identity_root
        || c.identity_record_sha256 != i.record_sha256
        || c.continuity_head != b.continuity_head
    {
        errors.insert(CognitiveProfileRejection::ContinuityMismatch);
    }
    if validate_capability_envelope(cap, b, i, &policy.capability_policy).is_err()
        || input.capability_envelope_sha256 != cap.envelope_sha256
        || cap.identity_root != i.identity_root
    {
        errors.insert(CognitiveProfileRejection::CapabilityMismatch);
    }
    if !input
        .evidence
        .iter()
        .any(|e| e.category == CognitiveEvidenceCategory::Identity && e.sha256 == i.record_sha256)
        || !input.evidence.iter().any(|e| {
            e.category == CognitiveEvidenceCategory::Continuity && e.sha256 == c.record_sha256
        })
        || !input.evidence.iter().any(|e| {
            e.category == CognitiveEvidenceCategory::Capability && e.sha256 == cap.envelope_sha256
        })
    {
        errors.insert(CognitiveProfileRejection::EvidenceMismatch);
    }
}

fn validate_policy_and_input(
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
    identity_root: &str,
    expected_policy_sha256: Option<&str>,
    errors: &mut BTreeSet<CognitiveProfileRejection>,
) {
    if !valid_id(&input.profile_id) {
        errors.insert(CognitiveProfileRejection::InvalidProfileId);
    }
    if !is_sha(&policy.redaction_policy_sha256)
        || input.redaction_policy_sha256 != policy.redaction_policy_sha256
    {
        errors.insert(CognitiveProfileRejection::PrivacyViolation);
    }
    let mut policy_evidence_ids = BTreeMap::<String, &str>::new();
    for evidence in &policy.evidence {
        let key = evidence.id.to_ascii_lowercase();
        if policy_evidence_ids
            .insert(key, evidence.id.as_str())
            .is_some()
        {
            errors.insert(CognitiveProfileRejection::DuplicateEvidence);
        }
    }
    let mut policy_field_keys = BTreeMap::<String, &str>::new();
    for field in &policy.allowed_fields {
        let key = field.key.to_ascii_lowercase();
        if policy_field_keys.insert(key, field.key.as_str()).is_some() {
            errors.insert(CognitiveProfileRejection::UnsupportedField);
        }
    }
    let expected: BTreeMap<_, _> = policy.evidence.iter().map(|e| (e.id.as_str(), e)).collect();
    let mut seen = BTreeSet::new();
    for evidence in &input.evidence {
        if !seen.insert(evidence.id.as_str()) {
            errors.insert(CognitiveProfileRejection::DuplicateEvidence);
        }
        if !valid_evidence(evidence) {
            errors.insert(CognitiveProfileRejection::InvalidEvidence);
        }
        if expected.get(evidence.id.as_str()).copied() != Some(evidence) {
            errors.insert(CognitiveProfileRejection::EvidenceMismatch);
        }
    }
    for category in CognitiveEvidenceCategory::REQUIRED {
        if !input.evidence.iter().any(|e| e.category == category) {
            errors.insert(CognitiveProfileRejection::MissingEvidence { category });
        }
    }
    let evidence_by_id: BTreeMap<_, _> =
        input.evidence.iter().map(|e| (e.id.as_str(), e)).collect();
    let evidence_ids: BTreeSet<_> = evidence_by_id.keys().copied().collect();
    let rules: BTreeMap<_, _> = policy
        .allowed_fields
        .iter()
        .map(|f| (f.key.as_str(), f))
        .collect();
    for field in &input.fields {
        if unsafe_text(&field.key)
            || unsafe_text(&field.value)
            || !rules
                .get(field.key.as_str())
                .is_some_and(|rule| rule.allowed_values.contains(&field.value))
        {
            errors.insert(CognitiveProfileRejection::UnsupportedField);
        }
        if field.evidence_ids.is_empty()
            || field
                .evidence_ids
                .iter()
                .any(|id| !evidence_ids.contains(id.as_str()))
        {
            errors.insert(CognitiveProfileRejection::MissingFieldEvidence);
        }
        if rules
            .get(field.key.as_str())
            .is_some_and(|rule| rule.public)
            && field.evidence_ids.iter().any(|id| {
                evidence_by_id.get(id.as_str()).is_some_and(|evidence| {
                    evidence.visibility != CognitiveEvidenceVisibility::Public
                })
            })
        {
            errors.insert(CognitiveProfileRejection::PrivacyViolation);
        }
    }
    if input.fields.is_empty()
        || input.update_actor.trim().is_empty()
        || input.update_reason.trim().is_empty()
        || unsafe_text(&input.update_actor)
        || unsafe_text(&input.update_reason)
    {
        errors.insert(CognitiveProfileRejection::InvalidUpdate);
    }
    for required in &policy.required_nonclaims {
        if !input.nonclaims.contains(required) {
            errors.insert(CognitiveProfileRejection::MissingNonclaim);
        }
    }
    if input.nonclaims.iter().any(|v| !syntactic_id(v)) {
        errors.insert(CognitiveProfileRejection::ForbiddenInference);
    }
    match previous {
        None => {
            let field_keys: Vec<_> = input.fields.iter().map(|field| field.key.clone()).collect();
            if input.revision != 1
                || input.previous_profile_sha256.is_some()
                || !input.removed_fields.is_empty()
            {
                errors.insert(CognitiveProfileRejection::InvalidRevisionLink);
            }
            if input.added_fields != field_keys {
                errors.insert(CognitiveProfileRejection::UnexplainedMutation);
            }
        }
        Some(prev) => {
            if input.revision != prev.revision + 1
                || input.previous_profile_sha256.as_deref() != Some(&prev.profile_sha256)
                || prev.profile_id != input.profile_id
                || prev.identity_root != identity_root
                || expected_policy_sha256 != Some(prev.policy_sha256.as_str())
            {
                errors.insert(CognitiveProfileRejection::InvalidRevisionLink);
            }
            let old: BTreeSet<_> = prev.fields.iter().map(|f| f.key.clone()).collect();
            let new: BTreeSet<_> = input.fields.iter().map(|f| f.key.clone()).collect();
            let added: Vec<_> = new.difference(&old).cloned().collect();
            let removed: Vec<_> = old.difference(&new).cloned().collect();
            if added != input.added_fields
                || removed != input.removed_fields
                || (prev.fields != input.fields && input.update_reason.trim().len() < 8)
            {
                errors.insert(CognitiveProfileRejection::UnexplainedMutation);
            }
        }
    }
}

fn canonicalize_input(input: &mut CognitiveProfileInput) {
    input.evidence.sort();
    input.fields.iter_mut().for_each(|f| {
        f.evidence_ids.sort();
        f.evidence_ids.dedup();
    });
    input.fields.sort();
    input.fields.dedup();
    for values in [
        &mut input.added_fields,
        &mut input.removed_fields,
        &mut input.nonclaims,
    ] {
        values.sort();
        values.dedup();
    }
}
fn canonicalize_policy(policy: &mut CognitiveProfilePolicy) {
    policy.evidence.sort();
    for field in &mut policy.allowed_fields {
        field.allowed_values.sort();
        field.allowed_values.dedup();
    }
    policy.allowed_fields.sort();
    policy.required_nonclaims.sort();
    policy.required_nonclaims.dedup();
}
fn valid_evidence(e: &CognitiveEvidenceReference) -> bool {
    valid_id(&e.id) && safe_path(&e.path) && is_sha(&e.sha256) && is_sha(&e.revision_sha256)
}
fn safe_path(value: &str) -> bool {
    if value.trim() != value
        || value.is_empty()
        || value.contains(['\\', ':'])
        || value.starts_with('/')
    {
        return false;
    }
    let segments: Vec<_> = value.split('/').collect();
    !segments.iter().any(|s| {
        s.is_empty()
            || *s == "."
            || *s == ".."
            || matches!(
                s.to_ascii_lowercase().as_str(),
                "private" | "home" | "users" | "user"
            )
    }) && Path::new(value)
        .components()
        .all(|c| matches!(c, Component::Normal(_)))
        && !unsafe_text(value)
}
fn syntactic_id(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 256
        && v.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
}
fn valid_id(v: &str) -> bool {
    syntactic_id(v) && !unsafe_text(v)
}
fn unsafe_text(v: &str) -> bool {
    let l = v.to_ascii_lowercase();
    [
        "reputation",
        "standing",
        "rights",
        "personhood",
        "conscious",
        "diagnosis",
        "citizenship",
        "private_state",
        "raw_state",
        "bearer ",
        "api_key",
        "gho_",
        "sk-",
        "/users/",
        "/home/",
        "/private/",
    ]
    .iter()
    .any(|n| l.contains(n))
}
fn is_sha(v: &str) -> bool {
    v.len() == 64
        && v.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn digest<T: Serialize>(v: &T) -> Result<String, Vec<CognitiveProfileRejection>> {
    serde_jcs::to_vec(v)
        .map(|b| format!("{:x}", Sha256::digest(b)))
        .map_err(|_| vec![CognitiveProfileRejection::EncodingFailure])
}
