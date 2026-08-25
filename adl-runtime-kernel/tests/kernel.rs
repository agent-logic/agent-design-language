use std::{
    future::pending,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use adl_runtime_kernel::{
    channel,
    proof::{build_proof_runtime, load_capsule, run_proof},
    ChannelFullPolicy, ClockAuthority, Component, ComponentContext, ComponentError,
    ComponentFactory, ComponentId, ComponentRegistry, ComponentSpec, FailurePolicy, Kernel,
    KernelExit, LifecycleRole, PortSpec, RunningState, RuntimeRecorder, SupervisionScope,
    TopologyError,
};
use async_trait::async_trait;

#[tokio::test]
async fn bounded_reject_channel_reports_saturation() {
    let (sender, _receiver) = channel(1, ChannelFullPolicy::Reject);
    sender.send(1_u8).await.unwrap();
    assert!(sender.send(2_u8).await.is_err());
    assert_eq!(sender.metrics().sent(), 1);
    assert_eq!(sender.metrics().rejected(), 1);
}

#[tokio::test]
async fn channel_metrics_remain_safe_under_concurrent_pressure() {
    let (sender, mut receiver) = channel(8, ChannelFullPolicy::Reject);
    let metrics = sender.metrics();
    let mut producers = Vec::new();
    for producer in 0..8_u64 {
        let sender = sender.clone();
        producers.push(tokio::spawn(async move {
            for sequence in 0..1_000_u64 {
                let _ = sender.send((producer, sequence)).await;
                tokio::task::yield_now().await;
            }
        }));
    }
    drop(sender);
    let consumer = tokio::spawn(async move {
        let mut received = 0_u64;
        while receiver.recv().await.is_some() {
            received += 1;
        }
        received
    });
    for producer in producers {
        producer.await.unwrap();
    }
    let received = consumer.await.unwrap();
    let (_, capacity, depth, high_water, sent, rejected) = metrics.snapshot();
    assert_eq!(depth, 0);
    assert!(high_water <= capacity as u64);
    assert_eq!(sent, received);
    assert_eq!(sent + rejected, 8_000);
}

#[test]
fn topology_rejects_missing_dependencies_before_start() {
    let mut registry = ComponentRegistry::new();
    registry.register(SimpleFactory::new("child", &["missing"]));
    let result = registry.validate();
    assert!(matches!(
        result,
        Err(TopologyError::MissingDependency { .. })
    ));
}

#[test]
fn topology_rejects_cycles_before_start() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(SimpleFactory::new("first", &["second"]))
        .register(SimpleFactory::new("second", &["first"]));
    let result = registry.validate();
    assert!(matches!(result, Err(TopologyError::Cycle(_))));
}

#[tokio::test]
async fn representative_topology_promotes_events_and_clock_authority() {
    let directory = tempfile::tempdir().unwrap();
    let capsule_path = directory.path().join("continuity.json");
    let proof = build_proof_runtime(&capsule_path, 3).unwrap();
    assert!(matches!(
        proof.recorder.snapshot().clock,
        ClockAuthority::Degraded { .. }
    ));
    let handle = proof.kernel.start().await.unwrap();
    tokio::time::sleep(Duration::from_millis(40)).await;
    let snapshot = handle.recorder().snapshot();
    assert!(snapshot.observability_ready);
    assert!(matches!(
        snapshot.clock,
        ClockAuthority::Authoritative { .. }
    ));
    assert_eq!(
        snapshot.components.get(&ComponentId::from("checkpoint")),
        Some(&RunningState::Running)
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );

    let capsule = load_capsule(&capsule_path).await.unwrap();
    assert!(capsule.validate());
    assert_eq!(capsule.processed_sequences, vec![0, 1, 2]);
}

#[tokio::test]
async fn continuity_generation_advances_across_fresh_kernel_runs() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    let (_, first) = run_proof(&path, 2).await.unwrap();
    let (_, second) = run_proof(&path, 2).await.unwrap();
    assert_eq!(first.generation, 1);
    assert_eq!(second.generation, 2);
    assert!(second.validate());
}

