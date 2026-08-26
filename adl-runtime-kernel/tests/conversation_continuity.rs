use adl_runtime_kernel::{
    AttemptAdmission, AttemptOutcome, ConversationContinuityError, ConversationContinuityStore,
};

fn digest(label: &str) -> String {
    blake3::hash(label.as_bytes()).to_hex().to_string()
}

fn store(root: &tempfile::TempDir) -> ConversationContinuityStore {
    ConversationContinuityStore::open(root.path(), "principal-a", digest("authority")).unwrap()
}

#[test]
fn continuity_watermarks_and_receipts_survive_restart() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);

    continuity
        .advance_sender_watermark("conversation-a", 7, "sender-receipt-7", 1)
        .unwrap();
    continuity
        .advance_acknowledgement_watermark("conversation-a", 5, "ack-message-5", "ack-receipt-5", 2)
        .unwrap();
    continuity
        .record_delivery_receipt(
            "conversation-a",
            "attempt-1",
            "delivery-receipt-1",
            Some("response-receipt-1".to_string()),
            Some("ack-receipt-5".to_string()),
            3,
        )
        .unwrap();

    let snapshot = store(&root).snapshot().unwrap();
    assert_eq!(snapshot.sender_watermarks["conversation-a"], 7);
    assert_eq!(snapshot.acknowledgement_watermarks["conversation-a"], 5);
    let receipt = &snapshot.delivery_receipts["delivery-receipt-1"];
    assert_eq!(
        receipt.response_receipt_id.as_deref(),
        Some("response-receipt-1")
    );
    assert_eq!(
        receipt.acknowledgement_receipt_id.as_deref(),
        Some("ack-receipt-5")
    );
}

#[test]
fn completed_attempt_is_idempotent_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);

    assert_eq!(
        continuity
            .admit_attempt("conversation-a", "idem-1")
            .unwrap(),
        AttemptAdmission::NewOrRetryable
    );
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-1",
            "idem-1",
            AttemptOutcome::Completed,
            "completed-receipt-1",
            1,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-1")
            .unwrap(),
        AttemptAdmission::DuplicateCompleted {
            attempt_id: "attempt-1".to_string(),
            receipt_id: "completed-receipt-1".to_string()
        }
    );
}

#[test]
fn idempotency_is_scoped_by_conversation_after_restart() {
    let root = tempfile::tempdir().unwrap();
    store(&root)
        .record_attempt(
            "conversation-a",
            "attempt-1",
            "idem-shared",
            AttemptOutcome::Completed,
            "completed-receipt-1",
            1,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-b", "idem-shared")
            .unwrap(),
        AttemptAdmission::NewOrRetryable
    );
    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-shared")
            .unwrap(),
        AttemptAdmission::DuplicateCompleted {
            attempt_id: "attempt-1".to_string(),
            receipt_id: "completed-receipt-1".to_string()
        }
    );
}

#[test]
fn ambiguous_dispatch_remains_ambiguous_after_restart() {
    let root = tempfile::tempdir().unwrap();
    store(&root)
        .record_attempt(
            "conversation-a",
            "attempt-ambiguous",
            "idem-ambiguous",
            AttemptOutcome::DispatchedAmbiguous,
            "ambiguous-receipt",
            1,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-ambiguous")
            .unwrap(),
        AttemptAdmission::DuplicateAmbiguous {
            attempt_id: "attempt-ambiguous".to_string(),
            receipt_id: "ambiguous-receipt".to_string()
        }
    );
}

#[test]
fn retryable_attempt_cannot_downgrade_completed_attempt_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-completed",
            "idem-downgrade-completed",
            AttemptOutcome::Completed,
            "completed-receipt",
            1,
        )
        .unwrap();
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-stale-retry",
            "idem-downgrade-completed",
            AttemptOutcome::PreDispatchRetryable,
            "stale-retry-receipt",
            2,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-downgrade-completed")
            .unwrap(),
        AttemptAdmission::DuplicateCompleted {
            attempt_id: "attempt-completed".to_string(),
            receipt_id: "completed-receipt".to_string()
        }
    );
}

#[test]
fn retryable_attempt_cannot_downgrade_ambiguous_attempt_after_restart() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-ambiguous",
            "idem-downgrade-ambiguous",
            AttemptOutcome::DispatchedAmbiguous,
            "ambiguous-receipt",
            1,
        )
        .unwrap();
    continuity
        .record_attempt(
            "conversation-a",
            "attempt-stale-retry",
            "idem-downgrade-ambiguous",
            AttemptOutcome::PreDispatchRetryable,
            "stale-retry-receipt",
            2,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-downgrade-ambiguous")
            .unwrap(),
        AttemptAdmission::DuplicateAmbiguous {
            attempt_id: "attempt-ambiguous".to_string(),
            receipt_id: "ambiguous-receipt".to_string()
        }
    );
}

#[test]
fn pre_dispatch_retryable_attempt_does_not_block_retry_after_restart() {
    let root = tempfile::tempdir().unwrap();
    store(&root)
        .record_attempt(
            "conversation-a",
            "attempt-pre-dispatch",
            "idem-retry",
            AttemptOutcome::PreDispatchRetryable,
            "retryable-receipt",
            1,
        )
        .unwrap();

    assert_eq!(
        store(&root)
            .admit_attempt("conversation-a", "idem-retry")
            .unwrap(),
        AttemptAdmission::NewOrRetryable
    );
}

#[test]
fn stale_acknowledgement_watermark_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);
    continuity
        .advance_acknowledgement_watermark(
            "conversation-a",
            10,
            "ack-message-10",
            "ack-receipt-10",
            1,
        )
        .unwrap();

    assert!(matches!(
        continuity.advance_acknowledgement_watermark(
            "conversation-a",
            10,
            "ack-message-10-duplicate",
            "ack-receipt-10-duplicate",
            2,
        ),
        Err(ConversationContinuityError::StaleAcknowledgementWatermark)
    ));
}

#[test]
fn replay_decision_is_owned_and_restored() {
    let root = tempfile::tempdir().unwrap();
    store(&root)
        .record_replay_decision(
            "conversation-a",
            "replay-1",
            "runtime-owner-a",
            17,
            "replay-receipt-1",
            1,
        )
        .unwrap();

    let snapshot = store(&root).snapshot().unwrap();
    let replay = &snapshot.replay_decisions["replay-1"];
    assert_eq!(replay.owner_id, "runtime-owner-a");
    assert_eq!(replay.high_watermark, 17);
    assert_eq!(replay.receipt_id, "replay-receipt-1");
}

#[test]
fn deleted_conversation_events_are_not_replayed_into_continuity_snapshot() {
    let root = tempfile::tempdir().unwrap();
    let continuity = store(&root);
    continuity
        .advance_sender_watermark("conversation-a", 1, "sender-receipt-1", 1)
        .unwrap();
    let journal = adl_runtime_kernel::ConversationJournal::open(root.path()).unwrap();
    journal
        .record_deletion(adl_runtime_kernel::ConversationDeletionMarker {
            conversation_id: "conversation-a".to_string(),
            reason: "operator-deletion".to_string(),
            authority_audit_hash: digest("deletion-authority"),
            created_at_epoch_ms: 2,
        })
        .unwrap();

    assert!(store(&root)
        .snapshot()
        .unwrap()
        .sender_watermarks
        .is_empty());
}
