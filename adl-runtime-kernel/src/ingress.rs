use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex as AsyncMutex, Notify};
use tokio_util::sync::CancellationToken;

use crate::{
    channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, Component, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, FailurePolicy, OperationError,
    OperationRequest, OperationalFactory, RuntimeEvent, RuntimeRecorder, SendError,
    OPERATION_REQUEST_SCHEMA,
};

pub const DOMAIN_WORK_SCHEMA: &str = "adl.runtime.domain_work.v1";
pub const DOMAIN_RESULT_SCHEMA: &str = "adl.runtime.domain_result.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainWork {
    pub schema: String,
    pub work_id: String,
    pub kind: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DomainResult {
    pub schema: String,
    pub work_id: String,
    pub accepted_sequence: u64,
    pub result_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_output: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct IngressSnapshot {
    pub accepted_through: u64,
    pub completed: BTreeMap<String, DomainResult>,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IngressError {
    #[error("domain work is invalid")]
    Invalid,
    #[error("domain work id was reused with a different payload")]
    Conflict,
    #[error("canonical ingress is saturated")]
    Saturated,
    #[error("canonical ingress is closed")]
    Closed,
    #[error("domain work kind is not allowlisted")]
    UnsupportedKind,
    #[error("domain work execution failed")]
    ExecutionFailed,
    #[error("canonical ingress drain timed out")]
    DrainTimeout,
}

struct Envelope {
    work: DomainWork,
    correlation_id: String,
    cancellation: CancellationToken,
    reply: oneshot::Sender<Result<DomainResult, IngressError>>,
}

#[derive(Clone)]
pub struct CanonicalIngress {
    sender: BoundedSender<Envelope>,
    receiver: Arc<AsyncMutex<BoundedReceiver<Envelope>>>,
    state: Arc<Mutex<IngressSnapshot>>,
    recorder: RuntimeRecorder,
    admission: Arc<Mutex<AdmissionState>>,
    drained: Arc<Notify>,
    dispatchers: Arc<BTreeMap<String, OperationalFactory>>,
}

struct AdmissionState {
    open: bool,
    active: usize,
}

struct AdmissionLease(Arc<Mutex<AdmissionState>>, Arc<Notify>);

impl Drop for AdmissionLease {
    fn drop(&mut self) {
        let mut admission = self.0.lock().expect("ingress admission mutex poisoned");
        admission.active = admission.active.saturating_sub(1);
        if admission.active == 0 {
            self.1.notify_one();
        }
    }
}

impl CanonicalIngress {
    pub fn new(
        capacity: usize,
        recorder: RuntimeRecorder,
        dispatchers: BTreeMap<String, OperationalFactory>,
    ) -> Self {
        let (sender, receiver) = channel(capacity, ChannelFullPolicy::Reject);
        recorder.set_queue_health("canonical_ingress", &sender.metrics());
        Self {
            sender,
            receiver: Arc::new(AsyncMutex::new(receiver)),
            state: Arc::new(Mutex::new(IngressSnapshot::default())),
            recorder,
            admission: Arc::new(Mutex::new(AdmissionState {
                open: true,
                active: 0,
            })),
            drained: Arc::new(Notify::new()),
            dispatchers: Arc::new(dispatchers),
        }
    }

    pub async fn submit(
        &self,
        work: DomainWork,
        correlation_id: String,
    ) -> Result<DomainResult, IngressError> {
        self.submit_with_cancellation(work, correlation_id, CancellationToken::new())
            .await
    }

    pub async fn submit_with_cancellation(
        &self,
        work: DomainWork,
        correlation_id: String,
        cancellation: CancellationToken,
    ) -> Result<DomainResult, IngressError> {
        validate(&work)?;
        let _lease = self.begin_admission()?;
        let (reply, result) = oneshot::channel();
        self.sender
            .send(Envelope {
                work,
                correlation_id,
                cancellation,
                reply,
            })
            .await
            .map_err(|error| match error {
                SendError::Full => IngressError::Saturated,
                SendError::Closed => IngressError::Closed,
            })?;
        self.recorder
            .set_queue_health("canonical_ingress", &self.sender.metrics());
        result.await.map_err(|_| IngressError::Closed)?
    }

    pub fn snapshot(&self) -> IngressSnapshot {
        self.state
            .lock()
            .expect("ingress state mutex poisoned")
            .clone()
    }

    pub fn restore(&self, snapshot: IngressSnapshot) {
        *self.state.lock().expect("ingress state mutex poisoned") = snapshot;
    }

    pub fn close(&self) {
        let mut admission = self
            .admission
            .lock()
            .expect("ingress admission mutex poisoned");
        admission.open = false;
    }

    pub async fn close_and_drain(&self, deadline: Duration) -> Result<(), IngressError> {
        self.close();
        tokio::time::timeout(deadline, async {
            loop {
                let drained = self.drained.notified();
                let active = self
                    .admission
                    .lock()
                    .expect("ingress admission mutex poisoned")
                    .active;
                if active == 0 {
                    return;
                }
                drained.await;
            }
        })
        .await
        .map_err(|_| IngressError::DrainTimeout)
    }

    pub fn reopen(&self) {
        self.admission
            .lock()
            .expect("ingress admission mutex poisoned")
            .open = true;
    }

    pub fn admission_is_open(&self) -> bool {
        self.admission
            .lock()
            .expect("ingress admission mutex poisoned")
            .open
    }

    fn begin_admission(&self) -> Result<AdmissionLease, IngressError> {
        let mut admission = self
            .admission
            .lock()
            .expect("ingress admission mutex poisoned");
        if !admission.open {
            return Err(IngressError::Closed);
        }
        admission.active = admission.active.saturating_add(1);
        Ok(AdmissionLease(self.admission.clone(), self.drained.clone()))
    }

    async fn dispatch(
        &self,
        work: &DomainWork,
        cancellation: CancellationToken,
    ) -> Result<DomainResult, IngressError> {
        let dispatcher = self
            .dispatchers
            .get(&work.kind)
            .ok_or(IngressError::UnsupportedKind)?;
        let operation = dispatcher
            .submit_with_cancellation(
                OperationRequest {
                    schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                    request_id: work.work_id.clone(),
                    idempotency_key: work.work_id.clone(),
                    principal: "canonical-ingress".to_owned(),
                    payload: work.payload.clone(),
                    permit: None,
                },
                cancellation,
            )
            .await
            .map_err(|error| match error {
                OperationError::InvalidRequest => IngressError::Conflict,
                _ => IngressError::ExecutionFailed,
            })?;
        apply(&self.state, work, &operation)
    }
}

#[async_trait]
impl Component for CanonicalIngress {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        loop {
            tokio::select! {
                _ = context.cancellation.cancelled() => return Ok(()),
                envelope = async { self.receiver.lock().await.recv().await } => {
                    let Some(envelope) = envelope else { return Ok(()); };
                    self.recorder.set_queue_health("canonical_ingress", &self.sender.metrics());
                    let result = self
                        .dispatch(&envelope.work, envelope.cancellation)
                        .await;
                    if result.is_ok() {
                        self.recorder.emit_correlated(Some(ComponentId::new("canonical_ingress")),
                            RuntimeEvent::DomainWorkCompleted, Some(&envelope.correlation_id));
                    }
                    let _ = envelope.reply.send(result);
                }
            }
        }
    }
}

impl ComponentFactory for CanonicalIngress {
    fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            id: ComponentId::new("canonical_ingress"),
            dependencies: vec![],
            inputs: vec![],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }
    fn build(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }

    fn lifecycle_role(&self) -> crate::LifecycleRole {
        crate::LifecycleRole::Ingress
    }

    fn required_core(&self) -> bool {
        true
    }
}

