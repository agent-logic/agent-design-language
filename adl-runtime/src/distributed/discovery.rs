use std::{collections::BTreeMap, fmt, future::Future, net::SocketAddr, time::Duration};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio_util::sync::CancellationToken;

pub const JOIN_REQUEST_SCHEMA: &str = "adl.distributed.join_request.v1";
pub const JOIN_PROPOSAL_SCHEMA: &str = "adl.distributed.join_proposal.v1";
const PROPOSAL_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-JOIN-PROPOSAL-V1\0";
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_SEEDS: usize = 64;
const MAX_MESSAGE_BYTES: usize = 64 * 1024;
const MAX_ATTEMPT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CLOCK_SKEW_SECS: u64 = 5 * 60;
const MAX_REQUEST_LIFETIME_SECS: u64 = 10 * 60;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiscoveryError {
    InvalidPolicy,
    InvalidSeed,
    DuplicateSeed,
    TooManySeeds,
    InvalidRequest,
    RequestTooLarge,
    RequestNotYetValid,
    RequestExpired,
    WrongDomain,
    UnexpectedPeer,
    PeerNotEnrolled,
    MalformedMessage,
    Replay,
    ResourceExhausted,
    Timeout,
    Cancelled,
    Transport,
    NoSeedAccepted,
}

impl DiscoveryError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidSeed => "invalid_seed",
            Self::DuplicateSeed => "duplicate_seed",
            Self::TooManySeeds => "too_many_seeds",
            Self::InvalidRequest => "invalid_request",
            Self::RequestTooLarge => "request_too_large",
            Self::RequestNotYetValid => "request_not_yet_valid",
            Self::RequestExpired => "request_expired",
            Self::WrongDomain => "wrong_domain",
            Self::UnexpectedPeer => "unexpected_peer",
            Self::PeerNotEnrolled => "peer_not_enrolled",
            Self::MalformedMessage => "malformed_message",
            Self::Replay => "replay",
            Self::ResourceExhausted => "resource_exhausted",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::Transport => "transport_error",
            Self::NoSeedAccepted => "no_seed_accepted",
        }
    }
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for DiscoveryError {}

pub type DiscoveryResult<T> = Result<T, DiscoveryError>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SeedEndpoint {
    pub address: SocketAddr,
    pub expected_node_id: String,
    pub expected_guardian_id: String,
    pub expected_certificate_generation: u64,
}

impl SeedEndpoint {
    pub fn new(
        address: SocketAddr,
        expected_node_id: impl Into<String>,
        expected_guardian_id: impl Into<String>,
        expected_certificate_generation: u64,
    ) -> DiscoveryResult<Self> {
        let seed = Self {
            address,
            expected_node_id: expected_node_id.into(),
            expected_guardian_id: expected_guardian_id.into(),
            expected_certificate_generation,
        };
        seed.validate()?;
        Ok(seed)
    }

