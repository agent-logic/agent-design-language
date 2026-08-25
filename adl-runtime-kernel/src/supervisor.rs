use std::{
    collections::{BTreeMap, VecDeque},
    panic::AssertUnwindSafe,
    sync::Arc,
    time::{Duration, Instant},
};

use futures::FutureExt;
use thiserror::Error;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
    time::timeout,
};
use tokio_util::sync::CancellationToken;

use crate::{
    component::RuntimePortRegistry, ComponentContext, ComponentFactory, ComponentId, FailurePolicy,
    LifecycleRole, LifecycleState, RunningState, RuntimeEvent, RuntimeRecorder, SupervisionScope,
    ValidatedTopology,
};

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("component {component} failed: {message}")]
    ComponentFailed {
        component: ComponentId,
        message: String,
    },
    #[error("component {0} failed to report readiness")]
    Readiness(ComponentId),
    #[error("kernel command channel closed")]
    CommandChannelClosed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KernelExit {
    Clean,
    Fatal { component: ComponentId },
    ShutdownFailed { components: Vec<ComponentId> },
    ShutdownDeadlineExceeded { aborted: Vec<ComponentId> },
}

enum KernelCommand {
    Shutdown {
        grace: Duration,
        response: oneshot::Sender<KernelExit>,
    },
}

pub struct Kernel {
    topology: ValidatedTopology,
    recorder: RuntimeRecorder,
    readiness_timeout: Duration,
}

impl Kernel {
    pub fn new(topology: ValidatedTopology, recorder: RuntimeRecorder) -> Self {
        Self {
            topology,
            recorder,
            readiness_timeout: Duration::from_secs(5),
        }
    }

    pub fn with_readiness_timeout(mut self, timeout: Duration) -> Self {
        self.readiness_timeout = timeout;
        self
    }

    pub async fn start(self) -> Result<KernelHandle, KernelError> {
        let (command_tx, command_rx) = mpsc::channel(4);
        let recorder = self.recorder.clone();
        recorder.set_lifecycle(LifecycleState::Starting);
        let (started_tx, started_rx) = oneshot::channel();
        let task = tokio::spawn(async move { self.run(command_rx, started_tx).await });
        started_rx
            .await
            .map_err(|_| KernelError::CommandChannelClosed)??;
        Ok(KernelHandle {
            control: KernelControl { command_tx },
            recorder,
            task: Some(task),
        })
    }

