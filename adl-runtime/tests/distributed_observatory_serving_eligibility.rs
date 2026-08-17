#![cfg(feature = "internal-test-fixtures")]
use adl_runtime::distributed::{
    authority_protocol::{
        test_observatory_published_authority_for_operation, PublishedAuthorityResult,
    },
    observatory_serving_eligibility::{ObservatoryEligibilityError, ObservatoryEligibilityStore},
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    serving_authority::{
        ObservatoryBindingFixture, ObservatoryIdentifierField, ObservatoryTransitionAction,
        VerifiedServingAuthorityCut,
    },
    shepherd_serving_eligibility::{
        verify_committed_child_lineage_pair, ShepherdEligibilityError, ShepherdEligibilityStore,
    },
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
    fn load(&self, o: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.0.lock().unwrap().get(o).cloned())
    }
    fn compare_and_swap(
        &self,
        e: Option<&ConsensusCheckpoint>,
        c: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut m = self.0.lock().unwrap();
        if m.get(&c.object) != e {
            return Err(PolisRuntimeError::StateRegression);
        }
        m.insert(c.object.clone(), c.clone());
        Ok(())
    }
}
fn pair(
    s: &str,
    operation: &str,
    index: u64,
    a: ObservatoryTransitionAction,
    p: Option<&str>,
) -> (
    PublishedAuthorityResult,
    VerifiedServingAuthorityCut,
    ObservatoryBindingFixture,
) {
    let mut f = ObservatoryBindingFixture::new(s);
    f.set_invalid_identifier(ObservatoryIdentifierField::LineageId, "lineage-shared");
    f.set_invalid_identifier(ObservatoryIdentifierField::OperationId, operation);
    let fence = match a {
        ObservatoryTransitionAction::Acquire => 1,
        ObservatoryTransitionAction::Renew => 2,
        ObservatoryTransitionAction::Transfer => 3,
        ObservatoryTransitionAction::Revoke => 4,
    };
    f.set_operation(operation, index);
    f.set_integers(index, 1, fence);
    f.set_transition(a, p);
    let authority =
        test_observatory_published_authority_for_operation(f.artifact_bytes(), operation, index);
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&f);
    (authority, cut, f)
}
fn open(d: &TempDir, a: Arc<MemoryAuthority>) -> ObservatoryEligibilityStore {
    ObservatoryEligibilityStore::open(&d.path().canonicalize().unwrap(), a, 64).unwrap()
}
fn open_shepherd(d: &TempDir, a: Arc<MemoryAuthority>) -> ShepherdEligibilityStore {
    ShepherdEligibilityStore::open(&d.path().canonicalize().unwrap(), a, 8).unwrap()
}
fn shepherd_cut(lineage: &str) -> VerifiedServingAuthorityCut {
    VerifiedServingAuthorityCut::fixture(
        lineage.into(),
        7,
        "owner-commit".into(),
        9,
        "lease-1".into(),
        "11".repeat(32),
        "22".repeat(32),
        "33".repeat(32),
    )
}

