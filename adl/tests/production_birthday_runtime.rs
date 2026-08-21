use std::fs;

use adl::resident_tool_execution::{
    authenticate_resident_tool_receipt_v1, validate_resident_tool_receipt_for_birthday_v1,
    ResidentToolReceiptDecisionV1, ResidentToolReceiptV1,
};
use adl_runtime_kernel::{
    decide_birthday, BirthdayCandidate, ProductionBirthdayInput, ProductionBirthdayStore,
};
use ed25519_dalek::SigningKey;

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

#[test]
fn authenticated_runtime_receipt_drives_exactly_once_birthday_and_restart() {
    let candidate_path = format!(
        "{}/../adl-runtime-kernel/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(candidate_path).unwrap()).unwrap();
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
    let verified = validate_resident_tool_receipt_for_birthday_v1(
        &receipt,
        &authenticated,
        &signing_key.verifying_key(),
    )
    .unwrap();
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
        witness_packet_sha256: hash('f'),
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
    receipt.reason_code = "rewritten_after_execution".into();
    assert!(validate_resident_tool_receipt_for_birthday_v1(
        &receipt,
        &authenticated,
        &signing_key.verifying_key()
    )
    .is_err());
}
