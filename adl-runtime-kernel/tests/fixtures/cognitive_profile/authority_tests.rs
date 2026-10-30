//! PVF: deterministic-core, release-gating contract proof with small resource profile.

use super::input_from_profile;
use crate::*;
use ed25519_dalek::SigningKey;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path},
};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const R: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

pub(crate) fn birthday(root: &str) -> BirthdayCandidate {
    let mut value: BirthdayCandidate = serde_json::from_value(json!({
        "schema":BIRTHDAY_CANDIDATE_SCHEMA,"candidate_id":"candidate-5830","lifecycle_event":"birth_candidate",
        "stable_name":"Aster","identity_root":root,"continuity_head":H,
        "bounded_cycles":[{"cycle":1,"identity_root":root,"continuity_head":R},{"cycle":2,"identity_root":root,"continuity_head":H}],
        "evidence":EvidenceKind::REQUIRED.iter().enumerate().map(|(n,k)|json!({"kind":k,"path":format!("evidence/birthday/{n}.json"),"sha256":H,"visibility":"reviewer_visible"})).collect::<Vec<_>>(),
        "cognitive_profile":SUPPORTED_COGNITIVE_PROFILE,"public_claims":[],"packet_sha256":""
    })).unwrap();
    value.packet_sha256 = candidate_digest(&value).unwrap();
    value
}
fn identity(root: &str) -> BirthdayIdentityRecord {
    let mut value: BirthdayIdentityRecord=serde_json::from_value(json!({
        "schema":BIRTHDAY_IDENTITY_RECORD_SCHEMA,"stable_name":"Aster","identity_root":root,"aliases":[],
        "origin":{"event_id":"origin","provenance_id":"origin","reference":{"id":"origin","path":"evidence/origin.json","sha256":H}},
        "continuity":{"identity_root":root,"head_sha256":H,"reference":{"id":"continuity","path":"evidence/continuity.json","sha256":H}},
        "provenance":[{"id":"origin","path":"evidence/origin.json","sha256":H}],"witnesses":[{"id":"witness","path":"evidence/witness.json","sha256":H}],
        "governed_projection":{"id":"projection","path":"evidence/projection.json","sha256":H},
        "projection_receipt":{"schema":"adl.birthday.verified_projection_receipt.v1","subject_id":"Aster","lineage_id":"lineage","generation":1,"signing_key_id":"key","accepted_record_hash":H,"projection_sha256":H,"visible_fields":{},"redacted_fields":[]},"record_sha256":""
    })).unwrap();
    value.record_sha256 = birthday_identity::record_digest(&value).unwrap();
    value
}
fn continuity(identity: &BirthdayIdentityRecord) -> BirthdayContinuityRecord {
    let mut value: BirthdayContinuityRecord=serde_json::from_value(json!({
        "schema":"adl.birthday.continuity_record.v1","identity_root":identity.identity_root,"identity_record_sha256":identity.record_sha256,
        "predecessor_head":H,"authority_context_sha256":H,"cycles":[{"generation":1,"accepted_through":1,"previous_integrity":H,"integrity":H,"signing_key_id":"key","reference":{"id":"cycle","path":"evidence/cycle.json","sha256":H}}],
        "grade":"evidence_backed","continuity_head":H,"record_sha256":""
    })).unwrap();
    value.record_sha256 = continuity_record_digest(&value).unwrap();
    value
}
fn cap_evidence(
    id: &str,
    kind: CapabilityEvidenceKind,
    issue: u64,
    sha: &str,
) -> CapabilityEvidenceReference {
    CapabilityEvidenceReference {
        id: id.into(),
        kind,
        issue,
        path: format!("evidence/{id}.json"),
        sha256: sha.into(),
        revision_sha256: R.into(),
    }
}
fn declaration(id: &str, p: &str) -> CapabilityDeclaration {
    CapabilityDeclaration {
        id: id.into(),
        provenance_ids: vec![p.into()],
    }
}
pub(crate) fn capability(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
) -> (CapabilityEnvelope, CapabilityEnvelopePolicy) {
    let evidence = vec![
        cap_evidence(
            "wp08",
            CapabilityEvidenceKind::Birthday,
            5825,
            &b.packet_sha256,
        ),
        cap_evidence(
            "wp09",
            CapabilityEvidenceKind::Identity,
            5826,
            &i.record_sha256,
        ),
        cap_evidence(
            "retained",
            CapabilityEvidenceKind::RetainedCapability,
            4761,
            H,
        ),
        cap_evidence("provider", CapabilityEvidenceKind::Provider, 5665, H),
        cap_evidence("model", CapabilityEvidenceKind::Model, 5665, H),
        cap_evidence("authority", CapabilityEvidenceKind::Authority, 4761, H),
    ];
    let limits = CapabilityResourceLimits {
        max_prompt_tokens: 4096,
        max_output_tokens: 1024,
        max_tool_calls: 4,
        max_skill_invocations: 2,
        timeout_ms: 30_000,
        max_recurrence_depth: 1,
    };
    let policy = CapabilityEnvelopePolicy {
        schema: CAPABILITY_ENVELOPE_POLICY_SCHEMA.into(),
        evidence: evidence.clone(),
        provider_models: vec![ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "claude-opus-5".into(),
            provenance_ids: vec!["model".into(), "provider".into()],
        }],
        allowed_tools: vec!["tool:read".into()],
        allowed_skills: vec!["skill:review".into()],
        allowed_grants: vec!["grant:invoke".into()],
        required_denials: vec!["deny:private".into()],
        maximum_limits: limits.clone(),
        required_unsupported_claims: vec!["unsupported:unlimited".into()],
    };
    let input = CapabilityEnvelopeInput {
        schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.into(),
        birthday_candidate_sha256: b.packet_sha256.clone(),
        identity_record_sha256: i.record_sha256.clone(),
        evidence,
        provider_model: ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "claude-opus-5".into(),
            provenance_ids: vec!["model".into(), "provider".into()],
        },
        tools: vec![declaration("tool:read", "retained")],
        skills: vec![declaration("skill:review", "retained")],
        grants: vec![declaration("grant:invoke", "authority")],
        denials: vec![declaration("deny:private", "authority")],
        resource_limits: limits,
        unsupported_claims: vec![declaration("unsupported:unlimited", "retained")],
    };
    (
        build_capability_envelope(b, i, &input, &policy).unwrap(),
        policy,
    )
}

