use csdlc_v3::adapters::{
    ChildCredentialInjector, CommandInvocation, CredentialScope, FakeGitAdapter,
    FakeProcessAdapter, GitAdapter, ProcessAdapter, ProcessOutput, ProcessStatus,
    RealProcessAdapter, StaticCredentialResolver,
};
use csdlc_v3::lifecycle::{
    decide, transition_matrix, Capability, CapabilitySet, LifecycleCommand, LifecycleState,
    ProjectionInvalidation, RejectReason, ReviewRecoveryProvenance, TransitionOutcome,
};
use csdlc_v3::storage::{
    classify_recovery, CommitResult, DurableTransactionStore, ProjectionWrite,
    RecoveryClassification, RecoveryObservation, RecoveryRejectReason, RecoveryRepair, StateRecord,
    TransactionIntent, TransactionStore,
};
use csdlc_v3::LIFECYCLE_KERNEL_PREDECESSORS;
use std::collections::BTreeSet;
use std::fs;

fn full_capabilities() -> CapabilitySet {
    CapabilitySet::new([
        Capability::BoundTopology,
        Capability::ImplementationEvidence,
        Capability::IndependentExactHeadReview,
        Capability::PublicationLinkage,
        Capability::MergeReadinessEvidence,
        Capability::LiveMergeEvidence,
        Capability::LiveTerminalEvidence,
        Capability::TerminalReceipt,
    ])
}

#[test]
fn transition_matrix_explicitly_classifies_every_state_command_pair() {
    assert_eq!(LIFECYCLE_KERNEL_PREDECESSORS, [168, 169, 170]);
    for issue in LIFECYCLE_KERNEL_PREDECESSORS {
        assert!(csdlc_v3::is_v3c_lifecycle_predecessor(issue));
    }
    assert!(!csdlc_v3::is_v3c_lifecycle_predecessor(167));
    assert!(!csdlc_v3::is_v3c_lifecycle_predecessor(171));
    let matrix = transition_matrix();
    assert_eq!(matrix.len(), 90);
    let unique_pairs = matrix
        .iter()
        .map(|decision| (decision.from, decision.command))
        .collect::<BTreeSet<_>>();
    assert_eq!(unique_pairs.len(), matrix.len());
    assert!(matrix.iter().all(|decision| matches!(
        decision.outcome,
        TransitionOutcome::Allowed { .. } | TransitionOutcome::Rejected { .. }
    )));
}

#[test]
fn transition_review_recovery_matches_retained_v2_behavior() {
    for state in [
        LifecycleState::Reviewed,
        LifecycleState::Published,
        LifecycleState::MergeReady,
    ] {
        let decision = decide(state, LifecycleCommand::RecoverReview, &full_capabilities());
        assert_eq!(
            decision.outcome,
            TransitionOutcome::Allowed {
                to: LifecycleState::Implemented,
                invalidates: vec![
                    ProjectionInvalidation::Readiness,
                    ProjectionInvalidation::Review,
                    ProjectionInvalidation::Publication,
                    ProjectionInvalidation::Terminal
                ]
            }
        );
    }
    for state in [LifecycleState::Merged, LifecycleState::ClosedOut] {
        let decision = decide(state, LifecycleCommand::RecoverReview, &full_capabilities());
        assert_eq!(
            decision.outcome,
            TransitionOutcome::Rejected {
                reason: RejectReason::InvalidState
            }
        );
    }
}

#[test]
fn review_recovery_requires_structured_stale_truth_provenance() {
    let provenance =
        ReviewRecoveryProvenance::new("worker-6", "review failed exact-head check", "b8c42844")
            .expect("structured provenance");
    assert_eq!(
        provenance.audit_provenance(),
        r"review_recovery actor=worker-6 reason=review\sfailed\sexact-head\scheck stale_review_revision=b8c42844"
    );
    assert!(ReviewRecoveryProvenance::new("", "reason", "head").is_err());
    assert!(ReviewRecoveryProvenance::new("actor", "", "head").is_err());
    assert!(ReviewRecoveryProvenance::new("actor", "reason", "").is_err());
}