#[tokio::test]
async fn proof_waits_for_all_items_instead_of_sleeping() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    let (_, capsule) = run_proof(&path, 1_000).await.unwrap();
    assert_eq!(capsule.processed_sequences.len(), 1_000);
    assert_eq!(capsule.processed_sequences[999], 999);
}

#[tokio::test]
async fn corrupt_capsule_fails_closed_during_shutdown() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("continuity.json");
    std::fs::write(&path, b"not valid continuity").unwrap();
    let proof = build_proof_runtime(&path, 1).unwrap();
    let handle = proof.kernel.start().await.unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        loop {
            if proof.evidence.lock().unwrap().len() == 1 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::ShutdownFailed {
            components: vec![ComponentId::from("checkpoint")]
        }
    );
    assert_eq!(std::fs::read(&path).unwrap(), b"not valid continuity");
}

#[tokio::test]
async fn restart_policy_rebuilds_failed_component() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(RestartFactory {
        builds: builds.clone(),
    });
    let recorder = RuntimeRecorder::new(16);
    let handle = Kernel::new(registry.validate().unwrap(), recorder)
        .start()
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(40)).await;
    assert_eq!(builds.load(Ordering::SeqCst), 2);
    assert_eq!(
        handle.recorder().snapshot().components[&ComponentId::from("restartable")],
        RunningState::Running
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[tokio::test]
async fn shutdown_remains_responsive_during_restart_backoff() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(LongBackoffFactory {
        builds: builds.clone(),
    });
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while builds.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
        handle.shutdown(Duration::from_millis(100)).await.unwrap()
    })
    .await
    .expect("shutdown must not wait for the five-second restart backoff");
}

#[tokio::test]
async fn fatal_component_exit_is_observable_by_process_owner() {
    let mut registry = ComponentRegistry::new();
    registry.register(FatalFactory);
    let recorder = RuntimeRecorder::new(16);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.wait().await.unwrap(),
        KernelExit::Fatal {
            component: ComponentId::from("fatal")
        }
    );
    assert_eq!(
        recorder.snapshot().lifecycle,
        adl_runtime_kernel::LifecycleState::Failed
    );
}

#[tokio::test]
async fn shutdown_deadline_aborts_non_cooperative_component() {
    let mut registry = ComponentRegistry::new();
    registry.register(StuckFactory);
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    let exit = handle.shutdown(Duration::from_millis(10)).await.unwrap();
    assert!(matches!(
        exit,
        KernelExit::ShutdownDeadlineExceeded { aborted } if aborted == vec![ComponentId::from("stuck")]
    ));
}

#[tokio::test]
async fn startup_and_shutdown_follow_dependency_order() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(SimpleFactory::new("foundation", &[]))
        .register(SimpleFactory::new("dependent", &["foundation"]));
    let recorder = RuntimeRecorder::new(32);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );

    let transitions = recorder
        .events()
        .into_iter()
        .filter(|event| event.event == "state:Running" || event.event == "state:Stopping")
        .map(|event| (event.component.unwrap(), event.event))
        .collect::<Vec<_>>();
    assert_eq!(
        transitions,
        vec![
            (ComponentId::from("foundation"), "state:Running".to_owned()),
            (ComponentId::from("dependent"), "state:Running".to_owned()),
            (ComponentId::from("dependent"), "state:Stopping".to_owned()),
            (ComponentId::from("foundation"), "state:Stopping".to_owned()),
        ]
    );
}

#[test]
fn topology_rejects_mismatched_port_types() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(TypedFactory::producer())
        .register(TypedFactory::mismatched_consumer());
    assert!(matches!(
        registry.validate(),
        Err(TopologyError::UnsatisfiedInput { component, .. })
            if component == ComponentId::from("consumer")
    ));
}

