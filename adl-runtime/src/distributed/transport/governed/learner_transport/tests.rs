use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    net::Ipv4Addr,
    sync::{Arc, Mutex, RwLock as StdRwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::*;
use crate::distributed::{
    authority_protocol::{
        test_published_reconciliation_token, AuthorityNodeIdentity, CanonicalAuthorityTime,
        PrepareAuthorityIntent,
    },
    certificates::{
        AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
        CertificateValidity, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS,
    },
    identity::LocalNodeGuardianIdentity,
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
        MembershipState,
    },
    membership_coordinator::{
        membership_set_sha256, stable_map_sha256, verify_authorized_transition_inputs,
        AuthorizedMembershipTransition, GovernedMembershipRuntime, MembershipCoordinator,
        MembershipCoordinatorError, MembershipCoordinatorPhase, MembershipCrashBoundary,
        PromoteVoterArtifact, VerifiedPromoteVoter,
    },
    polis_runtime::{
        serve_authorized_learner_connection, ConsensusCheckpoint, ConsensusCheckpointAuthority,
        PolisCommand, PolisLogStore, PolisRaft, PolisRuntimeError, PolisStateMachineStore,
        PolisTypeConfig, SecureBootGenerationAuthority, SecureLearnerNetworkFactory,
        SecurePolisNetworkConnection, SecurePolisNetworkFactory,
    },
    transport::{
        client_endpoint, server_endpoint, AuthenticatedConnection, ConnectionSecurity, PeerBinding,
        TransportAuthorization, TransportError, TransportLimits, VerifiedPolisRouteCut,
    },
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use openraft::{
    error::{InstallSnapshotError, NetworkError, RPCError, RaftError, RemoteError},
    network::{RPCOption, RaftNetwork, RaftNetworkFactory},
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    BasicNode,
};
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ED25519,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::sync::CancellationToken;

const NOW: i64 = 1_000;
const CUT: [u8; 32] = [7; 32];
const MEMBERSHIP: [u8; 32] = [8; 32];

fn mark(name: &str) {
    eprintln!("ADL_ISSUE_202_CASE_V1 {name}=passed");
}

fn assertion(case_name: &str, assertion_name: &str) {
    eprintln!("ADL_ISSUE_202_ASSERTION_V1 {case_name} {assertion_name}");
}

fn identity() -> LearnerIdentity {
    LearnerIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-4".to_owned(),
        guardian_id: "guardian-4".to_owned(),
        guardian_control_public_key: SigningKey::from_bytes(&[44; 32]).verifying_key().to_bytes(),
        stable_raft_id: 4,
        certificate_generation: 4,
        boot_generation: 9,
        address: "127.0.0.1:4404".parse().unwrap(),
    }
}

fn authority_identity() -> AuthorityNodeIdentity {
    AuthorityNodeIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-1".to_owned(),
        guardian_id: "guardian-1".to_owned(),
        boot_generation: 3,
    }
}

fn endorsement_fixture() -> (
    MembershipState,
    AuthorityMembership,
    BTreeMap<Vec<u8>, u64>,
    Vec<LocalNodeGuardianIdentity>,
) {
    let signers = (0..3)
        .map(|_| LocalNodeGuardianIdentity::generate("runtime-prod", 4).unwrap())
        .collect::<Vec<_>>();
    let mut membership =
        MembershipState::new(MembershipPolicy::new("runtime-prod", 8, 16).unwrap());
    let mut index = 0_u64;
    for signer in &signers {
        let public = signer.public_identity();
        index += 1;
        membership
            .apply(&CommittedMembershipEvent::new(
                "runtime-prod",
                [index as u8; 32],
                index,
                index,
                MembershipOperation::Join {
                    member: Member {
                        node_id: public.node_id.clone(),
                        guardian_id: public.guardian_id.clone(),
                        identity_generation: public.identity_generation,
                        guardian_control_public_key: public.guardian_control_public_key,
                        role: MemberRole::NonVoting,
                    },
                },
            ))
            .unwrap();
    }
    for signer in &signers {
        index += 1;
        membership
            .apply(&CommittedMembershipEvent::new(
                "runtime-prod",
                [index as u8; 32],
                index,
                index,
                MembershipOperation::Promote {
                    node_id: signer.public_identity().node_id.clone(),
                },
            ))
            .unwrap();
    }
    let guardians = signers
        .iter()
        .map(|signer| signer.public_identity().guardian_id.as_bytes().to_vec())
        .collect::<BTreeSet<_>>();
    let voters = signers
        .iter()
        .map(|signer| VoterAuthority {
            guardian_id: signer.public_identity().guardian_id.as_bytes().to_vec(),
            trust_domain_id: b"runtime-prod".to_vec(),
            certificate_generation: 4,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: NOW - 100,
            not_after_unix_seconds: NOW + 100,
            revoked: false,
            control_public_key: signer.public_identity().guardian_control_public_key,
        })
        .collect();
    let authority = AuthorityMembership::new(
        b"runtime-prod".to_vec(),
        4,
        membership.committed_log_index(),
        vec![guardians],
        voters,
    )
    .unwrap();
    let boots = authority
        .voters
        .keys()
        .map(|guardian| (guardian.clone(), 4))
        .collect();
    (membership, authority, boots, signers)
}

fn enroll_token_with(
    identity: LearnerIdentity,
    operation: &str,
    index: u64,
    deadline: i64,
    previous: Option<[u8; 32]>,
) -> PublishedAuthorityResult {
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        identity,
        CUT,
        previous,
        MEMBERSHIP,
        Some(deadline - 10),
        deadline,
    )
    .unwrap();
    test_published_reconciliation_token(
        authority_identity(),
        operation,
        artifact,
        index,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    )
}

fn enroll_token() -> PublishedAuthorityResult {
    enroll_token_with(identity(), "enroll-4", 41, NOW + 100, None)
}

fn remove_token_with(
    identity: LearnerIdentity,
    operation: &str,
    index: u64,
) -> PublishedAuthorityResult {
    let artifact = LearnerMembershipArtifact::remove_voter(
        identity,
        CUT,
        MEMBERSHIP,
        NOW + 100,
        "operator_remove",
    )
    .unwrap();
    test_published_reconciliation_token(
        authority_identity(),
        operation,
        artifact,
        index,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    )
}

fn admission() -> VerifiedLearnerAdmission {
    VerifiedLearnerAdmission::from_published_membership_for_test(
        &enroll_token(),
        &identity(),
        CUT,
        NOW,
    )
    .unwrap()
}

fn session() -> EstablishedLearnerSession {
    let directory = portable_tempdir();
    let authority =
        ProductionLearnerAuthority::open(directory.path(), Arc::new(MemoryCheckpoint::default()))
            .unwrap();
    let admission = admission();
    authority.activate_admission(&admission).unwrap();
    EstablishedLearnerSession::new(
        &admission,
        CUT,
        LearnerVoterBinding {
            stable_raft_id: 1,
            node_id: "node-1".to_owned(),
            guardian_id: "guardian-1".to_owned(),
            certificate_generation: 4,
            boot_generation: 3,
            control_public_key: SigningKey::from_bytes(&[1; 32]).verifying_key().to_bytes(),
        },
        LearnerEndpointRole::Voter,
        authority,
        NOW,
    )
    .unwrap()
}

#[derive(Default)]
struct MemoryCheckpoint {
    values: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
    fail_next: Mutex<bool>,
}

impl ConsensusCheckpointAuthority for MemoryCheckpoint {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.values.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        if std::mem::take(&mut *self.fail_next.lock().unwrap()) {
            return Err(PolisRuntimeError::StateRegression);
        }
        let mut values = self.values.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn portable_tempdir() -> tempfile::TempDir {
    tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap()
}

struct EndpointMaterial {
    certificate: CertificateDer<'static>,
    private_key_der: Vec<u8>,
    subject_public_key: VerifyingKey,
}

impl EndpointMaterial {
    fn private_key(&self) -> PrivateKeyDer<'static> {
        PrivatePkcs8KeyDer::from(self.private_key_der.clone()).into()
    }
}

fn certificate_authority() -> CertifiedIssuer<'static, KeyPair> {
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::CrlSign,
    ];
    CertifiedIssuer::self_signed(params, KeyPair::generate().unwrap()).unwrap()
}

fn leaf(issuer: &CertifiedIssuer<'_, KeyPair>, name: &str) -> EndpointMaterial {
    let mut params = CertificateParams::new(vec!["localhost".to_owned()]).unwrap();
    params
        .distinguished_name
        .push(rcgen::DnType::CommonName, name);
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    params.extended_key_usages = vec![
        ExtendedKeyUsagePurpose::ServerAuth,
        ExtendedKeyUsagePurpose::ClientAuth,
    ];
    let key = KeyPair::generate_for(&PKCS_ED25519).unwrap();
    let subject_public_key =
        VerifyingKey::from_bytes(key.public_key_raw().try_into().unwrap()).unwrap();
    let certificate = params.signed_by(&key, issuer).unwrap().der().clone();
    EndpointMaterial {
        certificate,
        private_key_der: key.serialize_der(),
        subject_public_key,
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn transport_authorization(
    store: &Arc<DistributedCertificateStore>,
    root: &SigningKey,
    node: &str,
    key: VerifyingKey,
) -> TransportAuthorization {
    let issued = unix_now().saturating_sub(1);
    let body = CertificateBody::new(
        "runtime-prod",
        node,
        CertificatePurpose::Transport,
        4,
        CertificateValidity {
            issued_at_unix_secs: issued,
            expires_at_unix_secs: issued + 600,
        },
        key,
        &root.verifying_key(),
    );
    let certificate = AuthorityCertificate::issue(body, root).unwrap();
    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, unix_now())
        .unwrap();
    TransportAuthorization::new_for_test(Arc::clone(store), &certificate).unwrap()
}

async fn live_learner_pair(
    voter_node: u64,
) -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    live_learner_pair_for(voter_node, &identity()).await
}

async fn live_learner_pair_for(
    voter_node: u64,
    learner_identity: &LearnerIdentity,
) -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    let issuer = certificate_authority();
    let root_certificate = issuer.der().clone();
    let voter_node_id = format!("node-{voter_node}");
    let voter_guardian_id = format!("guardian-{voter_node}");
    let voter = leaf(&issuer, &voter_node_id);
    let learner = leaf(&issuer, &learner_identity.node_id);
    let signing_root = SigningKey::from_bytes(&[91; 32]);
    let policy = CertificatePolicy::new("runtime-prod", [signing_root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 128, 128)
        .unwrap();
    let store_dir = portable_tempdir();
    let store = Arc::new(
        DistributedCertificateStore::open(
            &TEST_CERTIFICATE_STORE_ACCESS,
            store_dir.path().join("certificates.redb"),
            policy,
        )
        .unwrap(),
    );
    let voter_authorization = transport_authorization(
        &store,
        &signing_root,
        &voter_node_id,
        voter.subject_public_key,
    );
    let learner_authorization = transport_authorization(
        &store,
        &signing_root,
        &learner_identity.node_id,
        learner.subject_public_key,
    );
    let voter_binding = PeerBinding::new(
        &voter.certificate,
        "runtime-prod",
        voter_node_id,
        voter_guardian_id,
        1,
        4,
    )
    .unwrap();
    let learner_binding = PeerBinding::new(
        &learner.certificate,
        "runtime-prod",
        learner_identity.node_id.clone(),
        learner_identity.guardian_id.clone(),
        1,
        4,
    )
    .unwrap();
    let limits = TransportLimits::bounded(
        256 * 1024,
        32,
        Duration::from_secs(10),
        Duration::from_secs(30),
    )
    .unwrap();
    let learner_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![learner.certificate.clone()],
        learner.private_key(),
        std::slice::from_ref(&root_certificate),
        &limits,
    )
    .unwrap();
    let voter_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![voter.certificate.clone()],
        voter.private_key(),
        std::slice::from_ref(&root_certificate),
        &limits,
    )
    .unwrap();
    let learner_address = learner_endpoint.local_addr().unwrap();
    let (voter_connection, learner_connection) = tokio::join!(
        AuthenticatedConnection::connect(
            &voter_endpoint,
            learner_address,
            "localhost",
            ConnectionSecurity::new(
                voter_binding.clone(),
                learner_binding.clone(),
                voter_authorization.clone(),
                learner_authorization.clone(),
                limits.clone(),
                CancellationToken::new(),
            )
            .unwrap(),
        ),
        AuthenticatedConnection::accept(
            &learner_endpoint,
            ConnectionSecurity::new(
                learner_binding,
                voter_binding,
                learner_authorization,
                voter_authorization,
                limits,
                CancellationToken::new(),
            )
            .unwrap(),
        )
    );
    (
        Arc::new(voter_connection.unwrap()),
        Arc::new(learner_connection.unwrap()),
        voter_endpoint,
        learner_endpoint,
        store_dir,
    )
}

