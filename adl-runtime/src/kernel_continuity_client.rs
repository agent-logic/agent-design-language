//! Opaque production Guardian client for the private kernel continuity plane.

use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use adl_runtime_kernel::{
    build_mutual_tls_client_config, decode_canonical, load_identity, load_trust_roots, sha256,
    verify_spki_generation, ContinuityCommand, ContinuityControlError, ContinuityControlInitConfig,
    ContinuityEnvelope, ContinuityOperation, ContinuityReply, ContinuityResultState,
    ContinuityWireReply, RuntimeInitConfig, TlsIdentityPaths, CONTROL_REQUEST_SCHEMA,
    EXPORTER_LABEL, PRIVATE_ALPN,
};
use fs2::FileExt;
use rustls::pki_types::ServerName;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CLIENT_JOURNAL_SCHEMA: &str = "adl.runtime.kernel_continuity.client_journal.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClientAcceptedOperation {
    operation: ContinuityOperation,
    command_sha256: String,
    response: Option<adl_runtime_kernel::ContinuityResponse>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClientJournalState {
    schema: String,
    channel_epoch: u64,
    next_sequence: u64,
    accepted: BTreeMap<String, ClientAcceptedOperation>,
}

struct ClientJournal {
    root: PathBuf,
    _root_handle: File,
    _lock: File,
    state: ClientJournalState,
    max_entries: usize,
    max_frame_bytes: usize,
}

