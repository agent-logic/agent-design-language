//! Pure semantic policy. Facts are policy inputs, never public mutation credentials.
use super::LifecycleState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

/// The semantic meaning of an amendment, independent of the card or field that
/// happened to carry the changed value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmendmentClass {
    ScopeAcceptance,
    Plan,
    ProofValidator,
    Binding,
    Implementation,
    Review,
    DisplayOnly,
}

/// A check which must be satisfied before an amendment can be admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmendmentPrerequisite {
    CurrentSourceVersion,
    MatchingIssueCheckout,
    IntactEvidence,
    ApprovedSemanticTransition,
    BoundTopology,
    ImplementationRevision,
    CurrentProof,
    IndependentReview,
    ProjectionChange,
}

/// How an admitted class derives the next semantic phase from its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmendmentPhaseEffect {
    Ready,
    Bound,
    Implemented,
    Preserve,
    PreserveReadyOtherwiseBound,
}

/// One row of the executable amendment table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmendmentRule {
    pub class: AmendmentClass,
    pub source_states: Vec<LifecycleState>,
    pub prerequisites: Vec<AmendmentPrerequisite>,
    pub phase_effect: AmendmentPhaseEffect,
    pub invalidations: Vec<Invalidation>,
}

/// Verified facts supplied by the application owner. These booleans are policy
/// inputs and do not constitute mutation authority by themselves.
#[derive(Debug, Clone, Default)]
pub struct AmendmentFacts {
    pub source_version_current: bool,
    pub issue_checkout_match: bool,
    pub evidence_integrity: bool,
    pub transition_approved: bool,
    pub topology: bool,
    pub implementation_revision: bool,
    pub current_proof: bool,
    pub independent_review: bool,
    pub projection_change: bool,
    /// Exact-head review currency is revision-bound even for display-only work.
    pub new_commit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCurrency {
    Preserved,
    FreshExactHeadReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmendmentRefusal {
    StaleSourceVersion,
    WrongIssueCheckout,
    CorruptedEvidence,
    UnapprovedTransition,
    MissingTopology,
    MissingImplementationRevision,
    MissingCurrentProof,
    MissingIndependentReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmendmentInapplicability {
    SourceState,
    NoProjectionChange,
}

/// The affected evidence plus the semantic class which caused invalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CausalInvalidation {
    pub evidence: Invalidation,
    pub cause: AmendmentClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "disposition", rename_all = "snake_case")]
pub enum AmendmentOutcome {
    Admitted {
        phase: LifecycleState,
        invalidations: Vec<CausalInvalidation>,
        review_currency: ReviewCurrency,
    },
    Refused {
        reason: AmendmentRefusal,
    },
    Inapplicable {
        reason: AmendmentInapplicability,
    },
}

const ACTIVE_STATES: [LifecycleState; 6] = [
    LifecycleState::Ready,
    LifecycleState::Bound,
    LifecycleState::Implemented,
    LifecycleState::Reviewed,
    LifecycleState::Published,
    LifecycleState::MergeReady,
];
const EXECUTABLE_STATES: [LifecycleState; 5] = [
    LifecycleState::Bound,
    LifecycleState::Implemented,
    LifecycleState::Reviewed,
    LifecycleState::Published,
    LifecycleState::MergeReady,
];
const REVIEW_STATES: [LifecycleState; 4] = [
    LifecycleState::Implemented,
    LifecycleState::Reviewed,
    LifecycleState::Published,
    LifecycleState::MergeReady,
];
const PROJECTABLE_STATES: [LifecycleState; 8] = [
    LifecycleState::Ready,
    LifecycleState::Bound,
    LifecycleState::Implemented,
    LifecycleState::Reviewed,
    LifecycleState::Published,
    LifecycleState::MergeReady,
    LifecycleState::Merged,
    LifecycleState::ClosedOut,
];

fn all_semantic_invalidations() -> Vec<Invalidation> {
    vec![
        Invalidation::Proof,
        Invalidation::Readiness,
        Invalidation::Review,
        Invalidation::Publication,
        Invalidation::Terminal,
        Invalidation::Cleanup,
    ]
}

/// Return the complete, machine-readable policy row for one amendment class.
pub fn amendment_rule(class: AmendmentClass) -> AmendmentRule {
    use AmendmentClass::*;
    use AmendmentPhaseEffect::*;
    use AmendmentPrerequisite::*;

    let common = vec![CurrentSourceVersion, MatchingIssueCheckout, IntactEvidence];
    let (source_states, mut prerequisites, phase_effect, invalidations) = match class {
        ScopeAcceptance => (
            ACTIVE_STATES.to_vec(),
            vec![ApprovedSemanticTransition],
            Ready,
            all_semantic_invalidations(),
        ),
        Plan => (
            ACTIVE_STATES.to_vec(),
            vec![ApprovedSemanticTransition],
            PreserveReadyOtherwiseBound,
            all_semantic_invalidations(),
        ),
        ProofValidator => (
            EXECUTABLE_STATES.to_vec(),
            vec![ApprovedSemanticTransition],
            Bound,
            all_semantic_invalidations(),
        ),
        Binding => (
            ACTIVE_STATES.to_vec(),
            vec![ApprovedSemanticTransition, BoundTopology],
            PreserveReadyOtherwiseBound,
            all_semantic_invalidations(),
        ),
        Implementation => (
            EXECUTABLE_STATES.to_vec(),
            vec![ApprovedSemanticTransition, ImplementationRevision],
            Bound,
            all_semantic_invalidations(),
        ),
        Review => (
            REVIEW_STATES.to_vec(),
            vec![ApprovedSemanticTransition, CurrentProof, IndependentReview],
            Implemented,
            vec![
                Invalidation::Readiness,
                Invalidation::Review,
                Invalidation::Publication,
                Invalidation::Terminal,
                Invalidation::Cleanup,
            ],
        ),
        DisplayOnly => (
            PROJECTABLE_STATES.to_vec(),
            vec![ProjectionChange],
            Preserve,
            vec![],
        ),
    };
    let mut required = common;
    required.append(&mut prerequisites);
    AmendmentRule {
        class,
        source_states,
        prerequisites: required,
        phase_effect,
        invalidations,
    }
}

/// Classify an amendment without mutating semantic or projection state.
///
/// Storage may retain the admitted result and its causal invalidations, but it
/// must independently bind these facts to the current record and transaction.
pub fn decide_amendment(
    from: LifecycleState,
    class: AmendmentClass,
    facts: &AmendmentFacts,
) -> AmendmentOutcome {
    use AmendmentInapplicability::*;
    use AmendmentPrerequisite::*;

    let rule = amendment_rule(class);
    if !rule.source_states.contains(&from) {
        return AmendmentOutcome::Inapplicable {
            reason: SourceState,
        };
    }

    for prerequisite in &rule.prerequisites {
        let refusal = match prerequisite {
            CurrentSourceVersion if !facts.source_version_current => {
                Some(AmendmentRefusal::StaleSourceVersion)
            }
            MatchingIssueCheckout if !facts.issue_checkout_match => {
                Some(AmendmentRefusal::WrongIssueCheckout)
            }
            IntactEvidence if !facts.evidence_integrity => {
                Some(AmendmentRefusal::CorruptedEvidence)
            }
            ApprovedSemanticTransition if !facts.transition_approved => {
                Some(AmendmentRefusal::UnapprovedTransition)
            }
            BoundTopology if !facts.topology => Some(AmendmentRefusal::MissingTopology),
            ImplementationRevision if !facts.implementation_revision => {
                Some(AmendmentRefusal::MissingImplementationRevision)
            }
            CurrentProof if !facts.current_proof => Some(AmendmentRefusal::MissingCurrentProof),
            IndependentReview if !facts.independent_review => {
                Some(AmendmentRefusal::MissingIndependentReview)
            }
            ProjectionChange if !facts.projection_change => {
                return AmendmentOutcome::Inapplicable {
                    reason: NoProjectionChange,
                };
            }
            _ => None,
        };
        if let Some(reason) = refusal {
            return AmendmentOutcome::Refused { reason };
        }
    }

    let phase = match rule.phase_effect {
        AmendmentPhaseEffect::Ready => LifecycleState::Ready,
        AmendmentPhaseEffect::Bound => LifecycleState::Bound,
        AmendmentPhaseEffect::Implemented => LifecycleState::Implemented,
        AmendmentPhaseEffect::Preserve => from,
        AmendmentPhaseEffect::PreserveReadyOtherwiseBound => {
            if from == LifecycleState::Ready {
                LifecycleState::Ready
            } else {
                LifecycleState::Bound
            }
        }
    };
    AmendmentOutcome::Admitted {
        phase,
        invalidations: rule
            .invalidations
            .into_iter()
            .map(|evidence| CausalInvalidation {
                evidence,
                cause: class,
            })
            .collect(),
        review_currency: if facts.new_commit {
            ReviewCurrency::FreshExactHeadReviewRequired
        } else {
            ReviewCurrency::Preserved
        },
    }
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
            AmendCards | AmendPlan | AmendValidation => active,
            AmendBinding => executable && facts.topology,
            Bind => from == Ready && facts.bind_target,
            RecordProof => executable,
            AssignReview => from == Implemented && facts.current_proof,
            RecordReviewPass => {
                from == Implemented && facts.current_proof && facts.independent_review
            }
            RecoverReview => {
                matches!(from, Reviewed | Published | MergeReady) && facts.recovery_provenance
            }
            Publish => {
                matches!(from, Reviewed | Published)
                    && facts.current_proof
                    && facts.independent_review
            }
            MarkMergeReady => from == Published && facts.merge_ready,
            RecordMerge => from == MergeReady && facts.merge_ready,
            Finish => from == Merged || (active && facts.merged && facts.terminal),
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
            (from == Merged || (active && facts.merged)) && facts.terminal,
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

#[cfg(test)]
mod phase_policy_tests {
    use super::*;

    fn terminal_merge_facts() -> Facts {
        Facts {
            merged: true,
            terminal: true,
            ..Facts::default()
        }
    }

    #[test]
    fn finish_closes_when_terminal_observation_catches_up_from_active_phase() {
        let decision = decide(
            Some(LifecycleState::Bound),
            SemanticCommand::Finish,
            Outcome::Success,
            &terminal_merge_facts(),
        )
        .expect("finish should admit an authenticated already-merged terminal closeout");

        assert_eq!(decision.phase, LifecycleState::ClosedOut);
        assert_eq!(
            decision.invalidations,
            vec![Invalidation::Terminal, Invalidation::Cleanup]
        );
    }

    #[test]
    fn finish_reservation_allows_already_merged_terminal_catch_up() {
        let mut facts = terminal_merge_facts();
        facts.original_command = Some(SemanticCommand::Finish);

        let decision = decide(
            Some(LifecycleState::Bound),
            SemanticCommand::Reserve,
            Outcome::Success,
            &facts,
        )
        .expect("reservation should admit an authenticated already-merged terminal closeout");

        assert_eq!(decision.phase, LifecycleState::Bound);
        assert!(decision.invalidations.is_empty());
    }

    #[test]
    fn finish_refuses_active_phase_without_authenticated_merge_observation() {
        let decision = decide(
            Some(LifecycleState::Bound),
            SemanticCommand::Finish,
            Outcome::Success,
            &Facts {
                terminal: true,
                ..Facts::default()
            },
        );

        assert_eq!(decision, Err(Rejection::MissingEvidence));
    }
}

#[cfg(test)]
mod amendment_tests {
    use super::*;

    fn valid_facts() -> AmendmentFacts {
        AmendmentFacts {
            source_version_current: true,
            issue_checkout_match: true,
            evidence_integrity: true,
            transition_approved: true,
            topology: true,
            implementation_revision: true,
            current_proof: true,
            independent_review: true,
            projection_change: true,
            new_commit: false,
        }
    }

    fn admitted(
        outcome: AmendmentOutcome,
    ) -> (LifecycleState, Vec<CausalInvalidation>, ReviewCurrency) {
        match outcome {
            AmendmentOutcome::Admitted {
                phase,
                invalidations,
                review_currency,
            } => (phase, invalidations, review_currency),
            other => panic!("expected admitted amendment, got {other:?}"),
        }
    }

    fn evidence(invalidations: &[CausalInvalidation]) -> Vec<Invalidation> {
        invalidations
            .iter()
            .map(|invalidation| invalidation.evidence)
            .collect()
    }

    #[test]
    fn scope_acceptance_amendment_rewinds_to_ready_and_invalidates_dependents() {
        let (phase, invalidations, currency) = admitted(decide_amendment(
            LifecycleState::Reviewed,
            AmendmentClass::ScopeAcceptance,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Ready);
        assert_eq!(evidence(&invalidations), all_semantic_invalidations());
        assert!(invalidations
            .iter()
            .all(|item| item.cause == AmendmentClass::ScopeAcceptance));
        assert_eq!(currency, ReviewCurrency::Preserved);
    }

    #[test]
    fn scope_acceptance_amendment_is_inapplicable_after_merge() {
        assert_eq!(
            decide_amendment(
                LifecycleState::Merged,
                AmendmentClass::ScopeAcceptance,
                &valid_facts(),
            ),
            AmendmentOutcome::Inapplicable {
                reason: AmendmentInapplicability::SourceState,
            }
        );
    }

    #[test]
    fn plan_amendment_preserves_ready_without_preserving_later_evidence() {
        let (phase, invalidations, _) = admitted(decide_amendment(
            LifecycleState::Ready,
            AmendmentClass::Plan,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Ready);
        assert_eq!(evidence(&invalidations), all_semantic_invalidations());
    }

    #[test]
    fn plan_amendment_refuses_an_unapproved_transition() {
        let mut facts = valid_facts();
        facts.transition_approved = false;
        assert_eq!(
            decide_amendment(LifecycleState::Bound, AmendmentClass::Plan, &facts),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::UnapprovedTransition,
            }
        );
    }

    #[test]
    fn proof_validator_amendment_rewinds_to_bound() {
        let (phase, invalidations, _) = admitted(decide_amendment(
            LifecycleState::Published,
            AmendmentClass::ProofValidator,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Bound);
        assert_eq!(evidence(&invalidations), all_semantic_invalidations());
    }

    #[test]
    fn proof_validator_amendment_is_inapplicable_before_binding() {
        assert_eq!(
            decide_amendment(
                LifecycleState::Ready,
                AmendmentClass::ProofValidator,
                &valid_facts(),
            ),
            AmendmentOutcome::Inapplicable {
                reason: AmendmentInapplicability::SourceState,
            }
        );
    }

    #[test]
    fn binding_amendment_rewinds_to_bound_with_causal_invalidations() {
        let (phase, invalidations, _) = admitted(decide_amendment(
            LifecycleState::MergeReady,
            AmendmentClass::Binding,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Bound);
        assert!(invalidations
            .iter()
            .all(|item| item.cause == AmendmentClass::Binding));
    }

    #[test]
    fn binding_head_refresh_preserves_ready_until_explicit_rebind() {
        let (phase, invalidations, _) = admitted(decide_amendment(
            LifecycleState::Ready,
            AmendmentClass::Binding,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Ready);
        assert!(!invalidations.is_empty());
        assert!(invalidations
            .iter()
            .all(|item| item.cause == AmendmentClass::Binding));
    }

    #[test]
    fn binding_amendment_refuses_missing_topology() {
        let mut facts = valid_facts();
        facts.topology = false;
        assert_eq!(
            decide_amendment(LifecycleState::Bound, AmendmentClass::Binding, &facts),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::MissingTopology,
            }
        );
    }

    #[test]
    fn implementation_amendment_invalidates_old_proof_and_review_currency() {
        let mut facts = valid_facts();
        facts.new_commit = true;
        let (phase, invalidations, currency) = admitted(decide_amendment(
            LifecycleState::Reviewed,
            AmendmentClass::Implementation,
            &facts,
        ));
        assert_eq!(phase, LifecycleState::Bound);
        assert_eq!(evidence(&invalidations), all_semantic_invalidations());
        assert_eq!(currency, ReviewCurrency::FreshExactHeadReviewRequired);
    }

    #[test]
    fn implementation_amendment_refuses_an_unidentified_revision() {
        let mut facts = valid_facts();
        facts.implementation_revision = false;
        assert_eq!(
            decide_amendment(
                LifecycleState::Implemented,
                AmendmentClass::Implementation,
                &facts,
            ),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::MissingImplementationRevision,
            }
        );
    }

    #[test]
    fn review_amendment_preserves_proof_and_rewinds_to_implemented() {
        let (phase, invalidations, _) = admitted(decide_amendment(
            LifecycleState::Published,
            AmendmentClass::Review,
            &valid_facts(),
        ));
        assert_eq!(phase, LifecycleState::Implemented);
        assert!(!evidence(&invalidations).contains(&Invalidation::Proof));
        assert!(evidence(&invalidations).contains(&Invalidation::Review));
        assert!(evidence(&invalidations).contains(&Invalidation::Publication));
    }

    #[test]
    fn review_amendment_refuses_without_current_proof() {
        let mut facts = valid_facts();
        facts.current_proof = false;
        assert_eq!(
            decide_amendment(LifecycleState::Reviewed, AmendmentClass::Review, &facts),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::MissingCurrentProof,
            }
        );
    }

    #[test]
    fn display_only_amendment_preserves_terminal_semantics_but_stales_commit_review() {
        let mut facts = valid_facts();
        facts.new_commit = true;
        let (phase, invalidations, currency) = admitted(decide_amendment(
            LifecycleState::ClosedOut,
            AmendmentClass::DisplayOnly,
            &facts,
        ));
        assert_eq!(phase, LifecycleState::ClosedOut);
        assert!(invalidations.is_empty());
        assert_eq!(currency, ReviewCurrency::FreshExactHeadReviewRequired);
    }

    #[test]
    fn display_only_amendment_is_inapplicable_without_projection_drift() {
        let mut facts = valid_facts();
        facts.projection_change = false;
        assert_eq!(
            decide_amendment(
                LifecycleState::Reviewed,
                AmendmentClass::DisplayOnly,
                &facts,
            ),
            AmendmentOutcome::Inapplicable {
                reason: AmendmentInapplicability::NoProjectionChange,
            }
        );
    }

    #[test]
    fn common_guards_refuse_stale_mismatched_and_corrupt_sources() {
        let mut facts = valid_facts();
        facts.source_version_current = false;
        assert_eq!(
            decide_amendment(
                LifecycleState::Bound,
                AmendmentClass::ProofValidator,
                &facts,
            ),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::StaleSourceVersion,
            }
        );

        let mut facts = valid_facts();
        facts.issue_checkout_match = false;
        assert_eq!(
            decide_amendment(
                LifecycleState::Bound,
                AmendmentClass::Implementation,
                &facts,
            ),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::WrongIssueCheckout,
            }
        );

        let mut facts = valid_facts();
        facts.evidence_integrity = false;
        assert_eq!(
            decide_amendment(
                LifecycleState::Reviewed,
                AmendmentClass::DisplayOnly,
                &facts,
            ),
            AmendmentOutcome::Refused {
                reason: AmendmentRefusal::CorruptedEvidence,
            }
        );
    }
}
