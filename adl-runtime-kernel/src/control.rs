use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fs::{self, File, OpenOptions},
    future::Future,
    io::Write,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    sync::{Mutex, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{
        ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade},
        DefaultBodyLimit, Path as AxumPath, Query, State,
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

use crate::ComponentId;
use crate::{
    conversation_rooms::{
        GovernedRoom, GovernedRoomDeliveryState, GovernedRoomParticipant,
        GovernedRoomParticipantState, GovernedRoomRoute, GovernedRoomTurnIntent,
        GOVERNED_ROOM_ROUTE_SCHEMA,
    },
    decode_acip_envelope, is_canonical_agent_name, AgentRosterEntry, AgentRosterQuery,
    CanonicalIngress, CheckpointManifest, DomainResult, DomainWork, InferenceReadinessState,
    IngressError, KernelControl, KernelExit, LiveContinuity, ObservabilityHealth,
    ResidentShepherdInitConfig, RuntimeEvent, RuntimeRecorder, RuntimeSnapshot,
    RuntimeTlsInitConfig, WeatherHealthReport, ACIP_WEBSOCKET_SCHEMA,
};

pub const CONTROL_COMMAND_SCHEMA: &str = "adl.runtime.control_command.v1";
pub const CONTROL_RESPONSE_SCHEMA: &str = "adl.runtime.control_response.v1";
pub const LEGACY_OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v1";
pub const PREVIOUS_OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v2";
pub const OBSERVATORY_FEED_SCHEMA: &str = "adl.runtime_v3.observatory_feed.v3";
pub const MAX_SHUTDOWN_GRACE_MILLIS: u64 = 60_000;
const AGENT_PROVIDER_EXECUTION_TIMEOUT: Duration = Duration::from_secs(15 * 60);
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
pub const AGENT_ADMISSION_SCHEMA: &str = "adl.runtime_v3.agent_admission.v1";
const DYNAMIC_AGENT_STORE_SCHEMA: &str = "adl.runtime_v3.dynamic_agents.v1";
pub const FREEZE_DRIED_AGENT_SCHEMA: &str = "adl.runtime_v3.freeze_dried_agent.v1";
pub const AGENT_CHECKPOINT_SCHEMA: &str = "adl.runtime_v3.agent_checkpoint.v1";
pub const OBSERVATORY_WS_AUTH_SCHEMA: &str = "adl.runtime_v3.observatory_ws_auth.v1";
pub const OBSERVATORY_WS_CONTROL_RESULT_SCHEMA: &str =
    "adl.runtime_v3.observatory_ws_control_result.v1";
pub const OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA: &str =
    "adl.runtime_v3.observatory_conversation_intent.v1";
pub const OBSERVATORY_WS_AGENT_INITIATION_INTENT_SCHEMA: &str =
    "adl.runtime_v3.observatory_agent_initiation_intent.v1";
pub const OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA: &str =
    "adl.runtime_v3.observatory_conversation_result.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObservatoryFeedVersion {
    V1,
    V2,
    V3,
}

