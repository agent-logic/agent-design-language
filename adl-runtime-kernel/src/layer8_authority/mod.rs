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
    pub prechecked_refusal: Option<RefusalReason>,
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
        let prechecked_refusal = if authenticated_sender.principal_id
            != self.profile.evidence.principal_id
            || authenticated_sender.polis_id != self.profile.evidence.polis_id
            || authenticated_sender.signing_key_id != self.profile.evidence.signing_key_id
            || authenticated_sender.credential_generation
                != self.profile.evidence.current_credential_generation
            || hex::decode(&self.profile.evidence.verifying_key_hex)
                .ok()
                .as_deref()
                != Some(authenticated_sender.verifying_key.as_bytes())
        {
            Some(RefusalReason::IdentityUnavailable)
        } else {
            authenticated_sender.ensure_valid_at(now_epoch_secs).err()
        };
        let request = AuthorityRequest {
            evidence: self.profile.evidence.clone(),
            action: action.clone(),
            conversation_id,
            recipients: recipients.clone(),
            attachment_id,
            replay_id,
            correlation_id: correlation_id.clone(),
            now_epoch_secs,
            prechecked_refusal,
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

#[allow(clippy::too_many_arguments)]
pub fn authorize_with_profile(
    store: &Layer8AuthorityStore,
    profile: &ConversationAuthorityProfile,
    authenticated_sender: &CommunicationVerifyingIdentity,
    action: Layer8Action,
    conversation_id: Option<String>,
    recipients: BTreeSet<String>,
    replay_id: String,
    correlation_id: String,
    now_epoch_secs: u64,
) -> AuthorityDecision {
    let prechecked_refusal = if authenticated_sender.principal_id != profile.evidence.principal_id
        || authenticated_sender.polis_id != profile.evidence.polis_id
        || authenticated_sender.signing_key_id != profile.evidence.signing_key_id
        || authenticated_sender.credential_generation
            != profile.evidence.current_credential_generation
        || hex::decode(&profile.evidence.verifying_key_hex)
            .ok()
            .as_deref()
            != Some(authenticated_sender.verifying_key.as_bytes())
    {
        Some(RefusalReason::IdentityUnavailable)
    } else {
        authenticated_sender.ensure_valid_at(now_epoch_secs).err()
    };
    let request = AuthorityRequest {
        evidence: profile.evidence.clone(),
        action,
        conversation_id,
        recipients,
        attachment_id: None,
        replay_id,
        correlation_id,
        now_epoch_secs,
        prechecked_refusal,
    };
    let principal = request.evidence.derive_principal(now_epoch_secs).ok();
    let candidate = principal.as_ref().and_then(|principal| {
        profile.capabilities.iter().find_map(|capability| {
            profile.agent_policies.iter().find_map(|agent_policy| {
                profile.polis_policies.iter().find_map(|polis_policy| {
                    candidate_matches(&request, principal, capability, agent_policy, polis_policy)
                        .then_some((capability, agent_policy, polis_policy))
                })
            })
        })
    });
    let Some(fallback) = profile
        .capabilities
        .first()
        .zip(profile.agent_policies.first())
        .zip(profile.polis_policies.first())
        .map(|((capability, agent), polis)| (capability, agent, polis))
    else {
        return AuthorityDecision::Refused(PublicRefusal {
            authorized: false,
            reason: RefusalReason::CapabilityDenied,
            retryable: false,
            correlation_id: request.correlation_id.clone(),
        });
    };
    let (capability, agent_policy, polis_policy) = candidate.unwrap_or(fallback);
    store.authorize(request, capability, agent_policy, polis_policy)
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

#[cfg(test)]
mod additional_tests {
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    use ed25519_dalek::{Signer, SigningKey};

    use super::*;

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos();
            let path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(".adl")
                .join("tmp")
                .join(format!(
                    "layer8-authority-{name}-{}-{nonce}",
                    std::process::id()
                ));
            std::fs::create_dir_all(&path).expect("create test root");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn key_hex(byte: u8) -> String {
        SigningKey::from_bytes(&[byte; 32])
            .verifying_key()
            .to_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }

    fn scope(action: Layer8Action, recipients: &[&str]) -> AuthorityScope {
        AuthorityScope {
            polis_id: "polis-test".to_owned(),
            action,
            conversation_id: None,
            recipients: recipients
                .iter()
                .map(|recipient| (*recipient).to_owned())
                .collect(),
            attachment_id: None,
        }
    }

    fn profile(
        evidence: RuntimeIdentityEvidence,
        scope: AuthorityScope,
    ) -> ConversationAuthorityProfile {
        ConversationAuthorityProfile {
            evidence,
            capabilities: vec![Layer8Capability {
                capability_id: "contact-shepherd".to_owned(),
                principal_id: "layer8-operator".to_owned(),
                scope: scope.clone(),
                epoch: 7,
                expires_at_epoch_secs: u64::MAX,
                revoked: false,
            }],
            agent_policies: vec![Layer8Policy {
                policy_id: "agent-contact".to_owned(),
                available: true,
                scope: scope.clone(),
                epoch: 7,
            }],
            polis_policies: vec![Layer8Policy {
                policy_id: "polis-contact".to_owned(),
                available: true,
                scope,
                epoch: 7,
            }],
        }
    }

    fn evidence() -> RuntimeIdentityEvidence {
        RuntimeIdentityEvidence {
            principal_id: "layer8-operator".to_owned(),
            polis_id: "polis-test".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            verifying_key_hex: key_hex(5),
            credential_generation: 3,
            current_credential_generation: 3,
            expires_at_epoch_secs: u64::MAX,
            revoked: false,
            authenticated: true,
        }
    }

    fn sender_identity() -> CommunicationVerifyingIdentity {
        CommunicationVerifyingIdentity {
            principal_id: "layer8-operator".to_owned(),
            polis_id: "polis-test".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            credential_generation: 3,
            verifying_key: SigningKey::from_bytes(&[5; 32]).verifying_key(),
            revoked: false,
            not_before_epoch_secs: 0,
            expires_at_epoch_secs: u64::MAX,
        }
    }

    fn test_authority(root: &Path) -> Layer8ConversationAuthority {
        Layer8ConversationAuthority::new(
            Layer8AuthorityStore::open(root.join("authority.jsonl")).expect("open authority store"),
            profile(evidence(), scope(Layer8Action::Contact, &["shepherd"])),
        )
        .expect("valid authority")
    }

    #[test]
    fn authorization_requires_identity_capability_policy_and_replay_freshness() {
        let root = TestRoot::new("authorize");
        let authority = test_authority(root.path());
        let sender = sender_identity();

        let granted = authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_owned(),
            "shepherd".to_owned(),
            "replay-1".to_owned(),
            "correlation-1".to_owned(),
            1_700_000_000,
        );
        assert!(matches!(granted, AuthorityDecision::Authorized(_)));

        let replay = authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_owned(),
            "shepherd".to_owned(),
            "replay-1".to_owned(),
            "correlation-2".to_owned(),
            1_700_000_001,
        );
        assert_refusal(replay, RefusalReason::ReplayRefused);

        let outside_scope = authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_owned(),
            "unapproved-agent".to_owned(),
            "replay-2".to_owned(),
            "correlation-3".to_owned(),
            1_700_000_002,
        );
        assert_refusal(outside_scope, RefusalReason::ScopeDenied);

        let audit_lines = std::fs::read_to_string(root.path().join("authority.jsonl"))
            .expect("audit log should be durable")
            .lines()
            .count();
        assert_eq!(audit_lines, 3);
    }

    #[test]
    fn stale_identity_and_wrong_key_fail_closed_before_grant() {
        let root = TestRoot::new("identity");
        let mut stale_evidence = evidence();
        stale_evidence.credential_generation = 2;
        let authority = Layer8ConversationAuthority::new(
            Layer8AuthorityStore::open(root.path().join("stale-authority.jsonl"))
                .expect("open authority store"),
            profile(stale_evidence, scope(Layer8Action::Contact, &["shepherd"])),
        )
        .expect("profile shape remains valid");
        assert_refusal(
            authority.authorize(
                &sender_identity(),
                Layer8Action::Contact,
                "conversation-1".to_owned(),
                "shepherd".to_owned(),
                "replay-stale".to_owned(),
                "correlation-stale".to_owned(),
                1_700_000_000,
            ),
            RefusalReason::StaleCredential,
        );

        let mut wrong_key_sender = sender_identity();
        wrong_key_sender.verifying_key = SigningKey::from_bytes(&[9; 32]).verifying_key();
        assert_refusal(
            test_authority(&root.path().join("wrong-key")).authorize(
                &wrong_key_sender,
                Layer8Action::Contact,
                "conversation-1".to_owned(),
                "shepherd".to_owned(),
                "replay-wrong-key".to_owned(),
                "correlation-wrong-key".to_owned(),
                1_700_000_000,
            ),
            RefusalReason::IdentityUnavailable,
        );

        let mut revoked_sender = sender_identity();
        revoked_sender.revoked = true;
        assert_refusal(
            test_authority(&root.path().join("revoked-sender")).authorize(
                &revoked_sender,
                Layer8Action::Contact,
                "conversation-1".to_owned(),
                "shepherd".to_owned(),
                "replay-revoked-sender".to_owned(),
                "correlation-revoked-sender".to_owned(),
                1_700_000_000,
            ),
            RefusalReason::IdentityRevoked,
        );

        let mut expired_sender = sender_identity();
        expired_sender.expires_at_epoch_secs = 1_700_000_000;
        assert_refusal(
            test_authority(&root.path().join("expired-sender")).authorize(
                &expired_sender,
                Layer8Action::Contact,
                "conversation-1".to_owned(),
                "shepherd".to_owned(),
                "replay-expired-sender".to_owned(),
                "correlation-expired-sender".to_owned(),
                1_700_000_000,
            ),
            RefusalReason::IdentityExpired,
        );
    }

    #[test]
    fn signed_exchange_binds_sender_recipient_payload_and_acknowledgement() {
        let root = TestRoot::new("exchange");
        let sender_key = root.path().join("sender.key");
        let recipient_key = root.path().join("recipient.key");
        std::fs::write(&sender_key, "05".repeat(32)).expect("write sender key");
        std::fs::write(&recipient_key, "06".repeat(32)).expect("write recipient key");

        let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
            sender: CommunicationKeyDescriptor {
                principal_id: "layer8-operator".to_owned(),
                polis_id: "polis-test".to_owned(),
                signing_key_id: "operator-key".to_owned(),
                credential_generation: 3,
                private_key_file: sender_key,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            },
            recipients: vec![CommunicationVerifyingDescriptor {
                principal_id: "shepherd".to_owned(),
                polis_id: "polis-test".to_owned(),
                signing_key_id: "shepherd-key".to_owned(),
                credential_generation: 4,
                verifying_key_hex: key_hex(6),
                revoked: false,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            }],
        })
        .expect("load exchange");

        assert_eq!(
            exchange.signed_request(
                "unknown-recipient",
                "conversation-1",
                "correlation-denied",
                "replay-denied",
                "{\"message\":\"hello\"}".to_owned(),
                1_700_000_000,
            ),
            Err(RefusalReason::IdentityUnavailable)
        );
        assert_eq!(
            exchange.signed_request(
                "shepherd",
                "conversation-1",
                "correlation-noncanonical",
                "replay-noncanonical",
                "{ \"message\": \"hello\" }".to_owned(),
                1_700_000_000,
            ),
            Err(RefusalReason::InvalidRequest)
        );

        let request = exchange
            .signed_request(
                "shepherd",
                "conversation-1",
                "correlation-1",
                "replay-1",
                "{\"message\":\"hello\"}".to_owned(),
                1_700_000_000,
            )
            .expect("sign request");
        exchange
            .verify_request(&request, 1_700_000_000)
            .expect("signed request verifies");

        let sender_key = SigningKey::from_bytes(&[5; 32]);
        let mut unknown_recipient_request = SignedIdentityMessage {
            schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
            message_kind: IdentityMessageKind::Request,
            message_id: "operator-key-manual-unknown".to_owned(),
            sender_id: "layer8-operator".to_owned(),
            recipient_id: "unknown-recipient".to_owned(),
            polis_id: "polis-test".to_owned(),
            conversation_id: "conversation-1".to_owned(),
            correlation_id: "correlation-manual-unknown".to_owned(),
            causation_id: String::new(),
            replay_id: "replay-manual-unknown".to_owned(),
            monotonic_sequence: 99,
            issued_at_epoch_secs: 1_700_000_000,
            expires_at_epoch_secs: 1_700_000_060,
            payload_json: "{\"message\":\"hello\"}".to_owned(),
            signing_key_id: "operator-key".to_owned(),
            credential_generation: 3,
            signature: String::new(),
        };
        unknown_recipient_request.signature = hex::encode(
            sender_key
                .sign(
                    &unknown_recipient_request
                        .signing_bytes()
                        .expect("signing bytes"),
                )
                .to_bytes(),
        );
        assert_eq!(
            exchange.verify_request(&unknown_recipient_request, 1_700_000_000),
            Err(RefusalReason::IdentityUnavailable)
        );

        let mut tampered = request.clone();
        tampered.payload_json = "{\"message\":\"goodbye\"}".to_owned();
        assert_eq!(
            exchange.verify_request(&tampered, 1_700_000_000),
            Err(RefusalReason::IdentityUnavailable)
        );

        let acknowledgement = sign_recipient_acknowledgement(
            &request,
            &CommunicationKeyDescriptor {
                principal_id: "shepherd".to_owned(),
                polis_id: "polis-test".to_owned(),
                signing_key_id: "shepherd-key".to_owned(),
                credential_generation: 4,
                private_key_file: recipient_key,
                not_before_epoch_secs: 0,
                expires_at_epoch_secs: u64::MAX,
            },
            "{\"status\":\"delivered\"}".to_owned(),
            1_700_000_000,
        )
        .expect("sign ack");
        exchange
            .verify_request_and_acknowledgement(&request, &acknowledgement, 1_700_000_000)
            .expect("ack verifies");
    }

    fn assert_refusal(decision: AuthorityDecision, reason: RefusalReason) {
        match decision {
            AuthorityDecision::Refused(refusal) => assert_eq!(refusal.reason, reason),
            AuthorityDecision::Authorized(grant) => {
                panic!("expected refusal {reason:?}, got grant {grant:?}")
            }
        }
    }
}

