use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;

use adl_runtime_kernel::layer8_authority::*;
use ed25519_dalek::{Signer, SigningKey};
use tempfile::TempDir;

fn recipients(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

type RequestMutation = Box<dyn Fn(&mut AuthorityRequest)>;
type AuthorityMutation =
    Box<dyn Fn(&mut AuthorityRequest, &mut Layer8Capability, &mut Layer8Policy)>;

fn fixture(
    action: Layer8Action,
) -> (
    AuthorityRequest,
    Layer8Capability,
    Layer8Policy,
    Layer8Policy,
) {
    let conversation = match action {
        Layer8Action::Discover | Layer8Action::Contact => None,
        _ => Some("conversation-7".into()),
    };
    let recipient_set = if action == Layer8Action::AddressRecipients {
        recipients(&["agent-a", "agent-b"])
    } else {
        BTreeSet::new()
    };
    let attachment = (action == Layer8Action::Attach).then(|| "attachment-3".into());
    let scope = AuthorityScope {
        polis_id: "polis-1".into(),
        action: action.clone(),
        conversation_id: conversation.clone(),
        recipients: recipient_set.clone(),
        attachment_id: attachment.clone(),
    };
    (
        AuthorityRequest {
            evidence: RuntimeIdentityEvidence {
                principal_id: "human-1".into(),
                polis_id: "polis-1".into(),
                signing_key_id: "human-1-key".into(),
                verifying_key_hex: hex::encode(
                    SigningKey::from_bytes(&[98_u8; 32])
                        .verifying_key()
                        .to_bytes(),
                ),
                credential_generation: 4,
                current_credential_generation: 4,
                expires_at_epoch_secs: 200,
                revoked: false,
                authenticated: true,
            },
            action,
            conversation_id: conversation,
            recipients: recipient_set,
            attachment_id: attachment,
            replay_id: "replay-1".into(),
            correlation_id: "correlation-1".into(),
            now_epoch_secs: 100,
        },
        Layer8Capability {
            capability_id: "cap-1".into(),
            principal_id: "human-1".into(),
            scope: scope.clone(),
            epoch: 9,
            expires_at_epoch_secs: 200,
            revoked: false,
        },
        Layer8Policy {
            policy_id: "agent-policy".into(),
            available: true,
            scope: scope.clone(),
            epoch: 9,
        },
        Layer8Policy {
            policy_id: "polis-policy".into(),
            available: true,
            scope,
            epoch: 9,
        },
    )
}

fn store(temp: &TempDir) -> Layer8AuthorityStore {
    Layer8AuthorityStore::open(temp.path().join("audit.jsonl")).unwrap()
}

fn signed_message(
    sender: &str,
    recipient: &str,
    kind: IdentityMessageKind,
    causation_id: &str,
    sequence: u64,
    key_id: &str,
    key: &SigningKey,
) -> SignedIdentityMessage {
    let mut message = SignedIdentityMessage {
        schema: ACIP_IDENTITY_MESSAGE_SCHEMA.to_owned(),
        message_kind: kind,
        message_id: format!("message-{sender}-{sequence}"),
        sender_id: sender.to_owned(),
        recipient_id: recipient.to_owned(),
        polis_id: "polis-1".to_owned(),
        conversation_id: "conversation-7".to_owned(),
        correlation_id: "correlation-1".to_owned(),
        causation_id: causation_id.to_owned(),
        replay_id: format!("{sender}:{sequence}"),
        monotonic_sequence: sequence,
        issued_at_epoch_secs: 100,
        expires_at_epoch_secs: 200,
        payload_json: r#"{"text":"hello"}"#.to_owned(),
        signing_key_id: key_id.to_owned(),
        signature: String::new(),
    };
    message.signature = hex::encode(key.sign(&message.signing_bytes().unwrap()).to_bytes());
    message
}

fn identity(principal: &str, key_id: &str, key: &SigningKey) -> CommunicationVerifyingIdentity {
    CommunicationVerifyingIdentity {
        principal_id: principal.to_owned(),
        polis_id: "polis-1".to_owned(),
        signing_key_id: key_id.to_owned(),
        verifying_key: key.verifying_key(),
        revoked: false,
        not_before_epoch_secs: 50,
        expires_at_epoch_secs: 250,
    }
}

#[test]
fn human_and_agent_senders_share_one_signed_identity_message_contract() {
    for (seed, sender, key_id) in [
        ([41_u8; 32], "layer8-operator", "operator-communication-v1"),
        ([42_u8; 32], "agent-a", "agent-a-communication-v1"),
    ] {
        let key = SigningKey::from_bytes(&seed);
        let message = signed_message(
            sender,
            "agent-b",
            IdentityMessageKind::Request,
            "operator-intent-1",
            1,
            key_id,
            &key,
        );
        verify_signed_identity_message(&message, &identity(sender, key_id, &key), "agent-b", 120)
            .unwrap();
    }
}

#[test]
fn recipient_signed_acknowledgement_is_bound_to_triggering_message() {
    let sender_key = SigningKey::from_bytes(&[43_u8; 32]);
    let recipient_key = SigningKey::from_bytes(&[44_u8; 32]);
    let request = signed_message(
        "agent-a",
        "agent-b",
        IdentityMessageKind::Request,
        "turn-1",
        1,
        "agent-a-communication-v1",
        &sender_key,
    );
    verify_signed_identity_message(
        &request,
        &identity("agent-a", "agent-a-communication-v1", &sender_key),
        "agent-b",
        120,
    )
    .unwrap();
    let mut acknowledgement = signed_message(
        "agent-b",
        "agent-a",
        IdentityMessageKind::Acknowledgement,
        &request.message_id,
        1,
        "agent-b-communication-v1",
        &recipient_key,
    );
    acknowledgement.replay_id = request.replay_id.clone();
    acknowledgement.signature = hex::encode(
        recipient_key
            .sign(&acknowledgement.signing_bytes().unwrap())
            .to_bytes(),
    );
    verify_recipient_acknowledgement(
        &request,
        &acknowledgement,
        &identity("agent-b", "agent-b-communication-v1", &recipient_key),
        120,
    )
    .unwrap();
    assert_eq!(acknowledgement.causation_id, request.message_id);
    assert_eq!(acknowledgement.correlation_id, request.correlation_id);
    assert_eq!(acknowledgement.replay_id, request.replay_id);
}

#[test]
fn signature_substitution_rotation_revocation_and_expiry_fail_closed() {
    let key = SigningKey::from_bytes(&[45_u8; 32]);
    let wrong_key = SigningKey::from_bytes(&[46_u8; 32]);
    let message = signed_message(
        "agent-a",
        "agent-b",
        IdentityMessageKind::Request,
        "turn-1",
        1,
        "agent-a-communication-v1",
        &key,
    );
    assert_eq!(
        verify_signed_identity_message(
            &message,
            &identity("agent-a", "agent-a-communication-v1", &wrong_key),
            "agent-b",
            120,
        ),
        Err(RefusalReason::IdentityUnavailable)
    );
    let mut rotated = identity("agent-a", "agent-a-communication-v2", &key);
    assert_eq!(
        verify_signed_identity_message(&message, &rotated, "agent-b", 120),
        Err(RefusalReason::IdentityUnavailable)
    );
    rotated.signing_key_id = "agent-a-communication-v1".to_owned();
    rotated.revoked = true;
    assert_eq!(
        verify_signed_identity_message(&message, &rotated, "agent-b", 120),
        Err(RefusalReason::IdentityRevoked)
    );
    rotated.revoked = false;
    assert_eq!(
        verify_signed_identity_message(&message, &rotated, "agent-b", 200),
        Err(RefusalReason::IdentityExpired)
    );
}

#[test]
fn authorizes_each_action_only_with_its_exact_capability_and_policy_intersection() {
    for (index, action) in [
        Layer8Action::Discover,
        Layer8Action::Contact,
        Layer8Action::Continue,
        Layer8Action::Attach,
        Layer8Action::AddressRecipients,
    ]
    .into_iter()
    .enumerate()
    {
        let temp = TempDir::new().unwrap();
        let (mut request, capability, agent, polis) = fixture(action.clone());
        request.replay_id = format!("replay-{index}");
        let AuthorityDecision::Authorized(grant) =
            store(&temp).authorize(request, &capability, &agent, &polis)
        else {
            panic!("{action:?} was refused")
        };
        assert_eq!(grant.action, action);
        assert_eq!(grant.principal.credential_generation, 4);
        assert_eq!(grant.audit_hash.len(), 64);
    }
}

#[test]
fn rejects_recipient_substitution_widening_action_conversation_and_cross_polis() {
    let mutations: Vec<RequestMutation> = vec![
        Box::new(|r| {
            r.recipients.remove("agent-b");
            r.recipients.insert("agent-c".into());
        }),
        Box::new(|r| {
            r.recipients.insert("agent-c".into());
        }),
        Box::new(|r| r.action = Layer8Action::Continue),
        Box::new(|r| r.conversation_id = Some("other-conversation".into())),
        Box::new(|r| r.evidence.polis_id = "polis-2".into()),
    ];
    for (index, mutate) in mutations.into_iter().enumerate() {
        let temp = TempDir::new().unwrap();
        let (mut request, capability, agent, polis) = fixture(Layer8Action::AddressRecipients);
        request.replay_id = format!("negative-{index}");
        mutate(&mut request);
        let AuthorityDecision::Refused(refusal) =
            store(&temp).authorize(request, &capability, &agent, &polis)
        else {
            panic!("scope widening authorized")
        };
        assert_eq!(refusal.reason, RefusalReason::ScopeDenied);
    }
}

#[test]
fn identity_capability_policy_epoch_and_replay_fail_closed() {
    let cases: Vec<AuthorityMutation> = vec![
        Box::new(|r, _, _| r.evidence.authenticated = false),
        Box::new(|r, _, _| r.evidence.expires_at_epoch_secs = r.now_epoch_secs),
        Box::new(|r, _, _| r.evidence.revoked = true),
        Box::new(|r, _, _| r.evidence.current_credential_generation += 1),
        Box::new(|_, c, _| c.revoked = true),
        Box::new(|r, c, _| c.expires_at_epoch_secs = r.now_epoch_secs),
        Box::new(|_, c, _| c.epoch += 1),
        Box::new(|_, _, p| p.available = false),
    ];
    for (index, mutate) in cases.into_iter().enumerate() {
        let temp = TempDir::new().unwrap();
        let (mut request, mut capability, mut agent, polis) = fixture(Layer8Action::Continue);
        request.replay_id = format!("closed-{index}");
        mutate(&mut request, &mut capability, &mut agent);
        assert!(matches!(
            store(&temp).authorize(request, &capability, &agent, &polis),
            AuthorityDecision::Refused(_)
        ));
    }
    let temp = TempDir::new().unwrap();
    let authority = store(&temp);
    let (request, capability, agent, polis) = fixture(Layer8Action::Continue);
    assert!(matches!(
        authority.authorize(request.clone(), &capability, &agent, &polis),
        AuthorityDecision::Authorized(_)
    ));
    let AuthorityDecision::Refused(refusal) =
        authority.authorize(request, &capability, &agent, &polis)
    else {
        panic!("replay authorized")
    };
    assert_eq!(refusal.reason, RefusalReason::ReplayRefused);
}

#[test]
fn refusal_is_bounded_and_contains_no_private_inputs() {
    let temp = TempDir::new().unwrap();
    let (mut request, capability, agent, polis) = fixture(Layer8Action::Continue);
    request.evidence.authenticated = false;
    request.evidence.principal_id = "private-principal".into();
    let AuthorityDecision::Refused(refusal) =
        store(&temp).authorize(request, &capability, &agent, &polis)
    else {
        panic!("authorized")
    };
    let projection = serde_json::to_string(&refusal).unwrap();
    assert_eq!(refusal.reason, RefusalReason::IdentityUnavailable);
    for forbidden in [
        "private-principal",
        "polis-1",
        "conversation-7",
        "cap-1",
        "agent-policy",
    ] {
        assert!(!projection.contains(forbidden));
    }
}

#[test]
fn restart_verifies_chain_and_restores_replay_defense() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("audit.jsonl");
    let (request, capability, agent, polis) = fixture(Layer8Action::Continue);
    assert!(matches!(
        Layer8AuthorityStore::open(&path).unwrap().authorize(
            request.clone(),
            &capability,
            &agent,
            &polis
        ),
        AuthorityDecision::Authorized(_)
    ));
    let restarted = Layer8AuthorityStore::open(&path).unwrap();
    let AuthorityDecision::Refused(refusal) =
        restarted.authorize(request, &capability, &agent, &polis)
    else {
        panic!("restart forgot replay")
    };
    assert_eq!(refusal.reason, RefusalReason::ReplayRefused);
    drop(restarted);
    let reopened_after_refusal = Layer8AuthorityStore::open(&path).unwrap();
    let (mut next_request, next_capability, next_agent, next_polis) =
        fixture(Layer8Action::Continue);
    next_request.replay_id = "replay-after-refusal".into();
    assert!(matches!(
        reopened_after_refusal.authorize(next_request, &next_capability, &next_agent, &next_polis),
        AuthorityDecision::Authorized(_)
    ));
    let contents = std::fs::read_to_string(path).unwrap();
    for forbidden in ["human-1", "polis-1", "conversation-7", "replay-1"] {
        assert!(!contents.contains(forbidden));
    }
}

