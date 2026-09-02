use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::cards::{
    apply, digest, initial_cards, render, terminal_validation_passed, validate_cross_card,
    validate_identity_version, validate_result, CardContent, CardKind, CardStatus, CardValues,
    InitialCardInput, PlanStep, SemanticOperation, StepStatus, ValidationResult,
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

    pub fn rollback_preserved(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.rollback-preserved"))
    }

    fn staging_dir(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.staging"))
    }

    pub(crate) fn lock(&self, issue: u64) -> Result<File> {
        crate::operator::verify_installed_owner_preflight(&self.root)?;
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
        crate::operator::verify_installed_owner_preflight(&self.root)?;
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

    pub(crate) fn load_legacy_terminal_receipt_projection_match(
        &self,
        issue: u64,
    ) -> Result<Option<bool>> {
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
        if receipt.schema != "csdlc.terminal_receipt.v1"
            || receipt.issue != issue
            || receipt.issue != receipt.record.issue
            || receipt.repository != receipt.record.repository
            || receipt.initialization_digest != receipt.record.initialization_digest
            || receipt.receipt_ref != format!("csdlc-v2/closeout/{issue}.json")
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "legacy terminal receipt identity is invalid",
            ));
        }
        Ok(Some(self.legacy_receipt_matches_projection(&receipt)?))
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
        let recovery_root = self
            .root
            .join(".csdlc/issues")
            .join(format!(".{issue}.recovery"));
        let recovery_root = match recovery_root.symlink_metadata() {
            Ok(_) => Some(crate::projection_recovery::open_private_recovery_dir(
                &recovery_root,
                "recovery root",
                None,
            )?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        };
        if let Some(recovery_root) = recovery_root {
            for name in recovery_root.names()? {
                let operation = name.to_str().unwrap_or_default();
                let attempt = match recovery_root.open_child(&name, "recovery attempt") {
                    Ok(attempt) => attempt,
                    Err(error) => {
                        if crate::projection_recovery::is_authorized_cleanup_ledger_entry(
                            self,
                            issue,
                            &recovery_root,
                            operation,
                        )? {
                            continue;
                        }
                        return Err(error);
                    }
                };
                if let Err(error) =
                    crate::projection_recovery::validate_completed_recovery_attempt_from_dir(
                        self, issue, &attempt, operation,
                    )
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("incomplete typed projection recovery must reach verified RECOVERED before ordinary commit: {}", error.message),
                    ));
                }
            }
        }
        self.recover_local_transaction(issue)?;
        self.recover_initialized_recovery_journal(issue)
    }

    fn initialized_recovery_journal_root(&self, issue: u64) -> PathBuf {
        self.root
            .join(".csdlc/issues")
            .join(format!(".{issue}.recovery-journal"))
    }

    fn recover_initialized_recovery_journal(&self, issue: u64) -> Result<()> {
        let journal_root = self.initialized_recovery_journal_root(issue);
        if !journal_root.exists() {
            return Ok(());
        }
        for entry in fs::read_dir(&journal_root)? {
            let entry = entry?;
            let transaction = entry.path();
            if !entry.file_type()?.is_dir() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "initialized recovery journal contains a non-directory entry",
                ));
            }
            let prepared = transaction.join("manifest.prepared.json");
            if !prepared.exists() {
                fs::remove_dir_all(&transaction)?;
                continue;
            }
            let manifest: InitializedRecoveryJournalManifest = read_json(&prepared)?;
            roll_forward_initialized_recovery(self, &transaction, &manifest)?;
            fs::write(transaction.join("commit.marker"), b"committed\n")?;
            fs::remove_dir_all(&transaction)?;
        }
        if fs::read_dir(&journal_root)?.next().is_none() {
            fs::remove_dir(&journal_root)?;
        }
        Ok(())
    }

    fn commit(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
    ) -> Result<()> {
        self.commit_with_authored(issue, record, cards, fail_after_backup, None, None)
    }

    fn commit_verified(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
        verifier: &mut dyn FnMut() -> Result<()>,
    ) -> Result<()> {
        self.commit_with_authored(
            issue,
            record,
            cards,
            fail_after_backup,
            None,
            Some(verifier),
        )
    }

    fn commit_with_authored(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        fail_after_backup: bool,
        authored_overrides: Option<&BTreeMap<String, String>>,
        mut verifier: Option<&mut dyn FnMut() -> Result<()>>,
    ) -> Result<()> {
        let current = self.issue_dir(issue);
        let staging = self.staging_dir(issue);
        let backup = self.interrupted_backup(issue);
        let rollback_preserved = self.rollback_preserved(issue);
        if rollback_preserved.symlink_metadata().is_ok() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "preserved failed projection requires typed classification and recovery",
            ));
        }
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
                    let mut file = File::create(&destination)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
                } else if source.is_file() {
                    fs::copy(source, &destination)?;
                    File::open(&destination)?.sync_all()?;
                }
                sync_dirs_through(
                    destination.parent().expect("authored destination parent"),
                    &staging,
                )?;
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
                    let mut file = File::create(&staged)?;
                    file.write_all(contents.as_bytes())?;
                    file.sync_all()?;
                    sync_dirs_through(
                        staged.parent().expect("authored override parent"),
                        &staging,
                    )?;
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
        if let Some(verify) = verifier.as_mut() {
            if let Err(error) = verify() {
                preserve_failed_projection_and_restore(&current, &backup, &rollback_preserved)?;
                return Err(error);
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

    #[allow(dead_code)] // Retained compatibility wrapper; live recovery uses descriptor-read bytes.
    pub(crate) fn projection_recovery_candidate_files_locked(
        &self,
        issue: u64,
        source: &Path,
        expected_digest: &str,
        actor: String,
        reason: String,
        operation: String,
    ) -> Result<(IssueRecord, BTreeMap<String, Vec<u8>>)> {
        let mut record: IssueRecord = read_json(&source.join("index.json"))?;
        if record.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery source projection namespace mismatch",
            ));
        }
        if record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before recovery audit commit",
            ));
        }
        let mut cards = BTreeMap::new();
        for kind in enum_iterator() {
            cards.insert(
                kind,
                read_json(&source.join("cards").join(format!("{kind}.values.json")))?,
            );
        }
        verify_record(&record)?;
        for (kind, values) in &cards {
            let rendered = render(values)?;
            let projection = record.cards.get(kind).ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("missing {kind} projection"),
                )
            })?;
            if values.kind() != *kind
                || values.identity.issue != record.issue
                || values.identity.repository != record.repository
                || values.identity.generation != record.generation
                || projection.values_digest != rendered.values_digest
                || projection.rendered_digest != rendered.rendered_digest
                || projection.ast_digest != rendered.ast_digest
                || digest(&fs::read(source.join("cards").join(format!("{kind}.md")))?)
                    != rendered.rendered_digest
            {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("recovery source {kind} projection drift"),
                ));
            }
        }
        let mut expected_audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut expected_audit, event)?;
            expected_audit.push(b'\n');
        }
        if fs::read(source.join("audit.jsonl"))? != expected_audit {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery source audit projection drift",
            ));
        }
        validate_updated_cards(self, &record, &cards)?;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason,
            operation,
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let mut files = BTreeMap::new();
        let mut index = serde_json::to_vec_pretty(&record)?;
        index.push(b'\n');
        files.insert("index.json".into(), index);
        let mut audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut audit, event)?;
            audit.push(b'\n');
        }
        files.insert("audit.jsonl".into(), audit);
        for (kind, values) in &cards {
            let mut value_bytes = serde_json::to_vec_pretty(values)?;
            value_bytes.push(b'\n');
            files.insert(format!("cards/{kind}.values.json"), value_bytes);
            files.insert(
                format!("cards/{kind}.md"),
                render(values)?.markdown.into_bytes(),
            );
        }
        Ok((record, files))
    }

    pub(crate) fn projection_recovery_candidate_files_from_bytes_locked(
        &self,
        issue: u64,
        source: &BTreeMap<String, Vec<u8>>,
        expected_digest: &str,
        actor: String,
        reason: String,
        operation: String,
    ) -> Result<(IssueRecord, BTreeMap<String, Vec<u8>>)> {
        let mut record: IssueRecord =
            serde_json::from_slice(source.get("index.json").ok_or_else(|| {
                V2Error::new(ErrorCode::CorruptRecord, "recovery source index missing")
            })?)?;
        if record.issue != issue || record.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "recovery source identity or digest changed",
            ));
        }
        let mut cards = BTreeMap::new();
        for kind in enum_iterator() {
            let key = format!("cards/{kind}.values.json");
            cards.insert(
                kind,
                serde_json::from_slice(source.get(&key).ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        format!("recovery source {kind} missing"),
                    )
                })?)?,
            );
        }
        verify_record(&record)?;
        for (kind, values) in &cards {
            let rendered = render(values)?;
            let projection = record.cards.get(kind).ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("missing {kind} projection"),
                )
            })?;
            if values.kind() != *kind
                || values.identity.issue != record.issue
                || values.identity.repository != record.repository
                || values.identity.generation != record.generation
                || projection.values_digest != rendered.values_digest
                || projection.rendered_digest != rendered.rendered_digest
                || projection.ast_digest != rendered.ast_digest
                || digest(source.get(&format!("cards/{kind}.md")).ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        format!("recovery source {kind} markdown missing"),
                    )
                })?) != rendered.rendered_digest
            {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("recovery source {kind} projection drift"),
                ));
            }
        }
        let mut expected_audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut expected_audit, event)?;
            expected_audit.push(b'\n');
        }
        if source.get("audit.jsonl") != Some(&expected_audit) {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "recovery source audit projection drift",
            ));
        }
        validate_updated_cards(self, &record, &cards)?;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason,
            operation,
        });
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let mut files = BTreeMap::new();
        let mut index = serde_json::to_vec_pretty(&record)?;
        index.push(b'\n');
        files.insert("index.json".into(), index);
        let mut audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut audit, event)?;
            audit.push(b'\n');
        }
        files.insert("audit.jsonl".into(), audit);
        for (kind, values) in &cards {
            let mut value_bytes = serde_json::to_vec_pretty(values)?;
            value_bytes.push(b'\n');
            files.insert(format!("cards/{kind}.values.json"), value_bytes);
            files.insert(
                format!("cards/{kind}.md"),
                render(values)?.markdown.into_bytes(),
            );
        }
        Ok((record, files))
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

    pub(crate) fn commit_initialized_code_repository_migration(
        &self,
        request: &crate::migration::InitializedCodeRepositoryMigrationRequest,
    ) -> Result<(
        IssueRecord,
        crate::migration::InitializedCodeRepositoryMigrationEvidence,
    )> {
        let _lock = self.lock(request.issue)?;
        self.recover_if_needed(request.issue)?;
        let mut record = self.load_record(request.issue)?;
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        authorize_initialized_code_repository_migration(&self.root, &record, request)?;

        // Re-read and reauthorize immediately before mutation while both the
        // binding and issue locks remain held.
        record = self.load_record(request.issue)?;
        let evidence =
            authorize_initialized_code_repository_migration(&self.root, &record, request)?;
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
        let (design_ref, design_digest, diagram_ref, diagram_digest) =
            match &cards[&CardKind::Spp].content {
                CardContent::Spp(values) => (
                    values.design_ref.clone(),
                    values.design_digest.clone(),
                    values.diagram_ref.clone(),
                    values.diagram_digest.clone(),
                ),
                _ => unreachable!("SPP"),
            };
        let refresh_used = record.audit.iter().any(|event| {
            event
                .operation
                .contains("refresh_authored_design_after_recovery")
        });
        let tuple_approved = !refresh_used
            || record
                .audit
                .iter()
                .rev()
                .find_map(|event| {
                    let value: serde_json::Value = serde_json::from_str(&event.operation).ok()?;
                    (value["operation"] == "approve_design").then_some(value)
                })
                .is_some_and(|value| {
                    value["design_ref"] == record.design_path
                        && value["design_digest"] == design_digest
                        && value["diagram_ref"] == record.diagram_path
                        && value["diagram_digest"] == diagram_digest
                });
        if !matches!(&record.design_review, DesignReview::Approved { revision, .. } if revision == &design_digest)
            || !tuple_approved
            || authored_digest(self, &record.design_path)? != design_digest
            || authored_digest(self, &record.diagram_path)? != diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review assignment requires current approved authored design tuple at commit",
            ));
        }
        let mut design_artifact = retain_authored_artifact(self.root(), Path::new(&design_ref))?;
        let mut diagram_artifact = retain_authored_artifact(self.root(), Path::new(&diagram_ref))?;
        if design_artifact.verify()? != design_digest
            || diagram_artifact.verify()? != diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "review assignment authored tuple changed before commit",
            ));
        }
        let registered = record.worktree.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "review assignment lost registered worktree at commit",
            )
        })?;
        let branch = record.branch.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "review assignment lost registered branch at commit",
            )
        })?;
        if fs::canonicalize(self.root())? != fs::canonicalize(registered)?
            || crate::git::current_branch(self.root())? != *branch
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "review assignment topology changed before commit",
            ));
        }
        let final_head = crate::git::run(self.root(), &["rev-parse", "HEAD"])?.stdout;
        let final_revision = crate::git::substantive_revision(self.root(), &assignment.scope)?;
        if final_revision != assignment.revision
            || final_revision != crate::git::clean_commit_revision(&final_head)
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "review assignment HEAD or substantive revision changed before commit",
            ));
        }
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
        let expected_design = design_digest;
        let expected_diagram = diagram_digest;
        let issue_dir = self.issue_dir(issue);
        let mut verifier = || {
            if design_artifact.verify_after_projection_swap(self.root(), &issue_dir)?
                != expected_design
                || diagram_artifact.verify_after_projection_swap(self.root(), &issue_dir)?
                    != expected_diagram
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "review assignment authored tuple changed across commit",
                ));
            }
            Ok(())
        };
        self.commit_verified(issue, &record, &cards, false, &mut verifier)?;
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

