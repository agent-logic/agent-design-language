//! Runtime-owned ACIP carrier contracts.
//!
//! ACIP is part of the CSM runtime communications plane. This module keeps the
//! carrier contract in `adl-runtime` so protobuf projection, WebSocket framing,
//! governance hooks, and fail-closed validation are available without depending
//! on the ADL compiler or C-SDLC control-plane crates.

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use prost::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const CSM_ACIP_COMPONENT: &str = "acip_carrier";
pub const CSM_ACIP_STATUS_SCHEMA: &str = "adl.csm.acip_carrier.status.v1";
pub const CSM_ACIP_CHANNELS_SCHEMA: &str = "adl.csm.acip_carrier.channels.v1";
pub const CSM_ACIP_PROTOBUF_SCHEMA: &str = "adl.csm.acip_carrier.protobuf_envelope.v1";
pub const CSM_ACIP_WEBSOCKET_SCHEMA: &str = "adl.csm.acip_carrier.websocket_frame.v1";
pub const CSM_ACIP_PROTOCOL_FAMILY: &str = "adl-acip-a2a";
pub const CSM_ACIP_VERSION_MAJOR: u32 = 1;
pub const CSM_ACIP_VERSION_MINOR: u32 = 0;
pub const CSM_ACIP_STATUS_REF: &str = "csm_acip_carrier_status.json";
pub const CSM_ACIP_MAX_PAYLOAD_BYTES: usize = 64 * 1024;
pub const CSM_ACIP_MAX_REQUIRED_FEATURES: usize = 32;

pub const CSM_ACIP_SUPPORTED_FEATURES: &[&str] = &[
    "acknowledgement",
    "authority-context",
    "causation",
    "correlation",
    "deterministic-json",
    "replay-identity",
    "trace-context",
];

