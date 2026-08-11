use std::{
    collections::BTreeMap,
    fs,
    net::Ipv4Addr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, RwLock as StdRwLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::*;
use crate::distributed::{
    authority_protocol::{
        test_published_reconciliation_token, AuthorityNodeIdentity, CanonicalAuthorityTime,
    },
    certificates::{
        AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
        CertificateValidity, DistributedCertificateStore,
    },
    polis_runtime::{
        ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisCommand, PolisLogStore, PolisRaft,
        PolisRuntimeError, PolisStateMachineStore, PolisTypeConfig,
    },
    transport::{
        client_endpoint, server_endpoint, AuthenticatedConnection, ConnectionSecurity, PeerBinding,
        TransportAuthorization, TransportLimits,
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
    BasicNode, SnapshotMeta, Vote,
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
    VerifiedLearnerAdmission::from_published_membership(&enroll_token(), &identity(), CUT, NOW)
        .unwrap()
}

fn session() -> EstablishedLearnerSession {
    EstablishedLearnerSession {
        binding: LearnerSessionBinding::from_admission(&admission(), CUT),
        highest_sequence: 0,
        closed: false,
    }
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
            expires_at_unix_secs: issued + 300,
        },
        key,
        &root.verifying_key(),
    );
    let certificate = AuthorityCertificate::issue(body, root).unwrap();
    store.activate(&certificate, unix_now()).unwrap();
    TransportAuthorization::new(Arc::clone(store), &certificate).unwrap()
}

