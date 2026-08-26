use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File, OpenOptions},
    io::{Seek, SeekFrom, Write},
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
        verify_certificate, AuthorityApplication, AuthorityMembership, LeaseState, OperationClass,
    },
    placement::{PlacementClock, PlacementInputs, PlacementRequest, PlacementService},
    shepherd_serving_eligibility::VerifiedCommittedChildLineagePair,
    snapshot_catalog::SnapshotCatalogVerifier,
};
#[cfg(test)]
use super::{
    fencing::{FencingStore, AUTHORITY_BOUND_FENCING_ACCESS},
    lease::{AuthorityLedger, AUTHORITY_BOUND_LEASE_ACCESS},
};

#[cfg(not(test))]
type MigrationLeaseAuthority = AuthorityBoundLeaseLedger;
#[cfg(test)]
type MigrationLeaseAuthority = AuthorityLedger;
#[cfg(not(test))]
type MigrationFencingAuthority = AuthorityBoundFencingStore;
#[cfg(test)]
type MigrationFencingAuthority = FencingStore;

fn migration_lease(
    ledger: &MigrationLeaseAuthority,
    lineage_id: &[u8],
) -> MigrationResult<Option<LeaseState>> {
    #[cfg(not(test))]
    return ledger
        .lease(lineage_id)
        .map_err(|_| MigrationError::SourceAuthorityRejected);
    #[cfg(test)]
    return Ok(ledger.lease(lineage_id).cloned());
}

fn migration_applied_log_index(ledger: &MigrationLeaseAuthority) -> MigrationResult<u64> {
    #[cfg(not(test))]
    return ledger
        .applied_log_index()
        .map_err(|_| MigrationError::SourceAuthorityRejected);
    #[cfg(test)]
    return Ok(ledger.applied_log_index());
}

fn migration_apply(
    ledger: &mut MigrationLeaseAuthority,
    certificate_bytes: &[u8],
    membership: &AuthorityMembership,
    application: AuthorityApplication<'_>,
) -> MigrationResult<LeaseState> {
    #[cfg(not(test))]
    return ledger
        .apply(certificate_bytes, membership, application)
        .map_err(|_| MigrationError::SourceAuthorityRejected);
    #[cfg(test)]
    return ledger
        .apply(
            &AUTHORITY_BOUND_LEASE_ACCESS,
            certificate_bytes,
            membership,
            application,
        )
        .cloned()
        .map_err(|_| MigrationError::SourceAuthorityRejected);
}

fn migration_floor(
    fencing: &MigrationFencingAuthority,
    lineage_id: &[u8],
) -> MigrationResult<Option<super::fencing::FenceReceipt>> {
    #[cfg(not(test))]
    return fencing
        .floor(lineage_id)
        .map_err(|_| MigrationError::SourceAuthorityRejected);
    #[cfg(test)]
    return Ok(fencing.floor(lineage_id).cloned());
}

fn migration_fence_commit(
    fencing: &mut MigrationFencingAuthority,
    request: FenceCommit<'_>,
) -> MigrationResult<super::fencing::FenceReceipt> {
    #[cfg(not(test))]
    return fencing
        .commit(request)
        .map_err(|_| MigrationError::FenceRejected);
    #[cfg(test)]
    return fencing
        .commit(&AUTHORITY_BOUND_FENCING_ACCESS, request)
        .map_err(|_| MigrationError::FenceRejected);
}

fn migration_authorize_active(
    fencing: &MigrationFencingAuthority,
    check: ActiveLeaseCheck<'_>,
) -> MigrationResult<()> {
    #[cfg(not(test))]
    return fencing
        .authorize_active_lease(check)
        .map_err(|_| MigrationError::SourceAuthorityRejected);
    #[cfg(test)]
    return fencing
        .authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)
        .map_err(|_| MigrationError::SourceAuthorityRejected);
}

pub const MIGRATION_STATE_SCHEMA: &str = "adl.distributed.migration_state.v1";
const STATE_FILE: &str = "migration-state.json";
const STATE_LOCK_FILE: &str = ".migration-state.lock";
const STATE_BACKUP_FILE: &str = ".migration-state.backup";
const MAX_ABSOLUTE_RECORDS: usize = 4096;
const MAX_ABSOLUTE_HISTORY: usize = 32;
const MAX_ABSOLUTE_IDENTITY_BYTES: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MigrationError {
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
    EvidenceMismatch,
    PlacementRejected,
    SourceAuthorityRejected,
    QuiescenceRejected,
    SnapshotRejected,
    RestoreRejected,
    FenceRejected,
    ActivationRejected,
    CommitRejected,
    ServingTransferRejected,
    PostFenceAbort,
    TimedOut,
    RevisionDrift,
}

impl MigrationError {
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
            Self::EvidenceMismatch => "evidence_mismatch",
            Self::PlacementRejected => "placement_rejected",
            Self::SourceAuthorityRejected => "source_authority_rejected",
            Self::QuiescenceRejected => "quiescence_rejected",
            Self::SnapshotRejected => "snapshot_rejected",
            Self::RestoreRejected => "restore_rejected",
            Self::FenceRejected => "fence_rejected",
            Self::ActivationRejected => "activation_rejected",
            Self::CommitRejected => "commit_rejected",
            Self::ServingTransferRejected => "serving_transfer_rejected",
            Self::PostFenceAbort => "post_fence_abort",
            Self::TimedOut => "timed_out",
            Self::RevisionDrift => "revision_drift",
        }
    }
}

impl fmt::Display for MigrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for MigrationError {}
pub type MigrationResult<T> = Result<T, MigrationError>;

#[derive(Clone, Debug)]
pub struct MigrationPolicy {
    pub trust_domain: String,
    pub max_records: usize,
    pub max_history_per_record: usize,
    pub max_identity_bytes: usize,
    pub max_state_bytes: usize,
    pub max_timeout_millis: u64,
    pub max_snapshot_bytes: u64,
    pub max_snapshot_chunks: usize,
}

