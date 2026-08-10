//! Production three-voter polis consensus owned by the Guardian process.
//!
//! OpenRaft is the only source of leader, quorum, committed-index, log
//! compaction, and snapshot-installation truth. Application authority changes
//! are applied only from committed log entries.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{Cursor, Write},
    ops::RangeBounds,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use adl_runtime_kernel::{
    DistributedObservatoryProjection, RuntimeDistributedInitConfig,
    DISTRIBUTED_OBSERVATORY_PROJECTION_SCHEMA,
};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use hmac::{Hmac, Mac};
use openraft::{
    error::{InstallSnapshotError, NetworkError, RPCError, RaftError, RemoteError, Unreachable},
    network::{RPCOption, RaftNetwork, RaftNetworkFactory},
    raft::{
        AppendEntriesRequest, AppendEntriesResponse, ClientWriteResponse, InstallSnapshotRequest,
        InstallSnapshotResponse, VoteRequest, VoteResponse,
    },
    storage::{LogFlushed, RaftLogStorage, RaftStateMachine, Snapshot},
    BasicNode, Entry, EntryPayload, ErrorSubject, ErrorVerb, LogId, LogState, RaftLogId,
    RaftLogReader, RaftSnapshotBuilder, RaftTypeConfig, SnapshotMeta, StorageError,
    StoredMembership, Vote,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::{sync::RwLock, task::JoinHandle};
use tokio_util::sync::CancellationToken;

pub const POLIS_WIRE_SCHEMA: &str = "adl.distributed.polis_rpc.v1";
pub const POLIS_OBSERVATORY_SCHEMA: &str = "adl.distributed.polis_observatory.v1";
pub const POLIS_LOCAL_MUTATION_SCHEMA: &str = "adl.distributed.local_governed_mutation.v1";
pub const POLIS_LOCAL_SNAPSHOT_SCHEMA: &str = "adl.distributed.local_snapshot_boundary.v1";
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalGovernedMutation {
    schema: String,
    mutation_id: String,
    payload_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocalSnapshotBoundary {
    schema: String,
    snapshot_sha256: String,
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

#[derive(Clone, Debug)]
pub struct PolisLogStore {
    path: PathBuf,
    inner: Arc<tokio::sync::Mutex<PersistedLog>>,
}

impl PolisLogStore {
    pub fn open(root: &Path) -> Result<Self, PolisRuntimeError> {
        ensure_store_root(root)?;
        let path = root.join("raft-log.json");
        let inner = if path.exists() {
            read_bounded_json(&path)?
        } else {
            PersistedLog::default()
        };
        Ok(Self {
            path,
            inner: Arc::new(tokio::sync::Mutex::new(inner)),
        })
    }

    #[allow(clippy::result_large_err)]
    fn persist(&self, state: &PersistedLog) -> Result<(), StorageError<NodeId>> {
        atomic_json_write(&self.path, state).map_err(|error| {
            StorageError::from_io_error(ErrorSubject::Store, ErrorVerb::Write, error)
        })
    }

    fn persist_io(&self, state: &PersistedLog) -> std::io::Result<()> {
        atomic_json_write(&self.path, state)
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
            .log
            .iter()
            .next_back()
            .map(|(_, entry)| *entry.get_log_id())
            .or(state.last_purged_log_id);
        Ok(LogState {
            last_purged_log_id: state.last_purged_log_id,
            last_log_id,
        })
    }

    async fn save_committed(
        &mut self,
        committed: Option<LogId<NodeId>>,
    ) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        state.committed = committed;
        self.persist(&state)
    }

    async fn read_committed(&mut self) -> Result<Option<LogId<NodeId>>, StorageError<NodeId>> {
        Ok(self.inner.lock().await.committed)
    }

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        state.vote = Some(*vote);
        self.persist(&state)
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        Ok(self.inner.lock().await.vote)
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
        for entry in entries {
            state.log.insert(entry.log_id.index, entry);
        }
        let result = self.persist_io(&state);
        match result {
            Ok(()) => {
                callback.log_io_completed(Ok(()));
                Ok(())
            }
            Err(error) => {
                callback
                    .log_io_completed(Err(std::io::Error::new(error.kind(), error.to_string())));
                Err(StorageError::from_io_error(
                    ErrorSubject::Logs,
                    ErrorVerb::Write,
                    error,
                ))
            }
        }
    }

    async fn truncate(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        state.log.retain(|index, _| *index < log_id.index);
        self.persist(&state)
    }

    async fn purge(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut state = self.inner.lock().await;
        state.log.retain(|index, _| *index > log_id.index);
        state.last_purged_log_id = Some(log_id);
        self.persist(&state)
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub struct PolisStateMachineStore {
    path: PathBuf,
    snapshot_path: PathBuf,
    state: Arc<RwLock<PersistedStateMachine>>,
    snapshot_sequence: Arc<AtomicU64>,
}

impl PolisStateMachineStore {
    pub fn open(root: &Path) -> Result<Self, PolisRuntimeError> {
        ensure_store_root(root)?;
        let path = root.join("raft-state-machine.json");
        let state = if path.exists() {
            read_bounded_json(&path)?
        } else {
            PersistedStateMachine::default()
        };
        Ok(Self {
            path,
            snapshot_path: root.join("raft-snapshot.json"),
            state: Arc::new(RwLock::new(state)),
            snapshot_sequence: Arc::new(AtomicU64::new(0)),
        })
    }

    pub async fn application_state(&self) -> PolisApplicationState {
        self.state.read().await.application.clone()
    }

    #[allow(clippy::result_large_err)]
    fn persist(&self, state: &PersistedStateMachine) -> Result<(), StorageError<NodeId>> {
        atomic_json_write(&self.path, state).map_err(|error| {
            StorageError::from_io_error(ErrorSubject::StateMachine, ErrorVerb::Write, error)
        })
    }
}

impl RaftSnapshotBuilder<PolisTypeConfig> for PolisStateMachineStore {
    async fn build_snapshot(&mut self) -> Result<Snapshot<PolisTypeConfig>, StorageError<NodeId>> {
        let state = self.state.read().await.clone();
        let data = serde_json::to_vec(&state).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::StateMachine,
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, error),
            )
        })?;
        atomic_bytes_write(&self.snapshot_path, &data).map_err(|error| {
            StorageError::from_io_error(ErrorSubject::Snapshot(None), ErrorVerb::Write, error)
        })?;
        let sequence = self.snapshot_sequence.fetch_add(1, Ordering::Relaxed) + 1;
        let meta = SnapshotMeta {
            last_log_id: state.last_applied_log,
            last_membership: state.last_membership,
            snapshot_id: format!("polis-{}-{sequence}", state.application.committed_index),
        };
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
        let state = self.state.read().await;
        Ok((state.last_applied_log, state.last_membership.clone()))
    }

    async fn apply<I>(&mut self, entries: I) -> Result<Vec<PolisResponse>, StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<PolisTypeConfig>> + Send,
    {
        let mut state = self.state.write().await;
        let mut responses = Vec::new();
        for entry in entries {
            state.last_applied_log = Some(entry.log_id);
            let (accepted, reason_code) = match entry.payload {
                EntryPayload::Blank => (true, "raft_internal"),
                EntryPayload::Membership(membership) => {
                    state.last_membership = StoredMembership::new(Some(entry.log_id), membership);
                    (true, "raft_internal")
                }
                EntryPayload::Normal(command) => {
                    let accepted = state
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
                committed_index: state.application.committed_index,
                epoch: state.application.epoch,
                accepted,
                reason_code: reason_code.to_owned(),
                state_sha256: state.application.digest().map_err(|error| {
                    StorageError::from_io_error(
                        ErrorSubject::StateMachine,
                        ErrorVerb::Read,
                        std::io::Error::new(std::io::ErrorKind::InvalidData, error.code()),
                    )
                })?,
            });
        }
        self.persist(&state)?;
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
        let mut state: PersistedStateMachine =
            serde_json::from_slice(snapshot.get_ref()).map_err(|error| {
                StorageError::from_io_error(
                    ErrorSubject::Snapshot(Some(meta.signature())),
                    ErrorVerb::Read,
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error),
                )
            })?;
        state.last_applied_log = meta.last_log_id;
        state.last_membership = meta.last_membership.clone();
        self.persist(&state)?;
        atomic_bytes_write(&self.snapshot_path, snapshot.get_ref()).map_err(|error| {
            StorageError::from_io_error(
                ErrorSubject::Snapshot(Some(meta.signature())),
                ErrorVerb::Write,
                error,
            )
        })?;
        *self.state.write().await = state;
        Ok(())
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<PolisTypeConfig>>, StorageError<NodeId>> {
        if !self.snapshot_path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&self.snapshot_path).map_err(|error| {
            StorageError::from_io_error(ErrorSubject::Snapshot(None), ErrorVerb::Read, error)
        })?;
        if data.len() > MAX_RPC_BYTES {
            return Err(StorageError::from_io_error(
                ErrorSubject::Snapshot(None),
                ErrorVerb::Read,
                std::io::Error::new(std::io::ErrorKind::InvalidData, "snapshot too large"),
            ));
        }
        let state = self.state.read().await;
        Ok(Some(Snapshot {
            meta: SnapshotMeta {
                last_log_id: state.last_applied_log,
                last_membership: state.last_membership.clone(),
                snapshot_id: format!("polis-{}-restored", state.application.committed_index),
            },
            snapshot: Box::new(Cursor::new(data)),
        }))
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignedRpc {
    schema: String,
    polis_id: String,
    trust_domain: String,
    sender: NodeId,
    receiver: NodeId,
    boot_generation: u64,
    sequence: u64,
    payload_base64: String,
    signature_hex: String,
}

