use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use async_trait::async_trait;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{oneshot, Mutex as AsyncMutex, Notify};

use crate::{
    channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, Component, ComponentContext,
    ComponentError, ComponentFactory, ComponentId, ComponentSpec, FailurePolicy,
    KernelDurableState, KernelDurableStateError, OperationError, OperationRequest,
    OperationalFactory, RuntimeEvent, RuntimeRecorder, SendError, OPERATION_REQUEST_SCHEMA,
};

pub const DOMAIN_WORK_SCHEMA: &str = "adl.runtime.domain_work.v1";
pub const DOMAIN_RESULT_SCHEMA: &str = "adl.runtime.domain_result.v1";
pub const LOCAL_AGENT_WORK_SCHEMA: &str = "adl.runtime.local_agent_work.v1";
pub const LAYER8_MESSAGE_TASK_OP: &str = "layer8_message";
pub const ACIP_IDENTITY_MESSAGE_SCHEMA: &str = "adl.acip.identity_message.v1";
pub const ACIP_IDENTITY_MESSAGE_SIGNING_DOMAIN: &[u8] = b"adl.acip.identity_message.v1\0";

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
    #[serde(default)]
    pub communication_sequences: BTreeMap<String, u64>,
    #[serde(default)]
    pub communication_acknowledgement_sequences: BTreeMap<String, u64>,
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
    reply: oneshot::Sender<DispatchOutcome>,
}

enum DispatchOutcome {
    NotDispatched(IngressError),
    Dispatched(Result<DomainResult, IngressError>),
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
    communication_keys: Arc<BTreeMap<String, CommunicationVerifyingIdentity>>,
    communication_sequences: Arc<Mutex<BTreeMap<String, u64>>>,
    communication_source_locks: Arc<Mutex<BTreeMap<String, Weak<AsyncMutex<()>>>>>,
    communication_replay_store: Option<Arc<KernelDurableState>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommunicationVerifyingIdentity {
    pub signing_key_id: String,
    pub verifying_key: VerifyingKey,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedIdentityMessage {
    pub schema: String,
    pub message_kind: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub correlation_id: String,
    pub causation_id: String,
    pub monotonic_sequence: u64,
    pub issued_at_unix_millis: u64,
    pub expires_at_unix_millis: u64,
    pub nonce: String,
    pub content: String,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

impl SignedIdentityMessage {
    pub fn signing_bytes(&self) -> Option<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        let mut bytes = ACIP_IDENTITY_MESSAGE_SIGNING_DOMAIN.to_vec();
        bytes.extend(serde_jcs::to_vec(&unsigned).ok()?);
        Some(bytes)
    }
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
        Self::new_with_communication_keys(capacity, recorder, dispatchers, BTreeMap::new())
    }

    pub fn new_with_communication_keys(
        capacity: usize,
        recorder: RuntimeRecorder,
        dispatchers: BTreeMap<String, OperationalFactory>,
        communication_keys: BTreeMap<String, CommunicationVerifyingIdentity>,
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
            communication_keys: Arc::new(communication_keys),
            communication_sequences: Arc::new(Mutex::new(BTreeMap::new())),
            communication_source_locks: Arc::new(Mutex::new(BTreeMap::new())),
            communication_replay_store: None,
        }
    }

    pub fn with_communication_replay_store(
        mut self,
        store: Arc<KernelDurableState>,
    ) -> Result<Self, IngressError> {
        let restored = store
            .communication_inbound_sequences()
            .map_err(|_| IngressError::ExecutionFailed)?;
        *self
            .communication_sequences
            .lock()
            .expect("communication sequence state poisoned") = restored.clone();
        self.state
            .lock()
            .expect("ingress state mutex poisoned")
            .communication_sequences = restored;
        self.communication_replay_store = Some(store);
        Ok(self)
    }

    pub async fn submit(
        &self,
        work: DomainWork,
        correlation_id: String,
    ) -> Result<DomainResult, IngressError> {
        validate(&work)?;
        let _lease = self.begin_admission()?;
        let signed_identity = validate_signed_identity_work(
            &work,
            &self.communication_keys,
            self.recorder.qualified_time_now_unix_millis(),
        )?;
        let communication_source_lock = signed_identity
            .as_ref()
            .map(|(sender_id, _)| self.communication_source_lock(sender_id));
        let _communication_source_guard = match &communication_source_lock {
            Some(lock) => Some(lock.lock().await),
            None => None,
        };
        let communication_reservation = if let Some((sender_id, sequence)) = signed_identity {
            Some(self.reserve_communication_sequence(sender_id, sequence)?)
        } else {
            None
        };
        let (reply, result) = oneshot::channel();
        if let Err(error) = self
            .sender
            .send(Envelope {
                work,
                correlation_id,
                reply,
            })
            .await
        {
            self.rollback_communication_sequence(communication_reservation);
            return Err(match error {
                SendError::Full => IngressError::Saturated,
                SendError::Closed => IngressError::Closed,
            });
        }
        self.recorder
            .set_queue_health("canonical_ingress", &self.sender.metrics());
        let outcome = match result.await {
            Ok(outcome) => outcome,
            Err(_) => {
                // Once dispatch succeeded, a dropped reply is ambiguous: the work may have
                // executed before the process or worker disappeared. Retain the replay
                // reservation so a restart cannot execute the signed message twice.
                if let Some((sender_id, sequence, _)) = communication_reservation {
                    self.persist_communication_sequence(sender_id, sequence);
                }
                return Err(IngressError::Closed);
            }
        };
        match outcome {
            DispatchOutcome::Dispatched(Ok(result)) => {
                if let Some((sender_id, sequence, _)) = communication_reservation {
                    self.persist_communication_sequence(sender_id, sequence);
                }
                Ok(result)
            }
            DispatchOutcome::Dispatched(Err(error)) => {
                // The adapter returned or may already have performed side effects. Retain the
                // durable replay watermark even when result projection or acknowledgement
                // verification fails.
                if let Some((sender_id, sequence, _)) = communication_reservation {
                    self.persist_communication_sequence(sender_id, sequence);
                }
                Err(error)
            }
            DispatchOutcome::NotDispatched(error) => {
                self.rollback_communication_sequence(communication_reservation);
                Err(error)
            }
        }
    }

