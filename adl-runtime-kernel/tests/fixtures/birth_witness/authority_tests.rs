//! PVF: deterministic-core, release-gating WP-15 authority/privacy proof.

use std::{
    fs,
    path::{Component, Path},
};

use super::*;
use crate::{candidate_digest, decide_birthday, BirthdayCandidate, BirthdayDecision};
use ed25519_dalek::{Signer, SigningKey};

struct Context {
    candidate: BirthdayCandidate,
    decision: BirthdayDecision,
    policy: BirthWitnessPolicy,
    keys: Vec<SigningKey>,
    attestations: Vec<BirthWitnessAttestation>,
}

fn birthday_fixture() -> String {
    format!(
        "{}/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn matrix_fixture() -> String {
    format!(
        "{}/tests/fixtures/birth_witness/matrix.json",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn trusted(keys: &[SigningKey]) -> Vec<TrustedBirthWitness> {
    BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| TrustedBirthWitness {
            witness_id: format!("witness-{}", index + 1),
            role,
            signing_key_id: format!("witness-key-{}", index + 1),
            verifying_key: keys[index].verifying_key(),
        })
        .collect()
}

fn context() -> Context {
    let keys = (1_u8..=4)
        .map(|seed| SigningKey::from_bytes(&[seed; 32]))
        .collect::<Vec<_>>();
    let trusted = trusted(&keys);
    let roster_sha256 = birth_witness_roster_digest(&trusted).expect("roster digest");
    let mut candidate: BirthdayCandidate = serde_json::from_str(
        &fs::read_to_string(birthday_fixture()).expect("read birthday candidate"),
    )
    .expect("parse birthday candidate");
    candidate
        .evidence
        .iter_mut()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .expect("witness-set reference")
        .sha256 = roster_sha256;
    candidate.packet_sha256 = candidate_digest(&candidate).expect("candidate digest");
    let decision = decide_birthday(&candidate);
    assert!(decision.accepted, "candidate fixture must remain accepted");
    let policy = BirthWitnessPolicy::establish(
        "runtime-v3-birth-witness-authority",
        candidate.packet_sha256.clone(),
        7,
        trusted,
    )
    .expect("provision witness policy");
    let evidence_set_sha256 = reviewed_evidence_set_digest(&candidate).expect("evidence digest");
    let attestations = BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| {
            signed_attestation(
                &keys[index],
                index,
                role,
                &candidate.packet_sha256,
                &evidence_set_sha256,
                7,
                WitnessDecision::Accept,
            )
        })
        .collect();
    Context {
        candidate,
        decision,
        policy,
        keys,
        attestations,
    }
}

#[allow(clippy::too_many_arguments)]
fn signed_attestation(
    key: &SigningKey,
    index: usize,
    role: BirthWitnessRole,
    candidate_sha256: &str,
    evidence_set_sha256: &str,
    generation: u64,
    decision: WitnessDecision,
) -> BirthWitnessAttestation {
    let mut attestation = BirthWitnessAttestation {
        schema: BIRTH_WITNESS_ATTESTATION_SCHEMA.to_owned(),
        witness_id: format!("witness-{}", index + 1),
        role,
        candidate_sha256: candidate_sha256.to_owned(),
        evidence_set_sha256: evidence_set_sha256.to_owned(),
        observed_generation: generation,
        decision,
        signing_key_id: format!("witness-key-{}", index + 1),
        signature: "0".repeat(128),
    };
    attestation.signature = hex::encode(
        key.sign(&witness_signing_bytes(&attestation).expect("signing bytes"))
            .to_bytes(),
    );
    attestation
}

fn resign(context: &Context, attestation: &mut BirthWitnessAttestation, index: usize) {
    attestation.signature = hex::encode(
        context.keys[index]
            .sign(&witness_signing_bytes(attestation).expect("signing bytes"))
            .to_bytes(),
    );
}

fn attestations_for(
    context: &Context,
    candidate: &BirthdayCandidate,
) -> Vec<BirthWitnessAttestation> {
    let evidence = reviewed_evidence_set_digest(candidate).expect("evidence digest");
    BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| {
            signed_attestation(
                &context.keys[index],
                index,
                role,
                &candidate.packet_sha256,
                &evidence,
                7,
                WitnessDecision::Accept,
            )
        })
        .collect()
}