fn authorize_initialized_code_repository_migration(
    root: &Path,
    record: &IssueRecord,
    request: &crate::migration::InitializedCodeRepositoryMigrationRequest,
) -> Result<crate::migration::InitializedCodeRepositoryMigrationEvidence> {
    if record.issue != request.issue || record.repository != request.source_issue_repository {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "initialized code repository migration issue identity does not match",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "initialized code repository migration digest is stale",
        ));
    }
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "initialized code repository migration generation is stale",
        ));
    }
    if record.phase != LifecyclePhase::Initialized {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "initialized code repository migration requires initialized phase",
        ));
    }
    if record.code_repository.is_some() {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "initialized code repository migration requires an absent code_repository",
        ));
    }
    if record.branch.is_some()
        || record.worktree.is_some()
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "initialized code repository migration requires unbound topology and no later lifecycle evidence",
        ));
    }
    let evidence_path = root.join(&request.canonical_issue_collision_evidence_ref);
    let evidence_bytes = fs::read(&evidence_path)?;
    let evidence_digest = crate::cards::digest(&evidence_bytes);
    if evidence_digest != request.canonical_issue_collision_evidence_digest {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical issue collision evidence digest does not match request",
        ));
    }
    let collision: crate::migration::InitializedCodeRepositoryCollisionEvidence =
        serde_json::from_slice(&evidence_bytes)?;
    if collision.schema != "csdlc.initialized_code_repository_collision_evidence.v1"
        || collision.source_issue_repository != request.source_issue_repository
        || collision.source_issue != request.issue
        || !collision
            .target_code_repository
            .eq_ignore_ascii_case(&request.code_repository)
        || collision.observed_state.trim().is_empty()
        || collision.operation_key.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "canonical issue collision evidence does not match request",
        ));
    }
    match collision.disposition {
        crate::migration::InitializedCanonicalCollisionDisposition::SameNumberAbsent => {
            if collision.target_same_number_issue.is_some() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "same-number-absent collision evidence must not name a target issue",
                ));
            }
        }
        crate::migration::InitializedCanonicalCollisionDisposition::SameNumberNonAuthoritative
        | crate::migration::InitializedCanonicalCollisionDisposition::SameNumberSuccessor => {
            if collision.target_same_number_issue != Some(request.issue) {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "same-number collision evidence must name the matching target issue number",
                ));
            }
        }
    }
    let origins = crate::git::github_remote_repositories(root, "origin")?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "initialized code repository migration requires an origin remote",
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
    Ok(
        crate::migration::InitializedCodeRepositoryMigrationEvidence {
            schema: "csdlc.initialized_code_repository_migration_evidence.v1".into(),
            issue: record.issue,
            actor: request.actor.clone(),
            reason: request.reason.clone(),
            pre_generation: record.generation,
            pre_digest: record.digest.clone(),
            previous_code_repository: None,
            source_issue_repository: request.source_issue_repository.clone(),
            requested_repository: request.code_repository.clone(),
            canonical_issue_collision_evidence_ref: request
                .canonical_issue_collision_evidence_ref
                .clone(),
            canonical_issue_collision_evidence_digest: request
                .canonical_issue_collision_evidence_digest
                .clone(),
            canonical_issue_collision_disposition: collision.disposition.clone(),
            cross_repository_authority_disposition:
                "legacy_issue_authority_with_canonical_code_repository".into(),
            topology_state: "initialized_unbound".into(),
            phase: record.phase,
            branch: None,
            worktree: None,
        },
    )
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RecoverDesignReviewRequest {
    pub issue: u64,
    pub expected_phase: LifecyclePhase,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub previous_reviewer: String,
    pub previous_revision: String,
    pub false_reviewer: String,
    pub actor: String,
    pub reason: String,
    pub disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PreservedAuthoredArtifact {
    pub path: String,
    pub byte_sha256: String,
    pub authored_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompositionGraphNode {
    pub node_id: String,
    pub issue: u64,
    pub role: String,
    pub repository: String,
    pub in_scope: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompositionGraphEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DecompositionGraphInput {
    pub nodes: Vec<DecompositionGraphNode>,
    pub edges: Vec<DecompositionGraphEdge>,
    pub parent_integration_owner: String,
    #[serde(default)]
    pub forbidden_cross_child_trust_redefinition: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DesignReviewRecoveryTruth {
    pub previous_review_state: DesignReview,
    pub new_review_state: DesignReview,
    pub false_reviewer: String,
    pub disposition: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InitializedDecompositionRecoveryReplacement {
    pub title: String,
    pub slug: String,
    pub version: String,
    pub goal: String,
    pub required_outcome: String,
    pub declared_scope: Vec<String>,
    pub authority_boundary: Vec<String>,
    pub initial_assumptions: Vec<String>,
    pub operator_constraints: Vec<String>,
    pub task_boundary: String,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<String>,
    pub repo_inputs: Vec<String>,
    pub non_goals: Vec<String>,
    pub plan_summary: String,
    pub plan_steps: Vec<crate::cards::PlanStep>,
    pub affected_areas: Vec<String>,
    pub invariants: Vec<String>,
    pub risks: Vec<String>,
    pub stop_conditions: Vec<String>,
    pub replan_triggers: Vec<String>,
    pub validation_summary: String,
    pub validation_lanes: Vec<crate::cards::ValidationLane>,
    pub failure_policy: String,
    pub review_scope: String,
    pub review_prompts: Vec<String>,
    #[serde(default)]
    pub residual_risk: Vec<String>,
    pub sor_summary: String,
    #[serde(default)]
    pub sor_artifacts: Vec<String>,
    #[serde(default)]
    pub sor_validation: Vec<crate::cards::ValidationResult>,
    #[serde(default)]
    pub sor_follow_ups: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InitializedRecoveryFailurePoint {
    BeforePreparedManifest,
    AfterPreparedManifest,
    AfterFirstTarget,
    AfterCommitMarker,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InitializedDecompositionRecoveryRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
    pub request_root: PathBuf,
    pub recovery_scope: Vec<String>,
    pub preserved_design: PreservedAuthoredArtifact,
    pub preserved_diagram: PreservedAuthoredArtifact,
    pub graph: DecompositionGraphInput,
    #[serde(default)]
    pub design_review_recovery: Option<DesignReviewRecoveryTruth>,
    pub replacements: InitializedDecompositionRecoveryReplacement,
    #[serde(default)]
    pub fail_at: Option<InitializedRecoveryFailurePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InitializedDecompositionRecoveryResult {
    pub schema: String,
    pub issue: u64,
    pub generation: u64,
    pub digest: String,
    pub journal: String,
    pub applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitializedRecoveryJournalManifest {
    schema: String,
    issue: u64,
    generation: u64,
    digest: String,
    targets: Vec<InitializedRecoveryJournalTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitializedRecoveryJournalTarget {
    path: String,
    preimage_sha256: String,
    postimage_sha256: String,
    blob: String,
    len: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RecoverInitializedDesignEnvelopeRequest {
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub expected_design_path: String,
    pub expected_diagram_path: String,
    pub expected_design_digest: String,
    pub expected_diagram_digest: String,
    pub new_design_path: String,
    pub new_diagram_path: String,
    pub prior_reviewer: String,
    pub canonical_reviewer: String,
    pub reviewer_session_uuid: String,
    pub reviewer_turn_uuid: String,
    pub spawned_task: String,
    pub thread_source: String,
    pub fork_turns: String,
    pub reviewed_generation: u64,
    pub reviewed_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DesignEnvelopeRecoveryJournal {
    schema: String,
    issue: u64,
    pre_generation: u64,
    pre_digest: String,
    post_generation: u64,
    old_design_path: String,
    old_diagram_path: String,
    new_design_path: String,
    new_diagram_path: String,
    design_digest: String,
    diagram_digest: String,
    #[serde(default)]
    design_identity: Option<(u64, u64)>,
    #[serde(default)]
    diagram_identity: Option<(u64, u64)>,
    #[serde(default)]
    post_state_digest: Option<String>,
    #[serde(default)]
    audit_sequence: Option<u64>,
    phase: String,
    #[serde(default)]
    attempt_id: String,
    #[serde(default)]
    sequence: u64,
    #[serde(default)]
    prior_receipt_digest: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesignRecoveryFailpoint {
    AfterPreparedReceipt,
    AfterDesignInstall,
    AfterDesignReceipt,
    AfterDiagramInstall,
    AfterArtifactsReceipt,
    AfterStatePreparedReceipt,
    BeforeStateCommit,
    AfterStateCommit,
}

pub fn recover_initialized_design_envelope(
    store: &Store,
    request: RecoverInitializedDesignEnvelopeRequest,
) -> Result<IssueRecord> {
    recover_initialized_design_envelope_with_hook(store, request, |_| false)
}

pub fn recover_initialized_design_envelope_with_hook(
    store: &Store,
    request: RecoverInitializedDesignEnvelopeRequest,
    mut hook: impl FnMut(DesignRecoveryFailpoint) -> bool,
) -> Result<IssueRecord> {
    macro_rules! checkpoint {
        ($point:expr) => {
            if hook($point) {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "injected design-recovery interruption",
                ));
            }
        };
    }
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    let mut record = store.load_record(request.issue)?;
    reconcile_design_envelope_journal(store, &record)?;
    record = store.load_record(request.issue)?;
    if record.phase != LifecyclePhase::Initialized
        || record.branch.is_some()
        || record.worktree.is_some()
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.migration.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(ErrorCode::InvalidTransition, "design-envelope recovery requires an initialized unbound issue without later lifecycle evidence"));
    }
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "design-envelope recovery generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "design-envelope recovery digest is stale",
        ));
    }
    if record.design_path != request.expected_design_path
        || record.diagram_path != request.expected_diagram_path
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact source paths drifted",
        ));
    }
    validate_recovery_provenance(&request)?;
    match &record.design_review {
        DesignReview::Approved { reviewer, revision }
            if reviewer == &request.prior_reviewer && revision == &request.reviewed_digest => {}
        _ => {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "prior reviewer provenance does not match canonical design review",
            ))
        }
    }
    validate_safe_authored_destination(&request.new_design_path)?;
    validate_safe_authored_destination(&request.new_diagram_path)?;
    if request.new_design_path == request.new_diagram_path
        || request.new_design_path == request.expected_design_path
        || request.new_design_path == request.expected_diagram_path
        || request.new_diagram_path == request.expected_design_path
        || request.new_diagram_path == request.expected_diagram_path
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design-envelope recovery paths must be pairwise distinct",
        ));
    }
    let design_bytes = read_regular_authored_artifact_with_hook(
        store.root(),
        Path::new(&record.design_path),
        |_| {},
    )?
    .ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored design artifact is absent",
        )
    })?;
    let diagram_bytes = read_regular_authored_artifact_with_hook(
        store.root(),
        Path::new(&record.diagram_path),
        |_| {},
    )?
    .ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored diagram artifact is absent",
        )
    })?;
    let old_design_digest = digest(&design_bytes);
    let old_diagram_digest = digest(&diagram_bytes);
    #[cfg(unix)]
    if file_identity_no_follow(store.root(), Path::new(&record.design_path))?
        == file_identity_no_follow(store.root(), Path::new(&record.diagram_path))?
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram sources must not alias the same inode",
        ));
    }
    if old_design_digest != request.expected_design_digest
        || old_diagram_digest != request.expected_diagram_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "authored artifact digest drifted",
        ));
    }
    if request.reviewed_generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "reviewed generation must equal the approved canonical generation",
        ));
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "system clock precedes epoch",
            )
        })?
        .as_nanos();
    let attempt_id = digest(
        format!(
            "{}:{}:{}:{}:{}:{}",
            request.issue,
            request.expected_digest,
            request.reviewer_session_uuid,
            request.reviewer_turn_uuid,
            std::process::id(),
            nonce
        )
        .as_bytes(),
    );
    let mut journal = DesignEnvelopeRecoveryJournal {
        schema: "csdlc.initialized_design_envelope_recovery_journal.v1".into(),
        issue: request.issue,
        pre_generation: record.generation,
        pre_digest: record.digest.clone(),
        post_generation: record.generation + 1,
        old_design_path: request.expected_design_path.clone(),
        old_diagram_path: request.expected_diagram_path.clone(),
        new_design_path: request.new_design_path.clone(),
        new_diagram_path: request.new_diagram_path.clone(),
        design_digest: old_design_digest.clone(),
        diagram_digest: old_diagram_digest.clone(),
        design_identity: None,
        diagram_identity: None,
        post_state_digest: None,
        audit_sequence: None,
        phase: "prepared".into(),
        attempt_id,
        sequence: 0,
        prior_receipt_digest: None,
    };
    let mut receipt_digest = write_recovery_journal(store.root(), &journal)?;
    checkpoint!(DesignRecoveryFailpoint::AfterPreparedReceipt);
    journal.design_identity = Some(stage_and_install_authored(
        store.root(),
        Path::new(&request.new_design_path),
        &design_bytes,
    )?);
    checkpoint!(DesignRecoveryFailpoint::AfterDesignInstall);
    journal.phase = "design_installed".into();
    journal.sequence = 10;
    journal.prior_receipt_digest = Some(receipt_digest);
    receipt_digest = write_recovery_journal(store.root(), &journal)?;
    checkpoint!(DesignRecoveryFailpoint::AfterDesignReceipt);
    cleanup_stage(
        store.root(),
        Path::new(&request.new_design_path),
        &old_design_digest,
    )?;
    match stage_and_install_authored(
        store.root(),
        Path::new(&request.new_diagram_path),
        &diagram_bytes,
    ) {
        Ok(identity) => journal.diagram_identity = Some(identity),
        Err(error) => {
            let _ = remove_authored_if_owned(
                store.root(),
                Path::new(&request.new_design_path),
                &old_design_digest,
                journal.design_identity,
            );
            return Err(error);
        }
    }
    checkpoint!(DesignRecoveryFailpoint::AfterDiagramInstall);
    journal.phase = "artifacts_installed".into();
    journal.sequence = 20;
    journal.prior_receipt_digest = Some(receipt_digest);
    receipt_digest = write_recovery_journal(store.root(), &journal)?;
    checkpoint!(DesignRecoveryFailpoint::AfterArtifactsReceipt);
    cleanup_stage(
        store.root(),
        Path::new(&request.new_diagram_path),
        &old_diagram_digest,
    )?;
    let mut cards = store.load_cards(request.issue)?;
    verify_card_projections(store, &record, &cards)?;
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("design card").content {
            CardContent::Spp(values) => {
                values.design_ref = request.new_design_path.clone();
                values.design_digest = old_design_digest.clone();
                values.diagram_ref = request.new_diagram_path.clone();
                values.diagram_digest = old_diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_ref = request.new_design_path.clone();
                values.design_digest = old_design_digest.clone();
                values.diagram_ref = request.new_diagram_path.clone();
                values.diagram_digest = old_diagram_digest.clone();
            }
            _ => unreachable!(),
        }
    }
    let old_review = record.design_review.clone();
    record.design_path = request.new_design_path.clone();
    record.diagram_path = request.new_diagram_path.clone();
    record.design_review = DesignReview::Pending;
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: "recover initialized design envelope and require fresh approval".into(),
        operation: serde_json::json!({
            "operation":"recover_initialized_design_envelope",
            "old_reviewer":request.prior_reviewer,"new_reviewer":request.canonical_reviewer,
            "old_design_path":request.expected_design_path,"new_design_path":request.new_design_path,
            "old_diagram_path":request.expected_diagram_path,"new_diagram_path":request.new_diagram_path,
            "old_design_digest":old_design_digest,"new_design_digest":old_design_digest,
            "old_diagram_digest":old_diagram_digest,"new_diagram_digest":old_diagram_digest,
            "reviewed_generation":request.reviewed_generation,"reviewed_digest":request.reviewed_digest,
            "reviewer_session_uuid":request.reviewer_session_uuid,"reviewer_turn_uuid":request.reviewer_turn_uuid,
            "spawned_task":request.spawned_task,"thread_source":request.thread_source,"fork_turns":request.fork_turns,
            "old_design_review":old_review,"approval_disposition":"invalidated_pending_reapproval",
            "pre_state_digest":request.expected_digest,"post_generation":record.generation,
            "post_state_digest":"pending-canonical-post-state-digest"
        }).to_string(),
    });
    hydrate_projections(&mut record, &cards)?;
    let post_state_digest = canonical_recovery_post_state_digest(&record)?;
    let operation = &mut record.audit.last_mut().expect("recovery audit").operation;
    let mut operation_value: serde_json::Value = serde_json::from_str(operation)?;
    operation_value["post_state_digest"] = serde_json::Value::String(post_state_digest);
    *operation = operation_value.to_string();
    journal.post_state_digest = Some(canonical_recovery_post_state_digest(&record)?);
    journal.audit_sequence = record.audit.last().map(|event| event.sequence);
    journal.phase = "state_prepared".into();
    journal.sequence = 30;
    journal.prior_receipt_digest = Some(receipt_digest);
    let _receipt_digest = write_recovery_journal(store.root(), &journal)?;
    checkpoint!(DesignRecoveryFailpoint::AfterStatePreparedReceipt);
    record.digest = record_digest(&record)?;
    checkpoint!(DesignRecoveryFailpoint::BeforeStateCommit);
    if let Err(error) = store.commit(request.issue, &record, &cards, false) {
        if store.load_record(request.issue).is_ok_and(|installed| {
            installed.digest == record.digest && recovery_post_state_matches(&installed, &journal)
        }) {
            resolve_recovery_ledger(store.root(), &journal)?;
            return Ok(record);
        }
        let _ = remove_authored_if_owned(
            store.root(),
            Path::new(&request.new_design_path),
            &old_design_digest,
            journal.design_identity,
        );
        let _ = remove_authored_if_owned(
            store.root(),
            Path::new(&request.new_diagram_path),
            &old_diagram_digest,
            journal.diagram_identity,
        );
        return Err(error);
    }
    checkpoint!(DesignRecoveryFailpoint::AfterStateCommit);
    resolve_recovery_ledger(store.root(), &journal)?;
    Ok(record)
}

