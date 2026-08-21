use std::{
    fs,
    sync::{Arc, Barrier},
    thread,
};

use adl_runtime_kernel::{
    decide_birthday, BirthdayCandidate, ProductionBirthdayError, ProductionBirthdayFailpoint,
    ProductionBirthdayInput, ProductionBirthdayStore, VerifiedToolAuthorityBinding,
    PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA,
};

fn input(transaction: &str) -> ProductionBirthdayInput {
    let fixture = format!(
        "{}/tests/fixtures/birthday/valid.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let candidate: BirthdayCandidate =
        serde_json::from_str(&fs::read_to_string(fixture).unwrap()).unwrap();
    let decision = decide_birthday(&candidate);
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
        witness_packet_sha256: hash('f'),
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
        ProductionBirthdayFailpoint::AfterIntentSync,
        ProductionBirthdayFailpoint::AfterReceiptSync,
        ProductionBirthdayFailpoint::AfterCommitRename,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let store = ProductionBirthdayStore::open(temp.path()).unwrap();
        let value = input("tx-recover");
        assert_eq!(
            store.activate_with_failpoint(&value, Some(point)),
            Err(ProductionBirthdayError::InjectedInterruption)
        );
        let recovered = store.recover_pending(&value).unwrap();
        assert_eq!(store.restore("resident-one").unwrap(), Some(recovered));
    }
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
}
