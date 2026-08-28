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
    RecoveryRepair, StateRecord, TransactionStore,
};
use csdlc_v3::LIFECYCLE_KERNEL_PREDECESSORS;
use std::collections::BTreeSet;

fn full_capabilities() -> CapabilitySet {
    CapabilitySet::new([
        Capability::BoundTopology,
        Capability::ImplementationEvidence,
        Capability::IndependentExactHeadReview,
        Capability::PublicationLinkage,
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
fn transaction_stale_writer_fails_before_commit() {
    let initial = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(initial.clone());
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest.clone(),
            "bind provenance",
        )
        .expect("bind transaction stages");
    store.commit(transaction, ProjectionWrite::Success);
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
    let mut store = TransactionStore::new(initial.clone());
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            initial.generation,
            initial.digest,
            "durable typed intent",
        )
        .expect("bind transaction stages");
    let result = store.commit(transaction, ProjectionWrite::FailAfterStateCommit);
    let CommitResult::ProjectionRepairRequired(committed) = result else {
        panic!("projection failure must not roll back state");
    };
    assert_eq!(committed.state, LifecycleState::Bound);
    assert!(committed.projections_repair_required);
    assert_eq!(committed.audit[0].provenance, "durable typed intent");
}

#[test]
fn recovery_classifies_interrupted_writes_without_losing_provenance() {
    let prior = StateRecord::new(LifecycleState::Ready);
    let mut store = TransactionStore::new(prior.clone());
    let transaction = store
        .begin(
            LifecycleCommand::Bind,
            &full_capabilities(),
            prior.generation,
            prior.digest.clone(),
            "recovery provenance",
        )
        .expect("bind transaction stages");
    let committed = match store.commit(transaction, ProjectionWrite::FailAfterStateCommit) {
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
fn adapter_invocations_are_argv_based_and_shell_strings_are_rejected() {
    assert!(CommandInvocation::new("git", ["status", "--short"]).is_ok());
    assert!(CommandInvocation::new("git status", ["--short"]).is_err());
    assert!(CommandInvocation::new("git", ["status && gh pr merge"]).is_err());
    assert!(CommandInvocation::new("sh", ["-c", "git status"]).is_err());
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
