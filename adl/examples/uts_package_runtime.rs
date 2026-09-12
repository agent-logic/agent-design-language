//! #877 required PVF runtime proof: deterministic local real read-only adapter,
//! compatibility, ACC/policy/binding/replay denial; no network/provider access.
use adl::acc::AccGrantStatusV1;
use adl::freedom_gate::FreedomGateToolGateContextV1;
use adl::governed_executor::GovernedToolAdapterV1;
use adl::resident_tool_execution::*;
use adl::tool_registry::ToolRegistryV1;
use adl::uts_acc_compiler::{ToolProposalV1, UtsAccPolicyContextV1};
use adl_runtime::resident_agent::CsmResidentAgentToolAuthorityBinding;
use adl_uts::UtsSideEffectClassV1;
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

struct ObservedAdapter {
    real: RuntimeObserveAdapterV1,
    results: RefCell<Vec<Value>>,
}
impl GovernedToolAdapterV1 for ObservedAdapter {
    fn execute(&self, id: &str, arguments: &BTreeMap<String, Value>) -> Result<Value, String> {
        let result = self.real.execute(id, arguments)?;
        self.results.borrow_mut().push(result.clone());
        Ok(result)
    }
}
fn dispatch(
    registry: ToolRegistryV1,
    scenario: &str,
    adapter: &ObservedAdapter,
) -> (String, ResidentToolReceiptV1) {
    let mut policy = UtsAccPolicyContextV1 {
        actor_id: "actor.uts.proof".into(),
        role: "operator".into(),
        standing: "active".into(),
        authenticated: true,
        grant_id: "grant.uts.proof".into(),
        grantor_actor_id: "actor.uts.proof".into(),
        grant_status: AccGrantStatusV1::Active,
        delegation: None,
        allowed_side_effects: vec![UtsSideEffectClassV1::Read],
        allowed_resource_scopes: vec!["aggregate-observation".into()],
        allow_sensitive_data: false,
        visibility_constructible: true,
        replay_allowed: true,
        execution_approved: true,
    };
    if scenario == "policy_denied" {
        policy.execution_approved = false;
    }
    let authority = CsmResidentAgentToolAuthorityBinding::new(
        "grant.uts.proof",
        "runtime://resident/actor.uts.proof/tool-authority",
        vec!["runtime.observe".into()],
    );
    if scenario == "authority_mismatch" {
        policy.grant_id = "grant.other".into();
    }
    let proposal = serde_json::to_string(&ResidentToolProposalEnvelopeV1 {
        tool_proposal: ToolProposalV1 {
            proposal_id: "proposal.package-observe".into(),
            tool_name: "runtime.observe".into(),
            tool_version: "1.0.0".into(),
            adapter_id: RUNTIME_OBSERVE_ADAPTER_V1.into(),
            arguments: BTreeMap::new(),
            dry_run_requested: true,
            ambiguous: false,
        },
    })
    .unwrap();
    let mut prior = BTreeSet::new();
    if scenario == "replay_denied" {
        use sha2::{Digest, Sha256};
        prior.insert(format!(
            "sha256:{}",
            hex::encode(Sha256::digest(b"proposal.package-observe"))
        ));
    }
    let receipt = govern_resident_tool_output_v1(
        &proposal,
        ResidentToolExecutionContextV1 {
            resident_id: "actor.uts.proof",
            role: "operator",
            authority: &authority,
            cycle_id: "cycle.package-proof",
            checkpoint_lineage: "checkpoint.package-proof",
            registry,
            policy,
            risk_class: "low",
            citizen_boundary_ref: "runtime.resident.boundary",
            gate_context: FreedomGateToolGateContextV1 {
                policy_decision: if scenario == "gate_denied" {
                    "denied"
                } else {
                    "allowed"
                }
                .into(),
                requires_operator_review: false,
                requires_human_challenge: false,
                escalation_available: false,
                citizen_action_boundary_intact: true,
                operator_action_boundary_intact: true,
                private_arguments_redacted: true,
            },
            prior_proposal_ids: &prior,
        },
        adapter,
    );
    (proposal, receipt)
}
fn main() {
    let snapshot = json!({"kind":"runtime_observation", "status":"proof_read_only", "redaction":"aggregate_only"});
    let adapter = ObservedAdapter {
        real: RuntimeObserveAdapterV1::new(snapshot.clone()).unwrap(),
        results: RefCell::new(Vec::new()),
    };
    let base = serde_json::to_value(runtime_observe_registry_v1()).unwrap();
    let mut cases = Vec::new();
    for version in ["uts.v1", "uts.v1.1"] {
        let mut wire = base.clone();
        let tool = &mut wire["tools"][0]["uts"];
        if version == "uts.v1" {
            let object = tool.as_object_mut().unwrap();
            for key in [
                "compatible_versions",
                "categories",
                "side_effects",
                "observability",
                "planning",
            ] {
                object.remove(key);
            }
            object.insert("schema_version".into(), json!(version));
        }
        let registry: ToolRegistryV1 =
            serde_json::from_value(wire).expect("packaged declaration load");
        let before = adapter.results.borrow().len();
        let (invocation, receipt) = dispatch(registry, "accepted", &adapter);
        assert_eq!(
            receipt.decision,
            ResidentToolReceiptDecisionV1::Executed,
            "{receipt:?}"
        );
        assert!(receipt.acc_contract_id.is_some());
        assert_eq!(adapter.results.borrow().len(), before + 1);
        assert_eq!(adapter.results.borrow().last(), Some(&snapshot));
        cases.push(json!({"case":version,"invocation":serde_json::from_str::<Value>(&invocation).unwrap(),"receipt":receipt,"real_result":snapshot}));
    }
    for scenario in [
        "unsupported_version",
        "malformed",
        "binding_mismatch",
        "side_effect_denied",
        "sensitive_data_denied",
        "authority_mismatch",
        "policy_denied",
        "gate_denied",
        "replay_denied",
    ] {
        let before = adapter.results.borrow().len();
        let mut wire = base.clone();
        match scenario {
            "unsupported_version" => wire["tools"][0]["uts"]["schema_version"] = json!("uts.v2"),
            "malformed" => wire["tools"][0]["uts"]["name"] = json!("!"),
            "binding_mismatch" => wire["adapters"][0]["tool_version"] = json!("99.0.0"),
            "side_effect_denied" => {
                wire["tools"][0]["uts"]["side_effect_class"] = json!("local_write");
                wire["tools"][0]["uts"]["categories"] = json!(["state_mutating"]);
                wire["tools"][0]["uts"]["side_effects"] = json!(["local_state"]);
                wire["adapters"][0]["side_effect_class"] = json!("local_write");
            }
            "sensitive_data_denied" => {
                wire["tools"][0]["uts"]["data_sensitivity"] = json!("secret")
            }
            _ => {}
        }
        let result = match serde_json::from_value::<ToolRegistryV1>(wire) {
            Ok(registry) => {
                let (_, receipt) = dispatch(registry, scenario, &adapter);
                assert_eq!(
                    receipt.decision,
                    ResidentToolReceiptDecisionV1::Denied,
                    "{scenario}: {receipt:?}"
                );
                json!({"receipt":receipt})
            }
            Err(error) => {
                assert!(matches!(scenario, "unsupported_version" | "malformed"));
                json!({"load_rejection":error.to_string()})
            }
        };
        assert_eq!(
            adapter.results.borrow().len(),
            before,
            "denial must precede effects"
        );
        cases.push(json!({"case":scenario,"result":result,"adapter_calls":0}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({"package":"adl-uts","package_version":adl_uts::PACKAGE_VERSION,"cases":cases,"real_tool_calls":adapter.results.borrow().len(),"provider_calls":0})).unwrap());
}
