//! Durable, redacted observation of a verifier-authenticated child-authority pair.
//!
//! This module never issues authority. Its only authority-bearing input is a
//! borrowed [`VerifiedCommittedChildLineagePair`] returned by the child verifier.
//!
//! A pair cannot be caller-constructed or passed by value:
//!
//! ```compile_fail
//! use adl_runtime::distributed::shepherd_serving_eligibility::VerifiedCommittedChildLineagePair;
//! let pair = VerifiedCommittedChildLineagePair { shepherd: todo!(), observatory: todo!() };
//! let _ = adl_runtime::distributed::integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::observe_owned(pair);
//! ```
//!
//! Separate sealed children cannot enter the integrated API:
//!
//! ```compile_fail
//! # use adl_runtime::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection;
//! # use adl_runtime::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection;
//! fn inject(s: &SealedShepherdCommittedProjection, o: &SealedObservatoryCommittedProjection) {
//!     let _ = adl_runtime::distributed::integrated_serving_authority_snapshot::IntegratedServingAuthoritySnapshotStore::observe_children(s, o);
//! }
//! ```
//!
//! Raw lineage and caller eligibility flags are not accepted:
//!
//! ```compile_fail
//! use adl_runtime::distributed::integrated_serving_authority_snapshot::{IntegratedOutcome, IntegratedServingAuthoritySnapshotStore};
//! fn inject(store: &mut IntegratedServingAuthoritySnapshotStore) {
//!     let _ = store.observe("operation", "raw-lineage", true, IntegratedOutcome::Success);
//! }
//! ```

use super::{
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
    shepherd_serving_eligibility::VerifiedCommittedChildLineagePair,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path, sync::Arc};

