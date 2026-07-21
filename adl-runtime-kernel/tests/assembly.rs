use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_live_assembly, mark_unavailable_live_services, AdapterKind,
    ClockAuthority, ComponentId, DegradedOperationExecutor, DomainWork, ExecutorError,
    IngressError, LiveBindings, OperationExecutor, OperationRequest, RunningState, RuntimeRecorder,
    TimeQualificationBounds, TimeSample, TimeSampleError, TimeSampleSource, DOMAIN_WORK_SCHEMA,
    PASSIVE_LIVE_SERVICES, REQUIRED_OPERATIONAL_ADAPTERS,
};
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use tokio::sync::Notify;

struct FixedTime;

struct EchoExecutor {
    calls: Arc<AtomicUsize>,
    request: Arc<Mutex<Option<OperationRequest>>>,
}

#[async_trait]
impl OperationExecutor for EchoExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        *self.request.lock().unwrap() = Some(request.clone());
        Ok(request.payload.clone())
    }
}

struct DelayedExecutor {
    started: Arc<Notify>,
    release: Arc<Notify>,
}

#[async_trait]
impl OperationExecutor for DelayedExecutor {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        self.started.notify_one();
        self.release.notified().await;
        Ok(request.payload.clone())
    }
}

#[async_trait]
impl TimeSampleSource for FixedTime {
    async fn sample(&self) -> Result<TimeSample, TimeSampleError> {
        Ok(TimeSample {
            source: "test-sntp".to_owned(),
            unix_millis: 1_720_000_000_000,
            offset_millis: 1,
            round_trip: Duration::from_millis(1),
        })
    }
}

fn bindings(recorder: RuntimeRecorder) -> LiveBindings {
    let executors = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            (
                kind,
                Arc::new(DegradedOperationExecutor::new("not configured"))
                    as Arc<dyn adl_runtime_kernel::OperationExecutor>,
            )
        })
        .collect();
    let key = SigningKey::from_bytes(&[31; 32]);
    LiveBindings {
        recorder: recorder.clone(),
        operation_executors: executors,
        permit_keys: BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
        reasoning: bootstrap_reasoning_services(recorder).unwrap(),
        time_source: Arc::new(FixedTime),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::from_millis(100),
            max_round_trip: Duration::from_millis(100),
        },
    }
}

#[test]
fn live_assembly_has_the_frozen_service_inventory() {
    let recorder = RuntimeRecorder::new(128);
    let assembly = build_live_assembly(bindings(recorder)).unwrap();
    let names = adl_runtime_kernel::live_service_names(&assembly.contracts);
    let expected = BTreeSet::from([
        "a2a".to_owned(),
        "acip".to_owned(),
        "adaptation_state".to_owned(),
        "aee".to_owned(),
        "agent_runtime".to_owned(),
        "checkpoint_store".to_owned(),
        "canonical_ingress".to_owned(),
        "chronosense".to_owned(),
        "cloud_bridge".to_owned(),
        "cognition_review_record".to_owned(),
        "curiosity_intelligence_theory_of_mind_adapter".to_owned(),
        "evaluation_feedback".to_owned(),
        "freedom_gate".to_owned(),
        "governance_audit".to_owned(),
        "governance_ingress".to_owned(),
        "lifelog".to_owned(),
        "loop_executor".to_owned(),
        "moral_affect_wellbeing_adapter".to_owned(),
        "mutation_gate".to_owned(),
        "observability".to_owned(),
        "provider".to_owned(),
        "reasoning_graph".to_owned(),
        "scheduler".to_owned(),
        "shepherd".to_owned(),
        "signed_continuity".to_owned(),
        "system_weather".to_owned(),
        "trusted_time".to_owned(),
    ]);
    assert_eq!(names, expected);
    assert_eq!(assembly.topology.startup_order().len(), 27);
}

#[test]
fn live_assembly_refuses_a_missing_executor_binding() {
    let recorder = RuntimeRecorder::new(128);
    let mut bindings = bindings(recorder);
    bindings
        .operation_executors
        .remove(&AdapterKind::CloudBridge);
    let error = match build_live_assembly(bindings) {
        Ok(_) => panic!("missing binding must be refused"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("CloudBridge"));
}

#[tokio::test]
async fn live_assembly_starts_and_qualifies_time() {
    let recorder = RuntimeRecorder::new(128);
    let assembly = build_live_assembly(bindings(recorder.clone())).unwrap();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder.clone())
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if matches!(
                recorder.snapshot().clock,
                ClockAuthority::Authoritative { .. }
            ) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    mark_unavailable_live_services(&recorder);
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.components.len(), 27);
    let degraded = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| kind.service_name())
        .chain(PASSIVE_LIVE_SERVICES)
        .collect::<BTreeSet<_>>();
    for (component, state) in &snapshot.components {
        let expected = if degraded.contains(component.as_str()) {
            RunningState::Degraded
        } else {
            RunningState::Running
        };
        assert_eq!(*state, expected, "unexpected state for {component:?}");
    }
    assert_eq!(
        snapshot.components[&ComponentId::new("observability")],
        RunningState::Running
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}

