use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use prost::Message;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const ACIP_PROTOBUF_SCHEMA: &str = "adl.csm.acip_carrier.protobuf_envelope.v1";
pub const ACIP_WEBSOCKET_SCHEMA: &str = "adl.csm.acip_carrier.websocket_frame.v1";
pub const ACIP_PROTOCOL_FAMILY: &str = "adl-acip-a2a";
pub const ACIP_VERSION_MAJOR: u32 = 1;
pub const ACIP_VERSION_MINOR: u32 = 0;
pub const ACIP_MAX_PAYLOAD_BYTES: usize = 64 * 1024;
pub const ACIP_SUPPORTED_FEATURES: &[&str] = &[
    "acknowledgement",
    "authority-context",
    "causation",
    "correlation",
    "deterministic-json",
    "replay-identity",
    "trace-context",
];

#[derive(Clone, PartialEq, Message)]
pub struct AcipEnvelope {
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

pub fn encode_acip_envelope(
    message_id: &str,
    source: &str,
    target: &str,
    route: &str,
    payload: &Value,
    monotonic_sequence: u64,
) -> Result<Vec<u8>, String> {
    encode_acip_envelope_with_context(
        message_id,
        source,
        target,
        route,
        payload,
        monotonic_sequence,
        "local-runtime",
        message_id,
        message_id,
        message_id,
        "runtime-api-authenticated",
    )
}

#[allow(clippy::too_many_arguments)]
pub fn encode_acip_envelope_with_context(
    message_id: &str,
    source: &str,
    target: &str,
    route: &str,
    payload: &Value,
    monotonic_sequence: u64,
    runtime_id: &str,
    correlation_id: &str,
    causation_id: &str,
    trace_id: &str,
    authority: &str,
) -> Result<Vec<u8>, String> {
    let replay_id = format!("{source}:{monotonic_sequence}");
    let envelope = AcipEnvelope {
        schema: ACIP_PROTOBUF_SCHEMA.to_owned(),
        message_id: required(message_id, "message_id")?.to_owned(),
        source: required(source, "source")?.to_owned(),
        target: required(target, "target")?.to_owned(),
        route: required(route, "route")?.to_owned(),
        payload_json: serde_jcs::to_string(payload)
            .map_err(|error| format!("canonical JSON projection failed: {error}"))?,
        monotonic_sequence,
        protocol_family: ACIP_PROTOCOL_FAMILY.to_owned(),
        version_major: ACIP_VERSION_MAJOR,
        version_minor: ACIP_VERSION_MINOR,
        runtime_id: required(runtime_id, "runtime_id")?.to_owned(),
        correlation_id: required(correlation_id, "correlation_id")?.to_owned(),
        causation_id: required(causation_id, "causation_id")?.to_owned(),
        trace_id: required(trace_id, "trace_id")?.to_owned(),
        replay_id,
        capability: route.to_owned(),
        authority: required(authority, "authority")?.to_owned(),
        payload_type: "application/json".to_owned(),
        acknowledgement_requested: true,
        error_code: None,
        required_features: Vec::new(),
    };
    validate(&envelope)?;
    Ok(envelope.encode_to_vec())
}

pub fn decode_acip_envelope(bytes: &[u8]) -> Result<AcipEnvelope, String> {
    if bytes.is_empty() {
        return Err("protobuf envelope must not be empty".to_owned());
    }
    if bytes.len() > ACIP_MAX_PAYLOAD_BYTES {
        return Err("protobuf envelope exceeds ACIP payload limit".to_owned());
    }
    let mut envelope = AcipEnvelope::decode(bytes)
        .map_err(|error| format!("malformed protobuf envelope: {error}"))?;
    normalize_legacy_v1_envelope(&mut envelope);
    validate(&envelope)?;
    Ok(envelope)
}

fn normalize_legacy_v1_envelope(envelope: &mut AcipEnvelope) {
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

    envelope.protocol_family = ACIP_PROTOCOL_FAMILY.to_owned();
    envelope.version_major = ACIP_VERSION_MAJOR;
    envelope.version_minor = ACIP_VERSION_MINOR;
    envelope.runtime_id = "local-runtime".to_owned();
    envelope.correlation_id = envelope.message_id.clone();
    envelope.causation_id = envelope.message_id.clone();
    envelope.trace_id = envelope.message_id.clone();
    envelope.replay_id = format!("{}:{}", envelope.source, envelope.monotonic_sequence);
    envelope.capability = envelope.route.clone();
    envelope.authority = "runtime-api-authenticated".to_owned();
    envelope.payload_type = "application/json".to_owned();
    envelope.acknowledgement_requested = true;
}

pub fn acip_frame_status(bytes: &[u8]) -> Value {
    match decode_acip_envelope(bytes) {
        Ok(envelope) => json!({
            "schema": ACIP_WEBSOCKET_SCHEMA,
            "status": "accepted",
            "message_id": envelope.message_id,
            "payload_hash": STANDARD_NO_PAD.encode(Sha256::digest(envelope.payload_json.as_bytes())),
            "sequence_reserved": true
        }),
        Err(reason) => json!({
            "schema": ACIP_WEBSOCKET_SCHEMA,
            "status": "rejected",
            "reason": reason,
            "sequence_reserved": false
        }),
    }
}

fn validate(envelope: &AcipEnvelope) -> Result<(), String> {
    if envelope.schema != ACIP_PROTOBUF_SCHEMA {
        return Err(format!("schema must be {ACIP_PROTOBUF_SCHEMA}"));
    }
    if envelope.protocol_family != ACIP_PROTOCOL_FAMILY {
        return Err(format!("protocol_family must be {ACIP_PROTOCOL_FAMILY}"));
    }
    if envelope.version_major != ACIP_VERSION_MAJOR {
        return Err(format!(
            "unsupported protocol major {}",
            envelope.version_major
        ));
    }
    if envelope.version_minor > ACIP_VERSION_MINOR {
        return Err(format!(
            "unsupported protocol minor {}",
            envelope.version_minor
        ));
    }
    required(&envelope.message_id, "message_id")?;
    required(&envelope.source, "source")?;
    required(&envelope.target, "target")?;
    required(&envelope.route, "route")?;
    required(&envelope.runtime_id, "runtime_id")?;
    required(&envelope.correlation_id, "correlation_id")?;
    required(&envelope.causation_id, "causation_id")?;
    required(&envelope.trace_id, "trace_id")?;
    required(&envelope.replay_id, "replay_id")?;
    required(&envelope.capability, "capability")?;
    required(&envelope.authority, "authority")?;
    required(&envelope.payload_type, "payload_type")?;
    if envelope.replay_id != format!("{}:{}", envelope.source, envelope.monotonic_sequence) {
        return Err("replay_id must bind source and monotonic_sequence".to_owned());
    }
    if envelope.required_features.len() > 32
        || envelope
            .required_features
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
    {
        return Err("required_features must be bounded, sorted, and unique".to_owned());
    }
    if let Some(feature) = envelope
        .required_features
        .iter()
        .find(|feature| !ACIP_SUPPORTED_FEATURES.contains(&feature.as_str()))
    {
        return Err(format!("unsupported required feature {feature}"));
    }
    if envelope.payload_json.len() > ACIP_MAX_PAYLOAD_BYTES {
        return Err("payload_json exceeds ACIP payload limit".to_owned());
    }
    let payload = serde_json::from_str::<Value>(&envelope.payload_json)
        .map_err(|error| format!("payload_json must be valid JSON: {error}"))?;
    let canonical = serde_jcs::to_string(&payload)
        .map_err(|error| format!("canonical JSON projection failed: {error}"))?;
    if envelope.payload_json != canonical {
        return Err("payload_json must be canonical JCS JSON".to_owned());
    }
    Ok(())
}

fn required<'a>(value: &'a str, field: &str) -> Result<&'a str, String> {
    if value.trim().is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_seven_field_v1_envelope_is_normalized() {
        let legacy = AcipEnvelope {
            schema: ACIP_PROTOBUF_SCHEMA.to_owned(),
            message_id: "legacy-1".to_owned(),
            source: "agent-a".to_owned(),
            target: "agent-b".to_owned(),
            route: "invoke".to_owned(),
            payload_json: r#"{"value":1}"#.to_owned(),
            monotonic_sequence: 7,
            ..Default::default()
        };
        let decoded = decode_acip_envelope(&legacy.encode_to_vec()).expect("legacy v1");
        assert_eq!(decoded.protocol_family, ACIP_PROTOCOL_FAMILY);
        assert_eq!(decoded.version_major, ACIP_VERSION_MAJOR);
        assert_eq!(decoded.runtime_id, "local-runtime");
        assert_eq!(decoded.replay_id, "agent-a:7");
        assert_eq!(decoded.capability, "invoke");
        assert_eq!(decoded.authority, "runtime-api-authenticated");
    }
}
