//! CSM resident-agent admission contracts.
//!
//! These contracts live in the runtime crate so model-backed agents enter CSM
//! through one runtime-owned shape rather than bespoke component policies.

use serde::{Deserialize, Serialize};

pub const CSM_RESIDENT_AGENT_SCHEMA: &str = "adl.csm.resident_agent.v1";
pub const CSM_RESIDENT_AGENT_SET_SCHEMA: &str = "adl.csm.resident_agent_set.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmResidentAgentAuthority {
    Ordinary,
    ShepherdOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CsmResidentAgentLifecycleState {
    Admitted,
    Ready,
    Running,
    Degraded,
    Quarantined,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmResidentAgentProviderBinding {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: String,
    pub runtime_surface: String,
    pub model_ref: String,
    pub provider_model_id: String,
    pub tool_calling_mode: String,
    pub structured_json_mode: String,
    pub binding_status: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmResidentAgentChannels {
    pub lifecycle: String,
    pub provider_request: String,
    pub provider_response: String,
    pub checkpoint: String,
    pub observability: String,
    pub lifelog: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmResidentAgentPolicyGates {
    pub freedom_gate_required: bool,
    pub cav_required: bool,
    pub constitutional_policy_required: bool,
    pub model_output_advisory_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmResidentAgentSpec {
    pub schema: String,
    pub agent_instance_id: String,
    pub display_name: String,
    pub agent_role: String,
    pub authority: CsmResidentAgentAuthority,
    pub lifecycle_state: CsmResidentAgentLifecycleState,
    pub provider_binding: CsmResidentAgentProviderBinding,
    pub channels: CsmResidentAgentChannels,
    pub policy_gates: CsmResidentAgentPolicyGates,
    pub checkpoint_policy: String,
    pub lifelog_policy: String,
    pub observability_policy: String,
    pub privilege_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CsmResidentAgentSet {
    pub schema: String,
    pub runtime_owner: String,
    pub admission_model: String,
    pub provider_entrypoint: String,
    pub agents: Vec<CsmResidentAgentSpec>,
}

impl CsmResidentAgentSpec {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, CSM_RESIDENT_AGENT_SCHEMA, "schema")?;
        require_non_empty(&self.agent_instance_id, "agent_instance_id")?;
        require_non_empty(&self.agent_role, "agent_role")?;
        require_non_empty(
            &self.provider_binding.provider_id,
            "provider_binding.provider_id",
        )?;
        require_non_empty(
            &self.provider_binding.model_ref,
            "provider_binding.model_ref",
        )?;
        require_non_empty(
            &self.provider_binding.provider_model_id,
            "provider_binding.provider_model_id",
        )?;
        require_non_empty(
            &self.provider_binding.runtime_surface,
            "provider_binding.runtime_surface",
        )?;
        require_exact(
            &self.provider_binding.binding_status,
            "provider_target_resolved",
            "provider_binding.binding_status",
        )?;
        if self.authority == CsmResidentAgentAuthority::ShepherdOperator {
            if !self.policy_gates.freedom_gate_required
                || !self.policy_gates.cav_required
                || !self.policy_gates.constitutional_policy_required
                || !self.policy_gates.model_output_advisory_only
            {
                return Err(
                    "shepherd resident agent must be gated by Freedom Gate, CAV, constitutional policy, and advisory output authority"
                        .to_string(),
                );
            }
        }
        Ok(())
    }
}

impl CsmResidentAgentSet {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, CSM_RESIDENT_AGENT_SET_SCHEMA, "schema")?;
        require_exact(&self.runtime_owner, "csm", "runtime_owner")?;
        require_exact(
            &self.provider_entrypoint,
            "provider_substrate",
            "provider_entrypoint",
        )?;
        if self.agents.len() < 3 {
            return Err("resident agent set must contain at least three agents".to_string());
        }
        let shepherd_count = self
            .agents
            .iter()
            .filter(|agent| agent.authority == CsmResidentAgentAuthority::ShepherdOperator)
            .count();
        if shepherd_count != 1 {
            return Err(
                "resident agent set must contain exactly one Shepherd operator".to_string(),
            );
        }
        for agent in &self.agents {
            agent.validate()?;
        }
        Ok(())
    }
}

fn require_non_empty(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
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

    fn binding(provider_id: &str, model_ref: &str) -> CsmResidentAgentProviderBinding {
        CsmResidentAgentProviderBinding {
            provider_id: provider_id.to_string(),
            provider_kind: "ollama".to_string(),
            vendor: "ollama".to_string(),
            transport: "http".to_string(),
            runtime_surface: "ollama_http".to_string(),
            model_ref: model_ref.to_string(),
            provider_model_id: model_ref.to_string(),
            tool_calling_mode: "native".to_string(),
            structured_json_mode: "prompt_based".to_string(),
            binding_status: "provider_target_resolved".to_string(),
            source: "provider_substrate".to_string(),
        }
    }

    fn channels(id: &str) -> CsmResidentAgentChannels {
        CsmResidentAgentChannels {
            lifecycle: format!("csm.lifecycle.{id}"),
            provider_request: format!("csm.provider_requests.{id}"),
            provider_response: format!("csm.provider_responses.{id}"),
            checkpoint: format!("csm.checkpoint.{id}"),
            observability: format!("csm.observability.{id}"),
            lifelog: format!("csm.lifelog.{id}"),
        }
    }

    fn gates() -> CsmResidentAgentPolicyGates {
        CsmResidentAgentPolicyGates {
            freedom_gate_required: true,
            cav_required: true,
            constitutional_policy_required: true,
            model_output_advisory_only: true,
        }
    }

    fn agent(id: &str, authority: CsmResidentAgentAuthority) -> CsmResidentAgentSpec {
        CsmResidentAgentSpec {
            schema: CSM_RESIDENT_AGENT_SCHEMA.to_string(),
            agent_instance_id: id.to_string(),
            display_name: id.to_string(),
            agent_role: "runtime_worker".to_string(),
            authority,
            lifecycle_state: CsmResidentAgentLifecycleState::Admitted,
            provider_binding: binding("local_ollama", "gemma4:12b-mlx"),
            channels: channels(id),
            policy_gates: gates(),
            checkpoint_policy: "periodic_and_agent_requested".to_string(),
            lifelog_policy: "append_lifecycle_provider_events".to_string(),
            observability_policy: "emit_provider_lifecycle_metrics_traces_logs".to_string(),
            privilege_reason: "ordinary_runtime_agent".to_string(),
        }
    }

    #[test]
    fn resident_agent_set_requires_one_shepherd_and_provider_bindings() {
        let set = CsmResidentAgentSet {
            schema: CSM_RESIDENT_AGENT_SET_SCHEMA.to_string(),
            runtime_owner: "csm".to_string(),
            admission_model: "provider_bound_resident_agents".to_string(),
            provider_entrypoint: "provider_substrate".to_string(),
            agents: vec![
                agent("shepherd", CsmResidentAgentAuthority::ShepherdOperator),
                agent("codex", CsmResidentAgentAuthority::Ordinary),
                agent("ollama-worker", CsmResidentAgentAuthority::Ordinary),
            ],
        };
        assert!(set.validate().is_ok());
    }

    #[test]
    fn shepherd_cannot_bypass_runtime_policy_gates() {
        let mut shepherd = agent("shepherd", CsmResidentAgentAuthority::ShepherdOperator);
        shepherd.policy_gates.cav_required = false;
        assert!(shepherd.validate().is_err());
    }
}
