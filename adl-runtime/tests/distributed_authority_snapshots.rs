// PVF: lane=exact-child-tests; proof=revisioned redacted authority snapshot contracts;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/capability_advertisement.rs"]
mod capability_advertisement;
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
#[allow(dead_code)]
#[path = "../src/distributed/failure_detection.rs"]
mod failure_detection;
#[allow(dead_code)]
#[path = "../src/distributed/fencing.rs"]
mod fencing;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;
#[allow(dead_code)]
#[path = "../src/distributed/membership.rs"]
mod membership;
#[allow(dead_code)]
#[path = "../src/distributed/migration.rs"]
mod migration;
mod integrated_serving_authority_snapshot {
    pub use adl_runtime::distributed::integrated_serving_authority_snapshot::*;
}
mod shepherd_serving_eligibility {
    pub use adl_runtime::distributed::shepherd_serving_eligibility::*;
}
#[cfg(feature = "internal-test-fixtures")]
mod distributed {
    pub use adl_runtime::distributed::{
        authority_protocol, observatory_serving_eligibility, polis_runtime, serving_authority,
        shepherd_serving_eligibility,
    };
}
#[allow(dead_code)]
#[path = "../src/distributed/placement.rs"]
mod placement;
#[allow(dead_code)]
#[path = "../src/distributed/recovery.rs"]
mod recovery;
#[allow(dead_code)]
#[path = "../src/distributed/resource_weather.rs"]
mod resource_weather;
#[allow(dead_code)]
#[path = "../src/distributed/snapshot_catalog.rs"]
mod snapshot_catalog;

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

use certificates::{
    ActivationOutcome, AuthorityCertificate, CertificateBody, CertificateError, CertificatePolicy,
    CertificatePurpose, CertificateValidity, DistributedCertificateStore,
    RedactedCertificateHealth, TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use failure_detection::{
    FailureDetector, FailureError, FailureMembershipSnapshot, FailurePolicy, FailureProbeClaims,
    FailureSnapshotReason, FailureThresholds, ProbeAuthority, ProbeResult, SignedFailureProbe,
    FAILURE_PROBE_SCHEMA,
};
use fencing::{
    FenceReceipt, FencingCheckpoint, FencingCheckpointAuthority, FencingError, FencingPolicy,
    FencingStore, TEST_FENCING_STORE_ACCESS,
};
use lease::{
    AuthorityLedger, AuthorityMembership, ControlCertificatePurpose, LeasePolicy, LeaseState,
    OperationClass, VoterAuthority, TEST_LEASE_STORE_ACCESS,
};
use migration::{
    MigrationCheckpoint, MigrationCheckpointAuthority, MigrationClock, MigrationPhase,
    MigrationPolicy, MigrationRecord, MigrationStore, TransitionEvidence,
};
use placement::{
    PlacementCapacityBand, PlacementClock, PlacementDecision, PlacementPolicy, PlacementService,
};
use recovery::{
    RecoveryCheckpoint, RecoveryCheckpointAuthority, RecoveryClock, RecoveryEvidence,
    RecoveryPhase, RecoveryPolicy, RecoveryRecord, RecoveryStore,
};
use sha2::{Digest, Sha256};

const CERT_DOMAIN: &str = "polis.example";
const FAILURE_DOMAIN: &str = "polis.test";
const NOW: u64 = 1_000_000;

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn expected_ref(kind: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"adl-projection-ref-v1");
    digest.update((kind.len() as u64).to_be_bytes());
    digest.update(kind);
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
    format!("id_{}", hex::encode(digest.finalize()))
}

fn certificate(
    root: &SigningKey,
    holder: &str,
    purpose: CertificatePurpose,
    generation: u64,
    subject: &SigningKey,
) -> AuthorityCertificate {
    AuthorityCertificate::issue(
        CertificateBody::new(
            CERT_DOMAIN,
            holder,
            purpose,
            generation,
            CertificateValidity {
                issued_at_unix_secs: NOW,
                expires_at_unix_secs: NOW + 300,
            },
            subject.verifying_key(),
            &root.verifying_key(),
        ),
        root,
    )
    .unwrap()
}

