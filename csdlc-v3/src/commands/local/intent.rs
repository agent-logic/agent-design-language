//! Narrow intent bridges into the existing local owner and transaction journal.
use super::*;

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
