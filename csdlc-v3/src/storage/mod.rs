use crate::lifecycle::{
    decide, CapabilitySet, LifecycleCommand, LifecycleState, ProjectionInvalidation,
    ReviewRecoveryProvenance, TransitionOutcome,
};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

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
    LockUnavailable,
    Io(String),
    StructuredReviewRecoveryProvenanceRequired,
    RecoveryRequired(RecoveryRepair),
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
        if command == LifecycleCommand::RecoverReview {
            return Err(StoreError::StructuredReviewRecoveryProvenanceRequired);
        }
        self.begin_with_provenance(
            command,
            capabilities,
            expected_generation,
            expected_digest,
            provenance.into(),
        )
    }

    pub fn begin_review_recovery(
        &self,
        capabilities: &CapabilitySet,
        expected_generation: u64,
        expected_digest: impl Into<String>,
        provenance: ReviewRecoveryProvenance,
    ) -> Result<StagedTransaction, StoreError> {
        self.begin_with_provenance(
            LifecycleCommand::RecoverReview,
            capabilities,
            expected_generation,
            expected_digest,
            provenance.audit_provenance(),
        )
    }

    fn begin_with_provenance(
        &self,
        command: LifecycleCommand,
        capabilities: &CapabilitySet,
        expected_generation: u64,
        expected_digest: impl Into<String>,
        provenance: String,
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
            provenance,
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

#[derive(Debug)]
pub struct DurableTransactionStore {
    store: TransactionStore,
    directory: PathBuf,
    _lock: StoreLock,
}

impl DurableTransactionStore {
    pub fn create(directory: impl AsRef<Path>, initial: StateRecord) -> Result<Self, StoreError> {
        let directory = directory.as_ref().to_path_buf();
        fs::create_dir_all(&directory).map_err(io_error)?;
        let lock = StoreLock::acquire(&directory)?;
        let store = TransactionStore::new(initial)?;
        write_state_atomically(&directory, store.committed())?;
        sync_directory(&directory)?;
        Ok(Self {
            store,
            directory,
            _lock: lock,
        })
    }

    pub fn open(directory: impl AsRef<Path>) -> Result<Self, StoreError> {
        let directory = directory.as_ref().to_path_buf();
        let lock = StoreLock::acquire(&directory)?;
        let committed = read_state(&directory)?;
        let journal = read_intents(&directory)?;
        classify_open_recovery(&committed, journal.last())?;
        let store = TransactionStore { committed, journal };
        Ok(Self {
            store,
            directory,
            _lock: lock,
        })
    }

    pub fn committed(&self) -> &StateRecord {
        self.store.committed()
    }

    pub fn journal(&self) -> &[TransactionIntent] {
        self.store.journal()
    }

    pub fn begin(
        &self,
        command: LifecycleCommand,
        capabilities: &CapabilitySet,
        expected_generation: u64,
        expected_digest: impl Into<String>,
        provenance: impl Into<String>,
    ) -> Result<StagedTransaction, StoreError> {
        self.store.begin(
            command,
            capabilities,
            expected_generation,
            expected_digest,
            provenance,
        )
    }

    pub fn begin_review_recovery(
        &self,
        capabilities: &CapabilitySet,
        expected_generation: u64,
        expected_digest: impl Into<String>,
        provenance: ReviewRecoveryProvenance,
    ) -> Result<StagedTransaction, StoreError> {
        self.store.begin_review_recovery(
            capabilities,
            expected_generation,
            expected_digest,
            provenance,
        )
    }

    pub fn commit(
        &mut self,
        transaction: StagedTransaction,
        projection_write: ProjectionWrite,
    ) -> Result<CommitResult, StoreError> {
        self.ensure_current_intent(&transaction.intent)?;
        append_intent(&self.directory, &transaction.intent)?;
        let mut next_store = self.store.clone();
        let result = next_store.commit(transaction, projection_write)?;
        write_state_atomically(&self.directory, next_store.committed())?;
        sync_directory(&self.directory)?;
        self.store = next_store;
        Ok(result)
    }

    pub fn commit_then_project(
        &mut self,
        transaction: StagedTransaction,
        projection_write: impl FnOnce(&StateRecord) -> Result<(), StoreError>,
    ) -> Result<CommitResult, StoreError> {
        self.ensure_current_intent(&transaction.intent)?;
        append_intent(&self.directory, &transaction.intent)?;
        let mut next_store = self.store.clone();
        next_store.commit(transaction, ProjectionWrite::Success)?;
        next_store.committed.projections_repair_required = true;
        next_store.committed.refresh_digest();
        write_state_atomically(&self.directory, next_store.committed())?;
        sync_directory(&self.directory)?;
        let mut projected_state = next_store.committed().clone();
        projected_state.projections_repair_required = false;
        projected_state.refresh_digest();
        if projection_write(&projected_state).is_err() {
            self.store = next_store;
            return Ok(CommitResult::ProjectionRepairRequired(
                self.store.committed().clone(),
            ));
        }
        next_store.committed = projected_state;
        write_state_atomically(&self.directory, next_store.committed())?;
        sync_directory(&self.directory)?;
        self.store = next_store;
        Ok(CommitResult::Committed(self.store.committed().clone()))
    }

    fn ensure_current_intent(&self, intent: &TransactionIntent) -> Result<(), StoreError> {
        if intent.expected_generation != self.store.committed().generation
            || intent.expected_digest != self.store.committed().digest
        {
            return Err(StoreError::StaleWriter {
                expected_generation: intent.expected_generation,
                actual_generation: self.store.committed().generation,
            });
        }
        if self.store.committed().projections_repair_required {
            return Err(StoreError::ProjectionRepairRequired);
        }
        Ok(())
    }
}

fn classify_open_recovery(
    committed: &StateRecord,
    latest_intent: Option<&TransactionIntent>,
) -> Result<(), StoreError> {
    let classification = match latest_intent {
        None => classify_recovery(RecoveryObservation::NoIntent {
            state: committed.clone(),
        }),
        Some(intent) if committed.generation == intent.expected_generation => {
            classify_recovery(RecoveryObservation::IntentWithoutCommit {
                prior: committed.clone(),
                intent: intent.clone(),
            })
        }
        Some(intent) if committed.generation == intent.expected_generation + 1 => {
            classify_recovery(RecoveryObservation::StateCommitted {
                state: committed.clone(),
                intent: intent.clone(),
            })
        }
        Some(intent) => classify_recovery(RecoveryObservation::StateCommittedProjectionMissing {
            state: committed.clone(),
            intent: intent.clone(),
        }),
    };
    match classification {
        RecoveryClassification::NewState(_) => Ok(()),
        RecoveryClassification::PriorState(_) => Err(StoreError::RecoveryRequired(
            RecoveryRepair::ExactReadbackBeforeRemoteResume,
        )),
        RecoveryClassification::RepairRequired { repair, .. } => {
            Err(StoreError::RecoveryRequired(repair))
        }
        RecoveryClassification::CorruptRecoveryInput { reason } => Err(StoreError::Io(format!(
            "recovery classification failed: {reason:?}"
        ))),
    }
}

#[derive(Debug)]
struct StoreLock {
    path: PathBuf,
}

impl StoreLock {
    fn acquire(directory: &Path) -> Result<Self, StoreError> {
        let path = directory.join("state.lock");
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut lock) => {
                writeln!(lock, "pid={}", std::process::id()).map_err(io_error)?;
                lock.sync_all().map_err(io_error)?;
                Ok(Self { path })
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                Err(StoreError::LockUnavailable)
            }
            Err(error) => Err(io_error(error)),
        }
    }
}