fn capability_with_continuity(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
) -> (
    CapabilityEnvelope,
    CapabilityEnvelopePolicy,
    CapabilityAuthorityPolicy,
) {
    let (component, policy) = capability(b, i);
    let input = CapabilityEnvelopeInput {
        schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.into(),
        birthday_candidate_sha256: component.birthday_candidate_sha256,
        identity_record_sha256: component.identity_record_sha256,
        evidence: component.evidence,
        provider_model: component.provider_model,
        tools: component.tools,
        skills: component.skills,
        grants: component.grants,
        denials: component.denials,
        resource_limits: component.resource_limits,
        unsupported_claims: component.unsupported_claims,
    };
    let authority = RuntimeCapabilityProvisioner::new()
        .provision(&policy, continuity)
        .unwrap();
    let envelope =
        build_capability_envelope_with_continuity(b, i, continuity, &authority, &input, &policy)
            .unwrap();
    (envelope, policy, authority)
}
fn evidence(
    id: &str,
    category: CognitiveEvidenceCategory,
    sha: &str,
    visibility: CognitiveEvidenceVisibility,
) -> CognitiveEvidenceReference {
    CognitiveEvidenceReference {
        id: id.into(),
        category,
        path: format!("evidence/profile/{id}.json"),
        sha256: sha.into(),
        revision_sha256: R.into(),
        visibility,
    }
}
fn authorities() -> (
    BirthdayCandidate,
    BirthdayIdentityRecord,
    BirthdayContinuityRecord,
    CapabilityEnvelope,
    CognitiveProfilePolicy,
) {
    let root = "c".repeat(64);
    let b = birthday(&root);
    let i = identity(&root);
    let c = continuity(&i);
    let (cap, cap_policy) = capability(&b, &i);
    let policy = CognitiveProfilePolicy {
        schema: COGNITIVE_PROFILE_POLICY_SCHEMA.into(),
        evidence: vec![
            evidence(
                "identity",
                CognitiveEvidenceCategory::Identity,
                &i.record_sha256,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "continuity",
                CognitiveEvidenceCategory::Continuity,
                &c.record_sha256,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "memory",
                CognitiveEvidenceCategory::Memory,
                H,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "capability",
                CognitiveEvidenceCategory::Capability,
                &cap.envelope_sha256,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "tom",
                CognitiveEvidenceCategory::TheoryOfMind,
                H,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "intelligence",
                CognitiveEvidenceCategory::Intelligence,
                H,
                CognitiveEvidenceVisibility::InternalRedacted,
            ),
            evidence(
                "learning",
                CognitiveEvidenceCategory::GovernedLearning,
                H,
                CognitiveEvidenceVisibility::Public,
            ),
        ],
        allowed_fields: vec![
            AllowedCognitiveField {
                key: "learning_mode".into(),
                allowed_values: vec!["governed".into()],
                public: true,
            },
            AllowedCognitiveField {
                key: "reasoning_mode".into(),
                allowed_values: vec!["evidence_bounded".into()],
                public: false,
            },
        ],
        required_nonclaims: vec![
            "no_reputation_inference".into(),
            "no_personhood_inference".into(),
            "no_rights_inference".into(),
        ],
        redaction_policy_sha256: H.into(),
        capability_policy: cap_policy,
    };
    (b, i, c, cap, policy)
}
fn input(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    p: &CognitiveProfilePolicy,
) -> CognitiveProfileInput {
    CognitiveProfileInput {
        schema: COGNITIVE_PROFILE_INPUT_SCHEMA.into(),
        profile_id: "profile-aster".into(),
        revision: 1,
        previous_profile_sha256: None,
        birthday_candidate_sha256: b.packet_sha256.clone(),
        identity_record_sha256: i.record_sha256.clone(),
        continuity_record_sha256: c.record_sha256.clone(),
        capability_envelope_sha256: cap.envelope_sha256.clone(),
        update_actor: "runtime-v3".into(),
        update_reason: "initial evidence map".into(),
        added_fields: vec!["reasoning_mode".into(), "learning_mode".into()],
        removed_fields: vec![],
        evidence: p.evidence.iter().cloned().rev().collect(),
        fields: vec![
            CognitiveProfileField {
                key: "reasoning_mode".into(),
                value: "evidence_bounded".into(),
                evidence_ids: vec!["intelligence".into(), "tom".into()],
            },
            CognitiveProfileField {
                key: "learning_mode".into(),
                value: "governed".into(),
                evidence_ids: vec!["learning".into()],
            },
        ],
        nonclaims: p.required_nonclaims.clone(),
        redaction_policy_sha256: H.into(),
    }
}

