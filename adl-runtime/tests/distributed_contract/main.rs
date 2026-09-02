use adl_runtime::acip::{
    decode_protobuf_envelope, deterministic_json_to_protobuf, encode_semantic_envelope,
    protobuf_to_deterministic_json, websocket_frame_status, AcipEnvelopeInput,
};
use adl_runtime::qualification::{
    AcipVectorProbe, DistributedQualificationContract, DrtBQualificationContract, ReceiptDecision,
    VectorOutcome,
};
use serde_json::{json, Value};

fn contract() -> DistributedQualificationContract {
    DistributedQualificationContract::deterministic_drt_a()
}

#[test]
fn qualification_contract() {
    let contract = contract();
    contract.validate_topology().expect("DRT-A topology");
    contract.validate_scenarios().expect("DRT-A scenarios");
    let receipt = contract
        .receipt_for("qualification-contract")
        .expect("qualification receipt");
    assert_eq!(receipt.status, "pass");
    assert_eq!(receipt.requirement_count, 2);
    assert_eq!(receipt.participant_count, 8);
    assert_eq!(receipt.scenario_count, 11);
    let baseline = contract
        .vector_by_id("positive-roundtrip")
        .expect("positive vector")
        .authority_digest
        .clone();
    let exact = contract
        .scenario_receipt(
            "qualification-contract",
            "duplicate-denial",
            &baseline,
            ReceiptDecision::Denied,
        )
        .expect("exact scenario receipt");
    println!("DRT_A_CONTRACT_DIGEST={}", contract.digest());
    println!(
        "DRT_A_DUPLICATE_DENIAL_RECEIPT={}",
        serde_json::to_string(&exact).expect("receipt json")
    );
    assert_eq!(exact.schema, "adl.runtime.qualification.drt_a_receipt.v1");
    assert_eq!(exact.scenario, "duplicate-denial");
    assert_eq!(exact.authority_digest, baseline);
    assert_eq!(exact.decision, ReceiptDecision::Denied);
    assert!(exact.cleanup.contains("duplicate-denial"));
}

#[test]
fn acip_authority() {
    let contract = contract();
    contract.validate_acip_vectors().expect("ACIP vectors");
    let first = encode_fixture(42, json!({"z": 1, "a": {"b": true}}));
    let second = encode_fixture(42, json!({"a": {"b": true}, "z": 1}));
    assert_eq!(first, second, "ACIP encoding must be byte-stable");
    let decoded = decode_protobuf_envelope(&first).expect("decode");
    assert_eq!(decoded.source, "agent-alpha");
    assert_eq!(decoded.target, "agent-beta");
    assert_eq!(decoded.authority, "runtime-api-authenticated");
    assert_eq!(decoded.capability, "drt-a.invoke");
    assert_eq!(decoded.correlation_id, "drt-a-correlation-42");
    assert_eq!(decoded.causation_id, "drt-a-causation-42");
    assert_eq!(decoded.replay_id, "drt-a:agent-alpha:42");
    assert_eq!(
        decoded.required_features,
        vec!["authority-context", "replay-identity"]
    );
}

#[test]
fn replay_conformance() {
    let contract = contract();
    let encoded = encode_fixture(42, json!({"kind": "drt-a", "step": "replay"}));
    let projected = protobuf_to_deterministic_json(&encoded).expect("project");
    let restored = deterministic_json_to_protobuf(&projected).expect("restore");
    assert_eq!(restored, encoded, "replay projection must be exact");
    let parsed: Value = serde_json::from_str(&projected).expect("projection json");
    assert_eq!(parsed["monotonic_sequence"], "42");
    let duplicate = contract
        .acip_probe_for("duplicate")
        .expect("duplicate probe");
    assert!(
        duplicate.seen_message_ids.contains(&duplicate.message_id),
        "duplicate probe must carry prior message state"
    );
    let duplicate = contract
        .evaluate_acip_probe(&duplicate)
        .expect("duplicate-denial receipt");
    println!(
        "DRT_A_DUPLICATE_VECTOR_RECEIPT={}",
        serde_json::to_string(&duplicate).expect("duplicate vector receipt")
    );
    assert_eq!(duplicate.decision, ReceiptDecision::Denied);
    assert_eq!(duplicate.scenario, "duplicate");
    assert_eq!(duplicate.mutation, "message-id-repeat");
    assert_eq!(
        duplicate.authority_digest,
        contract
            .vector_by_id("positive-roundtrip")
            .expect("positive vector")
            .authority_digest
    );
}

