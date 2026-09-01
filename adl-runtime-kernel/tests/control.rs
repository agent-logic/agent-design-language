use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    channel, load_control_tls, serve_control_listener, serve_control_listener_until,
    serve_control_listener_until_ready, start_config_reload_with_applier_and_shutdown,
    write_observability_event, write_payload, AdapterKind, AdapterPolicy, AuthorityMode,
    CanonicalIngress, CheckpointingControl, ClockAuthority, ComponentId, ComponentRegistry,
    ConfigApplier, ConfigParser, ConfigReloadError, ConfigReloadOptions, ContinuityHead,
    ControlAction, ControlApiPolicy, ControlAuthority, ControlCapability, ControlError,
    ControlExit, ControlObservabilityEvent, ControlOutcome, ControlService, DiskWeather,
    DomainWork, ExecutorError, Kernel, KernelExit, LifecycleControl, LiveContinuity,
    LiveKernelSnapshot, ObservabilityDegradation, ObservabilityHealth, Observation,
    OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory, ResourceState,
    RuntimeEvent, RuntimeRecorder, RuntimeTlsInitConfig, ShutdownDecision, SignedControlCommand,
    TrustedControlKey, WeatherConfig, WeatherHealthReport, WeatherSample, DOMAIN_WORK_SCHEMA,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Notify,
};
use tokio_rustls::{
    rustls::{pki_types::ServerName, ClientConfig},
    TlsConnector,
};

const TEST_BIND_HOST: &str = "127.0.0.1";

#[path = "../../adl-runtime/tests/support/tls.rs"]
mod tls_support;
use tls_support::TestPki;
#[path = "support/runtime_init.rs"]
mod runtime_init_support;

#[test]
fn polis_identity_reload_atomically_updates_every_parameter() {
    let evidence_root = std::path::Path::new("../.csdlc/evidence/551/control-tests");
    std::fs::create_dir_all(evidence_root).unwrap();
    let root = tempfile::tempdir_in(evidence_root.canonicalize().unwrap()).unwrap();
    let state = root.path().join("state");
    let config_path = runtime_init_support::write_for_state(
        root.path(),
        "127.0.0.1:20997".parse().unwrap(),
        &state,
    );
    let init = adl_runtime_kernel::RuntimeInitConfig::from_path(config_path).unwrap();
    let key = SigningKey::from_bytes(&[17; 32]);
    let service = ControlService::new(
        "instance-1",
        RuntimeRecorder::new(16),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Stop]),
        8,
    )
    .with_polis_identity(&init);

    let mut reload = init.clone();
    reload.polis.id = "another-polis".to_owned();
    reload.polis.display_name = "Renamed Polis".to_owned();
    reload.polis.public_domain = "new.example.test".to_owned();
    reload.polis.observatory_public_origin = "https://observe.new.example.test".to_owned();
    reload.api.public_base_url = "https://new.example.test".to_owned();
    reload.api.tls.server_name = "new.example.test".to_owned();
    reload.observatory.allowed_origins = vec!["https://observe.new.example.test".to_owned()];
    reload.observatory.additional_allowed_origins.clear();
    service.apply_runtime_init_reload(&reload).unwrap();

    let observed = service.observatory_feed().polis_identity;
    assert_eq!(observed.polis_id, "another-polis");
    assert_eq!(observed.display_name, "Renamed Polis");
    assert_eq!(observed.public_domain, "new.example.test");
    assert_eq!(observed.runtime_api_base, "https://new.example.test");
    assert_eq!(
        observed.observatory_public_origin,
        "https://observe.new.example.test"
    );
    assert_eq!(
        service.observatory_feed().control.public_base_url,
        "https://new.example.test"
    );
    assert!(service
        .observatory_origin_policy()
        .contains("https://observe.new.example.test"));
    assert!(!service
        .observatory_origin_policy()
        .contains("https://observatory.example.test"));

    let mut invalid = reload.clone();
    invalid.polis.display_name = "Must Not Apply".to_owned();
    invalid.observatory.allowed_origins = vec!["*".to_owned()];
    assert!(service.apply_runtime_init_reload(&invalid).is_err());
    assert_eq!(service.observatory_feed().polis_identity, observed);
    assert_eq!(
        service.observatory_feed().control.public_base_url,
        "https://new.example.test"
    );
    assert!(service
        .observatory_origin_policy()
        .contains("https://observe.new.example.test"));
    assert!(!service.observatory_origin_policy().contains("*"));

    let mut inconsistent = reload;
    inconsistent.polis.display_name = "Must Still Not Apply".to_owned();
    inconsistent.observatory.allowed_origins = vec!["https://different.example.test".to_owned()];
    assert!(service.apply_runtime_init_reload(&inconsistent).is_err());
    assert_eq!(service.observatory_feed().polis_identity, observed);
    assert!(service
        .observatory_origin_policy()
        .contains("https://observe.new.example.test"));
    assert!(!service
        .observatory_origin_policy()
        .contains("https://different.example.test"));
}

fn test_api_policy() -> ControlApiPolicy {
    ControlApiPolicy::new(
        Duration::from_secs(2),
        Duration::from_secs(5),
        Duration::from_millis(20),
        64 * 1024,
    )
    .unwrap()
}

async fn test_https() -> (axum_server::tls_rustls::RustlsConfig, TlsConnector) {
    let pki = TestPki::new("kernel control https");
    let identity = pki.server(&["localhost", TEST_BIND_HOST]);
    let server = axum_server::tls_rustls::RustlsConfig::from_pem(
        identity.certificate_pem(),
        identity.private_key_pem(),
    )
    .await
    .unwrap();
    let client = TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(pki.roots())
            .with_no_client_auth(),
    ));
    (server, client)
}

async fn https_request(
    client: &TlsConnector,
    address: std::net::SocketAddr,
    request: &[u8],
) -> String {
    let stream = tokio::net::TcpStream::connect(address).await.unwrap();
    let mut stream = client
        .connect(ServerName::try_from("localhost").unwrap(), stream)
        .await
        .unwrap();
    stream.write_all(request).await.unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await.unwrap();
    String::from_utf8(response).unwrap()
}

struct FakeLifecycle {
    calls: Arc<AtomicUsize>,
}

struct BlockingLifecycle {
    calls: Arc<AtomicUsize>,
    started: Arc<Notify>,
    release: Arc<Notify>,
}

struct EchoExecutor;

struct DelayedExecutor {
    started: Arc<Notify>,
    release: Arc<Notify>,
}

#[async_trait]
impl OperationExecutor for EchoExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Ok(request.payload.clone())
    }
}

#[async_trait]
impl OperationExecutor for DelayedExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.started.notify_one();
        self.release.notified().await;
        Ok(request.payload.clone())
    }
}

fn test_ingress(
    capacity: usize,
    recorder: RuntimeRecorder,
) -> (CanonicalIngress, OperationalFactory) {
    test_ingress_with(capacity, recorder, Arc::new(EchoExecutor))
}

fn test_ingress_with(
    capacity: usize,
    recorder: RuntimeRecorder,
    executor: Arc<dyn OperationExecutor>,
) -> (CanonicalIngress, OperationalFactory) {
    let adapter = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            AdapterPolicy {
                capacity,
                max_in_flight: capacity,
                shutdown_grace_millis: 1_000,
                max_attempts: 1,
                idempotency_entries: 16,
                authority: AuthorityMode::Internal,
            },
            executor,
        )
        .unwrap(),
    );
    let factory = OperationalFactory::new(adapter, vec![]);
    let ingress = CanonicalIngress::new(
        capacity,
        recorder,
        BTreeMap::from([("parity-a".to_owned(), factory.clone())]),
    );
    (ingress, factory)
}

