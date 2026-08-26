use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const AGENT_ROSTER_PAGE_SCHEMA: &str = "adl.runtime_v3.agent_roster_page.v1";
pub const AGENT_ROSTER_ENTRY_SCHEMA: &str = "adl.runtime_v3.agent_roster_entry.v1";
const MAX_PAGE_SIZE: usize = 100;
const MAX_FILTER_BYTES: usize = 64;
const MAX_ROSTER_ENTRIES: usize = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentPresence {
    Ready,
    Busy,
    Sleeping,
    Degraded,
    Unreachable,
    Migrating,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRuntimeEvidence {
    pub agent_id: String,
    pub display_name: String,
    pub public_role: String,
    pub presence: AgentPresence,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRosterPolicy {
    pub policy_subject: String,
    pub visible_agent_ids: BTreeSet<String>,
    pub reveal_capabilities: bool,
    pub reveal_location: bool,
}

impl AgentRosterPolicy {
    fn digest(&self) -> String {
        let projection = serde_json::json!({
            "policy_subject": self.policy_subject,
            "visible_agent_ids": self.visible_agent_ids,
            "reveal_capabilities": self.reveal_capabilities,
            "reveal_location": self.reveal_location,
        });
        blake3::hash(projection.to_string().as_bytes())
            .to_hex()
            .to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentRosterEntry {
    pub schema: String,
    pub id: String,
    pub label: String,
    pub role: String,
    pub presence: AgentPresence,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentRosterQuery {
    pub page_size: usize,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentRosterPage {
    pub schema: String,
    pub revision: u64,
    pub scope: String,
    pub visible_count: u64,
    pub page_count: u64,
    pub has_more: bool,
    pub next_page_token: Option<String>,
    pub event_cursor: String,
    pub population_complete: bool,
    pub agents: Vec<AgentRosterEntry>,
}

#[derive(Clone, Debug)]
pub struct AgentRoster {
    revision: u64,
    population_complete: bool,
    evidence: BTreeMap<String, AgentRuntimeEvidence>,
    token_key: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct PageToken {
    revision: u64,
    policy_digest: String,
    filter: Option<String>,
    page_size: usize,
    offset: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct EventCursor {
    revision: u64,
    policy_digest: String,
    filter: Option<String>,
    page_size: usize,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AgentRosterError {
    #[error("roster query bounds are invalid")]
    InvalidBounds,
    #[error("roster page token is malformed")]
    InvalidToken,
    #[error("roster page token does not match the current query or policy")]
    TokenContextMismatch,
    #[error("agent is not visible under the current policy")]
    NotVisible,
}

impl AgentRoster {
    pub fn new(
        revision: u64,
        population_complete: bool,
        evidence: impl IntoIterator<Item = AgentRuntimeEvidence>,
        token_key: [u8; 32],
    ) -> Result<Self, AgentRosterError> {
        if revision == 0 {
            return Err(AgentRosterError::InvalidBounds);
        }
        let mut by_id = BTreeMap::new();
        for item in evidence {
            if by_id.len() == MAX_ROSTER_ENTRIES {
                return Err(AgentRosterError::InvalidBounds);
            }
            validate_evidence(&item)?;
            if by_id.insert(item.agent_id.clone(), item).is_some() {
                return Err(AgentRosterError::InvalidBounds);
            }
        }
        Ok(Self {
            revision,
            population_complete,
            evidence: by_id,
            token_key,
        })
    }

    pub fn page(
        &self,
        policy: &AgentRosterPolicy,
        query: AgentRosterQuery,
        now_unix_millis: u64,
    ) -> Result<AgentRosterPage, AgentRosterError> {
        self.page_after(policy, query, now_unix_millis, None)
    }

    pub fn page_after(
        &self,
        policy: &AgentRosterPolicy,
        query: AgentRosterQuery,
        now_unix_millis: u64,
        event_cursor: Option<&str>,
    ) -> Result<AgentRosterPage, AgentRosterError> {
        self.page_iter(
            policy,
            query,
            now_unix_millis,
            event_cursor,
            self.evidence.values(),
        )
    }

    pub fn projection(
        revision: u64,
        population_complete: bool,
        token_key: [u8; 32],
    ) -> Result<Self, AgentRosterError> {
        if revision == 0 {
            return Err(AgentRosterError::InvalidBounds);
        }
        Ok(Self {
            revision,
            population_complete,
            evidence: BTreeMap::new(),
            token_key,
        })
    }

    pub fn page_evidence<T>(
        &self,
        evidence: impl IntoIterator<Item = T>,
        policy: &AgentRosterPolicy,
        query: AgentRosterQuery,
        now_unix_millis: u64,
        event_cursor: Option<&str>,
    ) -> Result<AgentRosterPage, AgentRosterError>
    where
        T: Borrow<AgentRuntimeEvidence>,
    {
        self.page_iter(policy, query, now_unix_millis, event_cursor, evidence)
    }

    fn page_iter<T>(
        &self,
        policy: &AgentRosterPolicy,
        query: AgentRosterQuery,
        now_unix_millis: u64,
        event_cursor: Option<&str>,
        evidence: impl IntoIterator<Item = T>,
    ) -> Result<AgentRosterPage, AgentRosterError>
    where
        T: Borrow<AgentRuntimeEvidence>,
    {
        validate_query(&query)?;
        let filter = query
            .filter
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase);
        let policy_digest = policy.digest();
        if let Some(cursor) = event_cursor {
            let decoded: EventCursor = self.decode_signed(cursor)?;
            if decoded.policy_digest != policy_digest
                || decoded.filter != filter
                || decoded.page_size != query.page_size
                || decoded.revision.checked_add(1) != Some(self.revision)
            {
                return Err(AgentRosterError::TokenContextMismatch);
            }
        }
        let offset = match query.page_token.as_deref() {
            Some(token) => {
                let decoded = self.decode_token(token)?;
                if decoded.revision != self.revision
                    || decoded.policy_digest != policy_digest
                    || decoded.filter != filter
                    || decoded.page_size != query.page_size
                {
                    return Err(AgentRosterError::TokenContextMismatch);
                }
                decoded.offset
            }
            None => 0,
        };

        let matches = |item: &AgentRuntimeEvidence| {
            policy.visible_agent_ids.contains(&item.agent_id)
                && filter.as_ref().is_none_or(|needle| {
                    item.agent_id.to_ascii_lowercase().contains(needle)
                        || item.display_name.to_ascii_lowercase().contains(needle)
                        || item.public_role.to_ascii_lowercase().contains(needle)
                })
        };
        let mut population_count = 0usize;
        let mut visible_count = 0usize;
        let mut agents = Vec::with_capacity(query.page_size);
        for item in evidence {
            population_count = population_count.saturating_add(1);
            if population_count > MAX_ROSTER_ENTRIES {
                return Err(AgentRosterError::InvalidBounds);
            }
            let item = item.borrow();
            validate_evidence(item)?;
            if matches(item) {
                if visible_count >= offset && agents.len() < query.page_size {
                    agents.push(project_entry(item, policy, now_unix_millis));
                }
                visible_count = visible_count.saturating_add(1);
            }
        }
        if offset > visible_count {
            return Err(AgentRosterError::TokenContextMismatch);
        }
        let end = offset.saturating_add(agents.len());
        let has_more = end < visible_count;
        let next_page_token = has_more
            .then(|| {
                self.encode_token(&PageToken {
                    revision: self.revision,
                    policy_digest,
                    filter: filter.clone(),
                    page_size: query.page_size,
                    offset: end,
                })
            })
            .transpose()?;
        let event_cursor = self.encode_signed(&EventCursor {
            revision: self.revision,
            policy_digest: policy.digest(),
            filter: filter.clone(),
            page_size: query.page_size,
        })?;
        Ok(AgentRosterPage {
            schema: AGENT_ROSTER_PAGE_SCHEMA.to_owned(),
            revision: self.revision,
            scope: "local_runtime".to_owned(),
            visible_count: visible_count as u64,
            page_count: agents.len() as u64,
            has_more,
            next_page_token,
            event_cursor,
            population_complete: self.population_complete,
            agents,
        })
    }

    pub fn detail(
        &self,
        policy: &AgentRosterPolicy,
        agent_id: &str,
        now_unix_millis: u64,
    ) -> Result<AgentRosterEntry, AgentRosterError> {
        if !policy.visible_agent_ids.contains(agent_id) {
            return Err(AgentRosterError::NotVisible);
        }
        self.evidence
            .get(agent_id)
            .map(|item| project_entry(item, policy, now_unix_millis))
            .ok_or(AgentRosterError::NotVisible)
    }

    fn encode_token(&self, token: &PageToken) -> Result<String, AgentRosterError> {
        self.encode_signed(token)
    }

    fn encode_signed<T: Serialize>(&self, token: &T) -> Result<String, AgentRosterError> {
        let payload = serde_json::to_vec(token).map_err(|_| AgentRosterError::InvalidToken)?;
        let signature = blake3::keyed_hash(&self.token_key, &payload);
        Ok(format!("{}.{}", hex::encode(payload), signature.to_hex()))
    }

    fn decode_token(&self, token: &str) -> Result<PageToken, AgentRosterError> {
        self.decode_signed(token)
    }

    fn decode_signed<T: for<'de> Deserialize<'de>>(
        &self,
        token: &str,
    ) -> Result<T, AgentRosterError> {
        let (payload, signature) = token
            .split_once('.')
            .ok_or(AgentRosterError::InvalidToken)?;
        let payload = hex::decode(payload).map_err(|_| AgentRosterError::InvalidToken)?;
        let signature =
            blake3::Hash::from_hex(signature).map_err(|_| AgentRosterError::InvalidToken)?;
        if blake3::keyed_hash(&self.token_key, &payload) != signature {
            return Err(AgentRosterError::InvalidToken);
        }
        serde_json::from_slice(&payload).map_err(|_| AgentRosterError::InvalidToken)
    }
}

fn validate_query(query: &AgentRosterQuery) -> Result<(), AgentRosterError> {
    if query.page_size == 0
        || query.page_size > MAX_PAGE_SIZE
        || query
            .filter
            .as_ref()
            .is_some_and(|value| value.len() > MAX_FILTER_BYTES)
    {
        return Err(AgentRosterError::InvalidBounds);
    }
    Ok(())
}

fn validate_evidence(item: &AgentRuntimeEvidence) -> Result<(), AgentRosterError> {
    if item.agent_id.is_empty()
        || item.agent_id.len() > 128
        || item.display_name.is_empty()
        || item.display_name.len() > 128
        || item.public_role.is_empty()
        || item.public_role.len() > 128
        || item.source_revision.is_empty()
        || item.provenance.is_empty()
        || item.observed_at_unix_millis == 0
        || item.freshness_deadline_unix_millis <= item.observed_at_unix_millis
        || item.capabilities.len() > 32
        || item.capabilities.iter().any(|value| value.len() > 128)
    {
        return Err(AgentRosterError::InvalidBounds);
    }
    Ok(())
}

fn project_entry(
    item: &AgentRuntimeEvidence,
    policy: &AgentRosterPolicy,
    now_unix_millis: u64,
) -> AgentRosterEntry {
    let stale = now_unix_millis > item.freshness_deadline_unix_millis;
    AgentRosterEntry {
        schema: AGENT_ROSTER_ENTRY_SCHEMA.to_owned(),
        id: item.agent_id.clone(),
        label: item.display_name.clone(),
        role: item.public_role.clone(),
        presence: if stale {
            AgentPresence::Unknown
        } else {
            item.presence
        },
        health: if stale {
            "stale".to_owned()
        } else {
            item.health.clone()
        },
        availability: if stale {
            "unknown".to_owned()
        } else {
            item.availability.clone()
        },
        activity: (!stale).then(|| item.activity.clone()).flatten(),
        capabilities: if policy.reveal_capabilities {
            item.capabilities.clone()
        } else {
            Vec::new()
        },
        location: policy
            .reveal_location
            .then(|| item.location.clone())
            .flatten(),
        communication_eligible: !stale && item.communication_eligible,
        observed_at_unix_millis: item.observed_at_unix_millis,
        freshness_deadline_unix_millis: item.freshness_deadline_unix_millis,
        source_revision: item.source_revision.clone(),
        provenance: item.provenance.clone(),
    }
}
