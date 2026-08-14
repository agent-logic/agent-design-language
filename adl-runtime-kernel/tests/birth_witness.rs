//! PVF: deterministic public serialization boundary for WP-15.

use std::{cell::RefCell, fs};

use adl_runtime_kernel::{
    candidate_digest, decide_birthday, reviewed_evidence_set_digest, witness_signing_bytes,
    BirthEventStatus, BirthWitnessAttestation, BirthWitnessPacket, BirthWitnessRole,
    BirthdayCandidate, EvidenceKind, RuntimeInitConfig, WitnessDecision,
    BIRTH_WITNESS_ATTESTATION_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};

#[path = "support/runtime_init.rs"]
mod runtime_init;

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
    let directory = tempfile::tempdir().expect("runtime init directory");
    let state_root = directory.path().join("state");
    fs::create_dir_all(&state_root).expect("state root");
    let state_root = state_root.canonicalize().expect("canonical state root");
    let init_path = runtime_init::write_for_state(
        directory.path(),
        "127.0.0.1:0".parse().expect("test address"),
        &state_root,
    );
    let init = RuntimeInitConfig::load(Some(init_path)).expect("validated runtime init");
    let owner = init
        .birth_witness_owner()
        .expect("boot-trusted birth-witness manifest");

    let fixture = format!(
        "{}/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(fixture).expect("read candidate"))
            .expect("parse candidate");
    candidate
        .evidence
        .iter_mut()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .expect("witness roster evidence")
        .sha256 = owner.roster_sha256().expect("provision roster digest");
    candidate.packet_sha256 = candidate_digest(&candidate).expect("candidate digest");

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
    let emitted = RefCell::new(Vec::new());
    let emitted_ref = &emitted;
    let packet = owner
        .build_validate_and_emit(&candidate, &decision, 7, &attestations, |receipt| {
            let receipt = receipt.to_vec();
            Ok(move || emitted_ref.borrow_mut().extend_from_slice(&receipt))
        })
        .expect("build, validate, and emit receipt");

    assert_eq!(
        packet.receipt.birth_event_status,
        BirthEventStatus::NotClaimed
    );
    assert_eq!(
        *emitted.borrow(),
        serde_jcs::to_vec(&packet.receipt).expect("canonical receipt")
    );
    assert!(!emitted.borrow().is_empty());

    let mut invalid = attestations.clone();
    invalid[0].signature = "0".repeat(128);
    let mut invalid_sink_calls = 0;
    let error = owner
        .build_validate_and_emit(&candidate, &decision, 7, &invalid, |_| {
            invalid_sink_calls += 1;
            Ok(|| {})
        })
        .expect_err("invalid witness must fail before emission");
    assert_eq!(
        error,
        adl_runtime_kernel::BirthWitnessError::InvalidSignature
    );
    assert_eq!(invalid_sink_calls, 0);

    let failed_sink_commits = 0;
    let error = owner
        .build_validate_and_emit(&candidate, &decision, 7, &attestations, |_| {
            Err::<fn(), ()>(())
        })
        .expect_err("sink preparation failure must be reported");
    assert_eq!(
        error,
        adl_runtime_kernel::BirthWitnessError::ReceiptEmission
    );
    assert_eq!(failed_sink_commits, 0);
}
