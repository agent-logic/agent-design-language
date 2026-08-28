use crate::lifecycle::{
    decide, CapabilitySet, LifecycleCommand, LifecycleState, ProjectionInvalidation,
    TransitionOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateRecord {
    pub generation: u64,
    pub digest: String,
    pub state: LifecycleState,
    pub audit: Vec<AuditEvent>,
    pub projections_repair_required: bool,
    pub invalidated_projections: Vec<ProjectionInvalidation>,
}

impl StateRecord {
    pub fn new(state: LifecycleState) -> Self {
        let mut record = Self {
            generation: 0,
            digest: String::new(),
            state,
            audit: Vec::new(),
            projections_repair_required: false,
            invalidated_projections: Vec::new(),
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
            &self.invalidated_projections,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub generation: u64,
    pub command: LifecycleCommand,
    pub from: LifecycleState,
    pub to: LifecycleState,
    pub invalidates: Vec<ProjectionInvalidation>,
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
    InvalidRecordDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionStore {
    committed: StateRecord,
    journal: Vec<TransactionIntent>,
}

impl TransactionStore {
    pub fn new(initial: StateRecord) -> Result<Self, StoreError> {
        if initial.digest
            != digest_for(
                initial.generation,
                initial.state,
                &initial.audit,
                initial.projections_repair_required,
                &initial.invalidated_projections,
            )
        {
            return Err(StoreError::InvalidRecordDigest);
        }
        Ok(Self {
            committed: initial,
            journal: Vec::new(),
        })
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
        let TransitionOutcome::Allowed { to, invalidates } = decision.outcome else {
            return Err(StoreError::RejectedTransition);
        };
        let mut next_audit = self.committed.audit.clone();
        let next_generation = self.committed.generation + 1;
        next_audit.push(AuditEvent {
            generation: next_generation,
            command,
            from: self.committed.state,
            to,
            invalidates: invalidates.clone(),
            provenance: provenance.into(),
        });
        let mut next_state = StateRecord {
            generation: next_generation,
            digest: String::new(),
            state: to,
            audit: next_audit,
            projections_repair_required: false,
            invalidated_projections: merge_invalidations(
                &self.committed.invalidated_projections,
                &invalidates,
            ),
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
    CorruptRecoveryInput {
        reason: RecoveryRejectReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRepair {
    RegenerateProjections,
    ExactReadbackBeforeRemoteResume,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRejectReason {
    InvalidStateDigest,
    IntentDoesNotMatchCommittedState,
    RepairIntentMissing,
}

pub fn classify_recovery(observation: RecoveryObservation) -> RecoveryClassification {
    match observation {
        RecoveryObservation::NoIntent { state } => {
            if let Err(reason) = validate_state_record(&state) {
                return RecoveryClassification::CorruptRecoveryInput { reason };
            }
            if state.projections_repair_required {
                return RecoveryClassification::CorruptRecoveryInput {
                    reason: RecoveryRejectReason::RepairIntentMissing,
                };
            }
            RecoveryClassification::NewState(state)
        }
        RecoveryObservation::IntentWithoutCommit { prior, intent } => {
            if let Err(reason) = validate_state_record(&prior) {
                return RecoveryClassification::CorruptRecoveryInput { reason };
            }
            if prior.generation != intent.expected_generation
                || prior.digest != intent.expected_digest
            {
                return RecoveryClassification::CorruptRecoveryInput {
                    reason: RecoveryRejectReason::IntentDoesNotMatchCommittedState,
                };
            }
            RecoveryClassification::PriorState(prior)
        }
        RecoveryObservation::StateCommittedProjectionMissing { state, intent } => {
            if let Err(reason) = validate_committed_recovery_input(&state, &intent) {
                return RecoveryClassification::CorruptRecoveryInput { reason };
            }
            RecoveryClassification::RepairRequired {
                state,
                intent,
                repair: RecoveryRepair::RegenerateProjections,
            }
        }
        RecoveryObservation::StateCommitted { state, intent } => {
            if let Err(reason) = validate_committed_recovery_input(&state, &intent) {
                return RecoveryClassification::CorruptRecoveryInput { reason };
            }
            if matches!(
                intent.command,
                LifecycleCommand::Publish
                    | LifecycleCommand::RecordMerge
                    | LifecycleCommand::Finish
            ) {
                RecoveryClassification::RepairRequired {
                    state,
                    intent,
                    repair: RecoveryRepair::ExactReadbackBeforeRemoteResume,
                }
            } else {
                RecoveryClassification::NewState(state)
            }
        }
    }
}

fn validate_committed_recovery_input(
    state: &StateRecord,
    intent: &TransactionIntent,
) -> Result<(), RecoveryRejectReason> {
    validate_state_record(state)?;
    let Some(event) = state.audit.last() else {
        return Err(RecoveryRejectReason::IntentDoesNotMatchCommittedState);
    };
    let prior_audit = &state.audit[..state.audit.len() - 1];
    let prior_invalidations = invalidations_for_audit(prior_audit);
    let prior_digest = digest_for(
        intent.expected_generation,
        event.from,
        prior_audit,
        false,
        &prior_invalidations,
    );
    let expected_invalidations = merge_invalidations(&prior_invalidations, &event.invalidates);
    if state.generation != intent.expected_generation + 1
        || event.generation != state.generation
        || event.command != intent.command
        || event.to != state.state
        || state.invalidated_projections != expected_invalidations
        || event.provenance != intent.provenance
        || intent.expected_digest != prior_digest
    {
        return Err(RecoveryRejectReason::IntentDoesNotMatchCommittedState);
    }
    Ok(())
}

fn validate_state_record(state: &StateRecord) -> Result<(), RecoveryRejectReason> {
    let expected_digest = digest_for(
        state.generation,
        state.state,
        &state.audit,
        state.projections_repair_required,
        &state.invalidated_projections,
    );
    if state.digest != expected_digest {
        return Err(RecoveryRejectReason::InvalidStateDigest);
    }
    Ok(())
}

fn digest_for(
    generation: u64,
    state: LifecycleState,
    audit: &[AuditEvent],
    projections_repair_required: bool,
    invalidated_projections: &[ProjectionInvalidation],
) -> String {
    let mut canonical =
        format!("generation={generation};state={state};repair={projections_repair_required};");
    for event in audit {
        canonical.push_str(&format!(
            "audit=g{}:{:?}:{}>{}:{}:",
            event.generation,
            event.command,
            event.from,
            event.to,
            escape_digest_field(&event.provenance),
        ));
        for invalidation in &event.invalidates {
            canonical.push_str(&format!("{invalidation:?},"));
        }
        canonical.push(';');
    }
    for invalidation in invalidated_projections {
        canonical.push_str(&format!("invalidates={invalidation:?};"));
    }
    format!("v3:{:016x}", stable_hash64(canonical.as_bytes()))
}

fn merge_invalidations(
    current: &[ProjectionInvalidation],
    next: &[ProjectionInvalidation],
) -> Vec<ProjectionInvalidation> {
    let mut merged = current.to_vec();
    merged.extend(next.iter().copied());
    merged.sort();
    merged.dedup();
    merged
}

fn invalidations_for_audit(audit: &[AuditEvent]) -> Vec<ProjectionInvalidation> {
    audit.iter().fold(Vec::new(), |merged, event| {
        merge_invalidations(&merged, &event.invalidates)
    })
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
