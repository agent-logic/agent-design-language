//! Production-only Guardian to kernel continuity control plane.
//!
//! Channel authentication is supplied by TLS 1.3 mutual authentication.  The
//! values in this module are durable replay/effect bindings, not a second
//! application credential.  Constructors that can mint effect capabilities
//! remain inside the kernel crate.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    net::SocketAddr,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};

use fs2::FileExt;

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::sync::CancellationToken;

use crate::{CanonicalIngress, ReasoningServices, RuntimeRecorder};

pub const CONTROL_REQUEST_SCHEMA: &str = "adl.runtime.kernel_continuity.request.v1";
pub const CONTINUITY_CONTROL_RESPONSE_SCHEMA: &str = "adl.runtime.kernel_continuity.response.v1";
pub const CHANNEL_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.channel.v1";
pub const JOURNAL_SCHEMA: &str = "adl.runtime.kernel_continuity.journal.v1";
pub const STAGE_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.stage.v1";
pub const EXPORTER_LABEL: &[u8] = b"EXPORTER-ADL-Guardian-Kernel-Continuity-v1";
pub const PRIVATE_ALPN: &[&[u8]] = &[b"adl-continuity/1"];
const REQUIRED_PARTICIPANTS: [&str; 5] = [
    "admission",
    "accepted_prefix",
    "reasoning_mutation",
    "governance",
    "operation_state",
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityControlBounds {
    pub max_frame_bytes: usize,
    pub max_blob_bytes: u64,
    pub max_total_bytes: u64,
    pub max_services: usize,
    pub max_journal_entries: usize,
    pub max_open_handles: usize,
}

impl ContinuityControlBounds {
    pub fn validate(&self) -> Result<(), ContinuityControlError> {
        if self.max_frame_bytes == 0
            || self.max_blob_bytes == 0
            || self.max_total_bytes == 0
            || self.max_services == 0
            || self.max_journal_entries == 0
            || self.max_open_handles == 0
            || self.max_blob_bytes > self.max_total_bytes
        {
            return Err(ContinuityControlError::InvalidBounds);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityControlTlsConfig {
    pub server_certificate_chain_path: PathBuf,
    pub server_private_key_path: PathBuf,
    pub server_trust_roots_path: PathBuf,
    pub server_name: String,
    pub guardian_certificate_chain_path: PathBuf,
    pub guardian_private_key_path: PathBuf,
    pub guardian_trust_roots_path: PathBuf,
    pub guardian_spki_sha256: String,
    pub server_spki_sha256: String,
    pub certificate_generation: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successor: Option<CertificateSuccession>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificateSuccession {
    pub predecessor_spki_sha256: String,
    pub predecessor_generation: u64,
    pub successor_spki_sha256: String,
    pub successor_generation: u64,
    pub activation_sequence: u64,
    pub retirement_unix_millis: u64,
    pub successor_guardian_certificate_chain_path: PathBuf,
    pub successor_guardian_private_key_path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityControlInitConfig {
    pub address: String,
    pub guardian_state_dir: PathBuf,
    pub state_dir: PathBuf,
    pub staging_dir: PathBuf,
    pub trust_domain: String,
    pub polis: String,
    pub source_node: String,
    pub target_node: String,
    pub guardian_id: String,
    pub kernel_control_id: String,
    pub channel_epoch: u64,
    pub tls: ContinuityControlTlsConfig,
    pub bounds: ContinuityControlBounds,
}

impl ContinuityControlInitConfig {
    pub fn socket_addr(&self) -> Result<SocketAddr, ContinuityControlError> {
        self.address
            .parse::<SocketAddr>()
            .map_err(|_| ContinuityControlError::InvalidAddress)
    }

    pub fn validate(
        &self,
        state_root: &Path,
        public_addresses: &[SocketAddr],
    ) -> Result<(), ContinuityControlError> {
        let address = self.socket_addr()?;
        if address.port() == 0 || !address.ip().is_loopback() || public_addresses.contains(&address)
        {
            return Err(ContinuityControlError::InvalidAddress);
        }
        self.bounds.validate()?;
        for value in [
            &self.trust_domain,
            &self.polis,
            &self.source_node,
            &self.target_node,
            &self.guardian_id,
            &self.kernel_control_id,
            &self.tls.server_name,
        ] {
            if !safe_identity(value) {
                return Err(ContinuityControlError::InvalidIdentity);
            }
        }
        if self.guardian_id == self.kernel_control_id
            || self.guardian_id == self.source_node
            || self.guardian_id == self.target_node
            || self.source_node == self.target_node
            || self.channel_epoch == 0
            || self.tls.certificate_generation == 0
        {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        validate_digest(&self.tls.guardian_spki_sha256)?;
        validate_digest(&self.tls.server_spki_sha256)?;
        validate_private_root(state_root, &self.state_dir)?;
        validate_private_root(state_root, &self.guardian_state_dir)?;
        validate_private_root(state_root, &self.staging_dir)?;
        if roots_overlap(&self.state_dir, &self.staging_dir)
            || roots_overlap(&self.guardian_state_dir, &self.state_dir)
            || roots_overlap(&self.guardian_state_dir, &self.staging_dir)
        {
            return Err(ContinuityControlError::UnsafeRoot);
        }
        for path in [
            &self.tls.server_certificate_chain_path,
            &self.tls.server_private_key_path,
            &self.tls.server_trust_roots_path,
            &self.tls.guardian_certificate_chain_path,
            &self.tls.guardian_private_key_path,
            &self.tls.guardian_trust_roots_path,
        ] {
            validate_absolute_nonsymlink_path(path)?;
        }
        let unique_paths = [
            &self.tls.server_certificate_chain_path,
            &self.tls.server_private_key_path,
            &self.tls.server_trust_roots_path,
            &self.tls.guardian_certificate_chain_path,
            &self.tls.guardian_private_key_path,
            &self.tls.guardian_trust_roots_path,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if unique_paths.len() != 6 {
            return Err(ContinuityControlError::UnsafeRoot);
        }
        if let Some(succession) = &self.tls.successor {
            validate_digest(&succession.predecessor_spki_sha256)?;
            validate_digest(&succession.successor_spki_sha256)?;
            if succession.predecessor_generation != self.tls.certificate_generation
                || succession.successor_generation
                    != succession.predecessor_generation.saturating_add(1)
                || succession.predecessor_spki_sha256 != self.tls.guardian_spki_sha256
                || succession.predecessor_spki_sha256 == succession.successor_spki_sha256
                || succession.activation_sequence == 0
                || succession.retirement_unix_millis == 0
            {
                return Err(ContinuityControlError::InvalidSuccession);
            }
            for path in [
                &succession.successor_guardian_certificate_chain_path,
                &succession.successor_guardian_private_key_path,
            ] {
                validate_absolute_nonsymlink_path(path)?;
                if unique_paths.contains(path) {
                    return Err(ContinuityControlError::UnsafeRoot);
                }
            }
            if succession.successor_guardian_certificate_chain_path
                == succession.successor_guardian_private_key_path
            {
                return Err(ContinuityControlError::UnsafeRoot);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityOperationKind {
    QuiesceAndExport,
    ResumeSource,
    ReadBundleRange,
    StageTarget,
    WriteTargetChunk,
    ValidateTarget,
    ActivateTarget,
    DiscardTarget,
    Status,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityOperation {
    pub schema: String,
    pub trust_domain: String,
    pub polis: String,
    pub source_node: String,
    pub target_node: String,
    pub guardian_id: String,
    pub kernel_control_id: String,
    pub channel_epoch: u64,
    pub sequence: u64,
    pub operation_id: String,
    pub kind: ContinuityOperationKind,
    pub deadline_unix_millis: u64,
    pub accepted_prefix: u64,
    pub payload_sha256: String,
}

impl ContinuityOperation {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, ContinuityControlError> {
        serde_jcs::to_vec(self).map_err(|error| ContinuityControlError::Encoding(error.to_string()))
    }

    pub fn digest(&self) -> Result<String, ContinuityControlError> {
        Ok(sha256(&self.canonical_bytes()?))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityEnvelope {
    pub exporter_sha256: String,
    pub certificate_generation: u64,
    pub leaf_spki_sha256: String,
    pub operation: ContinuityOperation,
    pub command: ContinuityCommand,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityResultState {
    Accepted,
    Completed,
    RecoveryRequired,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityResponse {
    pub schema: String,
    pub request_sha256: String,
    pub state: ContinuityResultState,
    pub receipt_sha256: String,
    pub result: ContinuityReply,
    pub response_sha256: String,
}

impl ContinuityResponse {
    pub fn new(
        request_sha256: String,
        state: ContinuityResultState,
        receipt_sha256: String,
        result: ContinuityReply,
    ) -> Result<Self, ContinuityControlError> {
        let mut value = Self {
            schema: CONTINUITY_CONTROL_RESPONSE_SCHEMA.to_owned(),
            request_sha256,
            state,
            receipt_sha256,
            result,
            response_sha256: String::new(),
        };
        value.response_sha256 = sha256(
            &serde_jcs::to_vec(&value)
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
        );
        Ok(value)
    }
}

/// Decode one complete RFC 8785 representation.  Comparing the input with its
/// canonical re-encoding also rejects duplicate keys and alternate encodings.
pub fn decode_canonical<T: DeserializeOwned + Serialize>(
    bytes: &[u8],
    max_bytes: usize,
) -> Result<T, ContinuityControlError> {
    if bytes.is_empty() || bytes.len() > max_bytes {
        return Err(ContinuityControlError::FrameBounds);
    }
    let value: T = serde_json::from_slice(bytes)
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
    let canonical = serde_jcs::to_vec(&value)
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
    if canonical != bytes {
        return Err(ContinuityControlError::NonCanonical);
    }
    Ok(value)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ChannelState {
    schema: String,
    trust_domain: String,
    polis: String,
    guardian_id: String,
    kernel_control_id: String,
    channel_epoch: u64,
    last_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AcceptedOperation {
    operation_id: String,
    operation_sha256: String,
    sequence: u64,
    certificate_generation: u64,
    leaf_spki_sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    response: Option<ContinuityResponse>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JournalState {
    schema: String,
    accepted: BTreeMap<String, AcceptedOperation>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CertificateSuccessionState {
    predecessor_generation: u64,
    predecessor_spki_sha256: String,
    successor_generation: u64,
    successor_spki_sha256: String,
    activation_sequence: u64,
    retirement_unix_millis: String,
    activated: bool,
    retired: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableControlState {
    schema: String,
    channel: ChannelState,
    journal: JournalState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    succession: Option<CertificateSuccessionState>,
}

const CONTROL_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.control_state.v2";

pub enum BeginOperation {
    New { operation_sha256: String },
    Reconcile { operation_sha256: String },
    Retry(Box<ContinuityResponse>),
}

/// Exclusively owned durable channel state. The operating-system lock is
/// retained for the lifetime of this value, so it survives process crashes
/// without leaving a stale lock that prevents restart.
pub struct DurableContinuityJournal {
    root: PathBuf,
    _root_handle: File,
    _lock: File,
    state: DurableControlState,
    max_entries: usize,
}

impl DurableContinuityJournal {
    pub fn open(config: &ContinuityControlInitConfig) -> Result<Self, ContinuityControlError> {
        fs::create_dir_all(&config.state_dir)?;
        validate_existing_directory(&config.state_dir)?;
        let lock_path = config.state_dir.join("writer.lock");
        if lock_path.exists() && fs::symlink_metadata(&lock_path)?.file_type().is_symlink() {
            return Err(ContinuityControlError::UnsafeRoot);
        }
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lock_path)?;
        lock.try_lock_exclusive().map_err(|error| {
            if error.kind() == std::io::ErrorKind::WouldBlock {
                ContinuityControlError::AlreadyOpen
            } else {
                error.into()
            }
        })?;
        let expected_channel = ChannelState {
            schema: CHANNEL_STATE_SCHEMA.to_owned(),
            trust_domain: config.trust_domain.clone(),
            polis: config.polis.clone(),
            guardian_id: config.guardian_id.clone(),
            kernel_control_id: config.kernel_control_id.clone(),
            channel_epoch: config.channel_epoch,
            last_sequence: 0,
        };
        let state_path = config.state_dir.join("control-state.json");
        let root_handle = File::open(&config.state_dir)?;
        let expected_succession =
            config
                .tls
                .successor
                .as_ref()
                .map(|succession| CertificateSuccessionState {
                    predecessor_generation: succession.predecessor_generation,
                    predecessor_spki_sha256: succession.predecessor_spki_sha256.clone(),
                    successor_generation: succession.successor_generation,
                    successor_spki_sha256: succession.successor_spki_sha256.clone(),
                    activation_sequence: succession.activation_sequence,
                    retirement_unix_millis: succession.retirement_unix_millis.to_string(),
                    activated: false,
                    retired: false,
                });
        let state = if state_path.exists() {
            read_canonical_file(&state_path, config.bounds.max_frame_bytes)?
        } else {
            let value = DurableControlState {
                schema: CONTROL_STATE_SCHEMA.to_owned(),
                channel: expected_channel.clone(),
                journal: JournalState {
                    schema: JOURNAL_SCHEMA.to_owned(),
                    accepted: BTreeMap::new(),
                },
                succession: expected_succession.clone(),
            };
            write_canonical_file(&state_path, &value)?;
            value
        };
        if state.schema != CONTROL_STATE_SCHEMA
            || state.channel.schema != CHANNEL_STATE_SCHEMA
            || state.channel.trust_domain != expected_channel.trust_domain
            || state.channel.polis != expected_channel.polis
            || state.channel.guardian_id != expected_channel.guardian_id
            || state.channel.kernel_control_id != expected_channel.kernel_control_id
            || state.channel.channel_epoch != expected_channel.channel_epoch
            || state.journal.schema != JOURNAL_SCHEMA
            || state.journal.accepted.len() > config.bounds.max_journal_entries
            || state.succession.as_ref().map(|value| {
                (
                    value.predecessor_generation,
                    &value.predecessor_spki_sha256,
                    value.successor_generation,
                    &value.successor_spki_sha256,
                    value.activation_sequence,
                    &value.retirement_unix_millis,
                )
            }) != expected_succession.as_ref().map(|value| {
                (
                    value.predecessor_generation,
                    &value.predecessor_spki_sha256,
                    value.successor_generation,
                    &value.successor_spki_sha256,
                    value.activation_sequence,
                    &value.retirement_unix_millis,
                )
            })
        {
            return Err(ContinuityControlError::ChannelIdentity);
        }
        Ok(Self {
            root: config.state_dir.clone(),
            _root_handle: root_handle,
            _lock: lock,
            state,
            max_entries: config.bounds.max_journal_entries,
        })
    }

    pub fn begin(
        &mut self,
        config: &ContinuityControlInitConfig,
        envelope: &ContinuityEnvelope,
        exporter: &[u8],
        now_unix_millis: u64,
    ) -> Result<BeginOperation, ContinuityControlError> {
        verify_directory_anchor(&self.root, &self._root_handle)?;
        let operation = &envelope.operation;
        if operation.schema != CONTROL_REQUEST_SCHEMA
            || operation.trust_domain != config.trust_domain
            || operation.polis != config.polis
            || operation.source_node != config.source_node
            || operation.target_node != config.target_node
            || operation.guardian_id != config.guardian_id
            || operation.kernel_control_id != config.kernel_control_id
            || operation.channel_epoch != self.state.channel.channel_epoch
            || !safe_identity(&operation.operation_id)
        {
            return Err(ContinuityControlError::ChannelIdentity);
        }
        validate_digest(&operation.payload_sha256)?;
        validate_digest(&envelope.leaf_spki_sha256)?;
        if operation.deadline_unix_millis < now_unix_millis {
            return Err(ContinuityControlError::Deadline);
        }
        if !constant_time_eq(
            envelope.exporter_sha256.as_bytes(),
            sha256(exporter).as_bytes(),
        ) {
            return Err(ContinuityControlError::ExporterBinding);
        }
        if operation.kind != envelope.command.kind()
            || operation.payload_sha256
                != sha256(
                    &serde_jcs::to_vec(&envelope.command)
                        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
                )
        {
            return Err(ContinuityControlError::ContentMismatch);
        }
        let operation_sha256 = operation.digest()?;
        if let Some(existing) = self.state.journal.accepted.get(&operation.operation_id) {
            if existing.operation_sha256 != operation_sha256 {
                return Err(ContinuityControlError::ConflictingRetry);
            }
            self.validate_retry_certificate(config, envelope, existing)?;
            return Ok(existing
                .response
                .clone()
                .map(|response| BeginOperation::Retry(Box::new(response)))
                .unwrap_or(BeginOperation::Reconcile { operation_sha256 }));
        }
        self.validate_new_certificate(config, envelope)?;
        if operation.sequence != self.state.channel.last_sequence.saturating_add(1) {
            return Err(ContinuityControlError::Sequence);
        }
        if self.state.journal.accepted.len() >= self.max_entries {
            return Err(ContinuityControlError::JournalCapacity);
        }
        self.state.journal.accepted.insert(
            operation.operation_id.clone(),
            AcceptedOperation {
                operation_id: operation.operation_id.clone(),
                operation_sha256: operation_sha256.clone(),
                sequence: operation.sequence,
                certificate_generation: envelope.certificate_generation,
                leaf_spki_sha256: envelope.leaf_spki_sha256.clone(),
                response: None,
            },
        );
        self.state.channel.last_sequence = operation.sequence;
        if let Some(succession) = self.state.succession.as_mut() {
            succession.activated |= operation.sequence >= succession.activation_sequence;
        }
        self.reconcile_certificate_retirement(now_unix_millis)?;
        self.persist()?;
        Ok(BeginOperation::New { operation_sha256 })
    }

    pub fn complete(
        &mut self,
        operation_id: &str,
        response: ContinuityResponse,
    ) -> Result<ContinuityResponse, ContinuityControlError> {
        let accepted = self
            .state
            .journal
            .accepted
            .get_mut(operation_id)
            .ok_or(ContinuityControlError::UnknownOperation)?;
        if response.request_sha256 != accepted.operation_sha256 {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        if let Some(existing) = &accepted.response {
            if existing != &response {
                return Err(ContinuityControlError::ConflictingRetry);
            }
            return Ok(existing.clone());
        }
        accepted.response = Some(response.clone());
        self.reconcile_certificate_retirement(unix_millis()?)?;
        self.persist()?;
        Ok(response)
    }

    fn validate_new_certificate(
        &self,
        config: &ContinuityControlInitConfig,
        envelope: &ContinuityEnvelope,
    ) -> Result<(), ContinuityControlError> {
        let current = envelope.certificate_generation == config.tls.certificate_generation
            && constant_time_eq(
                envelope.leaf_spki_sha256.as_bytes(),
                config.tls.guardian_spki_sha256.as_bytes(),
            );
        let successor = config.tls.successor.as_ref().is_some_and(|succession| {
            envelope.certificate_generation == succession.successor_generation
                && envelope.leaf_spki_sha256 == succession.successor_spki_sha256
                && envelope.operation.sequence >= succession.activation_sequence
        });
        if current {
            if config.tls.successor.as_ref().is_some_and(|succession| {
                envelope.operation.sequence >= succession.activation_sequence
            }) {
                return Err(ContinuityControlError::StaleCertificate);
            }
            return Ok(());
        }
        if successor {
            return Ok(());
        }
        Err(ContinuityControlError::StaleCertificate)
    }

    fn validate_retry_certificate(
        &self,
        config: &ContinuityControlInitConfig,
        envelope: &ContinuityEnvelope,
        accepted: &AcceptedOperation,
    ) -> Result<(), ContinuityControlError> {
        if self.state.succession.as_ref().is_some_and(|succession| {
            succession.retired
                && envelope.certificate_generation == succession.predecessor_generation
        }) {
            return Err(ContinuityControlError::StaleCertificate);
        }
        if envelope.certificate_generation == accepted.certificate_generation
            && envelope.leaf_spki_sha256 == accepted.leaf_spki_sha256
        {
            return Ok(());
        }
        if config.tls.successor.as_ref().is_some_and(|succession| {
            accepted.certificate_generation == succession.predecessor_generation
                && accepted.leaf_spki_sha256 == succession.predecessor_spki_sha256
                && envelope.certificate_generation == succession.successor_generation
                && envelope.leaf_spki_sha256 == succession.successor_spki_sha256
        }) {
            return Ok(());
        }
        Err(ContinuityControlError::StaleCertificate)
    }

    fn persist(&self) -> Result<(), ContinuityControlError> {
        verify_directory_anchor(&self.root, &self._root_handle)?;
        write_canonical_file(&self.root.join("control-state.json"), &self.state)
    }

    fn reconcile_certificate_retirement(
        &mut self,
        now_unix_millis: u64,
    ) -> Result<(), ContinuityControlError> {
        let Some(succession) = self.state.succession.as_mut() else {
            return Ok(());
        };
        let retirement_unix_millis = succession
            .retirement_unix_millis
            .parse::<u64>()
            .map_err(|_| ContinuityControlError::InvalidSuccession)?;
        if succession.retired || !succession.activated || now_unix_millis < retirement_unix_millis {
            return Ok(());
        }
        let predecessor_pending = self.state.journal.accepted.values().any(|accepted| {
            accepted.certificate_generation == succession.predecessor_generation
                && accepted.response.is_none()
        });
        if !predecessor_pending {
            succession.retired = true;
        }
        Ok(())
    }
}

impl Drop for DurableContinuityJournal {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self._lock);
    }
}

#[derive(Clone)]
pub struct LiveContinuityRegistry {
    ingress: CanonicalIngress,
    recorder: RuntimeRecorder,
    reasoning: Arc<ReasoningServices>,
    operation_state_root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ParticipantPrepareReceipt {
    participant: String,
    accepted_prefix: u64,
    snapshot_sha256: String,
    receipt_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SourceOperationPhase {
    Preparing,
    Committed,
    Resuming,
    Resumed,
    RecoveryRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceOperationState {
    generation: u64,
    accepted_prefix: u64,
    phase: SourceOperationPhase,
    participant_receipts: BTreeMap<String, ParticipantPrepareReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    checkpoint: Option<SourceCheckpointHandle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    resume_receipt: Option<SourceResumeReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableSourceState {
    schema: String,
    operations: BTreeMap<u64, SourceOperationState>,
}

const SOURCE_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.source_state.v2";

impl LiveContinuityRegistry {
    pub fn from_production_handles(
        ingress: CanonicalIngress,
        recorder: RuntimeRecorder,
        reasoning: Arc<ReasoningServices>,
        operation_state_root: PathBuf,
        max_services: usize,
    ) -> Result<Self, ContinuityControlError> {
        if REQUIRED_PARTICIPANTS.len() != max_services {
            return Err(ContinuityControlError::ParticipantRegistry);
        }
        let registry = Self {
            ingress,
            recorder,
            reasoning,
            operation_state_root,
        };
        registry.validate_complete()?;
        Ok(registry)
    }

    pub fn services(&self) -> BTreeSet<String> {
        REQUIRED_PARTICIPANTS
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    pub fn validate_complete(&self) -> Result<(), ContinuityControlError> {
        if self.services()
            != REQUIRED_PARTICIPANTS
                .into_iter()
                .map(str::to_owned)
                .collect()
        {
            return Err(ContinuityControlError::ParticipantRegistry);
        }
        self.reasoning
            .continuity_snapshot_bytes()
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        crate::governance_live_registry_snapshot()
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        crate::assembly::operation_state_projection(&self.operation_state_root)?;
        Ok(())
    }

    async fn prepare_admission(
        &self,
        deadline: std::time::Duration,
        cancellation: &CancellationToken,
    ) -> Result<u64, ContinuityControlError> {
        self.validate_complete()?;
        if cancellation.is_cancelled() {
            return Err(ContinuityControlError::Cancelled);
        }
        self.ingress
            .close_and_drain(deadline)
            .await
            .map_err(|_| ContinuityControlError::Quiesce)?;
        if cancellation.is_cancelled() {
            self.ingress.reopen();
            return Err(ContinuityControlError::Cancelled);
        }
        Ok(self.ingress.snapshot().accepted_through)
    }

    fn snapshot_participant(&self, participant: &str) -> Result<Vec<u8>, ContinuityControlError> {
        match participant {
            "admission" => serde_jcs::to_vec(&self.ingress.snapshot())
                .map_err(|error| ContinuityControlError::Encoding(error.to_string())),
            "accepted_prefix" => serde_jcs::to_vec(&serde_json::json!({
                "schema": "adl.runtime.accepted_prefix.v1",
                "accepted_through": self.ingress.snapshot().accepted_through,
                "recorder": self.recorder.snapshot(),
            }))
            .map_err(|error| ContinuityControlError::Encoding(error.to_string())),
            "reasoning_mutation" => self
                .reasoning
                .continuity_snapshot_bytes()
                .map_err(|error| ContinuityControlError::Encoding(error.to_string())),
            "governance" => crate::governance_live_registry_snapshot()
                .map_err(|error| ContinuityControlError::Encoding(error.to_string())),
            "operation_state" => {
                crate::assembly::operation_state_projection(&self.operation_state_root)
            }
            _ => Err(ContinuityControlError::ParticipantRegistry),
        }
    }

    pub fn resume(&self, receipt_id: impl Into<String>) -> SourceResumeReceipt {
        self.ingress.reopen();
        SourceResumeReceipt::new(receipt_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleEntry {
    pub ordinal: u32,
    pub service: String,
    pub schema: String,
    pub file: String,
    pub offset: u64,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedBundleCatalog {
    pub generation: u64,
    pub predecessor_sha256: Option<String>,
    pub accepted_prefix: u64,
    pub topology_sha256: String,
    pub config_sha256: String,
    pub entries: Vec<BundleEntry>,
    pub key_id: String,
    pub key_generation: u64,
    pub signature: String,
}

impl SignedBundleCatalog {
    pub fn unsigned_bytes(&self) -> Result<Vec<u8>, ContinuityControlError> {
        let mut value = self.clone();
        value.signature.clear();
        serde_jcs::to_vec(&value)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
    }

    pub fn verify(
        &self,
        trusted_keys: &BTreeMap<(String, u64), VerifyingKey>,
    ) -> Result<(), ContinuityControlError> {
        let key = trusted_keys
            .get(&(self.key_id.clone(), self.key_generation))
            .ok_or(ContinuityControlError::ManifestSignature)?;
        let bytes =
            hex::decode(&self.signature).map_err(|_| ContinuityControlError::ManifestSignature)?;
        let signature =
            Signature::from_slice(&bytes).map_err(|_| ContinuityControlError::ManifestSignature)?;
        key.verify(&self.unsigned_bytes()?, &signature)
            .map_err(|_| ContinuityControlError::ManifestSignature)
    }

    pub fn digest(&self) -> Result<String, ContinuityControlError> {
        Ok(sha256(&serde_jcs::to_vec(self).map_err(|error| {
            ContinuityControlError::Encoding(error.to_string())
        })?))
    }
}

/// Kernel-local checkpoint content signer.  The private key never appears in a
/// handle, receipt, response, journal, or downstream port.
pub struct CatalogSigningAuthority {
    key_id: String,
    key_generation: u64,
    signing_key: SigningKey,
}

impl CatalogSigningAuthority {
    pub fn from_secret(
        key_id: impl Into<String>,
        key_generation: u64,
        secret: &[u8; 32],
    ) -> Result<Self, ContinuityControlError> {
        let key_id = key_id.into();
        if !safe_identity(&key_id) || key_generation == 0 {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        Ok(Self {
            key_id,
            key_generation,
            signing_key: SigningKey::from_bytes(secret),
        })
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    fn sign(&self, catalog: &mut SignedBundleCatalog) -> Result<(), ContinuityControlError> {
        catalog.key_id = self.key_id.clone();
        catalog.key_generation = self.key_generation;
        catalog.signature.clear();
        catalog.signature =
            hex::encode(self.signing_key.sign(&catalog.unsigned_bytes()?).to_bytes());
        Ok(())
    }
}

pub struct SourceContinuityEffectPort {
    export_root: PathBuf,
    _export_root_handle: File,
    registry: LiveContinuityRegistry,
    authority: CatalogSigningAuthority,
    bounds: ContinuityControlBounds,
    root_generation: u64,
    retained: Mutex<BTreeMap<u64, SignedBundleCatalog>>,
    source_state: Mutex<DurableSourceState>,
}

impl SourceContinuityEffectPort {
    pub fn open(
        export_root: PathBuf,
        registry: LiveContinuityRegistry,
        authority: CatalogSigningAuthority,
        bounds: ContinuityControlBounds,
        root_generation: u64,
    ) -> Result<Self, ContinuityControlError> {
        if root_generation == 0 {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        bounds.validate()?;
        fs::create_dir_all(&export_root)?;
        validate_existing_directory(&export_root)?;
        let export_root_handle = File::open(&export_root)?;
        registry.validate_complete()?;
        let trusted_key = authority.verifying_key();
        let trusted_keys = BTreeMap::from([(
            (authority.key_id.clone(), authority.key_generation),
            trusted_key,
        )]);
        let mut retained = BTreeMap::new();
        for entry in fs::read_dir(&export_root)? {
            let entry = entry?;
            if entry.file_name() == "source-state.json" && entry.file_type()?.is_file() {
                continue;
            }
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_dir() {
                return Err(ContinuityControlError::UnsafeRoot);
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some(generation) = name
                .strip_prefix("generation-")
                .and_then(|value| value.parse::<u64>().ok())
            else {
                return Err(ContinuityControlError::UnsafeRoot);
            };
            let catalog: SignedBundleCatalog =
                read_canonical_file(&entry.path().join("catalog.json"), bounds.max_frame_bytes)?;
            validate_catalog(&catalog, &bounds)?;
            catalog.verify(&trusted_keys)?;
            if catalog.generation != generation {
                return Err(ContinuityControlError::CatalogMismatch);
            }
            retained.insert(generation, catalog);
        }
        let state_path = export_root.join("source-state.json");
        let mut source_state = if state_path.exists() {
            read_canonical_file(&state_path, bounds.max_frame_bytes)?
        } else {
            DurableSourceState {
                schema: SOURCE_STATE_SCHEMA.to_owned(),
                operations: BTreeMap::new(),
            }
        };
        if source_state.schema != SOURCE_STATE_SCHEMA
            || source_state.operations.len() > bounds.max_journal_entries
        {
            return Err(ContinuityControlError::CorruptJournal);
        }
        for operation in source_state.operations.values_mut() {
            if let Some(catalog) = retained.get(&operation.generation) {
                if operation.participant_receipts.len() == REQUIRED_PARTICIPANTS.len()
                    && operation.participant_receipts.values().all(|receipt| {
                        receipt.accepted_prefix == operation.accepted_prefix
                            && receipt.receipt_sha256
                                == participant_receipt_digest(
                                    &receipt.participant,
                                    receipt.accepted_prefix,
                                    &receipt.snapshot_sha256,
                                )
                    })
                {
                    operation.checkpoint = Some(SourceCheckpointHandle {
                        generation: operation.generation,
                        root_generation,
                        catalog_sha256: catalog.digest()?,
                        bundle_sha256: bundle_digest(catalog)?,
                    });
                    operation.phase = SourceOperationPhase::Committed;
                } else {
                    operation.phase = SourceOperationPhase::RecoveryRequired;
                }
            }
            match operation.phase {
                SourceOperationPhase::Committed | SourceOperationPhase::RecoveryRequired => {
                    registry.ingress.close();
                }
                SourceOperationPhase::Preparing | SourceOperationPhase::Resuming => {
                    registry.ingress.close();
                    if operation.participant_receipts.values().all(|receipt| {
                        receipt.accepted_prefix == operation.accepted_prefix
                            && receipt.receipt_sha256
                                == participant_receipt_digest(
                                    &receipt.participant,
                                    receipt.accepted_prefix,
                                    &receipt.snapshot_sha256,
                                )
                    }) {
                        let receipt = registry.resume(format!(
                            "source-restart-resumed:{}:{}",
                            operation.generation, operation.accepted_prefix
                        ));
                        operation.resume_receipt = Some(receipt);
                        operation.phase = SourceOperationPhase::Resumed;
                    } else {
                        operation.phase = SourceOperationPhase::RecoveryRequired;
                    }
                }
                SourceOperationPhase::Resumed => {}
            }
        }
        write_canonical_file(&state_path, &source_state)?;
        Ok(Self {
            export_root,
            _export_root_handle: export_root_handle,
            registry,
            authority,
            bounds,
            root_generation,
            retained: Mutex::new(retained),
            source_state: Mutex::new(source_state),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn quiesce_and_export(
        &self,
        generation: u64,
        predecessor_sha256: Option<String>,
        accepted_prefix: u64,
        topology_sha256: String,
        config_sha256: String,
        deadline: std::time::Duration,
        cancellation: &CancellationToken,
    ) -> Result<(SourceQuiesceReceipt, SourceCheckpointHandle), ContinuityControlError> {
        if generation == 0 {
            return Err(ContinuityControlError::CatalogMismatch);
        }
        self.verify_export_root_anchor()?;
        validate_digest(&topology_sha256)?;
        validate_digest(&config_sha256)?;
        if let Some(predecessor) = &predecessor_sha256 {
            validate_digest(predecessor)?;
        }
        if let Some(catalog) = self
            .retained
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .get(&generation)
            .cloned()
        {
            let catalog_sha256 = catalog.digest()?;
            let bundle_sha256 = bundle_digest(&catalog)?;
            return Ok((
                SourceQuiesceReceipt::new(format!("quiesced:{generation}:{accepted_prefix}")),
                SourceCheckpointHandle {
                    generation,
                    root_generation: self.root_generation,
                    catalog_sha256,
                    bundle_sha256,
                },
            ));
        }
        if self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .operations
            .values()
            .any(|state| {
                matches!(
                    state.phase,
                    SourceOperationPhase::Committed | SourceOperationPhase::RecoveryRequired
                )
            })
        {
            return Err(ContinuityControlError::ReconcileRequired);
        }
        let started = tokio::time::Instant::now();
        let actual_prefix = self
            .registry
            .prepare_admission(deadline, cancellation)
            .await?;
        if actual_prefix != accepted_prefix {
            self.registry.resume("source-resumed-after-prefix-mismatch");
            return Err(ContinuityControlError::AcceptedPrefix);
        }
        {
            let mut state = self
                .source_state
                .lock()
                .map_err(|_| ContinuityControlError::CorruptJournal)?;
            state.operations.insert(
                generation,
                SourceOperationState {
                    generation,
                    accepted_prefix,
                    phase: SourceOperationPhase::Preparing,
                    participant_receipts: BTreeMap::new(),
                    checkpoint: None,
                    resume_receipt: None,
                },
            );
            self.persist_source_state(&state)?;
        }
        let mut snapshots = BTreeMap::new();
        for participant in REQUIRED_PARTICIPANTS {
            if cancellation.is_cancelled() || started.elapsed() >= deadline {
                return self.rollback_source_prepare(
                    generation,
                    if cancellation.is_cancelled() {
                        ContinuityControlError::Cancelled
                    } else {
                        ContinuityControlError::Deadline
                    },
                );
            }
            let bytes = match self.registry.snapshot_participant(participant) {
                Ok(bytes) => bytes,
                Err(error) => return self.rollback_source_prepare(generation, error),
            };
            let snapshot_sha256 = sha256(&bytes);
            let receipt = ParticipantPrepareReceipt {
                participant: participant.to_owned(),
                accepted_prefix,
                receipt_sha256: participant_receipt_digest(
                    participant,
                    accepted_prefix,
                    &snapshot_sha256,
                ),
                snapshot_sha256,
            };
            {
                let mut state = self
                    .source_state
                    .lock()
                    .map_err(|_| ContinuityControlError::CorruptJournal)?;
                state
                    .operations
                    .get_mut(&generation)
                    .ok_or(ContinuityControlError::CorruptJournal)?
                    .participant_receipts
                    .insert(participant.to_owned(), receipt);
                self.persist_source_state(&state)?;
            }
            snapshots.insert(participant.to_owned(), bytes);
            // Give cancellation, deadline, and supervised state changes an
            // observation point between participant effects.
            tokio::task::yield_now().await;
        }
        if snapshots.len() > self.bounds.max_services {
            return self.rollback_source_prepare(generation, ContinuityControlError::InvalidBounds);
        }
        let pending = self
            .export_root
            .join(format!(".generation-{generation}.pending"));
        let committed = self.export_root.join(format!("generation-{generation}"));
        if pending.exists() || committed.exists() {
            return self
                .rollback_source_prepare(generation, ContinuityControlError::ConflictingRetry);
        }
        if let Err(error) = fs::create_dir(&pending) {
            return self.rollback_source_prepare(generation, error.into());
        }
        let result = (|| {
            let mut entries = Vec::with_capacity(snapshots.len());
            let mut offset = 0_u64;
            let mut total = 0_u64;
            for (index, (service, bytes)) in snapshots.into_iter().enumerate() {
                if bytes.is_empty() || bytes.len() as u64 > self.bounds.max_blob_bytes {
                    return Err(ContinuityControlError::InvalidBounds);
                }
                total = total
                    .checked_add(bytes.len() as u64)
                    .ok_or(ContinuityControlError::InvalidBounds)?;
                if total > self.bounds.max_total_bytes {
                    return Err(ContinuityControlError::InvalidBounds);
                }
                let ordinal = (index as u32).saturating_add(1);
                let file = format!("{ordinal:04}-{service}.bin");
                validate_leaf_name(&file)?;
                write_new_synced(&pending.join(&file), &bytes)?;
                entries.push(BundleEntry {
                    ordinal,
                    service,
                    schema: "adl.runtime.live_participant.v1".to_owned(),
                    file,
                    offset,
                    bytes: bytes.len() as u64,
                    sha256: sha256(&bytes),
                });
                offset = offset
                    .checked_add(bytes.len() as u64)
                    .ok_or(ContinuityControlError::InvalidBounds)?;
            }
            let mut catalog = SignedBundleCatalog {
                generation,
                predecessor_sha256,
                accepted_prefix,
                topology_sha256,
                config_sha256,
                entries,
                key_id: String::new(),
                key_generation: 0,
                signature: String::new(),
            };
            self.authority.sign(&mut catalog)?;
            validate_catalog(&catalog, &self.bounds)?;
            write_canonical_file(&pending.join("catalog.json"), &catalog)?;
            sync_dir(&pending)?;
            fs::rename(&pending, &committed)?;
            sync_dir(&self.export_root)?;
            Ok(catalog)
        })();
        let catalog = match result {
            Ok(catalog) => catalog,
            Err(error) => {
                let _ = fs::remove_dir_all(&pending);
                return self.rollback_source_prepare(generation, error);
            }
        };
        let catalog_sha256 = catalog.digest()?;
        let bundle_sha256 = bundle_digest(&catalog)?;
        self.retained
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .insert(generation, catalog);
        let checkpoint = SourceCheckpointHandle {
            generation,
            root_generation: self.root_generation,
            catalog_sha256: catalog_sha256.clone(),
            bundle_sha256: bundle_sha256.clone(),
        };
        {
            let mut state = self
                .source_state
                .lock()
                .map_err(|_| ContinuityControlError::CorruptJournal)?;
            let operation = state
                .operations
                .get_mut(&generation)
                .ok_or(ContinuityControlError::CorruptJournal)?;
            operation.phase = SourceOperationPhase::Committed;
            operation.checkpoint = Some(checkpoint.clone());
            self.persist_source_state(&state)?;
        }
        Ok((
            SourceQuiesceReceipt::new(format!("quiesced:{generation}:{accepted_prefix}")),
            checkpoint,
        ))
    }

    pub fn bundle_source(
        &self,
        handle: &SourceCheckpointHandle,
    ) -> Result<ContinuityBundleSourcePort, ContinuityControlError> {
        if handle.root_generation != self.root_generation {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        let catalog = self
            .retained
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .get(&handle.generation)
            .cloned()
            .ok_or(ContinuityControlError::UnknownOperation)?;
        if catalog.digest()? != handle.catalog_sha256
            || bundle_digest(&catalog)? != handle.bundle_sha256
        {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        Ok(ContinuityBundleSourcePort {
            generation_root: self
                .export_root
                .join(format!("generation-{}", handle.generation)),
            handle: handle.clone(),
            catalog,
            bounds: self.bounds.clone(),
        })
    }

    pub fn resume_source(
        &self,
        handle: &SourceCheckpointHandle,
    ) -> Result<SourceResumeReceipt, ContinuityControlError> {
        if handle.root_generation != self.root_generation
            || !self
                .retained
                .lock()
                .map_err(|_| ContinuityControlError::CorruptJournal)?
                .contains_key(&handle.generation)
        {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        let mut state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?;
        let operation = state
            .operations
            .get_mut(&handle.generation)
            .ok_or(ContinuityControlError::UnknownOperation)?;
        if operation.checkpoint.as_ref() != Some(handle) {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        if operation.phase == SourceOperationPhase::Resumed {
            return operation
                .resume_receipt
                .clone()
                .ok_or(ContinuityControlError::CorruptJournal);
        }
        if operation.phase != SourceOperationPhase::Committed {
            return Err(ContinuityControlError::RecoveryRequired);
        }
        operation.phase = SourceOperationPhase::Resuming;
        self.persist_source_state(&state)?;
        let receipt = self.registry.resume(format!(
            "source-resumed:{}:{}:{}",
            handle.generation, handle.catalog_sha256, handle.bundle_sha256
        ));
        let operation = state
            .operations
            .get_mut(&handle.generation)
            .ok_or(ContinuityControlError::CorruptJournal)?;
        operation.resume_receipt = Some(receipt.clone());
        operation.phase = SourceOperationPhase::Resumed;
        self.persist_source_state(&state)?;
        Ok(receipt)
    }

    pub fn source_requires_resume(
        &self,
        handle: &SourceCheckpointHandle,
    ) -> Result<bool, ContinuityControlError> {
        let state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?;
        let operation = state
            .operations
            .get(&handle.generation)
            .ok_or(ContinuityControlError::UnknownOperation)?;
        if operation.checkpoint.as_ref() != Some(handle) {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        Ok(matches!(
            operation.phase,
            SourceOperationPhase::Committed | SourceOperationPhase::RecoveryRequired
        ))
    }

    fn rollback_source_prepare<T>(
        &self,
        generation: u64,
        error: ContinuityControlError,
    ) -> Result<T, ContinuityControlError> {
        let mut state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        let accepted_prefix = {
            let operation = state
                .operations
                .get_mut(&generation)
                .ok_or(ContinuityControlError::RecoveryRequired)?;
            operation.phase = SourceOperationPhase::Resuming;
            operation.accepted_prefix
        };
        self.persist_source_state(&state)
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        let receipt = self.registry.resume(format!(
            "source-rollback-resumed:{}:{}",
            generation, accepted_prefix
        ));
        let operation = state
            .operations
            .get_mut(&generation)
            .ok_or(ContinuityControlError::RecoveryRequired)?;
        operation.resume_receipt = Some(receipt);
        operation.phase = SourceOperationPhase::Resumed;
        self.persist_source_state(&state)
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        Err(error)
    }

    fn persist_source_state(
        &self,
        state: &DurableSourceState,
    ) -> Result<(), ContinuityControlError> {
        write_canonical_file(&self.export_root.join("source-state.json"), state)
    }

    fn verify_export_root_anchor(&self) -> Result<(), ContinuityControlError> {
        verify_directory_anchor(&self.export_root, &self._export_root_handle)
    }
}

fn participant_receipt_digest(
    participant: &str,
    accepted_prefix: u64,
    snapshot_sha256: &str,
) -> String {
    sha256(format!("{participant}\0{accepted_prefix}\0{snapshot_sha256}").as_bytes())
}

pub struct ContinuityBundleSourcePort {
    generation_root: PathBuf,
    handle: SourceCheckpointHandle,
    catalog: SignedBundleCatalog,
    bounds: ContinuityControlBounds,
}

impl ContinuityBundleSourcePort {
    pub fn catalog(&self) -> &SignedBundleCatalog {
        &self.catalog
    }
    pub fn handle(&self) -> &SourceCheckpointHandle {
        &self.handle
    }

    pub fn read_entry_range(
        &self,
        ordinal: u32,
        relative_offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ContinuityControlError> {
        let entry = self
            .catalog
            .entries
            .iter()
            .find(|entry| entry.ordinal == ordinal)
            .ok_or(ContinuityControlError::CatalogMismatch)?;
        let end = relative_offset
            .checked_add(length)
            .ok_or(ContinuityControlError::InvalidBounds)?;
        if length == 0 || end > entry.bytes || length > self.bounds.max_blob_bytes {
            return Err(ContinuityControlError::InvalidBounds);
        }
        let path = self.generation_root.join(&entry.file);
        validate_regular_single_link_file(&path, entry.bytes)?;
        use std::io::{Seek, SeekFrom};
        let mut file = File::open(&path)?;
        file.seek(SeekFrom::Start(relative_offset))?;
        let mut bytes = Vec::new();
        file.take(length.saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 != length {
            return Err(ContinuityControlError::ContentMismatch);
        }
        Ok(bytes)
    }
}

fn bundle_digest(catalog: &SignedBundleCatalog) -> Result<String, ContinuityControlError> {
    let mut hash = Sha256::new();
    hash.update(catalog.digest()?.as_bytes());
    for entry in &catalog.entries {
        hash.update(entry.ordinal.to_be_bytes());
        hash.update(entry.sha256.as_bytes());
    }
    Ok(hex::encode(hash.finalize()))
}

macro_rules! opaque_receipt {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            id: String,
            sha256: String,
        }
        impl $name {
            fn new(id: impl Into<String>) -> Self {
                let id = id.into();
                let sha256 = sha256(id.as_bytes());
                Self { id, sha256 }
            }
            pub fn id(&self) -> &str {
                &self.id
            }
            pub fn digest(&self) -> &str {
                &self.sha256
            }
        }
    };
}

opaque_receipt!(SourceQuiesceReceipt);
opaque_receipt!(SourceResumeReceipt);
opaque_receipt!(TargetPossessionEvidence);
opaque_receipt!(TargetDiscardReceipt);
opaque_receipt!(TargetActivationReceipt);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceCheckpointHandle {
    generation: u64,
    root_generation: u64,
    catalog_sha256: String,
    bundle_sha256: String,
}

impl SourceCheckpointHandle {
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn catalog_digest(&self) -> &str {
        &self.catalog_sha256
    }
    pub fn bundle_digest(&self) -> &str {
        &self.bundle_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetStageHandle {
    stage_id: String,
    root_generation: u64,
    catalog_sha256: String,
}

impl TargetStageHandle {
    pub fn id(&self) -> &str {
        &self.stage_id
    }
    pub fn catalog_digest(&self) -> &str {
        &self.catalog_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetCleanupPermit {
    stage_id: String,
    root_generation: u64,
    channel_epoch: u64,
    catalog_sha256: String,
    cleanup_id: String,
}

impl TargetCleanupPermit {
    pub fn stage_id(&self) -> &str {
        &self.stage_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageCreated {
    pub handle: TargetStageHandle,
    pub cleanup: TargetCleanupPermit,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FinalizedMigrationDecision {
    decision_id: String,
    stage_id: String,
    possession_sha256: String,
    trust_domain: String,
    polis: String,
    target_node: String,
    channel_epoch: u64,
    route_cut_sha256: String,
    membership_cut_sha256: String,
    certificate_cut_sha256: String,
    boot_cut_sha256: String,
    lineage_sha256: String,
}

/// Signed #204 decision projection accepted by the kernel. Its fields may be
/// transported, but they do not become an effect capability until the target
/// coordinator verifies the signature against its boot-trusted authority set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationDecisionCertificate {
    pub decision_id: String,
    pub stage_id: String,
    pub possession_sha256: String,
    pub trust_domain: String,
    pub polis: String,
    pub target_node: String,
    pub channel_epoch: u64,
    pub route_cut_sha256: String,
    pub membership_cut_sha256: String,
    pub certificate_cut_sha256: String,
    pub boot_cut_sha256: String,
    pub lineage_sha256: String,
    pub authority_key_id: String,
    pub authority_key_generation: u64,
    pub signature: String,
}

impl MigrationDecisionCertificate {
    fn unsigned_bytes(&self) -> Result<Vec<u8>, ContinuityControlError> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_jcs::to_vec(&unsigned)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
    }

    pub fn sign(
        mut self,
        authority: &CatalogSigningAuthority,
    ) -> Result<Self, ContinuityControlError> {
        self.authority_key_id = authority.key_id.clone();
        self.authority_key_generation = authority.key_generation;
        self.signature.clear();
        self.signature = hex::encode(
            authority
                .signing_key
                .sign(&self.unsigned_bytes()?)
                .to_bytes(),
        );
        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuityCommand {
    Status,
    QuiesceAndExport {
        generation: u64,
        predecessor_sha256: Option<String>,
        topology_sha256: String,
        config_sha256: String,
        deadline_millis: u64,
    },
    ResumeSource {
        handle: SourceCheckpointHandle,
    },
    ReadBundleRange {
        handle: SourceCheckpointHandle,
        ordinal: u32,
        relative_offset: u64,
        length: u64,
    },
    StageTarget {
        stage_id: String,
        root_generation: u64,
        catalog: SignedBundleCatalog,
    },
    WriteTargetChunk {
        handle: TargetStageHandle,
        ordinal: u32,
        bytes_base64: String,
    },
    ValidateTarget {
        handle: TargetStageHandle,
        expected_generation: u64,
        expected_predecessor: Option<String>,
        expected_prefix: u64,
        topology_sha256: String,
        config_sha256: String,
        service_schemas: BTreeMap<String, String>,
    },
    ActivateTarget {
        handle: TargetStageHandle,
        possession: TargetPossessionEvidence,
        cleanup: TargetCleanupPermit,
        decision: Box<MigrationDecisionCertificate>,
    },
    DiscardTarget {
        handle: TargetStageHandle,
        cleanup: TargetCleanupPermit,
    },
}

impl ContinuityCommand {
    pub fn kind(&self) -> ContinuityOperationKind {
        match self {
            Self::Status => ContinuityOperationKind::Status,
            Self::QuiesceAndExport { .. } => ContinuityOperationKind::QuiesceAndExport,
            Self::ResumeSource { .. } => ContinuityOperationKind::ResumeSource,
            Self::ReadBundleRange { .. } => ContinuityOperationKind::ReadBundleRange,
            Self::StageTarget { .. } => ContinuityOperationKind::StageTarget,
            Self::WriteTargetChunk { .. } => ContinuityOperationKind::WriteTargetChunk,
            Self::ValidateTarget { .. } => ContinuityOperationKind::ValidateTarget,
            Self::ActivateTarget { .. } => ContinuityOperationKind::ActivateTarget,
            Self::DiscardTarget { .. } => ContinuityOperationKind::DiscardTarget,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "result", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuityReply {
    Ready,
    Exported {
        quiesce: SourceQuiesceReceipt,
        handle: SourceCheckpointHandle,
    },
    SourceResumed {
        receipt: SourceResumeReceipt,
    },
    BundleRange {
        bytes_base64: String,
    },
    StageCreated {
        handle: TargetStageHandle,
        cleanup: TargetCleanupPermit,
    },
    ChunkWritten,
    TargetValidated {
        possession: TargetPossessionEvidence,
    },
    TargetActivated {
        receipt: TargetActivationReceipt,
    },
    TargetDiscarded {
        receipt: TargetDiscardReceipt,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TargetStageStatus {
    Staging,
    Validated,
    Discarding,
    Discarded,
    Activating,
    Activated,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetStageState {
    schema: String,
    handle: TargetStageHandle,
    cleanup: TargetCleanupPermit,
    catalog: SignedBundleCatalog,
    status: TargetStageStatus,
    received: BTreeMap<u32, String>,
    possession: Option<TargetPossessionEvidence>,
    discard: Option<TargetDiscardReceipt>,
    activation: Option<TargetActivationReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    activation_decision_sha256: Option<String>,
    #[serde(default)]
    cleanup_consumed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ActiveTargetRecord {
    schema: String,
    stage_id: String,
    root_generation: u64,
    catalog_sha256: String,
    decision_sha256: String,
    receipt: TargetActivationReceipt,
}

const ACTIVE_TARGET_SCHEMA: &str = "adl.runtime.kernel_continuity.active_target.v1";

pub struct TargetContinuityCoordinator {
    config: ContinuityControlInitConfig,
    _lock: File,
    _staging_root_handle: File,
    trusted_keys: BTreeMap<(String, u64), VerifyingKey>,
    stages: Mutex<BTreeMap<String, TargetStageState>>,
}

impl TargetContinuityCoordinator {
    pub fn open(
        config: ContinuityControlInitConfig,
        trusted_keys: BTreeMap<(String, u64), VerifyingKey>,
    ) -> Result<Self, ContinuityControlError> {
        fs::create_dir_all(&config.staging_dir)?;
        validate_existing_directory(&config.staging_dir)?;
        fs::create_dir_all(&config.state_dir)?;
        validate_existing_directory(&config.state_dir)?;
        let lock_path = config.state_dir.join("target-writer.lock");
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&lock_path)?;
        lock.try_lock_exclusive().map_err(|error| {
            if error.kind() == std::io::ErrorKind::WouldBlock {
                ContinuityControlError::AlreadyOpen
            } else {
                error.into()
            }
        })?;
        let staging_root_handle = File::open(&config.staging_dir)?;
        let terminal_root = config.state_dir.join("target-results");
        fs::create_dir_all(&terminal_root)?;
        validate_existing_directory(&terminal_root)?;
        let mut stages = BTreeMap::new();
        for entry in fs::read_dir(&terminal_root)? {
            let entry = entry?;
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_file() {
                return Err(ContinuityControlError::UnsafeRoot);
            }
            let state: TargetStageState =
                read_canonical_file(&entry.path(), config.bounds.max_frame_bytes)?;
            if state.schema != STAGE_STATE_SCHEMA
                || !matches!(
                    state.status,
                    TargetStageStatus::Discarded | TargetStageStatus::Activated
                )
            {
                return Err(ContinuityControlError::CorruptStage);
            }
            stages.insert(state.handle.stage_id.clone(), state);
        }
        for entry in fs::read_dir(&config.staging_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_dir() {
                return Err(ContinuityControlError::UnsafeRoot);
            }
            let state_path = entry.path().join("stage.json");
            let state: TargetStageState =
                read_canonical_file(&state_path, config.bounds.max_frame_bytes)?;
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            let expected_stage_name = entry_name.strip_prefix(".discard-").unwrap_or(&entry_name);
            if state.schema != STAGE_STATE_SCHEMA || state.handle.stage_id != expected_stage_name {
                return Err(ContinuityControlError::CorruptStage);
            }
            let mut state = state;
            if entry_name.starts_with(".discard-") || state.status == TargetStageStatus::Discarding
            {
                let receipt = state
                    .discard
                    .clone()
                    .ok_or(ContinuityControlError::CorruptStage)?;
                state.status = TargetStageStatus::Discarded;
                state.discard = Some(receipt);
                write_canonical_file(
                    &terminal_root.join(format!("{}.json", state.handle.stage_id)),
                    &state,
                )?;
                fs::remove_dir_all(entry.path())?;
                sync_dir(&config.staging_dir)?;
            } else if state.status == TargetStageStatus::Activating {
                let decision_sha256 = state
                    .activation_decision_sha256
                    .clone()
                    .ok_or(ContinuityControlError::CorruptStage)?;
                let receipt = state
                    .activation
                    .clone()
                    .ok_or(ContinuityControlError::CorruptStage)?;
                let active = ActiveTargetRecord {
                    schema: ACTIVE_TARGET_SCHEMA.to_owned(),
                    stage_id: state.handle.stage_id.clone(),
                    root_generation: state.handle.root_generation,
                    catalog_sha256: state.handle.catalog_sha256.clone(),
                    decision_sha256,
                    receipt,
                };
                write_canonical_file(&config.state_dir.join("active-target.json"), &active)?;
                state.status = TargetStageStatus::Activated;
                write_canonical_file(&entry.path().join("stage.json"), &state)?;
                write_canonical_file(
                    &terminal_root.join(format!("{}.json", state.handle.stage_id)),
                    &state,
                )?;
            }
            if let Some(terminal) = stages.get(&state.handle.stage_id) {
                if terminal != &state && terminal.status != TargetStageStatus::Discarded {
                    return Err(ContinuityControlError::CorruptStage);
                }
            } else {
                stages.insert(state.handle.stage_id.clone(), state);
            }
        }
        Ok(Self {
            config,
            _lock: lock,
            _staging_root_handle: staging_root_handle,
            trusted_keys,
            stages: Mutex::new(stages),
        })
    }

    pub fn create_stage(
        &self,
        stage_id: &str,
        root_generation: u64,
        catalog: SignedBundleCatalog,
    ) -> Result<StageCreated, ContinuityControlError> {
        self.verify_staging_root_anchor()?;
        if !safe_identity(stage_id) || root_generation == 0 {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        catalog.verify(&self.trusted_keys)?;
        validate_catalog(&catalog, &self.config.bounds)?;
        let catalog_sha256 = catalog.digest()?;
        let handle = TargetStageHandle {
            stage_id: stage_id.to_owned(),
            root_generation,
            catalog_sha256: catalog_sha256.clone(),
        };
        let cleanup = TargetCleanupPermit {
            stage_id: stage_id.to_owned(),
            root_generation,
            channel_epoch: self.config.channel_epoch,
            catalog_sha256,
            cleanup_id: sha256(format!("cleanup:{stage_id}:{root_generation}").as_bytes()),
        };
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| ContinuityControlError::CorruptStage)?;
        if let Some(existing) = stages.get(stage_id) {
            if existing.handle == handle
                && existing.cleanup == cleanup
                && existing.catalog == catalog
                && matches!(
                    existing.status,
                    TargetStageStatus::Staging | TargetStageStatus::Validated
                )
            {
                return Ok(StageCreated { handle, cleanup });
            }
            return Err(ContinuityControlError::ConflictingRetry);
        }
        let path = self.stage_path(stage_id);
        if path.exists() {
            return Err(ContinuityControlError::CorruptStage);
        }
        fs::create_dir(&path)?;
        let state = TargetStageState {
            schema: STAGE_STATE_SCHEMA.to_owned(),
            handle: handle.clone(),
            cleanup: cleanup.clone(),
            catalog,
            status: TargetStageStatus::Staging,
            received: BTreeMap::new(),
            possession: None,
            discard: None,
            activation: None,
            activation_decision_sha256: None,
            cleanup_consumed: false,
        };
        write_canonical_file(&path.join("stage.json"), &state)?;
        sync_dir(&self.config.staging_dir)?;
        stages.insert(stage_id.to_owned(), state);
        Ok(StageCreated { handle, cleanup })
    }

    pub fn write_entry(
        &self,
        handle: &TargetStageHandle,
        ordinal: u32,
        bytes: &[u8],
    ) -> Result<(), ContinuityControlError> {
        self.verify_staging_root_anchor()?;
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| ContinuityControlError::CorruptStage)?;
        let state = stages
            .get_mut(&handle.stage_id)
            .ok_or(ContinuityControlError::UnknownStage)?;
        validate_handle(state, handle)?;
        if state.status != TargetStageStatus::Staging {
            return Err(ContinuityControlError::StageState);
        }
        let entry = state
            .catalog
            .entries
            .iter()
            .find(|entry| entry.ordinal == ordinal)
            .ok_or(ContinuityControlError::CatalogMismatch)?;
        if bytes.len() as u64 != entry.bytes || sha256(bytes) != entry.sha256 {
            return Err(ContinuityControlError::ContentMismatch);
        }
        if state.received.contains_key(&ordinal) {
            if state.received.get(&ordinal) == Some(&entry.sha256) {
                return Ok(());
            }
            return Err(ContinuityControlError::ConflictingRetry);
        }
        let path = self.stage_path(&handle.stage_id).join(&entry.file);
        validate_leaf_name(&entry.file)?;
        write_new_synced(&path, bytes)?;
        state.received.insert(ordinal, entry.sha256.clone());
        self.persist_stage(state)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate_stage(
        &self,
        handle: &TargetStageHandle,
        expected_generation: u64,
        expected_predecessor: Option<&str>,
        expected_prefix: u64,
        topology_sha256: &str,
        config_sha256: &str,
        service_schemas: &BTreeMap<String, String>,
    ) -> Result<TargetPossessionEvidence, ContinuityControlError> {
        self.verify_staging_root_anchor()?;
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| ContinuityControlError::CorruptStage)?;
        let state = stages
            .get_mut(&handle.stage_id)
            .ok_or(ContinuityControlError::UnknownStage)?;
        validate_handle(state, handle)?;
        if state.status == TargetStageStatus::Validated {
            return state
                .possession
                .clone()
                .ok_or(ContinuityControlError::CorruptStage);
        }
        if state.status != TargetStageStatus::Staging
            || state.catalog.generation != expected_generation
            || state.catalog.predecessor_sha256.as_deref() != expected_predecessor
            || state.catalog.accepted_prefix != expected_prefix
            || state.catalog.topology_sha256 != topology_sha256
            || state.catalog.config_sha256 != config_sha256
        {
            return Err(ContinuityControlError::CatalogMismatch);
        }
        let expected = state
            .catalog
            .entries
            .iter()
            .map(|entry| (entry.service.clone(), entry.schema.clone()))
            .collect::<BTreeMap<_, _>>();
        if &expected != service_schemas || state.received.len() != state.catalog.entries.len() {
            return Err(ContinuityControlError::CatalogMismatch);
        }
        for entry in &state.catalog.entries {
            let path = self.stage_path(&handle.stage_id).join(&entry.file);
            validate_regular_single_link_file(&path, entry.bytes)?;
            let bytes = read_bounded(&path, entry.bytes)?;
            if sha256(&bytes) != entry.sha256 {
                return Err(ContinuityControlError::ContentMismatch);
            }
        }
        let evidence = TargetPossessionEvidence::new(format!(
            "possession:{}:{}",
            handle.stage_id, handle.catalog_sha256
        ));
        state.status = TargetStageStatus::Validated;
        state.possession = Some(evidence.clone());
        self.persist_stage(state)?;
        Ok(evidence)
    }

    pub fn discard(
        &self,
        handle: &TargetStageHandle,
        cleanup: &TargetCleanupPermit,
    ) -> Result<TargetDiscardReceipt, ContinuityControlError> {
        self.verify_staging_root_anchor()?;
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| ContinuityControlError::CorruptStage)?;
        let state = stages
            .get_mut(&handle.stage_id)
            .ok_or(ContinuityControlError::UnknownStage)?;
        validate_handle(state, handle)?;
        if &state.cleanup != cleanup {
            return Err(ContinuityControlError::CleanupAuthority);
        }
        match state.status {
            TargetStageStatus::Discarded => {
                return state
                    .discard
                    .clone()
                    .ok_or(ContinuityControlError::CorruptStage)
            }
            TargetStageStatus::Activated | TargetStageStatus::Activating => {
                return Err(ContinuityControlError::Activated)
            }
            TargetStageStatus::Discarding => return Err(ContinuityControlError::RecoveryRequired),
            TargetStageStatus::Staging | TargetStageStatus::Validated => {}
        }
        let receipt = TargetDiscardReceipt::new(format!("discarded:{}", handle.stage_id));
        state.status = TargetStageStatus::Discarding;
        state.discard = Some(receipt.clone());
        self.persist_stage(state)?;
        let path = self.stage_path(&handle.stage_id);
        let tombstone = self
            .config
            .staging_dir
            .join(format!(".discard-{}", handle.stage_id));
        if tombstone.exists() {
            return Err(ContinuityControlError::RecoveryRequired);
        }
        fs::rename(&path, &tombstone)?;
        sync_dir(&self.config.staging_dir)?;
        state.status = TargetStageStatus::Discarded;
        self.persist_terminal(state)?;
        fs::remove_dir_all(&tombstone)?;
        sync_dir(&self.config.staging_dir)?;
        Ok(receipt)
    }

    pub fn activate(
        &self,
        handle: &TargetStageHandle,
        possession: &TargetPossessionEvidence,
        cleanup: &TargetCleanupPermit,
        certificate: &MigrationDecisionCertificate,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        self.verify_staging_root_anchor()?;
        let decision = self.verify_204_decision(certificate)?;
        let decision_sha256 = sha256(&certificate.unsigned_bytes()?);
        let mut stages = self
            .stages
            .lock()
            .map_err(|_| ContinuityControlError::CorruptStage)?;
        let state = stages
            .get_mut(&handle.stage_id)
            .ok_or(ContinuityControlError::UnknownStage)?;
        validate_handle(state, handle)?;
        if state.status == TargetStageStatus::Activated {
            if state.activation_decision_sha256.as_deref() != Some(&decision_sha256)
                || state.possession.as_ref() != Some(possession)
                || &state.cleanup != cleanup
                || !state.cleanup_consumed
            {
                return Err(ContinuityControlError::ConflictingRetry);
            }
            return state
                .activation
                .clone()
                .ok_or(ContinuityControlError::CorruptStage);
        }
        if state.status != TargetStageStatus::Validated
            || state.possession.as_ref() != Some(possession)
            || &state.cleanup != cleanup
            || decision.stage_id != handle.stage_id
            || decision.possession_sha256 != possession.digest()
            || decision.trust_domain != self.config.trust_domain
            || decision.polis != self.config.polis
            || decision.target_node != self.config.target_node
            || decision.channel_epoch != self.config.channel_epoch
        {
            return Err(ContinuityControlError::ActivationDecision);
        }
        for digest in [
            &decision.route_cut_sha256,
            &decision.membership_cut_sha256,
            &decision.certificate_cut_sha256,
            &decision.boot_cut_sha256,
            &decision.lineage_sha256,
        ] {
            validate_digest(digest)?;
        }
        let receipt = TargetActivationReceipt::new(format!(
            "activated:{}:{}",
            handle.stage_id, decision.decision_id
        ));
        state.status = TargetStageStatus::Activating;
        state.activation = Some(receipt.clone());
        state.activation_decision_sha256 = Some(decision_sha256.clone());
        state.cleanup_consumed = true;
        self.persist_stage(state)?;
        let active = ActiveTargetRecord {
            schema: ACTIVE_TARGET_SCHEMA.to_owned(),
            stage_id: handle.stage_id.clone(),
            root_generation: handle.root_generation,
            catalog_sha256: handle.catalog_sha256.clone(),
            decision_sha256,
            receipt: receipt.clone(),
        };
        write_canonical_file(&self.config.state_dir.join("active-target.json"), &active)?;
        state.status = TargetStageStatus::Activated;
        self.persist_stage(state)?;
        self.persist_terminal(state)?;
        Ok(receipt)
    }

    fn verify_204_decision(
        &self,
        certificate: &MigrationDecisionCertificate,
    ) -> Result<FinalizedMigrationDecision, ContinuityControlError> {
        if !safe_identity(&certificate.decision_id)
            || !safe_identity(&certificate.stage_id)
            || !safe_identity(&certificate.authority_key_id)
            || certificate.authority_key_generation == 0
        {
            return Err(ContinuityControlError::ActivationDecision);
        }
        for digest in [
            &certificate.possession_sha256,
            &certificate.route_cut_sha256,
            &certificate.membership_cut_sha256,
            &certificate.certificate_cut_sha256,
            &certificate.boot_cut_sha256,
            &certificate.lineage_sha256,
        ] {
            validate_digest(digest)?;
        }
        if certificate.trust_domain != self.config.trust_domain
            || certificate.polis != self.config.polis
            || certificate.target_node != self.config.target_node
            || certificate.channel_epoch != self.config.channel_epoch
        {
            return Err(ContinuityControlError::ActivationDecision);
        }
        let key = self
            .trusted_keys
            .get(&(
                certificate.authority_key_id.clone(),
                certificate.authority_key_generation,
            ))
            .ok_or(ContinuityControlError::ActivationDecision)?;
        let signature_bytes = hex::decode(&certificate.signature)
            .map_err(|_| ContinuityControlError::ActivationDecision)?;
        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|_| ContinuityControlError::ActivationDecision)?;
        key.verify(&certificate.unsigned_bytes()?, &signature)
            .map_err(|_| ContinuityControlError::ActivationDecision)?;
        Ok(FinalizedMigrationDecision {
            decision_id: certificate.decision_id.clone(),
            stage_id: certificate.stage_id.clone(),
            possession_sha256: certificate.possession_sha256.clone(),
            trust_domain: certificate.trust_domain.clone(),
            polis: certificate.polis.clone(),
            target_node: certificate.target_node.clone(),
            channel_epoch: certificate.channel_epoch,
            route_cut_sha256: certificate.route_cut_sha256.clone(),
            membership_cut_sha256: certificate.membership_cut_sha256.clone(),
            certificate_cut_sha256: certificate.certificate_cut_sha256.clone(),
            boot_cut_sha256: certificate.boot_cut_sha256.clone(),
            lineage_sha256: certificate.lineage_sha256.clone(),
        })
    }

    fn stage_path(&self, stage_id: &str) -> PathBuf {
        self.config.staging_dir.join(stage_id)
    }

    fn persist_stage(&self, state: &TargetStageState) -> Result<(), ContinuityControlError> {
        let path = self.stage_path(&state.handle.stage_id);
        write_canonical_file(&path.join("stage.json"), state)?;
        sync_dir(&path)
    }

    fn persist_terminal(&self, state: &TargetStageState) -> Result<(), ContinuityControlError> {
        let terminal_root = self.config.state_dir.join("target-results");
        fs::create_dir_all(&terminal_root)?;
        write_canonical_file(
            &terminal_root.join(format!("{}.json", state.handle.stage_id)),
            state,
        )?;
        sync_dir(&terminal_root)
    }

    fn verify_staging_root_anchor(&self) -> Result<(), ContinuityControlError> {
        verify_directory_anchor(&self.config.staging_dir, &self._staging_root_handle)
    }
}

impl Drop for TargetContinuityCoordinator {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self._lock);
    }
}

pub struct ContinuityControlService {
    config: ContinuityControlInitConfig,
    journal: Mutex<DurableContinuityJournal>,
    source: Arc<SourceContinuityEffectPort>,
    target: Arc<TargetContinuityCoordinator>,
    effect_slots: Arc<tokio::sync::Semaphore>,
}

impl ContinuityControlService {
    pub fn new(
        config: ContinuityControlInitConfig,
        journal: DurableContinuityJournal,
        source: Arc<SourceContinuityEffectPort>,
        target: Arc<TargetContinuityCoordinator>,
    ) -> Self {
        let effect_slots = Arc::new(tokio::sync::Semaphore::new(config.bounds.max_open_handles));
        Self {
            config,
            journal: Mutex::new(journal),
            source,
            target,
            effect_slots,
        }
    }

    pub(crate) async fn dispatch(
        &self,
        envelope: ContinuityEnvelope,
        exporter: &[u8],
        now_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<ContinuityResponse, ContinuityControlError> {
        let _effect_slot = tokio::select! {
            _ = cancellation.cancelled() => return Err(ContinuityControlError::Cancelled),
            permit = self.effect_slots.acquire() => permit.map_err(|_| ContinuityControlError::Cancelled)?,
        };
        check_effect_window(envelope.operation.deadline_unix_millis, cancellation)?;
        let begin = self
            .journal
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .begin(&self.config, &envelope, exporter, now_unix_millis)?;
        let request_sha256 = match begin {
            BeginOperation::Retry(response) => return Ok(*response),
            BeginOperation::New { operation_sha256 }
            | BeginOperation::Reconcile { operation_sha256 } => operation_sha256,
        };
        let operation_id = envelope.operation.operation_id.clone();
        let accepted_prefix = envelope.operation.accepted_prefix;
        let effect_deadline = envelope.operation.deadline_unix_millis;
        let result = match envelope.command {
            ContinuityCommand::Status => ContinuityReply::Ready,
            ContinuityCommand::QuiesceAndExport {
                generation,
                predecessor_sha256,
                topology_sha256,
                config_sha256,
                deadline_millis,
            } => {
                if deadline_millis == 0 {
                    return Err(ContinuityControlError::Deadline);
                }
                let (quiesce, handle) = self
                    .source
                    .quiesce_and_export(
                        generation,
                        predecessor_sha256,
                        accepted_prefix,
                        topology_sha256,
                        config_sha256,
                        std::time::Duration::from_millis(deadline_millis),
                        cancellation,
                    )
                    .await?;
                ContinuityReply::Exported { quiesce, handle }
            }
            ContinuityCommand::ResumeSource { handle } => ContinuityReply::SourceResumed {
                receipt: {
                    check_effect_window(effect_deadline, cancellation)?;
                    self.source.resume_source(&handle)?
                },
            },
            ContinuityCommand::ReadBundleRange {
                handle,
                ordinal,
                relative_offset,
                length,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                let port = self.source.bundle_source(&handle)?;
                let bytes = port.read_entry_range(ordinal, relative_offset, length)?;
                ContinuityReply::BundleRange {
                    bytes_base64: BASE64.encode(bytes),
                }
            }
            ContinuityCommand::StageTarget {
                stage_id,
                root_generation,
                catalog,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                let created = self
                    .target
                    .create_stage(&stage_id, root_generation, catalog)?;
                ContinuityReply::StageCreated {
                    handle: created.handle,
                    cleanup: created.cleanup,
                }
            }
            ContinuityCommand::WriteTargetChunk {
                handle,
                ordinal,
                bytes_base64,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                let bytes = BASE64
                    .decode(bytes_base64)
                    .map_err(|_| ContinuityControlError::ContentMismatch)?;
                self.target.write_entry(&handle, ordinal, &bytes)?;
                ContinuityReply::ChunkWritten
            }
            ContinuityCommand::ValidateTarget {
                handle,
                expected_generation,
                expected_predecessor,
                expected_prefix,
                topology_sha256,
                config_sha256,
                service_schemas,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                let possession = self.target.validate_stage(
                    &handle,
                    expected_generation,
                    expected_predecessor.as_deref(),
                    expected_prefix,
                    &topology_sha256,
                    &config_sha256,
                    &service_schemas,
                )?;
                ContinuityReply::TargetValidated { possession }
            }
            ContinuityCommand::ActivateTarget {
                handle,
                possession,
                cleanup,
                decision,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                let receipt = self
                    .target
                    .activate(&handle, &possession, &cleanup, &decision)?;
                ContinuityReply::TargetActivated { receipt }
            }
            ContinuityCommand::DiscardTarget { handle, cleanup } => {
                check_effect_window(effect_deadline, cancellation)?;
                let receipt = self.target.discard(&handle, &cleanup)?;
                ContinuityReply::TargetDiscarded { receipt }
            }
        };
        let receipt_sha256 = sha256(
            &serde_jcs::to_vec(&result)
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
        );
        let response = ContinuityResponse::new(
            request_sha256,
            ContinuityResultState::Completed,
            receipt_sha256,
            result,
        )?;
        self.journal
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .complete(&operation_id, response)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContinuityWireReply {
    Completed { response: Box<ContinuityResponse> },
    Denied { code: String },
}

pub async fn serve_private_continuity_listener(
    listener: tokio::net::TcpListener,
    tls_config: Arc<rustls::ServerConfig>,
    service: Arc<ContinuityControlService>,
    shutdown: CancellationToken,
) -> Result<(), ContinuityControlError> {
    let acceptor = tokio_rustls::TlsAcceptor::from(tls_config);
    let connection_slots = Arc::new(tokio::sync::Semaphore::new(
        service.config.bounds.max_open_handles,
    ));
    loop {
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let permit = tokio::select! {
                    _ = shutdown.cancelled() => return Ok(()),
                    permit = Arc::clone(&connection_slots).acquire_owned() => {
                        permit.map_err(|_| ContinuityControlError::Cancelled)?
                    }
                };
                let acceptor = acceptor.clone();
                let service = service.clone();
                let connection_shutdown = shutdown.child_token();
                tokio::spawn(async move {
                    let _permit = permit;
                    let Ok(stream) = acceptor.accept(stream).await else { return; };
                    let _ = serve_private_connection(stream, service, connection_shutdown).await;
                });
            }
        }
    }
}

fn check_effect_window(
    deadline_unix_millis: u64,
    cancellation: &CancellationToken,
) -> Result<(), ContinuityControlError> {
    if cancellation.is_cancelled() {
        return Err(ContinuityControlError::Cancelled);
    }
    if unix_millis()? > deadline_unix_millis {
        return Err(ContinuityControlError::Deadline);
    }
    Ok(())
}

async fn serve_private_connection(
    mut stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
    service: Arc<ContinuityControlService>,
    shutdown: CancellationToken,
) -> Result<(), ContinuityControlError> {
    let (leaf_spki_sha256, exporter) = {
        let (_, connection) = stream.get_ref();
        let certificate = connection
            .peer_certificates()
            .and_then(|chain| chain.first())
            .ok_or(ContinuityControlError::CertificateIdentity)?;
        let spki = certificate_spki_sha256(certificate.as_ref())?;
        let mut exporter = [0_u8; 32];
        connection
            .export_keying_material(&mut exporter, EXPORTER_LABEL, None)
            .map_err(|_| ContinuityControlError::ExporterBinding)?;
        (spki, exporter)
    };
    if leaf_spki_sha256 != service.config.tls.guardian_spki_sha256
        && service
            .config
            .tls
            .successor
            .as_ref()
            .is_none_or(|successor| leaf_spki_sha256 != successor.successor_spki_sha256)
    {
        return Err(ContinuityControlError::CertificateIdentity);
    }
    loop {
        let mut length = [0_u8; 4];
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            result = stream.read_exact(&mut length) => {
                if let Err(error) = result {
                    if error.kind() == std::io::ErrorKind::UnexpectedEof { return Ok(()); }
                    return Err(error.into());
                }
            }
        }
        let length = u32::from_be_bytes(length) as usize;
        if length == 0 || length > service.config.bounds.max_frame_bytes {
            return Err(ContinuityControlError::FrameBounds);
        }
        let mut bytes = vec![0_u8; length];
        stream.read_exact(&mut bytes).await?;
        let envelope: ContinuityEnvelope =
            decode_canonical(&bytes, service.config.bounds.max_frame_bytes)?;
        let reply = if envelope.leaf_spki_sha256 != leaf_spki_sha256 {
            ContinuityWireReply::Denied {
                code: "certificate_identity".to_owned(),
            }
        } else {
            match service
                .dispatch(envelope, &exporter, unix_millis()?, &shutdown)
                .await
            {
                Ok(response) => ContinuityWireReply::Completed {
                    response: Box::new(response),
                },
                Err(error) => ContinuityWireReply::Denied {
                    code: error.code().to_owned(),
                },
            }
        };
        let bytes = serde_jcs::to_vec(&reply)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        if bytes.len() > service.config.bounds.max_frame_bytes {
            return Err(ContinuityControlError::FrameBounds);
        }
        stream
            .write_all(&(bytes.len() as u32).to_be_bytes())
            .await?;
        stream.write_all(&bytes).await?;
        stream.flush().await?;
    }
}

fn validate_handle(
    state: &TargetStageState,
    handle: &TargetStageHandle,
) -> Result<(), ContinuityControlError> {
    if &state.handle != handle {
        Err(ContinuityControlError::ConflictingRetry)
    } else {
        Ok(())
    }
}

fn validate_catalog(
    catalog: &SignedBundleCatalog,
    bounds: &ContinuityControlBounds,
) -> Result<(), ContinuityControlError> {
    if catalog.generation == 0
        || catalog.entries.is_empty()
        || catalog.entries.len() > bounds.max_services
        || catalog.accepted_prefix == u64::MAX
    {
        return Err(ContinuityControlError::CatalogMismatch);
    }
    for digest in [&catalog.topology_sha256, &catalog.config_sha256] {
        validate_digest(digest)?;
    }
    if let Some(predecessor) = &catalog.predecessor_sha256 {
        validate_digest(predecessor)?;
    }
    let mut services = BTreeSet::new();
    let mut files = BTreeSet::new();
    let mut total = 0_u64;
    let mut offset = 0_u64;
    for (index, entry) in catalog.entries.iter().enumerate() {
        if entry.ordinal != (index as u32).saturating_add(1)
            || !safe_identity(&entry.service)
            || !safe_identity(&entry.schema)
            || !services.insert(entry.service.clone())
            || !files.insert(entry.file.clone())
            || entry.offset != offset
            || entry.bytes == 0
            || entry.bytes > bounds.max_blob_bytes
        {
            return Err(ContinuityControlError::CatalogMismatch);
        }
        validate_leaf_name(&entry.file)?;
        validate_digest(&entry.sha256)?;
        total = total
            .checked_add(entry.bytes)
            .ok_or(ContinuityControlError::InvalidBounds)?;
        offset = offset
            .checked_add(entry.bytes)
            .ok_or(ContinuityControlError::InvalidBounds)?;
    }
    if total > bounds.max_total_bytes {
        return Err(ContinuityControlError::InvalidBounds);
    }
    Ok(())
}

pub fn verify_spki_generation(
    certificate_der: &[u8],
    expected_spki_sha256: &str,
) -> Result<(), ContinuityControlError> {
    let (remaining, certificate) = x509_parser::parse_x509_certificate(certificate_der)
        .map_err(|_| ContinuityControlError::CertificateIdentity)?;
    if !remaining.is_empty() || sha256(certificate.public_key().raw) != expected_spki_sha256 {
        return Err(ContinuityControlError::CertificateIdentity);
    }
    Ok(())
}

fn certificate_spki_sha256(certificate_der: &[u8]) -> Result<String, ContinuityControlError> {
    let (remaining, certificate) = x509_parser::parse_x509_certificate(certificate_der)
        .map_err(|_| ContinuityControlError::CertificateIdentity)?;
    if !remaining.is_empty() {
        return Err(ContinuityControlError::CertificateIdentity);
    }
    Ok(sha256(certificate.public_key().raw))
}

fn unix_millis() -> Result<u64, ContinuityControlError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ContinuityControlError::Clock)
        .and_then(|duration| {
            duration
                .as_millis()
                .try_into()
                .map_err(|_| ContinuityControlError::Clock)
        })
}

pub fn exporter_digest(exporter: &[u8]) -> String {
    sha256(exporter)
}

pub fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn safe_identity(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
}

fn validate_digest(value: &str) -> Result<(), ContinuityControlError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(ContinuityControlError::InvalidDigest)
    }
}

fn validate_leaf_name(value: &str) -> Result<(), ContinuityControlError> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path.components().count() != 1
        || !matches!(path.components().next(), Some(Component::Normal(_)))
    {
        return Err(ContinuityControlError::UnsafePath);
    }
    Ok(())
}

fn validate_private_root(state_root: &Path, path: &Path) -> Result<(), ContinuityControlError> {
    if !state_root.is_absolute()
        || !path.is_absolute()
        || !path.starts_with(state_root)
        || path == state_root
    {
        return Err(ContinuityControlError::UnsafeRoot);
    }
    validate_absolute_nonsymlink_path(path)
}

fn validate_absolute_nonsymlink_path(path: &Path) -> Result<(), ContinuityControlError> {
    if !path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(ContinuityControlError::UnsafePath);
    }
    let mut current = PathBuf::new();
    for component in path.components() {
        current.push(component.as_os_str());
        if let Ok(metadata) = fs::symlink_metadata(&current) {
            if metadata.file_type().is_symlink() {
                return Err(ContinuityControlError::UnsafePath);
            }
        }
    }
    Ok(())
}

fn roots_overlap(left: &Path, right: &Path) -> bool {
    left.starts_with(right) || right.starts_with(left)
}

fn validate_existing_directory(path: &Path) -> Result<(), ContinuityControlError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ContinuityControlError::UnsafeRoot);
    }
    Ok(())
}

fn verify_directory_anchor(path: &Path, handle: &File) -> Result<(), ContinuityControlError> {
    let path_metadata = fs::symlink_metadata(path)?;
    let handle_metadata = handle.metadata()?;
    if path_metadata.file_type().is_symlink()
        || !path_metadata.is_dir()
        || !handle_metadata.is_dir()
    {
        return Err(ContinuityControlError::UnsafeRoot);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if path_metadata.dev() != handle_metadata.dev()
            || path_metadata.ino() != handle_metadata.ino()
            || path_metadata.nlink() == 0
        {
            return Err(ContinuityControlError::UnsafeRoot);
        }
    }
    Ok(())
}

fn validate_regular_single_link_file(
    path: &Path,
    expected_bytes: u64,
) -> Result<(), ContinuityControlError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() != expected_bytes
    {
        return Err(ContinuityControlError::UnsafePath);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(ContinuityControlError::UnsafePath);
        }
    }
    Ok(())
}

fn read_bounded(path: &Path, expected_bytes: u64) -> Result<Vec<u8>, ContinuityControlError> {
    let limit = expected_bytes
        .checked_add(1)
        .ok_or(ContinuityControlError::InvalidBounds)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() != expected_bytes {
        return Err(ContinuityControlError::UnsafePath);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(ContinuityControlError::UnsafePath);
        }
    }
    let mut bytes = Vec::new();
    file.take(limit).read_to_end(&mut bytes)?;
    if bytes.len() as u64 != expected_bytes {
        return Err(ContinuityControlError::ContentMismatch);
    }
    Ok(bytes)
}

fn write_new_synced(path: &Path, bytes: &[u8]) -> Result<(), ContinuityControlError> {
    validate_absolute_nonsymlink_path(path)?;
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn write_canonical_file<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), ContinuityControlError> {
    validate_absolute_nonsymlink_path(path)?;
    let bytes = serde_jcs::to_vec(value)
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
    let pending = path.with_extension("pending");
    if pending.exists() {
        let metadata = fs::symlink_metadata(&pending)?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(ContinuityControlError::UnsafePath);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.nlink() != 1 {
                return Err(ContinuityControlError::UnsafePath);
            }
        }
        fs::remove_file(&pending)?;
    }
    {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&pending)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    fs::rename(&pending, path)?;
    sync_dir(path.parent().ok_or(ContinuityControlError::UnsafePath)?)
}

fn read_canonical_file<T: DeserializeOwned + Serialize>(
    path: &Path,
    max_bytes: usize,
) -> Result<T, ContinuityControlError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > max_bytes as u64
    {
        return Err(ContinuityControlError::UnsafePath);
    }
    let bytes = read_bounded(path, metadata.len())?;
    decode_canonical(&bytes, max_bytes)
}

fn sync_dir(path: &Path) -> Result<(), ContinuityControlError> {
    File::open(path)?.sync_all().map_err(Into::into)
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |value, (left, right)| value | (left ^ right))
        == 0
}

#[derive(Debug, Error)]
pub enum ContinuityControlError {
    #[error("continuity private address is not a unique nonzero loopback endpoint")]
    InvalidAddress,
    #[error("continuity identities are invalid or overlap")]
    InvalidIdentity,
    #[error("continuity bounds are zero, inconsistent, or exceeded")]
    InvalidBounds,
    #[error("continuity private root is unsafe, replaceable, overlapping, or symlinked")]
    UnsafeRoot,
    #[error("continuity path is caller-controlled, traversing, replaced, or symlinked")]
    UnsafePath,
    #[error("continuity digest is not canonical lower-case SHA-256")]
    InvalidDigest,
    #[error("certificate succession is skipped, ambiguous, or inconsistent")]
    InvalidSuccession,
    #[error("TLS certificate identity does not match the configured SPKI")]
    CertificateIdentity,
    #[error("only the configured TLS-authenticated channel identity may perform continuity work")]
    ChannelIdentity,
    #[error("TLS exporter binding does not match this session")]
    ExporterBinding,
    #[error("certificate generation is stale or not in the persisted succession lineage")]
    StaleCertificate,
    #[error("operation deadline elapsed before effect")]
    Deadline,
    #[error("system clock cannot represent the continuity deadline domain")]
    Clock,
    #[error("operation was cancelled before effect")]
    Cancelled,
    #[error("operation sequence is stale, future, or reordered")]
    Sequence,
    #[error("operation id was reused with conflicting bytes")]
    ConflictingRetry,
    #[error("accepted operation requires restart reconciliation before reply")]
    ReconcileRequired,
    #[error("durable continuity journal is corrupt")]
    CorruptJournal,
    #[error("durable continuity journal capacity is exhausted")]
    JournalCapacity,
    #[error("durable continuity namespace is already open")]
    AlreadyOpen,
    #[error("operation is unknown")]
    UnknownOperation,
    #[error("canonical frame is empty or exceeds its bound")]
    FrameBounds,
    #[error("JSON is not the exact canonical RFC 8785 encoding")]
    NonCanonical,
    #[error("canonical encoding failed: {0}")]
    Encoding(String),
    #[error("live continuity participant registry is incomplete or synthetic")]
    ParticipantRegistry,
    #[error("live participant quiesce or rollback failed")]
    Quiesce,
    #[error("live accepted prefix does not match the committed operation prefix")]
    AcceptedPrefix,
    #[error("continuity effect requires durable recovery before service may resume")]
    RecoveryRequired,
    #[error("signed bundle catalog signature is invalid")]
    ManifestSignature,
    #[error("signed bundle catalog does not match expected entry order or bindings")]
    CatalogMismatch,
    #[error("target bytes do not match the exact signed entry")]
    ContentMismatch,
    #[error("target stage is unknown")]
    UnknownStage,
    #[error("target stage state is corrupt")]
    CorruptStage,
    #[error("target stage cannot perform this transition")]
    StageState,
    #[error("cleanup permit does not authorize this exact target stage")]
    CleanupAuthority,
    #[error("activation requires the exact finalized #204 decision and bindings")]
    ActivationDecision,
    #[error("activated target cannot be discarded")]
    Activated,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ContinuityControlError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidAddress => "invalid_address",
            Self::InvalidIdentity => "invalid_identity",
            Self::InvalidBounds => "invalid_bounds",
            Self::UnsafeRoot => "unsafe_root",
            Self::UnsafePath => "unsafe_path",
            Self::InvalidDigest => "invalid_digest",
            Self::InvalidSuccession => "invalid_succession",
            Self::CertificateIdentity => "certificate_identity",
            Self::ChannelIdentity => "channel_identity",
            Self::ExporterBinding => "exporter_binding",
            Self::StaleCertificate => "stale_certificate",
            Self::Deadline => "deadline",
            Self::Clock => "clock",
            Self::Cancelled => "cancelled",
            Self::Sequence => "sequence",
            Self::ConflictingRetry => "conflicting_retry",
            Self::ReconcileRequired => "reconcile_required",
            Self::CorruptJournal => "corrupt_journal",
            Self::JournalCapacity => "journal_capacity",
            Self::AlreadyOpen => "already_open",
            Self::UnknownOperation => "unknown_operation",
            Self::FrameBounds => "frame_bounds",
            Self::NonCanonical => "noncanonical",
            Self::Encoding(_) => "encoding",
            Self::ParticipantRegistry => "participant_registry",
            Self::Quiesce => "quiesce",
            Self::AcceptedPrefix => "accepted_prefix",
            Self::RecoveryRequired => "recovery_required",
            Self::ManifestSignature => "manifest_signature",
            Self::CatalogMismatch => "catalog_mismatch",
            Self::ContentMismatch => "content_mismatch",
            Self::UnknownStage => "unknown_stage",
            Self::CorruptStage => "corrupt_stage",
            Self::StageState => "stage_state",
            Self::CleanupAuthority => "cleanup_authority",
            Self::ActivationDecision => "activation_decision",
            Self::Activated => "activated",
            Self::Io(_) => "io",
        }
    }
}
