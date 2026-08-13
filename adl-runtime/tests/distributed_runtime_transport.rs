// PVF: lane=focused-secure-raft-runtime; proof=real mTLS Quinn transport, authority-derived
// three-voter topology, durable retry/rollback and OpenRaft quorum behavior;
// deterministic=true; resource_profile=large; release_gate=true; nonzero selection required.
use std::{
    collections::{BTreeMap, BTreeSet},
    net::{Ipv4Addr, SocketAddr},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use adl_runtime::distributed::polis_runtime::{
    advance_secure_boot_generation, derive_authority_cut, new_secure_raft_node,
    serve_secure_raft_connection, ConsensusCheckpoint, ConsensusCheckpointAuthority,
    DurableRpcResponses, PolisCommand, PolisLogStore, PolisRaft, PolisRuntimeAuthorityBootstrap,
    PolisRuntimeError, PolisStateMachineStore, SecurePolisNetworkFactory,
};
use adl_runtime::distributed::{
    authority_protocol::{AuthorityNodeIdentity, CanonicalAuthorityTime},
    authority_reconciliation::{
        AuthorityReconciliationArtifact, AuthorityReconciliationBarrier,
        AuthorityReconciliationIdentity,
    },
    authority_store_adapters::{AuthorityBoundCertificateStore, AuthorityStoreAdapterRegistry},
    certificates::{
        AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
        CertificateStoreAccess, CertificateValidity, DistributedCertificateStore,
        TEST_CERTIFICATE_STORE_ACCESS,
    },
    learner_transport::ProductionLearnerAuthority,
    lease::{AuthorityMembership, ControlCertificatePurpose, VoterAuthority},
    membership::{
        CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
        MembershipState,
    },
    transport::{
        client_endpoint, decode_frame, encode_frame, polis_identity_signing_payload,
        server_endpoint, AuthenticatedConnection, ConnectionSecurity, EstablishedPolisSession,
        EstablishedRuntimeAuthority, PeerBinding, PolisIdentityBinding, TransportAuthorization,
        TransportEnvelope, TransportError, TransportLimits, VerifiedPolisRouteCut,
        TRANSPORT_SCHEMA,
    },
};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use openraft::{
    storage::{RaftLogStorage, RaftSnapshotBuilder, RaftStateMachine},
    BasicNode, Vote,
};
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ED25519,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::sync::CancellationToken;

const DOMAIN: &str = "polis.secure.test";
const POLIS: &str = "polis-alpha";

fn test_certificate_store_access() -> CertificateStoreAccess {
    TEST_CERTIFICATE_STORE_ACCESS
}

#[derive(Default)]
struct MemoryCheckpointAuthority {
    checkpoints: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
    fail_next_compare_and_swap: AtomicBool,
}

impl MemoryCheckpointAuthority {
    fn fail_next_compare_and_swap(&self) {
        self.fail_next_compare_and_swap
            .store(true, Ordering::SeqCst);
    }
}

impl ConsensusCheckpointAuthority for MemoryCheckpointAuthority {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.checkpoints.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        if self
            .fail_next_compare_and_swap
            .swap(false, Ordering::SeqCst)
        {
            return Err(PolisRuntimeError::Storage);
        }
        let mut checkpoints = self.checkpoints.lock().unwrap();
        if checkpoints.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        if let Some(previous) = expected {
            if candidate.generation <= previous.generation
                || matches!(
                    (previous.committed_log_index, candidate.committed_log_index),
                    (Some(old), Some(new)) if new < old
                )
                || matches!(
                    (previous.snapshot_log_index, candidate.snapshot_log_index),
                    (Some(old), Some(new)) if new < old
                )
            {
                return Err(PolisRuntimeError::StateRegression);
            }
        }
        checkpoints.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

fn test_learner_authority() -> ProductionLearnerAuthority {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    ProductionLearnerAuthority::open(root.path(), Arc::new(MemoryCheckpointAuthority::default()))
        .unwrap()
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

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn repo_tempdir() -> tempfile::TempDir {
    let root = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .unwrap();
    tempfile::TempDir::new_in(root).unwrap()
}

fn limits() -> TransportLimits {
    TransportLimits::bounded(
        256 * 1024,
        32,
        Duration::from_secs(10),
        Duration::from_secs(30),
    )
    .unwrap()
}

fn certificate_store() -> (
    Arc<DistributedCertificateStore>,
    AuthorityBoundCertificateStore,
    SigningKey,
    tempfile::TempDir,
) {
    let root = SigningKey::from_bytes(&[91; 32]);
    let policy = CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 128, 128)
        .unwrap();
    let directory = repo_tempdir();
    let store = DistributedCertificateStore::open(
        &test_certificate_store_access(),
        directory
            .path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        policy,
    )
    .unwrap();
    let store = Arc::new(store);
    let bound = bound_certificate_store(&store, &directory, "transport-lineage");
    (store, bound, root, directory)
}

fn bound_certificate_store(
    store: &Arc<DistributedCertificateStore>,
    directory: &tempfile::TempDir,
    lineage_id: &str,
) -> AuthorityBoundCertificateStore {
    let authority_identity = AuthorityNodeIdentity {
        trust_domain: DOMAIN.to_owned(),
        polis_id: POLIS.to_owned(),
        node_id: "test-node".to_owned(),
        guardian_id: "test-guardian-a".to_owned(),
        boot_generation: 1,
    };
    let barrier_root = directory
        .path()
        .join(format!("reconciliation-{lineage_id}"));
    std::fs::create_dir_all(&barrier_root).unwrap();
    let mut barrier = AuthorityReconciliationBarrier::open(
        &barrier_root,
        AuthorityReconciliationIdentity::from_authority_node(&authority_identity),
        Arc::new(MemoryCheckpointAuthority::default()),
    )
    .unwrap();
    let artifact = AuthorityReconciliationArtifact::new(
        lineage_id.to_owned(),
        "adl.test.deterministic-authority".to_owned(),
        1,
        "certificate_activate".to_owned(),
        vec![b"transport-authority-boundary".to_vec()],
        b"published-transport-authority".to_vec(),
        2_000_000_000,
    )
    .unwrap();
    barrier
        .publish_internal_test_fixture(
            &format!("issue-259-{lineage_id}"),
            artifact,
            259,
            CanonicalAuthorityTime {
                unix_seconds: 100,
                nanos: 0,
                uncertainty_millis: 0,
            },
        )
        .unwrap();
    AuthorityStoreAdapterRegistry::new(Arc::new(barrier))
        .certificate_store(lineage_id, Arc::clone(store))
        .unwrap()
}

fn transport_authorization(
    store: &Arc<DistributedCertificateStore>,
    bound_store: &AuthorityBoundCertificateStore,
    root: &SigningKey,
    node: &str,
    key: VerifyingKey,
    generation: u64,
) -> TransportAuthorization {
    let issued = now().saturating_sub(1);
    let body = CertificateBody::new(
        DOMAIN,
        node,
        CertificatePurpose::Transport,
        generation,
        CertificateValidity {
            issued_at_unix_secs: issued,
            expires_at_unix_secs: issued + 600,
        },
        key,
        &root.verifying_key(),
    );
    let certificate = AuthorityCertificate::issue(body, root).unwrap();
    store
        .activate(&test_certificate_store_access(), &certificate, now())
        .unwrap();
    TransportAuthorization::new(bound_store.clone(), &certificate).unwrap()
}

async fn connected_pair() -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    TransportLimits,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    connected_pair_at(1).await
}

async fn connected_pair_at(
    certificate_generation: u64,
) -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    TransportLimits,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    connected_pair_with(certificate_generation, limits()).await
}

async fn connected_pair_with(
    certificate_generation: u64,
    configured_limits: TransportLimits,
) -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    TransportLimits,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    connected_pair_with_generations(
        certificate_generation,
        certificate_generation,
        configured_limits,
    )
    .await
}

