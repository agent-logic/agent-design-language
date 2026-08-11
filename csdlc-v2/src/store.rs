use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{
    apply, digest, initial_cards, render, terminal_validation_passed, validate_cross_card,
    validate_result, CardContent, CardKind, CardStatus, CardValues, InitialCardInput,
    SemanticOperation, StepStatus, ValidationResult,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{
    AuditEvent, CardProjection, DesignReview, IssueRecord, LifecyclePhase, PublicationEvidence,
    ReviewAssignment, ReviewEvidence, TerminalEvidence, TerminalReceipt, TransitionEvent,
};
use crate::review::evaluate_publication_review_in_repo;

#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) struct ImplementationCommit {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub summary: String,
    pub changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub validation: Vec<ValidationResult>,
}

#[derive(Debug, Clone)]
pub(crate) struct ReviewCommit {
    pub issue: u64,
    pub expected_digest: String,
    pub actor: String,
    pub evidence: ReviewEvidence,
    pub result: crate::cards::ReviewResult,
    pub advance_reviewed: bool,
}

impl Store {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn issue_dir(&self, issue: u64) -> PathBuf {
        self.root.join(".csdlc/issues").join(issue.to_string())
    }

    pub fn interrupted_backup(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.backup"))
    }

    fn staging_dir(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.staging"))
    }

