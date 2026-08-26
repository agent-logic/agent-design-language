use adl_runtime_kernel::layer8_authority::{
    sign_recipient_acknowledgement, AuthorityDecision, AuthorityScope, CommunicationKeyDescriptor,
    CommunicationVerifyingDescriptor, CommunicationVerifyingIdentity, ConversationAuthorityProfile,
    ConversationSigningProfile, IdentityMessageKind, Layer8Action, Layer8AuthorityStore,
    Layer8Capability, Layer8ConversationAuthority, Layer8Policy, Layer8SignedExchange,
    PublicRefusal, RefusalReason, RuntimeIdentityEvidence, SignedIdentityMessage,
    ACIP_IDENTITY_MESSAGE_SCHEMA,
};
use ed25519_dalek::{Signer, SigningKey};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const NOW: u64 = 1_800_000_000;

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
        std::fs::create_dir_all(&path).expect("create repo-local test root");
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

fn key(root: &std::path::Path, id: &str, secret: [u8; 32]) -> CommunicationKeyDescriptor {
    let private_key_file = root.join(format!("{id}.key"));
    std::fs::write(&private_key_file, hex::encode(secret)).unwrap();
    CommunicationKeyDescriptor {
        principal_id: id.to_string(),
        polis_id: "polis-a".to_string(),
        signing_key_id: format!("{id}-key"),
        credential_generation: 1,
        private_key_file,
        not_before_epoch_secs: NOW - 1,
        expires_at_epoch_secs: NOW + 60,
    }
}

fn verifying(id: &str, secret: [u8; 32]) -> CommunicationVerifyingDescriptor {
    CommunicationVerifyingDescriptor {
        principal_id: id.to_string(),
        polis_id: "polis-a".to_string(),
        signing_key_id: format!("{id}-key"),
        credential_generation: 1,
        verifying_key_hex: hex::encode(SigningKey::from_bytes(&secret).verifying_key().as_bytes()),
        revoked: false,
        not_before_epoch_secs: NOW - 1,
        expires_at_epoch_secs: NOW + 60,
    }
}

fn signed_request_for(recipient_id: &str, signing_key: &SigningKey) -> SignedIdentityMessage {
    signed_request_for_sender(recipient_id, "operator", "operator-key", signing_key)
}

fn signed_request_for_sender(
    recipient_id: &str,
    sender_id: &str,
    signing_key_id: &str,
    signing_key: &SigningKey,
) -> SignedIdentityMessage {
    let mut message = SignedIdentityMessage {
        schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_string(),
        message_kind: IdentityMessageKind::Request,
        message_id: "manual-message-1".to_string(),
        sender_id: sender_id.to_string(),
        recipient_id: recipient_id.to_string(),
        polis_id: "polis-a".to_string(),
        conversation_id: "conversation-1".to_string(),
        correlation_id: "manual-correlation".to_string(),
        causation_id: String::new(),
        replay_id: "manual-replay".to_string(),
        monotonic_sequence: 1,
        issued_at_epoch_secs: NOW,
        expires_at_epoch_secs: NOW + 60,
        payload_json: r#"{"intent":"contact"}"#.to_string(),
        signing_key_id: signing_key_id.to_string(),
        credential_generation: 1,
        signature: String::new(),
    };
    message.signature = hex::encode(
        signing_key
            .sign(&message.signing_bytes().unwrap())
            .to_bytes(),
    );
    message
}

fn audit_lines(path: &Path) -> usize {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .count()
}

fn identity_from_descriptor(
    descriptor: &CommunicationVerifyingDescriptor,
) -> CommunicationVerifyingIdentity {
    let bytes = hex::decode(&descriptor.verifying_key_hex).unwrap();
    let bytes: [u8; 32] = bytes.try_into().unwrap();
    CommunicationVerifyingIdentity {
        principal_id: descriptor.principal_id.clone(),
        polis_id: descriptor.polis_id.clone(),
        signing_key_id: descriptor.signing_key_id.clone(),
        credential_generation: descriptor.credential_generation,
        verifying_key: ed25519_dalek::VerifyingKey::from_bytes(&bytes).unwrap(),
        revoked: descriptor.revoked,
        not_before_epoch_secs: descriptor.not_before_epoch_secs,
        expires_at_epoch_secs: descriptor.expires_at_epoch_secs,
    }
}