fn recovery_journal_relative(journal: &DesignEnvelopeRecoveryJournal) -> PathBuf {
    PathBuf::from("csdlc-v2/recovery-journals")
        .join(journal.issue.to_string())
        .join(&journal.attempt_id)
        .join(format!("{:03}-{}.json", journal.sequence, journal.phase))
}

fn git_common_dir(root: &Path) -> Result<PathBuf> {
    Ok(crate::git::shared_request_path(root, 1)?
        .parent()
        .and_then(Path::parent)
        .expect("Git common csdlc-v2")
        .parent()
        .expect("Git common directory")
        .to_path_buf())
}

fn write_recovery_journal(root: &Path, journal: &DesignEnvelopeRecoveryJournal) -> Result<String> {
    let common = git_common_dir(root)?;
    let relative = recovery_journal_relative(journal);
    let bytes = serde_json::to_vec_pretty(journal)?;
    write_authored_exclusive(&common, &relative, &bytes)?;
    Ok(digest(&bytes))
}

fn read_recovery_journal(root: &Path, issue: u64) -> Result<Option<DesignEnvelopeRecoveryJournal>> {
    let common = git_common_dir(root)?;
    let issue_dir = common
        .join("csdlc-v2/recovery-journals")
        .join(issue.to_string());
    if !issue_dir.exists() {
        return Ok(None);
    }
    let attempts: Vec<_> = std::fs::read_dir(&issue_dir)?.collect::<std::result::Result<_, _>>()?;
    let mut active = Vec::new();
    for attempt in attempts {
        if !attempt.file_type()?.is_dir() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recovery attempt entry is invalid",
            ));
        }
        let mut receipts: Vec<_> =
            std::fs::read_dir(attempt.path())?.collect::<std::result::Result<_, _>>()?;
        receipts.sort_by_key(|entry| entry.file_name());
        let mut prior = None;
        let mut latest = None;
        for entry in receipts {
            if !entry.file_type()?.is_file() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "recovery receipt is not regular",
                ));
            }
            let bytes = std::fs::read(entry.path())?;
            let receipt: DesignEnvelopeRecoveryJournal = serde_json::from_slice(&bytes)?;
            if receipt.issue != issue
                || receipt.attempt_id != attempt.file_name().to_string_lossy()
                || receipt.prior_receipt_digest != prior
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "recovery receipt chain is invalid",
                ));
            }
            prior = Some(digest(&bytes));
            latest = Some(receipt);
        }
        let latest = latest.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recovery attempt ledger is empty",
            )
        })?;
        if latest.phase != "resolved" {
            active.push(latest);
        }
    }
    if active.len() > 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recovery attempt ledger is ambiguous",
        ));
    }
    Ok(active.pop())
}

fn resolve_recovery_ledger(root: &Path, journal: &DesignEnvelopeRecoveryJournal) -> Result<()> {
    let mut resolved = journal.clone();
    resolved.sequence = 90;
    resolved.phase = "resolved".into();
    resolved.prior_receipt_digest = Some(digest(&serde_json::to_vec_pretty(journal)?));
    write_recovery_journal(root, &resolved).map(|_| ())
}

fn recovery_post_state_matches(
    record: &IssueRecord,
    journal: &DesignEnvelopeRecoveryJournal,
) -> bool {
    record.generation == journal.post_generation
        && record.design_path == journal.new_design_path
        && record.diagram_path == journal.new_diagram_path
        && matches!(record.design_review, DesignReview::Pending)
        && journal.post_state_digest.as_ref().is_some_and(|expected| {
            canonical_recovery_post_state_digest(record).is_ok_and(|actual| &actual == expected)
        })
        && record.audit.last().is_some_and(|event| {
            Some(event.sequence) == journal.audit_sequence
                && serde_json::from_str::<serde_json::Value>(&event.operation)
                    .ok()
                    .and_then(|value| {
                        value
                            .get("operation")
                            .and_then(|v| v.as_str())
                            .map(str::to_owned)
                    })
                    .as_deref()
                    == Some("recover_initialized_design_envelope")
        })
}

fn canonical_recovery_post_state_digest(record: &IssueRecord) -> Result<String> {
    let mut value = record.clone();
    value.digest.clear();
    let operation = &mut value
        .audit
        .last_mut()
        .ok_or_else(|| V2Error::new(ErrorCode::ReconciliationRequired, "recovery audit absent"))?
        .operation;
    let mut operation_value: serde_json::Value = serde_json::from_str(operation)?;
    operation_value["post_state_digest"] =
        serde_json::Value::String("pending-canonical-post-state-digest".into());
    *operation = operation_value.to_string();
    Ok(digest(&serde_json::to_vec(&value)?))
}

fn reconcile_design_envelope_journal(store: &Store, record: &IssueRecord) -> Result<()> {
    let Some(journal) = read_recovery_journal(store.root(), record.issue)? else {
        return Ok(());
    };
    if journal.schema != "csdlc.initialized_design_envelope_recovery_journal.v1"
        || journal.issue != record.issue
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design-envelope recovery journal identity is invalid",
        ));
    }
    if recovery_post_state_matches(record, &journal) {
        require_authored_digest(
            store.root(),
            &journal.new_design_path,
            &journal.design_digest,
        )?;
        require_authored_digest(
            store.root(),
            &journal.new_diagram_path,
            &journal.diagram_digest,
        )?;
        return resolve_recovery_ledger(store.root(), &journal);
    }
    if record.generation == journal.pre_generation
        && record.digest == journal.pre_digest
        && record.design_path == journal.old_design_path
        && record.diagram_path == journal.old_diagram_path
    {
        remove_authored_if_owned(
            store.root(),
            Path::new(&journal.new_design_path),
            &journal.design_digest,
            journal.design_identity,
        )?;
        remove_authored_if_owned(
            store.root(),
            Path::new(&journal.new_diagram_path),
            &journal.diagram_digest,
            journal.diagram_identity,
        )?;
        return resolve_recovery_ledger(store.root(), &journal);
    }
    Err(V2Error::new(
        ErrorCode::ReconciliationRequired,
        "recovery journal does not match canonical pre-state or post-state",
    ))
}

fn require_authored_digest(root: &Path, relative: &str, expected: &str) -> Result<()> {
    let actual = read_regular_authored_artifact_with_hook(root, Path::new(relative), |_| {})?
        .map(|bytes| digest(&bytes));
    if actual.as_deref() != Some(expected) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recovered authored artifact digest is absent or drifted",
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn staged_authored_path(relative: &Path) -> Result<PathBuf> {
    let file_name = relative.file_name().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "authored destination file name missing",
        )
    })?;
    Ok(relative.with_file_name(format!(".{}.csdlc-stage", file_name.to_string_lossy())))
}

#[cfg(unix)]
fn cleanup_stage(root: &Path, relative: &Path, expected: &str) -> Result<()> {
    let stage = staged_authored_path(relative)?;
    reconcile_owned_quarantine(root, &stage, expected)?;
    unlink_anchored(root, &stage, Some(expected))
}

#[cfg(unix)]
fn reconcile_owned_quarantine(root: &Path, relative: &Path, expected: &str) -> Result<()> {
    let parent = relative.parent().unwrap_or_else(|| Path::new("."));
    let name = relative
        .file_name()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cleanup file name missing"))?
        .to_string_lossy();
    let prefix = format!(".{name}.csdlc-delete-");
    let directory = root.join(parent);
    if !directory.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let candidate_name = entry.file_name();
        let candidate_text = candidate_name.to_string_lossy();
        let Some(suffix) = candidate_text.strip_prefix(&prefix) else {
            continue;
        };
        let parts: Vec<_> = suffix.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let identity = (
            parts[0].parse::<u64>().map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "quarantine identity invalid",
                )
            })?,
            parts[1].parse::<u64>().map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "quarantine identity invalid",
                )
            })?,
        );
        let candidate = parent.join(candidate_name);
        unlink_owned_anchored(root, &candidate, expected, identity)?;
    }
    Ok(())
}

#[cfg(unix)]
fn reconcile_owned_delete_quarantine(root: &Path, relative: &Path, expected: &str) -> Result<()> {
    let parent = relative.parent().unwrap_or_else(|| Path::new("."));
    let name = relative
        .file_name()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cleanup file name missing"))?
        .to_string_lossy();
    let prefix = format!(".{name}.csdlc-owned-delete-");
    let directory = root.join(parent);
    if !directory.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let candidate_name = entry.file_name();
        let candidate_text = candidate_name.to_string_lossy();
        let Some(suffix) = candidate_text.strip_prefix(&prefix) else {
            continue;
        };
        let parts: Vec<_> = suffix.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let identity = (
            parts[0].parse::<u64>().map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "owned quarantine identity invalid",
                )
            })?,
            parts[1].parse::<u64>().map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "owned quarantine identity invalid",
                )
            })?,
        );
        let candidate = parent.join(candidate_name);
        unlink_owned_anchored(root, &candidate, expected, identity)?;
    }
    Ok(())
}

#[cfg(unix)]
fn stage_and_install_authored(root: &Path, relative: &Path, bytes: &[u8]) -> Result<(u64, u64)> {
    let staged = staged_authored_path(relative)?;
    write_authored_exclusive(root, &staged, bytes)?;
    if let Err(error) = link_no_replace_anchored(root, &staged, relative) {
        let _ = unlink_anchored(root, &staged, Some(&digest(bytes)));
        return Err(error);
    }
    file_identity_no_follow(root, relative)
}

#[cfg(not(unix))]
fn stage_and_install_authored(_root: &Path, _relative: &Path, _bytes: &[u8]) -> Result<(u64, u64)> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "safe authored artifact installation requires anchored platform primitives",
    ))
}

#[cfg(unix)]
fn write_authored_exclusive(root: &Path, relative: &Path, bytes: &[u8]) -> Result<()> {
    use std::ffi::CString;
    use std::io::Write;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    let root_file = File::open(root)?;
    let mut retained = Vec::new();
    let mut directory_fd = root_file.as_raw_fd();
    let components: Vec<_> = relative.components().collect();
    for (index, component) in components.iter().enumerate() {
        let std::path::Component::Normal(name) = component else {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "authored destination contains non-normal component",
            ));
        };
        let name = CString::new(name.as_bytes()).map_err(|_| {
            V2Error::new(ErrorCode::InvalidInput, "authored destination contains NUL")
        })?;
        let last = index + 1 == components.len();
        if last {
            let fd = unsafe {
                libc::openat(
                    directory_fd,
                    name.as_ptr(),
                    libc::O_WRONLY
                        | libc::O_CREAT
                        | libc::O_EXCL
                        | libc::O_CLOEXEC
                        | libc::O_NOFOLLOW,
                    0o644,
                )
            };
            if fd < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            let mut file = unsafe { File::from_raw_fd(fd) };
            file.write_all(bytes)?;
            file.sync_all()?;
        } else {
            let mut fd = unsafe {
                libc::openat(
                    directory_fd,
                    name.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                )
            };
            if fd < 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ENOENT) {
                if unsafe { libc::mkdirat(directory_fd, name.as_ptr(), 0o755) } != 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
                fd = unsafe {
                    libc::openat(
                        directory_fd,
                        name.as_ptr(),
                        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                    )
                };
            }
            if fd < 0 {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "authored destination ancestor is unsafe",
                ));
            }
            let opened = unsafe { File::from_raw_fd(fd) };
            retained.push(opened);
            directory_fd = retained.last().unwrap().as_raw_fd();
        }
    }
    Ok(())
}

#[cfg(unix)]
fn open_parent_no_follow(root: &Path, relative: &Path) -> Result<(File, std::ffi::CString)> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    let root_file = File::open(root)?;
    let mut current = root_file;
    let parent = relative.parent().unwrap_or_else(|| Path::new(""));
    for component in parent.components() {
        let std::path::Component::Normal(name) = component else {
            return Err(V2Error::new(ErrorCode::InvalidInput, "non-normal path"));
        };
        let name = CString::new(name.as_bytes())
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "NUL path"))?;
        let mut fd = unsafe {
            libc::openat(
                current.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if fd < 0 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ENOENT) {
            if unsafe { libc::mkdirat(current.as_raw_fd(), name.as_ptr(), 0o755) } != 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            fd = unsafe {
                libc::openat(
                    current.as_raw_fd(),
                    name.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                )
            };
        }
        if fd < 0 {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "unsafe path ancestor",
            ));
        }
        current = unsafe { File::from_raw_fd(fd) };
    }
    let name = relative
        .file_name()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "file name missing"))?;
    Ok((
        current,
        CString::new(name.as_bytes())
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "NUL path"))?,
    ))
}

#[cfg(unix)]
fn link_no_replace_anchored(root: &Path, source: &Path, destination: &Path) -> Result<()> {
    use std::os::unix::io::AsRawFd;
    let (source_parent, source_name) = open_parent_no_follow(root, source)?;
    let (dest_parent, dest_name) = open_parent_no_follow(root, destination)?;
    if unsafe {
        libc::linkat(
            source_parent.as_raw_fd(),
            source_name.as_ptr(),
            dest_parent.as_raw_fd(),
            dest_name.as_ptr(),
            0,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    dest_parent.sync_all()?;
    Ok(())
}

#[cfg(unix)]
fn unlink_anchored(root: &Path, relative: &Path, expected_digest: Option<&str>) -> Result<()> {
    use std::io::Read;
    use std::os::unix::io::{AsRawFd, FromRawFd};
    let (parent, name) = open_parent_no_follow(root, relative)?;
    if let Some(expected) = expected_digest {
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if fd < 0 {
            let error = std::io::Error::last_os_error();
            if error.raw_os_error() == Some(libc::ENOENT) {
                return Ok(());
            }
            return Err(error.into());
        }
        let mut target = unsafe { File::from_raw_fd(fd) };
        let opened = target.metadata()?;
        let mut bytes = Vec::new();
        target.read_to_end(&mut bytes)?;
        if digest(&bytes) != expected {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "cleanup target digest drifted",
            ));
        }
        use std::os::unix::fs::MetadataExt;
        let quarantine = relative.with_file_name(format!(
            ".{}.csdlc-delete-{}-{}",
            relative
                .file_name()
                .expect("cleanup file name")
                .to_string_lossy(),
            opened.dev(),
            opened.ino()
        ));
        rename_no_replace_anchored(root, relative, &quarantine)?;
        let quarantined = match read_opened_metadata_no_follow(root, &quarantine) {
            Ok(metadata) => metadata,
            Err(error) => {
                let _ = rename_no_replace_anchored(root, &quarantine, relative);
                return Err(error);
            }
        };
        if opened.dev() != quarantined.dev() || opened.ino() != quarantined.ino() {
            let _ = rename_no_replace_anchored(root, &quarantine, relative);
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "cleanup target identity changed",
            ));
        }
        return unlink_anchored(root, &quarantine, None);
    }
    if unsafe { libc::unlinkat(parent.as_raw_fd(), name.as_ptr(), 0) } != 0 {
        let e = std::io::Error::last_os_error();
        if e.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(e.into());
    }
    parent.sync_all()?;
    Ok(())
}

