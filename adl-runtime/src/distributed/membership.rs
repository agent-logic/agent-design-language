use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const MEMBERSHIP_EVENT_SCHEMA: &str = "adl.distributed.membership_event.v1";
pub const MEMBERSHIP_SNAPSHOT_SCHEMA: &str = "adl.distributed.membership_snapshot.v1";

const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_SNAPSHOT_BYTES: usize = 1024 * 1024;
const SNAPSHOT_BASE_BUDGET: usize = 1024;
const SNAPSHOT_MEMBER_BUDGET: usize = 1024;
const SNAPSHOT_REPLAY_ENTRY_BUDGET: usize = 320;
const SNAPSHOT_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-MEMBERSHIP-SNAPSHOT-V1\0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MembershipError {
    InvalidPolicy,
    InvalidEvent,
    WrongTrustDomain,
    EpochGap,
    StaleEpoch,
    ReplayConflict,
    MemberAlreadyExists,
    MemberNotFound,
    AlreadyVoter,
    DuplicateGuardianControlKey,
    ResourceExhausted,
    SnapshotTooLarge,
    SnapshotCorrupt,
    Encoding,
}

impl MembershipError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidEvent => "invalid_event",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::EpochGap => "epoch_gap",
            Self::StaleEpoch => "stale_epoch",
            Self::ReplayConflict => "replay_conflict",
            Self::MemberAlreadyExists => "member_already_exists",
            Self::MemberNotFound => "member_not_found",
            Self::AlreadyVoter => "already_voter",
            Self::DuplicateGuardianControlKey => "duplicate_guardian_control_key",
            Self::ResourceExhausted => "resource_exhausted",
            Self::SnapshotTooLarge => "snapshot_too_large",
            Self::SnapshotCorrupt => "snapshot_corrupt",
            Self::Encoding => "encoding_error",
        }
    }
}

impl fmt::Display for MembershipError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for MembershipError {}

pub type MembershipResult<T> = Result<T, MembershipError>;

#[derive(Clone, Debug)]
pub struct MembershipPolicy {
    trust_domain: String,
    max_members: usize,
    replay_window: usize,
}

impl MembershipPolicy {
    pub fn new(
        trust_domain: impl Into<String>,
        max_members: usize,
        replay_window: usize,
    ) -> MembershipResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_members,
            replay_window,
        };
        if !valid_identifier(&policy.trust_domain)
            || policy.max_members == 0
            || policy.replay_window == 0
            || policy.replay_window < policy.max_members
            || policy.max_members > 4096
            || policy.replay_window > 65_536
            || snapshot_policy_budget(policy.max_members, policy.replay_window)
                .is_none_or(|bytes| bytes > MAX_SNAPSHOT_BYTES)
        {
            return Err(MembershipError::InvalidPolicy);
        }
        Ok(policy)
    }

    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    NonVoting,
    Voter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub node_id: String,
    pub guardian_id: String,
    pub identity_generation: u64,
    pub guardian_control_public_key: [u8; 32],
    pub role: MemberRole,
}

