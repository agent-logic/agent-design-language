#![cfg(unix)]

use std::{
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    encode_acip_envelope, verify_signed_identity_message, AcipEnvelope,
    CommunicationVerifyingIdentity, ControlAction, SignedControlCommand, SignedIdentityMessage,
    ACIP_IDENTITY_MESSAGE_SCHEMA, ACIP_PROTOBUF_SCHEMA, ACIP_PROTOCOL_FAMILY, ACIP_VERSION_MAJOR,
    ACIP_VERSION_MINOR, ACIP_WEBSOCKET_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};
use futures::{SinkExt, StreamExt};
use prost::Message as ProstMessage;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::rustls::{
    pki_types::{CertificateDer, ServerName},
    ClientConfig, RootCertStore,
};
use tokio_tungstenite::{
    tungstenite::{
        client::IntoClientRequest,
        http::{Request, StatusCode},
        protocol::frame::coding::CloseCode,
        Message,
    },
    Connector,
};

#[path = "support/runtime_init.rs"]
mod runtime_init;

const OBSERVATORY_TOKEN: &str = "guardian-observatory-token-00000001";
const ACIP_WRITE_TOKEN: &str = "guardian-acip-write-token-000000001";

struct GuardianLease {
    address: std::net::SocketAddr,
    token: String,
    release: Option<std::sync::mpsc::Sender<()>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl GuardianLease {
    fn start() -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let token = "production-acip-guardian-lease-000001".to_owned();
        let expected = token.clone();
        let (release_tx, release_rx) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            let (mut stream, peer) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if release_rx.try_recv().is_ok() {
                            return;
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("Guardian lease accept failed: {error}"),
                }
            };
            assert!(peer.ip().is_loopback());
            let mut supplied = vec![0_u8; expected.len()];
            stream.read_exact(&mut supplied).unwrap();
            assert_eq!(supplied, expected.as_bytes());
            stream.write_all(b"ok").unwrap();
            let _ = release_rx.recv();
        });
        Self {
            address,
            token,
            release: Some(release_tx),
            thread: Some(thread),
        }
    }

    fn apply(&self, command: &mut Command) {
        command
            .env(
                "ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS",
                self.address.to_string(),
            )
            .env("ADL_RUNTIME_GUARDIAN_LEASE_TOKEN", &self.token);
    }
}

impl Drop for GuardianLease {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            let _ = release.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

struct ChildGuard(Option<Child>);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(child) = self.0.as_mut() {
            if matches!(child.try_wait(), Ok(None)) {
                let _ = child.kill();
            }
            let _ = child.wait();
        }
    }
}

fn state_root(directory: &Path) -> std::path::PathBuf {
    let root = directory.join("production-acip-state");
    std::fs::create_dir_all(&root).unwrap();
    root.canonicalize().unwrap()
}

fn client_config(certificate_der: Vec<u8>) -> Arc<ClientConfig> {
    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(certificate_der)).unwrap();
    Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    )
}

fn acip_request(address: std::net::SocketAddr, token: &str) -> Request<()> {
    let mut request = format!("wss://localhost:{}/v1/acip/ws", address.port())
        .into_client_request()
        .unwrap();
    request
        .headers_mut()
        .insert("Authorization", format!("Bearer {token}").parse().unwrap());
    request
}

async fn next_acip_json<S>(socket: &mut tokio_tungstenite::WebSocketStream<S>) -> serde_json::Value
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    tokio::time::timeout(Duration::from_secs(3), async {
        while let Some(message) = socket.next().await {
            if let Message::Text(payload) = message.unwrap() {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == ACIP_WEBSOCKET_SCHEMA {
                    return value;
                }
            }
        }
        panic!("ACIP session ended before a protocol result arrived");
    })
    .await
    .expect("ACIP protocol result timed out")
}

fn signed_agent_message(sequence: u64) -> SignedIdentityMessage {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let mut message = SignedIdentityMessage {
        schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
        message_kind: "request".to_owned(),
        sender_id: "agent-0001".to_owned(),
        recipient_id: "agent-0002".to_owned(),
        correlation_id: format!("agent-correlation-{sequence:08}"),
        causation_id: format!("agent-causation-{sequence:08}"),
        monotonic_sequence: sequence,
        issued_at_unix_millis: now,
        expires_at_unix_millis: now + 60_000,
        nonce: format!("agent-message-{sequence:08}"),
        content: "Confirm governed agent-to-agent delivery.".to_owned(),
        signing_algorithm: "ed25519".to_owned(),
        signing_key_id: "agent-0001-communication".to_owned(),
        signature: String::new(),
    };
    message.signature = hex::encode(
        SigningKey::from_bytes(&[73; 32])
            .sign(&message.signing_bytes().unwrap())
            .to_bytes(),
    );
    message
}

