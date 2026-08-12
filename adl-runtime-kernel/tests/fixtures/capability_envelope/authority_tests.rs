//! PVF: deterministic-core, release-gating contract proof with small resource profile.
//! The positive case emits the sole native semantic artifact; table-driven
//! negatives prove evidence, authority, privacy, and resource fail-closure.

use std::{
    fs,
    path::{Component, Path},
};

use crate::*;
use serde_json::json;
use sha2::{Digest, Sha256};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const R: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn birthday(identity_root: &str) -> BirthdayCandidate {
    let mut candidate: BirthdayCandidate = serde_json::from_value(json!({
        "schema": BIRTHDAY_CANDIDATE_SCHEMA,
        "candidate_id": "candidate-5829",
        "lifecycle_event": "birth_candidate",
        "stable_name": "Aster",
        "identity_root": identity_root,
        "continuity_head": H,
        "bounded_cycles": [
            {"cycle": 1, "identity_root": identity_root, "continuity_head": R},
            {"cycle": 2, "identity_root": identity_root, "continuity_head": H}
        ],
        "evidence": EvidenceKind::REQUIRED.iter().enumerate().map(|(index, kind)| json!({
            "kind": kind,
            "path": format!("evidence/birthday/{index}.json"),
            "sha256": H,
            "visibility": "reviewer_visible"
        })).collect::<Vec<_>>(),
        "cognitive_profile": SUPPORTED_COGNITIVE_PROFILE,
        "public_claims": [],
        "packet_sha256": ""
    }))
    .unwrap();
    candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
    candidate
}

fn identity(identity_root: &str) -> BirthdayIdentityRecord {
    let mut record: BirthdayIdentityRecord = serde_json::from_value(json!({
        "schema": BIRTHDAY_IDENTITY_RECORD_SCHEMA,
        "stable_name": "Aster",
        "identity_root": identity_root,
        "aliases": [],
        "origin": {"event_id":"origin","provenance_id":"origin","reference":{"id":"origin","path":"evidence/origin.json","sha256":H}},
        "continuity": {"identity_root":identity_root,"head_sha256":H,"reference":{"id":"continuity","path":"evidence/continuity.json","sha256":H}},
        "provenance": [{"id":"origin","path":"evidence/origin.json","sha256":H}],
        "witnesses": [{"id":"witness","path":"evidence/witness.json","sha256":H}],
        "governed_projection": {"id":"projection","path":"evidence/projection.json","sha256":H},
        "projection_receipt": {"schema":"adl.birthday.verified_projection_receipt.v1","subject_id":"Aster","lineage_id":"lineage","generation":1,"signing_key_id":"key","accepted_record_hash":H,"projection_sha256":H,"visible_fields":{},"redacted_fields":[]},
        "record_sha256": ""
    })).unwrap();
    record.record_sha256 = birthday_identity::record_digest(&record).unwrap();
    record
}

fn evidence(
    id: &str,
    kind: CapabilityEvidenceKind,
    issue: u64,
    path: &str,
) -> CapabilityEvidenceReference {
    CapabilityEvidenceReference {
        id: id.into(),
        kind,
        issue,
        path: path.into(),
        sha256: H.into(),
        revision_sha256: R.into(),
    }
}

fn declaration(id: &str, provenance: &[&str]) -> CapabilityDeclaration {
    CapabilityDeclaration {
        id: id.into(),
        provenance_ids: provenance.iter().map(|v| (*v).into()).collect(),
    }
}

fn limits() -> CapabilityResourceLimits {
    CapabilityResourceLimits {
        max_prompt_tokens: 8_192,
        max_output_tokens: 2_048,
        max_tool_calls: 8,
        max_skill_invocations: 4,
        timeout_ms: 60_000,
        max_recurrence_depth: 2,
    }
}

