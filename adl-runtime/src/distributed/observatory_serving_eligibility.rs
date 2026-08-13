//! Durable Observatory eligibility driven only by a sealed authority projection.
use super::{
    authority_protocol::PublishedAuthorityResult,
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
    serving_authority::{
        verify_observatory_authority_projection, ObservatoryTransitionAction,
        VerifiedObservatoryAuthorityProjection, VerifiedServingAuthorityCut,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path, sync::Arc};
const MAX: usize = 4096;
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservatoryEligibilityError {
    InvalidAuthority,
    StaleAuthority,
    RetryConflict,
    CapacityExceeded,
    Storage,
    Serialization,
}
impl From<PolisRuntimeError> for ObservatoryEligibilityError {
    fn from(_: PolisRuntimeError) -> Self {
        Self::Storage
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Eligible,
    Revoked,
    Expired,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Grant {
    trust: String,
    polis: String,
    lineage: String,
    operation: String,
    index: u64,
    generation: u64,
    fence: u64,
    result: String,
    signers: String,
    signer_count: usize,
    deadline_seconds: i64,
    deadline_nanos: u32,
    deadline_uncertainty: u64,
    final_seconds: i64,
    final_nanos: u32,
    final_uncertainty: u64,
    status: Status,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    operation: String,
    input: String,
    prior: String,
    result_state: String,
    action: ObservatoryTransitionAction,
    predecessor: Option<String>,
    projection: StoredProjection,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredProjection {
    status: String,
    trust: Option<String>,
    polis: Option<String>,
    lineage: Option<String>,
    operation: Option<String>,
    index: u64,
    generation: u64,
    fence: u64,
    result: Option<String>,
    signers: Option<String>,
    signer_count: usize,
}
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    revision: u64,
    current: Option<Grant>,
    operations: BTreeMap<String, Receipt>,
}
#[derive(Clone, Debug)]
struct TransitionCommand {
    grant: Grant,
    action: ObservatoryTransitionAction,
    predecessor: Option<String>,
}
impl TransitionCommand {
    fn verified(p: &VerifiedObservatoryAuthorityProjection) -> Self {
        Self {
            grant: grant(p, Status::Eligible),
            action: p.transition_action(),
            predecessor: p.predecessor_operation_ref().map(str::to_owned),
        }
    }
}
impl CheckpointMetadataSource for State {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        let b = serde_jcs::to_vec(self).map_err(|_| PolisRuntimeError::Serialization)?;
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.revision),
            state_sha256: Some(hex::encode(Sha256::digest(b))),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ObservatoryEligibilityProjection {
    pub schema: String,
    pub status: String,
    pub trust_domain_ref: Option<String>,
    pub polis_ref: Option<String>,
    pub lineage_ref: Option<String>,
    pub operation_ref: Option<String>,
    pub committed_log_index: u64,
    pub foundation_generation: u64,
    pub fencing_generation: u64,
    pub authority_result_sha256: Option<String>,
    pub signer_set_sha256: Option<String>,
    pub signer_count: usize,
    pub state_sha256: String,
    pub receipt_sha256: String,
}
pub struct ObservatoryEligibilityStore {
    store: CheckpointedJson<State>,
    envelope: DurableEnvelope<State>,
    capacity: usize,
}
impl ObservatoryEligibilityStore {
    pub fn open(
        root: &Path,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> Result<Self, ObservatoryEligibilityError> {
        if capacity == 0 || capacity > MAX {
            return Err(ObservatoryEligibilityError::CapacityExceeded);
        }
        let (store, envelope) = CheckpointedJson::open(
            root,
            "observatory-serving-eligibility",
            "observatory-serving-eligibility.json",
            State::default(),
            authority,
        )?;
        if envelope.payload().operations.len() > capacity {
            return Err(ObservatoryEligibilityError::CapacityExceeded);
        }
        Ok(Self {
            store,
            envelope,
            capacity,
        })
    }
    pub fn apply(
        &mut self,
        authority: &PublishedAuthorityResult,
        cut: &VerifiedServingAuthorityCut,
        seconds: i64,
        nanos: u32,
    ) -> Result<ObservatoryEligibilityProjection, ObservatoryEligibilityError> {
        let p = verify_observatory_authority_projection(authority, cut)
            .map_err(|_| ObservatoryEligibilityError::InvalidAuthority)?;
        let op = p.authority_result_sha256().to_owned();
        let input = input_digest(&p, seconds, nanos)?;
        if let Some(old) = self.envelope.payload().operations.get(&op) {
            if old.input != input {
                return Err(ObservatoryEligibilityError::RetryConflict);
            }
            return output(old);
        }
        if self.envelope.payload().operations.len() >= self.capacity {
            return Err(ObservatoryEligibilityError::CapacityExceeded);
        }
        let expired = p
            .is_expired_at(seconds, nanos)
            .map_err(|_| ObservatoryEligibilityError::InvalidAuthority)?;
        self.commit(TransitionCommand::verified(&p), input, expired)
    }
    fn commit(
        &mut self,
        command: TransitionCommand,
        input: String,
        expired: bool,
    ) -> Result<ObservatoryEligibilityProjection, ObservatoryEligibilityError> {
        validate_transition(self.envelope.payload().current.as_ref(), &command, expired)?;
        let status = if command.action == ObservatoryTransitionAction::Revoke {
            Status::Revoked
        } else if expired {
            Status::Expired
        } else {
            Status::Eligible
        };
        let prior = digest(self.envelope.payload())?;
        let mut next = self.envelope.payload().clone();
        next.revision += 1;
        let mut resulting_grant = command.grant.clone();
        resulting_grant.status = status;
        next.current = Some(resulting_grant);
        let mut receipt = Receipt {
            operation: command.grant.operation.clone(),
            input,
            prior,
            result_state: String::new(),
            action: command.action,
            predecessor: command.predecessor.clone(),
            projection: stored(&next),
        };
        next.operations
            .insert(command.grant.result.clone(), receipt.clone());
        // Hash the complete durable state with only this self-referential digest
        // field cleared; this canonical convention avoids recursive hashing.
        receipt.result_state = digest(&next)?;
        next.operations
            .insert(command.grant.result, receipt.clone());
        self.envelope = self.store.commit(&self.envelope, next)?;
        output(&receipt)
    }
}
fn validate_transition(
    current: Option<&Grant>,
    command: &TransitionCommand,
    expired: bool,
) -> Result<(), ObservatoryEligibilityError> {
    match command.action {
        ObservatoryTransitionAction::Acquire => {
            if current.is_some() || command.predecessor.is_some() {
                return Err(ObservatoryEligibilityError::StaleAuthority);
            }
        }
        ObservatoryTransitionAction::Renew
        | ObservatoryTransitionAction::Transfer
        | ObservatoryTransitionAction::Revoke => {
            let prior = current.ok_or(ObservatoryEligibilityError::StaleAuthority)?;
            if prior.status != Status::Eligible
                || command.predecessor.as_deref() != Some(prior.operation.as_str())
                || command.grant.lineage != prior.lineage
                || command.grant.trust != prior.trust
                || command.grant.polis != prior.polis
                || command.grant.index < prior.index
                || command.grant.fence <= prior.fence
            {
                return Err(ObservatoryEligibilityError::StaleAuthority);
            }
        }
    }
    if expired && command.action == ObservatoryTransitionAction::Acquire {
        return Err(ObservatoryEligibilityError::StaleAuthority);
    }
    Ok(())
}
fn grant(p: &VerifiedObservatoryAuthorityProjection, status: Status) -> Grant {
    Grant {
        trust: p.trust_domain_ref().into(),
        polis: p.polis_ref().into(),
        lineage: p.lineage_ref().into(),
        operation: p.operation_ref().into(),
        index: p.committed_log_index(),
        generation: p.foundation_generation(),
        fence: p.fencing_generation(),
        result: p.authority_result_sha256().into(),
        signers: p.signer_set_sha256().into(),
        signer_count: p.signer_count(),
        deadline_seconds: p.inclusive_deadline_unix_seconds(),
        deadline_nanos: p.inclusive_deadline_nanos(),
        deadline_uncertainty: p.inclusive_deadline_uncertainty_millis(),
        final_seconds: p.finalization_unix_seconds(),
        final_nanos: p.finalization_nanos(),
        final_uncertainty: p.finalization_uncertainty_millis(),
        status,
    }
}
fn stored(s: &State) -> StoredProjection {
    let g = s.current.as_ref();
    StoredProjection {
        status: g
            .map_or("ineligible", |x| match x.status {
                Status::Eligible => "eligible",
                Status::Revoked => "revoked",
                Status::Expired => "expired",
            })
            .into(),
        trust: g.map(|x| x.trust.clone()),
        polis: g.map(|x| x.polis.clone()),
        lineage: g.map(|x| x.lineage.clone()),
        operation: g.map(|x| x.operation.clone()),
        index: g.map_or(0, |x| x.index),
        generation: g.map_or(0, |x| x.generation),
        fence: g.map_or(0, |x| x.fence),
        result: g.map(|x| x.result.clone()),
        signers: g.map(|x| x.signers.clone()),
        signer_count: g.map_or(0, |x| x.signer_count),
    }
}
fn output(r: &Receipt) -> Result<ObservatoryEligibilityProjection, ObservatoryEligibilityError> {
    let p = &r.projection;
    Ok(ObservatoryEligibilityProjection {
        schema: "adl.observatory-serving-eligibility.projection.v1".into(),
        status: p.status.clone(),
        trust_domain_ref: p.trust.clone(),
        polis_ref: p.polis.clone(),
        lineage_ref: p.lineage.clone(),
        operation_ref: p.operation.clone(),
        committed_log_index: p.index,
        foundation_generation: p.generation,
        fencing_generation: p.fence,
        authority_result_sha256: p.result.clone(),
        signer_set_sha256: p.signers.clone(),
        signer_count: p.signer_count,
        state_sha256: r.result_state.clone(),
        receipt_sha256: hex::encode(Sha256::digest(
            serde_jcs::to_vec(r).map_err(|_| ObservatoryEligibilityError::Serialization)?,
        )),
    })
}
fn digest(s: &State) -> Result<String, ObservatoryEligibilityError> {
    serde_jcs::to_vec(s)
        .map(|b| hex::encode(Sha256::digest(b)))
        .map_err(|_| ObservatoryEligibilityError::Serialization)
}
fn input_digest(
    p: &VerifiedObservatoryAuthorityProjection,
    seconds: i64,
    nanos: u32,
) -> Result<String, ObservatoryEligibilityError> {
    #[derive(Serialize)]
    struct Input<'a> {
        operation: &'a str,
        action: ObservatoryTransitionAction,
        predecessor: Option<&'a str>,
        result: &'a str,
        seconds: i64,
        nanos: u32,
    }
    let value = Input {
        operation: p.operation_ref(),
        action: p.transition_action(),
        predecessor: p.predecessor_operation_ref(),
        result: p.authority_result_sha256(),
        seconds,
        nanos,
    };
    serde_jcs::to_vec(&value)
        .map(|b| hex::encode(Sha256::digest(b)))
        .map_err(|_| ObservatoryEligibilityError::Serialization)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn grant(operation: &str, fence: u64, status: Status) -> Grant {
        Grant {
            trust: "trust".into(),
            polis: "polis".into(),
            lineage: "lineage".into(),
            operation: operation.into(),
            index: 2,
            generation: 1,
            fence,
            result: format!("result-{operation}"),
            signers: "signers".into(),
            signer_count: 3,
            deadline_seconds: 10,
            deadline_nanos: 5,
            deadline_uncertainty: 1,
            final_seconds: 9,
            final_nanos: 5,
            final_uncertainty: 1,
            status,
        }
    }
    fn command(
        action: ObservatoryTransitionAction,
        operation: &str,
        predecessor: Option<&str>,
        fence: u64,
    ) -> TransitionCommand {
        TransitionCommand {
            grant: grant(operation, fence, Status::Eligible),
            action,
            predecessor: predecessor.map(str::to_owned),
        }
    }
    #[test]
    fn distinct_authenticated_transition_guard_is_monotone() {
        let acquired = grant("acquire", 1, Status::Eligible);
        assert!(validate_transition(
            None,
            &command(ObservatoryTransitionAction::Acquire, "acquire", None, 1),
            false
        )
        .is_ok());
        assert!(validate_transition(
            Some(&acquired),
            &command(
                ObservatoryTransitionAction::Renew,
                "renew",
                Some("acquire"),
                2
            ),
            false
        )
        .is_ok());
        let renewed = grant("renew", 2, Status::Eligible);
        assert!(validate_transition(
            Some(&renewed),
            &command(
                ObservatoryTransitionAction::Transfer,
                "transfer",
                Some("renew"),
                3
            ),
            false
        )
        .is_ok());
        let transferred = grant("transfer", 3, Status::Eligible);
        assert!(validate_transition(
            Some(&transferred),
            &command(
                ObservatoryTransitionAction::Revoke,
                "revoke",
                Some("transfer"),
                4
            ),
            false
        )
        .is_ok());
        assert_eq!(
            validate_transition(
                Some(&grant("revoke", 4, Status::Revoked)),
                &command(ObservatoryTransitionAction::Acquire, "revive", None, 5),
                false
            ),
            Err(ObservatoryEligibilityError::StaleAuthority)
        );
        assert_eq!(
            validate_transition(
                Some(&grant("expired", 4, Status::Expired)),
                &command(ObservatoryTransitionAction::Acquire, "revive", None, 5),
                false
            ),
            Err(ObservatoryEligibilityError::StaleAuthority)
        );
        assert_eq!(
            validate_transition(
                Some(&transferred),
                &command(
                    ObservatoryTransitionAction::Transfer,
                    "stale",
                    Some("wrong"),
                    4
                ),
                false
            ),
            Err(ObservatoryEligibilityError::StaleAuthority)
        );
    }
}