#[test]
fn transition_branch_observation_alone_does_not_authorize_bind() {
    let decision = decide(
        LifecycleState::Ready,
        LifecycleCommand::Bind,
        &CapabilitySet::default(),
    );
    assert_eq!(
        decision.outcome,
        TransitionOutcome::Rejected {
            reason: RejectReason::BranchObservationOnly
        }
    );
}

#[test]
fn transition_merge_ready_requires_current_readiness_evidence() {
    let decision = decide(
        LifecycleState::Published,
        LifecycleCommand::MarkMergeReady,
        &CapabilitySet::default(),
    );
    assert_eq!(
        decision.outcome,
        TransitionOutcome::Rejected {
            reason: RejectReason::MissingCapability(Capability::MergeReadinessEvidence)
        }
    );
}

#[test]
fn transition_finish_requires_live_terminal_evidence_not_existing_receipt() {
    let receipt_only = decide(
        LifecycleState::Merged,
        LifecycleCommand::Finish,
        &CapabilitySet::new([Capability::TerminalReceipt]),
    );
    assert_eq!(
        receipt_only.outcome,
        TransitionOutcome::Rejected {
            reason: RejectReason::MissingCapability(Capability::LiveTerminalEvidence)
        }
    );

    let live_terminal_evidence = decide(
        LifecycleState::Merged,
        LifecycleCommand::Finish,
        &CapabilitySet::new([Capability::LiveTerminalEvidence]),
    );
    assert_eq!(
        live_terminal_evidence.outcome,
        TransitionOutcome::Allowed {
            to: LifecycleState::ClosedOut,
            invalidates: vec![
                ProjectionInvalidation::Terminal,
                ProjectionInvalidation::CleanupEligibility
            ]
        }
    );

    let cleanup_without_receipt = decide(
        LifecycleState::ClosedOut,
        LifecycleCommand::Cleanup,
        &CapabilitySet::new([Capability::LiveTerminalEvidence]),
    );
    assert_eq!(
        cleanup_without_receipt.outcome,
        TransitionOutcome::Rejected {
            reason: RejectReason::TerminalReceiptRequired
        }
    );
}

#[test]
fn transaction_stale_writer_fails_before_commit() {
    let initial = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(initial.clone()).expect("valid initial digest");
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest.clone(),
            "bind provenance",
        )
        .expect("bind transaction stages");
    store
        .commit(transaction, ProjectionWrite::Success)
        .expect("initial commit succeeds");
    let stale = store.begin(
        LifecycleCommand::RecordImplementation,
        &full_capabilities(),
        initial.generation,
        initial.digest,
        "stale writer",
    );
    assert!(matches!(
        stale,
        Err(csdlc_v3::storage::StoreError::StaleWriter { .. })
    ));
    assert_eq!(store.committed().generation, 1);
    assert_eq!(store.committed().state, LifecycleState::Bound);
}

#[test]
fn transaction_state_commit_is_atomic_and_projection_failure_requires_repair() {
    let initial = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(initial.clone()).expect("valid initial digest");
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest,
            "durable typed intent",
        )
        .expect("bind transaction stages");
    let result = store
        .commit(transaction, ProjectionWrite::FailAfterStateCommit)
        .expect("post-state projection failure still commits state");
    let CommitResult::ProjectionRepairRequired(committed) = result else {
        panic!("projection failure must not roll back state");
    };
    assert_eq!(committed.state, LifecycleState::Bound);
    assert!(committed.projections_repair_required);
    assert_eq!(
        classify_recovery(RecoveryObservation::NoIntent {
            state: committed.clone()
        }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::RepairIntentMissing
        }
    );
    assert_eq!(committed.audit[0].provenance, "durable typed intent");
    assert_eq!(store.committed().digest, committed.digest);
    let blocked = store.begin(
        LifecycleCommand::RecordImplementation,
        &full_capabilities(),
        committed.generation,
        committed.digest,
        "must wait for projection repair",
    );
    assert_eq!(
        blocked,
        Err(csdlc_v3::storage::StoreError::ProjectionRepairRequired)
    );
}