#[test]
fn topology_rejects_duplicate_and_ambiguous_port_authority() {
    let duplicate = TypedFactory {
        spec: ComponentSpec {
            id: ComponentId::from("duplicate"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![
                PortSpec::typed::<u8>("values"),
                PortSpec::typed::<u8>("values"),
            ],
            failure_policy: FailurePolicy::Fatal,
        },
    };
    let mut registry = ComponentRegistry::new();
    registry.register(duplicate);
    assert!(matches!(
        registry.validate(),
        Err(TopologyError::DuplicatePort { .. })
    ));

    let input = PortSpec::typed::<u8>("values");
    let mut registry = ComponentRegistry::new();
    registry
        .register(TypedFactory::producer())
        .register(TypedFactory {
            spec: ComponentSpec {
                id: ComponentId::from("producer_two"),
                dependencies: vec![],
                inputs: vec![],
                outputs: vec![input.clone()],
                failure_policy: FailurePolicy::Fatal,
            },
        })
        .register(TypedFactory {
            spec: ComponentSpec {
                id: ComponentId::from("consumer"),
                dependencies: vec![
                    ComponentId::from("producer"),
                    ComponentId::from("producer_two"),
                ],
                inputs: vec![input],
                outputs: vec![],
                failure_policy: FailurePolicy::Fatal,
            },
        });
    assert!(matches!(
        registry.validate(),
        Err(TopologyError::AmbiguousInput { .. })
    ));
}

#[derive(Clone)]
struct SimpleFactory {
    id: &'static str,
    dependencies: &'static [&'static str],
}

impl SimpleFactory {
    fn new(id: &'static str, dependencies: &'static [&'static str]) -> Self {
        Self { id, dependencies }
    }
}

impl ComponentFactory for SimpleFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: self
                .dependencies
                .iter()
                .map(|id| ComponentId::from(*id))
                .collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(WaitingComponent)
    }
}

struct WaitingComponent;

#[async_trait]
impl Component for WaitingComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct RestartFactory {
    builds: Arc<AtomicU32>,
}

struct LongBackoffFactory {
    builds: Arc<AtomicU32>,
}

impl ComponentFactory for LongBackoffFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("long_backoff"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::restart(2, Duration::from_secs(5)),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        self.builds.fetch_add(1, Ordering::SeqCst);
        Box::new(AlwaysFailComponent)
    }
}

struct AlwaysFailComponent;

#[async_trait]
impl Component for AlwaysFailComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        Err(ComponentError::new("injected restart failure"))
    }
}

struct FatalFactory;

impl ComponentFactory for FatalFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("fatal"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(AlwaysFailComponent)
    }
}

struct TypedFactory {
    spec: ComponentSpec,
}

impl TypedFactory {
    fn producer() -> Self {
        Self {
            spec: ComponentSpec {
                id: ComponentId::from("producer"),
                dependencies: vec![],
                inputs: vec![],
                outputs: vec![PortSpec::typed::<u8>("values")],
                failure_policy: FailurePolicy::Fatal,
            },
        }
    }

    fn mismatched_consumer() -> Self {
        Self {
            spec: ComponentSpec {
                id: ComponentId::from("consumer"),
                dependencies: vec![ComponentId::from("producer")],
                inputs: vec![PortSpec::typed::<u16>("values")],
                outputs: vec![],
                failure_policy: FailurePolicy::Fatal,
            },
        }
    }
}

impl ComponentFactory for TypedFactory {
    fn spec(&self) -> ComponentSpec {
        self.spec.clone()
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(WaitingComponent)
    }
}

impl ComponentFactory for RestartFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("restartable"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![PortSpec::typed::<u8>("proof")],
            failure_policy: FailurePolicy::restart(1, Duration::from_millis(1)),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(RestartComponent {
            generation: self.builds.fetch_add(1, Ordering::SeqCst),
        })
    }
}

struct RestartComponent {
    generation: u32,
}

#[async_trait]
impl Component for RestartComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        if self.generation == 0 {
            return Err(ComponentError::new("injected first-generation failure"));
        }
        context.cancellation.cancelled().await;
        Ok(())
    }
}

struct StuckFactory;

impl ComponentFactory for StuckFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("stuck"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(StuckComponent)
    }
}

struct StuckComponent;

#[async_trait]
impl Component for StuckComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        pending::<()>().await;
        Ok(())
    }
}

#[derive(Clone)]
struct PortFactory {
    producer: bool,
    observed: Arc<AtomicU32>,
}

