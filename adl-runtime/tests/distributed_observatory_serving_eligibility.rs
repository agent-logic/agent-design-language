#![cfg(feature = "internal-test-fixtures")]
use adl_runtime::distributed::{
    authority_protocol::{test_observatory_published_authority, PublishedAuthorityResult},
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
    a: ObservatoryTransitionAction,
    p: Option<&str>,
) -> (
    PublishedAuthorityResult,
    VerifiedServingAuthorityCut,
    ObservatoryBindingFixture,
) {
    let mut f = ObservatoryBindingFixture::new(s);
    f.set_invalid_identifier(ObservatoryIdentifierField::LineageId, "lineage-shared");
    f.set_invalid_identifier(ObservatoryIdentifierField::OperationId, "operation");
    f.set_integers(2, 1, 1);
    f.set_transition(a, p);
    let authority = test_observatory_published_authority(f.artifact_bytes());
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
    let (a, c, _) = pair("a", ObservatoryTransitionAction::Acquire, None);
    let deadline = 1_700_000_000;
    let p = s.apply(&a, &c, deadline, 123_456_789).unwrap();
    assert_eq!(p.status, "eligible");
    assert_eq!(s.apply(&a, &c, deadline, 123_456_789).unwrap(), p);
    drop(s);
    let mut s = open(&d, auth);
    assert_eq!(s.apply(&a, &c, deadline, 123_456_789).unwrap(), p);
}
#[test]
fn stale_predecessor_overlap_and_conflicting_retry_fail_closed() {
    let d = TempDir::new().unwrap();
    let auth = Arc::new(MemoryAuthority::default());
    let mut s = open(&d, auth);
    let (a, c, _) = pair("base", ObservatoryTransitionAction::Acquire, None);
    let p = s.apply(&a, &c, 1_700_000_000, 123_456_789).unwrap();
    let (other, oc, _) = pair("other", ObservatoryTransitionAction::Acquire, None);
    assert_eq!(
        s.apply(&other, &oc, 1_700_000_000, 123_456_789),
        Err(ObservatoryEligibilityError::RetryConflict)
    );
    let (r, rc, _) = pair(
        "wrong",
        ObservatoryTransitionAction::Renew,
        Some("wrong-predecessor"),
    );
    assert_eq!(
        s.apply(&r, &rc, 1_700_000_000, 123_456_789),
        Err(ObservatoryEligibilityError::RetryConflict)
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
    let (a, c, _) = pair("time", ObservatoryTransitionAction::Acquire, None);
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
    let (b, bc, _) = pair("b", ObservatoryTransitionAction::Acquire, None);
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
    let (a, c, _) = pair("secret", ObservatoryTransitionAction::Acquire, None);
    let p = s.apply(&a, &c, 1_700_000_000, 123_456_789).unwrap();
    let bytes = serde_json::to_string(&p).unwrap();
    assert!(!bytes.contains("trust-secret"));
    assert!(!bytes.contains("polis-secret"));
    assert!(!bytes.contains("operation-secret"));
}