    pub(crate) fn lock(&self, issue: u64) -> Result<File> {
        let relative = PathBuf::from(format!(".csdlc/locks/{issue}.lock"));
        require_canonical_parent_beneath(&self.root, &relative)?;
        let dir = self.root.join(".csdlc/locks");
        fs::create_dir_all(&dir)?;
        require_canonical_parent_beneath(&self.root, &relative)?;
        require_regular_or_absent_beneath(&self.root, &relative)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.root.join(&relative))?;
        file.lock_exclusive()?;
        require_canonical_parent_beneath(&self.root, &relative)?;
        require_regular_or_absent_beneath(&self.root, &relative)?;
        Ok(file)
    }

    pub fn authority_projection_lock(&self, issue: u64) -> Result<File> {
        self.lock(issue)
    }

    pub(crate) fn binding_lock(&self) -> Result<File> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        let dir = PathBuf::from(common).join("csdlc-v2");
        fs::create_dir_all(&dir)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(dir.join("bindings.lock"))?;
        file.lock_exclusive()?;
        Ok(file)
    }

    pub fn load_record(&self, issue: u64) -> Result<IssueRecord> {
        let record: IssueRecord =
            serde_json::from_slice(&fs::read(self.issue_dir(issue).join("index.json"))?)?;
        if record.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!(
                    "issue projection namespace mismatch: requested {issue}, embedded {}",
                    record.issue
                ),
            ));
        }
        Ok(record)
    }

    pub(crate) fn load_record_for_topology_scan(&self, issue: u64) -> Result<IssueRecord> {
        let path = self.issue_dir(issue).join("index.json");
        let record: IssueRecord = serde_json::from_slice(&fs::read(path)?)?;
        if record.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "issue projection namespace mismatch during topology scan",
            ));
        }
        verify_record_with_options(&record, true)?;
        Ok(record)
    }

    pub fn load_cards(&self, issue: u64) -> Result<BTreeMap<CardKind, CardValues>> {
        let mut cards = BTreeMap::new();
        for kind in enum_iterator() {
            let path = self
                .issue_dir(issue)
                .join("cards")
                .join(format!("{kind}.values.json"));
            cards.insert(kind, read_json(&path)?);
        }
        Ok(cards)
    }

    fn git_common_relative(&self, path: &Path) -> Result<(PathBuf, PathBuf)> {
        let common = PathBuf::from(
            crate::git::run(
                &self.root,
                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
            )?
            .stdout,
        );
        let relative = path.strip_prefix(&common).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal receipt path escapes its Git-common root",
            )
        })?;
        if !crate::pvf::clean_relative(relative) {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal receipt path is not clean beneath its Git-common root",
            ));
        }
        Ok((common, relative.to_path_buf()))
    }

    pub fn terminal_receipt_path(&self, issue: u64) -> Result<PathBuf> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        Ok(PathBuf::from(common)
            .join("csdlc-v2/closeout")
            .join(format!("{issue}.json")))
    }

    pub fn load_terminal_receipt(&self, issue: u64) -> Result<Option<TerminalReceipt>> {
        let path = self.terminal_receipt_path(issue)?;
        let (common, relative) = self.git_common_relative(&path)?;
        let Some(metadata) = canonical_path_metadata_beneath(&common, &relative)? else {
            return Ok(None);
        };
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal receipt is not a canonical regular file: {}",
                    path.display()
                ),
            ));
        }
        let receipt: TerminalReceipt = read_json(&path)?;
        validate_terminal_receipt(&receipt)?;
        if receipt.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!(
                    "terminal receipt namespace mismatch: requested {issue}, embedded {}",
                    receipt.issue
                ),
            ));
        }
        Ok(Some(receipt))
    }

    fn legacy_receipt_matches_projection(&self, receipt: &TerminalReceipt) -> Result<bool> {
        let local = self.load_record(receipt.issue)?;
        let cards = self.load_cards(receipt.issue)?;
        if receipt.record != local
            || receipt.cards != cards
            || verify_cards(self, &local, &cards).is_err()
        {
            return Ok(false);
        }
        for (path, expected) in &receipt.authored_artifacts {
            let Some(actual) = read_regular_authored_artifact(&self.root, Path::new(path))? else {
                return Ok(false);
            };
            if actual != expected.as_bytes() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn recover_local_transaction(&self, issue: u64) -> Result<()> {
        let current = self.issue_dir(issue);
        let backup = self.interrupted_backup(issue);
        let staging = self.staging_dir(issue);
        if !current.exists() && backup.exists() {
            fs::rename(&backup, &current)?;
        }
        if staging.exists() {
            fs::remove_dir_all(staging)?;
        }
        Ok(())
    }

    fn recover_if_needed(&self, issue: u64) -> Result<()> {
        self.recover_local_transaction(issue)
    }

    fn commit(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
    ) -> Result<()> {
        self.commit_with_authored(issue, record, cards, fail_after_backup, None)
    }

    fn commit_with_authored(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
        authored_overrides: Option<&BTreeMap<String, String>>,
    ) -> Result<()> {
        let current = self.issue_dir(issue);
        let staging = self.staging_dir(issue);
        let backup = self.interrupted_backup(issue);
        if staging.exists() {
            fs::remove_dir_all(&staging)?;
        }
        if backup.exists() {
            fs::remove_dir_all(&backup)?;
        }
        write_complete(&staging, record, cards)?;
        // Preserve authored design artifacts when they live inside the issue
        // directory. The atomic directory swap must not discard them.
        for authored_path in [&record.design_path, &record.diagram_path] {
            let source = self.root.join(authored_path);
            if let Ok(relative) = source.strip_prefix(&current) {
                let destination = staging.join(relative);
                if let Some(parent) = destination.parent() {
                    fs::create_dir_all(parent)?;
                }
                if let Some(contents) =
                    authored_overrides.and_then(|overrides| overrides.get(authored_path))
                {
                    let mut file = File::create(destination)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
                } else if source.is_file() {
                    fs::copy(source, destination)?;
                }
            }
        }
        if let Some(overrides) = authored_overrides {
            for (authored_path, contents) in overrides {
                if !crate::pvf::clean_relative(Path::new(authored_path)) {
                    return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "authored override path must be repository-relative",
                    ));
                }
                let destination = self.root.join(authored_path);
                if let Ok(relative) = destination.strip_prefix(&current) {
                    let staged = staging.join(relative);
                    if let Some(parent) = staged.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut file = File::create(staged)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
                }
            }
        }
        if current.exists() {
            fs::rename(&current, &backup)?;
            sync_dir(current.parent().expect("issue parent"))?;
        }
        if fail_after_backup {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected interruption after preserving complete prior generation",
            ));
        }
        fs::rename(&staging, &current)?;
        sync_dir(current.parent().expect("issue parent"))?;
        if let Some(overrides) = authored_overrides {
            for (authored_path, contents) in overrides {
                let destination = self.root.join(authored_path);
                if destination.strip_prefix(&current).is_ok() {
                    continue;
                }
                destination.parent().ok_or_else(|| {
                    V2Error::new(ErrorCode::InvalidInput, "authored override has no parent")
                })?;
                replace_regular_authored_artifact(
                    &self.root,
                    Path::new(authored_path),
                    contents.as_bytes(),
                    "authored-commit-tmp",
                )?;
            }
        }
        if backup.exists() {
            fs::remove_dir_all(&backup)?;
            sync_dir(current.parent().expect("issue parent"))?;
        }
        Ok(())
    }

    pub(crate) fn replace_record(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<()> {
        let _lock = self.lock(issue)?;
        self.replace_record_locked(issue, expected_digest, record)
    }

    pub(crate) fn replace_record_locked(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<()> {
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before compare-and-swap commit",
            ));
        }
        let cards = self.load_cards(issue)?;
        verify_cards(self, &current, &cards)?;
        self.commit(issue, record, &cards, false)
    }

    pub(crate) fn replace_pre_topology_record_locked(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<()> {
        self.recover_if_needed(issue)?;
        let current = self.load_record_for_topology_scan(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before topology migration commit",
            ));
        }
        let cards = self.load_cards(issue)?;
        verify_pre_topology_cards(self, &current, &cards)?;
        self.commit(issue, record, &cards, false)
    }

    pub(crate) fn materialize_terminal_from_derived(
        &self,
        issue: u64,
        expected_generation: u64,
        expected_digest: &str,
        actor: &str,
        reason: &str,
        envelope: &crate::finish::DerivedTerminalEnvelope,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != expected_generation {
            return Err(V2Error::new(
                ErrorCode::StaleGeneration,
                "terminal materialization generation is stale",
            ));
        }
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal materialization digest is stale",
            ));
        }
        if actor.trim().is_empty() || reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal materialization actor and reason are required",
            ));
        }
        let source_projection_match = envelope.issue == record.issue
            && envelope.repository == record.repository
            && envelope.initialization_digest == record.initialization_digest
            && envelope.canonical_generation == record.generation
            && envelope.canonical_digest == record.digest;
        let already_materialized_match = terminal_matches_derived(&record, envelope);
        if !source_projection_match && !already_materialized_match {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal envelope does not match the expected source projection",
            ));
        }
        crate::finish::validate_envelope(envelope)?;
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        verify_canonical_projection_bytes(self, &record, &cards)?;
        let terminal_cards_complete = terminal_cards_match_disposition(
            &cards,
            terminal_disposition_from_finish(envelope.disposition),
        );
        if already_materialized_match && terminal_cards_complete {
            if self
                .load_terminal_receipt(issue)?
                .as_ref()
                .is_some_and(|receipt| {
                    self.legacy_receipt_matches_projection(receipt)
                        .unwrap_or(false)
                })
            {
                return Ok(record);
            }
            let receipt = self.build_terminal_receipt(issue, &record, &cards)?;
            self.write_terminal_receipt(issue, &receipt)?;
            return Ok(record);
        }
        let rollback_record = record.clone();
        let rollback_cards = cards.clone();

        let branch = record.branch.clone();
        let worktree = record.worktree.clone();
        match envelope.disposition {
            crate::finish::FinishDisposition::Merged
            | crate::finish::FinishDisposition::ClosedUnmerged => {
                let publication = record.publication.as_ref().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "PR terminal materialization requires publication evidence",
                    )
                })?;
                if Some(publication.pull_request) != envelope.pull_request {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "terminal materialization PR does not match publication evidence",
                    ));
                }
            }
            crate::finish::FinishDisposition::ClosedNoPr => {
                if record.publication.is_some() || envelope.pull_request.is_some() {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "closed-no-PR materialization cannot contain publication evidence",
                    ));
                }
            }
        }
        let design_bytes = fs::read(self.root.join(&record.design_path))?;
        let diagram_bytes = fs::read(self.root.join(&record.diagram_path))?;
        let design_digest = digest(&design_bytes);
        let diagram_digest = digest(&diagram_bytes);
        if let DesignReview::Approved { revision, .. } = &mut record.design_review {
            *revision = design_digest.clone();
        }
        match &mut cards.get_mut(&CardKind::Spp).expect("SPP").content {
            CardContent::Spp(values) => {
                values.design_ref = record.design_path.clone();
                values.design_digest = design_digest.clone();
                values.diagram_ref = record.diagram_path.clone();
                values.diagram_digest = diagram_digest.clone();
                for step in &mut values.steps {
                    step.status = StepStatus::Completed;
                }
            }
            _ => unreachable!("SPP"),
        }
        match &mut cards.get_mut(&CardKind::Vpp).expect("VPP").content {
            CardContent::Vpp(values) => {
                values.design_ref = record.design_path.clone();
                values.design_digest = design_digest;
                values.diagram_ref = record.diagram_path.clone();
                values.diagram_digest = diagram_digest;
            }
            _ => unreachable!("VPP"),
        }

        let (terminal_disposition, observed_state, integration_state, merge_state) =
            match envelope.disposition {
                crate::finish::FinishDisposition::Merged => (
                    crate::readiness::TerminalDisposition::Merged,
                    "merged",
                    crate::cards::IntegrationState::Merged,
                    crate::cards::MergeState::Merged,
                ),
                crate::finish::FinishDisposition::ClosedUnmerged => (
                    crate::readiness::TerminalDisposition::ClosedUnmerged,
                    "closed_unmerged",
                    crate::cards::IntegrationState::ClosedNoPr,
                    crate::cards::MergeState::ClosedUnmerged,
                ),
                crate::finish::FinishDisposition::ClosedNoPr => (
                    crate::readiness::TerminalDisposition::ClosedNoPr,
                    "closed_no_pr",
                    crate::cards::IntegrationState::ClosedNoPr,
                    crate::cards::MergeState::ClosedUnmerged,
                ),
            };
        if let Some(publication) = record.publication.as_mut() {
            publication.observed_state = observed_state.into();
        }

        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.integration_state = integration_state;
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.merge_state = merge_state;
        sor.closeout_state = crate::cards::CloseoutState::Complete;
        cards.get_mut(&CardKind::Sor).expect("SOR").status = CardStatus::Complete;

        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.terminal = Some(TerminalEvidence {
            pull_request: envelope.pull_request,
            disposition: terminal_disposition,
            observed_sha: envelope.head_sha.clone(),
            observed_state: observed_state.into(),
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            branch,
            worktree,
        });
        match envelope.disposition {
            crate::finish::FinishDisposition::Merged => match record.phase {
                LifecyclePhase::Published => {
                    push_legacy_terminal_transition(
                        &mut record,
                        LifecyclePhase::MergeReady,
                        actor,
                        "observed required checks, review, and conflict readiness",
                    );
                    push_legacy_terminal_transition(
                        &mut record,
                        LifecyclePhase::Merged,
                        actor,
                        "observed exact PR merged",
                    );
                }
                LifecyclePhase::MergeReady => push_legacy_terminal_transition(
                    &mut record,
                    LifecyclePhase::Merged,
                    actor,
                    "observed exact PR merged",
                ),
                LifecyclePhase::Merged | LifecyclePhase::ClosedOut => {}
                _ => {
                    return Err(V2Error::new(
                        ErrorCode::InvalidTransition,
                        "merged materialization requires published, merge_ready, or merged phase",
                    ));
                }
            },
            crate::finish::FinishDisposition::ClosedUnmerged
            | crate::finish::FinishDisposition::ClosedNoPr => match record.phase {
                LifecyclePhase::Implemented
                | LifecyclePhase::Reviewed
                | LifecyclePhase::Published
                | LifecyclePhase::MergeReady
                | LifecyclePhase::ClosedOut => {}
                LifecyclePhase::Merged => {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "an already-merged issue cannot materialize an unmerged disposition",
                    ));
                }
                _ => {
                    return Err(V2Error::new(
                        ErrorCode::InvalidTransition,
                        "unmerged materialization requires implemented, reviewed, published, or merge_ready phase",
                    ));
                }
            },
        }
        if record.phase != LifecyclePhase::ClosedOut {
            push_legacy_terminal_transition(&mut record, LifecyclePhase::ClosedOut, actor, reason);
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: actor.into(),
            reason: reason.into(),
            operation: "materialize_derived_terminal".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let receipt = self.build_terminal_receipt(issue, &record, &cards)?;
        self.commit(issue, &record, &cards, false)?;
        if let Err(error) = self.write_terminal_receipt(issue, &receipt) {
            self.commit(issue, &rollback_record, &rollback_cards, false)?;
            verify_cards(self, &rollback_record, &rollback_cards)?;
            return Err(error);
        }
        Ok(record)
    }

    fn build_terminal_receipt(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<TerminalReceipt> {
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{issue}.json"),
            authored_artifacts: BTreeMap::new(),
            record: record.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        for authored_path in [&record.design_path, &record.diagram_path] {
            let bytes = read_regular_projection(&self.root, Path::new(authored_path))?;
            receipt.authored_artifacts.insert(
                authored_path.clone(),
                String::from_utf8(bytes).map_err(|error| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        format!("terminal authored artifact is not UTF-8: {error}"),
                    )
                })?,
            );
        }
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        Ok(receipt)
    }

    fn write_terminal_receipt(&self, issue: u64, receipt: &TerminalReceipt) -> Result<()> {
        let receipt_path = self.terminal_receipt_path(issue)?;
        let (common, relative) = self.git_common_relative(&receipt_path)?;
        replace_regular_authored_artifact(
            &common,
            &relative,
            &serde_json::to_vec_pretty(&receipt)?,
            "tmp",
        )
    }

    pub(crate) fn commit_migration(
        &self,
        issue: u64,
        expected_digest: &str,
        evidence: crate::model::MigrationEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "migration record changed before commit",
            ));
        }
        if let Some(existing) = &record.migration {
            if existing == &evidence {
                return Ok(record);
            }
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "existing migration evidence differs from the source digest or retained authored truth",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.migration = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: "csdlc-import".into(),
            reason: "attach one-way legacy authored-content evidence and sunset metadata".into(),
            operation: "record_migration".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_code_repository_migration(
        &self,
        request: &crate::migration::CodeRepositoryMigrationRequest,
    ) -> Result<(
        IssueRecord,
        crate::migration::CodeRepositoryMigrationEvidence,
    )> {
        let _lock = self.lock(request.issue)?;
        self.recover_if_needed(request.issue)?;
        let mut record = self.load_record(request.issue)?;
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        authorize_code_repository_migration(&self.root, &record, request)?;

        // Re-read and reauthorize immediately before mutation while both the
        // binding and issue locks remain held.
        record = self.load_record(request.issue)?;
        let evidence = authorize_code_repository_migration(&self.root, &record, request)?;
        record.code_repository = Some(request.code_repository.clone());
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.actor.clone(),
            reason: request.reason.clone(),
            operation: serde_json::to_string(&evidence)?,
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(request.issue, &record, &cards, false)?;
        Ok((record, evidence))
    }

    pub(crate) fn commit_publication(
        &self,
        issue: u64,
        expected_digest: &str,
        actor: String,
        evidence: PublicationEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "publication record changed before commit",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.integration_state = crate::cards::IntegrationState::PrOpen;
        sor.merge_state = crate::cards::MergeState::NotMerged;
        sor.publication_state = if evidence.draft {
            crate::cards::PublicationState::Draft
        } else {
            crate::cards::PublicationState::Ready
        };
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.publication = Some(evidence);
        if record.phase == LifecyclePhase::Reviewed {
            record.advance(
                LifecyclePhase::Published,
                actor.clone(),
                "observed exact PR after current review".into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "atomically record observed GitHub publication and SOR projection".into(),
            operation: "record_publication".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_implementation(
        &self,
        commit: ImplementationCommit,
        staged_evidence: &Path,
        evidence_dir: &Path,
    ) -> Result<IssueRecord> {
        let issue = commit.issue;
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != commit.expected_generation
            || record.digest != commit.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "implementation finalization changed before commit",
            ));
        }
        if record.phase != LifecyclePhase::Bound || commit.validation.is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "implementation finalization requires bound phase and validation evidence",
            ));
        }
        for result in &commit.validation {
            validate_result(result)?;
        }
        if !terminal_validation_passed(&commit.validation) {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                "implementation finalization requires passing validation evidence",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.summary = commit.summary;
        sor.actual_changes.extend(commit.changes);
        sor.artifacts.extend(commit.artifacts);
        sor.actual_validation.extend(commit.validation);
        record.advance(
            LifecyclePhase::Implemented,
            commit.actor.clone(),
            "execution and passing validation finalized atomically".into(),
        )?;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: commit.actor,
            reason: "atomically record execution, validation, and implemented phase".into(),
            operation: "finalize_implementation".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let staged_metadata = fs::symlink_metadata(staged_evidence).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "staged finalize evidence is missing",
            )
        })?;
        if staged_metadata.file_type().is_symlink() || !staged_metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "staged finalize evidence must be a real directory",
            ));
        }
        let evidence_parent = evidence_dir.parent().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "evidence directory has no parent")
        })?;
        let backup = evidence_parent.join(format!(
            ".csdlc-finalize-backup-{issue}-{}",
            std::process::id()
        ));
        if backup.exists() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "stale finalize evidence backup requires reconciliation",
            ));
        }
        let had_evidence = evidence_dir.exists();
        if had_evidence {
            fs::rename(evidence_dir, &backup)?;
        }
        if let Err(error) = fs::rename(staged_evidence, evidence_dir) {
            if had_evidence && fs::rename(&backup, evidence_dir).is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "failed to restore evidence after finalize publication error",
                ));
            }
            return Err(error.into());
        }
        if fs::symlink_metadata(evidence_dir)
            .map(|metadata| metadata.file_type().is_symlink() || !metadata.is_dir())
            .unwrap_or(true)
        {
            let _ = fs::remove_file(evidence_dir);
            if had_evidence && fs::rename(&backup, evidence_dir).is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "failed to restore evidence after unsafe finalize publication",
                ));
            }
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "published finalize evidence must be a real directory",
            ));
        }
        if let Err(error) = self.commit(issue, &record, &cards, false) {
            let remove_result = fs::remove_dir_all(evidence_dir);
            let restore_result = if had_evidence {
                fs::rename(&backup, evidence_dir)
            } else {
                Ok(())
            };
            if remove_result.is_err() || restore_result.is_err() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!(
                        "state commit failed and evidence rollback requires reconciliation: {}",
                        error.message
                    ),
                ));
            }
            return Err(error);
        }
        if had_evidence {
            fs::remove_dir_all(&backup)?;
        }
        Ok(record)
    }

    pub(crate) fn commit_review(&self, commit: ReviewCommit) -> Result<IssueRecord> {
        let issue = commit.issue;
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != commit.expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review record changed before commit",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.reviewer = Some(commit.evidence.reviewer.clone());
        srp.review_scope = commit.evidence.scope.join("\n");
        srp.review_revision = Some(commit.evidence.reviewed_revision.clone());
        srp.review_result = commit.result;
        srp.residual_risk = commit.evidence.residual_risks.clone();
        srp.findings = commit
            .evidence
            .findings
            .iter()
            .map(|finding| crate::cards::ReviewFinding {
                id: finding.id.clone(),
                severity: finding.severity,
                summary: finding.summary.clone(),
                actionable: finding.actionable,
                in_scope: finding.in_scope,
                disposition: finding.disposition,
                fix_revision: finding.fix_revision.clone(),
                route: finding.route.clone(),
            })
            .collect();
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.review = Some(commit.evidence);
        if commit.advance_reviewed && commit.result == crate::cards::ReviewResult::Pass {
            record.advance(
                LifecyclePhase::Reviewed,
                commit.actor.clone(),
                "exact scoped review passed".into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: commit.actor,
            reason: if commit.advance_reviewed {
                "atomically record exact review evidence and reviewed phase"
            } else {
                "atomically record assigned review evidence and SRP projection"
            }
            .into(),
            operation: "record_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_review_assignment(
        &self,
        issue: u64,
        expected_digest: &str,
        assignment: ReviewAssignment,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review assignment changed before commit",
            ));
        }
        if record.phase != LifecyclePhase::Implemented {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review assignment requires implemented phase",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        let actor = assignment.assigned_by.clone();
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.review_scope = assignment.scope.join("\n");
        record.review_assignment = Some(assignment);
        record.review = None;
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: "assign bounded exact-revision review".into(),
            operation: "assign_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_review_recovery(
        &self,
        issue: u64,
        expected_generation: u64,
        expected_digest: &str,
        actor: String,
        reason: String,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        if record.generation != expected_generation {
            return Err(V2Error::new(
                ErrorCode::StaleGeneration,
                "review recovery generation changed before commit",
            ));
        }
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "review recovery record changed before commit",
            ));
        }
        let implemented_with_review_truth = record.phase == LifecyclePhase::Implemented
            && (record.review_assignment.is_some() || record.review.is_some());
        if !implemented_with_review_truth
            && !matches!(
                record.phase,
                LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
            )
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review recovery requires review truth or a reviewed/published phase",
            ));
        }
        let mut cards = self.load_cards(issue)?;
        verify_cards(self, &record, &cards)?;
        cards.get_mut(&CardKind::Srp).expect("SRP").status = crate::cards::CardStatus::Draft;
        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!("SRP"),
        };
        srp.review_scope.clear();
        srp.review_revision = None;
        srp.reviewer = None;
        srp.findings.clear();
        srp.residual_risk.clear();
        srp.review_result = crate::cards::ReviewResult::PreReview;
        if let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            sor.publication_state = crate::cards::PublicationState::NotPublished;
            sor.integration_state = crate::cards::IntegrationState::WorktreeOnly;
            sor.merge_state = crate::cards::MergeState::NotMerged;
            sor.closeout_state = crate::cards::CloseoutState::NotStarted;
        }
        if record.phase == LifecyclePhase::MergeReady {
            record.phase = LifecyclePhase::Implemented;
            record.transitions.push(TransitionEvent {
                sequence: record.transitions.len() as u64 + 1,
                from: LifecyclePhase::MergeReady,
                to: LifecyclePhase::Implemented,
                actor: actor.clone(),
                reason: reason.clone(),
            });
        } else if record.phase != LifecyclePhase::Implemented {
            record.advance(LifecyclePhase::Implemented, actor.clone(), reason.clone())?;
        }
        record.review_assignment = None;
        record.review = None;
        record.publication = None;
        record.readiness = None;
        record.terminal = None;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason,
            operation: "recover_review".into(),
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }
}

