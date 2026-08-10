use std::sync::Arc;

use adl_runtime_kernel::{
    KernelDurableState, KernelDurableStateError, KERNEL_DURABLE_STATE_DB_FILE,
};
use tempfile::TempDir;

#[test]
fn redb_local_checkpoint_persists_restores_and_rejects_identity_drift() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    let stored = state
        .store_local_checkpoint(
            "checkpoint_store",
            "checkpoint",
            "store-1",
            "alice",
            "writer-1",
            b"real-state",
        )
        .unwrap();
    assert_eq!(stored["generation"], 1);
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    let restored = reopened.restore_local_checkpoint("alice").unwrap();
    assert_eq!(restored["state_hex"], hex::encode(b"real-state"));
    assert!(reopened.restore_local_checkpoint("bob").is_err());
    assert!(root.path().join(KERNEL_DURABLE_STATE_DB_FILE).exists());
    assert!(!root.path().join("checkpoint.json").exists());
}

#[test]
fn governed_state_and_lifelog_survive_restart_without_flat_files() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    state
        .store_governed_state("parity-c", br#"{"state":1}"#)
        .unwrap();
    state
        .append_governed_lifelog(&serde_json::json!({
            "schema":"adl.runtime.parity_c.lifelog.v1",
            "request_id":"r1",
            "citizen_id":"alice",
            "action":"provider.invoke",
            "result_hash":"abc",
            "checkpoint_generation":1,
            "redacted_fields":["payload","keys"]
        }))
        .unwrap();
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    assert_eq!(
        reopened.load_governed_state("parity-c").unwrap().unwrap(),
        br#"{"state":1}"#
    );
    assert_eq!(reopened.governed_lifelog_len().unwrap(), 1);
    assert!(!root.path().join("lifelog.jsonl").exists());
}

#[test]
fn legacy_flat_persistence_fails_closed_before_database_open() {
    let root = TempDir::new().unwrap();
    std::fs::write(root.path().join("checkpoint.json"), b"legacy-state").unwrap();
    assert!(KernelDurableState::open(root.path()).is_err());
}

#[test]
fn concurrent_writes_advance_sequences_and_head_atomically() {
    let root = TempDir::new().unwrap();
    let state = Arc::new(KernelDurableState::open(root.path()).unwrap());
    let mut handles = Vec::new();
    for index in 0..16 {
        let state = state.clone();
        handles.push(std::thread::spawn(move || {
            let payload = format!("state-{index}");
            state
                .store_local_checkpoint(
                    "checkpoint_store",
                    "checkpoint",
                    &format!("store-{index}"),
                    "alice",
                    "writer-1",
                    payload.as_bytes(),
                )
                .unwrap();
            state
                .append_local_lifelog(
                    "lifelog",
                    "lifelog",
                    &format!("log-{index}"),
                    "alice",
                    payload.as_bytes(),
                    false,
                )
                .unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let restored = state.restore_local_checkpoint("alice").unwrap();
    assert_eq!(restored["generation"], 16);
    assert_eq!(state.local_lifelog_len().unwrap(), 16);
}

#[test]
fn communication_outbound_sequences_are_per_principal_and_survive_restart() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    assert_eq!(
        state
            .next_communication_outbound_sequence("agent-0001")
            .unwrap(),
        1
    );
    assert_eq!(
        state
            .next_communication_outbound_sequence("agent-0002")
            .unwrap(),
        1
    );
    assert_eq!(
        state
            .next_communication_outbound_sequence("agent-0001")
            .unwrap(),
        2
    );
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    assert_eq!(
        reopened
            .next_communication_outbound_sequence("agent-0001")
            .unwrap(),
        3
    );
    assert_eq!(
        reopened
            .next_communication_outbound_sequence("agent-0002")
            .unwrap(),
        2
    );
}

#[test]
fn communication_inbound_reservations_survive_restart_and_roll_back_definite_failures() {
    let root = TempDir::new().unwrap();
    let state = KernelDurableState::open(root.path()).unwrap();
    assert_eq!(
        state
            .reserve_communication_inbound_sequence("agent-0001", 7)
            .unwrap(),
        None
    );
    assert_eq!(
        state.communication_inbound_sequences().unwrap()["agent-0001"],
        7
    );
    drop(state);

    let reopened = KernelDurableState::open(root.path()).unwrap();
    assert!(matches!(
        reopened.reserve_communication_inbound_sequence("agent-0001", 7),
        Err(KernelDurableStateError::CommunicationSequenceConflict)
    ));
    assert_eq!(
        reopened
            .reserve_communication_inbound_sequence("agent-0001", 8)
            .unwrap(),
        Some(7)
    );
    reopened
        .rollback_communication_inbound_sequence("agent-0001", 8, Some(7))
        .unwrap();
    assert_eq!(
        reopened.communication_inbound_sequences().unwrap()["agent-0001"],
        7
    );
}