impl Member {
    fn validate(&self) -> MembershipResult<()> {
        if !valid_identifier(&self.node_id)
            || !valid_identifier(&self.guardian_id)
            || self.identity_generation == 0
            || self.guardian_control_public_key == [0; 32]
            || self.role != MemberRole::NonVoting
        {
            return Err(MembershipError::InvalidEvent);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum MembershipOperation {
    Join { member: Member },
    Promote { node_id: String },
    Remove { node_id: String },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommittedMembershipEvent {
    pub schema: String,
    pub trust_domain: String,
    pub event_id: [u8; 32],
    pub epoch: u64,
    pub committed_log_index: u64,
    pub operation: MembershipOperation,
}

impl CommittedMembershipEvent {
    pub fn new(
        trust_domain: impl Into<String>,
        event_id: [u8; 32],
        epoch: u64,
        committed_log_index: u64,
        operation: MembershipOperation,
    ) -> Self {
        Self {
            schema: MEMBERSHIP_EVENT_SCHEMA.to_owned(),
            trust_domain: trust_domain.into(),
            event_id,
            epoch,
            committed_log_index,
            operation,
        }
    }

    fn validate(&self) -> MembershipResult<()> {
        if self.schema != MEMBERSHIP_EVENT_SCHEMA
            || !valid_identifier(&self.trust_domain)
            || self.event_id == [0; 32]
            || self.epoch == 0
            || self.committed_log_index == 0
        {
            return Err(MembershipError::InvalidEvent);
        }
        match &self.operation {
            MembershipOperation::Join { member } => member.validate(),
            MembershipOperation::Promote { node_id } | MembershipOperation::Remove { node_id } => {
                if valid_identifier(node_id) {
                    Ok(())
                } else {
                    Err(MembershipError::InvalidEvent)
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplyOutcome {
    Applied,
    AlreadyApplied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AppliedEvent {
    event_id: [u8; 32],
    event_digest: [u8; 32],
    epoch: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotBody {
    schema: String,
    trust_domain: String,
    epoch: u64,
    committed_log_index: u64,
    max_members: usize,
    replay_window: usize,
    members: Vec<Member>,
    applied_events: Vec<AppliedEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotEnvelope {
    body: SnapshotBody,
    digest: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct MembershipState {
    policy: MembershipPolicy,
    epoch: u64,
    committed_log_index: u64,
    members: BTreeMap<String, Member>,
    applied_events: BTreeMap<[u8; 32], [u8; 32]>,
    replay_order: VecDeque<([u8; 32], u64)>,
}

impl MembershipState {
    pub fn new(policy: MembershipPolicy) -> Self {
        Self {
            policy,
            epoch: 0,
            committed_log_index: 0,
            members: BTreeMap::new(),
            applied_events: BTreeMap::new(),
            replay_order: VecDeque::new(),
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    pub fn member(&self, node_id: &str) -> Option<&Member> {
        self.members.get(node_id)
    }

    pub fn apply(&mut self, event: &CommittedMembershipEvent) -> MembershipResult<ApplyOutcome> {
        event.validate()?;
        if event.trust_domain != self.policy.trust_domain {
            return Err(MembershipError::WrongTrustDomain);
        }
        let event_digest = canonical_digest(event)?;
        if let Some(observed) = self.applied_events.get(&event.event_id) {
            return if observed == &event_digest {
                Ok(ApplyOutcome::AlreadyApplied)
            } else {
                Err(MembershipError::ReplayConflict)
            };
        }
        let expected_epoch = self
            .epoch
            .checked_add(1)
            .ok_or(MembershipError::ResourceExhausted)?;
        if event.epoch < expected_epoch || event.committed_log_index <= self.committed_log_index {
            return Err(MembershipError::StaleEpoch);
        }
        if event.epoch != expected_epoch {
            return Err(MembershipError::EpochGap);
        }

        match &event.operation {
            MembershipOperation::Join { member } => {
                if self.members.contains_key(&member.node_id) {
                    return Err(MembershipError::MemberAlreadyExists);
                }
                if self.members.len() >= self.policy.max_members {
                    return Err(MembershipError::ResourceExhausted);
                }
                self.members.insert(member.node_id.clone(), member.clone());
            }
            MembershipOperation::Promote { node_id } => {
                let candidate = self
                    .members
                    .get(node_id)
                    .ok_or(MembershipError::MemberNotFound)?;
                if candidate.role == MemberRole::Voter {
                    return Err(MembershipError::AlreadyVoter);
                }
                let key = candidate.guardian_control_public_key;
                if self.members.values().any(|member| {
                    member.node_id != *node_id
                        && member.role == MemberRole::Voter
                        && member.guardian_control_public_key == key
                }) {
                    return Err(MembershipError::DuplicateGuardianControlKey);
                }
                self.members.get_mut(node_id).expect("checked member").role = MemberRole::Voter;
            }
            MembershipOperation::Remove { node_id } => {
                if self.members.remove(node_id).is_none() {
                    return Err(MembershipError::MemberNotFound);
                }
            }
        }

        self.epoch = event.epoch;
        self.committed_log_index = event.committed_log_index;
        self.applied_events.insert(event.event_id, event_digest);
        self.replay_order.push_back((event.event_id, event.epoch));
        while self.replay_order.len() > self.policy.replay_window {
            if let Some((event_id, _)) = self.replay_order.pop_front() {
                self.applied_events.remove(&event_id);
            }
        }
        Ok(ApplyOutcome::Applied)
    }

    pub fn snapshot(&self) -> MembershipResult<Vec<u8>> {
        let body = SnapshotBody {
            schema: MEMBERSHIP_SNAPSHOT_SCHEMA.to_owned(),
            trust_domain: self.policy.trust_domain.clone(),
            epoch: self.epoch,
            committed_log_index: self.committed_log_index,
            max_members: self.policy.max_members,
            replay_window: self.policy.replay_window,
            members: self.members.values().cloned().collect(),
            applied_events: self
                .replay_order
                .iter()
                .map(|(event_id, epoch)| AppliedEvent {
                    event_id: *event_id,
                    event_digest: self.applied_events[event_id],
                    epoch: *epoch,
                })
                .collect(),
        };
        let digest = snapshot_digest(&body)?;
        let bytes = serde_jcs::to_vec(&SnapshotEnvelope { body, digest })
            .map_err(|_| MembershipError::Encoding)?;
        if bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(MembershipError::SnapshotTooLarge);
        }
        Ok(bytes)
    }

    pub fn restore(
        policy: MembershipPolicy,
        bytes: &[u8],
        trusted_commitment: [u8; 32],
    ) -> MembershipResult<Self> {
        if bytes.is_empty() || bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(MembershipError::SnapshotTooLarge);
        }
        let envelope: SnapshotEnvelope =
            serde_json::from_slice(bytes).map_err(|_| MembershipError::SnapshotCorrupt)?;
        if trusted_commitment == [0; 32]
            || envelope.digest != trusted_commitment
            || envelope.digest != snapshot_digest(&envelope.body)?
            || envelope.body.schema != MEMBERSHIP_SNAPSHOT_SCHEMA
            || envelope.body.trust_domain != policy.trust_domain
            || envelope.body.max_members != policy.max_members
            || envelope.body.replay_window != policy.replay_window
            || envelope.body.members.len() > policy.max_members
            || envelope.body.applied_events.len() > policy.replay_window
            || (envelope.body.epoch == 0) != (envelope.body.committed_log_index == 0)
        {
            return Err(MembershipError::SnapshotCorrupt);
        }

        let mut members = BTreeMap::new();
        let mut voter_keys = BTreeSet::new();
        for member in envelope.body.members {
            let original_role = member.role;
            let mut shape = member.clone();
            shape.role = MemberRole::NonVoting;
            shape
                .validate()
                .map_err(|_| MembershipError::SnapshotCorrupt)?;
            if original_role == MemberRole::Voter
                && !voter_keys.insert(member.guardian_control_public_key)
            {
                return Err(MembershipError::SnapshotCorrupt);
            }
            if members.insert(member.node_id.clone(), member).is_some() {
                return Err(MembershipError::SnapshotCorrupt);
            }
        }

        let mut applied_events = BTreeMap::new();
        let mut replay_order = VecDeque::new();
        let mut previous_epoch = 0;
        for applied in envelope.body.applied_events {
            if applied.event_id == [0; 32]
                || applied.event_digest == [0; 32]
                || applied.epoch == 0
                || applied.epoch <= previous_epoch
                || applied.epoch > envelope.body.epoch
                || applied_events
                    .insert(applied.event_id, applied.event_digest)
                    .is_some()
            {
                return Err(MembershipError::SnapshotCorrupt);
            }
            previous_epoch = applied.epoch;
            replay_order.push_back((applied.event_id, applied.epoch));
        }

        Ok(Self {
            policy,
            epoch: envelope.body.epoch,
            committed_log_index: envelope.body.committed_log_index,
            members,
            applied_events,
            replay_order,
        })
    }
}

pub fn committed_snapshot_digest(bytes: &[u8]) -> MembershipResult<[u8; 32]> {
    if bytes.is_empty() || bytes.len() > MAX_SNAPSHOT_BYTES {
        return Err(MembershipError::SnapshotTooLarge);
    }
    let envelope: SnapshotEnvelope =
        serde_json::from_slice(bytes).map_err(|_| MembershipError::SnapshotCorrupt)?;
    Ok(envelope.digest)
}

fn snapshot_policy_budget(max_members: usize, replay_window: usize) -> Option<usize> {
    SNAPSHOT_BASE_BUDGET
        .checked_add(max_members.checked_mul(SNAPSHOT_MEMBER_BUDGET)?)?
        .checked_add(replay_window.checked_mul(SNAPSHOT_REPLAY_ENTRY_BUDGET)?)
}

fn snapshot_digest(body: &SnapshotBody) -> MembershipResult<[u8; 32]> {
    let body = serde_jcs::to_vec(body).map_err(|_| MembershipError::Encoding)?;
    let mut hasher = Sha256::new();
    hasher.update(SNAPSHOT_DOMAIN);
    hasher.update(body);
    Ok(hasher.finalize().into())
}

fn canonical_digest<T: Serialize>(value: &T) -> MembershipResult<[u8; 32]> {
    let bytes = serde_jcs::to_vec(value).map_err(|_| MembershipError::Encoding)?;
    Ok(Sha256::digest(bytes).into())
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_IDENTIFIER_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':'))
}
