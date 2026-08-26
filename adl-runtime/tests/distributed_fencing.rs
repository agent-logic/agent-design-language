#[path = "../src/distributed/fencing.rs"]
mod fencing;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;

use std::{
    collections::BTreeSet,
    fs,
    sync::{Arc, Mutex},
};

use ed25519_dalek::SigningKey;
use fencing::{
    ActiveLeaseCheck, FenceCommit, FencingCheckpoint, FencingCheckpointAuthority, FencingError,
    FencingPolicy, FencingStore, TEST_FENCING_STORE_ACCESS,
};
use lease::{
    activation_signature, decode_certificate, encode_certificate, endorse, AuthorityApplication,
    AuthorityCertificateBodyV1, AuthorityCertificateV1, AuthorityLedger, AuthorityMembership,
    ControlCertificatePurpose, LeasePolicy, LeaseState, OperationClass, VoterAuthority,
    AUTHORITY_CERTIFICATE_SCHEMA_VERSION, SIGNING_ALGORITHM_ED25519, TEST_LEASE_STORE_ACCESS,
};
use sha2::{Digest, Sha256};

const TRUST: &[u8] = b"trust-domain";
const LINEAGE: &[u8] = b"lineage-a";
const NODE: &[u8] = b"node-a";
const HOLDER: &[u8] = b"guardian-a";
const NOW: i64 = 1_787_000_100;

fn marker(case: &str, result: &str) {
    println!(
        "ADL_ISSUE_5870_NEGATIVE_CASE_V1 {}",
        serde_json::json!({"case": case, "result": result})
    );
}

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

struct Fixture {
    ids: Vec<Vec<u8>>,
    keys: Vec<SigningKey>,
    membership: AuthorityMembership,
    activation: SigningKey,
}

impl Fixture {
    fn new(committed_log_index: u64) -> Self {
        let ids = (0..3)
            .map(|index| format!("guardian-{index}").into_bytes())
            .collect::<Vec<_>>();
        let keys = (1..=3).map(key).collect::<Vec<_>>();
        let voters = ids
            .iter()
            .zip(&keys)
            .map(|(id, key)| VoterAuthority {
                guardian_id: id.clone(),
                trust_domain_id: TRUST.to_vec(),
                certificate_generation: 7,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: NOW - 1_000,
                not_after_unix_seconds: NOW + 1_000,
                revoked: false,
                control_public_key: key.verifying_key().to_bytes(),
            })
            .collect::<Vec<_>>();
        let config = ids.iter().cloned().collect::<BTreeSet<_>>();
        Self {
            ids,
            keys,
            membership: AuthorityMembership::new(
                TRUST.to_vec(),
                7,
                committed_log_index,
                vec![config],
                voters,
            )
            .unwrap(),
            activation: key(100),
        }
    }

    fn certificate(&self, body: AuthorityCertificateBodyV1) -> Vec<u8> {
        let mut endorsements = [0, 1]
            .into_iter()
            .map(|index| endorse(&body, self.ids[index].clone(), 7, &self.keys[index]))
            .collect::<Vec<_>>();
        endorsements.sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
        encode_certificate(&AuthorityCertificateV1 {
            body: Some(body),
            endorsements,
        })
        .unwrap()
    }

    fn body(
        &self,
        operation: OperationClass,
        index: u64,
        epoch: u64,
    ) -> AuthorityCertificateBodyV1 {
        AuthorityCertificateBodyV1 {
            schema_version: AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
            trust_domain_id: TRUST.to_vec(),
            lineage_id: LINEAGE.to_vec(),
            voter_set_generation: 7,
            raft_term: 3,
            committed_log_index: index,
            epoch,
            holder_node_id: NODE.to_vec(),
            holder_guardian_id: HOLDER.to_vec(),
            activation_key_sha256: Sha256::digest(self.activation.verifying_key().to_bytes())
                .to_vec(),
            operation_class: operation as u32,
            issued_unix_seconds: NOW,
            issued_nanos: 0,
            lease_duration_millis: 2_000,
            policy_sha256: lease_policy().sha256().unwrap().to_vec(),
            signing_algorithm: SIGNING_ALGORITHM_ED25519,
        }
    }

