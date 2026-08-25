use std::{
    collections::BTreeMap,
    fmt,
    marker::PhantomData,
    sync::{Arc, Mutex as StdMutex},
    time::Duration,
};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
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
    pub(crate) terminal_shutdown: CancellationToken,
    pub recorder: RuntimeRecorder,
    ports: ComponentPorts,
    ready: Option<oneshot::Sender<()>>,
    health: mpsc::Sender<(ComponentId, u64, ComponentHealthSignal)>,
    incarnation: u64,
}

pub(crate) struct ComponentLifecycle {
    pub(crate) cancellation: CancellationToken,
    pub(crate) terminal_shutdown: CancellationToken,
    pub(crate) incarnation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ComponentHealthSignal {
    Degraded,
}

impl ComponentContext {
    pub(crate) fn new(
        id: ComponentId,
        lifecycle: ComponentLifecycle,
        recorder: RuntimeRecorder,
        ports: ComponentPorts,
        ready: oneshot::Sender<()>,
        health: mpsc::Sender<(ComponentId, u64, ComponentHealthSignal)>,
    ) -> Self {
        let ComponentLifecycle {
            cancellation,
            terminal_shutdown,
            incarnation,
        } = lifecycle;
        Self {
            id,
            cancellation,
            terminal_shutdown,
            recorder,
            ports,
            ready: Some(ready),
            health,
            incarnation,
        }
    }

    pub fn ready(&mut self) {
        if let Some(ready) = self.ready.take() {
            let _ = ready.send(());
        }
    }

    pub async fn degraded(&self) -> Result<(), ComponentError> {
        self.health
            .send((
                self.id.clone(),
                self.incarnation,
                ComponentHealthSignal::Degraded,
            ))
            .await
            .map_err(|_| ComponentError::new("supervisor health channel closed"))
    }

    pub async fn send<T>(&self, port: &str, value: &T) -> Result<(), PortAccessError>
    where
        T: PortProtocol + Serialize,
    {
        self.ports.send(port, value).await
    }