#[test]
fn transaction_commits_preserve_projection_invalidations() {
    let initial = StateRecord::new(LifecycleState::Reviewed);
    let mut store = TransactionStore::new(initial.clone()).expect("valid initial digest");
    let publish = store
        .begin(
            LifecycleCommand::Publish,
            &full_capabilities(),
            initial.generation,
            initial.digest,
            "publish linkage",
        )
        .expect("publish stages");
    let published = match store
        .commit(publish, ProjectionWrite::Success)
        .expect("publish commits")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    assert_eq!(
        published.invalidated_projections,
        [ProjectionInvalidation::Publication]
    );
    assert_eq!(
        store.begin(
            LifecycleCommand::RecoverReview,
            &full_capabilities(),
            published.generation,
            published.digest.clone(),
            "review stale",
        ),
        Err(csdlc_v3::storage::StoreError::StructuredReviewRecoveryProvenanceRequired)
    );
    let recover = store
        .begin_review_recovery(
            &full_capabilities(),
            published.generation,
            published.digest,
            ReviewRecoveryProvenance::new("worker-6", "review stale", "old-head")
                .expect("structured review recovery provenance"),
        )
        .expect("review recovery stages with structured provenance");
    let recovered = match store
        .commit(recover, ProjectionWrite::Success)
        .expect("review recovery commits")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    assert_eq!(
        recovered.invalidated_projections,
        [
            ProjectionInvalidation::Readiness,
            ProjectionInvalidation::Review,
            ProjectionInvalidation::Publication,
            ProjectionInvalidation::Terminal
        ]
    );
}