#[cfg(unix)]
fn read_opened_metadata_no_follow(root: &Path, relative: &Path) -> Result<std::fs::Metadata> {
    use std::os::unix::io::{AsRawFd, FromRawFd};
    let (parent, name) = open_parent_no_follow(root, relative)?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(unsafe { File::from_raw_fd(fd) }.metadata()?)
}

#[cfg(unix)]
fn file_identity_no_follow(root: &Path, relative: &Path) -> Result<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let metadata = read_opened_metadata_no_follow(root, relative)?;
    Ok((metadata.dev(), metadata.ino()))
}

#[cfg(unix)]
fn unlink_owned_anchored(
    root: &Path,
    relative: &Path,
    expected_digest: &str,
    expected_identity: (u64, u64),
) -> Result<()> {
    let quarantine = relative.with_file_name(format!(
        ".{}.csdlc-owned-delete-{}-{}",
        relative
            .file_name()
            .expect("cleanup file name")
            .to_string_lossy(),
        expected_identity.0,
        expected_identity.1
    ));
    if file_identity_no_follow(root, &quarantine).is_err() {
        rename_no_replace_anchored(root, relative, &quarantine)?;
    }
    let actual_identity = file_identity_no_follow(root, &quarantine)?;
    if actual_identity != expected_identity {
        rename_no_replace_anchored(root, &quarantine, relative)?;
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "cleanup target is not the journal-owned artifact",
        ));
    }
    unlink_anchored(root, &quarantine, Some(expected_digest))
}

#[cfg(target_os = "macos")]
fn rename_no_replace_anchored(root: &Path, source: &Path, destination: &Path) -> Result<()> {
    use std::os::unix::io::AsRawFd;
    let (source_parent, source_name) = open_parent_no_follow(root, source)?;
    let (dest_parent, dest_name) = open_parent_no_follow(root, destination)?;
    if unsafe {
        libc::renameatx_np(
            source_parent.as_raw_fd(),
            source_name.as_ptr(),
            dest_parent.as_raw_fd(),
            dest_name.as_ptr(),
            libc::RENAME_EXCL,
        )
    } != 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    dest_parent.sync_all()?;
    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn rename_no_replace_anchored(root: &Path, source: &Path, destination: &Path) -> Result<()> {
    use std::os::unix::io::AsRawFd;
    let (source_parent, source_name) = open_parent_no_follow(root, source)?;
    let (dest_parent, dest_name) = open_parent_no_follow(root, destination)?;
    #[cfg(target_os = "linux")]
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source_parent.as_raw_fd(),
            source_name.as_ptr(),
            dest_parent.as_raw_fd(),
            dest_name.as_ptr(),
            libc::RENAME_NOREPLACE,
        ) as libc::c_int
    };
    #[cfg(not(target_os = "linux"))]
    let result = -1;
    if result != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    dest_parent.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn unlink_anchored(_root: &Path, _relative: &Path, _expected: Option<&str>) -> Result<()> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "anchored unlink unavailable",
    ))
}

#[cfg(not(unix))]
fn write_authored_exclusive(_root: &Path, _relative: &Path, _bytes: &[u8]) -> Result<()> {
    Err(V2Error::new(
        ErrorCode::UnsafeCheckout,
        "safe authored artifact writes require anchored no-follow platform primitives",
    ))
}

fn remove_authored_if_owned(
    root: &Path,
    relative: &Path,
    expected: &str,
    identity: Option<(u64, u64)>,
) -> Result<()> {
    reconcile_owned_quarantine(root, &staged_authored_path(relative)?, expected)?;
    reconcile_owned_delete_quarantine(root, relative, expected)?;
    reconcile_owned_delete_quarantine(root, &staged_authored_path(relative)?, expected)?;
    let Some(identity) = identity else {
        // A crash before the journal identity update leaves the retained stage
        // as the ownership witness for the installed hard link.
        let stage = staged_authored_path(relative)?;
        let stage_identity = match file_identity_no_follow(root, &stage) {
            Ok(value) => value,
            Err(_) => {
                if read_regular_authored_artifact_with_hook(root, relative, |_| {})?.is_none() {
                    return Ok(());
                }
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "recovery artifact ownership is unproven",
                ));
            }
        };
        let destination_identity = file_identity_no_follow(root, relative).ok();
        if destination_identity.is_some_and(|value| value != stage_identity) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recovery destination is not linked to its owned stage",
            ));
        }
        if destination_identity.is_some() {
            unlink_owned_anchored(root, relative, expected, stage_identity)?;
        }
        return unlink_owned_anchored(root, &stage, expected, stage_identity);
    };
    let quarantine = relative.with_file_name(format!(
        ".{}.csdlc-delete-{}-{}",
        relative
            .file_name()
            .expect("cleanup file name")
            .to_string_lossy(),
        identity.0,
        identity.1
    ));
    if let Ok(quarantine_identity) = file_identity_no_follow(root, &quarantine) {
        if quarantine_identity != identity {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "quarantined cleanup inode drifted",
            ));
        }
        unlink_owned_anchored(root, &quarantine, expected, identity)?;
    }
    let current_identity = match file_identity_no_follow(root, relative) {
        Ok(value) => value,
        Err(error) if error.message.contains("No such file") => return Ok(()),
        Err(error) => return Err(error),
    };
    if current_identity != identity {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "cleanup target is not the journal-owned inode",
        ));
    }
    let stage = staged_authored_path(relative)?;
    if file_identity_no_follow(root, &stage).ok() == Some(identity) {
        unlink_owned_anchored(root, &stage, expected, identity)?;
    }
    match read_regular_authored_artifact_with_hook(root, relative, |_| {})? {
        Some(bytes) if digest(&bytes) == expected => {
            unlink_owned_anchored(root, relative, expected, identity)?;
            Ok(())
        }
        Some(_) => Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "cleanup target digest drifted",
        )),
        None => Ok(()),
    }
}

fn validate_recovery_provenance(request: &RecoverInitializedDesignEnvelopeRequest) -> Result<()> {
    let uuid = |value: &str| {
        let parts: Vec<_> = value.split('-').collect();
        parts.iter().map(|part| part.len()).eq([8, 4, 4, 4, 12])
            && value.bytes().all(|b| b == b'-' || b.is_ascii_hexdigit())
    };
    if !uuid(&request.reviewer_session_uuid)
        || !uuid(&request.reviewer_turn_uuid)
        || request.canonical_reviewer != format!("fresh-session:{}", request.reviewer_session_uuid)
        || request.spawned_task.trim().is_empty()
        || request.thread_source != "subagent"
        || request.fork_turns != "none"
        || request.prior_reviewer.trim().is_empty()
        || request.reviewed_digest.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "complete canonical projectless reviewer provenance is required",
        ));
    }
    Ok(())
}

fn validate_safe_authored_destination(value: &str) -> Result<()> {
    let path = Path::new(value);
    if !crate::pvf::clean_relative(path)
        || path
            .components()
            .next()
            .is_some_and(|c| c.as_os_str() == ".git")
        || (value.starts_with(".csdlc/")
            && !value.starts_with(".csdlc/prepared/")
            && !is_issue_local_authored_destination(value))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "authored artifact destination is unsafe",
        ));
    }
    Ok(())
}

fn is_issue_local_authored_destination(value: &str) -> bool {
    let Some(rest) = value.strip_prefix(".csdlc/issues/") else {
        return false;
    };
    let Some((issue, authored_path)) = rest.split_once('/') else {
        return false;
    };
    issue.parse::<u64>().is_ok_and(|issue| issue > 0)
        && authored_path.starts_with("authored/")
        && authored_path.len() > "authored/".len()
}

pub fn approve_design(store: &Store, request: ApproveDesignRequest) -> Result<IssueRecord> {
    approve_design_with_hook(store, request, |_| {})
}

pub fn recover_design_review(
    store: &Store,
    request: RecoverDesignReviewRequest,
) -> Result<IssueRecord> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    if request.issue == 0
        || request.expected_digest.trim().is_empty()
        || request.previous_reviewer.trim().is_empty()
        || request.previous_revision.trim().is_empty()
        || request.false_reviewer.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.disposition.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design review recovery requires exact authority, CAS, actor, reason, and disposition",
        ));
    }
    if request.false_reviewer != request.previous_reviewer {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "false reviewer must equal the current approved reviewer",
        ));
    }

    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "design review recovery generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "design review recovery digest is stale",
        ));
    }
    if record.phase != request.expected_phase
        || !matches!(
            record.phase,
            LifecyclePhase::Bound | LifecyclePhase::Implemented
        )
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design review recovery requires the exact bound or implemented phase",
        ));
    }
    if record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.migration.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design review recovery refuses later lifecycle authority",
        ));
    }
    let (reviewer, revision) = match &record.design_review {
        DesignReview::Approved { reviewer, revision } => (reviewer, revision),
        _ => {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "design review recovery requires a current approved review",
            ))
        }
    };
    if reviewer != &request.previous_reviewer || revision != &request.previous_revision {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design review recovery approval identity or revision does not match",
        ));
    }

    let branch = record.branch.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design review recovery requires a registered branch",
        )
    })?;
    let worktree = record.worktree.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design review recovery requires a registered worktree",
        )
    })?;
    let actual_root = fs::canonicalize(store.root())?;
    let registered_root = fs::canonicalize(worktree).map_err(|error| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("registered worktree is unavailable: {error}"),
        )
    })?;
    if actual_root != registered_root || crate::git::current_branch(store.root())? != branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "design review recovery invocation does not match registered topology",
        ));
    }
    let registered = crate::git::worktrees(store.root())?
        .into_iter()
        .filter(|(candidate_branch, candidate_path)| {
            candidate_branch == branch
                && fs::canonicalize(candidate_path)
                    .is_ok_and(|candidate| candidate == registered_root)
        })
        .count();
    if registered != 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design review recovery topology is missing or ambiguous",
        ));
    }

    let mut cards = store.load_cards(request.issue)?;
    verify_cards(store, &record, &cards)?;
    record.design_review = DesignReview::Pending;
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: serde_json::to_string(&serde_json::json!({
            "reason": request.reason,
            "disposition": request.disposition,
            "previous_approval": {
                "reviewer": request.previous_reviewer,
                "revision": request.previous_revision,
            },
            "false_reviewer": request.false_reviewer,
        }))?,
        operation: "recover_design_review".into(),
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

pub fn recover_initialized_decomposition(
    store: &Store,
    request: InitializedDecompositionRecoveryRequest,
) -> Result<InitializedDecompositionRecoveryResult> {
    let _lock = store.lock(request.issue)?;
    store.recover_if_needed(request.issue)?;
    validate_initialized_recovery_root(store, &request)?;
    validate_initialized_recovery_request_identity(&request)?;
    validate_decomposition_graph(request.issue, &request.graph)?;

    let mut record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "initialized decomposition recovery generation is stale",
        ));
    }
    if record.digest != request.expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "initialized decomposition recovery digest is stale",
        ));
    }
    if record.phase != LifecyclePhase::Initialized {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "initialized decomposition recovery requires initialized phase",
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
            "initialized decomposition recovery requires unbound nonterminal issue state",
        ));
    }

    let mut cards = store.load_cards(request.issue)?;
    verify_cards(store, &record, &cards)?;
    validate_preserved_artifact(
        store,
        &record.design_path,
        &request.preserved_design,
        "design",
    )?;
    validate_preserved_artifact(
        store,
        &record.diagram_path,
        &request.preserved_diagram,
        "diagram",
    )?;

    apply_initialized_recovery_replacements(&mut cards, &request.replacements)?;
    if let Some(review_truth) = &request.design_review_recovery {
        validate_design_review_recovery_truth(&record, review_truth)?;
        record.design_review = review_truth.new_review_state.clone();
    } else {
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
    let audit_operation = serde_json::json!({
        "operation": "recover_initialized_decomposition",
        "recovery_scope": request.recovery_scope,
        "preserved_design": request.preserved_design,
        "preserved_diagram": request.preserved_diagram,
        "graph": request.graph,
        "design_review_recovery": request.design_review_recovery,
    })
    .to_string();
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor,
        reason: request.reason,
        operation: audit_operation,
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    let journal = commit_initialized_recovery_with_journal(
        store,
        request.issue,
        &record,
        &cards,
        request.fail_at,
    )?;
    Ok(InitializedDecompositionRecoveryResult {
        schema: "csdlc.initialized_decomposition_recovery.result.v1".into(),
        issue: request.issue,
        generation: record.generation,
        digest: record.digest,
        journal,
        applied: true,
    })
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
    if !canonical_fresh_session(&request.reviewer) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design reviewer must be a canonical fresh-session UUID",
        ));
    }
    if record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "design approval requires cleared review and publication authority",
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
        revision: design_digest.clone(),
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
        operation: serde_json::json!({
            "operation": "approve_design",
            "design_ref": record.design_path,
            "design_digest": design_digest,
            "diagram_ref": record.diagram_path,
            "diagram_digest": diagram_digest,
        })
        .to_string(),
    });
    hydrate_projections(&mut record, &cards)?;
    record.digest = record_digest(&record)?;
    store.commit(request.issue, &record, &cards, false)?;
    Ok(record)
}

fn canonical_fresh_session(value: &str) -> bool {
    let Some(uuid) = value.strip_prefix("fresh-session:") else {
        return false;
    };
    let bytes = uuid.as_bytes();
    bytes.len() == 36
        && [8, 13, 18, 23]
            .into_iter()
            .all(|index| bytes[index] == b'-')
        && bytes.iter().enumerate().all(|(index, byte)| {
            [8, 13, 18, 23].contains(&index)
                || byte.is_ascii_digit()
                || (b'a'..=b'f').contains(byte)
        })
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
    validate_safe_authored_destination(&request.design_path)?;
    validate_safe_authored_destination(&request.diagram_path)?;
    if request.design_path == request.diagram_path {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram paths must be distinct",
        ));
    }
    Ok(())
}

