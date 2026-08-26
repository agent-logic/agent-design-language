use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    birthday_identity::record_digest as identity_digest, candidate_digest,
    continuity_record_digest, decide_birthday, validate_capability_envelope,
    validate_capability_envelope_with_continuity, BirthdayCandidate, BirthdayContinuityRecord,
    BirthdayIdentityRecord, CapabilityAuthorityPolicy, CapabilityEnvelope,
    CapabilityEnvelopePolicy, VerifiedBirthdayContinuity, BIRTHDAY_IDENTITY_RECORD_SCHEMA,
};

pub const COGNITIVE_PROFILE_INPUT_SCHEMA: &str = "adl.cognitive_profile.input.v1";
pub const COGNITIVE_PROFILE_POLICY_SCHEMA: &str = "adl.cognitive_profile.policy.v1";
pub const COGNITIVE_PROFILE_SCHEMA: &str = "adl.cognitive_profile.v1";
pub const COGNITIVE_PROFILE_PUBLIC_SCHEMA: &str = "adl.cognitive_profile.public.v1";
pub const COGNITIVE_AUTHORITY_STATEMENT_SCHEMA: &str =
    "adl.cognitive_profile.authority_statement.v1";
pub const COGNITIVE_AUTHORITY_ROTATION_SCHEMA: &str = "adl.cognitive_profile.authority_rotation.v1";

#[derive(Clone)]
struct CognitiveAuthorityRegistry {
    genesis: ActiveCognitiveAuthority,
}

/// Opaque runtime-established authority and input-policy capability.
///
/// Neither the trusted signing root nor the authorized policy/evidence digests
/// can be nominated by an external caller:
///
/// ```compile_fail
/// use adl_runtime_kernel::CognitiveAuthorityPolicy;
/// let _attacker_policy_factory = CognitiveAuthorityPolicy::establish;
/// ```
#[derive(Clone)]
pub struct CognitiveAuthorityPolicy {
    registry: CognitiveAuthorityRegistry,
    policy_sha256: String,
    evidence_sha256: String,
}