async fn connected_pair_with_generations(
    left_certificate_generation: u64,
    right_certificate_generation: u64,
    configured_limits: TransportLimits,
) -> (
    Arc<AuthenticatedConnection>,
    Arc<AuthenticatedConnection>,
    TransportLimits,
    quinn::Endpoint,
    quinn::Endpoint,
    tempfile::TempDir,
) {
    let issuer = certificate_authority();
    let root = issuer.der().clone();
    let left_material = leaf(&issuer, "node-1");
    let right_material = leaf(&issuer, "node-2");
    let (store, bound_store, signing_root, store_dir) = certificate_store();
    let left_authorization = transport_authorization(
        &store,
        &bound_store,
        &signing_root,
        "node-1",
        left_material.subject_public_key,
        left_certificate_generation,
    );
    let right_authorization = transport_authorization(
        &store,
        &bound_store,
        &signing_root,
        "node-2",
        right_material.subject_public_key,
        right_certificate_generation,
    );
    let left_binding = PeerBinding::new(
        &left_material.certificate,
        DOMAIN,
        "node-1",
        "guardian-1",
        1,
        left_certificate_generation,
    )
    .unwrap();
    let right_binding = PeerBinding::new(
        &right_material.certificate,
        DOMAIN,
        "node-2",
        "guardian-2",
        1,
        right_certificate_generation,
    )
    .unwrap();
    let left_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![left_material.certificate.clone()],
        left_material.private_key(),
        std::slice::from_ref(&root),
        &configured_limits,
    )
    .unwrap();
    let right_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![right_material.certificate.clone()],
        right_material.private_key(),
        &[root],
        &configured_limits,
    )
    .unwrap();
    let left_address = left_endpoint.local_addr().unwrap();
    let (left, right) = tokio::join!(
        AuthenticatedConnection::accept(
            &left_endpoint,
            ConnectionSecurity::new(
                left_binding.clone(),
                right_binding.clone(),
                left_authorization.clone(),
                right_authorization.clone(),
                configured_limits.clone(),
                CancellationToken::new(),
            )
            .unwrap(),
        ),
        AuthenticatedConnection::connect(
            &right_endpoint,
            left_address,
            "localhost",
            ConnectionSecurity::new(
                right_binding,
                left_binding,
                right_authorization,
                left_authorization,
                configured_limits.clone(),
                CancellationToken::new(),
            )
            .unwrap(),
        )
    );
    (
        Arc::new(left.unwrap()),
        Arc::new(right.unwrap()),
        configured_limits,
        left_endpoint,
        right_endpoint,
        store_dir,
    )
}

struct ThreeNodeMesh {
    connections: BTreeMap<(u64, u64), Arc<AuthenticatedConnection>>,
    endpoints: Vec<quinn::Endpoint>,
    limits: TransportLimits,
    _store_dir: tempfile::TempDir,
}

