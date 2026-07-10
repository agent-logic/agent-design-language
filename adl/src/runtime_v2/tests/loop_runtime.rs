use super::*;

#[test]
fn runtime_v2_loop_runtime_contract_integrates_reasoning_graph() {
    let packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.validate().expect("valid loop runtime packet");

    assert_eq!(packet.schema_version, RUNTIME_V2_LOOP_RUNTIME_SCHEMA);
    assert_eq!(packet.reasoning_graph_id, "reasoning-graph-v0-91-7-wp-11");
    assert_eq!(packet.loop_definition.max_iterations, 4);
    assert_eq!(packet.replay.events.len(), 4);
    assert_eq!(
        packet.replay.final_state.status,
        RuntimeV2LoopStatus::Terminated
    );
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_LOOP_RUNTIME_TEST_MARKER)));
}

#[test]
fn runtime_v2_loop_runtime_rejects_missing_graph_state_binding() {
    let graph = runtime_v2_reasoning_graph_contract().expect("reasoning graph");
    let mut state = RuntimeV2LoopState::ready_for_graph(&graph);
    state.graph_id = "other-graph".to_string();

    assert!(runtime_v2_loop_runtime_contract_for_graph(&graph, state)
        .expect_err("state graph mismatch should fail")
        .to_string()
        .contains("missing graph/state binding"));
}

#[test]
fn runtime_v2_loop_runtime_rejects_invalid_loop_definitions() {
    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.loop_definition.steps[0].edge_id = "missing-edge".to_string();

    assert!(packet
        .validate()
        .expect_err("packet validation should catch replay/definition mismatch")
        .to_string()
        .contains("loop runtime"));

    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.loop_definition.max_iterations = 0;
    assert!(packet
        .validate()
        .expect_err("zero termination limit should fail")
        .to_string()
        .contains("max_iterations"));
}

#[test]
fn runtime_v2_loop_runtime_rejects_missing_graph_nodes() {
    let graph = runtime_v2_reasoning_graph_contract().expect("reasoning graph");
    let mut state = RuntimeV2LoopState::ready_for_graph(&graph);
    state.current_node_id = "missing-node".to_string();

    assert!(runtime_v2_loop_runtime_contract_for_graph(&graph, state)
        .expect_err("missing state node should fail")
        .to_string()
        .contains("missing graph node"));

    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.loop_definition.terminal_node_ids = vec!["missing-node".to_string()];
    assert!(packet
        .validate()
        .expect_err("missing terminal node should fail")
        .to_string()
        .contains("terminal node is missing"));
}

#[test]
fn runtime_v2_loop_runtime_enforces_termination_limits() {
    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.loop_definition.max_iterations = 3;

    assert!(packet
        .validate()
        .expect_err("steps beyond limit should fail")
        .to_string()
        .contains("termination limit"));

    let graph = runtime_v2_reasoning_graph_contract().expect("reasoning graph");
    let mut state = RuntimeV2LoopState::ready_for_graph(&graph);
    state.iteration = 4;
    assert!(runtime_v2_loop_runtime_contract_for_graph(&graph, state)
        .expect_err("state at limit should fail")
        .to_string()
        .contains("termination limit"));
}

#[test]
fn runtime_v2_loop_runtime_replay_order_is_deterministic() {
    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.loop_definition.terminal_node_ids.reverse();
    packet.validation_commands.reverse();
    packet.non_claims.reverse();

    let json = String::from_utf8(packet.pretty_json_bytes().expect("loop runtime json"))
        .expect("utf8 loop runtime json");
    let reparsed: RuntimeV2LoopRuntimePacket =
        serde_json::from_str(&json).expect("reparse loop runtime json");

    assert_eq!(reparsed.replay.events[0].event_sequence, 1);
    assert_eq!(reparsed.replay.events[0].step_id, "step-0001-propose");
    assert_eq!(
        reparsed.loop_definition.steps[0].step_id,
        "step-0001-propose"
    );
    reparsed.validate().expect("canonical packet remains valid");
}

#[test]
fn runtime_v2_loop_runtime_rejects_forged_replay_order_and_final_state() {
    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.replay.events.swap(0, 1);
    assert!(packet
        .validate()
        .expect_err("reordered replay should fail")
        .to_string()
        .contains("replay"));

    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.replay.final_state.status = RuntimeV2LoopStatus::Running;
    assert!(packet
        .validate()
        .expect_err("forged final state should fail")
        .to_string()
        .contains("final state"));
}

#[test]
fn runtime_v2_loop_runtime_rejects_invalid_resumed_state() {
    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet
        .initial_state
        .completed_step_ids
        .push("missing-step".to_string());
    assert!(packet
        .validate()
        .expect_err("unknown completed step should fail")
        .to_string()
        .contains("unknown completed step"));

    let mut packet = runtime_v2_loop_runtime_contract().expect("loop runtime packet");
    packet.initial_state.completed_step_ids = vec![
        "step-0001-propose".to_string(),
        "step-0003-decide".to_string(),
    ];
    assert!(packet
        .validate()
        .expect_err("non-prefix completed steps should fail")
        .to_string()
        .contains("deterministic prefix"));
}
