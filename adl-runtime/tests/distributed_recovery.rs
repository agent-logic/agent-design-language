// PVF: lane=exact-child-tests; proof=durable quorum-bound recovery state machine;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/capability_advertisement.rs"]
mod capability_advertisement;
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
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
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use capability_advertisement::{CapabilityEvidence, VerifiedCapabilityAdvertisement};
use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::SigningKey;
use fencing::{
    ActiveLeaseCheck, FencingCheckpoint, FencingCheckpointAuthority, FencingError, FencingPolicy,
    FencingStore, TEST_FENCING_STORE_ACCESS,
};
use lease::{
    activation_signature, encode_certificate, endorse, AuthorityApplication,
    AuthorityCertificateBodyV1, AuthorityCertificateV1, AuthorityLedger, AuthorityMembership,
    ControlCertificatePurpose, LeasePolicy, LeaseState, OperationClass, VoterAuthority,
    AUTHORITY_CERTIFICATE_SCHEMA_VERSION, SIGNING_ALGORITHM_ED25519, TEST_LEASE_STORE_ACCESS,
};
use membership::{
    CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
    MembershipState,
};
use migration::{
    IsolatedRestoreAuthority, IsolatedRestoreRequest, MigrationCheckpoint,
    MigrationCheckpointAuthority, MigrationClock, MigrationError, MigrationPhase, MigrationPolicy,
    MigrationRequest, MigrationResult, MigrationStore, QuiescenceRequest,
    SourceQuiescenceAuthority,
};
use placement::{
    CapabilityRequirement, PlacementCapabilitySnapshot, PlacementClock, PlacementError,
    PlacementFencingSnapshot, PlacementInputs, PlacementPolicy, PlacementRequest, PlacementService,
    PlacementWeatherSnapshot,
};
use recovery::{
    LocalHistory, RecoveryCheckpoint, RecoveryCheckpointAuthority, RecoveryClock, RecoveryError,
    RecoveryPhase, RecoveryPolicy, RecoveryRequest, RecoveryResult, RecoveryStore,
    RecoveryTargetAuthority, RecoveryTime, TargetCleanupRequest,
};
use resource_weather::{PlacementWeather, WeatherAvailability};
use sha2::{Digest, Sha256};
use snapshot_catalog::{
    SignedSnapshotCatalogEntry, SignedTransferManifest, SnapshotCatalogEntryBody,
    SnapshotCatalogPolicy, SnapshotCatalogVerifier, SnapshotDescriptor, TransferManifestBody,
};

const DOMAIN: &str = "polis.example";
const LINEAGE: &[u8] = b"lineage-a";
const SOURCE_NODE: &[u8] = b"node-0";
const SOURCE_GUARDIAN: &[u8] = b"guardian-0";
const TARGET_NODE: &[u8] = b"node-1";
const TARGET_GUARDIAN: &[u8] = b"guardian-1";
const NOW: u64 = 20_000;

fn marker(case: &str, result: &str) {
    println!(
        "ADL_ISSUE_5876_NEGATIVE_CASE_V1 {}",
        serde_json::json!({"case": case, "result": result})
    );
}

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

#[derive(Debug, Default)]
struct FenceCheckpointAuthority(Mutex<Option<FencingCheckpoint>>);

