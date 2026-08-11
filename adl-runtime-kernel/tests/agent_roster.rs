use std::collections::{BTreeMap, BTreeSet};

use adl_runtime_kernel::{
    AgentPresence, AgentRoster, AgentRosterError, AgentRosterPolicy, AgentRosterQuery,
    AgentRuntimeEvidence, ComponentId, ControlAuthority, ControlService, KernelExit,
    LifecycleControl, RunningState, RuntimeRecorder,
};

struct NoopLifecycle;

#[async_trait::async_trait]
impl LifecycleControl for NoopLifecycle {
    async fn shutdown(&self, _grace: std::time::Duration) -> Result<KernelExit, ()> {
        Ok(KernelExit::Clean)
    }
}

fn evidence(id: &str, label: &str, presence: AgentPresence) -> AgentRuntimeEvidence {
    AgentRuntimeEvidence {
        agent_id: id.to_owned(),
        display_name: label.to_owned(),
        public_role: "resident agent".to_owned(),
        presence,
        health: "healthy".to_owned(),
        availability: "available".to_owned(),
        activity: Some("governed work".to_owned()),
        capabilities: vec!["conversation".to_owned()],
        location: Some("local".to_owned()),
        communication_eligible: true,
        observed_at_unix_millis: 1_000,
        freshness_deadline_unix_millis: 2_000,
        source_revision: "runtime-revision-7".to_owned(),
        provenance: "runtime_component_state".to_owned(),
    }
}

fn policy(ids: &[&str]) -> AgentRosterPolicy {
    AgentRosterPolicy {
        policy_subject: "operator:local".to_owned(),
        visible_agent_ids: ids.iter().map(|id| (*id).to_owned()).collect(),
        reveal_capabilities: true,
        reveal_location: true,
    }
}

