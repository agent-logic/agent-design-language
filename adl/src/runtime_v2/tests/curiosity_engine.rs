use super::*;

#[test]
fn runtime_v2_curiosity_engine_contract_is_stable() {
    let packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.validate().expect("valid curiosity packet");

    assert_eq!(packet.schema_version, RUNTIME_V2_CURIOSITY_ENGINE_SCHEMA);
    assert_eq!(packet.milestone, "v0.91.7");
    assert_eq!(packet.wp, "WP-10");
    assert_eq!(packet.signals.len(), 2);
    assert_eq!(packet.proposals.len(), 2);
    assert!(packet.governance.freedom_gate_required);
    assert!(packet.governance.constructability_gate_required);
    assert!(packet
        .handoff
        .csm_component_followups
        .iter()
        .any(|value| value.contains("issue-5124")));
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_CURIOSITY_ENGINE_TEST_MARKER)));
}

#[test]
fn runtime_v2_curiosity_engine_canonical_json_is_deterministic() {
    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.signals.reverse();
    packet.proposals.reverse();
    packet.validation_commands.reverse();

    let json = String::from_utf8(packet.pretty_json_bytes().expect("curiosity json"))
        .expect("utf8 curiosity json");
    let reparsed: RuntimeV2CuriosityEnginePacket =
        serde_json::from_str(&json).expect("reparse curiosity json");

    assert_eq!(reparsed.signals[0].signal_id, "signal-capability-delta");
    assert_eq!(
        reparsed.proposals[0].proposal_id,
        "proposal-bounded-discovery-proof"
    );
    reparsed
        .validate()
        .expect("canonical curiosity packet remains valid");
}

#[test]
fn runtime_v2_curiosity_engine_rejects_ungated_governance() {
    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.governance.freedom_gate_required = false;

    assert!(packet
        .validate()
        .expect_err("Freedom Gate must be required")
        .to_string()
        .contains("Freedom Gate"));

    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.proposals[0]
        .gated_by
        .retain(|gate| gate != "constructability_anchor");
    assert!(packet
        .validate()
        .expect_err("Constructability gate must be required")
        .to_string()
        .contains("constructability_anchor"));
}

#[test]
fn runtime_v2_curiosity_engine_rejects_unbounded_or_orphan_proposals() {
    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.budget.max_external_actions = 1;
    assert!(packet
        .validate()
        .expect_err("external actions are not allowed in WP-10 proof")
        .to_string()
        .contains("external actions"));

    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.proposals[0].source_signal_id = "missing-signal".to_string();
    assert!(packet
        .validate()
        .expect_err("proposal must cite an existing signal")
        .to_string()
        .contains("missing source signal"));

    let mut packet = runtime_v2_curiosity_engine_contract().expect("curiosity engine packet");
    packet.proposals[0]
        .experiment_plan
        .push("extra step outside the budget".to_string());
    assert!(packet
        .validate()
        .expect_err("proposal must stay within budget")
        .to_string()
        .contains("max_experiment_steps"));
}
