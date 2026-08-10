use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
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
    communication_keys: Arc<BTreeMap<String, CommunicationVerifyingIdentity>>,
    communication_sequences: Arc<Mutex<BTreeMap<String, u64>>>,
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
        }
    }

    pub async fn submit(
        &self,
        work: DomainWork,
        correlation_id: String,
    ) -> Result<DomainResult, IngressError> {
        validate(&work)?;
        let _lease = self.begin_admission()?;
        let communication_reservation = if let Some((sender_id, sequence)) =
            validate_signed_identity_work(&work, &self.communication_keys, now_unix_millis())?
        {
            let mut sequences = self
                .communication_sequences
                .lock()
                .expect("communication sequence state poisoned");
            if sequence <= sequences.get(&sender_id).copied().unwrap_or(0) {
                return Err(IngressError::Conflict);
            }
            let previous = sequences.insert(sender_id.clone(), sequence);
            Some((sender_id, sequence, previous))
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
                self.rollback_communication_sequence(communication_reservation);
                return Err(IngressError::Closed);
            }
        };
        match outcome {
            Ok(result) => {
                if let Some((sender_id, sequence, _)) = communication_reservation {
                    self.persist_communication_sequence(sender_id, sequence);
                }
                Ok(result)
            }
            Err(error) => {
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

    pub fn restore(&self, snapshot: IngressSnapshot) {
        *self
            .communication_sequences
            .lock()
            .expect("communication sequence state poisoned") =
            snapshot.communication_sequences.clone();
        *self.state.lock().expect("ingress state mutex poisoned") = snapshot;
    }

    fn rollback_communication_sequence(&self, reservation: Option<(String, u64, Option<u64>)>) {
        let Some((sender_id, sequence, previous)) = reservation else {
            return;
        };
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
            .map_err(|error| {
                tracing::warn!(
                    work_id = %work.work_id,
                    work_kind = %work.kind,
                    error = %error,
                    "canonical ingress operation dispatch failed"
                );
                match error {
                    OperationError::InvalidRequest => IngressError::Conflict,
                    _ => IngressError::ExecutionFailed,
                }
            })?;
        apply(&self.state, work, &operation, &self.communication_keys)
    }
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
    now_unix_millis: u64,
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
    verify_signed_identity_message(&message, communication_keys, now_unix_millis)?;
    Ok(Some((message.sender_id, message.monotonic_sequence)))
}

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
    communication_keys: &BTreeMap<String, CommunicationVerifyingIdentity>,
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
        now_unix_millis(),
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
    use crate::verifying_key_from_hex;
    use ed25519_dalek::{Signer, SigningKey};

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
        let valid = serde_json::to_value(signed_ack(&signing_key)).unwrap();
        assert!(public_agent_output(
            &layer8_work(),
            &operation_with_output(valid),
            &keys,
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
                &mut BTreeMap::new(),
            ),
            Err(IngressError::ExecutionFailed)
        );

        let now = now_unix_millis();
        let mut wrong_causation = signed_ack(&signing_key);
        wrong_causation.causation_id = "different-request-nonce".to_owned();
        resign(&mut wrong_causation, &signing_key);
        assert_eq!(
            public_agent_output(
                &layer8_work(),
                &operation_with_output(serde_json::to_value(wrong_causation).unwrap()),
                &keys,
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
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            RuntimeRecorder::new(4),
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
        let now = now_unix_millis();
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
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": message}]
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