#[test]
fn roster_is_policy_filtered_before_serialization_and_never_claims_global_completeness() {
    let roster = AgentRoster::new(
        7,
        false,
        [
            evidence("shepherd", "Shepherd", AgentPresence::Ready),
            evidence("private-agent", "Private", AgentPresence::Busy),
        ],
        [3; 32],
    )
    .unwrap();
    let page = roster
        .page(
            &policy(&["shepherd"]),
            AgentRosterQuery {
                page_size: 10,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(page.visible_count, 1);
    assert_eq!(page.agents[0].id, "shepherd");
    assert!(!page.population_complete);
    let json = serde_json::to_string(&page).unwrap();
    assert!(!json.contains("private-agent"));
    assert!(!json.contains("Private"));
}

#[test]
fn stale_presence_fails_closed_without_changing_stable_identity() {
    let roster = AgentRoster::new(
        8,
        false,
        [evidence("shepherd", "Shepherd", AgentPresence::Ready)],
        [4; 32],
    )
    .unwrap();
    let entry = roster
        .detail(&policy(&["shepherd"]), "shepherd", 2_001)
        .unwrap();
    assert_eq!(entry.id, "shepherd");
    assert_eq!(entry.presence, AgentPresence::Unknown);
    assert_eq!(entry.health, "stale");
    assert!(!entry.communication_eligible);
}

#[test]
fn every_declared_presence_state_round_trips() {
    let states = [
        AgentPresence::Ready,
        AgentPresence::Busy,
        AgentPresence::Sleeping,
        AgentPresence::Degraded,
        AgentPresence::Unreachable,
        AgentPresence::Migrating,
        AgentPresence::Unknown,
    ];
    for (index, expected) in states.into_iter().enumerate() {
        let id = format!("agent-{index}");
        let roster =
            AgentRoster::new(1, false, [evidence(&id, &id, expected)], [index as u8; 32]).unwrap();
        assert_eq!(
            roster.detail(&policy(&[&id]), &id, 1_500).unwrap().presence,
            expected
        );
    }
}

#[test]
fn page_tokens_bind_revision_policy_filter_and_page_size() {
    let roster = AgentRoster::new(
        9,
        false,
        [
            evidence("agent-a", "Alpha", AgentPresence::Ready),
            evidence("agent-b", "Beta", AgentPresence::Busy),
            evidence("agent-c", "Gamma", AgentPresence::Sleeping),
        ],
        [5; 32],
    )
    .unwrap();
    let all = policy(&["agent-a", "agent-b", "agent-c"]);
    let first = roster
        .page(
            &all,
            AgentRosterQuery {
                page_size: 1,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(first.agents[0].id, "agent-a");
    assert!(first.has_more);
    let token = first.next_page_token.unwrap();
    let second = roster
        .page(
            &all,
            AgentRosterQuery {
                page_size: 1,
                page_token: Some(token.clone()),
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(second.agents[0].id, "agent-b");
    assert_eq!(
        roster.page(
            &all,
            AgentRosterQuery {
                page_size: 2,
                page_token: Some(token.clone()),
                filter: None
            },
            1_500,
        ),
        Err(AgentRosterError::TokenContextMismatch)
    );
    let narrowed = policy(&["agent-a", "agent-b"]);
    assert_eq!(
        roster.page(
            &narrowed,
            AgentRosterQuery {
                page_size: 1,
                page_token: Some(token),
                filter: None
            },
            1_500,
        ),
        Err(AgentRosterError::TokenContextMismatch)
    );
}

#[test]
fn tampered_tokens_and_unbounded_queries_fail_closed() {
    let roster = AgentRoster::new(
        10,
        false,
        [evidence("shepherd", "Shepherd", AgentPresence::Ready)],
        [6; 32],
    )
    .unwrap();
    assert_eq!(
        roster.page(
            &policy(&["shepherd"]),
            AgentRosterQuery {
                page_size: 0,
                page_token: None,
                filter: None
            },
            1_500,
        ),
        Err(AgentRosterError::InvalidBounds)
    );
    assert_eq!(
        roster.page(
            &policy(&["shepherd"]),
            AgentRosterQuery {
                page_size: 1,
                page_token: Some("00.invalid".to_owned()),
                filter: None,
            },
            1_500,
        ),
        Err(AgentRosterError::InvalidToken)
    );
}

#[test]
fn policy_redacts_capabilities_and_location_without_client_cooperation() {
    let roster = AgentRoster::new(
        11,
        false,
        [evidence("shepherd", "Shepherd", AgentPresence::Ready)],
        [7; 32],
    )
    .unwrap();
    let restricted = AgentRosterPolicy {
        policy_subject: "operator:restricted".to_owned(),
        visible_agent_ids: BTreeSet::from(["shepherd".to_owned()]),
        reveal_capabilities: false,
        reveal_location: false,
    };
    let entry = roster.detail(&restricted, "shepherd", 1_500).unwrap();
    assert!(entry.capabilities.is_empty());
    assert_eq!(entry.location, None);
}

#[test]
fn large_local_roster_remains_page_bounded_and_deterministic() {
    let evidence = (0..10_000)
        .map(|index| {
            let id = format!("agent-{index:05}");
            evidence(&id, &id, AgentPresence::Ready)
        })
        .collect::<Vec<_>>();
    let visible_agent_ids = evidence.iter().map(|item| item.agent_id.clone()).collect();
    let roster = AgentRoster::new(12, false, evidence, [8; 32]).unwrap();
    let page = roster
        .page(
            &AgentRosterPolicy {
                policy_subject: "operator:scale".to_owned(),
                visible_agent_ids,
                reveal_capabilities: false,
                reveal_location: false,
            },
            AgentRosterQuery {
                page_size: 100,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(page.visible_count, 10_000);
    assert_eq!(page.page_count, 100);
    assert_eq!(page.agents.first().unwrap().id, "agent-00000");
    assert_eq!(page.agents.last().unwrap().id, "agent-00099");
    assert!(page.has_more);
    assert!(serde_json::to_vec(&page).unwrap().len() < 80_000);
}

#[test]
fn production_feed_admits_shepherd_only_from_current_runtime_component_truth() {
    let recorder = RuntimeRecorder::new(16);
    let service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder.clone(),
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        adl_runtime_kernel::AgentPopulationFeed::resident_shepherd(),
    );

    let absent = service.observatory_feed();
    assert_eq!(absent.agents.total_count, 0);
    assert!(absent.agents.sample.is_empty());
    assert!(!absent.agents.population_complete);

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    let running = service.observatory_feed();
    assert_eq!(running.agents.total_count, 1);
    assert_eq!(running.agents.sample[0].id, "shepherd");
    assert_eq!(running.agents.sample[0].state, "ready");
    assert_eq!(running.agents.sample[0].health, "healthy");
    assert!(running.agents.sample[0].communication_eligible);
    let stable_id = running.agents.sample[0].id.clone();

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Restarting);
    let restarting = service.observatory_feed();
    assert_eq!(restarting.agents.sample[0].id, stable_id);
    assert_eq!(restarting.agents.sample[0].state, "migrating");
    assert!(!restarting.agents.sample[0].communication_eligible);
}
