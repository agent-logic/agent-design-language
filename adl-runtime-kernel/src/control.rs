use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::OpenOptions,
    future::Future,
    io::Write,
    net::SocketAddr,
    path::Path,
    sync::Arc,
    sync::{Mutex, RwLock},
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

use crate::layer8_authority::{
    AuthorityDecision, Layer8Action, Layer8ConversationAuthority, Layer8SignedExchange,
    RefusalReason, SignedIdentityMessage,
};

use crate::{
    conversation_rooms::{
        GovernedRoom, GovernedRoomDeliveryState, GovernedRoomParticipant,
        GovernedRoomParticipantState, GovernedRoomRoute, GovernedRoomTurnIntent,
        GOVERNED_ROOM_ROUTE_SCHEMA,
    },
    decode_acip_envelope, AgentRosterEntry, AgentRosterQuery, CanonicalIngress, CheckpointManifest,
    DomainResult, DomainWork, IngressError, KernelControl, KernelExit, LiveContinuity,
    ObservabilityHealth, RuntimeRecorder, RuntimeSnapshot, RuntimeTlsInitConfig,
    WeatherHealthReport, ACIP_WEBSOCKET_SCHEMA,
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
pub const RECIPIENT_ACKNOWLEDGEMENT_PATH: &str = "/v1/layer8/recipient-acknowledgement";
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
pub const OBSERVATORY_WS_GOVERNED_ROOM_INTENT_SCHEMA: &str =
    "adl.runtime_v3.observatory_governed_room_intent.v1";
pub const RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_REQUEST_SCHEMA: &str =
    "adl.runtime_v3.layer8.recipient_acknowledgement_request.v1";
pub const RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_RESPONSE_SCHEMA: &str =
    "adl.runtime_v3.layer8.recipient_acknowledgement_response.v1";
pub const CONTROL_MAX_BODY_BYTES: usize = 64 * 1024;
const OBSERVATORY_CONVERSATION_RESULT_QUEUE_CAPACITY: usize = 32;
const RUNTIME_OPENAPI_DOCUMENT: &str = include_str!("../../docs/api/runtime-v3/v1/openapi.json");
const OBSERVATORY_OPENAPI_DOCUMENT: &str =
    include_str!("../../docs/api/runtime-v3/v1/observatory.openapi.json");

#[path = "control/feeds.rs"]
mod feeds;
#[path = "control/replay.rs"]
mod replay;
use feeds::ObservedWeather;
pub use feeds::{
    AgentPopulationFeed, AgentSample, ObservatoryContinuityFeed, ObservatoryControlFeed,
    ObservatoryFeed, ObservatoryHealthFeed, ObservatoryProofFeed, ObservatoryWeatherFreshness,
    RuntimeReadinessReport, RUNTIME_READINESS_SCHEMA,
};
#[cfg(test)]
use replay::ACIP_MAX_SEQUENCE_ADVANCE;
use replay::{
    commit_replay_sequence, reserve_replay_sequence, rollback_replay_sequence, AcipReplayDomain,
    AcipReplayState, AcipSequenceReservation,
};

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

pub fn load_or_create_runtime_instance_id(state_root: &Path) -> std::io::Result<String> {
    let path = state_root.join("runtime-instance-id");
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(mut file) => {
            let instance_id = generate_runtime_instance_id();
            file.write_all(instance_id.as_bytes())?;
            file.write_all(b"\n")?;
            file.sync_all()?;
            Ok(instance_id)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = std::fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "runtime instance identity must be a regular file",
                ));
            }
            let instance_id = std::fs::read_to_string(path)?.trim().to_owned();
            if instance_id.len() != 32 || !instance_id.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "runtime instance identity is invalid",
                ));
            }
            Ok(instance_id)
        }
        Err(error) => Err(error),
    }
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

#[derive(Default)]
struct ConversationSessions {
    sessions: BTreeMap<String, ConversationSession>,
    next_sequence: u64,
}

struct ConversationSession {
    sequence: u64,
    recipient_id: String,
    next_sequence: u64,
    dispatch_gate: Arc<ConversationDispatchGate>,
    turns: BTreeMap<String, ConversationTurn>,
}

impl ConversationSessions {
    fn retain_capacity_for_new_session(&mut self, max_records: usize) -> bool {
        if self.sessions.len() < max_records {
            return true;
        }

        let oldest_terminal_session = self
            .sessions
            .iter()
            .filter(|(_, session)| session.turns.values().all(|turn| turn.terminal.is_some()))
            .min_by_key(|(_, session)| session.sequence)
            .map(|(conversation_id, _)| conversation_id.clone());
        let Some(conversation_id) = oldest_terminal_session else {
            return false;
        };
        self.sessions.remove(&conversation_id);
        true
    }
}

impl ConversationSession {
    fn retain_capacity_for_new_turn(&mut self, max_records: usize) -> bool {
        if self.turns.len() < max_records {
            return true;
        }

        let oldest_terminal_turn = self
            .turns
            .iter()
            .filter(|(_, turn)| turn.terminal.is_some())
            .min_by_key(|(_, turn)| turn.sequence)
            .map(|(turn_id, _)| turn_id.clone());
        let Some(turn_id) = oldest_terminal_turn else {
            return false;
        };
        self.turns.remove(&turn_id);
        true
    }
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

#[cfg(test)]
pub(crate) struct ConversationAttachmentTestHook {
    conversation_id: String,
    turn_id: String,
    first_intent_seen: Mutex<bool>,
    duplicate_observed: tokio::sync::Notify,
    allow_duplicate: tokio::sync::Semaphore,
    attachment_ready: tokio::sync::Notify,
    allow_timeout: tokio::sync::Semaphore,
}

#[cfg(test)]
impl ConversationAttachmentTestHook {
    pub(crate) fn new(conversation_id: &str, turn_id: &str) -> Arc<Self> {
        Arc::new(Self {
            conversation_id: conversation_id.to_owned(),
            turn_id: turn_id.to_owned(),
            first_intent_seen: Mutex::new(false),
            duplicate_observed: tokio::sync::Notify::new(),
            allow_duplicate: tokio::sync::Semaphore::new(0),
            attachment_ready: tokio::sync::Notify::new(),
            allow_timeout: tokio::sync::Semaphore::new(0),
        })
    }

    fn matches(&self, conversation_id: &str, turn_id: &str) -> bool {
        self.conversation_id == conversation_id && self.turn_id == turn_id
    }

    async fn observe_intent(&self) {
        let duplicate = {
            let mut seen = self
                .first_intent_seen
                .lock()
                .expect("conversation attachment test hook poisoned");
            std::mem::replace(&mut *seen, true)
        };
        if duplicate {
            self.duplicate_observed.notify_one();
            self.allow_duplicate
                .acquire()
                .await
                .expect("conversation attachment test hook closed")
                .forget();
        }
    }

    pub(crate) async fn wait_for_duplicate(&self) {
        self.duplicate_observed.notified().await;
    }

    pub(crate) fn permit_duplicate(&self) {
        self.allow_duplicate.add_permits(1);
    }

    pub(crate) async fn wait_for_attachment(&self) {
        self.attachment_ready.notified().await;
    }

    fn attachment_ready(&self) {
        self.attachment_ready.notify_one();
    }

    async fn wait_for_timeout_permission(&self) {
        self.allow_timeout
            .acquire()
            .await
            .expect("conversation attachment test hook closed")
            .forget();
    }

    pub(crate) fn release_all(&self) {
        self.allow_duplicate.add_permits(1);
        self.allow_timeout.add_permits(1);
    }

