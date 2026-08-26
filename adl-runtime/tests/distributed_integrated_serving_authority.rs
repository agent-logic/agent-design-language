#![cfg(feature = "internal-test-fixtures")]

use adl_runtime::distributed::{
    authority_protocol::{
        test_observatory_published_authority_for_operation, PublishedAuthorityResult,
    },
    integrated_serving_authority_snapshot::{
        IntegratedOutcome, IntegratedServingAuthoritySnapshotStore, IntegratedSnapshotError,
    },
    observatory_serving_eligibility::ObservatoryEligibilityStore,
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    serving_authority::{
        ObservatoryBindingFixture, ObservatoryIdentifierField, ObservatoryTransitionAction,
        VerifiedServingAuthorityCut,
    },
    shepherd_serving_eligibility::{verify_committed_child_lineage_pair, ShepherdEligibilityStore},
};
use std::{
    collections::BTreeMap,
    fs,
    sync::{Arc, Mutex},
};
use tempfile::TempDir;

#[derive(Default)]
struct MemoryAuthority {
    checkpoints: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
    reject_next_cas: Mutex<bool>,
}
impl MemoryAuthority {
    fn reject_next_compare_and_swap(&self) {
        *self.reject_next_cas.lock().unwrap() = true;
    }

    fn poison_checkpoint(&self, object: &str) {
        let mut checkpoints = self.checkpoints.lock().unwrap();
        let checkpoint = checkpoints.get_mut(object).unwrap();
        checkpoint.payload_sha256 = "00".repeat(32);
    }
}
impl ConsensusCheckpointAuthority for MemoryAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.checkpoints.lock().unwrap().get(object).cloned())
    }
    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        if std::mem::take(&mut *self.reject_next_cas.lock().unwrap()) {
            return Err(PolisRuntimeError::StateRegression);
        }
        let mut values = self.checkpoints.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn observatory_input_for(
    operation: &str,
    lineage: &str,
    action: ObservatoryTransitionAction,
    predecessor: Option<&str>,
    committed: u64,
    generation: u64,
    fence: u64,
) -> (PublishedAuthorityResult, VerifiedServingAuthorityCut) {
    let mut fixture = ObservatoryBindingFixture::new(operation);
    fixture.set_invalid_identifier(ObservatoryIdentifierField::LineageId, lineage);
    fixture.set_invalid_identifier(ObservatoryIdentifierField::OperationId, operation);
    fixture.set_operation(operation, committed);
    fixture.set_integers(committed, generation, fence);
    fixture.set_transition(action, predecessor);
    (
        test_observatory_published_authority_for_operation(
            fixture.artifact_bytes(),
            operation,
            committed,
        ),
        VerifiedServingAuthorityCut::fixture_from_observatory(&fixture),
    )
}

fn observatory_input(operation: &str) -> (PublishedAuthorityResult, VerifiedServingAuthorityCut) {
    observatory_input_for(
        operation,
        "lineage-shared",
        ObservatoryTransitionAction::Acquire,
        None,
        2,
        1,
        1,
    )
}

fn shepherd_cut_for(lineage: &str, generation: u64, fence: u64) -> VerifiedServingAuthorityCut {
    VerifiedServingAuthorityCut::fixture(
        lineage.into(),
        generation,
        format!("owner-{lineage}"),
        fence,
        format!("lease-{lineage}"),
        "11".repeat(32),
        "22".repeat(32),
        "33".repeat(32),
    )
}

fn shepherd_cut() -> VerifiedServingAuthorityCut {
    shepherd_cut_for("lineage-shared", 7, 9)
}

fn committed_pair(
    lineage: &str,
    shepherd_terminal: Option<&str>,
    observatory_terminal: Option<ObservatoryTransitionAction>,
) -> (
    adl_runtime::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection,
    adl_runtime::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection,
) {
    committed_pair_with_versions(
        lineage,
        7,
        9,
        100,
        2,
        1,
        1,
        shepherd_terminal,
        observatory_terminal,
    )
}