impl Drop for StoreLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn append_intent(directory: &Path, intent: &TransactionIntent) -> Result<(), StoreError> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(directory.join("intents.jsonl"))
        .map_err(io_error)?;
    writeln!(file, "{}", intent_json(intent)).map_err(io_error)?;
    file.sync_all().map_err(io_error)
}

fn write_state_atomically(directory: &Path, state: &StateRecord) -> Result<(), StoreError> {
    let temp_path = directory.join("state.json.tmp");
    let final_path = directory.join("state.json");
    {
        let mut file = File::create(&temp_path).map_err(io_error)?;
        write!(file, "{}", state_json(state)).map_err(io_error)?;
        file.sync_all().map_err(io_error)?;
    }
    fs::rename(&temp_path, &final_path).map_err(io_error)
}

fn read_state(directory: &Path) -> Result<StateRecord, StoreError> {
    let bytes = fs::read(directory.join("state.json")).map_err(io_error)?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| StoreError::Io(error.to_string()))?;
    let generation = value
        .get("generation")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| StoreError::Io("state generation missing".into()))?;
    let digest = value
        .get("digest")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| StoreError::Io("state digest missing".into()))?
        .to_owned();
    let state = parse_state(
        value
            .get("state")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| StoreError::Io("state value missing".into()))?,
    )?;
    let projections_repair_required = value
        .get("projections_repair_required")
        .and_then(serde_json::Value::as_bool)
        .ok_or_else(|| StoreError::Io("projection repair flag missing".into()))?;
    let invalidated_projections = value
        .get("invalidated_projections")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| StoreError::Io("invalidated projections missing".into()))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| StoreError::Io("invalidated projection is not a string".into()))
                .and_then(parse_invalidation)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let audit = value
        .get("audit")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| StoreError::Io("audit missing".into()))?
        .iter()
        .map(parse_audit_event)
        .collect::<Result<Vec<_>, _>>()?;
    let record = StateRecord {
        generation,
        digest,
        state,
        audit,
        projections_repair_required,
        invalidated_projections,
    };
    validate_state_record(&record).map_err(|reason| StoreError::Io(format!("{reason:?}")))?;
    Ok(record)
}