#[test]
fn durable_transaction_store_persists_intent_before_atomic_state_replacement() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-store-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    {
        let initial = StateRecord::new(LifecycleState::Ready);
        let mut store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        assert!(directory.join("state.lock").exists());
        let transaction = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest,
                "durable pre-network intent",
            )
            .expect("bind transaction stages");
        let committed = match store
            .commit(transaction, ProjectionWrite::Success)
            .expect("durable commit succeeds")
        {
            CommitResult::Committed(state) => state,
            CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
        };
        assert_eq!(committed.generation, 1);
        assert_eq!(store.journal().len(), 1);
        assert!(fs::read_to_string(directory.join("intents.jsonl"))
            .expect("intent journal")
            .contains("durable pre-network intent"));
        let state_json = fs::read_to_string(directory.join("state.json")).expect("state json");
        assert!(state_json.contains("\"generation\":1"));
        assert!(state_json.contains(&committed.digest));
        assert!(!directory.join("state.json.tmp").exists());
    }
    {
        let reopened = DurableTransactionStore::open(&directory).expect("durable store reopens");
        assert_eq!(reopened.committed().generation, 1);
        assert_eq!(reopened.journal().len(), 1);
        assert_eq!(
            reopened.journal()[0].provenance,
            "durable pre-network intent"
        );
    }
    assert!(!directory.join("state.lock").exists());
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn durable_transaction_store_rejects_stale_intent_before_journal_append() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-stale-intent-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    {
        let initial = StateRecord::new(LifecycleState::Ready);
        let mut store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        let stale = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest.clone(),
                "must not persist when stale",
            )
            .expect("stale candidate staged before intervening commit");
        let current = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest,
                "current intent",
            )
            .expect("current transaction stages");
        store
            .commit(current, ProjectionWrite::Success)
            .expect("current commit succeeds");
        assert!(matches!(
            store.commit(stale, ProjectionWrite::Success),
            Err(csdlc_v3::storage::StoreError::StaleWriter { .. })
        ));
        let intent_journal =
            fs::read_to_string(directory.join("intents.jsonl")).expect("intent journal exists");
        assert!(intent_journal.contains("current intent"));
        assert!(!intent_journal.contains("must not persist when stale"));
    }
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn durable_open_fails_closed_on_interrupted_intent() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-interrupted-intent-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    let initial = StateRecord::new(LifecycleState::Ready);
    {
        let store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        drop(store);
    }
    fs::write(
        directory.join("intents.jsonl"),
        format!(
            "{{\"schema\":\"csdlc.v3.transaction_intent.v1\",\"expected_generation\":{},\"expected_digest\":\"{}\",\"command\":\"Bind\",\"provenance\":\"interrupted before state replacement\"}}\n",
            initial.generation, initial.digest
        ),
    )
    .expect("write interrupted intent fixture");
    match DurableTransactionStore::open(&directory) {
        Err(csdlc_v3::storage::StoreError::RecoveryRequired(_)) => {}
        Err(error) => panic!("unexpected reopen error: {error:?}"),
        Ok(_) => panic!("interrupted intent must not reopen as a usable store"),
    }
    assert!(!directory.join("state.lock").exists());
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn durable_failed_state_replacement_does_not_advance_live_memory() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-state-write-failure-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    let initial = StateRecord::new(LifecycleState::Ready);
    {
        let mut store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        fs::create_dir(directory.join("state.json.tmp")).expect("block temp-file replacement");
        let transaction = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest.clone(),
                "state replacement will fail",
            )
            .expect("transaction stages");
        assert!(matches!(
            store.commit(transaction, ProjectionWrite::Success),
            Err(csdlc_v3::storage::StoreError::Io(_))
        ));
        assert_eq!(store.committed().generation, initial.generation);
        assert_eq!(store.committed().digest, initial.digest);
        let journal =
            fs::read_to_string(directory.join("intents.jsonl")).expect("intent journal exists");
        assert!(journal.contains("state replacement will fail"));
    }
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn durable_projection_failure_after_state_commit_is_repair_authority() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-projection-failure-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    let initial = StateRecord::new(LifecycleState::Ready);
    {
        let mut store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        let transaction = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest,
                "projection writer fails after durable state",
            )
            .expect("transaction stages");
        let result = store
            .commit_then_project(transaction, |_| {
                Err(csdlc_v3::storage::StoreError::Io(
                    "projection write failed after commit".to_owned(),
                ))
            })
            .expect("projection failure is recorded as repair");
        let CommitResult::ProjectionRepairRequired(state) = result else {
            panic!("expected projection repair result");
        };
        assert!(state.projections_repair_required);
        assert!(store.committed().projections_repair_required);
        let state_json = fs::read_to_string(directory.join("state.json")).expect("state json");
        assert!(state_json.contains("\"projections_repair_required\":true"));
    }
    match DurableTransactionStore::open(&directory) {
        Err(csdlc_v3::storage::StoreError::RecoveryRequired(
            RecoveryRepair::RegenerateProjections,
        )) => {}
        Err(other) => panic!("unexpected open error: {other:?}"),
        Ok(_) => panic!("interrupted intent must fail closed"),
    }
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn durable_projection_success_is_repair_required_until_projection_is_written() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/csdlc-v3-durable-projection-crash-window-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&directory);
    let observed_repair_window = std::cell::Cell::new(false);
    let initial = StateRecord::new(LifecycleState::Ready);
    {
        let mut store =
            DurableTransactionStore::create(&directory, initial.clone()).expect("durable store");
        let transaction = store
            .begin(
                LifecycleCommand::Bind,
                &full_capabilities(),
                initial.generation,
                initial.digest,
                "projection writer observes repair-required durable state",
            )
            .expect("transaction stages");
        let result = store
            .commit_then_project(transaction, |projected_state| {
                assert!(!projected_state.projections_repair_required);
                let state_json = fs::read_to_string(directory.join("state.json"))
                    .expect("state json visible during projection write");
                observed_repair_window
                    .set(state_json.contains("\"projections_repair_required\":true"));
                Ok(())
            })
            .expect("projection success clears repair requirement");
        let CommitResult::Committed(state) = result else {
            panic!("expected committed result");
        };
        assert!(observed_repair_window.get());
        assert!(!state.projections_repair_required);
        assert!(!store.committed().projections_repair_required);
        let state_json = fs::read_to_string(directory.join("state.json")).expect("state json");
        assert!(state_json.contains("\"projections_repair_required\":false"));
    }
    let reopened = DurableTransactionStore::open(&directory).expect("clean store reopens");
    assert!(!reopened.committed().projections_repair_required);
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn recovery_classifies_interrupted_writes_without_losing_provenance() {
    let prior = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(prior.clone()).expect("valid initial digest");
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            prior.generation,
            prior.digest.clone(),
            "recovery provenance",
        )
        .expect("bind transaction stages");
    let committed = match store
        .commit(transaction, ProjectionWrite::FailAfterStateCommit)
        .expect("post-state projection failure still commits state")
    {
        CommitResult::ProjectionRepairRequired(state) => state,
        CommitResult::Committed(_) => panic!("expected projection repair"),
    };
    let intent = store.journal()[0].clone();
    assert_eq!(
        classify_recovery(RecoveryObservation::IntentWithoutCommit {
            prior: prior.clone(),
            intent: intent.clone()
        }),
        RecoveryClassification::PriorState(prior)
    );
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommittedProjectionMissing {
            state: committed.clone(),
            intent: intent.clone()
        }),
        RecoveryClassification::RepairRequired {
            state: committed.clone(),
            intent,
            repair: RecoveryRepair::RegenerateProjections
        }
    );
    assert_eq!(committed.audit[0].provenance, "recovery provenance");
}

