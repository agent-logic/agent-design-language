use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::cards::{
    digest, render, CardContent, CardKind, InitialCardInput, PlanStep, StepStatus, ValidationLane,
};
use crate::error::{ErrorCode, Result, V2Error};
use crate::lifecycle::initialize_issue;
use crate::model::{AuditEvent, IssueRecord, TerminalEvidence, TransitionEvent};
use crate::model::{LifecyclePhase, MigrationEvidence};
use crate::readiness::TerminalDisposition;
use crate::{
    edit_issue, BootstrapRequest, CardStatus, EditRequest, PlanningProfile, SemanticOperation,
    Store,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ImportStatus {
    Imported,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LegacyImportRequest {
    pub schema: String,
    pub legacy_root: PathBuf,
    pub output_root: PathBuf,
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
    pub card_paths: BTreeMap<CardKind, String>,
    pub design_path: String,
    pub diagram_path: String,
    pub design_reviewer: String,
    pub actor: String,
    pub planning_profile: PlanningProfile,
    pub validation_lanes: Vec<ValidationLane>,
    pub imported_unix_seconds: u64,
    pub default_cutover_unix_seconds: u64,
    pub legacy_phase: LifecyclePhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MigrationDiagnostic {
    pub card: Option<CardKind>,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ImportReport {
    pub schema: String,
    pub status: ImportStatus,
    pub issue: u64,
    pub source_digest: Option<String>,
    pub retained_section_count: usize,
    pub diagnostics: Vec<MigrationDiagnostic>,
    pub compatibility_view: Option<String>,
    pub sunset_unix_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrationIssueState {
    Open,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ClosedIssueEvidence {
    pub pull_request: Option<u64>,
    pub disposition: TerminalDisposition,
    pub observed_sha: Option<String>,
    pub observed_state: String,
    pub receipt_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BoundTopologyMigrationItem {
    pub issue: u64,
    pub state: MigrationIssueState,
    pub terminal: Option<ClosedIssueEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BoundTopologyMigrationRequest {
    pub schema: String,
    pub apply: bool,
    pub actor: String,
    pub issue_state_evidence: String,
    pub issues: Vec<BoundTopologyMigrationItem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BoundTopologyDisposition {
    ResetInitialized,
    AdoptedVerifiedTopology,
    ClosedOut,
    AlreadyCurrent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BoundTopologyMigrationResult {
    pub issue: u64,
    pub disposition: BoundTopologyDisposition,
    pub source_digest: String,
    pub normalized_digest: String,
    pub resulting_digest: String,
    pub branch: Option<String>,
    pub worktree: Option<String>,
    pub terminal_evidence_digest: Option<String>,
    pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BoundTopologyMigrationReport {
    pub schema: String,
    pub applied: bool,
    pub changed: usize,
    pub issue_state_evidence: String,
    pub issue_state_evidence_digest: String,
    pub results: Vec<BoundTopologyMigrationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeRepositoryMigrationRequest {
    pub schema: String,
    pub issue: u64,
    pub code_repository: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeRepositoryMigrationEvidence {
    pub schema: String,
    pub issue: u64,
    pub actor: String,
    pub reason: String,
    pub pre_generation: u64,
    pub pre_digest: String,
    pub previous_code_repository: Option<String>,
    pub requested_repository: String,
    pub fetch_repositories: Vec<String>,
    pub push_repositories: Vec<String>,
    pub phase: LifecyclePhase,
    pub branch: String,
    pub worktree: String,
    pub clean_worktree: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeRepositoryMigrationReport {
    pub schema: String,
    pub issue: u64,
    pub changed: bool,
    pub phase: LifecyclePhase,
    pub branch: String,
    pub worktree: String,
    pub code_repository: String,
    pub resulting_generation: u64,
    pub resulting_digest: String,
    pub evidence: CodeRepositoryMigrationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InitializedCanonicalCollisionDisposition {
    SameNumberAbsent,
    SameNumberNonAuthoritative,
    SameNumberSuccessor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InitializedCodeRepositoryCollisionEvidence {
    pub schema: String,
    pub source_issue_repository: String,
    pub source_issue: u64,
    pub target_code_repository: String,
    pub target_same_number_issue: Option<u64>,
    pub disposition: InitializedCanonicalCollisionDisposition,
    pub observed_state: String,
    pub operation_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InitializedCodeRepositoryMigrationRequest {
    pub schema: String,
    pub issue: u64,
    pub source_issue_repository: String,
    pub code_repository: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
    pub canonical_issue_collision_evidence_ref: String,
    pub canonical_issue_collision_evidence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InitializedCodeRepositoryMigrationEvidence {
    pub schema: String,
    pub issue: u64,
    pub actor: String,
    pub reason: String,
    pub pre_generation: u64,
    pub pre_digest: String,
    pub previous_code_repository: Option<String>,
    pub source_issue_repository: String,
    pub requested_repository: String,
    pub canonical_issue_collision_evidence_ref: String,
    pub canonical_issue_collision_evidence_digest: String,
    pub canonical_issue_collision_disposition: InitializedCanonicalCollisionDisposition,
    pub cross_repository_authority_disposition: String,
    pub topology_state: String,
    pub phase: LifecyclePhase,
    pub branch: Option<String>,
    pub worktree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InitializedCodeRepositoryMigrationReport {
    pub schema: String,
    pub issue: u64,
    pub changed: bool,
    pub phase: LifecyclePhase,
    pub topology_state: String,
    pub branch: Option<String>,
    pub worktree: Option<String>,
    pub code_repository: String,
    pub resulting_generation: u64,
    pub resulting_digest: String,
    pub evidence: InitializedCodeRepositoryMigrationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoundIssueIdentityMigrationRequest {
    pub schema: String,
    pub source_issue: u64,
    pub target_issue: u64,
    pub source_repository: String,
    pub target_repository: String,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub reason: String,
    pub target_issue_evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoundIssueIdentityMigrationEvidence {
    pub schema: String,
    pub source_issue: u64,
    pub target_issue: u64,
    pub source_repository: String,
    pub target_repository: String,
    pub pre_generation: u64,
    pub pre_digest: String,
    pub resulting_generation: u64,
    pub resulting_digest: String,
    pub target_issue_evidence: String,
    pub target_issue_evidence_digest: String,
    pub previous_phase: LifecyclePhase,
    pub resulting_phase: LifecyclePhase,
    pub cleared_publication: bool,
    pub cleared_readiness: bool,
    pub branch: Option<String>,
    pub worktree: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BoundIssueIdentityMigrationReport {
    pub schema: String,
    pub changed: bool,
    pub source_issue: u64,
    pub target_issue: u64,
    pub source_repository: String,
    pub target_repository: String,
    pub resulting_generation: u64,
    pub resulting_digest: String,
    pub evidence: BoundIssueIdentityMigrationEvidence,
}

#[derive(Debug, Deserialize)]
struct BoundIssueTargetEvidence {
    schema: String,
    repository: String,
    issue: u64,
    state: String,
    title: String,
    operation_key: String,
}

pub fn migrate_code_repository(
    store: &Store,
    request: CodeRepositoryMigrationRequest,
) -> Result<CodeRepositoryMigrationReport> {
    if request.schema != "csdlc.code_repository_migration_request.v1"
        || request.issue == 0
        || !valid_repository_identity(&request.code_repository)
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "code repository migration request is incomplete",
        ));
    }
    let _binding_lock = store.binding_lock()?;
    let (record, evidence) = store.commit_code_repository_migration(&request)?;
    Ok(CodeRepositoryMigrationReport {
        schema: "csdlc.code_repository_migration_report.v1".into(),
        issue: record.issue,
        changed: true,
        phase: record.phase,
        branch: evidence.branch.clone(),
        worktree: evidence.worktree.clone(),
        code_repository: record
            .code_repository
            .clone()
            .expect("migration commits repository identity"),
        resulting_generation: record.generation,
        resulting_digest: record.digest.clone(),
        evidence,
    })
}

pub fn migrate_initialized_code_repository(
    store: &Store,
    request: InitializedCodeRepositoryMigrationRequest,
) -> Result<InitializedCodeRepositoryMigrationReport> {
    if request.schema != "csdlc.initialized_code_repository_migration_request.v1"
        || request.issue == 0
        || !valid_repository_identity(&request.source_issue_repository)
        || !valid_repository_identity(&request.code_repository)
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || !clean_relative_path(&request.canonical_issue_collision_evidence_ref)
        || request
            .canonical_issue_collision_evidence_digest
            .trim()
            .is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "initialized code repository migration request is incomplete",
        ));
    }
    let _binding_lock = store.binding_lock()?;
    let (record, evidence) = store.commit_initialized_code_repository_migration(&request)?;
    Ok(InitializedCodeRepositoryMigrationReport {
        schema: "csdlc.initialized_code_repository_migration_report.v1".into(),
        issue: record.issue,
        changed: true,
        phase: record.phase,
        topology_state: "initialized_unbound".into(),
        branch: None,
        worktree: None,
        code_repository: record
            .code_repository
            .clone()
            .expect("migration commits repository identity"),
        resulting_generation: record.generation,
        resulting_digest: record.digest.clone(),
        evidence,
    })
}

pub fn migrate_bound_issue_identity(
    store: &Store,
    request: BoundIssueIdentityMigrationRequest,
) -> Result<BoundIssueIdentityMigrationReport> {
    if request.schema != "csdlc.bound_issue_identity_migration_request.v1"
        || request.source_issue == 0
        || request.target_issue == 0
        || request.source_issue == request.target_issue
        || !valid_repository_identity(&request.source_repository)
        || !valid_repository_identity(&request.target_repository)
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
        || request.reason.trim().is_empty()
        || !clean_relative_path(&request.target_issue_evidence)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bound issue identity migration request is incomplete",
        ));
    }
    let _binding_lock = store.binding_lock()?;
    let _source_lock = store.lock(request.source_issue)?;
    let _target_lock = store.lock(request.target_issue)?;
    recover_interrupted_migration(store)?;

    let target_issue_dir = store.issue_dir(request.target_issue);
    if target_issue_dir.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "target issue namespace already exists",
        ));
    }
    let evidence_path = store.root().join(&request.target_issue_evidence);
    let evidence_bytes = fs::read(&evidence_path)?;
    let evidence_digest = digest(&evidence_bytes);
    let target: BoundIssueTargetEvidence = serde_json::from_slice(&evidence_bytes)?;
    if target.schema != "csdlc.bound_issue_identity_target_evidence.v1"
        || target.repository != request.target_repository
        || target.issue != request.target_issue
        || target.state != "open"
        || target.title.trim().is_empty()
        || target.operation_key.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "target issue evidence does not match migration request",
        ));
    }

    let mut record = store.load_record(request.source_issue)?;
    if record.issue != request.source_issue
        || record.repository != request.source_repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "bound issue identity migration source identity is stale",
        ));
    }
    if matches!(record.phase, LifecyclePhase::ClosedOut) || record.terminal.is_some() {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "bound issue identity migration refuses terminal records",
        ));
    }
    verify_topology_for_identity_migration(store, &record)?;
    verify_clean_except_locks(store.root())?;

    let mut cards = store.load_cards(request.source_issue)?;
    crate::store::verify_cards(store, &record, &cards)?;
    let previous_phase = record.phase;
    let cleared_publication = record.publication.take().is_some();
    let cleared_readiness = record.readiness.take().is_some();
    if record.phase == LifecyclePhase::MergeReady {
        record.transitions.push(TransitionEvent {
            sequence: record.transitions.len() as u64 + 1,
            from: previous_phase,
            to: LifecyclePhase::Published,
            actor: request.actor.clone(),
            reason: "migrate issue identity and require typed republication".into(),
        });
        record.phase = LifecyclePhase::Published;
    }
    record.issue = request.target_issue;
    record.repository = request.target_repository.clone();
    record.generation += 1;
    for values in cards.values_mut() {
        values.identity.issue = request.target_issue;
        values.identity.repository = request.target_repository.clone();
        values.identity.generation = record.generation;
    }
    let operation = serde_json::json!({
        "operation": "migrate_bound_issue_identity",
        "source_issue": request.source_issue,
        "target_issue": request.target_issue,
        "source_repository": request.source_repository,
        "target_repository": request.target_repository,
        "target_issue_evidence": request.target_issue_evidence,
        "target_issue_evidence_digest": evidence_digest,
        "cleared_publication": cleared_publication,
        "cleared_readiness": cleared_readiness,
    })
    .to_string();
    record.audit.push(AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: request.actor.clone(),
        reason: request.reason.clone(),
        operation,
    });
    hydrate_identity_migration_projections(&mut record, &cards)?;
    let resulting_digest = record.digest.clone();
    let evidence = BoundIssueIdentityMigrationEvidence {
        schema: "csdlc.bound_issue_identity_migration_evidence.v1".into(),
        source_issue: request.source_issue,
        target_issue: request.target_issue,
        source_repository: request.source_repository,
        target_repository: request.target_repository,
        pre_generation: request.expected_generation,
        pre_digest: request.expected_digest,
        resulting_generation: record.generation,
        resulting_digest: resulting_digest.clone(),
        target_issue_evidence: request.target_issue_evidence,
        target_issue_evidence_digest: evidence_digest,
        previous_phase,
        resulting_phase: record.phase,
        cleared_publication,
        cleared_readiness,
        branch: record.branch.clone(),
        worktree: record.worktree.clone(),
    };
    write_identity_migration_namespace(
        store,
        request.source_issue,
        request.target_issue,
        &record,
        &cards,
    )?;
    Ok(BoundIssueIdentityMigrationReport {
        schema: "csdlc.bound_issue_identity_migration_report.v1".into(),
        changed: true,
        source_issue: evidence.source_issue,
        target_issue: evidence.target_issue,
        source_repository: evidence.source_repository.clone(),
        target_repository: evidence.target_repository.clone(),
        resulting_generation: evidence.resulting_generation,
        resulting_digest,
        evidence,
    })
}

fn verify_topology_for_identity_migration(store: &Store, record: &IssueRecord) -> Result<()> {
    let Some(branch) = record.branch.as_deref() else {
        return Ok(());
    };
    let worktree = record.worktree.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "bound issue identity migration requires complete topology",
        )
    })?;
    if crate::git::current_branch(store.root())? != branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound issue identity migration must run in the recorded branch",
        ));
    }
    if store.root().canonicalize()? != Path::new(worktree).canonicalize()? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound issue identity migration must run in the recorded worktree",
        ));
    }
    Ok(())
}