#[test]
fn negative_matrix() {
    let contract = contract();
    contract.validate_acip_vectors().expect("ACIP vectors");
    for id in [
        "duplicate",
        "reordered",
        "stale",
        "malformed",
        "unsigned",
        "wrong-domain",
        "cross-polis",
        "authority-mutation",
        "credential-binding",
        "permit-binding",
        "correlation-binding",
        "causation-binding",
    ] {
        let vector = contract.vector_by_id(id).expect("negative vector");
        assert_eq!(vector.expected, VectorOutcome::Denied, "{id} must deny");
        let probe = contract.acip_probe_for(id).expect("negative probe");
        let receipt = contract
            .evaluate_acip_probe(&probe)
            .expect("negative receipt");
        assert_eq!(receipt.decision, ReceiptDecision::Denied, "{id} must deny");
        assert_eq!(receipt.scenario, id);
        assert_eq!(receipt.mutation, vector.mutation);

        let repaired = repair_negative_probe(&contract, probe);
        let err = contract
            .evaluate_acip_probe(&repaired)
            .expect_err("repaired negative probe must not be label-denied");
        assert!(
            err.contains("contained no invalid condition"),
            "{id} repaired probe failed for unexpected reason: {err}"
        );
    }

    assert_eq!(
        websocket_frame_status(&encode_fixture(7, json!({"ok": true})), false)["status"],
        "rejected"
    );
    assert_eq!(
        websocket_frame_status(&[], true)["status"],
        "rejected",
        "malformed input must fail closed"
    );
}

