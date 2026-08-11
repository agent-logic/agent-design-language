use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    io::Write,
    net::SocketAddr,
    sync::Arc,
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{
        ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade},
        DefaultBodyLimit, Query, State,
    },
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::sync::CancellationToken;
use tracing::Instrument;
use utoipa_swagger_ui::{Config as SwaggerConfig, SwaggerUi, Url as SwaggerUrl};

use crate::{
    decode_acip_envelope, AgentPresence, AgentRoster, AgentRosterEntry, AgentRosterPolicy,
    AgentRosterQuery, AgentRuntimeEvidence, BootstrapEvent, CanonicalIngress, CheckpointManifest,
    ComponentId, DomainResult, DomainWork, IngressError, KernelControl, KernelExit, LifecycleState,
    LiveContinuity, ObservabilityHealth, RunningState, RuntimeRecorder, RuntimeSnapshot,
    RuntimeTlsInitConfig, WeatherHealthReport, ACIP_WEBSOCKET_SCHEMA, AGENT_ROSTER_PAGE_SCHEMA,
};

pub const CONTROL_COMMAND_SCHEMA: &str = "adl.runtime.control_command.v1";
pub const CONTROL_RESPONSE_SCHEMA: &str = "adl.runtime.control_response.v1";
pub const LEGACY_OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v1";
pub const OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v2";
pub const MAX_SHUTDOWN_GRACE_MILLIS: u64 = 60_000;
pub const API_DOCS_PATH: &str = "/v1/docs/";
pub const OBSERVATORY_API_DOCS_PATH: &str = "/v1/observatory/docs/";
pub const RUNTIME_OPENAPI_PATH: &str = "/v1/openapi.json";
pub const OBSERVATORY_OPENAPI_PATH: &str = "/v1/observatory/openapi.json";
pub const RUNTIME_HEALTH_PATH: &str = "/v1/health";
pub const RUNTIME_READY_PATH: &str = "/v1/ready";
pub const RUNTIME_METRICS_PATH: &str = "/v1/metrics";
pub const ACIP_WS_PATH: &str = "/v1/acip/ws";
pub const OBSERVATORY_WS_PATH: &str = "/v1/observatory/ws";
pub const OBSERVATORY_WS_AUTH_SCHEMA: &str = "adl.runtime_v3.observatory_ws_auth.v1";
pub const OBSERVATORY_WS_CONTROL_RESULT_SCHEMA: &str =
    "adl.runtime_v3.observatory_ws_control_result.v1";
pub const OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA: &str =
    "adl.runtime_v3.observatory_conversation_intent.v1";
pub const OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA: &str =
    "adl.runtime_v3.observatory_conversation_result.v1";
pub const OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA: &str =
    "adl.runtime_v3.observatory_conversation_cancel.v1";
pub const CONTROL_MAX_BODY_BYTES: usize = 64 * 1024;
const RUNTIME_OPENAPI_DOCUMENT: &str = include_str!("../../docs/api/runtime-v3/v1/openapi.json");
const OBSERVATORY_OPENAPI_DOCUMENT: &str =
    include_str!("../../docs/api/runtime-v3/v1/observatory.openapi.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlApiPolicy {
    pub shutdown_grace: Duration,
    pub websocket_auth_timeout: Duration,
    pub websocket_refresh: Duration,
    pub websocket_max_frame_bytes: usize,
    pub control_max_body_bytes: usize,
}

impl ControlApiPolicy {
    pub fn new(
        shutdown_grace: Duration,
        websocket_auth_timeout: Duration,
        websocket_refresh: Duration,
        websocket_max_frame_bytes: usize,
    ) -> Result<Self, ControlApiError> {
        if shutdown_grace.is_zero()
            || websocket_auth_timeout.is_zero()
            || websocket_refresh.is_zero()
            || websocket_max_frame_bytes == 0
        {
            return Err(ControlApiError::MissingPolicy);
        }
        Ok(Self {
            shutdown_grace,
            websocket_auth_timeout,
            websocket_refresh,
            websocket_max_frame_bytes,
            control_max_body_bytes: CONTROL_MAX_BODY_BYTES,
        })
    }
}