impl CognitiveAuthorityPolicy {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn establish(
        authority_id: String,
        key_id: String,
        epoch: u64,
        verifying_key: VerifyingKey,
        policy_sha256: String,
        evidence_sha256: String,
    ) -> Result<Self, Vec<CognitiveProfileRejection>> {
        let mut context = CognitiveAuthorityContext {
            authority_id,
            key_id,
            epoch,
            context_sha256: String::new(),
            verifying_key_hex: hex::encode(verifying_key.as_bytes()),
        };
        context.context_sha256 = authority_context_payload_digest(&context)?;
        if !valid_authority_context(&context) {
            return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
        }
        if !is_sha(&policy_sha256) || !is_sha(&evidence_sha256) {
            return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
        }
        Ok(Self {
            registry: CognitiveAuthorityRegistry {
                genesis: ActiveCognitiveAuthority {
                    context,
                    verifying_key,
                },
            },
            policy_sha256,
            evidence_sha256,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveAuthorityContext {
    pub authority_id: String,
    pub key_id: String,
    pub epoch: u64,
    pub context_sha256: String,
    pub verifying_key_hex: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveAuthorityStatement {
    pub schema: String,
    pub authority_context_sha256: String,
    pub profile_id: String,
    pub revision: u64,
    pub previous_profile_sha256: Option<String>,
    pub canonical_input_sha256: String,
    pub policy_sha256: String,
    pub evidence_sha256: String,
    pub signature: String,
}

impl CognitiveAuthorityStatement {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, Vec<CognitiveProfileRejection>> {
        self.signature.clear();
        self.signature = sign_canonical(&self, key)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveAuthorityRotation {
    pub schema: String,
    pub from_authority_context_sha256: String,
    pub to: CognitiveAuthorityContext,
    pub profile_id: String,
    pub revision: u64,
    pub signature: String,
}

impl CognitiveAuthorityRotation {
    pub fn sign(mut self, key: &SigningKey) -> Result<Self, Vec<CognitiveProfileRejection>> {
        self.signature.clear();
        self.signature = sign_canonical(&self, key)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CognitiveAuthorityProof {
    pub context: CognitiveAuthorityContext,
    pub statement: CognitiveAuthorityStatement,
    pub rotation: Option<CognitiveAuthorityRotation>,
}

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
    pub authority: CognitiveAuthorityProof,
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
    _birthday: &BirthdayCandidate,
    _identity: &BirthdayIdentityRecord,
    _continuity: &BirthdayContinuityRecord,
    _capability: &CapabilityEnvelope,
    _input: &CognitiveProfileInput,
    _policy: &CognitiveProfilePolicy,
    _previous: Option<&CognitiveProfile>,
) -> Result<CognitiveProfile, Vec<CognitiveProfileRejection>> {
    Err(vec![CognitiveProfileRejection::InvalidAuthority])
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_governed_cognitive_profile(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    complete_history: &[CognitiveProfile],
    authority_policy: &CognitiveAuthorityPolicy,
    proof: &CognitiveAuthorityProof,
) -> Result<CognitiveProfile, Vec<CognitiveProfileRejection>> {
    if complete_history.len() as u64 != input.revision.saturating_sub(1) || input.revision == 0 {
        return Err(vec![CognitiveProfileRejection::InvalidRevisionLink]);
    }
    let active = validate_complete_authority_history(
        complete_history,
        birthday,
        identity,
        continuity,
        capability,
        policy,
        authority_policy,
    )?;
    validate_authority_policy(input, policy, authority_policy)?;
    if let Some(rotation) = &proof.rotation {
        let genesis = &authority_policy.registry.genesis.context;
        if std::iter::once(genesis)
            .chain(
                complete_history
                    .iter()
                    .map(|profile| &profile.authority.context),
            )
            .any(|context| {
                context.key_id == rotation.to.key_id
                    || context.verifying_key_hex == rotation.to.verifying_key_hex
                    || digest(context).ok() == digest(&rotation.to).ok()
            })
        {
            return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
        }
    }
    verify_authority_proof(input, policy, &active, proof)?;
    build_cognitive_profile_payload(
        birthday,
        identity,
        continuity,
        capability,
        input,
        policy,
        complete_history.last(),
        proof,
    )
}

#[allow(clippy::too_many_arguments)]
/// Builds governed cognition only after opaque Runtime continuity verification.
///
/// ```compile_fail
/// use adl_runtime_kernel::build_governed_cognitive_profile;
/// ```
pub fn build_governed_cognitive_profile_with_continuity(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
    capability_authority: &CapabilityAuthorityPolicy,
    capability: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    complete_history: &[CognitiveProfile],
    authority_policy: &CognitiveAuthorityPolicy,
    proof: &CognitiveAuthorityProof,
) -> Result<CognitiveProfile, Vec<CognitiveProfileRejection>> {
    validate_verified_continuity(identity, continuity)?;
    validate_capability_envelope_with_continuity(
        capability,
        birthday,
        identity,
        continuity,
        capability_authority,
        &policy.capability_policy,
    )
    .map_err(|_| vec![CognitiveProfileRejection::CapabilityMismatch])?;
    build_governed_cognitive_profile(
        birthday,
        identity,
        continuity.record(),
        capability,
        input,
        policy,
        complete_history,
        authority_policy,
        proof,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_cognitive_profile_payload(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
    authority: &CognitiveAuthorityProof,
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
        &continuity.continuity_head,
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
        authority: authority.clone(),
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
    _profile: &CognitiveProfile,
    _birthday: &BirthdayCandidate,
    _identity: &BirthdayIdentityRecord,
    _continuity: &BirthdayContinuityRecord,
    _capability: &CapabilityEnvelope,
    _policy: &CognitiveProfilePolicy,
    _previous: Option<&CognitiveProfile>,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    Err(vec![CognitiveProfileRejection::InvalidAuthority])
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn validate_governed_cognitive_profile(
    profile: &CognitiveProfile,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    complete_history: &[CognitiveProfile],
    authority_policy: &CognitiveAuthorityPolicy,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    let input = input_from_profile(profile);
    match build_governed_cognitive_profile(
        birthday,
        identity,
        continuity,
        capability,
        &input,
        policy,
        complete_history,
        authority_policy,
        &profile.authority,
    ) {
        Ok(expected) if &expected == profile => Ok(()),
        Ok(_) => Err(vec![CognitiveProfileRejection::NonCanonicalProfile]),
        Err(errors) => Err(errors),
    }
}

#[allow(clippy::too_many_arguments)]
/// Revalidates governed cognition against the same opaque continuity token.
///
/// ```compile_fail
/// use adl_runtime_kernel::validate_governed_cognitive_profile;
/// ```
pub fn validate_governed_cognitive_profile_with_continuity(
    profile: &CognitiveProfile,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
    capability_authority: &CapabilityAuthorityPolicy,
    capability: &CapabilityEnvelope,
    cognitive_policy: &CognitiveProfilePolicy,
    complete_history: &[CognitiveProfile],
    authority_policy: &CognitiveAuthorityPolicy,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    validate_verified_continuity(identity, continuity)?;
    validate_capability_envelope_with_continuity(
        capability,
        birthday,
        identity,
        continuity,
        capability_authority,
        &cognitive_policy.capability_policy,
    )
    .map_err(|_| vec![CognitiveProfileRejection::CapabilityMismatch])?;
    validate_governed_cognitive_profile(
        profile,
        birthday,
        identity,
        continuity.record(),
        capability,
        cognitive_policy,
        complete_history,
        authority_policy,
    )
}

fn validate_verified_continuity(
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    let record = continuity.record();
    if record.identity_root != identity.identity_root
        || record.identity_record_sha256 != identity.record_sha256
        || record.predecessor_head != continuity.identity_checkpoint_head()
    {
        return Err(vec![CognitiveProfileRejection::ContinuityMismatch]);
    }
    Ok(())
}

#[derive(Clone)]
struct ActiveCognitiveAuthority {
    context: CognitiveAuthorityContext,
    verifying_key: VerifyingKey,
}

#[allow(clippy::too_many_arguments)]
fn validate_complete_authority_history(
    history: &[CognitiveProfile],
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    capability: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    authority_policy: &CognitiveAuthorityPolicy,
) -> Result<ActiveCognitiveAuthority, Vec<CognitiveProfileRejection>> {
    let mut active = authority_policy.registry.genesis.clone();
    let mut used_key_ids = BTreeSet::from([active.context.key_id.clone()]);
    let mut used_keys = BTreeSet::from([active.context.verifying_key_hex.clone()]);
    let mut used_contexts = BTreeSet::from([digest(&active.context)?]);
    let mut previous: Option<&CognitiveProfile> = None;
    for (index, retained) in history.iter().enumerate() {
        if retained.revision != (index as u64) + 1 {
            return Err(vec![CognitiveProfileRejection::InvalidRevisionLink]);
        }
        let retained_input = input_from_profile(retained);
        validate_authority_policy(&retained_input, policy, authority_policy)?;
        active = verify_authority_proof(&retained_input, policy, &active, &retained.authority)?;
        if retained.authority.rotation.is_some()
            && (!used_key_ids.insert(active.context.key_id.clone())
                || !used_keys.insert(active.context.verifying_key_hex.clone())
                || !used_contexts.insert(digest(&active.context)?))
        {
            return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
        }
        let rebuilt = build_cognitive_profile_payload(
            birthday,
            identity,
            continuity,
            capability,
            &retained_input,
            policy,
            previous,
            &retained.authority,
        )?;
        if rebuilt != *retained {
            return Err(vec![CognitiveProfileRejection::NonCanonicalProfile]);
        }
        previous = Some(retained);
    }
    Ok(active)
}

fn validate_authority_policy(
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    authority_policy: &CognitiveAuthorityPolicy,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    let mut canonical_policy = policy.clone();
    canonicalize_policy(&mut canonical_policy);
    let mut canonical_evidence = input.evidence.clone();
    canonical_evidence.sort();
    if digest(&canonical_policy)? != authority_policy.policy_sha256
        || digest(&canonical_evidence)? != authority_policy.evidence_sha256
    {
        return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
    }
    Ok(())
}

fn verify_authority_proof(
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    current: &ActiveCognitiveAuthority,
    proof: &CognitiveAuthorityProof,
) -> Result<ActiveCognitiveAuthority, Vec<CognitiveProfileRejection>> {
    let active = if let Some(rotation) = &proof.rotation {
        verify_rotation(rotation, input, current)?
    } else {
        current.clone()
    };
    if proof.context != active.context {
        return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
    }
    let mut canonical_input = input.clone();
    canonicalize_input(&mut canonical_input);
    let mut canonical_policy = policy.clone();
    canonicalize_policy(&mut canonical_policy);
    let policy_sha256 = digest(&canonical_policy)?;
    let evidence_sha256 = digest(&canonical_input.evidence)?;
    let canonical_input_sha256 = digest(&canonical_input)?;
    let authority_context_sha256 = digest(&active.context)?;
    let statement = &proof.statement;
    if statement.schema != COGNITIVE_AUTHORITY_STATEMENT_SCHEMA
        || statement.authority_context_sha256 != authority_context_sha256
        || statement.profile_id != canonical_input.profile_id
        || statement.revision != canonical_input.revision
        || statement.previous_profile_sha256 != canonical_input.previous_profile_sha256
        || statement.canonical_input_sha256 != canonical_input_sha256
        || statement.policy_sha256 != policy_sha256
        || statement.evidence_sha256 != evidence_sha256
        || verify_statement_signature(statement, &active.verifying_key).is_err()
    {
        return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
    }
    Ok(active)
}

fn verify_rotation(
    rotation: &CognitiveAuthorityRotation,
    input: &CognitiveProfileInput,
    current: &ActiveCognitiveAuthority,
) -> Result<ActiveCognitiveAuthority, Vec<CognitiveProfileRejection>> {
    let current_digest = digest(&current.context)?;
    if rotation.schema != COGNITIVE_AUTHORITY_ROTATION_SCHEMA
        || rotation.from_authority_context_sha256 != current_digest
        || rotation.profile_id != input.profile_id
        || rotation.revision != input.revision
        || rotation.to.authority_id != current.context.authority_id
        || rotation.to.key_id == current.context.key_id
        || rotation.to.verifying_key_hex == current.context.verifying_key_hex
        || rotation.to.epoch != current.context.epoch.checked_add(1).unwrap_or(0)
        || !valid_authority_context(&rotation.to)
        || verify_rotation_signature(rotation, &current.verifying_key).is_err()
    {
        return Err(vec![CognitiveProfileRejection::InvalidAuthority]);
    }
    let key_bytes = hex::decode(&rotation.to.verifying_key_hex)
        .ok()
        .and_then(|bytes| <[u8; 32]>::try_from(bytes).ok())
        .ok_or_else(|| vec![CognitiveProfileRejection::InvalidAuthority])?;
    let verifying_key = VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_| vec![CognitiveProfileRejection::InvalidAuthority])?;
    Ok(ActiveCognitiveAuthority {
        context: rotation.to.clone(),
        verifying_key,
    })
}

fn valid_authority_context(context: &CognitiveAuthorityContext) -> bool {
    valid_id(&context.authority_id)
        && valid_id(&context.key_id)
        && context.epoch > 0
        && authority_context_payload_digest(context).ok().as_deref()
            == Some(context.context_sha256.as_str())
        && context.verifying_key_hex.len() == 64
        && context
            .verifying_key_hex
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

pub fn authority_context_payload_digest(
    context: &CognitiveAuthorityContext,
) -> Result<String, Vec<CognitiveProfileRejection>> {
    let mut unsigned = context.clone();
    unsigned.context_sha256.clear();
    digest(&unsigned)
}

fn sign_canonical<T: Serialize>(
    value: &T,
    key: &SigningKey,
) -> Result<String, Vec<CognitiveProfileRejection>> {
    let bytes =
        serde_jcs::to_vec(value).map_err(|_| vec![CognitiveProfileRejection::EncodingFailure])?;
    Ok(hex::encode(key.sign(&bytes).to_bytes()))
}

fn verify_statement_signature(
    statement: &CognitiveAuthorityStatement,
    key: &VerifyingKey,
) -> Result<(), ()> {
    let mut unsigned = statement.clone();
    let signature = std::mem::take(&mut unsigned.signature);
    verify_canonical_signature(&unsigned, &signature, key)
}

fn verify_rotation_signature(
    rotation: &CognitiveAuthorityRotation,
    key: &VerifyingKey,
) -> Result<(), ()> {
    let mut unsigned = rotation.clone();
    let signature = std::mem::take(&mut unsigned.signature);
    verify_canonical_signature(&unsigned, &signature, key)
}

#[cfg(any(test, feature = "test-support"))]
#[path = "../tests/fixtures/cognitive_profile/authority_tests.rs"]
#[cfg_attr(feature = "test-support", allow(dead_code, unused_imports))]
pub(crate) mod authority_tests;

fn verify_canonical_signature<T: Serialize>(
    value: &T,
    signature: &str,
    key: &VerifyingKey,
) -> Result<(), ()> {
    if signature.len() != 128
        || !signature
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(());
    }
    let bytes = serde_jcs::to_vec(value).map_err(|_| ())?;
    let signature_bytes = hex::decode(signature).map_err(|_| ())?;
    let signature = Signature::from_slice(&signature_bytes).map_err(|_| ())?;
    key.verify(&bytes, &signature).map_err(|_| ())
}

fn input_from_profile(profile: &CognitiveProfile) -> CognitiveProfileInput {
    CognitiveProfileInput {
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
        || c.predecessor_head != b.continuity_head
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
    continuity_head: &str,
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
    let mut input_field_keys = BTreeSet::new();
    for field in &input.fields {
        if !input_field_keys.insert(field.key.to_ascii_lowercase()) {
            errors.insert(CognitiveProfileRejection::UnsupportedField);
        }
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
    if input.nonclaims != policy.required_nonclaims {
        if policy
            .required_nonclaims
            .iter()
            .any(|required| !input.nonclaims.contains(required))
        {
            errors.insert(CognitiveProfileRejection::MissingNonclaim);
        } else {
            errors.insert(CognitiveProfileRejection::ForbiddenInference);
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
            if !valid_prior_profile(
                prev,
                input,
                policy,
                identity_root,
                continuity_head,
                expected_policy_sha256,
            ) {
                errors.insert(CognitiveProfileRejection::NonCanonicalProfile);
            }
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

fn valid_prior_profile(
    previous: &CognitiveProfile,
    current: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    identity_root: &str,
    continuity_head: &str,
    expected_policy_sha256: Option<&str>,
) -> bool {
    let prior_input = input_from_profile(previous);
    let mut canonical_prior_input = prior_input.clone();
    canonicalize_input(&mut canonical_prior_input);
    let mut payload_errors = BTreeSet::new();
    validate_policy_and_input(
        &canonical_prior_input,
        policy,
        None,
        identity_root,
        continuity_head,
        expected_policy_sha256,
        &mut payload_errors,
    );
    payload_errors.remove(&CognitiveProfileRejection::InvalidRevisionLink);
    payload_errors.remove(&CognitiveProfileRejection::UnexplainedMutation);

    let rules: BTreeMap<_, _> = policy
        .allowed_fields
        .iter()
        .map(|field| (field.key.as_str(), field))
        .collect();
    let public_fields = previous
        .fields
        .iter()
        .filter(|field| {
            rules
                .get(field.key.as_str())
                .is_some_and(|rule| rule.public)
        })
        .map(|field| PublicCognitiveField {
            key: field.key.clone(),
            value: field.value.clone(),
        })
        .collect::<Vec<_>>();
    let mut expected_projection = PublicCognitiveProfile {
        schema: COGNITIVE_PROFILE_PUBLIC_SCHEMA.into(),
        profile_id: previous.profile_id.clone(),
        revision: previous.revision,
        identity_root: previous.identity_root.clone(),
        fields: public_fields,
        nonclaims: previous.nonclaims.clone(),
        source_profile_sha256: previous.profile_sha256.clone(),
        projection_sha256: String::new(),
    };
    let projection_digest = public_projection_digest(&expected_projection).ok();
    expected_projection.projection_sha256 = projection_digest.unwrap_or_default();

    payload_errors.is_empty()
        && previous.schema == COGNITIVE_PROFILE_SCHEMA
        && prior_input == canonical_prior_input
        && digest(&canonical_prior_input).ok().as_deref()
            == Some(previous.canonical_input_sha256.as_str())
        && profile_digest(previous).ok().as_deref() == Some(previous.profile_sha256.as_str())
        && expected_policy_sha256 == Some(previous.policy_sha256.as_str())
        && previous.identity_root == identity_root
        && previous.continuity_head == continuity_head
        && previous.birthday_candidate_sha256 == current.birthday_candidate_sha256
        && previous.identity_record_sha256 == current.identity_record_sha256
        && previous.continuity_record_sha256 == current.continuity_record_sha256
        && previous.capability_envelope_sha256 == current.capability_envelope_sha256
        && valid_prior_revision_shape(previous)
        && previous.public_projection.fields.len() < previous.fields.len()
        && previous.public_projection == expected_projection
}

fn valid_prior_revision_shape(previous: &CognitiveProfile) -> bool {
    if previous.revision == 0 {
        return false;
    }
    if previous.revision == 1 {
        let field_keys = previous
            .fields
            .iter()
            .map(|field| field.key.as_str())
            .collect::<Vec<_>>();
        return previous.previous_profile_sha256.is_none()
            && previous.removed_fields.is_empty()
            && previous
                .added_fields
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                == field_keys;
    }
    previous
        .previous_profile_sha256
        .as_deref()
        .is_some_and(is_sha)
        && previous
            .added_fields
            .iter()
            .chain(&previous.removed_fields)
            .all(|value| syntactic_id(value))
        && previous
            .added_fields
            .iter()
            .all(|value| !previous.removed_fields.contains(value))
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
