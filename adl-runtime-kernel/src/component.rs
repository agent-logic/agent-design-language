use std::{collections::BTreeMap, fmt, sync::Arc, time::Duration};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex};
use tokio_util::sync::CancellationToken;

use crate::{
    channel::{channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, SendError},
    telemetry::RuntimeRecorder,
    ValidatedPortRoute,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentId(String);

impl ComponentId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<&str> for ComponentId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PortSpec {
    pub name: String,
    pub protocol: String,
    pub capacity: usize,
    pub full_policy: ChannelFullPolicy,
}

/// Stable, versioned wire identity for a message admitted to a Runtime port.
pub trait PortProtocol {
    const PROTOCOL: &'static str;
}

impl PortSpec {
    /// Compatibility constructor for existing v1 assemblies. The resulting
    /// protocol identity is stable across Rust refactors and intentionally does
    /// not expose `type_name::<T>()` as contract authority.
    pub fn typed<T: 'static>(name: impl Into<String>) -> Self {
        let name = name.into();
        let primitive = std::any::TypeId::of::<T>();
        let protocol = if primitive == std::any::TypeId::of::<u8>() {
            "adl.scalar.u8.v1".to_owned()
        } else if primitive == std::any::TypeId::of::<u16>() {
            "adl.scalar.u16.v1".to_owned()
        } else if primitive == std::any::TypeId::of::<u64>() {
            "adl.scalar.u64.v1".to_owned()
        } else if primitive == std::any::TypeId::of::<String>() {
            "adl.scalar.string.v1".to_owned()
        } else {
            format!("adl.runtime.port.{name}.v1")
        };
        Self {
            protocol,
            name,
            capacity: 64,
            full_policy: ChannelFullPolicy::Block,
        }
    }

    pub fn protocol<T: PortProtocol>(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            protocol: T::PROTOCOL.to_owned(),
            capacity: 64,
            full_policy: ChannelFullPolicy::Block,
        }
    }

    pub fn bounded(
        name: impl Into<String>,
        protocol: impl Into<String>,
        capacity: usize,
        full_policy: ChannelFullPolicy,
    ) -> Self {
        Self {
            name: name.into(),
            protocol: protocol.into(),
            capacity,
            full_policy,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupervisionScope {
    OneForOne,
    OneForAll,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FailurePolicy {
    Fatal,
    Degrade,
    Restart {
        max_restarts: u32,
        backoff_millis: u64,
        window_millis: u64,
        scope: SupervisionScope,
    },
}

impl FailurePolicy {
    pub fn restart(max_restarts: u32, backoff: Duration) -> Self {
        Self::Restart {
            max_restarts,
            backoff_millis: backoff.as_millis().try_into().unwrap_or(u64::MAX),
            window_millis: Duration::from_secs(60).as_millis() as u64,
            scope: SupervisionScope::OneForOne,
        }
    }

    pub fn restart_windowed(
        max_restarts: u32,
        backoff: Duration,
        window: Duration,
        scope: SupervisionScope,
    ) -> Self {
        Self::Restart {
            max_restarts,
            backoff_millis: backoff.as_millis().try_into().unwrap_or(u64::MAX),
            window_millis: window.as_millis().try_into().unwrap_or(u64::MAX),
            scope,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComponentSpec {
    pub id: ComponentId,
    pub dependencies: Vec<ComponentId>,
    pub inputs: Vec<PortSpec>,
    pub outputs: Vec<PortSpec>,
    pub failure_policy: FailurePolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunningState {
    Starting,
    Ready,
    Running,
    Restarting,
    Degraded,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleRole {
    Ingress,
    Workload,
    Checkpoint,
    Telemetry,
    Egress,
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct ComponentError {
    message: String,
}

impl ComponentError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub struct ComponentContext {
    pub id: ComponentId,
    pub cancellation: CancellationToken,
    pub recorder: RuntimeRecorder,
    ports: ComponentPorts,
    ready: Option<oneshot::Sender<()>>,
}

impl ComponentContext {
    pub(crate) fn new(
        id: ComponentId,
        cancellation: CancellationToken,
        recorder: RuntimeRecorder,
        ports: ComponentPorts,
        ready: oneshot::Sender<()>,
    ) -> Self {
        Self {
            id,
            cancellation,
            recorder,
            ports,
            ready: Some(ready),
        }
    }

    pub fn ready(&mut self) {
        if let Some(ready) = self.ready.take() {
            let _ = ready.send(());
        }
    }

    pub fn degraded(&self) {
        self.recorder
            .set_component_state(self.id.clone(), RunningState::Degraded);
        self.recorder.emit(
            Some(self.id.clone()),
            crate::RuntimeEvent::ComponentDegraded,
        );
    }

    pub fn running(&self) {
        self.recorder
            .set_component_state(self.id.clone(), RunningState::Running);
    }

    pub async fn send(&self, port: &str, value: serde_json::Value) -> Result<(), PortAccessError> {
        self.ports.send(port, value).await
    }

    pub async fn recv(&self, port: &str) -> Result<Option<serde_json::Value>, PortAccessError> {
        self.ports.recv(port).await
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum PortAccessError {
    #[error("component has no declared output port: {0}")]
    UndeclaredOutput(String),
    #[error("component has no declared input port: {0}")]
    UndeclaredInput(String),
    #[error("declared output port is closed: {0}")]
    Closed(String),
    #[error("declared output port is full: {0}")]
    Full(String),
}

#[derive(Clone, Default)]
pub struct ComponentPorts {
    outputs: BTreeMap<String, Vec<BoundedSender<serde_json::Value>>>,
    inputs: BTreeMap<String, Arc<Mutex<BoundedReceiver<serde_json::Value>>>>,
}

impl ComponentPorts {
    async fn send(&self, port: &str, value: serde_json::Value) -> Result<(), PortAccessError> {
        let outputs = self
            .outputs
            .get(port)
            .ok_or_else(|| PortAccessError::UndeclaredOutput(port.to_owned()))?;
        for output in outputs {
            output
                .send(value.clone())
                .await
                .map_err(|error| match error {
                    SendError::Full => PortAccessError::Full(port.to_owned()),
                    SendError::Closed => PortAccessError::Closed(port.to_owned()),
                })?;
        }
        Ok(())
    }

    async fn recv(&self, port: &str) -> Result<Option<serde_json::Value>, PortAccessError> {
        let input = self
            .inputs
            .get(port)
            .ok_or_else(|| PortAccessError::UndeclaredInput(port.to_owned()))?;
        Ok(input.lock().await.recv().await)
    }
}

pub(crate) struct RuntimePortRegistry {
    components: BTreeMap<ComponentId, ComponentPorts>,
}

impl RuntimePortRegistry {
    pub(crate) fn new(
        component_ids: impl IntoIterator<Item = ComponentId>,
        routes: &[ValidatedPortRoute],
        recorder: &RuntimeRecorder,
    ) -> Self {
        let mut components = component_ids
            .into_iter()
            .map(|id| (id, ComponentPorts::default()))
            .collect::<BTreeMap<_, _>>();
        for route in routes {
            let (sender, receiver) = channel(route.spec.capacity, route.spec.full_policy);
            recorder.set_queue_health(
                format!("{}:{}->{}", route.provider, route.spec.name, route.consumer),
                &sender.metrics(),
            );
            components
                .get_mut(&route.provider)
                .expect("validated provider exists")
                .outputs
                .entry(route.spec.name.clone())
                .or_default()
                .push(sender);
            components
                .get_mut(&route.consumer)
                .expect("validated consumer exists")
                .inputs
                .insert(route.spec.name.clone(), Arc::new(Mutex::new(receiver)));
        }
        Self { components }
    }

    pub(crate) fn for_component(&self, id: &ComponentId) -> ComponentPorts {
        self.components.get(id).cloned().unwrap_or_default()
    }
}

#[async_trait]
pub trait Component: Send + 'static {
    async fn run(self: Box<Self>, context: ComponentContext) -> Result<(), ComponentError>;
}

pub trait ComponentFactory: Send + Sync + 'static {
    fn spec(&self) -> ComponentSpec;
    fn build(&self) -> Box<dyn Component>;

    fn lifecycle_role(&self) -> LifecycleRole {
        LifecycleRole::Workload
    }

    fn required_core(&self) -> bool {
        false
    }
}

impl<T> ComponentFactory for Arc<T>
where
    T: ComponentFactory + ?Sized,
{
    fn spec(&self) -> ComponentSpec {
        (**self).spec()
    }

    fn build(&self) -> Box<dyn Component> {
        (**self).build()
    }

    fn lifecycle_role(&self) -> LifecycleRole {
        (**self).lifecycle_role()
    }

    fn required_core(&self) -> bool {
        (**self).required_core()
    }
}