pub fn control_ready_event(
    instance_id: &str,
    address: SocketAddr,
    public_base_url: &str,
) -> String {
    assert!(
        is_safe_identifier(instance_id),
        "runtime instance id must be bounded"
    );
    assert!(
        is_safe_https_base(public_base_url),
        "runtime public base URL must be bounded HTTPS"
    );
    format!(
        "adl_event schema=adl.runtime.instance.v1 event=control_ready instance_id={instance_id} port={} public_base_url={public_base_url}",
        address.port(),
    )
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlCapability {
    Read,
    Execute,
    Stop,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ControlAction {
    Snapshot,
    Submit {
        work: DomainWork,
    },
    Shutdown {
        grace_millis: u64,
    },
    Restart {
        expected_incarnation_id: String,
        grace_millis: u64,
    },
}

impl ControlAction {
    fn capability(&self) -> ControlCapability {
        match self {
            Self::Snapshot => ControlCapability::Read,
            Self::Submit { .. } => ControlCapability::Execute,
            Self::Shutdown { .. } => ControlCapability::Stop,
            Self::Restart { .. } => ControlCapability::Stop,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedControlCommand {
    pub schema: String,
    pub runtime_instance_id: String,
    pub command_id: String,
    pub correlation_id: String,
    pub principal: String,
    pub action: ControlAction,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signature: String,
}

impl SignedControlCommand {
    pub fn sign(
        command_id: impl Into<String>,
        correlation_id: impl Into<String>,
        runtime_instance_id: impl Into<String>,
        principal: impl Into<String>,
        action: ControlAction,
        key_id: impl Into<String>,
        key: &SigningKey,
    ) -> Result<Self, ControlError> {
        let mut command = Self {
            schema: CONTROL_COMMAND_SCHEMA.to_owned(),
            runtime_instance_id: runtime_instance_id.into(),
            command_id: command_id.into(),
            correlation_id: correlation_id.into(),
            principal: principal.into(),
            action,
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: key_id.into(),
            signature: String::new(),
        };
        command.validate_public_fields()?;
        command.signature = hex::encode(key.sign(&command.signing_bytes()?).to_bytes());
        Ok(command)
    }

    fn signing_bytes(&self) -> Result<Vec<u8>, ControlError> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_json::to_vec(&unsigned).map_err(|error| ControlError::Encoding(error.to_string()))
    }

    fn fingerprint(&self) -> Result<String, ControlError> {
        Ok(blake3::hash(&self.signing_bytes()?).to_hex().to_string())
    }

    fn validate_public_fields(&self) -> Result<(), ControlError> {
        for value in [
            &self.command_id,
            &self.runtime_instance_id,
            &self.principal,
            &self.signing_key_id,
        ] {
            if !is_safe_identifier(value) {
                return Err(ControlError::InvalidIdentifier);
            }
        }
        if !is_correlation_id(&self.correlation_id) {
            return Err(ControlError::InvalidIdentifier);
        }
        if matches!(
            self.action,
            ControlAction::Shutdown { grace_millis }
                | ControlAction::Restart { grace_millis, .. }
                if grace_millis == 0 || grace_millis > MAX_SHUTDOWN_GRACE_MILLIS
        ) {
            return Err(ControlError::InvalidBounds);
        }
        if let ControlAction::Restart {
            expected_incarnation_id,
            ..
        } = &self.action
        {
            uuid::Uuid::parse_str(expected_incarnation_id)
                .map_err(|_| ControlError::InvalidIdentifier)?;
        }
        Ok(())
    }
}

pub struct TrustedControlKey {
    pub principal: String,
    pub verifying_key: VerifyingKey,
    pub capabilities: BTreeSet<ControlCapability>,
}

pub struct ControlAuthority {
    keys: BTreeMap<String, TrustedControlKey>,
}

impl ControlAuthority {
    pub fn new(keys: BTreeMap<String, TrustedControlKey>) -> Self {
        Self { keys }
    }

    fn authorize(&self, command: &SignedControlCommand) -> Result<(), ControlError> {
        if command.signing_algorithm != "ed25519" {
            return Err(ControlError::Authentication);
        }
        let trusted = self
            .keys
            .get(&command.signing_key_id)
            .ok_or(ControlError::Authentication)?;
        let signature_bytes =
            hex::decode(&command.signature).map_err(|_| ControlError::Authentication)?;
        let signature =
            Signature::from_slice(&signature_bytes).map_err(|_| ControlError::Authentication)?;
        trusted
            .verifying_key
            .verify(&command.signing_bytes()?, &signature)
            .map_err(|_| ControlError::Authentication)?;
        command.validate_public_fields()?;
        if command.schema != CONTROL_COMMAND_SCHEMA || trusted.principal != command.principal {
            return Err(ControlError::Authentication);
        }
        if !trusted.capabilities.contains(&command.action.capability()) {
            return Err(ControlError::Unauthorized);
        }
        Ok(())
    }
}

pub fn generate_runtime_instance_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

pub fn verifying_key_from_hex(value: &str) -> Result<VerifyingKey, ControlError> {
    let bytes: [u8; 32] = hex::decode(value)
        .map_err(|_| ControlError::Authentication)?
        .try_into()
        .map_err(|_| ControlError::Authentication)?;
    VerifyingKey::from_bytes(&bytes).map_err(|_| ControlError::Authentication)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlExit {
    Clean,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case")]
pub enum ControlOutcome {
    Snapshot { snapshot: Box<RuntimeSnapshot> },
    Submitted { work_result: DomainResult },
    Shutdown { exit: ControlExit },
    Restart { accepted: bool },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ControlResponse {
    pub schema: String,
    pub command_id: String,
    pub correlation_id: String,
    pub outcome: ControlOutcome,
}

#[async_trait]
pub trait LifecycleControl: Send + Sync {
    async fn shutdown(&self, grace: Duration) -> Result<KernelExit, ()>;

    async fn restart(&self, grace: Duration) -> Result<(), ()> {
        self.shutdown(grace).await.map(|_| ())
    }
}

#[async_trait]
impl LifecycleControl for KernelControl {
    async fn shutdown(&self, grace: Duration) -> Result<KernelExit, ()> {
        KernelControl::shutdown(self, grace).await.map_err(|_| ())
    }
}

struct CommandRecord {
    fingerprint: String,
    response: Option<ControlResponse>,
}

struct IdempotencyState {
    records: LruCache<String, CommandRecord>,
    terminal_action: Option<String>,
    admission_open: bool,
}

struct AcipReplayState {
    sequences_by_source: LruCache<String, u64>,
}

struct AcipSequenceReservation {
    source: String,
    sequence: u64,
    previous: Option<u64>,
}

#[derive(Default)]
struct ConversationSessions {
    sessions: BTreeMap<String, ConversationSession>,
}

struct ConversationSession {
    recipient_id: String,
    next_sequence: u64,
    dispatch_gate: Arc<ConversationDispatchGate>,
    turns: BTreeMap<String, ConversationTurn>,
}

struct ConversationDispatchGate {
    state: Mutex<ConversationDispatchGateState>,
    changed: tokio::sync::Notify,
}

struct ConversationDispatchGateState {
    next_sequence: u64,
    completed: BTreeSet<u64>,
}

impl ConversationDispatchGate {
    fn new() -> Self {
        Self {
            state: Mutex::new(ConversationDispatchGateState {
                next_sequence: 1,
                completed: BTreeSet::new(),
            }),
            changed: tokio::sync::Notify::new(),
        }
    }

    fn ready(&self, sequence: u64) -> bool {
        self.state
            .lock()
            .expect("conversation dispatch gate poisoned")
            .next_sequence
            == sequence
    }

    async fn wait_turn(
        &self,
        sequence: u64,
        deadline: tokio::time::Instant,
        cancellation: &CancellationToken,
    ) -> bool {
        loop {
            let changed = self.changed.notified();
            if self.ready(sequence) {
                return true;
            }
            let notified = tokio::select! {
                _ = cancellation.cancelled() => return false,
                result = tokio::time::timeout_at(deadline, changed) => result,
            };
            if notified.is_err() {
                return false;
            }
        }
    }

    fn complete(&self, sequence: u64) {
        let mut state = self
            .state
            .lock()
            .expect("conversation dispatch gate poisoned");
        state.completed.insert(sequence);
        while {
            let next_sequence = state.next_sequence;
            state.completed.remove(&next_sequence)
        } {
            state.next_sequence = state.next_sequence.saturating_add(1);
        }
        drop(state);
        self.changed.notify_waiters();
    }
}

struct ConversationTurn {
    fingerprint: String,
    correlation_id: String,
    sequence: u64,
    cancellation: CancellationToken,
    completion: tokio::sync::watch::Sender<Option<ObservatoryConversationResult>>,
    terminal: Option<ObservatoryConversationResult>,
}

struct ConversationDispatch {
    intent: ObservatoryConversationIntent,
    sequence: u64,
    cancellation: CancellationToken,
    dispatch_gate: Arc<ConversationDispatchGate>,
    work_id: String,
}

enum ConversationAcceptance {
    Dispatch {
        accepted: ObservatoryConversationResult,
        dispatch: ConversationDispatch,
    },
    Response(ObservatoryConversationResult),
}

pub struct ControlService<C> {
    instance_id: String,
    runtime_incarnation_id: String,
    recorder: RuntimeRecorder,
    lifecycle: C,
    authority: ControlAuthority,
    max_records: usize,
    idempotency: Mutex<IdempotencyState>,
    acip_replay: Mutex<AcipReplayState>,
    conversation_sessions: Mutex<ConversationSessions>,
    weather: Mutex<Option<ObservedWeather>>,
    weather_stale_after_millis: Mutex<u64>,
    observatory_bearer_digest: Mutex<Option<blake3::Hash>>,
    acip_write_bearer_digest: Mutex<Option<blake3::Hash>>,
    observatory_allowed_origins: BTreeSet<String>,
    agent_population: AgentPopulationFeed,
    control_addr: Mutex<SocketAddr>,
    public_base_url: Mutex<String>,
    canonical_ingress: Option<CanonicalIngress>,
    agent_roster_token_key: Mutex<[u8; 32]>,
    api_policy: Mutex<Option<ControlApiPolicy>>,
}

impl<C: LifecycleControl + 'static> ControlService<C> {
    pub fn new(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
    ) -> Self {
        Self::new_with_observatory_config_and_agents(
            instance_id,
            recorder,
            lifecycle,
            authority,
            max_records,
            std::iter::empty(),
            AgentPopulationFeed::empty(),
        )
    }

    pub fn new_with_observatory_config(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
        observatory_allowed_origins: impl IntoIterator<Item = String>,
    ) -> Self {
        Self::new_with_observatory_config_and_agents(
            instance_id,
            recorder,
            lifecycle,
            authority,
            max_records,
            observatory_allowed_origins,
            AgentPopulationFeed::empty(),
        )
    }

    pub fn new_with_observatory_config_and_agents(
        instance_id: impl Into<String>,
        recorder: RuntimeRecorder,
        lifecycle: C,
        authority: ControlAuthority,
        max_records: usize,
        observatory_allowed_origins: impl IntoIterator<Item = String>,
        agent_population: AgentPopulationFeed,
    ) -> Self {
        assert!(max_records > 0, "idempotency capacity must be non-zero");
        let instance_id = instance_id.into();
        assert!(
            is_safe_identifier(&instance_id),
            "runtime instance id must be bounded"
        );
        let observatory_allowed_origins = observatory_allowed_origins.into_iter().collect();
        Self {
            instance_id,
            runtime_incarnation_id: uuid::Uuid::new_v4().to_string(),
            recorder,
            lifecycle,
            authority,
            max_records,
            idempotency: Mutex::new(IdempotencyState {
                records: LruCache::unbounded(),
                terminal_action: None,
                admission_open: true,
            }),
            acip_replay: Mutex::new(AcipReplayState {
                sequences_by_source: LruCache::unbounded(),
            }),
            conversation_sessions: Mutex::new(ConversationSessions::default()),
            weather: Mutex::new(None),
            weather_stale_after_millis: Mutex::new(30_000),
            observatory_bearer_digest: Mutex::new(None),
            acip_write_bearer_digest: Mutex::new(None),
            observatory_allowed_origins,
            agent_population,
            control_addr: Mutex::new(SocketAddr::from(([127, 0, 0, 1], 0))),
            public_base_url: Mutex::new("https://localhost".to_owned()),
            canonical_ingress: None,
            agent_roster_token_key: Mutex::new(blake3::derive_key(
                "adl.runtime_v3.agent_roster.page_token.ephemeral.v1",
                uuid::Uuid::new_v4().as_bytes(),
            )),
            api_policy: Mutex::new(None),
        }
    }

    pub fn set_agent_roster_token_key(&self, key: [u8; 32]) {
        *self
            .agent_roster_token_key
            .lock()
            .expect("agent roster token key mutex poisoned") = key;
    }

    fn set_api_policy(&self, policy: ControlApiPolicy) {
        *self
            .api_policy
            .lock()
            .expect("control API policy mutex poisoned") = Some(policy);
    }

    fn api_policy(&self) -> ControlApiPolicy {
        self.api_policy
            .lock()
            .expect("control API policy mutex poisoned")
            .expect("control API policy validated before router startup")
    }

    pub fn with_canonical_ingress(mut self, ingress: CanonicalIngress) -> Self {
        self.canonical_ingress = Some(ingress);
        self
    }

    fn accept_conversation_intent(
        &self,
        intent: &ObservatoryConversationIntent,
    ) -> ConversationAcceptance {
        let outcome = |status, error, sequence| ObservatoryConversationResult {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status,
            conversation_id: intent.conversation_id.clone(),
            turn_id: intent.turn_id.clone(),
            recipient_id: intent.recipient_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            reply: None,
            accepted_sequence: None,
            turn_sequence: sequence,
            error: Some(error),
        };
        if intent.schema != OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA
            || !is_safe_identifier(&intent.conversation_id)
            || !is_safe_identifier(&intent.turn_id)
            || !is_safe_identifier(&intent.recipient_id)
            || !is_correlation_id(&intent.correlation_id)
            || intent.message.trim().is_empty()
            || intent.message.len() > 4_096
        {
            return ConversationAcceptance::Response(outcome(
                "refused",
                "invalid_conversation_intent",
                None,
            ));
        }
        let roster = match self.agent_roster_page(100, None, None) {
            Ok(roster) => roster,
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "agent_roster_unavailable",
                    None,
                ))
            }
        };
        let recipient = roster
            .sample
            .iter()
            .find(|agent| agent.id == intent.recipient_id);
        match recipient {
            None => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "unknown_recipient",
                    None,
                ))
            }
            Some(agent) if !agent.communication_eligible => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "recipient_unavailable",
                    None,
                ))
            }
            Some(_) => {}
        }
        let Some(ingress) = self.canonical_ingress.as_ref() else {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "conversation_ingress_unavailable",
                None,
            ));
        };
        let _ = ingress;
        let fingerprint = match serde_json::to_vec(intent) {
            Ok(bytes) => blake3::hash(&bytes).to_hex().to_string(),
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "invalid_conversation_intent",
                    None,
                ))
            }
        };
        let mut sessions = self
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        if !sessions.sessions.contains_key(&intent.conversation_id)
            && sessions.sessions.len() >= self.max_records
        {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "conversation_capacity_exhausted",
                None,
            ));
        }
        let session = sessions
            .sessions
            .entry(intent.conversation_id.clone())
            .or_insert_with(|| ConversationSession {
                recipient_id: intent.recipient_id.clone(),
                next_sequence: 0,
                dispatch_gate: Arc::new(ConversationDispatchGate::new()),
                turns: BTreeMap::new(),
            });
        if session.recipient_id != intent.recipient_id {
            return ConversationAcceptance::Response(outcome(
                "refused",
                "conversation_recipient_conflict",
                None,
            ));
        }
        if let Some(existing) = session.turns.get(&intent.turn_id) {
            if existing.fingerprint != fingerprint {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "conversation_conflict",
                    Some(existing.sequence),
                ));
            }
            return ConversationAcceptance::Response(existing.terminal.clone().unwrap_or_else(
                || {
                    outcome(
                        "accepted",
                        "conversation_in_flight",
                        Some(existing.sequence),
                    )
                },
            ));
        }
        if session.turns.len() >= self.max_records {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "conversation_capacity_exhausted",
                None,
            ));
        }
        let Some(sequence) = session.next_sequence.checked_add(1) else {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "conversation_sequence_exhausted",
                None,
            ));
        };
        session.next_sequence = sequence;
        let cancellation = CancellationToken::new();
        let (completion, _) = tokio::sync::watch::channel(None);
        session.turns.insert(
            intent.turn_id.clone(),
            ConversationTurn {
                fingerprint,
                correlation_id: intent.correlation_id.clone(),
                sequence,
                cancellation: cancellation.clone(),
                completion,
                terminal: None,
            },
        );
        let accepted = ObservatoryConversationResult {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: "accepted",
            conversation_id: intent.conversation_id.clone(),
            turn_id: intent.turn_id.clone(),
            recipient_id: intent.recipient_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            reply: None,
            accepted_sequence: None,
            turn_sequence: Some(sequence),
            error: None,
        };
        let work_id = format!(
            "conversation-{}",
            &blake3::hash(format!("{}:{}", intent.conversation_id, intent.turn_id).as_bytes())
                .to_hex()[..32]
        );
        ConversationAcceptance::Dispatch {
            accepted,
            dispatch: ConversationDispatch {
                intent: intent.clone(),
                sequence,
                cancellation,
                dispatch_gate: session.dispatch_gate.clone(),
                work_id,
            },
        }
    }

    async fn complete_conversation_dispatch(
        &self,
        dispatch: ConversationDispatch,
    ) -> ObservatoryConversationResult {
        let outcome = |status, error| ObservatoryConversationResult {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status,
            conversation_id: dispatch.intent.conversation_id.clone(),
            turn_id: dispatch.intent.turn_id.clone(),
            recipient_id: dispatch.intent.recipient_id.clone(),
            correlation_id: dispatch.intent.correlation_id.clone(),
            reply: None,
            accepted_sequence: None,
            turn_sequence: Some(dispatch.sequence),
            error: Some(error),
        };
        let payload = serde_json::to_vec(&serde_json::json!({
            "schema": "adl.runtime.local_agent_work.v1",
            "tasks": [{
                "op": "conversation_message",
                "recipient_id": dispatch.intent.recipient_id,
                "input": dispatch.intent.message,
            }],
        }));
        let deadline = tokio::time::Instant::now() + self.api_policy().websocket_auth_timeout;
        let turn_ready = dispatch
            .dispatch_gate
            .wait_turn(dispatch.sequence, deadline, &dispatch.cancellation)
            .await;
        let result = if !turn_ready {
            if dispatch.cancellation.is_cancelled() {
                outcome("cancelled", "conversation_cancelled")
            } else {
                outcome("timed_out", "conversation_timed_out")
            }
        } else {
            match (payload, self.canonical_ingress.as_ref()) {
                (Err(_), _) => outcome("refused", "invalid_conversation_intent"),
                (_, None) => outcome("failed", "conversation_ingress_unavailable"),
                (Ok(payload), Some(ingress)) => {
                    let submitted = tokio::time::timeout_at(
                        deadline,
                        ingress.submit_with_cancellation(
                            DomainWork {
                                schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
                                work_id: dispatch.work_id.clone(),
                                kind: "agent_runtime".to_owned(),
                                payload,
                            },
                            dispatch.intent.correlation_id.clone(),
                            dispatch.cancellation.clone(),
                        ),
                    )
                    .await;
                    if submitted.is_err() {
                        dispatch.cancellation.cancel();
                    }
                    match submitted {
                        Err(_) => outcome("timed_out", "conversation_timed_out"),
                        Ok(Err(_)) if dispatch.cancellation.is_cancelled() => {
                            outcome("cancelled", "conversation_cancelled")
                        }
                        Ok(Ok(result)) => {
                            let reply = result
                                .public_output
                                .as_ref()
                                .and_then(|output| output.get("message"))
                                .and_then(serde_json::Value::as_str)
                                .map(str::to_owned);
                            match reply {
                                Some(reply) => ObservatoryConversationResult {
                                    schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                                    status: "delivered",
                                    conversation_id: dispatch.intent.conversation_id.clone(),
                                    turn_id: dispatch.intent.turn_id.clone(),
                                    recipient_id: dispatch.intent.recipient_id.clone(),
                                    correlation_id: dispatch.intent.correlation_id.clone(),
                                    reply: Some(reply),
                                    accepted_sequence: Some(result.accepted_sequence),
                                    turn_sequence: Some(dispatch.sequence),
                                    error: None,
                                },
                                None => outcome("failed", "conversation_reply_unavailable"),
                            }
                        }
                        Ok(Err(IngressError::Saturated | IngressError::Closed)) => {
                            outcome("failed", "conversation_temporarily_unavailable")
                        }
                        Ok(Err(IngressError::UnsupportedKind)) => {
                            outcome("refused", "recipient_unavailable")
                        }
                        Ok(Err(IngressError::Conflict)) => {
                            outcome("refused", "conversation_conflict")
                        }
                        Ok(Err(_)) => outcome("failed", "conversation_failed"),
                    }
                }
            }
        };
        dispatch.dispatch_gate.complete(dispatch.sequence);
        if let Some(turn) = self
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned")
            .sessions
            .get_mut(&dispatch.intent.conversation_id)
            .and_then(|session| session.turns.get_mut(&dispatch.intent.turn_id))
        {
            turn.terminal = Some(result.clone());
            turn.completion.send_replace(Some(result.clone()));
        }
        result
    }

    async fn wait_for_conversation_terminal(
        &self,
        conversation_id: &str,
        turn_id: &str,
    ) -> Option<ObservatoryConversationResult> {
        loop {
            let mut completion = {
                let sessions = self
                    .conversation_sessions
                    .lock()
                    .expect("conversation sessions mutex poisoned");
                let turn = sessions.sessions.get(conversation_id)?.turns.get(turn_id)?;
                if let Some(terminal) = &turn.terminal {
                    return Some(terminal.clone());
                }
                turn.completion.subscribe()
            };
            if completion.changed().await.is_err() {
                return None;
            }
        }
    }

    fn cancel_conversation_turn(
        &self,
        cancel: &ObservatoryConversationCancel,
    ) -> ObservatoryConversationResult {
        let mut sessions = self
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        let Some(session) = sessions.sessions.get_mut(&cancel.conversation_id) else {
            return ObservatoryConversationResult::refused_cancel(
                cancel,
                "unknown_conversation_turn",
            );
        };
        let recipient_id = session.recipient_id.clone();
        let Some(turn) = session.turns.get_mut(&cancel.turn_id) else {
            return ObservatoryConversationResult::refused_cancel(
                cancel,
                "unknown_conversation_turn",
            );
        };
        if turn.correlation_id != cancel.correlation_id {
            return ObservatoryConversationResult::refused_cancel(
                cancel,
                "conversation_correlation_conflict",
            );
        }
        if let Some(terminal) = &turn.terminal {
            return terminal.clone();
        }
        turn.cancellation.cancel();
        ObservatoryConversationResult {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: "accepted",
            conversation_id: cancel.conversation_id.clone(),
            turn_id: cancel.turn_id.clone(),
            recipient_id,
            correlation_id: cancel.correlation_id.clone(),
            reply: None,
            accepted_sequence: None,
            turn_sequence: Some(turn.sequence),
            error: None,
        }
    }

    pub fn initialize_observability(
        &self,
        health: ObservabilityHealth,
    ) -> Vec<crate::BootstrapEvent> {
        self.recorder.initialize_observability(health)
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    pub fn set_weather_report(&self, report: WeatherHealthReport) {
        self.set_weather_report_at(report, now_unix_millis());
    }

    pub fn set_weather_report_at(&self, report: WeatherHealthReport, observed_at_unix_millis: u64) {
        *self.weather.lock().expect("weather mutex poisoned") = Some(ObservedWeather {
            report,
            observed_at_unix_millis,
        });
    }

    pub fn set_weather_stale_after(&self, duration: Duration) {
        let millis = u64::try_from(duration.as_millis())
            .unwrap_or(u64::MAX)
            .max(1);
        *self
            .weather_stale_after_millis
            .lock()
            .expect("weather staleness mutex poisoned") = millis;
    }

    pub fn set_observatory_bearer_token(&self, token: &str) -> Result<(), ControlError> {
        if !(32..=256).contains(&token.len()) || token.chars().any(char::is_whitespace) {
            return Err(ControlError::Authentication);
        }
        *self
            .observatory_bearer_digest
            .lock()
            .expect("observatory credential mutex poisoned") = Some(blake3::hash(token.as_bytes()));
        Ok(())
    }

    pub fn set_acip_write_bearer_token(&self, token: &str) -> Result<(), ControlError> {
        if !(32..=256).contains(&token.len()) || token.chars().any(char::is_whitespace) {
            return Err(ControlError::Authentication);
        }
        *self
            .acip_write_bearer_digest
            .lock()
            .expect("ACIP write credential mutex poisoned") = Some(blake3::hash(token.as_bytes()));
        Ok(())
    }

    fn observatory_token_authorized(&self, token: &str) -> bool {
        let Some(expected) = *self
            .observatory_bearer_digest
            .lock()
            .expect("observatory credential mutex poisoned")
        else {
            return false;
        };
        constant_time_eq(
            expected.as_bytes(),
            blake3::hash(token.as_bytes()).as_bytes(),
        )
    }

    fn acip_write_token_authorized(&self, token: &str) -> bool {
        let Some(expected) = *self
            .acip_write_bearer_digest
            .lock()
            .expect("ACIP write credential mutex poisoned")
        else {
            return false;
        };
        constant_time_eq(
            expected.as_bytes(),
            blake3::hash(token.as_bytes()).as_bytes(),
        )
    }

    pub fn set_control_addr(&self, address: SocketAddr) {
        *self
            .control_addr
            .lock()
            .expect("control address mutex poisoned") = address;
    }

    pub fn set_public_base_url(&self, public_base_url: &str) -> Result<(), ControlError> {
        if !is_safe_https_base(public_base_url) {
            return Err(ControlError::InvalidBounds);
        }
        *self
            .public_base_url
            .lock()
            .expect("public base URL mutex poisoned") = public_base_url.to_owned();
        Ok(())
    }

    pub async fn close_admission_and_drain(&self, deadline: Duration) -> Result<(), IngressError> {
        self.idempotency
            .lock()
            .expect("idempotency mutex poisoned")
            .admission_open = false;
        if let Some(ingress) = &self.canonical_ingress {
            ingress.close_and_drain(deadline).await?;
        }
        Ok(())
    }

    pub async fn serialize_terminal_checkpoint(
        &self,
        continuity: &mut LiveContinuity,
        deadline: Duration,
    ) -> Result<CheckpointManifest, String> {
        self.close_admission_and_drain(deadline)
            .await
            .map_err(|error| error.to_string())?;
        continuity
            .checkpoint(&self.recorder, deadline)
            .await
            .map_err(|error| error.to_string())
    }

    pub fn reopen_admission_if_no_terminal(&self) -> bool {
        let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
        if state.terminal_action.is_some() {
            return false;
        }
        state.admission_open = true;
        if let Some(ingress) = &self.canonical_ingress {
            ingress.reopen();
        }
        true
    }

    fn reserve_acip_sequence(
        &self,
        source: &str,
        sequence: u64,
    ) -> Option<AcipSequenceReservation> {
        if sequence == 0 {
            return None;
        }
        let mut state = self.acip_replay.lock().expect("ACIP replay mutex poisoned");
        let previous = state.sequences_by_source.get(source).copied();
        if let Some(previous) = previous {
            if sequence <= previous {
                return None;
            }
        } else {
            while state.sequences_by_source.len() >= self.max_records {
                state.sequences_by_source.pop_lru();
            }
        }
        state.sequences_by_source.put(source.to_owned(), sequence);
        Some(AcipSequenceReservation {
            source: source.to_owned(),
            sequence,
            previous,
        })
    }

    fn rollback_acip_sequence(&self, reservation: AcipSequenceReservation) {
        let mut state = self.acip_replay.lock().expect("ACIP replay mutex poisoned");
        if state.sequences_by_source.peek(&reservation.source).copied()
            != Some(reservation.sequence)
        {
            return;
        }
        match reservation.previous {
            Some(previous) => {
                state.sequences_by_source.put(reservation.source, previous);
            }
            None => {
                state.sequences_by_source.pop(&reservation.source);
            }
        }
    }

    async fn dispatch_acip_payload(&self, payload: &[u8]) -> serde_json::Value {
        let envelope = match decode_acip_envelope(payload) {
            Ok(envelope) => envelope,
            Err(reason) => {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "reason": reason,
                    "sequence_reserved": false
                });
            }
        };
        let Some(ingress) = &self.canonical_ingress else {
            return serde_json::json!({
                "schema": ACIP_WEBSOCKET_SCHEMA,
                "status": "rejected",
                "message_id": envelope.message_id,
                "reason": "canonical_ingress_unavailable",
                "sequence_reserved": false
            });
        };
        let Some(reservation) =
            self.reserve_acip_sequence(&envelope.source, envelope.monotonic_sequence)
        else {
            return serde_json::json!({
                "schema": ACIP_WEBSOCKET_SCHEMA,
                "status": "rejected",
                "message_id": envelope.message_id,
                "reason": "monotonic_sequence_must_advance",
                "sequence_reserved": false
            });
        };
        let work = DomainWork {
            schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: envelope.message_id.clone(),
            kind: envelope.route.clone(),
            payload: envelope.payload_json.as_bytes().to_vec(),
        };
        match ingress.submit(work, envelope.message_id.clone()).await {
            Ok(result) => serde_json::json!({
                "schema": ACIP_WEBSOCKET_SCHEMA,
                "status": "completed",
                "message_id": envelope.message_id,
                "accepted_sequence": result.accepted_sequence,
                "result_hash": result.result_hash,
                "sequence_reserved": true
            }),
            Err(error) => {
                self.rollback_acip_sequence(reservation);
                serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": error.to_string(),
                    "sequence_reserved": false
                })
            }
        }
    }

    pub fn observatory_feed(&self) -> ObservatoryFeed {
        let snapshot = self.recorder.snapshot();
        let observability_ready = matches!(snapshot.observability, ObservabilityHealth::Ready);
        let continuity_head = snapshot.continuity_head.clone();
        let events = self.recorder.events();
        let weather = self.weather.lock().expect("weather mutex poisoned").clone();
        let stale_after_millis = *self
            .weather_stale_after_millis
            .lock()
            .expect("weather staleness mutex poisoned");
        let now = now_unix_millis();
        let weather_freshness = weather.as_ref().map(|weather| {
            let observed_at_unix_millis = weather.observed_at_unix_millis;
            let age_millis = now.saturating_sub(observed_at_unix_millis);
            ObservatoryWeatherFreshness {
                observed_at_unix_millis,
                age_millis,
                stale_after_millis,
                stale: age_millis > stale_after_millis,
            }
        });
        let agents = self.agent_population.clone().with_runtime_snapshot_query(
            &snapshot,
            now,
            *self
                .agent_roster_token_key
                .lock()
                .expect("agent roster token key mutex poisoned"),
            AgentRosterQuery {
                page_size: 100,
                page_token: None,
                filter: None,
            },
        );
        ObservatoryFeed {
            schema: OBSERVATORY_FEED_SCHEMA.to_owned(),
            runtime_instance_id: self.instance_id.clone(),
            runtime_incarnation_id: self.runtime_incarnation_id.clone(),
            runtime_process_id: std::process::id(),
            default_runtime_changed: false,
            runtime_selection: "runtime_v3_explicit_opt_in".to_owned(),
            control: ObservatoryControlFeed {
                port: self
                    .control_addr
                    .lock()
                    .expect("control address mutex poisoned")
                    .port(),
                public_base_url: self
                    .public_base_url
                    .lock()
                    .expect("public base URL mutex poisoned")
                    .clone(),
                read_endpoint: "/v1/observatory".to_owned(),
                websocket_endpoint: OBSERVATORY_WS_PATH.to_owned(),
                websocket_full_duplex: true,
                websocket_acip_binary_schema: ACIP_WEBSOCKET_SCHEMA.to_owned(),
                signed_command_endpoint: "/v1/control".to_owned(),
                signed_commands_required_for_mutation: true,
                bearer_token_required_for_read: false,
                login_required_for_mutation: true,
                browser_mutation_authority: true,
            },
            health: ObservatoryHealthFeed {
                snapshot,
                observability_ready,
            },
            weather: weather.map(|weather| weather.report),
            weather_freshness,
            continuity: ObservatoryContinuityFeed {
                checkpoint: continuity_head,
            },
            ingress: self
                .canonical_ingress
                .as_ref()
                .map(CanonicalIngress::snapshot)
                .unwrap_or_default(),
            agents,
            proof: ObservatoryProofFeed {
                default_runtime_switch_authorized: false,
                runtime_v2_decommission_authorized: false,
                sidecar_required: false,
                vector_cloudwatch_route: "vector.runtime_v3_cloudwatch_emf".to_owned(),
            },
            events,
        }
    }

    pub fn readiness_report(&self) -> RuntimeReadinessReport {
        let feed = self.observatory_feed();
        let weather_freshness = feed.weather_freshness.clone();
        let weather_stale = weather_freshness
            .as_ref()
            .is_none_or(|freshness| freshness.stale);
        let mut degraded_reasons = Vec::new();
        if !feed.health.observability_ready {
            degraded_reasons.push("observability_not_ready".to_owned());
        }
        if weather_stale {
            degraded_reasons.push("weather_stale".to_owned());
        }
        RuntimeReadinessReport {
            schema: RUNTIME_READINESS_SCHEMA.to_owned(),
            ready: degraded_reasons.is_empty(),
            lifecycle: feed.health.snapshot.lifecycle,
            observability_ready: feed.health.observability_ready,
            runtime_instance_id: feed.runtime_instance_id,
            runtime_incarnation_id: feed.runtime_incarnation_id,
            runtime_process_id: feed.runtime_process_id,
            weather_freshness,
            degraded_reasons,
        }
    }

    pub fn agent_roster_page(
        &self,
        page_size: usize,
        page_token: Option<String>,
        filter: Option<String>,
    ) -> Result<AgentPopulationFeed, ControlError> {
        let snapshot = self.recorder.snapshot();
        let now = now_unix_millis();
        self.agent_population
            .clone()
            .try_with_runtime_snapshot_query(
                &snapshot,
                now,
                *self
                    .agent_roster_token_key
                    .lock()
                    .expect("agent roster token key mutex poisoned"),
                AgentRosterQuery {
                    page_size,
                    page_token,
                    filter,
                },
            )
            .map_err(|_| ControlError::InvalidBounds)
    }

    pub async fn execute(
        self: &Arc<Self>,
        command: SignedControlCommand,
    ) -> Result<ControlResponse, ControlError> {
        self.authority.authorize(&command)?;
        if command.runtime_instance_id != self.instance_id {
            return Err(ControlError::StaleRuntimeInstance);
        }
        if let ControlAction::Restart {
            expected_incarnation_id,
            ..
        } = &command.action
        {
            if expected_incarnation_id != &self.runtime_incarnation_id {
                return Err(ControlError::StaleRuntimeInstance);
            }
        }
        let fingerprint = command.fingerprint()?;
        {
            let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
            if let Some(record) = state.records.get(&command.command_id) {
                if record.fingerprint != fingerprint {
                    return Err(ControlError::IdempotencyConflict);
                }
                return record.response.clone().ok_or(ControlError::InFlight);
            }
            if !state.admission_open {
                return Err(ControlError::AdmissionClosed);
            }
            while state.records.len() >= self.max_records {
                let completed = state
                    .records
                    .iter()
                    .rev()
                    .find_map(|(id, record)| record.response.is_some().then(|| id.clone()));
                let Some(completed) = completed else {
                    return Err(ControlError::IdempotencyCapacity);
                };
                state.records.pop(&completed);
            }
            if matches!(
                command.action,
                ControlAction::Shutdown { .. } | ControlAction::Restart { .. }
            ) {
                if state.terminal_action.is_some() {
                    return Err(ControlError::LifecycleAlreadyRequested);
                }
                state.terminal_action = Some(command.command_id.clone());
                state.admission_open = false;
                if let Some(ingress) = &self.canonical_ingress {
                    ingress.close();
                }
            }
            state.records.put(
                command.command_id.clone(),
                CommandRecord {
                    fingerprint: fingerprint.clone(),
                    response: None,
                },
            );
        }

        let command_id = command.command_id.clone();
        let terminal = matches!(command.action, ControlAction::Shutdown { .. });
        let service = Arc::clone(self);
        let result = tokio::spawn(async move { service.execute_reserved(command).await })
            .await
            .map_err(|_| ControlError::Internal)?;
        if result.is_err() {
            let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
            state.records.pop(&command_id);
            if terminal && state.terminal_action.as_deref() == Some(&command_id) {
                state.terminal_action = None;
                state.admission_open = true;
                if let Some(ingress) = &self.canonical_ingress {
                    ingress.reopen();
                }
            }
        }
        result
    }

    async fn execute_reserved(
        &self,
        command: SignedControlCommand,
    ) -> Result<ControlResponse, ControlError> {
        let span = tracing::info_span!(
            "runtime_v3.control_command",
            command_id = %command.command_id,
            correlation_id = %command.correlation_id,
            principal = %command.principal,
        );
        let outcome = async {
            match command.action {
                ControlAction::Snapshot => Ok(ControlOutcome::Snapshot {
                    snapshot: Box::new(self.recorder.snapshot()),
                }),
                ControlAction::Submit { work } => {
                    let result = self
                        .canonical_ingress
                        .as_ref()
                        .ok_or(ControlError::AdmissionClosed)?
                        .submit(work, command.correlation_id.clone())
                        .await
                        .map_err(|error| match error {
                            IngressError::Invalid | IngressError::UnsupportedKind => {
                                ControlError::InvalidBounds
                            }
                            IngressError::Conflict => ControlError::IdempotencyConflict,
                            IngressError::Saturated | IngressError::Closed => {
                                ControlError::AdmissionClosed
                            }
                            IngressError::ExecutionFailed | IngressError::DrainTimeout => {
                                ControlError::Internal
                            }
                        })?;
                    Ok(ControlOutcome::Submitted {
                        work_result: result,
                    })
                }
                ControlAction::Shutdown { grace_millis } => {
                    let exit = self
                        .lifecycle
                        .shutdown(Duration::from_millis(grace_millis))
                        .await
                        .map(|exit| match exit {
                            KernelExit::Clean => ControlExit::Clean,
                            _ => ControlExit::Failed,
                        })
                        .unwrap_or(ControlExit::Failed);
                    Ok(ControlOutcome::Shutdown { exit })
                }
                ControlAction::Restart {
                    expected_incarnation_id,
                    grace_millis,
                } => {
                    debug_assert_eq!(expected_incarnation_id, self.runtime_incarnation_id);
                    self.lifecycle
                        .restart(Duration::from_millis(grace_millis))
                        .await
                        .map_err(|_| ControlError::Internal)?;
                    Ok(ControlOutcome::Restart { accepted: true })
                }
            }
        }
        .instrument(span)
        .await?;
        let response = ControlResponse {
            schema: CONTROL_RESPONSE_SCHEMA.to_owned(),
            command_id: command.command_id.clone(),
            correlation_id: command.correlation_id.clone(),
            outcome,
        };
        self.recorder.emit_correlated(
            None,
            crate::RuntimeEvent::ControlCommandCompleted,
            Some(&command.correlation_id),
        );
        let mut state = self.idempotency.lock().expect("idempotency mutex poisoned");
        state
            .records
            .get_mut(&command.command_id)
            .expect("reserved command record must exist")
            .response = Some(response.clone());
        Ok(response)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryControlFeed {
    pub port: u16,
    pub public_base_url: String,
    pub read_endpoint: String,
    pub websocket_endpoint: String,
    pub websocket_full_duplex: bool,
    pub websocket_acip_binary_schema: String,
    pub signed_command_endpoint: String,
    pub signed_commands_required_for_mutation: bool,
    pub bearer_token_required_for_read: bool,
    pub login_required_for_mutation: bool,
    pub browser_mutation_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryWeatherFreshness {
    pub observed_at_unix_millis: u64,
    pub age_millis: u64,
    pub stale_after_millis: u64,
    pub stale: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObservedWeather {
    report: WeatherHealthReport,
    observed_at_unix_millis: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryHealthFeed {
    pub snapshot: RuntimeSnapshot,
    pub observability_ready: bool,
}

pub const RUNTIME_READINESS_SCHEMA: &str = "adl.runtime_v3.readiness.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeReadinessReport {
    pub schema: String,
    pub ready: bool,
    pub lifecycle: LifecycleState,
    pub observability_ready: bool,
    pub runtime_instance_id: String,
    pub runtime_incarnation_id: String,
    pub runtime_process_id: u32,
    pub weather_freshness: Option<ObservatoryWeatherFreshness>,
    pub degraded_reasons: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryContinuityFeed {
    pub checkpoint: Option<crate::ContinuityHead>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentPopulationFeed {
    pub schema: String,
    pub revision: u64,
    pub scope: String,
    pub total_count: u64,
    pub rendered_sample_count: u64,
    pub has_more: bool,
    pub next_page_token: Option<String>,
    pub population_complete: bool,
    pub sample: Vec<AgentSample>,
}

impl AgentPopulationFeed {
    pub fn empty() -> Self {
        Self {
            schema: AGENT_ROSTER_PAGE_SCHEMA.to_owned(),
            revision: 0,
            scope: "local_runtime".to_owned(),
            total_count: 0,
            rendered_sample_count: 0,
            has_more: false,
            next_page_token: None,
            population_complete: false,
            sample: Vec::new(),
        }
    }

    pub fn resident_shepherd() -> Self {
        Self {
            sample: vec![AgentSample {
                id: "shepherd".to_owned(),
                label: "Shepherd".to_owned(),
                role: "resident shepherd".to_owned(),
                state: "unknown".to_owned(),
                detail: "Awaiting production Runtime admission".to_owned(),
                health: "unknown".to_owned(),
                availability: "unknown".to_owned(),
                activity: None,
                capabilities: vec!["conversation".to_owned()],
                location: Some("local_runtime".to_owned()),
                communication_eligible: false,
                observed_at_unix_millis: 0,
                freshness_deadline_unix_millis: 0,
                source_revision: "unobserved".to_owned(),
                provenance: "runtime_component_state".to_owned(),
            }],
            ..Self::empty()
        }
    }

    fn with_runtime_snapshot_query(
        self,
        snapshot: &RuntimeSnapshot,
        now_unix_millis: u64,
        token_key: [u8; 32],
        query: AgentRosterQuery,
    ) -> Self {
        self.clone()
            .try_with_runtime_snapshot_query(snapshot, now_unix_millis, token_key, query)
            .unwrap_or_else(|_| Self::empty())
    }

    fn try_with_runtime_snapshot_query(
        mut self,
        snapshot: &RuntimeSnapshot,
        now_unix_millis: u64,
        token_key: [u8; 32],
        query: AgentRosterQuery,
    ) -> Result<Self, crate::AgentRosterError> {
        let has_runtime_projection = self
            .sample
            .iter()
            .any(|agent| agent.provenance == "runtime_component_state");
        self.sample.retain_mut(|agent| {
            if agent.provenance != "runtime_component_state" {
                return true;
            }
            let Some(state) = snapshot.components.get(&ComponentId::new(&agent.id)) else {
                return false;
            };
            let Some(admission) = snapshot.agent_admissions.get(&agent.id) else {
                return false;
            };
            let (presence, health, availability, eligible, detail) = match state {
                RunningState::Running => (
                    AgentPresence::Ready,
                    "healthy",
                    "available",
                    true,
                    "Production Runtime component is running",
                ),
                RunningState::Starting | RunningState::Ready => (
                    AgentPresence::Unknown,
                    "starting",
                    "unavailable",
                    false,
                    "Production Runtime component is starting",
                ),
                RunningState::Restarting => (
                    AgentPresence::Migrating,
                    "recovering",
                    "unavailable",
                    false,
                    "Production Runtime component is restarting",
                ),
                RunningState::Degraded => (
                    AgentPresence::Degraded,
                    "degraded",
                    "unavailable",
                    false,
                    "Production Runtime component is degraded",
                ),
                RunningState::Stopping | RunningState::Stopped | RunningState::Failed => (
                    AgentPresence::Unreachable,
                    "unhealthy",
                    "unavailable",
                    false,
                    "Production Runtime component is not running",
                ),
            };
            agent.state = serde_json::to_value(presence)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_else(|| "unknown".to_owned());
            agent.health = health.to_owned();
            agent.availability = availability.to_owned();
            agent.communication_eligible = eligible;
            agent.detail = detail.to_owned();
            agent.observed_at_unix_millis = admission.observed_at_unix_millis;
            agent.freshness_deadline_unix_millis = admission.freshness_deadline_unix_millis;
            agent.source_revision = admission.source_revision.clone();
            true
        });
        if has_runtime_projection {
            let evidence = self
                .sample
                .iter()
                .map(AgentRuntimeEvidence::from)
                .collect::<Vec<_>>();
            let visible = evidence
                .iter()
                .map(|item| item.agent_id.clone())
                .collect::<BTreeSet<_>>();
            let Ok(roster) = AgentRoster::new(snapshot.revision.max(1), false, evidence, token_key)
            else {
                return Ok(Self::empty());
            };
            let page = roster.page(
                &AgentRosterPolicy {
                    policy_subject: "public-observatory".to_owned(),
                    visible_agent_ids: visible,
                    reveal_capabilities: true,
                    reveal_location: true,
                },
                query,
                now_unix_millis,
            )?;
            self.revision = page.revision;
            self.total_count = page.visible_count;
            self.rendered_sample_count = page.page_count;
            self.has_more = page.has_more;
            self.next_page_token = page.next_page_token;
            self.sample = page.agents.into_iter().map(AgentSample::from).collect();
        }
        Ok(self)
    }
}

impl From<&AgentSample> for AgentRuntimeEvidence {
    fn from(agent: &AgentSample) -> Self {
        Self {
            agent_id: agent.id.clone(),
            display_name: agent.label.clone(),
            public_role: agent.role.clone(),
            presence: match agent.state.as_str() {
                "ready" => AgentPresence::Ready,
                "busy" => AgentPresence::Busy,
                "sleeping" => AgentPresence::Sleeping,
                "degraded" => AgentPresence::Degraded,
                "unreachable" => AgentPresence::Unreachable,
                "migrating" => AgentPresence::Migrating,
                _ => AgentPresence::Unknown,
            },
            health: agent.health.clone(),
            availability: agent.availability.clone(),
            activity: agent.activity.clone(),
            capabilities: agent.capabilities.clone(),
            location: agent.location.clone(),
            communication_eligible: agent.communication_eligible,
            observed_at_unix_millis: agent.observed_at_unix_millis,
            freshness_deadline_unix_millis: agent.freshness_deadline_unix_millis,
            source_revision: agent.source_revision.clone(),
            provenance: agent.provenance.clone(),
        }
    }
}

impl From<AgentRosterEntry> for AgentSample {
    fn from(agent: AgentRosterEntry) -> Self {
        let state = serde_json::to_value(agent.presence)
            .ok()
            .and_then(|value| value.as_str().map(str::to_owned))
            .unwrap_or_else(|| "unknown".to_owned());
        Self {
            id: agent.id,
            label: agent.label,
            role: agent.role,
            state,
            detail: "Runtime-authorized local roster projection".to_owned(),
            health: agent.health,
            availability: agent.availability,
            activity: agent.activity,
            capabilities: agent.capabilities,
            location: agent.location,
            communication_eligible: agent.communication_eligible,
            observed_at_unix_millis: agent.observed_at_unix_millis,
            freshness_deadline_unix_millis: agent.freshness_deadline_unix_millis,
            source_revision: agent.source_revision,
            provenance: agent.provenance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentSample {
    pub id: String,
    pub label: String,
    pub role: String,
    pub state: String,
    pub detail: String,
    pub health: String,
    pub availability: String,
    pub activity: Option<String>,
    pub capabilities: Vec<String>,
    pub location: Option<String>,
    pub communication_eligible: bool,
    pub observed_at_unix_millis: u64,
    pub freshness_deadline_unix_millis: u64,
    pub source_revision: String,
    pub provenance: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryProofFeed {
    pub default_runtime_switch_authorized: bool,
    pub runtime_v2_decommission_authorized: bool,
    pub sidecar_required: bool,
    pub vector_cloudwatch_route: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryFeed {
    pub schema: String,
    pub runtime_instance_id: String,
    pub runtime_incarnation_id: String,
    pub runtime_process_id: u32,
    pub default_runtime_changed: bool,
    pub runtime_selection: String,
    pub control: ObservatoryControlFeed,
    pub health: ObservatoryHealthFeed,
    pub weather: Option<WeatherHealthReport>,
    pub weather_freshness: Option<ObservatoryWeatherFreshness>,
    pub continuity: ObservatoryContinuityFeed,
    pub ingress: crate::IngressSnapshot,
    pub agents: AgentPopulationFeed,
    pub proof: ObservatoryProofFeed,
    pub events: Vec<BootstrapEvent>,
}

pub async fn load_control_tls(
    config: &RuntimeTlsInitConfig,
) -> Result<axum_server::tls_rustls::RustlsConfig, ControlApiError> {
    crate::tls::load_axum_server_tls(&config.identity_paths(), &config.server_validation())
        .await
        .map_err(|error| ControlApiError::Tls(error.to_string()))
}

pub async fn serve_control_listener<C: LifecycleControl + 'static>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    api_policy: ControlApiPolicy,
) -> Result<(), ControlApiError> {
    serve_control_listener_until(service, listener, tls, api_policy, std::future::pending()).await
}

pub async fn serve_control_listener_until<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    api_policy: ControlApiPolicy,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    serve_control_listener_until_inner(service, listener, tls, api_policy, None, shutdown).await
}

pub async fn serve_control_listener_until_ready<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    api_policy: ControlApiPolicy,
    ready: tokio::sync::oneshot::Sender<SocketAddr>,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    serve_control_listener_until_inner(service, listener, tls, api_policy, Some(ready), shutdown)
        .await
}

async fn serve_control_listener_until_inner<C, F>(
    service: Arc<ControlService<C>>,
    listener: tokio::net::TcpListener,
    tls: axum_server::tls_rustls::RustlsConfig,
    api_policy: ControlApiPolicy,
    ready: Option<tokio::sync::oneshot::Sender<SocketAddr>>,
    shutdown: F,
) -> Result<(), ControlApiError>
where
    C: LifecycleControl + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    service.set_api_policy(api_policy);
    let address = listener
        .local_addr()
        .map_err(|error| ControlApiError::Bind(error.to_string()))?;
    service.set_control_addr(address);
    let listener = listener
        .into_std()
        .map_err(|error| ControlApiError::Bind(error.to_string()))?;
    let swagger_ui = SwaggerUi::new(API_DOCS_PATH).config(
        SwaggerConfig::new([
            SwaggerUrl::with_primary("Runtime Core", RUNTIME_OPENAPI_PATH, true),
            SwaggerUrl::new("Observatory", OBSERVATORY_OPENAPI_PATH),
        ])
        .validator_url("none"),
    );
    let observatory_swagger_ui = SwaggerUi::new(OBSERVATORY_API_DOCS_PATH).config(
        SwaggerConfig::new([SwaggerUrl::with_primary(
            "Observatory",
            OBSERVATORY_OPENAPI_PATH,
            true,
        )])
        .validator_url("none"),
    );
    let router = Router::new()
        .route(RUNTIME_HEALTH_PATH, get(runtime_health_handler::<C>))
        .route(
            RUNTIME_READY_PATH,
            get(runtime_ready_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(RUNTIME_METRICS_PATH, get(runtime_metrics_handler::<C>))
        .route(ACIP_WS_PATH, get(acip_ws_handler::<C>))
        .route(RUNTIME_OPENAPI_PATH, get(runtime_openapi_handler))
        .route(OBSERVATORY_OPENAPI_PATH, get(observatory_openapi_handler))
        .route(
            "/v1/observatory",
            get(observatory_feed_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(
            "/v1/agents",
            get(agent_roster_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(OBSERVATORY_WS_PATH, get(observatory_ws_handler::<C>))
        .route(
            "/v1/control",
            post(control_handler::<C>)
                .options(control_preflight_handler::<C>)
                .layer(DefaultBodyLimit::max(api_policy.control_max_body_bytes)),
        )
        .merge(swagger_ui)
        .merge(observatory_swagger_ui)
        .with_state(service);
    let handle = axum_server::Handle::new();
    let shutdown_handle = handle.clone();
    let shutdown_task = tokio::spawn(async move {
        shutdown.await;
        shutdown_handle.graceful_shutdown(Some(api_policy.shutdown_grace));
    });
    let server = axum_server::from_tcp_rustls(listener, tls)
        .map_err(|error| ControlApiError::Bind(error.to_string()))?
        .handle(handle.clone());
    let readiness_task = ready.map(|ready| {
        let readiness_handle = handle.clone();
        tokio::spawn(async move {
            if let Some(address) = readiness_handle.listening().await {
                let _ = ready.send(address);
            }
        })
    });
    let result = server
        .serve(router.into_make_service())
        .await
        .map_err(|error| ControlApiError::Serve(error.to_string()));
    shutdown_task.abort();
    if let Some(task) = readiness_task {
        task.abort();
    }
    result
}

async fn runtime_openapi_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/json")],
        RUNTIME_OPENAPI_DOCUMENT,
    )
}

async fn observatory_openapi_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/json")],
        OBSERVATORY_OPENAPI_DOCUMENT,
    )
}

async fn runtime_health_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
) -> Response {
    Json(service.observatory_feed().health).into_response()
}

async fn runtime_ready_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    let report = service.readiness_report();
    let status = if report.ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    observatory_json(status, report, allowed_origin)
}

async fn runtime_metrics_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
) -> Response {
    Json(service.recorder.snapshot().observability_pipeline).into_response()
}

async fn acip_ws_handler<C: LifecycleControl + 'static>(
    ws: WebSocketUpgrade,
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let Some(bearer_token) = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| service.acip_write_token_authorized(token))
        .map(str::to_owned)
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let api_policy = service.api_policy();
    ws.max_frame_size(api_policy.websocket_max_frame_bytes)
        .max_message_size(api_policy.websocket_max_frame_bytes)
        .on_upgrade(move |socket| acip_ws_session(socket, service, bearer_token))
}

async fn acip_ws_session<C: LifecycleControl + 'static>(
    mut socket: WebSocket,
    service: Arc<ControlService<C>>,
    bearer_token: String,
) {
    let authenticated = serde_json::json!({
        "schema": ACIP_WEBSOCKET_SCHEMA,
        "event": "authenticated",
        "path": ACIP_WS_PATH,
        "bidirectional": true
    });
    if socket
        .send(Message::Text(authenticated.to_string().into()))
        .await
        .is_err()
    {
        return;
    }

    while let Some(message) = socket.recv().await {
        if !service.acip_write_token_authorized(&bearer_token) {
            let _ = socket
                .send(Message::Close(Some(CloseFrame {
                    code: close_code::POLICY,
                    reason: "credential_revoked".into(),
                })))
                .await;
            return;
        }
        match message {
            Ok(Message::Binary(payload)) => {
                let response = service.dispatch_acip_payload(&payload).await;
                if socket
                    .send(Message::Text(response.to_string().into()))
                    .await
                    .is_err()
                {
                    return;
                }
            }
            Ok(Message::Ping(payload)) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    return;
                }
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => return,
            Ok(Message::Text(_)) | Err(_) => {
                let _ = socket
                    .send(Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "binary_acip_frame_required".into(),
                    })))
                    .await;
                return;
            }
        }
    }
}

