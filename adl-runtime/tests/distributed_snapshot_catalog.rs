// PVF: lane=exact-child-tests; proof=authenticated content-bound snapshot catalog and transfer
// manifest positive and fail-closed behavior; deterministic=true; resource_profile=medium;
// release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
#[allow(dead_code)]
#[path = "../src/distributed/fencing.rs"]
mod fencing;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;
#[path = "../src/distributed/snapshot_catalog.rs"]
mod snapshot_catalog;

use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

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
use sha2::{Digest, Sha256};
use snapshot_catalog::{
    SignedSnapshotCatalogEntry, SignedTransferManifest, SnapshotCatalogEntryBody,
    SnapshotCatalogPolicy, SnapshotCatalogVerifier, SnapshotDescriptor, SnapshotError,
    TransferManifestBody, SNAPSHOT_CATALOG_SCHEMA,
};

const DOMAIN: &str = "polis.example";
const LINEAGE: &[u8] = b"lineage-a";
const NODE: &[u8] = b"node-a";
const GUARDIAN: &[u8] = b"guardian-a";
const TARGET: &[u8] = b"node-b";
const NOW: u64 = 20_000;

fn marker(case: &str, result: &str) {
    println!(
        "ADL_ISSUE_5874_NEGATIVE_CASE_V1 {}",
        serde_json::json!({"case": case, "result": result})
    );
}

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

#[derive(Debug, Default)]
struct CheckpointAuthority(Mutex<Option<FencingCheckpoint>>);

