use adl_runtime_kernel::{
    ConversationDeletionMarker, ConversationJournal, ConversationJournalEntry,
    ConversationJournalError, ConversationJournalEvent, ConversationJournalMigration,
    ConversationRetentionMarker, CONVERSATION_JOURNAL_FILE,
};

fn digest(label: &str) -> String {
    blake3::hash(label.as_bytes()).to_hex().to_string()
}

fn event(id: &str, conversation: &str) -> ConversationJournalEvent {
    ConversationJournalEvent {
        event_id: id.to_string(),
        conversation_id: conversation.to_string(),
        principal_id: "principal-a".to_string(),
        authority_audit_hash: digest("authority"),
        payload_hash: digest(id),
        receipt_ref: Some(format!("receipt:{id}")),
        created_at_epoch_ms: 1_786_608_000_000,
    }
}

#[test]
fn durable_journal_appends_and_reloads_committed_events() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();

    let first = journal
        .append_event(event("event-1", "conversation-a"))
        .unwrap();
    let second = journal
        .append_event(event("event-2", "conversation-a"))
        .unwrap();

    assert_eq!(first.sequence, 1);
    assert_eq!(first.previous_hash, "conversation-journal-genesis-v1");
    assert_eq!(second.sequence, 2);
    assert_eq!(second.previous_hash, first.record_hash);

    let reopened = ConversationJournal::open(root.path()).unwrap();
    let snapshot = reopened.snapshot().unwrap();
    let committed: Vec<_> = snapshot.committed_events().collect();
    assert_eq!(snapshot.head_sequence, 2);
    assert_eq!(committed.len(), 2);
    assert_eq!(committed[0].event_id, "event-1");
    assert_eq!(committed[1].event_id, "event-2");
}

#[test]
fn durable_journal_refuses_partial_trailing_record_on_restart() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .append_event(event("event-1", "conversation-a"))
        .unwrap();

    let path = root.path().join(CONVERSATION_JOURNAL_FILE);
    let mut bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.pop(), Some(b'\n'));
    std::fs::write(&path, bytes).unwrap();

    assert!(matches!(
        ConversationJournal::open(root.path()),
        Err(ConversationJournalError::CorruptRecord)
    ));
}

#[test]
fn durable_journal_refuses_hash_chain_corruption_on_restart() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .append_event(event("event-1", "conversation-a"))
        .unwrap();

    let path = root.path().join(CONVERSATION_JOURNAL_FILE);
    let mut line: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).expect("journal line is json");
    line["entry"]["event"]["payload_hash"] = serde_json::Value::String(digest("tampered"));
    std::fs::write(
        &path,
        format!("{}\n", serde_json::to_string(&line).unwrap()),
    )
    .unwrap();

    assert!(matches!(
        ConversationJournal::open(root.path()),
        Err(ConversationJournalError::CorruptRecord)
    ));
}

#[test]
fn durable_journal_retention_and_deletion_markers_are_auditable_without_replay_policy() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .append_event(event("event-1", "conversation-a"))
        .unwrap();
    journal
        .record_retention(ConversationRetentionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "operator-retention-window".to_string(),
            retain_until_epoch_ms: Some(1_786_694_400_000),
            authority_audit_hash: digest("retention-authority"),
            created_at_epoch_ms: 1_786_608_001_000,
        })
        .unwrap();
    journal
        .record_deletion(ConversationDeletionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "operator-deletion-request".to_string(),
            authority_audit_hash: digest("deletion-authority"),
            created_at_epoch_ms: 1_786_608_002_000,
        })
        .unwrap();

    let snapshot = ConversationJournal::open(root.path())
        .unwrap()
        .snapshot()
        .unwrap();
    assert_eq!(snapshot.records.len(), 3);
    assert!(snapshot.deleted_conversations.contains("conversation-a"));
    assert_eq!(
        snapshot
            .retention_by_conversation
            .get("conversation-a")
            .unwrap()
            .reason,
        "operator-retention-window"
    );
    assert_eq!(snapshot.committed_events().count(), 0);
}

#[test]
fn durable_journal_supports_current_schema_migration_marker_and_rejects_future_schema() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .record_migration(ConversationJournalMigration {
            from_schema_version: 1,
            to_schema_version: 1,
            migration_id: "noop-v1".to_string(),
            authority_audit_hash: digest("migration-authority"),
            created_at_epoch_ms: 1_786_608_003_000,
        })
        .unwrap();

    let snapshot = journal.snapshot().unwrap();
    assert_eq!(snapshot.schema_version, 1);
    assert!(matches!(
        journal.record_migration(ConversationJournalMigration {
            from_schema_version: 1,
            to_schema_version: 2,
            migration_id: "future-v2".to_string(),
            authority_audit_hash: digest("migration-authority"),
            created_at_epoch_ms: 1_786_608_004_000,
        }),
        Err(ConversationJournalError::UnsupportedSchema)
    ));
}

#[test]
fn durable_journal_rejects_empty_authority_or_payload_identity() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    let mut invalid = event("event-1", "conversation-a");
    invalid.authority_audit_hash.clear();

    assert!(matches!(
        journal.append_event(invalid),
        Err(ConversationJournalError::InvalidEntry(
            "authority_audit_hash"
        ))
    ));
}

#[test]
fn durable_journal_record_kinds_remain_foundation_only() {
    let root = tempfile::tempdir().unwrap();
    let journal = ConversationJournal::open(root.path()).unwrap();
    journal
        .append_event(event("event-1", "conversation-a"))
        .unwrap();
    let snapshot = journal.snapshot().unwrap();

    assert!(matches!(
        snapshot.records[0].entry,
        ConversationJournalEntry::Event(_)
    ));
}
