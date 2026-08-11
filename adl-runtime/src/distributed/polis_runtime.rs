//! Production three-voter polis consensus owned by the Guardian process.
//!
//! OpenRaft is the only source of leader, quorum, committed-index, log
//! compaction, and snapshot-installation truth. Application authority changes
//! are applied only from committed log entries.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs::{File, OpenOptions},
    future::Future,
    io::{Cursor, Read, Write},
    ops::RangeBounds,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use fs2::FileExt;
use openraft::{
    error::{InstallSnapshotError, NetworkError, RPCError, RaftError, RemoteError},
    network::{RPCOption, RaftNetwork, RaftNetworkFactory},
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    storage::{LogFlushed, RaftLogStorage, RaftStateMachine, Snapshot},
    BasicNode, Entry, EntryPayload, ErrorSubject, ErrorVerb, LogId, LogState, RaftLogId,
    RaftLogReader, RaftSnapshotBuilder, RaftTypeConfig, SnapshotMeta, StorageError,
    StoredMembership, Vote,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::{watch, RwLock};
use tokio_util::sync::CancellationToken;

use crate::distributed::certificates::{AuthorityCertificate, DistributedCertificateStore};
use crate::distributed::lease::AuthorityMembership;
use crate::distributed::membership::MembershipPolicy;
use crate::distributed::transport::{
    AuthenticatedConnection, EstablishedPolisSession, EstablishedRuntimeAuthority,
    IncomingPolisRequest, PendingPolisSession, PolisIdentityBinding, PolisSessionBinding,
    RuntimeAuthorityInitializer, TransportLimits, TransportResult, VerifiedPolisRouteCut,
};
use crate::kernel_continuity_client::KernelContinuityClient;
use adl_runtime_kernel::{
    sha256, ContinuityCommand, ContinuityControlError, ContinuityReply,
    MigrationDecisionCertificate, RuntimeInitConfig, SignedBundleCatalog, SourceCheckpointHandle,
    SourceQuiesceReceipt, SourceResumeReceipt, TargetActivationReceipt, TargetChunkReceipt,
    TargetCleanupPermit, TargetDiscardReceipt, TargetPossessionEvidence, TargetStageHandle,
};

const MAX_RPC_BYTES: usize = 16 * 1024 * 1024;
const MAX_TEXT_BYTES: usize = 256;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub type NodeId = u64;

openraft::declare_raft_types!(
    pub PolisTypeConfig:
        D = PolisCommand,
        R = PolisResponse,
);

pub type PolisRaft = openraft::Raft<PolisTypeConfig>;

/// Non-forgeable continuity capability injected into the production Guardian
/// polis runtime only after the Guardian has validated the common init
/// contract and constructed the opaque private client.
struct PolisRuntimeContinuityCapability {
    client: Arc<KernelContinuityClient>,
    transfer_210: TransferContinuityPort,
    migration_204: MigrationContinuityPort,
}

/// Production owner of the private continuity capability used by the running
/// Polis Runtime. The Guardian retains this value for the full supervised
/// process lifetime; downstream #210 and #204 code can request only their
/// role-specific effects through these methods. The low-level client is never
/// installed as Guardian status state or exposed to a public route.
pub struct ProductionPolisRuntime {
    continuity: PolisRuntimeContinuityCapability,
}

impl ProductionPolisRuntime {
    pub async fn from_runtime_init(
        init: &RuntimeInitConfig,
    ) -> Result<Self, ContinuityControlError> {
        Ok(Self {
            continuity: PolisRuntimeContinuityCapability::from_runtime_init(init).await?,
        })
    }

    pub(crate) async fn establish_continuity(
        &self,
        attempt: u32,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<(), ContinuityControlError> {
        self.continuity
            .client
            .establish_attempt_with_cancellation(attempt, deadline_unix_millis, cancellation)
            .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn source_checkpoint_210(
        &self,
        operation_id: &str,
        generation: u64,
        predecessor_sha256: Option<String>,
        accepted_prefix: u64,
        topology_sha256: String,
        config_sha256: String,
        quiesce_millis: u64,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<(SourceQuiesceReceipt, SourceCheckpointHandle), ContinuityControlError> {
        self.continuity
            .transfer_210
            .source_checkpoint(
                operation_id,
                generation,
                predecessor_sha256,
                accepted_prefix,
                topology_sha256,
                config_sha256,
                quiesce_millis,
                deadline_unix_millis,
                cancellation,
            )
            .await
    }

    pub async fn activate_target_204(
        &self,
        operation_id: &str,
        verified: VerifiedTransferPossession,
        decision: MigrationDecisionCertificate,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        self.continuity
            .migration_204
            .activate_target(
                operation_id,
                verified,
                decision,
                deadline_unix_millis,
                cancellation,
            )
            .await
    }

    pub fn transfer_210(&self) -> &TransferContinuityPort {
        &self.continuity.transfer_210
    }

    pub fn migration_204(&self) -> &MigrationContinuityPort {
        &self.continuity.migration_204
    }
}

impl PolisRuntimeContinuityCapability {
    /// Production bootstrap is the only public constructor. The returned
    /// capability contains distinct, sealed #210 transfer and #204 migration
    /// views; neither view exposes the generic control protocol.
    async fn from_runtime_init(init: &RuntimeInitConfig) -> Result<Self, ContinuityControlError> {
        let client = Arc::new(KernelContinuityClient::from_runtime_init(init).await?);
        let cleanup_permits = Arc::new(std::sync::Mutex::new(BTreeMap::new()));
        Ok(Self {
            transfer_210: TransferContinuityPort {
                client: Arc::clone(&client),
                cleanup_permits: Arc::clone(&cleanup_permits),
            },
            migration_204: MigrationContinuityPort {
                client: Arc::clone(&client),
                cleanup_permits,
            },
            client,
        })
    }
}

/// The only successful #210 transfer result. Its private fields prevent a
/// caller from manufacturing verified possession or acquiring cleanup or
/// activation authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTransferPossession {
    stage: TargetStageHandle,
    possession: TargetPossessionEvidence,
}

impl VerifiedTransferPossession {
    pub fn stage(&self) -> &TargetStageHandle {
        &self.stage
    }

    pub fn possession(&self) -> &TargetPossessionEvidence {
        &self.possession
    }
}

/// Sealed #210 view. Its private fields prevent caller construction and its
/// API omits source resume, target activation, target discard, and the generic
/// command channel.
pub struct TransferContinuityPort {
    client: Arc<KernelContinuityClient>,
    cleanup_permits: Arc<std::sync::Mutex<BTreeMap<String, TargetCleanupPermit>>>,
}

impl TransferContinuityPort {
    #[allow(clippy::too_many_arguments)]
    pub async fn source_checkpoint(
        &self,
        operation_id: &str,
        generation: u64,
        predecessor_sha256: Option<String>,
        accepted_prefix: u64,
        topology_sha256: String,
        config_sha256: String,
        quiesce_millis: u64,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<(SourceQuiesceReceipt, SourceCheckpointHandle), ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::QuiesceAndExport {
                    generation,
                    predecessor_sha256,
                    topology_sha256,
                    config_sha256,
                    deadline_millis: quiesce_millis,
                },
                accepted_prefix,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::Exported { quiesce, handle } => Ok((quiesce, handle)),
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn read_signed_range(
        &self,
        operation_id: &str,
        handle: SourceCheckpointHandle,
        ordinal: u32,
        relative_offset: u64,
        length: u64,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::ReadBundleRange {
                    handle,
                    ordinal,
                    relative_offset,
                    length,
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::BundleRange { bytes_base64 } => BASE64
                .decode(bytes_base64)
                .map_err(|_| ContinuityControlError::ContentMismatch),
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    pub async fn create_target_stage(
        &self,
        operation_id: &str,
        stage_id: String,
        root_generation: u64,
        catalog: SignedBundleCatalog,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetStageHandle, ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::StageTarget {
                    stage_id,
                    root_generation,
                    catalog,
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::StageCreated { handle, cleanup } => {
                self.cleanup_permits
                    .lock()
                    .map_err(|_| ContinuityControlError::CorruptJournal)?
                    .insert(handle.id().to_owned(), cleanup);
                Ok(handle)
            }
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn write_target_chunk(
        &self,
        operation_id: &str,
        handle: TargetStageHandle,
        ordinal: u32,
        chunk_index: u32,
        relative_offset: u64,
        predecessor_sha256: Option<String>,
        bytes: &[u8],
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetChunkReceipt, ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::WriteTargetChunk {
                    handle,
                    ordinal,
                    chunk_index,
                    relative_offset,
                    predecessor_sha256,
                    chunk_sha256: sha256(bytes),
                    bytes_base64: BASE64.encode(bytes),
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::ChunkWritten { receipt } => Ok(receipt),
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn verify_target(
        &self,
        operation_id: &str,
        handle: TargetStageHandle,
        expected_generation: u64,
        expected_predecessor: Option<String>,
        expected_prefix: u64,
        topology_sha256: String,
        config_sha256: String,
        service_schemas: BTreeMap<String, String>,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<VerifiedTransferPossession, ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::ValidateTarget {
                    handle: handle.clone(),
                    expected_generation,
                    expected_predecessor,
                    expected_prefix,
                    topology_sha256,
                    config_sha256,
                    service_schemas,
                },
                expected_prefix,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::TargetValidated { possession } => Ok(VerifiedTransferPossession {
                stage: handle,
                possession,
            }),
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    pub async fn request_cleanup(
        &self,
        operation_id: &str,
        handle: TargetStageHandle,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetDiscardReceipt, ContinuityControlError> {
        let cleanup = self
            .cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .get(handle.id())
            .cloned()
            .ok_or(ContinuityControlError::CleanupAuthority)?;
        let result = match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::DiscardTarget {
                    handle: handle.clone(),
                    cleanup,
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::TargetDiscarded { receipt } => Ok(receipt),
            _ => Err(ContinuityControlError::ContentMismatch),
        }?;
        self.cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .remove(handle.id());
        Ok(result)
    }
}

/// Sealed #204 view. It is constructible only through production capability
/// initialization, consumes the unforgeable verified-transfer result for
/// activation, and exposes no generic executor.
pub struct MigrationContinuityPort {
    client: Arc<KernelContinuityClient>,
    cleanup_permits: Arc<std::sync::Mutex<BTreeMap<String, TargetCleanupPermit>>>,
}

impl MigrationContinuityPort {
    pub async fn resume_source(
        &self,
        operation_id: &str,
        handle: SourceCheckpointHandle,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<SourceResumeReceipt, ContinuityControlError> {
        match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::ResumeSource { handle },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::SourceResumed { receipt } => Ok(receipt),
            _ => Err(ContinuityControlError::ContentMismatch),
        }
    }

    pub async fn activate_target(
        &self,
        operation_id: &str,
        verified: VerifiedTransferPossession,
        decision: MigrationDecisionCertificate,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        let cleanup = self
            .cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .get(verified.stage().id())
            .cloned()
            .ok_or(ContinuityControlError::CleanupAuthority)?;
        let handle = verified.stage().clone();
        let possession = verified.possession().clone();
        let result = match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::ActivateTarget {
                    handle: handle.clone(),
                    possession,
                    cleanup,
                    decision: Box::new(decision),
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::TargetActivated { receipt } => Ok(receipt),
            _ => Err(ContinuityControlError::ContentMismatch),
        }?;
        self.cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .remove(handle.id());
        Ok(result)
    }

    pub async fn discard_target(
        &self,
        operation_id: &str,
        handle: TargetStageHandle,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<TargetDiscardReceipt, ContinuityControlError> {
        let cleanup = self
            .cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .get(handle.id())
            .cloned()
            .ok_or(ContinuityControlError::CleanupAuthority)?;
        let result = match self
            .client
            .run_role_command(
                operation_id,
                ContinuityCommand::DiscardTarget {
                    handle: handle.clone(),
                    cleanup,
                },
                0,
                deadline_unix_millis,
                cancellation,
            )
            .await?
        {
            ContinuityReply::TargetDiscarded { receipt } => Ok(receipt),
            _ => Err(ContinuityControlError::ContentMismatch),
        }?;
        self.cleanup_permits
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .remove(handle.id());
        Ok(result)
    }
}

/// Trusted deployment bootstrap for the authority accepted by one Polis Runtime.
///
/// The host configuration boundary owns the configured certificate store and
/// externally retained membership commitment. Untrusted transport and RPC
/// inputs cannot invoke the low-level authority initializer or nominate roots,
/// membership bytes, or an authority lineage during route authorization.
pub struct PolisRuntimeAuthorityBootstrap {
    initializer: RuntimeAuthorityInitializer,
}

impl PolisRuntimeAuthorityBootstrap {
    pub fn restore_configured(
        certificate_store: Arc<DistributedCertificateStore>,
        membership_policy: MembershipPolicy,
        membership_snapshot: &[u8],
        trusted_membership_commitment: [u8; 32],
    ) -> TransportResult<Self> {
        Ok(Self {
            initializer: RuntimeAuthorityInitializer::restore(
                certificate_store,
                membership_policy,
                membership_snapshot,
                trusted_membership_commitment,
            )?,
        })
    }

    pub fn accept_signed_lineage(
        &self,
        authority: &AuthorityMembership,
        guardian_certificates: &BTreeMap<Vec<u8>, AuthorityCertificate>,
        now_unix_seconds: u64,
    ) -> TransportResult<EstablishedRuntimeAuthority> {
        self.initializer
            .accept_signed_lineage(authority, guardian_certificates, now_unix_seconds)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "operation", deny_unknown_fields)]
pub enum PolisCommand {
    GovernedMutation {
        mutation_id: String,
        payload_sha256: String,
    },
    SnapshotBoundary {
        snapshot_sha256: String,
    },
    FenceVoter {
        voter_id: String,
        epoch: u64,
    },
    ActivateOwner {
        owner_id: String,
        epoch: u64,
    },
    ActivateShepherd {
        shepherd_identity_ref: String,
        epoch: u64,
    },
    AcquireObservatory {
        owner_id: String,
        epoch: u64,
        expires_unix_millis: u64,
    },
    DemoteVoter {
        voter_id: String,
        epoch: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolisResponse {
    pub committed_index: u64,
    pub epoch: u64,
    pub accepted: bool,
    pub reason_code: String,
    pub state_sha256: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolisApplicationState {
    pub committed_index: u64,
    pub epoch: u64,
    pub mutation_ids: BTreeSet<String>,
    pub snapshot_sha256: Option<String>,
    pub fenced_voters: BTreeMap<String, u64>,
    pub active_owner: Option<String>,
    pub active_shepherd: Option<String>,
    pub observatory_owner: Option<String>,
    pub observatory_expires_unix_millis: Option<u64>,
    pub demoted_voters: BTreeMap<String, u64>,
}

impl PolisApplicationState {
    fn apply(&mut self, index: u64, command: &PolisCommand) -> Result<bool, PolisRuntimeError> {
        if index <= self.committed_index {
            return Err(PolisRuntimeError::StateRegression);
        }
        let mut accepted = true;
        match command {
            PolisCommand::GovernedMutation {
                mutation_id,
                payload_sha256,
            } => {
                validate_text(mutation_id)?;
                validate_sha256(payload_sha256)?;
                accepted = self.mutation_ids.insert(mutation_id.clone());
            }
            PolisCommand::SnapshotBoundary { snapshot_sha256 } => {
                validate_sha256(snapshot_sha256)?;
                self.snapshot_sha256 = Some(snapshot_sha256.clone());
            }
            PolisCommand::FenceVoter { voter_id, epoch } => {
                validate_text(voter_id)?;
                if *epoch <= self.epoch {
                    accepted = false;
                } else {
                    self.epoch = *epoch;
                    self.fenced_voters.insert(voter_id.clone(), *epoch);
                    if self.observatory_owner.as_deref() == Some(voter_id) {
                        self.observatory_owner = None;
                        self.observatory_expires_unix_millis = None;
                    }
                }
            }
            PolisCommand::ActivateOwner { owner_id, epoch } => {
                validate_text(owner_id)?;
                if *epoch != self.epoch || self.fenced_voters.get(owner_id) == Some(epoch) {
                    accepted = false;
                } else {
                    self.active_owner = Some(owner_id.clone());
                }
            }
            PolisCommand::ActivateShepherd {
                shepherd_identity_ref,
                epoch,
            } => {
                validate_text(shepherd_identity_ref)?;
                if *epoch != self.epoch || self.active_owner.is_none() {
                    accepted = false;
                } else {
                    self.active_shepherd = Some(shepherd_identity_ref.clone());
                }
            }
            PolisCommand::AcquireObservatory {
                owner_id,
                epoch,
                expires_unix_millis,
            } => {
                validate_text(owner_id)?;
                if *epoch != self.epoch
                    || self.active_owner.as_deref() != Some(owner_id)
                    || *expires_unix_millis == 0
                {
                    accepted = false;
                } else {
                    self.observatory_owner = Some(owner_id.clone());
                    self.observatory_expires_unix_millis = Some(*expires_unix_millis);
                }
            }
            PolisCommand::DemoteVoter { voter_id, epoch } => {
                validate_text(voter_id)?;
                if *epoch != self.epoch || self.fenced_voters.get(voter_id) != Some(epoch) {
                    accepted = false;
                } else {
                    self.demoted_voters.insert(voter_id.clone(), *epoch);
                }
            }
        }
        self.committed_index = index;
        Ok(accepted)
    }

    pub fn digest(&self) -> Result<String, PolisRuntimeError> {
        let bytes = serde_jcs::to_vec(self).map_err(|_| PolisRuntimeError::Serialization)?;
        Ok(hex::encode(Sha256::digest(bytes)))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersistedStateMachine {
    last_applied_log: Option<LogId<NodeId>>,
    last_membership: StoredMembership<NodeId, BasicNode>,
    application: PolisApplicationState,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersistedLog {
    last_purged_log_id: Option<LogId<NodeId>>,
    log: BTreeMap<u64, Entry<PolisTypeConfig>>,
    committed: Option<LogId<NodeId>>,
    vote: Option<Vote<NodeId>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SnapshotRecord {
    meta: Option<SnapshotMeta<NodeId, BasicNode>>,
    state: PersistedStateMachine,
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsensusCheckpoint {
    pub object: String,
    pub generation: u64,
    pub payload_sha256: String,
    pub committed_log_index: Option<u64>,
    pub state_sha256: Option<String>,
    pub snapshot_log_index: Option<u64>,
    pub snapshot_sha256: Option<String>,
}

pub trait ConsensusCheckpointAuthority: Send + Sync {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError>;

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableEnvelope<T> {
    schema: String,
    generation: u64,
    payload_sha256: String,
    payload: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableJournal<T> {
    expected: Option<ConsensusCheckpoint>,
    previous: Option<DurableEnvelope<T>>,
    candidate: DurableEnvelope<T>,
}

#[derive(Clone, Debug, Default)]
struct CheckpointMetadata {
    committed_log_index: Option<u64>,
    state_sha256: Option<String>,
    snapshot_log_index: Option<u64>,
    snapshot_sha256: Option<String>,
}

trait CheckpointMetadataSource {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError>;
}

struct CheckpointedJson<T> {
    object: String,
    path: PathBuf,
    journal_path: PathBuf,
    _writer_lock: Arc<File>,
    authority: Arc<dyn ConsensusCheckpointAuthority>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> CheckpointedJson<T>
where
    T: Clone + Serialize + DeserializeOwned + CheckpointMetadataSource,
{
    fn open(
        root: &Path,
        object: &str,
        file_name: &str,
        default_payload: T,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<(Self, DurableEnvelope<T>), PolisRuntimeError> {
        ensure_store_root(root)?;
        validate_text(object)?;
        let lock_path = root.join(format!(".{file_name}.lock"));
        let writer_lock = acquire_writer_lock(&lock_path)?;
        let store = Self {
            object: object.to_owned(),
            path: root.join(file_name),
            journal_path: root.join(format!(".{file_name}.journal")),
            _writer_lock: Arc::new(writer_lock),
            authority,
            _marker: std::marker::PhantomData,
        };
        let mut envelope = if store.path.exists() {
            read_bounded_json(&store.path)?
        } else {
            durable_envelope(0, default_payload.clone())?
        };
        validate_durable_envelope(&envelope)?;
        let checkpoint = checkpoint_for(&store.object, &envelope)?;
        let external = store.authority.load(&store.object)?;

        if store.journal_path.exists() {
            let journal: DurableJournal<T> = read_bounded_json(&store.journal_path)?;
            if let Some(previous) = journal.previous.as_ref() {
                validate_durable_envelope(previous)?;
            }
            validate_durable_envelope(&journal.candidate)?;
            let candidate_checkpoint = checkpoint_for(&store.object, &journal.candidate)?;
            let previous_checkpoint = journal
                .previous
                .as_ref()
                .map(|previous| checkpoint_for(&store.object, previous))
                .transpose()?;
            let disk_checkpoint = checkpoint_for(&store.object, &envelope)?;
            if journal.expected != previous_checkpoint {
                return Err(PolisRuntimeError::StateRegression);
            }
            match external.as_ref() {
                Some(current)
                    if current == &candidate_checkpoint
                        && disk_checkpoint == candidate_checkpoint =>
                {
                    envelope = journal.candidate;
                }
                Some(current)
                    if Some(current) == journal.expected.as_ref()
                        && disk_checkpoint == candidate_checkpoint
                        && journal.previous.is_some() =>
                {
                    let previous = journal.previous.ok_or(PolisRuntimeError::StateRegression)?;
                    atomic_json_write(&store.path, &previous)
                        .map_err(|_| PolisRuntimeError::Storage)?;
                    envelope = previous;
                }
                Some(current)
                    if Some(current) == journal.expected.as_ref()
                        && Some(&disk_checkpoint) == journal.expected.as_ref()
                        && journal.previous.is_some() =>
                {
                    envelope = journal.previous.ok_or(PolisRuntimeError::StateRegression)?;
                }
                None if journal.expected.is_none() && disk_checkpoint == candidate_checkpoint => {
                    remove_file_and_sync(&store.path)?;
                    remove_file_and_sync(&store.journal_path)?;
                    envelope = durable_envelope(0, default_payload)?;
                    let candidate_checkpoint = checkpoint_for(&store.object, &envelope)?;
                    store.initialize(&envelope, &candidate_checkpoint)?;
                    return Ok((store, envelope));
                }
                _ => return Err(PolisRuntimeError::StateRegression),
            }
            remove_file_and_sync(&store.journal_path)?;
        } else {
            match external.as_ref() {
                Some(current) if current == &checkpoint => {}
                None if !store.path.exists() => {
                    store.initialize(&envelope, &checkpoint)?;
                }
                _ => return Err(PolisRuntimeError::StateRegression),
            }
        }
        Ok((store, envelope))
    }

    fn initialize(
        &self,
        candidate: &DurableEnvelope<T>,
        checkpoint: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        atomic_json_write(
            &self.journal_path,
            &DurableJournal::<T> {
                expected: None,
                previous: None,
                candidate: candidate.clone(),
            },
        )
        .map_err(|_| PolisRuntimeError::Storage)?;
        atomic_json_write(&self.path, candidate).map_err(|_| PolisRuntimeError::Storage)?;
        if let Err(error) = self.authority.compare_and_swap(None, checkpoint) {
            remove_file_and_sync(&self.path)?;
            remove_file_and_sync(&self.journal_path)?;
            return Err(error);
        }
        remove_file_and_sync(&self.journal_path)?;
        Ok(())
    }

    fn commit(
        &self,
        current: &DurableEnvelope<T>,
        payload: T,
    ) -> Result<DurableEnvelope<T>, PolisRuntimeError> {
        let generation = current
            .generation
            .checked_add(1)
            .ok_or(PolisRuntimeError::StateRegression)?;
        let candidate = durable_envelope(generation, payload)?;
        let expected = checkpoint_for(&self.object, current)?;
        let candidate_checkpoint = checkpoint_for(&self.object, &candidate)?;
        validate_checkpoint_transition(&expected, &candidate_checkpoint)?;
        if self.authority.load(&self.object)?.as_ref() != Some(&expected) {
            return Err(PolisRuntimeError::StateRegression);
        }
        atomic_json_write(
            &self.journal_path,
            &DurableJournal {
                expected: Some(expected.clone()),
                previous: Some(current.clone()),
                candidate: candidate.clone(),
            },
        )
        .map_err(|_| PolisRuntimeError::Storage)?;
        atomic_json_write(&self.path, &candidate).map_err(|_| PolisRuntimeError::Storage)?;
        self.authority
            .compare_and_swap(Some(&expected), &candidate_checkpoint)?;
        remove_file_and_sync(&self.journal_path)?;
        Ok(candidate)
    }
}

fn durable_envelope<T: Serialize>(
    generation: u64,
    payload: T,
) -> Result<DurableEnvelope<T>, PolisRuntimeError> {
    let payload_bytes =
        serde_jcs::to_vec(&payload).map_err(|_| PolisRuntimeError::Serialization)?;
    if payload_bytes.len() > MAX_RPC_BYTES {
        return Err(PolisRuntimeError::FrameTooLarge);
    }
    Ok(DurableEnvelope {
        schema: "adl.distributed.checkpointed_json.v1".to_owned(),
        generation,
        payload_sha256: hex::encode(Sha256::digest(payload_bytes)),
        payload,
    })
}

fn validate_durable_envelope<T: Serialize>(
    envelope: &DurableEnvelope<T>,
) -> Result<(), PolisRuntimeError> {
    if envelope.schema != "adl.distributed.checkpointed_json.v1" {
        return Err(PolisRuntimeError::Serialization);
    }
    let bytes =
        serde_jcs::to_vec(&envelope.payload).map_err(|_| PolisRuntimeError::Serialization)?;
    if bytes.len() > MAX_RPC_BYTES || hex::encode(Sha256::digest(bytes)) != envelope.payload_sha256
    {
        return Err(PolisRuntimeError::StateRegression);
    }
    Ok(())
}

fn checkpoint_for<T: CheckpointMetadataSource>(
    object: &str,
    envelope: &DurableEnvelope<T>,
) -> Result<ConsensusCheckpoint, PolisRuntimeError> {
    let metadata = envelope.payload.checkpoint_metadata()?;
    Ok(ConsensusCheckpoint {
        object: object.to_owned(),
        generation: envelope.generation,
        payload_sha256: envelope.payload_sha256.clone(),
        committed_log_index: metadata.committed_log_index,
        state_sha256: metadata.state_sha256,
        snapshot_log_index: metadata.snapshot_log_index,
        snapshot_sha256: metadata.snapshot_sha256,
    })
}

fn validate_checkpoint_transition(
    previous: &ConsensusCheckpoint,
    candidate: &ConsensusCheckpoint,
) -> Result<(), PolisRuntimeError> {
    if previous.object != candidate.object || candidate.generation <= previous.generation {
        return Err(PolisRuntimeError::StateRegression);
    }
    for (old, new) in [
        (previous.committed_log_index, candidate.committed_log_index),
        (previous.snapshot_log_index, candidate.snapshot_log_index),
    ] {
        if matches!((old, new), (Some(old), Some(new)) if new < old)
            || matches!((old, new), (Some(_), None))
        {
            return Err(PolisRuntimeError::StateRegression);
        }
    }
    if previous.committed_log_index == candidate.committed_log_index
        && previous.state_sha256.is_some()
        && previous.state_sha256 != candidate.state_sha256
    {
        return Err(PolisRuntimeError::StateRegression);
    }
    if previous.snapshot_log_index == candidate.snapshot_log_index
        && previous.snapshot_sha256.is_some()
        && previous.snapshot_sha256 != candidate.snapshot_sha256
    {
        return Err(PolisRuntimeError::StateRegression);
    }
    Ok(())
}

impl CheckpointMetadataSource for PersistedLog {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: self.committed.map(|log| log.index),
            ..CheckpointMetadata::default()
        })
    }
}

#[derive(Clone)]
pub struct PolisLogStore {
    durable: Arc<CheckpointedJson<PersistedLog>>,
    inner: Arc<tokio::sync::Mutex<DurableEnvelope<PersistedLog>>>,
}

impl PolisLogStore {
    pub fn open(
        root: &Path,
        node_id: NodeId,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
        if node_id == 0 {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        let (durable, inner) = CheckpointedJson::open(
            root,
            &format!("raft-log-node-{node_id}"),
            "raft-log.json",
            PersistedLog::default(),
            authority,
        )?;
        Ok(Self {
            durable: Arc::new(durable),
            inner: Arc::new(tokio::sync::Mutex::new(inner)),
        })
    }

    #[allow(clippy::result_large_err)]
    fn persist(
        &self,
        current: &DurableEnvelope<PersistedLog>,
        state: PersistedLog,
    ) -> Result<DurableEnvelope<PersistedLog>, StorageError<NodeId>> {
        self.durable.commit(current, state).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::Store,
                ErrorVerb::Write,
                std::io::Error::other(error.code()),
            )
        })
    }
}

impl RaftLogReader<PolisTypeConfig> for PolisLogStore {
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<PolisTypeConfig>>, StorageError<NodeId>> {
        Ok(self
            .inner
            .lock()
            .await
            .payload
            .log
            .range(range)
            .map(|(_, entry)| entry.clone())
            .collect())
    }
}

impl RaftLogStorage<PolisTypeConfig> for PolisLogStore {
    type LogReader = Self;

    async fn get_log_state(&mut self) -> Result<LogState<PolisTypeConfig>, StorageError<NodeId>> {
        let state = self.inner.lock().await;
        let last_log_id = state
            .payload
            .log
            .iter()
            .next_back()
            .map(|(_, entry)| *entry.get_log_id())
            .or(state.payload.last_purged_log_id);
        Ok(LogState {
            last_purged_log_id: state.payload.last_purged_log_id,
            last_log_id,
        })
    }

    async fn save_committed(
        &mut self,
        committed: Option<LogId<NodeId>>,
    ) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        let mut candidate = state.payload.clone();
        candidate.committed = committed;
        *state = self.persist(&state, candidate)?;
        Ok(())
    }

    async fn read_committed(&mut self) -> Result<Option<LogId<NodeId>>, StorageError<NodeId>> {
        Ok(self.inner.lock().await.payload.committed)
    }

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        let mut candidate = state.payload.clone();
        candidate.vote = Some(*vote);
        *state = self.persist(&state, candidate)?;
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        Ok(self.inner.lock().await.payload.vote)
    }

    async fn append<I>(
        &mut self,
        entries: I,
        callback: LogFlushed<PolisTypeConfig>,
    ) -> Result<(), StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<PolisTypeConfig>> + Send,
    {
        let mut state = self.inner.lock().await;
        let mut candidate = state.payload.clone();
        for entry in entries {
            candidate.log.insert(entry.log_id.index, entry);
        }
        let result = self.persist(&state, candidate);
        match result {
            Ok(committed) => {
                *state = committed;
                callback.log_io_completed(Ok(()));
                Ok(())
            }
            Err(error) => {
                callback.log_io_completed(Err(std::io::Error::other(error.to_string())));
                Err(error)
            }
        }
    }

    async fn truncate(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        let mut candidate = state.payload.clone();
        candidate.log.retain(|index, _| *index < log_id.index);
        *state = self.persist(&state, candidate)?;
        Ok(())
    }

    async fn purge(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        let mut candidate = state.payload.clone();
        candidate.log.retain(|index, _| *index > log_id.index);
        candidate.last_purged_log_id = Some(log_id);
        *state = self.persist(&state, candidate)?;
        Ok(())
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedStateMachineStorage {
    current: PersistedStateMachine,
    snapshot: SnapshotRecord,
}

impl Default for PersistedStateMachineStorage {
    fn default() -> Self {
        let current = PersistedStateMachine::default();
        Self {
            snapshot: SnapshotRecord {
                meta: None,
                state: current.clone(),
                bytes: Vec::new(),
            },
            current,
        }
    }
}

impl CheckpointMetadataSource for PersistedStateMachineStorage {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata {
            committed_log_index: self.current.last_applied_log.map(|log| log.index),
            state_sha256: Some(canonical_sha256(&self.current)?),
            snapshot_log_index: self
                .snapshot
                .meta
                .as_ref()
                .and_then(|meta| meta.last_log_id)
                .map(|log| log.index),
            snapshot_sha256: if self.snapshot.bytes.is_empty() {
                None
            } else {
                Some(canonical_sha256(&self.snapshot)?)
            },
        })
    }
}

#[derive(Clone)]
pub struct PolisStateMachineStore {
    durable: Arc<CheckpointedJson<PersistedStateMachineStorage>>,
    inner: Arc<RwLock<DurableEnvelope<PersistedStateMachineStorage>>>,
}

impl PolisStateMachineStore {
    pub fn open(
        root: &Path,
        node_id: NodeId,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
        if node_id == 0 {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        let (durable, inner) = CheckpointedJson::open(
            root,
            &format!("raft-state-node-{node_id}"),
            "raft-state.json",
            PersistedStateMachineStorage::default(),
            authority,
        )?;
        Ok(Self {
            durable: Arc::new(durable),
            inner: Arc::new(RwLock::new(inner)),
        })
    }

    pub async fn application_state(&self) -> PolisApplicationState {
        self.inner.read().await.payload.current.application.clone()
    }

    #[allow(clippy::result_large_err)]
    fn persist(
        &self,
        current: &DurableEnvelope<PersistedStateMachineStorage>,
        state: PersistedStateMachineStorage,
    ) -> Result<DurableEnvelope<PersistedStateMachineStorage>, StorageError<NodeId>> {
        self.durable.commit(current, state).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::StateMachine,
                ErrorVerb::Write,
                std::io::Error::other(error.code()),
            )
        })
    }
}

impl RaftSnapshotBuilder<PolisTypeConfig> for PolisStateMachineStore {
    async fn build_snapshot(&mut self) -> Result<Snapshot<PolisTypeConfig>, StorageError<NodeId>> {
        let mut durable = self.inner.write().await;
        let current = durable.payload.current.clone();
        let data = serde_jcs::to_vec(&current).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::StateMachine,
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, error),
            )
        })?;
        let meta = SnapshotMeta {
            last_log_id: current.last_applied_log,
            last_membership: current.last_membership.clone(),
            snapshot_id: canonical_snapshot_id(&current, &data),
        };
        let mut candidate = durable.payload.clone();
        candidate.snapshot = SnapshotRecord {
            meta: Some(meta.clone()),
            state: current,
            bytes: data.clone(),
        };
        *durable = self.persist(&durable, candidate)?;
        Ok(Snapshot {
            meta,
            snapshot: Box::new(Cursor::new(data)),
        })
    }
}

impl RaftStateMachine<PolisTypeConfig> for PolisStateMachineStore {
    type SnapshotBuilder = Self;

    async fn applied_state(
        &mut self,
    ) -> Result<(Option<LogId<NodeId>>, StoredMembership<NodeId, BasicNode>), StorageError<NodeId>>
    {
        let state = self.inner.read().await;
        Ok((
            state.payload.current.last_applied_log,
            state.payload.current.last_membership.clone(),
        ))
    }

    async fn apply<I>(&mut self, entries: I) -> Result<Vec<PolisResponse>, StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<PolisTypeConfig>> + Send,
    {
        let mut durable = self.inner.write().await;
        let mut candidate = durable.payload.clone();
        let mut responses = Vec::new();
        for entry in entries {
            candidate.current.last_applied_log = Some(entry.log_id);
            let (accepted, reason_code) = match entry.payload {
                EntryPayload::Blank => (true, "raft_internal"),
                EntryPayload::Membership(membership) => {
                    candidate.current.last_membership =
                        StoredMembership::new(Some(entry.log_id), membership);
                    (true, "raft_internal")
                }
                EntryPayload::Normal(command) => {
                    let accepted = candidate
                        .current
                        .application
                        .apply(entry.log_id.index, &command)
                        .map_err(|error| {
                            StorageError::from_io_error(
                                ErrorSubject::Apply(entry.log_id),
                                ErrorVerb::Write,
                                std::io::Error::new(std::io::ErrorKind::InvalidData, error.code()),
                            )
                        })?;
                    (
                        accepted,
                        if accepted {
                            "committed"
                        } else {
                            "governed_rejection"
                        },
                    )
                }
            };
            responses.push(PolisResponse {
                committed_index: candidate.current.application.committed_index,
                epoch: candidate.current.application.epoch,
                accepted,
                reason_code: reason_code.to_owned(),
                state_sha256: candidate.current.application.digest().map_err(|error| {
                    StorageError::from_io_error(
                        ErrorSubject::StateMachine,
                        ErrorVerb::Read,
                        std::io::Error::new(std::io::ErrorKind::InvalidData, error.code()),
                    )
                })?,
            });
        }
        *durable = self.persist(&durable, candidate)?;
        Ok(responses)
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<<PolisTypeConfig as RaftTypeConfig>::SnapshotData>, StorageError<NodeId>> {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<NodeId, BasicNode>,
        snapshot: Box<<PolisTypeConfig as RaftTypeConfig>::SnapshotData>,
    ) -> Result<(), StorageError<NodeId>> {
        if snapshot.get_ref().len() > MAX_RPC_BYTES {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, "snapshot too large"),
            ));
        }
        let current: PersistedStateMachine =
            serde_json::from_slice(snapshot.get_ref()).map_err(|error| {
                StorageError::from_io_error(
                    ErrorSubject::Snapshot(Some(meta.signature())),
                    ErrorVerb::Read,
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error),
                )
            })?;
        if serde_jcs::to_vec(&current).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, error),
            )
        })? != *snapshot.get_ref()
            || current.last_applied_log != meta.last_log_id
            || current.last_membership != meta.last_membership
            || meta.snapshot_id != canonical_snapshot_id(&current, snapshot.get_ref())
        {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Read,
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "snapshot metadata mismatch",
                ),
            ));
        }
        let mut durable = self.inner.write().await;
        if matches!(
            (durable.payload.current.last_applied_log, meta.last_log_id),
            (Some(current), Some(candidate)) if candidate.index < current.index
        ) {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Write,
                std::io::Error::new(std::io::ErrorKind::InvalidData, "snapshot regression"),
            ));
        }
        let mut candidate = durable.payload.clone();
        candidate.current = current.clone();
        candidate.snapshot = SnapshotRecord {
            meta: Some(meta.clone()),
            state: current,
            bytes: snapshot.get_ref().clone(),
        };
        *durable = self.persist(&durable, candidate)?;
        Ok(())
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<PolisTypeConfig>>, StorageError<NodeId>> {
        let durable = self.inner.read().await;
        let snapshot = &durable.payload.snapshot;
        if snapshot.bytes.is_empty() {
            return Ok(None);
        }
        if snapshot.bytes.len() > MAX_RPC_BYTES {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(None),
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, "snapshot too large"),
            ));
        }
        Ok(Some(Snapshot {
            meta: snapshot.meta.clone().ok_or_else(|| {
                StorageError::from_io_error(
                    ErrorSubject::Snapshot(None),
                    ErrorVerb::Read,
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "snapshot metadata absent",
                    ),
                )
            })?,
            snapshot: Box::new(Cursor::new(snapshot.bytes.clone())),
        }))
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }
}
#[derive(Clone)]
pub struct SecurePolisNetworkFactory {
    local: NodeId,
    cut: Arc<RwLock<VerifiedPolisRouteCut>>,
    connections: Arc<RwLock<BTreeMap<NodeId, SecurePeerRoute>>>,
}

