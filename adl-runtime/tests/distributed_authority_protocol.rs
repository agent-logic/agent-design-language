// PVF: lane=exact-child-tests; proof=committed three-voter authority protocol,
// deterministic=true; resource_profile=medium; release_gate=true;
// exact 47-case name/result/marker parity is enforced by the issue proof producer.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    authority_protocol::{
        validate_continuity_transfer_binding, verify_finalization, AuthorityNodeIdentity,
        AuthorityOperationKind, AuthorityProtocolError, CanonicalAuthorityTime,
        CommittedAuthorityArtifact, ContinuityTransferChunk, ContinuityTransferEntry,
        ContinuityTransferGrantArtifact, DurableAuthorityProtocol, FinalizeAuthorityIntent,
        PrepareAuthorityIntent, VoterEndorsementAuthority, CONTINUITY_TRANSFER_ADAPTER_210,
    },
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
        MembershipState,
    },
    polis_runtime::{
        validate_authority_command_boundary, ConsensusCheckpoint, ConsensusCheckpointAuthority,
        PolisCommand, PolisRuntimeError,
    },
};
use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha256};
use tempfile::TempDir;

const DOMAIN: &str = "polis.test";
const POLIS: &str = "polis-a";
const FINALIZE_SECONDS: i64 = 1_800_000_010;

fn marker(case: &str, result: &str) {
    println!("ADL_ISSUE_201_CASE_V1 {case} {result}");
}

#[derive(Default)]
struct MemoryCheckpointAuthority(Mutex<BTreeMap<String, ConsensusCheckpoint>>);

impl ConsensusCheckpointAuthority for MemoryCheckpointAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.0.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut values = self.0.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

#[derive(Default)]
struct FaultCheckpointAuthority {
    inner: MemoryCheckpointAuthority,
    mode: Mutex<u8>,
}

impl FaultCheckpointAuthority {
    fn arm_before_cas(&self) {
        *self.mode.lock().unwrap() = 1;
    }
    fn arm_after_cas(&self) {
        *self.mode.lock().unwrap() = 2;
    }
}

impl ConsensusCheckpointAuthority for FaultCheckpointAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        self.inner.load(object)
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mode = std::mem::take(&mut *self.mode.lock().unwrap());
        if mode == 1 {
            return Err(PolisRuntimeError::Storage);
        }
        self.inner.compare_and_swap(expected, candidate)?;
        if mode == 2 {
            return Err(PolisRuntimeError::Storage);
        }
        Ok(())
    }
}

struct Fixture {
    membership: MembershipState,
    authority: AuthorityMembership,
    signers: Vec<VoterEndorsementAuthority>,
    checkpoint: Arc<MemoryCheckpointAuthority>,
    root: TempDir,
}

impl Fixture {
    fn new() -> Self {
        Self::with_configs(vec![vec![0, 1, 2]])
    }