#[async_trait]
impl LifecycleControl for BlockingLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.started.notify_one();
        self.release.notified().await;
        Ok(KernelExit::Clean)
    }
}

#[async_trait]
impl LifecycleControl for FakeLifecycle {
    async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(KernelExit::Clean)
    }
}

fn authority(
    key: &SigningKey,
    capabilities: impl IntoIterator<Item = ControlCapability>,
) -> ControlAuthority {
    ControlAuthority::new(BTreeMap::from([(
        "operator-key".to_owned(),
        TrustedControlKey {
            principal: "operator".to_owned(),
            verifying_key: key.verifying_key(),
            capabilities: capabilities.into_iter().collect::<BTreeSet<_>>(),
        },
    )]))
}

fn signed(key: &SigningKey, id: &str, action: ControlAction) -> SignedControlCommand {
    let correlation_id = blake3::hash(id.as_bytes()).to_hex()[..32].to_owned();
    SignedControlCommand::sign(
        id,
        correlation_id,
        "instance-1",
        "operator",
        action,
        "operator-key",
        key,
    )
    .unwrap()
}

#[tokio::test]
async fn signed_restart_is_bound_to_the_current_runtime_incarnation() {
    let key = SigningKey::from_bytes(&[91; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(16),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Stop]),
        8,
    ));
    let incarnation = service.observatory_feed().runtime_incarnation_id;
    let stale = service
        .execute(signed(
            &key,
            "restart-stale",
            ControlAction::Restart {
                expected_incarnation_id: uuid::Uuid::new_v4().to_string(),
                grace_millis: 50,
            },
        ))
        .await;
    assert_eq!(stale, Err(ControlError::StaleRuntimeInstance));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let response = service
        .execute(signed(
            &key,
            "restart-current",
            ControlAction::Restart {
                expected_incarnation_id: incarnation,
                grace_millis: 50,
            },
        ))
        .await
        .unwrap();
    assert_eq!(response.outcome, ControlOutcome::Restart { accepted: true });
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn signed_ingress_checkpoints_replays_and_is_observatory_visible() {
    let root = tempfile::tempdir().unwrap();
    let key = SigningKey::from_bytes(&[37; 32]);
    let recorder = RuntimeRecorder::new(32);
    let (ingress, operation) = test_ingress(2, recorder.clone());
    let mut registry = ComponentRegistry::new();
    registry.register(operation);
    registry.register(ingress.clone());
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    let service = Arc::new(
        ControlService::new(
            "instance-1",
            recorder.clone(),
            handle.control(),
            authority(&key, [ControlCapability::Execute]),
            8,
        )
        .with_canonical_ingress(ingress.clone()),
    );
    let work = DomainWork {
        schema: DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "work-1".to_owned(),
        kind: "parity-a".to_owned(),
        payload: b"deterministic".to_vec(),
    };
    let first = service
        .execute(signed(
            &key,
            "submit-1",
            ControlAction::Submit { work: work.clone() },
        ))
        .await
        .unwrap();
    let ControlOutcome::Submitted {
        work_result: first_result,
    } = first.outcome
    else {
        panic!("submit outcome")
    };
    assert_eq!(first_result.accepted_sequence, 1);
    assert_eq!(
        service.observatory_feed().ingress.completed["work-1"],
        first_result
    );
    let invalid = DomainWork {
        schema: "adl.runtime.domain_work.v999".to_owned(),
        ..work.clone()
    };
    assert_eq!(
        service
            .execute(signed(
                &key,
                "submit-invalid",
                ControlAction::Submit { work: invalid },
            ))
            .await
            .unwrap_err(),
        ControlError::InvalidBounds
    );
    let oversized = DomainWork {
        work_id: "oversized-work".to_owned(),
        payload: vec![0; 1_048_577],
        ..work.clone()
    };
    assert_eq!(
        service
            .execute(signed(
                &key,
                "submit-oversized",
                ControlAction::Submit { work: oversized },
            ))
            .await
            .unwrap_err(),
        ControlError::InvalidBounds
    );

    let identity = LiveKernelSnapshot::new(
        blake3::hash(b"topology").to_hex().to_string(),
        blake3::hash(b"config").to_hex().to_string(),
        BTreeMap::new(),
    );
    let mut continuity = LiveContinuity::new(root.path(), "live", &[41; 32], identity.clone(), 0)
        .with_canonical_ingress(ingress);
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );

    let restored_recorder = RuntimeRecorder::new(32);
    let (restored_ingress, restored_operation) = test_ingress(2, restored_recorder.clone());
    let mut restored = LiveContinuity::new(root.path(), "live", &[41; 32], identity, 1)
        .with_canonical_ingress(restored_ingress.clone());
    assert_eq!(
        restored.restore_latest(&restored_recorder).await.unwrap(),
        Some(1)
    );
    let mut registry = ComponentRegistry::new();
    registry.register(restored_operation);
    registry.register(restored_ingress.clone());
    let handle = Kernel::new(registry.validate().unwrap(), restored_recorder.clone())
        .start()
        .await
        .unwrap();
    let service = Arc::new(
        ControlService::new(
            "instance-1",
            restored_recorder,
            handle.control(),
            authority(&key, [ControlCapability::Execute]),
            8,
        )
        .with_canonical_ingress(restored_ingress),
    );
    let replay = service
        .execute(signed(
            &key,
            "submit-2",
            ControlAction::Submit { work: work.clone() },
        ))
        .await
        .unwrap();
    let ControlOutcome::Submitted {
        work_result: replay_result,
    } = replay.outcome
    else {
        panic!("submit outcome")
    };
    assert_eq!(replay_result, first_result);
    let next = service
        .execute(signed(
            &key,
            "submit-3",
            ControlAction::Submit {
                work: DomainWork {
                    work_id: "work-2".to_owned(),
                    ..work
                },
            },
        ))
        .await
        .unwrap();
    let ControlOutcome::Submitted {
        work_result: next_result,
    } = next.outcome
    else {
        panic!("submit outcome")
    };
    assert_eq!(next_result.accepted_sequence, 2);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[tokio::test]
