use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    serve_control_listener, AdapterKind, AdapterPolicy, AgentPopulationFeed, AuthorityMode,
    CanonicalIngress, ComponentId, ComponentRegistry, ControlApiPolicy, ControlAuthority,
    ControlService, ExecutorError, FailureClass, Kernel, KernelExit, LifecycleControl,
    OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory, RunningState,
    RuntimeRecorder, OBSERVATORY_FEED_SCHEMA, OBSERVATORY_WS_AUTH_SCHEMA,
    OBSERVATORY_WS_CONTROL_RESULT_SCHEMA, OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA,
    OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
    OBSERVATORY_WS_PATH,
};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio_rustls::rustls::ClientConfig;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{client::IntoClientRequest, http::HeaderValue, Message},
    Connector,
};

#[path = "../../adl-runtime/tests/support/tls.rs"]
mod tls_support;
use tls_support::TestPki;

const TOKEN: &str = "conversation-test-token-000000000001";

struct FakeLifecycle;
struct ConversationExecutor {
    dispatches: Arc<AtomicUsize>,
    completions: Arc<AtomicUsize>,
}

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}

#[async_trait]
impl OperationExecutor for ConversationExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        let work: serde_json::Value =
            serde_json::from_slice(&request.payload).map_err(|error| ExecutorError {
                class: FailureClass::Fatal,
                message: error.to_string(),
            })?;
        let recipient_id =
            work["tasks"][0]["recipient_id"]
                .as_str()
                .ok_or_else(|| ExecutorError {
                    class: FailureClass::Fatal,
                    message: "missing recipient".to_owned(),
                })?;
        let projected_recipient = if work["tasks"][0]["input"] == "forge recipient" {
            "agent-9999"
        } else {
            recipient_id
        };
        if work["tasks"][0]["input"] == "delay" {
            tokio::time::sleep(Duration::from_millis(250)).await;
        } else if work["tasks"][0]["input"] == "delay ordered" {
            tokio::time::sleep(Duration::from_millis(60)).await;
        } else if work["tasks"][0]["input"] == "delay budget" {
            tokio::time::sleep(Duration::from_millis(70)).await;
        }
        self.completions.fetch_add(1, Ordering::SeqCst);
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.runtime.local_agent_execution.v1",
            "outputs": [{
                "unit": 0,
                "output": {
                    "recipient_id": projected_recipient,
                    "message": format!("{recipient_id} received your message."),
                    "adapter_secret": "must-not-cross-public-boundary"
                }
            }]
        }))
        .map_err(|error| ExecutorError {
            class: FailureClass::Fatal,
            message: error.to_string(),
        })
    }
}