fn verify_clean_except_locks(root: &Path) -> Result<()> {
    let dirty = crate::git::run(
        root,
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            ".",
            ":(exclude).csdlc/locks/*.lock",
        ],
    )?
    .stdout;
    if dirty.is_empty() {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "bound issue identity migration requires a clean worktree except lifecycle locks",
        ))
    }
}

fn hydrate_identity_migration_projections(
    record: &mut IssueRecord,
    cards: &BTreeMap<CardKind, crate::cards::CardValues>,
) -> Result<()> {
    let mut projections = BTreeMap::new();
    for (kind, values) in cards {
        let rendered = render(values)?;
        projections.insert(
            *kind,
            crate::model::CardProjection {
                values_digest: rendered.values_digest,
                rendered_digest: rendered.rendered_digest,
                ast_digest: rendered.ast_digest,
            },
        );
    }
    record.cards = projections;
    let mut digest_record = record.clone();
    digest_record.digest.clear();
    record.digest = digest(&serde_json::to_vec(&digest_record)?);
    crate::store::verify_record(record)
}

fn write_identity_migration_namespace(
    store: &Store,
    source_issue: u64,
    target_issue: u64,
    record: &IssueRecord,
    cards: &BTreeMap<CardKind, crate::cards::CardValues>,
) -> Result<()> {
    let source = store.issue_dir(source_issue);
    let target = store.issue_dir(target_issue);
    let temporary = store.root().join(".csdlc/issues").join(format!(
        ".{source_issue}-to-{target_issue}.identity-migration-tmp"
    ));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)?;
    }
    fs::create_dir(&temporary)?;
    fs::create_dir(temporary.join("cards"))?;
    let result = (|| {
        let mut index = serde_json::to_vec_pretty(record)?;
        index.push(b'\n');
        fs::write(temporary.join("index.json"), index)?;
        let mut audit = Vec::new();
        for event in &record.audit {
            serde_json::to_writer(&mut audit, event)?;
            audit.push(b'\n');
        }
        fs::write(temporary.join("audit.jsonl"), audit)?;
        for (kind, values) in cards {
            let mut value_bytes = serde_json::to_vec_pretty(values)?;
            value_bytes.push(b'\n');
            fs::write(
                temporary.join("cards").join(format!("{kind}.values.json")),
                value_bytes,
            )?;
            fs::write(
                temporary.join("cards").join(format!("{kind}.md")),
                render(values)?.markdown,
            )?;
        }
        if target.exists() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "target issue namespace appeared during migration",
            ));
        }
        fs::rename(&temporary, &target)?;
        fs::remove_dir_all(&source)?;
        Ok(())
    })();
    if result.is_err() && temporary.exists() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result
}