pub fn edit_issue(store: &Store, request: EditRequest) -> Result<IssueRecord> {
    let _binding_lock = if matches!(
        request.operation,
        SemanticOperation::RefreshAuthoredDesignAfterRecovery
    ) {
        Some(store.binding_lock()?)
    } else {
        None
    };
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
    let implemented_design_refresh = matches!(
        request.operation,
        SemanticOperation::RefreshAuthoredDesignAfterRecovery
    );
    let prebind_contract_repair = is_prebind_contract_repair(&record, &request);
    let prebind_operator_constraints_correction = matches!(
        (record.phase, request.card, &request.operation),
        (
            LifecyclePhase::Initialized | LifecyclePhase::Ready,
            CardKind::Sip,
            SemanticOperation::CorrectOperatorConstraintsBeforeBind { .. }
        )
    );
    if implemented_design_refresh {
        verify_cards_without_authored_tuple(store, &record, &cards)?;
    } else if prebind_contract_repair {
        verify_prebind_contract_repair_inputs(store, &record, &cards)?;
    } else {
        verify_cards(store, &record, &cards)?;
    }
    if prebind_operator_constraints_correction {
        validate_prebind_operator_constraints_correction(&record, &cards, &request)?;
    }
    let mut retained_refresh_artifacts = None;
    let design_refresh = if implemented_design_refresh {
        let (refresh, design_artifact, diagram_artifact) =
            prepare_implemented_design_refresh(store, &record, &cards, &request)?;
        retained_refresh_artifacts = Some((design_artifact, diagram_artifact));
        Some(refresh)
    } else {
        None
    };
    let prior_design_approval = implemented_design_refresh.then(|| record.design_review.clone());
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
            | SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition { .. }
    );
    if identity_update {
        match &request.operation {
            SemanticOperation::UpdateIdentityVersion { .. } => {
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
            }
            SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition {
                title,
                slug,
                live_issue_title,
                live_issue_url,
                live_issue_body_digest,
            } => validate_implemented_identity_title_slug_repair(
                &record,
                title,
                slug,
                live_issue_title,
                live_issue_url,
                live_issue_body_digest,
            )?,
            _ => unreachable!("identity_update match"),
        }
    } else {
        authorize_card_operation(record.phase, request.card, &request.operation)?;
    }
    if prebind_contract_repair {
        validate_prebind_contract_repair(&cards, &request)?;
    }
    let implemented_card_truth_repair =
        is_implemented_card_truth_repair(&record, request.card, &request.operation);
    if implemented_card_truth_repair
        && !implemented_pre_publication_review_recovery_is_clear(&record)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "implemented card truth repair requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
        ));
    }
    if matches!(
        (request.card, &request.operation),
        (
            CardKind::Sor,
            SemanticOperation::AdvanceStatus {
                status: CardStatus::Ready,
            },
        )
    ) && !sor_contains_execution_evidence(&cards)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "implemented SOR status repair requires existing execution evidence",
        ));
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectReviewPromptsAfterRecovery { .. }
    ) && !implemented_pre_publication_review_recovery_is_clear(&record)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "post-recovery review prompt correction requires cleared review and publication truth",
        ));
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
        SemanticOperation::CorrectStpDependenciesAfterRecovery { .. }
            | SemanticOperation::CorrectStpRepoInputsAfterRecovery { .. }
            | SemanticOperation::CorrectPlanStepsAfterRecovery { .. }
    ) {
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "post-recovery card correction requires actor and reason",
            ));
        }
        if !implemented_pre_publication_review_recovery_is_clear(&record) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery card correction requires current typed recorded-review recovery provenance and cleared review, publication, readiness, and terminal truth",
            ));
        }
        let operation_name = match request.operation {
            SemanticOperation::CorrectStpDependenciesAfterRecovery { .. } => {
                "correct_stp_dependencies_after_recovery"
            }
            SemanticOperation::CorrectStpRepoInputsAfterRecovery { .. } => {
                "correct_stp_repo_inputs_after_recovery"
            }
            SemanticOperation::CorrectPlanStepsAfterRecovery { .. } => {
                "correct_plan_steps_after_recovery"
            }
            _ => unreachable!("post-recovery correction operation"),
        };
        if recovery_epoch_already_contains_operation(&record, operation_name) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                format!("{operation_name} is already recorded for this review recovery epoch"),
            ));
        }
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectPlanSummaryAfterRecovery { .. }
            | SemanticOperation::CorrectValidationSummaryAfterRecovery { .. }
            | SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { .. }
            | SemanticOperation::CorrectSorFollowUpsAfterRecovery { .. }
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
                "post-recovery text correction requires actor and reason",
            ));
        }
        let current_recovery = latest_review_operation.is_some_and(|event| {
            event.operation == "recover_review"
                && implemented_pre_publication_review_recovery_is_clear(&record)
                && recovery_follows_recorded_review(&record, event.sequence)
                && record
                    .audit
                    .iter()
                    .filter(|candidate| candidate.sequence > event.sequence)
                    .all(|candidate| recovery_epoch_operation_is_allowed(&candidate.operation))
        });
        let already_repaired = record
            .audit
            .iter()
            .skip_while(|candidate| {
                latest_review_operation.is_some_and(|event| candidate.sequence <= event.sequence)
            })
            .any(|candidate| {
                recovery_epoch_operation_name(&candidate.operation).is_some_and(|operation| {
                    matches!(
                        (&request.operation, operation.as_str()),
                        (
                            SemanticOperation::CorrectPlanSummaryAfterRecovery { .. },
                            "correct_plan_summary_after_recovery"
                        ) | (
                            SemanticOperation::CorrectValidationSummaryAfterRecovery { .. },
                            "correct_validation_summary_after_recovery"
                        ) | (
                            SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { .. },
                            "correct_validation_failure_policy_after_recovery"
                        ) | (
                            SemanticOperation::CorrectSorFollowUpsAfterRecovery { .. },
                            "correct_sor_follow_ups_after_recovery"
                        )
                    )
                })
            });
        if !current_recovery
            || already_repaired
            || record.review_assignment.is_some()
            || record.review.is_some()
            || record.publication.is_some()
            || record.readiness.is_some()
            || record.terminal.is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "post-recovery text correction requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
            ));
        }
    }
    if matches!(
        request.operation,
        SemanticOperation::CorrectGoalAfterRecovery { .. }
            | SemanticOperation::CorrectRequiredOutcomeAfterRecovery { .. }
    ) && !implemented_pre_publication_review_recovery_is_immediate(&record)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "post-recovery text correction requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
        ));
    }
    if matches!(
        request.operation,
        SemanticOperation::ReplaceSorFollowUpsAfterRecovery { .. }
    ) && (!implemented_pre_publication_review_recovery_is_clear(&record)
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some())
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "post-recovery SOR follow-up repair requires current typed recovery provenance and cleared review, publication, readiness, and terminal truth",
        ));
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
    let stp_dependencies_before = if matches!(
        request.operation,
        SemanticOperation::CorrectStpDependenciesAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Stp].content {
            CardContent::Stp(value) => Some(value.dependencies.clone()),
            _ => unreachable!("STP"),
        }
    } else {
        None
    };
    let stp_repo_inputs_before = if matches!(
        request.operation,
        SemanticOperation::CorrectStpRepoInputsAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Stp].content {
            CardContent::Stp(value) => Some(value.repo_inputs.clone()),
            _ => unreachable!("STP"),
        }
    } else {
        None
    };
    let plan_steps_before = if matches!(
        request.operation,
        SemanticOperation::CorrectPlanStepsAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Spp].content {
            CardContent::Spp(value) => Some(value.steps.clone()),
            _ => unreachable!("SPP"),
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
    let validation_summary_before = if matches!(
        request.operation,
        SemanticOperation::CorrectValidationSummaryAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Vpp].content {
            CardContent::Vpp(value) => Some(value.summary.clone()),
            _ => unreachable!("VPP"),
        }
    } else {
        None
    };
    let validation_failure_policy_before = if matches!(
        request.operation,
        SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Vpp].content {
            CardContent::Vpp(value) => Some(value.failure_policy.clone()),
            _ => unreachable!("VPP"),
        }
    } else {
        None
    };
    let sor_follow_ups_before = if matches!(
        request.operation,
        SemanticOperation::CorrectSorFollowUpsAfterRecovery { .. }
            | SemanticOperation::ReplaceSorFollowUpsAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Sor].content {
            CardContent::Sor(value) => Some(value.follow_ups.clone()),
            _ => unreachable!("SOR"),
        }
    } else {
        None
    };
    let required_outcome_before = if matches!(
        request.operation,
        SemanticOperation::CorrectRequiredOutcomeAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Sip].content {
            CardContent::Sip(value) => Some(value.required_outcome.clone()),
            _ => unreachable!("SIP"),
        }
    } else {
        None
    };
    let goal_before = if matches!(
        request.operation,
        SemanticOperation::CorrectGoalAfterRecovery { .. }
    ) {
        match &cards[&CardKind::Sip].content {
            CardContent::Sip(value) => Some(value.goal.clone()),
            _ => unreachable!("SIP"),
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
    let identity_before = if matches!(
        request.operation,
        SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition { .. }
    ) {
        cards
            .values()
            .next()
            .map(|values| (values.identity.title.clone(), values.identity.slug.clone()))
    } else {
        None
    };
    let binding_refresh = if prebind_contract_repair {
        Some(refresh_prebind_design_bindings(store, &record, &mut cards)?)
    } else if let Some(refresh) = design_refresh.as_ref() {
        apply_design_binding_refresh(&mut cards, refresh);
        Some(refresh.clone())
    } else {
        None
    };
    if let (SemanticOperation::CorrectPlanStepsAfterRecovery { steps }, Some(previous_steps)) =
        (&request.operation, plan_steps_before.as_ref())
    {
        validate_status_only_plan_step_recovery(previous_steps, steps)?;
    }
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
                "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
                "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
            })
            .to_string()
        }
        (SemanticOperation::CorrectStpDependenciesAfterRecovery { values }, _) => {
            serde_json::json!({
                "operation": "correct_stp_dependencies_after_recovery",
                "previous_values": stp_dependencies_before
                    .expect("STP dependency correction snapshot"),
                "new_values": values,
                "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
                "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
            })
            .to_string()
        }
        (SemanticOperation::CorrectStpRepoInputsAfterRecovery { values }, _) => {
            serde_json::json!({
                "operation": "correct_stp_repo_inputs_after_recovery",
                "previous_values": stp_repo_inputs_before
                    .expect("STP repo input correction snapshot"),
                "new_values": values,
                "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
                "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
            })
            .to_string()
        }
        (SemanticOperation::CorrectPlanStepsAfterRecovery { steps }, _) => serde_json::json!({
            "operation": "correct_plan_steps_after_recovery",
            "previous_steps": plan_steps_before.expect("SPP step correction snapshot"),
            "new_steps": steps,
            "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
            "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
        })
        .to_string(),
        (SemanticOperation::CorrectPlanSummaryAfterRecovery { value }, _) => serde_json::json!({
            "operation": "correct_plan_summary_after_recovery",
            "previous_value": plan_summary_before.expect("SPP summary correction snapshot"),
            "new_value": value,
            "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
            "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
        })
        .to_string(),
        (SemanticOperation::CorrectValidationSummaryAfterRecovery { value }, _) => serde_json::json!({
            "operation": "correct_validation_summary_after_recovery",
            "previous_value": validation_summary_before.expect("VPP summary correction snapshot"),
            "new_value": value,
            "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
            "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
        })
        .to_string(),
        (SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { value }, _) => serde_json::json!({
            "operation": "correct_validation_failure_policy_after_recovery",
            "previous_value": validation_failure_policy_before.expect("VPP failure policy correction snapshot"),
            "new_value": value,
            "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
            "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
        })
        .to_string(),
        (SemanticOperation::CorrectSorFollowUpsAfterRecovery { values }, _) => serde_json::json!({
            "operation": "correct_sor_follow_ups_after_recovery",
            "previous_values": sor_follow_ups_before.expect("SOR follow-up correction snapshot"),
            "new_values": values,
            "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
            "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
        })
        .to_string(),
        (SemanticOperation::CorrectRequiredOutcomeAfterRecovery { value }, _) => {
            serde_json::json!({
                "operation": "correct_required_outcome_after_recovery",
                "previous_value": required_outcome_before
                    .expect("SIP required-outcome correction snapshot"),
                "new_value": value,
            })
            .to_string()
        }
        (SemanticOperation::CorrectGoalAfterRecovery { value }, _) => {
            serde_json::json!({
                "operation": "correct_goal_after_recovery",
                "previous_value": goal_before
                    .expect("SIP goal correction snapshot"),
                "new_value": value,
                "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
                "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
            })
            .to_string()
        }
        (SemanticOperation::ReplaceSorFollowUpsAfterRecovery { values }, _) => {
            serde_json::json!({
                "operation": "replace_sor_follow_ups_after_recovery",
                "previous_values": sor_follow_ups_before
                    .expect("SOR follow-up correction snapshot"),
                "new_values": values,
                "recovery_sequence": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.sequence),
                "recovery_generation": record.audit.iter().rev().find(|event| event.operation == "recover_review").map(|event| event.generation),
            })
            .to_string()
        }
        (SemanticOperation::CorrectOperatorConstraintsBeforeBind { values }, _) => {
            serde_json::json!({
                "operation": "correct_operator_constraints_before_bind",
                "previous_values": operator_constraints_before
                    .expect("SIP operator-constraint correction snapshot"),
                "new_values": values,
            })
            .to_string()
        }
        (
            SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition {
                title,
                slug,
                live_issue_title,
                live_issue_url,
                live_issue_body_digest,
            },
            _,
        ) => {
            let (previous_title, previous_slug) =
                identity_before.expect("identity correction snapshot");
            serde_json::json!({
                "operation": "correct_identity_title_slug_after_decomposition",
                "previous_title": previous_title,
                "new_title": title,
                "previous_slug": previous_slug,
                "new_slug": slug,
                "live_issue_evidence": {
                    "title": live_issue_title,
                    "url": live_issue_url,
                    "body_digest": live_issue_body_digest,
                },
            })
            .to_string()
        }
        _ if binding_refresh.is_some() => {
            let refresh = binding_refresh.as_ref().expect("pre-bind refresh");
            let recovery = implemented_design_refresh.then(|| {
                record
                    .audit
                    .iter()
                    .rev()
                    .find(|event| event.operation == "recover_review")
                    .map(|event| (event.sequence, event.generation))
                    .expect("implemented design refresh requires recovery provenance")
            });
            let mut operation = serde_json::json!({
                "operation": request.operation,
                "design_binding_refresh": {
                    "design_ref": record.design_path,
                    "old_design_digest": refresh.old_design_digest,
                    "new_design_digest": refresh.new_design_digest,
                    "diagram_ref": record.diagram_path,
                    "old_diagram_digest": refresh.old_diagram_digest,
                    "new_diagram_digest": refresh.new_diagram_digest,
                    "prior_design_approval": prior_design_approval,
                }
            });
            if let Some((sequence, generation)) = recovery {
                let object = operation
                    .as_object_mut()
                    .expect("design binding refresh audit is an object");
                object.insert("recovery_sequence".into(), sequence.into());
                object.insert("recovery_generation".into(), generation.into());
            }
            operation.to_string()
        }
        _ => serde_json::to_string(&request.operation)?,
    };
    if identity_update {
        match &request.operation {
            SemanticOperation::UpdateIdentityVersion { .. } => {
                for values in cards.values_mut() {
                    apply(values, &request.operation)?;
                }
            }
            SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition {
                title, slug, ..
            } => {
                for values in cards.values_mut() {
                    values.identity.title = title.clone();
                    values.identity.slug = slug.clone();
                }
            }
            _ => unreachable!("identity_update apply"),
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
    if prebind_contract_repair
        || prebind_operator_constraints_correction
        || implemented_design_refresh
    {
        record.design_review = DesignReview::Pending;
    }
    if implemented_design_refresh {
        let (design_artifact, diagram_artifact) = retained_refresh_artifacts
            .as_mut()
            .expect("implemented refresh retains paired artifacts");
        let final_design_digest = design_artifact.verify()?;
        let final_diagram_digest = diagram_artifact.verify()?;
        let refresh = design_refresh.as_ref().expect("implemented design refresh");
        if final_design_digest != refresh.new_design_digest
            || final_diagram_digest != refresh.new_diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "authored design tuple changed before canonical commit",
            ));
        }
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
    if let Some((design_artifact, diagram_artifact)) = retained_refresh_artifacts.as_mut() {
        let refresh = design_refresh.as_ref().expect("retained refresh binding");
        if design_artifact.verify()? != refresh.new_design_digest
            || diagram_artifact.verify()? != refresh.new_diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "paired authored artifact handles changed at commit boundary",
            ));
        }
    }
    if let Some((design_artifact, diagram_artifact)) = retained_refresh_artifacts.as_mut() {
        let refresh = design_refresh.as_ref().expect("retained refresh binding");
        let expected_design = refresh.new_design_digest.clone();
        let expected_diagram = refresh.new_diagram_digest.clone();
        let issue_dir = store.issue_dir(request.issue);
        let mut verifier = || {
            if design_artifact.verify_after_projection_swap(store.root(), &issue_dir)?
                != expected_design
                || diagram_artifact.verify_after_projection_swap(store.root(), &issue_dir)?
                    != expected_diagram
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "paired authored artifact handles changed across canonical commit",
                ));
            }
            Ok(())
        };
        store.commit_verified(
            request.issue,
            &record,
            &cards,
            request.fail_after_backup,
            &mut verifier,
        )?;
    } else {
        store.commit(request.issue, &record, &cards, request.fail_after_backup)?;
    }
    Ok(record)
}

