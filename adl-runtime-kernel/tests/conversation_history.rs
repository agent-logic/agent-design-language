use std::collections::BTreeSet;

use adl_runtime_kernel::{
    ConversationDeletionMarker, ConversationHistoryA2aExchange, ConversationHistoryAccessPolicy,
    ConversationHistoryError, ConversationHistoryMessage, ConversationHistoryPageRequest,
    ConversationHistorySearchRequest, ConversationHistoryStore, ConversationJournal,
    ConversationJournalEvent, CONVERSATION_HISTORY_A2A_EXCHANGE_SCHEMA,
    CONVERSATION_HISTORY_SCHEMA,
};

const H: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn policy(conversation: &str) -> ConversationHistoryAccessPolicy {
    ConversationHistoryAccessPolicy {
        principal_id: "operator-a".to_string(),
        authority_audit_hash: H.to_string(),
        allowed_conversations: BTreeSet::from([conversation.to_string()]),
        revoked_conversations: BTreeSet::new(),
        allow_export: true,
        allow_redaction: true,
    }
}

fn message(
    conversation: &str,
    message_id: &str,
    body: &str,
    ms: u64,
) -> ConversationHistoryMessage {
    ConversationHistoryMessage {
        schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
        conversation_id: conversation.to_string(),
        message_id: message_id.to_string(),
        speaker_id: "agent-a".to_string(),
        body: body.to_string(),
        created_at_epoch_ms: ms,
    }
}

fn append(
    store: &ConversationHistoryStore,
    conversation: &str,
    message_id: &str,
    body: &str,
    ms: u64,
) {
    store
        .append_message("operator-a", H, message(conversation, message_id, body, ms))
        .unwrap();
}

fn a2a_exchange(conversation: &str, exchange_id: &str) -> ConversationHistoryA2aExchange {
    ConversationHistoryA2aExchange {
        schema: CONVERSATION_HISTORY_A2A_EXCHANGE_SCHEMA.to_string(),
        conversation_id: conversation.to_string(),
        exchange_id: exchange_id.to_string(),
        sender_id: "beacon".to_string(),
        recipient_id: "ember".to_string(),
        outbound_message_id: format!("{exchange_id}:outbound"),
        outbound_body: "Ember, please answer Beacon verbatim.".to_string(),
        outbound_created_at_epoch_ms: 10,
        reply_message_id: format!("{exchange_id}:reply"),
        reply_body: "Beacon, here is Ember's verbatim reply.".to_string(),
        reply_created_at_epoch_ms: 11,
        status: "delivered".to_string(),
        causal_id: format!("{conversation}:{exchange_id}:a2a-work-001"),
        work_id: "a2a-work-001".to_string(),
    }
}

#[test]
fn authorized_pagination_rejects_stale_cursor_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    append(&store, "conversation-a", "m1", "hello one", 1);
    append(&store, "conversation-a", "m2", "hello two", 2);
    let first = store
        .page(
            &policy("conversation-a"),
            ConversationHistoryPageRequest {
                conversation_id: "conversation-a".to_string(),
                page_size: 1,
                cursor: None,
            },
        )
        .unwrap();
    assert_eq!(first.records[0].message_id, "m1");
    assert!(first.next_cursor.is_some());

    append(&store, "conversation-a", "m3", "hello three", 3);
    let reopened = ConversationHistoryStore::open(root.path()).unwrap();
    assert!(matches!(
        reopened.page(
            &policy("conversation-a"),
            ConversationHistoryPageRequest {
                conversation_id: "conversation-a".to_string(),
                page_size: 1,
                cursor: first.next_cursor,
            },
        ),
        Err(ConversationHistoryError::StaleCursor)
    ));
}

#[test]
fn a2a_exchange_restores_verbatim_causal_records_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    store
        .append_a2a_exchange(
            "operator-a",
            H,
            a2a_exchange("conversation-a", "exchange-1"),
        )
        .unwrap();

    let restored = ConversationHistoryStore::open(root.path())
        .unwrap()
        .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
        .unwrap();
    assert_eq!(restored.len(), 2);
    assert_eq!(
        restored[0].history_kind.as_deref(),
        Some("agent_to_agent_turn")
    );
    assert_eq!(restored[0].a2a_role.as_deref(), Some("outbound"));
    assert_eq!(restored[0].speaker_id, "agent:beacon");
    assert_eq!(restored[0].body, "Ember, please answer Beacon verbatim.");
    assert_eq!(restored[1].a2a_role.as_deref(), Some("reply"));
    assert_eq!(restored[1].speaker_id, "agent:ember");
    assert_eq!(restored[1].body, "Beacon, here is Ember's verbatim reply.");
    assert_eq!(restored[0].causal_id, restored[1].causal_id);
    assert_eq!(restored[0].sender_id.as_deref(), Some("beacon"));
    assert_eq!(restored[0].recipient_id.as_deref(), Some("ember"));
    assert_eq!(restored[0].work_id.as_deref(), Some("a2a-work-001"));
}