fn valid_repository_identity(value: &str) -> bool {
    let mut parts = value.split('/');
    matches!((parts.next(), parts.next(), parts.next()), (Some(owner), Some(repo), None)
        if !owner.is_empty()
            && !repo.is_empty()
            && !owner.chars().any(char::is_whitespace)
            && !repo.chars().any(char::is_whitespace))
}

#[derive(Debug, Deserialize)]
struct IssueStateEvidence {
    schema: String,
    issues: Vec<ObservedIssueState>,
}

#[derive(Debug, Deserialize)]
struct ObservedIssueState {
    issue: u64,
    state: MigrationIssueState,
    closed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TerminalObservation {
    schema: String,
    issue: u64,
    issue_state: MigrationIssueState,
    closed_at: Option<String>,
    disposition: TerminalDisposition,
    pull_request: Option<u64>,
    observed_sha: Option<String>,
}

pub fn migrate_bound_topology(
    store: &Store,
    request: BoundTopologyMigrationRequest,
) -> Result<BoundTopologyMigrationReport> {
    migrate_bound_topology_inner(store, request, None)
}

#[doc(hidden)]
pub fn migrate_bound_topology_with_failure_for_test(
    store: &Store,
    request: BoundTopologyMigrationRequest,
    fail_after_commits: usize,
) -> Result<BoundTopologyMigrationReport> {
    migrate_bound_topology_inner(store, request, Some((fail_after_commits, true)))
}

#[doc(hidden)]
pub fn migrate_bound_topology_with_crash_for_test(
    store: &Store,
    request: BoundTopologyMigrationRequest,
    fail_after_commits: usize,
) -> Result<BoundTopologyMigrationReport> {
    migrate_bound_topology_inner(store, request, Some((fail_after_commits, false)))
}

fn migrate_bound_topology_inner(
    store: &Store,
    request: BoundTopologyMigrationRequest,
    fail_after_commits: Option<(usize, bool)>,
) -> Result<BoundTopologyMigrationReport> {
    let _migration_lock = store.binding_lock()?;
    if request.schema != "csdlc.bound_topology_migration_request.v1"
        || request.actor.trim().is_empty()
        || !clean_relative_path(&request.issue_state_evidence)
        || request.issues.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bound topology migration request is incomplete",
        ));
    }
    let unique = request
        .issues
        .iter()
        .map(|item| item.issue)
        .collect::<BTreeSet<_>>();
    if unique.len() != request.issues.len() || unique.contains(&0) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "bound topology migration issues must be unique and nonzero",
        ));
    }

    recover_interrupted_migration(store)?;
    let state_evidence_bytes = fs::read(store.root().join(&request.issue_state_evidence))?;
    let state_evidence_digest = crate::cards::digest(&state_evidence_bytes);
    let observed: IssueStateEvidence = serde_json::from_slice(&state_evidence_bytes)?;
    if observed.schema != "csdlc.bound_topology_issue_states.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue state evidence schema is unsupported",
        ));
    }
    let observed_states = observed
        .issues
        .into_iter()
        .map(|entry| (entry.issue, (entry.state, entry.closed_at)))
        .collect::<BTreeMap<_, _>>();
    if observed_states.len() != request.issues.len()
        || request.issues.iter().any(|item| {
            observed_states
                .get(&item.issue)
                .is_none_or(|(state, closed_at)| {
                    *state != item.state
                        || (*state == MigrationIssueState::Closed && closed_at.is_none())
                })
        })
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "migration request does not match retained live issue state evidence",
        ));
    }

    let worktrees = crate::git::worktrees(store.root())?;
    let mut planned = Vec::new();
    for item in &request.issues {
        let mut terminal_evidence_digest = None;
        let source_value: serde_json::Value =
            serde_json::from_slice(&fs::read(store.issue_dir(item.issue).join("index.json"))?)?;
        let source_digest = source_value
            .get("digest")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "source digest is missing"))?
            .to_owned();
        let record = store.load_record_for_topology_scan(item.issue)?;
        let normalized_digest = record.digest.clone();
        let cards = store.load_cards(item.issue)?;
        crate::store::verify_pre_topology_cards(store, &record, &cards)?;
        if record.phase != LifecyclePhase::Bound
            || record.branch.is_some()
            || record.worktree.is_some()
        {
            planned.push((
                item.issue,
                source_digest,
                normalized_digest,
                record,
                BoundTopologyDisposition::AlreadyCurrent,
                terminal_evidence_digest,
                false,
            ));
            continue;
        }

        let topology = if item.state == MigrationIssueState::Open {
            verified_topology_candidates(store.root(), item.issue, &worktrees)?
        } else {
            BTreeSet::new()
        };
        if topology.len() > 1 {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("issue {} has ambiguous Git topology", item.issue),
            ));
        }
        let mut migrated = record;
        let disposition = if let Some((branch, worktree)) = topology.into_iter().next() {
            migrated.branch = Some(branch);
            migrated.worktree = Some(worktree);
            BoundTopologyDisposition::AdoptedVerifiedTopology
        } else {
            match item.state {
                MigrationIssueState::Open => {
                    migrated.transitions.push(TransitionEvent {
                        sequence: migrated.transitions.len() as u64 + 1,
                        from: LifecyclePhase::Bound,
                        to: LifecyclePhase::Initialized,
                        actor: "csdlc-topology-migrate".into(),
                        reason: "migrate pre-topology bound record".into(),
                    });
                    migrated.phase = LifecyclePhase::Initialized;
                    BoundTopologyDisposition::ResetInitialized
                }
                MigrationIssueState::Closed => {
                    let terminal = item.terminal.as_ref().ok_or_else(|| {
                        V2Error::new(
                            ErrorCode::InvalidInput,
                            format!("closed issue {} requires terminal evidence", item.issue),
                        )
                    })?;
                    if terminal.observed_state != "closed"
                        || terminal.receipt_path.trim().is_empty()
                        || !clean_relative_path(&terminal.receipt_path)
                        || match terminal.disposition {
                            TerminalDisposition::Merged => {
                                terminal.pull_request.is_none()
                                    || terminal.observed_sha.as_deref().is_none_or(str::is_empty)
                            }
                            TerminalDisposition::ClosedUnmerged => terminal.pull_request.is_none(),
                            TerminalDisposition::ClosedNoPr => {
                                terminal.pull_request.is_some() || terminal.observed_sha.is_some()
                            }
                        }
                    {
                        return Err(V2Error::new(
                            ErrorCode::InvalidInput,
                            format!(
                                "closed issue {} terminal evidence is incomplete",
                                item.issue
                            ),
                        ));
                    }
                    let terminal_bytes = fs::read(store.root().join(&terminal.receipt_path))?;
                    terminal_evidence_digest = Some(crate::cards::digest(&terminal_bytes));
                    let observation: TerminalObservation = serde_json::from_slice(&terminal_bytes)?;
                    if observation.schema != "csdlc.bound_topology_terminal_observation.v1"
                        || observation.issue != item.issue
                        || observation.issue_state != MigrationIssueState::Closed
                        || observation.closed_at.is_none()
                        || observation.disposition != terminal.disposition
                        || observation.pull_request != terminal.pull_request
                        || observation.observed_sha != terminal.observed_sha
                    {
                        return Err(V2Error::new(
                            ErrorCode::ReconciliationRequired,
                            format!(
                                "closed issue {} evidence does not match request",
                                item.issue
                            ),
                        ));
                    }
                    migrated.transitions.push(TransitionEvent {
                        sequence: migrated.transitions.len() as u64 + 1,
                        from: LifecyclePhase::Bound,
                        to: LifecyclePhase::ClosedOut,
                        actor: "csdlc-topology-migrate".into(),
                        reason: "migrate pre-topology bound record".into(),
                    });
                    migrated.phase = LifecyclePhase::ClosedOut;
                    migrated.terminal = Some(TerminalEvidence {
                        pull_request: terminal.pull_request,
                        disposition: terminal.disposition,
                        observed_sha: terminal.observed_sha.clone(),
                        observed_state: terminal.observed_state.clone(),
                        receipt_path: terminal.receipt_path.clone(),
                        branch: None,
                        worktree: None,
                    });
                    BoundTopologyDisposition::ClosedOut
                }
            }
        };
        migrated.audit.push(AuditEvent {
            sequence: migrated.audit.len() as u64 + 1,
            generation: migrated.generation,
            actor: request.actor.clone(),
            reason: "migrate pre-#5886 bound record to Git topology authority".into(),
            operation: serde_json::json!({
                "operation": "migrate_bound_topology",
                "issue_state_evidence": request.issue_state_evidence,
                "issue_state_evidence_digest": state_evidence_digest,
                "terminal_evidence_digest": terminal_evidence_digest,
            })
            .to_string(),
        });
        migrated.digest = crate::store::record_digest(&migrated)?;
        crate::store::verify_record(&migrated)?;
        planned.push((
            item.issue,
            source_digest,
            normalized_digest,
            migrated,
            disposition,
            terminal_evidence_digest,
            true,
        ));
    }

    if request.apply {
        let _locks = request
            .issues
            .iter()
            .map(|item| store.lock(item.issue))
            .collect::<Result<Vec<_>>>()?;
        for (issue, _, normalized_digest, _, _, _, _) in &planned {
            if store.load_record_for_topology_scan(*issue)?.digest != *normalized_digest {
                return Err(V2Error::new(
                    ErrorCode::StaleDigest,
                    "record changed after topology migration classification",
                ));
            }
        }
        let changed_issues = planned
            .iter()
            .filter(|(_, _, _, _, _, _, changed)| *changed)
            .map(|(issue, _, _, _, _, _, _)| *issue)
            .collect::<Vec<_>>();
        let snapshot = snapshot_issue_directories(store, &changed_issues)?;
        let apply_result = (|| {
            let mut committed = 0usize;
            for (issue, _, normalized_digest, record, _, _, changed) in &planned {
                if *changed {
                    store.replace_pre_topology_record_locked(*issue, normalized_digest, record)?;
                    committed += 1;
                    if fail_after_commits.is_some_and(|(count, _)| count == committed) {
                        return Err(V2Error::new(
                            ErrorCode::InterruptedTransaction,
                            "injected bound topology batch interruption",
                        ));
                    }
                }
            }
            Ok(())
        })();
        if let Err(error) = apply_result {
            if fail_after_commits.is_none_or(|(_, rollback)| rollback) {
                restore_issue_directories(store, &snapshot, &changed_issues)?;
            }
            return Err(error);
        }
        mark_snapshot_committed(&snapshot, &changed_issues)?;
        fs::remove_dir_all(snapshot)?;
    }
    let results = planned
        .into_iter()
        .map(
            |(
                issue,
                source_digest,
                normalized_digest,
                record,
                disposition,
                terminal_evidence_digest,
                changed,
            )| {
                BoundTopologyMigrationResult {
                    issue,
                    disposition,
                    resulting_digest: record.digest.clone(),
                    source_digest,
                    normalized_digest,
                    branch: record.branch,
                    worktree: record.worktree,
                    terminal_evidence_digest,
                    changed: changed && request.apply,
                }
            },
        )
        .collect::<Vec<_>>();
    Ok(BoundTopologyMigrationReport {
        schema: "csdlc.bound_topology_migration_report.v1".into(),
        applied: request.apply,
        changed: results.iter().filter(|result| result.changed).count(),
        issue_state_evidence: request.issue_state_evidence,
        issue_state_evidence_digest: state_evidence_digest,
        results,
    })
}