impl FencingCheckpointAuthority for CheckpointAuthority {
    fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<FencingCheckpoint>,
        next: FencingCheckpoint,
    ) -> Result<(), FencingError> {
        let mut current = self.0.lock().unwrap();
        if *current != expected
            || current.is_some_and(|checkpoint| next.generation <= checkpoint.generation)
        {
            return Err(FencingError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

struct LeaseFixture {
    ids: Vec<Vec<u8>>,
    keys: Vec<SigningKey>,
    membership: AuthorityMembership,
    activation: SigningKey,
}

impl LeaseFixture {
    fn new(index: u64) -> Self {
        let ids = (0..3)
            .map(|position| format!("voter-{position}").into_bytes())
            .collect::<Vec<_>>();
        let keys = (1..=3).map(key).collect::<Vec<_>>();
        let voters = ids
            .iter()
            .zip(&keys)
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
        let config = ids.iter().cloned().collect::<BTreeSet<_>>();
        Self {
            ids,
            keys,
            membership: AuthorityMembership::new(
                DOMAIN.as_bytes().to_vec(),
                7,
                index,
                vec![config],
                voters,
            )
            .unwrap(),
            activation: key(90),
        }
    }

    fn body(&self, index: u64) -> AuthorityCertificateBodyV1 {
        AuthorityCertificateBodyV1 {
            schema_version: AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
            trust_domain_id: DOMAIN.as_bytes().to_vec(),
            lineage_id: LINEAGE.to_vec(),
            voter_set_generation: 7,
            raft_term: 3,
            committed_log_index: index,
            epoch: 1,
            holder_node_id: NODE.to_vec(),
            holder_guardian_id: GUARDIAN.to_vec(),
            activation_key_sha256: Sha256::digest(self.activation.verifying_key().to_bytes())
                .to_vec(),
            operation_class: OperationClass::LeaseGrant as u32,
            issued_unix_seconds: NOW as i64,
            issued_nanos: 0,
            lease_duration_millis: 2_000,
            policy_sha256: lease_policy().sha256().unwrap().to_vec(),
            signing_algorithm: SIGNING_ALGORITHM_ED25519,
        }
    }

    fn grant(&self, index: u64) -> (LeaseState, Vec<u8>) {
        let body = self.body(index);
        let proof = activation_signature(&body, &self.activation);
        let mut endorsements = [0, 1]
            .into_iter()
            .map(|position| endorse(&body, self.ids[position].clone(), 7, &self.keys[position]))
            .collect::<Vec<_>>();
        endorsements.sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
        let certificate = encode_certificate(&AuthorityCertificateV1 {
            body: Some(body),
            endorsements,
        })
        .unwrap();
        let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
        ledger
            .apply(
                &TEST_LEASE_STORE_ACCESS,
                &certificate,
                &self.membership,
                AuthorityApplication {
                    now_unix_seconds: NOW as i64,
                    now_unix_nanos: 0,
                    now_elapsed_millis: 10,
                    clock_uncertainty_millis: 5,
                    activation_public_key: self.activation.verifying_key().to_bytes(),
                    activation_proof: &proof,
                },
            )
            .unwrap();
        (ledger.lease(LINEAGE).unwrap().clone(), proof.to_vec())
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

struct Fixture {
    _directory: tempfile::TempDir,
    replay_path: PathBuf,
    certificate_store: Arc<DistributedCertificateStore>,
    fencing_store: FencingStore,
    lease_fixture: LeaseFixture,
    lease: LeaseState,
    activation_proof: Vec<u8>,
    signer: SigningKey,
    certificate: AuthorityCertificate,
    policy: SnapshotCatalogPolicy,
    chunks: Vec<Vec<u8>>,
}

impl Fixture {
    fn new(purpose: CertificatePurpose) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root_path = directory.path().canonicalize().unwrap();
        let issuer = key(40);
        let signer = key(41);
        let certificate_store = Arc::new(
            DistributedCertificateStore::open(
                &TEST_CERTIFICATE_STORE_ACCESS,
                root_path.join("certificates.redb"),
                CertificatePolicy::new(DOMAIN, [issuer.verifying_key()])
                    .unwrap()
                    .with_bounds(3600, 60, 60, 64, 64)
                    .unwrap(),
            )
            .unwrap(),
        );
        let certificate = AuthorityCertificate::issue(
            CertificateBody::new(
                DOMAIN,
                String::from_utf8(NODE.to_vec()).unwrap(),
                purpose,
                1,
                CertificateValidity {
                    issued_at_unix_secs: NOW - 100,
                    expires_at_unix_secs: NOW + 600,
                },
                signer.verifying_key(),
                &issuer.verifying_key(),
            ),
            &issuer,
        )
        .unwrap();
        certificate_store
            .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW - 10)
            .unwrap();
        let fence_root = root_path.join("fencing");
        fs::create_dir(&fence_root).unwrap();
        let fencing_store = FencingStore::create(
            &TEST_FENCING_STORE_ACCESS,
            &fence_root,
            fencing_policy(),
            Arc::new(CheckpointAuthority::default()),
        )
        .unwrap();
        let lease_fixture = LeaseFixture::new(11);
        let (lease, activation_proof) = lease_fixture.grant(11);
        Self {
            _directory: directory,
            replay_path: root_path.join("snapshot-replay.redb"),
            certificate_store,
            fencing_store,
            lease_fixture,
            lease,
            activation_proof,
            signer,
            certificate,
            policy: snapshot_policy(),
            chunks: vec![
                b"private-state-part-a".to_vec(),
                b"private-state-part-b".to_vec(),
            ],
        }
    }

    fn verifier(&self) -> SnapshotCatalogVerifier {
        SnapshotCatalogVerifier::open_for_test(
            self.certificate_store.clone(),
            self.policy.clone(),
            &self.replay_path,
        )
        .unwrap()
    }

    fn active_check(&self) -> ActiveLeaseCheck<'_> {
        ActiveLeaseCheck {
            membership: Some(&self.lease_fixture.membership),
            lease: &self.lease,
            applied_log_index: 11,
            now_unix_seconds: NOW as i64,
            now_unix_millis: NOW * 1000,
            now_elapsed_millis: 20,
            activation_proof: &self.activation_proof,
        }
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
            source_owner_id: NODE.to_vec(),
            source_guardian_id: GUARDIAN.to_vec(),
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
        let body = SnapshotCatalogEntryBody::new(
            String::from_utf8(NODE.to_vec()).unwrap(),
            1,
            1,
            self.snapshot(),
            &self.policy,
        )
        .unwrap();
        SignedSnapshotCatalogEntry::issue(
            body,
            self.certificate.clone(),
            &self.signer,
            &self.policy,
        )
        .unwrap()
    }

    fn manifest(
        &self,
        transfer_id: &[u8],
        target: &[u8],
        catalog: &SignedSnapshotCatalogEntry,
    ) -> SignedTransferManifest {
        let body = TransferManifestBody::new(transfer_id, target, catalog, &self.policy).unwrap();
        SignedTransferManifest::issue(body, self.certificate.clone(), &self.signer, &self.policy)
            .unwrap()
    }

    fn verify_transfer(
        &self,
        verifier: &SnapshotCatalogVerifier,
        manifest: &SignedTransferManifest,
        catalog: &SignedSnapshotCatalogEntry,
        chunks: &[Vec<u8>],
        target: &[u8],
    ) -> Result<snapshot_catalog::VerifiedSnapshotTransfer, SnapshotError> {
        verifier.verify_transfer(
            manifest,
            catalog,
            chunks,
            target,
            &self.fencing_store,
            self.active_check(),
        )
    }
}

fn assert_case<T: std::fmt::Debug>(
    case: &str,
    actual: Result<T, SnapshotError>,
    expected: SnapshotError,
) {
    assert_eq!(actual.unwrap_err(), expected);
    marker(case, "rejected");
}

#[test]
fn authenticated_catalog_and_manifest_accept_complete_content() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let verified_catalog = verifier
        .verify_catalog(&catalog, &fixture.fencing_store, fixture.active_check())
        .unwrap();
    let manifest = fixture.manifest(b"transfer-1", TARGET, &catalog);
    let verified = verifier
        .decode_transfer_and_verify(
            &manifest.encode(&fixture.policy).unwrap(),
            &catalog.encode(&fixture.policy).unwrap(),
            &fixture.chunks,
            TARGET,
            &fixture.fencing_store,
            fixture.active_check(),
        )
        .unwrap();
    assert_eq!(verified.snapshot, fixture.snapshot());
    assert_eq!(
        verified.catalog_entry_sha256,
        verified_catalog.catalog_entry_sha256
    );
    assert_eq!(verified.redacted(), verified_catalog.redacted());
}

#[test]
fn decode_requires_canonical_exact_schema_payloads() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let bytes = catalog.encode(&fixture.policy).unwrap();
    let mut noncanonical = bytes.clone();
    // Valid unknown protobuf field 15; canonical re-encoding must reject it.
    noncanonical.extend_from_slice(&[0x78, 0x01]);
    assert_case(
        "noncanonical_catalog",
        verifier.decode_catalog_and_verify(
            &noncanonical,
            &fixture.fencing_store,
            fixture.active_check(),
        ),
        SnapshotError::NonCanonical,
    );
    let mut wrong_schema = catalog;
    wrong_schema.body.schema = "adl.distributed.snapshot_catalog_entry.v2".to_owned();
    assert_case(
        "wrong_schema",
        verifier.verify_catalog(
            &wrong_schema,
            &fixture.fencing_store,
            fixture.active_check(),
        ),
        SnapshotError::InvalidSnapshot,
    );
}