impl ObservatoryFeedVersion {
    fn parse(value: Option<&str>) -> Result<Self, ()> {
        match value {
            None | Some("") | Some("v2") | Some(PREVIOUS_OBSERVATORY_FEED_SCHEMA) => Ok(Self::V2),
            Some("v1") | Some(LEGACY_OBSERVATORY_FEED_SCHEMA) => Ok(Self::V1),
            Some("v3") | Some(OBSERVATORY_FEED_SCHEMA) => Ok(Self::V3),
            Some(_) => Err(()),
        }
    }
}
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentAdmissionRequest {
    pub schema: String,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub office: String,
    #[serde(default)]
    pub role: String,
    pub provider: String,
    pub model: String,
    pub endpoint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentAdmissionResponse {
    pub schema: String,
    pub status: String,
    pub agent_id: String,
    pub model: String,
    pub roster_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct DynamicAgentStore {
    schema: String,
    agents: Vec<AgentAdmissionRequest>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FreezeDriedAgent {
    pub schema: String,
    pub source_runtime_instance_id: String,
    pub declaration: AgentAdmissionRequest,
    pub checkpoint: AgentCheckpoint,
    pub dehydrated_at_unix_millis: u64,
    pub bundle_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentCheckpoint {
    pub schema: String,
    pub runtime_instance_id: String,
    pub declaration: AgentAdmissionRequest,
    pub roster_state: CheckpointAgentSample,
    pub conversation_history: Vec<AgentConversationCheckpoint>,
    pub created_at_unix_millis: u64,
    pub checkpoint_digest: String,
}

/// Stable v1 checkpoint representation. Keep this separate from the live
/// Observatory projection so additive API fields cannot change checkpoint
/// bytes or integrity digests.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CheckpointAgentSample {
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

impl From<AgentSample> for CheckpointAgentSample {
    fn from(sample: AgentSample) -> Self {
        Self {
            id: sample.id,
            label: sample.label,
            role: sample.role,
            state: sample.state,
            detail: sample.detail,
            health: sample.health,
            availability: sample.availability,
            activity: sample.activity,
            capabilities: sample.capabilities,
            location: sample.location,
            communication_eligible: sample.communication_eligible,
            observed_at_unix_millis: sample.observed_at_unix_millis,
            freshness_deadline_unix_millis: sample.freshness_deadline_unix_millis,
            source_revision: sample.source_revision,
            provenance: sample.provenance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentConversationCheckpoint {
    pub conversation_id: String,
    pub session_sequence: u64,
    pub next_turn_sequence: u64,
    pub turns: Vec<AgentTurnCheckpoint>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentTurnCheckpoint {
    pub turn_id: String,
    pub fingerprint: String,
    pub correlation_id: String,
    pub sequence: u64,
    pub terminal_status: String,
    pub reply: Option<String>,
    pub accepted_sequence: Option<u64>,
    pub turn_sequence: Option<u64>,
    pub terminal_error: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MigrationCommitRequest {
    bundle_digest: String,
}
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
    PolisIdentityFeed, RuntimeReadinessReport, RUNTIME_READINESS_SCHEMA,
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
        Self::at(1)
    }

    fn at(next_sequence: u64) -> Self {
        Self {
            state: Mutex::new(ConversationDispatchGateState {
                next_sequence,
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
        cancellation: &CancellationToken,
        deadline: tokio::time::Instant,
    ) -> bool {
        loop {
            let changed = self.changed.notified();
            if self.ready(sequence) {
                return true;
            }
            tokio::select! {
                _ = cancellation.cancelled() => return false,
                _ = tokio::time::sleep_until(deadline) => return false,
                _ = changed => {},
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
    initiation: Option<AgentInitiationMetadata>,
    sequence: u64,
    cancellation: CancellationToken,
    dispatch_gate: Arc<ConversationDispatchGate>,
    work_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct AgentInitiationMetadata {
    sender_id: String,
    initiated_recipient_id: String,
    initiated_conversation_id: String,
    initiated_turn_id: String,
    initiated_correlation_id: String,
    initiated_work_id: String,
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
        accepted: Box<ObservatoryConversationResult>,
        dispatch: ConversationDispatch,
    },
    Response(ObservatoryConversationResult),
}

pub struct ControlService<C> {
    instance_id: String,
    runtime_incarnation_id: String,
    guardian_process_id: u32,
    active_init_hash: String,
    config_generation: String,
    config_receipt_digest: String,
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
    runtime_presentation: Arc<RwLock<RuntimePresentationState>>,
    readiness_time: Option<Arc<dyn crate::TrustedTime>>,
    agent_population: RwLock<AgentPopulationFeed>,
    dynamic_agent_store: Mutex<Option<PathBuf>>,
    dynamic_agents: Mutex<Vec<AgentAdmissionRequest>>,
    pending_agent_migrations: Mutex<BTreeMap<String, FreezeDriedAgent>>,
    dynamic_agent_admission: Mutex<()>,
    control_addr: Mutex<SocketAddr>,
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
        let origins = validate_observatory_origins(observatory_allowed_origins, true)
            .expect("observatory origins must be approved exact origins");
        agent_population
            .sample
            .sort_by(|left, right| left.id.cmp(&right.id));
        let polis_identity = PolisIdentityFeed::unavailable(&instance_id);
        let runtime_presentation = Arc::new(RwLock::new(RuntimePresentationState {
            public_base_url: "https://localhost".to_owned(),
            polis_identity,
            observatory_allowed_origins: Arc::new(origins),
        }));
        let observatory_origin_policy =
            ObservatoryOriginPolicy::from_state(Arc::clone(&runtime_presentation));
        Self {
            instance_id,
            runtime_incarnation_id: uuid::Uuid::new_v4().to_string(),
            guardian_process_id: std::process::id(),
            active_init_hash: blake3::hash(b"").to_hex().to_string(),
            config_generation: blake3::hash(b"").to_hex().to_string(),
            config_receipt_digest: blake3::hash(b"").to_hex().to_string(),
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
            runtime_presentation,
            readiness_time: None,
            agent_population: RwLock::new(agent_population),
            dynamic_agent_store: Mutex::new(None),
            dynamic_agents: Mutex::new(Vec::new()),
            pending_agent_migrations: Mutex::new(BTreeMap::new()),
            dynamic_agent_admission: Mutex::new(()),
            control_addr: Mutex::new(SocketAddr::from(([127, 0, 0, 1], 0))),
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

    pub fn with_runtime_ownership(
        mut self,
        guardian_process_id: u32,
        active_init_hash: impl Into<String>,
    ) -> Self {
        assert!(
            guardian_process_id > 0,
            "Guardian process id must be non-zero"
        );
        let active_init_hash = active_init_hash.into();
        assert!(
            active_init_hash.len() == 64
                && active_init_hash
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit()),
            "active Runtime init hash must be a BLAKE3 hex digest"
        );
        self.guardian_process_id = guardian_process_id;
        self.active_init_hash = active_init_hash;
        self
    }

    pub fn with_config_generation(
        mut self,
        generation: impl Into<String>,
        receipt_digest: impl Into<String>,
    ) -> Self {
        let generation = generation.into();
        let receipt_digest = receipt_digest.into();
        assert!(
            generation.len() == 64 && generation.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "Runtime configuration generation must be a hex digest"
        );
        assert!(
            receipt_digest.len() == 64
                && receipt_digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "Runtime configuration receipt digest must be a hex digest"
        );
        self.config_generation = generation;
        self.config_receipt_digest = receipt_digest;
        self
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

    pub fn with_polis_identity(self, init: &crate::RuntimeInitConfig) -> Self {
        let mut active = self
            .runtime_presentation
            .write()
            .expect("runtime presentation state poisoned");
        active.public_base_url = init.api.public_base_url.clone();
        active.polis_identity = PolisIdentityFeed {
            polis_id: init.polis.id.clone(),
            display_name: init.polis.display_name.clone(),
            public_domain: init.polis.public_domain.clone(),
            runtime_api_base: init.api.public_base_url.clone(),
            observatory_public_origin: init.polis.observatory_public_origin.clone(),
        };
        drop(active);
        self
    }

    pub fn with_readiness_time(mut self, trusted_time: Arc<dyn crate::TrustedTime>) -> Self {
        self.readiness_time = Some(trusted_time);
        self
    }

    pub fn apply_runtime_init_reload(&self, init: &crate::RuntimeInitConfig) -> Result<(), String> {
        init.validate().map_err(|error| error.to_string())?;
        let next_identity = PolisIdentityFeed {
            polis_id: init.polis.id.clone(),
            display_name: init.polis.display_name.clone(),
            public_domain: init.polis.public_domain.clone(),
            runtime_api_base: init.api.public_base_url.clone(),
            observatory_public_origin: init.polis.observatory_public_origin.clone(),
        };
        let next_origins = Arc::new(validate_observatory_origins(
            init.observatory_allowed_origins(),
            true,
        )?);
        let mut active = self
            .runtime_presentation
            .write()
            .map_err(|_| "runtime presentation state unavailable".to_owned())?;
        active.public_base_url = init.api.public_base_url.clone();
        active.polis_identity = next_identity;
        active.observatory_allowed_origins = next_origins;
        Ok(())
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
        self.accept_conversation_intent_inner(intent, None)
    }

    fn accept_agent_initiation_intent(
        &self,
        intent: &ObservatoryAgentInitiationIntent,
    ) -> ConversationAcceptance {
        self.accept_agent_initiation_intent_inner(intent, true)
    }

    fn accept_runtime_agent_initiation_intent(
        &self,
        intent: &ObservatoryAgentInitiationIntent,
    ) -> ConversationAcceptance {
        self.accept_agent_initiation_intent_inner(intent, false)
    }

    fn accept_agent_initiation_intent_inner(
        &self,
        intent: &ObservatoryAgentInitiationIntent,
        require_sender_signing_identity: bool,
    ) -> ConversationAcceptance {
        let metadata = AgentInitiationMetadata {
            sender_id: intent.sender_id.clone(),
            initiated_recipient_id: intent.recipient_id.clone(),
            initiated_conversation_id: intent.conversation_id.clone(),
            initiated_turn_id: intent.turn_id.clone(),
            initiated_correlation_id: intent.correlation_id.clone(),
            initiated_work_id: intent.work_id.clone(),
        };
        let conversation_intent = ObservatoryConversationIntent {
            schema: OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA.to_owned(),
            conversation_id: intent.conversation_id.clone(),
            turn_id: intent.turn_id.clone(),
            recipient_id: intent.recipient_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            message: intent.message.clone(),
        };
        let outcome = |status, error| {
            ObservatoryConversationResult::from_parts(ObservatoryConversationResultParts {
                status,
                conversation_id: conversation_intent.conversation_id.clone(),
                turn_id: conversation_intent.turn_id.clone(),
                recipient_id: conversation_intent.recipient_id.clone(),
                correlation_id: conversation_intent.correlation_id.clone(),
                reply: None,
                accepted_sequence: None,
                turn_sequence: None,
                error: Some(error),
                initiation: Some(metadata.clone()),
            })
        };
        if intent.schema != OBSERVATORY_WS_AGENT_INITIATION_INTENT_SCHEMA
            || !is_safe_identifier(&intent.conversation_id)
            || !is_safe_identifier(&intent.turn_id)
            || !is_safe_identifier(&intent.sender_id)
            || !is_safe_identifier(&intent.recipient_id)
            || !is_safe_identifier(&intent.work_id)
            || !is_correlation_id(&intent.correlation_id)
            || intent.sender_id == intent.recipient_id
            || intent.message.trim().is_empty()
            || intent.message.len() > 4_096
        {
            return ConversationAcceptance::Response(outcome(
                "refused",
                "invalid_agent_initiation_intent",
            ));
        }
        match self.conversation_recipient_eligibility(&intent.sender_id) {
            Ok(Some(true)) => {}
            Ok(Some(false)) | Ok(None) => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "unauthorized_initiation",
                ));
            }
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "agent_roster_unavailable",
                ));
            }
        }
        if self.layer8_authority.is_none() {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "agent_initiation_authority_unavailable",
            ));
        }
        let Some(exchange) = self.layer8_signed_exchange.as_ref() else {
            return ConversationAcceptance::Response(outcome(
                "failed",
                "conversation_signing_unavailable",
            ));
        };
        if require_sender_signing_identity
            && exchange.sender_verifying_identity().principal_id != intent.sender_id
        {
            return ConversationAcceptance::Response(outcome(
                "refused",
                "sender_identity_mismatch",
            ));
        }
        let accepted = self.accept_conversation_intent_inner(&conversation_intent, Some(metadata));
        if matches!(accepted, ConversationAcceptance::Dispatch { .. }) {
            self.recorder.emit_correlated(
                Some(ComponentId::new("agent_initiation")),
                RuntimeEvent::AgentToAgentInitiated,
                Some(&intent.correlation_id),
            );
        }
        accepted
    }

    fn accept_conversation_intent_inner(
        &self,
        intent: &ObservatoryConversationIntent,
        initiation: Option<AgentInitiationMetadata>,
    ) -> ConversationAcceptance {
        let outcome = |status, error, sequence| {
            ObservatoryConversationResult::from_parts(ObservatoryConversationResultParts {
                status,
                conversation_id: intent.conversation_id.clone(),
                turn_id: intent.turn_id.clone(),
                recipient_id: intent.recipient_id.clone(),
                correlation_id: intent.correlation_id.clone(),
                reply: None,
                accepted_sequence: None,
                turn_sequence: sequence,
                error: Some(error),
                initiation: initiation.clone(),
            })
        };
        let initiated_work_id = initiation
            .as_ref()
            .map(|metadata| metadata.initiated_work_id.clone());
        let fingerprint_source = serde_json::json!({
            "intent": intent,
            "initiation": initiation,
        });
        let is_initiated = initiation.is_some();
        let valid_intent =
            intent.schema == OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA || is_initiated;
        if !valid_intent
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
        let fingerprint = match serde_json::to_vec(&fingerprint_source) {
            Ok(bytes) => blake3::hash(&bytes).to_hex().to_string(),
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "invalid_conversation_intent",
                    None,
                ));
            }
        };
        let work_id = initiated_work_id.unwrap_or_else(|| {
            format!(
                "conversation-{}",
                &blake3::hash(format!("{}:{}", intent.conversation_id, intent.turn_id).as_bytes())
                    .to_hex()[..32]
            )
        });
        let recipient = match self.conversation_recipient_eligibility(&intent.recipient_id) {
            Ok(recipient) => recipient,
            Err(_) => {
                return ConversationAcceptance::Response(outcome(
                    "failed",
                    "agent_roster_unavailable",
                    None,
                ));
            }
        };
        match recipient {
            None => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "unknown_recipient",
                    None,
                ));
            }
            Some(false) => {
                return ConversationAcceptance::Response(outcome(
                    "refused",
                    "recipient_unavailable",
                    None,
                ));
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
                        ));
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
                        ));
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
            sender_id: initiation
                .as_ref()
                .map(|metadata| metadata.sender_id.clone()),
            initiated_recipient_id: initiation
                .as_ref()
                .map(|metadata| metadata.initiated_recipient_id.clone()),
            initiated_conversation_id: initiation
                .as_ref()
                .map(|metadata| metadata.initiated_conversation_id.clone()),
            initiated_turn_id: initiation
                .as_ref()
                .map(|metadata| metadata.initiated_turn_id.clone()),
            initiated_correlation_id: initiation
                .as_ref()
                .map(|metadata| metadata.initiated_correlation_id.clone()),
            initiated_work_id: initiation
                .as_ref()
                .map(|metadata| metadata.initiated_work_id.clone()),
            initiated_reply: None,
            reply: None,
            accepted_sequence: None,
            turn_sequence: Some(sequence),
            error: None,
        };
        ConversationAcceptance::Dispatch {
            accepted: Box::new(accepted),
            dispatch: ConversationDispatch {
                intent: intent.clone(),
                initiation,
                sequence,
                cancellation,
                dispatch_gate: session.dispatch_gate.clone(),
                work_id,
            },
        }
    }

    fn agent_initiation_intent_from_public_output(
        dispatch: &ConversationDispatch,
        output: &serde_json::Value,
    ) -> Result<Option<ObservatoryAgentInitiationIntent>, &'static str> {
        let Some(action) = output.get("agent_to_agent_initiation") else {
            return Ok(None);
        };
        if action.get("schema").and_then(serde_json::Value::as_str)
            != Some(crate::ingress::AGENT_TO_AGENT_INITIATION_REQUEST_SCHEMA)
        {
            return Err("invalid_agent_initiation_action");
        }
        let field = |name| {
            action
                .get(name)
                .and_then(serde_json::Value::as_str)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .ok_or("invalid_agent_initiation_action")
        };
        let recipient_id = field("recipient_id")?;
        let message = field("message")?;
        let sender_id = dispatch.intent.recipient_id.clone();
        let seed = serde_json::json!({
            "schema": "adl.runtime.agent_to_agent_runtime_derived_ids.v1",
            "parent_conversation_id": dispatch.intent.conversation_id,
            "parent_turn_id": dispatch.intent.turn_id,
            "parent_correlation_id": dispatch.intent.correlation_id,
            "sender_id": sender_id,
            "recipient_id": recipient_id,
            "message": message,
        });
        let seed_bytes =
            serde_json::to_vec(&seed).map_err(|_| "invalid_agent_initiation_action")?;
        let digest = blake3::hash(&seed_bytes).to_hex().to_string();
        Ok(Some(ObservatoryAgentInitiationIntent {
            schema: OBSERVATORY_WS_AGENT_INITIATION_INTENT_SCHEMA.to_owned(),
            conversation_id: format!("a2a-{}-{}-{}", sender_id, recipient_id, &digest[..16]),
            turn_id: format!("turn-a2a-{}", &digest[16..32]),
            sender_id,
            recipient_id,
            correlation_id: digest[..32].to_owned(),
            work_id: format!("a2a-work-{}", &digest[32..48]),
            message,
        }))
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
            sender_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.sender_id.clone()),
            initiated_recipient_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_recipient_id.clone()),
            initiated_conversation_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_conversation_id.clone()),
            initiated_turn_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_turn_id.clone()),
            initiated_correlation_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_correlation_id.clone()),
            initiated_work_id: dispatch
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_work_id.clone()),
            initiated_reply: None,
            reply: None,
            accepted_sequence: None,
            turn_sequence: Some(dispatch.sequence),
            error: Some(error),
        };
        let mut dispatch_gate_completed = false;
        let shepherd_name = self
            .agent_population
            .read()
            .expect("agent population lock poisoned")
            .sample
            .iter()
            .find(|agent| {
                agent.id == dispatch.intent.recipient_id
                    && (agent.id == "shepherd" || agent.id.starts_with("shepherd:"))
            })
            .map(|agent| agent.name.clone());
        let dynamic_binding = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .iter()
            .find(|agent| agent.id == dispatch.intent.recipient_id)
            .cloned();
        let agent_task = match dynamic_binding {
            Some(agent) => serde_json::json!({
                "op": "conversation_message",
                "recipient_id": dispatch.intent.recipient_id,
                "conversation_id": dispatch.intent.conversation_id,
                "turn_id": dispatch.intent.turn_id,
                "correlation_id": dispatch.intent.correlation_id,
                "input": dispatch.intent.message,
                "sender_id": dispatch.initiation.as_ref().map(|metadata| metadata.sender_id.clone()),
                "initiated_work_id": dispatch.initiation.as_ref().map(|metadata| metadata.initiated_work_id.clone()),
                "provider": agent.provider,
                "model": agent.model,
                "endpoint": agent.endpoint,
            }),
            None => serde_json::json!({
                "op": "conversation_message",
                "recipient_id": dispatch.intent.recipient_id,
                "conversation_id": dispatch.intent.conversation_id,
                "turn_id": dispatch.intent.turn_id,
                "correlation_id": dispatch.intent.correlation_id,
                "input": dispatch.intent.message,
                "sender_id": dispatch.initiation.as_ref().map(|metadata| metadata.sender_id.clone()),
                "initiated_work_id": dispatch.initiation.as_ref().map(|metadata| metadata.initiated_work_id.clone()),
            }),
        };
        let (work_kind, payload) = if let Some(shepherd_name) = shepherd_name {
            (
                "shepherd",
                serde_json::to_vec(&crate::ShepherdRequest {
                    schema: crate::SHEPHERD_REQUEST_SCHEMA.to_owned(),
                    correlation_id: dispatch.intent.correlation_id.clone(),
                    runtime_id: self.instance_id.clone(),
                    shepherd_name: Some(shepherd_name),
                    conversation_recipient_id: Some(dispatch.intent.recipient_id.clone()),
                    prompt: dispatch.intent.message.clone(),
                }),
            )
        } else {
            (
                "agent_runtime",
                serde_json::to_vec(&serde_json::json!({
                    "schema": "adl.runtime.local_agent_work.v1",
                    "tasks": [agent_task],
                })),
            )
        };
        // Conversation execution is not an authentication handshake. Give local
        // models enough room for cold starts while preserving explicit operator
        // cancellation and bounded shutdown behavior.
        // Queueing and provider execution have independent allowances. A turn
        // may wait generously for earlier work, but cannot remain stuck behind
        // a lost sequence forever. Once admitted, it receives a fresh provider
        // execution window below.
        let queue_deadline = tokio::time::Instant::now() + Duration::from_secs(600);
        let turn_ready = dispatch
            .dispatch_gate
            .wait_turn(dispatch.sequence, &dispatch.cancellation, queue_deadline)
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
                    let deadline = tokio::time::Instant::now() + AGENT_PROVIDER_EXECUTION_TIMEOUT;
                    let submit = ingress.submit_with_cancellation(
                        DomainWork {
                            schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
                            work_id: dispatch.work_id.clone(),
                            kind: work_kind.to_owned(),
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
                            let public_output = result.public_output.as_ref();
                            let reply = public_output
                                .and_then(|output| output.get("message"))
                                .and_then(serde_json::Value::as_str)
                                .map(str::to_owned);
                            let requested_agent_initiation = if dispatch.initiation.is_none() {
                                public_output
                                    .map(|output| {
                                        Self::agent_initiation_intent_from_public_output(
                                            &dispatch, output,
                                        )
                                    })
                                    .unwrap_or(Ok(None))
                            } else {
                                Ok(None)
                            };
                            match reply {
                                Some(reply) => match requested_agent_initiation {
                                    Ok(Some(intent)) => {
                                        dispatch.dispatch_gate.complete(dispatch.sequence);
                                        dispatch_gate_completed = true;
                                        let initiated = match self
                                            .accept_runtime_agent_initiation_intent(&intent)
                                        {
                                            ConversationAcceptance::Dispatch {
                                                dispatch, ..
                                            } => {
                                                Box::pin(
                                                    self.complete_conversation_dispatch(dispatch),
                                                )
                                                .await
                                            }
                                            ConversationAcceptance::Response(response) => response,
                                        };
                                        ObservatoryConversationResult {
                                            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                                            status: initiated.status,
                                            conversation_id: dispatch
                                                .intent
                                                .conversation_id
                                                .clone(),
                                            turn_id: dispatch.intent.turn_id.clone(),
                                            recipient_id: dispatch.intent.recipient_id.clone(),
                                            correlation_id: dispatch.intent.correlation_id.clone(),
                                            sender_id: Some(intent.sender_id),
                                            initiated_recipient_id: Some(intent.recipient_id),
                                            initiated_conversation_id: Some(intent.conversation_id),
                                            initiated_turn_id: Some(intent.turn_id),
                                            initiated_correlation_id: Some(intent.correlation_id),
                                            initiated_work_id: Some(intent.work_id),
                                            initiated_reply: initiated.reply,
                                            // The initiating agent's operator-facing reply and
                                            // the recipient's governed result are separate facts.
                                            // The latter remains correlated through the initiated
                                            // identifiers and Runtime events; do not replace the
                                            // former with peer output.
                                            reply: Some(reply),
                                            accepted_sequence: initiated
                                                .accepted_sequence
                                                .or(Some(result.accepted_sequence)),
                                            turn_sequence: Some(dispatch.sequence),
                                            error: initiated.error,
                                        }
                                    }
                                    Ok(None) => ObservatoryConversationResult {
                                        schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                                        status: "delivered",
                                        conversation_id: dispatch.intent.conversation_id.clone(),
                                        turn_id: dispatch.intent.turn_id.clone(),
                                        recipient_id: dispatch.intent.recipient_id.clone(),
                                        correlation_id: dispatch.intent.correlation_id.clone(),
                                        sender_id: dispatch
                                            .initiation
                                            .as_ref()
                                            .map(|metadata| metadata.sender_id.clone()),
                                        initiated_recipient_id: dispatch.initiation.as_ref().map(
                                            |metadata| metadata.initiated_recipient_id.clone(),
                                        ),
                                        initiated_conversation_id: dispatch
                                            .initiation
                                            .as_ref()
                                            .map(|metadata| {
                                                metadata.initiated_conversation_id.clone()
                                            }),
                                        initiated_turn_id: dispatch
                                            .initiation
                                            .as_ref()
                                            .map(|metadata| metadata.initiated_turn_id.clone()),
                                        initiated_correlation_id: dispatch.initiation.as_ref().map(
                                            |metadata| metadata.initiated_correlation_id.clone(),
                                        ),
                                        initiated_work_id: dispatch
                                            .initiation
                                            .as_ref()
                                            .map(|metadata| metadata.initiated_work_id.clone()),
                                        initiated_reply: None,
                                        reply: Some(reply),
                                        accepted_sequence: Some(result.accepted_sequence),
                                        turn_sequence: Some(dispatch.sequence),
                                        error: None,
                                    },
                                    Err(error) => outcome("refused", error),
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
        if result.status == "delivered" && dispatch.initiation.is_some() {
            self.recorder.emit_correlated(
                Some(ComponentId::new("agent_initiation")),
                RuntimeEvent::AgentToAgentCompleted,
                Some(&dispatch.intent.correlation_id),
            );
        } else if dispatch.initiation.is_some() {
            self.recorder.emit_correlated(
                Some(ComponentId::new("agent_initiation")),
                RuntimeEvent::AgentToAgentFailed,
                Some(&dispatch.intent.correlation_id),
            );
        }
        if !dispatch_gate_completed {
            dispatch.dispatch_gate.complete(dispatch.sequence);
        }
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
            sender_id: None,
            initiated_recipient_id: None,
            initiated_conversation_id: None,
            initiated_turn_id: None,
            initiated_correlation_id: None,
            initiated_work_id: None,
            initiated_reply: None,
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

    pub fn configure_dynamic_agent_store(&self, path: PathBuf) -> Result<(), ControlError> {
        let agents = if path.exists() {
            let bytes = fs::read(&path).map_err(|error| ControlError::Io(error.to_string()))?;
            let store: DynamicAgentStore = serde_json::from_slice(&bytes)
                .map_err(|error| ControlError::Encoding(error.to_string()))?;
            if store.schema != DYNAMIC_AGENT_STORE_SCHEMA {
                return Err(ControlError::InvalidIdentifier);
            }
            store.agents
        } else {
            Vec::new()
        };
        let mut seen = BTreeSet::new();
        let mut population = self
            .agent_population
            .write()
            .expect("agent population state poisoned");
        for agent in &agents {
            validate_persisted_agent_admission(agent)?;
            if !seen.insert(agent.id.clone()) {
                return Err(ControlError::InvalidIdentifier);
            }
            let mut sample = agent_sample(agent);
            sample.name = persisted_agent_canonical_name(agent);
            population.admit_dynamic(sample);
        }
        *self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned") = agents;
        *self
            .dynamic_agent_store
            .lock()
            .expect("dynamic agent store state poisoned") = Some(path);
        Ok(())
    }

    pub async fn refresh_dynamic_agent_health(&self) {
        let declarations = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .clone();
        let mut checks = tokio::task::JoinSet::new();
        for declaration in declarations {
            if self
                .pending_agent_migrations
                .lock()
                .expect("pending migration state poisoned")
                .contains_key(&declaration.id)
            {
                continue;
            }
            checks.spawn(async move {
                let (readiness, failure_reason) = match verify_ollama_model(&declaration).await {
                    Ok(()) => (InferenceReadinessState::Ready, None),
                    Err(failure) => (
                        inference_readiness_from_agent_admission_failure(&failure),
                        Some(agent_admission_failure_reason(&failure).to_owned()),
                    ),
                };
                (declaration, readiness, failure_reason, now_unix_millis())
            });
        }
        while let Some(Ok((declaration, readiness, failure_reason, observed_at_unix_millis))) =
            checks.join_next().await
        {
            let mut population = self
                .agent_population
                .write()
                .expect("agent population state poisoned");
            let Some(sample) = population
                .sample
                .iter_mut()
                .find(|sample| sample.id == declaration.id)
            else {
                continue;
            };
            sample.observed_at_unix_millis = observed_at_unix_millis;
            sample.freshness_deadline_unix_millis =
                observed_at_unix_millis.saturating_add(crate::AGENT_ADMISSION_HEARTBEAT_TTL_MILLIS);
            let projection = readiness.projection();
            sample.inference_readiness = readiness;
            sample.state = readiness.as_str().to_owned();
            sample.health = projection.health.to_owned();
            sample.availability = projection.availability.to_owned();
            sample.communication_eligible = projection.communication_eligible;
            sample.activity = projection.activity.map(str::to_owned);
            sample.detail = if readiness == InferenceReadinessState::Ready {
                format!("ollama model {} verified", declaration.model)
            } else if let Some(reason) = failure_reason {
                format!("Ollama provider health verification failed: {reason}")
            } else {
                "Ollama provider health verification failed".to_owned()
            };
        }
    }

    async fn admit_agent(
        &self,
        request: AgentAdmissionRequest,
    ) -> Result<AgentAdmissionResponse, AgentAdmissionFailure> {
        validate_agent_admission(&request)
            .map_err(|_| AgentAdmissionFailure::Invalid("invalid_agent_declaration"))?;
        verify_agent_provider_route(&request).await?;
        let _transaction = self
            .dynamic_agent_admission
            .lock()
            .expect("dynamic agent admission mutex poisoned");
        let path = self
            .dynamic_agent_store
            .lock()
            .expect("dynamic agent store state poisoned")
            .clone()
            .ok_or(AgentAdmissionFailure::Unavailable(
                "dynamic_store_unconfigured",
            ))?;
        let mut agents_guard = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned");
        let mut agents = agents_guard.clone();
        let status = match agents.iter().find(|agent| agent.id == request.id) {
            Some(existing) if existing == &request => "already_present",
            Some(_) => return Err(AgentAdmissionFailure::Conflict("agent_id_conflict")),
            None => {
                agents.push(request.clone());
                agents.sort_by(|left, right| left.id.cmp(&right.id));
                persist_dynamic_agents(&path, &agents)
                    .map_err(|_| AgentAdmissionFailure::Unavailable("persistence_failed"))?;
                self.agent_population
                    .write()
                    .expect("agent population state poisoned")
                    .admit_dynamic(agent_sample(&request));
                *agents_guard = agents;
                "admitted"
            }
        };
        Ok(AgentAdmissionResponse {
            schema: AGENT_ADMISSION_SCHEMA.to_owned(),
            status: status.to_owned(),
            agent_id: request.id,
            model: request.model,
            roster_path: "/v1/agents".to_owned(),
        })
    }

    fn remove_agent(&self, agent_id: &str) -> Result<&'static str, AgentAdmissionFailure> {
        if agent_id == "shepherd" || !is_safe_identifier(agent_id) {
            return Err(AgentAdmissionFailure::Invalid("protected_or_invalid_agent"));
        }
        let _transaction = self
            .dynamic_agent_admission
            .lock()
            .expect("dynamic agent admission mutex poisoned");
        let path = self
            .dynamic_agent_store
            .lock()
            .expect("dynamic agent store state poisoned")
            .clone()
            .ok_or(AgentAdmissionFailure::Unavailable(
                "dynamic_store_unconfigured",
            ))?;
        let mut agents = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned");
        if !agents.iter().any(|agent| agent.id == agent_id) {
            return Ok("already_absent");
        }
        let next = agents
            .iter()
            .filter(|agent| agent.id != agent_id)
            .cloned()
            .collect::<Vec<_>>();
        persist_dynamic_agents(&path, &next)
            .map_err(|_| AgentAdmissionFailure::Unavailable("persistence_failed"))?;
        *agents = next;
        self.agent_population
            .write()
            .expect("agent population state poisoned")
            .remove_dynamic(agent_id);
        self.conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned")
            .sessions
            .retain(|_, session| session.recipient_id != agent_id);
        self.pending_agent_migrations
            .lock()
            .expect("pending migration state poisoned")
            .remove(agent_id);
        Ok("removed")
    }

    fn dehydrate_agent(&self, agent_id: &str) -> Result<FreezeDriedAgent, AgentAdmissionFailure> {
        if agent_id == "shepherd" || !is_safe_identifier(agent_id) {
            return Err(AgentAdmissionFailure::Invalid("protected_or_invalid_agent"));
        }
        if let Some(bundle) = self
            .pending_agent_migrations
            .lock()
            .expect("pending migration state poisoned")
            .get(agent_id)
            .cloned()
        {
            return Ok(bundle);
        }
        let declaration = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .iter()
            .find(|agent| agent.id == agent_id)
            .cloned()
            .ok_or(AgentAdmissionFailure::Invalid("agent_not_found"))?;
        let checkpoint = self.checkpoint_agent(agent_id)?;
        let mut population = self
            .agent_population
            .write()
            .expect("agent population state poisoned");
        let mut bundle = FreezeDriedAgent {
            schema: FREEZE_DRIED_AGENT_SCHEMA.to_owned(),
            source_runtime_instance_id: self.instance_id.clone(),
            declaration,
            checkpoint,
            dehydrated_at_unix_millis: now_unix_millis(),
            bundle_digest: String::new(),
        };
        bundle.bundle_digest = freeze_dried_agent_digest(&bundle)
            .map_err(|_| AgentAdmissionFailure::Unavailable("bundle_encoding_failed"))?;
        if let Some(sample) = population
            .sample
            .iter_mut()
            .find(|agent| agent.id == agent_id)
        {
            sample.state = "migrating".to_owned();
            sample.availability = "unavailable".to_owned();
            sample.communication_eligible = false;
            sample.detail = "Freeze-dried migration awaiting durable commit".to_owned();
        }
        drop(population);
        self.pending_agent_migrations
            .lock()
            .expect("pending migration state poisoned")
            .insert(agent_id.to_owned(), bundle.clone());
        Ok(bundle)
    }

    fn checkpoint_agent(&self, agent_id: &str) -> Result<AgentCheckpoint, AgentAdmissionFailure> {
        if agent_id == "shepherd" || !is_safe_identifier(agent_id) {
            return Err(AgentAdmissionFailure::Invalid("protected_or_invalid_agent"));
        }
        let declaration = self
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .iter()
            .find(|agent| agent.id == agent_id)
            .cloned()
            .ok_or(AgentAdmissionFailure::Invalid("agent_not_found"))?;
        let roster_state = self
            .agent_population
            .read()
            .expect("agent population state poisoned")
            .sample
            .iter()
            .find(|agent| agent.id == agent_id)
            .cloned()
            .ok_or(AgentAdmissionFailure::Invalid("agent_not_found"))?;
        let conversation_history =
            self.conversation_sessions
                .lock()
                .expect("conversation sessions mutex poisoned")
                .sessions
                .iter()
                .filter(|(_, session)| session.recipient_id == agent_id)
                .map(|(conversation_id, session)| {
                    let turns =
                        session
                            .turns
                            .iter()
                            .map(|(turn_id, turn)| {
                                let terminal = turn.terminal.clone().ok_or(
                                    AgentAdmissionFailure::Conflict("agent_conversation_in_flight"),
                                )?;
                                Ok(AgentTurnCheckpoint {
                                    turn_id: turn_id.clone(),
                                    fingerprint: turn.fingerprint.clone(),
                                    correlation_id: turn.correlation_id.clone(),
                                    sequence: turn.sequence,
                                    terminal_status: terminal.status.to_owned(),
                                    reply: terminal.reply,
                                    accepted_sequence: terminal.accepted_sequence,
                                    turn_sequence: terminal.turn_sequence,
                                    terminal_error: terminal.error.map(str::to_owned),
                                })
                            })
                            .collect::<Result<Vec<_>, AgentAdmissionFailure>>()?;
                    Ok(AgentConversationCheckpoint {
                        conversation_id: conversation_id.clone(),
                        session_sequence: session.sequence,
                        next_turn_sequence: session.next_sequence,
                        turns,
                    })
                })
                .collect::<Result<Vec<_>, AgentAdmissionFailure>>()?;
        let mut checkpoint = AgentCheckpoint {
            schema: AGENT_CHECKPOINT_SCHEMA.to_owned(),
            runtime_instance_id: self.instance_id.clone(),
            declaration,
            roster_state: roster_state.into(),
            conversation_history,
            created_at_unix_millis: now_unix_millis(),
            checkpoint_digest: String::new(),
        };
        checkpoint.checkpoint_digest = agent_checkpoint_digest(&checkpoint)
            .map_err(|_| AgentAdmissionFailure::Unavailable("checkpoint_encoding_failed"))?;
        let store = self
            .dynamic_agent_store
            .lock()
            .expect("dynamic agent store state poisoned")
            .clone()
            .ok_or(AgentAdmissionFailure::Unavailable(
                "dynamic_store_unconfigured",
            ))?;
        let path = store
            .parent()
            .ok_or(AgentAdmissionFailure::Unavailable(
                "dynamic_store_unconfigured",
            ))?
            .join("agent-checkpoints")
            .join(format!("{agent_id}.json"));
        persist_json_atomically(&path, &checkpoint)
            .map_err(|_| AgentAdmissionFailure::Unavailable("checkpoint_persistence_failed"))?;
        Ok(checkpoint)
    }

    fn commit_agent_migration(
        &self,
        agent_id: &str,
        bundle_digest: &str,
    ) -> Result<&'static str, AgentAdmissionFailure> {
        let pending = self
            .pending_agent_migrations
            .lock()
            .expect("pending migration state poisoned")
            .get(agent_id)
            .cloned()
            .ok_or(AgentAdmissionFailure::Conflict("migration_not_pending"))?;
        if pending.bundle_digest != bundle_digest {
            return Err(AgentAdmissionFailure::Conflict("migration_digest_conflict"));
        }
        self.remove_agent(agent_id)
    }

    async fn rehydrate_agent(
        &self,
        bundle: FreezeDriedAgent,
    ) -> Result<AgentAdmissionResponse, AgentAdmissionFailure> {
        if bundle.schema != FREEZE_DRIED_AGENT_SCHEMA
            || bundle.declaration.id != bundle.checkpoint.roster_state.id
            || bundle.declaration != bundle.checkpoint.declaration
            || agent_checkpoint_digest(&bundle.checkpoint)
                .map_err(|_| AgentAdmissionFailure::Invalid("bundle_invalid"))?
                != bundle.checkpoint.checkpoint_digest
            || freeze_dried_agent_digest(&bundle)
                .map_err(|_| AgentAdmissionFailure::Invalid("bundle_invalid"))?
                != bundle.bundle_digest
        {
            return Err(AgentAdmissionFailure::Invalid("bundle_integrity_failed"));
        }
        self.validate_agent_conversation_checkpoint(&bundle)?;
        let response = self.admit_agent(bundle.declaration.clone()).await?;
        if let Err(error) = self.restore_agent_conversations(&bundle) {
            let _ = self.remove_agent(&bundle.declaration.id);
            return Err(error);
        }
        Ok(response)
    }

    fn validate_agent_conversation_checkpoint(
        &self,
        bundle: &FreezeDriedAgent,
    ) -> Result<(), AgentAdmissionFailure> {
        let sessions = self
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        let mut conversation_ids = BTreeSet::new();
        for conversation in &bundle.checkpoint.conversation_history {
            if !is_safe_identifier(&conversation.conversation_id)
                || !conversation_ids.insert(conversation.conversation_id.as_str())
                || sessions
                    .sessions
                    .contains_key(&conversation.conversation_id)
            {
                return Err(AgentAdmissionFailure::Conflict(
                    "conversation_restore_conflict",
                ));
            }
            let mut turn_ids = BTreeSet::new();
            for turn in &conversation.turns {
                if !is_safe_identifier(&turn.turn_id)
                    || !turn_ids.insert(turn.turn_id.as_str())
                    || turn.sequence == 0
                    || !matches!(
                        turn.terminal_status.as_str(),
                        "delivered" | "failed" | "refused" | "cancelled" | "timed_out"
                    )
                {
                    return Err(AgentAdmissionFailure::Invalid(
                        "conversation_checkpoint_invalid",
                    ));
                }
            }
        }
        Ok(())
    }

    fn restore_agent_conversations(
        &self,
        bundle: &FreezeDriedAgent,
    ) -> Result<(), AgentAdmissionFailure> {
        let mut sessions = self
            .conversation_sessions
            .lock()
            .expect("conversation sessions mutex poisoned");
        if bundle
            .checkpoint
            .conversation_history
            .iter()
            .any(|conversation| {
                sessions
                    .sessions
                    .contains_key(&conversation.conversation_id)
            })
        {
            return Err(AgentAdmissionFailure::Conflict(
                "conversation_restore_conflict",
            ));
        }
        for conversation in &bundle.checkpoint.conversation_history {
            let mut turns = BTreeMap::new();
            for turn in &conversation.turns {
                let terminal = ObservatoryConversationResult {
                    schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
                    status: match turn.terminal_status.as_str() {
                        "delivered" => "delivered",
                        "failed" => "failed",
                        "refused" => "refused",
                        "cancelled" => "cancelled",
                        "timed_out" => "timed_out",
                        _ => {
                            return Err(AgentAdmissionFailure::Invalid(
                                "conversation_checkpoint_invalid",
                            ));
                        }
                    },
                    conversation_id: conversation.conversation_id.clone(),
                    turn_id: turn.turn_id.clone(),
                    recipient_id: bundle.declaration.id.clone(),
                    correlation_id: turn.correlation_id.clone(),
                    sender_id: None,
                    initiated_recipient_id: None,
                    initiated_conversation_id: None,
                    initiated_turn_id: None,
                    initiated_correlation_id: None,
                    initiated_work_id: None,
                    initiated_reply: None,
                    reply: turn.reply.clone(),
                    accepted_sequence: turn.accepted_sequence,
                    turn_sequence: turn.turn_sequence,
                    error: turn
                        .terminal_error
                        .as_ref()
                        .map(|_| "restored_conversation_terminal"),
                };
                let (completion, _) = tokio::sync::watch::channel(Some(terminal.clone()));
                turns.insert(
                    turn.turn_id.clone(),
                    ConversationTurn {
                        fingerprint: turn.fingerprint.clone(),
                        correlation_id: turn.correlation_id.clone(),
                        sequence: turn.sequence,
                        cancellation: CancellationToken::new(),
                        completion,
                        terminal: Some(terminal),
                    },
                );
            }
            sessions.next_sequence = sessions.next_sequence.max(conversation.session_sequence);
            sessions.sessions.insert(
                conversation.conversation_id.clone(),
                ConversationSession {
                    sequence: conversation.session_sequence,
                    recipient_id: bundle.declaration.id.clone(),
                    next_sequence: conversation.next_turn_sequence,
                    dispatch_gate: Arc::new(ConversationDispatchGate::at(
                        conversation.next_turn_sequence.saturating_add(1),
                    )),
                    turns,
                },
            );
        }
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
        self.runtime_presentation
            .write()
            .expect("runtime presentation state poisoned")
            .public_base_url = public_base_url.to_owned();
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
        let agents = self
            .agent_population
            .read()
            .expect("agent population state poisoned")
            .with_runtime_snapshot_query(
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
        let runtime_presentation = self
            .runtime_presentation
            .read()
            .expect("runtime presentation state poisoned")
            .clone();
        ObservatoryFeed {
            schema: OBSERVATORY_FEED_SCHEMA.to_owned(),
            polis_identity: runtime_presentation.polis_identity,
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
                public_base_url: runtime_presentation.public_base_url,
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

    fn observatory_feed_projection(&self, version: ObservatoryFeedVersion) -> serde_json::Value {
        let mut value = serde_json::to_value(self.observatory_feed())
            .expect("Observatory feed must remain serializable");
        let Some(feed) = value.as_object_mut() else {
            unreachable!("Observatory feed must serialize as an object");
        };
        match version {
            ObservatoryFeedVersion::V3 => value,
            ObservatoryFeedVersion::V2 => {
                feed.insert(
                    "schema".to_owned(),
                    serde_json::Value::String(PREVIOUS_OBSERVATORY_FEED_SCHEMA.to_owned()),
                );
                feed.remove("polis_identity");
                value
            }
            ObservatoryFeedVersion::V1 => {
                feed.insert(
                    "schema".to_owned(),
                    serde_json::Value::String(LEGACY_OBSERVATORY_FEED_SCHEMA.to_owned()),
                );
                for field in [
                    "polis_identity",
                    "runtime_incarnation_id",
                    "runtime_process_id",
                    "weather_freshness",
                    "ingress",
                ] {
                    feed.remove(field);
                }
                if let Some(control) = feed
                    .get_mut("control")
                    .and_then(serde_json::Value::as_object_mut)
                {
                    for field in [
                        "public_base_url",
                        "websocket_endpoint",
                        "websocket_full_duplex",
                        "websocket_acip_binary_schema",
                        "bearer_token_required_for_read",
                        "login_required_for_mutation",
                    ] {
                        control.remove(field);
                    }
                }
                if let Some(agents) = feed
                    .get_mut("agents")
                    .and_then(serde_json::Value::as_object_mut)
                {
                    for field in [
                        "schema",
                        "revision",
                        "scope",
                        "has_more",
                        "next_page_token",
                        "event_cursor",
                        "population_complete",
                    ] {
                        agents.remove(field);
                    }
                    if let Some(sample) = agents
                        .get_mut("sample")
                        .and_then(serde_json::Value::as_array_mut)
                    {
                        for agent in sample {
                            if let Some(agent) = agent.as_object_mut() {
                                agent.retain(|field, _| {
                                    matches!(
                                        field.as_str(),
                                        "id" | "label" | "role" | "state" | "detail"
                                    )
                                });
                            }
                        }
                    }
                }
                value
            }
        }
    }

    pub fn readiness_report(&self) -> RuntimeReadinessReport {
        let feed = self.observatory_feed();
        let now = self.readiness_now_unix_millis();
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
        let shepherd_ready = shepherd_admission_is_fresh(&feed.health.snapshot, now);
        if !shepherd_ready {
            degraded_reasons.push("shepherd_not_admitted".to_owned());
        }
        RuntimeReadinessReport {
            schema: RUNTIME_READINESS_SCHEMA.to_owned(),
            ready: degraded_reasons.is_empty(),
            lifecycle: feed.health.snapshot.lifecycle,
            observability_ready: feed.health.observability_ready,
            runtime_instance_id: feed.runtime_instance_id,
            runtime_incarnation_id: feed.runtime_incarnation_id,
            runtime_process_id: feed.runtime_process_id,
            guardian_process_id: self.guardian_process_id,
            active_init_hash: self.active_init_hash.clone(),
            config_generation: self.config_generation.clone(),
            config_receipt_digest: self.config_receipt_digest.clone(),
            weather_freshness,
            degraded_reasons,
        }
    }

    fn readiness_now_unix_millis(&self) -> u64 {
        self.readiness_time
            .as_ref()
            .map_or_else(now_unix_millis, |trusted_time| {
                trusted_time.now_unix_millis()
            })
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
            .read()
            .expect("agent population state poisoned")
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
            .read()
            .expect("agent population state poisoned")
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

    pub fn update_resident_shepherd_health(&self, name: &str, state: &str, detail: &str) {
        self.agent_population
            .write()
            .expect("agent population state poisoned")
            .update_resident_shepherd_health(name, state, detail);
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
        .route(
            RUNTIME_HEALTH_PATH,
            get(runtime_health_handler::<C>).options(observatory_preflight_handler::<C>),
        )
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
            get(agent_roster_handler::<C>)
                .post(agent_admission_handler::<C>)
                .options(control_preflight_handler::<C>)
                .layer(DefaultBodyLimit::max(api_policy.control_max_body_bytes)),
        )
        .route(
            "/v1/agents/{agent_id}",
            get(agent_detail_handler::<C>)
                .delete(agent_remove_handler::<C>)
                .options(control_preflight_handler::<C>),
        )
        .route(
            "/v1/agents/{agent_id}/checkpoint",
            post(agent_checkpoint_handler::<C>).options(control_preflight_handler::<C>),
        )
        .route(
            "/v1/agents/{agent_id}/dehydrate",
            post(agent_dehydrate_handler::<C>).options(control_preflight_handler::<C>),
        )
        .route(
            "/v1/agents/{agent_id}/dehydrate/commit",
            post(agent_migration_commit_handler::<C>).options(control_preflight_handler::<C>),
        )
        .route(
            "/v1/agents/rehydrate",
            post(agent_rehydrate_handler::<C>).options(control_preflight_handler::<C>),
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
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    observatory_json(
        StatusCode::OK,
        service.observatory_feed().health,
        allowed_origin,
    )
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

async fn agent_admission_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
    Json(request): Json<AgentAdmissionRequest>,
) -> Response {
    let authorized = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| service.acip_write_token_authorized(token));
    if !authorized {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error":"authentication_required"})),
        )
            .into_response();
    }
    match service.admit_agent(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(AgentAdmissionFailure::Invalid(reason)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({"error":reason})),
        )
            .into_response(),
        Err(AgentAdmissionFailure::Conflict(reason)) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error":reason})),
        )
            .into_response(),
        Err(AgentAdmissionFailure::Unavailable(reason)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"error":reason})),
        )
            .into_response(),
    }
}

