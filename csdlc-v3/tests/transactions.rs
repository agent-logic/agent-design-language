use csdlc_v3::adapters::{
    CommandInvocation, CredentialScope, FakeGitAdapter, FakeProcessAdapter, GitAdapter,
    ProcessAdapter, ProcessOutput, ProcessStatus,
};
use csdlc_v3::lifecycle::{
    decide, transition_matrix, Capability, CapabilitySet, LifecycleCommand, LifecycleState,
    ProjectionInvalidation, RejectReason, TransitionOutcome,
};
use csdlc_v3::storage::{
    classify_recovery, CommitResult, ProjectionWrite, RecoveryClassification, RecoveryObservation,
    RecoveryRejectReason, RecoveryRepair, StateRecord, TransactionIntent, TransactionStore,
};
use csdlc_v3::LIFECYCLE_KERNEL_PREDECESSORS;
use std::collections::BTreeSet;

fn full_capabilities() -> CapabilitySet {
    CapabilitySet::new([
        Capability::BoundTopology,
        Capability::ImplementationEvidence,
        Capability::IndependentExactHeadReview,
        Capability::PublicationLinkage,
        Capability::MergeReadinessEvidence,
        Capability::LiveMergeEvidence,
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
    let invocation = CommandInvocation::new("git", ["fetch", "token=abc123"])
        .expect("argv invocation")
        .with_child_credential("ADL_GITHUB_TOKEN_FILE");
    assert_eq!(
        invocation.credential_scope,
        CredentialScope::ChildProcessOnly {
            name: "ADL_GITHUB_TOKEN_FILE".to_owned()
        }
    );
    assert_eq!(invocation.redacted_argv(), ["fetch", "[REDACTED]"]);
    let split_secret = CommandInvocation::new(
        "gh",
        [
            "api",
            "--token",
            "abc123",
            "--password",
            "hunter2",
            "--api-key=also-secret",
            "TOKEN=upper-secret",
            "Authorization: Bearer secret",
            "https://user:token@example.test/path",
            "repos",
        ],
    )
    .expect("argv invocation");
    assert_eq!(
        split_secret.redacted_argv(),
        [
            "api",
            "--token",
            "[REDACTED]",
            "--password",
            "[REDACTED]",
            "[REDACTED]",
            "[REDACTED]",
            "[REDACTED]",
            "[REDACTED]",
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
fn adapter_branch_observation_never_authorizes_lifecycle_work() {
    let mut git = FakeGitAdapter::default();
    let observation = git.observe_branch(
        CommandInvocation::new("git", ["branch", "--show-current", "codex/502"])
            .expect("argv invocation"),
    );
    assert_eq!(observation.branch, "codex/502");
    assert!(!observation.authorizes_lifecycle);
}