#[test]
fn certificate_purpose_is_snapshot_specific() {
    let fixture = Fixture::new(CertificatePurpose::AdvertisementSigning);
    let body = SnapshotCatalogEntryBody {
        schema: SNAPSHOT_CATALOG_SCHEMA.to_owned(),
        signer_id: String::from_utf8(NODE.to_vec()).unwrap(),
        certificate_generation: 1,
        sequence: 1,
        snapshot: fixture.snapshot(),
    };
    assert_case(
        "wrong_certificate_purpose",
        SignedSnapshotCatalogEntry::issue(
            body,
            fixture.certificate.clone(),
            &fixture.signer,
            &fixture.policy,
        ),
        SnapshotError::CertificateAuthorization,
    );
}

#[test]
fn stale_or_mismatched_authority_is_rejected() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let mut catalog = fixture.catalog();
    catalog.body.snapshot.source_epoch = 2;
    assert_case(
        "stale_source_epoch",
        verifier.verify_catalog(&catalog, &fixture.fencing_store, fixture.active_check()),
        SnapshotError::AuthorityMismatch,
    );
    let catalog = fixture.catalog();
    let mut check = fixture.active_check();
    check.applied_log_index = 12;
    assert_case(
        "stale_applied_authority",
        verifier.verify_catalog(&catalog, &fixture.fencing_store, check),
        SnapshotError::AuthorityMismatch,
    );
}

#[test]
fn expired_catalog_is_rejected() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let mut check = fixture.active_check();
    check.now_unix_seconds = (NOW + 60) as i64;
    check.now_unix_millis = (NOW + 60) * 1000;
    assert_case(
        "expired_catalog",
        verifier.verify_catalog(&catalog, &fixture.fencing_store, check),
        SnapshotError::Expired,
    );
}

#[test]
fn target_and_catalog_substitution_are_rejected() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let manifest = fixture.manifest(b"transfer-2", TARGET, &catalog);
    assert_case(
        "wrong_target",
        fixture.verify_transfer(&verifier, &manifest, &catalog, &fixture.chunks, b"node-c"),
        SnapshotError::WrongTarget,
    );
    let mut substituted = catalog.clone();
    substituted.body.sequence = 2;
    assert_case(
        "catalog_substitution",
        fixture.verify_transfer(&verifier, &manifest, &substituted, &fixture.chunks, TARGET),
        SnapshotError::CatalogMismatch,
    );
}

