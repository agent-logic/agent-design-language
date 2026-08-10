//! Embedded CSM runtime API contracts.

use std::{collections::BTreeSet, net::SocketAddr};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const CSM_RUNTIME_API_SCHEMA: &str = "adl.csm.runtime_api.v1";
pub const CSM_RUNTIME_API_STATUS_SCHEMA: &str = "adl.csm.runtime_api.status.v1";
pub const CSM_RUNTIME_API_HEALTH_SCHEMA: &str = "adl.csm.runtime_api.health.v1";
pub const CSM_RUNTIME_API_READY_SCHEMA: &str = "adl.csm.runtime_api.ready.v1";
pub const CSM_RUNTIME_API_METRICS_SCHEMA: &str = "adl.csm.runtime_api.metrics.v1";
pub const CSM_RUNTIME_API_EVENTS_SCHEMA: &str = "adl.csm.runtime_api.events.v1";
pub const CSM_RUNTIME_API_CHRONOSENSE_SCHEMA: &str = "adl.csm.runtime_api.chronosense.v1";
pub const CSM_RUNTIME_API_SHEPHERD_SCHEMA: &str = "adl.csm.runtime_api.shepherd.v1";
pub const CSM_RUNTIME_API_CAV_SCHEMA: &str = "adl.csm.runtime_api.cav.v1";
pub const CSM_RUNTIME_API_CURIOSITY_SCHEMA: &str = "adl.csm.runtime_api.curiosity.v1";
pub const CSM_RUNTIME_API_ACIP_SCHEMA: &str = "adl.csm.runtime_api.acip.v1";
pub const CSM_RUNTIME_API_FREEDOM_GATE_SCHEMA: &str = "adl.csm.runtime_api.freedom_gate.v1";
pub const CSM_RUNTIME_API_REASONING_SCHEMA: &str = "adl.csm.runtime_api.reasoning.v1";
pub const CSM_RUNTIME_API_WEATHER_SCHEMA: &str = "adl.csm.runtime_api.weather.v1";
pub const CSM_RUNTIME_API_API_GATEWAY_BRIDGE_SCHEMA: &str =
    "adl.csm.runtime_api.api_gateway_bridge.v1";
