use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::{
    apply, digest, initial_cards, render, terminal_validation_passed, validate_cross_card,
    validate_identity_version, validate_result, CardContent, CardKind, CardValues,
    InitialCardInput, SemanticOperation, StepStatus, ValidationResult,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{
    AuditEvent, CardProjection, Claim, CorruptHistoricalMergedRecoveryRequest,
    CorruptTerminalReceiptReconciliationRequest, DesignReview,
    HistoricalMergedReconciliationRequest, IssueRecord, LifecyclePhase, PublicationEvidence,
    ReadinessEvidence, ReconcileTerminalRequest, RecordlessClosureKind,
    RecordlessTerminalRecoveryRequest, ReviewAssignment, ReviewEvidence,
    TerminalDesignRepairRequest, TerminalDispositionRepairRequest, TerminalEvidence,
    TerminalPlanStepRepairRequest, TerminalReceipt, TerminalReceiptTransportRequest,
    TerminalSorArtifactRepairRequest, TerminalSorValidationRepairRequest, TransitionEvent,
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
    pub claim_id: String,
    pub actor: String,
    pub summary: String,
    pub changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub validation: Vec<ValidationResult>,
}

#[derive(Debug, Clone)]
struct CorruptHistoricalSource {
    authority_worktree: String,
    commit: String,
    expected_projection_digest: String,
    required_checks: Vec<String>,
    require_review: bool,
    expected_target_claim: Claim,
}

struct HistoricalSourceSnapshot {
    record: IssueRecord,
    cards: BTreeMap<CardKind, CardValues>,
    authored_artifacts: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ReviewCommit {
    pub issue: u64,
    pub expected_digest: String,
    pub actor: String,
    pub claim_id: String,
    pub evidence: ReviewEvidence,
    pub result: crate::cards::ReviewResult,
    pub advance_reviewed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TerminalTransactionJournal {
    schema: String,
    issue: u64,
    stage: String,
    #[serde(default)]
    origin_worktree: String,
    #[serde(default)]
    origin_git_common_dir: String,
    original_record_digest: Option<String>,
    #[serde(default)]
    original_projection_digest: Option<String>,
    target_record_digest: String,
    original_receipt: Option<Vec<u8>>,
    #[serde(default)]
    original_projection: Option<TerminalProjectionSnapshot>,
    #[serde(default)]
    original_artifacts: BTreeMap<String, Option<Vec<u8>>>,
    target_receipt: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TerminalProjectionSnapshot {
    record: IssueRecord,
    cards: BTreeMap<CardKind, CardValues>,
    authored_artifacts: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RepairIdentityRequest {
    pub authority_issue: u64,
    pub target_issue: u64,
    pub expected_authority_generation: u64,
    pub expected_authority_digest: String,
    pub expected_target_generation: u64,
    pub expected_target_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub operation: SemanticOperation,
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

    fn terminal_transaction_path(&self, issue: u64) -> Result<PathBuf> {
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        Ok(PathBuf::from(common)
            .join("csdlc-v2/terminal-transactions")
            .join(format!("{issue}.json")))
    }

    fn terminal_transaction_origin(&self) -> Result<(String, String)> {
        let worktree = self.root.canonicalize()?.to_string_lossy().into_owned();
        let common = crate::git::run(
            &self.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout;
        let common = PathBuf::from(common)
            .canonicalize()?
            .to_string_lossy()
            .into_owned();
        Ok((worktree, common))
    }

    fn terminal_repair_lock(&self) -> Result<File> {
        let common = PathBuf::from(
            crate::git::run(
                &self.root,
                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
            )?
            .stdout,
        );
        let relative = PathBuf::from("csdlc-v2/terminal-repairs.lock");
        require_canonical_parent_beneath(&common, &relative)?;
        let dir = common.join("csdlc-v2");
        fs::create_dir_all(&dir)?;
        require_canonical_parent_beneath(&common, &relative)?;
        require_regular_or_absent_beneath(&common, &relative)?;
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(common.join(&relative))?;
        file.lock_exclusive()?;
        require_canonical_parent_beneath(&common, &relative)?;
        require_regular_or_absent_beneath(&common, &relative)?;
        Ok(file)
    }

    fn lock(&self, issue: u64) -> Result<File> {
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

    pub(crate) fn authority_projection_lock(&self, issue: u64) -> Result<File> {
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
        let record: IssueRecord = read_json(&self.issue_dir(issue).join("index.json"))?;
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

    pub fn corrupt_projection_digest(&self, issue: u64) -> Result<String> {
        Ok(projection_snapshot_digest(
            &self.snapshot_issue_projection_bytes(issue)?,
        ))
    }

    fn snapshot_issue_projection_bytes(
        &self,
        issue: u64,
    ) -> Result<BTreeMap<String, Option<Vec<u8>>>> {
        let relative = PathBuf::from(".csdlc/issues").join(issue.to_string());
        let mut snapshot = BTreeMap::new();
        snapshot_regular_tree(&self.root, &relative, &mut snapshot)?;
        Ok(snapshot)
    }

    fn load_historical_source(&self, issue: u64, commit: &str) -> Result<HistoricalSourceSnapshot> {
        let issue_dir = format!(".csdlc/issues/{issue}");
        let index = git_blob(&self.root, commit, &format!("{issue_dir}/index.json"))?;
        let record: IssueRecord = serde_json::from_slice(&index)?;
        if record.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "historical source issue namespace mismatch",
            ));
        }
        verify_record(&record)?;
        let mut expected_index = serde_json::to_vec_pretty(&record)?;
        expected_index.push(b'\n');
        if index != expected_index {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "historical source index is not canonical",
            ));
        }
        let mut expected_audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut expected_audit, event)?;
            expected_audit.push(b'\n');
        }
        if git_blob(&self.root, commit, &format!("{issue_dir}/audit.jsonl"))? != expected_audit {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "historical source audit is not canonical",
            ));
        }
        let mut cards = BTreeMap::new();
        for kind in enum_iterator() {
            let values_path = format!("{issue_dir}/cards/{kind}.values.json");
            let values_bytes = git_blob(&self.root, commit, &values_path)?;
            let values: CardValues = serde_json::from_slice(&values_bytes)?;
            let mut expected_values = serde_json::to_vec_pretty(&values)?;
            expected_values.push(b'\n');
            let rendered = render(&values)?;
            let projection = record.cards.get(&kind).ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("missing {kind} projection"),
                )
            })?;
            if values.kind() != kind
                || values.identity.issue != issue
                || values.identity.repository != record.repository
                || values.identity.generation != record.generation
                || values_bytes != expected_values
                || git_blob(&self.root, commit, &format!("{issue_dir}/cards/{kind}.md"))?
                    != rendered.markdown.as_bytes()
                || projection.values_digest != rendered.values_digest
                || projection.rendered_digest != rendered.rendered_digest
                || projection.ast_digest != rendered.ast_digest
            {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    format!("historical source {kind} projection is invalid"),
                ));
            }
            cards.insert(kind, values);
        }
        let mut authored_artifacts = BTreeMap::new();
        for path in [&record.design_path, &record.diagram_path] {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "historical authored artifact path is unsafe",
                ));
            }
            let bytes = git_blob(&self.root, commit, path)?;
            let contents = String::from_utf8(bytes).map_err(|_| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "historical authored artifact is not UTF-8",
                )
            })?;
            authored_artifacts.insert(path.clone(), contents);
        }
        validate_cross_card(
            &cards,
            &record.design_path,
            &digest(authored_artifacts[&record.design_path].as_bytes()),
            &record.diagram_path,
            &digest(authored_artifacts[&record.diagram_path].as_bytes()),
        )?;
        Ok(HistoricalSourceSnapshot {
            record,
            cards,
            authored_artifacts,
        })
    }

    pub fn repair_identity(&self, request: RepairIdentityRequest) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "identity repair requires a distinct authority issue",
            ));
        }
        let version = match &request.operation {
            SemanticOperation::UpdateIdentityVersion { version } => version,
            _ => {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "identity repair requires update_identity_version",
                ))
            }
        };
        validate_identity_version(version)?;
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "authority issue digest is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "target issue digest is stale",
            ));
        }
        let authority_cards = self.load_cards(request.authority_issue)?;
        verify_cards(self, &authority, &authority_cards)?;
        verify_canonical_projection_bytes(self, &authority, &authority_cards)?;
        let mut target_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &target_cards)?;
        verify_canonical_projection_bytes(self, &target, &target_cards)?;
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.claim_id, now_seconds()?)?;
        let original_target = target.clone();
        let original_target_cards = target_cards.clone();
        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt = self.read_terminal_receipt_snapshot(&receipt_path)?;
        for values in target_cards.values_mut() {
            apply(values, &request.operation)?;
        }
        if target_cards
            .values()
            .any(|values| values.identity.version != *version)
        {
            return Err(V2Error::new(
                ErrorCode::CardInvalid,
                "identity repair did not update all cards",
            ));
        }
        let target_design_digest = digest(
            &read_regular_terminal_artifact(&self.root, Path::new(&target.design_path))?
                .ok_or_else(|| {
                    V2Error::new(ErrorCode::ReconciliationRequired, "target design is absent")
                })?,
        );
        let target_diagram_digest = digest(
            &read_regular_terminal_artifact(&self.root, Path::new(&target.diagram_path))?
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "target diagram is absent",
                    )
                })?,
        );
        validate_cross_card(
            &target_cards,
            &target.design_path,
            &target_design_digest,
            &target.diagram_path,
            &target_diagram_digest,
        )?;
        target.generation += 1;
        for values in target_cards.values_mut() {
            values.identity.generation = target.generation;
        }
        if let Some(claim) = target.claim.as_mut() {
            claim.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed identity repair authorized by issue {}",
                request.authority_issue
            ),
            operation: serde_json::to_string(&request.operation)?,
        });
        hydrate_projections(&mut target, &target_cards)?;
        target.digest = record_digest(&target)?;
        self.commit(request.target_issue, &target, &target_cards, false)?;
        if let Err(error) = self.refresh_terminal_receipt(&target, &target_cards) {
            self.commit(
                request.target_issue,
                &original_target,
                &original_target_cards,
                false,
            )?;
            if let Some(bytes) = original_receipt {
                self.restore_terminal_receipt(&receipt_path, &bytes)?;
            }
            return Err(error);
        }
        Ok(target)
    }

    pub fn repair_terminal_design(
        &self,
        request: TerminalDesignRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.reviewer.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_design_digest.trim().is_empty()
            || request.expected_diagram_digest.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal design repair identity or authority is incomplete",
            ));
        }
        for path in [&request.source_design_path, &request.source_diagram_path] {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "terminal design repair source path must be repository-relative",
                ));
            }
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal design repair requires a closed-out target without a claim",
            ));
        }
        let authority_claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing")
        })?;
        authority_claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(authority_claim, request.target_issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal design repair authority does not cover target issue",
            ));
        }
        let target_issue_root = PathBuf::from(format!(".csdlc/issues/{}", request.target_issue));
        for path in [&target.design_path, &target.diagram_path] {
            if Path::new(path).starts_with(&target_issue_root) {
                continue;
            }
            if !authority_claim.protected_paths.iter().any(|protected| {
                Path::new(path) == Path::new(protected)
                    || Path::new(path).starts_with(Path::new(protected))
            }) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    format!(
                        "terminal design repair authority does not cover external artifact {path}"
                    ),
                ));
            }
        }
        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        let design =
            read_regular_terminal_artifact(&self.root, Path::new(&request.source_design_path))?
                .ok_or_else(|| {
                    V2Error::new(ErrorCode::ReconciliationRequired, "repair design is absent")
                })?;
        let diagram =
            read_regular_terminal_artifact(&self.root, Path::new(&request.source_diagram_path))?
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "repair diagram is absent",
                    )
                })?;
        let design_digest = digest(&design);
        let diagram_digest = digest(&diagram);
        if design_digest != request.expected_design_digest
            || diagram_digest != request.expected_diagram_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair artifact hash does not match request",
            ));
        }
        let design_text = String::from_utf8(design).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "repair design must be UTF-8 Markdown",
            )
        })?;
        let diagram_text = String::from_utf8(diagram).map_err(|_| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "repair diagram must be UTF-8 Mermaid",
            )
        })?;
        if design_text.trim().is_empty() || diagram_text.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair artifacts must not be empty",
            ));
        }
        if !valid_mermaid_diagram(&diagram_text) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair diagram is not recognized Mermaid source",
            ));
        }
        to_mdast(&design_text, &ParseOptions::gfm()).map_err(|error| {
            V2Error::new(
                ErrorCode::InvalidInput,
                format!("repair design Markdown failed AST validation: {error}"),
            )
        })?;
        let mut cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &cards)?;
        for kind in [CardKind::Spp, CardKind::Vpp] {
            match &mut cards.get_mut(&kind).expect("design-bearing card").content {
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
        target.design_review = DesignReview::Approved {
            reviewer: request.reviewer.clone(),
            revision: design_digest.clone(),
        };
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed terminal design repair authorized by issue {}",
                request.authority_issue
            ),
            operation: "repair_terminal_design".into(),
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;
        let authored_artifacts = BTreeMap::from([
            (target.design_path.clone(), design_text),
            (target.diagram_path.clone(), diagram_text),
        ]);
        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.authored_artifacts = authored_artifacts.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: request.target_issue,
            stage: "prepared_terminal_design_repair".into(),
            original_record_digest: Some(original_receipt.record.digest.clone()),
            original_projection_digest: None,
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes),
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit_with_authored(
            request.target_issue,
            &target,
            &cards,
            false,
            Some(&authored_artifacts),
        ) {
            let _ = self.recover_terminal_transaction(request.target_issue);
            return Err(error);
        }
        journal.stage = "projection_committed_terminal_design_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            let _ = self.recover_terminal_transaction(request.target_issue);
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        journal.stage = "receipt_committed_terminal_design_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn repair_terminal_plan_step(
        &self,
        request: TerminalPlanStepRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.step_id.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal plan repair identity or authority is incomplete",
            ));
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal plan repair requires a closed-out target without a claim",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.authority_claim_id, now_seconds()?)?;
        if !authority
            .claim
            .as_ref()
            .is_some_and(|claim| claim_covers_issue(claim, request.target_issue))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        complete_terminal_plan_step(&mut cards, &request.step_id)?;

        let original_target = target.clone();
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor.clone(),
            reason: format!(
                "typed terminal plan repair authorized by issue {}",
                request.authority_issue
            ),
            operation: format!("repair_terminal_plan_step:{}", request.step_id),
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;

        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: request.target_issue,
            stage: "prepared_terminal_plan_repair".into(),
            original_record_digest: Some(original_receipt.record.digest.clone()),
            original_projection_digest: None,
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes.clone()),
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            self.remove_terminal_transaction_journal(request.target_issue)?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit(request.target_issue, &target, &cards, false) {
            let _ = self.remove_terminal_transaction_journal(request.target_issue);
            return Err(error);
        }
        journal.stage = "projection_committed_terminal_plan_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            self.rollback_terminal_repair(
                request.target_issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))
        {
            self.rollback_terminal_repair(
                request.target_issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(error);
        }
        journal.stage = "receipt_committed_terminal_plan_repair".into();
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn repair_terminal_sor_artifact(
        &self,
        request: TerminalSorArtifactRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_artifact_digest.trim().is_empty()
            || request.stale_ref == request.retained_ref
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal SOR artifact repair identity or authority is incomplete",
            ));
        }
        for path in [&request.stale_ref, &request.retained_ref] {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "terminal SOR artifact repair paths must be repository-relative",
                ));
            }
        }

        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal SOR artifact repair requires a closed-out target without a claim",
            ));
        }
        authority
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing"))?
            .validate(&request.authority_claim_id, now_seconds()?)?;
        if !authority
            .claim
            .as_ref()
            .is_some_and(|claim| claim_covers_issue(claim, request.target_issue))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }
        if request.retained_ref != target.design_path && request.retained_ref != target.diagram_path
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "replacement is not a canonical retained authored artifact",
            ));
        }
        let retained_bytes = original_receipt
            .authored_artifacts
            .get(&request.retained_ref)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::InvalidInput,
                    "replacement artifact is absent from the terminal receipt",
                )
            })?;
        if digest(retained_bytes.as_bytes()) != request.expected_artifact_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "replacement artifact bytes differ from the request",
            ));
        }

        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        replace_terminal_sor_artifact(&mut cards, &request.stale_ref, &request.retained_ref)?;

        self.commit_terminal_card_repair(
            target,
            cards,
            original_cards,
            original_receipt,
            original_receipt_bytes,
            receipt_path,
            &request.actor,
            format!(
                "typed terminal SOR artifact repair authorized by issue {}",
                request.authority_issue
            ),
            format!(
                "repair_terminal_sor_artifact:{}->{}",
                request.stale_ref, request.retained_ref
            ),
            "terminal_sor_artifact_repair",
            request.fail_after_stage.as_deref(),
        )
    }

    pub fn repair_terminal_sor_validation(
        &self,
        request: TerminalSorValidationRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.expected_authority_digest.trim().is_empty()
            || request.expected_target_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
            || request.expected_result == request.replacement_result
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal SOR validation repair identity or authority is incomplete",
            ));
        }
        validate_result(&request.replacement_result)?;
        validate_portable_validation_result(&request.replacement_result)?;

        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let target = self.load_record(request.target_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair authority record is stale",
            ));
        }
        if target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "repair target record is stale",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut || target.claim.is_some() {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal SOR validation repair requires a closed-out target without a claim",
            ));
        }
        let authority_claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::MissingClaim, "repair authority claim missing")
        })?;
        authority_claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(authority_claim, request.target_issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "repair authority claim does not cover the target issue",
            ));
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt_bytes = fs::read(&receipt_path)?;
        let original_receipt: TerminalReceipt = serde_json::from_slice(&original_receipt_bytes)?;
        validate_terminal_receipt(&original_receipt)?;
        if original_receipt.digest != request.expected_receipt_digest
            || original_receipt.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt is stale",
            ));
        }

        let original_cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &original_cards)?;
        let mut cards = original_cards.clone();
        replace_terminal_sor_validation(
            &mut cards,
            &request.expected_result,
            &request.replacement_result,
        )?;

        self.commit_terminal_card_repair(
            target,
            cards,
            original_cards,
            original_receipt,
            original_receipt_bytes,
            receipt_path,
            &request.actor,
            format!(
                "typed terminal SOR validation repair authorized by issue {}",
                request.authority_issue
            ),
            "repair_terminal_sor_validation".into(),
            "terminal_sor_validation_repair",
            request.fail_after_stage.as_deref(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn commit_terminal_card_repair(
        &self,
        mut target: IssueRecord,
        mut cards: BTreeMap<CardKind, CardValues>,
        original_cards: BTreeMap<CardKind, CardValues>,
        original_receipt: TerminalReceipt,
        original_receipt_bytes: Vec<u8>,
        receipt_path: PathBuf,
        actor: &str,
        reason: String,
        operation: String,
        stage_suffix: &str,
        fail_after_stage: Option<&str>,
    ) -> Result<IssueRecord> {
        let original_target = target.clone();
        target.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: actor.into(),
            reason,
            operation,
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;

        let mut repaired_receipt = original_receipt.clone();
        repaired_receipt.record = target.clone();
        repaired_receipt.cards = cards.clone();
        repaired_receipt.digest.clear();
        repaired_receipt.digest = terminal_receipt_digest(&repaired_receipt)?;
        validate_terminal_receipt(&repaired_receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: target.issue,
            stage: format!("prepared_{stage_suffix}"),
            original_record_digest: Some(original_receipt.record.digest),
            original_projection_digest: None,
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_receipt_bytes.clone()),
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt: serde_json::to_vec_pretty(&repaired_receipt)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if fail_after_stage == Some("after_journal") {
            self.remove_terminal_transaction_journal(target.issue)?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.commit(target.issue, &target, &cards, false) {
            let _ = self.remove_terminal_transaction_journal(target.issue);
            return Err(error);
        }
        journal.stage = format!("projection_committed_{stage_suffix}");
        self.write_terminal_transaction_journal(&journal)?;
        if fail_after_stage == Some("after_projection") {
            self.rollback_terminal_repair(
                target.issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected repair failure",
            ));
        }
        if let Err(error) = self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))
        {
            self.rollback_terminal_repair(
                target.issue,
                &original_target,
                &original_cards,
                &receipt_path,
                &original_receipt_bytes,
            )?;
            return Err(error);
        }
        journal.stage = format!("receipt_committed_{stage_suffix}");
        self.write_terminal_transaction_journal(&journal)?;
        self.remove_terminal_transaction_journal(target.issue)?;
        Ok(target)
    }

    fn rollback_terminal_repair(
        &self,
        issue: u64,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
        receipt_path: &Path,
        receipt: &[u8],
    ) -> Result<()> {
        self.commit(issue, record, cards, false)?;
        self.replace_receipt_bytes(receipt_path, Some(receipt))?;
        self.remove_terminal_transaction_journal(issue)
    }

    fn refresh_terminal_receipt(
        &self,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<()> {
        let path = self.terminal_receipt_path(record.issue)?;
        let Some(mut receipt) = self.load_terminal_receipt(record.issue)? else {
            return Ok(());
        };
        if receipt.issue != record.issue
            || receipt.repository != record.repository
            || receipt.initialization_digest != record.initialization_digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt identity differs from repair target",
            ));
        }
        receipt.record = record.clone();
        receipt.cards = cards.clone();
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        self.replace_receipt_bytes(&path, Some(&serde_json::to_vec_pretty(&receipt)?))
    }

    fn write_terminal_transaction_journal(
        &self,
        journal: &TerminalTransactionJournal,
    ) -> Result<()> {
        let mut journal = journal.clone();
        let (origin_worktree, origin_git_common_dir) = self.terminal_transaction_origin()?;
        journal.origin_worktree = origin_worktree;
        journal.origin_git_common_dir = origin_git_common_dir;
        let path = self.terminal_transaction_path(journal.issue)?;
        let (common, relative) = self.git_common_relative(&path)?;
        let mut bytes = serde_json::to_vec_pretty(&journal)?;
        bytes.push(b'\n');
        replace_regular_terminal_artifact(&common, &relative, &bytes, "json.transaction-tmp")
    }

    fn remove_terminal_transaction_journal(&self, issue: u64) -> Result<()> {
        let path = self.terminal_transaction_path(issue)?;
        let (common, relative) = self.git_common_relative(&path)?;
        require_canonical_parent_beneath(&common, &relative)?;
        if canonical_path_metadata_beneath(&common, &relative)?.is_some() {
            require_regular_or_absent_beneath(&common, &relative)?;
            fs::remove_file(&path)?;
            sync_dir(path.parent().expect("transaction parent"))?;
        }
        Ok(())
    }

    fn replace_receipt_bytes(&self, path: &Path, bytes: Option<&[u8]>) -> Result<()> {
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
        let parent = path
            .parent()
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "receipt has no parent"))?;
        require_canonical_parent_beneath(&common, relative)?;
        fs::create_dir_all(parent)?;
        require_canonical_parent_beneath(&common, relative)?;
        require_regular_or_absent_beneath(&common, relative)?;
        let lock_relative = relative.with_file_name("receipts.lock");
        require_canonical_parent_beneath(&common, &lock_relative)?;
        require_regular_or_absent_beneath(&common, &lock_relative)?;
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(common.join(&lock_relative))?;
        lock.lock_exclusive()?;
        require_canonical_parent_beneath(&common, relative)?;
        require_regular_or_absent_beneath(&common, relative)?;
        require_regular_or_absent_beneath(&common, &lock_relative)?;
        for suffix in [
            "json.reconcile-tmp",
            "json.recovery-tmp",
            "json.repair-tmp",
            "json.restore-tmp",
        ] {
            let temporary_relative = relative.with_extension(suffix);
            if canonical_path_metadata_beneath(&common, &temporary_relative)?.is_some() {
                require_regular_or_absent_beneath(&common, &temporary_relative)?;
                fs::remove_file(common.join(&temporary_relative))?;
            }
        }
        match bytes {
            Some(bytes) => {
                replace_regular_terminal_artifact(&common, relative, bytes, "json.recovery-tmp")?;
            }
            None if canonical_path_metadata_beneath(&common, relative)?.is_some() => {
                require_canonical_parent_beneath(&common, relative)?;
                require_regular_or_absent_beneath(&common, relative)?;
                fs::remove_file(common.join(relative))?;
                sync_dir(parent)?;
            }
            None => {}
        }
        Ok(())
    }

    fn restore_artifact_snapshots(
        &self,
        snapshots: &BTreeMap<String, Option<Vec<u8>>>,
    ) -> Result<()> {
        for (relative, original) in snapshots {
            if !crate::pvf::clean_relative(Path::new(relative)) {
                return Err(V2Error::new(
                    ErrorCode::CorruptRecord,
                    "terminal transaction artifact snapshot path is unsafe",
                ));
            }
            let relative = Path::new(relative);
            let path = self.root.join(relative);
            match original {
                Some(bytes) => {
                    replace_regular_terminal_artifact(
                        &self.root,
                        relative,
                        bytes,
                        "terminal-restore-tmp",
                    )?;
                }
                None => match canonical_path_metadata_beneath(&self.root, relative)? {
                    Some(metadata) if metadata.is_file() => {
                        fs::remove_file(&path)?;
                        if let Some(parent) = path.parent() {
                            sync_dir(parent)?;
                        }
                    }
                    Some(_) => {
                        return Err(V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "terminal rollback cannot remove a non-file authored path",
                        ));
                    }
                    None => {}
                },
            }
        }
        Ok(())
    }

    fn restore_terminal_transaction_original(
        &self,
        journal: &TerminalTransactionJournal,
        receipt_path: &Path,
    ) -> Result<()> {
        let receipt_projection = journal
            .original_receipt
            .as_deref()
            .map(|bytes| -> Result<TerminalProjectionSnapshot> {
                let receipt: TerminalReceipt = serde_json::from_slice(bytes)?;
                validate_terminal_receipt(&receipt)?;
                Ok(TerminalProjectionSnapshot {
                    record: receipt.record,
                    cards: receipt.cards,
                    authored_artifacts: receipt.authored_artifacts,
                })
            })
            .transpose()?;
        let projection = journal
            .original_projection
            .as_ref()
            .or(receipt_projection.as_ref());
        if let Some(projection) = projection {
            self.commit_with_authored(
                journal.issue,
                &projection.record,
                &projection.cards,
                false,
                Some(&projection.authored_artifacts),
            )?;
        } else if self.issue_dir(journal.issue).exists() {
            fs::remove_dir_all(self.issue_dir(journal.issue))?;
            sync_dir(
                self.issue_dir(journal.issue)
                    .parent()
                    .expect("issue parent"),
            )?;
        }
        self.restore_artifact_snapshots(&journal.original_artifacts)?;
        self.replace_receipt_bytes(receipt_path, journal.original_receipt.as_deref())
    }

    fn recover_terminal_transaction(&self, issue: u64) -> Result<()> {
        let path = self.terminal_transaction_path(issue)?;
        let (common, relative) = self.git_common_relative(&path)?;
        require_canonical_parent_beneath(&common, &relative)?;
        let Some(metadata) = canonical_path_metadata_beneath(&common, &relative)? else {
            return Ok(());
        };
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal transaction journal is not a canonical regular file",
            ));
        }
        let journal: TerminalTransactionJournal = read_json(&path)?;
        if journal.schema != "csdlc.terminal_transaction.v1" || journal.issue != issue {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal transaction journal identity is invalid",
            ));
        }
        let (origin_worktree, origin_git_common_dir) = self.terminal_transaction_origin()?;
        if journal.origin_worktree != origin_worktree
            || journal.origin_git_common_dir != origin_git_common_dir
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal transaction recovery must run in its originating worktree",
            ));
        }
        let receipt_path = self.terminal_receipt_path(issue)?;
        let target: TerminalReceipt = serde_json::from_slice(&journal.target_receipt)?;
        validate_terminal_receipt(&target)?;
        if target.issue != issue || target.record.digest != journal.target_record_digest {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "terminal transaction target receipt differs from journal identity",
            ));
        }
        let current = self.load_record(issue);
        let current_projection_digest = journal
            .original_projection_digest
            .as_ref()
            .map(|_| {
                self.snapshot_issue_projection_bytes(issue)
                    .map(|snapshot| projection_snapshot_digest(&snapshot))
            })
            .transpose()?;
        if current
            .as_ref()
            .is_ok_and(|record| record.digest == journal.target_record_digest)
            && self.verify_materialized_terminal_receipt(&target).is_ok()
        {
            self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        } else if current_projection_digest.as_ref() == journal.original_projection_digest.as_ref()
            && journal.original_projection_digest.is_some()
        {
            self.restore_terminal_transaction_original(&journal, &receipt_path)?;
        } else if current.as_ref().is_ok_and(|record| {
            record.digest != journal.target_record_digest
                && journal.original_record_digest.as_deref() != Some(record.digest.as_str())
        }) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal transaction journal does not match current projection",
            ));
        } else {
            self.restore_terminal_transaction_original(&journal, &receipt_path)?;
        }
        self.remove_terminal_transaction_journal(issue)
    }

    fn maybe_interrupt_terminal_transaction(issue: u64, stage: &str) -> Result<()> {
        let issue_matches = std::env::var("CSDLC_V2_TEST_INTERRUPT_ISSUE")
            .ok()
            .is_some_and(|value| value == issue.to_string());
        let stage_matches = std::env::var("CSDLC_V2_TEST_INTERRUPT_STAGE")
            .ok()
            .is_some_and(|value| value == stage);
        if issue_matches && stage_matches {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                format!("injected terminal interruption at {stage}"),
            ));
        }
        Ok(())
    }

    fn restore_terminal_receipt(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        if self
            .read_terminal_receipt_snapshot(path)?
            .is_some_and(|current| current != bytes)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt changed while rollback was in progress",
            ));
        }
        self.replace_receipt_bytes(path, Some(bytes))
    }

    fn read_terminal_receipt_snapshot(&self, path: &Path) -> Result<Option<Vec<u8>>> {
        let (common, relative) = self.git_common_relative(path)?;
        let Some(metadata) = canonical_path_metadata_beneath(&common, &relative)? else {
            return Ok(None);
        };
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal receipt snapshot is not a regular file: {}",
                    path.display()
                ),
            ));
        }
        Ok(Some(fs::read(path)?))
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

    pub fn verify_terminal_authority(&self, issue: u64) -> Result<()> {
        let receipt = self.load_terminal_receipt(issue)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt is absent",
            )
        })?;
        self.verify_materialized_terminal_receipt(&receipt)
    }

    pub(crate) fn has_claim_free_terminal_authority(
        &self,
        issue: u64,
        repository: &str,
        initialization_digest: &str,
    ) -> Result<bool> {
        let local = self.load_record(issue)?;
        if local.phase != LifecyclePhase::ClosedOut
            || local.claim.is_some()
            || local.repository != repository
            || local.initialization_digest != initialization_digest
        {
            return Ok(false);
        }
        let Some(receipt) = self.load_terminal_receipt(issue)? else {
            return Ok(false);
        };
        if receipt.record != local {
            return Ok(false);
        }
        self.verify_materialized_terminal_receipt(&receipt)?;
        Ok(true)
    }

    pub(crate) fn has_claim_free_retained_terminal_authority(
        &self,
        observed: &IssueRecord,
    ) -> Result<bool> {
        let Some(observed_claim) = observed.claim.as_ref() else {
            return Ok(false);
        };
        let local = self.load_record(observed.issue)?;
        if local.claim.is_some()
            || local.repository != observed.repository
            || local.initialization_digest != observed.initialization_digest
        {
            return Ok(false);
        }
        let Some(receipt) = self.load_terminal_receipt(observed.issue)? else {
            return Ok(false);
        };
        let Some(terminal) = receipt.record.terminal.as_ref() else {
            return Ok(false);
        };
        let mut released_paths = terminal.released_protected_paths.clone();
        let mut observed_paths = observed_claim.protected_paths.clone();
        released_paths.sort();
        observed_paths.sort();
        Ok(receipt.issue == observed.issue
            && receipt.repository == observed.repository
            && receipt.initialization_digest == observed.initialization_digest
            && receipt.record.issue == observed.issue
            && receipt.record.repository == observed.repository
            && receipt.record.initialization_digest == observed.initialization_digest
            && receipt.record.phase == LifecyclePhase::ClosedOut
            && receipt.record.claim.is_none()
            && receipt.record.generation > observed.generation
            && terminal.released_branch == observed_claim.branch
            && terminal.released_worktree == observed_claim.worktree
            && released_paths == observed_paths)
    }

    fn verify_materialized_terminal_receipt(&self, receipt: &TerminalReceipt) -> Result<()> {
        verify_canonical_projection_bytes(self, &receipt.record, &receipt.cards).map_err(
            |error| {
                if error.code == ErrorCode::UnsafeCheckout {
                    error
                } else {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "materialized terminal projection is not canonical regular-file state",
                    )
                }
            },
        )?;
        for (path, expected) in &receipt.authored_artifacts {
            let actual =
                read_regular_terminal_artifact(&self.root, Path::new(path))?.ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "materialized terminal authored artifact is absent",
                    )
                })?;
            if actual != expected.as_bytes() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "materialized terminal authored artifact differs from retained receipt",
                ));
            }
        }
        let local = self.load_record(receipt.issue)?;
        let cards = self.load_cards(receipt.issue)?;
        if local != receipt.record || cards != receipt.cards {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "materialized terminal projection differs from retained receipt",
            ));
        }
        if verify_cards(self, &local, &cards).is_err() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "materialized terminal card values or rendered Markdown differ from retained receipt",
            ));
        }
        Ok(())
    }

    pub fn transport_terminal_receipt(
        &self,
        request: TerminalReceiptTransportRequest,
    ) -> Result<IssueRecord> {
        let issue = request.receipt.issue;
        if issue == request.authority_issue || request.actor.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "receipt transport identity is incomplete",
            ));
        }
        for path in request.receipt.authored_artifacts.keys() {
            if !crate::pvf::clean_relative(Path::new(path)) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "transported receipt authored paths must be repository-relative",
                ));
            }
        }
        validate_terminal_receipt(&request.receipt)?;
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < issue {
            (request.authority_issue, issue)
        } else {
            (issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(issue)?;
        let authority = self.load_record(request.authority_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "receipt transport authority is stale",
            ));
        }
        if authority.repository != request.receipt.repository {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "transported receipt repository differs from repair authority",
            ));
        }
        let claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::MissingClaim,
                "receipt transport authority claim missing",
            )
        })?;
        claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(claim, issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "receipt transport authority does not cover target issue",
            ));
        }
        let target_issue_root = PathBuf::from(format!(".csdlc/issues/{issue}"));
        for (path, expected) in &request.receipt.authored_artifacts {
            if Path::new(path).starts_with(&target_issue_root) {
                continue;
            }
            let covered = claim.protected_paths.iter().any(|protected| {
                Path::new(path) == Path::new(protected)
                    || Path::new(path).starts_with(Path::new(protected))
            });
            if !covered {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    format!("receipt transport authority does not cover external artifact {path}"),
                ));
            }
            if let Some(actual) = read_regular_terminal_artifact(&self.root, Path::new(path))? {
                if actual != expected.as_bytes() {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("receipt transport refuses pre-existing external artifact byte drift at {path}"),
                    ));
                }
            }
        }
        let receipt_path = self.terminal_receipt_path(issue)?;
        let projection_exists = self.issue_dir(issue).exists();
        let receipt_exists = receipt_path.exists();
        let (original_record_digest, original_receipt, original_projection) = match (
            projection_exists,
            receipt_exists,
        ) {
            (false, false) => (None, None, None),
            (true, true) => {
                let local = self.load_record(issue)?;
                let local_cards = self.load_cards(issue)?;
                let retained = self.load_terminal_receipt(issue)?.ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "transport target is partially materialized",
                    )
                })?;
                if local == request.receipt.record && retained.digest == request.receipt.digest {
                    self.verify_materialized_terminal_receipt(&retained)?;
                    return Ok(local);
                }
                verify_cards(self, &local, &local_cards)?;
                verify_canonical_projection_bytes(self, &local, &local_cards)?;
                if retained != request.receipt
                    || local.phase != LifecyclePhase::ClosedOut
                    || local.claim.is_some()
                    || local.repository != request.receipt.repository
                    || local.initialization_digest != request.receipt.initialization_digest
                    || local.terminal != request.receipt.record.terminal
                    || request.receipt.record.generation <= local.generation
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "transport target conflicts with existing terminal authority",
                    ));
                }
                let authored_artifacts = [local.design_path.clone(), local.diagram_path.clone()]
                    .into_iter()
                    .map(|path| {
                        if !crate::pvf::clean_relative(Path::new(&path)) {
                            return Err(V2Error::new(
                                ErrorCode::InvalidInput,
                                "transport rollback artifacts must be repository-relative",
                            ));
                        }
                        let bytes = read_regular_terminal_artifact(&self.root, Path::new(&path))?
                            .ok_or_else(|| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "transport rollback artifact is absent",
                            )
                        })?;
                        let contents = String::from_utf8(bytes).map_err(|_| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "transport rollback artifact is not UTF-8",
                            )
                        })?;
                        Ok((path, contents))
                    })
                    .collect::<Result<BTreeMap<_, _>>>()?;
                let digest = local.digest.clone();
                (
                    Some(digest),
                    Some(fs::read(&receipt_path)?),
                    Some(TerminalProjectionSnapshot {
                        record: local,
                        cards: local_cards,
                        authored_artifacts,
                    }),
                )
            }
            (false, true) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "transport target has a receipt without its projection",
                ));
            }
            (true, false) => {
                let local = self.load_record(issue)?;
                let local_cards = self.load_cards(issue)?;
                verify_cards(self, &local, &local_cards)?;
                verify_canonical_projection_bytes(self, &local, &local_cards)?;
                if local.phase == LifecyclePhase::ClosedOut
                    || local.repository != request.receipt.repository
                    || local.initialization_digest != request.receipt.initialization_digest
                    || request.receipt.record.generation <= local.generation
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "transport replacement requires a strictly newer same-identity terminal receipt over a nonterminal projection",
                    ));
                }
                let authored_artifacts = [local.design_path.clone(), local.diagram_path.clone()]
                    .into_iter()
                    .map(|path| {
                        if !crate::pvf::clean_relative(Path::new(&path)) {
                            return Err(V2Error::new(
                                ErrorCode::InvalidInput,
                                "transport rollback artifacts must be repository-relative",
                            ));
                        }
                        let bytes = read_regular_terminal_artifact(&self.root, Path::new(&path))?
                            .ok_or_else(|| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "transport rollback artifact is absent",
                            )
                        })?;
                        let contents = String::from_utf8(bytes).map_err(|_| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "transport rollback artifact is not UTF-8",
                            )
                        })?;
                        Ok((path, contents))
                    })
                    .collect::<Result<BTreeMap<_, _>>>()?;
                let digest = local.digest.clone();
                (
                    Some(digest),
                    None,
                    Some(TerminalProjectionSnapshot {
                        record: local,
                        cards: local_cards,
                        authored_artifacts,
                    }),
                )
            }
        };
        let original_artifacts = request
            .receipt
            .authored_artifacts
            .keys()
            .map(|path| {
                let bytes = read_regular_terminal_artifact(&self.root, Path::new(path))?;
                Ok((path.clone(), bytes))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let target_receipt = serde_json::to_vec_pretty(&request.receipt)?;
        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue,
            stage: "prepared_terminal_receipt_transport".into(),
            original_record_digest,
            original_projection_digest: None,
            target_record_digest: request.receipt.record.digest.clone(),
            original_receipt,
            original_projection,
            original_artifacts,
            target_receipt,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected receipt transport failure",
            ));
        }
        self.commit_with_authored(
            issue,
            &request.receipt.record,
            &request.receipt.cards,
            false,
            Some(&request.receipt.authored_artifacts),
        )?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected receipt transport failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        self.remove_terminal_transaction_journal(issue)?;
        Ok(request.receipt.record)
    }

    pub fn reconcile_corrupt_terminal_receipt(
        &self,
        request: CorruptTerminalReceiptReconciliationRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.reason.trim().is_empty()
            || request.expected_corrupt_projection_digest.trim().is_empty()
            || request.expected_initialization_digest.trim().is_empty()
            || request.expected_receipt_digest.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "corrupt terminal receipt reconciliation identity is incomplete",
            ));
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _binding_lock = self.binding_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;

        let authority = self.load_record(request.authority_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "corrupt terminal reconciliation authority is stale",
            ));
        }
        let claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::MissingClaim,
                "corrupt terminal reconciliation authority claim missing",
            )
        })?;
        claim.validate(&request.authority_claim_id, now_seconds()?)?;
        let authority_root = self.root.canonicalize()?;
        let registered_authority =
            crate::git::worktrees(&self.root)?
                .into_iter()
                .any(|(branch, root)| {
                    branch == claim.branch
                        && PathBuf::from(root).canonicalize().ok() == Some(authority_root.clone())
                });
        if !registered_authority
            || crate::git::current_branch(&self.root)? != claim.branch
            || !claim_worktree_matches_store(self, claim)?
            || !claim_covers_issue(claim, request.authority_issue)
            || !claim_covers_issue(claim, request.target_issue)
            || !claim
                .protected_paths
                .iter()
                .any(|path| path.trim_end_matches('/') == "csdlc-v2")
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "corrupt terminal reconciliation authority does not match the active aggregate checkout",
            ));
        }

        let mut original_artifacts = self.snapshot_issue_projection_bytes(request.target_issue)?;
        if projection_snapshot_digest(&original_artifacts)
            != request.expected_corrupt_projection_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "corrupt terminal target projection changed",
            ));
        }
        let corrupt_index_path = format!(".csdlc/issues/{}/index.json", request.target_issue);
        let corrupt_index = original_artifacts
            .get(&corrupt_index_path)
            .and_then(Option::as_deref)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "corrupt terminal target index is absent",
                )
            })?;
        let corrupt_record: IssueRecord = serde_json::from_slice(corrupt_index)?;
        if let Some(target_claim) = corrupt_record.claim.as_ref() {
            let now = now_seconds()?;
            if now < target_claim.expires_unix_seconds {
                target_claim.validate(&target_claim.id, now)?;
                for (branch, root) in crate::git::worktrees(&self.root)? {
                    if branch != target_claim.branch {
                        continue;
                    }
                    let root = PathBuf::from(root).canonicalize().map_err(|error| {
                        V2Error::new(
                            ErrorCode::UnsafeCheckout,
                            format!("registered target checkout is unavailable: {error}"),
                        )
                    })?;
                    let target_store = Store::new(root);
                    let observed = target_store.load_record(request.target_issue).map_err(
                        |error| {
                            V2Error::new(
                                ErrorCode::UnsafeCheckout,
                                format!(
                                    "registered target checkout cannot authenticate its claim: {}",
                                    error.message
                                ),
                            )
                        },
                    )?;
                    if observed.repository == corrupt_record.repository
                        && observed.initialization_digest == corrupt_record.initialization_digest
                        && observed.claim.as_ref() == Some(target_claim)
                        && claim_worktree_matches_store(&target_store, target_claim)?
                    {
                        return Err(V2Error::new(
                            ErrorCode::UnsafeCheckout,
                            "corrupt terminal reconciliation refuses an authentic unexpired target checkout",
                        ));
                    }
                }
            }
        }

        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_receipt = self.read_terminal_receipt_snapshot(&receipt_path)?;
        let mut receipt = self
            .load_terminal_receipt(request.target_issue)?
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "terminal receipt missing"))?;
        if receipt.digest != request.expected_receipt_digest
            || receipt.issue != request.target_issue
            || receipt.repository != authority.repository
            || receipt.initialization_digest != request.expected_initialization_digest
            || receipt.record.issue != request.target_issue
            || receipt.record.repository != authority.repository
            || receipt.record.initialization_digest != request.expected_initialization_digest
            || receipt.record.phase != LifecyclePhase::ClosedOut
            || receipt.record.claim.is_some()
            || receipt.record.terminal.is_none()
            || corrupt_record.issue != request.target_issue
            || corrupt_record.repository != receipt.repository
            || corrupt_record.initialization_digest != receipt.initialization_digest
            || corrupt_record.generation >= receipt.record.generation
            || corrupt_record.digest == receipt.record.digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "corrupt target and retained terminal receipt identity or ordering differs",
            ));
        }
        let target_issue_root = PathBuf::from(format!(".csdlc/issues/{}", request.target_issue));
        for (path, expected) in &receipt.authored_artifacts {
            let relative = Path::new(path);
            if !crate::pvf::clean_relative(relative) {
                return Err(V2Error::new(
                    ErrorCode::InvalidInput,
                    "terminal receipt authored paths must be repository-relative",
                ));
            }
            if !relative.starts_with(&target_issue_root) {
                let covered = claim.protected_paths.iter().any(|protected| {
                    relative == Path::new(protected) || relative.starts_with(Path::new(protected))
                });
                let current =
                    read_regular_terminal_artifact(&self.root, relative)?.ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            format!("aggregate authored artifact is absent at {path}"),
                        )
                    })?;
                if !covered || current != expected.as_bytes() {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("aggregate authored artifact authority or bytes differ at {path}"),
                    ));
                }
                original_artifacts.insert(path.clone(), Some(current));
            }
        }

        let base_receipt_digest = receipt.digest.clone();
        let mut target = receipt.record.clone();
        let mut cards = receipt.cards.clone();
        target.generation += 1;
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor,
            reason: request.reason,
            operation: serde_json::json!({
                "operation": "reconcile_corrupt_terminal_receipt",
                "authority_issue": request.authority_issue,
                "corrupt_projection_digest": request.expected_corrupt_projection_digest,
                "source_receipt_digest": base_receipt_digest,
            })
            .to_string(),
        });
        for card in cards.values_mut() {
            card.identity.generation = target.generation;
        }
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;
        receipt.record = target.clone();
        receipt.cards = cards.clone();
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;

        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: request.target_issue,
            stage: "prepared_corrupt_terminal_receipt_reconciliation".into(),
            original_record_digest: None,
            original_projection_digest: Some(request.expected_corrupt_projection_digest),
            target_record_digest: target.digest.clone(),
            original_receipt,
            original_projection: None,
            original_artifacts,
            target_receipt: serde_json::to_vec_pretty(&receipt)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected corrupt terminal receipt reconciliation failure",
            ));
        }
        self.commit_with_authored(
            request.target_issue,
            &target,
            &cards,
            false,
            Some(&receipt.authored_artifacts),
        )?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected corrupt terminal receipt reconciliation failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn recover_recordless_terminal(
        &self,
        request: RecordlessTerminalRecoveryRequest,
    ) -> Result<IssueRecord> {
        validate_result(&request.validation)?;
        let issue = request.issue.number;
        let issue_observed = request.issue_evidence.issue.as_ref();
        if issue == 0
            || issue == request.authority_issue
            || request.actor.trim().is_empty()
            || request.reason.trim().is_empty()
            || request.issue.schema != "csdlc.github_issue.v1"
            || request.issue.state != "closed"
            || request.issue.repository.trim().is_empty()
            || request.issue_evidence.schema != "csdlc.github_action_result.v1"
            || !request.issue_evidence.is_producer_verified()
            || request.issue_evidence.action != crate::github::GithubAction::IssueRead
            || !request.issue_evidence.reconciled
            || request.issue_evidence.repository != request.issue.repository
            || issue_observed != Some(&request.issue)
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "recordless terminal recovery identity is incomplete",
            ));
        }
        let (disposition, pull_request, observed_sha, observed_merge_sha, observed_state) =
            match request.closure_kind {
                RecordlessClosureKind::Merged => {
                    let evidence = request.merged_evidence.as_ref().ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::InvalidInput,
                            "recordless merged recovery requires typed PR evidence",
                        )
                    })?;
                    let pr = evidence.pr_state.as_ref().ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::InvalidInput,
                            "recordless merged PR-state packet is absent",
                        )
                    })?;
                    let canonical_url = format!(
                        "https://github.com/{}/pull/{}",
                        request.issue.repository, pr.pull_request
                    );
                    if evidence.schema != "csdlc.github_action_result.v1"
                        || !evidence.is_producer_verified()
                        || evidence.action != crate::github::GithubAction::PrState
                        || !evidence.reconciled
                        || evidence.repository != request.issue.repository
                        || pr.schema != "csdlc.github_pr_state.v1"
                        || pr.repository != request.issue.repository
                        || pr.pull_request == 0
                        || pr.linked_issue != Some(issue)
                        || pr.linkage_source.as_deref() != Some("github_closing_issues_references")
                        || pr.draft
                        || !pr.merged
                        || pr.base_ref.as_deref() != Some("main")
                        || pr.head_ref.as_deref().unwrap_or_default().trim().is_empty()
                        || pr.url.as_deref() != Some(canonical_url.as_str())
                        || !valid_git_sha(&pr.head_sha)
                        || !pr.merge_commit_sha.as_deref().is_some_and(valid_git_sha)
                        || request.related_issue.is_some()
                        || request.related_issue_evidence.is_some()
                        || request.validation.outcome != crate::cards::EvidenceOutcome::Passed
                    {
                        return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "recordless merged recovery requires canonical reconciled PR, head, merge, linkage, and passing evidence",
                    ));
                    }
                    (
                        crate::readiness::TerminalDisposition::Merged,
                        Some(pr.pull_request),
                        Some(pr.head_sha.clone()),
                        pr.merge_commit_sha.clone(),
                        "merged".to_owned(),
                    )
                }
                RecordlessClosureKind::Duplicate | RecordlessClosureKind::Superseded => {
                    let related = request
                        .related_issue_evidence
                        .as_ref()
                        .and_then(|evidence| evidence.issue.as_ref());
                    if request.merged_evidence.is_some()
                        || request.related_issue.is_none()
                        || request.related_issue == Some(issue)
                        || request.validation.outcome != crate::cards::EvidenceOutcome::Passed
                        || request
                            .related_issue_evidence
                            .as_ref()
                            .is_none_or(|evidence| {
                                evidence.schema != "csdlc.github_action_result.v1"
                                    || !evidence.is_producer_verified()
                                    || evidence.action != crate::github::GithubAction::IssueRead
                                    || !evidence.reconciled
                                    || evidence.repository != request.issue.repository
                            })
                        || related.is_none_or(|packet| {
                            packet.schema != "csdlc.github_issue.v1"
                                || packet.repository != request.issue.repository
                                || Some(packet.number) != request.related_issue
                                || !matches!(packet.state.as_str(), "open" | "closed")
                        })
                        || !request
                            .reason
                            .contains(&format!("#{}", request.related_issue.unwrap_or_default()))
                    {
                        return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "recordless duplicate or superseded recovery requires one related issue and no PR evidence",
                    ));
                    }
                    (
                        crate::readiness::TerminalDisposition::ClosedNoPr,
                        None,
                        None,
                        None,
                        "closed_no_pr".to_owned(),
                    )
                }
            };
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < issue {
            (request.authority_issue, issue)
        } else {
            (issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(issue)?;
        let authority = self.load_record(request.authority_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "recordless recovery authority is stale",
            ));
        }
        if authority.repository != request.issue.repository {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "recordless issue repository differs from recovery authority",
            ));
        }
        let claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::MissingClaim,
                "recordless recovery authority claim missing",
            )
        })?;
        claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(claim, issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "recordless recovery authority does not cover target issue",
            ));
        }
        let receipt_path = self.terminal_receipt_path(issue)?;

        let closure_ref = match request.closure_kind {
            RecordlessClosureKind::Merged => format!(
                "pull_request:{} head:{} merge:{}",
                pull_request.expect("validated PR"),
                observed_sha.as_deref().expect("validated head"),
                observed_merge_sha.as_deref().expect("validated merge")
            ),
            RecordlessClosureKind::Duplicate => {
                format!(
                    "duplicate_of:issue:{}",
                    request.related_issue.expect("validated relation")
                )
            }
            RecordlessClosureKind::Superseded => format!(
                "superseded_by:issue:{}",
                request.related_issue.expect("validated relation")
            ),
        };
        let design_path = format!(".csdlc/issues/{issue}/retained/design.md");
        let diagram_path = format!(".csdlc/issues/{issue}/retained/diagram.mmd");
        let body_digest = digest(request.issue.body.as_bytes());
        let design = format!(
            "# Recordless terminal recovery for issue #{issue}\n\n{}\n\nClosure evidence: `{closure_ref}`. GitHub issue state: closed. Source body digest: `{body_digest}`.\n\nThis recovery records observed terminal disposition only. It does not reconstruct or claim historical implementation, review, publication, readiness, or CI execution.\n",
            request.reason,
        );
        let diagram = format!(
            "flowchart LR\n  G[GitHub issue #{issue} closed] --> E[Exact terminal evidence]\n  E --> R[Typed recordless recovery]\n  R --> T[Terminal receipt and projection]\n"
        );
        let design_digest = digest(design.as_bytes());
        let diagram_digest = digest(diagram.as_bytes());
        let version = request
            .issue
            .labels
            .iter()
            .find(|label| label.starts_with("version:"))
            .cloned()
            .unwrap_or_else(|| "version:unclassified".into());
        let recovery_non_claim = "No historical implementation, review, publication, readiness, or CI lifecycle is reconstructed by this terminal recovery.".to_owned();
        let mut cards = initial_cards(
            issue,
            &request.issue.repository,
            &design_path,
            &design_digest,
            &diagram_path,
            &diagram_digest,
            InitialCardInput {
                title: request.issue.title.clone(),
                slug: format!("recordless-terminal-recovery-{issue}"),
                version,
                goal: format!("Retain truthful terminal authority for closed issue #{issue}."),
                required_outcome: "A receipt-backed closed-out projection containing terminal evidence and explicit historical non-claims.".into(),
                declared_scope: vec![format!(".csdlc/issues/{issue}/")],
                authority_boundary: vec![recovery_non_claim.clone()],
                operator_constraints: vec!["typed C-SDLC v2 only".into()],
                task_boundary: "Record observed closure without reconstructing unavailable lifecycle history.".into(),
                deliverables: vec!["terminal projection".into(), "retained terminal receipt".into()],
                acceptance_criteria: vec!["issue is observed closed".into(), "terminal evidence is internally consistent".into()],
                dependencies: request.related_issue.map(|value| vec![format!("issue:{value}")]).unwrap_or_else(|| vec!["exact merged PR evidence".into()]),
                repo_inputs: vec![format!("github-issue:{issue}"), closure_ref, format!("github-body-blake3:{body_digest}")],
                non_goals: vec![recovery_non_claim.clone()],
                plan_summary: "Validate exact closure evidence and atomically retain terminal authority.".into(),
                steps: vec![crate::cards::PlanStep { id: "S1".into(), action: "Retain terminal recovery evidence".into(), acceptance_ids: vec!["AC-1".into(), "AC-2".into()], status: StepStatus::Completed }],
                invariants: vec!["target projection and receipt are absent before recovery".into(), recovery_non_claim.clone()],
                risks: vec!["remote evidence supplied to this deterministic operation must come from the typed GitHub observation surface".into()],
                planning_profile: crate::cards::PlanningProfile::Small,
                stop_conditions: vec!["existing target authority".into(), "stale recovery authority".into(), "inconsistent closure evidence".into()],
                validation_lanes: vec![crate::cards::ValidationLane { lane: "terminal-evidence".into(), proof_role: "Validate observed closure evidence".into(), acceptance_ids: vec!["AC-1".into(), "AC-2".into()], deterministic: true, resource_profile: crate::cards::ResourceProfile::Small, budget_seconds: 30, budget_tokens: 100, argv: request.validation.command.clone(), parallel_group: "terminal-recovery".into(), defer_reason: None }],
                failure_policy: "Fail closed without exact, internally consistent terminal evidence.".into(),
                review_prompts: vec!["Does the receipt preserve explicit non-claims about unavailable lifecycle history?".into()],
                review_scope: format!(".csdlc/issues/{issue}/"),
            },
        )?;
        for card in cards.values_mut() {
            card.identity.generation = 1;
        }
        let CardContent::Srp(srp) = &mut cards.get_mut(&CardKind::Srp).expect("SRP").content else {
            unreachable!()
        };
        srp.residual_risk = vec![recovery_non_claim.clone()];
        let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).expect("SOR").content else {
            unreachable!()
        };
        sor.summary = request.reason.clone();
        sor.actual_changes = vec!["Typed recordless terminal recovery projection only.".into()];
        sor.artifacts = vec![design_path.clone(), diagram_path.clone()];
        sor.actual_validation = vec![request.validation.clone()];
        sor.integration_state = if disposition == crate::readiness::TerminalDisposition::Merged {
            crate::cards::IntegrationState::Merged
        } else {
            crate::cards::IntegrationState::ClosedNoPr
        };
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.merge_state = if disposition == crate::readiness::TerminalDisposition::Merged {
            crate::cards::MergeState::Merged
        } else {
            crate::cards::MergeState::ClosedUnmerged
        };
        sor.closeout_state = crate::cards::CloseoutState::Complete;
        sor.follow_ups = request
            .related_issue
            .map(|value| vec![format!("issue:{value}")])
            .unwrap_or_default();
        cards.get_mut(&CardKind::Sor).expect("SOR").status = crate::cards::CardStatus::Complete;
        let terminal = TerminalEvidence {
            pull_request,
            disposition,
            observed_sha,
            observed_state,
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            released_branch: String::new(),
            released_worktree: String::new(),
            released_protected_paths: Vec::new(),
        };
        let mut semantic_request = request.clone();
        semantic_request.fail_after_stage = None;
        let initialization_digest = digest(&serde_json::to_vec(&semantic_request)?);
        let transition = TransitionEvent {
            sequence: 1,
            from: LifecyclePhase::Initialized,
            to: LifecyclePhase::ClosedOut,
            actor: request.actor.clone(),
            reason: request.reason.clone(),
        };
        let audit = AuditEvent {
            sequence: 1,
            generation: 1,
            actor: request.actor.clone(),
            reason: request.reason.clone(),
            operation: "recover_recordless_terminal".into(),
        };
        let mut record = IssueRecord {
            schema: "csdlc.issue.index.v1".into(),
            issue,
            repository: request.issue.repository.clone(),
            initialization_digest,
            phase: LifecyclePhase::ClosedOut,
            generation: 1,
            digest: String::new(),
            claim: None,
            review_assignment: None,
            review: None,
            publication: None,
            readiness: None,
            terminal: Some(terminal),
            migration: None,
            design_path: design_path.clone(),
            diagram_path: diagram_path.clone(),
            design_review: DesignReview::Approved {
                reviewer: request.actor.clone(),
                revision: design_digest,
            },
            cards: BTreeMap::new(),
            transitions: vec![transition],
            audit: vec![audit],
        };
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        let authored_artifacts = BTreeMap::from([(design_path, design), (diagram_path, diagram)]);
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{issue}.json"),
            authored_artifacts,
            record: record.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        match (self.issue_dir(issue).exists(), receipt_path.exists()) {
            (true, true) => {
                let retained = self.load_terminal_receipt(issue)?.ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "recordless replay receipt is absent",
                    )
                })?;
                if retained != receipt {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "recordless recovery conflicts with existing terminal authority",
                    ));
                }
                self.verify_materialized_terminal_receipt(&retained)?;
                return Ok(retained.record);
            }
            (false, false) => {}
            _ => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "recordless recovery target is partially materialized",
                ));
            }
        }
        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue,
            stage: "prepared_recordless_terminal_recovery".into(),
            original_record_digest: None,
            original_projection_digest: None,
            target_record_digest: record.digest.clone(),
            original_receipt: None,
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt: serde_json::to_vec_pretty(&receipt)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected recordless recovery failure",
            ));
        }
        self.commit_with_authored(
            issue,
            &record,
            &cards,
            false,
            Some(&receipt.authored_artifacts),
        )?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected recordless recovery failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        self.remove_terminal_transaction_journal(issue)?;
        Ok(record)
    }

    pub fn reconcile_historical_merged(
        &self,
        request: HistoricalMergedReconciliationRequest,
    ) -> Result<IssueRecord> {
        self.reconcile_historical_merged_inner(request, None)
    }

    pub fn recover_corrupt_historical_merged(
        &self,
        request: CorruptHistoricalMergedRecoveryRequest,
    ) -> Result<IssueRecord> {
        let source = CorruptHistoricalSource {
            authority_worktree: request.authority_worktree,
            commit: request.source_commit,
            expected_projection_digest: request.expected_corrupt_projection_digest,
            required_checks: request.required_checks,
            require_review: request.require_review,
            expected_target_claim: request.expected_target_claim,
        };
        self.reconcile_historical_merged_inner(
            HistoricalMergedReconciliationRequest {
                authority_issue: request.authority_issue,
                expected_authority_generation: request.expected_authority_generation,
                expected_authority_digest: request.expected_authority_digest,
                authority_claim_id: request.authority_claim_id,
                target_issue: request.target_issue,
                expected_target_generation: request.expected_source_generation,
                expected_target_digest: request.expected_source_digest,
                expected_initialization_digest: request.expected_initialization_digest,
                reviewed_commit: request.reviewed_commit,
                review: request.review,
                issue_evidence: request.issue_evidence,
                merged_evidence: request.merged_evidence,
                actor: request.actor,
                operator_authority: request.operator_authority,
                reason: request.reason,
                validation: request.validation,
                fail_after_stage: request.fail_after_stage,
            },
            Some(source),
        )
    }

    fn reconcile_historical_merged_inner(
        &self,
        request: HistoricalMergedReconciliationRequest,
        corrupt_source: Option<CorruptHistoricalSource>,
    ) -> Result<IssueRecord> {
        validate_result(&request.validation)?;
        let issue = request.target_issue;
        let observed_issue = request.issue_evidence.issue.as_ref();
        let pr =
            request.merged_evidence.pr_state.as_ref().ok_or_else(|| {
                V2Error::new(ErrorCode::InvalidInput, "historical PR packet missing")
            })?;
        let canonical_url = format!(
            "https://github.com/{}/pull/{}",
            pr.repository, pr.pull_request
        );
        if request.authority_issue == issue
            || request.actor.trim().is_empty()
            || request.operator_authority.trim().is_empty()
            || request.reason.trim().is_empty()
            || request.reviewed_commit.len() != 40
            || !request
                .reviewed_commit
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || corrupt_source.as_ref().is_some_and(|source| {
                source.commit.len() != 40
                    || !source.commit.bytes().all(|byte| byte.is_ascii_hexdigit())
                    || source.expected_projection_digest.trim().is_empty()
                    || source.required_checks.is_empty()
                    || source
                        .required_checks
                        .iter()
                        .any(|name| name.trim().is_empty())
            })
            || !request.review.completed
            || request.review.findings.iter().any(|finding| {
                finding.actionable
                    && finding.in_scope
                    && finding.disposition == crate::cards::FindingDisposition::Open
            })
            || request.validation.outcome != crate::cards::EvidenceOutcome::Passed
            || request.issue_evidence.schema != "csdlc.github_action_result.v1"
            || !request.issue_evidence.is_producer_verified()
            || request.issue_evidence.action != crate::github::GithubAction::IssueRead
            || !request.issue_evidence.reconciled
            || observed_issue.is_none_or(|observed| {
                observed.schema != "csdlc.github_issue.v1"
                    || observed.number != issue
                    || observed.repository != pr.repository
                    || observed.state != "closed"
            })
            || request.merged_evidence.schema != "csdlc.github_action_result.v1"
            || !request.merged_evidence.is_producer_verified()
            || request.merged_evidence.action != crate::github::GithubAction::PrState
            || !request.merged_evidence.reconciled
            || request.merged_evidence.repository != pr.repository
            || pr.schema != "csdlc.github_pr_state.v1"
            || pr.pull_request == 0
            || pr.linked_issue != Some(issue)
            || pr.linkage_source.as_deref() != Some("github_closing_issues_references")
            || pr.repository != request.issue_evidence.repository
            || pr.base_ref.as_deref() != Some("main")
            || pr.head_ref.as_deref().is_none_or(str::is_empty)
            || pr.url.as_deref() != Some(canonical_url.as_str())
            || pr.draft
            || !pr.merged
            || !valid_git_sha(&pr.head_sha)
            || !pr.merge_commit_sha.as_deref().is_some_and(valid_git_sha)
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "historical merge reconciliation requires closed linked issue, exact merged PR, current review, validation, and explicit operator authority",
            ));
        }
        if let Some(source) = &corrupt_source {
            if pr.required_check_names != source.required_checks
                || (source.require_review && pr.review_decision != "approved")
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "corrupt historical recovery requires explicit exact CI and repository-required review observations",
                ));
            }
            if !git_is_ancestor(&self.root, &source.commit, &pr.head_sha)? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "corrupt historical source is not an ancestor of the merged PR head",
                ));
            }
        }
        // A historical PR body without a closing keyword is accepted only in
        // this recovery-only path: linked_issue, a reconciled closed issue, and
        // explicit operator authority above are all mandatory. Normal publish
        // validation remains unchanged.
        let body_has_closing_keyword = pr.body.as_deref().is_some_and(|body| {
            let lower = body.to_ascii_lowercase();
            ["closes", "closed", "fixes", "fixed", "resolves", "resolved"]
                .iter()
                .any(|keyword| lower.contains(&format!("{keyword} #{issue}")))
        });
        let (changed_paths, metadata_direction) = if corrupt_source.is_some() {
            // Corrupt historical recovery can run after unrelated work has
            // landed between the recorded review and the merged PR head. The
            // target checkout is authenticated against `pr.head_sha` below,
            // and the declared review scope is independently required to
            // remain byte-identical to `reviewed_commit`. Do not require the
            // repository-wide intervening delta to be lifecycle metadata.
            (Vec::new(), "reviewed_scope_exact_at_merged_head")
        } else if request.reviewed_commit == pr.head_sha {
            (Vec::new(), "exact_head")
        } else if let Ok(paths) = crate::git::metadata_only_changed_paths(
            &self.root,
            &pr.head_sha,
            &request.reviewed_commit,
        ) {
            (paths, "merged_head_to_reviewed_commit")
        } else {
            let paths = crate::git::metadata_only_changed_paths(
                &self.root,
                &request.reviewed_commit,
                &pr.head_sha,
            )
            .map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "historical PR head is not a verified forward lifecycle-metadata revision",
                )
            })?;
            (paths, "reviewed_commit_to_merged_head")
        };

        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _binding_lock = self.binding_lock()?;
        let authority_store = corrupt_source
            .as_ref()
            .map(|source| {
                let root = PathBuf::from(&source.authority_worktree)
                    .canonicalize()
                    .map_err(|error| {
                        V2Error::new(
                            ErrorCode::UnsafeCheckout,
                            format!("corrupt recovery authority worktree is unavailable: {error}"),
                        )
                    })?;
                if root == self.root.canonicalize()? {
                    return Err(V2Error::new(
                        ErrorCode::UnsafeCheckout,
                        "corrupt recovery requires distinct authority and target worktrees",
                    ));
                }
                Ok(Store::new(root))
            })
            .transpose()?;
        let authority_store_ref = authority_store.as_ref().unwrap_or(self);
        let (_first_lock, _second_lock) = if corrupt_source.is_some() {
            (
                authority_store_ref.lock(request.authority_issue)?,
                self.lock(issue)?,
            )
        } else {
            let (first, second) = if request.authority_issue < issue {
                (request.authority_issue, issue)
            } else {
                (issue, request.authority_issue)
            };
            (self.lock(first)?, self.lock(second)?)
        };
        authority_store_ref.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(issue)?;
        let authority = authority_store_ref.load_record(request.authority_issue)?;
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "historical recovery authority is stale",
            ));
        }
        let authority_claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::MissingClaim,
                "historical recovery authority claim missing",
            )
        })?;
        authority_claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if corrupt_source.is_some()
            && (!crate::git::worktrees(&self.root)?
                .into_iter()
                .any(|(branch, root)| {
                    branch == authority_claim.branch
                        && PathBuf::from(root).canonicalize().ok()
                            == authority_store_ref.root.canonicalize().ok()
                })
                || crate::git::current_branch(authority_store_ref.root())?
                    != authority_claim.branch
                || !claim_worktree_matches_store(authority_store_ref, authority_claim)?)
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "corrupt recovery authority claim does not match its active checkout",
            ));
        }
        let aggregate_recovery_authority = authority_claim
            .protected_paths
            .iter()
            .any(|path| path.trim_end_matches('/') == "csdlc-v2")
            && claim_covers_issue(authority_claim, request.authority_issue);
        if corrupt_source.is_some() && !aggregate_recovery_authority {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "corrupt historical recovery authority does not own its C-SDLC recovery surface",
            ));
        }
        if corrupt_source.is_none() && !claim_covers_issue(authority_claim, issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "historical recovery authority does not cover target issue",
            ));
        }
        let (original, target_cards, source_authored_artifacts, corrupt_projection_snapshot) =
            if let Some(source) = &corrupt_source {
                let snapshot = self.snapshot_issue_projection_bytes(issue)?;
                if projection_snapshot_digest(&snapshot) != source.expected_projection_digest {
                    return Err(V2Error::new(
                        ErrorCode::StaleDigest,
                        "corrupt historical target projection changed",
                    ));
                }
                let corrupt_index_bytes = snapshot
                    .get(&format!(".csdlc/issues/{issue}/index.json"))
                    .and_then(Option::as_deref)
                    .ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::CorruptRecord,
                            "corrupt historical target index is absent",
                        )
                    })?;
                let corrupt_record: IssueRecord = serde_json::from_slice(corrupt_index_bytes)?;
                let corrupt_claim = corrupt_record.claim.as_ref().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::MissingClaim,
                        "corrupt historical target claim is absent",
                    )
                })?;
                if corrupt_record.issue != issue
                    || corrupt_record.repository != pr.repository
                    || corrupt_record.initialization_digest
                        != request.expected_initialization_digest
                    || corrupt_claim != &source.expected_target_claim
                    || corrupt_claim.generation != corrupt_record.generation
                    || !claim_covers_issue(corrupt_claim, issue)
                    || !request.review.scope.iter().all(|reviewed| {
                        corrupt_claim
                            .protected_paths
                            .iter()
                            .any(|claimed| claimed.trim_end_matches('/') == reviewed)
                    })
                {
                    return Err(V2Error::new(
                        ErrorCode::InvalidClaim,
                        "corrupt historical target claim does not match exact active authority",
                    ));
                }
                corrupt_claim.validate(&source.expected_target_claim.id, now_seconds()?)?;
                let target_root = self.root.canonicalize()?;
                let registered_target_roots = crate::git::worktrees(&self.root)?
                    .into_iter()
                    .filter(|(branch, _)| branch == &corrupt_claim.branch)
                    .filter_map(|(_, root)| PathBuf::from(root).canonicalize().ok())
                    .collect::<Vec<_>>();
                if registered_target_roots.as_slice() != [target_root.clone()]
                    || crate::git::current_branch(&self.root)? != corrupt_claim.branch
                    || crate::git::run(&self.root, &["rev-parse", "HEAD"])?.stdout != pr.head_sha
                    || !claim_worktree_matches_store(self, corrupt_claim)?
                {
                    return Err(V2Error::new(
                        ErrorCode::UnsafeCheckout,
                        "corrupt historical target claim does not match its unique active checkout",
                    ));
                }
                let source_snapshot = self.load_historical_source(issue, &source.commit)?;
                let mut snapshot = snapshot;
                for (path, contents) in &source_snapshot.authored_artifacts {
                    let relative = Path::new(path);
                    let current = read_regular_terminal_artifact(&self.root, relative)?
                        .ok_or_else(|| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "corrupt recovery authored artifact is absent",
                            )
                        })?;
                    if current != contents.as_bytes() {
                        return Err(V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "corrupt recovery authored artifact differs from the pinned source",
                        ));
                    }
                    snapshot.insert(path.clone(), Some(current));
                }
                (
                    source_snapshot.record,
                    source_snapshot.cards,
                    Some(source_snapshot.authored_artifacts),
                    snapshot,
                )
            } else {
                let original = self.load_record(issue)?;
                let target_cards = self.load_cards(issue)?;
                verify_cards(self, &original, &target_cards)?;
                verify_canonical_projection_bytes(self, &original, &target_cards)?;
                (original, target_cards, None, BTreeMap::new())
            };
        let review_commit = request
            .review
            .reviewed_revision
            .strip_prefix("git-blake3:")
            .and_then(|value| value.split(':').next());
        if original.generation != request.expected_target_generation
            || original.digest != request.expected_target_digest
            || original.initialization_digest != request.expected_initialization_digest
            || original.repository != pr.repository
            || matches!(
                original.phase,
                LifecyclePhase::Merged | LifecyclePhase::ClosedOut
            )
            || !matches!(
                original.phase,
                LifecyclePhase::Implemented
                    | LifecyclePhase::Reviewed
                    | LifecyclePhase::Published
                    | LifecyclePhase::MergeReady
            )
            || review_commit != Some(request.reviewed_commit.as_str())
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "historical target identity, state, or compare-and-swap is stale",
            ));
        }
        let rehome_operation = original
            .audit
            .last()
            .and_then(|event| serde_json::from_str::<serde_json::Value>(&event.operation).ok())
            .filter(|operation| {
                operation.get("operation").and_then(|value| value.as_str())
                    == Some("rehome_claim_authority")
            });
        let mut rehome_source_lock = None;
        let mut cards = if corrupt_source.is_some() {
            if rehome_operation.is_some() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "corrupt historical source cannot be a rehome carrier",
                ));
            }
            if !crate::git::substantive_scope_matches_commit(
                &self.root,
                &request.reviewed_commit,
                &request.review.scope,
            )? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "historical target checkout differs from the reviewed substantive scope",
                ));
            }
            target_cards.clone()
        } else if let Some(operation) = rehome_operation {
            let source_worktree = operation
                .get("source_worktree")
                .and_then(|value| value.as_str())
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source worktree evidence is absent",
                    )
                })?;
            let source_branch = operation
                .get("source_branch")
                .and_then(|value| value.as_str())
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source branch evidence is absent",
                    )
                })?;
            let source_digest = operation
                .get("source_digest")
                .and_then(|value| value.as_str())
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source digest evidence is absent",
                    )
                })?;
            let source_generation = operation
                .get("source_generation")
                .and_then(|value| value.as_u64())
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source generation evidence is absent",
                    )
                })?;
            let source_commit = operation
                .get("source_commit")
                .and_then(|value| value.as_str())
                .ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source commit evidence is absent",
                    )
                })?;
            let source_root = PathBuf::from(source_worktree)
                .canonicalize()
                .map_err(|error| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("rehome source worktree is unavailable: {error}"),
                    )
                })?;
            let source_registered =
                crate::git::worktrees(&self.root)?
                    .into_iter()
                    .any(|(branch, root)| {
                        branch == source_branch
                            && PathBuf::from(root)
                                .canonicalize()
                                .is_ok_and(|candidate| candidate == source_root)
                    });
            let source_store = Store::new(source_root);
            rehome_source_lock = Some(source_store.lock(issue)?);
            if !source_registered
                || crate::git::current_branch(source_store.root())? != source_branch
                || crate::git::run(source_store.root(), &["rev-parse", "HEAD"])?.stdout
                    != source_commit
                || source_commit != request.reviewed_commit
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "rehome source branch, worktree, or reviewed commit changed",
                ));
            }
            let source = source_store.load_record(issue)?;
            let source_cards = source_store.load_cards(issue)?;
            verify_cards(&source_store, &source, &source_cards)?;
            verify_canonical_projection_bytes(&source_store, &source, &source_cards)?;
            if source.repository != original.repository
                || source.initialization_digest != original.initialization_digest
                || source.generation != source_generation
                || source.digest != source_digest
                || source.claim.is_some()
                || source.phase != LifecyclePhase::Reviewed
                || source.review.as_ref() != Some(&request.review)
                || original.review.as_ref() != Some(&request.review)
                || crate::git::substantive_revision(source_store.root(), &request.review.scope)?
                    != request.review.reviewed_revision
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "rehome lineage no longer resolves to the exact reviewed source authority",
                ));
            }
            for path in [&source.design_path, &source.diagram_path] {
                if read_regular_projection(source_store.root(), Path::new(path))?
                    != read_regular_projection(&self.root, Path::new(path))?
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "rehome source authored artifacts differ from the carrier checkout",
                    ));
                }
            }
            source_cards
        } else {
            if !crate::git::substantive_scope_matches_commit(
                &self.root,
                &request.reviewed_commit,
                &request.review.scope,
            )? {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "historical target checkout differs from the reviewed substantive scope",
                ));
            }
            target_cards.clone()
        };
        let _rehome_source_lock = rehome_source_lock;
        let authored_artifacts = if let Some(authored) = source_authored_artifacts {
            authored
        } else {
            [original.design_path.clone(), original.diagram_path.clone()]
                .into_iter()
                .map(|path| {
                    let bytes = read_regular_terminal_artifact(&self.root, Path::new(&path))?
                        .ok_or_else(|| {
                            V2Error::new(
                                ErrorCode::ReconciliationRequired,
                                "historical authored artifact is absent",
                            )
                        })?;
                    let contents = String::from_utf8(bytes).map_err(|_| {
                        V2Error::new(
                            ErrorCode::CorruptRecord,
                            "historical authored artifact is not UTF-8",
                        )
                    })?;
                    Ok((path, contents))
                })
                .collect::<Result<BTreeMap<_, _>>>()?
        };
        let original_projection = corrupt_source
            .is_none()
            .then(|| TerminalProjectionSnapshot {
                record: original.clone(),
                cards: target_cards.clone(),
                authored_artifacts: authored_artifacts.clone(),
            });

        let srp = match &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            CardContent::Srp(values) => values,
            _ => unreachable!(),
        };
        srp.reviewer = Some(request.review.reviewer.clone());
        srp.review_scope = request.review.scope.join("\n");
        srp.review_revision = Some(request.review.reviewed_revision.clone());
        srp.review_result = crate::cards::ReviewResult::Pass;
        srp.residual_risk = request.review.residual_risks.clone();
        srp.findings = request
            .review
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
        let sor_values = cards.get_mut(&CardKind::Sor).expect("SOR");
        sor_values.status = crate::cards::CardStatus::Complete;
        let CardContent::Sor(sor) = &mut sor_values.content else {
            unreachable!()
        };
        if !sor.actual_validation.contains(&request.validation) {
            sor.actual_validation.push(request.validation.clone());
        }
        sor.integration_state = crate::cards::IntegrationState::Merged;
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.merge_state = crate::cards::MergeState::Merged;
        sor.closeout_state = crate::cards::CloseoutState::Complete;

        let checks = pr
            .checks
            .iter()
            .map(|check| crate::readiness::CheckObservation {
                name: check.name.clone(),
                requirement: if check.required {
                    crate::readiness::CheckRequirement::Required
                } else {
                    crate::readiness::CheckRequirement::Optional
                },
                conclusion: match check.conclusion.as_str() {
                    "success" => crate::readiness::CheckConclusion::Success,
                    "failure" => crate::readiness::CheckConclusion::Failure,
                    "cancelled" => crate::readiness::CheckConclusion::Cancelled,
                    "skipped" => crate::readiness::CheckConclusion::Skipped,
                    "neutral" => crate::readiness::CheckConclusion::Neutral,
                    "pending" => crate::readiness::CheckConclusion::Pending,
                    _ => crate::readiness::CheckConclusion::Unknown,
                },
                details_url: check.details_url.clone(),
            })
            .collect::<Vec<_>>();
        if pr.required_check_names.iter().any(|name| {
            !checks.iter().any(|check| {
                check.name == *name
                    && check.requirement == crate::readiness::CheckRequirement::Required
                    && check.conclusion == crate::readiness::CheckConclusion::Success
            })
        }) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "historical required checks are not exact successful observations",
            ));
        }
        let review_state = match pr.review_decision.as_str() {
            "approved" => crate::readiness::RemoteReviewState::Approved,
            "changes_requested" => crate::readiness::RemoteReviewState::ChangesRequested,
            "pending" => crate::readiness::RemoteReviewState::Pending,
            _ => crate::readiness::RemoteReviewState::Unknown,
        };
        let publication = PublicationEvidence {
            repository: pr.repository.clone(),
            issue,
            pull_request: pr.pull_request,
            url: canonical_url,
            base: "main".into(),
            head: pr.head_ref.clone().expect("validated head ref"),
            revision: crate::git::clean_commit_revision(&pr.head_sha),
            draft: false,
            observed_state: "merged".into(),
        };
        let readiness = ReadinessEvidence {
            pull_request: pr.pull_request,
            head_sha: pr.head_sha.clone(),
            checks,
            review_state,
            conflict_state: crate::readiness::ConflictState::Clean,
            post_publication_findings: Vec::new(),
            ready: true,
            blockers: Vec::new(),
        };
        let released = corrupt_source
            .as_ref()
            .map(|source| source.expected_target_claim.clone())
            .or_else(|| original.claim.clone());
        let terminal = TerminalEvidence {
            pull_request: Some(pr.pull_request),
            disposition: crate::readiness::TerminalDisposition::Merged,
            observed_sha: Some(pr.head_sha.clone()),
            observed_state: "merged".into(),
            receipt_path: format!("csdlc-v2/closeout/{issue}.json"),
            released_branch: released
                .as_ref()
                .map(|claim| claim.branch.clone())
                .unwrap_or_default(),
            released_worktree: released
                .as_ref()
                .map(|claim| claim.worktree.clone())
                .unwrap_or_default(),
            released_protected_paths: released
                .map(|claim| claim.protected_paths)
                .unwrap_or_default(),
        };
        let mut target = original.clone();
        target.generation += 1;
        target.claim = None;
        target.review = Some(request.review);
        target.publication = Some(publication);
        target.readiness = Some(readiness);
        target.terminal = Some(terminal);
        for next in [
            LifecyclePhase::Reviewed,
            LifecyclePhase::Published,
            LifecyclePhase::MergeReady,
            LifecyclePhase::Merged,
            LifecyclePhase::ClosedOut,
        ] {
            if target.phase == next {
                continue;
            }
            if target.phase.allows(next) {
                let from = target.phase;
                target.phase = next;
                target.transitions.push(TransitionEvent {
                    sequence: target.transitions.len() as u64 + 1,
                    from,
                    to: next,
                    actor: request.actor.clone(),
                    reason: "reconcile exact historical merged lifecycle evidence".into(),
                });
            }
        }
        if target.phase != LifecyclePhase::ClosedOut {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "historical merge reconciliation cannot construct a valid forward lifecycle",
            ));
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor,
            reason: request.reason,
            operation: serde_json::json!({
                "operation": "reconcile_historical_merged",
                "operator_authority": request.operator_authority,
                "reviewed_commit": request.reviewed_commit,
                "merged_head": pr.head_sha,
                "merge_commit": pr.merge_commit_sha,
                "metadata_only_paths": changed_paths,
                "metadata_direction": metadata_direction,
                "historical_closing_keyword_present": body_has_closing_keyword,
                "linkage_source": "typed_linked_issue_and_closed_issue_evidence",
                "released_target_claim": corrupt_source.as_ref().map(|source| serde_json::json!({
                    "id": source.expected_target_claim.id,
                    "owner": source.expected_target_claim.owner,
                    "generation": source.expected_target_claim.generation,
                    "protected_paths_digest": digest(&serde_json::to_vec(&source.expected_target_claim.protected_paths).expect("claim paths serialize")),
                })),
            })
            .to_string(),
        });
        for values in cards.values_mut() {
            values.identity.generation = target.generation;
        }
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: target.repository.clone(),
            initialization_digest: target.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{issue}.json"),
            authored_artifacts,
            record: target.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let receipt_path = self.terminal_receipt_path(issue)?;
        let original_receipt = self.read_terminal_receipt_snapshot(&receipt_path)?;
        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue,
            stage: "prepared_historical_merged_reconciliation".into(),
            original_record_digest: corrupt_source.is_none().then_some(original.digest),
            original_projection_digest: corrupt_source
                .as_ref()
                .map(|source| source.expected_projection_digest.clone()),
            target_record_digest: target.digest.clone(),
            original_receipt,
            original_projection,
            original_artifacts: corrupt_projection_snapshot,
            target_receipt: serde_json::to_vec_pretty(&receipt)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected historical reconciliation failure",
            ));
        }
        if corrupt_source.is_some() {
            self.commit_with_authored(
                issue,
                &target,
                &cards,
                false,
                Some(&receipt.authored_artifacts),
            )?;
        } else {
            self.commit(issue, &target, &cards, false)?;
        }
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected historical reconciliation failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        self.remove_terminal_transaction_journal(issue)?;
        Ok(target)
    }

    pub fn repair_terminal_disposition(
        &self,
        request: TerminalDispositionRepairRequest,
    ) -> Result<IssueRecord> {
        if request.authority_issue == request.target_issue
            || request.actor.trim().is_empty()
            || request.correction_note.trim().is_empty()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal disposition repair identity is incomplete",
            ));
        }
        let evidence = &request.merged_evidence;
        let pr = evidence.pr_state.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "merged PR evidence packet is absent",
            )
        })?;
        let canonical_url = format!(
            "https://github.com/{}/pull/{}",
            pr.repository, pr.pull_request
        );
        if evidence.schema != "csdlc.github_action_result.v1"
            || !evidence.is_producer_verified()
            || evidence.action != crate::github::GithubAction::PrState
            || !evidence.reconciled
            || evidence.repository != pr.repository
            || pr.schema != "csdlc.github_pr_state.v1"
            || pr.linked_issue != Some(request.target_issue)
            || !pr.merged
            || pr.draft
            || pr.base_ref.as_deref() != Some("main")
            || pr.head_ref.as_deref().unwrap_or_default().trim().is_empty()
            || pr.url.as_deref() != Some(canonical_url.as_str())
            || !valid_git_sha(&pr.head_sha)
            || !pr.merge_commit_sha.as_deref().is_some_and(valid_git_sha)
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal disposition repair requires canonical provenance-bound merged PR evidence",
            ));
        }
        let replacement_terminal = TerminalEvidence {
            pull_request: Some(pr.pull_request),
            disposition: crate::readiness::TerminalDisposition::Merged,
            observed_sha: Some(pr.head_sha.clone()),
            observed_state: "merged".into(),
            receipt_path: request.expected_terminal.receipt_path.clone(),
            released_branch: request.expected_terminal.released_branch.clone(),
            released_worktree: request.expected_terminal.released_worktree.clone(),
            released_protected_paths: request.expected_terminal.released_protected_paths.clone(),
        };
        let replacement_publication = PublicationEvidence {
            repository: pr.repository.clone(),
            issue: request.target_issue,
            pull_request: pr.pull_request,
            url: canonical_url,
            base: "main".into(),
            head: pr.head_ref.clone().expect("validated head ref"),
            revision: crate::git::clean_commit_revision(&pr.head_sha),
            draft: false,
            observed_state: "merged".into(),
        };
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let (first, second) = if request.authority_issue < request.target_issue {
            (request.authority_issue, request.target_issue)
        } else {
            (request.target_issue, request.authority_issue)
        };
        let _first_lock = self.lock(first)?;
        let _second_lock = self.lock(second)?;
        self.recover_with_terminal_lock(request.authority_issue)?;
        self.recover_with_terminal_lock(request.target_issue)?;
        let authority = self.load_record(request.authority_issue)?;
        let mut target = self.load_record(request.target_issue)?;
        if replacement_publication.repository != target.repository {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "replacement publication repository differs from target",
            ));
        }
        if authority.generation != request.expected_authority_generation
            || authority.digest != request.expected_authority_digest
            || target.generation != request.expected_target_generation
            || target.digest != request.expected_target_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal disposition repair state is stale",
            ));
        }
        let claim = authority.claim.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::MissingClaim,
                "terminal disposition repair authority claim missing",
            )
        })?;
        claim.validate(&request.authority_claim_id, now_seconds()?)?;
        if !claim_covers_issue(claim, request.target_issue) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal disposition repair authority does not cover target issue",
            ));
        }
        if target.phase != LifecyclePhase::ClosedOut
            || target.claim.is_some()
            || target.terminal.as_ref() != Some(&request.expected_terminal)
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal disposition repair expected evidence differs",
            ));
        }
        let receipt_path = self.terminal_receipt_path(request.target_issue)?;
        let original_bytes = fs::read(&receipt_path)?;
        let original: TerminalReceipt = serde_json::from_slice(&original_bytes)?;
        validate_terminal_receipt(&original)?;
        if original.digest != request.expected_receipt_digest
            || original.record.digest != target.digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal disposition repair receipt is stale",
            ));
        }
        let original_record_digest = original.record.digest.clone();
        let mut cards = self.load_cards(request.target_issue)?;
        verify_cards(self, &target, &cards)?;
        target.terminal = Some(replacement_terminal);
        target.publication = Some(replacement_publication);
        target.generation += 1;
        if let Some(review) = target.review.as_mut() {
            if !review.residual_risks.contains(&request.correction_note) {
                review.residual_risks.push(request.correction_note.clone());
            }
        }
        for card in cards.values_mut() {
            card.identity.generation = target.generation;
        }
        if let CardContent::Srp(srp) = &mut cards.get_mut(&CardKind::Srp).expect("SRP").content {
            if !srp.residual_risk.contains(&request.correction_note) {
                srp.residual_risk.push(request.correction_note.clone());
            }
        }
        if let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            sor.integration_state = crate::cards::IntegrationState::Merged;
            sor.merge_state = crate::cards::MergeState::Merged;
        }
        target.audit.push(AuditEvent {
            sequence: target.audit.len() as u64 + 1,
            generation: target.generation,
            actor: request.actor,
            reason: request.correction_note,
            operation: "repair_terminal_disposition".into(),
        });
        hydrate_projections(&mut target, &cards)?;
        target.digest = record_digest(&target)?;
        let mut repaired = original;
        repaired.record = target.clone();
        repaired.cards = cards.clone();
        repaired.digest.clear();
        repaired.digest = terminal_receipt_digest(&repaired)?;
        validate_terminal_receipt(&repaired)?;
        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: request.target_issue,
            stage: "prepared_terminal_disposition_repair".into(),
            original_record_digest: Some(original_record_digest),
            original_projection_digest: None,
            target_record_digest: target.digest.clone(),
            original_receipt: Some(original_bytes),
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt: serde_json::to_vec_pretty(&repaired)?,
        };
        self.write_terminal_transaction_journal(&journal)?;
        if request.fail_after_stage.as_deref() == Some("after_journal") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected terminal disposition repair failure",
            ));
        }
        self.commit(request.target_issue, &target, &cards, false)?;
        if request.fail_after_stage.as_deref() == Some("after_projection") {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                "injected terminal disposition repair failure",
            ));
        }
        self.replace_receipt_bytes(&receipt_path, Some(&journal.target_receipt))?;
        self.remove_terminal_transaction_journal(request.target_issue)?;
        Ok(target)
    }

    pub fn retain_terminal_receipt(&self, issue: u64) -> Result<TerminalReceipt> {
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _lock = self.lock(issue)?;
        require_canonical_issue_projection_components(&self.root, issue)?;
        self.recover_with_terminal_lock(issue)?;
        require_canonical_issue_projection_files(&self.root, issue)?;
        let mut record = self.load_record(issue)?;
        let mut cards = self.load_cards(issue)?;
        let receipt_ref = format!("csdlc-v2/closeout/{issue}.json");
        let path = self.terminal_receipt_path(issue)?;
        let common = PathBuf::from(
            crate::git::run(
                &self.root,
                &["rev-parse", "--path-format=absolute", "--git-common-dir"],
            )?
            .stdout,
        );
        let receipt_relative = PathBuf::from(&receipt_ref);
        require_canonical_parent_beneath(&common, &receipt_relative)?;
        let parent = path.parent().expect("receipt parent");
        fs::create_dir_all(parent)?;
        require_canonical_parent_beneath(&common, &receipt_relative)?;
        let lock_relative = PathBuf::from("csdlc-v2/closeout/receipts.lock");
        require_regular_or_absent_beneath(&common, &lock_relative)?;
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        if let Some(existing) = self.load_terminal_receipt(issue)? {
            let terminal_matches = record.terminal.as_ref().is_some_and(|local| {
                existing.record.terminal.as_ref().is_some_and(|retained| {
                    local.pull_request == retained.pull_request
                        && local.disposition == retained.disposition
                        && local.observed_sha == retained.observed_sha
                        && local.observed_state == retained.observed_state
                        && local.released_branch == retained.released_branch
                        && local.released_worktree == retained.released_worktree
                        && local.released_protected_paths == retained.released_protected_paths
                })
            });
            if existing.repository != record.repository
                || existing.initialization_digest != record.initialization_digest
                || existing.record.generation != record.generation
                || existing.record.digest != record.digest
                || existing.cards != cards
                || !terminal_matches
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "terminal receipt conflicts with retained authority",
                ));
            }
            verify_canonical_projection_bytes(self, &record, &cards)?;
            for (relative, expected) in &existing.authored_artifacts {
                let observed = read_regular_terminal_artifact(&self.root, Path::new(relative))?
                    .ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "terminal authored artifact is absent",
                        )
                    })?;
                if observed != expected.as_bytes() {
                    return Err(V2Error::new(
                        ErrorCode::CorruptRecord,
                        format!(
                            "terminal authored artifact differs from retained authority: {relative}"
                        ),
                    ));
                }
            }
            return Ok(existing);
        }
        let authored_artifacts = [record.design_path.clone(), record.diagram_path.clone()]
            .into_iter()
            .map(|path| {
                let bytes = read_regular_terminal_artifact(&self.root, Path::new(&path))?
                    .ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "terminal authored artifact is absent",
                        )
                    })?;
                let contents = String::from_utf8(bytes).map_err(|_| {
                    V2Error::new(
                        ErrorCode::CorruptRecord,
                        "terminal authored artifact is not UTF-8",
                    )
                })?;
                Ok((path, contents))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        verify_cards(self, &record, &cards)?;
        let terminal = record.terminal.as_mut().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidTransition, "terminal evidence missing")
        })?;
        if terminal.receipt_path != receipt_ref {
            terminal.receipt_path = receipt_ref.clone();
            record.generation += 1;
            for values in cards.values_mut() {
                values.identity.generation = record.generation;
            }
            record.audit.push(AuditEvent {
                sequence: record.audit.len() as u64 + 1,
                generation: record.generation,
                actor: "csdlc-closeout".into(),
                reason: "normalize legacy terminal receipt path to portable reference".into(),
                operation: "normalize_terminal_receipt_ref".into(),
            });
            hydrate_projections(&mut record, &cards)?;
            record.digest = record_digest(&record)?;
            self.commit(issue, &record, &cards, false)?;
        }
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue,
            repository: record.repository.clone(),
            initialization_digest: record.initialization_digest.clone(),
            receipt_ref,
            authored_artifacts,
            record,
            cards,
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let mut encoded = serde_json::to_vec_pretty(&receipt)?;
        encoded.push(b'\n');
        replace_regular_terminal_artifact(&common, &receipt_relative, &encoded, "json.retain-tmp")?;
        let retained = self.load_terminal_receipt(issue)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "newly retained terminal receipt is absent",
            )
        })?;
        if retained != receipt {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "newly retained terminal receipt differs from committed authority",
            ));
        }
        Ok(retained)
    }

    pub fn reconcile_terminal(&self, request: ReconcileTerminalRequest) -> Result<IssueRecord> {
        if request.actor.trim().is_empty() || request.reason.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "reconciliation actor and reason are required",
            ));
        }
        if request.expected_branch.trim().is_empty()
            || request.expected_worktree.trim().is_empty()
            || request.expected_branch == "main"
            || crate::git::current_branch(&self.root)? != request.expected_branch
            || self.root.canonicalize()? != Path::new(&request.expected_worktree).canonicalize()?
        {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal reconciliation requires the declared dedicated branch and worktree",
            ));
        }
        let _terminal_repair_lock = self.terminal_repair_lock()?;
        let _lock = self.lock(request.issue)?;
        self.recover_with_terminal_lock(request.issue)?;
        let mut receipt = self
            .load_terminal_receipt(request.issue)?
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "terminal receipt missing"))?;
        if receipt.issue != request.issue {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal receipt issue differs from reconciliation request",
            ));
        }
        if receipt.initialization_digest != request.expected_initialization_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal receipt initialization digest differs from reconciliation request",
            ));
        }
        let issue_dir = self.issue_dir(request.issue);
        let local = match self.load_record(request.issue) {
            Ok(local) => {
                if receipt.initialization_digest != local.initialization_digest
                    || receipt.repository != local.repository
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "terminal receipt identity differs from local issue",
                    ));
                }
                local
            }
            Err(error) if error.code == ErrorCode::Io && !issue_dir.exists() => {
                receipt.record.clone()
            }
            Err(error) => return Err(error),
        };
        let requested_follow_ups = request
            .follow_ups
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        if requested_follow_ups.len() != request.follow_ups.len() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal follow-ups must be non-empty and unique",
            ));
        }
        let existing_follow_ups = match receipt
            .cards
            .get(&CardKind::Sor)
            .map(|values| &values.content)
        {
            Some(CardContent::Sor(values)) => {
                values.follow_ups.iter().cloned().collect::<BTreeSet<_>>()
            }
            _ => BTreeSet::new(),
        };
        let local_cards = match self.load_cards(request.issue) {
            Ok(cards) => Some(cards),
            Err(_) if !issue_dir.exists() => None,
            Err(error) => return Err(error),
        };
        let local_integrity = (|| -> Result<bool> {
            verify_record(&local)?;
            let Some(current_cards) = local_cards.as_ref() else {
                return Ok(false);
            };
            let mut checked = local.clone();
            hydrate_projections(&mut checked, current_cards)?;
            Ok(checked.digest == record_digest(&checked)? && checked.digest == local.digest)
        })()?;
        if local.phase == LifecyclePhase::ClosedOut
            && local == receipt.record
            && local.terminal == receipt.record.terminal
            && local.publication.as_ref().is_some_and(|publication| {
                !publication.draft && publication.observed_state == "merged"
            })
            && existing_follow_ups == requested_follow_ups
            && local_integrity
            && local.audit.last().is_some_and(|event| {
                event.operation == "reconcile_terminal"
                    && event.actor == request.actor
                    && event.reason == request.reason
            })
        {
            return Ok(local);
        }
        // A complete, valid tracked terminal projection may contain newer
        // append-only audit provenance than the machine-local receipt. Preserve
        // that history and refresh the receipt from it; use the receipt as the
        // recovery authority only when the tracked projection is absent,
        // incomplete, or invalid.
        let local_cards_match_receipt_semantics = local_cards.as_ref().is_some_and(|values| {
            let mut local_values = values.clone();
            let mut receipt_values = receipt.cards.clone();
            for card in local_values.values_mut() {
                card.identity.generation = 0;
            }
            for card in receipt_values.values_mut() {
                card.identity.generation = 0;
            }
            local_values == receipt_values
        });
        let prefer_local = local_integrity
            && local.phase == LifecyclePhase::ClosedOut
            && local.claim.is_none()
            && local.generation >= receipt.record.generation
            && local.terminal == receipt.record.terminal
            && local_cards_match_receipt_semantics;
        let (mut projection, mut cards) = if prefer_local {
            (local.clone(), local_cards.expect("validated local cards"))
        } else {
            (receipt.record, receipt.cards)
        };
        if local_integrity
            && local.phase == LifecyclePhase::ClosedOut
            && local.claim.is_none()
            && local.terminal == projection.terminal
        {
            for (retained, tracked) in projection.audit.iter_mut().zip(local.audit.iter()) {
                if retained.sequence != tracked.sequence
                    || retained.generation != tracked.generation
                    || retained.operation != tracked.operation
                {
                    break;
                }
                // Sequence, generation, and operation identify the same durable
                // event. Preserve the tracked actor/reason provenance when an
                // older machine-local receipt retained different attribution.
                *retained = tracked.clone();
            }
        }
        if let (Some(publication), Some(terminal)) = (
            projection.publication.as_mut(),
            projection.terminal.as_ref(),
        ) {
            if terminal.disposition == crate::readiness::TerminalDisposition::Merged
                && terminal.pull_request == Some(publication.pull_request)
                && terminal.observed_state == "merged"
            {
                publication.draft = false;
                publication.observed_state = "merged".into();
            }
        }
        let current_review_passes = cards.get(&CardKind::Srp).is_some_and(|card| {
            matches!(&card.content, CardContent::Srp(srp)
            if srp.review_result == crate::cards::ReviewResult::Pass
                && srp.review_revision.as_deref().is_some_and(|value| !value.is_empty())
                && srp.reviewer.as_deref().is_some_and(|value| !value.is_empty())
                && !srp.findings.iter().any(|finding| {
                    finding.actionable
                        && finding.disposition == crate::cards::FindingDisposition::Open
                }))
        });
        if projection
            .review
            .as_ref()
            .is_some_and(|review| review.completed)
            && current_review_passes
        {
            cards.get_mut(&CardKind::Srp).expect("SRP card").status =
                crate::cards::CardStatus::Complete;
        }
        let routed = match cards.get(&CardKind::Srp).map(|values| &values.content) {
            Some(CardContent::Srp(values)) => values
                .residual_risk
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>(),
            _ => BTreeSet::new(),
        };
        if !requested_follow_ups.is_subset(&routed) {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "terminal follow-ups must be routed by SRP residual risk",
            ));
        }
        let design = receipt
            .authored_artifacts
            .get(&projection.design_path)
            .expect("validated receipt design")
            .clone();
        let diagram = receipt
            .authored_artifacts
            .get(&projection.diagram_path)
            .expect("validated receipt diagram")
            .clone();
        let design_path = format!(".csdlc/issues/{}/retained/design.md", request.issue);
        let diagram_path = format!(".csdlc/issues/{}/retained/diagram.mmd", request.issue);
        projection.design_path = design_path.clone();
        projection.diagram_path = diagram_path.clone();
        for kind in [CardKind::Spp, CardKind::Vpp] {
            match &mut cards.get_mut(&kind).expect("design card").content {
                CardContent::Spp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                CardContent::Vpp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                _ => unreachable!("design card"),
            }
        }
        if !requested_follow_ups.is_empty() {
            match &mut cards.get_mut(&CardKind::Sor).expect("SOR card").content {
                CardContent::Sor(values) => {
                    values.follow_ups = requested_follow_ups.into_iter().collect();
                }
                _ => unreachable!("SOR card"),
            }
        }
        projection.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = projection.generation;
        }
        projection.audit.push(AuditEvent {
            sequence: projection.audit.len() as u64 + 1,
            generation: projection.generation,
            actor: request.actor,
            reason: request.reason,
            operation: "reconcile_terminal".into(),
        });
        validate_cross_card(
            &cards,
            &design_path,
            &digest(design.as_bytes()),
            &diagram_path,
            &digest(diagram.as_bytes()),
        )?;
        hydrate_projections(&mut projection, &cards)?;
        projection.digest = record_digest(&projection)?;
        let retained_artifacts = BTreeMap::from([(design_path, design), (diagram_path, diagram)]);
        let receipt_path = self.terminal_receipt_path(request.issue)?;
        let receipt_parent = receipt_path.parent().expect("receipt parent");
        let receipt_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(receipt_parent.join("receipts.lock"))?;
        receipt_lock.lock_exclusive()?;
        let original_receipt = fs::read(&receipt_path)?;
        drop(receipt_lock);
        receipt.record = projection.clone();
        receipt.cards = cards.clone();
        receipt.authored_artifacts = retained_artifacts.clone();
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt)?;
        validate_terminal_receipt(&receipt)?;
        let target_receipt = serde_json::to_vec_pretty(&receipt)?;
        let mut journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: request.issue,
            stage: "prepared".into(),
            original_record_digest: Some(local.digest.clone()),
            original_projection_digest: None,
            target_record_digest: projection.digest.clone(),
            original_receipt: Some(original_receipt),
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt,
        };
        Self::maybe_interrupt_terminal_transaction(request.issue, "before_journal")?;
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_journal")?;
        if let Err(error) = self.commit_with_authored(
            request.issue,
            &projection,
            &cards,
            false,
            Some(&retained_artifacts),
        ) {
            if !matches!(&error.code, ErrorCode::InterruptedTransaction) {
                let _ = self.recover_terminal_transaction(request.issue);
            }
            return Err(error);
        }
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_projection")?;
        journal.stage = "projection_committed".into();
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_projection_journal")?;
        let refresh = (|| -> Result<()> {
            let parent = receipt_path.parent().expect("receipt parent");
            let lock = OpenOptions::new()
                .create(true)
                .truncate(false)
                .read(true)
                .write(true)
                .open(parent.join("receipts.lock"))?;
            lock.lock_exclusive()?;
            let temporary = receipt_path.with_extension("json.reconcile-tmp");
            let mut file = File::create(&temporary)?;
            file.write_all(&journal.target_receipt)?;
            file.sync_all()?;
            Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_write")?;
            fs::rename(temporary, &receipt_path)?;
            sync_dir(parent)?;
            Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_rename")?;
            Ok(())
        })();
        if let Err(error) = refresh {
            if !matches!(&error.code, ErrorCode::InterruptedTransaction) {
                let _ = self.recover_terminal_transaction(request.issue);
            }
            return Err(error);
        }
        journal.stage = "receipt_committed".into();
        self.write_terminal_transaction_journal(&journal)?;
        Self::maybe_interrupt_terminal_transaction(request.issue, "after_receipt_journal")?;
        self.remove_terminal_transaction_journal(request.issue)?;
        Ok(projection)
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
        self.recover_local_transaction(issue)?;
        if self.terminal_transaction_path(issue)?.is_file() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal transaction recovery requires the shared terminal lock",
            ));
        }
        Ok(())
    }

    fn recover_with_terminal_lock(&self, issue: u64) -> Result<()> {
        self.recover_local_transaction(issue)?;
        self.recover_terminal_transaction(issue)?;
        Ok(())
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
                replace_regular_terminal_artifact(
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

    pub(crate) fn replace_authority_record(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "record changed before compare-and-swap authority commit",
            ));
        }
        let cards = self.load_cards(issue)?;
        // Authority recovery accepts projection drift only when the typed card
        // values, identities, generations, and rendered Markdown agree.
        verify_authority_card_inputs(self, &current, &cards)?;
        let mut repaired = record.clone();
        hydrate_projections(&mut repaired, &cards)?;
        repaired.digest = record_digest(&repaired)?;
        self.commit(issue, &repaired, &cards, false)?;
        Ok(repaired)
    }

    pub(crate) fn replace_authority_projection_locked(
        &self,
        issue: u64,
        expected_digest: &str,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<IssueRecord> {
        self.recover_if_needed(issue)?;
        let current = self.load_record(issue)?;
        if current.digest != expected_digest {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "projection changed before compare-and-swap authority materialization",
            ));
        }
        let current_cards = self.load_cards(issue)?;
        verify_cards(self, &current, &current_cards)?;
        verify_canonical_projection_bytes(self, &current, &current_cards)?;
        let mut materialized = record.clone();
        hydrate_projections(&mut materialized, cards)?;
        materialized.digest = record_digest(&materialized)?;
        self.commit(issue, &materialized, cards, false)?;
        if let Err(error) = verify_cards(self, &materialized, cards) {
            self.commit(issue, &current, &current_cards, false)?;
            verify_cards(self, &current, &current_cards)?;
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!(
                    "authority projection failed post-commit verification and was rolled back: {}",
                    error.message
                ),
            ));
        }
        Ok(materialized)
    }

    pub(crate) fn verify_canonical_authority_projection(
        &self,
        record: &IssueRecord,
        cards: &BTreeMap<CardKind, CardValues>,
    ) -> Result<()> {
        verify_canonical_projection_bytes(self, record, cards)
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
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
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

    pub(crate) fn commit_publication(
        &self,
        issue: u64,
        expected_digest: &str,
        claim_id: &str,
        actor: String,
        evidence: PublicationEvidence,
        merged: bool,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(issue)?;
        self.recover_if_needed(issue)?;
        let mut record = self.load_record(issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
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
        sor.integration_state = if merged {
            crate::cards::IntegrationState::Merged
        } else {
            crate::cards::IntegrationState::PrOpen
        };
        sor.merge_state = if merged {
            crate::cards::MergeState::Merged
        } else {
            crate::cards::MergeState::NotMerged
        };
        sor.publication_state = if evidence.draft {
            crate::cards::PublicationState::Draft
        } else {
            crate::cards::PublicationState::Ready
        };
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.publication = Some(evidence);
        if record.phase == LifecyclePhase::Reviewed {
            record.advance(
                LifecyclePhase::Published,
                actor.clone(),
                if merged {
                    "observed exact merged PR after current review"
                } else {
                    "observed exact PR after current review"
                }
                .into(),
            )?;
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor,
            reason: if merged {
                "atomically record observed merged GitHub publication and SOR projection"
            } else {
                "atomically record observed GitHub publication and SOR projection"
            }
            .into(),
            operation: if merged {
                "record_merged_publication"
            } else {
                "record_publication"
            }
            .into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_ready_publication(
        &self,
        request: &crate::publication::ReadyPublicationRequest,
        evidence: PublicationEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(request.issue)?;
        self.recover_if_needed(request.issue)?;
        let mut record = self.load_record(request.issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&request.claim_id, now_seconds()?)?;
        if record.generation != request.expected_generation
            || record.digest != request.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "ready publication record changed before commit",
            ));
        }
        if record.phase != LifecyclePhase::Published
            || record.publication.as_ref().is_none_or(|publication| {
                publication.repository != request.repository
                    || publication.pull_request != request.pull_request
                    || !publication.draft
            })
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical publication is no longer the governed draft",
            ));
        }
        let publication = record.publication.as_ref().expect("publication checked");
        let observed_revision = crate::git::clean_commit_revision(&request.expected_head_sha);
        if publication.revision != observed_revision {
            let Some(from_commit) = publication
                .revision
                .strip_prefix("git-blake3:")
                .and_then(|value| value.split(':').next())
            else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "canonical publication revision is invalid",
                ));
            };
            let changed = crate::git::metadata_only_changed_paths(
                &self.root,
                from_commit,
                &request.expected_head_sha,
            )
            .map_err(|_| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "ready head is not a forward metadata-only publication revision",
                )
            })?;
            if changed.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "ready head changed without typed publication metadata",
                ));
            }
        }
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(values) => values,
            _ => unreachable!("SOR"),
        };
        sor.publication_state = crate::cards::PublicationState::Ready;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.publication = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.actor.clone(),
            reason: "record exact existing PR ready-for-review after remote success".into(),
            operation: "record_ready_publication".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(request.issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_readiness(
        &self,
        request: crate::readiness::ReadinessRequest,
        evidence: ReadinessEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(request.issue)?;
        self.recover_if_needed(request.issue)?;
        let mut record = self.load_record(request.issue)?;
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&request.claim_id, now_seconds()?)?;
        if record.generation != request.expected_generation
            || record.digest != request.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "readiness request does not match canonical record",
            ));
        }
        if !matches!(
            record.phase,
            LifecyclePhase::Published | LifecyclePhase::MergeReady
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "readiness requires published state",
            ));
        }
        let publication = record.publication.as_ref().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidTransition, "publication evidence missing")
        })?;
        if publication.pull_request != request.pull_request {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "readiness observation does not match published PR revision",
            ));
        }
        let observed_revision = crate::git::clean_commit_revision(&request.head_sha);
        let publication_revision_reconciled = if publication.revision != observed_revision {
            let Some(from_commit) = publication
                .revision
                .strip_prefix("git-blake3:")
                .and_then(|value| value.split(':').next())
            else {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published PR revision is not a clean commit identity",
                ));
            };
            let changed_paths =
                crate::git::metadata_only_changed_paths(&self.root, from_commit, &request.head_sha)
                    .map_err(|_| {
                        V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            "readiness observation does not match published PR revision",
                        )
                    })?;
            if changed_paths.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published PR revision changed without typed metadata delta",
                ));
            }
            true
        } else {
            false
        };
        if record.readiness.as_ref() == Some(&evidence) {
            return Ok(record);
        }
        let mut cards = self.load_cards(request.issue)?;
        verify_cards(self, &record, &cards)?;
        if publication_revision_reconciled {
            record.publication.as_mut().expect("publication").revision = observed_revision;
        }
        let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        if evidence.ready {
            let validation_ready = terminal_validation_passed(&sor.actual_validation);
            if !validation_ready {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "merge readiness requires passing local PVF evidence",
                ));
            }
            sor.publication_state = crate::cards::PublicationState::Ready;
            if record.phase == LifecyclePhase::Published {
                record.advance(
                    LifecyclePhase::MergeReady,
                    request.actor.clone(),
                    "observed required checks, review, and conflict readiness".into(),
                )?;
            }
        } else {
            sor.publication_state = crate::cards::PublicationState::Draft;
            if record.phase == LifecyclePhase::MergeReady {
                record.advance(
                    LifecyclePhase::Published,
                    request.actor.clone(),
                    "latest remote observation revoked merge readiness".into(),
                )?;
            }
        }
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
        }
        record.readiness = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: request.actor,
            reason: "record normalized remote readiness without replacing pre-publication review"
                .into(),
            operation: if publication_revision_reconciled {
                "record_readiness_reconcile_metadata_only_published_revision"
            } else {
                "record_readiness"
            }
            .into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(request.issue, &record, &cards, false)?;
        Ok(record)
    }

    pub(crate) fn commit_terminal(
        &self,
        observation: crate::readiness::TerminalObservation,
        mut evidence: TerminalEvidence,
    ) -> Result<IssueRecord> {
        let _lock = self.lock(observation.issue)?;
        self.recover_if_needed(observation.issue)?;
        let mut record = self.load_record(observation.issue)?;
        if let Some(current) = &record.terminal {
            if record.phase == LifecyclePhase::ClosedOut
                && current.pull_request == evidence.pull_request
                && current.disposition == evidence.disposition
                && current.observed_sha == evidence.observed_sha
                && current.observed_state == evidence.observed_state
            {
                return Ok(record);
            }
        }
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&observation.claim_id, now_seconds()?)?;
        if record.generation != observation.expected_generation
            || record.digest != observation.expected_digest
        {
            return Err(V2Error::new(
                ErrorCode::StaleDigest,
                "terminal observation does not match canonical record",
            ));
        }
        if !crate::readiness::terminal_phase_allowed(record.phase, observation.disposition) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal disposition is not valid from the current lifecycle phase",
            ));
        }
        match (
            &record.publication,
            observation.pull_request,
            observation.observed_sha.as_deref(),
        ) {
            (Some(publication), Some(pr), Some(sha)) => {
                if publication.pull_request != pr
                    || publication.revision != crate::git::clean_commit_revision(sha)
                {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "terminal PR or SHA differs from exact publication evidence",
                    ));
                }
            }
            (Some(_), None, _) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published issue cannot use no-PR closeout",
                ));
            }
            (Some(_), Some(_), None) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "published terminal observation is missing the exact head SHA",
                ));
            }
            (None, Some(_), _) => {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "terminal PR has no canonical publication evidence",
                ));
            }
            _ => {}
        }
        let mut cards = self.load_cards(observation.issue)?;
        verify_cards(self, &record, &cards)?;
        let current_validation = match &cards[&CardKind::Sor].content {
            CardContent::Sor(value) => &value.actual_validation,
            _ => unreachable!(),
        };
        if !terminal_validation_passed(current_validation) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "terminal closeout requires current passing validation evidence",
            ));
        }
        let sor_values = cards.get_mut(&CardKind::Sor).expect("SOR");
        sor_values.status = crate::cards::CardStatus::Complete;
        let sor = match &mut sor_values.content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        sor.publication_state = crate::cards::PublicationState::Closed;
        sor.closeout_state = crate::cards::CloseoutState::Complete;
        match observation.disposition {
            crate::readiness::TerminalDisposition::Merged => {
                sor.integration_state = crate::cards::IntegrationState::Merged;
                sor.merge_state = crate::cards::MergeState::Merged;
                record.advance(
                    LifecyclePhase::Merged,
                    observation.actor.clone(),
                    "observed exact PR merged".into(),
                )?;
                record.advance(
                    LifecyclePhase::ClosedOut,
                    observation.actor.clone(),
                    "terminal truth recorded and claim released".into(),
                )?;
            }
            crate::readiness::TerminalDisposition::ClosedUnmerged
            | crate::readiness::TerminalDisposition::ClosedNoPr => {
                sor.integration_state = crate::cards::IntegrationState::ClosedNoPr;
                sor.merge_state = crate::cards::MergeState::ClosedUnmerged;
                let from = record.phase;
                record.phase = LifecyclePhase::ClosedOut;
                record.transitions.push(TransitionEvent {
                    sequence: record.transitions.len() as u64 + 1,
                    from,
                    to: LifecyclePhase::ClosedOut,
                    actor: observation.actor.clone(),
                    reason: "observed approved non-merged terminal disposition".into(),
                });
            }
        }
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        let released = record.claim.take().expect("validated claim");
        evidence.released_branch = released.branch;
        evidence.released_worktree = released.worktree;
        evidence.released_protected_paths = released.protected_paths;
        record.terminal = Some(evidence);
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: observation.actor,
            reason: "atomically finalize SOR/index and release claim/protected paths".into(),
            operation: "record_terminal".into(),
        });
        validate_updated_cards(self, &record, &cards)?;
        hydrate_projections(&mut record, &cards)?;
        record.digest = record_digest(&record)?;
        self.commit(observation.issue, &record, &cards, false)?;
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
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&commit.claim_id, now_seconds()?)?;
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
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
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
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(&commit.claim_id, now_seconds()?)?;
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
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
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
        claim_id: &str,
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
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
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
        claim_id: &str,
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
        record
            .claim
            .as_ref()
            .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
            .validate(claim_id, now_seconds()?)?;
        if !matches!(
            record.phase,
            LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
        ) {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "review recovery requires reviewed phase",
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
        record.advance(LifecyclePhase::Implemented, actor.clone(), reason.clone())?;
        record.review_assignment = None;
        record.review = None;
        record.publication = None;
        record.readiness = None;
        record.terminal = None;
        record.generation += 1;
        for values in cards.values_mut() {
            values.identity.generation = record.generation;
        }
        if let Some(claim) = record.claim.as_mut() {
            claim.generation = record.generation;
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BootstrapRequest {
    pub issue: u64,
    pub repository: String,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    #[serde(default)]
    pub design_approved: bool,
    pub claim: Claim,
    pub initial: InitialCardInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EditRequest {
    pub issue: u64,
    pub card: CardKind,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
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
    pub claim_id: String,
    pub reviewer: String,
}

pub fn approve_design(store: &Store, request: ApproveDesignRequest) -> Result<IssueRecord> {
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
    record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "claim missing"))?
        .validate(&request.claim_id, now_seconds()?)?;
    let mut cards = store.load_cards(request.issue)?;
    verify_card_projections(store, &record, &cards)?;
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
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
    let lifecycle_reapproval = matches!(
        record.phase,
        LifecyclePhase::Bound | LifecyclePhase::Implemented
    );
    if !initial_approval && !initialized_reapproval && !lifecycle_reapproval {
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
    if let Some(claim) = record.claim.as_mut() {
        claim.generation = record.generation;
    }
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.reviewer,
        reason: if initialized_reapproval {
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
    let bootstrap_actor = request.claim.owner.clone();
    let design_digest = digest(&fs::read(store.root.join(&request.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&request.diagram_path))?);
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
        initialization_digest,
        phase: LifecyclePhase::Initialized,
        generation: 0,
        digest: String::new(),
        claim: Some(request.claim),
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
    let now = now_seconds()?;
    if (request.design_approved && request.design_reviewer.trim().is_empty())
        || request.claim.id.trim().is_empty()
        || request.claim.owner.trim().is_empty()
        || request.claim.purpose.trim().is_empty()
        || request.claim.branch.trim().is_empty()
        || request.claim.worktree.trim().is_empty()
        || request.claim.generation != 0
        || request.claim.protected_paths.is_empty()
        || request.claim.heartbeat_unix_seconds < request.claim.acquired_unix_seconds
        || request.claim.expires_unix_seconds <= request.claim.heartbeat_unix_seconds
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bootstrap claim/reviewer invariants are incomplete",
        ));
    }
    request.claim.validate(&request.claim.id, now)?;
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
    let now = now_seconds()?;
    let claim = record
        .claim
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::MissingClaim, "mutation requires a claim"))?;
    claim.validate(&request.claim_id, now)?;
    if claim.generation != record.generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "claim generation is stale",
        ));
    }
    let mut cards = store.load_cards(request.issue)?;
    verify_cards(store, &record, &cards)?;
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
    let replan_before = match &request.operation {
        SemanticOperation::Replan { field, .. } => Some(current_text_value(
            cards
                .get(&request.card)
                .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "card projection missing"))?,
            *field,
        )?),
        _ => None,
    };
    let audit_operation = match (&request.operation, replan_before) {
        (SemanticOperation::Replan { field, value }, Some(previous)) => serde_json::json!({
            "operation": "replan",
            "field": field.as_ref(),
            "previous_value": previous,
            "new_value": value,
        })
        .to_string(),
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
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
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
    if let Some(claim) = record.claim.as_mut() {
        claim.generation = record.generation;
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
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
    validate_cross_card(
        cards,
        &record.design_path,
        &design_digest,
        &record.diagram_path,
        &diagram_digest,
    )?;
    Ok(())
}

fn verify_card_projections(
    store: &Store,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, CardValues>,
) -> Result<()> {
    verify_authority_card_inputs(store, record, cards)?;
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
) -> Result<()> {
    verify_record(record)?;
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
    if record.phase == LifecyclePhase::ClosedOut {
        if record.claim.is_some() || record.terminal.is_none() {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "closed-out record must have terminal evidence and no active claim",
            ));
        }
    } else if let Some(claim) = record.claim.as_ref() {
        if claim.generation != record.generation
            || claim.id.is_empty()
            || claim.owner.is_empty()
            || claim.protected_paths.is_empty()
            || claim.branch.is_empty()
            || claim.worktree.is_empty()
            || claim.heartbeat_unix_seconds < claim.acquired_unix_seconds
            || claim.expires_unix_seconds <= claim.heartbeat_unix_seconds
        {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "claim invariant failed",
            ));
        }
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
        if event.sequence != index as u64 + 1
            || event.from != phase
            || (!event.from.allows(event.to) && !direct_recordless_closeout)
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
    let validation_passed = terminal_validation_passed(&sor.actual_validation);
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
    if next == LifecyclePhase::MergeReady
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
            || !validation_passed
            || sor.publication_state != crate::cards::PublicationState::Ready)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "merge readiness requires current review, passing evidence, and ready publication",
        ));
    }
    if next == LifecyclePhase::Merged && sor.merge_state != crate::cards::MergeState::Merged {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "merged phase requires observed merged state",
        ));
    }
    if next == LifecyclePhase::ClosedOut
        && (sor.closeout_state != crate::cards::CloseoutState::Complete
            || !matches!(
                sor.integration_state,
                crate::cards::IntegrationState::Merged | crate::cards::IntegrationState::ClosedNoPr
            )
            || !matches!(
                sor.merge_state,
                crate::cards::MergeState::Merged | crate::cards::MergeState::ClosedUnmerged
            )
            || cards[&CardKind::Sor].status != crate::cards::CardStatus::Complete)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "closeout phase requires terminal SOR truth",
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
            CardKind::Sip | CardKind::Stp | CardKind::Spp | CardKind::Vpp,
            SemanticOperation::SetField { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
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
            SemanticOperation::UpdatePlanStep { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Spp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::AffectedAreas,
                ..
            },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Sip,
            SemanticOperation::ReplaceOperatorConstraints { .. }
                | SemanticOperation::ReplacePlanningCollection {
                    field: crate::cards::PlanningCollectionField::AuthorityBoundary,
                    ..
                },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria { .. },
        ) | (
            LifecyclePhase::Implemented,
            CardKind::Srp,
            SemanticOperation::ReplacePlanningCollection {
                field: crate::cards::PlanningCollectionField::ReviewPrompts,
                ..
            },
        ) | (
            LifecyclePhase::Bound | LifecyclePhase::Implemented,
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
        ) | (
            LifecyclePhase::MergeReady,
            CardKind::Sor,
            SemanticOperation::RecordMerge { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. },
        ) | (
            LifecyclePhase::MergeReady,
            CardKind::Srp,
            SemanticOperation::RecordFinding { .. } | SemanticOperation::DisposeFinding { .. },
        ) | (
            LifecyclePhase::Merged,
            CardKind::Sor,
            SemanticOperation::RecordCloseout { .. }
                | SemanticOperation::RecordValidation { .. }
                | SemanticOperation::AppendReference { .. }
                | SemanticOperation::AdvanceStatus { .. },
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
    let design_digest = digest(&fs::read(store.root.join(&record.design_path))?);
    let diagram_digest = digest(&fs::read(store.root.join(&record.diagram_path))?);
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
        || receipt.record.claim.is_some()
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

fn issue_projection_paths(issue: u64) -> Vec<PathBuf> {
    let issue_dir = PathBuf::from(".csdlc/issues").join(issue.to_string());
    let mut paths = vec![issue_dir.join("index.json"), issue_dir.join("audit.jsonl")];
    for kind in enum_iterator() {
        paths.push(issue_dir.join(format!("cards/{kind}.values.json")));
        paths.push(issue_dir.join(format!("cards/{kind}.md")));
    }
    paths
}

fn projection_snapshot_digest(snapshot: &BTreeMap<String, Option<Vec<u8>>>) -> String {
    let mut hasher = blake3::Hasher::new();
    for (path, bytes) in snapshot {
        hasher.update(path.as_bytes());
        hasher.update(&[0]);
        match bytes {
            Some(bytes) => {
                hasher.update(&[1]);
                hasher.update(&(bytes.len() as u64).to_le_bytes());
                hasher.update(bytes);
            }
            None => {
                hasher.update(&[0]);
            }
        }
    }
    hasher.finalize().to_hex().to_string()
}

fn snapshot_regular_tree(
    root: &Path,
    relative: &Path,
    snapshot: &mut BTreeMap<String, Option<Vec<u8>>>,
) -> Result<()> {
    let metadata = canonical_path_metadata_beneath(root, relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "corrupt recovery issue projection is absent",
        )
    })?;
    if metadata.is_file() {
        snapshot.insert(
            relative.to_string_lossy().into_owned(),
            Some(fs::read(root.join(relative))?),
        );
        return Ok(());
    }
    if !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "corrupt recovery tree contains a non-regular entry: {}",
                root.join(relative).display()
            ),
        ));
    }
    let mut children = fs::read_dir(root.join(relative))?
        .map(|entry| entry.map(|entry| entry.file_name()))
        .collect::<std::io::Result<Vec<_>>>()?;
    children.sort();
    for child in children {
        snapshot_regular_tree(root, &relative.join(child), snapshot)?;
    }
    Ok(())
}