impl ClientJournal {
    fn open(config: &ContinuityControlInitConfig) -> Result<Self, ContinuityControlError> {
        fs::create_dir_all(&config.guardian_state_dir)?;
        let metadata = fs::symlink_metadata(&config.guardian_state_dir)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(ContinuityControlError::UnsafeRoot);
        }
        let lock_path = config.guardian_state_dir.join("writer.lock");
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
        let path = config.guardian_state_dir.join("journal.json");
        let root_handle = File::open(&config.guardian_state_dir)?;
        let state = if path.exists() {
            let value: ClientJournalState = read_state(&path, config.bounds.max_frame_bytes)?;
            if value.schema != CLIENT_JOURNAL_SCHEMA
                || value.channel_epoch != config.channel_epoch
                || value.accepted.len() > config.bounds.max_journal_entries
            {
                return Err(ContinuityControlError::CorruptJournal);
            }
            value
        } else {
            let value = ClientJournalState {
                schema: CLIENT_JOURNAL_SCHEMA.to_owned(),
                channel_epoch: config.channel_epoch,
                next_sequence: 1,
                accepted: BTreeMap::new(),
            };
            write_state(&path, &value)?;
            value
        };
        Ok(Self {
            root: config.guardian_state_dir.clone(),
            _root_handle: root_handle,
            _lock: lock,
            state,
            max_entries: config.bounds.max_journal_entries,
            max_frame_bytes: config.bounds.max_frame_bytes,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn reserve(
        &mut self,
        config: &ContinuityControlInitConfig,
        operation_id: &str,
        command: &ContinuityCommand,
        accepted_prefix: u64,
        deadline_unix_millis: u64,
    ) -> Result<ContinuityOperation, ContinuityControlError> {
        let command_bytes = serde_jcs::to_vec(command)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        let command_sha256 = sha256(&command_bytes);
        if let Some(existing) = self.state.accepted.get(operation_id) {
            if existing.command_sha256 != command_sha256
                || existing.operation.accepted_prefix != accepted_prefix
            {
                return Err(ContinuityControlError::ConflictingRetry);
            }
            return Ok(existing.operation.clone());
        }
        if self.state.accepted.len() >= self.max_entries {
            return Err(ContinuityControlError::JournalCapacity);
        }
        if operation_id.is_empty() || operation_id.len() > 128 || deadline_unix_millis == 0 {
            return Err(ContinuityControlError::InvalidIdentity);
        }
        let sequence = self.state.next_sequence;
        let operation = ContinuityOperation {
            schema: CONTROL_REQUEST_SCHEMA.to_owned(),
            trust_domain: config.trust_domain.clone(),
            polis: config.polis.clone(),
            source_node: config.source_node.clone(),
            target_node: config.target_node.clone(),
            guardian_id: config.guardian_id.clone(),
            kernel_control_id: config.kernel_control_id.clone(),
            channel_epoch: config.channel_epoch,
            sequence,
            operation_id: operation_id.to_owned(),
            kind: command.kind(),
            deadline_unix_millis,
            accepted_prefix,
            payload_sha256: command_sha256.clone(),
        };
        self.state.accepted.insert(
            operation_id.to_owned(),
            ClientAcceptedOperation {
                operation: operation.clone(),
                command_sha256,
                response: None,
            },
        );
        self.state.next_sequence = sequence
            .checked_add(1)
            .ok_or(ContinuityControlError::Sequence)?;
        self.persist()?;
        Ok(operation)
    }

    fn cached(&self, operation_id: &str) -> Option<adl_runtime_kernel::ContinuityResponse> {
        self.state
            .accepted
            .get(operation_id)
            .and_then(|entry| entry.response.clone())
    }

    fn complete(
        &mut self,
        operation_id: &str,
        response: adl_runtime_kernel::ContinuityResponse,
    ) -> Result<(), ContinuityControlError> {
        let entry = self
            .state
            .accepted
            .get_mut(operation_id)
            .ok_or(ContinuityControlError::UnknownOperation)?;
        if response.request_sha256 != entry.operation.digest()? {
            return Err(ContinuityControlError::ConflictingRetry);
        }
        if let Some(existing) = &entry.response {
            if existing != &response {
                return Err(ContinuityControlError::ConflictingRetry);
            }
            return Ok(());
        }
        entry.response = Some(response);
        self.persist()
    }

    fn persist(&self) -> Result<(), ContinuityControlError> {
        verify_directory_anchor(&self.root, &self._root_handle)?;
        let bytes = serde_jcs::to_vec(&self.state)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        if bytes.len() > self.max_frame_bytes {
            return Err(ContinuityControlError::FrameBounds);
        }
        write_state(&self.root.join("journal.json"), &self.state)
    }
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

impl Drop for ClientJournal {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self._lock);
    }
}

/// The only production capability passed from Guardian initialization into the
/// distributed Runtime.  It contains no signing key, path accessor, or public
/// HTTP route.
pub(crate) struct KernelContinuityClient {
    config: ContinuityControlInitConfig,
    tls: Arc<rustls::ClientConfig>,
    successor_tls: Option<Arc<rustls::ClientConfig>>,
    journal: Mutex<ClientJournal>,
    operation_slots: Arc<tokio::sync::Semaphore>,
}

impl KernelContinuityClient {
    pub(crate) async fn from_runtime_init(
        init: &RuntimeInitConfig,
    ) -> Result<Self, ContinuityControlError> {
        let config = init
            .continuity_control
            .clone()
            .ok_or(ContinuityControlError::InvalidAddress)?;
        config.validate(
            init.state_root(),
            &init
                .socket_addrs()
                .map_err(|_| ContinuityControlError::InvalidAddress)?,
        )?;
        let identity = load_identity(&TlsIdentityPaths {
            certificate_chain_path: config.tls.guardian_certificate_chain_path.clone(),
            private_key_path: config.tls.guardian_private_key_path.clone(),
        })
        .await
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        let leaf = identity
            .certificate_chain()
            .first()
            .ok_or(ContinuityControlError::CertificateIdentity)?;
        verify_spki_generation(leaf.as_ref(), &config.tls.guardian_spki_sha256)?;
        let roots = load_trust_roots(&config.tls.guardian_trust_roots_path)
            .await
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        let tls = build_mutual_tls_client_config(identity, roots, PRIVATE_ALPN)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        let successor_tls = if let Some(successor) = &config.tls.successor {
            let identity = load_identity(&TlsIdentityPaths {
                certificate_chain_path: successor.successor_guardian_certificate_chain_path.clone(),
                private_key_path: successor.successor_guardian_private_key_path.clone(),
            })
            .await
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
            let leaf = identity
                .certificate_chain()
                .first()
                .ok_or(ContinuityControlError::CertificateIdentity)?;
            verify_spki_generation(leaf.as_ref(), &successor.successor_spki_sha256)?;
            let roots = load_trust_roots(&config.tls.guardian_trust_roots_path)
                .await
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
            Some(
                build_mutual_tls_client_config(identity, roots, PRIVATE_ALPN)
                    .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
            )
        } else {
            None
        };
        let journal = ClientJournal::open(&config)?;
        Ok(Self {
            operation_slots: Arc::new(tokio::sync::Semaphore::new(config.bounds.max_open_handles)),
            config,
            tls,
            successor_tls,
            journal: Mutex::new(journal),
        })
    }