impl MigrationPolicy {
    pub fn new(trust_domain: impl Into<String>) -> MigrationResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_records: 256,
            max_history_per_record: 16,
            max_identity_bytes: 128,
            max_state_bytes: 4 * 1024 * 1024,
            max_timeout_millis: 3_600_000,
            max_snapshot_bytes: 64 * 1024 * 1024 * 1024,
            max_snapshot_chunks: 4096,
        };
        policy.validate()?;
        Ok(policy)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_bounds(
        trust_domain: impl Into<String>,
        max_records: usize,
        max_history_per_record: usize,
        max_identity_bytes: usize,
        max_state_bytes: usize,
        max_timeout_millis: u64,
        max_snapshot_bytes: u64,
        max_snapshot_chunks: usize,
    ) -> MigrationResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_records,
            max_history_per_record,
            max_identity_bytes,
            max_state_bytes,
            max_timeout_millis,
            max_snapshot_bytes,
            max_snapshot_chunks,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn validate(&self) -> MigrationResult<()> {
        if !valid_text(&self.trust_domain, self.max_identity_bytes)
            || self.max_records == 0
            || self.max_records > MAX_ABSOLUTE_RECORDS
            || !(8..=MAX_ABSOLUTE_HISTORY).contains(&self.max_history_per_record)
            || self.max_identity_bytes == 0
            || self.max_identity_bytes > MAX_ABSOLUTE_IDENTITY_BYTES
            || !(1024..=16 * 1024 * 1024).contains(&self.max_state_bytes)
            || self.max_timeout_millis == 0
            || self.max_timeout_millis > 86_400_000
            || self.max_snapshot_bytes == 0
            || self.max_snapshot_chunks == 0
            || self.max_snapshot_chunks > 16_384
        {
            return Err(MigrationError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationPhase {
    Prepared,
    Quiesced,
    Checkpointed,
    Transferred,
    Validated,
    Fenced,
    Activated,
    Committed,
    ServingTransferred,
    Aborted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationRequest {
    pub migration_id: Vec<u8>,
    pub trust_domain: String,
    pub lineage_id: Vec<u8>,
    pub source_node_id: Vec<u8>,
    pub source_guardian_id: Vec<u8>,
    pub timeout_millis: u64,
}

pub trait MigrationClock: fmt::Debug + Send + Sync {
    fn now_millis(&self) -> MigrationResult<u64>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionEvidence {
    pub phase: MigrationPhase,
    pub evidence_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationRecord {
    pub migration_id: Vec<u8>,
    pub request_sha256: [u8; 32],
    pub trust_domain: String,
    pub lineage_id: Vec<u8>,
    pub source_node_id: Vec<u8>,
    pub source_guardian_id: Vec<u8>,
    pub source_epoch: u64,
    pub source_log_index: u64,
    pub source_certificate_sha256: [u8; 32],
    pub target_node_id: Vec<u8>,
    pub target_guardian_id: Vec<u8>,
    pub placement_membership_epoch: u64,
    pub placement_log_index: u64,
    pub placement_capability_sequence: u64,
    pub placement_weather_sequence: u64,
    pub timeout_millis: u64,
    pub started_at_millis: u64,
    pub deadline_millis: u64,
    pub phase: MigrationPhase,
    pub source_authoritative: bool,
    pub target_authoritative: bool,
    pub quiescence_sha256: Option<[u8; 32]>,
    pub catalog_entry_sha256: Option<[u8; 32]>,
    pub snapshot_content_sha256: Option<[u8; 32]>,
    pub snapshot_schema: Option<String>,
    pub snapshot_byte_length: Option<u64>,
    pub snapshot_chunk_count: Option<usize>,
    pub snapshot_expiry_unix_secs: Option<u64>,
    pub transfer_id: Option<Vec<u8>>,
    pub transfer_manifest_sha256: Option<[u8; 32]>,
    pub restore_receipt_sha256: Option<[u8; 32]>,
    pub fence_request_id: Option<Vec<u8>>,
    pub fence_epoch: Option<u64>,
    pub fence_log_index: Option<u64>,
    pub fence_certificate_sha256: Option<[u8; 32]>,
    pub activation_log_index: Option<u64>,
    pub activation_certificate_sha256: Option<[u8; 32]>,
    pub commit_log_index: Option<u64>,
    pub commit_certificate_sha256: Option<[u8; 32]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_operation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_input_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_result_state_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serving_receipt_sha256: Option<String>,
    pub history: Vec<TransitionEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationCheckpoint {
    pub generation: u64,
    pub state_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MigrationAuthorityRevision {
    checkpoint_generation: u64,
    state_sha256: [u8; 32],
}

impl MigrationAuthorityRevision {
    pub fn checkpoint_generation(&self) -> u64 {
        self.checkpoint_generation
    }

    pub fn state_sha256(&self) -> [u8; 32] {
        self.state_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedMigrationRow {
    migration_ref: String,
    lineage_ref: String,
    source_node_ref: String,
    source_guardian_ref: String,
    target_node_ref: String,
    target_guardian_ref: String,
    phase: MigrationPhase,
    source_authoritative: bool,
    target_authoritative: bool,
}

impl RedactedMigrationRow {
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

    pub fn phase(&self) -> MigrationPhase {
        self.phase
    }

    pub fn source_authoritative(&self) -> bool {
        self.source_authoritative
    }

    pub fn target_authoritative(&self) -> bool {
        self.target_authoritative
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedMigrationSnapshot {
    trust_domain: String,
    revision: MigrationAuthorityRevision,
    rows: Vec<RedactedMigrationRow>,
}

impl RedactedMigrationSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn revision(&self) -> MigrationAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedMigrationRow> {
        self.rows.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommitJournal {
    expected: MigrationCheckpoint,
    next: MigrationCheckpoint,
}

pub trait MigrationCheckpointAuthority: fmt::Debug + Send + Sync {
    fn current(&self) -> MigrationResult<Option<MigrationCheckpoint>>;

    fn compare_and_swap(
        &self,
        expected: Option<MigrationCheckpoint>,
        next: MigrationCheckpoint,
    ) -> MigrationResult<()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuiescenceRequest<'a> {
    pub migration_id: &'a [u8],
    pub lineage_id: &'a [u8],
    pub source_node_id: &'a [u8],
    pub source_guardian_id: &'a [u8],
    pub source_epoch: u64,
    pub source_log_index: u64,
    pub remaining_timeout_millis: u64,
}

pub trait SourceQuiescenceAuthority: fmt::Debug + Send + Sync {
    fn quiesce(&self, request: QuiescenceRequest<'_>) -> MigrationResult<Vec<u8>>;

    fn resume(&self, request: QuiescenceRequest<'_>) -> MigrationResult<()>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IsolatedRestoreRequest<'a> {
    pub migration_id: &'a [u8],
    pub transfer_id: &'a [u8],
    pub target_node_id: &'a [u8],
    pub target_guardian_id: &'a [u8],
    pub snapshot_schema: &'a str,
    pub content_sha256: [u8; 32],
    pub byte_length: u64,
    pub chunk_count: usize,
    pub remaining_timeout_millis: u64,
}

pub trait IsolatedRestoreAuthority: fmt::Debug + Send + Sync {
    fn validate(&self, request: IsolatedRestoreRequest<'_>) -> MigrationResult<Vec<u8>>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateBody {
    schema: String,
    generation: u64,
    records: Vec<MigrationRecord>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateEnvelope {
    body: StateBody,
    digest: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct MigrationStore {
    root: PathBuf,
    policy: MigrationPolicy,
    records: BTreeMap<Vec<u8>, MigrationRecord>,
    generation: u64,
    state_sha256: [u8; 32],
    checkpoint_authority: Arc<dyn MigrationCheckpointAuthority>,
    clock: Arc<dyn MigrationClock>,
}

impl MigrationStore {
    pub fn create(
        root: impl AsRef<Path>,
        policy: MigrationPolicy,
        checkpoint_authority: Arc<dyn MigrationCheckpointAuthority>,
        clock: Arc<dyn MigrationClock>,
    ) -> MigrationResult<Self> {
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        if root.join(STATE_FILE).exists() {
            return Err(MigrationError::StateCorrupt);
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
        let checkpoint = store.persist_records(0, &store.records)?;
        store
            .checkpoint_authority
            .compare_and_swap(None, checkpoint)?;
        store.state_sha256 = checkpoint.state_sha256;
        Ok(store)
    }

    pub fn open(
        root: impl AsRef<Path>,
        policy: MigrationPolicy,
        checkpoint_authority: Arc<dyn MigrationCheckpointAuthority>,
        clock: Arc<dyn MigrationClock>,
    ) -> MigrationResult<Self> {
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        recover_interrupted_commit(&root, checkpoint_authority.as_ref())?;
        let path = root.join(STATE_FILE);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                MigrationError::StateMissing
            } else {
                MigrationError::UnsafeStatePath
            }
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(MigrationError::UnsafeStatePath);
        }
        let bytes = fs::read(&path).map_err(|_| MigrationError::StateCorrupt)?;
        if bytes.is_empty() || bytes.len() > policy.max_state_bytes {
            return Err(MigrationError::ResourceExhausted);
        }
        let envelope: StateEnvelope =
            serde_json::from_slice(&bytes).map_err(|_| MigrationError::StateCorrupt)?;
        let body_bytes =
            serde_jcs::to_vec(&envelope.body).map_err(|_| MigrationError::StateCorrupt)?;
        if envelope.body.schema != MIGRATION_STATE_SCHEMA
            || envelope.digest != sha256(&body_bytes)
            || serde_jcs::to_vec(&envelope).map_err(|_| MigrationError::StateCorrupt)? != bytes
            || envelope.body.records.len() > policy.max_records
        {
            return Err(MigrationError::StateCorrupt);
        }
        let records = collect_records(&policy, envelope.body.records)?;
        let checkpoint = checkpoint_authority
            .current()?
            .ok_or(MigrationError::Rollback)?;
        let state_sha256 = sha256(&bytes);
        if checkpoint.generation != envelope.body.generation
            || checkpoint.state_sha256 != state_sha256
        {
            return Err(MigrationError::Rollback);
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

    pub fn checkpoint(&self) -> MigrationCheckpoint {
        MigrationCheckpoint {
            generation: self.generation,
            state_sha256: self.state_sha256,
        }
    }

    pub fn authority_revision(&self) -> MigrationResult<MigrationAuthorityRevision> {
        self.verify_current_state()?;
        Ok(MigrationAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        })
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: MigrationAuthorityRevision,
    ) -> MigrationResult<RedactedMigrationSnapshot> {
        self.verify_current_state()?;
        let revision = MigrationAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        };
        if revision != expected_revision {
            return Err(MigrationError::RevisionDrift);
        }
        if self.records.len() > self.policy.max_records {
            return Err(MigrationError::ResourceExhausted);
        }
        let rows = self
            .records
            .values()
            .map(|record| RedactedMigrationRow {
                migration_ref: projection_ref(b"migration", &record.migration_id),
                lineage_ref: projection_ref(b"lineage", &record.lineage_id),
                source_node_ref: projection_ref(b"node", &record.source_node_id),
                source_guardian_ref: projection_ref(b"guardian", &record.source_guardian_id),
                target_node_ref: projection_ref(b"node", &record.target_node_id),
                target_guardian_ref: projection_ref(b"guardian", &record.target_guardian_id),
                phase: record.phase,
                source_authoritative: record.source_authoritative,
                target_authoritative: record.target_authoritative,
            })
            .collect();
        Ok(RedactedMigrationSnapshot {
            trust_domain: self.policy.trust_domain.clone(),
            revision,
            rows,
        })
    }

    #[cfg(test)]
    pub(crate) fn seed_record_for_snapshot_test(
        &mut self,
        record: MigrationRecord,
    ) -> MigrationResult<()> {
        validate_record(&self.policy, &record)?;
        let mut prospective = self.records.clone();
        prospective.insert(record.migration_id.clone(), record);
        self.commit_records(prospective)
    }

    pub fn record(&self, migration_id: &[u8]) -> Option<&MigrationRecord> {
        self.records.get(migration_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prepare<C: PlacementClock>(
        &mut self,
        request: MigrationRequest,
        placement: &PlacementService<C>,
        placement_request: &PlacementRequest,
        placement_inputs: PlacementInputs<'_>,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        validate_request(&self.policy, &request)?;
        let started_at_millis = self.clock.now_millis()?;
        let deadline_millis = started_at_millis
            .checked_add(request.timeout_millis)
            .ok_or(MigrationError::ResourceExhausted)?;
        validate_source_request(&request, &source_check)?;
        migration_authorize_active(fencing, copy_active_check(&source_check))?;
        let decision = placement
            .decide(placement_request, placement_inputs)
            .map_err(|_| MigrationError::PlacementRejected)?;
        if decision.lineage_id.as_bytes() != request.lineage_id
            || placement_request.lineage_id.as_bytes() != request.lineage_id
            || decision.committed_log_index != source_check.applied_log_index
            || decision.node_id.as_bytes() == request.source_node_id
            || decision.guardian_id.as_bytes() == request.source_guardian_id
            || !valid_bytes(decision.node_id.as_bytes(), self.policy.max_identity_bytes)
            || !valid_bytes(
                decision.guardian_id.as_bytes(),
                self.policy.max_identity_bytes,
            )
        {
            return Err(MigrationError::EvidenceMismatch);
        }
        let request_sha256 = request_digest(&request, &decision);
        if let Some(existing) = self.records.get(&request.migration_id) {
            self.ensure_live(existing)?;
            validate_source_record(existing, &source_check)?;
            return if existing.request_sha256 == request_sha256 {
                Ok(existing.clone())
            } else {
                Err(MigrationError::ReplayMismatch)
            };
        }
        if self.records.len() >= self.policy.max_records {
            return Err(MigrationError::ResourceExhausted);
        }
        let source_certificate_sha256 = sha256(&source_check.lease.certificate_bytes);
        let prepared_evidence = sha256_many(&[
            &request_sha256,
            &source_certificate_sha256,
            decision.node_id.as_bytes(),
            decision.guardian_id.as_bytes(),
        ]);
        if self.clock.now_millis()? >= deadline_millis {
            return Err(MigrationError::TimedOut);
        }
        let record = MigrationRecord {
            migration_id: request.migration_id.clone(),
            request_sha256,
            trust_domain: request.trust_domain,
            lineage_id: request.lineage_id,
            source_node_id: request.source_node_id,
            source_guardian_id: request.source_guardian_id,
            source_epoch: source_check.lease.epoch,
            source_log_index: source_check.lease.committed_log_index,
            source_certificate_sha256,
            target_node_id: decision.node_id.into_bytes(),
            target_guardian_id: decision.guardian_id.into_bytes(),
            placement_membership_epoch: decision.membership_epoch,
            placement_log_index: decision.committed_log_index,
            placement_capability_sequence: decision.capability_sequence,
            placement_weather_sequence: decision.weather_sequence,
            timeout_millis: request.timeout_millis,
            started_at_millis,
            deadline_millis,
            phase: MigrationPhase::Prepared,
            source_authoritative: true,
            target_authoritative: false,
            quiescence_sha256: None,
            catalog_entry_sha256: None,
            snapshot_content_sha256: None,
            snapshot_schema: None,
            snapshot_byte_length: None,
            snapshot_chunk_count: None,
            snapshot_expiry_unix_secs: None,
            transfer_id: None,
            transfer_manifest_sha256: None,
            restore_receipt_sha256: None,
            fence_request_id: None,
            fence_epoch: None,
            fence_log_index: None,
            fence_certificate_sha256: None,
            activation_log_index: None,
            activation_certificate_sha256: None,
            commit_log_index: None,
            commit_certificate_sha256: None,
            serving_operation_ref: None,
            serving_input_sha256: None,
            serving_result_state_sha256: None,
            serving_receipt_sha256: None,
            history: vec![TransitionEvidence {
                phase: MigrationPhase::Prepared,
                evidence_sha256: prepared_evidence,
            }],
        };
        self.insert_new(record.clone())?;
        Ok(record)
    }

    pub fn quiesce(
        &mut self,
        migration_id: &[u8],
        authority: &dyn SourceQuiescenceAuthority,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        let remaining_timeout_millis = self.ensure_live(&current)?;
        validate_source_record(&current, &source_check)?;
        migration_authorize_active(fencing, copy_active_check(&source_check))?;
        let receipt = authority
            .quiesce(quiescence_request(&current, remaining_timeout_millis))
            .map_err(|_| MigrationError::QuiescenceRejected)?;
        if receipt.is_empty() || receipt.len() > self.policy.max_state_bytes / 4 {
            return Err(MigrationError::QuiescenceRejected);
        }
        let digest = sha256_many(&[
            b"ADL-MIGRATION-QUIESCENCE-V1\0",
            &current.request_sha256,
            &receipt,
        ]);
        self.ensure_live(&current)?;
        self.advance(
            migration_id,
            MigrationPhase::Prepared,
            MigrationPhase::Quiesced,
            digest,
            |record| {
                record.quiescence_sha256 = Some(digest);
            },
        )
    }

    pub fn checkpoint_snapshot(
        &mut self,
        migration_id: &[u8],
        encoded_catalog: &[u8],
        verifier: &SnapshotCatalogVerifier,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        self.ensure_live(&current)?;
        if current.phase != MigrationPhase::Quiesced
            && current.phase != MigrationPhase::Checkpointed
        {
            return Err(MigrationError::InvalidTransition);
        }
        validate_source_record(&current, &source_check)?;
        let verified = verifier
            .decode_catalog_and_verify(encoded_catalog, fencing, copy_active_check(&source_check))
            .map_err(|_| MigrationError::SnapshotRejected)?;
        validate_snapshot(&self.policy, &current, &verified.snapshot)?;
        let evidence = sha256(encoded_catalog);
        self.ensure_live(&current)?;
        if current.phase == MigrationPhase::Checkpointed {
            return exact_retry(&current, MigrationPhase::Checkpointed, evidence);
        }
        let snapshot = verified.snapshot;
        self.advance(
            migration_id,
            MigrationPhase::Quiesced,
            MigrationPhase::Checkpointed,
            evidence,
            |record| {
                record.catalog_entry_sha256 = Some(verified.catalog_entry_sha256);
                record.snapshot_content_sha256 = Some(snapshot.content_sha256);
                record.snapshot_schema = Some(snapshot.snapshot_schema);
                record.snapshot_byte_length = Some(snapshot.byte_length);
                record.snapshot_chunk_count = Some(snapshot.chunk_sha256.len());
                record.snapshot_expiry_unix_secs = Some(snapshot.expires_at_unix_secs);
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn transfer(
        &mut self,
        migration_id: &[u8],
        encoded_manifest: &[u8],
        encoded_catalog: &[u8],
        chunks: &[Vec<u8>],
        verifier: &SnapshotCatalogVerifier,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        self.ensure_live(&current)?;
        if current.phase == MigrationPhase::Transferred {
            validate_source_record(&current, &source_check)?;
            migration_authorize_active(fencing, copy_active_check(&source_check))?;
            let manifest_sha256 = sha256(encoded_manifest);
            if current.transfer_manifest_sha256 != Some(manifest_sha256)
                || current.catalog_entry_sha256 != Some(sha256(encoded_catalog))
            {
                return Err(MigrationError::ReplayMismatch);
            }
            let evidence = sha256_many(&[
                &manifest_sha256,
                current
                    .catalog_entry_sha256
                    .as_ref()
                    .ok_or(MigrationError::EvidenceMismatch)?,
                current
                    .snapshot_content_sha256
                    .as_ref()
                    .ok_or(MigrationError::EvidenceMismatch)?,
                current
                    .transfer_id
                    .as_deref()
                    .ok_or(MigrationError::EvidenceMismatch)?,
            ]);
            return exact_retry(&current, MigrationPhase::Transferred, evidence);
        }
        if current.phase != MigrationPhase::Checkpointed {
            return Err(MigrationError::InvalidTransition);
        }
        validate_source_record(&current, &source_check)?;
        let verified = verifier
            .decode_transfer_and_verify(
                encoded_manifest,
                encoded_catalog,
                chunks,
                &current.target_node_id,
                fencing,
                copy_active_check(&source_check),
            )
            .map_err(|_| MigrationError::SnapshotRejected)?;
        validate_snapshot(&self.policy, &current, &verified.snapshot)?;
        if Some(verified.catalog_entry_sha256) != current.catalog_entry_sha256 {
            return Err(MigrationError::EvidenceMismatch);
        }
        let manifest_sha256 = sha256(encoded_manifest);
        let evidence = sha256_many(&[
            &manifest_sha256,
            &verified.catalog_entry_sha256,
            &verified.snapshot.content_sha256,
            &verified.transfer_id,
        ]);
        self.ensure_live(&current)?;
        self.advance(
            migration_id,
            MigrationPhase::Checkpointed,
            MigrationPhase::Transferred,
            evidence,
            |record| {
                record.transfer_id = Some(verified.transfer_id);
                record.transfer_manifest_sha256 = Some(manifest_sha256);
            },
        )
    }

    pub fn validate_restore(
        &mut self,
        migration_id: &[u8],
        authority: &dyn IsolatedRestoreAuthority,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        let remaining_timeout_millis = self.ensure_live(&current)?;
        if current.phase != MigrationPhase::Transferred
            && current.phase != MigrationPhase::Validated
        {
            return Err(MigrationError::InvalidTransition);
        }
        validate_source_record(&current, &source_check)?;
        migration_authorize_active(fencing, copy_active_check(&source_check))?;
        let transfer_id = current
            .transfer_id
            .as_deref()
            .ok_or(MigrationError::EvidenceMismatch)?;
        let receipt = authority
            .validate(IsolatedRestoreRequest {
                migration_id,
                transfer_id,
                target_node_id: &current.target_node_id,
                target_guardian_id: &current.target_guardian_id,
                snapshot_schema: current
                    .snapshot_schema
                    .as_deref()
                    .ok_or(MigrationError::EvidenceMismatch)?,
                content_sha256: current
                    .snapshot_content_sha256
                    .ok_or(MigrationError::EvidenceMismatch)?,
                byte_length: current
                    .snapshot_byte_length
                    .ok_or(MigrationError::EvidenceMismatch)?,
                chunk_count: current
                    .snapshot_chunk_count
                    .ok_or(MigrationError::EvidenceMismatch)?,
                remaining_timeout_millis,
            })
            .map_err(|_| MigrationError::RestoreRejected)?;
        if receipt.is_empty() || receipt.len() > self.policy.max_state_bytes / 4 {
            return Err(MigrationError::RestoreRejected);
        }
        let digest = sha256_many(&[
            b"ADL-MIGRATION-RESTORE-V1\0",
            &current.request_sha256,
            transfer_id,
            current
                .snapshot_content_sha256
                .as_ref()
                .ok_or(MigrationError::EvidenceMismatch)?,
            &receipt,
        ]);
        self.ensure_live(&current)?;
        if current.phase == MigrationPhase::Validated {
            return exact_retry(&current, MigrationPhase::Validated, digest);
        }
        self.advance(
            migration_id,
            MigrationPhase::Transferred,
            MigrationPhase::Validated,
            digest,
            |record| record.restore_receipt_sha256 = Some(digest),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fence(
        &mut self,
        migration_id: &[u8],
        fence_request_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut MigrationLeaseAuthority,
        fencing: &mut MigrationFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        self.ensure_live(&current)?;
        if current.phase != MigrationPhase::Validated && current.phase != MigrationPhase::Fenced {
            return Err(MigrationError::InvalidTransition);
        }
        if !valid_bytes(fence_request_id, self.policy.max_identity_bytes) {
            return Err(MigrationError::ResourceExhausted);
        }
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)
                .map_err(|_| MigrationError::FenceRejected)?;
        let body = verified.body;
        if body.operation_class != OperationClass::Fence as u32
            || body.trust_domain_id != current.trust_domain.as_bytes()
            || body.lineage_id != current.lineage_id
            || body.holder_node_id != current.source_node_id
            || body.holder_guardian_id != current.source_guardian_id
            || body.epoch
                != current
                    .source_epoch
                    .checked_add(1)
                    .ok_or(MigrationError::ResourceExhausted)?
        {
            return Err(MigrationError::FenceRejected);
        }
        let certificate_sha256 = sha256(certificate_bytes);
        let evidence = sha256_many(&[
            fence_request_id,
            &certificate_sha256,
            &body.epoch.to_be_bytes(),
            &body.committed_log_index.to_be_bytes(),
        ]);
        let existing_floor = migration_floor(fencing, &current.lineage_id)?;
        let ledger_state = migration_lease(ledger, &current.lineage_id)?;
        if existing_floor.is_none() {
            let source = ledger_state
                .as_ref()
                .filter(|lease| !lease.revoked)
                .ok_or(MigrationError::FenceRejected)?;
            validate_source_lease(&current, source)?;
            migration_fence_commit(
                fencing,
                FenceCommit {
                    request_id: fence_request_id,
                    certificate_bytes,
                    membership: Some(membership),
                    current_lease: source,
                    now_unix_seconds: application.now_unix_seconds,
                },
            )?;
        }
        let floor =
            migration_floor(fencing, &current.lineage_id)?.ok_or(MigrationError::FenceRejected)?;
        if floor.request_id != fence_request_id
            || floor.epoch != body.epoch
            || floor.committed_log_index != body.committed_log_index
            || floor.certificate_sha256 != certificate_sha256
            || floor.operation_class != OperationClass::Fence as u32
        {
            return Err(MigrationError::ReplayMismatch);
        }
        let already_applied = migration_lease(ledger, &current.lineage_id)?.is_some_and(|lease| {
            lease.revoked
                && lease.epoch == body.epoch
                && lease.committed_log_index == body.committed_log_index
                && lease.certificate_bytes == certificate_bytes
        });
        if !already_applied {
            migration_apply(ledger, certificate_bytes, membership, application)
                .map_err(|_| MigrationError::FenceRejected)?;
        }
        let fenced =
            migration_lease(ledger, &current.lineage_id)?.ok_or(MigrationError::FenceRejected)?;
        if !fenced.revoked
            || fenced.epoch != floor.epoch
            || fenced.committed_log_index != floor.committed_log_index
            || fenced.certificate_bytes != certificate_bytes
        {
            return Err(MigrationError::FenceRejected);
        }
        self.ensure_live(&current)?;
        self.advance(
            migration_id,
            MigrationPhase::Validated,
            MigrationPhase::Fenced,
            evidence,
            |record| {
                record.source_authoritative = false;
                record.target_authoritative = false;
                record.fence_request_id = Some(fence_request_id.to_vec());
                record.fence_epoch = Some(floor.epoch);
                record.fence_log_index = Some(floor.committed_log_index);
                record.fence_certificate_sha256 = Some(certificate_sha256);
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn activate(
        &mut self,
        migration_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut MigrationLeaseAuthority,
        fencing: &MigrationFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> MigrationResult<MigrationRecord> {
        self.apply_target_authority(
            migration_id,
            certificate_bytes,
            membership,
            ledger,
            fencing,
            application,
            OperationClass::Activate,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn commit_owner(
        &mut self,
        migration_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut MigrationLeaseAuthority,
        fencing: &MigrationFencingAuthority,
        application: AuthorityApplication<'_>,
    ) -> MigrationResult<MigrationRecord> {
        self.apply_target_authority(
            migration_id,
            certificate_bytes,
            membership,
            ledger,
            fencing,
            application,
            OperationClass::OwnerCommit,
        )
    }

    pub fn transfer_serving_authority(
        &mut self,
        migration_id: &[u8],
        serving_authority: &mut IntegratedServingAuthoritySnapshotStore,
        pair: &VerifiedCommittedChildLineagePair<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        if current.phase != MigrationPhase::Committed
            && current.phase != MigrationPhase::ServingTransferred
        {
            return Err(MigrationError::InvalidTransition);
        }
        validate_serving_pair(&current, pair)?;
        let operation_ref = serving_operation_ref(&current);
        if current.phase == MigrationPhase::ServingTransferred {
            let receipt = serving_authority
                .receipt(&operation_ref)
                .ok_or(MigrationError::ServingTransferRejected)?
                .clone();
            validate_serving_receipt(&current, &operation_ref, &receipt)?;
            let evidence = serving_transfer_evidence(&current, &operation_ref, &receipt)?;
            if current.serving_operation_ref.as_deref() == Some(operation_ref.as_str())
                && current.serving_input_sha256.as_deref() == Some(receipt.input_sha256.as_str())
                && current.serving_result_state_sha256.as_deref()
                    == Some(receipt.result_state_sha256.as_str())
                && current.serving_receipt_sha256.as_deref()
                    == Some(receipt.receipt_sha256.as_str())
            {
                return exact_retry(&current, MigrationPhase::ServingTransferred, evidence);
            }
            return Err(MigrationError::ReplayMismatch);
        }
        if let Some(receipt) = serving_authority.receipt(&operation_ref).cloned() {
            validate_serving_receipt(&current, &operation_ref, &receipt)?;
            let evidence = serving_transfer_evidence(&current, &operation_ref, &receipt)?;
            return self.advance(
                migration_id,
                MigrationPhase::Committed,
                MigrationPhase::ServingTransferred,
                evidence,
                |record| {
                    record_serving_transfer(record, operation_ref, receipt);
                },
            );
        }
        self.ensure_live(&current)?;
        let receipt = serving_authority
            .observe(&operation_ref, pair, IntegratedOutcome::Success)
            .map_err(|_| MigrationError::ServingTransferRejected)?;
        validate_serving_receipt(&current, &operation_ref, &receipt)?;
        let evidence = serving_transfer_evidence(&current, &operation_ref, &receipt)?;
        self.advance(
            migration_id,
            MigrationPhase::Committed,
            MigrationPhase::ServingTransferred,
            evidence,
            |record| {
                record_serving_transfer(record, operation_ref, receipt);
            },
        )
    }

    pub fn abort_before_fence(
        &mut self,
        migration_id: &[u8],
        authority: &dyn SourceQuiescenceAuthority,
        fencing: &MigrationFencingAuthority,
        source_check: ActiveLeaseCheck<'_>,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        if matches!(
            current.phase,
            MigrationPhase::Fenced
                | MigrationPhase::Activated
                | MigrationPhase::Committed
                | MigrationPhase::ServingTransferred
        ) {
            return Err(MigrationError::PostFenceAbort);
        }
        if current.phase == MigrationPhase::Aborted {
            return Ok(current);
        }
        let recovery_started_millis = self.clock.now_millis()?;
        let (recovery_deadline_millis, remaining_timeout_millis) = match self.ensure_live(&current)
        {
            Ok(remaining) => (current.deadline_millis, remaining),
            Err(MigrationError::TimedOut) => {
                let budget = self.policy.max_timeout_millis.min(60_000);
                let deadline = recovery_started_millis
                    .checked_add(budget)
                    .ok_or(MigrationError::ResourceExhausted)?;
                (deadline, budget)
            }
            Err(error) => return Err(error),
        };
        validate_source_record(&current, &source_check)?;
        migration_authorize_active(fencing, copy_active_check(&source_check))?;
        authority
            .resume(quiescence_request(&current, remaining_timeout_millis))
            .map_err(|_| MigrationError::QuiescenceRejected)?;
        let recovery_finished_millis = self.clock.now_millis()?;
        if recovery_finished_millis < recovery_started_millis
            || recovery_finished_millis >= recovery_deadline_millis
        {
            return Err(MigrationError::TimedOut);
        }
        let evidence = sha256_many(&[b"abort", migration_id, &current.request_sha256]);
        self.advance_any_pre_fence(migration_id, evidence, |record| {
            record.phase = MigrationPhase::Aborted;
            record.source_authoritative = true;
            record.target_authoritative = false;
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_target_authority(
        &mut self,
        migration_id: &[u8],
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        ledger: &mut MigrationLeaseAuthority,
        fencing: &MigrationFencingAuthority,
        application: AuthorityApplication<'_>,
        operation: OperationClass,
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        self.ensure_live(&current)?;
        let (expected_phase, target_phase, error) = match operation {
            OperationClass::Activate => (
                MigrationPhase::Fenced,
                MigrationPhase::Activated,
                MigrationError::ActivationRejected,
            ),
            OperationClass::OwnerCommit => (
                MigrationPhase::Activated,
                MigrationPhase::Committed,
                MigrationError::CommitRejected,
            ),
            _ => return Err(MigrationError::InvalidTransition),
        };
        if current.phase != expected_phase && current.phase != target_phase {
            return Err(MigrationError::InvalidTransition);
        }
        let verified =
            verify_certificate(certificate_bytes, membership, application.now_unix_seconds)
                .map_err(|_| error.clone())?;
        let body = verified.body;
        if body.operation_class != operation as u32
            || body.trust_domain_id != current.trust_domain.as_bytes()
            || body.lineage_id != current.lineage_id
            || body.holder_node_id != current.target_node_id
            || body.holder_guardian_id != current.target_guardian_id
            || Some(body.epoch) != current.fence_epoch
            || body.committed_log_index != membership.committed_log_index
        {
            return Err(error);
        }
        let certificate_sha256 = sha256(certificate_bytes);
        let evidence = sha256_many(&[
            &certificate_sha256,
            &body.epoch.to_be_bytes(),
            &body.committed_log_index.to_be_bytes(),
            &[operation as u8],
        ]);
        let already_applied = migration_lease(ledger, &current.lineage_id)?.is_some_and(|lease| {
            !lease.revoked
                && lease.epoch == body.epoch
                && lease.holder_node_id == current.target_node_id
                && lease.holder_guardian_id == current.target_guardian_id
                && lease.committed_log_index == body.committed_log_index
                && lease.certificate_bytes == certificate_bytes
        });
        if !already_applied {
            migration_apply(ledger, certificate_bytes, membership, application)
                .map_err(|_| error.clone())?;
        }
        let lease = migration_lease(ledger, &current.lineage_id)?.ok_or_else(|| error.clone())?;
        if operation == OperationClass::Activate {
            migration_authorize_active(
                fencing,
                ActiveLeaseCheck {
                    membership: Some(membership),
                    lease: &lease,
                    applied_log_index: migration_applied_log_index(ledger)?,
                    now_unix_seconds: application.now_unix_seconds,
                    now_unix_millis: unix_millis(
                        application.now_unix_seconds,
                        application.now_unix_nanos,
                    )
                    .ok_or_else(|| error.clone())?,
                    now_elapsed_millis: application.now_elapsed_millis,
                    activation_proof: application.activation_proof,
                },
            )
            .map_err(|_| error.clone())?;
        } else {
            let floor =
                migration_floor(fencing, &current.lineage_id)?.ok_or_else(|| error.clone())?;
            if floor.operation_class != OperationClass::Fence as u32
                || floor.epoch != lease.epoch
                || floor.committed_log_index >= lease.committed_log_index
                || lease.revoked
            {
                return Err(error);
            }
        }
        self.ensure_live(&current)?;
        self.advance(
            migration_id,
            expected_phase,
            target_phase,
            evidence,
            |record| match operation {
                OperationClass::Activate => {
                    record.target_authoritative = true;
                    record.activation_log_index = Some(body.committed_log_index);
                    record.activation_certificate_sha256 = Some(certificate_sha256);
                }
                OperationClass::OwnerCommit => {
                    record.target_authoritative = true;
                    record.commit_log_index = Some(body.committed_log_index);
                    record.commit_certificate_sha256 = Some(certificate_sha256);
                }
                _ => {}
            },
        )
    }

    fn required_record(&self, migration_id: &[u8]) -> MigrationResult<&MigrationRecord> {
        self.records
            .get(migration_id)
            .ok_or(MigrationError::NotFound)
    }

    fn ensure_live(&self, record: &MigrationRecord) -> MigrationResult<u64> {
        let now = self.clock.now_millis()?;
        if now < record.started_at_millis || now >= record.deadline_millis {
            return Err(MigrationError::TimedOut);
        }
        record
            .deadline_millis
            .checked_sub(now)
            .filter(|remaining| *remaining > 0)
            .ok_or(MigrationError::TimedOut)
    }

    fn insert_new(&mut self, record: MigrationRecord) -> MigrationResult<()> {
        let mut prospective = self.records.clone();
        if prospective
            .insert(record.migration_id.clone(), record)
            .is_some()
        {
            return Err(MigrationError::ReplayMismatch);
        }
        self.commit_records(prospective)
    }

    fn advance(
        &mut self,
        migration_id: &[u8],
        expected: MigrationPhase,
        target: MigrationPhase,
        evidence_sha256: [u8; 32],
        update: impl FnOnce(&mut MigrationRecord),
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        if current.phase == target {
            return exact_retry(&current, target, evidence_sha256);
        }
        if current.phase != expected {
            return Err(MigrationError::InvalidTransition);
        }
        let mut next = current;
        update(&mut next);
        next.phase = target;
        push_history(&self.policy, &mut next, target, evidence_sha256)?;
        let mut prospective = self.records.clone();
        prospective.insert(migration_id.to_vec(), next.clone());
        self.commit_records(prospective)?;
        Ok(next)
    }

    fn advance_any_pre_fence(
        &mut self,
        migration_id: &[u8],
        evidence_sha256: [u8; 32],
        update: impl FnOnce(&mut MigrationRecord),
    ) -> MigrationResult<MigrationRecord> {
        let current = self.required_record(migration_id)?.clone();
        if matches!(
            current.phase,
            MigrationPhase::Fenced
                | MigrationPhase::Activated
                | MigrationPhase::Committed
                | MigrationPhase::ServingTransferred
        ) {
            return Err(MigrationError::PostFenceAbort);
        }
        let mut next = current;
        update(&mut next);
        push_history(
            &self.policy,
            &mut next,
            MigrationPhase::Aborted,
            evidence_sha256,
        )?;
        let mut prospective = self.records.clone();
        prospective.insert(migration_id.to_vec(), next.clone());
        self.commit_records(prospective)?;
        Ok(next)
    }

    fn commit_records(
        &mut self,
        prospective: BTreeMap<Vec<u8>, MigrationRecord>,
    ) -> MigrationResult<()> {
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(MigrationError::ResourceExhausted)?;
        let (bytes, checkpoint) = self.encode_records(generation, &prospective)?;
        let expected = self.checkpoint();
        let mut lock = self.acquire_lock(CommitJournal {
            expected,
            next: checkpoint,
        })?;
        let result = self.verify_current_state().and_then(|_| {
            fs::rename(
                self.root.join(STATE_FILE),
                self.root.join(STATE_BACKUP_FILE),
            )
            .map_err(|_| MigrationError::DurabilityFailure)?;
            write_atomic(&self.root, &bytes)?;
            self.checkpoint_authority
                .compare_and_swap(Some(expected), checkpoint)?;
            fs::remove_file(self.root.join(STATE_BACKUP_FILE))
                .map_err(|_| MigrationError::DurabilityFailure)?;
            sync_directory(&self.root)
        });
        if result.is_err() {
            drop(lock);
            recover_interrupted_commit(&self.root, self.checkpoint_authority.as_ref())?;
            if self.checkpoint_authority.current()? != Some(checkpoint) {
                return result;
            }
        } else {
            lock.set_len(0)
                .and_then(|_| lock.seek(SeekFrom::Start(0)).map(|_| ()))
                .and_then(|_| lock.sync_all())
                .map_err(|_| MigrationError::DurabilityFailure)?;
            FileExt::unlock(&lock).map_err(|_| MigrationError::DurabilityFailure)?;
        }
        self.records = prospective;
        self.generation = generation;
        self.state_sha256 = checkpoint.state_sha256;
        Ok(())
    }

    fn verify_current_state(&self) -> MigrationResult<()> {
        let bytes =
            fs::read(self.root.join(STATE_FILE)).map_err(|_| MigrationError::DurabilityFailure)?;
        if sha256(&bytes) != self.state_sha256
            || self.checkpoint_authority.current()? != Some(self.checkpoint())
        {
            return Err(MigrationError::Rollback);
        }
        let envelope: StateEnvelope =
            serde_json::from_slice(&bytes).map_err(|_| MigrationError::StateCorrupt)?;
        if envelope.body.generation != self.generation {
            return Err(MigrationError::Rollback);
        }
        Ok(())
    }

    fn persist_records(
        &self,
        generation: u64,
        records: &BTreeMap<Vec<u8>, MigrationRecord>,
    ) -> MigrationResult<MigrationCheckpoint> {
        let (bytes, checkpoint) = self.encode_records(generation, records)?;
        write_atomic(&self.root, &bytes)?;
        Ok(checkpoint)
    }

    fn encode_records(
        &self,
        generation: u64,
        records: &BTreeMap<Vec<u8>, MigrationRecord>,
    ) -> MigrationResult<(Vec<u8>, MigrationCheckpoint)> {
        if records.len() > self.policy.max_records
            || records
                .values()
                .any(|record| validate_record(&self.policy, record).is_err())
        {
            return Err(MigrationError::ResourceExhausted);
        }
        let body = StateBody {
            schema: MIGRATION_STATE_SCHEMA.to_owned(),
            generation,
            records: records.values().cloned().collect(),
        };
        let body_bytes = serde_jcs::to_vec(&body).map_err(|_| MigrationError::StateCorrupt)?;
        let envelope = StateEnvelope {
            body,
            digest: sha256(&body_bytes),
        };
        let bytes = serde_jcs::to_vec(&envelope).map_err(|_| MigrationError::StateCorrupt)?;
        if bytes.len() > self.policy.max_state_bytes {
            return Err(MigrationError::ResourceExhausted);
        }
        let checkpoint = MigrationCheckpoint {
            generation,
            state_sha256: sha256(&bytes),
        };
        Ok((bytes, checkpoint))
    }

    fn acquire_lock(&self, journal: CommitJournal) -> MigrationResult<File> {
        let lock_path = self.root.join(STATE_LOCK_FILE);
        let mut lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|_| MigrationError::DurabilityFailure)?;
        lock.try_lock_exclusive()
            .map_err(|_| MigrationError::DurabilityFailure)?;
        let bytes = serde_jcs::to_vec(&journal).map_err(|_| MigrationError::StateCorrupt)?;
        lock.set_len(0)
            .and_then(|_| lock.seek(SeekFrom::Start(0)).map(|_| ()))
            .and_then(|_| lock.write_all(&bytes))
            .and_then(|_| lock.sync_all())
            .map_err(|_| MigrationError::DurabilityFailure)?;
        sync_directory(&self.root)?;
        Ok(lock)
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

fn validate_request(policy: &MigrationPolicy, request: &MigrationRequest) -> MigrationResult<()> {
    if request.trust_domain != policy.trust_domain {
        return Err(MigrationError::WrongTrustDomain);
    }
    if !valid_bytes(&request.migration_id, policy.max_identity_bytes)
        || !valid_bytes(&request.lineage_id, policy.max_identity_bytes)
        || !valid_bytes(&request.source_node_id, policy.max_identity_bytes)
        || !valid_bytes(&request.source_guardian_id, policy.max_identity_bytes)
        || request.timeout_millis == 0
        || request.timeout_millis > policy.max_timeout_millis
    {
        return Err(MigrationError::ResourceExhausted);
    }
    Ok(())
}

fn validate_source_request(
    request: &MigrationRequest,
    check: &ActiveLeaseCheck<'_>,
) -> MigrationResult<()> {
    if request.trust_domain.as_bytes()
        != check
            .membership
            .ok_or(MigrationError::SourceAuthorityRejected)?
            .trust_domain_id
        || request.lineage_id != check.lease.lineage_id
        || request.source_node_id != check.lease.holder_node_id
        || request.source_guardian_id != check.lease.holder_guardian_id
        || check.lease.revoked
    {
        return Err(MigrationError::EvidenceMismatch);
    }
    Ok(())
}

fn validate_source_record(
    record: &MigrationRecord,
    check: &ActiveLeaseCheck<'_>,
) -> MigrationResult<()> {
    if record.trust_domain.as_bytes()
        != check
            .membership
            .ok_or(MigrationError::SourceAuthorityRejected)?
            .trust_domain_id
        || record.lineage_id != check.lease.lineage_id
        || record.source_node_id != check.lease.holder_node_id
        || record.source_guardian_id != check.lease.holder_guardian_id
        || record.source_epoch != check.lease.epoch
        || record.source_log_index != check.lease.committed_log_index
        || record.source_certificate_sha256 != sha256(&check.lease.certificate_bytes)
        || check.lease.revoked
    {
        return Err(MigrationError::EvidenceMismatch);
    }
    Ok(())
}

fn validate_source_lease(record: &MigrationRecord, lease: &LeaseState) -> MigrationResult<()> {
    if lease.revoked
        || lease.lineage_id != record.lineage_id
        || lease.holder_node_id != record.source_node_id
        || lease.holder_guardian_id != record.source_guardian_id
        || lease.epoch != record.source_epoch
        || lease.committed_log_index != record.source_log_index
        || sha256(&lease.certificate_bytes) != record.source_certificate_sha256
    {
        return Err(MigrationError::EvidenceMismatch);
    }
    Ok(())
}

fn validate_snapshot(
    policy: &MigrationPolicy,
    record: &MigrationRecord,
    snapshot: &super::snapshot_catalog::SnapshotDescriptor,
) -> MigrationResult<()> {
    if snapshot.trust_domain != record.trust_domain
        || snapshot.lineage_id != record.lineage_id
        || snapshot.source_owner_id != record.source_node_id
        || snapshot.source_guardian_id != record.source_guardian_id
        || snapshot.source_epoch != record.source_epoch
        || snapshot.authority_log_index != record.source_log_index
        || snapshot.byte_length == 0
        || snapshot.byte_length > policy.max_snapshot_bytes
        || snapshot.chunk_sha256.is_empty()
        || snapshot.chunk_sha256.len() > policy.max_snapshot_chunks
    {
        return Err(MigrationError::EvidenceMismatch);
    }
    if let Some(content) = record.snapshot_content_sha256 {
        if content != snapshot.content_sha256
            || record.snapshot_schema.as_deref() != Some(snapshot.snapshot_schema.as_str())
            || record.snapshot_byte_length != Some(snapshot.byte_length)
            || record.snapshot_chunk_count != Some(snapshot.chunk_sha256.len())
            || record.snapshot_expiry_unix_secs != Some(snapshot.expires_at_unix_secs)
        {
            return Err(MigrationError::EvidenceMismatch);
        }
    }
    Ok(())
}

fn serving_operation_ref(record: &MigrationRecord) -> String {
    format!(
        "migration:{}",
        projection_ref(b"migration", &record.migration_id)
    )
}

fn validate_serving_pair(
    record: &MigrationRecord,
    pair: &VerifiedCommittedChildLineagePair<'_>,
) -> MigrationResult<()> {
    let expected_lineage = serving_lineage_ref(&record.lineage_id)?;
    if (record.phase != MigrationPhase::Committed
        && record.phase != MigrationPhase::ServingTransferred)
        || record.source_authoritative
        || !record.target_authoritative
        || record.commit_log_index.is_none()
        || record.commit_certificate_sha256.is_none()
        || pair.shepherd().lineage_ref() != Some(expected_lineage.as_str())
        || pair.observatory().lineage_ref() != Some(expected_lineage.as_str())
    {
        return Err(MigrationError::EvidenceMismatch);
    }
    Ok(())
}

fn is_sha256_text(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_serving_receipt(
    record: &MigrationRecord,
    operation_ref: &str,
    receipt: &IntegratedSnapshotReceipt,
) -> MigrationResult<()> {
    let expected_lineage = serving_lineage_ref(&record.lineage_id)?;
    if receipt.operation_ref != operation_ref
        || receipt.outcome != IntegratedOutcome::Success
        || receipt.shepherd.lineage_ref != expected_lineage
        || receipt.observatory.lineage_ref != expected_lineage
        || !is_sha256_text(&receipt.input_sha256)
        || !is_sha256_text(&receipt.result_state_sha256)
        || !is_sha256_text(&receipt.receipt_sha256)
    {
        return Err(MigrationError::ServingTransferRejected);
    }
    Ok(())
}

fn serving_transfer_evidence(
    record: &MigrationRecord,
    operation_ref: &str,
    receipt: &IntegratedSnapshotReceipt,
) -> MigrationResult<[u8; 32]> {
    let receipt_bytes =
        serde_jcs::to_vec(receipt).map_err(|_| MigrationError::ServingTransferRejected)?;
    Ok(sha256_many(&[
        b"ADL-MIGRATION-SERVING-TRANSFER-V1\0",
        &record.request_sha256,
        operation_ref.as_bytes(),
        &receipt_bytes,
    ]))
}

fn record_serving_transfer(
    record: &mut MigrationRecord,
    operation_ref: String,
    receipt: IntegratedSnapshotReceipt,
) {
    record.serving_operation_ref = Some(operation_ref);
    record.serving_input_sha256 = Some(receipt.input_sha256);
    record.serving_result_state_sha256 = Some(receipt.result_state_sha256);
    record.serving_receipt_sha256 = Some(receipt.receipt_sha256);
}

fn serving_lineage_ref(lineage_id: &[u8]) -> MigrationResult<String> {
    let lineage = std::str::from_utf8(lineage_id).map_err(|_| MigrationError::EvidenceMismatch)?;
    Ok(hex::encode(Sha256::digest(format!(
        "ADL-SERVING-REF-V1\0lineage\0{lineage}"
    ))))
}

fn quiescence_request(
    record: &MigrationRecord,
    remaining_timeout_millis: u64,
) -> QuiescenceRequest<'_> {
    QuiescenceRequest {
        migration_id: &record.migration_id,
        lineage_id: &record.lineage_id,
        source_node_id: &record.source_node_id,
        source_guardian_id: &record.source_guardian_id,
        source_epoch: record.source_epoch,
        source_log_index: record.source_log_index,
        remaining_timeout_millis,
    }
}

fn exact_retry(
    record: &MigrationRecord,
    phase: MigrationPhase,
    evidence_sha256: [u8; 32],
) -> MigrationResult<MigrationRecord> {
    if record
        .history
        .last()
        .is_some_and(|entry| entry.phase == phase && entry.evidence_sha256 == evidence_sha256)
    {
        Ok(record.clone())
    } else {
        Err(MigrationError::ReplayMismatch)
    }
}

fn push_history(
    policy: &MigrationPolicy,
    record: &mut MigrationRecord,
    phase: MigrationPhase,
    evidence_sha256: [u8; 32],
) -> MigrationResult<()> {
    if record.history.len() >= policy.max_history_per_record {
        return Err(MigrationError::ResourceExhausted);
    }
    record.history.push(TransitionEvidence {
        phase,
        evidence_sha256,
    });
    Ok(())
}

fn collect_records(
    policy: &MigrationPolicy,
    records: Vec<MigrationRecord>,
) -> MigrationResult<BTreeMap<Vec<u8>, MigrationRecord>> {
    let mut collected = BTreeMap::new();
    for record in records {
        validate_record(policy, &record).map_err(|_| MigrationError::StateCorrupt)?;
        if collected
            .insert(record.migration_id.clone(), record)
            .is_some()
        {
            return Err(MigrationError::StateCorrupt);
        }
    }
    Ok(collected)
}

fn validate_record(policy: &MigrationPolicy, record: &MigrationRecord) -> MigrationResult<()> {
    let ordered_history = record
        .history
        .first()
        .is_some_and(|entry| entry.phase == MigrationPhase::Prepared)
        && record.history.windows(2).all(|pair| {
            pair[1].phase == MigrationPhase::Aborted
                || normal_phase_rank(pair[1].phase)
                    == normal_phase_rank(pair[0].phase).and_then(|rank| rank.checked_add(1))
        });
    let checkpointed = phase_at_least(record.phase, MigrationPhase::Checkpointed);
    let transferred = phase_at_least(record.phase, MigrationPhase::Transferred);
    let validated = phase_at_least(record.phase, MigrationPhase::Validated);
    let fenced = phase_at_least(record.phase, MigrationPhase::Fenced);
    let activated = phase_at_least(record.phase, MigrationPhase::Activated);
    let committed = phase_at_least(record.phase, MigrationPhase::Committed);
    let serving_transferred = record.phase == MigrationPhase::ServingTransferred;
    if !valid_bytes(&record.migration_id, policy.max_identity_bytes)
        || record.trust_domain != policy.trust_domain
        || !valid_text(&record.trust_domain, policy.max_identity_bytes)
        || !valid_bytes(&record.lineage_id, policy.max_identity_bytes)
        || !valid_bytes(&record.source_node_id, policy.max_identity_bytes)
        || !valid_bytes(&record.source_guardian_id, policy.max_identity_bytes)
        || !valid_bytes(&record.target_node_id, policy.max_identity_bytes)
        || !valid_bytes(&record.target_guardian_id, policy.max_identity_bytes)
        || record.source_node_id == record.target_node_id
        || record.source_guardian_id == record.target_guardian_id
        || record.source_epoch == 0
        || record.source_log_index == 0
        || record.placement_membership_epoch == 0
        || record.placement_log_index == 0
        || record.timeout_millis == 0
        || record.timeout_millis > policy.max_timeout_millis
        || record.deadline_millis
            != record
                .started_at_millis
                .checked_add(record.timeout_millis)
                .unwrap_or(0)
        || record.source_certificate_sha256 == [0; 32]
        || record.history.is_empty()
        || record.history.len() > policy.max_history_per_record
        || !ordered_history
        || record.history.last().map(|entry| entry.phase) != Some(record.phase)
        || record.source_authoritative && record.target_authoritative
        || matches!(
            record.phase,
            MigrationPhase::Fenced
                | MigrationPhase::Activated
                | MigrationPhase::Committed
                | MigrationPhase::ServingTransferred
        ) && record.source_authoritative
        || record.phase == MigrationPhase::Fenced && record.target_authoritative
        || matches!(
            record.phase,
            MigrationPhase::Activated
                | MigrationPhase::Committed
                | MigrationPhase::ServingTransferred
        ) && !record.target_authoritative
        || checkpointed
            && (record.catalog_entry_sha256.is_none()
                || record.snapshot_content_sha256.is_none()
                || record.snapshot_schema.is_none()
                || record.snapshot_byte_length.is_none()
                || record.snapshot_chunk_count.is_none()
                || record.snapshot_expiry_unix_secs.is_none())
        || transferred
            && (record.transfer_id.is_none() || record.transfer_manifest_sha256.is_none())
        || validated && record.restore_receipt_sha256.is_none()
        || fenced
            && (record.fence_request_id.is_none()
                || record.fence_epoch.is_none()
                || record.fence_log_index.is_none()
                || record.fence_certificate_sha256.is_none())
        || activated
            && (record.activation_log_index.is_none()
                || record.activation_certificate_sha256.is_none())
        || committed
            && (record.commit_log_index.is_none() || record.commit_certificate_sha256.is_none())
        || serving_transferred
            && (record
                .serving_operation_ref
                .as_deref()
                .is_none_or(|value| !valid_text(value, 128))
                || record
                    .serving_input_sha256
                    .as_deref()
                    .is_none_or(|value| !is_sha256_text(value))
                || record
                    .serving_result_state_sha256
                    .as_deref()
                    .is_none_or(|value| !is_sha256_text(value))
                || record
                    .serving_receipt_sha256
                    .as_deref()
                    .is_none_or(|value| !is_sha256_text(value)))
    {
        return Err(MigrationError::StateCorrupt);
    }
    Ok(())
}

fn normal_phase_rank(phase: MigrationPhase) -> Option<u8> {
    match phase {
        MigrationPhase::Prepared => Some(0),
        MigrationPhase::Quiesced => Some(1),
        MigrationPhase::Checkpointed => Some(2),
        MigrationPhase::Transferred => Some(3),
        MigrationPhase::Validated => Some(4),
        MigrationPhase::Fenced => Some(5),
        MigrationPhase::Activated => Some(6),
        MigrationPhase::Committed => Some(7),
        MigrationPhase::ServingTransferred => Some(8),
        MigrationPhase::Aborted => None,
    }
}

fn phase_at_least(current: MigrationPhase, threshold: MigrationPhase) -> bool {
    normal_phase_rank(current)
        .zip(normal_phase_rank(threshold))
        .is_some_and(|(current, threshold)| current >= threshold)
}

fn request_digest(
    request: &MigrationRequest,
    decision: &super::placement::PlacementDecision,
) -> [u8; 32] {
    sha256_many(&[
        b"ADL-MIGRATION-REQUEST-V1\0",
        &request.migration_id,
        request.trust_domain.as_bytes(),
        &request.lineage_id,
        &request.source_node_id,
        &request.source_guardian_id,
        &request.timeout_millis.to_be_bytes(),
        decision.node_id.as_bytes(),
        decision.guardian_id.as_bytes(),
        &decision.membership_epoch.to_be_bytes(),
        &decision.committed_log_index.to_be_bytes(),
        &decision.capability_sequence.to_be_bytes(),
        &decision.weather_sequence.to_be_bytes(),
    ])
}

fn unix_millis(seconds: i64, nanos: u32) -> Option<u64> {
    u64::try_from(seconds)
        .ok()?
        .checked_mul(1_000)?
        .checked_add(u64::from(nanos) / 1_000_000)
}

fn validate_root(path: &Path) -> MigrationResult<PathBuf> {
    if !path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(MigrationError::UnsafeStatePath);
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| MigrationError::UnsafeStatePath)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(MigrationError::UnsafeStatePath);
    }
    path.canonicalize()
        .map_err(|_| MigrationError::UnsafeStatePath)
}

fn checkpoint_matches(path: &Path, checkpoint: MigrationCheckpoint) -> bool {
    fs::read(path)
        .ok()
        .filter(|bytes| sha256(bytes) == checkpoint.state_sha256)
        .and_then(|bytes| serde_json::from_slice::<StateEnvelope>(&bytes).ok())
        .is_some_and(|envelope| envelope.body.generation == checkpoint.generation)
}

fn recover_interrupted_commit(
    root: &Path,
    checkpoint_authority: &dyn MigrationCheckpointAuthority,
) -> MigrationResult<()> {
    let lock_path = root.join(STATE_LOCK_FILE);
    if !lock_path.exists() {
        if root.join(STATE_BACKUP_FILE).exists() {
            return Err(MigrationError::DurabilityFailure);
        }
        return Ok(());
    }
    let mut lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| MigrationError::DurabilityFailure)?;
    lock.try_lock_exclusive()
        .map_err(|_| MigrationError::DurabilityFailure)?;
    let bytes = fs::read(&lock_path).map_err(|_| MigrationError::DurabilityFailure)?;
    if bytes.is_empty() {
        if root.join(STATE_BACKUP_FILE).exists() {
            return Err(MigrationError::DurabilityFailure);
        }
        FileExt::unlock(&lock).map_err(|_| MigrationError::DurabilityFailure)?;
        return Ok(());
    }
    let journal: CommitJournal =
        serde_json::from_slice(&bytes).map_err(|_| MigrationError::DurabilityFailure)?;
    let state_path = root.join(STATE_FILE);
    let backup_path = root.join(STATE_BACKUP_FILE);
    match checkpoint_authority.current()? {
        Some(checkpoint) if checkpoint == journal.expected => {
            if backup_path.exists() && checkpoint_matches(&backup_path, journal.expected) {
                if state_path.exists() {
                    fs::remove_file(&state_path).map_err(|_| MigrationError::DurabilityFailure)?;
                }
                fs::rename(&backup_path, &state_path)
                    .map_err(|_| MigrationError::DurabilityFailure)?;
            } else if !checkpoint_matches(&state_path, journal.expected) {
                return Err(MigrationError::Rollback);
            }
        }
        Some(checkpoint) if checkpoint == journal.next => {
            if !checkpoint_matches(&state_path, journal.next) {
                return Err(MigrationError::Rollback);
            }
            if backup_path.exists() {
                fs::remove_file(&backup_path).map_err(|_| MigrationError::DurabilityFailure)?;
            }
        }
        _ => return Err(MigrationError::Rollback),
    }
    if root.join(".migration-state.tmp").exists() {
        fs::remove_file(root.join(".migration-state.tmp"))
            .map_err(|_| MigrationError::DurabilityFailure)?;
    }
    sync_directory(root)?;
    lock.set_len(0)
        .and_then(|_| lock.seek(SeekFrom::Start(0)).map(|_| ()))
        .and_then(|_| lock.sync_all())
        .map_err(|_| MigrationError::DurabilityFailure)?;
    FileExt::unlock(&lock).map_err(|_| MigrationError::DurabilityFailure)
}

fn sync_directory(root: &Path) -> MigrationResult<()> {
    File::open(root)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| MigrationError::DurabilityFailure)
}

fn write_atomic(root: &Path, bytes: &[u8]) -> MigrationResult<()> {
    let temporary = root.join(".migration-state.tmp");
    let final_path = root.join(STATE_FILE);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| MigrationError::DurabilityFailure)?;
    if file.write_all(bytes).is_err() || file.sync_all().is_err() {
        return Err(MigrationError::DurabilityFailure);
    }
    fs::rename(&temporary, &final_path).map_err(|_| MigrationError::DurabilityFailure)?;
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

#[cfg(all(test, feature = "internal-test-fixtures"))]
mod serving_transfer_tests {
    use super::*;
    use crate::distributed::{
        authority_protocol::test_observatory_published_authority_for_operation,
        observatory_serving_eligibility::ObservatoryEligibilityStore,
        polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
        serving_authority::{
            ObservatoryBindingFixture, ObservatoryIdentifierField, ObservatoryTransitionAction,
            VerifiedServingAuthorityCut,
        },
        shepherd_serving_eligibility::{
            verify_committed_child_lineage_pair, ShepherdEligibilityStore,
        },
    };
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    const DOMAIN: &str = "polis.example";
    const LINEAGE: &[u8] = b"lineage-a";

    #[derive(Debug, Default)]
    struct MigrationAuthority(Mutex<Option<MigrationCheckpoint>>);

    impl MigrationCheckpointAuthority for MigrationAuthority {
        fn current(&self) -> MigrationResult<Option<MigrationCheckpoint>> {
            Ok(*self.0.lock().unwrap())
        }

        fn compare_and_swap(
            &self,
            expected: Option<MigrationCheckpoint>,
            next: MigrationCheckpoint,
        ) -> MigrationResult<()> {
            let mut current = self.0.lock().unwrap();
            if *current != expected {
                return Err(MigrationError::Rollback);
            }
            *current = Some(next);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct TestClock(u64);

    impl MigrationClock for TestClock {
        fn now_millis(&self) -> MigrationResult<u64> {
            Ok(self.0)
        }
    }

    #[derive(Debug, Default)]
    struct ServingAuthority(Mutex<BTreeMap<String, ConsensusCheckpoint>>);

    impl ConsensusCheckpointAuthority for ServingAuthority {
        fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
            Ok(self.0.lock().unwrap().get(object).cloned())
        }

        fn compare_and_swap(
            &self,
            expected: Option<&ConsensusCheckpoint>,
            candidate: &ConsensusCheckpoint,
        ) -> Result<(), PolisRuntimeError> {
            let mut current = self.0.lock().unwrap();
            if current.get(&candidate.object) != expected {
                return Err(PolisRuntimeError::StateRegression);
            }
            current.insert(candidate.object.clone(), candidate.clone());
            Ok(())
        }
    }

    fn observatory_input_for(
        operation: &str,
        lineage: &str,
    ) -> (
        crate::distributed::authority_protocol::PublishedAuthorityResult,
        VerifiedServingAuthorityCut,
    ) {
        let mut fixture = ObservatoryBindingFixture::new(operation);
        fixture.set_invalid_identifier(ObservatoryIdentifierField::LineageId, lineage);
        fixture.set_invalid_identifier(ObservatoryIdentifierField::OperationId, operation);
        fixture.set_operation(operation, 2);
        fixture.set_integers(2, 1, 1);
        fixture.set_transition(ObservatoryTransitionAction::Acquire, None);
        (
            test_observatory_published_authority_for_operation(
                fixture.artifact_bytes(),
                operation,
                2,
            ),
            VerifiedServingAuthorityCut::fixture_from_observatory(&fixture),
        )
    }

    fn shepherd_cut_for(lineage: &str) -> VerifiedServingAuthorityCut {
        VerifiedServingAuthorityCut::fixture(
            lineage.into(),
            7,
            format!("owner-{lineage}"),
            9,
            format!("lease-{lineage}"),
            "11".repeat(32),
            "22".repeat(32),
            "33".repeat(32),
        )
    }

    fn committed_pair_for_lineage(
        root: &Path,
        authority: Arc<ServingAuthority>,
        lineage: &str,
    ) -> (
        crate::distributed::shepherd_serving_eligibility::SealedShepherdCommittedProjection,
        crate::distributed::observatory_serving_eligibility::SealedObservatoryCommittedProjection,
    ) {
        let shepherd_root = root.join(format!("shepherd-{lineage}"));
        let observatory_root = root.join(format!("observatory-{lineage}"));
        fs::create_dir(&shepherd_root).unwrap();
        fs::create_dir(&observatory_root).unwrap();
        let mut shepherd =
            ShepherdEligibilityStore::open(&shepherd_root, authority.clone(), 8).unwrap();
        let shepherd_cut = shepherd_cut_for(lineage);
        shepherd
            .acquire(
                "shepherd-acquire",
                "shepherd-a",
                b"raw-secret-permit",
                &shepherd_cut,
                100,
                1,
            )
            .unwrap();
        let mut observatory =
            ObservatoryEligibilityStore::open(&observatory_root, authority, 8).unwrap();
        let (published, cut) = observatory_input_for("observatory-acquire", lineage);
        observatory
            .apply(&published, &cut, 1_700_000_000, 123_456_789)
            .unwrap();
        (
            shepherd.committed_projection().unwrap().unwrap(),
            observatory.committed_projection().unwrap().unwrap(),
        )
    }

    fn committed_record() -> MigrationRecord {
        MigrationRecord {
            migration_id: b"migration-1".to_vec(),
            request_sha256: [1; 32],
            trust_domain: DOMAIN.into(),
            lineage_id: LINEAGE.to_vec(),
            source_node_id: b"node-0".to_vec(),
            source_guardian_id: b"guardian-0".to_vec(),
            source_epoch: 1,
            source_log_index: 11,
            source_certificate_sha256: [2; 32],
            target_node_id: b"node-1".to_vec(),
            target_guardian_id: b"guardian-1".to_vec(),
            placement_membership_epoch: 1,
            placement_log_index: 11,
            placement_capability_sequence: 1,
            placement_weather_sequence: 1,
            timeout_millis: 10_000,
            started_at_millis: 1_000,
            deadline_millis: 11_000,
            phase: MigrationPhase::Committed,
            source_authoritative: false,
            target_authoritative: true,
            quiescence_sha256: Some([3; 32]),
            catalog_entry_sha256: Some([4; 32]),
            snapshot_content_sha256: Some([5; 32]),
            snapshot_schema: Some("adl.snapshot.fixture.v1".into()),
            snapshot_byte_length: Some(128),
            snapshot_chunk_count: Some(1),
            snapshot_expiry_unix_secs: Some(1_700_000_000),
            transfer_id: Some(b"transfer-1".to_vec()),
            transfer_manifest_sha256: Some([6; 32]),
            restore_receipt_sha256: Some([7; 32]),
            fence_request_id: Some(b"fence-1".to_vec()),
            fence_epoch: Some(2),
            fence_log_index: Some(12),
            fence_certificate_sha256: Some([8; 32]),
            activation_log_index: Some(13),
            activation_certificate_sha256: Some([9; 32]),
            commit_log_index: Some(14),
            commit_certificate_sha256: Some([10; 32]),
            serving_operation_ref: None,
            serving_input_sha256: None,
            serving_result_state_sha256: None,
            serving_receipt_sha256: None,
            history: vec![
                MigrationPhase::Prepared,
                MigrationPhase::Quiesced,
                MigrationPhase::Checkpointed,
                MigrationPhase::Transferred,
                MigrationPhase::Validated,
                MigrationPhase::Fenced,
                MigrationPhase::Activated,
                MigrationPhase::Committed,
            ]
            .into_iter()
            .enumerate()
            .map(|(index, phase)| TransitionEvidence {
                phase,
                evidence_sha256: [index as u8 + 1; 32],
            })
            .collect(),
        }
    }

    fn seeded_store(root: &Path, authority: Arc<MigrationAuthority>) -> MigrationStore {
        let mut store = MigrationStore::create(
            root,
            MigrationPolicy::new(DOMAIN).unwrap(),
            authority,
            Arc::new(TestClock(2_000)),
        )
        .unwrap();
        store
            .seed_record_for_snapshot_test(committed_record())
            .unwrap();
        store
    }

    #[test]
    fn committed_migration_transfers_verified_serving_authority_and_retries_exactly() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let migration_root = root.join("migration");
        let serving_root = root.join("integrated");
        fs::create_dir(&migration_root).unwrap();
        fs::create_dir(&serving_root).unwrap();
        let migration_authority = Arc::new(MigrationAuthority::default());
        let serving_authority = Arc::new(ServingAuthority::default());
        let mut store = seeded_store(&migration_root, migration_authority.clone());
        let (shepherd, observatory) = committed_pair_for_lineage(
            &root,
            serving_authority.clone(),
            std::str::from_utf8(LINEAGE).unwrap(),
        );
        let pair = verify_committed_child_lineage_pair(&shepherd, &observatory).unwrap();
        let mut integrated = IntegratedServingAuthoritySnapshotStore::open(
            &serving_root,
            serving_authority.clone(),
            8,
        )
        .unwrap();

        let transferred = store
            .transfer_serving_authority(b"migration-1", &mut integrated, &pair)
            .unwrap();
        assert_eq!(transferred.phase, MigrationPhase::ServingTransferred);
        assert!(transferred.serving_operation_ref.is_some());
        assert!(transferred.serving_receipt_sha256.is_some());
        assert_eq!(transferred.history.len(), 9);
        let generation = store.checkpoint().generation;
        assert_eq!(
            store
                .transfer_serving_authority(b"migration-1", &mut integrated, &pair)
                .unwrap(),
            transferred
        );
        assert_eq!(store.checkpoint().generation, generation);

        let recovery_root = root.join("migration-recovery");
        fs::create_dir(&recovery_root).unwrap();
        let mut recovery_store =
            seeded_store(&recovery_root, Arc::new(MigrationAuthority::default()));
        assert_eq!(
            recovery_store
                .transfer_serving_authority(b"migration-1", &mut integrated, &pair)
                .unwrap(),
            transferred
        );

        drop(store);
        drop(integrated);
        let mut store = MigrationStore::open(
            &migration_root,
            MigrationPolicy::new(DOMAIN).unwrap(),
            migration_authority,
            Arc::new(TestClock(2_000)),
        )
        .unwrap();
        let mut integrated =
            IntegratedServingAuthoritySnapshotStore::open(&serving_root, serving_authority, 8)
                .unwrap();
        assert_eq!(
            store
                .transfer_serving_authority(b"migration-1", &mut integrated, &pair)
                .unwrap(),
            transferred
        );
    }

    #[test]
    fn serving_transfer_rejects_uncommitted_and_wrong_lineage_without_receipt() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().canonicalize().unwrap();
        let migration_root = root.join("migration");
        let serving_root = root.join("integrated");
        fs::create_dir(&migration_root).unwrap();
        fs::create_dir(&serving_root).unwrap();
        let migration_authority = Arc::new(MigrationAuthority::default());
        let serving_authority = Arc::new(ServingAuthority::default());
        let mut store = seeded_store(&migration_root, migration_authority);
        store
            .records
            .get_mut(b"migration-1".as_slice())
            .unwrap()
            .phase = MigrationPhase::Validated;
        let (wrong_shepherd, wrong_observatory) =
            committed_pair_for_lineage(&root, serving_authority.clone(), "lineage-other");
        let wrong_pair =
            verify_committed_child_lineage_pair(&wrong_shepherd, &wrong_observatory).unwrap();
        let mut integrated =
            IntegratedServingAuthoritySnapshotStore::open(&serving_root, serving_authority, 8)
                .unwrap();
        assert_eq!(
            store.transfer_serving_authority(b"migration-1", &mut integrated, &wrong_pair),
            Err(MigrationError::InvalidTransition)
        );
        store
            .records
            .get_mut(b"migration-1".as_slice())
            .unwrap()
            .phase = MigrationPhase::Committed;
        assert_eq!(
            store.transfer_serving_authority(b"migration-1", &mut integrated, &wrong_pair),
            Err(MigrationError::EvidenceMismatch)
        );
    }
}
