use std::collections::BTreeMap;

use adl_runtime::resident_agent::CsmResidentAgentToolAuthorityBinding;
use adl_runtime_kernel::{VerifiedToolAuthorityBinding, PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
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
use crate::tool_registry::{RegisteredToolV1, ToolAdapterCapabilityV1, ToolRegistryV1};
use crate::uts::{
    UniversalToolSchemaV1_1, UtsAuthenticationModeV1, UtsAuthenticationRequirementV1,
    UtsCategoryV1, UtsCompatibleVersionV1, UtsDataSensitivityV1, UtsDeterminismV1, UtsErrorModelV1,
    UtsExecutionEnvironmentKindV1, UtsExecutionEnvironmentV1, UtsExfiltrationRiskV1,
    UtsIdempotenceV1, UtsJsonSchemaFragmentV1, UtsObservabilityV1, UtsPlanningMetadataV1,
    UtsReplaySafetyV1, UtsResourceRequirementV1, UtsSideEffectClassV1, UtsSideEffectTagV1,
    UTS_SCHEMA_VERSION_V1_1,
};
use crate::uts_acc_compiler::{
    compile_uts_to_acc_v1, ToolProposalV1, UtsAccCompilerInputV1, UtsAccPolicyContextV1,
};

pub const RUNTIME_OBSERVE_ADAPTER_V1: &str = "adapter.runtime.observe.dry_run";

pub fn runtime_observe_registry_v1() -> ToolRegistryV1 {
    let empty_object = || UtsJsonSchemaFragmentV1 {
        schema_type: "object".to_string(),
        keywords: BTreeMap::from([
            ("properties".to_string(), serde_json::json!({})),
            ("required".to_string(), serde_json::json!([])),
            ("additionalProperties".to_string(), serde_json::json!(false)),
        ]),
    };
    let uts = UniversalToolSchemaV1_1 {
        schema_version: UTS_SCHEMA_VERSION_V1_1.to_string(),
        compatible_versions: vec![UtsCompatibleVersionV1::V1, UtsCompatibleVersionV1::V1_1],
        name: "runtime.observe".to_string(),
        version: "1.0.0".to_string(),
        description: "Return a redacted aggregate observation of the current Runtime.".to_string(),
        categories: Some(vec![
            UtsCategoryV1::ReadOnly,
            UtsCategoryV1::ObservabilitySensitive,
        ]),
        input_schema: empty_object(),
        output_schema: empty_object(),
        side_effect_class: UtsSideEffectClassV1::Read,
        side_effects: Some(vec![UtsSideEffectTagV1::None]),
        determinism: UtsDeterminismV1::BoundedNondeterministic,
        replay_safety: UtsReplaySafetyV1::ReplaySafe,
        idempotence: UtsIdempotenceV1::Idempotent,
        resources: vec![UtsResourceRequirementV1 {
            resource_type: "runtime".to_string(),
            scope: "aggregate-observation".to_string(),
        }],
        authentication: UtsAuthenticationRequirementV1 {
            mode: UtsAuthenticationModeV1::None,
            required: false,
        },
        data_sensitivity: UtsDataSensitivityV1::Internal,
        exfiltration_risk: UtsExfiltrationRiskV1::None,
        execution_environment: UtsExecutionEnvironmentV1 {
            kind: UtsExecutionEnvironmentKindV1::DryRun,
            isolation: "runtime-owned aggregate-only adapter".to_string(),
        },
        errors: vec![UtsErrorModelV1 {
            code: "runtime_observation_unavailable".to_string(),
            message: "The redacted Runtime observation is unavailable.".to_string(),
            retryable: false,
        }],
        observability: Some(UtsObservabilityV1::Governance),
        planning: Some(UtsPlanningMetadataV1 {
            review_recommended: Some(false),
            ..UtsPlanningMetadataV1::default()
        }),
        extensions: BTreeMap::new(),
    };
    ToolRegistryV1 {
        schema_version: "tool_registry.v1".to_string(),
        registry_id: "runtime.resident.tools".to_string(),
        tools: vec![RegisteredToolV1::new(
            "runtime.observe.v1".to_string(),
            "runtime.observe".to_string(),
            "1.0.0".to_string(),
            true,
            uts,
            vec![RUNTIME_OBSERVE_ADAPTER_V1.to_string()],
        )],
        adapters: vec![ToolAdapterCapabilityV1 {
            adapter_id: RUNTIME_OBSERVE_ADAPTER_V1.to_string(),
            tool_name: "runtime.observe".to_string(),
            tool_version: "1.0.0".to_string(),
            capability_id: "capability.runtime.observe.v1".to_string(),
            side_effect_class: UtsSideEffectClassV1::Read,
            execution_environment: UtsExecutionEnvironmentKindV1::DryRun,
            supports_dry_run: true,
            approved_for_binding: true,
        }],
    }
}

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
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AuthenticatedResidentToolReceiptV1 {
    pub schema: String,
    pub implementation_revision_sha256: String,
    pub capability_envelope_sha256: String,
    pub cognitive_profile_sha256: String,
    pub receipt_sha256: String,
    pub key_id: String,
    pub signature_hex: String,
}

#[derive(Serialize)]
struct ResidentToolAuthenticationMaterial<'a> {
    receipt: &'a ResidentToolReceiptV1,
    implementation_revision_sha256: &'a str,
    capability_envelope_sha256: &'a str,
    cognitive_profile_sha256: &'a str,
    key_id: &'a str,
}

