use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    AgentPresence, AgentRoster, AgentRosterEntry, AgentRosterPolicy, AgentRosterQuery,
    AgentRuntimeEvidence, BootstrapEvent, ComponentId, InferenceReadinessState, LifecycleState,
    ResidentShepherdInitConfig, ResidentShepherdSetInitConfig, RunningState, RuntimeSnapshot,
    WeatherHealthReport, AGENT_ROSTER_PAGE_SCHEMA,
};

pub const RUNTIME_READINESS_SCHEMA: &str = "adl.runtime_v3.readiness.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryControlFeed {
    pub port: u16,
    pub public_base_url: String,
    pub read_endpoint: String,
    pub websocket_endpoint: String,
    pub websocket_full_duplex: bool,
    pub websocket_acip_binary_schema: String,
    pub signed_command_endpoint: String,
    pub signed_commands_required_for_mutation: bool,
    pub bearer_token_required_for_read: bool,
    pub login_required_for_mutation: bool,
    pub browser_mutation_authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryWeatherFreshness {
    pub observed_at_unix_millis: u64,
    pub age_millis: u64,
    pub stale_after_millis: u64,
    pub stale: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ObservedWeather {
    pub(super) report: WeatherHealthReport,
    pub(super) observed_at_unix_millis: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryHealthFeed {
    pub snapshot: RuntimeSnapshot,
    pub observability_ready: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeReadinessReport {
    pub schema: String,
    pub ready: bool,
    pub lifecycle: LifecycleState,
    pub observability_ready: bool,
    pub runtime_instance_id: String,
    pub runtime_incarnation_id: String,
    pub runtime_process_id: u32,
    pub guardian_process_id: u32,
    pub active_init_hash: String,
    pub weather_freshness: Option<ObservatoryWeatherFreshness>,
    pub degraded_reasons: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryContinuityFeed {
    pub checkpoint: Option<crate::ContinuityHead>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentPopulationFeed {
    pub schema: String,
    pub revision: u64,
    pub scope: String,
    pub total_count: u64,
    pub rendered_sample_count: u64,
    pub has_more: bool,
    pub next_page_token: Option<String>,
    pub event_cursor: Option<String>,
    pub population_complete: bool,
    pub sample: Vec<AgentSample>,
    #[serde(skip)]
    pub public_policy: Option<AgentRosterPolicy>,
}

impl AgentPopulationFeed {
    pub fn empty() -> Self {
        Self {
            schema: AGENT_ROSTER_PAGE_SCHEMA.to_owned(),
            revision: 0,
            scope: "local_runtime".to_owned(),
            total_count: 0,
            rendered_sample_count: 0,
            has_more: false,
            next_page_token: None,
            event_cursor: None,
            population_complete: false,
            sample: Vec::new(),
            public_policy: None,
        }
    }

    pub fn resident_shepherd() -> Self {
        Self::resident_shepherd_named("shepherd.runtime", "Shepherd", "resident shepherd")
    }

    pub fn resident_shepherd_from_config(config: &ResidentShepherdInitConfig) -> Self {
        Self::resident_shepherds_from_config(&ResidentShepherdSetInitConfig::One(config.clone()))
    }

    pub fn resident_shepherds_from_config(configs: &ResidentShepherdSetInitConfig) -> Self {
        let mut feed = Self::empty();
        for (index, config) in configs.iter().enumerate() {
            let id = if index == 0 {
                "shepherd".to_owned()
            } else {
                format!("shepherd:{}", config.name)
            };
            let readiness = InferenceReadinessState::ModelLoading;
            let projection = readiness.projection();
            feed.sample.push(AgentSample {
                id: id.clone(),
                name: config.name.clone(),
                label: config.display_name.clone(),
                role: config.office.clone(),
                provider: Some(config.provider.clone()),
                model: Some(config.model.clone()),
                inference_readiness: readiness,
                state: readiness.as_str().to_owned(),
                detail: "Provider model preload pending".to_owned(),
                health: projection.health.to_owned(),
                availability: projection.availability.to_owned(),
                activity: projection.activity.map(str::to_owned),
                capabilities: vec!["conversation".to_owned()],
                location: Some("local_runtime".to_owned()),
                communication_eligible: projection.communication_eligible,
                observed_at_unix_millis: 0,
                freshness_deadline_unix_millis: 0,
                source_revision: "configured".to_owned(),
                provenance: "runtime_resident_shepherd".to_owned(),
            });
            feed.public_policy
                .get_or_insert_with(|| AgentRosterPolicy {
                    policy_subject: "public-observatory".to_owned(),
                    visible_agent_ids: BTreeSet::new(),
                    reveal_capabilities: false,
                    reveal_location: false,
                })
                .visible_agent_ids
                .insert(id);
        }
        feed.total_count = feed.sample.len() as u64;
        feed.rendered_sample_count = feed.total_count;
        feed.population_complete = true;
        feed
    }

    pub fn resident_shepherd_named(
        name: impl Into<String>,
        label: impl Into<String>,
        role: impl Into<String>,
    ) -> Self {
        Self {
            sample: vec![AgentSample {
                id: "shepherd".to_owned(),
                name: name.into(),
                label: label.into(),
                role: role.into(),
                provider: None,
                model: None,
                inference_readiness: InferenceReadinessState::Unimplemented,
                state: "unknown".to_owned(),
                detail: "Awaiting production Runtime admission".to_owned(),
                health: "unknown".to_owned(),
                availability: "unknown".to_owned(),
                activity: None,
                capabilities: vec!["conversation".to_owned()],
                location: Some("local_runtime".to_owned()),
                communication_eligible: false,
                observed_at_unix_millis: 0,
                freshness_deadline_unix_millis: 0,
                source_revision: "unobserved".to_owned(),
                provenance: "runtime_component_state".to_owned(),
            }],
            public_policy: Some(AgentRosterPolicy {
                policy_subject: "public-observatory".to_owned(),
                visible_agent_ids: BTreeSet::from(["shepherd".to_owned()]),
                reveal_capabilities: false,
                reveal_location: false,
            }),
            ..Self::empty()
        }
    }

    pub fn with_public_policy(mut self, policy: AgentRosterPolicy) -> Self {
        self.public_policy = Some(policy);
        self
    }

    pub(super) fn admit_dynamic(&mut self, agent: AgentSample) {
        if let Some(existing) = self.sample.iter_mut().find(|item| item.id == agent.id) {
            *existing = agent;
        } else {
            self.sample.push(agent);
        }
        self.sample.sort_by(|left, right| left.id.cmp(&right.id));
        self.revision = self.revision.saturating_add(1).max(1);
        self.total_count = self.sample.len() as u64;
        if let Some(policy) = self.public_policy.as_mut() {
            for item in &self.sample {
                policy.visible_agent_ids.insert(item.id.clone());
            }
        }
    }

    pub(super) fn update_resident_shepherd_health(
        &mut self,
        name: &str,
        state: &str,
        detail: &str,
    ) {
        let Some(agent) = self.sample.iter_mut().find(|agent| agent.name == name) else {
            return;
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        let readiness = InferenceReadinessState::from_projection_state(state);
        let projection = readiness.projection();
        agent.inference_readiness = readiness;
        agent.state = readiness.as_str().to_owned();
        agent.detail = detail.to_owned();
        agent.health = projection.health.to_owned();
        agent.availability = projection.availability.to_owned();
        agent.communication_eligible = projection.communication_eligible;
        agent.activity = projection.activity.map(str::to_owned);
        agent.observed_at_unix_millis = now;
        agent.freshness_deadline_unix_millis = now.saturating_add(5 * 60 * 1_000);
        agent.source_revision = self.revision.saturating_add(1).to_string();
        self.revision = self.revision.saturating_add(1).max(1);
    }

    pub(super) fn remove_dynamic(&mut self, agent_id: &str) {
        self.sample.retain(|item| item.id != agent_id);
        self.revision = self.revision.saturating_add(1).max(1);
        self.total_count = self.sample.len() as u64;
        if let Some(policy) = self.public_policy.as_mut() {
            policy.visible_agent_ids.remove(agent_id);
        }
    }

    pub(super) fn with_runtime_snapshot_query(
        &self,
        snapshot: &RuntimeSnapshot,
        now_unix_millis: u64,
        token_key: [u8; 32],
        query: AgentRosterQuery,
    ) -> Self {
        self.try_with_runtime_snapshot_query(snapshot, now_unix_millis, token_key, query, None)
            .unwrap_or_else(|_| Self::empty())
    }

    pub(super) fn try_with_runtime_snapshot_query(
        &self,
        snapshot: &RuntimeSnapshot,
        now_unix_millis: u64,
        token_key: [u8; 32],
        query: AgentRosterQuery,
        event_cursor: Option<&str>,
    ) -> Result<Self, crate::AgentRosterError> {
        if self.sample.is_empty() {
            return Ok(Self::empty());
        }
        if !self
            .sample
            .iter()
            .any(|agent| agent.provenance == "runtime_component_state")
        {
            return Ok(self.clone());
        }
        let Some(public_policy) = self.public_policy.as_ref() else {
            return Ok(Self::empty());
        };
        let evidence = self
            .sample
            .iter()
            .filter_map(|agent| project_agent_evidence(agent, snapshot));
        let page = AgentRoster::projection(
            snapshot.revision.max(self.revision).max(1),
            false,
            token_key,
        )?
        .page_evidence(
            evidence,
            public_policy,
            query,
            now_unix_millis,
            event_cursor,
        )?;
        Ok(Self {
            schema: page.schema,
            revision: page.revision,
            scope: page.scope,
            total_count: page.visible_count,
            rendered_sample_count: page.page_count,
            has_more: page.has_more,
            next_page_token: page.next_page_token,
            event_cursor: Some(page.event_cursor),
            population_complete: page.population_complete,
            sample: page.agents.into_iter().map(AgentSample::from).collect(),
            public_policy: None,
        })
    }

    pub(super) fn agent_detail(
        &self,
        snapshot: &RuntimeSnapshot,
        now_unix_millis: u64,
        token_key: [u8; 32],
        agent_id: &str,
    ) -> Result<AgentRosterEntry, crate::AgentRosterError> {
        let policy = self
            .public_policy
            .as_ref()
            .ok_or(crate::AgentRosterError::NotVisible)?;
        if !policy.visible_agent_ids.contains(agent_id) {
            return Err(crate::AgentRosterError::NotVisible);
        }
        let sample = self
            .sample
            .iter()
            .find(|agent| agent.id == agent_id)
            .ok_or(crate::AgentRosterError::NotVisible)?;
        let evidence =
            project_agent_evidence(sample, snapshot).ok_or(crate::AgentRosterError::NotVisible)?;
        AgentRoster::new(snapshot.revision.max(1), false, [evidence], token_key)?.detail(
            policy,
            agent_id,
            now_unix_millis,
        )
    }
}

fn project_agent_evidence(
    agent: &AgentSample,
    snapshot: &RuntimeSnapshot,
) -> Option<AgentRuntimeEvidence> {
    if agent.provenance != "runtime_component_state" {
        return Some(AgentRuntimeEvidence::from(agent));
    }
    let state = snapshot.components.get(&ComponentId::new(&agent.id))?;
    let admission = snapshot.agent_admissions.get(&agent.id)?;
    let runtime_projection = match state {
        RunningState::Running => (AgentPresence::Ready, "healthy", "available", true),
        RunningState::Starting | RunningState::Ready => {
            (AgentPresence::Unknown, "starting", "unavailable", false)
        }
        RunningState::Restarting => (AgentPresence::Migrating, "recovering", "unavailable", false),
        RunningState::Degraded => (AgentPresence::Degraded, "degraded", "unavailable", false),
        RunningState::Stopping | RunningState::Stopped | RunningState::Failed => (
            AgentPresence::Unreachable,
            "unhealthy",
            "unavailable",
            false,
        ),
    };
    let readiness_projection = agent.inference_readiness.projection();
    let (presence, health, availability, eligible) =
        if agent.provider.is_some() || agent.model.is_some() {
            (
                readiness_projection.presence,
                readiness_projection.health,
                readiness_projection.availability,
                readiness_projection.communication_eligible,
            )
        } else {
            runtime_projection
        };
    Some(AgentRuntimeEvidence {
        agent_id: agent.id.clone(),
        name: agent.name.clone(),
        display_name: agent.label.clone(),
        public_role: agent.role.clone(),
        provider: agent.provider.clone(),
        model: agent.model.clone(),
        inference_readiness: agent.inference_readiness,
        presence,
        health: health.to_owned(),
        availability: availability.to_owned(),
        activity: agent.activity.clone(),
        capabilities: agent.capabilities.clone(),
        location: agent.location.clone(),
        communication_eligible: eligible,
        observed_at_unix_millis: admission.observed_at_unix_millis,
        freshness_deadline_unix_millis: admission.freshness_deadline_unix_millis,
        source_revision: admission.source_revision.clone(),
        provenance: agent.provenance.clone(),
    })
}

impl From<&AgentSample> for AgentRuntimeEvidence {
    fn from(agent: &AgentSample) -> Self {
        let readiness_projection = agent.inference_readiness.projection();
        let provider_backed = agent.provider.is_some() || agent.model.is_some();
        Self {
            agent_id: agent.id.clone(),
            name: agent.name.clone(),
            display_name: agent.label.clone(),
            public_role: agent.role.clone(),
            provider: agent.provider.clone(),
            model: agent.model.clone(),
            inference_readiness: agent.inference_readiness,
            presence: if provider_backed {
                readiness_projection.presence
            } else {
                match agent.state.as_str() {
                    "ready" => AgentPresence::Ready,
                    "busy" => AgentPresence::Busy,
                    "sleeping" => AgentPresence::Sleeping,
                    "degraded" => AgentPresence::Degraded,
                    "unreachable" => AgentPresence::Unreachable,
                    "migrating" => AgentPresence::Migrating,
                    _ => AgentPresence::Unknown,
                }
            },
            health: if provider_backed {
                readiness_projection.health.to_owned()
            } else {
                agent.health.clone()
            },
            availability: if provider_backed {
                readiness_projection.availability.to_owned()
            } else {
                agent.availability.clone()
            },
            activity: if provider_backed {
                readiness_projection.activity.map(str::to_owned)
            } else {
                agent.activity.clone()
            },
            capabilities: agent.capabilities.clone(),
            location: agent.location.clone(),
            communication_eligible: if provider_backed {
                readiness_projection.communication_eligible
            } else {
                agent.communication_eligible
            },
            observed_at_unix_millis: agent.observed_at_unix_millis,
            freshness_deadline_unix_millis: agent.freshness_deadline_unix_millis,
            source_revision: agent.source_revision.clone(),
            provenance: agent.provenance.clone(),
        }
    }
}

impl From<AgentRosterEntry> for AgentSample {
    fn from(agent: AgentRosterEntry) -> Self {
        let provider_backed = agent.provider.is_some() || agent.model.is_some();
        let state = if provider_backed {
            agent.inference_readiness.as_str().to_owned()
        } else {
            serde_json::to_value(agent.presence)
                .ok()
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_else(|| "unknown".to_owned())
        };
        Self {
            id: agent.id,
            name: agent.name,
            label: agent.label,
            role: agent.role,
            provider: agent.provider,
            model: agent.model,
            inference_readiness: agent.inference_readiness,
            state,
            detail: "Runtime-authorized local roster projection".to_owned(),
            health: agent.health,
            availability: agent.availability,
            activity: agent.activity,
            capabilities: agent.capabilities,
            location: agent.location,
            communication_eligible: agent.communication_eligible,
            observed_at_unix_millis: agent.observed_at_unix_millis,
            freshness_deadline_unix_millis: agent.freshness_deadline_unix_millis,
            source_revision: agent.source_revision,
            provenance: agent.provenance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentSample {
    pub id: String,
    pub name: String,
    pub label: String,
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub inference_readiness: InferenceReadinessState,
    pub state: String,
    pub detail: String,
    pub health: String,
    pub availability: String,
    pub activity: Option<String>,
    pub capabilities: Vec<String>,
    pub location: Option<String>,
    pub communication_eligible: bool,
    pub observed_at_unix_millis: u64,
    pub freshness_deadline_unix_millis: u64,
    pub source_revision: String,
    pub provenance: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryProofFeed {
    pub default_runtime_switch_authorized: bool,
    pub runtime_v2_decommission_authorized: bool,
    pub sidecar_required: bool,
    pub vector_cloudwatch_route: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservatoryFeed {
    pub schema: String,
    pub polis_identity: PolisIdentityFeed,
    pub runtime_instance_id: String,
    pub runtime_incarnation_id: String,
    pub runtime_process_id: u32,
    pub default_runtime_changed: bool,
    pub runtime_selection: String,
    pub control: ObservatoryControlFeed,
    pub health: ObservatoryHealthFeed,
    pub weather: Option<WeatherHealthReport>,
    pub weather_freshness: Option<ObservatoryWeatherFreshness>,
    pub continuity: ObservatoryContinuityFeed,
    pub ingress: crate::IngressSnapshot,
    pub agents: AgentPopulationFeed,
    pub proof: ObservatoryProofFeed,
    pub events: Vec<BootstrapEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolisIdentityFeed {
    pub polis_id: String,
    pub display_name: String,
    pub public_domain: String,
    pub runtime_api_base: String,
    pub observatory_public_origin: String,
}

impl PolisIdentityFeed {
    pub(crate) fn unavailable(instance_id: &str) -> Self {
        Self {
            polis_id: instance_id.to_owned(),
            display_name: "Unavailable".to_owned(),
            public_domain: "localhost".to_owned(),
            runtime_api_base: "https://localhost".to_owned(),
            observatory_public_origin: "https://localhost".to_owned(),
        }
    }
}