#[test]
fn recovery_honors_projection_repair_flag_from_committed_state() {
    let prior = StateRecord::new(LifecycleState::Ready);
    let mut clean_store = TransactionStore::new(prior.clone()).expect("valid initial digest");
    let clean_transaction = clean_store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            prior.generation,
            prior.digest.clone(),
            "clean projection write",
        )
        .expect("bind transaction stages");
    let clean_committed = match clean_store
        .commit(clean_transaction, ProjectionWrite::Success)
        .expect("clean projection commit succeeds")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommittedProjectionMissing {
            state: clean_committed,
            intent: clean_store.journal()[0].clone()
        }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::RepairIntentMissing
        }
    );

    let prior = StateRecord::new(LifecycleState::Ready);
    let mut repair_store = TransactionStore::new(prior.clone()).expect("valid initial digest");
    let repair_transaction = repair_store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            prior.generation,
            prior.digest,
            "projection write failed",
        )
        .expect("bind transaction stages");
    let repair_committed = match repair_store
        .commit(repair_transaction, ProjectionWrite::FailAfterStateCommit)
        .expect("state commit survives projection failure")
    {
        CommitResult::ProjectionRepairRequired(state) => state,
        CommitResult::Committed(_) => panic!("expected projection repair"),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommitted {
            state: repair_committed.clone(),
            intent: repair_store.journal()[0].clone()
        }),
        RecoveryClassification::RepairRequired {
            state: repair_committed,
            intent: repair_store.journal()[0].clone(),
            repair: RecoveryRepair::RegenerateProjections
        }
    );
}

#[test]
fn recovery_rejects_no_commit_records_with_invalid_integrity() {
    let mut corrupt = StateRecord::new(LifecycleState::Ready);
    corrupt.digest = "v3:wrong-digest".to_owned();
    assert_eq!(
        classify_recovery(RecoveryObservation::NoIntent { state: corrupt }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::InvalidStateDigest
        }
    );

    let prior = StateRecord::new(LifecycleState::Ready);
    let intent = TransactionIntent {
        expected_generation: prior.generation,
        expected_digest: "v3:wrong-prior-digest".to_owned(),
        command: LifecycleCommand::Bind,
        provenance: "uncommitted intent".to_owned(),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::IntentWithoutCommit { prior, intent }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::IntentDoesNotMatchCommittedState
        }
    );
}