async fn observatory_feed_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    observatory_json(StatusCode::OK, service.observatory_feed(), allowed_origin)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentRosterHttpQuery {
    #[serde(default = "default_roster_page_size")]
    page_size: usize,
    page_token: Option<String>,
    filter: Option<String>,
}

fn default_roster_page_size() -> usize {
    50
}

async fn agent_roster_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    Query(query): Query<AgentRosterHttpQuery>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    match service.agent_roster_page(query.page_size, query.page_token, query.filter) {
        Ok(page) => observatory_json(StatusCode::OK, page, allowed_origin),
        Err(_) => observatory_json(
            StatusCode::BAD_REQUEST,
            serde_json::json!({
                "schema": "adl.runtime_v3.agent_roster_error.v1",
                "code": "invalid_roster_query"
            }),
            allowed_origin,
        ),
    }
}

async fn observatory_ws_handler<C: LifecycleControl + 'static>(
    ws: WebSocketUpgrade,
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    if headers.contains_key(header::ORIGIN) && allowed_origin(&service, &headers).is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    let api_policy = service.api_policy();
    ws.max_frame_size(api_policy.websocket_max_frame_bytes)
        .max_message_size(api_policy.websocket_max_frame_bytes)
        .on_upgrade(move |socket| observatory_ws_session(socket, service))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryWsAuth {
    schema: String,
    bearer_token: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryConversationIntent {
    schema: String,
    conversation_id: String,
    turn_id: String,
    recipient_id: String,
    correlation_id: String,
    message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryConversationCancel {
    schema: String,
    conversation_id: String,
    turn_id: String,
    correlation_id: String,
}

#[derive(Clone, Serialize)]
struct ObservatoryConversationResult {
    schema: &'static str,
    status: &'static str,
    conversation_id: String,
    turn_id: String,
    recipient_id: String,
    correlation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turn_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

impl ObservatoryConversationResult {
    fn refused_cancel(cancel: &ObservatoryConversationCancel, error: &'static str) -> Self {
        Self {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: "refused",
            conversation_id: cancel.conversation_id.clone(),
            turn_id: cancel.turn_id.clone(),
            recipient_id: String::new(),
            correlation_id: cancel.correlation_id.clone(),
            reply: None,
            accepted_sequence: None,
            turn_sequence: None,
            error: Some(error),
        }
    }
}

#[derive(Serialize)]
struct ObservatoryWsControlResult {
    schema: &'static str,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    command_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<ControlResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

async fn observatory_ws_session<C: LifecycleControl + 'static>(
    mut socket: WebSocket,
    service: Arc<ControlService<C>>,
) {
    let api_policy = service.api_policy();
    let mut bearer_token: Option<String> = None;
    let (conversation_results_tx, mut conversation_results_rx) =
        tokio::sync::mpsc::unbounded_channel::<ObservatoryConversationResult>();
    let mut refresh = tokio::time::interval(api_policy.websocket_refresh);
    refresh.tick().await;
    let Ok(initial_feed) = serde_json::to_string(&service.observatory_feed()) else {
        return;
    };
    if socket
        .send(Message::Text(initial_feed.into()))
        .await
        .is_err()
    {
        return;
    }

    loop {
        tokio::select! {
            Some(result) = conversation_results_rx.recv() => {
                let Ok(payload) = serde_json::to_string(&result) else {
                    break;
                };
                if socket.send(Message::Text(payload.into())).await.is_err() {
                    break;
                }
            }
            _ = refresh.tick() => {
                if bearer_token.as_deref().is_some_and(|token| !service.observatory_token_authorized(token)) {
                    bearer_token = None;
                    let revoked = ObservatoryWsControlResult {
                        schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                        status: "rejected",
                        command_id: None,
                        correlation_id: None,
                        response: None,
                        error: Some("credential_revoked"),
                    };
                    let Ok(payload) = serde_json::to_string(&revoked) else {
                        break;
                    };
                    if socket.send(Message::Text(payload.into())).await.is_err() {
                        break;
                    }
                }
                let Ok(payload) = serde_json::to_string(&service.observatory_feed()) else {
                    break;
                };
                if socket.send(Message::Text(payload.into())).await.is_err() {
                    break;
                }
            }
            message = socket.recv() => match message {
                Some(Ok(Message::Ping(payload))) => {
                    if socket.send(Message::Pong(payload)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(Message::Pong(_))) => {}
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Text(payload))) => {
                    if let Ok(auth) = serde_json::from_str::<ObservatoryWsAuth>(&payload) {
                        let authorized = auth.schema == OBSERVATORY_WS_AUTH_SCHEMA
                            && service.observatory_token_authorized(&auth.bearer_token);
                        bearer_token = authorized.then_some(auth.bearer_token);
                        let result = ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: if authorized { "authenticated" } else { "rejected" },
                            command_id: None,
                            correlation_id: None,
                            response: None,
                            error: (!authorized).then_some("authentication_failed"),
                        };
                        let Ok(payload) = serde_json::to_string(&result) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                        continue;
                    }
                    if bearer_token
                        .as_deref()
                        .is_some_and(|token| !service.observatory_token_authorized(token))
                    {
                        bearer_token = None;
                    }
                    if let Ok(intent) = serde_json::from_str::<ObservatoryConversationIntent>(&payload) {
                        let result = if bearer_token.is_none() {
                            ConversationAcceptance::Response(ObservatoryConversationResult {
                                schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                                status: "refused",
                                conversation_id: intent.conversation_id.clone(),
                                turn_id: intent.turn_id.clone(),
                                recipient_id: intent.recipient_id.clone(),
                                correlation_id: intent.correlation_id.clone(),
                                reply: None,
                                accepted_sequence: None,
                                turn_sequence: None,
                                error: Some("write_authentication_required"),
                            })
                        } else {
                            service.accept_conversation_intent(&intent)
                        };
                        let (response, dispatch) = match result {
                            ConversationAcceptance::Dispatch { accepted, dispatch } => {
                                (accepted, Some(dispatch))
                            }
                            ConversationAcceptance::Response(response) => (response, None),
                        };
                        let attach_to_in_flight = response.status == "accepted"
                            && response.error == Some("conversation_in_flight");
                        let conversation_id = response.conversation_id.clone();
                        let turn_id = response.turn_id.clone();
                        let Ok(payload) = serde_json::to_string(&response) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                        if let Some(dispatch) = dispatch {
                            let service = service.clone();
                            let results = conversation_results_tx.clone();
                            tokio::spawn(async move {
                                let result = service.complete_conversation_dispatch(dispatch).await;
                                let _ = results.send(result);
                            });
                        } else if attach_to_in_flight {
                            let service = service.clone();
                            let results = conversation_results_tx.clone();
                            tokio::spawn(async move {
                                if let Some(result) = service
                                    .wait_for_conversation_terminal(&conversation_id, &turn_id)
                                    .await
                                {
                                    let _ = results.send(result);
                                }
                            });
                        }
                        continue;
                    }
                    if let Ok(cancel) = serde_json::from_str::<ObservatoryConversationCancel>(&payload) {
                        let result = if bearer_token.is_none() {
                            ObservatoryConversationResult::refused_cancel(&cancel, "write_authentication_required")
                        } else if cancel.schema != OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA
                            || !is_safe_identifier(&cancel.conversation_id)
                            || !is_safe_identifier(&cancel.turn_id)
                            || !is_correlation_id(&cancel.correlation_id)
                        {
                            ObservatoryConversationResult::refused_cancel(&cancel, "invalid_conversation_cancel")
                        } else {
                            service.cancel_conversation_turn(&cancel)
                        };
                        let Ok(payload) = serde_json::to_string(&result) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                        continue;
                    }
                    let command = serde_json::from_str::<SignedControlCommand>(&payload);
                    let (command_id, correlation_id) = command
                        .as_ref()
                        .map(|command| {
                            (
                                is_safe_identifier(&command.command_id)
                                    .then(|| command.command_id.clone()),
                                is_correlation_id(&command.correlation_id)
                                    .then(|| command.correlation_id.clone()),
                            )
                        })
                        .unwrap_or((None, None));
                    let result = if bearer_token.is_none() {
                        ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: "rejected",
                            command_id,
                            correlation_id,
                            response: None,
                            error: Some("write_authentication_required"),
                        }
                    } else {
                        match command {
                        Ok(command) => match service.execute(command).await {
                            Ok(response) => ObservatoryWsControlResult {
                                schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                                status: "accepted",
                                command_id,
                                correlation_id,
                                response: Some(response),
                                error: None,
                            },
                            Err(error) => ObservatoryWsControlResult {
                                schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                                status: "rejected",
                                command_id,
                                correlation_id,
                                response: None,
                                error: Some(control_error_code(&error)),
                            },
                        },
                        Err(_) => ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: "rejected",
                            command_id: None,
                            correlation_id: None,
                            response: None,
                            error: Some("invalid_request"),
                        },
                    }};
                    let Ok(payload) = serde_json::to_string(&result) else {
                        break;
                    };
                    if socket.send(Message::Text(payload.into())).await.is_err() {
                        break;
                    }
                }
                Some(Ok(Message::Binary(_))) => {
                    let _ = socket.send(Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "observatory_binary_frames_unsupported".into(),
                    }))).await;
                    break;
                }
                Some(Err(_)) => {
                    let _ = socket.send(Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "unsupported_websocket_frame".into(),
                    }))).await;
                    break;
                }
            }
        }
    }
}

