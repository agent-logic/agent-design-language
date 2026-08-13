use std::{
    collections::BTreeMap,
    fs,
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    serving_authority::{
        empty_state_sha256, ServingAuthorityBinding, ServingAuthorityError,
        ServingAuthorityIdentity, ServingAuthorityReceiptFixture, ServingAuthorityStore,
        ServingAuthorityTestBoundary,
    },
};
use sha2::{Digest, Sha256};
use tempfile::TempDir;

#[derive(Default)]
struct MemoryAuthority(Mutex<BTreeMap<String, ConsensusCheckpoint>>);

type ReceiptMutation = Box<dyn Fn(&mut ServingAuthorityReceiptFixture)>;

impl ConsensusCheckpointAuthority for MemoryAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.0.lock().unwrap().get(object).cloned())
    }
    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut values = self.0.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn identity() -> ServingAuthorityIdentity {
    ServingAuthorityIdentity {
        trust_domain: "trust.example".into(),
        polis_id: "polis-1".into(),
        node_id: "node-1".into(),
        guardian_id: "guardian-1".into(),
        boot_generation: 1,
    }
}

fn root(dir: &TempDir) -> std::path::PathBuf {
    dir.path().canonicalize().unwrap()
}

fn binding(operation: &str, prior: String) -> ServingAuthorityBinding {
    ServingAuthorityBinding::new(
        "trust.example".into(),
        "polis-1".into(),
        "lineage-1".into(),
        operation.into(),
        "adl.runtime.store.v1".into(),
        1,
        "owner_commit".into(),
        7,
        "owner-commit-secret".into(),
        9,
        "lease-secret".into(),
        prior,
        "22".repeat(32),
        "33".repeat(32),
    )
}

fn receipt(binding: &ServingAuthorityBinding) -> ServingAuthorityReceiptFixture {
    ServingAuthorityReceiptFixture {
        lineage_id: "lineage-1".into(),
        operation_id: "operation-1".into(),
        action_class: "owner_commit".into(),
        adapter_kind: "adl.runtime.store.v1".into(),
        adapter_version: 1,
        generation: 7,
        receipt_sha256: [0x33; 32],
        result_sha256: Sha256::digest(binding.canonical_preimage().unwrap()).into(),
    }
}

#[test]
fn publishes_only_after_exact_sealed_reconciliation_and_retries_exactly() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let root = root(&dir);
    let mut store = ServingAuthorityStore::open(&root, identity(), authority.clone(), 1).unwrap();
    let candidate = binding("operation-1", empty_state_sha256());
    let sealed = receipt(&candidate);
    let first = store
        .reconcile_and_publish_fixture(&sealed, &candidate)
        .unwrap();
    assert_eq!(first.readiness, "published");
    assert_eq!(first.generation, 7);
    assert_eq!(first.receipt_digest, "33".repeat(32));
    assert_eq!(
        store
            .reconcile_and_publish_fixture(&sealed, &candidate)
            .unwrap(),
        first
    );

    let json = serde_json::to_string(&first).unwrap();
    for secret in [
        "owner-commit-secret",
        "lease-secret",
        "trust.example",
        "polis-1",
        "lineage-1",
    ] {
        assert!(!json.contains(secret));
    }
    drop(store);
    let mut reopened = ServingAuthorityStore::open(&root, identity(), authority, 1).unwrap();
    assert_eq!(
        reopened
            .reconcile_and_publish_fixture(&sealed, &candidate)
            .unwrap(),
        first
    );
}

#[test]
fn rejects_every_direct_binding_mismatch_without_publication() {
    let mut cases: Vec<ReceiptMutation> = vec![
        Box::new(|r| r.lineage_id = "wrong".into()),
        Box::new(|r| r.operation_id = "wrong".into()),
        Box::new(|r| r.adapter_kind = "wrong".into()),
        Box::new(|r| r.adapter_version += 1),
        Box::new(|r| r.action_class = "wrong".into()),
        Box::new(|r| r.generation += 1),
        Box::new(|r| r.receipt_sha256[0] ^= 1),
        Box::new(|r| r.result_sha256[0] ^= 1),
    ];
    for mutate in cases.drain(..) {
        let dir = TempDir::new().unwrap();
        let authority = Arc::new(MemoryAuthority::default());
        let mut store = ServingAuthorityStore::open(&root(&dir), identity(), authority, 1).unwrap();
        let candidate = binding("operation-1", empty_state_sha256());
        let mut sealed = receipt(&candidate);
        mutate(&mut sealed);
        assert_eq!(
            store.reconcile_and_publish_fixture(&sealed, &candidate),
            Err(ServingAuthorityError::ReceiptMismatch)
        );
    }
}

#[test]
fn fails_closed_for_prior_state_conflict_capacity_and_corruption() {
    let dir = TempDir::new().unwrap();
    let authority = Arc::new(MemoryAuthority::default());
    let root = root(&dir);
    let mut store = ServingAuthorityStore::open(&root, identity(), authority.clone(), 1).unwrap();
    let wrong_prior = binding("operation-1", "aa".repeat(32));
    let sealed = receipt(&wrong_prior);
    assert_eq!(
        store.reconcile_and_publish_fixture(&sealed, &wrong_prior),
        Err(ServingAuthorityError::PriorStateMismatch)
    );
    let first = binding("operation-1", empty_state_sha256());
    let sealed = receipt(&first);
    store
        .reconcile_and_publish_fixture(&sealed, &first)
        .unwrap();
    let second = binding("operation-2", "bb".repeat(32));
    let mut second_receipt = receipt(&second);
    second_receipt.operation_id = "operation-2".into();
    second_receipt.result_sha256 = Sha256::digest(second.canonical_preimage().unwrap()).into();
    assert_eq!(
        store.reconcile_and_publish_fixture(&second_receipt, &second),
        Err(ServingAuthorityError::CapacityExceeded)
    );
    drop(store);
    fs::write(root.join("serving-authority.json"), b"{corrupt").unwrap();
    assert!(ServingAuthorityStore::open(&root, identity(), authority, 1).is_err());
}

#[cfg(unix)]
#[test]
fn unsafe_symlink_store_root_is_rejected() {
    use std::os::unix::fs::symlink;
    let outer = TempDir::new().unwrap();
    let target = TempDir::new().unwrap();
    let link = outer.path().join("linked");
    symlink(target.path(), &link).unwrap();
    assert!(ServingAuthorityStore::open(
        &link,
        identity(),
        Arc::new(MemoryAuthority::default()),
        1
    )
    .is_err());
}

#[test]
fn restart_from_pending_and_reconciled_resumes_before_publication() {
    for boundary in [
        ServingAuthorityTestBoundary::Pending,
        ServingAuthorityTestBoundary::Reconciled,
    ] {
        let dir = TempDir::new().unwrap();
        let root = root(&dir);
        let authority = Arc::new(MemoryAuthority::default());
        let candidate = binding("operation-1", empty_state_sha256());
        let sealed = receipt(&candidate);
        let mut store =
            ServingAuthorityStore::open(&root, identity(), authority.clone(), 1).unwrap();
        store
            .persist_fixture_boundary(&sealed, &candidate, boundary)
            .unwrap();
        drop(store);
        let mut reopened = ServingAuthorityStore::open(&root, identity(), authority, 1).unwrap();
        let projection = reopened
            .reconcile_and_publish_fixture(&sealed, &candidate)
            .unwrap();
        assert_eq!(projection.readiness, "published");
    }
}