fn agent_write_authorized<C: LifecycleControl + 'static>(
    service: &ControlService<C>,
    headers: &HeaderMap,
) -> bool {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| service.acip_write_token_authorized(token))
}

fn agent_failure_response(error: AgentAdmissionFailure) -> Response {
    let (status, reason) = match error {
        AgentAdmissionFailure::Invalid(reason) => (StatusCode::UNPROCESSABLE_ENTITY, reason),
        AgentAdmissionFailure::Conflict(reason) => (StatusCode::CONFLICT, reason),
        AgentAdmissionFailure::Unavailable(reason) => (StatusCode::SERVICE_UNAVAILABLE, reason),
    };
    (status, Json(serde_json::json!({"error":reason}))).into_response()
}

async fn agent_remove_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    AxumPath(agent_id): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    if !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    match service.remove_agent(&agent_id) {
        Ok(status) => Json(serde_json::json!({"schema":AGENT_ADMISSION_SCHEMA,"status":status,"agent_id":agent_id})).into_response(),
        Err(error) => agent_failure_response(error),
    }
}

async fn agent_checkpoint_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    AxumPath(agent_id): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    if !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    match service.checkpoint_agent(&agent_id) {
        Ok(checkpoint) => Json(checkpoint).into_response(),
        Err(error) => agent_failure_response(error),
    }
}