fn git_blob(root: &Path, commit: &str, path: &str) -> Result<Vec<u8>> {
    if commit.len() != 40
        || !commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        || !crate::pvf::clean_relative(Path::new(path))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "historical source commit or path is invalid",
        ));
    }
    let object = format!("{commit}:{path}");
    let output = Command::new("git")
        .current_dir(root)
        .args(["show", "--no-ext-diff", &object])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "historical source blob is unavailable: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    Ok(output.stdout)
}

fn git_is_ancestor(root: &Path, ancestor: &str, descendant: &str) -> Result<bool> {
    let status = Command::new("git")
        .current_dir(root)
        .args(["merge-base", "--is-ancestor", ancestor, descendant])
        .status()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    match status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(V2Error::new(
            ErrorCode::GitFailure,
            "git merge-base failed while validating historical source ancestry",
        )),
    }
}

fn require_canonical_issue_projection_components(root: &Path, issue: u64) -> Result<()> {
    for relative in issue_projection_paths(issue) {
        if let Some(metadata) = canonical_path_metadata_beneath(root, &relative)? {
            if !metadata.is_file() {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    format!(
                        "terminal authority projection is not a regular file: {}",
                        root.join(relative).display()
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn require_canonical_issue_projection_files(root: &Path, issue: u64) -> Result<()> {
    for relative in issue_projection_paths(issue) {
        let metadata = canonical_path_metadata_beneath(root, &relative)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!(
                    "terminal authority projection is absent: {}",
                    root.join(&relative).display()
                ),
            )
        })?;
        if !metadata.is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                format!(
                    "terminal authority projection is not a regular file: {}",
                    root.join(relative).display()
                ),
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

fn read_regular_terminal_artifact(root: &Path, relative: &Path) -> Result<Option<Vec<u8>>> {
    if !crate::pvf::clean_relative(relative) {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "terminal authored artifact path must be clean and repository-relative",
        ));
    }
    let path = root.join(relative);
    let Some(metadata) = canonical_path_metadata_beneath(root, relative)? else {
        return Ok(None);
    };
    if !metadata.is_file() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "transport target authored path is not a regular file: {}",
                path.display()
            ),
        ));
    }
    Ok(Some(fs::read(path)?))
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
            _ => unreachable!("clean_relative accepted a non-normal component"),
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