    pub async fn recv<T>(&self, port: &str) -> Result<Option<T>, PortAccessError>
    where
        T: PortProtocol + DeserializeOwned,
    {
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
    #[error("port {port} protocol mismatch: expected {expected}, requested {requested}")]
    ProtocolMismatch {
        port: String,
        expected: String,
        requested: String,
    },
    #[error("port {port} payload failed protocol decoding")]
    Decode { port: String },
}

#[derive(Clone, Debug)]
struct WireMessage {
    protocol: String,
    payload: Vec<u8>,
}

#[derive(Clone)]
pub struct ExternalInput<T> {
    spec: PortSpec,
    sender: Arc<StdMutex<Option<BoundedSender<WireMessage>>>>,
    marker: PhantomData<T>,
}

impl<T> ExternalInput<T>
where
    T: PortProtocol + Serialize,
{
    pub fn new(name: impl Into<String>, capacity: usize, full_policy: ChannelFullPolicy) -> Self {
        Self {
            spec: PortSpec::bounded(name, T::PROTOCOL, capacity, full_policy),
            sender: Arc::new(StdMutex::new(None)),
            marker: PhantomData,
        }
    }

    pub fn spec(&self) -> PortSpec {
        self.spec.clone()
    }

    pub async fn send(&self, value: &T) -> Result<(), PortAccessError> {
        let sender = self
            .sender
            .lock()
            .map_err(|_| PortAccessError::Closed(self.spec.name.clone()))?
            .clone()
            .ok_or_else(|| PortAccessError::Closed(self.spec.name.clone()))?;
        let payload = serde_json::to_vec(value).map_err(|_| PortAccessError::Decode {
            port: self.spec.name.clone(),
        })?;
        sender
            .send(WireMessage {
                protocol: T::PROTOCOL.to_owned(),
                payload,
            })
            .await
            .map_err(|error| match error {
                SendError::Full => PortAccessError::Full(self.spec.name.clone()),
                SendError::Closed => PortAccessError::Closed(self.spec.name.clone()),
            })
    }

    pub(crate) fn binding(&self) -> ExternalInputBinding {
        ExternalInputBinding {
            spec: self.spec.clone(),
            sender: self.sender.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ExternalInputBinding {
    spec: PortSpec,
    sender: Arc<StdMutex<Option<BoundedSender<WireMessage>>>>,
}

impl ExternalInputBinding {
    pub fn spec(&self) -> &PortSpec {
        &self.spec
    }

    fn bind(&self, sender: BoundedSender<WireMessage>) {
        *self.sender.lock().expect("external input binding poisoned") = Some(sender);
    }

    fn unbind(&self) {
        *self.sender.lock().expect("external input binding poisoned") = None;
    }
}

#[derive(Clone)]
struct OutputPort {
    protocol: String,
    senders: Arc<RwLock<BTreeMap<ComponentId, BoundedSender<WireMessage>>>>,
}

#[derive(Clone)]
struct InputPort {
    protocol: String,
    receiver: Arc<Mutex<BoundedReceiver<WireMessage>>>,
}

#[derive(Clone, Default)]
pub struct ComponentPorts {
    outputs: BTreeMap<String, OutputPort>,
    inputs: BTreeMap<String, InputPort>,
}

impl ComponentPorts {
    async fn send<T>(&self, port: &str, value: &T) -> Result<(), PortAccessError>
    where
        T: PortProtocol + Serialize,
    {
        let outputs = self
            .outputs
            .get(port)
            .ok_or_else(|| PortAccessError::UndeclaredOutput(port.to_owned()))?;
        if outputs.protocol != T::PROTOCOL {
            return Err(PortAccessError::ProtocolMismatch {
                port: port.to_owned(),
                expected: outputs.protocol.clone(),
                requested: T::PROTOCOL.to_owned(),
            });
        }
        let payload = serde_json::to_vec(value).map_err(|_| PortAccessError::Decode {
            port: port.to_owned(),
        })?;
        let senders = outputs.senders.read().await;
        if senders.is_empty() {
            return Err(PortAccessError::Closed(port.to_owned()));
        }
        for output in senders.values() {
            output
                .send(WireMessage {
                    protocol: T::PROTOCOL.to_owned(),
                    payload: payload.clone(),
                })
                .await
                .map_err(|error| match error {
                    SendError::Full => PortAccessError::Full(port.to_owned()),
                    SendError::Closed => PortAccessError::Closed(port.to_owned()),
                })?;
        }
        Ok(())
    }

    async fn recv<T>(&self, port: &str) -> Result<Option<T>, PortAccessError>
    where
        T: PortProtocol + DeserializeOwned,
    {
        let input = self
            .inputs
            .get(port)
            .ok_or_else(|| PortAccessError::UndeclaredInput(port.to_owned()))?;
        if input.protocol != T::PROTOCOL {
            return Err(PortAccessError::ProtocolMismatch {
                port: port.to_owned(),
                expected: input.protocol.clone(),
                requested: T::PROTOCOL.to_owned(),
            });
        }
        let Some(message) = input.receiver.lock().await.recv().await else {
            return Ok(None);
        };
        if message.protocol != T::PROTOCOL {
            return Err(PortAccessError::ProtocolMismatch {
                port: port.to_owned(),
                expected: T::PROTOCOL.to_owned(),
                requested: message.protocol,
            });
        }
        serde_json::from_slice(&message.payload)
            .map(Some)
            .map_err(|_| PortAccessError::Decode {
                port: port.to_owned(),
            })
    }
}

pub(crate) struct RuntimePortRegistry {
    components: StdMutex<BTreeMap<ComponentId, ComponentPorts>>,
    routes: Vec<ValidatedPortRoute>,
    external_inputs: BTreeMap<(ComponentId, String), ExternalInputBinding>,
    recorder: RuntimeRecorder,
}

impl RuntimePortRegistry {
    pub(crate) fn new(
        component_ids: impl IntoIterator<Item = ComponentId>,
        routes: &[ValidatedPortRoute],
        external_inputs: &BTreeMap<(ComponentId, String), ExternalInputBinding>,
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
            if route.external {
                external_inputs[&(route.consumer.clone(), route.spec.name.clone())].bind(sender);
            } else {
                let output = components
                    .get_mut(&route.provider)
                    .expect("validated provider exists")
                    .outputs
                    .entry(route.spec.name.clone())
                    .or_insert_with(|| OutputPort {
                        protocol: route.spec.protocol.clone(),
                        senders: Arc::new(RwLock::new(BTreeMap::new())),
                    });
                Arc::get_mut(&mut output.senders)
                    .expect("registry endpoints are not cloned during construction")
                    .get_mut()
                    .insert(route.consumer.clone(), sender);
            }
            components
                .get_mut(&route.consumer)
                .expect("validated consumer exists")
                .inputs
                .insert(
                    route.spec.name.clone(),
                    InputPort {
                        protocol: route.spec.protocol.clone(),
                        receiver: Arc::new(Mutex::new(receiver)),
                    },
                );
        }
        Self {
            components: StdMutex::new(components),
            routes: routes.to_vec(),
            external_inputs: external_inputs.clone(),
            recorder: recorder.clone(),
        }
    }

    pub(crate) fn for_component(&self, id: &ComponentId) -> ComponentPorts {
        self.components
            .lock()
            .expect("runtime port registry poisoned")
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    /// Drop the kernel's live consumer endpoint when a component exits. This
    /// makes producer sends observe closure instead of being kept artificially
    /// alive by registry ownership.
    pub(crate) async fn close_component(&self, id: &ComponentId) {
        for route in self.routes.iter().filter(|route| &route.consumer == id) {
            if route.external {
                self.external_inputs[&(route.consumer.clone(), route.spec.name.clone())].unbind();
                continue;
            }
            let output = self
                .components
                .lock()
                .expect("runtime port registry poisoned")
                .get(&route.provider)
                .and_then(|ports| ports.outputs.get(&route.spec.name))
                .cloned();
            if let Some(output) = output {
                output.senders.write().await.remove(id);
            }
        }
    }

    /// Recreate only the routes consumed by a restarting component and replace
    /// the corresponding provider handles atomically in the shared endpoint
    /// table. Existing provider contexts therefore target the new incarnation.
    pub(crate) async fn rebind_component(&self, id: &ComponentId) {
        for route in self.routes.iter().filter(|route| &route.consumer == id) {
            let (sender, receiver) = channel(route.spec.capacity, route.spec.full_policy);
            self.recorder.set_queue_health(
                format!("{}:{}->{}", route.provider, route.spec.name, route.consumer),
                &sender.metrics(),
            );
            let new_receiver = Arc::new(Mutex::new(receiver));
            if let Some(input) = self
                .components
                .lock()
                .expect("runtime port registry poisoned")
                .get_mut(&route.consumer)
                .and_then(|ports| ports.inputs.get_mut(&route.spec.name))
            {
                input.receiver = new_receiver;
            }
            if route.external {
                self.external_inputs[&(route.consumer.clone(), route.spec.name.clone())]
                    .bind(sender);
            } else {
                let output = self
                    .components
                    .lock()
                    .expect("runtime port registry poisoned")
                    .get(&route.provider)
                    .and_then(|ports| ports.outputs.get(&route.spec.name))
                    .cloned();
                if let Some(output) = output {
                    output
                        .senders
                        .write()
                        .await
                        .insert(route.consumer.clone(), sender);
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::*;
    use crate::{ChannelFullPolicy, PortSpec};

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestMessage(u64);

    impl PortProtocol for TestMessage {
        const PROTOCOL: &'static str = "adl.runtime.test.message.v1";
    }

    #[tokio::test]
    async fn consumer_exit_closes_route_and_restart_rebinds_existing_producer() {
        let producer = ComponentId::new("producer");
        let consumer = ComponentId::new("consumer");
        let route = ValidatedPortRoute {
            provider: producer.clone(),
            consumer: consumer.clone(),
            spec: PortSpec::bounded(
                "events",
                TestMessage::PROTOCOL,
                2,
                ChannelFullPolicy::Reject,
            ),
            external: false,
        };
        let registry = RuntimePortRegistry::new(
            [producer.clone(), consumer.clone()],
            &[route],
            &BTreeMap::new(),
            &RuntimeRecorder::new(8),
        );
        let producer_ports = registry.for_component(&producer);
        let consumer_ports = registry.for_component(&consumer);

        producer_ports
            .send("events", &TestMessage(1))
            .await
            .unwrap();
        assert_eq!(
            consumer_ports.recv::<TestMessage>("events").await.unwrap(),
            Some(TestMessage(1))
        );

        registry.close_component(&consumer).await;
        assert_eq!(
            producer_ports.send("events", &TestMessage(2)).await,
            Err(PortAccessError::Closed("events".to_owned()))
        );

        registry.rebind_component(&consumer).await;
        let restarted_consumer_ports = registry.for_component(&consumer);
        producer_ports
            .send("events", &TestMessage(3))
            .await
            .unwrap();
        assert_eq!(
            consumer_ports.recv::<TestMessage>("events").await.unwrap(),
            None,
            "the old incarnation must not observe the rebound receiver"
        );
        assert_eq!(
            restarted_consumer_ports
                .recv::<TestMessage>("events")
                .await
                .unwrap(),
            Some(TestMessage(3))
        );
    }

    #[tokio::test]
    async fn external_input_enforces_its_declared_capacity_and_policy() {
        let input = ExternalInput::<TestMessage>::new("external", 1, ChannelFullPolicy::Reject);
        let (sender, _receiver) = channel(1, ChannelFullPolicy::Reject);
        input.binding().bind(sender);
        input.send(&TestMessage(1)).await.unwrap();
        assert_eq!(
            input.send(&TestMessage(2)).await,
            Err(PortAccessError::Full("external".to_owned()))
        );
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

    fn external_inputs(&self) -> Vec<ExternalInputBinding> {
        Vec::new()
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

    fn external_inputs(&self) -> Vec<ExternalInputBinding> {
        (**self).external_inputs()
    }
}