#[test]
fn authentic_child_lineage_pairing_survives_restart_and_rejects_real_ab_stores() {
    let shepherd_dir = TempDir::new().unwrap();
    let observatory_dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    {
        let mut shepherd = open_shepherd(&shepherd_dir, authority.clone());
        shepherd
            .acquire(
                "shepherd-a",
                "shepherd-a",
                b"permit",
                &shepherd_cut("lineage-shared"),
                100,
                1,
            )
            .unwrap();
        let mut observatory = open(&observatory_dir, authority.clone());
        let (published, cut, _) = pair(
            "paired",
            "paired",
            2,
            ObservatoryTransitionAction::Acquire,
            None,
        );
        observatory
            .apply(&published, &cut, 1_700_000_000, 123_456_789)
            .unwrap();
        let shepherd_sealed = shepherd.committed_projection().unwrap().unwrap();
        let observatory_sealed = observatory.committed_projection().unwrap().unwrap();
        let pair =
            verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
        assert!(std::ptr::eq(pair.shepherd(), &shepherd_sealed));
        assert!(std::ptr::eq(pair.observatory(), &observatory_sealed));
    }
    let shepherd = open_shepherd(&shepherd_dir, authority.clone());
    let observatory = open(&observatory_dir, authority.clone());
    let shepherd_sealed = shepherd.committed_projection().unwrap().unwrap();
    let observatory_sealed = observatory.committed_projection().unwrap().unwrap();
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    assert!(std::ptr::eq(pair.shepherd(), &shepherd_sealed));
    assert!(std::ptr::eq(pair.observatory(), &observatory_sealed));

    let other_dir = TempDir::new().unwrap();
    let other_authority = Arc::new(MemoryAuthority::default());
    let mut other = open_shepherd(&other_dir, other_authority.clone());
    other
        .acquire(
            "shepherd-b",
            "shepherd-b",
            b"permit",
            &shepherd_cut("lineage-other"),
            100,
            1,
        )
        .unwrap();
    let other_sealed = other.committed_projection().unwrap().unwrap();
    assert!(matches!(
        verify_committed_child_lineage_pair(&other_sealed, &observatory_sealed),
        Err(ShepherdEligibilityError::StaleAuthority)
    ));
    drop(other);
    let reopened_other = open_shepherd(&other_dir, other_authority);
    assert!(matches!(
        verify_committed_child_lineage_pair(
            &reopened_other.committed_projection().unwrap().unwrap(),
            &observatory_sealed,
        ),
        Err(ShepherdEligibilityError::StaleAuthority)
    ));
}
#[test]
fn authenticated_lifecycle_and_restart_are_monotone() {
    let d = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut s = open(&d, auth.clone());
    let (a, c, _) = pair(
        "a",
        "acquire",
        2,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    let deadline = 1_700_000_000;
    let p = s.apply(&a, &c, deadline, 123_456_789).unwrap();
    assert_eq!(p.status, "eligible");
    let (r, rc, _) = pair(
        "r",
        "renew",
        3,
        ObservatoryTransitionAction::Renew,
        Some("acquire"),
    );
    let renewed = s.apply(&r, &rc, deadline, 123_456_789).unwrap();
    assert_ne!(renewed.operation_ref, p.operation_ref);
    let (t, tc, _) = pair(
        "t",
        "transfer",
        4,
        ObservatoryTransitionAction::Transfer,
        Some("renew"),
    );
    let transferred = s.apply(&t, &tc, deadline, 123_456_789).unwrap();
    assert_ne!(transferred.operation_ref, renewed.operation_ref);
    let (superseded, superseded_cut, _) = pair(
        "superseded",
        "superseded",
        5,
        ObservatoryTransitionAction::Renew,
        Some("acquire"),
    );
    assert_eq!(
        s.apply(&superseded, &superseded_cut, deadline, 123_456_789),
        Err(ObservatoryEligibilityError::StaleAuthority)
    );
    let (v, vc, _) = pair(
        "v",
        "revoke",
        6,
        ObservatoryTransitionAction::Revoke,
        Some("transfer"),
    );
    let revoked = s.apply(&v, &vc, deadline, 123_456_789).unwrap();
    assert_eq!(revoked.status, "revoked");
    let sealed = s.committed_projection().unwrap().unwrap();
    assert_eq!(sealed.child_kind(), "observatory");
    assert_eq!(sealed.status(), "revoked");
    assert_eq!(sealed.receipt_sha256(), revoked.receipt_sha256);
    let (revive, revive_cut, _) = pair(
        "revive",
        "revive",
        7,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    assert_eq!(
        s.apply(&revive, &revive_cut, deadline, 123_456_789),
        Err(ObservatoryEligibilityError::StaleAuthority)
    );
    assert_eq!(s.apply(&v, &vc, deadline, 123_456_789).unwrap(), revoked);
    drop(s);
    let mut s = open(&d, auth);
    let reopened = s.committed_projection().unwrap().unwrap();
    assert_eq!(
        reopened.canonical_bytes().unwrap(),
        sealed.canonical_bytes().unwrap()
    );
    assert_eq!(reopened.provenance_sha256(), sealed.provenance_sha256());
    let sealed_text = String::from_utf8(sealed.canonical_bytes().unwrap()).unwrap();
    assert!(!sealed_text.contains("trust-v"));
    assert!(!sealed_text.contains("polis-v"));
    assert_eq!(s.apply(&v, &vc, deadline, 123_456_789).unwrap(), revoked);
}

#[test]
fn empty_store_has_no_committed_projection() {
    let d = TempDir::new().unwrap();
    let s = open(&d, Arc::new(MemoryAuthority::default()));
    assert!(s.committed_projection().unwrap().is_none());
}

#[test]
fn corrupt_durable_payload_cannot_yield_committed_projection() {
    let d = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    {
        let mut s = open(&d, auth.clone());
        let (a, c, _) = pair(
            "corrupt",
            "corrupt",
            2,
            ObservatoryTransitionAction::Acquire,
            None,
        );
        s.apply(&a, &c, 1_700_000_000, 123_456_789).unwrap();
    }
    let path = d.path().join("observatory-serving-eligibility.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["payload"]["revision"] = 99.into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(matches!(
        ObservatoryEligibilityStore::open(&d.path().canonicalize().unwrap(), auth, 64),
        Err(ObservatoryEligibilityError::Storage)
    ));
}
#[test]
fn stale_predecessor_overlap_and_conflicting_retry_fail_closed() {
    let d = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut s = open(&d, auth);
    let (a, c, _) = pair(
        "base",
        "base",
        2,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    let p = s.apply(&a, &c, 1_700_000_000, 123_456_789).unwrap();
    let (other, oc, _) = pair(
        "other",
        "other",
        3,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    assert_eq!(
        s.apply(&other, &oc, 1_700_000_000, 123_456_789),
        Err(ObservatoryEligibilityError::StaleAuthority)
    );
    let (r, rc, _) = pair(
        "wrong",
        "wrong",
        3,
        ObservatoryTransitionAction::Renew,
        Some("wrong-predecessor"),
    );
    assert_eq!(
        s.apply(&r, &rc, 1_700_000_000, 123_456_789),
        Err(ObservatoryEligibilityError::StaleAuthority)
    );
    assert_eq!(
        s.apply(&a, &c, 1_700_000_001, 0),
        Err(ObservatoryEligibilityError::RetryConflict)
    );
    assert!(p.operation_ref.is_some());
}
#[test]
fn inclusive_nanos_expiry_and_pair_mismatch_fail_closed() {
    let d = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let (a, c, _) = pair(
        "time",
        "time",
        2,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    let mut equal = open(&d, auth);
    assert_eq!(
        equal
            .apply(&a, &c, 1_700_000_000, 123_456_789)
            .unwrap()
            .status,
        "eligible"
    );
    let d2 = TempDir::new().unwrap();
    let mut later = open(&d2, Arc::new(MemoryAuthority::default()));
    assert_eq!(
        later.apply(&a, &c, 1_800_000_001, 0),
        Err(ObservatoryEligibilityError::StaleAuthority)
    );
    let (b, bc, _) = pair("b", "b", 3, ObservatoryTransitionAction::Acquire, None);
    let d3 = TempDir::new().unwrap();
    let mut mismatched = open(&d3, Arc::new(MemoryAuthority::default()));
    assert_eq!(
        mismatched.apply(&b, &c, 1_700_000_000, 123_456_789),
        Err(ObservatoryEligibilityError::InvalidAuthority)
    );
    assert!(mismatched
        .apply(&b, &bc, 1_700_000_000, 123_456_789)
        .is_ok());
}
#[test]
fn projection_is_redacted() {
    let d = TempDir::new().unwrap();
    let mut s = open(&d, Arc::new(MemoryAuthority::default()));
    let (a, c, _) = pair(
        "secret",
        "secret",
        2,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    let p = s.apply(&a, &c, 1_700_000_000, 123_456_789).unwrap();
    let bytes = serde_json::to_string(&p).unwrap();
    assert!(!bytes.contains("trust-secret"));
    assert!(!bytes.contains("polis-secret"));
    assert!(!bytes.contains("operation-secret"));
}