fn require_canonical_parent_beneath(root: &Path, relative: &Path) -> Result<()> {
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

fn require_regular_or_absent_beneath(root: &Path, relative: &Path) -> Result<()> {
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

fn replace_regular_terminal_artifact(
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

pub(crate) fn now_seconds() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))
}

fn enum_iterator() -> impl Iterator<Item = CardKind> {
    use strum::IntoEnumIterator;
    CardKind::iter()
}

fn complete_terminal_plan_step(
    cards: &mut BTreeMap<CardKind, CardValues>,
    step_id: &str,
) -> Result<()> {
    let spp = match &mut cards.get_mut(&CardKind::Spp).expect("SPP").content {
        CardContent::Spp(values) => values,
        _ => unreachable!("SPP card content"),
    };
    let step = spp
        .steps
        .iter_mut()
        .find(|step| step.id == step_id)
        .ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "terminal plan step does not exist")
        })?;
    complete_step_status(&mut step.status)
}

fn replace_terminal_sor_artifact(
    cards: &mut BTreeMap<CardKind, CardValues>,
    stale_ref: &str,
    retained_ref: &str,
) -> Result<()> {
    let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR card content"),
    };
    let stale_count = sor
        .artifacts
        .iter()
        .filter(|artifact| artifact.as_str() == stale_ref)
        .count();
    if stale_count != 1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR artifact repair requires exactly one stale reference",
        ));
    }
    if sor
        .artifacts
        .iter()
        .any(|artifact| artifact == retained_ref)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal SOR artifact replacement is already present",
        ));
    }
    *sor.artifacts
        .iter_mut()
        .find(|artifact| artifact.as_str() == stale_ref)
        .expect("count checked") = retained_ref.to_owned();
    Ok(())
}