const DOMAIN: &str = "ADL-INTEGRATED-SERVING-AUTHORITY-SNAPSHOT-V1";
const STATE_PREFIX_DOMAIN: &str = "ADL-INTEGRATED-SERVING-AUTHORITY-STATE-PREFIX-V1";
const SCHEMA: &str = "adl.integrated-serving-authority-snapshot.v1";
const MAX_OPERATIONS: usize = 4096;
const IJSON_MAX_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegratedOutcome {
    Success,
    NoOp,
    Rejection,
    Recovery,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegratedSnapshotError {
    InvalidInput,
    RetryConflict,
    CapacityExceeded,
    Storage,
    Serialization,
}

impl From<PolisRuntimeError> for IntegratedSnapshotError {
    fn from(_: PolisRuntimeError) -> Self {
        Self::Storage
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RedactedChildProjection {
    pub child_kind: String,
    pub lineage_ref: String,
    pub status: String,
    pub authority_ref: Option<String>,
    pub generation: u64,
    pub fencing_generation: u64,
    pub committed_revision: u64,
    pub envelope_generation: u64,
    pub payload_sha256: String,
    pub state_sha256: String,
    pub receipt_sha256: String,
    pub canonical_sha256: String,
    pub provenance_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegratedSnapshotReceipt {
    pub schema: String,
    pub operation_ref: String,
    pub outcome: IntegratedOutcome,
    pub input_sha256: String,
    pub prior_state_sha256: String,
    pub result_state_sha256: String,
    pub receipt_sha256: String,
    pub shepherd: RedactedChildProjection,
    pub observatory: RedactedChildProjection,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    revision: u64,
    operations: BTreeMap<String, IntegratedSnapshotReceipt>,
}

#[derive(Serialize)]
struct StatePrefix<'a> {
    domain: &'static str,
    revision: u64,
    receipts: &'a [IntegratedSnapshotReceipt],
}

impl CheckpointMetadataSource for State {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.revision),
            state_sha256: Some(state_digest(self).map_err(|_| PolisRuntimeError::Serialization)?),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}

pub struct IntegratedServingAuthoritySnapshotStore {
    store: CheckpointedJson<State>,
    envelope: DurableEnvelope<State>,
    capacity: usize,
    recovery_source_revision: Option<u64>,
}

impl IntegratedServingAuthoritySnapshotStore {
    pub fn open(
        root: &Path,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> Result<Self, IntegratedSnapshotError> {
        if capacity == 0 || capacity > MAX_OPERATIONS {
            return Err(IntegratedSnapshotError::CapacityExceeded);
        }
        let (store, envelope) = CheckpointedJson::open(
            root,
            "integrated-serving-authority-snapshot",
            "integrated-serving-authority-snapshot.json",
            State::default(),
            authority,
        )?;
        validate_state(envelope.payload(), capacity)?;
        let recovery_source_revision =
            (envelope.payload().revision > 0).then_some(envelope.payload().revision);
        Ok(Self {
            store,
            envelope,
            capacity,
            recovery_source_revision,
        })
    }

    pub fn observe(
        &mut self,
        operation_ref: &str,
        pair: &VerifiedCommittedChildLineagePair<'_>,
        outcome: IntegratedOutcome,
    ) -> Result<IntegratedSnapshotReceipt, IntegratedSnapshotError> {
        if outcome == IntegratedOutcome::Recovery {
            return Err(IntegratedSnapshotError::InvalidInput);
        }
        validate_identifier(operation_ref)?;
        let shepherd = pair.shepherd();
        let observatory = pair.observatory();
        let lineage = shepherd
            .lineage_ref()
            .ok_or(IntegratedSnapshotError::InvalidInput)?;
        if observatory.lineage_ref() != Some(lineage) {
            return Err(IntegratedSnapshotError::InvalidInput);
        }
        let shepherd = RedactedChildProjection {
            child_kind: shepherd.child_kind().to_owned(),
            lineage_ref: lineage.to_owned(),
            status: shepherd.status().to_owned(),
            authority_ref: shepherd.subject_ref().map(str::to_owned),
            generation: shepherd.foundation_generation().unwrap_or(0),
            fencing_generation: shepherd.fencing_generation(),
            committed_revision: shepherd.committed_revision(),
            envelope_generation: shepherd.envelope_generation(),
            payload_sha256: shepherd.payload_sha256().to_owned(),
            state_sha256: shepherd.state_sha256().to_owned(),
            receipt_sha256: shepherd.receipt_sha256().to_owned(),
            canonical_sha256: digest_bytes(
                &shepherd
                    .canonical_bytes()
                    .map_err(|_| IntegratedSnapshotError::Serialization)?,
            ),
            provenance_sha256: shepherd.provenance_sha256().to_owned(),
        };
        let observatory = RedactedChildProjection {
            child_kind: observatory.child_kind().to_owned(),
            lineage_ref: lineage.to_owned(),
            status: observatory.status().to_owned(),
            authority_ref: observatory.operation_ref().map(str::to_owned),
            generation: observatory.foundation_generation(),
            fencing_generation: observatory.fencing_generation(),
            committed_revision: observatory.committed_revision(),
            envelope_generation: observatory.envelope_generation(),
            payload_sha256: observatory.payload_sha256().to_owned(),
            state_sha256: observatory.state_sha256().to_owned(),
            receipt_sha256: observatory.receipt_sha256().to_owned(),
            canonical_sha256: digest_bytes(
                &observatory
                    .canonical_bytes()
                    .map_err(|_| IntegratedSnapshotError::Serialization)?,
            ),
            provenance_sha256: observatory.provenance_sha256().to_owned(),
        };
        validate_child(&shepherd, "shepherd")?;
        validate_child(&observatory, "observatory")?;
        self.append_observation(operation_ref, shepherd, observatory, outcome)
    }

    pub fn recover(
        &mut self,
        operation_ref: &str,
    ) -> Result<IntegratedSnapshotReceipt, IntegratedSnapshotError> {
        validate_identifier(operation_ref)?;
        if self.recovery_source_revision != Some(self.envelope.payload().revision) {
            return Err(IntegratedSnapshotError::InvalidInput);
        }
        let latest = latest_receipt(self.envelope.payload())?
            .ok_or(IntegratedSnapshotError::InvalidInput)?;
        self.append_observation(
            operation_ref,
            latest.shepherd,
            latest.observatory,
            IntegratedOutcome::Recovery,
        )
    }

    pub fn recoverable_latest_receipt(
        &self,
    ) -> Result<Option<IntegratedSnapshotReceipt>, IntegratedSnapshotError> {
        if self.recovery_source_revision != Some(self.envelope.payload().revision) {
            return Err(IntegratedSnapshotError::InvalidInput);
        }
        latest_receipt(self.envelope.payload())
    }

    fn append_observation(
        &mut self,
        operation_ref: &str,
        shepherd: RedactedChildProjection,
        observatory: RedactedChildProjection,
        outcome: IntegratedOutcome,
    ) -> Result<IntegratedSnapshotReceipt, IntegratedSnapshotError> {
        let input_sha256 = digest_jcs(&(DOMAIN, operation_ref, outcome, &shepherd, &observatory))?;
        if let Some(existing) = self.envelope.payload().operations.get(operation_ref) {
            return if existing.input_sha256 == input_sha256 {
                Ok(existing.clone())
            } else {
                Err(IntegratedSnapshotError::RetryConflict)
            };
        }
        if self.envelope.payload().operations.len() >= self.capacity {
            return Err(IntegratedSnapshotError::CapacityExceeded);
        }
        enforce_non_overlapping_transition(
            self.envelope.payload(),
            &shepherd,
            &observatory,
            outcome,
        )?;
        let prior_state_sha256 = state_digest(self.envelope.payload())?;
        let mut next = self.envelope.payload().clone();
        next.revision = next
            .revision
            .checked_add(1)
            .filter(|v| *v <= IJSON_MAX_INTEGER)
            .ok_or(IntegratedSnapshotError::Serialization)?;
        let mut receipt = IntegratedSnapshotReceipt {
            schema: SCHEMA.into(),
            operation_ref: operation_ref.into(),
            outcome,
            input_sha256,
            prior_state_sha256,
            result_state_sha256: String::new(),
            receipt_sha256: String::new(),
            shepherd,
            observatory,
        };
        next.operations
            .insert(operation_ref.into(), receipt.clone());
        receipt.result_state_sha256 = normalized_result_digest(&next, operation_ref)?;
        next.operations
            .insert(operation_ref.into(), receipt.clone());
        receipt.receipt_sha256 = normalized_receipt_digest(&receipt)?;
        next.operations
            .insert(operation_ref.into(), receipt.clone());
        validate_state(&next, self.capacity)?;
        self.envelope = self.store.commit(&self.envelope, next)?;
        self.recovery_source_revision = None;
        Ok(receipt)
    }

    pub fn receipt(&self, operation_ref: &str) -> Option<&IntegratedSnapshotReceipt> {
        self.envelope.payload().operations.get(operation_ref)
    }
}

fn validate_state(state: &State, capacity: usize) -> Result<(), IntegratedSnapshotError> {
    if state.revision > IJSON_MAX_INTEGER
        || state.operations.len() > capacity
        || state.revision != state.operations.len() as u64
    {
        return Err(IntegratedSnapshotError::CapacityExceeded);
    }
    let ordered = ordered_receipts(state)?;
    let mut prefix = State::default();
    for receipt in ordered {
        let operation = receipt.operation_ref.as_str();
        let prior_state_sha256 = state_digest(&prefix)?;
        if receipt.prior_state_sha256 != prior_state_sha256 {
            return Err(IntegratedSnapshotError::Serialization);
        }
        validate_receipt(operation, &receipt)?;
        let mut candidate = prefix.clone();
        candidate.revision = candidate
            .revision
            .checked_add(1)
            .filter(|value| *value <= IJSON_MAX_INTEGER)
            .ok_or(IntegratedSnapshotError::Serialization)?;
        candidate
            .operations
            .insert(operation.to_owned(), receipt.clone());
        if normalized_result_digest(&candidate, operation)? != receipt.result_state_sha256 {
            return Err(IntegratedSnapshotError::Serialization);
        }
        prefix = candidate;
    }
    if &prefix != state {
        return Err(IntegratedSnapshotError::Serialization);
    }
    Ok(())
}

fn validate_receipt(
    operation: &str,
    receipt: &IntegratedSnapshotReceipt,
) -> Result<(), IntegratedSnapshotError> {
    if operation != receipt.operation_ref
        || receipt.schema != SCHEMA
        || normalized_receipt_digest(receipt)? != receipt.receipt_sha256
    {
        return Err(IntegratedSnapshotError::Serialization);
    }
    validate_child(&receipt.shepherd, "shepherd")?;
    validate_child(&receipt.observatory, "observatory")?;
    if receipt.shepherd.lineage_ref != receipt.observatory.lineage_ref {
        return Err(IntegratedSnapshotError::Serialization);
    }
    Ok(())
}

fn latest_receipt(
    state: &State,
) -> Result<Option<IntegratedSnapshotReceipt>, IntegratedSnapshotError> {
    Ok(ordered_receipts(state)?.pop())
}

fn ordered_receipts(
    state: &State,
) -> Result<Vec<IntegratedSnapshotReceipt>, IntegratedSnapshotError> {
    if state.revision != state.operations.len() as u64 {
        return Err(IntegratedSnapshotError::Serialization);
    }
    let mut ordered = Vec::new();
    let mut remaining: BTreeMap<&str, &IntegratedSnapshotReceipt> = state
        .operations
        .iter()
        .map(|(operation, receipt)| (operation.as_str(), receipt))
        .collect();
    while !remaining.is_empty() {
        let prior_state_sha256 = state_prefix_digest(ordered.len() as u64, &ordered)?;
        let mut matches = remaining
            .iter()
            .filter(|(_, receipt)| receipt.prior_state_sha256 == prior_state_sha256);
        let (operation, receipt) = matches
            .next()
            .map(|(operation, receipt)| (*operation, (**receipt).clone()))
            .ok_or(IntegratedSnapshotError::Serialization)?;
        if matches.next().is_some() {
            return Err(IntegratedSnapshotError::Serialization);
        }
        ordered.push(receipt);
        remaining.remove(operation);
    }
    Ok(ordered)
}

fn enforce_non_overlapping_transition(
    state: &State,
    shepherd: &RedactedChildProjection,
    observatory: &RedactedChildProjection,
    outcome: IntegratedOutcome,
) -> Result<(), IntegratedSnapshotError> {
    let Some(previous) = latest_receipt(state)? else {
        return Ok(());
    };
    if previous.shepherd.lineage_ref != shepherd.lineage_ref
        || previous.observatory.lineage_ref != observatory.lineage_ref
    {
        return Err(IntegratedSnapshotError::InvalidInput);
    }
    if outcome == IntegratedOutcome::Recovery {
        return if previous.shepherd == *shepherd && previous.observatory == *observatory {
            Ok(())
        } else {
            Err(IntegratedSnapshotError::InvalidInput)
        };
    }
    if !strictly_newer_child(&previous.shepherd, shepherd)
        || !strictly_newer_child(&previous.observatory, observatory)
    {
        return Err(IntegratedSnapshotError::InvalidInput);
    }
    Ok(())
}

fn strictly_newer_child(
    previous: &RedactedChildProjection,
    next: &RedactedChildProjection,
) -> bool {
    previous.child_kind == next.child_kind
        && previous.lineage_ref == next.lineage_ref
        && next.generation > previous.generation
        && next.fencing_generation > previous.fencing_generation
}

fn validate_child(
    child: &RedactedChildProjection,
    kind: &str,
) -> Result<(), IntegratedSnapshotError> {
    if child.child_kind != kind
        || !is_sha256(&child.lineage_ref)
        || child.committed_revision == 0
        || [
            child.generation,
            child.fencing_generation,
            child.committed_revision,
            child.envelope_generation,
        ]
        .into_iter()
        .any(|value| value > IJSON_MAX_INTEGER)
        || [
            &child.payload_sha256,
            &child.state_sha256,
            &child.receipt_sha256,
            &child.canonical_sha256,
            &child.provenance_sha256,
        ]
        .into_iter()
        .any(|value| !is_sha256(value))
    {
        return Err(IntegratedSnapshotError::InvalidInput);
    }
    Ok(())
}

fn normalized_result_digest(
    state: &State,
    operation: &str,
) -> Result<String, IntegratedSnapshotError> {
    let mut normalized = state.clone();
    let receipt = normalized
        .operations
        .get_mut(operation)
        .ok_or(IntegratedSnapshotError::Serialization)?;
    receipt.result_state_sha256.clear();
    receipt.receipt_sha256.clear();
    state_digest(&normalized)
}

fn normalized_receipt_digest(
    receipt: &IntegratedSnapshotReceipt,
) -> Result<String, IntegratedSnapshotError> {
    let mut normalized = receipt.clone();
    normalized.receipt_sha256.clear();
    digest_jcs(&normalized)
}

fn state_digest(state: &State) -> Result<String, IntegratedSnapshotError> {
    let receipts = ordered_receipts(state)?;
    state_prefix_digest(state.revision, &receipts)
}

fn state_prefix_digest(
    revision: u64,
    receipts: &[IntegratedSnapshotReceipt],
) -> Result<String, IntegratedSnapshotError> {
    digest_jcs(&StatePrefix {
        domain: STATE_PREFIX_DOMAIN,
        revision,
        receipts,
    })
}
fn digest_jcs(value: &impl Serialize) -> Result<String, IntegratedSnapshotError> {
    serde_jcs::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|_| IntegratedSnapshotError::Serialization)
}
fn digest_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn validate_identifier(value: &str) -> Result<(), IntegratedSnapshotError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b':' | b'-'))
    {
        return Err(IntegratedSnapshotError::InvalidInput);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalized_receipt_rejects_tamper() {
        let child = RedactedChildProjection {
            child_kind: "shepherd".into(),
            lineage_ref: "11".repeat(32),
            status: "eligible".into(),
            authority_ref: Some("s".into()),
            generation: 1,
            fencing_generation: 1,
            committed_revision: 1,
            envelope_generation: 1,
            payload_sha256: "22".repeat(32),
            state_sha256: "33".repeat(32),
            receipt_sha256: "44".repeat(32),
            canonical_sha256: "55".repeat(32),
            provenance_sha256: "66".repeat(32),
        };
        let mut receipt = IntegratedSnapshotReceipt {
            schema: SCHEMA.into(),
            operation_ref: "op".into(),
            outcome: IntegratedOutcome::Success,
            input_sha256: "77".repeat(32),
            prior_state_sha256: "88".repeat(32),
            result_state_sha256: "99".repeat(32),
            receipt_sha256: String::new(),
            shepherd: child.clone(),
            observatory: RedactedChildProjection {
                child_kind: "observatory".into(),
                ..child
            },
        };
        receipt.receipt_sha256 = normalized_receipt_digest(&receipt).unwrap();
        assert_eq!(
            normalized_receipt_digest(&receipt).unwrap(),
            receipt.receipt_sha256
        );
        receipt.outcome = IntegratedOutcome::Rejection;
        assert_ne!(
            normalized_receipt_digest(&receipt).unwrap(),
            receipt.receipt_sha256
        );
    }
}