    fn validate(&self) -> DiscoveryResult<()> {
        validate_identifier(&self.expected_node_id)?;
        validate_identifier(&self.expected_guardian_id)?;
        if self.address.port() == 0 || self.expected_certificate_generation == 0 {
            return Err(DiscoveryError::InvalidSeed);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct DiscoveryPolicy {
    trust_domain: String,
    protocol_version: u32,
    attempt_timeout: Duration,
    max_clock_skew_secs: u64,
    max_request_lifetime_secs: u64,
    max_message_bytes: usize,
}

impl DiscoveryPolicy {
    pub fn new(
        trust_domain: impl Into<String>,
        protocol_version: u32,
        attempt_timeout: Duration,
    ) -> DiscoveryResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            protocol_version,
            attempt_timeout,
            max_clock_skew_secs: 30,
            max_request_lifetime_secs: 120,
            max_message_bytes: 16 * 1024,
        };
        policy.validate()?;
        Ok(policy)
    }

    pub fn with_bounds(
        mut self,
        max_clock_skew_secs: u64,
        max_request_lifetime_secs: u64,
        max_message_bytes: usize,
    ) -> DiscoveryResult<Self> {
        self.max_clock_skew_secs = max_clock_skew_secs;
        self.max_request_lifetime_secs = max_request_lifetime_secs;
        self.max_message_bytes = max_message_bytes;
        self.validate()?;
        Ok(self)
    }

    fn validate(&self) -> DiscoveryResult<()> {
        validate_identifier(&self.trust_domain).map_err(|_| DiscoveryError::InvalidPolicy)?;
        if self.protocol_version == 0
            || self.attempt_timeout.is_zero()
            || self.attempt_timeout > MAX_ATTEMPT_TIMEOUT
            || self.max_clock_skew_secs > MAX_CLOCK_SKEW_SECS
            || self.max_request_lifetime_secs == 0
            || self.max_request_lifetime_secs > MAX_REQUEST_LIFETIME_SECS
            || !(256..=MAX_MESSAGE_BYTES).contains(&self.max_message_bytes)
        {
            return Err(DiscoveryError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JoinRequest {
    pub schema: String,
    pub trust_domain: String,
    pub protocol_version: u32,
    pub request_id: [u8; 32],
    pub node_id: String,
    pub guardian_id: String,
    pub identity_generation: u64,
    pub issued_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestValidity {
    pub issued_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

impl JoinRequest {
    pub fn new(
        identity: EnrolledPeer,
        protocol_version: u32,
        request_id: [u8; 32],
        validity: RequestValidity,
    ) -> Self {
        Self {
            schema: JOIN_REQUEST_SCHEMA.to_owned(),
            trust_domain: identity.trust_domain,
            protocol_version,
            request_id,
            node_id: identity.node_id,
            guardian_id: identity.guardian_id,
            identity_generation: identity.identity_generation,
            issued_at_unix_secs: validity.issued_at_unix_secs,
            expires_at_unix_secs: validity.expires_at_unix_secs,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposedRole {
    NonVoting,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JoinProposal {
    pub schema: String,
    pub trust_domain: String,
    pub protocol_version: u32,
    pub proposal_id: String,
    pub request_id: [u8; 32],
    pub seed_node_id: String,
    pub seed_guardian_id: String,
    pub candidate_node_id: String,
    pub candidate_guardian_id: String,
    pub candidate_identity_generation: u64,
    pub proposed_role: ProposedRole,
    pub expires_at_unix_secs: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticatedEnvelope {
    pub trust_domain: String,
    pub node_id: String,
    pub guardian_id: String,
    pub protocol_version: u32,
    pub certificate_generation: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnrolledPeer {
    pub trust_domain: String,
    pub node_id: String,
    pub guardian_id: String,
    pub identity_generation: u64,
    pub transport_certificate_generation: u64,
}

pub trait EnrollmentAuthority {
    fn enrollment(&self, node_id: &str) -> DiscoveryResult<Option<EnrolledPeer>>;
}

#[derive(Debug)]
pub struct ProposalReplayGuard {
    capacity: usize,
    requests: BTreeMap<[u8; 32], u64>,
    proposals: BTreeMap<String, u64>,
}

impl ProposalReplayGuard {
    pub fn new(capacity: usize) -> DiscoveryResult<Self> {
        if capacity == 0 || capacity > MAX_SEEDS {
            return Err(DiscoveryError::ResourceExhausted);
        }
        Ok(Self {
            capacity,
            requests: BTreeMap::new(),
            proposals: BTreeMap::new(),
        })
    }

    pub fn observe_request(
        &mut self,
        request_id: [u8; 32],
        expires_at_unix_secs: u64,
        now_unix_secs: u64,
    ) -> DiscoveryResult<()> {
        if request_id == [0; 32] {
            return Err(DiscoveryError::InvalidRequest);
        }
        self.prune_expired(now_unix_secs);
        if self.requests.contains_key(&request_id) {
            return Err(DiscoveryError::Replay);
        }
        if self.requests.len() >= self.capacity {
            return Err(DiscoveryError::ResourceExhausted);
        }
        self.requests.insert(request_id, expires_at_unix_secs);
        Ok(())
    }

    pub fn observe_acceptance(
        &mut self,
        request_id: [u8; 32],
        proposal_id: &str,
        expires_at_unix_secs: u64,
        now_unix_secs: u64,
    ) -> DiscoveryResult<()> {
        if request_id == [0; 32] {
            return Err(DiscoveryError::InvalidRequest);
        }
        validate_digest_identifier(proposal_id)?;
        self.prune_expired(now_unix_secs);
        if self.requests.contains_key(&request_id) || self.proposals.contains_key(proposal_id) {
            return Err(DiscoveryError::Replay);
        }
        if self.requests.len() >= self.capacity || self.proposals.len() >= self.capacity {
            return Err(DiscoveryError::ResourceExhausted);
        }
        self.requests.insert(request_id, expires_at_unix_secs);
        self.proposals
            .insert(proposal_id.to_owned(), expires_at_unix_secs);
        Ok(())
    }

    fn prune_expired(&mut self, now_unix_secs: u64) {
        self.requests.retain(|_, expires| *expires >= now_unix_secs);
        self.proposals
            .retain(|_, expires| *expires >= now_unix_secs);
    }
}

pub struct DiscoveryContext<'a> {
    replay: &'a mut ProposalReplayGuard,
    cancellation: &'a CancellationToken,
}

impl<'a> DiscoveryContext<'a> {
    pub fn new(replay: &'a mut ProposalReplayGuard, cancellation: &'a CancellationToken) -> Self {
        Self {
            replay,
            cancellation,
        }
    }
}

pub fn encode_request(request: &JoinRequest, policy: &DiscoveryPolicy) -> DiscoveryResult<Vec<u8>> {
    validate_request(request, policy, request.issued_at_unix_secs)?;
    encode_bounded(request, policy.max_message_bytes)
}

pub fn propose_join<A: EnrollmentAuthority>(
    authenticated_request: &AuthenticatedEnvelope,
    local_seed: &EnrolledPeer,
    authority: &A,
    policy: &DiscoveryPolicy,
    replay: &mut ProposalReplayGuard,
    now_unix_secs: u64,
) -> DiscoveryResult<JoinProposal> {
    validate_authenticated_peer(authenticated_request, None, authority, policy)?;
    validate_enrolled_shape(local_seed, policy)?;
    require_enrolled(local_seed, authority)?;
    let request: JoinRequest =
        decode_bounded(&authenticated_request.payload, policy.max_message_bytes)?;
    validate_request(&request, policy, now_unix_secs)?;
    if request.node_id != authenticated_request.node_id
        || request.guardian_id != authenticated_request.guardian_id
    {
        return Err(DiscoveryError::UnexpectedPeer);
    }
    let enrolled = require_enrolled_identity(
        &request.node_id,
        &request.guardian_id,
        request.identity_generation,
        authority,
        policy,
    )?;
    if enrolled.trust_domain != request.trust_domain {
        return Err(DiscoveryError::WrongDomain);
    }
    replay.observe_request(
        request.request_id,
        request.expires_at_unix_secs,
        now_unix_secs,
    )?;
    let proposal_id = proposal_id(local_seed, &request)?;
    Ok(JoinProposal {
        schema: JOIN_PROPOSAL_SCHEMA.to_owned(),
        trust_domain: policy.trust_domain.clone(),
        protocol_version: policy.protocol_version,
        proposal_id,
        request_id: request.request_id,
        seed_node_id: local_seed.node_id.clone(),
        seed_guardian_id: local_seed.guardian_id.clone(),
        candidate_node_id: request.node_id,
        candidate_guardian_id: request.guardian_id,
        candidate_identity_generation: request.identity_generation,
        proposed_role: ProposedRole::NonVoting,
        expires_at_unix_secs: request.expires_at_unix_secs,
    })
}

pub fn encode_proposal(
    proposal: &JoinProposal,
    policy: &DiscoveryPolicy,
) -> DiscoveryResult<Vec<u8>> {
    validate_proposal_shape(proposal, policy)?;
    encode_bounded(proposal, policy.max_message_bytes)
}

pub fn accept_proposal<A: EnrollmentAuthority>(
    seed: &SeedEndpoint,
    request: &JoinRequest,
    authenticated_response: &AuthenticatedEnvelope,
    authority: &A,
    policy: &DiscoveryPolicy,
    replay: &mut ProposalReplayGuard,
    now_unix_secs: u64,
) -> DiscoveryResult<JoinProposal> {
    seed.validate()?;
    validate_request(request, policy, now_unix_secs)?;
    validate_authenticated_peer(authenticated_response, Some(seed), authority, policy)?;
    let proposal: JoinProposal =
        decode_bounded(&authenticated_response.payload, policy.max_message_bytes)?;
    validate_proposal_shape(&proposal, policy)?;
    if proposal.request_id != request.request_id
        || proposal.candidate_node_id != request.node_id
        || proposal.candidate_guardian_id != request.guardian_id
        || proposal.candidate_identity_generation != request.identity_generation
        || proposal.expires_at_unix_secs != request.expires_at_unix_secs
        || proposal.seed_node_id != authenticated_response.node_id
        || proposal.seed_guardian_id != authenticated_response.guardian_id
    {
        return Err(DiscoveryError::UnexpectedPeer);
    }
    if now_unix_secs > proposal.expires_at_unix_secs {
        return Err(DiscoveryError::RequestExpired);
    }
    let enrolled_seed = require_enrolled_transport(
        &authenticated_response.node_id,
        &authenticated_response.guardian_id,
        authenticated_response.certificate_generation,
        authority,
        policy,
    )?;
    let expected = proposal_id(&enrolled_seed, request)?;
    if proposal.proposal_id != expected {
        return Err(DiscoveryError::MalformedMessage);
    }
    replay.observe_acceptance(
        proposal.request_id,
        &proposal.proposal_id,
        proposal.expires_at_unix_secs,
        now_unix_secs,
    )?;
    Ok(proposal)
}

pub async fn discover<A, C, F, Fut>(
    seeds: &[SeedEndpoint],
    request: &JoinRequest,
    authority: &A,
    policy: &DiscoveryPolicy,
    context: DiscoveryContext<'_>,
    mut clock: C,
    mut exchange: F,
) -> DiscoveryResult<JoinProposal>
where
    A: EnrollmentAuthority,
    C: FnMut() -> DiscoveryResult<u64>,
    F: FnMut(SeedEndpoint, Vec<u8>) -> Fut,
    Fut: Future<Output = DiscoveryResult<AuthenticatedEnvelope>>,
{
    if context.cancellation.is_cancelled() {
        return Err(DiscoveryError::Cancelled);
    }
    let seeds = validated_seeds(seeds)?;
    validate_request(request, policy, clock()?)?;
    require_enrolled_identity(
        &request.node_id,
        &request.guardian_id,
        request.identity_generation,
        authority,
        policy,
    )?;
    let request_bytes = encode_request(request, policy)?;
    let mut saw_timeout = false;
    let mut last_rejection = None;
    for seed in seeds {
        let response = tokio::select! {
            _ = context.cancellation.cancelled() => return Err(DiscoveryError::Cancelled),
            result = tokio::time::timeout(
                policy.attempt_timeout,
                exchange(seed.clone(), request_bytes.clone()),
            ) => match result {
                Ok(Ok(response)) => response,
                Ok(Err(_)) => continue,
                Err(_) => {
                    saw_timeout = true;
                    continue;
                }
            },
        };
        let response_time = clock()?;
        require_enrolled_identity(
            &request.node_id,
            &request.guardian_id,
            request.identity_generation,
            authority,
            policy,
        )?;
        match accept_proposal(
            &seed,
            request,
            &response,
            authority,
            policy,
            context.replay,
            response_time,
        ) {
            Ok(proposal) => return Ok(proposal),
            Err(error) => last_rejection = Some(error),
        }
    }
    if let Some(error) = last_rejection {
        Err(error)
    } else if saw_timeout {
        Err(DiscoveryError::Timeout)
    } else {
        Err(DiscoveryError::NoSeedAccepted)
    }
}

fn validated_seeds(seeds: &[SeedEndpoint]) -> DiscoveryResult<Vec<SeedEndpoint>> {
    if seeds.is_empty() || seeds.len() > MAX_SEEDS {
        return Err(if seeds.len() > MAX_SEEDS {
            DiscoveryError::TooManySeeds
        } else {
            DiscoveryError::InvalidSeed
        });
    }
    let mut ordered = seeds.to_vec();
    ordered.sort();
    for seed in &ordered {
        seed.validate()?;
    }
    if ordered.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(DiscoveryError::DuplicateSeed);
    }
    Ok(ordered)
}

fn validate_authenticated_peer<A: EnrollmentAuthority>(
    envelope: &AuthenticatedEnvelope,
    seed: Option<&SeedEndpoint>,
    authority: &A,
    policy: &DiscoveryPolicy,
) -> DiscoveryResult<()> {
    validate_identifier(&envelope.trust_domain)?;
    validate_identifier(&envelope.node_id)?;
    validate_identifier(&envelope.guardian_id)?;
    if envelope.payload.len() > policy.max_message_bytes {
        return Err(DiscoveryError::RequestTooLarge);
    }
    if envelope.trust_domain != policy.trust_domain {
        return Err(DiscoveryError::WrongDomain);
    }
    if envelope.protocol_version != policy.protocol_version || envelope.certificate_generation == 0
    {
        return Err(DiscoveryError::UnexpectedPeer);
    }
    if let Some(seed) = seed {
        if envelope.node_id != seed.expected_node_id
            || envelope.guardian_id != seed.expected_guardian_id
            || envelope.certificate_generation != seed.expected_certificate_generation
        {
            return Err(DiscoveryError::UnexpectedPeer);
        }
    }
    require_enrolled_transport(
        &envelope.node_id,
        &envelope.guardian_id,
        envelope.certificate_generation,
        authority,
        policy,
    )?;
    Ok(())
}

fn require_enrolled<A: EnrollmentAuthority>(
    peer: &EnrolledPeer,
    authority: &A,
) -> DiscoveryResult<()> {
    let observed = authority
        .enrollment(&peer.node_id)?
        .ok_or(DiscoveryError::PeerNotEnrolled)?;
    if observed != *peer {
        return Err(DiscoveryError::PeerNotEnrolled);
    }
    Ok(())
}

fn require_enrolled_identity<A: EnrollmentAuthority>(
    node_id: &str,
    guardian_id: &str,
    generation: u64,
    authority: &A,
    policy: &DiscoveryPolicy,
) -> DiscoveryResult<EnrolledPeer> {
    let enrolled = authority
        .enrollment(node_id)?
        .ok_or(DiscoveryError::PeerNotEnrolled)?;
    validate_enrolled_shape(&enrolled, policy)?;
    if enrolled.node_id != node_id
        || enrolled.guardian_id != guardian_id
        || enrolled.identity_generation != generation
    {
        return Err(DiscoveryError::PeerNotEnrolled);
    }
    Ok(enrolled)
}

fn require_enrolled_transport<A: EnrollmentAuthority>(
    node_id: &str,
    guardian_id: &str,
    certificate_generation: u64,
    authority: &A,
    policy: &DiscoveryPolicy,
) -> DiscoveryResult<EnrolledPeer> {
    let enrolled = authority
        .enrollment(node_id)?
        .ok_or(DiscoveryError::PeerNotEnrolled)?;
    validate_enrolled_shape(&enrolled, policy)?;
    if enrolled.node_id != node_id
        || enrolled.guardian_id != guardian_id
        || enrolled.transport_certificate_generation != certificate_generation
    {
        return Err(DiscoveryError::PeerNotEnrolled);
    }
    Ok(enrolled)
}

fn validate_enrolled_shape(peer: &EnrolledPeer, policy: &DiscoveryPolicy) -> DiscoveryResult<()> {
    validate_identifier(&peer.node_id)?;
    validate_identifier(&peer.guardian_id)?;
    if peer.trust_domain != policy.trust_domain
        || peer.identity_generation == 0
        || peer.transport_certificate_generation == 0
    {
        return Err(DiscoveryError::PeerNotEnrolled);
    }
    Ok(())
}

fn validate_request(
    request: &JoinRequest,
    policy: &DiscoveryPolicy,
    now_unix_secs: u64,
) -> DiscoveryResult<()> {
    if request.schema != JOIN_REQUEST_SCHEMA
        || request.protocol_version != policy.protocol_version
        || request.identity_generation == 0
        || request.request_id == [0; 32]
    {
        return Err(DiscoveryError::InvalidRequest);
    }
    validate_identifier(&request.trust_domain)?;
    validate_identifier(&request.node_id)?;
    validate_identifier(&request.guardian_id)?;
    if request.trust_domain != policy.trust_domain {
        return Err(DiscoveryError::WrongDomain);
    }
    let lifetime = request
        .expires_at_unix_secs
        .checked_sub(request.issued_at_unix_secs)
        .ok_or(DiscoveryError::InvalidRequest)?;
    if lifetime == 0 || lifetime > policy.max_request_lifetime_secs {
        return Err(DiscoveryError::InvalidRequest);
    }
    if request.issued_at_unix_secs > now_unix_secs.saturating_add(policy.max_clock_skew_secs) {
        return Err(DiscoveryError::RequestNotYetValid);
    }
    if now_unix_secs > request.expires_at_unix_secs {
        return Err(DiscoveryError::RequestExpired);
    }
    Ok(())
}

fn validate_proposal_shape(
    proposal: &JoinProposal,
    policy: &DiscoveryPolicy,
) -> DiscoveryResult<()> {
    if proposal.schema != JOIN_PROPOSAL_SCHEMA
        || proposal.protocol_version != policy.protocol_version
        || proposal.request_id == [0; 32]
        || proposal.candidate_identity_generation == 0
        || proposal.proposed_role != ProposedRole::NonVoting
    {
        return Err(DiscoveryError::MalformedMessage);
    }
    validate_digest_identifier(&proposal.proposal_id)?;
    validate_identifier(&proposal.trust_domain)?;
    validate_identifier(&proposal.seed_node_id)?;
    validate_identifier(&proposal.seed_guardian_id)?;
    validate_identifier(&proposal.candidate_node_id)?;
    validate_identifier(&proposal.candidate_guardian_id)?;
    if proposal.trust_domain != policy.trust_domain {
        return Err(DiscoveryError::WrongDomain);
    }
    Ok(())
}

fn proposal_id(seed: &EnrolledPeer, request: &JoinRequest) -> DiscoveryResult<String> {
    let canonical = serde_json::to_vec(&(
        &request.trust_domain,
        request.protocol_version,
        request.request_id,
        &seed.node_id,
        &seed.guardian_id,
        &request.node_id,
        &request.guardian_id,
        request.identity_generation,
        request.expires_at_unix_secs,
        ProposedRole::NonVoting,
    ))
    .map_err(|_| DiscoveryError::MalformedMessage)?;
    let mut digest = Sha256::new();
    digest.update(PROPOSAL_DOMAIN);
    digest.update((canonical.len() as u64).to_be_bytes());
    digest.update(canonical);
    Ok(hex::encode(digest.finalize()))
}

fn encode_bounded<T: Serialize>(value: &T, limit: usize) -> DiscoveryResult<Vec<u8>> {
    let bytes = serde_json::to_vec(value).map_err(|_| DiscoveryError::MalformedMessage)?;
    if bytes.len() > limit {
        return Err(DiscoveryError::RequestTooLarge);
    }
    Ok(bytes)
}

fn decode_bounded<T: for<'de> Deserialize<'de>>(bytes: &[u8], limit: usize) -> DiscoveryResult<T> {
    if bytes.len() > limit {
        return Err(DiscoveryError::RequestTooLarge);
    }
    serde_json::from_slice(bytes).map_err(|_| DiscoveryError::MalformedMessage)
}

fn validate_identifier(value: &str) -> DiscoveryResult<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(DiscoveryError::MalformedMessage);
    }
    Ok(())
}

fn validate_digest_identifier(value: &str) -> DiscoveryResult<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(DiscoveryError::MalformedMessage);
    }
    Ok(())
}
