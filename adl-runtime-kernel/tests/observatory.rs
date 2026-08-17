use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    encode_acip_envelope, serve_control_listener, AdapterKind, AdapterPolicy, AuthorityMode,
    CanonicalIngress, ComponentRegistry, ControlAction, ControlApiPolicy, ControlAuthority,
    ControlCapability, ControlService, ExecutorError, FailureClass, Kernel, KernelExit,
    LifecycleControl, OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory,
    OperatorAttentionError as AttentionError, OperatorAttentionIdentity as AttentionSourceIdentity,
    OperatorAttentionInbox, OperatorAttentionOutcome as AttentionOutcome,
    OperatorAttentionPriority as AttentionPriority, OperatorAttentionReason as AttentionReason,
    OperatorAttentionRequestInput as AttentionRequestInput,
    OperatorAttentionSettings as AttentionInboxConfig, OperatorAttentionStatus as AttentionStatus,
    RuntimeRecorder, SignedControlCommand, TrustedControlKey, ACIP_WEBSOCKET_SCHEMA,
    OBSERVATORY_FEED_SCHEMA, OBSERVATORY_WS_AUTH_SCHEMA, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
    OBSERVATORY_WS_PATH, OPERATOR_ATTENTION_REQUEST_SCHEMA,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use futures::{SinkExt, StreamExt};
use tokio_rustls::rustls::ClientConfig;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderValue, Request},
        Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

#[path = "../../adl-runtime/tests/support/tls.rs"]
mod tls_support;
use tls_support::TestPki;

const ACIP_WRITE_TOKEN: &str = "test-acip-write-token-000000000001";

fn attention_input(source: &str, correlation: &str) -> AttentionRequestInput {
    AttentionRequestInput {
        schema: OPERATOR_ATTENTION_REQUEST_SCHEMA.to_owned(),
        source_agent_id: source.to_owned(),
        source_identity: AttentionSourceIdentity {
            agent_id: source.to_owned(),
            principal_id: format!("{source}-principal"),
            display_name: Some(format!("{source} display")),
            can_mark_urgent: false,
            can_request_attention: true,
        },
        reason: AttentionReason::Clarification,
        priority: AttentionPriority::Normal,
        correlation_id: correlation.to_owned(),
        message: "Need operator guidance for the next bounded step.".to_owned(),
        created_at_millis: 1_000,
        expires_at_millis: Some(10_000),
        related_conversation_id: Some("conversation-1".to_owned()),
        related_work_id: Some("work-1".to_owned()),
        group_key: Some("operator-work-group".to_owned()),
    }
}

fn trust_attention_source(inbox: &mut OperatorAttentionInbox, source: &str, can_mark_urgent: bool) {
    inbox
        .trust_source(AttentionSourceIdentity {
            agent_id: source.to_owned(),
            principal_id: format!("{source}-principal"),
            display_name: Some(format!("{source} display")),
            can_mark_urgent,
            can_request_attention: true,
        })
        .unwrap();
}

#[test]
fn operator_attention_deduplicates_and_rate_limits_by_source() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig {
        capacity: 8,
        max_active_per_source: 1,
        max_message_chars: 256,
        grouping_window_millis: 0,
        ..AttentionInboxConfig::default()
    })
    .unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    trust_attention_source(&mut inbox, "agent-b", false);
    let first_id = inbox
        .submit(attention_input("agent-a", "corr-agent-a-1"))
        .unwrap()
        .request_id
        .clone();

    let duplicate = inbox
        .submit(attention_input("agent-a", "corr-agent-a-1"))
        .unwrap();
    assert_eq!(duplicate.request_id, first_id);
    assert_eq!(duplicate.duplicate_count, 1);
    assert_eq!(
        inbox.submit(attention_input("agent-a", "corr-agent-a-2")),
        Err(AttentionError::RateLimited)
    );
    let snapshot_before_stale_duplicate = inbox.snapshot(1_002);
    let request_before_stale_duplicate = snapshot_before_stale_duplicate
        .requests
        .iter()
        .find(|request| request.request_id == first_id)
        .expect("request remains available")
        .clone();
    let mut stale_duplicate = attention_input("agent-a", "corr-agent-a-1");
    stale_duplicate.created_at_millis = request_before_stale_duplicate
        .updated_at_millis
        .saturating_sub(1);
    assert_eq!(
        inbox.submit(stale_duplicate),
        Err(AttentionError::InvalidRequest(
            "request_timestamp_monotonic"
        ))
    );
    let snapshot_after_stale_duplicate = inbox.snapshot(1_003);
    let request_after_stale_duplicate = snapshot_after_stale_duplicate
        .requests
        .iter()
        .find(|request| request.request_id == first_id)
        .expect("request remains available after stale duplicate");
    assert_eq!(
        request_after_stale_duplicate.duplicate_count,
        request_before_stale_duplicate.duplicate_count
    );
    assert_eq!(
        request_after_stale_duplicate.updated_at_millis,
        request_before_stale_duplicate.updated_at_millis
    );

    inbox
        .submit(attention_input("agent-b", "corr-agent-b-1"))
        .unwrap();
    assert_eq!(inbox.snapshot(1_010).open_count, 2);
}

