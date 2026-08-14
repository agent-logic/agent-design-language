//! Deterministic single-Shepherd serving-eligibility authority.

use super::{
    observatory_serving_eligibility::SealedObservatoryCommittedProjection,
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
    serving_authority::VerifiedServingAuthorityCut,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path, sync::Arc};

const MAX_OPERATIONS: usize = 4096;
const IJSON_MAX_INTEGER: u64 = 9_007_199_254_740_991;
const SEALED_SHEPHERD_DOMAIN: &str = "ADL-SEALED-SHEPHERD-COMMITTED-PROJECTION-V1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShepherdEligibilityError {
    InvalidInput,
    StaleAuthority,
    RetryConflict,
    CapacityExceeded,
    Storage,
    Serialization,
}
impl From<PolisRuntimeError> for ShepherdEligibilityError {
    fn from(_: PolisRuntimeError) -> Self {
        Self::Storage
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Vacant,
    Eligible,
    Revoked,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Grant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lineage_ref: Option<String>,
    subject_ref: String,
    permit_sha256: String,
    owner_commit_sha256: String,
    lease_sha256: String,
    foundation_state_sha256: String,
    foundation_result_sha256: String,
    foundation_generation: u64,
    foundation_receipt_sha256: String,
    fencing_generation: u64,
    expires_at: u64,
    status: Status,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    operation_id: String,
    input_sha256: String,
    prior_state_sha256: String,
    candidate_state_sha256: String,
    transition: String,
    result: StoredProjection,
    owner_commit_sha256: String,
    lease_sha256: String,
    foundation_receipt_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredProjection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lineage_ref: Option<String>,
    status: String,
    subject_ref: Option<String>,
    fencing_generation: u64,
    expiry_class: String,
    foundation_generation: Option<u64>,
    foundation_state_sha256: Option<String>,
    foundation_result_sha256: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    revision: u64,
    fence_floor: u64,
    current: Option<Grant>,
    operations: BTreeMap<String, Receipt>,
}
impl CheckpointMetadataSource for State {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        let bytes = serde_jcs::to_vec(self).map_err(|_| PolisRuntimeError::Serialization)?;
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.revision),
            state_sha256: Some(hex::encode(Sha256::digest(bytes))),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShepherdEligibilityProjection {
    pub schema: String,
    pub status: String,
    pub subject_ref: Option<String>,
    #[serde(default)]
    pub lineage_ref: Option<String>,
    pub fencing_generation: u64,
    pub expiry_class: String,
    pub foundation_state_sha256: Option<String>,
    pub foundation_generation: Option<u64>,
    pub foundation_result_sha256: Option<String>,
    pub state_sha256: String,
    pub receipt_sha256: String,
}

/// Store-derived committed Shepherd eligibility evidence.
///
/// Its fields and constructor are private. Caller-created projections cannot be
/// promoted into committed evidence.
///
/// ```compile_fail
/// use adl_runtime::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection;
/// let _ = SealedShepherdCommittedProjection { provenance_sha256: String::new() };
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedShepherdCommittedProjection {
    preimage: ShepherdCommittedPreimage,
    provenance_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ShepherdCommittedPreimage {
    domain: String,
    child_kind: String,
    envelope_generation: u64,
    committed_revision: u64,
    payload_sha256: String,
    receipt_sha256: String,
    state_sha256: String,
    projection: ShepherdEligibilityProjection,
}

impl SealedShepherdCommittedProjection {
    pub fn child_kind(&self) -> &str {
        &self.preimage.child_kind
    }
    pub fn envelope_generation(&self) -> u64 {
        self.preimage.envelope_generation
    }
    pub fn committed_revision(&self) -> u64 {
        self.preimage.committed_revision
    }
    pub fn payload_sha256(&self) -> &str {
        &self.preimage.payload_sha256
    }
    pub fn receipt_sha256(&self) -> &str {
        &self.preimage.receipt_sha256
    }
    pub fn state_sha256(&self) -> &str {
        &self.preimage.state_sha256
    }
    pub fn provenance_sha256(&self) -> &str {
        &self.provenance_sha256
    }
    pub fn status(&self) -> &str {
        &self.preimage.projection.status
    }
    pub fn subject_ref(&self) -> Option<&str> {
        self.preimage.projection.subject_ref.as_deref()
    }
    pub fn lineage_ref(&self) -> Option<&str> {
        self.preimage.projection.lineage_ref.as_deref()
    }
    pub fn fencing_generation(&self) -> u64 {
        self.preimage.projection.fencing_generation
    }
    pub fn expiry_class(&self) -> &str {
        &self.preimage.projection.expiry_class
    }
    pub fn foundation_state_sha256(&self) -> Option<&str> {
        self.preimage.projection.foundation_state_sha256.as_deref()
    }
    pub fn foundation_generation(&self) -> Option<u64> {
        self.preimage.projection.foundation_generation
    }
    pub fn foundation_result_sha256(&self) -> Option<&str> {
        self.preimage.projection.foundation_result_sha256.as_deref()
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ShepherdEligibilityError> {
        serde_jcs::to_vec(self).map_err(|_| ShepherdEligibilityError::Serialization)
    }
}

/// A verifier-returned pair of exact store-derived child projections.
///
/// Its fields are private and it cannot be deserialized or constructed by callers.
///
/// ```compile_fail
/// use adl_runtime::distributed::shepherd_serving_eligibility::VerifiedCommittedChildLineagePair;
/// let _ = VerifiedCommittedChildLineagePair { shepherd: todo!(), observatory: todo!() };
/// ```
pub struct VerifiedCommittedChildLineagePair<'a> {
    shepherd: &'a SealedShepherdCommittedProjection,
    observatory: &'a SealedObservatoryCommittedProjection,
}

impl<'a> VerifiedCommittedChildLineagePair<'a> {
    pub fn shepherd(&self) -> &'a SealedShepherdCommittedProjection {
        self.shepherd
    }

    pub fn observatory(&self) -> &'a SealedObservatoryCommittedProjection {
        self.observatory
    }
}

