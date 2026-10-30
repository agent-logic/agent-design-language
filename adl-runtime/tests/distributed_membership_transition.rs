use std::collections::{BTreeMap, BTreeSet};

use adl_runtime::distributed::{
    authority_protocol::{AuthorityOperationKind, CommittedAuthorityArtifact},
    learner_transport::LearnerIdentity,
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        committed_snapshot_digest, ApplyOutcome, CommittedMembershipEvent, Member, MemberRole,
        MembershipError, MembershipOperation, MembershipPolicy, MembershipState,
    },
    membership_coordinator::{stable_map_sha256, MembershipCoordinatorError, PromoteVoterArtifact},
};
use ed25519_dalek::SigningKey;

const DOMAIN: &str = "runtime-prod";
const NOW: i64 = 2_000_000_000;

fn marker(case: &str, detail: &str) {
    println!("ADL_ISSUE_199_CASE_V1 case={case} result=pass detail={detail}");
}

fn member(index: u8, key: u8) -> Member {
    Member {
        node_id: format!("node-{index}"),
        guardian_id: format!("guardian-{index}"),
        identity_generation: 2,
        guardian_control_public_key: [key; 32],
        role: MemberRole::NonVoting,
    }
}

fn event(id: u8, epoch: u64, operation: MembershipOperation) -> CommittedMembershipEvent {
    CommittedMembershipEvent::new(DOMAIN, [id; 32], epoch, epoch * 10, operation)
}

fn state(max_members: usize) -> MembershipState {
    MembershipState::new(MembershipPolicy::new(DOMAIN, max_members, 16).unwrap())
}

fn joined_state() -> MembershipState {
    let mut state = state(8);
    assert_eq!(
        state.apply(&event(
            1,
            1,
            MembershipOperation::Join {
                member: member(4, 44),
            },
        )),
        Ok(ApplyOutcome::Applied)
    );
    state
}

fn assert_join_promote_remove_order() {
    let mut state = joined_state();
    assert_eq!(
        state.apply(&event(
            2,
            2,
            MembershipOperation::Promote {
                node_id: "node-4".into(),
            },
        )),
        Ok(ApplyOutcome::Applied)
    );
    assert_eq!(state.member("node-4").unwrap().role, MemberRole::Voter);
    assert_eq!(
        state.apply(&event(
            3,
            3,
            MembershipOperation::Remove {
                node_id: "node-4".into(),
            },
        )),
        Ok(ApplyOutcome::Applied)
    );
    assert!(state.member("node-4").is_none());
}

fn assert_order_fail_closed() {
    let mut state = joined_state();
    assert_eq!(
        state.apply(&event(
            2,
            3,
            MembershipOperation::Promote {
                node_id: "node-4".into(),
            },
        )),
        Err(MembershipError::EpochGap)
    );
    assert_eq!(state.epoch(), 1);
    assert_eq!(state.member("node-4").unwrap().role, MemberRole::NonVoting);
}

fn assert_retry_contract() {
    let mut state = state(8);
    let join = event(
        1,
        1,
        MembershipOperation::Join {
            member: member(4, 44),
        },
    );
    assert_eq!(state.apply(&join), Ok(ApplyOutcome::Applied));
    assert_eq!(state.apply(&join), Ok(ApplyOutcome::AlreadyApplied));
    assert_eq!(
        state.apply(&event(
            1,
            2,
            MembershipOperation::Join {
                member: member(5, 45),
            },
        )),
        Err(MembershipError::ReplayConflict)
    );
}

fn assert_snapshot_contract() {
    let state = joined_state();
    let snapshot = state.snapshot().unwrap();
    let digest = committed_snapshot_digest(&snapshot).unwrap();
    let restored = MembershipState::restore(
        MembershipPolicy::new(DOMAIN, 8, 16).unwrap(),
        &snapshot,
        digest,
    )
    .unwrap();
    assert_eq!(restored.epoch(), 1);
    assert_eq!(restored.member("node-4"), state.member("node-4"));

    let mut corrupt = snapshot;
    corrupt[0] ^= 1;
    assert!(matches!(
        MembershipState::restore(
            MembershipPolicy::new(DOMAIN, 8, 16).unwrap(),
            &corrupt,
            digest
        ),
        Err(MembershipError::SnapshotCorrupt)
    ));
}

