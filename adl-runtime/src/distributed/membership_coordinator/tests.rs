use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Mutex,
};

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

fn old_stable_ids() -> BTreeMap<Vec<u8>, u64> {
    BTreeMap::from([
        (b"guardian-1".to_vec(), 1),
        (b"guardian-2".to_vec(), 2),
        (b"guardian-3".to_vec(), 3),
    ])
}

fn target_stable_ids() -> BTreeMap<Vec<u8>, u64> {
    let mut target = old_stable_ids();
    target.insert(b"guardian-4".to_vec(), 4);
    target
}

fn old_membership() -> BTreeSet<u64> {
    old_stable_ids().values().copied().collect()
}

fn target_membership() -> BTreeSet<u64> {
    target_stable_ids().values().copied().collect()
}

fn promotion_named(operation_id: &str) -> VerifiedPromoteVoter {
    let artifact = PromoteVoterArtifact::committed(
        identity(),
        [1; 32],
        [2; 32],
        7,
        stable_map_sha256(&old_stable_ids()).unwrap(),
        stable_map_sha256(&target_stable_ids()).unwrap(),
        membership_set_sha256(&target_membership()).unwrap(),
        NOW + 100,
    )
    .unwrap();
    let published = test_published_reconciliation_token(
        authority_identity(),
        operation_id,
        artifact,
        50,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    VerifiedPromoteVoter::from_published(
        &published,
        &identity(),
        [1; 32],
        stable_map_sha256(&old_stable_ids()).unwrap(),
        stable_map_sha256(&target_stable_ids()).unwrap(),
        NOW,
    )
    .unwrap()
}

fn promotion() -> VerifiedPromoteVoter {
    promotion_named("promote-4")
}

#[test]
fn promote_artifact_requires_exact_discriminator_and_maps() {
    let promotion = promotion();
    assert_eq!(promotion.identity().stable_raft_id, 4);
    assert_eq!(
        promotion.old_stable_map_sha256(),
        stable_map_sha256(&old_stable_ids()).unwrap()
    );
    assert_eq!(
        promotion.target_stable_map_sha256(),
        stable_map_sha256(&target_stable_ids()).unwrap()
    );
    assert_eq!(
        promotion.target_membership_sha256(),
        membership_set_sha256(&target_membership()).unwrap()
    );
}

#[test]
fn authorized_transition_binds_exact_stable_maps_and_membership() {
    let promotion = promotion();
    assert_eq!(
        verify_authorized_transition_inputs(
            &promotion,
            &old_stable_ids(),
            &target_stable_ids(),
            &old_membership(),
            &target_membership(),
        ),
        Ok(())
    );
    let unrelated_target = BTreeSet::from([1, 2, 3, 9]);
    assert_eq!(
        verify_authorized_transition_inputs(
            &promotion,
            &old_stable_ids(),
            &target_stable_ids(),
            &old_membership(),
            &unrelated_target,
        ),
        Err(MembershipCoordinatorError::WrongStableMap)
    );
    let mut remapped = target_stable_ids();
    remapped.insert(b"guardian-4".to_vec(), 9);
    assert_eq!(
        verify_authorized_transition_inputs(
            &promotion,
            &old_stable_ids(),
            &remapped,
            &old_membership(),
            &target_membership(),
        ),
        Err(MembershipCoordinatorError::WrongStableMap)
    );
    println!("ADL_ISSUE_199_ASSERTION_V1 case=old_cut_mismatch assertion=authorized_stable_maps_and_target_membership_bound_before_raft_effect");
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
    let old = BTreeSet::from([1, 2, 3]);
    let target = BTreeSet::from([1, 2, 3, 4]);
    assert!(membership_configs_are_exact_old(&[old.clone()], &old));
    assert!(!membership_configs_are_exact_old(
        &[old.clone(), target.clone()],
        &old
    ));
    assert!(!membership_configs_are_exact_old(&[target], &old));
    assert_eq!(
        prepare_enrollment_stable_ids(
            &BTreeMap::from([(b"guardian-other".to_vec(), 4)]),
            &identity(),
        ),
        Err(MembershipCoordinatorError::WrongStableMap)
    );
    assert_eq!(
        prepare_enrollment_stable_ids(&BTreeMap::from([(b"guardian-4".to_vec(), 5)]), &identity(),),
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
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .observe_external_authority(&promotion, &receipt)
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .record_learner_caught_up(promotion.operation_sha256())
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .record_joint_membership(promotion.operation_sha256(), [9; 32])
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .record_final_membership(promotion.operation_sha256(), [10; 32])
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .reconcile_authority_parity(promotion.operation_sha256(), &receipt, [11; 32], [12; 32])
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    coordinator
        .checkpoint(promotion.operation_sha256())
        .unwrap();
    drop(coordinator);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint.clone()).unwrap();
    let result = coordinator
        .publish(promotion.operation_sha256(), &target_stable_ids())
        .unwrap();
    assert_ne!(result, [0; 32]);
    assert_eq!(coordinator.published_generation(), 1);

    drop(coordinator);
    let mut restored = MembershipCoordinator::open(root.path(), checkpoint).unwrap();
    assert_eq!(
        restored
            .publish(promotion.operation_sha256(), &target_stable_ids())
            .unwrap(),
        result
    );
    assert_eq!(restored.published_generation(), 1);
    println!("ADL_ISSUE_199_ASSERTION_V1 case=crash_every_phase assertion=durable_saga_restart_exact_retry_no_duplicate_publication");
}

#[test]
fn conflicting_transition_and_receipt_fail_closed() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let promotion = promotion();
    let conflicting = promotion_named("promote-other");
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint).unwrap();
    coordinator.begin_promotion(&promotion).unwrap();
    assert_eq!(
        coordinator.begin_promotion(&conflicting),
        Err(MembershipCoordinatorError::StateRegression)
    );
    let wrong_operation =
        GovernedMembershipAuthorityReceipt::for_membership_coordinator_test([0x77; 32], 7, [8; 32]);
    assert_eq!(
        coordinator.observe_external_authority(&promotion, &wrong_operation),
        Err(MembershipCoordinatorError::ReceiptMismatch)
    );
    println!("ADL_ISSUE_199_ASSERTION_V1 case=conflicting_retry assertion=conflicting_operation_and_receipt_denied_before_effect");
}