fn core_authority_with(
    root: &Path,
    update_capability: impl FnOnce(&mut Layer8Capability),
    update_agent_policy: impl FnOnce(&mut Layer8Policy),
    update_polis_policy: impl FnOnce(&mut Layer8Policy),
) -> (Layer8ConversationAuthority, CommunicationVerifyingIdentity) {
    let sender_descriptor = verifying("operator", [21; 32]);
    let sender_identity = {
        let descriptor = sender_descriptor.clone();
        RuntimeIdentityEvidence {
            principal_id: descriptor.principal_id,
            polis_id: descriptor.polis_id,
            signing_key_id: descriptor.signing_key_id,
            verifying_key_hex: descriptor.verifying_key_hex,
            credential_generation: descriptor.credential_generation,
            current_credential_generation: descriptor.credential_generation,
            expires_at_epoch_secs: NOW + 60,
            revoked: false,
            authenticated: true,
        }
    };
    let scope = AuthorityScope {
        polis_id: "polis-a".to_string(),
        action: Layer8Action::Contact,
        conversation_id: Some("conversation-1".to_string()),
        recipients: BTreeSet::from(["agent".to_string()]),
        attachment_id: None,
    };
    let mut capability = Layer8Capability {
        capability_id: "capability-1".to_string(),
        principal_id: sender_identity.principal_id.clone(),
        scope: scope.clone(),
        epoch: 1,
        expires_at_epoch_secs: NOW + 60,
        revoked: false,
    };
    let mut agent_policy = Layer8Policy {
        policy_id: "agent-policy-1".to_string(),
        available: true,
        scope: scope.clone(),
        epoch: 1,
    };
    let mut polis_policy = Layer8Policy {
        policy_id: "polis-policy-1".to_string(),
        available: true,
        scope,
        epoch: 1,
    };
    update_capability(&mut capability);
    update_agent_policy(&mut agent_policy);
    update_polis_policy(&mut polis_policy);
    let authority = Layer8ConversationAuthority::new(
        Layer8AuthorityStore::open(root.join("audit.jsonl")).unwrap(),
        ConversationAuthorityProfile {
            evidence: sender_identity.clone(),
            capabilities: vec![capability],
            agent_policies: vec![agent_policy],
            polis_policies: vec![polis_policy],
        },
    )
    .unwrap();
    (authority, identity_from_descriptor(&sender_descriptor))
}

fn core_authority(root: &Path) -> (Layer8ConversationAuthority, CommunicationVerifyingIdentity) {
    core_authority_with(root, |_| {}, |_| {}, |_| {})
}

fn assert_empty_recipients_refused_for_action(
    name: &str,
    action: Layer8Action,
    attachment_id: Option<&str>,
) {
    let root = TestRoot::new(name);
    let audit_path = root.path().join("audit.jsonl");
    let attachment = attachment_id.map(str::to_string);
    let (authority, sender) = core_authority_with(
        root.path(),
        |capability| {
            capability.scope.action = action.clone();
            capability.scope.attachment_id = attachment.clone();
        },
        |agent_policy| {
            agent_policy.scope.action = action.clone();
            agent_policy.scope.attachment_id = attachment.clone();
        },
        |polis_policy| {
            polis_policy.scope.action = action.clone();
            polis_policy.scope.attachment_id = attachment.clone();
        },
    );

    assert!(matches!(
        authority.authorize_scoped(
            &sender,
            action,
            Some("conversation-1".to_string()),
            BTreeSet::new(),
            attachment,
            format!("replay-empty-{name}"),
            format!("correlation-empty-{name}"),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::ScopeDenied,
            ..
        })
    ));
    assert_eq!(audit_lines(&audit_path), 1);
}