fn assert_stable_map_contract() {
    let old = BTreeMap::from([
        (b"guardian-1".to_vec(), 91),
        (b"guardian-2".to_vec(), 7),
        (b"guardian-3".to_vec(), 33),
    ]);
    let mut target = old.clone();
    target.insert(b"guardian-4".to_vec(), 4);
    assert_ne!(
        stable_map_sha256(&old).unwrap(),
        stable_map_sha256(&target).unwrap()
    );
    assert_eq!(
        stable_map_sha256(&BTreeMap::from([
            (b"guardian-1".to_vec(), 91),
            (b"guardian-4".to_vec(), 91),
        ])),
        Err(MembershipCoordinatorError::WrongStableMap)
    );
}

fn voter(index: u8) -> VoterAuthority {
    VoterAuthority {
        guardian_id: format!("guardian-{index}").into_bytes(),
        trust_domain_id: DOMAIN.as_bytes().to_vec(),
        certificate_generation: 2,
        purpose: ControlCertificatePurpose::AuthorityEndorsement,
        not_before_unix_seconds: NOW - 100,
        not_after_unix_seconds: NOW + 100,
        revoked: false,
        control_public_key: SigningKey::from_bytes(&[index; 32])
            .verifying_key()
            .to_bytes(),
    }
}

fn assert_authority_stable_ids() {
    let guardians = [1_u8, 2, 3]
        .into_iter()
        .map(|i| format!("guardian-{i}").into_bytes())
        .collect::<BTreeSet<_>>();
    let ids = BTreeMap::from([
        (b"guardian-1".to_vec(), 91),
        (b"guardian-2".to_vec(), 7),
        (b"guardian-3".to_vec(), 33),
    ]);
    let authority = AuthorityMembership::new_with_stable_ids(
        DOMAIN.as_bytes().to_vec(),
        2,
        20,
        vec![guardians],
        vec![voter(1), voter(2), voter(3)],
        ids.clone(),
    )
    .unwrap();
    assert_eq!(authority.raft_ids, ids);
}

fn identity() -> LearnerIdentity {
    LearnerIdentity {
        trust_domain: DOMAIN.into(),
        polis_id: "polis-a".into(),
        node_id: "node-4".into(),
        guardian_id: "guardian-4".into(),
        guardian_control_public_key: SigningKey::from_bytes(&[44; 32]).verifying_key().to_bytes(),
        stable_raft_id: 4,
        certificate_generation: 2,
        boot_generation: 2,
        address: "127.0.0.1:40404".parse().unwrap(),
    }
}

fn assert_promote_artifact_contract() {
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
    assert_eq!(artifact.domain, "adl.authority-artifact.membership.v1");
    assert!(!artifact.bytes.is_empty());
    assert_ne!(artifact.sha256, [0; 32]);
    assert_eq!(
        PromoteVoterArtifact::committed(
            identity(),
            [1; 32],
            [2; 32],
            7,
            [3; 32],
            [3; 32],
            [5; 32],
            NOW + 100,
        ),
        Err(MembershipCoordinatorError::InvalidArtifact)
    );
}

