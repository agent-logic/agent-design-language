use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(not(test))]
use super::authority_store_adapters::{AuthorityBoundFencingStore, AuthorityBoundLeaseLedger};
use super::{
    fencing::{ActiveLeaseCheck, FenceCommit},
    integrated_serving_authority_snapshot::{
        IntegratedOutcome, IntegratedServingAuthoritySnapshotStore, IntegratedSnapshotReceipt,
    },
    lease::{
        verify_certificate, AuthorityApplication, AuthorityLedger, AuthorityMembership,
        LeasePolicy, OperationClass,
    },
    migration::{MigrationPhase, MigrationStore, SourceQuiescenceAuthority},
};
#[cfg(test)]
use super::{
    fencing::{FencingStore, AUTHORITY_BOUND_FENCING_ACCESS},
    lease::AUTHORITY_BOUND_LEASE_ACCESS,
};

#[cfg(not(test))]
type RecoveryLeaseAuthority = AuthorityBoundLeaseLedger;
#[cfg(test)]
type RecoveryLeaseAuthority = AuthorityLedger;
#[cfg(not(test))]
type RecoveryFencingAuthority = AuthorityBoundFencingStore;
#[cfg(test)]
type RecoveryFencingAuthority = FencingStore;

fn recovery_lease(
    ledger: &RecoveryLeaseAuthority,
    lineage_id: &[u8],
) -> RecoveryResult<Option<super::lease::LeaseState>> {
    #[cfg(not(test))]
    return ledger
        .lease(lineage_id)
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return Ok(ledger.lease(lineage_id).cloned());
}

fn recovery_applied_log_index(ledger: &RecoveryLeaseAuthority) -> RecoveryResult<u64> {
    #[cfg(not(test))]
    return ledger
        .applied_log_index()
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return Ok(ledger.applied_log_index());
}

fn recovery_apply(
    ledger: &mut RecoveryLeaseAuthority,
    certificate_bytes: &[u8],
    membership: &AuthorityMembership,
    application: AuthorityApplication<'_>,
) -> RecoveryResult<super::lease::LeaseState> {
    #[cfg(not(test))]
    return ledger
        .apply(certificate_bytes, membership, application)
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return ledger
        .apply(
            &AUTHORITY_BOUND_LEASE_ACCESS,
            certificate_bytes,
            membership,
            application,
        )
        .cloned()
        .map_err(|_| RecoveryError::AuthorityRejected);
}

fn recovery_floor(
    fencing: &RecoveryFencingAuthority,
    lineage_id: &[u8],
) -> RecoveryResult<Option<super::fencing::FenceReceipt>> {
    #[cfg(not(test))]
    return fencing
        .floor(lineage_id)
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return Ok(fencing.floor(lineage_id).cloned());
}

fn recovery_fence_commit(
    fencing: &mut RecoveryFencingAuthority,
    request: FenceCommit<'_>,
) -> RecoveryResult<super::fencing::FenceReceipt> {
    #[cfg(not(test))]
    return fencing
        .commit(request)
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return fencing
        .commit(&AUTHORITY_BOUND_FENCING_ACCESS, request)
        .map_err(|_| RecoveryError::AuthorityRejected);
}

fn recovery_authorize_active(
    fencing: &RecoveryFencingAuthority,
    check: ActiveLeaseCheck<'_>,
) -> RecoveryResult<()> {
    #[cfg(not(test))]
    return fencing
        .authorize_active_lease(check)
        .map_err(|_| RecoveryError::AuthorityRejected);
    #[cfg(test)]
    return fencing
        .authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)
        .map_err(|_| RecoveryError::AuthorityRejected);
}

pub const RECOVERY_STATE_SCHEMA: &str = "adl.distributed.recovery_state.v1";
const STATE_FILE: &str = "recovery-state.json";
const STATE_BACKUP_FILE: &str = ".recovery-state.backup";
const STATE_LOCK_FILE: &str = ".recovery-state.lock";
const STATE_TEMP_FILE: &str = ".recovery-state.tmp";
const MAX_ABSOLUTE_RECORDS: usize = 4096;
const MAX_ABSOLUTE_HISTORY: usize = 24;
const MAX_ABSOLUTE_IDENTITY_BYTES: usize = 256;
const MAX_JOURNAL_BYTES: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoveryError {
    InvalidPolicy,
    UnsafeStatePath,
    StateMissing,
    StateCorrupt,
    Rollback,
    DurabilityFailure,
    ResourceExhausted,
    NotFound,
    InvalidTransition,
    ReplayMismatch,
    WrongTrustDomain,
    MigrationMismatch,
    QuorumRequired,
    AuthorityRejected,
    FenceRejected,
    SafetyWindow,
    TimedOut,
    OperatorRequired,
    RevisionDrift,
    ServingTransferRejected,
}

impl RecoveryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::UnsafeStatePath => "unsafe_state_path",
            Self::StateMissing => "state_missing",
            Self::StateCorrupt => "state_corrupt",
            Self::Rollback => "rollback",
            Self::DurabilityFailure => "durability_failure",
            Self::ResourceExhausted => "resource_exhausted",
            Self::NotFound => "not_found",
            Self::InvalidTransition => "invalid_transition",
            Self::ReplayMismatch => "replay_mismatch",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::MigrationMismatch => "migration_mismatch",
            Self::QuorumRequired => "quorum_required",
            Self::AuthorityRejected => "authority_rejected",
            Self::FenceRejected => "fence_rejected",
            Self::SafetyWindow => "safety_window",
            Self::TimedOut => "timed_out",
            Self::OperatorRequired => "operator_required",
            Self::RevisionDrift => "revision_drift",
            Self::ServingTransferRejected => "serving_transfer_rejected",
        }
    }
}

impl fmt::Display for RecoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for RecoveryError {}
pub type RecoveryResult<T> = Result<T, RecoveryError>;

#[derive(Clone, Debug)]
pub struct RecoveryPolicy {
    pub trust_domain: String,
    pub max_records: usize,
    pub max_history_per_record: usize,
    pub max_local_histories: usize,
    pub max_identity_bytes: usize,
    pub max_state_bytes: usize,
    pub max_timeout_millis: u64,
}