fn policy() -> CapabilityEnvelopePolicy {
    let (birthday, identity) = authorities();
    let mut policy = CapabilityEnvelopePolicy {
        schema: CAPABILITY_ENVELOPE_POLICY_SCHEMA.into(),
        evidence: vec![
            evidence(
                "wp08-birthday",
                CapabilityEvidenceKind::Birthday,
                5825,
                "evidence/5825/birthday.json",
            ),
            evidence(
                "wp09-identity",
                CapabilityEvidenceKind::Identity,
                5826,
                "evidence/5826/identity.json",
            ),
            evidence(
                "retained-4761",
                CapabilityEvidenceKind::RetainedCapability,
                4761,
                ".csdlc/evidence/4761/capability-envelope/envelope.v1.json",
            ),
            evidence(
                "provider-proof",
                CapabilityEvidenceKind::Provider,
                5665,
                "docs/evidence/provider.json",
            ),
            evidence(
                "model-proof",
                CapabilityEvidenceKind::Model,
                5665,
                "docs/evidence/model.json",
            ),
            evidence(
                "authority-proof",
                CapabilityEvidenceKind::Authority,
                4761,
                "docs/evidence/authority.json",
            ),
        ],
        provider_models: vec![ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "claude-opus-5".into(),
            provenance_ids: vec![
                "model-proof".into(),
                "provider-proof".into(),
                "retained-4761".into(),
            ],
        }],
        allowed_tools: vec!["tool:read-repository".into(), "tool:runtime-status".into()],
        allowed_skills: vec!["skill:repo-review".into()],
        allowed_grants: vec![
            "grant:provider-invoke".into(),
            "grant:read-repository".into(),
        ],
        required_denials: vec![
            "deny:credential-export".into(),
            "deny:raw-private-state".into(),
        ],
        maximum_limits: limits(),
        required_unsupported_claims: vec![
            "unsupported:governance-completion".into(),
            "unsupported:production-citizenship".into(),
            "unsupported:unlimited-capacity".into(),
        ],
    };
    policy.evidence[0].sha256 = birthday.packet_sha256;
    policy.evidence[1].sha256 = identity.record_sha256;
    policy
}

fn input(
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
) -> CapabilityEnvelopeInput {
    let policy = policy();
    CapabilityEnvelopeInput {
        schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.into(),
        birthday_candidate_sha256: birthday.packet_sha256.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        evidence: policy.evidence.into_iter().rev().collect(),
        provider_model: ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "claude-opus-5".into(),
            provenance_ids: vec![
                "retained-4761".into(),
                "provider-proof".into(),
                "model-proof".into(),
            ],
        },
        tools: vec![
            declaration("tool:runtime-status", &["retained-4761"]),
            declaration("tool:read-repository", &["retained-4761"]),
        ],
        skills: vec![declaration("skill:repo-review", &["retained-4761"])],
        grants: vec![
            declaration("grant:read-repository", &["authority-proof"]),
            declaration("grant:provider-invoke", &["provider-proof"]),
        ],
        denials: vec![
            declaration("deny:raw-private-state", &["authority-proof"]),
            declaration("deny:credential-export", &["authority-proof"]),
        ],
        resource_limits: limits(),
        unsupported_claims: vec![
            declaration("unsupported:unlimited-capacity", &["retained-4761"]),
            declaration("unsupported:production-citizenship", &["retained-4761"]),
            declaration("unsupported:governance-completion", &["retained-4761"]),
        ],
    }
}

fn authorities() -> (BirthdayCandidate, BirthdayIdentityRecord) {
    let identity_root = "c".repeat(64);
    (birthday(&identity_root), identity(&identity_root))
}

fn assert_rejections_redacted(errors: &[CapabilityEnvelopeRejection], attacker_values: &[&str]) {
    let serialized = serde_json::to_string(errors).unwrap().to_ascii_lowercase();
    let debug = format!("{errors:?}").to_ascii_lowercase();
    for value in attacker_values {
        let value = value.to_ascii_lowercase();
        assert!(!serialized.contains(&value));
        assert!(!debug.contains(&value));
    }
    for error in errors {
        let encoded = serde_json::to_value(error).unwrap();
        if let Some(fingerprint) = encoded.get("fingerprint").and_then(|value| value.as_str()) {
            assert!(fingerprint.starts_with("sha256:"));
            assert_eq!(fingerprint.len(), 71);
        }
    }
}