#[test]
fn accepts_exact_policy_complete_witnesses_and_emits_canonical_semantics() {
    let context = context();
    let packet = build_birth_witness_packet(
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .expect("build witness packet");
    validate_birth_witness_packet(
        &packet,
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .expect("validate witness packet");
    assert_eq!(
        packet.receipt.disposition,
        ReceiptDisposition::WitnessesAccepted
    );
    assert_eq!(
        packet.receipt.birth_event_status,
        BirthEventStatus::NotClaimed
    );
    assert_eq!(packet.witness_set.witnesses.len(), 4);
    assert!(packet
        .receipt
        .public_evidence
        .iter()
        .all(|entry| entry.path.starts_with("evidence-ref/") && !entry.path.contains('=')));
    assert!(packet.receipt.public_evidence.iter().all(|projected| {
        context
            .candidate
            .evidence
            .iter()
            .all(|source| source.path != projected.path)
    }));

    if let Ok(output) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let relative = Path::new(&output);
        assert!(!relative.is_absolute());
        assert!(!relative
            .components()
            .any(|part| matches!(part, Component::ParentDir)));
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        let output = root.join(relative);
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).expect("create semantic parent");
        }
        fs::write(
            output,
            serde_jcs::to_vec(&packet).expect("canonical packet"),
        )
        .expect("write semantic packet");
    }
}

#[test]
fn equivalent_witness_orders_are_byte_stable() {
    let mut context = context();
    let first = build_birth_witness_packet(
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .unwrap();
    context.attestations.reverse();
    let second = build_birth_witness_packet(
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        serde_jcs::to_vec(&first).unwrap(),
        serde_jcs::to_vec(&second).unwrap()
    );
}

#[test]
fn valid_signed_rejection_yields_caveated_not_claimed_receipt() {
    let mut context = context();
    context.attestations[2].decision = WitnessDecision::Reject;
    let mut changed = context.attestations[2].clone();
    resign(&context, &mut changed, 2);
    context.attestations[2] = changed;
    let packet = build_birth_witness_packet(
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .unwrap();
    assert_eq!(
        packet.receipt.disposition,
        ReceiptDisposition::WitnessesRejected
    );
    assert_eq!(
        packet.receipt.birth_event_status,
        BirthEventStatus::NotClaimed
    );
    assert!(packet
        .receipt
        .caveats
        .iter()
        .any(|value| value == "receipt_is_review_surface_not_birth_authority"));
}

#[test]
fn missing_duplicate_and_substituted_witnesses_fail_closed() {
    let context = context();
    let missing = &context.attestations[..3];
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            missing
        )
        .unwrap_err(),
        BirthWitnessError::MissingRequiredRole
    );

    let mut duplicate = context.attestations.clone();
    duplicate[3] = duplicate[0].clone();
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &duplicate
        )
        .unwrap_err(),
        BirthWitnessError::DuplicateWitness
    );

    let mut unknown = context.attestations.clone();
    unknown[0].witness_id = "unknown-witness".to_owned();
    resign(&context, &mut unknown[0], 0);
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &unknown
        )
        .unwrap_err(),
        BirthWitnessError::UnauthorizedWitness
    );
}

#[test]
fn stale_candidate_and_evidence_substitutions_fail_closed() {
    let context = context();
    for field in ["generation", "candidate", "evidence"] {
        let mut attestations = context.attestations.clone();
        match field {
            "generation" => attestations[0].observed_generation = 6,
            "candidate" => attestations[0].candidate_sha256 = "a".repeat(64),
            "evidence" => attestations[0].evidence_set_sha256 = "b".repeat(64),
            _ => unreachable!(),
        }
        resign(&context, &mut attestations[0], 0);
        let error = build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &attestations,
        )
        .unwrap_err();
        assert!(matches!(
            error,
            BirthWitnessError::StaleWitness | BirthWitnessError::AttestationBindingMismatch
        ));
    }
}