fn canonical_input_sha(input: &CognitiveProfileInput) -> String {
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
    format!("{:x}", Sha256::digest(serde_jcs::to_vec(&value).unwrap()))
}
fn policy_sha(policy: &CognitiveProfilePolicy) -> String {
    let mut value = policy.clone();
    value.evidence.sort();
    for field in &mut value.allowed_fields {
        field.allowed_values.sort();
        field.allowed_values.dedup();
    }
    value.allowed_fields.sort();
    value.required_nonclaims.sort();
    value.required_nonclaims.dedup();
    format!("{:x}", Sha256::digest(serde_jcs::to_vec(&value).unwrap()))
}
fn evidence_sha(input: &CognitiveProfileInput) -> String {
    let mut evidence = input.evidence.clone();
    evidence.sort();
    format!(
        "{:x}",
        Sha256::digest(serde_jcs::to_vec(&evidence).unwrap())
    )
}
fn context(
    authority_id: &str,
    key_id: &str,
    epoch: u64,
    key: &SigningKey,
) -> CognitiveAuthorityContext {
    let mut context = CognitiveAuthorityContext {
        authority_id: authority_id.into(),
        key_id: key_id.into(),
        epoch,
        context_sha256: String::new(),
        verifying_key_hex: hex::encode(key.verifying_key().as_bytes()),
    };
    context.context_sha256 = authority_context_payload_digest(&context).unwrap();
    context
}
fn trusted(
    key: &SigningKey,
    policy: &CognitiveProfilePolicy,
    input: &CognitiveProfileInput,
) -> CognitiveAuthorityPolicy {
    CognitiveAuthorityPolicy::establish(
        "cognitive-board".into(),
        "cognitive-key-1".into(),
        1,
        key.verifying_key(),
        policy_sha(policy),
        evidence_sha(input),
    )
    .unwrap()
}
fn authority_proof(
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    context: CognitiveAuthorityContext,
    key: &SigningKey,
    rotation: Option<CognitiveAuthorityRotation>,
) -> CognitiveAuthorityProof {
    let statement = CognitiveAuthorityStatement {
        schema: COGNITIVE_AUTHORITY_STATEMENT_SCHEMA.into(),
        authority_context_sha256: format!(
            "{:x}",
            Sha256::digest(serde_jcs::to_vec(&context).unwrap())
        ),
        profile_id: input.profile_id.clone(),
        revision: input.revision,
        previous_profile_sha256: input.previous_profile_sha256.clone(),
        canonical_input_sha256: canonical_input_sha(input),
        policy_sha256: policy_sha(policy),
        evidence_sha256: evidence_sha(input),
        signature: String::new(),
    }
    .sign(key)
    .unwrap();
    CognitiveAuthorityProof {
        context,
        statement,
        rotation,
    }
}
fn build_cognitive_profile(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    input: &CognitiveProfileInput,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
) -> Result<CognitiveProfile, Vec<CognitiveProfileRejection>> {
    let key = SigningKey::from_bytes(&[7; 32]);
    let authority = trusted(&key, policy, input);
    let context = context("cognitive-board", "cognitive-key-1", 1, &key);
    let proof = authority_proof(input, policy, context, &key, None);
    let history = previous.map(std::slice::from_ref).unwrap_or(&[]);
    build_governed_cognitive_profile(b, i, c, cap, input, policy, history, &authority, &proof)
}
fn validate_cognitive_profile(
    profile: &CognitiveProfile,
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    previous: Option<&CognitiveProfile>,
) -> Result<(), Vec<CognitiveProfileRejection>> {
    let key = SigningKey::from_bytes(&[7; 32]);
    let input = input_from_profile(profile);
    let authority = trusted(&key, policy, &input);
    let history = previous.map(std::slice::from_ref).unwrap_or(&[]);
    validate_governed_cognitive_profile(profile, b, i, c, cap, policy, history, &authority)
}
fn next_input(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    history: &[CognitiveProfile],
) -> CognitiveProfileInput {
    let mut value = input(b, i, c, cap, policy);
    if let Some(previous) = history.last() {
        value.revision = previous.revision + 1;
        value.previous_profile_sha256 = Some(previous.profile_sha256.clone());
        value.added_fields.clear();
        value.update_reason = "revalidate retained governed fields".into();
    }
    value
}
#[allow(clippy::too_many_arguments)]
fn governed_build(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &BirthdayContinuityRecord,
    cap: &CapabilityEnvelope,
    policy: &CognitiveProfilePolicy,
    history: &[CognitiveProfile],
    genesis: &CognitiveAuthorityPolicy,
    active_key: &SigningKey,
    active_context: CognitiveAuthorityContext,
    rotation: Option<CognitiveAuthorityRotation>,
) -> CognitiveProfile {
    let input = next_input(b, i, c, cap, policy, history);
    let proof = authority_proof(&input, policy, active_context, active_key, rotation);
    build_governed_cognitive_profile(b, i, c, cap, &input, policy, history, genesis, &proof)
        .unwrap()
}
fn rotation(
    from: &CognitiveAuthorityContext,
    to: CognitiveAuthorityContext,
    profile_id: &str,
    revision: u64,
    signing_key: &SigningKey,
) -> CognitiveAuthorityRotation {
    CognitiveAuthorityRotation {
        schema: COGNITIVE_AUTHORITY_ROTATION_SCHEMA.into(),
        from_authority_context_sha256: format!(
            "{:x}",
            Sha256::digest(serde_jcs::to_vec(from).unwrap())
        ),
        to,
        profile_id: profile_id.into(),
        revision,
        signature: String::new(),
    }
    .sign(signing_key)
    .unwrap()
}