#[test]
fn layer8_authority_core_rejects_empty_recipient_non_address_scopes() {
    assert_empty_recipients_refused_for_action(
        "empty-recipient-discover",
        Layer8Action::Discover,
        None,
    );
    assert_empty_recipients_refused_for_action(
        "empty-recipient-contact",
        Layer8Action::Contact,
        None,
    );
    assert_empty_recipients_refused_for_action(
        "empty-recipient-continue",
        Layer8Action::Continue,
        None,
    );
    assert_empty_recipients_refused_for_action(
        "empty-recipient-attach",
        Layer8Action::Attach,
        Some("attachment-1"),
    );
}

#[test]
fn layer8_authority_core_rejects_replay_and_requires_known_recipient() {
    let root = TestRoot::new("integration");
    let operator_key = key(root.path(), "operator", [21; 32]);
    let agent_key = key(root.path(), "agent", [22; 32]);
    let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
        sender: operator_key,
        recipients: vec![verifying("agent", [22; 32])],
    })
    .unwrap();
    assert_eq!(
        exchange.signed_request(
            "unknown",
            "conversation-1",
            "correlation-unknown",
            "replay-unknown",
            r#"{"intent":"contact"}"#.to_string(),
            NOW
        ),
        Err(RefusalReason::IdentityUnavailable)
    );
    assert_eq!(
        exchange.verify_request(
            &signed_request_for("unknown", &SigningKey::from_bytes(&[21; 32])),
            NOW,
        ),
        Err(RefusalReason::IdentityUnavailable)
    );

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
    let mut expired_agent_key = agent_key.clone();
    expired_agent_key.expires_at_epoch_secs = NOW;
    assert_eq!(
        sign_recipient_acknowledgement(
            &request,
            &expired_agent_key,
            r#"{"accepted":true}"#.to_string(),
            NOW,
        ),
        Err(RefusalReason::IdentityExpired)
    );
    let acknowledgement = sign_recipient_acknowledgement(
        &request,
        &agent_key,
        r#"{"accepted":true}"#.to_string(),
        NOW,
    )
    .unwrap();
    exchange
        .verify_request_and_acknowledgement(&request, &acknowledgement, NOW)
        .unwrap();

    let mut stale_sender_generation = exchange.sender_verifying_identity();
    stale_sender_generation.credential_generation = 2;
    assert_eq!(
        adl_runtime_kernel::layer8_authority::verify_signed_identity_message(
            &request,
            &stale_sender_generation,
            "agent",
            NOW,
        ),
        Err(RefusalReason::IdentityUnavailable)
    );

    let mut cross_polis_agent = verifying("agent-cross-polis", [23; 32]);
    cross_polis_agent.polis_id = "polis-b".to_string();
    let cross_polis_exchange = Layer8SignedExchange::load(ConversationSigningProfile {
        sender: key(root.path(), "operator-cross-polis", [24; 32]),
        recipients: vec![cross_polis_agent],
    })
    .unwrap();
    assert_eq!(
        cross_polis_exchange.signed_request(
            "agent-cross-polis",
            "conversation-cross-polis",
            "correlation-cross-polis",
            "replay-cross-polis",
            r#"{"intent":"contact"}"#.to_string(),
            NOW,
        ),
        Err(RefusalReason::InvalidRequest)
    );
    assert_eq!(
        cross_polis_exchange.verify_request(
            &signed_request_for_sender(
                "agent-cross-polis",
                "operator-cross-polis",
                "operator-cross-polis-key",
                &SigningKey::from_bytes(&[24; 32])
            ),
            NOW,
        ),
        Err(RefusalReason::InvalidRequest)
    );

    let sender = exchange.sender_verifying_identity();
    let evidence = RuntimeIdentityEvidence {
        principal_id: "operator".to_string(),
        polis_id: "polis-a".to_string(),
        signing_key_id: "operator-key".to_string(),
        verifying_key_hex: hex::encode(sender.verifying_key.as_bytes()),
        credential_generation: 1,
        current_credential_generation: 1,
        expires_at_epoch_secs: NOW + 60,
        revoked: false,
        authenticated: true,
    };
    let scope = AuthorityScope {
        polis_id: "polis-a".to_string(),
        action: Layer8Action::Contact,
        conversation_id: Some("conversation-1".to_string()),
        recipients: BTreeSet::from(["agent".to_string()]),
        attachment_id: None,
    };
    let authority = Layer8ConversationAuthority::new(
        Layer8AuthorityStore::open(root.path().join("audit.jsonl")).unwrap(),
        ConversationAuthorityProfile {
            evidence: evidence.clone(),
            capabilities: vec![Layer8Capability {
                capability_id: "capability-1".to_string(),
                principal_id: evidence.principal_id,
                scope: scope.clone(),
                epoch: 1,
                expires_at_epoch_secs: NOW + 60,
                revoked: false,
            }],
            agent_policies: vec![Layer8Policy {
                policy_id: "agent-policy-1".to_string(),
                available: true,
                scope: scope.clone(),
                epoch: 1,
            }],
            polis_policies: vec![Layer8Policy {
                policy_id: "polis-policy-1".to_string(),
                available: true,
                scope,
                epoch: 1,
            }],
        },
    )
    .unwrap();

    assert!(matches!(
        authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-2".to_string(),
            "correlation-2".to_string(),
            NOW,
        ),
        AuthorityDecision::Authorized(_)
    ));
    let mut revoked_sender = sender.clone();
    revoked_sender.revoked = true;
    assert!(matches!(
        authority.authorize(
            &revoked_sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-revoked-sender".to_string(),
            "correlation-revoked-sender".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::IdentityRevoked,
            ..
        })
    ));
    assert!(matches!(
        authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-2".to_string(),
            "correlation-3".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::ReplayRefused,
            ..
        })
    ));
    let secret_correlation = "SECRET-OPERATOR-CONTENT-MUST-NOT-ECHO".to_string();
    let refused = authority.authorize(
        &sender,
        Layer8Action::Contact,
        "conversation-1".to_string(),
        "unapproved-agent".to_string(),
        "replay-public-refusal".to_string(),
        secret_correlation.clone(),
        NOW,
    );
    match refused {
        AuthorityDecision::Refused(PublicRefusal { correlation_id, .. }) => {
            assert_ne!(correlation_id, secret_correlation);
            assert_eq!(correlation_id.len(), 64);
        }
        AuthorityDecision::Authorized(grant) => {
            panic!("expected public refusal with redacted correlation id, got {grant:?}");
        }
    }
}