    pub fn snapshot(&self) -> IngressSnapshot {
        self.state
            .lock()
            .expect("ingress state mutex poisoned")
            .clone()
    }

    pub fn verify_communication_message(
        &self,
        message: &SignedIdentityMessage,
        now_unix_millis: u64,
    ) -> Result<(), IngressError> {
        verify_signed_identity_message(message, &self.communication_keys, now_unix_millis)
    }

    fn communication_source_lock(&self, sender_id: &str) -> Arc<AsyncMutex<()>> {
        let mut locks = self
            .communication_source_locks
            .lock()
            .expect("communication source lock registry poisoned");
        locks.retain(|_, lock| lock.strong_count() > 0);
        if let Some(lock) = locks.get(sender_id).and_then(Weak::upgrade) {
            return lock;
        }
        let lock = Arc::new(AsyncMutex::new(()));
        locks.insert(sender_id.to_owned(), Arc::downgrade(&lock));
        lock
    }

    pub fn restore(&self, snapshot: IngressSnapshot) {
        let mut snapshot = snapshot;
        if let Some(store) = &self.communication_replay_store {
            if let Ok(durable) = store.communication_inbound_sequences() {
                for (sender_id, sequence) in durable {
                    let restored = snapshot
                        .communication_sequences
                        .entry(sender_id)
                        .or_default();
                    *restored = (*restored).max(sequence);
                }
            }
        }
        *self
            .communication_sequences
            .lock()
            .expect("communication sequence state poisoned") =
            snapshot.communication_sequences.clone();
        *self.state.lock().expect("ingress state mutex poisoned") = snapshot;
    }

    fn reserve_communication_sequence(
        &self,
        sender_id: String,
        sequence: u64,
    ) -> Result<(String, u64, Option<u64>), IngressError> {
        let previous = if let Some(store) = &self.communication_replay_store {
            store
                .reserve_communication_inbound_sequence(&sender_id, sequence)
                .map_err(|error| match error {
                    KernelDurableStateError::CommunicationSequenceConflict => {
                        IngressError::Conflict
                    }
                    _ => IngressError::ExecutionFailed,
                })?
        } else {
            let sequences = self
                .communication_sequences
                .lock()
                .expect("communication sequence state poisoned");
            let previous = sequences.get(&sender_id).copied();
            if sequence <= previous.unwrap_or(0) {
                return Err(IngressError::Conflict);
            }
            previous
        };
        let mut sequences = self
            .communication_sequences
            .lock()
            .expect("communication sequence state poisoned");
        let current = sequences.entry(sender_id.clone()).or_default();
        *current = (*current).max(sequence);
        Ok((sender_id, sequence, previous))
    }

    fn rollback_communication_sequence(&self, reservation: Option<(String, u64, Option<u64>)>) {
        let Some((sender_id, sequence, previous)) = reservation else {
            return;
        };
        if let Some(store) = &self.communication_replay_store {
            if let Err(error) =
                store.rollback_communication_inbound_sequence(&sender_id, sequence, previous)
            {
                tracing::warn!(
                    sender_id = %sender_id,
                    sequence,
                    error = %error,
                    "durable communication replay reservation retained after rollback failure"
                );
            }
        }
        let mut sequences = self
            .communication_sequences
            .lock()
            .expect("communication sequence state poisoned");
        if sequences.get(&sender_id).copied() != Some(sequence) {
            return;
        }
        match previous {
            Some(previous) => {
                sequences.insert(sender_id, previous);
            }
            None => {
                sequences.remove(&sender_id);
            }
        }
    }