/// Verifies that two store-derived child projections belong to one authority lineage.
///
/// Public projection DTOs and raw lineage strings cannot enter this boundary.
pub fn verify_committed_child_lineage_pair<'a>(
    shepherd: &'a SealedShepherdCommittedProjection,
    observatory: &'a SealedObservatoryCommittedProjection,
) -> Result<VerifiedCommittedChildLineagePair<'a>, ShepherdEligibilityError> {
    if shepherd.child_kind() != "shepherd"
        || observatory.child_kind() != "observatory"
        || shepherd
            .lineage_ref()
            .filter(|value| is_sha256(value))
            .is_none()
        || observatory
            .lineage_ref()
            .filter(|value| is_sha256(value))
            .is_none()
        || shepherd.lineage_ref() != observatory.lineage_ref()
    {
        return Err(ShepherdEligibilityError::StaleAuthority);
    }
    Ok(VerifiedCommittedChildLineagePair {
        shepherd,
        observatory,
    })
}

pub struct ShepherdEligibilityStore {
    store: CheckpointedJson<State>,
    envelope: DurableEnvelope<State>,
    capacity: usize,
}
impl ShepherdEligibilityStore {
    pub fn open(
        root: &Path,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> Result<Self, ShepherdEligibilityError> {
        if capacity == 0 || capacity > MAX_OPERATIONS {
            return Err(ShepherdEligibilityError::CapacityExceeded);
        }
        let (store, envelope) = CheckpointedJson::open(
            root,
            "shepherd-serving-eligibility",
            "shepherd-serving-eligibility.json",
            State::default(),
            authority,
        )?;
        if envelope.payload().operations.len() > capacity {
            return Err(ShepherdEligibilityError::CapacityExceeded);
        }
        Ok(Self {
            store,
            envelope,
            capacity,
        })
    }

    pub fn committed_projection(
        &self,
    ) -> Result<Option<SealedShepherdCommittedProjection>, ShepherdEligibilityError> {
        let state = self.envelope.payload();
        if state.current.is_none() {
            return Ok(None);
        }
        let current_projection = stored_projection(state);
        let mut matches = state.operations.values().filter(|receipt| {
            receipt.result == current_projection
                && normalized_state_digest_for_receipt(state, &receipt.operation_id)
                    .is_ok_and(|digest| digest == receipt.candidate_state_sha256)
        });
        let receipt = matches
            .next()
            .ok_or(ShepherdEligibilityError::Serialization)?;
        if matches.next().is_some() {
            return Err(ShepherdEligibilityError::Serialization);
        }
        let projection = projection_from_receipt(receipt)?;
        let payload_sha256 = state_digest(state)?;
        if payload_sha256 != self.envelope.payload_sha256() {
            return Err(ShepherdEligibilityError::Serialization);
        }
        seal_shepherd_projection(ShepherdCommittedPreimage {
            domain: SEALED_SHEPHERD_DOMAIN.into(),
            child_kind: "shepherd".into(),
            envelope_generation: self.envelope.generation(),
            committed_revision: state.revision,
            payload_sha256,
            receipt_sha256: projection.receipt_sha256.clone(),
            state_sha256: projection.state_sha256.clone(),
            projection,
        })
        .map(Some)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn acquire(
        &mut self,
        operation_id: &str,
        subject: &str,
        permit: &[u8],
        cut: &VerifiedServingAuthorityCut,
        expires_at: u64,
        logical_now: u64,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        if !self
            .envelope
            .payload()
            .operations
            .contains_key(operation_id)
            && self
                .envelope
                .payload()
                .current
                .as_ref()
                .is_some_and(|g| g.status == Status::Eligible && logical_now < g.expires_at)
        {
            return Err(ShepherdEligibilityError::StaleAuthority);
        }
        self.transition(
            operation_id,
            subject,
            permit,
            cut,
            expires_at,
            logical_now,
            "acquire",
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn replace(
        &mut self,
        operation_id: &str,
        subject: &str,
        permit: &[u8],
        cut: &VerifiedServingAuthorityCut,
        expires_at: u64,
        logical_now: u64,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        if let Some(old) = self.envelope.payload().operations.get(operation_id) {
            let input = grant_input_sha256(subject, permit, cut, expires_at)?;
            if old.transition != "replace" || old.input_sha256 != input {
                return Err(ShepherdEligibilityError::RetryConflict);
            }
            return projection_from_receipt(old);
        }
        if self
            .envelope
            .payload()
            .current
            .as_ref()
            .is_none_or(|g| g.status != Status::Eligible)
        {
            return Err(ShepherdEligibilityError::StaleAuthority);
        }
        self.transition(
            operation_id,
            subject,
            permit,
            cut,
            expires_at,
            logical_now,
            "replace",
        )
    }

    pub fn revoke(
        &mut self,
        operation_id: &str,
        cut: &VerifiedServingAuthorityCut,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        self.end(operation_id, cut, Status::Revoked, 0)
    }
    pub fn expire(
        &mut self,
        operation_id: &str,
        cut: &VerifiedServingAuthorityCut,
        logical_now: u64,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        self.end(operation_id, cut, Status::Expired, logical_now)
    }

    #[allow(clippy::too_many_arguments)]
    fn transition(
        &mut self,
        operation_id: &str,
        subject: &str,
        permit: &[u8],
        cut: &VerifiedServingAuthorityCut,
        expires_at: u64,
        logical_now: u64,
        transition: &str,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        validate_id(operation_id)?;
        validate_id(subject)?;
        if permit.is_empty()
            || expires_at <= logical_now
            || (!self
                .envelope
                .payload()
                .operations
                .contains_key(operation_id)
                && cut.fencing_generation() <= self.envelope.payload().fence_floor)
        {
            return Err(ShepherdEligibilityError::StaleAuthority);
        }
        let grant = Grant {
            lineage_ref: Some(cut.lineage_ref()),
            subject_ref: keyed("subject", subject),
            permit_sha256: hex::encode(Sha256::digest(permit)),
            owner_commit_sha256: keyed("owner", cut.owner_commit_id()),
            lease_sha256: keyed("lease", cut.lease_id()),
            foundation_state_sha256: cut.state_sha256().into(),
            foundation_result_sha256: cut.result_sha256().into(),
            foundation_generation: cut.generation(),
            foundation_receipt_sha256: cut.receipt_digest().into(),
            fencing_generation: cut.fencing_generation(),
            expires_at,
            status: Status::Eligible,
        };
        self.commit(operation_id, grant, transition)
    }
    fn end(
        &mut self,
        operation_id: &str,
        cut: &VerifiedServingAuthorityCut,
        status: Status,
        now: u64,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        validate_id(operation_id)?;
        let lineage_ref = cut.lineage_ref();
        if let Some(old) = self.envelope.payload().operations.get(operation_id) {
            if old.transition
                != if status == Status::Expired {
                    "expire"
                } else {
                    "revoke"
                }
                || old.result.fencing_generation != cut.fencing_generation()
                || old.result.foundation_generation != Some(cut.generation())
                || old.result.foundation_state_sha256.as_deref() != Some(cut.state_sha256())
                || old.result.foundation_result_sha256.as_deref() != Some(cut.result_sha256())
                || old.result.lineage_ref.as_deref() != Some(lineage_ref.as_str())
                || old.owner_commit_sha256 != keyed("owner", cut.owner_commit_id())
                || old.lease_sha256 != keyed("lease", cut.lease_id())
                || old.foundation_receipt_sha256 != cut.receipt_digest()
            {
                return Err(ShepherdEligibilityError::RetryConflict);
            }
            return projection_from_receipt(old);
        }
        let mut grant = self
            .envelope
            .payload()
            .current
            .clone()
            .ok_or(ShepherdEligibilityError::StaleAuthority)?;
        if grant.fencing_generation != cut.fencing_generation()
            || grant.foundation_state_sha256 != cut.state_sha256()
            || grant.foundation_result_sha256 != cut.result_sha256()
            || grant.foundation_generation != cut.generation()
            || grant.foundation_receipt_sha256 != cut.receipt_digest()
            || grant.lineage_ref.as_deref() != Some(lineage_ref.as_str())
            || grant.owner_commit_sha256 != keyed("owner", cut.owner_commit_id())
            || grant.lease_sha256 != keyed("lease", cut.lease_id())
            || (status == Status::Expired && now < grant.expires_at)
        {
            return Err(ShepherdEligibilityError::StaleAuthority);
        }
        grant.status = status;
        let transition = if status == Status::Expired {
            "expire"
        } else {
            "revoke"
        };
        self.commit(operation_id, grant, transition)
    }
    fn commit(
        &mut self,
        operation_id: &str,
        grant: Grant,
        transition: &str,
    ) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
        let prior = state_digest(self.envelope.payload())?;
        let input = hex::encode(Sha256::digest(
            serde_jcs::to_vec(&grant).map_err(|_| ShepherdEligibilityError::Serialization)?,
        ));
        if let Some(old) = self.envelope.payload().operations.get(operation_id) {
            if old.input_sha256 != input || old.transition != transition {
                return Err(ShepherdEligibilityError::RetryConflict);
            }
            return projection_from_receipt(old);
        }
        if self.envelope.payload().operations.len() >= self.capacity {
            return Err(ShepherdEligibilityError::CapacityExceeded);
        }
        let mut next = self.envelope.payload().clone();
        next.revision += 1;
        next.fence_floor = next.fence_floor.max(grant.fencing_generation);
        next.current = Some(grant.clone());
        let candidate = state_digest(&next)?;
        let stored = stored_projection(&next);
        let receipt = Receipt {
            operation_id: operation_id.into(),
            input_sha256: input,
            prior_state_sha256: prior,
            candidate_state_sha256: candidate,
            transition: transition.into(),
            result: stored,
            owner_commit_sha256: grant.owner_commit_sha256.clone(),
            lease_sha256: grant.lease_sha256.clone(),
            foundation_receipt_sha256: grant.foundation_receipt_sha256.clone(),
        };
        next.operations.insert(operation_id.into(), receipt.clone());
        self.envelope = self.store.commit(&self.envelope, next)?;
        projection(self.envelope.payload(), &receipt)
    }
}

fn seal_shepherd_projection(
    preimage: ShepherdCommittedPreimage,
) -> Result<SealedShepherdCommittedProjection, ShepherdEligibilityError> {
    validate_shepherd_preimage(&preimage)?;
    let provenance_sha256 = hex::encode(Sha256::digest(
        serde_jcs::to_vec(&preimage).map_err(|_| ShepherdEligibilityError::Serialization)?,
    ));
    Ok(SealedShepherdCommittedProjection {
        preimage,
        provenance_sha256,
    })
}

#[cfg(test)]
fn verify_sealed_shepherd_projection(
    sealed: &SealedShepherdCommittedProjection,
) -> Result<(), ShepherdEligibilityError> {
    validate_shepherd_preimage(&sealed.preimage)?;
    let expected = hex::encode(Sha256::digest(
        serde_jcs::to_vec(&sealed.preimage).map_err(|_| ShepherdEligibilityError::Serialization)?,
    ));
    if expected != sealed.provenance_sha256 {
        return Err(ShepherdEligibilityError::Serialization);
    }
    Ok(())
}

fn validate_shepherd_preimage(
    p: &ShepherdCommittedPreimage,
) -> Result<(), ShepherdEligibilityError> {
    if p.domain != SEALED_SHEPHERD_DOMAIN
        || p.child_kind != "shepherd"
        || p.envelope_generation > IJSON_MAX_INTEGER
        || p.committed_revision == 0
        || p.committed_revision > IJSON_MAX_INTEGER
        || p.projection.schema != "adl.shepherd-serving-eligibility.projection.v1"
        || p.receipt_sha256 != p.projection.receipt_sha256
        || p.state_sha256 != p.projection.state_sha256
        || p.projection
            .lineage_ref
            .as_deref()
            .is_none_or(|value| !is_sha256(value))
        || !is_sha256(&p.payload_sha256)
        || !is_sha256(&p.receipt_sha256)
        || !is_sha256(&p.state_sha256)
    {
        return Err(ShepherdEligibilityError::Serialization);
    }
    Ok(())
}

fn normalized_state_digest_for_receipt(
    state: &State,
    operation_id: &str,
) -> Result<String, ShepherdEligibilityError> {
    let mut normalized = state.clone();
    if normalized.operations.remove(operation_id).is_none() {
        return Err(ShepherdEligibilityError::Serialization);
    }
    state_digest(&normalized)
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn projection(
    state: &State,
    receipt: &Receipt,
) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
    let _ = state;
    projection_from_receipt(receipt)
}
fn stored_projection(state: &State) -> StoredProjection {
    let grant = state.current.as_ref();
    StoredProjection {
        lineage_ref: grant.and_then(|g| g.lineage_ref.clone()),
        status: grant
            .map_or("vacant", |g| match g.status {
                Status::Eligible => "eligible",
                Status::Revoked => "revoked",
                Status::Expired => "expired",
                Status::Vacant => "vacant",
            })
            .into(),
        subject_ref: grant.map(|g| g.subject_ref.clone()),
        fencing_generation: state.fence_floor,
        expiry_class: grant
            .map_or("none", |g| {
                if g.status == Status::Eligible {
                    "bounded"
                } else {
                    "ended"
                }
            })
            .into(),
        foundation_state_sha256: grant.map(|g| g.foundation_state_sha256.clone()),
        foundation_generation: grant.map(|g| g.foundation_generation),
        foundation_result_sha256: grant.map(|g| g.foundation_result_sha256.clone()),
    }
}
fn projection_from_receipt(
    receipt: &Receipt,
) -> Result<ShepherdEligibilityProjection, ShepherdEligibilityError> {
    let result = &receipt.result;
    Ok(ShepherdEligibilityProjection {
        schema: "adl.shepherd-serving-eligibility.projection.v1".into(),
        status: result.status.clone(),
        subject_ref: result.subject_ref.clone(),
        lineage_ref: result.lineage_ref.clone(),
        fencing_generation: result.fencing_generation,
        expiry_class: result.expiry_class.clone(),
        foundation_state_sha256: result.foundation_state_sha256.clone(),
        foundation_generation: result.foundation_generation,
        foundation_result_sha256: result.foundation_result_sha256.clone(),
        state_sha256: receipt.candidate_state_sha256.clone(),
        receipt_sha256: hex::encode(Sha256::digest(
            serde_jcs::to_vec(receipt).map_err(|_| ShepherdEligibilityError::Serialization)?,
        )),
    })
}
fn state_digest(state: &State) -> Result<String, ShepherdEligibilityError> {
    serde_jcs::to_vec(state)
        .map(|b| hex::encode(Sha256::digest(b)))
        .map_err(|_| ShepherdEligibilityError::Serialization)
}
fn keyed(domain: &str, value: &str) -> String {
    hex::encode(Sha256::digest(format!(
        "ADL-SHEPHERD-ELIGIBILITY-REF-V1\0{domain}\0{value}"
    )))
}
fn grant_input_sha256(
    subject: &str,
    permit: &[u8],
    cut: &VerifiedServingAuthorityCut,
    expires_at: u64,
) -> Result<String, ShepherdEligibilityError> {
    validate_id(subject)?;
    let grant = Grant {
        lineage_ref: Some(cut.lineage_ref()),
        subject_ref: keyed("subject", subject),
        permit_sha256: hex::encode(Sha256::digest(permit)),
        owner_commit_sha256: keyed("owner", cut.owner_commit_id()),
        lease_sha256: keyed("lease", cut.lease_id()),
        foundation_state_sha256: cut.state_sha256().into(),
        foundation_result_sha256: cut.result_sha256().into(),
        foundation_generation: cut.generation(),
        foundation_receipt_sha256: cut.receipt_digest().into(),
        fencing_generation: cut.fencing_generation(),
        expires_at,
        status: Status::Eligible,
    };
    serde_jcs::to_vec(&grant)
        .map(|bytes| hex::encode(Sha256::digest(bytes)))
        .map_err(|_| ShepherdEligibilityError::Serialization)
}
fn validate_id(v: &str) -> Result<(), ShepherdEligibilityError> {
    if v.is_empty()
        || v.len() > 128
        || !v
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
    {
        Err(ShepherdEligibilityError::InvalidInput)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_preimage() -> ShepherdCommittedPreimage {
        ShepherdCommittedPreimage {
            domain: SEALED_SHEPHERD_DOMAIN.into(),
            child_kind: "shepherd".into(),
            envelope_generation: 2,
            committed_revision: 1,
            payload_sha256: "11".repeat(32),
            receipt_sha256: "22".repeat(32),
            state_sha256: "33".repeat(32),
            projection: ShepherdEligibilityProjection {
                lineage_ref: Some("99".repeat(32)),
                schema: "adl.shepherd-serving-eligibility.projection.v1".into(),
                status: "eligible".into(),
                subject_ref: Some("44".repeat(32)),
                fencing_generation: 7,
                expiry_class: "bounded".into(),
                foundation_state_sha256: Some("55".repeat(32)),
                foundation_generation: Some(3),
                foundation_result_sha256: Some("66".repeat(32)),
                state_sha256: "33".repeat(32),
                receipt_sha256: "22".repeat(32),
            },
        }
    }

    #[test]
    fn sealed_committed_projection_private_provenance() {
        let sealed = seal_shepherd_projection(valid_preimage()).unwrap();
        verify_sealed_shepherd_projection(&sealed).unwrap();
        let bytes = serde_jcs::to_vec(&sealed.preimage).unwrap();
        let reopened: ShepherdCommittedPreimage = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(reopened, sealed.preimage);

        for mutation in 0..5 {
            let mut changed = sealed.clone();
            match mutation {
                0 => changed.preimage.child_kind = "observatory".into(),
                1 => changed.preimage.envelope_generation += 1,
                2 => changed.preimage.committed_revision += 1,
                3 => changed.preimage.receipt_sha256 = "77".repeat(32),
                _ => changed.preimage.projection.subject_ref = Some("88".repeat(32)),
            }
            assert_eq!(
                verify_sealed_shepherd_projection(&changed),
                Err(ShepherdEligibilityError::Serialization)
            );
        }
        let mut unknown = serde_json::to_value(&sealed.preimage).unwrap();
        unknown
            .as_object_mut()
            .unwrap()
            .insert("unknown".into(), true.into());
        assert!(serde_json::from_value::<ShepherdCommittedPreimage>(unknown).is_err());

        let mut missing = valid_preimage();
        missing.projection.lineage_ref = None;
        assert_eq!(
            seal_shepherd_projection(missing),
            Err(ShepherdEligibilityError::Serialization)
        );

        let legacy_json = serde_json::json!({
            "subject_ref": "44".repeat(32),
            "permit_sha256": "55".repeat(32),
            "owner_commit_sha256": "66".repeat(32),
            "lease_sha256": "77".repeat(32),
            "foundation_state_sha256": "88".repeat(32),
            "foundation_result_sha256": "99".repeat(32),
            "foundation_generation": 3,
            "foundation_receipt_sha256": "aa".repeat(32),
            "fencing_generation": 7,
            "expires_at": 10,
            "status": "eligible"
        });
        let legacy: Grant = serde_json::from_value(legacy_json).unwrap();
        assert_eq!(legacy.lineage_ref, None);
    }
}