#[test]
fn forged_signature_and_wrong_signing_key_fail_closed() {
    let context = context();
    let mut forged = context.attestations.clone();
    forged[0].signature = "0".repeat(128);
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &forged
        )
        .unwrap_err(),
        BirthWitnessError::InvalidSignature
    );

    let mut wrong_key = context.attestations.clone();
    wrong_key[0].signature = hex::encode(
        context.keys[1]
            .sign(&witness_signing_bytes(&wrong_key[0]).unwrap())
            .to_bytes(),
    );
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &wrong_key
        )
        .unwrap_err(),
        BirthWitnessError::InvalidSignature
    );
}

#[test]
fn policy_candidate_and_roster_bindings_fail_closed() {
    let context = context();
    let wrong_candidate_policy = BirthWitnessPolicy::establish(
        "runtime-v3-birth-witness-authority",
        "a".repeat(64),
        7,
        trusted(&context.keys),
    )
    .unwrap();
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &wrong_candidate_policy,
            &context.attestations
        )
        .unwrap_err(),
        BirthWitnessError::CandidateDigestMismatch
    );

    let mut candidate = context.candidate.clone();
    candidate
        .evidence
        .iter_mut()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .unwrap()
        .sha256 = "b".repeat(64);
    candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
    let policy = BirthWitnessPolicy::establish(
        "runtime-v3-birth-witness-authority",
        candidate.packet_sha256.clone(),
        7,
        trusted(&context.keys),
    )
    .unwrap();
    assert_eq!(
        build_birth_witness_packet(
            &candidate,
            &decide_birthday(&candidate),
            &policy,
            &context.attestations
        )
        .unwrap_err(),
        BirthWitnessError::RosterDigestMismatch
    );
}

#[test]
fn packet_validator_reconstructs_every_public_field() {
    let context = context();
    let packet = build_birth_witness_packet(
        &context.candidate,
        &context.decision,
        &context.policy,
        &context.attestations,
    )
    .unwrap();
    let mut mutations = Vec::new();
    let mut status = packet.clone();
    status.receipt.birth_event_status = BirthEventStatus::Claimed;
    mutations.push(status);
    let mut caveat = packet.clone();
    caveat.receipt.caveats.push("citizen".to_owned());
    mutations.push(caveat);
    let mut summary = packet.clone();
    summary.witness_set.witnesses[0].decision = WitnessDecision::Reject;
    mutations.push(summary);
    for mutation in mutations {
        assert_eq!(
            validate_birth_witness_packet(
                &mutation,
                &context.candidate,
                &context.decision,
                &context.policy,
                &context.attestations
            )
            .unwrap_err(),
            BirthWitnessError::PacketMismatch
        );
    }
}

#[test]
fn private_or_machine_local_public_evidence_never_enters_receipt() {
    let context = context();
    for path in [
        "/Users/operator/private.json",
        "home/runner/key",
        "C:/secret.json",
        "evidence/../secret.json",
        "evidence/gho_secret.json",
        "evidence/password=review-secret",
    ] {
        let mut candidate = context.candidate.clone();
        candidate.evidence[0].path = path.to_owned();
        candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
        let policy = BirthWitnessPolicy::establish(
            "runtime-v3-birth-witness-authority",
            candidate.packet_sha256.clone(),
            7,
            trusted(&context.keys),
        )
        .unwrap();
        let decision = BirthdayDecision {
            candidate_id: candidate.candidate_id.clone(),
            ..decide_birthday(&candidate)
        };
        let attestations = attestations_for(&context, &candidate);
        let error =
            build_birth_witness_packet(&candidate, &decision, &policy, &attestations).unwrap_err();
        if path == "evidence/password=review-secret" {
            assert_eq!(error, BirthWitnessError::UnsafePublicEvidence);
        } else {
            assert!(matches!(
                error,
                BirthWitnessError::CandidateRejected | BirthWitnessError::UnsafePublicEvidence
            ));
        }
    }
}