    fn persist_communication_sequence(&self, sender_id: String, sequence: u64) {
        let mut state = self.state.lock().expect("ingress state mutex poisoned");
        let persisted = state.communication_sequences.entry(sender_id).or_default();
        *persisted = (*persisted).max(sequence);
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

    async fn dispatch(&self, work: &DomainWork) -> DispatchOutcome {
        let Some(dispatcher) = self.dispatchers.get(&work.kind) else {
            return DispatchOutcome::NotDispatched(IngressError::UnsupportedKind);
        };
        let operation = match dispatcher
            .submit(OperationRequest {
                schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                request_id: work.work_id.clone(),
                idempotency_key: work.work_id.clone(),
                principal: "canonical-ingress".to_owned(),
                payload: work.payload.clone(),
                permit: None,
            })
            .await
        {
            Ok(operation) => operation,
            Err(error) => {
                let definitely_not_dispatched =
                    operation_error_is_definitely_not_dispatched(&error);
                tracing::warn!(
                    work_id = %work.work_id,
                    work_kind = %work.kind,
                    error = %error,
                    "canonical ingress operation dispatch failed"
                );
                let ingress_error = match error {
                    OperationError::InvalidRequest => IngressError::Conflict,
                    _ => IngressError::ExecutionFailed,
                };
                return if definitely_not_dispatched {
                    DispatchOutcome::NotDispatched(ingress_error)
                } else {
                    DispatchOutcome::Dispatched(Err(ingress_error))
                };
            }
        };
        DispatchOutcome::Dispatched(apply(
            &self.state,
            work,
            &operation,
            &self.communication_keys,
            self.recorder.qualified_time_now_unix_millis(),
        ))
    }
}

fn operation_error_is_definitely_not_dispatched(error: &OperationError) -> bool {
    matches!(
        error,
        OperationError::InvalidPolicy
            | OperationError::InvalidRequest
            | OperationError::MissingAuthority
            | OperationError::Saturated
    )
}

pub fn verify_signed_identity_message(
    message: &SignedIdentityMessage,
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
    now_unix_millis: u64,
) -> Result<(), IngressError> {
    const MAX_INTEROPERABLE_INTEGER: u64 = 9_007_199_254_740_991;
    let safe = |value: &str| {
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'_' | b'-'))
    };
    if message.schema != ACIP_IDENTITY_MESSAGE_SCHEMA
        || !matches!(message.message_kind.as_str(), "request" | "ack")
        || !safe(&message.sender_id)
        || !safe(&message.recipient_id)
        || !safe(&message.correlation_id)
        || !safe(&message.causation_id)
        || !safe(&message.nonce)
        || message.monotonic_sequence == 0
        || message.monotonic_sequence > MAX_INTEROPERABLE_INTEGER
        || message.issued_at_unix_millis > MAX_INTEROPERABLE_INTEGER
        || message.expires_at_unix_millis > MAX_INTEROPERABLE_INTEGER
        || message.content.trim().is_empty()
        || message.content.len() > 4_000
        || message
            .content
            .chars()
            .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        || message.signing_algorithm != "ed25519"
        || message.issued_at_unix_millis > now_unix_millis.saturating_add(5_000)
        || message.expires_at_unix_millis < now_unix_millis
        || message.expires_at_unix_millis <= message.issued_at_unix_millis
        || message
            .expires_at_unix_millis
            .saturating_sub(message.issued_at_unix_millis)
            > 60_000
    {
        return Err(IngressError::Invalid);
    }
    let identity = communication_keys
        .get(&message.sender_id)
        .ok_or(IngressError::UnsupportedKind)?;
    if message.signing_key_id != identity.signing_key_id {
        return Err(IngressError::ExecutionFailed);
    }
    let signature_bytes =
        hex::decode(&message.signature).map_err(|_| IngressError::ExecutionFailed)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| IngressError::ExecutionFailed)?;
    identity
        .verifying_key
        .verify(
            &message.signing_bytes().ok_or(IngressError::Invalid)?,
            &signature,
        )
        .map_err(|_| IngressError::ExecutionFailed)
}

fn validate_signed_identity_work(
    work: &DomainWork,
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
    now_unix_millis: Option<u64>,
) -> Result<Option<(String, u64)>, IngressError> {
    if work.kind != "agent" && work.kind != "shepherd" {
        return Ok(None);
    }
    let payload: serde_json::Value =
        serde_json::from_slice(&work.payload).map_err(|_| IngressError::Invalid)?;
    if payload.get("schema").and_then(serde_json::Value::as_str) != Some(LOCAL_AGENT_WORK_SCHEMA) {
        return Ok(None);
    }
    let tasks = payload
        .get("tasks")
        .and_then(serde_json::Value::as_array)
        .ok_or(IngressError::Invalid)?;
    let identity_tasks: Vec<_> = tasks
        .iter()
        .filter(|task| {
            task.get("op").and_then(serde_json::Value::as_str) == Some(LAYER8_MESSAGE_TASK_OP)
        })
        .collect();
    if identity_tasks.is_empty() {
        return Ok(None);
    }
    if tasks.len() != 1 || identity_tasks.len() != 1 {
        return Err(IngressError::Invalid);
    }
    let message: SignedIdentityMessage = serde_json::from_value(
        identity_tasks[0]
            .get("message")
            .cloned()
            .ok_or(IngressError::Invalid)?,
    )
    .map_err(|_| IngressError::Invalid)?;
    if message.message_kind != "request"
        || (work.kind == "shepherd" && message.recipient_id != "shepherd")
        || (work.kind == "agent" && message.recipient_id == "shepherd")
    {
        return Err(IngressError::Invalid);
    }
    verify_signed_identity_message(
        &message,
        communication_keys,
        now_unix_millis.ok_or(IngressError::ExecutionFailed)?,
    )?;
    Ok(Some((message.sender_id, message.monotonic_sequence)))
}