#[test]
fn canonical_creation_is_deterministic_and_public_projection_is_narrow() {
    let matrix: serde_json::Value = serde_json::from_str(include_str!("matrix.json")).unwrap();
    assert!(matrix["negative_cases"].as_array().unwrap().len() >= 20);
    let (b, i, c, cap, p) = authorities();
    let first = input(&b, &i, &c, &cap, &p);
    let mut second = first.clone();
    second.evidence.reverse();
    second.fields.reverse();
    second.nonclaims.reverse();
    let a = build_cognitive_profile(&b, &i, &c, &cap, &first, &p, None).unwrap();
    let z = build_cognitive_profile(&b, &i, &c, &cap, &second, &p, None).unwrap();
    assert_eq!(a, z);
    assert_eq!(a.public_projection.fields.len(), 1);
    assert!(a.public_projection.fields[0].key == "learning_mode");
    validate_cognitive_profile(&a, &b, &i, &c, &cap, &p, None).unwrap();
    if let Ok(rel) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = Path::new(&rel);
        assert!(
            !path.is_absolute() && path.components().all(|c| matches!(c, Component::Normal(_)))
        );
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let out = root.join(path);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::write(out, serde_jcs::to_vec(&a).unwrap()).unwrap();
    }
}
#[test]
fn stale_missing_duplicate_and_forbidden_evidence_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    for case in 0..4 {
        let mut x = input(&b, &i, &c, &cap, &p);
        match case {
            0 => x.evidence[0].revision_sha256 = H.into(),
            1 => {
                x.evidence.pop();
            }
            2 => x.evidence.push(x.evidence[0].clone()),
            _ => x.evidence[0].path = "/Users/operator/private.json".into(),
        }
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());
    }
}
#[test]
fn identity_continuity_capability_and_birthday_substitution_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    let base = input(&b, &i, &c, &cap, &p);
    for case in 0..4 {
        let (mut bb, mut ii, mut cc, mut cp) = (b.clone(), i.clone(), c.clone(), cap.clone());
        match case {
            0 => bb.packet_sha256 = H.into(),
            1 => ii.record_sha256 = H.into(),
            2 => cc.record_sha256 = H.into(),
            _ => cp.envelope_sha256 = H.into(),
        }
        assert!(build_cognitive_profile(&bb, &ii, &cc, &cp, &base, &p, None).is_err());
    }
}

