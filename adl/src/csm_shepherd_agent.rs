//! Polis Shepherd Agent runtime component surfaces.
//!
//! The Shepherd is the agent-backed operator component inside CSM. Its model
//! output is advisory until admitted by the runtime policy gates.

use anyhow::{Context, Result};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub const CSM_SHEPHERD_AGENT_STATUS_SCHEMA: &str = "adl.csm.shepherd_agent.status.v1";
pub const CSM_SHEPHERD_AGENT_DECISION_SCHEMA: &str = "adl.csm.shepherd_agent.decision.v1";
pub const CSM_SHEPHERD_MODEL_POLICY_SCHEMA: &str = "adl.csm.shepherd_agent.model_policy.v1";
pub const CSM_SHEPHERD_STATUS_REF: &str = "csm_shepherd_agent_status.json";

pub fn runtime_capability() -> Value {
    json!({
        "status": "integrated",
        "component": "polis_shepherd_agent",
        "component_class": "agent_backed_csm_operator",
        "process_model": "in_process_csm_runtime_component",
        "authority_model": "advisory_until_admitted_by_freedom_gate_cav_and_runtime_policy",
        "input_channels": [
            "runtime_health",
            "daemon_lifecycle",
            "checkpoint_continuity",
            "lifelog",
            "observability",
            "freedom_gate_policy",
            "cav_security",
            "cloud_bridge_notices"
        ],
        "decision_schema": CSM_SHEPHERD_AGENT_DECISION_SCHEMA,
        "decision_actions": shepherd_decision_actions(),
        "checkpoint_authority": {
            "can_request_urgent_checkpoint": true,
            "request_limit": "policy_governed_min_interval",
            "cannot_skip_checkpoint_validation": true
        },
        "recovery_authority": {
            "can_quiesce_admission": true,
            "can_request_degrade": true,
            "can_request_governed_restart": true,
            "cannot_blind_restart_corrupted_state": true
        },
        "retained_status_ref": CSM_SHEPHERD_STATUS_REF,
        "model_policy": shepherd_model_policy()
    })
}

pub fn shepherd_model_policy() -> Value {
    json!({
        "schema": CSM_SHEPHERD_MODEL_POLICY_SCHEMA,
        "status": "resident_candidate_under_test",
        "local_runtime": "ollama",
        "candidate": {
            "model": "gemma4:12b-mlx",
            "provider": "ollama_local",
            "architecture": "gemma4_unified",
            "parameters": "12.4B",
            "context_length": 262144,
            "embedding_length": 3840,
            "quantization": "nvfp4",
            "capabilities": ["completion", "tools", "thinking"],
            "license": "Apache-2.0",
            "ollama_min_version": "0.31.0",
            "local_observed_ollama_version": "0.31.1",
            "local_observed_model_size": "7.7 GB",
            "mlx_mtp_note": {
                "source": "ollama_faster_gemma_4_mlx_mtp_2026_06_29",
                "claim": "multi-token prediction is enabled by default in Ollama 0.31 MLX builds and improves generation speed without changing model output"
            }
        },
        "resident_fallback": "Qwen3.5:9b",
        "low_memory_triage": "FastContext-4B",
        "diagnostic_fallback": "qwen3-coder:30b",
        "heavy_incident_escalation_only": "Qwen3.5:35b-a3b",
        "defaulting_rule": "gemma4:12b-mlx_not_default_until_shepherd_eval_passes",
        "required_eval_gates": [
            "decision_quality",
            "refusal_discipline",
            "typed_tool_call_formatting",
            "latency",
            "memory_pressure",
            "degraded_runtime_behavior"
        ],
        "output_authority": "advisory_only_until_runtime_policy_admits_typed_decision"
    })
}

pub fn shepherd_decision_actions() -> Value {
    json!([
        "preserve",
        "resume",
        "quarantine",
        "degrade",
        "escalate",
        "quiesce",
        "safe_fail"
    ])
}

pub fn build_status_snapshot(
    agent_instance_id: &str,
    daemon_state: &str,
    agent_state: Option<&str>,
    checkpoint_observed: bool,
    backpressure_health: Option<&str>,
) -> Value {
    let decision = classify_decision(
        daemon_state,
        agent_state,
        checkpoint_observed,
        backpressure_health,
    );
    json!({
        "schema": CSM_SHEPHERD_AGENT_STATUS_SCHEMA,
        "runtime_owner": "csm",
        "component": "polis_shepherd_agent",
        "agent_instance_id": agent_instance_id,
        "status": "monitoring",
        "model_policy": shepherd_model_policy(),
        "decision": decision,
        "observed_inputs": {
            "daemon_state": daemon_state,
            "agent_state": agent_state.unwrap_or("unknown"),
            "checkpoint_observed": checkpoint_observed,
            "backpressure_health": backpressure_health.unwrap_or("unknown")
        },
        "policy_gates": {
            "freedom_gate_required": true,
            "cav_required": true,
            "constitutional_policy_required": true,
            "model_output_advisory_only": true
        },
        "retention": {
            "status_ref": CSM_SHEPHERD_STATUS_REF,
            "lifelog_required": true,
            "observability_required": true,
            "cloud_notice_on_escalation": true
        },
        "updated_at": Utc::now()
    })
}

