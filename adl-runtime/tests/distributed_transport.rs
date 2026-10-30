// PVF: lane=exact-child-tests; proof=mutual-TLS transport positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
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
    net::Ipv4Addr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, RevocationReason,
    TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use prost::Message;
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ED25519,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::sync::CancellationToken;
use transport::{
    client_endpoint, decode_frame, encode_frame, server_endpoint, AuthenticatedConnection,
    ConnectionSecurity, PeerBinding, TransportAuthorization, TransportEnvelope, TransportError,
    TransportLimits, TRANSPORT_SCHEMA,
};

const DOMAIN: &str = "polis.example";

#[test]
fn transport_raw_store_constructor_remains_cfg_test_crate_private() {
    let source = include_str!("../src/distributed/transport/core.rs");
    assert!(
        source.contains(
            "pub fn new(\n        store: AuthorityBoundCertificateStore,\n        certificate: &AuthorityCertificate,"
        ),
        "production TransportAuthorization::new must require the #258 authority-bound adapter"
    );
    assert!(
        source.contains(
            "#[cfg(test)]\n    pub(crate) fn new_for_test(\n        store: Arc<DistributedCertificateStore>,"
        ),
        "raw-store test constructor must stay cfg(test) and crate-private"
    );
    assert!(
        !source.contains("#[cfg(feature = \"internal-test-fixtures\")]\n    pub fn new_for_test"),
        "raw-store transport constructor must not become a public feature-gated bypass"
    );
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

fn limits(authorization_lifetime: Duration) -> TransportLimits {
    TransportLimits::bounded(4096, 8, Duration::from_secs(5), authorization_lifetime).unwrap()
}

fn binding(certificate: &CertificateDer<'_>, node: &str, guardian: &str) -> PeerBinding {
    PeerBinding::new(certificate, DOMAIN, node, guardian, 1, 1).unwrap()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn authority_store() -> (Arc<DistributedCertificateStore>, SigningKey) {
    let root = SigningKey::from_bytes(&[91; 32]);
    let policy = CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3600, 60, 60, 64, 64)
        .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let canonical_directory = directory.path().canonicalize().unwrap();
    let store = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        canonical_directory.join("certificates.redb"),
        policy,
    )
    .unwrap();
    let _ = directory.keep();
    (Arc::new(store), root)
}

fn activate_authority_with_subject(
    store: &Arc<DistributedCertificateStore>,
    root: &SigningKey,
    holder: &str,
    subject_public_key: VerifyingKey,
    purpose: CertificatePurpose,
) -> AuthorityCertificate {
    let issued_at = now().saturating_sub(1);
    let body = CertificateBody::new(
        DOMAIN,
        holder,
        purpose,
        1,
        CertificateValidity {
            issued_at_unix_secs: issued_at,
            expires_at_unix_secs: issued_at + 300,
        },
        subject_public_key,
        &root.verifying_key(),
    );
    let certificate = AuthorityCertificate::issue(body, root).unwrap();
    store
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, now())
        .unwrap();
    certificate
}

fn transport_authority(
    store: &Arc<DistributedCertificateStore>,
    root: &SigningKey,
    holder: &str,
    subject_public_key: VerifyingKey,
) -> (TransportAuthorization, String) {
    let certificate = activate_authority_with_subject(
        store,
        root,
        holder,
        subject_public_key,
        CertificatePurpose::Transport,
    );
    let certificate_id = certificate.certificate_id().unwrap();
    (
        TransportAuthorization::new_for_test(store.clone(), &certificate).unwrap(),
        certificate_id,
    )
}

fn security(
    local: PeerBinding,
    expected_peer: PeerBinding,
    local_authorization: TransportAuthorization,
    peer_authorization: TransportAuthorization,
    limits: TransportLimits,
    cancellation: CancellationToken,
) -> ConnectionSecurity {
    ConnectionSecurity::new(
        local,
        expected_peer,
        local_authorization,
        peer_authorization,
        limits,
        cancellation,
    )
    .unwrap()
}

async fn connected_pair(
    authorization_lifetime: Duration,
) -> (AuthenticatedConnection, AuthenticatedConnection) {
    let (server, client, _, _) =
        connected_pair_with_expected_client_node(authorization_lifetime, "node-client").await;
    (server, client)
}