async fn observatory_preflight_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let Some(origin) = allowed_origin(&service, &headers) else {
        return StatusCode::FORBIDDEN.into_response();
    };
    let mut response = StatusCode::NO_CONTENT.into_response();
    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET"),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Authorization"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
        .headers_mut()
        .insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

async fn control_preflight_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
) -> Response {
    let Some(origin) = allowed_origin(&service, &headers) else {
        return StatusCode::FORBIDDEN.into_response();
    };
    let mut response = StatusCode::NO_CONTENT.into_response();
    response
        .headers_mut()
        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("POST"),
    );
    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type, Authorization"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
        .headers_mut()
        .insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

async fn control_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let allowed_origin = if headers.contains_key(header::ORIGIN) {
        match allowed_origin(&service, &headers) {
            Some(origin) => Some(origin),
            None => return StatusCode::FORBIDDEN.into_response(),
        }
    } else {
        None
    };
    let command = match serde_json::from_slice::<SignedControlCommand>(&body) {
        Ok(command) => command,
        Err(_) => {
            return control_error_response(
                ControlError::Encoding("invalid request".into()),
                allowed_origin,
            )
        }
    };
    match service.execute(command).await {
        Ok(response) => observatory_json(StatusCode::OK, response, allowed_origin),
        Err(error) => control_error_response(error, allowed_origin),
    }
}