#[allow(clippy::too_many_arguments)]
fn committed_pair_with_versions(
    lineage: &str,
    shepherd_generation: u64,
    shepherd_fence: u64,
    shepherd_committed: u64,
    observatory_committed: u64,
    observatory_generation: u64,
    observatory_fence: u64,
    shepherd_terminal: Option<&str>,
    observatory_terminal: Option<ObservatoryTransitionAction>,
) -> (
    adl_runtime::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection,
    adl_runtime::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection,
) {
    let shepherd_dir = TempDir::new().unwrap();
    let observatory_dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let mut shepherd = ShepherdEligibilityStore::open(
        &shepherd_dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    let shepherd_cut = shepherd_cut_for(lineage, shepherd_generation, shepherd_fence);
    shepherd
        .acquire(
            "shepherd-acquire",
            "shepherd-a",
            b"raw-secret-permit",
            &shepherd_cut,
            shepherd_committed,
            1,
        )
        .unwrap();
    match shepherd_terminal {
        Some("revoke") => {
            shepherd.revoke("shepherd-revoke", &shepherd_cut).unwrap();
        }
        Some("expire") => {
            shepherd
                .expire("shepherd-expire", &shepherd_cut, 101)
                .unwrap();
        }
        None => {}
        Some(other) => panic!("unknown shepherd terminal fixture {other}"),
    }

    let mut observatory = ObservatoryEligibilityStore::open(
        &observatory_dir.path().canonicalize().unwrap(),
        authority,
        8,
    )
    .unwrap();
    let (published, cut) = observatory_input_for(
        "observatory-acquire",
        lineage,
        ObservatoryTransitionAction::Acquire,
        None,
        observatory_committed,
        observatory_generation,
        observatory_fence,
    );
    observatory
        .apply(&published, &cut, 1_700_000_000, 123_456_789)
        .unwrap();
    if let Some(action) = observatory_terminal {
        let (published, cut) = observatory_input_for(
            "observatory-terminal",
            lineage,
            action,
            Some("observatory-acquire"),
            observatory_committed + 1,
            observatory_generation + 1,
            observatory_fence + 1,
        );
        observatory
            .apply(&published, &cut, 1_700_000_001, 123_456_789)
            .unwrap();
    }
    (
        shepherd.committed_projection().unwrap().unwrap(),
        observatory.committed_projection().unwrap().unwrap(),
    )
}

fn verified_pair_for(
    lineage: &str,
) -> (
    adl_runtime::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection,
    adl_runtime::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection,
) {
    committed_pair(lineage, None, None)
}

#[test]
fn authentic_pair_snapshot_retry_restart_and_redaction() {
    let shepherd_dir = TempDir::new().unwrap();
    let observatory_dir = TempDir::new().unwrap();
    let integrated_dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let mut shepherd = ShepherdEligibilityStore::open(
        &shepherd_dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    shepherd
        .acquire(
            "shepherd-acquire",
            "shepherd-a",
            b"raw-secret-permit",
            &shepherd_cut(),
            100,
            1,
        )
        .unwrap();
    let mut observatory = ObservatoryEligibilityStore::open(
        &observatory_dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    let (published, cut) = observatory_input("observatory-acquire");
    observatory
        .apply(&published, &cut, 1_700_000_000, 123_456_789)
        .unwrap();
    let shepherd_sealed = shepherd.committed_projection().unwrap().unwrap();
    let observatory_sealed = observatory.committed_projection().unwrap().unwrap();
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &integrated_dir.path().canonicalize().unwrap(),
        authority.clone(),
        4,
    )
    .unwrap();
    let first = store
        .observe("integrate-1", &pair, IntegratedOutcome::Success)
        .unwrap();
    assert_eq!(
        store
            .observe("integrate-1", &pair, IntegratedOutcome::Success)
            .unwrap(),
        first
    );
    assert_eq!(
        store.observe("integrate-1", &pair, IntegratedOutcome::NoOp),
        Err(IntegratedSnapshotError::RetryConflict)
    );
    let bytes = serde_jcs::to_vec(&first).unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    assert!(!text.contains("raw-secret-permit"));
    assert!(!text.contains("owner-commit"));
    drop(store);
    let reopened = IntegratedServingAuthoritySnapshotStore::open(
        &integrated_dir.path().canonicalize().unwrap(),
        authority,
        4,
    )
    .unwrap();
    assert_eq!(
        serde_jcs::to_vec(reopened.receipt("integrate-1").unwrap()).unwrap(),
        bytes
    );
}

#[test]
fn immutable_multi_operation_prefix_and_four_outcomes() {
    let integrated_dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &integrated_dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    let versions = [
        (
            "prefix-success",
            IntegratedOutcome::Success,
            7,
            9,
            100,
            2,
            1,
            1,
        ),
        ("prefix-noop", IntegratedOutcome::NoOp, 8, 10, 101, 3, 2, 2),
        (
            "prefix-rejection",
            IntegratedOutcome::Rejection,
            9,
            11,
            102,
            4,
            3,
            3,
        ),
    ];
    let mut observed_digests = Vec::new();
    for (
        operation,
        outcome,
        shepherd_generation,
        shepherd_fence,
        shepherd_committed,
        observatory_committed,
        observatory_generation,
        observatory_fence,
    ) in versions
    {
        let (shepherd_sealed, observatory_sealed) = committed_pair_with_versions(
            "lineage-prefix",
            shepherd_generation,
            shepherd_fence,
            shepherd_committed,
            observatory_committed,
            observatory_generation,
            observatory_fence,
            None,
            None,
        );
        let pair =
            verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
        let receipt = store.observe(operation, &pair, outcome).unwrap();
        assert_eq!(receipt.outcome, outcome);
        assert_ne!(receipt.prior_state_sha256, receipt.result_state_sha256);
        assert!(!observed_digests.contains(&receipt.result_state_sha256));
        observed_digests.push(receipt.result_state_sha256);
    }
    let (stale_shepherd, stale_observatory) =
        committed_pair_with_versions("lineage-prefix", 9, 11, 102, 4, 3, 3, None, None);
    let stale_pair =
        verify_committed_child_lineage_pair(&stale_shepherd, &stale_observatory).unwrap();
    assert_eq!(
        store.observe("prefix-stale", &stale_pair, IntegratedOutcome::Success),
        Err(IntegratedSnapshotError::InvalidInput)
    );
    assert_eq!(
        store.recover("prefix-live-recovery"),
        Err(IntegratedSnapshotError::InvalidInput)
    );
    assert_eq!(
        store.observe(
            "prefix-forged-recovery",
            &stale_pair,
            IntegratedOutcome::Recovery
        ),
        Err(IntegratedSnapshotError::InvalidInput)
    );
    drop(store);
    let mut reopened = IntegratedServingAuthoritySnapshotStore::open(
        &integrated_dir.path().canonicalize().unwrap(),
        authority,
        8,
    )
    .unwrap();
    let recovery = reopened.recover("prefix-recovery").unwrap();
    assert_eq!(recovery.outcome, IntegratedOutcome::Recovery);
    assert!(!observed_digests.contains(&recovery.result_state_sha256));
    assert_eq!(
        reopened.receipt("prefix-success").unwrap().outcome,
        IntegratedOutcome::Success
    );
}

#[test]
fn capacity_and_invalid_operation_fail_closed() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    assert!(matches!(
        IntegratedServingAuthoritySnapshotStore::open(
            &dir.path().canonicalize().unwrap(),
            authority,
            0
        ),
        Err(IntegratedSnapshotError::CapacityExceeded)
    ));
    let (shepherd_sealed, observatory_sealed) = verified_pair_for("lineage-invalid");
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        Arc::new(MemoryAuthority::default()),
        1,
    )
    .unwrap();
    assert_eq!(
        store.observe("", &pair, IntegratedOutcome::Success),
        Err(IntegratedSnapshotError::InvalidInput)
    );
    store
        .observe("capacity-one", &pair, IntegratedOutcome::Success)
        .unwrap();
    assert_eq!(
        store.observe("capacity-two", &pair, IntegratedOutcome::Success),
        Err(IntegratedSnapshotError::CapacityExceeded)
    );
}

