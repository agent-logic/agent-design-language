use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    io::Write,
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, Weak,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::{
        ws::{close_code, CloseFrame, Message, WebSocket, WebSocketUpgrade},
        DefaultBodyLimit, State,
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
use tracing::Instrument;
use utoipa_swagger_ui::{Config as SwaggerConfig, SwaggerUi, Url as SwaggerUrl};

use crate::{
    decode_acip_envelope, decode_strict_acip_envelope, BootstrapEvent, CanonicalIngress,
    CheckpointManifest, ClockAuthority, CommunicationSigningIdentity, ComponentId, DomainResult,
    DomainWork, IngressError, KernelControl, KernelExit, LifecycleState, LiveContinuity,
    ObservabilityHealth, RunningState, RuntimeRecorder, RuntimeSnapshot, RuntimeTlsInitConfig,
    SignedIdentityMessage, WeatherHealthReport, ACIP_IDENTITY_MESSAGE_SCHEMA,
    ACIP_WEBSOCKET_SCHEMA, LAYER8_MESSAGE_TASK_OP, LOCAL_AGENT_WORK_SCHEMA,
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
pub const OBSERVATORY_WS_LAYER8_INTENT_SCHEMA: &str = "adl.runtime_v3.observatory_layer8_intent.v1";
pub const CONTROL_MAX_BODY_BYTES: usize = 64 * 1024;
const MAX_INTEROPERABLE_SEQUENCE: u64 = 9_007_199_254_740_991;
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
    Submit { work: DomainWork },
    Shutdown { grace_millis: u64 },
    Restart { grace_millis: u64 },
}

impl ControlAction {
    fn capability(&self) -> ControlCapability {
        match self {
            Self::Snapshot => ControlCapability::Read,
            Self::Submit { .. } => ControlCapability::Execute,
            Self::Shutdown { .. } | Self::Restart { .. } => ControlCapability::Stop,
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
            ControlAction::Shutdown { grace_millis } | ControlAction::Restart { grace_millis }
                if grace_millis == 0 || grace_millis > MAX_SHUTDOWN_GRACE_MILLIS
        ) {
            return Err(ControlError::InvalidBounds);
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
    Restart { exit: ControlExit },
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

    async fn restart(&self, grace: Duration) -> Result<KernelExit, ()> {
        self.shutdown(grace).await
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

pub struct ControlService<C> {
    instance_id: String,
    incarnation_id: String,
    polis_name: Mutex<String>,
    recorder: RuntimeRecorder,
    lifecycle: C,
    authority: ControlAuthority,
    max_records: usize,
    idempotency: Mutex<IdempotencyState>,
    acip_replay: Mutex<AcipReplayState>,
    acip_source_locks: Mutex<BTreeMap<String, Weak<tokio::sync::Mutex<()>>>>,
    weather: Mutex<Option<ObservedWeather>>,
    weather_stale_after_millis: Mutex<u64>,
    observatory_bearer_digest: Mutex<Option<blake3::Hash>>,
    acip_write_bearer_digest: Mutex<Option<blake3::Hash>>,
    observatory_allowed_origins: BTreeSet<String>,
    agent_population: AgentPopulationFeed,
    control_addr: Mutex<SocketAddr>,
    public_base_url: Mutex<String>,
    canonical_ingress: Option<CanonicalIngress>,
    api_policy: Mutex<Option<ControlApiPolicy>>,
    layer8_signer: Option<CommunicationSigningIdentity>,
    layer8_sequence: AtomicU64,
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
            incarnation_id: uuid::Uuid::new_v4().simple().to_string(),
            polis_name: Mutex::new("Unconfigured Polis".to_owned()),
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
            acip_source_locks: Mutex::new(BTreeMap::new()),
            weather: Mutex::new(None),
            weather_stale_after_millis: Mutex::new(30_000),
            observatory_bearer_digest: Mutex::new(None),
            acip_write_bearer_digest: Mutex::new(None),
            observatory_allowed_origins,
            agent_population,
            control_addr: Mutex::new(SocketAddr::from(([127, 0, 0, 1], 0))),
            public_base_url: Mutex::new("https://runtime.invalid".to_owned()),
            canonical_ingress: None,
            api_policy: Mutex::new(None),
            layer8_signer: None,
            layer8_sequence: AtomicU64::new(0),
        }
    }

    pub fn with_layer8_signer(mut self, signer: CommunicationSigningIdentity) -> Self {
        self.layer8_signer = Some(signer);
        self
    }

    async fn execute_layer8_intent(
        self: &Arc<Self>,
        intent: ObservatoryWsLayer8Intent,
    ) -> Result<ControlResponse, ControlError> {
        if intent.schema != OBSERVATORY_WS_LAYER8_INTENT_SCHEMA
            || !is_safe_identifier(&intent.recipient_id)
            || !is_correlation_id(&intent.correlation_id)
            || !is_safe_identifier(&intent.causation_id)
            || intent.content.trim().is_empty()
            || intent.content.len() > 4_000
            || intent
                .content
                .chars()
                .any(|character| character.is_control() && !matches!(character, '\n' | '\t'))
        {
            return Err(ControlError::InvalidBounds);
        }
        let signer = self
            .layer8_signer
            .as_ref()
            .ok_or(ControlError::Unauthorized)?;
        let now = self
            .recorder
            .qualified_time_now_unix_millis()
            .ok_or(ControlError::Internal)?;
        let sequence = self.allocate_layer8_sequence(now)?;
        let nonce = intent.causation_id.clone();
        let mut message = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "request".to_owned(),
            sender_id: "layer8-operator".to_owned(),
            recipient_id: intent.recipient_id.clone(),
            correlation_id: intent.correlation_id.clone(),
            causation_id: intent.causation_id,
            monotonic_sequence: sequence,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now.saturating_add(60_000),
            nonce: nonce.clone(),
            content: intent.content,
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: signer.signing_key_id.clone(),
            signature: String::new(),
        };
        message.signature = hex::encode(
            signer
                .signing_key
                .sign(&message.signing_bytes().ok_or(ControlError::Encoding(
                    "identity message encoding".to_owned(),
                ))?)
                .to_bytes(),
        );
        let work_id = format!("layer8-{sequence:016x}");
        let work = DomainWork {
            schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: work_id.clone(),
            kind: if intent.recipient_id == crate::RESIDENT_SHEPHERD_ID {
                crate::RESIDENT_SHEPHERD_ID.to_owned()
            } else {
                "agent".to_owned()
            },
            payload: serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": message}]
            }))
            .map_err(|error| ControlError::Encoding(error.to_string()))?,
        };
        let addressable_agents = self
            .agent_population
            .clone()
            .with_runtime_snapshot(&self.recorder.snapshot());
        validate_layer8_recipient(&work, &addressable_agents, &intent.correlation_id)?;
        let result = self
            .canonical_ingress
            .as_ref()
            .ok_or(ControlError::AdmissionClosed)?
            .submit(work, intent.correlation_id.clone())
            .await
            .map_err(|error| match error {
                IngressError::Invalid | IngressError::UnsupportedKind => {
                    ControlError::InvalidBounds
                }
                IngressError::Conflict => ControlError::IdempotencyConflict,
                IngressError::Saturated => ControlError::Backpressure,
                IngressError::Closed => ControlError::AdmissionClosed,
                IngressError::ExecutionFailed | IngressError::DrainTimeout => {
                    ControlError::Internal
                }
            })?;
        Ok(ControlResponse {
            schema: CONTROL_RESPONSE_SCHEMA.to_owned(),
            command_id: work_id,
            correlation_id: intent.correlation_id,
            outcome: ControlOutcome::Submitted {
                work_result: result,
            },
        })
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
        let restored_sequence = ingress
            .snapshot()
            .communication_sequences
            .get("layer8-operator")
            .copied()
            .unwrap_or(0);
        self.layer8_sequence
            .fetch_max(restored_sequence, Ordering::SeqCst);
        self.canonical_ingress = Some(ingress);
        self
    }

    fn allocate_layer8_sequence(&self, qualified_now: u64) -> Result<u64, ControlError> {
        self.layer8_sequence
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                next_layer8_sequence(current, qualified_now)
            })
            .ok()
            .and_then(|current| next_layer8_sequence(current, qualified_now))
            .ok_or(ControlError::Internal)
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

    pub fn set_polis_name(&self, polis_name: &str) -> Result<(), ControlError> {
        if polis_name.trim().is_empty()
            || polis_name != polis_name.trim()
            || polis_name.len() > 128
            || polis_name.chars().any(char::is_control)
        {
            return Err(ControlError::InvalidBounds);
        }
        *self.polis_name.lock().expect("Polis name mutex poisoned") = polis_name.to_owned();
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

    fn acip_source_lock(&self, source: &str) -> Arc<tokio::sync::Mutex<()>> {
        let mut locks = self
            .acip_source_locks
            .lock()
            .expect("ACIP source lock registry poisoned");
        locks.retain(|_, lock| lock.strong_count() > 0);
        if let Some(lock) = locks.get(source).and_then(Weak::upgrade) {
            return lock;
        }
        let lock = Arc::new(tokio::sync::Mutex::new(()));
        locks.insert(source.to_owned(), Arc::downgrade(&lock));
        lock
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
        let secure_route = matches!(envelope.route.as_str(), "agent" | "shepherd");
        let secure_message = if secure_route {
            if decode_strict_acip_envelope(payload).is_err() {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "secure_carrier_v1_required",
                    "sequence_reserved": false
                });
            }
            let message =
                match serde_json::from_str::<SignedIdentityMessage>(&envelope.payload_json) {
                    Ok(message) => message,
                    Err(_) => {
                        return serde_json::json!({
                            "schema": ACIP_WEBSOCKET_SCHEMA,
                            "status": "rejected",
                            "message_id": envelope.message_id,
                            "reason": "signed_identity_message_required",
                            "sequence_reserved": false
                        });
                    }
                };
            let expected_route = if message.recipient_id == crate::RESIDENT_SHEPHERD_ID {
                "shepherd"
            } else {
                "agent"
            };
            let expected_replay_id =
                format!("{}:{}", message.sender_id, message.monotonic_sequence);
            if message.message_kind != "request"
                || envelope.message_id != message.nonce
                || envelope.source != message.sender_id
                || envelope.target != message.recipient_id
                || envelope.route != expected_route
                || envelope.capability != expected_route
                || envelope.correlation_id != message.correlation_id
                || envelope.causation_id != message.causation_id
                || envelope.trace_id != message.correlation_id
                || envelope.monotonic_sequence != message.monotonic_sequence
                || envelope.replay_id != expected_replay_id
                || envelope.runtime_id != self.instance_id
                || envelope.authority != "signed-communication-identity"
                || envelope.payload_type != "application/json"
                || !envelope.acknowledgement_requested
                || envelope.error_code.is_some()
                || !envelope.required_features.is_empty()
            {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "signed_identity_carrier_mismatch",
                    "sequence_reserved": false
                });
            }
            let Some(now) = self.recorder.qualified_time_now_unix_millis() else {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "trusted_time_unavailable",
                    "sequence_reserved": false
                });
            };
            if ingress.verify_communication_message(&message, now).is_err() {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "signed_identity_verification_failed",
                    "sequence_reserved": false
                });
            }
            let addressable_agents = self
                .agent_population
                .clone()
                .with_runtime_snapshot(&self.recorder.snapshot());
            if !recipient_is_running(&addressable_agents, &message.sender_id) {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "sender_not_running",
                    "sequence_reserved": false
                });
            }
            if !recipient_is_running(&addressable_agents, &message.recipient_id) {
                return serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": "recipient_not_running",
                    "sequence_reserved": false
                });
            }
            Some(message)
        } else {
            None
        };
        let source_lock = self.acip_source_lock(&envelope.source);
        let _source_dispatch = source_lock.lock().await;
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
        let work_payload = match secure_message {
            Some(message) => serde_json::to_vec(&serde_json::json!({
                "schema": LOCAL_AGENT_WORK_SCHEMA,
                "tasks": [{"op": LAYER8_MESSAGE_TASK_OP, "message": message}]
            }))
            .expect("signed communication work projection is JSON serializable"),
            None => envelope.payload_json.as_bytes().to_vec(),
        };
        let work = DomainWork {
            schema: crate::DOMAIN_WORK_SCHEMA.to_owned(),
            work_id: envelope.message_id.clone(),
            kind: envelope.route.clone(),
            payload: work_payload,
        };
        let communication_sequence_before = if secure_route {
            ingress
                .snapshot()
                .communication_sequences
                .get(&envelope.source)
                .copied()
        } else {
            None
        };
        match ingress.submit(work, envelope.correlation_id.clone()).await {
            Ok(result) => serde_json::json!({
                "schema": ACIP_WEBSOCKET_SCHEMA,
                "status": "completed",
                "message_id": envelope.message_id,
                "accepted_sequence": result.accepted_sequence,
                "result_hash": result.result_hash,
                "signed_ack": result.public_output,
                "sequence_reserved": true
            }),
            Err(error) => {
                let communication_sequence_after = ingress
                    .snapshot()
                    .communication_sequences
                    .get(&envelope.source)
                    .copied();
                let sequence_reserved = secure_route
                    && communication_sequence_before
                        .is_none_or(|sequence| sequence < envelope.monotonic_sequence)
                    && communication_sequence_after
                        .is_some_and(|sequence| sequence >= envelope.monotonic_sequence);
                if !sequence_reserved {
                    self.rollback_acip_sequence(reservation);
                }
                let reason = if secure_route && error == IngressError::Conflict {
                    "monotonic_sequence_must_advance".to_owned()
                } else {
                    error.to_string()
                };
                serde_json::json!({
                    "schema": ACIP_WEBSOCKET_SCHEMA,
                    "status": "rejected",
                    "message_id": envelope.message_id,
                    "reason": reason,
                    "sequence_reserved": sequence_reserved
                })
            }
        }
    }

    pub fn observatory_feed(&self) -> ObservatoryFeed {
        let captured_at_unix_millis = self.recorder.qualified_time_now_unix_millis();
        let snapshot = self.recorder.snapshot();
        let agents = self
            .agent_population
            .clone()
            .with_runtime_snapshot(&snapshot);
        let observability_ready = matches!(snapshot.observability, ObservabilityHealth::Ready);
        let continuity_head = snapshot.continuity_head.clone();
        let events = self.recorder.events();
        let weather = self.weather.lock().expect("weather mutex poisoned").clone();
        let stale_after_millis = *self
            .weather_stale_after_millis
            .lock()
            .expect("weather staleness mutex poisoned");
        let weather_freshness = weather.as_ref().map(|weather| {
            let observed_at_unix_millis = weather.observed_at_unix_millis;
            let age_millis = captured_at_unix_millis
                .map(|now| now.saturating_sub(observed_at_unix_millis))
                .unwrap_or(u64::MAX);
            ObservatoryWeatherFreshness {
                observed_at_unix_millis,
                age_millis,
                stale_after_millis,
                stale: captured_at_unix_millis.is_none() || age_millis > stale_after_millis,
            }
        });
        ObservatoryFeed {
            schema: OBSERVATORY_FEED_SCHEMA.to_owned(),
            source_revision: env!("ADL_BUILD_SOURCE_REVISION").to_owned(),
            polis_name: self
                .polis_name
                .lock()
                .expect("Polis name mutex poisoned")
                .clone(),
            runtime_instance_id: self.instance_id.clone(),
            runtime_incarnation_id: self.incarnation_id.clone(),
            runtime_process_id: std::process::id(),
            captured_at_unix_millis,
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
        if matches!(feed.health.snapshot.clock, ClockAuthority::Degraded { .. }) {
            degraded_reasons.push("trusted_time_unavailable".to_owned());
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
            runtime_process_id: feed.runtime_process_id,
            weather_freshness,
            degraded_reasons,
        }
    }

    pub async fn execute(
        self: &Arc<Self>,
        command: SignedControlCommand,
    ) -> Result<ControlResponse, ControlError> {
        self.authority.authorize(&command)?;
        if command.runtime_instance_id != self.instance_id {
            return Err(ControlError::StaleRuntimeInstance);
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
        let terminal = matches!(
            command.action,
            ControlAction::Shutdown { .. } | ControlAction::Restart { .. }
        );
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
                    let addressable_agents = self
                        .agent_population
                        .clone()
                        .with_runtime_snapshot(&self.recorder.snapshot());
                    if let Err(error) = validate_layer8_recipient(
                        &work,
                        &addressable_agents,
                        &command.correlation_id,
                    ) {
                        tracing::warn!(
                            error = %error,
                            stage = "layer8_recipient_validation",
                            "runtime control submission rejected"
                        );
                        return Err(error);
                    }
                    let result = self
                        .canonical_ingress
                        .as_ref()
                        .ok_or(ControlError::AdmissionClosed)?
                        .submit(work, command.correlation_id.clone())
                        .await
                        .map_err(|error| {
                            tracing::warn!(
                                error = %error,
                                stage = "canonical_ingress",
                                "runtime control submission rejected"
                            );
                            match error {
                                IngressError::Invalid | IngressError::UnsupportedKind => {
                                    ControlError::InvalidBounds
                                }
                                IngressError::Conflict => ControlError::IdempotencyConflict,
                                IngressError::Saturated => ControlError::Backpressure,
                                IngressError::Closed => ControlError::AdmissionClosed,
                                IngressError::ExecutionFailed | IngressError::DrainTimeout => {
                                    ControlError::Internal
                                }
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
                ControlAction::Restart { grace_millis } => {
                    let exit = self
                        .lifecycle
                        .restart(Duration::from_millis(grace_millis))
                        .await
                        .map(|exit| match exit {
                            KernelExit::Clean => ControlExit::Clean,
                            _ => ControlExit::Failed,
                        })
                        .unwrap_or(ControlExit::Failed);
                    Ok(ControlOutcome::Restart { exit })
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

fn next_layer8_sequence(current: u64, qualified_now: u64) -> Option<u64> {
    current
        .checked_add(1)
        .map(|next| next.max(qualified_now))
        .filter(|next| *next <= MAX_INTEROPERABLE_SEQUENCE)
}

fn validate_layer8_recipient(
    work: &DomainWork,
    agents: &AgentPopulationFeed,
    command_correlation_id: &str,
) -> Result<(), ControlError> {
    if !matches!(work.kind.as_str(), "agent" | "shepherd") {
        return Ok(());
    }
    let command: serde_json::Value =
        serde_json::from_slice(&work.payload).map_err(|_| ControlError::InvalidBounds)?;
    if command.get("schema").and_then(serde_json::Value::as_str) != Some(LOCAL_AGENT_WORK_SCHEMA) {
        return Ok(());
    }
    let Some(tasks) = command.get("tasks").and_then(serde_json::Value::as_array) else {
        return Err(ControlError::InvalidBounds);
    };
    let layer8_tasks = tasks
        .iter()
        .filter(|task| {
            task.get("op").and_then(serde_json::Value::as_str) == Some(LAYER8_MESSAGE_TASK_OP)
        })
        .collect::<Vec<_>>();
    if layer8_tasks.is_empty() {
        return Ok(());
    }
    if tasks.len() != 1 || layer8_tasks.len() != 1 {
        return Err(ControlError::InvalidBounds);
    }
    let message: SignedIdentityMessage = serde_json::from_value(
        layer8_tasks[0]
            .get("message")
            .cloned()
            .ok_or(ControlError::InvalidBounds)?,
    )
    .map_err(|_| ControlError::InvalidBounds)?;
    if message.message_kind != "request"
        || message.correlation_id != command_correlation_id
        || (work.kind == "shepherd" && message.recipient_id != crate::RESIDENT_SHEPHERD_ID)
        || (work.kind == "agent" && message.recipient_id == crate::RESIDENT_SHEPHERD_ID)
    {
        return Err(ControlError::InvalidBounds);
    }
    recipient_is_running(agents, &message.recipient_id)
        .then_some(())
        .ok_or(ControlError::Unauthorized)
}

fn recipient_is_running(agents: &AgentPopulationFeed, recipient_id: &str) -> bool {
    agents
        .sample
        .iter()
        .any(|agent| agent.id == recipient_id && agent.state == "running")
}

pub const RUNTIME_READINESS_SCHEMA: &str = "adl.runtime_v3.readiness.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeReadinessReport {
    pub schema: String,
    pub ready: bool,
    pub lifecycle: LifecycleState,
    pub observability_ready: bool,
    pub runtime_instance_id: String,
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
    pub total_count: u64,
    pub rendered_sample_count: u64,
    pub sample: Vec<AgentSample>,
}

impl AgentPopulationFeed {
    pub fn empty() -> Self {
        Self {
            total_count: 0,
            rendered_sample_count: 0,
            sample: Vec::new(),
        }
    }

    pub fn resident_shepherd_with_identity(signing_key_id: &str, signing_public_key: &str) -> Self {
        Self {
            total_count: 0,
            rendered_sample_count: 0,
            sample: vec![AgentSample {
                id: crate::RESIDENT_SHEPHERD_ID.to_owned(),
                label: "Shepherd".to_owned(),
                role: "resident shepherd".to_owned(),
                state: "starting".to_owned(),
                detail: "Runtime component state: starting".to_owned(),
                signing_algorithm: "ed25519".to_owned(),
                signing_key_id: signing_key_id.to_owned(),
                signing_public_key: signing_public_key.to_owned(),
            }],
        }
    }

    fn with_runtime_snapshot(mut self, snapshot: &RuntimeSnapshot) -> Self {
        let shepherd_id = ComponentId::new(crate::RESIDENT_SHEPHERD_ID);
        let agent_id = ComponentId::new("agent_runtime");
        let shepherd_state = snapshot.components.get(&shepherd_id);
        let agent_state = snapshot.components.get(&agent_id);
        self.sample.retain_mut(|agent| {
            let state = if agent.id == crate::RESIDENT_SHEPHERD_ID {
                shepherd_state
            } else {
                agent_state
            };
            let Some(state) = state else {
                return false;
            };
            agent.state = runtime_agent_state(state).to_owned();
            agent.detail = format!("Runtime component state: {}", runtime_agent_state(state));
            true
        });
        self.rendered_sample_count = self.sample.len() as u64;
        if self.sample.is_empty() {
            self.total_count = 0;
        } else {
            self.total_count = self.total_count.max(self.rendered_sample_count);
        }
        self
    }
}

fn runtime_agent_state(state: &RunningState) -> &'static str {
    match state {
        RunningState::Starting | RunningState::Ready => "starting",
        RunningState::Running => "running",
        RunningState::Restarting => "restarting",
        RunningState::Degraded => "degraded",
        RunningState::Stopping | RunningState::Stopped => "stopped",
        RunningState::Failed => "failed",
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentSample {
    pub id: String,
    pub label: String,
    pub role: String,
    pub state: String,
    pub detail: String,
    pub signing_algorithm: String,
    pub signing_key_id: String,
    pub signing_public_key: String,
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
    pub source_revision: String,
    pub polis_name: String,
    pub runtime_instance_id: String,
    pub runtime_incarnation_id: String,
    pub runtime_process_id: u32,
    pub captured_at_unix_millis: Option<u64>,
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryWsLayer8Intent {
    schema: String,
    recipient_id: String,
    correlation_id: String,
    causation_id: String,
    content: String,
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
    let (command_results_tx, mut command_results_rx) = tokio::sync::mpsc::channel::<String>(16);
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
            Some(payload) = command_results_rx.recv() => {
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
                    if let Ok(intent) = serde_json::from_str::<ObservatoryWsLayer8Intent>(&payload) {
                        let correlation_id = is_correlation_id(&intent.correlation_id)
                            .then(|| intent.correlation_id.clone());
                        let token_was_present = bearer_token.is_some();
                        let authorized = bearer_token.as_deref().is_some_and(|token| {
                            service.observatory_token_authorized(token)
                        });
                        if !authorized {
                            bearer_token = None;
                            let rejected = ObservatoryWsControlResult {
                                schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                                status: "rejected",
                                command_id: None,
                                correlation_id,
                                response: None,
                                error: Some(if token_was_present {
                                    "credential_revoked"
                                } else {
                                    "write_authentication_required"
                                }),
                            };
                            let Ok(payload) = serde_json::to_string(&rejected) else {
                                break;
                            };
                            if socket.send(Message::Text(payload.into())).await.is_err() {
                                break;
                            }
                            continue;
                        }
                        if let Ok(permit) = command_results_tx.clone().try_reserve_owned() {
                            let service = service.clone();
                            tokio::spawn(async move {
                                let result = match service.execute_layer8_intent(intent).await {
                                    Ok(response) => ObservatoryWsControlResult {
                                        schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                                        status: "accepted",
                                        command_id: None,
                                        correlation_id,
                                        response: Some(response),
                                        error: None,
                                    },
                                    Err(error) => ObservatoryWsControlResult {
                                        schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                                        status: "rejected",
                                        command_id: None,
                                        correlation_id,
                                        response: None,
                                        error: Some(control_error_code(&error)),
                                    },
                                };
                                if let Ok(payload) = serde_json::to_string(&result) {
                                    permit.send(payload);
                                }
                            });
                            continue;
                        }
                        let backpressure = ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: "rejected",
                            command_id: None,
                            correlation_id,
                            response: None,
                            error: Some("backpressure"),
                        };
                        let Ok(payload) = serde_json::to_string(&backpressure) else {
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
                    } else if let Ok(permit) = command_results_tx.clone().try_reserve_owned() {
                        let service = service.clone();
                        tokio::spawn(async move {
                            let result = match command {
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
                            };
                            if let Ok(payload) = serde_json::to_string(&result) {
                                permit.send(payload);
                            }
                        });
                        continue;
                    } else {
                        ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: "rejected",
                            command_id,
                            correlation_id,
                            response: None,
                            error: Some("backpressure"),
                        }
                    };
                    let Ok(payload) = serde_json::to_string(&result) else {
                        break;
                    };
                    if socket.send(Message::Text(payload.into())).await.is_err() {
                        break;
                    }
                }
                Some(Ok(Message::Binary(payload))) => {
                    if bearer_token
                        .as_deref()
                        .is_some_and(|token| !service.observatory_token_authorized(token))
                    {
                        bearer_token = None;
                    }
                    if bearer_token.is_none() {
                        let rejected = ObservatoryWsControlResult {
                            schema: OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
                            status: "rejected",
                            command_id: None,
                            correlation_id: None,
                            response: None,
                            error: Some("write_authentication_required"),
                        };
                        let Ok(payload) = serde_json::to_string(&rejected) else {
                            break;
                        };
                        if socket.send(Message::Text(payload.into())).await.is_err() {
                            break;
                        }
                        continue;
                    }
                    let Ok(permit) = command_results_tx.clone().try_reserve_owned() else {
                        let backpressure = serde_json::json!({
                            "schema": ACIP_WEBSOCKET_SCHEMA,
                            "status": "rejected",
                            "reason": "backpressure",
                            "sequence_reserved": false
                        });
                        if socket.send(Message::Text(backpressure.to_string().into())).await.is_err() {
                            break;
                        }
                        continue;
                    };
                    let service = service.clone();
                    tokio::spawn(async move {
                        let response = service.dispatch_acip_payload(&payload).await;
                        permit.send(response.to_string());
                    });
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
    tracing::warn!(
        error = %error,
        code = control_error_code(&error),
        "runtime control request rejected"
    );
    let status = match &error {
        ControlError::Authentication => StatusCode::UNAUTHORIZED,
        ControlError::Unauthorized => StatusCode::FORBIDDEN,
        ControlError::IdempotencyConflict
        | ControlError::InFlight
        | ControlError::LifecycleAlreadyRequested => StatusCode::CONFLICT,
        ControlError::AdmissionClosed
        | ControlError::Backpressure
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
        ControlError::Backpressure => "backpressure",
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
    #[error("control command admission is under backpressure")]
    Backpressure,
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
mod tests {
    use super::*;
    use crate::{
        AcipEnvelope, AdapterKind, AdapterPolicy, AuthorityMode, CommunicationVerifyingIdentity,
        ExecutorError, FailureClass, IngressSnapshot, OperationExecutor, OperationRequest,
        OperationalAdapter, OperationalFactory, ACIP_PROTOBUF_SCHEMA, ACIP_PROTOCOL_FAMILY,
        ACIP_VERSION_MAJOR, ACIP_VERSION_MINOR,
    };
    use prost::Message as _;

    struct NoopLifecycle;

    #[async_trait]
    impl LifecycleControl for NoopLifecycle {
        async fn shutdown(&self, _grace: Duration) -> Result<KernelExit, ()> {
            Ok(KernelExit::Clean)
        }
    }

    struct FatalExecutor;

    #[async_trait]
    impl OperationExecutor for FatalExecutor {
        async fn execute(&self, _request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
            Err(ExecutorError {
                class: FailureClass::Fatal,
                message: "injected post-dispatch failure".to_owned(),
            })
        }
    }

    #[test]
    fn restored_layer8_watermark_advances_when_qualified_time_moves_backward() {
        let recorder = RuntimeRecorder::new(4);
        let ingress = CanonicalIngress::new(1, recorder.clone(), BTreeMap::new());
        ingress.restore(IngressSnapshot {
            communication_sequences: BTreeMap::from([("layer8-operator".to_owned(), 9_876)]),
            ..IngressSnapshot::default()
        });
        let service = ControlService::new(
            "runtime-sequence-test",
            recorder,
            NoopLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            4,
        )
        .with_canonical_ingress(ingress);

        assert_eq!(service.allocate_layer8_sequence(100).unwrap(), 9_877);
        assert_eq!(service.allocate_layer8_sequence(99).unwrap(), 9_878);
    }

    #[test]
    fn layer8_sequence_exhaustion_fails_closed_without_reuse() {
        assert_eq!(next_layer8_sequence(MAX_INTEROPERABLE_SEQUENCE, 1), None);
    }

    #[tokio::test]
    async fn ambiguous_secure_dispatch_reports_retained_then_rejects_replay() {
        let signing_key = SigningKey::from_bytes(&[93; 32]);
        let now = now_unix_millis();
        let recorder = RuntimeRecorder::new(4);
        recorder.set_clock_authority(ClockAuthority::Authoritative {
            source: "test-qualified-clock".to_owned(),
            unix_millis: now,
        });
        recorder.set_component_state(ComponentId::new("agent_runtime"), RunningState::Running);
        let adapter = Arc::new(
            OperationalAdapter::new(
                AdapterKind::Agent,
                AdapterPolicy {
                    capacity: 1,
                    max_in_flight: 1,
                    shutdown_grace_millis: 1_000,
                    max_attempts: 1,
                    idempotency_entries: 4,
                    authority: AuthorityMode::Internal,
                },
                Arc::new(FatalExecutor),
            )
            .unwrap(),
        );
        let operation = OperationalFactory::new(adapter, Vec::new());
        let ingress = CanonicalIngress::new_with_communication_keys(
            1,
            recorder.clone(),
            BTreeMap::from([("agent".to_owned(), operation.clone())]),
            BTreeMap::from([(
                "agent-0001".to_owned(),
                CommunicationVerifyingIdentity {
                    signing_key_id: "agent-0001-key".to_owned(),
                    verifying_key: signing_key.verifying_key(),
                },
            )]),
        );
        let population = AgentPopulationFeed {
            total_count: 2,
            rendered_sample_count: 2,
            sample: ["agent-0001", "agent-0002"]
                .into_iter()
                .map(|id| AgentSample {
                    id: id.to_owned(),
                    label: id.to_owned(),
                    role: "agent".to_owned(),
                    state: "running".to_owned(),
                    detail: "Runtime component state: running".to_owned(),
                    signing_algorithm: "ed25519".to_owned(),
                    signing_key_id: format!("{id}-key"),
                    signing_public_key: hex::encode(signing_key.verifying_key().as_bytes()),
                })
                .collect(),
        };
        let service = ControlService::new_with_observatory_config_and_agents(
            "runtime-ambiguous-test",
            recorder,
            NoopLifecycle,
            ControlAuthority::new(BTreeMap::new()),
            4,
            std::iter::empty(),
            population,
        )
        .with_canonical_ingress(ingress.clone());
        let mut registry = crate::ComponentRegistry::new();
        registry.register(operation);
        registry.register(ingress);
        let _kernel = crate::Kernel::new(registry.validate().unwrap(), RuntimeRecorder::new(4))
            .start()
            .await
            .unwrap();
        let mut message = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: "request".to_owned(),
            sender_id: "agent-0001".to_owned(),
            recipient_id: "agent-0002".to_owned(),
            correlation_id: "ambiguous-correlation-000000001".to_owned(),
            causation_id: "ambiguous-causation-0000000001".to_owned(),
            monotonic_sequence: 1,
            issued_at_unix_millis: now,
            expires_at_unix_millis: now + 60_000,
            nonce: "ambiguous-message-0000000000001".to_owned(),
            content: "test ambiguous dispatch retention".to_owned(),
            signing_algorithm: "ed25519".to_owned(),
            signing_key_id: "agent-0001-key".to_owned(),
            signature: String::new(),
        };
        message.signature = hex::encode(
            signing_key
                .sign(&message.signing_bytes().unwrap())
                .to_bytes(),
        );
        let route = "agent";
        let payload = AcipEnvelope {
            schema: ACIP_PROTOBUF_SCHEMA.to_owned(),
            message_id: message.nonce.clone(),
            source: message.sender_id.clone(),
            target: message.recipient_id.clone(),
            route: route.to_owned(),
            payload_json: serde_jcs::to_string(&message).unwrap(),
            monotonic_sequence: message.monotonic_sequence,
            protocol_family: ACIP_PROTOCOL_FAMILY.to_owned(),
            version_major: ACIP_VERSION_MAJOR,
            version_minor: ACIP_VERSION_MINOR,
            runtime_id: "runtime-ambiguous-test".to_owned(),
            correlation_id: message.correlation_id.clone(),
            causation_id: message.causation_id.clone(),
            trace_id: message.correlation_id.clone(),
            replay_id: "agent-0001:1".to_owned(),
            capability: route.to_owned(),
            authority: "signed-communication-identity".to_owned(),
            payload_type: "application/json".to_owned(),
            acknowledgement_requested: true,
            error_code: None,
            required_features: Vec::new(),
        }
        .encode_to_vec();

        let failed = service.dispatch_acip_payload(&payload).await;
        assert_eq!(failed["status"], "rejected", "{failed}");
        assert_eq!(failed["sequence_reserved"], true, "{failed}");

        let replay = service.dispatch_acip_payload(&payload).await;
        assert_eq!(replay["status"], "rejected", "{replay}");
        assert_eq!(replay["reason"], "monotonic_sequence_must_advance");
        assert_eq!(replay["sequence_reserved"], false, "{replay}");
    }
}