fn control_error_response(error: ControlError, allowed_origin: Option<HeaderValue>) -> Response {
    let status = match &error {
        ControlError::Authentication => StatusCode::UNAUTHORIZED,
        ControlError::Unauthorized => StatusCode::FORBIDDEN,
        ControlError::IdempotencyConflict
        | ControlError::InFlight
        | ControlError::LifecycleAlreadyRequested => StatusCode::CONFLICT,
        ControlError::AdmissionClosed
        | ControlError::IdempotencyCapacity
        | ControlError::Internal => StatusCode::SERVICE_UNAVAILABLE,
        ControlError::StaleRuntimeInstance => StatusCode::GONE,
        _ => StatusCode::BAD_REQUEST,
    };
    let payload = ControlErrorPayload {
        schema: "adl.runtime.control_error.v1",
        code: control_error_code(&error),
    };
    observatory_json(status, payload, allowed_origin)
}

fn control_error_code(error: &ControlError) -> &'static str {
    match error {
        ControlError::Authentication => "authentication_failed",
        ControlError::Unauthorized => "unauthorized",
        ControlError::IdempotencyConflict
        | ControlError::InFlight
        | ControlError::LifecycleAlreadyRequested => "idempotency_conflict",
        ControlError::AdmissionClosed
        | ControlError::IdempotencyCapacity
        | ControlError::Internal => "temporarily_unavailable",
        ControlError::StaleRuntimeInstance => "stale_runtime_instance",
        _ => "invalid_request",
    }
}

