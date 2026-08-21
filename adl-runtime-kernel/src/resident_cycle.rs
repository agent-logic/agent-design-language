//! Runtime-owned resident cycle integration for capability envelopes and
//! governed cognitive profiles.
//!
//! This module is deliberately above the lower-level capability/profile
//! builders. A resident-cycle caller supplies runtime-observed identity,
//! continuity, policy, evidence, and signing authority inputs; the live Runtime
//! assembly owns the provisioning boundary and returns typed verified handles
//! rather than caller-authored digest metadata.

use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    authority_context_payload_digest, build_capability_envelope_with_continuity, envelope_digest,
    build_governed_cognitive_profile_with_continuity, profile_digest, public_projection_digest,
    validate_capability_envelope_with_continuity,
    validate_governed_cognitive_profile_with_continuity, BirthdayCandidate, BirthdayIdentityRecord,
    CapabilityAuthorityPolicy, CapabilityEnvelope, CapabilityEnvelopeInput,
    CapabilityEnvelopePolicy, CognitiveAuthorityContext, CognitiveAuthorityPolicy,
    CognitiveAuthorityProof, CognitiveAuthorityRotation, CognitiveAuthorityStatement,
    CognitiveEvidenceReference, CognitiveProfile, CognitiveProfileInput, CognitiveProfilePolicy,
    COGNITIVE_AUTHORITY_ROTATION_SCHEMA, COGNITIVE_AUTHORITY_STATEMENT_SCHEMA,
};

pub const RESIDENT_CYCLE_RECORD_SCHEMA: &str = "adl.runtime.resident_cycle_record.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentCycleRecord {
    pub schema: String,
    pub resident_id: String,
    pub cycle_id: String,
    pub identity_root: String,
    pub continuity_head: String,
    pub continuity_record_sha256: String,
    pub capability_envelope_sha256: String,
    pub capability_policy_sha256: String,
    pub cognitive_profile_sha256: String,
    pub cognitive_profile_revision: u64,
    pub cognitive_public_projection_sha256: String,
    pub cognitive_authority_context_sha256: String,
    pub implementation_revision_sha256: String,
    pub evidence: Vec<CognitiveEvidenceReference>,
    pub record_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCapabilityEnvelopeHandle {
    envelope: CapabilityEnvelope,
}

impl VerifiedCapabilityEnvelopeHandle {
    pub fn envelope(&self) -> &CapabilityEnvelope {
        &self.envelope
    }

