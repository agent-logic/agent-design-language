use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use adl::csm_runtime_api::{
    authorize_layer8_runtime_delivery, Layer8RuntimeDelivery, Layer8RuntimeDeliveryRequest,
};
use adl_runtime::layer8_authority::{
    sign_recipient_acknowledgement, AuthorityScope, CommunicationKeyDescriptor,
    CommunicationVerifyingDescriptor, ConversationAuthorityProfile, ConversationSigningProfile,
    Layer8Action, Layer8AuthorityStore, Layer8Capability, Layer8ConversationAuthority,
    Layer8Policy, Layer8SignedExchange, RefusalReason, RuntimeIdentityEvidence,
};

struct TestRoot(PathBuf);

impl TestRoot {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-layer8-runtime-api-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();
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

fn authority(root: &Path) -> Layer8ConversationAuthority {
    let scope = AuthorityScope {
        polis_id: "polis-test".to_owned(),
        action: Layer8Action::Contact,
        conversation_id: None,
        recipients: BTreeSet::from(["shepherd".to_owned()]),
        attachment_id: None,
    };
    Layer8ConversationAuthority::new(
        Layer8AuthorityStore::open(root.join("authority.jsonl")).unwrap(),
        ConversationAuthorityProfile {
            evidence: RuntimeIdentityEvidence {
                principal_id: "layer8-operator".to_owned(),
                polis_id: "polis-test".to_owned(),
                credential_generation: 3,
                current_credential_generation: 3,
                expires_at_epoch_secs: u64::MAX,
                revoked: false,
                authenticated: true,
            },
            capabilities: vec![Layer8Capability {
                capability_id: "contact".to_owned(),
                principal_id: "layer8-operator".to_owned(),
                scope: scope.clone(),
                epoch: 7,
                expires_at_epoch_secs: u64::MAX,
                revoked: false,
            }],
            agent_policies: vec![Layer8Policy {
                policy_id: "agent".to_owned(),
                available: true,
                scope: scope.clone(),
                epoch: 7,
            }],
            polis_policies: vec![Layer8Policy {
                policy_id: "polis".to_owned(),
                available: true,
                scope,
                epoch: 7,
            }],
        },
    )
    .unwrap()
}

fn exchange(root: &Path) -> (Layer8SignedExchange, CommunicationKeyDescriptor) {
    let sender = root.join("sender.key");
    let recipient = root.join("recipient.key");
    std::fs::write(&sender, "05".repeat(32)).unwrap();
    std::fs::write(&recipient, "06".repeat(32)).unwrap();
    let descriptor =
        |principal_id: &str, signing_key_id: &str, private_key_file| CommunicationKeyDescriptor {
            principal_id: principal_id.to_owned(),
            polis_id: "polis-test".to_owned(),
            signing_key_id: signing_key_id.to_owned(),
            private_key_file,
            not_before_epoch_secs: 0,
            expires_at_epoch_secs: u64::MAX,
        };
    let recipient_descriptor = descriptor("shepherd", "shepherd-key", recipient.clone());
    let exchange = Layer8SignedExchange::load(ConversationSigningProfile {
        sender: descriptor("layer8-operator", "operator-key", sender),
        recipients: vec![CommunicationVerifyingDescriptor {
            principal_id: "shepherd".to_owned(),
            polis_id: "polis-test".to_owned(),
            signing_key_id: "shepherd-key".to_owned(),
            verifying_key_hex: ed25519_dalek::SigningKey::from_bytes(&[6_u8; 32])
                .verifying_key()
                .to_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
            revoked: false,
            not_before_epoch_secs: 0,
            expires_at_epoch_secs: u64::MAX,
        }],
    })
    .unwrap();
    (exchange, recipient_descriptor)
}

fn request(
    exchange: &Layer8SignedExchange,
    recipient_id: &str,
    replay_id: &str,
) -> Layer8RuntimeDeliveryRequest {
    let signed_request = exchange
        .signed_request(
            recipient_id,
            "conversation-1",
            "correlation-1",
            replay_id,
            "{\"action\":\"contact\",\"message\":\"hello\"}".to_owned(),
            1_700_000_000,
        )
        .unwrap();
    Layer8RuntimeDeliveryRequest {
        action: Layer8Action::Contact,
        conversation_id: "conversation-1".to_owned(),
        recipient_id: recipient_id.to_owned(),
        replay_id: replay_id.to_owned(),
        correlation_id: "correlation-1".to_owned(),
        now_epoch_secs: 1_700_000_000,
        signed_request,
        sender_identity: exchange.sender_verifying_identity(),
    }
}

#[test]
fn runtime_api_delivers_only_after_authority_grants() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let (exchange, recipient_signing) = exchange(root.path());
    let deliveries = AtomicUsize::new(0);