#[tokio::test]
async fn real_continuity_composes_through_capability_and_cognition() {
    let (identity, continuity_policy, manifests) =
        crate::birthday_continuity::authority_tests::real_live_material().await;
    let cycles = crate::birthday_continuity::authority_tests::verify(
        &continuity_policy,
        &identity,
        &manifests,
    )
    .unwrap();
    let record = build_birthday_continuity(&identity, &cycles).unwrap();
    let verified = verify_birthday_continuity_record(&record, &identity, &cycles).unwrap();

    // A second fully valid Runtime history can share the identity checkpoint
    // predecessor while producing a distinct continuity record and head.
    let authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let mut alternate_manifests = manifests.clone();
    alternate_manifests[0].snapshots[0].checksum = "d".repeat(64);
    authority
        .sign_manifest(&mut alternate_manifests[0])
        .unwrap();
    alternate_manifests[1].previous_integrity = Some(alternate_manifests[0].integrity.clone());
    alternate_manifests[1].snapshots[0].checksum = "e".repeat(64);
    authority
        .sign_manifest(&mut alternate_manifests[1])
        .unwrap();
    let alternate_cycles = crate::birthday_continuity::authority_tests::verify(
        &continuity_policy,
        &identity,
        &alternate_manifests,
    )
    .unwrap();
    let alternate_record = build_birthday_continuity(&identity, &alternate_cycles).unwrap();
    let alternate_verified =
        verify_birthday_continuity_record(&alternate_record, &identity, &alternate_cycles).unwrap();
    assert_eq!(record.predecessor_head, alternate_record.predecessor_head);
    assert_ne!(record.record_sha256, alternate_record.record_sha256);
    assert_ne!(record.continuity_head, alternate_record.continuity_head);

    let mut b = birthday(&identity.identity_root);
    b.stable_name = identity.stable_name.clone();
    b.continuity_head = verified.identity_checkpoint_head().to_owned();
    for cycle in &mut b.bounded_cycles {
        cycle.continuity_head = b.continuity_head.clone();
    }
    b.packet_sha256 = candidate_digest(&b).unwrap();
    let (capability, capability_policy, capability_authority) =
        capability_with_continuity(&b, &identity, &verified);
    validate_capability_envelope_with_continuity(
        &capability,
        &b,
        &identity,
        &verified,
        &capability_authority,
        &capability_policy,
    )
    .unwrap();
    assert!(validate_capability_envelope_with_continuity(
        &capability,
        &b,
        &identity,
        &alternate_verified,
        &capability_authority,
        &capability_policy,
    )
    .unwrap_err()
    .contains(&CapabilityEnvelopeRejection::ContinuityBindingMismatch));

    // Rewrite every caller-controlled capability binding and digest to token
    // B. Retained authority A still rejects it; only an explicit Runtime
    // establishment against B authorizes the rewritten envelope.
    let mut rewritten_capability = capability.clone();
    rewritten_capability.continuity_head = alternate_verified.continuity_head().to_owned();
    rewritten_capability.continuity_record_sha256 =
        alternate_verified.record().record_sha256.clone();
    rewritten_capability.envelope_sha256 = envelope_digest(&rewritten_capability).unwrap();
    assert!(validate_capability_envelope_with_continuity(
        &rewritten_capability,
        &b,
        &identity,
        &alternate_verified,
        &capability_authority,
        &capability_policy,
    )
    .unwrap_err()
    .contains(&CapabilityEnvelopeRejection::ContinuityBindingMismatch));
    let alternate_capability_authority = RuntimeCapabilityProvisioner::new()
        .provision(&capability_policy, &alternate_verified)
        .unwrap();
    validate_capability_envelope_with_continuity(
        &rewritten_capability,
        &b,
        &identity,
        &alternate_verified,
        &alternate_capability_authority,
        &capability_policy,
    )
    .unwrap();

    let (_, _, _, _, mut cognitive_policy) = authorities();
    cognitive_policy.capability_policy = capability_policy;
    for item in &mut cognitive_policy.evidence {
        item.sha256 = match item.category {
            CognitiveEvidenceCategory::Identity => identity.record_sha256.clone(),
            CognitiveEvidenceCategory::Continuity => record.record_sha256.clone(),
            CognitiveEvidenceCategory::Capability => capability.envelope_sha256.clone(),
            _ => item.sha256.clone(),
        };
    }
    let cognitive_input = input(&b, &identity, &record, &capability, &cognitive_policy);
    let key = SigningKey::from_bytes(&[7; 32]);
    let authority = trusted(&key, &cognitive_policy, &cognitive_input);
    let authority_context = context("cognitive-board", "cognitive-key-1", 1, &key);
    let proof = authority_proof(
        &cognitive_input,
        &cognitive_policy,
        authority_context,
        &key,
        None,
    );
    let profile = build_governed_cognitive_profile_with_continuity(
        &b,
        &identity,
        &verified,
        &capability_authority,
        &capability,
        &cognitive_input,
        &cognitive_policy,
        &[],
        &authority,
        &proof,
    )
    .unwrap();
    validate_governed_cognitive_profile_with_continuity(
        &profile,
        &b,
        &identity,
        &verified,
        &capability_authority,
        &capability,
        &cognitive_policy,
        &[],
        &authority,
    )
    .unwrap();

    // Rebuild and re-sign every downstream cognitive artifact for the second
    // valid record. The capability remains bound to the first opaque token,
    // so substitution still fails closed rather than accepting shared
    // identity/predecessor fields.
    let mut substituted_policy = cognitive_policy.clone();
    for item in &mut substituted_policy.evidence {
        match item.category {
            CognitiveEvidenceCategory::Continuity => {
                item.sha256 = alternate_record.record_sha256.clone();
            }
            CognitiveEvidenceCategory::Capability => {
                item.sha256 = rewritten_capability.envelope_sha256.clone();
            }
            _ => {}
        }
    }
    let substituted_input = input(
        &b,
        &identity,
        &alternate_record,
        &rewritten_capability,
        &substituted_policy,
    );
    let substituted_authority = trusted(&key, &substituted_policy, &substituted_input);
    let substituted_proof = authority_proof(
        &substituted_input,
        &substituted_policy,
        context("cognitive-board", "cognitive-key-1", 1, &key),
        &key,
        None,
    );
    assert_eq!(
        build_governed_cognitive_profile_with_continuity(
            &b,
            &identity,
            &alternate_verified,
            &capability_authority,
            &rewritten_capability,
            &substituted_input,
            &substituted_policy,
            &[],
            &substituted_authority,
            &substituted_proof,
        )
        .unwrap_err(),
        vec![CognitiveProfileRejection::CapabilityMismatch]
    );

    let mut wrong_identity = identity.clone();
    wrong_identity.identity_root = H.into();
    assert!(validate_capability_envelope_with_continuity(
        &capability,
        &b,
        &wrong_identity,
        &verified,
        &capability_authority,
        &cognitive_policy.capability_policy,
    )
    .is_err());

    let mut replayed_candidate = b.clone();
    replayed_candidate.continuity_head = verified.continuity_head().to_owned();
    for cycle in &mut replayed_candidate.bounded_cycles {
        cycle.continuity_head = replayed_candidate.continuity_head.clone();
    }
    replayed_candidate.packet_sha256 = candidate_digest(&replayed_candidate).unwrap();
    assert!(build_capability_envelope_with_continuity(
        &replayed_candidate,
        &identity,
        &verified,
        &capability_authority,
        &CapabilityEnvelopeInput {
            schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.into(),
            birthday_candidate_sha256: replayed_candidate.packet_sha256.clone(),
            identity_record_sha256: identity.record_sha256.clone(),
            evidence: capability.evidence.clone(),
            provider_model: capability.provider_model.clone(),
            tools: capability.tools.clone(),
            skills: capability.skills.clone(),
            grants: capability.grants.clone(),
            denials: capability.denials.clone(),
            resource_limits: capability.resource_limits.clone(),
            unsupported_claims: capability.unsupported_claims.clone(),
        },
        &cognitive_policy.capability_policy,
    )
    .is_err());
}
#[test]
fn unsupported_labels_diagnosis_and_status_inference_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    for value in [
        "horoscope",
        "diagnosis:anxiety",
        "high_reputation",
        "legal_standing",
        "personhood",
        "consciousness",
        "rights_holder",
    ] {
        let mut x = input(&b, &i, &c, &cap, &p);
        x.fields[0].value = value.into();
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());
    }
}
#[test]
fn revision_updates_link_and_explain_exact_delta() {
    let (b, i, c, cap, p) = authorities();
    let first =
        build_cognitive_profile(&b, &i, &c, &cap, &input(&b, &i, &c, &cap, &p), &p, None).unwrap();
    let mut update = input(&b, &i, &c, &cap, &p);
    update.revision = 2;
    update.previous_profile_sha256 = Some(first.profile_sha256.clone());
    update.fields.retain(|f| f.key != "learning_mode");
    update.added_fields.clear();
    update.removed_fields = vec!["learning_mode".into()];
    update.update_reason = "remove field after evidence review".into();
    let second = build_cognitive_profile(&b, &i, &c, &cap, &update, &p, Some(&first)).unwrap();
    assert_eq!(
        second.previous_profile_sha256.as_deref(),
        Some(first.profile_sha256.as_str())
    );
    validate_cognitive_profile(&second, &b, &i, &c, &cap, &p, Some(&first)).unwrap();
}
#[test]
fn broken_revision_link_and_unexplained_delta_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    let first =
        build_cognitive_profile(&b, &i, &c, &cap, &input(&b, &i, &c, &cap, &p), &p, None).unwrap();
    for case in 0..3 {
        let mut x = input(&b, &i, &c, &cap, &p);
        x.revision = 2;
        x.previous_profile_sha256 = Some(first.profile_sha256.clone());
        x.fields.pop();
        x.removed_fields = vec!["learning_mode".into()];
        match case {
            0 => x.revision = 3,
            1 => x.previous_profile_sha256 = Some(H.into()),
            _ => x.removed_fields.clear(),
        }
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, Some(&first)).is_err());
    }
    for case in 0..3 {
        let mut previous = first.clone();
        match case {
            0 => previous.profile_id = "other-profile".into(),
            1 => previous.identity_root = H.into(),
            _ => previous.policy_sha256 = H.into(),
        }
        let mut x = input(&b, &i, &c, &cap, &p);
        x.revision = 2;
        x.previous_profile_sha256 = Some(previous.profile_sha256.clone());
        x.added_fields.clear();
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, Some(&previous)).is_err());
    }
    for case in 0..4 {
        let mut previous = first.clone();
        match case {
            0 => previous.fields[0].value = "forged".into(),
            1 => previous.public_projection.fields.clear(),
            2 => previous.public_projection.source_profile_sha256 = H.into(),
            _ => previous.public_projection.projection_sha256 = H.into(),
        }
        let mut x = input(&b, &i, &c, &cap, &p);
        x.revision = 2;
        x.previous_profile_sha256 = Some(previous.profile_sha256.clone());
        x.added_fields.clear();
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, Some(&previous)).is_err());
    }

    let mut forged_continuity = first.clone();
    forged_continuity.continuity_head = R.into();
    forged_continuity.profile_sha256 = profile_digest(&forged_continuity).unwrap();
    forged_continuity.public_projection.source_profile_sha256 =
        forged_continuity.profile_sha256.clone();
    forged_continuity.public_projection.projection_sha256 =
        public_projection_digest(&forged_continuity.public_projection).unwrap();
    let mut x = input(&b, &i, &c, &cap, &p);
    x.revision = 2;
    x.previous_profile_sha256 = Some(forged_continuity.profile_sha256.clone());
    x.added_fields.clear();
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, Some(&forged_continuity)).is_err());

    let mut second_input = input(&b, &i, &c, &cap, &p);
    second_input.revision = 2;
    second_input.previous_profile_sha256 = Some(first.profile_sha256.clone());
    second_input.added_fields.clear();
    let mut forged_history =
        build_cognitive_profile(&b, &i, &c, &cap, &second_input, &p, Some(&first)).unwrap();
    forged_history.previous_profile_sha256 = None;
    second_input.previous_profile_sha256 = None;
    forged_history.canonical_input_sha256 = format!(
        "{:x}",
        Sha256::digest(serde_jcs::to_vec(&second_input).unwrap())
    );
    forged_history.profile_sha256 = profile_digest(&forged_history).unwrap();
    forged_history.public_projection.source_profile_sha256 = forged_history.profile_sha256.clone();
    forged_history.public_projection.projection_sha256 =
        public_projection_digest(&forged_history.public_projection).unwrap();
    let mut third_input = input(&b, &i, &c, &cap, &p);
    third_input.revision = 3;
    third_input.previous_profile_sha256 = Some(forged_history.profile_sha256.clone());
    third_input.added_fields.clear();
    assert!(
        build_cognitive_profile(&b, &i, &c, &cap, &third_input, &p, Some(&forged_history)).is_err()
    );
}
#[test]
fn privacy_secrets_paths_and_raw_state_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    for value in [
        "Bearer secret",
        "api_key=secret",
        "gho_secret",
        "private_state",
        "raw_state",
        "/home/operator",
    ] {
        let mut x = input(&b, &i, &c, &cap, &p);
        x.update_reason = value.into();
        assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());
    }
    let mut x = input(&b, &i, &c, &cap, &p);
    x.update_reason = "Bearer gho_api_key".into();
    let errors = build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).unwrap_err();
    let serialized = serde_json::to_string(&errors).unwrap();
    let debug = format!("{errors:?}");
    for secret in ["Bearer", "gho_", "api_key"] {
        assert!(!serialized.contains(secret));
        assert!(!debug.contains(secret));
    }
}
#[test]
fn missing_nonclaims_and_public_everything_fail_closed() {
    let (b, i, c, cap, mut p) = authorities();
    let mut x = input(&b, &i, &c, &cap, &p);
    x.nonclaims.pop();
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());
    for attacker_value in ["gho_secret", "private_state"] {
        let mut x = input(&b, &i, &c, &cap, &p);
        x.nonclaims.push(attacker_value.into());
        let errors = build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).unwrap_err();
        let rejection = format!("{errors:?} {}", serde_json::to_string(&errors).unwrap());
        assert!(!rejection.contains(attacker_value));
    }
    p.allowed_fields.iter_mut().for_each(|f| f.public = true);
    let x = input(&b, &i, &c, &cap, &p);
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());

    let (_, _, _, _, p) = authorities();
    let mut x = input(&b, &i, &c, &cap, &p);
    x.fields
        .iter_mut()
        .find(|field| field.key == "learning_mode")
        .unwrap()
        .evidence_ids = vec!["tom".into()];
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_err());
}
#[test]
fn unknown_json_fields_fail_closed() {
    let (b, i, c, cap, p) = authorities();
    let base = serde_json::to_value(input(&b, &i, &c, &cap, &p)).unwrap();
    for pointer in ["", "/fields/0", "/evidence/0"] {
        let mut v = base.clone();
        v.pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("ambient_private".into(), json!(true));
        assert!(serde_json::from_value::<CognitiveProfileInput>(v).is_err());
    }
}
#[test]
fn exported_validator_rejects_rehashed_forgery() {
    let (b, i, c, cap, p) = authorities();
    let profile =
        build_cognitive_profile(&b, &i, &c, &cap, &input(&b, &i, &c, &cap, &p), &p, None).unwrap();
    let mut forged = profile.clone();
    forged.fields[0].value = "high_reputation".into();
    forged.profile_sha256 = profile_digest(&forged).unwrap();
    assert!(validate_cognitive_profile(&forged, &b, &i, &c, &cap, &p, None).is_err());
}
#[test]
fn exact_duplicates_canonicalize_and_case_collisions_fail() {
    let (b, i, c, cap, p) = authorities();
    let mut x = input(&b, &i, &c, &cap, &p);
    x.fields.push(x.fields[0].clone());
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &p, None).is_ok());
    let mut bad = x;
    bad.fields.push(CognitiveProfileField {
        key: "REASONING_MODE".into(),
        value: "evidence_bounded".into(),
        evidence_ids: vec!["tom".into()],
    });
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &bad, &p, None).is_err());

    let mut update_policy = p.clone();
    update_policy
        .allowed_fields
        .iter_mut()
        .find(|field| field.key == "reasoning_mode")
        .unwrap()
        .allowed_values
        .push("bounded".into());
    let first = build_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &input(&b, &i, &c, &cap, &update_policy),
        &update_policy,
        None,
    )
    .unwrap();
    let mut update = input(&b, &i, &c, &cap, &update_policy);
    update.revision = 2;
    update.previous_profile_sha256 = Some(first.profile_sha256.clone());
    update.added_fields.clear();
    update.fields.push(CognitiveProfileField {
        key: "reasoning_mode".into(),
        value: "bounded".into(),
        evidence_ids: vec!["tom".into()],
    });
    assert!(
        build_cognitive_profile(&b, &i, &c, &cap, &update, &update_policy, Some(&first)).is_err()
    );

    let mut bad_policy = p.clone();
    let mut duplicate_evidence = bad_policy.evidence[0].clone();
    duplicate_evidence.id = duplicate_evidence.id.to_ascii_uppercase();
    bad_policy.evidence.push(duplicate_evidence);
    let x = input(&b, &i, &c, &cap, &bad_policy);
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &bad_policy, None).is_err());

    let mut bad_policy = p.clone();
    let mut duplicate_field = bad_policy.allowed_fields[0].clone();
    duplicate_field.key = duplicate_field.key.to_ascii_uppercase();
    bad_policy.allowed_fields.push(duplicate_field);
    let x = input(&b, &i, &c, &cap, &bad_policy);
    assert!(build_cognitive_profile(&b, &i, &c, &cap, &x, &bad_policy, None).is_err());
}