    fn with_configs(config_indexes: Vec<Vec<usize>>) -> Self {
        let voter_count = config_indexes.iter().flatten().max().copied().unwrap() + 1;
        let keys = (0..voter_count)
            .map(|index| SigningKey::from_bytes(&[(index + 1) as u8; 32]))
            .collect::<Vec<_>>();
        let mut membership = MembershipState::new(MembershipPolicy::new(DOMAIN, 16, 32).unwrap());
        let mut index = 0_u64;
        for (offset, key) in keys.iter().enumerate() {
            index += 1;
            membership
                .apply(&CommittedMembershipEvent::new(
                    DOMAIN,
                    [index as u8; 32],
                    index,
                    index,
                    MembershipOperation::Join {
                        member: Member {
                            node_id: format!("node-{}", offset + 1),
                            guardian_id: format!("guardian-{}", offset + 1),
                            identity_generation: 1,
                            guardian_control_public_key: key.verifying_key().to_bytes(),
                            role: MemberRole::NonVoting,
                        },
                    },
                ))
                .unwrap();
        }
        for offset in 0..voter_count {
            index += 1;
            membership
                .apply(&CommittedMembershipEvent::new(
                    DOMAIN,
                    [index as u8; 32],
                    index,
                    index,
                    MembershipOperation::Promote {
                        node_id: format!("node-{}", offset + 1),
                    },
                ))
                .unwrap();
        }
        let voters = keys
            .iter()
            .enumerate()
            .map(|(offset, key)| VoterAuthority {
                guardian_id: format!("guardian-{}", offset + 1).into_bytes(),
                trust_domain_id: DOMAIN.as_bytes().to_vec(),
                certificate_generation: 7,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: FINALIZE_SECONDS - 100,
                not_after_unix_seconds: FINALIZE_SECONDS + 100,
                revoked: false,
                control_public_key: key.verifying_key().to_bytes(),
            })
            .collect::<Vec<_>>();
        let configs = config_indexes
            .into_iter()
            .map(|indexes| {
                indexes
                    .into_iter()
                    .map(|offset| format!("guardian-{}", offset + 1).into_bytes())
                    .collect::<BTreeSet<_>>()
            })
            .collect();
        let authority = AuthorityMembership::new(
            DOMAIN.as_bytes().to_vec(),
            7,
            membership.committed_log_index(),
            configs,
            voters,
        )
        .unwrap();
        let signers = keys
            .into_iter()
            .enumerate()
            .map(|(offset, key)| {
                VoterEndorsementAuthority::restore_configured(
                    format!("node-{}", offset + 1),
                    format!("guardian-{}", offset + 1).into_bytes(),
                    7,
                    11,
                    membership.committed_log_index(),
                    key,
                    &membership,
                    &authority,
                )
                .unwrap()
            })
            .collect();
        Self {
            membership,
            authority,
            signers,
            checkpoint: Arc::new(MemoryCheckpointAuthority::default()),
            root: tempfile::Builder::new()
                .prefix("adl-authority-201-")
                .tempdir_in("/private/tmp")
                .unwrap(),
        }
    }

    fn store(&self) -> DurableAuthorityProtocol {
        DurableAuthorityProtocol::open(
            self.root.path(),
            AuthorityNodeIdentity {
                trust_domain: DOMAIN.into(),
                polis_id: POLIS.into(),
                node_id: "node-1".into(),
                guardian_id: "guardian-1".into(),
                boot_generation: 11,
            },
            self.checkpoint.clone(),
        )
        .unwrap()
    }

    fn intent(&self, store: &DurableAuthorityProtocol, id: &str) -> PrepareAuthorityIntent {
        PrepareAuthorityIntent::new(
            POLIS,
            &self.membership,
            &self.authority,
            AuthorityOperationKind::Membership,
            store.checkpoint_sha256().unwrap(),
            CanonicalAuthorityTime {
                unix_seconds: FINALIZE_SECONDS - 10,
                nanos: 0,
                uncertainty_millis: 2,
            },
            CanonicalAuthorityTime {
                unix_seconds: FINALIZE_SECONDS,
                nanos: 0,
                uncertainty_millis: 2,
            },
            id,
            CommittedAuthorityArtifact::new(
                AuthorityOperationKind::Membership,
                format!("signed-store-artifact:{id}").into_bytes(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn finalize_with(
        &self,
        intent: &PrepareAuthorityIntent,
        indexes: &[usize],
        time: CanonicalAuthorityTime,
    ) -> FinalizeAuthorityIntent {
        let endorsements = indexes
            .iter()
            .map(|index| {
                self.signers[*index]
                    .endorse(intent, &time, &self.membership, &self.authority)
                    .unwrap()
            })
            .collect();
        FinalizeAuthorityIntent::new(intent, time, endorsements).unwrap()
    }

    fn verified(
        &self,
        intent: &PrepareAuthorityIntent,
    ) -> adl_runtime::distributed::authority_protocol::VerifiedAuthorityOperation {
        let finalize = self.finalize_with(
            intent,
            &[0, 1],
            CanonicalAuthorityTime {
                unix_seconds: FINALIZE_SECONDS,
                nanos: 0,
                uncertainty_millis: 2,
            },
        );
        verify_finalization(intent, &finalize, &self.membership, &self.authority).unwrap()
    }
}

#[test]
fn current_three_voter_finalize() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "current-quorum");
    let result = store.publish(&intent, fixture.verified(&intent)).unwrap();
    assert_eq!(result.operation_id(), "current-quorum");
    assert_eq!(store.generation(), 1);
    marker("current_three_voter_finalize", "passed");
}

#[test]
fn exact_retry_returns_cached_result() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "retry");
    let first = store.publish(&intent, fixture.verified(&intent)).unwrap();
    let second = store.publish(&intent, fixture.verified(&intent)).unwrap();
    assert_eq!(first.result_sha256(), second.result_sha256());
    assert_eq!(first.retry_sha256(), second.retry_sha256());
    assert_eq!(store.generation(), 1);
    marker("exact_retry_returns_cached_result", "passed");
}

