//! Production-only Guardian to kernel continuity control plane.
//!
//! Channel authentication is supplied by TLS 1.3 mutual authentication.  The
//! values in this module are durable replay/effect bindings, not a second
//! application credential.  Constructors that can mint effect capabilities
//! remain inside the kernel crate.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    net::SocketAddr,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};

#[cfg(not(unix))]
use std::fs::OpenOptions;

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
pub const STAGE_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.stage.v2";
pub const EXPORTER_LABEL: &[u8] = b"EXPORTER-ADL-Guardian-Kernel-Continuity-v1";
pub const PRIVATE_ALPN: &[&[u8]] = &[b"adl-continuity/1"];
const REQUIRED_PARTICIPANTS: [&str; 5] = [
    "admission",
    "accepted_prefix",
    "reasoning_mutation",
    "governance",
    "operation_state",
];
const PRIVATE_CONTROL_IO_CAP: std::time::Duration = std::time::Duration::from_secs(5);

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
        let root_handle = File::open(&config.state_dir)?;
        #[cfg(unix)]
        let lock = open_file_at(
            &root_handle,
            "writer.lock",
            libc::O_CREAT | libc::O_RDWR,
            0o600,
        )?;
        #[cfg(not(unix))]
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(config.state_dir.join("writer.lock"))?;
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
        #[cfg(unix)]
        let existing_state = match read_canonical_at(
            &root_handle,
            "control-state.json",
            config.bounds.max_frame_bytes,
        ) {
            Ok(state) => Some(state),
            Err(ContinuityControlError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(error) => return Err(error),
        };
        #[cfg(not(unix))]
        let existing_state = {
            let state_path = config.state_dir.join("control-state.json");
            if state_path.exists() {
                Some(read_canonical_file(
                    &state_path,
                    config.bounds.max_frame_bytes,
                )?)
            } else {
                None
            }
        };
        let state = if let Some(state) = existing_state {
            state
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
            #[cfg(unix)]
            write_canonical_at(&root_handle, "control-state.json", &value)?;
            #[cfg(not(unix))]
            write_canonical_file(&config.state_dir.join("control-state.json"), &value)?;
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
        // Retirement is durable channel state, not a derived in-memory hint.
        // Refresh and persist it before the retry lookup so an exact retry can
        // never be accepted using a predecessor that should already be retired.
        if self.reconcile_certificate_retirement(now_unix_millis)? {
            self.persist()?;
        }
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
        #[cfg(unix)]
        {
            write_canonical_at(&self._root_handle, "control-state.json", &self.state)
        }
        #[cfg(not(unix))]
        {
            write_canonical_file(&self.root.join("control-state.json"), &self.state)
        }
    }

    fn reconcile_certificate_retirement(
        &mut self,
        now_unix_millis: u64,
    ) -> Result<bool, ContinuityControlError> {
        let Some(succession) = self.state.succession.as_mut() else {
            return Ok(false);
        };
        let retirement_unix_millis = succession
            .retirement_unix_millis
            .parse::<u64>()
            .map_err(|_| ContinuityControlError::InvalidSuccession)?;
        if succession.retired || !succession.activated || now_unix_millis < retirement_unix_millis {
            return Ok(false);
        }
        let predecessor_pending = self.state.journal.accepted.values().any(|accepted| {
            accepted.certificate_generation == succession.predecessor_generation
                && accepted.response.is_none()
        });
        if !predecessor_pending {
            succession.retired = true;
            return Ok(true);
        }
        Ok(false)
    }
}

impl Drop for DurableContinuityJournal {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self._lock);
    }
}

#[derive(Clone)]
pub struct LiveOperationContinuity {
    factories: Arc<BTreeMap<String, crate::OperationalFactory>>,
}

impl LiveOperationContinuity {
    pub fn from_factories(
        factories: BTreeMap<String, crate::OperationalFactory>,
    ) -> Result<Self, ContinuityControlError> {
        let expected = crate::assembly::REQUIRED_OPERATIONAL_ADAPTERS
            .into_iter()
            .map(|kind| kind.service_name().to_owned())
            .collect::<BTreeSet<_>>();
        if factories.keys().cloned().collect::<BTreeSet<_>>() != expected {
            return Err(ContinuityControlError::ParticipantRegistry);
        }
        Ok(Self {
            factories: Arc::new(factories),
        })
    }

    async fn quiesce_governance(&self) -> Result<Vec<u8>, ContinuityControlError> {
        let mut snapshots = BTreeMap::new();
        for (name, factory) in self
            .factories
            .iter()
            .filter(|(_, factory)| factory.is_governed())
        {
            snapshots.insert(
                name.clone(),
                factory
                    .continuity_quiesce()
                    .await
                    .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
            );
        }
        if snapshots.len() != 2 {
            return Err(ContinuityControlError::ParticipantRegistry);
        }
        serde_jcs::to_vec(&snapshots)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
    }

    async fn quiesce_all(&self) -> Result<Vec<u8>, ContinuityControlError> {
        let mut snapshots = BTreeMap::new();
        for (name, factory) in self.factories.iter() {
            snapshots.insert(
                name.clone(),
                factory
                    .continuity_quiesce()
                    .await
                    .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
            );
        }
        serde_jcs::to_vec(&snapshots)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
    }