#[test]
fn recovery_rejects_committed_state_intent_mismatches() {
    let prior = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(prior.clone()).expect("valid initial digest");
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            prior.generation,
            prior.digest.clone(),
            "matched provenance",
        )
        .expect("bind transaction stages");
    let committed = match store
        .commit(transaction, ProjectionWrite::Success)
        .expect("state commit succeeds")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    let mismatched_intent = TransactionIntent {
        expected_generation: 99,
        expected_digest: prior.digest,
        command: LifecycleCommand::Publish,
        provenance: "different operation".to_owned(),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommitted {
            state: committed.clone(),
            intent: mismatched_intent
        }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::IntentDoesNotMatchCommittedState
        }
    );

    let intent = TransactionIntent {
        expected_generation: 0,
        expected_digest: "v3:wrong-prior-digest".to_owned(),
        command: LifecycleCommand::Bind,
        provenance: "matched provenance".to_owned(),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommitted {
            state: committed.clone(),
            intent
        }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::IntentDoesNotMatchCommittedState
        }
    );

    let mut corrupt_state = committed;
    corrupt_state.digest = "v3:wrong-state-digest".to_owned();
    let valid_intent = TransactionIntent {
        expected_generation: 0,
        expected_digest: StateRecord::new(LifecycleState::Ready).digest,
        command: LifecycleCommand::Bind,
        provenance: "matched provenance".to_owned(),
    };
    assert_eq!(
        classify_recovery(RecoveryObservation::StateCommitted {
            state: corrupt_state,
            intent: valid_intent
        }),
        RecoveryClassification::CorruptRecoveryInput {
            reason: RecoveryRejectReason::InvalidStateDigest
        }
    );
}

#[test]
fn transaction_commit_rechecks_cas_and_digest_binds_contents() {
    let initial = StateRecord::new(LifecycleState::Ready);
    assert_eq!(initial.digest.len(), "v3:".len() + 64);
    assert!(initial
        .digest
        .strip_prefix("v3:")
        .expect("v3 digest prefix")
        .chars()
        .all(|character| character.is_ascii_hexdigit()));
    let mut store = TransactionStore::new(initial.clone()).expect("valid initial digest");
    let first = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest.clone(),
            "first writer provenance",
        )
        .expect("first transaction stages");
    let second = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest.clone(),
            "second writer provenance",
        )
        .expect("second transaction stages from same snapshot");
    let committed = match store
        .commit(first, ProjectionWrite::Success)
        .expect("first commit succeeds")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    assert_eq!(
        store.commit(second, ProjectionWrite::Success),
        Err(csdlc_v3::storage::StoreError::StaleWriter {
            expected_generation: 0,
            actual_generation: 1
        })
    );

    let mut alternate = TransactionStore::new(initial.clone()).expect("valid initial digest");
    let alternate_transaction = alternate
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest,
            "different provenance",
        )
        .expect("alternate transaction stages");
    let alternate_committed = match alternate
        .commit(alternate_transaction, ProjectionWrite::Success)
        .expect("alternate commit succeeds")
    {
        CommitResult::Committed(state) => state,
        CommitResult::ProjectionRepairRequired(_) => panic!("unexpected projection repair"),
    };
    assert_ne!(
        committed.audit[0].provenance,
        alternate_committed.audit[0].provenance
    );
    assert_ne!(committed.digest, alternate_committed.digest);
}

#[test]
fn transaction_store_rejects_tampered_record_digest_on_ingress() {
    let mut tampered = StateRecord::new(LifecycleState::Ready);
    tampered.state = LifecycleState::Bound;
    assert_eq!(
        TransactionStore::new(tampered),
        Err(csdlc_v3::storage::StoreError::InvalidRecordDigest)
    );
}