#[test]
fn certificate_snapshot_is_complete_redacted_revisioned_and_overlap_aware() {
    let directory = tempfile::tempdir().unwrap();
    let root = key(1);
    let policy = CertificatePolicy::new(CERT_DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3_600, 30, 10, 64, 64)
        .unwrap();
    let store = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        directory
            .path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        policy,
    )
    .unwrap();
    let first = certificate(
        &root,
        "node-a",
        CertificatePurpose::NodeIdentity,
        1,
        &key(2),
    );
    let second = certificate(
        &root,
        "node-a",
        CertificatePurpose::NodeIdentity,
        2,
        &key(3),
    );
    assert!(matches!(
        store
            .activate(&TEST_CERTIFICATE_STORE_ACCESS, &first, NOW)
            .unwrap(),
        ActivationOutcome::Activated(_)
    ));
    let revision_one = store.authority_revision().unwrap();
    let one = store.redacted_snapshot_at(revision_one, NOW + 1).unwrap();
    assert_eq!(one.trust_domain(), CERT_DOMAIN);
    assert_eq!(one.rows().len(), 1);
    let first_row = one.rows().next().unwrap();
    assert!(first_row.node_ref().unwrap().starts_with("id_"));
    assert_ne!(first_row.node_ref(), Some("node-a"));
    assert_eq!(
        first_row.node_ref(),
        Some(expected_ref(b"node", b"node-a").as_str())
    );
    assert!(!format!("{first_row:?}").contains("node-a"));

    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &second, NOW + 5)
        .unwrap();
    assert_eq!(
        store
            .redacted_snapshot_at(revision_one, NOW + 6)
            .unwrap_err(),
        CertificateError::RevisionDrift
    );
    let rotated = store
        .redacted_snapshot_at(store.authority_revision().unwrap(), NOW + 6)
        .unwrap();
    assert_eq!(rotated.rows().len(), 2);
    assert!(rotated
        .rows()
        .any(|row| row.health() == RedactedCertificateHealth::RotationOverlap));
}

#[derive(Default)]
struct FailureAuthority {
    keys: BTreeMap<(String, u64), VerifyingKey>,
    members: BTreeSet<(String, u64)>,
    complete: Vec<(String, String)>,
}

impl ProbeAuthority for FailureAuthority {
    fn current_observer_identity(&self, node: &str) -> Option<(u64, VerifyingKey)> {
        self.keys
            .get(&(node.to_owned(), 1))
            .copied()
            .map(|key| (1, key))
    }

    fn is_member(&self, node: &str, epoch: u64) -> bool {
        self.members.contains(&(node.to_owned(), epoch))
    }
}

fn failure_policy() -> FailurePolicy {
    FailurePolicy::new(
        FAILURE_DOMAIN,
        "node-local",
        7,
        FailureThresholds {
            suspect_after_secs: 5,
            unavailable_after_secs: 10,
            evidence_window_secs: 5,
            flap_window_secs: 30,
        },
        2,
    )
    .unwrap()
    .with_bounds(2, 8, 8, 4, 8)
    .unwrap()
}

#[test]
fn failure_snapshot_enumerates_missing_members_and_rejects_revision_drift() {
    let observer = key(8);
    let mut authority = FailureAuthority {
        complete: vec![
            ("node-a".into(), "guardian-a".into()),
            ("node-b".into(), "guardian-b".into()),
        ],
        ..Default::default()
    };
    authority
        .keys
        .insert(("node-a".into(), 1), observer.verifying_key());
    authority.members.insert(("node-a".into(), 7));
    authority.members.insert(("node-b".into(), 7));
    let membership = FailureMembershipSnapshot::from_test_rows(
        FAILURE_DOMAIN,
        7,
        11,
        authority.complete.clone(),
    );
    let foreign_membership = FailureMembershipSnapshot::from_test_rows(
        "foreign.test",
        7,
        11,
        authority.complete.clone(),
    );
    let mut detector = FailureDetector::new(failure_policy());
    assert_eq!(
        detector
            .authority_revision(&foreign_membership, 100)
            .unwrap_err(),
        FailureError::WrongTrustDomain
    );
    let empty_revision = detector.authority_revision(&membership, 100).unwrap();
    let empty = detector
        .redacted_snapshot_at(empty_revision, &membership, 100)
        .unwrap();
    assert_eq!(empty.rows().len(), 2);
    assert!(empty
        .rows()
        .all(|row| row.reason() == FailureSnapshotReason::NoEvidence));

    let probe = SignedFailureProbe::sign(
        FailureProbeClaims {
            schema: FAILURE_PROBE_SCHEMA.into(),
            trust_domain: FAILURE_DOMAIN.into(),
            membership_epoch: 7,
            observer_node_id: "node-a".into(),
            observer_identity_generation: 1,
            subject_node_id: "node-b".into(),
            sequence: 1,
            observed_at_unix_secs: 100,
            expires_at_unix_secs: 120,
            result: ProbeResult::Reachable,
        },
        &observer,
    )
    .unwrap();
    detector.observe(&authority, &probe, 100).unwrap();
    assert_eq!(
        detector
            .redacted_snapshot_at(empty_revision, &membership, 100)
            .unwrap_err(),
        FailureError::RevisionDrift
    );
    let changed = detector
        .redacted_snapshot_at(
            detector.authority_revision(&membership, 100).unwrap(),
            &membership,
            100,
        )
        .unwrap();
    assert_eq!(changed.rows().len(), 2);
    assert_eq!(
        changed.rows().filter(|row| row.class().is_some()).count(),
        1
    );
}