#[test]
fn a2a_exchange_replay_does_not_duplicate_and_redaction_still_applies() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    let exchange = a2a_exchange("conversation-a", "exchange-1");
    store
        .append_a2a_exchange("operator-a", H, exchange.clone())
        .unwrap();
    store
        .append_a2a_exchange("operator-a", H, exchange)
        .unwrap();
    store
        .redact(
            &policy("conversation-a"),
            "conversation-a",
            "exchange-1:reply",
            "operator redaction",
            12,
        )
        .unwrap();

    let restored = store
        .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
        .unwrap();
    assert_eq!(restored.len(), 2, "replayed A2A receipt must not duplicate");
    assert_eq!(restored[0].body, "Ember, please answer Beacon verbatim.");
    assert_eq!(restored[1].body, "[redacted]");
    assert_eq!(
        restored[1].redaction_reason.as_deref(),
        Some("operator redaction")
    );
}

#[test]
fn revoked_access_and_private_memory_fail_closed() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    append(&store, "conversation-a", "m1", "visible transcript", 1);
    ConversationJournal::open(root.path())
        .unwrap()
        .append_event(ConversationJournalEvent {
            event_id: "private-memory-event".to_string(),
            conversation_id: "conversation-a".to_string(),
            principal_id: "operator-a".to_string(),
            authority_audit_hash: H.to_string(),
            payload_hash: H.to_string(),
            receipt_ref: Some("private-memory:raw-secret-that-must-not-export".to_string()),
            created_at_epoch_ms: 2,
        })
        .unwrap();

    let mut revoked = policy("conversation-a");
    revoked
        .revoked_conversations
        .insert("conversation-a".to_string());
    assert!(matches!(
        store.restore_observatory_transcript(&revoked, "conversation-a"),
        Err(ConversationHistoryError::Unauthorized)
    ));

    let export = store
        .export(&policy("conversation-a"), "conversation-a")
        .unwrap();
    assert_eq!(export.records.len(), 1);
    assert_eq!(export.records[0].body, "visible transcript");
    assert!(!serde_json::to_string(&export)
        .unwrap()
        .contains("raw-secret-that-must-not-export"));
}

#[test]
fn deletion_marker_hides_page_search_export_and_restore_history() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    append(&store, "conversation-a", "m1", "deleted transcript", 1);
    ConversationJournal::open(root.path())
        .unwrap()
        .record_deletion(ConversationDeletionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "retention window elapsed".to_string(),
            authority_audit_hash: H.to_string(),
            created_at_epoch_ms: 2,
        })
        .unwrap();

    let page = store
        .page(
            &policy("conversation-a"),
            ConversationHistoryPageRequest {
                conversation_id: "conversation-a".to_string(),
                page_size: 10,
                cursor: None,
            },
        )
        .unwrap();
    assert!(page.records.is_empty());
    assert!(page.next_cursor.is_none());

    assert!(store
        .search(
            &policy("conversation-a"),
            ConversationHistorySearchRequest {
                conversation_id: "conversation-a".to_string(),
                query: "deleted".to_string(),
                limit: 10,
            },
        )
        .unwrap()
        .is_empty());
    assert!(store
        .export(&policy("conversation-a"), "conversation-a")
        .unwrap()
        .records
        .is_empty());
    assert!(ConversationHistoryStore::open(root.path())
        .unwrap()
        .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
        .unwrap()
        .is_empty());
}

#[test]
fn search_export_redaction_and_restore_apply_public_safe_redactions() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    append(&store, "conversation-a", "m1", "public hello", 1);
    append(
        &store,
        "conversation-a",
        "m2",
        "token secret should redact",
        2,
    );

    let hits = store
        .search(
            &policy("conversation-a"),
            ConversationHistorySearchRequest {
                conversation_id: "conversation-a".to_string(),
                query: "secret".to_string(),
                limit: 10,
            },
        )
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].message_id, "m2");

    store
        .redact(
            &policy("conversation-a"),
            "conversation-a",
            "m2",
            "operator redaction",
            3,
        )
        .unwrap();

    for record in store
        .export(&policy("conversation-a"), "conversation-a")
        .unwrap()
        .records
    {
        if record.message_id == "m2" {
            assert!(record.redacted);
            assert_eq!(record.body, "[redacted]");
            assert_eq!(
                record.redaction_reason.as_deref(),
                Some("operator redaction")
            );
        }
    }
    let restored = ConversationHistoryStore::open(root.path())
        .unwrap()
        .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
        .unwrap();
    assert_eq!(restored.len(), 2);
    assert!(restored
        .iter()
        .any(|record| record.redacted && record.body == "[redacted]"));
}

#[test]
fn export_and_redaction_require_explicit_authority() {
    let root = tempfile::tempdir().unwrap();
    let store = ConversationHistoryStore::open(root.path()).unwrap();
    append(&store, "conversation-a", "m1", "hello", 1);
    let mut no_export = policy("conversation-a");
    no_export.allow_export = false;
    assert!(matches!(
        store.export(&no_export, "conversation-a"),
        Err(ConversationHistoryError::ExportForbidden)
    ));
    let mut no_redaction = policy("conversation-a");
    no_redaction.allow_redaction = false;
    assert!(matches!(
        store.redact(&no_redaction, "conversation-a", "m1", "nope", 2),
        Err(ConversationHistoryError::RedactionForbidden)
    ));
}