#[cfg(test)]
fn now_unix_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().try_into().unwrap_or(u64::MAX))
        .unwrap_or(0)
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
                    if matches!(&result, DispatchOutcome::Dispatched(Ok(_))) {
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
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
    now_unix_millis: Option<u64>,
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
    let public_output = public_agent_output(
        work,
        operation,
        communication_keys,
        now_unix_millis,
        &mut state.communication_acknowledgement_sequences,
    )?;
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

fn public_agent_output(
    work: &DomainWork,
    operation: &crate::OperationResult,
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
    now_unix_millis: Option<u64>,
    acknowledgement_sequences: &mut BTreeMap<String, u64>,
) -> Result<Option<serde_json::Value>, IngressError> {
    if work.kind != "agent" && work.kind != "shepherd" {
        return Ok(None);
    }
    let execution: serde_json::Value =
        serde_json::from_slice(&operation.payload).map_err(|_| IngressError::ExecutionFailed)?;
    if execution.get("schema").and_then(serde_json::Value::as_str)
        != Some("adl.runtime.local_agent_execution.v1")
    {
        return Ok(None);
    }
    let Some(outputs) = execution
        .get("outputs")
        .and_then(serde_json::Value::as_array)
    else {
        return Ok(None);
    };
    if outputs.len() != 1 {
        return Ok(None);
    }
    let tasks: serde_json::Value =
        serde_json::from_slice(&work.payload).map_err(|_| IngressError::ExecutionFailed)?;
    let Some(tasks) = tasks.get("tasks").and_then(serde_json::Value::as_array) else {
        return Ok(None);
    };
    if tasks.len() != 1
        || tasks[0].get("op").and_then(serde_json::Value::as_str) != Some(LAYER8_MESSAGE_TASK_OP)
    {
        return Ok(None);
    }
    let request: SignedIdentityMessage = serde_json::from_value(
        tasks[0]
            .get("message")
            .cloned()
            .ok_or(IngressError::ExecutionFailed)?,
    )
    .map_err(|_| IngressError::ExecutionFailed)?;
    let output: SignedIdentityMessage = serde_json::from_value(
        outputs
            .first()
            .and_then(|value| value.get("output"))
            .cloned()
            .ok_or(IngressError::ExecutionFailed)?,
    )
    .map_err(|_| IngressError::ExecutionFailed)?;
    if output.message_kind != "ack"
        || output.sender_id != request.recipient_id
        || output.recipient_id != request.sender_id
        || output.correlation_id != request.correlation_id
        || output.causation_id != request.nonce
        || output.content.len() > 512
    {
        return Err(IngressError::ExecutionFailed);
    }
    verify_signed_identity_message_with_watermark(
        &output,
        communication_keys,
        now_unix_millis.ok_or(IngressError::ExecutionFailed)?,
        acknowledgement_sequences,
    )
    .map_err(|_| IngressError::ExecutionFailed)?;
    serde_json::to_value(output)
        .map(Some)
        .map_err(|_| IngressError::ExecutionFailed)
}

pub fn verify_signed_identity_message_with_watermark(
    message: &SignedIdentityMessage,
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
    now_unix_millis: u64,
    sequences: &mut BTreeMap<String, u64>,
) -> Result<(), IngressError> {
    verify_signed_identity_message(message, communication_keys, now_unix_millis)?;
    let previous = sequences.get(&message.sender_id).copied().unwrap_or(0);
    if message.monotonic_sequence <= previous {
        return Err(IngressError::Conflict);
    }
    sequences.insert(message.sender_id.clone(), message.monotonic_sequence);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::{
        verifying_key_from_hex, AdapterKind, AdapterPolicy, AuthorityMode, ExecutorError,
        FailureClass, OperationExecutor, OperationalAdapter,
    };
    use ed25519_dalek::{Signer, SigningKey};
    use tokio::sync::Notify;
    use tokio_util::sync::CancellationToken;

    struct StartedPendingExecutor {
        started: Arc<Notify>,
    }

    #[async_trait]
    impl OperationExecutor for StartedPendingExecutor {
        async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
            self.started.notify_one();
            std::future::pending().await
        }
    }

    struct StartedFatalExecutor {
        invocations: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl OperationExecutor for StartedFatalExecutor {
        async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
            self.invocations.fetch_add(1, Ordering::SeqCst);
            Err(ExecutorError {
                class: FailureClass::Fatal,
                message: "injected operation failure".to_owned(),
            })
        }
    }

    fn test_agent_policy() -> AdapterPolicy {
        AdapterPolicy {
            capacity: 1,
            max_in_flight: 1,
            shutdown_grace_millis: 100,
            max_attempts: 1,
            idempotency_entries: 4,
            authority: AuthorityMode::Internal,
        }
    }

    fn operator_communication_keys(
        signing_key: &SigningKey,
    ) -> BTreeMap<String, CommunicationVerifyingIdentity> {
        BTreeMap::from([(
            "layer8-operator".to_owned(),
            CommunicationVerifyingIdentity {
                signing_key_id: "operator-key".to_owned(),
                verifying_key: signing_key.verifying_key(),
            },
        )])
    }

    fn start_component(
        factory: &impl ComponentFactory,
        recorder: RuntimeRecorder,
    ) -> (
        CancellationToken,
        tokio::task::JoinHandle<Result<(), ComponentError>>,
        oneshot::Receiver<()>,
    ) {
        let cancellation = CancellationToken::new();
        let (ready_tx, ready_rx) = oneshot::channel();
        let context =
            ComponentContext::new(factory.spec().id, cancellation.clone(), recorder, ready_tx);
        let component = factory.build();
        let task = tokio::spawn(async move { component.run(context).await });
        (cancellation, task, ready_rx)
    }

    fn layer8_work() -> DomainWork {
        let now = now_unix_millis();
        let request = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "request".to_owned(),
            sender_id: "layer8-operator".to_owned(),
            recipient_id: "agent-0001".to_owned(),
            correlation_id: "projection-correlation".to_owned(),
            causation_id: "projection-causation".to_owned(),
            monotonic_sequence: 1,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now.saturating_add(60_000),
            nonce: "projection-nonce".to_owned(),
            content: "Hello".to_owned(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            signature: "00".repeat(64),
        };
        DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: "layer8-projection".to_owned(),
            kind: "agent".to_owned(),
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{
                    "op": LAYER8_MESSAGE_TASK_OP,
                    "message": request
                }]
            }))
            .unwrap(),
        }
    }

    fn signed_ack(signing_key: &SigningKey) -> SignedIdentityMessage {
        let now = now_unix_millis();
        let mut output = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "ack".to_owned(),
            sender_id: "agent-0001".to_owned(),
            recipient_id: "layer8-operator".to_owned(),
            correlation_id: "projection-correlation".to_owned(),
            causation_id: "projection-nonce".to_owned(),
            monotonic_sequence: 2,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now.saturating_add(60_000),
            nonce: "projection-ack".to_owned(),
            content: "agent-0001 received your message.".to_owned(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: "agent-key".to_owned(),
            signature: String::new(),
        };
        output.signature = hex::encode(
            signing_key
                .sign(&output.signing_bytes().unwrap())
                .to_bytes(),
        );
        output
    }

    fn resign(message: &mut SignedIdentityMessage, signing_key: &SigningKey) {
        message.signature = hex::encode(
            signing_key
                .sign(&message.signing_bytes().unwrap())
                .to_bytes(),
        );
    }

    fn communication_keys(
        signing_key: &SigningKey,
    ) -> BTreeMap<String, CommunicationVerifyingIdentity> {
        BTreeMap::from([(
            "agent-0001".to_owned(),
            CommunicationVerifyingIdentity {
                signing_key_id: "agent-key".to_owned(),
                verifying_key: signing_key.verifying_key(),
            },
        )])
    }

    fn signed_request_work(signing_key: &SigningKey, sequence: u64) -> DomainWork {
        let now = now_unix_millis();
        let mut request = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "request".to_owned(),
            sender_id: "layer8-operator".to_owned(),
            recipient_id: "agent-0001".to_owned(),
            correlation_id: "durable-dispatch-correlation".to_owned(),
            causation_id: "durable-dispatch-causation".to_owned(),
            monotonic_sequence: sequence,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now.saturating_add(60_000),
            nonce: "durable-dispatch-nonce".to_owned(),
            content: "execute once".to_owned(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            signature: String::new(),
        };
        resign(&mut request, signing_key);
        DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: "durable-dispatch-work".to_owned(),
            kind: "agent".to_owned(),
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": request}]
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
        let signing_key = SigningKey::from_bytes(&[91; 32]);
        let keys = communication_keys(&signing_key);
        let now = now_unix_millis();
        let valid = serde_json::to_value(signed_ack(&signing_key)).unwrap();
        assert!(public_agent_output(
            &layer8_work(),
            &operation_with_output(valid),
            &keys,
            Some(now),
            &mut BTreeMap::new(),
        )
        .unwrap()
        .is_some());

        let mut leaked = serde_json::to_value(signed_ack(&signing_key)).unwrap();
        leaked["private_provider_trace"] = serde_json::json!("must not escape");
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(leaked),
                &keys,
                Some(now),
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );

        let mut mismatched = signed_ack(&signing_key);
        mismatched.correlation_id = "different-correlation".to_owned();
        mismatched.signature = hex::encode(
            signing_key
                .sign(&mismatched.signing_bytes().unwrap())
                .to_bytes(),
        );
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(serde_json::to_value(mismatched).unwrap()),
                &keys,
                Some(now),
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );

        let mut wrong_causation = signed_ack(&signing_key);
        wrong_causation.causation_id = "different-request-nonce".to_owned();
        resign(&mut wrong_causation, &signing_key);
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(serde_json::to_value(wrong_causation).unwrap()),
                &keys,
                Some(now),
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );

        let mut stale = signed_ack(&signing_key);
        stale.issued_at_unix_millis = now.saturating_sub(120_000);
        stale.expires_at_unix_millis = now.saturating_sub(60_000);
        resign(&mut stale, &signing_key);
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(serde_json::to_value(stale).unwrap()),
                &keys,
                Some(now),
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );

        let mut future = signed_ack(&signing_key);
        future.issued_at_unix_millis = now.saturating_add(6_000);
        future.expires_at_unix_millis = now.saturating_add(66_000);
        resign(&mut future, &signing_key);
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(serde_json::to_value(future).unwrap()),
                &keys,
                Some(now),
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );
    }

    #[test]
    fn signed_identity_message_rejects_non_positive_validity_window() {
        let signing_key = SigningKey::from_bytes(&[92; 32]);
        let mut message = signed_ack(&signing_key);
        message.issued_at_unix_millis = 10;
        message.expires_at_unix_millis = 10;
        message.signature = hex::encode(
            signing_key
                .sign(&message.signing_bytes().unwrap())
                .to_bytes(),
        );
        assert_eq!(
            verify_signed_identity_message(&message, &communication_keys(&signing_key), 10),
            Err(IngressError::Invalid)
        );
    }

    #[tokio::test]
    async fn post_dispatch_projection_failure_retains_durable_replay_watermark() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let signing_key = SigningKey::from_bytes(&[95; 32]);
        let now = now_unix_millis();
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now,
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder,
            BTreeMap::new(),
            BTreeMap::from([(
                "layer8-operator".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "operator-key".to_owned(),
                    verifying_key: signing_key.verifying_key(),
                },
            )]),
        )
        .with_communication_replay_store(store.clone())
        .unwrap();

        let receiver = ingress.receiver.clone();
        let worker = tokio::spawn(async move {
            let envelope = receiver.lock().await.recv().await.unwrap();
            envelope
                .reply
                .send(DispatchOutcome::Dispatched(Err(
                    IngressError::ExecutionFailed,
                )))
                .ok();
        });
        let work = signed_request_work(&signing_key, 1);
        assert_eq!(
            ingress
                .submit(work.clone(), "durable-dispatch-correlation".to_owned())
                .await,
            Err(IngressError::ExecutionFailed)
        );
        worker.await.unwrap();
        assert_eq!(
            store
                .communication_inbound_sequences()
                .unwrap()
                .get("layer8-operator"),
            Some(&1)
        );

        drop(ingress);
        drop(store);
        let reopened_store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now,
        });
        let restarted = CanonicalIngress::new_with_communication_keys(
            1,
            recorder,
            BTreeMap::new(),
            BTreeMap::from([(
                "layer8-operator".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "operator-key".to_owned(),
                    verifying_key: signing_key.verifying_key(),
                },
            )]),
        )
        .with_communication_replay_store(reopened_store)
        .unwrap();
        assert_eq!(
            restarted
                .submit(work, "durable-dispatch-replay".to_owned())
                .await,
            Err(IngressError::Conflict)
        );
    }

    #[tokio::test]
    async fn definitely_not_dispatched_failure_rolls_back_durable_reservation() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let signing_key = SigningKey::from_bytes(&[96; 32]);
        let now = now_unix_millis();
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now,
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder,
            BTreeMap::new(),
            BTreeMap::from([(
                "layer8-operator".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "operator-key".to_owned(),
                    verifying_key: signing_key.verifying_key(),
                },
            )]),
        )
        .with_communication_replay_store(store.clone())
        .unwrap();
        let work = signed_request_work(&signing_key, 1);

        let receiver = ingress.receiver.clone();
        let worker = tokio::spawn(async move {
            let envelope = receiver.lock().await.recv().await.unwrap();
            envelope
                .reply
                .send(DispatchOutcome::NotDispatched(IngressError::Saturated))
                .ok();
        });
        assert_eq!(
            ingress
                .submit(work.clone(), "pre-dispatch-correlation".to_owned())
                .await,
            Err(IngressError::Saturated)
        );
        worker.await.unwrap();
        assert!(store
            .communication_inbound_sequences()
            .unwrap()
            .get("layer8-operator")
            .is_none());

        let receiver = ingress.receiver.clone();
        let worker = tokio::spawn(async move {
            let envelope = receiver.lock().await.recv().await.unwrap();
            envelope
                .reply
                .send(DispatchOutcome::Dispatched(Err(
                    IngressError::ExecutionFailed,
                )))
                .ok();
        });
        assert_eq!(
            ingress.submit(work, "retry-correlation".to_owned()).await,
            Err(IngressError::ExecutionFailed)
        );
        worker.await.unwrap();
        assert_eq!(
            store
                .communication_inbound_sequences()
                .unwrap()
                .get("layer8-operator"),
            Some(&1)
        );
    }

    #[tokio::test]
    async fn concurrent_pre_dispatch_failures_serialize_per_sender_without_stranding_sequence() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let signing_key = SigningKey::from_bytes(&[99; 32]);
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now_unix_millis(),
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            2,
            recorder,
            BTreeMap::new(),
            operator_communication_keys(&signing_key),
        )
        .with_communication_replay_store(store.clone())
        .unwrap();
        let receiver = ingress.receiver.clone();

        let first = {
            let ingress = ingress.clone();
            let work = signed_request_work(&signing_key, 1);
            tokio::spawn(async move { ingress.submit(work, "concurrent-first".to_owned()).await })
        };
        let first_envelope = receiver.lock().await.recv().await.unwrap();
        let second = {
            let ingress = ingress.clone();
            let work = signed_request_work(&signing_key, 2);
            tokio::spawn(async move { ingress.submit(work, "concurrent-second".to_owned()).await })
        };
        assert!(tokio::time::timeout(Duration::from_millis(100), async {
            receiver.lock().await.recv().await
        })
        .await
        .is_err());
        first_envelope
            .reply
            .send(DispatchOutcome::NotDispatched(IngressError::Saturated))
            .ok();
        assert_eq!(first.await.unwrap(), Err(IngressError::Saturated));

        let second_envelope = receiver.lock().await.recv().await.unwrap();
        second_envelope
            .reply
            .send(DispatchOutcome::NotDispatched(IngressError::Saturated))
            .ok();
        assert_eq!(second.await.unwrap(), Err(IngressError::Saturated));
        assert!(store
            .communication_inbound_sequences()
            .unwrap()
            .get("layer8-operator")
            .is_none());

        let retry = {
            let ingress = ingress.clone();
            let work = signed_request_work(&signing_key, 1);
            tokio::spawn(async move { ingress.submit(work, "concurrent-retry".to_owned()).await })
        };
        let retry_envelope = receiver.lock().await.recv().await.unwrap();
        retry_envelope
            .reply
            .send(DispatchOutcome::Dispatched(Err(
                IngressError::ExecutionFailed,
            )))
            .ok();
        assert_eq!(retry.await.unwrap(), Err(IngressError::ExecutionFailed));
        assert_eq!(
            store
                .communication_inbound_sequences()
                .unwrap()
                .get("layer8-operator"),
            Some(&1)
        );
    }

    #[tokio::test]
    async fn real_in_flight_cancellation_retains_watermark_across_restart() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let signing_key = SigningKey::from_bytes(&[97; 32]);
        let started = Arc::new(Notify::new());
        let adapter = Arc::new(
            OperationalAdapter::new(
                AdapterKind::Agent,
                test_agent_policy(),
                Arc::new(StartedPendingExecutor {
                    started: started.clone(),
                }),
            )
            .unwrap(),
        );
        let factory = OperationalFactory::new(adapter, Vec::new());
        let recorder = RuntimeRecorder::new(8);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now_unix_millis(),
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder.clone(),
            BTreeMap::from([("agent".to_owned(), factory.clone())]),
            operator_communication_keys(&signing_key),
        )
        .with_communication_replay_store(store.clone())
        .unwrap();
        let (factory_cancel, factory_task, factory_ready) =
            start_component(&factory, recorder.clone());
        let (ingress_cancel, ingress_task, ingress_ready) =
            start_component(&ingress, recorder.clone());
        factory_ready.await.unwrap();
        ingress_ready.await.unwrap();

        let work = signed_request_work(&signing_key, 1);
        let submitted = {
            let ingress = ingress.clone();
            let work = work.clone();
            tokio::spawn(async move {
                ingress
                    .submit(work, "real-cancellation-correlation".to_owned())
                    .await
            })
        };
        tokio::time::timeout(Duration::from_secs(1), started.notified())
            .await
            .expect("executor must start before cancellation");
        factory_cancel.cancel();
        assert_eq!(submitted.await.unwrap(), Err(IngressError::ExecutionFailed));
        assert!(factory_task.await.unwrap().is_ok());
        assert_eq!(
            store
                .communication_inbound_sequences()
                .unwrap()
                .get("layer8-operator"),
            Some(&1)
        );
        ingress_cancel.cancel();
        assert!(ingress_task.await.unwrap().is_ok());
        drop(ingress);
        drop(factory);
        drop(store);

        let reopened_store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let recorder = RuntimeRecorder::new(8);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now_unix_millis(),
        });
        let restarted = CanonicalIngress::new_with_communication_keys(
            1,
            recorder,
            BTreeMap::new(),
            operator_communication_keys(&signing_key),
        )
        .with_communication_replay_store(reopened_store)
        .unwrap();
        assert_eq!(
            restarted
                .submit(work, "real-cancellation-replay".to_owned())
                .await,
            Err(IngressError::Conflict)
        );
    }

    #[tokio::test]
    async fn real_factory_saturation_rolls_back_and_allows_same_sequence_retry() {
        let root = tempfile::tempdir().unwrap();
        let store = Arc::new(KernelDurableState::open(root.path()).unwrap());
        let signing_key = SigningKey::from_bytes(&[98; 32]);
        let invocations = Arc::new(AtomicUsize::new(0));
        let adapter = Arc::new(
            OperationalAdapter::new(
                AdapterKind::Agent,
                test_agent_policy(),
                Arc::new(StartedFatalExecutor {
                    invocations: invocations.clone(),
                }),
            )
            .unwrap(),
        );
        let factory = OperationalFactory::new(adapter, Vec::new());
        let recorder = RuntimeRecorder::new(8);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now_unix_millis(),
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder.clone(),
            BTreeMap::from([("agent".to_owned(), factory.clone())]),
            operator_communication_keys(&signing_key),
        )
        .with_communication_replay_store(store.clone())
        .unwrap();
        let (ingress_cancel, ingress_task, ingress_ready) =
            start_component(&ingress, recorder.clone());
        ingress_ready.await.unwrap();

        let occupied = {
            let factory = factory.clone();
            tokio::spawn(async move {
                factory
                    .submit(OperationRequest {
                        schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                        request_id: "occupy-factory-queue".to_owned(),
                        idempotency_key: "occupy-factory-queue".to_owned(),
                        principal: "canonical-ingress".to_owned(),
                        payload: b"queued-before-component-start".to_vec(),
                        permit: None,
                    })
                    .await
            })
        };
        tokio::time::timeout(Duration::from_secs(1), async {
            while factory.queued_requests() != 1 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("the first request must occupy the real factory queue");
        assert!(!occupied.is_finished());
        let work = signed_request_work(&signing_key, 1);
        assert_eq!(
            ingress
                .submit(work.clone(), "real-saturation-correlation".to_owned())
                .await,
            Err(IngressError::ExecutionFailed)
        );
        assert!(store
            .communication_inbound_sequences()
            .unwrap()
            .get("layer8-operator")
            .is_none());

        let (factory_cancel, factory_task, factory_ready) =
            start_component(&factory, recorder.clone());
        factory_ready.await.unwrap();
        assert!(occupied.await.unwrap().is_err());
        assert_eq!(invocations.load(Ordering::SeqCst), 1);
        assert_eq!(
            ingress
                .submit(work, "real-saturation-retry".to_owned())
                .await,
            Err(IngressError::ExecutionFailed)
        );
        assert_eq!(
            invocations.load(Ordering::SeqCst),
            2,
            "retry must independently reach the real executor"
        );
        assert_eq!(
            store
                .communication_inbound_sequences()
                .unwrap()
                .get("layer8-operator"),
            Some(&1)
        );
        factory_cancel.cancel();
        ingress_cancel.cancel();
        assert!(factory_task.await.unwrap().is_ok());
        assert!(ingress_task.await.unwrap().is_ok());
    }

    #[test]
    fn only_unambiguous_pre_dispatch_operation_errors_authorize_rollback() {
        assert!(operation_error_is_definitely_not_dispatched(
            &OperationError::Saturated
        ));
        assert!(operation_error_is_definitely_not_dispatched(
            &OperationError::InvalidRequest
        ));
        assert!(!operation_error_is_definitely_not_dispatched(
            &OperationError::AdmissionClosed
        ));
        assert!(!operation_error_is_definitely_not_dispatched(
            &OperationError::Fatal("reply lost after execution began".to_owned())
        ));
    }

    #[test]
    fn signed_identity_jcs_golden_vector_is_stable() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../docs/api/runtime-v3/v1/acip-identity-message-golden.json"
        ))
        .unwrap();
        let message: SignedIdentityMessage =
            serde_json::from_value(fixture["message"].clone()).unwrap();
        assert_eq!(
            hex::encode(message.signing_bytes().unwrap()),
            fixture["signing_bytes_hex"].as_str().unwrap()
        );
        let public_key = verifying_key_from_hex(fixture["public_key_hex"].as_str().unwrap())
            .expect("golden public key");
        verify_signed_identity_message(
            &message,
            &BTreeMap::from([(
                "agent-golden".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "golden-key".to_owned(),
                    verifying_key: public_key,
                },
            )]),
            1_725_000_000_000,
        )
        .unwrap();
    }

    #[test]
    fn restore_rehydrates_secure_replay_watermarks() {
        let ingress = CanonicalIngress::new(1, RuntimeRecorder::new(4), BTreeMap::new());
        ingress.persist_communication_sequence("agent-0001".to_owned(), 19);
        ingress.persist_communication_sequence("agent-0001".to_owned(), 18);
        assert_eq!(
            ingress.snapshot().communication_sequences.get("agent-0001"),
            Some(&19)
        );
        ingress.restore(IngressSnapshot {
            accepted_through: 9,
            completed: BTreeMap::new(),
            communication_sequences: BTreeMap::from([("agent-0001".to_owned(), 17)]),
            communication_acknowledgement_sequences: BTreeMap::from([(
                "agent-0001".to_owned(),
                23,
            )]),
        });
        assert_eq!(
            ingress
                .communication_sequences
                .lock()
                .unwrap()
                .get("agent-0001"),
            Some(&17)
        );
        assert_eq!(
            ingress.snapshot().communication_sequences.get("agent-0001"),
            Some(&17)
        );
        assert_eq!(
            ingress
                .snapshot()
                .communication_acknowledgement_sequences
                .get("agent-0001"),
            Some(&23)
        );
    }

    #[tokio::test]
    async fn restored_ingress_rejects_signed_replay_before_dispatch() {
        let signing_key = SigningKey::from_bytes(&[93; 32]);
        let now = 10_000;
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(crate::ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now,
        });
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder,
            BTreeMap::new(),
            BTreeMap::from([(
                "layer8-operator".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "operator-key".to_owned(),
                    verifying_key: signing_key.verifying_key(),
                },
            )]),
        );
        ingress.restore(IngressSnapshot {
            accepted_through: 3,
            completed: BTreeMap::new(),
            communication_sequences: BTreeMap::from([("layer8-operator".to_owned(), 17)]),
            communication_acknowledgement_sequences: BTreeMap::new(),
        });
        let mut message = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "request".to_owned(),
            sender_id: "layer8-operator".to_owned(),
            recipient_id: "agent-0001".to_owned(),
            correlation_id: "restored-replay-correlation".to_owned(),
            causation_id: "restored-replay-causation".to_owned(),
            monotonic_sequence: 17,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now + 60_000,
            nonce: "restored-replay-nonce".to_owned(),
            content: "must not dispatch".to_owned(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            signature: String::new(),
        };
        message.signature = hex::encode(
            signing_key
                .sign(&message.signing_bytes().unwrap())
                .to_bytes(),
        );
        let work = DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: "restored-replay-work".to_owned(),
            kind: "agent".to_owned(),
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": message.clone()}]
            }))
            .unwrap(),
        };
        assert_eq!(
            ingress
                .submit(work, "restored-replay-correlation".to_owned())
                .await,
            Err(IngressError::Conflict)
        );
        assert_eq!(
            ingress
                .snapshot()
                .communication_sequences
                .get("layer8-operator"),
            Some(&17)
        );

        ingress
            .recorder
            .set_clock_authority(crate::ClockAuthority::Degraded {
                reason: "qualified clock unavailable".to_owned(),
            });
        message.monotonic_sequence = 18;
        message.nonce = "degraded-clock-nonce".to_owned();
        resign(&mut message, &signing_key);
        let degraded_work = DomainWork {
            schema: DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: "degraded-clock-work".to_owned(),
            kind: "agent".to_owned(),
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": message}]
            }))
            .unwrap(),
        };
        assert_eq!(
            ingress
                .submit(degraded_work, "degraded-clock-correlation".to_owned())
                .await,
            Err(IngressError::ExecutionFailed)
        );
        assert_eq!(
            ingress
                .snapshot()
                .communication_sequences
                .get("layer8-operator"),
            Some(&17)
        );
    }

    #[test]
    fn restored_acknowledgement_watermark_rejects_a_pre_restart_replay() {
        let signing_key = SigningKey::from_bytes(&[94; 32]);
        let keys = communication_keys(&signing_key);
        let mut watermarks = BTreeMap::from([("agent-0001".to_owned(), 2)]);
        let replay = signed_ack(&signing_key);
        assert_eq!(replay.monotonic_sequence, 2);
        assert_eq!(
            verify_signed_identity_message_with_watermark(
                &replay,
                &keys,
                now_unix_millis(),
                &mut watermarks,
            ),
            Err(IngressError::Conflict)
        );
        assert_eq!(watermarks.get("agent-0001"), Some(&2));

        let mut advanced = replay;
        advanced.monotonic_sequence = 3;
        advanced.nonce = "projection-ack-advanced".to_owned();
        resign(&mut advanced, &signing_key);
        verify_signed_identity_message_with_watermark(
            &advanced,
            &keys,
            now_unix_millis(),
            &mut watermarks,
        )
        .unwrap();
        assert_eq!(watermarks.get("agent-0001"), Some(&3));
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