#[test]
fn adapter_invocations_are_argv_based_and_shell_strings_are_rejected() {
    assert!(CommandInvocation::new("git", ["status", "--short"]).is_ok());
    assert!(CommandInvocation::new("git status", ["--short"]).is_err());
    assert!(CommandInvocation::new("git", ["status && gh pr merge"]).is_err());
    assert!(CommandInvocation::new("sh", ["-c", "git status"]).is_err());
    assert!(CommandInvocation::new("/bin/sh", ["-c", "git status"]).is_err());
    assert!(CommandInvocation::new("./bash", ["-c", "git status"]).is_err());
    assert!(CommandInvocation::new("tools/pwsh", ["-c", "git status"]).is_err());
    assert!(CommandInvocation::new("cmd.exe", ["/C", "git status"]).is_err());
    assert!(CommandInvocation::new("tools/powershell.exe", ["git status"]).is_err());
    assert!(CommandInvocation::new("git", ["status", "$(cat secret)"]).is_err());
}

#[test]
fn adapter_outcomes_preserve_status_output_timeout_cancel_and_redaction() {
    assert_eq!(
        CommandInvocation::new("git", ["fetch", "token=abc123"]),
        Err(csdlc_v3::adapters::AdapterError::SecretArgumentRejected)
    );
    assert_eq!(
        CommandInvocation::new("gh", ["api", "--token", "abc123"]),
        Err(csdlc_v3::adapters::AdapterError::SecretArgumentRejected)
    );
    let resolver = StaticCredentialResolver::new("ADL_GITHUB_TOKEN_FILE", "/safe/token/path");
    let invocation = CommandInvocation::new("git", ["fetch", "origin"])
        .expect("argv invocation")
        .with_child_credential("ADL_GITHUB_TOKEN_FILE")
        .expect("child credential scoped");
    assert_eq!(
        invocation.credential_scope,
        CredentialScope::ChildProcessOnly {
            name: "ADL_GITHUB_TOKEN_FILE".to_owned()
        }
    );
    assert_eq!(
        invocation.child_credential_name(),
        Some("ADL_GITHUB_TOKEN_FILE")
    );
    let mut injector = RecordingCredentialInjector::default();
    invocation
        .inject_child_credential_for_process(&resolver, &mut injector)
        .expect("child credential injects only at process boundary");
    assert_eq!(injector.names, vec!["ADL_GITHUB_TOKEN_FILE".to_owned()]);
    assert_eq!(invocation.redacted_argv(), ["fetch", "origin"]);
    assert!(!format!("{invocation:?}").contains("/safe/token/path"));
    assert!(!format!("{resolver:?}").contains("/safe/token/path"));
    assert_eq!(
        CommandInvocation::new(
            "git",
            [
                "status",
                "--token",
                "abc123",
                "--password",
                "hunter2",
                "--api-key=also-secret",
                "TOKEN=upper-secret",
                "Authorization: Bearer secret",
                "https://user:token@example.test/path",
                "--client-secret",
                "separate-secret",
                "https://access-token@example.test/path",
                "repos",
            ],
        ),
        Err(csdlc_v3::adapters::AdapterError::SecretArgumentRejected)
    );
    let safe_invocation = CommandInvocation::new(
        "git",
        [
            "status",
            "--worktree",
            "/repo",
            "--pathspec-from-file",
            "repos",
        ],
    )
    .expect("safe argv invocation");
    assert_eq!(
        safe_invocation.redacted_argv(),
        [
            "status",
            "--worktree",
            "/repo",
            "--pathspec-from-file",
            "repos"
        ]
    );
    let mut adapter = FakeProcessAdapter::new(ProcessOutput {
        status: ProcessStatus::TimedOut,
        stdout: "partial".to_owned(),
        stderr: "still running".to_owned(),
        truncated: true,
    });
    let output = adapter.run(invocation);
    assert_eq!(output.status, ProcessStatus::TimedOut);
    assert_eq!(output.stdout, "partial");
    assert_eq!(output.stderr, "still running");
    assert!(output.truncated);

    let mut cancelled = FakeProcessAdapter::new(ProcessOutput {
        status: ProcessStatus::Cancelled,
        stdout: String::new(),
        stderr: "cancelled".to_owned(),
        truncated: false,
    });
    assert_eq!(
        cancelled
            .run(CommandInvocation::new("git", ["status"]).expect("argv invocation"))
            .status,
        ProcessStatus::Cancelled
    );
}

