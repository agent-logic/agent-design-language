use std::collections::BTreeSet;

use adl_runtime_kernel::{
    AttemptAdmission, AttemptOutcome, ConversationContinuityStore, ConversationDeletionMarker,
    ConversationHistoryAccessPolicy, ConversationHistoryMessage, ConversationHistoryPageRequest,
    ConversationHistoryStore, ConversationJournal, ConversationRetentionMarker,
    CONVERSATION_HISTORY_SCHEMA,
};

const H: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn digest(label: &str) -> String {
    blake3::hash(label.as_bytes()).to_hex().to_string()
}

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

fn message(conversation_id: &str, message_id: &str, body: &str) -> ConversationHistoryMessage {
    ConversationHistoryMessage {
        schema: CONVERSATION_HISTORY_SCHEMA.to_string(),
        conversation_id: conversation_id.to_string(),
        message_id: message_id.to_string(),
        speaker_id: "agent-a".to_string(),
        body: body.to_string(),
        created_at_epoch_ms: 1_786_608_000_000,
    }
}

#[test]
fn durable_history_parent_chain_survives_restart_and_deletion_coherently() {
    let root = tempfile::tempdir().unwrap();
    let history = ConversationHistoryStore::open(root.path()).unwrap();
    let continuity =
        ConversationContinuityStore::open(root.path(), "principal-a", digest("authority")).unwrap();

    history
        .append_message(
            "operator-a",
            H,
            message("conversation-a", "m1", "visible before retention"),
        )
        .unwrap();
    continuity
        .advance_sender_watermark("conversation-a", 11, "sender-receipt-11", 1)
        .unwrap();
    continuity
        .advance_acknowledgement_watermark("conversation-a", 9, "ack-message-9", "ack-receipt-9", 2)
        .unwrap();
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-1",
            "idem-1",
            AttemptOutcome::Completed,
            "completed-receipt-1",
            3,
        )
        .unwrap();
    continuity
        .record_delivery_receipt(
            "conversation-a",
            "attempt-1",
            "delivery-receipt-1",
            Some("response-receipt-1".to_string()),
            Some("ack-receipt-9".to_string()),
            4,
        )
        .unwrap();
    continuity
        .record_replay_decision(
            "conversation-a",
            "replay-1",
            "runtime-owner-a",
            11,
            "replay-receipt-1",
            5,
        )
        .unwrap();

    let restarted_history = ConversationHistoryStore::open(root.path()).unwrap();
    let restarted_continuity =
        ConversationContinuityStore::open(root.path(), "principal-a", digest("authority")).unwrap();
    assert_eq!(
        restarted_continuity
            .admit_attempt("conversation-a", "idem-1")
            .unwrap(),
        AttemptAdmission::DuplicateCompleted {
            attempt_id: "attempt-1".to_string(),
            receipt_id: "completed-receipt-1".to_string()
        }
    );
    let continuity_snapshot = restarted_continuity.snapshot().unwrap();
    assert_eq!(continuity_snapshot.sender_watermarks["conversation-a"], 11);
    assert_eq!(
        continuity_snapshot.acknowledgement_watermarks["conversation-a"],
        9
    );
    assert_eq!(
        continuity_snapshot.delivery_receipts["delivery-receipt-1"]
            .acknowledgement_receipt_id
            .as_deref(),
        Some("ack-receipt-9")
    );
    assert_eq!(
        continuity_snapshot.replay_decisions["replay-1"].owner_id,
        "runtime-owner-a"
    );
    assert_eq!(
        restarted_history
            .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
            .unwrap()
            .len(),
        1
    );

    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .record_retention(ConversationRetentionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "operator-retention-window".to_string(),
            retain_until_epoch_ms: Some(1_786_694_400_000),
            authority_audit_hash: digest("retention-authority"),
            created_at_epoch_ms: 1_786_608_001_000,
        })
        .unwrap();

    let retained_snapshot = ConversationJournal::open(root.path())
        .unwrap()
        .snapshot()
        .unwrap();
    let retained_marker = retained_snapshot
        .retention_by_conversation
        .get("conversation-a")
        .expect("retention marker must persist across journal restart");
    assert_eq!(retained_marker.reason, "operator-retention-window");
    assert_eq!(
        retained_marker.retain_until_epoch_ms,
        Some(1_786_694_400_000)
    );
    assert_eq!(
        retained_marker.authority_audit_hash,
        digest("retention-authority")
    );
    assert_eq!(retained_snapshot.records.len(), 7);
    assert_eq!(retained_snapshot.committed_events().count(), 6);
    assert_eq!(
        ConversationHistoryStore::open(root.path())
            .unwrap()
            .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
            .unwrap()
            .len(),
        1
    );

    journal
        .record_deletion(ConversationDeletionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "operator-deletion-request".to_string(),
            authority_audit_hash: digest("deletion-authority"),
            created_at_epoch_ms: 1_786_608_002_000,
        })
        .unwrap();

    let after_delete_history = ConversationHistoryStore::open(root.path()).unwrap();
    let after_delete_continuity =
        ConversationContinuityStore::open(root.path(), "principal-a", digest("authority")).unwrap();
    assert!(after_delete_history
        .page(
            &policy("conversation-a"),
            ConversationHistoryPageRequest {
                conversation_id: "conversation-a".to_string(),
                page_size: 10,
                cursor: None,
            },
        )
        .unwrap()
        .records
        .is_empty());
    assert!(after_delete_history
        .restore_observatory_transcript(&policy("conversation-a"), "conversation-a")
        .unwrap()
        .is_empty());
    assert!(after_delete_continuity
        .snapshot()
        .unwrap()
        .sender_watermarks
        .is_empty());
    assert!(ConversationJournal::open(root.path())
        .unwrap()
        .snapshot()
        .unwrap()
        .deleted_conversations
        .contains("conversation-a"));
}