    pub fn envelope_sha256(&self) -> &str {
        &self.envelope.envelope_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCognitiveProfileHandle {
    profile: CognitiveProfile,
}

impl VerifiedCognitiveProfileHandle {
    pub fn profile(&self) -> &CognitiveProfile {
        &self.profile
    }

    pub fn profile_sha256(&self) -> &str {
        &self.profile.profile_sha256
    }

    pub fn revision(&self) -> u64 {
        self.profile.revision
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedResidentCycle {
    pub resident_id: String,
    pub cycle_id: String,
    pub capability: VerifiedCapabilityEnvelopeHandle,
    pub cognitive_profile: VerifiedCognitiveProfileHandle,
    pub record: ResidentCycleRecord,
}

#[derive(Clone)]
pub struct ResidentCycleAuthority {
    authority_id: String,
    key_id: String,
    epoch: u64,
    signing_key: SigningKey,
}

impl ResidentCycleAuthority {
    pub(crate) fn new(
        authority_id: String,
        key_id: String,
        epoch: u64,
        signing_key: SigningKey,
    ) -> Result<Self, ResidentCycleError> {
        context(&authority_id, &key_id, epoch, &signing_key)?;
        Ok(Self {
            authority_id,
            key_id,
            epoch,
            signing_key,
        })
    }
}

#[derive(Clone)]
pub struct ResidentCycleAuthorityRotation {
    key_id: String,
    epoch: u64,
    signing_key: SigningKey,
}

impl ResidentCycleAuthorityRotation {
    pub(crate) fn new(
        key_id: String,
        epoch: u64,
        signing_key: SigningKey,
    ) -> Result<Self, ResidentCycleError> {
        if key_id.trim().is_empty() || epoch == 0 {
            return Err(ResidentCycleError::InvalidResidentCycle);
        }
        Ok(Self {
            key_id,
            epoch,
            signing_key,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_verified_resident_cycle(
    resident_id: &str,
    cycle_id: &str,
    implementation_revision_sha256: &str,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &crate::VerifiedBirthdayContinuity,
    capability_authority: &CapabilityAuthorityPolicy,
    capability_input: &CapabilityEnvelopeInput,
    capability_policy: &CapabilityEnvelopePolicy,
    cognitive_input: &CognitiveProfileInput,
    cognitive_policy: &CognitiveProfilePolicy,
    complete_history: &[CognitiveProfile],
    authority: ResidentCycleAuthority,
    rotation: Option<ResidentCycleAuthorityRotation>,
) -> Result<VerifiedResidentCycle, ResidentCycleError> {
    if resident_id.trim().is_empty()
        || cycle_id.trim().is_empty()
        || !is_sha256(implementation_revision_sha256)
    {
        return Err(ResidentCycleError::InvalidResidentCycle);
    }
    let envelope = build_capability_envelope_with_continuity(
        birthday,
        identity,
        continuity,
        capability_authority,
        capability_input,
        capability_policy,
    )
    .map_err(ResidentCycleError::Capability)?;

    let proof_material =
        ResidentCycleProofMaterial::new(cognitive_input, cognitive_policy, authority, rotation)?;
    let authority_policy = CognitiveAuthorityPolicy::establish(
        proof_material.genesis_context.authority_id.clone(),
        proof_material.genesis_context.key_id.clone(),
        proof_material.genesis_context.epoch,
        proof_material.genesis_key.verifying_key(),
        cognitive_policy_sha256(cognitive_policy)?,
        cognitive_evidence_sha256(&cognitive_input.evidence)?,
    )
    .map_err(ResidentCycleError::Cognitive)?;
    let proof = proof_material.proof(cognitive_input, cognitive_policy)?;
    let profile = build_governed_cognitive_profile_with_continuity(
        birthday,
        identity,
        continuity,
        capability_authority,
        &envelope,
        cognitive_input,
        cognitive_policy,
        complete_history,
        &authority_policy,
        &proof,
    )
    .map_err(ResidentCycleError::Cognitive)?;

    validate_capability_envelope_with_continuity(
        &envelope,
        birthday,
        identity,
        continuity,
        capability_authority,
        capability_policy,
    )
    .map_err(ResidentCycleError::Capability)?;
    validate_governed_cognitive_profile_with_continuity(
        &profile,
        birthday,
        identity,
        continuity,
        capability_authority,
        &envelope,
        cognitive_policy,
        complete_history,
        &authority_policy,
    )
    .map_err(ResidentCycleError::Cognitive)?;

    let mut record = ResidentCycleRecord {
        schema: RESIDENT_CYCLE_RECORD_SCHEMA.to_owned(),
        resident_id: resident_id.to_owned(),
        cycle_id: cycle_id.to_owned(),
        identity_root: identity.identity_root.clone(),
        continuity_head: continuity.continuity_head().to_owned(),
        continuity_record_sha256: continuity.record().record_sha256.clone(),
        capability_envelope_sha256: envelope.envelope_sha256.clone(),
        capability_policy_sha256: envelope.policy_sha256.clone(),
        cognitive_profile_sha256: profile.profile_sha256.clone(),
        cognitive_profile_revision: profile.revision,
        cognitive_public_projection_sha256: profile.public_projection.projection_sha256.clone(),
        cognitive_authority_context_sha256: profile.authority.context.context_sha256.clone(),
        implementation_revision_sha256: implementation_revision_sha256.to_owned(),
        evidence: profile.evidence.clone(),
        record_sha256: String::new(),
    };
    record.record_sha256 = resident_cycle_record_digest(&record)?;
    Ok(VerifiedResidentCycle {
        resident_id: resident_id.to_owned(),
        cycle_id: cycle_id.to_owned(),
        capability: VerifiedCapabilityEnvelopeHandle { envelope },
        cognitive_profile: VerifiedCognitiveProfileHandle { profile },
        record,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn rehydrate_verified_resident_cycle(
    record: &ResidentCycleRecord,
    capability: CapabilityEnvelope,
    profile: CognitiveProfile,
) -> Result<VerifiedResidentCycle, ResidentCycleError> {
    validate_resident_cycle_record(record, &capability, &profile)?;
    Ok(VerifiedResidentCycle {
        resident_id: record.resident_id.clone(),
        cycle_id: record.cycle_id.clone(),
        capability: VerifiedCapabilityEnvelopeHandle {
            envelope: capability,
        },
        cognitive_profile: VerifiedCognitiveProfileHandle { profile },
        record: record.clone(),
    })
}

pub fn validate_resident_cycle_record(
    record: &ResidentCycleRecord,
    capability: &CapabilityEnvelope,
    profile: &CognitiveProfile,
) -> Result<(), ResidentCycleError> {
    if record.schema != RESIDENT_CYCLE_RECORD_SCHEMA
        || resident_cycle_record_digest(record)? != record.record_sha256
        || record.identity_root != capability.identity_root
        || record.identity_root != profile.identity_root
        || record.continuity_head != capability.continuity_head
        || record.continuity_head != profile.continuity_head
        || record.continuity_record_sha256 != capability.continuity_record_sha256
        || record.continuity_record_sha256 != profile.continuity_record_sha256
        || record.capability_envelope_sha256 != capability.envelope_sha256
        || record.capability_policy_sha256 != capability.policy_sha256
        || envelope_digest(capability).map_err(ResidentCycleError::Capability)?
            != capability.envelope_sha256
        || record.cognitive_profile_sha256 != profile.profile_sha256
        || record.cognitive_profile_revision != profile.revision
        || record.cognitive_public_projection_sha256 != profile.public_projection.projection_sha256
        || record.cognitive_authority_context_sha256 != profile.authority.context.context_sha256
        || !is_sha256(&record.implementation_revision_sha256)
        || record.evidence != profile.evidence
        || profile_digest(profile).map_err(ResidentCycleError::Cognitive)? != profile.profile_sha256
        || public_projection_digest(&profile.public_projection)
            .map_err(ResidentCycleError::Cognitive)?
            != profile.public_projection.projection_sha256
    {
        return Err(ResidentCycleError::InvalidResidentCycle);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ResidentCycleError {
    #[error("invalid resident cycle")]
    InvalidResidentCycle,
    #[error("capability envelope rejected: {0:?}")]
    Capability(Vec<crate::CapabilityEnvelopeRejection>),
    #[error("cognitive profile rejected: {0:?}")]
    Cognitive(Vec<crate::CognitiveProfileRejection>),
    #[error("resident cycle encoding failed")]
    Encoding,
}

struct ResidentCycleProofMaterial {
    genesis_context: CognitiveAuthorityContext,
    genesis_key: SigningKey,
    active_context: CognitiveAuthorityContext,
    active_key: SigningKey,
    rotation: Option<CognitiveAuthorityRotation>,
}

impl ResidentCycleProofMaterial {
    fn new(
        input: &CognitiveProfileInput,
        policy: &CognitiveProfilePolicy,
        authority: ResidentCycleAuthority,
        rotation: Option<ResidentCycleAuthorityRotation>,
    ) -> Result<Self, ResidentCycleError> {
        let genesis_context = context(
            &authority.authority_id,
            &authority.key_id,
            authority.epoch,
            &authority.signing_key,
        )?;
        if let Some(rotation) = rotation {
            let to = context(
                &authority.authority_id,
                &rotation.key_id,
                rotation.epoch,
                &rotation.signing_key,
            )?;
            let rotation_statement = CognitiveAuthorityRotation {
                schema: COGNITIVE_AUTHORITY_ROTATION_SCHEMA.to_owned(),
                from_authority_context_sha256: digest(&genesis_context)?,
                to: to.clone(),
                profile_id: input.profile_id.clone(),
                revision: input.revision,
                signature: String::new(),
            }
            .sign(&authority.signing_key)
            .map_err(ResidentCycleError::Cognitive)?;
            Ok(Self {
                genesis_context,
                genesis_key: authority.signing_key,
                active_context: to,
                active_key: rotation.signing_key,
                rotation: Some(rotation_statement),
            })
        } else {
            let _ = policy;
            Ok(Self {
                active_context: genesis_context.clone(),
                active_key: authority.signing_key.clone(),
                genesis_context,
                genesis_key: authority.signing_key,
                rotation: None,
            })
        }
    }

    fn proof(
        &self,
        input: &CognitiveProfileInput,
        policy: &CognitiveProfilePolicy,
    ) -> Result<CognitiveAuthorityProof, ResidentCycleError> {
        let statement = CognitiveAuthorityStatement {
            schema: COGNITIVE_AUTHORITY_STATEMENT_SCHEMA.to_owned(),
            authority_context_sha256: digest(&self.active_context)?,
            profile_id: input.profile_id.clone(),
            revision: input.revision,
            previous_profile_sha256: input.previous_profile_sha256.clone(),
            canonical_input_sha256: cognitive_input_sha256(input)?,
            policy_sha256: cognitive_policy_sha256(policy)?,
            evidence_sha256: cognitive_evidence_sha256(&input.evidence)?,
            signature: String::new(),
        }
        .sign(&self.active_key)
        .map_err(ResidentCycleError::Cognitive)?;
        Ok(CognitiveAuthorityProof {
            context: self.active_context.clone(),
            statement,
            rotation: self.rotation.clone(),
        })
    }
}

fn context(
    authority_id: &str,
    key_id: &str,
    epoch: u64,
    key: &SigningKey,
) -> Result<CognitiveAuthorityContext, ResidentCycleError> {
    let mut context = CognitiveAuthorityContext {
        authority_id: authority_id.to_owned(),
        key_id: key_id.to_owned(),
        epoch,
        context_sha256: String::new(),
        verifying_key_hex: hex::encode(key.verifying_key().as_bytes()),
    };
    context.context_sha256 =
        authority_context_payload_digest(&context).map_err(ResidentCycleError::Cognitive)?;
    Ok(context)
}

fn resident_cycle_record_digest(
    record: &ResidentCycleRecord,
) -> Result<String, ResidentCycleError> {
    let mut value = record.clone();
    value.record_sha256.clear();
    digest(&value)
}

fn cognitive_input_sha256(input: &CognitiveProfileInput) -> Result<String, ResidentCycleError> {
    let mut value = input.clone();
    value.evidence.sort();
    value.fields.iter_mut().for_each(|field| {
        field.evidence_ids.sort();
        field.evidence_ids.dedup();
    });
    value.fields.sort();
    value.fields.dedup();
    value.added_fields.sort();
    value.added_fields.dedup();
    value.removed_fields.sort();
    value.removed_fields.dedup();
    value.nonclaims.sort();
    value.nonclaims.dedup();
    digest(&value)
}

fn cognitive_policy_sha256(policy: &CognitiveProfilePolicy) -> Result<String, ResidentCycleError> {
    let mut value = policy.clone();
    value.evidence.sort();
    for field in &mut value.allowed_fields {
        field.allowed_values.sort();
        field.allowed_values.dedup();
    }
    value.allowed_fields.sort();
    value.required_nonclaims.sort();
    value.required_nonclaims.dedup();
    digest(&value)
}

fn cognitive_evidence_sha256(
    evidence: &[CognitiveEvidenceReference],
) -> Result<String, ResidentCycleError> {
    let mut value = evidence.to_vec();
    value.sort();
    digest(&value)
}

fn digest<T: Serialize>(value: &T) -> Result<String, ResidentCycleError> {
    let encoded = serde_jcs::to_vec(value).map_err(|_| ResidentCycleError::Encoding)?;
    Ok(format!("{:x}", Sha256::digest(encoded)))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bootstrap_reasoning_services, build_birthday_continuity, build_live_assembly,
        build_production_operation_executors_with_recorder, candidate_digest,
        verify_birthday_continuity_record, BirthdayContinuityRecord, CognitiveEvidenceCategory,
        CognitiveProfileField, LiveBindings, RuntimeRecorder, TimeQualificationBounds, TimeSample,
        TimeSampleError, TimeSampleSource, COGNITIVE_PROFILE_INPUT_SCHEMA,
    };
    use std::{collections::BTreeMap, sync::Arc, time::Duration};

    const REV: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";

    async fn verified_material() -> (
        BirthdayCandidate,
        BirthdayIdentityRecord,
        crate::VerifiedBirthdayContinuity,
        CapabilityEnvelopeInput,
        CapabilityEnvelopePolicy,
        CognitiveProfileInput,
        CognitiveProfilePolicy,
    ) {
        let (identity, continuity_policy, manifests) =
            crate::birthday_continuity::authority_tests::real_live_material().await;
        let cycles = crate::birthday_continuity::authority_tests::verify(
            &continuity_policy,
            &identity,
            &manifests,
        )
        .unwrap();
        let record = build_birthday_continuity(&identity, &cycles).unwrap();
        let continuity = verify_birthday_continuity_record(&record, &identity, &cycles).unwrap();
        let mut birthday =
            crate::cognitive_profile::authority_tests::birthday(&identity.identity_root);
        birthday.stable_name = identity.stable_name.clone();
        birthday.continuity_head = continuity.identity_checkpoint_head().to_owned();
        for cycle in &mut birthday.bounded_cycles {
            cycle.continuity_head = birthday.continuity_head.clone();
        }
        birthday.packet_sha256 = candidate_digest(&birthday).unwrap();
        let (capability, capability_policy) =
            crate::cognitive_profile::authority_tests::capability(&birthday, &identity);
        let capability_input = CapabilityEnvelopeInput {
            schema: crate::CAPABILITY_ENVELOPE_INPUT_SCHEMA.to_owned(),
            birthday_candidate_sha256: capability.birthday_candidate_sha256,
            identity_record_sha256: capability.identity_record_sha256,
            evidence: capability.evidence,
            provider_model: capability.provider_model,
            tools: capability.tools,
            skills: capability.skills,
            grants: capability.grants,
            denials: capability.denials,
            resource_limits: capability.resource_limits,
            unsupported_claims: capability.unsupported_claims,
        };
        let capability_authority = crate::RuntimeCapabilityProvisioner::new()
            .provision(&capability_policy, &continuity)
            .unwrap();
        let capability = build_capability_envelope_with_continuity(
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
        )
        .unwrap();
        let (cognitive_policy, cognitive_input) = cognitive_policy_and_input(
            &birthday,
            &identity,
            continuity.record(),
            &capability,
            &capability_policy,
        );
        (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            cognitive_input,
            cognitive_policy,
        )
    }

    fn cognitive_policy_and_input(
        birthday: &BirthdayCandidate,
        identity: &BirthdayIdentityRecord,
        continuity: &BirthdayContinuityRecord,
        capability: &CapabilityEnvelope,
        capability_policy: &CapabilityEnvelopePolicy,
    ) -> (CognitiveProfilePolicy, CognitiveProfileInput) {
        let evidence = vec![
            evidence(
                "identity",
                CognitiveEvidenceCategory::Identity,
                &identity.record_sha256,
            ),
            evidence(
                "continuity",
                CognitiveEvidenceCategory::Continuity,
                &continuity.record_sha256,
            ),
            evidence(
                "memory",
                CognitiveEvidenceCategory::Memory,
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ),
            evidence(
                "capability",
                CognitiveEvidenceCategory::Capability,
                &capability.envelope_sha256,
            ),
            evidence(
                "tom",
                CognitiveEvidenceCategory::TheoryOfMind,
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ),
            evidence(
                "intelligence",
                CognitiveEvidenceCategory::Intelligence,
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ),
            public_evidence(
                "learning",
                CognitiveEvidenceCategory::GovernedLearning,
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ),
        ];
        let policy = CognitiveProfilePolicy {
            schema: crate::COGNITIVE_PROFILE_POLICY_SCHEMA.to_owned(),
            evidence: evidence.clone(),
            allowed_fields: vec![
                crate::AllowedCognitiveField {
                    key: "learning_mode".to_owned(),
                    allowed_values: vec!["governed".to_owned()],
                    public: true,
                },
                crate::AllowedCognitiveField {
                    key: "reasoning_mode".to_owned(),
                    allowed_values: vec!["evidence_bounded".to_owned()],
                    public: false,
                },
            ],
            required_nonclaims: vec![
                "no_reputation_inference".to_owned(),
                "no_personhood_inference".to_owned(),
                "no_rights_inference".to_owned(),
            ],
            redaction_policy_sha256:
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            capability_policy: capability_policy.clone(),
        };
        let input = CognitiveProfileInput {
            schema: COGNITIVE_PROFILE_INPUT_SCHEMA.to_owned(),
            profile_id: "resident-profile".to_owned(),
            revision: 1,
            previous_profile_sha256: None,
            birthday_candidate_sha256: birthday.packet_sha256.clone(),
            identity_record_sha256: identity.record_sha256.clone(),
            continuity_record_sha256: continuity.record_sha256.clone(),
            capability_envelope_sha256: capability.envelope_sha256.clone(),
            update_actor: "runtime-resident-cycle".to_owned(),
            update_reason: "initial resident cycle construction".to_owned(),
            added_fields: vec!["learning_mode".to_owned(), "reasoning_mode".to_owned()],
            removed_fields: vec![],
            evidence,
            fields: vec![
                CognitiveProfileField {
                    key: "learning_mode".to_owned(),
                    value: "governed".to_owned(),
                    evidence_ids: vec!["learning".to_owned()],
                },
                CognitiveProfileField {
                    key: "reasoning_mode".to_owned(),
                    value: "evidence_bounded".to_owned(),
                    evidence_ids: vec!["intelligence".to_owned(), "tom".to_owned()],
                },
            ],
            nonclaims: policy.required_nonclaims.clone(),
            redaction_policy_sha256: policy.redaction_policy_sha256.clone(),
        };
        (policy, input)
    }

    fn evidence(
        id: &str,
        category: CognitiveEvidenceCategory,
        sha256: &str,
    ) -> CognitiveEvidenceReference {
        CognitiveEvidenceReference {
            id: id.to_owned(),
            category,
            path: format!("evidence/resident/{id}.json"),
            sha256: sha256.to_owned(),
            revision_sha256: REV.to_owned(),
            visibility: crate::CognitiveEvidenceVisibility::InternalRedacted,
        }
    }

    fn public_evidence(
        id: &str,
        category: CognitiveEvidenceCategory,
        sha256: &str,
    ) -> CognitiveEvidenceReference {
        CognitiveEvidenceReference {
            visibility: crate::CognitiveEvidenceVisibility::Public,
            ..evidence(id, category, sha256)
        }
    }

    struct FixedTime;

    #[async_trait::async_trait]
    impl TimeSampleSource for FixedTime {
        async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
            Ok(TimeSample {
                source: "resident-cycle-test".to_owned(),
                unix_millis: 1_720_000_000_000,
                offset_millis: 0,
                round_trip: Duration::from_millis(1),
            })
        }
    }

    fn next_input(previous: &CognitiveProfile) -> CognitiveProfileInput {
        let mut input = CognitiveProfileInput {
            schema: COGNITIVE_PROFILE_INPUT_SCHEMA.to_owned(),
            profile_id: previous.profile_id.clone(),
            revision: previous.revision + 1,
            previous_profile_sha256: Some(previous.profile_sha256.clone()),
            birthday_candidate_sha256: previous.birthday_candidate_sha256.clone(),
            identity_record_sha256: previous.identity_record_sha256.clone(),
            continuity_record_sha256: previous.continuity_record_sha256.clone(),
            capability_envelope_sha256: previous.capability_envelope_sha256.clone(),
            update_actor: "runtime-resident-cycle".to_owned(),
            update_reason: "revalidate retained governed resident fields".to_owned(),
            added_fields: vec![],
            removed_fields: vec![],
            evidence: previous.evidence.clone(),
            fields: previous.fields.clone(),
            nonclaims: previous.nonclaims.clone(),
            redaction_policy_sha256: previous.redaction_policy_sha256.clone(),
        };
        input.evidence.reverse();
        input
    }

    async fn live_assembly() -> crate::LiveAssembly {
        let root = tempfile::tempdir().unwrap();
        let recorder = RuntimeRecorder::new(16);
        let key = SigningKey::from_bytes(&[31; 32]);
        build_live_assembly(LiveBindings {
            recorder: recorder.clone(),
            canonical_ingress_capacity: 64,
            operation_executors: build_production_operation_executors_with_recorder(
                root.path().join("production"),
                recorder.clone(),
            )
            .unwrap(),
            permit_keys: BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
            reasoning: bootstrap_reasoning_services(recorder).unwrap(),
            time_source: Arc::new(FixedTime),
            time_bounds: TimeQualificationBounds {
                timeout: Duration::from_secs(1),
                max_offset: Duration::from_millis(100),
                max_round_trip: Duration::from_millis(100),
                retry_delay: Duration::from_millis(10),
                refresh_interval: Duration::from_secs(60),
            },
        })
        .unwrap()
    }

    #[tokio::test]
    async fn live_assembly_builds_verified_capability_and_profile_handles() {
        let (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            cognitive_input,
            cognitive_policy,
        ) = verified_material().await;
        let assembly = live_assembly().await;
        let cycle = assembly
            .build_verified_resident_cycle(
                "resident-1",
                "cycle-1",
                REV,
                &birthday,
                &identity,
                &continuity,
                &capability_input,
                &capability_policy,
                &cognitive_input,
                &cognitive_policy,
                &[],
                ResidentCycleAuthority {
                    authority_id: "runtime-cognitive-board".to_owned(),
                    key_id: "runtime-cognitive-key-1".to_owned(),
                    epoch: 1,
                    signing_key: SigningKey::from_bytes(&[7; 32]),
                },
                None,
            )
            .unwrap();
        assert_eq!(
            cycle.capability.envelope_sha256(),
            cycle.record.capability_envelope_sha256
        );
        assert_eq!(cycle.cognitive_profile.revision(), 1);
        assert_eq!(
            cycle.cognitive_profile.profile_sha256(),
            cycle.record.cognitive_profile_sha256
        );
        assert_eq!(
            cycle.cognitive_profile.profile().capability_envelope_sha256,
            cycle.capability.envelope_sha256()
        );
        validate_resident_cycle_record(
            &cycle.record,
            cycle.capability.envelope(),
            cycle.cognitive_profile.profile(),
        )
        .unwrap();
    }

    #[tokio::test]
    async fn resident_cycle_supports_governed_update_and_authority_rotation() {
        let (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            cognitive_input,
            cognitive_policy,
        ) = verified_material().await;
        let capability_authority = crate::RuntimeCapabilityProvisioner::new()
            .provision(&capability_policy, &continuity)
            .unwrap();
        let initial = build_verified_resident_cycle(
            "resident-1",
            "cycle-1",
            REV,
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
            &cognitive_input,
            &cognitive_policy,
            &[],
            ResidentCycleAuthority {
                authority_id: "runtime-cognitive-board".to_owned(),
                key_id: "runtime-cognitive-key-1".to_owned(),
                epoch: 1,
                signing_key: SigningKey::from_bytes(&[7; 32]),
            },
            None,
        )
        .unwrap();
        let update_input = next_input(initial.cognitive_profile.profile());
        let rotated = build_verified_resident_cycle(
            "resident-1",
            "cycle-2",
            REV,
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
            &update_input,
            &cognitive_policy,
            &[initial.cognitive_profile.profile().clone()],
            ResidentCycleAuthority {
                authority_id: "runtime-cognitive-board".to_owned(),
                key_id: "runtime-cognitive-key-1".to_owned(),
                epoch: 1,
                signing_key: SigningKey::from_bytes(&[7; 32]),
            },
            Some(ResidentCycleAuthorityRotation {
                key_id: "runtime-cognitive-key-2".to_owned(),
                epoch: 2,
                signing_key: SigningKey::from_bytes(&[8; 32]),
            }),
        )
        .unwrap();
        assert_eq!(rotated.cognitive_profile.revision(), 2);
        assert_eq!(
            rotated
                .cognitive_profile
                .profile()
                .previous_profile_sha256
                .as_deref(),
            Some(initial.cognitive_profile.profile_sha256())
        );
        assert!(rotated
            .cognitive_profile
            .profile()
            .authority
            .rotation
            .is_some());
    }

    #[tokio::test]
    async fn legacy_profile_builder_remains_fail_closed_for_resident_cycle() {
        let (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            cognitive_input,
            cognitive_policy,
        ) = verified_material().await;
        let capability_authority = crate::RuntimeCapabilityProvisioner::new()
            .provision(&capability_policy, &continuity)
            .unwrap();
        let capability = build_capability_envelope_with_continuity(
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
        )
        .unwrap();
        assert_eq!(
            crate::build_cognitive_profile(
                &birthday,
                &identity,
                continuity.record(),
                &capability,
                &cognitive_input,
                &cognitive_policy,
                None,
            ),
            Err(vec![crate::CognitiveProfileRejection::InvalidAuthority])
        );
    }

    #[tokio::test]
    async fn resident_cycle_rehydrates_and_rejects_tamper() {
        let (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            cognitive_input,
            cognitive_policy,
        ) = verified_material().await;
        let capability_authority = crate::RuntimeCapabilityProvisioner::new()
            .provision(&capability_policy, &continuity)
            .unwrap();
        let cycle = build_verified_resident_cycle(
            "resident-1",
            "cycle-1",
            REV,
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
            &cognitive_input,
            &cognitive_policy,
            &[],
            ResidentCycleAuthority {
                authority_id: "runtime-cognitive-board".to_owned(),
                key_id: "runtime-cognitive-key-1".to_owned(),
                epoch: 1,
                signing_key: SigningKey::from_bytes(&[7; 32]),
            },
            None,
        )
        .unwrap();
        let restored = rehydrate_verified_resident_cycle(
            &cycle.record,
            cycle.capability.envelope().clone(),
            cycle.cognitive_profile.profile().clone(),
        )
        .unwrap();
        assert_eq!(restored.record.record_sha256, cycle.record.record_sha256);

        let mut tampered = cycle.record.clone();
        tampered.cognitive_profile_revision = 999;
        assert!(rehydrate_verified_resident_cycle(
            &tampered,
            cycle.capability.envelope().clone(),
            cycle.cognitive_profile.profile().clone(),
        )
        .is_err());

        let mut tampered_evidence = cycle.record.clone();
        tampered_evidence.evidence[0].visibility = crate::CognitiveEvidenceVisibility::Public;
        tampered_evidence.record_sha256 = resident_cycle_record_digest(&tampered_evidence).unwrap();
        assert!(rehydrate_verified_resident_cycle(
            &tampered_evidence,
            cycle.capability.envelope().clone(),
            cycle.cognitive_profile.profile().clone(),
        )
        .is_err());

        let mut invalid_revision = cycle.record.clone();
        invalid_revision.implementation_revision_sha256 = "not-a-sha256".to_owned();
        invalid_revision.record_sha256 = resident_cycle_record_digest(&invalid_revision).unwrap();
        assert!(rehydrate_verified_resident_cycle(
            &invalid_revision,
            cycle.capability.envelope().clone(),
            cycle.cognitive_profile.profile().clone(),
        )
        .is_err());

        let mut forged_capability = cycle.capability.envelope().clone();
        forged_capability.resource_limits.max_tool_calls += 1;
        assert!(rehydrate_verified_resident_cycle(
            &cycle.record,
            forged_capability,
            cycle.cognitive_profile.profile().clone(),
        )
        .is_err());
    }

    #[tokio::test]
    async fn resident_cycle_rejects_stale_continuity_and_authority_rotation_mismatch() {
        let (
            birthday,
            identity,
            continuity,
            capability_input,
            capability_policy,
            mut cognitive_input,
            cognitive_policy,
        ) = verified_material().await;
        let capability_authority = crate::RuntimeCapabilityProvisioner::new()
            .provision(&capability_policy, &continuity)
            .unwrap();
        cognitive_input.continuity_record_sha256 =
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_owned();
        assert!(build_verified_resident_cycle(
            "resident-1",
            "cycle-1",
            REV,
            &birthday,
            &identity,
            &continuity,
            &capability_authority,
            &capability_input,
            &capability_policy,
            &cognitive_input,
            &cognitive_policy,
            &[],
            ResidentCycleAuthority {
                authority_id: "runtime-cognitive-board".to_owned(),
                key_id: "runtime-cognitive-key-1".to_owned(),
                epoch: 1,
                signing_key: SigningKey::from_bytes(&[7; 32]),
            },
            Some(ResidentCycleAuthorityRotation {
                key_id: "runtime-cognitive-key-1".to_owned(),
                epoch: 1,
                signing_key: SigningKey::from_bytes(&[8; 32]),
            }),
        )
        .is_err());
    }
}