#[test]
fn operator_attention_quiet_mode_suppresses_non_urgent_noise() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig {
        quiet_mode: true,
        ..AttentionInboxConfig::default()
    })
    .unwrap();
    trust_attention_source(&mut inbox, "agent-a", true);

    assert_eq!(
        inbox.submit(attention_input("agent-a", "corr-quiet-normal")),
        Err(AttentionError::QuietModeSuppressed)
    );

    let mut urgent = attention_input("agent-a", "corr-quiet-urgent");
    urgent.priority = AttentionPriority::Urgent;
    urgent.source_identity.can_mark_urgent = true;
    assert_eq!(
        inbox.submit(urgent).unwrap().priority,
        AttentionPriority::Urgent
    );
}

#[test]
fn operator_attention_groups_related_requests_without_new_rows() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig {
        grouping_window_millis: 500,
        max_active_per_source: 4,
        ..AttentionInboxConfig::default()
    })
    .unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let first_id = inbox
        .submit(attention_input("agent-a", "corr-group-1"))
        .unwrap()
        .request_id
        .clone();
    let mut grouped = attention_input("agent-a", "corr-group-2");
    grouped.created_at_millis = 1_250;
    grouped.priority = AttentionPriority::High;
    let grouped_result = inbox.submit(grouped).unwrap();
    assert_eq!(grouped_result.request_id, first_id);
    assert_eq!(grouped_result.grouped_count, 1);
    assert_eq!(grouped_result.priority, AttentionPriority::High);
    assert_eq!(inbox.snapshot(1_260).requests.len(), 1);

    let mut later = attention_input("agent-a", "corr-group-3");
    later.created_at_millis = 1_750;
    later.group_key = Some("operator-work-group-later".to_owned());
    inbox.submit(later).unwrap();
    assert_eq!(inbox.snapshot(1_760).requests.len(), 2);
}

#[test]
fn operator_attention_rejects_spoofed_identity_and_unauthorized_urgency() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig::default()).unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let mut spoofed = attention_input("agent-a", "corr-spoofed");
    spoofed.source_identity.agent_id = "agent-b".to_owned();
    assert_eq!(
        inbox.submit(spoofed),
        Err(AttentionError::UnauthorizedSource)
    );

    let mut urgent = attention_input("agent-a", "corr-urgent-denied");
    urgent.priority = AttentionPriority::Urgent;
    assert_eq!(
        inbox.submit(urgent),
        Err(AttentionError::UnauthorizedUrgency)
    );

    let mut allowed = attention_input("agent-a", "corr-urgent-allowed");
    allowed.priority = AttentionPriority::Urgent;
    allowed.source_identity.can_mark_urgent = true;
    assert_eq!(
        inbox.submit(allowed.clone()),
        Err(AttentionError::UnauthorizedUrgency),
        "caller-controlled urgent privilege must not override trusted source policy"
    );
    trust_attention_source(&mut inbox, "agent-a", true);
    assert_eq!(
        inbox.submit(allowed).unwrap().priority,
        AttentionPriority::Urgent
    );

    let mut forged_principal = attention_input("agent-a", "corr-principal");
    forged_principal.source_identity.principal_id = "attacker-principal".to_owned();
    assert_eq!(
        inbox.submit(forged_principal),
        Err(AttentionError::UnauthorizedSource)
    );
}