impl SignedRpc {
    #[allow(clippy::too_many_arguments)]
    fn sign<T: Serialize>(
        polis_id: &str,
        trust_domain: &str,
        sender: NodeId,
        receiver: NodeId,
        boot_generation: u64,
        sequence: u64,
        value: &T,
        key: &SigningKey,
    ) -> Result<Self, PolisRuntimeError> {
        let payload = serde_json::to_vec(value).map_err(|_| PolisRuntimeError::Serialization)?;
        if payload.len() > MAX_RPC_BYTES || sequence == 0 || boot_generation == 0 {
            return Err(PolisRuntimeError::FrameTooLarge);
        }
        let payload_base64 = BASE64.encode(payload);
        let signature = key.sign(&signature_payload(
            polis_id,
            trust_domain,
            sender,
            receiver,
            boot_generation,
            sequence,
            &payload_base64,
        ));
        Ok(Self {
            schema: POLIS_WIRE_SCHEMA.to_owned(),
            polis_id: polis_id.to_owned(),
            trust_domain: trust_domain.to_owned(),
            sender,
            receiver,
            boot_generation,
            sequence,
            payload_base64,
            signature_hex: hex::encode(signature.to_bytes()),
        })
    }

    fn verify<T: DeserializeOwned>(
        &self,
        expected_polis_id: &str,
        expected_trust_domain: &str,
        expected_sender: NodeId,
        expected_receiver: NodeId,
        key: &VerifyingKey,
    ) -> Result<T, PolisRuntimeError> {
        if self.schema != POLIS_WIRE_SCHEMA
            || self.sender != expected_sender
            || self.receiver != expected_receiver
            || self.polis_id != expected_polis_id
            || self.trust_domain != expected_trust_domain
            || self.boot_generation == 0
            || self.sequence == 0
            || self.payload_base64.len() > MAX_RPC_BYTES.saturating_mul(2)
        {
            return Err(PolisRuntimeError::Authentication);
        }
        let signature_bytes =
            hex::decode(&self.signature_hex).map_err(|_| PolisRuntimeError::Authentication)?;
        let signature = ed25519_dalek::Signature::from_slice(&signature_bytes)
            .map_err(|_| PolisRuntimeError::Authentication)?;
        key.verify(
            &signature_payload(
                &self.polis_id,
                &self.trust_domain,
                self.sender,
                self.receiver,
                self.boot_generation,
                self.sequence,
                &self.payload_base64,
            ),
            &signature,
        )
        .map_err(|_| PolisRuntimeError::Authentication)?;
        let payload = BASE64
            .decode(&self.payload_base64)
            .map_err(|_| PolisRuntimeError::Authentication)?;
        if payload.len() > MAX_RPC_BYTES {
            return Err(PolisRuntimeError::FrameTooLarge);
        }
        serde_json::from_slice(&payload).map_err(|_| PolisRuntimeError::Serialization)
    }
}