fn authorize_code_repository_migration(
    root: &Path,
    record: &IssueRecord,
    request: &crate::migration::CodeRepositoryMigrationRequest,
) -> Result<crate::migration::CodeRepositoryMigrationEvidence> {
    if record.issue != request.issue {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "code repository migration issue identity does not match",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "code repository migration digest is stale",
        ));
    }
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "code repository migration generation is stale",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Bound | LifecyclePhase::Implemented | LifecyclePhase::Reviewed
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "code repository migration requires bound, implemented, or reviewed phase",
        ));
    }
    if record.code_repository.is_some() {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "code repository migration requires an absent code_repository",
        ));
    }
    let branch = record.branch.clone().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "code repository migration requires a registered branch",
        )
    })?;
    let worktree = record.worktree.clone().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "code repository migration requires a registered worktree",
        )
    })?;
    let actual_root = fs::canonicalize(root)?;
    let registered_root = fs::canonicalize(&worktree).map_err(|error| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("registered worktree is unavailable: {error}"),
        )
    })?;
    if actual_root != registered_root || crate::git::current_branch(root)? != branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "invocation does not match the registered branch and canonical worktree",
        ));
    }
    let registered = crate::git::worktrees(root)?
        .into_iter()
        .filter(|(candidate_branch, candidate_path)| {
            candidate_branch == &branch
                && fs::canonicalize(candidate_path)
                    .is_ok_and(|candidate| candidate == registered_root)
        })
        .count();
    if registered != 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "registered branch and worktree topology is missing or ambiguous",
        ));
    }
    if !crate::git::worktree_is_clean(root)? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "code repository migration requires a clean tracked and untracked worktree",
        ));
    }
    let origins = crate::git::github_remote_repositories(root, "origin")?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "code repository migration requires an origin remote",
        )
    })?;
    let mut identities = origins.fetch.iter().chain(&origins.push);
    let canonical = identities.next().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "origin repository identity is unavailable",
        )
    })?;
    if identities.any(|identity| !identity.eq_ignore_ascii_case(canonical))
        || !request.code_repository.eq_ignore_ascii_case(canonical)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "requested code repository does not match every origin fetch and push identity",
        ));
    }
    Ok(crate::migration::CodeRepositoryMigrationEvidence {
        schema: "csdlc.code_repository_migration_evidence.v1".into(),
        issue: record.issue,
        actor: request.actor.clone(),
        reason: request.reason.clone(),
        pre_generation: record.generation,
        pre_digest: record.digest.clone(),
        previous_code_repository: None,
        requested_repository: request.code_repository.clone(),
        fetch_repositories: origins.fetch,
        push_repositories: origins.push,
        phase: record.phase,
        branch,
        worktree: registered_root.to_string_lossy().into_owned(),
        clean_worktree: true,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BootstrapRequest {
    pub issue: u64,
    pub repository: String,
    pub actor: String,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    #[serde(default)]
    pub design_approved: bool,
    pub initial: InitialCardInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditRequest {
    pub issue: u64,
    pub card: CardKind,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
    pub operation: SemanticOperation,
    #[serde(default)]
    pub fail_after_backup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ApproveDesignRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub reviewer: String,
}

pub fn approve_design(store: &Store, request: ApproveDesignRequest) -> Result<IssueRecord> {
    approve_design_with_hook(store, request, |_| {})
}

fn approve_design_with_hook(
    store: &Store,
    request: ApproveDesignRequest,
    mut authored_hook: impl FnMut(AuthoredReadStage),
) -> Result<IssueRecord> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "design approval generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "design approval digest is stale",
        ));
    }
    if request.reviewer.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design reviewer is required",
        ));
    }
    let mut cards = store.load_cards(request.issue)?;
    verify_card_projections(store, &record, &cards)?;
    let (design_digest, diagram_digest) = approval_authored_digests_with_hook(
        store,
        &record.design_path,
        &record.diagram_path,
        &mut authored_hook,
    )?;
    let initial_approval = record.phase == LifecyclePhase::Initialized
        && matches!(
            record.design_review,
            DesignReview::Pending | DesignReview::ChangesRequired { .. }
        );
    let initialized_reapproval = record.phase == LifecyclePhase::Initialized
        && matches!(record.design_review, DesignReview::Approved { .. })
        && [CardKind::Spp, CardKind::Vpp]
            .iter()
            .any(|kind| match &cards[kind].content {
                CardContent::Spp(values) => {
                    values.design_digest != design_digest || values.diagram_digest != diagram_digest
                }
                CardContent::Vpp(values) => {
                    values.design_digest != design_digest || values.diagram_digest != diagram_digest
                }
                _ => unreachable!("design-bearing card"),
            });
    let ready_reapproval = record.phase == LifecyclePhase::Ready
        && matches!(
            record.design_review,
            DesignReview::Pending | DesignReview::ChangesRequired { .. }
        )
        && record.branch.is_none()
        && record.worktree.is_none()
        && record.review_assignment.is_none()
        && record.review.is_none()
        && record.publication.is_none()
        && record.readiness.is_none()
        && record.migration.is_none()
        && record.terminal.is_none();
    let lifecycle_reapproval = matches!(
        record.phase,
        LifecyclePhase::Bound | LifecyclePhase::Implemented
    );
    if !initial_approval && !initialized_reapproval && !ready_reapproval && !lifecycle_reapproval {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design approval requires pending initialized review, stale initialized approved inputs, or bound/implemented reapproval",
        ));
    }
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("card").content {
            CardContent::Spp(values) => {
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_digest = design_digest.clone();
                values.diagram_digest = diagram_digest.clone();
            }
            _ => unreachable!("design-bearing card"),
        }
    }
    record.design_review = DesignReview::Approved {
        reviewer: request.reviewer.clone(),
        revision: design_digest,
    };
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.reviewer,
        reason: if ready_reapproval {
            "reapprove repaired ready issue design"
        } else if initialized_reapproval {
            "reapprove stale initialized issue design"
        } else if lifecycle_reapproval {
            "reapprove changed issue design"
        } else {
            "approve completed issue design"
        }
        .into(),
        operation: "approve_design".into(),
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub(crate) fn bootstrap_issue(store: &Store, request: BootstrapRequest) -> Result<IssueRecord> {
    validate_bootstrap_request(&request)?;
    let initialization_digest = digest(&serde_json::to_vec(&request)?);
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let index_path = store.issue_dir(request.issue).join("index.json");
    if index_path.exists() {
        let existing = store.load_record(request.issue)?;
        verify_cards(store, &existing, &store.load_cards(request.issue)?)?;
        if existing.initialization_digest == initialization_digest {
            return Ok(existing);
        }
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue exists with different initialization truth",
        ));
    }
    let bootstrap_actor = request.actor.clone();
    let design_digest = authored_digest(store, &request.design_path)?;
    let diagram_digest = authored_digest(store, &request.diagram_path)?;
    let cards = initial_cards(
        request.issue,
        &request.repository,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
        request.initial,
    )?;
    let mut record = IssueRecord {
        schema: "csdlc.issue.index.v1".into(),
        issue: request.issue,
        repository: request.repository,
        code_repository: None,
        initialization_digest,
        phase: LifecyclePhase::Initialized,
        generation: 0,
        digest: String::new(),
        branch: None,
        worktree: None,
        review_assignment: None,
        review: None,
        publication: None,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: request.design_path,
        diagram_path: request.diagram_path,
        design_review: if request.design_approved {
            DesignReview::Approved {
                reviewer: request.design_reviewer,
                revision: design_digest,
            }
        } else {
            DesignReview::Pending
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: vec![AuditEvent {
            sequence: 1,
            generation: 0,
            actor: bootstrap_actor,
            reason: "initialize issue record and all six cards".into(),
            operation: "bootstrap".into(),
        }],
    };
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub(crate) fn validate_bootstrap_request(request: &BootstrapRequest) -> Result<()> {
    if request.issue == 0 || request.repository.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue and repository are required",
        ));
    }
    if (request.design_approved && request.design_reviewer.trim().is_empty())
        || request.actor.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bootstrap actor/reviewer invariants are incomplete",
        ));
    }
    Ok(())
}