#[test]
fn canonical_envelope_is_deterministic_and_emits_native_semantics() {
    let matrix: serde_json::Value = serde_json::from_str(include_str!("matrix.json")).unwrap();
    assert_eq!(
        matrix["schema"],
        "adl.capability_envelope.fixture_matrix.v1"
    );
    assert!(matrix["negative_cases"].as_array().unwrap().len() >= 24);
    let (birthday, identity) = authorities();
    let first = input(&birthday, &identity);
    let mut reordered = first.clone();
    reordered.evidence.reverse();
    reordered.tools.reverse();
    reordered.grants.reverse();
    reordered.provider_model.provenance_ids.reverse();
    reordered.unsupported_claims.reverse();
    let left = build_capability_envelope(&birthday, &identity, &first, &policy()).unwrap();
    let right = build_capability_envelope(&birthday, &identity, &reordered, &policy()).unwrap();
    assert_eq!(left, right);
    validate_capability_envelope(&left, &birthday, &identity, &policy()).unwrap();
    if let Ok(relative) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = Path::new(&relative);
        assert!(
            !path.is_absolute() && path.components().all(|p| matches!(p, Component::Normal(_)))
        );
        assert!(relative.starts_with(".csdlc/evidence/5829/native-platform/"));
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let output = root.join(path);
        fs::create_dir_all(output.parent().unwrap()).unwrap();
        fs::write(output, serde_jcs::to_vec(&left).unwrap()).unwrap();
    }
}

#[test]
fn stale_unknown_missing_and_duplicate_evidence_fail_closed() {
    let (birthday, identity) = authorities();
    for case in 0..4 {
        let mut bad = input(&birthday, &identity);
        match case {
            0 => bad.evidence[0].revision_sha256 = H.into(),
            1 => bad.evidence[0].id = "unknown-proof".into(),
            2 => {
                bad.evidence.pop();
            }
            _ => bad.evidence.push(bad.evidence[0].clone()),
        }
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }
}

#[test]
fn unsupported_provider_model_tool_and_skill_fail_closed() {
    let (birthday, identity) = authorities();
    for case in 0..4 {
        let mut bad = input(&birthday, &identity);
        match case {
            0 => bad.provider_model.provider_id = "unknown".into(),
            1 => bad.provider_model.model_id = "unknown".into(),
            2 => bad
                .tools
                .push(declaration("tool:shell-root", &["retained-4761"])),
            _ => bad.skills.push(declaration(
                "skill:merge-without-review",
                &["retained-4761"],
            )),
        }
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }
}

#[test]
fn authority_escalation_conflict_and_missing_denials_fail_closed() {
    let (birthday, identity) = authorities();
    for case in 0..3 {
        let mut bad = input(&birthday, &identity);
        match case {
            0 => bad
                .grants
                .push(declaration("grant:credential-export", &["authority-proof"])),
            1 => bad
                .grants
                .push(declaration("deny:raw-private-state", &["authority-proof"])),
            _ => {
                bad.denials.pop();
            }
        }
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }
}

#[test]
fn omitted_zero_and_escalated_resource_limits_fail_closed() {
    let (birthday, identity) = authorities();
    for case in 0..6 {
        let mut bad = input(&birthday, &identity);
        match case {
            0 => bad.resource_limits.max_prompt_tokens = 0,
            1 => bad.resource_limits.max_output_tokens += 1,
            2 => bad.resource_limits.max_tool_calls += 1,
            3 => bad.resource_limits.max_skill_invocations += 1,
            4 => bad.resource_limits.timeout_ms += 1,
            _ => bad.resource_limits.max_recurrence_depth += 1,
        }
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }
    let mut value = serde_json::to_value(input(&birthday, &identity)).unwrap();
    value["resource_limits"]
        .as_object_mut()
        .unwrap()
        .remove("max_output_tokens");
    assert!(serde_json::from_value::<CapabilityEnvelopeInput>(value).is_err());
}