#[tokio::test]
async fn canonical_ingress_dispatches_allowlisted_work_and_commits_only_success() {
    let recorder = RuntimeRecorder::new(128);
    let calls = Arc::new(AtomicUsize::new(0));
    let dispatched = Arc::new(Mutex::new(None));
    let mut live = bindings(recorder.clone());
    live.operation_executors.insert(
        AdapterKind::Agent,
        Arc::new(EchoExecutor {
            calls: calls.clone(),
            request: dispatched.clone(),
        }),
    );
    let assembly = build_live_assembly(live).unwrap();
    let ingress = assembly.canonical_ingress.clone();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .unwrap();
    let work = DomainWork {
        schema: DOMAIN_WORK_SCHEMA.to_owned(),
        work_id: "dispatch-success".to_owned(),
        kind: "parity-a".to_owned(),
        payload: b"component-output".to_vec(),
    };
    let result = ingress
        .submit(work.clone(), "0123456789abcdef0123456789abcdef".to_owned())
        .await
        .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(result.accepted_sequence, 1);
    assert_eq!(
        dispatched.lock().unwrap().as_ref().unwrap(),
        &OperationRequest {
            schema: adl_runtime_kernel::OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: "dispatch-success".to_owned(),
            idempotency_key: "dispatch-success".to_owned(),
            principal: "canonical-ingress".to_owned(),
            payload: b"component-output".to_vec(),
            permit: None,
        }
    );

    let unsupported = ingress
        .submit(
            DomainWork {
                work_id: "dispatch-unsupported".to_owned(),
                kind: "not-allowlisted".to_owned(),
                ..work.clone()
            },
            "1123456789abcdef0123456789abcdef".to_owned(),
        )
        .await;
    assert_eq!(unsupported, Err(IngressError::UnsupportedKind));
    let failed = ingress
        .submit(
            DomainWork {
                work_id: "dispatch-failed".to_owned(),
                kind: AdapterKind::Shepherd.service_name().to_owned(),
                ..work
            },
            "2123456789abcdef0123456789abcdef".to_owned(),
        )
        .await;
    assert_eq!(failed, Err(IngressError::ExecutionFailed));
    assert_eq!(ingress.snapshot().accepted_through, 1);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}

#[tokio::test]
async fn closing_ingress_rejects_new_work_and_drains_an_accepted_dispatch() {
    let recorder = RuntimeRecorder::new(128);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let mut live = bindings(recorder.clone());
    live.operation_executors.insert(
        AdapterKind::Agent,
        Arc::new(DelayedExecutor {
            started: started.clone(),
            release: release.clone(),
        }),
    );
    let assembly = build_live_assembly(live).unwrap();
    let ingress = assembly.canonical_ingress.clone();
    let handle = adl_runtime_kernel::Kernel::new(assembly.topology, recorder)
        .start()
        .await
        .unwrap();
    let accepted = {
        let ingress = ingress.clone();
        tokio::spawn(async move {
            ingress
                .submit(
                    DomainWork {
                        schema: DOMAIN_WORK_SCHEMA.to_owned(),
                        work_id: "accepted-before-close".to_owned(),
                        kind: "parity-a".to_owned(),
                        payload: b"delayed".to_vec(),
                    },
                    "3123456789abcdef0123456789abcdef".to_owned(),
                )
                .await
        })
    };
    started.notified().await;
    let drain = {
        let ingress = ingress.clone();
        tokio::spawn(async move { ingress.close_and_drain(Duration::from_secs(1)).await })
    };
    tokio::task::yield_now().await;
    assert_eq!(
        ingress
            .submit(
                DomainWork {
                    schema: DOMAIN_WORK_SCHEMA.to_owned(),
                    work_id: "rejected-after-close".to_owned(),
                    kind: "parity-a".to_owned(),
                    payload: b"late".to_vec(),
                },
                "4123456789abcdef0123456789abcdef".to_owned(),
            )
            .await,
        Err(IngressError::Closed)
    );
    assert!(!drain.is_finished());
    release.notify_one();
    assert!(accepted.await.unwrap().is_ok());
    drain.await.unwrap().unwrap();
    assert_eq!(ingress.snapshot().accepted_through, 1);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        adl_runtime_kernel::KernelExit::Clean
    );
}
