//! Deterministic single-Shepherd serving-eligibility authority.

use super::{
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ShepherdEligibilityProjection {
    pub schema: String,
    pub status: String,
    pub subject_ref: Option<String>,
    pub fencing_generation: u64,
    pub expiry_class: String,
    pub foundation_state_sha256: Option<String>,
    pub foundation_generation: Option<u64>,
    pub foundation_result_sha256: Option<String>,
    pub state_sha256: String,
    pub receipt_sha256: String,
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