#[test]
fn incomplete_and_corrupt_chunks_fail_before_acceptance() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let manifest = fixture.manifest(b"transfer-3", TARGET, &catalog);
    assert_case(
        "incomplete_transfer",
        fixture.verify_transfer(&verifier, &manifest, &catalog, &fixture.chunks[..1], TARGET),
        SnapshotError::IncompleteTransfer,
    );
    let mut corrupt = fixture.chunks.clone();
    corrupt[1][0] ^= 1;
    assert_case(
        "corrupt_chunk",
        fixture.verify_transfer(&verifier, &manifest, &catalog, &corrupt, TARGET),
        SnapshotError::CorruptChunk,
    );
    fixture
        .verify_transfer(&verifier, &manifest, &catalog, &fixture.chunks, TARGET)
        .unwrap();
}

#[test]
fn whole_content_length_and_digest_are_enforced() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let mut catalog = fixture.catalog();
    catalog.body.snapshot.byte_length += 1;
    let catalog = SignedSnapshotCatalogEntry::issue(
        catalog.body,
        fixture.certificate.clone(),
        &fixture.signer,
        &fixture.policy,
    )
    .unwrap();
    let manifest = fixture.manifest(b"transfer-length", TARGET, &catalog);
    assert_case(
        "content_length_mismatch",
        fixture.verify_transfer(&verifier, &manifest, &catalog, &fixture.chunks, TARGET),
        SnapshotError::ContentDigestMismatch,
    );
}

#[test]
fn chunk_larger_than_signed_total_is_rejected_before_digest_work() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let manifest = fixture.manifest(b"transfer-oversized-chunk", TARGET, &catalog);
    let oversized = vec![
        vec![0_u8; fixture.snapshot().byte_length as usize + 1],
        Vec::new(),
    ];
    assert_case(
        "chunk_exceeds_signed_total",
        fixture.verify_transfer(&verifier, &manifest, &catalog, &oversized, TARGET),
        SnapshotError::ContentDigestMismatch,
    );
}

#[test]
fn accepted_transfer_replay_survives_restart() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let catalog = fixture.catalog();
    let manifest = fixture.manifest(b"transfer-restart", TARGET, &catalog);
    {
        let verifier = fixture.verifier();
        fixture
            .verify_transfer(&verifier, &manifest, &catalog, &fixture.chunks, TARGET)
            .unwrap();
    }
    let reopened = fixture.verifier();
    assert_case(
        "replay_after_restart",
        fixture.verify_transfer(&reopened, &manifest, &catalog, &fixture.chunks, TARGET),
        SnapshotError::Replay,
    );
}

#[test]
fn reused_transfer_id_with_different_manifest_is_rejected() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let first = fixture.manifest(b"transfer-conflict", TARGET, &catalog);
    fixture
        .verify_transfer(&verifier, &first, &catalog, &fixture.chunks, TARGET)
        .unwrap();
    let second = fixture.manifest(b"transfer-conflict", b"node-c", &catalog);
    assert_case(
        "replay_mismatch",
        fixture.verify_transfer(&verifier, &second, &catalog, &fixture.chunks, b"node-c"),
        SnapshotError::ReplayMismatch,
    );
}

#[test]
fn signature_tampering_is_rejected() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let mut catalog = fixture.catalog();
    catalog.signature[0] ^= 1;
    assert_case(
        "tampered_signature",
        verifier.verify_catalog(&catalog, &fixture.fencing_store, fixture.active_check()),
        SnapshotError::WrongSigner,
    );
}

#[test]
fn redacted_projection_contains_no_private_or_topology_material() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let verifier = fixture.verifier();
    let catalog = fixture.catalog();
    let verified = verifier
        .verify_catalog(&catalog, &fixture.fencing_store, fixture.active_check())
        .unwrap();
    let json = serde_json::to_string(&verified.redacted()).unwrap();
    for forbidden in [
        "private-state-part",
        "lineage-a",
        "node-a",
        "guardian-a",
        "context-7",
        "content_sha256",
        "chunk_sha256",
    ] {
        assert!(!json.contains(forbidden));
    }
}

#[test]
fn oversized_and_unsafe_replay_state_paths_fail_closed() {
    let fixture = Fixture::new(CertificatePurpose::SnapshotSigning);
    let policy = SnapshotCatalogPolicy::new(DOMAIN)
        .unwrap()
        .with_bounds(1, 8, 120, 2, 1024, 1)
        .unwrap();
    assert_case(
        "oversized_snapshot",
        SnapshotCatalogEntryBody::new("guardian-a", 1, 1, fixture.snapshot(), &policy),
        SnapshotError::InvalidSnapshot,
    );
    assert_eq!(
        SnapshotCatalogVerifier::open_for_test(
            fixture.certificate_store.clone(),
            fixture.policy.clone(),
            "relative.redb",
        )
        .err(),
        Some(SnapshotError::InvalidPolicy)
    );
    marker("relative_replay_path", "rejected");
}
