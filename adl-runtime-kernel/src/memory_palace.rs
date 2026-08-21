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
pub const MEMORY_PALACE_PACKET_SCHEMA: &str = "adl.memory_palace.context_packet.v2";
const MEMORY_PALACE_AUTHORITY_SCHEMA: &str = "adl.memory_palace.authority.v1";
pub const MEMORY_PALACE_CONTEXT_CACHE_SCHEMA: &str = "adl.memory_palace.context_cache.v1";

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

/// Wire-compatible subset of the authoritative `adl::obsmem_contract::MemoryRecord`.
/// The adapter consumes this normalized shape so callers cannot invent a parallel
/// context-record identity at the Memory Palace boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizedObsMemRecord {
    pub id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub tags: Vec<String>,
    pub payload: String,
    pub score: String,
    pub citations: Vec<NormalizedObsMemCitation>,
    pub trace_event_refs: Vec<NormalizedObsMemTraceRef>,
    pub temporal_anchor: Option<NormalizedObsMemTemporalAnchor>,
    pub review_findings: Vec<serde_json::Value>,
    pub residual_risks: Vec<String>,
    pub follow_on_refs: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizedObsMemCitation {
    pub path: String,
    pub hash: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizedObsMemTraceRef {
    pub event_sequence: usize,
    pub event_kind: String,
    pub step_id: Option<String>,
    pub delegation_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NormalizedObsMemTemporalAnchor {
    pub t_created_epoch_ms: u128,
    pub t_observed_epoch_ms: Option<u128>,
    pub t_effective_epoch_ms: Option<u128>,
    pub continuity_id: Option<String>,
    pub event_sequence: Option<usize>,
}

pub fn adapt_normalized_obsmem_record(
    mut source: NormalizedObsMemRecord,
    visibility: MemoryVisibility,
    identity_root: &str,
    continuity_head: &str,
    trace_reference: &MemoryReference,
) -> Result<ObsMemContextRecord, MemoryPalaceRejection> {
    source.tags.sort();
    source.tags.dedup();
    source.citations.sort();
    source.citations.dedup();
    source.trace_event_refs.sort();
    source.trace_event_refs.dedup();
    source.residual_risks.sort();
    source.residual_risks.dedup();
    if !valid_identifier(&source.id)
        || !valid_identifier(&source.run_id)
        || !valid_identifier(&source.workflow_id)
        || source.payload.trim().is_empty()
        || source.citations.is_empty()
        || source.trace_event_refs.is_empty()
        || source
            .citations
            .iter()
            .any(|citation| !safe_repo_path(&citation.path) || !is_sha256(&citation.hash))
        || source.trace_event_refs.iter().any(|event| {
            event.event_sequence == 0
                || !valid_identifier(&event.event_kind)
                || event
                    .step_id
                    .as_deref()
                    .is_some_and(|v| !valid_identifier(v))
                || event
                    .delegation_id
                    .as_deref()
                    .is_some_and(|v| !valid_identifier(v))
        })
        || unsafe_content(&serde_jcs::to_string(&source).unwrap_or_default())
    {
        return Err(MemoryPalaceRejection::InvalidRecord { id: source.id });
    }
    let trace_digest =
        digest(&source.trace_event_refs).map_err(|_| MemoryPalaceRejection::EncodingFailure)?;
    if trace_reference.id != format!("trace:{trace_digest}")
        || !source.citations.iter().any(|citation| {
            citation.path == trace_reference.path && citation.hash == trace_reference.sha256
        })
    {
        return Err(MemoryPalaceRejection::TraceMismatch { id: source.id });
    }
    let anchor = source.temporal_anchor.as_ref().ok_or_else(|| {
        MemoryPalaceRejection::InvalidTemporalAnchor {
            id: source.id.clone(),
        }
    })?;
    if anchor.continuity_id.as_deref() != Some(continuity_head) {
        return Err(MemoryPalaceRejection::AuthorityMismatch { id: source.id });
    }
    let created_epoch_ms = u64::try_from(anchor.t_created_epoch_ms).map_err(|_| {
        MemoryPalaceRejection::InvalidTemporalAnchor {
            id: source.id.clone(),
        }
    })?;
    let observed_epoch_ms = u64::try_from(
        anchor
            .t_observed_epoch_ms
            .unwrap_or(anchor.t_created_epoch_ms),
    )
    .map_err(|_| MemoryPalaceRejection::InvalidTemporalAnchor {
        id: source.id.clone(),
    })?;
    let effective_epoch_ms = u64::try_from(
        anchor.t_effective_epoch_ms.unwrap_or(
            anchor
                .t_observed_epoch_ms
                .unwrap_or(anchor.t_created_epoch_ms),
        ),
    )
    .map_err(|_| MemoryPalaceRejection::InvalidTemporalAnchor {
        id: source.id.clone(),
    })?;
    let event_sequence = u64::try_from(anchor.event_sequence.unwrap_or(0)).map_err(|_| {
        MemoryPalaceRejection::InvalidTemporalAnchor {
            id: source.id.clone(),
        }
    })?;
    let citations = source
        .citations
        .into_iter()
        .map(|citation| MemoryReference {
            id: if citation.path == trace_reference.path && citation.hash == trace_reference.sha256
            {
                trace_reference.id.clone()
            } else {
                format!(
                    "citation:{}",
                    digest(&(citation.path.as_str(), citation.hash.as_str())).unwrap()
                )
            },
            path: citation.path,
            sha256: citation.hash,
        })
        .collect();
    Ok(ObsMemContextRecord {
        id: source.id,
        run_id: source.run_id,
        workflow_id: source.workflow_id,
        payload: source.payload,
        visibility,
        identity_root: identity_root.to_owned(),
        continuity_head: continuity_head.to_owned(),
        trace_id: trace_reference.id.clone(),
        citations,
        temporal_anchor: MemoryTemporalAnchor {
            created_epoch_ms,
            observed_epoch_ms,
            effective_epoch_ms,
            continuity_head: continuity_head.to_owned(),
            event_sequence,
        },
    })
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPalaceRecordStatus {
    Selected,
    Excluded,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceRecordIndexEntry {
    pub record_id: String,
    pub run_id: String,
    pub workflow_id: String,
    pub room_id: String,
    pub anchor_id: String,
    pub visibility: MemoryVisibility,
    pub citations: Vec<MemoryReference>,
    pub temporal_anchor: MemoryTemporalAnchor,
    pub status: MemoryPalaceRecordStatus,
    pub disposition_reason: String,
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
    pub observed_epoch_ms: u64,
    pub stale_after_ms: u64,
    pub max_working_set_items: usize,
    pub authority_sha256: String,
    pub canonical_input_sha256: String,
    pub rooms: Vec<MemoryPalaceRoom>,
    pub record_index: Vec<MemoryPalaceRecordIndexEntry>,
    pub working_set: Vec<MemoryPalaceItem>,
    pub overflow: Vec<MemoryPalaceOverflow>,
    pub packet_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPalaceContextCache {
    pub schema: String,
    pub memory_palace_generation: u64,
    pub birthday_continuity_generation: u64,
    pub identity_root: String,
    pub identity_record_sha256: String,
    pub continuity_head: String,
    pub continuity_record_sha256: String,
    pub packet_sha256: String,
    pub canonical_input_sha256: String,
    pub source_packet_ref: MemoryReference,
    pub working_set: Vec<MemoryPalaceItem>,
    pub cache_sha256: String,
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
    DuplicateRoom { id: String },
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
        if !valid_identifier(&record.id)
            || !valid_identifier(&record.run_id)
            || !valid_identifier(&record.workflow_id)
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
    let mut record_index = Vec::new();
    let mut working_set = Vec::new();
    let mut overflow = Vec::new();
    for record in records {
        let room_id = format!("room:{}", stable_id(&record.workflow_id));
        let anchor_id = anchor_id(&record.workflow_id, &record.id);
        let record_sha256 = digest(&record)?;
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
            record_index.push(MemoryPalaceRecordIndexEntry {
                record_id: record.id.clone(),
                run_id: record.run_id.clone(),
                workflow_id: record.workflow_id.clone(),
                room_id: room_id.clone(),
                anchor_id,
                visibility: record.visibility,
                citations: record.citations.clone(),
                temporal_anchor: record.temporal_anchor.clone(),
                status: MemoryPalaceRecordStatus::Selected,
                disposition_reason: "canonical traversal within configured working-set bound"
                    .to_owned(),
                record_sha256: record_sha256.clone(),
            });
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
            record_index.push(MemoryPalaceRecordIndexEntry {
                record_id: record.id.clone(),
                run_id: record.run_id.clone(),
                workflow_id: record.workflow_id.clone(),
                room_id: room_id.clone(),
                anchor_id,
                visibility: record.visibility,
                citations: record.citations.clone(),
                temporal_anchor: record.temporal_anchor.clone(),
                status: MemoryPalaceRecordStatus::Excluded,
                disposition_reason: format!(
                    "excluded after max_working_set_items={}",
                    input.max_working_set_items
                ),
                record_sha256: record_sha256.clone(),
            });
            overflow.push(MemoryPalaceOverflow {
                record_id: record.id,
                reason: format!("bounded_after_{}", input.max_working_set_items),
                record_sha256,
            });
        }
    }
    let mut rooms: Vec<_> = room_records
        .into_iter()
        .map(|(workflow_id, mut record_ids)| {
            record_ids.sort();
            MemoryPalaceRoom {
                room_id: format!("room:{}", stable_id(&workflow_id)),
                workflow_id,
                record_ids,
            }
        })
        .collect();
    rooms.sort_by(|a, b| a.workflow_id.cmp(&b.workflow_id));
    record_index.sort_by(|a, b| a.record_id.cmp(&b.record_id));
    working_set.sort_by(|a, b| a.record_id.cmp(&b.record_id));
    overflow.sort_by(|a, b| a.record_id.cmp(&b.record_id));
    let mut packet = MemoryPalaceContextPacket {
        schema: MEMORY_PALACE_PACKET_SCHEMA.to_owned(),
        identity_root: identity.identity_root.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        continuity_head: continuity.continuity_head.clone(),
        continuity_record_sha256: continuity.record_sha256.clone(),
        trace_id: input.trace_reference.id.clone(),
        trace_reference: input.trace_reference.clone(),
        redaction_policy_sha256: input.redaction_policy_sha256.clone(),
        observed_epoch_ms: input.observed_epoch_ms,
        stale_after_ms: input.stale_after_ms,
        max_working_set_items: input.max_working_set_items,
        authority_sha256,
        canonical_input_sha256,
        rooms,
        record_index,
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
        || !is_sha256(&packet.identity_root)
        || !is_sha256(&packet.identity_record_sha256)
        || !is_sha256(&packet.continuity_head)
        || !is_sha256(&packet.continuity_record_sha256)
        || !is_sha256(&packet.redaction_policy_sha256)
        || !is_sha256(&packet.authority_sha256)
        || !is_sha256(&packet.canonical_input_sha256)
        || !valid_reference(&packet.trace_reference)
        || packet.trace_id != packet.trace_reference.id
        || !packet
            .trace_reference
            .path
            .starts_with(".adl/runtime-v3/observability/")
        || packet.max_working_set_items == 0
        || packet.max_working_set_items > 64
        || packet.working_set.len() > packet.max_working_set_items
        || packet.stale_after_ms == 0
    {
        return Err(MemoryPalaceRejection::EncodingFailure);
    }
    let mut room_ids = BTreeSet::new();
    let mut room_records = BTreeMap::<&str, BTreeSet<&str>>::new();
    for room in &packet.rooms {
        if !valid_identifier(&room.workflow_id)
            || room.room_id != format!("room:{}", stable_id(&room.workflow_id))
            || !room_ids.insert(room.room_id.as_str())
            || room.record_ids.is_empty()
            || !room.record_ids.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Err(MemoryPalaceRejection::DuplicateRoom {
                id: room.room_id.clone(),
            });
        }
        let records: BTreeSet<_> = room.record_ids.iter().map(String::as_str).collect();
        if records.len() != room.record_ids.len()
            || room_records
                .insert(room.room_id.as_str(), records)
                .is_some()
        {
            return Err(MemoryPalaceRejection::DuplicateRoom {
                id: room.room_id.clone(),
            });
        }
    }
    if !packet
        .rooms
        .windows(2)
        .all(|pair| pair[0].workflow_id < pair[1].workflow_id)
        || !packet
            .working_set
            .windows(2)
            .all(|pair| pair[0].record_id < pair[1].record_id)
        || !packet
            .record_index
            .windows(2)
            .all(|pair| pair[0].record_id < pair[1].record_id)
        || !packet
            .overflow
            .windows(2)
            .all(|pair| pair[0].record_id < pair[1].record_id)
    {
        return Err(MemoryPalaceRejection::EncodingFailure);
    }
    let working_by_id: BTreeMap<_, _> = packet
        .working_set
        .iter()
        .map(|item| (item.record_id.as_str(), item))
        .collect();
    let overflow_by_id: BTreeMap<_, _> = packet
        .overflow
        .iter()
        .map(|overflow| (overflow.record_id.as_str(), overflow))
        .collect();
    let mut indexed_records = BTreeSet::new();
    for entry in &packet.record_index {
        if !valid_identifier(&entry.record_id)
            || !valid_identifier(&entry.run_id)
            || !valid_identifier(&entry.workflow_id)
            || !indexed_records.insert(entry.record_id.as_str())
            || entry.room_id != format!("room:{}", stable_id(&entry.workflow_id))
            || entry.anchor_id != anchor_id(&entry.workflow_id, &entry.record_id)
            || matches!(
                entry.visibility,
                MemoryVisibility::Private | MemoryVisibility::RawPrivate
            )
            || entry.citations.is_empty()
            || entry
                .citations
                .iter()
                .any(|citation| !valid_reference(citation))
            || !entry
                .citations
                .iter()
                .any(|citation| citation == &packet.trace_reference)
            || !entry.citations.windows(2).all(|pair| pair[0] < pair[1])
            || entry.temporal_anchor.continuity_head != packet.continuity_head
            || entry.temporal_anchor.created_epoch_ms > entry.temporal_anchor.observed_epoch_ms
            || entry.temporal_anchor.effective_epoch_ms < entry.temporal_anchor.created_epoch_ms
            || entry.temporal_anchor.effective_epoch_ms > entry.temporal_anchor.observed_epoch_ms
            || entry.temporal_anchor.event_sequence == 0
            || entry.temporal_anchor.observed_epoch_ms > packet.observed_epoch_ms
            || packet
                .observed_epoch_ms
                .saturating_sub(entry.temporal_anchor.effective_epoch_ms)
                > packet.stale_after_ms
            || !room_records
                .get(entry.room_id.as_str())
                .is_some_and(|records| records.contains(entry.record_id.as_str()))
            || !is_sha256(&entry.record_sha256)
        {
            return Err(MemoryPalaceRejection::InvalidRecord {
                id: entry.record_id.clone(),
            });
        }
        match entry.status {
            MemoryPalaceRecordStatus::Selected => {
                let Some(item) = working_by_id.get(entry.record_id.as_str()) else {
                    return Err(MemoryPalaceRejection::InvalidRecord {
                        id: entry.record_id.clone(),
                    });
                };
                if item.room_id != entry.room_id
                    || item.visibility != entry.visibility
                    || item.citations != entry.citations
                    || item.temporal_anchor != entry.temporal_anchor
                    || entry.disposition_reason
                        != "canonical traversal within configured working-set bound"
                {
                    return Err(MemoryPalaceRejection::InvalidRecord {
                        id: entry.record_id.clone(),
                    });
                }
                let expected_record_sha256 = digest(&ObsMemContextRecord {
                    id: entry.record_id.clone(),
                    run_id: entry.run_id.clone(),
                    workflow_id: entry.workflow_id.clone(),
                    payload: item.payload.clone(),
                    visibility: entry.visibility,
                    identity_root: packet.identity_root.clone(),
                    continuity_head: packet.continuity_head.clone(),
                    trace_id: packet.trace_id.clone(),
                    citations: entry.citations.clone(),
                    temporal_anchor: entry.temporal_anchor.clone(),
                })
                .map_err(|_| MemoryPalaceRejection::EncodingFailure)?;
                if entry.record_sha256 != expected_record_sha256 {
                    return Err(MemoryPalaceRejection::InvalidRecord {
                        id: entry.record_id.clone(),
                    });
                }
            }
            MemoryPalaceRecordStatus::Excluded => {
                let Some(overflow) = overflow_by_id.get(entry.record_id.as_str()) else {
                    return Err(MemoryPalaceRejection::InvalidRecord {
                        id: entry.record_id.clone(),
                    });
                };
                if overflow.record_sha256 != entry.record_sha256
                    || overflow.reason != format!("bounded_after_{}", packet.max_working_set_items)
                    || entry.disposition_reason
                        != format!(
                            "excluded after max_working_set_items={}",
                            packet.max_working_set_items
                        )
                {
                    return Err(MemoryPalaceRejection::InvalidRecord {
                        id: entry.record_id.clone(),
                    });
                }
            }
        }
    }
    let mut seen_records = BTreeSet::new();
    for item in &packet.working_set {
        if !valid_identifier(&item.record_id)
            || !seen_records.insert(item.record_id.as_str())
            || !room_records
                .get(item.room_id.as_str())
                .is_some_and(|records| records.contains(item.record_id.as_str()))
            || matches!(
                item.visibility,
                MemoryVisibility::Private | MemoryVisibility::RawPrivate
            )
            || unsafe_content(&item.payload)
            || (item.visibility == MemoryVisibility::Redacted && item.payload != "[REDACTED]")
            || item.citations.is_empty()
            || item
                .citations
                .iter()
                .any(|citation| !valid_reference(citation))
            || !item
                .citations
                .iter()
                .any(|citation| citation == &packet.trace_reference)
            || item.temporal_anchor.continuity_head != packet.continuity_head
            || item.temporal_anchor.created_epoch_ms > item.temporal_anchor.observed_epoch_ms
            || item.temporal_anchor.effective_epoch_ms < item.temporal_anchor.created_epoch_ms
            || item.temporal_anchor.effective_epoch_ms > item.temporal_anchor.observed_epoch_ms
            || item.temporal_anchor.event_sequence == 0
            || item.temporal_anchor.observed_epoch_ms > packet.observed_epoch_ms
            || packet
                .observed_epoch_ms
                .saturating_sub(item.temporal_anchor.effective_epoch_ms)
                > packet.stale_after_ms
            || !item.citations.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Err(MemoryPalaceRejection::InvalidRecord {
                id: item.record_id.clone(),
            });
        }
        let expected = digest(&(
            &item.record_id,
            &item.room_id,
            &item.payload,
            item.visibility,
            &item.citations,
            &item.temporal_anchor,
            &packet.authority_sha256,
        ))
        .map_err(|_| MemoryPalaceRejection::EncodingFailure)?;
        if item.item_sha256 != expected {
            return Err(MemoryPalaceRejection::EncodingFailure);
        }
    }
    for overflow in &packet.overflow {
        if !valid_identifier(&overflow.record_id)
            || !seen_records.insert(overflow.record_id.as_str())
            || !is_sha256(&overflow.record_sha256)
            || overflow.reason != format!("bounded_after_{}", packet.max_working_set_items)
        {
            return Err(MemoryPalaceRejection::InvalidRecord {
                id: overflow.record_id.clone(),
            });
        }
    }
    let declared: BTreeSet<_> = room_records
        .values()
        .flat_map(|records| records.iter().copied())
        .collect();
    if declared != seen_records || declared != indexed_records {
        return Err(MemoryPalaceRejection::EncodingFailure);
    }
    Ok(())
}

pub fn build_memory_palace_context_cache(
    memory_palace_generation: u64,
    birthday_continuity_generation: u64,
    source_packet_ref: MemoryReference,
    packet: &MemoryPalaceContextPacket,
) -> Result<MemoryPalaceContextCache, Vec<MemoryPalaceRejection>> {
    validate_memory_palace_packet(packet).map_err(|error| vec![error])?;
    if memory_palace_generation == 0 || birthday_continuity_generation == 0 {
        return Err(vec![MemoryPalaceRejection::InvalidAuthority]);
    }
    let mut cache = MemoryPalaceContextCache {
        schema: MEMORY_PALACE_CONTEXT_CACHE_SCHEMA.to_owned(),
        memory_palace_generation,
        birthday_continuity_generation,
        identity_root: packet.identity_root.clone(),
        identity_record_sha256: packet.identity_record_sha256.clone(),
        continuity_head: packet.continuity_head.clone(),
        continuity_record_sha256: packet.continuity_record_sha256.clone(),
        packet_sha256: packet.packet_sha256.clone(),
        canonical_input_sha256: packet.canonical_input_sha256.clone(),
        source_packet_ref,
        working_set: packet.working_set.clone(),
        cache_sha256: String::new(),
    };
    cache.cache_sha256 = digest(&cache)?;
    Ok(cache)
}

pub fn validate_memory_palace_context_cache(
    cache: &MemoryPalaceContextCache,
    packet: &MemoryPalaceContextPacket,
) -> Result<(), MemoryPalaceRejection> {
    validate_memory_palace_packet(packet)?;
    let expected = build_memory_palace_context_cache(
        cache.memory_palace_generation,
        cache.birthday_continuity_generation,
        cache.source_packet_ref.clone(),
        packet,
    )
    .map_err(|_| MemoryPalaceRejection::EncodingFailure)?;
    if cache.schema != MEMORY_PALACE_CONTEXT_CACHE_SCHEMA
        || cache.cache_sha256 != expected.cache_sha256
        || cache != &expected
    {
        return Err(MemoryPalaceRejection::EncodingFailure);
    }
    Ok(())
}

fn valid_reference(reference: &MemoryReference) -> bool {
    valid_identifier(&reference.id)
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
        "/home/",
        "/private/",
        "private_state",
        "raw-state",
        "raw_state",
        "sealed_payload",
        "bearer ",
        "gho_",
        "sk-",
        "api_key",
        "private key",
        "raw_chat_transcript",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 256
        && !unsafe_content(value)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn stable_id(value: &str) -> String {
    let slug: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    let suffix = format!("{:x}", Sha256::digest(value.as_bytes()));
    format!("{slug}-{}", &suffix[..16])
}

fn anchor_id(workflow_id: &str, record_id: &str) -> String {
    format!("anchor:{}:{}", stable_id(workflow_id), stable_id(record_id))
}

fn digest<T: Serialize>(value: &T) -> Result<String, Vec<MemoryPalaceRejection>> {
    serde_jcs::to_vec(value)
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| vec![MemoryPalaceRejection::EncodingFailure])
}