pub fn edit_issue(store: &Store, request: EditRequest) -> Result<IssueRecord> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "expected generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "expected issue digest is stale",
        ));
    }
    let mut cards = store.load_cards(request.issue)?;
    let prebind_contract_repair = is_prebind_contract_repair(&record, &request);
    let prebind_operator_constraints_correction = matches!(
        (record.phase, request.card, &request.operation),
        (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip,
            SemanticOperation::CorrectOperatorConstraintsBeforeBind { .. }
        )
    );
    if prebind_contract_repair {
        verify_prebind_contract_repair_inputs(store, &record, &cards)?;
    } else {
        verify_cards(store, &record, &cards)?;
    }
    if prebind_operator_constraints_correction {
        validate_prebind_operator_constraints_correction(&record, &cards, &request)?;
    }
    if matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) && matches!(
        request.operation,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented
        }
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "reviewed work must use typed csdlc-review recover",
        ));
    }
    let identity_update = matches!(
        request.operation,
        SemanticOperation::UpdateIdentityVersion { .. }
    );
    if identity_update {
        if !matches!(
            record.phase,
            LifecyclePhase::Initialized
                | LifecyclePhase::Ready
                | LifecyclePhase::Bound
                | LifecyclePhase::Implemented
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "identity version repair requires an active pre-review issue",
            ));
        }
    } else {
        authorize_card_operation(record.phase, request.card, &request.operation)?;
    }
    if prebind_contract_repair {
        validate_prebind_contract_repair(&cards, &request)?;
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectReviewPromptsAfterRecovery { .. }
    ) {
        let recovered = record.transitions.last().is_some_and(|transition| {
            transition.to == LifecyclePhase::Implemented
                && matches!(
                    transition.from,
                    LifecyclePhase::Reviewed
                        | LifecyclePhase::Published
                        | LifecyclePhase::MergeReady
                )
        });
        if !recovered
            || record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery review prompt correction requires cleared review and publication truth",
            ));
        }
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectDeclaredScopeBeforePublication { .. }
    ) {
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "implemented SIP scope correction requires actor and reason",
            ));
        }
        if record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "implemented SIP scope correction requires cleared review and publication truth",
            ));
        }
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectStpDeliverablesAfterRecovery { .. }
    ) {
        let latest_review_operation = record.audit.iter().rev().find(|event| {
            matches!(
                event.operation.as_str(),
                "assign_review" | "record_review" | "recover_review"
            )
        });
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "post-recovery STP deliverable correction requires actor and reason",
            ));
        }
        if latest_review_operation.is_none_or(|event| event.operation != "recover_review")
            || record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
            || record.terminal.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery STP deliverable correction requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
            ));
        }
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectPlanSummaryAfterRecovery { .. }
    ) {
        let latest_review_operation = record.audit.iter().rev().find(|event| {
            matches!(
                event.operation.as_str(),
                "assign_review" | "record_review" | "recover_review"
            )
        });
        let latest_transition = record.transitions.last();
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "post-recovery SPP summary correction requires actor and reason",
            ));
        }
        let current_recovery = latest_review_operation.is_some_and(|event| {
            event.operation == "recover_review"
                && event.generation == record.generation
                && latest_transition.is_some_and(|transition| {
                    transition.to == LifecyclePhase::Implemented
                        && matches!(
                            transition.from,
                            LifecyclePhase::Reviewed
                                | LifecyclePhase::Published
                                | LifecyclePhase::MergeReady
                        )
                        && transition.actor == event.actor
                        && transition.reason == event.reason
                })
        });
        if !current_recovery
            || record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
            || record.terminal.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery SPP summary correction requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
            ));
        }
    }
    let replan_before = match &request.operation {
        SemanticOperation::Replan { field, .. } => Some(current_text_value(
            cards
                .get(&request.card)
                .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?,
            *field,
        )?),
        _ => None,
    };
    let declared_scope_before = if matches!(
        request.operation,
        SemanticOperation::CorrectDeclaredScopeBeforePublication { .. }
    ) {
        match &cards[&CardKind::Sip].content {
            CardContent::Sip(value) => Some(value.declared_scope.clone()),
            _ => unreachable!("SIP"),
        }
    } else {
        None
    };
    let stp_deliverables_before = if matches!(
        request.operation,
        SemanticOperation::CorrectStpDeliverablesAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Stp].content {
            CardContent::Stp(value) => Some(value.deliverables.clone()),
            _ => unreachable!("STP"),
        }
    } else {
        None
    };
    let plan_summary_before = if matches!(
        request.operation,
        SemanticOperation::CorrectPlanSummaryAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Spp].content {
            CardContent::Spp(value) => Some(value.summary.clone()),
            _ => unreachable!("SPP"),
        }
    } else {
        None
    };
    let operator_constraints_before = if matches!(
        request.operation,
        SemanticOperation::CorrectOperatorConstraintsBeforeBind { .. }
    ) {
        match &cards[&CardKind::Sip].content {
            CardContent::Sip(value) => Some(value.operator_constraints.clone()),
            _ => unreachable!("SIP"),
        }
    } else {
        None
    };
    let binding_refresh = if prebind_contract_repair {
        Some(refresh_prebind_design_bindings(store, &record, &mut cards)?)
    } else {
        None
    };
    let audit_operation = match (&request.operation, replan_before) {
        (SemanticOperation::Replan { field, value }, Some(previous)) => serde_json::json!({
            "operation": "replan",
            "field": field.as_ref(),
            "previous_value": previous,
            "new_value": value,
        })
        .to_string(),
        (SemanticOperation::CorrectDeclaredScopeBeforePublication { values }, _) => {
            serde_json::json!({
                "operation": "correct_declared_scope_before_publication",
                "previous_values": declared_scope_before.expect("scope correction snapshot"),
                "new_values": values,
            })
            .to_string()
        }
        (SemanticOperation::CorrectStpDeliverablesAfterRecovery { values }, _) => {
            serde_json::json!({
                "operation": "correct_stp_deliverables_after_recovery",
                "previous_values": stp_deliverables_before
                    .expect("STP deliverable correction snapshot"),
                "new_values": values,
            })
            .to_string()
        }
        (SemanticOperation::CorrectPlanSummaryAfterRecovery { value }, _) => serde_json::json!({
            "operation": "correct_plan_summary_after_recovery",
            "previous_value": plan_summary_before.expect("SPP summary correction snapshot"),
            "new_value": value,
        })
        .to_string(),
        (SemanticOperation::CorrectOperatorConstraintsBeforeBind { values }, _) => {
            serde_json::json!({
                "operation": "correct_operator_constraints_before_bind",
                "previous_values": operator_constraints_before
                    .expect("SIP operator-constraint correction snapshot"),
                "new_values": values,
            })
            .to_string()
        }
        _ if binding_refresh.is_some() => {
            let refresh = binding_refresh.as_ref().expect("pre-bind refresh");
            serde_json::json!({
                "operation": request.operation,
                "design_binding_refresh": {
                    "design_ref": record.design_path,
                    "old_design_digest": refresh.old_design_digest,
                    "new_design_digest": refresh.new_design_digest,
                    "diagram_ref": record.diagram_path,
                    "old_diagram_digest": refresh.old_diagram_digest,
                    "new_diagram_digest": refresh.new_diagram_digest,
                }
            })
            .to_string()
        }
        _ => serde_json::to_string(&request.operation)?,
    };
    if identity_update {
        for values in cards.values_mut() {
            apply(values, &request.operation)?;
        }
    } else if let SemanticOperation::ReplaceAcceptancePlan {
        acceptance_criteria,
        steps,
        validation_lanes,
    } = &request.operation
    {
        crate::cards::replace_acceptance_plan(
            &mut cards,
            acceptance_criteria,
            steps,
            validation_lanes,
        )?;
    } else {
        let values = cards
            .get_mut(&request.card)
            .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?;
        if let Some(next) = apply(values, &request.operation)? {
            validate_phase_guard(store, &record, &cards, next)?;
            record.advance(next, request.actor.clone(), request.reason.clone())?;
        }
    }
    if prebind_contract_repair || prebind_operator_constraints_correction {
        record.design_review = DesignReview::Pending;
    }
    let design_digest = authored_digest(store, &record.design_path)?;
    let diagram_digest = authored_digest(store, &record.diagram_path)?;
    validate_cross_card(
        &cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )?;
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: audit_operation,
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, request.fail_after_backup)?;
    Ok(record)
}

#[derive(Debug)]
struct DesignBindingRefresh {
    old_design_digest: String,
    new_design_digest: String,
    old_diagram_digest: String,
    new_diagram_digest: String,
}

fn is_prebind_contract_repair(record: &IssueRecord, request: &EditRequest) -> bool {
    matches!(
        record.phase,
        LifecyclePhase::Initialized | LifecyclePhase::Ready
    ) && matches!(
        (request.card, &request.operation),
        (
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. }
        ) | (CardKind::Spp, SemanticOperation::ReplacePlanSteps { .. })
    )
}

fn validate_prebind_contract_repair(
    cards: &BTreeMap<CardKind, CardValues>,
    request: &EditRequest,
) -> Result<()> {
    let current_count = match &cards[&CardKind::Stp].content {
        CardContent::Stp(values) => values.acceptance_criteria.len(),
        _ => unreachable!("STP"),
    };
    match &request.operation {
        SemanticOperation::ReplaceAcceptanceCriteria { values } => {
            if values.len() != current_count
                || values
                    .iter()
                    .enumerate()
                    .any(|(index, value)| !value.starts_with(&format!("AC-{}:", index + 1)))
            {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "pre-bind acceptance repair must preserve the exact ordered AC-1 through AC-N denominator",
                ));
            }
        }
        SemanticOperation::ReplacePlanSteps { steps } => {
            let expected: std::collections::BTreeSet<_> =
                (1..=current_count).map(|n| format!("AC-{n}")).collect();
            let mapped_acceptance: Vec<_> = steps
                .iter()
                .flat_map(|step| step.acceptance_ids.iter().cloned())
                .collect();
            let actual: std::collections::BTreeSet<_> = mapped_acceptance.iter().cloned().collect();
            if actual != expected
                || mapped_acceptance.len() != expected.len()
                || steps.iter().any(|step| step.status != StepStatus::Pending)
            {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "pre-bind plan repair must remain pending and cover exactly the STP denominator",
                ));
            }
        }
        _ => unreachable!("pre-bind contract repair operation"),
    }
    Ok(())
}

fn verify_prebind_contract_repair_inputs(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    if record.branch.is_some()
        || record.worktree.is_some()
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.migration.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "pre-bind contract repair requires unbound topology and no later lifecycle evidence",
        ));
    }
    verify_card_projections(store, record, cards)?;
    let mut expected_audit = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut expected_audit, event)?;
        expected_audit.push(b'\n');
    }
    if fs::read(store.issue_dir(record.issue).join("audit.jsonl"))? != expected_audit {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "audit projection drift",
        ));
    }
    let (spp, vpp) = match (
        &cards[&CardKind::Spp].content,
        &cards[&CardKind::Vpp].content,
    ) {
        (CardContent::Spp(spp), CardContent::Vpp(vpp)) => (spp, vpp),
        _ => unreachable!("design-bearing cards"),
    };
    if spp.design_ref != record.design_path
        || vpp.design_ref != record.design_path
        || spp.diagram_ref != record.diagram_path
        || vpp.diagram_ref != record.diagram_path
        || spp.design_digest != vpp.design_digest
        || spp.diagram_digest != vpp.diagram_digest
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "pre-bind repair design/diagram references disagree with issue authority",
        ));
    }
    Ok(())
}

fn validate_prebind_operator_constraints_correction(
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    request: &EditRequest,
) -> Result<()> {
    if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "pre-bind SIP operator-constraint correction requires actor and reason",
        ));
    }
    if record.branch.is_some()
        || record.worktree.is_some()
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.migration.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "pre-bind SIP operator-constraint correction requires unmigrated, unbound topology and no later lifecycle evidence",
        ));
    }
    let sor = match &cards[&CardKind::Sor].content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR"),
    };
    if !sor.actual_changes.is_empty()
        || !sor.artifacts.is_empty()
        || !sor.actual_validation.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "pre-bind SIP operator-constraint correction requires absent execution and validation truth",
        ));
    }
    Ok(())
}

fn refresh_prebind_design_bindings(
    store: &Store,
    record: &IssueRecord,
    cards: &mut BTreeMap<CardKind, CardValues>,
) -> Result<DesignBindingRefresh> {
    let new_design_digest = authored_digest(store, &record.design_path)?;
    let new_diagram_digest = authored_digest(store, &record.diagram_path)?;
    let (old_design_digest, old_diagram_digest) = match &cards[&CardKind::Spp].content {
        CardContent::Spp(values) => (values.design_digest.clone(), values.diagram_digest.clone()),
        _ => unreachable!("SPP"),
    };
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("design-bearing card").content {
            CardContent::Spp(values) => {
                values.design_digest = new_design_digest.clone();
                values.diagram_digest = new_diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_digest = new_design_digest.clone();
                values.diagram_digest = new_diagram_digest.clone();
            }
            _ => unreachable!("design-bearing card"),
        }
    }
    Ok(DesignBindingRefresh {
        old_design_digest,
        new_design_digest,
        old_diagram_digest,
        new_diagram_digest,
    })
}