fn signature_payload(
    polis_id: &str,
    trust_domain: &str,
    sender: NodeId,
    receiver: NodeId,
    boot_generation: u64,
    sequence: u64,
    payload_base64: &str,
) -> Vec<u8> {
    format!(
        "{POLIS_WIRE_SCHEMA}\0{polis_id}\0{trust_domain}\0{sender}\0{receiver}\0{boot_generation}\0{sequence}\0{payload_base64}"
    )
    .into_bytes()
}

#[derive(Clone)]
struct PolisNetworkFactory {
    client: reqwest::Client,
    local_id: NodeId,
    polis_id: Arc<str>,
    trust_domain: Arc<str>,
    boot_generation: u64,
    signing_key: Arc<SigningKey>,
    peer_keys: Arc<BTreeMap<NodeId, VerifyingKey>>,
    sequence: Arc<AtomicU64>,
    local_routes: Arc<BTreeMap<NodeId, BasicNode>>,
}

impl RaftNetworkFactory<PolisTypeConfig> for PolisNetworkFactory {
    type Network = PolisNetworkConnection;

    async fn new_client(&mut self, target: NodeId, _node: &BasicNode) -> Self::Network {
        PolisNetworkConnection {
            factory: self.clone(),
            target,
            target_node: self
                .local_routes
                .get(&target)
                .cloned()
                .unwrap_or_else(|| BasicNode::new("127.0.0.1:0")),
        }
    }
}

struct PolisNetworkConnection {
    factory: PolisNetworkFactory,
    target: NodeId,
    target_node: BasicNode,
}

type PolisRpcError<E = openraft::error::Infallible> =
    RPCError<NodeId, BasicNode, RaftError<NodeId, E>>;

