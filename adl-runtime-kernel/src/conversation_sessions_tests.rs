use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::control::ConversationAttachmentTestHook;
use crate::{
    serve_control_listener, AdapterKind, AdapterPolicy, AgentPopulationFeed, AgentRosterPolicy,
    AgentSample, AuthorityMode, CanonicalIngress, ComponentId, ComponentRegistry, ControlApiPolicy,
    ControlAuthority, ControlService, ExecutorError, FailureClass, Kernel, KernelExit,
    LifecycleControl, OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory,
    RunningState, RuntimeRecorder, OBSERVATORY_WS_AUTH_SCHEMA,
    OBSERVATORY_WS_CONTROL_RESULT_SCHEMA, OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA,
    OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
    OBSERVATORY_WS_PATH, PREVIOUS_OBSERVATORY_FEED_SCHEMA,
};
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tokio::sync::{Notify, Semaphore};
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
const ROTATED_TOKEN: &str = "rotated-conversation-test-token-0002";

struct CleanupRaceReleaseGuard {
    hook: Arc<ConversationAttachmentTestHook>,
    execution: Arc<Semaphore>,
    execution_released: bool,
    completed: bool,
}

impl CleanupRaceReleaseGuard {
    fn release_execution(&mut self) {
        self.execution.add_permits(1);
        self.execution_released = true;
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

impl Drop for CleanupRaceReleaseGuard {
    fn drop(&mut self) {
        if !self.completed {
            self.hook.release_all();
            if !self.execution_released {
                self.execution.add_permits(1);
            }
        }
    }
}

#[test]
fn cleanup_race_guard_releases_every_barrier_during_unwind() {
    let hook = ConversationAttachmentTestHook::new("cleanup-panic", "turn-panic");
    let execution = Arc::new(Semaphore::new(0));
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe({
        let hook = hook.clone();
        let execution = execution.clone();
        move || {
            let _guard = CleanupRaceReleaseGuard {
                hook,
                execution,
                execution_released: false,
                completed: false,
            };
            panic!("exercise cleanup-race fail-safe release");
        }
    }));
    assert!(unwind.is_err());
    assert_eq!(hook.fail_safe_permits(), (1, 1));
    assert_eq!(execution.available_permits(), 1);
}

struct FakeLifecycle;
struct ConversationExecutor {
    dispatches: Arc<AtomicUsize>,
    completions: Arc<AtomicUsize>,
    barrier_started: Arc<Notify>,
    barrier_release: Arc<Semaphore>,
}

struct ShepherdConversationExecutor {
    dispatches: Arc<AtomicUsize>,
    completions: Arc<AtomicUsize>,
    barrier_started: Arc<Notify>,
    barrier_release: Arc<Semaphore>,
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
        } else if work["tasks"][0]["input"] == "delay revoke" {
            tokio::time::sleep(Duration::from_millis(25)).await;
        } else if work["tasks"][0]["input"] == "barrier cleanup" {
            self.barrier_started.notify_one();
            self.barrier_release
                .acquire()
                .await
                .expect("barrier release semaphore closed")
                .forget();
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

#[async_trait]
impl OperationExecutor for ShepherdConversationExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        let work: crate::ShepherdRequest =
            serde_json::from_slice(&request.payload).map_err(|error| ExecutorError {
                class: FailureClass::Fatal,
                message: error.to_string(),
            })?;
        if work.prompt == "provider failure" {
            return Err(ExecutorError {
                class: FailureClass::Retryable,
                message: "configured provider failed".to_owned(),
            });
        }
        if work.prompt == "delay" {
            tokio::time::sleep(Duration::from_millis(250)).await;
        } else if work.prompt == "delay ordered" {
            tokio::time::sleep(Duration::from_millis(60)).await;
        } else if work.prompt == "delay budget" {
            tokio::time::sleep(Duration::from_millis(70)).await;
        } else if work.prompt == "delay revoke" {
            tokio::time::sleep(Duration::from_millis(25)).await;
        } else if work.prompt == "barrier cleanup" {
            self.barrier_started.notify_one();
            self.barrier_release
                .acquire()
                .await
                .expect("barrier release semaphore closed")
                .forget();
        }
        self.completions.fetch_add(1, Ordering::SeqCst);
        let response = format!("Beacon generated: {}", work.prompt);
        serde_json::to_vec(&crate::ShepherdResponse {
            schema: crate::SHEPHERD_RESPONSE_SCHEMA.to_owned(),
            correlation_id: work.correlation_id,
            runtime_id: work.runtime_id,
            execution_class: crate::ShepherdExecutionClass::DeterministicTestDouble,
            provenance: crate::ShepherdProvenance::LiveExecution,
            retained: false,
            backend_identity_sha256: Some("1".repeat(64)),
            model_identity_sha256: "2".repeat(64),
            model_artifact_sha256: None,
            runner_program_sha256: "3".repeat(64),
            runner_launch_sha256: "4".repeat(64),
            runner_nonce_sha256: None,
            elapsed_millis: 1,
            response_sha256: "5".repeat(64),
            response,
        })
        .map_err(|error| ExecutorError {
            class: FailureClass::Fatal,
            message: error.to_string(),
        })
    }
}