    fn grant(&self, index: u64) -> LeaseState {
        let mut membership = self.membership.clone();
        membership.committed_log_index = index;
        let body = self.body(OperationClass::LeaseGrant, index, 1);
        let proof = activation_signature(&body, &self.activation);
        let certificate = self.certificate(body);
        let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
        ledger
            .apply(
                &TEST_LEASE_STORE_ACCESS,
                &certificate,
                &membership,
                AuthorityApplication {
                    now_unix_seconds: NOW,
                    now_unix_nanos: 0,
                    now_elapsed_millis: 10,
                    clock_uncertainty_millis: 5,
                    activation_public_key: self.activation.verifying_key().to_bytes(),
                    activation_proof: &proof,
                },
            )
            .unwrap();
        ledger.lease(LINEAGE).unwrap().clone()
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

fn state_dir() -> tempfile::TempDir {
    tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap()
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

fn checkpoint_authority() -> Arc<dyn FencingCheckpointAuthority> {
    Arc::new(CheckpointAuthority::default())
}

fn commit<'a>(
    store: &mut FencingStore,
    fixture: &'a Fixture,
    lease: &'a LeaseState,
    request_id: &'a [u8],
    operation: OperationClass,
    index: u64,
    epoch: u64,
) -> Result<fencing::FenceReceipt, FencingError> {
    let mut membership = fixture.membership.clone();
    membership.committed_log_index = index;
    let certificate = fixture.certificate(fixture.body(operation, index, epoch));
    store.commit(
        &TEST_FENCING_STORE_ACCESS,
        FenceCommit {
            request_id,
            certificate_bytes: &certificate,
            membership: Some(&membership),
            current_lease: lease,
            now_unix_seconds: NOW,
        },
    )
}

fn activation_proof(fixture: &Fixture, lease: &LeaseState) -> [u8; 64] {
    let certificate = decode_certificate(&lease.certificate_bytes).unwrap();
    activation_signature(certificate.body.as_ref().unwrap(), &fixture.activation)
}

fn active_lease(
    fixture: &Fixture,
    index: u64,
    epoch: u64,
) -> (LeaseState, AuthorityCertificateBodyV1) {
    let mut body = fixture.body(OperationClass::Activate, index, epoch);
    body.issued_unix_seconds = NOW + 3;
    let certificate_bytes = fixture.certificate(body.clone());
    (
        LeaseState {
            lineage_id: body.lineage_id.clone(),
            holder_node_id: body.holder_node_id.clone(),
            holder_guardian_id: body.holder_guardian_id.clone(),
            activation_public_key: fixture.activation.verifying_key().to_bytes(),
            raft_term: body.raft_term,
            committed_log_index: body.committed_log_index,
            epoch: body.epoch,
            certificate_generation: body.voter_set_generation,
            activated_elapsed_millis: 100,
            deadline_elapsed_millis: 2_100,
            deadline_unix_millis: ((NOW + 3) as u64) * 1_000 + 2_000,
            certificate_bytes,
            revoked: false,
            last_mutation_sequence: 0,
        },
        body,
    )
}

#[test]
fn quorum_fence_revoke_epoch_and_replay_contract() {
    let fixture = Fixture::new(101);
    let lease = fixture.grant(100);
    let directory = state_dir();
    let mut store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        directory.path(),
        fencing_policy(),
        checkpoint_authority(),
    )
    .unwrap();

    let receipt = commit(
        &mut store,
        &fixture,
        &lease,
        b"fence-1",
        OperationClass::Fence,
        101,
        2,
    )
    .unwrap();
    assert_eq!(receipt.epoch, 2);
    marker("fence_without_old_holder_activation_proof", "fenced");

    assert_eq!(
        commit(
            &mut store,
            &fixture,
            &lease,
            b"same",
            OperationClass::Fence,
            102,
            1,
        ),
        Err(FencingError::StaleEpoch)
    );
    marker("fence_same_epoch", "denied");
    assert_eq!(
        commit(
            &mut store,
            &fixture,
            &lease,
            b"gap",
            OperationClass::Fence,
            102,
            3,
        ),
        Err(FencingError::EpochGap)
    );
    marker("fence_epoch_gap", "denied");

    let stale_membership = Fixture::new(102);
    let stale_certificate = fixture.certificate(fixture.body(OperationClass::Fence, 101, 2));
    assert!(store
        .commit(
            &TEST_FENCING_STORE_ACCESS,
            FenceCommit {
                request_id: b"uncommitted",
                certificate_bytes: &stale_certificate,
                membership: Some(&stale_membership.membership),
                current_lease: &lease,
                now_unix_seconds: NOW,
            }
        )
        .is_err());
    marker("fence_uncommitted_next_epoch", "denied");

    let mut old_generation = fixture.membership.clone();
    old_generation.voter_set_generation = 8;
    assert!(store
        .commit(
            &TEST_FENCING_STORE_ACCESS,
            FenceCommit {
                request_id: b"stale-membership",
                certificate_bytes: &stale_certificate,
                membership: Some(&old_generation),
                current_lease: &lease,
                now_unix_seconds: NOW,
            }
        )
        .is_err());
    marker("stale_authority_membership", "denied");

    assert_eq!(
        store.commit(
            &TEST_FENCING_STORE_ACCESS,
            FenceCommit {
                request_id: b"no-membership",
                certificate_bytes: &stale_certificate,
                membership: None,
                current_lease: &lease,
                now_unix_seconds: NOW,
            }
        ),
        Err(FencingError::MembershipRequired)
    );
    marker("no_current_authority_membership", "denied");

    let grant = fixture.certificate(fixture.body(OperationClass::LeaseGrant, 102, 1));
    let mut current = fixture.membership.clone();
    current.committed_log_index = 102;
    assert_eq!(
        store.commit(
            &TEST_FENCING_STORE_ACCESS,
            FenceCommit {
                request_id: b"wrong-operation",
                certificate_bytes: &grant,
                membership: Some(&current),
                current_lease: &lease,
                now_unix_seconds: NOW,
            }
        ),
        Err(FencingError::UnauthorizedOperation)
    );
    marker("unauthorized_operation", "denied");

    let mut forged_lease = lease.clone();
    let forged_body = fixture.body(OperationClass::LeaseGrant, 100, 1);
    forged_lease.certificate_bytes = encode_certificate(&AuthorityCertificateV1 {
        body: Some(forged_body),
        endorsements: Vec::new(),
    })
    .unwrap();
    assert_eq!(
        commit(
            &mut store,
            &fixture,
            &forged_lease,
            b"forged-current-lease",
            OperationClass::Fence,
            102,
            2,
        ),
        Err(FencingError::InvalidCertificate)
    );

    assert_eq!(
        commit(
            &mut store,
            &fixture,
            &lease,
            b"fence-1",
            OperationClass::Fence,
            102,
            2,
        ),
        Err(FencingError::ReplayMismatch)
    );
    marker("replay_receipt_mismatch", "denied");

    let revoke_dir = state_dir();
    let mut revoke_store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        revoke_dir.path(),
        fencing_policy(),
        checkpoint_authority(),
    )
    .unwrap();
    commit(
        &mut revoke_store,
        &fixture,
        &lease,
        b"revoke-1",
        OperationClass::Revoke,
        101,
        1,
    )
    .unwrap();
    let mut post_revoke_membership = fixture.membership.clone();
    post_revoke_membership.committed_log_index = 102;
    let (same_epoch, same_epoch_body) = active_lease(&fixture, 102, 1);
    let same_epoch_proof = activation_signature(&same_epoch_body, &fixture.activation);
    assert_eq!(
        revoke_store.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&post_revoke_membership),
                lease: &same_epoch,
                applied_log_index: 102,
                now_unix_seconds: NOW + 3,
                now_unix_millis: ((NOW + 3) as u64) * 1_000,
                now_elapsed_millis: 21,
                activation_proof: &same_epoch_proof,
            }
        ),
        Err(FencingError::Fenced)
    );
    let (next_epoch, next_epoch_body) = active_lease(&fixture, 102, 2);
    let next_epoch_proof = activation_signature(&next_epoch_body, &fixture.activation);
    revoke_store
        .authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&post_revoke_membership),
                lease: &next_epoch,
                applied_log_index: 102,
                now_unix_seconds: NOW + 3,
                now_unix_millis: ((NOW + 3) as u64) * 1_000,
                now_elapsed_millis: 21,
                activation_proof: &next_epoch_proof,
            },
        )
        .unwrap();
    marker("revoke_without_old_holder_activation_proof", "fenced");
}

