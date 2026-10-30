#![cfg(feature = "internal-test-fixtures")]

use adl_runtime::distributed::{
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    serving_authority::VerifiedServingAuthorityCut,
    shepherd_serving_eligibility::{ShepherdEligibilityError, ShepherdEligibilityStore},
};
use std::{
    collections::BTreeMap,
    fs,
    sync::{Arc, Mutex},
};
use tempfile::TempDir;

#[derive(Default)]
struct MemoryAuthority(Mutex<BTreeMap<String, ConsensusCheckpoint>>);
impl ConsensusCheckpointAuthority for MemoryAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.0.lock().unwrap().get(object).cloned())
    }
    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut map = self.0.lock().unwrap();
        if map.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        map.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}
fn cut(fence: u64, state: &str) -> VerifiedServingAuthorityCut {
    cut_with_lineage("lineage-1", fence, state)
}
fn cut_with_lineage(lineage: &str, fence: u64, state: &str) -> VerifiedServingAuthorityCut {
    VerifiedServingAuthorityCut::fixture(
        lineage.into(),
        7,
        "owner-commit".into(),
        fence,
        "lease-1".into(),
        state.into(),
        "22".repeat(32),
        "33".repeat(32),
    )
}
fn open(
    dir: &TempDir,
    authority: Arc<MemoryAuthority>,
    capacity: usize,
) -> ShepherdEligibilityStore {
    ShepherdEligibilityStore::open(&dir.path().canonicalize().unwrap(), authority, capacity)
        .unwrap()
}

#[test]
fn acquire_retry_replace_never_exposes_two_shepherds() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 8);
    let p1 = store
        .acquire(
            "acquire-1",
            "shepherd-a",
            b"permit-a",
            &cut(9, "11"),
            100,
            1,
        )
        .unwrap();
    assert_eq!(p1.status, "eligible");
    let retry = store
        .acquire(
            "acquire-1",
            "shepherd-a",
            b"permit-a",
            &cut(9, "11"),
            100,
            1,
        )
        .unwrap();
    assert_eq!(retry.receipt_sha256, p1.receipt_sha256);
    assert_eq!(
        store.acquire(
            "acquire-2",
            "shepherd-b",
            b"permit-b",
            &cut(10, "12"),
            100,
            1
        ),
        Err(ShepherdEligibilityError::StaleAuthority)
    );
    let p2 = store
        .replace(
            "replace-1",
            "shepherd-b",
            b"permit-b",
            &cut(10, "12"),
            100,
            1,
        )
        .unwrap();
    assert_eq!(p2.status, "eligible");
    assert_ne!(p2.subject_ref, p1.subject_ref);
    assert_eq!(
        store
            .acquire(
                "acquire-1",
                "shepherd-a",
                b"permit-a",
                &cut(9, "11"),
                100,
                1,
            )
            .unwrap(),
        p1
    );
    let historical_replace = p2.clone();
    store
        .revoke("revoke-after-replace", &cut(10, "12"))
        .unwrap();
    assert_eq!(
        store
            .replace(
                "replace-1",
                "shepherd-b",
                b"permit-b",
                &cut(10, "12"),
                100,
                1,
            )
            .unwrap(),
        historical_replace
    );
}

#[test]
fn conflicting_retry_and_stale_fence_fail_closed() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 8);
    store
        .acquire("op", "shepherd-a", b"permit", &cut(4, "11"), 10, 1)
        .unwrap();
    assert_eq!(
        store.acquire("op", "shepherd-a", b"other", &cut(4, "11"), 10, 1),
        Err(ShepherdEligibilityError::RetryConflict)
    );
    assert_eq!(
        store.replace("replace", "shepherd-b", b"permit", &cut(4, "12"), 10, 1),
        Err(ShepherdEligibilityError::StaleAuthority)
    );
    assert_eq!(
        store.replace("op", "shepherd-a", b"permit", &cut(4, "11"), 10, 1),
        Err(ShepherdEligibilityError::RetryConflict)
    );
}

#[test]
fn revoke_and_expiry_are_terminal_for_old_cut() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 8);
    let c = cut(5, "11");
    store
        .acquire("a", "shepherd-a", b"permit", &c, 10, 1)
        .unwrap();
    let revoked = store.revoke("r", &c).unwrap();
    assert_eq!(revoked.status, "revoked");
    assert_eq!(
        store.acquire("again", "shepherd-a", b"permit", &c, 20, 2),
        Err(ShepherdEligibilityError::StaleAuthority)
    );
    let newer = cut(6, "12");
    store
        .acquire("new", "shepherd-b", b"permit2", &newer, 10, 1)
        .unwrap();
    assert_eq!(
        store.expire("early", &newer, 9),
        Err(ShepherdEligibilityError::StaleAuthority)
    );
    assert_eq!(
        store.expire("expire", &newer, 10).unwrap().status,
        "expired"
    );
    assert_eq!(store.revoke("r", &c).unwrap(), revoked);
    assert_eq!(
        store.revoke("r", &newer),
        Err(ShepherdEligibilityError::RetryConflict)
    );
}

