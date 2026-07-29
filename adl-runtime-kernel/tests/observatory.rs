use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    encode_acip_envelope, serve_control_listener, ControlApiPolicy, ControlAuthority,
    ControlCapability, ControlService, KernelExit, LifecycleControl, RuntimeRecorder,
    TrustedControlKey, ACIP_WEBSOCKET_SCHEMA, OBSERVATORY_FEED_SCHEMA, OBSERVATORY_WS_AUTH_SCHEMA,
    OBSERVATORY_WS_PATH,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use futures::{SinkExt, StreamExt};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderValue, Request},
        Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

struct FakeLifecycle;

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}

fn service(token: &str) -> Arc<ControlService<FakeLifecycle>> {
    let key = SigningKey::from_bytes(&[42; 32]);
    let authority = ControlAuthority::new(BTreeMap::from([(
        "operator-key".to_owned(),
        TrustedControlKey {
            principal: "operator".to_owned(),
            verifying_key: key.verifying_key(),
            capabilities: BTreeSet::from([ControlCapability::Read]),
        },
    )]));
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-ws",
        RuntimeRecorder::new(8),
        FakeLifecycle,
        authority,
        8,
        ["https://observatory.example.test".to_owned()],
    ));
    service.set_observatory_bearer_token(token).unwrap();
    service
        .set_public_base_url("https://observatory.example.test:20997")
        .unwrap();
    service
}

async fn websocket_server(
    service: Arc<ControlService<FakeLifecycle>>,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
) {
    let CertifiedKey { cert, signing_key } =
        generate_simple_self_signed(["localhost".to_owned()]).unwrap();
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem(
        cert.pem().as_bytes().to_vec(),
        signing_key.serialize_pem().into_bytes(),
    )
    .await
    .unwrap();
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(cert.der().to_vec()))
        .unwrap();
    let connector = Connector::Rustls(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(serve_control_listener(
        service,
        listener,
        tls,
        ControlApiPolicy::new(
            Duration::from_secs(2),
            Duration::from_secs(5),
            Duration::from_secs(1),
            64 * 1024,
        )
        .unwrap(),
    ));
    (address, connector, server)
}

fn request(address: std::net::SocketAddr, origin: &str) -> Request<()> {
    let mut request = format!("wss://localhost:{}{}", address.port(), OBSERVATORY_WS_PATH)
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Origin", HeaderValue::from_str(origin).unwrap());
    request
}

type TestSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

async fn connect_authenticated(
    address: std::net::SocketAddr,
    connector: Connector,
    token: &str,
) -> TestSocket {
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let payload = tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap();
    assert!(payload.contains(OBSERVATORY_FEED_SCHEMA));
    socket
}

async fn next_acip_status(socket: &mut TestSocket) -> serde_json::Value {
    tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                    return value;
                }
            }
        }
        panic!("authenticated Observatory session ended before ACIP status");
    })
    .await
    .unwrap()
}

#[tokio::test]
async fn observatory_websocket_requires_allowed_origin_and_session_auth() {
    let token = "test-observatory-websocket-token-0001";
    let (address, connector, server) = websocket_server(service(token)).await;

    let denied = connect_async_tls_with_config(
        request(address, "https://denied.example.test"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("403"));

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let payload = tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap();
    assert!(payload.contains(OBSERVATORY_FEED_SCHEMA));
    assert!(payload.contains("instance-ws"));
    assert!(payload.contains("https://observatory.example.test:20997"));
    assert!(payload.contains(OBSERVATORY_WS_PATH));
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_acip_replay_after_reconnect() {
    let token = "test-observatory-websocket-token-0007";
    let (address, connector, server) = websocket_server(service(token)).await;
    let mut first = connect_authenticated(address, connector.clone(), token).await;
    first
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1",
                "agent-source",
                "runtime-target",
                "consult",
                &serde_json::json!({"message": "first"}),
                1,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let accepted = next_acip_status(&mut first).await;
    assert_eq!(accepted["status"], "accepted");
    assert_eq!(accepted["message_id"], "acip-reconnect-1");
    assert_eq!(accepted["sequence_reserved"], true);
    first.close(None).await.unwrap();

    let mut second = connect_authenticated(address, connector, token).await;
    second
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1-replay",
                "agent-source",
                "runtime-target",
                "consult",
                &serde_json::json!({"message": "replayed"}),
                1,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let replayed = next_acip_status(&mut second).await;
    assert_eq!(replayed["status"], "rejected");
    assert_eq!(replayed["message_id"], "acip-reconnect-1-replay");
    assert_eq!(replayed["reason"], "monotonic_sequence_must_advance");
    assert_eq!(replayed["sequence_reserved"], false);

    second
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-2",
                "agent-source",
                "runtime-target",
                "consult",
                &serde_json::json!({"message": "second"}),
                2,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let advanced = next_acip_status(&mut second).await;
    assert_eq!(advanced["status"], "accepted");
    assert_eq!(advanced["message_id"], "acip-reconnect-2");
    assert_eq!(advanced["sequence_reserved"], true);
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_bad_auth_and_client_data() {
    let token = "test-observatory-websocket-token-0002";
    let (address, connector, server) = websocket_server(service(token)).await;
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": "invalid-observatory-token-0000000",
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text("{not-json".into()))
        .await
        .unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let _ = socket.next().await;
    socket
        .send(Message::Binary(vec![1, 2, 3].into()))
        .await
        .unwrap();
    let rejected = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                    return value;
                }
            }
        }
        panic!("authenticated Observatory session ended before ACIP rejection");
    })
    .await
    .unwrap();
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["sequence_reserved"], false);
    socket.send(Message::Ping(Vec::new().into())).await.unwrap();
    let pong = tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if matches!(message, Ok(Message::Pong(_))) {
                return true;
            }
        }
        false
    })
    .await
    .unwrap();
    assert!(pong);
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_a_token_after_rotation() {
    let token = "test-observatory-websocket-token-0003";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    service
        .set_observatory_bearer_token("rotated-observatory-websocket-token-0004")
        .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_revokes_an_authenticated_session_after_rotation() {
    let token = "test-observatory-websocket-token-0005";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": token,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    assert!(matches!(socket.next().await, Some(Ok(Message::Text(_)))));
    service
        .set_observatory_bearer_token("rotated-observatory-websocket-token-0006")
        .unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(3), socket.next()).await,
        Ok(Some(Ok(Message::Close(_))))
    ));
    server.abort();
}