#[derive(Clone)]
struct SecurePeerRoute {
    connection: Arc<RwLock<Arc<AuthenticatedConnection>>>,
    session: Arc<RwLock<EstablishedPolisSession>>,
    dispatch_lock: Arc<tokio::sync::Mutex<()>>,
    sequence: Arc<tokio::sync::Mutex<OutboundSequenceState>>,
    replacement: watch::Sender<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OutstandingRequest {
    sequence: u64,
    message_kind: &'static str,
    payload_sha256: [u8; 32],
}

#[derive(Clone, Debug, Default)]
struct OutboundSequenceState {
    acknowledged: u64,
    outstanding: Option<OutstandingRequest>,
}

impl SecurePolisNetworkFactory {
    pub fn from_authority_cut(
        local: NodeId,
        cut: VerifiedPolisRouteCut,
    ) -> Result<Self, PolisRuntimeError> {
        if local == 0 || !cut.contains(local) || cut.len() != 3 {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        Ok(Self {
            local,
            cut: Arc::new(RwLock::new(cut)),
            connections: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    pub async fn pending_session(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
    ) -> Result<PendingPolisSession, PolisRuntimeError> {
        self.cut
            .read()
            .await
            .pending_session(self.local, target, connection)
            .map_err(|_| PolisRuntimeError::AuthorityDenied)
    }

    pub async fn install_route(
        &self,
        target: NodeId,
        connection: Arc<AuthenticatedConnection>,
        session: EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        if !self
            .cut
            .read()
            .await
            .session_matches(self.local, target, &connection, &session)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let mut routes = self.connections.write().await;
        if routes.contains_key(&target) {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        let (replacement, _) = watch::channel(0);
        routes.insert(
            target,
            SecurePeerRoute {
                connection: Arc::new(RwLock::new(connection)),
                session: Arc::new(RwLock::new(session)),
                dispatch_lock: Arc::new(tokio::sync::Mutex::new(())),
                sequence: Arc::new(tokio::sync::Mutex::new(OutboundSequenceState::default())),
                replacement,
            },
        );
        Ok(())
    }

    pub async fn replace_route(
        &self,
        target: NodeId,
        connection: Arc<AuthenticatedConnection>,
        session: EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        if !self
            .cut
            .read()
            .await
            .session_matches(self.local, target, &connection, &session)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let route = self
            .connections
            .read()
            .await
            .get(&target)
            .cloned()
            .ok_or(PolisRuntimeError::InvalidConfiguration)?;
        let previous_namespace = {
            let previous = route.session.read().await;
            (
                previous.binding().local_certificate_generation(),
                previous.binding().local_boot_generation(),
                previous.binding().peer_certificate_generation(),
                previous.binding().peer_boot_generation(),
                previous.binding().committed_membership_index(),
            )
        };
        let next_namespace = (
            session.binding().local_certificate_generation(),
            session.binding().local_boot_generation(),
            session.binding().peer_certificate_generation(),
            session.binding().peer_boot_generation(),
            session.binding().committed_membership_index(),
        );
        *route.connection.write().await = connection;
        *route.session.write().await = session;
        if previous_namespace != next_namespace {
            *route.sequence.lock().await = OutboundSequenceState::default();
        }
        let next_generation = (*route.replacement.borrow())
            .checked_add(1)
            .ok_or(PolisRuntimeError::StateRegression)?;
        route.replacement.send_replace(next_generation);
        Ok(())
    }

    pub async fn replace_authority_cut(
        &self,
        candidate: VerifiedPolisRouteCut,
    ) -> Result<(), PolisRuntimeError> {
        let mut current = self.cut.write().await;
        if !candidate.same_polis_and_domain(&current)
            || !candidate.same_authority_lineage(&current)
            || candidate.committed_membership_index() < current.committed_membership_index()
            || !candidate.contains(self.local)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        for node in current.routes().keys() {
            if candidate
                .boot_generation(*node)
                .zip(current.boot_generation(*node))
                .is_some_and(|(next, previous)| next < previous)
            {
                return Err(PolisRuntimeError::StateRegression);
            }
        }
        *current = candidate;
        Ok(())
    }

    pub async fn request_bytes(
        &self,
        target: NodeId,
        message_kind: &'static str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, PolisRuntimeError> {
        if payload.len() > MAX_RPC_BYTES {
            return Err(PolisRuntimeError::FrameTooLarge);
        }
        let route = self
            .connections
            .read()
            .await
            .get(&target)
            .cloned()
            .ok_or(PolisRuntimeError::Network)?;
        let _dispatch = route.dispatch_lock.lock().await;
        {
            let cut = self.cut.read().await;
            let connection = route.connection.read().await;
            let session = route.session.read().await;
            if !cut.session_matches(self.local, target, &connection, &session) {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
        }
        let payload_sha256: [u8; 32] = Sha256::digest(&payload).into();
        let sequence = reserve_outbound(&route, message_kind, payload_sha256).await?;
        let mut replacement = route.replacement.subscribe();
        let generation = *replacement.borrow();
        let connection = route.connection.read().await.clone();
        let session = route.session.read().await.clone();
        match connection
            .request_polis(&session, sequence, message_kind, payload.clone())
            .await
        {
            Ok(response) => {
                acknowledge_outbound(&route, sequence, message_kind, payload_sha256).await?;
                Ok(response)
            }
            Err(_) => {
                tokio::time::timeout(std::time::Duration::from_secs(2), async {
                    while *replacement.borrow() == generation {
                        replacement
                            .changed()
                            .await
                            .map_err(|_| PolisRuntimeError::Network)?;
                    }
                    Ok::<(), PolisRuntimeError>(())
                })
                .await
                .map_err(|_| PolisRuntimeError::Network)??;
                let replacement_connection = route.connection.read().await.clone();
                let replacement_session = route.session.read().await.clone();
                let replacement_sequence =
                    reserve_outbound(&route, message_kind, payload_sha256).await?;
                let response = replacement_connection
                    .request_polis(
                        &replacement_session,
                        replacement_sequence,
                        message_kind,
                        payload,
                    )
                    .await
                    .map_err(|_| PolisRuntimeError::Network)?;
                acknowledge_outbound(&route, replacement_sequence, message_kind, payload_sha256)
                    .await?;
                Ok(response)
            }
        }
    }

    async fn validate_ready(&self) -> Result<(), PolisRuntimeError> {
        let cut = self.cut.read().await;
        let expected = cut
            .routes()
            .keys()
            .copied()
            .filter(|node| *node != self.local)
            .collect::<BTreeSet<_>>();
        let actual = self
            .connections
            .read()
            .await
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        if actual != expected {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let routes = self.connections.read().await;
        for (peer, route) in routes.iter() {
            let connection = route.connection.read().await;
            let session = route.session.read().await;
            if !cut.session_matches(self.local, *peer, &connection, &session) {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
        }
        Ok(())
    }

    pub fn connection_owner(local: NodeId, peer: NodeId) -> Result<NodeId, PolisRuntimeError> {
        if local == 0 || peer == 0 || local == peer {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        Ok(local.min(peer))
    }
}

async fn reserve_outbound(
    route: &SecurePeerRoute,
    message_kind: &'static str,
    payload_sha256: [u8; 32],
) -> Result<u64, PolisRuntimeError> {
    let mut state = route.sequence.lock().await;
    if let Some(outstanding) = &state.outstanding {
        if outstanding.message_kind != message_kind || outstanding.payload_sha256 != payload_sha256
        {
            return Err(PolisRuntimeError::Replay);
        }
        return Ok(outstanding.sequence);
    }
    let sequence = state
        .acknowledged
        .checked_add(1)
        .ok_or(PolisRuntimeError::Replay)?;
    state.outstanding = Some(OutstandingRequest {
        sequence,
        message_kind,
        payload_sha256,
    });
    Ok(sequence)
}

async fn acknowledge_outbound(
    route: &SecurePeerRoute,
    sequence: u64,
    message_kind: &'static str,
    payload_sha256: [u8; 32],
) -> Result<(), PolisRuntimeError> {
    let mut state = route.sequence.lock().await;
    if state.outstanding.as_ref()
        != Some(&OutstandingRequest {
            sequence,
            message_kind,
            payload_sha256,
        })
    {
        return Err(PolisRuntimeError::StateRegression);
    }
    if sequence
        != state
            .acknowledged
            .checked_add(1)
            .ok_or(PolisRuntimeError::Replay)?
    {
        return Err(PolisRuntimeError::StateRegression);
    }
    state.acknowledged = sequence;
    state.outstanding = None;
    Ok(())
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BootGenerationState {
    generation: u64,
}

impl CheckpointMetadataSource for BootGenerationState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata::default())
    }
}

pub fn advance_secure_boot_generation(
    state_root: &Path,
    node_id: NodeId,
    checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
) -> Result<u64, PolisRuntimeError> {
    if node_id == 0 {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    let (durable, current) = CheckpointedJson::open(
        state_root,
        &format!("raft-boot-node-{node_id}"),
        "raft-boot-generation.json",
        BootGenerationState::default(),
        checkpoint_authority,
    )?;
    let generation = current
        .payload
        .generation
        .checked_add(1)
        .ok_or(PolisRuntimeError::StateRegression)?;
    durable.commit(&current, BootGenerationState { generation })?;
    Ok(generation)
}

pub async fn new_secure_raft_node(
    node_id: NodeId,
    state_root: &Path,
    network: SecurePolisNetworkFactory,
    checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
) -> Result<(PolisRaft, PolisStateMachineStore), PolisRuntimeError> {
    if node_id == 0 {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    network.validate_ready().await?;
    let log_store = PolisLogStore::open(state_root, node_id, Arc::clone(&checkpoint_authority))?;
    let state_machine = PolisStateMachineStore::open(state_root, node_id, checkpoint_authority)?;
    let configuration = Arc::new(
        openraft::Config {
            cluster_name: "adl-secure-polis".to_owned(),
            heartbeat_interval: 100,
            election_timeout_min: 300,
            election_timeout_max: 600,
            snapshot_policy: openraft::SnapshotPolicy::LogsSinceLast(16),
            max_in_snapshot_log_to_keep: 8,
            ..Default::default()
        }
        .validate()
        .map_err(|_| PolisRuntimeError::InvalidConfiguration)?,
    );
    let raft = PolisRaft::new(
        node_id,
        configuration,
        network,
        log_store,
        state_machine.clone(),
    )
    .await
    .map_err(|_| PolisRuntimeError::Storage)?;
    Ok((raft, state_machine))
}

impl RaftNetworkFactory<PolisTypeConfig> for SecurePolisNetworkFactory {
    type Network = SecurePolisNetworkConnection;

    async fn new_client(&mut self, target: NodeId, _node: &BasicNode) -> Self::Network {
        SecurePolisNetworkConnection {
            target,
            factory: self.clone(),
        }
    }
}

pub struct SecurePolisNetworkConnection {
    target: NodeId,
    factory: SecurePolisNetworkFactory,
}

impl SecurePolisNetworkConnection {
    async fn send<Req, Resp, E>(
        &self,
        message_kind: &'static str,
        request: Req,
    ) -> Result<Resp, RPCError<NodeId, BasicNode, RaftError<NodeId, E>>>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        E: std::error::Error + Serialize + DeserializeOwned + Send + Sync + 'static,
    {
        let payload = serde_jcs::to_vec(&request)
            .map_err(|_| RPCError::Network(NetworkError::new(&PolisRuntimeError::Serialization)))?;
        let response = self
            .factory
            .request_bytes(self.target, message_kind, payload)
            .await
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
        if response.len() > MAX_RPC_BYTES {
            return Err(RPCError::Network(NetworkError::new(
                &PolisRuntimeError::FrameTooLarge,
            )));
        }
        let result: Result<Resp, RaftError<NodeId, E>> = serde_json::from_slice(&response)
            .map_err(|_| RPCError::Network(NetworkError::new(&PolisRuntimeError::Serialization)))?;
        result.map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
    }
}

impl RaftNetwork<PolisTypeConfig> for SecurePolisNetworkConnection {
    async fn append_entries(
        &mut self,
        request: AppendEntriesRequest<PolisTypeConfig>,
        option: RPCOption,
    ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, BasicNode, RaftError<NodeId>>> {
        tokio::time::timeout(option.hard_ttl(), self.send("append_entries", request))
            .await
            .map_err(|_| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))?
    }

    async fn install_snapshot(
        &mut self,
        request: InstallSnapshotRequest<PolisTypeConfig>,
        option: RPCOption,
    ) -> Result<
        InstallSnapshotResponse<NodeId>,
        RPCError<NodeId, BasicNode, RaftError<NodeId, InstallSnapshotError>>,
    > {
        tokio::time::timeout(option.hard_ttl(), self.send("install_snapshot", request))
            .await
            .map_err(|_| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))?
    }

    async fn vote(
        &mut self,
        request: VoteRequest<NodeId>,
        option: RPCOption,
    ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, BasicNode, RaftError<NodeId>>> {
        tokio::time::timeout(option.hard_ttl(), self.send("vote", request))
            .await
            .map_err(|_| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))?
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayResponseState {
    highest_sequence: u64,
    entries: BTreeMap<u64, CachedRpcResponse>,
}

impl CheckpointMetadataSource for ReplayResponseState {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        Ok(CheckpointMetadata::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CachedRpcResponse {
    request_sha256: String,
    response: Vec<u8>,
}

#[derive(Clone)]
pub struct DurableRpcResponses {
    durable: Arc<CheckpointedJson<ReplayResponseState>>,
    state: Arc<tokio::sync::Mutex<DurableEnvelope<ReplayResponseState>>>,
    dispatch: Arc<tokio::sync::Mutex<()>>,
    local_node_id: String,
    peer_node_id: String,
    local_certificate_generation: u64,
    local_boot_generation: u64,
    peer_certificate_generation: u64,
    peer_boot_generation: u64,
    committed_membership_index: u64,
    session_namespace_sha256: [u8; 32],
    max_entries: usize,
}

fn rpc_session_namespace(binding: &PolisSessionBinding) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"adl-polis-rpc-session-v1\0");
    for value in [
        binding.polis_id(),
        binding.trust_domain(),
        binding.local_node_id(),
        binding.local_guardian_id(),
        binding.peer_node_id(),
        binding.peer_guardian_id(),
    ] {
        hasher.update(u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
        hasher.update(value.as_bytes());
    }
    for value in [
        binding.local_certificate_generation(),
        binding.local_boot_generation(),
        binding.peer_certificate_generation(),
        binding.peer_boot_generation(),
        binding.committed_membership_index(),
    ] {
        hasher.update(value.to_be_bytes());
    }
    hasher.update(binding.local_control_public_key());
    hasher.update(binding.peer_control_public_key());
    hasher.finalize().into()
}

impl DurableRpcResponses {
    pub fn open(
        root: &Path,
        local: NodeId,
        peer: NodeId,
        session: &EstablishedPolisSession,
        max_entries: usize,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
        let binding = session.binding();
        let session_namespace_sha256 = rpc_session_namespace(binding);
        Self::open_inner(
            root,
            local,
            peer,
            binding.local_node_id().to_owned(),
            binding.peer_node_id().to_owned(),
            binding.local_certificate_generation(),
            binding.local_boot_generation(),
            binding.peer_certificate_generation(),
            binding.peer_boot_generation(),
            binding.committed_membership_index(),
            session_namespace_sha256,
            max_entries,
            authority,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn open_inner(
        root: &Path,
        local: NodeId,
        peer: NodeId,
        local_node_id: String,
        peer_node_id: String,
        local_certificate_generation: u64,
        local_boot_generation: u64,
        peer_certificate_generation: u64,
        peer_boot_generation: u64,
        committed_membership_index: u64,
        session_namespace_sha256: [u8; 32],
        max_entries: usize,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
        if local == 0
            || peer == 0
            || local == peer
            || validate_text(&local_node_id).is_err()
            || validate_text(&peer_node_id).is_err()
            || local_certificate_generation == 0
            || local_boot_generation == 0
            || peer_certificate_generation == 0
            || peer_boot_generation == 0
            || committed_membership_index == 0
            || session_namespace_sha256 == [0; 32]
            || !(8..=4096).contains(&max_entries)
        {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        let object = format!("raft-rpc-session-{}", hex::encode(session_namespace_sha256));
        let (durable, state) = CheckpointedJson::open(
            root,
            &object,
            &format!("{object}.json"),
            ReplayResponseState::default(),
            authority,
        )?;
        Ok(Self {
            durable: Arc::new(durable),
            state: Arc::new(tokio::sync::Mutex::new(state)),
            dispatch: Arc::new(tokio::sync::Mutex::new(())),
            local_node_id,
            peer_node_id,
            local_certificate_generation,
            local_boot_generation,
            peer_certificate_generation,
            peer_boot_generation,
            committed_membership_index,
            session_namespace_sha256,
            max_entries,
        })
    }

    pub async fn lookup(
        &self,
        sequence: u64,
        request_sha256: &[u8; 32],
    ) -> Result<Option<Vec<u8>>, PolisRuntimeError> {
        let state = self.state.lock().await;
        if let Some(cached) = state.payload.entries.get(&sequence) {
            return if cached.request_sha256 == hex::encode(request_sha256) {
                Ok(Some(cached.response.clone()))
            } else {
                Err(PolisRuntimeError::Replay)
            };
        }
        if sequence <= state.payload.highest_sequence {
            return Err(PolisRuntimeError::Replay);
        }
        if sequence
            != state
                .payload
                .highest_sequence
                .checked_add(1)
                .ok_or(PolisRuntimeError::Replay)?
        {
            return Err(PolisRuntimeError::Replay);
        }
        Ok(None)
    }

    pub async fn dispatch_once<F, Fut>(
        &self,
        sequence: u64,
        request_sha256: &[u8; 32],
        dispatch: F,
    ) -> Result<Vec<u8>, PolisRuntimeError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Vec<u8>, PolisRuntimeError>>,
    {
        let _dispatch = self.dispatch.lock().await;
        if let Some(cached) = self.lookup(sequence, request_sha256).await? {
            return Ok(cached);
        }
        let response = dispatch().await?;
        self.commit(sequence, request_sha256, response.clone())
            .await?;
        Ok(response)
    }

    pub async fn commit(
        &self,
        sequence: u64,
        request_sha256: &[u8; 32],
        response: Vec<u8>,
    ) -> Result<(), PolisRuntimeError> {
        if response.len() > MAX_RPC_BYTES {
            return Err(PolisRuntimeError::FrameTooLarge);
        }
        let mut state = self.state.lock().await;
        if let Some(cached) = state.payload.entries.get(&sequence) {
            return if cached.request_sha256 == hex::encode(request_sha256)
                && cached.response == response
            {
                Ok(())
            } else {
                Err(PolisRuntimeError::Replay)
            };
        }
        if sequence <= state.payload.highest_sequence {
            return Err(PolisRuntimeError::Replay);
        }
        if sequence
            != state
                .payload
                .highest_sequence
                .checked_add(1)
                .ok_or(PolisRuntimeError::Replay)?
        {
            return Err(PolisRuntimeError::Replay);
        }
        let mut candidate = state.payload.clone();
        candidate.highest_sequence = sequence;
        candidate.entries.insert(
            sequence,
            CachedRpcResponse {
                request_sha256: hex::encode(request_sha256),
                response,
            },
        );
        while candidate.entries.len() > self.max_entries {
            let first = *candidate
                .entries
                .keys()
                .next()
                .ok_or(PolisRuntimeError::Storage)?;
            candidate.entries.remove(&first);
        }
        *state = self.durable.commit(&state, candidate)?;
        Ok(())
    }
}

pub async fn serve_secure_raft_connection(
    raft: PolisRaft,
    connection: Arc<AuthenticatedConnection>,
    session: EstablishedPolisSession,
    limits: TransportLimits,
    responses: DurableRpcResponses,
    cancellation: CancellationToken,
) -> Result<(), PolisRuntimeError> {
    if session.binding().local_node_id() != responses.local_node_id
        || session.binding().peer_node_id() != responses.peer_node_id
        || session.binding().local_certificate_generation()
            != responses.local_certificate_generation
        || session.binding().local_boot_generation() != responses.local_boot_generation
        || session.binding().peer_certificate_generation() != responses.peer_certificate_generation
        || session.binding().peer_boot_generation() != responses.peer_boot_generation
        || session.binding().committed_membership_index() != responses.committed_membership_index
        || rpc_session_namespace(session.binding()) != responses.session_namespace_sha256
    {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    let capacity = Arc::new(tokio::sync::Semaphore::new(32));
    loop {
        let permit = tokio::select! {
            _ = cancellation.cancelled() => return Ok(()),
            result = Arc::clone(&capacity).acquire_owned() => {
                result.map_err(|_| PolisRuntimeError::Network)?
            }
        };
        let incoming = tokio::select! {
            _ = cancellation.cancelled() => return Ok(()),
            result = connection.accept_polis_request(&session) => {
                result.map_err(|_| PolisRuntimeError::Network)?
            }
        };
        let raft = raft.clone();
        let responses = responses.clone();
        let limits = limits.clone();
        tokio::spawn(async move {
            let _permit = permit;
            let request_sha256 = incoming.request_sha256;
            let result = async {
                let response = responses
                    .dispatch_once(incoming.sequence, &request_sha256, || {
                        dispatch_raft_rpc(&raft, &incoming)
                    })
                    .await?;
                incoming
                    .respond(response, &limits)
                    .await
                    .map_err(|_| PolisRuntimeError::Network)
            }
            .await;
            if result.is_err() {
                // The connection-level authority and OpenRaft retry path own recovery.
            }
        });
    }
}

async fn dispatch_raft_rpc(
    raft: &PolisRaft,
    request: &IncomingPolisRequest,
) -> Result<Vec<u8>, PolisRuntimeError> {
    match request.message_kind.as_str() {
        "append_entries" => {
            let value: AppendEntriesRequest<PolisTypeConfig> =
                decode_bounded_json(&request.payload)?;
            encode_bounded_json(&raft.append_entries(value).await)
        }
        "install_snapshot" => {
            let value: InstallSnapshotRequest<PolisTypeConfig> =
                decode_bounded_json(&request.payload)?;
            encode_bounded_json(&raft.install_snapshot(value).await)
        }
        "vote" => {
            let value: VoteRequest<NodeId> = decode_bounded_json(&request.payload)?;
            encode_bounded_json(&raft.vote(value).await)
        }
        _ => Err(PolisRuntimeError::Authentication),
    }
}

fn encode_bounded_json<T: Serialize>(value: &T) -> Result<Vec<u8>, PolisRuntimeError> {
    let bytes = serde_jcs::to_vec(value).map_err(|_| PolisRuntimeError::Serialization)?;
    if bytes.len() > MAX_RPC_BYTES {
        return Err(PolisRuntimeError::FrameTooLarge);
    }
    Ok(bytes)
}

fn decode_bounded_json<T: DeserializeOwned + Serialize>(
    bytes: &[u8],
) -> Result<T, PolisRuntimeError> {
    if bytes.len() > MAX_RPC_BYTES {
        return Err(PolisRuntimeError::FrameTooLarge);
    }
    let value = serde_json::from_slice(bytes).map_err(|_| PolisRuntimeError::Serialization)?;
    if serde_jcs::to_vec(&value).map_err(|_| PolisRuntimeError::Serialization)? != bytes {
        return Err(PolisRuntimeError::Serialization);
    }
    Ok(value)
}

pub fn derive_authority_cut(
    polis: &PolisIdentityBinding,
    established: &EstablishedRuntimeAuthority,
    addresses: &BTreeMap<String, std::net::SocketAddr>,
    now_unix_seconds: i64,
) -> Result<VerifiedPolisRouteCut, PolisRuntimeError> {
    VerifiedPolisRouteCut::verify(polis, established, addresses, now_unix_seconds)
        .map_err(|_| PolisRuntimeError::AuthorityDenied)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolisRuntimeError {
    InvalidConfiguration,
    InvalidText,
    InvalidDigest,
    Serialization,
    Storage,
    Network,
    Authentication,
    FrameTooLarge,
    Replay,
    StateRegression,
    AuthorityDenied,
}

impl PolisRuntimeError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidConfiguration => "polis_invalid_configuration",
            Self::InvalidText => "polis_invalid_text",
            Self::InvalidDigest => "polis_invalid_digest",
            Self::Serialization => "polis_serialization_failed",
            Self::Storage => "polis_storage_failed",
            Self::Network => "polis_network_failed",
            Self::Authentication => "polis_authentication_failed",
            Self::FrameTooLarge => "polis_frame_too_large",
            Self::Replay => "polis_replay_rejected",
            Self::StateRegression => "polis_state_regression",
            Self::AuthorityDenied => "polis_authority_denied",
        }
    }
}

impl std::fmt::Display for PolisRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for PolisRuntimeError {}

fn validate_text(value: &str) -> Result<(), PolisRuntimeError> {
    if value.is_empty()
        || value.len() > MAX_TEXT_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(PolisRuntimeError::InvalidText);
    }
    Ok(())
}

fn validate_sha256(value: &str) -> Result<(), PolisRuntimeError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(PolisRuntimeError::InvalidDigest);
    }
    Ok(())
}

fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, PolisRuntimeError> {
    let bytes = serde_jcs::to_vec(value).map_err(|_| PolisRuntimeError::Serialization)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn canonical_snapshot_id(state: &PersistedStateMachine, bytes: &[u8]) -> String {
    format!(
        "polis-{}-{}",
        state.application.committed_index,
        &hex::encode(Sha256::digest(bytes))[..16]
    )
}

fn acquire_writer_lock(path: &Path) -> Result<File, PolisRuntimeError> {
    let parent = path
        .parent()
        .ok_or(PolisRuntimeError::InvalidConfiguration)?;
    validate_path_components(parent, false).map_err(|_| PolisRuntimeError::Storage)?;
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(PolisRuntimeError::Storage),
    }
    let mut options = OpenOptions::new();
    options.create(true).read(true).write(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(path).map_err(|_| PolisRuntimeError::Storage)?;
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PolisRuntimeError::Storage)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    file.try_lock_exclusive()
        .map_err(|_| PolisRuntimeError::StateRegression)?;
    Ok(file)
}

fn ensure_store_root(root: &Path) -> Result<(), PolisRuntimeError> {
    if !root.is_absolute() {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    validate_path_components(root, true).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    std::fs::create_dir_all(root).map_err(|_| PolisRuntimeError::Storage)?;
    validate_path_components(root, false).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    let metadata = std::fs::symlink_metadata(root).map_err(|_| PolisRuntimeError::Storage)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    Ok(())
}

fn read_bounded_json<T: DeserializeOwned + Serialize>(path: &Path) -> Result<T, PolisRuntimeError> {
    validate_path_components(path, false).map_err(|_| PolisRuntimeError::Storage)?;
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PolisRuntimeError::Storage)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() > MAX_RPC_BYTES as u64
    {
        return Err(PolisRuntimeError::Storage);
    }
    let mut file = File::open(path).map_err(|_| PolisRuntimeError::Storage)?;
    let opened = file.metadata().map_err(|_| PolisRuntimeError::Storage)?;
    if !opened.is_file() || opened.len() > MAX_RPC_BYTES as u64 {
        return Err(PolisRuntimeError::Storage);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.dev() != opened.dev() || metadata.ino() != opened.ino() {
            return Err(PolisRuntimeError::Storage);
        }
    }
    let mut bytes = Vec::with_capacity(opened.len().min(MAX_RPC_BYTES as u64) as usize);
    Read::by_ref(&mut file)
        .take((MAX_RPC_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| PolisRuntimeError::Storage)?;
    if bytes.len() > MAX_RPC_BYTES {
        return Err(PolisRuntimeError::Storage);
    }
    let value = serde_json::from_slice(&bytes).map_err(|_| PolisRuntimeError::Storage)?;
    if serde_jcs::to_vec(&value).map_err(|_| PolisRuntimeError::Storage)? != bytes {
        return Err(PolisRuntimeError::Storage);
    }
    Ok(value)
}

fn atomic_json_write<T: Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    let bytes = serde_jcs::to_vec(value)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    atomic_bytes_write(path, &bytes)
}

fn atomic_bytes_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if bytes.len() > MAX_RPC_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "state exceeds bounded size",
        ));
    }
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "state path has no parent")
    })?;
    validate_path_components(parent, false)?;
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "state path is not an ordinary file",
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed) + 1;
    let temp = parent.join(format!(".polis-{}-{sequence}.tmp", std::process::id()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&temp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    std::fs::rename(&temp, path)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn remove_file_and_sync(path: &Path) -> Result<(), PolisRuntimeError> {
    let parent = path.parent().ok_or(PolisRuntimeError::Storage)?;
    match std::fs::remove_file(path) {
        Ok(()) => File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| PolisRuntimeError::Storage),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(PolisRuntimeError::Storage),
    }
}

fn validate_path_components(path: &Path, allow_missing: bool) -> std::io::Result<()> {
    if !path.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path must be absolute",
        ));
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        if current.parent().is_none() {
            continue;
        }
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "path contains a symbolic link",
                    ));
                }
                if current != path && !metadata.is_dir() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "path ancestor is not a directory",
                    ));
                }
            }
            Err(error) if allow_missing && error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(());
            }
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
