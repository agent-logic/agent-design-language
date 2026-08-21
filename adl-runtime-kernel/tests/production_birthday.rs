use std::{
    fs,
    sync::{Arc, Barrier},
    thread,
};

use adl_runtime_kernel::{
    candidate_digest, decide_birthday, reviewed_evidence_set_digest, witness_signing_bytes,
    BirthWitnessAttestation, BirthWitnessRole, BirthdayCandidate, EvidenceKind,
    ProductionBirthdayError, ProductionBirthdayFailpoint, ProductionBirthdayInput,
    ProductionBirthdayStore, RuntimeInitConfig, VerifiedToolAuthorityBinding, WitnessDecision,
    BIRTH_WITNESS_ATTESTATION_SCHEMA, PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};

#[path = "support/runtime_init.rs"]
mod runtime_init;

fn input(transaction: &str) -> ProductionBirthdayInput {
    let fixture = format!(
        "{}/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(fixture).unwrap()).unwrap();
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
    candidate.packet_sha256 = candidate_digest(&candidate).unwrap();
    let decision = decide_birthday(&candidate);
    let evidence = reviewed_evidence_set_digest(&candidate).unwrap();
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
    let birth_witness = owner
        .build_validate_and_emit_verified(&candidate, &decision, 7, &attestations, |_| Ok(|| {}))
        .unwrap();
    let hash = |byte: char| byte.to_string().repeat(64);
    ProductionBirthdayInput {
        resident_id: "resident-one".into(),
        cycle_id: "cycle-one".into(),
        transaction_id: transaction.into(),
        implementation_revision_sha256: hash('a'),
        identity_root_sha256: candidate.identity_root.clone(),
        continuity_head_sha256: candidate.continuity_head.clone(),
        memory_palace_authority_sha256: hash('b'),
        capability_envelope_sha256: hash('c'),
        cognitive_profile_sha256: hash('d'),
        adaptive_learning_receipt_sha256: hash('e'),
        birth_witness,
        trusted_time_unix_millis: 1_725_000_000_000,
        tool_authority: VerifiedToolAuthorityBinding {
            schema: PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA.into(),
            resident_id: "resident-one".into(),
            cycle_id: "cycle-one".into(),
            continuity_head_sha256: candidate.continuity_head.clone(),
            capability_envelope_sha256: hash('c'),
            cognitive_profile_sha256: hash('d'),
            implementation_revision_sha256: hash('a'),
            decision: "executed".into(),
            authentication_key_id: "birthday-key-one".into(),
            receipt_sha256: hash('1'),
            authentication_sha256: hash('2'),
        },
        candidate,
        decision,
    }
}

#[test]
fn commits_once_and_restores_exact_receipt() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProductionBirthdayStore::open(temp.path()).unwrap();
    let value = input("tx-one");
    let receipt = store.activate(&value).unwrap();
    assert_eq!(store.activate(&value).unwrap(), receipt);
    assert_eq!(store.restore("resident-one").unwrap(), Some(receipt));
}

#[test]
fn rejects_denial_and_cross_binding_drift() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProductionBirthdayStore::open(temp.path()).unwrap();
    let mut denied = input("tx-denied");
    denied.decision.accepted = false;
    assert_eq!(
        store.activate(&denied),
        Err(ProductionBirthdayError::BirthdayDenied)
    );
    let mut drift = input("tx-drift");
    drift.tool_authority.resident_id = "other".into();
    assert_eq!(
        store.activate(&drift),
        Err(ProductionBirthdayError::CrossBindingMismatch)
    );
}

#[test]
fn recovers_each_durable_interruption_boundary() {
    for point in [
        ProductionBirthdayFailpoint::BeforeIntent,
        ProductionBirthdayFailpoint::AfterIntentSync,
        ProductionBirthdayFailpoint::AfterWitnessSync,
        ProductionBirthdayFailpoint::AfterReceiptSync,
        ProductionBirthdayFailpoint::BeforeCommitRename,
        ProductionBirthdayFailpoint::AfterCommitRename,
        ProductionBirthdayFailpoint::AfterDirectorySync,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let store = ProductionBirthdayStore::open(temp.path()).unwrap();
        let value = input("tx-recover");
        assert_eq!(
            store.activate_with_failpoint(&value, Some(point)),
            Err(ProductionBirthdayError::InjectedInterruption)
        );
        let recovered = if point == ProductionBirthdayFailpoint::BeforeIntent {
            store.activate(&value).unwrap()
        } else {
            store.recover_pending(&value).unwrap()
        };
        assert_eq!(store.restore("resident-one").unwrap(), Some(recovered));
    }

    let temp = tempfile::tempdir().unwrap();
    fs::write(temp.path().join("resident-one.lock"), b"abandoned").unwrap();
    let store = ProductionBirthdayStore::open(temp.path()).unwrap();
    assert!(store.activate(&input("tx-stale-lock")).is_ok());
}

#[test]
fn two_independent_stores_commit_one_lineage() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();
    let barrier = Arc::new(Barrier::new(2));
    let handles = ["tx-a", "tx-b"].map(|transaction| {
        let root = root.clone();
        let barrier = barrier.clone();
        thread::spawn(move || {
            let store = ProductionBirthdayStore::open(root).unwrap();
            let value = input(transaction);
            barrier.wait();
            store.activate(&value)
        })
    });
    let results = handles.map(|handle| handle.join().unwrap());
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(result, Err(ProductionBirthdayError::ConflictingTransaction)))
            .count(),
        1
    );
}

#[test]
fn rejects_changed_transaction_after_commit() {
    let temp = tempfile::tempdir().unwrap();
    let store = ProductionBirthdayStore::open(temp.path()).unwrap();
    store.activate(&input("tx-first")).unwrap();
    assert_eq!(
        store.activate(&input("tx-second")),
        Err(ProductionBirthdayError::AlreadyCommitted)
    );
    let mut rebound = input("tx-first");
    rebound.memory_palace_authority_sha256 = "9".repeat(64);
    assert_eq!(
        store.activate(&rebound),
        Err(ProductionBirthdayError::AlreadyCommitted)
    );
}