fn replace_terminal_sor_validation(
    cards: &mut BTreeMap<CardKind, CardValues>,
    expected: &ValidationResult,
    replacement: &ValidationResult,
) -> Result<()> {
    let sor = match &mut cards.get_mut(&CardKind::Sor).expect("SOR").content {
        CardContent::Sor(values) => values,
        _ => unreachable!("SOR card content"),
    };
    let matches: Vec<_> = sor
        .actual_validation
        .iter_mut()
        .filter(|result| *result == expected)
        .collect();
    if matches.len() != 1 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR validation repair requires exactly one expected result",
        ));
    }
    *matches.into_iter().next().expect("one match") = replacement.clone();
    Ok(())
}

fn validate_portable_validation_result(result: &ValidationResult) -> Result<()> {
    if result
        .command
        .iter()
        .any(|part| contains_machine_local_path(part, true))
        || contains_machine_local_path(&result.evidence_ref, false)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "terminal SOR validation replacement contains a machine-local path",
        ));
    }
    Ok(())
}

fn contains_machine_local_path(value: &str, shell_context: bool) -> bool {
    if value.to_ascii_lowercase().contains("file://")
        || contains_shell_expansion(value)
        || (shell_context && value.contains('`'))
        || contains_backtick_path_expansion(value)
        || contains_windows_environment_expansion(value)
    {
        return true;
    }
    value.split_whitespace().any(|word| {
        if word.starts_with("http://") || word.starts_with("https://") {
            return false;
        }
        word.split(['=', '[', '(', '{', ',', ';', '>', '<', '|', '&'])
            .any(|segment| {
                let candidate = segment.trim_matches(|character: char| {
                    matches!(character, '\'' | '"' | ')' | ']' | '}')
                });
                candidate.starts_with('/')
                    || candidate.starts_with("~/")
                    || candidate.starts_with("~\\")
                    || candidate.starts_with("\\\\")
                    || candidate.starts_with("//")
                    || is_windows_absolute_path(candidate)
            })
    })
}

