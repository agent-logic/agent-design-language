//! Authenticated, bounded, advisory-only distributed failure detection.
//!
//! This module remains unregistered until integration issue #5878. It consumes signed probes from
//! enrolled membership identities and emits deterministic advisory projections. It never grants,
//! transfers, or revokes authority.

use std::{
    collections::{BTreeMap, VecDeque},
    fmt,
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const FAILURE_PROBE_SCHEMA: &str = "adl.distributed.failure_probe.v1";
pub const FAILURE_EVENT_SCHEMA: &str = "adl.distributed.failure_event.v1";
const SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-FAILURE-PROBE-V1\0";
const EVENT_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-FAILURE-EVENT-V1\0";
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_PROBE_BYTES: usize = 16 * 1024;
const MAX_PROBE_LIFETIME_SECS: u64 = 300;
const MAX_FUTURE_SKEW_SECS: u64 = 30;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureError {
    InvalidPolicy,
    InvalidProbe,
    ProbeTooLarge,
    WrongTrustDomain,
    WrongMembershipEpoch,
    ObserverNotEnrolled,
    ObserverGenerationNotCurrent,
    ObserverNotMember,
    SubjectNotMember,
    InvalidSignature,
    Replay,
    StaleProbe,
    FutureProbe,
    ResourceExhausted,
    UnknownSubject,
    Encoding,
    RevisionDrift,
}

impl FailureError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidProbe => "invalid_probe",
            Self::ProbeTooLarge => "probe_too_large",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::WrongMembershipEpoch => "wrong_membership_epoch",
            Self::ObserverNotEnrolled => "observer_not_enrolled",
            Self::ObserverGenerationNotCurrent => "observer_generation_not_current",
            Self::ObserverNotMember => "observer_not_member",
            Self::SubjectNotMember => "subject_not_member",
            Self::InvalidSignature => "invalid_signature",
            Self::Replay => "replay",
            Self::StaleProbe => "stale_probe",
            Self::FutureProbe => "future_probe",
            Self::ResourceExhausted => "resource_exhausted",
            Self::UnknownSubject => "unknown_subject",
            Self::Encoding => "encoding_error",
            Self::RevisionDrift => "revision_drift",
        }
    }
}

impl fmt::Display for FailureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for FailureError {}

pub type FailureResult<T> = Result<T, FailureError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FailureThresholds {
    pub suspect_after_secs: u64,
    pub unavailable_after_secs: u64,
    pub evidence_window_secs: u64,
    pub flap_window_secs: u64,
}

#[derive(Clone, Debug)]
pub struct FailurePolicy {
    trust_domain: String,
    local_node_id: String,
    membership_epoch: u64,
    thresholds: FailureThresholds,
    observer_quorum: usize,
    recovery_confirmations: u8,
    flap_limit: usize,
    max_nodes: usize,
    max_observers_per_node: usize,
    max_events: usize,
}

impl FailurePolicy {
    pub fn new(
        trust_domain: impl Into<String>,
        local_node_id: impl Into<String>,
        membership_epoch: u64,
        thresholds: FailureThresholds,
        observer_quorum: usize,
    ) -> FailureResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            local_node_id: local_node_id.into(),
            membership_epoch,
            thresholds,
            observer_quorum,
            recovery_confirmations: 2,
            flap_limit: 4,
            max_nodes: 4096,
            max_observers_per_node: 64,
            max_events: 4096,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn with_bounds(
        mut self,
        recovery_confirmations: u8,
        flap_limit: usize,
        max_nodes: usize,
        max_observers_per_node: usize,
        max_events: usize,
    ) -> FailureResult<Self> {
        self.recovery_confirmations = recovery_confirmations;
        self.flap_limit = flap_limit;
        self.max_nodes = max_nodes;
        self.max_observers_per_node = max_observers_per_node;
        self.max_events = max_events;
        self.validate()?;
        Ok(self)
    }