#[derive(Clone, Copy, Debug)]
struct FixedPlacementClock;

impl PlacementClock for FixedPlacementClock {
    fn now_unix_secs(&self) -> placement::PlacementResult<u64> {
        Ok(100)
    }
}

fn placement_decision(node: &str, pressure_permille: u16) -> PlacementDecision {
    PlacementDecision {
        lineage_id: "lineage-a".into(),
        node_id: node.into(),
        guardian_id: format!("guardian-{node}"),
        membership_epoch: 7,
        committed_log_index: 11,
        capability_sequence: 3,
        weather_sequence: 4,
        pressure_permille,
        remaining_slots: 2,
    }
}

#[test]
fn placement_snapshot_retains_replaces_removes_and_buckets_capacity() {
    let restarted = PlacementService::new(
        PlacementPolicy::new(CERT_DOMAIN).unwrap(),
        FixedPlacementClock,
    );
    assert_eq!(
        restarted.authority_revision().unwrap_err(),
        placement::PlacementError::AuthorityUnavailable
    );
    let service = PlacementService::new(
        PlacementPolicy::new(CERT_DOMAIN).unwrap(),
        FixedPlacementClock,
    );
    service
        .seed_decision_for_snapshot_test(placement_decision("node-a", 100), 100)
        .unwrap();
    let first_revision = service.authority_revision().unwrap();
    let first = service.redacted_snapshot_at(first_revision).unwrap();
    let first_row = first.rows().next().unwrap();
    assert_eq!(
        first_row.lineage_ref(),
        expected_ref(b"lineage", b"lineage-a")
    );
    assert_eq!(first_row.capacity(), PlacementCapacityBand::Available);
    assert!(!format!("{first_row:?}").contains("node-a"));

    service
        .seed_decision_for_snapshot_test(placement_decision("node-b", 900), 101)
        .unwrap();
    assert_eq!(
        service.redacted_snapshot_at(first_revision).unwrap_err(),
        placement::PlacementError::RevisionDrift
    );
    let replacement = service
        .redacted_snapshot_at(service.authority_revision().unwrap())
        .unwrap();
    assert_eq!(replacement.rows().len(), 1);
    assert_eq!(
        replacement.rows().next().unwrap().capacity(),
        PlacementCapacityBand::Constrained
    );
    assert!(service.remove_decision("lineage-a").unwrap());
    assert_eq!(
        service
            .redacted_snapshot_at(service.authority_revision().unwrap())
            .unwrap()
            .rows()
            .len(),
        0
    );
}

#[derive(Clone, Debug, Default)]
struct MigrationAuthority(Arc<Mutex<Option<MigrationCheckpoint>>>);

