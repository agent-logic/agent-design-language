//! Runtime-owned authorization for governed Layer 8 conversation actions.

use std::collections::BTreeSet;

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

mod audit;
mod exchange;
mod identity;

use audit::scope_matches;
pub use audit::Layer8AuthorityStore;
pub use exchange::{
    sign_recipient_acknowledgement, verify_recipient_acknowledgement,
    verify_signed_identity_message, ConversationSigningProfile, IdentityMessageKind,
    Layer8SignedExchange, SignedIdentityMessage,
};
pub use identity::{
    CommunicationKeyDescriptor, CommunicationVerifyingDescriptor, CommunicationVerifyingIdentity,
    Layer8Principal, RuntimeIdentityEvidence,
};

pub const LAYER8_AUDIT_SCHEMA: &str = "adl.runtime.layer8_authority.audit.v1";
pub const ACIP_IDENTITY_MESSAGE_SCHEMA: &str = "adl.runtime.acip.identity_message.v1";

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
#[serde(deny_unknown_fields)]
pub struct AuthorityScope {
    pub polis_id: String,
    pub action: Layer8Action,
    pub conversation_id: Option<String>,
    pub recipients: BTreeSet<String>,
    pub attachment_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layer8Capability {
    pub capability_id: String,
    pub principal_id: String,
    pub scope: AuthorityScope,
    pub epoch: u64,
    pub expires_at_epoch_secs: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationAuthorityProfile {
    pub evidence: RuntimeIdentityEvidence,
    pub capabilities: Vec<Layer8Capability>,
    pub agent_policies: Vec<Layer8Policy>,
    pub polis_policies: Vec<Layer8Policy>,
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
        if profile.evidence.principal_id.trim().is_empty()
            || profile.evidence.polis_id.trim().is_empty()
            || profile.evidence.current_credential_generation == 0
            || profile.capabilities.is_empty()
            || profile.agent_policies.is_empty()
            || profile.polis_policies.is_empty()
        {
            return Err(RefusalReason::InvalidRequest);
        }
        let verifying_key = hex::decode(&profile.evidence.verifying_key_hex)
            .map_err(|_| RefusalReason::InvalidRequest)?;
        let verifying_key: [u8; 32] = verifying_key
            .try_into()
            .map_err(|_| RefusalReason::InvalidRequest)?;
        VerifyingKey::from_bytes(&verifying_key).map_err(|_| RefusalReason::InvalidRequest)?;
        Ok(Self { store, profile })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authorize(
        &self,
        authenticated_sender: &CommunicationVerifyingIdentity,
        action: Layer8Action,
        conversation_id: String,
        recipient_id: String,
        replay_id: String,
        correlation_id: String,
        now_epoch_secs: u64,
    ) -> AuthorityDecision {
        self.authorize_scoped(
            authenticated_sender,
            action,
            Some(conversation_id),
            BTreeSet::from([recipient_id]),
            None,
            replay_id,
            correlation_id,
            now_epoch_secs,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authorize_scoped(
        &self,
        authenticated_sender: &CommunicationVerifyingIdentity,
        action: Layer8Action,
        conversation_id: Option<String>,
        recipients: BTreeSet<String>,
        attachment_id: Option<String>,
        replay_id: String,
        correlation_id: String,
        now_epoch_secs: u64,
    ) -> AuthorityDecision {
        if authenticated_sender.principal_id != self.profile.evidence.principal_id
            || authenticated_sender.polis_id != self.profile.evidence.polis_id
            || authenticated_sender.signing_key_id != self.profile.evidence.signing_key_id
            || hex::decode(&self.profile.evidence.verifying_key_hex)
                .ok()
                .as_deref()
                != Some(authenticated_sender.verifying_key.as_bytes())
        {
            return AuthorityDecision::Refused(PublicRefusal {
                authorized: false,
                reason: RefusalReason::IdentityUnavailable,
                retryable: false,
                correlation_id,
            });
        }
        let request = AuthorityRequest {
            evidence: self.profile.evidence.clone(),
            action: action.clone(),
            conversation_id,
            recipients: recipients.clone(),
            attachment_id,
            replay_id,
            correlation_id: correlation_id.clone(),
            now_epoch_secs,
        };
        let principal = match request.evidence.derive_principal(now_epoch_secs) {
            Ok(principal) => principal,
            Err(_) => {
                return self.store.authorize(
                    request,
                    &self.profile.capabilities[0],
                    &self.profile.agent_policies[0],
                    &self.profile.polis_policies[0],
                );
            }
        };
        let candidate = self.profile.capabilities.iter().find_map(|capability| {
            self.profile.agent_policies.iter().find_map(|agent_policy| {
                self.profile.polis_policies.iter().find_map(|polis_policy| {
                    candidate_matches(&request, &principal, capability, agent_policy, polis_policy)
                        .then_some((capability, agent_policy, polis_policy))
                })
            })
        });
        let (capability, agent_policy, polis_policy) = candidate.unwrap_or((
            &self.profile.capabilities[0],
            &self.profile.agent_policies[0],
            &self.profile.polis_policies[0],
        ));
        self.store
            .authorize(request, capability, agent_policy, polis_policy)
    }
}

fn candidate_matches(
    request: &AuthorityRequest,
    principal: &Layer8Principal,
    capability: &Layer8Capability,
    agent_policy: &Layer8Policy,
    polis_policy: &Layer8Policy,
) -> bool {
    !capability.revoked
        && request.now_epoch_secs < capability.expires_at_epoch_secs
        && capability.epoch != 0
        && capability.epoch == agent_policy.epoch
        && capability.epoch == polis_policy.epoch
        && agent_policy.available
        && polis_policy.available
        && capability.principal_id == principal.principal_id
        && scope_matches(request, principal, &capability.scope)
        && scope_matches(request, principal, &agent_policy.scope)
        && scope_matches(request, principal, &polis_policy.scope)
}