#[test]
fn checkpoint_cas_failure_preserves_last_commit() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let (shepherd_sealed, observatory_sealed) = verified_pair_for("lineage-cas");
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    let first = store
        .observe("cas-before", &pair, IntegratedOutcome::Success)
        .unwrap();
    drop(store);
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        authority.clone(),
        8,
    )
    .unwrap();
    authority.reject_next_compare_and_swap();
    assert_eq!(
        store.recover("cas-after"),
        Err(IntegratedSnapshotError::Storage)
    );
    drop(store);
    let reopened = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        authority,
        8,
    )
    .unwrap();
    assert_eq!(reopened.receipt("cas-before"), Some(&first));
    assert!(reopened.receipt("cas-after").is_none());
}

#[test]
fn corrupt_truncated_and_unknown_state_fail_closed() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let (shepherd_sealed, observatory_sealed) = verified_pair_for("lineage-corrupt");
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    {
        let mut store = IntegratedServingAuthoritySnapshotStore::open(
            &dir.path().canonicalize().unwrap(),
            authority.clone(),
            8,
        )
        .unwrap();
        store
            .observe("corrupt-before", &pair, IntegratedOutcome::Success)
            .unwrap();
    }
    let path = dir
        .path()
        .join("integrated-serving-authority-snapshot.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["payload"]["unknown"] = true.into();
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(matches!(
        IntegratedServingAuthoritySnapshotStore::open(
            &dir.path().canonicalize().unwrap(),
            authority,
            8
        ),
        Err(IntegratedSnapshotError::Storage)
    ));

    let truncated_dir = TempDir::new().unwrap();
    let truncated_authority = Arc::new(MemoryAuthority::default());
    {
        let mut store = IntegratedServingAuthoritySnapshotStore::open(
            &truncated_dir.path().canonicalize().unwrap(),
            truncated_authority.clone(),
            8,
        )
        .unwrap();
        store
            .observe("truncate-before", &pair, IntegratedOutcome::Success)
            .unwrap();
    }
    fs::write(
        truncated_dir
            .path()
            .join("integrated-serving-authority-snapshot.json"),
        b"{",
    )
    .unwrap();
    assert!(matches!(
        IntegratedServingAuthoritySnapshotStore::open(
            &truncated_dir.path().canonicalize().unwrap(),
            truncated_authority,
            8
        ),
        Err(IntegratedSnapshotError::Storage)
    ));
}