pub fn authenticate_resident_tool_receipt_v1(
    receipt: &ResidentToolReceiptV1,
    implementation_revision_sha256: &str,
    capability_envelope_sha256: &str,
    cognitive_profile_sha256: &str,
    key_id: &str,
    signing_key: &SigningKey,
) -> Result<AuthenticatedResidentToolReceiptV1, String> {
    if !is_sha256(implementation_revision_sha256)
        || !is_sha256(capability_envelope_sha256)
        || !is_sha256(cognitive_profile_sha256)
        || key_id.trim().is_empty()
    {
        return Err("invalid_resident_tool_authentication_binding".to_string());
    }
    let material = ResidentToolAuthenticationMaterial {
        receipt,
        implementation_revision_sha256,
        capability_envelope_sha256,
        cognitive_profile_sha256,
        key_id,
    };
    let bytes = serde_jcs::to_vec(&material).map_err(|error| error.to_string())?;
    let receipt_sha256 = hex::encode(Sha256::digest(&bytes));
    Ok(AuthenticatedResidentToolReceiptV1 {
        schema: "adl.runtime.authenticated_resident_tool_receipt.v1".to_string(),
        implementation_revision_sha256: implementation_revision_sha256.to_string(),
        capability_envelope_sha256: capability_envelope_sha256.to_string(),
        cognitive_profile_sha256: cognitive_profile_sha256.to_string(),
        receipt_sha256,
        key_id: key_id.to_string(),
        signature_hex: hex::encode(signing_key.sign(&bytes).to_bytes()),
    })
}