#[test]
fn durable_floor_fences_restart_rollback_and_failed_commit() {
    let fixture = Fixture::new(101);
    let lease = fixture.grant(100);
    let directory = state_dir();
    let authority = checkpoint_authority();
    let mut store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        directory.path(),
        fencing_policy(),
        authority.clone(),
    )
    .unwrap();
    let old_state = fs::read(directory.path().join("fencing-state.json")).unwrap();
    commit(
        &mut store,
        &fixture,
        &lease,
        b"fence",
        OperationClass::Fence,
        101,
        2,
    )
    .unwrap();
    let current_state = fs::read(directory.path().join("fencing-state.json")).unwrap();

    let mut active_membership = fixture.membership.clone();
    active_membership.committed_log_index = 100;
    assert_eq!(
        store.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: (NOW as u64) * 1_000,
                now_elapsed_millis: 20,
                activation_proof: &[],
            }
        ),
        Err(FencingError::Fenced)
    );
    marker("fenced_mutation", "denied");

    fs::write(directory.path().join("fencing-state.json"), &old_state).unwrap();
    assert_eq!(
        FencingStore::open(
            &TEST_FENCING_STORE_ACCESS,
            directory.path(),
            fencing_policy(),
            authority.clone()
        )
        .unwrap_err(),
        FencingError::Rollback
    );
    marker("rollback_below_floor", "denied");
    fs::write(directory.path().join("fencing-state.json"), current_state).unwrap();

    let reopened = FencingStore::open(
        &TEST_FENCING_STORE_ACCESS,
        directory.path(),
        fencing_policy(),
        authority.clone(),
    )
    .unwrap();
    assert_eq!(reopened.floor(LINEAGE).unwrap().epoch, 2);
    marker("restart_floor_retained", "fenced");

    let active_dir = state_dir();
    let active_store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        active_dir.path(),
        fencing_policy(),
        checkpoint_authority(),
    )
    .unwrap();
    let proof = activation_proof(&fixture, &lease);
    active_store
        .authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: lease.deadline_unix_millis - 1,
                now_elapsed_millis: 2_009,
                activation_proof: &proof,
            },
        )
        .unwrap();
    assert_eq!(
        active_store.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: lease.deadline_unix_millis - 1,
                now_elapsed_millis: 2_009,
                activation_proof: &[0; 64],
            }
        ),
        Err(FencingError::ActivationPossession)
    );
    assert_eq!(
        active_store.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: lease.deadline_unix_millis,
                now_elapsed_millis: 2_020,
                activation_proof: &proof,
            }
        ),
        Err(FencingError::LeaseExpired)
    );

    let stale_dir = state_dir();
    let stale_authority = checkpoint_authority();
    let mut writer = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        stale_dir.path(),
        fencing_policy(),
        stale_authority.clone(),
    )
    .unwrap();
    let mut stale = FencingStore::open(
        &TEST_FENCING_STORE_ACCESS,
        stale_dir.path(),
        fencing_policy(),
        stale_authority.clone(),
    )
    .unwrap();
    commit(
        &mut writer,
        &fixture,
        &lease,
        b"writer",
        OperationClass::Fence,
        101,
        2,
    )
    .unwrap();
    assert_eq!(
        stale.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: lease.deadline_unix_millis - 1,
                now_elapsed_millis: 2_009,
                activation_proof: &proof,
            }
        ),
        Err(FencingError::Rollback)
    );
    assert_eq!(
        commit(
            &mut stale,
            &fixture,
            &lease,
            b"stale-writer",
            OperationClass::Fence,
            102,
            2,
        ),
        Err(FencingError::Rollback)
    );

    let failed_dir = state_dir();
    let mut failed = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        failed_dir.path(),
        fencing_policy(),
        checkpoint_authority(),
    )
    .unwrap();
    fs::write(
        failed_dir.path().join(".fencing-state.json.tmp"),
        b"collision",
    )
    .unwrap();
    assert_eq!(
        commit(
            &mut failed,
            &fixture,
            &lease,
            b"atomic",
            OperationClass::Fence,
            101,
            2,
        ),
        Err(FencingError::DurabilityFailure)
    );
    assert!(failed.floor(LINEAGE).is_none());
    marker("atomic_receipt_failure", "fail_closed");

    let cleanup_dir = state_dir();
    let cleanup_authority = checkpoint_authority();
    let mut cleanup = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        cleanup_dir.path(),
        fencing_policy(),
        cleanup_authority.clone(),
    )
    .unwrap();
    cleanup.fail_next_lock_cleanup_for_test();
    assert_eq!(
        commit(
            &mut cleanup,
            &fixture,
            &lease,
            b"cleanup",
            OperationClass::Fence,
            101,
            2,
        ),
        Err(FencingError::DurabilityFailure)
    );
    assert_eq!(cleanup.floor(LINEAGE).unwrap().epoch, 2);
    assert_eq!(
        cleanup.checkpoint(),
        cleanup_authority.current().unwrap().unwrap()
    );
    assert_eq!(
        cleanup.authorize_active_lease(
            &TEST_FENCING_STORE_ACCESS,
            ActiveLeaseCheck {
                membership: Some(&active_membership),
                lease: &lease,
                applied_log_index: 100,
                now_unix_seconds: NOW,
                now_unix_millis: lease.deadline_unix_millis - 1,
                now_elapsed_millis: 2_009,
                activation_proof: &proof,
            }
        ),
        Err(FencingError::DurabilityFailure)
    );
}

