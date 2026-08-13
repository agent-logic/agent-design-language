use adl_runtime::distributed::{
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    serving_authority::VerifiedServingAuthorityCut,
    shepherd_serving_eligibility::{ShepherdEligibilityError, ShepherdEligibilityStore},
};
use std::{
    collections::BTreeMap,
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
    VerifiedServingAuthorityCut::fixture(
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
    let first = {
        let mut store = open(&dir, auth.clone(), 8);
        store
            .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
            .unwrap()
    };
    let mut reopened = open(&dir, auth, 8);
    let retry = reopened
        .acquire("a", "shepherd-a", b"permit", &cut(5, "11"), 10, 1)
        .unwrap();
    assert_eq!(retry.receipt_sha256, first.receipt_sha256);
    assert_eq!(retry.state_sha256, first.state_sha256);
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