fn clean_relative_path(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|part| matches!(part, std::path::Component::Normal(_)))
}

#[derive(Debug, Serialize, Deserialize)]
struct MigrationSnapshotManifest {
    schema: String,
    issues: Vec<u64>,
    committed: bool,
}

fn migration_snapshot_path(store: &Store) -> PathBuf {
    store
        .root()
        .join(".csdlc/issues/.bound-topology-migration-backup")
}

fn recover_interrupted_migration(store: &Store) -> Result<()> {
    let snapshot = migration_snapshot_path(store);
    if !snapshot.exists() {
        return Ok(());
    }
    let manifest_path = snapshot.join("manifest.json");
    if !manifest_path.is_file() {
        fs::remove_dir_all(snapshot)?;
        return Ok(());
    }
    let manifest: MigrationSnapshotManifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    if manifest.schema != "csdlc.bound_topology_snapshot.v1"
        || manifest.issues.is_empty()
        || manifest.issues.contains(&0)
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "bound topology migration recovery manifest is invalid",
        ));
    }
    if manifest.committed {
        fs::remove_dir_all(snapshot)?;
        return Ok(());
    }
    let _locks = manifest
        .issues
        .iter()
        .map(|issue| store.lock(*issue))
        .collect::<Result<Vec<_>>>()?;
    restore_issue_directories(store, &snapshot, &manifest.issues)
}