fn live_voter_cut(boot_generations: [u64; 3]) -> VerifiedPolisRouteCut {
    live_voter_cut_for("polis-a", boot_generations)
}

fn live_voter_cut_for(polis_id: &str, boot_generations: [u64; 3]) -> VerifiedPolisRouteCut {
    let routes = (1..=3)
        .map(|node| {
            (
                node,
                format!("127.0.0.1:{}", 45_000 + node).parse().unwrap(),
            )
        })
        .collect();
    let identities = (1..=3)
        .map(|node| {
            (
                node,
                (
                    format!("node-{node}"),
                    format!("guardian-{node}"),
                    SigningKey::from_bytes(&[node as u8; 32])
                        .verifying_key()
                        .to_bytes(),
                    boot_generations[(node - 1) as usize],
                ),
            )
        })
        .collect();
    VerifiedPolisRouteCut::test_from_parts(polis_id, "runtime-prod", routes, identities)
}

fn exact_voter_target(cut: &VerifiedPolisRouteCut, raft_id: u64) -> LearnerIdentity {
    let (node_id, guardian_id, boot_generation) = cut.authority_node_identity(raft_id).unwrap();
    let voter = cut
        .authority_membership()
        .voters
        .get(guardian_id.as_bytes())
        .unwrap();
    LearnerIdentity {
        trust_domain: cut.trust_domain().to_owned(),
        polis_id: cut.polis_id().to_owned(),
        node_id,
        guardian_id,
        guardian_control_public_key: voter.control_public_key,
        stable_raft_id: raft_id,
        certificate_generation: voter.certificate_generation,
        boot_generation,
        address: cut.routes()[&raft_id],
    }
}