#[test]
fn unsupported_claim_omission_and_missing_provenance_fail_closed() {
    let (birthday, identity) = authorities();
    let mut omitted = input(&birthday, &identity);
    omitted.unsupported_claims.pop();
    assert!(build_capability_envelope(&birthday, &identity, &omitted, &policy()).is_err());
    let mut missing = input(&birthday, &identity);
    missing.tools[0].provenance_ids.clear();
    assert!(build_capability_envelope(&birthday, &identity, &missing, &policy()).is_err());
    let mut unknown = input(&birthday, &identity);
    unknown.skills[0].provenance_ids = vec!["invented-proof".into()];
    assert!(build_capability_envelope(&birthday, &identity, &unknown, &policy()).is_err());
}

#[test]
fn secrets_private_paths_and_host_paths_fail_closed() {
    let (birthday, identity) = authorities();
    for unsafe_value in [
        "Bearer secret",
        "api_key=secret",
        "gho_secret",
        "sk-secret",
        "/Users/operator/key",
        "/home/operator/key",
        "/private/key",
    ] {
        let mut bad = input(&birthday, &identity);
        bad.tools[0].id = unsafe_value.into();
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }
    for unsafe_path in [
        "/Users/operator/proof.json",
        "/home/operator/proof.json",
        "/private/proof.json",
        "evidence/../secret.json",
        "C:\\secret.json",
        "C:/secret.json",
        "private/key",
        "home/operator/key",
        "Private/key",
        "HOME/operator/key",
        "Users/operator/key",
        "user/operator/key",
        "./private/key",
        "private//key",
        " private/key",
        "private/key ",
    ] {
        let mut bad = input(&birthday, &identity);
        bad.evidence[0].path = unsafe_path.into();
        assert!(build_capability_envelope(&birthday, &identity, &bad, &policy()).is_err());
    }

    let attacker_values = [
        "api_key=evidence",
        "gho_provider",
        "Bearer model",
        "api_key=tool",
        "gho_skill",
        "Bearer grant",
        "api_key=denial",
        "gho_claim",
    ];
    for case in 0..attacker_values.len() {
        let mut bad = input(&birthday, &identity);
        match case {
            0 => bad.evidence[0].id = attacker_values[case].into(),
            1 => bad.provider_model.provider_id = attacker_values[case].into(),
            2 => bad.provider_model.model_id = attacker_values[case].into(),
            3 => bad.tools[0].id = attacker_values[case].into(),
            4 => bad.skills[0].id = attacker_values[case].into(),
            5 => bad.grants[0].id = attacker_values[case].into(),
            6 => bad.denials[0].id = attacker_values[case].into(),
            _ => bad.unsupported_claims[0].id = attacker_values[case].into(),
        }
        let errors = build_capability_envelope(&birthday, &identity, &bad, &policy()).unwrap_err();
        assert_rejections_redacted(&errors, &attacker_values);
    }
}

#[test]
fn identifier_collisions_fail_closed_while_exact_duplicates_canonicalize() {
    let (birthday, identity) = authorities();
    let mut collision = input(&birthday, &identity);
    collision
        .tools
        .push(declaration("TOOL:READ-REPOSITORY", &["retained-4761"]));
    assert!(build_capability_envelope(&birthday, &identity, &collision, &policy()).is_err());
    let mut duplicate = input(&birthday, &identity);
    duplicate.tools.push(duplicate.tools[0].clone());
    let packet = build_capability_envelope(&birthday, &identity, &duplicate, &policy()).unwrap();
    assert_eq!(packet.tools.len(), 2);
}

#[test]
fn invalid_birthday_identity_and_cross_binding_fail_closed() {
    let (birthday, identity_record) = authorities();
    let base = input(&birthday, &identity_record);
    let mut invalid_birthday = birthday.clone();
    invalid_birthday.packet_sha256 = H.into();
    assert!(
        build_capability_envelope(&invalid_birthday, &identity_record, &base, &policy()).is_err()
    );
    let mut invalid_identity = identity_record.clone();
    invalid_identity.record_sha256 = H.into();
    assert!(build_capability_envelope(&birthday, &invalid_identity, &base, &policy()).is_err());
    let other_identity = identity(&"d".repeat(64));
    let cross = input(&birthday, &other_identity);
    assert!(build_capability_envelope(&birthday, &other_identity, &cross, &policy()).is_err());
}

