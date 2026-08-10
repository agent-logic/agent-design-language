use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    birthday_identity::record_digest as identity_record_digest, continuity_record_digest,
    BirthdayContinuityRecord, BirthdayIdentityRecord,
};

pub const MEMORY_PALACE_INPUT_SCHEMA: &str = "adl.memory_palace.input.v1";
pub const MEMORY_PALACE_PACKET_SCHEMA: &str = "adl.memory_palace.context_packet.v1";
const MEMORY_PALACE_AUTHORITY_SCHEMA: &str = "adl.memory_palace.authority.v1";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryReference {
    pub id: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryVisibility {
    Public,
    Redacted,
    Private,
    RawPrivate,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryTemporalAnchor {
    pub created_epoch_ms: u64,
    pub observed_epoch_ms: u64,
    pub effective_epoch_ms: u64,
    pub continuity_head: String,
    pub event_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObsMemContextRecord {
    pub id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub payload: String,
    pub visibility: MemoryVisibility,
    pub identity_root: String,
    pub continuity_head: String,
    pub trace_id: String,
    pub citations: Vec<MemoryReference>,
    pub temporal_anchor: MemoryTemporalAnchor,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceInput {
    pub schema: String,
    pub identity_record_sha256: String,
    pub continuity_record_sha256: String,
    pub trace_reference: MemoryReference,
    pub redaction_policy_sha256: String,
    pub observed_epoch_ms: u64,
    pub stale_after_ms: u64,
    pub max_working_set_items: usize,
    pub records: Vec<ObsMemContextRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceRoom {
    pub room_id: String,
    pub workflow_id: String,
    pub record_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceItem {
    pub record_id: String,
    pub room_id: String,
    pub payload: String,
    pub visibility: MemoryVisibility,
    pub citations: Vec<MemoryReference>,
    pub temporal_anchor: MemoryTemporalAnchor,
    pub item_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceOverflow {
    pub record_id: String,
    pub reason: String,
    pub record_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceContextPacket {
    pub schema: String,
    pub identity_root: String,
    pub identity_record_sha256: String,
    pub continuity_head: String,
    pub continuity_record_sha256: String,
    pub trace_id: String,
    pub trace_reference: MemoryReference,
    pub redaction_policy_sha256: String,
    pub authority_sha256: String,
    pub canonical_input_sha256: String,
    pub rooms: Vec<MemoryPalaceRoom>,
    pub working_set: Vec<MemoryPalaceItem>,
    pub overflow: Vec<MemoryPalaceOverflow>,
    pub packet_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum MemoryPalaceRejection {
    UnsupportedSchema,
    IdentityRecordMismatch,
    ContinuityRecordMismatch,
    ContinuityIdentityMismatch,
    InvalidAuthority,
    InvalidBounds,
    EmptyRecords,
    InvalidRecord { id: String },
    DuplicateRecord { id: String },
    AuthorityMismatch { id: String },
    TraceMismatch { id: String },
    InvalidCitation { id: String },
    DuplicateCitation { id: String },
    MissingCitation { id: String },
    UnsafeCitationPath { id: String },
    CitationDigestMismatch { id: String },
    InvalidTemporalAnchor { id: String },
    StaleContext { id: String },
    PrivateMemoryAccess { id: String },
    UnsafePayload { id: String },
    InvalidRedaction { id: String },
    EncodingFailure,
}

pub fn build_memory_palace(
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    input: &MemoryPalaceInput,
) -> Result<MemoryPalaceContextPacket, Vec<MemoryPalaceRejection>> {
    let mut errors = BTreeSet::new();
    if input.schema != MEMORY_PALACE_INPUT_SCHEMA {
        errors.insert(MemoryPalaceRejection::UnsupportedSchema);
    }
    if identity_record_digest(identity).ok().as_deref() != Some(&identity.record_sha256)
        || input.identity_record_sha256 != identity.record_sha256
    {
        errors.insert(MemoryPalaceRejection::IdentityRecordMismatch);
    }
    if continuity_record_digest(continuity).ok().as_deref() != Some(&continuity.record_sha256)
        || input.continuity_record_sha256 != continuity.record_sha256
    {
        errors.insert(MemoryPalaceRejection::ContinuityRecordMismatch);
    }
    if continuity.identity_root != identity.identity_root
        || continuity.identity_record_sha256 != identity.record_sha256
    {
        errors.insert(MemoryPalaceRejection::ContinuityIdentityMismatch);
    }
    if !valid_reference(&input.trace_reference)
        || !input
            .trace_reference
            .path
            .starts_with(".adl/runtime-v3/observability/")
        || !is_sha256(&input.redaction_policy_sha256)
    {
        errors.insert(MemoryPalaceRejection::InvalidAuthority);
    }
    if input.max_working_set_items == 0
        || input.max_working_set_items > 64
        || input.stale_after_ms == 0
    {
        errors.insert(MemoryPalaceRejection::InvalidBounds);
    }
    if input.records.is_empty() {
        errors.insert(MemoryPalaceRejection::EmptyRecords);
    }

    let mut records = input.records.clone();
    records.sort_by(|a, b| {
        a.workflow_id
            .cmp(&b.workflow_id)
            .then_with(|| {
                a.temporal_anchor
                    .effective_epoch_ms
                    .cmp(&b.temporal_anchor.effective_epoch_ms)
            })
            .then_with(|| a.run_id.cmp(&b.run_id))
            .then_with(|| a.id.cmp(&b.id))
    });
    let mut seen_records = BTreeSet::new();
    for record in &mut records {
        record.citations.sort();
        if record.id.trim().is_empty()
            || record.run_id.trim().is_empty()
            || record.workflow_id.trim().is_empty()
            || record.payload.trim().is_empty()
        {
            errors.insert(MemoryPalaceRejection::InvalidRecord {
                id: record.id.clone(),
            });
        }
        if !seen_records.insert(record.id.clone()) {
            errors.insert(MemoryPalaceRejection::DuplicateRecord {
                id: record.id.clone(),
            });
        }
        if record.identity_root != identity.identity_root
            || record.continuity_head != continuity.continuity_head
            || record.temporal_anchor.continuity_head != continuity.continuity_head
        {
            errors.insert(MemoryPalaceRejection::AuthorityMismatch {
                id: record.id.clone(),
            });
        }
        if record.trace_id != input.trace_reference.id {
            errors.insert(MemoryPalaceRejection::TraceMismatch {
                id: record.id.clone(),
            });
        }
        if record.citations.is_empty() {
            errors.insert(MemoryPalaceRejection::MissingCitation {
                id: record.id.clone(),
            });
        }
        let mut seen_citations = BTreeSet::new();
        for citation in &record.citations {
            if !seen_citations.insert((citation.id.clone(), citation.path.clone())) {
                errors.insert(MemoryPalaceRejection::DuplicateCitation {
                    id: record.id.clone(),
                });
            }
            if citation.id.trim().is_empty() || !is_sha256(&citation.sha256) {
                errors.insert(MemoryPalaceRejection::InvalidCitation {
                    id: record.id.clone(),
                });
            }
            if !safe_repo_path(&citation.path) {
                errors.insert(MemoryPalaceRejection::UnsafeCitationPath {
                    id: record.id.clone(),
                });
            }
        }
        if record.citations.iter().any(|citation| {
            citation.id == input.trace_reference.id
                && (citation.path != input.trace_reference.path
                    || citation.sha256 != input.trace_reference.sha256)
        }) || !record
            .citations
            .iter()
            .any(|citation| citation.id == input.trace_reference.id)
        {
            errors.insert(MemoryPalaceRejection::CitationDigestMismatch {
                id: record.id.clone(),
            });
        }
        let anchor = &record.temporal_anchor;
        if anchor.created_epoch_ms > anchor.observed_epoch_ms
            || anchor.observed_epoch_ms > input.observed_epoch_ms
            || anchor.effective_epoch_ms < anchor.created_epoch_ms
            || anchor.effective_epoch_ms > anchor.observed_epoch_ms
            || anchor.event_sequence == 0
        {
            errors.insert(MemoryPalaceRejection::InvalidTemporalAnchor {
                id: record.id.clone(),
            });
        } else if input
            .observed_epoch_ms
            .saturating_sub(anchor.effective_epoch_ms)
            > input.stale_after_ms
        {
            errors.insert(MemoryPalaceRejection::StaleContext {
                id: record.id.clone(),
            });
        }
        if matches!(
            record.visibility,
            MemoryVisibility::Private | MemoryVisibility::RawPrivate
        ) {
            errors.insert(MemoryPalaceRejection::PrivateMemoryAccess {
                id: record.id.clone(),
            });
        }
        if unsafe_content(&record.payload) {
            errors.insert(MemoryPalaceRejection::UnsafePayload {
                id: record.id.clone(),
            });
        }
        if record.visibility == MemoryVisibility::Redacted && record.payload != "[REDACTED]" {
            errors.insert(MemoryPalaceRejection::InvalidRedaction {
                id: record.id.clone(),
            });
        }
    }
    if !errors.is_empty() {
        return Err(errors.into_iter().collect());
    }

    let authority_sha256 = digest(&(
        MEMORY_PALACE_AUTHORITY_SCHEMA,
        &identity.identity_root,
        &identity.record_sha256,
        &continuity.continuity_head,
        &continuity.record_sha256,
        &input.trace_reference,
        &input.redaction_policy_sha256,
    ))?;
    let canonical_input_sha256 = digest(&records)?;
    let mut room_records = BTreeMap::<String, Vec<String>>::new();
    let mut working_set = Vec::new();
    let mut overflow = Vec::new();
    for record in records {
        let room_id = format!("room:{}", stable_id(&record.workflow_id));
        room_records
            .entry(record.workflow_id.clone())
            .or_default()
            .push(record.id.clone());
        let item_sha256 = digest(&(
            &record.id,
            &room_id,
            &record.payload,
            record.visibility,
            &record.citations,
            &record.temporal_anchor,
            &authority_sha256,
        ))?;
        if working_set.len() < input.max_working_set_items {
            working_set.push(MemoryPalaceItem {
                record_id: record.id,
                room_id,
                payload: record.payload,
                visibility: record.visibility,
                citations: record.citations,
                temporal_anchor: record.temporal_anchor,
                item_sha256,
            });
        } else {
            overflow.push(MemoryPalaceOverflow {
                record_id: record.id,
                reason: format!("bounded_after_{}", input.max_working_set_items),
                record_sha256: item_sha256,
            });
        }
    }
    let rooms = room_records
        .into_iter()
        .map(|(workflow_id, record_ids)| MemoryPalaceRoom {
            room_id: format!("room:{}", stable_id(&workflow_id)),
            workflow_id,
            record_ids,
        })
        .collect();
    let mut packet = MemoryPalaceContextPacket {
        schema: MEMORY_PALACE_PACKET_SCHEMA.to_owned(),
        identity_root: identity.identity_root.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        continuity_head: continuity.continuity_head.clone(),
        continuity_record_sha256: continuity.record_sha256.clone(),
        trace_id: input.trace_reference.id.clone(),
        trace_reference: input.trace_reference.clone(),
        redaction_policy_sha256: input.redaction_policy_sha256.clone(),
        authority_sha256,
        canonical_input_sha256,
        rooms,
        working_set,
        overflow,
        packet_sha256: String::new(),
    };
    packet.packet_sha256 = digest(&packet)?;
    Ok(packet)
}

pub fn validate_memory_palace_packet(
    packet: &MemoryPalaceContextPacket,
) -> Result<(), MemoryPalaceRejection> {
    let mut unsigned = packet.clone();
    unsigned.packet_sha256.clear();
    if packet.schema != MEMORY_PALACE_PACKET_SCHEMA
        || digest(&unsigned).ok().as_deref() != Some(&packet.packet_sha256)
    {
        return Err(MemoryPalaceRejection::EncodingFailure);
    }
    Ok(())
}

fn valid_reference(reference: &MemoryReference) -> bool {
    !reference.id.trim().is_empty()
        && safe_repo_path(&reference.path)
        && is_sha256(&reference.sha256)
}

fn safe_repo_path(value: &str) -> bool {
    if value.trim().is_empty() || value.contains('\\') || unsafe_content(value) {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn unsafe_content(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "/users/",
        "/private/",
        "private_state",
        "raw-state",
        "raw_state",
        "sealed_payload",
        "bearer ",
        "api_key",
        "private key",
        "raw_chat_transcript",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn stable_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn digest<T: Serialize>(value: &T) -> Result<String, Vec<MemoryPalaceRejection>> {
    serde_jcs::to_vec(value)
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| vec![MemoryPalaceRejection::EncodingFailure])
}
