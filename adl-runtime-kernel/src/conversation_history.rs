use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    ConversationJournal, ConversationJournalError, ConversationJournalEvent,
    ConversationJournalRecord,
};

pub const CONVERSATION_HISTORY_MESSAGE_PREFIX: &str = "history-message-jcs-hex:";
pub const CONVERSATION_HISTORY_REDACTION_PREFIX: &str = "history-redaction-jcs-hex:";
pub const CONVERSATION_HISTORY_CURSOR_PREFIX: &str = "history-cursor-jcs-hex:";
pub const CONVERSATION_HISTORY_SCHEMA: &str = "adl.runtime.conversation_history.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryMessage {
    pub schema: String,
    pub conversation_id: String,
    pub message_id: String,
    pub speaker_id: String,
    pub body: String,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryRedaction {
    pub schema: String,
    pub conversation_id: String,
    pub message_id: String,
    pub reason: String,
    pub redacted_by_principal_id: String,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryAccessPolicy {
    pub principal_id: String,
    pub authority_audit_hash: String,
    pub allowed_conversations: BTreeSet<String>,
    #[serde(default)]
    pub revoked_conversations: BTreeSet<String>,
    pub allow_export: bool,
    pub allow_redaction: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationHistoryPageRequest {
    pub conversation_id: String,
    pub page_size: usize,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationHistorySearchRequest {
    pub conversation_id: String,
    pub query: String,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryCursor {
    pub schema: String,
    pub conversation_id: String,
    pub principal_id: String,
    pub authority_audit_hash: String,
    pub snapshot_head_sequence: u64,
    pub snapshot_head_hash: String,
    pub next_offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryRecord {
    pub conversation_id: String,
    pub message_id: String,
    pub speaker_id: String,
    pub body: String,
    pub created_at_epoch_ms: u64,
    pub journal_sequence: u64,
    pub redacted: bool,
    pub redaction_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryPage {
    pub schema: String,
    pub conversation_id: String,
    pub records: Vec<ConversationHistoryRecord>,
    pub next_cursor: Option<String>,
    pub snapshot_head_sequence: u64,
    pub snapshot_head_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationHistoryExport {
    pub schema: String,
    pub conversation_id: String,
    pub exported_by_principal_id: String,
    pub records: Vec<ConversationHistoryRecord>,
    pub snapshot_head_sequence: u64,
    pub snapshot_head_hash: String,
    pub public_safe_redacted: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversationHistoryError {
    #[error(transparent)]
    Journal(#[from] ConversationJournalError),
    #[error("conversation history request is invalid: {0}")]
    InvalidRequest(&'static str),
    #[error("conversation history access is unauthorized")]
    Unauthorized,
    #[error("conversation history cursor is stale")]
    StaleCursor,
    #[error("conversation history export is not authorized")]
    ExportForbidden,
    #[error("conversation history redaction is not authorized")]
    RedactionForbidden,
}

pub type ConversationHistoryResult<T> = Result<T, ConversationHistoryError>;

pub struct ConversationHistoryStore {
    journal: ConversationJournal,
}

impl ConversationHistoryStore {
    pub fn open(root: impl AsRef<Path>) -> ConversationHistoryResult<Self> {
        Ok(Self {
            journal: ConversationJournal::open(root)?,
        })
    }

    pub fn append_message(
        &self,
        principal_id: impl Into<String>,
        authority_audit_hash: impl Into<String>,
        message: ConversationHistoryMessage,
    ) -> ConversationHistoryResult<ConversationJournalRecord> {
        validate_message(&message)?;
        let principal_id = principal_id.into();
        let authority_audit_hash = authority_audit_hash.into();
        require_non_empty(&principal_id, "principal_id")?;
        require_digest(&authority_audit_hash, "authority_audit_hash")?;
        let payload = serde_jcs::to_vec(&message)
            .map_err(|_| ConversationHistoryError::InvalidRequest("payload"))?;
        let payload_hash = blake3::hash(&payload).to_hex().to_string();
        self.journal
            .append_event(ConversationJournalEvent {
                event_id: format!("history:{}:{}", message.conversation_id, message.message_id),
                conversation_id: message.conversation_id,
                principal_id,
                authority_audit_hash,
                payload_hash,
                receipt_ref: Some(format!(
                    "{CONVERSATION_HISTORY_MESSAGE_PREFIX}{}",
                    hex::encode(payload)
                )),
                created_at_epoch_ms: message.created_at_epoch_ms,
            })
            .map_err(Into::into)
    }

    pub fn page(
        &self,
        policy: &ConversationHistoryAccessPolicy,
        request: ConversationHistoryPageRequest,
    ) -> ConversationHistoryResult<ConversationHistoryPage> {
        authorize(policy, &request.conversation_id)?;
        if request.page_size == 0 || request.page_size > 100 {
            return Err(ConversationHistoryError::InvalidRequest("page_size"));
        }
        let snapshot = self.journal.snapshot()?;
        let cursor = match request.cursor {
            Some(value) => Some(decode_cursor(&value)?),
            None => None,
        };
        let offset = match cursor {
            Some(cursor)
                if cursor.conversation_id == request.conversation_id
                    && cursor.principal_id == policy.principal_id
                    && cursor.authority_audit_hash == policy.authority_audit_hash
                    && cursor.snapshot_head_sequence == snapshot.head_sequence
                    && cursor.snapshot_head_hash == snapshot.head_hash =>
            {
                cursor.next_offset
            }
            Some(_) => return Err(ConversationHistoryError::StaleCursor),
            None => 0,
        };
        let records = visible_records(&snapshot, &request.conversation_id, policy)?;
        let page_records = records
            .iter()
            .skip(offset)
            .take(request.page_size)
            .cloned()
            .collect::<Vec<_>>();
        let next_offset = offset + page_records.len();
        let next_cursor = (next_offset < records.len())
            .then(|| encode_cursor(policy, &request.conversation_id, &snapshot, next_offset))
            .transpose()?;
        Ok(ConversationHistoryPage {
            schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
            conversation_id: request.conversation_id,
            records: page_records,
            next_cursor,
            snapshot_head_sequence: snapshot.head_sequence,
            snapshot_head_hash: snapshot.head_hash,
        })
    }

    pub fn search(
        &self,
        policy: &ConversationHistoryAccessPolicy,
        request: ConversationHistorySearchRequest,
    ) -> ConversationHistoryResult<Vec<ConversationHistoryRecord>> {
        authorize(policy, &request.conversation_id)?;
        let query = request.query.trim().to_ascii_lowercase();
        if query.is_empty() || request.limit == 0 || request.limit > 100 {
            return Err(ConversationHistoryError::InvalidRequest("search"));
        }
        let snapshot = self.journal.snapshot()?;
        Ok(
            visible_records(&snapshot, &request.conversation_id, policy)?
                .into_iter()
                .filter(|record| {
                    record.message_id.to_ascii_lowercase().contains(&query)
                        || record.speaker_id.to_ascii_lowercase().contains(&query)
                        || record.body.to_ascii_lowercase().contains(&query)
                })
                .take(request.limit)
                .collect(),
        )
    }

    pub fn export(
        &self,
        policy: &ConversationHistoryAccessPolicy,
        conversation_id: impl Into<String>,
    ) -> ConversationHistoryResult<ConversationHistoryExport> {
        let conversation_id = conversation_id.into();
        authorize(policy, &conversation_id)?;
        if !policy.allow_export {
            return Err(ConversationHistoryError::ExportForbidden);
        }
        let snapshot = self.journal.snapshot()?;
        Ok(ConversationHistoryExport {
            schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
            conversation_id: conversation_id.clone(),
            exported_by_principal_id: policy.principal_id.clone(),
            records: visible_records(&snapshot, &conversation_id, policy)?,
            snapshot_head_sequence: snapshot.head_sequence,
            snapshot_head_hash: snapshot.head_hash,
            public_safe_redacted: true,
        })
    }

    pub fn redact(
        &self,
        policy: &ConversationHistoryAccessPolicy,
        conversation_id: impl Into<String>,
        message_id: impl Into<String>,
        reason: impl Into<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationHistoryResult<ConversationJournalRecord> {
        let conversation_id = conversation_id.into();
        authorize(policy, &conversation_id)?;
        if !policy.allow_redaction {
            return Err(ConversationHistoryError::RedactionForbidden);
        }
        let redaction = ConversationHistoryRedaction {
            schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
            conversation_id: conversation_id.clone(),
            message_id: message_id.into(),
            reason: reason.into(),
            redacted_by_principal_id: policy.principal_id.clone(),
            created_at_epoch_ms,
        };
        validate_redaction(&redaction)?;
        let payload = serde_jcs::to_vec(&redaction)
            .map_err(|_| ConversationHistoryError::InvalidRequest("payload"))?;
        let payload_hash = blake3::hash(&payload).to_hex().to_string();
        self.journal
            .append_event(ConversationJournalEvent {
                event_id: format!(
                    "history-redaction:{conversation_id}:{}",
                    redaction.message_id
                ),
                conversation_id,
                principal_id: policy.principal_id.clone(),
                authority_audit_hash: policy.authority_audit_hash.clone(),
                payload_hash,
                receipt_ref: Some(format!(
                    "{CONVERSATION_HISTORY_REDACTION_PREFIX}{}",
                    hex::encode(payload)
                )),
                created_at_epoch_ms,
            })
            .map_err(Into::into)
    }

    pub fn restore_observatory_transcript(
        &self,
        policy: &ConversationHistoryAccessPolicy,
        conversation_id: impl Into<String>,
    ) -> ConversationHistoryResult<Vec<ConversationHistoryRecord>> {
        let conversation_id = conversation_id.into();
        authorize(policy, &conversation_id)?;
        let snapshot = self.journal.snapshot()?;
        visible_records(&snapshot, &conversation_id, policy)
    }
}

fn visible_records(
    snapshot: &crate::ConversationJournalSnapshot,
    conversation_id: &str,
    policy: &ConversationHistoryAccessPolicy,
) -> ConversationHistoryResult<Vec<ConversationHistoryRecord>> {
    authorize(policy, conversation_id)?;
    if snapshot.deleted_conversations.contains(conversation_id) {
        return Ok(Vec::new());
    }
    let redactions = history_redactions(snapshot, conversation_id);
    let mut records = Vec::new();
    for record in &snapshot.records {
        let crate::ConversationJournalEntry::Event(event) = &record.entry else {
            continue;
        };
        if event.conversation_id != conversation_id || event.principal_id != policy.principal_id {
            continue;
        }
        let Some(message) = decode_history_message(event) else {
            continue;
        };
        let redaction = redactions.get(&message.message_id);
        records.push(ConversationHistoryRecord {
            conversation_id: message.conversation_id,
            message_id: message.message_id,
            speaker_id: message.speaker_id,
            body: if redaction.is_some() {
                "[redacted]".to_string()
            } else {
                message.body
            },
            created_at_epoch_ms: message.created_at_epoch_ms,
            journal_sequence: record.sequence,
            redacted: redaction.is_some(),
            redaction_reason: redaction.map(|value| value.reason.clone()),
        });
    }
    Ok(records)
}

fn history_redactions(
    snapshot: &crate::ConversationJournalSnapshot,
    conversation_id: &str,
) -> BTreeMap<String, ConversationHistoryRedaction> {
    let mut redactions = BTreeMap::new();
    for record in &snapshot.records {
        let crate::ConversationJournalEntry::Event(event) = &record.entry else {
            continue;
        };
        if event.conversation_id != conversation_id {
            continue;
        }
        if let Some(redaction) = decode_history_redaction(event) {
            redactions.insert(redaction.message_id.clone(), redaction);
        }
    }
    redactions
}

fn decode_history_message(event: &ConversationJournalEvent) -> Option<ConversationHistoryMessage> {
    let encoded = event
        .receipt_ref
        .as_ref()?
        .strip_prefix(CONVERSATION_HISTORY_MESSAGE_PREFIX)?;
    let bytes = hex::decode(encoded).ok()?;
    let message: ConversationHistoryMessage = serde_json::from_slice(&bytes).ok()?;
    let canonical = serde_jcs::to_vec(&message).ok()?;
    let digest = blake3::hash(&canonical).to_hex().to_string();
    (digest == event.payload_hash
        && message.schema == CONVERSATION_HISTORY_SCHEMA
        && message.conversation_id == event.conversation_id)
        .then_some(message)
}

fn decode_history_redaction(
    event: &ConversationJournalEvent,
) -> Option<ConversationHistoryRedaction> {
    let encoded = event
        .receipt_ref
        .as_ref()?
        .strip_prefix(CONVERSATION_HISTORY_REDACTION_PREFIX)?;
    let bytes = hex::decode(encoded).ok()?;
    let redaction: ConversationHistoryRedaction = serde_json::from_slice(&bytes).ok()?;
    let canonical = serde_jcs::to_vec(&redaction).ok()?;
    let digest = blake3::hash(&canonical).to_hex().to_string();
    (digest == event.payload_hash
        && redaction.schema == CONVERSATION_HISTORY_SCHEMA
        && redaction.conversation_id == event.conversation_id)
        .then_some(redaction)
}

fn encode_cursor(
    policy: &ConversationHistoryAccessPolicy,
    conversation_id: &str,
    snapshot: &crate::ConversationJournalSnapshot,
    next_offset: usize,
) -> ConversationHistoryResult<String> {
    let cursor = ConversationHistoryCursor {
        schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
        conversation_id: conversation_id.to_string(),
        principal_id: policy.principal_id.clone(),
        authority_audit_hash: policy.authority_audit_hash.clone(),
        snapshot_head_sequence: snapshot.head_sequence,
        snapshot_head_hash: snapshot.head_hash.clone(),
        next_offset,
    };
    Ok(format!(
        "{CONVERSATION_HISTORY_CURSOR_PREFIX}{}",
        hex::encode(
            serde_jcs::to_vec(&cursor)
                .map_err(|_| ConversationHistoryError::InvalidRequest("cursor"))?
        )
    ))
}

fn decode_cursor(value: &str) -> ConversationHistoryResult<ConversationHistoryCursor> {
    let encoded = value
        .strip_prefix(CONVERSATION_HISTORY_CURSOR_PREFIX)
        .ok_or(ConversationHistoryError::StaleCursor)?;
    let bytes = hex::decode(encoded).map_err(|_| ConversationHistoryError::StaleCursor)?;
    let cursor: ConversationHistoryCursor =
        serde_json::from_slice(&bytes).map_err(|_| ConversationHistoryError::StaleCursor)?;
    if cursor.schema != CONVERSATION_HISTORY_SCHEMA {
        return Err(ConversationHistoryError::StaleCursor);
    }
    Ok(cursor)
}

fn authorize(
    policy: &ConversationHistoryAccessPolicy,
    conversation_id: &str,
) -> ConversationHistoryResult<()> {
    require_non_empty(&policy.principal_id, "principal_id")?;
    require_digest(&policy.authority_audit_hash, "authority_audit_hash")?;
    require_non_empty(conversation_id, "conversation_id")?;
    if policy.revoked_conversations.contains(conversation_id)
        || !policy.allowed_conversations.contains(conversation_id)
    {
        return Err(ConversationHistoryError::Unauthorized);
    }
    Ok(())
}

fn validate_message(message: &ConversationHistoryMessage) -> ConversationHistoryResult<()> {
    if message.schema != CONVERSATION_HISTORY_SCHEMA {
        return Err(ConversationHistoryError::InvalidRequest("schema"));
    }
    require_non_empty(&message.conversation_id, "conversation_id")?;
    require_non_empty(&message.message_id, "message_id")?;
    require_non_empty(&message.speaker_id, "speaker_id")?;
    require_non_empty(&message.body, "body")?;
    Ok(())
}

fn validate_redaction(redaction: &ConversationHistoryRedaction) -> ConversationHistoryResult<()> {
    if redaction.schema != CONVERSATION_HISTORY_SCHEMA {
        return Err(ConversationHistoryError::InvalidRequest("schema"));
    }
    require_non_empty(&redaction.conversation_id, "conversation_id")?;
    require_non_empty(&redaction.message_id, "message_id")?;
    require_non_empty(&redaction.reason, "reason")?;
    require_non_empty(
        &redaction.redacted_by_principal_id,
        "redacted_by_principal_id",
    )?;
    Ok(())
}

fn require_non_empty(value: &str, field: &'static str) -> ConversationHistoryResult<()> {
    if value.trim().is_empty() {
        return Err(ConversationHistoryError::InvalidRequest(field));
    }
    Ok(())
}

fn require_digest(value: &str, field: &'static str) -> ConversationHistoryResult<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ConversationHistoryError::InvalidRequest(field));
    }
    Ok(())
}
