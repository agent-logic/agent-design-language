#![cfg(unix)]

use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    encode_acip_envelope, CanonicalIngress, ControlAction, DomainWork, IngressError,
    RuntimeRecorder, SignedControlCommand, ACIP_WEBSOCKET_SCHEMA, DOMAIN_WORK_SCHEMA,
};
use ed25519_dalek::SigningKey;
use futures::{SinkExt, StreamExt};
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
            let mut acknowledgement = b"ok".to_vec();
            acknowledgement.extend_from_slice(&std::process::id().to_be_bytes());
            stream.write_all(&acknowledgement).unwrap();
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
    let body = serde_json::to_vec(&command).unwrap();
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

#[tokio::test]
async fn production_binary_acip_wss_produces_observed_receipt() {
    let directory = tempfile::tempdir().unwrap();
    let root = state_root(directory.path());
    let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = probe.local_addr().unwrap();
    drop(probe);
    let (init, certificate_der) =
        runtime_init::write_with_certificate_for_state_and_ingress_capacity(
            directory.path(),
            address,
            &root,
            1,
        );
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
    let instance_id = match ready_rx.recv_timeout(Duration::from_secs(10)) {
        Ok(instance_id) => instance_id,
        Err(error) => {
            let mut process = child.0.take().unwrap();
            if matches!(process.try_wait(), Ok(None)) {
                let _ = process.kill();
            }
            let status = process.wait().unwrap();
            let stderr = stderr_reader.join().unwrap();
            panic!(
                "production kernel did not report control readiness ({error}; {status}): {stderr}"
            );
        }
    };

    let mut assertions = Vec::new();
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

    let mut pressure_sockets = Vec::new();
    let mut pressure_frames = Vec::new();
    for index in 0..4 {
        let stream = tokio::net::TcpStream::connect(address).await.unwrap();
        let (mut pressure_socket, _) = tokio_tungstenite::client_async_tls_with_config(
            acip_request(address, ACIP_WRITE_TOKEN),
            stream,
            None,
            Some(Connector::Rustls(config.clone())),
        )
        .await
        .unwrap();
        assert_eq!(
            next_acip_json(&mut pressure_socket).await["event"],
            "authenticated"
        );
        let frame = encode_acip_envelope(
            &format!("production-acip-pressure-{index}"),
            &format!("pressure-client-{index}"),
            "runtime",
            "agent_runtime",
            &serde_json::json!({
                "schema": "adl.runtime.local_agent_work.v1",
                "tasks": [{"op": "sleep_millis", "millis": 250}]
            }),
            1,
        )
        .unwrap();
        pressure_socket
            .send(Message::Binary(frame.clone().into()))
            .await
            .unwrap();
        pressure_sockets.push(pressure_socket);
        pressure_frames.push(frame);
    }
    let mut rejected_pressure = None;
    for (index, pressure_socket) in pressure_sockets.iter_mut().enumerate() {
        let result = next_acip_json(pressure_socket).await;
        if result["status"] == "rejected" {
            assert_eq!(result["reason"], "canonical ingress is saturated");
            assert_eq!(result["sequence_reserved"], false);
            rejected_pressure = Some(index);
        }
    }
    let rejected_pressure =
        rejected_pressure.expect("production WSS never observed bounded pressure");
    pressure_sockets[rejected_pressure]
        .send(Message::Binary(
            pressure_frames[rejected_pressure].clone().into(),
        ))
        .await
        .unwrap();
    let recovered = next_acip_json(&mut pressure_sockets[rejected_pressure]).await;
    assert_eq!(recovered["status"], "completed", "{recovered}");
    assertions.push(serde_json::json!({
        "name": "production_wss_pressure_rolls_back_and_recovers",
        "class": "negative_case",
        "result": "passed",
        "evidence": "the production WSS endpoint returned typed saturation, released its replay reservation, and completed an exact corrected retry"
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

    let terminal_sequence = encode_acip_envelope(
        "production-acip-terminal-sequence",
        "hostile-proof-client",
        "runtime",
        "agent_runtime",
        &serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{"op": "blake3", "input": "must not reserve terminal sequence"}]
        }),
        u64::MAX,
    )
    .unwrap();
    socket
        .send(Message::Binary(terminal_sequence.into()))
        .await
        .unwrap();
    let terminal_rejection = next_acip_json(&mut socket).await;
    assert_eq!(terminal_rejection["status"], "rejected");
    assert_eq!(
        terminal_rejection["reason"],
        "monotonic_sequence_must_advance"
    );

    let isolated = encode_acip_envelope(
        "production-acip-isolated-after-terminal",
        "independent-proof-client",
        "runtime",
        "agent_runtime",
        &serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{"op": "blake3", "input": "independent replay domain remains live"}]
        }),
        1,
    )
    .unwrap();
    socket.send(Message::Binary(isolated.into())).await.unwrap();
    let isolated_completion = next_acip_json(&mut socket).await;
    assert_eq!(isolated_completion["status"], "completed");
    assertions.push(serde_json::json!({
        "name": "terminal_sequence_rejected_without_cross_domain_poisoning",
        "class": "negative_case",
        "result": "passed",
        "evidence": "u64::MAX was rejected and an independent authenticated replay domain still completed sequence 1"
    }));

    let unsupported = encode_acip_envelope(
        "production-acip-retryable",
        "retry-proof-client",
        "runtime",
        "not_allowlisted",
        &serde_json::json!({"schema": "adl.runtime.unsupported.v1"}),
        1,
    )
    .unwrap();
    socket
        .send(Message::Binary(unsupported.into()))
        .await
        .unwrap();
    let typed_error = next_acip_json(&mut socket).await;
    assert_eq!(typed_error["status"], "rejected");
    assert_eq!(typed_error["reason"], "domain work kind is not allowlisted");
    assert_eq!(typed_error["sequence_reserved"], false);

    let retry = encode_acip_envelope(
        "production-acip-retryable",
        "retry-proof-client",
        "runtime",
        "agent_runtime",
        &serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{"op": "blake3", "input": "typed rejection rolled back reservation"}]
        }),
        1,
    )
    .unwrap();
    socket.send(Message::Binary(retry.into())).await.unwrap();
    let retry_completion = next_acip_json(&mut socket).await;
    assert_eq!(retry_completion["status"], "completed");
    assertions.push(serde_json::json!({
        "name": "typed_ingress_error_rolls_back_sequence",
        "class": "negative_case",
        "result": "passed",
        "evidence": "unsupported work produced a structured rejection and corrected work reused the sequence successfully"
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

#[tokio::test]
async fn canonical_ingress_refuses_before_kernel_port_binding() {
    let ingress = CanonicalIngress::new(1, RuntimeRecorder::new(8), BTreeMap::new());
    let refused = ingress
        .submit(
            DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "pressure-rejected".to_owned(),
                kind: "agent_runtime".to_owned(),
                payload: br#"{"schema":"adl.runtime.local_agent_work.v1"}"#.to_vec(),
            },
            "pressure-rejected".to_owned(),
        )
        .await;
    assert_eq!(refused, Err(IngressError::Closed));
}