async fn agent_dehydrate_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    AxumPath(agent_id): AxumPath<String>,
    headers: HeaderMap,
) -> Response {
    if !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    match service.dehydrate_agent(&agent_id) {
        Ok(bundle) => Json(bundle).into_response(),
        Err(error) => agent_failure_response(error),
    }
}

async fn agent_migration_commit_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    AxumPath(agent_id): AxumPath<String>,
    headers: HeaderMap,
    Json(request): Json<MigrationCommitRequest>,
) -> Response {
    if !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    match service.commit_agent_migration(&agent_id, &request.bundle_digest) {
        Ok(status) => Json(serde_json::json!({"schema":FREEZE_DRIED_AGENT_SCHEMA,"status":status,"agent_id":agent_id,"bundle_digest":request.bundle_digest})).into_response(),
        Err(error) => agent_failure_response(error),
    }
}

async fn agent_rehydrate_handler<C: LifecycleControl + 'static>(
    State(service): State<Arc<ControlService<C>>>,
    headers: HeaderMap,
    Json(bundle): Json<FreezeDriedAgent>,
) -> Response {
    if !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    match service.rehydrate_agent(bundle).await {
        Ok(response) => Json(response).into_response(),
        Err(error) => agent_failure_response(error),
    }
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
    Query(query): Query<ObservatoryFeedQuery>,
    headers: HeaderMap,
) -> Response {
    let allowed_origin = allowed_origin(&service, &headers);
    if headers.contains_key(header::ORIGIN) && allowed_origin.is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    let Ok(version) = ObservatoryFeedVersion::parse(query.schema.as_deref()) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    observatory_json(
        StatusCode::OK,
        service.observatory_feed_projection(version),
        allowed_origin,
    )
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct ObservatoryFeedQuery {
    schema: Option<String>,
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
    if headers.contains_key(header::AUTHORIZATION) && !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
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
    if headers.contains_key(header::AUTHORIZATION) && !agent_write_authorized(&service, &headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
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
    Query(query): Query<ObservatoryFeedQuery>,
    headers: HeaderMap,
) -> Response {
    if headers.contains_key(header::ORIGIN) && allowed_origin(&service, &headers).is_none() {
        return StatusCode::FORBIDDEN.into_response();
    }
    let Ok(version) = ObservatoryFeedVersion::parse(query.schema.as_deref()) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let api_policy = service.api_policy();
    ws.max_frame_size(api_policy.websocket_max_frame_bytes)
        .max_message_size(api_policy.websocket_max_frame_bytes)
        .on_upgrade(move |socket| observatory_ws_session(socket, service, version))
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryAgentInitiationIntent {
    schema: String,
    conversation_id: String,
    turn_id: String,
    sender_id: String,
    recipient_id: String,
    correlation_id: String,
    work_id: String,
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

#[derive(Clone, Debug, Serialize)]
struct ObservatoryConversationResult {
    schema: &'static str,
    status: &'static str,
    conversation_id: String,
    turn_id: String,
    recipient_id: String,
    correlation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_turn_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_work_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initiated_reply: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accepted_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turn_sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

struct ObservatoryConversationResultParts {
    status: &'static str,
    conversation_id: String,
    turn_id: String,
    recipient_id: String,
    correlation_id: String,
    reply: Option<String>,
    accepted_sequence: Option<u64>,
    turn_sequence: Option<u64>,
    error: Option<&'static str>,
    initiation: Option<AgentInitiationMetadata>,
}

impl ObservatoryConversationResult {
    fn from_parts(parts: ObservatoryConversationResultParts) -> Self {
        Self {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: parts.status,
            conversation_id: parts.conversation_id,
            turn_id: parts.turn_id,
            recipient_id: parts.recipient_id,
            correlation_id: parts.correlation_id,
            sender_id: parts
                .initiation
                .as_ref()
                .map(|metadata| metadata.sender_id.clone()),
            initiated_recipient_id: parts
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_recipient_id.clone()),
            initiated_conversation_id: parts
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_conversation_id.clone()),
            initiated_turn_id: parts
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_turn_id.clone()),
            initiated_correlation_id: parts
                .initiation
                .as_ref()
                .map(|metadata| metadata.initiated_correlation_id.clone()),
            initiated_work_id: parts.initiation.map(|metadata| metadata.initiated_work_id),
            initiated_reply: None,
            reply: parts.reply,
            accepted_sequence: parts.accepted_sequence,
            turn_sequence: parts.turn_sequence,
            error: parts.error,
        }
    }

    fn refused_cancel(cancel: &ObservatoryConversationCancel, error: &'static str) -> Self {
        Self {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: "refused",
            conversation_id: cancel.conversation_id.clone(),
            turn_id: cancel.turn_id.clone(),
            recipient_id: String::new(),
            correlation_id: cancel.correlation_id.clone(),
            sender_id: None,
            initiated_recipient_id: None,
            initiated_conversation_id: None,
            initiated_turn_id: None,
            initiated_correlation_id: None,
            initiated_work_id: None,
            initiated_reply: None,
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
    version: ObservatoryFeedVersion,
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
    let Ok(initial_feed) = serde_json::to_string(&service.observatory_feed_projection(version))
    else {
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
                let Ok(payload) = serde_json::to_string(&service.observatory_feed_projection(version)) else {
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
                    if let Ok(intent) = serde_json::from_str::<ObservatoryAgentInitiationIntent>(&payload) {
                        let result = if bearer_token.is_none() {
                            ConversationAcceptance::Response(
                                ObservatoryConversationResult::from_parts(ObservatoryConversationResultParts {
                                    status: "refused",
                                    conversation_id: intent.conversation_id.clone(),
                                    turn_id: intent.turn_id.clone(),
                                    recipient_id: intent.recipient_id.clone(),
                                    correlation_id: intent.correlation_id.clone(),
                                    reply: None,
                                    accepted_sequence: None,
                                    turn_sequence: None,
                                    error: Some("write_authentication_required"),
                                    initiation: Some(AgentInitiationMetadata {
                                        sender_id: intent.sender_id.clone(),
                                        initiated_recipient_id: intent.recipient_id.clone(),
                                        initiated_conversation_id: intent.conversation_id.clone(),
                                        initiated_turn_id: intent.turn_id.clone(),
                                        initiated_correlation_id: intent.correlation_id.clone(),
                                        initiated_work_id: intent.work_id.clone(),
                                    }),
                                }),
                            )
                        } else {
                            service.accept_agent_initiation_intent(&intent)
                        };
                        let (response, dispatch) = match result {
                            ConversationAcceptance::Dispatch { accepted, dispatch } => {
                                (*accepted, Some(dispatch))
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
                                .expect("authenticated agent initiation dispatch has a bearer token");
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
                                .expect("authenticated agent initiation replay has a bearer token");
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
                                sender_id: None,
                                initiated_recipient_id: None,
                                initiated_conversation_id: None,
                                initiated_turn_id: None,
                                initiated_correlation_id: None,
                                initiated_work_id: None,
                                initiated_reply: None,
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
                                (*accepted, Some(dispatch))
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
            );
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
            );
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
struct RuntimePresentationState {
    public_base_url: String,
    polis_identity: PolisIdentityFeed,
    observatory_allowed_origins: Arc<BTreeSet<String>>,
}

#[derive(Clone, Debug)]
pub struct ObservatoryOriginPolicy {
    state: Arc<RwLock<RuntimePresentationState>>,
}

impl ObservatoryOriginPolicy {
    pub fn new(origins: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let origins = Arc::new(validate_observatory_origins(origins, true)?);
        Ok(Self::from_state(Arc::new(RwLock::new(
            RuntimePresentationState {
                public_base_url: "https://localhost".to_owned(),
                polis_identity: PolisIdentityFeed::unavailable("local_runtime"),
                observatory_allowed_origins: origins,
            },
        ))))
    }

    fn from_state(state: Arc<RwLock<RuntimePresentationState>>) -> Self {
        Self { state }
    }

    pub fn replace(&self, origins: impl IntoIterator<Item = String>) -> Result<(), String> {
        let origins = Arc::new(validate_observatory_origins(origins, true)?);
        let mut active = self
            .state
            .write()
            .map_err(|_| "observatory_origin_policy_unavailable".to_owned())?;
        active.observatory_allowed_origins = origins;
        Ok(())
    }

    pub fn contains(&self, origin: &str) -> bool {
        self.state
            .read()
            .map(|state| state.observatory_allowed_origins.contains(origin))
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

fn shepherd_admission_is_fresh(snapshot: &RuntimeSnapshot, now_unix_millis: u64) -> bool {
    snapshot
        .agent_admissions
        .get("shepherd")
        .is_some_and(|admission| {
            admission.observed_at_unix_millis <= now_unix_millis
                && admission.freshness_deadline_unix_millis >= now_unix_millis
        })
}

#[cfg(test)]
mod shepherd_readiness_tests {
    use super::*;

    struct FakeLifecycle;

    #[async_trait]
    impl LifecycleControl for FakeLifecycle {
        async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
            Ok(KernelExit::Clean)
        }
    }

    struct FixedTrustedTime(u64);

    impl crate::TrustedTime for FixedTrustedTime {
        fn now_unix_millis(&self) -> u64 {
            self.0
        }
    }

    #[test]
    fn readiness_accepts_the_deadline_and_fails_closed_after_heartbeat_loss() {
        let recorder = RuntimeRecorder::new(4);

        assert!(recorder.record_agent_admission(
            "shepherd",
            1_000,
            31_000,
            "1111111111111111111111111111111111111111",
        ));
        assert!(shepherd_admission_is_fresh(&recorder.snapshot(), 31_000));
        assert!(!shepherd_admission_is_fresh(&recorder.snapshot(), 31_001));

        assert!(recorder.record_agent_heartbeat("shepherd", 2_000, 32_000));
        assert!(shepherd_admission_is_fresh(&recorder.snapshot(), 32_000));
        assert!(!shepherd_admission_is_fresh(&recorder.snapshot(), 32_001));
    }

    #[test]
    fn readiness_uses_the_same_trusted_clock_as_shepherd_admission() {
        let service = ControlService::new(
            "trusted-readiness-runtime",
            RuntimeRecorder::new(4),
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            4,
        )
        .with_readiness_time(Arc::new(FixedTrustedTime(12_345)));

        assert_eq!(service.readiness_now_unix_millis(), 12_345);
    }
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
    use crate::{AgentRosterPolicy, AuthorityMode, ComponentId, RunningState};

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
                name: format!("{id}.runtime"),
                label: label.to_owned(),
                role: "conversation agent".to_owned(),
                provider: None,
                model: None,
                inference_readiness: InferenceReadinessState::Ready,
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

    struct AgentInitiationExecutor {
        observed_tasks: Arc<Mutex<Vec<serde_json::Value>>>,
        fail: bool,
        delay: Duration,
    }

    #[async_trait]
    impl crate::OperationExecutor for AgentInitiationExecutor {
        async fn execute(
            &self,
            request: &crate::OperationRequest,
        ) -> Result<Vec<u8>, crate::ExecutorError> {
            if !self.delay.is_zero() {
                tokio::time::sleep(self.delay).await;
            }
            if self.fail {
                return Err(crate::ExecutorError {
                    class: crate::FailureClass::Retryable,
                    message: "provider failed".to_owned(),
                });
            }
            let work: serde_json::Value =
                serde_json::from_slice(&request.payload).map_err(|error| crate::ExecutorError {
                    class: crate::FailureClass::Fatal,
                    message: error.to_string(),
                })?;
            let task = work["tasks"][0].clone();
            self.observed_tasks
                .lock()
                .expect("observed task mutex poisoned")
                .push(task.clone());
            let recipient_id =
                task["recipient_id"]
                    .as_str()
                    .ok_or_else(|| crate::ExecutorError {
                        class: crate::FailureClass::Fatal,
                        message: "missing recipient".to_owned(),
                    })?;
            let output = if recipient_id == "beacon"
                && task.get("sender_id").is_none_or(serde_json::Value::is_null)
            {
                serde_json::json!({
                    "recipient_id": recipient_id,
                    "message": "Beacon is initiating governed contact with Ember.",
                    "agent_to_agent_initiation": {
                        "schema": crate::ingress::AGENT_TO_AGENT_INITIATION_REQUEST_SCHEMA,
                        "recipient_id": "ember",
                        "message": "Ember, please answer Beacon through the governed A2A path."
                    }
                })
            } else {
                serde_json::json!({
                    "recipient_id": recipient_id,
                    "message": format!(
                        "{} handled initiated work {} from {}",
                        recipient_id,
                        task["initiated_work_id"].as_str().unwrap_or("none"),
                        task["sender_id"].as_str().unwrap_or("none")
                    )
                })
            };
            serde_json::to_vec(&serde_json::json!({
                "schema": "adl.runtime.local_agent_execution.v1",
                "outputs": [{
                    "unit": 0,
                    "output": output
                }]
            }))
            .map_err(|error| crate::ExecutorError {
                class: crate::FailureClass::Fatal,
                message: error.to_string(),
            })
        }
    }

    fn agent_initiation_intent(turn_id: &str, work_id: &str) -> ObservatoryAgentInitiationIntent {
        ObservatoryAgentInitiationIntent {
            schema: OBSERVATORY_WS_AGENT_INITIATION_INTENT_SCHEMA.to_owned(),
            conversation_id: "conversation-beacon-ember".to_owned(),
            turn_id: turn_id.to_owned(),
            sender_id: "beacon".to_owned(),
            recipient_id: "ember".to_owned(),
            correlation_id: "abababababababababababababababab".to_owned(),
            work_id: work_id.to_owned(),
            message: "please summarize the governed state".to_owned(),
        }
    }

    fn agent_pair_initiation_intent(
        sender_id: &str,
        recipient_id: &str,
        turn_id: &str,
        correlation_id: &str,
        work_id: &str,
    ) -> ObservatoryAgentInitiationIntent {
        ObservatoryAgentInitiationIntent {
            schema: OBSERVATORY_WS_AGENT_INITIATION_INTENT_SCHEMA.to_owned(),
            conversation_id: format!("conversation-{sender_id}-{recipient_id}"),
            turn_id: turn_id.to_owned(),
            sender_id: sender_id.to_owned(),
            recipient_id: recipient_id.to_owned(),
            correlation_id: correlation_id.to_owned(),
            work_id: work_id.to_owned(),
            message: format!("{recipient_id}, please answer {sender_id} through governed A2A."),
        }
    }

    fn agent_initiation_layer8_fixture(
        sender_id: &str,
        recipient_id: &str,
    ) -> (
        Layer8ConversationAuthority,
        Layer8SignedExchange,
        tempfile::TempDir,
    ) {
        let temp_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".adl")
            .join("tmp");
        std::fs::create_dir_all(&temp_root).expect("create test temp root");
        let root = tempfile::tempdir_in(temp_root).expect("create agent initiation fixture");
        let sender_key = SigningKey::from_bytes(&[45; 32]);
        let recipient_key = SigningKey::from_bytes(&[46; 32]);
        let scribe_key = SigningKey::from_bytes(&[47; 32]);
        let sender_key_id = format!("{sender_id}-key");
        let recipient_key_id = format!("{recipient_id}-key");
        let sender_key_file = root.path().join(format!("{sender_id}.key"));
        let recipient_key_file = root.path().join(format!("{recipient_id}.key"));
        std::fs::write(&sender_key_file, hex::encode(sender_key.to_bytes()))
            .expect("write sender key");
        std::fs::write(&recipient_key_file, hex::encode(recipient_key.to_bytes()))
            .expect("write recipient key");
        let evidence = RuntimeIdentityEvidence {
            principal_id: sender_id.to_owned(),
            polis_id: "conversation-runtime".to_owned(),
            signing_key_id: sender_key_id.clone(),
            verifying_key_hex: hex::encode(sender_key.verifying_key().to_bytes()),
            credential_generation: 1,
            current_credential_generation: 1,
            expires_at_epoch_secs: u64::MAX,
            revoked: false,
            authenticated: true,
        };
        let scope = |action| AuthorityScope {
            polis_id: "conversation-runtime".to_owned(),
            action,
            conversation_id: None,
            recipients: BTreeSet::from([
                sender_id.to_owned(),
                recipient_id.to_owned(),
                "scribe".to_owned(),
            ]),
            attachment_id: None,
        };
        let contact_scope = scope(Layer8Action::Contact);
        let continue_scope = scope(Layer8Action::Continue);
        let capabilities = [
            ("agent-initiation-contact", contact_scope.clone()),
            ("agent-initiation-continue", continue_scope.clone()),
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
            ("agent-initiation-agent-contact", contact_scope.clone()),
            ("agent-initiation-agent-continue", continue_scope.clone()),
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
            ("agent-initiation-polis-contact", contact_scope),
            ("agent-initiation-polis-continue", continue_scope),
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
        .expect("agent initiation authority profile is valid");
        let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
            sender: CommunicationKeyDescriptor {
                principal_id: sender_id.to_owned(),
                polis_id: "conversation-runtime".to_owned(),
                signing_key_id: sender_key_id.clone(),
                credential_generation: 1,
                private_key_file: sender_key_file,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            },
            recipients: vec![
                CommunicationVerifyingDescriptor {
                    principal_id: sender_id.to_owned(),
                    polis_id: "conversation-runtime".to_owned(),
                    signing_key_id: sender_key_id.clone(),
                    credential_generation: 1,
                    verifying_key_hex: hex::encode(sender_key.verifying_key().to_bytes()),
                    revoked: false,
                    not_before_epoch_secs: 0,
                    expires_at_epoch_secs: u64::MAX,
                },
                CommunicationVerifyingDescriptor {
                    principal_id: recipient_id.to_owned(),
                    polis_id: "conversation-runtime".to_owned(),
                    signing_key_id: recipient_key_id,
                    credential_generation: 1,
                    verifying_key_hex: hex::encode(recipient_key.verifying_key().to_bytes()),
                    revoked: false,
                    not_before_epoch_secs: 0,
                    expires_at_epoch_secs: u64::MAX,
                },
                CommunicationVerifyingDescriptor {
                    principal_id: "scribe".to_owned(),
                    polis_id: "conversation-runtime".to_owned(),
                    signing_key_id: "scribe-key".to_owned(),
                    credential_generation: 1,
                    verifying_key_hex: hex::encode(scribe_key.verifying_key().to_bytes()),
                    revoked: false,
                    not_before_epoch_secs: 0,
                    expires_at_epoch_secs: u64::MAX,
                },
            ],
        })
        .expect("agent initiation exchange profile is valid");
        (authority, exchange, root)
    }

    async fn agent_initiation_service(
        fail: bool,
        delay: Duration,
    ) -> (
        Arc<ControlService<FakeLifecycle>>,
        crate::KernelHandle,
        RuntimeRecorder,
        Arc<Mutex<Vec<serde_json::Value>>>,
        tempfile::TempDir,
    ) {
        let recorder = RuntimeRecorder::new(16);
        let now = now_unix_millis();
        let mut population = AgentPopulationFeed::empty();
        for (id, label, provider, model) in [
            ("beacon", "Beacon Axioma", None, None),
            ("scribe", "Scribe Axioma", None, None),
            (
                "ember",
                "Ember Axioma",
                Some("ollama".to_owned()),
                Some("gemma3-local".to_owned()),
            ),
        ] {
            recorder.set_component_state(ComponentId::new(id), RunningState::Running);
            assert!(recorder.record_agent_admission(
                id,
                now,
                now + 30_000,
                "1111111111111111111111111111111111111111",
            ));
            let inference_readiness = InferenceReadinessState::Ready;
            let projection = inference_readiness.projection();
            population.sample.push(AgentSample {
                id: id.to_owned(),
                name: format!("{id}.runtime"),
                label: label.to_owned(),
                role: "resident agent".to_owned(),
                provider,
                model,
                inference_readiness,
                state: "ready".to_owned(),
                detail: "Configured test provider ready for deterministic A2A work".to_owned(),
                health: projection.health.to_owned(),
                availability: projection.availability.to_owned(),
                activity: projection.activity.map(str::to_owned),
                capabilities: vec!["conversation".to_owned()],
                location: Some("local_runtime".to_owned()),
                communication_eligible: projection.communication_eligible,
                observed_at_unix_millis: 0,
                freshness_deadline_unix_millis: 0,
                source_revision: "unobserved".to_owned(),
                provenance: "runtime_component_state".to_owned(),
            });
        }
        population = population.with_public_policy(AgentRosterPolicy {
            policy_subject: "agent-initiation-test".to_owned(),
            visible_agent_ids: BTreeSet::from([
                "beacon".to_owned(),
                "ember".to_owned(),
                "scribe".to_owned(),
            ]),
            reveal_capabilities: false,
            reveal_location: false,
        });
        let (authority, exchange, layer8_root) = agent_initiation_layer8_fixture("beacon", "ember");
        let observed_tasks = Arc::new(Mutex::new(Vec::new()));
        let adapter = Arc::new(
            crate::OperationalAdapter::new(
                crate::AdapterKind::Agent,
                crate::AdapterPolicy {
                    capacity: 4,
                    max_in_flight: 2,
                    shutdown_grace_millis: 1_000,
                    max_attempts: 1,
                    idempotency_entries: 16,
                    authority: AuthorityMode::Internal,
                },
                Arc::new(AgentInitiationExecutor {
                    observed_tasks: observed_tasks.clone(),
                    fail,
                    delay,
                }),
            )
            .expect("agent adapter"),
        );
        let operation = crate::OperationalFactory::new(adapter, vec![]);
        let ingress = CanonicalIngress::new(
            4,
            recorder.clone(),
            BTreeMap::from([("agent_runtime".to_owned(), operation.clone())]),
        );
        let service = Arc::new(
            ControlService::new_with_observatory_config_and_agents(
                "conversation-runtime",
                recorder.clone(),
                FakeLifecycle,
                ControlAuthority::new(BTreeMap::new()),
                8,
                ["https://observatory.example.test".to_owned()],
                population,
            )
            .with_canonical_ingress(ingress.clone())
            .with_layer8_authority(authority)
            .with_layer8_signed_exchange(exchange),
        );
        service
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .push(AgentAdmissionRequest {
                schema: AGENT_ADMISSION_SCHEMA.to_owned(),
                id: "ember".to_owned(),
                name: "ember.runtime".to_owned(),
                display_name: "Ember Axioma".to_owned(),
                office: "resident agent".to_owned(),
                role: "resident agent".to_owned(),
                provider: "ollama".to_owned(),
                model: "gemma3-local".to_owned(),
                endpoint: "http://127.0.0.1:11434".to_owned(),
            });
        service
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .push(AgentAdmissionRequest {
                schema: AGENT_ADMISSION_SCHEMA.to_owned(),
                id: "scribe".to_owned(),
                name: "scribe.runtime".to_owned(),
                display_name: "Scribe Axioma".to_owned(),
                office: "resident agent".to_owned(),
                role: "resident agent".to_owned(),
                provider: "ollama".to_owned(),
                model: "gemma3-local".to_owned(),
                endpoint: "http://127.0.0.1:11434".to_owned(),
            });
        service
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .push(AgentAdmissionRequest {
                schema: AGENT_ADMISSION_SCHEMA.to_owned(),
                id: "beacon".to_owned(),
                name: "beacon.runtime".to_owned(),
                display_name: "Beacon Axioma".to_owned(),
                office: "resident agent".to_owned(),
                role: "resident agent".to_owned(),
                provider: "ollama".to_owned(),
                model: "gemma3-local".to_owned(),
                endpoint: "http://127.0.0.1:11434".to_owned(),
            });
        let mut registry = crate::ComponentRegistry::new();
        registry.register(operation);
        registry.register(ingress);
        let kernel = crate::Kernel::new(
            registry.validate().expect("valid registry"),
            recorder.clone(),
        )
        .start()
        .await
        .expect("kernel starts");
        (service, kernel, recorder, observed_tasks, layer8_root)
    }

    async fn live_style_ollama_a2a_provider() -> (
        String,
        Arc<Mutex<Vec<serde_json::Value>>>,
        tokio::task::JoinHandle<()>,
    ) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind isolated live-style Ollama fixture");
        let address = listener.local_addr().expect("fixture address");
        let observed = Arc::new(Mutex::new(Vec::new()));
        let requests = observed.clone();
        let task = tokio::spawn(async move {
            loop {
                let Ok((mut socket, _)) = listener.accept().await else {
                    return;
                };
                let mut request = Vec::with_capacity(8_192);
                while request.len() < 65_536
                    && !request.windows(4).any(|window| window == b"\r\n\r\n")
                {
                    let mut chunk = [0_u8; 2_048];
                    let read = socket.read(&mut chunk).await.unwrap_or_default();
                    if read == 0 {
                        break;
                    }
                    request.extend_from_slice(&chunk[..read]);
                }
                let header_end = request
                    .windows(4)
                    .position(|window| window == b"\r\n\r\n")
                    .map(|index| index + 4)
                    .unwrap_or(request.len());
                let content_length = String::from_utf8_lossy(&request[..header_end])
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or_default();
                while request.len() < header_end.saturating_add(content_length) {
                    let mut chunk = [0_u8; 2_048];
                    let read = socket.read(&mut chunk).await.unwrap_or_default();
                    if read == 0 {
                        break;
                    }
                    request.extend_from_slice(&chunk[..read]);
                }
                let request_line = String::from_utf8_lossy(&request[..header_end])
                    .lines()
                    .next()
                    .unwrap_or_default()
                    .to_owned();
                let body = serde_json::from_slice::<serde_json::Value>(
                    request
                        .get(header_end..header_end.saturating_add(content_length))
                        .unwrap_or_default(),
                )
                .unwrap_or(serde_json::Value::Null);
                requests
                    .lock()
                    .expect("fixture requests poisoned")
                    .push(serde_json::json!({"request_line": request_line, "body": body}));
                let response_body = if request_line.starts_with("POST /api/chat ") {
                    serde_json::json!({
                        "model": "beacon-model",
                        "message": {
                            "role": "assistant",
                            "content": "I can ask Ember through the governed action channel.",
                            "tool_calls": [{
                                "function": {
                                    "name": "initiate_agent",
                                    "arguments": {
                                        "recipient_id": "ember",
                                        "message": "Ember, please answer Beacon through governed A2A."
                                    }
                                }
                            }]
                        },
                        "done": true
                    })
                } else if request_line.starts_with("POST /api/generate ") {
                    serde_json::json!({
                        "model": "ember-model",
                        "response": "Ember generated a governed response for Beacon.",
                        "done": true
                    })
                } else {
                    serde_json::json!({"error": "unexpected fixture route"})
                };
                let encoded = serde_json::to_vec(&response_body).expect("encode fixture response");
                let headers = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    encoded.len()
                );
                let _ = socket.write_all(headers.as_bytes()).await;
                let _ = socket.write_all(&encoded).await;
            }
        });
        (format!("http://{address}"), observed, task)
    }

    #[tokio::test]
    async fn agent_to_agent_initiation_delivers_configured_provider_work_and_activity() {
        let (service, kernel, recorder, observed_tasks, _layer8_root) =
            agent_initiation_service(false, Duration::ZERO).await;
        let accepted = match service
            .accept_agent_initiation_intent(&agent_initiation_intent("turn-a2a", "a2a-work-001"))
        {
            ConversationAcceptance::Dispatch { accepted, dispatch } => {
                assert_eq!(dispatch.work_id, "a2a-work-001");
                let delivered = service.complete_conversation_dispatch(dispatch).await;
                assert_eq!(delivered.status, "delivered");
                assert_eq!(delivered.sender_id.as_deref(), Some("beacon"));
                assert_eq!(delivered.recipient_id, "ember");
                assert_eq!(delivered.initiated_work_id.as_deref(), Some("a2a-work-001"));
                assert!(
                    delivered
                        .reply
                        .as_deref()
                        .is_some_and(|reply| reply.contains("Ember") || reply.contains("ember")),
                    "recipient reply should be projected from executed work: {delivered:?}"
                );
                *accepted
            }
            ConversationAcceptance::Response(response) => {
                panic!("agent initiation refused: {:?}", response.error)
            }
        };
        assert_eq!(accepted.status, "accepted");
        assert_eq!(accepted.sender_id.as_deref(), Some("beacon"));
        assert_eq!(accepted.initiated_work_id.as_deref(), Some("a2a-work-001"));
        {
            let tasks = observed_tasks.lock().expect("observed task mutex poisoned");
            assert_eq!(tasks.len(), 1);
            assert_eq!(tasks[0]["sender_id"], "beacon");
            assert_eq!(tasks[0]["recipient_id"], "ember");
            assert_eq!(tasks[0]["initiated_work_id"], "a2a-work-001");
            assert_eq!(tasks[0]["provider"], "ollama");
            assert_eq!(tasks[0]["model"], "gemma3-local");
            assert_eq!(tasks[0]["endpoint"], "http://127.0.0.1:11434");
        }
        let events = recorder.events();
        assert!(
            events.iter().any(|event| {
                event.event == "agent_to_agent_initiated"
                    && event
                        .component
                        .as_ref()
                        .is_some_and(|component| component.as_str() == "agent_initiation")
                    && event.correlation_id.as_deref() == Some("abababababababababababababababab")
            }),
            "Observatory feed should expose authoritative correlated initiation activity: {events:?}"
        );
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();
    }

    #[tokio::test]
    async fn agent_to_agent_model_action_from_conversation_delivers_peer_response() {
        let (mut service, old_kernel, recorder, _observed_tasks, _layer8_root) =
            agent_initiation_service(false, Duration::ZERO).await;
        old_kernel.shutdown(Duration::from_secs(1)).await.unwrap();
        let (endpoint, provider_requests, provider_task) = live_style_ollama_a2a_provider().await;
        for agent in service
            .dynamic_agents
            .lock()
            .expect("dynamic agents state poisoned")
            .iter_mut()
        {
            agent.endpoint = endpoint.clone();
            agent.model = if agent.id == "beacon" {
                "beacon-model".to_owned()
            } else {
                "ember-model".to_owned()
            };
        }
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/tmp");
        std::fs::create_dir_all(&fixture_root).expect("create repository-local fixture root");
        let state = tempfile::tempdir_in(fixture_root).expect("create production executor state");
        let agent_executor = Arc::new(crate::assembly::InProcessOperationExecutor::with_state_dir(
            crate::AdapterKind::Agent,
            state.path(),
        ));
        let adapter = Arc::new(
            crate::OperationalAdapter::new(
                crate::AdapterKind::Agent,
                crate::AdapterPolicy {
                    capacity: 4,
                    max_in_flight: 2,
                    shutdown_grace_millis: 1_000,
                    max_attempts: 1,
                    idempotency_entries: 16,
                    authority: AuthorityMode::Internal,
                },
                agent_executor,
            )
            .expect("production agent adapter"),
        );
        let operation = crate::OperationalFactory::new(adapter, vec![]);
        let ingress = CanonicalIngress::new(
            4,
            recorder.clone(),
            BTreeMap::from([("agent_runtime".to_owned(), operation.clone())]),
        );
        Arc::get_mut(&mut service)
            .expect("service has no other owners")
            .canonical_ingress = Some(ingress.clone());
        let mut registry = crate::ComponentRegistry::new();
        registry.register(operation);
        registry.register(ingress);
        let kernel = crate::Kernel::new(
            registry
                .validate()
                .expect("valid production-ingress registry"),
            recorder.clone(),
        )
        .start()
        .await
        .expect("production-ingress kernel starts");
        let intent = ObservatoryConversationIntent {
            schema: OBSERVATORY_WS_CONVERSATION_INTENT_SCHEMA.to_owned(),
            conversation_id: "conversation-operator-beacon".to_owned(),
            turn_id: "turn-operator-asks-beacon".to_owned(),
            recipient_id: "beacon".to_owned(),
            correlation_id: "efefefefefefefefefefefefefefefef".to_owned(),
            message: "Please ask Ember for a governed response.".to_owned(),
        };
        let delivered = match service.accept_conversation_intent(&intent) {
            ConversationAcceptance::Dispatch { dispatch, .. } => {
                service.complete_conversation_dispatch(dispatch).await
            }
            ConversationAcceptance::Response(response) => {
                panic!(
                    "conversation refused before Beacon could act: {:?}",
                    response.error
                )
            }
        };
        assert_eq!(delivered.status, "delivered");
        assert_eq!(delivered.recipient_id, "beacon");
        assert_eq!(delivered.sender_id.as_deref(), Some("beacon"));
        assert_eq!(delivered.initiated_recipient_id.as_deref(), Some("ember"));
        assert!(
            delivered
                .initiated_conversation_id
                .as_deref()
                .is_some_and(|value| {
                    value.starts_with("a2a-beacon-ember-") && is_safe_identifier(value)
                }),
            "operator-visible result should structurally identify the peer conversation: {delivered:?}"
        );
        assert!(
            delivered
                .initiated_turn_id
                .as_deref()
                .is_some_and(|value| value.starts_with("turn-a2a-") && is_safe_identifier(value)),
            "operator-visible result should structurally identify the peer turn: {delivered:?}"
        );
        assert!(
            delivered
                .initiated_correlation_id
                .as_deref()
                .is_some_and(is_correlation_id),
            "operator-visible result should structurally expose the governed peer correlation id: {delivered:?}"
        );
        assert!(
            delivered
                .initiated_work_id
                .as_deref()
                .is_some_and(|value| value.starts_with("a2a-work-") && is_safe_identifier(value)),
            "operator-visible result should structurally expose the governed peer work id: {delivered:?}"
        );
        assert_eq!(
            delivered.reply.as_deref(),
            Some("I can ask Ember through the governed action channel."),
            "Beacon's operator reply must remain distinct from Ember's governed result"
        );
        assert_eq!(
            delivered.initiated_reply.as_deref(),
            Some("Ember generated a governed response for Beacon."),
            "the authoritative result must expose Ember's distinct correlated reply"
        );
        {
            let requests = provider_requests
                .lock()
                .expect("provider request fixture poisoned");
            assert_eq!(
                requests.len(),
                2,
                "initiator and recipient must both execute"
            );
            assert_eq!(requests[0]["request_line"], "POST /api/chat HTTP/1.1");
            assert_eq!(
                requests[0]["body"]["tools"][0]["function"]["name"],
                "initiate_agent"
            );
            assert!(requests[0]["body"]["messages"][0]["content"]
                .as_str()
                .is_some_and(|prompt| {
                    prompt.contains("provided `initiate_agent` tool")
                        && !prompt.contains("adl.runtime.agent_conversation_response.v1")
                }));
            assert_eq!(requests[1]["request_line"], "POST /api/generate HTTP/1.1");
            assert_eq!(requests[1]["body"]["model"], "ember-model");
            assert!(requests[1]["body"]["prompt"]
                .as_str()
                .is_some_and(|prompt| prompt.contains("Ember, please answer Beacon")));
        }
        let events = recorder.events();
        assert!(
            events.iter().any(|event| {
                event.event == "agent_to_agent_initiated"
                    && event.correlation_id.as_deref()
                        == delivered.initiated_correlation_id.as_deref()
            }),
            "A2A model action should emit correlated initiation activity: {events:?}"
        );
        assert!(
            events.iter().any(|event| {
                event.event == "agent_to_agent_completed"
                    && event.correlation_id.as_deref()
                        == delivered.initiated_correlation_id.as_deref()
            }),
            "recipient completion must be separately observable and correlated: {events:?}"
        );
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();
        provider_task.abort();
    }

    #[tokio::test]
    async fn agent_to_agent_runtime_internal_initiation_allows_resident_agent_pairs() {
        let (service, kernel, _recorder, observed_tasks, _layer8_root) =
            agent_initiation_service(false, Duration::ZERO).await;
        let resident_ids = ["beacon", "ember", "scribe"];
        let pairs = resident_ids
            .iter()
            .flat_map(|sender_id| {
                resident_ids
                    .iter()
                    .filter(move |recipient_id| recipient_id != &sender_id)
                    .map(move |recipient_id| (*sender_id, *recipient_id))
            })
            .enumerate()
            .map(|(index, (sender_id, recipient_id))| {
                (
                    sender_id,
                    recipient_id,
                    format!("turn-{sender_id}-{recipient_id}"),
                    format!("{:032x}", index + 1),
                    format!("a2a-work-{sender_id}-{recipient_id}"),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            pairs.len(),
            resident_ids.len() * (resident_ids.len() - 1),
            "runtime-internal A2A test should cover every ordered resident pair"
        );
        for (sender_id, recipient_id, turn_id, correlation_id, work_id) in &pairs {
            let sender_id = *sender_id;
            let recipient_id = *recipient_id;
            let turn_id = turn_id.as_str();
            let correlation_id = correlation_id.as_str();
            let work_id = work_id.as_str();
            let accepted =
                match service.accept_runtime_agent_initiation_intent(&agent_pair_initiation_intent(
                    sender_id,
                    recipient_id,
                    turn_id,
                    correlation_id,
                    work_id,
                )) {
                    ConversationAcceptance::Dispatch { accepted, dispatch } => {
                        assert_eq!(accepted.status, "accepted");
                        assert_eq!(accepted.sender_id.as_deref(), Some(sender_id));
                        assert_eq!(
                            accepted.initiated_recipient_id.as_deref(),
                            Some(recipient_id)
                        );
                        assert_eq!(accepted.initiated_work_id.as_deref(), Some(work_id));
                        let delivered = service.complete_conversation_dispatch(dispatch).await;
                        assert_eq!(delivered.status, "delivered");
                        assert_eq!(delivered.sender_id.as_deref(), Some(sender_id));
                        assert_eq!(delivered.recipient_id, recipient_id);
                        assert_eq!(
                            delivered.initiated_recipient_id.as_deref(),
                            Some(recipient_id)
                        );
                        assert_eq!(delivered.initiated_work_id.as_deref(), Some(work_id));
                        assert_eq!(
                            delivered.initiated_correlation_id.as_deref(),
                            Some(correlation_id)
                        );
                        delivered
                    }
                    ConversationAcceptance::Response(response) => {
                        panic!(
                            "runtime-internal {sender_id}->{recipient_id} A2A refused: {:?}",
                            response.error
                        )
                    }
                };
            assert!(
                accepted.reply.as_deref().is_some_and(|reply| {
                    reply.contains(recipient_id)
                        && reply.contains(work_id)
                        && reply.contains(sender_id)
                }),
                "resident pair reply should preserve governed sender and work identity: {accepted:?}"
            );
        }
        {
            let tasks = observed_tasks.lock().expect("observed task mutex poisoned");
            for (sender_id, recipient_id, _turn_id, correlation_id, work_id) in pairs {
                assert!(
                    tasks.iter().any(|task| {
                        task["sender_id"] == sender_id
                            && task["recipient_id"] == recipient_id
                            && task["correlation_id"] == correlation_id
                            && task["initiated_work_id"] == work_id
                    }),
                    "expected governed task for resident pair {sender_id}->{recipient_id}: {tasks:?}"
                );
            }
        }
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();
    }

    #[tokio::test]
    async fn agent_to_agent_initiation_replay_and_conflict_are_explicit() {
        let (service, kernel, _recorder, observed_tasks, _layer8_root) =
            agent_initiation_service(false, Duration::ZERO).await;
        let intent = agent_initiation_intent("turn-a2a-replay", "a2a-work-replay");
        let dispatch = match service.accept_agent_initiation_intent(&intent) {
            ConversationAcceptance::Dispatch { dispatch, .. } => dispatch,
            ConversationAcceptance::Response(response) => {
                panic!("agent initiation refused: {:?}", response.error)
            }
        };
        let delivered = service.complete_conversation_dispatch(dispatch).await;
        assert_eq!(delivered.status, "delivered");

        let replay = match service.accept_agent_initiation_intent(&intent) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("exact replay dispatched again"),
        };
        assert_eq!(replay.status, "delivered");
        assert_eq!(replay.initiated_work_id.as_deref(), Some("a2a-work-replay"));
        assert_eq!(observed_tasks.lock().unwrap().len(), 1);

        let mut conflict = intent;
        conflict.work_id = "a2a-work-conflict".to_owned();
        let conflict = match service.accept_agent_initiation_intent(&conflict) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("conflicting replay dispatched"),
        };
        assert_eq!(conflict.status, "refused");
        assert_eq!(conflict.error, Some("conversation_conflict"));
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();
    }

    #[tokio::test]
    async fn agent_to_agent_initiation_terminal_failures_are_truthful() {
        let (service, kernel, recorder, _observed_tasks, _layer8_root) =
            agent_initiation_service(false, Duration::from_millis(75)).await;
        let mut unauthorized = agent_initiation_intent("turn-a2a-unauthorized", "a2a-work-unauth");
        unauthorized.sender_id = "unknown-beacon".to_owned();
        let refused = match service.accept_agent_initiation_intent(&unauthorized) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("unauthorized sender dispatched"),
        };
        assert_eq!(refused.status, "refused");
        assert_eq!(refused.error, Some("unauthorized_initiation"));

        let mut forged_sender = agent_initiation_intent("turn-a2a-forged", "a2a-work-forged");
        forged_sender.sender_id = "scribe".to_owned();
        let refused = match service.accept_agent_initiation_intent(&forged_sender) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("forged sender dispatched"),
        };
        assert_eq!(refused.status, "refused");
        assert_eq!(refused.error, Some("sender_identity_mismatch"));

        let mut missing_recipient = agent_initiation_intent("turn-a2a-missing", "a2a-work-missing");
        missing_recipient.recipient_id = "missing-ember".to_owned();
        let missing = match service.accept_agent_initiation_intent(&missing_recipient) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("missing recipient dispatched"),
        };
        assert_eq!(missing.status, "refused");
        assert_eq!(missing.error, Some("unknown_recipient"));

        service
            .recorder
            .set_component_state(ComponentId::new("ember"), RunningState::Degraded);
        let stale = match service.accept_agent_initiation_intent(&agent_initiation_intent(
            "turn-a2a-stale",
            "a2a-work-stale",
        )) {
            ConversationAcceptance::Response(response) => response,
            ConversationAcceptance::Dispatch { .. } => panic!("stale recipient dispatched"),
        };
        assert_eq!(stale.status, "refused");
        assert_eq!(stale.error, Some("recipient_unavailable"));
        service
            .recorder
            .set_component_state(ComponentId::new("ember"), RunningState::Running);

        let dispatch = match service.accept_agent_initiation_intent(&agent_initiation_intent(
            "turn-a2a-cancel",
            "a2a-work-cancel",
        )) {
            ConversationAcceptance::Dispatch { dispatch, .. } => dispatch,
            ConversationAcceptance::Response(response) => {
                panic!("agent initiation refused: {:?}", response.error)
            }
        };
        let cancel = ObservatoryConversationCancel {
            schema: OBSERVATORY_WS_CONVERSATION_CANCEL_SCHEMA.to_owned(),
            conversation_id: "conversation-beacon-ember".to_owned(),
            turn_id: "turn-a2a-cancel".to_owned(),
            correlation_id: "abababababababababababababababab".to_owned(),
        };
        let accepted_cancel = service.cancel_conversation_turn(&cancel);
        assert_eq!(accepted_cancel.status, "accepted");
        let cancelled = service.complete_conversation_dispatch(dispatch).await;
        assert_eq!(cancelled.status, "cancelled");
        assert_eq!(cancelled.error, Some("conversation_cancelled"));
        assert!(recorder.events().iter().any(|event| {
            event.event == "agent_to_agent_failed"
                && event.correlation_id.as_deref() == Some("abababababababababababababababab")
        }));
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();

        let (service, kernel, _recorder, _observed_tasks, _layer8_root) =
            agent_initiation_service(true, Duration::ZERO).await;
        let dispatch = match service.accept_agent_initiation_intent(&agent_initiation_intent(
            "turn-a2a-provider-fail",
            "a2a-work-provider-fail",
        )) {
            ConversationAcceptance::Dispatch { dispatch, .. } => dispatch,
            ConversationAcceptance::Response(response) => {
                panic!("agent initiation refused: {:?}", response.error)
            }
        };
        let failed = service.complete_conversation_dispatch(dispatch).await;
        assert_eq!(failed.status, "failed");
        assert_eq!(failed.error, Some("conversation_failed"));
        kernel.shutdown(Duration::from_secs(1)).await.unwrap();
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
            ConversationAcceptance::Dispatch { accepted, .. } => *accepted,
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
            ConversationAcceptance::Dispatch { accepted, .. } => *accepted,
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

#[cfg(test)]
mod agent_lifecycle {
    use super::*;
    use crate::{ComponentId, RunningState};

    struct FakeLifecycle;

    #[async_trait]
    impl LifecycleControl for FakeLifecycle {
        async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
            Ok(KernelExit::Clean)
        }
    }

    fn service(store: PathBuf) -> ControlService<FakeLifecycle> {
        let service = ControlService::new_with_observatory_config_and_agents(
            "runtime-test",
            RuntimeRecorder::new(16),
            FakeLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            16,
            std::iter::empty(),
            AgentPopulationFeed::resident_shepherd(),
        );
        service
            .configure_dynamic_agent_store(store)
            .expect("configure dynamic store");
        service
    }

    async fn ollama() -> (String, tokio::task::JoinHandle<()>) {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock Ollama");
        let address = listener.local_addr().expect("mock address");
        let task = tokio::spawn(async move {
            loop {
                let Ok((mut socket, _)) = listener.accept().await else {
                    return;
                };
                let mut request = Vec::with_capacity(4096);
                while request.len() < 4096
                    && !request.windows(4).any(|window| window == b"\r\n\r\n")
                {
                    let mut chunk = [0_u8; 1024];
                    let read = socket.read(&mut chunk).await.unwrap_or_default();
                    if read == 0 {
                        break;
                    }
                    request.extend_from_slice(&chunk[..read]);
                }
                let header_end = request
                    .windows(4)
                    .position(|window| window == b"\r\n\r\n")
                    .map(|index| index + 4)
                    .unwrap_or(request.len());
                let content_length = String::from_utf8_lossy(&request[..header_end])
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                    .unwrap_or_default();
                while request.len() < header_end.saturating_add(content_length) {
                    let mut chunk = [0_u8; 1024];
                    let read = socket.read(&mut chunk).await.unwrap_or_default();
                    if read == 0 {
                        break;
                    }
                    request.extend_from_slice(&chunk[..read]);
                }
                let is_generate = request.starts_with(b"POST /api/generate ");
                let body: &[u8] = if is_generate {
                    br#"{"response":"model reply"}"#
                } else {
                    br#"{"models":[{"name":"gemma4:e4b-mlx","model":"gemma4:e4b-mlx"}]}"#
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.write_all(body).await;
                let _ = socket.write_all(b"\r\n0\r\n\r\n").await;
            }
        });
        (format!("http://{address}"), task)
    }

    fn declaration(endpoint: String) -> AgentAdmissionRequest {
        AgentAdmissionRequest {
            schema: AGENT_ADMISSION_SCHEMA.to_owned(),
            id: "gemma-e4b".to_owned(),
            name: "ember.axioma".to_owned(),
            display_name: "Ember Axioma".to_owned(),
            office: "local assistant".to_owned(),
            role: String::new(),
            provider: "ollama".to_owned(),
            model: "gemma4:e4b-mlx".to_owned(),
            endpoint,
        }
    }

    fn vertex_declaration() -> AgentAdmissionRequest {
        AgentAdmissionRequest {
            schema: AGENT_ADMISSION_SCHEMA.to_owned(),
            id: "gemini-flash".to_owned(),
            name: "ember.axioma".to_owned(),
            display_name: "Ember Axioma".to_owned(),
            office: "local assistant".to_owned(),
            role: String::new(),
            provider: "vertex_ai".to_owned(),
            model: "gemini-2.5-flash".to_owned(),
            endpoint: "https://us-central1-aiplatform.googleapis.com/v1/projects/agent-logic-dev/locations/us-central1/publishers/google/models/gemini-2.5-flash:generateContent".to_owned(),
        }
    }

    #[test]
    fn vertex_ai_agent_admission_uses_explicit_provider_route() {
        let request = vertex_declaration();
        assert!(validate_agent_admission(&request).is_ok());

        let mut ambient = request.clone();
        ambient.endpoint =
            "https://aiplatform.googleapis.com/v1/models/gemini-2.5-flash".to_owned();
        assert!(validate_agent_admission(&ambient).is_err());

        let mut mismatched_model = request.clone();
        mismatched_model.model = "gemini-2.5-pro".to_owned();
        assert!(validate_agent_admission(&mismatched_model).is_err());

        let mut credentialish = request.clone();
        credentialish.endpoint = format!("{}?access_token=secret", credentialish.endpoint);
        assert!(validate_agent_admission(&credentialish).is_err());
    }

    #[tokio::test]
    async fn vertex_ai_provider_invocation_fails_closed_before_live_paid_call() {
        let request = vertex_declaration();
        assert_eq!(
            invoke_provider_model(
                &request.provider,
                &request.endpoint,
                &request.model,
                "hello",
                &CancellationToken::new(),
            )
            .await
            .unwrap_err(),
            "agent_provider_live_call_deferred"
        );
    }

    #[tokio::test]
    async fn agent_lifecycle_is_idempotent_portable_and_restart_safe() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join(".adl/tmp");
        fs::create_dir_all(&fixture_root).expect("create repository-local fixture root");
        let root = tempfile::tempdir_in(fixture_root).expect("temp root");
        let source_path = root.path().join("source/dynamic-agent-admissions.json");
        let destination_path = root
            .path()
            .join("destination/dynamic-agent-admissions.json");
        let (endpoint, ollama_task) = ollama().await;
        let legacy_path = root.path().join("legacy/dynamic-agent-admissions.json");
        fs::create_dir_all(legacy_path.parent().unwrap()).unwrap();
        fs::write(
            &legacy_path,
            serde_json::to_vec(&serde_json::json!({
                "schema": DYNAMIC_AGENT_STORE_SCHEMA,
                "agents": [{
                    "schema": AGENT_ADMISSION_SCHEMA,
                    "id": "legacy-gemma",
                    "name": "Gemma",
                    "role": "legacy persisted assistant",
                    "provider": "ollama",
                    "model": "gemma4:e4b-mlx",
                    "endpoint": endpoint.clone(),
                }]
            }))
            .unwrap(),
        )
        .unwrap();
        let legacy_service = service(legacy_path);
        let legacy_sample = legacy_service
            .agent_population
            .read()
            .unwrap()
            .sample
            .iter()
            .find(|sample| sample.id == "legacy-gemma")
            .cloned()
            .unwrap();
        assert_eq!(legacy_sample.label, "Gemma");
        assert_eq!(legacy_sample.role, "legacy persisted assistant");
        assert_eq!(legacy_sample.name, "legacy-gemma.legacy");
        let observed_at = now_unix_millis();
        legacy_service
            .recorder
            .set_component_state(ComponentId::new("legacy-gemma"), RunningState::Running);
        assert!(legacy_service.recorder.record_agent_admission(
            "legacy-gemma",
            observed_at,
            observed_at.saturating_add(30_000),
            "1111111111111111111111111111111111111111",
        ));
        let legacy_page = legacy_service
            .agent_roster_page(10, None, None, None)
            .expect("legacy agent remains listable");
        assert_eq!(legacy_page.sample[0].name, "legacy-gemma.legacy");
        let legacy_detail = legacy_service
            .agent_roster_detail("legacy-gemma")
            .expect("legacy agent remains addressable");
        assert_eq!(legacy_detail.name, "legacy-gemma.legacy");
        let source = service(source_path.clone());
        let request = declaration(endpoint);

        for invalid_name in ["ember.axioma.local", "ember.axioma-", "Gemma.local"] {
            let mut invalid = request.clone();
            invalid.name = invalid_name.to_owned();
            assert!(source.admit_agent(invalid).await.is_err(), "{invalid_name}");
        }
        let mut legacy = request.clone();
        legacy.name = "Gemma".to_owned();
        legacy.display_name.clear();
        legacy.office.clear();
        legacy.role = "legacy persisted assistant".to_owned();
        assert!(validate_persisted_agent_admission(&legacy).is_ok());
        assert!(validate_agent_admission(&legacy).is_err());
        assert!(validate_persisted_agent_admission(&request).is_ok());
        let mut conflicting = legacy.clone();
        conflicting.office = "current office".to_owned();
        assert!(validate_persisted_agent_admission(&conflicting).is_err());
        let mut malformed_current = request.clone();
        malformed_current.name = "Gemma".to_owned();
        assert!(validate_persisted_agent_admission(&malformed_current).is_err());

        assert_eq!(
            source.admit_agent(request.clone()).await.unwrap().status,
            "admitted"
        );
        assert_eq!(
            source.admit_agent(request.clone()).await.unwrap().status,
            "already_present"
        );
        let terminal = ObservatoryConversationResult {
            schema: OBSERVATORY_WS_CONVERSATION_RESULT_SCHEMA,
            status: "delivered",
            conversation_id: "conversation-1".to_owned(),
            turn_id: "turn-1".to_owned(),
            recipient_id: "gemma-e4b".to_owned(),
            correlation_id: "correlation-1".to_owned(),
            sender_id: None,
            initiated_recipient_id: None,
            initiated_conversation_id: None,
            initiated_turn_id: None,
            initiated_correlation_id: None,
            initiated_work_id: None,
            initiated_reply: None,
            reply: Some("retained reply".to_owned()),
            accepted_sequence: Some(1),
            turn_sequence: Some(1),
            error: None,
        };
        let (completion, _) = tokio::sync::watch::channel(Some(terminal.clone()));
        source
            .conversation_sessions
            .lock()
            .unwrap()
            .sessions
            .insert(
                "conversation-1".to_owned(),
                ConversationSession {
                    sequence: 1,
                    recipient_id: "gemma-e4b".to_owned(),
                    next_sequence: 1,
                    dispatch_gate: Arc::new(ConversationDispatchGate::at(2)),
                    turns: BTreeMap::from([(
                        "turn-1".to_owned(),
                        ConversationTurn {
                            fingerprint: "fingerprint-1".to_owned(),
                            correlation_id: "correlation-1".to_owned(),
                            sequence: 1,
                            cancellation: CancellationToken::new(),
                            completion,
                            terminal: Some(terminal),
                        },
                    )]),
                },
            );
        let checkpoint = source.checkpoint_agent("gemma-e4b").unwrap();
        assert_eq!(checkpoint.declaration.name, "ember.axioma");
        assert_eq!(checkpoint.declaration.display_name, "Ember Axioma");
        assert_eq!(checkpoint.declaration.office, "local assistant");
        assert_eq!(
            agent_checkpoint_digest(&checkpoint).unwrap(),
            checkpoint.checkpoint_digest
        );
        assert_eq!(checkpoint.conversation_history.len(), 1);
        let checkpoint_json = serde_json::to_value(&checkpoint).unwrap();
        assert!(checkpoint_json["roster_state"].get("name").is_none());
        let decoded_checkpoint: AgentCheckpoint =
            serde_json::from_value(checkpoint_json).expect("v1 checkpoint remains readable");
        assert_eq!(
            decoded_checkpoint.checkpoint_digest,
            checkpoint.checkpoint_digest
        );
        source.refresh_dynamic_agent_health().await;
        assert_eq!(
            source
                .agent_population
                .read()
                .unwrap()
                .sample
                .iter()
                .find(|agent| agent.id == "gemma-e4b")
                .unwrap()
                .health,
            "healthy"
        );
        assert_eq!(
            invoke_ollama_model(
                &request.endpoint,
                &request.model,
                "hello",
                &CancellationToken::new(),
            )
            .await
            .unwrap(),
            "model reply"
        );

        let restarted = service(source_path);
        assert!(restarted
            .dynamic_agents
            .lock()
            .unwrap()
            .iter()
            .any(|agent| agent.id == "gemma-e4b"));

        let bundle = source.dehydrate_agent("gemma-e4b").unwrap();
        assert_eq!(bundle.declaration.name, "ember.axioma");
        assert_eq!(bundle.declaration.display_name, "Ember Axioma");
        assert_eq!(bundle.declaration.office, "local assistant");
        assert_eq!(source.dehydrate_agent("gemma-e4b").unwrap(), bundle);
        assert_eq!(
            freeze_dried_agent_digest(&bundle).unwrap(),
            bundle.bundle_digest
        );
        assert!(source.commit_agent_migration("gemma-e4b", "wrong").is_err());
        assert_eq!(
            source
                .commit_agent_migration("gemma-e4b", &bundle.bundle_digest)
                .unwrap(),
            "removed"
        );

        let destination = service(destination_path);
        assert_eq!(
            destination
                .rehydrate_agent(bundle.clone())
                .await
                .unwrap()
                .status,
            "admitted"
        );
        assert!(destination
            .conversation_sessions
            .lock()
            .unwrap()
            .sessions
            .contains_key("conversation-1"));
        let mut tampered = bundle;
        tampered.declaration.name = "ember.axioma.local".to_owned();
        tampered.checkpoint.declaration = tampered.declaration.clone();
        tampered.checkpoint.checkpoint_digest =
            agent_checkpoint_digest(&tampered.checkpoint).unwrap();
        tampered.bundle_digest = freeze_dried_agent_digest(&tampered).unwrap();
        assert!(destination.rehydrate_agent(tampered).await.is_err());
        assert!(destination.remove_agent("shepherd").is_err());
        ollama_task.abort();
    }
}

#[derive(Debug)]
enum AgentAdmissionFailure {
    Invalid(&'static str),
    Conflict(&'static str),
    Unavailable(&'static str),
}

fn agent_admission_failure_reason(failure: &AgentAdmissionFailure) -> &'static str {
    match failure {
        AgentAdmissionFailure::Invalid(reason)
        | AgentAdmissionFailure::Conflict(reason)
        | AgentAdmissionFailure::Unavailable(reason) => reason,
    }
}

fn inference_readiness_from_agent_admission_failure(
    failure: &AgentAdmissionFailure,
) -> InferenceReadinessState {
    match agent_admission_failure_reason(failure) {
        "invalid_agent_declaration" | "resident_shepherd_provider_unsupported" => {
            InferenceReadinessState::Unimplemented
        }
        "provider_unreachable" | "provider_temporarily_unavailable" | "model_not_installed" => {
            InferenceReadinessState::Unavailable
        }
        "provider_response_invalid" => InferenceReadinessState::Failed,
        _ => match failure {
            AgentAdmissionFailure::Unavailable(_) => InferenceReadinessState::Unavailable,
            AgentAdmissionFailure::Invalid(_) | AgentAdmissionFailure::Conflict(_) => {
                InferenceReadinessState::Failed
            }
        },
    }
}

fn validate_agent_admission_base(request: &AgentAdmissionRequest) -> Result<(), ControlError> {
    if request.schema != AGENT_ADMISSION_SCHEMA
        || request.id == "shepherd"
        || !is_safe_identifier(&request.id)
        || !is_safe_identifier(&request.provider)
        || request.name.is_empty()
        || request.name.len() > 128
        || request.display_name.len() > 128
        || request.office.len() > 128
        || request.role.len() > 128
        || request
            .name
            .chars()
            .chain(request.display_name.chars())
            .chain(request.office.chars())
            .chain(request.role.chars())
            .any(char::is_control)
    {
        return Err(ControlError::InvalidIdentifier);
    }
    match request.provider.as_str() {
        "ollama" | "openai-compatible" => {
            validate_private_provider_binding(&request.model, &request.endpoint)?;
        }
        "vertex_ai" => {
            validate_vertex_ai_provider_endpoint(&request.endpoint, &request.model)?;
        }
        _ => return Err(ControlError::InvalidIdentifier),
    }
    Ok(())
}

pub(crate) fn validate_private_provider_binding(
    model: &str,
    endpoint: &str,
) -> Result<(), ControlError> {
    if !is_safe_identifier(model) {
        return Err(ControlError::InvalidIdentifier);
    }
    let (host, _) = parse_private_provider_endpoint(endpoint)?;
    let host = host.as_str();
    let private = host == "localhost"
        || host.ends_with(".local")
        || host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|address| match address {
                std::net::IpAddr::V4(address) => address.is_loopback() || address.is_private(),
                std::net::IpAddr::V6(address) => address.is_loopback() || address.is_unique_local(),
            });
    if !private {
        return Err(ControlError::InvalidIdentifier);
    }
    Ok(())
}

fn validate_agent_admission(request: &AgentAdmissionRequest) -> Result<(), ControlError> {
    validate_agent_admission_base(request)?;
    if !is_canonical_agent_name(&request.name)
        || request.display_name.is_empty()
        || request.office.is_empty()
        || !request.role.is_empty()
    {
        return Err(ControlError::InvalidIdentifier);
    }
    Ok(())
}

fn validate_persisted_agent_admission(request: &AgentAdmissionRequest) -> Result<(), ControlError> {
    validate_agent_admission_base(request)?;
    match (request.office.is_empty(), request.role.is_empty()) {
        (true, false) => Ok(()),
        (false, true)
            if is_canonical_agent_name(&request.name) && !request.display_name.is_empty() =>
        {
            Ok(())
        }
        _ => Err(ControlError::InvalidIdentifier),
    }
}

fn persisted_agent_canonical_name(request: &AgentAdmissionRequest) -> String {
    if is_canonical_agent_name(&request.name) {
        return request.name.clone();
    }
    let candidate = format!("{}.legacy", request.id);
    if is_canonical_agent_name(&candidate) {
        candidate
    } else {
        format!(
            "legacy.{}",
            &blake3::hash(request.id.as_bytes()).to_hex()[..16]
        )
    }
}

async fn verify_ollama_model(request: &AgentAdmissionRequest) -> Result<(), AgentAdmissionFailure> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let (host, port) = parse_private_provider_endpoint(&request.endpoint)
        .map_err(|_| AgentAdmissionFailure::Invalid("invalid_agent_declaration"))?;
    let address = tokio::net::lookup_host((host.as_str(), port))
        .await
        .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?
        .find(|address| match address.ip() {
            std::net::IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
            std::net::IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
        })
        .ok_or(AgentAdmissionFailure::Unavailable("provider_unreachable"))?;
    let mut stream = tokio::time::timeout(
        Duration::from_secs(2),
        tokio::net::TcpStream::connect(address),
    )
    .await
    .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?
    .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?;
    stream
        .write_all(
            format!(
                "GET /api/tags HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\nAccept: application/json\r\n\r\n"
            )
            .as_bytes(),
        )
        .await
        .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?;
    let mut bytes = Vec::new();
    tokio::time::timeout(
        Duration::from_secs(10),
        stream.take(1_048_577).read_to_end(&mut bytes),
    )
    .await
    .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?
    .map_err(|_| AgentAdmissionFailure::Unavailable("provider_unreachable"))?;
    if bytes.len() > 1_048_576 {
        return Err(AgentAdmissionFailure::Unavailable(
            "provider_response_invalid",
        ));
    }
    let split = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or(AgentAdmissionFailure::Unavailable(
            "provider_response_invalid",
        ))?;
    let headers = std::str::from_utf8(&bytes[..split])
        .map_err(|_| AgentAdmissionFailure::Unavailable("provider_response_invalid"))?;
    if !headers
        .lines()
        .next()
        .is_some_and(|line| line.contains(" 200 "))
    {
        return Err(AgentAdmissionFailure::Unavailable("provider_unreachable"));
    }
    let encoded_body = &bytes[split + 4..];
    let body = if headers.lines().any(|line| {
        line.eq_ignore_ascii_case("transfer-encoding: chunked")
            || line
                .to_ascii_lowercase()
                .starts_with("transfer-encoding: chunked")
    }) {
        decode_http_chunked_body(encoded_body).ok_or(AgentAdmissionFailure::Unavailable(
            "provider_response_invalid",
        ))?
    } else {
        encoded_body.to_vec()
    };
    let value: serde_json::Value = serde_json::from_slice(&body)
        .map_err(|_| AgentAdmissionFailure::Unavailable("provider_response_invalid"))?;
    let found = value["models"].as_array().is_some_and(|models| {
        models.iter().any(|model| {
            model["name"].as_str() == Some(request.model.as_str())
                || model["model"].as_str() == Some(request.model.as_str())
        })
    });
    if !found {
        return Err(AgentAdmissionFailure::Invalid("model_not_installed"));
    }
    Ok(())
}

pub async fn preload_resident_shepherd_model(
    config: &ResidentShepherdInitConfig,
    cancellation: &CancellationToken,
) -> Result<(), &'static str> {
    if !crate::resident_shepherd_provider_is_available(&config.provider) {
        return Err("resident_shepherd_provider_unsupported");
    }
    let request = AgentAdmissionRequest {
        schema: AGENT_ADMISSION_SCHEMA.to_owned(),
        id: "resident-shepherd-preload".to_owned(),
        name: config.name.clone(),
        display_name: config.display_name.clone(),
        office: config.office.clone(),
        role: String::new(),
        provider: config.provider.clone(),
        model: config.model.clone(),
        endpoint: config.endpoint.clone(),
    };
    if config.provider == "ollama" {
        verify_ollama_model(&request)
            .await
            .map_err(|failure| match failure {
                AgentAdmissionFailure::Invalid(reason)
                | AgentAdmissionFailure::Conflict(reason)
                | AgentAdmissionFailure::Unavailable(reason) => reason,
            })?;
    }
    if config.preload.enabled || config.provider != "ollama" {
        invoke_resident_shepherd_provider(
            &config.provider,
            &config.endpoint,
            &config.model,
            "Reply with READY.",
            cancellation,
        )
        .await?;
    }
    Ok(())
}

pub(crate) async fn invoke_resident_shepherd_provider(
    provider: &str,
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<String, &'static str> {
    match provider {
        "ollama" => invoke_ollama_model(endpoint, model, prompt, cancellation).await,
        "openai-compatible" => {
            invoke_openai_compatible_model(endpoint, model, prompt, cancellation).await
        }
        _ => Err("resident_shepherd_provider_unsupported"),
    }
}

async fn verify_agent_provider_route(
    request: &AgentAdmissionRequest,
) -> Result<(), AgentAdmissionFailure> {
    match request.provider.as_str() {
        "ollama" => verify_ollama_model(request).await,
        "vertex_ai" => {
            validate_vertex_ai_provider_endpoint(&request.endpoint, &request.model)
                .map_err(|_| AgentAdmissionFailure::Invalid("invalid_agent_declaration"))?;
            Ok(())
        }
        _ => Err(AgentAdmissionFailure::Invalid("invalid_agent_declaration")),
    }
}

fn decode_http_chunked_body(encoded: &[u8]) -> Option<Vec<u8>> {
    let mut cursor = 0_usize;
    let mut decoded = Vec::new();
    loop {
        let line_end = encoded[cursor..]
            .windows(2)
            .position(|window| window == b"\r\n")?
            + cursor;
        let size_text = std::str::from_utf8(&encoded[cursor..line_end]).ok()?;
        let size = usize::from_str_radix(size_text.split(';').next()?.trim(), 16).ok()?;
        cursor = line_end.checked_add(2)?;
        if size == 0 {
            return encoded
                .get(cursor..cursor + 2)
                .filter(|tail| *tail == b"\r\n")
                .map(|_| decoded);
        }
        let data_end = cursor.checked_add(size)?;
        decoded.extend_from_slice(encoded.get(cursor..data_end)?);
        if encoded.get(data_end..data_end + 2)? != b"\r\n" {
            return None;
        }
        cursor = data_end + 2;
        if decoded.len() > 1_048_576 {
            return None;
        }
    }
}

fn decode_http_response_body(response_headers: &str, encoded: &[u8]) -> Option<Vec<u8>> {
    if response_headers.lines().any(|line| {
        line.to_ascii_lowercase()
            .starts_with("transfer-encoding: chunked")
    }) {
        decode_http_chunked_body(encoded)
    } else if encoded.len() <= 4_194_304 {
        Some(encoded.to_vec())
    } else {
        None
    }
}

pub(crate) async fn invoke_ollama_model(
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<String, &'static str> {
    let request = AgentAdmissionRequest {
        schema: AGENT_ADMISSION_SCHEMA.to_owned(),
        id: "execution-probe".to_owned(),
        name: "execution.probe".to_owned(),
        display_name: "Execution Probe".to_owned(),
        office: "provider execution".to_owned(),
        role: String::new(),
        provider: "ollama".to_owned(),
        model: model.to_owned(),
        endpoint: endpoint.to_owned(),
    };
    validate_agent_admission(&request).map_err(|_| "agent_provider_binding_invalid")?;
    let operation = async {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let (host, port) = parse_private_provider_endpoint(endpoint)
            .map_err(|_| "agent_provider_binding_invalid")?;
        let address = tokio::net::lookup_host((host.as_str(), port))
            .await
            .map_err(|_| "agent_provider_unreachable")?
            .find(|address| match address.ip() {
                std::net::IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
                std::net::IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
            })
            .ok_or("agent_provider_unreachable")?;
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let body = serde_json::to_vec(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "keep_alive": -1,
        }))
        .map_err(|_| "agent_provider_request_invalid")?;
        let headers = format!(
            "POST /api/generate HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n",
            body.len()
        );
        stream
            .write_all(headers.as_bytes())
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        stream
            .write_all(&body)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let mut bytes = Vec::new();
        stream
            .take(4_194_305)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        if bytes.len() > 4_194_304 {
            return Err("agent_provider_response_invalid");
        }
        let split = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or("agent_provider_response_invalid")?;
        let response_headers =
            std::str::from_utf8(&bytes[..split]).map_err(|_| "agent_provider_response_invalid")?;
        if !response_headers
            .lines()
            .next()
            .is_some_and(|line| line.contains(" 200 "))
        {
            return Err("agent_provider_failed");
        }
        let encoded = &bytes[split + 4..];
        let decoded = decode_http_response_body(response_headers, encoded)
            .ok_or("agent_provider_response_invalid")?;
        let response: serde_json::Value =
            serde_json::from_slice(&decoded).map_err(|_| "agent_provider_response_invalid")?;
        response["response"]
            .as_str()
            .filter(|reply| !reply.trim().is_empty())
            .map(str::to_owned)
            .ok_or("agent_provider_response_invalid")
    };
    tokio::select! {
        _ = cancellation.cancelled() => Err("operation cancelled"),
        result = tokio::time::timeout(AGENT_PROVIDER_EXECUTION_TIMEOUT, operation) => {
            result.map_err(|_| "agent_provider_timed_out")?
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderConversationOutput {
    pub message: String,
    pub agent_to_agent: Option<ProviderAgentToAgentAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProviderAgentToAgentAction {
    pub recipient_id: String,
    pub message: String,
}

pub(crate) async fn invoke_provider_conversation(
    provider: &str,
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<ProviderConversationOutput, &'static str> {
    match provider {
        "ollama" => match invoke_ollama_conversation(endpoint, model, prompt, cancellation).await {
            Ok(output) => Ok(output),
            Err("agent_provider_tools_unsupported") => {
                invoke_ollama_model(endpoint, model, prompt, cancellation)
                    .await
                    .map(|message| ProviderConversationOutput {
                        message,
                        agent_to_agent: None,
                    })
            }
            Err(error) => Err(error),
        },
        "vertex_ai" => {
            validate_vertex_ai_provider_endpoint(endpoint, model)
                .map_err(|_| "agent_provider_binding_invalid")?;
            Err("agent_provider_live_call_deferred")
        }
        _ => Err("agent_provider_binding_invalid"),
    }
}

async fn invoke_ollama_conversation(
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<ProviderConversationOutput, &'static str> {
    let request = AgentAdmissionRequest {
        schema: AGENT_ADMISSION_SCHEMA.to_owned(),
        id: "conversation-probe".to_owned(),
        name: "conversation.probe".to_owned(),
        display_name: "Conversation Probe".to_owned(),
        office: "provider execution".to_owned(),
        role: String::new(),
        provider: "ollama".to_owned(),
        model: model.to_owned(),
        endpoint: endpoint.to_owned(),
    };
    validate_agent_admission(&request).map_err(|_| "agent_provider_binding_invalid")?;
    let operation = async {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let (host, port) = parse_private_provider_endpoint(endpoint)
            .map_err(|_| "agent_provider_binding_invalid")?;
        let address = tokio::net::lookup_host((host.as_str(), port))
            .await
            .map_err(|_| "agent_provider_unreachable")?
            .find(|address| match address.ip() {
                std::net::IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
                std::net::IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
            })
            .ok_or("agent_provider_unreachable")?;
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let body = serde_json::to_vec(&serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "tools": [{
                "type": "function",
                "function": {
                    "name": "initiate_agent",
                    "description": "Request a bounded governed message to another admitted resident agent. The Runtime decides whether it is authorized and delivered.",
                    "parameters": {
                        "type": "object",
                        "required": ["recipient_id", "message"],
                        "properties": {
                            "recipient_id": {
                                "type": "string",
                                "description": "Canonical id of the resident recipient"
                            },
                            "message": {
                                "type": "string",
                                "description": "Bounded message for the recipient"
                            }
                        }
                    }
                }
            }],
            "stream": false,
            "keep_alive": -1
        }))
        .map_err(|_| "agent_provider_request_invalid")?;
        let headers = format!(
            "POST /api/chat HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n",
            body.len()
        );
        stream
            .write_all(headers.as_bytes())
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        stream
            .write_all(&body)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let mut bytes = Vec::new();
        stream
            .take(4_194_305)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        if bytes.len() > 4_194_304 {
            return Err("agent_provider_response_invalid");
        }
        let split = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or("agent_provider_response_invalid")?;
        let response_headers =
            std::str::from_utf8(&bytes[..split]).map_err(|_| "agent_provider_response_invalid")?;
        let status_line = response_headers.lines().next().unwrap_or_default();
        if status_line.contains(" 400 ") || status_line.contains(" 404 ") {
            return Err("agent_provider_tools_unsupported");
        }
        if !status_line.contains(" 200 ") {
            return Err("agent_provider_failed");
        }
        let decoded = decode_http_response_body(response_headers, &bytes[split + 4..])
            .ok_or("agent_provider_response_invalid")?;
        let response: serde_json::Value =
            serde_json::from_slice(&decoded).map_err(|_| "agent_provider_response_invalid")?;
        normalize_ollama_conversation_response(&response)
    };
    tokio::select! {
        _ = cancellation.cancelled() => Err("operation cancelled"),
        result = tokio::time::timeout(AGENT_PROVIDER_EXECUTION_TIMEOUT, operation) => {
            result.map_err(|_| "agent_provider_timed_out")?
        }
    }
}

fn normalize_ollama_conversation_response(
    response: &serde_json::Value,
) -> Result<ProviderConversationOutput, &'static str> {
    let message = response
        .get("message")
        .and_then(serde_json::Value::as_object)
        .ok_or("agent_provider_response_invalid")?;
    let content = message
        .get("content")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    let tool_calls = message
        .get("tool_calls")
        .and_then(serde_json::Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    if tool_calls.is_empty() {
        if content.trim().is_empty() {
            return Err("agent_provider_response_invalid");
        }
        return Ok(ProviderConversationOutput {
            message: content.to_owned(),
            agent_to_agent: None,
        });
    }
    if tool_calls.len() != 1 {
        return Err("agent_provider_action_invalid");
    }
    let function = tool_calls[0]
        .get("function")
        .and_then(serde_json::Value::as_object)
        .ok_or("agent_provider_action_invalid")?;
    if function.get("name").and_then(serde_json::Value::as_str) != Some("initiate_agent") {
        return Err("agent_provider_action_invalid");
    }
    let parsed_arguments;
    let arguments = match function.get("arguments") {
        Some(serde_json::Value::Object(arguments)) => arguments,
        Some(serde_json::Value::String(arguments)) => {
            parsed_arguments = serde_json::from_str::<serde_json::Value>(arguments)
                .map_err(|_| "agent_provider_action_invalid")?;
            parsed_arguments
                .as_object()
                .ok_or("agent_provider_action_invalid")?
        }
        _ => return Err("agent_provider_action_invalid"),
    };
    let recipient_id = arguments
        .get("recipient_id")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 128)
        .ok_or("agent_provider_action_invalid")?;
    let peer_message = arguments
        .get("message")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 4_096)
        .ok_or("agent_provider_action_invalid")?;
    let operator_message = if content.trim().is_empty() {
        format!("Requested governed contact with {recipient_id}.")
    } else {
        content.to_owned()
    };
    Ok(ProviderConversationOutput {
        message: operator_message,
        agent_to_agent: Some(ProviderAgentToAgentAction {
            recipient_id: recipient_id.to_owned(),
            message: peer_message.to_owned(),
        }),
    })
}