#[tokio::test]
async fn authenticated_selected_agent_conversation_uses_canonical_wss_ingress() {
    let recorder = RuntimeRecorder::new(32);
    let dispatches = Arc::new(AtomicUsize::new(0));
    let completions = Arc::new(AtomicUsize::new(0));
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity: 4,
                max_in_flight: 2,
                shutdown_grace_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            Arc::new(ConversationExecutor {
                dispatches: dispatches.clone(),
                completions: completions.clone(),
            }),
        )
        .unwrap(),
    );
    let operation = OperationalFactory::new(adapter, vec![]);
    let ingress = CanonicalIngress::new(
        4,
        recorder.clone(),
        BTreeMap::from([("agent_runtime".to_owned(), operation.clone())]),
    );
    let admitted_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    assert!(recorder.record_agent_admission(
        "shepherd",
        admitted_at,
        admitted_at + 30_000,
        "1111111111111111111111111111111111111111",
    ));
    let service = Arc::new(
        ControlService::new_with_observatory_config_and_agents(
            "conversation-runtime",
            recorder.clone(),
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            8,
            ["https://observatory.example.test".to_owned()],
            AgentPopulationFeed::resident_shepherd(),
        )
        .with_canonical_ingress(ingress.clone()),
    );
    service.set_observatory_bearer_token(TOKEN).unwrap();
    service
        .set_public_base_url("https://observatory.example.test:20997")
        .unwrap();

    let pki = TestPki::new("conversation wss");
    let identity = pki.server(&["localhost"]);
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem(
        identity.certificate_pem(),
        identity.private_key_pem(),
    )
    .await
    .unwrap();
    let connector = Connector::Rustls(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(pki.roots())
            .with_no_client_auth(),
    ));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let mut registry = ComponentRegistry::new();
    registry.register(operation);
    registry.register(ingress);
    let kernel = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    let server = tokio::spawn(async move {
        serve_control_listener(
            service,
            listener,
            tls,
            ControlApiPolicy::new(
                Duration::from_secs(2),
                Duration::from_millis(100),
                Duration::from_millis(20),
                64 * 1024,
            )
            .unwrap(),
        )
        .await
    });

    let mut request = format!("wss://localhost:{}{OBSERVATORY_WS_PATH}", address.port())
        .into_client_request()
        .unwrap();
    request.headers_mut().insert(
        "Origin",
        HeaderValue::from_static("https://observatory.example.test"),
    );
    let (mut socket, _) = connect_async_tls_with_config(request, None, false, Some(connector))
        .await
        .unwrap();
    let feed: serde_json::Value =
        serde_json::from_str(socket.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();
    assert_eq!(feed["schema"], OBSERVATORY_FEED_SCHEMA);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": TOKEN,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let authenticated =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-positive",
                "recipient_id": "shepherd",
                "correlation_id": "0123456789abcdef0123456789abcdef",
                "message": "Hello"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    assert_eq!(accepted["turn_sequence"], 1);
    let delivered =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(delivered["status"], "delivered");
    assert_eq!(delivered["reply"], "shepherd received your message.");
    assert!(!delivered.to_string().contains("adapter_secret"));
    assert_eq!(dispatches.load(Ordering::SeqCst), 1);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-positive",
                "recipient_id": "shepherd",
                "correlation_id": "0123456789abcdef0123456789abcdef",
                "message": "Hello"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let duplicate =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(duplicate["status"], "delivered");
    assert_eq!(dispatches.load(Ordering::SeqCst), 1);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-positive",
                "recipient_id": "shepherd",
                "correlation_id": "0123456789abcdef0123456789abcdef",
                "message": "Changed payload"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let conflict =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(conflict["status"], "refused");
    assert_eq!(conflict["error"], "conversation_conflict");
    assert_eq!(dispatches.load(Ordering::SeqCst), 1);

    for (turn_id, correlation_id, message) in [
        (
            "turn-ordered-1",
            "55555555555555555555555555555555",
            "delay ordered",
        ),
        (
            "turn-ordered-2",
            "66666666666666666666666666666666",
            "Hello",
        ),
    ] {
        socket
            .send(Message::Text(
                serde_json::json!({
                    "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                    "conversation_id": "conversation-shepherd",
                    "turn_id": turn_id,
                    "recipient_id": "shepherd",
                    "correlation_id": correlation_id,
                    "message": message
                })
                .to_string()
                .into(),
            ))
            .await
            .unwrap();
    }
    let first_accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let second_accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(first_accepted["turn_id"], "turn-ordered-1");
    assert_eq!(second_accepted["turn_id"], "turn-ordered-2");
    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Degraded);
    let first_terminal =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let second_terminal =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(first_terminal["turn_id"], "turn-ordered-1");
    assert_eq!(first_terminal["status"], "delivered");
    assert_eq!(second_terminal["turn_id"], "turn-ordered-2");
    assert_eq!(second_terminal["status"], "refused");
    assert_eq!(second_terminal["error"], "recipient_unavailable");
    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    let resumed_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    assert!(recorder.record_agent_heartbeat("shepherd", resumed_at, resumed_at + 30_000,));

    for (turn_id, correlation_id) in [
        ("turn-budget-1", "77777777777777777777777777777777"),
        ("turn-budget-2", "88888888888888888888888888888888"),
    ] {
        socket
            .send(Message::Text(
                serde_json::json!({
                    "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                    "conversation_id": "conversation-shepherd",
                    "turn_id": turn_id,
                    "recipient_id": "shepherd",
                    "correlation_id": correlation_id,
                    "message": "delay budget"
                })
                .to_string()
                .into(),
            ))
            .await
            .unwrap();
    }
    let _ = next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let _ = next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let budget_first =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let budget_second =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(budget_first["turn_id"], "turn-budget-1");
    assert_eq!(budget_first["status"], "delivered");
    assert_eq!(budget_second["turn_id"], "turn-budget-2");
    assert_eq!(budget_second["status"], "timed_out");

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-unknown",
                "turn_id": "turn-unknown",
                "recipient_id": "agent-9999",
                "correlation_id": "abcdef0123456789abcdef0123456789",
                "message": "Hello"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let refused =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(refused["status"], "refused");
    assert_eq!(refused["error"], "unknown_recipient");

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-forged-output",
                "recipient_id": "shepherd",
                "correlation_id": "11111111111111111111111111111111",
                "message": "forge recipient"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let failed =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(failed["status"], "accepted");
    let failed =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(failed["status"], "failed");
    assert!(failed.get("reply").is_none());

    let delayed_intent = serde_json::json!({
        "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
        "conversation_id": "conversation-shepherd",
        "turn_id": "turn-disconnect",
        "recipient_id": "shepherd",
        "correlation_id": "22222222222222222222222222222222",
        "message": "delay"
    });
    socket
        .send(Message::Text(delayed_intent.to_string().into()))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted");
    socket.close(None).await.unwrap();
    tokio::time::sleep(Duration::from_millis(150)).await;

    let mut request = format!("wss://localhost:{}{OBSERVATORY_WS_PATH}", address.port())
        .into_client_request()
        .unwrap();
    request.headers_mut().insert(
        "Origin",
        HeaderValue::from_static("https://observatory.example.test"),
    );
    let (mut socket, _) = connect_async_tls_with_config(
        request,
        None,
        false,
        Some(Connector::Rustls(Arc::new(
            ClientConfig::builder()
                .with_root_certificates(pki.roots())
                .with_no_client_auth(),
        ))),
    )
    .await
    .unwrap();
    let _ = next_frame_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": TOKEN,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let _ = next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    socket
        .send(Message::Text(delayed_intent.to_string().into()))
        .await
        .unwrap();
    let timed_out =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(timed_out["status"], "timed_out");
    assert_eq!(dispatches.load(Ordering::SeqCst), 6);
    assert_eq!(completions.load(Ordering::SeqCst), 4);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-cancel",
                "recipient_id": "shepherd",
                "correlation_id": "33333333333333333333333333333333",
                "message": "delay"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted");
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-cancel",
                "correlation_id": "33333333333333333333333333333333"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let first =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let second =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    let statuses = [
        first["status"].as_str().unwrap(),
        second["status"].as_str().unwrap(),
    ];
    assert!(statuses.contains(&"accepted"));
    assert!(statuses.contains(&"cancelled"));
    tokio::time::sleep(Duration::from_millis(275)).await;
    assert_eq!(completions.load(Ordering::SeqCst), 4);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-shepherd",
                "turn_id": "turn-over-capacity",
                "recipient_id": "shepherd",
                "correlation_id": "44444444444444444444444444444444",
                "message": "Hello"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let capacity =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(capacity["status"], "failed");
    assert_eq!(capacity["error"], "conversation_capacity_exhausted");

    socket.close(None).await.unwrap();
    server.abort();
    kernel.shutdown(Duration::from_secs(1)).await.unwrap();
}

async fn next_frame_with_schema<S>(
    socket: &mut tokio_tungstenite::WebSocketStream<S>,
    schema: &str,
) -> serde_json::Value
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    loop {
        let frame = socket.next().await.unwrap().unwrap();
        let value: serde_json::Value = serde_json::from_str(frame.to_text().unwrap()).unwrap();
        if value["schema"] == schema {
            return value;
        }
    }
}