pub fn validate_resident_tool_receipt_for_birthday_v1(
    receipt: &ResidentToolReceiptV1,
    authenticated: &AuthenticatedResidentToolReceiptV1,
    verifying_key: &VerifyingKey,
) -> Result<VerifiedToolAuthorityBinding, String> {
    if authenticated.schema != "adl.runtime.authenticated_resident_tool_receipt.v1"
        || receipt.schema != "adl.runtime.resident_tool_receipt.v1"
        || !is_sha256(&authenticated.implementation_revision_sha256)
        || !is_sha256(&authenticated.capability_envelope_sha256)
        || !is_sha256(&authenticated.cognitive_profile_sha256)
    {
        return Err("invalid_authenticated_resident_tool_receipt".to_string());
    }
    let material = ResidentToolAuthenticationMaterial {
        receipt,
        implementation_revision_sha256: &authenticated.implementation_revision_sha256,
        capability_envelope_sha256: &authenticated.capability_envelope_sha256,
        cognitive_profile_sha256: &authenticated.cognitive_profile_sha256,
        key_id: &authenticated.key_id,
    };
    let bytes = serde_jcs::to_vec(&material).map_err(|error| error.to_string())?;
    if authenticated.receipt_sha256 != hex::encode(Sha256::digest(&bytes)) {
        return Err("resident_tool_receipt_digest_mismatch".to_string());
    }
    let signature_bytes: [u8; 64] = hex::decode(&authenticated.signature_hex)
        .map_err(|_| "resident_tool_receipt_signature_invalid".to_string())?
        .try_into()
        .map_err(|_| "resident_tool_receipt_signature_invalid".to_string())?;
    let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes);
    verifying_key
        .verify(&bytes, &signature)
        .map_err(|_| "resident_tool_receipt_signature_invalid".to_string())?;
    Ok(VerifiedToolAuthorityBinding {
        schema: PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA.to_string(),
        resident_id: receipt.resident_id.clone(),
        cycle_id: receipt.cycle_id.clone(),
        continuity_head_sha256: receipt.checkpoint_lineage.clone(),
        capability_envelope_sha256: authenticated.capability_envelope_sha256.clone(),
        cognitive_profile_sha256: authenticated.cognitive_profile_sha256.clone(),
        implementation_revision_sha256: authenticated.implementation_revision_sha256.clone(),
        decision: match receipt.decision {
            ResidentToolReceiptDecisionV1::Executed => "executed",
            ResidentToolReceiptDecisionV1::Denied => "denied",
        }
        .to_string(),
        receipt_sha256: authenticated.receipt_sha256.clone(),
        authentication_sha256: hex::encode(Sha256::digest(authenticated.signature_hex.as_bytes())),
    })
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

pub struct RuntimeObserveAdapterV1 {
    snapshot: Value,
}

impl RuntimeObserveAdapterV1 {
    pub fn new(snapshot: Value) -> Result<Self, String> {
        let object = snapshot
            .as_object()
            .ok_or_else(|| "runtime_observation_must_be_object".to_string())?;
        let allowed = [
            "kind",
            "status",
            "resident_id",
            "cycle_id",
            "checkpoint_lineage",
            "redaction",
        ];
        if object.keys().any(|key| !allowed.contains(&key.as_str()))
            || object.values().any(|value| {
                value
                    .as_str()
                    .map(|text| text.len() > 256 || text.contains(['{', '}', '\n', '\r']))
                    .unwrap_or(true)
            })
        {
            return Err("runtime_observation_not_redacted".to_string());
        }
        Ok(Self { snapshot })
    }
}

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
        Ok(self.snapshot.clone())
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
    pub risk_class: &'a str,
    pub citizen_boundary_ref: &'a str,
    pub gate_context: FreedomGateToolGateContextV1,
}