#[derive(Debug, Clone)]
struct DesignBindingRefresh {
    old_design_digest: String,
    new_design_digest: String,
    old_diagram_digest: String,
    new_diagram_digest: String,
}

fn apply_design_binding_refresh(
    cards: &mut BTreeMap<CardKind, CardValues>,
    refresh: &DesignBindingRefresh,
) {
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("design-bearing card").content {
            CardContent::Spp(values) => {
                values.design_digest = refresh.new_design_digest.clone();
                values.diagram_digest = refresh.new_diagram_digest.clone();
            }
            CardContent::Vpp(values) => {
                values.design_digest = refresh.new_design_digest.clone();
                values.diagram_digest = refresh.new_diagram_digest.clone();
            }
            _ => unreachable!("design-bearing card"),
        }
    }
}

fn prepare_implemented_design_refresh(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    request: &EditRequest,
) -> Result<(
    DesignBindingRefresh,
    RetainedAuthoredArtifact,
    RetainedAuthoredArtifact,
)> {
    if request.card != CardKind::Spp
        || record.phase != LifecyclePhase::Implemented
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "authored design refresh requires an implemented SPP recovery operation",
        ));
    }
    if !implemented_authored_design_refresh_recovery_is_clear(record) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "authored design refresh requires current review recovery and cleared downstream authority",
        ));
    }
    require_registered_worktree(store, record)?;
    let (old_design_digest, old_diagram_digest) = match &cards[&CardKind::Spp].content {
        CardContent::Spp(values) => (values.design_digest.clone(), values.diagram_digest.clone()),
        _ => unreachable!("SPP"),
    };
    let mut design_artifact =
        retain_authored_artifact(store.root(), Path::new(&record.design_path))?;
    let mut diagram_artifact =
        retain_authored_artifact(store.root(), Path::new(&record.diagram_path))?;
    let new_design_digest = design_artifact.verify()?;
    let new_diagram_digest = diagram_artifact.verify()?;
    if new_design_digest == old_design_digest && new_diagram_digest == old_diagram_digest {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "authored design refresh is a no-op",
        ));
    }
    Ok((
        DesignBindingRefresh {
            old_design_digest,
            new_design_digest,
            old_diagram_digest,
            new_diagram_digest,
        },
        design_artifact,
        diagram_artifact,
    ))
}

fn require_registered_worktree(store: &Store, record: &IssueRecord) -> Result<()> {
    let registered = record.worktree.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored design refresh requires a registered worktree",
        )
    })?;
    let branch = record.branch.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored design refresh requires a registered branch",
        )
    })?;
    let actual = fs::canonicalize(store.root())?;
    let expected = fs::canonicalize(registered)?;
    if actual != expected || crate::git::current_branch(store.root())? != *branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "authored design refresh invocation does not match registered worktree",
        ));
    }
    Ok(())
}

#[cfg(unix)]
struct RetainedAuthoredArtifact {
    root: File,
    relative: PathBuf,
    file: File,
    bytes: Vec<u8>,
    identity: std::fs::Metadata,
}

#[cfg(unix)]
impl RetainedAuthoredArtifact {
    fn verify_retained(&mut self) -> Result<String> {
        use std::os::unix::fs::MetadataExt;
        let current = self.file.metadata()?;
        self.file.seek(SeekFrom::Start(0))?;
        let bytes = read_exact_current_file(&mut self.file, current.len())?;
        if current.nlink() != 1
            || !same_file_identity(&self.identity, &current)
            || current.len() != self.identity.len()
            || current.mtime() != self.identity.mtime()
            || current.mtime_nsec() != self.identity.mtime_nsec()
            || current.ctime() != self.identity.ctime()
            || current.ctime_nsec() != self.identity.ctime_nsec()
            || bytes != self.bytes
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "retained authored artifact changed before commit",
            ));
        }
        Ok(digest(&bytes))
    }

    fn verify_path_identity(&self) -> Result<()> {
        use std::os::unix::fs::MetadataExt;
        let current = self.file.metadata()?;
        let path_file = open_relative_no_follow(&self.root, &self.relative)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "retained authored artifact path disappeared",
            )
        })?;
        let path_meta = path_file.metadata()?;
        if path_meta.nlink() != 1 || !same_file_identity(&current, &path_meta) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "retained authored artifact path identity changed before commit",
            ));
        }
        Ok(())
    }

    fn verify(&mut self) -> Result<String> {
        let retained_digest = self.verify_retained()?;
        self.verify_path_identity()?;
        Ok(retained_digest)
    }

    fn verify_after_projection_swap(&mut self, root: &Path, issue_dir: &Path) -> Result<String> {
        let retained_digest = self.verify_retained()?;
        let canonical = root.join(&self.relative);
        if canonical.strip_prefix(issue_dir).is_ok() {
            let canonical_bytes = read_regular_authored_artifact(root, &self.relative)?
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "copied authored artifact disappeared after projection swap",
                    )
                })?;
            if canonical_bytes != self.bytes {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "copied authored artifact changed across projection swap",
                ));
            }
        } else {
            self.verify_path_identity()?;
        }
        Ok(retained_digest)
    }
}

#[cfg(unix)]
fn retain_authored_artifact(root: &Path, relative: &Path) -> Result<RetainedAuthoredArtifact> {
    validate_authored_relative_path(relative)?;
    let root_file = File::open(root)?;
    let mut file = open_relative_no_follow(&root_file, relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact is absent",
        )
    })?;
    let identity = file.metadata()?;
    use std::os::unix::fs::MetadataExt;
    if !identity.is_file() || identity.nlink() != 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "authored artifact must be a regular single-link file",
        ));
    }
    let bytes = read_exact_current_file(&mut file, identity.len())?;
    Ok(RetainedAuthoredArtifact {
        root: root_file,
        relative: relative.to_path_buf(),
        file,
        bytes,
        identity,
    })
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

fn validate_status_only_plan_step_recovery(
    previous_steps: &[PlanStep],
    replacement_steps: &[PlanStep],
) -> Result<()> {
    let same_non_status_shape = previous_steps.len() == replacement_steps.len()
        && previous_steps
            .iter()
            .zip(replacement_steps)
            .all(|(previous, replacement)| {
                previous.id == replacement.id
                    && previous.action == replacement.action
                    && previous.acceptance_ids == replacement.acceptance_ids
            });
    if !same_non_status_shape {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "SPP recovery plan-step correction may change only step status",
        ));
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
        || sor.integration_state != crate::cards::IntegrationState::NotStarted
        || sor.publication_state != crate::cards::PublicationState::NotPublished
        || sor.merge_state != crate::cards::MergeState::NotMerged
        || sor.closeout_state != crate::cards::CloseoutState::NotStarted
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

fn validate_initialized_recovery_request_identity(
    request: &InitializedDecompositionRecoveryRequest,
) -> Result<()> {
    if request.issue == 0
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || request.recovery_scope.is_empty()
        || request
            .recovery_scope
            .iter()
            .any(|value| value.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "initialized decomposition recovery requires issue, CAS, actor, reason, and scope",
        ));
    }
    Ok(())
}

fn validate_initialized_recovery_root(
    store: &Store,
    request: &InitializedDecompositionRecoveryRequest,
) -> Result<()> {
    let store_root = fs::canonicalize(store.root())?;
    let request_root = fs::canonicalize(&request.request_root)?;
    let cwd = fs::canonicalize(std::env::current_dir()?)?;
    if store_root != request_root || store_root != cwd {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "initialized decomposition recovery requires repo root, request_root, and cwd to match",
        ));
    }
    Ok(())
}

fn validate_preserved_artifact(
    store: &Store,
    expected_path: &str,
    artifact: &PreservedAuthoredArtifact,
    label: &str,
) -> Result<()> {
    if artifact.path != expected_path {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("preserved {label} path does not match issue authority"),
        ));
    }
    let bytes = read_regular_authored_artifact(store.root(), Path::new(&artifact.path))?
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("preserved {label} artifact is absent"),
            )
        })?;
    if sha256_hex(&bytes) != artifact.byte_sha256 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("preserved {label} byte SHA-256 drifted"),
        ));
    }
    if digest(&bytes) != artifact.authored_digest {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("preserved {label} authored digest drifted"),
        ));
    }
    Ok(())
}

fn validate_design_review_recovery_truth(
    record: &IssueRecord,
    review_truth: &DesignReviewRecoveryTruth,
) -> Result<()> {
    if review_truth.false_reviewer.trim().is_empty() || review_truth.disposition.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design review recovery requires false reviewer and disposition",
        ));
    }
    if review_truth.previous_review_state != record.design_review {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "design review recovery previous state does not match issue truth",
        ));
    }
    if !matches!(review_truth.new_review_state, DesignReview::Pending) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "initialized decomposition recovery can only reset design review to pending",
        ));
    }
    Ok(())
}

fn validate_decomposition_graph(issue: u64, graph: &DecompositionGraphInput) -> Result<()> {
    if graph.forbidden_cross_child_trust_redefinition {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "decomposition graph attempts forbidden cross-child trust redefinition",
        ));
    }
    if graph.nodes.is_empty()
        || graph.edges.is_empty()
        || graph.parent_integration_owner.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "decomposition graph requires nodes, edges, and parent integration owner",
        ));
    }
    let mut nodes = std::collections::BTreeMap::new();
    let mut parent_count = 0_u64;
    let mut roles = std::collections::BTreeSet::new();
    for node in &graph.nodes {
        if node.node_id.trim().is_empty()
            || node.issue == 0
            || node.role.trim().is_empty()
            || node.repository.trim().is_empty()
            || !node.in_scope
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph node identity is incomplete or out of scope",
            ));
        }
        if nodes.insert(node.node_id.clone(), node).is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph contains duplicate node id",
            ));
        }
        if !roles.insert(node.role.clone()) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph contains duplicate role",
            ));
        }
        if node.node_id == graph.parent_integration_owner {
            parent_count += 1;
        }
    }
    if parent_count != 1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "decomposition graph must name exactly one parent integration owner",
        ));
    }
    let parent = nodes
        .get(&graph.parent_integration_owner)
        .expect("parent count proves parent node exists");
    if parent.issue != issue {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "decomposition graph parent integration owner does not match the recovered issue",
        ));
    }
    let mut outgoing: std::collections::BTreeMap<&str, Vec<&str>> =
        std::collections::BTreeMap::new();
    let mut incoming: std::collections::BTreeMap<&str, Vec<&str>> =
        std::collections::BTreeMap::new();
    let mut edges = std::collections::BTreeSet::new();
    for edge in &graph.edges {
        if edge.from.trim().is_empty()
            || edge.to.trim().is_empty()
            || edge.relation.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph edge identity is incomplete",
            ));
        }
        if !nodes.contains_key(&edge.from) || !nodes.contains_key(&edge.to) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph edge references a missing node",
            ));
        }
        if edge.from == graph.parent_integration_owner {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph edge orientation is inverted from parent owner",
            ));
        }
        if !edges.insert((&edge.from, &edge.to, &edge.relation)) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "decomposition graph contains duplicate directed edge",
            ));
        }
        outgoing.entry(&edge.from).or_default().push(&edge.to);
        incoming.entry(&edge.to).or_default().push(&edge.from);
    }
    let mut visiting = std::collections::BTreeSet::new();
    let mut visited = std::collections::BTreeSet::new();
    for node in nodes.keys() {
        visit_decomposition_node(node, &outgoing, &mut visiting, &mut visited)?;
    }
    let mut connected_to_parent = std::collections::BTreeSet::new();
    let mut pending = vec![graph.parent_integration_owner.as_str()];
    while let Some(node) = pending.pop() {
        if connected_to_parent.insert(node) {
            pending.extend(incoming.get(node).into_iter().flatten().copied());
        }
    }
    if connected_to_parent.len() != nodes.len() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "every decomposition graph node must have a directed path to the parent integration owner",
        ));
    }
    Ok(())
}

fn visit_decomposition_node<'a>(
    node: &'a str,
    outgoing: &std::collections::BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut std::collections::BTreeSet<&'a str>,
    visited: &mut std::collections::BTreeSet<&'a str>,
) -> Result<()> {
    if visited.contains(node) {
        return Ok(());
    }
    if !visiting.insert(node) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "decomposition graph must be acyclic",
        ));
    }
    if let Some(children) = outgoing.get(node) {
        for child in children {
            visit_decomposition_node(child, outgoing, visiting, visited)?;
        }
    }
    visiting.remove(node);
    visited.insert(node);
    Ok(())
}

