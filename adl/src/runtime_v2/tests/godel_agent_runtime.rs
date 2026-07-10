use super::*;

#[test]
fn runtime_v2_godel_agent_runtime_supports_ten_independent_agents() {
    let packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    packet.validate().expect("valid Godel agent runtime packet");

    assert_eq!(packet.schema_version, RUNTIME_V2_GODEL_AGENT_RUNTIME_SCHEMA);
    assert_eq!(packet.agents.len(), 10);
    assert_eq!(packet.scheduling.max_concurrent_agents, 10);
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_GODEL_AGENT_RUNTIME_TEST_MARKER)));

    let provider_counts = runtime_v2_godel_provider_summary(&packet);
    assert_eq!(provider_counts.values().sum::<usize>(), 10);
    assert!(provider_counts.contains_key("local_qwen"));
    assert!(provider_counts.contains_key("bedrock_nova_pro"));
    assert!(provider_counts.contains_key("z_ai_glm"));
    assert!(provider_counts.contains_key("fable_5"));
    assert!(packet
        .provider_registry
        .iter()
        .any(|provider| provider.transport == "local_cli"));
    assert!(packet
        .provider_registry
        .iter()
        .any(|provider| provider.structured_json_mode == "prompt_based"));
}

#[test]
fn runtime_v2_godel_agent_runtime_binds_agents_to_runtime_graph_and_loop() {
    let packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    for agent in &packet.agents {
        assert_eq!(agent.reasoning_graph_id, packet.reasoning_graph_id);
        assert_eq!(agent.loop_runtime_id, packet.loop_runtime_id);
        assert_eq!(agent.lifecycle_state, "ready");
    }
    assert_eq!(
        packet.replay.scheduled_agent_count,
        packet.agents.len() as u32
    );
}

#[test]
fn runtime_v2_godel_agent_runtime_rejects_underprovisioned_agent_sets() {
    let graph = runtime_v2_reasoning_graph_contract().expect("reasoning graph");
    let loop_runtime = runtime_v2_loop_runtime_contract().expect("loop runtime");
    let err =
        runtime_v2_godel_agent_runtime_contract_for(9, &graph.graph_id, &loop_runtime.runtime_id)
            .expect_err("less than ten agents should fail");

    assert!(err.to_string().contains("agent count"));
}

#[test]
fn runtime_v2_godel_agent_runtime_rejects_provider_and_replay_mismatch() {
    let mut packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    packet.agents[0].provider_id = "missing_provider".to_string();
    assert!(packet
        .validate()
        .expect_err("missing provider should fail")
        .to_string()
        .contains("unknown provider"));

    let mut packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    packet.agents[0].model_ref = "hosted:bedrock/nova-pro".to_string();
    assert!(packet
        .validate()
        .expect_err("agent model ref must match provider target")
        .to_string()
        .contains("model_ref must match provider"));

    let mut packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    packet.replay.independent_agent_ids.pop();
    assert!(packet
        .validate()
        .expect_err("replay ids should match agents")
        .to_string()
        .contains("independent agent ids"));
}

#[test]
fn runtime_v2_godel_agent_runtime_rejects_schedule_agent_count_mismatch() {
    let mut packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    let mut extra_agent = packet.agents[0].clone();
    extra_agent.agent_instance_id = "godel-agent-11".to_string();
    extra_agent.initial_state_id = "godel-agent-11-state-0001".to_string();
    extra_agent.channel_id = "godel-agent-11-runtime-channel".to_string();
    extra_agent.evidence_root_ref =
        "runtime_v2/godel_agent_runtime/agents/godel-agent-11".to_string();
    packet.agents.push(extra_agent);
    packet.replay.scheduled_agent_count = packet.agents.len() as u32;
    packet
        .replay
        .independent_agent_ids
        .push("godel-agent-11".to_string());
    packet.scheduling.max_independent_agents = 10;
    assert!(packet
        .validate()
        .expect_err("actual agents above scheduling maximum should fail")
        .to_string()
        .contains("exceeds scheduling maximum"));

    let mut packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    packet.scheduling.max_concurrent_agents = 11;
    assert!(packet
        .validate()
        .expect_err("concurrency above actual agents should fail")
        .to_string()
        .contains("cannot exceed actual agent count"));
}

#[test]
fn runtime_v2_godel_agent_runtime_keeps_hosted_invocation_explicitly_bounded() {
    let packet = runtime_v2_godel_agent_runtime_contract().expect("Godel agent runtime packet");
    let hosted = packet
        .provider_registry
        .iter()
        .filter(|provider| provider.runtime_surface == "hosted_http")
        .collect::<Vec<_>>();

    assert!(hosted.len() >= 3);
    for provider in hosted {
        assert_eq!(
            provider.invocation_status,
            "provider_target_resolved_not_invoked"
        );
    }
    assert!(packet
        .non_claims
        .iter()
        .any(|claim| claim == "not_live_hosted_provider_invocation"));
}