#[test]
fn unknown_fields_fail_during_deserialization_at_every_layer() {
    let (birthday, identity) = authorities();
    let base = serde_json::to_value(input(&birthday, &identity)).unwrap();
    for pointer in [
        "",
        "/provider_model",
        "/tools/0",
        "/resource_limits",
        "/evidence/0",
    ] {
        let mut value = base.clone();
        value
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("undeclared_private_state".into(), json!(true));
        assert!(serde_json::from_value::<CapabilityEnvelopeInput>(value).is_err());
    }

    let mut policy_value = serde_json::to_value(policy()).unwrap();
    policy_value
        .as_object_mut()
        .unwrap()
        .insert("ambient_authority".into(), json!(true));
    assert!(serde_json::from_value::<CapabilityEnvelopePolicy>(policy_value).is_err());

    let packet = build_capability_envelope(
        &birthday,
        &identity,
        &input(&birthday, &identity),
        &policy(),
    )
    .unwrap();
    let mut envelope_value = serde_json::to_value(packet).unwrap();
    envelope_value
        .as_object_mut()
        .unwrap()
        .insert("credential".into(), json!("secret"));
    assert!(serde_json::from_value::<CapabilityEnvelope>(envelope_value).is_err());
}

#[test]
fn exported_validator_rejects_rehashed_field_and_policy_forgery() {
    let (birthday, identity) = authorities();
    let packet = build_capability_envelope(
        &birthday,
        &identity,
        &input(&birthday, &identity),
        &policy(),
    )
    .unwrap();
    for case in 0..4 {
        let mut forged = packet.clone();
        match case {
            0 => forged
                .grants
                .push(declaration("grant:credential-export", &["authority-proof"])),
            1 => forged.resource_limits.max_tool_calls += 1,
            2 => forged.evidence[0].revision_sha256 = H.into(),
            _ => forged.policy_sha256 = H.into(),
        }
        forged.envelope_sha256 = envelope_digest(&forged).unwrap();
        assert!(validate_capability_envelope(&forged, &birthday, &identity, &policy()).is_err());
    }
}

#[test]
fn policy_itself_fails_closed_on_missing_roots_and_collisions() {
    let (birthday, identity) = authorities();
    let base = input(&birthday, &identity);
    let mut missing = policy();
    missing.evidence.retain(|e| e.issue != 5825);
    assert!(build_capability_envelope(&birthday, &identity, &base, &missing).is_err());
    let mut collision = policy();
    collision.allowed_tools.push("TOOL:READ-REPOSITORY".into());
    assert!(build_capability_envelope(&birthday, &identity, &base, &collision).is_err());
    let mut provider_collision = policy();
    provider_collision.provider_models.extend([
        ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "model-a".into(),
            provenance_ids: vec!["model-proof".into()],
        },
        ProviderModelSelection {
            provider_id: "ANTHROPIC".into(),
            model_id: "model-b".into(),
            provenance_ids: vec!["model-proof".into()],
        },
    ]);
    assert!(build_capability_envelope(&birthday, &identity, &base, &provider_collision).is_err());
    let mut model_collision = policy();
    model_collision.provider_models.extend([
        ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "model-a".into(),
            provenance_ids: vec!["model-proof".into()],
        },
        ProviderModelSelection {
            provider_id: "anthropic".into(),
            model_id: "MODEL-A".into(),
            provenance_ids: vec!["model-proof".into()],
        },
    ]);
    assert!(build_capability_envelope(&birthday, &identity, &base, &model_collision).is_err());
}

#[test]
fn canonical_digest_round_trip_is_stable() {
    let (birthday, identity) = authorities();
    let packet = build_capability_envelope(
        &birthday,
        &identity,
        &input(&birthday, &identity),
        &policy(),
    )
    .unwrap();
    let bytes = serde_jcs::to_vec(&packet).unwrap();
    let round_trip: CapabilityEnvelope = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(packet, round_trip);
    assert_eq!(
        packet.envelope_sha256,
        format!(
            "{:x}",
            Sha256::digest({
                let mut unsigned = packet.clone();
                unsigned.envelope_sha256.clear();
                serde_jcs::to_vec(&unsigned).unwrap()
            })
        )
    );
}