#[test]
fn operator_attention_outcomes_do_not_create_authority_approval() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig::default()).unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let request_id = inbox
        .submit(attention_input("agent-a", "corr-outcome"))
        .unwrap()
        .request_id
        .clone();
    let replied = inbox
        .apply_outcome(
            &request_id,
            "operator-a",
            AttentionOutcome::Reply {
                message: "Acknowledged; continue only after Runtime authorization.".to_owned(),
            },
            1_100,
        )
        .unwrap();
    assert_eq!(replied.status, AttentionStatus::Replied);
    let encoded = serde_json::to_string(replied).unwrap();
    assert!(!encoded.contains("approved"));
    assert!(!encoded.contains("capability"));

    inbox
        .apply_outcome(&request_id, "operator-a", AttentionOutcome::Resolve, 1_101)
        .unwrap();
    assert_eq!(
        inbox.apply_outcome(
            &request_id,
            "operator-a",
            AttentionOutcome::Acknowledge,
            1_102
        ),
        Err(AttentionError::TerminalRequest)
    );
}

#[test]
fn operator_attention_invalid_outcomes_do_not_mutate_request_state() {
    let mut inbox = OperatorAttentionInbox::new(AttentionInboxConfig::default()).unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let request_id = inbox
        .submit(attention_input("agent-a", "corr-invalid-outcome"))
        .unwrap()
        .request_id
        .clone();

    let before = inbox.snapshot(1_000);
    let request_before = before
        .requests
        .iter()
        .find(|request| request.request_id == request_id)
        .cloned()
        .unwrap();
    let events_before = before.events.len();

    assert_eq!(
        inbox.apply_outcome(
            &request_id,
            "operator-a",
            AttentionOutcome::Reply {
                message: "   ".to_owned(),
            },
            1_100,
        ),
        Err(AttentionError::InvalidRequest("reply_required"))
    );
    assert_eq!(
        inbox.apply_outcome(
            &request_id,
            "operator-a",
            AttentionOutcome::Refuse {
                reason: "".to_owned(),
            },
            1_101,
        ),
        Err(AttentionError::InvalidRequest("refusal_reason_required"))
    );
    assert_eq!(
        inbox.apply_outcome(
            &request_id,
            "operator-a",
            AttentionOutcome::Defer {
                until_millis: 1_101
            },
            1_101,
        ),
        Err(AttentionError::InvalidRequest("defer_until_future"))
    );
    assert_eq!(
        inbox.apply_outcome(&request_id, "operator-a", AttentionOutcome::Resolve, 999),
        Err(AttentionError::InvalidRequest(
            "outcome_timestamp_monotonic"
        ))
    );

    let after = inbox.snapshot(1_200);
    let request_after = after
        .requests
        .iter()
        .find(|request| request.request_id == request_id)
        .cloned()
        .unwrap();
    assert_eq!(request_after.status, request_before.status);
    assert_eq!(
        request_after.updated_at_millis,
        request_before.updated_at_millis
    );
    assert_eq!(
        request_after.operator_response,
        request_before.operator_response
    );
    assert_eq!(
        request_after.deferred_until_millis,
        request_before.deferred_until_millis
    );
    assert_eq!(after.events.len(), events_before);
}

#[test]
fn operator_attention_expiry_and_restart_preserve_receipts() {
    let settings = AttentionInboxConfig::default();
    let mut inbox = OperatorAttentionInbox::new(settings.clone()).unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let request_id = inbox
        .submit(attention_input("agent-a", "corr-expiry"))
        .unwrap()
        .request_id
        .clone();
    assert_eq!(inbox.expire(10_000), 1);
    let snapshot = inbox.snapshot(10_001);
    assert_eq!(snapshot.open_count, 0);
    assert_eq!(snapshot.requests[0].request_id, request_id);
    assert_eq!(snapshot.requests[0].status, AttentionStatus::Expired);

    let mut restored = OperatorAttentionInbox::restore(settings, snapshot).unwrap();
    let snapshot_before_duplicate = restored.snapshot(10_002);
    let events_before_duplicate = snapshot_before_duplicate.events.len();
    let restored_before_duplicate = snapshot_before_duplicate
        .requests
        .iter()
        .find(|request| request.request_id == request_id)
        .expect("restored request remains available")
        .clone();
    let duplicate = restored
        .submit(attention_input("agent-a", "corr-expiry"))
        .unwrap();
    assert_eq!(duplicate.request_id, request_id);
    assert_eq!(duplicate.status, AttentionStatus::Expired);
    assert_eq!(
        duplicate.duplicate_count,
        restored_before_duplicate.duplicate_count
    );
    assert_eq!(
        duplicate.updated_at_millis,
        restored_before_duplicate.updated_at_millis
    );
    assert_eq!(
        restored.snapshot(10_003).events.len(),
        events_before_duplicate
    );
}