#[test]
fn corrupt_and_truncated_audit_refuse_restart() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("audit.jsonl");
    let (request, capability, agent, polis) = fixture(Layer8Action::Continue);
    let authority = Layer8AuthorityStore::open(&path).unwrap();
    assert!(matches!(
        authority.authorize(request, &capability, &agent, &polis),
        AuthorityDecision::Authorized(_)
    ));
    let mut contents = std::fs::read_to_string(&path).unwrap();
    contents = contents.replacen("\"authorized\":true", "\"authorized\":false", 1);
    std::fs::write(&path, contents).unwrap();
    assert!(matches!(
        Layer8AuthorityStore::open(&path),
        Err(RefusalReason::AuditUnavailable)
    ));

    let truncated = temp.path().join("truncated.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&truncated)
        .unwrap();
    file.write_all(b"{\"schema\":").unwrap();
    file.sync_all().unwrap();
    assert!(matches!(
        Layer8AuthorityStore::open(truncated),
        Err(RefusalReason::AuditUnavailable)
    ));
}

#[test]
fn concurrent_store_handles_serialize_against_the_current_audit_head() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("audit.jsonl");
    let first = Layer8AuthorityStore::open(&path).unwrap();
    let second = Layer8AuthorityStore::open(&path).unwrap();
    let (request, capability, agent, polis) = fixture(Layer8Action::Continue);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let run = |store: Layer8AuthorityStore| {
        let barrier = barrier.clone();
        let request = request.clone();
        let capability = capability.clone();
        let agent = agent.clone();
        let polis = polis.clone();
        std::thread::spawn(move || {
            barrier.wait();
            store.authorize(request, &capability, &agent, &polis)
        })
    };
    let first = run(first);
    let second = run(second);
    barrier.wait();
    let decisions = [first.join().unwrap(), second.join().unwrap()];
    assert_eq!(
        decisions
            .iter()
            .filter(|decision| matches!(decision, AuthorityDecision::Authorized(_)))
            .count(),
        1
    );
    assert_eq!(
        decisions
            .iter()
            .filter(|decision| matches!(decision, AuthorityDecision::Refused(refusal) if refusal.reason == RefusalReason::ReplayRefused))
            .count(),
        1
    );
    Layer8AuthorityStore::open(path).unwrap();
}

