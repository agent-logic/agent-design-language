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
        let mut record = Self {
            generation: 0,
            digest: String::new(),
            state,
            audit: Vec::new(),
            projections_repair_required: false,
        };
        record.refresh_digest();
        record
    }

    fn refresh_digest(&mut self) {
        self.digest = digest_for(
            self.generation,
            self.state,
            &self.audit,
            self.projections_repair_required,
        );
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
    ProjectionRepairRequired,
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
        if self.committed.projections_repair_required {
            return Err(StoreError::ProjectionRepairRequired);
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
        let mut next_state = StateRecord {
            generation: next_generation,
            digest: String::new(),
            state: to,
            audit: next_audit,
            projections_repair_required: false,
        };
        next_state.refresh_digest();
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
    ) -> Result<CommitResult, StoreError> {
        if transaction.intent.expected_generation != self.committed.generation
            || transaction.intent.expected_digest != self.committed.digest
        {
            return Err(StoreError::StaleWriter {
                expected_generation: transaction.intent.expected_generation,
                actual_generation: self.committed.generation,
            });
        }
        self.journal.push(transaction.intent);
        self.committed = transaction.next_state;
        match projection_write {
            ProjectionWrite::Success => Ok(CommitResult::Committed(self.committed.clone())),
            ProjectionWrite::FailAfterStateCommit => {
                self.committed.projections_repair_required = true;
                self.committed.refresh_digest();
                Ok(CommitResult::ProjectionRepairRequired(
                    self.committed.clone(),
                ))
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

fn digest_for(
    generation: u64,
    state: LifecycleState,
    audit: &[AuditEvent],
    projections_repair_required: bool,
) -> String {
    let mut canonical =
        format!("generation={generation};state={state};repair={projections_repair_required};");
    for event in audit {
        canonical.push_str(&format!(
            "audit=g{}:{:?}:{}>{}:{};",
            event.generation,
            event.command,
            event.from,
            event.to,
            escape_digest_field(&event.provenance)
        ));
    }
    format!("v3:{:016x}", stable_hash64(canonical.as_bytes()))
}

fn stable_hash64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn escape_digest_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(':', "\\:")
}