pub fn write_status_snapshot(
    state_root: &Path,
    agent_instance_id: &str,
    daemon_state: &str,
    agent_state: Option<&str>,
    checkpoint_observed: bool,
    backpressure_health: Option<&str>,
) -> Result<Value> {
    fs::create_dir_all(state_root)
        .with_context(|| format!("create CSM Shepherd state root {}", state_root.display()))?;
    let snapshot = build_status_snapshot(
        agent_instance_id,
        daemon_state,
        agent_state,
        checkpoint_observed,
        backpressure_health,
    );
    let path = state_root.join(CSM_SHEPHERD_STATUS_REF);
    fs::write(&path, serde_json::to_vec_pretty(&snapshot)?)
        .with_context(|| format!("write CSM Shepherd status {}", path.display()))?;
    Ok(snapshot)
}

pub fn api_status(
    agent_instance_id: &str,
    artifact: &Value,
    runtime_capability: Value,
    daemon_state: &str,
    agent_state: &str,
    checkpoint_status: &str,
    backpressure_health: Option<&str>,
) -> Value {
    let fallback = build_status_snapshot(
        agent_instance_id,
        daemon_state,
        Some(agent_state),
        checkpoint_status == "fresh" || checkpoint_status == "observed",
        backpressure_health,
    );
    let value = artifact
        .get("value")
        .cloned()
        .unwrap_or_else(|| fallback.clone());
    json!({
        "status": artifact.get("status").cloned().unwrap_or_else(|| json!("missing")),
        "ref": CSM_SHEPHERD_STATUS_REF,
        "schema": value.get("schema").cloned().unwrap_or_else(|| json!(CSM_SHEPHERD_AGENT_STATUS_SCHEMA)),
        "runtime_owner": "csm",
        "component": "polis_shepherd_agent",
        "capability": runtime_capability,
        "decision": value.get("decision").cloned().unwrap_or_else(|| fallback["decision"].clone()),
        "model_policy": value.get("model_policy").cloned().unwrap_or_else(shepherd_model_policy),
        "observed_inputs": value.get("observed_inputs").cloned().unwrap_or_else(|| fallback["observed_inputs"].clone()),
        "policy_gates": value.get("policy_gates").cloned().unwrap_or_else(|| fallback["policy_gates"].clone())
    })
}

fn classify_decision(
    daemon_state: &str,
    agent_state: Option<&str>,
    checkpoint_observed: bool,
    backpressure_health: Option<&str>,
) -> Value {
    let action = if !checkpoint_observed {
        "safe_fail"
    } else if matches!(backpressure_health, Some("capacity_degraded" | "critical")) {
        "degrade"
    } else if daemon_state == "governed_stopped" {
        "preserve"
    } else if matches!(agent_state, Some("failed")) {
        "quarantine"
    } else {
        "preserve"
    };
    json!({
        "schema": CSM_SHEPHERD_AGENT_DECISION_SCHEMA,
        "action": action,
        "authority": "advisory",
        "requires_policy_admission": true,
        "requires_checkpoint_validation": true,
        "requires_lifelog_retention": true,
        "reason": decision_reason(action)
    })
}

fn decision_reason(action: &str) -> &'static str {
    match action {
        "safe_fail" => "checkpoint_continuity_not_observed",
        "degrade" => "runtime_backpressure_degraded",
        "quarantine" => "agent_state_requires_recovery_classification",
        "preserve" => "runtime_state_recoverable_or_healthy",
        _ => "operator_review_required",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shepherd_policy_keeps_gemma_as_candidate_until_eval_passes() {
        let policy = shepherd_model_policy();
        assert_eq!(policy["candidate"]["model"], "gemma4:12b-mlx");
        assert_eq!(
            policy["defaulting_rule"],
            "gemma4:12b-mlx_not_default_until_shepherd_eval_passes"
        );
        assert_eq!(
            policy["output_authority"],
            "advisory_only_until_runtime_policy_admits_typed_decision"
        );
    }

    #[test]
    fn shepherd_decision_safe_fails_when_checkpoint_is_missing() {
        let status = build_status_snapshot("polis-alpha", "running", Some("idle"), false, None);
        assert_eq!(status["decision"]["action"], "safe_fail");
        assert_eq!(
            status["decision"]["reason"],
            "checkpoint_continuity_not_observed"
        );
    }

    #[test]
    fn shepherd_decision_preserves_normal_active_cycle_state() {
        let status =
            build_status_snapshot("polis-alpha", "running", Some("running_cycle"), true, None);
        assert_eq!(status["decision"]["action"], "preserve");
        assert_eq!(
            status["policy_gates"]["freedom_gate_required"],
            serde_json::Value::Bool(true)
        );
    }

    #[test]
    fn shepherd_decision_quarantines_failed_agent_state() {
        let status = build_status_snapshot("polis-alpha", "running", Some("failed"), true, None);
        assert_eq!(status["decision"]["action"], "quarantine");
    }
}
