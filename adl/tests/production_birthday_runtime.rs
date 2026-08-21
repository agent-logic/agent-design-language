use std::fs;

use adl::resident_tool_execution::{
    authenticate_resident_tool_receipt_v1, validate_resident_tool_receipt_for_birthday_v1,
    ResidentToolReceiptDecisionV1, ResidentToolReceiptV1, RuntimeResidentToolTrust,
};
use adl_runtime_kernel::{
    candidate_digest, decide_birthday, reviewed_evidence_set_digest, witness_signing_bytes,
    BirthWitnessAttestation, BirthWitnessRole, BirthdayCandidate, EvidenceKind,
    ProductionBirthdayInput, ProductionBirthdayStore, RuntimeInitConfig,
    VerifiedBirthWitnessBinding, WitnessDecision, BIRTH_WITNESS_ATTESTATION_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};

#[path = "../../adl-runtime-kernel/tests/support/runtime_init.rs"]
mod runtime_init;

fn hash(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn governed_receipt(candidate: &BirthdayCandidate) -> ResidentToolReceiptV1 {
    ResidentToolReceiptV1 {
        schema: "adl.runtime.resident_tool_receipt.v1".into(),
        resident_id: "resident-one".into(),
        authority_id: "authority-one".into(),
        authority_sha256: hash('3'),
        cycle_id: "cycle-one".into(),
        checkpoint_lineage: candidate.continuity_head.clone(),
        proposal_sha256: hash('4'),
        proposal_id: Some(format!("sha256:{}", hash('5'))),
        acc_contract_id: Some("acc-one".into()),
        gate_reason_code: Some("allowed".into()),
        adapter_id: Some("adapter.runtime.observe.dry_run".into()),
        decision: ResidentToolReceiptDecisionV1::Executed,
        reason_code: "governed_execution_completed".into(),
    }
}

fn tool_trust(key_id: &str, signing_key: &SigningKey) -> RuntimeResidentToolTrust {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resident-tool-trust.json");
    fs::write(
        &path,
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.runtime.resident_tool_trust.v1",
            "authorities": [{
                "key_id": key_id,
                "verifying_key_hex": hex::encode(signing_key.verifying_key().to_bytes())
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    RuntimeResidentToolTrust::load_trusted_manifest(&path).unwrap()
}

fn witness(candidate: &mut BirthdayCandidate) -> VerifiedBirthWitnessBinding {
    let directory = tempfile::tempdir().unwrap();
    let state = directory.path().join("state");
    fs::create_dir_all(&state).unwrap();
    let init = RuntimeInitConfig::load(Some(runtime_init::write_for_state(
        directory.path(),
        "127.0.0.1:0".parse().unwrap(),
        &state.canonicalize().unwrap(),
    )))
    .unwrap();
    let owner = init.birth_witness_owner().unwrap();
    candidate
        .evidence
        .iter_mut()
        .find(|entry| entry.kind == EvidenceKind::WitnessSet)
        .unwrap()
        .sha256 = owner.roster_sha256().unwrap();
    candidate.packet_sha256 = candidate_digest(candidate).unwrap();
    let evidence = reviewed_evidence_set_digest(candidate).unwrap();
    let keys = (1_u8..=4)
        .map(|seed| SigningKey::from_bytes(&[seed; 32]))
        .collect::<Vec<_>>();
    let attestations = BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| {
            let mut value = BirthWitnessAttestation {
                schema: BIRTH_WITNESS_ATTESTATION_SCHEMA.into(),
                witness_id: format!("witness-{}", index + 1),
                role,
                candidate_sha256: candidate.packet_sha256.clone(),
                evidence_set_sha256: evidence.clone(),
                observed_generation: 7,
                decision: WitnessDecision::Accept,
                signing_key_id: format!("witness-key-{}", index + 1),
                signature: "0".repeat(128),
            };
            value.signature = hex::encode(
                keys[index]
                    .sign(&witness_signing_bytes(&value).unwrap())
                    .to_bytes(),
            );
            value
        })
        .collect::<Vec<_>>();
    let decision = decide_birthday(candidate);
    owner
        .build_validate_and_emit_verified(candidate, &decision, 7, &attestations, |_| Ok(|| {}))
        .unwrap()
}

#[test]
fn authenticated_runtime_receipt_drives_exactly_once_birthday_and_restart() {
    let candidate_path = format!(
        "{}/../adl-runtime-kernel/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(candidate_path).unwrap()).unwrap();
    let birth_witness = witness(&mut candidate);
    let decision = decide_birthday(&candidate);
    let receipt = governed_receipt(&candidate);
    let signing_key = SigningKey::from_bytes(&[17_u8; 32]);
    let authenticated = authenticate_resident_tool_receipt_v1(
        &receipt,
        &hash('a'),
        &hash('c'),
        &hash('d'),
        "birthday-tool-key",
        &signing_key,
    )
    .unwrap();
    let trust = tool_trust("birthday-tool-key", &signing_key);
    let verified =
        validate_resident_tool_receipt_for_birthday_v1(&receipt, &authenticated, &trust).unwrap();
    let input = ProductionBirthdayInput {
        resident_id: "resident-one".into(),
        cycle_id: "cycle-one".into(),
        transaction_id: "birthday-one".into(),
        implementation_revision_sha256: hash('a'),
        identity_root_sha256: candidate.identity_root.clone(),
        continuity_head_sha256: candidate.continuity_head.clone(),
        memory_palace_authority_sha256: hash('b'),
        capability_envelope_sha256: hash('c'),
        cognitive_profile_sha256: hash('d'),
        adaptive_learning_receipt_sha256: hash('e'),
        birth_witness,
        trusted_time_unix_millis: 1_725_000_000_000,
        candidate,
        decision,
        tool_authority: verified,
    };
    let root = std::env::temp_dir().join(format!("adl-451-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let first = ProductionBirthdayStore::open(&root)
        .unwrap()
        .activate(&input)
        .unwrap();
    let restarted = ProductionBirthdayStore::open(&root).unwrap();
    assert_eq!(
        restarted.restore("resident-one").unwrap(),
        Some(first.clone())
    );
    assert_eq!(restarted.activate(&input).unwrap(), first);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn tampered_tool_receipt_cannot_enter_birthday_authority() {
    let candidate_path = format!(
        "{}/../adl-runtime-kernel/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(candidate_path).unwrap()).unwrap();
    let mut receipt = governed_receipt(&candidate);
    let signing_key = SigningKey::from_bytes(&[19_u8; 32]);
    let authenticated = authenticate_resident_tool_receipt_v1(
        &receipt,
        &hash('a'),
        &hash('c'),
        &hash('d'),
        "birthday-tool-key",
        &signing_key,
    )
    .unwrap();
    let trust = tool_trust("birthday-tool-key", &signing_key);
    receipt.reason_code = "rewritten_after_execution".into();
    assert!(
        validate_resident_tool_receipt_for_birthday_v1(&receipt, &authenticated, &trust).is_err()
    );

    let rogue_key = SigningKey::from_bytes(&[23_u8; 32]);
    let rogue = authenticate_resident_tool_receipt_v1(
        &governed_receipt(&candidate),
        &hash('a'),
        &hash('c'),
        &hash('d'),
        "rogue-tool-key",
        &rogue_key,
    )
    .unwrap();
    assert!(validate_resident_tool_receipt_for_birthday_v1(
        &governed_receipt(&candidate),
        &rogue,
        &trust
    )
    .is_err());
}
