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
};
use std::{
    collections::BTreeMap,
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
    assert_eq!(s.apply(&v, &vc, deadline, 123_456_789).unwrap(), revoked);
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