impl ComponentFactory for PortFactory {
    fn spec(&self) -> ComponentSpec {
        let port = PortSpec::bounded("events", "adl.test.events.v1", 1, ChannelFullPolicy::Reject);
        ComponentSpec {
            id: ComponentId::from(if self.producer {
                "producer"
            } else {
                "consumer"
            }),
            dependencies: if self.producer {
                vec![]
            } else {
                vec![ComponentId::from("producer")]
            },
            inputs: if self.producer {
                vec![]
            } else {
                vec![port.clone()]
            },
            outputs: if self.producer { vec![port] } else { vec![] },
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(PortComponent {
            producer: self.producer,
            observed: self.observed.clone(),
        })
    }
}

struct PortComponent {
    producer: bool,
    observed: Arc<AtomicU32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TestEvent {
    sequence: u32,
}

impl adl_runtime_kernel::PortProtocol for TestEvent {
    const PROTOCOL: &'static str = "adl.test.events.v1";
}

#[async_trait]
impl Component for PortComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        if self.producer {
            context
                .send("events", &TestEvent { sequence: 7 })
                .await
                .map_err(|error| ComponentError::new(error.to_string()))?;
            assert!(context
                .send("undeclared", &TestEvent { sequence: 0 })
                .await
                .is_err());
            context.ready();
            context.cancellation.cancelled().await;
        } else {
            let value = context
                .recv::<TestEvent>("events")
                .await
                .map_err(|error| ComponentError::new(error.to_string()))?
                .ok_or_else(|| ComponentError::new("declared input closed"))?;
            self.observed.store(value.sequence, Ordering::SeqCst);
            assert!(context.recv::<TestEvent>("undeclared").await.is_err());
            context.ready();
            context.cancellation.cancelled().await;
        }
        Ok(())
    }
}

#[tokio::test]
async fn kernel_owned_ports_are_operational_and_undeclared_access_is_denied() {
    let observed = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry
        .register(PortFactory {
            producer: true,
            observed: observed.clone(),
        })
        .register(PortFactory {
            producer: false,
            observed: observed.clone(),
        });
    let recorder = RuntimeRecorder::new(32);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    assert_eq!(observed.load(Ordering::SeqCst), 7);
    let snapshot = recorder.snapshot();
    assert_eq!(snapshot.queues.len(), 1);
    assert_eq!(snapshot.queues.values().next().unwrap().sent, 1);
    assert!(recorder.health().ready);
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

struct WindowRestartFactory {
    builds: Arc<AtomicU32>,
}

impl ComponentFactory for WindowRestartFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("windowed"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::restart_windowed(
                1,
                Duration::from_millis(1),
                Duration::from_millis(10),
                SupervisionScope::OneForOne,
            ),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(WindowRestartComponent {
            generation: self.builds.fetch_add(1, Ordering::SeqCst),
        })
    }
}

struct WindowRestartComponent {
    generation: u32,
}

#[async_trait]
impl Component for WindowRestartComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        if self.generation < 2 {
            if self.generation == 1 {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            return Err(ComponentError::new("windowed failure"));
        }
        context.cancellation.cancelled().await;
        Ok(())
    }
}

#[tokio::test]
async fn restart_budget_resets_after_the_declared_time_window() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(WindowRestartFactory {
        builds: builds.clone(),
    });
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while builds.load(Ordering::SeqCst) < 3 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

struct ReadinessRestartFactory {
    builds: Arc<AtomicU32>,
}

impl ComponentFactory for ReadinessRestartFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("readiness_restart"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::restart_windowed(
                2,
                Duration::from_millis(1),
                Duration::from_secs(1),
                SupervisionScope::OneForOne,
            ),
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(ReadinessRestartComponent {
            generation: self.builds.fetch_add(1, Ordering::SeqCst),
        })
    }
}

struct ReadinessRestartComponent {
    generation: u32,
}

#[async_trait]
impl Component for ReadinessRestartComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        match self.generation {
            0 => {
                context.cancellation.cancelled().await;
                Ok(())
            }
            1 => {
                context.cancellation.cancelled().await;
                Ok(())
            }
            _ => {
                context.ready();
                context.cancellation.cancelled().await;
                Ok(())
            }
        }
    }
}

