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

// PVF: deterministic local CPU/disk/process protocol qualification; required Gate A
// proof, no network/credentials. Process death proves lock release, not power-loss durability.
mod semantic_gate_a {
    use super::*;
    use csdlc_v3::commands::remote::{
        github_mutation_operation_digest, github_mutation_operation_marker, GithubMutation,
        GithubMutationIntent, GithubMutationRequest,
    };
    use csdlc_v3::lifecycle::semantic::{self, Facts, Outcome, SemanticCommand};
    use csdlc_v3::storage::semantic::{
        AcceptedIntentPlan, Admission, CommitOutcome, Digest, Error, IssueInputs, IssueKey,
        LocalChange, NativeWriterFenceGuard, Observation, PlanStep, ProjectionWriteProof,
        Publication, SemanticRoot, Snapshot, Validator,
    };
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{Duration, Instant};

    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        directory: PathBuf,
        root: SemanticRoot,
        key: IssueKey,
    }
    impl Fixture {
        fn new() -> Self {
            let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join(format!(
                    "semantic-gate-a-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(directory.join("repo/.git/objects")).unwrap();
            fs::write(directory.join("repo/.git/HEAD"), "ref: refs/heads/main\n").unwrap();
            fs::write(
                directory.join("repo/.git/config"),
                "[core]\nrepositoryformatversion = 0\n",
            )
            .unwrap();
            let root =
                SemanticRoot::from_git_common(directory.join("repo/.git"), "example/repo").unwrap();
            Self {
                directory,
                root,
                key: IssueKey::new("example/repo", 870).unwrap(),
            }
        }
        fn issue_dir(&self) -> PathBuf {
            self.directory
                .join("repo/.git/csdlc-v3/semantic/issues/870")
        }
        fn prepare(&self) -> Snapshot {
            match DurableTransactionStore::prepare_issue(&self.root, self.key.clone(), inputs())
                .unwrap()
            {
                CommitOutcome::Committed(s) => *s,
                _ => panic!("new issue must commit"),
            }
        }
        fn project(&self, snapshot: &Snapshot) -> ProjectionWriteProof {
            let path = self.root.projection_path(snapshot).unwrap();
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, snapshot.projection_bytes().unwrap()).unwrap();
            ProjectionWriteProof::verify(&self.root, snapshot).unwrap()
        }
        fn current(&self) -> Snapshot {
            match DurableTransactionStore::observe_issue(&self.root, &self.key).unwrap() {
                Observation::Current(s) => *s,
                other => panic!("unexpected {other:?}"),
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.directory);
        }
    }
    fn accepted_plan() -> AcceptedIntentPlan {
        AcceptedIntentPlan {
            schema: "csdlc.v3.intent_plan.v1".into(),
            slug: "semantic-owner".into(),
            cards: ["sip", "stp", "spp", "vpp", "srp", "sor"]
                .into_iter()
                .map(|k| {
                    (
                        k.into(),
                        serde_json::json!({"task":"typed task", "nested":[1, true]}),
                    )
                })
                .collect(),
            validators: vec![Validator {
                id: "transaction".into(),
                program: "cargo".into(),
                args: vec!["test".into()],
                success_marker: "test result: ok.".into(),
                timeout_seconds: 123,
            }],
            publication: Publication {
                base: "main".into(),
                title: "semantic owner".into(),
                body: "Closes #870".into(),
                draft: true,
            },
        }
    }
    fn inputs() -> IssueInputs {
        IssueInputs::new(
            "bounded semantic work".into(),
            accepted_plan(),
            vec![PlanStep {
                id: "implement".into(),
                acceptance: "semantic owner".into(),
            }],
            None,
            Digest::authority(b"authority fixture"),
        )
        .unwrap()
    }
    fn legacy_inputs(worktree: &Path) -> IssueInputs {
        let mut plan = accepted_plan();
        for card in plan.cards.values_mut() {
            card["branch"] = "codex/870-semantic-owner".into();
            card["worktree"] = worktree.to_string_lossy().into_owned().into();
        }
        IssueInputs::new(
            "bounded semantic work".into(),
            plan,
            vec![PlanStep {
                id: "implement".into(),
                acceptance: "semantic owner".into(),
            }],
            None,
            Digest::authority(b"authority fixture"),
        )
        .unwrap()
    }
    fn admission(snapshot: &Snapshot) -> Admission {
        Admission::new(
            snapshot.key().clone(),
            snapshot.version().clone(),
            snapshot.inputs().authority().clone(),
        )
    }
    fn cards(text: &str) -> LocalChange {
        let mut cards = accepted_plan().cards;
        cards.insert("stp".into(), serde_json::json!({"task":text}));
        LocalChange::AmendCards(cards)
    }
    fn inventory(path: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        let mut out = BTreeMap::new();
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                let p = entry.unwrap().path();
                if p.is_dir() {
                    out.extend(inventory(&p));
                } else {
                    out.insert(p.clone(), fs::read(p).unwrap());
                }
            }
        }
        out
    }

    #[test]
    fn semantic_schema_canonical_order_duplicate_unknown_and_tamper() {
        let fixture = Fixture::new();
        let snapshot = fixture.prepare();
        let bytes = snapshot.canonical_bytes().unwrap();
        assert_eq!(Snapshot::from_bytes(&bytes).unwrap(), snapshot);
        let original: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let mut reversed = serde_json::Map::new();
        for (k, v) in original.as_object().unwrap().iter().rev() {
            reversed.insert(k.clone(), v.clone());
        }
        assert_eq!(
            Snapshot::from_bytes(&serde_json::to_vec(&reversed).unwrap())
                .unwrap()
                .canonical_bytes()
                .unwrap(),
            bytes
        );
        for field in ["schema", "unknown"] {
            let mut changed = original.clone();
            changed[field] = serde_json::json!("unrecognized");
            assert!(Snapshot::from_bytes(&serde_json::to_vec(&changed).unwrap()).is_err());
        }
        let text = String::from_utf8(bytes.clone()).unwrap();
        let duplicate = text.replacen("{", "{\"schema\":\"csdlc.v3.semantic_commit.v1\",", 1);
        assert!(Snapshot::from_bytes(duplicate.as_bytes()).is_err());
        let mut changed = original.clone();
        changed["payload"]["inputs"]["intent"] = serde_json::json!("tampered");
        assert_eq!(
            Snapshot::from_bytes(&serde_json::to_vec(&changed).unwrap()),
            Err(Error::InvalidDigest)
        );
        let mut changed = original;
        changed["payload"]["key"]["issue"] = serde_json::json!(0);
        assert!(Snapshot::from_bytes(&serde_json::to_vec(&changed).unwrap()).is_err());
        assert!(snapshot
            .version()
            .digest()
            .as_str()
            .starts_with("semantic-state-v1:"));
        assert!(snapshot
            .inputs_version()
            .digest()
            .as_str()
            .starts_with("semantic-input-v1:"));
        assert_ne!(
            snapshot.version().digest(),
            snapshot.inputs_version().digest()
        );
    }

    #[test]
    fn semantic_create_only_and_observation_do_not_repair_or_mutate() {
        let fixture = Fixture::new();
        let before = inventory(&fixture.directory);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::Absent
        );
        assert_eq!(before, inventory(&fixture.directory));
        let prepared = fixture.prepare();
        assert!(DurableTransactionStore::create(
            fixture.issue_dir(),
            StateRecord::new(LifecycleState::Ready)
        )
        .is_err());
        assert!(DurableTransactionStore::open(fixture.issue_dir()).is_err());
        let before = inventory(&fixture.directory);
        let repeated =
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), inputs())
                .unwrap();
        let CommitOutcome::Unchanged(repeated) = repeated else {
            panic!("identical ready preparation must be unchanged");
        };
        assert_eq!(repeated.version(), prepared.version());
        fixture.current();
        assert_eq!(before, inventory(&fixture.directory));
        fs::write(
            fixture.issue_dir().join("current.next"),
            b"interrupted pointer",
        )
        .unwrap();
        let before = inventory(&fixture.directory);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::RecoveryRequired
        );
        assert_eq!(before, inventory(&fixture.directory));
    }

    #[test]
    fn semantic_projection_acknowledgement_rechecks_exact_view() {
        let fixture = Fixture::new();
        let snapshot = fixture.prepare();
        assert!(ProjectionWriteProof::verify(&fixture.root, &snapshot).is_err());
        let proof = fixture.project(&snapshot);
        fs::write(
            fixture.root.projection_path(&snapshot).unwrap(),
            b"tampered view",
        )
        .unwrap();
        let before = inventory(&fixture.directory);
        assert!(DurableTransactionStore::commit_issue_local(
            &fixture.root,
            admission(&snapshot),
            LocalChange::AcknowledgeProjection(proof)
        )
        .is_err());
        assert_eq!(before, inventory(&fixture.directory));
        assert_eq!(fixture.current().version(), snapshot.version());
    }

    #[cfg(unix)]
    #[test]
    fn semantic_symlink_store_is_refused_without_following_writer() {
        let fixture = Fixture::new();
        let parent = fixture.directory.join("repo/.git/csdlc-v3/semantic");
        fs::create_dir_all(&parent).unwrap();
        let external = fixture.directory.join("external");
        fs::create_dir(&external).unwrap();
        std::os::unix::fs::symlink(&external, parent.join("issues")).unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key),
            Err(Error::UnsafePath)
        );
        assert_eq!(
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), inputs()),
            Err(Error::UnsafePath)
        );
        assert_eq!(fs::read_dir(external).unwrap().count(), 0);
    }

    // PVF: deterministic local owner-contract regression; tiny filesystem fixtures;
    // required #1046 publication gate; no remote service or runtime proof.
    #[test]
    fn issue_1046_publication_amendment_preserves_history_and_rejects_stale_or_invalid() {
        let fixture = Fixture::new();
        let first = fixture.prepare();
        let before = inventory(&fixture.issue_dir());
        let mut publication = first.inputs().publication().clone();
        publication.body = "Closes #870\n\nCorrected publication body".into();
        DurableTransactionStore::commit_issue_local(
            &fixture.root,
            admission(&first),
            LocalChange::AmendPublication(publication.clone()),
        )
        .unwrap();
        let current = fixture.current();
        assert_eq!(current.inputs().publication(), &publication);
        assert_ne!(current.inputs_version(), first.inputs_version());
        let projection: serde_json::Value =
            serde_json::from_slice(&current.projection_bytes().unwrap()).unwrap();
        for surface in ["proof", "review", "publication"] {
            assert!(projection["invalidations"]
                .as_array()
                .unwrap()
                .contains(&surface.into()));
        }
        let mut retained = 0;
        for (path, bytes) in before {
            if path.to_string_lossy().contains("/commits/")
                || path.to_string_lossy().contains("/intents/")
            {
                retained += 1;
                assert_eq!(fs::read(fixture.issue_dir().join(path)).unwrap(), bytes);
            }
        }
        assert!(retained > 0);
        assert_eq!(
            DurableTransactionStore::commit_issue_local(
                &fixture.root,
                admission(&first),
                LocalChange::AmendPublication(publication.clone())
            ),
            Err(Error::StaleVersion)
        );
        let stable = inventory(&fixture.directory);
        for case in ["inline", "foreign", "base", "title"] {
            let mut bad = publication.clone();
            match case {
                "inline" => bad.body = "Description. Closes #870".into(),
                "foreign" => bad.body = "Closes #870\nFixes #871".into(),
                "base" => bad.base = "different".into(),
                _ => bad.title = " ".into(),
            }
            assert!(
                DurableTransactionStore::commit_issue_local(
                    &fixture.root,
                    admission(&current),
                    LocalChange::AmendPublication(bad)
                )
                .is_err(),
                "{case}"
            );
            assert_eq!(stable, inventory(&fixture.directory), "{case}");
        }
    }

    #[test]
    fn semantic_bookkeeping_preserves_evidence_inputs_and_amendments_advance_them() {
        let fixture = Fixture::new();
        let first = fixture.prepare();
        DurableTransactionStore::commit_issue_local(
            &fixture.root,
            admission(&first),
            LocalChange::AcknowledgeProjection(fixture.project(&first)),
        )
        .unwrap();
        let second = fixture.current();
        assert_eq!(second.version().generation(), 2);
        assert_eq!(first.inputs_version(), second.inputs_version());
        assert!(!second.projection_required());
        assert_eq!(
            first.projection_bytes().unwrap(),
            second.projection_bytes().unwrap()
        );
        // No rewrite after acknowledgement: G's content is also the view of G+1.
        let second_proof = ProjectionWriteProof::verify(&fixture.root, &second).unwrap();
        let before = inventory(&fixture.directory);
        assert!(matches!(
            DurableTransactionStore::commit_issue_local(
                &fixture.root,
                admission(&second),
                LocalChange::AcknowledgeProjection(second_proof)
            )
            .unwrap(),
            CommitOutcome::Unchanged(_)
        ));
        assert_eq!(before, inventory(&fixture.directory));
        DurableTransactionStore::commit_issue_local(
            &fixture.root,
            admission(&second),
            cards("changed typed task"),
        )
        .unwrap();
        let third = fixture.current();
        assert_eq!(third.version().generation(), 3);
        assert_eq!(third.inputs_version().revision(), 2);
        assert_ne!(
            third.inputs_version().digest(),
            second.inputs_version().digest()
        );
        assert!(third.projection_required());
        assert_eq!(
            DurableTransactionStore::commit_issue_local(
                &fixture.root,
                admission(&second),
                cards("stale")
            ),
            Err(Error::StaleVersion)
        );
    }

    #[test]
    fn semantic_legacy_residue_refuses_conversion_and_foreign_identity() {
        let fixture = Fixture::new();
        fs::create_dir_all(
            fixture
                .directory
                .join("repo/.git/csdlc-v3/local/issues/870"),
        )
        .unwrap();
        let before = inventory(&fixture.directory);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::LegacyMigrationRequired
        );
        assert_eq!(
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), inputs()),
            Err(Error::LegacyMigrationRequired)
        );
        assert_eq!(before, inventory(&fixture.directory));
        assert_eq!(
            DurableTransactionStore::observe_issue(
                &fixture.root,
                &IssueKey::new("foreign/repo", 870).unwrap()
            ),
            Err(Error::WrongRepository)
        );
    }

    fn write_issue_1029_legacy_ready_fixture(fixture: &Fixture, worktree: &Path) {
        let issue_root = fixture
            .directory
            .join("repo/.git/csdlc-v3/local/issues/870");
        fs::create_dir_all(issue_root.join("cards")).unwrap();
        for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            fs::write(
                issue_root.join("cards").join(format!("{kind}.md")),
                "card\n",
            )
            .unwrap();
            fs::write(
                issue_root.join("cards").join(format!("{kind}.values.json")),
                "{}\n",
            )
            .unwrap();
        }
        let mut index = serde_json::json!({
            "schema":"csdlc.v3.local_state.v1","issue":870,
            "repository":"example/repo","phase":"ready","generation":4,
            "branch":"codex/870-semantic-owner","worktree":worktree,
        });
        let mut hasher = blake3::Hasher::new();
        hasher.update(&serde_json::to_vec(&index).unwrap());
        for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            for suffix in ["values.json", "md"] {
                hasher.update(
                    &fs::read(issue_root.join("cards").join(format!("{kind}.{suffix}"))).unwrap(),
                );
            }
        }
        index["digest"] = hasher.finalize().to_hex().to_string().into();
        fs::write(
            issue_root.join("index.json"),
            serde_json::to_vec(&index).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn issue_1029_explicit_fenced_prepare_preserves_valid_unbound_legacy_record() {
        let fixture = Fixture::new();
        let worktree = fixture.directory.join("missing-bound-worktree");
        write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
        let issue_root = fixture
            .directory
            .join("repo/.git/csdlc-v3/local/issues/870");
        let before = inventory(&issue_root);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::LegacyMigrationRequired
        );
        let fence =
            NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870]).unwrap();
        let committed = DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
            &fixture.root,
            fixture.key.clone(),
            legacy_inputs(&worktree),
            &fence,
        )
        .unwrap();
        assert!(matches!(committed, CommitOutcome::Committed(_)));
        assert_eq!(before, inventory(&issue_root));
        match DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap() {
            Observation::Current(snapshot) => assert!(snapshot.projection_required()),
            other => panic!("unexpected post-prepare observation: {other:?}"),
        }
    }

    #[test]
    fn issue_1029_fenced_prepare_accepts_valid_completed_local_and_remote_receipts() {
        for replay in [false, true] {
            let fixture = Fixture::new();
            let worktree = fixture.directory.join("missing-bound-worktree");
            write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
            let request = GithubMutationRequest {
                repository: "example/repo".into(),
                issue: 870,
                pull_request: None,
                cutover_issue: None,
                operator_approval: None,
                expected_head_sha: "f".repeat(40),
                credential_names: vec!["GITHUB_TOKEN".into()],
                recovery: None,
                mutation: GithubMutation::IssueEdit {
                    title: None,
                    body: Some("updated body".into()),
                    labels: None,
                    assignees: None,
                    milestone: None,
                },
            };
            let operation = github_mutation_operation_digest(&request);
            let marker = github_mutation_operation_marker(&operation);
            let intent = GithubMutationIntent {
                schema: "csdlc.v3.github_mutation_intent.v1".into(),
                operation_digest: operation.clone(),
                operation_marker: marker.clone(),
                authority_selector_digest: "a".repeat(64),
                request,
                adapter: "github-api-operational".into(),
                resolved_edit: None,
                resolved_ready_target: None,
            };
            let stable_digest = |values: &[&str]| {
                let mut hasher = blake3::Hasher::new();
                for value in values {
                    hasher.update(value.as_bytes());
                    hasher.update(b"\0");
                }
                hasher.finalize().to_hex().to_string()
            };
            let intent_digest = stable_digest(&[
                &intent.schema,
                &operation,
                &marker,
                &intent.authority_selector_digest,
                &intent.adapter,
            ]);
            let completed = fixture.directory.join(format!(
                "repo/.git/csdlc-v3/local/transactions/completed/870/edit-{operation}.json"
            ));
            fs::create_dir_all(completed.parent().unwrap()).unwrap();
            fs::write(
            &completed,
            serde_json::to_vec(&serde_json::json!({
                "schema":"csdlc.v3.local_mutation_completion.v1","issue":870,
                "route":"edit","request_digest":operation,
                "result":{"route":"edit","issue":870,"mutated":true,"phase":"ready",
                    "generation":2,"digest":"e".repeat(64),"next_route":"validate","findings":[]}
            }))
            .unwrap(),
        )
        .unwrap();
            let receipt = fixture.directory.join(format!(
                "repo/.git/csdlc-v3/remote/mutations/{operation}.json"
            ));
            fs::create_dir_all(receipt.parent().unwrap()).unwrap();
            let intent_path = fixture.directory.join(format!(
                "repo/.git/csdlc-v3/remote/intents/{operation}.json"
            ));
            fs::create_dir_all(intent_path.parent().unwrap()).unwrap();
            fs::write(&intent_path, serde_json::to_vec(&intent).unwrap()).unwrap();
            fs::write(
                &receipt,
                serde_json::to_vec(&serde_json::json!({
                    "schema":"csdlc.v3.github_mutation_receipt.v2",
                    "repository":"example/repo","issue":870,"pull_request":null,
                    "expected_head_sha":"f".repeat(40),"operation_digest":operation,
                    "response_digest":null,"readback_digest":"1".repeat(64),
                    "intent_digest":intent_digest,"reconciliation_digest":"3".repeat(64),
                    "adapter":"github-api-operational","authenticated":true,
                    "idempotent_replay":replay
                }))
                .unwrap(),
            )
            .unwrap();
            let completed_before = fs::read(&completed).unwrap();
            let receipt_before = fs::read(&receipt).unwrap();
            let intent_before = fs::read(&intent_path).unwrap();
            let fence =
                NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                    .unwrap();
            assert!(matches!(
                DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                    &fixture.root,
                    fixture.key.clone(),
                    legacy_inputs(&worktree),
                    &fence,
                )
                .unwrap(),
                CommitOutcome::Committed(_)
            ));
            assert_eq!(completed_before, fs::read(completed).unwrap());
            assert_eq!(receipt_before, fs::read(receipt).unwrap());
            assert_eq!(intent_before, fs::read(intent_path).unwrap());
        }
    }

    fn write_retained_ready_edit_completion(
        fixture: &Fixture,
        generation: u64,
        request_byte: char,
        result_digest: &str,
    ) -> PathBuf {
        let request_digest = request_byte.to_string().repeat(64);
        let path = fixture.directory.join(format!(
            "repo/.git/csdlc-v3/local/transactions/completed/870/edit-{request_digest}.json"
        ));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(
            &path,
            serde_json::to_vec(&serde_json::json!({
                "schema":"csdlc.v3.local_mutation_completion.v1",
                "issue":870,"route":"edit","request_digest":request_digest,
                "result":{"route":"edit","issue":870,"mutated":true,"phase":"ready",
                    "generation":generation,"digest":result_digest,
                    "next_route":"validate","findings":[]}
            }))
            .unwrap(),
        )
        .unwrap();
        path
    }

    #[test]
    fn issue_1036_fenced_prepare_accepts_authentic_creation_receipt_census() {
        let fixture = Fixture::new();
        let worktree = fixture.directory.join("missing-bound-worktree");
        write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
        let issue_root = fixture
            .directory
            .join("repo/.git/csdlc-v3/local/issues/870");
        let index: serde_json::Value =
            serde_json::from_slice(&fs::read(issue_root.join("index.json")).unwrap()).unwrap();
        write_retained_ready_edit_completion(&fixture, 2, 'b', &"d".repeat(64));
        write_retained_ready_edit_completion(
            &fixture,
            index["generation"].as_u64().unwrap(),
            'c',
            index["digest"].as_str().unwrap(),
        );

        let (creation_intent, creation_receipt) =
            write_repository_scoped_creation_receipt(&fixture, 870);
        let (unrelated_intent, unrelated_receipt) =
            write_repository_scoped_creation_receipt(&fixture, 871);
        let merges = fixture.directory.join("repo/.git/csdlc-v3/remote/merges");
        fs::create_dir_all(&merges).unwrap();
        fs::write(
            merges.join("unrelated.intent.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema":"csdlc.v3.merge_intent.v1",
                "request":{"repository":"example/repo","issue":999}
            }))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            merges.join("unrelated.response.json"),
            serde_json::to_vec(&serde_json::json!({"retained":"sidecar"})).unwrap(),
        )
        .unwrap();

        let completed = fixture
            .directory
            .join("repo/.git/csdlc-v3/local/transactions/completed/870");
        let issue_before = inventory(&issue_root);
        let completed_before = inventory(&completed);
        let remote = fixture.directory.join("repo/.git/csdlc-v3/remote");
        let remote_before = inventory(&remote);
        let exact_creation_bytes = [
            fs::read(&creation_intent).unwrap(),
            fs::read(&creation_receipt).unwrap(),
            fs::read(&unrelated_intent).unwrap(),
            fs::read(&unrelated_receipt).unwrap(),
        ];
        let fence =
            NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870]).unwrap();
        assert!(matches!(
            DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                &fixture.root,
                fixture.key.clone(),
                legacy_inputs(&worktree),
                &fence,
            )
            .unwrap(),
            CommitOutcome::Committed(_)
        ));
        assert_eq!(issue_before, inventory(&issue_root));
        assert_eq!(completed_before, inventory(&completed));
        assert_eq!(remote_before, inventory(&remote));
        assert_eq!(exact_creation_bytes[0], fs::read(creation_intent).unwrap());
        assert_eq!(exact_creation_bytes[1], fs::read(creation_receipt).unwrap());
        assert_eq!(exact_creation_bytes[2], fs::read(unrelated_intent).unwrap());
        assert_eq!(
            exact_creation_bytes[3],
            fs::read(unrelated_receipt).unwrap()
        );
    }

    #[test]
    fn issue_1036_fenced_prepare_rejects_damaged_creation_receipt_census() {
        for tamper in [
            "intent_operation_digest",
            "intent_operation_marker",
            "intent_request",
            "receipt_operation_digest",
            "receipt_intent_digest",
            "receipt_filename",
        ] {
            let fixture = Fixture::new();
            let worktree = fixture.directory.join("missing-bound-worktree");
            write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
            let (intent_path, receipt_path) =
                write_repository_scoped_creation_receipt(&fixture, 870);
            let path = if tamper.starts_with("intent_") {
                &intent_path
            } else {
                &receipt_path
            };
            if tamper == "receipt_filename" {
                fs::rename(
                    &receipt_path,
                    receipt_path.parent().unwrap().join("wrong.json"),
                )
                .unwrap();
            } else {
                let mut value: serde_json::Value =
                    serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
                match tamper {
                    "intent_operation_digest" | "receipt_operation_digest" => {
                        value["operation_digest"] = serde_json::json!("cross-linked")
                    }
                    "intent_operation_marker" => {
                        value["operation_marker"] = serde_json::json!("<!-- wrong -->")
                    }
                    "intent_request" => {
                        value["request"]["expected_head_sha"] = serde_json::json!("other")
                    }
                    "receipt_intent_digest" => value["intent_digest"] = serde_json::json!("wrong"),
                    _ => unreachable!(),
                }
                fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            let fence =
                NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                    .unwrap();
            let before = inventory(&fixture.directory);
            assert_eq!(
                DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                    &fixture.root,
                    fixture.key.clone(),
                    legacy_inputs(&worktree),
                    &fence,
                ),
                Err(Error::RecoveryRequired),
                "{tamper}"
            );
            assert_eq!(before, inventory(&fixture.directory), "{tamper}");
        }
    }

    #[test]
    fn issue_1029_merge_sidecars_are_scoped_through_their_retained_intent() {
        for suffix in ["target", "input", "response", "dispatch-prestate"] {
            for case in [
                "unrelated",
                "same_issue",
                "missing_intent",
                "wrong_repository",
            ] {
                if case == "wrong_repository" && suffix != "target" {
                    continue;
                }
                let fixture = Fixture::new();
                let worktree = fixture.directory.join("missing-bound-worktree");
                write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
                let remote = fixture.directory.join("repo/.git/csdlc-v3/remote/merges");
                fs::create_dir_all(&remote).unwrap();
                if case != "missing_intent" {
                    fs::write(remote.join("retained.intent.json"), serde_json::to_vec(
                        &serde_json::json!({"schema":"csdlc.v3.merge_intent.v1",
                            "request":{"repository":"example/repo", "issue":if case == "same_issue" {870} else {999}}})
                    ).unwrap()).unwrap();
                }
                let value = if suffix == "target" {
                    serde_json::json!({"schema":"csdlc.v3.merge_target.v1",
                        "operation_digest":"retained", "repository":if case == "wrong_repository" {"foreign/repo"} else {"example/repo"}})
                } else {
                    serde_json::json!({"retained":"sidecar"})
                };
                fs::write(
                    remote.join(format!("retained.{suffix}.json")),
                    serde_json::to_vec(&value).unwrap(),
                )
                .unwrap();
                let fence =
                    NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                        .unwrap();
                let before = inventory(&fixture.directory);
                let remote_before = inventory(&remote);
                let result =
                    DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                        &fixture.root,
                        fixture.key.clone(),
                        legacy_inputs(&worktree),
                        &fence,
                    );
                let allowed = case == "unrelated";
                assert_eq!(result.is_ok(), allowed, "{suffix}: {case}: {result:?}");
                assert_eq!(remote_before, inventory(&remote));
                if !allowed {
                    assert_eq!(before, inventory(&fixture.directory));
                }
            }
        }
    }

    #[test]
    fn issue_1029_fenced_prepare_rejects_pending_or_bound_legacy_record() {
        for forbidden in ["transactions/870.json", "bindings/870.json"] {
            let fixture = Fixture::new();
            let worktree = fixture.directory.join("missing-bound-worktree");
            write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
            let path = fixture
                .directory
                .join("repo/.git/csdlc-v3/local")
                .join(forbidden);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, "pending\n").unwrap();
            let fence =
                NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                    .unwrap();
            let before = inventory(&fixture.directory);
            assert_eq!(
                DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                    &fixture.root,
                    fixture.key.clone(),
                    legacy_inputs(&worktree),
                    &fence,
                ),
                Err(Error::PendingOperation),
                "{forbidden}"
            );
            assert_eq!(before, inventory(&fixture.directory));
        }
    }

    #[test]
    fn issue_1029_fenced_prepare_rejects_stale_digest_and_wrong_identity() {
        for case in ["stale_digest", "wrong_repository", "existing_worktree"] {
            let fixture = Fixture::new();
            let worktree = fixture.directory.join("missing-bound-worktree");
            write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
            let issue_root = fixture
                .directory
                .join("repo/.git/csdlc-v3/local/issues/870");
            match case {
                "stale_digest" => fs::write(issue_root.join("cards/sip.md"), "changed\n").unwrap(),
                "wrong_repository" => {
                    let path = issue_root.join("index.json");
                    let mut index: serde_json::Value =
                        serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                    index["repository"] = "foreign/repo".into();
                    fs::write(path, serde_json::to_vec(&index).unwrap()).unwrap();
                }
                "existing_worktree" => fs::create_dir_all(&worktree).unwrap(),
                _ => unreachable!(),
            }
            let fence =
                NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                    .unwrap();
            let before = inventory(&fixture.directory);
            assert!(
                DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                    &fixture.root,
                    fixture.key.clone(),
                    legacy_inputs(&worktree),
                    &fence,
                )
                .is_err(),
                "{case}"
            );
            assert_eq!(before, inventory(&fixture.directory), "{case}");
        }
    }

    #[test]
    fn issue_1029_fenced_prepare_rejects_remote_linked_and_damaged_completion_residue() {
        for case in [
            "remote_pending",
            "orphan_remote_receipt",
            "tampered_remote_intent",
            "linked_residue",
            "damaged_completion",
        ] {
            let fixture = Fixture::new();
            let worktree = fixture.directory.join("missing-bound-worktree");
            write_issue_1029_legacy_ready_fixture(&fixture, &worktree);
            match case {
                "remote_pending" => {
                    let path = fixture
                        .directory
                        .join("repo/.git/csdlc-v3/remote/intents/pending.json");
                    fs::create_dir_all(path.parent().unwrap()).unwrap();
                    fs::write(
                        path,
                        serde_json::to_vec(&serde_json::json!({
                            "schema":"csdlc.v3.github_mutation_intent.v2",
                            "request":{"repository":"example/repo","issue":870}
                        }))
                        .unwrap(),
                    )
                    .unwrap();
                }
                "orphan_remote_receipt" | "tampered_remote_intent" => {
                    let operation = "d".repeat(64);
                    let mutation = fixture.directory.join(format!(
                        "repo/.git/csdlc-v3/remote/mutations/{operation}.json"
                    ));
                    fs::create_dir_all(mutation.parent().unwrap()).unwrap();
                    fs::write(
                        mutation,
                        serde_json::to_vec(&serde_json::json!({
                            "schema":"csdlc.v3.github_mutation_receipt.v2",
                            "repository":"example/repo","issue":870,"pull_request":null,
                            "expected_head_sha":"f".repeat(40),"operation_digest":operation,
                            "response_digest":null,"readback_digest":"1".repeat(64),
                            "intent_digest":"2".repeat(64),"reconciliation_digest":"3".repeat(64),
                            "adapter":"github-api-operational","authenticated":true,
                            "idempotent_replay":true
                        }))
                        .unwrap(),
                    )
                    .unwrap();
                    if case == "tampered_remote_intent" {
                        let intent = fixture.directory.join(format!(
                            "repo/.git/csdlc-v3/remote/intents/{operation}.json"
                        ));
                        fs::create_dir_all(intent.parent().unwrap()).unwrap();
                        fs::write(intent, b"{}\n").unwrap();
                    }
                }
                "linked_residue" => {
                    let registration = fixture.directory.join("repo/.git/worktrees/linked");
                    fs::create_dir_all(&registration).unwrap();
                    fs::write(
                        registration.join("gitdir"),
                        fixture
                            .directory
                            .join("linked/.git")
                            .to_string_lossy()
                            .as_bytes(),
                    )
                    .unwrap();
                    let residue = fixture
                        .directory
                        .join("linked/.csdlc/issues/870/index.json");
                    fs::create_dir_all(residue.parent().unwrap()).unwrap();
                    fs::write(residue, "damaged\n").unwrap();
                }
                "damaged_completion" => {
                    let path = fixture.directory.join(format!(
                        "repo/.git/csdlc-v3/local/transactions/completed/870/edit-{}.json",
                        "c".repeat(64)
                    ));
                    fs::create_dir_all(path.parent().unwrap()).unwrap();
                    fs::write(path, "{}\n").unwrap();
                }
                _ => unreachable!(),
            }
            let fence =
                NativeWriterFenceGuard::acquire(&fixture.directory.join("repo/.git"), [870])
                    .unwrap();
            let before = inventory(&fixture.directory);
            assert!(
                DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
                    &fixture.root,
                    fixture.key.clone(),
                    legacy_inputs(&worktree),
                    &fence,
                )
                .is_err(),
                "{case}"
            );
            assert_eq!(before, inventory(&fixture.directory), "{case}");
        }
    }

    #[test]
    fn semantic_native_residue_collision_matrix_is_read_only() {
        let residues = [
            "locks/870.lock",
            "issues/870/index.json",
            "prepared/issues/870/cards/sip.md",
            "transactions/870.json",
            "transactions/completed/870/edit-digest.json",
            "bindings/870.json",
            "issues/.issue-870-edit-digest.stage/intent-plan.json",
            "issues/.issue-870-edit-digest.backup/index.json",
            "issues/.issue-870.init-123-1/cards/sip.md",
            "v3/issues/870/terminal.json",
            "evidence/870/terminal-receipt.json",
            "archives/870-intent-digest/manifest.json",
            "archives/870-head-closeout/precleanup-audit.json",
        ];
        for base in ["repo/.git/csdlc-v3/local", "repo/.csdlc", "linked/.csdlc"] {
            for residue in residues {
                let fixture = Fixture::new();
                if base.starts_with("linked") {
                    let registration = fixture.directory.join("repo/.git/worktrees/linked");
                    fs::create_dir_all(&registration).unwrap();
                    fs::write(
                        registration.join("gitdir"),
                        fixture.directory.join("linked/.git").to_str().unwrap(),
                    )
                    .unwrap();
                }
                let path = fixture.directory.join(base).join(residue);
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, b"retained or interrupted native state").unwrap();
                let before = inventory(&fixture.directory);
                assert_eq!(
                    DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
                    Observation::LegacyMigrationRequired,
                    "{base}/{residue}"
                );
                assert_eq!(
                    DurableTransactionStore::prepare_issue(
                        &fixture.root,
                        fixture.key.clone(),
                        inputs()
                    ),
                    Err(Error::LegacyMigrationRequired),
                    "{base}/{residue}"
                );
                assert_eq!(before, inventory(&fixture.directory));
            }
        }
        let fixture = Fixture::new();
        fs::create_dir_all(
            fixture
                .directory
                .join("repo/.git/csdlc-v3/local/archives/8700-other"),
        )
        .unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::Absent
        );
    }

    #[test]
    fn semantic_intent_plan_roundtrips_native_contract_without_losing_fields() {
        let plan = accepted_plan();
        let bytes = serde_json::to_vec(&plan).unwrap();
        let native: csdlc_v3::application::intent::IntentPlan =
            serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            serde_json::to_value(&native).unwrap(),
            serde_json::to_value(&plan).unwrap()
        );
        let stored = IssueInputs::from_intent_plan_bytes(
            "intent".into(),
            &serde_json::to_vec(&native).unwrap(),
            inputs().plan().to_vec(),
            None,
            Digest::authority(b"authority fixture"),
        )
        .unwrap();
        assert_eq!(stored.accepted_plan(), &plan);
        assert_eq!(stored.validation()[0].timeout_seconds, 123);
        assert_eq!(stored.validation()[0].success_marker, "test result: ok.");
        assert_eq!(
            stored.cards()["stp"]["nested"],
            serde_json::json!([1, true])
        );
        assert_eq!(stored.slug(), "semantic-owner");
        assert!(stored.publication().draft);
        let fixture = Fixture::new();
        let CommitOutcome::Committed(snapshot) =
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), stored)
                .unwrap()
        else {
            panic!("prepared");
        };
        assert_eq!(fixture.current().inputs().accepted_plan(), &plan);
        assert_eq!(snapshot.audit_command(), SemanticCommand::Prepare);
        assert!(!snapshot.audit_identity().as_str().is_empty());
        assert!(snapshot.invalidations().is_empty());
    }

    #[test]
    fn semantic_remote_residue_is_repository_issue_scoped_and_ambiguous_fails_closed() {
        for (namespace, schema, request) in [
            ("intents", "csdlc.v3.github_mutation_intent.v1", true),
            ("mutations", "csdlc.v3.github_mutation_receipt.v2", false),
            ("recoveries", "csdlc.v3.github_mutation_recovery.v1", false),
            ("merges", "csdlc.v3.merge_intent.v1", true),
        ] {
            for (repository, issue, collision) in [
                ("example/repo", 870, true),
                ("other/repo", 870, false),
                ("example/repo", 871, false),
                ("example/repo", 0, false),
            ] {
                let fixture = Fixture::new();
                let directory = fixture
                    .directory
                    .join("repo/.git/csdlc-v3/remote")
                    .join(namespace);
                fs::create_dir_all(&directory).unwrap();
                let identity = serde_json::json!({"repository":repository,"issue":issue});
                let mut record = if request {
                    serde_json::json!({"request":identity})
                } else {
                    identity
                };
                record["schema"] = serde_json::json!(schema);
                fs::write(
                    directory.join("retained.json"),
                    serde_json::to_vec(&record).unwrap(),
                )
                .unwrap();
                let before = inventory(&fixture.directory);
                assert_eq!(
                    DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
                    if collision {
                        Observation::LegacyMigrationRequired
                    } else {
                        Observation::Absent
                    }
                );
                assert_eq!(before, inventory(&fixture.directory));
            }
        }
        for bytes in [
            b"{broken".as_slice(),
            br#"{"schema":"unknown","repository":"example/repo","issue":870}"#.as_slice(),
            br#"{"schema":"csdlc.v3.github_mutation_receipt.v2","repository":"example/repo"}"#
                .as_slice(),
        ] {
            let fixture = Fixture::new();
            let directory = fixture
                .directory
                .join("repo/.git/csdlc-v3/remote/mutations");
            fs::create_dir_all(&directory).unwrap();
            fs::write(directory.join("damaged.json"), bytes).unwrap();
            let before = inventory(&fixture.directory);
            assert_eq!(
                DurableTransactionStore::observe_issue(&fixture.root, &fixture.key),
                Err(Error::RecoveryRequired)
            );
            assert_eq!(
                DurableTransactionStore::prepare_issue(
                    &fixture.root,
                    fixture.key.clone(),
                    inputs()
                ),
                Err(Error::RecoveryRequired)
            );
            assert_eq!(before, inventory(&fixture.directory));
        }
    }

    #[test]
    fn repository_scoped_issue_creation_receipt_does_not_block_created_issue_preparation() {
        let fixture = Fixture::new();
        write_repository_scoped_creation_receipt(&fixture, 871);
        write_repository_scoped_creation_receipt(&fixture, 870);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::Absent
        );
        assert!(matches!(
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), inputs()),
            Ok(CommitOutcome::Committed(_))
        ));
    }

    #[test]
    fn repository_scoped_issue_creation_receipt_mismatches_fail_closed() {
        for tamper in [
            "intent_operation_digest",
            "intent_operation_marker",
            "intent_request",
            "receipt_operation_digest",
            "receipt_intent_digest",
            "receipt_filename",
        ] {
            let fixture = Fixture::new();
            let (intent_path, receipt_path) =
                write_repository_scoped_creation_receipt(&fixture, 870);
            let path = if tamper.starts_with("intent_") {
                &intent_path
            } else {
                &receipt_path
            };
            if tamper == "receipt_filename" {
                fs::rename(
                    &receipt_path,
                    receipt_path.parent().unwrap().join("wrong.json"),
                )
                .unwrap();
            } else {
                let mut value: serde_json::Value =
                    serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
                match tamper {
                    "intent_operation_digest" | "receipt_operation_digest" => {
                        value["operation_digest"] = serde_json::json!("cross-linked")
                    }
                    "intent_operation_marker" => {
                        value["operation_marker"] = serde_json::json!("<!-- wrong -->")
                    }
                    "intent_request" => {
                        value["request"]["expected_head_sha"] = serde_json::json!("other")
                    }
                    "receipt_intent_digest" => value["intent_digest"] = serde_json::json!("wrong"),
                    _ => unreachable!(),
                }
                fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            assert_eq!(
                DurableTransactionStore::observe_issue(&fixture.root, &fixture.key),
                Err(Error::RecoveryRequired),
                "{tamper}"
            );
            assert_eq!(
                DurableTransactionStore::prepare_issue(
                    &fixture.root,
                    fixture.key.clone(),
                    inputs()
                ),
                Err(Error::RecoveryRequired),
                "{tamper}"
            );
        }
    }

    fn write_repository_scoped_creation_receipt(
        fixture: &Fixture,
        assigned_issue: u64,
    ) -> (PathBuf, PathBuf) {
        use csdlc_v3::commands::remote::{
            github_mutation_operation_digest, GithubMutation, GithubMutationRequest,
        };

        let remote = fixture.directory.join("repo/.git/csdlc-v3/remote");
        fs::create_dir_all(remote.join("intents")).unwrap();
        fs::create_dir_all(remote.join("mutations")).unwrap();
        let request = GithubMutationRequest {
            repository: "example/repo".into(),
            issue: 0,
            pull_request: None,
            cutover_issue: None,
            operator_approval: None,
            expected_head_sha: "head".into(),
            credential_names: vec!["GITHUB_TOKEN".into()],
            recovery: None,
            mutation: GithubMutation::IssueCreate {
                title: format!("created {assigned_issue}"),
                body: "body".into(),
                labels: Vec::new(),
                assignees: Vec::new(),
                milestone: None,
            },
        };
        let operation_digest = github_mutation_operation_digest(&request);
        let operation_marker = format!("<!-- csdlc-v3-operation:{operation_digest} -->");
        let schema = "csdlc.v3.github_mutation_intent.v1";
        let authority = "authority";
        let adapter = "github-api-operational";
        let mut hasher = blake3::Hasher::new();
        for value in [
            schema,
            operation_digest.as_str(),
            operation_marker.as_str(),
            authority,
            adapter,
        ] {
            hasher.update(value.as_bytes());
            hasher.update(b"\0");
        }
        let intent_digest = hasher.finalize().to_hex().to_string();
        let intent_path = remote
            .join("intents")
            .join(format!("{operation_digest}.json"));
        fs::write(
            &intent_path,
            serde_json::to_vec(&serde_json::json!({
                "schema":schema,
                "operation_digest":operation_digest,
                "operation_marker":operation_marker,
                "authority_selector_digest":authority,
                "request":request,
                "adapter":adapter
            }))
            .unwrap(),
        )
        .unwrap();
        let receipt_path = remote
            .join("mutations")
            .join(format!("{operation_digest}.json"));
        fs::write(
            &receipt_path,
            serde_json::to_vec(&serde_json::json!({
                "schema":"csdlc.v3.github_mutation_receipt.v2",
                "repository":"example/repo",
                "issue":assigned_issue,
                "pull_request":null,
                "expected_head_sha":"head",
                "operation_digest":operation_digest,
                "response_digest":null,
                "readback_digest":"readback",
                "intent_digest":intent_digest,
                "reconciliation_digest":"reconciliation",
                "adapter":adapter,
                "authenticated":true,
                "idempotent_replay":true
            }))
            .unwrap(),
        )
        .unwrap();
        (intent_path, receipt_path)
    }

    #[test]
    fn semantic_stale_projection_after_ack_is_observational_repair() {
        let fixture = Fixture::new();
        let first = fixture.prepare();
        let proof = fixture.project(&first);
        DurableTransactionStore::commit_issue_local(
            &fixture.root,
            admission(&first),
            LocalChange::AcknowledgeProjection(proof),
        )
        .unwrap();
        let second = fixture.current();
        assert!(!second.projection_required());
        ProjectionWriteProof::verify(&fixture.root, &second).unwrap();
        fs::write(
            fixture.root.projection_path(&second).unwrap(),
            first.canonical_bytes().unwrap(),
        )
        .unwrap();
        let before = inventory(&fixture.directory);
        assert!(matches!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::ProjectionRepairRequired(_)
        ));
        assert_eq!(before, inventory(&fixture.directory));
    }

    #[test]
    fn semantic_held_native_issue_lock_is_collision_without_unlink() {
        let fixture = Fixture::new();
        let directory = fixture.directory.join("repo/.git/csdlc-v3/local/locks");
        fs::create_dir_all(&directory).unwrap();
        let path = directory.join("870.lock");
        let file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        fs2::FileExt::lock_exclusive(&file).unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::LegacyMigrationRequired
        );
        assert_eq!(
            DurableTransactionStore::prepare_issue(&fixture.root, fixture.key.clone(), inputs()),
            Err(Error::LegacyMigrationRequired)
        );
        assert!(path.is_file());
        fs2::FileExt::unlock(&file).unwrap();
        drop(file);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::LegacyMigrationRequired
        );
        fs::remove_file(path).unwrap();
        fs::write(directory.join("871.lock"), b"").unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::Absent
        );
    }

    #[test]
    fn semantic_unactivated_intent_is_observational_recovery() {
        let fixture = Fixture::new();
        let snapshot = fixture.prepare();
        let bytes = fs::read(fixture.issue_dir().join("intents/1.json")).unwrap();
        fs::write(fixture.issue_dir().join("intents/2.json"), bytes).unwrap();
        let before = inventory(&fixture.directory);
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key).unwrap(),
            Observation::RecoveryRequired
        );
        assert_eq!(
            DurableTransactionStore::commit_issue_local(
                &fixture.root,
                admission(&snapshot),
                cards("must not activate")
            ),
            Err(Error::RecoveryRequired)
        );
        assert_eq!(before, inventory(&fixture.directory));
    }

    #[test]
    fn semantic_reservation_and_failure_do_not_require_success_witnesses() {
        use LifecycleState::*;
        use SemanticCommand::*;
        let cases = [
            (Reviewed, Publish),
            (Published, MarkMergeReady),
            (MergeReady, RecordMerge),
            (Merged, Finish),
            (ClosedOut, RecordCleanup),
            (Ready, RecordInstall),
            (Ready, RecordCutover),
            (Ready, RecordRollback),
            (Ready, FinishWithoutPr),
        ];
        for (phase, command) in cases {
            let facts = Facts {
                current_proof: true,
                independent_review: true,
                merge_ready: true,
                terminal_receipt: true,
                administrative: true,
                no_pr_disposition: true,
                original_command: Some(command),
                ..Default::default()
            };
            assert_eq!(
                semantic::decide(Some(phase), Reserve, Outcome::Success, &facts)
                    .unwrap()
                    .phase,
                phase
            );
            assert_eq!(
                semantic::decide(Some(phase), command, Outcome::Failure, &facts)
                    .unwrap()
                    .phase,
                phase
            );
            if command != MarkMergeReady {
                assert!(semantic::decide(Some(phase), command, Outcome::Success, &facts).is_err());
            }
            assert!(
                semantic::decide(
                    Some(phase),
                    Reserve,
                    Outcome::Success,
                    &Facts {
                        original_command: Some(command),
                        ..Default::default()
                    }
                )
                .is_err()
                    || command == Finish
            );
        }
    }

    #[test]
    fn semantic_no_pr_policy_and_amendment_invalidation() {
        use SemanticCommand::*;
        let facts = Facts {
            terminal: true,
            no_pr_disposition: true,
            ..Default::default()
        };
        assert_eq!(
            semantic::decide(
                Some(LifecycleState::Ready),
                FinishWithoutPr,
                Outcome::Success,
                &facts
            )
            .unwrap()
            .phase,
            LifecycleState::ClosedOut
        );
        assert!(semantic::decide(
            Some(LifecycleState::Ready),
            Finish,
            Outcome::Success,
            &facts
        )
        .is_err());
        assert!(semantic::decide(
            Some(LifecycleState::Ready),
            FinishWithoutPr,
            Outcome::Success,
            &Facts::default()
        )
        .is_err());
        let amendment = semantic::decide(
            Some(LifecycleState::Reviewed),
            AmendPlan,
            Outcome::Success,
            &Facts::default(),
        )
        .unwrap();
        assert_eq!(amendment.phase, LifecycleState::Reviewed);
        assert!(amendment
            .invalidations
            .contains(&semantic::Invalidation::Proof));
        assert!(semantic::decide(
            Some(LifecycleState::ClosedOut),
            AmendPlan,
            Outcome::Success,
            &Facts::default()
        )
        .is_err());
    }

    fn wait_for(path: &Path) {
        let until = Instant::now() + Duration::from_secs(10);
        while !path.exists() {
            assert!(Instant::now() < until, "child handshake timeout");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    fn child(fixture: &Fixture, role: &str) -> std::process::Child {
        std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "semantic_gate_a::semantic_process_child",
                "--nocapture",
            ])
            .env("CSDLC_GATE_A_CHILD", role)
            .env("CSDLC_GATE_A_FIXTURE", &fixture.directory)
            .spawn()
            .unwrap()
    }
    #[test]
    fn semantic_process_child() {
        let Ok(role) = std::env::var("CSDLC_GATE_A_CHILD") else {
            return;
        };
        let dir = PathBuf::from(std::env::var_os("CSDLC_GATE_A_FIXTURE").unwrap());
        let root = SemanticRoot::from_git_common(dir.join("repo/.git"), "example/repo").unwrap();
        let key = IssueKey::new("example/repo", 870).unwrap();
        if role == "lock" {
            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(dir.join("repo/.git/csdlc-v3/semantic/issues/870/state.lock"))
                .unwrap();
            fs2::FileExt::lock_exclusive(&file).unwrap();
            fs::write(dir.join("lock-ready"), b"ready").unwrap();
            loop {
                std::thread::park();
            }
        }
        let Observation::Current(snapshot) =
            DurableTransactionStore::observe_issue(&root, &key).unwrap()
        else {
            panic!("current snapshot")
        };
        fs::write(dir.join(format!("{role}-ready")), b"ready").unwrap();
        wait_for(&dir.join("go"));
        let until = Instant::now() + Duration::from_secs(10);
        let result = loop {
            match DurableTransactionStore::commit_issue_local(
                &root,
                admission(&snapshot),
                cards(&role),
            ) {
                Err(Error::Busy) if Instant::now() < until => {
                    std::thread::sleep(Duration::from_millis(5))
                }
                other => break other,
            }
        };
        let label = match result {
            Ok(CommitOutcome::Committed(_)) => "committed",
            Err(Error::StaleVersion) => "stale",
            other => panic!("unexpected {other:?}"),
        };
        fs::write(dir.join(format!("{role}-result")), label).unwrap();
    }
    #[test]
    fn semantic_two_process_cas_and_crash_releasing_lock() {
        let fixture = Fixture::new();
        fixture.prepare();
        let mut holder = child(&fixture, "lock");
        wait_for(&fixture.directory.join("lock-ready"));
        assert_eq!(
            DurableTransactionStore::observe_issue(&fixture.root, &fixture.key),
            Err(Error::Busy)
        );
        holder.kill().unwrap();
        holder.wait().unwrap();
        fixture.current();
        let mut a = child(&fixture, "writer-a");
        let mut b = child(&fixture, "writer-b");
        wait_for(&fixture.directory.join("writer-a-ready"));
        wait_for(&fixture.directory.join("writer-b-ready"));
        fs::write(fixture.directory.join("go"), b"start").unwrap();
        assert!(a.wait().unwrap().success());
        assert!(b.wait().unwrap().success());
        let mut results = vec![
            fs::read_to_string(fixture.directory.join("writer-a-result")).unwrap(),
            fs::read_to_string(fixture.directory.join("writer-b-result")).unwrap(),
        ];
        results.sort();
        assert_eq!(results, ["committed", "stale"]);
        assert_eq!(fixture.current().version().generation(), 2);
    }

    #[test]
    fn semantic_readers_observe_only_coherent_commit_boundaries() {
        let fixture = Fixture::new();
        fixture.prepare();
        std::thread::scope(|scope| {
            let root = &fixture.root;
            let key = &fixture.key;
            let writer = scope.spawn(move || {
                for index in 0..12 {
                    loop {
                        let snapshot = match DurableTransactionStore::observe_issue(root, key) {
                            Ok(Observation::Current(s)) => s,
                            Err(Error::Busy) => {
                                std::thread::yield_now();
                                continue;
                            }
                            other => panic!("{other:?}"),
                        };
                        match DurableTransactionStore::commit_issue_local(
                            root,
                            admission(&snapshot),
                            cards(&format!("revision {index}")),
                        ) {
                            Ok(_) => break,
                            Err(Error::Busy) => std::thread::yield_now(),
                            other => panic!("{other:?}"),
                        }
                    }
                }
            });
            let mut observed = 0;
            while !writer.is_finished() {
                match DurableTransactionStore::observe_issue(root, key) {
                    Ok(Observation::Current(s)) => {
                        assert_eq!(
                            Snapshot::from_bytes(&s.canonical_bytes().unwrap()).unwrap(),
                            *s
                        );
                        observed += 1;
                    }
                    Err(Error::Busy) => std::thread::yield_now(),
                    other => panic!("incoherent observation {other:?}"),
                }
            }
            writer.join().unwrap();
            assert!(observed > 0);
        });
        assert_eq!(fixture.current().version().generation(), 13);
    }
}