#[test]
fn real_process_adapter_injects_child_credentials_and_redacts_process_output() {
    let invocation = CommandInvocation::new("printenv", ["ADL_TEST_GITHUB_TOKEN"])
        .expect("printenv argv")
        .with_child_credential("ADL_TEST_GITHUB_TOKEN")
        .expect("safe credential name");
    let resolver = StaticCredentialResolver::new("ADL_TEST_GITHUB_TOKEN", "real-secret-value");
    let mut adapter = RealProcessAdapter::new(resolver);
    let output = adapter.run(invocation);
    assert_eq!(output.status, ProcessStatus::Exit(0));
    assert_eq!(output.stdout, "[REDACTED]\n");
    assert!(!output.stderr.contains("real-secret-value"));
    assert!(!output.truncated);
}

#[test]
fn real_process_adapter_fails_closed_on_missing_credentials_and_truncates_output() {
    let invocation = CommandInvocation::new("printenv", ["ADL_TEST_GITHUB_TOKEN"])
        .expect("printenv argv")
        .with_child_credential("ADL_TEST_GITHUB_TOKEN")
        .expect("safe credential name");
    let resolver = StaticCredentialResolver::new("OTHER_TOKEN", "real-secret-value");
    let mut adapter = RealProcessAdapter::new(resolver);
    let output = adapter.run(invocation);
    assert_eq!(output.status, ProcessStatus::Exit(126));
    assert_eq!(output.stderr, "credential resolution failed");

    let invocation = CommandInvocation::new("printenv", ["ADL_TEST_GITHUB_TOKEN"])
        .expect("printenv argv")
        .with_child_credential("ADL_TEST_GITHUB_TOKEN")
        .expect("safe credential name");
    let resolver = StaticCredentialResolver::new("ADL_TEST_GITHUB_TOKEN", "bad\"token");
    let mut adapter = RealProcessAdapter::new(resolver);
    let output = adapter.run(invocation);
    assert_eq!(output.status, ProcessStatus::Exit(126));
    assert_eq!(output.stderr, "credential resolution failed");

    let mut adapter = RealProcessAdapter::new(StaticCredentialResolver::new("UNUSED", "unused"))
        .with_max_output_bytes(3);
    let output = adapter.run(CommandInvocation::new("printf", ["abcdef"]).expect("printf argv"));
    assert_eq!(output.status, ProcessStatus::Exit(0));
    assert_eq!(output.stdout, "abc");
    assert!(output.truncated);
}

#[derive(Default)]
struct RecordingCredentialInjector {
    names: Vec<String>,
}

impl ChildCredentialInjector for RecordingCredentialInjector {
    fn inject_child_credential(&mut self, name: &str, value: &str) {
        assert_eq!(value, "/safe/token/path");
        self.names.push(name.to_owned());
    }
}

#[test]
fn adapter_branch_observation_never_authorizes_lifecycle_work() {
    let mut git = FakeGitAdapter::default();
    let observation = git.observe_branch(
        CommandInvocation::new("git", ["branch", "--show-current", "codex/502"])
            .expect("argv invocation"),
    );
    assert_eq!(observation.branch, "codex/502");
    assert!(!observation.authorizes_lifecycle);
}