#[cfg(test)]
mod generated_tests {
    use std::fs;
    use std::path::Path;

    use ed25519_dalek::SigningKey;
    use tempfile::TempDir;

    use super::*;

    const NOW: u64 = 1_800_000_000;

    struct PrincipalFixture {
        descriptor: CommunicationKeyDescriptor,
        verifying: CommunicationVerifyingDescriptor,
        evidence: RuntimeIdentityEvidence,
    }

    fn principal(root: &Path, id: &str, secret: [u8; 32]) -> PrincipalFixture {
        let signing_key = SigningKey::from_bytes(&secret);
        let private_key_file = root.join(format!("{id}.key"));
        fs::write(&private_key_file, hex::encode(secret)).unwrap();
        let verifying_key_hex = hex::encode(signing_key.verifying_key().as_bytes());
        PrincipalFixture {
            descriptor: CommunicationKeyDescriptor {
                principal_id: id.to_string(),
                polis_id: "polis-a".to_string(),
                signing_key_id: format!("{id}-signing-key-1"),
                credential_generation: 7,
                private_key_file,
                not_before_epoch_secs: NOW - 10,
                expires_at_epoch_secs: NOW + 600,
            },
            verifying: CommunicationVerifyingDescriptor {
                principal_id: id.to_string(),
                polis_id: "polis-a".to_string(),
                signing_key_id: format!("{id}-signing-key-1"),
                credential_generation: 7,
                verifying_key_hex: verifying_key_hex.clone(),
                revoked: false,
                not_before_epoch_secs: NOW - 10,
                expires_at_epoch_secs: NOW + 600,
            },
            evidence: RuntimeIdentityEvidence {
                principal_id: id.to_string(),
                polis_id: "polis-a".to_string(),
                signing_key_id: format!("{id}-signing-key-1"),
                verifying_key_hex,
                credential_generation: 7,
                current_credential_generation: 7,
                expires_at_epoch_secs: NOW + 600,
                revoked: false,
                authenticated: true,
            },
        }
    }