fn snapshot_issue_directories(store: &Store, issues: &[u64]) -> Result<PathBuf> {
    let snapshot = migration_snapshot_path(store);
    if snapshot.exists() {
        return Err(V2Error::new(
            ErrorCode::InterruptedTransaction,
            "bound topology migration snapshot already exists",
        ));
    }
    fs::create_dir(&snapshot)?;
    let result = issues.iter().try_for_each(|issue| {
        copy_directory(&store.issue_dir(*issue), &snapshot.join(issue.to_string()))
    });
    if let Err(error) = result {
        let _ = fs::remove_dir_all(&snapshot);
        return Err(error);
    }
    let manifest = MigrationSnapshotManifest {
        schema: "csdlc.bound_topology_snapshot.v1".into(),
        issues: issues.to_vec(),
        committed: false,
    };
    write_snapshot_manifest(&snapshot, &manifest)?;
    Ok(snapshot)
}

fn mark_snapshot_committed(snapshot: &Path, issues: &[u64]) -> Result<()> {
    write_snapshot_manifest(
        snapshot,
        &MigrationSnapshotManifest {
            schema: "csdlc.bound_topology_snapshot.v1".into(),
            issues: issues.to_vec(),
            committed: true,
        },
    )
}

fn write_snapshot_manifest(snapshot: &Path, manifest: &MigrationSnapshotManifest) -> Result<()> {
    let temporary = snapshot.join("manifest.json.tmp");
    let file = fs::File::create(&temporary)?;
    serde_json::to_writer_pretty(&file, manifest)?;
    file.sync_all()?;
    fs::rename(temporary, snapshot.join("manifest.json"))?;
    fs::File::open(snapshot)?.sync_all()?;
    Ok(())
}

fn restore_issue_directories(store: &Store, snapshot: &Path, issues: &[u64]) -> Result<()> {
    for issue in issues {
        let backup = snapshot.join(issue.to_string());
        let destination = store.issue_dir(*issue);
        if backup.exists() {
            if destination.exists() {
                fs::remove_dir_all(&destination)?;
            }
            fs::rename(backup, destination)?;
        } else if !destination.exists() {
            return Err(V2Error::new(
                ErrorCode::InterruptedTransaction,
                format!("issue {issue} has neither migration backup nor restored record"),
            ));
        }
    }
    fs::remove_dir_all(snapshot)?;
    Ok(())
}

fn copy_directory(source: &Path, destination: &Path) -> Result<()> {
    if source.symlink_metadata()?.file_type().is_symlink() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "topology migration refuses symlinked issue directories",
        ));
    }
    fs::create_dir(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = destination.join(entry.file_name());
        if file_type.is_symlink() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "topology migration refuses symlinked issue contents",
            ));
        }
        if file_type.is_dir() {
            copy_directory(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target)?;
        } else {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "topology migration refuses special issue files",
            ));
        }
    }
    Ok(())
}

fn verified_topology_candidates(
    root: &Path,
    issue: u64,
    worktrees: &[(String, String)],
) -> Result<BTreeSet<(String, String)>> {
    let mut candidates = BTreeSet::new();
    for (listed_branch, listed_path) in worktrees {
        if listed_branch.is_empty() || !Path::new(listed_path).exists() {
            continue;
        }
        let candidate_store = Store::new(listed_path);
        let index = candidate_store.issue_dir(issue).join("index.json");
        if !index.is_file() {
            continue;
        }
        let value: serde_json::Value = serde_json::from_slice(&fs::read(index)?)?;
        let Some(branch) = value.get("branch").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let Some(worktree) = value.get("worktree").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let canonical = Path::new(listed_path).canonicalize()?;
        let recorded = if Path::new(worktree).is_absolute() {
            PathBuf::from(worktree)
        } else {
            root.join(worktree)
        };
        if branch != listed_branch || recorded.canonicalize()? != canonical {
            continue;
        }
        let record = candidate_store.load_record(issue)?;
        crate::store::verify_cards(
            &candidate_store,
            &record,
            &candidate_store.load_cards(issue)?,
        )?;
        reject_conflicting_topology_owner(root, issue, branch, &canonical, worktrees)?;
        candidates.insert((
            listed_branch.clone(),
            canonical.to_string_lossy().into_owned(),
        ));
    }
    Ok(candidates)
}