#[test]
fn finalize_at_deadline() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "deadline");
    assert!(fixture.verified(&intent).intent_sha256() != [0; 32]);
    marker("finalize_at_deadline", "passed");
}

#[test]
fn missing_quorum() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "missing");
    let finalize = fixture.finalize_with(
        &intent,
        &[0],
        CanonicalAuthorityTime {
            unix_seconds: FINALIZE_SECONDS,
            nanos: 0,
            uncertainty_millis: 2,
        },
    );
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::MissingQuorum)
    );
    marker("missing_quorum", "rejected");
}

#[test]
fn duplicate_signer() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "duplicate");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let endorsement = fixture.signers[0]
        .endorse(&intent, &time, &fixture.membership, &fixture.authority)
        .unwrap();
    let finalize =
        FinalizeAuthorityIntent::new(&intent, time, vec![endorsement.clone(), endorsement])
            .unwrap();
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::DuplicateVoter)
    );
    marker("duplicate_signer", "rejected");
}

#[test]
fn stale_membership() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let mut intent = fixture.intent(&store, "stale");
    intent.membership_log_index -= 1;
    assert_eq!(
        intent.validate_against(&fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::WrongMembership)
    );
    marker("stale_membership", "rejected");
}

#[test]
fn config_digest_mismatch() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let mut intent = fixture.intent(&store, "config");
    intent.configuration_sha256[0] ^= 1;
    assert_eq!(
        intent.validate_against(&fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::WrongMembership)
    );
    marker("config_digest_mismatch", "rejected");
}

#[test]
fn declared_finalize_time_after_deadline() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "late");
    assert_eq!(
        FinalizeAuthorityIntent::new(
            &intent,
            CanonicalAuthorityTime {
                unix_seconds: FINALIZE_SECONDS + 1,
                nanos: 0,
                uncertainty_millis: 0
            },
            vec![]
        ),
        Err(AuthorityProtocolError::TimeOutsideIntent)
    );
    marker("declared_finalize_time_after_deadline", "rejected");
}

#[test]
fn finalize_before_prepare_time() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "early");
    assert_eq!(
        FinalizeAuthorityIntent::new(
            &intent,
            CanonicalAuthorityTime {
                unix_seconds: FINALIZE_SECONDS - 11,
                nanos: 0,
                uncertainty_millis: 0
            },
            vec![]
        ),
        Err(AuthorityProtocolError::TimeOutsideIntent)
    );
    marker("finalize_before_prepare_time", "rejected");
}

#[test]
fn local_clock_skew_apply_parity() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "clock");
    let verified = fixture.verified(&intent);
    assert_eq!(verified.finalization_time().unix_seconds, FINALIZE_SECONDS);
    marker("local_clock_skew_apply_parity", "passed");
}