#[cfg(test)]
mod provider_conversation_tool_tests {
    use super::*;

    async fn read_fixture_request(socket: &mut tokio::net::TcpStream) -> String {
        use tokio::io::AsyncReadExt;
        let mut request = Vec::new();
        while request.len() < 65_536 {
            let mut chunk = [0_u8; 2_048];
            let read = socket.read(&mut chunk).await.unwrap_or_default();
            if read == 0 {
                break;
            }
            request.extend_from_slice(&chunk[..read]);
            let Some(header_end) = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .map(|index| index + 4)
            else {
                continue;
            };
            let content_length = String::from_utf8_lossy(&request[..header_end])
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or_default();
            if request.len() >= header_end + content_length {
                break;
            }
        }
        String::from_utf8_lossy(&request)
            .lines()
            .next()
            .unwrap_or_default()
            .to_owned()
    }

    #[test]
    fn ordinary_assistant_content_stays_an_operator_reply() {
        let output = normalize_ollama_conversation_response(&serde_json::json!({
            "message": {"role": "assistant", "content": "I can help directly."}
        }))
        .expect("ordinary reply");
        assert_eq!(output.message, "I can help directly.");
        assert!(output.agent_to_agent.is_none());
    }

    #[test]
    fn native_tool_call_is_a_typed_action_separate_from_prose() {
        let output = normalize_ollama_conversation_response(&serde_json::json!({
            "message": {
                "role": "assistant",
                "content": "I will request governed contact.",
                "tool_calls": [{
                    "function": {
                        "name": "initiate_agent",
                        "arguments": {
                            "recipient_id": "ember",
                            "message": "Please report your current state."
                        }
                    }
                }]
            }
        }))
        .expect("native action");
        assert_eq!(output.message, "I will request governed contact.");
        assert_eq!(
            output.agent_to_agent,
            Some(ProviderAgentToAgentAction {
                recipient_id: "ember".to_owned(),
                message: "Please report your current state.".to_owned(),
            })
        );
    }