#[test]
fn layer8_identity_precheck_refusals_are_hash_chain_audited() {
    let root = TestRoot::new("identity-precheck-audit");
    let audit_path = root.path().join("audit.jsonl");
    let (authority, sender) = core_authority(root.path());

    let mut stale_generation = sender.clone();
    stale_generation.credential_generation = 2;
    assert!(matches!(
        authority.authorize(
            &stale_generation,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-stale-generation".to_string(),
            "SECRET-stale-generation".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::IdentityUnavailable,
            ..
        })
    ));
    assert_eq!(audit_lines(&audit_path), 1);
    assert!(!std::fs::read_to_string(&audit_path)
        .unwrap()
        .contains("SECRET-stale-generation"));

    let mut revoked = sender.clone();
    revoked.revoked = true;
    assert!(matches!(
        authority.authorize(
            &revoked,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-revoked".to_string(),
            "correlation-revoked".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::IdentityRevoked,
            ..
        })
    ));
    assert_eq!(audit_lines(&audit_path), 2);

    let mut expired = sender;
    expired.expires_at_epoch_secs = NOW;
    assert!(matches!(
        authority.authorize(
            &expired,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-expired".to_string(),
            "correlation-expired".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::IdentityExpired,
            ..
        })
    ));
    assert_eq!(audit_lines(&audit_path), 3);
}