#[tokio::test]
async fn initial_readiness_failure_uses_the_declared_restart_policy() {
    let builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry.register(ReadinessRestartFactory {
        builds: builds.clone(),
    });
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .with_readiness_timeout(Duration::from_millis(10))
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while builds.load(Ordering::SeqCst) < 3 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[derive(Clone)]
struct ScopeFactory {
    id: &'static str,
    dependency: Option<&'static str>,
    builds: Arc<AtomicU32>,
    fail_first: bool,
    slow_cancel_first: bool,
}

impl ComponentFactory for ScopeFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: self.dependency.into_iter().map(ComponentId::from).collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: if self.fail_first {
                FailurePolicy::restart_windowed(
                    1,
                    Duration::from_millis(1),
                    Duration::from_secs(1),
                    SupervisionScope::OneForAll,
                )
            } else {
                FailurePolicy::Fatal
            },
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(ScopeComponent {
            generation: self.builds.fetch_add(1, Ordering::SeqCst),
            fail_first: self.fail_first,
            slow_cancel_first: self.slow_cancel_first,
        })
    }
}

struct ScopeComponent {
    generation: u32,
    fail_first: bool,
    slow_cancel_first: bool,
}

#[async_trait]
impl Component for ScopeComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        if self.fail_first && self.generation == 0 {
            tokio::task::yield_now().await;
            return Err(ComponentError::new("one-for-all trigger"));
        }
        context.cancellation.cancelled().await;
        if self.slow_cancel_first && self.generation == 0 {
            tokio::time::sleep(Duration::from_millis(50)).await;
            context.degraded().await?;
        }
        Ok(())
    }
}

#[tokio::test]
async fn one_for_all_restarts_transitive_dependents() {
    let parent_builds = Arc::new(AtomicU32::new(0));
    let child_builds = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry
        .register(ScopeFactory {
            id: "parent",
            dependency: None,
            builds: parent_builds.clone(),
            fail_first: true,
            slow_cancel_first: false,
        })
        .register(ScopeFactory {
            id: "child",
            dependency: Some("parent"),
            builds: child_builds.clone(),
            fail_first: false,
            slow_cancel_first: true,
        });
    let recorder = RuntimeRecorder::new(32);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while parent_builds.load(Ordering::SeqCst) < 2 || child_builds.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        recorder.snapshot().components[&ComponentId::from("child")],
        RunningState::Running,
        "a delayed old incarnation must not close or stop its replacement"
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[derive(Clone)]
struct DegradeFactory {
    id: &'static str,
    dependencies: &'static [&'static str],
    degrade: bool,
    required_core: bool,
}

impl ComponentFactory for DegradeFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: self
                .dependencies
                .iter()
                .copied()
                .map(ComponentId::from)
                .collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: if self.degrade {
                FailurePolicy::Degrade
            } else {
                FailurePolicy::Fatal
            },
        }
    }

    fn build(&self) -> Box<dyn Component> {
        if self.degrade {
            Box::new(AlwaysFailComponent)
        } else {
            Box::new(WaitingComponent)
        }
    }

    fn required_core(&self) -> bool {
        self.required_core
    }
}

#[tokio::test]
async fn degradation_propagates_capability_loss_to_dependents() {
    let mut registry = ComponentRegistry::new();
    registry
        .register(DegradeFactory {
            id: "foundation",
            dependencies: &[],
            degrade: false,
            required_core: true,
        })
        .register(DegradeFactory {
            id: "optional",
            dependencies: &["foundation"],
            degrade: true,
            required_core: false,
        })
        .register(DegradeFactory {
            id: "dependent",
            dependencies: &["optional"],
            degrade: false,
            required_core: false,
        });
    let recorder = RuntimeRecorder::new(32);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while recorder.health().degraded_components.len() < 2 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        recorder.health().degraded_components,
        vec![
            ComponentId::from("dependent"),
            ComponentId::from("optional")
        ]
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[tokio::test]
async fn declared_required_core_degradation_is_terminal() {
    let mut registry = ComponentRegistry::new();
    registry.register(DegradeFactory {
        id: "required-core",
        dependencies: &[],
        degrade: true,
        required_core: true,
    });
    let mut handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.wait_for_exit().await.unwrap(),
        KernelExit::Fatal {
            component: ComponentId::from("required-core")
        }
    );
}