    pub(crate) async fn establish_attempt(
        &self,
        attempt: u32,
        deadline_unix_millis: u64,
    ) -> Result<(), ContinuityControlError> {
        let operation_id = format!("startup-status-{}-{attempt}", self.config.channel_epoch);
        loop {
            match self
                .execute(
                    &operation_id,
                    ContinuityCommand::Status,
                    0,
                    deadline_unix_millis,
                )
                .await
            {
                Ok(ContinuityReply::Ready) => return Ok(()),
                Ok(_) => return Err(ContinuityControlError::ContentMismatch),
                Err(ContinuityControlError::Io(error))
                    if error.kind() == std::io::ErrorKind::ConnectionRefused
                        && current_unix_millis()? < deadline_unix_millis =>
                {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    async fn execute(
        &self,
        operation_id: &str,
        command: ContinuityCommand,
        accepted_prefix: u64,
        deadline_unix_millis: u64,
    ) -> Result<ContinuityReply, ContinuityControlError> {
        let _operation_slot = self
            .operation_slots
            .acquire()
            .await
            .map_err(|_| ContinuityControlError::Cancelled)?;
        if current_unix_millis()? > deadline_unix_millis {
            return Err(ContinuityControlError::Deadline);
        }
        if let Some(response) = self
            .journal
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .cached(operation_id)
        {
            return Ok(response.result);
        }
        let operation = self
            .journal
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .reserve(
                &self.config,
                operation_id,
                &command,
                accepted_prefix,
                deadline_unix_millis,
            )?;
        let (certificate_generation, leaf_spki_sha256, tls) = self.active_leaf(operation.sequence);
        let address = self.config.socket_addr()?;
        let tcp = tokio::net::TcpStream::connect(address).await?;
        let server_name = ServerName::try_from(self.config.tls.server_name.clone())
            .map_err(|_| ContinuityControlError::CertificateIdentity)?;
        let connector = tokio_rustls::TlsConnector::from(tls);
        let mut stream = connector
            .connect(server_name, tcp)
            .await
            .map_err(|_| ContinuityControlError::CertificateIdentity)?;
        let exporter = {
            let (_, connection) = stream.get_ref();
            let certificate = connection
                .peer_certificates()
                .and_then(|chain| chain.first())
                .ok_or(ContinuityControlError::CertificateIdentity)?;
            verify_spki_generation(certificate.as_ref(), &self.config.tls.server_spki_sha256)?;
            let mut exporter = [0_u8; 32];
            connection
                .export_keying_material(&mut exporter, EXPORTER_LABEL, None)
                .map_err(|_| ContinuityControlError::ExporterBinding)?;
            exporter
        };
        let envelope = ContinuityEnvelope {
            exporter_sha256: sha256(&exporter),
            certificate_generation,
            leaf_spki_sha256,
            operation,
            command,
        };
        let bytes = serde_jcs::to_vec(&envelope)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
        if bytes.len() > self.config.bounds.max_frame_bytes {
            return Err(ContinuityControlError::FrameBounds);
        }
        stream
            .write_all(&(bytes.len() as u32).to_be_bytes())
            .await?;
        stream.write_all(&bytes).await?;
        stream.flush().await?;
        let mut length = [0_u8; 4];
        stream.read_exact(&mut length).await?;
        let length = u32::from_be_bytes(length) as usize;
        if length == 0 || length > self.config.bounds.max_frame_bytes {
            return Err(ContinuityControlError::FrameBounds);
        }
        let mut bytes = vec![0_u8; length];
        stream.read_exact(&mut bytes).await?;
        let reply: ContinuityWireReply =
            decode_canonical(&bytes, self.config.bounds.max_frame_bytes)?;
        let response = match reply {
            ContinuityWireReply::Completed { response } => *response,
            ContinuityWireReply::Denied { code } => {
                return Err(ContinuityControlError::Encoding(format!(
                    "private continuity denied:{code}"
                )))
            }
        };
        validate_response(&response)?;
        let result = response.result.clone();
        self.journal
            .lock()
            .map_err(|_| ContinuityControlError::CorruptJournal)?
            .complete(operation_id, response)?;
        Ok(result)
    }

    fn active_leaf(&self, sequence: u64) -> (u64, String, Arc<rustls::ClientConfig>) {
        if let Some(successor) = &self.config.tls.successor {
            if sequence >= successor.activation_sequence {
                return (
                    successor.successor_generation,
                    successor.successor_spki_sha256.clone(),
                    self.successor_tls
                        .as_ref()
                        .expect("validated successor identity")
                        .clone(),
                );
            }
        }
        (
            self.config.tls.certificate_generation,
            self.config.tls.guardian_spki_sha256.clone(),
            self.tls.clone(),
        )
    }
}

fn current_unix_millis() -> Result<u64, ContinuityControlError> {
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

fn validate_response(
    response: &adl_runtime_kernel::ContinuityResponse,
) -> Result<(), ContinuityControlError> {
    if response.state != ContinuityResultState::Completed {
        return Err(ContinuityControlError::ReconcileRequired);
    }
    let mut unsigned = response.clone();
    let expected = unsigned.response_sha256.clone();
    unsigned.response_sha256.clear();
    let actual = sha256(
        &serde_jcs::to_vec(&unsigned)
            .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
    );
    if actual != expected {
        return Err(ContinuityControlError::ContentMismatch);
    }
    Ok(())
}

fn write_state<T: Serialize>(path: &Path, value: &T) -> Result<(), ContinuityControlError> {
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
    File::open(path.parent().ok_or(ContinuityControlError::UnsafePath)?)?.sync_all()?;
    Ok(())
}

fn read_state<T: for<'de> Deserialize<'de> + Serialize>(
    path: &Path,
    max_bytes: usize,
) -> Result<T, ContinuityControlError> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > max_bytes as u64 {
        return Err(ContinuityControlError::UnsafePath);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(ContinuityControlError::UnsafePath);
        }
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    use std::io::Read as _;
    file.take(metadata.len().saturating_add(1))
        .read_to_end(&mut bytes)?;
    decode_canonical(&bytes, max_bytes)
}
