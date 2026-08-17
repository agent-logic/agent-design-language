use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const OPERATOR_ATTENTION_REQUEST_SCHEMA: &str = "adl.runtime_v3.operator_attention.request.v1";
pub const OPERATOR_ATTENTION_EVENT_SCHEMA: &str = "adl.runtime_v3.operator_attention.event.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorAttentionIdentity {
    pub agent_id: String,
    pub principal_id: String,
    pub display_name: Option<String>,
    pub can_request_attention: bool,
    pub can_mark_urgent: bool,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorAttentionPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorAttentionReason {
    Clarification,
    Approval,
    Help,
    Acknowledgement,
    PolicyIntervention,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorAttentionStatus {
    Open,
    Acknowledged,
    Replied,
    Deferred,
    Resolved,
    Refused,
    Expired,
}

impl OperatorAttentionStatus {
    pub fn is_active(self) -> bool {
        matches!(
            self,
            Self::Open | Self::Acknowledged | Self::Replied | Self::Deferred
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorAttentionRequestInput {
    pub schema: String,
    pub source_agent_id: String,
    pub source_identity: OperatorAttentionIdentity,
    pub reason: OperatorAttentionReason,
    pub priority: OperatorAttentionPriority,
    pub message: String,
    pub correlation_id: String,
    pub related_conversation_id: Option<String>,
    pub related_work_id: Option<String>,
    pub group_key: Option<String>,
    pub created_at_millis: u64,
    pub expires_at_millis: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorAttentionRequest {
    pub schema: String,
    pub request_id: String,
    pub source_agent_id: String,
    pub source_principal_id: String,
    pub display_name: Option<String>,
    pub reason: OperatorAttentionReason,
    pub priority: OperatorAttentionPriority,
    pub message: String,
    pub correlation_id: String,
    pub related_conversation_id: Option<String>,
    pub related_work_id: Option<String>,
    pub group_key: Option<String>,
    pub created_at_millis: u64,
    pub updated_at_millis: u64,
    pub expires_at_millis: Option<u64>,
    pub status: OperatorAttentionStatus,
    pub duplicate_count: u64,
    pub grouped_count: u64,
    pub operator_response: Option<String>,
    pub deferred_until_millis: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorAttentionOutcome {
    Acknowledge,
    Reply { message: String },
    Defer { until_millis: u64 },
    Resolve,
    Refuse { reason: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorAttentionEvent {
    pub schema: String,
    pub request_id: String,
    pub status: OperatorAttentionStatus,
    pub actor_id: String,
    pub at_millis: u64,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorAttentionSnapshot {
    pub schema: String,
    pub generated_at_millis: u64,
    pub open_count: usize,
    pub requests: Vec<OperatorAttentionRequest>,
    pub events: Vec<OperatorAttentionEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorAttentionSettings {
    pub capacity: usize,
    pub max_active_per_source: usize,
    pub max_message_chars: usize,
    pub quiet_mode: bool,
    pub grouping_window_millis: u64,
}

impl Default for OperatorAttentionSettings {
    fn default() -> Self {
        Self {
            capacity: 128,
            max_active_per_source: 16,
            max_message_chars: 4096,
            quiet_mode: false,
            grouping_window_millis: 60_000,
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum OperatorAttentionError {
    #[error("operator attention schema mismatch")]
    SchemaMismatch,
    #[error("operator attention source is not authorized")]
    UnauthorizedSource,
    #[error("operator attention urgent priority is not authorized")]
    UnauthorizedUrgency,
    #[error("operator attention quiet mode suppresses this request")]
    QuietModeSuppressed,
    #[error("operator attention request is invalid: {0}")]
    InvalidRequest(&'static str),
    #[error("operator attention inbox is full")]
    CapacityExceeded,
    #[error("operator attention source rate limit exceeded")]
    RateLimited,
    #[error("operator attention request not found")]
    NotFound,
    #[error("operator attention request is terminal")]
    TerminalRequest,
}

#[derive(Clone, Debug)]
pub struct OperatorAttentionInbox {
    settings: OperatorAttentionSettings,
    trusted_sources: BTreeMap<String, OperatorAttentionIdentity>,
    requests: BTreeMap<String, OperatorAttentionRequest>,
    by_correlation: BTreeMap<(String, String), String>,
    events: Vec<OperatorAttentionEvent>,
}

impl OperatorAttentionInbox {
    pub fn new(settings: OperatorAttentionSettings) -> Result<Self, OperatorAttentionError> {
        if settings.capacity == 0 {
            return Err(OperatorAttentionError::InvalidRequest("capacity_required"));
        }
        if settings.max_active_per_source == 0 {
            return Err(OperatorAttentionError::InvalidRequest(
                "max_active_per_source_required",
            ));
        }
        if settings.max_message_chars == 0 {
            return Err(OperatorAttentionError::InvalidRequest(
                "max_message_chars_required",
            ));
        }
        Ok(Self {
            settings,
            trusted_sources: BTreeMap::new(),
            requests: BTreeMap::new(),
            by_correlation: BTreeMap::new(),
            events: Vec::new(),
        })
    }

    pub fn trust_source(
        &mut self,
        identity: OperatorAttentionIdentity,
    ) -> Result<(), OperatorAttentionError> {
        if !is_safe_token(&identity.agent_id) {
            return Err(OperatorAttentionError::InvalidRequest(
                "trusted_agent_invalid",
            ));
        }
        if identity.principal_id.trim().is_empty() {
            return Err(OperatorAttentionError::InvalidRequest(
                "trusted_principal_required",
            ));
        }
        self.trusted_sources
            .insert(identity.agent_id.clone(), identity);
        Ok(())
    }

    pub fn restore(
        settings: OperatorAttentionSettings,
        snapshot: OperatorAttentionSnapshot,
    ) -> Result<Self, OperatorAttentionError> {
        let mut inbox = Self::new(settings)?;
        if snapshot.schema != "adl.runtime_v3.operator_attention.snapshot.v1"
            || snapshot.requests.len() > inbox.settings.capacity
        {
            return Err(OperatorAttentionError::InvalidRequest("snapshot_invalid"));
        }
        for request in snapshot.requests {
            validate_restored_request(&request, inbox.settings.max_message_chars)?;
            let key = (
                request.source_agent_id.clone(),
                request.correlation_id.clone(),
            );
            if inbox.by_correlation.contains_key(&key)
                || inbox.requests.contains_key(&request.request_id)
            {
                return Err(OperatorAttentionError::InvalidRequest("snapshot_duplicate"));
            }
            let active_for_source = inbox
                .requests
                .values()
                .filter(|candidate| {
                    candidate.source_agent_id == request.source_agent_id
                        && candidate.status.is_active()
                })
                .count();
            if request.status.is_active()
                && active_for_source >= inbox.settings.max_active_per_source
            {
                return Err(OperatorAttentionError::RateLimited);
            }
            inbox
                .trusted_sources
                .entry(request.source_agent_id.clone())
                .or_insert_with(|| OperatorAttentionIdentity {
                    agent_id: request.source_agent_id.clone(),
                    principal_id: request.source_principal_id.clone(),
                    display_name: request.display_name.clone(),
                    can_request_attention: true,
                    can_mark_urgent: request.priority == OperatorAttentionPriority::Urgent,
                });
            inbox.by_correlation.insert(key, request.request_id.clone());
            inbox.requests.insert(request.request_id.clone(), request);
        }
        for event in snapshot.events {
            validate_restored_event(&event, &inbox.requests)?;
            inbox.events.push(event);
        }
        Ok(inbox)
    }

    pub fn submit(
        &mut self,
        input: OperatorAttentionRequestInput,
    ) -> Result<&OperatorAttentionRequest, OperatorAttentionError> {
        self.validate_input(&input)?;
        let trusted = self
            .trusted_sources
            .get(&input.source_agent_id)
            .cloned()
            .ok_or(OperatorAttentionError::UnauthorizedSource)?;
        let key = (input.source_agent_id.clone(), input.correlation_id.clone());
        if let Some(existing_id) = self.by_correlation.get(&key).cloned() {
            let existing = self
                .requests
                .get_mut(&existing_id)
                .ok_or(OperatorAttentionError::NotFound)?;
            existing.duplicate_count = existing.duplicate_count.saturating_add(1);
            existing.updated_at_millis = input.created_at_millis;
            return self
                .requests
                .get(&existing_id)
                .ok_or(OperatorAttentionError::NotFound);
        }
        if self.settings.quiet_mode && input.priority < OperatorAttentionPriority::Urgent {
            return Err(OperatorAttentionError::QuietModeSuppressed);
        }
        if let Some(existing_id) = self.group_candidate(&input) {
            let existing = self
                .requests
                .get_mut(&existing_id)
                .ok_or(OperatorAttentionError::NotFound)?;
            existing.grouped_count = existing.grouped_count.saturating_add(1);
            existing.updated_at_millis = input.created_at_millis;
            if input.priority > existing.priority {
                existing.priority = input.priority;
            }
            return self
                .requests
                .get(&existing_id)
                .ok_or(OperatorAttentionError::NotFound);
        }
        if self.requests.len() >= self.settings.capacity {
            return Err(OperatorAttentionError::CapacityExceeded);
        }
        let active_for_source = self
            .requests
            .values()
            .filter(|request| {
                request.source_agent_id == input.source_agent_id && request.status.is_active()
            })
            .count();
        if active_for_source >= self.settings.max_active_per_source {
            return Err(OperatorAttentionError::RateLimited);
        }
        let request_id = stable_attention_request_id(&input);
        let request = OperatorAttentionRequest {
            schema: OPERATOR_ATTENTION_REQUEST_SCHEMA.to_owned(),
            request_id: request_id.clone(),
            source_agent_id: input.source_agent_id.clone(),
            source_principal_id: trusted.principal_id,
            display_name: trusted.display_name,
            reason: input.reason,
            priority: input.priority,
            message: input.message.clone(),
            correlation_id: input.correlation_id.clone(),
            related_conversation_id: input.related_conversation_id.clone(),
            related_work_id: input.related_work_id.clone(),
            group_key: input.group_key.clone(),
            created_at_millis: input.created_at_millis,
            updated_at_millis: input.created_at_millis,
            expires_at_millis: input.expires_at_millis,
            status: OperatorAttentionStatus::Open,
            duplicate_count: 0,
            grouped_count: 0,
            operator_response: None,
            deferred_until_millis: None,
        };
        self.by_correlation.insert(key, request_id.clone());
        self.push_event(
            &request_id,
            OperatorAttentionStatus::Open,
            &request.source_agent_id,
            input.created_at_millis,
            Some("attention_requested".to_owned()),
        );
        self.requests.insert(request_id.clone(), request);
        self.requests
            .get(&request_id)
            .ok_or(OperatorAttentionError::NotFound)
    }

    pub fn apply_outcome(
        &mut self,
        request_id: &str,
        actor_id: &str,
        outcome: OperatorAttentionOutcome,
        at_millis: u64,
    ) -> Result<&OperatorAttentionRequest, OperatorAttentionError> {
        if actor_id.trim().is_empty() {
            return Err(OperatorAttentionError::InvalidRequest("actor_id_required"));
        }
        let (status, detail) = {
            let request = self
                .requests
                .get_mut(request_id)
                .ok_or(OperatorAttentionError::NotFound)?;
            if !request.status.is_active() {
                return Err(OperatorAttentionError::TerminalRequest);
            }
            request.updated_at_millis = at_millis;
            let detail;
            request.status = match outcome {
                OperatorAttentionOutcome::Acknowledge => {
                    detail = None;
                    OperatorAttentionStatus::Acknowledged
                }
                OperatorAttentionOutcome::Reply { message } => {
                    let response = bounded_text(&message, self.settings.max_message_chars)
                        .ok_or(OperatorAttentionError::InvalidRequest("reply_required"))?;
                    request.operator_response = Some(response.clone());
                    detail = Some(response);
                    OperatorAttentionStatus::Replied
                }
                OperatorAttentionOutcome::Defer { until_millis } => {
                    if until_millis <= at_millis {
                        return Err(OperatorAttentionError::InvalidRequest("defer_until_future"));
                    }
                    request.deferred_until_millis = Some(until_millis);
                    detail = Some(until_millis.to_string());
                    OperatorAttentionStatus::Deferred
                }
                OperatorAttentionOutcome::Resolve => {
                    detail = None;
                    OperatorAttentionStatus::Resolved
                }
                OperatorAttentionOutcome::Refuse { reason } => {
                    let reason = bounded_text(&reason, self.settings.max_message_chars).ok_or(
                        OperatorAttentionError::InvalidRequest("refusal_reason_required"),
                    )?;
                    request.operator_response = Some(reason.clone());
                    detail = Some(reason);
                    OperatorAttentionStatus::Refused
                }
            };
            (request.status, detail)
        };
        self.push_event(request_id, status, actor_id, at_millis, detail);
        self.requests
            .get(request_id)
            .ok_or(OperatorAttentionError::NotFound)
    }

    pub fn expire(&mut self, now_millis: u64) -> usize {
        let expired: Vec<String> = self
            .requests
            .iter()
            .filter(|(_, request)| {
                request.status.is_active()
                    && request
                        .expires_at_millis
                        .is_some_and(|expires| expires <= now_millis)
            })
            .map(|(request_id, _)| request_id.clone())
            .collect();
        for request_id in &expired {
            if let Some(request) = self.requests.get_mut(request_id) {
                request.status = OperatorAttentionStatus::Expired;
                request.updated_at_millis = now_millis;
            }
            self.push_event(
                request_id,
                OperatorAttentionStatus::Expired,
                "runtime",
                now_millis,
                Some("attention_request_expired".to_owned()),
            );
        }
        expired.len()
    }

    pub fn snapshot(&self, generated_at_millis: u64) -> OperatorAttentionSnapshot {
        let mut requests: Vec<_> = self.requests.values().cloned().collect();
        requests.sort_by(|left, right| {
            right
                .priority
                .cmp(&left.priority)
                .then_with(|| left.created_at_millis.cmp(&right.created_at_millis))
                .then_with(|| left.request_id.cmp(&right.request_id))
        });
        let open_count = requests
            .iter()
            .filter(|request| request.status.is_active())
            .count();
        OperatorAttentionSnapshot {
            schema: "adl.runtime_v3.operator_attention.snapshot.v1".to_owned(),
            generated_at_millis,
            open_count,
            requests,
            events: self.events.clone(),
        }
    }

    fn validate_input(
        &self,
        input: &OperatorAttentionRequestInput,
    ) -> Result<(), OperatorAttentionError> {
        if input.schema != OPERATOR_ATTENTION_REQUEST_SCHEMA {
            return Err(OperatorAttentionError::SchemaMismatch);
        }
        if input.source_agent_id.trim().is_empty() {
            return Err(OperatorAttentionError::InvalidRequest(
                "source_agent_id_required",
            ));
        }
        if input.source_identity.agent_id != input.source_agent_id {
            return Err(OperatorAttentionError::UnauthorizedSource);
        }
        let trusted = self
            .trusted_sources
            .get(&input.source_agent_id)
            .ok_or(OperatorAttentionError::UnauthorizedSource)?;
        if input.source_identity.principal_id != trusted.principal_id {
            return Err(OperatorAttentionError::UnauthorizedSource);
        }
        if !trusted.can_request_attention {
            return Err(OperatorAttentionError::UnauthorizedSource);
        }
        if input.priority == OperatorAttentionPriority::Urgent && !trusted.can_mark_urgent {
            return Err(OperatorAttentionError::UnauthorizedUrgency);
        }
        if trusted.principal_id.trim().is_empty() {
            return Err(OperatorAttentionError::InvalidRequest(
                "principal_id_required",
            ));
        }
        if !is_safe_token(&input.correlation_id) {
            return Err(OperatorAttentionError::InvalidRequest(
                "correlation_id_invalid",
            ));
        }
        if bounded_text(&input.message, self.settings.max_message_chars).is_none() {
            return Err(OperatorAttentionError::InvalidRequest("message_required"));
        }
        if input
            .expires_at_millis
            .is_some_and(|expires| expires <= input.created_at_millis)
        {
            return Err(OperatorAttentionError::InvalidRequest("expires_at_future"));
        }
        Ok(())
    }

    fn group_candidate(&self, input: &OperatorAttentionRequestInput) -> Option<String> {
        if self.settings.grouping_window_millis == 0 {
            return None;
        }
        let group_key = attention_group_key(input)?;
        self.requests
            .values()
            .filter(|request| {
                request.status.is_active()
                    && request.source_agent_id == input.source_agent_id
                    && request.reason == input.reason
                    && request.group_key.as_deref() == Some(group_key.as_str())
                    && input
                        .created_at_millis
                        .saturating_sub(request.created_at_millis)
                        <= self.settings.grouping_window_millis
            })
            .min_by_key(|request| request.created_at_millis)
            .map(|request| request.request_id.clone())
    }

    fn push_event(
        &mut self,
        request_id: &str,
        status: OperatorAttentionStatus,
        actor_id: &str,
        at_millis: u64,
        detail: Option<String>,
    ) {
        self.events.push(OperatorAttentionEvent {
            schema: OPERATOR_ATTENTION_EVENT_SCHEMA.to_owned(),
            request_id: request_id.to_owned(),
            status,
            actor_id: actor_id.to_owned(),
            at_millis,
            detail,
        });
        if self.events.len() > self.settings.capacity.saturating_mul(4) {
            let retain_from = self.events.len() - self.settings.capacity.saturating_mul(4);
            self.events.drain(0..retain_from);
        }
    }
}

fn stable_attention_request_id(input: &OperatorAttentionRequestInput) -> String {
    let payload = serde_jcs::to_vec(input).unwrap_or_else(|_| {
        format!(
            "{}:{}:{}",
            input.source_agent_id, input.correlation_id, input.created_at_millis
        )
        .into_bytes()
    });
    format!("op-attn-{}", blake3::hash(&payload).to_hex())
}

fn validate_restored_request(
    request: &OperatorAttentionRequest,
    max_message_chars: usize,
) -> Result<(), OperatorAttentionError> {
    if request.schema != OPERATOR_ATTENTION_REQUEST_SCHEMA {
        return Err(OperatorAttentionError::SchemaMismatch);
    }
    if !is_safe_token(&request.request_id)
        || !is_safe_token(&request.source_agent_id)
        || !is_safe_token(&request.correlation_id)
        || request.source_principal_id.trim().is_empty()
        || bounded_text(&request.message, max_message_chars).is_none()
        || request.updated_at_millis < request.created_at_millis
        || request
            .expires_at_millis
            .is_some_and(|expires| expires <= request.created_at_millis)
    {
        return Err(OperatorAttentionError::InvalidRequest(
            "snapshot_request_invalid",
        ));
    }
    Ok(())
}

fn validate_restored_event(
    event: &OperatorAttentionEvent,
    requests: &BTreeMap<String, OperatorAttentionRequest>,
) -> Result<(), OperatorAttentionError> {
    if event.schema != OPERATOR_ATTENTION_EVENT_SCHEMA
        || !requests.contains_key(&event.request_id)
        || event.actor_id.trim().is_empty()
    {
        return Err(OperatorAttentionError::InvalidRequest(
            "snapshot_event_invalid",
        ));
    }
    Ok(())
}

fn bounded_text(value: &str, max_chars: usize) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > max_chars {
        return None;
    }
    Some(trimmed.to_owned())
}

fn is_safe_token(value: &str) -> bool {
    let text = value.trim();
    !text.is_empty()
        && text.len() <= 160
        && text
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | ':' | '-'))
}

fn attention_group_key(input: &OperatorAttentionRequestInput) -> Option<String> {
    input
        .group_key
        .clone()
        .or_else(|| input.related_work_id.clone())
        .or_else(|| input.related_conversation_id.clone())
        .filter(|value| is_safe_token(value))
}

pub fn active_attention_sources(snapshot: &OperatorAttentionSnapshot) -> BTreeSet<String> {
    snapshot
        .requests
        .iter()
        .filter(|request| request.status.is_active())
        .map(|request| request.source_agent_id.clone())
        .collect()
}