impl PolisNetworkConnection {
    async fn send<Req, Resp, Err>(
        &self,
        route: &str,
        request: Req,
    ) -> Result<Resp, RPCError<NodeId, BasicNode, Err>>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
        Err: std::error::Error + DeserializeOwned,
    {
        let sequence = self.factory.sequence.fetch_add(1, Ordering::Relaxed) + 1;
        let envelope = SignedRpc::sign(
            &self.factory.polis_id,
            &self.factory.trust_domain,
            self.factory.local_id,
            self.target,
            self.factory.boot_generation,
            sequence,
            &request,
            &self.factory.signing_key,
        )
        .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
        let url = format!("http://{}/{route}", self.target_node.addr);
        let response = self
            .factory
            .client
            .post(url)
            .json(&envelope)
            .send()
            .await
            .map_err(|error| {
                if error.is_connect() || error.is_timeout() {
                    RPCError::Unreachable(Unreachable::new(&error))
                } else {
                    RPCError::Network(NetworkError::new(&error))
                }
            })?;
        let signed: SignedRpc = response
            .error_for_status()
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?
            .json()
            .await
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
        let key = self.factory.peer_keys.get(&self.target).ok_or_else(|| {
            RPCError::Network(NetworkError::new(&PolisRuntimeError::Authentication))
        })?;
        let result: Result<Resp, Err> = signed
            .verify(
                &self.factory.polis_id,
                &self.factory.trust_domain,
                self.target,
                self.factory.local_id,
                key,
            )
            .map_err(|error| RPCError::Network(NetworkError::new(&error)))?;
        result.map_err(|error| RPCError::RemoteError(RemoteError::new(self.target, error)))
    }
}

impl RaftNetwork<PolisTypeConfig> for PolisNetworkConnection {
    async fn append_entries(
        &mut self,
        request: AppendEntriesRequest<PolisTypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<NodeId>, PolisRpcError> {
        self.send("internal/raft/append", request).await
    }

    async fn install_snapshot(
        &mut self,
        request: InstallSnapshotRequest<PolisTypeConfig>,
        _option: RPCOption,
    ) -> Result<InstallSnapshotResponse<NodeId>, PolisRpcError<InstallSnapshotError>> {
        self.send("internal/raft/snapshot", request).await
    }

    async fn vote(
        &mut self,
        request: VoteRequest<NodeId>,
        _option: RPCOption,
    ) -> Result<VoteResponse<NodeId>, PolisRpcError> {
        self.send("internal/raft/vote", request).await
    }
}

#[derive(Clone)]
struct RpcServerState {
    raft: PolisRaft,
    local_id: NodeId,
    polis_id: Arc<str>,
    trust_domain: Arc<str>,
    boot_generation: u64,
    signing_key: Arc<SigningKey>,
    peer_keys: Arc<BTreeMap<NodeId, VerifyingKey>>,
    replay_path: PathBuf,
    replay_ledger: Arc<tokio::sync::Mutex<BTreeMap<NodeId, (u64, u64)>>>,
    local_kernel_token_mac: [u8; 32],
}

impl RpcServerState {
    async fn decode<T: DeserializeOwned>(&self, envelope: SignedRpc) -> Result<T, StatusCode> {
        let key = self
            .peer_keys
            .get(&envelope.sender)
            .ok_or(StatusCode::UNAUTHORIZED)?;
        let value = envelope
            .verify(
                &self.polis_id,
                &self.trust_domain,
                envelope.sender,
                self.local_id,
                key,
            )
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let mut ledger = self.replay_ledger.lock().await;
        let previous = ledger.entry(envelope.sender).or_insert((0, 0));
        if envelope.boot_generation < previous.0
            || (envelope.boot_generation == previous.0 && envelope.sequence <= previous.1)
        {
            return Err(StatusCode::CONFLICT);
        }
        *previous = (envelope.boot_generation, envelope.sequence);
        atomic_json_write(&self.replay_path, &*ledger)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(value)
    }