    #[test]
    fn native_tool_call_accepts_json_encoded_arguments() {
        let output = normalize_ollama_conversation_response(&serde_json::json!({
            "message": {
                "role": "assistant",
                "content": "I will ask Ember.",
                "tool_calls": [{
                    "function": {
                        "name": "initiate_agent",
                        "arguments": "{\"recipient_id\":\"ember\",\"message\":\"Please reply through governed A2A.\"}"
                    }
                }]
            }
        }))
        .expect("JSON-encoded Ollama tool arguments should remain first-class");
        assert_eq!(output.message, "I will ask Ember.");
        assert_eq!(
            output.agent_to_agent,
            Some(ProviderAgentToAgentAction {
                recipient_id: "ember".to_owned(),
                message: "Please reply through governed A2A.".to_owned(),
            })
        );
    }

    #[test]
    fn unknown_or_ambiguous_tool_calls_fail_closed() {
        for tool_calls in [
            serde_json::json!([{"function":{"name":"send_unchecked","arguments":{}}}]),
            serde_json::json!([
                {"function":{"name":"initiate_agent","arguments":{"recipient_id":"ember","message":"one"}}},
                {"function":{"name":"initiate_agent","arguments":{"recipient_id":"scribe","message":"two"}}}
            ]),
        ] {
            let error = normalize_ollama_conversation_response(&serde_json::json!({
                "message": {
                    "role": "assistant",
                    "content": "This must not dispatch.",
                    "tool_calls": tool_calls
                }
            }))
            .expect_err("unrecognized or multiple tools must fail closed");
            assert_eq!(error, "agent_provider_action_invalid");
        }
    }

