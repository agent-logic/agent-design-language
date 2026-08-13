use std::{
    collections::BTreeMap,
    fmt,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::lease::{
    certificate_body_sha256, verify_certificate, AuthorityMembership, LeaseState, OperationClass,
    ACTIVATION_DOMAIN,
};

pub const FENCING_STATE_SCHEMA: &str = "adl.distributed.fencing_state.v1";
const STATE_FILE: &str = "fencing-state.json";
const STATE_LOCK_FILE: &str = ".fencing-state.lock";
const MAX_IDENTITY_BYTES: usize = 128;
const MAX_REQUEST_ID_BYTES: usize = 128;

mod raw_access {
    const FENCING_STORE_ACCESS_MAGIC: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x46, 0x45, 0x4e, 0x43, 0x49, 0x4e, 0x47, 0x2d, 0x53, 0x54, 0x4f,
        0x52, 0x45, 0x2d, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x56, 0x31, 0x2d, 0x53, 0x45,
        0x04, 0x5a,
    ];

    #[derive(Debug)]
    struct FencingStoreAccessSeal {
        magic: [u8; 32],
    }

    static AUTHORITY_BOUND_SEAL: FencingStoreAccessSeal = FencingStoreAccessSeal {
        magic: FENCING_STORE_ACCESS_MAGIC,
    };

    #[cfg(any(test, feature = "internal-test-fixtures"))]
    static TEST_FIXTURE_SEAL: FencingStoreAccessSeal = FencingStoreAccessSeal {
        magic: FENCING_STORE_ACCESS_MAGIC,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct FencingStoreAccess {
        seal: &'static FencingStoreAccessSeal,
    }

    pub(crate) const AUTHORITY_BOUND: FencingStoreAccess = FencingStoreAccess {
        seal: &AUTHORITY_BOUND_SEAL,
    };

    #[cfg(test)]
    #[doc(hidden)]
    pub(crate) const TEST_FIXTURE: FencingStoreAccess = FencingStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    #[cfg(all(not(test), feature = "internal-test-fixtures"))]
    #[doc(hidden)]
    pub const TEST_FIXTURE: FencingStoreAccess = FencingStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    pub(super) fn validate(access: &FencingStoreAccess) -> bool {
        #[cfg(any(test, feature = "internal-test-fixtures"))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)
            || std::ptr::eq(access.seal, &TEST_FIXTURE_SEAL);
        #[cfg(not(any(test, feature = "internal-test-fixtures")))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL);
        known_seal && access.seal.magic == FENCING_STORE_ACCESS_MAGIC
    }
}

pub use raw_access::FencingStoreAccess;
#[allow(unused_imports)]
pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_FENCING_ACCESS;
#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use raw_access::TEST_FIXTURE as TEST_FENCING_STORE_ACCESS;
#[cfg(all(not(test), feature = "internal-test-fixtures"))]
#[doc(hidden)]
#[allow(unused_imports)]
pub use raw_access::TEST_FIXTURE as TEST_FENCING_STORE_ACCESS;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FencingError {
    InvalidPolicy,
    UnsafeStatePath,
    StateMissing,
    StateCorrupt,
    ResourceExhausted,
    MembershipRequired,
    StaleMembership,
    StaleAppliedIndex,
    UnauthorizedOperation,
    InvalidCertificate,
    HolderMismatch,
    StaleEpoch,
    EpochGap,
    ReplayMismatch,
    Fenced,
    SafetyWindow,
    Rollback,
    ActivationPossession,
    LeaseExpired,
    DurabilityFailure,
    RevisionDrift,
}

impl FencingError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::UnsafeStatePath => "unsafe_state_path",
            Self::StateMissing => "state_missing",
            Self::StateCorrupt => "state_corrupt",
            Self::ResourceExhausted => "resource_exhausted",
            Self::MembershipRequired => "membership_required",
            Self::StaleMembership => "stale_membership",
            Self::StaleAppliedIndex => "stale_applied_index",
            Self::UnauthorizedOperation => "unauthorized_operation",
            Self::InvalidCertificate => "invalid_certificate",
            Self::HolderMismatch => "holder_mismatch",
            Self::StaleEpoch => "stale_epoch",
            Self::EpochGap => "epoch_gap",
            Self::ReplayMismatch => "replay_mismatch",
            Self::Fenced => "fenced",
            Self::SafetyWindow => "safety_window",
            Self::Rollback => "rollback",
            Self::ActivationPossession => "activation_possession_failed",
            Self::LeaseExpired => "lease_expired",
            Self::DurabilityFailure => "durability_failure",
            Self::RevisionDrift => "revision_drift",
        }
    }
}