fn allowed_origin<C: LifecycleControl + 'static>(
    service: &ControlService<C>,
    headers: &HeaderMap,
) -> Option<HeaderValue> {
    let origin = headers
        .get(header::ORIGIN)
        .and_then(|value| value.to_str().ok())?;
    service
        .observatory_allowed_origins
        .contains(origin)
        .then(|| HeaderValue::from_str(origin).ok())
        .flatten()
}

fn cors_json<T: Serialize>(
    status: StatusCode,
    payload: T,
    allowed_origin: Option<HeaderValue>,
) -> Response {
    let mut response = (status, Json(payload)).into_response();
    if let Some(origin) = allowed_origin {
        response
            .headers_mut()
            .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
    }
    response
        .headers_mut()
        .insert(header::VARY, HeaderValue::from_static("Origin"));
    response
}

fn observatory_json<T: Serialize>(
    status: StatusCode,
    payload: T,
    allowed_origin: Option<HeaderValue>,
) -> Response {
    let mut response = cors_json(status, payload, allowed_origin);
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

#[derive(Serialize)]
struct ControlErrorPayload {
    schema: &'static str,
    code: &'static str,
}

fn now_unix_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .fold(0_u8, |difference, (left, right)| {
                difference | (left ^ right)
            })
            == 0
}