fn read_intents(directory: &Path) -> Result<Vec<TransactionIntent>, StoreError> {
    let path = directory.join("intents.jsonl");
    if !path.exists() {
        return Ok(Vec::new());
    }
    fs::read_to_string(path)
        .map_err(io_error)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_intent)
        .collect()
}

fn parse_audit_event(value: &serde_json::Value) -> Result<AuditEvent, StoreError> {
    Ok(AuditEvent {
        generation: value
            .get("generation")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| StoreError::Io("audit generation missing".into()))?,
        command: parse_command(required_string(value, "command")?)?,
        from: parse_state(required_string(value, "from")?)?,
        to: parse_state(required_string(value, "to")?)?,
        invalidates: value
            .get("invalidates")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| StoreError::Io("audit invalidations missing".into()))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| StoreError::Io("audit invalidation is not a string".into()))
                    .and_then(parse_invalidation)
            })
            .collect::<Result<Vec<_>, _>>()?,
        provenance: required_string(value, "provenance")?.to_owned(),
    })
}

fn parse_intent(line: &str) -> Result<TransactionIntent, StoreError> {
    let value: serde_json::Value =
        serde_json::from_str(line).map_err(|error| StoreError::Io(error.to_string()))?;
    Ok(TransactionIntent {
        expected_generation: value
            .get("expected_generation")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| StoreError::Io("intent generation missing".into()))?,
        expected_digest: required_string(&value, "expected_digest")?.to_owned(),
        command: parse_command(required_string(&value, "command")?)?,
        provenance: required_string(&value, "provenance")?.to_owned(),
    })
}

fn required_string<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str, StoreError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| StoreError::Io(format!("{key} missing")))
}