impl MigrationCheckpointAuthority for MigrationAuthority {
    fn current(&self) -> migration::MigrationResult<Option<MigrationCheckpoint>> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<MigrationCheckpoint>,
        next: MigrationCheckpoint,
    ) -> migration::MigrationResult<()> {
        let mut current = self.0.lock().unwrap();
        if *current != expected {
            return Err(migration::MigrationError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct FixedMigrationClock;

impl MigrationClock for FixedMigrationClock {
    fn now_millis(&self) -> migration::MigrationResult<u64> {
        Ok(100)
    }
}

fn migration_record() -> MigrationRecord {
    MigrationRecord {
        migration_id: b"migration-a".to_vec(),
        request_sha256: [1; 32],
        trust_domain: CERT_DOMAIN.into(),
        lineage_id: b"lineage-a".to_vec(),
        source_node_id: b"node-a".to_vec(),
        source_guardian_id: b"guardian-node-a".to_vec(),
        source_epoch: 1,
        source_log_index: 10,
        source_certificate_sha256: [2; 32],
        target_node_id: b"node-b".to_vec(),
        target_guardian_id: b"guardian-node-b".to_vec(),
        placement_membership_epoch: 7,
        placement_log_index: 11,
        placement_capability_sequence: 3,
        placement_weather_sequence: 4,
        timeout_millis: 1_000,
        started_at_millis: 100,
        deadline_millis: 1_100,
        phase: MigrationPhase::Prepared,
        source_authoritative: true,
        target_authoritative: false,
        quiescence_sha256: None,
        catalog_entry_sha256: None,
        snapshot_content_sha256: None,
        snapshot_schema: None,
        snapshot_byte_length: None,
        snapshot_chunk_count: None,
        snapshot_expiry_unix_secs: None,
        transfer_id: None,
        transfer_manifest_sha256: None,
        restore_receipt_sha256: None,
        fence_request_id: None,
        fence_epoch: None,
        fence_log_index: None,
        fence_certificate_sha256: None,
        activation_log_index: None,
        activation_certificate_sha256: None,
        commit_log_index: None,
        commit_certificate_sha256: None,
        serving_operation_ref: None,
        serving_input_sha256: None,
        serving_result_state_sha256: None,
        serving_receipt_sha256: None,
        history: vec![TransitionEvidence {
            phase: MigrationPhase::Prepared,
            evidence_sha256: [3; 32],
        }],
    }
}

#[test]
fn migration_snapshot_enumeration_and_revision_survive_restart() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let authority = MigrationAuthority::default();
    let policy = MigrationPolicy::new(CERT_DOMAIN).unwrap();
    let mut store = MigrationStore::create(
        &root,
        policy.clone(),
        Arc::new(authority.clone()),
        Arc::new(FixedMigrationClock),
    )
    .unwrap();
    let empty = store.authority_revision().unwrap();
    store
        .seed_record_for_snapshot_test(migration_record())
        .unwrap();
    assert_eq!(
        store.redacted_snapshot_at(empty).unwrap_err(),
        migration::MigrationError::RevisionDrift
    );
    let before = store.authority_revision().unwrap();
    let snapshot = store.redacted_snapshot_at(before).unwrap();
    assert_eq!(snapshot.rows().len(), 1);
    let row = snapshot.rows().next().unwrap();
    assert_eq!(row.lineage_ref(), expected_ref(b"lineage", b"lineage-a"));
    assert_eq!(
        row.migration_ref(),
        expected_ref(b"migration", b"migration-a")
    );
    assert!(!format!("{row:?}").contains("lineage-a"));
    drop(store);
    let reopened = MigrationStore::open(
        &root,
        policy,
        Arc::new(authority),
        Arc::new(FixedMigrationClock),
    )
    .unwrap();
    assert_eq!(reopened.authority_revision().unwrap(), before);
    assert_eq!(reopened.redacted_snapshot_at(before).unwrap(), snapshot);
}

#[derive(Clone, Debug, Default)]
struct RecoveryAuthority(Arc<Mutex<Option<RecoveryCheckpoint>>>);

impl RecoveryCheckpointAuthority for RecoveryAuthority {
    fn current(&self) -> recovery::RecoveryResult<Option<RecoveryCheckpoint>> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<RecoveryCheckpoint>,
        next: RecoveryCheckpoint,
    ) -> recovery::RecoveryResult<()> {
        let mut current = self.0.lock().unwrap();
        if *current != expected {
            return Err(recovery::RecoveryError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct FixedRecoveryClock;

impl RecoveryClock for FixedRecoveryClock {
    fn now(&self) -> recovery::RecoveryResult<recovery::RecoveryTime> {
        Ok(recovery::RecoveryTime {
            unix_seconds: 1,
            unix_nanos: 0,
            elapsed_millis: 100,
            clock_uncertainty_millis: 0,
        })
    }
}

fn recovery_record() -> RecoveryRecord {
    RecoveryRecord {
        recovery_id: b"recovery-a".to_vec(),
        migration_id: b"migration-a".to_vec(),
        migration_record_sha256: [4; 32],
        trust_domain: CERT_DOMAIN.into(),
        lineage_id: b"lineage-a".to_vec(),
        source_node_id: b"node-a".to_vec(),
        source_guardian_id: b"guardian-node-a".to_vec(),
        target_node_id: b"node-b".to_vec(),
        target_guardian_id: b"guardian-node-b".to_vec(),
        target_transfer_id: None,
        target_content_sha256: None,
        target_cleanup_required: false,
        target_cleanup_receipt_sha256: None,
        observed_migration_phase: MigrationPhase::Prepared,
        local_histories_sha256: [5; 32],
        committed_prefix_sha256: None,
        committed_prefix_epoch: None,
        committed_prefix_log_index: None,
        committed_prefix_voter_generation: None,
        committed_prefix_certificate_sha256: None,
        fence_epoch: None,
        fence_log_index: None,
        fence_certificate_sha256: None,
        started_at_millis: 100,
        deadline_millis: 1_100,
        phase: RecoveryPhase::Assessing,
        owner_node_id: None,
        owner_guardian_id: None,
        owner_epoch: None,
        committed_log_index: None,
        authority_certificate_sha256: None,
        serving_operation_ref: None,
        serving_input_sha256: None,
        serving_result_state_sha256: None,
        serving_receipt_sha256: None,
        history: vec![RecoveryEvidence {
            phase: RecoveryPhase::Assessing,
            evidence_sha256: [6; 32],
        }],
    }
}

#[test]
fn recovery_snapshot_enumeration_and_revision_survive_restart() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let authority = RecoveryAuthority::default();
    let policy = RecoveryPolicy::new(CERT_DOMAIN).unwrap();
    let mut store = RecoveryStore::create(
        &root,
        policy.clone(),
        Arc::new(authority.clone()),
        Arc::new(FixedRecoveryClock),
    )
    .unwrap();
    let empty = store.authority_revision().unwrap();
    store
        .seed_record_for_snapshot_test(recovery_record())
        .unwrap();
    assert_eq!(
        store.redacted_snapshot_at(empty).unwrap_err(),
        recovery::RecoveryError::RevisionDrift
    );
    let before = store.authority_revision().unwrap();
    let snapshot = store.redacted_snapshot_at(before).unwrap();
    assert_eq!(snapshot.rows().len(), 1);
    let row = snapshot.rows().next().unwrap();
    assert!(!row.operator_required());
    assert_eq!(row.lineage_ref(), expected_ref(b"lineage", b"lineage-a"));
    assert_eq!(
        row.migration_ref(),
        expected_ref(b"migration", b"migration-a")
    );
    drop(store);
    let reopened = RecoveryStore::open(
        &root,
        policy,
        Arc::new(authority),
        Arc::new(FixedRecoveryClock),
    )
    .unwrap();
    assert_eq!(reopened.authority_revision().unwrap(), before);
    assert_eq!(reopened.redacted_snapshot_at(before).unwrap(), snapshot);
}

fn authority_membership(committed_log_index: u64) -> AuthorityMembership {
    let ids = [
        b"guardian-0".to_vec(),
        b"guardian-1".to_vec(),
        b"guardian-2".to_vec(),
    ];
    let voters = ids
        .iter()
        .enumerate()
        .map(|(index, id)| VoterAuthority {
            guardian_id: id.clone(),
            trust_domain_id: CERT_DOMAIN.as_bytes().to_vec(),
            certificate_generation: 1,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: 1,
            not_after_unix_seconds: 10_000,
            revoked: false,
            control_public_key: key(index as u8 + 20).verifying_key().to_bytes(),
        })
        .collect();
    AuthorityMembership::new(
        CERT_DOMAIN.as_bytes().to_vec(),
        1,
        committed_log_index,
        vec![ids.into_iter().collect()],
        voters,
    )
    .unwrap()
}

fn lease_policy() -> LeasePolicy {
    LeasePolicy {
        max_lease_duration_millis: 1_000,
        max_clock_uncertainty_millis: 10,
        message_delay_margin_millis: 5,
        max_lineages: 64,
        max_snapshot_bytes: 1024 * 1024,
    }
}

fn lease_state() -> LeaseState {
    LeaseState {
        lineage_id: b"lineage-a".to_vec(),
        holder_node_id: b"node-a".to_vec(),
        holder_guardian_id: b"guardian-node-a".to_vec(),
        activation_public_key: [7; 32],
        raft_term: 1,
        committed_log_index: 11,
        epoch: 2,
        certificate_generation: 1,
        activated_elapsed_millis: 100,
        deadline_elapsed_millis: 2_100,
        deadline_unix_millis: 2_000,
        certificate_bytes: vec![1],
        revoked: false,
        last_mutation_sequence: 0,
    }
}

#[test]
fn lease_snapshot_is_complete_content_bound_and_drift_safe() {
    let membership = authority_membership(11);
    let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
    let empty = ledger.authority_revision().unwrap();
    ledger.seed_lease_for_snapshot_test(lease_state()).unwrap();
    assert_eq!(
        ledger
            .redacted_snapshot_at(empty, &membership, 100)
            .unwrap_err(),
        lease::AuthorityError::RevisionDrift
    );
    let revision = ledger.authority_revision().unwrap();
    let snapshot = ledger
        .redacted_snapshot_at(revision, &membership, 100)
        .unwrap();
    assert_eq!(snapshot.rows().len(), 1);
    assert_eq!(
        snapshot.rows().next().unwrap().lineage_ref(),
        expected_ref(b"lineage", b"lineage-a")
    );
    assert_eq!(snapshot.revision(), revision);
}

#[derive(Clone, Debug, Default)]
struct FenceAuthority(Arc<Mutex<Option<FencingCheckpoint>>>);

impl FencingCheckpointAuthority for FenceAuthority {
    fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<FencingCheckpoint>,
        next: FencingCheckpoint,
    ) -> Result<(), FencingError> {
        let mut current = self.0.lock().unwrap();
        if *current != expected {
            return Err(FencingError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

fn fencing_policy() -> FencingPolicy {
    FencingPolicy {
        max_lineages: 64,
        max_receipts: 64,
        max_state_bytes: 1024 * 1024,
        max_clock_uncertainty_millis: 10,
        message_delay_margin_millis: 5,
    }
}

#[test]
fn fencing_snapshot_is_complete_content_bound_restart_stable_and_ref_aligned() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let authority = FenceAuthority::default();
    let membership = authority_membership(11);
    let mut store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        &root,
        fencing_policy(),
        Arc::new(authority.clone()),
    )
    .unwrap();
    let empty = store.authority_revision().unwrap();
    store
        .seed_floor_for_snapshot_test(FenceReceipt {
            request_id: b"fence-a".to_vec(),
            request_sha256: [8; 32],
            trust_domain_id: CERT_DOMAIN.as_bytes().to_vec(),
            lineage_id: b"lineage-a".to_vec(),
            epoch: 2,
            committed_log_index: 11,
            voter_set_generation: 1,
            operation_class: OperationClass::Fence as u32,
            certificate_sha256: [9; 32],
            safety_deadline_unix_millis: 2_000,
        })
        .unwrap();
    assert_eq!(
        store.redacted_snapshot_at(empty, &membership).unwrap_err(),
        FencingError::RevisionDrift
    );
    let revision = store.authority_revision().unwrap();
    let snapshot = store.redacted_snapshot_at(revision, &membership).unwrap();
    assert_eq!(snapshot.rows().len(), 1);
    assert_eq!(
        snapshot.rows().next().unwrap().lineage_ref(),
        expected_ref(b"lineage", b"lineage-a")
    );
    drop(store);
    let reopened = FencingStore::open(
        &TEST_FENCING_STORE_ACCESS,
        &root,
        fencing_policy(),
        Arc::new(authority),
    )
    .unwrap();
    assert_eq!(reopened.authority_revision().unwrap(), revision);
    assert_eq!(
        reopened
            .redacted_snapshot_at(revision, &membership)
            .unwrap(),
        snapshot
    );

    let ledger = {
        let mut value = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
        value.seed_lease_for_snapshot_test(lease_state()).unwrap();
        value
    };
    let lease_snapshot = ledger
        .redacted_snapshot_at(ledger.authority_revision().unwrap(), &membership, 100)
        .unwrap();
    assert_eq!(
        snapshot.rows().next().unwrap().lineage_ref(),
        lease_snapshot.rows().next().unwrap().lineage_ref()
    );
}

#[test]
fn snapshot_bounds_reject_n_plus_one_without_partial_mutation() {
    let certificate_directory = tempfile::tempdir().unwrap();
    let certificate_root = key(40);
    let bounded_certificate_policy =
        CertificatePolicy::new(CERT_DOMAIN, [certificate_root.verifying_key()])
            .unwrap()
            .with_bounds(3_600, 30, 10, 1, 1)
            .unwrap();
    let certificate_store = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        certificate_directory
            .path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        bounded_certificate_policy,
    )
    .unwrap();
    certificate_store
        .activate(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate(
                &certificate_root,
                "node-a",
                CertificatePurpose::Transport,
                1,
                &key(41),
            ),
            NOW,
        )
        .unwrap();
    let certificate_revision = certificate_store.authority_revision().unwrap();
    assert_eq!(
        certificate_store
            .activate(
                &TEST_CERTIFICATE_STORE_ACCESS,
                &certificate(
                    &certificate_root,
                    "node-b",
                    CertificatePurpose::Transport,
                    1,
                    &key(42),
                ),
                NOW,
            )
            .unwrap_err(),
        CertificateError::ResourceExhausted
    );
    assert_eq!(
        certificate_store.authority_revision().unwrap(),
        certificate_revision
    );

    let oversized_membership = FailureMembershipSnapshot::from_test_rows(
        FAILURE_DOMAIN,
        7,
        11,
        vec![
            ("node-a".into(), "guardian-a".into()),
            ("node-b".into(), "guardian-b".into()),
            ("node-c".into(), "guardian-c".into()),
        ],
    );
    let bounded_failure =
        FailureDetector::new(failure_policy().with_bounds(2, 8, 2, 4, 8).unwrap());
    assert_eq!(
        bounded_failure
            .authority_revision(&oversized_membership, 100)
            .unwrap_err(),
        FailureError::ResourceExhausted
    );

    let bounded_placement = PlacementService::new(
        PlacementPolicy::with_bounds(CERT_DOMAIN, 1, 4, 100, 1_000, 10, 900, 5).unwrap(),
        FixedPlacementClock,
    );
    bounded_placement
        .seed_decision_for_snapshot_test(placement_decision("node-a", 100), 100)
        .unwrap();
    let placement_revision = bounded_placement.authority_revision().unwrap();
    let mut second = placement_decision("node-b", 100);
    second.lineage_id = "lineage-b".into();
    assert_eq!(
        bounded_placement
            .seed_decision_for_snapshot_test(second, 100)
            .unwrap_err(),
        placement::PlacementError::ResourceExhausted
    );
    assert_eq!(
        bounded_placement.authority_revision().unwrap(),
        placement_revision
    );

    let mut bounded_lease_policy = lease_policy();
    bounded_lease_policy.max_lineages = 1;
    let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, bounded_lease_policy).unwrap();
    ledger.seed_lease_for_snapshot_test(lease_state()).unwrap();
    let lease_revision = ledger.authority_revision().unwrap();
    let mut second_lease = lease_state();
    second_lease.lineage_id = b"lineage-b".to_vec();
    assert_eq!(
        ledger
            .seed_lease_for_snapshot_test(second_lease)
            .unwrap_err(),
        lease::AuthorityError::ResourceExhausted
    );
    assert_eq!(ledger.authority_revision().unwrap(), lease_revision);

    let directory = tempfile::tempdir().unwrap();
    let authority = FenceAuthority::default();
    let mut bounded_fencing_policy = fencing_policy();
    bounded_fencing_policy.max_lineages = 1;
    let mut fencing = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        directory.path().canonicalize().unwrap(),
        bounded_fencing_policy,
        Arc::new(authority),
    )
    .unwrap();
    let floor = |lineage: &[u8]| FenceReceipt {
        request_id: lineage.to_vec(),
        request_sha256: [8; 32],
        trust_domain_id: CERT_DOMAIN.as_bytes().to_vec(),
        lineage_id: lineage.to_vec(),
        epoch: 2,
        committed_log_index: 11,
        voter_set_generation: 1,
        operation_class: OperationClass::Fence as u32,
        certificate_sha256: [9; 32],
        safety_deadline_unix_millis: 2_000,
    };
    fencing
        .seed_floor_for_snapshot_test(floor(b"lineage-a"))
        .unwrap();
    let fencing_revision = fencing.authority_revision().unwrap();
    assert_eq!(
        fencing
            .seed_floor_for_snapshot_test(floor(b"lineage-b"))
            .unwrap_err(),
        FencingError::ResourceExhausted
    );
    assert_eq!(fencing.authority_revision().unwrap(), fencing_revision);
}