#[derive(Clone, PartialEq, Message)]
pub struct AcipRuntimeEnvelopeProto {
    #[prost(string, tag = "1")]
    pub schema: String,
    #[prost(string, tag = "2")]
    pub message_id: String,
    #[prost(string, tag = "3")]
    pub source: String,
    #[prost(string, tag = "4")]
    pub target: String,
    #[prost(string, tag = "5")]
    pub route: String,
    #[prost(string, tag = "6")]
    pub payload_json: String,
    #[prost(uint64, tag = "7")]
    pub monotonic_sequence: u64,
    #[prost(string, tag = "8")]
    pub protocol_family: String,
    #[prost(uint32, tag = "9")]
    pub version_major: u32,
    #[prost(uint32, tag = "10")]
    pub version_minor: u32,
    #[prost(string, tag = "11")]
    pub runtime_id: String,
    #[prost(string, tag = "12")]
    pub correlation_id: String,
    #[prost(string, tag = "13")]
    pub causation_id: String,
    #[prost(string, tag = "14")]
    pub trace_id: String,
    #[prost(string, tag = "15")]
    pub replay_id: String,
    #[prost(string, tag = "16")]
    pub capability: String,
    #[prost(string, tag = "17")]
    pub authority: String,
    #[prost(string, tag = "18")]
    pub payload_type: String,
    #[prost(bool, tag = "19")]
    pub acknowledgement_requested: bool,
    #[prost(string, optional, tag = "20")]
    pub error_code: Option<String>,
    #[prost(string, repeated, tag = "21")]
    pub required_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcipJsonProjection {
    pub acknowledgement_requested: bool,
    pub authority: String,
    pub capability: String,
    pub causation_id: String,
    pub correlation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    pub message_id: String,
    pub monotonic_sequence: String,
    pub payload_json: String,
    pub payload_type: String,
    pub protocol_family: String,
    pub replay_id: String,
    pub required_features: Vec<String>,
    pub route: String,
    pub runtime_id: String,
    pub schema: String,
    pub source: String,
    pub target: String,
    pub trace_id: String,
    pub version_major: u32,
    pub version_minor: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcipEnvelopeInput<'a> {
    pub message_id: &'a str,
    pub source: &'a str,
    pub target: &'a str,
    pub route: &'a str,
    pub runtime_id: &'a str,
    pub correlation_id: &'a str,
    pub causation_id: &'a str,
    pub trace_id: &'a str,
    pub replay_id: &'a str,
    pub capability: &'a str,
    pub authority: &'a str,
    pub payload_type: &'a str,
    pub monotonic_sequence: u64,
    pub acknowledgement_requested: bool,
    pub error_code: Option<&'a str>,
    pub required_features: &'a [&'a str],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcipNegotiationOffer {
    pub protocol_family: String,
    pub supported_major: u32,
    pub minimum_minor: u32,
    pub maximum_minor: u32,
    pub required_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcipNegotiatedVersion {
    pub protocol_family: String,
    pub version_major: u32,
    pub version_minor: u32,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipCarrierChannels {
    pub schema: String,
    pub ingress: String,
    pub egress: String,
    pub websocket_frames: String,
    pub protobuf_projection: String,
    pub checkpoint: String,
    pub lifelog: String,
    pub observability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipGovernanceHooks {
    pub runtime_api_auth_required: bool,
    pub freedom_gate_required: bool,
    pub cav_required: bool,
    pub constructability_required: bool,
    pub malformed_input_policy: String,
    pub unauthorized_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipProjectionProfile {
    pub json_projection: String,
    pub protobuf_crate: String,
    pub protobuf_schema: String,
    pub websocket_schema: String,
    pub deterministic_projection: String,
    pub future_read_guarantee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmAcipCarrierStatus {
    pub schema: String,
    pub runtime_owner: String,
    pub component: String,
    pub status: String,
    pub readiness: String,
    pub process_model: String,
    pub runtime_api_path: String,
    pub websocket_path: String,
    pub channels: CsmAcipCarrierChannels,
    pub governance_hooks: CsmAcipGovernanceHooks,
    pub projection_profile: CsmAcipProjectionProfile,
    pub retained_status_ref: String,
}

impl CsmAcipCarrierChannels {
    pub fn new() -> Self {
        Self {
            schema: CSM_ACIP_CHANNELS_SCHEMA.to_string(),
            ingress: "csm.acip_carrier.ingress".to_string(),
            egress: "csm.acip_carrier.egress".to_string(),
            websocket_frames: "csm.acip_carrier.websocket_frames".to_string(),
            protobuf_projection: "csm.acip_carrier.protobuf_projection".to_string(),
            checkpoint: "csm.checkpoint.acip_carrier".to_string(),
            lifelog: "csm.lifelog.acip_carrier".to_string(),
            observability: "csm.observability.acip_carrier".to_string(),
        }
    }
}

impl Default for CsmAcipCarrierChannels {
    fn default() -> Self {
        Self::new()
    }
}

impl CsmAcipGovernanceHooks {
    pub fn required() -> Self {
        Self {
            runtime_api_auth_required: true,
            freedom_gate_required: true,
            cav_required: true,
            constructability_required: true,
            malformed_input_policy: "fail_closed_retain_rejection".to_string(),
            unauthorized_policy: "runtime_api_auth_denied_before_sequence_reservation".to_string(),
        }
    }
}

impl CsmAcipProjectionProfile {
    pub fn runtime_default() -> Self {
        Self {
            json_projection: "canonical_serde_jcs_payload_projection".to_string(),
            protobuf_crate: "prost".to_string(),
            protobuf_schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
            websocket_schema: CSM_ACIP_WEBSOCKET_SCHEMA.to_string(),
            deterministic_projection: "sha256_over_jcs_payload_then_prost_envelope".to_string(),
            future_read_guarantee:
                "schema_versioned_envelope_fields_are_append_only_for_v0_91_7_to_v0_92".to_string(),
        }
    }
}

impl CsmAcipCarrierStatus {
    pub fn runtime_default() -> Self {
        Self {
            schema: CSM_ACIP_STATUS_SCHEMA.to_string(),
            runtime_owner: "csm".to_string(),
            component: CSM_ACIP_COMPONENT.to_string(),
            status: "available".to_string(),
            readiness: "ready".to_string(),
            process_model: "embedded_csm_runtime_component".to_string(),
            runtime_api_path: "/acip".to_string(),
            websocket_path: "/v1/acip/ws".to_string(),
            channels: CsmAcipCarrierChannels::new(),
            governance_hooks: CsmAcipGovernanceHooks::required(),
            projection_profile: CsmAcipProjectionProfile::runtime_default(),
            retained_status_ref: CSM_ACIP_STATUS_REF.to_string(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, CSM_ACIP_STATUS_SCHEMA, "schema")?;
        require_exact(&self.runtime_owner, "csm", "runtime_owner")?;
        require_exact(&self.component, CSM_ACIP_COMPONENT, "component")?;
        require_exact(
            &self.process_model,
            "embedded_csm_runtime_component",
            "process_model",
        )?;
        require_exact(&self.runtime_api_path, "/acip", "runtime_api_path")?;
        require_exact(&self.websocket_path, "/v1/acip/ws", "websocket_path")?;
        require_exact(
            &self.channels.schema,
            CSM_ACIP_CHANNELS_SCHEMA,
            "channels.schema",
        )?;
        require_exact(
            &self.projection_profile.protobuf_crate,
            "prost",
            "projection_profile.protobuf_crate",
        )?;
        require_exact(
            &self.projection_profile.protobuf_schema,
            CSM_ACIP_PROTOBUF_SCHEMA,
            "projection_profile.protobuf_schema",
        )?;
        require_exact(
            &self.projection_profile.websocket_schema,
            CSM_ACIP_WEBSOCKET_SCHEMA,
            "projection_profile.websocket_schema",
        )?;
        require_exact(
            &self.retained_status_ref,
            CSM_ACIP_STATUS_REF,
            "retained_status_ref",
        )?;
        if !self.governance_hooks.runtime_api_auth_required
            || !self.governance_hooks.freedom_gate_required
            || !self.governance_hooks.cav_required
            || !self.governance_hooks.constructability_required
            || self.governance_hooks.malformed_input_policy != "fail_closed_retain_rejection"
        {
            return Err(
                "ACIP carrier must require runtime API auth, Freedom Gate, CAV, Constructability, and fail-closed malformed input"
                    .to_string(),
            );
        }
        if self.readiness != "ready" {
            return Err("ACIP carrier readiness must be ready for admission".to_string());
        }
        Ok(())
    }
}

pub fn runtime_capability() -> Value {
    json!({
        "status": "integrated",
        "component": CSM_ACIP_COMPONENT,
        "component_class": "embedded_csm_runtime_component",
        "process_model": "in_process_no_sidecar_no_separate_binary",
        "runtime_api_path": "/acip",
        "websocket_path": "/v1/acip/ws",
        "protobuf_crate": "prost",
        "channels": CsmAcipCarrierChannels::new(),
        "governance_hooks": CsmAcipGovernanceHooks::required(),
        "projection_profile": CsmAcipProjectionProfile::runtime_default(),
        "retained_status_ref": CSM_ACIP_STATUS_REF,
        "non_claims": [
            "does_not_claim_external_inter_polis_federation",
            "does_not_open_a_new_port",
            "does_not_bypass_runtime_api_auth"
        ]
    })
}

pub fn api_status(agent_instance_id: &str, artifact: &Value, runtime_capability: Value) -> Value {
    let default_status = CsmAcipCarrierStatus::runtime_default();
    let artifact_status = artifact
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("missing");
    let candidate = artifact
        .get("value")
        .and_then(|value| serde_json::from_value::<CsmAcipCarrierStatus>(value.clone()).ok())
        .unwrap_or_else(|| default_status.clone());
    let validation = candidate
        .validate()
        .map(|_| json!({"status": "passed"}))
        .unwrap_or_else(|reason| json!({"status": "fail_closed", "reason": reason}));
    json!({
        "schema": CSM_ACIP_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": CSM_ACIP_COMPONENT,
        "agent_instance_id": agent_instance_id,
        "status": if validation["status"] == "passed" { candidate.status.as_str() } else { "blocked" },
        "readiness": if validation["status"] == "passed" { candidate.readiness.as_str() } else { "blocked" },
        "runtime_capability": runtime_capability,
        "value": candidate,
        "validation": validation,
        "retained_artifact_status": artifact_status,
        "evidence_source": if artifact_status == "serialized" { "retained_artifact" } else { "computed_runtime_contract" }
    })
}

pub fn encode_protobuf_envelope(
    message_id: &str,
    source: &str,
    target: &str,
    route: &str,
    payload: &Value,
    monotonic_sequence: u64,
) -> Result<Vec<u8>, String> {
    let replay_id = format!("{source}:{monotonic_sequence}");
    encode_semantic_envelope(
        AcipEnvelopeInput {
            message_id,
            source,
            target,
            route,
            runtime_id: "local-runtime",
            correlation_id: message_id,
            causation_id: message_id,
            trace_id: message_id,
            replay_id: &replay_id,
            capability: route,
            authority: "runtime-api-authenticated",
            payload_type: "application/json",
            monotonic_sequence,
            acknowledgement_requested: true,
            error_code: None,
            required_features: &[],
        },
        payload,
    )
}

pub fn encode_semantic_envelope(
    input: AcipEnvelopeInput<'_>,
    payload: &Value,
) -> Result<Vec<u8>, String> {
    let payload_json = deterministic_payload_json(payload)?;
    let envelope = AcipRuntimeEnvelopeProto {
        schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
        message_id: require_string(input.message_id, "message_id")?.to_string(),
        source: require_string(input.source, "source")?.to_string(),
        target: require_string(input.target, "target")?.to_string(),
        route: require_string(input.route, "route")?.to_string(),
        payload_json,
        monotonic_sequence: input.monotonic_sequence,
        protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
        version_major: CSM_ACIP_VERSION_MAJOR,
        version_minor: CSM_ACIP_VERSION_MINOR,
        runtime_id: require_string(input.runtime_id, "runtime_id")?.to_string(),
        correlation_id: require_string(input.correlation_id, "correlation_id")?.to_string(),
        causation_id: require_string(input.causation_id, "causation_id")?.to_string(),
        trace_id: require_string(input.trace_id, "trace_id")?.to_string(),
        replay_id: require_string(input.replay_id, "replay_id")?.to_string(),
        capability: require_string(input.capability, "capability")?.to_string(),
        authority: require_string(input.authority, "authority")?.to_string(),
        payload_type: require_string(input.payload_type, "payload_type")?.to_string(),
        acknowledgement_requested: input.acknowledgement_requested,
        error_code: input.error_code.map(str::to_owned),
        required_features: input
            .required_features
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    };
    validate_envelope(&envelope)?;
    Ok(envelope.encode_to_vec())
}

pub fn decode_protobuf_envelope(bytes: &[u8]) -> Result<AcipRuntimeEnvelopeProto, String> {
    if bytes.is_empty() {
        return Err("protobuf envelope must not be empty".to_string());
    }
    if bytes.len() > CSM_ACIP_MAX_PAYLOAD_BYTES {
        return Err("protobuf envelope exceeds CSM ACIP payload limit".to_string());
    }
    let mut envelope = AcipRuntimeEnvelopeProto::decode(bytes)
        .map_err(|err| format!("malformed protobuf envelope: {err}"))?;
    normalize_legacy_v1_envelope(&mut envelope);
    validate_envelope(&envelope)?;
    Ok(envelope)
}

fn normalize_legacy_v1_envelope(envelope: &mut AcipRuntimeEnvelopeProto) {
    let is_legacy = envelope.protocol_family.is_empty()
        && envelope.version_major == 0
        && envelope.version_minor == 0
        && envelope.runtime_id.is_empty()
        && envelope.correlation_id.is_empty()
        && envelope.causation_id.is_empty()
        && envelope.trace_id.is_empty()
        && envelope.replay_id.is_empty()
        && envelope.capability.is_empty()
        && envelope.authority.is_empty()
        && envelope.payload_type.is_empty()
        && !envelope.acknowledgement_requested
        && envelope.error_code.is_none()
        && envelope.required_features.is_empty();
    if !is_legacy {
        return;
    }

    envelope.protocol_family = CSM_ACIP_PROTOCOL_FAMILY.to_string();
    envelope.version_major = CSM_ACIP_VERSION_MAJOR;
    envelope.version_minor = CSM_ACIP_VERSION_MINOR;
    envelope.runtime_id = "local-runtime".to_string();
    envelope.correlation_id = envelope.message_id.clone();
    envelope.causation_id = envelope.message_id.clone();
    envelope.trace_id = envelope.message_id.clone();
    envelope.replay_id = format!("{}:{}", envelope.source, envelope.monotonic_sequence);
    envelope.capability = envelope.route.clone();
    envelope.authority = "runtime-api-authenticated".to_string();
    envelope.payload_type = "application/json".to_string();
    envelope.acknowledgement_requested = true;
}

pub fn protobuf_to_deterministic_json(bytes: &[u8]) -> Result<String, String> {
    let envelope = decode_protobuf_envelope(bytes)?;
    serde_jcs::to_string(&AcipJsonProjection::from(&envelope))
        .map_err(|error| format!("canonical JSON projection failed: {error}"))
}

pub fn deterministic_json_to_protobuf(json: &str) -> Result<Vec<u8>, String> {
    let projection: AcipJsonProjection = serde_json::from_str(json)
        .map_err(|error| format!("invalid deterministic JSON projection: {error}"))?;
    let canonical = serde_jcs::to_string(&projection)
        .map_err(|error| format!("canonical JSON projection failed: {error}"))?;
    if canonical != json {
        return Err(
            "JSON projection must use canonical JCS ordering and representation".to_string(),
        );
    }
    let envelope = AcipRuntimeEnvelopeProto::try_from(projection)?;
    validate_envelope(&envelope)?;
    Ok(envelope.encode_to_vec())
}

pub fn negotiate_version(offer: &AcipNegotiationOffer) -> Result<AcipNegotiatedVersion, String> {
    require_exact(
        &offer.protocol_family,
        CSM_ACIP_PROTOCOL_FAMILY,
        "protocol_family",
    )?;
    if offer.supported_major != CSM_ACIP_VERSION_MAJOR {
        return Err(format!(
            "unsupported protocol major {}; supported major is {}",
            offer.supported_major, CSM_ACIP_VERSION_MAJOR
        ));
    }
    if offer.minimum_minor > offer.maximum_minor
        || !(offer.minimum_minor..=offer.maximum_minor).contains(&CSM_ACIP_VERSION_MINOR)
    {
        return Err("no compatible protocol minor version".to_string());
    }
    let mut features = offer.required_features.clone();
    features.sort();
    features.dedup();
    validate_features(&features)?;
    Ok(AcipNegotiatedVersion {
        protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
        version_major: CSM_ACIP_VERSION_MAJOR,
        version_minor: CSM_ACIP_VERSION_MINOR,
        features,
    })
}

impl From<&AcipRuntimeEnvelopeProto> for AcipJsonProjection {
    fn from(envelope: &AcipRuntimeEnvelopeProto) -> Self {
        Self {
            acknowledgement_requested: envelope.acknowledgement_requested,
            authority: envelope.authority.clone(),
            capability: envelope.capability.clone(),
            causation_id: envelope.causation_id.clone(),
            correlation_id: envelope.correlation_id.clone(),
            error_code: envelope.error_code.clone(),
            message_id: envelope.message_id.clone(),
            monotonic_sequence: envelope.monotonic_sequence.to_string(),
            payload_json: envelope.payload_json.clone(),
            payload_type: envelope.payload_type.clone(),
            protocol_family: envelope.protocol_family.clone(),
            replay_id: envelope.replay_id.clone(),
            required_features: envelope.required_features.clone(),
            route: envelope.route.clone(),
            runtime_id: envelope.runtime_id.clone(),
            schema: envelope.schema.clone(),
            source: envelope.source.clone(),
            target: envelope.target.clone(),
            trace_id: envelope.trace_id.clone(),
            version_major: envelope.version_major,
            version_minor: envelope.version_minor,
        }
    }
}

impl TryFrom<AcipJsonProjection> for AcipRuntimeEnvelopeProto {
    type Error = String;

    fn try_from(value: AcipJsonProjection) -> Result<Self, Self::Error> {
        let sequence = value.monotonic_sequence.as_bytes();
        let canonical_decimal = value.monotonic_sequence == "0"
            || (sequence
                .first()
                .is_some_and(|digit| matches!(digit, b'1'..=b'9'))
                && sequence.iter().all(u8::is_ascii_digit));
        if !canonical_decimal {
            return Err(
                "monotonic_sequence must be a canonical unsigned decimal string".to_string(),
            );
        }
        let monotonic_sequence = value.monotonic_sequence.parse::<u64>().map_err(|_| {
            "monotonic_sequence must be a canonical unsigned decimal string".to_string()
        })?;
        Ok(Self {
            schema: value.schema,
            message_id: value.message_id,
            source: value.source,
            target: value.target,
            route: value.route,
            payload_json: value.payload_json,
            monotonic_sequence,
            protocol_family: value.protocol_family,
            version_major: value.version_major,
            version_minor: value.version_minor,
            runtime_id: value.runtime_id,
            correlation_id: value.correlation_id,
            causation_id: value.causation_id,
            trace_id: value.trace_id,
            replay_id: value.replay_id,
            capability: value.capability,
            authority: value.authority,
            payload_type: value.payload_type,
            acknowledgement_requested: value.acknowledgement_requested,
            error_code: value.error_code,
            required_features: value.required_features,
        })
    }
}

pub fn websocket_frame_status(bytes: &[u8], authorized: bool) -> Value {
    if !authorized {
        return json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": "runtime_api_auth_required",
            "sequence_reserved": false
        });
    }
    match decode_protobuf_envelope(bytes) {
        Ok(envelope) => json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "accepted",
            "message_id": envelope.message_id,
            "payload_hash": payload_hash(&envelope.payload_json),
            "sequence_reserved": true
        }),
        Err(reason) => json!({
            "schema": CSM_ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": reason,
            "sequence_reserved": false
        }),
    }
}

fn validate_envelope(envelope: &AcipRuntimeEnvelopeProto) -> Result<(), String> {
    require_exact(&envelope.schema, CSM_ACIP_PROTOBUF_SCHEMA, "schema")?;
    require_exact(
        &envelope.protocol_family,
        CSM_ACIP_PROTOCOL_FAMILY,
        "protocol_family",
    )?;
    if envelope.version_major != CSM_ACIP_VERSION_MAJOR {
        return Err(format!(
            "unsupported protocol major {}",
            envelope.version_major
        ));
    }
    if envelope.version_minor > CSM_ACIP_VERSION_MINOR {
        return Err(format!(
            "unsupported protocol minor {}",
            envelope.version_minor
        ));
    }
    require_string(&envelope.message_id, "message_id")?;
    require_string(&envelope.source, "source")?;
    require_string(&envelope.target, "target")?;
    require_string(&envelope.route, "route")?;
    require_string(&envelope.runtime_id, "runtime_id")?;
    require_string(&envelope.correlation_id, "correlation_id")?;
    require_string(&envelope.causation_id, "causation_id")?;
    require_string(&envelope.trace_id, "trace_id")?;
    require_string(&envelope.replay_id, "replay_id")?;
    require_string(&envelope.capability, "capability")?;
    require_string(&envelope.authority, "authority")?;
    require_string(&envelope.payload_type, "payload_type")?;
    validate_features(&envelope.required_features)?;
    if envelope
        .error_code
        .as_deref()
        .is_some_and(|code| code.trim().is_empty())
    {
        return Err("error_code must be omitted or non-empty".to_string());
    }
    if envelope.payload_json.len() > CSM_ACIP_MAX_PAYLOAD_BYTES {
        return Err("payload_json exceeds CSM ACIP payload limit".to_string());
    }
    let parsed = serde_json::from_str::<Value>(&envelope.payload_json)
        .map_err(|err| format!("payload_json must be valid JSON: {err}"))?;
    let canonical = deterministic_payload_json(&parsed)?;
    if envelope.payload_json != canonical {
        return Err(
            "payload_json must be canonical JCS JSON before protobuf envelope admission"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_features(features: &[String]) -> Result<(), String> {
    if features.len() > CSM_ACIP_MAX_REQUIRED_FEATURES {
        return Err("required_features exceeds declared limit".to_string());
    }
    if features.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("required_features must be sorted and unique".to_string());
    }
    if let Some(unsupported) = features
        .iter()
        .find(|feature| !CSM_ACIP_SUPPORTED_FEATURES.contains(&feature.as_str()))
    {
        return Err(format!("unsupported required feature {unsupported}"));
    }
    Ok(())
}

fn deterministic_payload_json(payload: &Value) -> Result<String, String> {
    serde_jcs::to_string(payload).map_err(|err| format!("canonical JSON projection failed: {err}"))
}

fn payload_hash(payload_json: &str) -> String {
    let digest = Sha256::digest(payload_json.as_bytes());
    STANDARD_NO_PAD.encode(digest)
}

fn require_string<'a>(value: &'a str, field: &str) -> Result<&'a str, String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(value)
}

fn require_exact(value: &str, expected: &str, field: &str) -> Result<(), String> {
    if value != expected {
        return Err(format!("{field} must be {expected}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acip_status_requires_embedded_governed_runtime_component() {
        CsmAcipCarrierStatus::runtime_default()
            .validate()
            .expect("default ACIP carrier is valid");
    }

    #[test]
    fn acip_status_rejects_missing_cav_gate() {
        let mut status = CsmAcipCarrierStatus::runtime_default();
        status.governance_hooks.cav_required = false;
        assert!(status
            .validate()
            .expect_err("cav gate required")
            .contains("CAV"));
    }

    #[test]
    fn protobuf_projection_round_trips_with_deterministic_json() {
        let first = json!({"z": 1, "a": {"b": true}});
        let second = json!({"a": {"b": true}, "z": 1});
        let left = encode_protobuf_envelope("m-1", "agent-a", "agent-b", "invoke", &first, 7)
            .expect("encode first");
        let right = encode_protobuf_envelope("m-1", "agent-a", "agent-b", "invoke", &second, 7)
            .expect("encode second");
        assert_eq!(left, right);
        let decoded = decode_protobuf_envelope(&left).expect("decode envelope");
        assert_eq!(decoded.schema, CSM_ACIP_PROTOBUF_SCHEMA);
        assert_eq!(decoded.payload_json, r#"{"a":{"b":true},"z":1}"#);
        let projected = protobuf_to_deterministic_json(&left).expect("project JSON");
        let restored = deterministic_json_to_protobuf(&projected).expect("restore protobuf");
        assert_eq!(restored, left);
    }

    #[test]
    fn legacy_seven_field_v1_envelope_is_normalized() {
        let legacy = AcipRuntimeEnvelopeProto {
            schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
            message_id: "legacy-1".to_string(),
            source: "agent-a".to_string(),
            target: "agent-b".to_string(),
            route: "invoke".to_string(),
            payload_json: r#"{"value":1}"#.to_string(),
            monotonic_sequence: 7,
            ..Default::default()
        };
        let decoded = decode_protobuf_envelope(&legacy.encode_to_vec()).expect("legacy v1");
        assert_eq!(decoded.protocol_family, CSM_ACIP_PROTOCOL_FAMILY);
        assert_eq!(decoded.version_major, CSM_ACIP_VERSION_MAJOR);
        assert_eq!(decoded.runtime_id, "local-runtime");
        assert_eq!(decoded.replay_id, "agent-a:7");
        assert_eq!(decoded.capability, "invoke");
        assert_eq!(decoded.authority, "runtime-api-authenticated");
    }

    #[test]
    fn malformed_or_unauthorized_websocket_frames_fail_closed() {
        let unauthorized = websocket_frame_status(b"not-protobuf", false);
        assert_eq!(unauthorized["status"], "rejected");
        assert_eq!(unauthorized["sequence_reserved"], false);
        let malformed = websocket_frame_status(b"not-protobuf", true);
        assert_eq!(malformed["status"], "rejected");
        assert_eq!(malformed["sequence_reserved"], false);
    }

    #[test]
    fn protobuf_decode_rejects_noncanonical_payload_json() {
        let envelope = AcipRuntimeEnvelopeProto {
            schema: CSM_ACIP_PROTOBUF_SCHEMA.to_string(),
            message_id: "m-1".to_string(),
            source: "agent-a".to_string(),
            target: "agent-b".to_string(),
            route: "invoke".to_string(),
            payload_json: r#"{"z":1,"a":{"b":true}}"#.to_string(),
            monotonic_sequence: 1,
            protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
            version_major: CSM_ACIP_VERSION_MAJOR,
            version_minor: CSM_ACIP_VERSION_MINOR,
            runtime_id: "runtime-a".to_string(),
            correlation_id: "corr-1".to_string(),
            causation_id: "cause-1".to_string(),
            trace_id: "trace-1".to_string(),
            replay_id: "replay-1".to_string(),
            capability: "invoke".to_string(),
            authority: "runtime-api-authenticated".to_string(),
            payload_type: "application/json".to_string(),
            acknowledgement_requested: true,
            error_code: None,
            required_features: vec![],
        };
        let err = decode_protobuf_envelope(&envelope.encode_to_vec())
            .expect_err("noncanonical payload must fail closed");
        assert!(err.contains("canonical JCS JSON"));
    }

    #[test]
    fn negotiation_rejects_unknown_major_and_required_feature() {
        let accepted = negotiate_version(&AcipNegotiationOffer {
            protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
            supported_major: 1,
            minimum_minor: 0,
            maximum_minor: 0,
            required_features: vec!["correlation".to_string(), "trace-context".to_string()],
        })
        .expect("compatible offer");
        assert_eq!(accepted.version_major, 1);

        let unsupported_major = negotiate_version(&AcipNegotiationOffer {
            supported_major: 2,
            ..AcipNegotiationOffer {
                protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
                supported_major: 1,
                minimum_minor: 0,
                maximum_minor: 0,
                required_features: vec![],
            }
        })
        .expect_err("unsupported major");
        assert!(unsupported_major.contains("unsupported protocol major"));

        let unsupported_feature = negotiate_version(&AcipNegotiationOffer {
            protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
            supported_major: 1,
            minimum_minor: 0,
            maximum_minor: 0,
            required_features: vec!["future-required-field".to_string()],
        })
        .expect_err("unknown required feature");
        assert!(unsupported_feature.contains("unsupported required feature"));
    }

    #[test]
    fn deterministic_json_rejects_unknown_fields_and_numeric_u64() {
        let bytes = encode_protobuf_envelope(
            "m-1",
            "agent-a",
            "agent-b",
            "invoke",
            &json!({"value": 1}),
            u64::MAX,
        )
        .expect("encode");
        let json = protobuf_to_deterministic_json(&bytes).expect("project");
        assert!(json.contains(&format!(r#""monotonic_sequence":"{}""#, u64::MAX)));
        let with_unknown = json.replacen('{', r#"{"unknown":true,"#, 1);
        assert!(deterministic_json_to_protobuf(&with_unknown)
            .expect_err("unknown field")
            .contains("unknown field"));

        for noncanonical in ["01", "+1", " 1", "1 "] {
            let candidate = json.replacen(
                &format!(r#""monotonic_sequence":"{}""#, u64::MAX),
                &format!(r#""monotonic_sequence":"{noncanonical}""#),
                1,
            );
            assert!(deterministic_json_to_protobuf(&candidate)
                .expect_err("noncanonical unsigned decimal")
                .contains("canonical unsigned decimal"));
        }
    }
}