#[test]
fn conversation_authority_refuses_same_identity_signed_by_an_untrusted_key() {
    let temp = TempDir::new().unwrap();
    let (request, capability, agent_policy, polis_policy) = fixture(Layer8Action::Continue);
    let profile = ConversationAuthorityProfile {
        evidence: request.evidence,
        capabilities: vec![capability],
        agent_policies: vec![agent_policy],
        polis_policies: vec![polis_policy],
    };
    let authority = Layer8ConversationAuthority::new(
        Layer8AuthorityStore::open(temp.path().join("audit.jsonl")).unwrap(),
        profile,
    )
    .unwrap();
    let sender = identity(
        "human-1",
        "human-1-key",
        &SigningKey::from_bytes(&[99_u8; 32]),
    );
    assert!(matches!(
        authority.authorize(
            &sender,
            Layer8Action::Continue,
            "conversation-7".to_owned(),
            "agent-a".to_owned(),
            "cross-principal-replay".to_owned(),
            "cross-principal-correlation".to_owned(),
            100,
        ),
        AuthorityDecision::Refused(PublicRefusal {
            reason: RefusalReason::IdentityUnavailable,
            retryable: false,
            ..
        })
    ));
}

#[test]
fn retryable_policy_refusal_does_not_consume_replay_identity() {
    let temp = TempDir::new().unwrap();
    let authority = store(&temp);
    let (request, capability, mut agent, polis) = fixture(Layer8Action::Continue);
    agent.available = false;
    let AuthorityDecision::Refused(refusal) =
        authority.authorize(request.clone(), &capability, &agent, &polis)
    else {
        panic!("unavailable policy authorized")
    };
    assert_eq!(refusal.reason, RefusalReason::PolicyUnavailable);
    assert!(refusal.retryable);
    agent.available = true;
    assert!(matches!(
        authority.authorize(request, &capability, &agent, &polis),
        AuthorityDecision::Authorized(_)
    ));
}