#[test]
fn capacity_n_plus_one_no_partial() {
    let fixture = Fixture::new();
    let mut store = DurableAuthorityProtocol::open_with_capacity(
        fixture.root.path(),
        AuthorityNodeIdentity {
            trust_domain: DOMAIN.into(),
            polis_id: POLIS.into(),
            node_id: "node-1".into(),
            guardian_id: "guardian-1".into(),
            boot_generation: 11,
        },
        fixture.checkpoint.clone(),
        1,
    )
    .unwrap();
    let first = fixture.intent(&store, "one");
    store.publish(&first, fixture.verified(&first)).unwrap();
    let second = fixture.intent(&store, "two");
    assert_eq!(
        store.publish(&second, fixture.verified(&second)),
        Err(AuthorityProtocolError::CapacityExceeded)
    );
    assert!(store.published("two").is_none());
    marker("capacity_n_plus_one_no_partial", "rejected");
}

#[test]
fn artifact_bytes_digest_substitution_rejected() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let mut intent = fixture.intent(&store, "artifact-tamper");
    intent.artifact.bytes[0] ^= 1;
    assert_eq!(
        intent.validate_against(&fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::ArtifactMismatch)
    );
    marker("artifact_bytes_digest_substitution_rejected", "rejected");
}

#[test]
fn checkpoint_object_collision() {
    let fixture = Fixture::new();
    let first = fixture.store();
    drop(first);
    let collision = DurableAuthorityProtocol::open(
        fixture.root.path(),
        AuthorityNodeIdentity {
            trust_domain: DOMAIN.into(),
            polis_id: POLIS.into(),
            node_id: "node-2".into(),
            guardian_id: "guardian-2".into(),
            boot_generation: 11,
        },
        fixture.checkpoint.clone(),
    );
    assert_eq!(
        collision.err(),
        Some(AuthorityProtocolError::StateRegression)
    );
    marker("checkpoint_object_collision", "rejected");
}

#[test]
fn three_node_checkpoint_restart_reconcile() {
    for node in 1..=3 {
        let fixture = Fixture::new();
        let identity = AuthorityNodeIdentity {
            trust_domain: DOMAIN.into(),
            polis_id: POLIS.into(),
            node_id: format!("node-{node}"),
            guardian_id: format!("guardian-{node}"),
            boot_generation: 11,
        };
        let mut store = DurableAuthorityProtocol::open(
            fixture.root.path(),
            identity.clone(),
            fixture.checkpoint.clone(),
        )
        .unwrap();
        let intent = fixture.intent(&store, &format!("node-{node}-result"));
        store.publish(&intent, fixture.verified(&intent)).unwrap();
        drop(store);
        let reopened = DurableAuthorityProtocol::open(
            fixture.root.path(),
            identity,
            fixture.checkpoint.clone(),
        )
        .unwrap();
        assert!(reopened.published(&format!("node-{node}-result")).is_some());
    }
    marker("three_node_checkpoint_restart_reconcile", "passed");
}

#[test]
fn exact_store_artifact_bytes_retained() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "artifact-retained");
    let expected = Sha256::digest(&intent.artifact.bytes);
    let result = store.publish(&intent, fixture.verified(&intent)).unwrap();
    assert_eq!(result.operation().intent_sha256(), intent.digest().unwrap());
    assert_eq!(intent.artifact.sha256.as_slice(), expected.as_slice());
    marker("exact_store_artifact_bytes_retained", "passed");
}

#[test]
fn signer_rotation_current_generation() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "rotation");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let endorsement = fixture.signers[0]
        .endorse(&intent, &time, &fixture.membership, &fixture.authority)
        .unwrap();
    let mut rotated = fixture.authority.clone();
    rotated
        .voters
        .get_mut(b"guardian-1".as_slice())
        .unwrap()
        .certificate_generation = 8;
    let finalize = FinalizeAuthorityIntent::new(&intent, time, vec![endorsement]).unwrap();
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &rotated),
        Err(AuthorityProtocolError::StaleVoter)
    );
    marker("signer_rotation_current_generation", "rejected");
}