fn authored_digest(store: &Store, relative: &str) -> Result<String> {
    authored_digest_with_hook(store, relative, |_| {})
}

fn authored_digest_with_hook(
    store: &Store,
    relative: &str,
    hook: impl FnMut(AuthoredReadStage),
) -> Result<String> {
    let bytes = read_regular_authored_artifact_with_hook(store.root(), Path::new(relative), hook)?
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("authored design artifact is absent: {relative}"),
            )
        })?;
    Ok(digest(&bytes))
}

fn approval_authored_digests_with_hook(
    store: &Store,
    design_path: &str,
    diagram_path: &str,
    mut hook: impl FnMut(AuthoredReadStage),
) -> Result<(String, String)> {
    let design_digest = authored_digest_with_hook(store, design_path, &mut hook)?;
    let diagram_digest = authored_digest_with_hook(store, diagram_path, &mut hook)?;
    Ok((design_digest, diagram_digest))
}

fn current_text_value(values: &CardValues, field: crate::cards::TextField) -> Result<String> {
    match (&values.content, field) {
        (CardContent::Sip(value), crate::cards::TextField::Goal) => Ok(value.goal.clone()),
        (CardContent::Sip(value), crate::cards::TextField::RequiredOutcome) => {
            Ok(value.required_outcome.clone())
        }
        (CardContent::Stp(value), crate::cards::TextField::TaskBoundary) => {
            Ok(value.task_boundary.clone())
        }
        (CardContent::Spp(value), crate::cards::TextField::PlanSummary) => {
            Ok(value.summary.clone())
        }
        (CardContent::Srp(value), crate::cards::TextField::ReviewScope) => {
            Ok(value.review_scope.clone())
        }
        _ => Err(V2Error::new(
            ErrorCode::FieldOwnership,
            "replan field is not owned by the selected planning card",
        )),
    }
}

pub(crate) fn verify_cards(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_card_projections(store, record, cards)?;
    let mut expected_audit = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut expected_audit, event)?;
        expected_audit.push(b'\n');
    }
    if fs::read(store.issue_dir(record.issue).join("audit.jsonl"))? != expected_audit {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "audit projection drift",
        ));
    }
    let design_digest = authored_digest(store, &record.design_path)?;
    let diagram_digest = authored_digest(store, &record.diagram_path)?;
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )?;
    Ok(())
}

pub(crate) fn verify_pre_topology_cards(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_card_projections_with_options(store, record, cards, true)?;
    let mut expected_audit = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut expected_audit, event)?;
        expected_audit.push(b'\n');
    }
    if fs::read(store.issue_dir(record.issue).join("audit.jsonl"))? != expected_audit {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "audit projection drift",
        ));
    }
    let design_digest = authored_digest(store, &record.design_path)?;
    let diagram_digest = authored_digest(store, &record.diagram_path)?;
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )
}

fn verify_card_projections(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_card_projections_with_options(store, record, cards, false)
}

fn verify_card_projections_with_options(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    allow_pre_topology_bound: bool,
) -> Result<()> {
    verify_authority_card_inputs(store, record, cards, allow_pre_topology_bound)?;
    for (kind, values) in cards {
        let rendered = render(values)?;
        let projection = record.cards.get(kind).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("missing {kind} projection"),
            )
        })?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} digest drift"),
            ));
        }
    }
    Ok(())
}

fn verify_authority_card_inputs(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    allow_pre_topology_bound: bool,
) -> Result<()> {
    verify_record_with_options(record, allow_pre_topology_bound)?;
    for (kind, values) in cards {
        if values.kind() != *kind
            || values.identity.issue != record.issue
            || values.identity.repository != record.repository
            || values.identity.generation != record.generation
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} identity/generation mismatch"),
            ));
        }
        let rendered = render(values)?;
        let tracked = fs::read(
            store
                .issue_dir(record.issue)
                .join("cards")
                .join(format!("{kind}.md")),
        )?;
        if digest(&tracked) != rendered.rendered_digest {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                format!("{kind} rendered Markdown drift"),
            ));
        }
    }
    Ok(())
}

pub(crate) fn verify_record(record: &IssueRecord) -> Result<()> {
    verify_record_with_options(record, false)
}

fn verify_record_with_options(record: &IssueRecord, allow_pre_topology_bound: bool) -> Result<()> {
    if record.schema != "csdlc.issue.index.v1"
        || record.issue == 0
        || record.repository.is_empty()
        || record.initialization_digest.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "invalid index identity/schema",
        ));
    }
    if record.digest != record_digest(record)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "index digest mismatch",
        ));
    }
    if record.phase == LifecyclePhase::ClosedOut && record.terminal.is_none() {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "closed-out record must have terminal evidence",
        ));
    }
    let pre_topology_bound = allow_pre_topology_bound
        && record.phase == LifecyclePhase::Bound
        && record.branch.is_none()
        && record.worktree.is_none();
    if record.branch.is_some() != record.worktree.is_some()
        || (!pre_topology_bound
            && record.phase == LifecyclePhase::Bound
            && (record.branch.as_deref().is_none_or(str::is_empty)
                || record.worktree.as_deref().is_none_or(str::is_empty)))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "issue branch/worktree topology is incomplete",
        ));
    }
    if let DesignReview::Approved { reviewer, revision } = &record.design_review {
        if reviewer.trim().is_empty() || revision.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "design review evidence is empty",
            ));
        }
    }
    if record.audit.is_empty()
        || record.audit.iter().enumerate().any(|(index, event)| {
            event.sequence != index as u64 + 1
                || event.generation > record.generation
                || event.actor.is_empty()
                || event.reason.is_empty()
        })
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "audit sequence invariant failed",
        ));
    }
    let mut phase = LifecyclePhase::Initialized;
    let recordless_recovery = record.generation == 1
        && record.audit.len() == 1
        && record.transitions.len() == 1
        && record.audit[0].operation == "recover_recordless_terminal"
        && record.audit[0].generation == 1
        && record.audit[0].actor == record.transitions[0].actor
        && record.audit[0].reason == record.transitions[0].reason;
    for (index, event) in record.transitions.iter().enumerate() {
        let direct_recordless_closeout = recordless_recovery
            && record.transitions.len() == 1
            && event.from == LifecyclePhase::Initialized
            && event.to == LifecyclePhase::ClosedOut;
        let legacy_terminal_transition = matches!(
            (event.from, event.to),
            (LifecyclePhase::Published, LifecyclePhase::MergeReady)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Published)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Implemented)
                | (LifecyclePhase::MergeReady, LifecyclePhase::Merged)
                | (LifecyclePhase::Merged, LifecyclePhase::ClosedOut)
                | (LifecyclePhase::Reviewed, LifecyclePhase::ClosedOut)
        );
        let topology_migration_transition = event.actor == "csdlc-topology-migrate"
            && event.reason == "migrate pre-topology bound record"
            && matches!(
                (event.from, event.to),
                (LifecyclePhase::Bound, LifecyclePhase::Initialized)
                    | (LifecyclePhase::Bound, LifecyclePhase::ClosedOut)
            );
        if event.sequence != index as u64 + 1
            || event.from != phase
            || (!event.from.allows(event.to)
                && !direct_recordless_closeout
                && !legacy_terminal_transition
                && !topology_migration_transition)
            || event.actor.is_empty()
            || event.reason.is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "transition log invariant failed",
            ));
        }
        phase = event.to;
    }
    if phase != record.phase {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "phase does not match transition log",
        ));
    }
    Ok(())
}

fn validate_phase_guard(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    next: LifecyclePhase,
) -> Result<()> {
    if next == LifecyclePhase::Ready {
        verify_cards(store, record, cards)?;
        if !matches!(record.design_review, DesignReview::Approved { .. })
            || !matches!(
                cards[&CardKind::Sip].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Stp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Spp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
            || !matches!(
                cards[&CardKind::Vpp].status,
                crate::cards::CardStatus::Ready | crate::cards::CardStatus::Approved
            )
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "ready phase guard failed",
            ));
        }
    }
    if next == LifecyclePhase::Implemented {
        if let CardContent::Sor(sor) = &cards[&CardKind::Sor].content {
            if sor.actual_changes.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "implementation evidence missing",
                ));
            }
        }
    }
    if next == LifecyclePhase::Reviewed {
        if let CardContent::Srp(srp) = &cards[&CardKind::Srp].content {
            if srp
                .review_revision
                .as_deref()
                .unwrap_or_default()
                .is_empty()
                || srp.reviewer.as_deref().unwrap_or_default().is_empty()
                || srp.review_result != crate::cards::ReviewResult::Pass
                || srp.findings.iter().any(|finding| {
                    finding.actionable
                        && finding.disposition == crate::cards::FindingDisposition::Open
                })
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "review evidence is incomplete",
                ));
            }
        }
    }
    let srp = match &cards[&CardKind::Srp].content {
        CardContent::Srp(values) => values,
        _ => unreachable!("SRP"),
    };
    let sor = match &cards[&CardKind::Sor].content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR"),
    };
    let review_current = srp.review_result == crate::cards::ReviewResult::Pass
        && srp.review_revision.as_deref().unwrap_or_default() != ""
        && srp.reviewer.as_deref().unwrap_or_default() != ""
        && !srp.findings.iter().any(|finding| {
            finding.actionable && finding.disposition == crate::cards::FindingDisposition::Open
        });
    if next == LifecyclePhase::Published
        && (!review_current
            || record.review.as_ref().is_none_or(|review| {
                crate::git::substantive_revision(store.root(), &review.scope).map_or(
                    true,
                    |current| {
                        !evaluate_publication_review_in_repo(
                            store.root(),
                            record.review.as_ref(),
                            &current,
                        )
                        .ready
                    },
                )
            })
            || !matches!(
                sor.publication_state,
                crate::cards::PublicationState::Draft | crate::cards::PublicationState::Ready
            ))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "publication observation or current review evidence is missing",
        ));
    }
    Ok(())
}

