use crate::lifecycle::{
    decide, CapabilitySet, LifecycleCommand, LifecycleState, TransitionOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateRecord {
    pub generation: u64,
    pub digest: String,
    pub state: LifecycleState,
    pub audit: Vec<AuditEvent>,
    pub projections_repair_required: bool,
}

impl StateRecord {
    pub fn new(state: LifecycleState) -> Self {
        Self {
            generation: 0,
            digest: digest_for(0, state, &[]),
            state,
            audit: Vec::new(),
            projections_repair_required: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub generation: u64,
    pub command: LifecycleCommand,
    pub from: LifecycleState,
    pub to: LifecycleState,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionIntent {
    pub expected_generation: u64,
    pub expected_digest: String,
    pub command: LifecycleCommand,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedTransaction {
    intent: TransactionIntent,
    next_state: StateRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitResult {
    Committed(StateRecord),
    ProjectionRepairRequired(StateRecord),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    StaleWriter {
        expected_generation: u64,
        actual_generation: u64,
    },
    RejectedTransition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionStore {
    committed: StateRecord,
    journal: Vec<TransactionIntent>,
}

impl TransactionStore {
    pub fn new(initial: StateRecord) -> Self {
        Self {
            committed: initial,
            journal: Vec::new(),
        }
    }

    pub fn committed(&self) -> &StateRecord {
        &self.committed
    }

    pub fn journal(&self) -> &[TransactionIntent] {
        &self.journal
    }

    pub fn begin(
        &self,
        command: LifecycleCommand,
        capabilities: &CapabilitySet,
        expected_generation: u64,
        expected_digest: impl Into<String>,
        provenance: impl Into<String>,
    ) -> Result<StagedTransaction, StoreError> {
        let expected_digest = expected_digest.into();
        if expected_generation != self.committed.generation
            || expected_digest != self.committed.digest
        {
            return Err(StoreError::StaleWriter {
                expected_generation,
                actual_generation: self.committed.generation,
            });
        }
        let decision = decide(self.committed.state, command, capabilities);
        let TransitionOutcome::Allowed { to, .. } = decision.outcome else {
            return Err(StoreError::RejectedTransition);
        };
        let mut next_audit = self.committed.audit.clone();
        let next_generation = self.committed.generation + 1;
        next_audit.push(AuditEvent {
            generation: next_generation,
            command,
            from: self.committed.state,
            to,
            provenance: provenance.into(),
        });
        let next_state = StateRecord {
            generation: next_generation,
            digest: digest_for(next_generation, to, &next_audit),
            state: to,
            audit: next_audit,
            projections_repair_required: false,
        };
        Ok(StagedTransaction {
            intent: TransactionIntent {
                expected_generation,
                expected_digest,
                command,
                provenance: next_state
                    .audit
                    .last()
                    .expect("audit event staged")
                    .provenance
                    .clone(),
            },
            next_state,
        })
    }

    pub fn commit(
        &mut self,
        transaction: StagedTransaction,
        projection_write: ProjectionWrite,
    ) -> CommitResult {
        self.journal.push(transaction.intent);
        self.committed = transaction.next_state;
        match projection_write {
            ProjectionWrite::Success => CommitResult::Committed(self.committed.clone()),
            ProjectionWrite::FailAfterStateCommit => {
                self.committed.projections_repair_required = true;
                CommitResult::ProjectionRepairRequired(self.committed.clone())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionWrite {
    Success,
    FailAfterStateCommit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryObservation {
    NoIntent {
        state: StateRecord,
    },
    IntentWithoutCommit {
        prior: StateRecord,
        intent: TransactionIntent,
    },
    StateCommittedProjectionMissing {
        state: StateRecord,
        intent: TransactionIntent,
    },
    StateCommitted {
        state: StateRecord,
        intent: TransactionIntent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryClassification {
    PriorState(StateRecord),
    NewState(StateRecord),
    RepairRequired {
        state: StateRecord,
        intent: TransactionIntent,
        repair: RecoveryRepair,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRepair {
    RegenerateProjections,
    ExactReadbackBeforeRemoteResume,
}

pub fn classify_recovery(observation: RecoveryObservation) -> RecoveryClassification {
    match observation {
        RecoveryObservation::NoIntent { state } => RecoveryClassification::NewState(state),
        RecoveryObservation::IntentWithoutCommit { prior, .. } => {
            RecoveryClassification::PriorState(prior)
        }
        RecoveryObservation::StateCommittedProjectionMissing { state, intent } => {
            RecoveryClassification::RepairRequired {
                state,
                intent,
                repair: RecoveryRepair::RegenerateProjections,
            }
        }
        RecoveryObservation::StateCommitted { state, intent }
            if matches!(
                intent.command,
                LifecycleCommand::Publish
                    | LifecycleCommand::RecordMerge
                    | LifecycleCommand::Finish
            ) =>
        {
            RecoveryClassification::RepairRequired {
                state,
                intent,
                repair: RecoveryRepair::ExactReadbackBeforeRemoteResume,
            }
        }
        RecoveryObservation::StateCommitted { state, .. } => {
            RecoveryClassification::NewState(state)
        }
    }
}

fn digest_for(generation: u64, state: LifecycleState, audit: &[AuditEvent]) -> String {
    format!("v3:{generation}:{state}:{}", audit.len())
}