#[test]
fn terminal_child_combinations_remain_evidence_only() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let mut store = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        authority,
        8,
    )
    .unwrap();
    for (
        operation,
        shepherd_terminal,
        observatory_terminal,
        shepherd_generation,
        shepherd_fence,
        shepherd_committed,
        observatory_committed,
        observatory_generation,
        observatory_fence,
        expected_shepherd_status,
        expected_observatory_status,
    ) in [
        (
            "terminal-active",
            None,
            None,
            7,
            9,
            100,
            2,
            1,
            1,
            "eligible",
            "eligible",
        ),
        (
            "terminal-renew-expire",
            Some("expire"),
            Some(ObservatoryTransitionAction::Renew),
            8,
            10,
            101,
            4,
            2,
            2,
            "expired",
            "eligible",
        ),
        (
            "terminal-transfer-revoke",
            Some("revoke"),
            Some(ObservatoryTransitionAction::Transfer),
            9,
            11,
            102,
            6,
            3,
            3,
            "revoked",
            "eligible",
        ),
        (
            "terminal-revoke",
            Some("revoke"),
            Some(ObservatoryTransitionAction::Revoke),
            10,
            12,
            103,
            8,
            4,
            4,
            "revoked",
            "revoked",
        ),
    ] {
        let (shepherd_sealed, observatory_sealed) = committed_pair_with_versions(
            "lineage-terminal",
            shepherd_generation,
            shepherd_fence,
            shepherd_committed,
            observatory_committed,
            observatory_generation,
            observatory_fence,
            shepherd_terminal,
            observatory_terminal,
        );
        let pair =
            verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
        let receipt = store
            .observe(operation, &pair, IntegratedOutcome::NoOp)
            .unwrap();
        assert_eq!(receipt.shepherd.status, expected_shepherd_status);
        assert_eq!(receipt.observatory.status, expected_observatory_status);
        assert_eq!(receipt.outcome, IntegratedOutcome::NoOp);
    }
}

#[test]
fn authentic_ab_substitution_is_denied_before_commit() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let (shepherd_a, _) = verified_pair_for("lineage-a");
    let (_, observatory_b) = verified_pair_for("lineage-b");
    assert!(verify_committed_child_lineage_pair(&shepherd_a, &observatory_b).is_err());
    let store = IntegratedServingAuthoritySnapshotStore::open(
        &dir.path().canonicalize().unwrap(),
        authority,
        8,
    )
    .unwrap();
    assert!(store.receipt("ab-substitution").is_none());
}

#[test]
fn independent_prefix_receipt_and_checkpoint_tamper_is_denied() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let (shepherd_sealed, observatory_sealed) = verified_pair_for("lineage-tamper");
    let pair = verify_committed_child_lineage_pair(&shepherd_sealed, &observatory_sealed).unwrap();
    {
        let mut store = IntegratedServingAuthoritySnapshotStore::open(
            &dir.path().canonicalize().unwrap(),
            authority.clone(),
            8,
        )
        .unwrap();
        store
            .observe("tamper-before", &pair, IntegratedOutcome::Success)
            .unwrap();
    }
    let path = dir
        .path()
        .join("integrated-serving-authority-snapshot.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["payload"]["operations"]["tamper-before"]["receipt_sha256"] =
        serde_json::Value::String("aa".repeat(32));
    fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(matches!(
        IntegratedServingAuthoritySnapshotStore::open(
            &dir.path().canonicalize().unwrap(),
            authority.clone(),
            8
        ),
        Err(IntegratedSnapshotError::Storage)
    ));

    let checkpoint_dir = TempDir::new().unwrap();
    let checkpoint_authority = Arc::new(MemoryAuthority::default());
    {
        let mut store = IntegratedServingAuthoritySnapshotStore::open(
            &checkpoint_dir.path().canonicalize().unwrap(),
            checkpoint_authority.clone(),
            8,
        )
        .unwrap();
        store
            .observe("checkpoint-before", &pair, IntegratedOutcome::Success)
            .unwrap();
    }
    checkpoint_authority.poison_checkpoint("integrated-serving-authority-snapshot");
    assert!(matches!(
        IntegratedServingAuthoritySnapshotStore::open(
            &checkpoint_dir.path().canonicalize().unwrap(),
            checkpoint_authority,
            8
        ),
        Err(IntegratedSnapshotError::Storage)
    ));
}
