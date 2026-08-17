// PVF: lane=exact-child-tests; proof=durable authority-bound migration state machine;
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
#[path = "../src/distributed/resource_weather.rs"]
mod resource_weather;
#[allow(dead_code)]
#[path = "../src/distributed/snapshot_catalog.rs"]
mod snapshot_catalog;

use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
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
        "ADL_ISSUE_5875_NEGATIVE_CASE_V1 {}",
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

#[test]
fn migration_orders_authority_and_survives_restart() {
    let mut fixture = Fixture::new();
    let mut store = fixture.migration_store();
    run_to_validated(&mut fixture, &mut store);
    assert_eq!(
        store.record(b"migration-1").unwrap().phase,
        MigrationPhase::Validated
    );
    drop(store);
    let mut store = MigrationStore::open(
        &fixture.migration_root,
        MigrationPolicy::new(DOMAIN).unwrap(),
        fixture.migration_authority.clone(),
        fixture.migration_clock.clone(),
    )
    .unwrap();

    let membership = fixture.authority.membership(12);
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
    store
        .fence(
            b"migration-1",
            b"fence-migration-1",
            &fence_certificate,
            &membership,
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
    let fenced = store.record(b"migration-1").unwrap();
    assert!(!fenced.source_authoritative && !fenced.target_authoritative);
    let generation = store.checkpoint().generation;
    store
        .fence(
            b"migration-1",
            b"fence-migration-1",
            &fence_certificate,
            &membership,
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
    assert_eq!(store.checkpoint().generation, generation);
    assert_eq!(
        store.abort_before_fence(
            b"migration-1",
            &QuiescenceAuthority,
            &fixture.fencing,
            ActiveLeaseCheck {
                membership: Some(&membership),
                lease: fixture.ledger.lease(LINEAGE).unwrap(),
                applied_log_index: 12,
                now_unix_seconds: NOW as i64,
                now_unix_millis: NOW * 1_000,
                now_elapsed_millis: 30,
                activation_proof: &fixture.source_proof,
            },
        ),
        Err(MigrationError::PostFenceAbort)
    );
    marker("post_fence_abort", "rejected");

    let membership = fixture.authority.membership(13);
    let activate_body = fixture.authority.body(
        13,
        2,
        OperationClass::Activate,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let activation_proof =
        activation_signature(&activate_body, &fixture.authority.target_activation);
    let wrong_activation_proof =
        activation_signature(&activate_body, &fixture.authority.source_activation);
    let activation_certificate = fixture.authority.certificate(activate_body);
    assert_eq!(
        store.activate(
            b"migration-1",
            &activation_certificate,
            &membership,
            &mut fixture.ledger,
            &fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 3,
                now_unix_nanos: 0,
                now_elapsed_millis: 3_020,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .source_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &wrong_activation_proof,
            },
        ),
        Err(MigrationError::ActivationRejected)
    );
    marker("wrong_activation_key", "rejected");
    store
        .activate(
            b"migration-1",
            &activation_certificate,
            &membership,
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
                activation_proof: &activation_proof,
            },
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    store
        .activate(
            b"migration-1",
            &activation_certificate,
            &membership,
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
                activation_proof: &activation_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
    let membership = fixture.authority.membership(14);
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
    let committed = store
        .commit_owner(
            b"migration-1",
            &commit_certificate,
            &membership,
            &mut fixture.ledger,
            &fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 3,
                now_unix_nanos: 0,
                now_elapsed_millis: 3_030,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &commit_proof,
            },
        )
        .unwrap();
    let generation = store.checkpoint().generation;
    store
        .commit_owner(
            b"migration-1",
            &commit_certificate,
            &membership,
            &mut fixture.ledger,
            &fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 3,
                now_unix_nanos: 0,
                now_elapsed_millis: 3_030,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &commit_proof,
            },
        )
        .unwrap();
    assert_eq!(store.checkpoint().generation, generation);
    assert_eq!(committed.phase, MigrationPhase::Committed);
    assert!(!committed.source_authoritative && committed.target_authoritative);
    assert_eq!(committed.history.len(), 8);

    let newer_membership = fixture.authority.membership(15);
    let newer_commit_body = fixture.authority.body(
        15,
        3,
        OperationClass::Fence,
        TARGET_NODE,
        TARGET_GUARDIAN,
        &fixture.authority.target_activation,
        NOW + 3,
    );
    let newer_commit_certificate = fixture.authority.certificate(newer_commit_body);
    fixture
        .ledger
        .apply(
            &TEST_LEASE_STORE_ACCESS,
            &newer_commit_certificate,
            &newer_membership,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 4,
                now_unix_nanos: 0,
                now_elapsed_millis: 4_030,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &[],
            },
        )
        .unwrap();
    assert_eq!(
        store.commit_owner(
            b"migration-1",
            &commit_certificate,
            &membership,
            &mut fixture.ledger,
            &fixture.fencing,
            AuthorityApplication {
                now_unix_seconds: NOW as i64 + 4,
                now_unix_nanos: 0,
                now_elapsed_millis: 4_040,
                clock_uncertainty_millis: 5,
                activation_public_key: fixture
                    .authority
                    .target_activation
                    .verifying_key()
                    .to_bytes(),
                activation_proof: &commit_proof,
            },
        ),
        Err(MigrationError::CommitRejected)
    );
    marker("stale_authority_retry", "rejected");
}