fn extract_single_proposal_envelope(output: &str) -> Option<ResidentToolProposalEnvelopeV1> {
    let trimmed = output.strip_prefix("USER:\n").unwrap_or(output).trim();
    if let Ok(envelope) = serde_json::from_str(trimmed) {
        return Some(envelope);
    }
    let mut envelopes = Vec::new();
    for (start, byte) in trimmed.bytes().enumerate() {
        if byte != b'{' || !trimmed.is_char_boundary(start) {
            continue;
        }
        let mut stream = serde_json::Deserializer::from_str(&trimmed[start..])
            .into_iter::<ResidentToolProposalEnvelopeV1>();
        if let Some(Ok(envelope)) = stream.next() {
            envelopes.push(envelope);
        }
    }
    (envelopes.len() == 1).then(|| envelopes.remove(0))
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
        proposal_id: proposal_id
            .map(|value| format!("sha256:{}", hex::encode(Sha256::digest(value.as_bytes())))),
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
    let envelope = match extract_single_proposal_envelope(output) {
        Some(value) => value,
        None => return denied("invalid_or_multiple_tool_proposals", None),
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
        risk_class: context.risk_class.to_string(),
        operator_actor_id: context.resident_id.to_string(),
        citizen_boundary_ref: context.citizen_boundary_ref.to_string(),
        private_argument_digest,
    };
    let gate = evaluate_tool_candidate_freedom_gate_v1(&candidate, &context.gate_context);
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
        proposal_id: Some(format!(
            "sha256:{}",
            hex::encode(Sha256::digest(proposal.proposal_id.as_bytes()))
        )),
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
        CsmResidentAgentToolAuthorityBinding::new(
            "grant.compiler.fixture",
            "runtime://resident/actor.operator.alice/tool-authority",
            vec![tool_name.to_string()],
        )
    }

    fn allowed_gate() -> FreedomGateToolGateContextV1 {
        FreedomGateToolGateContextV1 {
            policy_decision: "allowed".to_string(),
            requires_operator_review: false,
            requires_human_challenge: false,
            escalation_available: false,
            citizen_action_boundary_intact: true,
            operator_action_boundary_intact: true,
            private_arguments_redacted: true,
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
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: allowed_gate(),
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
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: allowed_gate(),
            },
            &AllowFixtureAdapter,
        );
        assert_eq!(receipt.decision, ResidentToolReceiptDecisionV1::Denied);
        assert_eq!(receipt.reason_code, "resident_authority_mismatch");
        assert!(receipt.acc_contract_id.is_none());
    }

    #[test]
    fn identical_duplicate_proposals_are_multiple_and_denied() {
        let compiler = wp09_compiler_input_fixture("fixture.safe_read");
        let binding = authority(&compiler.proposal.tool_name);
        let role = compiler.policy_context.role.clone();
        let envelope = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
            tool_proposal: compiler.proposal,
        })
        .unwrap();
        let receipt = govern_resident_tool_output_v1(
            &format!("{envelope}\n{envelope}"),
            ResidentToolExecutionContextV1 {
                resident_id: "actor.operator.alice",
                role: &role,
                authority: &binding,
                cycle_id: "cycle.duplicate",
                checkpoint_lineage: "checkpoint.duplicate",
                registry: compiler.registry,
                policy: compiler.policy_context,
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: allowed_gate(),
            },
            &AllowFixtureAdapter,
        );
        assert_eq!(receipt.decision, ResidentToolReceiptDecisionV1::Denied);
        assert_eq!(receipt.reason_code, "invalid_or_multiple_tool_proposals");
    }

    #[test]
    fn runtime_observe_adapter_rejects_every_unlisted_adapter() {
        let adapter = RuntimeObserveAdapterV1::new(serde_json::json!({
            "kind": "runtime_observation",
            "status": "available",
            "resident_id": "resident.test",
            "cycle_id": "cycle.test",
            "checkpoint_lineage": "checkpoint.test",
            "redaction": "aggregate_only"
        }))
        .unwrap();
        assert_eq!(
            adapter
                .execute("adapter.shell", &BTreeMap::new())
                .unwrap_err(),
            "unsupported_runtime_adapter"
        );
    }

    #[test]
    fn production_runtime_observe_compiles_gates_and_executes() {
        let mut policy = crate::uts_acc_compiler::wp09_policy_context_fixture();
        policy.allowed_side_effects = vec![UtsSideEffectClassV1::Read];
        policy.allowed_resource_scopes = vec!["aggregate-observation".to_string()];
        let resident_id = policy.actor_id.clone();
        let role = policy.role.clone();
        let binding = CsmResidentAgentToolAuthorityBinding::new(
            policy.grant_id.clone(),
            format!("runtime://resident/{resident_id}/tool-authority"),
            vec!["runtime.observe".to_string()],
        );
        let output = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
            tool_proposal: ToolProposalV1 {
                proposal_id: "proposal.runtime-observe".to_string(),
                tool_name: "runtime.observe".to_string(),
                tool_version: "1.0.0".to_string(),
                adapter_id: RUNTIME_OBSERVE_ADAPTER_V1.to_string(),
                arguments: BTreeMap::new(),
                dry_run_requested: true,
                ambiguous: false,
            },
        })
        .unwrap();
        let receipt = govern_resident_tool_output_v1(
            &output,
            ResidentToolExecutionContextV1 {
                resident_id: &resident_id,
                role: &role,
                authority: &binding,
                cycle_id: "cycle.runtime.1",
                checkpoint_lineage: "checkpoint.runtime.1#sha256:abc",
                registry: runtime_observe_registry_v1(),
                policy,
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: allowed_gate(),
            },
            &RuntimeObserveAdapterV1::new(serde_json::json!({
                "kind": "runtime_observation",
                "status": "available",
                "resident_id": resident_id,
                "cycle_id": "cycle.runtime.1",
                "checkpoint_lineage": "checkpoint.runtime.1",
                "redaction": "aggregate_only"
            }))
            .unwrap(),
        );
        assert_eq!(receipt.decision, ResidentToolReceiptDecisionV1::Executed);
        assert_eq!(
            receipt.adapter_id.as_deref(),
            Some(RUNTIME_OBSERVE_ADAPTER_V1)
        );
    }

    #[test]
    fn configured_freedom_gate_denial_stops_before_adapter() {
        let compiler = wp09_compiler_input_fixture("fixture.safe_read");
        let binding = authority(&compiler.proposal.tool_name);
        let role = compiler.policy_context.role.clone();
        let output = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
            tool_proposal: compiler.proposal,
        })
        .unwrap();
        let mut gate = allowed_gate();
        gate.policy_decision = "denied".to_string();
        let receipt = govern_resident_tool_output_v1(
            &output,
            ResidentToolExecutionContextV1 {
                resident_id: "actor.operator.alice",
                role: &role,
                authority: &binding,
                cycle_id: "cycle.denied",
                checkpoint_lineage: "checkpoint.denied",
                registry: compiler.registry,
                policy: compiler.policy_context,
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: gate,
            },
            &AllowFixtureAdapter,
        );
        assert_eq!(receipt.decision, ResidentToolReceiptDecisionV1::Denied);
        assert_eq!(receipt.gate_reason_code.as_deref(), Some("policy_denied"));
    }

    #[test]
    fn denial_receipt_hashes_secret_shaped_model_proposal_id() {
        let compiler = wp09_compiler_input_fixture("fixture.safe_read");
        let binding = authority(&compiler.proposal.tool_name);
        let role = compiler.policy_context.role.clone();
        let secret = "secret=/private/keys/operator-token";
        let mut proposal = compiler.proposal;
        proposal.proposal_id = secret.to_string();
        proposal.tool_name = "unauthorized.tool".to_string();
        let output = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
            tool_proposal: proposal,
        })
        .unwrap();
        let receipt = govern_resident_tool_output_v1(
            &output,
            ResidentToolExecutionContextV1 {
                resident_id: "actor.operator.alice",
                role: &role,
                authority: &binding,
                cycle_id: "cycle.redaction",
                checkpoint_lineage: "checkpoint.redaction",
                registry: compiler.registry,
                policy: compiler.policy_context,
                risk_class: "low",
                citizen_boundary_ref: "runtime.resident.boundary",
                gate_context: allowed_gate(),
            },
            &AllowFixtureAdapter,
        );
        let encoded = serde_json::to_string(&receipt).unwrap();
        assert_eq!(receipt.reason_code, "tool_not_authorized");
        assert!(!encoded.contains(secret));
        assert!(receipt
            .proposal_id
            .as_deref()
            .unwrap()
            .starts_with("sha256:"));
    }
}