#[tokio::test]
async fn shepherd_conversation_invokes_configured_provider_and_preserves_canonical_wss_ingress() {
    let recorder = RuntimeRecorder::new(32);
    let dispatches = Arc::new(AtomicUsize::new(0));
    let completions = Arc::new(AtomicUsize::new(0));
    let barrier_started = Arc::new(Notify::new());
    let barrier_release = Arc::new(Semaphore::new(0));
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
                barrier_started: barrier_started.clone(),
                barrier_release: barrier_release.clone(),
            }),
        )
        .unwrap(),
    );
    let operation = OperationalFactory::new(adapter, vec![]);
    let shepherd_adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Shepherd,
            AdapterPolicy {
                capacity: 4,
                max_in_flight: 2,
                shutdown_grace_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            Arc::new(ShepherdConversationExecutor {
                dispatches: dispatches.clone(),
                completions: completions.clone(),
                barrier_started: barrier_started.clone(),
                barrier_release: barrier_release.clone(),
            }),
        )
        .unwrap(),
    );
    let shepherd_operation = OperationalFactory::new(shepherd_adapter, vec![]);
    let ingress = CanonicalIngress::new(
        4,
        recorder.clone(),
        BTreeMap::from([
            ("agent_runtime".to_owned(), operation.clone()),
            ("shepherd".to_owned(), shepherd_operation.clone()),
        ]),
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
    let mut population = AgentPopulationFeed::resident_shepherd();
    for index in 0..=100 {
        let agent_id = format!("agent-{index:04}");
        recorder.set_component_state(ComponentId::new(&agent_id), RunningState::Running);
        assert!(recorder.record_agent_admission(
            &agent_id,
            admitted_at,
            admitted_at + 30_000,
            "1111111111111111111111111111111111111111",
        ));
        population.sample.push(AgentSample {
            id: agent_id.clone(),
            name: format!("{agent_id}.runtime"),
            label: format!("Agent {index:04}"),
            role: "conversation agent".to_owned(),
            provider: None,
            model: None,
            last_snapshot_at_unix_millis: None,
            last_archive_at_unix_millis: None,
            snapshot_sequence: None,
            pending_archive_count: 0,
            snapshot_state: crate::AgentSnapshotState::NeverSnapshotted,
            archive_state: crate::AgentArchiveState::Disabled,
            inference_readiness: crate::InferenceReadinessState::Ready,
            state: "unknown".to_owned(),
            detail: "Awaiting Runtime projection".to_owned(),
            health: "unknown".to_owned(),
            availability: "unknown".to_owned(),
            activity: None,
            capabilities: vec!["conversation".to_owned()],
            location: Some("local_runtime".to_owned()),
            communication_eligible: false,
            observed_at_unix_millis: 0,
            freshness_deadline_unix_millis: 0,
            source_revision: "unobserved".to_owned(),
            provenance: "runtime_component_state".to_owned(),
            orientation: None,
        });
    }
    let visible_agent_ids = population
        .sample
        .iter()
        .map(|agent| agent.id.clone())
        .collect::<BTreeSet<_>>();
    population = population.with_public_policy(AgentRosterPolicy {
        policy_subject: "conversation-test".to_owned(),
        visible_agent_ids,
        reveal_capabilities: false,
        reveal_location: false,
    });
    let service = Arc::new(
        ControlService::new_with_observatory_config_and_agents(
            "conversation-runtime",
            recorder.clone(),
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            8,
            ["https://observatory.example.test".to_owned()],
            population,
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
    registry.register(shepherd_operation);
    registry.register(ingress);
    let kernel = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    let server_service = service.clone();
    let server = tokio::spawn(async move {
        serve_control_listener(
            server_service,
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
    assert_eq!(feed["schema"], PREVIOUS_OBSERVATORY_FEED_SCHEMA);

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

    let same_token_reauth = serde_json::json!({
        "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
        "conversation_id": "conversation-same-token-reauth",
        "turn_id": "turn-same-token-reauth",
        "recipient_id": "shepherd",
        "correlation_id": "21212121212121212121212121212121",
        "message": "delay revoke"
    });
    socket
        .send(Message::Text(same_token_reauth.to_string().into()))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
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
    assert!(
        tokio::time::timeout(
            Duration::from_millis(80),
            next_conversation_result_for_turn(&mut socket, "turn-same-token-reauth")
        )
        .await
        .is_err(),
        "the prior authentication generation received an in-flight result"
    );
    socket
        .send(Message::Text(same_token_reauth.to_string().into()))
        .await
        .unwrap();
    let completed =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(completed["status"], "delivered", "{completed}");

    let bounded_duplicate = serde_json::json!({
        "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
        "conversation_id": "conversation-bounded-duplicate",
        "turn_id": "turn-bounded-duplicate",
        "recipient_id": "shepherd",
        "correlation_id": "23232323232323232323232323232323",
        "message": "delay ordered"
    });
    socket
        .send(Message::Text(bounded_duplicate.to_string().into()))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    for _ in 0..64 {
        socket
            .send(Message::Text(bounded_duplicate.to_string().into()))
            .await
            .unwrap();
    }
    for _ in 0..64 {
        let duplicate =
            next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
        assert_eq!(duplicate["error"], "conversation_in_flight", "{duplicate}");
    }
    let delivered = next_conversation_result_for_turn(&mut socket, "turn-bounded-duplicate").await;
    assert_eq!(delivered["status"], "delivered", "{delivered}");
    assert!(
        tokio::time::timeout(
            Duration::from_millis(80),
            next_conversation_result_for_turn(&mut socket, "turn-bounded-duplicate")
        )
        .await
        .is_err(),
        "duplicate terminal conversation frame was emitted"
    );

    let rotate_back = serde_json::json!({
        "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
        "conversation_id": "conversation-rotate-back",
        "turn_id": "turn-rotate-back",
        "recipient_id": "shepherd",
        "correlation_id": "24242424242424242424242424242424",
        "message": "delay ordered"
    });
    socket
        .send(Message::Text(rotate_back.to_string().into()))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    service.set_observatory_bearer_token(ROTATED_TOKEN).unwrap();
    let revoked = next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(revoked["error"], "credential_revoked", "{revoked}");
    service.set_observatory_bearer_token(TOKEN).unwrap();
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
    assert!(
        tokio::time::timeout(
            Duration::from_millis(100),
            next_conversation_result_for_turn(&mut socket, "turn-rotate-back")
        )
        .await
        .is_err(),
        "restoring old token bytes revived an earlier authentication generation"
    );
    socket
        .send(Message::Text(rotate_back.to_string().into()))
        .await
        .unwrap();
    let completed =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(completed["status"], "delivered", "{completed}");

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
    assert_eq!(delivered["reply"], "Beacon generated: Hello");
    assert_eq!(
        delivered["schema"],
        OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA
    );
    assert_eq!(delivered["recipient_id"], "shepherd");
    assert_eq!(delivered["conversation_id"], "conversation-shepherd");
    assert_eq!(delivered["turn_id"], "turn-positive");
    assert_eq!(
        delivered["correlation_id"],
        "0123456789abcdef0123456789abcdef"
    );
    assert!(!delivered.to_string().contains("adapter_secret"));
    assert_eq!(dispatches.load(Ordering::SeqCst), 4);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-page-two",
                "turn_id": "turn-page-two",
                "recipient_id": "agent-0100",
                "correlation_id": "10101010101010101010101010101010",
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
    let delivered =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(delivered["status"], "delivered", "{delivered}");
    assert_eq!(delivered["recipient_id"], "agent-0100");
    assert_eq!(dispatches.load(Ordering::SeqCst), 5);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-revoked-in-flight",
                "turn_id": "turn-revoked-in-flight",
                "recipient_id": "shepherd",
                "correlation_id": "20202020202020202020202020202020",
                "message": "delay revoke"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    service.set_observatory_bearer_token(ROTATED_TOKEN).unwrap();
    let revoked = next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(revoked["status"], "rejected");
    assert_eq!(revoked["error"], "credential_revoked");

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": ROTATED_TOKEN,
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
    let duplicate =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(duplicate["status"], "delivered");
    assert_eq!(dispatches.load(Ordering::SeqCst), 6);

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
    assert_eq!(dispatches.load(Ordering::SeqCst), 6);

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
    assert_eq!(budget_second["status"], "delivered");

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
                "conversation_id": "conversation-forged-output",
                "turn_id": "turn-forged-output",
                "recipient_id": "agent-0100",
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
    let _ = next_frame_with_schema(&mut socket, PREVIOUS_OBSERVATORY_FEED_SCHEMA).await;
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": ROTATED_TOKEN,
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
    let still_running =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(still_running["status"], "accepted");
    let completed_after_reconnect =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(completed_after_reconnect["status"], "delivered");
    assert_eq!(dispatches.load(Ordering::SeqCst), 11);
    assert_eq!(completions.load(Ordering::SeqCst), 11);

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
    assert_eq!(completions.load(Ordering::SeqCst), 11);

    let cleanup_race = serde_json::json!({
        "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
        "conversation_id": "conversation-cleanup-race",
        "turn_id": "turn-cleanup-race",
        "recipient_id": "shepherd",
        "correlation_id": "34343434343434343434343434343434",
        "message": "barrier cleanup"
    });
    let cleanup_hook =
        ConversationAttachmentTestHook::new("conversation-cleanup-race", "turn-cleanup-race");
    service.install_conversation_attachment_test_hook(cleanup_hook.clone());
    let mut cleanup_release = CleanupRaceReleaseGuard {
        hook: cleanup_hook.clone(),
        execution: barrier_release.clone(),
        execution_released: false,
        completed: false,
    };
    socket
        .send(Message::Text(cleanup_race.to_string().into()))
        .await
        .unwrap();
    let accepted =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    tokio::time::timeout(Duration::from_secs(1), barrier_started.notified())
        .await
        .expect("old-generation execution did not reach the completion barrier");
    let scheduling_pressure = (0..64)
        .map(|_| {
            tokio::spawn(async {
                for _ in 0..64 {
                    tokio::task::yield_now().await;
                }
            })
        })
        .collect::<Vec<_>>();

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_AUTH_SCHEMA,
                "bearer_token": ROTATED_TOKEN,
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    // Queue the attachment behind re-authentication without spending the
    // conversation execution window on a client-side authentication round
    // trip. The server still processes these frames in order, so the proof
    // retains the generation transition while deterministically attaching to
    // the barrier-held turn before its bounded execution deadline.
    socket
        .send(Message::Text(cleanup_race.to_string().into()))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(10), cleanup_hook.wait_for_duplicate())
        .await
        .expect("server did not observe the cleanup duplicate");
    cleanup_hook.permit_duplicate();
    tokio::time::timeout(Duration::from_secs(10), cleanup_hook.wait_for_attachment())
        .await
        .expect("server did not install the current-generation attachment");
    let authenticated =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA).await;
    assert_eq!(authenticated["status"], "authenticated");
    let attached =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(attached["status"], "accepted", "{attached}");
    assert_eq!(attached["error"], "conversation_in_flight", "{attached}");

    cleanup_release.release_execution();
    let delivered = next_conversation_result_for_turn(&mut socket, "turn-cleanup-race").await;
    assert_eq!(delivered["status"], "delivered", "{delivered}");
    assert!(
        tokio::time::timeout(
            Duration::from_millis(80),
            next_conversation_result_for_turn(&mut socket, "turn-cleanup-race")
        )
        .await
        .is_err(),
        "stale completion removed or duplicated the current-generation attachment"
    );
    assert_eq!(completions.load(Ordering::SeqCst), 12);
    for task in scheduling_pressure {
        task.await.unwrap();
    }
    assert_eq!(cleanup_hook.fail_safe_permits(), (0, 0));
    cleanup_release.complete();
    drop(cleanup_release);

    let dispatches_before_turnover = dispatches.load(Ordering::SeqCst);
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
    assert_eq!(capacity["status"], "accepted", "{capacity}");
    let delivered = next_conversation_result_for_turn(&mut socket, "turn-over-capacity").await;
    assert_eq!(delivered["status"], "delivered", "{delivered}");
    assert_eq!(
        dispatches.load(Ordering::SeqCst),
        dispatches_before_turnover + 1,
        "a new turn should execute after the oldest terminal record is evicted"
    );

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
    let duplicate =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(duplicate["status"], "delivered", "{duplicate}");
    assert_eq!(
        dispatches.load(Ordering::SeqCst),
        dispatches_before_turnover + 1,
        "a retained terminal duplicate must not execute again"
    );

    for index in 0..8 {
        let turn_id = format!("turn-in-flight-{index}");
        socket
            .send(Message::Text(
                serde_json::json!({
                    "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                    "conversation_id": "conversation-in-flight-capacity",
                    "turn_id": turn_id,
                    "recipient_id": "shepherd",
                    "correlation_id": format!("{:032x}", 100 + index),
                    "message": "barrier cleanup"
                })
                .to_string()
                .into(),
            ))
            .await
            .unwrap();
        let accepted =
            next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
        assert_eq!(accepted["status"], "accepted", "{accepted}");
        if index == 0 {
            tokio::time::timeout(Duration::from_secs(1), barrier_started.notified())
                .await
                .expect("capacity fixture did not enter in-flight execution");
        }
    }
    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-in-flight-capacity",
                "turn_id": "turn-in-flight-over-capacity",
                "recipient_id": "shepherd",
                "correlation_id": "000000000000000000000000000000ff",
                "message": "Hello"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let capacity =
        next_frame_with_schema(&mut socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
    assert_eq!(capacity["status"], "failed", "{capacity}");
    assert_eq!(
        capacity["error"], "conversation_capacity_exhausted",
        "active turns must never be evicted to admit new work"
    );
    barrier_release.add_permits(8);

    socket
        .send(Message::Text(
            serde_json::json!({
                "schema": OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA,
                "conversation_id": "conversation-provider-failure",
                "turn_id": "turn-provider-failure",
                "recipient_id": "shepherd",
                "correlation_id": "99999999999999999999999999999999",
                "message": "provider failure"
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let accepted = next_conversation_result_for_turn(&mut socket, "turn-provider-failure").await;
    assert_eq!(accepted["status"], "accepted", "{accepted}");
    let failed = next_conversation_result_for_turn(&mut socket, "turn-provider-failure").await;
    assert_eq!(failed["status"], "failed", "{failed}");
    assert_eq!(failed["error"], "conversation_failed", "{failed}");
    assert!(failed["reply"].is_null(), "{failed}");
    assert!(!failed.to_string().contains("received your message"));

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

async fn next_conversation_result_for_turn<S>(
    socket: &mut tokio_tungstenite::WebSocketStream<S>,
    turn_id: &str,
) -> serde_json::Value
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    loop {
        let value = next_frame_with_schema(socket, OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA).await;
        if value["turn_id"] == turn_id {
            return value;
        }
    }
}