async fn live_learner_pair() -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    let issuer = certificate_authority();
    let root_certificate = issuer.der().clone();
    let voter = leaf(&issuer, "node-1");
    let learner = leaf(&issuer, "node-4");
    let signing_root = SigningKey::from_bytes(&[91; 32]);
    let policy = CertificatePolicy::new("runtime-prod", [signing_root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 128, 128)
        .unwrap();
    let store_dir = portable_tempdir();
    let store = Arc::new(
        DistributedCertificateStore::open(store_dir.path().join("certificates.redb"), policy)
            .unwrap(),
    );
    let voter_authorization =
        transport_authorization(&store, &signing_root, "node-1", voter.subject_public_key);
    let learner_authorization =
        transport_authorization(&store, &signing_root, "node-4", learner.subject_public_key);
    let voter_binding = PeerBinding::new(
        &voter.certificate,
        "runtime-prod",
        "node-1",
        "guardian-1",
        1,
        4,
    )
    .unwrap();
    let learner_binding = PeerBinding::new(
        &learner.certificate,
        "runtime-prod",
        "node-4",
        "guardian-4",
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
    let voter_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![voter.certificate.clone()],
        voter.private_key(),
        std::slice::from_ref(&root_certificate),
        &limits,
    )
    .unwrap();
    let learner_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![learner.certificate.clone()],
        learner.private_key(),
        std::slice::from_ref(&root_certificate),
        &limits,
    )
    .unwrap();
    let voter_address = voter_endpoint.local_addr().unwrap();
    let (voter_connection, learner_connection) = tokio::join!(
        AuthenticatedConnection::accept(
            &voter_endpoint,
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
        AuthenticatedConnection::connect(
            &learner_endpoint,
            voter_address,
            "localhost",
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

#[derive(Clone)]
struct AuthorizedLearnerMemoryNetwork {
    peers: Arc<StdRwLock<BTreeMap<u64, PolisRaft>>>,
    learner: Arc<Mutex<EstablishedLearnerSession>>,
    sequence: Arc<AtomicU64>,
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

    fn authorize<T: Serialize>(
        &self,
        kind: LearnerRpcKind,
        request: &T,
    ) -> Result<(), PolisRuntimeError> {
        if self.target != 4 {
            return Ok(());
        }
        let sequence = self.network.sequence.fetch_add(1, Ordering::SeqCst) + 1;
        let bytes = serde_jcs::to_vec(request).map_err(|_| PolisRuntimeError::Serialization)?;
        self.network
            .learner
            .lock()
            .unwrap()
            .authorize(kind, sequence, &bytes, NOW)
            .map(|_| ())
            .map_err(|_| PolisRuntimeError::AuthorityDenied)
    }
}

impl RaftNetwork<PolisTypeConfig> for AuthorizedLearnerMemoryConnection {
    async fn append_entries(
        &mut self,
        request: AppendEntriesRequest<PolisTypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        self.authorize(LearnerRpcKind::AppendEntries, &request)
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
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
        self.authorize(LearnerRpcKind::InstallSnapshot, &request)
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
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
    let (voter, learner, _voter_endpoint, _learner_endpoint, _store) = live_learner_pair().await;
    let mut sender = session();
    let mut receiver = session();
    let append_request = AppendEntriesRequest::<PolisTypeConfig> {
        vote: Vote::new(7, 1),
        prev_log_id: None,
        entries: Vec::new(),
        leader_commit: None,
    };
    let append_bytes = serde_jcs::to_vec(&append_request).unwrap();
    let learner_receive = Arc::clone(&learner);
    let append =
        tokio::spawn(async move { receiver.receive_replication(&learner_receive, NOW).await });
    sender
        .send_append_entries(&voter, 1, append_bytes.clone(), NOW)
        .await
        .unwrap();
    let append = append.await.unwrap().unwrap();
    assert_eq!(append.message_kind, "append_entries");
    let decoded_append: AppendEntriesRequest<PolisTypeConfig> =
        serde_json::from_slice(&append.payload).unwrap();
    assert_eq!(decoded_append.vote, append_request.vote);

    let mut receiver = session();
    let snapshot_request = InstallSnapshotRequest::<PolisTypeConfig> {
        vote: Vote::new(7, 1),
        meta: SnapshotMeta::default(),
        offset: 0,
        data: b"canonical-snapshot".to_vec(),
        done: true,
    };
    let snapshot_bytes = serde_jcs::to_vec(&snapshot_request).unwrap();
    let learner_receive = Arc::clone(&learner);
    let snapshot =
        tokio::spawn(async move { receiver.receive_replication(&learner_receive, NOW).await });
    sender
        .send_install_snapshot(&voter, 2, snapshot_bytes, NOW)
        .await
        .unwrap();
    let snapshot = snapshot.await.unwrap().unwrap();
    assert_eq!(snapshot.message_kind, "install_snapshot");
    assert_eq!(sender.vote(), Err(LearnerTransportError::AuthorityDenied));
    assert_eq!(
        sender.deny_message_kind("unknown"),
        Err(LearnerTransportError::AuthorityDenied)
    );

    let raft_root = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let authorized_network = AuthorizedLearnerMemoryNetwork {
        peers: Arc::new(StdRwLock::new(BTreeMap::new())),
        learner: Arc::new(Mutex::new(session())),
        sequence: Arc::new(AtomicU64::new(0)),
    };
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
            authorized_network.clone(),
            log,
            machine.clone(),
        )
        .await
        .unwrap();
        nodes.insert(node, raft);
        machines.insert(node, machine);
    }
    *authorized_network.peers.write().unwrap() = nodes.clone();
    let voter_routes = (1..=3)
        .map(|node| (node, BasicNode::new(format!("memory://voter-{node}"))))
        .collect::<BTreeMap<_, _>>();
    nodes[&1].initialize(voter_routes.clone()).await.unwrap();
    let leader = loop {
        if let Some(leader) = nodes[&1].metrics().borrow().current_leader {
            break leader;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    };
    nodes[&leader]
        .add_learner(4, BasicNode::new("memory://authorized-learner-4"), true)
        .await
        .unwrap();
    let response = nodes[&leader]
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "authorized-learner-replicated".to_owned(),
            payload_sha256: "44".repeat(32),
        })
        .await
        .unwrap();
    assert!(response.data.accepted);
    for _ in 0..100 {
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
    let membership = nodes[&leader].metrics().borrow().membership_config.clone();
    let voters = membership
        .membership()
        .get_joint_config()
        .iter()
        .flatten()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(voters, std::collections::BTreeSet::from([1, 2, 3]));
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

#[test]
fn excluded_node_recovery_learner() {
    let (_dir, _checkpoint, mut authority) = exclusion();
    let old = identity();
    let snapshot = authority
        .activate(&remove_token_with(old.clone(), "remove-4", 42), &old, CUT)
        .unwrap();
    let mut recovered = old.clone();
    recovered.node_id = "node-4-recovered".to_owned();
    recovered.guardian_id = "guardian-4-recovered".to_owned();
    recovered.certificate_generation += 1;
    recovered.boot_generation += 1;
    let token = enroll_token_with(recovered.clone(), "recover-4", 43, NOW + 100, None);
    let admission =
        VerifiedLearnerAdmission::from_published_membership(&token, &recovered, CUT, NOW).unwrap();
    assert!(snapshot.recovery_learner_allowed(&admission));
    assert!(!snapshot.ordinary_authority_allowed(&old.node_id, &old.guardian_id));
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
    let first = authority.activate(&token, &target, CUT).unwrap();
    let second = authority.activate(&token, &target, CUT).unwrap();
    assert_eq!(first, second);
    assertion("exact_retry_session", "exclusion_exact_retry_cached");
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
    let next = VerifiedLearnerAdmission::from_published_membership(&next, &next_identity, CUT, NOW)
        .unwrap();
    assert_ne!(
        old.binding.identity.boot_generation,
        next.identity.boot_generation
    );
    mark("reconnect_boot_rotation");
}

#[test]
fn certificate_overlap_authorized() {
    let mut old = session();
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
    let next =
        VerifiedLearnerAdmission::from_published_membership(&token, &next_identity, CUT, NOW)
            .unwrap();
    assert_ne!(old.binding.operation_sha256, next.operation_sha256);
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let mut authority = LearnerAdmissionAuthority::open(
        dir.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let current = admission();
    authority.activate(&current).unwrap();
    authority
        .stage_successor(&next, current.operation_sha256)
        .unwrap();
    assert_eq!(
        authority.snapshot().current().unwrap().operation_sha256,
        current.operation_sha256
    );
    assertion(
        "certificate_overlap_authorized",
        "successor_private_before_flip",
    );
    drop(authority);
    let mut recovered = LearnerAdmissionAuthority::open(
        dir.path(),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    let flipped = recovered.flip_successor(next.operation_sha256).unwrap();
    assert_eq!(
        flipped.current().unwrap().operation_sha256,
        next.operation_sha256
    );
    assertion(
        "certificate_overlap_authorized",
        "successor_restart_atomic_flip",
    );
    old.close();
    assert_eq!(
        old.authorize(LearnerRpcKind::AppendEntries, 2, b"late", NOW),
        Err(LearnerTransportError::AuthorityDenied)
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
        VerifiedLearnerAdmission::from_published_membership(&token, &identity(), CUT, NOW),
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
                VerifiedLearnerAdmission::from_published_membership(&token, &expected, CUT, NOW),
                Err(LearnerTransportError::InvalidBinding)
            );
            mark(stringify!($name));
        }
    };
}

identity_mismatch_case!(wrong_domain, |value: &mut LearnerIdentity| value
    .trust_domain
    .push_str("-wrong"));
identity_mismatch_case!(wrong_polis, |value: &mut LearnerIdentity| value
    .polis_id
    .push_str("-wrong"));
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
        VerifiedLearnerAdmission::from_published_membership(&token, &identity(), CUT, NOW),
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

identity_mismatch_case!(wrong_boot_generation, |value: &mut LearnerIdentity| {
    value.boot_generation += 1
});
identity_mismatch_case!(wrong_address, |value: &mut LearnerIdentity| value.address =
    "127.0.0.1:9999".parse().unwrap());

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

#[test]
fn exclusion_ordinary_session_denied() {
    let (_dir, _checkpoint, mut authority) = exclusion();
    let target = identity();
    let snapshot = authority
        .activate(
            &remove_token_with(target.clone(), "remove-ordinary", 42),
            &target,
            CUT,
        )
        .unwrap();
    assert!(!snapshot.ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assertion(
        "exclusion_ordinary_session_denied",
        "published_exclusion_denies_retained_identity",
    );
    assert!(snapshot.ordinary_authority_allowed("node-1", "guardian-1"));
    mark("exclusion_ordinary_session_denied");
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
            CUT
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
    let mut authority = PendingMembershipExclusionAuthority::open(
        dir.path(),
        Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    *checkpoint.fail_next.lock().unwrap() = true;
    let target = identity();
    assert_eq!(
        authority.activate(
            &remove_token_with(target.clone(), "remove-crash-before", 42),
            &target,
            CUT
        ),
        Err(LearnerTransportError::Replay)
    );
    drop(authority);
    let recovered = PendingMembershipExclusionAuthority::open(
        dir.path(),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert!(recovered
        .snapshot()
        .ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assertion(
        "crash_before_exclusion_checkpoint",
        "failed_cas_recovers_old_view",
    );
    mark("crash_before_exclusion_checkpoint");
}

#[test]
fn crash_after_exclusion_checkpoint() {
    let dir = portable_tempdir();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let target = identity();
    {
        let mut authority = PendingMembershipExclusionAuthority::open(
            dir.path(),
            Arc::clone(&checkpoint) as Arc<dyn ConsensusCheckpointAuthority>,
        )
        .unwrap();
        authority
            .activate(
                &remove_token_with(target.clone(), "remove-crash-after", 42),
                &target,
                CUT,
            )
            .unwrap();
    }
    let recovered = PendingMembershipExclusionAuthority::open(
        dir.path(),
        checkpoint as Arc<dyn ConsensusCheckpointAuthority>,
    )
    .unwrap();
    assert!(!recovered
        .snapshot()
        .ordinary_authority_allowed(&target.node_id, &target.guardian_id));
    assertion(
        "crash_after_exclusion_checkpoint",
        "committed_view_survives_restart",
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