    let delivery_request = request(&exchange, "shepherd", "request-1");
    let signed_request = delivery_request.signed_request.clone();
    let delivered =
        authorize_layer8_runtime_delivery(&authority, delivery_request, || Layer8RuntimeDelivery {
            value: deliveries.fetch_add(1, Ordering::SeqCst),
            acknowledgement: sign_recipient_acknowledgement(
                &signed_request,
                &recipient_signing,
                "{\"status\":\"delivered\"}".to_owned(),
                1_700_000_000,
            )
            .unwrap(),
            recipient_identity: exchange.recipient_verifying_identity("shepherd").unwrap(),
        })
        .unwrap();

    assert_eq!(delivered, 0);
    assert_eq!(deliveries.load(Ordering::SeqCst), 1);
}

#[test]
fn runtime_api_refusal_cannot_invoke_delivery() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let (exchange, _) = exchange(root.path());
    let deliveries = AtomicUsize::new(0);

    let refusal = authorize_layer8_runtime_delivery(
        &authority,
        request(&exchange, "agent-not-authorized", "request-2"),
        || -> Layer8RuntimeDelivery<()> { unreachable!("refused delivery must not run") },
    )
    .unwrap_err();

    assert_eq!(refusal.reason, RefusalReason::ScopeDenied);
    assert_eq!(deliveries.load(Ordering::SeqCst), 0);
}

#[test]
fn runtime_api_refuses_delivery_without_exact_recipient_acknowledgement() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let (exchange, recipient_signing) = exchange(root.path());
    let delivery_request = request(&exchange, "shepherd", "request-3");
    let signed_request = delivery_request.signed_request.clone();

    let refusal = authorize_layer8_runtime_delivery(
        &authority,
        delivery_request,
        || -> Layer8RuntimeDelivery<()> {
            let mut acknowledgement = sign_recipient_acknowledgement(
                &signed_request,
                &recipient_signing,
                "{\"status\":\"delivered\"}".to_owned(),
                1_700_000_000,
            )
            .unwrap();
            acknowledgement.causation_id = "different-request".to_owned();
            Layer8RuntimeDelivery {
                value: (),
                acknowledgement,
                recipient_identity: exchange.recipient_verifying_identity("shepherd").unwrap(),
            }
        },
    )
    .unwrap_err();

    assert_eq!(refusal.reason, RefusalReason::InvalidRequest);
}

#[test]
fn runtime_api_refuses_outer_fields_not_bound_to_signed_request() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let (exchange, _) = exchange(root.path());
    let mut delivery_request = request(&exchange, "shepherd", "request-4");
    delivery_request.conversation_id = "substituted-conversation".to_owned();

    let refusal = authorize_layer8_runtime_delivery(
        &authority,
        delivery_request,
        || -> Layer8RuntimeDelivery<()> { unreachable!("mismatched signed delivery must not run") },
    )
    .unwrap_err();

    assert_eq!(refusal.reason, RefusalReason::InvalidRequest);
}
