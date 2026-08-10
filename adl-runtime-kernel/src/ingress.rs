use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex as AsyncMutex, Notify};

use crate::{
    channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, Component, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, FailurePolicy, OperationError,
    OperationRequest, OperationalFactory, RuntimeEvent, RuntimeRecorder, SendError,
    OPERATION_REQUEST_SCHEMA,
};

pub const DOMAIN_WORK_SCHEMA: &str = "adl.runtime.domain_work.v1";
pub const DOMAIN_RESULT_SCHEMA: &str = "adl.runtime.domain_result.v1";
pub const LOCAL_AGENT_WORK_SCHEMA: &str = "adl.runtime.local_agent_work.v1";
pub const LAYER8_MESSAGE_TASK_OP: &str = "layer8_message";
pub const LAYER8_AGENT_RESPONSE_SCHEMA: &str = "adl.runtime.layer8.agent_response.v1";

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
        validate(&work)?;
        let _lease = self.begin_admission()?;
        let (reply, result) = oneshot::channel();
        self.sender
            .send(Envelope {
                work,
                correlation_id,
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

    async fn dispatch(&self, work: &DomainWork) -> Result<DomainResult, IngressError> {
        let dispatcher = self
            .dispatchers
            .get(&work.kind)
            .ok_or(IngressError::UnsupportedKind)?;
        let operation = dispatcher
            .submit(OperationRequest {
                schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                request_id: work.work_id.clone(),
                idempotency_key: work.work_id.clone(),
                principal: "canonical-ingress".to_owned(),
                payload: work.payload.clone(),
                permit: None,
            })
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
                    let result = self.dispatch(&envelope.work).await;
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
        public_output: public_agent_output(work, operation),
    };
    state.completed.insert(work.work_id.clone(), result.clone());
    Ok(result)
}

fn public_agent_output(
    work: &DomainWork,
    operation: &crate::OperationResult,
) -> Option<serde_json::Value> {
    if work.kind != "agent" {
        return None;
    }
    let execution: serde_json::Value = serde_json::from_slice(&operation.payload).ok()?;
    if execution.get("schema")?.as_str()? != "adl.runtime.local_agent_execution.v1" {
        return None;
    }
    let outputs = execution.get("outputs")?.as_array()?;
    if outputs.len() != 1 {
        return None;
    }
    let tasks: serde_json::Value = serde_json::from_slice(&work.payload).ok()?;
    let tasks = tasks.get("tasks")?.as_array()?;
    if tasks.len() != 1 || tasks[0].get("op")?.as_str()? != LAYER8_MESSAGE_TASK_OP {
        return None;
    }
    let expected_recipient = tasks[0].get("recipient_id")?.as_str()?;
    let expected_correlation = tasks[0].get("correlation_id")?.as_str()?;
    let output: Layer8PublicResponse =
        serde_json::from_value(outputs.first()?.get("output")?.clone()).ok()?;
    if output.schema != LAYER8_AGENT_RESPONSE_SCHEMA
        || output.recipient_id != expected_recipient
        || output.correlation_id != expected_correlation
        || output.status != "received"
        || output.message.is_empty()
        || output.message.len() > 512
        || output
            .message
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
    {
        return None;
    }
    Some(serde_json::json!({
        "schema": LAYER8_AGENT_RESPONSE_SCHEMA,
        "recipient_id": output.recipient_id,
        "correlation_id": output.correlation_id,
        "status": "received",
        "message": output.message
    }))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Layer8PublicResponse {
    schema: String,
    recipient_id: String,
    correlation_id: String,
    status: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer8_work() -> DomainWork {
        DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: "layer8-projection".to_owned(),
            kind: "agent".to_owned(),
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{
                    "op": LAYER8_MESSAGE_TASK_OP,
                    "sender": "layer8-operator",
                    "recipient_id": "agent-0001",
                    "correlation_id": "projection-correlation",
                    "content": "Hello"
                }]
            }))
            .unwrap(),
        }
    }

    fn operation_with_output(output: serde_json::Value) -> crate::OperationResult {
        crate::OperationResult {
            schema: crate::OPERATION_RESULT_SCHEMA.to_owned(),
            request_id: "layer8-projection".to_owned(),
            adapter: crate::AdapterKind::Agent,
            attempts: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": "adl.runtime.local_agent_execution.v1",
                "outputs": [{"unit": 0, "output": output}]
            }))
            .unwrap(),
        }
    }

    #[test]
    fn public_layer8_projection_rejects_extra_or_mismatched_executor_fields() {
        let valid = serde_json::json!({
            "schema": LAYER8_AGENT_RESPONSE_SCHEMA,
            "recipient_id": "agent-0001",
            "correlation_id": "projection-correlation",
            "status": "received",
            "message": "agent-0001 received your message."
        });
        assert!(public_agent_output(&layer8_work(), &operation_with_output(valid)).is_some());

        let leaked = serde_json::json!({
            "schema": LAYER8_AGENT_RESPONSE_SCHEMA,
            "recipient_id": "agent-0001",
            "correlation_id": "projection-correlation",
            "status": "received",
            "message": "agent-0001 received your message.",
            "private_provider_trace": "must not escape"
        });
        assert!(public_agent_output(&layer8_work(), &operation_with_output(leaked)).is_none());

        let mismatched = serde_json::json!({
            "schema": LAYER8_AGENT_RESPONSE_SCHEMA,
            "recipient_id": "agent-0001",
            "correlation_id": "different-correlation",
            "status": "received",
            "message": "agent-0001 received your message."
        });
        assert!(public_agent_output(&layer8_work(), &operation_with_output(mismatched)).is_none());
    }

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