impl fmt::Display for FencingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for FencingError {}
pub type FencingResult<T> = Result<T, FencingError>;

fn validate_raw_access(access: &FencingStoreAccess) -> FencingResult<()> {
    raw_access::validate(access)
        .then_some(())
        .ok_or(FencingError::UnauthorizedOperation)
}

#[derive(Clone, Debug)]
pub struct FencingPolicy {
    pub max_lineages: usize,
    pub max_receipts: usize,
    pub max_state_bytes: usize,
    pub max_clock_uncertainty_millis: u64,
    pub message_delay_margin_millis: u64,
}

impl FencingPolicy {
    pub fn validate(&self) -> FencingResult<()> {
        if self.max_lineages == 0
            || self.max_lineages > 4096
            || self.max_receipts == 0
            || self.max_receipts > 16_384
            || !(1024..=16 * 1024 * 1024).contains(&self.max_state_bytes)
            || self.max_clock_uncertainty_millis > 60_000
            || self.message_delay_margin_millis > 60_000
        {
            return Err(FencingError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FenceReceipt {
    pub request_id: Vec<u8>,
    pub request_sha256: [u8; 32],
    pub trust_domain_id: Vec<u8>,
    pub lineage_id: Vec<u8>,
    pub epoch: u64,
    pub committed_log_index: u64,
    pub voter_set_generation: u64,
    pub operation_class: u32,
    pub certificate_sha256: [u8; 32],
    pub safety_deadline_unix_millis: u64,
}

#[derive(Clone, Debug)]
pub struct FenceCommit<'a> {
    pub request_id: &'a [u8],
    pub certificate_bytes: &'a [u8],
    pub membership: Option<&'a AuthorityMembership>,
    pub current_lease: &'a LeaseState,
    pub now_unix_seconds: i64,
}

#[derive(Clone, Debug)]
pub struct ActiveLeaseCheck<'a> {
    pub membership: Option<&'a AuthorityMembership>,
    pub lease: &'a LeaseState,
    pub applied_log_index: u64,
    pub now_unix_seconds: i64,
    pub now_unix_millis: u64,
    pub now_elapsed_millis: u64,
    pub activation_proof: &'a [u8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FencingCheckpoint {
    pub generation: u64,
    pub state_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FencingAuthorityRevision {
    checkpoint_generation: u64,
    state_sha256: [u8; 32],
}

impl FencingAuthorityRevision {
    pub fn checkpoint_generation(&self) -> u64 {
        self.checkpoint_generation
    }

    pub fn state_sha256(&self) -> [u8; 32] {
        self.state_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedFencingRow {
    lineage_ref: String,
    epoch: u64,
    committed_log_index: u64,
    voter_set_generation: u64,
    operation_class: u32,
}

impl RedactedFencingRow {
    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn voter_set_generation(&self) -> u64 {
        self.voter_set_generation
    }

    pub fn operation_class(&self) -> u32 {
        self.operation_class
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedFencingSnapshot {
    trust_domain: String,
    revision: FencingAuthorityRevision,
    rows: Vec<RedactedFencingRow>,
}

impl RedactedFencingSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn revision(&self) -> FencingAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedFencingRow> {
        self.rows.iter()
    }
}

pub trait FencingCheckpointAuthority: fmt::Debug + Send + Sync {
    fn current(&self) -> FencingResult<Option<FencingCheckpoint>>;

    fn compare_and_swap(
        &self,
        expected: Option<FencingCheckpoint>,
        next: FencingCheckpoint,
    ) -> FencingResult<()>;
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateBody {
    schema: String,
    generation: u64,
    floors: Vec<FenceReceipt>,
    receipts: Vec<FenceReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateEnvelope {
    body: StateBody,
    digest: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct FencingStore {
    root: PathBuf,
    policy: FencingPolicy,
    floors: BTreeMap<Vec<u8>, FenceReceipt>,
    receipts: BTreeMap<Vec<u8>, FenceReceipt>,
    generation: u64,
    state_sha256: [u8; 32],
    checkpoint_authority: Arc<dyn FencingCheckpointAuthority>,
    #[cfg(test)]
    fail_lock_cleanup: std::cell::Cell<bool>,
}

struct PersistOutcome {
    checkpoint: FencingCheckpoint,
    post_commit_error: Option<FencingError>,
}

impl FencingStore {
    pub fn create(
        access: &FencingStoreAccess,
        root: impl AsRef<Path>,
        policy: FencingPolicy,
        checkpoint_authority: Arc<dyn FencingCheckpointAuthority>,
    ) -> FencingResult<Self> {
        validate_raw_access(access)?;
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        let path = root.join(STATE_FILE);
        if path.exists() {
            return Err(FencingError::StateCorrupt);
        }
        let mut store = Self {
            root,
            policy,
            floors: BTreeMap::new(),
            receipts: BTreeMap::new(),
            generation: 0,
            state_sha256: [0; 32],
            checkpoint_authority,
            #[cfg(test)]
            fail_lock_cleanup: std::cell::Cell::new(false),
        };
        store.state_sha256 = store.persist(0, &store.floors, &store.receipts)?;
        store
            .checkpoint_authority
            .compare_and_swap(None, store.checkpoint())?;
        Ok(store)
    }

    pub fn open(
        access: &FencingStoreAccess,
        root: impl AsRef<Path>,
        policy: FencingPolicy,
        checkpoint_authority: Arc<dyn FencingCheckpointAuthority>,
    ) -> FencingResult<Self> {
        validate_raw_access(access)?;
        policy.validate()?;
        let root = validate_root(root.as_ref())?;
        let path = root.join(STATE_FILE);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                FencingError::StateMissing
            } else {
                FencingError::UnsafeStatePath
            }
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(FencingError::UnsafeStatePath);
        }
        let bytes = fs::read(&path).map_err(|_| FencingError::StateCorrupt)?;
        if bytes.is_empty() || bytes.len() > policy.max_state_bytes {
            return Err(FencingError::ResourceExhausted);
        }
        let envelope: StateEnvelope =
            serde_json::from_slice(&bytes).map_err(|_| FencingError::StateCorrupt)?;
        let body_bytes =
            serde_jcs::to_vec(&envelope.body).map_err(|_| FencingError::StateCorrupt)?;
        if envelope.body.schema != FENCING_STATE_SCHEMA
            || envelope.digest != <[u8; 32]>::from(Sha256::digest(body_bytes))
            || serde_jcs::to_vec(&envelope).map_err(|_| FencingError::StateCorrupt)? != bytes
            || envelope.body.floors.len() > policy.max_lineages
            || envelope.body.receipts.len() > policy.max_receipts
        {
            return Err(FencingError::StateCorrupt);
        }
        let state_sha256 = <[u8; 32]>::from(Sha256::digest(&bytes));
        let checkpoint = checkpoint_authority
            .current()?
            .ok_or(FencingError::Rollback)?;
        if envelope.body.generation != checkpoint.generation
            || state_sha256 != checkpoint.state_sha256
        {
            return Err(FencingError::Rollback);
        }
        let generation = envelope.body.generation;
        let floors = collect_records(envelope.body.floors, true)?;
        let receipts = collect_records(envelope.body.receipts, false)?;
        Ok(Self {
            root,
            policy,
            floors,
            receipts,
            generation,
            state_sha256,
            checkpoint_authority,
            #[cfg(test)]
            fail_lock_cleanup: std::cell::Cell::new(false),
        })
    }

    pub fn checkpoint(&self) -> FencingCheckpoint {
        FencingCheckpoint {
            generation: self.generation,
            state_sha256: self.state_sha256,
        }
    }

    pub fn floor(&self, lineage_id: &[u8]) -> Option<&FenceReceipt> {
        self.floors.get(lineage_id)
    }

    pub fn authority_revision(&self) -> FencingResult<FencingAuthorityRevision> {
        self.verify_current_state()?;
        Ok(FencingAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        })
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: FencingAuthorityRevision,
        membership: &AuthorityMembership,
    ) -> FencingResult<RedactedFencingSnapshot> {
        self.verify_current_state()?;
        let revision = FencingAuthorityRevision {
            checkpoint_generation: self.generation,
            state_sha256: self.state_sha256,
        };
        if revision != expected_revision {
            return Err(FencingError::RevisionDrift);
        }
        if self.floors.len() > self.policy.max_lineages {
            return Err(FencingError::ResourceExhausted);
        }
        let trust_domain = std::str::from_utf8(&membership.trust_domain_id)
            .map_err(|_| FencingError::StaleMembership)?
            .to_owned();
        let rows = self
            .floors
            .values()
            .map(|floor| {
                if floor.trust_domain_id != membership.trust_domain_id
                    || floor.voter_set_generation > membership.voter_set_generation
                    || floor.committed_log_index > membership.committed_log_index
                {
                    return Err(FencingError::StaleMembership);
                }
                Ok(RedactedFencingRow {
                    lineage_ref: projection_ref(b"lineage", &floor.lineage_id),
                    epoch: floor.epoch,
                    committed_log_index: floor.committed_log_index,
                    voter_set_generation: floor.voter_set_generation,
                    operation_class: floor.operation_class,
                })
            })
            .collect::<FencingResult<Vec<_>>>()?;
        if self.authority_revision()? != revision {
            return Err(FencingError::RevisionDrift);
        }
        Ok(RedactedFencingSnapshot {
            trust_domain,
            revision,
            rows,
        })
    }

    #[cfg(test)]
    pub(crate) fn seed_floor_for_snapshot_test(
        &mut self,
        floor: FenceReceipt,
    ) -> FencingResult<()> {
        if self.floors.len() >= self.policy.max_lineages || floor.lineage_id.is_empty() {
            return Err(FencingError::ResourceExhausted);
        }
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(FencingError::ResourceExhausted)?;
        let mut floors = self.floors.clone();
        floors.insert(floor.lineage_id.clone(), floor);
        let outcome = self.persist_next(generation, &floors, &self.receipts)?;
        self.floors = floors;
        self.generation = outcome.checkpoint.generation;
        self.state_sha256 = outcome.checkpoint.state_sha256;
        outcome.post_commit_error.map_or(Ok(()), Err)
    }

    pub fn commit(
        &mut self,
        access: &FencingStoreAccess,
        request: FenceCommit<'_>,
    ) -> FencingResult<FenceReceipt> {
        validate_raw_access(access)?;
        if request.request_id.is_empty()
            || request.request_id.len() > MAX_REQUEST_ID_BYTES
            || !valid_identity(&request.current_lease.lineage_id)
        {
            return Err(FencingError::ResourceExhausted);
        }
        let membership = request.membership.ok_or(FencingError::MembershipRequired)?;
        let verified = verify_certificate(
            request.certificate_bytes,
            membership,
            request.now_unix_seconds,
        )
        .map_err(|_| FencingError::InvalidCertificate)?;
        let body = verified.body;
        let operation = body.operation_class;
        if operation != OperationClass::Fence as u32 && operation != OperationClass::Revoke as u32 {
            return Err(FencingError::UnauthorizedOperation);
        }
        if body.voter_set_generation != membership.voter_set_generation {
            return Err(FencingError::StaleMembership);
        }
        if body.committed_log_index != membership.committed_log_index {
            return Err(FencingError::StaleAppliedIndex);
        }
        let verified_lease = verify_certificate(
            &request.current_lease.certificate_bytes,
            membership,
            request.now_unix_seconds,
        )
        .map_err(|_| FencingError::InvalidCertificate)?;
        validate_lease_binding(&body, request.current_lease, &verified_lease.body)?;
        let expected_epoch = if operation == OperationClass::Fence as u32 {
            request
                .current_lease
                .epoch
                .checked_add(1)
                .ok_or(FencingError::ResourceExhausted)?
        } else {
            request.current_lease.epoch
        };
        if body.epoch < expected_epoch {
            return Err(FencingError::StaleEpoch);
        }
        if body.epoch != expected_epoch {
            return Err(FencingError::EpochGap);
        }
        let safety_deadline_unix_millis = request
            .current_lease
            .deadline_unix_millis
            .checked_add(self.policy.max_clock_uncertainty_millis)
            .and_then(|value| value.checked_add(self.policy.message_delay_margin_millis))
            .ok_or(FencingError::ResourceExhausted)?;
        let certificate_sha256 = Sha256::digest(request.certificate_bytes).into();
        let request_sha256 = request_digest(
            request.request_id,
            certificate_sha256,
            request.current_lease,
            membership,
        );
        if let Some(existing) = self.receipts.get(request.request_id) {
            return if existing.request_sha256 == request_sha256 {
                Ok(existing.clone())
            } else {
                Err(FencingError::ReplayMismatch)
            };
        }
        if self.receipts.len() >= self.policy.max_receipts {
            return Err(FencingError::ResourceExhausted);
        }
        if self
            .floors
            .get(&body.lineage_id)
            .is_some_and(|floor| request.current_lease.epoch < floor.epoch)
        {
            return Err(FencingError::StaleEpoch);
        }
        if let Some(floor) = self.floors.get(&body.lineage_id) {
            if body.epoch < floor.epoch {
                return Err(FencingError::StaleEpoch);
            }
            if body.committed_log_index <= floor.committed_log_index {
                return Err(FencingError::StaleAppliedIndex);
            }
        } else if self.floors.len() >= self.policy.max_lineages {
            return Err(FencingError::ResourceExhausted);
        }
        let receipt = FenceReceipt {
            request_id: request.request_id.to_vec(),
            request_sha256,
            trust_domain_id: body.trust_domain_id,
            lineage_id: body.lineage_id,
            epoch: body.epoch,
            committed_log_index: body.committed_log_index,
            voter_set_generation: body.voter_set_generation,
            operation_class: operation,
            certificate_sha256,
            safety_deadline_unix_millis,
        };
        let mut floors = self.floors.clone();
        let mut receipts = self.receipts.clone();
        floors.insert(receipt.lineage_id.clone(), receipt.clone());
        receipts.insert(receipt.request_id.clone(), receipt.clone());
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(FencingError::ResourceExhausted)?;
        let outcome = self.persist_next(generation, &floors, &receipts)?;
        self.floors = floors;
        self.receipts = receipts;
        self.generation = outcome.checkpoint.generation;
        self.state_sha256 = outcome.checkpoint.state_sha256;
        if let Some(error) = outcome.post_commit_error {
            return Err(error);
        }
        Ok(receipt)
    }

    pub fn authorize_active_lease(
        &self,
        access: &FencingStoreAccess,
        check: ActiveLeaseCheck<'_>,
    ) -> FencingResult<()> {
        validate_raw_access(access)?;
        let lock_path = self.acquire_state_lock()?;
        let result = self
            .verify_current_state()
            .and_then(|_| self.authorize_active_lease_inner(check));
        if self.release_state_lock(lock_path).is_err() {
            return Err(FencingError::DurabilityFailure);
        }
        result
    }

    fn authorize_active_lease_inner(&self, check: ActiveLeaseCheck<'_>) -> FencingResult<()> {
        let membership = check.membership.ok_or(FencingError::MembershipRequired)?;
        if check.applied_log_index != membership.committed_log_index {
            return Err(FencingError::StaleAppliedIndex);
        }
        if check.lease.revoked {
            return Err(FencingError::Fenced);
        }
        let verified = verify_certificate(
            &check.lease.certificate_bytes,
            membership,
            check.now_unix_seconds,
        )
        .map_err(|_| FencingError::InvalidCertificate)?;
        let body = verified.body;
        if body.voter_set_generation != membership.voter_set_generation {
            return Err(FencingError::StaleMembership);
        }
        if body.committed_log_index != check.applied_log_index
            || check.lease.committed_log_index != check.applied_log_index
        {
            return Err(FencingError::StaleAppliedIndex);
        }
        validate_lease_binding(&body, check.lease, &body)?;
        if !matches!(
            body.operation_class,
            value if value == OperationClass::LeaseGrant as u32
                || value == OperationClass::LeaseRenewal as u32
                || value == OperationClass::Activate as u32
                || value == OperationClass::OwnerCommit as u32
        ) {
            return Err(FencingError::UnauthorizedOperation);
        }
        if let Some(floor) = self.floors.get(&check.lease.lineage_id) {
            if check.lease.epoch < floor.epoch {
                return Err(FencingError::Fenced);
            }
            if check.lease.epoch == floor.epoch {
                if floor.operation_class == OperationClass::Revoke as u32
                    || body.operation_class != OperationClass::Activate as u32
                    || check.lease.committed_log_index <= floor.committed_log_index
                {
                    return Err(FencingError::Fenced);
                }
                if check.now_unix_millis < floor.safety_deadline_unix_millis {
                    return Err(FencingError::SafetyWindow);
                }
            }
        }
        if check.now_unix_millis >= check.lease.deadline_unix_millis
            || check.now_elapsed_millis >= check.lease.deadline_elapsed_millis
        {
            return Err(FencingError::LeaseExpired);
        }
        verify_activation_possession(&body, check.lease, check.activation_proof)?;
        Ok(())
    }

    fn persist_next(
        &self,
        generation: u64,
        floors: &BTreeMap<Vec<u8>, FenceReceipt>,
        receipts: &BTreeMap<Vec<u8>, FenceReceipt>,
    ) -> FencingResult<PersistOutcome> {
        let lock_path = self.acquire_state_lock()?;
        let result = self.verify_current_state().and_then(|_| {
            let state_sha256 = self.persist(generation, floors, receipts)?;
            let checkpoint = FencingCheckpoint {
                generation,
                state_sha256,
            };
            let authority_result = self
                .checkpoint_authority
                .compare_and_swap(Some(self.checkpoint()), checkpoint);
            Ok((checkpoint, authority_result.err()))
        });
        match result {
            Ok((checkpoint, authority_error)) => {
                let cleanup_error = self.release_state_lock(lock_path).err();
                Ok(PersistOutcome {
                    checkpoint,
                    post_commit_error: cleanup_error
                        .map(|_| FencingError::DurabilityFailure)
                        .or(authority_error),
                })
            }
            Err(error) => {
                if self.release_state_lock(lock_path).is_err() {
                    Err(FencingError::DurabilityFailure)
                } else {
                    Err(error)
                }
            }
        }
    }

    fn verify_current_state(&self) -> FencingResult<()> {
        let current =
            fs::read(self.root.join(STATE_FILE)).map_err(|_| FencingError::DurabilityFailure)?;
        if <[u8; 32]>::from(Sha256::digest(&current)) != self.state_sha256 {
            return Err(FencingError::Rollback);
        }
        let envelope: StateEnvelope =
            serde_json::from_slice(&current).map_err(|_| FencingError::StateCorrupt)?;
        if envelope.body.generation != self.generation
            || self.checkpoint_authority.current()? != Some(self.checkpoint())
        {
            return Err(FencingError::Rollback);
        }
        Ok(())
    }

    fn acquire_state_lock(&self) -> FencingResult<PathBuf> {
        let lock_path = self.root.join(STATE_LOCK_FILE);
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let lock = options
            .open(&lock_path)
            .map_err(|_| FencingError::DurabilityFailure)?;
        lock.sync_all()
            .map_err(|_| FencingError::DurabilityFailure)?;
        Ok(lock_path)
    }

    fn release_state_lock(&self, lock_path: PathBuf) -> std::io::Result<()> {
        #[cfg(test)]
        if self.fail_lock_cleanup.replace(false) {
            return Err(std::io::Error::other("injected lock cleanup failure"));
        }
        fs::remove_file(lock_path).and_then(|_| File::open(&self.root)?.sync_all())
    }

    #[cfg(test)]
    pub fn fail_next_lock_cleanup_for_test(&self) {
        self.fail_lock_cleanup.set(true);
    }

    fn persist(
        &self,
        generation: u64,
        floors: &BTreeMap<Vec<u8>, FenceReceipt>,
        receipts: &BTreeMap<Vec<u8>, FenceReceipt>,
    ) -> FencingResult<[u8; 32]> {
        let body = StateBody {
            schema: FENCING_STATE_SCHEMA.to_owned(),
            generation,
            floors: floors.values().cloned().collect(),
            receipts: receipts.values().cloned().collect(),
        };
        let body_bytes = serde_jcs::to_vec(&body).map_err(|_| FencingError::StateCorrupt)?;
        let envelope = StateEnvelope {
            body,
            digest: Sha256::digest(body_bytes).into(),
        };
        let bytes = serde_jcs::to_vec(&envelope).map_err(|_| FencingError::StateCorrupt)?;
        if bytes.len() > self.policy.max_state_bytes {
            return Err(FencingError::ResourceExhausted);
        }
        write_atomic(&self.root, &bytes)?;
        Ok(Sha256::digest(bytes).into())
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

fn collect_records(
    records: Vec<FenceReceipt>,
    by_lineage: bool,
) -> FencingResult<BTreeMap<Vec<u8>, FenceReceipt>> {
    let mut map = BTreeMap::new();
    for record in records {
        if !valid_identity(&record.trust_domain_id)
            || !valid_identity(&record.lineage_id)
            || record.request_id.is_empty()
            || record.request_id.len() > MAX_REQUEST_ID_BYTES
            || record.epoch == 0
            || record.committed_log_index == 0
            || record.voter_set_generation == 0
            || (record.operation_class != OperationClass::Fence as u32
                && record.operation_class != OperationClass::Revoke as u32)
        {
            return Err(FencingError::StateCorrupt);
        }
        let key = if by_lineage {
            record.lineage_id.clone()
        } else {
            record.request_id.clone()
        };
        if map.insert(key, record).is_some() {
            return Err(FencingError::StateCorrupt);
        }
    }
    Ok(map)
}

fn validate_lease_binding(
    body: &super::lease::AuthorityCertificateBodyV1,
    lease: &LeaseState,
    lease_body: &super::lease::AuthorityCertificateBodyV1,
) -> FencingResult<()> {
    if lease.lineage_id != body.lineage_id
        || lease.holder_node_id != body.holder_node_id
        || lease.holder_guardian_id != body.holder_guardian_id
        || lease.lineage_id != lease_body.lineage_id
        || lease.holder_node_id != lease_body.holder_node_id
        || lease.holder_guardian_id != lease_body.holder_guardian_id
        || lease.epoch != lease_body.epoch
        || lease.raft_term != lease_body.raft_term
        || lease.committed_log_index != lease_body.committed_log_index
        || lease.deadline_unix_millis
            != u64::try_from(lease_body.issued_unix_seconds)
                .ok()
                .and_then(|value| value.checked_mul(1_000))
                .and_then(|value| value.checked_add(u64::from(lease_body.issued_nanos) / 1_000_000))
                .and_then(|value| value.checked_add(lease_body.lease_duration_millis))
                .ok_or(FencingError::InvalidCertificate)?
        || <[u8; 32]>::from(Sha256::digest(lease.activation_public_key)).as_slice()
            != lease_body.activation_key_sha256
        || body.activation_key_sha256 != lease_body.activation_key_sha256
    {
        return Err(FencingError::HolderMismatch);
    }
    Ok(())
}

fn verify_activation_possession(
    body: &super::lease::AuthorityCertificateBodyV1,
    lease: &LeaseState,
    proof: &[u8],
) -> FencingResult<()> {
    let key = VerifyingKey::from_bytes(&lease.activation_public_key)
        .map_err(|_| FencingError::ActivationPossession)?;
    let signature = Signature::from_slice(proof).map_err(|_| FencingError::ActivationPossession)?;
    let mut digest = Sha256::new();
    digest.update(ACTIVATION_DOMAIN);
    digest.update(certificate_body_sha256(body));
    key.verify_strict(&digest.finalize(), &signature)
        .map_err(|_| FencingError::ActivationPossession)
}

fn request_digest(
    request_id: &[u8],
    certificate_sha256: [u8; 32],
    lease: &LeaseState,
    membership: &AuthorityMembership,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"ADL-FENCING-REQUEST-V1\0");
    for value in [
        request_id,
        lease.lineage_id.as_slice(),
        lease.holder_guardian_id.as_slice(),
        membership.trust_domain_id.as_slice(),
    ] {
        digest.update((value.len() as u64).to_be_bytes());
        digest.update(value);
    }
    digest.update(certificate_sha256);
    digest.update(membership.voter_set_generation.to_be_bytes());
    digest.update(membership.committed_log_index.to_be_bytes());
    digest.finalize().into()
}

fn valid_identity(value: &[u8]) -> bool {
    !value.is_empty() && value.len() <= MAX_IDENTITY_BYTES
}

fn validate_root(root: &Path) -> FencingResult<PathBuf> {
    if !root.is_absolute()
        || root.as_os_str().len() > 4096
        || root
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
    {
        return Err(FencingError::UnsafeStatePath);
    }
    let mut current = PathBuf::new();
    for component in root.components() {
        current.push(component.as_os_str());
        let metadata = fs::symlink_metadata(&current).map_err(|_| FencingError::UnsafeStatePath)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(FencingError::UnsafeStatePath);
        }
    }
    root.canonicalize()
        .map_err(|_| FencingError::UnsafeStatePath)
}

fn write_atomic(root: &Path, bytes: &[u8]) -> FencingResult<()> {
    let path = root.join(STATE_FILE);
    if fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.file_type().is_file())
    {
        return Err(FencingError::UnsafeStatePath);
    }
    let temporary = root.join(format!(".{STATE_FILE}.tmp"));
    if fs::symlink_metadata(&temporary).is_ok() {
        return Err(FencingError::DurabilityFailure);
    }
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&temporary)
        .map_err(|_| FencingError::DurabilityFailure)?;
    let result = file
        .write_all(bytes)
        .and_then(|_| file.sync_all())
        .and_then(|_| fs::rename(&temporary, &path))
        .and_then(|_| File::open(root)?.sync_all());
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
        return Err(FencingError::DurabilityFailure);
    }
    Ok(())
}