#[test]
fn committed_history_must_be_newer_than_authorized_operation() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let promotion = promotion();
    let receipt =
        GovernedMembershipAuthorityReceipt::for_membership_coordinator_test([2; 32], 7, [8; 32]);
    let mut coordinator = MembershipCoordinator::open(root.path(), checkpoint).unwrap();
    coordinator.begin_promotion(&promotion).unwrap();
    coordinator
        .observe_external_authority(&promotion, &receipt)
        .unwrap();
    coordinator
        .record_learner_caught_up(promotion.operation_sha256())
        .unwrap();

    let stale = vec![
        AppliedMembershipEntry {
            log_id: openraft::LogId::new(openraft::CommittedLeaderId::new(3, 1), 40),
            joint_configs: vec![old_membership(), target_membership()],
        },
        AppliedMembershipEntry {
            log_id: openraft::LogId::new(openraft::CommittedLeaderId::new(3, 1), 41),
            joint_configs: vec![target_membership()],
        },
    ];
    assert_eq!(
        coordinator.record_committed_membership_history(
            promotion.operation_sha256(),
            &stale,
            &old_membership(),
            &target_membership(),
        ),
        Err(MembershipCoordinatorError::StateRegression)
    );
    assert_eq!(
        coordinator.active_phase(),
        Some(MembershipCoordinatorPhase::LearnerCaughtUp)
    );

    let current = vec![
        stale[0].clone(),
        stale[1].clone(),
        AppliedMembershipEntry {
            log_id: openraft::LogId::new(openraft::CommittedLeaderId::new(4, 1), 51),
            joint_configs: vec![old_membership(), target_membership()],
        },
        AppliedMembershipEntry {
            log_id: openraft::LogId::new(openraft::CommittedLeaderId::new(4, 1), 52),
            joint_configs: vec![target_membership()],
        },
    ];
    coordinator
        .record_committed_membership_history(
            promotion.operation_sha256(),
            &current,
            &old_membership(),
            &target_membership(),
        )
        .unwrap();
    assert_eq!(
        coordinator.active_phase(),
        Some(MembershipCoordinatorPhase::FinalCommitted)
    );
    println!("ADL_ISSUE_199_ASSERTION_V1 case=leader_change_resume assertion=membership_history_entries_newer_than_authority_log_index_required");
}
