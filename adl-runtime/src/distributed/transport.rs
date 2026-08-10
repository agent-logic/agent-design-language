use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    io::Cursor,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use prost::Message;
use quinn::{
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
    Connection, Endpoint,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use sha2::{Digest, Sha256};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::{
    certificates::{AuthorityCertificate, CertificatePurpose, DistributedCertificateStore},
    lease::{AuthorityMembership, ControlCertificatePurpose},
    membership::{MemberRole, MembershipState},
};
use adl_runtime_kernel::tls::{
    build_mutual_tls_client_config, build_mutual_tls_server_config, trust_roots_from_der,
    TlsIdentity,
};

pub const TRANSPORT_SCHEMA: &str = "adl.distributed.transport_envelope.v1";
pub const TRANSPORT_ALPN: &[u8] = b"adl-guardian/1";
const CLOSE_CODE: u32 = 0x100;
const MAX_TEXT_LEN: usize = 128;
const LENGTH_PREFIX_SLACK: usize = 10;
const REPLAY_WINDOW_BITS: u64 = 64;
const POLIS_HANDSHAKE_SCHEMA: &str = "adl.distributed.polis_handshake.v1";

pub const POLIS_TRANSPORT_SCHEMA: &str = "adl.distributed.polis_transport.v1";
pub const POLIS_TRANSPORT_RESPONSE_SCHEMA: &str = "adl.distributed.polis_transport_response.v1";

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

#[derive(Clone, PartialEq, Message)]
struct PolisTransportEnvelope {
    #[prost(string, tag = "1")]
    schema: String,
    #[prost(string, tag = "2")]
    polis_id: String,
    #[prost(string, tag = "3")]
    trust_domain: String,
    #[prost(string, tag = "4")]
    sender_node_id: String,
    #[prost(string, tag = "5")]
    receiver_node_id: String,
    #[prost(uint64, tag = "6")]
    certificate_generation: u64,
    #[prost(uint64, tag = "7")]
    boot_generation: u64,
    #[prost(uint64, tag = "8")]
    committed_membership_index: u64,
    #[prost(uint64, tag = "9")]
    sequence: u64,
    #[prost(string, tag = "10")]
    message_kind: String,
    #[prost(bytes = "vec", tag = "11")]
    payload_sha256: Vec<u8>,
    #[prost(bytes = "vec", tag = "12")]
    payload: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct PolisTransportResponse {
    #[prost(string, tag = "1")]
    schema: String,
    #[prost(uint64, tag = "2")]
    sequence: u64,
    #[prost(bytes = "vec", tag = "3")]
    request_sha256: Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    payload_sha256: Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    payload: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct PolisHandshake {
    #[prost(string, tag = "1")]
    schema: String,
    #[prost(string, tag = "2")]
    polis_id: String,
    #[prost(string, tag = "3")]
    trust_domain: String,
    #[prost(string, tag = "4")]
    sender_node_id: String,
    #[prost(string, tag = "5")]
    receiver_node_id: String,
    #[prost(uint64, tag = "6")]
    sender_certificate_generation: u64,
    #[prost(uint64, tag = "7")]
    receiver_certificate_generation: u64,
    #[prost(uint64, tag = "8")]
    sender_boot_generation: u64,
    #[prost(uint64, tag = "9")]
    receiver_boot_generation: u64,
    #[prost(uint64, tag = "10")]
    committed_membership_index: u64,
    #[prost(bytes = "vec", tag = "11")]
    signature: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolisSessionBinding {
    polis_id: String,
    trust_domain: String,
    local_node_id: String,
    peer_node_id: String,
    local_guardian_id: String,
    peer_guardian_id: String,
    local_certificate_generation: u64,
    peer_certificate_generation: u64,
    local_boot_generation: u64,
    peer_boot_generation: u64,
    committed_membership_index: u64,
    local_control_public_key: [u8; 32],
    peer_control_public_key: [u8; 32],
}

impl PolisSessionBinding {
    fn validate(&self) -> TransportResult<()> {
        validate_text(&self.polis_id)?;
        validate_text(&self.trust_domain)?;
        validate_text(&self.local_node_id)?;
        validate_text(&self.peer_node_id)?;
        validate_text(&self.local_guardian_id)?;
        validate_text(&self.peer_guardian_id)?;
        if self.local_node_id == self.peer_node_id
            || self.local_certificate_generation == 0
            || self.peer_certificate_generation == 0
            || self.local_boot_generation == 0
            || self.peer_boot_generation == 0
            || self.committed_membership_index == 0
            || self.local_control_public_key == [0; 32]
            || self.peer_control_public_key == [0; 32]
            || VerifyingKey::from_bytes(&self.local_control_public_key).is_err()
            || VerifyingKey::from_bytes(&self.peer_control_public_key).is_err()
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(())
    }

    fn reverse(&self) -> Self {
        Self {
            polis_id: self.polis_id.clone(),
            trust_domain: self.trust_domain.clone(),
            local_node_id: self.peer_node_id.clone(),
            peer_node_id: self.local_node_id.clone(),
            local_guardian_id: self.peer_guardian_id.clone(),
            peer_guardian_id: self.local_guardian_id.clone(),
            local_certificate_generation: self.peer_certificate_generation,
            peer_certificate_generation: self.local_certificate_generation,
            local_boot_generation: self.peer_boot_generation,
            peer_boot_generation: self.local_boot_generation,
            committed_membership_index: self.committed_membership_index,
            local_control_public_key: self.peer_control_public_key,
            peer_control_public_key: self.local_control_public_key,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn from_authority(
        polis_id: String,
        trust_domain: String,
        local_node_id: String,
        peer_node_id: String,
        local_guardian_id: String,
        peer_guardian_id: String,
        local_certificate_generation: u64,
        peer_certificate_generation: u64,
        local_boot_generation: u64,
        peer_boot_generation: u64,
        committed_membership_index: u64,
        local_control_public_key: [u8; 32],
        peer_control_public_key: [u8; 32],
    ) -> TransportResult<Self> {
        let binding = Self {
            polis_id,
            trust_domain,
            local_node_id,
            peer_node_id,
            local_guardian_id,
            peer_guardian_id,
            local_certificate_generation,
            peer_certificate_generation,
            local_boot_generation,
            peer_boot_generation,
            committed_membership_index,
            local_control_public_key,
            peer_control_public_key,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn local_node_id(&self) -> &str {
        &self.local_node_id
    }
    pub fn peer_node_id(&self) -> &str {
        &self.peer_node_id
    }
    pub fn peer_certificate_generation(&self) -> u64 {
        self.peer_certificate_generation
    }
    pub fn peer_boot_generation(&self) -> u64 {
        self.peer_boot_generation
    }
    pub fn committed_membership_index(&self) -> u64 {
        self.committed_membership_index
    }
}

pub struct PendingPolisSession {
    binding: PolisSessionBinding,
}

impl PendingPolisSession {
    fn new(binding: PolisSessionBinding) -> Self {
        Self { binding }
    }
}

#[derive(Clone)]
pub struct EstablishedPolisSession {
    binding: PolisSessionBinding,
}

impl EstablishedPolisSession {
    pub fn binding(&self) -> &PolisSessionBinding {
        &self.binding
    }
}

#[derive(Clone, Debug)]
struct VerifiedRouteAuthority {
    node_id: String,
    guardian_id: String,
    control_public_key: [u8; 32],
    boot_generation: u64,
}

#[derive(Clone, Debug)]
pub struct PolisIdentityBinding {
    polis_id: String,
    trust_domain: String,
    committed_membership_index: u64,
    boot_generations: BTreeMap<u64, u64>,
}

impl PolisIdentityBinding {
    pub fn verify(
        polis_id: &str,
        trust_domain: &str,
        committed_membership_index: u64,
        boot_generations: &BTreeMap<u64, u64>,
        endorsements: &BTreeMap<Vec<u8>, Vec<u8>>,
        authority: &AuthorityMembership,
    ) -> TransportResult<Self> {
        let configured = authority
            .raft_membership
            .get_joint_config()
            .iter()
            .flatten()
            .copied()
            .collect::<BTreeSet<_>>();
        if boot_generations.keys().copied().collect::<BTreeSet<_>>() != configured
            || boot_generations.values().any(|generation| *generation == 0)
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        let payload = polis_identity_signing_payload(
            polis_id,
            trust_domain,
            committed_membership_index,
            boot_generations,
        )?;
        if authority.trust_domain_id.as_slice() != trust_domain.as_bytes()
            || authority.committed_log_index != committed_membership_index
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        let mut verified = BTreeSet::new();
        for (guardian, signature) in endorsements {
            let voter = authority
                .voters
                .get(guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            if voter.revoked || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement {
                return Err(TransportError::InvalidSessionBinding);
            }
            let signature = Signature::from_slice(signature)
                .map_err(|_| TransportError::InvalidSessionBinding)?;
            VerifyingKey::from_bytes(&voter.control_public_key)
                .map_err(|_| TransportError::InvalidSessionBinding)?
                .verify(&payload, &signature)
                .map_err(|_| TransportError::InvalidSessionBinding)?;
            verified.insert(
                *authority
                    .raft_ids
                    .get(guardian)
                    .ok_or(TransportError::InvalidSessionBinding)?,
            );
        }
        if authority
            .raft_membership
            .get_joint_config()
            .iter()
            .any(|config| {
                config.iter().filter(|node| verified.contains(node)).count() <= config.len() / 2
            })
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(Self {
            polis_id: polis_id.to_owned(),
            trust_domain: trust_domain.to_owned(),
            committed_membership_index,
            boot_generations: boot_generations.clone(),
        })
    }
}

pub fn polis_identity_signing_payload(
    polis_id: &str,
    trust_domain: &str,
    committed_membership_index: u64,
    boot_generations: &BTreeMap<u64, u64>,
) -> TransportResult<Vec<u8>> {
    validate_text(polis_id)?;
    validate_text(trust_domain)?;
    if committed_membership_index == 0 {
        return Err(TransportError::InvalidSessionBinding);
    }
    if boot_generations.is_empty() || boot_generations.values().any(|generation| *generation == 0) {
        return Err(TransportError::InvalidSessionBinding);
    }
    let boots = boot_generations
        .iter()
        .map(|(node, generation)| format!("{node}:{generation}"))
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!(
        "adl.distributed.polis_identity.v1\0{polis_id}\0{trust_domain}\0{committed_membership_index}\0{boots}"
    )
    .into_bytes())
}

#[derive(Clone, Debug)]
pub struct VerifiedPolisRouteCut {
    polis_id: String,
    trust_domain: String,
    committed_membership_index: u64,
    routes: BTreeMap<u64, SocketAddr>,
    authorities: BTreeMap<u64, VerifiedRouteAuthority>,
}

impl VerifiedPolisRouteCut {
    pub fn verify(
        polis: &PolisIdentityBinding,
        membership: &MembershipState,
        authority: &AuthorityMembership,
        addresses: &BTreeMap<String, SocketAddr>,
        now_unix_seconds: i64,
    ) -> TransportResult<Self> {
        if polis.trust_domain != membership.trust_domain()
            || polis.committed_membership_index != membership.committed_log_index()
            || authority.trust_domain_id.as_slice() != membership.trust_domain().as_bytes()
            || authority.committed_log_index != membership.committed_log_index()
            || now_unix_seconds <= 0
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        let members = membership
            .members()
            .filter(|member| member.role == MemberRole::Voter)
            .collect::<Vec<_>>();
        let configured = authority
            .raft_membership
            .get_joint_config()
            .iter()
            .flatten()
            .copied()
            .collect::<BTreeSet<_>>();
        if members.len() != 3
            || authority.voters.len() != 3
            || addresses.len() != 3
            || polis.boot_generations.len() != 3
            || configured.len() != 3
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        let mut routes = BTreeMap::new();
        let mut authorities = BTreeMap::new();
        for member in members {
            let guardian = member.guardian_id.as_bytes().to_vec();
            let voter = authority
                .voters
                .get(&guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let raft_id = *authority
                .raft_ids
                .get(&guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let address = *addresses
                .get(&member.node_id)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let boot_generation = *polis
                .boot_generations
                .get(&raft_id)
                .ok_or(TransportError::InvalidSessionBinding)?;
            if !configured.contains(&raft_id)
                || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
                || voter.revoked
                || voter.certificate_generation != member.identity_generation
                || voter.control_public_key != member.guardian_control_public_key
                || voter.not_before_unix_seconds > now_unix_seconds
                || voter.not_after_unix_seconds <= now_unix_seconds
                || boot_generation == 0
            {
                return Err(TransportError::InvalidSessionBinding);
            }
            routes.insert(raft_id, address);
            authorities.insert(
                raft_id,
                VerifiedRouteAuthority {
                    node_id: member.node_id.clone(),
                    guardian_id: member.guardian_id.clone(),
                    control_public_key: voter.control_public_key,
                    boot_generation,
                },
            );
        }
        if routes.len() != 3 || authorities.len() != 3 {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(Self {
            polis_id: polis.polis_id.clone(),
            trust_domain: membership.trust_domain().to_owned(),
            committed_membership_index: membership.committed_log_index(),
            routes,
            authorities,
        })
    }

    pub fn routes(&self) -> BTreeMap<u64, SocketAddr> {
        self.routes.clone()
    }
    pub fn contains(&self, node: u64) -> bool {
        self.authorities.contains_key(&node)
    }
    pub fn len(&self) -> usize {
        self.authorities.len()
    }
    pub fn is_empty(&self) -> bool {
        self.authorities.is_empty()
    }
    pub fn committed_membership_index(&self) -> u64 {
        self.committed_membership_index
    }

    pub fn pending_session(
        &self,
        local: u64,
        peer: u64,
        connection: &AuthenticatedConnection,
    ) -> TransportResult<PendingPolisSession> {
        if local == peer || !connection.has_authority_connection_role(local, peer) {
            return Err(TransportError::InvalidSessionBinding);
        }
        let local_authority = self
            .authorities
            .get(&local)
            .ok_or(TransportError::InvalidSessionBinding)?;
        let peer_authority = self
            .authorities
            .get(&peer)
            .ok_or(TransportError::InvalidSessionBinding)?;
        let (local_tls, peer_tls) = connection.local_peer_route();
        if local_tls.trust_domain != self.trust_domain
            || peer_tls.trust_domain != self.trust_domain
            || local_tls.node_id != local_authority.node_id
            || peer_tls.node_id != peer_authority.node_id
            || local_tls.guardian_id != local_authority.guardian_id
            || peer_tls.guardian_id != peer_authority.guardian_id
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(PendingPolisSession::new(
            PolisSessionBinding::from_authority(
                self.polis_id.clone(),
                self.trust_domain.clone(),
                local_authority.node_id.clone(),
                peer_authority.node_id.clone(),
                local_authority.guardian_id.clone(),
                peer_authority.guardian_id.clone(),
                local_tls.certificate_generation,
                peer_tls.certificate_generation,
                local_authority.boot_generation,
                peer_authority.boot_generation,
                self.committed_membership_index,
                local_authority.control_public_key,
                peer_authority.control_public_key,
            )?,
        ))
    }

    pub fn session_matches(
        &self,
        local: u64,
        peer: u64,
        connection: &AuthenticatedConnection,
        established: &EstablishedPolisSession,
    ) -> bool {
        self.pending_session(local, peer, connection)
            .is_ok_and(|pending| pending.binding == established.binding)
    }

    pub fn same_polis_and_domain(&self, other: &Self) -> bool {
        self.polis_id == other.polis_id && self.trust_domain == other.trust_domain
    }

    pub fn boot_generation(&self, node: u64) -> Option<u64> {
        self.authorities
            .get(&node)
            .map(|value| value.boot_generation)
    }
}

pub struct IncomingPolisRequest {
    pub sequence: u64,
    pub message_kind: String,
    pub request_sha256: [u8; 32],
    pub payload: Vec<u8>,
    send: quinn::SendStream,
    connection: Connection,
    cancellation: CancellationToken,
    authorization_deadline: Instant,
}

pub struct PendingPolisResponse {
    receive: quinn::RecvStream,
    connection: Connection,
    limits: TransportLimits,
    cancellation: CancellationToken,
    authorization_deadline: Instant,
    sequence: u64,
    request_sha256: [u8; 32],
}

impl PendingPolisResponse {
    pub async fn receive(mut self) -> TransportResult<Vec<u8>> {
        let limit = frame_read_limit(&self.limits)?;
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        let response_bytes = tokio::select! {
            _ = self.cancellation.cancelled() => {
                self.connection.close(CLOSE_CODE.into(), b"cancelled");
                return Err(TransportError::Cancelled);
            }
            _ = tokio::time::sleep_until(self.authorization_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"authorization expired");
                return Err(TransportError::AuthorizationExpired);
            }
            _ = tokio::time::sleep_until(idle_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"idle timeout");
                return Err(TransportError::IdleTimeout);
            }
            result = self.receive.read_to_end(limit) => result.map_err(|error| match error {
                quinn::ReadToEndError::TooLong => TransportError::FrameTooLarge,
                quinn::ReadToEndError::Read(_) => TransportError::Stream,
            })?,
        };
        let response: PolisTransportResponse = decode_prost_frame(&response_bytes, &self.limits)?;
        let response_digest: [u8; 32] = Sha256::digest(&response.payload).into();
        if response.schema != POLIS_TRANSPORT_RESPONSE_SCHEMA
            || response.sequence != self.sequence
            || response.request_sha256.as_slice() != self.request_sha256
            || response.payload_sha256.as_slice() != response_digest
        {
            return Err(TransportError::ResponseMismatch);
        }
        Ok(response.payload)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerBinding {
    pub leaf_certificate_sha256: [u8; 32],
    tls_subject_public_key: [u8; 32],
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
            tls_subject_public_key: ed25519_subject_public_key(leaf_certificate)?,
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
    trust_domain: String,
    generation: u64,
    certificate_id: String,
    subject_public_key: [u8; 32],
}

impl TransportAuthorization {
    pub fn new(
        store: Arc<DistributedCertificateStore>,
        certificate: &AuthorityCertificate,
    ) -> TransportResult<Self> {
        let body = &certificate.body;
        let verified = store
            .authorize(
                &body.holder_id,
                CertificatePurpose::Transport,
                body.generation,
                unix_time()?,
            )
            .map_err(|_| TransportError::CertificateAuthorization)?;
        let certificate_id = certificate
            .certificate_id()
            .map_err(|_| TransportError::CertificateAuthorization)?;
        if verified.certificate_id != certificate_id
            || body.purpose != CertificatePurpose::Transport
        {
            return Err(TransportError::CertificateAuthorization);
        }
        Ok(Self {
            store,
            holder_id: body.holder_id.clone(),
            trust_domain: body.trust_domain.clone(),
            generation: body.generation,
            certificate_id,
            subject_public_key: body.subject_public_key,
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
        if self.holder_id != binding.node_id
            || self.trust_domain != binding.trust_domain
            || self.generation != binding.certificate_generation
            || self.subject_public_key != binding.tls_subject_public_key
        {
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
        local_authorization.validate_binding(&local)?;
        peer_authorization.validate_binding(&expected_peer)?;
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
    IdleTimeout,
    Stream,
    InvalidSessionBinding,
    PayloadDigestMismatch,
    ResponseMismatch,
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
            Self::IdleTimeout => "idle_timeout",
            Self::Stream => "stream_error",
            Self::InvalidSessionBinding => "invalid_session_binding",
            Self::PayloadDigestMismatch => "payload_digest_mismatch",
            Self::ResponseMismatch => "response_mismatch",
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
    let identity = TlsIdentity::from_der(certificate_chain, private_key)
        .map_err(|_| TransportError::InvalidTlsMaterial)?;
    let roots =
        trust_roots_from_der(client_roots).map_err(|_| TransportError::InvalidTlsMaterial)?;
    let tls = build_mutual_tls_server_config(identity, roots, &[TRANSPORT_ALPN])
        .map_err(|_| TransportError::TlsConfiguration)?;
    let crypto =
        QuicServerConfig::try_from((*tls).clone()).map_err(|_| TransportError::TlsConfiguration)?;
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
    let identity = TlsIdentity::from_der(certificate_chain, private_key)
        .map_err(|_| TransportError::InvalidTlsMaterial)?;
    let roots =
        trust_roots_from_der(server_roots).map_err(|_| TransportError::InvalidTlsMaterial)?;
    let tls = build_mutual_tls_client_config(identity, roots, &[TRANSPORT_ALPN])
        .map_err(|_| TransportError::TlsConfiguration)?;
    let crypto =
        QuicClientConfig::try_from((*tls).clone()).map_err(|_| TransportError::TlsConfiguration)?;
    let mut config = quinn::ClientConfig::new(Arc::new(crypto));
    config.transport_config(transport_config(limits)?);
    let mut endpoint = Endpoint::client(bind).map_err(|_| TransportError::Endpoint)?;
    endpoint.set_default_client_config(config);
    Ok(endpoint)
}

fn ed25519_subject_public_key(certificate: &CertificateDer<'_>) -> TransportResult<[u8; 32]> {
    let (remaining, parsed) = x509_parser::parse_x509_certificate(certificate.as_ref())
        .map_err(|_| TransportError::InvalidTlsMaterial)?;
    if !remaining.is_empty()
        || parsed
            .tbs_certificate
            .subject_pki
            .algorithm
            .algorithm
            .to_id_string()
            != "1.3.101.112"
    {
        return Err(TransportError::InvalidTlsMaterial);
    }
    parsed
        .tbs_certificate
        .subject_pki
        .subject_public_key
        .data
        .as_ref()
        .try_into()
        .map_err(|_| TransportError::InvalidTlsMaterial)
}

fn transport_config(limits: &TransportLimits) -> TransportResult<Arc<quinn::TransportConfig>> {
    let mut config = quinn::TransportConfig::default();
    config.max_concurrent_uni_streams(limits.max_concurrent_uni_streams.into());
    config.max_concurrent_bidi_streams(limits.max_concurrent_uni_streams.into());
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
    role: ConnectionRole,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConnectionRole {
    Dialer,
    Acceptor,
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
        Self::from_connection(connection, security, ConnectionRole::Dialer)
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
        Self::from_connection(connection, security, ConnectionRole::Acceptor)
    }

    fn from_connection(
        connection: Connection,
        security: ConnectionSecurity,
        role: ConnectionRole,
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
            role,
        })
    }

    pub async fn send(&self, sequence: u64, payload: Vec<u8>) -> TransportResult<()> {
        self.require_authority()?;
        if sequence == 0 {
            return Err(TransportError::SequenceInvalid);
        }
        let frame = encode_frame(self.local.envelope(sequence, payload), &self.limits)?;
        let mut stream = self.open_uni_authorized().await?;
        self.write_authorized(&mut stream, &frame).await?;
        stream.finish().map_err(|_| TransportError::Stream)
    }

    pub async fn receive(&self) -> TransportResult<TransportEnvelope> {
        self.require_authority()?;
        let mut stream = self.accept_uni_authorized().await?;
        let bytes = self.read_authorized(&mut stream).await?;
        let envelope = decode_frame(&bytes, &self.limits)?;
        verify_envelope(&envelope, &self.expected_peer)?;
        self.replay_window
            .lock()
            .map_err(|_| TransportError::CertificateAuthorization)?
            .observe(envelope.sequence)?;
        Ok(envelope)
    }

    fn local_peer_route(&self) -> (&PeerBinding, &PeerBinding) {
        (&self.local, &self.expected_peer)
    }

    fn has_authority_connection_role(&self, local: u64, peer: u64) -> bool {
        matches!(
            (local < peer, self.role),
            (true, ConnectionRole::Acceptor) | (false, ConnectionRole::Dialer)
        )
    }

    pub async fn initiate_polis_session(
        &self,
        pending: PendingPolisSession,
        signing_key: &SigningKey,
    ) -> TransportResult<EstablishedPolisSession> {
        self.require_polis_binding(&pending.binding)?;
        validate_local_control_key(&pending.binding, signing_key)?;
        let request = signed_handshake(&pending.binding, signing_key)?;
        let request_bytes = encode_prost_frame(&request, &self.limits)?;
        let (mut send, mut receive) = self.open_bi_authorized().await?;
        self.write_authorized(&mut send, &request_bytes).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        let response = self.read_authorized(&mut receive).await?;
        let response: PolisHandshake = decode_prost_frame(&response, &self.limits)?;
        verify_handshake(&response, &pending.binding.reverse())?;
        Ok(EstablishedPolisSession {
            binding: pending.binding,
        })
    }

    pub async fn accept_polis_session(
        &self,
        pending: PendingPolisSession,
        signing_key: &SigningKey,
    ) -> TransportResult<EstablishedPolisSession> {
        self.require_polis_binding(&pending.binding)?;
        validate_local_control_key(&pending.binding, signing_key)?;
        let (mut send, mut receive) = self.accept_bi_authorized().await?;
        let request = self.read_authorized(&mut receive).await?;
        let request: PolisHandshake = decode_prost_frame(&request, &self.limits)?;
        verify_handshake(&request, &pending.binding.reverse())?;
        let response = signed_handshake(&pending.binding, signing_key)?;
        let response = encode_prost_frame(&response, &self.limits)?;
        self.write_authorized(&mut send, &response).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        Ok(EstablishedPolisSession {
            binding: pending.binding,
        })
    }

    pub async fn request_polis(
        &self,
        session: &EstablishedPolisSession,
        sequence: u64,
        message_kind: &str,
        payload: Vec<u8>,
    ) -> TransportResult<Vec<u8>> {
        self.begin_polis_request(session, sequence, message_kind, payload)
            .await?
            .receive()
            .await
    }

    pub async fn begin_polis_request(
        &self,
        session: &EstablishedPolisSession,
        sequence: u64,
        message_kind: &str,
        payload: Vec<u8>,
    ) -> TransportResult<PendingPolisResponse> {
        let binding = session.binding();
        self.require_polis_binding(binding)?;
        validate_text(message_kind)?;
        if sequence == 0 || payload.len() > self.limits.max_frame_bytes {
            return Err(TransportError::FrameTooLarge);
        }
        let payload_sha256: [u8; 32] = Sha256::digest(&payload).into();
        let envelope = PolisTransportEnvelope {
            schema: POLIS_TRANSPORT_SCHEMA.to_owned(),
            polis_id: binding.polis_id.clone(),
            trust_domain: binding.trust_domain.clone(),
            sender_node_id: binding.local_node_id.clone(),
            receiver_node_id: binding.peer_node_id.clone(),
            certificate_generation: binding.local_certificate_generation,
            boot_generation: binding.local_boot_generation,
            committed_membership_index: binding.committed_membership_index,
            sequence,
            message_kind: message_kind.to_owned(),
            payload_sha256: payload_sha256.to_vec(),
            payload,
        };
        let request_bytes = encode_prost_frame(&envelope, &self.limits)?;
        let request_sha256: [u8; 32] = Sha256::digest(&request_bytes).into();
        let (mut send, receive) = self.open_bi_authorized().await?;
        self.write_authorized(&mut send, &request_bytes).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        Ok(PendingPolisResponse {
            receive,
            connection: self.connection.clone(),
            limits: self.limits.clone(),
            cancellation: self.cancellation.clone(),
            authorization_deadline: self.authorization_deadline,
            sequence,
            request_sha256,
        })
    }

    pub async fn accept_polis_request(
        &self,
        session: &EstablishedPolisSession,
    ) -> TransportResult<IncomingPolisRequest> {
        let binding = session.binding();
        self.require_polis_binding(binding)?;
        let (send, mut receive) = self.accept_bi_authorized().await?;
        let bytes = self.read_authorized(&mut receive).await?;
        let envelope: PolisTransportEnvelope = decode_prost_frame(&bytes, &self.limits)?;
        let payload_digest: [u8; 32] = Sha256::digest(&envelope.payload).into();
        if envelope.schema != POLIS_TRANSPORT_SCHEMA
            || envelope.polis_id != binding.polis_id
            || envelope.trust_domain != binding.trust_domain
            || envelope.sender_node_id != binding.peer_node_id
            || envelope.receiver_node_id != binding.local_node_id
            || envelope.certificate_generation != binding.peer_certificate_generation
            || envelope.boot_generation != binding.peer_boot_generation
            || envelope.committed_membership_index != binding.committed_membership_index
            || envelope.sequence == 0
            || envelope.payload_sha256.as_slice() != payload_digest
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        validate_text(&envelope.message_kind)?;
        Ok(IncomingPolisRequest {
            sequence: envelope.sequence,
            message_kind: envelope.message_kind,
            request_sha256: Sha256::digest(&bytes).into(),
            payload: envelope.payload,
            send,
            connection: self.connection.clone(),
            cancellation: self.cancellation.clone(),
            authorization_deadline: self.authorization_deadline,
        })
    }

    fn require_polis_binding(&self, binding: &PolisSessionBinding) -> TransportResult<()> {
        self.require_authority()?;
        binding.validate()?;
        if binding.trust_domain != self.local.trust_domain
            || binding.local_node_id != self.local.node_id
            || binding.peer_node_id != self.expected_peer.node_id
            || binding.local_guardian_id != self.local.guardian_id
            || binding.peer_guardian_id != self.expected_peer.guardian_id
            || binding.local_certificate_generation != self.local.certificate_generation
            || binding.peer_certificate_generation != self.expected_peer.certificate_generation
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(())
    }

    async fn open_bi_authorized(&self) -> TransportResult<(quinn::SendStream, quinn::RecvStream)> {
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = self.connection.open_bi() => result.map_err(|_| TransportError::Stream),
        }
    }

    async fn accept_bi_authorized(
        &self,
    ) -> TransportResult<(quinn::SendStream, quinn::RecvStream)> {
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = self.connection.accept_bi() => result.map_err(|_| TransportError::Stream),
        }
    }

    async fn open_uni_authorized(&self) -> TransportResult<quinn::SendStream> {
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = self.connection.open_uni() => result.map_err(|_| TransportError::Stream),
        }
    }

    async fn accept_uni_authorized(&self) -> TransportResult<quinn::RecvStream> {
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = self.connection.accept_uni() => result.map_err(|_| TransportError::Stream),
        }
    }

    async fn write_authorized(
        &self,
        send: &mut quinn::SendStream,
        bytes: &[u8],
    ) -> TransportResult<()> {
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = send.write_all(bytes) => result.map_err(|_| TransportError::Stream),
        }
    }

    async fn read_authorized(&self, receive: &mut quinn::RecvStream) -> TransportResult<Vec<u8>> {
        let limit = frame_read_limit(&self.limits)?;
        let idle_deadline = Instant::now() + self.limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => self.cancel(),
            _ = tokio::time::sleep_until(self.authorization_deadline) => self.expire(),
            _ = tokio::time::sleep_until(idle_deadline) => self.timeout(),
            result = receive.read_to_end(limit) => result.map_err(|error| match error {
                quinn::ReadToEndError::TooLong => TransportError::FrameTooLarge,
                quinn::ReadToEndError::Read(_) => TransportError::Stream,
            }),
        }
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

    fn timeout<T>(&self) -> TransportResult<T> {
        self.connection.close(CLOSE_CODE.into(), b"idle timeout");
        Err(TransportError::IdleTimeout)
    }
}

impl IncomingPolisRequest {
    pub async fn respond(
        mut self,
        payload: Vec<u8>,
        limits: &TransportLimits,
    ) -> TransportResult<()> {
        if payload.len() > limits.max_frame_bytes {
            return Err(TransportError::FrameTooLarge);
        }
        let response = PolisTransportResponse {
            schema: POLIS_TRANSPORT_RESPONSE_SCHEMA.to_owned(),
            sequence: self.sequence,
            request_sha256: self.request_sha256.to_vec(),
            payload_sha256: Sha256::digest(&payload).to_vec(),
            payload,
        };
        let bytes = encode_prost_frame(&response, limits)?;
        let write_idle_deadline = Instant::now() + limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => {
                self.connection.close(CLOSE_CODE.into(), b"cancelled");
                return Err(TransportError::Cancelled);
            }
            _ = tokio::time::sleep_until(self.authorization_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"authorization expired");
                return Err(TransportError::AuthorizationExpired);
            }
            _ = tokio::time::sleep_until(write_idle_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"idle timeout");
                return Err(TransportError::IdleTimeout);
            }
            result = self.send.write_all(&bytes) => result.map_err(|_| TransportError::Stream)?,
        }
        self.send.finish().map_err(|_| TransportError::Stream)?;
        let idle_deadline = Instant::now() + limits.idle_timeout;
        tokio::select! {
            _ = self.cancellation.cancelled() => {
                self.connection.close(CLOSE_CODE.into(), b"cancelled");
                Err(TransportError::Cancelled)
            },
            _ = tokio::time::sleep_until(self.authorization_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"authorization expired");
                Err(TransportError::AuthorizationExpired)
            },
            _ = tokio::time::sleep_until(idle_deadline) => {
                self.connection.close(CLOSE_CODE.into(), b"idle timeout");
                Err(TransportError::IdleTimeout)
            },
            result = self.send.stopped() => match result {
                Ok(None) => Ok(()),
                _ => Err(TransportError::Stream),
            },
        }
    }
}

fn frame_read_limit(limits: &TransportLimits) -> TransportResult<usize> {
    limits
        .max_frame_bytes
        .checked_add(LENGTH_PREFIX_SLACK)
        .ok_or(TransportError::InvalidLimits)
}

fn validate_local_control_key(
    binding: &PolisSessionBinding,
    signing_key: &SigningKey,
) -> TransportResult<()> {
    if signing_key.verifying_key().to_bytes() != binding.local_control_public_key {
        return Err(TransportError::InvalidSessionBinding);
    }
    Ok(())
}

fn signed_handshake(
    binding: &PolisSessionBinding,
    signing_key: &SigningKey,
) -> TransportResult<PolisHandshake> {
    let mut handshake = PolisHandshake {
        schema: POLIS_HANDSHAKE_SCHEMA.to_owned(),
        polis_id: binding.polis_id.clone(),
        trust_domain: binding.trust_domain.clone(),
        sender_node_id: binding.local_node_id.clone(),
        receiver_node_id: binding.peer_node_id.clone(),
        sender_certificate_generation: binding.local_certificate_generation,
        receiver_certificate_generation: binding.peer_certificate_generation,
        sender_boot_generation: binding.local_boot_generation,
        receiver_boot_generation: binding.peer_boot_generation,
        committed_membership_index: binding.committed_membership_index,
        signature: Vec::new(),
    };
    handshake.signature = signing_key
        .sign(&handshake_signing_bytes(&handshake)?)
        .to_bytes()
        .to_vec();
    Ok(handshake)
}

fn verify_handshake(
    handshake: &PolisHandshake,
    expected: &PolisSessionBinding,
) -> TransportResult<()> {
    if handshake.schema != POLIS_HANDSHAKE_SCHEMA
        || handshake.polis_id != expected.polis_id
        || handshake.trust_domain != expected.trust_domain
        || handshake.sender_node_id != expected.local_node_id
        || handshake.receiver_node_id != expected.peer_node_id
        || handshake.sender_certificate_generation != expected.local_certificate_generation
        || handshake.receiver_certificate_generation != expected.peer_certificate_generation
        || handshake.sender_boot_generation != expected.local_boot_generation
        || handshake.receiver_boot_generation != expected.peer_boot_generation
        || handshake.committed_membership_index != expected.committed_membership_index
    {
        return Err(TransportError::InvalidSessionBinding);
    }
    let signature = Signature::from_slice(&handshake.signature)
        .map_err(|_| TransportError::InvalidSessionBinding)?;
    VerifyingKey::from_bytes(&expected.local_control_public_key)
        .map_err(|_| TransportError::InvalidSessionBinding)?
        .verify(&handshake_signing_bytes(handshake)?, &signature)
        .map_err(|_| TransportError::InvalidSessionBinding)
}

fn handshake_signing_bytes(handshake: &PolisHandshake) -> TransportResult<Vec<u8>> {
    let mut unsigned = handshake.clone();
    unsigned.signature.clear();
    let mut bytes = Vec::with_capacity(unsigned.encoded_len());
    unsigned
        .encode(&mut bytes)
        .map_err(|_| TransportError::MalformedFrame)?;
    Ok(bytes)
}

fn encode_prost_frame<T: Message>(
    message: &T,
    limits: &TransportLimits,
) -> TransportResult<Vec<u8>> {
    if message.encoded_len() > limits.max_frame_bytes {
        return Err(TransportError::FrameTooLarge);
    }
    let mut bytes = Vec::with_capacity(message.encoded_len() + LENGTH_PREFIX_SLACK);
    message
        .encode_length_delimited(&mut bytes)
        .map_err(|_| TransportError::MalformedFrame)?;
    if bytes.len() > frame_read_limit(limits)? {
        return Err(TransportError::FrameTooLarge);
    }
    Ok(bytes)
}

fn decode_prost_frame<T: Message + Default>(
    bytes: &[u8],
    limits: &TransportLimits,
) -> TransportResult<T> {
    if bytes.len() > frame_read_limit(limits)? {
        return Err(TransportError::FrameTooLarge);
    }
    let message = T::decode_length_delimited(bytes).map_err(|_| TransportError::MalformedFrame)?;
    let canonical = encode_prost_frame(&message, limits)?;
    if canonical != bytes {
        return Err(TransportError::MalformedFrame);
    }
    Ok(message)
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
    if encode_frame(envelope.clone(), limits)? != bytes {
        return Err(TransportError::MalformedFrame);
    }
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