#[test]
fn operator_attention_restore_rejects_malformed_snapshot_state() {
    let settings = AttentionInboxConfig {
        capacity: 1,
        max_active_per_source: 1,
        max_message_chars: 256,
        ..AttentionInboxConfig::default()
    };
    let mut inbox = OperatorAttentionInbox::new(settings.clone()).unwrap();
    trust_attention_source(&mut inbox, "agent-a", false);
    let request = inbox
        .submit(attention_input("agent-a", "corr-restore"))
        .unwrap()
        .clone();

    let mut duplicate = inbox.snapshot(2_000);
    duplicate.requests.push(request.clone());
    assert_eq!(
        OperatorAttentionInbox::restore(settings.clone(), duplicate).unwrap_err(),
        AttentionError::InvalidRequest("snapshot_invalid")
    );

    let mut bad_schema = inbox.snapshot(2_000);
    bad_schema.requests[0].schema = "wrong".to_owned();
    assert_eq!(
        OperatorAttentionInbox::restore(settings.clone(), bad_schema).unwrap_err(),
        AttentionError::SchemaMismatch
    );

    let mut bad_event = inbox.snapshot(2_000);
    bad_event.events[0].request_id = "missing".to_owned();
    assert_eq!(
        OperatorAttentionInbox::restore(settings, bad_event).unwrap_err(),
        AttentionError::InvalidRequest("snapshot_event_invalid")
    );
}

struct FakeLifecycle;

struct EchoExecutor;

struct FailOnceExecutor {
    attempts: AtomicUsize,
}

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}

#[async_trait]
impl OperationExecutor for EchoExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Ok(request.payload.clone())
    }
}

#[async_trait]
impl OperationExecutor for FailOnceExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        if self.attempts.fetch_add(1, Ordering::SeqCst) == 0 {
            return Err(ExecutorError {
                class: FailureClass::Fatal,
                message: "injected dispatch failure".to_owned(),
            });
        }
        Ok(request.payload.clone())
    }
}

#[derive(Clone)]
struct TestService {
    service: Arc<ControlService<FakeLifecycle>>,
    operation: OperationalFactory,
    ingress: CanonicalIngress,
    signing_key: SigningKey,
}

fn service(token: &str) -> TestService {
    service_with_executor(token, Arc::new(EchoExecutor))
}

fn service_with_executor(token: &str, executor: Arc<dyn OperationExecutor>) -> TestService {
    let key = SigningKey::from_bytes(&[42; 32]);
    let authority = ControlAuthority::new(BTreeMap::from([(
        "operator-key".to_owned(),
        TrustedControlKey {
            principal: "operator".to_owned(),
            verifying_key: key.verifying_key(),
            capabilities: BTreeSet::from([ControlCapability::Read]),
        },
    )]));
    let recorder = RuntimeRecorder::new(8);
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Acip,
            AdapterPolicy {
                capacity: 8,
                max_in_flight: 4,
                shutdown_grace_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            executor,
        )
        .unwrap(),
    );
    let operation = OperationalFactory::new(adapter, vec![]);
    let ingress = CanonicalIngress::new(
        8,
        recorder.clone(),
        BTreeMap::from([("acip".to_owned(), operation.clone())]),
    );
    let service = Arc::new(
        ControlService::new_with_observatory_config(
            "instance-ws",
            recorder,
            FakeLifecycle,
            authority,
            8,
            ["https://observatory.example.test".to_owned()],
        )
        .with_canonical_ingress(ingress.clone()),
    );
    service.set_observatory_bearer_token(token).unwrap();
    service
        .set_acip_write_bearer_token(ACIP_WRITE_TOKEN)
        .unwrap();
    service
        .set_public_base_url("https://observatory.example.test:20997")
        .unwrap();
    TestService {
        service,
        operation,
        ingress,
        signing_key: key,
    }
}

async fn websocket_server(
    test_service: TestService,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
) {
    let pki = TestPki::new("kernel observatory wss");
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
    registry.register(test_service.operation);
    registry.register(test_service.ingress);
    let kernel = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(8))
        .start()
        .await
        .unwrap();
    let server = tokio::spawn(async move {
        let result = serve_control_listener(
            test_service.service,
            listener,
            tls,
            ControlApiPolicy::new(
                Duration::from_secs(2),
                Duration::from_secs(5),
                Duration::from_secs(1),
                64 * 1024,
            )
            .unwrap(),
        )
        .await;
        let _ = kernel.shutdown(Duration::from_secs(1)).await;
        result
    });
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