#[test]
fn trusted_genesis_and_complete_four_revision_chain_replay() {
    let (b, i, c, cap, policy) = authorities();
    let key = SigningKey::from_bytes(&[7; 32]);
    let genesis = trusted(&key, &policy, &input(&b, &i, &c, &cap, &policy));
    let context = context("cognitive-board", "cognitive-key-1", 1, &key);
    let mut history = Vec::new();
    for _ in 0..4 {
        let profile = governed_build(
            &b,
            &i,
            &c,
            &cap,
            &policy,
            &history,
            &genesis,
            &key,
            context.clone(),
            None,
        );
        validate_governed_cognitive_profile(
            &profile, &b, &i, &c, &cap, &policy, &history, &genesis,
        )
        .unwrap();
        history.push(profile);
    }
    assert_eq!(history.len(), 4);
    assert_eq!(
        history[3].previous_profile_sha256.as_deref(),
        Some(history[2].profile_sha256.as_str())
    );
}

#[test]
fn old_key_governs_rotation_and_new_key_governs_followup() {
    let (b, i, c, cap, policy) = authorities();
    let old_key = SigningKey::from_bytes(&[7; 32]);
    let new_key = SigningKey::from_bytes(&[8; 32]);
    let genesis = trusted(&old_key, &policy, &input(&b, &i, &c, &cap, &policy));
    let old_context = context("cognitive-board", "cognitive-key-1", 1, &old_key);
    let new_context = context("cognitive-board", "cognitive-key-2", 2, &new_key);
    let mut history = vec![governed_build(
        &b,
        &i,
        &c,
        &cap,
        &policy,
        &[],
        &genesis,
        &old_key,
        old_context.clone(),
        None,
    )];
    let rotate = rotation(
        &old_context,
        new_context.clone(),
        "profile-aster",
        2,
        &old_key,
    );
    history.push(governed_build(
        &b,
        &i,
        &c,
        &cap,
        &policy,
        &history,
        &genesis,
        &new_key,
        new_context.clone(),
        Some(rotate),
    ));
    let third = governed_build(
        &b,
        &i,
        &c,
        &cap,
        &policy,
        &history,
        &genesis,
        &new_key,
        new_context,
        None,
    );
    validate_governed_cognitive_profile(&third, &b, &i, &c, &cap, &policy, &history, &genesis)
        .unwrap();
}

