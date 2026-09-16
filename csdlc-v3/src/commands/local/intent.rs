//! Narrow intent bridges into the existing local owner and transaction journal.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;

use super::cards::{initial_card_values, merge_json_object, render_template, structure_valid};
use super::context::{require_operational_cas, validate_context};
use super::issue::initialize_operational_issue_with_plan;
use super::lifecycle::inspect_lifecycle_issue_root;
use super::planning::{plan_cards, validate_contract};
use super::storage::{io_finding, lifecycle_digest, read_index_value};
use super::transactions::{
    acquire_issue_mutation_lock, local_transaction_journal_path, read_pending_local_transaction,
    recover_pending_local_transaction,
};
use super::worktree::has_canonical_existing_ancestor;
use super::{
    finding, DoctorFinding, LocalMutationJournal, LocalPreparationRequest, OperationalLocalContext,
    OperationalLocalResult, PlanStatus, PromptRegistry, REQUIRED_CARD_KINDS,
};

/// Recompute native card/plan integrity; an index's claimed digest is not proof
/// that its inputs stayed unchanged between observation and remote dispatch.
pub fn verify_integrity(issue_root: &Path, index: &Value) -> Result<(), Vec<DoctorFinding>> {
    let actual = lifecycle_digest(issue_root, index)?;
    if index["digest"].as_str() != Some(actual.as_str()) {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "lifecycle_digest_mismatch",
            "canonical cards or intent plan changed without a native transaction",
        )]);
    }
    Ok(())
}

pub fn recovery_source(
    state_root: &Path,
    issue: u64,
) -> Result<Option<PathBuf>, Vec<DoctorFinding>> {
    let path = state_root.join(format!("transactions/{issue}.json"));
    if !path.exists() {
        return Ok(None);
    }
    let journal: LocalMutationJournal = serde_json::from_slice(
        &fs::read(&path).map_err(io_finding("recovery_journal_unreadable"))?,
    )
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "recovery_journal_invalid",
            "pending journal is invalid",
        )]
    })?;
    if journal.schema != "csdlc.v3.local_mutation_journal.v1"
        || journal.issue != issue
        || !matches!(journal.route.as_str(), "bind" | "edit")
        || journal.request_digest.len() != 64
        || !journal
            .request_digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "recovery_journal_invalid",
            "pending transaction identity is invalid",
        )]);
    }
    let live = state_root.join(format!("issues/{issue}"));
    let backup = state_root.join(format!(
        "issues/.issue-{issue}-{}-{}.backup",
        journal.route, journal.request_digest
    ));
    let selected = if live.join("index.json").exists() {
        live
    } else {
        backup
    };
    if !has_canonical_existing_ancestor(&selected) {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "recovery_storage_symlink_denied",
            "pending transaction state must be canonical",
        )]);
    }
    Ok(Some(selected))
}

pub fn pending_bind_identity(
    state_root: &Path,
    issue: u64,
) -> Result<Option<(String, PathBuf)>, Vec<DoctorFinding>> {
    let path = state_root.join(format!("transactions/{issue}.json"));
    if !path.exists() {
        return Ok(None);
    }
    if path
        .symlink_metadata()
        .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_invalid",
            "lifecycle mutation journal must be a regular file",
        )]);
    }
    let journal: LocalMutationJournal = serde_json::from_slice(
        &fs::read(path).map_err(io_finding("local_transaction_journal_read_failed"))?,
    )
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_invalid",
            "lifecycle mutation journal identity is invalid",
        )]
    })?;
    if journal.schema != "csdlc.v3.local_mutation_journal.v1"
        || journal.issue != issue
        || journal.request_digest.len() != 64
        || !journal
            .request_digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_mismatch",
            "lifecycle mutation journal identity is invalid",
        )]);
    }
    if journal.route != "bind" {
        return Ok(None);
    }
    let branch = journal.bind_branch.ok_or_else(|| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_bind_identity_missing",
            "bind recovery requires its exact branch identity",
        )]
    })?;
    let worktree = journal.bind_worktree.ok_or_else(|| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_bind_identity_missing",
            "bind recovery requires its exact worktree identity",
        )]
    })?;
    Ok(Some((branch, worktree)))
}

pub fn prepare(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    plan: &Value,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    validate_contract(request)?;
    plan_cards(request.issue, &request.registry_version, registry)?;
    validate_context("issue", request, context)?;
    let _lock = acquire_issue_mutation_lock(&context.state_root, request.issue)?;
    validate_context("issue", request, context)?;
    if local_transaction_journal_path(context, request.issue).exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_recovery_required",
            "prepare does not replay pending transactions",
        )]);
    }
    let issue_root = context.state_root.join(format!("issues/{}", request.issue));
    require_operational_cas(
        "issue",
        request,
        &inspect_lifecycle_issue_root(&issue_root, request.issue, "v3"),
    )?;
    initialize_operational_issue_with_plan(request, registry, &issue_root, Some(plan))
}