pub const CSM_RUNTIME_API_CONSTRUCTABILITY_SCHEMA: &str = "adl.csm.runtime_api.constructability.v1";
pub const CSM_RUNTIME_API_PERSISTENCE_SCHEMA: &str = "adl.csm.runtime_api.persistence.v1";
pub const CSM_RUNTIME_API_WSS_AUTH_SCHEMA: &str = "adl.csm.runtime_api.wss_auth.v1";
pub const CSM_RUNTIME_API_WSS_SESSION_SCHEMA: &str = "adl.csm.runtime_api.wss_session.v1";
pub const CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA: &str = "adl.csm.runtime_api.feature_matrix.v1";
pub const CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA: &str = "adl.csm.runtime_api.telemetry_event.v1";
pub const CSM_RUNTIME_API_DEFAULT_PORT: u16 = 20_997;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeApiHealthState {
    Unimplemented,
    Unavailable,
    Failed,
    Healthy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiCapabilityHealth {
    pub capability: String,
    pub state: RuntimeApiHealthState,
    pub reason_code: String,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiHealthReport {
    pub schema: String,
    pub runtime_owner: String,
    pub capabilities: Vec<RuntimeApiCapabilityHealth>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiTelemetrySink {
    pub sink: String,
    pub supported_fields: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiTelemetryConfig {
    pub schema: String,
    pub sinks: Vec<RuntimeApiTelemetrySink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiFeatureMatrixRow {
    pub feature: String,
    pub adapter: String,
    pub claimed: bool,
    pub health_state: RuntimeApiHealthState,
    pub proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeApiFeatureMatrix {
    pub schema: String,
    pub unresolved_claimed_features: Vec<String>,
    pub rows: Vec<RuntimeApiFeatureMatrixRow>,
}

pub fn runtime_api_health_report(
    capabilities: Vec<RuntimeApiCapabilityHealth>,
) -> RuntimeApiHealthReport {
    RuntimeApiHealthReport {
        schema: CSM_RUNTIME_API_HEALTH_SCHEMA.to_string(),
        runtime_owner: crate::CSM_RUNTIME_OWNER.to_string(),
        capabilities,
    }
}

pub fn runtime_api_telemetry_event(
    config: &RuntimeApiTelemetryConfig,
    sink: &str,
    payload: &Value,
) -> Result<Value, String> {
    let capability = config
        .sinks
        .iter()
        .find(|candidate| candidate.sink == sink)
        .ok_or_else(|| "telemetry_sink_unavailable".to_string())?;
    let object = payload
        .as_object()
        .ok_or_else(|| "telemetry_payload_must_be_object".to_string())?;
    let fields = object
        .iter()
        .filter(|(key, _)| capability.supported_fields.contains(*key))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<serde_json::Map<_, _>>();
    Ok(json!({
        "schema": CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA,
        "sink": sink,
        "payload": fields,
        "dropped_unsupported_fields": object.len().saturating_sub(fields.len())
    }))
}

pub fn runtime_api_feature_matrix(
    rows: Vec<RuntimeApiFeatureMatrixRow>,
) -> RuntimeApiFeatureMatrix {
    let unresolved_claimed_features = rows
        .iter()
        .filter(|row| row.claimed && row.health_state != RuntimeApiHealthState::Healthy)
        .map(|row| row.feature.clone())
        .collect();
    RuntimeApiFeatureMatrix {
        schema: CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA.to_string(),
        unresolved_claimed_features,
        rows,
    }
}

pub fn configured_runtime_api_socket() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], CSM_RUNTIME_API_DEFAULT_PORT))
}

pub fn persistence_health(
    checkpoint: crate::continuity_history::DomainHealth,
    lifelog: crate::continuity_history::DomainHealth,
) -> serde_json::Value {
    serde_json::json!({
        "schema": CSM_RUNTIME_API_PERSISTENCE_SCHEMA,
        "checkpoint_continuity": checkpoint,
        "autobiographical_lifelog": lifelog,
        "restore_authority": "checkpoint_continuity_only",
        "failure_isolation": "independent_stores_and_lifecycle"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_health() -> RuntimeApiHealthReport {
        runtime_api_health_report(vec![RuntimeApiCapabilityHealth {
            capability: "runtime_api".to_string(),
            state: RuntimeApiHealthState::Healthy,
            reason_code: "unit_test".to_string(),
            evidence_ref: "adl-runtime/src/runtime_api.rs".to_string(),
        }])
    }

    fn test_telemetry() -> RuntimeApiTelemetryConfig {
        RuntimeApiTelemetryConfig {
            schema: "adl.csm.runtime_api.telemetry_config.v1".to_string(),
            sinks: vec![RuntimeApiTelemetrySink {
                sink: "local_jsonl".to_string(),
                supported_fields: BTreeSet::from(["event".to_string(), "state".to_string()]),
            }],
        }
    }

    fn test_matrix() -> RuntimeApiFeatureMatrix {
        runtime_api_feature_matrix(vec![
            RuntimeApiFeatureMatrixRow {
                feature: "healthy_feature".to_string(),
                adapter: "unit".to_string(),
                claimed: true,
                health_state: RuntimeApiHealthState::Healthy,
                proof: "unit".to_string(),
            },
            RuntimeApiFeatureMatrixRow {
                feature: "missing_feature".to_string(),
                adapter: "unit".to_string(),
                claimed: true,
                health_state: RuntimeApiHealthState::Unavailable,
                proof: "unit".to_string(),
            },
        ])
    }

    #[test]
    fn runtime_api_contract_constants_remain_stable() {
        assert_eq!(
            CSM_RUNTIME_API_STATUS_SCHEMA,
            "adl.csm.runtime_api.status.v1"
        );
        assert_eq!(CSM_RUNTIME_API_ACIP_SCHEMA, "adl.csm.runtime_api.acip.v1");
        let health = test_health();
        assert_eq!(health.schema, CSM_RUNTIME_API_HEALTH_SCHEMA);
        assert_eq!(health.runtime_owner, crate::CSM_RUNTIME_OWNER);
    }

    #[test]
    fn telemetry_events_filter_supported_fields_and_feature_matrix_flags_unhealthy_claims() {
        let config = test_telemetry();
        let event = runtime_api_telemetry_event(
            &config,
            "local_jsonl",
            &json!({"event":"tick","state":"ok","secret":"drop_me"}),
        )
        .expect("telemetry event");
        assert_eq!(event["schema"], CSM_RUNTIME_API_TELEMETRY_EVENT_SCHEMA);
        assert_eq!(event["payload"]["event"], "tick");
        assert_eq!(event["payload"]["state"], "ok");
        assert!(event["payload"].get("secret").is_none());
        assert_eq!(event["dropped_unsupported_fields"], 1);

        assert_eq!(
            runtime_api_telemetry_event(&config, "missing", &json!({})).unwrap_err(),
            "telemetry_sink_unavailable"
        );
        assert_eq!(
            runtime_api_telemetry_event(&config, "local_jsonl", &json!("bad")).unwrap_err(),
            "telemetry_payload_must_be_object"
        );

        let matrix = test_matrix();
        assert_eq!(
            matrix.unresolved_claimed_features,
            vec!["missing_feature".to_string()]
        );
        assert_eq!(matrix.schema, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA);
    }

    #[test]
    fn runtime_api_helper_payloads_preserve_operator_contracts() {
        assert_eq!(
            configured_runtime_api_socket(),
            SocketAddr::from(([127, 0, 0, 1], CSM_RUNTIME_API_DEFAULT_PORT))
        );

        let persistence = persistence_health(
            crate::continuity_history::DomainHealth {
                domain: "checkpoint",
                status: "healthy",
                schema: "test.schema",
                store: "memory",
                restore_authority: true,
                record_count: 1,
                last_sequence: Some(7),
                failure_policy: "fail_closed",
            },
            crate::continuity_history::DomainHealth {
                domain: "lifelog",
                status: "unavailable",
                schema: "test.schema",
                store: "memory",
                restore_authority: false,
                record_count: 0,
                last_sequence: None,
                failure_policy: "isolated",
            },
        );
        assert_eq!(persistence["schema"], CSM_RUNTIME_API_PERSISTENCE_SCHEMA);
        assert_eq!(
            persistence["restore_authority"],
            "checkpoint_continuity_only"
        );
        assert_eq!(
            persistence["failure_isolation"],
            "independent_stores_and_lifecycle"
        );
    }
}