#[test]
fn canonical_witness_identity_rejects_case_variant_equivocation() {
    let context = context();
    let mut changed = context.attestations.clone();
    changed[0].witness_id = changed[0].witness_id.to_ascii_uppercase();
    resign(&context, &mut changed[0], 0);
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            &changed,
        )
        .unwrap_err(),
        BirthWitnessError::UnauthorizedWitness
    );
}

#[test]
fn unsafe_identifiers_and_policy_collisions_fail_closed_without_echoing_values() {
    let context = context();
    let mut entries = trusted(&context.keys);
    entries[0].witness_id = "gho_secret".to_owned();
    let error = BirthWitnessPolicy::establish(
        "runtime-v3-birth-witness-authority",
        context.candidate.packet_sha256.clone(),
        7,
        entries,
    )
    .unwrap_err();
    assert_eq!(error, BirthWitnessError::InvalidPolicy);
    assert!(!format!("{error:?}").contains("gho_secret"));

    let mut collisions = trusted(&context.keys);
    collisions[1].witness_id = collisions[0].witness_id.to_ascii_uppercase();
    assert_eq!(
        BirthWitnessPolicy::establish(
            "runtime-v3-birth-witness-authority",
            context.candidate.packet_sha256.clone(),
            7,
            collisions
        )
        .unwrap_err(),
        BirthWitnessError::InvalidPolicy
    );
}

#[test]
fn rejected_or_digest_stale_birthday_candidate_is_not_witnessable() {
    let context = context();
    let mut stale = context.candidate.clone();
    stale.stable_name = "mutated".to_owned();
    assert_eq!(
        build_birth_witness_packet(
            &stale,
            &decide_birthday(&stale),
            &context.policy,
            &context.attestations
        )
        .unwrap_err(),
        BirthWitnessError::CandidateRejected
    );

    let mut wrong_decision = context.decision.clone();
    wrong_decision.accepted = false;
    assert_eq!(
        build_birth_witness_packet(
            &context.candidate,
            &wrong_decision,
            &context.policy,
            &context.attestations
        )
        .unwrap_err(),
        BirthWitnessError::DecisionMismatch
    );
}

