use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex as AsyncMutex, Notify};
use tokio_util::sync::CancellationToken;

use crate::{
    ChannelFullPolicy, Component, ComponentContext, ComponentError, ComponentFactory, ComponentId,
    ComponentSpec, ExternalInput, ExternalInputBinding, FailurePolicy, OperationError,
    OperationRequest, OperationalFactory, PortAccessError, PortProtocol, RuntimeEvent,
    RuntimeRecorder, OPERATION_REQUEST_SCHEMA,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Envelope {
    nonce: u64,
    work: DomainWork,
    correlation_id: String,
}

impl PortProtocol for Envelope {
    const PROTOCOL: &'static str = "adl.runtime.canonical_ingress.request.v1";
}

struct PendingIngress {
    incarnation: u64,
    cancellation: CancellationToken,
    reply: oneshot::Sender<Result<DomainResult, IngressError>>,
}

#[derive(Clone)]
pub struct CanonicalIngress {
    external_input: ExternalInput<Envelope>,
    pending: Arc<AsyncMutex<BTreeMap<u64, PendingIngress>>>,
    nonce: Arc<AtomicU64>,
    next_incarnation: Arc<AtomicU64>,
    active_incarnation: Arc<AtomicU64>,
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
        let external_input = ExternalInput::new(
            "runtime.canonical_ingress.requests",
            capacity,
            ChannelFullPolicy::Reject,
        );
        Self {
            external_input,
            pending: Arc::new(AsyncMutex::new(BTreeMap::new())),
            nonce: Arc::new(AtomicU64::new(0)),
            next_incarnation: Arc::new(AtomicU64::new(0)),
            active_incarnation: Arc::new(AtomicU64::new(0)),
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
        let nonce = self.nonce.fetch_add(1, Ordering::Relaxed);
        self.pending.lock().await.insert(
            nonce,
            PendingIngress {
                incarnation: self.active_incarnation.load(Ordering::Acquire),
                cancellation,
                reply,
            },
        );
        if let Err(error) = self
            .external_input
            .send(&Envelope {
                nonce,
                work,
                correlation_id,
            })
            .await
        {
            self.pending.lock().await.remove(&nonce);
            return Err(match error {
                PortAccessError::Full(_) => IngressError::Saturated,
                _ => IngressError::Closed,
            });
        }
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

struct CanonicalIngressComponent {
    ingress: CanonicalIngress,
    incarnation: u64,
}

fn take_pending_ingress(
    pending: &mut BTreeMap<u64, PendingIngress>,
    incarnation: u64,
) -> Vec<PendingIngress> {
    let keys = pending
        .iter()
        .filter_map(|(nonce, pending)| (pending.incarnation == incarnation).then_some(*nonce))
        .collect::<Vec<_>>();
    keys.into_iter()
        .filter_map(|nonce| pending.remove(&nonce))
        .collect()
}

#[async_trait]
impl Component for CanonicalIngressComponent {
    async fn run(self: Box<Self>, mut context: ComponentContext) -> Result<(), ComponentError> {
        context.ready();
        loop {
            tokio::select! {
                _ = context.cancellation.cancelled() => {
                    let mut pending = self.ingress.pending.lock().await;
                    let closed = take_pending_ingress(&mut pending, self.incarnation);
                    drop(pending);
                    for pending in closed {
                        let _ = pending.reply.send(Err(IngressError::Closed));
                    }
                    return Ok(())
                },
                envelope = context.recv::<Envelope>("runtime.canonical_ingress.requests") => {
                    let Some(envelope) = envelope.map_err(|error| ComponentError::new(error.to_string()))? else { return Ok(()); };
                    let Some(pending) = self.ingress.pending.lock().await.remove(&envelope.nonce) else { continue; };
                    let result = self.ingress
                        .dispatch(&envelope.work, pending.cancellation)
                        .await;
                    if result.is_ok() {
                        self.ingress.recorder.emit_correlated(Some(ComponentId::new("canonical_ingress")),
                            RuntimeEvent::DomainWorkCompleted, Some(&envelope.correlation_id));
                    }
                    let _ = pending.reply.send(result);
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
            inputs: vec![self.external_input.spec()],
            outputs: vec![],
            failure_policy: FailurePolicy::Fatal,
        }
    }
    fn build(&self) -> Box<dyn Component> {
        let incarnation = self.next_incarnation.fetch_add(1, Ordering::AcqRel);
        self.active_incarnation
            .store(incarnation, Ordering::Release);
        Box::new(CanonicalIngressComponent {
            ingress: self.clone(),
            incarnation,
        })
    }

    fn external_inputs(&self) -> Vec<ExternalInputBinding> {
        vec![self.external_input.binding()]
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
    if work.kind == crate::AdapterKind::Shepherd.service_name() {
        let request: crate::ShepherdRequest =
            serde_json::from_slice(&work.payload).map_err(|_| IngressError::ExecutionFailed)?;
        let response: crate::ShepherdResponse = serde_json::from_slice(&operation.payload)
            .map_err(|_| IngressError::ExecutionFailed)?;
        let recipient_id = request
            .conversation_recipient_id
            .filter(|value| !value.is_empty() && value.len() <= 128)
            .ok_or(IngressError::ExecutionFailed)?;
        if response.schema != crate::SHEPHERD_RESPONSE_SCHEMA
            || response.correlation_id != request.correlation_id
            || response.runtime_id != request.runtime_id
            || response.response.is_empty()
            || response.response.len() > 4_096
        {
            return Err(IngressError::ExecutionFailed);
        }
        return Ok(Some(serde_json::json!({
            "schema": "adl.runtime.conversation_reply.v1",
            "recipient_id": recipient_id,
            "message": response.response,
        })));
    }
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

    #[tokio::test]
    async fn cancelled_incarnation_closes_only_its_queued_requests() {
        let (old_reply, old_result) = oneshot::channel();
        let (new_reply, mut new_result) = oneshot::channel();
        let mut pending = BTreeMap::from([
            (
                1,
                PendingIngress {
                    incarnation: 0,
                    cancellation: CancellationToken::new(),
                    reply: old_reply,
                },
            ),
            (
                2,
                PendingIngress {
                    incarnation: 1,
                    cancellation: CancellationToken::new(),
                    reply: new_reply,
                },
            ),
        ]);
        for pending in take_pending_ingress(&mut pending, 0) {
            let _ = pending.reply.send(Err(IngressError::Closed));
        }
        assert_eq!(old_result.await.unwrap(), Err(IngressError::Closed));
        assert!(matches!(
            new_result.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
    }
}