#[test]
fn self_authorized_policy_statement_and_transplant_fail_closed() {
    let (b, i, c, cap, policy) = authorities();
    let trusted_key = SigningKey::from_bytes(&[7; 32]);
    let attacker_key = SigningKey::from_bytes(&[42; 32]);
    let genesis = trusted(&trusted_key, &policy, &input(&b, &i, &c, &cap, &policy));
    let mut candidate = input(&b, &i, &c, &cap, &policy);
    let attacker_context = context("cognitive-board", "attacker-key", 1, &attacker_key);
    let attacker_proof =
        authority_proof(&candidate, &policy, attacker_context, &attacker_key, None);
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &candidate,
        &policy,
        &[],
        &genesis,
        &attacker_proof,
    )
    .is_err());

    let trusted_context = context("cognitive-board", "cognitive-key-1", 1, &trusted_key);
    let proof = authority_proof(&candidate, &policy, trusted_context, &trusted_key, None);
    let mut altered_policy = policy.clone();
    altered_policy
        .required_nonclaims
        .push("new_nonclaim".into());
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &candidate,
        &altered_policy,
        &[],
        &genesis,
        &proof,
    )
    .is_err());
    candidate.profile_id = "transplanted-profile".into();
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &candidate,
        &policy,
        &[],
        &genesis,
        &proof,
    )
    .is_err());
    assert!(crate::build_cognitive_profile(&b, &i, &c, &cap, &candidate, &policy, None,).is_err());
}