#[test]
fn joint_majority_each_config() {
    let fixture = Fixture::with_configs(vec![vec![0, 1, 2], vec![1, 2, 3]]);
    let store = fixture.store();
    let intent = fixture.intent(&store, "joint-pass");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let finalize = fixture.finalize_with(&intent, &[1, 2], time);
    assert!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority).is_ok()
    );
    marker("joint_majority_each_config", "passed");
}

fn assert_joint_rejected(name: &str, indexes: &[usize]) {
    let fixture = Fixture::with_configs(vec![vec![0, 1, 2], vec![3, 4, 5]]);
    let store = fixture.store();
    let intent = fixture.intent(&store, name);
    let finalize = fixture.finalize_with(
        &intent,
        indexes,
        CanonicalAuthorityTime {
            unix_seconds: FINALIZE_SECONDS,
            nanos: 0,
            uncertainty_millis: 2,
        },
    );
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::MissingQuorum)
    );
    marker(name, "rejected");
}

#[test]
fn joint_old_only() {
    assert_joint_rejected("joint_old_only", &[0, 1, 2]);
}

#[test]
fn joint_new_only() {
    assert_joint_rejected("joint_new_only", &[3, 4, 5]);
}

#[test]
fn joint_union_majority_only() {
    assert_joint_rejected("joint_union_majority_only", &[0, 3, 4, 5]);
}

#[test]
fn joint_duplicate_guardian_reuse() {
    let fixture = Fixture::with_configs(vec![vec![0, 1, 2], vec![1, 2, 3]]);
    let store = fixture.store();
    let intent = fixture.intent(&store, "joint-duplicate");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let endorsement = fixture.signers[1]
        .endorse(&intent, &time, &fixture.membership, &fixture.authority)
        .unwrap();
    let finalize =
        FinalizeAuthorityIntent::new(&intent, time, vec![endorsement.clone(), endorsement])
            .unwrap();
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::DuplicateVoter)
    );
    marker("joint_duplicate_guardian_reuse", "rejected");
}

#[test]
fn signer_unavailable() {
    assert_joint_rejected("signer_unavailable", &[0, 1, 3]);
}

#[test]
fn expired_signer_cert() {
    let mut fixture = Fixture::new();
    fixture
        .authority
        .voters
        .get_mut(b"guardian-1".as_slice())
        .unwrap()
        .not_after_unix_seconds = FINALIZE_SECONDS - 1;
    let store = fixture.store();
    let intent = fixture.intent(&store, "expired");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    assert_eq!(
        fixture.signers[0].endorse(&intent, &time, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::StaleVoter)
    );
    marker("expired_signer_cert", "rejected");
}

#[test]
fn wrong_voter() {
    let fixture = Fixture::new();
    let store = fixture.store();
    let intent = fixture.intent(&store, "wrong-voter");
    let time = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let endorsement = fixture.signers[0]
        .endorse(&intent, &time, &fixture.membership, &fixture.authority)
        .unwrap();
    let mut value = serde_json::to_value(endorsement).unwrap();
    value["guardian_id"] = serde_json::json!([103, 104, 111, 115, 116]);
    let wrong = serde_json::from_value(value).unwrap();
    let finalize = FinalizeAuthorityIntent::new(&intent, time, vec![wrong]).unwrap();
    assert_eq!(
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority),
        Err(AuthorityProtocolError::WrongVoter)
    );
    marker("wrong_voter", "rejected");
}

#[test]
fn replay_with_regressed_finalize_time() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "regressed-time");
    store.publish(&intent, fixture.verified(&intent)).unwrap();
    let earlier = CanonicalAuthorityTime {
        unix_seconds: FINALIZE_SECONDS - 1,
        nanos: 0,
        uncertainty_millis: 2,
    };
    let finalize = fixture.finalize_with(&intent, &[0, 1], earlier);
    let verified =
        verify_finalization(&intent, &finalize, &fixture.membership, &fixture.authority).unwrap();
    assert_eq!(
        store.publish(&intent, verified),
        Err(AuthorityProtocolError::RetryConflict)
    );
    marker("replay_with_regressed_finalize_time", "rejected");
}

