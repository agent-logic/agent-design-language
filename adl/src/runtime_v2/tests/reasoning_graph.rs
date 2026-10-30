use super::*;

#[test]
fn runtime_v2_reasoning_graph_contract_is_stable() {
    let packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.validate().expect("valid reasoning graph packet");

    assert_eq!(packet.schema_version, RUNTIME_V2_REASONING_GRAPH_SCHEMA);
    assert_eq!(packet.milestone, "v0.91.7");
    assert_eq!(packet.wp, "WP-11");
    assert_eq!(packet.graph.nodes.len(), 5);
    assert_eq!(packet.graph.edges.len(), 4);
    assert!(packet
        .handoff
        .runtime_consumer_refs
        .iter()
        .any(|value| value == "adl/src/runtime_v2/moral_trace_schema.rs"));
    assert!(packet
        .handoff
        .obsmem_handoff_refs
        .iter()
        .any(|value| value.contains("obsmem/reasoning_graph")));
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_REASONING_GRAPH_TEST_MARKER)));
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim.contains("loop runtime sibling issue")));
}

#[test]
fn runtime_v2_reasoning_graph_canonical_json_is_deterministic() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.graph.nodes.reverse();
    packet.graph.edges.reverse();
    packet.validation_commands.reverse();

    let json = String::from_utf8(packet.pretty_json_bytes().expect("reasoning graph json"))
        .expect("utf8 reasoning graph json");
    let reparsed: RuntimeV2ReasoningGraphPacket =
        serde_json::from_str(&json).expect("reparse reasoning graph json");

    assert_eq!(reparsed.graph.nodes[0].node_id, "decision-0001");
    assert_eq!(
        reparsed.graph.edges[0].edge_id,
        "edge-decision-produces-outcome"
    );
    reparsed.validate().expect("canonical packet remains valid");
}

#[test]
fn runtime_v2_reasoning_graph_validation_rejects_missing_endpoint() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.graph.edges[0].to = "missing-node".to_string();

    assert!(packet
        .validate()
        .expect_err("missing endpoint should fail")
        .to_string()
        .contains("missing to node"));
}

#[test]
fn runtime_v2_reasoning_graph_validation_rejects_unsupported_refs() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.graph.nodes[0].trace_refs = vec!["runtime_v2/not-a-trace-uri".to_string()];
    assert!(packet
        .validate()
        .expect_err("bad trace ref should fail")
        .to_string()
        .contains("trace://"));

    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.handoff.runtime_consumer_refs = vec!["/tmp/runtime.rs".to_string()];
    assert!(packet
        .validate()
        .expect_err("absolute runtime consumer ref should fail")
        .to_string()
        .contains("repository-relative path"));
}

#[test]
fn runtime_v2_reasoning_graph_validation_rejects_claim_expansion() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet
        .non_claims
        .retain(|claim| !claim.contains("adl.skill.v1"));
    assert!(packet
        .validate()
        .expect_err("missing skill standard non-claim should fail")
        .to_string()
        .contains("non-claims"));

    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.claim_boundary = "WP-11 proves a complete reasoning engine.".to_string();
    assert!(packet
        .validate()
        .expect_err("expanded claim boundary should fail")
        .to_string()
        .contains("bounded"));
}

#[test]
fn runtime_v2_reasoning_graph_validation_rejects_unproven_decision_paths() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet
        .graph
        .edges
        .retain(|edge| edge.edge_kind != RuntimeV2ReasoningEdgeKind::Supports);
    assert!(packet
        .validate()
        .expect_err("unsupported hypothesis should fail")
        .to_string()
        .contains("evidence-supported"));

    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet
        .graph
        .edges
        .retain(|edge| edge.edge_kind != RuntimeV2ReasoningEdgeKind::Produces);
    assert!(packet
        .validate()
        .expect_err("orphan outcome should fail")
        .to_string()
        .contains("decision-produced"));
}

#[test]
fn runtime_v2_reasoning_graph_validation_rejects_wrong_edge_kinds() {
    let mut packet = runtime_v2_reasoning_graph_contract().expect("reasoning graph packet");
    packet.graph.edges[0].from = "evidence-0001".to_string();

    assert!(packet
        .validate()
        .expect_err("proposes edge from evidence should fail")
        .to_string()
        .contains("invalid Proposes endpoints"));
}