#[derive(Clone)]
struct LayerBarrierFactory {
    id: &'static str,
    entered: Arc<AtomicU32>,
}

impl ComponentFactory for LayerBarrierFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(LayerBarrierComponent {
            entered: self.entered.clone(),
        })
    }
}

struct LayerBarrierComponent {
    entered: Arc<AtomicU32>,
}

#[async_trait]
impl Component for LayerBarrierComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        self.entered.fetch_add(1, Ordering::SeqCst);
        tokio::time::timeout(Duration::from_millis(100), async {
            while self.entered.load(Ordering::SeqCst) < 2 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .map_err(|_| ComponentError::new("same-layer startup was serialized"))?;
        context.ready();
        context.cancellation.cancelled().await;
        Ok(())
    }
}

#[tokio::test]
async fn independent_components_start_concurrently_within_a_topology_layer() {
    let entered = Arc::new(AtomicU32::new(0));
    let mut registry = ComponentRegistry::new();
    registry
        .register(LayerBarrierFactory {
            id: "first",
            entered: entered.clone(),
        })
        .register(LayerBarrierFactory {
            id: "second",
            entered,
        });
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(16))
        .with_readiness_timeout(Duration::from_millis(200))
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[derive(Clone)]
struct HealthReportingFactory;

impl ComponentFactory for HealthReportingFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from("health-reporter"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(HealthReportingComponent)
    }
}

struct HealthReportingComponent;

#[async_trait]
impl Component for HealthReportingComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        tokio::task::yield_now().await;
        context.degraded().await?;
        context.cancellation.cancelled().await;
        Ok(())
    }
}

#[tokio::test]
async fn live_component_can_report_degraded_health_without_terminating() {
    let mut registry = ComponentRegistry::new();
    registry.register(HealthReportingFactory);
    let recorder = RuntimeRecorder::new(16);
    let handle = Kernel::new(registry.validate().unwrap(), recorder.clone())
        .start()
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), async {
        while recorder.health().degraded_components.is_empty() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(
        recorder.health().degraded_components,
        vec![ComponentId::from("health-reporter")]
    );
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
}

#[derive(Clone)]
struct LifecycleFactory {
    id: &'static str,
    dependency: Option<&'static str>,
    role: LifecycleRole,
    stopped: Arc<Mutex<Vec<&'static str>>>,
}

impl ComponentFactory for LifecycleFactory {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::from(self.id),
            dependencies: self.dependency.into_iter().map(ComponentId::from).collect(),
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }

    fn build(&self) -> Box<dyn Component> {
        Box::new(LifecycleComponent {
            id: self.id,
            stopped: self.stopped.clone(),
        })
    }

    fn lifecycle_role(&self) -> LifecycleRole {
        self.role
    }
}

struct LifecycleComponent {
    id: &'static str,
    stopped: Arc<Mutex<Vec<&'static str>>>,
}

#[async_trait]
impl Component for LifecycleComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        context.cancellation.cancelled().await;
        self.stopped.lock().unwrap().push(self.id);
        Ok(())
    }
}

#[tokio::test]
async fn shutdown_stops_ingress_then_flushes_checkpoint_telemetry_and_egress() {
    let stopped = Arc::new(Mutex::new(Vec::new()));
    let declarations = [
        ("ingress", None, LifecycleRole::Ingress),
        ("workload", Some("ingress"), LifecycleRole::Workload),
        ("checkpoint", Some("workload"), LifecycleRole::Checkpoint),
        ("telemetry", Some("checkpoint"), LifecycleRole::Telemetry),
        ("egress", Some("telemetry"), LifecycleRole::Egress),
    ];
    let mut registry = ComponentRegistry::new();
    for (id, dependency, role) in declarations {
        registry.register(LifecycleFactory {
            id,
            dependency,
            role,
            stopped: stopped.clone(),
        });
    }
    let handle = Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(32))
        .start()
        .await
        .unwrap();
    assert_eq!(
        handle.shutdown(Duration::from_secs(1)).await.unwrap(),
        KernelExit::Clean
    );
    assert_eq!(
        *stopped.lock().unwrap(),
        vec!["ingress", "workload", "checkpoint", "telemetry", "egress"]
    );
}