#[tokio::test]
async fn transport_instance_and_peer_pin_are_durable_and_unique() {
    let root = portable_tempdir();
    let alternate_root = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let authority = ProductionLearnerAuthority::open(
        root.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let first_instance = authority.transport_instance.lock().unwrap().instance_id();
    let owner = authority.take_transport_owner().unwrap();
    let mut lease = owner.write_lease().await;
    let peer_key =
        transport_peer_identity_key(LearnerEndpointRole::Learner, 4, "node-4", "guardian-4")
            .unwrap();
    authority
        .pin_peer_instance(&mut lease, &peer_key, [55; 32])
        .unwrap();
    authority
        .pin_peer_instance(&mut lease, &peer_key, [55; 32])
        .unwrap();
    assert_eq!(
        authority.pin_peer_instance(&mut lease, &peer_key, [56; 32]),
        Err(LearnerTransportError::AuthorityDenied)
    );
    drop(lease);
    drop(owner);
    drop(authority);

    let recovered = ProductionLearnerAuthority::open(
        root.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert_eq!(
        recovered.transport_instance.lock().unwrap().instance_id(),
        first_instance
    );
    assert_eq!(
        recovered
            .transport_instance
            .lock()
            .unwrap()
            .peer_instances()
            .get(&peer_key),
        Some(&[55; 32])
    );
    let alternate = ProductionLearnerAuthority::open(
        alternate_root.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    assert_ne!(
        alternate.transport_instance.lock().unwrap().instance_id(),
        first_instance
    );
    assert_ne!(
        transport_peer_identity_key(LearnerEndpointRole::Voter, 4, "node-4", "guardian-4",)
            .unwrap(),
        peer_key
    );
    assert_eq!(
        transport_peer_identity_key(
            LearnerEndpointRole::Learner,
            4,
            "node-4:guardian-4",
            "guardian-4",
        ),
        Err(TransportError::InvalidPeerBinding)
    );
    assertion(
        "transport_instance_and_peer_pin_are_durable_and_unique",
        "restart_preserves_instance_and_exact_peer_pin",
    );
    assertion(
        "transport_instance_and_peer_pin_are_durable_and_unique",
        "alternate_root_and_identity_alias_are_denied",
    );
    mark("transport_instance_and_peer_pin_are_durable_and_unique");
}

#[tokio::test]
async fn fresh_connection_requires_durable_peer_instance_pin() {
    let voter_root = portable_tempdir();
    let peer_root = portable_tempdir();
    let alternate_peer_root = portable_tempdir();
    let voter_checkpoint = Arc::new(MemoryCheckpoint::default());
    let peer_checkpoint = Arc::new(MemoryCheckpoint::default());
    let voter_authority = ProductionLearnerAuthority::open(
        voter_root.path(),
        Arc::clone(&voter_checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let peer_authority = ProductionLearnerAuthority::open(
        peer_root.path(),
        Arc::clone(&peer_checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let (voter_connection, peer_connection, voter_endpoint, peer_endpoint, _store) =
        live_learner_pair(1).await;
    let routes = [
        (4, "127.0.0.1:45404".parse().unwrap()),
        (1, peer_endpoint.local_addr().unwrap()),
        (3, "127.0.0.1:45303".parse().unwrap()),
    ]
    .into_iter()
    .collect();
    let identities = [
        (
            4,
            (
                "node-1".to_owned(),
                "guardian-1".to_owned(),
                SigningKey::from_bytes(&[1; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
        (
            1,
            (
                "node-4".to_owned(),
                "guardian-4".to_owned(),
                SigningKey::from_bytes(&[4; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
        (
            3,
            (
                "node-3".to_owned(),
                "guardian-3".to_owned(),
                SigningKey::from_bytes(&[3; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
    ]
    .into_iter()
    .collect();
    let cut = VerifiedPolisRouteCut::test_from_parts("polis-a", "runtime-prod", routes, identities);
    let voter_factory =
        SecurePolisNetworkFactory::from_authority_cut(4, cut.clone(), voter_authority.clone())
            .unwrap();
    let peer_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), peer_authority.clone())
            .unwrap();
    let voter_key = SigningKey::from_bytes(&[1; 32]);
    let peer_key = SigningKey::from_bytes(&[4; 32]);
    let (voter_session, peer_session) = tokio::join!(
        voter_factory.initiate_session(1, &voter_connection, &voter_key),
        peer_factory.accept_session(4, &peer_connection, &peer_key),
    );
    drop(voter_session.unwrap());
    drop(peer_session.unwrap());
    voter_connection.close();
    peer_connection.close();
    voter_endpoint.close(0_u32.into(), b"test phase complete");
    peer_endpoint.close(0_u32.into(), b"test phase complete");
    drop(peer_factory);
    drop(peer_authority);

    let recovered_peer_authority = ProductionLearnerAuthority::open(
        peer_root.path(),
        Arc::clone(&peer_checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let recovered_peer_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), recovered_peer_authority)
            .unwrap();
    let (voter_connection, peer_connection, voter_endpoint, peer_endpoint, _store) =
        live_learner_pair(1).await;
    let (voter_session, peer_session) = tokio::join!(
        voter_factory.initiate_session(1, &voter_connection, &voter_key),
        recovered_peer_factory.accept_session(4, &peer_connection, &peer_key),
    );
    drop(voter_session.unwrap());
    drop(peer_session.unwrap());
    voter_connection.close();
    peer_connection.close();
    voter_endpoint.close(0_u32.into(), b"test phase complete");
    peer_endpoint.close(0_u32.into(), b"test phase complete");
    drop(recovered_peer_factory);
    assertion(
        "fresh_connection_requires_durable_peer_instance_pin",
        "fresh_connection_accepts_restarted_peer_with_persisted_instance",
    );

    let alternate_peer_authority = ProductionLearnerAuthority::open(
        alternate_peer_root.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let alternate_peer_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut, alternate_peer_authority).unwrap();
    let (voter_connection, peer_connection, voter_endpoint, peer_endpoint, _store) =
        live_learner_pair(1).await;
    let stream_frames_before = voter_connection.test_stream_frames_sent();
    let (voter_result, peer_result) = tokio::join!(
        voter_factory.initiate_session(1, &voter_connection, &voter_key),
        alternate_peer_factory.accept_session(4, &peer_connection, &peer_key),
    );
    assert!(matches!(
        voter_result,
        Err(PolisRuntimeError::AuthorityDenied)
    ));
    assert!(matches!(
        peer_result,
        Err(PolisRuntimeError::AuthorityDenied)
    ));
    let stream_frames_after_handshake = voter_connection.test_stream_frames_sent();
    assert!(stream_frames_after_handshake >= stream_frames_before);
    tokio::task::yield_now().await;
    assert_eq!(
        voter_connection.test_stream_frames_sent(),
        stream_frames_after_handshake,
        "mismatched peer emitted a post-denial STREAM frame"
    );
    assertion(
        "fresh_connection_requires_durable_peer_instance_pin",
        "alternate_factory_denied_before_session_or_post_denial_stream",
    );
    voter_connection.close();
    peer_connection.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    peer_endpoint.close(0_u32.into(), b"test complete");
    mark("fresh_connection_requires_durable_peer_instance_pin");
}

fn exact_publisher(cut: &VerifiedPolisRouteCut, raft_id: u64) -> AuthorityNodeIdentity {
    let (node_id, guardian_id, boot_generation) = cut.authority_node_identity(raft_id).unwrap();
    AuthorityNodeIdentity {
        trust_domain: cut.trust_domain().to_owned(),
        polis_id: cut.polis_id().to_owned(),
        node_id,
        guardian_id,
        boot_generation,
    }
}

fn live_removal(
    cut: &VerifiedPolisRouteCut,
    target: &LearnerIdentity,
    publisher: AuthorityNodeIdentity,
    operation_id: &str,
) -> PublishedAuthorityResult {
    let artifact = LearnerMembershipArtifact::remove_voter(
        target.clone(),
        route_cut_digest(cut).unwrap(),
        MEMBERSHIP,
        NOW + 100,
        operation_id,
    )
    .unwrap();
    test_published_reconciliation_token(
        publisher,
        operation_id,
        artifact,
        42,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    )
}

fn live_admission(
    mut learner_identity: LearnerIdentity,
    address: std::net::SocketAddr,
    cut: &VerifiedPolisRouteCut,
) -> (VerifiedLearnerAdmission, i64) {
    learner_identity.address = address;
    let now = i64::try_from(unix_now()).unwrap();
    let cut_sha256 = route_cut_digest(cut).unwrap();
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        learner_identity.clone(),
        cut_sha256,
        None,
        MEMBERSHIP,
        Some(now + 240),
        now + 300,
    )
    .unwrap();
    let token = test_published_reconciliation_token(
        authority_identity(),
        "live-enroll-4",
        artifact,
        41,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    (
        VerifiedLearnerAdmission::from_published_membership(&token, &learner_identity, cut, now)
            .unwrap(),
        now,
    )
}

#[derive(Clone)]
struct AuthorizedLearnerMemoryNetwork {
    local: u64,
    peers: Arc<StdRwLock<BTreeMap<u64, PolisRaft>>>,
    learner_factories: Arc<StdRwLock<BTreeMap<u64, SecurePolisNetworkFactory>>>,
}

struct AuthorizedLearnerMemoryConnection {
    target: u64,
    network: AuthorizedLearnerMemoryNetwork,
}

impl RaftNetworkFactory<PolisTypeConfig> for AuthorizedLearnerMemoryNetwork {
    type Network = AuthorizedLearnerMemoryConnection;

    async fn new_client(&mut self, target: u64, _node: &BasicNode) -> Self::Network {
        AuthorizedLearnerMemoryConnection {
            target,
            network: self.clone(),
        }
    }
}

impl AuthorizedLearnerMemoryConnection {
    fn peer<E>(&self) -> Result<PolisRaft, RPCError<u64, BasicNode, RaftError<u64, E>>>
    where
        E: std::error::Error,
    {
        self.network
            .peers
            .read()
            .unwrap()
            .get(&self.target)
            .cloned()
            .ok_or_else(|| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))
    }

    async fn learner_client(&self) -> Result<SecurePolisNetworkConnection, PolisRuntimeError> {
        let mut factory = self
            .network
            .learner_factories
            .read()
            .unwrap()
            .get(&self.network.local)
            .cloned()
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        Ok(factory
            .new_client(self.target, &BasicNode::new("authorized-learner"))
            .await)
    }
}

impl RaftNetwork<PolisTypeConfig> for AuthorizedLearnerMemoryConnection {
    async fn append_entries(
        &mut self,
        request: AppendEntriesRequest<PolisTypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        if self.target == 4 {
            return self
                .learner_client()
                .await
                .map_err(|error| RPCError::Network(NetworkError::new(&error)))?
                .append_entries(request, _option)
                .await;
        }
        self.peer()?
            .append_entries(request)
            .await
            .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
    }

    async fn install_snapshot(
        &mut self,
        request: InstallSnapshotRequest<PolisTypeConfig>,
        _option: RPCOption,
    ) -> Result<
        InstallSnapshotResponse<u64>,
        RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>,
    > {
        if self.target == 4 {
            return self
                .learner_client()
                .await
                .map_err(|error| RPCError::Network(NetworkError::new(&error)))?
                .install_snapshot(request, _option)
                .await;
        }
        self.peer()?
            .install_snapshot(request)
            .await
            .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
    }

    async fn vote(
        &mut self,
        request: VoteRequest<u64>,
        _option: RPCOption,
    ) -> Result<VoteResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        if self.target == 4 {
            return Err(RPCError::Network(NetworkError::new(
                &PolisRuntimeError::AuthorityDenied,
            )));
        }
        self.peer()?
            .vote(request)
            .await
            .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
    }
}

fn exclusion() -> (
    tempfile::TempDir,
    Arc<MemoryCheckpoint>,
    PendingMembershipExclusionAuthority,
) {
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let authority = PendingMembershipExclusionAuthority::open(
        dir.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    (dir, checkpoint, authority)
}

#[tokio::test]
async fn real_four_node_learner_replication() {
    let raft_root = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let peers = Arc::new(StdRwLock::new(BTreeMap::new()));
    let learner_factories = Arc::new(StdRwLock::new(BTreeMap::new()));
    let configuration = Arc::new(
        openraft::Config {
            cluster_name: "adl-authorized-learner-test".to_owned(),
            heartbeat_interval: 50,
            election_timeout_min: 150,
            election_timeout_max: 300,
            ..Default::default()
        }
        .validate()
        .unwrap(),
    );
    let mut nodes = BTreeMap::new();
    let mut machines = BTreeMap::new();
    for node in 1..=4 {
        let node_root = raft_root.path().join(format!("node-{node}"));
        fs::create_dir(&node_root).unwrap();
        let log = PolisLogStore::open(
            &node_root,
            node,
            Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
        )
        .unwrap();
        let machine = PolisStateMachineStore::open(
            &node_root,
            node,
            Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
        )
        .unwrap();
        let raft = PolisRaft::new(
            node,
            Arc::clone(&configuration),
            AuthorizedLearnerMemoryNetwork {
                local: node,
                peers: Arc::clone(&peers),
                learner_factories: Arc::clone(&learner_factories),
            },
            log,
            machine.clone(),
        )
        .await
        .unwrap();
        nodes.insert(node, raft);
        machines.insert(node, machine);
    }
    *peers.write().unwrap() = nodes.clone();
    let voter_routes = (1..=3)
        .map(|node| (node, BasicNode::new(format!("memory://voter-{node}"))))
        .collect::<BTreeMap<_, _>>();
    nodes[&1].initialize(voter_routes.clone()).await.unwrap();
    let leader = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(leader) = nodes[&1].metrics().borrow().current_leader {
                break leader;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("leader election did not converge before promotion boundary writes");
    let before_learner = nodes[&leader]
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "authorized-learner-snapshot".to_owned(),
            payload_sha256: "33".repeat(32),
        })
        .await
        .unwrap();
    assert!(before_learner.data.accepted);
    nodes[&leader].trigger().snapshot().await.unwrap();
    let purge_upto = nodes[&leader]
        .metrics()
        .borrow()
        .last_applied
        .expect("leader has an applied log")
        .index;
    nodes[&leader]
        .trigger()
        .purge_log(purge_upto)
        .await
        .unwrap();

    let learner_control_identity = LocalNodeGuardianIdentity::generate("runtime-prod", 4).unwrap();
    let learner_public = learner_control_identity.public_identity();
    let boot_authority = SecureBootGenerationAuthority::open(
        &raft_root.path().join("learner-boot"),
        4,
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let learner_boot = boot_authority.advance().unwrap();
    let mut learner_identity = LearnerIdentity {
        trust_domain: learner_public.trust_domain.clone(),
        polis_id: "polis-a".to_owned(),
        node_id: learner_public.node_id.clone(),
        guardian_id: learner_public.guardian_id.clone(),
        guardian_control_public_key: learner_public.guardian_control_public_key,
        stable_raft_id: 4,
        certificate_generation: learner_public.identity_generation,
        boot_generation: learner_boot.generation(),
        address: "127.0.0.1:1".parse().unwrap(),
    };
    let (voter_connection, learner_connection, voter_endpoint, learner_endpoint, _store) =
        live_learner_pair_for(leader, &learner_identity).await;
    let learner_address = learner_endpoint.local_addr().unwrap();
    learner_identity.address = learner_address;
    let cut = live_voter_cut([3, 3, 3]);
    let removal_identity = exact_voter_target(&cut, 3);
    let voter_cut_sha256 = route_cut_digest(&cut).unwrap();
    let mut published_authority = cut.authority_membership().clone();
    let (admission, now) = live_admission(learner_identity, learner_address, &cut);
    let voter_authority = ProductionLearnerAuthority::open(
        &raft_root.path().join("voter-authority"),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let factory =
        SecurePolisNetworkFactory::from_authority_cut(leader, cut.clone(), voter_authority.clone())
            .unwrap();
    let admission_receipt = factory
        .activate_learner_admission(&admission, now)
        .await
        .unwrap();
    assert_eq!(
        admission_receipt.operation_sha256(),
        admission.operation_sha256()
    );
    assert_ne!(admission_receipt.published_state_sha256(), [0; 32]);
    assert_eq!(
        factory
            .observe_learner_admission_receipt(admission.operation_sha256())
            .await
            .unwrap(),
        Some(admission_receipt.clone())
    );
    assert_eq!(
        factory
            .observe_learner_admission_receipt([0x51; 32])
            .await
            .unwrap(),
        None
    );
    println!("ADL_ISSUE_199_ASSERTION_V1 case=add_learner_joint_final_publish assertion=factory_admission_receipt_exact_current_and_mismatch_denied");
    let learner_authority = ProductionLearnerAuthority::open(
        &raft_root.path().join("learner-owned-authority"),
        Arc::new(MemoryCheckpoint::default()) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let learner_effect_authority = learner_authority.clone();
    let learner_factory = Arc::new(
        SecureLearnerNetworkFactory::bootstrap(
            cut.clone(),
            admission.clone(),
            learner_authority,
            &learner_control_identity,
            learner_boot,
            now,
        )
        .await
        .unwrap(),
    );
    let voter_signing_key = SigningKey::from_bytes(&[leader as u8; 32]);
    let (installed, server_sessions) = tokio::join!(
        tokio::time::timeout(
            Duration::from_secs(10),
            factory.install_learner_route(
                4,
                Arc::clone(&voter_connection),
                &admission,
                now,
                &voter_signing_key,
            ),
        ),
        tokio::time::timeout(
            Duration::from_secs(10),
            learner_factory.server_sessions(&learner_connection, now),
        ),
    );
    let (inbound, outbound) = server_sessions.unwrap().unwrap();
    installed.unwrap().unwrap();
    learner_factories
        .write()
        .unwrap()
        .insert(leader, factory.clone());
    let cancellation = CancellationToken::new();
    let learner_server = tokio::spawn(serve_authorized_learner_connection(
        nodes[&4].clone(),
        learner_connection,
        inbound,
        outbound,
        cancellation.child_token(),
    ));

    let old_stable_ids = published_authority.raft_ids.clone();
    let mut target_stable_ids = old_stable_ids.clone();
    target_stable_ids.insert(
        admission.identity().guardian_id.as_bytes().to_vec(),
        admission.identity().stable_raft_id,
    );
    let old_membership = old_stable_ids.values().copied().collect::<BTreeSet<_>>();
    let target_membership = target_stable_ids.values().copied().collect::<BTreeSet<_>>();
    let promotion_artifact = PromoteVoterArtifact::committed(
        admission.identity().clone(),
        admission.voter_cut_sha256(),
        admission.operation_sha256(),
        admission_receipt.generation(),
        stable_map_sha256(&old_stable_ids).unwrap(),
        stable_map_sha256(&target_stable_ids).unwrap(),
        membership_set_sha256(&target_membership).unwrap(),
        admission.deadline_unix_seconds,
    )
    .unwrap();
    let leader = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(leader) = nodes[&1].metrics().borrow().current_leader {
                break leader;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("leader election did not converge before learner replication write");
    for sequence in 0..8 {
        nodes[&leader]
            .client_write(PolisCommand::GovernedMutation {
                mutation_id: format!("promotion-authority-boundary-{sequence}"),
                payload_sha256: format!("{:064x}", sequence + 100),
            })
            .await
            .unwrap();
    }
    let authority_index = nodes[&leader]
        .metrics()
        .borrow()
        .last_applied
        .expect("leader applied authority baseline")
        .index;
    let promotion_result = test_published_reconciliation_token(
        authority_identity(),
        "real-four-node-promote-4",
        promotion_artifact,
        authority_index,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let promotion = VerifiedPromoteVoter::from_published(
        &promotion_result,
        admission.identity(),
        admission.voter_cut_sha256(),
        stable_map_sha256(&old_stable_ids).unwrap(),
        stable_map_sha256(&target_stable_ids).unwrap(),
        now,
    )
    .unwrap();
    let mut published_membership =
        MembershipState::new(MembershipPolicy::new("runtime-prod", 8, 16).unwrap());
    for node in 1..=3 {
        let voter = published_authority
            .voters
            .get(format!("guardian-{node}").as_bytes())
            .unwrap();
        published_membership
            .apply(&CommittedMembershipEvent::new(
                "runtime-prod",
                [node as u8; 32],
                node,
                node,
                MembershipOperation::Join {
                    member: Member {
                        node_id: format!("node-{node}"),
                        guardian_id: format!("guardian-{node}"),
                        identity_generation: voter.certificate_generation,
                        guardian_control_public_key: voter.control_public_key,
                        role: MemberRole::NonVoting,
                    },
                },
            ))
            .unwrap();
    }
    for node in 1..=3 {
        published_membership
            .apply(&CommittedMembershipEvent::new(
                "runtime-prod",
                [node as u8 + 10; 32],
                node + 3,
                node + 3,
                MembershipOperation::Promote {
                    node_id: format!("node-{node}"),
                },
            ))
            .unwrap();
    }
    published_membership
        .apply(&CommittedMembershipEvent::new(
            "runtime-prod",
            [44; 32],
            7,
            7,
            MembershipOperation::Join {
                member: Member {
                    node_id: admission.identity().node_id.clone(),
                    guardian_id: admission.identity().guardian_id.clone(),
                    identity_generation: admission.identity().certificate_generation,
                    guardian_control_public_key: admission.identity().guardian_control_public_key,
                    role: MemberRole::NonVoting,
                },
            },
        ))
        .unwrap();
    published_authority.committed_log_index = published_membership.committed_log_index();
    let candidate_authority = VoterAuthority {
        guardian_id: admission.identity().guardian_id.as_bytes().to_vec(),
        trust_domain_id: admission.identity().trust_domain.as_bytes().to_vec(),
        certificate_generation: admission.identity().certificate_generation,
        purpose: ControlCertificatePurpose::AuthorityEndorsement,
        not_before_unix_seconds: now - 1,
        not_after_unix_seconds: admission.deadline_unix_seconds,
        revoked: false,
        control_public_key: admission.identity().guardian_control_public_key,
    };
    let transition = AuthorizedMembershipTransition {
        old_stable_ids,
        target_stable_ids,
        old_membership,
        target_membership,
    };
    let coordinator_root = raft_root.path().join("membership-coordinator");
    fs::create_dir(&coordinator_root).unwrap();
    let coordinator = MembershipCoordinator::open(
        &coordinator_root,
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let mut runtime = GovernedMembershipRuntime::new(
        coordinator,
        factory.clone(),
        nodes[&leader].clone(),
        machines[&leader].clone(),
        published_membership,
        published_authority,
    );
    for message_kind in ["vote", "generic", "unknown"] {
        assert_eq!(
            factory.request_bytes(4, message_kind, b"{}".to_vec()).await,
            Err(PolisRuntimeError::AuthorityDenied)
        );
    }
    let published = tokio::time::timeout(
        Duration::from_secs(20),
        runtime.promote(&promotion, &transition, candidate_authority.clone()),
    )
    .await
    .expect("governed promotion timed out")
    .unwrap();
    assert_ne!(published, [0; 32]);
    assert_eq!(runtime.coordinator().published_generation(), 1);
    assert_eq!(
        runtime
            .membership()
            .member(&admission.identity().node_id)
            .unwrap()
            .role,
        MemberRole::Voter
    );
    assert!(
        !learner_server.is_finished(),
        "learner server ended during catch-up"
    );
    let leader = loop {
        if let Some(leader) = nodes[&1].metrics().borrow().current_leader {
            break leader;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    };
    let replicated = nodes[&leader]
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "authorized-learner-replicated".to_owned(),
            payload_sha256: "44".repeat(32),
        })
        .await
        .unwrap();
    assert!(replicated.data.accepted);
    for _ in 0..1000 {
        if machines[&4]
            .application_state()
            .await
            .mutation_ids
            .contains("authorized-learner-replicated")
        {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(machines[&4]
        .application_state()
        .await
        .mutation_ids
        .contains("authorized-learner-replicated"));
    let hook = learner_effect_authority.install_dispatch_pause_for_test("learner_raft_effect");
    let effect_raft = nodes[&leader].clone();
    let effect = tokio::spawn(async move {
        effect_raft
            .client_write(PolisCommand::GovernedMutation {
                mutation_id: "authorized-learner-fenced-effect".to_owned(),
                payload_sha256: "45".repeat(32),
            })
            .await
    });
    tokio::time::timeout(Duration::from_secs(10), hook.reached.notified())
        .await
        .expect("real learner effect did not reach dispatch hook");
    let expiry_factory = Arc::clone(&learner_factory);
    let expiry_at = admission.deadline_unix_seconds;
    let expiry = tokio::spawn(async move { expiry_factory.expire_admission(expiry_at).await });
    for _ in 0..8 {
        tokio::task::yield_now().await;
    }
    assert!(
        !expiry.is_finished(),
        "expiry crossed an in-flight learner Raft effect/response lease"
    );
    hook.release.notify_one();
    let response = effect.await.unwrap().unwrap();
    assert!(response.data.accepted);
    expiry.await.unwrap().unwrap();
    assertion(
        "real_four_node_learner_replication",
        "expiry_writer_waits_through_real_raft_effect_and_response",
    );
    for _ in 0..200 {
        if machines[&4]
            .application_state()
            .await
            .mutation_ids
            .contains("authorized-learner-replicated")
        {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(machines[&4]
        .application_state()
        .await
        .mutation_ids
        .contains("authorized-learner-replicated"));
    assert!(machines[&4]
        .application_state()
        .await
        .mutation_ids
        .contains("authorized-learner-snapshot"));
    let membership = nodes[&leader].metrics().borrow().membership_config.clone();
    let voters = membership
        .membership()
        .get_joint_config()
        .iter()
        .flatten()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(voters, std::collections::BTreeSet::from([1, 2, 3, 4]));
    assertion(
        "real_four_node_learner_replication",
        "raft_add_learner_replicated",
    );
    assertion(
        "real_four_node_learner_replication",
        "voter_quorum_unchanged",
    );
    assertion(
        "real_four_node_learner_replication",
        "quinn_append_snapshot_only",
    );
    assert_eq!(voter_routes.len(), 3);
    let voters = BTreeMap::from([(1, "a"), (2, "b"), (3, "c")]);
    assert_eq!(voters.len(), 3);
    assert!(!voters.contains_key(&4));

    let removal_old_stable_ids = runtime.authority().raft_ids.clone();
    let mut removal_target_stable_ids = removal_old_stable_ids.clone();
    removal_target_stable_ids.remove(removal_identity.guardian_id.as_bytes());
    let removal_old_membership = removal_old_stable_ids
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let removal_target_membership = removal_target_stable_ids
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let removal_target_sha256 = membership_set_sha256(&removal_target_membership).unwrap();
    let removal_artifact = LearnerMembershipArtifact::remove_voter(
        removal_identity.clone(),
        voter_cut_sha256,
        removal_target_sha256,
        admission.deadline_unix_seconds,
        "real-four-node-remove-3",
    )
    .unwrap();
    let removal_authority_index = nodes[&leader]
        .metrics()
        .borrow()
        .last_applied
        .unwrap()
        .index;
    let removal_result = test_published_reconciliation_token(
        authority_identity(),
        "real-four-node-remove-3",
        removal_artifact,
        removal_authority_index,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let removal_transition = AuthorizedMembershipTransition {
        old_stable_ids: removal_old_stable_ids,
        target_stable_ids: removal_target_stable_ids,
        old_membership: removal_old_membership,
        target_membership: removal_target_membership,
    };
    for boundary in [
        MembershipCrashBoundary::BeforeExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityObservation,
        MembershipCrashBoundary::AfterJointHistory,
        MembershipCrashBoundary::AfterFinalHistory,
        MembershipCrashBoundary::AfterJointFinalObservation,
        MembershipCrashBoundary::AfterStableMapPreparation,
        MembershipCrashBoundary::AfterLocalProjectionPrepared,
        MembershipCrashBoundary::AfterParityReconciliation,
        MembershipCrashBoundary::BeforeCheckpoint,
        MembershipCrashBoundary::AfterCheckpoint,
        MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility,
    ] {
        runtime.inject_crash_boundary(boundary);
        let removal_attempt = runtime
            .remove(
                &removal_result,
                &removal_identity,
                voter_cut_sha256,
                now,
                &removal_transition,
            )
            .await;
        assert_eq!(
            removal_attempt,
            Err(MembershipCoordinatorError::StateRegression),
            "removal boundary {boundary:?} must fail closed"
        );
        assert!(
            runtime.crash_boundary_hit(),
            "removal boundary {boundary:?} was not reached"
        );
        runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
    }
    let removed = tokio::time::timeout(
        Duration::from_secs(20),
        runtime.remove(
            &removal_result,
            &removal_identity,
            voter_cut_sha256,
            now,
            &removal_transition,
        ),
    )
    .await
    .expect("governed removal timed out")
    .unwrap();
    assert_ne!(removed, [0; 32]);
    assert_eq!(runtime.coordinator().published_generation(), 2);
    assert!(runtime
        .membership()
        .member(&removal_identity.node_id)
        .is_none());
    assert!(!runtime
        .authority()
        .raft_ids
        .contains_key(removal_identity.guardian_id.as_bytes()));
    nodes[&3].shutdown().await.unwrap();
    let rejoin_node_root = raft_root.path().join("node-3-rejoin");
    fs::create_dir(&rejoin_node_root).unwrap();
    let rejoin_checkpoint: Arc<dyn ConsensusCheckpointAuthority> =
        Arc::new(MemoryCheckpoint::default());
    let rejoin_log =
        PolisLogStore::open(&rejoin_node_root, 3, Arc::clone(&rejoin_checkpoint)).unwrap();
    let rejoin_machine =
        PolisStateMachineStore::open(&rejoin_node_root, 3, rejoin_checkpoint).unwrap();
    let rejoin_configuration = Arc::new(
        openraft::Config {
            cluster_name: "adl-authorized-rejoin-test".to_owned(),
            heartbeat_interval: 50,
            election_timeout_min: 30_000,
            election_timeout_max: 31_000,
            ..Default::default()
        }
        .validate()
        .unwrap(),
    );
    let rejoin_raft = PolisRaft::new(
        3,
        rejoin_configuration,
        AuthorizedLearnerMemoryNetwork {
            local: 3,
            peers: Arc::clone(&peers),
            learner_factories: Arc::clone(&learner_factories),
        },
        rejoin_log,
        rejoin_machine.clone(),
    )
    .await
    .unwrap();
    nodes.insert(3, rejoin_raft);
    machines.insert(3, rejoin_machine);
    *peers.write().unwrap() = nodes.clone();

    let mut recovered = removal_identity.clone();
    recovered.node_id = "node-3-recovered".to_owned();
    recovered.guardian_id = "guardian-3-recovered".to_owned();
    recovered.guardian_control_public_key =
        SigningKey::from_bytes(&[93; 32]).verifying_key().to_bytes();
    recovered.certificate_generation += 1;
    recovered.boot_generation += 1;
    recovered.address = "127.0.0.1:46303".parse().unwrap();
    let enrollment_artifact = LearnerMembershipArtifact::enroll_non_voting(
        recovered.clone(),
        voter_cut_sha256,
        Some(admission.operation_sha256()),
        removal_target_sha256,
        None,
        admission.deadline_unix_seconds,
    )
    .unwrap();
    let enrollment_result = test_published_reconciliation_token(
        authority_identity(),
        "real-four-node-rejoin-enroll-3",
        enrollment_artifact,
        removal_authority_index + 100,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let recovery_admission = VerifiedLearnerAdmission::from_published_membership(
        &enrollment_result,
        &recovered,
        &cut,
        now,
    )
    .unwrap();
    let enrollment_log_index = runtime.membership().committed_log_index() + 1;
    factory
        .expire_learner_admission(admission.deadline_unix_seconds)
        .await
        .unwrap();
    let mut drifted_authority = runtime.authority().clone();
    drifted_authority
        .raft_ids
        .insert(b"guardian-1".to_vec(), 99);
    let exact_authority = runtime.replace_authority_for_test(drifted_authority);
    assert_eq!(
        runtime
            .enroll_non_voting(&recovery_admission, now, enrollment_log_index)
            .await,
        Err(MembershipCoordinatorError::WrongStableMap),
        "enrollment must reject local authority/Raft parity drift before journaling or #202 effects"
    );
    runtime.replace_authority_for_test(exact_authority);
    for boundary in [
        MembershipCrashBoundary::BeforeEnrollmentJournal,
        MembershipCrashBoundary::AfterEnrollmentJournal,
        MembershipCrashBoundary::BeforeExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityObservation,
        MembershipCrashBoundary::AfterStableMapPreparation,
        MembershipCrashBoundary::AfterLocalProjectionPrepared,
        MembershipCrashBoundary::BeforeCheckpoint,
        MembershipCrashBoundary::AfterCheckpoint,
        MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility,
    ] {
        runtime.inject_crash_boundary(boundary);
        assert_eq!(
            runtime
                .enroll_non_voting(&recovery_admission, now, enrollment_log_index)
                .await,
            Err(MembershipCoordinatorError::StateRegression),
            "enrollment boundary {boundary:?} must fail closed"
        );
        assert!(
            runtime.crash_boundary_hit(),
            "enrollment boundary {boundary:?} was not reached"
        );
        runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
    }
    assert_eq!(runtime.coordinator().published_generation(), 3);
    assert!(runtime.membership().member(&recovered.node_id).is_none());
    let enrolled = runtime
        .enroll_non_voting(&recovery_admission, now, enrollment_log_index)
        .await
        .unwrap();
    assert_ne!(enrolled, [0; 32]);
    assert_eq!(runtime.coordinator().published_generation(), 3);
    assert_eq!(
        runtime
            .membership()
            .member(&recovered.node_id)
            .unwrap()
            .role,
        MemberRole::NonVoting
    );

    let rejoin_old_stable_ids = runtime.authority().raft_ids.clone();
    let mut rejoin_target_stable_ids = rejoin_old_stable_ids.clone();
    rejoin_target_stable_ids.insert(recovered.guardian_id.as_bytes().to_vec(), 3);
    let rejoin_old_membership = rejoin_old_stable_ids
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let rejoin_target_membership = rejoin_target_stable_ids
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let recovery_receipt = factory
        .observe_learner_admission_receipt(recovery_admission.operation_sha256())
        .await
        .unwrap()
        .unwrap();
    let rejoin_artifact = PromoteVoterArtifact::committed(
        recovered.clone(),
        voter_cut_sha256,
        recovery_admission.operation_sha256(),
        recovery_receipt.generation(),
        stable_map_sha256(&rejoin_old_stable_ids).unwrap(),
        stable_map_sha256(&rejoin_target_stable_ids).unwrap(),
        membership_set_sha256(&rejoin_target_membership).unwrap(),
        admission.deadline_unix_seconds,
    )
    .unwrap();
    let rejoin_authority_index = enrollment_log_index;
    let rejoin_result = test_published_reconciliation_token(
        authority_identity(),
        "real-four-node-rejoin-promote-3",
        rejoin_artifact,
        rejoin_authority_index,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let rejoin = VerifiedPromoteVoter::from_published(
        &rejoin_result,
        &recovered,
        voter_cut_sha256,
        stable_map_sha256(&rejoin_old_stable_ids).unwrap(),
        stable_map_sha256(&rejoin_target_stable_ids).unwrap(),
        now,
    )
    .unwrap();
    let rejoin_transition = AuthorizedMembershipTransition {
        old_stable_ids: rejoin_old_stable_ids,
        target_stable_ids: rejoin_target_stable_ids,
        old_membership: rejoin_old_membership,
        target_membership: rejoin_target_membership,
    };
    assert_eq!(
        verify_authorized_transition_inputs(
            &rejoin,
            &rejoin_transition.old_stable_ids,
            &rejoin_transition.target_stable_ids,
            &rejoin_transition.old_membership,
            &rejoin_transition.target_membership,
        ),
        Ok(())
    );
    let recovered_authority = VoterAuthority {
        guardian_id: recovered.guardian_id.as_bytes().to_vec(),
        trust_domain_id: recovered.trust_domain.as_bytes().to_vec(),
        certificate_generation: recovered.certificate_generation,
        purpose: ControlCertificatePurpose::AuthorityEndorsement,
        not_before_unix_seconds: now - 1,
        not_after_unix_seconds: admission.deadline_unix_seconds,
        revoked: false,
        control_public_key: recovered.guardian_control_public_key,
    };
    for boundary in [
        MembershipCrashBoundary::BeforeExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityCall,
        MembershipCrashBoundary::AfterExternalAuthorityObservation,
        MembershipCrashBoundary::BeforeLearnerEffect,
        MembershipCrashBoundary::AfterLearnerEffect,
    ] {
        runtime.inject_crash_boundary(boundary);
        assert_eq!(
            runtime
                .promote(&rejoin, &rejoin_transition, recovered_authority.clone())
                .await,
            Err(MembershipCoordinatorError::StateRegression),
            "promotion boundary {boundary:?} must fail closed"
        );
        assert!(
            runtime.crash_boundary_hit(),
            "promotion boundary {boundary:?} was not reached"
        );
        runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
    }
    runtime.inject_membership_change_no_effect_failure();
    assert_eq!(
        runtime
            .promote(&rejoin, &rejoin_transition, recovered_authority.clone(),)
            .await,
        Err(MembershipCoordinatorError::StateRegression)
    );
    assert_eq!(
        runtime.coordinator().active_phase(),
        Some(MembershipCoordinatorPhase::LearnerCaughtUp)
    );
    runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
    runtime.inject_crash_after_membership_change();
    let rejoined_result = tokio::time::timeout(
        Duration::from_secs(20),
        runtime.promote(&rejoin, &rejoin_transition, recovered_authority.clone()),
    )
    .await
    .expect("governed rejoin timed out");
    let rejoined = match rejoined_result {
        Ok(result) => result,
        Err(MembershipCoordinatorError::StateRegression)
            if runtime.coordinator().active_phase()
                == Some(MembershipCoordinatorPhase::LearnerCaughtUp) =>
        {
            assert_eq!(runtime.coordinator().published_generation(), 3);
            assert_eq!(
                runtime
                    .membership()
                    .member(&recovered.node_id)
                    .unwrap()
                    .role,
                MemberRole::NonVoting,
                "local membership must not become visible before durable publication"
            );
            assert!(!runtime.authority().raft_ids.values().any(|id| *id == 3));
            // Resume immediately, before the final membership entry is
            // externally observed. The durable submitted marker must make the
            // coordinator wait for exact history instead of repeating the
            // membership-change effect.
            runtime.resume_consensus(nodes[&1].clone(), machines[&1].clone());
            runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
            for boundary in [
                MembershipCrashBoundary::AfterJointHistory,
                MembershipCrashBoundary::AfterFinalHistory,
                MembershipCrashBoundary::AfterJointFinalObservation,
                MembershipCrashBoundary::AfterStableMapPreparation,
                MembershipCrashBoundary::AfterLocalProjectionPrepared,
                MembershipCrashBoundary::AfterParityReconciliation,
                MembershipCrashBoundary::BeforeCheckpoint,
                MembershipCrashBoundary::AfterCheckpoint,
                MembershipCrashBoundary::AfterDurablePublicationBeforeVisibility,
            ] {
                runtime.inject_crash_boundary(boundary);
                assert_eq!(
                    runtime
                        .promote(&rejoin, &rejoin_transition, recovered_authority.clone(),)
                        .await,
                    Err(MembershipCoordinatorError::StateRegression),
                    "promotion boundary {boundary:?} must fail closed"
                );
                assert!(
                    runtime.crash_boundary_hit(),
                    "promotion boundary {boundary:?} was not reached"
                );
                runtime.reopen_coordinator(&coordinator_root, checkpoint.clone());
            }
            runtime
                .promote(&rejoin, &rejoin_transition, recovered_authority.clone())
                .await
                .unwrap()
        }
        Err(error) => panic!("unexpected rejoin error: {error:?}"),
    };
    assert_ne!(rejoined, [0; 32]);
    assert_eq!(runtime.coordinator().published_generation(), 4);
    assert!(runtime.has_published_result(promotion.operation_sha256()));
    assert_eq!(
        runtime
            .promote(&promotion, &transition, candidate_authority)
            .await
            .unwrap(),
        published,
        "an older retained operation must remain an exact retry-cache hit"
    );
    assert_eq!(runtime.coordinator().published_generation(), 4);
    assert_eq!(
        runtime
            .enroll_non_voting(&recovery_admission, now, enrollment_log_index)
            .await
            .unwrap(),
        enrolled,
        "an older retained enrollment must be a pure retry-cache hit"
    );
    assert_eq!(
        runtime
            .membership()
            .member(&recovered.node_id)
            .unwrap()
            .role,
        MemberRole::Voter,
        "older enrollment retry must not resurrect NonVoting visibility"
    );
    assert_eq!(
        runtime
            .promote(&rejoin, &rejoin_transition, recovered_authority.clone(),)
            .await
            .unwrap(),
        rejoined,
        "exact published retry must return the durable result without another transition"
    );
    assert_eq!(runtime.coordinator().published_generation(), 4);
    assert_eq!(
        runtime
            .membership()
            .member(&recovered.node_id)
            .unwrap()
            .role,
        MemberRole::Voter
    );
    assert!(runtime
        .authority()
        .raft_membership
        .voter_ids()
        .any(|voter| voter == 3));
    assert!(machines[&1]
        .applied_membership_history()
        .await
        .iter()
        .any(|entry| entry.joint_configs
            == vec![BTreeSet::from([1, 2, 4]), BTreeSet::from([1, 2, 3, 4])]));
    println!("ADL_ISSUE_199_ASSERTION_V1 case=remove_rejoin_real_nodes assertion=exclusion_retain_false_separate_enrollment_promotion_catchup_and_parity_publication");
    println!("ADL_ISSUE_199_ASSERTION_V1 case=crash_phase_matrix assertion=enrollment_removal_promotion_boundaries_retry_without_duplicate_visibility");
    for raft in nodes.values() {
        raft.shutdown().await.unwrap();
    }
    cancellation.cancel();
    voter_connection.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    let _ = learner_server.await;
    mark("real_four_node_learner_replication");
}

#[test]
fn current_voter_cut_unchanged() {
    let before = BTreeMap::from([(1, "a"), (2, "b"), (3, "c")]);
    let admission = admission();
    assert_eq!(before.len(), 3);
    assert!(!before.contains_key(&admission.identity.stable_raft_id));
    assert_eq!(admission.voter_cut_sha256, CUT);
    mark("current_voter_cut_unchanged");
}

#[tokio::test]
async fn excluded_node_recovery_learner() {
    let cut = live_voter_cut([3, 3, 3]);
    let cut_sha256 = route_cut_digest(&cut).unwrap();
    let old = exact_voter_target(&cut, 3);
    let removal = live_removal(&cut, &old, exact_publisher(&cut, 1), "remove-3-recovery");
    let authority_root = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_root.path(),
        Arc::new(MemoryCheckpoint::default()) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), authority.clone()).unwrap();
    let exclusion_receipt = factory
        .activate_pending_exclusion(&removal, &old, cut_sha256, MEMBERSHIP, NOW)
        .await
        .unwrap();
    assert_eq!(
        exclusion_receipt.operation_sha256(),
        removal.result_sha256()
    );
    assert_ne!(exclusion_receipt.published_state_sha256(), [0; 32]);
    assert_eq!(
        factory
            .observe_pending_exclusion_receipt(removal.result_sha256())
            .await
            .unwrap(),
        Some(exclusion_receipt)
    );
    assert_eq!(
        factory
            .observe_pending_exclusion_receipt([0x52; 32])
            .await
            .unwrap(),
        None
    );
    println!("ADL_ISSUE_199_ASSERTION_V1 case=remove_voter_pending_exclusion assertion=factory_exclusion_receipt_exact_current_and_mismatch_denied");
    let snapshot = authority.exclusion_snapshot().unwrap();
    let mut recovered = old.clone();
    recovered.node_id = "node-3-recovered".to_owned();
    recovered.guardian_id = "guardian-3-recovered".to_owned();
    recovered.guardian_control_public_key =
        SigningKey::from_bytes(&[93; 32]).verifying_key().to_bytes();
    recovered.certificate_generation += 1;
    recovered.boot_generation += 1;
    recovered.address = "127.0.0.1:46303".parse().unwrap();
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        recovered.clone(),
        cut_sha256,
        None,
        MEMBERSHIP,
        None,
        NOW + 100,
    )
    .unwrap();
    let token = test_published_reconciliation_token(
        exact_publisher(&cut, 1),
        "recover-3",
        artifact,
        43,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let admission =
        VerifiedLearnerAdmission::from_published_membership(&token, &recovered, &cut, NOW).unwrap();
    assert!(snapshot.recovery_learner_allowed(&admission));
    assert!(!snapshot.ordinary_authority_allowed(&old.node_id, &old.guardian_id));
    factory
        .activate_learner_admission(&admission, NOW)
        .await
        .unwrap();
    assert!(authority.admission_is_current(&admission).unwrap());
    let mut wrong_membership = admission.clone();
    wrong_membership.target_membership_sha256 = [94; 32];
    assert!(!authority.admission_is_current(&wrong_membership).unwrap());
    assertion(
        "excluded_node_recovery_learner",
        "production_factory_enforces_recovery_identity_index_and_membership",
    );
    mark("excluded_node_recovery_learner");
}

#[test]
fn learner_promotion_route_handoff() {
    let mut session = session();
    session.close();
    assert_eq!(
        session.authorize(LearnerRpcKind::AppendEntries, 1, b"x", NOW),
        Err(LearnerTransportError::AuthorityDenied)
    );
    mark("learner_promotion_route_handoff");
}

#[test]
fn exact_retry_session() {
    let (_dir, _checkpoint, mut authority) = exclusion();
    let target = identity();
    let token = remove_token_with(target.clone(), "remove-exact", 42);
    assert_eq!(
        authority.activate(&token, &target, CUT, [95; 32], NOW),
        Err(LearnerTransportError::InvalidBinding)
    );
    let first = authority
        .activate(&token, &target, CUT, MEMBERSHIP, NOW)
        .unwrap();
    let second = authority
        .activate(&token, &target, CUT, MEMBERSHIP, NOW + 101)
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        authority.activate(&token, &target, CUT, [95; 32], NOW + 101),
        Err(LearnerTransportError::InvalidBinding)
    );
    let expired_dir = portable_tempdir();
    let expired_checkpoint = Arc::new(MemoryCheckpoint::default());
    let mut expired = PendingMembershipExclusionAuthority::open(
        expired_dir.path(),
        expired_checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert_eq!(
        expired.activate(&token, &target, CUT, MEMBERSHIP, NOW + 101),
        Err(LearnerTransportError::Expired)
    );
    assertion("exact_retry_session", "exclusion_exact_retry_cached");
    assertion(
        "exact_retry_session",
        "removal_deadline_and_target_membership_bound_cache_first",
    );
    let admission_dir = portable_tempdir();
    let admission_checkpoint = Arc::new(MemoryCheckpoint::default());
    let mut admissions = LearnerAdmissionAuthority::open(
        admission_dir.path(),
        admission_checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let admission = admission();
    assert_eq!(
        admissions.activate(&admission).unwrap(),
        admissions.activate(&admission).unwrap()
    );
    assertion("exact_retry_session", "admission_exact_retry_cached");
    mark("exact_retry_session");
}

#[test]
fn reconnect_boot_rotation() {
    let old = session();
    let mut next_identity = identity();
    next_identity.boot_generation += 1;
    let next = enroll_token_with(
        next_identity.clone(),
        "boot-rotate",
        42,
        NOW + 100,
        Some(admission().operation_sha256),
    );
    let next = VerifiedLearnerAdmission::from_published_membership_for_test(
        &next,
        &next_identity,
        CUT,
        NOW,
    )
    .unwrap();
    assert_ne!(
        old.binding.identity.boot_generation,
        next.identity.boot_generation
    );
    mark("reconnect_boot_rotation");
}

#[test]
fn certificate_overlap_authorized() {
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let authority = ProductionLearnerAuthority::open(
        dir.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let current = admission();
    authority.activate_admission(&current).unwrap();
    let voter = LearnerVoterBinding {
        stable_raft_id: 1,
        node_id: "node-1".to_owned(),
        guardian_id: "guardian-1".to_owned(),
        certificate_generation: 4,
        boot_generation: 3,
        control_public_key: SigningKey::from_bytes(&[1; 32]).verifying_key().to_bytes(),
    };
    let mut old = EstablishedLearnerSession::new(
        &current,
        CUT,
        voter.clone(),
        LearnerEndpointRole::Voter,
        authority.clone(),
        NOW,
    )
    .unwrap();
    let mut retained_old = old.clone();
    assert!(old
        .authorize(LearnerRpcKind::AppendEntries, 1, b"old", NOW)
        .is_ok());
    let mut next_identity = identity();
    next_identity.certificate_generation += 1;
    let token = enroll_token_with(
        next_identity.clone(),
        "cert-successor",
        42,
        NOW + 200,
        Some(admission().operation_sha256),
    );
    let next = VerifiedLearnerAdmission::from_published_membership_for_test(
        &token,
        &next_identity,
        CUT,
        NOW,
    )
    .unwrap();
    assert_ne!(old.binding.operation_sha256, next.operation_sha256);
    authority.stage_successor(&next).unwrap();
    assert_eq!(
        authority
            .admission_snapshot()
            .unwrap()
            .current()
            .unwrap()
            .operation_sha256,
        current.operation_sha256
    );
    assertion(
        "certificate_overlap_authorized",
        "successor_private_before_flip",
    );
    let recovered = authority;
    let flipped = recovered.flip_successor(next.operation_sha256).unwrap();
    assert_eq!(
        flipped.current().unwrap().operation_sha256,
        next.operation_sha256
    );
    assertion("certificate_overlap_authorized", "successor_atomic_flip");
    assert_eq!(
        old.authorize(LearnerRpcKind::AppendEntries, 2, b"late", NOW),
        Err(LearnerTransportError::AuthorityDenied)
    );
    assert_eq!(
        retained_old.authorize(LearnerRpcKind::InstallSnapshot, 1, b"late-clone", NOW),
        Err(LearnerTransportError::AuthorityDenied)
    );
    let mut successor = EstablishedLearnerSession::new(
        &next,
        CUT,
        voter,
        LearnerEndpointRole::Voter,
        recovered,
        NOW,
    )
    .unwrap();
    assert!(successor
        .authorize(LearnerRpcKind::AppendEntries, 1, b"next", NOW)
        .is_ok());
    assertion(
        "certificate_overlap_authorized",
        "retained_old_clones_atomically_revoked",
    );
    mark("certificate_overlap_authorized");
}

#[test]
fn missing_201_token() {
    assert_eq!(
        session().vote(),
        Err(LearnerTransportError::AuthorityDenied)
    );
    mark("missing_201_token");
}

#[test]
fn public_caller_denied() {
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        NOW + 100,
    )
    .unwrap();
    assert_eq!(artifact.domain, MEMBERSHIP_ARTIFACT_DOMAIN);
    assert_ne!(artifact.sha256, [0; 32]);
    mark("public_caller_denied");
}

#[test]
fn wrong_operation_kind() {
    let membership = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        NOW + 100,
    )
    .unwrap();
    let wrong =
        CommittedAuthorityArtifact::new(AuthorityOperationKind::ExistingStore, membership.bytes)
            .unwrap();
    let token = test_published_reconciliation_token(
        authority_identity(),
        "wrong-kind",
        wrong,
        41,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    assert_eq!(
        VerifiedLearnerAdmission::from_published_membership_for_test(&token, &identity(), CUT, NOW,),
        Err(LearnerTransportError::ArtifactMismatch)
    );
    mark("wrong_operation_kind");
}

macro_rules! identity_mismatch_case {
    ($name:ident, $mutate:expr) => {
        #[test]
        fn $name() {
            let token = enroll_token();
            let mut expected = identity();
            $mutate(&mut expected);
            assert_eq!(
                VerifiedLearnerAdmission::from_published_membership_for_test(
                    &token, &expected, CUT, NOW,
                ),
                Err(LearnerTransportError::InvalidBinding)
            );
            mark(stringify!($name));
        }
    };
}

identity_mismatch_case!(wrong_domain, |value: &mut LearnerIdentity| value
    .trust_domain
    .push_str("-wrong"));

#[tokio::test]
async fn wrong_polis() {
    let (voter, _learner, voter_endpoint, learner_endpoint, _store) = live_learner_pair(1).await;
    let cut = live_voter_cut([3, 3, 3]);
    let cut_sha256 = route_cut_digest(&cut).unwrap();
    let mut learner = identity();
    learner.address = learner_endpoint.local_addr().unwrap();
    let now = i64::try_from(unix_now()).unwrap();
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        learner.clone(),
        cut_sha256,
        None,
        MEMBERSHIP,
        Some(now + 240),
        now + 300,
    )
    .unwrap();
    let mut wrong_publisher = authority_identity();
    wrong_publisher.polis_id = "polis-b".to_owned();
    let cross_polis_token = test_published_reconciliation_token(
        wrong_publisher,
        "cross-polis-enroll-4",
        artifact,
        41,
        CanonicalAuthorityTime {
            unix_seconds: now,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    assert_eq!(
        VerifiedLearnerAdmission::from_published_membership(
            &cross_polis_token,
            &learner,
            &cut,
            now,
        ),
        Err(LearnerTransportError::InvalidBinding)
    );
    assertion("wrong_polis", "cross_polis_published_result_denied");

    let (admission, admission_now) =
        live_admission(identity(), learner_endpoint.local_addr().unwrap(), &cut);
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    authority.activate_admission(&admission).unwrap();
    let cross_polis_cut = live_voter_cut_for("polis-b", [3, 3, 3]);
    let factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cross_polis_cut, authority).unwrap();
    assert!(matches!(
        factory
            .install_learner_route(
                4,
                Arc::clone(&voter),
                &admission,
                admission_now,
                &SigningKey::from_bytes(&[1; 32]),
            )
            .await,
        Err(PolisRuntimeError::AuthorityDenied)
    ));
    assertion("wrong_polis", "cross_polis_live_install_denied");
    voter.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    mark("wrong_polis");
}

identity_mismatch_case!(wrong_learner, |value: &mut LearnerIdentity| value
    .node_id
    .push_str("-wrong"));
identity_mismatch_case!(wrong_guardian, |value: &mut LearnerIdentity| value
    .guardian_id
    .push_str("-wrong"));
identity_mismatch_case!(
    wrong_certificate_generation,
    |value: &mut LearnerIdentity| value.certificate_generation += 1
);

#[test]
fn expired_certificate() {
    let token = enroll_token_with(identity(), "expired", 41, NOW, None);
    assert_eq!(
        VerifiedLearnerAdmission::from_published_membership_for_test(&token, &identity(), CUT, NOW,),
        Err(LearnerTransportError::InvalidBinding)
    );
    mark("expired_certificate");
}

#[test]
fn revoked_certificate() {
    let mut revoked = identity();
    revoked.certificate_generation = 0;
    assert_eq!(
        LearnerMembershipArtifact::enroll_non_voting(
            revoked,
            CUT,
            None,
            MEMBERSHIP,
            None,
            NOW + 100
        ),
        Err(LearnerTransportError::InvalidBinding)
    );
    mark("revoked_certificate");
}

#[tokio::test]
async fn wrong_boot_generation() {
    let token = enroll_token();
    let mut wrong_learner_boot = identity();
    wrong_learner_boot.boot_generation += 1;
    assert_eq!(
        VerifiedLearnerAdmission::from_published_membership_for_test(
            &token,
            &wrong_learner_boot,
            CUT,
            NOW,
        ),
        Err(LearnerTransportError::InvalidBinding)
    );

    let (voter, _learner, voter_endpoint, learner_endpoint, _store) = live_learner_pair(1).await;
    let current_cut = live_voter_cut([3, 3, 3]);
    let (admission, now) = live_admission(
        identity(),
        learner_endpoint.local_addr().unwrap(),
        &current_cut,
    );
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    authority.activate_admission(&admission).unwrap();
    let stale_boot_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, live_voter_cut([2, 3, 3]), authority)
            .unwrap();
    assert_eq!(
        stale_boot_factory
            .install_learner_route(
                4,
                Arc::clone(&voter),
                &admission,
                now,
                &SigningKey::from_bytes(&[1; 32]),
            )
            .await,
        Err(PolisRuntimeError::AuthorityDenied)
    );
    voter.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    assertion("wrong_boot_generation", "live_stale_voter_boot_rejected");
    mark("wrong_boot_generation");
}

#[tokio::test]
async fn stale_live_learner_boot_handshake_denied() {
    let (_voter, learner, voter_endpoint, learner_endpoint, _store) = live_learner_pair(1).await;
    let cut = live_voter_cut([3, 3, 3]);
    let (admission, now) = live_admission(identity(), learner_endpoint.local_addr().unwrap(), &cut);
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    authority.activate_admission(&admission).unwrap();
    let factory = SecurePolisNetworkFactory::from_authority_cut(1, cut, authority).unwrap();
    let stale = factory
        .learner_server_sessions(
            &learner,
            &admission,
            now,
            admission.identity().boot_generation - 1,
            &SigningKey::from_bytes(&[44; 32]),
        )
        .await;
    assert!(
        matches!(stale, Err(PolisRuntimeError::AuthorityDenied)),
        "unexpected stale boot result: {stale:?}"
    );
    assertion(
        "stale_live_learner_boot_handshake_denied",
        "live_boot_generation_must_match_signed_admission_binding",
    );
    learner.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    mark("stale_live_learner_boot_handshake_denied");
}

#[tokio::test]
async fn production_factory_boot_custody_current_then_stale_denied() {
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let boot_dir = portable_tempdir();
    let boot_authority = SecureBootGenerationAuthority::open(
        boot_dir.path(),
        4,
        checkpoint.clone() as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let generation_one = boot_authority.advance().unwrap();
    let local_identity = LocalNodeGuardianIdentity::generate("runtime-prod", 4).unwrap();
    let public = local_identity.public_identity();
    let mut learner = identity();
    learner.node_id = public.node_id.clone();
    learner.guardian_id = public.guardian_id.clone();
    learner.guardian_control_public_key = public.guardian_control_public_key;
    learner.certificate_generation = public.identity_generation;
    learner.boot_generation = generation_one.generation();
    let cut = live_voter_cut([3, 3, 3]);
    let (admission, now) = live_admission(learner, "127.0.0.1:4404".parse().unwrap(), &cut);
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        checkpoint.clone() as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let factory = SecurePolisNetworkFactory::from_authority_cut(1, cut, authority).unwrap();
    factory
        .activate_learner_admission(&admission, now)
        .await
        .unwrap();
    let attestation = factory
        .learner_boot_attestation(&local_identity, generation_one, &admission)
        .await
        .unwrap();
    assert!(attestation.require_current().is_ok());
    let generation_two = boot_authority.advance().unwrap();
    assert_eq!(
        generation_two.generation(),
        admission.identity().boot_generation + 1
    );
    assert!(matches!(
        attestation.require_current(),
        Err(LearnerTransportError::AuthorityDenied)
    ));
    assert!(matches!(
        attestation.sign(b"stale-generation-must-not-sign"),
        Err(LearnerTransportError::AuthorityDenied)
    ));
    assertion(
        "production_factory_boot_custody_current_then_stale_denied",
        "factory_attestation_rechecks_generation_under_signing_guard",
    );
    mark("production_factory_boot_custody_current_then_stale_denied");
}

#[tokio::test]
async fn wrong_address() {
    let (voter, _learner, voter_endpoint, learner_endpoint, _store) = live_learner_pair(1).await;
    let cut = live_voter_cut([3, 3, 3]);
    let (admission, now) = live_admission(identity(), "127.0.0.1:9999".parse().unwrap(), &cut);
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    authority.activate_admission(&admission).unwrap();
    let factory = SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), authority).unwrap();
    assert_eq!(
        factory
            .install_learner_route(
                4,
                Arc::clone(&voter),
                &admission,
                now,
                &SigningKey::from_bytes(&[1; 32]),
            )
            .await,
        Err(PolisRuntimeError::InvalidConfiguration)
    );
    let (direction_admission, direction_now) =
        live_admission(identity(), learner_endpoint.local_addr().unwrap(), &cut);
    let direction_dir = portable_tempdir();
    let direction_authority = ProductionLearnerAuthority::open(
        direction_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    direction_authority
        .activate_admission(&direction_admission)
        .unwrap();
    let direction_factory = SecurePolisNetworkFactory::from_authority_cut(
        1,
        live_voter_cut([3, 3, 3]),
        direction_authority,
    )
    .unwrap();
    assert!(direction_factory
        .learner_server_sessions(
            &voter,
            &direction_admission,
            direction_now,
            direction_admission.identity().boot_generation,
            &SigningKey::from_bytes(&[44; 32]),
        )
        .await
        .is_err());
    voter.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    assertion("wrong_address", "live_authorized_address_rejected");
    assertion("wrong_address", "live_wrong_direction_rejected");
    mark("wrong_address");
}

macro_rules! denied_case {
    ($name:ident, $method:ident) => {
        #[test]
        fn $name() {
            assert_eq!(
                session().$method(),
                Err(LearnerTransportError::AuthorityDenied)
            );
            mark(stringify!($name));
        }
    };
}

denied_case!(learner_vote_rpc_denied, vote);
denied_case!(learner_endorsement_denied, authority_endorse);
denied_case!(learner_finalize_denied, authority_finalize);
denied_case!(learner_mutation_denied, mutation);
denied_case!(learner_renewal_denied, renewal);
denied_case!(learner_shepherd_denied, shepherd);
denied_case!(learner_observatory_denied, observatory);

#[tokio::test]
async fn exclusion_ordinary_session_denied() {
    let dir = portable_tempdir();
    let authority =
        ProductionLearnerAuthority::open(dir.path(), Arc::new(MemoryCheckpoint::default()))
            .unwrap();
    let (membership, voter_authority, boots, signers) = endorsement_fixture();
    let excluded_public = signers[0].public_identity();
    let mut target = identity();
    target.node_id = excluded_public.node_id.clone();
    target.guardian_id = excluded_public.guardian_id.clone();
    target.guardian_control_public_key = excluded_public.guardian_control_public_key;
    let snapshot = authority
        .activate_exclusion(
            &remove_token_with(target.clone(), "remove-ordinary", 42),
            &target,
            CUT,
            MEMBERSHIP,
            NOW,
        )
        .unwrap();
    assert!(!snapshot.ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assertion(
        "exclusion_ordinary_session_denied",
        "published_exclusion_denies_retained_identity",
    );
    let intent = PrepareAuthorityIntent::new(
        "polis-a",
        &membership,
        &voter_authority,
        AuthorityOperationKind::ExistingStore,
        50,
        [9; 32],
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
        CanonicalAuthorityTime {
            unix_seconds: NOW + 50,
            nanos: 0,
            uncertainty_millis: 1,
        },
        "excluded-endorsement",
        CommittedAuthorityArtifact::new(
            AuthorityOperationKind::ExistingStore,
            b"excluded-endorsement".to_vec(),
        )
        .unwrap(),
    )
    .unwrap();
    let finalization_time = CanonicalAuthorityTime {
        unix_seconds: NOW + 1,
        nanos: 0,
        uncertainty_millis: 1,
    };
    assert_eq!(
        authority.endorse_committed_prepare(
            &signers[0],
            4,
            4,
            membership.committed_log_index(),
            &boots,
            &intent,
            &finalization_time,
            &membership,
            &voter_authority,
        ),
        Err(AuthorityProtocolError::WrongVoter)
    );
    assert!(authority
        .endorse_committed_prepare(
            &signers[1],
            4,
            4,
            membership.committed_log_index(),
            &boots,
            &intent,
            &finalization_time,
            &membership,
            &voter_authority,
        )
        .is_ok());
    assertion(
        "exclusion_ordinary_session_denied",
        "production_endorsement_uses_durable_exclusion",
    );

    let (voter_connection, learner_connection, voter_endpoint, learner_endpoint, _store) =
        live_learner_pair(1).await;
    let routes = [
        (4, "127.0.0.1:45404".parse().unwrap()),
        (1, learner_endpoint.local_addr().unwrap()),
        (3, "127.0.0.1:45303".parse().unwrap()),
    ]
    .into_iter()
    .collect();
    let identities = [
        (
            4,
            (
                "node-1".to_owned(),
                "guardian-1".to_owned(),
                SigningKey::from_bytes(&[1; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
        (
            1,
            (
                "node-4".to_owned(),
                "guardian-4".to_owned(),
                SigningKey::from_bytes(&[4; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
        (
            3,
            (
                "node-3".to_owned(),
                "guardian-3".to_owned(),
                SigningKey::from_bytes(&[3; 32]).verifying_key().to_bytes(),
                4,
            ),
        ),
    ]
    .into_iter()
    .collect();
    let cut = VerifiedPolisRouteCut::test_from_parts("polis-a", "runtime-prod", routes, identities);
    let cut_sha256 = route_cut_digest(&cut).unwrap();
    let session_authority_dir = portable_tempdir();
    let session_authority = ProductionLearnerAuthority::open(
        session_authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let learner_session_authority_dir = portable_tempdir();
    let learner_session_authority = ProductionLearnerAuthority::open(
        learner_session_authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let voter_factory =
        SecurePolisNetworkFactory::from_authority_cut(4, cut.clone(), session_authority.clone())
            .unwrap();
    let learner_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut, learner_session_authority).unwrap();
    let voter_signing_key = SigningKey::from_bytes(&[1; 32]);
    let learner_signing_key = SigningKey::from_bytes(&[4; 32]);
    let (voter_session, learner_session) = tokio::join!(
        voter_factory.initiate_session(1, &voter_connection, &voter_signing_key,),
        learner_factory.accept_session(4, &learner_connection, &learner_signing_key,),
    );
    let voter_session = voter_session.unwrap();
    let learner_session = learner_session.unwrap();
    let mut excluded = identity();
    excluded.stable_raft_id = 1;
    excluded.certificate_generation = 1;
    excluded.boot_generation = 4;
    excluded.guardian_control_public_key =
        SigningKey::from_bytes(&[4; 32]).verifying_key().to_bytes();
    excluded.address = learner_endpoint.local_addr().unwrap();
    let artifact = LearnerMembershipArtifact::remove_voter(
        excluded.clone(),
        cut_sha256,
        MEMBERSHIP,
        NOW + 100,
        "exclude_retained_session",
    )
    .unwrap();
    let mut publisher = authority_identity();
    publisher.boot_generation = 4;
    let removal = test_published_reconciliation_token(
        publisher,
        "exclude-retained-session",
        artifact,
        42,
        CanonicalAuthorityTime {
            unix_seconds: NOW,
            nanos: 0,
            uncertainty_millis: 1,
        },
    );
    let hook = session_authority.install_dispatch_pause_for_test("begin_polis_request");
    let stream_frames_before_inflight = voter_connection.test_stream_frames_sent();
    let dispatch_connection = Arc::clone(&voter_connection);
    let dispatch_session = voter_session.clone();
    let dispatch = tokio::spawn(async move {
        dispatch_connection
            .begin_polis_request(
                &dispatch_session,
                1,
                "append_entries",
                b"in-flight-before-exclusion".to_vec(),
            )
            .await
    });
    hook.reached.notified().await;
    let activation_factory = voter_factory.clone();
    let activation_identity = excluded.clone();
    let learner_removal = removal.clone();
    let activation = tokio::spawn(async move {
        activation_factory
            .activate_pending_exclusion(&removal, &activation_identity, cut_sha256, MEMBERSHIP, NOW)
            .await
    });
    for _ in 0..8 {
        tokio::task::yield_now().await;
    }
    assert!(
        !activation.is_finished(),
        "exclusion crossed a governed request between revalidation and stream creation"
    );
    hook.release.notify_one();
    let pending = dispatch
        .await
        .expect("in-flight dispatch joined")
        .expect("pre-exclusion dispatch retained its shared fence");
    drop(pending);
    activation
        .await
        .expect("exclusion activation joined")
        .expect("exclusion activated after in-flight dispatch");
    learner_factory
        .activate_pending_exclusion(&learner_removal, &excluded, cut_sha256, MEMBERSHIP, NOW)
        .await
        .expect("peer exclusion view activated");
    assert!(
        voter_connection.test_stream_frames_sent() > stream_frames_before_inflight,
        "the deliberately retained pre-exclusion dispatch did not open its stream"
    );
    let stream_frames_before = voter_connection.test_stream_frames_sent();
    assert_eq!(
        voter_connection
            .request_polis(&voter_session, 2, "append_entries", b"denied".to_vec())
            .await,
        Err(TransportError::InvalidSessionBinding)
    );
    assert!(matches!(
        voter_connection
            .begin_polis_request(&voter_session, 3, "install_snapshot", b"denied".to_vec())
            .await,
        Err(TransportError::InvalidSessionBinding)
    ));
    assert!(matches!(
        voter_factory
            .request_on_connection(
                1,
                &voter_connection,
                &voter_session,
                4,
                "append_entries",
                b"denied".to_vec(),
            )
            .await,
        Err(PolisRuntimeError::AuthorityDenied)
    ));
    assert_eq!(
        voter_connection.test_stream_frames_sent(),
        stream_frames_before,
        "excluded retained session emitted a STREAM frame"
    );
    assert!(matches!(
        learner_connection
            .accept_polis_request(&learner_session)
            .await,
        Err(TransportError::InvalidSessionBinding)
    ));
    assertion(
        "exclusion_ordinary_session_denied",
        "retained_excluded_session_zero_bytes_all_public_dispatch",
    );
    assertion(
        "exclusion_ordinary_session_denied",
        "actual_request_stream_fenced_against_exclusion_race",
    );
    voter_connection.close();
    learner_connection.close();
    voter_endpoint.close(0_u32.into(), b"test complete");
    learner_endpoint.close(0_u32.into(), b"test complete");
    mark("exclusion_ordinary_session_denied");
}

#[tokio::test]
async fn exclusion_exact_publisher_and_target_required() {
    let cut = live_voter_cut([3, 3, 3]);
    let cut_sha256 = route_cut_digest(&cut).unwrap();
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let factory = SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), authority).unwrap();
    let target = exact_voter_target(&cut, 2);
    let mut wrong_publisher = exact_publisher(&cut, 1);
    wrong_publisher.node_id.push_str("-forged");
    let wrong_publisher_result =
        live_removal(&cut, &target, wrong_publisher, "wrong-publisher-node");
    assert_eq!(
        factory
            .activate_pending_exclusion(
                &wrong_publisher_result,
                &target,
                cut_sha256,
                MEMBERSHIP,
                NOW,
            )
            .await,
        Err(PolisRuntimeError::AuthorityDenied)
    );

    let mut wrong_target = target.clone();
    wrong_target.certificate_generation += 1;
    wrong_target.boot_generation += 1;
    let wrong_target_result = live_removal(
        &cut,
        &wrong_target,
        exact_publisher(&cut, 1),
        "wrong-removal-target",
    );
    assert_eq!(
        factory
            .activate_pending_exclusion(
                &wrong_target_result,
                &wrong_target,
                cut_sha256,
                MEMBERSHIP,
                NOW,
            )
            .await,
        Err(PolisRuntimeError::AuthorityDenied)
    );
    assertion(
        "exclusion_exact_publisher_and_target_required",
        "wrong_publisher_node_denied",
    );
    assertion(
        "exclusion_exact_publisher_and_target_required",
        "wrong_target_certificate_and_boot_denied",
    );
    mark("exclusion_exact_publisher_and_target_required");
}

#[tokio::test]
async fn exclusion_waits_for_inflight_dispatch_fence() {
    let cut = live_voter_cut([3, 3, 3]);
    let cut_sha256 = route_cut_digest(&cut).unwrap();
    let authority_dir = portable_tempdir();
    let authority = ProductionLearnerAuthority::open(
        authority_dir.path(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), authority.clone()).unwrap();
    let target = exact_voter_target(&cut, 2);
    let removal = live_removal(
        &cut,
        &target,
        exact_publisher(&cut, 1),
        "dispatch-fence-race",
    );
    let in_flight = authority.dispatch_guard().await;
    let task = tokio::spawn(async move {
        factory
            .activate_pending_exclusion(&removal, &target, cut_sha256, MEMBERSHIP, NOW)
            .await
    });
    for _ in 0..8 {
        tokio::task::yield_now().await;
    }
    assert!(
        !task.is_finished(),
        "exclusion committed while a dispatch guard was retained"
    );
    drop(in_flight);
    assert!(tokio::time::timeout(Duration::from_secs(2), task)
        .await
        .expect("exclusive fence made progress")
        .expect("exclusion task joined")
        .is_ok());
    assertion(
        "exclusion_waits_for_inflight_dispatch_fence",
        "exclusive_exclusion_waits_for_shared_dispatch",
    );
    mark("exclusion_waits_for_inflight_dispatch_fence");
}

#[test]
fn exclusion_wrong_recovery_token() {
    let (_dir, _checkpoint, mut authority) = exclusion();
    let target = identity();
    let snapshot = authority
        .activate(
            &remove_token_with(target.clone(), "remove-wrong-recovery", 42),
            &target,
            CUT,
            MEMBERSHIP,
            NOW,
        )
        .unwrap();
    assert!(!snapshot.recovery_learner_allowed(&admission()));
    mark("exclusion_wrong_recovery_token");
}

#[test]
fn stale_admission() {
    let mut value = session();
    assert_eq!(
        value.authorize(LearnerRpcKind::AppendEntries, 1, b"x", NOW + 100),
        Err(LearnerTransportError::Expired)
    );
    mark("stale_admission");
}

#[test]
fn replay_conflict() {
    let mut value = session();
    assert!(value
        .authorize(LearnerRpcKind::AppendEntries, 1, b"x", NOW)
        .is_ok());
    assert_eq!(
        value.authorize(LearnerRpcKind::AppendEntries, 1, b"different", NOW),
        Err(LearnerTransportError::Replay)
    );
    mark("replay_conflict");
}

#[test]
fn oversized_frame() {
    let mut value = session();
    let payload = vec![0; MAX_LEARNER_RPC_BYTES + 1];
    assert_eq!(
        value.authorize(LearnerRpcKind::AppendEntries, 1, &payload, NOW),
        Err(LearnerTransportError::FrameTooLarge)
    );
    mark("oversized_frame");
}

#[test]
fn truncated_frame() {
    let mut value = session();
    let full = value
        .authorize(LearnerRpcKind::InstallSnapshot, 1, b"canonical", NOW)
        .unwrap();
    let mut other = session();
    let truncated = other
        .authorize(LearnerRpcKind::InstallSnapshot, 1, b"canonica", NOW)
        .unwrap();
    assert_ne!(full, truncated);
    mark("truncated_frame");
}

#[test]
fn capacity_n_plus_one_no_partial() {
    let (_dir, _checkpoint, mut authority) = exclusion();
    let first = identity();
    authority
        .activate(
            &remove_token_with(first.clone(), "remove-first", 42),
            &first,
            CUT,
            MEMBERSHIP,
            NOW,
        )
        .unwrap();
    let mut second = first.clone();
    second.node_id = "node-5".to_owned();
    second.guardian_id = "guardian-5".to_owned();
    second.stable_raft_id = 5;
    assert_eq!(
        authority.activate(
            &remove_token_with(second.clone(), "remove-second", 43),
            &second,
            CUT,
            MEMBERSHIP,
            NOW
        ),
        Err(LearnerTransportError::CapacityExceeded)
    );
    assert!(!authority
        .snapshot()
        .ordinary_authority_allowed(&first.node_id, &first.guardian_id));
    assert!(authority
        .snapshot()
        .ordinary_authority_allowed(&second.node_id, &second.guardian_id));
    mark("capacity_n_plus_one_no_partial");
}

#[test]
fn crash_before_exclusion_checkpoint() {
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let authority = ProductionLearnerAuthority::open(
        dir.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    *checkpoint.fail_next.lock().unwrap() = true;
    assert_eq!(
        authority.activate_admission(&admission()),
        Err(LearnerTransportError::Replay)
    );
    *checkpoint.fail_next.lock().unwrap() = true;
    let target = identity();
    assert_eq!(
        authority.activate_exclusion(
            &remove_token_with(target.clone(), "remove-crash-before", 42),
            &target,
            CUT,
            MEMBERSHIP,
            NOW
        ),
        Err(LearnerTransportError::Replay)
    );
    drop(authority);
    assert!(dir.path().join(".learner-admission.json.journal").exists());
    assert!(dir
        .path()
        .join(".pending-membership-exclusion.json.journal")
        .exists());
    let recovered = ProductionLearnerAuthority::open(
        dir.path(),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert!(recovered.admission_snapshot().unwrap().current().is_none());
    assert!(recovered
        .exclusion_snapshot()
        .unwrap()
        .ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assert!(!dir.path().join(".learner-admission.json.journal").exists());
    assert!(!dir
        .path()
        .join(".pending-membership-exclusion.json.journal")
        .exists());
    assertion(
        "crash_before_exclusion_checkpoint",
        "failed_admission_and_exclusion_cas_recover_old_view",
    );
    mark("crash_before_exclusion_checkpoint");
}

#[test]
fn crash_after_exclusion_checkpoint() {
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let target = identity();
    {
        let authority = ProductionLearnerAuthority::open(
            dir.path(),
            Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
        )
        .unwrap();
        authority.activate_admission(&admission()).unwrap();
        authority
            .activate_exclusion(
                &remove_token_with(target.clone(), "remove-crash-after", 42),
                &target,
                CUT,
                MEMBERSHIP,
                NOW,
            )
            .unwrap();
    }
    let recovered = ProductionLearnerAuthority::open(
        dir.path(),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert_eq!(
        recovered
            .admission_snapshot()
            .unwrap()
            .current()
            .unwrap()
            .operation_sha256,
        admission().operation_sha256
    );
    assert!(!recovered
        .exclusion_snapshot()
        .unwrap()
        .ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assert!(dir.path().join("learner-admission.json").exists());
    assert!(dir
        .path()
        .join("pending-membership-exclusion.json")
        .exists());
    assert!(!dir.path().join(".learner-admission.json.journal").exists());
    assert!(!dir
        .path()
        .join(".pending-membership-exclusion.json.journal")
        .exists());
    assertion(
        "crash_after_exclusion_checkpoint",
        "committed_admission_and_exclusion_survive_restart",
    );
    mark("crash_after_exclusion_checkpoint");
}

#[cfg(unix)]
#[test]
fn state_or_lock_symlink_rejected() {
    use std::os::unix::fs::symlink;
    let dir = portable_tempdir();
    let outside = portable_tempdir();
    symlink(outside.path(), dir.path().join("linked")).unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let result = PendingMembershipExclusionAuthority::open(
        &dir.path().join("linked"),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    );
    assert!(result.is_err());
    assert!(fs::read_dir(outside.path()).unwrap().next().is_none());
    mark("state_or_lock_symlink_rejected");
}