fn resign_agent_message(message: &mut SignedIdentityMessage) {
    resign_message(message, [73; 32]);
}

fn resign_message(message: &mut SignedIdentityMessage, seed: [u8; 32]) {
    message.signature = hex::encode(
        SigningKey::from_bytes(&seed)
            .sign(&message.signing_bytes().unwrap())
            .to_bytes(),
    );
}

fn signed_agent_carrier(message: &SignedIdentityMessage, target: &str) -> Vec<u8> {
    signed_agent_carrier_for_runtime(message, target, "test-runtime-instance")
}

fn signed_agent_carrier_for_runtime(
    message: &SignedIdentityMessage,
    target: &str,
    runtime_id: &str,
) -> Vec<u8> {
    let route = if target == "shepherd" {
        "shepherd"
    } else {
        "agent"
    };
    AcipEnvelope {
        schema: ACIP_PROTOBUF_SCHEMA.to_owned(),
        message_id: message.nonce.clone(),
        source: message.sender_id.clone(),
        target: target.to_owned(),
        route: route.to_owned(),
        payload_json: serde_jcs::to_string(message).unwrap(),
        monotonic_sequence: message.monotonic_sequence,
        protocol_family: ACIP_PROTOCOL_FAMILY.to_owned(),
        version_major: ACIP_VERSION_MAJOR,
        version_minor: ACIP_VERSION_MINOR,
        runtime_id: runtime_id.to_owned(),
        correlation_id: message.correlation_id.clone(),
        causation_id: message.causation_id.clone(),
        trace_id: message.correlation_id.clone(),
        replay_id: format!("{}:{}", message.sender_id, message.monotonic_sequence),
        capability: route.to_owned(),
        authority: "signed-communication-identity".to_owned(),
        payload_type: "application/json".to_owned(),
        acknowledgement_requested: true,
        error_code: None,
        required_features: Vec::new(),
    }
    .encode_to_vec()
}

fn legacy_signed_agent_carrier(message: &SignedIdentityMessage, target: &str) -> Vec<u8> {
    let mut envelope = AcipEnvelope::decode(signed_agent_carrier(message, target).as_slice())
        .expect("current carrier");
    envelope.protocol_family.clear();
    envelope.version_major = 0;
    envelope.version_minor = 0;
    envelope.runtime_id.clear();
    envelope.correlation_id.clear();
    envelope.causation_id.clear();
    envelope.trace_id.clear();
    envelope.replay_id.clear();
    envelope.capability.clear();
    envelope.authority.clear();
    envelope.payload_type.clear();
    envelope.acknowledgement_requested = false;
    envelope.encode_to_vec()
}

async fn shutdown(
    config: Arc<ClientConfig>,
    address: std::net::SocketAddr,
    instance_id: &str,
) -> String {
    let command = SignedControlCommand::sign(
        "production-acip-shutdown",
        blake3::hash(b"production-acip-shutdown").to_hex()[..32].to_owned(),
        instance_id,
        "operator",
        ControlAction::Shutdown { grace_millis: 500 },
        "operator",
        &SigningKey::from_bytes(&[17_u8; 32]),
    )
    .unwrap();
    post_control(config, address, &command).await
}