#[test]
fn restart_recovers_exact_committed_projection() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let (first, sealed) = {
        let mut store = open(&dir, auth.clone(), 8);
        let projection = store
            .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
            .unwrap();
        let sealed = store.committed_projection().unwrap().unwrap();
        (projection, sealed)
    };
    let mut reopened = open(&dir, auth, 8);
    let reopened_sealed = reopened.committed_projection().unwrap().unwrap();
    assert_eq!(
        reopened_sealed.canonical_bytes().unwrap(),
        sealed.canonical_bytes().unwrap()
    );
    assert_eq!(
        reopened_sealed.provenance_sha256(),
        sealed.provenance_sha256()
    );
    assert_eq!(sealed.child_kind(), "shepherd");
    assert_eq!(sealed.committed_revision(), 1);
    assert_eq!(sealed.status(), "eligible");
    assert_eq!(
        sealed.lineage_ref(),
        Some(cut(5, "11").lineage_ref().as_str())
    );
    assert_eq!(sealed.receipt_sha256(), first.receipt_sha256);
    let sealed_text = String::from_utf8(sealed.canonical_bytes().unwrap()).unwrap();
    assert!(!sealed_text.contains("shepherd-a"));
    assert!(!sealed_text.contains("permit"));
    let retry = reopened
        .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
        .unwrap();
    assert_eq!(retry.receipt_sha256, first.receipt_sha256);
    assert_eq!(retry.state_sha256, first.state_sha256);
}

#[test]
fn authenticated_lineage_is_replaced_and_terminal_transitions_preserve_it() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 8);
    let first = cut_with_lineage("lineage-a", 5, "11");
    store
        .acquire("a", "shepherd-a", b"permit", &first, 10, 1)
        .unwrap();
    assert_eq!(
        store.committed_projection().unwrap().unwrap().lineage_ref(),
        Some(first.lineage_ref().as_str())
    );
    let second = cut_with_lineage("lineage-b", 6, "12");
    store
        .replace("replace", "shepherd-b", b"permit2", &second, 20, 2)
        .unwrap();
    assert_eq!(
        store.committed_projection().unwrap().unwrap().lineage_ref(),
        Some(second.lineage_ref().as_str())
    );
    store.revoke("revoke", &second).unwrap();
    assert_eq!(
        store.committed_projection().unwrap().unwrap().lineage_ref(),
        Some(second.lineage_ref().as_str())
    );
}

#[test]
fn empty_store_has_no_committed_projection() {
    let dir = TempDir::new().unwrap();
    let store = open(&dir, Arc::new(MemoryAuthority::default()), 8);
    assert!(store.committed_projection().unwrap().is_none());
}

#[test]
fn corrupt_durable_payload_cannot_yield_committed_projection() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    {
        let mut store = open(&dir, auth.clone(), 8);
        store
            .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
            .unwrap();
    }
    let path = dir.path().join("shepherd-serving-eligibility.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["payload"]["revision"] = 99.into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(matches!(
        ShepherdEligibilityStore::open(&dir.path().canonicalize().unwrap(), auth, 8),
        Err(ShepherdEligibilityError::Storage)
    ));
}

#[test]
fn capacity_failure_preserves_last_commit() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 1);
    let first = store
        .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
        .unwrap();
    assert_eq!(
        store.revoke("r", &cut(5, "11")),
        Err(ShepherdEligibilityError::CapacityExceeded)
    );
    let retry = store
        .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
        .unwrap();
    assert_eq!(retry.state_sha256, first.state_sha256);
}

#[test]
fn projection_is_redacted() {
    let dir = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut store = open(&dir, auth, 8);
    let p = store
        .acquire(
            "a",
            "shepherd-secret",
            b"permit-secret",
            &cut(5, "11"),
            10,
            1,
        )
        .unwrap();
    let bytes = serde_json::to_string(&p).unwrap();
    assert!(!bytes.contains("shepherd-secret"));
    assert!(!bytes.contains("permit-secret"));
    assert!(!bytes.contains("owner-commit"));
    assert!(!bytes.contains("lease-1"));
}