fn validate(work: &DomainWork) -> Result<(), IngressError> {
    let safe = |value: &str| {
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'_' | b'-'))
    };
    if work.schema != DOMAIN_WORK_SCHEMA
        || !safe(&work.work_id)
        || !safe(&work.kind)
        || work.payload.is_empty()
        || work.payload.len() > 1_048_576
    {
        return Err(IngressError::Invalid);
    }
    Ok(())
}

fn apply(
    state: &Mutex<IngressSnapshot>,
    work: &DomainWork,
    operation: &crate::OperationResult,
) -> Result<DomainResult, IngressError> {
    let public_output = project_public_output(work, operation)?;
    let result_hash =
        blake3::hash(&serde_json::to_vec(&(work, operation)).map_err(|_| IngressError::Invalid)?)
            .to_hex()
            .to_string();
    let mut state = state.lock().expect("ingress state mutex poisoned");
    if let Some(existing) = state.completed.get(&work.work_id) {
        return (existing.result_hash == result_hash)
            .then(|| existing.clone())
            .ok_or(IngressError::Conflict);
    }
    state.accepted_through = state.accepted_through.saturating_add(1);
    let result = DomainResult {
        schema: DOMAIN_RESULT_SCHEMA.to_owned(),
        work_id: work.work_id.clone(),
        accepted_sequence: state.accepted_through,
        result_hash,
        public_output,
    };
    state.completed.insert(work.work_id.clone(), result.clone());
    Ok(result)
}