#[test]
fn deep_rehash_truncation_substitution_and_rotation_attacks_fail_closed() {
    let (b, i, c, cap, policy) = authorities();
    let old_key = SigningKey::from_bytes(&[7; 32]);
    let new_key = SigningKey::from_bytes(&[8; 32]);
    let wrong_key = SigningKey::from_bytes(&[9; 32]);
    let genesis = trusted(&old_key, &policy, &input(&b, &i, &c, &cap, &policy));
    let old_context = context("cognitive-board", "cognitive-key-1", 1, &old_key);
    let mut history = Vec::new();
    for _ in 0..3 {
        history.push(governed_build(
            &b,
            &i,
            &c,
            &cap,
            &policy,
            &history,
            &genesis,
            &old_key,
            old_context.clone(),
            None,
        ));
    }
    let next = next_input(&b, &i, &c, &cap, &policy, &history);
    let proof = authority_proof(&next, &policy, old_context.clone(), &old_key, None);
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &next,
        &policy,
        &history[1..],
        &genesis,
        &proof,
    )
    .is_err());

    let new_context = context("cognitive-board", "cognitive-key-2", 2, &new_key);
    let valid_rotation = rotation(
        &old_context,
        new_context.clone(),
        "profile-aster",
        4,
        &old_key,
    );
    let old_key_statement = authority_proof(
        &next,
        &policy,
        new_context.clone(),
        &old_key,
        Some(valid_rotation.clone()),
    );
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &next,
        &policy,
        &history,
        &genesis,
        &old_key_statement,
    )
    .is_err());
    let rotated = governed_build(
        &b,
        &i,
        &c,
        &cap,
        &policy,
        &history,
        &genesis,
        &new_key,
        new_context.clone(),
        Some(valid_rotation.clone()),
    );
    let mut rotated_history = history.clone();
    rotated_history.push(rotated);
    let after_rotation = next_input(&b, &i, &c, &cap, &policy, &rotated_history);
    let replayed_rotation = authority_proof(
        &after_rotation,
        &policy,
        new_context.clone(),
        &new_key,
        Some(valid_rotation),
    );
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &after_rotation,
        &policy,
        &rotated_history,
        &genesis,
        &replayed_rotation,
    )
    .is_err());
    let rotated_back_context = context("cognitive-board", "cognitive-key-1", 3, &old_key);
    let rotate_back = rotation(
        &new_context,
        rotated_back_context.clone(),
        "profile-aster",
        5,
        &new_key,
    );
    let rotate_back_proof = authority_proof(
        &after_rotation,
        &policy,
        rotated_back_context,
        &old_key,
        Some(rotate_back),
    );
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &after_rotation,
        &policy,
        &rotated_history,
        &genesis,
        &rotate_back_proof,
    )
    .is_err());
    let mut forged = history.clone();
    forged[1].fields[0].value = "forged".into();
    forged[1].profile_sha256 = profile_digest(&forged[1]).unwrap();
    forged[1].public_projection.source_profile_sha256 = forged[1].profile_sha256.clone();
    forged[1].public_projection.projection_sha256 =
        public_projection_digest(&forged[1].public_projection).unwrap();
    assert!(build_governed_cognitive_profile(
        &b, &i, &c, &cap, &next, &policy, &forged, &genesis, &proof,
    )
    .is_err());

    for (epoch, signer) in [(1, &new_key), (3, &old_key), (2, &wrong_key)] {
        let to = context("cognitive-board", "cognitive-key-2", epoch, &new_key);
        let rotate = rotation(&old_context, to.clone(), "profile-aster", 4, signer);
        let bad_proof = authority_proof(&next, &policy, to, &new_key, Some(rotate));
        assert!(build_governed_cognitive_profile(
            &b, &i, &c, &cap, &next, &policy, &history, &genesis, &bad_proof,
        )
        .is_err());
    }
    let renamed_same_key = context("cognitive-board", "renamed-key", 2, &old_key);
    let renamed_rotation = rotation(
        &old_context,
        renamed_same_key.clone(),
        "profile-aster",
        4,
        &old_key,
    );
    let renamed_proof = authority_proof(
        &next,
        &policy,
        renamed_same_key,
        &old_key,
        Some(renamed_rotation),
    );
    assert!(build_governed_cognitive_profile(
        &b,
        &i,
        &c,
        &cap,
        &next,
        &policy,
        &history,
        &genesis,
        &renamed_proof,
    )
    .is_err());
}