    #[tokio::test]
    async fn unsupported_tools_fall_back_to_plain_generation_without_a2a() {
        use tokio::io::AsyncWriteExt;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind fallback fixture");
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let fixture = tokio::spawn(async move {
            let (mut chat, _) = listener.accept().await.expect("chat request");
            assert_eq!(
                read_fixture_request(&mut chat).await,
                "POST /api/chat HTTP/1.1"
            );
            chat.write_all(
                b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )
            .await
            .unwrap();
            drop(chat);

            let (mut generate, _) = listener.accept().await.expect("generate fallback");
            assert_eq!(
                read_fixture_request(&mut generate).await,
                "POST /api/generate HTTP/1.1"
            );
            let body = br#"{"response":"A normal reply from a model without tools."}"#;
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            generate.write_all(headers.as_bytes()).await.unwrap();
            generate.write_all(body).await.unwrap();
        });

        let output = invoke_provider_conversation(
            "ollama",
            &endpoint,
            "plain-model",
            "answer normally",
            &CancellationToken::new(),
        )
        .await
        .expect("plain generation fallback");
        assert_eq!(output.message, "A normal reply from a model without tools.");
        assert!(output.agent_to_agent.is_none());
        fixture.await.unwrap();
    }
}

async fn invoke_openai_compatible_model(
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<String, &'static str> {
    validate_private_provider_binding(model, endpoint)
        .map_err(|_| "agent_provider_binding_invalid")?;
    let operation = async {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let (host, port) = parse_private_provider_endpoint(endpoint)
            .map_err(|_| "agent_provider_binding_invalid")?;
        let address = tokio::net::lookup_host((host.as_str(), port))
            .await
            .map_err(|_| "agent_provider_unreachable")?
            .find(|address| match address.ip() {
                std::net::IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
                std::net::IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
            })
            .ok_or("agent_provider_unreachable")?;
        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let body = serde_json::to_vec(&serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": false
        }))
        .map_err(|_| "agent_provider_request_invalid")?;
        let headers = format!(
            "POST /v1/chat/completions HTTP/1.1\r\nHost: {host}:{port}\r\nConnection: close\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: {}\r\n\r\n",
            body.len()
        );
        stream
            .write_all(headers.as_bytes())
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        stream
            .write_all(&body)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let mut bytes = Vec::new();
        stream
            .take(4_194_305)
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| "agent_provider_unreachable")?;
        let split = bytes
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or("agent_provider_response_invalid")?;
        let response_headers =
            std::str::from_utf8(&bytes[..split]).map_err(|_| "agent_provider_response_invalid")?;
        if !response_headers
            .lines()
            .next()
            .is_some_and(|line| line.contains(" 200 "))
        {
            return Err("agent_provider_failed");
        }
        let decoded = decode_http_response_body(response_headers, &bytes[split + 4..])
            .ok_or("agent_provider_response_invalid")?;
        let response: serde_json::Value =
            serde_json::from_slice(&decoded).map_err(|_| "agent_provider_response_invalid")?;
        response["choices"][0]["message"]["content"]
            .as_str()
            .filter(|reply| !reply.trim().is_empty())
            .map(str::to_owned)
            .ok_or("agent_provider_response_invalid")
    };
    tokio::select! {
        _ = cancellation.cancelled() => Err("operation cancelled"),
        result = tokio::time::timeout(AGENT_PROVIDER_EXECUTION_TIMEOUT, operation) => result.map_err(|_| "agent_provider_timed_out")?,
    }
}