fn authorize_card_operation(
    phase: LifecyclePhase,
    card: CardKind,
    operation: &SemanticOperation,
) -> Result<()> {
    if matches!(operation, SemanticOperation::AdvancePhase { .. }) {
        return Ok(());
    }
    let allowed = matches!(
        (phase, card, operation),
        (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. },
        ) | (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Spp,
            SemanticOperation::ReplacePlanSteps { .. },
        ) | (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Vpp,
            SemanticOperation::SetField { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Spp,
            SemanticOperation::ReplacePlanSteps { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Spp,
            SemanticOperation::ReplaceAcceptancePlan { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sip | CardKind::Stp | CardKind::Spp,
            SemanticOperation::Replan { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Srp,
            SemanticOperation::Replan {
                field: crate::cards::TextField::ReviewScope,
                ..
            },
        ) | (
            LifecyclePhase::Bound | LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::UpdatePlanStep { .. }
                | SemanticOperation::RecordAdvisoryEstimate { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::AffectedAreas,
                ..
            } | SemanticOperation::ReplacePlanSteps { .. }
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::Invariants
                        | crate::cards::PlanningCollectionField::StopConditions,
                    ..
                },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints { .. }
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::AuthorityBoundary,
                    ..
                }
                | SemanticOperation::CorrectDeclaredScopeBeforePublication { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. }
                | SemanticOperation::CorrectStpDeliverablesAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::CorrectPlanSummaryAfterRecovery { .. },
        ) | (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip,
            SemanticOperation::CorrectOperatorConstraintsBeforeBind { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::ReviewPrompts,
                ..
            },
        ) | (
            LifecyclePhase::Initialized
                | LifecyclePhase::Ready
                | LifecyclePhase::Bound
                | LifecyclePhase::Implemented,
            CardKind::Vpp,
            SemanticOperation::ReplaceValidationLanes { .. },
        ) | (
            LifecyclePhase::Bound,
            CardKind::Sor,
            SemanticOperation::RecordExecution { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::CorrectReviewPromptsAfterRecovery { .. }
                | SemanticOperation::RecordReview { .. }
                | SemanticOperation::RecordFinding { .. }
                | SemanticOperation::DisposeFinding { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::RecordExecution { .. }
                | SemanticOperation::ReplaceExecution { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::Reviewed | LifecyclePhase::Published,
            CardKind::Srp,
            SemanticOperation::RecordFinding { .. }
                | SemanticOperation::DisposeFinding { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::Reviewed | LifecyclePhase::Published,
            CardKind::Sor,
            SemanticOperation::RecordPublication { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        )
    );
    if allowed {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!("{card} mutation is not allowed during {phase}"),
        ))
    }
}

fn hydrate_projections(
    record: &mut IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    record.cards.clear();
    for (kind, values) in cards {
        let rendered = render(values)?;
        record.cards.insert(
            *kind,
            CardProjection {
                values_digest: rendered.values_digest,
                rendered_digest: rendered.rendered_digest,
                ast_digest: rendered.ast_digest,
            },
        );
    }
    Ok(())
}

fn validate_updated_cards(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let design_digest = authored_digest(store, &record.design_path)?;
    let diagram_digest = authored_digest(store, &record.diagram_path)?;
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )
}

pub(crate) fn record_digest(record: &IssueRecord) -> Result<String> {
    let mut value = record.clone();
    value.digest.clear();
    Ok(digest(&serde_json::to_vec(&value)?))
}

fn terminal_receipt_digest(receipt: &TerminalReceipt) -> Result<String> {
    let mut value = receipt.clone();
    value.digest.clear();
    Ok(digest(&serde_json::to_vec(&value)?))
}

fn validate_terminal_receipt(receipt: &TerminalReceipt) -> Result<()> {
    if receipt.schema != "csdlc.terminal_receipt.v1"
        || receipt.issue == 0
        || receipt.issue != receipt.record.issue
        || receipt.repository != receipt.record.repository
        || receipt.initialization_digest != receipt.record.initialization_digest
        || receipt.receipt_ref != format!("csdlc-v2/closeout/{}.json", receipt.issue)
        || receipt.record.phase != LifecyclePhase::ClosedOut
        || receipt.record.terminal.is_none()
        || receipt.cards.len() != 6
        || receipt.authored_artifacts.len() != 2
        || receipt.digest != terminal_receipt_digest(receipt)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt identity, phase, or digest is invalid",
        ));
    }
    if !crate::pvf::clean_relative(Path::new(&receipt.record.design_path))
        || !crate::pvf::clean_relative(Path::new(&receipt.record.diagram_path))
        || receipt
            .authored_artifacts
            .keys()
            .any(|path| !crate::pvf::clean_relative(Path::new(path)))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt authored paths must be clean repository-relative paths",
        ));
    }
    verify_record(&receipt.record)?;
    let design = receipt
        .authored_artifacts
        .get(&receipt.record.design_path)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "receipt design missing"))?;
    let diagram = receipt
        .authored_artifacts
        .get(&receipt.record.diagram_path)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "receipt diagram missing"))?;
    for (kind, values) in &receipt.cards {
        if values.kind() != *kind
            || values.identity.issue != receipt.issue
            || values.identity.repository != receipt.repository
            || values.identity.generation != receipt.record.generation
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt card identity is invalid",
            ));
        }
        let rendered = render(values)?;
        let projection = receipt.record.cards.get(kind).ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt projection missing",
            )
        })?;
        if projection.values_digest != rendered.values_digest
            || projection.rendered_digest != rendered.rendered_digest
            || projection.ast_digest != rendered.ast_digest
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal receipt card digest drift",
            ));
        }
    }
    let terminal_disposition = receipt
        .record
        .terminal
        .as_ref()
        .expect("terminal presence checked")
        .disposition;
    if !terminal_cards_match_disposition(&receipt.cards, terminal_disposition) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt cards do not match the terminal disposition",
        ));
    }
    validate_cross_card(
        &receipt.cards,
        &receipt.record.design_path,
        &digest(design.as_bytes()),
        &receipt.record.diagram_path,
        &digest(diagram.as_bytes()),
    )?;
    if !matches!(
        &receipt.record.design_review,
        DesignReview::Approved { revision, .. } if revision == &digest(design.as_bytes())
    ) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal receipt design review is stale",
        ));
    }
    Ok(())
}

fn push_legacy_terminal_transition(
    record: &mut IssueRecord,
    next: LifecyclePhase,
    actor: &str,
    reason: &str,
) {
    let from = record.phase;
    record.phase = next;
    record.transitions.push(TransitionEvent {
        sequence: record.transitions.len() as u64 + 1,
        from,
        to: next,
        actor: actor.into(),
        reason: reason.into(),
    });
}

fn terminal_disposition_from_finish(
    disposition: crate::finish::FinishDisposition,
) -> crate::readiness::TerminalDisposition {
    match disposition {
        crate::finish::FinishDisposition::Merged => crate::readiness::TerminalDisposition::Merged,
        crate::finish::FinishDisposition::ClosedUnmerged => {
            crate::readiness::TerminalDisposition::ClosedUnmerged
        }
        crate::finish::FinishDisposition::ClosedNoPr => {
            crate::readiness::TerminalDisposition::ClosedNoPr
        }
    }
}

fn terminal_cards_match_disposition(
    cards: &BTreeMap<CardKind, CardValues>,
    disposition: crate::readiness::TerminalDisposition,
) -> bool {
    let spp_complete = cards
        .get(&CardKind::Spp)
        .is_some_and(|card| match &card.content {
            CardContent::Spp(values) => values
                .steps
                .iter()
                .all(|step| step.status == StepStatus::Completed),
            _ => false,
        });
    let sor_complete = cards.get(&CardKind::Sor).is_some_and(|card| {
        if card.status != CardStatus::Complete {
            return false;
        }
        match (&card.content, disposition) {
            (CardContent::Sor(values), crate::readiness::TerminalDisposition::Merged) => {
                values.integration_state == crate::cards::IntegrationState::Merged
                    && values.publication_state == crate::cards::PublicationState::Closed
                    && values.merge_state == crate::cards::MergeState::Merged
                    && values.closeout_state == crate::cards::CloseoutState::Complete
            }
            (
                CardContent::Sor(values),
                crate::readiness::TerminalDisposition::ClosedUnmerged
                | crate::readiness::TerminalDisposition::ClosedNoPr,
            ) => {
                values.integration_state == crate::cards::IntegrationState::ClosedNoPr
                    && values.publication_state == crate::cards::PublicationState::Closed
                    && values.merge_state == crate::cards::MergeState::ClosedUnmerged
                    && values.closeout_state == crate::cards::CloseoutState::Complete
            }
            _ => false,
        }
    });
    spp_complete && sor_complete
}

fn terminal_matches_derived(
    record: &IssueRecord,
    envelope: &crate::finish::DerivedTerminalEnvelope,
) -> bool {
    if record.phase != LifecyclePhase::ClosedOut
        || envelope.issue != record.issue
        || envelope.repository != record.repository
        || envelope.initialization_digest != record.initialization_digest
    {
        return false;
    }
    let Some(terminal) = record.terminal.as_ref() else {
        return false;
    };
    let disposition = match envelope.disposition {
        crate::finish::FinishDisposition::Merged => crate::readiness::TerminalDisposition::Merged,
        crate::finish::FinishDisposition::ClosedUnmerged => {
            crate::readiness::TerminalDisposition::ClosedUnmerged
        }
        crate::finish::FinishDisposition::ClosedNoPr => {
            crate::readiness::TerminalDisposition::ClosedNoPr
        }
    };
    terminal.disposition == disposition
        && terminal.pull_request == envelope.pull_request
        && terminal.observed_sha == envelope.head_sha
}

fn verify_canonical_projection_bytes(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let issue_dir = PathBuf::from(".csdlc/issues").join(record.issue.to_string());
    let mut index = serde_json::to_vec_pretty(record)?;
    index.push(b'\n');
    if read_regular_projection(&store.root, &issue_dir.join("index.json"))? != index {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "authority target index projection is not canonical",
        ));
    }
    let mut audit = Vec::new();
    for event in &record.audit {
        serde_json::to_writer(&mut audit, event)?;
        audit.push(b'\n');
    }
    if read_regular_projection(&store.root, &issue_dir.join("audit.jsonl"))? != audit {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "authority target audit projection is not canonical",
        ));
    }
    for (kind, values) in cards {
        let mut encoded = serde_json::to_vec_pretty(values)?;
        encoded.push(b'\n');
        let rendered = render(values)?;
        if read_regular_projection(
            &store.root,
            &issue_dir.join(format!("cards/{kind}.values.json")),
        )? != encoded
            || read_regular_projection(&store.root, &issue_dir.join(format!("cards/{kind}.md")))?
                != rendered.markdown.as_bytes()
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "authority target card projection is not canonical",
            ));
        }
    }
    Ok(())
}

fn read_regular_projection(root: &Path, relative: &Path) -> Result<Vec<u8>> {
    let path = root.join(relative);
    let metadata = canonical_path_metadata_beneath(root, relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("authority projection is absent: {}", path.display()),
        )
    })?;
    if !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "authority projection is not a regular file: {}",
                path.display()
            ),
        ));
    }
    Ok(fs::read(path)?)
}

pub(crate) fn read_regular_authored_artifact(
    root: &Path,
    relative: &Path,
) -> Result<Option<Vec<u8>>> {
    read_regular_authored_artifact_with_hook(root, relative, |_| {})
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthoredReadStage {
    AfterInitialOpen,
    BetweenReads,
    BeforeFinalOpen,
}

fn read_regular_authored_artifact_with_hook(
    root: &Path,
    relative: &Path,
    hook: impl FnMut(AuthoredReadStage),
) -> Result<Option<Vec<u8>>> {
    validate_authored_relative_path(relative)?;
    read_regular_authored_artifact_platform_with_hook(root, relative, hook)
}

fn validate_authored_relative_path(relative: &Path) -> Result<()> {
    let value = relative.to_str().ok_or_else(|| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            "authored artifact path must be UTF-8",
        )
    })?;
    let segments: Vec<_> = value.split('/').collect();
    let bytes = value.as_bytes();
    let has_ascii_drive_prefix =
        bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':';
    if value.is_empty()
        || value.contains('\\')
        || has_ascii_drive_prefix
        || !crate::pvf::clean_relative(relative)
        || segments.iter().any(|segment| segment.is_empty())
        || segments
            .iter()
            .any(|segment| *segment == "." || *segment == "..")
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "authored artifact path must be nonempty, clean, canonical, and repository-relative",
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn read_regular_authored_artifact_platform_with_hook(
    root: &Path,
    relative: &Path,
    mut hook: impl FnMut(AuthoredReadStage),
) -> Result<Option<Vec<u8>>> {
    use std::os::unix::fs::MetadataExt;
    let root_path_metadata = fs::symlink_metadata(root)?;
    if root_path_metadata.file_type().is_symlink() || !root_path_metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "authored artifact root is not a regular directory: {}",
                root.display()
            ),
        ));
    }
    let root_handle = File::open(root)?;
    let root_handle_metadata = root_handle.metadata()?;
    if !root_handle_metadata.is_dir()
        || !same_file_identity(&root_path_metadata, &root_handle_metadata)
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "authored artifact root changed identity while opening",
        ));
    }
    let Some(mut opened) = open_relative_no_follow(&root_handle, relative)? else {
        return Ok(None);
    };
    hook(AuthoredReadStage::AfterInitialOpen);
    let before = opened.metadata()?;
    if !before.is_file() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact target is not a regular file",
        ));
    }
    let first = read_exact_current_file(&mut opened, before.len())?;
    hook(AuthoredReadStage::BetweenReads);
    let middle = opened.metadata()?;
    opened.seek(SeekFrom::Start(0))?;
    let second = read_exact_current_file(&mut opened, middle.len())?;
    let after = opened.metadata()?;
    if first != second
        || !same_file_identity(&before, &middle)
        || !same_file_identity(&middle, &after)
        || before.len() != middle.len()
        || middle.len() != after.len()
        || before.mtime() != middle.mtime()
        || before.mtime_nsec() != middle.mtime_nsec()
        || middle.mtime() != after.mtime()
        || middle.mtime_nsec() != after.mtime_nsec()
        || before.ctime() != middle.ctime()
        || before.ctime_nsec() != middle.ctime_nsec()
        || middle.ctime() != after.ctime()
        || middle.ctime_nsec() != after.ctime_nsec()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact changed while reading",
        ));
    }
    hook(AuthoredReadStage::BeforeFinalOpen);
    let final_file = open_relative_no_follow(&root_handle, relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact disappeared before final verification",
        )
    })?;
    let final_metadata = final_file.metadata()?;
    if !final_metadata.is_file()
        || !same_file_identity(&after, &final_metadata)
        || after.len() != final_metadata.len()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact path changed identity before final verification",
        ));
    }
    let mut final_file = final_file;
    let final_bytes = read_exact_current_file(&mut final_file, final_metadata.len())?;
    let final_after = final_file.metadata()?;
    if final_bytes != first
        || !same_file_identity(&final_metadata, &final_after)
        || final_metadata.len() != final_after.len()
        || final_metadata.mtime() != final_after.mtime()
        || final_metadata.mtime_nsec() != final_after.mtime_nsec()
        || final_metadata.ctime() != final_after.ctime()
        || final_metadata.ctime_nsec() != final_after.ctime_nsec()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact changed during final verification",
        ));
    }
    Ok(Some(first))
}

