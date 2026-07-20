use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex as AsyncMutex};

use crate::{
    channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, Component, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, FailurePolicy, RuntimeEvent,
    RuntimeRecorder, SendError,
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
    accepting: Arc<AtomicBool>,
}

impl CanonicalIngress {
    pub fn new(capacity: usize, recorder: RuntimeRecorder) -> Self {
        let (sender, receiver) = channel(capacity, ChannelFullPolicy::Reject);
        recorder.set_queue_health("canonical_ingress", &sender.metrics());
        Self {
            sender,
            receiver: Arc::new(AsyncMutex::new(receiver)),
            state: Arc::new(Mutex::new(IngressSnapshot::default())),
            recorder,
            accepting: Arc::new(AtomicBool::new(true)),
        }
    }

    pub async fn submit(
        &self,
        work: DomainWork,
        correlation_id: String,
    ) -> Result<DomainResult, IngressError> {
        validate(&work)?;
        if !self.accepting.load(Ordering::Acquire) {
            return Err(IngressError::Closed);
        }
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

    pub fn pause_if_idle(&self) -> bool {
        if self.sender.metrics().depth() != 0 {
            return false;
        }
        self.accepting.store(false, Ordering::Release);
        true
    }

    pub fn reopen(&self) {
        self.accepting.store(true, Ordering::Release);
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
                    let result = apply(&self.state, &envelope.work);
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

fn apply(state: &Mutex<IngressSnapshot>, work: &DomainWork) -> Result<DomainResult, IngressError> {
    let result_hash = blake3::hash(&serde_json::to_vec(work).map_err(|_| IngressError::Invalid)?)
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
    };
    state.completed.insert(work.work_id.clone(), result.clone());
    Ok(result)
}
