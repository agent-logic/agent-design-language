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
        Arc, Mutex as StdMutex,
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

use super::super::{
    AuthenticatedConnection, EstablishedPolisSession, EstablishedRuntimeAuthority,
    IncomingPolisRequest, LearnerEndpointRole, PendingPolisResponse, PendingPolisSession,
    PolisIdentityBinding, PolisSessionBinding, RuntimeAuthorityInitializer,
    TransportAuthorityOwner, TransportLimits, TransportResult, VerifiedPolisRouteCut,
};
use super::learner_transport::{
    establish_learner_voter_sessions, establish_voter_learner_sessions,
    route_cut_digest as learner_route_cut_digest, EstablishedLearnerSession,
    LearnerBootAttestationCustody, LearnerIdentity, LearnerRpcKind, LearnerTransportError,
    LearnerVoterBinding, MembershipReceiptParts, PendingExclusionSnapshot,
    ProductionLearnerAuthority, VerifiedLearnerAdmission,
};
use crate::distributed::authority_protocol::{
    verify_replicated_finalization, AuthorityFinalizeProposal, AuthorityIntentEndorsement,
    AuthorityNodeIdentity, AuthorityPrepareProposal, AuthorityProtocolError,
    CanonicalAuthorityTime, DurableAuthorityProtocol, PrepareAuthorityIntent,
    PublishedAuthorityResult,
};
use crate::distributed::authority_reconciliation::{
    AuthorityReconciliationBarrier, AuthorityReconciliationError, PublishedReconciliationResult,
};
use crate::distributed::authority_store_adapters::AuthorityBoundCertificateStore;
use crate::distributed::certificates::AuthorityCertificate;
#[cfg(test)]
use crate::distributed::certificates::DistributedCertificateStore;
use crate::distributed::identity::LocalNodeGuardianIdentity;
use crate::distributed::lease::{AuthorityMembership, VoterAuthority};
use crate::distributed::membership::MembershipPolicy;
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
        let cleanup_permits = Arc::new(std::sync::Mutex::new(client.cleanup_permits()?));
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
        certificate_store: AuthorityBoundCertificateStore,
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

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn restore_configured_for_test(
        certificate_store: Arc<DistributedCertificateStore>,
        membership_policy: MembershipPolicy,
        membership_snapshot: &[u8],
        trusted_membership_commitment: [u8; 32],
    ) -> TransportResult<Self> {
        Ok(Self {
            initializer: RuntimeAuthorityInitializer::restore_for_test(
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
    PrepareAuthority {
        proposal: Box<AuthorityPrepareProposal>,
        boot_generations: Vec<AuthorityBootGeneration>,
    },
    FinalizeAuthority {
        proposal: AuthorityFinalizeProposal,
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
pub struct AuthorityBootGeneration {
    pub guardian_id: Vec<u8>,
    pub generation: u64,
}

fn canonical_boot_generations(
    boot_generations: &BTreeMap<Vec<u8>, u64>,
) -> Vec<AuthorityBootGeneration> {
    boot_generations
        .iter()
        .map(|(guardian_id, generation)| AuthorityBootGeneration {
            guardian_id: guardian_id.clone(),
            generation: *generation,
        })
        .collect()
}

fn decode_boot_generations(
    entries: &[AuthorityBootGeneration],
) -> Result<BTreeMap<Vec<u8>, u64>, PolisRuntimeError> {
    let decoded = entries
        .iter()
        .map(|entry| (entry.guardian_id.clone(), entry.generation))
        .collect::<BTreeMap<_, _>>();
    if decoded.len() != entries.len()
        || decoded.values().any(|generation| *generation == 0)
        || canonical_boot_generations(&decoded) != entries
    {
        return Err(PolisRuntimeError::AuthorityDenied);
    }
    Ok(decoded)
}

fn validate_boot_generations_for_authority(
    authority: &AuthorityMembership,
    boot_generations: &BTreeMap<Vec<u8>, u64>,
) -> Result<(), PolisRuntimeError> {
    let canonical = canonical_boot_generations(boot_generations);
    let decoded = decode_boot_generations(&canonical)?;
    if decoded.len() != authority.voters.len()
        || decoded
            .keys()
            .any(|guardian| !authority.voters.contains_key(guardian))
    {
        return Err(PolisRuntimeError::AuthorityDenied);
    }
    Ok(())
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
    #[serde(default)]
    current_authority: Option<ReplicatedAuthorityCustody>,
    prepared_authority: BTreeMap<String, ReplicatedPreparedAuthority>,
    finalized_authority: BTreeMap<String, ReplicatedFinalizedAuthority>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplicatedPreparedAuthority {
    intent: PrepareAuthorityIntent,
    authority: ReplicatedAuthorityCustody,
    boot_generations: Vec<AuthorityBootGeneration>,
    custody_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplicatedAuthorityCustody {
    polis_id: String,
    membership_epoch: u64,
    trust_domain_id: Vec<u8>,
    voter_set_generation: u64,
    committed_log_index: u64,
    configs: Vec<BTreeSet<Vec<u8>>>,
    voters: Vec<VoterAuthority>,
}

impl ReplicatedAuthorityCustody {
    fn same_committed_authority(&self, other: &Self) -> bool {
        self.polis_id == other.polis_id
            && self.membership_epoch == other.membership_epoch
            && self.trust_domain_id == other.trust_domain_id
            && self.voter_set_generation == other.voter_set_generation
            && self.committed_log_index == other.committed_log_index
            && self.configs == other.configs
            && self.voters == other.voters
    }

    fn from_authority(
        polis_id: &str,
        membership_epoch: u64,
        authority: &AuthorityMembership,
    ) -> Result<Self, PolisRuntimeError> {
        validate_text(polis_id)?;
        if membership_epoch == 0 {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let guardian_by_raft = authority
            .raft_ids
            .iter()
            .map(|(guardian, raft_id)| (*raft_id, guardian.clone()))
            .collect::<BTreeMap<_, _>>();
        let configs = authority
            .raft_membership
            .get_joint_config()
            .iter()
            .map(|config| {
                config
                    .iter()
                    .map(|raft_id| {
                        guardian_by_raft
                            .get(raft_id)
                            .cloned()
                            .ok_or(PolisRuntimeError::AuthorityDenied)
                    })
                    .collect::<Result<BTreeSet<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let custody = Self {
            polis_id: polis_id.to_owned(),
            membership_epoch,
            trust_domain_id: authority.trust_domain_id.clone(),
            voter_set_generation: authority.voter_set_generation,
            committed_log_index: authority.committed_log_index,
            configs,
            voters: authority.voters.values().cloned().collect(),
        };
        if custody.to_authority()? != *authority {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        Ok(custody)
    }

    fn to_authority(&self) -> Result<AuthorityMembership, PolisRuntimeError> {
        let authority = AuthorityMembership::new(
            self.trust_domain_id.clone(),
            self.voter_set_generation,
            self.committed_log_index,
            self.configs.clone(),
            self.voters.clone(),
        )
        .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        Ok(authority)
    }
}

fn prepared_custody_sha256(
    authority: &ReplicatedAuthorityCustody,
    boot_generations: &[AuthorityBootGeneration],
) -> Result<String, PolisRuntimeError> {
    canonical_sha256(&(authority, boot_generations))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplicatedFinalizedAuthority {
    proposal: AuthorityFinalizeProposal,
    committed_log_index: u64,
}

impl PolisApplicationState {
    fn install_trusted_authority(
        &mut self,
        polis_id: &str,
        membership_epoch: u64,
        authority: AuthorityMembership,
    ) -> Result<(), PolisRuntimeError> {
        let authority =
            ReplicatedAuthorityCustody::from_authority(polis_id, membership_epoch, &authority)?;
        if let Some(current) = self.current_authority.as_ref() {
            return if current == &authority {
                Ok(())
            } else {
                Err(PolisRuntimeError::AuthorityDenied)
            };
        }
        self.current_authority = Some(authority);
        Ok(())
    }

    pub fn prepared_authority_intent(&self, operation_id: &str) -> Option<&PrepareAuthorityIntent> {
        self.prepared_authority
            .get(operation_id)
            .map(|prepared| &prepared.intent)
    }

    #[cfg(test)]
    fn finalized_authority_log_index(&self, operation_id: &str) -> Option<u64> {
        self.finalized_authority
            .get(operation_id)
            .map(|finalized| finalized.committed_log_index)
    }

    /// Applies a command at the index assigned by the committed OpenRaft log.
    /// Authority protocol indices in serialized command bytes are bindings,
    /// never an alternate source of commit authority.
    fn apply_committed(
        &mut self,
        index: u64,
        command: &PolisCommand,
        trusted_custody: Option<&ReplicatedAuthorityCustody>,
        trusted_boot_generations: Option<&BTreeMap<Vec<u8>, u64>>,
    ) -> Result<bool, PolisRuntimeError> {
        if index <= self.committed_index {
            return Err(PolisRuntimeError::StateRegression);
        }
        validate_authority_command_boundary(command)?;
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
            PolisCommand::PrepareAuthority {
                proposal,
                boot_generations,
            } => {
                let current = self
                    .current_authority
                    .as_ref()
                    .ok_or(PolisRuntimeError::AuthorityDenied)?;
                let trusted = trusted_custody.unwrap_or(current);
                if !current.same_committed_authority(trusted) {
                    return Err(PolisRuntimeError::AuthorityDenied);
                }
                let authority = current.to_authority()?;
                let boot_generations = decode_boot_generations(boot_generations)?;
                let trusted_boot_generations =
                    trusted_boot_generations.ok_or(PolisRuntimeError::AuthorityDenied)?;
                if &boot_generations != trusted_boot_generations {
                    return Err(PolisRuntimeError::AuthorityDenied);
                }
                accepted = (|| {
                    let intent = proposal
                        .commit_at(
                            index,
                            &current.polis_id,
                            current.membership_epoch,
                            &authority,
                        )
                        .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
                    if self.prepared_authority.contains_key(&intent.operation_id)
                        || self.finalized_authority.contains_key(&intent.operation_id)
                    {
                        return Err(PolisRuntimeError::Replay);
                    }
                    let prepared_authority = current.clone();
                    let frozen_boot_generations = canonical_boot_generations(&boot_generations);
                    let custody_sha256 =
                        prepared_custody_sha256(&prepared_authority, &frozen_boot_generations)?;
                    self.prepared_authority.insert(
                        intent.operation_id.clone(),
                        ReplicatedPreparedAuthority {
                            intent,
                            authority: prepared_authority,
                            boot_generations: frozen_boot_generations,
                            custody_sha256,
                        },
                    );
                    Ok(())
                })()
                .is_ok();
            }
            PolisCommand::FinalizeAuthority { proposal } => {
                let verified = (|| {
                    let prepared = self
                        .prepared_authority
                        .get(&proposal.operation_id)
                        .ok_or(PolisRuntimeError::AuthorityDenied)?;
                    if self
                        .finalized_authority
                        .contains_key(&proposal.operation_id)
                    {
                        return Err(PolisRuntimeError::Replay);
                    }
                    let authority = prepared.authority.to_authority()?;
                    let boot_generations = decode_boot_generations(&prepared.boot_generations)?;
                    verify_replicated_finalization(
                        &prepared.intent,
                        proposal,
                        index,
                        &authority,
                        &boot_generations,
                    )
                    .map_err(|_| PolisRuntimeError::AuthorityDenied)
                })();
                if let Ok(verified) = verified {
                    self.finalized_authority.insert(
                        proposal.operation_id.clone(),
                        ReplicatedFinalizedAuthority {
                            proposal: proposal.clone(),
                            committed_log_index: verified.committed_log_index(),
                        },
                    );
                }
                // A committed finalize is only deterministic replicated truth.
                // No caller receives an authority token until node-local
                // journal/CAS reconciliation publishes the exact result.
                accepted = false;
            }
            PolisCommand::FenceVoter { .. }
            | PolisCommand::ActivateOwner { .. }
            | PolisCommand::ActivateShepherd { .. }
            | PolisCommand::AcquireObservatory { .. }
            | PolisCommand::DemoteVoter { .. } => {
                // These pre-authority-protocol commands could mint authority
                // from caller-controlled fields. Retained logs fail closed;
                // governed effects are applied only by their sealed adapters.
                return Err(PolisRuntimeError::AuthorityDenied);
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

/// Rejects the retired caller-field authority commands before replicated apply.
pub fn validate_authority_command_boundary(
    command: &PolisCommand,
) -> Result<(), PolisRuntimeError> {
    match command {
        PolisCommand::GovernedMutation { .. }
        | PolisCommand::SnapshotBoundary { .. }
        | PolisCommand::PrepareAuthority { .. }
        | PolisCommand::FinalizeAuthority { .. } => Ok(()),
        PolisCommand::FenceVoter { .. }
        | PolisCommand::ActivateOwner { .. }
        | PolisCommand::ActivateShepherd { .. }
        | PolisCommand::AcquireObservatory { .. }
        | PolisCommand::DemoteVoter { .. } => Err(PolisRuntimeError::AuthorityDenied),
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PersistedStateMachine {
    last_applied_log: Option<LogId<NodeId>>,
    last_membership: StoredMembership<NodeId, BasicNode>,
    #[serde(default)]
    membership_history: Vec<AppliedMembershipEntry>,
    application: PolisApplicationState,
}

const MAX_APPLIED_MEMBERSHIP_HISTORY: usize = 64;

/// One durably applied OpenRaft membership entry.
///
/// Joint configuration order is retained exactly as committed. Each
/// configuration is a `BTreeSet`, making member order canonical without
/// erasing the joint-consensus boundary between configurations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppliedMembershipEntry {
    pub log_id: LogId<NodeId>,
    pub joint_configs: Vec<BTreeSet<NodeId>>,
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
pub(crate) struct DurableEnvelope<T> {
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
pub(crate) struct CheckpointMetadata {
    pub(crate) committed_log_index: Option<u64>,
    pub(crate) state_sha256: Option<String>,
    pub(crate) snapshot_log_index: Option<u64>,
    pub(crate) snapshot_sha256: Option<String>,
}

pub(crate) trait CheckpointMetadataSource {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError>;
}

pub(crate) struct CheckpointedJson<T> {
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
    pub(crate) fn open(
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

    pub(crate) fn commit(
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

impl<T> DurableEnvelope<T> {
    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn payload_sha256(&self) -> &str {
        &self.payload_sha256
    }

    pub(crate) fn payload(&self) -> &T {
        &self.payload
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
    trusted_custody: Option<ReplicatedAuthorityCustody>,
    trusted_boot_generations: Option<BTreeMap<Vec<u8>, u64>>,
    authority_publication: Option<Arc<AuthorityPublicationContext>>,
}

struct TrustedAuthorityBootstrap {
    polis_id: String,
    membership_epoch: u64,
    authority: AuthorityMembership,
    boot_generations: BTreeMap<Vec<u8>, u64>,
    publication_identity: AuthorityNodeIdentity,
}

struct AuthorityPublicationContext {
    root: PathBuf,
    identity: AuthorityNodeIdentity,
    checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
}

impl PolisStateMachineStore {
    pub fn open(
        root: &Path,
        node_id: NodeId,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
        Self::open_internal(root, node_id, authority, None)
    }

    fn open_with_trusted_authority(
        root: &Path,
        node_id: NodeId,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        bootstrap: TrustedAuthorityBootstrap,
    ) -> Result<Self, PolisRuntimeError> {
        Self::open_internal(root, node_id, authority, Some(bootstrap))
    }

    fn open_internal(
        root: &Path,
        node_id: NodeId,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        trusted_authority: Option<TrustedAuthorityBootstrap>,
    ) -> Result<Self, PolisRuntimeError> {
        if node_id == 0 {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        if trusted_authority.is_some() {
            let publication_root = root.join("authority-publication");
            match std::fs::create_dir(&publication_root) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(_) => return Err(PolisRuntimeError::Storage),
            }
            ensure_store_root(&publication_root)?;
        }
        let mut initial = PersistedStateMachineStorage::default();
        if let Some(bootstrap) = trusted_authority.as_ref() {
            validate_boot_generations_for_authority(
                &bootstrap.authority,
                &bootstrap.boot_generations,
            )?;
            if bootstrap.publication_identity.polis_id != bootstrap.polis_id
                || bootstrap.publication_identity.boot_generation
                    != bootstrap
                        .boot_generations
                        .get(bootstrap.publication_identity.guardian_id.as_bytes())
                        .copied()
                        .ok_or(PolisRuntimeError::AuthorityDenied)?
                || bootstrap
                    .authority
                    .raft_ids
                    .get(bootstrap.publication_identity.guardian_id.as_bytes())
                    != Some(&node_id)
                || bootstrap.publication_identity.trust_domain.as_bytes()
                    != bootstrap.authority.trust_domain_id
            {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
            initial.current.application.install_trusted_authority(
                &bootstrap.polis_id,
                bootstrap.membership_epoch,
                bootstrap.authority.clone(),
            )?;
            initial.snapshot.state = initial.current.clone();
        }
        let (durable, inner) = CheckpointedJson::open(
            root,
            &format!("raft-state-node-{node_id}"),
            "raft-state.json",
            initial,
            Arc::clone(&authority),
        )?;
        let trusted_custody = trusted_authority
            .as_ref()
            .map(|bootstrap| {
                ReplicatedAuthorityCustody::from_authority(
                    &bootstrap.polis_id,
                    bootstrap.membership_epoch,
                    &bootstrap.authority,
                )
            })
            .transpose()?;
        let trusted_boot_generations = trusted_authority
            .as_ref()
            .map(|bootstrap| bootstrap.boot_generations.clone());
        let authority_publication = if let Some(bootstrap) = trusted_authority {
            let expected = ReplicatedAuthorityCustody::from_authority(
                &bootstrap.polis_id,
                bootstrap.membership_epoch,
                &bootstrap.authority,
            )?;
            if !inner
                .payload
                .current
                .application
                .current_authority
                .as_ref()
                .is_some_and(|current| current.same_committed_authority(&expected))
            {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
            Some(Arc::new(AuthorityPublicationContext {
                root: root.join("authority-publication"),
                identity: bootstrap.publication_identity,
                checkpoint_authority: Arc::clone(&authority),
            }))
        } else {
            None
        };
        Self::validate_application_authority(
            trusted_custody.as_ref(),
            &inner.payload.current.application,
        )?;
        Self::validate_application_authority(
            trusted_custody.as_ref(),
            &inner.payload.snapshot.state.application,
        )?;
        Ok(Self {
            durable: Arc::new(durable),
            inner: Arc::new(RwLock::new(inner)),
            trusted_custody,
            trusted_boot_generations,
            authority_publication,
        })
    }

    pub async fn application_state(&self) -> PolisApplicationState {
        self.inner.read().await.payload.current.application.clone()
    }

    /// Returns the bounded, durable OpenRaft membership-apply history.
    pub async fn applied_membership_history(&self) -> Vec<AppliedMembershipEntry> {
        self.inner
            .read()
            .await
            .payload
            .current
            .membership_history
            .clone()
    }

    pub async fn reconcile_authority_publication(
        &self,
        operation_id: &str,
    ) -> Result<PublishedAuthorityResult, PolisRuntimeError> {
        let publication = self
            .authority_publication
            .as_ref()
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        let (intent, finalized) = {
            let state = self.inner.read().await;
            let prepared = state
                .payload
                .current
                .application
                .prepared_authority
                .get(operation_id)
                .ok_or(PolisRuntimeError::AuthorityDenied)?;
            let finalized = state
                .payload
                .current
                .application
                .finalized_authority
                .get(operation_id)
                .ok_or(PolisRuntimeError::AuthorityDenied)?;
            (prepared.intent.clone(), finalized.clone())
        };
        let (authority, boot_generations) = {
            let state = self.inner.read().await;
            let prepared = state
                .payload
                .current
                .application
                .prepared_authority
                .get(operation_id)
                .ok_or(PolisRuntimeError::AuthorityDenied)?;
            (
                prepared.authority.to_authority()?,
                decode_boot_generations(&prepared.boot_generations)?,
            )
        };
        let verified = verify_replicated_finalization(
            &intent,
            &finalized.proposal,
            finalized.committed_log_index,
            &authority,
            &boot_generations,
        )
        .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        let mut protocol = DurableAuthorityProtocol::open(
            &publication.root,
            publication.identity.clone(),
            Arc::clone(&publication.checkpoint_authority),
        )
        .map_err(|_| PolisRuntimeError::Storage)?;
        protocol
            .publish(&intent, verified)
            .map_err(|_| PolisRuntimeError::Storage)
    }

    /// Carries only the opaque, locally published #201 result into the sealed
    /// concrete-store reconciliation registry. Raw commands and caller-created
    /// receipts never reach the barrier.
    pub async fn reconcile_concrete_authority(
        &self,
        operation_id: &str,
        barrier: &mut AuthorityReconciliationBarrier,
    ) -> Result<PublishedReconciliationResult, PolisRuntimeError> {
        let published = self.reconcile_authority_publication(operation_id).await?;
        barrier.reconcile(&published).map_err(|error| match error {
            AuthorityReconciliationError::Serialization => PolisRuntimeError::Serialization,
            AuthorityReconciliationError::StateRegression
            | AuthorityReconciliationError::CheckpointConflict => {
                PolisRuntimeError::StateRegression
            }
            AuthorityReconciliationError::Storage => PolisRuntimeError::Storage,
            AuthorityReconciliationError::CapacityExceeded => PolisRuntimeError::FrameTooLarge,
            _ => PolisRuntimeError::AuthorityDenied,
        })
    }

    fn validate_application_authority(
        trusted_custody: Option<&ReplicatedAuthorityCustody>,
        application: &PolisApplicationState,
    ) -> Result<(), PolisRuntimeError> {
        let Some(trusted) = trusted_custody else {
            return if application.current_authority.is_none()
                && application.prepared_authority.is_empty()
                && application.finalized_authority.is_empty()
            {
                Ok(())
            } else {
                Err(PolisRuntimeError::AuthorityDenied)
            };
        };
        let current_matches = application
            .current_authority
            .as_ref()
            .is_some_and(|current| current.same_committed_authority(trusted));
        if !current_matches
            || !application.fenced_voters.is_empty()
            || application.active_owner.is_some()
            || application.active_shepherd.is_some()
            || application.observatory_owner.is_some()
            || application.observatory_expires_unix_millis.is_some()
            || !application.demoted_voters.is_empty()
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let authority = trusted.to_authority()?;
        for (operation_id, prepared) in &application.prepared_authority {
            let custody_matches = prepared.authority.same_committed_authority(trusted);
            let prepared_boot_generations = decode_boot_generations(&prepared.boot_generations)?;
            let prepared_authority = prepared.authority.to_authority()?;
            if operation_id != &prepared.intent.operation_id
                || !custody_matches
                || validate_sha256(&prepared.custody_sha256).is_err()
                || prepared_custody_sha256(&prepared.authority, &prepared.boot_generations)?
                    != prepared.custody_sha256
                || prepared_boot_generations.len() != prepared_authority.voters.len()
                || prepared_boot_generations
                    .keys()
                    .any(|guardian| !prepared_authority.voters.contains_key(guardian))
                || prepared.intent.polis_id != trusted.polis_id
                || prepared.intent.membership_epoch != trusted.membership_epoch
                || prepared.intent.prepare_log_index > application.committed_index
            {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
            prepared
                .intent
                .validate_against_authority(&authority)
                .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        }
        for (operation_id, finalized) in &application.finalized_authority {
            let prepared = application
                .prepared_authority
                .get(operation_id)
                .ok_or(PolisRuntimeError::AuthorityDenied)?;
            if finalized.proposal.operation_id != *operation_id
                || finalized.committed_log_index > application.committed_index
            {
                return Err(PolisRuntimeError::AuthorityDenied);
            }
            let prepared_authority = prepared.authority.to_authority()?;
            let prepared_boot_generations = decode_boot_generations(&prepared.boot_generations)?;
            verify_replicated_finalization(
                &prepared.intent,
                &finalized.proposal,
                finalized.committed_log_index,
                &prepared_authority,
                &prepared_boot_generations,
            )
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        }
        Ok(())
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
                    candidate
                        .current
                        .membership_history
                        .push(AppliedMembershipEntry {
                            log_id: entry.log_id,
                            joint_configs: membership.get_joint_config().to_vec(),
                        });
                    let overflow = candidate
                        .current
                        .membership_history
                        .len()
                        .saturating_sub(MAX_APPLIED_MEMBERSHIP_HISTORY);
                    if overflow != 0 {
                        candidate.current.membership_history.drain(..overflow);
                    }
                    candidate.current.last_membership =
                        StoredMembership::new(Some(entry.log_id), membership);
                    (true, "raft_internal")
                }
                EntryPayload::Normal(command) => {
                    let finalized_operation = match &command {
                        PolisCommand::FinalizeAuthority { proposal } => {
                            Some(proposal.operation_id.clone())
                        }
                        _ => None,
                    };
                    let accepted = candidate
                        .current
                        .application
                        .apply_committed(
                            entry.log_id.index,
                            &command,
                            self.trusted_custody.as_ref(),
                            self.trusted_boot_generations.as_ref(),
                        )
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
                        } else if finalized_operation.is_some_and(|operation_id| {
                            candidate
                                .current
                                .application
                                .finalized_authority
                                .contains_key(&operation_id)
                        }) {
                            "authority_publication_pending"
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
        Self::validate_application_authority(self.trusted_custody.as_ref(), &current.application)
            .map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, error.code()),
            )
        })?;
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
    trusted_polis_id: String,
    trusted_cut_sha256: Arc<RwLock<[u8; 32]>>,
    trusted_trust_domain: String,
    trusted_membership_epoch: u64,
    trusted_authority: AuthorityMembership,
    trusted_boot_generations: BTreeMap<Vec<u8>, u64>,
    trusted_node_identities: BTreeMap<Vec<u8>, (String, u64)>,
    local_publication_identity: AuthorityNodeIdentity,
    connections: Arc<RwLock<BTreeMap<NodeId, SecurePeerRoute>>>,
    learner_connections: Arc<RwLock<BTreeMap<NodeId, SecureLearnerRoute>>>,
    learner_authority: ProductionLearnerAuthority,
    transport_owner: Arc<tokio::sync::Mutex<TransportAuthorityOwner>>,
    authority_transition: Arc<tokio::sync::Mutex<()>>,
}

/// Opaque durable observation returned by the governed membership authority.
/// Callers may journal this projection, but only the factory can construct it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernedMembershipAuthorityReceipt {
    operation_sha256: [u8; 32],
    generation: u64,
    published_state_sha256: [u8; 32],
}

impl GovernedMembershipAuthorityReceipt {
    fn from_parts(parts: MembershipReceiptParts) -> Self {
        Self {
            operation_sha256: parts.operation_sha256,
            generation: parts.generation,
            published_state_sha256: parts.published_state_sha256,
        }
    }

    pub fn operation_sha256(&self) -> [u8; 32] {
        self.operation_sha256
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn published_state_sha256(&self) -> [u8; 32] {
        self.published_state_sha256
    }

    #[cfg(test)]
    pub(crate) fn for_membership_coordinator_test(
        operation_sha256: [u8; 32],
        generation: u64,
        published_state_sha256: [u8; 32],
    ) -> Self {
        Self {
            operation_sha256,
            generation,
            published_state_sha256,
        }
    }
}

#[derive(Clone)]
struct SecurePeerRoute {
    connection: Arc<RwLock<Arc<AuthenticatedConnection>>>,
    session: Arc<RwLock<EstablishedPolisSession>>,
    dispatch_lock: Arc<tokio::sync::Mutex<()>>,
    sequence: Arc<tokio::sync::Mutex<OutboundSequenceState>>,
    replacement: watch::Sender<u64>,
}

#[derive(Clone)]
struct SecureLearnerRoute {
    connection: Arc<AuthenticatedConnection>,
    outbound: Arc<tokio::sync::Mutex<EstablishedLearnerSession>>,
    responses: Arc<tokio::sync::Mutex<EstablishedLearnerSession>>,
    dispatch_lock: Arc<tokio::sync::Mutex<()>>,
    sequence: Arc<AtomicU64>,
}

/// Learner-owned ingress bootstrap. Unlike `SecurePolisNetworkFactory`, this
/// endpoint is rooted in the admitted learner identity rather than pretending
/// that the learner is one of the three voters.
pub struct SecureLearnerNetworkFactory {
    cut: VerifiedPolisRouteCut,
    admission: VerifiedLearnerAdmission,
    custody: LearnerBootAttestationCustody,
    learner_authority: ProductionLearnerAuthority,
    transport_owner: tokio::sync::Mutex<TransportAuthorityOwner>,
    authority_transition: tokio::sync::Mutex<()>,
}

impl SecureLearnerNetworkFactory {
    #[allow(clippy::too_many_arguments)]
    pub async fn bootstrap(
        cut: VerifiedPolisRouteCut,
        admission: VerifiedLearnerAdmission,
        learner_authority: ProductionLearnerAuthority,
        identity: &LocalNodeGuardianIdentity,
        boot: SecureBootGenerationCustody,
        now_unix_seconds: i64,
    ) -> Result<Self, PolisRuntimeError> {
        if cut.len() != 3
            || !admission.is_live_at(now_unix_seconds)
            || !admission_matches_cut(&admission, &cut)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let custody =
            LearnerBootAttestationCustody::establish(boot, identity, admission.identity())
                .map_err(map_learner_error)?;
        let transport_owner = learner_authority
            .take_transport_owner()
            .map_err(map_learner_error)?;
        let cut_sha256 = learner_route_cut_digest(&cut).map_err(map_learner_error)?;
        let exclusion = learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if !exclusion.learner_route_allowed(
            &admission,
            cut.contains(admission.identity().stable_raft_id),
        ) {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let current = learner_authority
            .admission_snapshot()
            .map_err(map_learner_error)?
            .current()
            .cloned();
        if current
            .as_ref()
            .is_some_and(|value| !exclusion.learner_admission_allowed(value))
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        transport_owner
            .bind_current_view(
                cut_sha256,
                current
                    .as_ref()
                    .map(VerifiedLearnerAdmission::operation_sha256),
                exclusion.transport_identity(),
                exclusion.generation(),
            )
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        {
            let mut lease = transport_owner.write_lease().await;
            learner_authority
                .governed_activate_admission(&mut lease, &admission)
                .map_err(map_learner_error)?;
        }
        Ok(Self {
            cut,
            admission,
            custody,
            learner_authority,
            transport_owner: tokio::sync::Mutex::new(transport_owner),
            authority_transition: tokio::sync::Mutex::new(()),
        })
    }

    pub async fn server_sessions(
        &self,
        connection: &AuthenticatedConnection,
        now_unix_seconds: i64,
    ) -> Result<(EstablishedLearnerSession, EstablishedLearnerSession), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        if !self.admission.is_live_at(now_unix_seconds)
            || !admission_matches_cut(&self.admission, &self.cut)
            || !self
                .learner_authority
                .admission_is_current(&self.admission)
                .map_err(map_learner_error)?
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let voter = learner_voter_binding_for_connection(&self.cut, &self.admission, connection)?;
        let mut inbound = EstablishedLearnerSession::new(
            &self.admission,
            self.admission.voter_cut_sha256(),
            voter.clone(),
            LearnerEndpointRole::Learner,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        let mut outbound = EstablishedLearnerSession::new(
            &self.admission,
            self.admission.voter_cut_sha256(),
            voter,
            LearnerEndpointRole::Learner,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        inbound
            .validate_connection(connection)
            .map_err(map_learner_error)?;
        outbound
            .validate_connection(connection)
            .map_err(map_learner_error)?;
        drop(_transition);
        establish_learner_voter_sessions(connection, &mut inbound, &mut outbound, &self.custody)
            .await
            .map_err(map_learner_error)?;
        let _transition = self.authority_transition.lock().await;
        if !self
            .learner_authority
            .session_is_current(&inbound)
            .map_err(map_learner_error)?
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        let (peer_key, peer_instance) = inbound
            .peer_transport_instance()
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        self.learner_authority
            .pin_peer_instance(&mut lease, peer_key, peer_instance)
            .map_err(map_learner_error)?;
        Ok((inbound, outbound))
    }

    /// Atomically expires this learner's locally owned admission. Existing
    /// ingress sessions share the same authority handle, so the write lease
    /// waits for an in-flight Raft effect and its response and then makes every
    /// retained session fail before another governed stream can be accepted.
    pub async fn expire_admission(&self, now_unix_seconds: i64) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        self.learner_authority
            .governed_expire_admission(&mut lease, now_unix_seconds)
            .map(|_| ())
            .map_err(map_learner_error)
    }
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
        learner_authority: ProductionLearnerAuthority,
    ) -> Result<Self, PolisRuntimeError> {
        if local == 0 || !cut.contains(local) || cut.len() != 3 {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let transport_owner = learner_authority
            .take_transport_owner()
            .map_err(map_learner_error)?;
        let (node_id, guardian_id, boot_generation) = cut
            .authority_node_identity(local)
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        let trusted_cut_sha256 = learner_route_cut_digest(&cut).map_err(map_learner_error)?;
        let exclusion = learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        let admission = learner_authority
            .admission_snapshot()
            .map_err(map_learner_error)?
            .current()
            .cloned();
        if admission
            .as_ref()
            .is_some_and(|current| !exclusion.learner_admission_allowed(current))
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let current_learner_operation = admission
            .as_ref()
            .map(VerifiedLearnerAdmission::operation_sha256);
        transport_owner
            .bind_current_view(
                trusted_cut_sha256,
                current_learner_operation,
                exclusion.transport_identity(),
                exclusion.generation(),
            )
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        let trusted_node_identities = cut
            .routes()
            .keys()
            .filter_map(|raft_id| {
                cut.authority_node_identity(*raft_id).map(
                    |(node_id, guardian_id, boot_generation)| {
                        (guardian_id.into_bytes(), (node_id, boot_generation))
                    },
                )
            })
            .collect();
        Ok(Self {
            local,
            trusted_polis_id: cut.polis_id().to_owned(),
            trusted_cut_sha256: Arc::new(RwLock::new(trusted_cut_sha256)),
            trusted_trust_domain: cut.trust_domain().to_owned(),
            trusted_membership_epoch: cut.membership_epoch(),
            trusted_authority: cut.authority_membership().clone(),
            trusted_boot_generations: cut.authority_boot_generations(),
            trusted_node_identities,
            local_publication_identity: AuthorityNodeIdentity {
                trust_domain: cut.trust_domain().to_owned(),
                polis_id: cut.polis_id().to_owned(),
                node_id,
                guardian_id,
                boot_generation,
            },
            cut: Arc::new(RwLock::new(cut)),
            connections: Arc::new(RwLock::new(BTreeMap::new())),
            learner_connections: Arc::new(RwLock::new(BTreeMap::new())),
            learner_authority,
            transport_owner: Arc::new(tokio::sync::Mutex::new(transport_owner)),
            authority_transition: Arc::new(tokio::sync::Mutex::new(())),
        })
    }

    fn trusted_authority_bootstrap(&self) -> TrustedAuthorityBootstrap {
        TrustedAuthorityBootstrap {
            polis_id: self.trusted_polis_id.clone(),
            membership_epoch: self.trusted_membership_epoch,
            authority: self.trusted_authority.clone(),
            boot_generations: self.trusted_boot_generations.clone(),
            publication_identity: self.local_publication_identity.clone(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn endorse_committed_prepare(
        &self,
        identity: &LocalNodeGuardianIdentity,
        certificate_generation: u64,
        boot_generation: u64,
        membership_log_index: u64,
        intent: &PrepareAuthorityIntent,
        finalization_time: &CanonicalAuthorityTime,
        membership: &crate::distributed::membership::MembershipState,
    ) -> Result<AuthorityIntentEndorsement, AuthorityProtocolError> {
        let public = identity.public_identity();
        if public.trust_domain != self.local_publication_identity.trust_domain
            || public.node_id != self.local_publication_identity.node_id
            || public.guardian_id != self.local_publication_identity.guardian_id
            || boot_generation != self.local_publication_identity.boot_generation
            || membership_log_index != self.trusted_authority.committed_log_index
        {
            return Err(AuthorityProtocolError::WrongVoter);
        }
        self.learner_authority.endorse_committed_prepare(
            identity,
            certificate_generation,
            boot_generation,
            membership_log_index,
            &self.trusted_boot_generations,
            intent,
            finalization_time,
            membership,
            &self.trusted_authority,
        )
    }

    pub async fn pending_session(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
    ) -> Result<PendingPolisSession, PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        self.cut
            .read()
            .await
            .pending_session_with_exclusion(
                self.local,
                target,
                connection,
                self.learner_authority.transport_authority(),
            )
            .map_err(|_| PolisRuntimeError::AuthorityDenied)
    }

    pub async fn initiate_session(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<EstablishedPolisSession, PolisRuntimeError> {
        let pending = self.pending_session(target, connection).await?;
        let established = connection
            .initiate_polis_session(pending, signing_key)
            .await
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        self.pin_established_peer(target, connection, &established)
            .await?;
        Ok(established)
    }

    pub async fn accept_session(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<EstablishedPolisSession, PolisRuntimeError> {
        let pending = self.pending_session(target, connection).await?;
        let established = connection
            .accept_polis_session(pending, signing_key)
            .await
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        self.pin_established_peer(target, connection, &established)
            .await?;
        Ok(established)
    }

    async fn pin_established_peer(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        established: &EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if !self.cut.read().await.session_matches_with_exclusion(
            self.local,
            target,
            connection,
            established,
            &exclusion,
        ) {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        self.learner_authority
            .pin_peer_instance(
                &mut lease,
                established.peer_identity_key(),
                established.peer_authority_instance_id(),
            )
            .map_err(map_learner_error)
    }

    async fn validate_uninstalled_session(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        session: &EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if self
            .cut
            .read()
            .await
            .session_matches_with_exclusion(self.local, target, connection, session, &exclusion)
        {
            Ok(())
        } else {
            Err(PolisRuntimeError::AuthorityDenied)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn request_on_connection(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        session: &EstablishedPolisSession,
        sequence: u64,
        message_kind: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, PolisRuntimeError> {
        self.validate_uninstalled_session(target, connection, session)
            .await?;
        connection
            .request_polis(session, sequence, message_kind, payload)
            .await
            .map_err(|_| PolisRuntimeError::Network)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn begin_request_on_connection(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        session: &EstablishedPolisSession,
        sequence: u64,
        message_kind: &str,
        payload: Vec<u8>,
    ) -> Result<PendingPolisResponse, PolisRuntimeError> {
        self.validate_uninstalled_session(target, connection, session)
            .await?;
        connection
            .begin_polis_request(session, sequence, message_kind, payload)
            .await
            .map_err(|_| PolisRuntimeError::Network)
    }

    pub async fn accept_request_on_connection(
        &self,
        target: NodeId,
        connection: &AuthenticatedConnection,
        session: &EstablishedPolisSession,
    ) -> Result<IncomingPolisRequest, PolisRuntimeError> {
        self.validate_uninstalled_session(target, connection, session)
            .await?;
        connection
            .accept_polis_request(session)
            .await
            .map_err(|_| PolisRuntimeError::Network)
    }

    pub async fn install_route(
        &self,
        target: NodeId,
        connection: Arc<AuthenticatedConnection>,
        session: EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if !self.cut.read().await.session_matches_with_exclusion(
            self.local,
            target,
            &connection,
            &session,
            &exclusion,
        ) {
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

    pub async fn install_learner_route(
        &self,
        target: NodeId,
        connection: Arc<AuthenticatedConnection>,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
        voter_signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let cut = self.cut.read().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if target == 0
            || target != admission.identity().stable_raft_id
            || !exclusion.learner_route_allowed(admission, cut.contains(target))
            || !admission.matches_route_cut(&cut)
            || admission.voter_cut_sha256()
                != learner_route_cut_digest(&cut).map_err(map_learner_error)?
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let voter =
            learner_voter_binding(&cut, self.local, connection.local_certificate_generation())?;
        let mut outbound = EstablishedLearnerSession::new(
            admission,
            admission.voter_cut_sha256(),
            voter.clone(),
            LearnerEndpointRole::Voter,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        let mut responses = EstablishedLearnerSession::new(
            admission,
            admission.voter_cut_sha256(),
            voter,
            LearnerEndpointRole::Voter,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        outbound
            .validate_connection(&connection)
            .map_err(map_learner_error)?;
        responses
            .validate_connection(&connection)
            .map_err(map_learner_error)?;
        drop(cut);
        drop(_transition);
        establish_voter_learner_sessions(
            &connection,
            &mut outbound,
            &mut responses,
            voter_signing_key,
        )
        .await
        .map_err(map_learner_error)?;
        let _transition = self.authority_transition.lock().await;
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        let cut = self.cut.read().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        let (local_node_id, local_guardian_id, _) = cut
            .authority_node_identity(self.local)
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        if target != admission.identity().stable_raft_id
            || !exclusion.learner_route_allowed(admission, cut.contains(target))
            || !admission.matches_route_cut(&cut)
            || !self
                .learner_authority
                .session_is_current(&outbound)
                .map_err(map_learner_error)?
            || !exclusion.ordinary_authority_allowed(&local_node_id, &local_guardian_id)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let (peer_key, peer_instance) = outbound
            .peer_transport_instance()
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        self.learner_authority
            .pin_peer_instance(&mut lease, peer_key, peer_instance)
            .map_err(map_learner_error)?;
        drop(cut);
        let mut routes = self.learner_connections.write().await;
        if routes.contains_key(&target) {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        routes.insert(
            target,
            SecureLearnerRoute {
                connection,
                outbound: Arc::new(tokio::sync::Mutex::new(outbound)),
                responses: Arc::new(tokio::sync::Mutex::new(responses)),
                dispatch_lock: Arc::new(tokio::sync::Mutex::new(())),
                sequence: Arc::new(AtomicU64::new(0)),
            },
        );
        Ok(())
    }

    async fn learner_server_sessions_attested(
        &self,
        connection: &AuthenticatedConnection,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
        custody: &LearnerBootAttestationCustody,
    ) -> Result<(EstablishedLearnerSession, EstablishedLearnerSession), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let cut = self.cut.read().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if !exclusion
            .learner_route_allowed(admission, cut.contains(admission.identity().stable_raft_id))
            || !admission.matches_route_cut(&cut)
            || admission.voter_cut_sha256()
                != learner_route_cut_digest(&cut).map_err(map_learner_error)?
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let voter = learner_voter_binding_for_connection(&cut, admission, connection)?;
        let mut inbound = EstablishedLearnerSession::new(
            admission,
            admission.voter_cut_sha256(),
            voter.clone(),
            LearnerEndpointRole::Learner,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        let mut outbound = EstablishedLearnerSession::new(
            admission,
            admission.voter_cut_sha256(),
            voter,
            LearnerEndpointRole::Learner,
            self.learner_authority.clone(),
            now_unix_seconds,
        )
        .map_err(map_learner_error)?;
        inbound
            .validate_connection(connection)
            .map_err(map_learner_error)?;
        outbound
            .validate_connection(connection)
            .map_err(map_learner_error)?;
        drop(cut);
        drop(_transition);
        establish_learner_voter_sessions(connection, &mut inbound, &mut outbound, custody)
            .await
            .map_err(map_learner_error)?;
        let _transition = self.authority_transition.lock().await;
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        let cut = self.cut.read().await;
        let (local_node_id, local_guardian_id, _) = cut
            .authority_node_identity(self.local)
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if cut.contains(admission.identity().stable_raft_id)
            || !admission.matches_route_cut(&cut)
            || !self
                .learner_authority
                .session_is_current(&inbound)
                .map_err(map_learner_error)?
            || !exclusion.ordinary_authority_allowed(&local_node_id, &local_guardian_id)
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let (peer_key, peer_instance) = inbound
            .peer_transport_instance()
            .ok_or(PolisRuntimeError::AuthorityDenied)?;
        self.learner_authority
            .pin_peer_instance(&mut lease, peer_key, peer_instance)
            .map_err(map_learner_error)?;
        Ok((inbound, outbound))
    }

    #[cfg(not(test))]
    pub async fn learner_server_sessions(
        &self,
        connection: &AuthenticatedConnection,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
        custody: &LearnerBootAttestationCustody,
    ) -> Result<(EstablishedLearnerSession, EstablishedLearnerSession), PolisRuntimeError> {
        self.learner_server_sessions_attested(connection, admission, now_unix_seconds, custody)
            .await
    }

    #[cfg(test)]
    pub async fn learner_server_sessions(
        &self,
        connection: &AuthenticatedConnection,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
        live_learner_boot_generation: u64,
        learner_signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<(EstablishedLearnerSession, EstablishedLearnerSession), PolisRuntimeError> {
        let custody = LearnerBootAttestationCustody::for_test(
            live_learner_boot_generation,
            learner_signing_key,
        );
        self.learner_server_sessions_attested(connection, admission, now_unix_seconds, &custody)
            .await
    }

    pub async fn learner_boot_attestation(
        &self,
        identity: &LocalNodeGuardianIdentity,
        boot: SecureBootGenerationCustody,
        admission: &VerifiedLearnerAdmission,
    ) -> Result<LearnerBootAttestationCustody, PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let cut = self.cut.read().await;
        if cut.contains(admission.identity().stable_raft_id)
            || !admission.matches_route_cut(&cut)
            || !self
                .learner_authority
                .admission_is_current(admission)
                .map_err(map_learner_error)?
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        LearnerBootAttestationCustody::establish(boot, identity, admission.identity())
            .map_err(map_learner_error)
    }

    pub async fn activate_learner_admission(
        &self,
        admission: &VerifiedLearnerAdmission,
        now_unix_seconds: i64,
    ) -> Result<GovernedMembershipAuthorityReceipt, PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        if !admission.is_live_at(now_unix_seconds)
            || !self.admission_matches_trusted_cut(admission).await
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        let snapshot = self
            .learner_authority
            .governed_activate_admission(&mut lease, admission)
            .map_err(map_learner_error)?;
        snapshot
            .membership_receipt_parts()
            .map_err(map_learner_error)?
            .filter(|parts| parts.operation_sha256 == admission.operation_sha256())
            .map(GovernedMembershipAuthorityReceipt::from_parts)
            .ok_or(PolisRuntimeError::AuthorityDenied)
    }

    pub async fn observe_learner_admission_receipt(
        &self,
        operation_sha256: [u8; 32],
    ) -> Result<Option<GovernedMembershipAuthorityReceipt>, PolisRuntimeError> {
        self.learner_authority
            .admission_snapshot()
            .map_err(map_learner_error)?
            .membership_receipt_parts()
            .map_err(map_learner_error)
            .map(|parts| {
                parts
                    .filter(|parts| parts.operation_sha256 == operation_sha256)
                    .map(GovernedMembershipAuthorityReceipt::from_parts)
            })
    }

    pub async fn stage_learner_successor(
        &self,
        successor: &VerifiedLearnerAdmission,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        if !self.admission_matches_trusted_cut(successor).await {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        self.learner_authority
            .governed_stage_successor(&mut lease, successor)
            .map_err(map_learner_error)
    }

    pub async fn flip_learner_successor(
        &self,
        operation_sha256: [u8; 32],
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let routes = self
            .learner_connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut dispatch_guards = Vec::with_capacity(routes.len());
        for route in &routes {
            dispatch_guards.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let transport_owner = self.transport_owner.lock().await;
        let mut authority_transition = transport_owner.write_lease().await;
        self.learner_authority
            .governed_flip_successor(&mut authority_transition, operation_sha256)
            .map_err(map_learner_error)?;
        for route in &routes {
            route.connection.close();
        }
        self.learner_connections.write().await.clear();
        drop(dispatch_guards);
        Ok(())
    }

    pub async fn expire_learner_admission(
        &self,
        now_unix_seconds: i64,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let routes = self
            .learner_connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut dispatch_guards = Vec::with_capacity(routes.len());
        for route in &routes {
            dispatch_guards.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let owner = self.transport_owner.lock().await;
        let mut lease = owner.write_lease().await;
        self.learner_authority
            .governed_expire_admission(&mut lease, now_unix_seconds)
            .map_err(map_learner_error)?;
        for route in &routes {
            route.connection.close();
        }
        self.learner_connections.write().await.clear();
        drop(dispatch_guards);
        Ok(())
    }

    pub async fn activate_pending_exclusion(
        &self,
        result: &PublishedAuthorityResult,
        expected_identity: &LearnerIdentity,
        expected_voter_cut_sha256: [u8; 32],
        expected_target_membership_sha256: [u8; 32],
        now_unix_seconds: i64,
    ) -> Result<GovernedMembershipAuthorityReceipt, PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let routes = self
            .connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let learner_routes = self
            .learner_connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut dispatch_guards = Vec::with_capacity(routes.len());
        for route in &routes {
            dispatch_guards.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let mut learner_dispatch_guards = Vec::with_capacity(learner_routes.len());
        for route in &learner_routes {
            learner_dispatch_guards.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let transport_owner = self.transport_owner.lock().await;
        let mut exclusion = transport_owner.write_lease().await;
        if expected_identity.trust_domain != self.trusted_trust_domain
            || expected_identity.polis_id != self.trusted_polis_id
            || expected_voter_cut_sha256 != *self.trusted_cut_sha256.read().await
            || !self.cut.read().await.exact_removal_target_matches(
                expected_identity.stable_raft_id,
                &expected_identity.trust_domain,
                &expected_identity.polis_id,
                &expected_identity.node_id,
                &expected_identity.guardian_id,
                expected_identity.guardian_control_public_key,
                expected_identity.certificate_generation,
                expected_identity.boot_generation,
                expected_identity.address,
            )
            || !published_result_matches_trusted_cut(
                result,
                &self.trusted_polis_id,
                &self.trusted_trust_domain,
                &self.trusted_authority,
                &self.trusted_node_identities,
            )
        {
            return Err(PolisRuntimeError::AuthorityDenied);
        }
        let snapshot = self
            .learner_authority
            .governed_activate_exclusion(
                &mut exclusion,
                result,
                expected_identity,
                expected_voter_cut_sha256,
                expected_target_membership_sha256,
                now_unix_seconds,
            )
            .map_err(map_learner_error)?;
        exclusion
            .commit_exclusion(
                &expected_identity.node_id,
                &expected_identity.guardian_id,
                snapshot.generation(),
            )
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        let cut = self.cut.read().await;
        let denied = self
            .connections
            .read()
            .await
            .keys()
            .copied()
            .filter(|target| !ordinary_route_allowed(&cut, self.local, *target, &snapshot))
            .collect::<Vec<_>>();
        drop(cut);
        let mut installed = self.connections.write().await;
        for target in denied {
            if let Some(route) = installed.remove(&target) {
                route.connection.read().await.close();
            }
        }
        drop(installed);
        let mut denied_learners = Vec::new();
        for (target, route) in self.learner_connections.read().await.iter() {
            let session = route.outbound.lock().await;
            if !self
                .learner_authority
                .session_is_current(&session)
                .map_err(map_learner_error)?
            {
                denied_learners.push(*target);
            }
        }
        let mut learners = self.learner_connections.write().await;
        for target in denied_learners {
            if let Some(route) = learners.remove(&target) {
                route.connection.close();
            }
        }
        drop(learners);
        drop(dispatch_guards);
        drop(learner_dispatch_guards);
        snapshot
            .membership_receipt_parts()
            .map_err(map_learner_error)?
            .filter(|parts| parts.operation_sha256 == result.result_sha256())
            .map(GovernedMembershipAuthorityReceipt::from_parts)
            .ok_or(PolisRuntimeError::AuthorityDenied)
    }

    pub async fn observe_pending_exclusion_receipt(
        &self,
        operation_sha256: [u8; 32],
    ) -> Result<Option<GovernedMembershipAuthorityReceipt>, PolisRuntimeError> {
        self.learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?
            .membership_receipt_parts()
            .map_err(map_learner_error)
            .map(|parts| {
                parts
                    .filter(|parts| parts.operation_sha256 == operation_sha256)
                    .map(GovernedMembershipAuthorityReceipt::from_parts)
            })
    }

    async fn admission_matches_trusted_cut(&self, admission: &VerifiedLearnerAdmission) -> bool {
        let cut = self.cut.read().await;
        admission_matches_cut(admission, &cut)
            && admission.voter_cut_sha256() == *self.trusted_cut_sha256.read().await
    }

    pub async fn replace_route(
        &self,
        target: NodeId,
        connection: Arc<AuthenticatedConnection>,
        session: EstablishedPolisSession,
    ) -> Result<(), PolisRuntimeError> {
        let _transition = self.authority_transition.lock().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
        if !self.cut.read().await.session_matches_with_exclusion(
            self.local,
            target,
            &connection,
            &session,
            &exclusion,
        ) {
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
        let _transition = self.authority_transition.lock().await;
        let ordinary_routes = self
            .connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let learner_routes = self
            .learner_connections
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut ordinary_dispatch = Vec::with_capacity(ordinary_routes.len());
        for route in &ordinary_routes {
            ordinary_dispatch.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let mut learner_dispatch = Vec::with_capacity(learner_routes.len());
        for route in &learner_routes {
            learner_dispatch.push(route.dispatch_lock.clone().lock_owned().await);
        }
        let owner = self.transport_owner.lock().await;
        let mut authority = owner.write_lease().await;
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
        let next_digest = learner_route_cut_digest(&candidate).map_err(map_learner_error)?;
        let changed = next_digest != *self.trusted_cut_sha256.read().await;
        *current = candidate;
        authority
            .replace_voter_cut(next_digest)
            .map_err(|_| PolisRuntimeError::AuthorityDenied)?;
        *self.trusted_cut_sha256.write().await = next_digest;
        drop(current);
        if changed {
            for route in &ordinary_routes {
                route.connection.read().await.close();
            }
            for route in &learner_routes {
                route.connection.close();
            }
            self.connections.write().await.clear();
            self.learner_connections.write().await.clear();
        }
        drop(ordinary_dispatch);
        drop(learner_dispatch);
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
        let route = self.connections.read().await.get(&target).cloned();
        let Some(route) = route else {
            return self
                .request_learner_bytes(target, message_kind, payload)
                .await;
        };
        let _dispatch = route.dispatch_lock.lock().await;
        {
            let cut = self.cut.read().await;
            let exclusion = self
                .learner_authority
                .exclusion_snapshot()
                .map_err(map_learner_error)?;
            let connection = route.connection.read().await;
            let session = route.session.read().await;
            if !cut.session_matches_with_exclusion(
                self.local,
                target,
                &connection,
                &session,
                &exclusion,
            ) {
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

    async fn request_learner_bytes(
        &self,
        target: NodeId,
        message_kind: &'static str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, PolisRuntimeError> {
        let kind = match message_kind {
            "append_entries" => LearnerRpcKind::AppendEntries,
            "install_snapshot" => LearnerRpcKind::InstallSnapshot,
            _ => return Err(PolisRuntimeError::AuthorityDenied),
        };
        let route = self
            .learner_connections
            .read()
            .await
            .get(&target)
            .cloned()
            .ok_or(PolisRuntimeError::Network)?;
        let _dispatch = route.dispatch_lock.lock().await;
        let sequence = route
            .sequence
            .fetch_add(1, Ordering::SeqCst)
            .checked_add(1)
            .ok_or(PolisRuntimeError::Replay)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| PolisRuntimeError::StateRegression)?
            .as_secs();
        let now = i64::try_from(now).map_err(|_| PolisRuntimeError::StateRegression)?;
        let pending = {
            let mut outbound = route.outbound.lock().await;
            match kind {
                LearnerRpcKind::AppendEntries => {
                    outbound
                        .send_append_entries(&route.connection, sequence, payload, now)
                        .await
                }
                LearnerRpcKind::InstallSnapshot => {
                    outbound
                        .send_install_snapshot(&route.connection, sequence, payload, now)
                        .await
                }
            }
            .map_err(map_learner_error)?
        };
        let response = route
            .responses
            .lock()
            .await
            .receive_response(&route.connection, pending, now)
            .await
            .map_err(map_learner_error);
        response
    }

    async fn validate_ready(&self) -> Result<(), PolisRuntimeError> {
        let cut = self.cut.read().await;
        let exclusion = self
            .learner_authority
            .exclusion_snapshot()
            .map_err(map_learner_error)?;
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
            if !cut.session_matches_with_exclusion(
                self.local,
                *peer,
                &connection,
                &session,
                &exclusion,
            ) {
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

fn learner_voter_binding(
    cut: &VerifiedPolisRouteCut,
    voter: NodeId,
    certificate_generation: u64,
) -> Result<LearnerVoterBinding, PolisRuntimeError> {
    let (node_id, guardian_id, boot_generation) = cut
        .authority_node_identity(voter)
        .ok_or(PolisRuntimeError::AuthorityDenied)?;
    if certificate_generation == 0 {
        return Err(PolisRuntimeError::AuthorityDenied);
    }
    let control_public_key = cut
        .authority_membership()
        .voters
        .get(guardian_id.as_bytes())
        .map(|authority| authority.control_public_key)
        .ok_or(PolisRuntimeError::AuthorityDenied)?;
    Ok(LearnerVoterBinding {
        stable_raft_id: voter,
        node_id,
        guardian_id,
        certificate_generation,
        boot_generation,
        control_public_key,
    })
}

fn learner_voter_binding_for_connection(
    cut: &VerifiedPolisRouteCut,
    admission: &VerifiedLearnerAdmission,
    connection: &AuthenticatedConnection,
) -> Result<LearnerVoterBinding, PolisRuntimeError> {
    let routes = cut.routes();
    let mut matches = routes.keys().filter_map(|raft_id| {
        let voter =
            learner_voter_binding(cut, *raft_id, connection.peer_certificate_generation()).ok()?;
        connection
            .matches_learner_route(
                LearnerEndpointRole::Learner,
                cut.trust_domain(),
                &voter.node_id,
                &voter.guardian_id,
                voter.certificate_generation,
                &admission.identity().node_id,
                &admission.identity().guardian_id,
                admission.identity().certificate_generation,
                admission.identity().address,
            )
            .then_some(voter)
    });
    let voter = matches.next().ok_or(PolisRuntimeError::AuthorityDenied)?;
    if matches.next().is_some() {
        return Err(PolisRuntimeError::AuthorityDenied);
    }
    Ok(voter)
}

fn admission_matches_cut(
    admission: &VerifiedLearnerAdmission,
    cut: &VerifiedPolisRouteCut,
) -> bool {
    let node_identities = cut
        .routes()
        .keys()
        .filter_map(|raft_id| {
            cut.authority_node_identity(*raft_id)
                .map(|(node_id, guardian_id, boot_generation)| {
                    (guardian_id.into_bytes(), (node_id, boot_generation))
                })
        })
        .collect::<BTreeMap<_, _>>();
    admission.identity().trust_domain == cut.trust_domain()
        && admission.identity().polis_id == cut.polis_id()
        && admission.matches_route_cut(cut)
        && admission.publication_identity_matches(
            cut.polis_id(),
            cut.trust_domain(),
            cut.authority_membership(),
            &node_identities,
        )
}

fn ordinary_route_allowed(
    cut: &VerifiedPolisRouteCut,
    local: NodeId,
    peer: NodeId,
    exclusion: &PendingExclusionSnapshot,
) -> bool {
    [local, peer].into_iter().all(|node| {
        cut.authority_node_identity(node)
            .is_some_and(|(node_id, guardian_id, _)| {
                exclusion.ordinary_authority_allowed(&node_id, &guardian_id)
            })
    })
}

fn published_result_matches_trusted_cut(
    result: &PublishedAuthorityResult,
    polis_id: &str,
    trust_domain: &str,
    authority: &AuthorityMembership,
    node_identities: &BTreeMap<Vec<u8>, (String, u64)>,
) -> bool {
    let identity = result.authority_identity_for_sealed_consumer();
    authority.trust_domain_id.as_slice() == trust_domain.as_bytes()
        && identity.polis_id == polis_id
        && identity.trust_domain == trust_domain
        && authority
            .voters
            .get(identity.guardian_id.as_bytes())
            .is_some_and(|voter| {
                !voter.revoked
                    && node_identities.get(identity.guardian_id.as_bytes())
                        == Some(&(identity.node_id.clone(), identity.boot_generation))
            })
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

fn map_learner_error(error: LearnerTransportError) -> PolisRuntimeError {
    match error {
        LearnerTransportError::FrameTooLarge => PolisRuntimeError::FrameTooLarge,
        LearnerTransportError::Replay => PolisRuntimeError::Replay,
        LearnerTransportError::Storage => PolisRuntimeError::Storage,
        LearnerTransportError::InvalidBinding | LearnerTransportError::ArtifactMismatch => {
            PolisRuntimeError::InvalidConfiguration
        }
        LearnerTransportError::AuthorityDenied
        | LearnerTransportError::Expired
        | LearnerTransportError::CapacityExceeded => PolisRuntimeError::AuthorityDenied,
    }
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

struct SecureBootGenerationAuthorityInner {
    durable: CheckpointedJson<BootGenerationState>,
    current: StdMutex<DurableEnvelope<BootGenerationState>>,
    node_id: NodeId,
}

#[derive(Clone)]
pub struct SecureBootGenerationAuthority {
    inner: Arc<SecureBootGenerationAuthorityInner>,
}

pub struct SecureBootGenerationCustody {
    authority: SecureBootGenerationAuthority,
    generation: u64,
}

impl SecureBootGenerationAuthority {
    pub fn open(
        state_root: &Path,
        node_id: NodeId,
        checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
    ) -> Result<Self, PolisRuntimeError> {
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
        Ok(Self {
            inner: Arc::new(SecureBootGenerationAuthorityInner {
                durable,
                current: StdMutex::new(current),
                node_id,
            }),
        })
    }

    pub fn advance(&self) -> Result<SecureBootGenerationCustody, PolisRuntimeError> {
        let mut current = self
            .inner
            .current
            .lock()
            .map_err(|_| PolisRuntimeError::Storage)?;
        let generation = current
            .payload()
            .generation
            .checked_add(1)
            .ok_or(PolisRuntimeError::StateRegression)?;
        *current = self
            .inner
            .durable
            .commit(&current, BootGenerationState { generation })?;
        Ok(SecureBootGenerationCustody {
            authority: self.clone(),
            generation,
        })
    }
}

impl SecureBootGenerationCustody {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn node_id(&self) -> NodeId {
        self.authority.inner.node_id
    }

    pub(crate) fn require_current(&self) -> Result<(), PolisRuntimeError> {
        self.with_current(|| ())
    }

    pub(crate) fn with_current<T>(
        &self,
        action: impl FnOnce() -> T,
    ) -> Result<T, PolisRuntimeError> {
        let current = self
            .authority
            .inner
            .current
            .lock()
            .map_err(|_| PolisRuntimeError::Storage)?;
        if current.payload().generation == self.generation {
            Ok(action())
        } else {
            Err(PolisRuntimeError::AuthorityDenied)
        }
    }
}

pub fn advance_secure_boot_generation(
    state_root: &Path,
    node_id: NodeId,
    checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
) -> Result<u64, PolisRuntimeError> {
    Ok(
        SecureBootGenerationAuthority::open(state_root, node_id, checkpoint_authority)?
            .advance()?
            .generation(),
    )
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
    let state_machine = PolisStateMachineStore::open_with_trusted_authority(
        state_root,
        node_id,
        checkpoint_authority,
        network.trusted_authority_bootstrap(),
    )?;
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

pub async fn serve_authorized_learner_connection(
    raft: PolisRaft,
    connection: Arc<AuthenticatedConnection>,
    mut inbound: EstablishedLearnerSession,
    mut outbound: EstablishedLearnerSession,
    cancellation: CancellationToken,
) -> Result<(), PolisRuntimeError> {
    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| PolisRuntimeError::StateRegression)?
            .as_secs();
        let now = i64::try_from(now).map_err(|_| PolisRuntimeError::StateRegression)?;
        let request = tokio::select! {
            _ = cancellation.cancelled() => return Ok(()),
            result = inbound.receive_replication(&connection, now) => result.map_err(map_learner_error)?,
        };
        #[cfg(test)]
        inbound
            .pause_after_revalidation_for_test("learner_raft_effect")
            .await;
        let response = match request.message_kind() {
            "append_entries" => {
                let value: AppendEntriesRequest<PolisTypeConfig> =
                    decode_bounded_json(request.payload())?;
                encode_bounded_json(&raft.append_entries(value).await)?
            }
            "install_snapshot" => {
                let value: InstallSnapshotRequest<PolisTypeConfig> =
                    decode_bounded_json(request.payload())?;
                encode_bounded_json(&raft.install_snapshot(value).await)?
            }
            _ => return Err(PolisRuntimeError::AuthorityDenied),
        };
        outbound
            .send_response(&connection, request, response, now)
            .await
            .map_err(map_learner_error)?;
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

#[cfg(test)]
mod authority_consensus_tests {
    use super::*;
    use crate::distributed::{
        authority_protocol::{
            endorse_committed_authority_prepare_with_exclusion, AuthorityEligibilityExclusion,
            AuthorityFinalizeProposal, AuthorityOperationKind, AuthorityPrepareProposal,
            CanonicalAuthorityTime, CommittedAuthorityArtifact,
        },
        identity::LocalNodeGuardianIdentity,
        lease::{ControlCertificatePurpose, VoterAuthority},
        membership::{
            CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipState,
        },
    };
    use std::sync::{
        atomic::{AtomicBool, Ordering as AtomicOrdering},
        Mutex, RwLock as StdRwLock,
    };
    use std::time::Duration;

    struct NoPendingExclusion;

    impl AuthorityEligibilityExclusion for NoPendingExclusion {
        fn ordinary_authority_allowed(&self, _node_id: &str, _guardian_id: &[u8]) -> bool {
            true
        }
    }

    #[derive(Default)]
    struct MemoryAuthority {
        values: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
        fail_after_cas: AtomicBool,
    }

    impl MemoryAuthority {
        fn arm_after_cas(&self) {
            self.fail_after_cas.store(true, AtomicOrdering::SeqCst);
        }
    }

    impl ConsensusCheckpointAuthority for MemoryAuthority {
        fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
            Ok(self.values.lock().unwrap().get(object).cloned())
        }

        fn compare_and_swap(
            &self,
            expected: Option<&ConsensusCheckpoint>,
            candidate: &ConsensusCheckpoint,
        ) -> Result<(), PolisRuntimeError> {
            let mut current = self.values.lock().unwrap();
            if current.get(&candidate.object) != expected {
                return Err(PolisRuntimeError::StateRegression);
            }
            current.insert(candidate.object.clone(), candidate.clone());
            if self.fail_after_cas.swap(false, AtomicOrdering::SeqCst) {
                return Err(PolisRuntimeError::Storage);
            }
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct MemoryNetwork {
        peers: Arc<StdRwLock<BTreeMap<NodeId, PolisRaft>>>,
    }

    struct MemoryConnection {
        target: NodeId,
        peers: Arc<StdRwLock<BTreeMap<NodeId, PolisRaft>>>,
    }

    impl RaftNetworkFactory<PolisTypeConfig> for MemoryNetwork {
        type Network = MemoryConnection;

        async fn new_client(&mut self, target: NodeId, _node: &BasicNode) -> Self::Network {
            MemoryConnection {
                target,
                peers: Arc::clone(&self.peers),
            }
        }
    }

    impl MemoryConnection {
        fn peer(&self) -> Result<PolisRaft, RPCError<NodeId, BasicNode, RaftError<NodeId>>> {
            self.peers
                .read()
                .unwrap()
                .get(&self.target)
                .cloned()
                .ok_or_else(|| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))
        }
    }

    impl RaftNetwork<PolisTypeConfig> for MemoryConnection {
        async fn append_entries(
            &mut self,
            request: AppendEntriesRequest<PolisTypeConfig>,
            _option: RPCOption,
        ) -> Result<AppendEntriesResponse<NodeId>, RPCError<NodeId, BasicNode, RaftError<NodeId>>>
        {
            self.peer()?
                .append_entries(request)
                .await
                .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
        }

        async fn install_snapshot(
            &mut self,
            request: InstallSnapshotRequest<PolisTypeConfig>,
            _option: RPCOption,
        ) -> Result<
            InstallSnapshotResponse<NodeId>,
            RPCError<NodeId, BasicNode, RaftError<NodeId, InstallSnapshotError>>,
        > {
            let peer = self
                .peers
                .read()
                .unwrap()
                .get(&self.target)
                .cloned()
                .ok_or_else(|| RPCError::Network(NetworkError::new(&PolisRuntimeError::Network)))?;
            peer.install_snapshot(request)
                .await
                .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
        }

        async fn vote(
            &mut self,
            request: VoteRequest<NodeId>,
            _option: RPCOption,
        ) -> Result<VoteResponse<NodeId>, RPCError<NodeId, BasicNode, RaftError<NodeId>>> {
            self.peer()?
                .vote(request)
                .await
                .map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
        }
    }

    fn authority_fixture(
        seed: u64,
    ) -> (
        MembershipState,
        AuthorityMembership,
        BTreeMap<Vec<u8>, u64>,
        Vec<LocalNodeGuardianIdentity>,
    ) {
        let identities = (0..3)
            .map(|_| LocalNodeGuardianIdentity::generate("polis.authority.test", seed).unwrap())
            .collect::<Vec<_>>();
        let mut membership =
            MembershipState::new(MembershipPolicy::new("polis.authority.test", 8, 16).unwrap());
        let mut index = 0_u64;
        for identity in &identities {
            let public = identity.public_identity();
            index += 1;
            membership
                .apply(&CommittedMembershipEvent::new(
                    "polis.authority.test",
                    [index as u8; 32],
                    index,
                    index,
                    MembershipOperation::Join {
                        member: Member {
                            node_id: public.node_id.clone(),
                            guardian_id: public.guardian_id.clone(),
                            identity_generation: public.identity_generation,
                            guardian_control_public_key: public.guardian_control_public_key,
                            role: MemberRole::NonVoting,
                        },
                    },
                ))
                .unwrap();
        }
        for identity in &identities {
            index += 1;
            membership
                .apply(&CommittedMembershipEvent::new(
                    "polis.authority.test",
                    [index as u8; 32],
                    index,
                    index,
                    MembershipOperation::Promote {
                        node_id: identity.public_identity().node_id.clone(),
                    },
                ))
                .unwrap();
        }
        let guardians = identities
            .iter()
            .map(|identity| identity.public_identity().guardian_id.as_bytes().to_vec())
            .collect::<BTreeSet<_>>();
        let voters = identities
            .iter()
            .map(|identity| VoterAuthority {
                guardian_id: identity.public_identity().guardian_id.as_bytes().to_vec(),
                trust_domain_id: b"polis.authority.test".to_vec(),
                certificate_generation: seed,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: 1_799_999_900,
                not_after_unix_seconds: 1_800_000_100,
                revoked: false,
                control_public_key: identity.public_identity().guardian_control_public_key,
            })
            .collect();
        let authority = AuthorityMembership::new(
            b"polis.authority.test".to_vec(),
            seed,
            membership.committed_log_index(),
            vec![guardians],
            voters,
        )
        .unwrap();
        let boot_generations = authority
            .voters
            .keys()
            .map(|guardian| (guardian.clone(), seed))
            .collect::<BTreeMap<_, _>>();
        (membership, authority, boot_generations, identities)
    }

    fn publication_identity(
        node: NodeId,
        membership: &MembershipState,
        authority: &AuthorityMembership,
        boot_generation: u64,
    ) -> AuthorityNodeIdentity {
        let guardian = authority
            .raft_ids
            .iter()
            .find_map(|(guardian, raft_id)| (*raft_id == node).then_some(guardian))
            .unwrap();
        let guardian_id = String::from_utf8(guardian.clone()).unwrap();
        let node_id = membership
            .members()
            .find(|member| member.guardian_id == guardian_id)
            .unwrap()
            .node_id
            .clone();
        AuthorityNodeIdentity {
            trust_domain: "polis.authority.test".into(),
            polis_id: "polis-a".into(),
            node_id,
            guardian_id,
            boot_generation,
        }
    }

    async fn write_on_leader(
        nodes: &BTreeMap<NodeId, PolisRaft>,
        command: PolisCommand,
    ) -> (NodeId, openraft::raft::ClientWriteResponse<PolisTypeConfig>) {
        tokio::time::timeout(Duration::from_secs(15), async {
            loop {
                for (node, raft) in nodes {
                    if let Ok(response) = raft.client_write(command.clone()).await {
                        return (*node, response);
                    }
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .expect("three-voter cluster elected a writable leader")
    }

    #[derive(Clone, Copy)]
    enum SnapshotCase {
        Valid,
        CurrentPolis,
        CurrentEpoch,
        CurrentMembership,
        CurrentBoot,
        PreparedPolis,
        PreparedEpoch,
        PreparedMembership,
        PreparedBoot,
        LaterPrepared,
        LegacyOwner,
        LegacyShepherd,
        LegacyObservatory,
        LegacyFence,
        LegacyDemotion,
        MissingProposal,
        MissingEndorsements,
        WrongOperation,
        InsufficientQuorum,
        DuplicateQuorum,
        BadSignature,
        StaleCertificate,
        WrongBoot,
        InvalidTime,
        WrongPrepareIndex,
        WrongFinalizeIndex,
        CustodyOmitted,
        CustodyReencoded,
        CustodyInjected,
        CustodySubstituted,
        CustodyByteDigestMismatch,
        EvidenceOmitted,
        EvidenceReencoded,
        EvidenceInjected,
        EvidenceSubstituted,
        EvidenceByteDigestMismatch,
    }

    fn snapshot_application_fixture() -> (
        PolisApplicationState,
        MembershipState,
        AuthorityMembership,
        BTreeMap<Vec<u8>, u64>,
        Vec<LocalNodeGuardianIdentity>,
    ) {
        let (membership, authority, boots, signers) = authority_fixture(17);
        let mut application = PolisApplicationState::default();
        application
            .install_trusted_authority("polis-a", membership.epoch(), authority.clone())
            .unwrap();
        for (offset, operation_id) in ["snapshot-op-a", "snapshot-op-b"].iter().enumerate() {
            let prepare_index = 20 + (offset as u64 * 2);
            let finalize_index = prepare_index + 1;
            let prepare = AuthorityPrepareProposal::new(
                "polis-a",
                &membership,
                &authority,
                AuthorityOperationKind::Membership,
                [17; 32],
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_000,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_050,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                *operation_id,
                CommittedAuthorityArtifact::new(
                    AuthorityOperationKind::Membership,
                    format!("snapshot-artifact-{offset}").into_bytes(),
                )
                .unwrap(),
            )
            .unwrap();
            assert!(application
                .apply_committed(
                    prepare_index,
                    &PolisCommand::PrepareAuthority {
                        proposal: Box::new(prepare),
                        boot_generations: canonical_boot_generations(&boots),
                    },
                    None,
                    Some(&boots),
                )
                .unwrap());
            let intent = application
                .prepared_authority_intent(operation_id)
                .unwrap()
                .clone();
            let finalization_time = CanonicalAuthorityTime {
                unix_seconds: 1_800_000_010 + offset as i64,
                nanos: 0,
                uncertainty_millis: 1,
            };
            let endorsements = signers[..2]
                .iter()
                .map(|signer| {
                    endorse_committed_authority_prepare_with_exclusion(
                        signer,
                        authority.voter_set_generation,
                        authority.voter_set_generation,
                        membership.committed_log_index(),
                        &boots,
                        &intent,
                        &finalization_time,
                        &membership,
                        &authority,
                        &NoPendingExclusion,
                    )
                    .unwrap()
                })
                .collect();
            let proposal =
                AuthorityFinalizeProposal::new(&intent, finalization_time, endorsements).unwrap();
            assert!(!application
                .apply_committed(
                    finalize_index,
                    &PolisCommand::FinalizeAuthority { proposal },
                    None,
                    Some(&boots),
                )
                .unwrap());
        }
        (application, membership, authority, boots, signers)
    }

    async fn run_snapshot_case(case: SnapshotCase) {
        let (mut application, membership, authority, boots, _signers) =
            snapshot_application_fixture();
        let first = "snapshot-op-a";
        let second = "snapshot-op-b";
        match case {
            SnapshotCase::CurrentPolis => application
                .current_authority
                .as_mut()
                .unwrap()
                .polis_id
                .push('x'),
            SnapshotCase::CurrentEpoch => {
                application
                    .current_authority
                    .as_mut()
                    .unwrap()
                    .membership_epoch += 1
            }
            SnapshotCase::CurrentMembership => {
                application
                    .current_authority
                    .as_mut()
                    .unwrap()
                    .voter_set_generation += 1
            }
            SnapshotCase::CurrentBoot => {}
            SnapshotCase::PreparedPolis => application
                .prepared_authority
                .get_mut(first)
                .unwrap()
                .authority
                .polis_id
                .push('x'),
            SnapshotCase::PreparedEpoch => {
                application
                    .prepared_authority
                    .get_mut(first)
                    .unwrap()
                    .authority
                    .membership_epoch += 1
            }
            SnapshotCase::PreparedMembership => {
                application
                    .prepared_authority
                    .get_mut(first)
                    .unwrap()
                    .authority
                    .voter_set_generation += 1
            }
            SnapshotCase::PreparedBoot => {
                application
                    .prepared_authority
                    .get_mut(first)
                    .unwrap()
                    .boot_generations[0]
                    .generation += 1
            }
            SnapshotCase::LaterPrepared => application
                .prepared_authority
                .get_mut(second)
                .unwrap()
                .authority
                .polis_id
                .push('x'),
            SnapshotCase::LegacyOwner => application.active_owner = Some("legacy".into()),
            SnapshotCase::LegacyShepherd => application.active_shepherd = Some("legacy".into()),
            SnapshotCase::LegacyObservatory => {
                application.observatory_owner = Some("legacy".into());
                application.observatory_expires_unix_millis = Some(1);
            }
            SnapshotCase::LegacyFence => {
                application.fenced_voters.insert("legacy".into(), 1);
            }
            SnapshotCase::LegacyDemotion => {
                application.demoted_voters.insert("legacy".into(), 1);
            }
            SnapshotCase::MissingEndorsements => application
                .finalized_authority
                .get_mut(first)
                .unwrap()
                .proposal
                .endorsements
                .clear(),
            SnapshotCase::WrongOperation | SnapshotCase::EvidenceSubstituted => application
                .finalized_authority
                .get_mut(first)
                .unwrap()
                .proposal
                .operation_id
                .push('x'),
            SnapshotCase::InsufficientQuorum => application
                .finalized_authority
                .get_mut(first)
                .unwrap()
                .proposal
                .endorsements
                .truncate(1),
            SnapshotCase::DuplicateQuorum => {
                let endorsement =
                    application.finalized_authority[first].proposal.endorsements[0].clone();
                application
                    .finalized_authority
                    .get_mut(first)
                    .unwrap()
                    .proposal
                    .endorsements
                    .push(endorsement);
            }
            SnapshotCase::InvalidTime => {
                application
                    .finalized_authority
                    .get_mut(first)
                    .unwrap()
                    .proposal
                    .finalization_time
                    .unix_seconds += 100
            }
            SnapshotCase::WrongPrepareIndex => {
                application
                    .prepared_authority
                    .get_mut(first)
                    .unwrap()
                    .intent
                    .prepare_log_index += 1
            }
            SnapshotCase::WrongFinalizeIndex => {
                application
                    .finalized_authority
                    .get_mut(first)
                    .unwrap()
                    .committed_log_index = application.prepared_authority[first]
                    .intent
                    .prepare_log_index
            }
            SnapshotCase::Valid
            | SnapshotCase::MissingProposal
            | SnapshotCase::CustodyOmitted
            | SnapshotCase::CustodyReencoded
            | SnapshotCase::CustodyInjected
            | SnapshotCase::CustodySubstituted
            | SnapshotCase::CustodyByteDigestMismatch
            | SnapshotCase::EvidenceOmitted
            | SnapshotCase::EvidenceReencoded
            | SnapshotCase::EvidenceInjected
            | SnapshotCase::EvidenceByteDigestMismatch => {}
            SnapshotCase::BadSignature
            | SnapshotCase::StaleCertificate
            | SnapshotCase::WrongBoot => {}
        }
        let current = PersistedStateMachine {
            last_applied_log: Some(LogId::new(openraft::CommittedLeaderId::new(1, 1), 23)),
            last_membership: StoredMembership::default(),
            membership_history: Vec::new(),
            application,
        };
        let mut value = serde_json::to_value(&current).unwrap();
        match case {
            SnapshotCase::CurrentBoot => {
                value["application"]["current_authority"]["boot_generations"] =
                    serde_json::to_value(canonical_boot_generations(&boots)).unwrap();
            }
            SnapshotCase::MissingProposal | SnapshotCase::EvidenceOmitted => {
                value["application"]["finalized_authority"][first]
                    .as_object_mut()
                    .unwrap()
                    .remove("proposal");
            }
            SnapshotCase::CustodyOmitted => {
                value["application"]
                    .as_object_mut()
                    .unwrap()
                    .remove("current_authority");
            }
            SnapshotCase::CustodyInjected => {
                value["application"]["current_authority"]["injected"] = serde_json::json!(true);
            }
            SnapshotCase::CustodySubstituted => {
                value["application"]["current_authority"]["polis_id"] =
                    serde_json::json!("polis-x");
            }
            SnapshotCase::EvidenceInjected => {
                value["application"]["finalized_authority"][first]["proposal"]["injected"] =
                    serde_json::json!(true);
            }
            SnapshotCase::BadSignature => {
                value["application"]["finalized_authority"][first]["proposal"]["endorsements"][0]
                    ["signature"][0] = serde_json::json!(255);
            }
            SnapshotCase::StaleCertificate => {
                value["application"]["finalized_authority"][first]["proposal"]["endorsements"][0]
                    ["certificate_generation"] = serde_json::json!(999);
            }
            SnapshotCase::WrongBoot => {
                value["application"]["finalized_authority"][first]["proposal"]["endorsements"][0]
                    ["boot_generation"] = serde_json::json!(999);
            }
            _ => {}
        }
        let mut data = if matches!(
            case,
            SnapshotCase::CustodyReencoded | SnapshotCase::EvidenceReencoded
        ) {
            serde_json::to_vec_pretty(&value).unwrap()
        } else {
            serde_jcs::to_vec(&value).unwrap()
        };
        if matches!(
            case,
            SnapshotCase::CustodyByteDigestMismatch | SnapshotCase::EvidenceByteDigestMismatch
        ) {
            data.push(b' ');
        }
        let decoded: PersistedStateMachine = serde_json::from_value(value).unwrap_or_default();
        let meta = SnapshotMeta {
            last_log_id: decoded.last_applied_log,
            last_membership: decoded.last_membership.clone(),
            snapshot_id: canonical_snapshot_id(&decoded, &data),
        };
        let root = tempfile::Builder::new()
            .prefix("adl-authority-snapshot-")
            .tempdir_in(std::env::current_dir().unwrap())
            .unwrap();
        let checkpoint = Arc::new(MemoryAuthority::default());
        let mut store = PolisStateMachineStore::open_with_trusted_authority(
            root.path(),
            1,
            checkpoint.clone(),
            TrustedAuthorityBootstrap {
                polis_id: "polis-a".into(),
                membership_epoch: membership.epoch(),
                authority: authority.clone(),
                boot_generations: boots.clone(),
                publication_identity: publication_identity(1, &membership, &authority, 17),
            },
        )
        .unwrap();
        let result = store
            .install_snapshot(&meta, Box::new(Cursor::new(data)))
            .await;
        if matches!(case, SnapshotCase::Valid) {
            result.unwrap();
            drop(store);
            let restarted_boots: BTreeMap<Vec<u8>, u64> = boots
                .iter()
                .map(|(guardian, generation)| (guardian.clone(), generation + 1))
                .collect();
            let mut reopened = PolisStateMachineStore::open_with_trusted_authority(
                root.path(),
                1,
                checkpoint.clone(),
                TrustedAuthorityBootstrap {
                    polis_id: "polis-a".into(),
                    membership_epoch: membership.epoch(),
                    authority: authority.clone(),
                    boot_generations: restarted_boots.clone(),
                    publication_identity: publication_identity(1, &membership, &authority, 18),
                },
            )
            .unwrap();
            let immediate_snapshot = reopened.build_snapshot().await.unwrap();
            let immediate_root = root.path().join("immediate-current-peer");
            std::fs::create_dir(&immediate_root).unwrap();
            let mut immediate_peer = PolisStateMachineStore::open_with_trusted_authority(
                &immediate_root,
                2,
                Arc::new(MemoryAuthority::default()),
                TrustedAuthorityBootstrap {
                    polis_id: "polis-a".into(),
                    membership_epoch: membership.epoch(),
                    authority: authority.clone(),
                    boot_generations: restarted_boots.clone(),
                    publication_identity: publication_identity(2, &membership, &authority, 18),
                },
            )
            .unwrap();
            immediate_peer
                .install_snapshot(
                    &immediate_snapshot.meta,
                    Box::new(Cursor::new(immediate_snapshot.snapshot.get_ref().clone())),
                )
                .await
                .unwrap();
            let proposal = AuthorityPrepareProposal::new(
                "polis-a",
                &membership,
                &authority,
                AuthorityOperationKind::Membership,
                [18; 32],
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_020,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_050,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                "snapshot-op-after-restart",
                CommittedAuthorityArtifact::new(
                    AuthorityOperationKind::Membership,
                    b"snapshot-artifact-after-restart".to_vec(),
                )
                .unwrap(),
            )
            .unwrap();
            let canonical_cut = canonical_boot_generations(&restarted_boots);
            let wire_command = PolisCommand::PrepareAuthority {
                proposal: Box::new(proposal.clone()),
                boot_generations: canonical_cut.clone(),
            };
            let wire_bytes = serde_jcs::to_vec(&wire_command).unwrap();
            assert_eq!(
                serde_json::from_slice::<PolisCommand>(&wire_bytes).unwrap(),
                wire_command
            );
            let mut duplicate_cut = canonical_cut.clone();
            duplicate_cut.push(canonical_cut[0].clone());
            assert!(decode_boot_generations(&duplicate_cut).is_err());
            let mut reordered_cut = canonical_cut.clone();
            reordered_cut.swap(0, 1);
            assert!(decode_boot_generations(&reordered_cut).is_err());
            let mut zero_cut = canonical_cut.clone();
            zero_cut[0].generation = 0;
            assert!(decode_boot_generations(&zero_cut).is_err());
            assert!(decode_bounded_json::<PolisCommand>(
                &serde_json::to_vec_pretty(&wire_command).unwrap()
            )
            .is_err());
            for invalid_cut in [duplicate_cut, reordered_cut, zero_cut] {
                let before = reopened.application_state().await;
                assert!(reopened
                    .apply([Entry {
                        log_id: LogId::new(openraft::CommittedLeaderId::new(1, 1), 24),
                        payload: EntryPayload::Normal(PolisCommand::PrepareAuthority {
                            proposal: Box::new(proposal.clone()),
                            boot_generations: invalid_cut,
                        }),
                    }])
                    .await
                    .is_err());
                assert_eq!(reopened.application_state().await, before);
            }
            let responses = reopened
                .apply([Entry {
                    log_id: LogId::new(openraft::CommittedLeaderId::new(1, 1), 24),
                    payload: EntryPayload::Normal(wire_command),
                }])
                .await
                .unwrap();
            assert!(responses[0].accepted);
            let restarted_application = reopened.application_state().await;
            assert_eq!(
                restarted_application.prepared_authority["snapshot-op-after-restart"]
                    .boot_generations,
                canonical_boot_generations(&restarted_boots)
            );
            assert_eq!(
                restarted_application.prepared_authority["snapshot-op-a"].boot_generations,
                canonical_boot_generations(&boots)
            );
            let built = reopened.build_snapshot().await.unwrap();
            let built_meta = built.meta.clone();
            let built_bytes = built.snapshot.get_ref().clone();

            let stale_root = root.path().join("stale-peer");
            std::fs::create_dir(&stale_root).unwrap();
            let mut stale_peer = PolisStateMachineStore::open_with_trusted_authority(
                &stale_root,
                2,
                Arc::new(MemoryAuthority::default()),
                TrustedAuthorityBootstrap {
                    polis_id: "polis-a".into(),
                    membership_epoch: membership.epoch(),
                    authority: authority.clone(),
                    boot_generations: boots.clone(),
                    publication_identity: publication_identity(2, &membership, &authority, 17),
                },
            )
            .unwrap();
            let stale_before = stale_peer.application_state().await;
            let stale_cut_prepare = AuthorityPrepareProposal::new(
                "polis-a",
                &membership,
                &authority,
                AuthorityOperationKind::Membership,
                [19; 32],
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_020,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_050,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                "stale-cut-rejected",
                CommittedAuthorityArtifact::new(
                    AuthorityOperationKind::Membership,
                    b"stale-cut-rejected".to_vec(),
                )
                .unwrap(),
            )
            .unwrap();
            assert!(stale_peer
                .apply([Entry {
                    log_id: LogId::new(openraft::CommittedLeaderId::new(1, 1), 24),
                    payload: EntryPayload::Normal(PolisCommand::PrepareAuthority {
                        proposal: Box::new(stale_cut_prepare),
                        boot_generations: canonical_boot_generations(&restarted_boots),
                    }),
                }])
                .await
                .is_err());
            assert_eq!(stale_peer.application_state().await, stale_before);
            stale_peer
                .install_snapshot(&built_meta, Box::new(Cursor::new(built_bytes.clone())))
                .await
                .unwrap();
            assert_eq!(
                stale_peer.application_state().await.current_authority,
                Some(
                    ReplicatedAuthorityCustody::from_authority(
                        "polis-a",
                        membership.epoch(),
                        &authority,
                    )
                    .unwrap(),
                )
            );

            let current_root = root.path().join("current-peer");
            std::fs::create_dir(&current_root).unwrap();
            let mut current_peer = PolisStateMachineStore::open_with_trusted_authority(
                &current_root,
                2,
                Arc::new(MemoryAuthority::default()),
                TrustedAuthorityBootstrap {
                    polis_id: "polis-a".into(),
                    membership_epoch: membership.epoch(),
                    authority: authority.clone(),
                    boot_generations: restarted_boots.clone(),
                    publication_identity: publication_identity(2, &membership, &authority, 18),
                },
            )
            .unwrap();
            current_peer
                .install_snapshot(&built_meta, Box::new(Cursor::new(built_bytes)))
                .await
                .unwrap();
            let installed = current_peer.application_state().await;
            assert_eq!(
                installed.current_authority,
                Some(
                    ReplicatedAuthorityCustody::from_authority(
                        "polis-a",
                        membership.epoch(),
                        &authority,
                    )
                    .unwrap(),
                )
            );
            assert_eq!(
                installed.prepared_authority["snapshot-op-a"].boot_generations,
                canonical_boot_generations(&boots)
            );
            assert!(installed.finalized_authority.contains_key("snapshot-op-a"));
        } else {
            assert!(result.is_err());
        }
    }

    macro_rules! snapshot_case {
        ($name:ident, $case:ident, $result:literal) => {
            #[tokio::test]
            async fn $name() {
                run_snapshot_case(SnapshotCase::$case).await;
                println!(concat!(
                    "ADL_ISSUE_201_CASE_V2 ",
                    stringify!($name),
                    " ",
                    $result
                ));
            }
        };
    }

    snapshot_case!(
        snapshot_valid_multi_prepared_finalized_restart,
        Valid,
        "passed"
    );
    snapshot_case!(snapshot_current_polis_mismatch, CurrentPolis, "rejected");
    snapshot_case!(snapshot_current_epoch_mismatch, CurrentEpoch, "rejected");
    snapshot_case!(
        snapshot_current_membership_mismatch,
        CurrentMembership,
        "rejected"
    );
    snapshot_case!(snapshot_current_boot_mismatch, CurrentBoot, "rejected");
    snapshot_case!(snapshot_prepared_polis_mismatch, PreparedPolis, "rejected");
    snapshot_case!(snapshot_prepared_epoch_mismatch, PreparedEpoch, "rejected");
    snapshot_case!(
        snapshot_prepared_membership_mismatch,
        PreparedMembership,
        "rejected"
    );
    snapshot_case!(snapshot_prepared_boot_mismatch, PreparedBoot, "rejected");
    snapshot_case!(
        snapshot_later_prepared_custody_mismatch,
        LaterPrepared,
        "rejected"
    );
    snapshot_case!(snapshot_legacy_owner_injection, LegacyOwner, "rejected");
    snapshot_case!(
        snapshot_legacy_shepherd_injection,
        LegacyShepherd,
        "rejected"
    );
    snapshot_case!(
        snapshot_legacy_observatory_injection,
        LegacyObservatory,
        "rejected"
    );
    snapshot_case!(snapshot_legacy_fence_injection, LegacyFence, "rejected");
    snapshot_case!(
        snapshot_legacy_demotion_injection,
        LegacyDemotion,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_missing_proposal,
        MissingProposal,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_missing_endorsements,
        MissingEndorsements,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_wrong_operation,
        WrongOperation,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_insufficient_quorum,
        InsufficientQuorum,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_duplicate_quorum,
        DuplicateQuorum,
        "rejected"
    );
    snapshot_case!(snapshot_finalized_bad_signature, BadSignature, "rejected");
    snapshot_case!(
        snapshot_finalized_stale_certificate,
        StaleCertificate,
        "rejected"
    );
    snapshot_case!(snapshot_finalized_wrong_boot, WrongBoot, "rejected");
    snapshot_case!(snapshot_finalized_invalid_time, InvalidTime, "rejected");
    snapshot_case!(
        snapshot_finalized_wrong_prepare_index,
        WrongPrepareIndex,
        "rejected"
    );
    snapshot_case!(
        snapshot_finalized_wrong_finalize_index,
        WrongFinalizeIndex,
        "rejected"
    );
    snapshot_case!(snapshot_custody_omitted, CustodyOmitted, "rejected");
    snapshot_case!(snapshot_custody_reencoded, CustodyReencoded, "rejected");
    snapshot_case!(snapshot_custody_injected, CustodyInjected, "rejected");
    snapshot_case!(snapshot_custody_substituted, CustodySubstituted, "rejected");
    snapshot_case!(
        snapshot_custody_byte_digest_mismatch,
        CustodyByteDigestMismatch,
        "rejected"
    );
    snapshot_case!(snapshot_evidence_omitted, EvidenceOmitted, "rejected");
    snapshot_case!(snapshot_evidence_reencoded, EvidenceReencoded, "rejected");
    snapshot_case!(snapshot_evidence_injected, EvidenceInjected, "rejected");
    snapshot_case!(
        snapshot_evidence_substituted,
        EvidenceSubstituted,
        "rejected"
    );
    snapshot_case!(
        snapshot_evidence_byte_digest_mismatch,
        EvidenceByteDigestMismatch,
        "rejected"
    );

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn real_three_voter_authority_prepare_finalize_uses_applied_log_ids() {
        let root = tempfile::Builder::new()
            .prefix("adl-authority-raft-")
            .tempdir_in(std::env::current_dir().unwrap())
            .unwrap();
        let checkpoint = Arc::new(MemoryAuthority::default());
        let network = MemoryNetwork::default();
        let (membership, authority, boot_generations, signers) = authority_fixture(7);
        let publication_identities = (1..=3)
            .map(|node| (node, publication_identity(node, &membership, &authority, 7)))
            .collect::<BTreeMap<_, _>>();
        let fake_checkpoint = Arc::new(MemoryAuthority::default());
        let fake_root = root.path().join("fake-authority-publication");
        std::fs::create_dir(&fake_root).unwrap();
        let mut fake_identity = publication_identities[&1].clone();
        fake_identity.node_id = "node-fake".into();
        let fake_protocol =
            DurableAuthorityProtocol::open(&fake_root, fake_identity, fake_checkpoint.clone())
                .unwrap();
        assert!(fake_protocol.published("three-voter-authority").is_none());
        drop(fake_protocol);
        let fake_cas_before = fake_checkpoint.values.lock().unwrap().clone();

        let fake_machine_root = root.path().join("fake-node-machine");
        std::fs::create_dir(&fake_machine_root).unwrap();
        assert!(PolisStateMachineStore::open_with_trusted_authority(
            &fake_machine_root,
            1,
            fake_checkpoint.clone(),
            TrustedAuthorityBootstrap {
                polis_id: "polis-a".into(),
                membership_epoch: membership.epoch(),
                authority: authority.clone(),
                boot_generations: boot_generations.clone(),
                publication_identity: publication_identities[&2].clone(),
            },
        )
        .is_err());
        let configuration = Arc::new(
            openraft::Config {
                cluster_name: "adl-authority-protocol-test".to_owned(),
                heartbeat_interval: 50,
                election_timeout_min: 150,
                election_timeout_max: 300,
                ..Default::default()
            }
            .validate()
            .unwrap(),
        );
        let mut nodes = BTreeMap::new();
        let mut machines = BTreeMap::new();
        for node in 1..=3 {
            let node_root = root.path().join(format!("node-{node}"));
            std::fs::create_dir(&node_root).unwrap();
            let log = PolisLogStore::open(&node_root, node, checkpoint.clone()).unwrap();
            let machine = PolisStateMachineStore::open_with_trusted_authority(
                &node_root,
                node,
                checkpoint.clone(),
                TrustedAuthorityBootstrap {
                    polis_id: "polis-a".into(),
                    membership_epoch: membership.epoch(),
                    authority: authority.clone(),
                    boot_generations: boot_generations.clone(),
                    publication_identity: publication_identities[&node].clone(),
                },
            )
            .unwrap();
            let raft = PolisRaft::new(
                node,
                Arc::clone(&configuration),
                network.clone(),
                log,
                machine.clone(),
            )
            .await
            .unwrap();
            nodes.insert(node, raft);
            machines.insert(node, machine);
        }
        *network.peers.write().unwrap() = nodes.clone();
        nodes[&1]
            .initialize(
                (1..=3)
                    .map(|node| (node, BasicNode::new(format!("memory://{node}"))))
                    .collect::<BTreeMap<_, _>>(),
            )
            .await
            .unwrap();

        let mut protocol_checkpoint = None;
        for node in 1..=3 {
            let authority_root = root
                .path()
                .join(format!("node-{node}/authority-publication"));
            let protocol = DurableAuthorityProtocol::open(
                &authority_root,
                publication_identities[&node].clone(),
                checkpoint.clone(),
            )
            .unwrap();
            let observed = protocol.checkpoint_sha256().unwrap();
            assert!(protocol_checkpoint.is_none_or(|expected| expected == observed));
            protocol_checkpoint = Some(observed);
        }

        for sequence in 0..6 {
            write_on_leader(
                &nodes,
                PolisCommand::GovernedMutation {
                    mutation_id: format!("advance-{sequence}"),
                    payload_sha256: format!("{sequence:02x}").repeat(32),
                },
            )
            .await;
        }
        let prepare = AuthorityPrepareProposal::new(
            "polis-a",
            &membership,
            &authority,
            AuthorityOperationKind::Membership,
            protocol_checkpoint.unwrap(),
            CanonicalAuthorityTime {
                unix_seconds: 1_800_000_000,
                nanos: 0,
                uncertainty_millis: 1,
            },
            CanonicalAuthorityTime {
                unix_seconds: 1_800_000_050,
                nanos: 0,
                uncertainty_millis: 1,
            },
            "three-voter-authority",
            CommittedAuthorityArtifact::new(
                AuthorityOperationKind::Membership,
                b"exact-authority-store-artifact".to_vec(),
            )
            .unwrap(),
        )
        .unwrap();
        let (prepare_leader, prepare_response) = write_on_leader(
            &nodes,
            PolisCommand::PrepareAuthority {
                proposal: Box::new(prepare),
                boot_generations: canonical_boot_generations(&boot_generations),
            },
        )
        .await;
        let committed_intent = machines[&prepare_leader]
            .application_state()
            .await
            .prepared_authority_intent("three-voter-authority")
            .unwrap()
            .clone();
        assert_eq!(
            committed_intent.prepare_log_index,
            prepare_response.log_id.index
        );
        let finalization_time = CanonicalAuthorityTime {
            unix_seconds: 1_800_000_010,
            nanos: 0,
            uncertainty_millis: 1,
        };
        let endorsements = signers[..2]
            .iter()
            .map(|identity| {
                endorse_committed_authority_prepare_with_exclusion(
                    identity,
                    authority.voter_set_generation,
                    authority.voter_set_generation,
                    membership.committed_log_index(),
                    &boot_generations,
                    &committed_intent,
                    &finalization_time,
                    &membership,
                    &authority,
                    &NoPendingExclusion,
                )
                .unwrap()
            })
            .collect();
        let finalize =
            AuthorityFinalizeProposal::new(&committed_intent, finalization_time, endorsements)
                .unwrap();
        let (_, finalize_response) = write_on_leader(
            &nodes,
            PolisCommand::FinalizeAuthority { proposal: finalize },
        )
        .await;
        assert!(!finalize_response.data.accepted);
        assert_eq!(
            finalize_response.data.reason_code,
            "authority_publication_pending"
        );
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if machines.values().all(|machine| {
                    machine.inner.try_read().is_ok_and(|state| {
                        state
                            .payload
                            .current
                            .application
                            .finalized_authority_log_index("three-voter-authority")
                            == Some(finalize_response.log_id.index)
                    })
                }) {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        })
        .await
        .expect("all three voters applied the exact finalize entry");

        for node in 1..=3 {
            let authority_root = root
                .path()
                .join(format!("node-{node}/authority-publication"));
            checkpoint.arm_after_cas();
            assert!(machines[&node]
                .reconcile_authority_publication("three-voter-authority")
                .await
                .is_err());
            let recovered = DurableAuthorityProtocol::open(
                &authority_root,
                publication_identities[&node].clone(),
                checkpoint.clone(),
            )
            .unwrap();
            assert_eq!(
                recovered
                    .published("three-voter-authority")
                    .unwrap()
                    .committed_log_index(),
                finalize_response.log_id.index
            );
            drop(recovered);
            let published = machines[&node]
                .reconcile_authority_publication("three-voter-authority")
                .await
                .unwrap();
            assert_eq!(
                published.committed_log_index(),
                finalize_response.log_id.index
            );
            drop(published);
            let reopened = DurableAuthorityProtocol::open(
                &authority_root,
                publication_identities[&node].clone(),
                checkpoint.clone(),
            )
            .unwrap();
            assert_eq!(
                reopened
                    .published("three-voter-authority")
                    .unwrap()
                    .committed_log_index(),
                finalize_response.log_id.index
            );
        }
        assert_eq!(
            *fake_checkpoint.values.lock().unwrap(),
            fake_cas_before,
            "the configured voter must never write through a caller-selected CAS"
        );
        let fake_reopened = DurableAuthorityProtocol::open(
            &fake_root,
            AuthorityNodeIdentity {
                trust_domain: "polis.authority.test".into(),
                polis_id: "polis-a".into(),
                node_id: "node-fake".into(),
                guardian_id: publication_identities[&1].guardian_id.clone(),
                boot_generation: 7,
            },
            fake_checkpoint.clone(),
        )
        .unwrap();
        assert!(fake_reopened.published("three-voter-authority").is_none());
        drop(fake_reopened);

        let proposal_for = |operation_id: &str| {
            AuthorityPrepareProposal::new(
                "polis-a",
                &membership,
                &authority,
                AuthorityOperationKind::Membership,
                [9; 32],
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_020,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                CanonicalAuthorityTime {
                    unix_seconds: 1_800_000_050,
                    nanos: 0,
                    uncertainty_millis: 1,
                },
                operation_id,
                CommittedAuthorityArtifact::new(
                    AuthorityOperationKind::Membership,
                    format!("{operation_id}-artifact").into_bytes(),
                )
                .unwrap(),
            )
            .unwrap()
        };
        let mut wrong_polis = serde_json::to_value(proposal_for("wrong-polis")).unwrap();
        wrong_polis["intent_template"]["polis_id"] = serde_json::json!("polis-attacker");
        let wrong_polis = serde_json::from_value(wrong_polis).unwrap();
        let (_, wrong_polis_response) = write_on_leader(
            &nodes,
            PolisCommand::PrepareAuthority {
                proposal: Box::new(wrong_polis),
                boot_generations: canonical_boot_generations(&boot_generations),
            },
        )
        .await;
        assert!(!wrong_polis_response.data.accepted);
        assert_eq!(wrong_polis_response.data.reason_code, "governed_rejection");

        let mut wrong_epoch = serde_json::to_value(proposal_for("wrong-epoch")).unwrap();
        wrong_epoch["intent_template"]["membership_epoch"] =
            serde_json::json!(membership.epoch() + 1);
        let wrong_epoch = serde_json::from_value(wrong_epoch).unwrap();
        let (_, wrong_epoch_response) = write_on_leader(
            &nodes,
            PolisCommand::PrepareAuthority {
                proposal: Box::new(wrong_epoch),
                boot_generations: canonical_boot_generations(&boot_generations),
            },
        )
        .await;
        assert!(!wrong_epoch_response.data.accepted);
        assert_eq!(wrong_epoch_response.data.reason_code, "governed_rejection");
        for machine in machines.values() {
            let state = machine.application_state().await;
            assert!(state.prepared_authority_intent("wrong-polis").is_none());
            assert!(state.prepared_authority_intent("wrong-epoch").is_none());
        }

        let (attacker_membership, attacker_authority, _, _) = authority_fixture(8);
        let attacker = AuthorityPrepareProposal::new(
            "polis-a",
            &attacker_membership,
            &attacker_authority,
            AuthorityOperationKind::Membership,
            [8; 32],
            CanonicalAuthorityTime {
                unix_seconds: 1_800_000_000,
                nanos: 0,
                uncertainty_millis: 1,
            },
            CanonicalAuthorityTime {
                unix_seconds: 1_800_000_050,
                nanos: 0,
                uncertainty_millis: 1,
            },
            "attacker-selected-authority",
            CommittedAuthorityArtifact::new(
                AuthorityOperationKind::Membership,
                b"attacker-artifact".to_vec(),
            )
            .unwrap(),
        )
        .unwrap();
        let rejected = tokio::time::timeout(
            Duration::from_secs(3),
            nodes[&prepare_leader].client_write(PolisCommand::PrepareAuthority {
                proposal: Box::new(attacker),
                boot_generations: canonical_boot_generations(&boot_generations),
            }),
        )
        .await;
        assert!(!matches!(rejected, Ok(Ok(response)) if response.data.accepted));

        for raft in nodes.values() {
            let _ = raft.shutdown().await;
        }
    }

    #[tokio::test]
    async fn membership_history_retains_joint_and_uniform_entries_from_one_apply_batch() {
        let root = tempfile::Builder::new()
            .prefix("membership-history-")
            .tempdir_in(std::env::current_dir().unwrap())
            .unwrap();
        let checkpoint: Arc<dyn ConsensusCheckpointAuthority> =
            Arc::new(MemoryAuthority::default());
        let mut machine = PolisStateMachineStore::open(root.path(), 1, checkpoint.clone()).unwrap();
        let nodes = (1..=4)
            .map(|node| (node, BasicNode::new(format!("memory://{node}"))))
            .collect::<BTreeMap<_, _>>();
        let old = [1, 2, 3].into_iter().collect::<BTreeSet<_>>();
        let target = [2, 3, 4].into_iter().collect::<BTreeSet<_>>();
        let joint_log = LogId::new(openraft::CommittedLeaderId::new(7, 2), 41);
        let uniform_log = LogId::new(openraft::CommittedLeaderId::new(7, 2), 42);

        machine
            .apply([
                Entry {
                    log_id: joint_log,
                    payload: EntryPayload::Membership(openraft::Membership::new(
                        vec![old.clone(), target.clone()],
                        nodes.clone(),
                    )),
                },
                Entry {
                    log_id: uniform_log,
                    payload: EntryPayload::Membership(openraft::Membership::new(
                        vec![target.clone()],
                        nodes,
                    )),
                },
            ])
            .await
            .unwrap();

        let expected = vec![
            AppliedMembershipEntry {
                log_id: joint_log,
                joint_configs: vec![old, target.clone()],
            },
            AppliedMembershipEntry {
                log_id: uniform_log,
                joint_configs: vec![target],
            },
        ];
        assert_eq!(machine.applied_membership_history().await, expected);
        drop(machine);

        let reopened = PolisStateMachineStore::open(root.path(), 1, checkpoint).unwrap();
        assert_eq!(reopened.applied_membership_history().await, expected);
        println!("ADL_ISSUE_199_ASSERTION_V1 case=add_learner_joint_final_publish assertion=same_batch_joint_and_uniform_history_survives_restart");
    }

    #[test]
    fn production_boot_custody_denies_retained_generation_after_advance() {
        let root = tempfile::Builder::new()
            .prefix("boot-custody-")
            .tempdir_in(std::env::current_dir().unwrap())
            .unwrap();
        let checkpoint: Arc<dyn ConsensusCheckpointAuthority> =
            Arc::new(MemoryAuthority::default());
        let authority = SecureBootGenerationAuthority::open(root.path(), 41, checkpoint).unwrap();
        let generation_one = authority.advance().unwrap();
        let identity = LocalNodeGuardianIdentity::generate("boot-custody.test", 9).unwrap();
        let public = identity.public_identity();
        let learner = LearnerIdentity {
            trust_domain: public.trust_domain.clone(),
            polis_id: "polis-boot-custody".to_owned(),
            node_id: public.node_id.clone(),
            guardian_id: public.guardian_id.clone(),
            guardian_control_public_key: public.guardian_control_public_key,
            stable_raft_id: 41,
            certificate_generation: public.identity_generation,
            boot_generation: generation_one.generation(),
            address: "127.0.0.1:4041".parse().unwrap(),
        };
        let attestation =
            LearnerBootAttestationCustody::establish(generation_one, &identity, &learner).unwrap();
        assert!(attestation.require_current().is_ok());

        let generation_two = authority.advance().unwrap();
        assert_eq!(generation_two.generation(), learner.boot_generation + 1);
        assert!(matches!(
            attestation.require_current(),
            Err(LearnerTransportError::AuthorityDenied)
        ));
    }
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