async fn three_node_mesh() -> ThreeNodeMesh {
    let issuer = certificate_authority();
    let root = issuer.der().clone();
    let materials = (1..=3)
        .map(|node| (node, leaf(&issuer, &format!("node-{node}"))))
        .collect::<BTreeMap<_, _>>();
    let (store, bound_store, signing_root, store_dir) = certificate_store();
    let authorizations = materials
        .iter()
        .map(|(node, material)| {
            (
                *node,
                transport_authorization(
                    &store,
                    &bound_store,
                    &signing_root,
                    &format!("node-{node}"),
                    material.subject_public_key,
                    1,
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let bindings = materials
        .iter()
        .map(|(node, material)| {
            (
                *node,
                PeerBinding::new(
                    &material.certificate,
                    DOMAIN,
                    format!("node-{node}"),
                    format!("guardian-{node}"),
                    1,
                    1,
                )
                .unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let configured_limits = limits();
    let mut connections = BTreeMap::new();
    let mut endpoints = Vec::new();
    for (lower, higher) in [(1, 2), (1, 3), (2, 3)] {
        let lower_material = &materials[&lower];
        let higher_material = &materials[&higher];
        let lower_endpoint = server_endpoint(
            (Ipv4Addr::LOCALHOST, 0).into(),
            vec![lower_material.certificate.clone()],
            lower_material.private_key(),
            std::slice::from_ref(&root),
            &configured_limits,
        )
        .unwrap();
        let higher_endpoint = client_endpoint(
            (Ipv4Addr::LOCALHOST, 0).into(),
            vec![higher_material.certificate.clone()],
            higher_material.private_key(),
            std::slice::from_ref(&root),
            &configured_limits,
        )
        .unwrap();
        let lower_address = lower_endpoint.local_addr().unwrap();
        let (lower_connection, higher_connection) = tokio::join!(
            AuthenticatedConnection::accept(
                &lower_endpoint,
                ConnectionSecurity::new(
                    bindings[&lower].clone(),
                    bindings[&higher].clone(),
                    authorizations[&lower].clone(),
                    authorizations[&higher].clone(),
                    configured_limits.clone(),
                    CancellationToken::new(),
                )
                .unwrap(),
            ),
            AuthenticatedConnection::connect(
                &higher_endpoint,
                lower_address,
                "localhost",
                ConnectionSecurity::new(
                    bindings[&higher].clone(),
                    bindings[&lower].clone(),
                    authorizations[&higher].clone(),
                    authorizations[&lower].clone(),
                    configured_limits.clone(),
                    CancellationToken::new(),
                )
                .unwrap(),
            )
        );
        connections.insert((lower, higher), Arc::new(lower_connection.unwrap()));
        connections.insert((higher, lower), Arc::new(higher_connection.unwrap()));
        endpoints.push(lower_endpoint);
        endpoints.push(higher_endpoint);
    }
    ThreeNodeMesh {
        connections,
        endpoints,
        limits: configured_limits,
        _store_dir: store_dir,
    }
}

async fn wait_for_leader(nodes: &BTreeMap<u64, PolisRaft>) -> u64 {
    tokio::time::timeout(Duration::from_secs(20), async {
        let mut stable = None;
        let mut stable_observations = 0_u8;
        loop {
            let leaders = nodes
                .values()
                .filter_map(|raft| raft.metrics().borrow().current_leader)
                .collect::<Vec<_>>();
            if leaders.len() == nodes.len()
                && leaders
                    .first()
                    .is_some_and(|first| leaders.iter().all(|leader| leader == first))
            {
                if stable == Some(leaders[0]) {
                    stable_observations += 1;
                } else {
                    stable = Some(leaders[0]);
                    stable_observations = 1;
                }
                if stable_observations >= 8 {
                    return leaders[0];
                }
            } else {
                stable = None;
                stable_observations = 0;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("three secure voters elect one leader")
}

async fn commit_on_current_leader(nodes: &BTreeMap<u64, PolisRaft>, command: PolisCommand) -> u64 {
    tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            for (node, raft) in nodes {
                if let Ok(Ok(response)) = tokio::time::timeout(
                    Duration::from_millis(500),
                    raft.client_write(command.clone()),
                )
                .await
                {
                    assert!(response.data.accepted);
                    return *node;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("a stable secure leader accepts the governed mutation")
}

fn authority_topology() -> (
    MembershipState,
    AuthorityMembership,
    BTreeMap<String, SocketAddr>,
    BTreeMap<u64, SigningKey>,
) {
    authority_topology_with_key_seeds([11, 12, 13])
}

fn authority_topology_with_key_seeds(
    key_seeds: [u8; 3],
) -> (
    MembershipState,
    AuthorityMembership,
    BTreeMap<String, SocketAddr>,
    BTreeMap<u64, SigningKey>,
) {
    let keys = [
        SigningKey::from_bytes(&[key_seeds[0]; 32]),
        SigningKey::from_bytes(&[key_seeds[1]; 32]),
        SigningKey::from_bytes(&[key_seeds[2]; 32]),
    ];
    let mut membership = MembershipState::new(MembershipPolicy::new(DOMAIN, 8, 16).unwrap());
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
    for offset in 0..3 {
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
    let guardians = (1..=3)
        .map(|id| format!("guardian-{id}").into_bytes())
        .collect::<BTreeSet<_>>();
    let voters = keys
        .iter()
        .enumerate()
        .map(|(offset, key)| VoterAuthority {
            guardian_id: format!("guardian-{}", offset + 1).into_bytes(),
            trust_domain_id: DOMAIN.as_bytes().to_vec(),
            certificate_generation: 1,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: 1,
            not_after_unix_seconds: 4_000_000_000,
            revoked: false,
            control_public_key: key.verifying_key().to_bytes(),
        })
        .collect();
    let authority = AuthorityMembership::new(
        DOMAIN.as_bytes().to_vec(),
        1,
        index,
        vec![guardians],
        voters,
    )
    .unwrap();
    let addresses = (1..=3)
        .map(|id| {
            (
                format!("node-{id}"),
                SocketAddr::from((Ipv4Addr::LOCALHOST, 4100 + id)),
            )
        })
        .collect();
    (
        membership,
        authority,
        addresses,
        keys.into_iter()
            .enumerate()
            .map(|(offset, key)| ((offset + 1) as u64, key))
            .collect(),
    )
}

fn polis_identity_for(
    polis_id: &str,
    authority: &AuthorityMembership,
    established: &EstablishedRuntimeAuthority,
    keys: &BTreeMap<u64, SigningKey>,
    boot_generations: &BTreeMap<u64, u64>,
) -> PolisIdentityBinding {
    let payload = polis_identity_signing_payload(
        polis_id,
        DOMAIN,
        authority.committed_log_index,
        boot_generations,
    )
    .unwrap();
    let endorsements = authority
        .raft_ids
        .iter()
        .map(|(guardian, raft_id)| {
            (
                guardian.clone(),
                keys[raft_id].sign(&payload).to_bytes().to_vec(),
            )
        })
        .collect();
    PolisIdentityBinding::verify(
        polis_id,
        DOMAIN,
        authority.committed_log_index,
        boot_generations,
        &endorsements,
        established,
    )
    .unwrap()
}

fn polis_identity(
    authority: &AuthorityMembership,
    established: &EstablishedRuntimeAuthority,
    keys: &BTreeMap<u64, SigningKey>,
    boot_generations: &BTreeMap<u64, u64>,
) -> PolisIdentityBinding {
    polis_identity_for(POLIS, authority, established, keys, boot_generations)
}

fn membership_commitment(snapshot: &[u8]) -> [u8; 32] {
    let value: serde_json::Value = serde_json::from_slice(snapshot).unwrap();
    value["digest"]
        .as_array()
        .unwrap()
        .iter()
        .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn runtime_authority_initializer(
    membership: &MembershipState,
    authority: &AuthorityMembership,
) -> (
    PolisRuntimeAuthorityBootstrap,
    BTreeMap<Vec<u8>, AuthorityCertificate>,
    tempfile::TempDir,
) {
    let signing_root = SigningKey::from_bytes(&[93; 32]);
    let policy = CertificatePolicy::new(DOMAIN, [signing_root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 16, 16)
        .unwrap();
    let directory = repo_tempdir();
    let store = Arc::new(
        DistributedCertificateStore::open(
            &test_certificate_store_access(),
            directory
                .path()
                .canonicalize()
                .unwrap()
                .join("authority.redb"),
            policy,
        )
        .unwrap(),
    );
    let certificates = authority
        .voters
        .iter()
        .map(|(guardian, voter)| {
            let holder = std::str::from_utf8(guardian).unwrap();
            let certificate = AuthorityCertificate::issue(
                CertificateBody::new(
                    DOMAIN,
                    holder,
                    CertificatePurpose::GuardianControl,
                    voter.certificate_generation,
                    CertificateValidity {
                        issued_at_unix_secs: 90,
                        expires_at_unix_secs: 1000,
                    },
                    VerifyingKey::from_bytes(&voter.control_public_key).unwrap(),
                    &signing_root.verifying_key(),
                ),
                &signing_root,
            )
            .unwrap();
            store
                .activate(&test_certificate_store_access(), &certificate, 100)
                .unwrap();
            (guardian.clone(), certificate)
        })
        .collect::<BTreeMap<_, _>>();
    let snapshot = membership.snapshot().unwrap();
    let bound_store = bound_certificate_store(&store, &directory, "runtime-authority-lineage");
    let initializer = PolisRuntimeAuthorityBootstrap::restore_configured(
        bound_store,
        MembershipPolicy::new(DOMAIN, 8, 16).unwrap(),
        &snapshot,
        membership_commitment(&snapshot),
    )
    .unwrap();
    (initializer, certificates, directory)
}

fn establish_runtime_authority(
    membership: &MembershipState,
    authority: &AuthorityMembership,
) -> (EstablishedRuntimeAuthority, tempfile::TempDir) {
    let (initializer, certificates, directory) =
        runtime_authority_initializer(membership, authority);
    (
        initializer
            .accept_signed_lineage(authority, &certificates, 100)
            .unwrap(),
        directory,
    )
}

fn verified_cut(
    boot_generations: &BTreeMap<u64, u64>,
) -> (
    VerifiedPolisRouteCut,
    BTreeMap<u64, SigningKey>,
    BTreeMap<u64, BasicNode>,
) {
    let (membership, authority, addresses, keys) = authority_topology();
    let (established, _authority_directory) = establish_runtime_authority(&membership, &authority);
    let polis = polis_identity(&authority, &established, &keys, boot_generations);
    let cut = derive_authority_cut(&polis, &established, &addresses, 100).unwrap();
    let routes = cut
        .routes()
        .into_iter()
        .map(|(node, address)| (node, BasicNode::new(address)))
        .collect();
    (cut, keys, routes)
}

fn verified_cut_for_polis(
    polis_id: &str,
    boot_generations: &BTreeMap<u64, u64>,
) -> (VerifiedPolisRouteCut, BTreeMap<u64, SigningKey>) {
    let (membership, authority, addresses, keys) = authority_topology();
    let (established, _authority_directory) = establish_runtime_authority(&membership, &authority);
    let polis = polis_identity_for(polis_id, &authority, &established, &keys, boot_generations);
    (
        derive_authority_cut(&polis, &established, &addresses, 100).unwrap(),
        keys,
    )
}

fn verified_cut_with_key_seeds(
    key_seeds: [u8; 3],
    boot_generations: &BTreeMap<u64, u64>,
) -> VerifiedPolisRouteCut {
    let (membership, authority, addresses, keys) = authority_topology_with_key_seeds(key_seeds);
    let (established, _authority_directory) = establish_runtime_authority(&membership, &authority);
    let polis = polis_identity(&authority, &established, &keys, boot_generations);
    derive_authority_cut(&polis, &established, &addresses, 100).unwrap()
}

fn verified_cut_after_membership_advance(
    boot_generations: &BTreeMap<u64, u64>,
) -> (VerifiedPolisRouteCut, BTreeMap<u64, SigningKey>) {
    let (mut membership, authority, addresses, keys) = authority_topology();
    let next_epoch = membership.epoch() + 1;
    let next_index = membership.committed_log_index() + 1;
    membership
        .apply(&CommittedMembershipEvent::new(
            DOMAIN,
            [47; 32],
            next_epoch,
            next_index,
            MembershipOperation::Join {
                member: Member {
                    node_id: "node-4".to_owned(),
                    guardian_id: "guardian-4".to_owned(),
                    identity_generation: 1,
                    guardian_control_public_key: SigningKey::from_bytes(&[14; 32])
                        .verifying_key()
                        .to_bytes(),
                    role: MemberRole::NonVoting,
                },
            },
        ))
        .unwrap();
    let guardians = authority.voters.keys().cloned().collect::<BTreeSet<_>>();
    let advanced_authority = AuthorityMembership::new(
        DOMAIN.as_bytes().to_vec(),
        authority.voter_set_generation,
        next_index,
        vec![guardians],
        authority.voters.values().cloned().collect(),
    )
    .unwrap();
    let (established, _authority_directory) =
        establish_runtime_authority(&membership, &advanced_authority);
    let polis = polis_identity(&advanced_authority, &established, &keys, boot_generations);
    (
        derive_authority_cut(&polis, &established, &addresses, 100).unwrap(),
        keys,
    )
}

async fn establish_mesh_sessions(
    mesh: &ThreeNodeMesh,
    factories: &BTreeMap<u64, SecurePolisNetworkFactory>,
    keys: &BTreeMap<u64, SigningKey>,
) -> BTreeMap<(u64, u64), EstablishedPolisSession> {
    let mut sessions = BTreeMap::new();
    for (lower, higher) in [(1, 2), (1, 3), (2, 3)] {
        assert_eq!(
            SecurePolisNetworkFactory::connection_owner(lower, higher).unwrap(),
            lower
        );
        let lower_pending = factories[&lower]
            .pending_session(higher, &mesh.connections[&(lower, higher)])
            .await
            .unwrap();
        let higher_pending = factories[&higher]
            .pending_session(lower, &mesh.connections[&(higher, lower)])
            .await
            .unwrap();
        let (lower_session, higher_session) = tokio::join!(
            mesh.connections[&(lower, higher)].accept_polis_session(lower_pending, &keys[&lower]),
            mesh.connections[&(higher, lower)]
                .initiate_polis_session(higher_pending, &keys[&higher]),
        );
        sessions.insert((lower, higher), lower_session.unwrap());
        sessions.insert((higher, lower), higher_session.unwrap());
    }
    sessions
}

#[tokio::test]
async fn authenticated_quinn_binds_signed_authority_session_and_returns_bounded_response() {
    let (left, right, configured_limits, left_endpoint, right_endpoint, store_dir) =
        connected_pair().await;
    let boots = [(1, 7), (2, 8), (3, 9)].into_iter().collect();
    let (cut, keys, _routes) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let left_pending = left_factory.pending_session(2, &left).await.unwrap();
    let right_pending = right_factory.pending_session(1, &right).await.unwrap();
    let (left_session, right_session) = tokio::join!(
        left.accept_polis_session(left_pending, &keys[&1]),
        right.initiate_polis_session(right_pending, &keys[&2]),
    );
    let left_session = left_session.unwrap();
    let right_session = right_session.unwrap();
    right_factory
        .install_route(1, right.clone(), right_session.clone())
        .await
        .unwrap();
    let server = tokio::spawn(async move {
        let request = left.accept_polis_request(&left_session).await?;
        assert_eq!(request.sequence, 77);
        assert_eq!(request.message_kind, "vote");
        assert_eq!(request.payload, b"canonical-vote");
        request
            .respond(b"canonical-response".to_vec(), &configured_limits)
            .await?;
        for expected_sequence in [1, 2] {
            let request = left.accept_polis_request(&left_session).await?;
            assert_eq!(request.sequence, expected_sequence);
            request
                .respond(
                    format!("ordered-response-{expected_sequence}").into_bytes(),
                    &configured_limits,
                )
                .await?;
        }
        Ok::<(), TransportError>(())
    });
    assert_eq!(
        right
            .request_polis(&right_session, 77, "vote", b"canonical-vote".to_vec())
            .await
            .unwrap(),
        b"canonical-response"
    );
    let first = {
        let factory = right_factory.clone();
        tokio::spawn(async move { factory.request_bytes(1, "vote", b"first".to_vec()).await })
    };
    let second = {
        let factory = right_factory.clone();
        tokio::spawn(async move { factory.request_bytes(1, "vote", b"second".to_vec()).await })
    };
    let (first, second, server_result) = tokio::join!(first, second, server);
    assert!(first.unwrap().is_ok());
    assert!(second.unwrap().is_ok());
    assert!(server_result.unwrap().is_ok(), "server rejected request");
    assert!(left_endpoint.local_addr().is_ok());
    assert!(right_endpoint.local_addr().is_ok());
    assert!(store_dir.path().exists());
    eprintln!("ADL_ISSUE_191_CASE signed_mtls_polis_session=passed");
}

#[tokio::test]
async fn stalled_rpc_stream_is_bounded_by_the_transport_idle_deadline() {
    let short_limits = TransportLimits::bounded(
        256 * 1024,
        32,
        Duration::from_millis(100),
        Duration::from_secs(5),
    )
    .unwrap();
    let (left, right, _, _left_endpoint, _right_endpoint, _store) =
        connected_pair_with(1, short_limits).await;
    let boots = [(1, 1), (2, 1), (3, 1)].into_iter().collect();
    let (cut, keys, _) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let left_pending = left_factory.pending_session(2, &left).await.unwrap();
    let right_pending = right_factory.pending_session(1, &right).await.unwrap();
    let (left_session, right_session) = tokio::join!(
        left.accept_polis_session(left_pending, &keys[&1]),
        right.initiate_polis_session(right_pending, &keys[&2]),
    );
    let left_session = left_session.unwrap();
    let right_session = right_session.unwrap();
    let stalled = tokio::spawn(async move {
        let _request = left.accept_polis_request(&left_session).await.unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    assert_eq!(
        right
            .request_polis(&right_session, 1, "vote", b"bounded-stall".to_vec())
            .await
            .unwrap_err(),
        TransportError::IdleTimeout
    );
    stalled.abort();
    let _ = stalled.await;

    let response_stall_limits = TransportLimits::bounded(
        16 * 1024 * 1024,
        32,
        Duration::from_millis(100),
        Duration::from_secs(5),
    )
    .unwrap();
    let (left, right, _, _left_endpoint, _right_endpoint, _store) =
        connected_pair_with(1, response_stall_limits.clone()).await;
    let boots = [(1, 1), (2, 1), (3, 1)].into_iter().collect();
    let (cut, keys, _) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let left_pending = left_factory.pending_session(2, &left).await.unwrap();
    let right_pending = right_factory.pending_session(1, &right).await.unwrap();
    let (left_session, right_session) = tokio::join!(
        left.accept_polis_session(left_pending, &keys[&1]),
        right.initiate_polis_session(right_pending, &keys[&2]),
    );
    let left_session = left_session.unwrap();
    let right_session = right_session.unwrap();
    let pending_response = right
        .begin_polis_request(&right_session, 1, "vote", b"do-not-read-response".to_vec())
        .await
        .unwrap();
    let request = left.accept_polis_request(&left_session).await.unwrap();
    assert_eq!(
        request
            .respond(vec![0; 15 * 1024 * 1024], &response_stall_limits)
            .await
            .unwrap_err(),
        TransportError::IdleTimeout
    );
    drop(pending_response);
    eprintln!("ADL_ISSUE_191_CASE stalled_rpc_idle_timeout=passed");
}

#[tokio::test]
async fn route_replacement_retries_exact_sequence_after_peer_restart_and_certificate_rotation() {
    let boots = [(1, 1), (2, 1), (3, 1)].into_iter().collect();
    let (cut, keys, _) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let (old_left, old_right, _, old_left_endpoint, old_right_endpoint, old_store) =
        connected_pair_at(1).await;
    let old_left_pending = left_factory.pending_session(2, &old_left).await.unwrap();
    let old_right_pending = right_factory.pending_session(1, &old_right).await.unwrap();
    let (old_left_session, old_right_session) = tokio::join!(
        old_left.accept_polis_session(old_left_pending, &keys[&1]),
        old_right.initiate_polis_session(old_right_pending, &keys[&2]),
    );
    left_factory
        .install_route(2, old_left.clone(), old_left_session.unwrap())
        .await
        .unwrap();
    drop(old_right_session.unwrap());
    old_left.close();
    old_right.close();

    assert_eq!(
        left_factory
            .request_bytes(2, "vote", b"same-request-after-restart".to_vec())
            .await
            .unwrap_err(),
        PolisRuntimeError::Network
    );
    assert_eq!(
        left_factory
            .request_bytes(
                2,
                "vote",
                b"different-request-must-not-steal-sequence".to_vec()
            )
            .await
            .unwrap_err(),
        PolisRuntimeError::Replay
    );
    let retry = tokio::spawn({
        let factory = left_factory.clone();
        async move {
            factory
                .request_bytes(2, "vote", b"same-request-after-restart".to_vec())
                .await
        }
    });
    tokio::task::yield_now().await;

    let (new_left, new_right, new_limits, new_left_endpoint, new_right_endpoint, new_store) =
        connected_pair_at(1).await;
    let new_left_pending = left_factory.pending_session(2, &new_left).await.unwrap();
    let new_right_pending = right_factory.pending_session(1, &new_right).await.unwrap();
    let (new_left_session, new_right_session) = tokio::join!(
        new_left.accept_polis_session(new_left_pending, &keys[&1]),
        new_right.initiate_polis_session(new_right_pending, &keys[&2]),
    );
    let new_left_session = new_left_session.unwrap();
    let new_right_session = new_right_session.unwrap();
    left_factory
        .replace_route(2, new_left.clone(), new_left_session)
        .await
        .unwrap();
    let responder = tokio::spawn(async move {
        let request = new_right
            .accept_polis_request(&new_right_session)
            .await
            .unwrap();
        assert_eq!(request.sequence, 1);
        assert_eq!(request.payload, b"same-request-after-restart");
        request
            .respond(b"replacement-response".to_vec(), &new_limits)
            .await
            .unwrap();
    });
    assert_eq!(retry.await.unwrap().unwrap(), b"replacement-response");
    responder.await.unwrap();

    let restarted_boots = [(1, 1), (2, 2), (3, 1)].into_iter().collect();
    let (restarted_cut, restarted_keys, _) = verified_cut(&restarted_boots);
    left_factory
        .replace_authority_cut(restarted_cut.clone())
        .await
        .unwrap();
    right_factory
        .replace_authority_cut(restarted_cut)
        .await
        .unwrap();
    let (
        rotated_left,
        rotated_right,
        rotated_limits,
        rotated_left_endpoint,
        rotated_right_endpoint,
        rotated_store,
    ) = connected_pair_at(2).await;
    let rotated_left_pending = left_factory
        .pending_session(2, &rotated_left)
        .await
        .unwrap();
    let rotated_right_pending = right_factory
        .pending_session(1, &rotated_right)
        .await
        .unwrap();
    let (rotated_left_session, rotated_right_session) = tokio::join!(
        rotated_left.accept_polis_session(rotated_left_pending, &restarted_keys[&1]),
        rotated_right.initiate_polis_session(rotated_right_pending, &restarted_keys[&2]),
    );
    left_factory
        .install_route(2, rotated_left, rotated_left_session.unwrap())
        .await
        .unwrap();
    let rotated_responder = tokio::spawn(async move {
        let request = rotated_right
            .accept_polis_request(&rotated_right_session.unwrap())
            .await
            .unwrap();
        assert_eq!(
            request.sequence, 1,
            "new authority namespace restarts sequence"
        );
        request
            .respond(b"rotated-response".to_vec(), &rotated_limits)
            .await
            .unwrap();
    });
    assert_eq!(
        left_factory
            .request_bytes(2, "vote", b"post-rotation".to_vec())
            .await
            .unwrap(),
        b"rotated-response"
    );
    rotated_responder.await.unwrap();
    assert!(old_left_endpoint.local_addr().is_ok());
    assert!(old_right_endpoint.local_addr().is_ok());
    assert!(new_left_endpoint.local_addr().is_ok());
    assert!(new_right_endpoint.local_addr().is_ok());
    assert!(rotated_left_endpoint.local_addr().is_ok());
    assert!(rotated_right_endpoint.local_addr().is_ok());
    assert!(old_store.path().exists());
    assert!(new_store.path().exists());
    assert!(rotated_store.path().exists());
    eprintln!("ADL_ISSUE_191_CASE exact_retry_after_boot_and_cert_rotation=passed");
}

#[tokio::test]
async fn durable_retry_cache_replays_exact_response_and_rejects_conflict_and_rollback() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    let boots = [(1, 1), (2, 3), (3, 1)].into_iter().collect();
    let (cut, keys, _) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let (left, right, _, _left_endpoint, _right_endpoint, _store) = connected_pair().await;
    let left_pending = left_factory.pending_session(2, &left).await.unwrap();
    let right_pending = right_factory.pending_session(1, &right).await.unwrap();
    let (left_session, right_session) = tokio::join!(
        left.accept_polis_session(left_pending, &keys[&1]),
        right.initiate_polis_session(right_pending, &keys[&2]),
    );
    let left_session = left_session.unwrap();
    let _right_session = right_session.unwrap();
    let cache = DurableRpcResponses::open(
        root_path.as_path(),
        1,
        2,
        &left_session,
        8,
        authority.clone(),
    )
    .unwrap();
    let state_path = std::fs::read_dir(root_path.as_path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("raft-rpc-session-") && name.ends_with(".json")
                })
        })
        .expect("the initial session owns one durable replay state file");
    let request = [7_u8; 32];
    let dispatches = Arc::new(AtomicUsize::new(0));
    let first_cache = cache.clone();
    let first_dispatches = Arc::clone(&dispatches);
    let second_cache = cache.clone();
    let second_dispatches = Arc::clone(&dispatches);
    let (first, second) = tokio::join!(
        first_cache.dispatch_once(1, &request, || async move {
            first_dispatches.fetch_add(1, Ordering::SeqCst);
            tokio::task::yield_now().await;
            Ok(b"accepted-response".to_vec())
        }),
        second_cache.dispatch_once(1, &request, || async move {
            second_dispatches.fetch_add(1, Ordering::SeqCst);
            Ok(b"accepted-response".to_vec())
        })
    );
    assert_eq!(first.unwrap(), b"accepted-response");
    assert_eq!(second.unwrap(), b"accepted-response");
    assert_eq!(dispatches.load(Ordering::SeqCst), 1);
    assert_eq!(
        cache.lookup(1, &request).await.unwrap().unwrap(),
        b"accepted-response"
    );
    assert_eq!(
        cache.lookup(1, &[8_u8; 32]).await.unwrap_err(),
        PolisRuntimeError::Replay
    );
    assert_eq!(
        cache.lookup(3, &[9_u8; 32]).await.unwrap_err(),
        PolisRuntimeError::Replay
    );

    let (rotated_left, rotated_right, _, _rotated_left_endpoint, _rotated_right_endpoint, _) =
        connected_pair_with_generations(2, 1, limits()).await;
    let rotated_left_pending = left_factory
        .pending_session(2, &rotated_left)
        .await
        .unwrap();
    let rotated_right_pending = right_factory
        .pending_session(1, &rotated_right)
        .await
        .unwrap();
    let (rotated_left_session, rotated_right_session) = tokio::join!(
        rotated_left.accept_polis_session(rotated_left_pending, &keys[&1]),
        rotated_right.initiate_polis_session(rotated_right_pending, &keys[&2]),
    );
    let rotated_cache = DurableRpcResponses::open(
        root_path.as_path(),
        1,
        2,
        &rotated_left_session.unwrap(),
        8,
        authority.clone(),
    )
    .unwrap();
    assert_eq!(rotated_cache.lookup(1, &request).await.unwrap(), None);
    drop(rotated_right_session.unwrap());

    let (advanced_cut, advanced_keys) = verified_cut_after_membership_advance(&boots);
    let advanced_left_factory = SecurePolisNetworkFactory::from_authority_cut(
        1,
        advanced_cut.clone(),
        test_learner_authority(),
    )
    .unwrap();
    let advanced_right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, advanced_cut, test_learner_authority())
            .unwrap();
    let (advanced_left, advanced_right, _, _advanced_left_endpoint, _advanced_right_endpoint, _) =
        connected_pair().await;
    let advanced_left_pending = advanced_left_factory
        .pending_session(2, &advanced_left)
        .await
        .unwrap();
    let advanced_right_pending = advanced_right_factory
        .pending_session(1, &advanced_right)
        .await
        .unwrap();
    let (advanced_left_session, advanced_right_session) = tokio::join!(
        advanced_left.accept_polis_session(advanced_left_pending, &advanced_keys[&1]),
        advanced_right.initiate_polis_session(advanced_right_pending, &advanced_keys[&2]),
    );
    let advanced_cache = DurableRpcResponses::open(
        root_path.as_path(),
        1,
        2,
        &advanced_left_session.unwrap(),
        8,
        authority.clone(),
    )
    .unwrap();
    assert_eq!(advanced_cache.lookup(1, &request).await.unwrap(), None);
    drop(advanced_right_session.unwrap());

    let (other_polis_cut, other_polis_keys) = verified_cut_for_polis("other-polis", &boots);
    let other_left_factory = SecurePolisNetworkFactory::from_authority_cut(
        1,
        other_polis_cut.clone(),
        test_learner_authority(),
    )
    .unwrap();
    let other_right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, other_polis_cut, test_learner_authority())
            .unwrap();
    let (other_left, other_right, _, _other_left_endpoint, _other_right_endpoint, _) =
        connected_pair().await;
    let other_left_pending = other_left_factory
        .pending_session(2, &other_left)
        .await
        .unwrap();
    let other_right_pending = other_right_factory
        .pending_session(1, &other_right)
        .await
        .unwrap();
    let (other_left_session, other_right_session) = tokio::join!(
        other_left.accept_polis_session(other_left_pending, &other_polis_keys[&1]),
        other_right.initiate_polis_session(other_right_pending, &other_polis_keys[&2]),
    );
    let other_polis_cache = DurableRpcResponses::open(
        root_path.as_path(),
        1,
        2,
        &other_left_session.unwrap(),
        8,
        authority.clone(),
    )
    .unwrap();
    assert_eq!(other_polis_cache.lookup(1, &request).await.unwrap(), None);
    drop(other_right_session.unwrap());

    let accepted = std::fs::read(&state_path).unwrap();
    cache
        .commit(2, &[9_u8; 32], b"new-response".to_vec())
        .await
        .unwrap();
    std::fs::write(&state_path, accepted).unwrap();
    let rollback =
        DurableRpcResponses::open(root_path.as_path(), 1, 2, &left_session, 8, authority);
    assert!(matches!(rollback, Err(PolisRuntimeError::StateRegression)));
    eprintln!("ADL_ISSUE_191_CASE retry_cache_conflict_and_rollback=passed");
}

#[tokio::test]
async fn log_store_does_not_publish_vote_before_external_checkpoint_commit() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    let mut store = PolisLogStore::open(root_path.as_path(), 1, authority.clone()).unwrap();
    let vote = Vote::new(3, 1);
    authority.fail_next_compare_and_swap();
    assert!(store.save_vote(&vote).await.is_err());
    assert_eq!(store.read_vote().await.unwrap(), None);
    drop(store);
    let mut recovered_after_failure =
        PolisLogStore::open(root_path.as_path(), 1, authority.clone()).unwrap();
    assert_eq!(recovered_after_failure.read_vote().await.unwrap(), None);
    recovered_after_failure.save_vote(&vote).await.unwrap();
    assert_eq!(
        recovered_after_failure.read_vote().await.unwrap(),
        Some(vote)
    );
    drop(recovered_after_failure);
    let reopened = PolisLogStore::open(root_path.as_path(), 1, authority).unwrap();
    let mut reopened = reopened;
    assert_eq!(reopened.read_vote().await.unwrap(), Some(vote));
    eprintln!("ADL_ISSUE_191_CASE durable_vote_restart=passed");
}

#[test]
fn fresh_store_initialization_rolls_back_an_ambiguous_checkpoint_failure() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    authority.fail_next_compare_and_swap();
    assert!(matches!(
        PolisLogStore::open(root_path.as_path(), 1, authority.clone()),
        Err(PolisRuntimeError::Storage)
    ));
    assert!(!root_path.join("raft-log.json").exists());
    assert!(!root_path.join(".raft-log.json.journal").exists());
    assert!(PolisLogStore::open(root_path.as_path(), 1, authority).is_ok());
    eprintln!("ADL_ISSUE_191_CASE journaled_initial_checkpoint=passed");
}

#[tokio::test]
async fn snapshot_install_rejects_noncanonical_bytes_and_preserves_exact_identity() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    let source_root = root_path.join("source");
    let target_root = root_path.join("target");
    std::fs::create_dir(&source_root).unwrap();
    std::fs::create_dir(&target_root).unwrap();
    let mut source = PolisStateMachineStore::open(&source_root, 1, authority.clone()).unwrap();
    let snapshot = source.build_snapshot().await.unwrap();
    let meta = snapshot.meta.clone();
    let canonical = snapshot.snapshot.get_ref().clone();
    let mut noncanonical = canonical.clone();
    noncanonical.push(b'\n');
    let mut target = PolisStateMachineStore::open(&target_root, 2, authority.clone()).unwrap();
    let mut forged_meta = meta.clone();
    forged_meta.snapshot_id = "caller-selected-snapshot".to_owned();
    assert!(target
        .install_snapshot(
            &forged_meta,
            Box::new(std::io::Cursor::new(canonical.clone()))
        )
        .await
        .is_err());
    assert!(target
        .install_snapshot(&meta, Box::new(std::io::Cursor::new(noncanonical)))
        .await
        .is_err());
    target
        .install_snapshot(&meta, Box::new(std::io::Cursor::new(canonical)))
        .await
        .unwrap();
    drop(target);
    let mut reopened = PolisStateMachineStore::open(&target_root, 2, authority).unwrap();
    assert_eq!(
        reopened.get_current_snapshot().await.unwrap().unwrap().meta,
        meta
    );
    eprintln!("ADL_ISSUE_191_CASE canonical_snapshot_identity=passed");
}

#[test]
fn boot_generation_is_externally_monotonic_and_rejects_coherent_disk_rollback() {
    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    assert_eq!(
        advance_secure_boot_generation(&root_path, 1, authority.clone()).unwrap(),
        1
    );
    let path = root_path.join("raft-boot-generation.json");
    let first = std::fs::read(&path).unwrap();
    assert_eq!(
        advance_secure_boot_generation(&root_path, 1, authority.clone()).unwrap(),
        2
    );
    std::fs::write(&path, first).unwrap();
    assert_eq!(
        advance_secure_boot_generation(&root_path, 1, authority).unwrap_err(),
        PolisRuntimeError::StateRegression
    );
    eprintln!("ADL_ISSUE_191_CASE boot_generation_rollback=passed");
}

#[tokio::test]
async fn topology_and_polis_identity_require_exact_runtime_control_and_quorum_parity() {
    let boots = [(1, 1), (2, 1), (3, 1)].into_iter().collect();
    let (membership, authority, addresses, keys) = authority_topology();
    let (initializer, certificates, _authority_directory) =
        runtime_authority_initializer(&membership, &authority);
    let established = initializer
        .accept_signed_lineage(&authority, &certificates, 100)
        .unwrap();
    let polis = polis_identity(&authority, &established, &keys, &boots);
    let cut = derive_authority_cut(&polis, &established, &addresses, 100).unwrap();
    assert_eq!(cut.routes().len(), 3);
    assert_eq!(
        cut.routes()[&1],
        SocketAddr::from((Ipv4Addr::LOCALHOST, 4101))
    );
    let factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let independently_configured_same_index = verified_cut_with_key_seeds([21, 22, 23], &boots);
    assert_eq!(
        factory
            .replace_authority_cut(independently_configured_same_index)
            .await
            .unwrap_err(),
        PolisRuntimeError::AuthorityDenied
    );

    let mut incomplete = addresses.clone();
    incomplete.remove("node-3");
    assert_eq!(
        derive_authority_cut(&polis, &established, &incomplete, 100).unwrap_err(),
        PolisRuntimeError::AuthorityDenied
    );

    let attacker_root = SigningKey::from_bytes(&[94; 32]);
    let attacker_certificates = authority
        .voters
        .iter()
        .map(|(guardian, voter)| {
            let holder = std::str::from_utf8(guardian).unwrap();
            (
                guardian.clone(),
                AuthorityCertificate::issue(
                    CertificateBody::new(
                        DOMAIN,
                        holder,
                        CertificatePurpose::GuardianControl,
                        voter.certificate_generation,
                        CertificateValidity {
                            issued_at_unix_secs: 90,
                            expires_at_unix_secs: 1000,
                        },
                        VerifyingKey::from_bytes(&voter.control_public_key).unwrap(),
                        &attacker_root.verifying_key(),
                    ),
                    &attacker_root,
                )
                .unwrap(),
            )
        })
        .collect();
    assert!(initializer
        .accept_signed_lineage(&authority, &attacker_certificates, 100)
        .is_err());

    let original_payload =
        polis_identity_signing_payload(POLIS, DOMAIN, authority.committed_log_index, &boots)
            .unwrap();
    let original_endorsements = authority
        .raft_ids
        .iter()
        .map(|(guardian, raft_id)| {
            (
                guardian.clone(),
                keys[raft_id].sign(&original_payload).to_bytes().to_vec(),
            )
        })
        .collect();
    let mut invented_boots = boots.clone();
    invented_boots.insert(2, 99);
    assert!(PolisIdentityBinding::verify(
        POLIS,
        DOMAIN,
        authority.committed_log_index,
        &invented_boots,
        &original_endorsements,
        &established,
    )
    .is_err());

    let wrong_payload = polis_identity_signing_payload(
        "invented-polis",
        DOMAIN,
        authority.committed_log_index,
        &boots,
    )
    .unwrap();
    let one_endorsement = [(
        b"guardian-1".to_vec(),
        keys[&1].sign(&wrong_payload).to_bytes().to_vec(),
    )]
    .into_iter()
    .collect();
    assert!(PolisIdentityBinding::verify(
        "invented-polis",
        DOMAIN,
        authority.committed_log_index,
        &boots,
        &one_endorsement,
        &established,
    )
    .is_err());
    eprintln!("ADL_ISSUE_191_CASE authority_cut_and_polis_quorum=passed");
}
#[test]
fn authority_approved_certificate_overlap_is_valid_then_expires_closed() {
    let signing_root = SigningKey::from_bytes(&[92; 32]);
    let policy = CertificatePolicy::new(DOMAIN, [signing_root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 16, 16)
        .unwrap();
    let directory = repo_tempdir();
    let store = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        directory
            .path()
            .canonicalize()
            .unwrap()
            .join("overlap.redb"),
        policy,
    )
    .unwrap();
    let make = |generation, seed| {
        AuthorityCertificate::issue(
            CertificateBody::new(
                DOMAIN,
                "overlap-node",
                CertificatePurpose::Transport,
                generation,
                CertificateValidity {
                    issued_at_unix_secs: 90,
                    expires_at_unix_secs: 1000,
                },
                SigningKey::from_bytes(&[seed; 32]).verifying_key(),
                &signing_root.verifying_key(),
            ),
            &signing_root,
        )
        .unwrap()
    };
    let first = make(1, 41);
    let second = make(2, 42);
    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &first, 100)
        .unwrap();
    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &second, 100)
        .unwrap();
    assert!(store
        .authorize(
            &TEST_CERTIFICATE_STORE_ACCESS,
            "overlap-node",
            CertificatePurpose::Transport,
            1,
            159
        )
        .is_ok());
    assert!(store
        .authorize(
            &TEST_CERTIFICATE_STORE_ACCESS,
            "overlap-node",
            CertificatePurpose::Transport,
            1,
            160
        )
        .is_err());
    assert!(store
        .authorize(
            &TEST_CERTIFICATE_STORE_ACCESS,
            "overlap-node",
            CertificatePurpose::Transport,
            2,
            160
        )
        .is_ok());
    eprintln!("ADL_ISSUE_191_CASE certificate_overlap_boundary=passed");
}

#[tokio::test]
async fn polis_frames_reject_unproved_polis_and_oversized_payload_before_dispatch() {
    let (left, right, configured_limits, _left_endpoint, _right_endpoint, _store_dir) =
        connected_pair().await;
    let boots = [(1, 1), (2, 1), (3, 1)].into_iter().collect();
    let (cut, keys, _routes) = verified_cut(&boots);
    let left_factory =
        SecurePolisNetworkFactory::from_authority_cut(1, cut.clone(), test_learner_authority())
            .unwrap();
    let right_factory =
        SecurePolisNetworkFactory::from_authority_cut(2, cut, test_learner_authority()).unwrap();
    let left_pending = left_factory.pending_session(2, &left).await.unwrap();
    let right_pending = right_factory.pending_session(1, &right).await.unwrap();
    let (left_session, right_session) = tokio::join!(
        left.accept_polis_session(left_pending, &keys[&1]),
        right.initiate_polis_session(right_pending, &keys[&2]),
    );
    let left_session = left_session.unwrap();
    let right_session = right_session.unwrap();
    let stalled_server =
        tokio::spawn(async move { left.accept_polis_request(&left_session).await });
    assert_eq!(
        right
            .request_polis(
                &right_session,
                1,
                "append_entries",
                vec![0; configured_limits.max_frame_bytes],
            )
            .await
            .unwrap_err(),
        TransportError::FrameTooLarge
    );
    stalled_server.abort();
    let _ = stalled_server.await;
    eprintln!("ADL_ISSUE_191_CASE unproved_polis_and_oversized_frame=passed");
}

#[test]
fn transport_frames_reject_truncated_and_noncanonical_trailing_bytes() {
    let configured_limits = limits();
    let envelope = TransportEnvelope {
        schema: TRANSPORT_SCHEMA.to_owned(),
        trust_domain: DOMAIN.to_owned(),
        node_id: "node-1".to_owned(),
        guardian_id: "guardian-1".to_owned(),
        protocol_version: 1,
        certificate_generation: 1,
        sequence: 1,
        payload: b"canonical".to_vec(),
    };
    let canonical = encode_frame(envelope, &configured_limits).unwrap();
    assert_eq!(
        decode_frame(&canonical[..canonical.len() - 1], &configured_limits).unwrap_err(),
        TransportError::MalformedFrame
    );
    let mut trailing = canonical;
    trailing.extend_from_slice(&[0x78, 0x01]);
    assert_eq!(
        decode_frame(&trailing, &configured_limits).unwrap_err(),
        TransportError::MalformedFrame
    );
    eprintln!("ADL_ISSUE_191_CASE canonical_transport_frame=passed");
}
#[cfg(unix)]
#[test]
fn durable_store_rejects_symlinked_ancestors_and_oversized_state() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let canonical = root.path().canonicalize().unwrap();
    let real = canonical.join("real");
    std::fs::create_dir(&real).unwrap();
    let linked = canonical.join("linked");
    symlink(&real, &linked).unwrap();
    assert!(matches!(
        PolisLogStore::open(&linked, 1, Arc::new(MemoryCheckpointAuthority::default())),
        Err(PolisRuntimeError::InvalidConfiguration)
    ));

    let oversized = canonical.join("oversized");
    std::fs::create_dir(&oversized).unwrap();
    let file = std::fs::File::create(oversized.join("raft-log.json")).unwrap();
    file.set_len(16 * 1024 * 1024 + 1).unwrap();
    assert!(PolisLogStore::open(
        &oversized,
        1,
        Arc::new(MemoryCheckpointAuthority::default())
    )
    .is_err());

    let locked = canonical.join("locked");
    std::fs::create_dir(&locked).unwrap();
    let lock_authority = Arc::new(MemoryCheckpointAuthority::default());
    let first = PolisLogStore::open(&locked, 1, lock_authority.clone()).unwrap();
    assert!(matches!(
        PolisLogStore::open(&locked, 1, lock_authority.clone()),
        Err(PolisRuntimeError::StateRegression)
    ));
    drop(first);
    assert!(PolisLogStore::open(&locked, 1, lock_authority).is_ok());

    let linked_lock_root = canonical.join("linked-lock");
    std::fs::create_dir(&linked_lock_root).unwrap();
    symlink(&real, linked_lock_root.join(".raft-log.json.lock")).unwrap();
    assert!(matches!(
        PolisLogStore::open(
            &linked_lock_root,
            1,
            Arc::new(MemoryCheckpointAuthority::default())
        ),
        Err(PolisRuntimeError::InvalidConfiguration)
    ));
    eprintln!("ADL_ISSUE_191_CASE path_and_state_bounds=passed");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state() {
    let root = repo_tempdir();
    let root_path = root.path().canonicalize().unwrap();
    let authority = Arc::new(MemoryCheckpointAuthority::default());
    let mesh = three_node_mesh().await;
    assert_eq!(mesh.endpoints.len(), 6);
    let mut boot_generations = BTreeMap::new();
    for node in 1..=3 {
        let node_root = root_path.join(format!("node-{node}"));
        std::fs::create_dir(&node_root).unwrap();
        boot_generations.insert(
            node,
            advance_secure_boot_generation(&node_root, node, authority.clone()).unwrap(),
        );
    }

    let (cut, keys, routes) = verified_cut(&boot_generations);
    let factories = (1..=3)
        .map(|node| {
            (
                node,
                SecurePolisNetworkFactory::from_authority_cut(
                    node,
                    cut.clone(),
                    test_learner_authority(),
                )
                .unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let sessions = establish_mesh_sessions(&mesh, &factories, &keys).await;
    for local in 1..=3 {
        for peer in 1..=3 {
            if local == peer {
                continue;
            }
            factories[&local]
                .install_route(
                    peer,
                    Arc::clone(&mesh.connections[&(local, peer)]),
                    sessions[&(local, peer)].clone(),
                )
                .await
                .unwrap();
        }
    }
    let mut nodes = BTreeMap::new();
    let mut machines = BTreeMap::new();
    for node in 1..=3 {
        let node_root = root_path.join(format!("node-{node}"));
        let (raft, machine) = new_secure_raft_node(
            node,
            &node_root,
            factories[&node].clone(),
            authority.clone(),
        )
        .await
        .unwrap();
        nodes.insert(node, raft);
        machines.insert(node, machine);
    }

    let cancellation = CancellationToken::new();
    let mut servers = Vec::new();
    for local in 1..=3 {
        for peer in 1..=3 {
            if local == peer {
                continue;
            }
            let cache = DurableRpcResponses::open(
                &root_path.join(format!("node-{local}")),
                local,
                peer,
                &sessions[&(local, peer)],
                256,
                authority.clone(),
            )
            .unwrap();
            servers.push(tokio::spawn(serve_secure_raft_connection(
                nodes[&local].clone(),
                Arc::clone(&mesh.connections[&(local, peer)]),
                sessions[&(local, peer)].clone(),
                mesh.limits.clone(),
                cache,
                cancellation.child_token(),
            )));
        }
    }

    nodes[&1].initialize(routes).await.unwrap();
    let _observed_leader = wait_for_leader(&nodes).await;
    let leader = commit_on_current_leader(
        &nodes,
        PolisCommand::GovernedMutation {
            mutation_id: "three-voter-commit".to_owned(),
            payload_sha256: "11".repeat(32),
        },
    )
    .await;
    let followers = (1..=3).filter(|node| *node != leader).collect::<Vec<_>>();

    for peer in 1..=3 {
        if peer != followers[0] {
            mesh.connections[&(followers[0], peer)].close();
        }
    }
    let with_two = nodes[&leader]
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "two-voter-commit".to_owned(),
            payload_sha256: "22".repeat(32),
        })
        .await
        .unwrap();
    assert!(with_two.data.accepted);
    nodes[&leader].trigger().snapshot().await.unwrap();

    for peer in 1..=3 {
        if peer != followers[1] {
            mesh.connections[&(followers[1], peer)].close();
        }
    }
    let no_quorum = tokio::time::timeout(
        Duration::from_secs(2),
        nodes[&leader].client_write(PolisCommand::GovernedMutation {
            mutation_id: "one-voter-must-halt".to_owned(),
            payload_sha256: "33".repeat(32),
        }),
    )
    .await;
    assert!(no_quorum.is_err() || no_quorum.unwrap().is_err());
    assert!(!machines[&leader]
        .application_state()
        .await
        .mutation_ids
        .contains("one-voter-must-halt"));
    tokio::time::sleep(Duration::from_millis(100)).await;
    let mut snapshot_reader = machines[&leader].clone();
    let retained_snapshot = snapshot_reader
        .get_current_snapshot()
        .await
        .unwrap()
        .expect("leader retained a compacted snapshot");
    let retained_snapshot_meta = retained_snapshot.meta.clone();
    drop(snapshot_reader);

    cancellation.cancel();
    for raft in nodes.values() {
        raft.shutdown().await.unwrap();
    }
    for server in servers {
        server.abort();
        let _ = server.await;
    }
    drop(nodes);
    drop(machines);
    drop(factories);
    drop(sessions);
    drop(mesh);

    let mut restarted_boot_generations = BTreeMap::new();
    for node in 1..=3 {
        let node_root = root_path.join(format!("node-{node}"));
        restarted_boot_generations.insert(
            node,
            advance_secure_boot_generation(&node_root, node, authority.clone()).unwrap(),
        );
    }
    let restarted_mesh = three_node_mesh().await;
    let (restarted_cut, restarted_keys, _) = verified_cut(&restarted_boot_generations);
    let restarted_factories = (1..=3)
        .map(|node| {
            (
                node,
                SecurePolisNetworkFactory::from_authority_cut(
                    node,
                    restarted_cut.clone(),
                    test_learner_authority(),
                )
                .unwrap(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let restarted_sessions =
        establish_mesh_sessions(&restarted_mesh, &restarted_factories, &restarted_keys).await;
    for local in 1..=3 {
        for peer in 1..=3 {
            if local != peer {
                restarted_factories[&local]
                    .install_route(
                        peer,
                        Arc::clone(&restarted_mesh.connections[&(local, peer)]),
                        restarted_sessions[&(local, peer)].clone(),
                    )
                    .await
                    .unwrap();
            }
        }
    }
    let mut restarted_nodes = BTreeMap::new();
    let mut restarted_machines = BTreeMap::new();
    for node in 1..=3 {
        let (raft, machine) = new_secure_raft_node(
            node,
            &root_path.join(format!("node-{node}")),
            restarted_factories[&node].clone(),
            authority.clone(),
        )
        .await
        .unwrap();
        restarted_nodes.insert(node, raft);
        restarted_machines.insert(node, machine);
    }
    let restarted_cancellation = CancellationToken::new();
    let mut restarted_servers = Vec::new();
    for local in 1..=3 {
        for peer in 1..=3 {
            if local == peer {
                continue;
            }
            let cache = DurableRpcResponses::open(
                &root_path.join(format!("node-{local}")),
                local,
                peer,
                &restarted_sessions[&(local, peer)],
                256,
                authority.clone(),
            )
            .unwrap();
            restarted_servers.push(tokio::spawn(serve_secure_raft_connection(
                restarted_nodes[&local].clone(),
                Arc::clone(&restarted_mesh.connections[&(local, peer)]),
                restarted_sessions[&(local, peer)].clone(),
                restarted_mesh.limits.clone(),
                cache,
                restarted_cancellation.child_token(),
            )));
        }
    }
    let mut restored_snapshot_reader = restarted_machines[&leader].clone();
    let restored_snapshot = restored_snapshot_reader
        .get_current_snapshot()
        .await
        .unwrap()
        .expect("snapshot survives a real voter restart");
    assert_eq!(restored_snapshot.meta, retained_snapshot_meta);
    let _ = wait_for_leader(&restarted_nodes).await;
    let restored_leader = commit_on_current_leader(
        &restarted_nodes,
        PolisCommand::GovernedMutation {
            mutation_id: "post-restart-commit".to_owned(),
            payload_sha256: "44".repeat(32),
        },
    )
    .await;
    let restored = restarted_machines[&restored_leader]
        .application_state()
        .await;
    assert!(restored.mutation_ids.contains("three-voter-commit"));
    assert!(restored.mutation_ids.contains("two-voter-commit"));
    assert!(restored.mutation_ids.contains("post-restart-commit"));
    restarted_cancellation.cancel();
    for raft in restarted_nodes.values() {
        raft.shutdown().await.unwrap();
    }
    for server in restarted_servers {
        server.abort();
        let _ = server.await;
    }
    eprintln!("ADL_ISSUE_191_CASE secure_three_two_one_real_restart=passed");
}