    pub(crate) fn fail_safe_permits(&self) -> (usize, usize) {
        (
            self.allow_duplicate.available_permits(),
            self.allow_timeout.available_permits(),
        )
    }
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
    governed_rooms: Mutex<BTreeMap<String, GovernedRoom>>,
    weather: Mutex<Option<ObservedWeather>>,
    weather_stale_after_millis: Mutex<u64>,
    observatory_bearer_digest: Mutex<Option<blake3::Hash>>,
    acip_write_bearer_digest: Mutex<Option<blake3::Hash>>,
    observatory_origin_policy: ObservatoryOriginPolicy,
    agent_population: AgentPopulationFeed,
    control_addr: Mutex<SocketAddr>,
    public_base_url: Mutex<String>,
    canonical_ingress: Option<CanonicalIngress>,
    layer8_authority: Option<Arc<Layer8ConversationAuthority>>,
    layer8_signed_exchange: Option<Arc<Layer8SignedExchange>>,
    agent_roster_token_key: Mutex<[u8; 32]>,
    api_policy: Mutex<Option<ControlApiPolicy>>,
    #[cfg(test)]
    conversation_attachment_test_hook: Mutex<Option<Arc<ConversationAttachmentTestHook>>>,
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
        mut agent_population: AgentPopulationFeed,
    ) -> Self {
        assert!(max_records > 0, "idempotency capacity must be non-zero");
        let instance_id = instance_id.into();
        assert!(
            is_safe_identifier(&instance_id),
            "runtime instance id must be bounded"
        );
        let observatory_origin_policy = ObservatoryOriginPolicy::new(observatory_allowed_origins)
            .expect("observatory origins must be approved exact origins");
        agent_population
            .sample
            .sort_by(|left, right| left.id.cmp(&right.id));
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
                sequences_by_principal: BTreeMap::new(),
            }),
            conversation_sessions: Mutex::new(ConversationSessions::default()),
            governed_rooms: Mutex::new(BTreeMap::new()),
            weather: Mutex::new(None),
            weather_stale_after_millis: Mutex::new(30_000),
            observatory_bearer_digest: Mutex::new(None),
            acip_write_bearer_digest: Mutex::new(None),
            observatory_origin_policy,
            agent_population,
            control_addr: Mutex::new(SocketAddr::from(([127, 0, 0, 1], 0))),
            public_base_url: Mutex::new("https://localhost".to_owned()),
            canonical_ingress: None,
            layer8_authority: None,
            layer8_signed_exchange: None,
            agent_roster_token_key: Mutex::new(blake3::derive_key(
                "adl.runtime_v3.agent_roster.page_token.ephemeral.v1",
                uuid::Uuid::new_v4().as_bytes(),
            )),
            api_policy: Mutex::new(None),
            #[cfg(test)]
            conversation_attachment_test_hook: Mutex::new(None),
        }
    }

    pub fn observatory_origin_policy(&self) -> ObservatoryOriginPolicy {
        self.observatory_origin_policy.clone()
    }

    pub fn replace_observatory_allowed_origins(
        &self,
        origins: impl IntoIterator<Item = String>,
    ) -> Result<(), String> {
        self.observatory_origin_policy.replace(origins)
    }

    pub fn replace_observatory_allowed_origins_from_runtime_init(
        &self,
        init: &crate::RuntimeInitConfig,
    ) -> Result<(), String> {
        self.replace_observatory_allowed_origins(init.observatory_allowed_origins())
    }

    #[cfg(test)]
    pub(crate) fn install_conversation_attachment_test_hook(
        &self,
        hook: Arc<ConversationAttachmentTestHook>,
    ) {
        *self
            .conversation_attachment_test_hook
            .lock()
            .expect("conversation attachment test hook mutex poisoned") = Some(hook);
    }

    #[cfg(test)]
    fn conversation_attachment_test_hook(
        &self,
        conversation_id: &str,
        turn_id: &str,
    ) -> Option<Arc<ConversationAttachmentTestHook>> {
        self.conversation_attachment_test_hook
            .lock()
            .expect("conversation attachment test hook mutex poisoned")
            .as_ref()
            .filter(|hook| hook.matches(conversation_id, turn_id))
            .cloned()
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

    pub fn with_layer8_authority(mut self, authority: Layer8ConversationAuthority) -> Self {
        self.layer8_authority = Some(Arc::new(authority));
        self
    }

    pub fn with_layer8_signed_exchange(mut self, exchange: Layer8SignedExchange) -> Self {
        self.layer8_signed_exchange = Some(Arc::new(exchange));
        self
    }

    fn conversation_recipient_eligibility(
        &self,
        recipient_id: &str,
    ) -> Result<Option<bool>, ControlError> {
        match self.agent_roster_detail(recipient_id) {
            Ok(agent) => Ok(Some(agent.communication_eligible)),
            Err(ControlError::InvalidBounds) => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn governed_room_refusal(
        intent: &GovernedRoomTurnIntent,
        error: &'static str,
    ) -> GovernedRoomRoute {
        GovernedRoomRoute {
            schema: GOVERNED_ROOM_ROUTE_SCHEMA,
            status: "refused",
            room_id: intent.room_id.clone(),
            turn_id: intent.turn_id.clone(),
            turn_sequence: intent.turn_sequence,
            sender_id: intent.sender_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            room_epoch: 0,
            addressed_recipients: intent.addressed_recipients.clone(),
            mentions: Vec::new(),
            deliveries: Vec::new(),
            error: Some(error),
        }
    }

    fn governed_room_participants(
        &self,
        intent: &GovernedRoomTurnIntent,
    ) -> Vec<GovernedRoomParticipant> {
        intent
            .addressed_recipients
            .iter()
            .filter_map(|recipient_id| {
                self.agent_roster_detail(recipient_id)
                    .ok()
                    .map(|agent| GovernedRoomParticipant {
                        participant_id: agent.id,
                        polis_id: "local_runtime".to_owned(),
                        display_name: agent.label,
                        policy_eligible: agent.communication_eligible,
                        state: if agent.communication_eligible {
                            GovernedRoomParticipantState::Joined
                        } else {
                            GovernedRoomParticipantState::Left
                        },
                    })
            })
            .collect()
    }

    fn accept_governed_room_intent(
        &self,
        envelope: &ObservatoryGovernedRoomIntent,
    ) -> GovernedRoomRoute {
        if envelope.schema != OBSERVATORY_WS_GOVERNED_ROOM_INTENT_SCHEMA
            || envelope.runtime_incarnation_id != self.runtime_incarnation_id
        {
            return Self::governed_room_refusal(&envelope.intent, "invalid_governed_room_intent");
        }
        let participants = self.governed_room_participants(&envelope.intent);
        let mut rooms = self
            .governed_rooms
            .lock()
            .expect("governed room state mutex poisoned");
        if let Some(room) = rooms.get_mut(&envelope.intent.room_id) {
            return match room.plan_turn(&envelope.intent) {
                Ok(route) => {
                    let delivery_states = route
                        .addressed_recipients
                        .iter()
                        .map(|recipient_id| {
                            (recipient_id.clone(), GovernedRoomDeliveryState::Accepted)
                        })
                        .collect();
                    route.with_delivery_states(delivery_states)
                }
                Err(error) => Self::governed_room_refusal(&envelope.intent, error.code()),
            };
        }
        if rooms.len() >= self.max_records {
            return Self::governed_room_refusal(
                &envelope.intent,
                "governed_room_capacity_exhausted",
            );
        }
        let mut room = GovernedRoom {
            room_id: envelope.intent.room_id.clone(),
            polis_id: "local_runtime".to_owned(),
            epoch: 1,
            next_turn_sequence: 1,
            seen_turn_ids: BTreeSet::new(),
            closed: false,
            participants,
        };
        match room.plan_turn(&envelope.intent) {
            Ok(route) => {
                let delivery_states = route
                    .addressed_recipients
                    .iter()
                    .map(|recipient_id| (recipient_id.clone(), GovernedRoomDeliveryState::Accepted))
                    .collect();
                let route = route.with_delivery_states(delivery_states);
                rooms.insert(envelope.intent.room_id.clone(), room);
                route
            }
            Err(error) => Self::governed_room_refusal(&envelope.intent, error.code()),
        }
    }

    fn accept_recipient_acknowledgement(
        &self,
        request: RuntimeRecipientAcknowledgementRequest,
    ) -> RuntimeRecipientAcknowledgementResponse {
        let projection = RuntimeRecipientAcknowledgementProjection::from_messages(
            &request.signed_request,
            &request.acknowledgement,
        );
        let refused =
            |error| RuntimeRecipientAcknowledgementResponse::refused(projection.clone(), error);
        if request.schema != RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_REQUEST_SCHEMA {
            return refused("invalid_request");
        }
        let Some(exchange) = self.layer8_signed_exchange.as_ref() else {
            return RuntimeRecipientAcknowledgementResponse::failed(
                projection,
                "recipient_acknowledgement_unavailable",
            );
        };
        let now_epoch_secs = now_unix_millis() / 1_000;
        match exchange.verify_request_and_acknowledgement(
            &request.signed_request,
            &request.acknowledgement,
            now_epoch_secs,
        ) {
            Ok(()) => match recipient_acknowledgement_delivery(
                &request.signed_request,
                &request.acknowledgement,
            ) {
                Ok(RecipientAcknowledgementDelivery::Accepted) => {
                    RuntimeRecipientAcknowledgementResponse::delivered(
                        projection,
                        request.signed_request.credential_generation,
                        request.acknowledgement.credential_generation,
                    )
                }
                Ok(RecipientAcknowledgementDelivery::Refused) => {
                    RuntimeRecipientAcknowledgementResponse::refused(
                        projection,
                        "recipient_refused_delivery",
                    )
                }
                Err(reason) => refused(layer8_refusal_code(reason)),
            },
            Err(reason) => refused(layer8_refusal_code(reason)),
        }
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
        let recipient = match self.conversation_recipient_eligibility(&intent.recipient_id) {
            Ok(recipient) => recipient,
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "agent_roster_unavailable",
                    None,
                ))
            }
        };
        match recipient {
            None => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "unknown_recipient",
                    None,
                ))
            }
            Some(false) => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "recipient_unavailable",
                    None,
                ))
            }
            Some(true) => {}
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
        let existing_turn_present = sessions
            .sessions
            .get(&intent.conversation_id)
            .and_then(|session| session.turns.get(&intent.turn_id))
            .is_some();
        if !existing_turn_present {
            let action = if sessions.sessions.contains_key(&intent.conversation_id) {
                Layer8Action::Continue
            } else {
                Layer8Action::Contact
            };
            if let Some(authority) = self.layer8_authority.as_ref() {
                let Some(exchange) = self.layer8_signed_exchange.as_ref() else {
                    return ConversationAcceptance::Response(outcome(
                        "failed",
                        "conversation_signing_unavailable",
                        None,
                    ));
                };
                let now_epoch_secs = now_unix_millis() / 1_000;
                let replay_id = format!(
                    "{}:{}:{}",
                    self.instance_id, intent.conversation_id, intent.turn_id
                );
                let payload_json = match serde_jcs::to_string(&serde_json::json!({
                    "action": action.clone(),
                    "message": intent.message,
                    "recipient_id": intent.recipient_id,
                })) {
                    Ok(payload) => payload,
                    Err(_) => {
                        return ConversationAcceptance::Response(outcome(
                            "refused",
                            "invalid_conversation_intent",
                            None,
                        ))
                    }
                };
                let signed_request = match exchange.signed_request(
                    &intent.recipient_id,
                    &intent.conversation_id,
                    &intent.correlation_id,
                    &replay_id,
                    payload_json,
                    now_epoch_secs,
                ) {
                    Ok(request) => request,
                    Err(_) => {
                        return ConversationAcceptance::Response(outcome(
                            "failed",
                            "conversation_signing_unavailable",
                            None,
                        ))
                    }
                };
                if exchange
                    .verify_request(&signed_request, now_epoch_secs)
                    .is_err()
                {
                    return ConversationAcceptance::Response(outcome(
                        "refused",
                        "conversation_request_signature_invalid",
                        None,
                    ));
                }
                let decision = authority.authorize(
                    &exchange.sender_verifying_identity(),
                    action,
                    intent.conversation_id.clone(),
                    intent.recipient_id.clone(),
                    replay_id,
                    intent.correlation_id.clone(),
                    now_epoch_secs,
                );
                if !matches!(decision, AuthorityDecision::Authorized(_)) {
                    return ConversationAcceptance::Response(outcome(
                        "refused",
                        "conversation_authority_refused",
                        None,
                    ));
                }
            }
        }
        if !sessions.sessions.contains_key(&intent.conversation_id) {
            if !sessions.retain_capacity_for_new_session(self.max_records) {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "conversation_capacity_exhausted",
                    None,
                ));
            }
            let Some(sequence) = sessions.next_sequence.checked_add(1) else {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "conversation_sequence_exhausted",
                    None,
                ));
            };
            sessions.next_sequence = sequence;
            sessions.sessions.insert(
                intent.conversation_id.clone(),
                ConversationSession {
                    sequence,
                    recipient_id: intent.recipient_id.clone(),
                    next_sequence: 0,
                    dispatch_gate: Arc::new(ConversationDispatchGate::new()),
                    turns: BTreeMap::new(),
                },
            );
        }
        let session = sessions
            .sessions
            .get_mut(&intent.conversation_id)
            .expect("conversation session inserted before lookup");
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
        if !session.retain_capacity_for_new_turn(self.max_records) {
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
        } else if !matches!(
            self.conversation_recipient_eligibility(&dispatch.intent.recipient_id),
            Ok(Some(true))
        ) {
            outcome("refused", "recipient_unavailable")
        } else {
            match (payload, self.canonical_ingress.as_ref()) {
                (Err(_), _) => outcome("refused", "invalid_conversation_intent"),
                (_, None) => outcome("failed", "conversation_ingress_unavailable"),
                (Ok(payload), Some(ingress)) => {
                    let submit = ingress.submit_with_cancellation(
                        DomainWork {
                            schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
                            work_id: dispatch.work_id.clone(),
                            kind: "agent_runtime".to_owned(),
                            payload,
                        },
                        dispatch.intent.correlation_id.clone(),
                        dispatch.cancellation.clone(),
                    );
                    #[cfg(test)]
                    let submitted = if let Some(hook) = self.conversation_attachment_test_hook(
                        &dispatch.intent.conversation_id,
                        &dispatch.intent.turn_id,
                    ) {
                        tokio::select! {
                            result = submit => Some(result),
                            _ = async {
                                tokio::time::sleep_until(deadline).await;
                                hook.wait_for_timeout_permission().await;
                            } => None,
                        }
                    } else {
                        tokio::time::timeout_at(deadline, submit).await.ok()
                    };
                    #[cfg(not(test))]
                    let submitted = tokio::time::timeout_at(deadline, submit).await.ok();
                    if submitted.is_none() {
                        dispatch.cancellation.cancel();
                    }
                    match submitted {
                        None => outcome("timed_out", "conversation_timed_out"),
                        Some(Err(_)) if dispatch.cancellation.is_cancelled() => {
                            outcome("cancelled", "conversation_cancelled")
                        }
                        Some(Ok(result)) => {
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
                        Some(Err(IngressError::Saturated | IngressError::Closed)) => {
                            outcome("failed", "conversation_temporarily_unavailable")
                        }
                        Some(Err(IngressError::UnsupportedKind)) => {
                            outcome("refused", "recipient_unavailable")
                        }
                        Some(Err(IngressError::Conflict)) => {
                            outcome("refused", "conversation_conflict")
                        }
                        Some(Err(_)) => outcome("failed", "conversation_failed"),
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
        let next = blake3::hash(token.as_bytes());
        let mut active = self
            .acip_write_bearer_digest
            .lock()
            .expect("ACIP write credential mutex poisoned");
        if active.is_some_and(|current| current != next) {
            self.acip_replay
                .lock()
                .expect("ACIP replay mutex poisoned")
                .sequences_by_principal
                .clear();
        }
        *active = Some(next);
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
        principal_digest: &blake3::Hash,
        runtime_id: &str,
        source: &str,
        sequence: u64,
    ) -> Option<AcipSequenceReservation> {
        if sequence == 0 || sequence == u64::MAX {
            return None;
        }
        let principal = *principal_digest.as_bytes();
        let domain = AcipReplayDomain {
            runtime_id: runtime_id.to_owned(),
            source: source.to_owned(),
        };
        let mut state = self.acip_replay.lock().expect("ACIP replay mutex poisoned");
        reserve_replay_sequence(&mut state, self.max_records, principal, domain, sequence)
    }

    fn commit_acip_sequence(&self, reservation: &AcipSequenceReservation) {
        let mut state = self.acip_replay.lock().expect("ACIP replay mutex poisoned");
        commit_replay_sequence(&mut state, reservation);
    }

    fn rollback_acip_sequence(&self, reservation: AcipSequenceReservation) {
        let mut state = self.acip_replay.lock().expect("ACIP replay mutex poisoned");
        rollback_replay_sequence(&mut state, reservation);
    }

    async fn dispatch_acip_payload(
        &self,
        authenticated_principal: &blake3::Hash,
        payload: &[u8],
    ) -> serde_json::Value {
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
        let Some(reservation) = self.reserve_acip_sequence(
            authenticated_principal,
            &envelope.runtime_id,
            &envelope.source,
            envelope.monotonic_sequence,
        ) else {
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
            Ok(result) => {
                self.commit_acip_sequence(&reservation);
                serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "completed",
                    "message_id": envelope.message_id,
                    "accepted_sequence": result.accepted_sequence,
                    "result_hash": result.result_hash,
                    "sequence_reserved": true
                })
            }
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
        let agents = self.agent_population.with_runtime_snapshot_query(
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
        event_cursor: Option<String>,
    ) -> Result<AgentPopulationFeed, ControlError> {
        let snapshot = self.recorder.snapshot();
        let now = now_unix_millis();
        self.agent_population
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
                event_cursor.as_deref(),
            )
            .map_err(|_| ControlError::InvalidBounds)
    }

    pub fn agent_roster_detail(&self, agent_id: &str) -> Result<AgentRosterEntry, ControlError> {
        let snapshot = self.recorder.snapshot();
        let now = now_unix_millis();
        self.agent_population
            .agent_detail(
                &snapshot,
                now,
                *self
                    .agent_roster_token_key
                    .lock()
                    .expect("agent roster token key mutex poisoned"),
                agent_id,
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
            RECIPIENT_ACKNOWLEDGEMENT_PATH,
            post(recipient_acknowledgement_handler::<C>)
                .options(control_preflight_handler::<C>)
                .layer(DefaultBodyLimit::max(api_policy.control_max_body_bytes)),
        )
        .route(
            "/v1/observatory",
            get(observatory_feed_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(
            "/v1/agents",
            get(agent_roster_handler::<C>).options(observatory_preflight_handler::<C>),
        )
        .route(
            "/v1/agents/{agent_id}",
            get(agent_detail_handler::<C>).options(observatory_preflight_handler::<C>),
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
                let principal = blake3::hash(bearer_token.as_bytes());
                let response = service.dispatch_acip_payload(&principal, &payload).await;
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
    event_cursor: Option<String>,
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
    match service.agent_roster_page(
        query.page_size,
        query.page_token,
        query.filter,
        query.event_cursor,
    ) {
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

async fn agent_detail_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    axum::extract::Path(agent_id): axum::extract::Path<String>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    match service.agent_roster_detail(&agent_id) {
        Ok(agent) => observatory_json(StatusCode::OK, agent, allowed_origin),
        Err(_) => observatory_json(
            StatusCode::NOT_FOUND,
            serde_json::json!({
                "schema": "adl.runtime_v3.agent_roster_error.v1",
                "code": "agent_not_visible"
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
struct ObservatoryGovernedRoomIntent {
    schema: String,
    runtime_incarnation_id: String,
    intent: GovernedRoomTurnIntent,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryConversationCancel {
    schema: String,
    conversation_id: String,
    turn_id: String,
    correlation_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeRecipientAcknowledgementRequest {
    schema: String,
    signed_request: SignedIdentityMessage,
    acknowledgement: SignedIdentityMessage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RecipientAcknowledgementDelivery {
    Accepted,
    Refused,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RecipientAcknowledgementPayload {
    delivery: RecipientAcknowledgementDelivery,
    recipient_id: String,
}

fn recipient_acknowledgement_delivery(
    request: &SignedIdentityMessage,
    acknowledgement: &SignedIdentityMessage,
) -> Result<RecipientAcknowledgementDelivery, RefusalReason> {
    let payload: RecipientAcknowledgementPayload =
        serde_json::from_str(&acknowledgement.payload_json)
            .map_err(|_| RefusalReason::InvalidRequest)?;
    if payload.recipient_id != request.recipient_id {
        return Err(RefusalReason::InvalidRequest);
    }
    Ok(payload.delivery)
}

#[derive(Clone, Debug)]
struct RuntimeRecipientAcknowledgementProjection {
    conversation_id: Option<String>,
    request_message_id: Option<String>,
    acknowledgement_message_id: Option<String>,
    sender_id: Option<String>,
    recipient_id: Option<String>,
    correlation_hash: Option<String>,
}

impl RuntimeRecipientAcknowledgementProjection {
    fn from_messages(
        request: &SignedIdentityMessage,
        acknowledgement: &SignedIdentityMessage,
    ) -> Self {
        let correlation_hash = if is_correlation_id(&request.correlation_id)
            && request.correlation_id == acknowledgement.correlation_id
        {
            Some(
                blake3::hash(request.correlation_id.as_bytes())
                    .to_hex()
                    .to_string(),
            )
        } else {
            None
        };
        Self {
            conversation_id: is_safe_identifier(&request.conversation_id)
                .then(|| request.conversation_id.clone()),
            request_message_id: is_safe_identifier(&request.message_id)
                .then(|| request.message_id.clone()),
            acknowledgement_message_id: is_safe_identifier(&acknowledgement.message_id)
                .then(|| acknowledgement.message_id.clone()),
            sender_id: is_safe_identifier(&request.sender_id).then(|| request.sender_id.clone()),
            recipient_id: is_safe_identifier(&request.recipient_id)
                .then(|| request.recipient_id.clone()),
            correlation_hash,
        }
    }
}

#[derive(Clone, Serialize)]
struct RuntimeRecipientAcknowledgementResponse {
    schema: &'static str,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acknowledgement_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_credential_generation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipient_credential_generation: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

impl RuntimeRecipientAcknowledgementResponse {
    fn delivered(
        projection: RuntimeRecipientAcknowledgementProjection,
        sender_credential_generation: u64,
        recipient_credential_generation: u64,
    ) -> Self {
        Self {
            schema: RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_RESPONSE_SCHEMA,
            status: "delivered",
            conversation_id: projection.conversation_id,
            request_message_id: projection.request_message_id,
            acknowledgement_message_id: projection.acknowledgement_message_id,
            sender_id: projection.sender_id,
            recipient_id: projection.recipient_id,
            correlation_hash: projection.correlation_hash,
            sender_credential_generation: Some(sender_credential_generation),
            recipient_credential_generation: Some(recipient_credential_generation),
            error: None,
        }
    }

    fn refused(projection: RuntimeRecipientAcknowledgementProjection, error: &'static str) -> Self {
        Self {
            schema: RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_RESPONSE_SCHEMA,
            status: "refused",
            conversation_id: projection.conversation_id,
            request_message_id: projection.request_message_id,
            acknowledgement_message_id: projection.acknowledgement_message_id,
            sender_id: projection.sender_id,
            recipient_id: projection.recipient_id,
            correlation_hash: projection.correlation_hash,
            sender_credential_generation: None,
            recipient_credential_generation: None,
            error: Some(error),
        }
    }

    fn failed(projection: RuntimeRecipientAcknowledgementProjection, error: &'static str) -> Self {
        Self {
            schema: RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_RESPONSE_SCHEMA,
            status: "failed",
            conversation_id: projection.conversation_id,
            request_message_id: projection.request_message_id,
            acknowledgement_message_id: projection.acknowledgement_message_id,
            sender_id: projection.sender_id,
            recipient_id: projection.recipient_id,
            correlation_hash: projection.correlation_hash,
            sender_credential_generation: None,
            recipient_credential_generation: None,
            error: Some(error),
        }
    }
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
    let mut authentication_generation = 0_u64;
    let mut conversation_attachments = HashSet::<(u64, String, String)>::new();
    let (conversation_results_tx, mut conversation_results_rx) =
        tokio::sync::mpsc::channel::<(u64, [u8; 32], ObservatoryConversationResult)>(
            OBSERVATORY_CONVERSATION_RESULT_QUEUE_CAPACITY,
        );
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
            Some((authorized_generation, authorized_token_digest, result)) = conversation_results_rx.recv() => {
                conversation_attachments.remove(&(
                    authorized_generation,
                    result.conversation_id.clone(),
                    result.turn_id.clone(),
                ));
                if authorized_generation != authentication_generation {
                    continue;
                }
                if bearer_token.as_deref().map(|token| *blake3::hash(token.as_bytes()).as_bytes())
                    != Some(authorized_token_digest)
                {
                    continue;
                }
                if bearer_token
                    .as_deref()
                    .is_none_or(|token| !service.observatory_token_authorized(token))
                {
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
                    continue;
                }
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
                    let Some(next_generation) = authentication_generation.checked_add(1) else {
                        break;
                    };
                    authentication_generation = next_generation;
                    conversation_attachments.clear();
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
                        let Some(next_generation) = authentication_generation.checked_add(1) else {
                            break;
                        };
                        authentication_generation = next_generation;
                        conversation_attachments.clear();
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
                        let Some(next_generation) = authentication_generation.checked_add(1) else {
                            break;
                        };
                        authentication_generation = next_generation;
                        conversation_attachments.clear();
                    }
                    if let Ok(intent) = serde_json::from_str::<ObservatoryConversationIntent>(&payload) {
                        #[cfg(test)]
                        if let Some(hook) = service.conversation_attachment_test_hook(
                            &intent.conversation_id,
                            &intent.turn_id,
                        ) {
                            hook.observe_intent().await;
                        }
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
                        let attachment_inserted = if dispatch.is_some() || attach_to_in_flight {
                            conversation_attachments.insert((
                                authentication_generation,
                                conversation_id.clone(),
                                turn_id.clone(),
                            ))
                        } else {
                            false
                        };
                        #[cfg(test)]
                        if attach_to_in_flight && attachment_inserted {
                            if let Some(hook) = service.conversation_attachment_test_hook(
                                &conversation_id,
                                &turn_id,
                            ) {
                                hook.attachment_ready();
                            }
                        }
                        let Ok(payload) = serde_json::to_string(&response) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                        if let Some(dispatch) = dispatch {
                            let service = service.clone();
                            let results = conversation_results_tx.clone();
                            let authorized_generation = authentication_generation;
                            let authorized_token_digest = bearer_token
                                .as_deref()
                                .map(|token| *blake3::hash(token.as_bytes()).as_bytes())
                                .expect("authenticated conversation dispatch has a bearer token");
                            tokio::spawn(async move {
                                let result = service.complete_conversation_dispatch(dispatch).await;
                                let _ = results
                                    .send((authorized_generation, authorized_token_digest, result))
                                    .await;
                            });
                        } else if attach_to_in_flight && attachment_inserted {
                            let service = service.clone();
                            let results = conversation_results_tx.clone();
                            let authorized_generation = authentication_generation;
                            let authorized_token_digest = bearer_token
                                .as_deref()
                                .map(|token| *blake3::hash(token.as_bytes()).as_bytes())
                                .expect("authenticated conversation replay has a bearer token");
                            tokio::spawn(async move {
                                if let Some(result) = service
                                    .wait_for_conversation_terminal(&conversation_id, &turn_id)
                                    .await
                                {
                                    let _ = results
                                        .send((authorized_generation, authorized_token_digest, result))
                                        .await;
                                }
                            });
                        }
                        continue;
                    }
                    if let Ok(room_intent) = serde_json::from_str::<ObservatoryGovernedRoomIntent>(&payload) {
                        let result = if bearer_token.is_none() {
                            ControlService::<C>::governed_room_refusal(
                                &room_intent.intent,
                                "write_authentication_required",
                            )
                        } else {
                            service.accept_governed_room_intent(&room_intent)
                        };
                        let Ok(payload) = serde_json::to_string(&result) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
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

async fn recipient_acknowledgement_handler<C: LifecycleControl + 'static>(
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
    let request = match serde_json::from_slice::<RuntimeRecipientAcknowledgementRequest>(&body) {
        Ok(request) => request,
        Err(_) => {
            return observatory_json(
                StatusCode::BAD_REQUEST,
                RuntimeRecipientAcknowledgementResponse::refused(
                    RuntimeRecipientAcknowledgementProjection {
                        conversation_id: None,
                        request_message_id: None,
                        acknowledgement_message_id: None,
                        sender_id: None,
                        recipient_id: None,
                        correlation_hash: None,
                    },
                    "invalid_request",
                ),
                allowed_origin,
            )
        }
    };
    let response = service.accept_recipient_acknowledgement(request);
    let status = match response.status {
        "delivered" => StatusCode::OK,
        "failed" => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_REQUEST,
    };
    observatory_json(status, response, allowed_origin)
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

fn layer8_refusal_code(reason: RefusalReason) -> &'static str {
    match reason {
        RefusalReason::InvalidRequest => "invalid_acknowledgement",
        RefusalReason::IdentityUnavailable => "identity_unavailable",
        RefusalReason::IdentityExpired => "identity_expired",
        RefusalReason::IdentityRevoked => "identity_revoked",
        RefusalReason::StaleCredential => "stale_credential",
        RefusalReason::CapabilityDenied => "capability_denied",
        RefusalReason::CapabilityExpired => "capability_expired",
        RefusalReason::CapabilityRevoked => "capability_revoked",
        RefusalReason::StaleCapability => "stale_capability",
        RefusalReason::PolicyUnavailable => "policy_unavailable",
        RefusalReason::ScopeDenied => "scope_denied",
        RefusalReason::ReplayRefused => "replay_refused",
        RefusalReason::AuditUnavailable => "audit_unavailable",
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
        .observatory_origin_policy
        .contains(origin)
        .then(|| HeaderValue::from_str(origin).ok())
        .flatten()
}

#[derive(Clone, Debug)]
pub struct ObservatoryOriginPolicy {
    origins: Arc<RwLock<Arc<BTreeSet<String>>>>,
}

impl ObservatoryOriginPolicy {
    pub fn new(origins: impl IntoIterator<Item = String>) -> Result<Self, String> {
        Ok(Self {
            origins: Arc::new(RwLock::new(Arc::new(validate_observatory_origins(
                origins, true,
            )?))),
        })
    }

    pub fn replace(&self, origins: impl IntoIterator<Item = String>) -> Result<(), String> {
        let origins = Arc::new(validate_observatory_origins(origins, true)?);
        let mut active = self
            .origins
            .write()
            .map_err(|_| "observatory_origin_policy_unavailable".to_owned())?;
        *active = origins;
        Ok(())
    }

    pub fn contains(&self, origin: &str) -> bool {
        self.origins
            .read()
            .map(|origins| origins.contains(origin))
            .unwrap_or(false)
    }
}

fn validate_observatory_origins(
    origins: impl IntoIterator<Item = String>,
    allow_empty: bool,
) -> Result<BTreeSet<String>, String> {
    let mut unique = BTreeSet::new();
    for origin in origins {
        if !valid_observatory_origin(&origin) {
            return Err("observatory_allowed_origins_must_be_approved_exact_origins".to_owned());
        }
        if !unique.insert(origin) {
            return Err("observatory_allowed_origins_must_be_unique".to_owned());
        }
    }
    if !allow_empty && unique.is_empty() {
        return Err("observatory_allowed_origins_required".to_owned());
    }
    Ok(unique)
}

fn valid_observatory_origin(origin: &str) -> bool {
    if origin == "*" || origin.len() > 512 || origin.bytes().any(|byte| byte.is_ascii_control()) {
        return false;
    }
    if origin == "http://localhost:8000" {
        return true;
    }
    let Ok(uri) = origin.parse::<axum::http::Uri>() else {
        return false;
    };
    uri.scheme_str() == Some("https")
        && uri.authority().is_some()
        && uri.path() == "/"
        && uri.query().is_none()
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

#[cfg(test)]
mod layer8_conversation_ingress_tests {
    use super::*;
    use crate::layer8_authority::{
        sign_recipient_acknowledgement, AuthorityScope, CommunicationKeyDescriptor,
        CommunicationVerifyingDescriptor, ConversationAuthorityProfile, ConversationSigningProfile,
        Layer8AuthorityStore, Layer8Capability, Layer8Policy, RuntimeIdentityEvidence,
    };
    use crate::{AgentRosterPolicy, ComponentId, RunningState};

    struct FakeLifecycle;

    #[async_trait]
    impl LifecycleControl for FakeLifecycle {
        async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
            Ok(KernelExit::Clean)
        }
    }

    struct Layer8Fixture {
        root: tempfile::TempDir,
        authority: Layer8ConversationAuthority,
        exchange: Layer8SignedExchange,
        recipient_descriptor: CommunicationKeyDescriptor,
    }

    fn layer8_fixture(contact_recipient: &str, continue_recipient: &str) -> Layer8Fixture {
        let temp_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".adl")
            .join("tmp");
        std::fs::create_dir_all(&temp_root).expect("create test temp root");
        let root = tempfile::tempdir_in(temp_root).expect("create layer8 fixture");
        let sender_key = SigningKey::from_bytes(&[41; 32]);
        let recipient_key = SigningKey::from_bytes(&[42; 32]);
        let sender_key_file = root.path().join("operator.key");
        let recipient_key_file = root.path().join("shepherd.key");
        std::fs::write(&sender_key_file, hex::encode(sender_key.to_bytes()))
            .expect("write sender key");
        std::fs::write(&recipient_key_file, hex::encode(recipient_key.to_bytes()))
            .expect("write recipient key");
        let evidence = RuntimeIdentityEvidence {
            principal_id: "operator".to_owned(),
            polis_id: "conversation-runtime".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            verifying_key_hex: hex::encode(sender_key.verifying_key().to_bytes()),
            credential_generation: 1,
            current_credential_generation: 1,
            expires_at_epoch_secs: u64::MAX,
            revoked: false,
            authenticated: true,
        };
        let scope = |action, recipient: &str| AuthorityScope {
            polis_id: "conversation-runtime".to_owned(),
            action,
            conversation_id: None,
            recipients: BTreeSet::from([recipient.to_owned()]),
            attachment_id: None,
        };
        let contact_scope = scope(Layer8Action::Contact, contact_recipient);
        let continue_scope = scope(Layer8Action::Continue, continue_recipient);
        let capabilities = [
            ("contact-capability", contact_scope.clone()),
            ("continue-capability", continue_scope.clone()),
        ]
        .into_iter()
        .map(|(capability_id, scope)| Layer8Capability {
            capability_id: capability_id.to_owned(),
            principal_id: evidence.principal_id.clone(),
            scope,
            epoch: 1,
            expires_at_epoch_secs: u64::MAX,
            revoked: false,
        })
        .collect();
        let agent_policies = [
            ("agent-contact-policy", contact_scope.clone()),
            ("agent-continue-policy", continue_scope.clone()),
        ]
        .into_iter()
        .map(|(policy_id, scope)| Layer8Policy {
            policy_id: policy_id.to_owned(),
            available: true,
            scope,
            epoch: 1,
        })
        .collect();
        let polis_policies = [
            ("polis-contact-policy", contact_scope),
            ("polis-continue-policy", continue_scope),
        ]
        .into_iter()
        .map(|(policy_id, scope)| Layer8Policy {
            policy_id: policy_id.to_owned(),
            available: true,
            scope,
            epoch: 1,
        })
        .collect();
        let authority = Layer8ConversationAuthority::new(
            Layer8AuthorityStore::open(root.path().join("audit.jsonl")).expect("open audit"),
            ConversationAuthorityProfile {
                evidence: evidence.clone(),
                capabilities,
                agent_policies,
                polis_policies,
            },
        )
        .expect("authority profile is valid");
        let recipient_descriptor = CommunicationKeyDescriptor {
            principal_id: "shepherd".to_owned(),
            polis_id: "conversation-runtime".to_owned(),
            signing_key_id: "shepherd-key".to_owned(),
            credential_generation: 1,
            private_key_file: recipient_key_file,
            not_before_epoch_secs: 0,
            expires_at_epoch_secs: u64::MAX,
        };
        let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
            sender: CommunicationKeyDescriptor {
                principal_id: "operator".to_owned(),
                polis_id: "conversation-runtime".to_owned(),
                signing_key_id: "operator-key".to_owned(),
                credential_generation: 1,
                private_key_file: sender_key_file,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            },
            recipients: vec![CommunicationVerifyingDescriptor {
                principal_id: "shepherd".to_owned(),
                polis_id: "conversation-runtime".to_owned(),
                signing_key_id: "shepherd-key".to_owned(),
                credential_generation: 1,
                verifying_key_hex: hex::encode(recipient_key.verifying_key().to_bytes()),
                revoked: false,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            }],
        })
        .expect("exchange profile is valid");
        Layer8Fixture {
            root,
            authority,
            exchange,
            recipient_descriptor,
        }
    }

    fn service_from_layer8_parts(
        authority: Layer8ConversationAuthority,
        exchange: Layer8SignedExchange,
    ) -> ControlService<FakeLifecycle> {
        let recorder = RuntimeRecorder::new(16);
        let now = now_unix_millis();
        recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
        assert!(recorder.record_agent_admission(
            "shepherd",
            now,
            now + 30_000,
            "1111111111111111111111111111111111111111",
        ));
        let ingress = CanonicalIngress::new(4, recorder.clone(), BTreeMap::new());
        ControlService::new_with_observatory_config_and_agents(
            "conversation-runtime",
            recorder,
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            8,
            ["https://observatory.example.test".to_owned()],
            AgentPopulationFeed::resident_shepherd(),
        )
        .with_canonical_ingress(ingress)
        .with_layer8_authority(authority)
        .with_layer8_signed_exchange(exchange)
    }

    fn service_with_layer8(
        contact_recipient: &str,
        continue_recipient: &str,
    ) -> (ControlService<FakeLifecycle>, tempfile::TempDir) {
        let fixture = layer8_fixture(contact_recipient, continue_recipient);
        let Layer8Fixture {
            root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = service_from_layer8_parts(authority, exchange);
        (service, root)
    }

    fn service_with_room_agents() -> ControlService<FakeLifecycle> {
        let recorder = RuntimeRecorder::new(16);
        let now = now_unix_millis();
        let mut population = AgentPopulationFeed::empty();
        for (id, label) in [("shepherd", "Shepherd"), ("scribe", "Scribe")] {
            recorder.set_component_state(ComponentId::new(id), RunningState::Running);
            assert!(recorder.record_agent_admission(
                id,
                now,
                now + 30_000,
                "1111111111111111111111111111111111111111",
            ));
            population.sample.push(AgentSample {
                id: id.to_owned(),
                label: label.to_owned(),
                role: "conversation agent".to_owned(),
                state: "unknown".to_owned(),
                detail: "Awaiting Runtime projection".to_owned(),
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
            });
        }
        population = population.with_public_policy(AgentRosterPolicy {
            policy_subject: "governed-room-test".to_owned(),
            visible_agent_ids: BTreeSet::from(["shepherd".to_owned(), "scribe".to_owned()]),
            reveal_capabilities: false,
            reveal_location: false,
        });
        ControlService::new_with_observatory_config_and_agents(
            "conversation-runtime",
            recorder,
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            8,
            ["https://observatory.example.test".to_owned()],
            population,
        )
    }

    fn room_envelope(
        service: &ControlService<FakeLifecycle>,
        turn_id: &str,
        sequence: u64,
        recipients: Vec<&str>,
    ) -> ObservatoryGovernedRoomIntent {
        ObservatoryGovernedRoomIntent {
            schema: OBSERVATORY_WS_GOVERNED_ROOM_INTENT_SCHEMA.to_owned(),
            runtime_incarnation_id: service.runtime_incarnation_id.clone(),
            intent: GovernedRoomTurnIntent {
                schema: crate::conversation_rooms::GOVERNED_ROOM_TURN_SCHEMA.to_owned(),
                room_id: "room-shepherd-scribe".to_owned(),
                turn_id: turn_id.to_owned(),
                turn_sequence: sequence,
                sender_id: "operator".to_owned(),
                correlation_id: format!("corr:{turn_id}"),
                addressed_recipients: recipients.into_iter().map(str::to_owned).collect(),
                message: "hello room".to_owned(),
            },
        }
    }

    #[test]
    fn governed_room_ws_intent_routes_explicit_runtime_recipients() {
        let service = service_with_room_agents();
        let route = service.accept_governed_room_intent(&room_envelope(
            &service,
            "room-turn-1",
            1,
            vec!["scribe", "shepherd"],
        ));
        assert_eq!(route.schema, GOVERNED_ROOM_ROUTE_SCHEMA);
        assert_eq!(route.status, "accepted");
        assert_eq!(route.addressed_recipients, vec!["scribe", "shepherd"]);
        assert_eq!(route.deliveries.len(), 2);
        assert!(route
            .deliveries
            .iter()
            .all(|delivery| delivery.state == GovernedRoomDeliveryState::Accepted));
        assert_eq!(
            route
                .mentions
                .iter()
                .map(|mention| (mention.recipient_id.as_str(), mention.display_name.as_str()))
                .collect::<Vec<_>>(),
            vec![("scribe", "Scribe"), ("shepherd", "Shepherd")]
        );
    }

    #[test]
    fn governed_room_ws_intent_rejects_implicit_broadcast_without_consuming_sequence() {
        let service = service_with_room_agents();
        let refused =
            service.accept_governed_room_intent(&room_envelope(&service, "room-turn-1", 1, vec![]));
        assert_eq!(refused.status, "refused");
        assert_eq!(refused.error, Some("implicit_broadcast_denied"));

        let accepted = service.accept_governed_room_intent(&room_envelope(
            &service,
            "room-turn-1",
            1,
            vec!["shepherd"],
        ));
        assert_eq!(accepted.status, "accepted");
        assert_eq!(accepted.turn_sequence, 1);
        assert_eq!(accepted.addressed_recipients, vec!["shepherd"]);
    }

    #[test]
    fn governed_room_ws_intent_rejects_non_initial_first_sequence() {
        let service = service_with_room_agents();
        let refused = service.accept_governed_room_intent(&room_envelope(
            &service,
            "room-turn-2",
            2,
            vec!["shepherd"],
        ));
        assert_eq!(refused.status, "refused");
        assert_eq!(refused.error, Some("reordered_room_turn"));

        let accepted = service.accept_governed_room_intent(&room_envelope(
            &service,
            "room-turn-1",
            1,
            vec!["shepherd"],
        ));
        assert_eq!(accepted.status, "accepted");
        assert_eq!(accepted.addressed_recipients, vec!["shepherd"]);
    }

    fn intent() -> ObservatoryConversationIntent {
        ObservatoryConversationIntent {
            schema: OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA.to_owned(),
            conversation_id: "conversation-layer8".to_owned(),
            turn_id: "turn-layer8".to_owned(),
            recipient_id: "shepherd".to_owned(),
            correlation_id: "12121212121212121212121212121212".to_owned(),
            message: "Hello".to_owned(),
        }
    }

    fn continue_intent(turn_id: &str) -> ObservatoryConversationIntent {
        ObservatoryConversationIntent {
            turn_id: turn_id.to_owned(),
            message: format!("Continue with {turn_id}"),
            ..intent()
        }
    }

    fn recipient_ack_request_with_payload(
        exchange: &Layer8SignedExchange,
        recipient_descriptor: &CommunicationKeyDescriptor,
        acknowledgement_payload: serde_json::Value,
    ) -> RuntimeRecipientAcknowledgementRequest {
        let now = now_unix_millis() / 1_000;
        let payload_json = serde_jcs::to_string(&serde_json::json!({
            "action": "Contact",
            "message": "Hello",
            "recipient_id": "shepherd",
        }))
        .expect("request payload is canonical");
        let signed_request = exchange
            .signed_request(
                "shepherd",
                "conversation-layer8",
                "12121212121212121212121212121212",
                "conversation-runtime:conversation-layer8:turn-layer8",
                payload_json,
                now,
            )
            .expect("signed request");
        let acknowledgement_payload =
            serde_jcs::to_string(&acknowledgement_payload).expect("ack payload is canonical");
        let acknowledgement = sign_recipient_acknowledgement(
            &signed_request,
            recipient_descriptor,
            acknowledgement_payload,
            now,
        )
        .expect("signed acknowledgement");
        RuntimeRecipientAcknowledgementRequest {
            schema: RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_REQUEST_SCHEMA.to_owned(),
            signed_request,
            acknowledgement,
        }
    }

    fn recipient_ack_request(
        exchange: &Layer8SignedExchange,
        recipient_descriptor: &CommunicationKeyDescriptor,
    ) -> RuntimeRecipientAcknowledgementRequest {
        recipient_ack_request_with_payload(
            exchange,
            recipient_descriptor,
            serde_json::json!({
                "delivery": "accepted",
                "recipient_id": "shepherd",
            }),
        )
    }

    #[test]
    fn layer8_ingress_refuses_before_conversation_side_effects() {
        let (service, _root) = service_with_layer8("agent-other", "agent-other");
        let response = match service.accept_conversation_intent(&intent()) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => {
                panic!("unauthorized conversation dispatched")
            }
        };
        assert_eq!(response.status, "refused");
        assert_eq!(response.error, Some("conversation_authority_refused"));
        assert!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .is_empty(),
            "authority refusal must happen before session or turn mutation"
        );
    }

    #[test]
    fn layer8_ingress_authorizes_before_dispatch() {
        let (service, _root) = service_with_layer8("shepherd", "shepherd");
        let accepted = match service.accept_conversation_intent(&intent()) {
            ConversationAcceptance::Dispatch { accepted, .. } => accepted,
            ConversationAcceptance::Response(response) => {
                panic!("authorized conversation was refused: {:?}", response.error)
            }
        };
        assert_eq!(accepted.status, "accepted");
        assert_eq!(accepted.error, None);
        assert_eq!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .len(),
            1,
            "authorized ingress may create the session after authority grants"
        );
    }

    #[test]
    fn layer8_ingress_continue_refuses_before_turn_side_effects() {
        let (service, _root) = service_with_layer8("shepherd", "agent-other");
        match service.accept_conversation_intent(&intent()) {
            ConversationAcceptance::Dispatch { .. } => {}
            ConversationAcceptance::Response(response) => {
                panic!("initial contact was refused: {:?}", response.error)
            }
        }

        let response = match service.accept_conversation_intent(&continue_intent("turn-continue")) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => {
                panic!("unauthorized continue dispatched")
            }
        };
        assert_eq!(response.status, "refused");
        assert_eq!(response.error, Some("conversation_authority_refused"));
        let sessions = service
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        let session = sessions
            .sessions
            .get("conversation-layer8")
            .expect("initial authorized contact created a session");
        assert_eq!(session.turns.len(), 1);
        assert!(
            !session.turns.contains_key("turn-continue"),
            "authority refusal must happen before adding a continuation turn"
        );
    }

    #[test]
    fn layer8_ingress_continue_authorizes_before_dispatch() {
        let (service, _root) = service_with_layer8("shepherd", "shepherd");
        match service.accept_conversation_intent(&intent()) {
            ConversationAcceptance::Dispatch { .. } => {}
            ConversationAcceptance::Response(response) => {
                panic!("initial contact was refused: {:?}", response.error)
            }
        }

        let accepted = match service.accept_conversation_intent(&continue_intent("turn-continue")) {
            ConversationAcceptance::Dispatch { accepted, .. } => accepted,
            ConversationAcceptance::Response(response) => {
                panic!("authorized continue was refused: {:?}", response.error)
            }
        };
        assert_eq!(accepted.status, "accepted");
        assert_eq!(accepted.error, None);
        let sessions = service
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        let session = sessions
            .sessions
            .get("conversation-layer8")
            .expect("initial authorized contact created a session");
        assert_eq!(
            session.turns.len(),
            2,
            "authorized continuation may add the turn after authority grants"
        );
        assert!(session.turns.contains_key("turn-continue"));
    }

    #[test]
    fn recipient_acknowledgement_api_delivers_verified_ack_with_redacted_correlation() {
        let fixture = layer8_fixture("shepherd", "shepherd");
        let request = recipient_ack_request(&fixture.exchange, &fixture.recipient_descriptor);
        let raw_correlation = request.signed_request.correlation_id.clone();
        let Layer8Fixture {
            root: _root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = service_from_layer8_parts(authority, exchange);
        let response = service.accept_recipient_acknowledgement(request);

        assert_eq!(response.status, "delivered");
        assert_eq!(response.error, None);
        assert_eq!(
            response.conversation_id.as_deref(),
            Some("conversation-layer8")
        );
        assert_eq!(response.sender_id.as_deref(), Some("operator"));
        assert_eq!(response.recipient_id.as_deref(), Some("shepherd"));
        assert_eq!(response.sender_credential_generation, Some(1));
        assert_eq!(response.recipient_credential_generation, Some(1));
        assert_ne!(
            response.correlation_hash.as_deref(),
            Some(raw_correlation.as_str())
        );
        assert_eq!(
            response.correlation_hash.as_deref(),
            Some(blake3::hash(raw_correlation.as_bytes()).to_hex().as_str())
        );
        assert!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .is_empty(),
            "acknowledgement verification must not create conversation history"
        );
    }

    #[test]
    fn recipient_acknowledgement_api_refuses_tampered_credential_generation_before_side_effects() {
        let fixture = layer8_fixture("shepherd", "shepherd");
        let mut request = recipient_ack_request(&fixture.exchange, &fixture.recipient_descriptor);
        request.acknowledgement.credential_generation = 2;
        let Layer8Fixture {
            root: _root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = service_from_layer8_parts(authority, exchange);
        let response = service.accept_recipient_acknowledgement(request);

        assert_eq!(response.status, "refused");
        assert_eq!(response.error, Some("identity_unavailable"));
        assert_eq!(response.sender_credential_generation, None);
        assert_eq!(response.recipient_credential_generation, None);
        assert!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .is_empty(),
            "invalid acknowledgement must be refused before session mutation"
        );
    }

    #[test]
    fn recipient_acknowledgement_api_refuses_recipient_signed_delivery_refusal() {
        let fixture = layer8_fixture("shepherd", "shepherd");
        let request = recipient_ack_request_with_payload(
            &fixture.exchange,
            &fixture.recipient_descriptor,
            serde_json::json!({
                "delivery": "refused",
                "recipient_id": "shepherd",
            }),
        );
        let Layer8Fixture {
            root: _root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = service_from_layer8_parts(authority, exchange);
        let response = service.accept_recipient_acknowledgement(request);

        assert_eq!(response.status, "refused");
        assert_eq!(response.error, Some("recipient_refused_delivery"));
        assert_eq!(response.sender_credential_generation, None);
        assert_eq!(response.recipient_credential_generation, None);
        assert!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .is_empty(),
            "recipient refusal must not create conversation history"
        );
    }

    #[test]
    fn recipient_acknowledgement_api_refuses_unrelated_signed_payload() {
        let fixture = layer8_fixture("shepherd", "shepherd");
        let request = recipient_ack_request_with_payload(
            &fixture.exchange,
            &fixture.recipient_descriptor,
            serde_json::json!({
                "delivery": "accepted",
                "recipient_id": "agent-other",
            }),
        );
        let Layer8Fixture {
            root: _root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = service_from_layer8_parts(authority, exchange);
        let response = service.accept_recipient_acknowledgement(request);

        assert_eq!(response.status, "refused");
        assert_eq!(response.error, Some("invalid_acknowledgement"));
        assert!(
            service
                .conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .is_empty(),
            "malformed acknowledgement payload must not create conversation history"
        );
    }

    #[tokio::test]
    async fn recipient_acknowledgement_route_serves_delivery_without_raw_correlation() {
        let fixture = layer8_fixture("shepherd", "shepherd");
        let request = recipient_ack_request(&fixture.exchange, &fixture.recipient_descriptor);
        let raw_correlation = request.signed_request.correlation_id.clone();
        let body = serde_json::to_vec(&request).expect("serialize request");
        let Layer8Fixture {
            root: _root,
            authority,
            exchange,
            recipient_descriptor: _,
        } = fixture;
        let service = Arc::new(service_from_layer8_parts(authority, exchange));

        let response =
            recipient_acknowledgement_handler(State(service), HeaderMap::new(), Bytes::from(body))
                .await;
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("read response body");
        let value: serde_json::Value = serde_json::from_slice(&body).expect("parse response");

        assert_eq!(
            value["schema"],
            RUNTIME_RECIPIENT_ACKNOWLEDGEMENT_RESPONSE_SCHEMA
        );
        assert_eq!(value["status"], "delivered");
        assert!(value["correlation_hash"].as_str().is_some());
        assert!(
            !std::str::from_utf8(&body)
                .expect("response body is UTF-8")
                .contains(&raw_correlation),
            "served acknowledgement route must not echo the raw correlation id"
        );
    }
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

    fn session(sequence: u64, terminal: bool) -> ConversationSession {
        let (completion, _) = tokio::sync::watch::channel(None);
        let mut turns = BTreeMap::new();
        turns.insert(
            "turn".to_owned(),
            ConversationTurn {
                fingerprint: "fingerprint".to_owned(),
                correlation_id: "00000000000000000000000000000000".to_owned(),
                sequence: 1,
                cancellation: CancellationToken::new(),
                completion,
                terminal: terminal.then(|| ObservatoryConversationResult {
                    schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                    status: "delivered",
                    conversation_id: "conversation".to_owned(),
                    turn_id: "turn".to_owned(),
                    recipient_id: "shepherd".to_owned(),
                    correlation_id: "00000000000000000000000000000000".to_owned(),
                    reply: None,
                    accepted_sequence: Some(1),
                    turn_sequence: Some(1),
                    error: None,
                }),
            },
        );
        ConversationSession {
            sequence,
            recipient_id: "shepherd".to_owned(),
            next_sequence: 1,
            dispatch_gate: Arc::new(ConversationDispatchGate::new()),
            turns,
        }
    }

    #[test]
    fn session_capacity_evicts_only_the_oldest_terminal_session() {
        let mut sessions = ConversationSessions::default();
        sessions
            .sessions
            .insert("old-terminal".to_owned(), session(1, true));
        sessions
            .sessions
            .insert("active".to_owned(), session(2, false));
        sessions
            .sessions
            .insert("new-terminal".to_owned(), session(3, true));

        assert!(sessions.retain_capacity_for_new_session(3));
        assert!(!sessions.sessions.contains_key("old-terminal"));
        assert!(sessions.sessions.contains_key("active"));
        assert!(sessions.sessions.contains_key("new-terminal"));
    }

    #[test]
    fn session_capacity_refuses_when_every_session_is_active() {
        let mut sessions = ConversationSessions::default();
        sessions
            .sessions
            .insert("active-1".to_owned(), session(1, false));
        sessions
            .sessions
            .insert("active-2".to_owned(), session(2, false));

        assert!(!sessions.retain_capacity_for_new_session(2));
        assert_eq!(sessions.sessions.len(), 2);
    }

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

#[cfg(test)]
mod acip_replay_tests {
    use super::*;

    fn replay_state() -> AcipReplayState {
        AcipReplayState {
            sequences_by_principal: BTreeMap::new(),
        }
    }

    fn domain(runtime_id: &str, source: &str) -> AcipReplayDomain {
        AcipReplayDomain {
            runtime_id: runtime_id.to_owned(),
            source: source.to_owned(),
        }
    }

    #[test]
    fn structured_domains_do_not_delimiter_collide() {
        let mut state = replay_state();
        assert!(reserve_replay_sequence(&mut state, 2, [1; 32], domain("a:b", "c"), 1).is_some());
        assert!(reserve_replay_sequence(&mut state, 2, [1; 32], domain("a", "b:c"), 1).is_some());
    }

    #[test]
    fn domain_capacity_is_partitioned_by_principal() {
        let mut state = replay_state();
        assert!(
            reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "one"), 1).is_some()
        );
        assert!(
            reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "two"), 1).is_none()
        );
        assert!(
            reserve_replay_sequence(&mut state, 1, [2; 32], domain("runtime", "two"), 1).is_some()
        );
    }

    #[test]
    fn rejected_excessive_advances_do_not_consume_domain_capacity() {
        let mut state = replay_state();
        for index in 0..4 {
            assert!(reserve_replay_sequence(
                &mut state,
                1,
                [1; 32],
                domain("runtime", &format!("rejected-{index}")),
                ACIP_MAX_SEQUENCE_ADVANCE + 1,
            )
            .is_none());
        }
        assert!(
            reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "valid"), 1)
                .is_some()
        );
    }

    #[test]
    fn failed_concurrent_reservations_never_resurrect() {
        let mut state = replay_state();
        let first = reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "source"), 1)
            .unwrap();
        commit_replay_sequence(&mut state, &first);
        let second =
            reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "source"), 2)
                .unwrap();
        let third = reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "source"), 3)
            .unwrap();
        rollback_replay_sequence(&mut state, second);
        rollback_replay_sequence(&mut state, third);
        assert!(
            reserve_replay_sequence(&mut state, 1, [1; 32], domain("runtime", "source"), 2)
                .is_some()
        );
    }
}