impl RecoveryPolicy {
    pub fn new(trust_domain: impl Into<String>) -> RecoveryResult<Self> {
        Self::with_bounds(trust_domain, 256, 16, 32, 128, 4 * 1024 * 1024, 3_600_000)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_bounds(
        trust_domain: impl Into<String>,
        max_records: usize,
        max_history_per_record: usize,
        max_local_histories: usize,
        max_identity_bytes: usize,
        max_state_bytes: usize,
        max_timeout_millis: u64,
    ) -> RecoveryResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_records,
            max_history_per_record,
            max_local_histories,
            max_identity_bytes,
            max_state_bytes,
            max_timeout_millis,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> RecoveryResult<()> {
        if !valid_text(&self.trust_domain, self.max_identity_bytes)
            || self.max_records == 0
            || self.max_records > MAX_ABSOLUTE_RECORDS
            || !(10..=MAX_ABSOLUTE_HISTORY).contains(&self.max_history_per_record)
            || self.max_local_histories == 0
            || self.max_local_histories > 1024
            || self.max_identity_bytes == 0
            || self.max_identity_bytes > MAX_ABSOLUTE_IDENTITY_BYTES
            || !(1024..=16 * 1024 * 1024).contains(&self.max_state_bytes)
            || self.max_timeout_millis == 0
            || self.max_timeout_millis > 86_400_000
        {
            return Err(RecoveryError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryTime {
    pub unix_seconds: i64,
    pub unix_nanos: u32,
    pub elapsed_millis: u64,
    pub clock_uncertainty_millis: u64,
}

pub trait RecoveryClock: fmt::Debug + Send + Sync {
    fn now(&self) -> RecoveryResult<RecoveryTime>;
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPhase {
    Assessing,
    Planned,
    CleanupPending,
    TargetDiscarded,
    RollbackPending,
    FencePending,
    Fenced,
    OperatorRequired,
    ActivatePending,
    Restored,
    CommitPending,
    Committed,
    ServingTransferPending,
    ServingTransferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalHistory {
    pub node_id: Vec<u8>,
    pub guardian_id: Vec<u8>,
    pub claimed_epoch: u64,
    pub claimed_log_index: u64,
    pub claimed_owner: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryRequest {
    pub recovery_id: Vec<u8>,
    pub migration_id: Vec<u8>,
    pub trust_domain: String,
    pub timeout_millis: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetCleanupRequest<'a> {
    pub recovery_id: &'a [u8],
    pub migration_id: &'a [u8],
    pub target_node_id: &'a [u8],
    pub target_guardian_id: &'a [u8],
    pub transfer_id: Option<&'a [u8]>,
    pub content_sha256: Option<[u8; 32]>,
    pub remaining_timeout_millis: u64,
}

pub trait RecoveryTargetAuthority: fmt::Debug + Send + Sync {
    fn discard_incomplete(&self, request: TargetCleanupRequest<'_>) -> RecoveryResult<Vec<u8>>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryEvidence {
    pub phase: RecoveryPhase,
    pub evidence_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryRecord {
    pub recovery_id: Vec<u8>,
    pub migration_id: Vec<u8>,
    pub migration_record_sha256: [u8; 32],
    pub trust_domain: String,
    pub lineage_id: Vec<u8>,
    pub source_node_id: Vec<u8>,
    pub source_guardian_id: Vec<u8>,
    pub target_node_id: Vec<u8>,
    pub target_guardian_id: Vec<u8>,
    pub target_transfer_id: Option<Vec<u8>>,
    pub target_content_sha256: Option<[u8; 32]>,
    pub target_cleanup_required: bool,
    pub target_cleanup_receipt_sha256: Option<[u8; 32]>,
    pub observed_migration_phase: MigrationPhase,
    pub local_histories_sha256: [u8; 32],
    pub committed_prefix_sha256: Option<[u8; 32]>,
    pub committed_prefix_epoch: Option<u64>,
    pub committed_prefix_log_index: Option<u64>,
    pub committed_prefix_voter_generation: Option<u64>,
    pub committed_prefix_certificate_sha256: Option<[u8; 32]>,
    pub fence_epoch: Option<u64>,
    pub fence_log_index: Option<u64>,
    pub fence_certificate_sha256: Option<[u8; 32]>,
    pub started_at_millis: u64,
    pub deadline_millis: u64,
    pub phase: RecoveryPhase,
    pub owner_node_id: Option<Vec<u8>>,
    pub owner_guardian_id: Option<Vec<u8>>,
    pub owner_epoch: Option<u64>,
    pub committed_log_index: Option<u64>,
    pub authority_certificate_sha256: Option<[u8; 32]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_operation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_input_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_result_state_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_receipt_sha256: Option<String>,
    pub history: Vec<RecoveryEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryCheckpoint {
    pub generation: u64,
    pub state_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryAuthorityRevision {
    checkpoint_generation: u64,
    state_sha256: [u8; 32],
}

impl RecoveryAuthorityRevision {
    pub fn checkpoint_generation(&self) -> u64 {
        self.checkpoint_generation
    }

    pub fn state_sha256(&self) -> [u8; 32] {
        self.state_sha256
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoverySnapshotReason {
    AssessingInterruptedMigration,
    PlannedRecovery,
    TargetCleanupRequired,
    TargetDiscarded,
    RollbackRequired,
    FencingRequired,
    Fenced,
    OperatorRequired,
    ActivationRequired,
    Restored,
    CommitRequired,
    Committed,
    ServingTransferRequired,
    ServingTransferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedRecoveryRow {
    recovery_ref: String,
    migration_ref: String,
    lineage_ref: String,
    source_node_ref: String,
    source_guardian_ref: String,
    target_node_ref: String,
    target_guardian_ref: String,
    owner_node_ref: Option<String>,
    owner_guardian_ref: Option<String>,
    phase: RecoveryPhase,
    reason: RecoverySnapshotReason,
    operator_required: bool,
}

impl RedactedRecoveryRow {
    pub fn recovery_ref(&self) -> &str {
        &self.recovery_ref
    }

    pub fn migration_ref(&self) -> &str {
        &self.migration_ref
    }

    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }

    pub fn source_node_ref(&self) -> &str {
        &self.source_node_ref
    }

    pub fn source_guardian_ref(&self) -> &str {
        &self.source_guardian_ref
    }

    pub fn target_node_ref(&self) -> &str {
        &self.target_node_ref
    }

    pub fn target_guardian_ref(&self) -> &str {
        &self.target_guardian_ref
    }

    pub fn owner_node_ref(&self) -> Option<&str> {
        self.owner_node_ref.as_deref()
    }

    pub fn owner_guardian_ref(&self) -> Option<&str> {
        self.owner_guardian_ref.as_deref()
    }

    pub fn phase(&self) -> RecoveryPhase {
        self.phase
    }

    pub fn reason(&self) -> RecoverySnapshotReason {
        self.reason
    }

    pub fn operator_required(&self) -> bool {
        self.operator_required
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedRecoverySnapshot {
    trust_domain: String,
    revision: RecoveryAuthorityRevision,
    rows: Vec<RedactedRecoveryRow>,
}

impl RedactedRecoverySnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn revision(&self) -> RecoveryAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedRecoveryRow> {
        self.rows.iter()
    }
}

pub trait RecoveryCheckpointAuthority: fmt::Debug + Send + Sync {
    fn current(&self) -> RecoveryResult<Option<RecoveryCheckpoint>>;
    fn compare_and_swap(
        &self,
        expected: Option<RecoveryCheckpoint>,
        next: RecoveryCheckpoint,
    ) -> RecoveryResult<()>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommitJournal {
    expected: Option<RecoveryCheckpoint>,
    next: RecoveryCheckpoint,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateBody {
    schema: String,
    generation: u64,
    records: Vec<RecoveryRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateEnvelope {
    body: StateBody,
    digest: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct RecoveryStore {
    root: PathBuf,
    policy: RecoveryPolicy,
    records: BTreeMap<Vec<u8>, RecoveryRecord>,
    generation: u64,
    state_sha256: [u8; 32],
    checkpoint_authority: Arc<dyn RecoveryCheckpointAuthority>,
    clock: Arc<dyn RecoveryClock>,
}

impl RecoveryStore {
    pub fn create(
        root: impl AsRef<Path>,
        policy: RecoveryPolicy,
        checkpoint_authority: Arc<dyn RecoveryCheckpointAuthority>,
        clock: Arc<dyn RecoveryClock>,
    ) -> RecoveryResult<Self> {
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        if root.join(STATE_FILE).exists() || root.join(STATE_LOCK_FILE).exists() {
            recover_interrupted_commit(
                &root,
                checkpoint_authority.as_ref(),
                policy.max_state_bytes,
            )?;
            if root.join(STATE_FILE).exists() {
                return Self::open(root, policy, checkpoint_authority, clock);
            }
        }
        let mut store = Self {
            root,
            policy,
            records: BTreeMap::new(),
            generation: 0,
            state_sha256: [0; 32],
            checkpoint_authority,
            clock,
        };
        let (bytes, checkpoint) = store.encode_records(0, &store.records)?;
        let mut lock = store.acquire_lock(CommitJournal {
            expected: None,
            next: checkpoint,
        })?;
        let result = write_atomic(&store.root, &bytes).and_then(|_| {
            store
                .checkpoint_authority
                .compare_and_swap(None, checkpoint)
        });
        if result.is_err() {
            drop(lock);
            recover_interrupted_commit(
                &store.root,
                store.checkpoint_authority.as_ref(),
                store.policy.max_state_bytes,
            )?;
            if store.checkpoint_authority.current()? != Some(checkpoint) {
                return Err(result.err().unwrap_or(RecoveryError::DurabilityFailure));
            }
        } else {
            clear_lock(&mut lock)?;
        }
        store.state_sha256 = checkpoint.state_sha256;
        Ok(store)
    }

    pub fn open(
        root: impl AsRef<Path>,
        policy: RecoveryPolicy,
        checkpoint_authority: Arc<dyn RecoveryCheckpointAuthority>,
        clock: Arc<dyn RecoveryClock>,
    ) -> RecoveryResult<Self> {
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        recover_interrupted_commit(&root, checkpoint_authority.as_ref(), policy.max_state_bytes)?;
        let state_path = root.join(STATE_FILE);
        let metadata = fs::symlink_metadata(&state_path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                RecoveryError::StateMissing
            } else {
                RecoveryError::UnsafeStatePath
            }
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(RecoveryError::UnsafeStatePath);
        }
        let bytes = read_bounded_regular(
            &state_path,
            policy.max_state_bytes,
            RecoveryError::StateMissing,
        )?;
        if bytes.is_empty() || bytes.len() > policy.max_state_bytes {
            return Err(RecoveryError::ResourceExhausted);
        }
        let envelope: StateEnvelope =
            serde_json::from_slice(&bytes).map_err(|_| RecoveryError::StateCorrupt)?;
        let body_bytes =
            serde_jcs::to_vec(&envelope.body).map_err(|_| RecoveryError::StateCorrupt)?;
        if envelope.body.schema != RECOVERY_STATE_SCHEMA
            || envelope.digest != sha256(&body_bytes)
            || serde_jcs::to_vec(&envelope).map_err(|_| RecoveryError::StateCorrupt)? != bytes
            || envelope.body.records.len() > policy.max_records
        {
            return Err(RecoveryError::StateCorrupt);
        }
        let records = collect_records(&policy, envelope.body.records)?;
        let checkpoint = checkpoint_authority
            .current()?
            .ok_or(RecoveryError::Rollback)?;
        let state_sha256 = sha256(&bytes);
        if checkpoint.generation != envelope.body.generation
            || checkpoint.state_sha256 != state_sha256
        {
            return Err(RecoveryError::Rollback);
        }
        Ok(Self {
            root,
            policy,
            records,
            generation: envelope.body.generation,
            state_sha256,
            checkpoint_authority,
            clock,
        })
    }

    pub fn checkpoint(&self) -> RecoveryCheckpoint {
        RecoveryCheckpoint {
            generation: self.generation,
            state_sha256: self.state_sha256,
        }
    }

    pub fn authority_revision(&self) -> RecoveryResult<RecoveryAuthorityRevision> {
        self.verify_current_state()?;
        Ok(RecoveryAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        })
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: RecoveryAuthorityRevision,
    ) -> RecoveryResult<RedactedRecoverySnapshot> {
        self.verify_current_state()?;
        let revision = RecoveryAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        };
        if revision != expected_revision {
            return Err(RecoveryError::RevisionDrift);
        }
        if self.records.len() > self.policy.max_records {
            return Err(RecoveryError::ResourceExhausted);
        }
        let rows = self
            .records
            .values()
            .map(|record| RedactedRecoveryRow {
                recovery_ref: projection_ref(b"recovery", &record.recovery_id),
                migration_ref: projection_ref(b"migration", &record.migration_id),
                lineage_ref: projection_ref(b"lineage", &record.lineage_id),
                source_node_ref: projection_ref(b"node", &record.source_node_id),
                source_guardian_ref: projection_ref(b"guardian", &record.source_guardian_id),
                target_node_ref: projection_ref(b"node", &record.target_node_id),
                target_guardian_ref: projection_ref(b"guardian", &record.target_guardian_id),
                owner_node_ref: record
                    .owner_node_id
                    .as_deref()
                    .map(|value| projection_ref(b"node", value)),
                owner_guardian_ref: record
                    .owner_guardian_id
                    .as_deref()
                    .map(|value| projection_ref(b"guardian", value)),
                phase: record.phase,
                reason: recovery_snapshot_reason(record.phase),
                operator_required: record.phase == RecoveryPhase::OperatorRequired,
            })
            .collect();
        Ok(RedactedRecoverySnapshot {
            trust_domain: self.policy.trust_domain.clone(),
            revision,
            rows,
        })
    }

    #[cfg(test)]
    pub(crate) fn seed_record_for_snapshot_test(
        &mut self,
        record: RecoveryRecord,
    ) -> RecoveryResult<()> {
        validate_record(&self.policy, &record)?;
        let mut prospective = self.records.clone();
        prospective.insert(record.recovery_id.clone(), record);
        self.commit_records(prospective)
    }

    pub fn record(&self, recovery_id: &[u8]) -> Option<&RecoveryRecord> {
        self.records.get(recovery_id)
    }

    pub fn begin(
        &mut self,
        request: RecoveryRequest,
        migration: &MigrationStore,
        local_histories: &[LocalHistory],
    ) -> RecoveryResult<RecoveryRecord> {
        validate_request(&self.policy, &request, local_histories)?;
        let migration_record = migration
            .record(&request.migration_id)
            .ok_or(RecoveryError::MigrationMismatch)?;
        if request.trust_domain != migration_record.trust_domain {
            return Err(RecoveryError::WrongTrustDomain);
        }
        let migration_bytes =
            serde_jcs::to_vec(migration_record).map_err(|_| RecoveryError::StateCorrupt)?;
        let migration_record_sha256 = sha256(&migration_bytes);
        let local_histories_sha256 = local_histories_digest(local_histories);
        let request_sha256 = sha256_many(&[
            b"ADL-RECOVERY-BEGIN-V1\0",
            &request.recovery_id,
            &request.migration_id,
            request.trust_domain.as_bytes(),
            &request.timeout_millis.to_be_bytes(),
            &migration_record_sha256,
            &local_histories_sha256,
        ]);
        if let Some(existing) = self.records.get(&request.recovery_id) {
            return exact_retry(existing, existing.phase, request_sha256);
        }
        if self.records.len() >= self.policy.max_records {
            return Err(RecoveryError::ResourceExhausted);
        }
        let started_at_millis = self.clock.now()?.elapsed_millis;
        let deadline_millis = started_at_millis
            .checked_add(request.timeout_millis)
            .ok_or(RecoveryError::ResourceExhausted)?;
        let record = RecoveryRecord {
            recovery_id: request.recovery_id.clone(),
            migration_id: request.migration_id,
            migration_record_sha256,
            trust_domain: request.trust_domain,
            lineage_id: migration_record.lineage_id.clone(),
            source_node_id: migration_record.source_node_id.clone(),
            source_guardian_id: migration_record.source_guardian_id.clone(),
            target_node_id: migration_record.target_node_id.clone(),
            target_guardian_id: migration_record.target_guardian_id.clone(),
            target_transfer_id: migration_record.transfer_id.clone(),
            target_content_sha256: migration_record.snapshot_content_sha256,
            target_cleanup_required: matches!(
                migration_record.phase,
                MigrationPhase::Quiesced
                    | MigrationPhase::Checkpointed
                    | MigrationPhase::Transferred
                    | MigrationPhase::Validated
            ),
            target_cleanup_receipt_sha256: None,
            observed_migration_phase: migration_record.phase,
            local_histories_sha256,
            committed_prefix_sha256: None,
            committed_prefix_epoch: None,
            committed_prefix_log_index: None,
            committed_prefix_voter_generation: None,
            committed_prefix_certificate_sha256: None,
            fence_epoch: None,
            fence_log_index: None,
            fence_certificate_sha256: None,
            started_at_millis,
            deadline_millis,
            phase: RecoveryPhase::Assessing,
            owner_node_id: None,
            owner_guardian_id: None,
            owner_epoch: None,
            committed_log_index: None,
            authority_certificate_sha256: None,
            serving_operation_ref: None,
            serving_input_sha256: None,
            serving_result_state_sha256: None,
            serving_receipt_sha256: None,
            history: vec![RecoveryEvidence {
                phase: RecoveryPhase::Assessing,
                evidence_sha256: request_sha256,
            }],
        };
        self.insert_new(record.clone())?;
        Ok(record)
    }

    pub fn select_committed_prefix(
        &mut self,
        recovery_id: &[u8],
        candidate_snapshots: &[Vec<u8>],
        lease_policy: &LeasePolicy,
        membership: &AuthorityMembership,
    ) -> RecoveryResult<RecoveryRecord> {
        let current = self.required_record(recovery_id)?.clone();
        self.ensure_live(&current)?;
        if candidate_snapshots.is_empty()
            || candidate_snapshots.len() > self.policy.max_local_histories
        {
            return Err(RecoveryError::ResourceExhausted);
        }
        if membership.trust_domain_id != current.trust_domain.as_bytes() {
            return Err(RecoveryError::WrongTrustDomain);
        }
        let now = self.clock.now()?;
        if now.elapsed_millis < current.started_at_millis
            || now.elapsed_millis >= current.deadline_millis
        {
            return Err(RecoveryError::TimedOut);
        }
        let mut total = 0usize;
        let mut valid = BTreeMap::<[u8; 32], (u64, u64, u64, [u8; 32])>::new();
        for snapshot in candidate_snapshots {
            total = total
                .checked_add(snapshot.len())
                .ok_or(RecoveryError::ResourceExhausted)?;
            if snapshot.is_empty() || total > self.policy.max_state_bytes {
                return Err(RecoveryError::ResourceExhausted);
            }
            let Ok(ledger) = AuthorityLedger::restore(
                lease_policy.clone(),
                snapshot,
                membership,
                now.unix_seconds,
            ) else {
                continue;
            };
            let Some(lease) = ledger.lease(&current.lineage_id) else {
                continue;
            };
            if !is_candidate(&current, &lease.holder_node_id, &lease.holder_guardian_id) {
                continue;
            }
            let Ok(verified) =
                verify_certificate(&lease.certificate_bytes, membership, now.unix_seconds)
            else {
                continue;
            };
            let digest = sha256(snapshot);
            valid.insert(
                digest,
                (
                    lease.epoch,
                    ledger.applied_log_index(),
                    verified.body.voter_set_generation,
                    sha256(&lease.certificate_bytes),
                ),
            );
        }
        let candidate_set_sha256 = sha256_many(
            &valid
                .keys()
                .map(|digest| digest.as_slice())
                .collect::<Vec<_>>(),
        );
        if valid.len() != 1 {
            let evidence = sha256_many(&[
                b"ADL-RECOVERY-DIVERGENT-PREFIX-V1\0",
                recovery_id,
                &candidate_set_sha256,
                &(valid.len() as u64).to_be_bytes(),
            ]);
            return self.advance(
                recovery_id,
                &[RecoveryPhase::Assessing, RecoveryPhase::Fenced],
                RecoveryPhase::OperatorRequired,
                evidence,
                |_| {},
            );
        }
        let (digest, (epoch, log_index, generation, certificate_sha256)) = valid
            .into_iter()
            .next()
            .ok_or(RecoveryError::OperatorRequired)?;
        let evidence = sha256_many(&[
            b"ADL-RECOVERY-COMMITTED-PREFIX-V1\0",
            recovery_id,
            &digest,
            &epoch.to_be_bytes(),
            &log_index.to_be_bytes(),
            &generation.to_be_bytes(),
            &certificate_sha256,
        ]);
        self.advance(
            recovery_id,
            &[RecoveryPhase::Assessing],
            RecoveryPhase::Planned,
            evidence,
            |record| {
                record.committed_prefix_sha256 = Some(digest);
                record.committed_prefix_epoch = Some(epoch);
                record.committed_prefix_log_index = Some(log_index);
                record.committed_prefix_voter_generation = Some(generation);
                record.committed_prefix_certificate_sha256 = Some(certificate_sha256);
            },
        )
    }

    pub fn require_operator(
        &mut self,
        recovery_id: &[u8],
        reason: &[u8],
    ) -> RecoveryResult<RecoveryRecord> {
        let current = self.required_record(recovery_id)?.clone();
        self.ensure_live(&current)?;
        if reason.is_empty() || reason.len() > self.policy.max_identity_bytes {
            return Err(RecoveryError::ResourceExhausted);
        }
        let evidence = sha256_many(&[b"ADL-RECOVERY-OPERATOR-V1\0", recovery_id, reason]);
        self.advance(
            recovery_id,
            &[
                RecoveryPhase::Assessing,
                RecoveryPhase::Planned,
                RecoveryPhase::TargetDiscarded,
                RecoveryPhase::Fenced,
            ],
            RecoveryPhase::OperatorRequired,
            evidence,
            |_| {},
        )
    }

    pub fn discard_incomplete_target(
        &mut self,
        recovery_id: &[u8],
        authority: &dyn RecoveryTargetAuthority,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        let mut remaining_timeout_millis =
            self.ensure_operation_live(&current, RecoveryPhase::CleanupPending)?;
        if !current.target_cleanup_required {
            return Err(RecoveryError::InvalidTransition);
        }
        let intent = sha256_many(&[
            b"ADL-RECOVERY-TARGET-DISCARD-INTENT-V1\0",
            recovery_id,
            &current.migration_record_sha256,
            &current.target_node_id,
            &current.target_guardian_id,
            current.target_transfer_id.as_deref().unwrap_or_default(),
            current
                .target_content_sha256
                .as_ref()
                .map_or(&[][..], |value| value.as_slice()),
        ]);
        if current.phase == RecoveryPhase::Planned {
            self.advance(
                recovery_id,
                &[RecoveryPhase::Planned],
                RecoveryPhase::CleanupPending,
                intent,
                |_| {},
            )?;
            current = self.required_record(recovery_id)?.clone();
            remaining_timeout_millis =
                self.ensure_operation_live(&current, RecoveryPhase::CleanupPending)?;
        } else if current.phase == RecoveryPhase::CleanupPending {
            ensure_last_evidence(&current, RecoveryPhase::CleanupPending, intent)?;
        } else if current.phase != RecoveryPhase::TargetDiscarded {
            return Err(RecoveryError::InvalidTransition);
        }
        let receipt = authority.discard_incomplete(TargetCleanupRequest {
            recovery_id,
            migration_id: &current.migration_id,
            target_node_id: &current.target_node_id,
            target_guardian_id: &current.target_guardian_id,
            transfer_id: current.target_transfer_id.as_deref(),
            content_sha256: current.target_content_sha256,
            remaining_timeout_millis,
        })?;
        if receipt.is_empty() || receipt.len() > self.policy.max_state_bytes / 4 {
            return Err(RecoveryError::ResourceExhausted);
        }
        self.ensure_post_action_live(recovery_id, RecoveryPhase::CleanupPending)?;
        let receipt_sha256 = sha256_many(&[
            b"ADL-RECOVERY-TARGET-DISCARD-V1\0",
            recovery_id,
            &current.migration_record_sha256,
            &receipt,
        ]);
        if current.phase == RecoveryPhase::TargetDiscarded {
            return if current.target_cleanup_receipt_sha256 == Some(receipt_sha256) {
                Ok(current)
            } else {
                Err(RecoveryError::ReplayMismatch)
            };
        }
        self.advance(
            recovery_id,
            &[RecoveryPhase::CleanupPending],
            RecoveryPhase::TargetDiscarded,
            receipt_sha256,
            |record| record.target_cleanup_receipt_sha256 = Some(receipt_sha256),
        )
    }

    pub fn rollback_pre_fence(
        &mut self,
        recovery_id: &[u8],
        migration: &mut MigrationStore,
        authority: &dyn SourceQuiescenceAuthority,
        fencing: &RecoveryFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        self.ensure_operation_live(&current, RecoveryPhase::RollbackPending)?;
        if matches!(
            current.observed_migration_phase,
            MigrationPhase::Fenced | MigrationPhase::Activated | MigrationPhase::Committed
        ) {
            return Err(RecoveryError::InvalidTransition);
        }
        let expected_phase = if current.target_cleanup_required {
            RecoveryPhase::TargetDiscarded
        } else {
            RecoveryPhase::Planned
        };
        if current.phase != expected_phase
            && current.phase != RecoveryPhase::RollbackPending
            && current.phase != RecoveryPhase::Restored
        {
            return Err(RecoveryError::InvalidTransition);
        }
        validate_source_check(&current, &source_check)?;
        validate_selected_lease(&current, source_check.lease, source_check.applied_log_index)?;
        self.validate_active_check_time(&source_check)?;
        recovery_authorize_active(fencing, copy_active_check(&source_check))?;
        let intent = sha256_many(&[
            b"ADL-RECOVERY-ROLLBACK-INTENT-V1\0",
            recovery_id,
            &current.migration_record_sha256,
            &sha256(&source_check.lease.certificate_bytes),
            &source_check.applied_log_index.to_be_bytes(),
        ]);
        if current.phase == expected_phase {
            self.advance(
                recovery_id,
                &[expected_phase],
                RecoveryPhase::RollbackPending,
                intent,
                |_| {},
            )?;
            current = self.required_record(recovery_id)?.clone();
        } else if current.phase == RecoveryPhase::RollbackPending {
            ensure_last_evidence(&current, RecoveryPhase::RollbackPending, intent)?;
        }
        let aborted = migration
            .abort_before_fence(
                &current.migration_id,
                authority,
                fencing,
                copy_active_check(&source_check),
            )
            .map_err(|_| RecoveryError::AuthorityRejected)?;
        if aborted.phase != MigrationPhase::Aborted
            || !aborted.source_authoritative
            || aborted.target_authoritative
        {
            return Err(RecoveryError::MigrationMismatch);
        }
        self.ensure_post_action_live(recovery_id, RecoveryPhase::RollbackPending)?;
        let certificate_sha256 = sha256(&source_check.lease.certificate_bytes);
        let evidence = sha256_many(&[
            b"ADL-RECOVERY-PRE-FENCE-ROLLBACK-V1\0",
            &current.migration_record_sha256,
            &certificate_sha256,
            &source_check.applied_log_index.to_be_bytes(),
        ]);
        self.restore_record(
            recovery_id,
            &[RecoveryPhase::RollbackPending],
            evidence,
            &current.source_node_id,
            &current.source_guardian_id,
            source_check.lease.epoch,
            source_check.applied_log_index,
            certificate_sha256,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fence_ambiguous(
        &mut self,
        recovery_id: &[u8],
        fence_request_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut RecoveryLeaseAuthority,
        fencing: &mut RecoveryFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        self.ensure_operation_live(&current, RecoveryPhase::FencePending)?;
        self.validate_application_time(&current, &application)?;
        if !valid_bytes(fence_request_id, self.policy.max_identity_bytes) {
            return Err(RecoveryError::ResourceExhausted);
        }
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)
                .map_err(|_| RecoveryError::QuorumRequired)?;
        let body = verified.body;
        if body.operation_class != OperationClass::Fence as u32
            || body.trust_domain_id != current.trust_domain.as_bytes()
            || body.lineage_id != current.lineage_id
            || body.committed_log_index != membership.committed_log_index
            || !is_candidate(&current, &body.holder_node_id, &body.holder_guardian_id)
        {
            return Err(RecoveryError::FenceRejected);
        }
        let certificate_sha256 = sha256(certificate_bytes);
        let intent = sha256_many(&[
            b"ADL-RECOVERY-FENCE-INTENT-V1\0",
            fence_request_id,
            &certificate_sha256,
            &body.epoch.to_be_bytes(),
            &body.committed_log_index.to_be_bytes(),
        ]);
        if current.phase == RecoveryPhase::Planned {
            self.advance(
                recovery_id,
                &[RecoveryPhase::Planned],
                RecoveryPhase::FencePending,
                intent,
                |record| {
                    record.fence_epoch = Some(body.epoch);
                    record.fence_log_index = Some(body.committed_log_index);
                    record.fence_certificate_sha256 = Some(certificate_sha256);
                },
            )?;
            current = self.required_record(recovery_id)?.clone();
        } else if current.phase == RecoveryPhase::FencePending {
            ensure_last_evidence(&current, RecoveryPhase::FencePending, intent)?;
        } else if current.phase != RecoveryPhase::Fenced {
            return Err(RecoveryError::InvalidTransition);
        }
        let existing = recovery_floor(fencing, &current.lineage_id)?;
        let exact_floor = existing.as_ref().is_some_and(|floor| {
            floor.request_id == fence_request_id
                && floor.certificate_sha256 == certificate_sha256
                && floor.epoch == body.epoch
                && floor.committed_log_index == body.committed_log_index
                && floor.operation_class == OperationClass::Fence as u32
        });
        if !exact_floor {
            let lease = recovery_lease(ledger, &current.lineage_id)?
                .filter(|lease| !lease.revoked)
                .ok_or(RecoveryError::AuthorityRejected)?;
            if let Some(floor) = existing.as_ref() {
                validate_active_successor(
                    &current,
                    &lease,
                    recovery_applied_log_index(ledger)?,
                    floor,
                )?;
            } else {
                validate_selected_lease(&current, &lease, recovery_applied_log_index(ledger)?)?;
            }
            recovery_fence_commit(
                fencing,
                FenceCommit {
                    request_id: fence_request_id,
                    certificate_bytes,
                    membership: Some(membership),
                    current_lease: &lease,
                    now_unix_seconds: application.now_unix_seconds,
                },
            )
            .map_err(|_| RecoveryError::FenceRejected)?;
        }
        let floor =
            recovery_floor(fencing, &current.lineage_id)?.ok_or(RecoveryError::FenceRejected)?;
        if floor.request_id != fence_request_id
            || floor.certificate_sha256 != certificate_sha256
            || floor.epoch != body.epoch
            || floor.committed_log_index != body.committed_log_index
            || floor.operation_class != OperationClass::Fence as u32
        {
            return Err(RecoveryError::ReplayMismatch);
        }
        let applied = recovery_lease(ledger, &current.lineage_id)?.is_some_and(|lease| {
            lease.revoked
                && lease.epoch == body.epoch
                && lease.committed_log_index == body.committed_log_index
                && lease.certificate_bytes == certificate_bytes
        });
        if !applied {
            let prior = recovery_lease(ledger, &current.lineage_id)?
                .ok_or(RecoveryError::AuthorityRejected)?;
            validate_selected_lease(&current, &prior, recovery_applied_log_index(ledger)?)?;
            recovery_apply(ledger, certificate_bytes, membership, application)
                .map_err(|_| RecoveryError::FenceRejected)?;
        }
        let lease =
            recovery_lease(ledger, &current.lineage_id)?.ok_or(RecoveryError::FenceRejected)?;
        if !lease.revoked || lease.certificate_bytes != certificate_bytes {
            return Err(RecoveryError::FenceRejected);
        }
        self.ensure_post_action_live(recovery_id, RecoveryPhase::FencePending)?;
        let evidence = sha256_many(&[b"ADL-RECOVERY-FENCE-COMPLETE-V1\0", &intent]);
        self.advance(
            recovery_id,
            &[RecoveryPhase::FencePending],
            RecoveryPhase::Fenced,
            evidence,
            |record| {
                clear_owner(record);
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn restore_quorum_owner(
        &mut self,
        recovery_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut RecoveryLeaseAuthority,
        fencing: &RecoveryFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        self.ensure_operation_live(&current, RecoveryPhase::ActivatePending)?;
        self.validate_application_time(&current, &application)?;
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)
                .map_err(|_| RecoveryError::QuorumRequired)?;
        let body = verified.body;
        if body.operation_class != OperationClass::Activate as u32
            || body.trust_domain_id != current.trust_domain.as_bytes()
            || body.lineage_id != current.lineage_id
            || body.committed_log_index != membership.committed_log_index
            || !is_candidate(&current, &body.holder_node_id, &body.holder_guardian_id)
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        if recovery_floor(fencing, &current.lineage_id)?.is_some_and(|floor| {
            unix_millis(application.now_unix_seconds, application.now_unix_nanos)
                .is_none_or(|now| now < floor.safety_deadline_unix_millis)
        }) {
            return Err(RecoveryError::SafetyWindow);
        }
        let certificate_sha256 = sha256(certificate_bytes);
        let intent = sha256_many(&[
            b"ADL-RECOVERY-ACTIVATE-INTENT-V1\0",
            &certificate_sha256,
            &body.epoch.to_be_bytes(),
            &body.committed_log_index.to_be_bytes(),
        ]);
        if matches!(
            current.phase,
            RecoveryPhase::Fenced | RecoveryPhase::OperatorRequired
        ) {
            let expected = current.phase;
            self.advance(
                recovery_id,
                &[expected],
                RecoveryPhase::ActivatePending,
                intent,
                |_| {},
            )?;
            current = self.required_record(recovery_id)?.clone();
        } else if current.phase == RecoveryPhase::ActivatePending {
            ensure_last_evidence(&current, RecoveryPhase::ActivatePending, intent)?;
        } else if current.phase != RecoveryPhase::Restored {
            return Err(RecoveryError::InvalidTransition);
        }
        let applied = recovery_lease(ledger, &current.lineage_id)?.is_some_and(|lease| {
            !lease.revoked
                && lease.epoch == body.epoch
                && lease.holder_node_id == body.holder_node_id
                && lease.holder_guardian_id == body.holder_guardian_id
                && lease.committed_log_index == body.committed_log_index
                && lease.certificate_bytes == certificate_bytes
        });
        if !applied {
            let prior = recovery_lease(ledger, &current.lineage_id)?
                .ok_or(RecoveryError::AuthorityRejected)?;
            validate_activation_predecessor(&current, &prior, recovery_applied_log_index(ledger)?)?;
            recovery_apply(ledger, certificate_bytes, membership, application)?;
        }
        let lease =
            recovery_lease(ledger, &current.lineage_id)?.ok_or(RecoveryError::AuthorityRejected)?;
        recovery_authorize_active(
            fencing,
            ActiveLeaseCheck {
                membership: Some(membership),
                lease: &lease,
                applied_log_index: recovery_applied_log_index(ledger)?,
                now_unix_seconds: application.now_unix_seconds,
                now_unix_millis: unix_millis(
                    application.now_unix_seconds,
                    application.now_unix_nanos,
                )
                .ok_or(RecoveryError::AuthorityRejected)?,
                now_elapsed_millis: application.now_elapsed_millis,
                activation_proof: application.activation_proof,
            },
        )?;
        self.ensure_post_action_live(recovery_id, RecoveryPhase::ActivatePending)?;
        let evidence = sha256_many(&[b"ADL-RECOVERY-ACTIVATE-COMPLETE-V1\0", &intent]);
        self.restore_record(
            recovery_id,
            &[RecoveryPhase::ActivatePending],
            evidence,
            &body.holder_node_id,
            &body.holder_guardian_id,
            body.epoch,
            body.committed_log_index,
            certificate_sha256,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn commit_owner(
        &mut self,
        recovery_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut RecoveryLeaseAuthority,
        fencing: &RecoveryFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        self.ensure_operation_live(&current, RecoveryPhase::CommitPending)?;
        self.validate_application_time(&current, &application)?;
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)
                .map_err(|_| RecoveryError::QuorumRequired)?;
        let body = verified.body;
        if body.operation_class != OperationClass::OwnerCommit as u32
            || body.trust_domain_id != current.trust_domain.as_bytes()
            || body.lineage_id != current.lineage_id
            || body.committed_log_index != membership.committed_log_index
            || current.owner_node_id.as_deref() != Some(body.holder_node_id.as_slice())
            || current.owner_guardian_id.as_deref() != Some(body.holder_guardian_id.as_slice())
            || current.owner_epoch != Some(body.epoch)
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        let certificate_sha256 = sha256(certificate_bytes);
        let intent = sha256_many(&[
            b"ADL-RECOVERY-OWNER-COMMIT-INTENT-V1\0",
            &certificate_sha256,
            &body.epoch.to_be_bytes(),
            &body.committed_log_index.to_be_bytes(),
        ]);
        if current.phase == RecoveryPhase::Restored {
            self.advance(
                recovery_id,
                &[RecoveryPhase::Restored],
                RecoveryPhase::CommitPending,
                intent,
                |_| {},
            )?;
            current = self.required_record(recovery_id)?.clone();
        } else if current.phase == RecoveryPhase::CommitPending {
            ensure_last_evidence(&current, RecoveryPhase::CommitPending, intent)?;
        } else if current.phase != RecoveryPhase::Committed {
            return Err(RecoveryError::InvalidTransition);
        }
        let applied = recovery_lease(ledger, &current.lineage_id)?.is_some_and(|lease| {
            !lease.revoked
                && lease.epoch == body.epoch
                && lease.committed_log_index == body.committed_log_index
                && lease.certificate_bytes == certificate_bytes
        });
        if !applied {
            recovery_apply(ledger, certificate_bytes, membership, application)?;
        }
        let lease =
            recovery_lease(ledger, &current.lineage_id)?.ok_or(RecoveryError::AuthorityRejected)?;
        let floor_valid = recovery_floor(fencing, &current.lineage_id)?.map_or_else(
            || {
                current.fence_epoch.is_none()
                    && current.fence_log_index.is_none()
                    && current.fence_certificate_sha256.is_none()
                    && current
                        .committed_prefix_log_index
                        .is_some_and(|index| index < lease.committed_log_index)
            },
            |floor| {
                floor.epoch == lease.epoch && floor.committed_log_index < lease.committed_log_index
            },
        );
        if lease.revoked || lease.certificate_bytes != certificate_bytes || !floor_valid {
            return Err(RecoveryError::AuthorityRejected);
        }
        self.ensure_post_action_live(recovery_id, RecoveryPhase::CommitPending)?;
        let evidence = sha256_many(&[b"ADL-RECOVERY-OWNER-COMMIT-COMPLETE-V1\0", &intent]);
        self.advance(
            recovery_id,
            &[RecoveryPhase::CommitPending],
            RecoveryPhase::Committed,
            evidence,
            |record| {
                record.committed_log_index = Some(body.committed_log_index);
                record.authority_certificate_sha256 = Some(certificate_sha256);
            },
        )
    }

    pub fn transfer_serving_authority(
        &mut self,
        recovery_id: &[u8],
        serving_authority: &mut IntegratedServingAuthoritySnapshotStore,
    ) -> RecoveryResult<RecoveryRecord> {
        let mut current = self.required_record(recovery_id)?.clone();
        self.ensure_operation_live(&current, RecoveryPhase::ServingTransferPending)?;
        if current.phase == RecoveryPhase::ServingTransferred {
            validate_recovery_serving_record(&current, serving_authority)?;
            let evidence = recovery_serving_transfer_evidence(&current)?;
            return exact_retry(&current, RecoveryPhase::ServingTransferred, evidence);
        }
        if current.phase != RecoveryPhase::Committed
            && current.phase != RecoveryPhase::ServingTransferPending
        {
            return Err(RecoveryError::InvalidTransition);
        }
        let intent = sha256_many(&[
            b"ADL-RECOVERY-SERVING-TRANSFER-INTENT-V1\0",
            recovery_id,
            &current.migration_id,
            &current.migration_record_sha256,
            &current.authority_certificate_sha256.unwrap_or([0; 32]),
        ]);
        if current.phase == RecoveryPhase::Committed {
            self.advance(
                recovery_id,
                &[RecoveryPhase::Committed],
                RecoveryPhase::ServingTransferPending,
                intent,
                |_| {},
            )?;
            current = self.required_record(recovery_id)?.clone();
        } else {
            ensure_last_evidence(&current, RecoveryPhase::ServingTransferPending, intent)?;
        }
        let operation_ref = recovery_serving_operation_ref(recovery_id);
        let expected_lineage_ref = serving_lineage_ref(&current.lineage_id)?;
        let source_receipt = serving_authority
            .recoverable_latest_receipt()
            .map_err(|_| RecoveryError::ServingTransferRejected)?
            .ok_or(RecoveryError::ServingTransferRejected)?;
        validate_serving_source_lineage(&expected_lineage_ref, &source_receipt)?;
        let receipt = serving_authority
            .recover(&operation_ref)
            .map_err(|_| RecoveryError::ServingTransferRejected)?;
        let input_sha256 = receipt.input_sha256.clone();
        let result_state_sha256 = receipt.result_state_sha256.clone();
        let receipt_sha256 = receipt.receipt_sha256.clone();
        validate_serving_receipt_fields(
            &operation_ref,
            &input_sha256,
            &result_state_sha256,
            &receipt_sha256,
            &expected_lineage_ref,
            &receipt,
        )?;
        self.ensure_post_action_live(recovery_id, RecoveryPhase::ServingTransferPending)?;
        let evidence = recovery_serving_transfer_values_evidence(
            recovery_id,
            &current.migration_id,
            &operation_ref,
            &input_sha256,
            &result_state_sha256,
            &receipt_sha256,
        );
        self.advance(
            recovery_id,
            &[RecoveryPhase::ServingTransferPending],
            RecoveryPhase::ServingTransferred,
            evidence,
            |record| {
                record.serving_operation_ref = Some(operation_ref);
                record.serving_input_sha256 = Some(input_sha256);
                record.serving_result_state_sha256 = Some(result_state_sha256);
                record.serving_receipt_sha256 = Some(receipt_sha256);
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn restore_record(
        &mut self,
        recovery_id: &[u8],
        expected: &[RecoveryPhase],
        evidence: [u8; 32],
        node_id: &[u8],
        guardian_id: &[u8],
        epoch: u64,
        committed_log_index: u64,
        certificate_sha256: [u8; 32],
    ) -> RecoveryResult<RecoveryRecord> {
        self.advance(
            recovery_id,
            expected,
            RecoveryPhase::Restored,
            evidence,
            |record| {
                record.owner_node_id = Some(node_id.to_vec());
                record.owner_guardian_id = Some(guardian_id.to_vec());
                record.owner_epoch = Some(epoch);
                record.committed_log_index = Some(committed_log_index);
                record.authority_certificate_sha256 = Some(certificate_sha256);
            },
        )
    }

    fn required_record(&self, recovery_id: &[u8]) -> RecoveryResult<&RecoveryRecord> {
        self.records.get(recovery_id).ok_or(RecoveryError::NotFound)
    }

    fn ensure_live(&self, record: &RecoveryRecord) -> RecoveryResult<u64> {
        let now = self.clock.now()?.elapsed_millis;
        if now < record.started_at_millis || now >= record.deadline_millis {
            return Err(RecoveryError::TimedOut);
        }
        record
            .deadline_millis
            .checked_sub(now)
            .filter(|value| *value > 0)
            .ok_or(RecoveryError::TimedOut)
    }

    fn operation_deadline(
        &self,
        record: &RecoveryRecord,
        pending: RecoveryPhase,
    ) -> RecoveryResult<u64> {
        if record.phase == pending {
            record
                .deadline_millis
                .checked_add(self.policy.max_timeout_millis.min(60_000))
                .ok_or(RecoveryError::TimedOut)
        } else {
            Ok(record.deadline_millis)
        }
    }

    fn ensure_operation_live(
        &self,
        record: &RecoveryRecord,
        pending: RecoveryPhase,
    ) -> RecoveryResult<u64> {
        let now = self.clock.now()?.elapsed_millis;
        let deadline = self.operation_deadline(record, pending)?;
        if now < record.started_at_millis || now >= deadline {
            return Err(RecoveryError::TimedOut);
        }
        deadline.checked_sub(now).ok_or(RecoveryError::TimedOut)
    }

    fn validate_application_time(
        &self,
        record: &RecoveryRecord,
        application: &AuthorityApplication<'_>,
    ) -> RecoveryResult<()> {
        let now = self.clock.now()?;
        if application.now_unix_seconds != now.unix_seconds
            || application.now_unix_nanos != now.unix_nanos
            || application.now_elapsed_millis != now.elapsed_millis
            || application.clock_uncertainty_millis != now.clock_uncertainty_millis
            || now.elapsed_millis < record.started_at_millis
            || now.elapsed_millis >= self.operation_deadline(record, record.phase)?
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        Ok(())
    }

    fn ensure_post_action_live(
        &mut self,
        recovery_id: &[u8],
        pending: RecoveryPhase,
    ) -> RecoveryResult<()> {
        let current = self.required_record(recovery_id)?.clone();
        match self.ensure_operation_live(&current, pending) {
            Ok(_) => Ok(()),
            Err(RecoveryError::TimedOut) if current.phase == pending => {
                let evidence = sha256_many(&[
                    b"ADL-RECOVERY-POST-ACTION-TIMEOUT-V1\0",
                    recovery_id,
                    &[pending as u8],
                ]);
                self.advance(
                    recovery_id,
                    &[pending],
                    RecoveryPhase::OperatorRequired,
                    evidence,
                    |_| {},
                )?;
                Err(RecoveryError::TimedOut)
            }
            Err(error) => Err(error),
        }
    }

    fn validate_active_check_time(&self, check: &ActiveLeaseCheck<'_>) -> RecoveryResult<()> {
        let now = self.clock.now()?;
        if check.now_unix_seconds != now.unix_seconds
            || check.now_unix_millis
                != unix_millis(now.unix_seconds, now.unix_nanos)
                    .ok_or(RecoveryError::AuthorityRejected)?
            || check.now_elapsed_millis != now.elapsed_millis
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        Ok(())
    }

    fn insert_new(&mut self, record: RecoveryRecord) -> RecoveryResult<()> {
        let mut prospective = self.records.clone();
        if prospective
            .insert(record.recovery_id.clone(), record)
            .is_some()
        {
            return Err(RecoveryError::ReplayMismatch);
        }
        self.commit_records(prospective)
    }

    fn advance(
        &mut self,
        recovery_id: &[u8],
        expected: &[RecoveryPhase],
        target: RecoveryPhase,
        evidence_sha256: [u8; 32],
        update: impl FnOnce(&mut RecoveryRecord),
    ) -> RecoveryResult<RecoveryRecord> {
        let current = self.required_record(recovery_id)?.clone();
        if current.phase == target {
            return exact_retry(&current, target, evidence_sha256);
        }
        if !expected.contains(&current.phase) {
            return Err(RecoveryError::InvalidTransition);
        }
        let mut next = current;
        update(&mut next);
        next.phase = target;
        push_history(&self.policy, &mut next, target, evidence_sha256)?;
        let mut prospective = self.records.clone();
        prospective.insert(recovery_id.to_vec(), next.clone());
        self.commit_records(prospective)?;
        Ok(next)
    }

    fn commit_records(
        &mut self,
        prospective: BTreeMap<Vec<u8>, RecoveryRecord>,
    ) -> RecoveryResult<()> {
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(RecoveryError::ResourceExhausted)?;
        let (bytes, checkpoint) = self.encode_records(generation, &prospective)?;
        let expected = self.checkpoint();
        let mut lock = self.acquire_lock(CommitJournal {
            expected: Some(expected),
            next: checkpoint,
        })?;
        let result = self.verify_current_state().and_then(|_| {
            fs::rename(
                self.root.join(STATE_FILE),
                self.root.join(STATE_BACKUP_FILE),
            )
            .map_err(|_| RecoveryError::DurabilityFailure)?;
            write_atomic(&self.root, &bytes)?;
            self.checkpoint_authority
                .compare_and_swap(Some(expected), checkpoint)?;
            fs::remove_file(self.root.join(STATE_BACKUP_FILE))
                .map_err(|_| RecoveryError::DurabilityFailure)?;
            sync_directory(&self.root)
        });
        if result.is_err() {
            drop(lock);
            recover_interrupted_commit(
                &self.root,
                self.checkpoint_authority.as_ref(),
                self.policy.max_state_bytes,
            )?;
            if self.checkpoint_authority.current()? != Some(checkpoint) {
                return result;
            }
        } else {
            clear_lock(&mut lock)?;
        }
        self.records = prospective;
        self.generation = generation;
        self.state_sha256 = checkpoint.state_sha256;
        Ok(())
    }

    fn verify_current_state(&self) -> RecoveryResult<()> {
        let bytes = read_bounded_regular(
            &self.root.join(STATE_FILE),
            self.policy.max_state_bytes,
            RecoveryError::DurabilityFailure,
        )?;
        if sha256(&bytes) != self.state_sha256
            || self.checkpoint_authority.current()? != Some(self.checkpoint())
        {
            return Err(RecoveryError::Rollback);
        }
        Ok(())
    }

    fn persist_records(
        &self,
        generation: u64,
        records: &BTreeMap<Vec<u8>, RecoveryRecord>,
    ) -> RecoveryResult<RecoveryCheckpoint> {
        let (bytes, checkpoint) = self.encode_records(generation, records)?;
        write_atomic(&self.root, &bytes)?;
        Ok(checkpoint)
    }

    fn encode_records(
        &self,
        generation: u64,
        records: &BTreeMap<Vec<u8>, RecoveryRecord>,
    ) -> RecoveryResult<(Vec<u8>, RecoveryCheckpoint)> {
        if records.len() > self.policy.max_records
            || records
                .values()
                .any(|record| validate_record(&self.policy, record).is_err())
        {
            return Err(RecoveryError::ResourceExhausted);
        }
        let body = StateBody {
            schema: RECOVERY_STATE_SCHEMA.to_owned(),
            generation,
            records: records.values().cloned().collect(),
        };
        let body_bytes = serde_jcs::to_vec(&body).map_err(|_| RecoveryError::StateCorrupt)?;
        let envelope = StateEnvelope {
            body,
            digest: sha256(&body_bytes),
        };
        let bytes = serde_jcs::to_vec(&envelope).map_err(|_| RecoveryError::StateCorrupt)?;
        if bytes.len() > self.policy.max_state_bytes {
            return Err(RecoveryError::ResourceExhausted);
        }
        let checkpoint = RecoveryCheckpoint {
            generation,
            state_sha256: sha256(&bytes),
        };
        Ok((bytes, checkpoint))
    }

    fn acquire_lock(&self, journal: CommitJournal) -> RecoveryResult<File> {
        reject_nonregular_if_exists(&self.root.join(STATE_LOCK_FILE))?;
        let mut lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.root.join(STATE_LOCK_FILE))
            .map_err(|_| RecoveryError::DurabilityFailure)?;
        lock.try_lock_exclusive()
            .map_err(|_| RecoveryError::DurabilityFailure)?;
        let bytes = serde_jcs::to_vec(&journal).map_err(|_| RecoveryError::StateCorrupt)?;
        lock.set_len(0)
            .and_then(|_| lock.seek(SeekFrom::Start(0)).map(|_| ()))
            .and_then(|_| lock.write_all(&bytes))
            .and_then(|_| lock.sync_all())
            .map_err(|_| RecoveryError::DurabilityFailure)?;
        sync_directory(&self.root)?;
        Ok(lock)
    }
}

fn recovery_snapshot_reason(phase: RecoveryPhase) -> RecoverySnapshotReason {
    match phase {
        RecoveryPhase::Assessing => RecoverySnapshotReason::AssessingInterruptedMigration,
        RecoveryPhase::Planned => RecoverySnapshotReason::PlannedRecovery,
        RecoveryPhase::CleanupPending => RecoverySnapshotReason::TargetCleanupRequired,
        RecoveryPhase::TargetDiscarded => RecoverySnapshotReason::TargetDiscarded,
        RecoveryPhase::RollbackPending => RecoverySnapshotReason::RollbackRequired,
        RecoveryPhase::FencePending => RecoverySnapshotReason::FencingRequired,
        RecoveryPhase::Fenced => RecoverySnapshotReason::Fenced,
        RecoveryPhase::OperatorRequired => RecoverySnapshotReason::OperatorRequired,
        RecoveryPhase::ActivatePending => RecoverySnapshotReason::ActivationRequired,
        RecoveryPhase::Restored => RecoverySnapshotReason::Restored,
        RecoveryPhase::CommitPending => RecoverySnapshotReason::CommitRequired,
        RecoveryPhase::Committed => RecoverySnapshotReason::Committed,
        RecoveryPhase::ServingTransferPending => RecoverySnapshotReason::ServingTransferRequired,
        RecoveryPhase::ServingTransferred => RecoverySnapshotReason::ServingTransferred,
    }
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

fn map_authority_error(error: super::lease::AuthorityError) -> RecoveryError {
    match error.code() {
        "quorum_not_reached" | "stale_membership" | "certificate_unauthorized" => {
            RecoveryError::QuorumRequired
        }
        "lease_expired" => RecoveryError::SafetyWindow,
        _ => RecoveryError::AuthorityRejected,
    }
}

fn validate_request(
    policy: &RecoveryPolicy,
    request: &RecoveryRequest,
    histories: &[LocalHistory],
) -> RecoveryResult<()> {
    if request.trust_domain != policy.trust_domain {
        return Err(RecoveryError::WrongTrustDomain);
    }
    if !valid_bytes(&request.recovery_id, policy.max_identity_bytes)
        || !valid_bytes(&request.migration_id, policy.max_identity_bytes)
        || request.timeout_millis == 0
        || request.timeout_millis > policy.max_timeout_millis
        || histories.len() > policy.max_local_histories
        || histories.iter().any(|history| {
            !valid_bytes(&history.node_id, policy.max_identity_bytes)
                || !valid_bytes(&history.guardian_id, policy.max_identity_bytes)
        })
    {
        return Err(RecoveryError::ResourceExhausted);
    }
    Ok(())
}

fn validate_source_check(
    record: &RecoveryRecord,
    check: &ActiveLeaseCheck<'_>,
) -> RecoveryResult<()> {
    let membership = check.membership.ok_or(RecoveryError::AuthorityRejected)?;
    if membership.trust_domain_id != record.trust_domain.as_bytes()
        || check.lease.lineage_id != record.lineage_id
        || check.lease.holder_node_id != record.source_node_id
        || check.lease.holder_guardian_id != record.source_guardian_id
        || check.lease.revoked
        || check.applied_log_index != membership.committed_log_index
    {
        return Err(RecoveryError::AuthorityRejected);
    }
    Ok(())
}

fn validate_selected_lease(
    record: &RecoveryRecord,
    lease: &super::lease::LeaseState,
    applied_log_index: u64,
) -> RecoveryResult<()> {
    if lease.lineage_id != record.lineage_id
        || applied_log_index != record.committed_prefix_log_index.unwrap_or(0)
        || lease.committed_log_index != record.committed_prefix_log_index.unwrap_or(0)
        || lease.epoch != record.committed_prefix_epoch.unwrap_or(0)
        || lease.certificate_generation != record.committed_prefix_voter_generation.unwrap_or(0)
        || sha256(&lease.certificate_bytes)
            != record
                .committed_prefix_certificate_sha256
                .unwrap_or([0; 32])
    {
        return Err(RecoveryError::AuthorityRejected);
    }
    Ok(())
}

fn validate_activation_predecessor(
    record: &RecoveryRecord,
    lease: &super::lease::LeaseState,
    applied_log_index: u64,
) -> RecoveryResult<()> {
    if let (Some(epoch), Some(log_index), Some(certificate_sha256)) = (
        record.fence_epoch,
        record.fence_log_index,
        record.fence_certificate_sha256,
    ) {
        if !lease.revoked
            || lease.lineage_id != record.lineage_id
            || lease.epoch != epoch
            || lease.committed_log_index != log_index
            || applied_log_index != log_index
            || sha256(&lease.certificate_bytes) != certificate_sha256
        {
            return Err(RecoveryError::AuthorityRejected);
        }
        Ok(())
    } else {
        validate_selected_lease(record, lease, applied_log_index)
    }
}

fn validate_active_successor(
    record: &RecoveryRecord,
    lease: &super::lease::LeaseState,
    applied_log_index: u64,
    floor: &super::fencing::FenceReceipt,
) -> RecoveryResult<()> {
    if lease.revoked
        || lease.lineage_id != record.lineage_id
        || !is_candidate(record, &lease.holder_node_id, &lease.holder_guardian_id)
        || lease.epoch != floor.epoch
        || lease.committed_log_index <= floor.committed_log_index
        || lease.committed_log_index != applied_log_index
    {
        return Err(RecoveryError::AuthorityRejected);
    }
    Ok(())
}

fn is_candidate(record: &RecoveryRecord, node: &[u8], guardian: &[u8]) -> bool {
    (node == record.source_node_id && guardian == record.source_guardian_id)
        || (node == record.target_node_id && guardian == record.target_guardian_id)
}

fn copy_active_check<'a>(check: &ActiveLeaseCheck<'a>) -> ActiveLeaseCheck<'a> {
    ActiveLeaseCheck {
        membership: check.membership,
        lease: check.lease,
        applied_log_index: check.applied_log_index,
        now_unix_seconds: check.now_unix_seconds,
        now_unix_millis: check.now_unix_millis,
        now_elapsed_millis: check.now_elapsed_millis,
        activation_proof: check.activation_proof,
    }
}

fn local_histories_digest(histories: &[LocalHistory]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"ADL-RECOVERY-LOCAL-HISTORIES-V1\0");
    for history in histories {
        hasher.update((history.node_id.len() as u64).to_be_bytes());
        hasher.update(&history.node_id);
        hasher.update((history.guardian_id.len() as u64).to_be_bytes());
        hasher.update(&history.guardian_id);
        hasher.update(history.claimed_epoch.to_be_bytes());
        hasher.update(history.claimed_log_index.to_be_bytes());
        hasher.update([u8::from(history.claimed_owner)]);
    }
    hasher.finalize().into()
}

fn exact_retry(
    record: &RecoveryRecord,
    phase: RecoveryPhase,
    evidence_sha256: [u8; 32],
) -> RecoveryResult<RecoveryRecord> {
    if record
        .history
        .last()
        .is_some_and(|entry| entry.phase == phase && entry.evidence_sha256 == evidence_sha256)
    {
        Ok(record.clone())
    } else {
        Err(RecoveryError::ReplayMismatch)
    }
}

fn ensure_last_evidence(
    record: &RecoveryRecord,
    phase: RecoveryPhase,
    evidence_sha256: [u8; 32],
) -> RecoveryResult<()> {
    if record.phase == phase
        && record
            .history
            .last()
            .is_some_and(|entry| entry.phase == phase && entry.evidence_sha256 == evidence_sha256)
    {
        Ok(())
    } else {
        Err(RecoveryError::ReplayMismatch)
    }
}

fn push_history(
    policy: &RecoveryPolicy,
    record: &mut RecoveryRecord,
    phase: RecoveryPhase,
    evidence_sha256: [u8; 32],
) -> RecoveryResult<()> {
    if record.history.len() >= policy.max_history_per_record {
        return Err(RecoveryError::ResourceExhausted);
    }
    record.history.push(RecoveryEvidence {
        phase,
        evidence_sha256,
    });
    Ok(())
}

fn clear_owner(record: &mut RecoveryRecord) {
    record.owner_node_id = None;
    record.owner_guardian_id = None;
    record.owner_epoch = None;
    record.committed_log_index = None;
    record.authority_certificate_sha256 = None;
}

fn validate_serving_receipt_fields(
    operation_ref: &str,
    input_sha256: &str,
    result_state_sha256: &str,
    receipt_sha256: &str,
    expected_lineage_ref: &str,
    receipt: &IntegratedSnapshotReceipt,
) -> RecoveryResult<()> {
    if receipt.operation_ref != operation_ref
        || receipt.outcome != IntegratedOutcome::Recovery
        || receipt.shepherd.lineage_ref != expected_lineage_ref
        || receipt.observatory.lineage_ref != expected_lineage_ref
        || receipt.input_sha256 != input_sha256
        || receipt.result_state_sha256 != result_state_sha256
        || receipt.receipt_sha256 != receipt_sha256
        || !is_sha256_text(input_sha256)
        || !is_sha256_text(result_state_sha256)
        || !is_sha256_text(receipt_sha256)
    {
        return Err(RecoveryError::ServingTransferRejected);
    }
    Ok(())
}

fn validate_serving_source_lineage(
    expected_lineage_ref: &str,
    receipt: &IntegratedSnapshotReceipt,
) -> RecoveryResult<()> {
    if receipt.shepherd.lineage_ref != expected_lineage_ref
        || receipt.observatory.lineage_ref != expected_lineage_ref
    {
        return Err(RecoveryError::ServingTransferRejected);
    }
    Ok(())
}

fn validate_recovery_serving_record(
    record: &RecoveryRecord,
    serving_authority: &IntegratedServingAuthoritySnapshotStore,
) -> RecoveryResult<()> {
    let operation_ref = record
        .serving_operation_ref
        .as_deref()
        .ok_or(RecoveryError::ServingTransferRejected)?;
    let input_sha256 = record
        .serving_input_sha256
        .as_deref()
        .ok_or(RecoveryError::ServingTransferRejected)?;
    let result_state_sha256 = record
        .serving_result_state_sha256
        .as_deref()
        .ok_or(RecoveryError::ServingTransferRejected)?;
    let receipt_sha256 = record
        .serving_receipt_sha256
        .as_deref()
        .ok_or(RecoveryError::ServingTransferRejected)?;
    let receipt = serving_authority
        .receipt(operation_ref)
        .ok_or(RecoveryError::ServingTransferRejected)?;
    let expected_lineage_ref = serving_lineage_ref(&record.lineage_id)?;
    validate_serving_receipt_fields(
        operation_ref,
        input_sha256,
        result_state_sha256,
        receipt_sha256,
        &expected_lineage_ref,
        receipt,
    )
}

fn recovery_serving_operation_ref(recovery_id: &[u8]) -> String {
    format!("recovery:{}", projection_ref(b"recovery", recovery_id))
}

fn serving_lineage_ref(lineage_id: &[u8]) -> RecoveryResult<String> {
    let lineage = std::str::from_utf8(lineage_id).map_err(|_| RecoveryError::MigrationMismatch)?;
    Ok(hex::encode(Sha256::digest(format!(
        "ADL-SERVING-REF-V1\0lineage\0{lineage}"
    ))))
}

fn recovery_serving_transfer_values_evidence(
    recovery_id: &[u8],
    migration_id: &[u8],
    operation_ref: &str,
    input_sha256: &str,
    result_state_sha256: &str,
    receipt_sha256: &str,
) -> [u8; 32] {
    sha256_many(&[
        b"ADL-RECOVERY-SERVING-TRANSFER-V1\0",
        recovery_id,
        migration_id,
        operation_ref.as_bytes(),
        input_sha256.as_bytes(),
        result_state_sha256.as_bytes(),
        receipt_sha256.as_bytes(),
    ])
}

fn recovery_serving_transfer_evidence(record: &RecoveryRecord) -> RecoveryResult<[u8; 32]> {
    Ok(recovery_serving_transfer_values_evidence(
        &record.recovery_id,
        &record.migration_id,
        record
            .serving_operation_ref
            .as_deref()
            .ok_or(RecoveryError::ServingTransferRejected)?,
        record
            .serving_input_sha256
            .as_deref()
            .ok_or(RecoveryError::ServingTransferRejected)?,
        record
            .serving_result_state_sha256
            .as_deref()
            .ok_or(RecoveryError::ServingTransferRejected)?,
        record
            .serving_receipt_sha256
            .as_deref()
            .ok_or(RecoveryError::ServingTransferRejected)?,
    ))
}

fn is_sha256_text(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn collect_records(
    policy: &RecoveryPolicy,
    records: Vec<RecoveryRecord>,
) -> RecoveryResult<BTreeMap<Vec<u8>, RecoveryRecord>> {
    let mut collected = BTreeMap::new();
    for record in records {
        validate_record(policy, &record).map_err(|_| RecoveryError::StateCorrupt)?;
        if collected
            .insert(record.recovery_id.clone(), record)
            .is_some()
        {
            return Err(RecoveryError::StateCorrupt);
        }
    }
    Ok(collected)
}

fn validate_record(policy: &RecoveryPolicy, record: &RecoveryRecord) -> RecoveryResult<()> {
    let owner_required = matches!(
        record.phase,
        RecoveryPhase::Restored
            | RecoveryPhase::CommitPending
            | RecoveryPhase::Committed
            | RecoveryPhase::ServingTransferPending
            | RecoveryPhase::ServingTransferred
    );
    let prefix_required = record
        .history
        .iter()
        .any(|entry| entry.phase == RecoveryPhase::Planned);
    let prefix_field_count = [
        record.committed_prefix_sha256.is_some(),
        record.committed_prefix_epoch.is_some(),
        record.committed_prefix_log_index.is_some(),
        record.committed_prefix_voter_generation.is_some(),
        record.committed_prefix_certificate_sha256.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let prefix_present = prefix_field_count == 5;
    let fence_field_count = [
        record.fence_epoch.is_some(),
        record.fence_log_index.is_some(),
        record.fence_certificate_sha256.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let fence_required = record.history.iter().any(|entry| {
        matches!(
            entry.phase,
            RecoveryPhase::FencePending | RecoveryPhase::Fenced
        )
    });
    let serving_field_count = [
        record.serving_operation_ref.is_some(),
        record.serving_input_sha256.is_some(),
        record.serving_result_state_sha256.is_some(),
        record.serving_receipt_sha256.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let serving_transferred = record.phase == RecoveryPhase::ServingTransferred;
    let transitions_valid = record.history.windows(2).all(|pair| {
        matches!(
            (pair[0].phase, pair[1].phase),
            (RecoveryPhase::Assessing, RecoveryPhase::Planned)
                | (RecoveryPhase::Assessing, RecoveryPhase::OperatorRequired)
                | (RecoveryPhase::Planned, RecoveryPhase::CleanupPending)
                | (
                    RecoveryPhase::CleanupPending,
                    RecoveryPhase::TargetDiscarded
                )
                | (
                    RecoveryPhase::CleanupPending,
                    RecoveryPhase::OperatorRequired
                )
                | (RecoveryPhase::Planned, RecoveryPhase::OperatorRequired)
                | (RecoveryPhase::Planned, RecoveryPhase::RollbackPending)
                | (
                    RecoveryPhase::TargetDiscarded,
                    RecoveryPhase::RollbackPending
                )
                | (RecoveryPhase::RollbackPending, RecoveryPhase::Restored)
                | (
                    RecoveryPhase::RollbackPending,
                    RecoveryPhase::OperatorRequired
                )
                | (
                    RecoveryPhase::TargetDiscarded,
                    RecoveryPhase::OperatorRequired
                )
                | (RecoveryPhase::Planned, RecoveryPhase::FencePending)
                | (RecoveryPhase::FencePending, RecoveryPhase::Fenced)
                | (RecoveryPhase::FencePending, RecoveryPhase::OperatorRequired)
                | (RecoveryPhase::Fenced, RecoveryPhase::OperatorRequired)
                | (RecoveryPhase::Fenced, RecoveryPhase::ActivatePending)
                | (
                    RecoveryPhase::OperatorRequired,
                    RecoveryPhase::ActivatePending
                )
                | (RecoveryPhase::ActivatePending, RecoveryPhase::Restored)
                | (
                    RecoveryPhase::ActivatePending,
                    RecoveryPhase::OperatorRequired
                )
                | (RecoveryPhase::Restored, RecoveryPhase::CommitPending)
                | (RecoveryPhase::CommitPending, RecoveryPhase::Committed)
                | (
                    RecoveryPhase::CommitPending,
                    RecoveryPhase::OperatorRequired
                )
                | (
                    RecoveryPhase::Committed,
                    RecoveryPhase::ServingTransferPending
                )
                | (
                    RecoveryPhase::ServingTransferPending,
                    RecoveryPhase::ServingTransferred
                )
                | (
                    RecoveryPhase::ServingTransferPending,
                    RecoveryPhase::OperatorRequired
                )
        )
    });
    if !valid_bytes(&record.recovery_id, policy.max_identity_bytes)
        || !valid_bytes(&record.migration_id, policy.max_identity_bytes)
        || record.trust_domain != policy.trust_domain
        || !valid_bytes(&record.lineage_id, policy.max_identity_bytes)
        || !valid_bytes(&record.source_node_id, policy.max_identity_bytes)
        || !valid_bytes(&record.source_guardian_id, policy.max_identity_bytes)
        || !valid_bytes(&record.target_node_id, policy.max_identity_bytes)
        || !valid_bytes(&record.target_guardian_id, policy.max_identity_bytes)
        || record.started_at_millis >= record.deadline_millis
        || record.history.is_empty()
        || record.history.len() > policy.max_history_per_record
        || record.history.first().map(|entry| entry.phase) != Some(RecoveryPhase::Assessing)
        || record.history.last().map(|entry| entry.phase) != Some(record.phase)
        || !transitions_valid
        || !matches!(prefix_field_count, 0 | 5)
        || (prefix_required && !prefix_present)
        || !matches!(fence_field_count, 0 | 3)
        || (fence_required && fence_field_count != 3)
        || !matches!(serving_field_count, 0 | 4)
        || (serving_transferred && serving_field_count != 4)
        || (!serving_transferred && serving_field_count != 0)
        || record.target_cleanup_required
            != matches!(
                record.observed_migration_phase,
                MigrationPhase::Quiesced
                    | MigrationPhase::Checkpointed
                    | MigrationPhase::Transferred
                    | MigrationPhase::Validated
            )
        || matches!(
            record.phase,
            RecoveryPhase::TargetDiscarded
                | RecoveryPhase::RollbackPending
                | RecoveryPhase::Restored
                | RecoveryPhase::CommitPending
                | RecoveryPhase::Committed
                | RecoveryPhase::ServingTransferPending
                | RecoveryPhase::ServingTransferred
        ) && record.target_cleanup_required
            && record.target_cleanup_receipt_sha256.is_none()
        || record.target_cleanup_receipt_sha256.is_some() && !record.target_cleanup_required
        || owner_required
            != (record.owner_node_id.is_some()
                && record.owner_guardian_id.is_some()
                && record.owner_epoch.is_some()
                && record.committed_log_index.is_some()
                && record.authority_certificate_sha256.is_some())
        || record
            .owner_node_id
            .as_deref()
            .zip(record.owner_guardian_id.as_deref())
            .is_some_and(|(node, guardian)| !is_candidate(record, node, guardian))
    {
        return Err(RecoveryError::StateCorrupt);
    }
    Ok(())
}

fn unix_millis(seconds: i64, nanos: u32) -> Option<u64> {
    u64::try_from(seconds)
        .ok()?
        .checked_mul(1_000)?
        .checked_add(u64::from(nanos) / 1_000_000)
}

fn validate_root(path: &Path) -> RecoveryResult<PathBuf> {
    if !path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(RecoveryError::UnsafeStatePath);
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| RecoveryError::UnsafeStatePath)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(RecoveryError::UnsafeStatePath);
    }
    path.canonicalize()
        .map_err(|_| RecoveryError::UnsafeStatePath)
}

fn checkpoint_matches(path: &Path, checkpoint: RecoveryCheckpoint, max_bytes: usize) -> bool {
    read_bounded_regular(path, max_bytes, RecoveryError::DurabilityFailure)
        .ok()
        .filter(|bytes| sha256(bytes) == checkpoint.state_sha256)
        .and_then(|bytes| serde_json::from_slice::<StateEnvelope>(&bytes).ok())
        .is_some_and(|envelope| envelope.body.generation == checkpoint.generation)
}

fn recover_interrupted_commit(
    root: &Path,
    authority: &dyn RecoveryCheckpointAuthority,
    max_state_bytes: usize,
) -> RecoveryResult<()> {
    let lock_path = root.join(STATE_LOCK_FILE);
    reject_nonregular_if_exists(&lock_path)?;
    if !lock_path.exists() {
        return if root.join(STATE_BACKUP_FILE).exists() {
            Err(RecoveryError::DurabilityFailure)
        } else {
            Ok(())
        };
    }
    let mut lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    lock.try_lock_exclusive()
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    let bytes = read_bounded_regular(
        &lock_path,
        MAX_JOURNAL_BYTES,
        RecoveryError::DurabilityFailure,
    )?;
    if bytes.is_empty() {
        if root.join(STATE_BACKUP_FILE).exists() {
            return Err(RecoveryError::DurabilityFailure);
        }
        FileExt::unlock(&lock).map_err(|_| RecoveryError::DurabilityFailure)?;
        return Ok(());
    }
    let journal: CommitJournal =
        serde_json::from_slice(&bytes).map_err(|_| RecoveryError::DurabilityFailure)?;
    let state_path = root.join(STATE_FILE);
    let backup_path = root.join(STATE_BACKUP_FILE);
    reject_nonregular_if_exists(&state_path)?;
    reject_nonregular_if_exists(&backup_path)?;
    let current = authority.current()?;
    if current == journal.expected {
        if let Some(expected) = journal.expected {
            if backup_path.exists() && checkpoint_matches(&backup_path, expected, max_state_bytes) {
                if state_path.exists() {
                    fs::remove_file(&state_path).map_err(|_| RecoveryError::DurabilityFailure)?;
                }
                fs::rename(&backup_path, &state_path)
                    .map_err(|_| RecoveryError::DurabilityFailure)?;
            } else if !checkpoint_matches(&state_path, expected, max_state_bytes) {
                return Err(RecoveryError::Rollback);
            }
        } else {
            if state_path.exists() {
                fs::remove_file(&state_path).map_err(|_| RecoveryError::DurabilityFailure)?;
            }
            if backup_path.exists() {
                fs::remove_file(&backup_path).map_err(|_| RecoveryError::DurabilityFailure)?;
            }
        }
    } else {
        match current {
            Some(checkpoint) if checkpoint == journal.next => {
                if !checkpoint_matches(&state_path, journal.next, max_state_bytes) {
                    return Err(RecoveryError::Rollback);
                }
                if backup_path.exists() {
                    fs::remove_file(&backup_path).map_err(|_| RecoveryError::DurabilityFailure)?;
                }
            }
            _ => return Err(RecoveryError::Rollback),
        }
    }
    if root.join(STATE_TEMP_FILE).exists() {
        fs::remove_file(root.join(STATE_TEMP_FILE))
            .map_err(|_| RecoveryError::DurabilityFailure)?;
    }
    sync_directory(root)?;
    clear_lock(&mut lock)
}

fn clear_lock(lock: &mut File) -> RecoveryResult<()> {
    lock.set_len(0)
        .and_then(|_| lock.seek(SeekFrom::Start(0)).map(|_| ()))
        .and_then(|_| lock.sync_all())
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    FileExt::unlock(lock).map_err(|_| RecoveryError::DurabilityFailure)
}

fn reject_nonregular_if_exists(path: &Path) -> RecoveryResult<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            Ok(())
        }
        Ok(_) => Err(RecoveryError::UnsafeStatePath),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(RecoveryError::UnsafeStatePath),
    }
}

fn read_bounded_regular(
    path: &Path,
    max_bytes: usize,
    missing_error: RecoveryError,
) -> RecoveryResult<Vec<u8>> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            missing_error.clone()
        } else {
            RecoveryError::DurabilityFailure
        }
    })?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(RecoveryError::UnsafeStatePath);
    }
    if metadata.len() > max_bytes as u64 {
        return Err(RecoveryError::ResourceExhausted);
    }
    let file = File::open(path).map_err(|_| RecoveryError::DurabilityFailure)?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(max_bytes as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    if bytes.len() > max_bytes {
        return Err(RecoveryError::ResourceExhausted);
    }
    Ok(bytes)
}

fn sync_directory(root: &Path) -> RecoveryResult<()> {
    File::open(root)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| RecoveryError::DurabilityFailure)
}

fn write_atomic(root: &Path, bytes: &[u8]) -> RecoveryResult<()> {
    let temporary = root.join(STATE_TEMP_FILE);
    let final_path = root.join(STATE_FILE);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| RecoveryError::DurabilityFailure)?;
    fs::rename(&temporary, &final_path).map_err(|_| RecoveryError::DurabilityFailure)?;
    sync_directory(root)
}

fn valid_text(value: &str, max: usize) -> bool {
    valid_bytes(value.as_bytes(), max)
        && value
            .chars()
            .all(|character| !character.is_control() && character != '\u{7f}')
}

fn valid_bytes(value: &[u8], max: usize) -> bool {
    !value.is_empty() && value.len() <= max
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn sha256_many(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    hasher.finalize().into()
}