    async fn resume_governance(
        &self,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<(), ContinuityControlError> {
        for factory in self
            .factories
            .values()
            .filter(|factory| factory.is_governed())
        {
            resume_operation_factory(factory, guard).await?;
        }
        Ok(())
    }

    async fn resume_all(
        &self,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<(), ContinuityControlError> {
        for factory in self.factories.values() {
            resume_operation_factory(factory, guard).await?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct LiveContinuityRegistry {
    ingress: CanonicalIngress,
    recorder: RuntimeRecorder,
    reasoning: Arc<ReasoningServices>,
    operation_state_root: PathBuf,
    operation_continuity: LiveOperationContinuity,
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
#[serde(deny_unknown_fields)]
struct ParticipantResumeReceipt {
    participant: String,
    prepare_receipt_sha256: String,
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
    predecessor_sha256: Option<String>,
    topology_sha256: String,
    config_sha256: String,
    phase: SourceOperationPhase,
    #[serde(default)]
    participant_started: BTreeSet<String>,
    participant_receipts: BTreeMap<String, ParticipantPrepareReceipt>,
    #[serde(default)]
    participant_resume_receipts: BTreeMap<String, ParticipantResumeReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    checkpoint: Option<SourceCheckpointHandle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    resume_receipt: Option<SourceResumeReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableSourceState {
    schema: String,
    root_generation: u64,
    operations: BTreeMap<u64, SourceOperationState>,
}

const SOURCE_STATE_SCHEMA: &str = "adl.runtime.kernel_continuity.source_state.v4";

impl LiveContinuityRegistry {
    pub fn from_production_handles(
        ingress: CanonicalIngress,
        recorder: RuntimeRecorder,
        reasoning: Arc<ReasoningServices>,
        operation_state_root: PathBuf,
        operation_continuity: LiveOperationContinuity,
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
            operation_continuity,
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

    async fn prepare_participant(
        &self,
        participant: &str,
        accepted_prefix: u64,
    ) -> Result<Vec<u8>, ContinuityControlError> {
        match participant {
            "admission" => {
                if self.ingress.admission_is_open()
                    || self.ingress.snapshot().accepted_through != accepted_prefix
                {
                    return Err(ContinuityControlError::AcceptedPrefix);
                }
                serde_jcs::to_vec(&self.ingress.snapshot())
                    .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
            }
            "accepted_prefix" => {
                let snapshot = self.ingress.snapshot();
                if self.ingress.admission_is_open() || snapshot.accepted_through != accepted_prefix
                {
                    return Err(ContinuityControlError::AcceptedPrefix);
                }
                serde_jcs::to_vec(&serde_json::json!({
                    "schema": "adl.runtime.accepted_prefix.v1",
                    "accepted_through": snapshot.accepted_through,
                    "recorder": self.recorder.snapshot(),
                }))
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
            }
            "reasoning_mutation" => self
                .reasoning
                .continuity_quiesce()
                .map_err(|error| ContinuityControlError::Encoding(error.to_string())),
            "governance" => self.operation_continuity.quiesce_governance().await,
            "operation_state" => {
                let live = self.operation_continuity.quiesce_all().await?;
                let durable =
                    crate::assembly::operation_state_projection(&self.operation_state_root)?;
                serde_jcs::to_vec(&serde_json::json!({
                    "schema": "adl.runtime.operation_state_continuity.v1",
                    "live": BASE64.encode(live),
                    "durable": BASE64.encode(durable),
                }))
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
            }
            _ => Err(ContinuityControlError::ParticipantRegistry),
        }
    }

    async fn resume_started(
        &self,
        started: &BTreeSet<String>,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<(), ContinuityControlError> {
        for participant in REQUIRED_PARTICIPANTS.iter().rev() {
            if !started.contains(*participant) {
                continue;
            }
            check_optional_guard(guard)?;
            match *participant {
                "operation_state" => self.operation_continuity.resume_all(guard).await?,
                "governance" => self.operation_continuity.resume_governance(guard).await?,
                "reasoning_mutation" => {
                    run_optional_guarded(guard, async {
                        self.reasoning
                            .continuity_resume()
                            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
                    })
                    .await??;
                }
                "accepted_prefix" | "admission" => {}
                _ => return Err(ContinuityControlError::ParticipantRegistry),
            }
            check_optional_guard(guard)?;
        }
        run_optional_guarded(guard, async { self.ingress.reopen() }).await?;
        Ok(())
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
            #[cfg(unix)]
            let catalog: SignedBundleCatalog = {
                let generation_root = open_directory_at(&export_root_handle, &name)?;
                read_canonical_at(&generation_root, "catalog.json", bounds.max_frame_bytes)?
            };
            #[cfg(not(unix))]
            let catalog: SignedBundleCatalog =
                read_canonical_file(&entry.path().join("catalog.json"), bounds.max_frame_bytes)?;
            validate_catalog(&catalog, &bounds)?;
            catalog.verify(&trusted_keys)?;
            if catalog.generation != generation {
                return Err(ContinuityControlError::CatalogMismatch);
            }
            retained.insert(generation, catalog);
        }
        #[cfg(not(unix))]
        let state_path = export_root.join("source-state.json");
        #[cfg(unix)]
        let existing_source_state = match read_canonical_at(
            &export_root_handle,
            "source-state.json",
            bounds.max_frame_bytes,
        ) {
            Ok(state) => Some(state),
            Err(ContinuityControlError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(error) => return Err(error),
        };
        #[cfg(not(unix))]
        let existing_source_state = if state_path.exists() {
            Some(read_canonical_file(&state_path, bounds.max_frame_bytes)?)
        } else {
            None
        };
        let mut source_state = if let Some(state) = existing_source_state {
            state
        } else {
            DurableSourceState {
                schema: SOURCE_STATE_SCHEMA.to_owned(),
                root_generation,
                operations: BTreeMap::new(),
            }
        };
        if source_state.schema != SOURCE_STATE_SCHEMA
            || source_state.operations.len() > bounds.max_journal_entries
        {
            return Err(ContinuityControlError::CorruptJournal);
        }
        if source_state.root_generation != root_generation {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        for operation in source_state.operations.values_mut() {
            if let Some(catalog) = retained.get(&operation.generation) {
                if catalog.predecessor_sha256 == operation.predecessor_sha256
                    && catalog.accepted_prefix == operation.accepted_prefix
                    && catalog.topology_sha256 == operation.topology_sha256
                    && catalog.config_sha256 == operation.config_sha256
                    && exact_prepare_receipts(operation)
                {
                    operation.checkpoint = Some(SourceCheckpointHandle {
                        generation: operation.generation,
                        root_generation,
                        catalog_sha256: catalog.digest()?,
                        bundle_sha256: bundle_digest(catalog)?,
                    });
                    if operation.phase == SourceOperationPhase::Preparing {
                        operation.phase = SourceOperationPhase::Committed;
                    }
                } else {
                    operation.phase = SourceOperationPhase::RecoveryRequired;
                }
            } else if operation.phase == SourceOperationPhase::Preparing {
                operation.phase = SourceOperationPhase::RecoveryRequired;
            }
            match operation.phase {
                SourceOperationPhase::Committed | SourceOperationPhase::RecoveryRequired => {
                    registry.ingress.close();
                }
                SourceOperationPhase::Preparing | SourceOperationPhase::Resuming => {
                    registry.ingress.close();
                    // A process restart cannot manufacture acknowledgements
                    // for live effects. Keep admission closed until an explicit
                    // reconciliation path invokes the actual participant
                    // resume effects and records their receipts.
                    operation.phase = SourceOperationPhase::RecoveryRequired;
                }
                SourceOperationPhase::Resumed => {
                    if !exact_prepare_receipts(operation)
                        || !exact_resume_receipts(operation)
                        || operation.resume_receipt.is_none()
                    {
                        registry.ingress.close();
                        operation.phase = SourceOperationPhase::RecoveryRequired;
                    }
                }
            }
        }
        #[cfg(unix)]
        write_canonical_at(&export_root_handle, "source-state.json", &source_state)?;
        #[cfg(not(unix))]
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
            if catalog.predecessor_sha256 != predecessor_sha256
                || catalog.accepted_prefix != accepted_prefix
                || catalog.topology_sha256 != topology_sha256
                || catalog.config_sha256 != config_sha256
            {
                return Err(ContinuityControlError::ConflictingRetry);
            }
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
        // Persist the exact source intent before admission is closed. A crash
        // can therefore never leave a closed admission gate with no durable
        // operation identity to reconcile.
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
                    predecessor_sha256: predecessor_sha256.clone(),
                    topology_sha256: topology_sha256.clone(),
                    config_sha256: config_sha256.clone(),
                    phase: SourceOperationPhase::Preparing,
                    participant_started: BTreeSet::new(),
                    participant_receipts: BTreeMap::new(),
                    participant_resume_receipts: BTreeMap::new(),
                    checkpoint: None,
                    resume_receipt: None,
                },
            );
            self.persist_source_state(&state)?;
        }
        let actual_prefix = match self
            .registry
            .prepare_admission(deadline, cancellation)
            .await
        {
            Ok(prefix) => prefix,
            Err(error) => return self.mark_source_recovery_required(generation, error),
        };
        if actual_prefix != accepted_prefix {
            return self
                .mark_source_recovery_required(generation, ContinuityControlError::AcceptedPrefix);
        }
        let mut snapshots = BTreeMap::new();
        for participant in REQUIRED_PARTICIPANTS {
            if cancellation.is_cancelled() || started.elapsed() >= deadline {
                return self
                    .rollback_source_prepare(
                        generation,
                        if cancellation.is_cancelled() {
                            ContinuityControlError::Cancelled
                        } else {
                            ContinuityControlError::Deadline
                        },
                    )
                    .await;
            }
            {
                let mut state = self
                    .source_state
                    .lock()
                    .map_err(|_| ContinuityControlError::CorruptJournal)?;
                state
                    .operations
                    .get_mut(&generation)
                    .ok_or(ContinuityControlError::CorruptJournal)?
                    .participant_started
                    .insert(participant.to_owned());
                self.persist_source_state(&state)?;
            }
            let remaining = deadline.saturating_sub(started.elapsed());
            let bytes = match tokio::select! {
                _ = cancellation.cancelled() => Err(ContinuityControlError::Cancelled),
                result = tokio::time::timeout(
                    remaining,
                    self.registry.prepare_participant(participant, accepted_prefix),
                ) => result.map_err(|_| ContinuityControlError::Deadline)?,
            } {
                Ok(bytes) => bytes,
                Err(error) => return self.rollback_source_prepare(generation, error).await,
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
            return self
                .rollback_source_prepare(generation, ContinuityControlError::InvalidBounds)
                .await;
        }
        let pending_name = format!(".generation-{generation}.pending");
        let committed_name = format!("generation-{generation}");
        #[cfg(unix)]
        let pending_root = match create_directory_at(&self._export_root_handle, &pending_name) {
            Ok(handle) => handle,
            Err(error) => return self.rollback_source_prepare(generation, error).await,
        };
        #[cfg(not(unix))]
        let pending_root = {
            let pending = self.export_root.join(&pending_name);
            let committed = self.export_root.join(&committed_name);
            if pending.exists() || committed.exists() {
                return self
                    .rollback_source_prepare(generation, ContinuityControlError::ConflictingRetry)
                    .await;
            }
            if let Err(error) = fs::create_dir(&pending) {
                return self.rollback_source_prepare(generation, error.into()).await;
            }
            File::open(pending)?
        };
        let result = (|| {
            let mut entries = Vec::with_capacity(snapshots.len());
            let mut offset = 0_u64;
            let mut total = 0_u64;
            for (index, (service, bytes)) in snapshots.into_iter().enumerate() {
                if cancellation.is_cancelled() || started.elapsed() >= deadline {
                    return Err(if cancellation.is_cancelled() {
                        ContinuityControlError::Cancelled
                    } else {
                        ContinuityControlError::Deadline
                    });
                }
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
                #[cfg(unix)]
                write_new_synced_at(&pending_root, &file, &bytes)?;
                #[cfg(not(unix))]
                write_new_synced(&self.export_root.join(&pending_name).join(&file), &bytes)?;
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
            if cancellation.is_cancelled() || started.elapsed() >= deadline {
                return Err(if cancellation.is_cancelled() {
                    ContinuityControlError::Cancelled
                } else {
                    ContinuityControlError::Deadline
                });
            }
            #[cfg(unix)]
            {
                write_canonical_at(&pending_root, "catalog.json", &catalog)?;
                pending_root.sync_all()?;
                drop(pending_root);
                rename_at(
                    &self._export_root_handle,
                    &pending_name,
                    &self._export_root_handle,
                    &committed_name,
                )?;
            }
            #[cfg(not(unix))]
            {
                let pending = self.export_root.join(&pending_name);
                let committed = self.export_root.join(&committed_name);
                write_canonical_file(&pending.join("catalog.json"), &catalog)?;
                sync_dir(&pending)?;
                fs::rename(&pending, &committed)?;
                sync_dir(&self.export_root)?;
            }
            Ok(catalog)
        })();
        let catalog = match result {
            Ok(catalog) => catalog,
            Err(error) => {
                // Leave an anchored pending generation for explicit recovery;
                // never chase a replaced path recursively during error cleanup.
                return self.rollback_source_prepare(generation, error).await;
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
            #[cfg(not(unix))]
            generation_root: self
                .export_root
                .join(format!("generation-{}", handle.generation)),
            #[cfg(unix)]
            generation_root_handle: open_directory_at(
                &self._export_root_handle,
                &format!("generation-{}", handle.generation),
            )?,
            handle: handle.clone(),
            catalog,
            bounds: self.bounds.clone(),
        })
    }

    pub async fn resume_source(
        &self,
        handle: &SourceCheckpointHandle,
    ) -> Result<SourceResumeReceipt, ContinuityControlError> {
        self.resume_source_inner(handle, None).await
    }

    pub async fn resume_source_bounded(
        &self,
        handle: &SourceCheckpointHandle,
        deadline_unix_millis: u64,
        cancellation: &CancellationToken,
    ) -> Result<SourceResumeReceipt, ContinuityControlError> {
        let guard = EffectGuard {
            deadline_unix_millis,
            cancellation,
        };
        self.resume_source_inner(handle, Some(&guard)).await
    }

    async fn resume_source_inner(
        &self,
        handle: &SourceCheckpointHandle,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<SourceResumeReceipt, ContinuityControlError> {
        check_optional_guard(guard)?;
        if handle.root_generation != self.root_generation
            || !self
                .retained
                .lock()
                .map_err(|_| ContinuityControlError::CorruptJournal)?
                .contains_key(&handle.generation)
        {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        let (started, bindings) = {
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
            if operation.phase != SourceOperationPhase::Committed
                || !exact_prepare_receipts(operation)
            {
                operation.phase = SourceOperationPhase::RecoveryRequired;
                self.persist_source_state(&state)?;
                return Err(ContinuityControlError::RecoveryRequired);
            }
            operation.phase = SourceOperationPhase::Resuming;
            let started = operation.participant_started.clone();
            let bindings = started
                .iter()
                .map(|participant| {
                    (
                        participant.clone(),
                        participant_resume_binding(operation, participant),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            self.persist_source_state(&state)?;
            (started, bindings)
        };
        if self.registry.resume_started(&started, guard).await.is_err() {
            return self.mark_source_recovery_required(
                handle.generation,
                ContinuityControlError::RecoveryRequired,
            );
        }
        check_optional_guard(guard)?;
        let mut state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?;
        let operation = state
            .operations
            .get_mut(&handle.generation)
            .ok_or(ContinuityControlError::CorruptJournal)?;
        operation.participant_resume_receipts = bindings
            .into_iter()
            .map(|(participant, binding)| {
                let receipt = ParticipantResumeReceipt {
                    participant: participant.clone(),
                    prepare_receipt_sha256: binding.clone(),
                    receipt_sha256: participant_resume_receipt_digest(&participant, &binding),
                };
                (participant, receipt)
            })
            .collect();
        if !exact_resume_receipts(operation) {
            operation.phase = SourceOperationPhase::RecoveryRequired;
            self.persist_source_state(&state)?;
            return Err(ContinuityControlError::RecoveryRequired);
        }
        let receipt = SourceResumeReceipt::new(format!(
            "source-resumed:{}:{}:{}:{}",
            handle.generation,
            handle.catalog_sha256,
            handle.bundle_sha256,
            resume_receipts_digest(operation)?
        ));
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

    async fn rollback_source_prepare<T>(
        &self,
        generation: u64,
        error: ContinuityControlError,
    ) -> Result<T, ContinuityControlError> {
        let (accepted_prefix, started, bindings) = {
            let mut state = self
                .source_state
                .lock()
                .map_err(|_| ContinuityControlError::RecoveryRequired)?;
            let operation = state
                .operations
                .get_mut(&generation)
                .ok_or(ContinuityControlError::RecoveryRequired)?;
            operation.phase = SourceOperationPhase::Resuming;
            let bindings = operation
                .participant_started
                .iter()
                .map(|participant| {
                    (
                        participant.clone(),
                        participant_resume_binding(operation, participant),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let result = (
                operation.accepted_prefix,
                operation.participant_started.clone(),
                bindings,
            );
            self.persist_source_state(&state)
                .map_err(|_| ContinuityControlError::RecoveryRequired)?;
            result
        };
        if self.registry.resume_started(&started, None).await.is_err() {
            return self.mark_source_recovery_required(
                generation,
                ContinuityControlError::RecoveryRequired,
            );
        }
        let mut state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        let operation = state
            .operations
            .get_mut(&generation)
            .ok_or(ContinuityControlError::RecoveryRequired)?;
        operation.participant_resume_receipts = bindings
            .into_iter()
            .map(|(participant, prepare_receipt_sha256)| {
                let receipt = ParticipantResumeReceipt {
                    participant: participant.clone(),
                    prepare_receipt_sha256: prepare_receipt_sha256.clone(),
                    receipt_sha256: participant_resume_receipt_digest(
                        &participant,
                        &prepare_receipt_sha256,
                    ),
                };
                (participant, receipt)
            })
            .collect();
        if !exact_resume_receipts(operation) {
            operation.phase = SourceOperationPhase::RecoveryRequired;
            self.persist_source_state(&state)
                .map_err(|_| ContinuityControlError::RecoveryRequired)?;
            return Err(ContinuityControlError::RecoveryRequired);
        }
        let receipt = SourceResumeReceipt::new(format!(
            "source-rollback-resumed:{}:{}:{}",
            generation,
            accepted_prefix,
            resume_receipts_digest(operation)
                .map_err(|_| ContinuityControlError::RecoveryRequired)?
        ));
        operation.resume_receipt = Some(receipt);
        operation.phase = SourceOperationPhase::Resumed;
        self.persist_source_state(&state)
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        Err(error)
    }

    fn mark_source_recovery_required<T>(
        &self,
        generation: u64,
        _cause: ContinuityControlError,
    ) -> Result<T, ContinuityControlError> {
        let mut state = self
            .source_state
            .lock()
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        let operation = state
            .operations
            .get_mut(&generation)
            .ok_or(ContinuityControlError::RecoveryRequired)?;
        operation.phase = SourceOperationPhase::RecoveryRequired;
        self.registry.ingress.close();
        self.persist_source_state(&state)
            .map_err(|_| ContinuityControlError::RecoveryRequired)?;
        Err(ContinuityControlError::RecoveryRequired)
    }

    fn persist_source_state(
        &self,
        state: &DurableSourceState,
    ) -> Result<(), ContinuityControlError> {
        #[cfg(unix)]
        {
            write_canonical_at(&self._export_root_handle, "source-state.json", state)
        }
        #[cfg(not(unix))]
        {
            write_canonical_file(&self.export_root.join("source-state.json"), state)
        }
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

fn exact_prepare_receipts(operation: &SourceOperationState) -> bool {
    operation.participant_started.len() == REQUIRED_PARTICIPANTS.len()
        && operation.participant_receipts.len() == REQUIRED_PARTICIPANTS.len()
        && REQUIRED_PARTICIPANTS.iter().all(|participant| {
            operation
                .participant_receipts
                .get(*participant)
                .is_some_and(|receipt| {
                    receipt.participant == *participant
                        && receipt.accepted_prefix == operation.accepted_prefix
                        && receipt.receipt_sha256
                            == participant_receipt_digest(
                                participant,
                                receipt.accepted_prefix,
                                &receipt.snapshot_sha256,
                            )
                })
        })
}

fn participant_resume_receipt_digest(participant: &str, prepare_receipt_sha256: &str) -> String {
    sha256(format!("resume\0{participant}\0{prepare_receipt_sha256}").as_bytes())
}

fn participant_resume_binding(operation: &SourceOperationState, participant: &str) -> String {
    operation
        .participant_receipts
        .get(participant)
        .map(|receipt| receipt.receipt_sha256.clone())
        .unwrap_or_else(|| {
            sha256(
                format!(
                    "uncertain-prepare\0{}\0{}\0{}",
                    operation.generation, operation.accepted_prefix, participant
                )
                .as_bytes(),
            )
        })
}

fn exact_resume_receipts(operation: &SourceOperationState) -> bool {
    operation.participant_resume_receipts.len() == operation.participant_started.len()
        && operation.participant_started.iter().all(|participant| {
            let binding = participant_resume_binding(operation, participant);
            operation
                .participant_resume_receipts
                .get(participant)
                .is_some_and(|receipt| {
                    receipt.participant == *participant
                        && receipt.prepare_receipt_sha256 == binding
                        && receipt.receipt_sha256
                            == participant_resume_receipt_digest(participant, &binding)
                })
        })
}

fn resume_receipts_digest(
    operation: &SourceOperationState,
) -> Result<String, ContinuityControlError> {
    if !exact_resume_receipts(operation) {
        return Err(ContinuityControlError::RecoveryRequired);
    }
    Ok(sha256(
        &serde_jcs::to_vec(&operation.participant_resume_receipts)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
    ))
}

pub struct ContinuityBundleSourcePort {
    #[cfg(not(unix))]
    generation_root: PathBuf,
    #[cfg(unix)]
    generation_root_handle: File,
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
        #[cfg(unix)]
        let mut file = open_file_at(&self.generation_root_handle, &entry.file, libc::O_RDONLY, 0)?;
        #[cfg(not(unix))]
        let mut file = {
            let path = self.generation_root.join(&entry.file);
            validate_regular_single_link_file(&path, entry.bytes)?;
            File::open(&path)?
        };
        use std::io::{Seek, SeekFrom};
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
    pub fn root_generation(&self) -> u64 {
        self.root_generation
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

    pub fn root_generation(&self) -> u64 {
        self.root_generation
    }

    pub fn channel_epoch(&self) -> u64 {
        self.channel_epoch
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_sha256
    }

    pub fn matches_stage(&self, handle: &TargetStageHandle, channel_epoch: u64) -> bool {
        self.stage_id == handle.stage_id
            && self.root_generation == handle.root_generation
            && self.catalog_sha256 == handle.catalog_sha256
            && self.is_self_consistent(channel_epoch)
    }

    pub fn is_self_consistent(&self, channel_epoch: u64) -> bool {
        self.root_generation != 0
            && self.channel_epoch == channel_epoch
            && hex::decode(&self.catalog_sha256).is_ok_and(|bytes| bytes.len() == 32)
            && self.cleanup_id
                == sha256(format!("cleanup:{}:{}", self.stage_id, self.root_generation).as_bytes())
    }

    pub fn digest(&self) -> Result<String, ContinuityControlError> {
        Ok(sha256(&serde_jcs::to_vec(self).map_err(|error| {
            ContinuityControlError::Encoding(error.to_string())
        })?))
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
    root_generation: u64,
    catalog_sha256: String,
    cleanup_permit_sha256: String,
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
    pub root_generation: u64,
    pub catalog_sha256: String,
    pub cleanup_permit_sha256: String,
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
    pub fn unsigned_bytes(&self) -> Result<Vec<u8>, ContinuityControlError> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_jcs::to_vec(&unsigned)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))
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
        chunk_index: u32,
        relative_offset: u64,
        predecessor_sha256: Option<String>,
        chunk_sha256: String,
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
    ChunkWritten {
        receipt: TargetChunkReceipt,
    },
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
    #[serde(default)]
    received: BTreeMap<String, TargetChunkReceipt>,
    possession: Option<TargetPossessionEvidence>,
    discard: Option<TargetDiscardReceipt>,
    activation: Option<TargetActivationReceipt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    activation_decision_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    activation_decision: Option<MigrationDecisionCertificate>,
    #[serde(default)]
    cleanup_consumed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetChunkReceipt {
    ordinal: u32,
    chunk_index: u32,
    relative_offset: u64,
    bytes: u64,
    chunk_sha256: String,
    predecessor_sha256: Option<String>,
    receipt_sha256: String,
}

impl TargetChunkReceipt {
    pub fn digest(&self) -> &str {
        &self.receipt_sha256
    }
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

/// Kernel-owned target effect port. It performs filesystem effects but owns no
/// migration policy and cannot mint a #204 decision.
pub struct TargetContinuityEffectPort {
    config: ContinuityControlInitConfig,
    _lock: File,
    _staging_root_handle: File,
    _state_root_handle: File,
    _terminal_root_handle: File,
    trusted_catalog_keys: BTreeMap<(String, u64), VerifyingKey>,
    trusted_decision_keys: BTreeMap<(String, u64), VerifyingKey>,
    stages: Mutex<BTreeMap<String, TargetStageState>>,
}

struct EffectGuard<'a> {
    deadline_unix_millis: u64,
    cancellation: &'a CancellationToken,
}

impl EffectGuard<'_> {
    fn check(&self) -> Result<(), ContinuityControlError> {
        check_effect_window(self.deadline_unix_millis, self.cancellation)
    }

    fn remaining(&self) -> Result<std::time::Duration, ContinuityControlError> {
        self.check()?;
        remaining_effect_window(self.deadline_unix_millis)
    }
}

fn check_optional_guard(guard: Option<&EffectGuard<'_>>) -> Result<(), ContinuityControlError> {
    guard.map_or(Ok(()), EffectGuard::check)
}

async fn resume_operation_factory(
    factory: &crate::OperationalFactory,
    guard: Option<&EffectGuard<'_>>,
) -> Result<(), ContinuityControlError> {
    let Some(guard) = guard else {
        factory.continuity_resume().await;
        return Ok(());
    };
    let remaining = guard.remaining()?;
    if factory
        .continuity_resume_bounded(remaining, guard.cancellation)
        .await
    {
        guard.check()
    } else {
        Err(guard
            .check()
            .err()
            .unwrap_or(ContinuityControlError::RecoveryRequired))
    }
}

async fn run_optional_guarded<F, T>(
    guard: Option<&EffectGuard<'_>>,
    effect: F,
) -> Result<T, ContinuityControlError>
where
    F: std::future::Future<Output = T>,
{
    let Some(guard) = guard else {
        return Ok(effect.await);
    };
    let remaining = guard.remaining()?;
    tokio::select! {
        biased;
        _ = guard.cancellation.cancelled() => Err(ContinuityControlError::Cancelled),
        _ = tokio::time::sleep(remaining) => Err(ContinuityControlError::Deadline),
        output = effect => {
            guard.check()?;
            Ok(output)
        }
    }
}

impl TargetContinuityEffectPort {
    pub fn open(
        config: ContinuityControlInitConfig,
        trusted_catalog_keys: BTreeMap<(String, u64), VerifyingKey>,
        trusted_decision_keys: BTreeMap<(String, u64), VerifyingKey>,
    ) -> Result<Self, ContinuityControlError> {
        if trusted_catalog_keys.is_empty()
            || trusted_decision_keys.is_empty()
            || trusted_catalog_keys.values().any(|catalog| {
                trusted_decision_keys
                    .values()
                    .any(|decision| catalog == decision)
            })
        {
            return Err(ContinuityControlError::ActivationDecision);
        }
        fs::create_dir_all(&config.staging_dir)?;
        validate_existing_directory(&config.staging_dir)?;
        fs::create_dir_all(&config.state_dir)?;
        validate_existing_directory(&config.state_dir)?;
        let state_root_handle = File::open(&config.state_dir)?;
        #[cfg(unix)]
        let lock = open_file_at(
            &state_root_handle,
            "target-writer.lock",
            libc::O_CREAT | libc::O_RDWR,
            0o600,
        )?;
        #[cfg(not(unix))]
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(config.state_dir.join("target-writer.lock"))?;
        lock.try_lock_exclusive().map_err(|error| {
            if error.kind() == std::io::ErrorKind::WouldBlock {
                ContinuityControlError::AlreadyOpen
            } else {
                error.into()
            }
        })?;
        let staging_root_handle = File::open(&config.staging_dir)?;
        let terminal_root = config.state_dir.join("target-results");
        #[cfg(unix)]
        let terminal_root_handle = match open_directory_at(&state_root_handle, "target-results") {
            Ok(handle) => handle,
            Err(ContinuityControlError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                create_directory_at(&state_root_handle, "target-results")?
            }
            Err(error) => return Err(error),
        };
        #[cfg(not(unix))]
        let terminal_root_handle = {
            fs::create_dir_all(&terminal_root)?;
            validate_existing_directory(&terminal_root)?;
            File::open(&terminal_root)?
        };
        let mut stages = BTreeMap::new();
        for entry in fs::read_dir(&terminal_root)? {
            let entry = entry?;
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_file() {
                return Err(ContinuityControlError::UnsafeRoot);
            }
            let leaf = entry.file_name().to_string_lossy().into_owned();
            #[cfg(unix)]
            let state: TargetStageState =
                read_canonical_at(&terminal_root_handle, &leaf, config.bounds.max_frame_bytes)?;
            #[cfg(not(unix))]
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
            validate_loaded_stage(
                &state,
                &config,
                &trusted_catalog_keys,
                &trusted_decision_keys,
                None,
                None,
            )?;
            stages.insert(state.handle.stage_id.clone(), state);
        }
        for entry in fs::read_dir(&config.staging_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_symlink() || !entry.file_type()?.is_dir() {
                return Err(ContinuityControlError::UnsafeRoot);
            }
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            #[cfg(unix)]
            let stage_root = open_directory_at(&staging_root_handle, &entry_name)?;
            #[cfg(unix)]
            let state: TargetStageState =
                read_canonical_at(&stage_root, "stage.json", config.bounds.max_frame_bytes)?;
            #[cfg(not(unix))]
            let state: TargetStageState = read_canonical_file(
                &entry.path().join("stage.json"),
                config.bounds.max_frame_bytes,
            )?;
            let expected_stage_name = entry_name.strip_prefix(".discard-").unwrap_or(&entry_name);
            if state.schema != STAGE_STATE_SCHEMA || state.handle.stage_id != expected_stage_name {
                return Err(ContinuityControlError::CorruptStage);
            }
            let mut state = state;
            validate_loaded_stage(
                &state,
                &config,
                &trusted_catalog_keys,
                &trusted_decision_keys,
                #[cfg(unix)]
                Some(&stage_root),
                #[cfg(not(unix))]
                None,
                #[cfg(unix)]
                None,
                #[cfg(not(unix))]
                Some(&entry.path()),
            )?;
            if entry_name.starts_with(".discard-") || state.status == TargetStageStatus::Discarding
            {
                let receipt = state
                    .discard
                    .clone()
                    .ok_or(ContinuityControlError::CorruptStage)?;
                state.status = TargetStageStatus::Discarded;
                state.discard = Some(receipt);
                #[cfg(unix)]
                write_canonical_at(
                    &terminal_root_handle,
                    &format!("{}.json", state.handle.stage_id),
                    &state,
                )?;
                #[cfg(not(unix))]
                write_canonical_file(
                    &terminal_root.join(format!("{}.json", state.handle.stage_id)),
                    &state,
                )?;
                #[cfg(unix)]
                {
                    for catalog_entry in &state.catalog.entries {
                        match unlink_at(&stage_root, &catalog_entry.file, false) {
                            Ok(()) => {}
                            Err(ContinuityControlError::Io(error))
                                if error.kind() == std::io::ErrorKind::NotFound => {}
                            Err(error) => return Err(error),
                        }
                    }
                    unlink_at(&stage_root, "stage.json", false)?;
                    drop(stage_root);
                    unlink_at(&staging_root_handle, &entry_name, true)?;
                }
                #[cfg(not(unix))]
                {
                    fs::remove_dir_all(entry.path())?;
                    sync_dir(&config.staging_dir)?;
                }
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
                #[cfg(unix)]
                write_canonical_at(&state_root_handle, "active-target.json", &active)?;
                #[cfg(not(unix))]
                write_canonical_file(&config.state_dir.join("active-target.json"), &active)?;
                state.status = TargetStageStatus::Activated;
                #[cfg(unix)]
                {
                    write_canonical_at(&stage_root, "stage.json", &state)?;
                    write_canonical_at(
                        &terminal_root_handle,
                        &format!("{}.json", state.handle.stage_id),
                        &state,
                    )?;
                }
                #[cfg(not(unix))]
                {
                    write_canonical_file(&entry.path().join("stage.json"), &state)?;
                    write_canonical_file(
                        &terminal_root.join(format!("{}.json", state.handle.stage_id)),
                        &state,
                    )?;
                }
            }
            if let Some(terminal) = stages.get(&state.handle.stage_id) {
                if terminal != &state && terminal.status != TargetStageStatus::Discarded {
                    return Err(ContinuityControlError::CorruptStage);
                }
            } else {
                stages.insert(state.handle.stage_id.clone(), state);
            }
        }
        let activated = stages
            .values()
            .filter(|state| state.status == TargetStageStatus::Activated)
            .collect::<Vec<_>>();
        if activated.len() > 1 {
            return Err(ContinuityControlError::CorruptStage);
        }
        #[cfg(unix)]
        let active_record: Option<ActiveTargetRecord> = match read_canonical_at(
            &state_root_handle,
            "active-target.json",
            config.bounds.max_frame_bytes,
        ) {
            Ok(record) => Some(record),
            Err(ContinuityControlError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound =>
            {
                None
            }
            Err(error) => return Err(error),
        };
        #[cfg(not(unix))]
        let active_record: Option<ActiveTargetRecord> = {
            let path = config.state_dir.join("active-target.json");
            if path.exists() {
                Some(read_canonical_file(&path, config.bounds.max_frame_bytes)?)
            } else {
                None
            }
        };
        match (activated.first(), active_record) {
            (None, None) => {}
            (Some(state), Some(active))
                if active.schema == ACTIVE_TARGET_SCHEMA
                    && active.stage_id == state.handle.stage_id
                    && active.root_generation == state.handle.root_generation
                    && active.catalog_sha256 == state.handle.catalog_sha256
                    && Some(active.decision_sha256.as_str())
                        == state.activation_decision_sha256.as_deref()
                    && Some(&active.receipt) == state.activation.as_ref() => {}
            _ => return Err(ContinuityControlError::CorruptStage),
        }
        Ok(Self {
            config,
            _lock: lock,
            _staging_root_handle: staging_root_handle,
            _state_root_handle: state_root_handle,
            _terminal_root_handle: terminal_root_handle,
            trusted_catalog_keys,
            trusted_decision_keys,
            stages: Mutex::new(stages),
        })
    }

    pub fn create_stage(
        &self,
        stage_id: &str,
        root_generation: u64,
        catalog: SignedBundleCatalog,
    ) -> Result<StageCreated, ContinuityControlError> {
        self.create_stage_inner(stage_id, root_generation, catalog, None)
    }

    fn create_stage_bounded(
        &self,
        stage_id: &str,
        root_generation: u64,
        catalog: SignedBundleCatalog,
        guard: &EffectGuard<'_>,
    ) -> Result<StageCreated, ContinuityControlError> {
        self.create_stage_inner(stage_id, root_generation, catalog, Some(guard))
    }

    fn create_stage_inner(
        &self,
        stage_id: &str,
        root_generation: u64,
        catalog: SignedBundleCatalog,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<StageCreated, ContinuityControlError> {
        check_optional_guard(guard)?;
        self.verify_staging_root_anchor()?;
        if !safe_identity(stage_id) || root_generation == 0 {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        catalog.verify(&self.trusted_catalog_keys)?;
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
        check_optional_guard(guard)?;
        #[cfg(unix)]
        let stage_root = create_directory_at(&self._staging_root_handle, stage_id)?;
        #[cfg(not(unix))]
        let stage_root = {
            let path = self.stage_path(stage_id);
            if path.exists() {
                return Err(ContinuityControlError::CorruptStage);
            }
            fs::create_dir(&path)?;
            File::open(path)?
        };
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
            activation_decision: None,
            cleanup_consumed: false,
        };
        check_optional_guard(guard)?;
        #[cfg(unix)]
        write_canonical_at(&stage_root, "stage.json", &state)?;
        #[cfg(not(unix))]
        write_canonical_file(&self.stage_path(stage_id).join("stage.json"), &state)?;
        self._staging_root_handle.sync_all()?;
        stages.insert(stage_id.to_owned(), state);
        Ok(StageCreated { handle, cleanup })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn write_chunk(
        &self,
        handle: &TargetStageHandle,
        ordinal: u32,
        chunk_index: u32,
        relative_offset: u64,
        predecessor_sha256: Option<&str>,
        chunk_sha256: &str,
        bytes: &[u8],
    ) -> Result<TargetChunkReceipt, ContinuityControlError> {
        self.write_chunk_inner(
            handle,
            ordinal,
            chunk_index,
            relative_offset,
            predecessor_sha256,
            chunk_sha256,
            bytes,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn write_chunk_bounded(
        &self,
        handle: &TargetStageHandle,
        ordinal: u32,
        chunk_index: u32,
        relative_offset: u64,
        predecessor_sha256: Option<&str>,
        chunk_sha256: &str,
        bytes: &[u8],
        guard: &EffectGuard<'_>,
    ) -> Result<TargetChunkReceipt, ContinuityControlError> {
        self.write_chunk_inner(
            handle,
            ordinal,
            chunk_index,
            relative_offset,
            predecessor_sha256,
            chunk_sha256,
            bytes,
            Some(guard),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn write_chunk_inner(
        &self,
        handle: &TargetStageHandle,
        ordinal: u32,
        chunk_index: u32,
        relative_offset: u64,
        predecessor_sha256: Option<&str>,
        chunk_sha256: &str,
        bytes: &[u8],
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<TargetChunkReceipt, ContinuityControlError> {
        check_optional_guard(guard)?;
        self.verify_staging_root_anchor()?;
        let max_chunk_bytes = self.config.bounds.max_frame_bytes.saturating_div(2).max(1);
        if bytes.is_empty()
            || bytes.len() > max_chunk_bytes
            || bytes.len() as u64 > self.config.bounds.max_blob_bytes
            || sha256(bytes) != chunk_sha256
        {
            return Err(ContinuityControlError::FrameBounds);
        }
        validate_digest(chunk_sha256)?;
        if let Some(predecessor) = predecessor_sha256 {
            validate_digest(predecessor)?;
        }
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
        let end = relative_offset
            .checked_add(bytes.len() as u64)
            .ok_or(ContinuityControlError::InvalidBounds)?;
        if end > entry.bytes {
            return Err(ContinuityControlError::InvalidBounds);
        }
        let key = target_chunk_key(ordinal, chunk_index);
        let expected_predecessor = if chunk_index == 0 {
            if relative_offset != 0 {
                return Err(ContinuityControlError::ContentMismatch);
            }
            None
        } else {
            let previous = state
                .received
                .get(&target_chunk_key(ordinal, chunk_index - 1))
                .ok_or(ContinuityControlError::ContentMismatch)?;
            if previous.relative_offset.saturating_add(previous.bytes) != relative_offset {
                return Err(ContinuityControlError::ContentMismatch);
            }
            Some(previous.receipt_sha256.as_str())
        };
        if expected_predecessor != predecessor_sha256 {
            return Err(ContinuityControlError::ContentMismatch);
        }
        let receipt_sha256 = target_chunk_receipt_digest(
            ordinal,
            chunk_index,
            relative_offset,
            bytes.len() as u64,
            chunk_sha256,
            predecessor_sha256,
        );
        let receipt = TargetChunkReceipt {
            ordinal,
            chunk_index,
            relative_offset,
            bytes: bytes.len() as u64,
            chunk_sha256: chunk_sha256.to_owned(),
            predecessor_sha256: predecessor_sha256.map(str::to_owned),
            receipt_sha256,
        };
        if let Some(existing) = state.received.get(&key) {
            if existing == &receipt {
                return Ok(existing.clone());
            }
            return Err(ContinuityControlError::ConflictingRetry);
        }
        validate_leaf_name(&entry.file)?;
        #[cfg(unix)]
        let mut file = {
            let stage_root = open_directory_at(&self._staging_root_handle, &handle.stage_id)?;
            open_file_at(
                &stage_root,
                &entry.file,
                libc::O_CREAT | libc::O_RDWR,
                0o600,
            )?
        };
        #[cfg(not(unix))]
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.stage_path(&handle.stage_id).join(&entry.file))?;
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(ContinuityControlError::UnsafePath);
        }
        use std::io::{Seek, SeekFrom};
        match metadata.len() {
            len if len == relative_offset => {
                check_optional_guard(guard)?;
                file.seek(SeekFrom::Start(relative_offset))?;
                file.write_all(bytes)?;
                file.sync_all()?;
            }
            len if len == end => {
                file.seek(SeekFrom::Start(relative_offset))?;
                let mut existing = vec![0_u8; bytes.len()];
                file.read_exact(&mut existing)?;
                if existing != bytes {
                    return Err(ContinuityControlError::ConflictingRetry);
                }
            }
            _ => return Err(ContinuityControlError::RecoveryRequired),
        }
        check_optional_guard(guard)?;
        state.received.insert(key, receipt.clone());
        self.persist_stage(state)?;
        Ok(receipt)
    }

    /// Convenience used only by local callers that already hold one bounded
    /// complete entry. Production transport uses `write_chunk` explicitly.
    pub fn write_entry(
        &self,
        handle: &TargetStageHandle,
        ordinal: u32,
        bytes: &[u8],
    ) -> Result<(), ContinuityControlError> {
        self.write_chunk(handle, ordinal, 0, 0, None, &sha256(bytes), bytes)
            .map(|_| ())
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
        self.validate_stage_inner(
            handle,
            expected_generation,
            expected_predecessor,
            expected_prefix,
            topology_sha256,
            config_sha256,
            service_schemas,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_stage_bounded(
        &self,
        handle: &TargetStageHandle,
        expected_generation: u64,
        expected_predecessor: Option<&str>,
        expected_prefix: u64,
        topology_sha256: &str,
        config_sha256: &str,
        service_schemas: &BTreeMap<String, String>,
        guard: &EffectGuard<'_>,
    ) -> Result<TargetPossessionEvidence, ContinuityControlError> {
        self.validate_stage_inner(
            handle,
            expected_generation,
            expected_predecessor,
            expected_prefix,
            topology_sha256,
            config_sha256,
            service_schemas,
            Some(guard),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_stage_inner(
        &self,
        handle: &TargetStageHandle,
        expected_generation: u64,
        expected_predecessor: Option<&str>,
        expected_prefix: u64,
        topology_sha256: &str,
        config_sha256: &str,
        service_schemas: &BTreeMap<String, String>,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<TargetPossessionEvidence, ContinuityControlError> {
        check_optional_guard(guard)?;
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
        if &expected != service_schemas {
            return Err(ContinuityControlError::CatalogMismatch);
        }
        for entry in &state.catalog.entries {
            check_optional_guard(guard)?;
            let mut next_offset = 0_u64;
            let mut next_index = 0_u32;
            let mut predecessor: Option<String> = None;
            while let Some(chunk) = state
                .received
                .get(&target_chunk_key(entry.ordinal, next_index))
            {
                if chunk.relative_offset != next_offset
                    || chunk.predecessor_sha256 != predecessor
                    || chunk.receipt_sha256
                        != target_chunk_receipt_digest(
                            chunk.ordinal,
                            chunk.chunk_index,
                            chunk.relative_offset,
                            chunk.bytes,
                            &chunk.chunk_sha256,
                            chunk.predecessor_sha256.as_deref(),
                        )
                {
                    return Err(ContinuityControlError::CatalogMismatch);
                }
                next_offset = next_offset
                    .checked_add(chunk.bytes)
                    .ok_or(ContinuityControlError::InvalidBounds)?;
                predecessor = Some(chunk.receipt_sha256.clone());
                next_index = next_index
                    .checked_add(1)
                    .ok_or(ContinuityControlError::InvalidBounds)?;
            }
            if next_offset != entry.bytes {
                return Err(ContinuityControlError::CatalogMismatch);
            }
            #[cfg(unix)]
            let bytes = {
                let stage_root = open_directory_at(&self._staging_root_handle, &handle.stage_id)?;
                read_bounded_at(&stage_root, &entry.file, entry.bytes)?
            };
            #[cfg(not(unix))]
            let bytes = {
                let path = self.stage_path(&handle.stage_id).join(&entry.file);
                validate_regular_single_link_file(&path, entry.bytes)?;
                read_bounded(&path, entry.bytes)?
            };
            if sha256(&bytes) != entry.sha256 {
                return Err(ContinuityControlError::ContentMismatch);
            }
        }
        let evidence = TargetPossessionEvidence::new(format!(
            "possession:{}:{}",
            handle.stage_id, handle.catalog_sha256
        ));
        check_optional_guard(guard)?;
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
        self.discard_inner(handle, cleanup, None)
    }

    fn discard_bounded(
        &self,
        handle: &TargetStageHandle,
        cleanup: &TargetCleanupPermit,
        guard: &EffectGuard<'_>,
    ) -> Result<TargetDiscardReceipt, ContinuityControlError> {
        self.discard_inner(handle, cleanup, Some(guard))
    }

    fn discard_inner(
        &self,
        handle: &TargetStageHandle,
        cleanup: &TargetCleanupPermit,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<TargetDiscardReceipt, ContinuityControlError> {
        check_optional_guard(guard)?;
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
        check_optional_guard(guard)?;
        state.status = TargetStageStatus::Discarding;
        state.discard = Some(receipt.clone());
        self.persist_stage(state)?;
        let tombstone_name = format!(".discard-{}", handle.stage_id);
        check_optional_guard(guard)?;
        #[cfg(unix)]
        {
            rename_at(
                &self._staging_root_handle,
                &handle.stage_id,
                &self._staging_root_handle,
                &tombstone_name,
            )?;
        }
        #[cfg(not(unix))]
        {
            let path = self.stage_path(&handle.stage_id);
            let tombstone = self.config.staging_dir.join(&tombstone_name);
            if tombstone.exists() {
                return Err(ContinuityControlError::RecoveryRequired);
            }
            fs::rename(&path, &tombstone)?;
            sync_dir(&self.config.staging_dir)?;
        }
        check_optional_guard(guard)?;
        state.status = TargetStageStatus::Discarded;
        self.persist_terminal(state)?;
        #[cfg(unix)]
        {
            let tombstone = open_directory_at(&self._staging_root_handle, &tombstone_name)?;
            for entry in &state.catalog.entries {
                match unlink_at(&tombstone, &entry.file, false) {
                    Ok(()) => {}
                    Err(ContinuityControlError::Io(error))
                        if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => return Err(error),
                }
            }
            unlink_at(&tombstone, "stage.json", false)?;
            drop(tombstone);
            unlink_at(&self._staging_root_handle, &tombstone_name, true)?;
        }
        #[cfg(not(unix))]
        {
            fs::remove_dir_all(self.config.staging_dir.join(&tombstone_name))?;
            sync_dir(&self.config.staging_dir)?;
        }
        Ok(receipt)
    }

    pub fn activate(
        &self,
        handle: &TargetStageHandle,
        possession: &TargetPossessionEvidence,
        cleanup: &TargetCleanupPermit,
        certificate: &MigrationDecisionCertificate,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        self.activate_inner(handle, possession, cleanup, certificate, None)
    }

    fn activate_bounded(
        &self,
        handle: &TargetStageHandle,
        possession: &TargetPossessionEvidence,
        cleanup: &TargetCleanupPermit,
        certificate: &MigrationDecisionCertificate,
        guard: &EffectGuard<'_>,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        self.activate_inner(handle, possession, cleanup, certificate, Some(guard))
    }

    fn activate_inner(
        &self,
        handle: &TargetStageHandle,
        possession: &TargetPossessionEvidence,
        cleanup: &TargetCleanupPermit,
        certificate: &MigrationDecisionCertificate,
        guard: Option<&EffectGuard<'_>>,
    ) -> Result<TargetActivationReceipt, ContinuityControlError> {
        check_optional_guard(guard)?;
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
            || decision.root_generation != handle.root_generation
            || decision.catalog_sha256 != handle.catalog_sha256
            || decision.cleanup_permit_sha256 != cleanup.digest()?
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
        check_optional_guard(guard)?;
        state.status = TargetStageStatus::Activating;
        state.activation = Some(receipt.clone());
        state.activation_decision_sha256 = Some(decision_sha256.clone());
        state.activation_decision = Some(certificate.clone());
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
        check_optional_guard(guard)?;
        #[cfg(unix)]
        write_canonical_at(&self._state_root_handle, "active-target.json", &active)?;
        #[cfg(not(unix))]
        write_canonical_file(&self.config.state_dir.join("active-target.json"), &active)?;
        check_optional_guard(guard)?;
        state.status = TargetStageStatus::Activated;
        self.persist_stage(state)?;
        self.persist_terminal(state)?;
        Ok(receipt)
    }

    fn verify_204_decision(
        &self,
        certificate: &MigrationDecisionCertificate,
    ) -> Result<FinalizedMigrationDecision, ContinuityControlError> {
        verify_204_decision_certificate(&self.config, &self.trusted_decision_keys, certificate)
    }

    #[cfg(not(unix))]
    fn stage_path(&self, stage_id: &str) -> PathBuf {
        self.config.staging_dir.join(stage_id)
    }

    fn persist_stage(&self, state: &TargetStageState) -> Result<(), ContinuityControlError> {
        #[cfg(unix)]
        {
            let stage_root = open_directory_at(&self._staging_root_handle, &state.handle.stage_id)?;
            write_canonical_at(&stage_root, "stage.json", state)
        }
        #[cfg(not(unix))]
        {
            let path = self.stage_path(&state.handle.stage_id);
            write_canonical_file(&path.join("stage.json"), state)?;
            sync_dir(&path)
        }
    }

    fn persist_terminal(&self, state: &TargetStageState) -> Result<(), ContinuityControlError> {
        let leaf = format!("{}.json", state.handle.stage_id);
        #[cfg(unix)]
        {
            write_canonical_at(&self._terminal_root_handle, &leaf, state)
        }
        #[cfg(not(unix))]
        {
            let terminal_root = self.config.state_dir.join("target-results");
            fs::create_dir_all(&terminal_root)?;
            write_canonical_file(&terminal_root.join(leaf), state)?;
            sync_dir(&terminal_root)
        }
    }

    fn verify_staging_root_anchor(&self) -> Result<(), ContinuityControlError> {
        verify_directory_anchor(&self.config.staging_dir, &self._staging_root_handle)
    }
}

fn verify_204_decision_certificate(
    config: &ContinuityControlInitConfig,
    trusted_decision_keys: &BTreeMap<(String, u64), VerifyingKey>,
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
        &certificate.catalog_sha256,
        &certificate.cleanup_permit_sha256,
        &certificate.possession_sha256,
        &certificate.route_cut_sha256,
        &certificate.membership_cut_sha256,
        &certificate.certificate_cut_sha256,
        &certificate.boot_cut_sha256,
        &certificate.lineage_sha256,
    ] {
        validate_digest(digest)?;
    }
    if certificate.trust_domain != config.trust_domain
        || certificate.polis != config.polis
        || certificate.target_node != config.target_node
        || certificate.channel_epoch != config.channel_epoch
    {
        return Err(ContinuityControlError::ActivationDecision);
    }
    let key = trusted_decision_keys
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
        root_generation: certificate.root_generation,
        catalog_sha256: certificate.catalog_sha256.clone(),
        cleanup_permit_sha256: certificate.cleanup_permit_sha256.clone(),
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

fn validate_loaded_stage(
    state: &TargetStageState,
    config: &ContinuityControlInitConfig,
    trusted_catalog_keys: &BTreeMap<(String, u64), VerifyingKey>,
    trusted_decision_keys: &BTreeMap<(String, u64), VerifyingKey>,
    stage_root: Option<&File>,
    stage_root_path: Option<&Path>,
) -> Result<(), ContinuityControlError> {
    #[cfg(unix)]
    let _ = stage_root_path;
    #[cfg(not(unix))]
    let _ = stage_root;
    if state.schema != STAGE_STATE_SCHEMA
        || !safe_identity(&state.handle.stage_id)
        || state.handle.root_generation == 0
        || state.cleanup.stage_id != state.handle.stage_id
        || state.cleanup.root_generation != state.handle.root_generation
        || state.cleanup.channel_epoch != config.channel_epoch
        || state.cleanup.catalog_sha256 != state.handle.catalog_sha256
        || state.cleanup.cleanup_id
            != sha256(
                format!(
                    "cleanup:{}:{}",
                    state.handle.stage_id, state.handle.root_generation
                )
                .as_bytes(),
            )
    {
        return Err(ContinuityControlError::CorruptStage);
    }
    validate_catalog(&state.catalog, &config.bounds)?;
    state.catalog.verify(trusted_catalog_keys)?;
    if state.catalog.digest()? != state.handle.catalog_sha256
        || state.received.len() > config.bounds.max_journal_entries
    {
        return Err(ContinuityControlError::CorruptStage);
    }

    let mut observed_keys = BTreeSet::new();
    for entry in &state.catalog.entries {
        let mut offset = 0_u64;
        let mut chunk_index = 0_u32;
        let mut predecessor = None::<String>;
        while let Some(receipt) = state
            .received
            .get(&target_chunk_key(entry.ordinal, chunk_index))
        {
            let key = target_chunk_key(entry.ordinal, chunk_index);
            observed_keys.insert(key);
            let end = receipt
                .relative_offset
                .checked_add(receipt.bytes)
                .ok_or(ContinuityControlError::CorruptStage)?;
            if receipt.ordinal != entry.ordinal
                || receipt.chunk_index != chunk_index
                || receipt.relative_offset != offset
                || receipt.bytes == 0
                || end > entry.bytes
                || receipt.predecessor_sha256 != predecessor
                || receipt.receipt_sha256
                    != target_chunk_receipt_digest(
                        receipt.ordinal,
                        receipt.chunk_index,
                        receipt.relative_offset,
                        receipt.bytes,
                        &receipt.chunk_sha256,
                        receipt.predecessor_sha256.as_deref(),
                    )
            {
                return Err(ContinuityControlError::CorruptStage);
            }
            validate_digest(&receipt.chunk_sha256)?;
            #[cfg(unix)]
            if let Some(root) = stage_root {
                let bytes =
                    read_range_at(root, &entry.file, receipt.relative_offset, receipt.bytes)?;
                if sha256(&bytes) != receipt.chunk_sha256 {
                    return Err(ContinuityControlError::CorruptStage);
                }
            }
            #[cfg(not(unix))]
            if let Some(root) = stage_root_path {
                let bytes = read_bounded(&root.join(&entry.file), entry.bytes)?;
                let start = usize::try_from(receipt.relative_offset)
                    .map_err(|_| ContinuityControlError::CorruptStage)?;
                let length = usize::try_from(receipt.bytes)
                    .map_err(|_| ContinuityControlError::CorruptStage)?;
                let end = start
                    .checked_add(length)
                    .ok_or(ContinuityControlError::CorruptStage)?;
                if end > bytes.len() || sha256(&bytes[start..end]) != receipt.chunk_sha256 {
                    return Err(ContinuityControlError::CorruptStage);
                }
            }
            offset = end;
            predecessor = Some(receipt.receipt_sha256.clone());
            chunk_index = chunk_index
                .checked_add(1)
                .ok_or(ContinuityControlError::CorruptStage)?;
        }
        if matches!(
            state.status,
            TargetStageStatus::Validated
                | TargetStageStatus::Activating
                | TargetStageStatus::Activated
        ) && offset != entry.bytes
        {
            return Err(ContinuityControlError::CorruptStage);
        }
    }
    if observed_keys.len() != state.received.len() {
        return Err(ContinuityControlError::CorruptStage);
    }

    let expected_possession = TargetPossessionEvidence::new(format!(
        "possession:{}:{}",
        state.handle.stage_id, state.handle.catalog_sha256
    ));
    let expected_discard =
        TargetDiscardReceipt::new(format!("discarded:{}", state.handle.stage_id));
    match state.status {
        TargetStageStatus::Staging => {
            if state.possession.is_some()
                || state.discard.is_some()
                || state.activation.is_some()
                || state.activation_decision.is_some()
                || state.activation_decision_sha256.is_some()
                || state.cleanup_consumed
            {
                return Err(ContinuityControlError::CorruptStage);
            }
        }
        TargetStageStatus::Validated => {
            if state.possession.as_ref() != Some(&expected_possession)
                || state.discard.is_some()
                || state.activation.is_some()
                || state.activation_decision.is_some()
                || state.activation_decision_sha256.is_some()
                || state.cleanup_consumed
            {
                return Err(ContinuityControlError::CorruptStage);
            }
        }
        TargetStageStatus::Discarding | TargetStageStatus::Discarded => {
            if state.discard.as_ref() != Some(&expected_discard)
                || state.activation.is_some()
                || state.activation_decision.is_some()
                || state.activation_decision_sha256.is_some()
                || state.cleanup_consumed
            {
                return Err(ContinuityControlError::CorruptStage);
            }
        }
        TargetStageStatus::Activating | TargetStageStatus::Activated => {
            let decision = state
                .activation_decision
                .as_ref()
                .ok_or(ContinuityControlError::CorruptStage)?;
            verify_204_decision_certificate(config, trusted_decision_keys, decision)?;
            let decision_sha256 = sha256(&decision.unsigned_bytes()?);
            let expected_activation = TargetActivationReceipt::new(format!(
                "activated:{}:{}",
                state.handle.stage_id, decision.decision_id
            ));
            if state.possession.as_ref() != Some(&expected_possession)
                || state.activation.as_ref() != Some(&expected_activation)
                || state.activation_decision_sha256.as_deref() != Some(&decision_sha256)
                || state.discard.is_some()
                || !state.cleanup_consumed
            {
                return Err(ContinuityControlError::CorruptStage);
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn read_range_at(
    parent: &File,
    leaf: &str,
    offset: u64,
    length: u64,
) -> Result<Vec<u8>, ContinuityControlError> {
    use std::io::{Seek, SeekFrom};
    let mut file = open_file_at(parent, leaf, libc::O_RDONLY, 0)?;
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = Vec::new();
    file.take(length.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 != length {
        return Err(ContinuityControlError::CorruptStage);
    }
    Ok(bytes)
}

impl Drop for TargetContinuityEffectPort {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self._lock);
    }
}

/// Compatibility name for callers that predate the role-specific port name.
pub type TargetContinuityCoordinator = TargetContinuityEffectPort;

pub struct ContinuityControlService {
    config: ContinuityControlInitConfig,
    journal: Mutex<DurableContinuityJournal>,
    source: Arc<SourceContinuityEffectPort>,
    target: Arc<TargetContinuityEffectPort>,
    effect_slots: Arc<tokio::sync::Semaphore>,
}

impl ContinuityControlService {
    pub fn new(
        config: ContinuityControlInitConfig,
        journal: DurableContinuityJournal,
        source: Arc<SourceContinuityEffectPort>,
        target: Arc<TargetContinuityEffectPort>,
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
        let permit_window = remaining_effect_window(envelope.operation.deadline_unix_millis)?;
        let _effect_slot = tokio::select! {
            _ = cancellation.cancelled() => return Err(ContinuityControlError::Cancelled),
            permit = tokio::time::timeout(permit_window, self.effect_slots.acquire()) => {
                permit
                    .map_err(|_| ContinuityControlError::Deadline)?
                    .map_err(|_| ContinuityControlError::Cancelled)?
            },
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
        let effect_guard = EffectGuard {
            deadline_unix_millis: effect_deadline,
            cancellation,
        };
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
                let remaining = remaining_effect_window(effect_deadline)?;
                let inner_deadline =
                    std::time::Duration::from_millis(deadline_millis).min(remaining);
                let (quiesce, handle) = self
                    .source
                    .quiesce_and_export(
                        generation,
                        predecessor_sha256,
                        accepted_prefix,
                        topology_sha256,
                        config_sha256,
                        inner_deadline,
                        cancellation,
                    )
                    .await?;
                ContinuityReply::Exported { quiesce, handle }
            }
            ContinuityCommand::ResumeSource { handle } => ContinuityReply::SourceResumed {
                receipt: {
                    check_effect_window(effect_deadline, cancellation)?;
                    self.source
                        .resume_source_bounded(&handle, effect_deadline, cancellation)
                        .await?
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
                let created = self.target.create_stage_bounded(
                    &stage_id,
                    root_generation,
                    catalog,
                    &effect_guard,
                )?;
                ContinuityReply::StageCreated {
                    handle: created.handle,
                    cleanup: created.cleanup,
                }
            }
            ContinuityCommand::WriteTargetChunk {
                handle,
                ordinal,
                chunk_index,
                relative_offset,
                predecessor_sha256,
                chunk_sha256,
                bytes_base64,
            } => {
                check_effect_window(effect_deadline, cancellation)?;
                if bytes_base64.len() > self.config.bounds.max_frame_bytes {
                    return Err(ContinuityControlError::FrameBounds);
                }
                let bytes = BASE64
                    .decode(bytes_base64)
                    .map_err(|_| ContinuityControlError::ContentMismatch)?;
                let receipt = self.target.write_chunk_bounded(
                    &handle,
                    ordinal,
                    chunk_index,
                    relative_offset,
                    predecessor_sha256.as_deref(),
                    &chunk_sha256,
                    &bytes,
                    &effect_guard,
                )?;
                ContinuityReply::ChunkWritten { receipt }
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
                let possession = self.target.validate_stage_bounded(
                    &handle,
                    expected_generation,
                    expected_predecessor.as_deref(),
                    expected_prefix,
                    &topology_sha256,
                    &config_sha256,
                    &service_schemas,
                    &effect_guard,
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
                let receipt = self.target.activate_bounded(
                    &handle,
                    &possession,
                    &cleanup,
                    &decision,
                    &effect_guard,
                )?;
                ContinuityReply::TargetActivated { receipt }
            }
            ContinuityCommand::DiscardTarget { handle, cleanup } => {
                check_effect_window(effect_deadline, cancellation)?;
                let receipt = self
                    .target
                    .discard_bounded(&handle, &cleanup, &effect_guard)?;
                ContinuityReply::TargetDiscarded { receipt }
            }
        };
        effect_guard.check()?;
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
        effect_guard.check()?;
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
                    permit = tokio::time::timeout(
                        PRIVATE_CONTROL_IO_CAP,
                        Arc::clone(&connection_slots).acquire_owned(),
                    ) => {
                        permit
                            .map_err(|_| ContinuityControlError::Deadline)?
                            .map_err(|_| ContinuityControlError::Cancelled)?
                    }
                };
                let acceptor = acceptor.clone();
                let service = service.clone();
                let connection_shutdown = shutdown.child_token();
                tokio::spawn(async move {
                    let _permit = permit;
                    let stream = tokio::select! {
                        _ = connection_shutdown.cancelled() => return,
                        accepted = tokio::time::timeout(
                            PRIVATE_CONTROL_IO_CAP,
                            acceptor.accept(stream),
                        ) => match accepted {
                            Ok(Ok(stream)) => stream,
                            _ => return,
                        },
                    };
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

fn remaining_effect_window(
    deadline_unix_millis: u64,
) -> Result<std::time::Duration, ContinuityControlError> {
    let now = unix_millis()?;
    if now > deadline_unix_millis {
        return Err(ContinuityControlError::Deadline);
    }
    Ok(std::time::Duration::from_millis(
        deadline_unix_millis.saturating_sub(now).max(1),
    ))
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
            result = tokio::time::timeout(PRIVATE_CONTROL_IO_CAP, stream.read_exact(&mut length)) => {
                let result = result.map_err(|_| ContinuityControlError::Deadline)?;
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
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            result = tokio::time::timeout(PRIVATE_CONTROL_IO_CAP, stream.read_exact(&mut bytes)) => {
                result.map_err(|_| ContinuityControlError::Deadline)??;
            }
        }
        let envelope: ContinuityEnvelope =
            decode_canonical(&bytes, service.config.bounds.max_frame_bytes)?;
        let response_deadline = envelope.operation.deadline_unix_millis;
        let reply = if envelope.leaf_spki_sha256 != leaf_spki_sha256 {
            ContinuityWireReply::Denied {
                code: "certificate_identity".to_owned(),
            }
        } else {
            let effect_cancellation = shutdown.child_token();
            let window = remaining_effect_window(response_deadline)?;
            let dispatched = tokio::select! {
                _ = shutdown.cancelled() => return Ok(()),
                result = tokio::time::timeout(
                    window,
                    service.dispatch(
                        envelope,
                        &exporter,
                        unix_millis()?,
                        &effect_cancellation,
                    ),
                ) => match result {
                    Ok(result) => result,
                    Err(_) => {
                        effect_cancellation.cancel();
                        Err(ContinuityControlError::Deadline)
                    }
                },
            };
            match dispatched {
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
        let write_window = remaining_effect_window(response_deadline)?.min(PRIVATE_CONTROL_IO_CAP);
        tokio::select! {
            _ = shutdown.cancelled() => return Ok(()),
            result = tokio::time::timeout(write_window, async {
                stream.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
                stream.write_all(&bytes).await?;
                stream.flush().await
            }) => {
                result.map_err(|_| ContinuityControlError::Deadline)??;
            }
        }
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

fn target_chunk_key(ordinal: u32, chunk_index: u32) -> String {
    format!("{ordinal:010}:{chunk_index:010}")
}

fn target_chunk_receipt_digest(
    ordinal: u32,
    chunk_index: u32,
    relative_offset: u64,
    bytes: u64,
    chunk_sha256: &str,
    predecessor_sha256: Option<&str>,
) -> String {
    sha256(
        format!(
            "target-chunk\0{ordinal}\0{chunk_index}\0{relative_offset}\0{bytes}\0{chunk_sha256}\0{}",
            predecessor_sha256.unwrap_or("")
        )
        .as_bytes(),
    )
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

#[cfg(unix)]
fn leaf_cstring(value: &str) -> Result<std::ffi::CString, ContinuityControlError> {
    validate_leaf_name(value)?;
    std::ffi::CString::new(value.as_bytes()).map_err(|_| ContinuityControlError::UnsafePath)
}

#[cfg(unix)]
fn open_directory_at(parent: &File, leaf: &str) -> Result<File, ContinuityControlError> {
    use std::os::fd::{AsRawFd, FromRawFd};
    let leaf = leaf_cstring(leaf)?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            leaf.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    if !file.metadata()?.is_dir() {
        return Err(ContinuityControlError::UnsafePath);
    }
    Ok(file)
}

#[cfg(not(unix))]
fn open_directory_at(_parent: &File, _leaf: &str) -> Result<File, ContinuityControlError> {
    Err(ContinuityControlError::UnsafePath)
}

#[cfg(unix)]
fn create_directory_at(parent: &File, leaf: &str) -> Result<File, ContinuityControlError> {
    use std::os::fd::AsRawFd;
    let leaf_c = leaf_cstring(leaf)?;
    let result = unsafe { libc::mkdirat(parent.as_raw_fd(), leaf_c.as_ptr(), 0o700) };
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    parent.sync_all()?;
    open_directory_at(parent, leaf)
}

#[cfg(not(unix))]
fn create_directory_at(_parent: &File, _leaf: &str) -> Result<File, ContinuityControlError> {
    Err(ContinuityControlError::UnsafePath)
}

#[cfg(unix)]
fn open_file_at(
    parent: &File,
    leaf: &str,
    flags: libc::c_int,
    mode: libc::mode_t,
) -> Result<File, ContinuityControlError> {
    use std::os::fd::{AsRawFd, FromRawFd};
    let leaf = leaf_cstring(leaf)?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            leaf.as_ptr(),
            flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            mode as libc::c_uint,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(ContinuityControlError::UnsafePath);
    }
    use std::os::unix::fs::MetadataExt;
    if metadata.nlink() != 1 {
        return Err(ContinuityControlError::UnsafePath);
    }
    Ok(file)
}

#[cfg(unix)]
fn unlink_at(parent: &File, leaf: &str, directory: bool) -> Result<(), ContinuityControlError> {
    use std::os::fd::AsRawFd;
    let leaf = leaf_cstring(leaf)?;
    let flags = if directory { libc::AT_REMOVEDIR } else { 0 };
    let result = unsafe { libc::unlinkat(parent.as_raw_fd(), leaf.as_ptr(), flags) };
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    parent.sync_all()?;
    Ok(())
}

#[cfg(unix)]
fn rename_at(
    old_parent: &File,
    old_leaf: &str,
    new_parent: &File,
    new_leaf: &str,
) -> Result<(), ContinuityControlError> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::MetadataExt;
    let old_leaf = leaf_cstring(old_leaf)?;
    let new_leaf = leaf_cstring(new_leaf)?;
    let result = unsafe {
        libc::renameat(
            old_parent.as_raw_fd(),
            old_leaf.as_ptr(),
            new_parent.as_raw_fd(),
            new_leaf.as_ptr(),
        )
    };
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    old_parent.sync_all()?;
    if old_parent.metadata()?.ino() != new_parent.metadata()?.ino() {
        new_parent.sync_all()?;
    }
    Ok(())
}

#[cfg(unix)]
fn write_new_synced_at(
    parent: &File,
    leaf: &str,
    bytes: &[u8],
) -> Result<(), ContinuityControlError> {
    let mut file = open_file_at(
        parent,
        leaf,
        libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
        0o600,
    )?;
    file.write_all(bytes)?;
    file.sync_all()?;
    parent.sync_all()?;
    Ok(())
}

#[cfg(unix)]
fn write_canonical_at<T: Serialize>(
    parent: &File,
    leaf: &str,
    value: &T,
) -> Result<(), ContinuityControlError> {
    let bytes = serde_jcs::to_vec(value)
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
    let pending = format!(".{leaf}.pending");
    match open_file_at(parent, &pending, libc::O_RDONLY, 0) {
        Ok(file) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                if file.metadata()?.nlink() != 1 {
                    return Err(ContinuityControlError::UnsafePath);
                }
            }
            drop(file);
            unlink_at(parent, &pending, false)?;
        }
        Err(ContinuityControlError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    write_new_synced_at(parent, &pending, &bytes)?;
    rename_at(parent, &pending, parent, leaf)
}

#[cfg(unix)]
fn read_bounded_at(
    parent: &File,
    leaf: &str,
    expected_bytes: u64,
) -> Result<Vec<u8>, ContinuityControlError> {
    let file = open_file_at(parent, leaf, libc::O_RDONLY, 0)?;
    let metadata = file.metadata()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(ContinuityControlError::UnsafePath);
        }
    }
    if metadata.len() != expected_bytes {
        return Err(ContinuityControlError::ContentMismatch);
    }
    let mut bytes = Vec::new();
    file.take(expected_bytes.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 != expected_bytes {
        return Err(ContinuityControlError::ContentMismatch);
    }
    Ok(bytes)
}

#[cfg(unix)]
fn read_canonical_at<T: DeserializeOwned + Serialize>(
    parent: &File,
    leaf: &str,
    max_bytes: usize,
) -> Result<T, ContinuityControlError> {
    let file = open_file_at(parent, leaf, libc::O_RDONLY, 0)?;
    let metadata = file.metadata()?;
    if metadata.len() == 0 || metadata.len() > max_bytes as u64 {
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
    file.take(metadata.len().saturating_add(1))
        .read_to_end(&mut bytes)?;
    decode_canonical(&bytes, max_bytes)
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

#[cfg(not(unix))]
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

#[cfg(not(unix))]
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

#[cfg(not(unix))]
fn write_new_synced(path: &Path, bytes: &[u8]) -> Result<(), ContinuityControlError> {
    validate_absolute_nonsymlink_path(path)?;
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
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

#[cfg(not(unix))]
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

#[cfg(not(unix))]
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