impl FencingCheckpointAuthority for FenceCheckpointAuthority {
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

#[derive(Debug, Default)]
struct MigrationAuthority(Mutex<Option<MigrationCheckpoint>>);

impl MigrationCheckpointAuthority for MigrationAuthority {
    fn current(&self) -> MigrationResult<Option<MigrationCheckpoint>> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<MigrationCheckpoint>,
        next: MigrationCheckpoint,
    ) -> MigrationResult<()> {
        let mut current = self.0.lock().unwrap();
        if *current != expected
            || current.is_some_and(|checkpoint| next.generation <= checkpoint.generation)
        {
            return Err(MigrationError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

#[derive(Debug)]
struct TestClock(Mutex<u64>);

impl MigrationClock for TestClock {
    fn now_millis(&self) -> MigrationResult<u64> {
        Ok(*self.0.lock().unwrap())
    }
}

#[derive(Debug, Default)]
struct QuiescenceAuthority;

impl SourceQuiescenceAuthority for QuiescenceAuthority {
    fn quiesce(&self, request: QuiescenceRequest<'_>) -> MigrationResult<Vec<u8>> {
        Ok([
            b"quiesced:".as_slice(),
            request.migration_id,
            request.lineage_id,
            &request.source_epoch.to_be_bytes(),
            &request.source_log_index.to_be_bytes(),
        ]
        .concat())
    }

    fn resume(&self, _request: QuiescenceRequest<'_>) -> MigrationResult<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
struct RestoreAuthority;

impl IsolatedRestoreAuthority for RestoreAuthority {
    fn validate(&self, request: IsolatedRestoreRequest<'_>) -> MigrationResult<Vec<u8>> {
        if request.target_node_id != TARGET_NODE
            || request.target_guardian_id != TARGET_GUARDIAN
            || request.chunk_count != 2
            || request.byte_length == 0
        {
            return Err(MigrationError::RestoreRejected);
        }
        Ok([
            b"restored:".as_slice(),
            request.migration_id,
            request.transfer_id,
            &request.content_sha256,
            request.snapshot_schema.as_bytes(),
        ]
        .concat())
    }
}

#[derive(Clone, Copy, Debug)]
struct FixedClock(u64);

impl PlacementClock for FixedClock {
    fn now_unix_secs(&self) -> Result<u64, PlacementError> {
        Ok(self.0)
    }
}

struct AuthorityFixture {
    ids: Vec<Vec<u8>>,
    keys: Vec<SigningKey>,
    source_activation: SigningKey,
    target_activation: SigningKey,
}

impl AuthorityFixture {
    fn new() -> Self {
        Self {
            ids: (0..3)
                .map(|index| format!("voter-{index}").into_bytes())
                .collect(),
            keys: (1..=3).map(key).collect(),
            source_activation: key(90),
            target_activation: key(91),
        }
    }

    fn membership(&self, index: u64) -> AuthorityMembership {
        let voters = self
            .ids
            .iter()
            .zip(&self.keys)
            .map(|(id, key)| VoterAuthority {
                guardian_id: id.clone(),
                trust_domain_id: DOMAIN.as_bytes().to_vec(),
                certificate_generation: 7,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: NOW as i64 - 100,
                not_after_unix_seconds: NOW as i64 + 100,
                revoked: false,
                control_public_key: key.verifying_key().to_bytes(),
            })
            .collect::<Vec<_>>();
        AuthorityMembership::new(
            DOMAIN.as_bytes().to_vec(),
            7,
            index,
            vec![self.ids.iter().cloned().collect::<BTreeSet<_>>()],
            voters,
        )
        .unwrap()
    }

    fn certificate(&self, body: AuthorityCertificateBodyV1) -> Vec<u8> {
        let mut endorsements = [0, 1]
            .into_iter()
            .map(|position| endorse(&body, self.ids[position].clone(), 7, &self.keys[position]))
            .collect::<Vec<_>>();
        endorsements.sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
        encode_certificate(&AuthorityCertificateV1 {
            body: Some(body),
            endorsements,
        })
        .unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn body(
        &self,
        index: u64,
        epoch: u64,
        operation: OperationClass,
        node: &[u8],
        guardian: &[u8],
        activation: &SigningKey,
        issued: u64,
    ) -> AuthorityCertificateBodyV1 {
        AuthorityCertificateBodyV1 {
            schema_version: AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
            trust_domain_id: DOMAIN.as_bytes().to_vec(),
            lineage_id: LINEAGE.to_vec(),
            voter_set_generation: 7,
            raft_term: 3,
            committed_log_index: index,
            epoch,
            holder_node_id: node.to_vec(),
            holder_guardian_id: guardian.to_vec(),
            activation_key_sha256: Sha256::digest(activation.verifying_key().to_bytes()).to_vec(),
            operation_class: operation as u32,
            issued_unix_seconds: issued as i64,
            issued_nanos: 0,
            lease_duration_millis: 2_000,
            policy_sha256: lease_policy().sha256().unwrap().to_vec(),
            signing_algorithm: SIGNING_ALGORITHM_ED25519,
        }
    }
}

fn lease_policy() -> LeasePolicy {
    LeasePolicy {
        max_lease_duration_millis: 2_000,
        max_clock_uncertainty_millis: 10,
        message_delay_margin_millis: 5,
        max_lineages: 64,
        max_snapshot_bytes: 1024 * 1024,
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

fn snapshot_policy() -> SnapshotCatalogPolicy {
    SnapshotCatalogPolicy::new(DOMAIN)
        .unwrap()
        .with_bounds(8, 4096, 120, 2, 16 * 1024, 8)
        .unwrap()
}

fn placement_membership() -> MembershipState {
    let mut state = MembershipState::new(MembershipPolicy::new(DOMAIN, 32, 64).unwrap());
    let mut log_index = 5_u64;
    for index in 0..3_u8 {
        state
            .apply(&CommittedMembershipEvent::new(
                DOMAIN,
                [index.saturating_add(1); 32],
                u64::from(index) + 1,
                log_index,
                MembershipOperation::Join {
                    member: Member {
                        node_id: format!("node-{index}"),
                        guardian_id: format!("guardian-{index}"),
                        identity_generation: 3,
                        guardian_control_public_key: [index.saturating_add(1); 32],
                        role: MemberRole::NonVoting,
                    },
                },
            ))
            .unwrap();
        log_index += 1;
    }
    for index in 0..3_u8 {
        state
            .apply(&CommittedMembershipEvent::new(
                DOMAIN,
                [index.saturating_add(10); 32],
                u64::from(index) + 4,
                9 + u64::from(index),
                MembershipOperation::Promote {
                    node_id: format!("node-{index}"),
                },
            ))
            .unwrap();
    }
    assert_eq!(state.committed_log_index(), 11);
    state
}

struct Fixture {
    directory: tempfile::TempDir,
    migration_root: PathBuf,
    migration_authority: Arc<MigrationAuthority>,
    migration_clock: Arc<TestClock>,
    fencing: FencingStore,
    authority: AuthorityFixture,
    ledger: AuthorityLedger,
    source_proof: Vec<u8>,
    snapshot_verifier: SnapshotCatalogVerifier,
    snapshot_policy: SnapshotCatalogPolicy,
    snapshot_signer: SigningKey,
    snapshot_certificate: AuthorityCertificate,
    chunks: Vec<Vec<u8>>,
    placement_membership: MembershipState,
    placement_capabilities: PlacementCapabilitySnapshot,
    placement_weather: PlacementWeatherSnapshot,
    placement_fencing: PlacementFencingSnapshot,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let migration_root = root.join("migration");
        let fencing_root = root.join("fencing");
        fs::create_dir(&migration_root).unwrap();
        fs::create_dir(&fencing_root).unwrap();
        let migration_authority = Arc::new(MigrationAuthority::default());
        let migration_clock = Arc::new(TestClock(Mutex::new(NOW * 1_000)));
        let fencing = FencingStore::create(
            &TEST_FENCING_STORE_ACCESS,
            &fencing_root,
            fencing_policy(),
            Arc::new(FenceCheckpointAuthority::default()),
        )
        .unwrap();

        let authority = AuthorityFixture::new();
        let membership = authority.membership(11);
        let grant_body = authority.body(
            11,
            1,
            OperationClass::LeaseGrant,
            SOURCE_NODE,
            SOURCE_GUARDIAN,
            &authority.source_activation,
            NOW,
        );
        let source_signature = activation_signature(&grant_body, &authority.source_activation);
        let grant = authority.certificate(grant_body);
        let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
        ledger
            .apply(
                &TEST_LEASE_STORE_ACCESS,
                &grant,
                &membership,
                AuthorityApplication {
                    now_unix_seconds: NOW as i64,
                    now_unix_nanos: 0,
                    now_elapsed_millis: 10,
                    clock_uncertainty_millis: 5,
                    activation_public_key: authority.source_activation.verifying_key().to_bytes(),
                    activation_proof: &source_signature,
                },
            )
            .unwrap();

        let issuer = key(40);
        let snapshot_signer = key(41);
        let certificate_store = Arc::new(
            DistributedCertificateStore::open(
                &TEST_CERTIFICATE_STORE_ACCESS,
                root.join("certificates.redb"),
                CertificatePolicy::new(DOMAIN, [issuer.verifying_key()])
                    .unwrap()
                    .with_bounds(3600, 60, 60, 64, 64)
                    .unwrap(),
            )
            .unwrap(),
        );
        let snapshot_certificate = AuthorityCertificate::issue(
            CertificateBody::new(
                DOMAIN,
                String::from_utf8(SOURCE_NODE.to_vec()).unwrap(),
                CertificatePurpose::SnapshotSigning,
                1,
                CertificateValidity {
                    issued_at_unix_secs: NOW - 100,
                    expires_at_unix_secs: NOW + 600,
                },
                snapshot_signer.verifying_key(),
                &issuer.verifying_key(),
            ),
            &issuer,
        )
        .unwrap();
        certificate_store
            .activate(
                &TEST_CERTIFICATE_STORE_ACCESS,
                &snapshot_certificate,
                NOW - 10,
            )
            .unwrap();
        let snapshot_policy = snapshot_policy();
        let snapshot_verifier = SnapshotCatalogVerifier::open_for_test(
            certificate_store,
            snapshot_policy.clone(),
            root.join("snapshot-replay.redb"),
        )
        .unwrap();

        let placement_membership = placement_membership();
        let capability = VerifiedCapabilityAdvertisement {
            trust_domain: DOMAIN.to_owned(),
            issuer_id: String::from_utf8(TARGET_GUARDIAN.to_vec()).unwrap(),
            certificate_id: "certificate-target".to_owned(),
            certificate_generation: 3,
            sequence: 7,
            measured_at_unix_secs: NOW - 2,
            expires_at_unix_secs: NOW + 60,
            verification_deadline_unix_secs: NOW + 60,
            capabilities: vec![CapabilityEvidence::new("cpu", 16)],
        };
        let placement_capabilities =
            PlacementCapabilitySnapshot::from_rows_for_test(NOW, &[capability]);
        let weather = PlacementWeather {
            holder_id: String::from_utf8(TARGET_GUARDIAN.to_vec()).unwrap(),
            certificate_generation: 3,
            sequence: 8,
            sampled_at_unix_secs: NOW - 1,
            expires_at_unix_secs: NOW + 60,
            availability: WeatherAvailability::Available,
            pressure_permille: Some(100),
            available_slots: Some(8),
            advisory_only: true,
        };
        let placement_weather = PlacementWeatherSnapshot::from_rows_for_test(
            NOW,
            &[(weather, "certificate-target".to_owned())],
        );
        let placement_policy = PlacementPolicy::new(DOMAIN).unwrap();
        let placement_fencing = PlacementFencingSnapshot::from_receipts_for_test(
            &placement_policy,
            &placement_membership,
            &[],
        )
        .unwrap();
        Self {
            directory,
            migration_root,
            migration_authority,
            migration_clock,
            fencing,
            authority,
            ledger,
            source_proof: source_signature.to_vec(),
            snapshot_verifier,
            snapshot_policy,
            snapshot_signer,
            snapshot_certificate,
            chunks: vec![b"private-state-a".to_vec(), b"private-state-b".to_vec()],
            placement_membership,
            placement_capabilities,
            placement_weather,
            placement_fencing,
        }
    }

    fn migration_store(&self) -> MigrationStore {
        MigrationStore::create(
            &self.migration_root,
            MigrationPolicy::new(DOMAIN).unwrap(),
            self.migration_authority.clone(),
            self.migration_clock.clone(),
        )
        .unwrap()
    }

    fn source_lease(&self) -> &LeaseState {
        self.ledger.lease(LINEAGE).unwrap()
    }

    fn snapshot(&self) -> SnapshotDescriptor {
        let mut whole = Sha256::new();
        let mut byte_length = 0_u64;
        let chunk_sha256 = self
            .chunks
            .iter()
            .map(|chunk| {
                whole.update(chunk);
                byte_length += chunk.len() as u64;
                Sha256::digest(chunk).into()
            })
            .collect();
        SnapshotDescriptor {
            trust_domain: DOMAIN.to_owned(),
            lineage_id: LINEAGE.to_vec(),
            source_owner_id: SOURCE_NODE.to_vec(),
            source_guardian_id: SOURCE_GUARDIAN.to_vec(),
            source_epoch: 1,
            authority_log_index: 11,
            snapshot_schema: "adl.private_state.v1".to_owned(),
            content_sha256: whole.finalize().into(),
            byte_length,
            chunk_sha256,
            created_at_unix_secs: NOW - 1,
            expires_at_unix_secs: NOW + 60,
            encryption_context_id: "context-7".to_owned(),
        }
    }

    fn catalog(&self) -> SignedSnapshotCatalogEntry {
        SignedSnapshotCatalogEntry::issue(
            SnapshotCatalogEntryBody::new(
                String::from_utf8(SOURCE_NODE.to_vec()).unwrap(),
                1,
                1,
                self.snapshot(),
                &self.snapshot_policy,
            )
            .unwrap(),
            self.snapshot_certificate.clone(),
            &self.snapshot_signer,
            &self.snapshot_policy,
        )
        .unwrap()
    }

    fn manifest(&self, catalog: &SignedSnapshotCatalogEntry) -> SignedTransferManifest {
        SignedTransferManifest::issue(
            TransferManifestBody::new(b"transfer-1", TARGET_NODE, catalog, &self.snapshot_policy)
                .unwrap(),
            self.snapshot_certificate.clone(),
            &self.snapshot_signer,
            &self.snapshot_policy,
        )
        .unwrap()
    }

    fn prepare(&self, store: &mut MigrationStore) {
        let policy = PlacementPolicy::new(DOMAIN).unwrap();
        let service = PlacementService::new(policy, FixedClock(NOW));
        let request = PlacementRequest {
            lineage_id: String::from_utf8(LINEAGE.to_vec()).unwrap(),
            minimum_membership_epoch: self.placement_membership.epoch(),
            minimum_committed_log_index: self.placement_membership.committed_log_index(),
            required_slots: 1,
            requirements: vec![CapabilityRequirement::new("cpu", 1)],
        };
        let membership = self.authority.membership(11);
        let source_check = ActiveLeaseCheck {
            membership: Some(&membership),
            lease: self.source_lease(),
            applied_log_index: 11,
            now_unix_seconds: NOW as i64,
            now_unix_millis: NOW * 1_000,
            now_elapsed_millis: 20,
            activation_proof: &self.source_proof,
        };
        store
            .prepare(
                MigrationRequest {
                    migration_id: b"migration-1".to_vec(),
                    trust_domain: DOMAIN.to_owned(),
                    lineage_id: LINEAGE.to_vec(),
                    source_node_id: SOURCE_NODE.to_vec(),
                    source_guardian_id: SOURCE_GUARDIAN.to_vec(),
                    timeout_millis: 60_000,
                },
                &service,
                &request,
                PlacementInputs {
                    membership: &self.placement_membership,
                    capabilities: &self.placement_capabilities,
                    weather: &self.placement_weather,
                    fencing: &self.placement_fencing,
                },
                &self.fencing,
                source_check,
            )
            .unwrap();
    }
}

fn run_to_validated(fixture: &mut Fixture, store: &mut MigrationStore) {
    fixture.prepare(store);
    let membership = fixture.authority.membership(11);
    let source_check = ActiveLeaseCheck {
        membership: Some(&membership),
        lease: fixture.source_lease(),
        applied_log_index: 11,
        now_unix_seconds: NOW as i64,
        now_unix_millis: NOW * 1_000,
        now_elapsed_millis: 20,
        activation_proof: &fixture.source_proof,
    };
    store
        .quiesce(
            b"migration-1",
            &QuiescenceAuthority,
            &fixture.fencing,
            source_check,
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    let membership = fixture.authority.membership(11);
    store
        .quiesce(
            b"migration-1",
            &QuiescenceAuthority,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
    let catalog = fixture.catalog();
    let catalog_bytes = catalog.encode(&fixture.snapshot_policy).unwrap();
    let membership = fixture.authority.membership(11);
    store
        .checkpoint_snapshot(
            b"migration-1",
            &catalog_bytes,
            &fixture.snapshot_verifier,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    let membership = fixture.authority.membership(11);
    store
        .checkpoint_snapshot(
            b"migration-1",
            &catalog_bytes,
            &fixture.snapshot_verifier,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
    let manifest = fixture.manifest(&catalog);
    let manifest_bytes = manifest.encode(&fixture.snapshot_policy).unwrap();
    let membership = fixture.authority.membership(11);
    store
        .transfer(
            b"migration-1",
            &manifest_bytes,
            &catalog_bytes,
            &fixture.chunks,
            &fixture.snapshot_verifier,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    let membership = fixture.authority.membership(11);
    store
        .transfer(
            b"migration-1",
            &manifest_bytes,
            &catalog_bytes,
            &fixture.chunks,
            &fixture.snapshot_verifier,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
    let membership = fixture.authority.membership(11);
    store
        .validate_restore(
            b"migration-1",
            &RestoreAuthority,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    let membership = fixture.authority.membership(11);
    store
        .validate_restore(
            b"migration-1",
            &RestoreAuthority,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.source_lease(),
                applied_log_index: 11,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &fixture.source_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
}

#[derive(Debug, Default)]
struct RecoveryAuthority {
    checkpoint: Mutex<Option<RecoveryCheckpoint>>,
    fail_generation: Mutex<Option<u64>>,
}

impl RecoveryAuthority {
    fn fail_generation(&self, generation: u64) {
        *self.fail_generation.lock().unwrap() = Some(generation);
    }
}

impl RecoveryCheckpointAuthority for RecoveryAuthority {
    fn current(&self) -> RecoveryResult<Option<RecoveryCheckpoint>> {
        Ok(*self.checkpoint.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<RecoveryCheckpoint>,
        next: RecoveryCheckpoint,
    ) -> RecoveryResult<()> {
        if *self.fail_generation.lock().unwrap() == Some(next.generation) {
            *self.fail_generation.lock().unwrap() = None;
            return Err(RecoveryError::DurabilityFailure);
        }
        let mut current = self.checkpoint.lock().unwrap();
        if *current != expected
            || current.is_some_and(|checkpoint| next.generation <= checkpoint.generation)
        {
            return Err(RecoveryError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

#[derive(Debug)]
struct RecoveryTestClock(Mutex<RecoveryTime>);

impl RecoveryTestClock {
    fn set(&self, unix_seconds: i64, elapsed_millis: u64) {
        *self.0.lock().unwrap() = RecoveryTime {
            unix_seconds,
            unix_nanos: 0,
            elapsed_millis,
            clock_uncertainty_millis: 5,
        };
    }

    fn time(&self) -> RecoveryTime {
        *self.0.lock().unwrap()
    }
}

impl RecoveryClock for RecoveryTestClock {
    fn now(&self) -> RecoveryResult<RecoveryTime> {
        Ok(self.time())
    }
}

struct RecoveryHarness {
    root: PathBuf,
    authority: Arc<RecoveryAuthority>,
    clock: Arc<RecoveryTestClock>,
    policy: RecoveryPolicy,
}

#[derive(Debug, Default)]
struct TargetAuthority(Mutex<Vec<Vec<u8>>>);

impl RecoveryTargetAuthority for TargetAuthority {
    fn discard_incomplete(&self, request: TargetCleanupRequest<'_>) -> RecoveryResult<Vec<u8>> {
        if request.recovery_id.is_empty()
            || request.migration_id != b"migration-1"
            || request.target_node_id != TARGET_NODE
            || request.target_guardian_id != TARGET_GUARDIAN
            || request.remaining_timeout_millis == 0
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        self.0.lock().unwrap().push(request.recovery_id.to_vec());
        Ok([
            b"discarded:".as_slice(),
            request.recovery_id,
            request.migration_id,
            request.transfer_id.unwrap_or_default(),
            request
                .content_sha256
                .as_ref()
                .map_or(&[][..], |digest| digest.as_slice()),
        ]
        .concat())
    }
}

#[derive(Debug)]
struct ExpiringTargetAuthority(Arc<RecoveryTestClock>);

impl RecoveryTargetAuthority for ExpiringTargetAuthority {
    fn discard_incomplete(&self, request: TargetCleanupRequest<'_>) -> RecoveryResult<Vec<u8>> {
        self.0.set(NOW as i64, 226);
        Ok([b"discarded-after-deadline:".as_slice(), request.recovery_id].concat())
    }
}

impl RecoveryHarness {
    fn new(fixture: &Fixture) -> Self {
        let root = fixture.directory.path().join("recovery");
        fs::create_dir(&root).unwrap();
        Self {
            root,
            authority: Arc::new(RecoveryAuthority::default()),
            clock: Arc::new(RecoveryTestClock(Mutex::new(RecoveryTime {
                unix_seconds: NOW as i64,
                unix_nanos: 0,
                elapsed_millis: 25,
                clock_uncertainty_millis: 5,
            }))),
            policy: RecoveryPolicy::new(DOMAIN).unwrap(),
        }
    }

    fn with_policy(fixture: &Fixture, policy: RecoveryPolicy) -> Self {
        let mut harness = Self::new(fixture);
        harness.policy = policy;
        harness
    }

    fn create(&self) -> RecoveryStore {
        RecoveryStore::create(
            &self.root,
            self.policy.clone(),
            self.authority.clone(),
            self.clock.clone(),
        )
        .unwrap()
    }

    fn open(&self) -> RecoveryStore {
        RecoveryStore::open(
            &self.root,
            self.policy.clone(),
            self.authority.clone(),
            self.clock.clone(),
        )
        .unwrap()
    }
}

#[cfg(feature = "internal-test-fixtures")]
#[derive(Debug, Default)]
struct ServingCheckpointAuthority(
    Mutex<BTreeMap<String, distributed::polis_runtime::ConsensusCheckpoint>>,
);

#[cfg(feature = "internal-test-fixtures")]
impl distributed::polis_runtime::ConsensusCheckpointAuthority for ServingCheckpointAuthority {
    fn load(
        &self,
        object: &str,
    ) -> Result<
        Option<distributed::polis_runtime::ConsensusCheckpoint>,
        distributed::polis_runtime::PolisRuntimeError,
    > {
        Ok(self.0.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&distributed::polis_runtime::ConsensusCheckpoint>,
        candidate: &distributed::polis_runtime::ConsensusCheckpoint,
    ) -> Result<(), distributed::polis_runtime::PolisRuntimeError> {
        let mut current = self.0.lock().unwrap();
        if current.get(&candidate.object) != expected {
            return Err(distributed::polis_runtime::PolisRuntimeError::StateRegression);
        }
        current.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

#[cfg(feature = "internal-test-fixtures")]
fn observatory_input_for(
    operation: &str,
    lineage: &str,
) -> (
    distributed::authority_protocol::PublishedAuthorityResult,
    distributed::serving_authority::VerifiedServingAuthorityCut,
) {
    let mut fixture = distributed::serving_authority::ObservatoryBindingFixture::new(operation);
    fixture.set_invalid_identifier(
        distributed::serving_authority::ObservatoryIdentifierField::LineageId,
        lineage,
    );
    fixture.set_invalid_identifier(
        distributed::serving_authority::ObservatoryIdentifierField::OperationId,
        operation,
    );
    fixture.set_operation(operation, 2);
    fixture.set_integers(2, 1, 1);
    fixture.set_transition(
        distributed::serving_authority::ObservatoryTransitionAction::Acquire,
        None,
    );
    (
        distributed::authority_protocol::test_observatory_published_authority_for_operation(
            fixture.artifact_bytes(),
            operation,
            2,
        ),
        distributed::serving_authority::VerifiedServingAuthorityCut::fixture_from_observatory(
            &fixture,
        ),
    )
}

#[cfg(feature = "internal-test-fixtures")]
fn shepherd_cut_for(lineage: &str) -> distributed::serving_authority::VerifiedServingAuthorityCut {
    distributed::serving_authority::VerifiedServingAuthorityCut::fixture(
        lineage.into(),
        7,
        format!("owner-{lineage}"),
        9,
        format!("lease-{lineage}"),
        "11".repeat(32),
        "22".repeat(32),
        "33".repeat(32),
    )
}

#[cfg(feature = "internal-test-fixtures")]
fn committed_pair_for_lineage(
    root: &Path,
    authority: Arc<ServingCheckpointAuthority>,
    lineage: &str,
) -> (
    distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection,
    distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection,
) {
    let shepherd_root = root.join(format!("shepherd-{lineage}"));
    let observatory_root = root.join(format!("observatory-{lineage}"));
    fs::create_dir(&shepherd_root).unwrap();
    fs::create_dir(&observatory_root).unwrap();
    let mut shepherd = distributed::shepherd_serving_eligibility::ShepherdEligibilityStore::open(
        &shepherd_root,
        authority.clone(),
        8,
    )
    .unwrap();
    let shepherd_cut = shepherd_cut_for(lineage);
    shepherd
        .acquire(
            "shepherd-acquire",
            "shepherd-a",
            b"raw-secret-permit",
            &shepherd_cut,
            100,
            1,
        )
        .unwrap();
    let mut observatory =
        distributed::observatory_serving_eligibility::ObservatoryEligibilityStore::open(
            &observatory_root,
            authority,
            8,
        )
        .unwrap();
    let (published, cut) = observatory_input_for("observatory-acquire", lineage);
    observatory
        .apply(&published, &cut, 1_700_000_000, 123_456_789)
        .unwrap();
    (
        shepherd.committed_projection().unwrap().unwrap(),
        observatory.committed_projection().unwrap().unwrap(),
    )
}

#[cfg(feature = "internal-test-fixtures")]
fn serving_lineage_ref(lineage_id: &[u8]) -> String {
    let lineage = std::str::from_utf8(lineage_id).unwrap();
    hex::encode(Sha256::digest(format!(
        "ADL-SERVING-REF-V1\0lineage\0{lineage}"
    )))
}

fn histories() -> Vec<LocalHistory> {
    vec![
        LocalHistory {
            node_id: SOURCE_NODE.to_vec(),
            guardian_id: SOURCE_GUARDIAN.to_vec(),
            claimed_epoch: 90,
            claimed_log_index: 9_000,
            claimed_owner: true,
        },
        LocalHistory {
            node_id: TARGET_NODE.to_vec(),
            guardian_id: TARGET_GUARDIAN.to_vec(),
            claimed_epoch: 91,
            claimed_log_index: 9_001,
            claimed_owner: true,
        },
    ]
}

fn begin_request(id: &[u8], timeout_millis: u64) -> RecoveryRequest {
    RecoveryRequest {
        recovery_id: id.to_vec(),
        migration_id: b"migration-1".to_vec(),
        trust_domain: DOMAIN.to_owned(),
        timeout_millis,
    }
}

fn begin_and_plan(
    store: &mut RecoveryStore,
    migration: &MigrationStore,
    fixture: &Fixture,
    recovery_id: &[u8],
) {
    let assessing = store
        .begin(begin_request(recovery_id, 60_000), migration, &histories())
        .unwrap();
    assert_eq!(assessing.phase, RecoveryPhase::Assessing);
    assert_eq!(assessing.owner_epoch, None);
    assert_eq!(assessing.owner_node_id, None);
    assert_eq!(assessing.committed_log_index, None);

    let snapshot = fixture.ledger.snapshot().unwrap();
    let planned = store
        .select_committed_prefix(
            recovery_id,
            &[snapshot],
            &lease_policy(),
            &fixture.authority.membership(11),
        )
        .unwrap();
    assert_eq!(planned.phase, RecoveryPhase::Planned);
    assert_eq!(planned.committed_prefix_epoch, Some(1));
    assert_eq!(planned.committed_prefix_log_index, Some(11));
    assert_eq!(planned.owner_epoch, None);
    assert_eq!(planned.owner_node_id, None);
}

fn application<'a>(
    time: RecoveryTime,
    activation: &'a SigningKey,
    proof: &'a [u8],
) -> AuthorityApplication<'a> {
    AuthorityApplication {
        now_unix_seconds: time.unix_seconds,
        now_unix_nanos: time.unix_nanos,
        now_elapsed_millis: time.elapsed_millis,
        clock_uncertainty_millis: time.clock_uncertainty_millis,
        activation_public_key: activation.verifying_key().to_bytes(),
        activation_proof: proof,
    }
}

fn active_check<'a>(
    fixture: &'a Fixture,
    membership: &'a AuthorityMembership,
    time: RecoveryTime,
) -> ActiveLeaseCheck<'a> {
    ActiveLeaseCheck {
        membership: Some(membership),
        lease: fixture.ledger.lease(LINEAGE).unwrap(),
        applied_log_index: fixture.ledger.applied_log_index(),
        now_unix_seconds: time.unix_seconds,
        now_unix_millis: u64::try_from(time.unix_seconds).unwrap() * 1_000,
        now_elapsed_millis: time.elapsed_millis,
        activation_proof: &fixture.source_proof,
    }
}

fn recover_target_to_committed(
    fixture: &mut Fixture,
    migration: &MigrationStore,
    harness: &RecoveryHarness,
    store: &mut RecoveryStore,
    recovery_id: &[u8],
) -> recovery::RecoveryRecord {
    begin_and_plan(store, migration, fixture, recovery_id);

    let membership12 = fixture.authority.membership(12);
    let fence_body = fixture.authority.body(
        12,
        2,
        OperationClass::Fence,
        SOURCE_NODE,
        SOURCE_GUARDIAN,
        &fixture.authority.source_activation,
        NOW,
    );
    let fence_certificate = fixture.authority.certificate(fence_body);
    let time = harness.clock.time();
    store
        .fence_ambiguous(
            recovery_id,
            b"fence-recovery-serving",
            &fence_certificate,
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.source_activation, &[]),
        )
        .unwrap();

    harness.clock.set(NOW as i64 + 3, 3_020);
    let time = harness.clock.time();
    let membership13 = fixture.authority.membership(13);
    let activate_body = fixture.authority.body(
        13,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let activate_proof = activation_signature(&activate_body, &fixture.authority.target_activation);
    let activate_certificate = fixture.authority.certificate(activate_body);
    store
        .restore_quorum_owner(
            recovery_id,
            &activate_certificate,
            &membership13,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &activate_proof),
        )
        .unwrap();

    let membership14 = fixture.authority.membership(14);
    let commit_body = fixture.authority.body(
        14,
        2,
        OperationClass::OwnerCommit,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let commit_proof = activation_signature(&commit_body, &fixture.authority.target_activation);
    let commit_certificate = fixture.authority.certificate(commit_body);
    store
        .commit_owner(
            recovery_id,
            &commit_certificate,
            &membership14,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &commit_proof),
        )
        .unwrap()
}

#[test]
fn recovery_begin_and_committed_prefix_survive_restart_without_trusting_local_owner_claims() {
    let mut fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    run_to_validated(&mut fixture, &mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut store = harness.create();

    begin_and_plan(&mut store, &migration, &fixture, b"recovery-durable");
    let checkpoint = store.checkpoint();
    drop(store);

    let reopened = harness.open();
    assert_eq!(reopened.checkpoint(), checkpoint);
    let record = reopened.record(b"recovery-durable").unwrap();
    assert_eq!(record.phase, RecoveryPhase::Planned);
    assert_eq!(record.committed_prefix_epoch, Some(1));
    assert_eq!(record.owner_epoch, None);
    assert_eq!(record.owner_node_id, None);
    marker("local_claimed_owner_not_authority", "rejected");
}

#[test]
fn recovery_pre_fence_rollback_requires_live_source_authority() {
    let fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut store = harness.create();
    begin_and_plan(&mut store, &migration, &fixture, b"recovery-rollback");
    let membership = fixture.authority.membership(11);
    let time = harness.clock.time();

    let wrong_membership = fixture.authority.membership(12);
    assert_eq!(
        store.rollback_pre_fence(
            b"recovery-rollback",
            &mut migration,
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &wrong_membership, time),
        ),
        Err(RecoveryError::AuthorityRejected)
    );
    marker("rollback_without_live_source_authority", "rejected");

    let restored = store
        .rollback_pre_fence(
            b"recovery-rollback",
            &mut migration,
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &membership, time),
        )
        .unwrap();
    assert_eq!(restored.phase, RecoveryPhase::Restored);
    assert_eq!(restored.owner_node_id.as_deref(), Some(SOURCE_NODE));
    assert_eq!(restored.owner_epoch, Some(1));
    assert_eq!(
        migration.record(b"migration-1").unwrap().phase,
        MigrationPhase::Aborted
    );
}

#[test]
fn recovery_quorum_fence_safety_window_activation_commit_and_restart() {
    let mut fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut store = harness.create();
    begin_and_plan(&mut store, &migration, &fixture, b"recovery-owner");

    let membership12 = fixture.authority.membership(12);
    let fence_body = fixture.authority.body(
        12,
        2,
        OperationClass::Fence,
        SOURCE_NODE,
        SOURCE_GUARDIAN,
        &fixture.authority.source_activation,
        NOW,
    );
    let mut fence_message = AuthorityCertificateV1 {
        body: Some(fence_body.clone()),
        endorsements: vec![endorse(
            &fence_body,
            fixture.authority.ids[0].clone(),
            7,
            &fixture.authority.keys[0],
        )],
    };
    let minority = encode_certificate(&fence_message).unwrap();
    let time = harness.clock.time();
    assert_eq!(
        store.fence_ambiguous(
            b"recovery-owner",
            b"fence-minority",
            &minority,
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.source_activation, &[]),
        ),
        Err(RecoveryError::QuorumRequired)
    );
    marker("minority_fence", "rejected");

    let mut wrong_domain_body = fence_body.clone();
    wrong_domain_body.trust_domain_id = b"other.example".to_vec();
    fence_message.body = Some(wrong_domain_body.clone());
    fence_message.endorsements = [0, 1]
        .into_iter()
        .map(|position| {
            endorse(
                &wrong_domain_body,
                fixture.authority.ids[position].clone(),
                7,
                &fixture.authority.keys[position],
            )
        })
        .collect();
    fence_message
        .endorsements
        .sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
    assert_eq!(
        store.fence_ambiguous(
            b"recovery-owner",
            b"fence-wrong-domain",
            &encode_certificate(&fence_message).unwrap(),
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.source_activation, &[]),
        ),
        Err(RecoveryError::QuorumRequired)
    );
    marker("wrong_trust_domain_fence", "rejected");

    let fence_certificate = fixture.authority.certificate(fence_body);
    harness
        .authority
        .fail_generation(store.checkpoint().generation + 2);
    assert_eq!(
        store.fence_ambiguous(
            b"recovery-owner",
            b"fence-recovery-owner",
            &fence_certificate,
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.source_activation, &[]),
        ),
        Err(RecoveryError::DurabilityFailure)
    );
    assert_eq!(
        store.record(b"recovery-owner").unwrap().phase,
        RecoveryPhase::FencePending
    );
    assert!(fixture.fencing.floor(LINEAGE).is_some());
    assert!(fixture.ledger.lease(LINEAGE).unwrap().revoked);
    marker("pending_fence_crash_reconciliation", "recovered");
    let fenced = store
        .fence_ambiguous(
            b"recovery-owner",
            b"fence-recovery-owner",
            &fence_certificate,
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.source_activation, &[]),
        )
        .unwrap();
    assert_eq!(fenced.phase, RecoveryPhase::Fenced);

    let membership13 = fixture.authority.membership(13);
    let early_body = fixture.authority.body(
        13,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW,
    );
    let early_proof = activation_signature(&early_body, &fixture.authority.target_activation);
    let early_certificate = fixture.authority.certificate(early_body);
    assert_eq!(
        store.restore_quorum_owner(
            b"recovery-owner",
            &early_certificate,
            &membership13,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &early_proof),
        ),
        Err(RecoveryError::SafetyWindow)
    );
    marker("activation_inside_safety_window", "rejected");

    harness.clock.set(NOW as i64 + 3, 3_020);
    let time = harness.clock.time();
    let activate_body = fixture.authority.body(
        13,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let activate_proof = activation_signature(&activate_body, &fixture.authority.target_activation);
    let activate_certificate = fixture.authority.certificate(activate_body);
    let restored = store
        .restore_quorum_owner(
            b"recovery-owner",
            &activate_certificate,
            &membership13,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &activate_proof),
        )
        .unwrap();
    assert_eq!(restored.phase, RecoveryPhase::Restored);
    assert_eq!(restored.owner_node_id.as_deref(), Some(TARGET_NODE));

    let membership14 = fixture.authority.membership(14);
    let stale_membership = fixture.authority.membership(13);
    let commit_body = fixture.authority.body(
        14,
        2,
        OperationClass::OwnerCommit,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let commit_proof = activation_signature(&commit_body, &fixture.authority.target_activation);
    let commit_certificate = fixture.authority.certificate(commit_body);
    assert_eq!(
        store.commit_owner(
            b"recovery-owner",
            &commit_certificate,
            &stale_membership,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &commit_proof),
        ),
        Err(RecoveryError::QuorumRequired)
    );
    marker("stale_membership_generation", "rejected");

    let committed = store
        .commit_owner(
            b"recovery-owner",
            &commit_certificate,
            &membership14,
            &mut fixture.ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &commit_proof),
        )
        .unwrap();
    assert_eq!(committed.phase, RecoveryPhase::Committed);
    assert_eq!(committed.committed_log_index, Some(14));
    let checkpoint = store.checkpoint();
    drop(store);
    let reopened = harness.open();
    assert_eq!(reopened.checkpoint(), checkpoint);
    assert_eq!(
        reopened.record(b"recovery-owner").unwrap().phase,
        RecoveryPhase::Committed
    );
}

#[cfg(feature = "internal-test-fixtures")]
#[test]
fn recovery_committed_owner_transfers_serving_authority_and_retries_exactly() {
    let mut fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut recovery = harness.create();
    let committed = recover_target_to_committed(
        &mut fixture,
        &migration,
        &harness,
        &mut recovery,
        b"recovery-serving",
    );
    assert_eq!(committed.phase, RecoveryPhase::Committed);

    let root = fixture.directory.path().canonicalize().unwrap();
    let serving_root = root.join("integrated-serving-recovery");
    fs::create_dir(&serving_root).unwrap();
    let serving_authority = Arc::new(ServingCheckpointAuthority::default());
    let lineage_ref = serving_lineage_ref(LINEAGE);
    let (shepherd, observatory) = committed_pair_for_lineage(
        &root,
        serving_authority.clone(),
        std::str::from_utf8(LINEAGE).unwrap(),
    );
    let pair = distributed::shepherd_serving_eligibility::verify_committed_child_lineage_pair(
        &shepherd,
        &observatory,
    )
    .unwrap();
    let mut integrated =
        integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority.clone(),
            8,
        )
        .unwrap();
    integrated
        .observe(
            "serving-before-recovery",
            &pair,
            integrated_serving_authority_snapshot::IntegratedOutcome::Success,
        )
        .unwrap();
    drop(integrated);
    let mut integrated =
        integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority.clone(),
            8,
        )
        .unwrap();
    let latest = integrated.recoverable_latest_receipt().unwrap().unwrap();
    assert_eq!(latest.shepherd.lineage_ref, lineage_ref);
    assert_eq!(latest.observatory.lineage_ref, lineage_ref);

    let transferred = recovery
        .transfer_serving_authority(b"recovery-serving", &mut integrated)
        .unwrap();
    assert_eq!(transferred.phase, RecoveryPhase::ServingTransferred);
    assert!(transferred.serving_operation_ref.is_some());
    assert!(transferred.serving_receipt_sha256.is_some());
    let generation = recovery.checkpoint().generation;
    assert_eq!(
        recovery
            .transfer_serving_authority(b"recovery-serving", &mut integrated)
            .unwrap(),
        transferred
    );
    assert_eq!(recovery.checkpoint().generation, generation);
    drop(recovery);
    drop(integrated);

    let mut reopened_recovery = harness.open();
    let mut reopened_integrated =
        integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority,
            8,
        )
        .unwrap();
    assert_eq!(
        reopened_recovery
            .transfer_serving_authority(b"recovery-serving", &mut reopened_integrated)
            .unwrap(),
        transferred
    );
    marker("recovery_serving_transfer", "recovered");
}

#[cfg(feature = "internal-test-fixtures")]
#[test]
fn recovery_serving_transfer_rejects_wrong_integrated_lineage_before_advancing() {
    let mut fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut recovery = harness.create();
    recover_target_to_committed(
        &mut fixture,
        &migration,
        &harness,
        &mut recovery,
        b"recovery-wrong-serving",
    );

    let root = fixture.directory.path().canonicalize().unwrap();
    let serving_root = root.join("integrated-serving-wrong-lineage");
    fs::create_dir(&serving_root).unwrap();
    let serving_authority = Arc::new(ServingCheckpointAuthority::default());
    let (shepherd, observatory) =
        committed_pair_for_lineage(&root, serving_authority.clone(), &"44".repeat(32));
    let pair = distributed::shepherd_serving_eligibility::verify_committed_child_lineage_pair(
        &shepherd,
        &observatory,
    )
    .unwrap();
    let mut integrated =
        integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority.clone(),
            8,
        )
        .unwrap();
    integrated
        .observe(
            "serving-before-wrong-recovery",
            &pair,
            integrated_serving_authority_snapshot::IntegratedOutcome::Success,
        )
        .unwrap();
    drop(integrated);
    let mut integrated =
        integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority,
            8,
        )
        .unwrap();

    assert_eq!(
        recovery.transfer_serving_authority(b"recovery-wrong-serving", &mut integrated),
        Err(RecoveryError::ServingTransferRejected)
    );
    assert_eq!(
        recovery.record(b"recovery-wrong-serving").unwrap().phase,
        RecoveryPhase::ServingTransferPending
    );
    marker("wrong_serving_lineage_recovery", "rejected");
}

#[test]
fn recovery_refences_a_real_activated_migration_before_owner_commit() {
    let mut fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    run_to_validated(&mut fixture, &mut migration);

    let membership12 = fixture.authority.membership(12);
    let fence_body = fixture.authority.body(
        12,
        2,
        OperationClass::Fence,
        SOURCE_NODE,
        SOURCE_GUARDIAN,
        &fixture.authority.source_activation,
        NOW,
    );
    let fence_certificate = fixture.authority.certificate(fence_body);
    migration
        .fence(
            b"migration-1",
            b"migration-activation-fence",
            &fence_certificate,
            &membership12,
            &mut fixture.ledger,
            &mut fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64,
                now_unix_nanos: 0,
                now_elapsed_millis: 30,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .source_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &[],
            },
        )
        .unwrap();

    let membership13 = fixture.authority.membership(13);
    let activate_body = fixture.authority.body(
        13,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let activate_proof = activation_signature(&activate_body, &fixture.authority.target_activation);
    let activate_certificate = fixture.authority.certificate(activate_body);
    migration
        .activate(
            b"migration-1",
            &activate_certificate,
            &membership13,
            &mut fixture.ledger,
            &fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 3,
                now_unix_nanos: 0,
                now_elapsed_millis: 3_020,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &activate_proof,
            },
        )
        .unwrap();
    assert_eq!(
        migration.record(b"migration-1").unwrap().phase,
        MigrationPhase::Activated
    );

    let activated_snapshot = fixture.ledger.snapshot().unwrap();
    let harness = RecoveryHarness::new(&fixture);
    harness.clock.set(NOW as i64 + 3, 3_020);
    let mut recovery = harness.create();
    recovery
        .begin(
            begin_request(b"recovery-after-activate", 60_000),
            &migration,
            &histories(),
        )
        .unwrap();
    recovery
        .select_committed_prefix(
            b"recovery-after-activate",
            &[activated_snapshot],
            &lease_policy(),
            &membership13,
        )
        .unwrap();

    let membership14 = fixture.authority.membership(14);
    let recovery_fence_body = fixture.authority.body(
        14,
        3,
        OperationClass::Fence,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let recovery_fence_certificate = fixture.authority.certificate(recovery_fence_body);
    let time = harness.clock.time();
    let fenced = recovery
        .fence_ambiguous(
            b"recovery-after-activate",
            b"recovery-activation-fence",
            &recovery_fence_certificate,
            &membership14,
            &mut fixture.ledger,
            &mut fixture.fencing,
            application(time, &fixture.authority.target_activation, &[]),
        )
        .unwrap();
    assert_eq!(fenced.phase, RecoveryPhase::Fenced);
    assert_eq!(
        fixture.fencing.floor(LINEAGE).unwrap().committed_log_index,
        14
    );
    assert!(fixture.ledger.lease(LINEAGE).unwrap().revoked);
    marker("post_activation_ambiguity_refence", "recovered");
}

#[test]
fn recovery_restored_prefix_activates_and_commits_without_an_explicit_fencing_floor() {
    let fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut store = harness.create();
    begin_and_plan(
        &mut store,
        &migration,
        &fixture,
        b"recovery-restored-prefix",
    );
    store
        .require_operator(b"recovery-restored-prefix", b"process-restart")
        .unwrap();

    let membership11 = fixture.authority.membership(11);
    let snapshot = fixture.ledger.snapshot().unwrap();
    let mut restored_ledger =
        AuthorityLedger::restore(lease_policy(), &snapshot, &membership11, NOW as i64).unwrap();
    assert!(restored_ledger.lease(LINEAGE).unwrap().revoked);
    assert!(fixture.fencing.floor(LINEAGE).is_none());

    harness.clock.set(NOW as i64 + 3, 3_020);
    let time = harness.clock.time();
    let membership12 = fixture.authority.membership(12);
    let activate_body = fixture.authority.body(
        12,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let activate_proof = activation_signature(&activate_body, &fixture.authority.target_activation);
    let activate_certificate = fixture.authority.certificate(activate_body);
    store
        .restore_quorum_owner(
            b"recovery-restored-prefix",
            &activate_certificate,
            &membership12,
            &mut restored_ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &activate_proof),
        )
        .unwrap();

    let membership13 = fixture.authority.membership(13);
    let commit_body = fixture.authority.body(
        13,
        2,
        OperationClass::OwnerCommit,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let commit_proof = activation_signature(&commit_body, &fixture.authority.target_activation);
    let commit_certificate = fixture.authority.certificate(commit_body);
    let committed = store
        .commit_owner(
            b"recovery-restored-prefix",
            &commit_certificate,
            &membership13,
            &mut restored_ledger,
            &fixture.fencing,
            application(time, &fixture.authority.target_activation, &commit_proof),
        )
        .unwrap();
    assert_eq!(committed.phase, RecoveryPhase::Committed);
}

#[cfg(unix)]
#[test]
fn recovery_open_rejects_a_symlinked_state_file() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    let harness = RecoveryHarness::new(&fixture);
    let store = harness.create();
    drop(store);
    let state = harness.root.join("recovery-state.json");
    let target = harness.root.join("attacker-state.json");
    fs::rename(&state, &target).unwrap();
    symlink(&target, &state).unwrap();
    assert!(matches!(
        RecoveryStore::open(
            &harness.root,
            harness.policy.clone(),
            harness.authority.clone(),
            harness.clock.clone(),
        ),
        Err(RecoveryError::UnsafeStatePath)
    ));
    marker("symlinked_recovery_state", "rejected");
}

#[cfg(unix)]
#[test]
fn recovery_open_rejects_a_symlinked_journal_file() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    let harness = RecoveryHarness::new(&fixture);
    let store = harness.create();
    drop(store);
    let lock = harness.root.join(".recovery-state.lock");
    let target = harness.root.join("attacker-journal.json");
    fs::rename(&lock, &target).unwrap();
    symlink(&target, &lock).unwrap();
    assert!(matches!(
        RecoveryStore::open(
            &harness.root,
            harness.policy.clone(),
            harness.authority.clone(),
            harness.clock.clone(),
        ),
        Err(RecoveryError::UnsafeStatePath)
    ));
    marker("symlinked_recovery_journal", "rejected");
}

#[test]
fn recovery_create_recovers_after_initial_checkpoint_failure() {
    let fixture = Fixture::new();
    let harness = RecoveryHarness::new(&fixture);
    harness.authority.fail_generation(0);
    assert!(matches!(
        RecoveryStore::create(
            &harness.root,
            harness.policy.clone(),
            harness.authority.clone(),
            harness.clock.clone(),
        ),
        Err(RecoveryError::DurabilityFailure)
    ));
    let store = harness.create();
    assert_eq!(store.checkpoint().generation, 0);
    marker("initial_checkpoint_failure", "recovered");
}

#[test]
fn recovery_rejects_oversized_state_and_journal_before_reading_them() {
    let fixture = Fixture::new();
    let harness = RecoveryHarness::new(&fixture);
    let store = harness.create();
    drop(store);
    fs::write(
        harness.root.join("recovery-state.json"),
        vec![b'x'; harness.policy.max_state_bytes + 1],
    )
    .unwrap();
    assert!(matches!(
        RecoveryStore::open(
            &harness.root,
            harness.policy.clone(),
            harness.authority.clone(),
            harness.clock.clone(),
        ),
        Err(RecoveryError::ResourceExhausted)
    ));
    marker("oversized_recovery_state", "rejected");

    let other_fixture = Fixture::new();
    let other_harness = RecoveryHarness::new(&other_fixture);
    let store = other_harness.create();
    drop(store);
    fs::write(
        other_harness.root.join(".recovery-state.lock"),
        vec![b'x'; 4097],
    )
    .unwrap();
    assert!(matches!(
        RecoveryStore::open(
            &other_harness.root,
            other_harness.policy.clone(),
            other_harness.authority.clone(),
            other_harness.clock.clone(),
        ),
        Err(RecoveryError::ResourceExhausted)
    ));
    marker("oversized_recovery_journal", "rejected");
}

#[test]
fn recovery_is_bounded_and_times_out_fail_closed() {
    let fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let policy = RecoveryPolicy::with_bounds(DOMAIN, 1, 10, 2, 128, 1024 * 1024, 100).unwrap();
    let harness = RecoveryHarness::with_policy(&fixture, policy);
    let mut store = harness.create();
    store
        .begin(begin_request(b"bounded-a", 100), &migration, &histories())
        .unwrap();
    assert_eq!(
        store.begin(begin_request(b"bounded-b", 100), &migration, &histories()),
        Err(RecoveryError::ResourceExhausted)
    );
    marker("record_capacity", "rejected");

    harness.clock.set(NOW as i64, 125);
    assert_eq!(
        store.require_operator(b"bounded-a", b"late decision"),
        Err(RecoveryError::TimedOut)
    );
    marker("recovery_timeout", "rejected");

    let other_fixture = Fixture::new();
    let mut other_migration = other_fixture.migration_store();
    other_fixture.prepare(&mut other_migration);
    let other_harness = RecoveryHarness::new(&other_fixture);
    let mut other_store = other_harness.create();
    let mut wrong_domain = begin_request(b"wrong-domain", 100);
    wrong_domain.trust_domain = "other.example".to_owned();
    assert_eq!(
        other_store.begin(wrong_domain, &other_migration, &histories()),
        Err(RecoveryError::WrongTrustDomain)
    );
    marker("wrong_trust_domain_begin", "rejected");
}

#[test]
fn recovery_side_effect_crossing_the_bounded_recovery_deadline_requires_operator() {
    let fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let policy = RecoveryPolicy::with_bounds(DOMAIN, 2, 10, 2, 128, 1024 * 1024, 100).unwrap();
    let harness = RecoveryHarness::with_policy(&fixture, policy);
    let membership = fixture.authority.membership(11);
    let time = harness.clock.time();
    migration
        .quiesce(
            b"migration-1",
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &membership, time),
        )
        .unwrap();
    let mut store = harness.create();
    store
        .begin(
            begin_request(b"crossed-deadline", 100),
            &migration,
            &histories(),
        )
        .unwrap();
    store
        .select_committed_prefix(
            b"crossed-deadline",
            &[fixture.ledger.snapshot().unwrap()],
            &lease_policy(),
            &fixture.authority.membership(11),
        )
        .unwrap();
    assert_eq!(
        store.discard_incomplete_target(
            b"crossed-deadline",
            &ExpiringTargetAuthority(harness.clock.clone()),
        ),
        Err(RecoveryError::TimedOut)
    );
    assert_eq!(
        store.record(b"crossed-deadline").unwrap().phase,
        RecoveryPhase::OperatorRequired
    );
    marker("side_effect_crossed_recovery_deadline", "fail_closed");
}

#[test]
fn recovery_requires_one_committed_prefix_and_cleans_incomplete_target_before_rollback() {
    let fixture = Fixture::new();
    let mut migration = fixture.migration_store();
    fixture.prepare(&mut migration);
    let harness = RecoveryHarness::new(&fixture);
    let mut store = harness.create();

    store
        .begin(
            begin_request(b"zero-prefix", 60_000),
            &migration,
            &histories(),
        )
        .unwrap();
    let zero = store
        .select_committed_prefix(
            b"zero-prefix",
            &[b"not-an-authority-snapshot".to_vec()],
            &lease_policy(),
            &fixture.authority.membership(11),
        )
        .unwrap();
    assert_eq!(zero.phase, RecoveryPhase::OperatorRequired);
    marker("zero_valid_committed_prefix", "fail_closed");

    store
        .begin(
            begin_request(b"divergent-prefix", 60_000),
            &migration,
            &histories(),
        )
        .unwrap();
    let source_snapshot = fixture.ledger.snapshot().unwrap();
    let membership11 = fixture.authority.membership(11);
    let mut target_ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
    let target_grant_body = fixture.authority.body(
        11,
        1,
        OperationClass::LeaseGrant,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW,
    );
    let target_proof =
        activation_signature(&target_grant_body, &fixture.authority.target_activation);
    target_ledger
        .apply(
            &TEST_LEASE_STORE_ACCESS,
            &fixture.authority.certificate(target_grant_body),
            &membership11,
            AuthorityApplication {
                now_unix_seconds: NOW as i64,
                now_unix_nanos: 0,
                now_elapsed_millis: 10,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &target_proof,
            },
        )
        .unwrap();
    let divergent = store
        .select_committed_prefix(
            b"divergent-prefix",
            &[source_snapshot.clone(), target_ledger.snapshot().unwrap()],
            &lease_policy(),
            &membership11,
        )
        .unwrap();
    assert_eq!(divergent.phase, RecoveryPhase::OperatorRequired);
    assert_eq!(
        store.select_committed_prefix(
            b"divergent-prefix",
            std::slice::from_ref(&source_snapshot),
            &lease_policy(),
            &membership11,
        ),
        Err(RecoveryError::InvalidTransition)
    );
    marker("operator_required_terminal", "fail_closed");
    marker("divergent_committed_prefix", "fail_closed");

    let membership11 = fixture.authority.membership(11);
    let time = harness.clock.time();
    migration
        .quiesce(
            b"migration-1",
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &membership11, time),
        )
        .unwrap();
    begin_and_plan(&mut store, &migration, &fixture, b"cleanup-before-rollback");
    assert!(
        store
            .record(b"cleanup-before-rollback")
            .unwrap()
            .target_cleanup_required
    );
    assert_eq!(
        store.rollback_pre_fence(
            b"cleanup-before-rollback",
            &mut migration,
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &membership11, time),
        ),
        Err(RecoveryError::InvalidTransition)
    );
    marker("rollback_before_target_cleanup", "rejected");

    let target = TargetAuthority::default();
    let discarded = store
        .discard_incomplete_target(b"cleanup-before-rollback", &target)
        .unwrap();
    assert_eq!(discarded.phase, RecoveryPhase::TargetDiscarded);
    assert!(discarded.target_cleanup_receipt_sha256.is_some());
    assert_eq!(
        target.0.lock().unwrap().as_slice(),
        &[b"cleanup-before-rollback".to_vec()]
    );
    let restored = store
        .rollback_pre_fence(
            b"cleanup-before-rollback",
            &mut migration,
            &QuiescenceAuthority,
            &fixture.fencing,
            active_check(&fixture, &membership11, time),
        )
        .unwrap();
    assert_eq!(restored.phase, RecoveryPhase::Restored);
    assert_eq!(restored.owner_node_id.as_deref(), Some(SOURCE_NODE));
}
