use std::collections::BTreeMap;

use adl_runtime::resident_agent::CsmResidentAgentToolAuthorityBinding;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::freedom_gate::{
    evaluate_tool_candidate_freedom_gate_v1, FreedomGateToolCandidateV1,
    FreedomGateToolGateContextV1,
};
use crate::governed_executor::{
    execute_governed_action_with_adapter_v1, GovernedExecutorInputV1, GovernedExecutorSourceV1,
    GovernedToolAdapterV1,
};
use crate::tool_registry::ToolRegistryV1;
use crate::uts_acc_compiler::{
    compile_uts_to_acc_v1, ToolProposalV1, UtsAccCompilerInputV1, UtsAccPolicyContextV1,
};

pub const RUNTIME_OBSERVE_ADAPTER_V1: &str = "adapter.runtime.observe.dry_run";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ResidentToolProposalEnvelopeV1 {
    pub tool_proposal: ToolProposalV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResidentToolReceiptDecisionV1 {
    Executed,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResidentToolReceiptV1 {
    pub schema: String,
    pub resident_id: String,
    pub authority_id: String,
    pub authority_sha256: String,
    pub cycle_id: String,
    pub checkpoint_lineage: String,
    pub proposal_sha256: String,
    pub proposal_id: Option<String>,
    pub acc_contract_id: Option<String>,
    pub gate_reason_code: Option<String>,
    pub adapter_id: Option<String>,
    pub decision: ResidentToolReceiptDecisionV1,
    pub reason_code: String,
}

pub struct RuntimeObserveAdapterV1;

impl GovernedToolAdapterV1 for RuntimeObserveAdapterV1 {
    fn execute(
        &self,
        adapter_id: &str,
        arguments: &BTreeMap<String, Value>,
    ) -> Result<Value, String> {
        if adapter_id != RUNTIME_OBSERVE_ADAPTER_V1 {
            return Err("unsupported_runtime_adapter".to_string());
        }
        if !arguments.is_empty() {
            return Err("runtime_observe_arguments_not_allowed".to_string());
        }
        Ok(serde_json::json!({
            "kind": "runtime_observation",
            "status": "available",
            "redaction": "aggregate_only"
        }))
    }
}

pub struct ResidentToolExecutionContextV1<'a> {
    pub resident_id: &'a str,
    pub role: &'a str,
    pub authority: &'a CsmResidentAgentToolAuthorityBinding,
    pub cycle_id: &'a str,
    pub checkpoint_lineage: &'a str,
    pub registry: ToolRegistryV1,
    pub policy: UtsAccPolicyContextV1,
}

pub fn govern_resident_tool_output_v1(
    output: &str,
    context: ResidentToolExecutionContextV1<'_>,
    adapter: &dyn GovernedToolAdapterV1,
) -> ResidentToolReceiptV1 {
    let proposal_sha256 = hex::encode(Sha256::digest(output.as_bytes()));
    let denied = |reason_code: &str, proposal_id: Option<String>| ResidentToolReceiptV1 {
        schema: "adl.runtime.resident_tool_receipt.v1".to_string(),
        resident_id: context.resident_id.to_string(),
        authority_id: context.authority.authority_id.clone(),
        authority_sha256: context.authority.authority_sha256.clone(),
        cycle_id: context.cycle_id.to_string(),
        checkpoint_lineage: context.checkpoint_lineage.to_string(),
        proposal_sha256: proposal_sha256.clone(),
        proposal_id,
        acc_contract_id: None,
        gate_reason_code: None,
        adapter_id: None,
        decision: ResidentToolReceiptDecisionV1::Denied,
        reason_code: reason_code.to_string(),
    };

    if context.authority.validate().is_err()
        || context.policy.actor_id != context.resident_id
        || context.policy.role != context.role
        || context.policy.grant_id != context.authority.authority_id
    {
        return denied("resident_authority_mismatch", None);
    }
    let envelope: ResidentToolProposalEnvelopeV1 = match serde_json::from_str(output) {
        Ok(value) => value,
        Err(_) => return denied("invalid_or_multiple_tool_proposals", None),
    };
    let proposal = envelope.tool_proposal;
    if context
        .authority
        .allowed_tools
        .binary_search(&proposal.tool_name)
        .is_err()
    {
        return denied("tool_not_authorized", Some(proposal.proposal_id));
    }

    let compiler = compile_uts_to_acc_v1(&UtsAccCompilerInputV1 {
        proposal: proposal.clone(),
        registry: context.registry.clone(),
        policy_context: context.policy,
    });
    let Some(acc) = compiler.acc else {
        return denied("uts_acc_compiler_denied", Some(proposal.proposal_id));
    };
    let private_argument_digest = format!(
        "sha256:{}",
        hex::encode(Sha256::digest(
            serde_json::to_vec(&proposal.arguments).unwrap_or_default()
        ))
    );
    let candidate = FreedomGateToolCandidateV1 {
        candidate_id: format!("candidate.{}", proposal.proposal_id),
        proposal_id: proposal.proposal_id.clone(),
        normalized_proposal_ref: format!("normalized.{}", proposal.proposal_id),
        acc_contract_id: acc.contract_id.clone(),
        policy_evidence_ref: context.authority.authority_id.clone(),
        action_kind: proposal.tool_name.clone(),
        risk_class: "low".to_string(),
        operator_actor_id: context.resident_id.to_string(),
        citizen_boundary_ref: "runtime.resident.boundary".to_string(),
        private_argument_digest,
    };
    let gate = evaluate_tool_candidate_freedom_gate_v1(
        &candidate,
        &FreedomGateToolGateContextV1 {
            policy_decision: "allowed".to_string(),
            requires_operator_review: false,
            requires_human_challenge: false,
            escalation_available: false,
            citizen_action_boundary_intact: true,
            operator_action_boundary_intact: true,
            private_arguments_redacted: true,
        },
    );
    let execution = execute_governed_action_with_adapter_v1(
        &GovernedExecutorInputV1 {
            source: GovernedExecutorSourceV1::RegistryCompiler,
            action_id: format!("action.{}", proposal.proposal_id),
            proposal_id: proposal.proposal_id.clone(),
            acc: Some(acc.clone()),
            registry: context.registry,
            arguments: proposal.arguments,
            gate_decision: gate.clone(),
        },
        adapter,
    );
    let executed = execution.execution_result.is_some() && execution.rejected_actions.is_empty();
    ResidentToolReceiptV1 {
        schema: "adl.runtime.resident_tool_receipt.v1".to_string(),
        resident_id: context.resident_id.to_string(),
        authority_id: context.authority.authority_id.clone(),
        authority_sha256: context.authority.authority_sha256.clone(),
        cycle_id: context.cycle_id.to_string(),
        checkpoint_lineage: context.checkpoint_lineage.to_string(),
        proposal_sha256,
        proposal_id: Some(proposal.proposal_id),
        acc_contract_id: Some(acc.contract_id),
        gate_reason_code: Some(gate.reason_code),
        adapter_id: Some(acc.tool.adapter_id),
        decision: if executed {
            ResidentToolReceiptDecisionV1::Executed
        } else {
            ResidentToolReceiptDecisionV1::Denied
        },
        reason_code: if executed {
            "governed_execution_completed".to_string()
        } else {
            execution
                .rejected_actions
                .first()
                .map(|record| record.reason_code.clone())
                .unwrap_or_else(|| "governed_execution_denied".to_string())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uts_acc_compiler::wp09_compiler_input_fixture;

    struct AllowFixtureAdapter;

    impl GovernedToolAdapterV1 for AllowFixtureAdapter {
        fn execute(
            &self,
            _adapter_id: &str,
            _arguments: &BTreeMap<String, Value>,
        ) -> Result<Value, String> {
            Ok(serde_json::json!({"status":"redacted_test_execution"}))
        }
    }

    fn authority(tool_name: &str) -> CsmResidentAgentToolAuthorityBinding {
        CsmResidentAgentToolAuthorityBinding {
            authority_id: "grant.compiler.fixture".to_string(),
            authority_ref: "runtime://resident/actor.operator.alice/tool-authority".to_string(),
            authority_sha256: "a".repeat(64),
            allowed_tools: vec![tool_name.to_string()],
        }
    }

    #[test]
    fn authorized_proposal_compiles_gates_dispatches_and_receipts() {
        let compiler = wp09_compiler_input_fixture("fixture.safe_read");
        let tool_name = compiler.proposal.tool_name.clone();
        let role = compiler.policy_context.role.clone();
        let output = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
            tool_proposal: compiler.proposal,
        })
        .unwrap();
        let binding = authority(&tool_name);
        let receipt = govern_resident_tool_output_v1(
            &output,
            ResidentToolExecutionContextV1 {
                resident_id: "actor.operator.alice",
                role: &role,
                authority: &binding,
                cycle_id: "cycle.1",
                checkpoint_lineage: "checkpoint.1",
                registry: compiler.registry,
                policy: compiler.policy_context,
            },
            &AllowFixtureAdapter,
        );
        assert_eq!(
            receipt.decision,
            ResidentToolReceiptDecisionV1::Executed,
            "{receipt:?}"
        );
        assert_eq!(receipt.reason_code, "governed_execution_completed");
        assert!(receipt.acc_contract_id.is_some());
        assert_eq!(receipt.proposal_sha256.len(), 64);
    }

    #[test]
    fn authority_mismatch_and_multiple_proposals_deny_before_dispatch() {
        let compiler = wp09_compiler_input_fixture("fixture.safe_read");
        let binding = authority(&compiler.proposal.tool_name);
        let role = compiler.policy_context.role.clone();
        let receipt = govern_resident_tool_output_v1(
            "[]",
            ResidentToolExecutionContextV1 {
                resident_id: "different.resident",
                role: &role,
                authority: &binding,
                cycle_id: "cycle.2",
                checkpoint_lineage: "checkpoint.2",
                registry: compiler.registry,
                policy: compiler.policy_context,
            },
            &AllowFixtureAdapter,
        );
        assert_eq!(receipt.decision, ResidentToolReceiptDecisionV1::Denied);
        assert_eq!(receipt.reason_code, "resident_authority_mismatch");
        assert!(receipt.acc_contract_id.is_none());
    }

    #[test]
    fn runtime_observe_adapter_rejects_every_unlisted_adapter() {
        assert_eq!(
            RuntimeObserveAdapterV1
                .execute("adapter.shell", &BTreeMap::new())
                .unwrap_err(),
            "unsupported_runtime_adapter"
        );
    }
}