fn reject_conflicting_topology_owner(
    root: &Path,
    issue: u64,
    branch: &str,
    worktree: &Path,
    worktrees: &[(String, String)],
) -> Result<()> {
    for (_, listed_path) in worktrees {
        let issue_root = Path::new(listed_path).join(".csdlc/issues");
        if !issue_root.is_dir() {
            continue;
        }
        for entry in fs::read_dir(issue_root)? {
            let entry = entry?;
            let Some(other_issue) = entry
                .file_name()
                .to_str()
                .and_then(|value| value.parse::<u64>().ok())
            else {
                continue;
            };
            if other_issue == issue || !entry.path().join("index.json").is_file() {
                continue;
            }
            let value: serde_json::Value =
                serde_json::from_slice(&fs::read(entry.path().join("index.json"))?)?;
            let other_branch = value.get("branch").and_then(serde_json::Value::as_str);
            let other_worktree = value
                .get("worktree")
                .and_then(serde_json::Value::as_str)
                .and_then(|value| {
                    let path = Path::new(value);
                    let resolved = if path.is_absolute() {
                        path.to_path_buf()
                    } else {
                        root.join(path)
                    };
                    resolved.canonicalize().ok()
                });
            if other_branch == Some(branch)
                || other_worktree
                    .as_deref()
                    .is_some_and(|path| path == worktree)
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!(
                        "topology candidate for issue {issue} is claimed by issue {other_issue}"
                    ),
                ));
            }
        }
    }
    Ok(())
}

pub fn import_legacy(request: LegacyImportRequest) -> Result<ImportReport> {
    validate_request(&request)?;
    let mut authored = BTreeMap::new();
    let mut authored_sources = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut source = blake3::Hasher::new();
    let legacy_root = request.legacy_root.canonicalize()?;
    let output_root = request.output_root.canonicalize()?;
    let mut canonical_sources = BTreeSet::new();
    for kind in enum_cards() {
        let Some(relative) = request.card_paths.get(&kind) else {
            diagnostics.push(diag(
                Some(kind),
                "card_missing",
                "all six legacy card paths are required",
            ));
            continue;
        };
        let relative_path = std::path::Path::new(relative);
        if relative_path.is_absolute()
            || !relative_path
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
        {
            diagnostics.push(diag(
                Some(kind),
                "source_path_unsafe",
                "legacy card path must be a clean relative path",
            ));
            continue;
        }
        let path = request.legacy_root.join(relative);
        let canonical = match path.canonicalize() {
            Ok(path) if path.starts_with(&legacy_root) && !path.starts_with(&output_root) => path,
            _ => {
                diagnostics.push(diag(
                    Some(kind),
                    "source_path_outside_legacy_root",
                    "legacy source resolves outside the disjoint legacy root",
                ));
                continue;
            }
        };
        if !canonical_sources.insert(canonical.clone()) {
            diagnostics.push(diag(
                Some(kind),
                "source_path_reused",
                "one legacy file cannot own multiple cards",
            ));
            continue;
        }
        let bytes = match fs::read(&canonical) {
            Ok(bytes) => bytes,
            Err(_) => {
                diagnostics.push(diag(
                    Some(kind),
                    "card_unreadable",
                    &format!("legacy card is unreadable: {relative}"),
                ));
                continue;
            }
        };
        source.update(&(relative.len() as u64).to_le_bytes());
        source.update(relative.as_bytes());
        source.update(&(bytes.len() as u64).to_le_bytes());
        source.update(&bytes);
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => {
                diagnostics.push(diag(
                    Some(kind),
                    "card_not_utf8",
                    "legacy Markdown must be UTF-8",
                ));
                continue;
            }
        };
        authored_sources.insert(kind.to_string(), text.clone());
        match parse_sections(&text) {
            Ok(sections) => {
                for required in required_headings(kind) {
                    if !sections.contains_key(*required) {
                        diagnostics.push(diag(
                            Some(kind),
                            "required_heading_missing",
                            &format!("required heading is missing: {required}"),
                        ));
                    }
                }
                authored.insert(kind.to_string(), sections);
            }
            Err(error) => diagnostics.push(diag(Some(kind), "markdown_ambiguous", &error.message)),
        }
    }
    let sunset = request
        .default_cutover_unix_seconds
        .checked_add(30 * 24 * 60 * 60)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cutover sunset overflow"))?;
    let source_digest = format!("blake3:{}", source.finalize().to_hex());
    if !diagnostics.is_empty() {
        return Ok(ImportReport {
            schema: "csdlc.legacy_import_report.v1".into(),
            status: ImportStatus::Unsupported,
            issue: request.issue,
            source_digest: None,
            retained_section_count: authored.values().map(BTreeMap::len).sum(),
            diagnostics,
            compatibility_view: None,
            sunset_unix_seconds: sunset,
        });
    }
    let initial = match build_initial(&request, &authored) {
        Ok(initial) => initial,
        Err(error) => {
            return Ok(unsupported(
                &request,
                sunset,
                Some(source_digest),
                authored.values().map(BTreeMap::len).sum(),
                vec![diag(None, "typed_values_unrepresentable", &error.message)],
            ));
        }
    };
    let design = request.output_root.join(&request.design_path);
    let diagram = request.output_root.join(&request.diagram_path);
    let safe_output_path = |value: &str| {
        !value.is_empty()
            && std::path::Path::new(value)
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
    };
    if !safe_output_path(&request.design_path)
        || !safe_output_path(&request.diagram_path)
        || !design.is_file()
        || !diagram.is_file()
    {
        return Ok(unsupported(
            &request,
            sunset,
            Some(source_digest),
            authored.values().map(BTreeMap::len).sum(),
            vec![diag(
                None,
                "design_or_diagram_unrepresentable",
                "safe existing output design and diagram files are required before import",
            )],
        ));
    }
    let design_digest = crate::cards::digest(&fs::read(&design)?);
    let diagram_digest = crate::cards::digest(&fs::read(&diagram)?);
    if let Err(error) = crate::cards::initial_cards(
        request.issue,
        &request.repository,
        &request.design_path,
        &design_digest,
        &request.diagram_path,
        &diagram_digest,
        initial.clone(),
    ) {
        return Ok(unsupported(
            &request,
            sunset,
            Some(source_digest),
            authored.values().map(BTreeMap::len).sum(),
            vec![diag(None, "typed_cards_unrepresentable", &error.message)],
        ));
    }
    let store = Store::new(&request.output_root);
    let mut record = initialize_issue(
        &store,
        BootstrapRequest {
            issue: request.issue,
            repository: request.repository.clone(),
            design_path: request.design_path.clone(),
            diagram_path: request.diagram_path.clone(),
            design_reviewer: request.design_reviewer,
            design_approved: true,
            actor: request.actor,
            initial,
        },
    )?;
    let compatibility_path = format!(".csdlc/compat/{}.md", request.issue);
    let compatibility = render_legacy_archive(request.issue, &authored_sources);
    record = store.commit_migration(
        request.issue,
        &record.digest,
        MigrationEvidence {
            schema: "csdlc.migration_evidence.v1".into(),
            imported_unix_seconds: request.imported_unix_seconds,
            sunset_unix_seconds: sunset,
            source_digest: source_digest.clone(),
            authored_sources: authored_sources.clone(),
            authored_sections: authored.clone(),
            compatibility_view: compatibility_path.clone(),
        },
    )?;
    if request.legacy_phase == LifecyclePhase::Ready && record.phase == LifecyclePhase::Initialized
    {
        let _ = edit_issue(
            &store,
            EditRequest {
                issue: request.issue,
                card: CardKind::Sip,
                expected_generation: record.generation,
                expected_digest: record.digest,
                actor: "csdlc-import".into(),
                reason: "normalize supported legacy ready outcome".into(),
                operation: SemanticOperation::AdvancePhase {
                    phase: LifecyclePhase::Ready,
                },
                fail_after_backup: false,
            },
        )?;
    }
    write_compatibility_view_atomic(&store, request.issue, &compatibility)?;
    Ok(ImportReport {
        schema: "csdlc.legacy_import_report.v1".into(),
        status: ImportStatus::Imported,
        issue: request.issue,
        source_digest: Some(source_digest),
        retained_section_count: authored.values().map(BTreeMap::len).sum(),
        diagnostics: Vec::new(),
        compatibility_view: Some(compatibility_path),
        sunset_unix_seconds: sunset,
    })
}