fn project_public_output(
    work: &DomainWork,
    operation: &crate::OperationResult,
) -> Result<Option<serde_json::Value>, IngressError> {
    let Ok(command) = serde_json::from_slice::<serde_json::Value>(&work.payload) else {
        return Ok(None);
    };
    let Some(tasks) = command.get("tasks").and_then(serde_json::Value::as_array) else {
        return Ok(None);
    };
    if command.get("schema").and_then(serde_json::Value::as_str)
        != Some("adl.runtime.local_agent_work.v1")
        || tasks.len() != 1
        || tasks[0].get("op").and_then(serde_json::Value::as_str) != Some("conversation_message")
    {
        return Ok(None);
    }
    let execution: serde_json::Value =
        serde_json::from_slice(&operation.payload).map_err(|_| IngressError::ExecutionFailed)?;
    if execution.get("schema").and_then(serde_json::Value::as_str)
        != Some("adl.runtime.local_agent_execution.v1")
    {
        return Err(IngressError::ExecutionFailed);
    }
    let output = execution
        .get("outputs")
        .and_then(serde_json::Value::as_array)
        .and_then(|outputs| outputs.first())
        .and_then(|entry| entry.get("output"))
        .ok_or(IngressError::ExecutionFailed)?;
    let recipient_id = output
        .get("recipient_id")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 128)
        .ok_or(IngressError::ExecutionFailed)?;
    let requested_recipient_id = tasks[0]
        .get("recipient_id")
        .and_then(serde_json::Value::as_str)
        .ok_or(IngressError::ExecutionFailed)?;
    if recipient_id != requested_recipient_id {
        return Err(IngressError::ExecutionFailed);
    }
    let message = output
        .get("message")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 4_096)
        .ok_or(IngressError::ExecutionFailed)?;
    Ok(Some(serde_json::json!({
        "schema": "adl.runtime.conversation_reply.v1",
        "recipient_id": recipient_id,
        "message": message,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_and_submit_admission_are_one_atomic_transition() {
        let ingress = CanonicalIngress::new(1, RuntimeRecorder::new(4), BTreeMap::new());
        let lease = ingress.begin_admission().unwrap();
        ingress.close();
        assert!(matches!(
            ingress.begin_admission(),
            Err(IngressError::Closed)
        ));
        drop(lease);
        ingress.reopen();
        assert!(ingress.begin_admission().is_ok());
    }
}