async fn connected_pair_with_expected_client_node(
    authorization_lifetime: Duration,
    expected_client_node: &str,
) -> (
    AuthenticatedConnection,
    AuthenticatedConnection,
    Arc<DistributedCertificateStore>,
    String,
) {
    let authority = certificate_authority();
    let root = authority.der().clone();
    let server_material = leaf(&authority, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let client_material = leaf(&authority, "client", ExtendedKeyUsagePurpose::ClientAuth);
    let server_binding = binding(
        &server_material.certificate,
        "node-server",
        "guardian-server",
    );
    let client_binding = binding(
        &client_material.certificate,
        "node-client",
        "guardian-client",
    );
    let expected_client_binding = binding(
        &client_material.certificate,
        expected_client_node,
        "guardian-client",
    );
    let (store, signing_root) = authority_store();
    let (server_authorization, _) = transport_authority(
        &store,
        &signing_root,
        "node-server",
        server_material.subject_public_key,
    );
    let (client_authorization, client_certificate_id) = transport_authority(
        &store,
        &signing_root,
        "node-client",
        client_material.subject_public_key,
    );
    let expected_client_authorization = if expected_client_node == "node-client" {
        client_authorization.clone()
    } else {
        transport_authority(
            &store,
            &signing_root,
            expected_client_node,
            client_material.subject_public_key,
        )
        .0
    };
    let configured_limits = limits(authorization_lifetime);

    let server_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![server_material.certificate],
        server_material.private_key,
        std::slice::from_ref(&root),
        &configured_limits,
    )
    .unwrap();
    let client_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![client_material.certificate],
        client_material.private_key,
        &[root],
        &configured_limits,
    )
    .unwrap();
    let server_address = server_endpoint.local_addr().unwrap();
    let server_cancel = CancellationToken::new();
    let client_cancel = CancellationToken::new();

    let (server, client) = tokio::join!(
        AuthenticatedConnection::accept(
            &server_endpoint,
            security(
                server_binding.clone(),
                expected_client_binding,
                server_authorization.clone(),
                expected_client_authorization,
                configured_limits.clone(),
                server_cancel,
            ),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            server_address,
            "localhost",
            security(
                client_binding,
                server_binding,
                client_authorization,
                server_authorization,
                configured_limits,
                client_cancel,
            ),
        )
    );
    (
        server.unwrap(),
        client.unwrap(),
        store,
        client_certificate_id,
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mutual_tls_loopback_carries_identity_bound_messages_both_ways() {
    let (server, client) = connected_pair(Duration::from_secs(30)).await;

    client.send(1, b"client-to-server".to_vec()).await.unwrap();
    let received = server.receive().await.unwrap();
    assert_eq!(received.node_id, "node-client");
    assert_eq!(received.guardian_id, "guardian-client");
    assert_eq!(received.payload, b"client-to-server");

    server.send(2, b"server-to-client".to_vec()).await.unwrap();
    let received = client.receive().await.unwrap();
    assert_eq!(received.node_id, "node-server");
    assert_eq!(received.guardian_id, "guardian-server");
    assert_eq!(received.payload, b"server-to-client");
    server.close();
    client.close();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn certificate_fingerprint_mismatch_fails_closed_after_tls_authentication() {
    let authority = certificate_authority();
    let root = authority.der().clone();
    let server_material = leaf(&authority, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let client_material = leaf(&authority, "client", ExtendedKeyUsagePurpose::ClientAuth);
    let other_client = leaf(
        &authority,
        "other-client",
        ExtendedKeyUsagePurpose::ClientAuth,
    );
    let server_binding = binding(&server_material.certificate, "server", "server-guardian");
    let wrong_client_binding = binding(&other_client.certificate, "client", "client-guardian");
    let client_binding = binding(&client_material.certificate, "client", "client-guardian");
    let (store, signing_root) = authority_store();
    let server_authorization = transport_authority(
        &store,
        &signing_root,
        "server",
        server_material.subject_public_key,
    )
    .0;
    let client_authorization = transport_authority(
        &store,
        &signing_root,
        "client",
        client_material.subject_public_key,
    )
    .0;
    let (wrong_store, wrong_root) = authority_store();
    let wrong_client_authorization = transport_authority(
        &wrong_store,
        &wrong_root,
        "client",
        other_client.subject_public_key,
    )
    .0;
    let configured_limits = limits(Duration::from_secs(30));
    let server_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![server_material.certificate],
        server_material.private_key,
        std::slice::from_ref(&root),
        &configured_limits,
    )
    .unwrap();
    let client_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![client_material.certificate],
        client_material.private_key,
        &[root],
        &configured_limits,
    )
    .unwrap();
    let address = server_endpoint.local_addr().unwrap();

    let (server, client) = tokio::join!(
        AuthenticatedConnection::accept(
            &server_endpoint,
            security(
                server_binding.clone(),
                wrong_client_binding,
                server_authorization.clone(),
                wrong_client_authorization,
                configured_limits.clone(),
                CancellationToken::new(),
            ),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            address,
            "localhost",
            security(
                client_binding,
                server_binding,
                client_authorization,
                server_authorization,
                configured_limits,
                CancellationToken::new(),
            ),
        )
    );
    assert!(matches!(
        server,
        Err(TransportError::PeerCertificateMismatch)
    ));
    assert!(client.is_ok());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authenticated_certificate_cannot_claim_a_different_node_identity() {
    let (server, client, _, _) =
        connected_pair_with_expected_client_node(Duration::from_secs(30), "different-node").await;
    client.send(1, b"claim".to_vec()).await.unwrap();
    assert_eq!(
        server.receive().await.unwrap_err(),
        TransportError::PeerIdentityMismatch
    );
    server.close();
    client.close();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn duplicate_sequence_is_rejected_with_a_bounded_replay_window() {
    let (server, client) = connected_pair(Duration::from_secs(30)).await;
    client.send(9, b"first".to_vec()).await.unwrap();
    assert_eq!(server.receive().await.unwrap().payload, b"first");
    client.send(9, b"replay".to_vec()).await.unwrap();
    assert_eq!(
        server.receive().await.unwrap_err(),
        TransportError::ReplayDetected
    );
    server.close();
    client.close();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn replay_window_accepts_unseen_in_window_and_rejects_stale_sequence() {
    let (server, client) = connected_pair(Duration::from_secs(30)).await;
    client.send(100, b"highest".to_vec()).await.unwrap();
    assert_eq!(server.receive().await.unwrap().sequence, 100);
    client.send(37, b"in-window".to_vec()).await.unwrap();
    assert_eq!(server.receive().await.unwrap().sequence, 37);
    client.send(36, b"stale".to_vec()).await.unwrap();
    assert_eq!(
        server.receive().await.unwrap_err(),
        TransportError::ReplayDetected
    );
    server.close();
    client.close();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revocation_closes_an_established_authorized_session() {
    let (server, client, store, client_certificate_id) =
        connected_pair_with_expected_client_node(Duration::from_secs(30), "node-client").await;
    store
        .revoke(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &client_certificate_id,
            now(),
            RevocationReason::OperatorRevoked,
        )
        .unwrap();
    assert_eq!(
        server.receive().await.unwrap_err(),
        TransportError::CertificateAuthorization
    );
    assert_eq!(
        client.send(1, b"revoked".to_vec()).await.unwrap_err(),
        TransportError::CertificateAuthorization
    );
    server.close();
    client.close();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn untrusted_client_certificate_is_rejected_by_mutual_tls() {
    let trusted_authority = certificate_authority();
    let untrusted_authority = certificate_authority();
    let trusted_root = trusted_authority.der().clone();
    let server_material = leaf(
        &trusted_authority,
        "localhost",
        ExtendedKeyUsagePurpose::ServerAuth,
    );
    let client_material = leaf(
        &untrusted_authority,
        "client",
        ExtendedKeyUsagePurpose::ClientAuth,
    );
    let server_binding = binding(&server_material.certificate, "server", "server-guardian");
    let client_binding = binding(&client_material.certificate, "client", "client-guardian");
    let (store, signing_root) = authority_store();
    let server_authorization = transport_authority(
        &store,
        &signing_root,
        "server",
        server_material.subject_public_key,
    )
    .0;
    let client_authorization = transport_authority(
        &store,
        &signing_root,
        "client",
        client_material.subject_public_key,
    )
    .0;
    let configured_limits = limits(Duration::from_secs(30));
    let server_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![server_material.certificate],
        server_material.private_key,
        std::slice::from_ref(&trusted_root),
        &configured_limits,
    )
    .unwrap();
    let client_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![client_material.certificate],
        client_material.private_key,
        &[trusted_root],
        &configured_limits,
    )
    .unwrap();
    let address = server_endpoint.local_addr().unwrap();
    let (server, client) = tokio::join!(
        AuthenticatedConnection::accept(
            &server_endpoint,
            security(
                server_binding.clone(),
                client_binding.clone(),
                server_authorization.clone(),
                client_authorization.clone(),
                configured_limits.clone(),
                CancellationToken::new(),
            ),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            address,
            "localhost",
            security(
                client_binding,
                server_binding,
                client_authorization,
                server_authorization,
                configured_limits,
                CancellationToken::new(),
            ),
        )
    );
    assert!(server.is_err() || client.is_err());
}

#[test]
fn framing_is_bounded_and_rejects_malformed_or_mismatched_identity() {
    let configured_limits =
        TransportLimits::bounded(128, 1, Duration::from_secs(1), Duration::from_secs(1)).unwrap();
    let envelope = TransportEnvelope {
        schema: TRANSPORT_SCHEMA.to_owned(),
        trust_domain: "polis.example".to_owned(),
        node_id: "node-a".to_owned(),
        guardian_id: "guardian-a".to_owned(),
        protocol_version: 1,
        certificate_generation: 1,
        sequence: 1,
        payload: vec![7; 256],
    };
    assert_eq!(
        encode_frame(envelope, &configured_limits).unwrap_err(),
        TransportError::FrameTooLarge
    );

    let valid = TransportEnvelope {
        schema: TRANSPORT_SCHEMA.to_owned(),
        trust_domain: "polis.example".to_owned(),
        node_id: "node-a".to_owned(),
        guardian_id: "guardian-a".to_owned(),
        protocol_version: 1,
        certificate_generation: 1,
        sequence: 1,
        payload: b"ok".to_vec(),
    };
    let mut encoded = encode_frame(valid, &configured_limits).unwrap();
    encoded.extend_from_slice(b"trailing");
    assert_eq!(
        decode_frame(&encoded, &configured_limits).unwrap_err(),
        TransportError::MalformedFrame
    );

    let body_over_limit = (1..=128)
        .map(|guardian_len| TransportEnvelope {
            schema: TRANSPORT_SCHEMA.to_owned(),
            trust_domain: DOMAIN.to_owned(),
            node_id: "node-a".to_owned(),
            guardian_id: "g".repeat(guardian_len),
            protocol_version: 1,
            certificate_generation: 1,
            sequence: 2,
            payload: Vec::new(),
        })
        .find(|candidate| {
            candidate.encoded_len() > configured_limits.max_frame_bytes
                && candidate.encoded_len() <= configured_limits.max_frame_bytes + 8
        })
        .unwrap();
    let mut encoded = Vec::new();
    body_over_limit
        .encode_length_delimited(&mut encoded)
        .unwrap();
    assert!(encoded.len() <= configured_limits.max_frame_bytes + 10);
    assert_eq!(
        decode_frame(&encoded, &configured_limits).unwrap_err(),
        TransportError::FrameTooLarge
    );
}

#[test]
fn non_transport_purpose_cannot_authorize_a_transport_session() {
    let (store, root) = authority_store();
    let certificate = activate_authority_with_subject(
        &store,
        &root,
        "node-wrong-purpose",
        SigningKey::from_bytes(&[71; 32]).verifying_key(),
        CertificatePurpose::GuardianControl,
    );
    assert!(matches!(
        TransportAuthorization::new_for_test(store, &certificate),
        Err(TransportError::CertificateAuthorization)
    ));
}

#[test]
fn transport_authority_rejects_wrong_domain_peer_binding() {
    let tls_authority = certificate_authority();
    let material = leaf(
        &tls_authority,
        "client",
        ExtendedKeyUsagePurpose::ClientAuth,
    );
    let (store, root) = authority_store();
    let authorization = transport_authority(&store, &root, "node", material.subject_public_key).0;
    let wrong_domain = PeerBinding::new(
        &material.certificate,
        "other.example",
        "node",
        "guardian",
        1,
        1,
    )
    .unwrap();

    assert!(matches!(
        ConnectionSecurity::new(
            wrong_domain.clone(),
            wrong_domain,
            authorization.clone(),
            authorization,
            limits(Duration::from_secs(30)),
            CancellationToken::new(),
        ),
        Err(TransportError::CertificateAuthorization)
    ));
}

#[test]
fn transport_authority_rejects_unrelated_tls_subject_key() {
    let tls_authority = certificate_authority();
    let material = leaf(
        &tls_authority,
        "client",
        ExtendedKeyUsagePurpose::ClientAuth,
    );
    let unrelated = SigningKey::from_bytes(&[72; 32]).verifying_key();
    let (store, root) = authority_store();
    let authorization = transport_authority(&store, &root, "node", unrelated).0;
    let binding = binding(&material.certificate, "node", "guardian");

    assert!(matches!(
        ConnectionSecurity::new(
            binding.clone(),
            binding,
            authorization.clone(),
            authorization,
            limits(Duration::from_secs(30)),
            CancellationToken::new(),
        ),
        Err(TransportError::CertificateAuthorization)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn configured_unidirectional_stream_ceiling_is_enforced() {
    let authority = certificate_authority();
    let root = authority.der().clone();
    let server_material = leaf(&authority, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let client_material = leaf(&authority, "client", ExtendedKeyUsagePurpose::ClientAuth);
    let configured_limits =
        TransportLimits::bounded(4096, 1, Duration::from_secs(5), Duration::from_secs(30)).unwrap();
    let server_endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![server_material.certificate],
        server_material.private_key,
        std::slice::from_ref(&root),
        &configured_limits,
    )
    .unwrap();
    let client_endpoint = client_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![client_material.certificate],
        client_material.private_key,
        &[root],
        &configured_limits,
    )
    .unwrap();
    let address = server_endpoint.local_addr().unwrap();
    let (server, client) = tokio::join!(
        async { server_endpoint.accept().await.unwrap().await.unwrap() },
        async {
            client_endpoint
                .connect(address, "localhost")
                .unwrap()
                .await
                .unwrap()
        }
    );

    let mut first = client.open_uni().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), client.open_uni())
            .await
            .is_err()
    );
    first.finish().unwrap();
    let mut received = server.accept_uni().await.unwrap();
    assert!(received.read_to_end(1).await.unwrap().is_empty());
    tokio::time::timeout(Duration::from_secs(1), client.open_uni())
        .await
        .unwrap()
        .unwrap();
    server.close(0_u32.into(), b"done");
    client.close(0_u32.into(), b"done");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_and_authorization_expiry_stop_transport_work() {
    let (server, client) = connected_pair(Duration::from_millis(20)).await;
    let (receive_result, send_result) = tokio::join!(server.receive(), async {
        tokio::time::sleep(Duration::from_millis(30)).await;
        client.send(1, b"late".to_vec()).await
    });
    assert_eq!(
        receive_result.unwrap_err(),
        TransportError::AuthorizationExpired
    );
    assert_eq!(
        send_result.unwrap_err(),
        TransportError::AuthorizationExpired
    );
    server.close();

    let cancellation = CancellationToken::new();
    cancellation.cancel();
    let authority = certificate_authority();
    let root = authority.der().clone();
    let material = leaf(&authority, "localhost", ExtendedKeyUsagePurpose::ServerAuth);
    let expected = binding(&material.certificate, "node", "guardian");
    let (store, signing_root) = authority_store();
    let authorization =
        transport_authority(&store, &signing_root, "node", material.subject_public_key).0;
    let endpoint = server_endpoint(
        (Ipv4Addr::LOCALHOST, 0).into(),
        vec![material.certificate],
        material.private_key,
        &[root],
        &limits(Duration::from_secs(1)),
    )
    .unwrap();
    assert!(matches!(
        AuthenticatedConnection::accept(
            &endpoint,
            security(
                expected.clone(),
                expected,
                authorization.clone(),
                authorization,
                limits(Duration::from_secs(1)),
                cancellation,
            ),
        )
        .await,
        Err(TransportError::Cancelled)
    ));
}

#[test]
fn dependency_contract_is_exactly_pinned_in_manifest_and_lockfile() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let lockfile = std::fs::read_to_string(root.join("Cargo.lock")).unwrap();
    for declaration in [
        "openraft = { version = \"=0.9.21\"",
        "quinn = { version = \"=0.11.11\"",
        "prost = \"=0.13.5\"",
        "rustls = { version = \"=0.23.43\"",
    ] {
        assert!(manifest.contains(declaration), "missing pin: {declaration}");
    }
    for package in [
        "name = \"openraft\"\nversion = \"0.9.21\"",
        "name = \"quinn\"\nversion = \"0.11.11\"",
        "name = \"prost\"\nversion = \"0.13.5\"",
        "name = \"rustls\"\nversion = \"0.23.43\"",
    ] {
        assert!(lockfile.contains(package), "missing lock entry: {package}");
    }
    assert!(!lockfile.contains("name = \"openraft\"\nversion = \"0.10.0-alpha"));
}
