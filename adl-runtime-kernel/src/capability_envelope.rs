use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    birthday_identity::record_digest as identity_record_digest, candidate_digest, decide_birthday,
    BirthdayCandidate, BirthdayIdentityRecord, VerifiedBirthdayContinuity,
    BIRTHDAY_IDENTITY_RECORD_SCHEMA,
};

pub const CAPABILITY_ENVELOPE_INPUT_SCHEMA: &str = "adl.capability_envelope.input.v1";
pub const CAPABILITY_ENVELOPE_POLICY_SCHEMA: &str = "adl.capability_envelope.policy.v1";
pub const CAPABILITY_ENVELOPE_SCHEMA: &str = "adl.capability_envelope.v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityEvidenceKind {
    Birthday,
    Identity,
    RetainedCapability,
    Provider,
    Model,
    Tool,
    Skill,
    Authority,
    Limit,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEvidenceReference {
    pub id: String,
    pub kind: CapabilityEvidenceKind,
    pub issue: u64,
    pub path: String,
    pub sha256: String,
    pub revision_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderModelSelection {
    pub provider_id: String,
    pub model_id: String,
    pub provenance_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDeclaration {
    pub id: String,
    pub provenance_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityResourceLimits {
    pub max_prompt_tokens: u64,
    pub max_output_tokens: u64,
    pub max_tool_calls: u64,
    pub max_skill_invocations: u64,
    pub timeout_ms: u64,
    pub max_recurrence_depth: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEnvelopeInput {
    pub schema: String,
    pub birthday_candidate_sha256: String,
    pub identity_record_sha256: String,
    pub evidence: Vec<CapabilityEvidenceReference>,
    pub provider_model: ProviderModelSelection,
    pub tools: Vec<CapabilityDeclaration>,
    pub skills: Vec<CapabilityDeclaration>,
    pub grants: Vec<CapabilityDeclaration>,
    pub denials: Vec<CapabilityDeclaration>,
    pub resource_limits: CapabilityResourceLimits,
    pub unsupported_claims: Vec<CapabilityDeclaration>,
}

/// Provisioned trust and authorization boundary for one capability envelope.
///
/// The policy is deliberately separate from the candidate input. A caller may
/// deserialize it from trusted runtime configuration, but changing an envelope
/// cannot change its evidence roots, allowlists, denials, or ceilings.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEnvelopePolicy {
    pub schema: String,
    pub evidence: Vec<CapabilityEvidenceReference>,
    pub provider_models: Vec<ProviderModelSelection>,
    pub allowed_tools: Vec<String>,
    pub allowed_skills: Vec<String>,
    pub allowed_grants: Vec<String>,
    pub required_denials: Vec<String>,
    pub maximum_limits: CapabilityResourceLimits,
    pub required_unsupported_claims: Vec<String>,
}

/// Opaque Runtime-established capability authority.
///
/// Callers may hold this value after Runtime provisioning, but cannot create
/// or rewrite its policy and continuity bindings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityAuthorityPolicy {
    policy_sha256: String,
    continuity_head: String,
    continuity_record_sha256: String,
}

impl CapabilityAuthorityPolicy {
    pub(crate) fn establish(
        policy: &CapabilityEnvelopePolicy,
        continuity: &VerifiedBirthdayContinuity,
    ) -> Result<Self, Vec<CapabilityEnvelopeRejection>> {
        let canonical = canonical_policy(policy);
        let mut errors = BTreeSet::new();
        validate_policy(&canonical, &mut errors);
        if !errors.is_empty() {
            return Err(errors.into_iter().collect());
        }
        Ok(Self {
            policy_sha256: digest(&canonical)?,
            continuity_head: continuity.continuity_head().to_owned(),
            continuity_record_sha256: continuity.record().record_sha256.clone(),
        })
    }
}

/// Unforgeable provisioning capability owned by a live Runtime assembly.
///
/// ```compile_fail
/// use adl_runtime_kernel::RuntimeCapabilityProvisioner;
/// let _forged = RuntimeCapabilityProvisioner {};
/// ```
///
/// ```compile_fail
/// use adl_runtime_kernel::RuntimeCapabilityProvisioner;
/// let _forged = RuntimeCapabilityProvisioner::new();
/// ```
#[derive(Clone, Debug)]
pub struct RuntimeCapabilityProvisioner {
    _private: (),
}

impl RuntimeCapabilityProvisioner {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }

    pub(crate) fn provision(
        &self,
        provisioned_policy: &CapabilityEnvelopePolicy,
        verified_continuity: &VerifiedBirthdayContinuity,
    ) -> Result<CapabilityAuthorityPolicy, Vec<CapabilityEnvelopeRejection>> {
        CapabilityAuthorityPolicy::establish(provisioned_policy, verified_continuity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityEnvelope {
    pub schema: String,
    pub identity_root: String,
    pub continuity_head: String,
    pub continuity_record_sha256: String,
    pub birthday_candidate_sha256: String,
    pub identity_record_sha256: String,
    pub evidence: Vec<CapabilityEvidenceReference>,
    pub provider_model: ProviderModelSelection,
    pub tools: Vec<CapabilityDeclaration>,
    pub skills: Vec<CapabilityDeclaration>,
    pub grants: Vec<CapabilityDeclaration>,
    pub denials: Vec<CapabilityDeclaration>,
    pub resource_limits: CapabilityResourceLimits,
    pub unsupported_claims: Vec<CapabilityDeclaration>,
    pub policy_sha256: String,
    pub canonical_input_sha256: String,
    pub envelope_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum CapabilityEnvelopeRejection {
    UnsupportedInputSchema,
    UnsupportedPolicySchema,
    BirthdayEvidenceInvalid,
    BirthdayEvidenceMismatch,
    IdentityEvidenceInvalid,
    IdentityEvidenceMismatch,
    IdentityRootMismatch,
    ContinuityBindingMismatch,
    MissingEvidence { kind: CapabilityEvidenceKind },
    UnknownEvidence { fingerprint: String },
    StaleEvidence { fingerprint: String },
    InvalidEvidence { fingerprint: String },
    DuplicateEvidence { fingerprint: String },
    UnknownProvider { fingerprint: String },
    UnknownModel { fingerprint: String },
    UnsupportedTool { fingerprint: String },
    UnsupportedSkill { fingerprint: String },
    AuthorityEscalation { fingerprint: String },
    MissingDenial { fingerprint: String },
    GrantDenialConflict { fingerprint: String },
    MissingLimits,
    LimitEscalation { fingerprint: String },
    MissingUnsupportedClaim { fingerprint: String },
    InvalidDeclaration { fingerprint: String },
    MissingProvenance { fingerprint: String },
    IdentifierCollision { fingerprint: String },
    UnsafeContent { fingerprint: String },
    NonCanonicalEnvelope,
    EnvelopeDigestMismatch,
    EncodingFailure,
}

pub(crate) fn build_capability_envelope(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    input: &CapabilityEnvelopeInput,
    policy: &CapabilityEnvelopePolicy,
) -> Result<CapabilityEnvelope, Vec<CapabilityEnvelopeRejection>> {
    let mut errors = BTreeSet::new();
    let canonical_policy = canonical_policy(policy);
    validate_authorities(birthday, identity, input, &mut errors);
    validate_policy(&canonical_policy, &mut errors);

    let mut canonical = input.clone();
    canonicalize_input(&mut canonical);
    validate_evidence(&canonical, &canonical_policy, &mut errors);
    validate_selection(&canonical, &canonical_policy, &mut errors);
    validate_limits(
        &canonical.resource_limits,
        &canonical_policy.maximum_limits,
        &mut errors,
    );

    if !errors.is_empty() {
        return Err(errors.into_iter().collect());
    }

    let policy_sha256 = digest(&canonical_policy)?;
    let canonical_input_sha256 = digest(&canonical)?;
    let mut envelope = CapabilityEnvelope {
        schema: CAPABILITY_ENVELOPE_SCHEMA.to_owned(),
        identity_root: identity.identity_root.clone(),
        continuity_head: String::new(),
        continuity_record_sha256: String::new(),
        birthday_candidate_sha256: canonical.birthday_candidate_sha256,
        identity_record_sha256: canonical.identity_record_sha256,
        evidence: canonical.evidence,
        provider_model: canonical.provider_model,
        tools: canonical.tools,
        skills: canonical.skills,
        grants: canonical.grants,
        denials: canonical.denials,
        resource_limits: canonical.resource_limits,
        unsupported_claims: canonical.unsupported_claims,
        policy_sha256,
        canonical_input_sha256,
        envelope_sha256: String::new(),
    };
    envelope.envelope_sha256 = envelope_digest(&envelope)?;
    Ok(envelope)
}

/// Builds the public capability envelope only from opaque verified continuity.
///
/// The former raw-record API is not public:
/// ```compile_fail
/// use adl_runtime_kernel::build_capability_envelope;
/// ```
pub fn build_capability_envelope_with_continuity(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
    authority: &CapabilityAuthorityPolicy,
    input: &CapabilityEnvelopeInput,
    policy: &CapabilityEnvelopePolicy,
) -> Result<CapabilityEnvelope, Vec<CapabilityEnvelopeRejection>> {
    validate_verified_continuity(birthday, identity, continuity)?;
    validate_capability_authority(authority, continuity, policy)?;
    let mut envelope = build_capability_envelope(birthday, identity, input, policy)?;
    envelope.continuity_head = continuity.continuity_head().to_owned();
    envelope.continuity_record_sha256 = continuity.record().record_sha256.clone();
    envelope.envelope_sha256 = envelope_digest(&envelope)?;
    Ok(envelope)
}

/// Revalidates every exported field against the original authorities and
/// provisioned policy. Re-hashing a forged packet is insufficient to pass.
pub(crate) fn validate_capability_envelope(
    envelope: &CapabilityEnvelope,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    policy: &CapabilityEnvelopePolicy,
) -> Result<(), Vec<CapabilityEnvelopeRejection>> {
    let mut errors = BTreeSet::new();
    if envelope.schema != CAPABILITY_ENVELOPE_SCHEMA {
        errors.insert(CapabilityEnvelopeRejection::NonCanonicalEnvelope);
    }
    if envelope_digest(envelope).ok().as_deref() != Some(&envelope.envelope_sha256) {
        errors.insert(CapabilityEnvelopeRejection::EnvelopeDigestMismatch);
    }
    let input = CapabilityEnvelopeInput {
        schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.to_owned(),
        birthday_candidate_sha256: envelope.birthday_candidate_sha256.clone(),
        identity_record_sha256: envelope.identity_record_sha256.clone(),
        evidence: envelope.evidence.clone(),
        provider_model: envelope.provider_model.clone(),
        tools: envelope.tools.clone(),
        skills: envelope.skills.clone(),
        grants: envelope.grants.clone(),
        denials: envelope.denials.clone(),
        resource_limits: envelope.resource_limits.clone(),
        unsupported_claims: envelope.unsupported_claims.clone(),
    };
    match build_capability_envelope(birthday, identity, &input, policy) {
        Ok(mut expected) => {
            expected.continuity_head = envelope.continuity_head.clone();
            expected.continuity_record_sha256 = envelope.continuity_record_sha256.clone();
            expected.envelope_sha256 = envelope_digest(&expected)?;
            if &expected != envelope {
                errors.insert(CapabilityEnvelopeRejection::NonCanonicalEnvelope);
            }
        }
        Err(found) => errors.extend(found),
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().collect())
    }
}

/// Revalidates a public envelope against the same opaque continuity authority.
///
/// ```compile_fail
/// use adl_runtime_kernel::validate_capability_envelope;
/// ```
pub fn validate_capability_envelope_with_continuity(
    envelope: &CapabilityEnvelope,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
    authority: &CapabilityAuthorityPolicy,
    policy: &CapabilityEnvelopePolicy,
) -> Result<(), Vec<CapabilityEnvelopeRejection>> {
    validate_verified_continuity(birthday, identity, continuity)?;
    validate_capability_authority(authority, continuity, policy)?;
    if envelope.continuity_head != continuity.continuity_head()
        || envelope.continuity_record_sha256 != continuity.record().record_sha256
    {
        return Err(vec![CapabilityEnvelopeRejection::ContinuityBindingMismatch]);
    }
    validate_capability_envelope(envelope, birthday, identity, policy)
}

fn validate_capability_authority(
    authority: &CapabilityAuthorityPolicy,
    continuity: &VerifiedBirthdayContinuity,
    policy: &CapabilityEnvelopePolicy,
) -> Result<(), Vec<CapabilityEnvelopeRejection>> {
    let policy_sha256 = digest(&canonical_policy(policy))?;
    if authority.policy_sha256 != policy_sha256
        || authority.continuity_head != continuity.continuity_head()
        || authority.continuity_record_sha256 != continuity.record().record_sha256
    {
        return Err(vec![CapabilityEnvelopeRejection::ContinuityBindingMismatch]);
    }
    Ok(())
}

fn validate_verified_continuity(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
) -> Result<(), Vec<CapabilityEnvelopeRejection>> {
    let record = continuity.record();
    if birthday.identity_root != record.identity_root
        || identity.identity_root != record.identity_root
        || identity.record_sha256 != record.identity_record_sha256
        || birthday.continuity_head != continuity.identity_checkpoint_head()
        || identity.continuity.head_sha256 != continuity.identity_checkpoint_head()
    {
        return Err(vec![CapabilityEnvelopeRejection::ContinuityBindingMismatch]);
    }
    Ok(())
}

pub fn envelope_digest(
    envelope: &CapabilityEnvelope,
) -> Result<String, Vec<CapabilityEnvelopeRejection>> {
    let mut unsigned = envelope.clone();
    unsigned.envelope_sha256.clear();
    digest(&unsigned)
}

fn validate_authorities(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    input: &CapabilityEnvelopeInput,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    if input.schema != CAPABILITY_ENVELOPE_INPUT_SCHEMA {
        errors.insert(CapabilityEnvelopeRejection::UnsupportedInputSchema);
    }
    let birthday_digest = candidate_digest(birthday).ok();
    if !decide_birthday(birthday).accepted
        || birthday_digest.as_deref() != Some(&birthday.packet_sha256)
    {
        errors.insert(CapabilityEnvelopeRejection::BirthdayEvidenceInvalid);
    }
    if input.birthday_candidate_sha256 != birthday.packet_sha256 {
        errors.insert(CapabilityEnvelopeRejection::BirthdayEvidenceMismatch);
    }
    if !input.evidence.iter().any(|reference| {
        reference.kind == CapabilityEvidenceKind::Birthday
            && reference.issue == 5825
            && reference.sha256 == birthday.packet_sha256
    }) {
        errors.insert(CapabilityEnvelopeRejection::BirthdayEvidenceMismatch);
    }
    if identity.schema != BIRTHDAY_IDENTITY_RECORD_SCHEMA
        || identity_record_digest(identity).ok().as_deref() != Some(&identity.record_sha256)
    {
        errors.insert(CapabilityEnvelopeRejection::IdentityEvidenceInvalid);
    }
    if input.identity_record_sha256 != identity.record_sha256 {
        errors.insert(CapabilityEnvelopeRejection::IdentityEvidenceMismatch);
    }
    if !input.evidence.iter().any(|reference| {
        reference.kind == CapabilityEvidenceKind::Identity
            && reference.issue == 5826
            && reference.sha256 == identity.record_sha256
    }) {
        errors.insert(CapabilityEnvelopeRejection::IdentityEvidenceMismatch);
    }
    if birthday.identity_root != identity.identity_root
        || birthday.stable_name != identity.stable_name
        || birthday.continuity_head != identity.continuity.head_sha256
    {
        errors.insert(CapabilityEnvelopeRejection::IdentityRootMismatch);
    }
}

fn validate_policy(
    policy: &CapabilityEnvelopePolicy,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    if policy.schema != CAPABILITY_ENVELOPE_POLICY_SCHEMA {
        errors.insert(CapabilityEnvelopeRejection::UnsupportedPolicySchema);
    }
    for reference in &policy.evidence {
        if !valid_evidence(reference) {
            errors.insert(CapabilityEnvelopeRejection::InvalidEvidence {
                fingerprint: safe_fingerprint(&reference.id),
            });
        }
    }
    let mut evidence_ids = BTreeMap::<String, &str>::new();
    for reference in &policy.evidence {
        let key = reference.id.to_ascii_lowercase();
        if let Some(previous) = evidence_ids.insert(key, reference.id.as_str()) {
            if previous != reference.id {
                errors.insert(CapabilityEnvelopeRejection::IdentifierCollision {
                    fingerprint: safe_fingerprint(&reference.id),
                });
            }
        }
    }
    if !policy
        .evidence
        .iter()
        .any(|e| e.kind == CapabilityEvidenceKind::Birthday && e.issue == 5825)
    {
        errors.insert(CapabilityEnvelopeRejection::MissingEvidence {
            kind: CapabilityEvidenceKind::Birthday,
        });
    }
    if !policy
        .evidence
        .iter()
        .any(|e| e.kind == CapabilityEvidenceKind::Identity && e.issue == 5826)
    {
        errors.insert(CapabilityEnvelopeRejection::MissingEvidence {
            kind: CapabilityEvidenceKind::Identity,
        });
    }
    if !policy
        .evidence
        .iter()
        .any(|e| e.kind == CapabilityEvidenceKind::RetainedCapability && e.issue == 4761)
    {
        errors.insert(CapabilityEnvelopeRejection::MissingEvidence {
            kind: CapabilityEvidenceKind::RetainedCapability,
        });
    }
    validate_identifier_collection(policy.allowed_tools.iter(), errors);
    validate_identifier_collection(policy.allowed_skills.iter(), errors);
    validate_identifier_collection(policy.allowed_grants.iter(), errors);
    validate_identifier_collection(policy.required_denials.iter(), errors);
    validate_identifier_collection(policy.required_unsupported_claims.iter(), errors);
    let mut provider_ids = BTreeMap::<String, &str>::new();
    let mut model_ids = BTreeMap::<(String, String), &str>::new();
    for selection in &policy.provider_models {
        if !valid_identifier(&selection.provider_id)
            || !valid_identifier(&selection.model_id)
            || selection.provenance_ids.is_empty()
            || selection
                .provenance_ids
                .iter()
                .any(|id| !valid_identifier(id))
        {
            errors.insert(CapabilityEnvelopeRejection::InvalidDeclaration {
                fingerprint: safe_fingerprint(&format!(
                    "{}/{}",
                    selection.provider_id, selection.model_id
                )),
            });
        }
        let provider_key = selection.provider_id.to_ascii_lowercase();
        if let Some(previous) =
            provider_ids.insert(provider_key.clone(), selection.provider_id.as_str())
        {
            if previous != selection.provider_id {
                errors.insert(CapabilityEnvelopeRejection::IdentifierCollision {
                    fingerprint: safe_fingerprint(&selection.provider_id),
                });
            }
        }
        let model_key = (provider_key, selection.model_id.to_ascii_lowercase());
        if let Some(previous) = model_ids.insert(model_key, selection.model_id.as_str()) {
            if previous != selection.model_id {
                errors.insert(CapabilityEnvelopeRejection::IdentifierCollision {
                    fingerprint: safe_fingerprint(&selection.model_id),
                });
            }
        }
    }
    if [
        policy.maximum_limits.max_prompt_tokens,
        policy.maximum_limits.max_output_tokens,
        policy.maximum_limits.max_tool_calls,
        policy.maximum_limits.max_skill_invocations,
        policy.maximum_limits.timeout_ms,
        policy.maximum_limits.max_recurrence_depth,
    ]
    .contains(&0)
    {
        errors.insert(CapabilityEnvelopeRejection::MissingLimits);
    }
}

fn validate_evidence(
    input: &CapabilityEnvelopeInput,
    policy: &CapabilityEnvelopePolicy,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    let expected: BTreeMap<_, _> = policy.evidence.iter().map(|e| (e.id.as_str(), e)).collect();
    let mut seen = BTreeSet::new();
    for reference in &input.evidence {
        if !seen.insert(reference.id.as_str()) {
            errors.insert(CapabilityEnvelopeRejection::DuplicateEvidence {
                fingerprint: safe_fingerprint(&reference.id),
            });
        }
        if !valid_evidence(reference) {
            errors.insert(CapabilityEnvelopeRejection::InvalidEvidence {
                fingerprint: safe_fingerprint(&reference.id),
            });
            continue;
        }
        match expected.get(reference.id.as_str()) {
            None => {
                errors.insert(CapabilityEnvelopeRejection::UnknownEvidence {
                    fingerprint: safe_fingerprint(&reference.id),
                });
            }
            Some(approved) if *approved != reference => {
                errors.insert(CapabilityEnvelopeRejection::StaleEvidence {
                    fingerprint: safe_fingerprint(&reference.id),
                });
            }
            Some(_) => {}
        }
    }
    for reference in &policy.evidence {
        if !seen.contains(reference.id.as_str()) {
            errors.insert(CapabilityEnvelopeRejection::MissingEvidence {
                kind: reference.kind,
            });
        }
    }
}

fn validate_selection(
    input: &CapabilityEnvelopeInput,
    policy: &CapabilityEnvelopePolicy,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    let known_provider = policy
        .provider_models
        .iter()
        .any(|allowed| allowed.provider_id == input.provider_model.provider_id);
    if !known_provider {
        errors.insert(CapabilityEnvelopeRejection::UnknownProvider {
            fingerprint: safe_fingerprint(&input.provider_model.provider_id),
        });
    } else if !policy.provider_models.iter().any(|allowed| {
        allowed.provider_id == input.provider_model.provider_id
            && allowed.model_id == input.provider_model.model_id
            && allowed.provenance_ids == input.provider_model.provenance_ids
    }) {
        errors.insert(CapabilityEnvelopeRejection::UnknownModel {
            fingerprint: safe_fingerprint(&input.provider_model.model_id),
        });
    }
    validate_declarations(&input.tools, &policy.allowed_tools, "tool", errors);
    validate_declarations(&input.skills, &policy.allowed_skills, "skill", errors);
    validate_declarations(&input.grants, &policy.allowed_grants, "grant", errors);
    validate_declarations(&input.denials, &policy.required_denials, "denial", errors);
    validate_declarations(
        &input.unsupported_claims,
        &policy.required_unsupported_claims,
        "unsupported",
        errors,
    );

    let denials: BTreeSet<_> = input.denials.iter().map(|d| d.id.as_str()).collect();
    for grant in &input.grants {
        if denials.contains(grant.id.as_str()) {
            errors.insert(CapabilityEnvelopeRejection::GrantDenialConflict {
                fingerprint: safe_fingerprint(&grant.id),
            });
        }
    }
    for required in &policy.required_denials {
        if !denials.contains(required.as_str()) {
            errors.insert(CapabilityEnvelopeRejection::MissingDenial {
                fingerprint: safe_fingerprint(required),
            });
        }
    }
    let unsupported: BTreeSet<_> = input
        .unsupported_claims
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    for required in &policy.required_unsupported_claims {
        if !unsupported.contains(required.as_str()) {
            errors.insert(CapabilityEnvelopeRejection::MissingUnsupportedClaim {
                fingerprint: safe_fingerprint(required),
            });
        }
    }

    let evidence_ids: BTreeSet<_> = input.evidence.iter().map(|e| e.id.as_str()).collect();
    for declaration in std::iter::once(&input.provider_model.provenance_ids)
        .chain(input.tools.iter().map(|d| &d.provenance_ids))
        .chain(input.skills.iter().map(|d| &d.provenance_ids))
        .chain(input.grants.iter().map(|d| &d.provenance_ids))
        .chain(input.denials.iter().map(|d| &d.provenance_ids))
        .chain(input.unsupported_claims.iter().map(|d| &d.provenance_ids))
    {
        if declaration.is_empty() {
            errors.insert(CapabilityEnvelopeRejection::MissingProvenance {
                fingerprint: safe_fingerprint("empty"),
            });
        }
        for id in declaration {
            if !evidence_ids.contains(id.as_str()) {
                errors.insert(CapabilityEnvelopeRejection::MissingProvenance {
                    fingerprint: safe_fingerprint(id),
                });
            }
        }
    }
}

fn validate_declarations(
    declarations: &[CapabilityDeclaration],
    allowed: &[String],
    kind: &str,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    let allowed: BTreeSet<_> = allowed.iter().map(String::as_str).collect();
    let mut folded = BTreeMap::<String, &str>::new();
    for declaration in declarations {
        if !valid_identifier(&declaration.id)
            || declaration
                .provenance_ids
                .iter()
                .any(|id| !valid_identifier(id))
        {
            errors.insert(CapabilityEnvelopeRejection::InvalidDeclaration {
                fingerprint: safe_fingerprint(&declaration.id),
            });
        }
        let key = declaration.id.to_ascii_lowercase();
        if let Some(previous) = folded.insert(key, declaration.id.as_str()) {
            if previous != declaration.id {
                errors.insert(CapabilityEnvelopeRejection::IdentifierCollision {
                    fingerprint: safe_fingerprint(&declaration.id),
                });
            }
        }
        if !allowed.contains(declaration.id.as_str()) {
            let rejection = match kind {
                "tool" => CapabilityEnvelopeRejection::UnsupportedTool {
                    fingerprint: safe_fingerprint(&declaration.id),
                },
                "skill" => CapabilityEnvelopeRejection::UnsupportedSkill {
                    fingerprint: safe_fingerprint(&declaration.id),
                },
                "grant" => CapabilityEnvelopeRejection::AuthorityEscalation {
                    fingerprint: safe_fingerprint(&declaration.id),
                },
                "denial" => CapabilityEnvelopeRejection::InvalidDeclaration {
                    fingerprint: safe_fingerprint(&declaration.id),
                },
                _ => CapabilityEnvelopeRejection::InvalidDeclaration {
                    fingerprint: safe_fingerprint(&declaration.id),
                },
            };
            errors.insert(rejection);
        }
    }
}

fn validate_limits(
    limits: &CapabilityResourceLimits,
    maximum: &CapabilityResourceLimits,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    let values = [
        (
            "max_prompt_tokens",
            limits.max_prompt_tokens,
            maximum.max_prompt_tokens,
        ),
        (
            "max_output_tokens",
            limits.max_output_tokens,
            maximum.max_output_tokens,
        ),
        (
            "max_tool_calls",
            limits.max_tool_calls,
            maximum.max_tool_calls,
        ),
        (
            "max_skill_invocations",
            limits.max_skill_invocations,
            maximum.max_skill_invocations,
        ),
        ("timeout_ms", limits.timeout_ms, maximum.timeout_ms),
        (
            "max_recurrence_depth",
            limits.max_recurrence_depth,
            maximum.max_recurrence_depth,
        ),
    ];
    if values.iter().any(|(_, value, _)| *value == 0) {
        errors.insert(CapabilityEnvelopeRejection::MissingLimits);
    }
    for (id, value, ceiling) in values {
        if value > ceiling {
            errors.insert(CapabilityEnvelopeRejection::LimitEscalation {
                fingerprint: safe_fingerprint(id),
            });
        }
    }
}

fn validate_identifier_collection<'a>(
    values: impl Iterator<Item = &'a String>,
    errors: &mut BTreeSet<CapabilityEnvelopeRejection>,
) {
    let mut folded = BTreeMap::<String, &str>::new();
    for value in values {
        if !valid_identifier(value) {
            errors.insert(CapabilityEnvelopeRejection::UnsafeContent {
                fingerprint: safe_fingerprint(value),
            });
        }
        let key = value.to_ascii_lowercase();
        if let Some(previous) = folded.insert(key, value) {
            if previous != value {
                errors.insert(CapabilityEnvelopeRejection::IdentifierCollision {
                    fingerprint: safe_fingerprint(value),
                });
            }
        }
    }
}

fn canonicalize_input(input: &mut CapabilityEnvelopeInput) {
    input.evidence.sort();
    canonicalize_ids(&mut input.provider_model.provenance_ids);
    for declarations in [
        &mut input.tools,
        &mut input.skills,
        &mut input.grants,
        &mut input.denials,
        &mut input.unsupported_claims,
    ] {
        for declaration in declarations.iter_mut() {
            canonicalize_ids(&mut declaration.provenance_ids);
        }
        declarations.sort();
        declarations.dedup();
    }
}

fn canonical_policy(policy: &CapabilityEnvelopePolicy) -> CapabilityEnvelopePolicy {
    let mut policy = policy.clone();
    policy.evidence.sort();
    for selection in &mut policy.provider_models {
        canonicalize_ids(&mut selection.provenance_ids);
    }
    policy.provider_models.sort();
    policy.provider_models.dedup();
    for values in [
        &mut policy.allowed_tools,
        &mut policy.allowed_skills,
        &mut policy.allowed_grants,
        &mut policy.required_denials,
        &mut policy.required_unsupported_claims,
    ] {
        canonicalize_ids(values);
    }
    policy
}

fn canonicalize_ids(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}

fn valid_evidence(reference: &CapabilityEvidenceReference) -> bool {
    valid_identifier(&reference.id)
        && reference.issue > 0
        && safe_repo_path(&reference.path)
        && is_sha256(&reference.sha256)
        && is_sha256(&reference.revision_sha256)
}

fn safe_repo_path(value: &str) -> bool {
    if value.trim().is_empty()
        || value.trim() != value
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.contains(':')
        || unsafe_content(value)
    {
        return false;
    }
    let segments: Vec<_> = value.split('/').collect();
    if segments.iter().any(|segment| {
        segment.is_empty() || *segment == "." || *segment == ".." || segment.starts_with('~')
    }) {
        return false;
    }
    if segments.first().is_some_and(|segment| {
        matches!(
            segment.to_ascii_lowercase().as_str(),
            "private" | "home" | "users" | "user"
        )
    }) {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn safe_fingerprint(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 256
        && !unsafe_content(value)
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
}

fn unsafe_content(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "/users/",
        "/home/",
        "/private/",
        "private_state",
        "raw-state",
        "raw_state",
        "sealed_payload",
        "bearer ",
        "gho_",
        "github_pat_",
        "sk-",
        "api_key",
        "apikey",
        "private key",
        "secret=",
        "token=",
        "raw_chat_transcript",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

#[cfg(test)]
#[path = "../tests/fixtures/capability_envelope/authority_tests.rs"]
mod authority_tests;

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn digest<T: Serialize>(value: &T) -> Result<String, Vec<CapabilityEnvelopeRejection>> {
    serde_jcs::to_vec(value)
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| vec![CapabilityEnvelopeRejection::EncodingFailure])
}