#[test]
fn drt_b_six_resident_uts() {
    let drt_a = contract();
    let drt_b = drt_a.deterministic_drt_b().expect("DRT-B contract");
    drt_b.validate().expect("DRT-B validates");

    assert_eq!(drt_b.resident_count, 6);
    assert_eq!(drt_b.residents.len(), 6);
    let resident_ids = drt_b
        .residents
        .iter()
        .map(|resident| resident.resident_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let receipt_ids = drt_b
        .residents
        .iter()
        .map(|resident| resident.workload_receipt_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(resident_ids.len(), 6, "resident IDs must be distinct");
    assert_eq!(
        receipt_ids.len(),
        6,
        "workload receipt IDs must be distinct"
    );
    assert!(drt_b
        .residents
        .iter()
        .all(|resident| resident.workload_receipt_id.len() == 64));
    assert_eq!(drt_b.requirements, ["#183", "#184"]);

    println!("DRT_B_CONTRACT_DIGEST={}", drt_b.digest());
    println!(
        "DRT_B_CONTRACT_JSON={}",
        serde_json::to_string_pretty(&drt_b).expect("DRT-B json")
    );
    let retained: DrtBQualificationContract = serde_json::from_str(include_str!(
        "../../../docs/milestones/v0.92.1/evidence/runtime/drt-b/qualification-contract.json"
    ))
    .expect("retained DRT-B evidence json");
    assert_eq!(
        retained, drt_b,
        "retained evidence must match code contract"
    );
}

#[test]
fn drt_b_continuity_reclamation() {
    let drt_b = contract().deterministic_drt_b().expect("DRT-B contract");
    drt_b.validate().expect("DRT-B validates");

    let dehydrated = serde_json::to_string(&drt_b).expect("dehydrate DRT-B");
    let restored: DrtBQualificationContract =
        serde_json::from_str(&dehydrated).expect("restore DRT-B");
    assert_eq!(restored, drt_b, "dehydrate/restore must be exact");
    assert_eq!(restored.dehydrate_restore, "exact");
    assert!(restored.cleanup_zero);
    assert_eq!(restored.resource_envelope["resident_slots"], 6);
    assert_eq!(restored.resource_envelope["workload_receipts"], 6);
    assert!(restored.cleanup_selectors.len() >= 3);
    assert!(restored
        .negative_matrix
        .iter()
        .all(|case| case.decision == "fail_closed"));
}

#[test]
fn drt_d_gcp_portability() {
    let retained_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json");
    let retained: Value = serde_json::from_str(
        &std::fs::read_to_string(&retained_path).expect("retained DRT-D qualification json"),
    )
    .expect("retained DRT-D qualification schema");

    assert_eq!(
        retained["schema"],
        "adl.v0921.drt_d.gcp_portability_qualification.v1"
    );
    assert_eq!(retained["issue"], 509);
    assert_eq!(retained["status"], "passed");
    assert_eq!(retained["paid_authorization"], true);
    assert_eq!(retained["reviewed_dependencies"]["494"], "terminal");
    assert_eq!(retained["reviewed_dependencies"]["495"], "terminal");
    assert_eq!(retained["reviewed_dependencies"]["508"], "terminal");
    assert_eq!(retained["topology"]["node_count"], 2);
    assert_eq!(retained["topology"]["ollama_public"], false);
    assert_eq!(retained["provider"]["kind"], "ollama");
    assert_eq!(
        retained["provider"]["runtime_surface"],
        "gcp_private_ollama_http"
    );
    assert_eq!(retained["provider"]["model_source"], "gcs_object_storage");
    assert!(retained["provider"]["artifact_manifest_sha256"]
        .as_str()
        .is_some_and(|digest| {
            digest.len() == 64 && digest.chars().all(|ch| ch.is_ascii_hexdigit())
        }));
    assert_eq!(
        retained["provider"]["models"],
        json!(["llama3.1:8b", "qwen3:8b", "phi4-mini:latest"])
    );
    assert_eq!(retained["aws_qualification_authority"], "unchanged");
    if let Ok(expected_revision) = std::env::var("ADL_DRT_D_EXPECTED_SOURCE_REVISION") {
        assert_eq!(retained["source_revision"], expected_revision);
    }

    let residents = retained["residents"].as_array().expect("resident array");
    assert_eq!(residents.len(), 6);
    let identities = residents
        .iter()
        .map(|resident| resident["identity"].as_str().expect("identity"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), 6, "resident identities must be unique");
    assert!(residents.iter().all(|resident| {
        resident["workload_completed"] == true
            && resident["receipt"]["decision"] == "executed"
            && resident["model"].as_str().is_some()
    }));

    assert_eq!(
        retained["dehydrated_population_digest"],
        retained["restored_population_digest"]
    );
    assert_eq!(retained["cost"]["currency"], "USD");
    assert_eq!(retained["cost"]["actual_cost_available"], false);
    assert!(retained["cost"]["actual_cost_usd"].is_null());
    assert!(retained["cost"]["max_budget_usd"]
        .as_f64()
        .is_some_and(|budget| budget > 0.0));
    assert!(retained["cost"]["method"]
        .as_str()
        .is_some_and(|method| method.contains("bounded-budget")));
    assert_eq!(retained["cleanup"]["runtime_instance"], "absent");
    assert_eq!(retained["cleanup"]["ollama_instance"], "absent");
    assert_eq!(retained["cleanup"]["run_selector"], "absent");
}

fn repair_negative_probe(
    contract: &DistributedQualificationContract,
    mut probe: AcipVectorProbe,
) -> AcipVectorProbe {
    let positive = contract
        .vector_by_id("positive-roundtrip")
        .expect("positive vector");
    probe.message_id = "drt-a-message-42".to_string();
    probe.seen_message_ids.clear();
    probe.authority_digest = positive.authority_digest.clone();
    probe.credential = "credential:adl:runtime:agent-alpha:v1".to_string();
    probe.permit = "permit:adl:runtime:agent-alpha:drt-a:v1".to_string();
    probe.signed = true;
    probe.domain = "runtime-api-authenticated".to_string();
    probe.polis_id = "polis-drt-a".to_string();
    probe.term = 7;
    probe.monotonic_sequence = 42;
    probe.correlation_id = "drt-a-correlation-42".to_string();
    probe.causation_id = "drt-a-causation-42".to_string();
    probe.payload_well_formed = true;
    probe
}

fn encode_fixture(sequence: u64, payload: Value) -> Vec<u8> {
    let replay_id = format!("drt-a:agent-alpha:{sequence}");
    let correlation_id = format!("drt-a-correlation-{sequence}");
    let causation_id = format!("drt-a-causation-{sequence}");
    let trace_id = format!("drt-a-trace-{sequence}");
    encode_semantic_envelope(
        AcipEnvelopeInput {
            message_id: "drt-a-message",
            source: "agent-alpha",
            target: "agent-beta",
            route: "drt-a.invoke",
            runtime_id: "drt-a-runtime",
            correlation_id: &correlation_id,
            causation_id: &causation_id,
            trace_id: &trace_id,
            replay_id: &replay_id,
            capability: "drt-a.invoke",
            authority: "runtime-api-authenticated",
            payload_type: "application/json",
            monotonic_sequence: sequence,
            acknowledgement_requested: true,
            error_code: None,
            required_features: &["authority-context", "replay-identity"],
        },
        &payload,
    )
    .expect("encode fixture")
}