#[test]
fn timeout_and_interrupted_commit_recovery_fail_closed() {
    let fixture = Fixture::new();
    let mut store = fixture.migration_store();
    fixture.prepare(&mut store);
    let checkpoint = store.checkpoint();
    let state_path = fixture.migration_root.join("migration-state.json");
    let backup_path = fixture.migration_root.join(".migration-state.backup");
    let lock_path = fixture.migration_root.join(".migration-state.lock");
    let state = fs::read(&state_path).unwrap();
    fs::write(&backup_path, &state).unwrap();
    fs::write(&state_path, b"interrupted-new-state").unwrap();
    fs::write(
        &lock_path,
        serde_json::to_vec(&serde_json::json!({
            "expected": checkpoint,
            "next": {"generation": checkpoint.generation + 1, "state_sha256": vec![0_u8; 32]}
        }))
        .unwrap(),
    )
    .unwrap();
    drop(store);
    let store = MigrationStore::open(
        &fixture.migration_root,
        MigrationPolicy::new(DOMAIN).unwrap(),
        fixture.migration_authority.clone(),
        fixture.migration_clock.clone(),
    )
    .unwrap();
    assert_eq!(store.checkpoint(), checkpoint);
    assert!(!backup_path.exists());
    drop(store);

    let state = fs::read(&state_path).unwrap();
    fs::write(&backup_path, b"old-state").unwrap();
    fs::write(
        &lock_path,
        serde_json::to_vec(&serde_json::json!({
            "expected": {"generation": checkpoint.generation - 1, "state_sha256": vec![0_u8; 32]},
            "next": checkpoint
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(&state_path, state).unwrap();
    let mut store = MigrationStore::open(
        &fixture.migration_root,
        MigrationPolicy::new(DOMAIN).unwrap(),
        fixture.migration_authority.clone(),
        fixture.migration_clock.clone(),
    )
    .unwrap();
    assert!(!backup_path.exists());
    marker("crash_recovery", "rejected");

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
    *fixture.migration_clock.0.lock().unwrap() = NOW * 1_000 + 60_000;
    assert_eq!(
        store.quiesce(
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
        ),
        Err(MigrationError::TimedOut)
    );
    let aborted = store
        .abort_before_fence(
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
    assert_eq!(aborted.phase, MigrationPhase::Aborted);
    assert!(aborted.source_authoritative && !aborted.target_authoritative);
    marker("timeout_expired", "rejected");
}

#[test]
fn pre_fence_abort_resumes_source_and_post_fence_skip_is_denied() {
    let fixture = Fixture::new();
    let mut store = fixture.migration_store();
    fixture.prepare(&mut store);
    let membership = fixture.authority.membership(11);
    let aborted = store
        .abort_before_fence(
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
    assert_eq!(aborted.phase, MigrationPhase::Aborted);
    assert!(aborted.source_authoritative && !aborted.target_authoritative);
    assert_eq!(
        store.validate_restore(
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
        ),
        Err(MigrationError::InvalidTransition)
    );
    marker("skip_after_abort", "rejected");
}

#[test]
fn mismatched_replay_corrupt_restart_and_bounds_fail_closed() {
    let fixture = Fixture::new();
    let mut store = fixture.migration_store();
    fixture.prepare(&mut store);
    let generation = store.checkpoint().generation;
    let membership = fixture.authority.membership(11);
    assert_eq!(
        store.quiesce(
            b"missing",
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
        ),
        Err(MigrationError::NotFound)
    );
    assert_eq!(store.checkpoint().generation, generation);
    marker("unknown_migration", "rejected");

    let old_state = fs::read(fixture.migration_root.join("migration-state.json")).unwrap();
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
    fs::write(
        fixture.migration_root.join("migration-state.json"),
        old_state,
    )
    .unwrap();
    assert_eq!(
        MigrationStore::open(
            &fixture.migration_root,
            MigrationPolicy::new(DOMAIN).unwrap(),
            fixture.migration_authority.clone(),
            fixture.migration_clock.clone(),
        )
        .unwrap_err(),
        MigrationError::Rollback
    );
    marker("rollback_restart", "rejected");

    fs::write(
        fixture.migration_root.join("migration-state.json"),
        b"corrupt",
    )
    .unwrap();
    assert_eq!(
        MigrationStore::open(
            &fixture.migration_root,
            MigrationPolicy::new(DOMAIN).unwrap(),
            fixture.migration_authority.clone(),
            fixture.migration_clock.clone(),
        )
        .unwrap_err(),
        MigrationError::StateCorrupt
    );
    marker("corrupt_restart", "rejected");
    assert!(MigrationPolicy::with_bounds(DOMAIN, 0, 8, 128, 1024, 1, 1, 1).is_err());
    marker("zero_capacity", "rejected");
    let _ = &fixture.directory;
}