    fn encode<T: Serialize>(
        &self,
        receiver: NodeId,
        sequence: u64,
        value: &T,
    ) -> Result<Json<SignedRpc>, StatusCode> {
        SignedRpc::sign(
            &self.polis_id,
            &self.trust_domain,
            self.local_id,
            receiver,
            self.boot_generation,
            sequence,
            value,
            &self.signing_key,
        )
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn append_handler(
    State(state): State<RpcServerState>,
    Json(envelope): Json<SignedRpc>,
) -> Result<Json<SignedRpc>, StatusCode> {
    let receiver = envelope.sender;
    let sequence = envelope.sequence;
    let request = state.decode(envelope).await?;
    let response = state.raft.append_entries(request).await;
    state.encode(receiver, sequence, &response)
}

async fn vote_handler(
    State(state): State<RpcServerState>,
    Json(envelope): Json<SignedRpc>,
) -> Result<Json<SignedRpc>, StatusCode> {
    let receiver = envelope.sender;
    let sequence = envelope.sequence;
    let request = state.decode(envelope).await?;
    let response = state.raft.vote(request).await;
    state.encode(receiver, sequence, &response)
}

async fn snapshot_handler(
    State(state): State<RpcServerState>,
    Json(envelope): Json<SignedRpc>,
) -> Result<Json<SignedRpc>, StatusCode> {
    let receiver = envelope.sender;
    let sequence = envelope.sequence;
    let request = state.decode(envelope).await?;
    let response = state.raft.install_snapshot(request).await;
    state.encode(receiver, sequence, &response)
}

async fn health_handler() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn local_governed_mutation_handler(
    State(state): State<RpcServerState>,
    headers: HeaderMap,
    Json(request): Json<LocalGovernedMutation>,
) -> Result<Json<PolisResponse>, StatusCode> {
    authorize_local_kernel(&headers, &state.local_kernel_token_mac)?;
    if request.schema != POLIS_LOCAL_MUTATION_SCHEMA
        || validate_text(&request.mutation_id).is_err()
        || validate_sha256(&request.payload_sha256).is_err()
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let response = state
        .raft
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: request.mutation_id,
            payload_sha256: request.payload_sha256,
        })
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(response.data))
}

async fn local_snapshot_boundary_handler(
    State(state): State<RpcServerState>,
    headers: HeaderMap,
    Json(request): Json<LocalSnapshotBoundary>,
) -> Result<Json<PolisResponse>, StatusCode> {
    authorize_local_kernel(&headers, &state.local_kernel_token_mac)?;
    if request.schema != POLIS_LOCAL_SNAPSHOT_SCHEMA
        || validate_sha256(&request.snapshot_sha256).is_err()
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let response = state
        .raft
        .client_write(PolisCommand::SnapshotBoundary {
            snapshot_sha256: request.snapshot_sha256,
        })
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    state
        .raft
        .trigger()
        .snapshot()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(response.data))
}

fn authorize_local_kernel(headers: &HeaderMap, expected: &[u8; 32]) -> Result<(), StatusCode> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    verify_local_kernel_token(token.as_bytes(), expected)
        .then_some(())
        .ok_or(StatusCode::UNAUTHORIZED)
}

pub struct PolisRuntime {
    pub raft: PolisRaft,
    pub state_machine: PolisStateMachineStore,
    local_id: NodeId,
    server: JoinHandle<()>,
    cancellation: CancellationToken,
    authority_task: Option<JoinHandle<()>>,
    local_kernel_token: String,
}

#[derive(Clone)]
pub struct PolisAuthorityConfig {
    pub polis_id: String,
    pub trust_domain: String,
    pub guardian_id: String,
    pub shepherd_identity_ref: String,
    pub voter_ids: BTreeMap<NodeId, String>,
    pub projection_path: PathBuf,
    pub lease_millis: u64,
}

pub struct PolisRuntimeConfig {
    pub polis_id: String,
    pub trust_domain: String,
    pub local_id: NodeId,
    pub listen_address: std::net::SocketAddr,
    pub nodes: BTreeMap<NodeId, BasicNode>,
    pub bootstrap: bool,
    pub state_root: PathBuf,
    pub signing_key: SigningKey,
    pub peer_keys: BTreeMap<NodeId, VerifyingKey>,
    pub local_kernel_token: String,
}

impl PolisRuntimeConfig {
    pub fn from_runtime_init(
        distributed: &RuntimeDistributedInitConfig,
        state_root: &Path,
    ) -> Result<Self, PolisRuntimeError> {
        distributed
            .validate()
            .map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
        let mut voter_ids = distributed.voter_ids.clone();
        voter_ids.sort();
        let local_id = voter_ids
            .iter()
            .position(|voter_id| voter_id == &distributed.local_voter_id)
            .and_then(|index| u64::try_from(index + 1).ok())
            .ok_or(PolisRuntimeError::InvalidConfiguration)?;
        let mut nodes = BTreeMap::new();
        let mut peer_keys = BTreeMap::new();
        for (index, voter_id) in voter_ids.iter().enumerate() {
            let node_id =
                u64::try_from(index + 1).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
            let address = if voter_id == &distributed.local_voter_id {
                &distributed.listen_address
            } else {
                distributed
                    .peer_addresses
                    .get(voter_id)
                    .ok_or(PolisRuntimeError::InvalidConfiguration)?
            };
            nodes.insert(node_id, BasicNode::new(address));
            if node_id != local_id {
                let key_path = distributed
                    .voter_public_key_paths
                    .get(voter_id)
                    .ok_or(PolisRuntimeError::InvalidConfiguration)?;
                peer_keys.insert(node_id, read_verifying_key(&state_root.join(key_path))?);
            }
        }
        let signing_key = read_signing_key(&state_root.join(&distributed.voter_signing_key_path))?;
        let local_kernel_token =
            read_secret_text(&state_root.join(&distributed.local_kernel_token_path))?;
        Ok(Self {
            polis_id: distributed.polis_id.clone(),
            trust_domain: distributed.trust_domain.clone(),
            local_id,
            listen_address: distributed
                .listen_address
                .parse()
                .map_err(|_| PolisRuntimeError::InvalidConfiguration)?,
            nodes,
            bootstrap: distributed.bootstrap,
            state_root: state_root.join(&distributed.consensus_state_dir),
            signing_key,
            peer_keys,
            local_kernel_token,
        })
    }
}

