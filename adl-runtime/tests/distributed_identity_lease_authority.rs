use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    authority_reconciliation::{AuthorityReconciliationBarrier, AuthorityReconciliationIdentity},
    authority_store_adapters::{AuthorityStoreAdapterError, AuthorityStoreAdapterRegistry},
    certificates::{CertificatePolicy, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS},
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
};
use ed25519_dalek::SigningKey;

// PVF: lane=identity-lease-fencing-authority-boundary; proof=#203 slice-1
// security-boundary guardrail; deterministic=true; resource_profile=small;
// release_gate=false.

#[derive(Default)]
struct MemoryCheckpoint {
    values: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
}

impl ConsensusCheckpointAuthority for MemoryCheckpoint {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.values.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut values = self.values.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn identity() -> AuthorityReconciliationIdentity {
    AuthorityReconciliationIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-a".to_owned(),
        guardian_id: "guardian-a".to_owned(),
        boot_generation: 7,
        protocol_instance: "adl.authority-reconciliation.v1".to_owned(),
    }
}

fn repo_local_root() -> tempfile::TempDir {
    let root = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .expect("current test directory must have a canonical symlink-free path");
    tempfile::TempDir::new_in(root).expect("portable repository-local test root")
}

fn assert_contains(source: &str, needle: &str) {
    assert!(
        source.contains(needle),
        "source guard missing expected snippet: {needle}"
    );
}

fn assert_not_contains(source: &str, needle: &str) {
    assert!(
        !source.contains(needle),
        "source guard found forbidden snippet: {needle}"
    );
}

#[test]
fn issue_203_authority_store_boundary_guardrails_are_bound() {
    let adapters = include_str!("../src/distributed/authority_store_adapters.rs");
    let certificates = include_str!("../src/distributed/certificates.rs");
    let lease = include_str!("../src/distributed/lease.rs");
    let fencing = include_str!("../src/distributed/fencing.rs");

    assert_contains(adapters, "pub struct AuthorityStoreAdapterRegistry");
    assert_contains(adapters, "pub struct PublishedStoreAuthorityReceiptView");
    assert_contains(adapters, "pub struct AuthorityBoundCertificateStore");
    assert_contains(adapters, "pub struct AuthorityBoundLeaseLedger");
    assert_contains(adapters, "pub struct AuthorityBoundFencingStore");
    assert_contains(adapters, "action_class: String");
    assert_contains(adapters, "adapter_kind: String");
    assert_contains(adapters, "adapter_version: u32");
    assert_contains(adapters, "receipt_sha256: [u8; 32]");
    assert_contains(
        adapters,
        "published_view_action_class(result.mutation_kind())",
    );
    assert_contains(adapters, "PUBLISHED_VIEW_ACTION_OWNER_COMMIT");
    assert_contains(adapters, "receipt_sha256: result.receipts_sha256()");
    assert_contains(
        adapters,
        "validate_permit(&permit, &AuthorityPermitAction::Read)",
    );
    assert_contains(
        adapters,
        "AuthorityPermitAction::Mutation(mutation_kind.to_owned())",
    );
    assert_contains(
        adapters,
        "authority_bound_certificate_store_for_test_fixture",
    );

    assert_contains(certificates, "pub struct CertificateStoreAccess");
    assert_contains(
        certificates,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_CERTIFICATE_ACCESS",
    );
    assert_contains(
        certificates,
        "pub fn open(\n        _access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn activate(\n        &self,\n        _access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn authorize(\n        &self,\n        _access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn revoke(\n        &self,\n        _access: &CertificateStoreAccess,",
    );

    assert_contains(lease, "pub struct LeaseState");
    assert_contains(lease, "pub struct LeaseStoreAccess");
    assert_contains(
        lease,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_LEASE_ACCESS",
    );
    assert_contains(
        lease,
        "pub fn new(_access: &LeaseStoreAccess, policy: LeasePolicy)",
    );
    assert_contains(
        lease,
        "pub fn apply(\n        &mut self,\n        _access: &LeaseStoreAccess,",
    );
    assert_contains(
        lease,
        "pub fn authorize_mutation(\n        &mut self,\n        _access: &LeaseStoreAccess,",
    );
    assert_not_contains(lease, "pub activated_elapsed_millis");
    assert_not_contains(lease, "pub deadline_elapsed_millis");
    assert_contains(lease, "pub deadline_unix_millis");
    assert_contains(lease, "pub now_unix_seconds: i64");
    assert_contains(lease, "pub now_unix_nanos: u32");
    assert_contains(lease, "now_unix_millis >= lease.deadline_unix_millis");
    assert_contains(lease, "restart_safety_deadline_unix_millis");

    assert_contains(fencing, "pub safety_deadline_unix_millis: u64");
    assert_contains(fencing, "pub struct FencingStoreAccess");
    assert_contains(
        fencing,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_FENCING_ACCESS",
    );
    assert_contains(
        fencing,
        "pub fn create(\n        _access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn open(\n        _access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn commit(\n        &mut self,\n        _access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn authorize_active_lease(\n        &self,\n        _access: &FencingStoreAccess,",
    );
    assert_not_contains(
        fencing,
        "pub fn authorize_active_lease(&self, check: ActiveLeaseCheck",
    );
    assert_contains(
        adapters,
        ".authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)",
    );
    assert_contains(
        fencing,
        "if check.now_unix_millis < floor.safety_deadline_unix_millis",
    );
    assert_contains(
        fencing,
        "if check.now_unix_millis >= check.lease.deadline_unix_millis",
    );
}

#[test]
fn authority_store_adapter_denies_unpublished_lineage() {
    let root = repo_local_root();
    let barrier = AuthorityReconciliationBarrier::open(
        root.path(),
        identity(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let registry = AuthorityStoreAdapterRegistry::new(Arc::new(barrier));

    assert!(matches!(
        registry.published_view("lineage-a"),
        Err(AuthorityStoreAdapterError::Reconciliation(
            adl_runtime::distributed::authority_reconciliation::AuthorityReconciliationError::ReconciliationRequired
        ))
    ));

    println!("ADL_ISSUE_203_ADAPTER_GUARD_V1 unpublished_lineage_denied");
}

#[test]
fn authority_store_adapter_refuses_certificate_handle_without_publication() {
    let root = repo_local_root();
    let barrier = AuthorityReconciliationBarrier::open(
        root.path(),
        identity(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let registry = AuthorityStoreAdapterRegistry::new(Arc::new(barrier));
    let signing_root = SigningKey::from_bytes(&[41; 32]);
    let policy = CertificatePolicy::new("runtime-prod", [signing_root.verifying_key()]).unwrap();
    let store = Arc::new(
        DistributedCertificateStore::open(
            &TEST_CERTIFICATE_STORE_ACCESS,
            root.path().join("certificates.redb"),
            policy,
        )
        .unwrap(),
    );

    assert!(matches!(
        registry.certificate_store("lineage-a", store),
        Err(AuthorityStoreAdapterError::Reconciliation(
            adl_runtime::distributed::authority_reconciliation::AuthorityReconciliationError::ReconciliationRequired
        ))
    ));

    println!("ADL_ISSUE_203_ADAPTER_GUARD_V1 unpublished_certificate_handle_denied");
}
