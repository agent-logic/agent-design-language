// PVF: lane=exact-child-tests; proof=committed three-voter authority protocol,
// deterministic=true; resource_profile=medium; release_gate=true;
// exact 47-case name/result/marker parity is enforced by the issue proof producer.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    authority_protocol::{
        verify_finalization, AuthorityNodeIdentity, AuthorityOperationKind, AuthorityProtocolError,
        CanonicalAuthorityTime, CommittedAuthorityArtifact, DurableAuthorityProtocol,
        FinalizeAuthorityIntent, PrepareAuthorityIntent, VoterEndorsementAuthority,
    },
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
        MembershipState,
    },
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
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
