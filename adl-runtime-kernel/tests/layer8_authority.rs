use adl_runtime_kernel::layer8_authority::{
    sign_recipient_acknowledgement, AuthorityDecision, AuthorityScope, CommunicationKeyDescriptor,
    CommunicationVerifyingDescriptor, ConversationAuthorityProfile, ConversationSigningProfile,
    IdentityMessageKind, Layer8Action, Layer8AuthorityStore, Layer8Capability,
    Layer8ConversationAuthority, Layer8Policy, Layer8SignedExchange, PublicRefusal, RefusalReason,
    RuntimeIdentityEvidence, SignedIdentityMessage, ACIP_IDENTITY_MESSAGE_SCHEMA,
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