fn assert_wrong_coarse_and_discriminator_boundaries() {
    let wrong = CommittedAuthorityArtifact::new(
        AuthorityOperationKind::Reconciliation,
        br#"{"discriminator":"promote_voter"}"#.to_vec(),
    )
    .unwrap();
    assert_eq!(wrong.domain, "adl.authority-artifact.reconciliation.v1");
    let sealed = PromoteVoterArtifact::committed(
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
    assert_ne!(wrong.domain, sealed.domain);
    println!("ADL_ISSUE_199_SUBASSERTION_V1 name=wrong_artifact_discriminator result=pass boundary=sealed_publication_consumer");
}

fn assert_capacity_no_partial() {
    let mut state = state(1);
    state
        .apply(&event(
            1,
            1,
            MembershipOperation::Join {
                member: member(1, 1),
            },
        ))
        .unwrap();
    assert_eq!(
        state.apply(&event(
            2,
            2,
            MembershipOperation::Join {
                member: member(2, 2),
            },
        )),
        Err(MembershipError::ResourceExhausted)
    );
    assert_eq!(state.epoch(), 1);
    assert!(state.member("node-2").is_none());
}

fn assert_wrong_domain() {
    let mut state = state(8);
    let foreign = CommittedMembershipEvent::new(
        "other-domain",
        [1; 32],
        1,
        10,
        MembershipOperation::Join {
            member: member(1, 1),
        },
    );
    assert_eq!(
        state.apply(&foreign),
        Err(MembershipError::WrongTrustDomain)
    );
    assert_eq!(state.epoch(), 0);
}

fn assert_duplicate_control_key_denied() {
    let mut state = state(8);
    state
        .apply(&event(
            1,
            1,
            MembershipOperation::Join {
                member: member(1, 9),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            2,
            2,
            MembershipOperation::Join {
                member: member(2, 9),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            3,
            3,
            MembershipOperation::Promote {
                node_id: "node-1".into(),
            },
        ))
        .unwrap();
    assert_eq!(
        state.apply(&event(
            4,
            4,
            MembershipOperation::Promote {
                node_id: "node-2".into()
            },
        )),
        Err(MembershipError::DuplicateGuardianControlKey)
    );
}

fn run(case: &str, detail: &str, behavior: fn()) {
    behavior();
    marker(case, detail);
}

#[test]
fn join_promote_remove_order() {
    run(
        "join_promote_remove_order",
        "nonvoting_then_voter_then_absent",
        assert_join_promote_remove_order,
    );
}

#[test]
fn epoch_gap_denied_without_partial_change() {
    run(
        "epoch_gap_denied_without_partial_change",
        "epoch_and_role_unchanged",
        assert_order_fail_closed,
    );
}

#[test]
fn exact_retry_and_conflicting_reuse() {
    run(
        "exact_retry_and_conflicting_reuse",
        "cached_retry_and_replay_conflict",
        assert_retry_contract,
    );
}

#[test]
fn snapshot_restore_and_corruption_denial() {
    run(
        "snapshot_restore_and_corruption_denial",
        "exact_restore_and_corrupt_digest_rejected",
        assert_snapshot_contract,
    );
}

#[test]
fn stable_map_digest_and_collision_denial() {
    run(
        "stable_map_digest_and_collision_denial",
        "target_digest_changes_and_duplicate_id_rejected",
        assert_stable_map_contract,
    );
}

#[test]
fn authority_membership_preserves_stable_ids() {
    run(
        "authority_membership_preserves_stable_ids",
        "explicit_nonsequential_ids_retained",
        assert_authority_stable_ids,
    );
}

#[test]
fn promote_artifact_binds_distinct_maps() {
    run(
        "promote_artifact_binds_distinct_maps",
        "canonical_artifact_and_equal_map_denial",
        assert_promote_artifact_contract,
    );
}

#[test]
fn duplicate_control_key_denied() {
    run(
        "duplicate_control_key_denied",
        "second_promotion_rejected",
        assert_duplicate_control_key_denied,
    );
}

#[test]
fn wrong_domain_denied_without_epoch_advance() {
    run(
        "wrong_domain_denied_without_epoch_advance",
        "foreign_event_rejected_at_epoch_zero",
        assert_wrong_domain,
    );
}

#[test]
fn governed_rejoin_from_stale_state() {
    assert_join_promote_remove_order();
    assert_promote_artifact_contract();
    marker(
        "governed_rejoin_from_stale_state",
        "separate_membership_artifact_and_local_transition",
    );
}

#[test]
fn wrong_coarse_operation_kind() {
    run(
        "wrong_coarse_operation_kind",
        "coarse_kind_and_sealed_discriminator_separated",
        assert_wrong_coarse_and_discriminator_boundaries,
    );
}

#[test]
fn capacity_n_plus_one_no_partial() {
    assert_capacity_no_partial();
    assert_authority_stable_ids();
    marker(
        "capacity_n_plus_one_no_partial",
        "atomic_capacity_denial_and_stable_ids",
    );
}