impl PolisRuntime {
    pub async fn start(config: PolisRuntimeConfig) -> Result<Self, PolisRuntimeError> {
        if config.nodes.len() != 3
            || !config.nodes.contains_key(&config.local_id)
            || config.peer_keys.len() != 2
            || config.peer_keys.contains_key(&config.local_id)
        {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        validate_text(&config.polis_id)?;
        validate_text(&config.trust_domain)?;
        let log_store = PolisLogStore::open(&config.state_root)?;
        let state_machine = PolisStateMachineStore::open(&config.state_root)?;
        let boot_generation = advance_boot_generation(&config.state_root)?;
        let replay_path = config.state_root.join("raft-rpc-replay.json");
        let replay_ledger = if replay_path.exists() {
            read_bounded_json(&replay_path)?
        } else {
            BTreeMap::new()
        };
        let signing_key = Arc::new(config.signing_key);
        let peer_keys = Arc::new(config.peer_keys);
        let polis_id: Arc<str> = Arc::from(config.polis_id);
        let trust_domain: Arc<str> = Arc::from(config.trust_domain);
        let network = PolisNetworkFactory {
            client: reqwest::Client::builder()
                .connect_timeout(std::time::Duration::from_secs(2))
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|_| PolisRuntimeError::Network)?,
            local_id: config.local_id,
            polis_id: polis_id.clone(),
            trust_domain: trust_domain.clone(),
            boot_generation,
            signing_key: signing_key.clone(),
            peer_keys: peer_keys.clone(),
            sequence: Arc::new(AtomicU64::new(0)),
            local_routes: Arc::new(config.nodes.clone()),
        };
        let raft_config = Arc::new(
            openraft::Config {
                cluster_name: "adl-runtime-polis".to_owned(),
                heartbeat_interval: 500,
                election_timeout_min: 1_500,
                election_timeout_max: 3_000,
                snapshot_policy: openraft::SnapshotPolicy::LogsSinceLast(16),
                max_in_snapshot_log_to_keep: 8,
                ..Default::default()
            }
            .validate()
            .map_err(|_| PolisRuntimeError::InvalidConfiguration)?,
        );
        let raft = PolisRaft::new(
            config.local_id,
            raft_config,
            network,
            log_store,
            state_machine.clone(),
        )
        .await
        .map_err(|_| PolisRuntimeError::Storage)?;
        let server_state = RpcServerState {
            raft: raft.clone(),
            local_id: config.local_id,
            polis_id,
            trust_domain,
            boot_generation,
            signing_key,
            peer_keys,
            replay_path,
            replay_ledger: Arc::new(tokio::sync::Mutex::new(replay_ledger)),
            local_kernel_token_mac: local_kernel_token_mac(config.local_kernel_token.as_bytes()),
        };
        let listener = tokio::net::TcpListener::bind(config.listen_address)
            .await
            .map_err(|_| PolisRuntimeError::Network)?;
        let cancellation = CancellationToken::new();
        let stop = cancellation.clone();
        let router = Router::new()
            .route("/internal/raft/append", post(append_handler))
            .route("/internal/raft/vote", post(vote_handler))
            .route("/internal/raft/snapshot", post(snapshot_handler))
            .route(
                "/internal/client/governed-mutation",
                post(local_governed_mutation_handler),
            )
            .route(
                "/internal/client/snapshot-boundary",
                post(local_snapshot_boundary_handler),
            )
            .route("/internal/health", get(health_handler))
            .layer(DefaultBodyLimit::max(MAX_RPC_BYTES * 2))
            .with_state(server_state);
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, router)
                .with_graceful_shutdown(stop.cancelled_owned())
                .await;
        });
        if config.bootstrap {
            raft.initialize(config.nodes)
                .await
                .map_err(|_| PolisRuntimeError::Consensus)?;
        }
        Ok(Self {
            raft,
            state_machine,
            local_id: config.local_id,
            server,
            cancellation,
            authority_task: None,
            local_kernel_token: config.local_kernel_token,
        })
    }

    pub fn start_authority_loop(
        &mut self,
        config: PolisAuthorityConfig,
    ) -> Result<(), PolisRuntimeError> {
        if self.authority_task.is_some()
            || config.voter_ids.len() != 3
            || config.lease_millis < 1_000
            || config.lease_millis > 600_000
            || !config.projection_path.is_absolute()
        {
            return Err(PolisRuntimeError::InvalidConfiguration);
        }
        for value in [
            config.polis_id.as_str(),
            config.trust_domain.as_str(),
            config.guardian_id.as_str(),
            config.shepherd_identity_ref.as_str(),
        ] {
            validate_text(value)?;
        }
        for voter_id in config.voter_ids.values() {
            validate_text(voter_id)?;
        }
        let raft = self.raft.clone();
        let state_machine = self.state_machine.clone();
        let cancellation = self.cancellation.clone();
        let local_id = self.local_id;
        let projection_token = self.local_kernel_token.clone();
        self.authority_task = Some(tokio::spawn(async move {
            let mut tick = tokio::time::interval(std::time::Duration::from_millis(200));
            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => break,
                    _ = tick.tick() => {}
                }
                let metrics = raft.metrics().borrow().clone();
                let application = state_machine.application_state().await;
                let now = unix_millis();
                let leader_voter_id = metrics
                    .current_leader
                    .and_then(|leader| config.voter_ids.get(&leader).cloned())
                    .unwrap_or_else(|| "unavailable".to_owned());
                let owner_guardian_id = application
                    .observatory_owner
                    .clone()
                    .unwrap_or_else(|| "unavailable".to_owned());
                let mut projection = DistributedObservatoryProjection {
                    schema: DISTRIBUTED_OBSERVATORY_PROJECTION_SCHEMA.to_owned(),
                    polis_id: config.polis_id.clone(),
                    trust_domain: config.trust_domain.clone(),
                    owner_guardian_id,
                    leader_voter_id,
                    voter_ids: config.voter_ids.values().cloned().collect(),
                    quorum_size: 2,
                    committed_index: application.committed_index,
                    epoch: application.epoch,
                    expires_unix_millis: application
                        .observatory_expires_unix_millis
                        .unwrap_or_default(),
                    active_shepherd_identity_ref: application.active_shepherd.clone(),
                    snapshot_sha256: application.snapshot_sha256.clone(),
                    state_sha256: application.digest().unwrap_or_else(|_| "0".repeat(64)),
                    authorization_hmac_sha256: String::new(),
                };
                projection.authorization_hmac_sha256 =
                    projection_authorization_hmac(&projection, &projection_token)
                        .unwrap_or_else(|_| "0".repeat(64));
                let _ = atomic_json_write(&config.projection_path, &projection);

                if metrics.current_leader != Some(local_id) {
                    continue;
                }
                let lease_expires = application
                    .observatory_expires_unix_millis
                    .unwrap_or_default();
                let owns_lease =
                    application.observatory_owner.as_deref() == Some(config.guardian_id.as_str());
                let is_active_owner =
                    application.active_owner.as_deref() == Some(config.guardian_id.as_str());
                if !owns_lease && lease_expires > now {
                    continue;
                }
                let mut commands = Vec::new();
                let mut target_epoch = application.epoch;
                if !owns_lease && !is_active_owner {
                    let next_epoch = if let Some(previous_owner) = &application.active_owner {
                        let Some(next_epoch) = application.epoch.checked_add(1) else {
                            continue;
                        };
                        commands.push(PolisCommand::FenceVoter {
                            voter_id: previous_owner.clone(),
                            epoch: next_epoch,
                        });
                        next_epoch
                    } else {
                        application.epoch
                    };
                    target_epoch = next_epoch;
                    commands.push(PolisCommand::ActivateOwner {
                        owner_id: config.guardian_id.clone(),
                        epoch: next_epoch,
                    });
                    commands.push(PolisCommand::ActivateShepherd {
                        shepherd_identity_ref: config.shepherd_identity_ref.clone(),
                        epoch: next_epoch,
                    });
                }
                if !owns_lease || lease_expires.saturating_sub(now) <= config.lease_millis / 2 {
                    let Some(expires_unix_millis) = now.checked_add(config.lease_millis) else {
                        continue;
                    };
                    commands.push(PolisCommand::AcquireObservatory {
                        owner_id: config.guardian_id.clone(),
                        epoch: target_epoch,
                        expires_unix_millis,
                    });
                }
                for command in commands {
                    let result = tokio::time::timeout(
                        std::time::Duration::from_millis(config.lease_millis / 3),
                        raft.client_write(command),
                    )
                    .await;
                    if !matches!(result, Ok(Ok(response)) if response.data.accepted) {
                        break;
                    }
                }
            }
        }));
        Ok(())
    }

    pub async fn client_write(
        &self,
        command: PolisCommand,
    ) -> Result<ClientWriteResponse<PolisTypeConfig>, PolisRuntimeError> {
        self.raft
            .client_write(command)
            .await
            .map_err(|_| PolisRuntimeError::Consensus)
    }

    pub async fn ensure_linearizable(&self) -> Result<(), PolisRuntimeError> {
        self.raft
            .ensure_linearizable()
            .await
            .map(|_| ())
            .map_err(|_| PolisRuntimeError::QuorumUnavailable)
    }

    pub async fn shutdown(mut self) -> Result<(), PolisRuntimeError> {
        self.cancellation.cancel();
        if let Some(task) = self.authority_task.take() {
            task.await.map_err(|_| PolisRuntimeError::Network)?;
        }
        self.raft
            .shutdown()
            .await
            .map_err(|_| PolisRuntimeError::Consensus)?;
        self.server.await.map_err(|_| PolisRuntimeError::Network)?;
        Ok(())
    }
}

