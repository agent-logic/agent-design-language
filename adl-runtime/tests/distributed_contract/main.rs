use adl_runtime::acip::{
    decode_protobuf_envelope, deterministic_json_to_protobuf, encode_semantic_envelope,
    protobuf_to_deterministic_json, websocket_frame_status, AcipEnvelopeInput,
};
use adl_runtime::qualification::{
    AcipVectorProbe, DistributedQualificationContract, ReceiptDecision, VectorOutcome,
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