    fn scope(action: Layer8Action, recipient: &str) -> AuthorityScope {
        AuthorityScope {
            polis_id: "polis-a".to_string(),
            action,
            conversation_id: Some("conversation-1".to_string()),
            recipients: BTreeSet::from([recipient.to_string()]),
            attachment_id: None,
        }
    }

    fn authority(
        root: &Path,
        evidence: RuntimeIdentityEvidence,
        action: Layer8Action,
        recipient: &str,
    ) -> Layer8ConversationAuthority {
        let scope = scope(action, recipient);
        Layer8ConversationAuthority::new(
            Layer8AuthorityStore::open(root.join("layer8-audit.jsonl")).unwrap(),
            ConversationAuthorityProfile {
                evidence: evidence.clone(),
                capabilities: vec![Layer8Capability {
                    capability_id: "capability-1".to_string(),
                    principal_id: evidence.principal_id,
                    scope: scope.clone(),
                    epoch: 3,
                    expires_at_epoch_secs: NOW + 300,
                    revoked: false,
                }],
                agent_policies: vec![Layer8Policy {
                    policy_id: "agent-policy-1".to_string(),
                    available: true,
                    scope: scope.clone(),
                    epoch: 3,
                }],
                polis_policies: vec![Layer8Policy {
                    policy_id: "polis-policy-1".to_string(),
                    available: true,
                    scope,
                    epoch: 3,
                }],
            },
        )
        .unwrap()
    }

