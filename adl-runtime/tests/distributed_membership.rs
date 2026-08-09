#[path = "../src/distributed/membership.rs"]
mod membership;

use membership::{
    ApplyOutcome, CommittedMembershipEvent, Member, MemberRole, MembershipError,
    MembershipOperation, MembershipPolicy, MembershipState,
};

fn policy(max_members: usize) -> MembershipPolicy {
    let policy = MembershipPolicy::new("polis.test", max_members, 8).expect("policy");
    assert_eq!(policy.trust_domain(), "polis.test");
    policy
}

fn member(node: u8, key: u8) -> Member {
    Member {
        node_id: format!("node_{node}"),
        guardian_id: format!("guardian_{node}"),
        identity_generation: 1,
        guardian_control_public_key: [key; 32],
        role: MemberRole::NonVoting,
    }
}

fn event(id: u8, epoch: u64, operation: MembershipOperation) -> CommittedMembershipEvent {
    CommittedMembershipEvent::new("polis.test", [id; 32], epoch, epoch * 10, operation)
}

#[test]
fn ordered_committed_events_converge_deterministically() {
    let events = [
        event(
            1,
            1,
            MembershipOperation::Join {
                member: member(1, 1),
            },
        ),
        event(
            2,
            2,
            MembershipOperation::Join {
                member: member(2, 2),
            },
        ),
        event(
            3,
            3,
            MembershipOperation::Promote {
                node_id: "node_1".into(),
            },
        ),
    ];
    let mut left = MembershipState::new(policy(4));
    let mut right = MembershipState::new(policy(4));
    for item in &events {
        assert_eq!(left.apply(item), Ok(ApplyOutcome::Applied));
        assert_eq!(right.apply(item), Ok(ApplyOutcome::Applied));
    }
    assert_eq!(left.snapshot().unwrap(), right.snapshot().unwrap());
    assert_eq!(left.members().count(), 2);
    assert_eq!(left.epoch(), 3);
    assert_eq!(left.committed_log_index(), 30);
}

#[test]
fn duplicate_is_idempotent_while_conflict_stale_and_gap_fail_closed() {
    let mut state = MembershipState::new(policy(4));
    let join = event(
        1,
        1,
        MembershipOperation::Join {
            member: member(1, 1),
        },
    );
    assert_eq!(state.apply(&join), Ok(ApplyOutcome::Applied));
    assert_eq!(state.apply(&join), Ok(ApplyOutcome::AlreadyApplied));

    let conflicting = event(
        1,
        2,
        MembershipOperation::Join {
            member: member(2, 2),
        },
    );
    assert_eq!(
        state.apply(&conflicting),
        Err(MembershipError::ReplayConflict)
    );
    assert_eq!(
        state.apply(&event(
            2,
            1,
            MembershipOperation::Remove {
                node_id: "node_1".into()
            }
        )),
        Err(MembershipError::StaleEpoch)
    );
    assert_eq!(
        state.apply(&event(
            3,
            3,
            MembershipOperation::Remove {
                node_id: "node_1".into()
            }
        )),
        Err(MembershipError::EpochGap)
    );
}

#[test]
fn duplicate_effective_voter_control_key_is_rejected() {
    let mut state = MembershipState::new(policy(4));
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
                node_id: "node_1".into(),
            },
        ))
        .unwrap();
    assert_eq!(
        state.apply(&event(
            4,
            4,
            MembershipOperation::Promote {
                node_id: "node_2".into()
            }
        )),
        Err(MembershipError::DuplicateGuardianControlKey)
    );
    assert_eq!(state.epoch(), 3);
    assert_eq!(state.member("node_2").unwrap().role, MemberRole::NonVoting);
}

#[test]
fn verified_restart_preserves_membership_voter_uniqueness_and_replay() {
    let mut state = MembershipState::new(policy(4));
    let join = event(
        1,
        1,
        MembershipOperation::Join {
            member: member(1, 7),
        },
    );
    state.apply(&join).unwrap();
    state
        .apply(&event(
            2,
            2,
            MembershipOperation::Promote {
                node_id: "node_1".into(),
            },
        ))
        .unwrap();

    let bytes = state.snapshot().unwrap();
    let mut restored = MembershipState::restore(policy(4), &bytes).unwrap();
    assert_eq!(restored.epoch(), 2);
    assert_eq!(restored.member("node_1").unwrap().role, MemberRole::Voter);
    assert_eq!(restored.apply(&join), Ok(ApplyOutcome::AlreadyApplied));

    restored
        .apply(&event(
            3,
            3,
            MembershipOperation::Join {
                member: member(2, 7),
            },
        ))
        .unwrap();
    assert_eq!(
        restored.apply(&event(
            4,
            4,
            MembershipOperation::Promote {
                node_id: "node_2".into()
            }
        )),
        Err(MembershipError::DuplicateGuardianControlKey)
    );
}

#[test]
fn tampered_or_policy_mismatched_snapshot_is_rejected() {
    let mut state = MembershipState::new(policy(2));
    state
        .apply(&event(
            1,
            1,
            MembershipOperation::Join {
                member: member(1, 1),
            },
        ))
        .unwrap();
    let bytes = state.snapshot().unwrap();
    assert_eq!(
        MembershipState::restore(policy(3), &bytes).unwrap_err(),
        MembershipError::SnapshotCorrupt
    );
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["body"]["epoch"] = serde_json::json!(99);
    let tampered = serde_json::to_vec(&value).unwrap();
    assert_eq!(
        MembershipState::restore(policy(2), &tampered).unwrap_err(),
        MembershipError::SnapshotCorrupt
    );
}

#[test]
fn bounds_wrong_domain_and_invalid_shape_fail_before_mutation() {
    let mut state = MembershipState::new(policy(1));
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
                member: member(2, 2)
            }
        )),
        Err(MembershipError::ResourceExhausted)
    );

    let mut wrong_domain = event(
        3,
        2,
        MembershipOperation::Remove {
            node_id: "node_1".into(),
        },
    );
    wrong_domain.trust_domain = "other.test".into();
    assert_eq!(
        state.apply(&wrong_domain),
        Err(MembershipError::WrongTrustDomain)
    );

    let invalid = event(
        4,
        2,
        MembershipOperation::Join {
            member: Member {
                node_id: String::new(),
                ..member(2, 2)
            },
        },
    );
    assert_eq!(state.apply(&invalid), Err(MembershipError::InvalidEvent));
    assert_eq!(state.epoch(), 1);
}

#[test]
fn removed_voter_key_can_be_reused_only_after_committed_removal() {
    let mut state = MembershipState::new(policy(3));
    state
        .apply(&event(
            1,
            1,
            MembershipOperation::Join {
                member: member(1, 5),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            2,
            2,
            MembershipOperation::Promote {
                node_id: "node_1".into(),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            3,
            3,
            MembershipOperation::Remove {
                node_id: "node_1".into(),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            4,
            4,
            MembershipOperation::Join {
                member: member(2, 5),
            },
        ))
        .unwrap();
    state
        .apply(&event(
            5,
            5,
            MembershipOperation::Promote {
                node_id: "node_2".into(),
            },
        ))
        .unwrap();
    assert_eq!(state.member("node_2").unwrap().role, MemberRole::Voter);
}