async fn terminal_serialization_drains_accepted_work_into_checkpoint() {
    let root = tempfile::tempdir().unwrap();
    let key = SigningKey::from_bytes(&[38; 32]);
    let recorder = RuntimeRecorder::new(32);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let (ingress, operation) = test_ingress_with(
        2,
        recorder.clone(),
        Arc::new(DelayedExecutor {
            started: started.clone(),
            release: release.clone(),
        }),
    );
    let mut registry = ComponentRegistry::new();
    registry.register(operation);
    registry.register(ingress.clone());
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    let service = Arc::new(
        ControlService::new(
            "instance-1",
            recorder,
            handle.control(),
            authority(&key, [ControlCapability::Read, ControlCapability::Execute]),
            8,
        )
        .with_canonical_ingress(ingress.clone()),
    );
    let accepted = {
        let service = service.clone();
        let submit_key = key.clone();
        tokio::spawn(async move {
            service
                .execute(signed(
                    &submit_key,
                    "accepted-before-terminal",
                    ControlAction::Submit {
                        work: DomainWork {
                            schema: DOMAIN_WORK_SCHEMA.to_owned(),
                            work_id: "accepted-before-terminal".to_owned(),
                            kind: "parity-a".to_owned(),
                            payload: b"delayed-terminal-work".to_vec(),
                        },
                    },
                ))
                .await
        })
    };
    started.notified().await;
    let identity = LiveKernelSnapshot::new(
        blake3::hash(b"terminal-topology").to_hex().to_string(),
        blake3::hash(b"terminal-config").to_hex().to_string(),
        BTreeMap::new(),
    );
    let mut continuity = LiveContinuity::new(root.path(), "live", &[42; 32], identity, 0)
        .with_canonical_ingress(ingress);
    let terminal = service.serialize_terminal_checkpoint(&mut continuity, Duration::from_secs(1));
    tokio::pin!(terminal);
    assert!(
        tokio::time::timeout(Duration::from_millis(10), terminal.as_mut())
            .await
            .is_err()
    );
    assert!(!root.path().join("generation-1").exists());
    assert_eq!(
        service
            .execute(signed(&key, "late-terminal-read", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    release.notify_one();
    let response = accepted.await.unwrap().unwrap();
    let ControlOutcome::Submitted { work_result } = response.outcome else {
        panic!("submit outcome")
    };
    assert_eq!(work_result.accepted_sequence, 1);
    terminal.await.unwrap();
    let checkpoint: serde_json::Value = serde_json::from_slice(
        &tokio::fs::read(root.path().join("generation-1/0000-live_kernel.bin"))
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint["ingress"]["accepted_through"], 1);
    assert!(checkpoint["ingress"]["completed"]["accepted-before-terminal"].is_object());
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[tokio::test]
async fn snapshot_is_revisioned_and_contains_complete_health_state() {
    let recorder = RuntimeRecorder::new(8);
    recorder.set_topology_generation(9);
    recorder.set_component_state(
        ComponentId::new("scheduler"),
        adl_runtime_kernel::RunningState::Degraded,
    );
    recorder.set_restart_count(ComponentId::new("scheduler"), 2);
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "sntp".to_owned(),
        unix_millis: 42,
    });
    let (sender, mut receiver) = channel(1, adl_runtime_kernel::ChannelFullPolicy::Reject);
    sender.send("first").await.unwrap();
    assert!(sender.send("second").await.is_err());
    recorder.set_queue_health("control", &sender.metrics());
    recorder.set_continuity_head(ContinuityHead {
        generation: 4,
        accepted_through: 77,
        topology_hash: "topology".to_owned(),
        config_hash: "config".to_owned(),
        integrity: "manifest-hash".to_owned(),
    });

    let snapshot = recorder.snapshot();
    assert!(snapshot.revision >= 6);
    assert_eq!(snapshot.topology_generation, 9);
    assert_eq!(
        snapshot.components[&ComponentId::new("scheduler")],
        adl_runtime_kernel::RunningState::Degraded
    );
    assert_eq!(snapshot.restart_counts[&ComponentId::new("scheduler")], 2);
    assert_eq!(snapshot.queues["control"].capacity, 1);
    assert_eq!(snapshot.queues["control"].generation, 2);
    assert_eq!(snapshot.queues["control"].depth, 1);
    assert_eq!(snapshot.queues["control"].high_water, 1);
    assert_eq!(snapshot.queues["control"].rejected, 1);
    assert_eq!(snapshot.continuity_head.unwrap().accepted_through, 77);
    assert_eq!(receiver.recv().await, Some("first"));

    let (sender, mut waiting_receiver) = channel(1, adl_runtime_kernel::ChannelFullPolicy::Block);
    let metrics = sender.metrics();
    let waiter = tokio::spawn(async move { waiting_receiver.recv().await });
    tokio::task::yield_now().await;
    sender.send("direct").await.unwrap();
    assert_eq!(waiter.await.unwrap(), Some("direct"));
    assert_eq!(metrics.depth(), 0);
}

#[tokio::test]
async fn forged_and_unauthorized_commands_never_reach_lifecycle_authority() {
    let key = SigningKey::from_bytes(&[3; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let shutdown = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 5 });
    assert_eq!(
        service.execute(shutdown).await.unwrap_err(),
        ControlError::Unauthorized
    );

    let mut forged = signed(&key, "read-1", ControlAction::Snapshot);
    forged.correlation_id = "correlation-forged".to_owned();
    assert_eq!(
        service.execute(forged).await.unwrap_err(),
        ControlError::Authentication
    );

    let stale_service = Arc::new(ControlService::new(
        "instance-2",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    assert_eq!(
        stale_service
            .execute(signed(&key, "read-2", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::StaleRuntimeInstance
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let submit = signed(
        &key,
        "submit-without-capability",
        ControlAction::Submit {
            work: DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: "unauthorized-work".to_owned(),
                kind: "parity-a".to_owned(),
                payload: vec![1],
            },
        },
    );
    assert_eq!(
        service.execute(submit).await.unwrap_err(),
        ControlError::Unauthorized
    );
}

#[tokio::test]
async fn pressure_admission_gate_refuses_new_commands_until_reopened() {
    let key = SigningKey::from_bytes(&[31; 32]);
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));

    service
        .close_admission_and_drain(Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(
        service
            .execute(signed(&key, "read-paused", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    assert!(service.reopen_admission_if_no_terminal());
    service
        .execute(signed(&key, "read-open", ControlAction::Snapshot))
        .await
        .unwrap();
}

#[tokio::test]
async fn pressure_cannot_reopen_after_signed_shutdown_is_enqueued() {
    let key = SigningKey::from_bytes(&[32; 32]);
    let (lifecycle, mut requests) = CheckpointingControl::channel(1);
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        lifecycle,
        authority(&key, [ControlCapability::Read, ControlCapability::Stop]),
        4,
    ));
    let shutdown = {
        let service = service.clone();
        let shutdown_key = key.clone();
        tokio::spawn(async move {
            service
                .execute(signed(
                    &shutdown_key,
                    "signed-terminal-race",
                    ControlAction::Shutdown { grace_millis: 50 },
                ))
                .await
        })
    };
    let request = tokio::time::timeout(Duration::from_secs(1), requests.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(!service.reopen_admission_if_no_terminal());
    assert_eq!(
        service
            .execute(signed(&key, "read-terminal-race", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    request.respond(Err(()));
    assert_eq!(
        shutdown.await.unwrap().unwrap().outcome,
        ControlOutcome::Shutdown {
            exit: ControlExit::Failed
        }
    );
}

#[tokio::test]
async fn duplicate_shutdown_executes_once_and_conflicting_reuse_fails() {
    let key = SigningKey::from_bytes(&[4; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let command = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 5 });
    let first = service.execute(command.clone()).await.unwrap();
    let second = service.execute(command).await.unwrap();
    assert_eq!(first, second);
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    let conflict = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 6 });
    assert_eq!(
        service.execute(conflict).await.unwrap_err(),
        ControlError::IdempotencyConflict
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn readiness_fails_closed_before_first_weather_sample() {
    let key = SigningKey::from_bytes(&[31; 32]);
    let service = ControlService::new(
        "instance-weather-missing",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    );

    let report = service.readiness_report();

    assert!(!report.ready);
    assert!(report
        .degraded_reasons
        .contains(&"weather_stale".to_owned()));
    assert!(report
        .degraded_reasons
        .contains(&"shepherd_not_admitted".to_owned()));
    assert!(report.weather_freshness.is_none());
}

#[tokio::test]
async fn idempotency_refresh_preserves_the_recent_completed_response() {
    let key = SigningKey::from_bytes(&[10; 32]);
    let recorder = RuntimeRecorder::new(8);
    let service = Arc::new(ControlService::new(
        "instance-1",
        recorder.clone(),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        2,
    ));
    let first_command = signed(&key, "read-first", ControlAction::Snapshot);
    let first = service.execute(first_command.clone()).await.unwrap();
    service
        .execute(signed(&key, "read-second", ControlAction::Snapshot))
        .await
        .unwrap();
    assert_eq!(service.execute(first_command.clone()).await.unwrap(), first);
    service
        .execute(signed(&key, "read-third", ControlAction::Snapshot))
        .await
        .unwrap();
    recorder.emit(None, RuntimeEvent::KernelStarting);
    assert_eq!(service.execute(first_command).await.unwrap(), first);
}

#[tokio::test]
async fn cancelled_client_does_not_cancel_execution_or_exceed_idempotency_bound() {
    let key = SigningKey::from_bytes(&[6; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: calls.clone(),
            started: started.clone(),
            release: release.clone(),
        },
        authority(&key, [ControlCapability::Read, ControlCapability::Stop]),
        1,
    ));
    let command = signed(
        &key,
        "stop-cancelled",
        ControlAction::Shutdown { grace_millis: 5 },
    );
    let request = {
        let service = service.clone();
        let command = command.clone();
        tokio::spawn(async move { service.execute(command).await })
    };
    started.notified().await;
    assert_eq!(
        service
            .execute(signed(&key, "read-closed", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    request.abort();
    assert_eq!(
        service
            .execute(signed(&key, "read-capacity", ControlAction::Snapshot))
            .await
            .unwrap_err(),
        ControlError::AdmissionClosed
    );
    release.notify_one();
    let response = loop {
        match service.execute(command.clone()).await {
            Ok(response) => break response,
            Err(ControlError::InFlight) => tokio::task::yield_now().await,
            other => panic!("unexpected retry result: {other:?}"),
        }
    };
    assert_eq!(
        response.outcome,
        ControlOutcome::Shutdown {
            exit: ControlExit::Clean
        }
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn axum_adapter_serves_signed_control_payloads() {
    let key = SigningKey::from_bytes(&[7; 32]);
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(serve_control_listener_until_ready(
        service,
        listener,
        tls,
        test_api_policy(),
        ready_sender,
        std::future::pending(),
    ));
    assert_eq!(ready_receiver.await.unwrap(), address);
    let body = serde_json::to_vec(&signed(&key, "read-http", ControlAction::Snapshot)).unwrap();
    let request = format!(
        "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        String::from_utf8(body).unwrap()
    );
    let response = https_request(&client, address, request.as_bytes()).await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains(adl_runtime_kernel::CONTROL_RESPONSE_SCHEMA));

    let response = https_request(
        &client,
        address,
        b"POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: 1\r\nConnection: close\r\n\r\n{",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(response.contains("adl.runtime.control_error.v1"));
    server.abort();
}

#[tokio::test]
async fn control_route_rejects_oversized_request_body_before_command_parse() {
    let key = SigningKey::from_bytes(&[17; 32]);
    let calls = Arc::new(AtomicUsize::new(0));
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: calls.clone(),
        },
        authority(&key, [ControlCapability::Read]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(serve_control_listener_until_ready(
        service,
        listener,
        tls,
        test_api_policy(),
        ready_sender,
        std::future::pending(),
    ));
    assert_eq!(ready_receiver.await.unwrap(), address);

    let body = vec![b' '; adl_runtime_kernel::CONTROL_MAX_BODY_BYTES + 1];
    let mut request = format!(
        "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len(),
    )
    .into_bytes();
    request.extend_from_slice(&body);
    let response = https_request(&client, address, &request).await;

    assert!(response.starts_with("HTTP/1.1 413 Payload Too Large"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    server.abort();
}

#[tokio::test]
async fn observatory_https_reads_are_public_and_report_weather_freshness() {
    let key = SigningKey::from_bytes(&[12; 32]);
    let recorder = RuntimeRecorder::new(8);
    recorder.set_topology_generation(11);
    recorder.set_component_state(
        ComponentId::new("runtime_api"),
        adl_runtime_kernel::RunningState::Running,
    );
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "sntp".to_owned(),
        unix_millis: 1_789_000_000,
    });
    recorder.set_continuity_head(ContinuityHead {
        generation: 3,
        accepted_through: 99,
        topology_hash: "topology-hash".to_owned(),
        config_hash: "config-hash".to_owned(),
        integrity: "snapshot-hash".to_owned(),
    });
    recorder.promote_observability();
    let admission_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    assert!(recorder.record_agent_admission(
        "shepherd",
        admission_now,
        admission_now.saturating_add(60_000),
        "0123456789abcdef0123456789abcdef01234567",
    ));
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-1",
        recorder,
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://localhost:8765".to_owned()],
    ));
    let weather_config = WeatherConfig {
        disk_stop_free_bytes: 256,
        disk_warning_free_bytes: 512,
        disk_recover_free_bytes: 1024,
        ..WeatherConfig::default()
    };
    let weather = WeatherHealthReport::from_sample(
        &weather_config,
        WeatherSample {
            platform: "test".to_owned(),
            cpu_basis_points: Observation {
                value: Some(250),
                source: "fixture".to_owned(),
            },
            per_core_basis_points: Observation {
                value: Some(vec![250]),
                source: "fixture".to_owned(),
            },
            memory_total_bytes: Observation {
                value: Some(1024),
                source: "fixture".to_owned(),
            },
            memory_available_bytes: Observation {
                value: Some(768),
                source: "fixture".to_owned(),
            },
            disks: Observation {
                value: Some(vec![DiskWeather {
                    mount: "/".to_owned(),
                    total_bytes: 4096,
                    available_bytes: 2048,
                }]),
                source: "fixture".to_owned(),
            },
            network_received_bytes: Observation {
                value: Some(13),
                source: "fixture".to_owned(),
            },
            network_transmitted_bytes: Observation {
                value: Some(21),
                source: "fixture".to_owned(),
            },
            max_temperature_millicelsius: Observation {
                value: Some(42_000),
                source: "fixture".to_owned(),
            },
            gpus: Observation {
                value: Some(Vec::new()),
                source: "fixture".to_owned(),
            },
        },
        ResourceState::Healthy,
    );
    assert_eq!(weather.shutdown_decision, ShutdownDecision::Continue);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    service.set_weather_stale_after(Duration::from_secs(1));
    service.set_weather_report_at(weather.clone(), now);
    service
        .set_observatory_bearer_token("test-observatory-token-0000000001")
        .unwrap();

    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(
        service.clone(),
        listener,
        tls,
        test_api_policy(),
    ));
    let runtime_openapi = https_request(
        &client,
        address,
        b"GET /v1/openapi.json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(runtime_openapi.starts_with("HTTP/1.1 200 OK"));
    assert!(runtime_openapi.contains("content-type: application/json"));
    assert!(runtime_openapi.contains("\"title\": \"ADL Runtime v3 Core API\""));
    assert!(runtime_openapi.contains("\"/v1/acip/ws\""));

    let observatory_openapi = https_request(
        &client,
        address,
        b"GET /v1/observatory/openapi.json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(observatory_openapi.starts_with("HTTP/1.1 200 OK"));
    assert!(observatory_openapi.contains("content-type: application/json"));
    assert!(observatory_openapi.contains("\"title\": \"ADL Observatory API\""));
    assert!(observatory_openapi.contains("\"/v1/observatory/ws\""));

    let swagger_docs = https_request(
        &client,
        address,
        b"GET /v1/docs/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(swagger_docs.starts_with("HTTP/1.1 200 OK"));
    assert!(swagger_docs.contains("content-type: text/html"));
    assert!(swagger_docs.contains("Swagger UI"));
    let swagger_initializer = https_request(
        &client,
        address,
        b"GET /v1/docs/swagger-initializer.js HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(swagger_initializer.starts_with("HTTP/1.1 200 OK"));
    assert!(swagger_initializer.contains("javascript"));
    assert!(swagger_initializer.contains("/v1/openapi.json"));
    assert!(swagger_initializer.contains("/v1/observatory/openapi.json"));
    let observatory_swagger_docs = https_request(
        &client,
        address,
        b"GET /v1/observatory/docs/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(observatory_swagger_docs.starts_with("HTTP/1.1 200 OK"));
    assert!(observatory_swagger_docs.contains("content-type: text/html"));
    let observatory_swagger_initializer = https_request(
        &client,
        address,
        b"GET /v1/observatory/docs/swagger-initializer.js HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(observatory_swagger_initializer.starts_with("HTTP/1.1 200 OK"));
    assert!(observatory_swagger_initializer.contains("/v1/observatory/openapi.json"));
    assert!(!observatory_swagger_initializer.contains("/v1/openapi.json"));

    let health = https_request(
        &client,
        address,
        b"GET /v1/health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(health.starts_with("HTTP/1.1 200 OK"));
    assert!(health.contains(adl_runtime_kernel::RUNTIME_SNAPSHOT_SCHEMA));

    let ready = https_request(
        &client,
        address,
        b"GET /v1/ready HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(ready.starts_with("HTTP/1.1 200 OK"));
    assert!(ready.contains(adl_runtime_kernel::RUNTIME_READINESS_SCHEMA));
    assert!(ready.contains("\"ready\":true"));
    assert!(ready.contains("\"degraded_reasons\":[]"));
    assert!(ready.contains("\"stale\":false"));

    let metrics = https_request(
        &client,
        address,
        b"GET /v1/metrics HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(metrics.starts_with("HTTP/1.1 200 OK"));

    let acip_unauthorized = https_request(
        &client,
        address,
        b"GET /v1/acip/ws HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(acip_unauthorized.starts_with("HTTP/1.1 400 Bad Request"));

    let preflight = https_request(
        &client,
        address,
        b"OPTIONS /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nAccess-Control-Request-Method: GET\r\nAccess-Control-Request-Headers: authorization\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(preflight.starts_with("HTTP/1.1 204 No Content"));
    assert!(preflight.contains("access-control-allow-origin: https://localhost:8765"));
    assert!(preflight.contains("access-control-allow-methods: GET"));
    assert!(preflight.contains("access-control-allow-headers: Authorization"));
    assert!(preflight.contains("cache-control: no-store"));
    let public_response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(public_response.starts_with("HTTP/1.1 200 OK"));
    assert!(public_response.contains("cache-control: no-store"));
    assert!(public_response.contains(adl_runtime_kernel::PREVIOUS_OBSERVATORY_FEED_SCHEMA));
    assert!(!public_response.contains("\"polis_identity\""));
    let v2: serde_json::Value = serde_json::from_str(
        public_response
            .split_once("\r\n\r\n")
            .expect("v2 response body")
            .1,
    )
    .unwrap();
    let mut v2_keys = v2
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v2_keys.sort_unstable();
    assert_eq!(
        v2_keys,
        [
            "agents",
            "continuity",
            "control",
            "default_runtime_changed",
            "events",
            "health",
            "ingress",
            "proof",
            "runtime_incarnation_id",
            "runtime_instance_id",
            "runtime_process_id",
            "runtime_selection",
            "schema",
            "weather",
            "weather_freshness",
        ]
    );
    let mut v2_control_keys = v2["control"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v2_control_keys.sort_unstable();
    assert_eq!(
        v2_control_keys,
        [
            "bearer_token_required_for_read",
            "browser_mutation_authority",
            "login_required_for_mutation",
            "port",
            "public_base_url",
            "read_endpoint",
            "signed_command_endpoint",
            "signed_commands_required_for_mutation",
            "websocket_acip_binary_schema",
            "websocket_endpoint",
            "websocket_full_duplex",
        ]
    );
    let mut v2_agent_keys = v2["agents"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v2_agent_keys.sort_unstable();
    assert_eq!(
        v2_agent_keys,
        [
            "event_cursor",
            "has_more",
            "next_page_token",
            "population_complete",
            "rendered_sample_count",
            "revision",
            "sample",
            "schema",
            "scope",
            "total_count",
        ]
    );
    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory?schema=v3 HTTP/1.1\r\nHost: localhost\r\nOrigin: https://localhost:8765\r\nAuthorization: Bearer test-observatory-token-0000000001\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("cache-control: no-store"));
    assert!(!response.contains(adl_runtime_kernel::LEGACY_OBSERVATORY_FEED_SCHEMA));
    assert!(!response.contains(adl_runtime_kernel::PREVIOUS_OBSERVATORY_FEED_SCHEMA));
    assert!(response.contains("access-control-allow-origin: https://localhost:8765"));
    assert!(response.contains(adl_runtime_kernel::OBSERVATORY_FEED_SCHEMA));
    assert!(response.contains("\"polis_identity\""));
    assert!(response.contains("\"runtime_selection\":\"runtime_v3_explicit_opt_in\""));
    assert!(response.contains("\"signed_commands_required_for_mutation\":true"));
    assert!(response.contains("\"bearer_token_required_for_read\":false"));
    assert!(response.contains("\"login_required_for_mutation\":true"));
    assert!(response.contains("\"browser_mutation_authority\":true"));
    assert!(response.contains(&format!("\"port\":{}", address.port())));
    assert!(response.contains("\"event\":\"state:Running\""));
    assert!(response.contains("\"event\":\"clock_authority_updated\""));
    assert!(response.contains("\"accepted_through\":99"));
    assert!(response.contains("\"cloudwatch_route\":\"vector.runtime_v3_cloudwatch_emf\""));
    assert!(response.contains("\"runtime_v2_decommission_authorized\":false"));
    assert!(response.contains("\"total_count\":0"));
    assert!(response.contains("\"population_complete\":false"));
    assert!(!response.contains("\"id\":\"agent-0001\""));
    assert!(response.contains("\"stale\":false"));

    let legacy = https_request(
        &client,
        address,
        b"GET /v1/observatory?schema=v1 HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(legacy.starts_with("HTTP/1.1 200 OK"));
    assert!(legacy.contains(adl_runtime_kernel::LEGACY_OBSERVATORY_FEED_SCHEMA));
    assert!(!legacy.contains("\"polis_identity\""));
    assert!(!legacy.contains("\"runtime_incarnation_id\""));
    assert!(!legacy.contains("\"weather_freshness\""));
    assert!(!legacy.contains("\"websocket_endpoint\""));
    assert!(!legacy.contains("\"population_complete\""));
    let v1: serde_json::Value =
        serde_json::from_str(legacy.split_once("\r\n\r\n").expect("v1 response body").1).unwrap();
    let mut v1_keys = v1
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v1_keys.sort_unstable();
    assert_eq!(
        v1_keys,
        [
            "agents",
            "continuity",
            "control",
            "default_runtime_changed",
            "events",
            "health",
            "proof",
            "runtime_instance_id",
            "runtime_selection",
            "schema",
            "weather",
        ]
    );
    let mut v1_control_keys = v1["control"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v1_control_keys.sort_unstable();
    assert_eq!(
        v1_control_keys,
        [
            "browser_mutation_authority",
            "port",
            "read_endpoint",
            "signed_command_endpoint",
            "signed_commands_required_for_mutation",
        ]
    );
    let mut v1_agent_keys = v1["agents"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();
    v1_agent_keys.sort_unstable();
    assert_eq!(
        v1_agent_keys,
        ["rendered_sample_count", "sample", "total_count"]
    );

    let unsupported = https_request(
        &client,
        address,
        b"GET /v1/observatory?schema=v4 HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(unsupported.starts_with("HTTP/1.1 400 Bad Request"));

    service.set_weather_report_at(weather, 0);
    let stale = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer test-observatory-token-0000000001\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(stale.starts_with("HTTP/1.1 200 OK"));
    assert!(stale.contains("\"stale\":true"));

    let stale_ready = https_request(
        &client,
        address,
        b"GET /v1/ready HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(stale_ready.starts_with("HTTP/1.1 503 Service Unavailable"));
    assert!(stale_ready.contains("\"ready\":false"));
    assert!(stale_ready.contains("\"weather_stale\""));
    assert!(stale_ready.contains("\"stale\":true"));
    server.abort();
}

#[tokio::test]
async fn observatory_feed_reports_large_agent_population_as_bounded_sample() {
    let key = SigningKey::from_bytes(&[14; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config_and_agents(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://observatory.example.test".to_owned()],
        adl_runtime_kernel::AgentPopulationFeed {
            total_count: 10_000,
            rendered_sample_count: 2,
            sample: vec![
                adl_runtime_kernel::AgentSample {
                    id: "agent-00001".to_owned(),
                    label: "Runtime agent 1".to_owned(),
                    role: "runtime agent".to_owned(),
                    state: "running".to_owned(),
                    detail: "sample 1 of 10000".to_owned(),
                    health: "healthy".to_owned(),
                    availability: "available".to_owned(),
                    activity: None,
                    capabilities: Vec::new(),
                    location: None,
                    communication_eligible: true,
                    observed_at_unix_millis: 1,
                    freshness_deadline_unix_millis: u64::MAX,
                    source_revision: "test".to_owned(),
                    provenance: "test_fixture".to_owned(),
                },
                adl_runtime_kernel::AgentSample {
                    id: "agent-00002".to_owned(),
                    label: "Runtime agent 2".to_owned(),
                    role: "runtime agent".to_owned(),
                    state: "running".to_owned(),
                    detail: "sample 2 of 10000".to_owned(),
                    health: "healthy".to_owned(),
                    availability: "available".to_owned(),
                    activity: None,
                    capabilities: Vec::new(),
                    location: None,
                    communication_eligible: true,
                    observed_at_unix_millis: 1,
                    freshness_deadline_unix_millis: u64::MAX,
                    source_revision: "test".to_owned(),
                    provenance: "test_fixture".to_owned(),
                },
            ],
            ..adl_runtime_kernel::AgentPopulationFeed::empty()
        },
    ));
    let feed = service.observatory_feed();
    assert_eq!(feed.agents.total_count, 10_000);
    assert_eq!(feed.agents.rendered_sample_count, 2);
    assert_eq!(feed.agents.sample.len(), 2);
    assert_eq!(feed.agents.sample[1].id, "agent-00002");
}

#[tokio::test]
async fn observatory_cors_allows_only_configured_origins_and_reports_canonical_port() {
    let key = SigningKey::from_bytes(&[13; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        [
            "https://observatory.example.test".to_owned(),
            "http://localhost:8000".to_owned(),
        ],
    ));
    service
        .set_observatory_bearer_token("test-observatory-token-0000000002")
        .unwrap();
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(
        service,
        listener,
        tls,
        test_api_policy(),
    ));

    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nAuthorization: Bearer test-observatory-token-0000000002\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("access-control-allow-origin: https://observatory.example.test"));
    assert!(response.contains(&format!("\"port\":{}", address.port())));

    let draft_observatory_response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: http://localhost:8000\r\nAuthorization: Bearer test-observatory-token-0000000002\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(
        draft_observatory_response.starts_with("HTTP/1.1 200 OK"),
        "{draft_observatory_response}"
    );
    assert!(
        draft_observatory_response.contains("access-control-allow-origin: http://localhost:8000")
    );

    let draft_observatory_preflight = https_request(
        &client,
        address,
        b"OPTIONS /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: http://localhost:8000\r\nAccess-Control-Request-Method: GET\r\nAccess-Control-Request-Headers: authorization\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(draft_observatory_preflight.starts_with("HTTP/1.1 204 No Content"));
    assert!(
        draft_observatory_preflight.contains("access-control-allow-origin: http://localhost:8000")
    );
    assert!(draft_observatory_preflight.contains("access-control-allow-methods: GET"));
    assert!(draft_observatory_preflight.contains("cache-control: no-store"));

    let draft_health = https_request(
        &client,
        address,
        b"GET /v1/health HTTP/1.1\r\nHost: localhost\r\nOrigin: http://localhost:8000\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(draft_health.starts_with("HTTP/1.1 200 OK"));
    assert!(draft_health.contains("access-control-allow-origin: http://localhost:8000"));
    assert!(draft_health.contains("cache-control: no-store"));
    assert!(draft_health.contains(adl_runtime_kernel::RUNTIME_SNAPSHOT_SCHEMA));

    let forbidden_health = https_request(
        &client,
        address,
        b"GET /v1/health HTTP/1.1\r\nHost: localhost\r\nOrigin: https://other.example.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(forbidden_health.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!forbidden_health.contains("access-control-allow-origin"));

    let response = https_request(
        &client,
        address,
        b"GET /v1/ready HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));
    assert!(response.contains("access-control-allow-origin: https://observatory.example.test"));
    assert!(response.contains("\"weather_stale\""));

    let control_preflight = https_request(
        &client,
        address,
        b"OPTIONS /v1/control HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nAccess-Control-Request-Method: POST\r\nAccess-Control-Request-Headers: content-type\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(control_preflight.starts_with("HTTP/1.1 204 No Content"));
    assert!(
        control_preflight.contains("access-control-allow-origin: https://observatory.example.test")
    );
    assert!(control_preflight.contains("access-control-allow-methods: POST"));
    assert!(control_preflight.contains("access-control-allow-headers: Content-Type, Authorization"));
    assert!(control_preflight.contains("cache-control: no-store"));

    let body = serde_json::to_vec(&signed(
        &key,
        "browser-control-read",
        ControlAction::Snapshot,
    ))
    .unwrap();
    let control_request = format!(
        "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        String::from_utf8(body).unwrap()
    );
    let control_response = https_request(&client, address, control_request.as_bytes()).await;
    assert!(control_response.starts_with("HTTP/1.1 200 OK"));
    assert!(
        control_response.contains("access-control-allow-origin: https://observatory.example.test")
    );
    assert!(control_response.contains("cache-control: no-store"));
    assert!(control_response.contains(adl_runtime_kernel::CONTROL_RESPONSE_SCHEMA));

    let invalid_control_response = https_request(
        &client,
        address,
        b"POST /v1/control HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.example.test\r\nContent-Type: application/json\r\nContent-Length: 1\r\nConnection: close\r\n\r\n{",
    )
    .await;
    assert!(invalid_control_response.starts_with("HTTP/1.1 400 Bad Request"));
    assert!(invalid_control_response
        .contains("access-control-allow-origin: https://observatory.example.test"));
    assert!(invalid_control_response.contains("adl.runtime.control_error.v1"));

    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://other.example.test\r\nAuthorization: Bearer test-observatory-token-0000000002\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!response.contains("access-control-allow-origin"));

    let forbidden_control = https_request(
        &client,
        address,
        b"POST /v1/control HTTP/1.1\r\nHost: localhost\r\nOrigin: https://other.example.test\r\nContent-Type: application/json\r\nContent-Length: 1\r\nConnection: close\r\n\r\n{",
    )
    .await;
    assert!(forbidden_control.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!forbidden_control.contains("access-control-allow-origin"));

    server.abort();
}

#[tokio::test]
async fn observatory_origin_policy_hot_loads_new_origin_and_rejects_invalid_replacement() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("observatory-origins.txt");
    tokio::fs::write(&config_path, "https://observatory.initial.test\n")
        .await
        .unwrap();
    let key = SigningKey::from_bytes(&[15; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://observatory.initial.test".to_owned()],
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(
        Arc::clone(&service),
        listener,
        tls,
        test_api_policy(),
    ));
    let reload_parser: ConfigParser<Vec<String>> = Arc::new(|raw| {
        Ok(raw
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToOwned::to_owned)
            .collect())
    });
    let reload_service = Arc::clone(&service);
    let reload_applier: ConfigApplier<Vec<String>> = Arc::new(move |origins| {
        reload_service
            .replace_observatory_allowed_origins(origins.clone())
            .map_err(ConfigReloadError::validation)
    });
    let reload = start_config_reload_with_applier_and_shutdown(
        &config_path,
        reload_parser,
        Some(reload_applier),
        ConfigReloadOptions {
            poll_interval: Duration::from_millis(10),
            debounce: Duration::from_millis(40),
        },
        tokio_util::sync::CancellationToken::new(),
    )
    .await
    .unwrap();
    let mut reload_handle = reload.handle();

    let absent_origin = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.new.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(absent_origin.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!absent_origin.contains("access-control-allow-origin"));

    tokio::fs::write(
        &config_path,
        "https://observatory.initial.test\nhttps://observatory.new.test\n",
    )
    .await
    .unwrap();
    tokio::time::timeout(Duration::from_secs(2), reload_handle.changed())
        .await
        .unwrap()
        .unwrap();
    let hot_loaded_origin = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.new.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(hot_loaded_origin.starts_with("HTTP/1.1 200 OK"));
    assert!(hot_loaded_origin.contains("access-control-allow-origin: https://observatory.new.test"));

    tokio::fs::write(&config_path, "*\n").await.unwrap();
    tokio::time::sleep(Duration::from_millis(120)).await;
    let fail_closed_rejection = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://attacker.invalid\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(fail_closed_rejection.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!fail_closed_rejection.contains("access-control-allow-origin"));

    let retained_origin = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.new.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(retained_origin.starts_with("HTTP/1.1 200 OK"));
    assert!(retained_origin.contains("access-control-allow-origin: https://observatory.new.test"));

    tokio::fs::write(&config_path, "").await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), reload_handle.changed())
        .await
        .unwrap()
        .unwrap();
    let cleared_origin = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: https://observatory.new.test\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(cleared_origin.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!cleared_origin.contains("access-control-allow-origin"));

    let outcome = reload.shutdown().await.unwrap();
    assert_eq!(outcome.reloads_applied, 2);
    assert_eq!(outcome.invalid_updates_rejected, 1);
    server.abort();
}

#[tokio::test]
async fn observatory_cors_rejects_draft_origin_without_explicit_allowance() {
    let key = SigningKey::from_bytes(&[14; 32]);
    let service = Arc::new(ControlService::new_with_observatory_config(
        "instance-1",
        RuntimeRecorder::new(4),
        FakeLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
        },
        authority(&key, [ControlCapability::Read]),
        4,
        ["https://localhost:8765".to_owned()],
    ));
    service
        .set_observatory_bearer_token("test-observatory-token-0000000003")
        .unwrap();
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener(
        service,
        listener,
        tls,
        test_api_policy(),
    ));

    let response = https_request(
        &client,
        address,
        b"GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nOrigin: http://localhost:8000\r\nAuthorization: Bearer test-observatory-token-0000000003\r\nConnection: close\r\n\r\n",
    )
    .await;
    assert!(response.starts_with("HTTP/1.1 403 Forbidden"));
    assert!(!response.contains("access-control-allow-origin"));

    server.abort();
}

#[tokio::test]
async fn graceful_api_shutdown_drains_an_active_control_response() {
    let key = SigningKey::from_bytes(&[8; 32]);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
            started: started.clone(),
            release: release.clone(),
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener_until(
        service,
        listener,
        tls,
        test_api_policy(),
        shutdown.clone().cancelled_owned(),
    ));
    let body = serde_json::to_vec(&signed(
        &key,
        "stop-drain",
        ControlAction::Shutdown { grace_millis: 5 },
    ))
    .unwrap();
    let client = tokio::spawn(async move {
        let request = format!(
            "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            String::from_utf8(body).unwrap()
        );
        https_request(&client, address, request.as_bytes()).await
    });
    started.notified().await;
    shutdown.cancel();
    release.notify_one();
    assert!(client.await.unwrap().starts_with("HTTP/1.1 200 OK"));
    server.await.unwrap().unwrap();
}

#[tokio::test]
async fn tls_configuration_fails_closed_for_missing_and_mismatched_pem_material() {
    let temp = tempfile::tempdir().unwrap();
    let missing = RuntimeTlsInitConfig {
        certificate_chain_path: temp.path().join("missing-cert.pem"),
        private_key_path: temp.path().join("missing-key.pem"),
        trust_roots_path: temp.path().join("missing-roots.pem"),
        server_name: "localhost".to_owned(),
    };
    assert!(load_control_tls(&missing).await.is_err());

    let pki = TestPki::new("kernel control mismatch");
    let first = pki.server(&["localhost"]);
    let second = pki.wrong_san_server();
    let certificate = temp.path().join("cert.pem");
    let wrong_key = temp.path().join("wrong-key.pem");
    let trust_roots = temp.path().join("roots.pem");
    std::fs::write(&certificate, first.certificate_pem()).unwrap();
    std::fs::write(&wrong_key, second.private_key_pem()).unwrap();
    std::fs::write(&trust_roots, pki.root_pem()).unwrap();
    let mismatched = RuntimeTlsInitConfig {
        certificate_chain_path: certificate,
        private_key_path: wrong_key,
        trust_roots_path: trust_roots,
        server_name: "localhost".to_owned(),
    };
    assert!(load_control_tls(&mismatched).await.is_err());

    let write_config = |name: &str,
                        identity: &tls_support::TestIdentity,
                        certificate_pem: Vec<u8>,
                        roots: &[u8],
                        server_name: &str| {
        let certificate = temp.path().join(format!("{name}-chain.pem"));
        let private_key = temp.path().join(format!("{name}-key.pem"));
        let trust_roots = temp.path().join(format!("{name}-roots.pem"));
        std::fs::write(&certificate, certificate_pem).unwrap();
        std::fs::write(&private_key, identity.private_key_pem()).unwrap();
        std::fs::write(&trust_roots, roots).unwrap();
        RuntimeTlsInitConfig {
            certificate_chain_path: certificate,
            private_key_path: private_key,
            trust_roots_path: trust_roots,
            server_name: server_name.to_owned(),
        }
    };

    let valid = pki.server(&["localhost"]);
    assert!(load_control_tls(&write_config(
        "valid",
        &valid,
        valid.certificate_pem(),
        pki.root_pem(),
        "localhost",
    ))
    .await
    .is_ok());

    let wrong_san = pki.wrong_san_server();
    assert!(load_control_tls(&write_config(
        "wrong-san",
        &wrong_san,
        wrong_san.certificate_pem(),
        pki.root_pem(),
        "localhost",
    ))
    .await
    .is_err());

    let unknown_ca = pki.server(&["localhost"]);
    assert!(load_control_tls(&write_config(
        "unknown-ca",
        &unknown_ca,
        unknown_ca.certificate_pem(),
        pki.wrong_root_pem(),
        "localhost",
    ))
    .await
    .is_err());

    let incomplete = pki.server(&["localhost"]);
    assert!(load_control_tls(&write_config(
        "incomplete-chain",
        &incomplete,
        incomplete.leaf_only_pem(),
        pki.root_pem(),
        "localhost",
    ))
    .await
    .is_err());

    let self_signed = pki.self_signed_server();
    assert!(load_control_tls(&write_config(
        "self-signed",
        &self_signed,
        self_signed.certificate_pem(),
        pki.root_pem(),
        "localhost",
    ))
    .await
    .is_err());
}

#[tokio::test]
async fn tls_shutdown_is_bounded_with_a_stalled_active_response() {
    let key = SigningKey::from_bytes(&[18; 32]);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let service = Arc::new(ControlService::new(
        "instance-1",
        RuntimeRecorder::new(4),
        BlockingLifecycle {
            calls: Arc::new(AtomicUsize::new(0)),
            started: started.clone(),
            release,
        },
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let listener = tokio::net::TcpListener::bind((TEST_BIND_HOST, 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let (tls, client) = test_https().await;
    let server = tokio::spawn(serve_control_listener_until(
        service,
        listener,
        tls,
        test_api_policy(),
        shutdown.clone().cancelled_owned(),
    ));
    let body = serde_json::to_vec(&signed(
        &key,
        "stop-stalled",
        ControlAction::Shutdown { grace_millis: 5 },
    ))
    .unwrap();
    let client = tokio::spawn(async move {
        let request = format!(
            "POST /v1/control HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            String::from_utf8(body).unwrap()
        );
        https_request(&client, address, request.as_bytes()).await
    });
    started.notified().await;
    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(3), server)
        .await
        .expect("TLS shutdown exceeded its bound")
        .unwrap()
        .unwrap();
    client.abort();
}

#[test]
fn runtime_identity_and_shutdown_bounds_are_owned_by_standard_crates() {
    let first = adl_runtime_kernel::generate_runtime_instance_id();
    let second = adl_runtime_kernel::generate_runtime_instance_id();
    assert_eq!(first.len(), 32);
    assert_ne!(first, second);

    let key = SigningKey::from_bytes(&[9; 32]);
    let result = SignedControlCommand::sign(
        "stop-too-long",
        "0123456789abcdef0123456789abcdef",
        "instance-1",
        "operator",
        ControlAction::Shutdown {
            grace_millis: adl_runtime_kernel::MAX_SHUTDOWN_GRACE_MILLIS + 1,
        },
        "operator-key",
        &key,
    );
    assert_eq!(result.unwrap_err(), ControlError::InvalidBounds);
}

#[test]
fn runtime_instance_identity_is_stable_in_one_state_root_and_distinct_across_roots() {
    let first_root = tempfile::tempdir().unwrap();
    let second_root = tempfile::tempdir().unwrap();
    let first = adl_runtime_kernel::load_or_create_runtime_instance_id(first_root.path()).unwrap();
    let restored =
        adl_runtime_kernel::load_or_create_runtime_instance_id(first_root.path()).unwrap();
    let separate =
        adl_runtime_kernel::load_or_create_runtime_instance_id(second_root.path()).unwrap();
    assert_eq!(first, restored);
    assert_ne!(first, separate);

    std::fs::write(first_root.path().join("runtime-instance-id"), "invalid\n").unwrap();
    assert!(adl_runtime_kernel::load_or_create_runtime_instance_id(first_root.path()).is_err());
}

#[test]
fn ready_event_reports_the_bound_ephemeral_port() {
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 43_123));
    let event = adl_runtime_kernel::control_ready_event(
        "instance-1",
        address,
        "https://runtime.example.test:43123",
    );
    assert!(event.contains("event=control_ready"));
    assert!(event.contains("port=43123"));
    assert!(!event.contains("port=20997"));
    assert!(event.contains("public_base_url=https://runtime.example.test:43123"));
}

#[test]
fn payload_and_human_observability_use_separate_redacted_channels() {
    let response = adl_runtime_kernel::ControlResponse {
        schema: adl_runtime_kernel::CONTROL_RESPONSE_SCHEMA.to_owned(),
        command_id: "read-1".to_owned(),
        correlation_id: "correlation-1".to_owned(),
        outcome: ControlOutcome::Snapshot {
            snapshot: Box::new(RuntimeRecorder::new(2).snapshot()),
        },
    };
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    write_payload(&mut stdout, &response).unwrap();
    let correlation_id = "0123456789abcdef0123456789abcdef";
    write_observability_event(
        &mut stderr,
        ControlObservabilityEvent::SnapshotCompleted,
        correlation_id,
    )
    .unwrap();
    let stdout = String::from_utf8(stdout).unwrap();
    let stderr = String::from_utf8(stderr).unwrap();
    assert!(stdout.starts_with('{') && !stdout.contains("adl_event"));
    assert!(stderr.starts_with("adl_event ") && !stderr.contains("authorization"));

    let mut rejected = Vec::new();
    assert_eq!(
        write_observability_event(
            &mut rejected,
            ControlObservabilityEvent::CommandRejected,
            "authorization-secret",
        )
        .unwrap_err(),
        ControlError::InvalidIdentifier
    );
    assert!(rejected.is_empty());
}

#[test]
fn bootstrap_promotes_once_after_explicit_degraded_readiness() {
    let recorder = RuntimeRecorder::new(4);
    recorder.emit(None, RuntimeEvent::KernelStarting);
    recorder.emit(None, RuntimeEvent::ComponentsReady);
    let promoted = recorder.initialize_observability(ObservabilityHealth::Degraded {
        reason: ObservabilityDegradation::ExporterUnavailable,
    });
    assert_eq!(promoted.len(), 2);
    assert_eq!(promoted[0].sequence, 0);
    assert_eq!(promoted[1].sequence, 1);
    assert!(recorder
        .initialize_observability(ObservabilityHealth::Ready)
        .is_empty());
    assert!(matches!(
        recorder.snapshot().observability,
        ObservabilityHealth::Degraded { .. }
    ));
}

#[tokio::test]
async fn signed_shutdown_routes_through_supervisor_and_carries_correlation() {
    let key = SigningKey::from_bytes(&[5; 32]);
    let recorder = RuntimeRecorder::new(8);
    let handle = Kernel::new(
        ComponentRegistry::new().validate().unwrap(),
        recorder.clone(),
    )
    .start()
    .await
    .unwrap();
    let service = Arc::new(ControlService::new(
        "instance-1",
        recorder.clone(),
        handle.control(),
        authority(&key, [ControlCapability::Stop]),
        4,
    ));
    let command = signed(&key, "stop-1", ControlAction::Shutdown { grace_millis: 50 });
    let response = service.execute(command).await.unwrap();
    assert_eq!(
        response.outcome,
        ControlOutcome::Shutdown {
            exit: ControlExit::Clean
        }
    );
    assert_eq!(handle.wait().await.unwrap(), KernelExit::Clean);
    assert!(recorder
        .events()
        .iter()
        .any(|event| event.correlation_id.as_deref() == Some(response.correlation_id.as_str())));
}
