//! Test-only constructors that run the production validators and return opaque
//! authority handles for downstream integration tests.

use ed25519_dalek::SigningKey;

use crate::{
    build_birthday_continuity, build_capability_envelope_with_continuity, candidate_digest,
    verify_birthday_continuity_record, BirthdayCandidate, CapabilityEnvelopeInput,
    CognitiveEvidenceCategory, CognitiveEvidenceReference, CognitiveEvidenceVisibility,
    CognitiveProfileField, CognitiveProfileInput, CognitiveProfilePolicy, ResidentCycleAuthority,
    VerifiedMemoryPalaceAuthority, VerifiedResidentCycle, COGNITIVE_PROFILE_INPUT_SCHEMA,
};

const REV: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// Build genuine verified Memory Palace and resident-cycle handles by running
/// the same identity, continuity, capability, and cognitive validators used by
/// production. This surface is unavailable unless the downstream test target
/// explicitly enables `test-support`.
pub fn verified_production_birthday_authorities(
    resident_id: &str,
    cycle_id: &str,
    implementation_revision_sha256: &str,
) -> (
    VerifiedMemoryPalaceAuthority,
    VerifiedResidentCycle,
    BirthdayCandidate,
) {
    let (identity, identity_evidence) =
        crate::birthday_continuity::authority_tests::verified_identity_fixture();
    let (_, continuity_policy, manifests) = crate::birthday_continuity::authority_tests::material();
    let cycles = crate::birthday_continuity::authority_tests::verify(
        &continuity_policy,
        &identity,
        &manifests,
    )
    .expect("verified continuity cycles");
    let record = build_birthday_continuity(&identity, &cycles).expect("continuity record");
    let continuity = verify_birthday_continuity_record(&record, &identity, &cycles)
        .expect("verified continuity record");
    let memory_palace = VerifiedMemoryPalaceAuthority::from_verified_components(
        identity.clone(),
        identity_evidence,
        continuity.clone(),
    );

    let mut birthday = crate::cognitive_profile::authority_tests::birthday(&identity.identity_root);
    birthday.stable_name = identity.stable_name.clone();
    birthday.continuity_head = continuity.identity_checkpoint_head().to_owned();
    for cycle in &mut birthday.bounded_cycles {
        cycle.continuity_head = birthday.continuity_head.clone();
    }
    birthday.packet_sha256 = candidate_digest(&birthday).expect("candidate digest");

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
        .expect("capability authority");
    let capability = build_capability_envelope_with_continuity(
        &birthday,
        &identity,
        &continuity,
        &capability_authority,
        &capability_input,
        &capability_policy,
    )
    .expect("verified capability");
    let evidence = vec![
        evidence(
            "identity",
            CognitiveEvidenceCategory::Identity,
            &identity.record_sha256,
        ),
        evidence(
            "continuity",
            CognitiveEvidenceCategory::Continuity,
            &record.record_sha256,
        ),
        evidence("memory", CognitiveEvidenceCategory::Memory, H),
        evidence(
            "capability",
            CognitiveEvidenceCategory::Capability,
            &capability.envelope_sha256,
        ),
        evidence("tom", CognitiveEvidenceCategory::TheoryOfMind, H),
        evidence("intelligence", CognitiveEvidenceCategory::Intelligence, H),
        public_evidence("learning", CognitiveEvidenceCategory::GovernedLearning, H),
    ];
    let cognitive_policy = CognitiveProfilePolicy {
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
        redaction_policy_sha256: H.to_owned(),
        capability_policy: capability_policy.clone(),
    };
    let cognitive_input = CognitiveProfileInput {
        schema: COGNITIVE_PROFILE_INPUT_SCHEMA.to_owned(),
        profile_id: "resident-profile".to_owned(),
        revision: 1,
        previous_profile_sha256: None,
        birthday_candidate_sha256: birthday.packet_sha256.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        continuity_record_sha256: record.record_sha256.clone(),
        capability_envelope_sha256: capability.envelope_sha256.clone(),
        update_actor: "runtime-resident-cycle".to_owned(),
        update_reason: "production birthday integration proof".to_owned(),
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
        nonclaims: cognitive_policy.required_nonclaims.clone(),
        redaction_policy_sha256: cognitive_policy.redaction_policy_sha256.clone(),
    };
    let cycle = crate::resident_cycle::build_verified_resident_cycle(
        resident_id,
        cycle_id,
        implementation_revision_sha256,
        &birthday,
        &identity,
        &continuity,
        &capability_authority,
        &capability_input,
        &capability_policy,
        &cognitive_input,
        &cognitive_policy,
        &[],
        ResidentCycleAuthority::new(
            "runtime-cognitive-board".to_owned(),
            "runtime-cognitive-key-1".to_owned(),
            1,
            SigningKey::from_bytes(&[7; 32]),
        )
        .expect("resident authority"),
        None,
    )
    .expect("verified resident cycle");
    (memory_palace, cycle, birthday)
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
        visibility: CognitiveEvidenceVisibility::InternalRedacted,
    }
}

fn public_evidence(
    id: &str,
    category: CognitiveEvidenceCategory,
    sha256: &str,
) -> CognitiveEvidenceReference {
    CognitiveEvidenceReference {
        visibility: CognitiveEvidenceVisibility::Public,
        ..evidence(id, category, sha256)
    }
}