/// A preview does not write a token. Execution consumes the caller-retained exact
/// digest while holding the same issue lock as the original transaction owner.
pub fn recover(
    request: &LocalPreparationRequest,
    context: &OperationalLocalContext,
    execute: bool,
    expected: Option<&str>,
) -> Result<Value, Vec<DoctorFinding>> {
    validate_context("doctor", request, context)?;
    let inspect = || -> Result<(Option<LocalMutationJournal>, String), Vec<DoctorFinding>> {
        let journal = read_pending_local_transaction(context, request.issue)?;
        let source = recovery_source(&context.state_root, request.issue)?
            .unwrap_or_else(|| context.state_root.join(format!("issues/{}", request.issue)));
        let index = read_index_value(&source)?;
        let input = serde_json::json!({"issue":request.issue,"index":index,"head":context.expected_head_sha,"authority":context.expected_authority_selector_digest,"journal":journal});
        let digest = blake3::hash(&serde_json::to_vec(&input).map_err(|_| {
            vec![finding(
                PlanStatus::Failed,
                "recovery_preview_invalid",
                "cannot serialize recovery identity",
            )]
        })?)
        .to_hex()
        .to_string();
        Ok((journal, digest))
    };
    let (journal, digest) = inspect()?;
    if !execute {
        return Ok(
            serde_json::json!({"schema":"csdlc.v3.intent_recovery.v1","read_only":true,"status":if journal.is_some(){"recovery_required"}else{"expected_noop"},"preview_digest":digest,"transaction":journal}),
        );
    }
    if journal.is_none() {
        if expected != Some(digest.as_str()) {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "recovery_preview_stale",
                "execute requires the exact current preview digest",
            )]);
        }
        return Ok(
            serde_json::json!({"schema":"csdlc.v3.intent_recovery.v1","read_only":true,"performed_mutation":false,"status":"expected_noop","preview_digest":digest}),
        );
    }
    let _lock = acquire_issue_mutation_lock(&context.state_root, request.issue)?;
    validate_context("doctor", request, context)?;
    let (journal, current) = inspect()?;
    if expected != Some(current.as_str()) {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "recovery_preview_stale",
            "execute requires the exact current preview digest",
        )]);
    }
    let changed = journal.is_some();
    if changed {
        recover_pending_local_transaction(context, request.issue)?;
    }
    Ok(
        serde_json::json!({"schema":"csdlc.v3.intent_recovery.v1","read_only":!changed,"performed_mutation":changed,"status":if changed{"completed"}else{"expected_noop"},"preview_digest":current}),
    )
}

/// Convert admitted native card inputs without publishing a legacy index or stage.
/// Authority/topology admission is the caller's responsibility. Rendered text is
/// validated in memory; complete card values remain canonical semantic content.
pub(crate) fn prepared_inputs(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    plan: &Value,
    authority: crate::storage::semantic::Digest,
) -> Result<crate::storage::semantic::IssueInputs, Vec<DoctorFinding>> {
    use crate::storage::semantic::{AcceptedIntentPlan, IssueInputs, PlanStep};
    validate_contract(request)?;
    plan_cards(request.issue, &request.registry_version, registry)?;
    let mut accepted: AcceptedIntentPlan = serde_json::from_value(plan.clone()).map_err(|_| {
        vec![finding(
            PlanStatus::Failed,
            "intent_plan_invalid",
            "accepted intent plan is malformed",
        )]
    })?;
    for kind in REQUIRED_CARD_KINDS {
        let mut values = initial_card_values(request, registry, kind);
        if let Some(update) = request.card_updates.get(kind) {
            merge_json_object(&mut values, update);
        }
        // Derived identity is authoritative even for advanced native callers.
        let identity = initial_card_values(request, registry, kind);
        for key in [
            "schema",
            "issue",
            "issue_padded",
            "issue_url",
            "title",
            "branch",
            "repository",
            "worktree",
            "card",
        ] {
            values[key] = identity[key].clone();
        }
        let template = fs::read_to_string(
            registry
                .template_paths
                .get(kind)
                .expect("validated registry"),
        )
        .map_err(io_finding("template_read_failed"))?;
        let rendered = render_template(&template, &values);
        if !structure_valid(registry, kind, &rendered) {
            return Err(vec![finding(
                PlanStatus::Failed,
                "prepared_card_structure_invalid",
                "canonical prepared card failed structure validation",
            )]);
        }
        accepted.cards.insert(kind.into(), values);
    }
    let spp = &accepted.cards["spp"];
    let fields = [
        ("dependencies", vec!["dependencies_inline"]),
        (
            "inspect",
            vec!["repo_inputs_inline", "target_files_surfaces_inline"],
        ),
        ("implement", vec!["deliverables_inline"]),
        (
            "validate",
            vec!["validation_plan_inline", "acceptance_criteria_inline"],
        ),
        ("record", vec!["notes_risks_inline"]),
    ];
    let mut steps = Vec::new();
    for (id, fields) in fields {
        let mut text = Vec::new();
        for field in fields {
            let value = spp[field]
                .as_str()
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| {
                    vec![finding(
                        PlanStatus::Failed,
                        "semantic_plan_field_missing",
                        &format!("SPP requires {field} before semantic preparation"),
                    )]
                })?;
            text.push(value);
        }
        steps.push(PlanStep {
            id: id.into(),
            acceptance: text.join("\n"),
        });
    }
    IssueInputs::new(request.title.clone(), accepted, steps, None, authority).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "semantic_inputs_invalid",
            &format!("{error:?}"),
        )]
    })
}

pub(crate) fn prepare_semantic(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    plan: &Value,
    root: &crate::storage::semantic::SemanticRoot,
    key: crate::storage::semantic::IssueKey,
    authority: crate::storage::semantic::Digest,
) -> Result<crate::storage::semantic::Snapshot, Vec<DoctorFinding>> {
    validate_context("issue", request, context)?;
    if key.issue() != request.issue || key.repository() != request.repository {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_identity_mismatch",
            "native and semantic issue identity must match",
        )]);
    }
    let inputs = prepared_inputs(request, registry, plan, authority)?;
    validate_context("issue", request, context)?;
    match crate::storage::DurableTransactionStore::prepare_issue(root, key, inputs) {
        Ok(crate::storage::semantic::CommitOutcome::Committed(snapshot)) => Ok(*snapshot),
        Ok(crate::storage::semantic::CommitOutcome::Unchanged(snapshot)) => Ok(*snapshot),
        Err(error) => Err(vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_refused",
            &format!("{error:?}"),
        )]),
    }
}