fn apply_initialized_recovery_replacements(
    cards: &mut BTreeMap<CardKind, CardValues>,
    replacements: &InitializedDecompositionRecoveryReplacement,
) -> Result<()> {
    validate_recovery_text(&replacements.title, "title")?;
    validate_recovery_text(&replacements.slug, "slug")?;
    validate_identity_version(&replacements.version)?;
    validate_recovery_text(&replacements.goal, "goal")?;
    validate_recovery_text(&replacements.required_outcome, "required outcome")?;
    validate_recovery_vec(&replacements.declared_scope, "declared scope")?;
    validate_recovery_vec(&replacements.authority_boundary, "authority boundary")?;
    validate_recovery_vec(&replacements.operator_constraints, "operator constraints")?;
    validate_recovery_text(&replacements.task_boundary, "task boundary")?;
    validate_recovery_vec(&replacements.deliverables, "deliverables")?;
    validate_recovery_vec(&replacements.acceptance_criteria, "acceptance criteria")?;
    validate_recovery_vec(&replacements.non_goals, "non-goals")?;
    validate_recovery_text(&replacements.plan_summary, "plan summary")?;
    validate_recovery_vec(&replacements.affected_areas, "affected areas")?;
    validate_recovery_vec(&replacements.invariants, "invariants")?;
    validate_recovery_vec(&replacements.risks, "risks")?;
    validate_recovery_vec(&replacements.stop_conditions, "stop conditions")?;
    validate_recovery_text(&replacements.validation_summary, "validation summary")?;
    validate_recovery_vec(&replacements.review_prompts, "review prompts")?;
    validate_recovery_text(&replacements.review_scope, "review scope")?;
    validate_recovery_text(&replacements.sor_summary, "SOR summary")?;
    validate_recovery_lanes(&replacements.validation_lanes)?;
    for result in &replacements.sor_validation {
        validate_result(result)?;
    }
    for values in cards.values_mut() {
        values.identity.title = replacements.title.clone();
        values.identity.slug = replacements.slug.clone();
        values.identity.version = replacements.version.clone();
    }
    match &mut cards.get_mut(&CardKind::Sip).expect("SIP").content {
        CardContent::Sip(values) => {
            values.goal = replacements.goal.clone();
            values.required_outcome = replacements.required_outcome.clone();
            values.declared_scope = replacements.declared_scope.clone();
            values.authority_boundary = replacements.authority_boundary.clone();
            values.initial_assumptions = replacements.initial_assumptions.clone();
            values.operator_constraints = replacements.operator_constraints.clone();
        }
        _ => unreachable!("SIP"),
    }
    match &mut cards.get_mut(&CardKind::Stp).expect("STP").content {
        CardContent::Stp(values) => {
            values.task_boundary = replacements.task_boundary.clone();
            values.deliverables = replacements.deliverables.clone();
            values.acceptance_criteria = replacements.acceptance_criteria.clone();
            values.dependencies = replacements.dependencies.clone();
            values.repo_inputs = replacements.repo_inputs.clone();
            values.non_goals = replacements.non_goals.clone();
        }
        _ => unreachable!("STP"),
    }
    match &mut cards.get_mut(&CardKind::Spp).expect("SPP").content {
        CardContent::Spp(values) => {
            values.plan_revision += 1;
            values.summary = replacements.plan_summary.clone();
            values.steps = replacements.plan_steps.clone();
            values.affected_areas = replacements.affected_areas.clone();
            values.invariants = replacements.invariants.clone();
            values.risks = replacements.risks.clone();
            values.stop_conditions = replacements.stop_conditions.clone();
            values.replan_triggers = replacements.replan_triggers.clone();
        }
        _ => unreachable!("SPP"),
    }
    match &mut cards.get_mut(&CardKind::Vpp).expect("VPP").content {
        CardContent::Vpp(values) => {
            values.summary = replacements.validation_summary.clone();
            values.lanes = replacements.validation_lanes.clone();
            values.failure_policy = replacements.failure_policy.clone();
        }
        _ => unreachable!("VPP"),
    }
    match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
        CardContent::Srp(values) => {
            values.review_scope = replacements.review_scope.clone();
            values.review_revision = None;
            values.reviewer = None;
            values.review_prompts = replacements.review_prompts.clone();
            values.findings.clear();
            values.residual_risk = replacements.residual_risk.clone();
            values.review_result = crate::cards::ReviewResult::PreReview;
        }
        _ => unreachable!("SRP"),
    }
    match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
        CardContent::Sor(values) => {
            values.summary = replacements.sor_summary.clone();
            values.actual_changes.clear();
            values.artifacts = replacements.sor_artifacts.clone();
            values.actual_validation = replacements.sor_validation.clone();
            values.integration_state = crate::cards::IntegrationState::NotStarted;
            values.publication_state = crate::cards::PublicationState::NotPublished;
            values.merge_state = crate::cards::MergeState::NotMerged;
            values.closeout_state = crate::cards::CloseoutState::NotStarted;
            values.follow_ups = replacements.sor_follow_ups.clone();
        }
        _ => unreachable!("SOR"),
    }
    Ok(())
}

fn validate_recovery_text(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            format!("initialized recovery {label} cannot be empty"),
        ));
    }
    Ok(())
}

fn validate_recovery_vec(values: &[String], label: &str) -> Result<()> {
    if values.is_empty() || values.iter().any(|value| value.trim().is_empty()) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            format!("initialized recovery {label} cannot be empty"),
        ));
    }
    Ok(())
}

fn validate_recovery_lanes(lanes: &[crate::cards::ValidationLane]) -> Result<()> {
    if lanes.is_empty() {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "initialized recovery validation lanes cannot be empty",
        ));
    }
    let mut ids = std::collections::BTreeSet::new();
    for lane in lanes {
        if !ids.insert(lane.lane.as_str())
            || lane.lane.trim().is_empty()
            || lane.proof_role.trim().is_empty()
            || lane.acceptance_ids.is_empty()
            || lane.acceptance_ids.iter().any(|id| id.trim().is_empty())
            || lane.argv.is_empty()
            || lane.argv.iter().any(|arg| arg.trim().is_empty())
            || lane.parallel_group.trim().is_empty()
            || lane.budget_seconds == 0
            || lane.budget_tokens == 0
        {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "initialized recovery validation lanes must be unique and complete",
            ));
        }
    }
    Ok(())
}

fn commit_initialized_recovery_with_journal(
    store: &Store,
    issue: u64,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
    fail_at: Option<InitializedRecoveryFailurePoint>,
) -> Result<String> {
    let issue_parent = store.root.join(".csdlc/issues");
    let current = store.issue_dir(issue);
    let output_staging = issue_parent.join(format!(".{issue}.recovery-output"));
    let journal_root = store.initialized_recovery_journal_root(issue);
    let txid = format!(
        "tx-g{}-{}",
        record.generation,
        record.digest.chars().take(12).collect::<String>()
    );
    let transaction = journal_root.join(&txid);
    if output_staging.exists() {
        fs::remove_dir_all(&output_staging)?;
    }
    if transaction.exists() {
        fs::remove_dir_all(&transaction)?;
    }
    fs::create_dir_all(transaction.join("blobs"))?;
    write_complete(&output_staging, record, cards)?;
    let mut targets = Vec::new();
    collect_recovery_targets(
        store.root(),
        &current,
        &output_staging,
        &output_staging,
        &mut targets,
    )?;
    let mut manifest_targets = Vec::new();
    for target in &targets {
        let blob_name = format!("{}.blob", target.postimage_sha256);
        let blob_path = transaction.join("blobs").join(&blob_name);
        let mut blob = File::create(&blob_path)?;
        blob.write_all(&target.postimage)?;
        blob.sync_all()?;
        manifest_targets.push(InitializedRecoveryJournalTarget {
            path: target.relative_path.clone(),
            preimage_sha256: target.preimage_sha256.clone(),
            postimage_sha256: target.postimage_sha256.clone(),
            blob: format!("blobs/{blob_name}"),
            len: target.postimage.len() as u64,
        });
    }
    sync_dir(&transaction.join("blobs"))?;
    let manifest = InitializedRecoveryJournalManifest {
        schema: "csdlc.initialized_recovery_journal.v1".into(),
        issue,
        generation: record.generation,
        digest: record.digest.clone(),
        targets: manifest_targets,
    };
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    let manifest_path = transaction.join("manifest.prepared.json");
    if fail_at == Some(InitializedRecoveryFailurePoint::BeforePreparedManifest) {
        fs::remove_dir_all(&output_staging)?;
        return Err(V2Error::new(
            ErrorCode::InterruptedTransaction,
            "injected interruption before prepared initialized recovery manifest",
        ));
    }
    let mut prepared = File::create(&manifest_path)?;
    prepared.write_all(&manifest_bytes)?;
    prepared.write_all(b"\n")?;
    prepared.sync_all()?;
    sync_dir(&transaction)?;
    if fail_at == Some(InitializedRecoveryFailurePoint::AfterPreparedManifest) {
        fs::remove_dir_all(&output_staging)?;
        return Err(V2Error::new(
            ErrorCode::InterruptedTransaction,
            "injected interruption after prepared initialized recovery manifest",
        ));
    }
    for (index, target) in manifest.targets.iter().enumerate() {
        apply_recovery_target(store.root(), &transaction, target)?;
        if index == 0 && fail_at == Some(InitializedRecoveryFailurePoint::AfterFirstTarget) {
            fs::remove_dir_all(&output_staging)?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected interruption after first initialized recovery target",
            ));
        }
    }
    fs::write(transaction.join("commit.marker"), b"committed\n")?;
    sync_dir(&transaction)?;
    if fail_at == Some(InitializedRecoveryFailurePoint::AfterCommitMarker) {
        fs::remove_dir_all(&output_staging)?;
        return Err(V2Error::new(
            ErrorCode::InterruptedTransaction,
            "injected interruption after initialized recovery commit marker",
        ));
    }
    fs::remove_dir_all(&output_staging)?;
    fs::remove_dir_all(&transaction)?;
    if fs::read_dir(&journal_root)?.next().is_none() {
        fs::remove_dir(&journal_root)?;
    }
    Ok(format!(".csdlc/issues/.{issue}.recovery-journal/{txid}"))
}

#[derive(Debug)]
struct RecoveryTargetBytes {
    relative_path: String,
    preimage_sha256: String,
    postimage_sha256: String,
    postimage: Vec<u8>,
}

fn collect_recovery_targets(
    root: &Path,
    current: &Path,
    staging_base: &Path,
    staging: &Path,
    targets: &mut Vec<RecoveryTargetBytes>,
) -> Result<()> {
    for entry in fs::read_dir(staging)? {
        let entry = entry?;
        let source = entry.path();
        let relative = source.strip_prefix(staging_base).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "recovery staging path escapes staging root",
            )
        })?;
        let target = current.join(relative);
        if entry.file_type()?.is_dir() {
            collect_recovery_targets(root, current, staging_base, &source, targets)?;
            continue;
        }
        let postimage = fs::read(&source)?;
        let preimage = fs::read(&target).unwrap_or_default();
        let target_relative = target.strip_prefix(root).map_err(|_| {
            V2Error::new(
                ErrorCode::UnsafeCheckout,
                "recovery target path escapes repository root",
            )
        })?;
        targets.push(RecoveryTargetBytes {
            relative_path: target_relative.to_string_lossy().into_owned(),
            preimage_sha256: sha256_hex(&preimage),
            postimage_sha256: sha256_hex(&postimage),
            postimage,
        });
    }
    targets.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(())
}

fn roll_forward_initialized_recovery(
    store: &Store,
    transaction: &Path,
    manifest: &InitializedRecoveryJournalManifest,
) -> Result<()> {
    if manifest.schema != "csdlc.initialized_recovery_journal.v1" {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "initialized recovery journal schema is unsupported",
        ));
    }
    for target in &manifest.targets {
        let path = store.root().join(&target.path);
        let current = fs::read(&path).unwrap_or_default();
        let current_hash = sha256_hex(&current);
        if current_hash != target.preimage_sha256 && current_hash != target.postimage_sha256 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "initialized recovery target has unexpected hash",
            ));
        }
    }
    for target in &manifest.targets {
        apply_recovery_target(store.root(), transaction, target)?;
    }
    Ok(())
}

fn apply_recovery_target(
    root: &Path,
    transaction: &Path,
    target: &InitializedRecoveryJournalTarget,
) -> Result<()> {
    if !crate::pvf::clean_relative(Path::new(&target.path))
        || !crate::pvf::clean_relative(Path::new(&target.blob))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "initialized recovery target or blob path is unsafe",
        ));
    }
    let destination = root.join(&target.path);
    let blob_path = transaction.join(&target.blob);
    let bytes = fs::read(&blob_path)?;
    if bytes.len() as u64 != target.len || sha256_hex(&bytes) != target.postimage_sha256 {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "initialized recovery staged blob does not match manifest",
        ));
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
        let tmp = parent.join(format!(
            ".{}.initialized-recovery-tmp",
            destination
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("target")
        ));
        {
            let mut file = File::create(&tmp)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
        }
        fs::rename(&tmp, &destination)?;
        sync_dir(parent)?;
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "initialized recovery target has no parent",
        ))
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut out, "{byte:02x}").expect("format SHA-256");
    }
    out
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
        (CardContent::Vpp(value), crate::cards::TextField::PlanSummary) => {
            Ok(value.summary.clone())
        }
        (CardContent::Vpp(value), crate::cards::TextField::FailurePolicy) => {
            Ok(value.failure_policy.clone())
        }
        (CardContent::Srp(value), crate::cards::TextField::ReviewScope) => {
            Ok(value.review_scope.clone())
        }
        (CardContent::Sor(value), crate::cards::TextField::SorSummary) => Ok(value.summary.clone()),
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

fn verify_cards_without_authored_tuple(
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
            SemanticOperation::RefreshAuthoredDesignAfterRecovery
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::AffectedAreas,
                    ..
                }
                | SemanticOperation::ReplacePlanSteps { .. }
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
                | SemanticOperation::CorrectStpDeliverablesAfterRecovery { .. }
                | SemanticOperation::CorrectStpDependenciesAfterRecovery { .. }
                | SemanticOperation::CorrectStpRepoInputsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Stp,
            SemanticOperation::SetField {
                field: crate::cards::TextField::TaskBoundary,
                ..
            } | SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::NonGoals,
                ..
            },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::CorrectPlanSummaryAfterRecovery { .. }
                | SemanticOperation::CorrectPlanStepsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Vpp,
            SemanticOperation::CorrectValidationSummaryAfterRecovery { .. }
                | SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::CorrectSorFollowUpsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sip,
            SemanticOperation::CorrectGoalAfterRecovery { .. }
                | SemanticOperation::CorrectRequiredOutcomeAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Vpp,
            SemanticOperation::SetField {
                field: crate::cards::TextField::PlanSummary
                    | crate::cards::TextField::FailurePolicy,
                ..
            },
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
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::AdvanceStatus {
                status: CardStatus::Ready,
            } | SemanticOperation::SetField {
                field: crate::cards::TextField::SorSummary,
                ..
            } | SemanticOperation::ReplaceSorFollowUpsAfterRecovery { .. },
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

fn is_implemented_card_truth_repair(
    record: &IssueRecord,
    card: CardKind,
    operation: &SemanticOperation,
) -> bool {
    matches!(
        (record.phase, card, operation),
        (
            LifecyclePhase::Implemented,
            CardKind::Stp,
            SemanticOperation::SetField {
                field: crate::cards::TextField::TaskBoundary,
                ..
            } | SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::NonGoals,
                ..
            } | SemanticOperation::CorrectStpDependenciesAfterRecovery { .. }
                | SemanticOperation::CorrectStpRepoInputsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::CorrectPlanSummaryAfterRecovery { .. }
                | SemanticOperation::CorrectPlanStepsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sip,
            SemanticOperation::CorrectGoalAfterRecovery { .. }
                | SemanticOperation::CorrectRequiredOutcomeAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Vpp,
            SemanticOperation::SetField {
                field: crate::cards::TextField::PlanSummary
                    | crate::cards::TextField::FailurePolicy,
                ..
            } | SemanticOperation::CorrectValidationSummaryAfterRecovery { .. }
                | SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::CorrectReviewPromptsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::SetField {
                field: crate::cards::TextField::SorSummary,
                ..
            } | SemanticOperation::ReplaceSorFollowUpsAfterRecovery { .. }
                | SemanticOperation::CorrectSorFollowUpsAfterRecovery { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sor,
            SemanticOperation::AdvanceStatus {
                status: CardStatus::Ready,
            },
        )
    )
}

