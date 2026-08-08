use std::{
    fmt,
    io::Cursor,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use prost::Message;
use quinn::{
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
    Connection, Endpoint,
};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore,
};
use sha2::{Digest, Sha256};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::certificates::{CertificatePurpose, DistributedCertificateStore};

pub const TRANSPORT_SCHEMA: &str = "adl.distributed.transport_envelope.v1";
pub const TRANSPORT_ALPN: &[u8] = b"adl-guardian/1";
const CLOSE_CODE: u32 = 0x100;
const MAX_TEXT_LEN: usize = 128;
const LENGTH_PREFIX_SLACK: usize = 10;
const REPLAY_WINDOW_BITS: u64 = 64;

#[derive(Clone, PartialEq, Message)]
pub struct TransportEnvelope {
    #[prost(string, tag = "1")]
    pub schema: String,
    #[prost(string, tag = "2")]
    pub trust_domain: String,
    #[prost(string, tag = "3")]
    pub node_id: String,
    #[prost(string, tag = "4")]
    pub guardian_id: String,
    #[prost(uint32, tag = "5")]
    pub protocol_version: u32,
    #[prost(uint64, tag = "6")]
    pub certificate_generation: u64,
    #[prost(uint64, tag = "7")]
    pub sequence: u64,
    #[prost(bytes = "vec", tag = "8")]
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerBinding {
    pub leaf_certificate_sha256: [u8; 32],
    pub trust_domain: String,
    pub node_id: String,
    pub guardian_id: String,
    pub protocol_version: u32,
    pub certificate_generation: u64,
}

impl PeerBinding {
    pub fn new(
        leaf_certificate: &CertificateDer<'_>,
        trust_domain: impl Into<String>,
        node_id: impl Into<String>,
        guardian_id: impl Into<String>,
        protocol_version: u32,
        certificate_generation: u64,
    ) -> TransportResult<Self> {
        let binding = Self {
            leaf_certificate_sha256: Sha256::digest(leaf_certificate.as_ref()).into(),
            trust_domain: trust_domain.into(),
            node_id: node_id.into(),
            guardian_id: guardian_id.into(),
            protocol_version,
            certificate_generation,
        };
        binding.validate()?;
        Ok(binding)
    }

    fn validate(&self) -> TransportResult<()> {
        validate_text(&self.trust_domain)?;
        validate_text(&self.node_id)?;
        validate_text(&self.guardian_id)?;
        if self.protocol_version == 0 || self.certificate_generation == 0 {
            return Err(TransportError::InvalidPeerBinding);
        }
        Ok(())
    }

    fn envelope(&self, sequence: u64, payload: Vec<u8>) -> TransportEnvelope {
        TransportEnvelope {
            schema: TRANSPORT_SCHEMA.to_owned(),
            trust_domain: self.trust_domain.clone(),
            node_id: self.node_id.clone(),
            guardian_id: self.guardian_id.clone(),
            protocol_version: self.protocol_version,
            certificate_generation: self.certificate_generation,
            sequence,
            payload,
        }
    }
}

#[derive(Clone)]
pub struct TransportAuthorization {
    store: Arc<DistributedCertificateStore>,
    holder_id: String,
    generation: u64,
    certificate_id: String,
}

impl TransportAuthorization {
    pub fn new(
        store: Arc<DistributedCertificateStore>,
        holder_id: impl Into<String>,
        generation: u64,
    ) -> TransportResult<Self> {
        let holder_id = holder_id.into();
        let verified = store
            .authorize(
                &holder_id,
                CertificatePurpose::Transport,
                generation,
                unix_time()?,
            )
            .map_err(|_| TransportError::CertificateAuthorization)?;
        Ok(Self {
            store,
            holder_id,
            generation,
            certificate_id: verified.certificate_id,
        })
    }

    fn revalidate(&self) -> TransportResult<u64> {
        let verified = self
            .store
            .authorize(
                &self.holder_id,
                CertificatePurpose::Transport,
                self.generation,
                unix_time()?,
            )
            .map_err(|_| TransportError::CertificateAuthorization)?;
        if verified.certificate_id != self.certificate_id
            || verified.holder_id != self.holder_id
            || verified.purpose != CertificatePurpose::Transport
            || verified.generation != self.generation
        {
            return Err(TransportError::CertificateAuthorization);
        }
        Ok(verified.authorization_deadline_unix_secs)
    }