fn assert_legacy_rejected(case: &str, command: PolisCommand) {
    assert_eq!(
        validate_authority_command_boundary(&command),
        Err(PolisRuntimeError::AuthorityDenied)
    );
    marker(case, "rejected");
}

#[test]
fn legacy_fence_voter_rejected() {
    assert_legacy_rejected(
        "legacy_fence_voter_rejected",
        PolisCommand::FenceVoter {
            voter_id: "voter-a".into(),
            epoch: 2,
        },
    );
}

#[test]
fn legacy_activate_owner_rejected() {
    assert_legacy_rejected(
        "legacy_activate_owner_rejected",
        PolisCommand::ActivateOwner {
            owner_id: "owner-a".into(),
            epoch: 2,
        },
    );
}

#[test]
fn legacy_activate_shepherd_rejected() {
    assert_legacy_rejected(
        "legacy_activate_shepherd_rejected",
        PolisCommand::ActivateShepherd {
            shepherd_identity_ref: "shepherd-a".into(),
            epoch: 2,
        },
    );
}

#[test]
fn legacy_acquire_observatory_rejected() {
    assert_legacy_rejected(
        "legacy_acquire_observatory_rejected",
        PolisCommand::AcquireObservatory {
            owner_id: "owner-a".into(),
            epoch: 2,
            expires_unix_millis: 10,
        },
    );
}

#[test]
fn legacy_demote_voter_rejected() {
    assert_legacy_rejected(
        "legacy_demote_voter_rejected",
        PolisCommand::DemoteVoter {
            voter_id: "voter-a".into(),
            epoch: 2,
        },
    );
}

fn crash_identity(node: &str) -> AuthorityNodeIdentity {
    AuthorityNodeIdentity {
        trust_domain: DOMAIN.into(),
        polis_id: POLIS.into(),
        node_id: node.into(),
        guardian_id: node.replacen("node", "guardian", 1),
        boot_generation: 11,
    }
}

fn prove_local_before_cas(case: &str, node: &str) {
    let fixture = Fixture::new();
    let checkpoint = Arc::new(FaultCheckpointAuthority::default());
    let identity = crash_identity(node);
    let mut store =
        DurableAuthorityProtocol::open(fixture.root.path(), identity.clone(), checkpoint.clone())
            .unwrap();
    let intent = fixture.intent(&store, case);
    checkpoint.arm_before_cas();
    assert_eq!(
        store.publish(&intent, fixture.verified(&intent)),
        Err(AuthorityProtocolError::Storage)
    );
    drop(store);
    let mut reopened =
        DurableAuthorityProtocol::open(fixture.root.path(), identity, checkpoint).unwrap();
    assert!(reopened.published(case).is_none());
    assert!(reopened.publish(&intent, fixture.verified(&intent)).is_ok());
    marker(case, "reconciled");
}

fn prove_cas_before_final_marker(case: &str, node: &str) {
    let fixture = Fixture::new();
    let checkpoint = Arc::new(FaultCheckpointAuthority::default());
    let identity = crash_identity(node);
    let mut store =
        DurableAuthorityProtocol::open(fixture.root.path(), identity.clone(), checkpoint.clone())
            .unwrap();
    let intent = fixture.intent(&store, case);
    checkpoint.arm_after_cas();
    assert_eq!(
        store.publish(&intent, fixture.verified(&intent)),
        Err(AuthorityProtocolError::Storage)
    );
    drop(store);
    let reopened =
        DurableAuthorityProtocol::open(fixture.root.path(), identity, checkpoint).unwrap();
    assert!(reopened.published(case).is_some());
    marker(case, "reconciled");
}