fn implemented_pre_publication_review_recovery_is_clear(record: &IssueRecord) -> bool {
    if record.phase != LifecyclePhase::Implemented
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some()
    {
        return false;
    }
    let Some(recovery) = record.audit.iter().rev().find(|event| {
        matches!(
            event.operation.as_str(),
            "assign_review" | "record_review" | "recover_review"
        )
    }) else {
        return false;
    };
    recovery.operation == "recover_review"
        && recovery_follows_recorded_review(record, recovery.sequence)
        && record
            .audit
            .iter()
            .filter(|candidate| candidate.sequence > recovery.sequence)
            .all(|candidate| recovery_epoch_operation_is_allowed(&candidate.operation))
}

fn implemented_authored_design_refresh_recovery_is_clear(record: &IssueRecord) -> bool {
    if record.phase != LifecyclePhase::Implemented
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some()
    {
        return false;
    }
    let Some(recovery) = record.audit.iter().rev().find(|event| {
        matches!(
            event.operation.as_str(),
            "assign_review" | "record_review" | "recover_review"
        )
    }) else {
        return false;
    };
    let after_recovery = || {
        record
            .audit
            .iter()
            .filter(|candidate| candidate.sequence > recovery.sequence)
    };
    let legacy_refresh_only = after_recovery().all(|candidate| {
        authored_design_refresh_epoch_operation_name(&candidate.operation).as_deref()
            == Some("refresh_authored_design_after_recovery")
    });
    let recorded_review_repair_epoch = recovery_follows_recorded_review(record, recovery.sequence)
        && after_recovery().all(|candidate| {
            recovery_epoch_operation_is_allowed(&candidate.operation)
                || authored_design_refresh_epoch_operation_is_allowed(&candidate.operation)
        });
    recovery.operation == "recover_review" && (legacy_refresh_only || recorded_review_repair_epoch)
}

fn authored_design_refresh_epoch_operation_is_allowed(operation: &str) -> bool {
    if operation == "recover_design_review" {
        return true;
    }
    matches!(
        authored_design_refresh_epoch_operation_name(operation),
        Some(ref name)
            if matches!(
                name.as_str(),
                "correct_stp_deliverables_after_recovery"
                    | "refresh_authored_design_after_recovery"
            )
    )
}

fn authored_design_refresh_epoch_operation_name(operation: &str) -> Option<String> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(operation) else {
        return None;
    };
    value.get("operation").and_then(|value| {
        value
            .as_str()
            .or_else(|| value.get("operation").and_then(serde_json::Value::as_str))
            .map(str::to_string)
    })
}

fn validate_implemented_identity_title_slug_repair(
    record: &IssueRecord,
    title: &str,
    slug: &str,
    live_issue_title: &str,
    live_issue_url: &str,
    live_issue_body_digest: &str,
) -> Result<()> {
    if record.phase != LifecyclePhase::Implemented
        || record.review_assignment.is_some()
        || record.review.is_some()
        || record.publication.is_some()
        || record.readiness.is_some()
        || record.terminal.is_some()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "implemented identity title/slug repair requires implemented phase with no review, publication, readiness, or terminal truth",
        ));
    }
    validate_identity_title(title)?;
    validate_identity_slug(slug)?;
    if live_issue_title.trim() != title.trim() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "live issue title evidence does not match requested title",
        ));
    }
    if !live_issue_url.contains(&format!("/issues/{}", record.issue))
        || live_issue_body_digest.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "live issue evidence is incomplete",
        ));
    }
    if !latest_review_audit_is_identity_repair_compatible(record) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "implemented identity title/slug repair requires compatible latest review-related audit state",
        ));
    }
    Ok(())
}

fn validate_identity_title(title: &str) -> Result<()> {
    let trimmed = title.trim();
    if trimmed.is_empty() || trimmed != title || trimmed.len() > 240 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "identity title is malformed",
        ));
    }
    Ok(())
}

fn validate_identity_slug(slug: &str) -> Result<()> {
    let trimmed = slug.trim();
    if trimmed.is_empty()
        || trimmed != slug
        || trimmed.len() > 120
        || slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
        || !slug
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "identity slug is malformed",
        ));
    }
    Ok(())
}

fn latest_review_audit_is_identity_repair_compatible(record: &IssueRecord) -> bool {
    let Some(event) = record
        .audit
        .iter()
        .rev()
        .find(|event| review_related_audit_operation(&event.operation).is_some())
    else {
        return true;
    };
    review_related_audit_operation(&event.operation).is_some_and(|operation| {
        operation == "recover_review"
            && implemented_pre_publication_review_recovery_is_clear(record)
    })
}

fn review_related_audit_operation(operation: &str) -> Option<String> {
    let name = recovery_epoch_operation_name(operation)?;
    match name.as_str() {
        "assign_review" | "record_review" | "recover_review" | "publish" | "record_publication"
        | "finish" | "record_closeout" | "record_merge" => Some(name),
        _ => None,
    }
}

fn implemented_pre_publication_review_recovery_is_immediate(record: &IssueRecord) -> bool {
    implemented_pre_publication_review_recovery_is_clear(record)
        && record
            .audit
            .iter()
            .rev()
            .find(|event| {
                matches!(
                    event.operation.as_str(),
                    "assign_review" | "record_review" | "recover_review"
                )
            })
            .is_some_and(|event| {
                event.operation == "recover_review" && event.generation == record.generation
            })
}

fn recovery_follows_recorded_review(record: &IssueRecord, recovery_sequence: u64) -> bool {
    record
        .audit
        .iter()
        .rev()
        .filter(|event| event.sequence < recovery_sequence)
        .find(|event| {
            matches!(
                event.operation.as_str(),
                "assign_review" | "record_review" | "recover_review"
            )
        })
        .is_some_and(|event| event.operation == "record_review")
}

fn recovery_epoch_already_contains_operation(record: &IssueRecord, operation_name: &str) -> bool {
    let Some(recovery) = record
        .audit
        .iter()
        .rev()
        .find(|event| event.operation == "recover_review")
    else {
        return false;
    };
    record
        .audit
        .iter()
        .filter(|event| event.sequence > recovery.sequence)
        .any(|event| {
            recovery_epoch_operation_name(&event.operation)
                .is_some_and(|candidate| candidate == operation_name)
        })
}

fn sor_contains_execution_evidence(cards: &BTreeMap<CardKind, CardValues>) -> bool {
    match &cards[&CardKind::Sor].content {
        CardContent::Sor(values) => {
            !values.summary.trim().is_empty()
                || !values.actual_changes.is_empty()
                || !values.artifacts.is_empty()
                || !values.actual_validation.is_empty()
        }
        _ => false,
    }
}

fn recovery_epoch_operation_name(operation: &str) -> Option<String> {
    if matches!(operation, "approve_design") {
        return Some("approve_design".into());
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(operation) else {
        return None;
    };
    value
        .get("operation")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
}

fn recovery_epoch_operation_is_allowed(operation: &str) -> bool {
    let Some(name) = recovery_epoch_operation_name(operation) else {
        return false;
    };
    match name.as_str() {
        "approve_design" => true,
        "replace_planning_collection" => {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(operation) else {
                return false;
            };
            matches!(
                value.get("field").and_then(serde_json::Value::as_str),
                Some("affected_areas" | "non_goals")
            )
        }
        "correct_plan_summary_after_recovery"
        | "correct_plan_steps_after_recovery"
        | "correct_stp_dependencies_after_recovery"
        | "correct_stp_repo_inputs_after_recovery"
        | "correct_goal_after_recovery"
        | "correct_required_outcome_after_recovery"
        | "correct_review_prompts_after_recovery"
        | "replace_sor_follow_ups_after_recovery"
        | "correct_validation_summary_after_recovery"
        | "correct_validation_failure_policy_after_recovery"
        | "correct_sor_follow_ups_after_recovery" => true,
        "set_field" => {
            let Ok(value) = serde_json::from_str::<serde_json::Value>(operation) else {
                return false;
            };
            matches!(
                value.get("field").and_then(serde_json::Value::as_str),
                Some("task_boundary" | "plan_summary" | "failure_policy" | "sor_summary")
            )
        }
        "replace_plan_steps" | "replace_validation_lanes" => true,
        _ => false,
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
    if !before.is_file() || before.nlink() != 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "authored artifact target must be a regular single-link file: {}",
                relative.display()
            ),
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
        || final_metadata.nlink() != 1
        || !same_file_identity(&after, &final_metadata)
        || after.len() != final_metadata.len()
        || after.ctime() != final_metadata.ctime()
        || after.ctime_nsec() != final_metadata.ctime_nsec()
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
        || final_after.nlink() != 1
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

fn sync_dirs_through(start: &Path, stop: &Path) -> Result<()> {
    let mut directory = start;
    loop {
        sync_dir(directory)?;
        if directory == stop {
            return Ok(());
        }
        directory = directory.parent().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "durable authored destination escaped the staging directory",
            )
        })?;
    }
}

fn preserve_failed_projection_and_restore(
    current: &Path,
    backup: &Path,
    rollback_preserved: &Path,
) -> Result<()> {
    if rollback_preserved.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "cannot overwrite an existing preserved failed projection",
        ));
    }
    let parent = current.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "issue projection has no parent for rollback",
        )
    })?;
    fs::rename(current, rollback_preserved)?;
    sync_dir(parent)?;
    fs::rename(backup, current)?;
    sync_dir(parent)?;
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
    fn implemented_authored_design_refresh_rejects_preexisting_hardlinks() {
        let temp = tempfile::tempdir().expect("tempdir");
        let design = temp.path().join("design.md");
        fs::write(&design, b"# design\n").expect("design");
        fs::hard_link(&design, temp.path().join("design-alias.md")).expect("hardlink alias");
        let error = read_regular_authored_artifact(temp.path(), Path::new("design.md"))
            .expect_err("pre-existing hardlink must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
        assert!(error.message.contains("single-link"));
    }

    #[cfg(unix)]
    #[test]
    fn implemented_authored_design_refresh_rejects_hardlink_added_before_final_open() {
        let temp = tempfile::tempdir().expect("tempdir");
        let design = temp.path().join("design.md");
        fs::write(&design, b"# design\n").expect("design");
        let error = read_regular_authored_artifact_with_hook(
            temp.path(),
            Path::new("design.md"),
            |stage| {
                if stage == AuthoredReadStage::BeforeFinalOpen {
                    fs::hard_link(&design, temp.path().join("late-alias.md")).expect("late alias");
                }
            },
        )
        .expect_err("late hardlink alias must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[cfg(unix)]
    #[test]
    fn implemented_authored_design_refresh_retains_handle_identity_until_commit_boundary() {
        let temp = tempfile::tempdir().expect("tempdir");
        let design = temp.path().join("design.md");
        fs::write(&design, b"# design\n").expect("design");
        let mut retained = retain_authored_artifact(temp.path(), Path::new("design.md"))
            .expect("retain artifact handle");
        fs::rename(&design, temp.path().join("old-design.md")).expect("move original");
        fs::write(&design, b"# replacement\n").expect("replace path");
        let error = retained
            .verify()
            .expect_err("replacement must fail final identity check");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[cfg(unix)]
    #[test]
    fn implemented_authored_design_refresh_accepts_expected_issue_local_copy_after_swap() {
        let temp = tempfile::tempdir().expect("tempdir");
        let issue_dir = temp.path().join(".csdlc/issues/7");
        let backup = temp.path().join(".csdlc/issues/.7.backup");
        fs::create_dir_all(&issue_dir).expect("issue directory");
        fs::write(issue_dir.join("design.md"), b"# design\n").expect("design");
        let mut retained =
            retain_authored_artifact(temp.path(), Path::new(".csdlc/issues/7/design.md"))
                .expect("retain issue-local artifact");

        fs::rename(&issue_dir, &backup).expect("preserve prior issue projection");
        fs::create_dir_all(&issue_dir).expect("install next issue projection");
        fs::copy(backup.join("design.md"), issue_dir.join("design.md"))
            .expect("copy authored artifact into next projection");

        assert_eq!(
            retained
                .verify_after_projection_swap(temp.path(), &issue_dir)
                .expect("expected issue-local copy remains valid"),
            digest(b"# design\n")
        );
        fs::write(issue_dir.join("design.md"), b"# injected\n").expect("inject drift");
        let error = retained
            .verify_after_projection_swap(temp.path(), &issue_dir)
            .expect_err("post-swap drift must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    #[test]
    fn failed_projection_rollback_preserves_unrelated_post_swap_state() {
        let temp = tempfile::tempdir().expect("tempdir");
        let parent = temp.path().join(".csdlc/issues");
        let current = parent.join("7");
        let backup = parent.join(".7.backup");
        let preserved = parent.join(".7.rollback-preserved");
        fs::create_dir_all(&current).expect("current projection");
        fs::create_dir_all(&backup).expect("backup projection");
        fs::write(current.join("index.json"), b"new").expect("new projection");
        fs::write(current.join("externally-injected.txt"), b"preserve me")
            .expect("external post-swap state");
        fs::write(backup.join("index.json"), b"prior").expect("prior projection");

        preserve_failed_projection_and_restore(&current, &backup, &preserved)
            .expect("non-destructive rollback");

        assert_eq!(fs::read(current.join("index.json")).unwrap(), b"prior");
        assert_eq!(
            fs::read(preserved.join("externally-injected.txt")).unwrap(),
            b"preserve me"
        );
        assert_eq!(fs::read(preserved.join("index.json")).unwrap(), b"new");
        assert!(!backup.exists());
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

    #[test]
    fn implemented_authored_design_refresh_is_exact_phase_and_card_only() {
        let operation = SemanticOperation::RefreshAuthoredDesignAfterRecovery;
        authorize_card_operation(LifecyclePhase::Implemented, CardKind::Spp, &operation)
            .expect("implemented SPP reaches recovery-sensitive guard");
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
            assert_eq!(
                authorize_card_operation(phase, CardKind::Spp, &operation)
                    .expect_err("wrong phase")
                    .code,
                ErrorCode::InvalidTransition
            );
        }
        assert_eq!(
            authorize_card_operation(LifecyclePhase::Implemented, CardKind::Vpp, &operation)
                .expect_err("wrong card")
                .code,
            ErrorCode::InvalidTransition
        );
    }

    #[test]
    fn canonical_fresh_design_reviewer_is_fail_closed() {
        assert!(canonical_fresh_session(
            "fresh-session:c4ee2e17-78fb-4e35-9442-11d2ac0e0478"
        ));
        for invalid in [
            "",
            "fresh-session:pending",
            "fresh-session:C4EE2E17-78FB-4E35-9442-11D2AC0E0478",
            "fresh-session:c4ee2e1778fb4e35944211d2ac0e0478",
            "reviewer:c4ee2e17-78fb-4e35-9442-11d2ac0e0478",
        ] {
            assert!(!canonical_fresh_session(invalid), "{invalid}");
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
