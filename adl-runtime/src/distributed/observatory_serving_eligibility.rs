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
const IJSON_MAX_INTEGER: u64 = 9_007_199_254_740_991;
const SEALED_OBSERVATORY_DOMAIN: &str = "ADL-SEALED-OBSERVATORY-COMMITTED-PROJECTION-V1";
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
/// Store-derived committed Observatory eligibility evidence.
///
/// ```compile_fail
/// use adl_runtime::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection;
/// let _ = SealedObservatoryCommittedProjection { provenance_sha256: String::new() };
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedObservatoryCommittedProjection {
    preimage: ObservatoryCommittedPreimage,
    provenance_sha256: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryCommittedPreimage {
    domain: String,
    child_kind: String,
    envelope_generation: u64,
    committed_revision: u64,
    payload_sha256: String,
    receipt_sha256: String,
    state_sha256: String,
    projection: ObservatoryEligibilityProjection,
}
impl SealedObservatoryCommittedProjection {
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
    pub fn trust_domain_ref(&self) -> Option<&str> {
        self.preimage.projection.trust_domain_ref.as_deref()
    }
    pub fn polis_ref(&self) -> Option<&str> {
        self.preimage.projection.polis_ref.as_deref()
    }
    pub fn lineage_ref(&self) -> Option<&str> {
        self.preimage.projection.lineage_ref.as_deref()
    }
    pub fn operation_ref(&self) -> Option<&str> {
        self.preimage.projection.operation_ref.as_deref()
    }
    pub fn committed_log_index(&self) -> u64 {
        self.preimage.projection.committed_log_index
    }
    pub fn foundation_generation(&self) -> u64 {
        self.preimage.projection.foundation_generation
    }
    pub fn fencing_generation(&self) -> u64 {
        self.preimage.projection.fencing_generation
    }
    pub fn authority_result_sha256(&self) -> Option<&str> {
        self.preimage.projection.authority_result_sha256.as_deref()
    }
    pub fn signer_set_sha256(&self) -> Option<&str> {
        self.preimage.projection.signer_set_sha256.as_deref()
    }
    pub fn signer_count(&self) -> usize {
        self.preimage.projection.signer_count
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ObservatoryEligibilityError> {
        serde_jcs::to_vec(self).map_err(|_| ObservatoryEligibilityError::Serialization)
    }
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
        if let Some(current) = envelope.payload().current.as_ref() {
            let receipt = envelope
                .payload()
                .operations
                .get(&current.result)
                .ok_or(ObservatoryEligibilityError::Serialization)?;
            if receipt.result_state
                != normalized_final_state_digest(envelope.payload(), &current.result)?
            {
                return Err(ObservatoryEligibilityError::Serialization);
            }
        }
        Ok(Self {
            store,
            envelope,
            capacity,
        })
    }
    pub fn committed_projection(
        &self,
    ) -> Result<Option<SealedObservatoryCommittedProjection>, ObservatoryEligibilityError> {
        let state = self.envelope.payload();
        let Some(current) = state.current.as_ref() else {
            return Ok(None);
        };
        let receipt = state
            .operations
            .get(&current.result)
            .ok_or(ObservatoryEligibilityError::Serialization)?;
        if receipt.result_state != normalized_final_state_digest(state, &current.result)? {
            return Err(ObservatoryEligibilityError::Serialization);
        }
        let projection = output(receipt)?;
        let payload_sha256 = digest(state)?;
        if payload_sha256 != self.envelope.payload_sha256() {
            return Err(ObservatoryEligibilityError::Serialization);
        }
        seal_observatory_projection(ObservatoryCommittedPreimage {
            domain: SEALED_OBSERVATORY_DOMAIN.into(),
            child_kind: "observatory".into(),
            envelope_generation: self.envelope.generation(),
            committed_revision: state.revision,
            payload_sha256,
            receipt_sha256: projection.receipt_sha256.clone(),
            state_sha256: projection.state_sha256.clone(),
            projection,
        })
        .map(Some)
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
        // Hash the final committed state with only this receipt's recursive
        // result-state field normalized empty.
        receipt.result_state = normalized_final_state_digest(&next, &command.grant.result)?;
        next.operations
            .insert(command.grant.result, receipt.clone());
        self.envelope = self.store.commit(&self.envelope, next)?;
        output(&receipt)
    }
}
fn seal_observatory_projection(
    preimage: ObservatoryCommittedPreimage,
) -> Result<SealedObservatoryCommittedProjection, ObservatoryEligibilityError> {
    validate_observatory_preimage(&preimage)?;
    let provenance_sha256 = hex::encode(Sha256::digest(
        serde_jcs::to_vec(&preimage).map_err(|_| ObservatoryEligibilityError::Serialization)?,
    ));
    Ok(SealedObservatoryCommittedProjection {
        preimage,
        provenance_sha256,
    })
}
#[cfg(test)]
fn verify_sealed_observatory_projection(
    sealed: &SealedObservatoryCommittedProjection,
) -> Result<(), ObservatoryEligibilityError> {
    validate_observatory_preimage(&sealed.preimage)?;
    let expected = hex::encode(Sha256::digest(
        serde_jcs::to_vec(&sealed.preimage)
            .map_err(|_| ObservatoryEligibilityError::Serialization)?,
    ));
    if expected != sealed.provenance_sha256 {
        return Err(ObservatoryEligibilityError::Serialization);
    }
    Ok(())
}
fn validate_observatory_preimage(
    p: &ObservatoryCommittedPreimage,
) -> Result<(), ObservatoryEligibilityError> {
    if p.domain != SEALED_OBSERVATORY_DOMAIN
        || p.child_kind != "observatory"
        || p.envelope_generation > IJSON_MAX_INTEGER
        || p.committed_revision == 0
        || p.committed_revision > IJSON_MAX_INTEGER
        || p.projection.schema != "adl.observatory-serving-eligibility.projection.v1"
        || p.receipt_sha256 != p.projection.receipt_sha256
        || p.state_sha256 != p.projection.state_sha256
        || !is_sha256(&p.payload_sha256)
        || !is_sha256(&p.receipt_sha256)
        || !is_sha256(&p.state_sha256)
    {
        return Err(ObservatoryEligibilityError::Serialization);
    }
    Ok(())
}
fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
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
fn normalized_final_state_digest(
    state: &State,
    operation_result: &str,
) -> Result<String, ObservatoryEligibilityError> {
    let mut normalized = state.clone();
    let receipt = normalized
        .operations
        .get_mut(operation_result)
        .ok_or(ObservatoryEligibilityError::Serialization)?;
    receipt.result_state.clear();
    digest(&normalized)
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

    #[test]
    fn normalized_final_state_digest_recomputes_and_rejects_tamper() {
        let result = "result-acquire".to_owned();
        let mut state = State {
            revision: 1,
            current: Some(grant("acquire", 1, Status::Eligible)),
            operations: BTreeMap::new(),
        };
        state.operations.insert(
            result.clone(),
            Receipt {
                operation: "acquire".into(),
                input: "input".into(),
                prior: "prior".into(),
                result_state: String::new(),
                action: ObservatoryTransitionAction::Acquire,
                predecessor: None,
                projection: stored(&state),
            },
        );
        let expected = normalized_final_state_digest(&state, &result).unwrap();
        state.operations.get_mut(&result).unwrap().result_state = expected.clone();
        assert_eq!(
            normalized_final_state_digest(&state, &result).unwrap(),
            expected
        );
        state.current.as_mut().unwrap().fence += 1;
        assert_ne!(
            normalized_final_state_digest(&state, &result).unwrap(),
            expected
        );
    }

    #[test]
    fn sealed_committed_projection_private_provenance() {
        let projection = ObservatoryEligibilityProjection {
            schema: "adl.observatory-serving-eligibility.projection.v1".into(),
            status: "eligible".into(),
            trust_domain_ref: Some("11".repeat(32)),
            polis_ref: Some("22".repeat(32)),
            lineage_ref: Some("33".repeat(32)),
            operation_ref: Some("44".repeat(32)),
            committed_log_index: 2,
            foundation_generation: 1,
            fencing_generation: 1,
            authority_result_sha256: Some("55".repeat(32)),
            signer_set_sha256: Some("66".repeat(32)),
            signer_count: 3,
            state_sha256: "77".repeat(32),
            receipt_sha256: "88".repeat(32),
        };
        let preimage = ObservatoryCommittedPreimage {
            domain: SEALED_OBSERVATORY_DOMAIN.into(),
            child_kind: "observatory".into(),
            envelope_generation: 2,
            committed_revision: 1,
            payload_sha256: "99".repeat(32),
            receipt_sha256: projection.receipt_sha256.clone(),
            state_sha256: projection.state_sha256.clone(),
            projection,
        };
        let sealed = seal_observatory_projection(preimage).unwrap();
        verify_sealed_observatory_projection(&sealed).unwrap();
        let bytes = serde_jcs::to_vec(&sealed.preimage).unwrap();
        let reopened: ObservatoryCommittedPreimage = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(reopened, sealed.preimage);

        for mutation in 0..6 {
            let mut changed = sealed.clone();
            match mutation {
                0 => changed.preimage.child_kind = "shepherd".into(),
                1 => changed.preimage.envelope_generation += 1,
                2 => changed.preimage.committed_revision += 1,
                3 => changed.preimage.receipt_sha256 = "aa".repeat(32),
                4 => changed.preimage.projection.operation_ref = Some("bb".repeat(32)),
                _ => changed.preimage.projection.committed_log_index += 1,
            }
            assert_eq!(
                verify_sealed_observatory_projection(&changed),
                Err(ObservatoryEligibilityError::Serialization)
            );
        }
        let mut unknown = serde_json::to_value(&sealed.preimage).unwrap();
        unknown
            .as_object_mut()
            .unwrap()
            .insert("unknown".into(), true.into());
        assert!(serde_json::from_value::<ObservatoryCommittedPreimage>(unknown).is_err());
    }
}