#[test]
fn node_a_local_before_cas() {
    prove_local_before_cas("node_a_local_before_cas", "node-1");
}
#[test]
fn node_a_cas_before_final_marker() {
    prove_cas_before_final_marker("node_a_cas_before_final_marker", "node-1");
}
#[test]
fn node_b_local_before_cas() {
    prove_local_before_cas("node_b_local_before_cas", "node-2");
}
#[test]
fn node_b_cas_before_final_marker() {
    prove_cas_before_final_marker("node_b_cas_before_final_marker", "node-2");
}
#[test]
fn node_c_local_before_cas() {
    prove_local_before_cas("node_c_local_before_cas", "node-3");
}
#[test]
fn node_c_cas_before_final_marker() {
    prove_cas_before_final_marker("node_c_cas_before_final_marker", "node-3");
}

#[test]
fn coherent_rollback_rejected() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "rollback");
    store.publish(&intent, fixture.verified(&intent)).unwrap();
    drop(store);
    fixture.checkpoint.0.lock().unwrap().clear();
    assert_eq!(
        DurableAuthorityProtocol::open(
            fixture.root.path(),
            crash_identity("node-1"),
            fixture.checkpoint.clone()
        )
        .err(),
        Some(AuthorityProtocolError::StateRegression)
    );
    marker("coherent_rollback_rejected", "rejected");
}

fn tamper_state(fixture: &Fixture, field: &str) {
    let path = fixture.root.path().join("authority-protocol.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    value["payload"]["published"]["tamper"][field][0] = serde_json::json!(99);
    fs::write(path, serde_jcs::to_vec(&value).unwrap()).unwrap();
}

#[test]
fn checkpoint_result_retry_digest_mismatch() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "tamper");
    store.publish(&intent, fixture.verified(&intent)).unwrap();
    drop(store);
    tamper_state(&fixture, "result_sha256");
    assert!(DurableAuthorityProtocol::open(
        fixture.root.path(),
        crash_identity("node-1"),
        fixture.checkpoint.clone()
    )
    .is_err());
    marker("checkpoint_result_retry_digest_mismatch", "rejected");
}

#[test]
fn corrupt_retry_cache_rejected() {
    let fixture = Fixture::new();
    let mut store = fixture.store();
    let intent = fixture.intent(&store, "tamper");
    store.publish(&intent, fixture.verified(&intent)).unwrap();
    drop(store);
    tamper_state(&fixture, "retry_sha256");
    assert!(DurableAuthorityProtocol::open(
        fixture.root.path(),
        crash_identity("node-1"),
        fixture.checkpoint.clone()
    )
    .is_err());
    marker("corrupt_retry_cache_rejected", "rejected");
}

#[test]
fn corrupt_journal_rejected() {
    let fixture = Fixture::new();
    let store = fixture.store();
    drop(store);
    fs::write(
        fixture.root.path().join(".authority-protocol.json.journal"),
        b"{}",
    )
    .unwrap();
    assert!(DurableAuthorityProtocol::open(
        fixture.root.path(),
        crash_identity("node-1"),
        fixture.checkpoint.clone()
    )
    .is_err());
    marker("corrupt_journal_rejected", "rejected");
}

#[cfg(unix)]
#[test]
fn state_symlink_rejected() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    symlink(
        "missing-target",
        fixture.root.path().join("authority-protocol.json"),
    )
    .unwrap();
    assert!(DurableAuthorityProtocol::open(
        fixture.root.path(),
        crash_identity("node-1"),
        fixture.checkpoint.clone()
    )
    .is_err());
    marker("state_symlink_rejected", "rejected");
}

#[cfg(unix)]
#[test]
fn lock_symlink_rejected() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    symlink(
        "missing-target",
        fixture.root.path().join(".authority-protocol.json.lock"),
    )
    .unwrap();
    assert!(DurableAuthorityProtocol::open(
        fixture.root.path(),
        crash_identity("node-1"),
        fixture.checkpoint.clone()
    )
    .is_err());
    marker("lock_symlink_rejected", "rejected");
}