#[test]
fn state_paths_and_capacity_fail_closed() {
    let relative = std::path::Path::new("relative-state");
    assert_eq!(
        FencingStore::create(
            &TEST_FENCING_STORE_ACCESS,
            relative,
            fencing_policy(),
            checkpoint_authority()
        )
        .unwrap_err(),
        FencingError::UnsafeStatePath
    );
    marker("unsafe_state_path", "denied");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let parent = state_dir();
        let target = parent.path().join("target");
        fs::create_dir(&target).unwrap();
        let link = parent.path().join("link");
        symlink(&target, &link).unwrap();
        assert_eq!(
            FencingStore::create(
                &TEST_FENCING_STORE_ACCESS,
                &link,
                fencing_policy(),
                checkpoint_authority()
            )
            .unwrap_err(),
            FencingError::UnsafeStatePath
        );
    }
    marker("symlink_state_path", "denied");

    let fixture = Fixture::new(101);
    let lease = fixture.grant(100);
    let directory = state_dir();
    let mut policy = fencing_policy();
    policy.max_receipts = 1;
    let mut store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        directory.path(),
        policy,
        checkpoint_authority(),
    )
    .unwrap();
    commit(
        &mut store,
        &fixture,
        &lease,
        b"first",
        OperationClass::Fence,
        101,
        2,
    )
    .unwrap();
    assert_eq!(
        commit(
            &mut store,
            &fixture,
            &lease,
            b"second",
            OperationClass::Fence,
            102,
            2,
        ),
        Err(FencingError::ResourceExhausted)
    );
    marker("state_capacity", "denied");
}