fn unix_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
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
    Consensus,
    QuorumUnavailable,
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
            Self::Consensus => "polis_consensus_failed",
            Self::QuorumUnavailable => "polis_quorum_unavailable",
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

fn read_key_hex(path: &Path) -> Result<[u8; 32], PolisRuntimeError> {
    validate_path_components(path, false).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    let metadata =
        std::fs::symlink_metadata(path).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 128 {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    let text =
        std::fs::read_to_string(path).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    let bytes = hex::decode(text.trim()).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    bytes
        .try_into()
        .map_err(|_| PolisRuntimeError::InvalidConfiguration)
}

fn read_secret_text(path: &Path) -> Result<String, PolisRuntimeError> {
    validate_path_components(path, false).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    let metadata =
        std::fs::symlink_metadata(path).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 512 {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    let token =
        std::fs::read_to_string(path).map_err(|_| PolisRuntimeError::InvalidConfiguration)?;
    let token = token.trim();
    if !(32..=256).contains(&token.len()) || !token.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err(PolisRuntimeError::InvalidConfiguration);
    }
    Ok(token.to_owned())
}

fn local_kernel_token_mac(token: &[u8]) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(b"adl-runtime-local-kernel-token-v1")
        .expect("fixed HMAC key is valid");
    mac.update(token);
    mac.finalize().into_bytes().into()
}

