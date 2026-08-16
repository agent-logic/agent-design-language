use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    ConversationJournal, ConversationJournalError, ConversationJournalEvent,
    ConversationJournalRecord,
};

pub const CONVERSATION_CONTINUITY_SCHEMA: &str = "adl.runtime.conversation_continuity.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationContinuityEvent {
    pub schema: String,
    pub conversation_id: String,
    pub event: ConversationContinuityEventKind,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConversationContinuityEventKind {
    SenderWatermark {
        watermark: u64,
        receipt_id: String,
    },
    AcknowledgementWatermark {
        watermark: u64,
        acknowledgement_message_id: String,
        receipt_id: String,
    },
    Attempt {
        attempt_id: String,
        idempotency_key: String,
        outcome: AttemptOutcome,
        receipt_id: String,
    },
    ReplayDecision {
        replay_id: String,
        owner_id: String,
        high_watermark: u64,
        receipt_id: String,
    },
    DeliveryReceipt {
        attempt_id: String,
        receipt_id: String,
        response_receipt_id: Option<String>,
        acknowledgement_receipt_id: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcome {
    PreDispatchRetryable,
    DispatchedAmbiguous,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttemptAdmission {
    NewOrRetryable,
    DuplicateAmbiguous {
        attempt_id: String,
        receipt_id: String,
    },
    DuplicateCompleted {
        attempt_id: String,
        receipt_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttemptState {
    pub conversation_id: String,
    pub attempt_id: String,
    pub idempotency_key: String,
    pub outcome: AttemptOutcome,
    pub receipt_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeliveryReceiptState {
    pub conversation_id: String,
    pub attempt_id: String,
    pub receipt_id: String,
    pub response_receipt_id: Option<String>,
    pub acknowledgement_receipt_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayDecisionState {
    pub conversation_id: String,
    pub replay_id: String,
    pub owner_id: String,
    pub high_watermark: u64,
    pub receipt_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConversationContinuitySnapshot {
    pub sender_watermarks: BTreeMap<String, u64>,
    pub acknowledgement_watermarks: BTreeMap<String, u64>,
    pub attempts_by_conversation_and_idempotency_key:
        BTreeMap<AttemptIdempotencyScope, AttemptState>,
    pub delivery_receipts: BTreeMap<String, DeliveryReceiptState>,
    pub replay_decisions: BTreeMap<String, ReplayDecisionState>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttemptIdempotencyScope {
    pub conversation_id: String,
    pub idempotency_key: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversationContinuityError {
    #[error(transparent)]
    Journal(#[from] ConversationJournalError),
    #[error("conversation continuity event is invalid: {0}")]
    InvalidEvent(&'static str),
    #[error("recipient acknowledgement watermark is stale")]
    StaleAcknowledgementWatermark,
}

pub type ConversationContinuityResult<T> = Result<T, ConversationContinuityError>;

pub struct ConversationContinuityStore {
    journal: ConversationJournal,
    principal_id: String,
    authority_audit_hash: String,
}

impl ConversationContinuityStore {
    pub fn open(
        root: impl AsRef<Path>,
        principal_id: impl Into<String>,
        authority_audit_hash: impl Into<String>,
    ) -> ConversationContinuityResult<Self> {
        let principal_id = principal_id.into();
        let authority_audit_hash = authority_audit_hash.into();
        require_non_empty(&principal_id, "principal_id")?;
        require_digest(&authority_audit_hash, "authority_audit_hash")?;
        Ok(Self {
            journal: ConversationJournal::open(root)?,
            principal_id,
            authority_audit_hash,
        })
    }

    pub fn snapshot(&self) -> ConversationContinuityResult<ConversationContinuitySnapshot> {
        let journal = self.journal.snapshot()?;
        let mut snapshot = ConversationContinuitySnapshot::default();
        let mut deleted = BTreeSet::new();
        for conversation in &journal.deleted_conversations {
            deleted.insert(conversation.clone());
        }
        for event in journal.committed_events() {
            if deleted.contains(&event.conversation_id) {
                continue;
            }
            if let Some(continuity) = decode_continuity_event(event) {
                apply_continuity_event(&mut snapshot, continuity)?;
            }
        }
        Ok(snapshot)
    }

    pub fn admit_attempt(
        &self,
        conversation_id: &str,
        idempotency_key: &str,
    ) -> ConversationContinuityResult<AttemptAdmission> {
        require_non_empty(conversation_id, "conversation_id")?;
        require_non_empty(idempotency_key, "idempotency_key")?;
        let scope = AttemptIdempotencyScope {
            conversation_id: conversation_id.to_string(),
            idempotency_key: idempotency_key.to_string(),
        };
        match self
            .snapshot()?
            .attempts_by_conversation_and_idempotency_key
            .get(&scope)
        {
            Some(state) if state.outcome == AttemptOutcome::Completed => {
                Ok(AttemptAdmission::DuplicateCompleted {
                    attempt_id: state.attempt_id.clone(),
                    receipt_id: state.receipt_id.clone(),
                })
            }
            Some(state) if state.outcome == AttemptOutcome::DispatchedAmbiguous => {
                Ok(AttemptAdmission::DuplicateAmbiguous {
                    attempt_id: state.attempt_id.clone(),
                    receipt_id: state.receipt_id.clone(),
                })
            }
            _ => Ok(AttemptAdmission::NewOrRetryable),
        }
    }

    pub fn advance_sender_watermark(
        &self,
        conversation_id: impl Into<String>,
        watermark: u64,
        receipt_id: impl Into<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        self.append(
            conversation_id,
            ConversationContinuityEventKind::SenderWatermark {
                watermark,
                receipt_id: receipt_id.into(),
            },
            created_at_epoch_ms,
        )
    }

    pub fn advance_acknowledgement_watermark(
        &self,
        conversation_id: impl Into<String>,
        watermark: u64,
        acknowledgement_message_id: impl Into<String>,
        receipt_id: impl Into<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        let conversation_id = conversation_id.into();
        let current = self
            .snapshot()?
            .acknowledgement_watermarks
            .get(&conversation_id)
            .copied()
            .unwrap_or_default();
        if watermark <= current {
            return Err(ConversationContinuityError::StaleAcknowledgementWatermark);
        }
        self.append(
            conversation_id,
            ConversationContinuityEventKind::AcknowledgementWatermark {
                watermark,
                acknowledgement_message_id: acknowledgement_message_id.into(),
                receipt_id: receipt_id.into(),
            },
            created_at_epoch_ms,
        )
    }

    pub fn record_attempt(
        &self,
        conversation_id: impl Into<String>,
        attempt_id: impl Into<String>,
        idempotency_key: impl Into<String>,
        outcome: AttemptOutcome,
        receipt_id: impl Into<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        self.append(
            conversation_id,
            ConversationContinuityEventKind::Attempt {
                attempt_id: attempt_id.into(),
                idempotency_key: idempotency_key.into(),
                outcome,
                receipt_id: receipt_id.into(),
            },
            created_at_epoch_ms,
        )
    }

    pub fn record_delivery_receipt(
        &self,
        conversation_id: impl Into<String>,
        attempt_id: impl Into<String>,
        receipt_id: impl Into<String>,
        response_receipt_id: Option<String>,
        acknowledgement_receipt_id: Option<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        self.append(
            conversation_id,
            ConversationContinuityEventKind::DeliveryReceipt {
                attempt_id: attempt_id.into(),
                receipt_id: receipt_id.into(),
                response_receipt_id,
                acknowledgement_receipt_id,
            },
            created_at_epoch_ms,
        )
    }

    pub fn record_replay_decision(
        &self,
        conversation_id: impl Into<String>,
        replay_id: impl Into<String>,
        owner_id: impl Into<String>,
        high_watermark: u64,
        receipt_id: impl Into<String>,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        self.append(
            conversation_id,
            ConversationContinuityEventKind::ReplayDecision {
                replay_id: replay_id.into(),
                owner_id: owner_id.into(),
                high_watermark,
                receipt_id: receipt_id.into(),
            },
            created_at_epoch_ms,
        )
    }

    fn append(
        &self,
        conversation_id: impl Into<String>,
        event: ConversationContinuityEventKind,
        created_at_epoch_ms: u64,
    ) -> ConversationContinuityResult<ConversationJournalRecord> {
        let continuity = ConversationContinuityEvent {
            schema: CONVERSATION_CONTINUITY_SCHEMA.to_string(),
            conversation_id: conversation_id.into(),
            event,
            created_at_epoch_ms,
        };
        validate_continuity_event(&continuity)?;
        let payload = serde_jcs::to_vec(&continuity)
            .map_err(|_| ConversationContinuityError::InvalidEvent("payload"))?;
        let payload_hash = blake3::hash(&payload).to_hex().to_string();
        let encoded_payload = hex::encode(payload);
        self.journal
            .append_event(ConversationJournalEvent {
                event_id: continuity_event_id(&continuity)?,
                conversation_id: continuity.conversation_id.clone(),
                principal_id: self.principal_id.clone(),
                authority_audit_hash: self.authority_audit_hash.clone(),
                payload_hash,
                receipt_ref: Some(format!("continuity-jcs-hex:{encoded_payload}")),
                created_at_epoch_ms,
            })
            .map_err(Into::into)
    }
}

fn decode_continuity_event(
    event: &ConversationJournalEvent,
) -> Option<ConversationContinuityEvent> {
    let encoded = event
        .receipt_ref
        .as_ref()?
        .strip_prefix("continuity-jcs-hex:")?;
    let bytes = hex::decode(encoded).ok()?;
    let continuity: ConversationContinuityEvent = serde_json::from_slice(&bytes).ok()?;
    let canonical = serde_jcs::to_vec(&continuity).ok()?;
    let digest = blake3::hash(&canonical).to_hex().to_string();
    (digest == event.payload_hash && continuity.conversation_id == event.conversation_id)
        .then_some(continuity)
}

fn apply_continuity_event(
    snapshot: &mut ConversationContinuitySnapshot,
    event: ConversationContinuityEvent,
) -> ConversationContinuityResult<()> {
    validate_continuity_event(&event)?;
    match event.event {
        ConversationContinuityEventKind::SenderWatermark {
            watermark,
            receipt_id: _,
        } => {
            snapshot
                .sender_watermarks
                .entry(event.conversation_id)
                .and_modify(|current| *current = (*current).max(watermark))
                .or_insert(watermark);
        }
        ConversationContinuityEventKind::AcknowledgementWatermark {
            watermark,
            acknowledgement_message_id: _,
            receipt_id: _,
        } => {
            snapshot
                .acknowledgement_watermarks
                .entry(event.conversation_id)
                .and_modify(|current| *current = (*current).max(watermark))
                .or_insert(watermark);
        }
        ConversationContinuityEventKind::Attempt {
            attempt_id,
            idempotency_key,
            outcome,
            receipt_id,
        } => {
            let scope = AttemptIdempotencyScope {
                conversation_id: event.conversation_id.clone(),
                idempotency_key: idempotency_key.clone(),
            };
            let state = AttemptState {
                conversation_id: event.conversation_id,
                attempt_id,
                idempotency_key,
                outcome,
                receipt_id,
            };
            match snapshot
                .attempts_by_conversation_and_idempotency_key
                .entry(scope)
            {
                Entry::Occupied(mut existing)
                    if attempt_outcome_priority(outcome)
                        >= attempt_outcome_priority(existing.get().outcome) =>
                {
                    existing.insert(state);
                }
                Entry::Occupied(_) => {}
                Entry::Vacant(vacant) => {
                    vacant.insert(state);
                }
            }
        }
        ConversationContinuityEventKind::ReplayDecision {
            replay_id,
            owner_id,
            high_watermark,
            receipt_id,
        } => {
            snapshot.replay_decisions.insert(
                replay_id.clone(),
                ReplayDecisionState {
                    conversation_id: event.conversation_id,
                    replay_id,
                    owner_id,
                    high_watermark,
                    receipt_id,
                },
            );
        }
        ConversationContinuityEventKind::DeliveryReceipt {
            attempt_id,
            receipt_id,
            response_receipt_id,
            acknowledgement_receipt_id,
        } => {
            snapshot.delivery_receipts.insert(
                receipt_id.clone(),
                DeliveryReceiptState {
                    conversation_id: event.conversation_id,
                    attempt_id,
                    receipt_id,
                    response_receipt_id,
                    acknowledgement_receipt_id,
                },
            );
        }
    }
    Ok(())
}

fn attempt_outcome_priority(outcome: AttemptOutcome) -> u8 {
    match outcome {
        AttemptOutcome::PreDispatchRetryable => 0,
        AttemptOutcome::DispatchedAmbiguous => 1,
        AttemptOutcome::Completed => 2,
    }
}

fn continuity_event_id(
    event: &ConversationContinuityEvent,
) -> ConversationContinuityResult<String> {
    let suffix = match &event.event {
        ConversationContinuityEventKind::SenderWatermark { watermark, .. } => {
            format!("sender-watermark-{watermark}")
        }
        ConversationContinuityEventKind::AcknowledgementWatermark { watermark, .. } => {
            format!("ack-watermark-{watermark}")
        }
        ConversationContinuityEventKind::Attempt {
            attempt_id,
            idempotency_key,
            outcome,
            ..
        } => format!("attempt-{attempt_id}-{idempotency_key}-{outcome:?}"),
        ConversationContinuityEventKind::ReplayDecision { replay_id, .. } => {
            format!("replay-{replay_id}")
        }
        ConversationContinuityEventKind::DeliveryReceipt {
            attempt_id,
            receipt_id,
            ..
        } => format!("receipt-{attempt_id}-{receipt_id}"),
    };
    Ok(format!("continuity:{}:{suffix}", event.conversation_id))
}

fn validate_continuity_event(
    event: &ConversationContinuityEvent,
) -> ConversationContinuityResult<()> {
    if event.schema != CONVERSATION_CONTINUITY_SCHEMA {
        return Err(ConversationContinuityError::InvalidEvent("schema"));
    }
    require_non_empty(&event.conversation_id, "conversation_id")?;
    match &event.event {
        ConversationContinuityEventKind::SenderWatermark {
            watermark: _,
            receipt_id,
        } => require_non_empty(receipt_id, "receipt_id")?,
        ConversationContinuityEventKind::AcknowledgementWatermark {
            watermark: _,
            acknowledgement_message_id,
            receipt_id,
        } => {
            require_non_empty(acknowledgement_message_id, "acknowledgement_message_id")?;
            require_non_empty(receipt_id, "receipt_id")?;
        }
        ConversationContinuityEventKind::Attempt {
            attempt_id,
            idempotency_key,
            receipt_id,
            ..
        } => {
            require_non_empty(attempt_id, "attempt_id")?;
            require_non_empty(idempotency_key, "idempotency_key")?;
            require_non_empty(receipt_id, "receipt_id")?;
        }
        ConversationContinuityEventKind::ReplayDecision {
            replay_id,
            owner_id,
            receipt_id,
            ..
        } => {
            require_non_empty(replay_id, "replay_id")?;
            require_non_empty(owner_id, "owner_id")?;
            require_non_empty(receipt_id, "receipt_id")?;
        }
        ConversationContinuityEventKind::DeliveryReceipt {
            attempt_id,
            receipt_id,
            ..
        } => {
            require_non_empty(attempt_id, "attempt_id")?;
            require_non_empty(receipt_id, "receipt_id")?;
        }
    }
    Ok(())
}

fn require_non_empty(value: &str, field: &'static str) -> ConversationContinuityResult<()> {
    if value.trim().is_empty() {
        return Err(ConversationContinuityError::InvalidEvent(field));
    }
    Ok(())
}

fn require_digest(value: &str, field: &'static str) -> ConversationContinuityResult<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ConversationContinuityError::InvalidEvent(field));
    }
    Ok(())
}