    fn validate_binding(&self, binding: &PeerBinding) -> TransportResult<u64> {
        if self.holder_id != binding.node_id || self.generation != binding.certificate_generation {
            return Err(TransportError::CertificateAuthorization);
        }
        self.revalidate()
    }
}

#[derive(Default)]
struct ReplayWindow {
    highest: u64,
    seen: u64,
}

impl ReplayWindow {
    fn observe(&mut self, sequence: u64) -> TransportResult<()> {
        if sequence == 0 {
            return Err(TransportError::SequenceInvalid);
        }
        if sequence > self.highest {
            let shift = sequence - self.highest;
            self.seen = if shift >= REPLAY_WINDOW_BITS {
                1
            } else {
                (self.seen << shift) | 1
            };
            self.highest = sequence;
            return Ok(());
        }
        let distance = self.highest - sequence;
        if distance >= REPLAY_WINDOW_BITS || self.seen & (1_u64 << distance) != 0 {
            return Err(TransportError::ReplayDetected);
        }
        self.seen |= 1_u64 << distance;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct TransportLimits {
    pub max_frame_bytes: usize,
    pub max_concurrent_uni_streams: u32,
    pub idle_timeout: Duration,
    pub authorization_lifetime: Duration,
}

impl TransportLimits {
    pub fn bounded(
        max_frame_bytes: usize,
        max_concurrent_uni_streams: u32,
        idle_timeout: Duration,
        authorization_lifetime: Duration,
    ) -> TransportResult<Self> {
        if !(64..=16 * 1024 * 1024).contains(&max_frame_bytes)
            || max_concurrent_uni_streams == 0
            || max_concurrent_uni_streams > 1024
            || idle_timeout.is_zero()
            || authorization_lifetime.is_zero()
        {
            return Err(TransportError::InvalidLimits);
        }
        Ok(Self {
            max_frame_bytes,
            max_concurrent_uni_streams,
            idle_timeout,
            authorization_lifetime,
        })
    }
}

pub struct ConnectionSecurity {
    local: PeerBinding,
    expected_peer: PeerBinding,
    local_authorization: TransportAuthorization,
    peer_authorization: TransportAuthorization,
    limits: TransportLimits,
    cancellation: CancellationToken,
}

impl ConnectionSecurity {
    pub fn new(
        local: PeerBinding,
        expected_peer: PeerBinding,
        local_authorization: TransportAuthorization,
        peer_authorization: TransportAuthorization,
        limits: TransportLimits,
        cancellation: CancellationToken,
    ) -> TransportResult<Self> {
        local.validate()?;
        expected_peer.validate()?;
        Ok(Self {
            local,
            expected_peer,
            local_authorization,
            peer_authorization,
            limits,
            cancellation,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransportError {
    InvalidLimits,
    InvalidPeerBinding,
    InvalidTlsMaterial,
    TlsConfiguration,
    Endpoint,
    Connection,
    PeerCertificateMissing,
    PeerCertificateMismatch,
    CertificateAuthorization,
    FrameTooLarge,
    MalformedFrame,
    PeerIdentityMismatch,
    SequenceInvalid,
    ReplayDetected,
    Cancelled,
    AuthorizationExpired,
    Stream,
}

impl TransportError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidLimits => "invalid_limits",
            Self::InvalidPeerBinding => "invalid_peer_binding",
            Self::InvalidTlsMaterial => "invalid_tls_material",
            Self::TlsConfiguration => "tls_configuration_error",
            Self::Endpoint => "endpoint_error",
            Self::Connection => "connection_error",
            Self::PeerCertificateMissing => "peer_certificate_missing",
            Self::PeerCertificateMismatch => "peer_certificate_mismatch",
            Self::CertificateAuthorization => "certificate_authorization_failed",
            Self::FrameTooLarge => "frame_too_large",
            Self::MalformedFrame => "malformed_frame",
            Self::PeerIdentityMismatch => "peer_identity_mismatch",
            Self::SequenceInvalid => "sequence_invalid",
            Self::ReplayDetected => "replay_detected",
            Self::Cancelled => "cancelled",
            Self::AuthorizationExpired => "authorization_expired",
            Self::Stream => "stream_error",
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for TransportError {}

pub type TransportResult<T> = Result<T, TransportError>;

pub fn server_endpoint(
    bind: SocketAddr,
    certificate_chain: Vec<CertificateDer<'static>>,
    private_key: PrivateKeyDer<'static>,
    client_roots: &[CertificateDer<'static>],
    limits: &TransportLimits,
) -> TransportResult<Endpoint> {
    if certificate_chain.is_empty() || client_roots.is_empty() {
        return Err(TransportError::InvalidTlsMaterial);
    }
    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    let roots = roots(client_roots)?;
    let verifier = WebPkiClientVerifier::builder_with_provider(Arc::new(roots), provider.clone())
        .build()
        .map_err(|_| TransportError::TlsConfiguration)?;
    let mut tls = rustls::ServerConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|_| TransportError::TlsConfiguration)?
        .with_client_cert_verifier(verifier)
        .with_single_cert(certificate_chain, private_key)
        .map_err(|_| TransportError::InvalidTlsMaterial)?;
    tls.alpn_protocols = vec![TRANSPORT_ALPN.to_vec()];
    let crypto = QuicServerConfig::try_from(tls).map_err(|_| TransportError::TlsConfiguration)?;
    let mut config = quinn::ServerConfig::with_crypto(Arc::new(crypto));
    config.transport_config(transport_config(limits)?);
    Endpoint::server(config, bind).map_err(|_| TransportError::Endpoint)
}

pub fn client_endpoint(
    bind: SocketAddr,
    certificate_chain: Vec<CertificateDer<'static>>,
    private_key: PrivateKeyDer<'static>,
    server_roots: &[CertificateDer<'static>],
    limits: &TransportLimits,
) -> TransportResult<Endpoint> {
    if certificate_chain.is_empty() || server_roots.is_empty() {
        return Err(TransportError::InvalidTlsMaterial);
    }
    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    let mut tls = rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|_| TransportError::TlsConfiguration)?
        .with_root_certificates(roots(server_roots)?)
        .with_client_auth_cert(certificate_chain, private_key)
        .map_err(|_| TransportError::InvalidTlsMaterial)?;
    tls.alpn_protocols = vec![TRANSPORT_ALPN.to_vec()];
    let crypto = QuicClientConfig::try_from(tls).map_err(|_| TransportError::TlsConfiguration)?;
    let mut config = quinn::ClientConfig::new(Arc::new(crypto));
    config.transport_config(transport_config(limits)?);
    let mut endpoint = Endpoint::client(bind).map_err(|_| TransportError::Endpoint)?;
    endpoint.set_default_client_config(config);
    Ok(endpoint)
}

fn roots(certificates: &[CertificateDer<'static>]) -> TransportResult<RootCertStore> {
    let mut roots = RootCertStore::empty();
    for certificate in certificates {
        roots
            .add(certificate.clone())
            .map_err(|_| TransportError::InvalidTlsMaterial)?;
    }
    Ok(roots)
}

fn transport_config(limits: &TransportLimits) -> TransportResult<Arc<quinn::TransportConfig>> {
    let mut config = quinn::TransportConfig::default();
    config.max_concurrent_uni_streams(limits.max_concurrent_uni_streams.into());
    config.max_concurrent_bidi_streams(0_u8.into());
    config
        .max_idle_timeout(Some(
            limits
                .idle_timeout
                .try_into()
                .map_err(|_| TransportError::InvalidLimits)?,
        ))
        .keep_alive_interval(Some(limits.idle_timeout / 2));
    Ok(Arc::new(config))
}

pub struct AuthenticatedConnection {
    connection: Connection,
    local: PeerBinding,
    expected_peer: PeerBinding,
    local_authorization: TransportAuthorization,
    peer_authorization: TransportAuthorization,
    limits: TransportLimits,
    authorization_deadline: Instant,
    cancellation: CancellationToken,
    replay_window: Mutex<ReplayWindow>,
}

impl AuthenticatedConnection {
    pub async fn connect(
        endpoint: &Endpoint,
        remote: SocketAddr,
        server_name: &str,
        security: ConnectionSecurity,
    ) -> TransportResult<Self> {
        validate_text(server_name)?;
        let cancellation = security.cancellation.clone();
        let connecting = endpoint
            .connect(remote, server_name)
            .map_err(|_| TransportError::Connection)?;
        let connection = tokio::select! {
            _ = cancellation.cancelled() => return Err(TransportError::Cancelled),
            result = connecting => result.map_err(|_| TransportError::Connection)?,
        };
        Self::from_connection(connection, security)
    }

    pub async fn accept(
        endpoint: &Endpoint,
        security: ConnectionSecurity,
    ) -> TransportResult<Self> {
        let cancellation = security.cancellation.clone();
        let incoming = tokio::select! {
            _ = cancellation.cancelled() => return Err(TransportError::Cancelled),
            result = endpoint.accept() => result.ok_or(TransportError::Connection)?,
        };
        let connection = tokio::select! {
            _ = cancellation.cancelled() => return Err(TransportError::Cancelled),
            result = incoming => result.map_err(|_| TransportError::Connection)?,
        };
        Self::from_connection(connection, security)
    }

    fn from_connection(
        connection: Connection,
        security: ConnectionSecurity,
    ) -> TransportResult<Self> {
        let ConnectionSecurity {
            local,
            expected_peer,
            local_authorization,
            peer_authorization,
            limits,
            cancellation,
        } = security;
        local.validate()?;
        expected_peer.validate()?;
        verify_peer_certificate(&connection, &expected_peer)?;
        let local_deadline = local_authorization.validate_binding(&local)?;
        let peer_deadline = peer_authorization.validate_binding(&expected_peer)?;
        let authorization_deadline = bounded_deadline(
            limits.authorization_lifetime,
            local_deadline.min(peer_deadline),
        )?;
        Ok(Self {
            connection,
            local,
            expected_peer,
            local_authorization,
            peer_authorization,
            limits,
            authorization_deadline,
            cancellation,
            replay_window: Mutex::new(ReplayWindow::default()),
        })
    }

    pub async fn send(&self, sequence: u64, payload: Vec<u8>) -> TransportResult<()> {
        self.require_authority()?;
        if sequence == 0 {
            return Err(TransportError::SequenceInvalid);
        }
        let frame = encode_frame(self.local.envelope(sequence, payload), &self.limits)?;
        let mut stream = tokio::select! {
            _ = self.cancellation.cancelled() => return self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => return self.expire(),
            result = self.connection.open_uni() => result.map_err(|_| TransportError::Stream)?,
        };
        tokio::select! {
            _ = self.cancellation.cancelled() => return self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => return self.expire(),
            result = stream.write_all(&frame) => result.map_err(|_| TransportError::Stream)?,
        }
        stream.finish().map_err(|_| TransportError::Stream)
    }

    pub async fn receive(&self) -> TransportResult<TransportEnvelope> {
        self.require_authority()?;
        let mut stream = tokio::select! {
            _ = self.cancellation.cancelled() => return self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => return self.expire(),
            result = self.connection.accept_uni() => result.map_err(|_| TransportError::Stream)?,
        };
        let limit = self
            .limits
            .max_frame_bytes
            .checked_add(LENGTH_PREFIX_SLACK)
            .ok_or(TransportError::InvalidLimits)?;
        let bytes = tokio::select! {
            _ = self.cancellation.cancelled() => return self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => return self.expire(),
            result = stream.read_to_end(limit) => result.map_err(|_| TransportError::FrameTooLarge)?,
        };
        let envelope = decode_frame(&bytes, &self.limits)?;
        verify_envelope(&envelope, &self.expected_peer)?;
        self.replay_window
            .lock()
            .map_err(|_| TransportError::CertificateAuthorization)?
            .observe(envelope.sequence)?;
        Ok(envelope)
    }

    pub fn close(&self) {
        self.connection
            .close(CLOSE_CODE.into(), b"adl transport closed");
    }

    fn require_authority(&self) -> TransportResult<()> {
        if self.cancellation.is_cancelled() {
            return self.cancel();
        }
        if Instant::now() >= self.authorization_deadline {
            self.connection
                .close(CLOSE_CODE.into(), b"authorization expired");
            return Err(TransportError::AuthorizationExpired);
        }
        let (local_deadline, peer_deadline) = match (
            self.local_authorization.validate_binding(&self.local),
            self.peer_authorization
                .validate_binding(&self.expected_peer),
        ) {
            (Ok(local), Ok(peer)) => (local, peer),
            _ => {
                self.connection
                    .close(CLOSE_CODE.into(), b"certificate authorization failed");
                return Err(TransportError::CertificateAuthorization);
            }
        };
        if unix_time()? >= local_deadline.min(peer_deadline) {
            return self.expire();
        }
        Ok(())
    }

    fn cancel<T>(&self) -> TransportResult<T> {
        self.connection.close(CLOSE_CODE.into(), b"cancelled");
        Err(TransportError::Cancelled)
    }

    fn expire<T>(&self) -> TransportResult<T> {
        self.connection
            .close(CLOSE_CODE.into(), b"authorization expired");
        Err(TransportError::AuthorizationExpired)
    }
}

fn verify_peer_certificate(connection: &Connection, expected: &PeerBinding) -> TransportResult<()> {
    let identity = connection
        .peer_identity()
        .ok_or(TransportError::PeerCertificateMissing)?;
    let certificates = identity
        .downcast::<Vec<CertificateDer<'static>>>()
        .map_err(|_| TransportError::PeerCertificateMissing)?;
    let leaf = certificates
        .first()
        .ok_or(TransportError::PeerCertificateMissing)?;
    let digest: [u8; 32] = Sha256::digest(leaf.as_ref()).into();
    if digest != expected.leaf_certificate_sha256 {
        connection.close(CLOSE_CODE.into(), b"peer certificate mismatch");
        return Err(TransportError::PeerCertificateMismatch);
    }
    Ok(())
}

pub fn encode_frame(
    envelope: TransportEnvelope,
    limits: &TransportLimits,
) -> TransportResult<Vec<u8>> {
    validate_envelope_shape(&envelope, limits)?;
    if envelope.encoded_len() > limits.max_frame_bytes {
        return Err(TransportError::FrameTooLarge);
    }
    let mut bytes = Vec::with_capacity(envelope.encoded_len() + LENGTH_PREFIX_SLACK);
    envelope
        .encode_length_delimited(&mut bytes)
        .map_err(|_| TransportError::MalformedFrame)?;
    if bytes.len() > limits.max_frame_bytes + LENGTH_PREFIX_SLACK {
        return Err(TransportError::FrameTooLarge);
    }
    Ok(bytes)
}

pub fn decode_frame(bytes: &[u8], limits: &TransportLimits) -> TransportResult<TransportEnvelope> {
    if bytes.len() > limits.max_frame_bytes + LENGTH_PREFIX_SLACK {
        return Err(TransportError::FrameTooLarge);
    }
    let mut cursor = Cursor::new(bytes);
    let envelope = TransportEnvelope::decode_length_delimited(&mut cursor)
        .map_err(|_| TransportError::MalformedFrame)?;
    if cursor.position() as usize != bytes.len() {
        return Err(TransportError::MalformedFrame);
    }
    validate_envelope_shape(&envelope, limits)?;
    Ok(envelope)
}

fn validate_envelope_shape(
    envelope: &TransportEnvelope,
    limits: &TransportLimits,
) -> TransportResult<()> {
    if envelope.schema != TRANSPORT_SCHEMA
        || envelope.protocol_version == 0
        || envelope.certificate_generation == 0
        || envelope.sequence == 0
    {
        return Err(TransportError::MalformedFrame);
    }
    validate_text(&envelope.trust_domain).map_err(|_| TransportError::MalformedFrame)?;
    validate_text(&envelope.node_id).map_err(|_| TransportError::MalformedFrame)?;
    validate_text(&envelope.guardian_id).map_err(|_| TransportError::MalformedFrame)?;
    if envelope.payload.len() > limits.max_frame_bytes {
        return Err(TransportError::FrameTooLarge);
    }
    if envelope.encoded_len() > limits.max_frame_bytes {
        return Err(TransportError::FrameTooLarge);
    }
    Ok(())
}

fn unix_time() -> TransportResult<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| TransportError::CertificateAuthorization)
        .map(|duration| duration.as_secs())
}

fn bounded_deadline(
    configured_lifetime: Duration,
    certificate_deadline_unix_secs: u64,
) -> TransportResult<Instant> {
    let certificate_remaining = certificate_deadline_unix_secs
        .checked_sub(unix_time()?)
        .ok_or(TransportError::CertificateAuthorization)?;
    Instant::now()
        .checked_add(configured_lifetime.min(Duration::from_secs(certificate_remaining)))
        .ok_or(TransportError::InvalidLimits)
}

fn verify_envelope(envelope: &TransportEnvelope, expected: &PeerBinding) -> TransportResult<()> {
    if envelope.trust_domain != expected.trust_domain
        || envelope.node_id != expected.node_id
        || envelope.guardian_id != expected.guardian_id
        || envelope.protocol_version != expected.protocol_version
        || envelope.certificate_generation != expected.certificate_generation
    {
        return Err(TransportError::PeerIdentityMismatch);
    }
    Ok(())
}

fn validate_text(value: &str) -> TransportResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_LEN
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(TransportError::InvalidPeerBinding);
    }
    Ok(())
}