#[cfg(unix)]
fn open_relative_no_follow(root: &File, relative: &Path) -> Result<Option<File>> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};

    let components: Vec<_> = relative.components().collect();
    let mut directories = Vec::with_capacity(components.len().saturating_sub(1));
    let mut directory_fd = root.as_raw_fd();
    for (index, component) in components.iter().enumerate() {
        let std::path::Component::Normal(name) = component else {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "authored artifact path contains a non-normal component",
            ));
        };
        let name = CString::new(name.as_bytes()).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "authored path contains a NUL byte",
            )
        })?;
        let last = index + 1 == components.len();
        let flags = libc::O_RDONLY
            | libc::O_CLOEXEC
            | libc::O_NOFOLLOW
            | if last { 0 } else { libc::O_DIRECTORY };
        // SAFETY: directory_fd is owned by root or a retained directory File;
        // name is NUL-terminated; a successful descriptor is immediately owned.
        let descriptor = unsafe { libc::openat(directory_fd, name.as_ptr(), flags) };
        if descriptor < 0 {
            let error = std::io::Error::last_os_error();
            return match error.raw_os_error() {
                Some(libc::ENOENT) => Ok(None),
                Some(code) if code == libc::ELOOP || code == libc::ENOTDIR => Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "authored artifact path contains a symlink or non-directory ancestor",
                )),
                _ => Err(error.into()),
            };
        }
        // SAFETY: descriptor is a new successful openat result owned here.
        let opened = unsafe { File::from_raw_fd(descriptor) };
        if last {
            return Ok(Some(opened));
        }
        if !opened.metadata()?.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "authored artifact ancestor is not a directory",
            ));
        }
        directories.push(opened);
        directory_fd = directories.last().expect("retained directory").as_raw_fd();
    }
    Ok(None)
}

#[cfg(windows)]
fn read_regular_authored_artifact_platform_with_hook(
    root: &Path,
    relative: &Path,
    mut hook: impl FnMut(AuthoredReadStage),
) -> Result<Option<Vec<u8>>> {
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    let path = root.join(relative);
    let Some(path_metadata) = canonical_path_metadata_beneath(root, relative)? else {
        return Ok(None);
    };
    if !path_metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact target is not a regular file",
        ));
    }
    let mut file = OpenOptions::new()
        .read(true)
        .share_mode(1) // FILE_SHARE_READ: deny concurrent write/delete/rename.
        .custom_flags(0x0020_0000) // FILE_FLAG_OPEN_REPARSE_POINT.
        .open(path)?;
    let before = file.metadata()?;
    if !before.is_file() || !same_file_identity(&path_metadata, &before) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact changed identity while opening",
        ));
    }
    hook(AuthoredReadStage::AfterInitialOpen);
    let first = read_exact_current_file(&mut file, before.len())?;
    hook(AuthoredReadStage::BetweenReads);
    file.seek(SeekFrom::Start(0))?;
    let second = read_exact_current_file(&mut file, before.len())?;
    let after = file.metadata()?;
    hook(AuthoredReadStage::BeforeFinalOpen);
    if first != second
        || !same_file_identity(&before, &after)
        || before.len() != after.len()
        || before.last_write_time() != after.last_write_time()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact changed while reading",
        ));
    }
    Ok(Some(first))
}

#[cfg(not(any(unix, windows)))]
fn read_regular_authored_artifact_platform_with_hook(
    _root: &Path,
    _relative: &Path,
    _hook: impl FnMut(AuthoredReadStage),
) -> Result<Option<Vec<u8>>> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "authored artifact reads require an anchored or mutation-denying platform primitive",
    ))
}

fn read_exact_current_file(file: &mut File, expected_len: u64) -> Result<Vec<u8>> {
    let expected_len = usize::try_from(expected_len).map_err(|_| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact is too large to read safely",
        )
    })?;
    let mut bytes = Vec::with_capacity(expected_len);
    file.read_to_end(&mut bytes)?;
    if bytes.len() != expected_len {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact size changed while reading",
        ));
    }
    Ok(bytes)
}

#[cfg(unix)]
fn same_file_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino()
}

#[cfg(windows)]
fn same_file_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    match (
        left.volume_serial_number(),
        left.file_index(),
        right.volume_serial_number(),
        right.file_index(),
    ) {
        (Some(left_volume), Some(left_index), Some(right_volume), Some(right_index)) => {
            left_volume == right_volume && left_index == right_index
        }
        _ => false,
    }
}

#[cfg(not(any(unix, windows)))]
fn same_file_identity(_left: &fs::Metadata, _right: &fs::Metadata) -> bool {
    // No stable file-identity primitive is available on this target. Fail
    // closed instead of accepting a pathname-only comparison.
    false
}

fn canonical_path_metadata_beneath(root: &Path, relative: &Path) -> Result<Option<fs::Metadata>> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "canonical path is not clean and root-relative: {}",
                relative.display()
            ),
        ));
    }
    let root_metadata = fs::symlink_metadata(root)?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "canonical root is not a regular directory: {}",
                root.display()
            ),
        ));
    }
    let mut current = root.to_path_buf();
    let components: Vec<_> = relative.components().collect();
    for (index, component) in components.iter().enumerate() {
        match component {
            std::path::Component::Normal(part) => current.push(part),
            _ => {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "authored artifact path contains a non-normal component",
                ));
            }
        }
        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        if metadata.file_type().is_symlink() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!("canonical path contains a symlink: {}", current.display()),
            ));
        }
        if index + 1 < components.len() && !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "canonical path ancestor is not a directory: {}",
                    current.display()
                ),
            ));
        }
        if index + 1 == components.len() {
            return Ok(Some(metadata));
        }
    }
    Ok(None)
}

pub(crate) fn require_canonical_parent_beneath(root: &Path, relative: &Path) -> Result<()> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "terminal write path is not clean and root-relative: {}",
                relative.display()
            ),
        ));
    }
    let parent = relative.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "terminal write path has no parent",
        )
    })?;
    if parent.as_os_str().is_empty() {
        let metadata = fs::symlink_metadata(root)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!("terminal write root is unsafe: {}", root.display()),
            ));
        }
        return Ok(());
    }
    if let Some(metadata) = canonical_path_metadata_beneath(root, parent)? {
        if !metadata.is_dir() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal write parent is not a directory: {}",
                    root.join(parent).display()
                ),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_regular_or_absent_beneath(root: &Path, relative: &Path) -> Result<()> {
    if let Some(metadata) = canonical_path_metadata_beneath(root, relative)? {
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal write target is not a regular file: {}",
                    root.join(relative).display()
                ),
            ));
        }
    }
    Ok(())
}

fn replace_regular_authored_artifact(
    root: &Path,
    relative: &Path,
    bytes: &[u8],
    temporary_extension: &str,
) -> Result<()> {
    require_canonical_parent_beneath(root, relative)?;
    let destination = root.join(relative);
    let parent = destination.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "terminal write target has no parent",
        )
    })?;
    fs::create_dir_all(parent)?;
    require_canonical_parent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, relative)?;

    let temporary_relative = relative.with_extension(temporary_extension);
    require_canonical_parent_beneath(root, &temporary_relative)?;
    if canonical_path_metadata_beneath(root, &temporary_relative)?.is_some() {
        require_regular_or_absent_beneath(root, &temporary_relative)?;
        fs::remove_file(root.join(&temporary_relative))?;
    }
    let temporary = root.join(&temporary_relative);
    let mut file = File::create(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;

    require_canonical_parent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, relative)?;
    require_regular_or_absent_beneath(root, &temporary_relative)?;
    fs::rename(&temporary, &destination)?;
    sync_dir(parent)?;
    Ok(())
}

fn write_complete(
    path: &Path,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    let cards_dir = path.join("cards");
    fs::create_dir_all(&cards_dir)?;
    write_json(&path.join("index.json"), record)?;
    let mut audit = File::create(path.join("audit.jsonl"))?;
    for event in &record.audit {
        serde_json::to_writer(&mut audit, event)?;
        audit.write_all(b"\n")?;
    }
    audit.sync_all()?;
    for (kind, values) in cards {
        let rendered = render(values)?;
        write_json(&cards_dir.join(format!("{kind}.values.json")), values)?;
        let mut file = File::create(cards_dir.join(format!("{kind}.md")))?;
        file.write_all(rendered.markdown.as_bytes())?;
        file.sync_all()?;
    }
    sync_dir(&cards_dir)?;
    sync_dir(path)?;
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}

fn enum_iterator() -> impl Iterator<Item = CardKind> {
    use strum::IntoEnumIterator;
    CardKind::iter()
}

