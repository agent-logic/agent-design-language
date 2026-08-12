//! Runtime-owned authorization for governed Layer 8 conversation actions.

use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const LAYER8_AUDIT_SCHEMA: &str = "adl.runtime.layer8_authority.audit.v1";
pub const ACIP_IDENTITY_MESSAGE_SCHEMA: &str = "adl.runtime.acip.identity_message.v1";
const GENESIS_HASH: &str = "layer8-authority-genesis-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityMessageKind {
    Request,
    Acknowledgement,
}

/// One identity-bound contract for operator-to-agent and direct agent-to-agent delivery.
/// Private keys are deliberately absent; callers inject externally held signing authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedIdentityMessage {
    pub schema: String,
    pub message_kind: IdentityMessageKind,
    pub message_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub polis_id: String,
    pub conversation_id: String,
    pub correlation_id: String,
    pub causation_id: String,
    pub replay_id: String,
    pub monotonic_sequence: u64,
    pub issued_at_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
    pub payload_json: String,
    pub signing_key_id: String,
    pub signature: String,
}

#[derive(Debug, Clone)]
pub struct CommunicationVerifyingIdentity {
    pub principal_id: String,
    pub polis_id: String,
    pub signing_key_id: String,
    pub verifying_key: VerifyingKey,
    pub revoked: bool,
    pub not_before_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
}

impl SignedIdentityMessage {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, RefusalReason> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_jcs::to_vec(&unsigned).map_err(|_| RefusalReason::InvalidRequest)
    }
}

pub fn verify_signed_identity_message(
    message: &SignedIdentityMessage,
    identity: &CommunicationVerifyingIdentity,
    expected_recipient: &str,
    now_epoch_secs: u64,
) -> Result<(), RefusalReason> {
    if message.schema != ACIP_IDENTITY_MESSAGE_SCHEMA
        || message.message_id.trim().is_empty()
        || message.sender_id.trim().is_empty()
        || message.recipient_id.trim().is_empty()
        || message.polis_id.trim().is_empty()
        || message.conversation_id.trim().is_empty()
        || message.correlation_id.trim().is_empty()
        || message.replay_id.trim().is_empty()
        || message.monotonic_sequence == 0
        || message.recipient_id != expected_recipient
    {
        return Err(RefusalReason::InvalidRequest);
    }
    if identity.revoked {
        return Err(RefusalReason::IdentityRevoked);
    }
    if now_epoch_secs < identity.not_before_epoch_secs
        || now_epoch_secs >= identity.expires_at_epoch_secs
        || now_epoch_secs < message.issued_at_epoch_secs
        || now_epoch_secs >= message.expires_at_epoch_secs
    {
        return Err(RefusalReason::IdentityExpired);
    }
    if message.sender_id != identity.principal_id
        || message.polis_id != identity.polis_id
        || message.signing_key_id != identity.signing_key_id
    {
        return Err(RefusalReason::IdentityUnavailable);
    }
    let payload: serde_json::Value =
        serde_json::from_str(&message.payload_json).map_err(|_| RefusalReason::InvalidRequest)?;
    if serde_jcs::to_string(&payload).map_err(|_| RefusalReason::InvalidRequest)?
        != message.payload_json
    {
        return Err(RefusalReason::InvalidRequest);
    }
    let signature_bytes =
        hex::decode(&message.signature).map_err(|_| RefusalReason::InvalidRequest)?;
    let signature =
        Signature::from_slice(&signature_bytes).map_err(|_| RefusalReason::InvalidRequest)?;
    identity
        .verifying_key
        .verify(&message.signing_bytes()?, &signature)
        .map_err(|_| RefusalReason::IdentityUnavailable)
}