pub(crate) async fn invoke_provider_model(
    provider: &str,
    endpoint: &str,
    model: &str,
    prompt: &str,
    cancellation: &CancellationToken,
) -> Result<String, &'static str> {
    match provider {
        "ollama" => invoke_ollama_model(endpoint, model, prompt, cancellation).await,
        "vertex_ai" => {
            validate_vertex_ai_provider_endpoint(endpoint, model)
                .map_err(|_| "agent_provider_binding_invalid")?;
            Err("agent_provider_live_call_deferred")
        }
        _ => Err("agent_provider_binding_invalid"),
    }
}

fn parse_private_provider_endpoint(endpoint: &str) -> Result<(String, u16), ControlError> {
    let authority = endpoint
        .strip_prefix("http://")
        .ok_or(ControlError::InvalidIdentifier)?;
    let authority = authority.strip_suffix('/').unwrap_or(authority);
    if authority.is_empty() || authority.contains(['/', '@', '?', '#', '\r', '\n', '\t', ' ']) {
        return Err(ControlError::InvalidIdentifier);
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (
            host,
            port.parse::<u16>()
                .map_err(|_| ControlError::InvalidIdentifier)?,
        ),
        None => (authority, 11434),
    };
    if host.is_empty() || port == 0 {
        return Err(ControlError::InvalidIdentifier);
    }
    Ok((host.to_owned(), port))
}

fn validate_vertex_ai_provider_endpoint(endpoint: &str, model: &str) -> Result<(), ControlError> {
    let without_scheme = endpoint
        .strip_prefix("https://")
        .ok_or(ControlError::InvalidIdentifier)?;
    if without_scheme.contains(['@', '?', '#', '\r', '\n', '\t', ' ']) {
        return Err(ControlError::InvalidIdentifier);
    }
    let (host, path) = without_scheme
        .split_once('/')
        .ok_or(ControlError::InvalidIdentifier)?;
    if !host.ends_with("-aiplatform.googleapis.com") {
        return Err(ControlError::InvalidIdentifier);
    }
    let expected_model_suffix = format!("/publishers/google/models/{model}:generateContent");
    let path = format!("/{path}");
    if !path.starts_with("/v1/projects/")
        || !path.contains("/locations/")
        || !path.ends_with(&expected_model_suffix)
    {
        return Err(ControlError::InvalidIdentifier);
    }
    Ok(())
}

fn agent_sample(request: &AgentAdmissionRequest) -> AgentSample {
    let now = now_unix_millis();
    let readiness = InferenceReadinessState::ModelLoading;
    let projection = readiness.projection();
    AgentSample {
        id: request.id.clone(),
        name: request.name.clone(),
        label: if request.display_name.is_empty() {
            request.name.clone()
        } else {
            request.display_name.clone()
        },
        role: if request.office.is_empty() {
            request.role.clone()
        } else {
            request.office.clone()
        },
        provider: Some(request.provider.clone()),
        model: Some(request.model.clone()),
        inference_readiness: readiness,
        state: readiness.as_str().to_owned(),
        detail: format!(
            "{} model {} verification pending",
            request.provider, request.model
        ),
        health: projection.health.to_owned(),
        availability: projection.availability.to_owned(),
        activity: projection.activity.map(str::to_owned),
        capabilities: vec!["conversation".to_owned()],
        location: Some("local_runtime".to_owned()),
        communication_eligible: projection.communication_eligible,
        observed_at_unix_millis: now,
        freshness_deadline_unix_millis: now.saturating_add(30_000),
        source_revision: blake3::hash(
            serde_json::to_vec(request)
                .expect("validated admission serializes")
                .as_slice(),
        )
        .to_hex()
        .to_string(),
        provenance: "runtime_dynamic_admission".to_owned(),
    }
}

fn persist_dynamic_agents(
    path: &Path,
    agents: &[AgentAdmissionRequest],
) -> Result<(), ControlError> {
    persist_json_atomically(
        path,
        &DynamicAgentStore {
            schema: DYNAMIC_AGENT_STORE_SCHEMA.to_owned(),
            agents: agents.to_vec(),
        },
    )
}

fn persist_json_atomically(path: &Path, value: &impl Serialize) -> Result<(), ControlError> {
    let parent = path.parent().ok_or(ControlError::InvalidIdentifier)?;
    fs::create_dir_all(parent).map_err(|error| ControlError::Io(error.to_string()))?;
    let temp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| ControlError::Encoding(error.to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp)
        .map_err(|error| ControlError::Io(error.to_string()))?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|error| ControlError::Io(error.to_string()))?;
    fs::rename(&temp, path).map_err(|error| ControlError::Io(error.to_string()))?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| ControlError::Io(error.to_string()))?;
    Ok(())
}

fn agent_checkpoint_digest(checkpoint: &AgentCheckpoint) -> Result<String, ControlError> {
    let mut unsigned = checkpoint.clone();
    unsigned.checkpoint_digest.clear();
    serde_json::to_vec(&unsigned)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| ControlError::Encoding(error.to_string()))
}

fn freeze_dried_agent_digest(bundle: &FreezeDriedAgent) -> Result<String, ControlError> {
    let mut unsigned = bundle.clone();
    unsigned.bundle_digest.clear();
    serde_json::to_vec(&unsigned)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| ControlError::Encoding(error.to_string()))
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
                    sender_id: None,
                    initiated_recipient_id: None,
                    initiated_conversation_id: None,
                    initiated_turn_id: None,
                    initiated_correlation_id: None,
                    initiated_work_id: None,
                    initiated_reply: None,
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
                    &later_cancellation,
                    tokio::time::Instant::now() + Duration::from_secs(60),
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

    #[tokio::test]
    async fn later_turn_queue_wait_expires_without_advancing_the_gate() {
        let gate = ConversationDispatchGate::new();
        let cancellation = CancellationToken::new();

        assert!(
            !gate
                .wait_turn(2, &cancellation, tokio::time::Instant::now())
                .await
        );
        assert!(gate.ready(1));
        assert!(!cancellation.is_cancelled());
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