#[cfg(test)]
mod edit_authorization_tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn authored_reader_rejects_non_utf8_path_before_open() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let temp = tempfile::tempdir().expect("tempdir");
        let path = PathBuf::from(OsString::from_vec(vec![
            b'd', b'e', 0xff, b's', b'i', b'g', b'n',
        ]));
        let error = read_regular_authored_artifact(temp.path(), &path)
            .expect_err("non-UTF authored path must fail closed");
        assert_eq!(error.code, ErrorCode::CorruptRecord);
        assert_eq!(error.message, "authored artifact path must be UTF-8");
    }

    #[cfg(windows)]
    #[test]
    fn authored_reader_rejects_non_utf8_path_before_open() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let temp = tempfile::tempdir().expect("tempdir");
        let path = PathBuf::from(OsString::from_wide(&[0x0064, 0xd800, 0x006e]));
        let error = read_regular_authored_artifact(temp.path(), &path)
            .expect_err("non-UTF authored path must fail closed");
        assert_eq!(error.code, ErrorCode::CorruptRecord);
        assert_eq!(error.message, "authored artifact path must be UTF-8");
    }

    #[test]
    fn authored_artifact_identity_distinguishes_equal_length_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let first = temp.path().join("first.md");
        let second = temp.path().join("second.md");
        fs::write(&first, b"same-length").expect("first artifact");
        fs::write(&second, b"other-byte!").expect("second artifact");
        let first_metadata = fs::metadata(first).expect("first metadata");
        let second_metadata = fs::metadata(second).expect("second metadata");
        assert_eq!(first_metadata.len(), second_metadata.len());
        assert!(same_file_identity(&first_metadata, &first_metadata));
        assert!(!same_file_identity(&first_metadata, &second_metadata));
    }

    #[test]
    fn approval_hashes_bind_exact_authored_bytes() {
        let temp = tempfile::tempdir().expect("tempdir");
        let design = b"# reviewed design\n";
        let diagram = b"flowchart LR\n  Review --> Approve\n";
        fs::write(temp.path().join("design.md"), design).expect("design");
        fs::write(temp.path().join("diagram.mmd"), diagram).expect("diagram");
        let store = Store::new(temp.path());
        let (design_digest, diagram_digest) =
            approval_authored_digests_with_hook(&store, "design.md", "diagram.mmd", |_| {})
                .expect("approval digests");
        assert_eq!(design_digest, digest(design));
        assert_eq!(diagram_digest, digest(diagram));
    }

    #[cfg(unix)]
    #[test]
    fn approval_hashing_rejects_symlinked_authored_path() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().expect("tempdir");
        fs::write(temp.path().join("real-design.md"), b"# design\n").expect("real design");
        fs::write(temp.path().join("diagram.mmd"), b"flowchart LR\n").expect("diagram");
        symlink("real-design.md", temp.path().join("design.md")).expect("design symlink");
        let store = Store::new(temp.path());
        let error = approval_authored_digests_with_hook(&store, "design.md", "diagram.mmd", |_| {})
            .expect_err("approval symlink must fail closed");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
    }

    #[cfg(unix)]
    #[test]
    fn approval_hashing_rejects_hardlink_replacement_during_read() {
        let temp = tempfile::tempdir().expect("tempdir");
        let design_path = temp.path().join("design.md");
        fs::write(&design_path, vec![b'o'; 32]).expect("design");
        fs::write(temp.path().join("malicious.md"), vec![b'x'; 32]).expect("malicious");
        fs::write(temp.path().join("diagram.mmd"), b"flowchart LR\n").expect("diagram");
        let store = Store::new(temp.path());
        let error =
            approval_authored_digests_with_hook(&store, "design.md", "diagram.mmd", |stage| {
                if stage == AuthoredReadStage::BeforeFinalOpen {
                    fs::remove_file(&design_path).expect("remove design name");
                    fs::hard_link(temp.path().join("malicious.md"), &design_path)
                        .expect("replace design with hardlink");
                }
            })
            .expect_err("approval replacement must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[cfg(unix)]
    #[test]
    fn anchored_authored_read_ignores_ancestor_swap_back_to_hardlink_tree() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("root");
        let authored = root.join("authored");
        fs::create_dir_all(&authored).expect("authored directory");
        let original = vec![b'o'; 32];
        let malicious = vec![b'x'; 32];
        fs::write(authored.join("design.md"), &original).expect("original artifact");
        fs::write(root.join("malicious.md"), &malicious).expect("malicious artifact");

        let bytes = read_regular_authored_artifact_with_hook(
            &root,
            Path::new("authored/design.md"),
            |stage| match stage {
                AuthoredReadStage::AfterInitialOpen => {
                    fs::rename(&authored, root.join("retained-authored"))
                        .expect("move opened ancestor");
                    fs::create_dir(&authored).expect("replacement ancestor");
                    fs::hard_link(root.join("malicious.md"), authored.join("design.md"))
                        .expect("replacement hardlink");
                }
                AuthoredReadStage::BeforeFinalOpen => {
                    fs::remove_file(authored.join("design.md")).expect("remove replacement file");
                    fs::remove_dir(&authored).expect("remove replacement ancestor");
                    fs::rename(root.join("retained-authored"), &authored)
                        .expect("restore original ancestor");
                }
                AuthoredReadStage::BetweenReads => {}
            },
        )
        .expect("anchored swap-back read")
        .expect("authored artifact");
        assert_eq!(bytes, original);
        assert_eq!(fs::read(authored.join("design.md")).unwrap(), original);
    }

    #[cfg(unix)]
    #[test]
    fn anchored_authored_read_rejects_retained_hardlink_replacement() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("root");
        fs::create_dir_all(root.join("authored")).expect("authored directory");
        fs::write(root.join("authored/design.md"), vec![b'o'; 32]).expect("original artifact");
        fs::write(root.join("malicious.md"), vec![b'x'; 32]).expect("malicious artifact");
        let error = read_regular_authored_artifact_with_hook(
            &root,
            Path::new("authored/design.md"),
            |stage| {
                if stage == AuthoredReadStage::BeforeFinalOpen {
                    fs::remove_file(root.join("authored/design.md")).expect("remove original name");
                    fs::hard_link(root.join("malicious.md"), root.join("authored/design.md"))
                        .expect("install hardlink replacement");
                }
            },
        )
        .expect_err("hardlink replacement must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[cfg(unix)]
    #[test]
    fn anchored_authored_read_rejects_same_length_in_place_mutation() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("root");
        fs::create_dir_all(root.join("authored")).expect("authored directory");
        let path = root.join("authored/design.md");
        fs::write(&path, vec![b'o'; 32]).expect("original artifact");
        let error = read_regular_authored_artifact_with_hook(
            &root,
            Path::new("authored/design.md"),
            |stage| {
                if stage == AuthoredReadStage::BetweenReads {
                    fs::write(&path, vec![b'x'; 32]).expect("same-length mutation");
                }
            },
        )
        .expect_err("same-length mutation must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    fn replacement_steps() -> Vec<crate::cards::PlanStep> {
        vec![crate::cards::PlanStep {
            id: "review-fix".into(),
            action: "correct bounded review finding".into(),
            acceptance_ids: vec!["AC-1".into()],
            status: crate::cards::StepStatus::Pending,
        }]
    }

    #[test]
    fn implemented_review_remediation_authorizes_only_bounded_operations() {
        for phase in [LifecyclePhase::Bound, LifecyclePhase::Implemented] {
            authorize_card_operation(
                phase,
                CardKind::Stp,
                &SemanticOperation::ReplaceAcceptanceCriteria {
                    values: vec!["AC-1: compatibility repair".into()],
                },
            )
            .expect("bound and implemented STP compatibility remains available");
            authorize_card_operation(
                phase,
                CardKind::Spp,
                &SemanticOperation::ReplacePlanSteps {
                    steps: replacement_steps(),
                },
            )
            .expect("bound and implemented SPP compatibility remains available");
        }

        for operation in [
            SemanticOperation::ReplacePlanSteps {
                steps: replacement_steps(),
            },
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::Invariants,
                values: vec!["invariant".into()],
            },
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::StopConditions,
                values: vec!["stop".into()],
            },
        ] {
            authorize_card_operation(LifecyclePhase::Implemented, CardKind::Spp, &operation)
                .expect("implemented bounded SPP remediation");
        }

        let error = authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Spp,
            &SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::Risks,
                values: vec!["risk".into()],
            },
        )
        .expect_err("unbounded SPP collection remains rejected");
        assert_eq!(error.code, ErrorCode::InvalidTransition);

        authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Sip,
            &SemanticOperation::CorrectDeclaredScopeBeforePublication {
                values: vec!["src/current.rs".into()],
            },
        )
        .expect("implemented SIP scope correction");
        for phase in [
            LifecyclePhase::Initialized,
            LifecyclePhase::Ready,
            LifecyclePhase::Bound,
            LifecyclePhase::Reviewed,
            LifecyclePhase::Published,
            LifecyclePhase::MergeReady,
            LifecyclePhase::Merged,
            LifecyclePhase::ClosedOut,
        ] {
            let error = authorize_card_operation(
                phase,
                CardKind::Sip,
                &SemanticOperation::CorrectDeclaredScopeBeforePublication {
                    values: vec!["src/late.rs".into()],
                },
            )
            .expect_err("scope correction is implemented-only");
            assert_eq!(error.code, ErrorCode::InvalidTransition);
        }
        let error = authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Stp,
            &SemanticOperation::CorrectDeclaredScopeBeforePublication {
                values: vec!["src/wrong-card.rs".into()],
            },
        )
        .expect_err("scope correction is SIP-only");
        assert_eq!(error.code, ErrorCode::InvalidTransition);

        authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Stp,
            &SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: vec!["src/current.rs".into()],
            },
        )
        .expect("implemented STP correction reaches recovery-sensitive guard");
        for phase in [
            LifecyclePhase::Initialized,
            LifecyclePhase::Ready,
            LifecyclePhase::Bound,
            LifecyclePhase::Reviewed,
            LifecyclePhase::Published,
            LifecyclePhase::MergeReady,
            LifecyclePhase::Merged,
            LifecyclePhase::ClosedOut,
        ] {
            let error = authorize_card_operation(
                phase,
                CardKind::Stp,
                &SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                    values: vec!["src/late.rs".into()],
                },
            )
            .expect_err("STP deliverable correction is implemented-only");
            assert_eq!(error.code, ErrorCode::InvalidTransition);
        }
        let error = authorize_card_operation(
            LifecyclePhase::Implemented,
            CardKind::Sip,
            &SemanticOperation::CorrectStpDeliverablesAfterRecovery {
                values: vec!["src/wrong-card.rs".into()],
            },
        )
        .expect_err("STP deliverable correction is STP-only");
        assert_eq!(error.code, ErrorCode::InvalidTransition);
    }

    #[test]
    fn post_review_spp_replacements_remain_rejected() {
        for phase in [
            LifecyclePhase::Reviewed,
            LifecyclePhase::Published,
            LifecyclePhase::MergeReady,
        ] {
            for operation in [
                SemanticOperation::ReplacePlanSteps {
                    steps: replacement_steps(),
                },
                SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::Invariants,
                    values: vec!["late invariant".into()],
                },
                SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::StopConditions,
                    values: vec!["late stop".into()],
                },
            ] {
                let error = authorize_card_operation(phase, CardKind::Spp, &operation)
                    .expect_err("late SPP replacement remains rejected");
                assert_eq!(error.code, ErrorCode::InvalidTransition);
            }
        }
    }
}
#[cfg(test)]
mod pre_field_compatibility_tests {
    use super::*;

    fn pre_field_record(issue: u64) -> IssueRecord {
        let mut record = IssueRecord {
            schema: "csdlc.issue.index.v1".into(),
            issue,
            repository: "example/repo".into(),
            code_repository: None,
            initialization_digest: "initialization".into(),
            phase: LifecyclePhase::Bound,
            generation: 1,
            digest: String::new(),
            branch: Some(format!("issue-{issue}")),
            worktree: Some(format!(".worktrees/issue-{issue}")),
            review_assignment: None,
            review: None,
            publication: None,
            readiness: None,
            terminal: None,
            migration: None,
            design_path: format!("design/issue-{issue}.md"),
            diagram_path: format!("design/issue-{issue}.mmd"),
            design_review: DesignReview::Pending,
            cards: BTreeMap::new(),
            transitions: Vec::new(),
            audit: Vec::new(),
        };
        record.digest = record_digest(&record).expect("pre-field digest");
        record
    }

    #[test]
    fn absent_code_repository_preserves_pre_field_record_and_receipt_digests() {
        let record = pre_field_record(45);
        let value = serde_json::to_value(&record).expect("pre-field record JSON");
        assert!(value.get("code_repository").is_none());
        let decoded_record: IssueRecord = serde_json::from_value(value).expect("current record");
        assert_eq!(
            decoded_record.digest,
            record_digest(&decoded_record).expect("decoded record digest")
        );

        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue: 45,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref: "csdlc-v2/closeout/45.json".into(),
            authored_artifacts: BTreeMap::new(),
            record: decoded_record,
            cards: BTreeMap::new(),
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt).expect("terminal receipt digest");
        let encoded_receipt = serde_json::to_value(&receipt).expect("encoded terminal receipt");
        assert!(encoded_receipt["record"].get("code_repository").is_none());
        let decoded_receipt: TerminalReceipt =
            serde_json::from_value(encoded_receipt).expect("decoded terminal receipt");
        assert_eq!(
            decoded_receipt.record.digest,
            record_digest(&decoded_receipt.record).expect("receipt record digest")
        );
        assert_eq!(
            decoded_receipt.digest,
            terminal_receipt_digest(&decoded_receipt).expect("decoded receipt digest")
        );
    }
}