fn validate_request(request: &LegacyImportRequest) -> Result<()> {
    if request.schema != "csdlc.legacy_import_request.v1"
        || request.issue == 0
        || !matches!(
            request.legacy_phase,
            LifecyclePhase::Initialized | LifecyclePhase::Ready
        )
        || request.validation_lanes.is_empty()
        || request.imported_unix_seconds == 0
        || request.default_cutover_unix_seconds == 0
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy import request is invalid or outside the bounded phase set",
        ));
    }
    let legacy = request.legacy_root.canonicalize()?;
    let output = request.output_root.canonicalize()?;
    if legacy == output || legacy.starts_with(&output) || output.starts_with(&legacy) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy and output roots must be canonical and disjoint",
        ));
    }
    let sunset = request
        .default_cutover_unix_seconds
        .checked_add(30 * 24 * 60 * 60)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "cutover sunset overflow"))?;
    if request.imported_unix_seconds > sunset {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy importer expired 30 days after default cutover",
        ));
    }
    Ok(())
}

fn parse_sections(text: &str) -> Result<BTreeMap<String, String>> {
    let ast = to_mdast(text, &ParseOptions::gfm())
        .map_err(|message| V2Error::new(ErrorCode::InvalidInput, message.to_string()))?;
    let children = ast
        .children()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "mdast root is absent"))?;
    let mut anchors = Vec::new();
    for child in children {
        if let Node::Heading(heading) = child {
            if heading.depth == 2 {
                let position = child.position().ok_or_else(|| {
                    V2Error::new(ErrorCode::InvalidInput, "heading source position is absent")
                })?;
                anchors.push((
                    child.to_string().trim().to_owned(),
                    position.start.offset,
                    position.end.offset,
                ));
            }
        }
    }
    let mut seen = BTreeSet::new();
    if anchors.is_empty()
        || anchors
            .iter()
            .any(|(name, _, _)| name.is_empty() || !seen.insert(name.clone()))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "level-two headings are missing, empty, or duplicated",
        ));
    }
    let mut sections = BTreeMap::new();
    for (index, (name, _, end)) in anchors.iter().enumerate() {
        let next = anchors
            .get(index + 1)
            .map_or(text.len(), |(_, start, _)| *start);
        let raw = text
            .get(*end..next)
            .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "heading offsets are invalid"))?;
        if raw.trim().is_empty() {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                format!("authored section is empty: {name}"),
            ));
        }
        sections.insert(name.clone(), raw.to_owned());
    }
    Ok(sections)
}

fn build_initial(
    request: &LegacyImportRequest,
    authored: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<InitialCardInput> {
    let get = |kind: CardKind, heading: &str| -> Result<String> {
        authored
            .get(&kind.to_string())
            .and_then(|sections| sections.get(heading))
            .cloned()
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::InvalidInput,
                    format!("{kind} {heading} is absent"),
                )
            })
    };
    let optional = |kind: CardKind, heading: &str| -> Option<String> {
        authored
            .get(&kind.to_string())
            .and_then(|sections| sections.get(heading))
            .cloned()
    };
    let acceptance_criteria = list(&get(CardKind::Stp, "Acceptance Criteria")?);
    let acceptance_ids: Vec<_> = (1..=acceptance_criteria.len())
        .map(|index| format!("AC-{index}"))
        .collect();
    let steps: Vec<_> = list(&get(CardKind::Spp, "Plan")?)
        .into_iter()
        .enumerate()
        .map(|(index, action)| PlanStep {
            id: format!("imported-{}", index + 1),
            action,
            acceptance_ids: acceptance_ids.clone(),
            status: StepStatus::Pending,
        })
        .collect();
    if steps.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "legacy Plan has no unambiguous list items",
        ));
    }
    Ok(InitialCardInput {
        title: request.title.clone(),
        slug: request.slug.clone(),
        version: request.version.clone(),
        goal: plain(&get(CardKind::Sip, "Goal")?),
        required_outcome: plain(&get(CardKind::Stp, "Required Outcome")?),
        declared_scope: list(&get(CardKind::Sip, "Scope")?),
        authority_boundary: list(&get(CardKind::Sip, "Authority")?),
        operator_constraints: optional(CardKind::Sip, "Operator Constraints")
            .map(|value| list(&value))
            .filter(|values| !values.is_empty())
            .unwrap_or_else(|| vec!["none".into()]),
        task_boundary: plain(&get(CardKind::Stp, "Summary")?),
        deliverables: list(&get(CardKind::Stp, "Deliverables")?),
        acceptance_criteria,
        dependencies: vec!["imported legacy observation".into()],
        repo_inputs: vec!["one-way imported authored cards".into()],
        non_goals: vec!["legacy implementation reuse".into()],
        plan_summary: plain(&get(CardKind::Spp, "Plan")?),
        steps,
        affected_areas: vec![format!(".csdlc/issues/{}", request.issue)],
        invariants: list(&get(CardKind::Spp, "Invariants")?),
        risks: list(&get(CardKind::Spp, "Risks")?),
        planning_profile: request.planning_profile,
        stop_conditions: list(&get(CardKind::Spp, "Stop Conditions")?),
        validation_lanes: request
            .validation_lanes
            .iter()
            .cloned()
            .map(|mut lane| {
                lane.acceptance_ids = acceptance_ids.clone();
                lane
            })
            .collect(),
        failure_policy: plain(&get(CardKind::Vpp, "Failure Policy")?),
        review_prompts: list(&get(CardKind::Srp, "Prompts")?),
        review_scope: plain(&get(CardKind::Srp, "Review Scope")?),
    })
}