async fn post_control(
    config: Arc<ClientConfig>,
    address: std::net::SocketAddr,
    command: &SignedControlCommand,
) -> String {
    let body = serde_json::to_vec(command).unwrap();
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = tokio_rustls::TlsConnector::from(config)
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    let headers = format!(
        "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes()).await.unwrap();
    stream.write_all(&body).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    String::from_utf8(response).unwrap()
}

async fn get_https(config: Arc<ClientConfig>, address: std::net::SocketAddr, path: &str) -> String {
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = tokio_rustls::TlsConnector::from(config)
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    let request = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    String::from_utf8(response).unwrap()
}

#[tokio::test]
async fn production_binary_acip_wss_produces_observed_receipt() {
    let directory = tempfile::tempdir().unwrap();
    let root = state_root(directory.path());
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        runtime_init::write_with_certificate_for_state(directory.path(), address, &root);
    let config = client_config(certificate_der);
    let lease = GuardianLease::start();
    let mut command = Command::new(env!("CARGO_BIN_EXE_adl-runtime-kernel"));
    command.arg("serve").arg("--init").arg(&init);
    lease.apply(&mut command);
    let mut child = ChildGuard(Some(
        command
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    ));
    let stderr = child.0.as_mut().unwrap().stderr.take().unwrap();
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let stderr_reader = std::thread::spawn(move || {
        let mut output = String::new();
        for line in BufReader::new(stderr).lines() {
            let line = line.unwrap();
            if line.contains("event=control_ready") {
                let instance_id = line
                    .split_whitespace()
                    .find_map(|field| field.strip_prefix("instance_id="))
                    .unwrap()
                    .to_owned();
                let _ = ready_tx.send(instance_id);
            }
            output.push_str(&line);
            output.push('\n');
        }
        output
    });
    let instance_id = ready_rx
        .recv_timeout(Duration::from_secs(10))
        .expect("production kernel did not report control readiness");
    let qualification_deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let response = get_https(config.clone(), address, "/v1/observatory").await;
        let captured = response
            .split_once("\r\n\r\n")
            .and_then(|(_, body)| serde_json::from_str::<serde_json::Value>(body).ok())
            .and_then(|feed| feed["captured_at_unix_millis"].as_u64());
        if captured.is_some() {
            break;
        }
        assert!(
            std::time::Instant::now() < qualification_deadline,
            "configured Runtime time authority did not qualify before secure messaging"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let unauthorized_layer8_submit = SignedControlCommand::sign(
        "layer8-must-not-control-runtime",
        blake3::hash(b"layer8-must-not-control-runtime").to_hex()[..32].to_owned(),
        &instance_id,
        "layer8-operator",
        ControlAction::Submit {
            work: adl_runtime_kernel::DomainWork {
                schema: adl_runtime_kernel::DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "layer8-general-control-submit".to_owned(),
                kind: "agent_runtime".to_owned(),
                payload: br#"{"schema":"adl.runtime.local_agent_work.v1","tasks":[{"op":"blake3","input":"must not dispatch"}]}"#.to_vec(),
            },
        },
        "layer8-communication",
        &SigningKey::from_bytes(&[72_u8; 32]),
    )
    .unwrap();
    let unauthorized = post_control(config.clone(), address, &unauthorized_layer8_submit).await;
    assert!(
        unauthorized.starts_with("HTTP/1.1 401 Unauthorized"),
        "Layer 8 communication key reached general control authority: {unauthorized}"
    );

    let mut assertions = Vec::new();
    assertions.push(serde_json::json!({
        "name": "layer8_communication_key_rejected_by_control_authority",
        "class": "negative_case",
        "result": "passed",
        "evidence": "the production Runtime rejected a general ControlAction::Submit signed by the configured Layer 8 communication key"
    }));
    assertions.push(serde_json::json!({
        "name": "exact_production_binary_tls_ready",
        "class": "lifecycle",
        "result": "passed",
        "evidence": "CARGO_BIN_EXE_adl-runtime-kernel reported control_ready on its configured TLS listener"
    }));

    let denied_stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let denied = tokio_tungstenite::client_async_tls_with_config(
        acip_request(address, OBSERVATORY_TOKEN),
        denied_stream,
        None,
        Some(Connector::Rustls(config.clone())),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("401"), "{denied}");
    assertions.push(serde_json::json!({
        "name": "observatory_read_token_rejected",
        "class": "negative_case",
        "result": "passed",
        "evidence": "the production ACIP upgrade returned HTTP 401 for the Observatory read credential"
    }));

    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let (mut socket, response) = tokio_tungstenite::client_async_tls_with_config(
        acip_request(address, ACIP_WRITE_TOKEN),
        stream,
        None,
        Some(Connector::Rustls(config.clone())),
    )
    .await
    .unwrap();
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
    let authenticated = next_acip_json(&mut socket).await;
    assert_eq!(authenticated["event"], "authenticated");
    assert_eq!(authenticated["path"], "/v1/acip/ws");
    assertions.push(serde_json::json!({
        "name": "acip_write_token_authenticated",
        "class": "successful_exchange",
        "result": "passed",
        "evidence": "the production WSS listener emitted its server-first authenticated frame"
    }));

    let frame = encode_acip_envelope(
        "production-acip-1",
        "proof-client",
        "runtime",
        "agent_runtime",
        &serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{"op": "blake3", "input": "production ACIP proof"}]
        }),
        1,
    )
    .unwrap();
    socket
        .send(Message::Binary(frame.clone().into()))
        .await
        .unwrap();
    let completed = next_acip_json(&mut socket).await;
    assert_eq!(completed["status"], "completed", "{completed}");
    assert_eq!(completed["message_id"], "production-acip-1");
    assert_eq!(completed["sequence_reserved"], true);
    assertions.push(serde_json::json!({
        "name": "binary_protobuf_dispatch_completed",
        "class": "successful_exchange",
        "result": "passed",
        "evidence": "a binary Protobuf ACIP envelope completed through canonical production ingress"
    }));

    socket.send(Message::Binary(frame.into())).await.unwrap();
    let replay = next_acip_json(&mut socket).await;
    assert_eq!(replay["status"], "rejected");
    assert_eq!(replay["reason"], "monotonic_sequence_must_advance");
    assert_eq!(replay["sequence_reserved"], false);
    assertions.push(serde_json::json!({
        "name": "replay_rejected",
        "class": "negative_case",
        "result": "passed",
        "evidence": "the production ingress rejected a repeated monotonic sequence"
    }));

    let direct = signed_agent_message(1);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&direct, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let delivered = next_acip_json(&mut socket).await;
    assert_eq!(delivered["status"], "completed", "{delivered}");
    let ack: SignedIdentityMessage =
        serde_json::from_value(delivered["signed_ack"].clone()).unwrap();
    verify_signed_identity_message(
        &ack,
        &std::collections::BTreeMap::from([(
            "agent-0002".to_owned(),
            CommunicationVerifyingIdentity {
                signing_key_id: "agent-0002-communication".to_owned(),
                verifying_key: SigningKey::from_bytes(&[74; 32]).verifying_key(),
            },
        )]),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    )
    .unwrap();
    assert_eq!(ack.sender_id, "agent-0002");
    assert_eq!(ack.recipient_id, "agent-0001");
    assert_eq!(ack.correlation_id, direct.correlation_id);
    assert_eq!(ack.causation_id, direct.nonce);
    assertions.push(serde_json::json!({
        "name": "signed_agent_to_agent_ack_verified",
        "class": "successful_exchange",
        "result": "passed",
        "evidence": "agent-0001 sent a signed ACIP identity message to agent-0002 and verified the returned agent-0002-signed acknowledgement"
    }));

    let advanced = signed_agent_message(2);
    let mut forged = advanced.clone();
    forged.content = "forged after signing".to_owned();
    socket
        .send(Message::Binary(
            signed_agent_carrier(&forged, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let forged_result = next_acip_json(&mut socket).await;
    assert_eq!(forged_result["status"], "rejected");
    assert_eq!(
        forged_result["reason"],
        "signed_identity_verification_failed"
    );
    assert_eq!(forged_result["sequence_reserved"], false);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&advanced, "wrong-recipient").into(),
        ))
        .await
        .unwrap();
    let tampered = next_acip_json(&mut socket).await;
    assert_eq!(tampered["status"], "rejected");
    assert_eq!(tampered["reason"], "signed_identity_carrier_mismatch");
    assert_eq!(tampered["sequence_reserved"], false);
    socket
        .send(Message::Binary(
            signed_agent_carrier_for_runtime(&advanced, "agent-0002", "other-runtime").into(),
        ))
        .await
        .unwrap();
    let wrong_runtime = next_acip_json(&mut socket).await;
    assert_eq!(wrong_runtime["status"], "rejected");
    assert_eq!(wrong_runtime["reason"], "signed_identity_carrier_mismatch");
    assert_eq!(wrong_runtime["sequence_reserved"], false);
    socket
        .send(Message::Binary(
            legacy_signed_agent_carrier(&advanced, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let legacy_secure = next_acip_json(&mut socket).await;
    assert_eq!(legacy_secure["status"], "rejected");
    assert_eq!(legacy_secure["reason"], "secure_carrier_v1_required");
    assert_eq!(legacy_secure["sequence_reserved"], false);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&advanced, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let corrected = next_acip_json(&mut socket).await;
    assert_eq!(corrected["status"], "completed", "{corrected}");
    socket
        .send(Message::Binary(
            signed_agent_carrier(&advanced, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let secure_replay = next_acip_json(&mut socket).await;
    assert_eq!(secure_replay["status"], "rejected");
    assert_eq!(secure_replay["reason"], "monotonic_sequence_must_advance");

    let mut wrong_recipient = signed_agent_message(3);
    wrong_recipient.recipient_id = "agent-missing".to_owned();
    resign_agent_message(&mut wrong_recipient);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&wrong_recipient, "agent-missing").into(),
        ))
        .await
        .unwrap();
    let refused_recipient = next_acip_json(&mut socket).await;
    assert_eq!(refused_recipient["status"], "rejected");
    assert_eq!(refused_recipient["reason"], "recipient_not_running");
    assert_eq!(refused_recipient["sequence_reserved"], false);
    let corrected_recipient = signed_agent_message(3);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&corrected_recipient, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let corrected_after_refusal = next_acip_json(&mut socket).await;
    assert_eq!(
        corrected_after_refusal["status"], "completed",
        "{corrected_after_refusal}"
    );
    let mut stopped_sender = signed_agent_message(1);
    stopped_sender.sender_id = "layer8-operator".to_owned();
    stopped_sender.signing_key_id = "layer8-communication".to_owned();
    stopped_sender.nonce = "layer8-direct-agent-message-00000001".to_owned();
    resign_message(&mut stopped_sender, [72; 32]);
    socket
        .send(Message::Binary(
            signed_agent_carrier(&stopped_sender, "agent-0002").into(),
        ))
        .await
        .unwrap();
    let refused_sender = next_acip_json(&mut socket).await;
    assert_eq!(refused_sender["status"], "rejected");
    assert_eq!(refused_sender["reason"], "sender_not_running");
    assert_eq!(refused_sender["sequence_reserved"], false);
    assertions.push(serde_json::json!({
        "name": "secure_carrier_tamper_and_replay_rejected",
        "class": "negative_case",
        "result": "passed",
        "evidence": "tampered routing did not reserve sequence state; corrected delivery succeeded once; replay was rejected; a correctly signed absent recipient was rejected before sequence consumption; and a configured signer absent from the running-agent roster could not originate direct agent traffic"
    }));

    socket
        .send(Message::Text("not-a-binary-acip-frame".into()))
        .await
        .unwrap();
    let close = tokio::time::timeout(Duration::from_secs(3), socket.next())
        .await
        .expect("binary-only policy close timed out")
        .expect("ACIP socket ended without a policy close")
        .expect("ACIP policy close failed");
    let Message::Close(Some(close)) = close else {
        panic!("expected a WebSocket close frame, got {close:?}");
    };
    assert_eq!(close.code, CloseCode::Policy);
    assert_eq!(close.reason, "binary_acip_frame_required");
    assertions.push(serde_json::json!({
        "name": "text_frame_rejected",
        "class": "negative_case",
        "result": "passed",
        "evidence": "the production listener closed text traffic with the binary-only policy code"
    }));

    let response = shutdown(config, address, &instance_id).await;
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    let status = loop {
        if let Some(status) = child.0.as_mut().unwrap().try_wait().unwrap() {
            break status;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "production kernel did not stop within its shutdown budget"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    let stderr = stderr_reader.join().unwrap();
    assert!(
        status.success(),
        "kernel shutdown failed ({status}): {stderr}"
    );
    assertions.push(serde_json::json!({
        "name": "signed_graceful_shutdown",
        "class": "lifecycle",
        "result": "passed",
        "evidence": "the exact production binary accepted signed shutdown and exited successfully"
    }));

    if let Some(path) = std::env::var_os("ADL_ACIP_PROOF_OUTPUT") {
        let successful_exchanges = assertions
            .iter()
            .filter(|assertion| assertion["class"] == "successful_exchange")
            .count();
        let negative_cases = assertions
            .iter()
            .filter(|assertion| assertion["class"] == "negative_case")
            .count();
        let proof = serde_json::json!({
            "schema": "adl.acip_native_platform_proof.v2",
            "platform": std::env::var("ADL_ACIP_PLATFORM").unwrap_or_else(|_| "linux".to_owned()),
            "producer": "adl-runtime-kernel/tests/production_acip_wss.rs::production_binary_acip_wss_produces_observed_receipt",
            "production_binary": "CARGO_BIN_EXE_adl-runtime-kernel",
            "successful_exchanges": successful_exchanges,
            "negative_cases": negative_cases,
            "assertions": assertions,
        });
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, serde_json::to_vec_pretty(&proof).unwrap()).unwrap();
    }
}