pub fn write_payload(
    mut stdout: impl Write,
    response: &ControlResponse,
) -> Result<(), ControlError> {
    serde_json::to_writer(&mut stdout, response)
        .map_err(|error| ControlError::Encoding(error.to_string()))?;
    writeln!(stdout).map_err(|error| ControlError::Io(error.to_string()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlObservabilityEvent {
    SnapshotCompleted,
    CommandRejected,
}

impl ControlObservabilityEvent {
    fn code(self) -> &'static str {
        match self {
            Self::SnapshotCompleted => "snapshot_completed",
            Self::CommandRejected => "command_rejected",
        }
    }
}

pub fn write_observability_event(
    mut stderr: impl Write,
    event: ControlObservabilityEvent,
    correlation_id: &str,
) -> Result<(), ControlError> {
    let correlation = is_correlation_id(correlation_id)
        .then_some(correlation_id)
        .ok_or(ControlError::InvalidIdentifier)?;
    let event = event.code();
    writeln!(
        stderr,
        "adl_event schema=adl.runtime.control_event.v1 event={event} correlation_id={correlation}"
    )
    .map_err(|error| ControlError::Io(error.to_string()))
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
}

fn is_safe_https_base(value: &str) -> bool {
    value.starts_with("https://")
        && value.len() <= 2_048
        && !value.ends_with('/')
        && !value.contains(['\r', '\n', '\t', ' ', '?', '#'])
}

fn is_correlation_id(value: &str) -> bool {
    value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ControlError {
    #[error("control authentication failed")]
    Authentication,
    #[error("control principal is not authorized for this action")]
    Unauthorized,
    #[error("control command contains an invalid identifier")]
    InvalidIdentifier,
    #[error("control command bounds are outside the supported range")]
    InvalidBounds,
    #[error("control command targets a stale runtime instance")]
    StaleRuntimeInstance,
    #[error("idempotency key was reused for a different command")]
    IdempotencyConflict,
    #[error("idempotent command is already in flight")]
    InFlight,
    #[error("control idempotency capacity is exhausted")]
    IdempotencyCapacity,
    #[error("control command admission is temporarily closed")]
    AdmissionClosed,
    #[error("a terminal lifecycle action has already been requested")]
    LifecycleAlreadyRequested,
    #[error("control execution failed internally")]
    Internal,
    #[error("control encoding failed: {0}")]
    Encoding(String),
    #[error("control output failed: {0}")]
    Io(String),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ControlApiError {
    #[error("control API policy is missing or contains a zero operational bound")]
    MissingPolicy,
    #[error("control API bind failed: {0}")]
    Bind(String),
    #[error("control API TLS configuration failed: {0}")]
    Tls(String),
    #[error("control API server failed: {0}")]
    Serve(String),
}

#[cfg(test)]
mod conversation_dispatch_gate_tests {
    use super::*;

    #[tokio::test]
    async fn later_turn_cannot_overtake_an_unfinished_earlier_sequence() {
        let gate = Arc::new(ConversationDispatchGate::new());
        let cancellation = CancellationToken::new();
        let later_gate = gate.clone();
        let later_cancellation = cancellation.clone();
        let mut later = tokio::spawn(async move {
            later_gate
                .wait_turn(
                    2,
                    tokio::time::Instant::now() + Duration::from_secs(1),
                    &later_cancellation,
                )
                .await
        });

        assert!(tokio::time::timeout(Duration::from_millis(20), &mut later)
            .await
            .is_err());
        assert!(gate.ready(1));
        gate.complete(1);
        assert!(later.await.unwrap());
    }
}