fn contains_shell_expansion(value: &str) -> bool {
    value.char_indices().any(|(index, character)| {
        if character != '$' {
            return false;
        }
        let suffix = &value[index + character.len_utf8()..];
        if suffix.starts_with(['(', '{']) {
            return true;
        }
        let boundary = value[..index].chars().next_back().is_none_or(|previous| {
            previous.is_whitespace() || matches!(previous, '=' | '\'' | '"')
        });
        boundary
            && suffix
                .chars()
                .next()
                .is_some_and(|next| next.is_ascii_alphabetic() || next == '_')
    })
}

fn contains_backtick_path_expansion(value: &str) -> bool {
    let Some(start) = value.find('`') else {
        return false;
    };
    let Some(end) = value[start + 1..].find('`') else {
        return false;
    };
    value[start + end + 2..].starts_with(['/', '\\'])
}

fn contains_windows_environment_expansion(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.iter().enumerate().any(|(start, byte)| {
        if *byte != b'%' {
            return false;
        }
        let name = &bytes[start + 1..];
        let Some(end) = name.iter().position(|candidate| *candidate == b'%') else {
            return false;
        };
        end > 0
            && name[..end]
                .iter()
                .all(|candidate| candidate.is_ascii_alphanumeric() || *candidate == b'_')
    })
}

fn is_windows_absolute_path(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && matches!(bytes[2], b'/' | b'\\')
}

fn claim_covers_issue(claim: &Claim, issue: u64) -> bool {
    let target = format!(".csdlc/issues/{issue}");
    claim
        .protected_paths
        .iter()
        .any(|path| path.trim_end_matches('/') == target)
}

fn claim_worktree_matches_store(store: &Store, claim: &Claim) -> Result<bool> {
    if claim.worktree == "." {
        return Ok(true);
    }
    let common_dir = PathBuf::from(
        crate::git::run(
            store.root(),
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    Ok(common_dir
        .parent()
        .map(|primary| primary.join(&claim.worktree))
        .and_then(|expected| expected.canonicalize().ok())
        .zip(store.root().canonicalize().ok())
        .is_some_and(|(expected, current)| expected == current))
}

fn valid_git_sha(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn complete_step_status(status: &mut StepStatus) -> Result<()> {
    if !matches!(*status, StepStatus::Pending | StepStatus::InProgress) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "terminal plan repair only allows forward completion",
        ));
    }
    *status = StepStatus::Completed;
    Ok(())
}

fn valid_mermaid_diagram(diagram: &str) -> bool {
    let first = diagram
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim();
    (first.starts_with("flowchart ")
        || first == "stateDiagram-v2"
        || first.starts_with("sequenceDiagram"))
        && diagram.lines().count() >= 2
}

#[cfg(test)]
mod terminal_design_repair_tests {
    use super::*;

    fn trusted_github(
        result: crate::github::GithubActionResult,
    ) -> crate::github::GithubActionResult {
        result.seal_for_test().expect("seal GitHub fixture")
    }

    #[test]
    fn claim_free_terminal_projection_is_backed_by_exact_materialized_receipt() {
        let (_temp, store, _authority, target, _receipt, _validation) =
            terminal_validation_fixture();
        assert_eq!(target.phase, LifecyclePhase::ClosedOut);
        assert!(target.claim.is_none());
        assert!(store
            .has_claim_free_terminal_authority(
                target.issue,
                &target.repository,
                &target.initialization_digest,
            )
            .expect("verify terminal authority"));
    }

    #[test]
    fn retained_terminal_authority_rejects_equal_generation_and_path_mismatch() {
        let (_temp, store, _authority, target, receipt, _validation) =
            terminal_validation_fixture();
        let terminal = receipt.record.terminal.as_ref().expect("terminal receipt");
        let mut observed = target.clone();
        observed.claim = Some(Claim {
            id: "stale-projection".into(),
            owner: "stale-owner".into(),
            generation: receipt.record.generation,
            acquired_unix_seconds: 1,
            expires_unix_seconds: 2,
            heartbeat_unix_seconds: 1,
            branch: terminal.released_branch.clone(),
            worktree: terminal.released_worktree.clone(),
            protected_paths: terminal.released_protected_paths.clone(),
            purpose: "stale projection fixture".into(),
        });
        observed.generation = receipt.record.generation;
        assert!(!store
            .has_claim_free_retained_terminal_authority(&observed)
            .expect("equal-generation receipt check"));

        observed.generation = receipt.record.generation.saturating_sub(1);
        observed
            .claim
            .as_mut()
            .expect("observed claim")
            .protected_paths
            .push("docs/unreleased".into());
        assert!(!store
            .has_claim_free_retained_terminal_authority(&observed)
            .expect("released-path mismatch check"));
    }

    #[test]
    fn terminal_receipt_rejects_noncanonical_authored_paths() {
        let (_temp, _store, _authority, _target, receipt, _validation) =
            terminal_validation_fixture();
        for replacement in ["../escape.md", "/absolute/escape.md", "./escape.md"] {
            let mut candidate = receipt.clone();
            let previous = candidate.record.design_path.clone();
            let contents = candidate
                .authored_artifacts
                .remove(&previous)
                .expect("design artifact");
            candidate.record.design_path = replacement.into();
            for kind in [CardKind::Spp, CardKind::Vpp] {
                match &mut candidate.cards.get_mut(&kind).expect("design card").content {
                    CardContent::Spp(values) => values.design_ref = replacement.into(),
                    CardContent::Vpp(values) => values.design_ref = replacement.into(),
                    _ => unreachable!("design card"),
                }
            }
            hydrate_projections(&mut candidate.record, &candidate.cards)
                .expect("receipt projections");
            candidate.record.digest =
                record_digest(&candidate.record).expect("receipt record digest");
            candidate
                .authored_artifacts
                .insert(replacement.into(), contents);
            candidate.digest.clear();
            candidate.digest = terminal_receipt_digest(&candidate).expect("receipt digest");

            let error = validate_terminal_receipt(&candidate)
                .expect_err("noncanonical authored path must fail");
            assert_eq!(error.code, ErrorCode::CorruptRecord);
            assert!(error
                .message
                .contains("authored paths must be clean repository-relative paths"));
        }
    }

