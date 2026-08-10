//! PVF: deterministic-core, release-gating contract proof with small resource profile.

use adl_runtime_kernel::*;
use serde_json::json;
use std::{
    fs,
    path::{Component, Path},
};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const R: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn birthday(root: &str) -> BirthdayCandidate {
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
        "predecessor_head":R,"authority_context_sha256":H,"cycles":[{"generation":1,"accepted_through":1,"previous_integrity":R,"integrity":H,"signing_key_id":"key","reference":{"id":"cycle","path":"evidence/cycle.json","sha256":H}}],
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
fn capability(
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

#[test]
fn canonical_creation_is_deterministic_and_public_projection_is_narrow() {
    let matrix: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/cognitive_profile/matrix.json")).unwrap();
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