fn executable_negative_case(name: &str) -> bool {
    let context = context();
    let build = |attestations: &[BirthWitnessAttestation]| {
        build_birth_witness_packet(
            &context.candidate,
            &context.decision,
            &context.policy,
            attestations,
        )
    };
    match name {
        "missing_required_role" => build(&context.attestations[..3]).is_err(),
        "duplicate_witness_id" => {
            let mut values = context.attestations.clone();
            let duplicate = values[0].witness_id.clone();
            values[1].witness_id = duplicate;
            build(&values).is_err()
        }
        "duplicate_role" => {
            let mut values = context.attestations.clone();
            values[1].role = values[0].role;
            build(&values).is_err()
        }
        "duplicate_signing_key" => {
            let mut values = context.attestations.clone();
            let duplicate = values[0].signing_key_id.clone();
            values[1].signing_key_id = duplicate;
            build(&values).is_err()
        }
        "unauthorized_witness" => {
            let mut values = context.attestations.clone();
            values[0].witness_id = "unknown-witness".to_owned();
            resign(&context, &mut values[0], 0);
            build(&values).is_err()
        }
        "role_substitution" => {
            let mut values = context.attestations.clone();
            values[0].role = BirthWitnessRole::MemoryCapability;
            resign(&context, &mut values[0], 0);
            build(&values).is_err()
        }
        "stale_generation" => {
            let mut values = context.attestations.clone();
            values[0].observed_generation = 6;
            resign(&context, &mut values[0], 0);
            build(&values).is_err()
        }
        "candidate_digest_substitution" | "evidence_digest_substitution" => {
            let mut values = context.attestations.clone();
            if name == "candidate_digest_substitution" {
                values[0].candidate_sha256 = "a".repeat(64);
            } else {
                values[0].evidence_set_sha256 = "b".repeat(64);
            }
            resign(&context, &mut values[0], 0);
            build(&values).is_err()
        }
        "forged_signature" => {
            let mut values = context.attestations.clone();
            values[0].signature = "0".repeat(128);
            build(&values).is_err()
        }
        "wrong_signing_key" => {
            let mut values = context.attestations.clone();
            values[0].signature = hex::encode(
                context.keys[1]
                    .sign(&witness_signing_bytes(&values[0]).unwrap())
                    .to_bytes(),
            );
            build(&values).is_err()
        }
        "roster_digest_mismatch" => {
            let mut candidate = context.candidate.clone();
            candidate
                .evidence
                .iter_mut()
                .find(|entry| entry.kind == EvidenceKind::WitnessSet)
                .unwrap()
                .sha256 = "c".repeat(64);
            candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
            let policy = BirthWitnessPolicy::establish(
                "runtime-v3-birth-witness-authority",
                candidate.packet_sha256.clone(),
                7,
                trusted(&context.keys),
            )
            .unwrap();
            build_birth_witness_packet(
                &candidate,
                &decide_birthday(&candidate),
                &policy,
                &attestations_for(&context, &candidate),
            )
            .is_err()
        }
        "policy_candidate_mismatch" => {
            let policy = BirthWitnessPolicy::establish(
                "runtime-v3-birth-witness-authority",
                "d".repeat(64),
                7,
                trusted(&context.keys),
            )
            .unwrap();
            build_birth_witness_packet(
                &context.candidate,
                &context.decision,
                &policy,
                &context.attestations,
            )
            .is_err()
        }
        "equivocal_witness" => {
            let mut values = context.attestations.clone();
            values[0].witness_id = values[0].witness_id.to_ascii_uppercase();
            resign(&context, &mut values[0], 0);
            build(&values).is_err()
        }
        "raw_private_state" | "machine_local_path" | "secret_like_identifier" => {
            let mut candidate = context.candidate.clone();
            candidate.evidence[0].path = match name {
                "raw_private_state" => "evidence/raw_private_state.json",
                "machine_local_path" => "/Users/operator/evidence.json",
                _ => "evidence/password=review-secret",
            }
            .to_owned();
            candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
            let policy = BirthWitnessPolicy::establish(
                "runtime-v3-birth-witness-authority",
                candidate.packet_sha256.clone(),
                7,
                trusted(&context.keys),
            )
            .unwrap();
            build_birth_witness_packet(
                &candidate,
                &decide_birthday(&candidate),
                &policy,
                &attestations_for(&context, &candidate),
            )
            .is_err()
        }
        "receipt_field_tamper" | "premature_birth_claim" => {
            let mut packet = build(&context.attestations).unwrap();
            if name == "receipt_field_tamper" {
                packet.receipt.caveats.push("citizen".to_owned());
            } else {
                packet.receipt.birth_event_status = BirthEventStatus::Claimed;
            }
            validate_birth_witness_packet(
                &packet,
                &context.candidate,
                &context.decision,
                &context.policy,
                &context.attestations,
            )
            .is_err()
        }
        "unknown_field" => {
            let packet = build(&context.attestations).unwrap();
            let mut value = serde_json::to_value(packet).unwrap();
            value
                .as_object_mut()
                .unwrap()
                .insert("raw_private_state".to_owned(), serde_json::json!(true));
            serde_json::from_value::<BirthWitnessPacket>(value).is_err()
        }
        _ => false,
    }
}

#[test]
fn fixture_matrix_executes_every_declared_negative() {
    let cases: Vec<String> =
        serde_json::from_str(&fs::read_to_string(matrix_fixture()).expect("read matrix"))
            .expect("parse matrix");
    assert_eq!(cases.len(), 20);
    for case in cases {
        assert!(
            executable_negative_case(&case),
            "case did not fail closed: {case}"
        );
    }
}
