use std::collections::{BTreeMap, BTreeSet};

use adl_runtime_kernel::{
    build_production_operation_executors_with_recorder, AdapterKind, AgentPresence, AgentRoster,
    AgentRosterError, AgentRosterPolicy, AgentRosterQuery, AgentRuntimeEvidence, ClockAuthority,
    ComponentId, ControlAuthority, ControlService, KernelExit, LifecycleControl, OperationRequest,
    ResidentShepherdInitConfig, RunningState, RuntimeRecorder, OPERATION_REQUEST_SCHEMA,
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
        name: format!("{id}.runtime"),
        display_name: label.to_owned(),
        public_role: "resident agent".to_owned(),
        provider: None,
        model: None,
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

#[test]
fn canonical_name_is_projected_and_old_v1_entries_remain_readable() {
    let roster = AgentRoster::new(
        1,
        false,
        [evidence("shepherd", "Shepherd", AgentPresence::Ready)],
        [1; 32],
    )
    .unwrap();
    let entry = roster
        .detail(&policy(&["shepherd"]), "shepherd", 1_500)
        .unwrap();
    assert_eq!(entry.name, "shepherd.runtime");
    let mut old = serde_json::to_value(&entry).unwrap();
    old.as_object_mut().unwrap().remove("name");
    let decoded: adl_runtime_kernel::AgentRosterEntry = serde_json::from_value(old).unwrap();
    assert_eq!(decoded.name, "");
}

#[test]
fn production_shepherd_construction_uses_configured_canonical_name() {
    let config = ResidentShepherdInitConfig {
        name: "beacon.axioma".to_owned(),
        display_name: "Beacon".to_owned(),
        office: "resident shepherd".to_owned(),
        provider: "ollama".to_owned(),
        model: "qwen3:8b".to_owned(),
        endpoint: "http://127.0.0.1:11434".to_owned(),
        preload: Default::default(),
    };
    let feed = adl_runtime_kernel::AgentPopulationFeed::resident_shepherd_from_config(&config);
    let shepherd = &feed.sample[0];

    assert_eq!(shepherd.id, "shepherd");
    assert_eq!(shepherd.name, config.name);
    assert_eq!(shepherd.label, config.display_name);
    assert_eq!(shepherd.role, config.office);
}

#[test]
fn shepherd_model_health_and_readiness_consistency_are_projected_from_one_feed() {
    let recorder = RuntimeRecorder::new(16);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    assert!(recorder.record_agent_admission(
        "shepherd",
        now,
        now + 30_000,
        "1111111111111111111111111111111111111111"
    ));
    let service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder,
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        adl_runtime_kernel::AgentPopulationFeed::resident_shepherd_from_config(
            &ResidentShepherdInitConfig {
                name: "beacon.axioma".to_owned(),
                display_name: "Beacon".to_owned(),
                office: "resident shepherd".to_owned(),
                provider: "ollama".to_owned(),
                model: "qwen3:8b".to_owned(),
                endpoint: "http://127.0.0.1:11434".to_owned(),
                preload: Default::default(),
            },
        ),
    );
    service.update_resident_shepherd_health("beacon.axioma", "degraded", "retry scheduled");
    let feed = service.observatory_feed();
    let shepherd = &feed.agents.sample[0];
    assert_eq!(shepherd.state, "degraded");
    assert_eq!(shepherd.provider.as_deref(), Some("ollama"));
    assert_eq!(shepherd.model.as_deref(), Some("qwen3:8b"));
    assert!(!shepherd.communication_eligible);
    assert!(!service
        .readiness_report()
        .degraded_reasons
        .iter()
        .any(|reason| reason.contains("model")));
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
    let rotated = AgentRoster::new(
        9,
        false,
        [
            evidence("agent-a", "Alpha", AgentPresence::Ready),
            evidence("agent-b", "Beta", AgentPresence::Busy),
            evidence("agent-c", "Gamma", AgentPresence::Sleeping),
        ],
        [6; 32],
    )
    .unwrap();
    assert_eq!(
        rotated.page(
            &all,
            AgentRosterQuery {
                page_size: 1,
                page_token: Some(token.clone()),
                filter: None
            },
            1_500,
        ),
        Err(AgentRosterError::InvalidToken),
        "rotating the continuity-bound MAC key invalidates outstanding page tokens",
    );
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
fn event_cursors_bind_policy_query_and_exact_revision_successor() {
    let all = policy(&["agent-a"]);
    let first_roster = AgentRoster::new(
        20,
        false,
        [evidence("agent-a", "Alpha", AgentPresence::Ready)],
        [11; 32],
    )
    .unwrap();
    let first = first_roster
        .page(
            &all,
            AgentRosterQuery {
                page_size: 10,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    let next = AgentRoster::new(
        21,
        false,
        [evidence("agent-a", "Alpha", AgentPresence::Ready)],
        [11; 32],
    )
    .unwrap();
    let query = AgentRosterQuery {
        page_size: 10,
        page_token: None,
        filter: None,
    };
    assert!(next
        .page_after(&all, query.clone(), 1_500, Some(&first.event_cursor))
        .is_ok());
    assert_eq!(
        first_roster.page_after(&all, query.clone(), 1_500, Some(&first.event_cursor)),
        Err(AgentRosterError::TokenContextMismatch),
        "a cursor cannot be replayed at the same revision",
    );
    let skipped = AgentRoster::new(
        22,
        false,
        [evidence("agent-a", "Alpha", AgentPresence::Ready)],
        [11; 32],
    )
    .unwrap();
    assert_eq!(
        skipped.page_after(&all, query.clone(), 1_500, Some(&first.event_cursor)),
        Err(AgentRosterError::TokenContextMismatch),
        "revision gaps require a full snapshot resynchronization",
    );
    let changed_policy = AgentRosterPolicy {
        policy_subject: "operator:changed".to_owned(),
        ..all.clone()
    };
    assert_eq!(
        next.page_after(
            &changed_policy,
            query.clone(),
            1_500,
            Some(&first.event_cursor)
        ),
        Err(AgentRosterError::TokenContextMismatch),
    );
    let changed_query = AgentRosterQuery {
        page_size: 9,
        ..query
    };
    assert_eq!(
        next.page_after(&all, changed_query, 1_500, Some(&first.event_cursor)),
        Err(AgentRosterError::TokenContextMismatch),
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
    let started = std::time::Instant::now();
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
    assert!(
        started.elapsed() < std::time::Duration::from_secs(1),
        "the explicit 10,000-entry scan ceiling must remain operationally bounded",
    );
}

#[test]
fn roster_rejects_population_above_the_explicit_resource_bound() {
    let evidence = (0..=10_000).map(|index| {
        let id = format!("agent-{index:05}");
        evidence(&id, &id, AgentPresence::Ready)
    });
    assert!(matches!(
        AgentRoster::new(13, false, evidence, [9; 32]),
        Err(AgentRosterError::InvalidBounds)
    ));
}

#[test]
fn first_page_projects_only_the_requested_entries_in_stable_id_order() {
    let evidence = (0..10_000)
        .rev()
        .map(|index| {
            let id = format!("agent-{index:05}");
            evidence(&id, &format!("Label {index:05}"), AgentPresence::Ready)
        })
        .collect::<Vec<_>>();
    let visible_agent_ids = evidence.iter().map(|item| item.agent_id.clone()).collect();
    let roster = AgentRoster::new(14, false, evidence, [10; 32]).unwrap();
    let page = roster
        .page(
            &AgentRosterPolicy {
                policy_subject: "public-observatory".to_owned(),
                visible_agent_ids,
                reveal_capabilities: false,
                reveal_location: false,
            },
            AgentRosterQuery {
                page_size: 3,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(page.page_count, 3);
    assert_eq!(page.agents.len(), 3);
    assert_eq!(page.agents[0].id, "agent-00000");
    assert_eq!(page.agents[2].id, "agent-00002");
    assert!(page.has_more);
}

#[test]
fn relocation_preserves_identity_and_advances_the_authoritative_revision() {
    let mut before = evidence("agent-7", "Agent Seven", AgentPresence::Ready);
    before.location = Some("node-a".to_owned());
    let first = AgentRoster::new(10, false, [before], [8; 32]).unwrap();
    let first_page = first
        .page(
            &policy(&["agent-7"]),
            AgentRosterQuery {
                page_size: 1,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(first_page.agents[0].location.as_deref(), Some("node-a"));

    let mut relocated = evidence("agent-7", "Agent Seven", AgentPresence::Migrating);
    relocated.location = Some("node-b".to_owned());
    let second = AgentRoster::new(11, false, [relocated], [8; 32]).unwrap();
    let second_page = second
        .page(
            &policy(&["agent-7"]),
            AgentRosterQuery {
                page_size: 1,
                page_token: None,
                filter: None,
            },
            1_500,
        )
        .unwrap();
    assert_eq!(second_page.agents[0].id, first_page.agents[0].id);
    assert_eq!(first_page.revision, 10);
    assert_eq!(second_page.revision, 11);
    assert_eq!(second_page.agents[0].location.as_deref(), Some("node-b"));
    assert_eq!(second_page.agents[0].presence, AgentPresence::Migrating);
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
    let first_incarnation = absent.runtime_incarnation_id.clone();
    assert!(!first_incarnation.is_empty());
    assert_eq!(absent.agents.total_count, 0);
    assert!(absent.agents.sample.is_empty());
    assert!(!absent.agents.population_complete);

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    let merely_running = service.observatory_feed();
    assert_eq!(merely_running.agents.total_count, 0);
    assert!(merely_running.agents.sample.is_empty());

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let admitted_at = now - 10_000;
    let initial_deadline = now - 5_000;
    let source_revision = "0123456789abcdef0123456789abcdef01234567";
    assert!(recorder.record_agent_admission(
        "shepherd",
        admitted_at,
        initial_deadline,
        source_revision
    ));
    let stale = service.observatory_feed();
    assert_eq!(stale.agents.sample[0].state, "unknown");
    assert_eq!(stale.agents.sample[0].health, "stale");
    assert!(!stale.agents.sample[0].communication_eligible);

    let heartbeat_at = now + 1;
    let heartbeat_deadline = heartbeat_at + 5_000;
    assert!(recorder.record_agent_heartbeat("shepherd", heartbeat_at, heartbeat_deadline));
    let running = service.observatory_feed();
    assert_eq!(running.agents.total_count, 1);
    assert_eq!(running.agents.sample[0].id, "shepherd");
    assert_eq!(running.agents.sample[0].state, "ready");
    assert_eq!(running.agents.sample[0].health, "healthy");
    assert!(running.agents.sample[0].communication_eligible);
    assert_eq!(
        running.agents.sample[0].observed_at_unix_millis,
        heartbeat_at
    );
    assert_eq!(
        running.agents.sample[0].freshness_deadline_unix_millis,
        heartbeat_deadline
    );
    assert_eq!(running.agents.sample[0].source_revision, source_revision);
    let detail = service.agent_roster_detail("shepherd").unwrap();
    assert_eq!(detail.schema, "adl.runtime_v3.agent_roster_entry.v1");
    assert_eq!(detail.id, "shepherd");
    assert!(detail.capabilities.is_empty());
    assert_eq!(detail.location, None);
    assert!(service.agent_roster_detail("private-agent").is_err());
    let stable_id = running.agents.sample[0].id.clone();

    let polled_again = service.observatory_feed();
    assert_eq!(polled_again.runtime_incarnation_id, first_incarnation);
    assert_eq!(
        polled_again.agents.sample[0].observed_at_unix_millis, heartbeat_at,
        "polling must not renew admission freshness"
    );
    assert_eq!(
        polled_again.agents.sample[0].freshness_deadline_unix_millis, heartbeat_deadline,
        "polling must not extend the bounded heartbeat deadline"
    );

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Restarting);
    let restarting = service.observatory_feed();
    assert_eq!(restarting.agents.sample[0].id, stable_id);
    assert_eq!(restarting.agents.sample[0].state, "migrating");
    assert!(!restarting.agents.sample[0].communication_eligible);

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Degraded);
    let degraded = service.observatory_feed();
    assert_eq!(degraded.agents.sample[0].id, stable_id);
    assert_eq!(degraded.agents.sample[0].state, "degraded");
    assert!(!degraded.agents.sample[0].communication_eligible);

    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Failed);
    let unreachable = service.observatory_feed();
    assert_eq!(unreachable.agents.sample[0].id, stable_id);
    assert_eq!(unreachable.agents.sample[0].state, "unreachable");
    assert!(!unreachable.agents.sample[0].communication_eligible);

    let restarted_service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder,
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        adl_runtime_kernel::AgentPopulationFeed::resident_shepherd(),
    );
    assert_ne!(
        restarted_service.observatory_feed().runtime_incarnation_id,
        first_incarnation,
        "a new process incarnation must be distinguishable even when stable instance identity is reused"
    );
}

#[test]
fn detail_lookup_uses_exact_policy_visible_id_not_search_filter_semantics() {
    let recorder = RuntimeRecorder::new(16);
    let long_id = format!("agent-{}", "x".repeat(80));
    let mut misleading =
        adl_runtime_kernel::AgentPopulationFeed::resident_shepherd().sample[0].clone();
    misleading.id = "agent-a".to_owned();
    misleading.label = "target-agent helper".to_owned();
    let mut target = misleading.clone();
    target.id = "target-agent".to_owned();
    target.label = "Target".to_owned();
    let mut long = target.clone();
    long.id = long_id.clone();
    long.label = "Long identity".to_owned();
    for id in [&misleading.id, &target.id, &long.id] {
        recorder.set_component_state(ComponentId::new(id), RunningState::Running);
        assert!(recorder.record_agent_admission(
            id,
            1_000,
            u64::MAX,
            "0123456789abcdef0123456789abcdef01234567",
        ));
    }
    let service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder,
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        adl_runtime_kernel::AgentPopulationFeed {
            sample: vec![misleading, target, long],
            ..adl_runtime_kernel::AgentPopulationFeed::empty()
        }
        .with_public_policy(policy(&["agent-a", "target-agent", &long_id])),
    );
    assert_eq!(
        service.agent_roster_detail("target-agent").unwrap().id,
        "target-agent"
    );
    assert_eq!(service.agent_roster_detail(&long_id).unwrap().id, long_id);
}

#[test]
fn production_public_projection_omits_configured_but_unauthorized_agents_and_redacts_fields() {
    let recorder = RuntimeRecorder::new(16);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    for id in ["shepherd", "private-agent"] {
        recorder.set_component_state(ComponentId::new(id), RunningState::Running);
        assert!(recorder.record_agent_admission(
            id,
            now,
            now + 5_000,
            "0123456789abcdef0123456789abcdef01234567"
        ));
    }
    let mut feed = adl_runtime_kernel::AgentPopulationFeed::resident_shepherd();
    let mut private = feed.sample[0].clone();
    private.id = "private-agent".to_owned();
    private.label = "Private Agent".to_owned();
    private.capabilities = vec!["private-capability".to_owned()];
    private.location = Some("private-location".to_owned());
    feed.sample.push(private);
    let service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder,
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        feed.with_public_policy(AgentRosterPolicy {
            policy_subject: "public-observatory".to_owned(),
            visible_agent_ids: BTreeSet::from(["shepherd".to_owned()]),
            reveal_capabilities: false,
            reveal_location: false,
        }),
    );
    let public = service.observatory_feed();
    assert_eq!(public.agents.total_count, 1);
    assert_eq!(public.agents.sample[0].id, "shepherd");
    assert!(public.agents.sample[0].capabilities.is_empty());
    assert_eq!(public.agents.sample[0].location, None);
    let serialized = serde_json::to_string(&public.agents).unwrap();
    assert!(!serialized.contains("private-agent"));
    assert!(!serialized.contains("private-capability"));
    assert!(!serialized.contains("private-location"));
}

#[tokio::test]
async fn production_shepherd_operation_is_the_admission_authority() {
    let recorder = RuntimeRecorder::new(16);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    recorder.set_clock_authority(ClockAuthority::Authoritative {
        source: "test-qualified-clock".to_owned(),
        unix_millis: now,
    });
    recorder.set_component_state(ComponentId::new("shepherd"), RunningState::Running);
    let root = tempfile::tempdir().unwrap();
    let executors = build_production_operation_executors_with_recorder(
        root.path().join("operation-state"),
        recorder.clone(),
    )
    .unwrap();
    let request = OperationRequest {
        schema: OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: "admit-shepherd".to_owned(),
        idempotency_key: "admit-shepherd-once".to_owned(),
        principal: "runtime-bootstrap".to_owned(),
        payload: br#"{"schema":"adl.runtime.local_shepherd_admission.v1","admit":true}"#.to_vec(),
        permit: None,
    };
    executors[&AdapterKind::Shepherd]
        .execute(&request)
        .await
        .unwrap();

    let service = ControlService::new_with_observatory_config_and_agents(
        "runtime-instance",
        recorder,
        NoopLifecycle,
        ControlAuthority::new(BTreeMap::new()),
        8,
        std::iter::empty(),
        adl_runtime_kernel::AgentPopulationFeed::resident_shepherd(),
    );
    let feed = service.observatory_feed();
    assert_eq!(feed.agents.sample.len(), 1);
    assert_eq!(feed.agents.sample[0].id, "shepherd");
    assert_eq!(feed.agents.sample[0].state, "ready");
    assert_eq!(feed.agents.sample[0].source_revision.len(), 40);
    assert!(feed.agents.sample[0]
        .source_revision
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit()));
}
