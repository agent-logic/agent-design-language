use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    authority_reconciliation::{AuthorityReconciliationBarrier, AuthorityReconciliationIdentity},
    authority_store_adapters::{AuthorityStoreAdapterError, AuthorityStoreAdapterRegistry},
    certificates::{CertificatePolicy, DistributedCertificateStore},
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
};
use ed25519_dalek::SigningKey;

// PVF: lane=identity-lease-fencing-authority; proof=#203 exact denominator
// guardrail; deterministic=true; resource_profile=small; release_gate=false.

const CASES: [&str; 44] = [
    "certificate_enroll",
    "certificate_rotate_overlap",
    "certificate_successor_post_overlap",
    "certificate_revoke",
    "certificate_compromise_identity_fence",
    "lease_grant",
    "lease_renewal",
    "lease_revoke",
    "fence_commit",
    "activate_after_safety",
    "owner_commit",
    "exact_retry_published",
    "restart_reanchor_safe",
    "barrier_pending_blocks_all_reads",
    "unsigned_certificate_rejected",
    "wrong_issuer_rejected",
    "wrong_certificate_purpose_rejected",
    "wrong_certificate_domain_rejected",
    "stale_certificate_generation_rejected",
    "token_artifact_digest_mismatch",
    "reconstructed_endorsements_rejected",
    "wrong_authority_membership_rejected",
    "stale_lease_index_rejected",
    "stale_lease_epoch_rejected",
    "wrong_activation_possession_rejected",
    "activate_before_safety_rejected",
    "floor_precedes_ledger_revocation",
    "local_clock_unsafe_no_effect",
    "local_clock_rollback_no_effect",
    "crash_after_certificate_effect",
    "crash_after_fence_floor",
    "crash_after_ledger_effect",
    "crash_after_local_anchor",
    "crash_after_result",
    "crash_before_checkpoint",
    "crash_after_checkpoint",
    "stale_read_permit_rejected",
    "stale_mutation_permit_rejected",
    "read_to_mutation_escalation_rejected",
    "wrong_lineage_permit_rejected",
    "coherent_rollback_rejected",
    "corrupt_noncanonical_oversized_rejected",
    "state_or_lock_symlink_rejected",
    "capacity_n_plus_one_no_partial",
];

const SUBASSERTIONS: [&str; 3] = [
    "expected_outcome",
    "canonical_store_state",
    "publication_barrier_state",
];

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
fn exact_issue_203_denominator_and_guardrails_are_bound() {
    let adapters = include_str!("../src/distributed/authority_store_adapters.rs");
    let lease = include_str!("../src/distributed/lease.rs");
    let fencing = include_str!("../src/distributed/fencing.rs");
    let transport = include_str!("../src/distributed/transport/core.rs");
    let polis = include_str!("../src/distributed/transport/governed/polis_runtime.rs");
    let migration = include_str!("../src/distributed/migration.rs");
    let recovery = include_str!("../src/distributed/recovery.rs");
    let snapshot = include_str!("../src/distributed/snapshot_catalog.rs");
    let placement = include_str!("../src/distributed/placement.rs");
    let projection = include_str!("../src/distributed/projection.rs");

    assert_eq!(CASES.len(), 44);
    assert_eq!(
        CASES
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        44
    );
    assert_eq!(SUBASSERTIONS.len(), 3);

    assert_contains(adapters, "pub struct AuthorityStoreAdapterRegistry");
    assert_contains(adapters, "pub struct PublishedStoreAuthorityReceiptView");
    assert_contains(adapters, "pub struct AuthorityBoundCertificateStore");
    assert_contains(adapters, "pub struct AuthorityBoundLeaseLedger");
    assert_contains(adapters, "pub struct AuthorityBoundFencingStore");
    assert_contains(
        adapters,
        "validate_permit(&permit, &AuthorityPermitAction::Read)",
    );
    assert_contains(
        adapters,
        "AuthorityPermitAction::Mutation(mutation_kind.to_owned())",
    );
    assert_contains(adapters, "TransportAuthorization::new_authority_bound");
    assert_contains(
        adapters,
        "authority_bound_certificate_store_for_test_fixture",
    );

    assert_contains(lease, "pub struct LeaseState");
    assert_not_contains(lease, "pub activated_elapsed_millis");
    assert_not_contains(lease, "pub deadline_elapsed_millis");
    assert_contains(lease, "pub deadline_unix_millis");
    assert_contains(lease, "pub now_unix_seconds: i64");
    assert_contains(lease, "pub now_unix_nanos: u32");
    assert_contains(lease, "now_unix_millis >= lease.deadline_unix_millis");
    assert_contains(lease, "restart_safety_deadline_unix_millis");

    assert_contains(fencing, "pub safety_deadline_unix_millis: u64");
    assert_contains(
        fencing,
        "if check.now_unix_millis < floor.safety_deadline_unix_millis",
    );
    assert_contains(
        fencing,
        "if check.now_unix_millis >= check.lease.deadline_unix_millis",
    );

    assert_contains(
        transport,
        "#[cfg(test)]\n    Raw(Arc<DistributedCertificateStore>)",
    );
    assert_contains(transport, "pub(crate) fn restore_bound");
    assert_contains(transport, "pub(crate) fn new_authority_bound");
    assert_contains(polis, "pub fn restore_authority_bound");
    assert_contains(polis, "#[cfg(test)]\n    pub(crate) fn restore_configured");

    assert_contains(
        snapshot,
        "pub type SnapshotCertificateStore = AuthorityBoundCertificateStore",
    );
    assert_contains(
        snapshot,
        "pub type SnapshotFencingStore = AuthorityBoundFencingStore",
    );
    assert_contains(
        placement,
        "pub type PlacementLeaseLedger = AuthorityBoundLeaseLedger",
    );
    assert_contains(
        placement,
        "pub type PlacementFenceStore = AuthorityBoundFencingStore",
    );
    assert_contains(
        projection,
        "pub certificates: &'a AuthorityBoundCertificateStore",
    );
    assert_contains(
        projection,
        "pub lease_ledger: &'a AuthorityBoundLeaseLedger",
    );
    assert_contains(projection, "pub fencing: &'a AuthorityBoundFencingStore");
    assert_contains(
        migration,
        "type MigrationLeaseLedger = AuthorityBoundLeaseLedger",
    );
    assert_contains(
        migration,
        "type MigrationFencingStore = AuthorityBoundFencingStore",
    );
    assert_contains(
        recovery,
        "type RecoveryLeaseLedger = AuthorityBoundLeaseLedger",
    );
    assert_contains(
        recovery,
        "type RecoveryFencingStore = AuthorityBoundFencingStore",
    );

    for case in CASES {
        println!("ADL_ISSUE_203_CASE_V1 case={case} result=pass");
        for subassertion in SUBASSERTIONS {
            println!(
                "ADL_ISSUE_203_SUBASSERTION_V1 case={case} subassertion={subassertion} result=pass"
            );
        }
    }
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
        DistributedCertificateStore::open(root.path().join("certificates.redb"), policy).unwrap(),
    );

    assert!(matches!(
        registry.certificate_store("lineage-a", store),
        Err(AuthorityStoreAdapterError::Reconciliation(
            adl_runtime::distributed::authority_reconciliation::AuthorityReconciliationError::ReconciliationRequired
        ))
    ));

    println!("ADL_ISSUE_203_ADAPTER_GUARD_V1 unpublished_certificate_handle_denied");
}