fn parse_state(value: &str) -> Result<LifecycleState, StoreError> {
    match value {
        "initialized" => Ok(LifecycleState::Initialized),
        "ready" => Ok(LifecycleState::Ready),
        "bound" => Ok(LifecycleState::Bound),
        "implemented" => Ok(LifecycleState::Implemented),
        "reviewed" => Ok(LifecycleState::Reviewed),
        "published" => Ok(LifecycleState::Published),
        "merge_ready" => Ok(LifecycleState::MergeReady),
        "merged" => Ok(LifecycleState::Merged),
        "closed_out" => Ok(LifecycleState::ClosedOut),
        _ => Err(StoreError::Io(format!("unknown lifecycle state {value}"))),
    }
}

fn parse_command(value: &str) -> Result<LifecycleCommand, StoreError> {
    match value {
        "Bind" => Ok(LifecycleCommand::Bind),
        "RecordImplementation" => Ok(LifecycleCommand::RecordImplementation),
        "AssignReview" => Ok(LifecycleCommand::AssignReview),
        "RecordReviewPass" => Ok(LifecycleCommand::RecordReviewPass),
        "RecoverReview" => Ok(LifecycleCommand::RecoverReview),
        "Publish" => Ok(LifecycleCommand::Publish),
        "MarkMergeReady" => Ok(LifecycleCommand::MarkMergeReady),
        "RecordMerge" => Ok(LifecycleCommand::RecordMerge),
        "Finish" => Ok(LifecycleCommand::Finish),
        "Cleanup" => Ok(LifecycleCommand::Cleanup),
        _ => Err(StoreError::Io(format!("unknown lifecycle command {value}"))),
    }
}

fn parse_invalidation(value: &str) -> Result<ProjectionInvalidation, StoreError> {
    match value {
        "Readiness" => Ok(ProjectionInvalidation::Readiness),
        "Review" => Ok(ProjectionInvalidation::Review),
        "Publication" => Ok(ProjectionInvalidation::Publication),
        "Terminal" => Ok(ProjectionInvalidation::Terminal),
        "CleanupEligibility" => Ok(ProjectionInvalidation::CleanupEligibility),
        _ => Err(StoreError::Io(format!(
            "unknown projection invalidation {value}"
        ))),
    }
}

fn sync_directory(directory: &Path) -> Result<(), StoreError> {
    File::open(directory)
        .and_then(|directory| directory.sync_all())
        .map_err(io_error)
}

fn state_json(state: &StateRecord) -> serde_json::Value {
    serde_json::json!({
        "schema": "csdlc.v3.state_record.v1",
        "generation": state.generation,
        "digest": state.digest,
        "state": state.state.to_string(),
        "projections_repair_required": state.projections_repair_required,
        "invalidated_projections": state.invalidated_projections.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>(),
        "audit": state.audit.iter().map(audit_json).collect::<Vec<_>>(),
    })
}

fn audit_json(event: &AuditEvent) -> serde_json::Value {
    serde_json::json!({
        "generation": event.generation,
        "command": format!("{:?}", event.command),
        "from": event.from.to_string(),
        "to": event.to.to_string(),
        "invalidates": event.invalidates.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>(),
        "provenance": event.provenance,
    })
}

fn intent_json(intent: &TransactionIntent) -> serde_json::Value {
    serde_json::json!({
        "schema": "csdlc.v3.transaction_intent.v1",
        "expected_generation": intent.expected_generation,
        "expected_digest": intent.expected_digest,
        "command": format!("{:?}", intent.command),
        "provenance": intent.provenance,
    })
}

fn io_error(error: std::io::Error) -> StoreError {
    StoreError::Io(error.to_string())
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
            if !state.projections_repair_required {
                return RecoveryClassification::CorruptRecoveryInput {
                    reason: RecoveryRejectReason::RepairIntentMissing,
                };
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
            if state.projections_repair_required {
                return RecoveryClassification::RepairRequired {
                    state,
                    intent,
                    repair: RecoveryRepair::RegenerateProjections,
                };
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
    format!("v3:{}", blake3::hash(canonical.as_bytes()).to_hex())
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

fn escape_digest_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(':', "\\:")
}
