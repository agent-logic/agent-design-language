use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    io::Cursor,
    net::SocketAddr,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use prost::Message;
use quinn::{
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
    Connection, Endpoint,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

#[cfg(test)]
use super::lease::VoterAuthority;
use super::{
    authority_store_adapters::{AuthorityBoundCertificateStore, AuthorityStoreAdapterError},
    certificates::{AuthorityCertificate, CertificatePurpose},
    lease::{AuthorityMembership, ControlCertificatePurpose},
    membership::{MemberRole, MembershipPolicy, MembershipState},
};
#[cfg(test)]
use super::certificates::DistributedCertificateStore;
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

pub(crate) fn transport_peer_identity_key(
    role: LearnerEndpointRole,
    stable_raft_id: u64,
    node_id: &str,
    guardian_id: &str,
) -> TransportResult<String> {
    validate_text(node_id)?;
    validate_text(guardian_id)?;
    if stable_raft_id == 0 {
        return Err(TransportError::InvalidSessionBinding);
    }
    #[derive(Serialize)]
    struct CanonicalPeerIdentity<'a> {
        schema: &'static str,
        role: &'static str,
        stable_raft_id: u64,
        node_id: &'a str,
        guardian_id: &'a str,
    }
    let role = match role {
        LearnerEndpointRole::Voter => "voter",
        LearnerEndpointRole::Learner => "learner",
    };
    let bytes = serde_jcs::to_vec(&CanonicalPeerIdentity {
        schema: "adl.distributed.transport_peer_identity.v1",
        role,
        stable_raft_id,
        node_id,
        guardian_id,
    })
    .map_err(|_| TransportError::InvalidSessionBinding)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearnerEndpointRole {
    Voter,
    Learner,
}

#[derive(Clone, Debug, Default)]
struct TransportAuthorityView {
    excluded_identity: Option<(String, String)>,
    exclusion_generation: u64,
    voter_cut_sha256: Option<[u8; 32]>,
    learner_operation_sha256: Option<[u8; 32]>,
    peer_instances: BTreeMap<String, [u8; 32]>,
}

#[derive(Clone)]
struct ProductionTransportAuthority {
    instance_id: [u8; 32],
    fence: Arc<tokio::sync::RwLock<()>>,
    view: Arc<Mutex<TransportAuthorityView>>,
    #[cfg(test)]
    dispatch_test_hook: Arc<Mutex<Option<TransportDispatchTestHook>>>,
}

#[cfg(test)]
#[derive(Clone)]
pub(crate) struct TransportDispatchTestHook {
    pub(crate) phase: &'static str,
    pub(crate) reached: Arc<tokio::sync::Notify>,
    pub(crate) release: Arc<tokio::sync::Notify>,
}

/// Sole mutation capability for one production transport-authority instance.
/// It is deliberately non-clone; the Runtime factory owns it for its lifetime.
struct TransportAuthorityOwner {
    authority: ProductionTransportAuthority,
}

impl TransportAuthorityOwner {
    fn bootstrap(
        instance_id: [u8; 32],
        peer_instances: BTreeMap<String, [u8; 32]>,
    ) -> Self {
        Self {
            authority: ProductionTransportAuthority {
                instance_id,
                fence: Arc::new(tokio::sync::RwLock::new(())),
                view: Arc::new(Mutex::new(TransportAuthorityView {
                    peer_instances,
                    ..TransportAuthorityView::default()
                })),
                #[cfg(test)]
                dispatch_test_hook: Arc::new(Mutex::new(None)),
            },
        }
    }

    fn authority(&self) -> ProductionTransportAuthority {
        self.authority.clone()
    }

    async fn write_lease(&self) -> TransportAuthorityWriteLease {
        TransportAuthorityWriteLease {
            authority: self.authority.clone(),
            _guard: Arc::clone(&self.authority.fence).write_owned().await,
        }
    }

    fn bind_current_view(
        &self,
        voter_cut_sha256: [u8; 32],
        learner_operation_sha256: Option<[u8; 32]>,
        exclusion: Option<(String, String)>,
        exclusion_generation: u64,
    ) -> TransportResult<()> {
        let mut view = self
            .authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        if view.voter_cut_sha256.is_some() {
            #[cfg(test)]
            {
                view.voter_cut_sha256 = Some(voter_cut_sha256);
                view.learner_operation_sha256 = learner_operation_sha256;
                view.excluded_identity = exclusion;
                view.exclusion_generation = exclusion_generation;
                return Ok(());
            }
            #[cfg(not(test))]
            return Err(TransportError::InvalidSessionBinding);
        }
        view.voter_cut_sha256 = Some(voter_cut_sha256);
        view.learner_operation_sha256 = learner_operation_sha256;
        view.excluded_identity = exclusion;
        view.exclusion_generation = exclusion_generation;
        Ok(())
    }
}

struct TransportAuthorityWriteLease {
    authority: ProductionTransportAuthority,
    _guard: tokio::sync::OwnedRwLockWriteGuard<()>,
}

impl TransportAuthorityWriteLease {
    fn require_authority(
        &self,
        authority: &ProductionTransportAuthority,
    ) -> TransportResult<()> {
        if Arc::ptr_eq(&self.authority.fence, &authority.fence)
            && self.authority.instance_id == authority.instance_id
        {
            Ok(())
        } else {
            Err(TransportError::InvalidSessionBinding)
        }
    }

    fn commit_exclusion(
        &mut self,
        node_id: &str,
        guardian_id: &str,
        generation: u64,
    ) -> TransportResult<()> {
        let mut view = self
            .authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        view.excluded_identity = Some((node_id.to_owned(), guardian_id.to_owned()));
        view.exclusion_generation = generation;
        Ok(())
    }


    fn replace_learner_operation(
        &mut self,
        operation_sha256: Option<[u8; 32]>,
    ) -> TransportResult<()> {
        self.authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?
            .learner_operation_sha256 = operation_sha256;
        Ok(())
    }

    fn replace_voter_cut(
        &mut self,
        voter_cut_sha256: [u8; 32],
    ) -> TransportResult<()> {
        self.authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?
            .voter_cut_sha256 = Some(voter_cut_sha256);
        Ok(())
    }

    fn commit_peer_instance(
        &mut self,
        guardian_id: &str,
        instance_id: [u8; 32],
    ) -> TransportResult<()> {
        let mut view = self
            .authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        match view.peer_instances.get(guardian_id) {
            Some(current) if current == &instance_id => Ok(()),
            Some(_) => Err(TransportError::InvalidSessionBinding),
            None => {
                view.peer_instances.insert(guardian_id.to_owned(), instance_id);
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
struct LearnerWireSession {
    authority: ProductionTransportAuthority,
    voter_cut_sha256: [u8; 32],
    operation_sha256: [u8; 32],
    peer_identity_key: String,
    expected_peer_instance_id: Option<[u8; 32]>,
}

struct LearnerSendPermit {
    instance_id: [u8; 32],
    sequence: u64,
    payload: Vec<u8>,
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

struct LearnerReceivePermit {
    instance_id: [u8; 32],
    guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

struct LearnerPendingResponse {
    instance_id: [u8; 32],
    guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

struct LearnerHandshakePermit {
    instance_id: [u8; 32],
    request: Vec<u8>,
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

struct LearnerAcceptPermit {
    instance_id: [u8; 32],
    guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

struct LearnerReceivedEnvelope {
    instance_id: [u8; 32],
    envelope: TransportEnvelope,
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

impl LearnerReceivedEnvelope {
    fn sequence(&self) -> u64 {
        self.envelope.sequence
    }

    fn payload(&self) -> &[u8] {
        &self.envelope.payload
    }

    fn response_permit(
        self,
        sequence: u64,
        payload: Vec<u8>,
    ) -> LearnerSendPermit {
        LearnerSendPermit {
            instance_id: self.instance_id,
            sequence,
            payload,
            _guard: self._guard,
        }
    }
}

struct PendingLearnerHandshake {
    send: quinn::SendStream,
    connection: Connection,
    limits: TransportLimits,
    cancellation: CancellationToken,
    authorization_deadline: Instant,
    request: Vec<u8>,
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

#[derive(Clone)]
enum TransportCertificateAuthority {
    Bound(AuthorityBoundCertificateStore),
    #[cfg(test)]
    TestRaw(Arc<DistributedCertificateStore>),
}

impl TransportCertificateAuthority {
    fn authorize(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> TransportResult<super::certificates::VerifiedCertificate> {
        match self {
            Self::Bound(store) => store
                .authorize(holder_id, purpose, generation, now_unix_secs)
                .map_err(transport_certificate_authorization_error),
            #[cfg(test)]
            Self::TestRaw(store) => store
                .authorize(
                    &super::certificates::AUTHORITY_BOUND_CERTIFICATE_ACCESS,
                    holder_id,
                    purpose,
                    generation,
                    now_unix_secs,
                )
                .map_err(|_| TransportError::CertificateAuthorization),
        }
    }
}

fn transport_certificate_authorization_error(
    error: AuthorityStoreAdapterError,
) -> TransportError {
    match error {
        AuthorityStoreAdapterError::Certificate(_) => TransportError::CertificateAuthorization,
        _ => TransportError::InvalidSessionBinding,
    }
}

impl PendingLearnerHandshake {
    fn request(&self) -> &[u8] {
        &self.request
    }
}

impl fmt::Debug for LearnerWireSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LearnerWireSession")
            .field("operation_sha256", &hex::encode(self.operation_sha256))
            .finish_non_exhaustive()
    }
}

impl LearnerWireSession {
    async fn read_lease(
        &self,
    ) -> TransportResult<tokio::sync::OwnedRwLockReadGuard<()>> {
        let guard = self.authority.read_lease().await;
        let current = self
            .authority
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        if current.voter_cut_sha256 != Some(self.voter_cut_sha256)
            || current.learner_operation_sha256 != Some(self.operation_sha256)
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(guard)
    }

    fn instance_id(&self) -> [u8; 32] {
        self.authority.instance_id
    }

    fn expected_peer_instance_id(&self) -> Option<[u8; 32]> {
        self.expected_peer_instance_id
    }

    fn peer_identity_key(&self) -> &str {
        &self.peer_identity_key
    }

    async fn send_permit(
        &self,
        sequence: u64,
        payload: Vec<u8>,
    ) -> TransportResult<LearnerSendPermit> {
        Ok(LearnerSendPermit {
            instance_id: self.instance_id(),
            sequence,
            payload,
            _guard: self.read_lease().await?,
        })
    }

    async fn receive_permit(&self) -> TransportResult<LearnerReceivePermit> {
        Ok(LearnerReceivePermit {
            instance_id: self.instance_id(),
            guard: self.read_lease().await?,
        })
    }

    async fn initiate_handshake_permit(
        &self,
        request: Vec<u8>,
    ) -> TransportResult<LearnerHandshakePermit> {
        Ok(LearnerHandshakePermit {
            instance_id: self.instance_id(),
            request,
            _guard: self.read_lease().await?,
        })
    }

    async fn accept_handshake_permit(&self) -> TransportResult<LearnerAcceptPermit> {
        Ok(LearnerAcceptPermit {
            instance_id: self.instance_id(),
            guard: self.read_lease().await?,
        })
    }
}

impl ProductionTransportAuthority {
    #[cfg(test)]
    pub(crate) fn set_learner_operation_for_test(
        &self,
        voter_cut_sha256: [u8; 32],
        operation_sha256: Option<[u8; 32]>,
    ) {
        let mut view = self.view.lock().expect("transport view");
        view.voter_cut_sha256 = Some(voter_cut_sha256);
        view.learner_operation_sha256 = operation_sha256;
    }

    #[cfg(test)]
    pub(crate) fn set_exclusion_for_test(&self, node_id: &str, guardian_id: &str, generation: u64) {
        let mut view = self.view.lock().expect("transport view");
        view.excluded_identity = Some((node_id.to_owned(), guardian_id.to_owned()));
        view.exclusion_generation = generation;
    }

    pub(crate) async fn read_lease(&self) -> tokio::sync::OwnedRwLockReadGuard<()> {
        Arc::clone(&self.fence).read_owned().await
    }

    pub(crate) async fn dispatch_guard(&self) -> tokio::sync::OwnedRwLockReadGuard<()> {
        self.read_lease().await
    }

    #[cfg(test)]
    pub(crate) fn install_dispatch_pause_for_test(
        &self,
        phase: &'static str,
    ) -> TransportDispatchTestHook {
        let hook = TransportDispatchTestHook {
            phase,
            reached: Arc::new(tokio::sync::Notify::new()),
            release: Arc::new(tokio::sync::Notify::new()),
        };
        *self.dispatch_test_hook.lock().expect("dispatch test hook") = Some(hook.clone());
        hook
    }

    #[cfg(test)]
    pub(crate) async fn pause_after_revalidation_for_test(&self, phase: &'static str) {
        let hook = {
            let mut installed = self.dispatch_test_hook.lock().expect("dispatch test hook");
            if installed.as_ref().is_some_and(|hook| hook.phase == phase) {
                installed.take()
            } else {
                None
            }
        };
        if let Some(hook) = hook {
            hook.reached.notify_one();
            hook.release.notified().await;
        }
    }

    pub(crate) fn ordinary_session_allowed(
        &self,
        local_node_id: &str,
        local_guardian_id: &str,
        peer_node_id: &str,
        peer_guardian_id: &str,
    ) -> TransportResult<bool> {
        let view = self
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        Ok(view.excluded_identity.as_ref().is_none_or(|excluded| {
            excluded != &(local_node_id.to_owned(), local_guardian_id.to_owned())
                && excluded != &(peer_node_id.to_owned(), peer_guardian_id.to_owned())
        }))
    }

    fn current_voter_cut_sha256(&self) -> TransportResult<[u8; 32]> {
        self.view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?
            .voter_cut_sha256
            .ok_or(TransportError::InvalidSessionBinding)
    }

    fn expected_peer_instance(&self, guardian_id: &str) -> TransportResult<Option<[u8; 32]>> {
        Ok(self
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?
            .peer_instances
            .get(guardian_id)
            .copied())
    }

    fn learner_wire_session(
        &self,
        voter_cut_sha256: [u8; 32],
        operation_sha256: [u8; 32],
        peer_identity_key: String,
    ) -> TransportResult<LearnerWireSession> {
        let view = self
            .view
            .lock()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        if view.voter_cut_sha256 != Some(voter_cut_sha256)
            || view.learner_operation_sha256 != Some(operation_sha256)
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(LearnerWireSession {
            authority: self.clone(),
            voter_cut_sha256,
            operation_sha256,
            expected_peer_instance_id: view.peer_instances.get(&peer_identity_key).copied(),
            peer_identity_key,
        })
    }
}

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
    #[prost(bytes = "vec", tag = "12")]
    sender_authority_instance_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "13")]
    receiver_authority_instance_id: Vec<u8>,
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

    pub(crate) fn polis_id(&self) -> &str {
        &self.polis_id
    }

    pub(crate) fn trust_domain(&self) -> &str {
        &self.trust_domain
    }
    pub fn local_node_id(&self) -> &str {
        &self.local_node_id
    }
    pub(crate) fn local_guardian_id(&self) -> &str {
        &self.local_guardian_id
    }
    pub(crate) fn local_control_public_key(&self) -> [u8; 32] {
        self.local_control_public_key
    }
    pub fn local_certificate_generation(&self) -> u64 {
        self.local_certificate_generation
    }
    pub fn local_boot_generation(&self) -> u64 {
        self.local_boot_generation
    }
    pub fn peer_node_id(&self) -> &str {
        &self.peer_node_id
    }
    pub(crate) fn peer_guardian_id(&self) -> &str {
        &self.peer_guardian_id
    }
    pub(crate) fn peer_control_public_key(&self) -> [u8; 32] {
        self.peer_control_public_key
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
    authority: ProductionTransportAuthority,
    voter_cut_sha256: [u8; 32],
    expected_peer_instance_id: Option<[u8; 32]>,
    peer_identity_key: String,
}

impl PendingPolisSession {
    fn new(
        binding: PolisSessionBinding,
        authority: ProductionTransportAuthority,
        peer_identity_key: String,
    ) -> Self {
        let voter_cut_sha256 = authority
            .current_voter_cut_sha256()
            .expect("bound production transport authority");
        let expected_peer_instance_id = authority
            .expected_peer_instance(&peer_identity_key)
            .expect("bound production transport authority");
        Self {
            binding,
            authority,
            voter_cut_sha256,
            expected_peer_instance_id,
            peer_identity_key,
        }
    }
}

#[derive(Clone)]
pub struct EstablishedPolisSession {
    binding: PolisSessionBinding,
    authority: ProductionTransportAuthority,
    voter_cut_sha256: [u8; 32],
    peer_authority_instance_id: [u8; 32],
    peer_identity_key: String,
}

impl EstablishedPolisSession {
    pub fn binding(&self) -> &PolisSessionBinding {
        &self.binding
    }

    fn revalidate_ordinary_authority(&self) -> TransportResult<()> {
        if self.authority.current_voter_cut_sha256()? != self.voter_cut_sha256 {
            return Err(TransportError::InvalidSessionBinding);
        }
        if self.authority.ordinary_session_allowed(
            &self.binding.local_node_id,
            &self.binding.local_guardian_id,
            &self.binding.peer_node_id,
            &self.binding.peer_guardian_id,
        )? {
            Ok(())
        } else {
            Err(TransportError::InvalidSessionBinding)
        }
    }

    fn authority_for_same_runtime(&self) -> ProductionTransportAuthority {
        self.authority.clone()
    }

    pub(crate) fn peer_authority_instance_id(&self) -> [u8; 32] {
        self.peer_authority_instance_id
    }

    pub(crate) fn peer_identity_key(&self) -> &str {
        &self.peer_identity_key
    }
}

#[derive(Clone, Debug)]
struct VerifiedRouteAuthority {
    node_id: String,
    guardian_id: String,
    control_public_key: [u8; 32],
    boot_generation: u64,
}

/// Opaque authority accepted by the configured Runtime trust roots.
///
/// Route and polis verification consume this handle rather than accepting a
/// caller-nominated `AuthorityMembership` at the authorization boundary.
#[derive(Clone)]
pub struct EstablishedRuntimeAuthority {
    membership: MembershipState,
    authority: AuthorityMembership,
    certificate_store: TransportCertificateAuthority,
    guardian_certificates: BTreeMap<Vec<u8>, String>,
    authorization_deadline_unix_seconds: u64,
}

/// Production bootstrap owner for one configured Runtime authority lineage.
///
/// The membership is restored from canonical durable bytes against an
/// externally retained commitment before any signed voter lineage can be
/// accepted. The configured certificate store owns the immutable approved
/// issuer roots for this Runtime instance.
pub(crate) struct RuntimeAuthorityInitializer {
    membership: MembershipState,
    certificate_store: TransportCertificateAuthority,
}

impl RuntimeAuthorityInitializer {
    pub(crate) fn restore(
        certificate_store: AuthorityBoundCertificateStore,
        membership_policy: MembershipPolicy,
        membership_snapshot: &[u8],
        trusted_membership_commitment: [u8; 32],
    ) -> TransportResult<Self> {
        let membership = MembershipState::restore(
            membership_policy,
            membership_snapshot,
            trusted_membership_commitment,
        )
        .map_err(|_| TransportError::InvalidSessionBinding)?;
        Ok(Self {
            membership,
            certificate_store: TransportCertificateAuthority::Bound(certificate_store),
        })
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn restore_for_test(
        certificate_store: Arc<DistributedCertificateStore>,
        membership_policy: MembershipPolicy,
        membership_snapshot: &[u8],
        trusted_membership_commitment: [u8; 32],
    ) -> TransportResult<Self> {
        let membership = MembershipState::restore(
            membership_policy,
            membership_snapshot,
            trusted_membership_commitment,
        )
        .map_err(|_| TransportError::InvalidSessionBinding)?;
        Ok(Self {
            membership,
            certificate_store: TransportCertificateAuthority::TestRaw(certificate_store),
        })
    }

    pub(crate) fn accept_signed_lineage(
        &self,
        authority: &AuthorityMembership,
        guardian_certificates: &BTreeMap<Vec<u8>, AuthorityCertificate>,
        now_unix_seconds: u64,
    ) -> TransportResult<EstablishedRuntimeAuthority> {
        EstablishedRuntimeAuthority::accept(
            &self.membership,
            authority,
            self.certificate_store.clone(),
            guardian_certificates,
            now_unix_seconds,
        )
    }
}

impl EstablishedRuntimeAuthority {
    fn accept(
        membership: &MembershipState,
        authority: &AuthorityMembership,
        certificate_store: TransportCertificateAuthority,
        guardian_certificates: &BTreeMap<Vec<u8>, AuthorityCertificate>,
        now_unix_seconds: u64,
    ) -> TransportResult<Self> {
        if now_unix_seconds == 0
            || authority.trust_domain_id.as_slice() != membership.trust_domain().as_bytes()
            || authority.committed_log_index != membership.committed_log_index()
            || authority.voters.len() != 3
            || guardian_certificates.len() != authority.voters.len()
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
        if members.len() != 3 || configured.len() != 3 {
            return Err(TransportError::InvalidSessionBinding);
        }
        let mut authorization_deadline = u64::MAX;
        let mut seen = BTreeSet::new();
        for member in members {
            let guardian = member.guardian_id.as_bytes().to_vec();
            let voter = authority
                .voters
                .get(&guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let raft_id = authority
                .raft_ids
                .get(&guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let certificate = guardian_certificates
                .get(&guardian)
                .ok_or(TransportError::InvalidSessionBinding)?;
            let body = &certificate.body;
            if !configured.contains(raft_id)
                || voter.revoked
                || voter.purpose != ControlCertificatePurpose::AuthorityEndorsement
                || voter.control_public_key != member.guardian_control_public_key
                || voter.certificate_generation != member.identity_generation
                || body.trust_domain != membership.trust_domain()
                || body.holder_id != member.guardian_id
                || body.purpose != CertificatePurpose::GuardianControl
                || body.generation != member.identity_generation
                || body.subject_public_key != member.guardian_control_public_key
            {
                return Err(TransportError::InvalidSessionBinding);
            }
            let verified = certificate_store
                .authorize(
                    &member.guardian_id,
                    CertificatePurpose::GuardianControl,
                    member.identity_generation,
                    now_unix_seconds,
                )
                .map_err(|_| TransportError::InvalidSessionBinding)?;
            if verified.certificate_id
                != certificate
                    .certificate_id()
                    .map_err(|_| TransportError::InvalidSessionBinding)?
            {
                return Err(TransportError::InvalidSessionBinding);
            }
            authorization_deadline =
                authorization_deadline.min(verified.authorization_deadline_unix_secs);
            seen.insert(guardian);
        }
        if seen != authority.voters.keys().cloned().collect() {
            return Err(TransportError::InvalidSessionBinding);
        }
        Ok(Self {
            membership: membership.clone(),
            authority: authority.clone(),
            certificate_store,
            guardian_certificates: guardian_certificates
                .iter()
                .map(|(guardian, certificate)| {
                    certificate
                        .certificate_id()
                        .map(|id| (guardian.clone(), id))
                        .map_err(|_| TransportError::InvalidSessionBinding)
                })
                .collect::<TransportResult<_>>()?,
            authorization_deadline_unix_seconds: authorization_deadline,
        })
    }

    fn revalidate(&self, now_unix_seconds: u64) -> TransportResult<()> {
        if now_unix_seconds == 0 || now_unix_seconds >= self.authorization_deadline_unix_seconds {
            return Err(TransportError::InvalidSessionBinding);
        }
        for (guardian, voter) in &self.authority.voters {
            let holder =
                std::str::from_utf8(guardian).map_err(|_| TransportError::InvalidSessionBinding)?;
            let verified = self
                .certificate_store
                .authorize(
                    holder,
                    CertificatePurpose::GuardianControl,
                    voter.certificate_generation,
                    now_unix_seconds,
                )
                .map_err(|_| TransportError::InvalidSessionBinding)?;
            if self.guardian_certificates.get(guardian) != Some(&verified.certificate_id) {
                return Err(TransportError::InvalidSessionBinding);
            }
        }
        Ok(())
    }
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
        established: &EstablishedRuntimeAuthority,
    ) -> TransportResult<Self> {
        let authority = &established.authority;
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
    membership_epoch: u64,
    committed_membership_index: u64,
    routes: BTreeMap<u64, SocketAddr>,
    authorities: BTreeMap<u64, VerifiedRouteAuthority>,
    authority_membership: AuthorityMembership,
}

/// Opaque ordinary-session exclusion boundary. Implementations are durable
/// published snapshots; callers cannot supply node eligibility as a boolean.
pub(crate) trait OrdinarySessionExclusion {
    fn ordinary_session_allowed(&self, node_id: &str, guardian_id: &str) -> bool;
}

impl OrdinarySessionExclusion for ProductionTransportAuthority {
    fn ordinary_session_allowed(&self, node_id: &str, guardian_id: &str) -> bool {
        self.ordinary_session_allowed(node_id, guardian_id, node_id, guardian_id)
            .unwrap_or(false)
    }
}

impl VerifiedPolisRouteCut {
    #[cfg(test)]
    pub(crate) fn test_from_parts(
        polis_id: &str,
        trust_domain: &str,
        routes: BTreeMap<u64, SocketAddr>,
        identities: BTreeMap<u64, (String, String, [u8; 32], u64)>,
    ) -> Self {
        let voters = identities
            .iter()
            .map(
                |(_, (_, guardian_id, control_public_key, _))| VoterAuthority {
                    guardian_id: guardian_id.as_bytes().to_vec(),
                    trust_domain_id: trust_domain.as_bytes().to_vec(),
                    certificate_generation: 1,
                    purpose: ControlCertificatePurpose::AuthorityEndorsement,
                    not_before_unix_seconds: 1,
                    not_after_unix_seconds: i64::MAX,
                    revoked: false,
                    control_public_key: *control_public_key,
                },
            )
            .collect::<Vec<_>>();
        let configs = vec![voters
            .iter()
            .map(|voter| voter.guardian_id.clone())
            .collect()];
        let authority_membership =
            AuthorityMembership::new(trust_domain.as_bytes().to_vec(), 1, 1, configs, voters)
                .expect("test authority membership");
        let authorities = identities
            .into_iter()
            .map(
                |(node, (node_id, guardian_id, control_public_key, boot_generation))| {
                    (
                        node,
                        VerifiedRouteAuthority {
                            node_id,
                            guardian_id,
                            control_public_key,
                            boot_generation,
                        },
                    )
                },
            )
            .collect();
        Self {
            polis_id: polis_id.to_owned(),
            trust_domain: trust_domain.to_owned(),
            membership_epoch: 1,
            committed_membership_index: 1,
            routes,
            authorities,
            authority_membership,
        }
    }

    pub fn verify(
        polis: &PolisIdentityBinding,
        established: &EstablishedRuntimeAuthority,
        addresses: &BTreeMap<String, SocketAddr>,
        now_unix_seconds: i64,
    ) -> TransportResult<Self> {
        let membership = &established.membership;
        let authority = &established.authority;
        established.revalidate(
            u64::try_from(now_unix_seconds).map_err(|_| TransportError::InvalidSessionBinding)?,
        )?;
        if polis.trust_domain != membership.trust_domain()
            || polis.committed_membership_index != membership.committed_log_index()
            || authority.trust_domain_id.as_slice() != membership.trust_domain().as_bytes()
            || authority.committed_log_index != membership.committed_log_index()
            || now_unix_seconds <= 0
            || u64::try_from(now_unix_seconds)
                .ok()
                .is_none_or(|now| now >= established.authorization_deadline_unix_seconds)
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
            membership_epoch: membership.epoch(),
            committed_membership_index: membership.committed_log_index(),
            routes,
            authorities,
            authority_membership: authority.clone(),
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

    pub(crate) fn polis_id(&self) -> &str {
        &self.polis_id
    }

    pub(crate) fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub(crate) fn membership_epoch(&self) -> u64 {
        self.membership_epoch
    }

    pub(crate) fn authority_node_identity(&self, node: u64) -> Option<(String, String, u64)> {
        let authority = self.authorities.get(&node)?;
        Some((
            authority.node_id.clone(),
            String::from_utf8(authority.guardian_id.as_bytes().to_vec()).ok()?,
            authority.boot_generation,
        ))
    }

    pub(crate) fn authority_membership(&self) -> &AuthorityMembership {
        &self.authority_membership
    }

    pub(crate) fn authority_boot_generations(&self) -> BTreeMap<Vec<u8>, u64> {
        self.authorities
            .values()
            .map(|authority| {
                (
                    authority.guardian_id.as_bytes().to_vec(),
                    authority.boot_generation,
                )
            })
            .collect()
    }

    fn pending_session_with_exclusion(
        &self,
        local: u64,
        peer: u64,
        connection: &AuthenticatedConnection,
        authority: ProductionTransportAuthority,
    ) -> TransportResult<PendingPolisSession> {
        self.pending_session_with_snapshot(local, peer, connection, &authority, authority.clone())
    }

    fn pending_session_with_snapshot(
        &self,
        local: u64,
        peer: u64,
        connection: &AuthenticatedConnection,
        exclusion: &dyn OrdinarySessionExclusion,
        authority: ProductionTransportAuthority,
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
        if !exclusion
            .ordinary_session_allowed(&local_authority.node_id, &local_authority.guardian_id)
            || !exclusion
                .ordinary_session_allowed(&peer_authority.node_id, &peer_authority.guardian_id)
        {
            return Err(TransportError::InvalidSessionBinding);
        }
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
            authority,
            transport_peer_identity_key(
                LearnerEndpointRole::Voter,
                peer,
                &peer_authority.node_id,
                &peer_authority.guardian_id,
            )?,
        ))
    }

    pub(crate) fn session_matches_with_exclusion(
        &self,
        local: u64,
        peer: u64,
        connection: &AuthenticatedConnection,
        established: &EstablishedPolisSession,
        exclusion: &dyn OrdinarySessionExclusion,
    ) -> bool {
        self.pending_session_with_snapshot(
            local,
            peer,
            connection,
            exclusion,
            established.authority_for_same_runtime(),
        )
        .is_ok_and(|pending| pending.binding == established.binding)
    }

    pub fn same_polis_and_domain(&self, other: &Self) -> bool {
        self.polis_id == other.polis_id && self.trust_domain == other.trust_domain
    }

    pub(crate) fn same_authority_lineage(&self, other: &Self) -> bool {
        self.authorities.len() == other.authorities.len()
            && self.authorities.iter().all(|(node, authority)| {
                other.authorities.get(node).is_some_and(|candidate| {
                    candidate.node_id == authority.node_id
                        && candidate.guardian_id == authority.guardian_id
                        && candidate.control_public_key == authority.control_public_key
                })
            })
    }

    pub fn boot_generation(&self, node: u64) -> Option<u64> {
        self.authorities
            .get(&node)
            .map(|value| value.boot_generation)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn exact_removal_target_matches(
        &self,
        stable_raft_id: u64,
        trust_domain: &str,
        polis_id: &str,
        node_id: &str,
        guardian_id: &str,
        guardian_control_public_key: [u8; 32],
        certificate_generation: u64,
        boot_generation: u64,
        address: SocketAddr,
    ) -> bool {
        let Some(route) = self.routes.get(&stable_raft_id) else {
            return false;
        };
        let Some(authority) = self.authorities.get(&stable_raft_id) else {
            return false;
        };
        let Some(voter) = self
            .authority_membership
            .voters
            .get(authority.guardian_id.as_bytes())
        else {
            return false;
        };
        trust_domain == self.trust_domain
            && polis_id == self.polis_id
            && node_id == authority.node_id
            && guardian_id == authority.guardian_id
            && guardian_control_public_key == authority.control_public_key
            && guardian_control_public_key == voter.control_public_key
            && certificate_generation == voter.certificate_generation
            && boot_generation == authority.boot_generation
            && address == *route
            && !voter.revoked
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
    authority: ProductionTransportAuthority,
    binding: PolisSessionBinding,
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
}

pub struct PendingPolisResponse {
    receive: quinn::RecvStream,
    connection: Connection,
    limits: TransportLimits,
    cancellation: CancellationToken,
    authorization_deadline: Instant,
    sequence: u64,
    request_sha256: [u8; 32],
    _guard: tokio::sync::OwnedRwLockReadGuard<()>,
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
    store: TransportCertificateAuthority,
    holder_id: String,
    trust_domain: String,
    generation: u64,
    certificate_id: String,
    subject_public_key: [u8; 32],
}

impl TransportAuthorization {
    pub fn new(
        store: AuthorityBoundCertificateStore,
        certificate: &AuthorityCertificate,
    ) -> TransportResult<Self> {
        Self::from_authority(TransportCertificateAuthority::Bound(store), certificate)
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(
        store: Arc<DistributedCertificateStore>,
        certificate: &AuthorityCertificate,
    ) -> TransportResult<Self> {
        Self::from_authority(TransportCertificateAuthority::TestRaw(store), certificate)
    }

    fn from_authority(
        store: TransportCertificateAuthority,
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
    local_address: SocketAddr,
    remote_address: SocketAddr,
    local: PeerBinding,
    expected_peer: PeerBinding,
    local_authorization: TransportAuthorization,
    peer_authorization: TransportAuthorization,
    limits: TransportLimits,
    authorization_deadline: Instant,
    cancellation: CancellationToken,
    replay_window: Mutex<ReplayWindow>,
    role: ConnectionRole,
    authority_instance_id: OnceLock<[u8; 32]>,
    peer_authority_instance_id: OnceLock<[u8; 32]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConnectionRole {
    Dialer,
    Acceptor,
}

impl AuthenticatedConnection {
    #[cfg(test)]
    pub(crate) fn test_stream_frames_sent(&self) -> u64 {
        self.connection.stats().frame_tx.stream
    }

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
        let local_address = endpoint
            .local_addr()
            .map_err(|_| TransportError::Connection)?;
        Self::from_connection(connection, security, ConnectionRole::Dialer, local_address)
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
        let local_address = endpoint
            .local_addr()
            .map_err(|_| TransportError::Connection)?;
        Self::from_connection(
            connection,
            security,
            ConnectionRole::Acceptor,
            local_address,
        )
    }

    fn from_connection(
        connection: Connection,
        security: ConnectionSecurity,
        role: ConnectionRole,
        local_address: SocketAddr,
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
        let remote_address = connection.remote_address();
        Ok(Self {
            connection,
            local_address,
            remote_address,
            local,
            expected_peer,
            local_authorization,
            peer_authorization,
            limits,
            authorization_deadline,
            cancellation,
            replay_window: Mutex::new(ReplayWindow::default()),
            role,
            authority_instance_id: OnceLock::new(),
            peer_authority_instance_id: OnceLock::new(),
        })
    }

    fn bind_authority_instance(&self, instance_id: [u8; 32]) -> TransportResult<()> {
        match self.authority_instance_id.get() {
            Some(current) if *current == instance_id => Ok(()),
            Some(_) => Err(TransportError::InvalidSessionBinding),
            None => self
                .authority_instance_id
                .set(instance_id)
                .map_err(|_| TransportError::InvalidSessionBinding),
        }
    }

    fn bind_peer_authority_instance(&self, instance_id: [u8; 32]) -> TransportResult<()> {
        match self.peer_authority_instance_id.get() {
            Some(current) if *current == instance_id => Ok(()),
            Some(_) => Err(TransportError::InvalidSessionBinding),
            None => self
                .peer_authority_instance_id
                .set(instance_id)
                .map_err(|_| TransportError::InvalidSessionBinding),
        }
    }

    async fn send_inner(&self, sequence: u64, payload: Vec<u8>) -> TransportResult<()> {
        self.require_authority()?;
        if sequence == 0 {
            return Err(TransportError::SequenceInvalid);
        }
        let frame = encode_frame(self.local.envelope(sequence, payload), &self.limits)?;
        let mut stream = self.open_uni_authorized().await?;
        self.write_authorized(&mut stream, &frame).await?;
        stream.finish().map_err(|_| TransportError::Stream)
    }

    async fn receive_inner(&self) -> TransportResult<TransportEnvelope> {
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

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) async fn send(&self, sequence: u64, payload: Vec<u8>) -> TransportResult<()> {
        self.send_inner(sequence, payload).await
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) async fn receive(&self) -> TransportResult<TransportEnvelope> {
        self.receive_inner().await
    }

    async fn dispatch_learner(
        &self,
        permit: LearnerSendPermit,
    ) -> TransportResult<LearnerPendingResponse> {
        self.bind_authority_instance(permit.instance_id)?;
        self.send_inner(permit.sequence, permit.payload).await?;
        Ok(LearnerPendingResponse {
            instance_id: permit.instance_id,
            guard: permit._guard,
        })
    }


    async fn receive_learner_response(
        &self,
        pending: LearnerPendingResponse,
    ) -> TransportResult<LearnerReceivedEnvelope> {
        self.receive_learner(LearnerReceivePermit {
            instance_id: pending.instance_id,
            guard: pending.guard,
        })
        .await
    }

    async fn receive_learner(
        &self,
        permit: LearnerReceivePermit,
    ) -> TransportResult<LearnerReceivedEnvelope> {
        self.bind_authority_instance(permit.instance_id)?;
        let envelope = self.receive_inner().await?;
        Ok(LearnerReceivedEnvelope {
            instance_id: permit.instance_id,
            envelope,
            _guard: permit.guard,
        })
    }

    async fn initiate_learner_handshake(
        &self,
        permit: LearnerHandshakePermit,
    ) -> TransportResult<Vec<u8>> {
        self.bind_authority_instance(permit.instance_id)?;
        let (mut send, mut receive) = self.open_bi_authorized().await?;
        self.write_authorized(&mut send, &permit.request).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        let result = self.read_authorized(&mut receive).await;
        drop(permit._guard);
        result
    }

    async fn accept_learner_handshake(
        &self,
        permit: LearnerAcceptPermit,
    ) -> TransportResult<PendingLearnerHandshake> {
        self.bind_authority_instance(permit.instance_id)?;
        let (send, mut receive) = self.accept_bi_authorized().await?;
        let request = self.read_authorized(&mut receive).await?;
        Ok(PendingLearnerHandshake {
            send,
            connection: self.connection.clone(),
            limits: self.limits.clone(),
            cancellation: self.cancellation.clone(),
            authorization_deadline: self.authorization_deadline,
            request,
            _guard: permit.guard,
        })
    }

    async fn respond_learner_handshake(
        &self,
        mut pending: PendingLearnerHandshake,
        response: Vec<u8>,
    ) -> TransportResult<()> {
        if pending.connection.stable_id() != self.connection.stable_id()
            || pending.cancellation.is_cancelled()
            || Instant::now() >= pending.authorization_deadline
            || response.len() > pending.limits.max_frame_bytes
        {
            return Err(TransportError::InvalidSessionBinding);
        }
        let result = async {
            self.write_authorized(&mut pending.send, &response).await?;
            pending.send.finish().map_err(|_| TransportError::Stream)
        }
        .await;
        drop(pending._guard);
        result
    }

    fn local_peer_route(&self) -> (&PeerBinding, &PeerBinding) {
        (&self.local, &self.expected_peer)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn matches_learner_route(
        &self,
        endpoint_role: LearnerEndpointRole,
        trust_domain: &str,
        voter_node_id: &str,
        voter_guardian_id: &str,
        voter_certificate_generation: u64,
        learner_node_id: &str,
        learner_guardian_id: &str,
        learner_certificate_generation: u64,
        authorized_learner_address: SocketAddr,
    ) -> bool {
        let voter_matches = |binding: &PeerBinding| {
            binding.trust_domain == trust_domain
                && binding.node_id == voter_node_id
                && binding.guardian_id == voter_guardian_id
                && binding.certificate_generation == voter_certificate_generation
        };
        let learner_matches = |binding: &PeerBinding| {
            binding.trust_domain == trust_domain
                && binding.node_id == learner_node_id
                && binding.guardian_id == learner_guardian_id
                && binding.certificate_generation == learner_certificate_generation
        };
        match endpoint_role {
            LearnerEndpointRole::Voter => {
                self.role == ConnectionRole::Dialer
                    && voter_matches(&self.local)
                    && learner_matches(&self.expected_peer)
                    && self.remote_address == authorized_learner_address
            }
            LearnerEndpointRole::Learner => {
                self.role == ConnectionRole::Acceptor
                    && learner_matches(&self.local)
                    && voter_matches(&self.expected_peer)
                    && self.local_address == authorized_learner_address
            }
        }
    }

    pub(crate) fn local_certificate_generation(&self) -> u64 {
        self.local.certificate_generation
    }

    pub(crate) fn peer_certificate_generation(&self) -> u64 {
        self.expected_peer.certificate_generation
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
        let _dispatch = pending.authority.dispatch_guard().await;
        self.bind_authority_instance(pending.authority.instance_id)?;
        if !pending.authority.ordinary_session_allowed(
            &pending.binding.local_node_id,
            &pending.binding.local_guardian_id,
            &pending.binding.peer_node_id,
            &pending.binding.peer_guardian_id,
        )? {
            return Err(TransportError::InvalidSessionBinding);
        }
        self.require_polis_binding(&pending.binding)?;
        validate_local_control_key(&pending.binding, signing_key)?;
        let request = signed_handshake(
            &pending.binding,
            pending.authority.instance_id,
            pending.expected_peer_instance_id,
            signing_key,
        )?;
        let request_bytes = encode_prost_frame(&request, &self.limits)?;
        let (mut send, mut receive) = self.open_bi_authorized().await?;
        self.write_authorized(&mut send, &request_bytes).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        let response = self.read_authorized(&mut receive).await?;
        let response: PolisHandshake = decode_prost_frame(&response, &self.limits)?;
        verify_handshake(
            &response,
            &pending.binding.reverse(),
            pending.expected_peer_instance_id,
            pending.authority.instance_id,
        )?;
        let peer_authority_instance_id = response
            .sender_authority_instance_id
            .as_slice()
            .try_into()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        self.bind_peer_authority_instance(peer_authority_instance_id)?;
        Ok(EstablishedPolisSession {
            binding: pending.binding,
            authority: pending.authority,
            voter_cut_sha256: pending.voter_cut_sha256,
            peer_authority_instance_id,
            peer_identity_key: pending.peer_identity_key,
        })
    }

    pub async fn accept_polis_session(
        &self,
        pending: PendingPolisSession,
        signing_key: &SigningKey,
    ) -> TransportResult<EstablishedPolisSession> {
        let _dispatch = pending.authority.dispatch_guard().await;
        self.bind_authority_instance(pending.authority.instance_id)?;
        if !pending.authority.ordinary_session_allowed(
            &pending.binding.local_node_id,
            &pending.binding.local_guardian_id,
            &pending.binding.peer_node_id,
            &pending.binding.peer_guardian_id,
        )? {
            return Err(TransportError::InvalidSessionBinding);
        }
        self.require_polis_binding(&pending.binding)?;
        validate_local_control_key(&pending.binding, signing_key)?;
        let (mut send, mut receive) = self.accept_bi_authorized().await?;
        let request = self.read_authorized(&mut receive).await?;
        let request: PolisHandshake = decode_prost_frame(&request, &self.limits)?;
        verify_handshake(
            &request,
            &pending.binding.reverse(),
            pending.expected_peer_instance_id,
            pending.authority.instance_id,
        )?;
        let peer_authority_instance_id = request
            .sender_authority_instance_id
            .as_slice()
            .try_into()
            .map_err(|_| TransportError::InvalidSessionBinding)?;
        self.bind_peer_authority_instance(peer_authority_instance_id)?;
        let response = signed_handshake(
            &pending.binding,
            pending.authority.instance_id,
            Some(peer_authority_instance_id),
            signing_key,
        )?;
        let response = encode_prost_frame(&response, &self.limits)?;
        self.write_authorized(&mut send, &response).await?;
        send.finish().map_err(|_| TransportError::Stream)?;
        Ok(EstablishedPolisSession {
            binding: pending.binding,
            authority: pending.authority,
            voter_cut_sha256: pending.voter_cut_sha256,
            peer_authority_instance_id,
            peer_identity_key: pending.peer_identity_key,
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
        let dispatch = session.authority.dispatch_guard().await;
        self.bind_authority_instance(session.authority.instance_id)?;
        session.revalidate_ordinary_authority()?;
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
        #[cfg(test)]
        session
            .authority
            .pause_after_revalidation_for_test("begin_polis_request")
            .await;
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
            _guard: dispatch,
        })
    }

    pub async fn accept_polis_request(
        &self,
        session: &EstablishedPolisSession,
    ) -> TransportResult<IncomingPolisRequest> {
        let dispatch = session.authority.dispatch_guard().await;
        self.bind_authority_instance(session.authority.instance_id)?;
        session.revalidate_ordinary_authority()?;
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
            authority: session.authority.clone(),
            binding: session.binding.clone(),
            _guard: dispatch,
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
        if !self.authority.ordinary_session_allowed(
            &self.binding.local_node_id,
            &self.binding.local_guardian_id,
            &self.binding.peer_node_id,
            &self.binding.peer_guardian_id,
        )? {
            return Err(TransportError::InvalidSessionBinding);
        }
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
    sender_authority_instance_id: [u8; 32],
    receiver_authority_instance_id: Option<[u8; 32]>,
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
        sender_authority_instance_id: sender_authority_instance_id.to_vec(),
        receiver_authority_instance_id: receiver_authority_instance_id
            .unwrap_or([0; 32])
            .to_vec(),
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
    expected_sender_instance_id: Option<[u8; 32]>,
    receiver_authority_instance_id: [u8; 32],
) -> TransportResult<()> {
    let sender_instance_id: [u8; 32] = handshake
        .sender_authority_instance_id
        .as_slice()
        .try_into()
        .map_err(|_| TransportError::InvalidSessionBinding)?;
    let receiver_instance_id: [u8; 32] = handshake
        .receiver_authority_instance_id
        .as_slice()
        .try_into()
        .map_err(|_| TransportError::InvalidSessionBinding)?;
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
        || sender_instance_id == [0; 32]
        || expected_sender_instance_id.is_some_and(|expected| expected != sender_instance_id)
        || (receiver_instance_id != [0; 32]
            && receiver_instance_id != receiver_authority_instance_id)
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