fn acip_request(address: std::net::SocketAddr, token: &str) -> Request<()> {
    let mut request = format!("wss://localhost:{}/v1/acip/ws", address.port())
        .into_client_request()
        .unwrap();
    request.headers_mut().insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    request
}

type TestSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

async fn connect_authenticated(
    address: std::net::SocketAddr,
    connector: Connector,
    token: &str,
) -> TestSocket {
    let mut socket = connect_public(address, connector).await;
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
    let authenticated =
        next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");
    socket
}

async fn connect_public(address: std::net::SocketAddr, connector: Connector) -> TestSocket {
    let (mut socket, _) = connect_async_tls_with_config(
        request(address, "https://observatory.example.test"),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    socket
}

async fn connect_acip(address: std::net::SocketAddr, connector: Connector) -> TestSocket {
    let (mut socket, _) = connect_async_tls_with_config(
        acip_request(address, ACIP_WRITE_TOKEN),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    let authenticated = next_acip_status(&mut socket).await;
    assert_eq!(authenticated["event"], "authenticated");
    socket
}

async fn next_json_with_schema(socket: &mut TestSocket, schema: &str) -> serde_json::Value {
    tokio::time::timeout(Duration::from_secs(2), async {
        while let Some(message) = socket.next().await {
            if let Ok(Message::Text(payload)) = message {
                let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
                if value["schema"] == schema {
                    return value;
                }
            }
        }
        panic!("Observatory session ended before {schema}");
    })
    .await
    .unwrap()
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
async fn acip_write_credential_is_distinct_from_observatory_read_credential() {
    let observatory_token = "test-observatory-websocket-token-0007";
    let test_service = service(observatory_token);
    let (address, connector, server) = websocket_server(test_service).await;

    let denied = connect_async_tls_with_config(
        acip_request(address, observatory_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("401"));

    let (mut socket, _) = connect_async_tls_with_config(
        acip_request(address, ACIP_WRITE_TOKEN),
        None,
        false,
        Some(connector),
    )
    .await
    .unwrap();
    let authenticated = next_acip_status(&mut socket).await;
    assert_eq!(authenticated["event"], "authenticated");

    let frame = encode_acip_envelope(
        "acip-write-auth-1",
        "source-write-auth",
        "runtime",
        "acip",
        &serde_json::json!({"payload": "credential-separated-dispatch"}),
        1,
    )
    .unwrap();
    socket.send(Message::Binary(frame.into())).await.unwrap();
    let accepted = next_acip_status(&mut socket).await;
    assert_eq!(accepted["status"], "completed");
    assert_eq!(accepted["message_id"], "acip-write-auth-1");
    socket.close(None).await.unwrap();
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_allows_public_reads_and_requires_login_for_writes() {
    let token = "test-observatory-websocket-token-0001";
    let test_service = service(token);
    let signing_key = test_service.signing_key.clone();
    let (address, connector, server) = websocket_server(test_service).await;

    let denied = connect_async_tls_with_config(
        request(address, "https://denied.example.test"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("403"));

    let native_request = format!("wss://localhost:{}{}", address.port(), OBSERVATORY_WS_PATH)
        .into_client_request()
        .unwrap();
    let (mut native_socket, _) =
        connect_async_tls_with_config(native_request, None, false, Some(connector.clone()))
            .await
            .unwrap();
    let native_feed = next_json_with_schema(&mut native_socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(native_feed["runtime_instance_id"], "instance-ws");
    native_socket.close(None).await.unwrap();

    let mut socket = connect_public(address, connector).await;
    let command = SignedControlCommand::sign(
        "login-command",
        "0123456789abcdef0123456789abcdef",
        "instance-ws",
        "operator",
        ControlAction::Snapshot,
        "operator-key",
        &signing_key,
    )
    .unwrap();
    socket
        .send(Message::Text(
            serde_json::to_string(&command).unwrap().into(),
        ))
        .await
        .unwrap();
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "write_authentication_required");

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
    let authenticated =
        next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");

    socket
        .send(Message::Text(
            serde_json::to_string(&command).unwrap().into(),
        ))
        .await
        .unwrap();
    let accepted = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted");
    assert_eq!(accepted["response"]["outcome"]["result"], "snapshot");
    server.abort();
}

#[tokio::test]
async fn canonical_acip_websocket_rejects_replay_after_reconnect() {
    let token = "test-observatory-websocket-token-0007";
    let (address, connector, server) = websocket_server(service(token)).await;
    let mut first = connect_acip(address, connector.clone()).await;
    first
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1",
                "agent-source",
                "runtime-target",
                "acip",
                &serde_json::json!({"message": "first"}),
                1,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let accepted = next_acip_status(&mut first).await;
    assert_eq!(accepted["status"], "completed");
    assert_eq!(accepted["message_id"], "acip-reconnect-1");
    assert_eq!(accepted["sequence_reserved"], true);
    first.close(None).await.unwrap();

    let mut second = connect_acip(address, connector).await;
    second
        .send(Message::Binary(
            encode_acip_envelope(
                "acip-reconnect-1-replay",
                "agent-source",
                "runtime-target",
                "acip",
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
                "acip",
                &serde_json::json!({"message": "second"}),
                2,
            )
            .unwrap()
            .into(),
        ))
        .await
        .unwrap();
    let advanced = next_acip_status(&mut second).await;
    assert_eq!(advanced["status"], "completed");
    assert_eq!(advanced["message_id"], "acip-reconnect-2");
    assert_eq!(advanced["sequence_reserved"], true);
    server.abort();
}

#[tokio::test]
async fn failed_acip_dispatch_releases_sequence_for_retry() {
    let token = "test-observatory-websocket-token-0008";
    let service = service_with_executor(
        token,
        Arc::new(FailOnceExecutor {
            attempts: AtomicUsize::new(0),
        }),
    );
    let (address, connector, server) = websocket_server(service).await;
    let mut socket = connect_acip(address, connector).await;
    let frame = encode_acip_envelope(
        "acip-retry-1",
        "agent-source-retry",
        "runtime-target",
        "acip",
        &serde_json::json!({"message": "retry me"}),
        1,
    )
    .unwrap();

    socket
        .send(Message::Binary(frame.clone().into()))
        .await
        .unwrap();
    let failed = next_acip_status(&mut socket).await;
    assert_eq!(failed["status"], "rejected");
    assert_eq!(failed["sequence_reserved"], false);

    let retry = encode_acip_envelope(
        "acip-retry-2",
        "agent-source-retry",
        "runtime-target",
        "acip",
        &serde_json::json!({"message": "retry after failure"}),
        1,
    )
    .unwrap();
    socket.send(Message::Binary(retry.into())).await.unwrap();
    let retried = next_acip_status(&mut socket).await;
    assert_eq!(retried["status"], "completed");
    assert_eq!(retried["message_id"], "acip-retry-2");
    assert_eq!(retried["sequence_reserved"], true);
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_bad_auth_and_client_data() {
    let token = "test-observatory-websocket-token-0002";
    let (address, connector, server) = websocket_server(service(token)).await;
    let mut socket = connect_public(address, connector.clone()).await;
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
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "authentication_failed");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");

    socket
        .send(Message::Text("{not-json".into()))
        .await
        .unwrap();
    let malformed = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(malformed["status"], "rejected");
    assert_eq!(malformed["error"], "write_authentication_required");

    let mut socket = connect_public(address, connector).await;
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
    let closed = tokio::time::timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let Message::Close(Some(frame)) = closed else {
        panic!("Observatory binary input must close the session");
    };
    assert_eq!(frame.reason, "observatory_binary_frames_unsupported");
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_rejects_a_token_after_rotation() {
    let token = "test-observatory-websocket-token-0003";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let mut socket = connect_public(address, connector).await;
    service
        .service
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
    let rejected = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["error"], "authentication_failed");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    server.abort();
}

#[tokio::test]
async fn observatory_websocket_revokes_an_authenticated_session_after_rotation() {
    let token = "test-observatory-websocket-token-0005";
    let service = service(token);
    let (address, connector, server) = websocket_server(service.clone()).await;
    let mut socket = connect_authenticated(address, connector, token).await;
    service
        .service
        .set_observatory_bearer_token("rotated-observatory-websocket-token-0006")
        .unwrap();
    let revoked = next_json_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(revoked["status"], "rejected");
    assert_eq!(revoked["error"], "credential_revoked");
    let feed = next_json_with_schema(&mut socket, OBSERVATORY_FEED_SCHEMA).await;
    assert_eq!(feed["runtime_instance_id"], "instance-ws");
    server.abort();
}