fn list(raw: &str) -> Vec<String> {
    raw.lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("- ")
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        })
        .collect()
}
fn plain(raw: &str) -> String {
    let items = list(raw);
    if items.is_empty() {
        raw.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        items.join("; ")
    }
}
fn diag(card: Option<CardKind>, code: &str, message: &str) -> MigrationDiagnostic {
    MigrationDiagnostic {
        card,
        code: code.into(),
        message: message.into(),
    }
}

fn unsupported(
    request: &LegacyImportRequest,
    sunset_unix_seconds: u64,
    source_digest: Option<String>,
    retained_section_count: usize,
    diagnostics: Vec<MigrationDiagnostic>,
) -> ImportReport {
    ImportReport {
        schema: "csdlc.legacy_import_report.v1".into(),
        status: ImportStatus::Unsupported,
        issue: request.issue,
        source_digest,
        retained_section_count,
        diagnostics,
        compatibility_view: None,
        sunset_unix_seconds,
    }
}
fn enum_cards() -> [CardKind; 6] {
    [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ]
}
fn required_headings(kind: CardKind) -> &'static [&'static str] {
    match kind {
        CardKind::Sip => &["Goal", "Scope", "Authority"],
        CardKind::Stp => &[
            "Summary",
            "Required Outcome",
            "Deliverables",
            "Acceptance Criteria",
        ],
        CardKind::Spp => &["Plan", "Invariants", "Risks", "Stop Conditions"],
        CardKind::Vpp => &["Validation", "Failure Policy"],
        CardKind::Srp => &["Review Scope", "Prompts"],
        CardKind::Sor => &[
            "Summary",
            "Artifacts",
            "Execution",
            "Integration",
            "Publication",
            "Closeout",
            "Follow Ups",
        ],
    }
}
fn render_legacy_archive(issue: u64, authored: &BTreeMap<String, String>) -> String {
    let mut out = format!("# Generated compatibility view for issue {issue}\n\nGenerated from canonical migration evidence. Do not edit.\n");
    for (card, source) in authored {
        out.push_str(&format!("\n<!-- BEGIN exact legacy source: {card} -->\n"));
        out.push_str(source);
        if !source.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&format!("<!-- END exact legacy source: {card} -->\n"));
    }
    out
}

pub fn generate_compatibility_view(store: &Store, issue: u64) -> Result<String> {
    let record = store.load_record(issue)?;
    let migration = record
        .migration
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "issue has no migration evidence"))?;
    Ok(render_legacy_archive(issue, &migration.authored_sources))
}

pub fn write_compatibility_view_atomic(store: &Store, issue: u64, view: &str) -> Result<String> {
    let directory = store.root().join(".csdlc/compat");
    fs::create_dir_all(&directory)?;
    let target = directory.join(format!("{issue}.md"));
    let temporary = directory.join(format!(".{issue}.tmp"));
    fs::write(&temporary, view)?;
    fs::rename(&temporary, &target)?;
    Ok(format!(".csdlc/compat/{issue}.md"))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct NormalizedOutcome {
    pub schema: String,
    pub issue: u64,
    pub phase: LifecyclePhase,
    pub card_statuses: BTreeMap<CardKind, CardStatus>,
    pub review_result: String,
    pub integration_state: String,
    pub publication_state: String,
    pub merge_state: String,
    pub closeout_state: String,
    pub doctor_status: String,
    pub doctor_findings: Vec<String>,
}

impl NormalizedOutcome {
    pub fn from_v2(store: &Store, issue: u64) -> Result<Self> {
        let record = store.load_record(issue)?;
        let cards = store.load_cards(issue)?;
        let doctor = crate::diagnose(store, issue);
        let mut doctor_findings: Vec<_> = doctor
            .findings
            .into_iter()
            .map(|finding| finding.code)
            .collect();
        doctor_findings.sort();
        let statuses = cards
            .iter()
            .map(|(kind, values)| (*kind, values.status))
            .collect();
        let srp = match &cards[&CardKind::Srp].content {
            CardContent::Srp(value) => value,
            _ => unreachable!(),
        };
        let sor = match &cards[&CardKind::Sor].content {
            CardContent::Sor(value) => value,
            _ => unreachable!(),
        };
        Ok(Self {
            schema: "csdlc.normalized_outcome.v1".into(),
            issue,
            phase: record.phase,
            card_statuses: statuses,
            review_result: srp.review_result.to_string(),
            integration_state: sor.integration_state.to_string(),
            publication_state: sor.publication_state.to_string(),
            merge_state: sor.merge_state.to_string(),
            closeout_state: sor.closeout_state.to_string(),
            doctor_status: doctor.status.to_string(),
            doctor_findings,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ShadowComparison {
    pub schema: String,
    pub equivalent: bool,
    pub differences: Vec<String>,
}

pub fn compare_shadow(legacy: &NormalizedOutcome, v2: &NormalizedOutcome) -> ShadowComparison {
    let mut differences = Vec::new();
    if legacy.issue != v2.issue {
        differences.push("issue".into());
    }
    if legacy.phase != v2.phase {
        differences.push("phase".into());
    }
    if legacy.card_statuses != v2.card_statuses {
        differences.push("card_statuses".into());
    }
    if legacy.review_result != v2.review_result {
        differences.push("review_result".into());
    }
    if legacy.integration_state != v2.integration_state {
        differences.push("integration_state".into());
    }
    if legacy.publication_state != v2.publication_state {
        differences.push("publication_state".into());
    }
    if legacy.merge_state != v2.merge_state {
        differences.push("merge_state".into());
    }
    if legacy.closeout_state != v2.closeout_state {
        differences.push("closeout_state".into());
    }
    if legacy.doctor_status != v2.doctor_status {
        differences.push("doctor_status".into());
    }
    if legacy.doctor_findings != v2.doctor_findings {
        differences.push("doctor_findings".into());
    }
    ShadowComparison {
        schema: "csdlc.shadow_comparison.v1".into(),
        equivalent: differences.is_empty(),
        differences,
    }
}
