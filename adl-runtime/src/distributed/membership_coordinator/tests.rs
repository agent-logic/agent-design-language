use std::{collections::BTreeMap, sync::Mutex};

use ed25519_dalek::SigningKey;

use super::*;
use crate::distributed::{
    authority_protocol::{
        test_published_reconciliation_token, AuthorityNodeIdentity, CanonicalAuthorityTime,
    },
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    polis_runtime::{ConsensusCheckpoint, PolisRuntimeError},
};

const NOW: i64 = 2_000_000_000;

#[derive(Default)]
struct MemoryCheckpoint(Mutex<BTreeMap<String, ConsensusCheckpoint>>);

impl ConsensusCheckpointAuthority for MemoryCheckpoint {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.0.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut state = self.0.lock().unwrap();
        if state.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::Replay);
        }
        state.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn identity() -> LearnerIdentity {
    LearnerIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-4".to_owned(),
        guardian_id: "guardian-4".to_owned(),
        guardian_control_public_key: SigningKey::from_bytes(&[44; 32]).verifying_key().to_bytes(),
        stable_raft_id: 4,
        certificate_generation: 2,
        boot_generation: 2,
        address: "127.0.0.1:40404".parse().unwrap(),
    }
}

fn authority_identity() -> AuthorityNodeIdentity {
    AuthorityNodeIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-1".to_owned(),
        guardian_id: "guardian-1".to_owned(),
        boot_generation: 2,
    }
}

fn promotion() -> VerifiedPromoteVoter {
    let artifact = PromoteVoterArtifact::committed(
        identity(),
        [1; 32],
        [2; 32],
        7,
        [3; 32],
        [4; 32],
        [5; 32],
        NOW + 100,
    )
    .unwrap();
    let published = test_published_reconciliation_token(
        authority_identity(),
        "promote-4",
        artifact,
        50,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    VerifiedPromoteVoter::from_published(&published, &identity(), [1; 32], [3; 32], [4; 32], NOW)
        .unwrap()
}

#[test]
fn promote_artifact_requires_exact_discriminator_and_maps() {
    let promotion = promotion();
    assert_eq!(promotion.identity().stable_raft_id, 4);
    assert_eq!(promotion.old_stable_map_sha256(), [3; 32]);
    assert_eq!(promotion.target_stable_map_sha256(), [4; 32]);
}

#[test]
fn stable_map_digest_rejects_collisions_and_zero() {
    let valid = BTreeMap::from([(b"guardian-1".to_vec(), 9), (b"guardian-2".to_vec(), 2)]);
    assert_ne!(stable_map_sha256(&valid).unwrap(), [0; 32]);
    assert_eq!(
        stable_map_sha256(&BTreeMap::from([
            (b"guardian-1".to_vec(), 9),
            (b"guardian-2".to_vec(), 9)
        ])),
        Err(MembershipCoordinatorError::WrongStableMap)
    );
}

#[test]
fn authority_membership_preserves_explicit_stable_ids() {
    let guardians = [
        b"guardian-b".to_vec(),
        b"guardian-c".to_vec(),
        b"guardian-d".to_vec(),
    ];
    let configs = vec![guardians.iter().cloned().collect()];
    let voters = guardians
        .iter()
        .enumerate()
        .map(|(index, guardian_id)| VoterAuthority {
            guardian_id: guardian_id.clone(),
            trust_domain_id: b"runtime-prod".to_vec(),
            certificate_generation: 3,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: NOW - 100,
            not_after_unix_seconds: NOW + 100,
            revoked: false,
            control_public_key: SigningKey::from_bytes(&[(index + 1) as u8; 32])
                .verifying_key()
                .to_bytes(),
        })
        .collect();
    let ids = BTreeMap::from([
        (b"guardian-b".to_vec(), 91),
        (b"guardian-c".to_vec(), 7),
        (b"guardian-d".to_vec(), 33),
    ]);
    let authority = AuthorityMembership::new_with_stable_ids(
        b"runtime-prod".to_vec(),
        3,
        20,
        configs,
        voters,
        ids.clone(),
    )
    .unwrap();
    assert_eq!(authority.raft_ids, ids);
}

#[test]
fn durable_saga_requires_exact_current_receipt_and_order() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let promotion = promotion();
    let receipt =
        GovernedMembershipAuthorityReceipt::for_membership_coordinator_test([2; 32], 7, [8; 32]);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator.begin_promotion(&promotion).unwrap();
    assert_eq!(
        coordinator.record_learner_caught_up(promotion.operation_sha256()),
        Err(MembershipCoordinatorError::StateRegression)
    );
    coordinator
        .observe_external_authority(&promotion, &receipt)
        .unwrap();
    coordinator
        .record_learner_caught_up(promotion.operation_sha256())
        .unwrap();
    coordinator
        .record_joint_membership(promotion.operation_sha256(), [9; 32])
        .unwrap();
    coordinator
        .record_final_membership(promotion.operation_sha256(), [10; 32])
        .unwrap();
    coordinator
        .reconcile_authority_parity(promotion.operation_sha256(), &receipt, [11; 32], [12; 32])
        .unwrap();
    coordinator
        .checkpoint(promotion.operation_sha256())
        .unwrap();
    let result = coordinator.publish(promotion.operation_sha256()).unwrap();
    assert_ne!(result, [0; 32]);
    assert_eq!(coordinator.published_generation(), 1);

    drop(coordinator);
    let mut restored = MembershipCoordinator::open(root.path(), checkpoint).unwrap();
    assert_eq!(
        restored.publish(promotion.operation_sha256()).unwrap(),
        result
    );
    assert_eq!(restored.published_generation(), 1);
}