#[test]
fn layer8_capability_policy_and_corrupt_audit_fail_closed() {
    let revoked_root = TestRoot::new("capability-revoked");
    let (revoked_authority, sender) = core_authority_with(
        revoked_root.path(),
        |capability| capability.revoked = true,
        |_| {},
        |_| {},
    );
    assert!(matches!(
        revoked_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-revoked-capability".to_string(),
            "correlation-revoked-capability".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::CapabilityRevoked,
            ..
        })
    ));
    assert_eq!(audit_lines(&revoked_root.path().join("audit.jsonl")), 1);

    let stale_root = TestRoot::new("capability-stale");
    let (stale_authority, sender) = core_authority_with(
        stale_root.path(),
        |capability| capability.epoch = 0,
        |_| {},
        |_| {},
    );
    assert!(matches!(
        stale_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-stale-capability".to_string(),
            "correlation-stale-capability".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::StaleCapability,
            ..
        })
    ));
    assert_eq!(audit_lines(&stale_root.path().join("audit.jsonl")), 1);

    let policy_root = TestRoot::new("policy-unavailable");
    let (policy_authority, sender) = core_authority_with(
        policy_root.path(),
        |_| {},
        |policy| policy.available = false,
        |_| {},
    );
    assert!(matches!(
        policy_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-policy-unavailable".to_string(),
            "correlation-policy-unavailable".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::PolicyUnavailable,
            ..
        })
    ));
    assert_eq!(audit_lines(&policy_root.path().join("audit.jsonl")), 1);

    let replay_root = TestRoot::new("refused-replay");
    let replay_audit_path = replay_root.path().join("audit.jsonl");
    let (first_authority, sender) = core_authority(replay_root.path());
    assert!(matches!(
        first_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "unapproved-agent".to_string(),
            "replay-refused-then-valid".to_string(),
            "correlation-refused-then-valid".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::ScopeDenied,
            ..
        })
    ));
    assert_eq!(audit_lines(&replay_audit_path), 1);
    let (restarted_authority, sender) = core_authority(replay_root.path());
    assert!(matches!(
        restarted_authority.authorize(
            &sender,
            Layer8Action::Contact,
            "conversation-1".to_string(),
            "agent".to_string(),
            "replay-refused-then-valid".to_string(),
            "correlation-valid-after-refused-replay".to_string(),
            NOW,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::ReplayRefused,
            ..
        })
    ));
    assert_eq!(audit_lines(&replay_audit_path), 2);

    let corrupt_root = TestRoot::new("corrupt-audit");
    let audit_path = corrupt_root.path().join("audit.jsonl");
    std::fs::write(&audit_path, b"{not-json}\n").unwrap();
    assert!(matches!(
        Layer8AuthorityStore::open(&audit_path),
        Err(RefusalReason::AuditUnavailable)
    ));
}

#[test]
fn layer8_signed_exchange_rejects_tamper_noncanonical_payloads_and_ack_substitution() {
    let root = TestRoot::new("signed-exchange-tamper");
    let operator_key = key(root.path(), "operator", [21; 32]);
    let agent_key = key(root.path(), "agent", [22; 32]);
    let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
        sender: operator_key,
        recipients: vec![verifying("agent", [22; 32])],
    })
    .unwrap();

    assert_eq!(
        exchange.signed_request(
            "agent",
            "conversation-1",
            "correlation-noncanonical",
            "replay-noncanonical",
            r#"{"z":1,"a":2}"#.to_string(),
            NOW,
        ),
        Err(RefusalReason::InvalidRequest)
    );

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
    let mut tampered_request = request.clone();
    tampered_request.recipient_id = "other-agent".to_string();
    assert_eq!(
        exchange.verify_request(&tampered_request, NOW),
        Err(RefusalReason::IdentityUnavailable)
    );

    let mut tampered_payload = request.clone();
    tampered_payload.payload_json = r#"{"intent":"other"}"#.to_string();
    assert_eq!(
        exchange.verify_request(&tampered_payload, NOW),
        Err(RefusalReason::IdentityUnavailable)
    );

    let acknowledgement = sign_recipient_acknowledgement(
        &request,
        &agent_key,
        r#"{"accepted":true}"#.to_string(),
        NOW,
    )
    .unwrap();
    let mut substituted_ack = acknowledgement.clone();
    substituted_ack.causation_id = "other-message".to_string();
    assert_eq!(
        exchange.verify_request_and_acknowledgement(&request, &substituted_ack, NOW),
        Err(RefusalReason::InvalidRequest)
    );
}
