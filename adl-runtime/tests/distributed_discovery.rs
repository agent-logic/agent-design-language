// PVF: lane=exact-child-tests; proof=bounded authenticated discovery over real Quinn/rustls;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
#[path = "../src/distributed/discovery.rs"]
mod discovery;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;
#[allow(dead_code)]
#[path = "../src/distributed/membership.rs"]
mod membership;
mod authority_store_adapters {
    use super::certificates::{CertificateError, CertificatePurpose, VerifiedCertificate};

    #[derive(Clone)]
    pub struct AuthorityBoundCertificateStore;

    #[allow(dead_code)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum AuthorityStoreAdapterError {
        Certificate(CertificateError),
        Reconciliation,
    }

    impl AuthorityBoundCertificateStore {
        pub fn authorize(
            &self,
            _holder_id: &str,
            _purpose: CertificatePurpose,
            _generation: u64,
            _now_unix_secs: u64,
        ) -> Result<VerifiedCertificate, AuthorityStoreAdapterError> {
            Err(AuthorityStoreAdapterError::Reconciliation)
        }
    }
}
#[allow(dead_code)]
#[path = "../src/distributed/transport.rs"]
mod transport;

use std::{
    collections::BTreeMap,
    net::Ipv4Addr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS,
};
use discovery::{
    accept_proposal, discover, encode_proposal, encode_request, propose_join,
    AuthenticatedEnvelope, DiscoveryContext, DiscoveryError, DiscoveryPolicy, EnrolledPeer,
    EnrollmentAuthority, JoinRequest, ProposalReplayGuard, ProposedRole, RequestValidity,
    SeedEndpoint,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ED25519,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::sync::CancellationToken;
use transport::{
    client_endpoint, server_endpoint, AuthenticatedConnection, ConnectionSecurity, PeerBinding,
    TransportAuthorization, TransportLimits,
};

const DOMAIN: &str = "polis.example";
const SERVER_NODE: &str = "node-server";
const SERVER_GUARDIAN: &str = "guardian-server";
const CLIENT_NODE: &str = "node-client";
const CLIENT_GUARDIAN: &str = "guardian-client";

#[derive(Default)]
struct MemoryAuthority(Mutex<BTreeMap<String, EnrolledPeer>>);

impl MemoryAuthority {
    fn with(peers: impl IntoIterator<Item = EnrolledPeer>) -> Self {
        Self(Mutex::new(
            peers
                .into_iter()
                .map(|peer| (peer.node_id.clone(), peer))
                .collect(),
        ))
    }

    fn remove(&self, node_id: &str) {
        self.0.lock().unwrap().remove(node_id);
    }

    fn insert_at(&self, lookup: impl Into<String>, peer: EnrolledPeer) {
        self.0.lock().unwrap().insert(lookup.into(), peer);
    }
}

impl EnrollmentAuthority for MemoryAuthority {
    fn enrollment(&self, node_id: &str) -> discovery::DiscoveryResult<Option<EnrolledPeer>> {
        Ok(self.0.lock().unwrap().get(node_id).cloned())
    }
}

fn peer(node_id: &str, guardian_id: &str) -> EnrolledPeer {
    EnrolledPeer {
        trust_domain: DOMAIN.to_owned(),
        node_id: node_id.to_owned(),
        guardian_id: guardian_id.to_owned(),
        identity_generation: 1,
        transport_certificate_generation: 1,
    }
}

fn policy(timeout: Duration) -> DiscoveryPolicy {
    DiscoveryPolicy::new(DOMAIN, 1, timeout)
        .unwrap()
        .with_bounds(5, 60, 4096)
        .unwrap()
}

fn replay_store(capacity: usize) -> ProposalReplayGuard {
    ProposalReplayGuard::new_for_test(capacity).unwrap()
}

fn request(now: u64) -> JoinRequest {
    JoinRequest::new(
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
        1,
        [17; 32],
        RequestValidity {
            issued_at_unix_secs: now,
            expires_at_unix_secs: now + 30,
        },
    )
}

fn envelope(node: &str, guardian: &str, payload: Vec<u8>) -> AuthenticatedEnvelope {
    AuthenticatedEnvelope {
        trust_domain: DOMAIN.to_owned(),
        node_id: node.to_owned(),
        guardian_id: guardian.to_owned(),
        protocol_version: 1,
        certificate_generation: 1,
        payload,
    }
}

struct EndpointMaterial {
    certificate: CertificateDer<'static>,
    private_key: PrivateKeyDer<'static>,
    subject_public_key: VerifyingKey,
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

fn leaf(
    issuer: &CertifiedIssuer<'_, KeyPair>,
    name: &str,
    usage: ExtendedKeyUsagePurpose,
) -> EndpointMaterial {
    let mut params = CertificateParams::new(vec![name.to_owned()]).unwrap();
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    params.extended_key_usages = vec![usage];
    let key = KeyPair::generate_for(&PKCS_ED25519).unwrap();
    let subject_public_key =
        VerifyingKey::from_bytes(key.public_key_raw().try_into().unwrap()).unwrap();
    let certificate = params.signed_by(&key, issuer).unwrap().der().clone();
    EndpointMaterial {
        certificate,
        private_key: PrivatePkcs8KeyDer::from(key.serialize_der()).into(),
        subject_public_key,
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn certificate_store() -> (Arc<DistributedCertificateStore>, SigningKey) {
    let root = SigningKey::from_bytes(&[91; 32]);
    let certificate_policy = CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 64, 64)
        .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let database = directory
        .path()
        .canonicalize()
        .unwrap()
        .join("certificates.redb");
    let store = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        database,
        certificate_policy,
    )
    .unwrap();
    let _ = directory.keep();
    (Arc::new(store), root)
}

fn transport_authorization(
    store: &Arc<DistributedCertificateStore>,
    root: &SigningKey,
    holder: &str,
    subject_public_key: VerifyingKey,
) -> TransportAuthorization {
    let issued_at = now().saturating_sub(1);
    let body = CertificateBody::new(
        DOMAIN,
        holder,
        CertificatePurpose::Transport,
        1,
        CertificateValidity {
            issued_at_unix_secs: issued_at,
            expires_at_unix_secs: issued_at + 600,
        },
        subject_public_key,
        &root.verifying_key(),
    );
    let certificate = AuthorityCertificate::issue(body, root).unwrap();
    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, now())
        .unwrap();
    TransportAuthorization::new_for_test(store.clone(), &certificate).unwrap()
}

fn transport_limits() -> TransportLimits {
    TransportLimits::bounded(4096, 4, Duration::from_secs(3), Duration::from_secs(30)).unwrap()
}

struct ConnectedPair {
    server: Arc<AuthenticatedConnection>,
    client: Arc<AuthenticatedConnection>,
    server_endpoint: quinn::Endpoint,
    client_endpoint: quinn::Endpoint,
}

async fn connected_pair() -> ConnectedPair {
    let issuer = certificate_authority();
    let root_certificate = issuer.der().clone();
    let server_material = leaf(&issuer, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let client_material = leaf(&issuer, "client", ExtendedKeyUsagePurpose::ClientAuth);
    let server_binding = PeerBinding::new(
        &server_material.certificate,
        DOMAIN,
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let client_binding = PeerBinding::new(
        &client_material.certificate,
        DOMAIN,
        CLIENT_NODE,
        CLIENT_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let (store, root) = certificate_store();
    let server_authorization = transport_authorization(
        &store,
        &root,
        SERVER_NODE,
        server_material.subject_public_key,
    );
    let client_authorization = transport_authorization(
        &store,
        &root,
        CLIENT_NODE,
        client_material.subject_public_key,
    );
    let limits = transport_limits();
    let server_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![server_material.certificate],
        server_material.private_key,
        std::slice::from_ref(&root_certificate),
        &limits,
    )
    .unwrap();
    let client_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![client_material.certificate],
        client_material.private_key,
        &[root_certificate],
        &limits,
    )
    .unwrap();
    let server_address = server_endpoint.local_addr().unwrap();
    let (server, client) = tokio::join!(
        AuthenticatedConnection::accept(
            &server_endpoint,
            ConnectionSecurity::new(
                server_binding.clone(),
                client_binding.clone(),
                server_authorization.clone(),
                client_authorization.clone(),
                limits.clone(),
                CancellationToken::new(),
            )
            .unwrap(),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            server_address,
            "localhost",
            ConnectionSecurity::new(
                client_binding,
                server_binding,
                client_authorization,
                server_authorization,
                limits,
                CancellationToken::new(),
            )
            .unwrap(),
        ),
    );
    ConnectedPair {
        server: Arc::new(server.unwrap()),
        client: Arc::new(client.unwrap()),
        server_endpoint,
        client_endpoint,
    }
}

#[tokio::test]
async fn real_quinn_rustls_discovery_returns_deterministic_non_voting_proposal() {
    let pair = connected_pair().await;
    let authority = Arc::new(MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]));
    let configured_policy = policy(Duration::from_secs(2));
    let timestamp = now();
    let join_request = request(timestamp);
    let seed = SeedEndpoint::new(
        pair.server_endpoint.local_addr().unwrap(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let server = pair.server.clone();
    let server_authority = authority.clone();
    let server_policy = configured_policy.clone();
    let server_task = tokio::spawn(async move {
        let received = server.receive().await.unwrap();
        let authenticated = AuthenticatedEnvelope {
            trust_domain: received.trust_domain,
            node_id: received.node_id,
            guardian_id: received.guardian_id,
            protocol_version: received.protocol_version,
            certificate_generation: received.certificate_generation,
            payload: received.payload,
        };
        let proposal = propose_join(
            &authenticated,
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            server_authority.as_ref(),
            &server_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap();
        let payload = encode_proposal(&proposal, &server_policy).unwrap();
        server.send(1, payload).await.unwrap();
        proposal
    });
    let client = pair.client.clone();
    let accepted = discover(
        std::slice::from_ref(&seed),
        &join_request,
        authority.as_ref(),
        &configured_policy,
        DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new()),
        || Ok(timestamp),
        move |_seed, bytes| {
            let client = client.clone();
            async move {
                client
                    .send(1, bytes)
                    .await
                    .map_err(|_| DiscoveryError::Transport)?;
                let received = client
                    .receive()
                    .await
                    .map_err(|_| DiscoveryError::Transport)?;
                Ok(AuthenticatedEnvelope {
                    trust_domain: received.trust_domain,
                    node_id: received.node_id,
                    guardian_id: received.guardian_id,
                    protocol_version: received.protocol_version,
                    certificate_generation: received.certificate_generation,
                    payload: received.payload,
                })
            }
        },
    )
    .await
    .unwrap();
    let proposed_by_server = server_task.await.unwrap();
    assert_eq!(accepted, proposed_by_server);
    assert_eq!(accepted.proposed_role, ProposedRole::NonVoting);
    assert_eq!(accepted.candidate_node_id, CLIENT_NODE);
    drop(pair.client_endpoint);
}

#[test]
fn configured_seed_identity_is_not_enrollment_authority() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let full_authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &full_authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    let response = envelope(
        SERVER_NODE,
        SERVER_GUARDIAN,
        encode_proposal(&proposal, &configured_policy).unwrap(),
    );
    let requester_only = MemoryAuthority::with([peer(CLIENT_NODE, CLIENT_GUARDIAN)]);
    let error = accept_proposal(
        &seed,
        &join_request,
        &response,
        &requester_only,
        &configured_policy,
        &mut replay_store(1),
        timestamp,
    )
    .unwrap_err();
    assert_eq!(error, DiscoveryError::PeerNotEnrolled);
}

#[test]
fn stale_wrong_domain_and_replayed_proposals_fail_closed() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    let response = envelope(
        SERVER_NODE,
        SERVER_GUARDIAN,
        encode_proposal(&proposal, &configured_policy).unwrap(),
    );
    let mut replay = replay_store(1);
    accept_proposal(
        &seed,
        &join_request,
        &response,
        &authority,
        &configured_policy,
        &mut replay,
        timestamp,
    )
    .unwrap();
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &response,
            &authority,
            &configured_policy,
            &mut replay,
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::Replay
    );
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &response,
            &authority,
            &configured_policy,
            &mut replay_store(1),
            timestamp + 31,
        )
        .unwrap_err(),
        DiscoveryError::RequestExpired
    );
    let mut wrong_domain = response;
    wrong_domain.trust_domain = "other.example".to_owned();
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &wrong_domain,
            &authority,
            &configured_policy,
            &mut replay_store(1),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::WrongDomain
    );
}

