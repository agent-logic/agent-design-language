use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecycleState {
    Initialized,
    Ready,
    Bound,
    Implemented,
    Reviewed,
    Published,
    MergeReady,
    Merged,
    ClosedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LifecycleCommand {
    Bind,
    RecordImplementation,
    AssignReview,
    RecordReviewPass,
    RecoverReview,
    Publish,
    MarkMergeReady,
    RecordMerge,
    Finish,
    Cleanup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability {
    BoundTopology,
    ImplementationEvidence,
    IndependentExactHeadReview,
    PublicationLinkage,
    MergeReadinessEvidence,
    LiveMergeEvidence,
    LiveTerminalEvidence,
    TerminalReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilitySet {
    values: Vec<Capability>,
}

impl CapabilitySet {
    pub fn new(values: impl IntoIterator<Item = Capability>) -> Self {
        let mut values = values.into_iter().collect::<Vec<_>>();
        values.sort();
        values.dedup();
        Self { values }
    }

    pub fn contains(&self, capability: Capability) -> bool {
        self.values.contains(&capability)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionDecision {
    pub command: LifecycleCommand,
    pub from: LifecycleState,
    pub outcome: TransitionOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionOutcome {
    Allowed {
        to: LifecycleState,
        invalidates: Vec<ProjectionInvalidation>,
    },
    Rejected {
        reason: RejectReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionInvalidation {
    Readiness,
    Review,
    Publication,
    Terminal,
    CleanupEligibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    InvalidState,
    MissingCapability(Capability),
    BranchObservationOnly,
    TerminalReceiptRequired,
}

pub fn decide(
    from: LifecycleState,
    command: LifecycleCommand,
    capabilities: &CapabilitySet,
) -> TransitionDecision {
    let outcome = match (from, command) {
        (LifecycleState::Ready, LifecycleCommand::Bind)
            if capabilities.contains(Capability::BoundTopology) =>
        {
            allow(LifecycleState::Bound, [ProjectionInvalidation::Readiness])
        }
        (LifecycleState::Ready, LifecycleCommand::Bind) => {
            reject(RejectReason::BranchObservationOnly)
        }
        (LifecycleState::Bound, LifecycleCommand::RecordImplementation)
            if capabilities.contains(Capability::ImplementationEvidence) =>
        {
            allow(
                LifecycleState::Implemented,
                [
                    ProjectionInvalidation::Readiness,
                    ProjectionInvalidation::Review,
                    ProjectionInvalidation::Publication,
                ],
            )
        }
        (LifecycleState::Implemented, LifecycleCommand::AssignReview) => allow(
            LifecycleState::Implemented,
            [ProjectionInvalidation::Review],
        ),
        (LifecycleState::Implemented, LifecycleCommand::RecordReviewPass)
            if capabilities.contains(Capability::IndependentExactHeadReview) =>
        {
            allow(
                LifecycleState::Reviewed,
                [
                    ProjectionInvalidation::Review,
                    ProjectionInvalidation::Publication,
                ],
            )
        }
        (
            LifecycleState::Reviewed | LifecycleState::Published | LifecycleState::MergeReady,
            LifecycleCommand::RecoverReview,
        ) => allow(
            LifecycleState::Implemented,
            [
                ProjectionInvalidation::Readiness,
                ProjectionInvalidation::Review,
                ProjectionInvalidation::Publication,
                ProjectionInvalidation::Terminal,
            ],
        ),
        (LifecycleState::Reviewed, LifecycleCommand::Publish)
            if capabilities.contains(Capability::PublicationLinkage) =>
        {
            allow(
                LifecycleState::Published,
                [ProjectionInvalidation::Publication],
            )
        }
        (LifecycleState::Published, LifecycleCommand::MarkMergeReady)
            if capabilities.contains(Capability::MergeReadinessEvidence) =>
        {
            allow(
                LifecycleState::MergeReady,
                [ProjectionInvalidation::Publication],
            )
        }
        (LifecycleState::MergeReady, LifecycleCommand::RecordMerge)
            if capabilities.contains(Capability::LiveMergeEvidence) =>
        {
            allow(
                LifecycleState::Merged,
                [
                    ProjectionInvalidation::Publication,
                    ProjectionInvalidation::Terminal,
                    ProjectionInvalidation::CleanupEligibility,
                ],
            )
        }
        (LifecycleState::Merged, LifecycleCommand::Finish)
            if capabilities.contains(Capability::LiveTerminalEvidence) =>
        {
            allow(
                LifecycleState::ClosedOut,
                [
                    ProjectionInvalidation::Terminal,
                    ProjectionInvalidation::CleanupEligibility,
                ],
            )
        }
        (LifecycleState::ClosedOut, LifecycleCommand::Cleanup)
            if capabilities.contains(Capability::TerminalReceipt) =>
        {
            allow(
                LifecycleState::ClosedOut,
                [ProjectionInvalidation::CleanupEligibility],
            )
        }
        (LifecycleState::ClosedOut, LifecycleCommand::Cleanup) => {
            reject(RejectReason::TerminalReceiptRequired)
        }
        (_, command)
            if required_capability(command).is_some_and(|cap| !capabilities.contains(cap)) =>
        {
            reject(RejectReason::MissingCapability(
                required_capability(command).expect("checked"),
            ))
        }
        _ => reject(RejectReason::InvalidState),
    };
    TransitionDecision {
        command,
        from,
        outcome,
    }
}

pub fn transition_matrix() -> Vec<TransitionDecision> {
    let capabilities = CapabilitySet::new([
        Capability::BoundTopology,
        Capability::ImplementationEvidence,
        Capability::IndependentExactHeadReview,
        Capability::PublicationLinkage,
        Capability::MergeReadinessEvidence,
        Capability::LiveMergeEvidence,
        Capability::LiveTerminalEvidence,
        Capability::TerminalReceipt,
    ]);
    let states = [
        LifecycleState::Initialized,
        LifecycleState::Ready,
        LifecycleState::Bound,
        LifecycleState::Implemented,
        LifecycleState::Reviewed,
        LifecycleState::Published,
        LifecycleState::MergeReady,
        LifecycleState::Merged,
        LifecycleState::ClosedOut,
    ];
    let commands = [
        LifecycleCommand::Bind,
        LifecycleCommand::RecordImplementation,
        LifecycleCommand::AssignReview,
        LifecycleCommand::RecordReviewPass,
        LifecycleCommand::RecoverReview,
        LifecycleCommand::Publish,
        LifecycleCommand::MarkMergeReady,
        LifecycleCommand::RecordMerge,
        LifecycleCommand::Finish,
        LifecycleCommand::Cleanup,
    ];
    let mut matrix = Vec::with_capacity(states.len() * commands.len());
    for state in states {
        for command in commands {
            matrix.push(decide(state, command, &capabilities));
        }
    }
    matrix
}

fn allow(
    to: LifecycleState,
    invalidates: impl IntoIterator<Item = ProjectionInvalidation>,
) -> TransitionOutcome {
    TransitionOutcome::Allowed {
        to,
        invalidates: invalidates.into_iter().collect(),
    }
}

fn reject(reason: RejectReason) -> TransitionOutcome {
    TransitionOutcome::Rejected { reason }
}

fn required_capability(command: LifecycleCommand) -> Option<Capability> {
    match command {
        LifecycleCommand::Bind => Some(Capability::BoundTopology),
        LifecycleCommand::RecordImplementation => Some(Capability::ImplementationEvidence),
        LifecycleCommand::RecordReviewPass => Some(Capability::IndependentExactHeadReview),
        LifecycleCommand::Publish => Some(Capability::PublicationLinkage),
        LifecycleCommand::MarkMergeReady => Some(Capability::MergeReadinessEvidence),
        LifecycleCommand::RecordMerge => Some(Capability::LiveMergeEvidence),
        LifecycleCommand::Finish => Some(Capability::LiveTerminalEvidence),
        LifecycleCommand::Cleanup => Some(Capability::TerminalReceipt),
        LifecycleCommand::AssignReview | LifecycleCommand::RecoverReview => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRecoveryProvenance {
    pub actor: String,
    pub reason: String,
    pub stale_review_revision: String,
}

impl ReviewRecoveryProvenance {
    pub fn new(
        actor: impl Into<String>,
        reason: impl Into<String>,
        stale_review_revision: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            actor: actor.into(),
            reason: reason.into(),
            stale_review_revision: stale_review_revision.into(),
        };
        if provenance.actor.trim().is_empty()
            || provenance.reason.trim().is_empty()
            || provenance.stale_review_revision.trim().is_empty()
        {
            return Err(ProvenanceError::MissingRecoveryField);
        }
        Ok(provenance)
    }

    pub fn audit_provenance(&self) -> String {
        format!(
            "review_recovery actor={} reason={} stale_review_revision={}",
            escape_provenance_field(&self.actor),
            escape_provenance_field(&self.reason),
            escape_provenance_field(&self.stale_review_revision)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceError {
    MissingRecoveryField,
}

fn escape_provenance_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('|', "\\|")
        .replace(' ', "\\s")
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Initialized => "initialized",
            Self::Ready => "ready",
            Self::Bound => "bound",
            Self::Implemented => "implemented",
            Self::Reviewed => "reviewed",
            Self::Published => "published",
            Self::MergeReady => "merge_ready",
            Self::Merged => "merged",
            Self::ClosedOut => "closed_out",
        })
    }
}
