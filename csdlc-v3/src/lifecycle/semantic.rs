//! Pure semantic policy. Facts are policy inputs, never public mutation credentials.
use super::LifecycleState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticCommand {
    Prepare,
    Bind,
    AmendCards,
    AmendPlan,
    AmendValidation,
    AmendBinding,
    RecordProof,
    AssignReview,
    RecordReviewPass,
    RecoverReview,
    RecordIssueMutation,
    Publish,
    MarkMergeReady,
    RecordMerge,
    Finish,
    FinishWithoutPr,
    RecordCleanup,
    RecordInstall,
    RecordCutover,
    RecordRollback,
    Reserve,
    AcknowledgeProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Success,
    Failure,
    Unresolved,
}

/// Testable policy facts. Storage accepts verified owner outcomes, not this type.
#[derive(Debug, Clone, Default)]
pub struct Facts {
    pub prepared: bool,
    pub topology: bool,
    pub bind_target: bool,
    pub terminal_receipt: bool,
    pub current_proof: bool,
    pub independent_review: bool,
    pub publication: bool,
    pub merge_ready: bool,
    pub merged: bool,
    pub terminal: bool,
    pub no_pr_disposition: bool,
    pub cleanup: bool,
    pub administrative: bool,
    pub administrative_outcome: bool,
    pub recovery_provenance: bool,
    pub original_command: Option<SemanticCommand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Invalidation {
    Proof,
    Readiness,
    Review,
    Publication,
    Terminal,
    Cleanup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub phase: LifecycleState,
    pub invalidations: Vec<Invalidation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rejection {
    InvalidState,
    MissingEvidence,
    PendingOutcome,
    InvalidReservation,
}

/// Complete phase policy; recovery must re-apply the original command with verified facts.
/// Matching completed replay bypasses this function and never changes phase/version.
pub fn decide(
    from: Option<LifecycleState>,
    command: SemanticCommand,
    outcome: Outcome,
    facts: &Facts,
) -> Result<Decision, Rejection> {
    use LifecycleState::*;
    use SemanticCommand::*;
    if outcome == Outcome::Unresolved {
        return Err(Rejection::PendingOutcome);
    }
    if command == Prepare {
        return if from.is_none() && facts.prepared && outcome == Outcome::Success {
            Ok(Decision {
                phase: Ready,
                invalidations: vec![],
            })
        } else {
            Err(Rejection::InvalidState)
        };
    }
    let from = from.ok_or(Rejection::InvalidState)?;
    let active = matches!(
        from,
        Ready | Bound | Implemented | Reviewed | Published | MergeReady
    );
    let executable = matches!(
        from,
        Bound | Implemented | Reviewed | Published | MergeReady
    );
    if command == Reserve || outcome == Outcome::Failure {
        let original = if command == Reserve {
            facts
                .original_command
                .ok_or(Rejection::InvalidReservation)?
        } else {
            command
        };
        let allowed = match original {
            Bind => from == Ready && facts.bind_target,
            RecordProof => executable,
            Publish => {
                matches!(from, Reviewed | Published)
                    && facts.current_proof
                    && facts.independent_review
            }
            RecordMerge => from == MergeReady && facts.merge_ready,
            Finish => from == Merged,
            FinishWithoutPr => active && facts.no_pr_disposition,
            RecordCleanup => from == ClosedOut && facts.terminal_receipt,
            RecordInstall | RecordCutover | RecordRollback => active && facts.administrative,
            RecordIssueMutation => active || matches!(from, Merged | ClosedOut),
            _ => false,
        };
        if !allowed {
            return Err(Rejection::InvalidReservation);
        }
        return Ok(Decision {
            phase: from,
            invalidations: if command != Reserve && original == RecordProof {
                vec![
                    Invalidation::Proof,
                    Invalidation::Readiness,
                    Invalidation::Review,
                    Invalidation::Publication,
                    Invalidation::Terminal,
                    Invalidation::Cleanup,
                ]
            } else {
                vec![]
            },
        });
    }
    let all = vec![
        Invalidation::Proof,
        Invalidation::Readiness,
        Invalidation::Review,
        Invalidation::Publication,
        Invalidation::Terminal,
        Invalidation::Cleanup,
    ];
    let (to, admitted, invalidations) = match command {
        AmendCards | AmendPlan | AmendValidation => (from, active, all),
        AmendBinding => (from, executable && facts.topology, all),
        Bind => (
            Bound,
            from == Ready && facts.topology,
            vec![Invalidation::Readiness],
        ),
        RecordProof if outcome == Outcome::Failure => (from, executable, all),
        RecordProof => (
            Implemented,
            executable && facts.current_proof,
            vec![
                Invalidation::Readiness,
                Invalidation::Review,
                Invalidation::Publication,
                Invalidation::Terminal,
                Invalidation::Cleanup,
            ],
        ),
        AssignReview => (
            Implemented,
            from == Implemented && facts.current_proof,
            vec![Invalidation::Review],
        ),
        RecordReviewPass => (
            Reviewed,
            from == Implemented && facts.current_proof && facts.independent_review,
            vec![Invalidation::Review, Invalidation::Publication],
        ),
        RecoverReview => (
            Implemented,
            matches!(from, Reviewed | Published | MergeReady) && facts.recovery_provenance,
            vec![
                Invalidation::Readiness,
                Invalidation::Review,
                Invalidation::Publication,
                Invalidation::Terminal,
            ],
        ),
        RecordIssueMutation => (from, active || matches!(from, Merged | ClosedOut), vec![]),
        Publish => (
            Published,
            matches!(from, Reviewed | Published)
                && facts.current_proof
                && facts.independent_review
                && facts.publication,
            vec![Invalidation::Publication],
        ),
        MarkMergeReady => (
            MergeReady,
            from == Published && facts.merge_ready,
            vec![Invalidation::Publication],
        ),
        RecordMerge => (
            Merged,
            from == MergeReady && facts.merged,
            vec![
                Invalidation::Publication,
                Invalidation::Terminal,
                Invalidation::Cleanup,
            ],
        ),
        Finish => (
            ClosedOut,
            from == Merged && facts.terminal,
            vec![Invalidation::Terminal, Invalidation::Cleanup],
        ),
        FinishWithoutPr => (
            ClosedOut,
            active && facts.no_pr_disposition && facts.terminal,
            vec![Invalidation::Terminal, Invalidation::Cleanup],
        ),
        RecordCleanup => (
            ClosedOut,
            from == ClosedOut && facts.cleanup,
            vec![Invalidation::Cleanup],
        ),
        RecordInstall | RecordCutover | RecordRollback => (
            from,
            active && facts.administrative && facts.administrative_outcome,
            all,
        ),
        AcknowledgeProjection => (from, true, vec![]),
        Reserve => unreachable!("handled before outcome selection"),
        Prepare => unreachable!("handled before state selection"),
    };
    if !admitted {
        return Err(Rejection::MissingEvidence);
    }
    // Failure never takes a success phase. Proof failure additionally invalidates proof.
    Ok(Decision {
        phase: if outcome == Outcome::Failure {
            from
        } else {
            to
        },
        invalidations: if outcome == Outcome::Failure && command != RecordProof {
            vec![]
        } else {
            invalidations
        },
    })
}
