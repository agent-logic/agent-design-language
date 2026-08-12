//! PVF: deterministic public serialization boundary for WP-15.

use std::fs;

use adl_runtime_kernel::{
    candidate_digest, decide_birthday, reviewed_evidence_set_digest, witness_signing_bytes,
    BirthEventStatus, BirthWitnessAttestation, BirthWitnessPacket, BirthWitnessRole,
    BirthdayCandidate, EvidenceKind, RuntimeBirthWitnessAuthority, RuntimeBirthWitnessService,
    WitnessDecision, BIRTH_WITNESS_ATTESTATION_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};

#[test]
fn public_packet_rejects_unknown_fields() {
    let value = serde_json::json!({
        "schema": "adl.birth_witness.packet.v1",
        "witness_set": {},
        "receipt": {},
        "packet_sha256": "0".repeat(64),
        "raw_private_state": true
    });
    assert!(serde_json::from_value::<BirthWitnessPacket>(value).is_err());
}

#[test]
fn runtime_service_builds_validates_and_emits_receipt() {
    let keys = (1_u8..=4)
        .map(|seed| SigningKey::from_bytes(&[seed; 32]))
        .collect::<Vec<_>>();
    let authorities = BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| RuntimeBirthWitnessAuthority {
            witness_id: format!("witness-{}", index + 1),
            role,
            signing_key_id: format!("witness-key-{}", index + 1),
            verifying_key: keys[index].verifying_key().to_bytes(),
        })
        .collect::<Vec<_>>();

    let fixture = format!(
        "{}/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(fixture).expect("read candidate"))
            .expect("parse candidate");
    let provisional = RuntimeBirthWitnessService::provision(
        "runtime-v3-birth-witness-authority",
        "0".repeat(64),
        7,
        authorities.clone(),
    )
    .expect("provision roster digest");
    candidate
        .evidence
        .iter_mut()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .expect("witness roster evidence")
        .sha256 = provisional.roster_sha256().to_owned();
    candidate.packet_sha256 = candidate_digest(&candidate).expect("candidate digest");

    let service = RuntimeBirthWitnessService::provision(
        "runtime-v3-birth-witness-authority",
        candidate.packet_sha256.clone(),
        7,
        authorities,
    )
    .expect("provision runtime service");
    let evidence_set_sha256 = reviewed_evidence_set_digest(&candidate).expect("evidence digest");
    let attestations = BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| {
            let mut attestation = BirthWitnessAttestation {
                schema: BIRTH_WITNESS_ATTESTATION_SCHEMA.to_owned(),
                witness_id: format!("witness-{}", index + 1),
                role,
                candidate_sha256: candidate.packet_sha256.clone(),
                evidence_set_sha256: evidence_set_sha256.clone(),
                observed_generation: 7,
                decision: WitnessDecision::Accept,
                signing_key_id: format!("witness-key-{}", index + 1),
                signature: "0".repeat(128),
            };
            attestation.signature = hex::encode(
                keys[index]
                    .sign(&witness_signing_bytes(&attestation).expect("signing bytes"))
                    .to_bytes(),
            );
            attestation
        })
        .collect::<Vec<_>>();

    let decision = decide_birthday(&candidate);
    let mut emitted = Vec::new();
    let packet = service
        .build_validate_and_emit(&candidate, &decision, &attestations, |receipt| {
            emitted.extend_from_slice(receipt);
        })
        .expect("build, validate, and emit receipt");

    assert_eq!(
        packet.receipt.birth_event_status,
        BirthEventStatus::NotClaimed
    );
    assert_eq!(
        emitted,
        serde_jcs::to_vec(&packet.receipt).expect("canonical receipt")
    );
    assert!(!emitted.is_empty());

    let mut invalid = attestations.clone();
    invalid[0].signature = "0".repeat(128);
    let mut invalid_sink_calls = 0;
    let error = service
        .build_validate_and_emit(&candidate, &decision, &invalid, |_| {
            invalid_sink_calls += 1;
        })
        .expect_err("invalid witness must fail before emission");
    assert_eq!(
        error,
        adl_runtime_kernel::BirthWitnessError::InvalidSignature
    );
    assert_eq!(invalid_sink_calls, 0);
}