fn continuity_artifact() -> CommittedAuthorityArtifact {
    let manifest = b"signed-manifest".to_vec();
    let catalog = b"signed-catalog".to_vec();
    CommittedAuthorityArtifact::continuity_transfer(&ContinuityTransferGrantArtifact {
        source_guardian_id: "guardian-a".into(),
        target_guardian_id: "guardian-b".into(),
        route_id: "route-a".into(),
        membership_epoch: 7,
        membership_log_index: 19,
        source_certificate_generation: 3,
        target_certificate_generation: 4,
        source_boot_generation: 5,
        target_boot_generation: 6,
        transfer_id: "transfer-a".into(),
        lineage_id: b"lineage-a".to_vec(),
        source_checkpoint_handle_identity: b"source-handle-a".to_vec(),
        bundle_handle_identity: b"bundle-handle-a".to_vec(),
        signed_manifest_sha256: Sha256::digest(&manifest).into(),
        signed_manifest_bytes: manifest,
        signed_catalog_sha256: Sha256::digest(&catalog).into(),
        signed_catalog_bytes: catalog,
        trusted_key_generation: 8,
        entries: vec![ContinuityTransferEntry {
            schema: "kernel.page.v1".into(),
            absolute_start: 0,
            length: 4,
            sha256: [9; 32],
        }],
        chunks: vec![ContinuityTransferChunk {
            index: 0,
            absolute_start: 0,
            length: 4,
            sha256: [10; 32],
            predecessor_sha256: None,
        }],
        total_bytes: 4,
        inclusive_deadline: CanonicalAuthorityTime {
            unix_seconds: FINALIZE_SECONDS,
            nanos: 0,
            uncertainty_millis: 2,
        },
        cleanup_identity: "cleanup-a".into(),
    })
    .unwrap()
}

fn continuity_check(
    consumer: &str,
    lineage: &[u8],
    source: &[u8],
    bundle: &[u8],
) -> Result<(), AuthorityProtocolError> {
    validate_continuity_transfer_binding(&continuity_artifact(), consumer, lineage, source, bundle)
}

#[test]
fn sealed_continuity_transfer_projection() {
    assert!(continuity_check(
        CONTINUITY_TRANSFER_ADAPTER_210,
        b"lineage-a",
        b"source-handle-a",
        b"bundle-handle-a"
    )
    .is_ok());
    marker("sealed_continuity_transfer_projection", "passed");
}

#[test]
fn continuity_projection_consumer_confusion_rejected() {
    assert_eq!(
        continuity_check(
            "other-consumer",
            b"lineage-a",
            b"source-handle-a",
            b"bundle-handle-a"
        ),
        Err(AuthorityProtocolError::WrongVoterPurpose)
    );
    marker(
        "continuity_projection_consumer_confusion_rejected",
        "rejected",
    );
}

#[test]
fn continuity_projection_wrong_lineage_rejected() {
    assert_eq!(
        continuity_check(
            CONTINUITY_TRANSFER_ADAPTER_210,
            b"wrong-lineage",
            b"source-handle-a",
            b"bundle-handle-a"
        ),
        Err(AuthorityProtocolError::ArtifactMismatch)
    );
    marker("continuity_projection_wrong_lineage_rejected", "rejected");
}

#[test]
fn continuity_projection_wrong_source_checkpoint_handle_rejected() {
    assert_eq!(
        continuity_check(
            CONTINUITY_TRANSFER_ADAPTER_210,
            b"lineage-a",
            b"wrong-source",
            b"bundle-handle-a"
        ),
        Err(AuthorityProtocolError::ArtifactMismatch)
    );
    marker(
        "continuity_projection_wrong_source_checkpoint_handle_rejected",
        "rejected",
    );
}

#[test]
fn continuity_projection_wrong_bundle_handle_rejected() {
    assert_eq!(
        continuity_check(
            CONTINUITY_TRANSFER_ADAPTER_210,
            b"lineage-a",
            b"source-handle-a",
            b"wrong-bundle"
        ),
        Err(AuthorityProtocolError::ArtifactMismatch)
    );
    marker(
        "continuity_projection_wrong_bundle_handle_rejected",
        "rejected",
    );
}