#[test]
fn proposal_derivation_is_deterministic_and_future_requests_fail_closed() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let authenticated = envelope(
        CLIENT_NODE,
        CLIENT_GUARDIAN,
        encode_request(&join_request, &configured_policy).unwrap(),
    );
    let mut request_replay = replay_store(64);
    let first = propose_join(
        &authenticated,
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut request_replay,
        timestamp,
    )
    .unwrap();
    assert_eq!(
        propose_join(
            &authenticated,
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &authority,
            &configured_policy,
            &mut request_replay,
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::Replay
    );
    let second = propose_join(
        &authenticated,
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.proposed_role, ProposedRole::NonVoting);

    let future = request(timestamp + 6);
    let future_envelope = envelope(
        CLIENT_NODE,
        CLIENT_GUARDIAN,
        encode_request(&future, &configured_policy).unwrap(),
    );
    assert_eq!(
        propose_join(
            &future_envelope,
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &authority,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::RequestNotYetValid
    );
}

#[tokio::test]
async fn public_discovery_rejects_cross_call_replay_duplicate_and_stale_seeds() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    let response = envelope(
        SERVER_NODE,
        SERVER_GUARDIAN,
        encode_proposal(&proposal, &configured_policy).unwrap(),
    );
    let mut proposal_replay = replay_store(64);
    discover(
        std::slice::from_ref(&seed),
        &join_request,
        &authority,
        &configured_policy,
        DiscoveryContext::new(&mut proposal_replay, &CancellationToken::new()),
        || Ok(timestamp),
        {
            let response = response.clone();
            move |_seed, _bytes| {
                let response = response.clone();
                async move { Ok(response) }
            }
        },
    )
    .await
    .unwrap();

    let second_node = "node-seed-b";
    let second_guardian = "guardian-seed-b";
    let two_seed_authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(second_node, second_guardian),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let second_seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 5555).into(),
        second_node,
        second_guardian,
        1,
        1,
    )
    .unwrap();
    let second_proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(second_node, second_guardian),
        &two_seed_authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    assert_ne!(proposal.proposal_id, second_proposal.proposal_id);
    assert_eq!(
        accept_proposal(
            &second_seed,
            &join_request,
            &envelope(
                second_node,
                second_guardian,
                encode_proposal(&second_proposal, &configured_policy).unwrap(),
            ),
            &two_seed_authority,
            &configured_policy,
            &mut proposal_replay,
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::Replay
    );

    assert_eq!(
        discover(
            std::slice::from_ref(&seed),
            &join_request,
            &authority,
            &configured_policy,
            DiscoveryContext::new(&mut proposal_replay, &CancellationToken::new()),
            || Ok(timestamp),
            {
                let response = response.clone();
                move |_seed, _bytes| {
                    let response = response.clone();
                    async move { Ok(response) }
                }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::Replay
    );

    assert_eq!(
        discover(
            &[seed.clone(), seed.clone()],
            &join_request,
            &authority,
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new(),),
            || Ok(timestamp),
            |_seed, _bytes| async { Err(DiscoveryError::Transport) },
        )
        .await
        .unwrap_err(),
        DiscoveryError::DuplicateSeed
    );

    let mut rotated_seed = peer(SERVER_NODE, SERVER_GUARDIAN);
    rotated_seed.transport_certificate_generation = 2;
    let stale_authority = MemoryAuthority::with([rotated_seed, peer(CLIENT_NODE, CLIENT_GUARDIAN)]);
    assert_eq!(
        discover(
            &[seed],
            &join_request,
            &stale_authority,
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new(),),
            || Ok(timestamp),
            move |_seed, _bytes| {
                let response = response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );
}

#[test]
fn replay_window_denies_live_entries_and_recovers_capacity_after_expiry() {
    let timestamp = now();
    let expires = timestamp + 30;
    let mut replay = replay_store(64);

    for value in 1_u8..=64 {
        replay
            .observe_acceptance([value; 32], &format!("{value:064x}"), expires, timestamp)
            .unwrap();
    }
    assert_eq!(
        replay
            .observe_acceptance([65; 32], &format!("{:064x}", 65), expires, timestamp)
            .unwrap_err(),
        DiscoveryError::ResourceExhausted
    );
    assert_eq!(
        replay
            .observe_acceptance([1; 32], &format!("{:064x}", 1), expires, timestamp)
            .unwrap_err(),
        DiscoveryError::Replay
    );
    replay
        .observe_acceptance([65; 32], &format!("{:064x}", 65), expires + 30, expires + 1)
        .unwrap();
}

#[test]
fn replay_window_survives_restart_within_the_signed_validity_horizon() {
    let timestamp = now();
    let expires = timestamp + 30;
    let directory = tempfile::tempdir().unwrap();
    let database = directory
        .path()
        .canonicalize()
        .unwrap()
        .join("discovery-replay.redb");
    let request_id = [77; 32];
    let proposal_id = format!("{:064x}", 77);

    {
        let mut replay = ProposalReplayGuard::open(&database, 64).unwrap();
        replay
            .observe_acceptance(request_id, &proposal_id, expires, timestamp)
            .unwrap();
    }

    let mut restarted = ProposalReplayGuard::open(&database, 64).unwrap();
    assert_eq!(
        restarted
            .observe_acceptance(request_id, &proposal_id, expires, timestamp + 1)
            .unwrap_err(),
        DiscoveryError::Replay
    );
    restarted
        .observe_acceptance([78; 32], &format!("{:064x}", 78), expires + 30, expires + 1)
        .unwrap();
}

#[test]
fn malformed_payload_and_transport_generation_mismatch_fail_closed() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    assert_eq!(
        propose_join(
            &envelope(CLIENT_NODE, CLIENT_GUARDIAN, b"not-json".to_vec()),
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &authority,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::MalformedMessage
    );
    let mut wrong_generation = envelope(
        CLIENT_NODE,
        CLIENT_GUARDIAN,
        encode_request(&request(timestamp), &configured_policy).unwrap(),
    );
    wrong_generation.certificate_generation = 2;
    assert_eq!(
        propose_join(
            &wrong_generation,
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &authority,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let mut request_with_unknown_protobuf_field =
        encode_request(&request(timestamp), &configured_policy).unwrap();
    request_with_unknown_protobuf_field.extend_from_slice(&[0xf8, 0x01, 0x01]);
    assert_eq!(
        propose_join(
            &envelope(
                CLIENT_NODE,
                CLIENT_GUARDIAN,
                request_with_unknown_protobuf_field,
            ),
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &authority,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::MalformedMessage
    );

    let join_request = request(timestamp);
    let proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    let mut proposal_with_unknown_protobuf_field =
        encode_proposal(&proposal, &configured_policy).unwrap();
    proposal_with_unknown_protobuf_field.extend_from_slice(&[0xf8, 0x01, 0x01]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &envelope(
                SERVER_NODE,
                SERVER_GUARDIAN,
                proposal_with_unknown_protobuf_field,
            ),
            &authority,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::MalformedMessage
    );
}

#[test]
fn tampered_expiry_and_inconsistent_authority_identity_fail_closed() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let authenticated_request = envelope(
        CLIENT_NODE,
        CLIENT_GUARDIAN,
        encode_request(&join_request, &configured_policy).unwrap(),
    );
    let mut proposal = propose_join(
        &authenticated_request,
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    proposal.expires_at_unix_secs = u64::MAX;
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &envelope(
                SERVER_NODE,
                SERVER_GUARDIAN,
                encode_proposal(&proposal, &configured_policy).unwrap(),
            ),
            &authority,
            &configured_policy,
            &mut replay_store(1),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::UnexpectedPeer
    );

    let inconsistent_candidate = MemoryAuthority::default();
    inconsistent_candidate.insert_at(CLIENT_NODE, peer("node-other", CLIENT_GUARDIAN));
    inconsistent_candidate.insert_at(SERVER_NODE, peer(SERVER_NODE, SERVER_GUARDIAN));
    assert_eq!(
        propose_join(
            &authenticated_request,
            &peer(SERVER_NODE, SERVER_GUARDIAN),
            &inconsistent_candidate,
            &configured_policy,
            &mut replay_store(64),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let inconsistent_seed = MemoryAuthority::with([peer(CLIENT_NODE, CLIENT_GUARDIAN)]);
    inconsistent_seed.insert_at(SERVER_NODE, peer("node-other", SERVER_GUARDIAN));
    let valid_proposal = propose_join(
        &authenticated_request,
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        &authority,
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    assert_eq!(
        accept_proposal(
            &seed,
            &join_request,
            &envelope(
                SERVER_NODE,
                SERVER_GUARDIAN,
                encode_proposal(&valid_proposal, &configured_policy).unwrap(),
            ),
            &inconsistent_seed,
            &configured_policy,
            &mut replay_store(1),
            timestamp,
        )
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );
}

#[tokio::test]
async fn expiry_and_candidate_revocation_or_rotation_during_exchange_fail_closed() {
    let configured_policy = policy(Duration::from_millis(50));
    let timestamp = now();
    let join_request = request(timestamp);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let authority = Arc::new(MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]));
    let proposal = propose_join(
        &envelope(
            CLIENT_NODE,
            CLIENT_GUARDIAN,
            encode_request(&join_request, &configured_policy).unwrap(),
        ),
        &peer(SERVER_NODE, SERVER_GUARDIAN),
        authority.as_ref(),
        &configured_policy,
        &mut replay_store(64),
        timestamp,
    )
    .unwrap();
    let response = envelope(
        SERVER_NODE,
        SERVER_GUARDIAN,
        encode_proposal(&proposal, &configured_policy).unwrap(),
    );

    let revoked_authority = authority.clone();
    let revoked_response = response.clone();
    assert_eq!(
        discover(
            std::slice::from_ref(&seed),
            &join_request,
            authority.as_ref(),
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new(),),
            || Ok(timestamp),
            move |_seed, _bytes| {
                revoked_authority.remove(CLIENT_NODE);
                let response = revoked_response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let certificate_rotated_authority = Arc::new(MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]));
    let rotating_certificate = certificate_rotated_authority.clone();
    let certificate_rotated_response = response.clone();
    assert_eq!(
        discover(
            std::slice::from_ref(&seed),
            &join_request,
            certificate_rotated_authority.as_ref(),
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new()),
            || Ok(timestamp),
            move |_seed, _bytes| {
                let mut rotated = peer(CLIENT_NODE, CLIENT_GUARDIAN);
                rotated.transport_certificate_generation = 2;
                rotating_certificate.insert_at(CLIENT_NODE, rotated);
                let response = certificate_rotated_response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let seed_rotated_authority = Arc::new(MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]));
    let rotating_seed = seed_rotated_authority.clone();
    let seed_rotated_response = response.clone();
    assert_eq!(
        discover(
            std::slice::from_ref(&seed),
            &join_request,
            seed_rotated_authority.as_ref(),
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new()),
            || Ok(timestamp),
            move |_seed, _bytes| {
                let mut rotated = peer(SERVER_NODE, SERVER_GUARDIAN);
                rotated.identity_generation = 2;
                rotated.transport_certificate_generation = 2;
                rotating_seed.insert_at(SERVER_NODE, rotated);
                let response = seed_rotated_response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let rotated_authority = Arc::new(MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]));
    let rotating = rotated_authority.clone();
    let rotated_response = response.clone();
    assert_eq!(
        discover(
            std::slice::from_ref(&seed),
            &join_request,
            rotated_authority.as_ref(),
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new(),),
            || Ok(timestamp),
            move |_seed, _bytes| {
                let mut rotated = peer(CLIENT_NODE, CLIENT_GUARDIAN);
                rotated.identity_generation = 2;
                rotating.insert_at(CLIENT_NODE, rotated);
                let response = rotated_response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::PeerNotEnrolled
    );

    let live_authority = MemoryAuthority::with([
        peer(SERVER_NODE, SERVER_GUARDIAN),
        peer(CLIENT_NODE, CLIENT_GUARDIAN),
    ]);
    let observed_time = Arc::new(AtomicU64::new(timestamp));
    let exchange_time = observed_time.clone();
    let clock_time = observed_time.clone();
    assert_eq!(
        discover(
            &[seed],
            &join_request,
            &live_authority,
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new(),),
            move || Ok(clock_time.load(Ordering::SeqCst)),
            move |_seed, _bytes| {
                exchange_time.store(timestamp + 31, Ordering::SeqCst);
                let response = response.clone();
                async move { Ok(response) }
            },
        )
        .await
        .unwrap_err(),
        DiscoveryError::RequestExpired
    );
}

#[test]
fn unsafe_time_policy_bounds_are_rejected() {
    assert_eq!(
        DiscoveryPolicy::new(DOMAIN, 1, Duration::from_secs(31)).unwrap_err(),
        DiscoveryError::InvalidPolicy
    );
    assert_eq!(
        DiscoveryPolicy::new(DOMAIN, 1, Duration::from_secs(1))
            .unwrap()
            .with_bounds(301, 60, 4096)
            .unwrap_err(),
        DiscoveryError::InvalidPolicy
    );
    assert_eq!(
        DiscoveryPolicy::new(DOMAIN, 1, Duration::from_secs(1))
            .unwrap()
            .with_bounds(5, 601, 4096)
            .unwrap_err(),
        DiscoveryError::InvalidPolicy
    );
}

#[tokio::test]
async fn discovery_timeout_and_cancellation_are_finite() {
    let configured_policy = policy(Duration::from_millis(20));
    let timestamp = now();
    let join_request = request(timestamp);
    let authority = MemoryAuthority::with([peer(CLIENT_NODE, CLIENT_GUARDIAN)]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    let error = discover(
        std::slice::from_ref(&seed),
        &join_request,
        &authority,
        &configured_policy,
        DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new()),
        || Ok(timestamp),
        |_seed, _bytes| async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            Err(DiscoveryError::Transport)
        },
    )
    .await
    .unwrap_err();
    assert_eq!(error, DiscoveryError::Timeout);

    let cancellation = CancellationToken::new();
    cancellation.cancel();
    let error = discover(
        &[seed],
        &join_request,
        &authority,
        &configured_policy,
        DiscoveryContext::new(&mut replay_store(64), &cancellation),
        || Ok(timestamp),
        |_seed, _bytes| async { Err(DiscoveryError::Transport) },
    )
    .await
    .unwrap_err();
    assert_eq!(error, DiscoveryError::Cancelled);
}

#[test]
fn seed_and_message_resource_bounds_reject_before_work() {
    let configured_policy = policy(Duration::from_millis(20));
    let too_many = (0..65)
        .map(|offset| {
            SeedEndpoint::new(
                (Ipv4Addr::LOCALHOST, 10_000 + offset).into(),
                format!("node-{offset}"),
                format!("guardian-{offset}"),
                1,
                1,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let authority = MemoryAuthority::with([peer(CLIENT_NODE, CLIENT_GUARDIAN)]);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let timestamp = now();
    let bounded_request = request(timestamp);
    let error = runtime
        .block_on(discover(
            &too_many,
            &bounded_request,
            &authority,
            &configured_policy,
            DiscoveryContext::new(&mut replay_store(64), &CancellationToken::new()),
            || Ok(timestamp),
            |_seed, _bytes| async { Err(DiscoveryError::Transport) },
        ))
        .unwrap_err();
    assert_eq!(error, DiscoveryError::TooManySeeds);

    let oversized = envelope(SERVER_NODE, SERVER_GUARDIAN, vec![0; 4097]);
    let seed = SeedEndpoint::new(
        (Ipv4Addr::LOCALHOST, 4444).into(),
        SERVER_NODE,
        SERVER_GUARDIAN,
        1,
        1,
    )
    .unwrap();
    assert_eq!(
        accept_proposal(
            &seed,
            &request(now()),
            &oversized,
            &MemoryAuthority::with([
                peer(SERVER_NODE, SERVER_GUARDIAN),
                peer(CLIENT_NODE, CLIENT_GUARDIAN),
            ]),
            &configured_policy,
            &mut replay_store(1),
            now(),
        )
        .unwrap_err(),
        DiscoveryError::RequestTooLarge
    );
}