pub fn verify_recipient_acknowledgement(
    request: &SignedIdentityMessage,
    acknowledgement: &SignedIdentityMessage,
    recipient_identity: &CommunicationVerifyingIdentity,
    now_epoch_secs: u64,
) -> Result<(), RefusalReason> {
    if request.message_kind != IdentityMessageKind::Request
        || acknowledgement.message_kind != IdentityMessageKind::Acknowledgement
        || acknowledgement.sender_id != request.recipient_id
        || acknowledgement.recipient_id != request.sender_id
        || acknowledgement.polis_id != request.polis_id
        || acknowledgement.conversation_id != request.conversation_id
        || acknowledgement.correlation_id != request.correlation_id
        || acknowledgement.causation_id != request.message_id
    {
        return Err(RefusalReason::InvalidRequest);
    }
    verify_signed_identity_message(
        acknowledgement,
        recipient_identity,
        &request.sender_id,
        now_epoch_secs,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layer8Action {
    Discover,
    Contact,
    Continue,
    Attach,
    AddressRecipients,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer8Principal {
    pub principal_id: String,
    pub polis_id: String,
    pub credential_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeIdentityEvidence {
    pub principal_id: String,
    pub polis_id: String,
    pub credential_generation: u64,
    pub current_credential_generation: u64,
    pub expires_at_epoch_secs: u64,
    pub revoked: bool,
    pub authenticated: bool,
}

impl RuntimeIdentityEvidence {
    pub fn derive_principal(&self, now: u64) -> Result<Layer8Principal, RefusalReason> {
        if !self.authenticated
            || self.principal_id.trim().is_empty()
            || self.polis_id.trim().is_empty()
        {
            return Err(RefusalReason::IdentityUnavailable);
        }
        if self.revoked {
            return Err(RefusalReason::IdentityRevoked);
        }
        if now >= self.expires_at_epoch_secs {
            return Err(RefusalReason::IdentityExpired);
        }
        if self.credential_generation == 0
            || self.credential_generation != self.current_credential_generation
        {
            return Err(RefusalReason::StaleCredential);
        }
        Ok(Layer8Principal {
            principal_id: self.principal_id.clone(),
            polis_id: self.polis_id.clone(),
            credential_generation: self.credential_generation,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityScope {
    pub polis_id: String,
    pub action: Layer8Action,
    pub conversation_id: Option<String>,
    pub recipients: BTreeSet<String>,
    pub attachment_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer8Capability {
    pub capability_id: String,
    pub principal_id: String,
    pub scope: AuthorityScope,
    pub epoch: u64,
    pub expires_at_epoch_secs: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layer8Policy {
    pub policy_id: String,
    pub available: bool,
    pub scope: AuthorityScope,
    pub epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityRequest {
    pub evidence: RuntimeIdentityEvidence,
    pub action: Layer8Action,
    pub conversation_id: Option<String>,
    pub recipients: BTreeSet<String>,
    pub attachment_id: Option<String>,
    pub replay_id: String,
    pub correlation_id: String,
    pub now_epoch_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalReason {
    InvalidRequest,
    IdentityUnavailable,
    IdentityExpired,
    IdentityRevoked,
    StaleCredential,
    CapabilityDenied,
    CapabilityExpired,
    CapabilityRevoked,
    StaleCapability,
    PolicyUnavailable,
    ScopeDenied,
    ReplayRefused,
    AuditUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublicRefusal {
    pub authorized: bool,
    pub reason: RefusalReason,
    pub retryable: bool,
    pub correlation_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationGrant {
    pub principal: Layer8Principal,
    pub action: Layer8Action,
    pub conversation_id: Option<String>,
    pub recipients: BTreeSet<String>,
    pub correlation_id: String,
    pub audit_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityDecision {
    Authorized(AuthorizationGrant),
    Refused(PublicRefusal),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuditRecord {
    schema: String,
    sequence: u64,
    previous_hash: String,
    record_hash: String,
    timestamp_epoch_secs: u64,
    principal_hash: String,
    polis_hash: String,
    action: Layer8Action,
    conversation_hash: Option<String>,
    recipient_set_hash: String,
    attachment_hash: Option<String>,
    replay_hash: String,
    capability_hash: String,
    policy_hash: String,
    authorized: bool,
    refusal: Option<RefusalReason>,
    correlation_hash: String,
}

#[derive(Debug, Serialize)]
struct AuditHashInput<'a> {
    schema: &'a str,
    sequence: u64,
    previous_hash: &'a str,
    timestamp_epoch_secs: u64,
    principal_hash: &'a str,
    polis_hash: &'a str,
    action: &'a Layer8Action,
    conversation_hash: &'a Option<String>,
    recipient_set_hash: &'a str,
    attachment_hash: &'a Option<String>,
    replay_hash: &'a str,
    capability_hash: &'a str,
    policy_hash: &'a str,
    authorized: bool,
    refusal: &'a Option<RefusalReason>,
    correlation_hash: &'a str,
}

#[derive(Debug)]
struct StoreState {
    sequence: u64,
    head_hash: String,
    replay_hashes: BTreeSet<String>,
}

fn load_store_state(file: &File) -> Result<StoreState, RefusalReason> {
    let mut state = StoreState {
        sequence: 0,
        head_hash: GENESIS_HASH.to_string(),
        replay_hashes: BTreeSet::new(),
    };
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|_| RefusalReason::AuditUnavailable)?;
        if line.trim().is_empty() {
            return Err(RefusalReason::AuditUnavailable);
        }
        let record: AuditRecord =
            serde_json::from_str(&line).map_err(|_| RefusalReason::AuditUnavailable)?;
        if record.schema != LAYER8_AUDIT_SCHEMA
            || record.sequence != state.sequence + 1
            || record.previous_hash != state.head_hash
            || record.record_hash != calculate_record_hash(&record)?
        {
            return Err(RefusalReason::AuditUnavailable);
        }
        if record.authorized {
            state.replay_hashes.insert(record.replay_hash.clone());
        }
        state.sequence = record.sequence;
        state.head_hash = record.record_hash;
    }
    Ok(state)
}

#[derive(Debug)]
pub struct Layer8AuthorityStore {
    path: PathBuf,
    state: Mutex<StoreState>,
}

#[derive(Debug, Clone)]
pub struct ConversationAuthorityProfile {
    pub principal_id: String,
    pub polis_id: String,
    pub current_credential_generation: u64,
    pub identity_expires_at_epoch_secs: u64,
    pub identity_revoked: bool,
    pub policy_epoch: u64,
    pub agent_policy_available: bool,
    pub polis_policy_available: bool,
    pub allowed_actions: BTreeSet<Layer8Action>,
    pub allowed_recipients: BTreeSet<String>,
}

#[derive(Debug)]
pub struct Layer8ConversationAuthority {
    store: Layer8AuthorityStore,
    profile: ConversationAuthorityProfile,
}

impl Layer8ConversationAuthority {
    pub fn new(
        store: Layer8AuthorityStore,
        profile: ConversationAuthorityProfile,
    ) -> Result<Self, RefusalReason> {
        if profile.principal_id.trim().is_empty()
            || profile.polis_id.trim().is_empty()
            || profile.current_credential_generation == 0
            || profile.policy_epoch == 0
            || profile.allowed_actions.is_empty()
        {
            return Err(RefusalReason::InvalidRequest);
        }
        Ok(Self { store, profile })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authorize(
        &self,
        action: Layer8Action,
        conversation_id: String,
        recipient_id: String,
        replay_id: String,
        correlation_id: String,
        now_epoch_secs: u64,
    ) -> AuthorityDecision {
        let recipients = BTreeSet::from([recipient_id]);
        let requested_scope = AuthorityScope {
            polis_id: self.profile.polis_id.clone(),
            action: action.clone(),
            conversation_id: Some(conversation_id.clone()),
            recipients: recipients.clone(),
            attachment_id: None,
        };
        let allowed = self.profile.allowed_actions.contains(&action)
            && recipients.is_subset(&self.profile.allowed_recipients);
        let policy_scope = if allowed {
            requested_scope.clone()
        } else {
            AuthorityScope {
                recipients: BTreeSet::new(),
                ..requested_scope.clone()
            }
        };
        let expires_at_epoch_secs = self.profile.identity_expires_at_epoch_secs;
        let request = AuthorityRequest {
            evidence: RuntimeIdentityEvidence {
                principal_id: self.profile.principal_id.clone(),
                polis_id: self.profile.polis_id.clone(),
                credential_generation: self.profile.current_credential_generation,
                current_credential_generation: self.profile.current_credential_generation,
                expires_at_epoch_secs,
                revoked: self.profile.identity_revoked,
                authenticated: true,
            },
            action,
            conversation_id: Some(conversation_id),
            recipients,
            attachment_id: None,
            replay_id,
            correlation_id,
            now_epoch_secs,
        };
        let capability = Layer8Capability {
            capability_id: "layer8-conversation".to_owned(),
            principal_id: self.profile.principal_id.clone(),
            scope: requested_scope,
            epoch: self.profile.policy_epoch,
            expires_at_epoch_secs,
            revoked: false,
        };
        let agent_policy = Layer8Policy {
            policy_id: "agent-conversation-policy".to_owned(),
            available: self.profile.agent_policy_available,
            scope: policy_scope.clone(),
            epoch: self.profile.policy_epoch,
        };
        let polis_policy = Layer8Policy {
            policy_id: "polis-conversation-policy".to_owned(),
            available: self.profile.polis_policy_available,
            scope: policy_scope,
            epoch: self.profile.policy_epoch,
        };
        self.store
            .authorize(request, &capability, &agent_policy, &polis_policy)
    }
}

impl Layer8AuthorityStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RefusalReason> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| RefusalReason::AuditUnavailable)?;
        }
        let state;
        match File::open(&path) {
            Ok(file) => {
                state = load_store_state(&file)?;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                state = StoreState {
                    sequence: 0,
                    head_hash: GENESIS_HASH.to_string(),
                    replay_hashes: BTreeSet::new(),
                };
            }
            Err(_) => return Err(RefusalReason::AuditUnavailable),
        }
        Ok(Self {
            path,
            state: Mutex::new(state),
        })
    }

    pub fn authorize(
        &self,
        request: AuthorityRequest,
        capability: &Layer8Capability,
        agent_policy: &Layer8Policy,
        polis_policy: &Layer8Policy,
    ) -> AuthorityDecision {
        let correlation_id = bounded_correlation(&request.correlation_id);
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => return refused(RefusalReason::AuditUnavailable, correlation_id),
        };
        let mut file = match OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.path)
        {
            Ok(file) => file,
            Err(_) => return refused(RefusalReason::AuditUnavailable, correlation_id),
        };
        if file.lock_exclusive().is_err() {
            return refused(RefusalReason::AuditUnavailable, correlation_id);
        }
        let current = match load_store_state(&file) {
            Ok(current) => current,
            Err(reason) => return refused(reason, correlation_id),
        };
        *state = current;
        let replay_hash = hash_text(&request.replay_id);
        let principal = request.evidence.derive_principal(request.now_epoch_secs);
        let reason = validate(
            &request,
            principal.as_ref().ok(),
            capability,
            agent_policy,
            polis_policy,
            &state.replay_hashes,
            &replay_hash,
        );
        let authorized = reason.is_none();
        let principal_hash = principal
            .as_ref()
            .map(|p| hash_text(&p.principal_id))
            .unwrap_or_else(|_| hash_text("unknown"));
        let record = build_record(
            &state,
            &request,
            capability,
            agent_policy,
            polis_policy,
            principal_hash,
            replay_hash.clone(),
            authorized,
            reason,
        );
        let record = match record.and_then(|record| Self::append_locked(&mut file, record)) {
            Ok(record) => record,
            Err(reason) => return refused(reason, correlation_id),
        };
        state.sequence = record.sequence;
        state.head_hash = record.record_hash.clone();
        if authorized {
            state.replay_hashes.insert(replay_hash);
        }
        match reason {
            Some(reason) => refused(reason, correlation_id),
            None => AuthorityDecision::Authorized(AuthorizationGrant {
                principal: principal.expect("validated principal"),
                action: request.action,
                conversation_id: request.conversation_id,
                recipients: request.recipients,
                correlation_id,
                audit_hash: record.record_hash,
            }),
        }
    }

    fn append_locked(file: &mut File, record: AuditRecord) -> Result<AuditRecord, RefusalReason> {
        let bytes = serde_json::to_vec(&record).map_err(|_| RefusalReason::AuditUnavailable)?;
        file.write_all(&bytes)
            .and_then(|_| file.write_all(b"\n"))
            .and_then(|_| file.sync_data())
            .map_err(|_| RefusalReason::AuditUnavailable)?;
        Ok(record)
    }
}

fn validate(
    request: &AuthorityRequest,
    principal: Option<&Layer8Principal>,
    capability: &Layer8Capability,
    agent_policy: &Layer8Policy,
    polis_policy: &Layer8Policy,
    replay_hashes: &BTreeSet<String>,
    replay_hash: &str,
) -> Option<RefusalReason> {
    let principal = match principal {
        Some(value) => value,
        None => {
            return request
                .evidence
                .derive_principal(request.now_epoch_secs)
                .err()
        }
    };
    if request.replay_id.trim().is_empty() || request.correlation_id.trim().is_empty() {
        return Some(RefusalReason::InvalidRequest);
    }
    if replay_hashes.contains(replay_hash) {
        return Some(RefusalReason::ReplayRefused);
    }
    if capability.revoked {
        return Some(RefusalReason::CapabilityRevoked);
    }
    if request.now_epoch_secs >= capability.expires_at_epoch_secs {
        return Some(RefusalReason::CapabilityExpired);
    }
    if capability.epoch == 0
        || capability.epoch != agent_policy.epoch
        || capability.epoch != polis_policy.epoch
    {
        return Some(RefusalReason::StaleCapability);
    }
    if !agent_policy.available || !polis_policy.available {
        return Some(RefusalReason::PolicyUnavailable);
    }
    if capability.principal_id != principal.principal_id {
        return Some(RefusalReason::CapabilityDenied);
    }
    if !scope_matches(request, principal, &capability.scope)
        || !scope_matches(request, principal, &agent_policy.scope)
        || !scope_matches(request, principal, &polis_policy.scope)
    {
        return Some(RefusalReason::ScopeDenied);
    }
    None
}

fn scope_matches(
    request: &AuthorityRequest,
    principal: &Layer8Principal,
    scope: &AuthorityScope,
) -> bool {
    scope.polis_id == principal.polis_id
        && scope.action == request.action
        && scope.conversation_id == request.conversation_id
        && scope.recipients == request.recipients
        && scope.attachment_id == request.attachment_id
        && !(request.action == Layer8Action::AddressRecipients && request.recipients.is_empty())
        && !(request.action == Layer8Action::Attach && request.attachment_id.is_none())
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    state: &StoreState,
    request: &AuthorityRequest,
    capability: &Layer8Capability,
    agent_policy: &Layer8Policy,
    polis_policy: &Layer8Policy,
    principal_hash: String,
    replay_hash: String,
    authorized: bool,
    refusal: Option<RefusalReason>,
) -> Result<AuditRecord, RefusalReason> {
    let mut record = AuditRecord {
        schema: LAYER8_AUDIT_SCHEMA.to_string(),
        sequence: state
            .sequence
            .checked_add(1)
            .ok_or(RefusalReason::AuditUnavailable)?,
        previous_hash: state.head_hash.clone(),
        record_hash: String::new(),
        timestamp_epoch_secs: request.now_epoch_secs,
        principal_hash,
        polis_hash: hash_text(&request.evidence.polis_id),
        action: request.action.clone(),
        conversation_hash: request.conversation_id.as_deref().map(hash_text),
        recipient_set_hash: hash_recipients(&request.recipients),
        attachment_hash: request.attachment_id.as_deref().map(hash_text),
        replay_hash,
        capability_hash: hash_text(&capability.capability_id),
        policy_hash: hash_text(&format!(
            "{}:{}",
            agent_policy.policy_id, polis_policy.policy_id
        )),
        authorized,
        refusal,
        correlation_hash: hash_text(&request.correlation_id),
    };
    record.record_hash = calculate_record_hash(&record)?;
    Ok(record)
}

fn calculate_record_hash(record: &AuditRecord) -> Result<String, RefusalReason> {
    let input = AuditHashInput {
        schema: &record.schema,
        sequence: record.sequence,
        previous_hash: &record.previous_hash,
        timestamp_epoch_secs: record.timestamp_epoch_secs,
        principal_hash: &record.principal_hash,
        polis_hash: &record.polis_hash,
        action: &record.action,
        conversation_hash: &record.conversation_hash,
        recipient_set_hash: &record.recipient_set_hash,
        attachment_hash: &record.attachment_hash,
        replay_hash: &record.replay_hash,
        capability_hash: &record.capability_hash,
        policy_hash: &record.policy_hash,
        authorized: record.authorized,
        refusal: &record.refusal,
        correlation_hash: &record.correlation_hash,
    };
    let bytes = serde_jcs::to_vec(&input).map_err(|_| RefusalReason::AuditUnavailable)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn hash_text(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

fn hash_recipients(recipients: &BTreeSet<String>) -> String {
    hash_text(&recipients.iter().cloned().collect::<Vec<_>>().join("\n"))
}

fn bounded_correlation(value: &str) -> String {
    if value.len() <= 128 && !value.chars().any(char::is_control) {
        value.to_string()
    } else {
        hash_text(value)
    }
}

fn refused(reason: RefusalReason, correlation_id: String) -> AuthorityDecision {
    AuthorityDecision::Refused(PublicRefusal {
        authorized: false,
        reason,
        retryable: matches!(
            reason,
            RefusalReason::PolicyUnavailable | RefusalReason::AuditUnavailable
        ),
        correlation_id,
    })
}