    fn validate(&self) -> FailureResult<()> {
        let t = self.thresholds;
        if !valid_identifier(&self.trust_domain)
            || !valid_identifier(&self.local_node_id)
            || self.membership_epoch == 0
            || t.suspect_after_secs == 0
            || t.unavailable_after_secs <= t.suspect_after_secs
            || t.evidence_window_secs == 0
            || t.evidence_window_secs > t.unavailable_after_secs
            || t.flap_window_secs < t.unavailable_after_secs
            || self.observer_quorum == 0
            || self.recovery_confirmations == 0
            || self.flap_limit < 2
            || self.max_nodes == 0
            || self.max_nodes > 4096
            || self.max_observers_per_node < self.observer_quorum
            || self.max_observers_per_node > 256
            || self.max_events == 0
            || self.max_events > 65_536
        {
            return Err(FailureError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeResult {
    Reachable,
    Unreachable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureProbeClaims {
    pub schema: String,
    pub trust_domain: String,
    pub membership_epoch: u64,
    pub observer_node_id: String,
    pub observer_identity_generation: u64,
    pub subject_node_id: String,
    pub sequence: u64,
    pub observed_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub result: ProbeResult,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedFailureProbe {
    pub claims: FailureProbeClaims,
    pub signature: Vec<u8>,
}

impl SignedFailureProbe {
    pub fn sign(claims: FailureProbeClaims, signer: &SigningKey) -> FailureResult<Self> {
        validate_claim_shape(&claims)?;
        let signature = signer.sign(&signing_bytes(&claims)?).to_bytes().to_vec();
        Ok(Self { claims, signature })
    }
}

pub trait ProbeAuthority {
    fn current_observer_identity(&self, observer_node_id: &str) -> Option<(u64, VerifyingKey)>;

    fn is_member(&self, node_id: &str, membership_epoch: u64) -> bool;
}

pub trait FailureMembershipAuthority {
    fn membership_epoch(&self) -> u64;

    fn committed_log_index(&self) -> u64;

    fn complete_members(&self) -> FailureResult<Vec<(String, String)>>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureClass {
    Healthy,
    Suspect,
    Unavailable,
    Partitioned,
    Recovered,
    Flapping,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FailureProjection {
    pub trust_domain: String,
    pub subject_node_id: String,
    pub membership_epoch: u64,
    pub class: FailureClass,
    pub evaluated_at_unix_secs: u64,
    pub supporting_observers: u16,
    pub advisory_only: bool,
    pub authority_granted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FailureEvent {
    pub schema: String,
    pub event_id: String,
    pub sequence: u64,
    pub projection: FailureProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FailureAuthorityRevision {
    sequence: u64,
    content_sha256: [u8; 32],
}

impl FailureAuthorityRevision {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn content_sha256(&self) -> [u8; 32] {
        self.content_sha256
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureSnapshotReason {
    NoEvidence,
    HealthyEvidence,
    SuspectedFailure,
    QuorumUnavailable,
    NetworkPartition,
    RecoveryEvidence,
    FlappingEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureEvidenceBand {
    None,
    Observed,
    Corroborated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureFreshness {
    Unavailable,
    Fresh,
    Stale,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedFailureRow {
    node_ref: String,
    guardian_ref: String,
    class: Option<FailureClass>,
    reason: FailureSnapshotReason,
    evidence: FailureEvidenceBand,
    freshness: FailureFreshness,
}

impl RedactedFailureRow {
    pub fn node_ref(&self) -> &str {
        &self.node_ref
    }

    pub fn guardian_ref(&self) -> &str {
        &self.guardian_ref
    }

    pub fn class(&self) -> Option<FailureClass> {
        self.class
    }

    pub fn reason(&self) -> FailureSnapshotReason {
        self.reason
    }

    pub fn evidence(&self) -> FailureEvidenceBand {
        self.evidence
    }

    pub fn freshness(&self) -> FailureFreshness {
        self.freshness
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedFailureSnapshot {
    trust_domain: String,
    membership_epoch: u64,
    committed_log_index: u64,
    captured_at_unix_secs: u64,
    revision: FailureAuthorityRevision,
    rows: Vec<RedactedFailureRow>,
}

impl RedactedFailureSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn membership_epoch(&self) -> u64 {
        self.membership_epoch
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn captured_at_unix_secs(&self) -> u64 {
        self.captured_at_unix_secs
    }

    pub fn revision(&self) -> FailureAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedFailureRow> {
        self.rows.iter()
    }
}

#[derive(Clone, Debug)]
struct Observation {
    result: ProbeResult,
    observed_at_unix_secs: u64,
    expires_at_unix_secs: u64,
}

#[derive(Clone, Debug)]
struct SubjectState {
    first_observed_at: u64,
    last_local_reachable_at: Option<u64>,
    local_result: Option<ProbeResult>,
    recovery_streak: u8,
    observers: BTreeMap<String, Observation>,
    last_class: FailureClass,
    transition_times: VecDeque<u64>,
}

impl SubjectState {
    fn new(now: u64) -> Self {
        Self {
            first_observed_at: now,
            last_local_reachable_at: None,
            local_result: None,
            recovery_streak: 0,
            observers: BTreeMap::new(),
            last_class: FailureClass::Healthy,
            transition_times: VecDeque::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FailureDetector {
    policy: FailurePolicy,
    subjects: BTreeMap<String, SubjectState>,
    last_sequences: BTreeMap<(String, String), (u64, u64)>,
    events: VecDeque<FailureEvent>,
    next_event_sequence: u64,
    revision_sequence: u64,
}

impl FailureDetector {
    pub fn new(policy: FailurePolicy) -> Self {
        Self {
            policy,
            subjects: BTreeMap::new(),
            last_sequences: BTreeMap::new(),
            events: VecDeque::new(),
            next_event_sequence: 1,
            revision_sequence: 0,
        }
    }

    pub fn observe<A: ProbeAuthority>(
        &mut self,
        authority: &A,
        probe: &SignedFailureProbe,
        now_unix_secs: u64,
    ) -> FailureResult<Option<FailureEvent>> {
        self.verify(authority, probe, now_unix_secs)?;
        self.revision_sequence
            .checked_add(2)
            .ok_or(FailureError::ResourceExhausted)?;
        let claims = &probe.claims;
        if !self.subjects.contains_key(&claims.subject_node_id)
            && self.subjects.len() >= self.policy.max_nodes
        {
            return Err(FailureError::ResourceExhausted);
        }
        let key = (
            claims.observer_node_id.clone(),
            claims.subject_node_id.clone(),
        );
        if let Some((generation, sequence)) = self.last_sequences.get(&key) {
            if claims.observer_identity_generation < *generation
                || (claims.observer_identity_generation == *generation
                    && claims.sequence <= *sequence)
            {
                return Err(FailureError::Replay);
            }
        }

        let state = self
            .subjects
            .entry(claims.subject_node_id.clone())
            .or_insert_with(|| SubjectState::new(claims.observed_at_unix_secs));
        if !state.observers.contains_key(&claims.observer_node_id)
            && state.observers.len() >= self.policy.max_observers_per_node
        {
            return Err(FailureError::ResourceExhausted);
        }
        if claims.observer_node_id == self.policy.local_node_id {
            state.local_result = Some(claims.result);
            match claims.result {
                ProbeResult::Reachable => {
                    state.last_local_reachable_at = Some(claims.observed_at_unix_secs);
                    state.recovery_streak = state.recovery_streak.saturating_add(1);
                }
                ProbeResult::Unreachable => state.recovery_streak = 0,
            }
        }
        state.observers.insert(
            claims.observer_node_id.clone(),
            Observation {
                result: claims.result,
                observed_at_unix_secs: claims.observed_at_unix_secs,
                expires_at_unix_secs: claims.expires_at_unix_secs,
            },
        );
        self.last_sequences
            .insert(key, (claims.observer_identity_generation, claims.sequence));
        let result = self.evaluate(&claims.subject_node_id, now_unix_secs);
        if result.is_ok() {
            self.advance_revision()?;
        }
        result
    }

    pub fn evaluate(
        &mut self,
        subject_node_id: &str,
        now_unix_secs: u64,
    ) -> FailureResult<Option<FailureEvent>> {
        self.revision_sequence
            .checked_add(1)
            .ok_or(FailureError::ResourceExhausted)?;
        let state = self
            .subjects
            .get_mut(subject_node_id)
            .ok_or(FailureError::UnknownSubject)?;
        let flap_cutoff = now_unix_secs.saturating_sub(self.policy.thresholds.flap_window_secs);
        while state
            .transition_times
            .front()
            .is_some_and(|timestamp| *timestamp < flap_cutoff)
        {
            state.transition_times.pop_front();
        }

        let (mut next, support) = classify(&self.policy, state, now_unix_secs);
        if next != state.last_class {
            state.transition_times.push_back(now_unix_secs);
            if state.transition_times.len() >= self.policy.flap_limit {
                next = FailureClass::Flapping;
            }
        }
        if next == state.last_class {
            self.advance_revision()?;
            return Ok(None);
        }
        state.last_class = next;
        let projection = FailureProjection {
            trust_domain: self.policy.trust_domain.clone(),
            subject_node_id: subject_node_id.to_owned(),
            membership_epoch: self.policy.membership_epoch,
            class: next,
            evaluated_at_unix_secs: now_unix_secs,
            supporting_observers: u16::try_from(support)
                .map_err(|_| FailureError::ResourceExhausted)?,
            advisory_only: true,
            authority_granted: false,
        };
        let sequence = self.next_event_sequence;
        self.next_event_sequence = self
            .next_event_sequence
            .checked_add(1)
            .ok_or(FailureError::ResourceExhausted)?;
        let event = FailureEvent {
            schema: FAILURE_EVENT_SCHEMA.to_owned(),
            event_id: event_id(sequence, &projection)?,
            sequence,
            projection,
        };
        if self.events.len() == self.policy.max_events {
            self.events.pop_front();
        }
        self.events.push_back(event.clone());
        self.advance_revision()?;
        Ok(Some(event))
    }

    pub fn projection(
        &self,
        subject_node_id: &str,
        now_unix_secs: u64,
    ) -> FailureResult<FailureProjection> {
        let state = self
            .subjects
            .get(subject_node_id)
            .ok_or(FailureError::UnknownSubject)?;
        let (class, support) = classify(&self.policy, state, now_unix_secs);
        Ok(FailureProjection {
            trust_domain: self.policy.trust_domain.clone(),
            subject_node_id: subject_node_id.to_owned(),
            membership_epoch: self.policy.membership_epoch,
            class,
            evaluated_at_unix_secs: now_unix_secs,
            supporting_observers: u16::try_from(support)
                .map_err(|_| FailureError::ResourceExhausted)?,
            advisory_only: true,
            authority_granted: false,
        })
    }

    pub fn events(&self) -> impl Iterator<Item = &FailureEvent> {
        self.events.iter()
    }

    pub fn replay_record_count(&self) -> usize {
        self.last_sequences.len()
    }

    pub fn authority_revision<A: FailureMembershipAuthority>(
        &self,
        membership: &A,
        now_unix_secs: u64,
    ) -> FailureResult<FailureAuthorityRevision> {
        let rows = self.redacted_rows(membership, now_unix_secs)?;
        Ok(FailureAuthorityRevision {
            sequence: self.revision_sequence,
            content_sha256: failure_content_sha256(
                &self.policy.trust_domain,
                membership.membership_epoch(),
                membership.committed_log_index(),
                now_unix_secs,
                &rows,
            ),
        })
    }

    pub fn redacted_snapshot_at<A: FailureMembershipAuthority>(
        &self,
        expected_revision: FailureAuthorityRevision,
        membership: &A,
        now_unix_secs: u64,
    ) -> FailureResult<RedactedFailureSnapshot> {
        let rows = self.redacted_rows(membership, now_unix_secs)?;
        let revision = FailureAuthorityRevision {
            sequence: self.revision_sequence,
            content_sha256: failure_content_sha256(
                &self.policy.trust_domain,
                membership.membership_epoch(),
                membership.committed_log_index(),
                now_unix_secs,
                &rows,
            ),
        };
        if revision != expected_revision {
            return Err(FailureError::RevisionDrift);
        }
        Ok(RedactedFailureSnapshot {
            trust_domain: self.policy.trust_domain.clone(),
            membership_epoch: membership.membership_epoch(),
            committed_log_index: membership.committed_log_index(),
            captured_at_unix_secs: now_unix_secs,
            revision,
            rows,
        })
    }

    fn redacted_rows<A: FailureMembershipAuthority>(
        &self,
        membership: &A,
        now_unix_secs: u64,
    ) -> FailureResult<Vec<RedactedFailureRow>> {
        let members = membership.complete_members()?;
        if membership.membership_epoch() != self.policy.membership_epoch
            || members.len() > self.policy.max_nodes
        {
            return Err(FailureError::WrongMembershipEpoch);
        }
        let mut previous = None;
        members
            .into_iter()
            .map(|(node_id, guardian_id)| {
                if !valid_identifier(&node_id)
                    || !valid_identifier(&guardian_id)
                    || previous
                        .as_deref()
                        .is_some_and(|value| value >= node_id.as_str())
                {
                    return Err(FailureError::InvalidProbe);
                }
                previous = Some(node_id.clone());
                let Some(state) = self.subjects.get(&node_id) else {
                    return Ok(RedactedFailureRow {
                        node_ref: projection_ref(b"node", node_id.as_bytes()),
                        guardian_ref: projection_ref(b"guardian", guardian_id.as_bytes()),
                        class: None,
                        reason: FailureSnapshotReason::NoEvidence,
                        evidence: FailureEvidenceBand::None,
                        freshness: FailureFreshness::Unavailable,
                    });
                };
                let (class, support) = classify(&self.policy, state, now_unix_secs);
                let reason = match class {
                    FailureClass::Healthy => FailureSnapshotReason::HealthyEvidence,
                    FailureClass::Suspect => FailureSnapshotReason::SuspectedFailure,
                    FailureClass::Unavailable => FailureSnapshotReason::QuorumUnavailable,
                    FailureClass::Partitioned => FailureSnapshotReason::NetworkPartition,
                    FailureClass::Recovered => FailureSnapshotReason::RecoveryEvidence,
                    FailureClass::Flapping => FailureSnapshotReason::FlappingEvidence,
                };
                Ok(RedactedFailureRow {
                    node_ref: projection_ref(b"node", node_id.as_bytes()),
                    guardian_ref: projection_ref(b"guardian", guardian_id.as_bytes()),
                    class: Some(class),
                    reason,
                    evidence: if support > 1 {
                        FailureEvidenceBand::Corroborated
                    } else {
                        FailureEvidenceBand::Observed
                    },
                    freshness: if state
                        .observers
                        .values()
                        .map(|observation| observation.expires_at_unix_secs)
                        .max()
                        .is_some_and(|expires_at| expires_at >= now_unix_secs)
                    {
                        FailureFreshness::Fresh
                    } else {
                        FailureFreshness::Stale
                    },
                })
            })
            .collect()
    }

    fn advance_revision(&mut self) -> FailureResult<()> {
        self.revision_sequence = self
            .revision_sequence
            .checked_add(1)
            .ok_or(FailureError::ResourceExhausted)?;
        Ok(())
    }

    fn verify<A: ProbeAuthority>(
        &self,
        authority: &A,
        probe: &SignedFailureProbe,
        now_unix_secs: u64,
    ) -> FailureResult<()> {
        let encoded = serde_jcs::to_vec(probe).map_err(|_| FailureError::Encoding)?;
        if encoded.len() > MAX_PROBE_BYTES {
            return Err(FailureError::ProbeTooLarge);
        }
        let claims = &probe.claims;
        validate_claim_shape(claims)?;
        if claims.trust_domain != self.policy.trust_domain {
            return Err(FailureError::WrongTrustDomain);
        }
        if claims.membership_epoch != self.policy.membership_epoch {
            return Err(FailureError::WrongMembershipEpoch);
        }
        if !authority.is_member(&claims.subject_node_id, claims.membership_epoch) {
            return Err(FailureError::SubjectNotMember);
        }
        if !authority.is_member(&claims.observer_node_id, claims.membership_epoch) {
            return Err(FailureError::ObserverNotMember);
        }
        let (current_generation, key) = authority
            .current_observer_identity(&claims.observer_node_id)
            .ok_or(FailureError::ObserverNotEnrolled)?;
        if claims.observer_identity_generation != current_generation {
            return Err(FailureError::ObserverGenerationNotCurrent);
        }
        if claims.observed_at_unix_secs > now_unix_secs.saturating_add(MAX_FUTURE_SKEW_SECS) {
            return Err(FailureError::FutureProbe);
        }
        if claims.expires_at_unix_secs < now_unix_secs {
            return Err(FailureError::StaleProbe);
        }
        let signature =
            Signature::from_slice(&probe.signature).map_err(|_| FailureError::InvalidSignature)?;
        key.verify(&signing_bytes(claims)?, &signature)
            .map_err(|_| FailureError::InvalidSignature)
    }
}

fn failure_content_sha256(
    trust_domain: &str,
    membership_epoch: u64,
    committed_log_index: u64,
    captured_at_unix_secs: u64,
    rows: &[RedactedFailureRow],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"ADL-FAILURE-AUTHORITY-REVISION-V1\0");
    digest.update((trust_domain.len() as u64).to_be_bytes());
    digest.update(trust_domain.as_bytes());
    digest.update(membership_epoch.to_be_bytes());
    digest.update(committed_log_index.to_be_bytes());
    digest.update(captured_at_unix_secs.to_be_bytes());
    for row in rows {
        for value in [&row.node_ref, &row.guardian_ref] {
            digest.update((value.len() as u64).to_be_bytes());
            digest.update(value.as_bytes());
        }
        digest.update([row.class.map_or(0, |value| value as u8 + 1)]);
        digest.update([row.reason as u8]);
        digest.update([row.evidence as u8]);
        digest.update([row.freshness as u8]);
    }
    digest.finalize().into()
}

fn projection_ref(kind: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"adl-projection-ref-v1");
    digest.update((kind.len() as u64).to_be_bytes());
    digest.update(kind);
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
    format!("id_{}", hex::encode(digest.finalize()))
}

fn classify(policy: &FailurePolicy, state: &SubjectState, now: u64) -> (FailureClass, usize) {
    let flap_cutoff = now.saturating_sub(policy.thresholds.flap_window_secs);
    if state.last_class == FailureClass::Flapping
        && state
            .transition_times
            .iter()
            .filter(|timestamp| **timestamp >= flap_cutoff)
            .count()
            >= policy.flap_limit
    {
        return (FailureClass::Flapping, 0);
    }
    if state.local_result == Some(ProbeResult::Reachable) {
        let last = state
            .last_local_reachable_at
            .unwrap_or(state.first_observed_at);
        if now.saturating_sub(last) < policy.thresholds.suspect_after_secs {
            let class = if state.last_class == FailureClass::Healthy {
                FailureClass::Healthy
            } else if state.recovery_streak >= policy.recovery_confirmations {
                FailureClass::Recovered
            } else {
                state.last_class
            };
            return (class, 1);
        }
    }

    let baseline = state
        .last_local_reachable_at
        .unwrap_or(state.first_observed_at);
    let silence = now.saturating_sub(baseline);
    if silence < policy.thresholds.suspect_after_secs {
        return (FailureClass::Healthy, 1);
    }
    if silence < policy.thresholds.unavailable_after_secs {
        return (FailureClass::Suspect, 0);
    }

    let cutoff = now.saturating_sub(policy.thresholds.evidence_window_secs);
    let mut reachable = 0;
    let mut unreachable = 0;
    for (observer, observation) in &state.observers {
        if observer == &policy.local_node_id
            || observation.observed_at_unix_secs < cutoff
            || observation.expires_at_unix_secs < now
        {
            continue;
        }
        match observation.result {
            ProbeResult::Reachable => reachable += 1,
            ProbeResult::Unreachable => unreachable += 1,
        }
    }
    if reachable >= policy.observer_quorum {
        (FailureClass::Partitioned, reachable)
    } else if unreachable >= policy.observer_quorum {
        (FailureClass::Unavailable, unreachable)
    } else {
        (FailureClass::Suspect, reachable + unreachable)
    }
}

fn validate_claim_shape(claims: &FailureProbeClaims) -> FailureResult<()> {
    if claims.schema != FAILURE_PROBE_SCHEMA
        || !valid_identifier(&claims.trust_domain)
        || !valid_identifier(&claims.observer_node_id)
        || !valid_identifier(&claims.subject_node_id)
        || claims.observer_node_id == claims.subject_node_id
        || claims.membership_epoch == 0
        || claims.observer_identity_generation == 0
        || claims.sequence == 0
        || claims.observed_at_unix_secs == 0
        || claims.expires_at_unix_secs < claims.observed_at_unix_secs
        || claims
            .expires_at_unix_secs
            .saturating_sub(claims.observed_at_unix_secs)
            > MAX_PROBE_LIFETIME_SECS
    {
        return Err(FailureError::InvalidProbe);
    }
    Ok(())
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_IDENTIFIER_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn signing_bytes(claims: &FailureProbeClaims) -> FailureResult<Vec<u8>> {
    let canonical = serde_jcs::to_vec(claims).map_err(|_| FailureError::Encoding)?;
    let mut bytes = Vec::with_capacity(SIGNING_DOMAIN.len() + canonical.len());
    bytes.extend_from_slice(SIGNING_DOMAIN);
    bytes.extend_from_slice(&canonical);
    Ok(bytes)
}

fn event_id(sequence: u64, projection: &FailureProjection) -> FailureResult<String> {
    let canonical = serde_jcs::to_vec(projection).map_err(|_| FailureError::Encoding)?;
    let mut hasher = Sha256::new();
    hasher.update(EVENT_DOMAIN);
    hasher.update(sequence.to_be_bytes());
    hasher.update(canonical);
    Ok(format!("failure_{}", hex::encode(hasher.finalize())))
}