    async fn run(
        self,
        mut commands: mpsc::Receiver<KernelCommand>,
        started: oneshot::Sender<Result<(), KernelError>>,
    ) -> Result<KernelExit, KernelError> {
        let mut tasks = JoinSet::<ComponentCompletion>::new();
        let ports = RuntimePortRegistry::new(
            self.topology.factories.keys().cloned(),
            self.topology.port_routes(),
            &self.recorder,
        );
        let mut cancellations = BTreeMap::<ComponentId, CancellationToken>::new();
        let mut restarts = BTreeMap::<ComponentId, VecDeque<Instant>>::new();
        let (restart_tx, mut restart_rx) =
            mpsc::channel::<(ComponentId, Arc<dyn ComponentFactory>)>(16);
        let (readiness_tx, mut readiness_rx) = mpsc::channel::<(ComponentId, bool)>(16);

        for layer in self.topology.dependency_layers() {
            let mut readiness = Vec::with_capacity(layer.len());
            for id in layer {
                let factory = self.topology.factories[&id].clone();
                let (ready_tx, ready_rx) = oneshot::channel();
                let cancellation = CancellationToken::new();
                cancellations.insert(id.clone(), cancellation.clone());
                self.recorder
                    .set_component_state(id.clone(), RunningState::Starting);
                spawn_component(
                    &mut tasks,
                    factory,
                    cancellation,
                    self.recorder.clone(),
                    ports.for_component(&id),
                    ready_tx,
                );
                readiness.push((id, ready_rx));
            }
            for (id, ready_rx) in readiness {
                match timeout(self.readiness_timeout, ready_rx).await {
                    Ok(Ok(())) => self
                        .recorder
                        .set_component_state(id.clone(), RunningState::Running),
                    _ => {
                        self.recorder.set_lifecycle(LifecycleState::Failed);
                        let error = KernelError::Readiness(id.clone());
                        let _ = started.send(Err(KernelError::Readiness(id.clone())));
                        cancel_all(&cancellations, self.topology.shutdown_order());
                        return Err(error);
                    }
                }
            }
        }
        self.recorder.set_lifecycle(LifecycleState::Running);
        let _ = started.send(Ok(()));

        loop {
            tokio::select! {
                command = commands.recv() => {
                    let Some(KernelCommand::Shutdown { grace, response }) = command else {
                        cancel_all(&cancellations, self.topology.shutdown_order());
                        return Ok(KernelExit::Clean);
                    };
                    self.recorder.set_lifecycle(LifecycleState::Stopping);
                    let exit = shutdown(
                        &mut tasks,
                        &cancellations,
                        shutdown_phases(&self.topology),
                        &self.recorder,
                        grace,
                    ).await;
                    self.recorder.set_lifecycle(match &exit {
                        KernelExit::Clean => LifecycleState::Stopped,
                        _ => LifecycleState::Failed,
                    });
                    let _ = response.send(exit.clone());
                    return Ok(exit);
                }
                Some((id, factory)) = restart_rx.recv() => {
                    let (ready_tx, ready_rx) = oneshot::channel();
                    let cancellation = CancellationToken::new();
                    cancellations.insert(id.clone(), cancellation.clone());
                    spawn_component(
                        &mut tasks,
                        factory,
                        cancellation,
                        self.recorder.clone(),
                        ports.for_component(&id),
                        ready_tx,
                    );
                    let readiness_tx = readiness_tx.clone();
                    let readiness_timeout = self.readiness_timeout;
                    tokio::spawn(async move {
                        let ready = matches!(timeout(readiness_timeout, ready_rx).await, Ok(Ok(())));
                        let _ = readiness_tx.send((id, ready)).await;
                    });
                }
                Some((id, ready)) = readiness_rx.recv() => {
                    if ready {
                        self.recorder.set_component_state(id, RunningState::Running);
                    } else {
                        let factory = self.topology.factories[&id].clone();
                        match failure_action(
                            &factory.spec().failure_policy,
                            restarts.entry(id.clone()).or_default(),
                        ) {
                            FailureAction::Fatal => {
                                self.recorder.set_lifecycle(LifecycleState::Failed);
                                self.recorder.set_component_state(id.clone(), RunningState::Failed);
                                cancel_all(&cancellations, self.topology.shutdown_order());
                                return Ok(KernelExit::Fatal { component: id });
                            }
                            FailureAction::Degrade => {
                                if apply_degradation(
                                    &self.topology,
                                    &cancellations,
                                    &self.recorder,
                                    &id,
                                ) {
                                    self.recorder.set_lifecycle(LifecycleState::Failed);
                                    cancel_all(&cancellations, self.topology.shutdown_order());
                                    return Ok(KernelExit::Fatal { component: id });
                                }
                            }
                            FailureAction::Restart { count, backoff, scope } => {
                                self.recorder.set_restart_count(id.clone(), count);
                                schedule_restart_scope(
                                    &self.topology,
                                    &cancellations,
                                    &self.recorder,
                                    &restart_tx,
                                    id,
                                    factory,
                                    backoff,
                                    scope,
                                );
                            }
                        }
                    }
                }
                completion = tasks.join_next(), if !tasks.is_empty() => {
                    let Some(completion) = completion else { continue; };
                    let completion = completion.map_err(|error| KernelError::ComponentFailed {
                        component: ComponentId::new("supervisor_wrapper"),
                        message: error.to_string(),
                    })?;
                    let id = completion.id.clone();
                    if completion.cancelled {
                        self.recorder.set_component_state(id, RunningState::Stopped);
                        continue;
                    }
                    if completion.error.is_some() {
                        let factory = self.topology.factories[&id].clone();
                        match factory.spec().failure_policy {
                            FailurePolicy::Fatal => {
                                self.recorder.set_lifecycle(LifecycleState::Failed);
                                self.recorder.set_component_state(id.clone(), RunningState::Failed);
                                cancel_all(&cancellations, self.topology.shutdown_order());
                                return Ok(KernelExit::Fatal { component: id });
                            }
                            FailurePolicy::Degrade => {
                                if apply_degradation(
                                    &self.topology,
                                    &cancellations,
                                    &self.recorder,
                                    &id,
                                ) {
                                    self.recorder.set_lifecycle(LifecycleState::Failed);
                                    cancel_all(&cancellations, self.topology.shutdown_order());
                                    return Ok(KernelExit::Fatal { component: id });
                                }
                            }
                            FailurePolicy::Restart { .. } => {
                                match failure_action(
                                    &factory.spec().failure_policy,
                                    restarts.entry(id.clone()).or_default(),
                                ) {
                                    FailureAction::Restart { count, backoff, scope } => {
                                        self.recorder.set_restart_count(id.clone(), count);
                                        schedule_restart_scope(
                                            &self.topology,
                                            &cancellations,
                                            &self.recorder,
                                            &restart_tx,
                                            id,
                                            factory,
                                            backoff,
                                            scope,
                                        );
                                    }
                                    FailureAction::Fatal => {
                                        self.recorder.set_lifecycle(LifecycleState::Failed);
                                        self.recorder.set_component_state(id.clone(), RunningState::Failed);
                                        cancel_all(&cancellations, self.topology.shutdown_order());
                                        return Ok(KernelExit::Fatal { component: id });
                                    }
                                    FailureAction::Degrade => unreachable!("restart policy cannot degrade"),
                                }
                            }
                        }
                    } else {
                        self.recorder.set_component_state(id, RunningState::Stopped);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct KernelControl {
    command_tx: mpsc::Sender<KernelCommand>,
}

impl KernelControl {
    pub async fn shutdown(&self, grace: Duration) -> Result<KernelExit, KernelError> {
        let (response_tx, response_rx) = oneshot::channel();
        self.command_tx
            .send(KernelCommand::Shutdown {
                grace,
                response: response_tx,
            })
            .await
            .map_err(|_| KernelError::CommandChannelClosed)?;
        response_rx
            .await
            .map_err(|_| KernelError::CommandChannelClosed)
    }
}

pub struct KernelHandle {
    control: KernelControl,
    recorder: RuntimeRecorder,
    task: Option<tokio::task::JoinHandle<Result<KernelExit, KernelError>>>,
}

impl KernelHandle {
    pub fn control(&self) -> KernelControl {
        self.control.clone()
    }

    pub fn recorder(&self) -> &RuntimeRecorder {
        &self.recorder
    }

    pub async fn shutdown(mut self, grace: Duration) -> Result<KernelExit, KernelError> {
        let response = self.control.shutdown(grace).await?;
        if let Some(task) = self.task.take() {
            task.await.map_err(|error| KernelError::ComponentFailed {
                component: ComponentId::new("kernel"),
                message: error.to_string(),
            })??;
        }
        Ok(response)
    }

    pub async fn wait(mut self) -> Result<KernelExit, KernelError> {
        self.task
            .take()
            .expect("kernel task must exist")
            .await
            .map_err(|error| KernelError::ComponentFailed {
                component: ComponentId::new("kernel"),
                message: error.to_string(),
            })?
    }

    pub async fn wait_for_exit(&mut self) -> Result<KernelExit, KernelError> {
        let result = self
            .task
            .as_mut()
            .expect("kernel task must exist")
            .await
            .map_err(|error| KernelError::ComponentFailed {
                component: ComponentId::new("kernel"),
                message: error.to_string(),
            })?;
        self.task.take();
        result
    }
}

struct ComponentCompletion {
    id: ComponentId,
    error: Option<String>,
    cancelled: bool,
}

enum FailureAction {
    Fatal,
    Degrade,
    Restart {
        count: u32,
        backoff: Duration,
        scope: SupervisionScope,
    },
}

fn failure_action(policy: &FailurePolicy, history: &mut VecDeque<Instant>) -> FailureAction {
    match *policy {
        FailurePolicy::Fatal => FailureAction::Fatal,
        FailurePolicy::Degrade => FailureAction::Degrade,
        FailurePolicy::Restart {
            max_restarts,
            backoff_millis,
            window_millis,
            scope,
        } => {
            let now = Instant::now();
            let window = Duration::from_millis(window_millis);
            while history
                .front()
                .is_some_and(|observed| now.duration_since(*observed) >= window)
            {
                history.pop_front();
            }
            if history.len() >= max_restarts as usize {
                FailureAction::Fatal
            } else {
                history.push_back(now);
                FailureAction::Restart {
                    count: history.len().try_into().unwrap_or(u32::MAX),
                    backoff: Duration::from_millis(backoff_millis),
                    scope,
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn schedule_restart_scope(
    topology: &ValidatedTopology,
    cancellations: &BTreeMap<ComponentId, CancellationToken>,
    recorder: &RuntimeRecorder,
    restart_tx: &mpsc::Sender<(ComponentId, Arc<dyn ComponentFactory>)>,
    id: ComponentId,
    factory: Arc<dyn ComponentFactory>,
    backoff: Duration,
    scope: SupervisionScope,
) {
    let mut members = vec![(id.clone(), factory)];
    if scope == SupervisionScope::OneForAll {
        members.extend(
            topology
                .transitive_dependents(&id)
                .into_iter()
                .map(|dependent| {
                    let factory = topology.factories[&dependent].clone();
                    (dependent, factory)
                }),
        );
    }
    for (member, factory) in members {
        if let Some(cancellation) = cancellations.get(&member) {
            cancellation.cancel();
        }
        recorder.set_component_state(member.clone(), RunningState::Restarting);
        let restart_tx = restart_tx.clone();
        tokio::spawn(async move {
            tokio::time::sleep(backoff).await;
            let _ = restart_tx.send((member, factory)).await;
        });
    }
}

fn apply_degradation(
    topology: &ValidatedTopology,
    cancellations: &BTreeMap<ComponentId, CancellationToken>,
    recorder: &RuntimeRecorder,
    id: &ComponentId,
) -> bool {
    recorder.emit(Some(id.clone()), RuntimeEvent::ComponentDegraded);
    recorder.set_component_state(id.clone(), RunningState::Degraded);
    for dependent in topology.transitive_dependents(id) {
        recorder.emit(Some(dependent.clone()), RuntimeEvent::CapabilityUnavailable);
        if let Some(cancellation) = cancellations.get(&dependent) {
            cancellation.cancel();
        }
        recorder.set_capability_unavailable(dependent);
    }
    topology.is_required_core(id)
}

fn spawn_component(
    tasks: &mut JoinSet<ComponentCompletion>,
    factory: Arc<dyn ComponentFactory>,
    cancellation: CancellationToken,
    recorder: RuntimeRecorder,
    ports: crate::component::ComponentPorts,
    ready: oneshot::Sender<()>,
) {
    let id = factory.spec().id;
    tasks.spawn(async move {
        let context =
            ComponentContext::new(id.clone(), cancellation.clone(), recorder, ports, ready);
        let outcome = AssertUnwindSafe(factory.build().run(context))
            .catch_unwind()
            .await;
        let error = match outcome {
            Ok(Ok(())) => None,
            Ok(Err(error)) => Some(error.to_string()),
            Err(_) => Some("component panicked".to_owned()),
        };
        ComponentCompletion {
            id,
            error,
            cancelled: cancellation.is_cancelled(),
        }
    });
}

fn cancel_all(
    cancellations: &BTreeMap<ComponentId, CancellationToken>,
    shutdown_order: &[ComponentId],
) {
    for id in shutdown_order {
        if let Some(cancellation) = cancellations.get(id) {
            cancellation.cancel();
        }
    }
}

async fn shutdown(
    tasks: &mut JoinSet<ComponentCompletion>,
    cancellations: &BTreeMap<ComponentId, CancellationToken>,
    shutdown_phases: Vec<Vec<ComponentId>>,
    recorder: &RuntimeRecorder,
    grace: Duration,
) -> KernelExit {
    let shutdown_order = shutdown_phases
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let drain = async {
        let mut failed = Vec::new();
        for layer in shutdown_phases {
            let snapshot = recorder.snapshot();
            let mut pending = layer
                .iter()
                .filter(|id| {
                    matches!(
                        snapshot.components.get(*id),
                        Some(
                            RunningState::Starting
                                | RunningState::Ready
                                | RunningState::Running
                                | RunningState::Restarting
                                | RunningState::Stopping
                        )
                    )
                })
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();
            for id in &layer {
                recorder.set_component_state(id.clone(), RunningState::Stopping);
                if let Some(cancellation) = cancellations.get(id) {
                    cancellation.cancel();
                }
            }
            while !pending.is_empty() {
                let Some(completion) = tasks.join_next().await else {
                    break;
                };
                if let Ok(completion) = completion {
                    pending.remove(&completion.id);
                    if completion.error.is_some() {
                        recorder.set_component_state(completion.id.clone(), RunningState::Failed);
                        failed.push(completion.id);
                    } else {
                        recorder.set_component_state(completion.id, RunningState::Stopped);
                    }
                }
            }
        }
        failed
    };
    match timeout(grace, drain).await {
        Ok(failed) if failed.is_empty() => KernelExit::Clean,
        Ok(components) => KernelExit::ShutdownFailed { components },
        Err(_) => {
            let aborted = shutdown_order
                .iter()
                .filter(|id| {
                    recorder.snapshot().components.get(*id) != Some(&RunningState::Stopped)
                })
                .cloned()
                .collect();
            tasks.abort_all();
            KernelExit::ShutdownDeadlineExceeded { aborted }
        }
    }
}

fn shutdown_phases(topology: &ValidatedTopology) -> Vec<Vec<ComponentId>> {
    let mut layers = topology.dependency_layers();
    layers.reverse();
    let ordered = layers.into_iter().flatten().collect::<Vec<_>>();
    let mut phases = Vec::new();
    for role in [
        LifecycleRole::Ingress,
        LifecycleRole::Workload,
        LifecycleRole::Checkpoint,
        LifecycleRole::Telemetry,
        LifecycleRole::Egress,
    ] {
        let members = ordered
            .iter()
            .filter(|id| topology.factories[*id].lifecycle_role() == role)
            .cloned()
            .collect::<Vec<_>>();
        if !members.is_empty() {
            phases.push(members);
        }
    }
    phases
}