fn verify_local_kernel_token(token: &[u8], expected: &[u8; 32]) -> bool {
    let mut mac = Hmac::<Sha256>::new_from_slice(b"adl-runtime-local-kernel-token-v1")
        .expect("fixed HMAC key is valid");
    mac.update(token);
    mac.verify_slice(expected).is_ok()
}

fn projection_authorization_hmac(
    projection: &DistributedObservatoryProjection,
    token: &str,
) -> Result<String, PolisRuntimeError> {
    let mut unsigned = projection.clone();
    unsigned.authorization_hmac_sha256.clear();
    let bytes = serde_jcs::to_vec(&unsigned).map_err(|_| PolisRuntimeError::Serialization)?;
    let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes())
        .map_err(|_| PolisRuntimeError::Authentication)?;
    mac.update(&bytes);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn read_signing_key(path: &Path) -> Result<SigningKey, PolisRuntimeError> {
    Ok(SigningKey::from_bytes(&read_key_hex(path)?))
}

fn read_verifying_key(path: &Path) -> Result<VerifyingKey, PolisRuntimeError> {
    VerifyingKey::from_bytes(&read_key_hex(path)?)
        .map_err(|_| PolisRuntimeError::InvalidConfiguration)
}

fn advance_boot_generation(root: &Path) -> Result<u64, PolisRuntimeError> {
    let path = root.join("raft-rpc-boot-generation");
    let previous = if path.exists() {
        let metadata = std::fs::symlink_metadata(&path).map_err(|_| PolisRuntimeError::Storage)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 32 {
            return Err(PolisRuntimeError::Storage);
        }
        std::fs::read_to_string(&path)
            .map_err(|_| PolisRuntimeError::Storage)?
            .trim()
            .parse::<u64>()
            .map_err(|_| PolisRuntimeError::Storage)?
    } else {
        0
    };
    let next = previous
        .checked_add(1)
        .ok_or(PolisRuntimeError::StateRegression)?;
    atomic_bytes_write(&path, next.to_string().as_bytes())
        .map_err(|_| PolisRuntimeError::Storage)?;
    Ok(next)
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

fn read_bounded_json<T: DeserializeOwned>(path: &Path) -> Result<T, PolisRuntimeError> {
    validate_path_components(path, false).map_err(|_| PolisRuntimeError::Storage)?;
    let metadata = std::fs::symlink_metadata(path).map_err(|_| PolisRuntimeError::Storage)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() > MAX_RPC_BYTES as u64
    {
        return Err(PolisRuntimeError::Storage);
    }
    let bytes = std::fs::read(path).map_err(|_| PolisRuntimeError::Storage)?;
    serde_json::from_slice(&bytes).map_err(|_| PolisRuntimeError::Storage)
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
