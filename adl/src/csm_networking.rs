//! CSM-owned local networking registry and runtime resource pooling plan.
use anyhow::{anyhow, bail, Context, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

pub const CSM_NETWORKING_SCHEMA: &str = "adl.csm.networking.v1";
pub const CSM_POOLING_PLAN_SCHEMA: &str = "adl.csm.pooling_plan.v1";
pub const CSM_LOCAL_PORT_RANGE_START: u16 = 19950;
pub const CSM_LOCAL_PORT_RANGE_END: u16 = 19999;
pub const CSM_MAIN_API_PORT: u16 = 19997;
pub const CSM_LOOPBACK_HOST: &str = "127.0.0.1";
pub const CSM_MAIN_API_BIND: &str = "127.0.0.1:19997";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CsmListenerRole {
    MainRuntimeApi,
    ApiGatewayBridge,
    ChronosenseNtp,
    OTelCollector,
    LocalTestHarness,
    FutureServiceListener,
}

impl CsmListenerRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MainRuntimeApi => "main_runtime_api",
            Self::ApiGatewayBridge => "api_gateway_bridge",
            Self::ChronosenseNtp => "chronosense_ntp",
            Self::OTelCollector => "otel_collector",
            Self::LocalTestHarness => "local_test_harness",
            Self::FutureServiceListener => "future_service_listener",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CsmListenerConfig {
    pub role: CsmListenerRole,
    pub bind_addr: SocketAddr,
    pub configured_by: String,
    pub reserved_range: String,
    pub canonical: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_ephemeral_reason: Option<String>,
}

impl CsmListenerConfig {
    pub fn to_observability_json(&self) -> Value {
        json!({
            "schema": CSM_NETWORKING_SCHEMA,
            "listener_role": self.role.as_str(),
            "bind_addr": self.bind_addr.to_string(),
            "configured_by": self.configured_by,
            "reserved_range": self.reserved_range,
            "canonical": self.canonical,
            "test_ephemeral_reason": self.test_ephemeral_reason,
            "remediation_hint": remediation_hint(self.role)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CsmPoolRolePlan {
    pub role: &'static str,
    pub strategy: &'static str,
    pub decision: &'static str,
    pub exhaustion_signal: &'static str,
}

pub fn default_main_runtime_api_listener() -> CsmListenerConfig {
    CsmListenerConfig {
        role: CsmListenerRole::MainRuntimeApi,
        bind_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), CSM_MAIN_API_PORT),
        configured_by: "canonical_default".to_string(),
        reserved_range: csm_reserved_range_label(),
        canonical: true,
        test_ephemeral_reason: None,
    }
}

pub fn resolve_main_runtime_api_listener(
    bind_override: Option<&str>,
    allow_test_ephemeral: bool,
) -> Result<CsmListenerConfig> {
    let Some(raw) = bind_override else {
        return Ok(default_main_runtime_api_listener());
    };
    let bind_addr = SocketAddr::from_str(raw)
        .with_context(|| format!("parse CSM main runtime API bind address {raw}"))?;
    ensure_loopback(bind_addr)?;
    if bind_addr.port() == 0 {
        if allow_test_ephemeral {
            return Ok(CsmListenerConfig {
                role: CsmListenerRole::LocalTestHarness,
                bind_addr,
                configured_by: "explicit_test_ephemeral_override".to_string(),
                reserved_range: csm_reserved_range_label(),
                canonical: false,
                test_ephemeral_reason: Some(
                    "ephemeral bind allowed only for classified bounded test harness execution"
                        .to_string(),
                ),
            });
        }
        bail!(
            "CSM main runtime API refuses unclassified loopback ephemeral bind {raw}; use the canonical {CSM_MAIN_API_BIND} or pass an explicit bounded test harness option"
        );
    }
    if !is_csm_reserved_local_port(bind_addr.port()) {
        bail!(
            "CSM main runtime API bind {raw} is outside reserved local CSM port range {}; choose a governed CSM port or document a new listener role",
            csm_reserved_range_label()
        );
    }
    Ok(CsmListenerConfig {
        role: CsmListenerRole::MainRuntimeApi,
        bind_addr,
        configured_by: "explicit_override".to_string(),
        reserved_range: csm_reserved_range_label(),
        canonical: bind_addr.port() == CSM_MAIN_API_PORT,
        test_ephemeral_reason: None,
    })
}

pub fn reject_temp_allocation_port(port: u16) -> Result<()> {
    if port == CSM_MAIN_API_PORT {
        bail!(
            "port {CSM_MAIN_API_PORT} is reserved for the CSM main runtime API and cannot be used for temporary allocation"
        );
    }
    if is_csm_reserved_local_port(port) {
        bail!(
            "port {port} is inside reserved CSM range {} and needs an explicit listener role before allocation",
            csm_reserved_range_label()
        );
    }
    Ok(())
}

pub fn csm_reserved_range_label() -> String {
    format!("{CSM_LOCAL_PORT_RANGE_START}-{CSM_LOCAL_PORT_RANGE_END}")
}

pub fn is_csm_reserved_local_port(port: u16) -> bool {
    (CSM_LOCAL_PORT_RANGE_START..=CSM_LOCAL_PORT_RANGE_END).contains(&port)
}

pub fn csm_listener_registry_json() -> Value {
    json!({
        "schema": CSM_NETWORKING_SCHEMA,
        "reserved_local_range": csm_reserved_range_label(),
        "loopback_host": CSM_LOOPBACK_HOST,
        "listeners": [
            {
                "role": CsmListenerRole::MainRuntimeApi.as_str(),
                "default_bind": CSM_MAIN_API_BIND,
                "ownership": "csm",
                "consumers": ["local_operator", "api_gateway_bridge"],
                "temporary_allocation_allowed": false
            },
            {
                "role": CsmListenerRole::ApiGatewayBridge.as_str(),
                "default_bind": "not_bound_by_5040",
                "ownership": "csm_runtime_side_contract_for_5039",
                "consumers": ["aws_api_gateway_bridge"],
                "temporary_allocation_allowed": false
            },
            {
                "role": CsmListenerRole::ChronosenseNtp.as_str(),
                "default_bind": "123/udp boundary owned by 5041",
                "ownership": "chronosense_boundary",
                "consumers": ["time_sync"],
                "temporary_allocation_allowed": false
            },
            {
                "role": CsmListenerRole::OTelCollector.as_str(),
                "default_bind": "collector_configured_endpoint",
                "ownership": "observability_pipeline",
                "consumers": ["otel_export"],
                "temporary_allocation_allowed": false
            },
            {
                "role": CsmListenerRole::LocalTestHarness.as_str(),
                "default_bind": "127.0.0.1:0 only with explicit test flags",
                "ownership": "bounded_test_infrastructure",
                "consumers": ["unit_tests", "cli_smoke"],
                "temporary_allocation_allowed": true
            }
        ],
        "external_boundaries": [
            {"port": 443, "role": "public_tls_or_aws_api_gateway", "owner": "external_or_gateway"},
            {"port": 8443, "role": "future_local_tls_dev_gateway", "owner": "unimplemented_future_listener"},
            {"port": 22, "role": "ssh_admin", "owner": "host_or_cloud_provider"},
            {"port": 2222, "role": "alternate_ssh_dev", "owner": "host_or_cloud_provider"},
            {"port": 123, "role": "ntp", "owner": "chronosense_boundary"},
            {"port": Value::Null, "role": "eventbridge_sns_sqs", "owner": "aws_control_plane_no_local_listener"}
        ]
    })
}

pub fn csm_connection_pooling_plan() -> Value {
    let roles = vec![
        CsmPoolRolePlan {
            role: "http_clients",
            strategy: "reuse reqwest/hyper client pools per runtime owner",
            decision: "do_not_add_deadpool_wrapper_for_http_clients",
            exhaustion_signal: "emit client_pool_error and retry/backpressure context",
        },
        CsmPoolRolePlan {
            role: "aws_sdk_clients",
            strategy: "reuse AWS SDK clients/config per account/profile/region",
            decision: "do_not_wrap_aws_sdk_clients_in_deadpool",
            exhaustion_signal: "emit aws_client_construction_or_throttle signal",
        },
        CsmPoolRolePlan {
            role: "database_or_lifelog_connections",
            strategy: "use deadpool-style bounded pools when a concrete DB backend lands",
            decision: "stage_deadpool_until_db_backend_exists",
            exhaustion_signal: "emit pool_exhausted with role and remediation hint",
        },
        CsmPoolRolePlan {
            role: "otel_export_sinks",
            strategy: "use bounded batch/export queues with explicit timeout",
            decision: "prefer_exporter_backpressure_over_generic_pool",
            exhaustion_signal: "emit otel_export_backpressure",
        },
        CsmPoolRolePlan {
            role: "internal_citizen_polis_channels",
            strategy: "bounded channels with named capacity and safe-fail policy",
            decision: "no_deadpool_for_in_memory_channels",
            exhaustion_signal: "emit channel_backpressure and safe_fail_action",
        },
    ];
    json!({
        "schema": CSM_POOLING_PLAN_SCHEMA,
        "decision_summary": "No blanket Deadpool dependency for CSM runtime resources in #5040; use existing client-native pooling for HTTP/AWS SDK clients and reserve Deadpool-style bounded pools for future concrete database/lifelog connection backends.",
        "pool_event_contract": {
            "required_fields": ["role", "event", "capacity_or_limit", "remediation_hint"],
            "events": ["pool_configured", "pool_exhausted", "pool_recovered", "client_reused"]
        },
        "roles": roles
    })
}

fn ensure_loopback(addr: SocketAddr) -> Result<()> {
    if !addr.ip().is_loopback() {
        return Err(anyhow!(
            "CSM listeners require loopback bind addresses unless remote auth is implemented"
        ));
    }
    Ok(())
}

fn remediation_hint(role: CsmListenerRole) -> &'static str {
    match role {
        CsmListenerRole::MainRuntimeApi => {
            "free 127.0.0.1:19997 or pass an explicit reserved CSM port override"
        }
        CsmListenerRole::LocalTestHarness => {
            "use ephemeral ports only with explicit bounded test harness flags"
        }
        _ => "declare the listener role and reserved CSM port before binding",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runtime_api_defaults_to_canonical_csm_port() {
        let listener = resolve_main_runtime_api_listener(None, false).unwrap();
        assert_eq!(listener.role, CsmListenerRole::MainRuntimeApi);
        assert_eq!(listener.bind_addr.to_string(), CSM_MAIN_API_BIND);
        assert!(listener.canonical);
    }

    #[test]
    fn main_runtime_api_rejects_unclassified_ephemeral_bind() {
        let err = resolve_main_runtime_api_listener(Some("127.0.0.1:0"), false)
            .expect_err("unclassified CSM runtime :0 bind must fail");
        assert!(err.to_string().contains("refuses unclassified"));
    }

    #[test]
    fn main_runtime_api_allows_classified_test_ephemeral_bind() {
        let listener = resolve_main_runtime_api_listener(Some("127.0.0.1:0"), true).unwrap();
        assert_eq!(listener.role, CsmListenerRole::LocalTestHarness);
        assert!(listener.test_ephemeral_reason.is_some());
    }

    #[test]
    fn reserved_csm_ports_are_not_temp_allocation_candidates() {
        let err = reject_temp_allocation_port(CSM_MAIN_API_PORT)
            .expect_err("main API port must not be temporary");
        assert!(err
            .to_string()
            .contains("reserved for the CSM main runtime API"));
        assert!(reject_temp_allocation_port(20001).is_ok());
    }
}