    fn copy_tree(source: &Path, destination: &Path) {
        fs::create_dir_all(destination).expect("create fixture directory");
        for entry in fs::read_dir(source).expect("read fixture directory") {
            let entry = entry.expect("fixture entry");
            let target = destination.join(entry.file_name());
            if entry.file_type().expect("fixture type").is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).expect("copy fixture file");
            }
        }
    }

    fn terminal_validation_fixture() -> (
        tempfile::TempDir,
        Store,
        IssueRecord,
        IssueRecord,
        TerminalReceipt,
        ValidationResult,
    ) {
        let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let source_store = Store::new(&source_root);
        let temp = tempfile::tempdir().expect("temp root");
        let status = std::process::Command::new("git")
            .args(["init", "-q"])
            .current_dir(temp.path())
            .status()
            .expect("git init");
        assert!(status.success());
        for issue in [5358, 5613] {
            copy_tree(
                &source_store.issue_dir(issue),
                &temp.path().join(".csdlc/issues").join(issue.to_string()),
            );
            let record = source_store.load_record(issue).expect("source record");
            for path in [&record.design_path, &record.diagram_path] {
                let destination = temp.path().join(path);
                fs::create_dir_all(destination.parent().expect("authored parent"))
                    .expect("create authored parent");
                fs::copy(source_root.join(path), destination).expect("copy authored file");
            }
        }

        let store = Store::new(temp.path());
        let mut authority = store.load_record(5613).expect("authority");
        authority.claim.get_or_insert_with(|| Claim {
            id: "terminal-validation-authority".into(),
            owner: "test".into(),
            generation: authority.generation,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "terminal-validation-test".into(),
            worktree: ".".into(),
            protected_paths: vec![
                ".csdlc/issues/5358/".into(),
                "csdlc-v2/closeout/5358.json".into(),
            ],
            purpose: "test terminal recovery authority".into(),
        });
        authority
            .claim
            .as_mut()
            .expect("authority claim")
            .expires_unix_seconds = u64::MAX;
        let target_for_scope = store.load_record(5358).expect("target scope");
        let authority_claim = authority.claim.as_mut().expect("authority claim");
        for path in [target_for_scope.design_path, target_for_scope.diagram_path] {
            if !authority_claim.protected_paths.contains(&path) {
                authority_claim.protected_paths.push(path);
            }
        }
        let authority_cards = store.load_cards(5613).expect("authority cards");
        hydrate_projections(&mut authority, &authority_cards).expect("authority projections");
        authority.digest = record_digest(&authority).expect("authority digest");
        store
            .commit(5613, &authority, &authority_cards, false)
            .expect("authority commit");

        let target = store.load_record(5358).expect("target");
        let cards = store.load_cards(5358).expect("target cards");
        let mut authored_artifacts = BTreeMap::new();
        for path in [&target.design_path, &target.diagram_path] {
            authored_artifacts.insert(
                path.clone(),
                fs::read_to_string(temp.path().join(path)).expect("authored contents"),
            );
        }
        let mut receipt = TerminalReceipt {
            schema: "csdlc.terminal_receipt.v1".into(),
            issue: target.issue,
            repository: target.repository.clone(),
            initialization_digest: target.initialization_digest.clone(),
            receipt_ref: format!("csdlc-v2/closeout/{}.json", target.issue),
            authored_artifacts,
            record: target.clone(),
            cards: cards.clone(),
            digest: String::new(),
        };
        receipt.digest = terminal_receipt_digest(&receipt).expect("receipt digest");
        validate_terminal_receipt(&receipt).expect("valid receipt");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("create receipt parent");
        write_json(&receipt_path, &receipt).expect("write receipt");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        let expected = sor.actual_validation.first().expect("validation").clone();
        (temp, store, authority, target, receipt, expected)
    }

    #[cfg(unix)]
    #[test]
    fn first_terminal_receipt_retention_rejects_receipt_symlink_components() {
        use std::os::unix::fs::symlink;

        for symlink_parent in [false, true] {
            let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
            let receipt_path = store
                .terminal_receipt_path(target.issue)
                .expect("receipt path");
            if symlink_parent {
                let closeout = receipt_path.parent().expect("closeout directory");
                let backup = closeout.with_extension("retain-parent-backup");
                fs::rename(closeout, &backup).expect("move receipt parent");
                symlink(&backup, closeout).expect("symlink receipt parent");
            } else {
                let backup = receipt_path.with_extension("json.retain-leaf-backup");
                fs::rename(&receipt_path, &backup).expect("move receipt");
                symlink(&backup, &receipt_path).expect("symlink receipt");
            }

            let error = store
                .retain_terminal_receipt(target.issue)
                .expect_err("symlinked receipt authority must fail closed");
            assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        }
    }

    #[cfg(unix)]
    #[test]
    fn first_terminal_receipt_retention_rejects_authored_symlink_components() {
        use std::os::unix::fs::symlink;

        for symlink_parent in [false, true] {
            let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
            fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
                .expect("remove retained receipt");
            let design = store.root.join(&target.design_path);
            if symlink_parent {
                let parent = design.parent().expect("design parent");
                let backup = store.root.join("retain-authored-parent-backup");
                fs::rename(parent, &backup).expect("move authored parent");
                symlink(&backup, parent).expect("symlink authored parent");
            } else {
                let backup = design.with_extension("md.retain-leaf-backup");
                fs::rename(&design, &backup).expect("move authored file");
                symlink(&backup, &design).expect("symlink authored file");
            }

            let error = store
                .retain_terminal_receipt(target.issue)
                .expect_err("symlinked authored authority must fail closed");
            assert_eq!(error.code, ErrorCode::UnsafeCheckout);
            assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
        }
    }

    #[cfg(unix)]
    #[test]
    fn first_terminal_receipt_retention_rejects_projection_symlink_components() {
        use std::os::unix::fs::symlink;

        for symlink_parent in [false, true] {
            let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
            let receipt_path = store
                .terminal_receipt_path(target.issue)
                .expect("receipt path");
            fs::remove_file(&receipt_path).expect("remove retained receipt");
            let cards = store.issue_dir(target.issue).join("cards");
            if symlink_parent {
                let backup = store.issue_dir(target.issue).join("cards.retain-backup");
                fs::rename(&cards, &backup).expect("move projection parent");
                symlink(&backup, &cards).expect("symlink projection parent");
            } else {
                let projection = cards.join("spp.values.json");
                let backup = cards.join("spp.values.retain-backup.json");
                fs::rename(&projection, &backup).expect("move projection leaf");
                symlink(&backup, &projection).expect("symlink projection leaf");
            }

            let error = store
                .retain_terminal_receipt(target.issue)
                .expect_err("symlinked projection authority must fail closed");
            assert_eq!(error.code, ErrorCode::UnsafeCheckout);
            assert!(!receipt_path.exists());
        }
    }

    #[cfg(unix)]
    #[test]
    fn existing_terminal_receipt_retention_revalidates_all_authority_bytes() {
        use std::os::unix::fs::symlink;

        for mutation in ["authored_symlink", "audit_drift", "rendered_drift"] {
            let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
            match mutation {
                "authored_symlink" => {
                    let design = store.root.join(&target.design_path);
                    let backup = design.with_extension("md.existing-receipt-backup");
                    fs::rename(&design, &backup).expect("move authored file");
                    symlink(&backup, &design).expect("symlink authored file");
                }
                "audit_drift" => {
                    fs::write(store.issue_dir(target.issue).join("audit.jsonl"), b"{}\n")
                        .expect("corrupt audit projection")
                }
                "rendered_drift" => fs::write(
                    store.issue_dir(target.issue).join("cards/spp.md"),
                    b"# drift\n",
                )
                .expect("corrupt rendered projection"),
                _ => unreachable!(),
            }

            let error = store
                .retain_terminal_receipt(target.issue)
                .expect_err("existing receipt must revalidate materialized authority");
            assert!(matches!(
                error.code,
                ErrorCode::UnsafeCheckout | ErrorCode::CorruptRecord
            ));
        }
    }

    #[test]
    fn terminal_receipt_rollback_rejects_conflicting_current_bytes() {
        let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        let original = fs::read(&receipt_path).expect("original receipt bytes");
        fs::write(&receipt_path, b"conflicting receipt bytes\n")
            .expect("replace receipt with conflicting bytes");

        let error = store
            .restore_terminal_receipt(&receipt_path, &original)
            .expect_err("rollback must not overwrite a concurrently changed receipt");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
        assert_eq!(
            fs::read(&receipt_path).expect("conflicting receipt retained"),
            b"conflicting receipt bytes\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn terminal_retention_rejects_lock_symlink_components() {
        use std::os::unix::fs::symlink;

        for attack in [
            "terminal_lock",
            "terminal_lock_parent",
            "issue_lock",
            "issue_lock_parent",
        ] {
            let (_temp, store, _authority, target, _receipt, _) = terminal_validation_fixture();
            let receipt_path = store
                .terminal_receipt_path(target.issue)
                .expect("receipt path");
            fs::remove_file(&receipt_path).expect("remove retained receipt");
            let common = PathBuf::from(
                crate::git::run(
                    &store.root,
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"],
                )
                .expect("Git common path")
                .stdout,
            );
            let outside = tempfile::tempdir().expect("outside target");
            match attack {
                "terminal_lock" => {
                    let lock = common.join("csdlc-v2/terminal-repairs.lock");
                    if lock.exists() {
                        fs::remove_file(&lock).expect("remove terminal repair lock");
                    }
                    symlink(outside.path().join("terminal-repairs.lock"), lock)
                        .expect("inject terminal repair lock symlink");
                }
                "terminal_lock_parent" => {
                    let parent = common.join("csdlc-v2");
                    let backup = common.join("csdlc-v2.lock-parent-backup");
                    fs::rename(&parent, &backup).expect("move terminal lock parent");
                    symlink(outside.path(), parent).expect("inject terminal lock parent symlink");
                }
                "issue_lock" => {
                    let parent = store.root.join(".csdlc/locks");
                    fs::create_dir_all(&parent).expect("create issue lock parent");
                    let lock = parent.join(format!("{}.lock", target.issue));
                    if lock.exists() {
                        fs::remove_file(&lock).expect("remove issue lock");
                    }
                    symlink(outside.path().join("issue.lock"), lock)
                        .expect("inject issue lock symlink");
                }
                "issue_lock_parent" => {
                    let parent = store.root.join(".csdlc/locks");
                    fs::create_dir_all(&parent).expect("create issue lock parent");
                    let backup = store.root.join(".csdlc/locks.lock-parent-backup");
                    fs::rename(&parent, &backup).expect("move issue lock parent");
                    symlink(outside.path(), parent).expect("inject issue lock parent symlink");
                }
                _ => unreachable!(),
            }

            let error = store
                .retain_terminal_receipt(target.issue)
                .expect_err("symlinked lock authority must fail closed");
            assert_eq!(error.code, ErrorCode::UnsafeCheckout);
            assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
        }
    }

    #[cfg(unix)]
    #[test]
    fn terminal_recovery_rejects_receipt_symlinks_after_durable_boundaries() {
        use std::os::unix::fs::symlink;

        for projection_committed in [false, true] {
            for attack in ["parent", "receipt", "lock", "temporary"] {
                let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
                let request = TerminalDesignRepairRequest {
                    authority_issue: authority.issue,
                    target_issue: target.issue,
                    expected_authority_generation: authority.generation,
                    expected_authority_digest: authority.digest.clone(),
                    expected_target_generation: target.generation,
                    expected_target_digest: target.digest.clone(),
                    expected_receipt_digest: receipt.digest.clone(),
                    authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                    actor: "codex:test".into(),
                    reviewer: "reviewer".into(),
                    source_design_path: target.design_path.clone(),
                    source_diagram_path: target.diagram_path.clone(),
                    expected_design_digest: digest(
                        &fs::read(store.root.join(&target.design_path)).unwrap(),
                    ),
                    expected_diagram_digest: digest(
                        &fs::read(store.root.join(&target.diagram_path)).unwrap(),
                    ),
                    fail_after_stage: Some("after_journal".into()),
                };
                let error = store
                    .repair_terminal_design(request)
                    .expect_err("injected durable interruption");
                assert_eq!(error.code, ErrorCode::InterruptedTransaction);

                let journal_path = store
                    .terminal_transaction_path(target.issue)
                    .expect("journal path");
                let mut journal: TerminalTransactionJournal =
                    read_json(&journal_path).expect("journal");
                if projection_committed {
                    let target_receipt: TerminalReceipt =
                        serde_json::from_slice(&journal.target_receipt).expect("target receipt");
                    store
                        .commit(
                            target.issue,
                            &target_receipt.record,
                            &target_receipt.cards,
                            false,
                        )
                        .expect("simulate committed projection");
                    journal.stage = "projection_committed_terminal_design_repair".into();
                    store
                        .write_terminal_transaction_journal(&journal)
                        .expect("advance journal stage");
                }

                let receipt_path = store
                    .terminal_receipt_path(target.issue)
                    .expect("receipt path");
                let closeout = receipt_path.parent().expect("closeout parent");
                let outside = tempfile::tempdir().expect("outside target");
                match attack {
                    "parent" => {
                        let backup = closeout.with_extension("recovery-backup");
                        fs::rename(closeout, &backup).expect("move receipt parent");
                        symlink(outside.path(), closeout).expect("inject receipt parent symlink");
                    }
                    "receipt" => {
                        fs::remove_file(&receipt_path).expect("remove receipt leaf");
                        symlink(outside.path().join("receipt.json"), &receipt_path)
                            .expect("inject receipt leaf symlink");
                    }
                    "lock" => {
                        let lock = closeout.join("receipts.lock");
                        if lock.exists() {
                            fs::remove_file(&lock).expect("remove receipt lock");
                        }
                        symlink(outside.path().join("receipts.lock"), &lock)
                            .expect("inject receipt lock symlink");
                    }
                    "temporary" => {
                        let temporary = receipt_path.with_extension("json.recovery-tmp");
                        symlink(outside.path().join("receipt.tmp"), &temporary)
                            .expect("inject receipt temporary symlink");
                    }
                    _ => unreachable!(),
                }

                let recovery_error = store
                    .recover_with_terminal_lock(target.issue)
                    .expect_err("recovery must reject receipt-path symlink");
                assert_eq!(recovery_error.code, ErrorCode::UnsafeCheckout);
                assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
                assert!(journal_path.exists());
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn terminal_recovery_rejects_authored_parent_symlink_after_durable_boundaries() {
        use std::os::unix::fs::symlink;

        for projection_committed in [false, true] {
            let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
            let request = TerminalDesignRepairRequest {
                authority_issue: authority.issue,
                target_issue: target.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                expected_target_generation: target.generation,
                expected_target_digest: target.digest.clone(),
                expected_receipt_digest: receipt.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                reviewer: "reviewer".into(),
                source_design_path: target.design_path.clone(),
                source_diagram_path: target.diagram_path.clone(),
                expected_design_digest: digest(
                    &fs::read(store.root.join(&target.design_path)).unwrap(),
                ),
                expected_diagram_digest: digest(
                    &fs::read(store.root.join(&target.diagram_path)).unwrap(),
                ),
                fail_after_stage: Some("after_journal".into()),
            };
            let error = store
                .repair_terminal_design(request)
                .expect_err("injected durable interruption");
            assert_eq!(error.code, ErrorCode::InterruptedTransaction);

            let journal_path = store
                .terminal_transaction_path(target.issue)
                .expect("journal path");
            let mut journal: TerminalTransactionJournal =
                read_json(&journal_path).expect("journal");
            if projection_committed {
                let target_receipt: TerminalReceipt =
                    serde_json::from_slice(&journal.target_receipt).expect("target receipt");
                store
                    .commit(
                        target.issue,
                        &target_receipt.record,
                        &target_receipt.cards,
                        false,
                    )
                    .expect("simulate committed projection");
                journal.stage = "projection_committed_terminal_design_repair".into();
                store
                    .write_terminal_transaction_journal(&journal)
                    .expect("advance journal stage");
            }

            let design_parent = store
                .root
                .join(&target.design_path)
                .parent()
                .expect("design parent")
                .to_path_buf();
            let authored_backup = store.root.join("recovery-authored-backup");
            fs::rename(&design_parent, &authored_backup).expect("move authored parent");
            let outside = tempfile::tempdir().expect("outside target");
            symlink(outside.path(), &design_parent).expect("inject authored parent symlink");

            let recovery_error = store
                .recover_with_terminal_lock(target.issue)
                .expect_err("recovery must reject post-journal symlink");
            assert_eq!(recovery_error.code, ErrorCode::UnsafeCheckout);
            assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
            assert!(journal_path.exists());
        }
    }

    fn validation_repair_request(
        authority: &IssueRecord,
        target: &IssueRecord,
        receipt: &TerminalReceipt,
        expected: ValidationResult,
        fail_after_stage: Option<&str>,
    ) -> TerminalSorValidationRepairRequest {
        let mut replacement = expected.clone();
        replacement.evidence_ref = "issue-5358:portable-terminal-proof".into();
        TerminalSorValidationRepairRequest {
            authority_issue: authority.issue,
            target_issue: target.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            expected_target_generation: target.generation,
            expected_target_digest: target.digest.clone(),
            expected_receipt_digest: receipt.digest.clone(),
            authority_claim_id: authority.claim.as_ref().expect("claim").id.clone(),
            actor: "codex:test".into(),
            expected_result: expected,
            replacement_result: replacement,
            fail_after_stage: fail_after_stage.map(str::to_owned),
        }
    }

    fn disposition_fixture() -> (
        tempfile::TempDir,
        Store,
        IssueRecord,
        IssueRecord,
        TerminalReceipt,
        TerminalEvidence,
    ) {
        let (temp, store, authority, mut target, mut receipt, _) = terminal_validation_fixture();
        let mut cards = store.load_cards(target.issue).expect("target cards");
        let mut expected = target.terminal.clone().expect("terminal evidence");
        expected.pull_request = None;
        expected.disposition = crate::readiness::TerminalDisposition::ClosedNoPr;
        expected.observed_sha = None;
        expected.observed_state = "closed_no_pr".into();
        target.publication = None;
        target.readiness = None;
        target.terminal = Some(expected.clone());
        let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).expect("SOR").content else {
            panic!("SOR");
        };
        sor.integration_state = crate::cards::IntegrationState::ClosedNoPr;
        sor.merge_state = crate::cards::MergeState::ClosedUnmerged;
        hydrate_projections(&mut target, &cards).expect("target projections");
        target.digest = record_digest(&target).expect("target digest");
        store
            .commit(target.issue, &target, &cards, false)
            .expect("target commit");
        receipt.record = target.clone();
        receipt.cards = cards;
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt).expect("receipt digest");
        validate_terminal_receipt(&receipt).expect("valid closed-no-pr receipt");
        write_json(
            &store
                .terminal_receipt_path(target.issue)
                .expect("receipt path"),
            &receipt,
        )
        .expect("write receipt");
        (temp, store, authority, target, receipt, expected)
    }

    fn disposition_request(
        authority: &IssueRecord,
        target: &IssueRecord,
        receipt: &TerminalReceipt,
        expected: TerminalEvidence,
        fail_after_stage: Option<&str>,
    ) -> TerminalDispositionRepairRequest {
        let sha = "92fde26a2ca073e204459fce1bb5e88d7c895528";
        TerminalDispositionRepairRequest {
            authority_issue: authority.issue,
            target_issue: target.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().expect("claim").id.clone(),
            expected_target_generation: target.generation,
            expected_target_digest: target.digest.clone(),
            expected_receipt_digest: receipt.digest.clone(),
            expected_terminal: expected,
            merged_evidence: trusted_github(crate::github::GithubActionResult {
                schema: "csdlc.github_action_result.v1".into(),
                repository: target.repository.clone(),
                action: crate::github::GithubAction::PrState,
                operation_key: Some("test:merged-observation".into()),
                issue: None,
                comment_id: None,
                pr_state: Some(crate::github::PrStatePacket {
                    schema: "csdlc.github_pr_state.v1".into(),
                    repository: target.repository.clone(),
                    pull_request: 5634,
                    linked_issue: Some(target.issue),
                    linkage_source: Some("github_closing_issues_references".into()),
                    draft: false,
                    merge_state: "unknown".into(),
                    review_decision: "approved".into(),
                    base_ref: Some("main".into()),
                    head_ref: Some("codex/5632-adl-pr-cycle-v2".into()),
                    head_sha: sha.into(),
                    url: Some(format!(
                        "https://github.com/{}/pull/5634",
                        target.repository
                    )),
                    body: Some(format!("Closes #{}", target.issue)),
                    merged: true,
                    merge_commit_sha: Some("4d68d4b1f4f70c15223ebdf71d59c9010e5e3d4c".into()),
                    checks: Vec::new(),
                    required_check_names: Vec::new(),
                    classification: "merged".into(),
                }),
                reconciled: true,
                producer_digest: None,
            }),
            actor: "codex:test".into(),
            correction_note:
                "Historical no-PR classification is superseded by exact merged PR evidence.".into(),
            fail_after_stage: fail_after_stage.map(str::to_owned),
        }
    }

    #[test]
    fn terminal_design_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_design(TerminalDesignRepairRequest {
                authority_issue: 5487,
                target_issue: 5467,
                expected_authority_generation: 1,
                expected_authority_digest: String::new(),
                expected_target_generation: 18,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                reviewer: "reviewer".into(),
                source_design_path: "design.md".into(),
                source_diagram_path: "diagram.mmd".into(),
                expected_design_digest: "design".into(),
                expected_diagram_digest: "diagram".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_design_repair_rejects_uncovered_external_artifacts() {
        let (_temp, store, mut authority, target, receipt, _) = terminal_validation_fixture();
        let authority_cards = store.load_cards(authority.issue).unwrap();
        let target_root = format!(".csdlc/issues/{}/", target.issue);
        authority
            .claim
            .as_mut()
            .unwrap()
            .protected_paths
            .retain(|path| path.starts_with(&target_root) || path.contains("closeout"));
        hydrate_projections(&mut authority, &authority_cards).unwrap();
        authority.digest = record_digest(&authority).unwrap();
        store
            .commit(authority.issue, &authority, &authority_cards, false)
            .unwrap();
        let error = store
            .repair_terminal_design(TerminalDesignRepairRequest {
                authority_issue: authority.issue,
                target_issue: target.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest,
                expected_target_generation: target.generation,
                expected_target_digest: target.digest,
                expected_receipt_digest: receipt.digest,
                authority_claim_id: authority.claim.unwrap().id,
                actor: "codex:test".into(),
                reviewer: "reviewer".into(),
                source_design_path: target.design_path.clone(),
                source_diagram_path: target.diagram_path.clone(),
                expected_design_digest: digest(
                    &fs::read(store.root.join(&target.design_path)).unwrap(),
                ),
                expected_diagram_digest: digest(
                    &fs::read(store.root.join(&target.diagram_path)).unwrap(),
                ),
                fail_after_stage: None,
            })
            .expect_err("external authored paths require explicit claim coverage");
        assert_eq!(error.code, ErrorCode::InvalidInput);
    }

    #[test]
    fn terminal_design_repair_mermaid_guard_is_fail_closed() {
        assert!(valid_mermaid_diagram("flowchart LR\n  A-->B\n"));
        assert!(!valid_mermaid_diagram("not mermaid\n  A-->B\n"));
        assert!(!valid_mermaid_diagram("flowchart LR\n"));
    }

    #[test]
    fn terminal_plan_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_plan_step(TerminalPlanStepRepairRequest {
                authority_issue: 5518,
                target_issue: 5516,
                expected_authority_generation: 0,
                expected_authority_digest: String::new(),
                expected_target_generation: 18,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                step_id: "S3".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_plan_repair_status_is_forward_only() {
        for initial in [StepStatus::Pending, StepStatus::InProgress] {
            let mut status = initial;
            complete_step_status(&mut status).expect("forward completion");
            assert_eq!(status, StepStatus::Completed);
        }
        let mut completed = StepStatus::Completed;
        let error = complete_step_status(&mut completed).expect_err("no rewrite");
        assert_eq!(error.code.to_string(), "invalid_transition");
    }

    #[test]
    fn terminal_plan_repair_requires_exact_target_scope() {
        let mut claim = Claim {
            id: "claim".into(),
            owner: "agent".into(),
            generation: 0,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "issue".into(),
            worktree: ".".into(),
            protected_paths: vec![".csdlc/issues/5517".into()],
            purpose: "repair".into(),
        };
        assert!(!claim_covers_issue(&claim, 5516));
        claim.protected_paths.push(".csdlc/issues/5516/".into());
        assert!(claim_covers_issue(&claim, 5516));
    }

    #[test]
    fn terminal_sor_artifact_repair_rejects_incomplete_authority_before_io() {
        let root = tempfile::tempdir().expect("temp root");
        let error = Store::new(root.path())
            .repair_terminal_sor_artifact(TerminalSorArtifactRepairRequest {
                authority_issue: 5527,
                target_issue: 5390,
                expected_authority_generation: 0,
                expected_authority_digest: String::new(),
                expected_target_generation: 39,
                expected_target_digest: "target".into(),
                expected_receipt_digest: "receipt".into(),
                authority_claim_id: "claim".into(),
                actor: "codex".into(),
                stale_ref: ".csdlc/issues/5390/diagram.mmd".into(),
                retained_ref: ".csdlc/issues/5390/retained/diagram.mmd".into(),
                expected_artifact_digest: "diagram".into(),
                fail_after_stage: None,
            })
            .expect_err("missing authority digest must fail closed");
        assert_eq!(error.code.to_string(), "invalid_input");
    }

    #[test]
    fn terminal_sor_artifact_replacement_is_exact_and_nonduplicating() {
        let mut cards = initial_cards(
            1,
            "example/repo",
            "docs/design.md",
            "design",
            "docs/diagram.mmd",
            "diagram",
            InitialCardInput {
                title: "test".into(),
                slug: "test".into(),
                version: "v0.91.7".into(),
                goal: "test".into(),
                required_outcome: "test".into(),
                declared_scope: vec!["test".into()],
                authority_boundary: vec!["test".into()],
                operator_constraints: vec!["none".into()],
                task_boundary: "test".into(),
                deliverables: vec!["test".into()],
                acceptance_criteria: vec!["test".into()],
                dependencies: vec!["test".into()],
                repo_inputs: vec!["test".into()],
                non_goals: vec!["test".into()],
                plan_summary: "test".into(),
                steps: vec![crate::cards::PlanStep {
                    id: "S1".into(),
                    action: "test".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    status: StepStatus::Pending,
                }],
                invariants: vec!["test".into()],
                risks: vec!["test".into()],
                planning_profile: crate::cards::PlanningProfile::Small,
                stop_conditions: vec!["test".into()],
                validation_lanes: vec![crate::cards::ValidationLane {
                    lane: "test".into(),
                    proof_role: "test".into(),
                    acceptance_ids: vec!["AC-1".into()],
                    deterministic: true,
                    resource_profile: crate::cards::ResourceProfile::Small,
                    budget_seconds: 1,
                    budget_tokens: 1,
                    argv: vec!["test".into()],
                    parallel_group: "test".into(),
                    defer_reason: None,
                }],
                failure_policy: "test".into(),
                review_prompts: vec!["test".into()],
                review_scope: "test".into(),
            },
        )
        .expect("cards");
        let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).unwrap().content else {
            panic!("SOR");
        };
        sor.artifacts = vec!["old".into()];
        replace_terminal_sor_artifact(&mut cards, "old", "retained").expect("replacement");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        assert_eq!(sor.artifacts, vec!["retained"]);
        assert!(replace_terminal_sor_artifact(&mut cards, "old", "retained").is_err());
    }

    #[test]
    fn terminal_sor_validation_repair_updates_projection_and_receipt_atomically() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let request =
            validation_repair_request(&authority, &target, &receipt, expected.clone(), None);
        let replacement = request.replacement_result.clone();
        let repaired = store
            .repair_terminal_sor_validation(request)
            .expect("terminal validation repair");
        assert_eq!(repaired.phase, LifecyclePhase::ClosedOut);
        assert!(repaired.claim.is_none());
        assert_eq!(repaired.generation, target.generation + 1);
        let cards = store.load_cards(target.issue).expect("repaired cards");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR");
        };
        assert!(!sor.actual_validation.contains(&expected));
        assert_eq!(
            sor.actual_validation
                .iter()
                .filter(|result| *result == &replacement)
                .count(),
            1
        );
        let repaired_receipt = store
            .load_terminal_receipt(target.issue)
            .expect("receipt load")
            .expect("receipt");
        assert_eq!(repaired_receipt.record.digest, repaired.digest);
        assert_eq!(repaired_receipt.cards, cards);
    }

    #[test]
    fn terminal_disposition_repair_updates_projection_and_receipt_atomically() {
        let (_temp, store, authority, target, receipt, expected) = disposition_fixture();
        let request = disposition_request(&authority, &target, &receipt, expected, None);
        let repaired = store
            .repair_terminal_disposition(request)
            .expect("disposition repair");
        let terminal = repaired.terminal.as_ref().expect("terminal");
        assert_eq!(
            terminal.disposition,
            crate::readiness::TerminalDisposition::Merged
        );
        assert_eq!(terminal.pull_request, Some(5634));
        assert_eq!(
            repaired
                .publication
                .as_ref()
                .expect("publication")
                .pull_request,
            5634
        );
        let cards = store.load_cards(target.issue).expect("cards");
        let CardContent::Sor(sor) = &cards[&CardKind::Sor].content else {
            panic!("SOR")
        };
        assert_eq!(
            sor.integration_state,
            crate::cards::IntegrationState::Merged
        );
        assert_eq!(sor.merge_state, crate::cards::MergeState::Merged);
        let retained = store.load_terminal_receipt(target.issue).unwrap().unwrap();
        assert_eq!(retained.record, repaired);
        assert_eq!(retained.cards, cards);
    }

    #[test]
    fn terminal_disposition_repair_recovers_after_projection_interruption() {
        let (_temp, store, authority, target, receipt, expected) = disposition_fixture();
        let request = disposition_request(
            &authority,
            &target,
            &receipt,
            expected,
            Some("after_projection"),
        );
        let error = store
            .repair_terminal_disposition(request)
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        store
            .recover_with_terminal_lock(target.issue)
            .expect("recover transaction");
        let repaired = store.load_record(target.issue).expect("record");
        let retained = store.load_terminal_receipt(target.issue).unwrap().unwrap();
        assert_eq!(retained.record, repaired);
        assert_eq!(repaired.terminal.as_ref().unwrap().pull_request, Some(5634));
    }

    #[test]
    fn terminal_transaction_journal_refuses_recovery_from_sibling_worktree() {
        let (temp, store, authority, target, receipt, expected) = disposition_fixture();
        assert!(std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .status()
            .expect("stage fixture")
            .success());
        assert!(std::process::Command::new("git")
            .args([
                "-c",
                "user.name=csdlc-test",
                "-c",
                "user.email=csdlc-test@example.invalid",
                "commit",
                "-q",
                "-m",
                "fixture",
            ])
            .current_dir(temp.path())
            .status()
            .expect("commit fixture")
            .success());
        let error = store
            .repair_terminal_disposition(disposition_request(
                &authority,
                &target,
                &receipt,
                expected,
                Some("after_journal"),
            ))
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);

        let foreign_parent = tempfile::tempdir().expect("foreign parent");
        let foreign = foreign_parent.path().join("worktree");
        assert!(std::process::Command::new("git")
            .args(["worktree", "add", "-q", "-b", "terminal-foreign"])
            .arg(&foreign)
            .arg("HEAD")
            .current_dir(temp.path())
            .status()
            .expect("add sibling worktree")
            .success());
        let foreign_store = Store::new(&foreign);
        let error = foreign_store
            .recover_with_terminal_lock(target.issue)
            .expect_err("foreign recovery must fail");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        store
            .recover_with_terminal_lock(target.issue)
            .expect("origin recovery");
    }

    #[test]
    fn terminal_disposition_repair_follows_typed_reconcile_for_stale_implemented_projection() {
        let (temp, store, authority, target, receipt, expected) = disposition_fixture();
        let cards = store.load_cards(target.issue).expect("terminal cards");
        let mut stale = target.clone();
        let implemented = stale
            .transitions
            .iter()
            .position(|event| event.to == LifecyclePhase::Implemented)
            .expect("implemented transition");
        stale.transitions.truncate(implemented + 1);
        stale.phase = LifecyclePhase::Implemented;
        stale.terminal = None;
        stale.publication = None;
        stale.readiness = None;
        hydrate_projections(&mut stale, &cards).expect("stale projections");
        stale.digest = record_digest(&stale).expect("stale digest");
        store
            .commit(stale.issue, &stale, &cards, false)
            .expect("write stale projection");
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap().unwrap(),
            receipt,
            "retained terminal authority must remain unchanged"
        );

        assert!(std::process::Command::new("git")
            .args(["checkout", "-q", "-b", "recordless-recovery-test"])
            .current_dir(temp.path())
            .status()
            .expect("create fixture branch")
            .success());
        let branch = crate::git::current_branch(temp.path()).expect("fixture branch");
        let reconciled = store
            .reconcile_terminal(ReconcileTerminalRequest {
                issue: target.issue,
                expected_initialization_digest: receipt.initialization_digest.clone(),
                expected_branch: branch,
                expected_worktree: temp.path().canonicalize().unwrap().to_string_lossy().into(),
                actor: "codex:test".into(),
                reason: "Materialize retained terminal authority before disposition repair.".into(),
                follow_ups: Vec::new(),
            })
            .expect("typed terminal reconciliation");
        assert_eq!(reconciled.phase, LifecyclePhase::ClosedOut);
        let reconciled_receipt = store.load_terminal_receipt(target.issue).unwrap().unwrap();
        assert_eq!(reconciled_receipt.record, reconciled);

        let repaired = store
            .repair_terminal_disposition(disposition_request(
                &authority,
                &reconciled,
                &reconciled_receipt,
                expected,
                None,
            ))
            .expect("disposition repair after reconciliation");
        assert_eq!(repaired.terminal.as_ref().unwrap().pull_request, Some(5634));
        assert_eq!(
            store
                .load_terminal_receipt(target.issue)
                .unwrap()
                .unwrap()
                .record,
            repaired
        );
    }

    #[test]
    fn terminal_receipt_transport_materializes_absent_clone_state_and_is_idempotent() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        for path in receipt.authored_artifacts.keys() {
            fs::remove_file(store.root.join(path)).expect("remove authored artifact");
        }
        let request = TerminalReceiptTransportRequest {
            authority_issue: authority.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            actor: "codex:test".into(),
            receipt: receipt.clone(),
            fail_after_stage: None,
        };
        let transported = store
            .transport_terminal_receipt(request.clone())
            .expect("transport");
        assert_eq!(transported, receipt.record);
        for (path, expected) in &receipt.authored_artifacts {
            assert_eq!(
                fs::read(store.root.join(path)).expect("materialized artifact"),
                expected.as_bytes()
            );
        }
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap().unwrap(),
            receipt
        );
        assert_eq!(
            store
                .transport_terminal_receipt(request)
                .expect("idempotent"),
            transported
        );
    }

    #[cfg(unix)]
    #[test]
    fn terminal_receipt_transport_idempotency_rejects_issue_local_artifact_symlink() {
        use std::os::unix::fs::symlink;

        let (_temp, store, authority, target, mut receipt, _) = terminal_validation_fixture();
        let old_design_path = receipt.record.design_path.clone();
        let old_diagram_path = receipt.record.diagram_path.clone();
        let design = receipt
            .authored_artifacts
            .remove(&old_design_path)
            .expect("design artifact");
        let diagram = receipt
            .authored_artifacts
            .remove(&old_diagram_path)
            .expect("diagram artifact");
        let design_path = format!(".csdlc/issues/{}/retained/design.md", target.issue);
        let diagram_path = format!(".csdlc/issues/{}/retained/diagram.mmd", target.issue);
        receipt.record.design_path = design_path.clone();
        receipt.record.diagram_path = diagram_path.clone();
        for kind in [CardKind::Spp, CardKind::Vpp] {
            match &mut receipt.cards.get_mut(&kind).expect("design card").content {
                CardContent::Spp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                CardContent::Vpp(values) => {
                    values.design_ref = design_path.clone();
                    values.diagram_ref = diagram_path.clone();
                }
                _ => unreachable!("design card"),
            }
        }
        hydrate_projections(&mut receipt.record, &receipt.cards).expect("receipt projections");
        receipt.record.digest = record_digest(&receipt.record).expect("receipt record digest");
        receipt.authored_artifacts =
            BTreeMap::from([(design_path.clone(), design), (diagram_path, diagram)]);
        receipt.digest.clear();
        receipt.digest = terminal_receipt_digest(&receipt).expect("receipt digest");
        validate_terminal_receipt(&receipt).expect("issue-local receipt");
        store
            .commit_with_authored(
                target.issue,
                &receipt.record,
                &receipt.cards,
                false,
                Some(&receipt.authored_artifacts),
            )
            .expect("materialize issue-local projection");
        write_json(
            &store.terminal_receipt_path(target.issue).unwrap(),
            &receipt,
        )
        .expect("write issue-local receipt");

        let design_file = store.root.join(&design_path);
        let backup = store.root.join("issue-local-design-backup.md");
        fs::rename(&design_file, &backup).expect("move issue-local design");
        symlink(&backup, &design_file).expect("symlink issue-local design");
        let before = transport_projection_bytes(&store, target.issue);
        let receipt_path = store.terminal_receipt_path(target.issue).unwrap();
        let receipt_bytes = fs::read(&receipt_path).expect("receipt bytes");
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt,
                fail_after_stage: None,
            })
            .expect_err("idempotent issue-local symlink must fail closed");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        assert_eq!(transport_projection_bytes(&store, target.issue), before);
        assert_eq!(fs::read(receipt_path).unwrap(), receipt_bytes);
        assert!(fs::symlink_metadata(design_file)
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(!store
            .terminal_transaction_path(target.issue)
            .unwrap()
            .exists());
    }

    fn write_newer_terminal_receipt_only(
        store: &Store,
        receipt: &TerminalReceipt,
    ) -> TerminalReceipt {
        let mut newer = receipt.clone();
        newer.record.generation += 1;
        for card in newer.cards.values_mut() {
            card.identity.generation = newer.record.generation;
        }
        newer.record.audit.push(AuditEvent {
            sequence: newer.record.audit.len() as u64 + 1,
            generation: newer.record.generation,
            actor: "csdlc-closeout".into(),
            reason: "normalize terminal receipt metadata in the source clone".into(),
            operation: "normalize_terminal_receipt_ref".into(),
        });
        hydrate_projections(&mut newer.record, &newer.cards).expect("newer projections");
        newer.record.digest = record_digest(&newer.record).expect("newer record digest");
        newer.digest.clear();
        newer.digest = terminal_receipt_digest(&newer).expect("newer receipt digest");
        validate_terminal_receipt(&newer).expect("newer receipt");
        write_json(&store.terminal_receipt_path(newer.issue).unwrap(), &newer)
            .expect("write newer receipt only");
        newer
    }

    #[test]
    fn corrupt_terminal_receipt_reconciliation_is_authorized_cas_guarded_and_recoverable() {
        let (temp, store, mut authority, target, receipt, _) = terminal_validation_fixture();
        let authority_cards = store.load_cards(authority.issue).expect("authority cards");
        let authority_claim = authority.claim.as_mut().expect("authority claim");
        authority_claim.branch = crate::git::current_branch(store.root()).expect("fixture branch");
        authority_claim.worktree = ".".into();
        authority_claim.protected_paths.extend([
            format!(".csdlc/issues/{}", authority.issue),
            "csdlc-v2".into(),
        ]);
        authority_claim.protected_paths.sort();
        authority_claim.protected_paths.dedup();
        hydrate_projections(&mut authority, &authority_cards).expect("authority projections");
        authority.digest = record_digest(&authority).expect("authority digest");
        store
            .commit(authority.issue, &authority, &authority_cards, false)
            .expect("authority commit");

        assert!(std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .status()
            .expect("stage aggregate fixture")
            .success());
        assert!(std::process::Command::new("git")
            .args([
                "-c",
                "user.name=csdlc-test",
                "-c",
                "user.email=csdlc-test@example.invalid",
                "commit",
                "-q",
                "-m",
                "aggregate fixture",
            ])
            .current_dir(temp.path())
            .status()
            .expect("commit aggregate fixture")
            .success());
        let active_target_root = temp.path().join("active-target");
        assert!(std::process::Command::new("git")
            .args(["worktree", "add", "-q", "-b", "active-target"])
            .arg(&active_target_root)
            .arg("HEAD")
            .current_dir(temp.path())
            .status()
            .expect("add active target worktree")
            .success());
        let active_target_store = Store::new(&active_target_root);
        let mut active_target = active_target_store
            .load_record(target.issue)
            .expect("active target record");
        let active_target_cards = active_target_store
            .load_cards(target.issue)
            .expect("active target cards");
        active_target.claim = Some(Claim {
            id: "active-target-claim".into(),
            owner: "target-session".into(),
            generation: active_target.generation,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "active-target".into(),
            worktree: "active-target".into(),
            protected_paths: vec![format!(".csdlc/issues/{}", target.issue)],
            purpose: "prove aggregate recovery refuses a live target checkout".into(),
        });
        hydrate_projections(&mut active_target, &active_target_cards)
            .expect("active target projections");
        active_target.digest = record_digest(&active_target).expect("active target digest");
        active_target_store
            .commit(
                active_target.issue,
                &active_target,
                &active_target_cards,
                false,
            )
            .expect("active target commit");

        let newer = write_newer_terminal_receipt_only(&store, &receipt);
        let index_path = store.issue_dir(target.issue).join("index.json");
        let mut corrupt_index = serde_json::to_value(&active_target).unwrap();
        corrupt_index["digest"] = serde_json::Value::String("f".repeat(64));
        let mut corrupt_index_bytes = serde_json::to_vec_pretty(&corrupt_index).unwrap();
        corrupt_index_bytes.push(b'\n');
        fs::write(&index_path, &corrupt_index_bytes).unwrap();
        let unexpected = store.issue_dir(target.issue).join("unexpected.bin");
        fs::write(&unexpected, b"preserve corrupt target evidence").unwrap();
        let corrupt_snapshot = store
            .snapshot_issue_projection_bytes(target.issue)
            .expect("corrupt snapshot");
        let corrupt_digest = projection_snapshot_digest(&corrupt_snapshot);
        let request = CorruptTerminalReceiptReconciliationRequest {
            authority_issue: authority.issue,
            target_issue: target.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            expected_corrupt_projection_digest: corrupt_digest.clone(),
            expected_initialization_digest: target.initialization_digest.clone(),
            expected_receipt_digest: newer.digest.clone(),
            actor: "codex:test".into(),
            reason: "recover exact corrupt aggregate terminal projection".into(),
            fail_after_stage: None,
        };

        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(request.clone())
                .expect_err("authentic active target checkout must fail closed")
                .code,
            ErrorCode::UnsafeCheckout
        );
        assert!(std::process::Command::new("git")
            .args(["worktree", "remove", "--force"])
            .arg(&active_target_root)
            .current_dir(temp.path())
            .status()
            .expect("remove active target worktree")
            .success());

        let mut forged_claim = request.clone();
        forged_claim.authority_claim_id = "forged".into();
        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(forged_claim)
                .expect_err("forged authority must fail")
                .code,
            ErrorCode::MissingClaim
        );
        let mut stale_projection = request.clone();
        stale_projection.expected_corrupt_projection_digest = "0".repeat(64);
        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(stale_projection)
                .expect_err("stale corrupt projection must fail")
                .code,
            ErrorCode::StaleDigest
        );
        let design_path = store.root.join(&target.design_path);
        let design_bytes = fs::read(&design_path).unwrap();
        fs::write(&design_path, b"drifted aggregate artifact").unwrap();
        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(request.clone())
                .expect_err("authored artifact drift must fail")
                .code,
            ErrorCode::ReconciliationRequired
        );
        fs::write(&design_path, design_bytes).unwrap();

        let mut after_journal = request.clone();
        after_journal.fail_after_stage = Some("after_journal".into());
        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(after_journal)
                .expect_err("injected journal interruption")
                .code,
            ErrorCode::InterruptedTransaction
        );
        store
            .recover_with_terminal_lock(target.issue)
            .expect("recover journal interruption");
        assert_eq!(
            store.snapshot_issue_projection_bytes(target.issue).unwrap(),
            corrupt_snapshot
        );
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap(),
            Some(newer.clone())
        );

        let mut after_projection = request.clone();
        after_projection.fail_after_stage = Some("after_projection".into());
        assert_eq!(
            store
                .reconcile_corrupt_terminal_receipt(after_projection)
                .expect_err("injected projection interruption")
                .code,
            ErrorCode::InterruptedTransaction
        );
        fs::write(
            store.issue_dir(target.issue).join("cards/sip.values.json"),
            b"{corrupt replacement",
        )
        .unwrap();
        store
            .recover_with_terminal_lock(target.issue)
            .expect("rollback corrupt replacement");
        assert_eq!(
            store.snapshot_issue_projection_bytes(target.issue).unwrap(),
            corrupt_snapshot
        );
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap(),
            Some(newer.clone())
        );

        let recovered = store
            .reconcile_corrupt_terminal_receipt(request)
            .expect("recover corrupt terminal projection");
        assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
        assert_eq!(recovered.generation, newer.record.generation + 1);
        assert!(recovered.claim.is_none());
        assert!(recovered
            .audit
            .last()
            .unwrap()
            .operation
            .contains("reconcile_corrupt_terminal_receipt"));
        let retained = store.load_terminal_receipt(target.issue).unwrap().unwrap();
        assert_eq!(retained.record, recovered);
        store
            .verify_materialized_terminal_receipt(&retained)
            .expect("recovered receipt materialized");
        assert!(!unexpected.exists());
    }

    #[test]
    fn terminal_receipt_transport_materializes_strictly_newer_terminal_receipt() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        let local = store
            .load_record(target.issue)
            .expect("older terminal projection");
        let newer = write_newer_terminal_receipt_only(&store, &receipt);
        let request = TerminalReceiptTransportRequest {
            authority_issue: authority.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            actor: "codex:test".into(),
            receipt: newer.clone(),
            fail_after_stage: None,
        };
        let transported = store
            .transport_terminal_receipt(request.clone())
            .expect("forward terminal transport");
        assert!(local.generation < transported.generation);
        assert_eq!(transported, newer.record);
        store
            .verify_materialized_terminal_receipt(&newer)
            .expect("newer terminal authority materialized");
        assert_eq!(
            store
                .transport_terminal_receipt(request)
                .expect("idempotent newer terminal transport"),
            transported
        );
    }

    #[test]
    fn terminal_receipt_transport_rolls_back_newer_terminal_interruption() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        let local = store
            .load_record(target.issue)
            .expect("older terminal projection");
        let local_cards = store
            .load_cards(target.issue)
            .expect("older terminal cards");
        let newer = write_newer_terminal_receipt_only(&store, &receipt);
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: newer.clone(),
                fail_after_stage: Some("after_projection".into()),
            })
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        fs::write(
            store.issue_dir(target.issue).join("cards/spp.md"),
            "corrupt replacement\n",
        )
        .expect("corrupt replacement card");
        store
            .recover_with_terminal_lock(target.issue)
            .expect("rollback interrupted replacement");
        assert_eq!(store.load_record(target.issue).unwrap(), local);
        assert_eq!(store.load_cards(target.issue).unwrap(), local_cards);
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap().unwrap(),
            newer
        );
    }

    #[test]
    fn terminal_receipt_transport_rejects_terminal_downgrade_or_identity_change() {
        for change_terminal in [false, true] {
            let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
            let local = store
                .load_record(target.issue)
                .expect("older terminal projection");
            let newer = write_newer_terminal_receipt_only(&store, &receipt);
            let mut candidate = if change_terminal {
                newer.clone()
            } else {
                receipt.clone()
            };
            if change_terminal {
                candidate.record.terminal.as_mut().unwrap().observed_sha =
                    Some("different-terminal-sha".into());
                candidate.record.digest =
                    record_digest(&candidate.record).expect("changed terminal digest");
                candidate.digest.clear();
                candidate.digest =
                    terminal_receipt_digest(&candidate).expect("changed receipt digest");
                validate_terminal_receipt(&candidate).expect("changed receipt");
                write_json(
                    &store.terminal_receipt_path(target.issue).unwrap(),
                    &candidate,
                )
                .expect("write changed retained receipt");
            }
            let error = store
                .transport_terminal_receipt(TerminalReceiptTransportRequest {
                    authority_issue: authority.issue,
                    expected_authority_generation: authority.generation,
                    expected_authority_digest: authority.digest.clone(),
                    authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                    actor: "codex:test".into(),
                    receipt: candidate,
                    fail_after_stage: None,
                })
                .expect_err("downgrade or terminal identity change must fail");
            assert_eq!(error.code, ErrorCode::ReconciliationRequired);
            assert_eq!(store.load_record(target.issue).unwrap(), local);
        }
    }

    #[test]
    fn terminal_receipt_transport_recovers_after_projection_interruption() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        for path in receipt.authored_artifacts.keys() {
            fs::remove_file(store.root.join(path)).expect("remove authored artifact");
        }
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: receipt.clone(),
                fail_after_stage: Some("after_projection".into()),
            })
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        store
            .recover_with_terminal_lock(target.issue)
            .expect("recover transaction");
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap().unwrap(),
            receipt
        );
        store
            .verify_materialized_terminal_receipt(&receipt)
            .expect("complete recovered transport");
    }

    #[test]
    fn terminal_receipt_transport_corruption_rolls_back_absent_projection_and_artifacts() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        for path in receipt.authored_artifacts.keys() {
            fs::remove_file(store.root.join(path)).expect("remove authored artifact");
        }
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: receipt.clone(),
                fail_after_stage: Some("after_projection".into()),
            })
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        fs::write(
            store.issue_dir(target.issue).join("cards/spp.md"),
            "corrupt\n",
        )
        .expect("corrupt target card");
        store
            .recover_with_terminal_lock(target.issue)
            .expect("rollback corrupt transaction");
        assert!(!store.issue_dir(target.issue).exists());
        assert!(!store.terminal_receipt_path(target.issue).unwrap().exists());
        for path in receipt.authored_artifacts.keys() {
            assert!(!store.root.join(path).exists());
        }
    }

    #[test]
    fn terminal_receipt_transport_artifact_corruption_rolls_back_absent_state() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        for path in receipt.authored_artifacts.keys() {
            fs::remove_file(store.root.join(path)).expect("remove authored artifact");
        }
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: receipt.clone(),
                fail_after_stage: Some("after_projection".into()),
            })
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        let corrupted = receipt
            .authored_artifacts
            .keys()
            .next()
            .expect("authored artifact");
        fs::write(store.root.join(corrupted), "corrupt artifact\n")
            .expect("corrupt target artifact");
        store
            .recover_with_terminal_lock(target.issue)
            .expect("rollback artifact corruption");
        assert!(!store.issue_dir(target.issue).exists());
        assert!(!store.terminal_receipt_path(target.issue).unwrap().exists());
        for path in receipt.authored_artifacts.keys() {
            assert!(!store.root.join(path).exists());
        }
    }

    #[test]
    fn terminal_receipt_transport_idempotency_detects_card_and_artifact_drift() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        let request = TerminalReceiptTransportRequest {
            authority_issue: authority.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            actor: "codex:test".into(),
            receipt: receipt.clone(),
            fail_after_stage: None,
        };
        store
            .transport_terminal_receipt(request.clone())
            .expect("transport");
        fs::write(
            store.issue_dir(target.issue).join("cards/spp.md"),
            "corrupt\n",
        )
        .expect("corrupt card");
        assert_eq!(
            store
                .transport_terminal_receipt(request.clone())
                .expect_err("card drift")
                .code,
            ErrorCode::ReconciliationRequired
        );
        let rendered = render(&receipt.cards[&CardKind::Spp]).expect("render SPP");
        fs::write(
            store.issue_dir(target.issue).join("cards/spp.md"),
            rendered.markdown,
        )
        .expect("restore card");
        fs::remove_file(store.root.join(&receipt.record.design_path)).expect("remove artifact");
        assert_eq!(
            store
                .transport_terminal_receipt(request)
                .expect_err("artifact drift")
                .code,
            ErrorCode::ReconciliationRequired
        );
    }

    #[test]
    fn terminal_receipt_transport_rejects_external_authored_artifact() {
        let (_temp, store, authority, target, mut receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        let content = receipt
            .authored_artifacts
            .remove(&receipt.record.design_path)
            .expect("design");
        receipt
            .authored_artifacts
            .insert("/tmp/external-design.md".into(), content);
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt,
                fail_after_stage: None,
            })
            .expect_err("external artifact");
        assert_eq!(error.code, ErrorCode::InvalidInput);
    }

    #[test]
    fn terminal_receipt_transport_refuses_covered_external_artifact_byte_drift() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_dir_all(store.issue_dir(target.issue)).expect("remove projection");
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        fs::write(store.root.join(&receipt.record.design_path), "local bytes").unwrap();
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt,
                fail_after_stage: None,
            })
            .expect_err("external byte drift must not be overwritten");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }

    fn write_nonterminal_transport_projection(
        store: &Store,
        receipt: &TerminalReceipt,
        generation: u64,
        initialization_digest: Option<&str>,
    ) -> IssueRecord {
        let mut local = receipt.record.clone();
        let reviewed = local
            .transitions
            .iter()
            .position(|event| event.to == LifecyclePhase::Reviewed)
            .expect("reviewed transition");
        local.transitions.truncate(reviewed + 1);
        local.phase = LifecyclePhase::Reviewed;
        local.generation = generation;
        local.claim = None;
        local.publication = None;
        local.readiness = None;
        local.terminal = None;
        if let Some(value) = initialization_digest {
            local.initialization_digest = value.into();
        }
        local.audit.retain(|event| event.generation <= generation);
        let mut cards = receipt.cards.clone();
        for card in cards.values_mut() {
            card.identity.generation = generation;
        }
        hydrate_projections(&mut local, &cards).expect("local projections");
        local.digest = record_digest(&local).expect("local digest");
        store
            .commit(local.issue, &local, &cards, false)
            .expect("write nonterminal projection");
        local
    }

    fn transport_projection_bytes(store: &Store, issue: u64) -> BTreeMap<String, Vec<u8>> {
        let issue_dir = store.issue_dir(issue);
        let mut projection = BTreeMap::new();
        for name in ["index.json", "audit.jsonl"] {
            projection.insert(name.into(), fs::read(issue_dir.join(name)).unwrap());
        }
        for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            for suffix in ["values.json", "md"] {
                let name = format!("cards/{card}.{suffix}");
                projection.insert(name.clone(), fs::read(issue_dir.join(name)).unwrap());
            }
        }
        projection
    }

    #[cfg(unix)]
    #[test]
    fn terminal_receipt_transport_rejects_older_local_artifact_symlink_without_mutation() {
        use std::os::unix::fs::symlink;

        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        write_nonterminal_transport_projection(
            &store,
            &receipt,
            receipt.record.generation - 1,
            None,
        );

        let local_design_path = format!("docs/issue-{}-local-design.md", target.issue);
        let local_design_target = format!("docs/issue-{}-local-design-target.md", target.issue);
        let mut local = store.load_record(target.issue).expect("local record");
        let mut local_cards = store.load_cards(target.issue).expect("local cards");
        local.design_path = local_design_path.clone();
        match &mut local_cards.get_mut(&CardKind::Spp).expect("SPP").content {
            CardContent::Spp(values) => values.design_ref = local_design_path.clone(),
            _ => unreachable!("SPP card"),
        }
        match &mut local_cards.get_mut(&CardKind::Vpp).expect("VPP").content {
            CardContent::Vpp(values) => values.design_ref = local_design_path.clone(),
            _ => unreachable!("VPP card"),
        }
        hydrate_projections(&mut local, &local_cards).expect("local projections");
        local.digest = record_digest(&local).expect("local digest");
        store
            .commit(target.issue, &local, &local_cards, false)
            .expect("write alternate local projection");

        fs::create_dir_all(store.root.join("docs")).expect("create docs directory");
        fs::write(
            store.root.join(&local_design_target),
            receipt
                .authored_artifacts
                .get(&receipt.record.design_path)
                .expect("receipt design"),
        )
        .expect("write symlink target");
        symlink(
            store.root.join(&local_design_target),
            store.root.join(&local_design_path),
        )
        .expect("create local design symlink");

        let before = transport_projection_bytes(&store, target.issue);
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt,
                fail_after_stage: None,
            })
            .expect_err("local authored-artifact symlink must fail closed");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        assert_eq!(transport_projection_bytes(&store, target.issue), before);
        assert!(fs::symlink_metadata(store.root.join(local_design_path))
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(!store
            .terminal_transaction_path(target.issue)
            .unwrap()
            .exists());
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
    }

    #[test]
    fn terminal_receipt_transport_replaces_strictly_older_same_identity_projection() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        let local = write_nonterminal_transport_projection(
            &store,
            &receipt,
            receipt.record.generation - 1,
            None,
        );
        let transported = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: receipt.clone(),
                fail_after_stage: None,
            })
            .expect("forward transport");
        assert!(local.generation < transported.generation);
        assert_eq!(transported, receipt.record);
        assert_eq!(
            store.load_terminal_receipt(target.issue).unwrap().unwrap(),
            receipt
        );
    }

    #[test]
    fn terminal_receipt_transport_preserves_original_projection_and_recovers_interruption() {
        let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
            .expect("remove receipt");
        let local = write_nonterminal_transport_projection(
            &store,
            &receipt,
            receipt.record.generation - 1,
            None,
        );
        let error = store
            .transport_terminal_receipt(TerminalReceiptTransportRequest {
                authority_issue: authority.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest.clone(),
                authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                actor: "codex:test".into(),
                receipt: receipt.clone(),
                fail_after_stage: Some("after_projection".into()),
            })
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        let journal: TerminalTransactionJournal = read_json(
            &store
                .terminal_transaction_path(target.issue)
                .expect("transaction path"),
        )
        .expect("transaction journal");
        assert_eq!(
            journal.original_record_digest.as_deref(),
            Some(local.digest.as_str())
        );
        assert_eq!(
            journal
                .original_projection
                .as_ref()
                .expect("snapshot")
                .record,
            local
        );
        fs::write(
            store.issue_dir(target.issue).join("cards/spp.md"),
            "corrupt replacement\n",
        )
        .expect("corrupt replacement card");
        store
            .recover_with_terminal_lock(target.issue)
            .expect("recover transaction");
        assert_eq!(store.load_record(target.issue).unwrap(), local);
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
        let restored_cards = store.load_cards(target.issue).expect("restored cards");
        verify_cards(&store, &local, &restored_cards).expect("restored projection");
    }

    #[test]
    fn terminal_receipt_transport_rejects_identity_drift_and_generation_downgrade() {
        for (initialization_digest, generation) in [
            (Some("different-initialization"), None),
            (None, Some(0_u64)),
        ] {
            let (_temp, store, authority, target, receipt, _) = terminal_validation_fixture();
            fs::remove_file(store.terminal_receipt_path(target.issue).unwrap())
                .expect("remove receipt");
            let local_generation = generation
                .map(|offset| receipt.record.generation + offset)
                .unwrap_or(receipt.record.generation - 1);
            let local = write_nonterminal_transport_projection(
                &store,
                &receipt,
                local_generation,
                initialization_digest,
            );
            let error = store
                .transport_terminal_receipt(TerminalReceiptTransportRequest {
                    authority_issue: authority.issue,
                    expected_authority_generation: authority.generation,
                    expected_authority_digest: authority.digest.clone(),
                    authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
                    actor: "codex:test".into(),
                    receipt: receipt.clone(),
                    fail_after_stage: None,
                })
                .expect_err("identity drift or downgrade must fail");
            assert_eq!(error.code, ErrorCode::ReconciliationRequired);
            assert_eq!(store.load_record(target.issue).unwrap(), local);
            assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
        }
    }

    fn recordless_fixture() -> (tempfile::TempDir, Store, IssueRecord) {
        let (temp, store, mut authority, _target, _receipt, _) = terminal_validation_fixture();
        let authority_cards = store.load_cards(authority.issue).expect("authority cards");
        authority
            .claim
            .as_mut()
            .expect("authority claim")
            .protected_paths
            .extend([".csdlc/issues/5718/".into(), ".csdlc/issues/5711/".into()]);
        hydrate_projections(&mut authority, &authority_cards).expect("authority projections");
        authority.digest = record_digest(&authority).expect("authority digest");
        store
            .commit(authority.issue, &authority, &authority_cards, false)
            .expect("authority commit");
        (temp, store, authority)
    }

    fn recordless_request(
        authority: &IssueRecord,
        issue: u64,
        closure_kind: RecordlessClosureKind,
        fail_after_stage: Option<&str>,
    ) -> RecordlessTerminalRecoveryRequest {
        let merged = closure_kind == RecordlessClosureKind::Merged;
        let issue_packet = crate::github::GithubIssuePacket {
            schema: "csdlc.github_issue.v1".into(),
            repository: authority.repository.clone(),
            number: issue,
            title: format!("Recordless issue {issue}"),
            body: "closed historical issue".into(),
            state: "closed".into(),
            labels: vec!["version:v0.91.8".into()],
            assignees: Vec::new(),
            milestone: None,
            marker_present: false,
        };
        let issue_evidence = trusted_github(crate::github::GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: authority.repository.clone(),
            action: crate::github::GithubAction::IssueRead,
            operation_key: Some(format!("test:issue:{issue}")),
            issue: Some(issue_packet.clone()),
            comment_id: None,
            pr_state: None,
            reconciled: true,
            producer_digest: None,
        });
        let related_issue_evidence = (!merged).then(|| {
            let packet = crate::github::GithubIssuePacket {
                number: 5702,
                title: "Related issue".into(),
                body: "related authority".into(),
                state: "open".into(),
                ..issue_packet.clone()
            };
            trusted_github(crate::github::GithubActionResult {
                schema: "csdlc.github_action_result.v1".into(),
                repository: authority.repository.clone(),
                action: crate::github::GithubAction::IssueRead,
                operation_key: Some("test:issue:5702".into()),
                issue: Some(packet),
                comment_id: None,
                pr_state: None,
                reconciled: true,
                producer_digest: None,
            })
        });
        let merged_evidence = merged.then(|| {
            trusted_github(crate::github::GithubActionResult {
                schema: "csdlc.github_action_result.v1".into(),
                repository: authority.repository.clone(),
                action: crate::github::GithubAction::PrState,
                operation_key: Some(format!("test:pr:5720:issue:{issue}")),
                issue: None,
                comment_id: None,
                pr_state: Some(crate::github::PrStatePacket {
                    schema: "csdlc.github_pr_state.v1".into(),
                    repository: authority.repository.clone(),
                    pull_request: 5720,
                    linked_issue: Some(issue),
                    linkage_source: Some("github_closing_issues_references".into()),
                    draft: false,
                    merge_state: "unknown".into(),
                    review_decision: "approved".into(),
                    base_ref: Some("main".into()),
                    head_ref: Some(format!("codex/{issue}-recordless-source")),
                    head_sha: "92fde26a2ca073e204459fce1bb5e88d7c895528".into(),
                    url: Some(format!(
                        "https://github.com/{}/pull/5720",
                        authority.repository
                    )),
                    body: Some(format!("Closes #{issue}")),
                    merged: true,
                    merge_commit_sha: Some("4d68d4b1f4f70c15223ebdf71d59c9010e5e3d4c".into()),
                    checks: Vec::new(),
                    required_check_names: Vec::new(),
                    classification: "merged".into(),
                }),
                reconciled: true,
                producer_digest: None,
            })
        });
        RecordlessTerminalRecoveryRequest {
            authority_issue: authority.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            actor: "codex:test".into(),
            issue: issue_packet,
            issue_evidence,
            closure_kind,
            merged_evidence,
            related_issue: (!merged).then_some(5702),
            related_issue_evidence,
            reason: if merged { "Retain exact observed closure without reconstructing lifecycle history." } else { "Retain exact observed closure related to #5702 without reconstructing lifecycle history." }.into(),
            validation: ValidationResult {
                command: if merged {
                    vec!["git".into(), "merge-base".into(), "--is-ancestor".into()]
                } else {
                    vec!["csdlc-github-issue".into(), "run".into()]
                },
                purpose: "Validate exact terminal evidence.".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: format!("issue-{issue}:typed-terminal-observation"),
            },
            fail_after_stage: fail_after_stage.map(str::to_owned),
        }
    }

    #[test]
    fn recordless_merged_recovery_creates_receipt_without_lifecycle_claims() {
        let (_temp, store, authority) = recordless_fixture();
        let recovered = store
            .recover_recordless_terminal(recordless_request(
                &authority,
                5718,
                RecordlessClosureKind::Merged,
                None,
            ))
            .expect("recordless recovery");
        assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
        assert!(
            recovered.claim.is_none()
                && recovered.review.is_none()
                && recovered.publication.is_none()
                && recovered.readiness.is_none()
        );
        assert_eq!(recovered.transitions.len(), 1);
        assert_eq!(recovered.audit[0].operation, "recover_recordless_terminal");
        assert_eq!(
            recovered.terminal.as_ref().unwrap().pull_request,
            Some(5720)
        );
        assert_eq!(
            store.load_terminal_receipt(5718).unwrap().unwrap().record,
            recovered
        );
    }

    #[test]
    fn recordless_merged_recovery_rejects_unreconciled_or_mismatched_pr_evidence() {
        let (_temp, store, authority) = recordless_fixture();
        let base = recordless_request(&authority, 5718, RecordlessClosureKind::Merged, None);

        let mut unreconciled = base.clone();
        unreconciled
            .merged_evidence
            .as_mut()
            .expect("merged evidence")
            .reconciled = false;
        assert_eq!(
            store
                .recover_recordless_terminal(unreconciled)
                .expect_err("unreconciled PR evidence")
                .code,
            ErrorCode::InvalidInput
        );

        let mut wrong_linkage = base.clone();
        wrong_linkage
            .merged_evidence
            .as_mut()
            .and_then(|evidence| evidence.pr_state.as_mut())
            .expect("PR state")
            .linked_issue = Some(9999);
        assert_eq!(
            store
                .recover_recordless_terminal(wrong_linkage)
                .expect_err("wrong issue linkage")
                .code,
            ErrorCode::InvalidInput
        );

        let mut wrong_url = base;
        wrong_url
            .merged_evidence
            .as_mut()
            .and_then(|evidence| evidence.pr_state.as_mut())
            .expect("PR state")
            .url = Some("https://github.com/example/other/pull/5720".into());
        assert_eq!(
            store
                .recover_recordless_terminal(wrong_url)
                .expect_err("noncanonical URL")
                .code,
            ErrorCode::InvalidInput
        );
    }

    #[test]
    fn recordless_merged_request_rejects_legacy_raw_pr_fields() {
        let (_temp, _store, authority) = recordless_fixture();
        let request = recordless_request(&authority, 5718, RecordlessClosureKind::Merged, None);
        let mut value = serde_json::to_value(request).expect("request JSON");
        value
            .as_object_mut()
            .expect("request object")
            .insert("pull_request".into(), serde_json::json!(5720));
        assert!(serde_json::from_value::<RecordlessTerminalRecoveryRequest>(value).is_err());
    }

    #[test]
    fn recordless_recovery_rejects_hand_constructed_serialized_evidence() {
        let (_temp, store, authority) = recordless_fixture();
        let trusted = recordless_request(&authority, 5718, RecordlessClosureKind::Merged, None);
        let mut mutated = trusted.clone();
        mutated
            .merged_evidence
            .as_mut()
            .and_then(|evidence| evidence.pr_state.as_mut())
            .expect("merged PR state")
            .head_sha = "0".repeat(40);
        assert!(!mutated
            .merged_evidence
            .as_ref()
            .unwrap()
            .is_producer_verified());
        assert_eq!(
            store
                .recover_recordless_terminal(mutated)
                .expect_err("mutated producer result is not producer-bound")
                .code,
            ErrorCode::InvalidInput
        );
        let forged: RecordlessTerminalRecoveryRequest =
            serde_json::from_value(serde_json::to_value(&trusted).unwrap()).unwrap();
        assert!(!forged.issue_evidence.is_producer_verified());
        assert!(!forged
            .merged_evidence
            .as_ref()
            .unwrap()
            .is_producer_verified());
        assert_eq!(
            store
                .recover_recordless_terminal(forged)
                .expect_err("serialized evidence is not producer-bound")
                .code,
            ErrorCode::InvalidInput
        );
    }

    #[test]
    fn historical_merged_reconciliation_rolls_back_corrupt_interrupted_projection() {
        let (temp, store, authority, mut target, _receipt, validation) =
            terminal_validation_fixture();
        let mut cards = store.load_cards(target.issue).unwrap();
        let published = target
            .transitions
            .iter()
            .position(|event| event.to == LifecyclePhase::Published)
            .unwrap();
        target.transitions.truncate(published + 1);
        target.phase = LifecyclePhase::Published;
        target.readiness = None;
        target.terminal = None;
        target.claim = None;
        let CardContent::Sor(sor) = &mut cards.get_mut(&CardKind::Sor).unwrap().content else {
            unreachable!()
        };
        sor.integration_state = crate::cards::IntegrationState::PrOpen;
        sor.publication_state = crate::cards::PublicationState::Ready;
        sor.merge_state = crate::cards::MergeState::NotMerged;
        sor.closeout_state = crate::cards::CloseoutState::NotStarted;
        hydrate_projections(&mut target, &cards).unwrap();
        target.digest = record_digest(&target).unwrap();
        store.commit(target.issue, &target, &cards, false).unwrap();
        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap()).unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "test@example.invalid"])
            .current_dir(temp.path())
            .status()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "C-SDLC Test"])
            .current_dir(temp.path())
            .status()
            .unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .status()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "historical source"])
            .current_dir(temp.path())
            .status()
            .unwrap();
        let reviewed_commit = crate::git::run(temp.path(), &["rev-parse", "HEAD"])
            .unwrap()
            .stdout;
        let mut review = target.review.clone().unwrap();
        review.reviewed_revision = format!("git-blake3:{reviewed_commit}:review");
        let issue_packet = crate::github::GithubIssuePacket {
            schema: "csdlc.github_issue.v1".into(),
            repository: target.repository.clone(),
            number: target.issue,
            title: "historical target".into(),
            body: "closed".into(),
            state: "closed".into(),
            labels: vec![],
            assignees: vec![],
            milestone: None,
            marker_present: false,
        };
        let issue_evidence = trusted_github(crate::github::GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: target.repository.clone(),
            action: crate::github::GithubAction::IssueRead,
            operation_key: None,
            issue: Some(issue_packet),
            comment_id: None,
            pr_state: None,
            reconciled: true,
            producer_digest: None,
        });
        let merged_evidence = trusted_github(crate::github::GithubActionResult {
            schema: "csdlc.github_action_result.v1".into(),
            repository: target.repository.clone(),
            action: crate::github::GithubAction::PrState,
            operation_key: None,
            issue: None,
            comment_id: None,
            pr_state: Some(crate::github::PrStatePacket {
                schema: "csdlc.github_pr_state.v1".into(),
                repository: target.repository.clone(),
                pull_request: 5638,
                linked_issue: Some(target.issue),
                linkage_source: Some("github_closing_issues_references".into()),
                draft: false,
                merge_state: "unknown".into(),
                review_decision: "approved".into(),
                base_ref: Some("main".into()),
                head_ref: Some("codex/historical".into()),
                head_sha: reviewed_commit.clone(),
                url: Some(format!(
                    "https://github.com/{}/pull/5638",
                    target.repository
                )),
                body: Some(format!("Closes #{}", target.issue)),
                merged: true,
                merge_commit_sha: Some("4d68d4b1f4f70c15223ebdf71d59c9010e5e3d4c".into()),
                checks: vec![],
                required_check_names: vec![],
                classification: "merged".into(),
            }),
            reconciled: true,
            producer_digest: None,
        });
        let mut request = HistoricalMergedReconciliationRequest {
            authority_issue: authority.issue,
            expected_authority_generation: authority.generation,
            expected_authority_digest: authority.digest.clone(),
            authority_claim_id: authority.claim.as_ref().unwrap().id.clone(),
            target_issue: target.issue,
            expected_target_generation: target.generation,
            expected_target_digest: target.digest.clone(),
            expected_initialization_digest: target.initialization_digest.clone(),
            reviewed_commit,
            review,
            issue_evidence,
            merged_evidence,
            actor: "codex:test".into(),
            operator_authority: "test authority".into(),
            reason: "historical recovery rollback proof".into(),
            validation,
            fail_after_stage: Some("after_projection".into()),
        };
        assert_eq!(
            store
                .reconcile_historical_merged(request.clone())
                .expect_err("injected interruption")
                .code,
            ErrorCode::InterruptedTransaction
        );
        fs::write(
            store.issue_dir(target.issue).join("cards/sip.values.json"),
            b"{corrupt",
        )
        .unwrap();
        store.recover_with_terminal_lock(target.issue).unwrap();
        assert_eq!(store.load_record(target.issue).unwrap(), target);
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());

        let source_root = temp.path().join("reviewed-source");
        std::process::Command::new("git")
            .args([
                "worktree",
                "add",
                "-b",
                "reviewed-source",
                source_root.to_str().unwrap(),
                &request.reviewed_commit,
            ])
            .current_dir(temp.path())
            .status()
            .unwrap()
            .success()
            .then_some(())
            .expect("source worktree");
        let source_store = Store::new(&source_root);
        let mut source = source_store.load_record(target.issue).unwrap();
        let mut source_cards = source_store.load_cards(target.issue).unwrap();
        request.review.reviewed_revision =
            crate::git::substantive_revision(source_store.root(), &request.review.scope).unwrap();
        source.phase = LifecyclePhase::Reviewed;
        source.claim = None;
        source.publication = None;
        source.readiness = None;
        source.terminal = None;
        source.review = Some(request.review.clone());
        while source
            .transitions
            .last()
            .is_some_and(|transition| transition.to != LifecyclePhase::Reviewed)
        {
            source.transitions.pop();
        }
        let CardContent::Srp(srp) = &mut source_cards.get_mut(&CardKind::Srp).expect("SRP").content
        else {
            unreachable!()
        };
        srp.reviewer = Some(request.review.reviewer.clone());
        srp.review_scope = request.review.scope.join("\n");
        srp.review_revision = Some(request.review.reviewed_revision.clone());
        srp.review_result = crate::cards::ReviewResult::Pass;
        let CardContent::Sor(sor) = &mut source_cards.get_mut(&CardKind::Sor).expect("SOR").content
        else {
            unreachable!()
        };
        sor.integration_state = crate::cards::IntegrationState::WorktreeOnly;
        sor.publication_state = crate::cards::PublicationState::NotPublished;
        sor.merge_state = crate::cards::MergeState::NotMerged;
        sor.closeout_state = crate::cards::CloseoutState::NotStarted;
        hydrate_projections(&mut source, &source_cards).unwrap();
        source.digest = record_digest(&source).unwrap();
        source_store
            .commit(target.issue, &source, &source_cards, false)
            .unwrap();

        let mut rehomed = source.clone();
        rehomed.claim = Some(Claim {
            id: "historical-rehome".into(),
            owner: "recovery-session".into(),
            generation: source.generation,
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "issue-7".into(),
            worktree: ".".into(),
            protected_paths: vec![format!(".csdlc/issues/{}", target.issue)],
            purpose: "historical finalize".into(),
        });
        rehomed.audit.push(AuditEvent {
            sequence: rehomed.audit.len() as u64 + 1,
            generation: rehomed.generation,
            actor: "recovery-session".into(),
            reason: "test exact rehome lineage".into(),
            operation: serde_json::json!({
                "operation": "rehome_claim_authority",
                "source_worktree": source_root.to_string_lossy(),
                "source_branch": "reviewed-source",
                "source_commit": request.reviewed_commit,
                "source_generation": source.generation,
                "source_digest": source.digest,
            })
            .to_string(),
        });
        hydrate_projections(&mut rehomed, &source_cards).unwrap();
        rehomed.digest = record_digest(&rehomed).unwrap();
        store
            .commit(target.issue, &rehomed, &source_cards, false)
            .unwrap();

        request.expected_target_generation = rehomed.generation;
        request.expected_target_digest = rehomed.digest.clone();
        request.fail_after_stage = Some("after_projection".into());
        let interrupted = store
            .reconcile_historical_merged(request.clone())
            .expect_err("rehomed finalize interruption");
        assert_eq!(
            interrupted.code,
            ErrorCode::InterruptedTransaction,
            "{}",
            interrupted.message
        );
        fs::write(
            store.issue_dir(target.issue).join("cards/sip.values.json"),
            b"{corrupt",
        )
        .unwrap();
        store.recover_with_terminal_lock(target.issue).unwrap();
        assert_eq!(store.load_record(target.issue).unwrap(), rehomed);
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());

        let source_audit = source_store.issue_dir(target.issue).join("audit.jsonl");
        let source_audit_bytes = fs::read(&source_audit).unwrap();
        fs::write(&source_audit, b"{}\n").unwrap();
        request.fail_after_stage = None;
        assert_eq!(
            store
                .reconcile_historical_merged(request.clone())
                .expect_err("source audit drift")
                .code,
            ErrorCode::CorruptRecord
        );
        fs::write(&source_audit, source_audit_bytes).unwrap();
        let closed = store
            .reconcile_historical_merged(request.clone())
            .expect("exact rehomed finalize");
        assert_eq!(closed.phase, LifecyclePhase::ClosedOut);
        assert!(closed.claim.is_none());
        assert_eq!(
            store
                .load_terminal_receipt(target.issue)
                .unwrap()
                .unwrap()
                .record,
            closed
        );

        fs::remove_file(store.terminal_receipt_path(target.issue).unwrap()).unwrap();
        let aggregate_root = temp.path().join("aggregate-authority");
        std::process::Command::new("git")
            .args([
                "worktree",
                "add",
                "-b",
                "aggregate-authority",
                aggregate_root.to_str().unwrap(),
                "HEAD",
            ])
            .current_dir(temp.path())
            .status()
            .unwrap()
            .success()
            .then_some(())
            .expect("aggregate authority worktree");
        let aggregate_store = Store::new(&aggregate_root);
        let mut aggregate_authority = aggregate_store.load_record(authority.issue).unwrap();
        let aggregate_cards = aggregate_store.load_cards(authority.issue).unwrap();
        let aggregate_claim = aggregate_authority.claim.as_mut().unwrap();
        aggregate_claim.branch = "aggregate-authority".into();
        aggregate_claim.worktree = "aggregate-authority".into();
        aggregate_claim.protected_paths.push("csdlc-v2".into());
        aggregate_claim
            .protected_paths
            .push(format!(".csdlc/issues/{}", aggregate_authority.issue));
        aggregate_claim.protected_paths.sort();
        aggregate_claim.protected_paths.dedup();
        hydrate_projections(&mut aggregate_authority, &aggregate_cards).unwrap();
        aggregate_authority.digest = record_digest(&aggregate_authority).unwrap();
        assert!(claim_worktree_matches_store(
            &aggregate_store,
            aggregate_authority.claim.as_ref().unwrap()
        )
        .unwrap());
        let mut mismatched_worktree = aggregate_authority.claim.clone().unwrap();
        mismatched_worktree.worktree = "different-worktree".into();
        assert!(!claim_worktree_matches_store(&aggregate_store, &mismatched_worktree).unwrap());
        aggregate_store
            .commit(
                aggregate_authority.issue,
                &aggregate_authority,
                &aggregate_cards,
                false,
            )
            .unwrap();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let mut corrupt_index: serde_json::Value =
            serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
        let target_claim = Claim {
            id: "corrupt-target-claim".into(),
            owner: "target-session".into(),
            generation: corrupt_index["generation"].as_u64().unwrap(),
            acquired_unix_seconds: 1,
            expires_unix_seconds: u64::MAX,
            heartbeat_unix_seconds: 1,
            branch: "main".into(),
            worktree: ".".into(),
            protected_paths: request
                .review
                .scope
                .iter()
                .cloned()
                .chain(std::iter::once(format!(".csdlc/issues/{}", target.issue)))
                .collect(),
            purpose: "exact corrupt target authority".into(),
        };
        corrupt_index["claim"] = serde_json::to_value(&target_claim).unwrap();
        corrupt_index["digest"] = serde_json::Value::String("f".repeat(64));
        let mut corrupt_bytes = serde_json::to_vec_pretty(&corrupt_index).unwrap();
        corrupt_bytes.push(b'\n');
        fs::write(&index_path, &corrupt_bytes).unwrap();
        let unexpected_path = store
            .issue_dir(target.issue)
            .join("unexpected-evidence.bin");
        fs::write(&unexpected_path, b"preserve unexpected evidence").unwrap();
        let corrupt_digest = store.corrupt_projection_digest(target.issue).unwrap();
        let mut corrupt_merged_evidence = request.merged_evidence.clone();
        let corrupt_pr = corrupt_merged_evidence.pr_state.as_mut().unwrap();
        corrupt_pr.required_check_names = vec!["ci".into()];
        corrupt_pr.checks = vec![crate::github::PrCheck {
            name: "ci".into(),
            required: true,
            conclusion: "success".into(),
            details_url: Some("https://example.invalid/check/ci".into()),
        }];
        corrupt_merged_evidence = trusted_github(corrupt_merged_evidence);
        let corrupt_request = CorruptHistoricalMergedRecoveryRequest {
            authority_issue: request.authority_issue,
            authority_worktree: aggregate_root.to_string_lossy().into_owned(),
            expected_authority_generation: aggregate_authority.generation,
            expected_authority_digest: aggregate_authority.digest,
            authority_claim_id: aggregate_authority.claim.unwrap().id,
            target_issue: request.target_issue,
            source_commit: request.reviewed_commit.clone(),
            expected_source_generation: target.generation,
            expected_source_digest: target.digest.clone(),
            expected_initialization_digest: target.initialization_digest.clone(),
            expected_corrupt_projection_digest: corrupt_digest,
            expected_target_claim: target_claim,
            required_checks: vec!["ci".into()],
            require_review: true,
            reviewed_commit: request.reviewed_commit,
            review: request.review,
            issue_evidence: request.issue_evidence,
            merged_evidence: corrupt_merged_evidence,
            actor: "codex:test".into(),
            operator_authority: "test corrupt historical recovery authority".into(),
            reason: "prove exact corrupt projection recovery".into(),
            validation: request.validation,
            fail_after_stage: Some("after_journal".into()),
        };
        let mut forged_claim = corrupt_request.clone();
        forged_claim.expected_target_claim.owner = "forged-owner".into();
        assert_eq!(
            store
                .recover_corrupt_historical_merged(forged_claim)
                .expect_err("forged target claim must fail closed")
                .code,
            ErrorCode::InvalidClaim
        );
        let authored_path = temp.path().join(&target.design_path);
        let authored_bytes = fs::read(&authored_path).unwrap();
        fs::write(&authored_path, b"drifted authored artifact").unwrap();
        assert_eq!(
            store
                .recover_corrupt_historical_merged(corrupt_request.clone())
                .expect_err("authored artifact drift must fail closed")
                .code,
            ErrorCode::ReconciliationRequired
        );
        fs::write(&authored_path, authored_bytes).unwrap();
        let sip_path = store.issue_dir(target.issue).join("cards/sip.md");
        let sip_bytes = fs::read(&sip_path).unwrap();
        let mut drifted_sip = sip_bytes.clone();
        drifted_sip.extend_from_slice(b"\ndrift\n");
        fs::write(&sip_path, drifted_sip).unwrap();
        assert_eq!(
            store
                .recover_corrupt_historical_merged(corrupt_request.clone())
                .expect_err("corrupt projection CAS must cover rendered cards")
                .code,
            ErrorCode::StaleDigest
        );
        fs::write(&sip_path, sip_bytes).unwrap();
        assert_eq!(
            store
                .recover_corrupt_historical_merged(corrupt_request.clone())
                .expect_err("injected pre-projection corrupt recovery interruption")
                .code,
            ErrorCode::InterruptedTransaction
        );
        store.recover_with_terminal_lock(target.issue).unwrap();
        assert_eq!(fs::read(&index_path).unwrap(), corrupt_bytes);
        assert_eq!(
            fs::read(&unexpected_path).unwrap(),
            b"preserve unexpected evidence"
        );
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
        let mut after_projection = corrupt_request.clone();
        after_projection.fail_after_stage = Some("after_projection".into());
        assert_eq!(
            store
                .recover_corrupt_historical_merged(after_projection)
                .expect_err("injected corrupt recovery interruption")
                .code,
            ErrorCode::InterruptedTransaction
        );
        fs::write(
            store.issue_dir(target.issue).join("cards/sip.values.json"),
            b"{interrupted",
        )
        .unwrap();
        store.recover_with_terminal_lock(target.issue).unwrap();
        assert_eq!(fs::read(&index_path).unwrap(), corrupt_bytes);
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_none());
        let mut retry = corrupt_request;
        retry.fail_after_stage = None;
        let recovered = store
            .recover_corrupt_historical_merged(retry)
            .expect("recover exact corrupt historical projection");
        assert_eq!(recovered.phase, LifecyclePhase::ClosedOut);
        assert!(store.load_terminal_receipt(target.issue).unwrap().is_some());
    }

    #[test]
    fn identity_repair_cannot_bless_a_corrupt_projection() {
        let (_temp, store, authority, target, _receipt, _validation) =
            terminal_validation_fixture();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let mut corrupt: IssueRecord =
            serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
        corrupt.digest = "f".repeat(64);
        let mut bytes = serde_json::to_vec_pretty(&corrupt).unwrap();
        bytes.push(b'\n');
        fs::write(index_path, bytes).unwrap();
        let error = store
            .repair_identity(RepairIdentityRequest {
                authority_issue: authority.issue,
                target_issue: target.issue,
                expected_authority_generation: authority.generation,
                expected_authority_digest: authority.digest,
                expected_target_generation: corrupt.generation,
                expected_target_digest: corrupt.digest,
                claim_id: authority.claim.unwrap().id,
                actor: "codex:test".into(),
                operation: SemanticOperation::UpdateIdentityVersion {
                    version: "v0.91.8".into(),
                },
            })
            .expect_err("identity repair must reject corrupt target authority");
        assert_eq!(error.code, ErrorCode::CorruptRecord);
    }

    #[cfg(unix)]
    #[test]
    fn terminal_transaction_journal_rejects_parent_leaf_and_temp_symlinks() {
        use std::os::unix::fs::symlink;

        let (_temp, store, _authority, target, receipt, _validation) =
            terminal_validation_fixture();
        let path = store.terminal_transaction_path(target.issue).unwrap();
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        let outside = tempfile::tempdir().unwrap();
        let outside_file = outside.path().join("outside");
        fs::write(&outside_file, b"outside").unwrap();
        let journal = TerminalTransactionJournal {
            schema: "csdlc.terminal_transaction.v1".into(),
            origin_worktree: String::new(),
            origin_git_common_dir: String::new(),
            issue: target.issue,
            stage: "symlink-proof".into(),
            original_record_digest: Some(target.digest.clone()),
            original_projection_digest: None,
            target_record_digest: target.digest,
            original_receipt: None,
            original_projection: None,
            original_artifacts: BTreeMap::new(),
            target_receipt: serde_json::to_vec_pretty(&receipt).unwrap(),
        };

        symlink(&outside_file, &path).unwrap();
        assert_eq!(
            store
                .write_terminal_transaction_journal(&journal)
                .unwrap_err()
                .code,
            ErrorCode::UnsafeCheckout
        );
        fs::remove_file(&path).unwrap();

        let temporary = path.with_extension("json.transaction-tmp");
        symlink(&outside_file, &temporary).unwrap();
        assert_eq!(
            store
                .write_terminal_transaction_journal(&journal)
                .unwrap_err()
                .code,
            ErrorCode::UnsafeCheckout
        );
        fs::remove_file(&temporary).unwrap();

        let backup = parent.with_extension("journal-parent-backup");
        fs::rename(parent, &backup).unwrap();
        symlink(&backup, parent).unwrap();
        assert_eq!(
            store
                .write_terminal_transaction_journal(&journal)
                .unwrap_err()
                .code,
            ErrorCode::UnsafeCheckout
        );
        fs::remove_file(parent).unwrap();
        fs::rename(backup, parent).unwrap();
    }

    #[test]
    fn recordless_terminal_recovery_is_idempotent_and_rejects_conflict() {
        let (_temp, store, authority) = recordless_fixture();
        let request = recordless_request(&authority, 5718, RecordlessClosureKind::Merged, None);
        let first = store
            .recover_recordless_terminal(request.clone())
            .expect("first recovery");
        let replay = store
            .recover_recordless_terminal(request.clone())
            .expect("exact replay");
        assert_eq!(replay, first);
        let mut conflict = request;
        conflict.reason.push_str(" conflicting assertion");
        assert_eq!(
            store
                .recover_recordless_terminal(conflict)
                .expect_err("conflict")
                .code,
            ErrorCode::ReconciliationRequired
        );
    }

    #[test]
    fn recordless_duplicate_recovery_recovers_after_projection_interruption() {
        let (_temp, store, authority) = recordless_fixture();
        let error = store
            .recover_recordless_terminal(recordless_request(
                &authority,
                5711,
                RecordlessClosureKind::Duplicate,
                Some("after_projection"),
            ))
            .expect_err("interrupted");
        assert_eq!(error.code, ErrorCode::InterruptedTransaction);
        store
            .recover_with_terminal_lock(5711)
            .expect("recover transaction");
        let recovered = store.load_record(5711).expect("projection");
        assert_eq!(
            store.load_terminal_receipt(5711).unwrap().unwrap().record,
            recovered
        );
        assert_eq!(
            recovered.terminal.as_ref().unwrap().disposition,
            crate::readiness::TerminalDisposition::ClosedNoPr
        );
    }

    #[test]
    fn recordless_duplicate_requires_passed_related_issue_provenance() {
        let (_temp, store, authority) = recordless_fixture();
        let mut request =
            recordless_request(&authority, 5711, RecordlessClosureKind::Duplicate, None);
        request.validation.outcome = crate::cards::EvidenceOutcome::SkippedNonGoal;
        assert_eq!(
            store
                .recover_recordless_terminal(request.clone())
                .expect_err("passing evidence")
                .code,
            ErrorCode::InvalidInput
        );
        request.validation.outcome = crate::cards::EvidenceOutcome::Passed;
        request
            .related_issue_evidence
            .as_mut()
            .expect("related evidence")
            .reconciled = false;
        assert_eq!(
            store
                .recover_recordless_terminal(request)
                .expect_err("provenance")
                .code,
            ErrorCode::InvalidInput
        );
    }

    #[test]
    fn terminal_sor_validation_repair_rejects_stale_receipt_without_mutation() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        let original_index = fs::read(&index_path).expect("index bytes");
        let original_receipt = fs::read(&receipt_path).expect("receipt bytes");
        let mut request = validation_repair_request(&authority, &target, &receipt, expected, None);
        request.expected_receipt_digest = "stale".into();
        let error = store
            .repair_terminal_sor_validation(request)
            .expect_err("stale receipt must fail");
        assert_eq!(error.code.to_string(), "stale_digest");
        assert_eq!(fs::read(index_path).expect("index bytes"), original_index);
        assert_eq!(
            fs::read(receipt_path).expect("receipt bytes"),
            original_receipt
        );
    }

    #[test]
    fn terminal_sor_validation_repair_rolls_back_projection_and_receipt() {
        let (_temp, store, authority, target, receipt, expected) = terminal_validation_fixture();
        let index_path = store.issue_dir(target.issue).join("index.json");
        let receipt_path = store
            .terminal_receipt_path(target.issue)
            .expect("receipt path");
        let original_index = fs::read(&index_path).expect("index bytes");
        let original_receipt = fs::read(&receipt_path).expect("receipt bytes");
        let request = validation_repair_request(
            &authority,
            &target,
            &receipt,
            expected,
            Some("after_projection"),
        );
        let error = store
            .repair_terminal_sor_validation(request)
            .expect_err("injected failure must roll back");
        assert_eq!(error.code.to_string(), "interrupted_transaction");
        assert_eq!(fs::read(index_path).expect("index bytes"), original_index);
        assert_eq!(
            fs::read(receipt_path).expect("receipt bytes"),
            original_receipt
        );
    }

    #[test]
    fn terminal_sor_validation_repair_enforces_portable_replacements() {
        for machine_local in [
            "/tmp/build",
            "--target-dir=/home/alice/build",
            "cd /mnt/worker/checkout",
            r"C:\Users\alice\checkout",
            r"--out=Z:\build\target",
            r"\\server\share\checkout",
            "~/checkout",
            "CARGO_TARGET_DIR=$HOME/build",
            "CARGO_TARGET_DIR=${HOME}/build",
            "sh -c 'cd ${HOME}/checkout'",
            "$(pwd)/target",
            "`pwd`/target",
            "file:///home/alice/proof.json",
            r"%USERPROFILE%\checkout",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into(), machine_local.into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: "evidence/portable.json".into(),
            };
            let error = validate_portable_validation_result(&result).expect_err(machine_local);
            assert_eq!(error.code.to_string(), "invalid_input");
        }

        for machine_local in [
            "proof\u{a0}/home/alice/out",
            "proof=[/home/alice/out]",
            "echo proof >/tmp/result",
            "tool 2>/home/alice/log",
            "cmd|/opt/local/tool",
            r"type NUL >C:\Users\alice\proof",
            r"proof&\\server\share\result",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: machine_local.into(),
            };
            validate_portable_validation_result(&result).expect_err(machine_local);
        }

        for portable in [
            "evidence/portable.json",
            "evidence/report$final.json",
            "--target-dir=target/coverage",
            "https://example.invalid/proof",
            "https://example.invalid/proof?$select=id",
            "retained terminal receipt",
        ] {
            let result = ValidationResult {
                command: vec!["proof".into(), portable.into()],
                purpose: "proof".into(),
                outcome: crate::cards::EvidenceOutcome::Passed,
                evidence_ref: portable.into(),
            };
            validate_portable_validation_result(&result).expect(portable);
        }

        let symbolic_result = ValidationResult {
            command: vec!["proof".into()],
            purpose: "proof".into(),
            outcome: crate::cards::EvidenceOutcome::Passed,
            evidence_ref: "reviewed `proof command`".into(),
        };
        validate_portable_validation_result(&symbolic_result).expect("stable symbolic evidence");

        let result = ValidationResult {
            command: vec!["proof".into()],
            purpose: "proof".into(),
            outcome: crate::cards::EvidenceOutcome::Passed,
            evidence_ref: "/home/runner/evidence.json".into(),
        };
        validate_portable_validation_result(&result).expect_err("machine-local evidence reference");
    }
}