    fn direct_exchange() -> (
        TempDir,
        PrincipalFixture,
        PrincipalFixture,
        Layer8SignedExchange,
    ) {
        let temp_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(".adl")
            .join("tmp");
        std::fs::create_dir_all(&temp_root).unwrap();
        let root = tempfile::tempdir_in(temp_root).unwrap();
        let operator = principal(root.path(), "operator", [11; 32]);
        let agent = principal(root.path(), "agent", [12; 32]);
        let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
            sender: operator.descriptor.clone(),
            recipients: vec![agent.verifying.clone()],
        })
        .unwrap();
        (root, operator, agent, exchange)
    }

    #[test]
    fn direct_agent_exchange_uses_signed_identity_message_and_signed_acknowledgement() {
        let (_root, _operator, agent, exchange) = direct_exchange();
        let request = exchange
            .signed_request(
                "agent",
                "conversation-1",
                "correlation-1",
                "replay-1",
                r#"{"intent":"contact"}"#.to_string(),
                NOW,
            )
            .unwrap();
        let acknowledgement = sign_recipient_acknowledgement(
            &request,
            &agent.descriptor,
            r#"{"accepted":true}"#.to_string(),
            NOW + 1,
        )
        .unwrap();

        exchange
            .verify_request_and_acknowledgement(&request, &acknowledgement, NOW + 2)
            .unwrap();
    }

    #[test]
    fn authority_authorizes_once_then_fails_closed_on_replay_scope_and_revocation() {
        let (root, operator, _agent, exchange) = direct_exchange();
        let conversation_authority = authority(
            root.path(),
            operator.evidence.clone(),
            Layer8Action::Contact,
            "agent",
        );
        let sender = exchange.sender_verifying_identity();

        let decision = conversation_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-2".to_string(),
            "correlation-2".to_string(),
            NOW,
        );
        assert!(matches!(decision, AuthorityDecision::Authorized(_)));

        let replay = conversation_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-2".to_string(),
            "correlation-2b".to_string(),
            NOW,
        );
        assert!(matches!(
            replay,
            AuthorityDecision::Refused(PublicRefusal {
                reason: RefusalReason::ReplayRefused,
                ..
            })
        ));

        let scope_escalation = conversation_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "different-agent".to_string(),
            "replay-3".to_string(),
            "correlation-3".to_string(),
            NOW,
        );
        assert!(matches!(
            scope_escalation,
            AuthorityDecision::Refused(PublicRefusal {
                reason: RefusalReason::ScopeDenied,
                ..
            })
        ));

        let mut revoked = operator.evidence;
        revoked.revoked = true;
        let revoked_authority = authority(root.path(), revoked, Layer8Action::Contact, "agent");
        let revoked_decision = revoked_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-4".to_string(),
            "correlation-4".to_string(),
            NOW,
        );
        assert!(matches!(
            revoked_decision,
            AuthorityDecision::Refused(PublicRefusal {
                reason: RefusalReason::IdentityRevoked,
                ..
            })
        ));
    }

    #[test]
    fn substituted_recipient_signature_and_payload_tampering_fail_closed() {
        let (root, _operator, agent, exchange) = direct_exchange();
        let attacker = principal(root.path(), "attacker", [13; 32]);
        let request = exchange
            .signed_request(
                "agent",
                "conversation-1",
                "correlation-5",
                "replay-5",
                r#"{"intent":"continue"}"#.to_string(),
                NOW,
            )
            .unwrap();
        let attacker_ack = sign_recipient_acknowledgement(
            &request,
            &attacker.descriptor,
            r#"{"accepted":true}"#.to_string(),
            NOW + 1,
        );
        assert_eq!(attacker_ack, Err(RefusalReason::IdentityUnavailable));

        let mut tampered = request.clone();
        tampered.payload_json = r#"{"intent":"attach"}"#.to_string();
        assert_eq!(
            verify_signed_identity_message(
                &tampered,
                &exchange.sender_verifying_identity(),
                "agent",
                NOW + 1,
            ),
            Err(RefusalReason::IdentityUnavailable)
        );

        let acknowledgement = sign_recipient_acknowledgement(
            &request,
            &agent.descriptor,
            r#"{"accepted":true}"#.to_string(),
            NOW + 1,
        )
        .unwrap();
        let wrong_recipient = principal(root.path(), "wrong-agent", [14; 32]);
        assert_eq!(
            verify_recipient_acknowledgement(
                &request,
                &acknowledgement,
                &CommunicationVerifyingIdentity {
                    principal_id: wrong_recipient.verifying.principal_id,
                    polis_id: wrong_recipient.verifying.polis_id,
                    signing_key_id: wrong_recipient.verifying.signing_key_id,
                    credential_generation: wrong_recipient.verifying.credential_generation,
                    verifying_key: SigningKey::from_bytes(&[14; 32]).verifying_key(),
                    revoked: false,
                    not_before_epoch_secs: NOW - 10,
                    expires_at_epoch_secs: NOW + 600,
                },
                NOW + 2,
            ),
            Err(RefusalReason::IdentityUnavailable)
        );
    }
}
