// PVF: lane=exact-child-tests; proof=mutual-TLS transport positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[path = "../src/distributed/transport.rs"]
mod transport;

use std::{net::Ipv4Addr, path::PathBuf, time::Duration};

use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::sync::CancellationToken;
use transport::{
    client_endpoint, decode_frame, encode_frame, server_endpoint, AuthenticatedConnection,
    PeerBinding, TransportEnvelope, TransportError, TransportLimits, TRANSPORT_SCHEMA,
};

struct EndpointMaterial {
    certificate: CertificateDer<'static>,
    private_key: PrivateKeyDer<'static>,
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
    let key = KeyPair::generate().unwrap();
    let certificate = params.signed_by(&key, issuer).unwrap().der().clone();
    EndpointMaterial {
        certificate,
        private_key: PrivatePkcs8KeyDer::from(key.serialize_der()).into(),
    }
}

fn limits(authorization_lifetime: Duration) -> TransportLimits {
    TransportLimits::bounded(4096, 8, Duration::from_secs(5), authorization_lifetime).unwrap()
}

fn binding(certificate: &CertificateDer<'_>, node: &str, guardian: &str) -> PeerBinding {
    PeerBinding::new(certificate, "polis.example", node, guardian, 1, 1).unwrap()
}

async fn connected_pair(
    authorization_lifetime: Duration,
) -> (AuthenticatedConnection, AuthenticatedConnection) {
    connected_pair_with_expected_client_node(authorization_lifetime, "node-client").await
}

async fn connected_pair_with_expected_client_node(
    authorization_lifetime: Duration,
    expected_client_node: &str,
) -> (AuthenticatedConnection, AuthenticatedConnection) {
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
            server_binding.clone(),
            expected_client_binding,
            configured_limits.clone(),
            server_cancel,
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            server_address,
            "localhost",
            client_binding,
            server_binding,
            configured_limits,
            client_cancel,
        )
    );
    (server.unwrap(), client.unwrap())
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
            server_binding.clone(),
            wrong_client_binding,
            configured_limits.clone(),
            CancellationToken::new(),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            address,
            "localhost",
            client_binding,
            server_binding,
            configured_limits,
            CancellationToken::new(),
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
    let (server, client) =
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
            server_binding.clone(),
            client_binding.clone(),
            configured_limits.clone(),
            CancellationToken::new(),
        ),
        AuthenticatedConnection::connect(
            &client_endpoint,
            address,
            "localhost",
            client_binding,
            server_binding,
            configured_limits,
            CancellationToken::new(),
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
            expected.clone(),
            expected,
            limits(Duration::from_secs(1)),
            cancellation,
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
        "rustls = { version = \"=0.23.42\"",
    ] {
        assert!(manifest.contains(declaration), "missing pin: {declaration}");
    }
    for package in [
        "name = \"openraft\"\nversion = \"0.9.21\"",
        "name = \"quinn\"\nversion = \"0.11.11\"",
        "name = \"prost\"\nversion = \"0.13.5\"",
        "name = \"rustls\"\nversion = \"0.23.42\"",
    ] {
        assert!(lockfile.contains(package), "missing lock entry: {package}");
    }
    assert!(!lockfile.contains("name = \"openraft\"\nversion = \"0.10.0-alpha"));
}
