//! Transactions for the native local owner.

use std::{
    fs,
    path::{Path, PathBuf},
};

use fs2::FileExt;

use super::lifecycle::inspect_lifecycle_issue_root;
use super::storage::{atomic_write_json, io_finding, local_transaction_failpoint};
use super::worktree::{
    ensure_bind_registration, git_worktree_registration, has_canonical_existing_ancestor,
    verify_bound_worktree,
};
use super::{
    finding, DoctorFinding, LocalMutationCompletion, LocalMutationJournal, LocalPreparationRequest,
    OperationalLocalContext, OperationalLocalResult, PlanStatus,
};

pub(super) fn acquire_issue_mutation_lock(
    state_root: &Path,
    issue: u64,
) -> Result<fs::File, Vec<DoctorFinding>> {
    let lock_root = state_root.join("locks");
    fs::create_dir_all(&lock_root).map_err(io_finding("issue_lock_parent_create_failed"))?;
    let lock_path = lock_root.join(format!("{issue}.lock"));
    if lock_path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err(vec![finding(
            PlanStatus::Failed,
            "issue_lock_symlink_denied",
            "issue mutation lock must not be a symlink",
        )]);
    }
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)
        .map_err(io_finding("issue_lock_open_failed"))?;
    file.lock_exclusive()
        .map_err(io_finding("issue_lock_acquire_failed"))?;
    Ok(file)
}
pub(super) fn local_request_digest(
    request: &LocalPreparationRequest,
) -> Result<String, Vec<DoctorFinding>> {
    serde_json::to_vec(request)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "local_request_digest_failed",
                &error.to_string(),
            )]
        })
}
pub(super) fn local_transaction_root(context: &OperationalLocalContext) -> PathBuf {
    context.state_root.join("transactions")
}
pub(super) fn local_transaction_journal_path(
    context: &OperationalLocalContext,
    issue: u64,
) -> PathBuf {
    local_transaction_root(context).join(format!("{issue}.json"))
}
pub(super) fn local_transaction_paths(
    context: &OperationalLocalContext,
    issue: u64,
    route: &str,
    request_digest: &str,
) -> (PathBuf, PathBuf, PathBuf) {
    let issue_parent = context.state_root.join("issues");
    let suffix = format!(".issue-{issue}-{route}-{request_digest}");
    let stage = issue_parent.join(format!("{suffix}.stage"));
    let backup = issue_parent.join(format!("{suffix}.backup"));
    let completion = local_transaction_root(context)
        .join("completed")
        .join(issue.to_string())
        .join(format!("{route}-{request_digest}.json"));
    (stage, backup, completion)
}
pub(super) fn prepare_local_transaction_stage(
    context: &OperationalLocalContext,
    issue: u64,
    route: &str,
    request_digest: &str,
) -> Result<PathBuf, Vec<DoctorFinding>> {
    let issue_root = context.state_root.join(format!("issues/{issue}"));
    let (stage, backup, _) = local_transaction_paths(context, issue, route, request_digest);
    if backup.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_orphaned_backup",
            "an unjournaled lifecycle backup requires operator recovery",
        )]);
    }
    if stage.exists() {
        fs::remove_dir_all(&stage).map_err(io_finding("local_transaction_stage_cleanup_failed"))?;
    }
    copy_local_issue_tree(&issue_root, &stage)?;
    Ok(stage)
}
pub(super) fn copy_local_issue_tree(
    source: &Path,
    destination: &Path,
) -> Result<(), Vec<DoctorFinding>> {
    let metadata = source
        .symlink_metadata()
        .map_err(io_finding("local_transaction_source_unavailable"))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_source_invalid",
            "lifecycle transaction source must be a real directory",
        )]);
    }
    fs::create_dir(destination).map_err(io_finding("local_transaction_stage_create_failed"))?;
    for entry in fs::read_dir(source).map_err(io_finding("local_transaction_source_read_failed"))? {
        let entry = entry.map_err(io_finding("local_transaction_entry_read_failed"))?;
        let file_type = entry
            .file_type()
            .map_err(io_finding("local_transaction_entry_metadata_failed"))?;
        if file_type.is_symlink() {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_symlink_denied",
                "lifecycle transaction images cannot contain symlinks",
            )]);
        }
        let target = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_local_issue_tree(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target)
                .map_err(io_finding("local_transaction_file_copy_failed"))?;
        } else {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_special_file_denied",
                "lifecycle transaction images may contain only files and directories",
            )]);
        }
    }
    Ok(())
}
pub(super) fn begin_local_transaction(
    context: &OperationalLocalContext,
    journal: LocalMutationJournal,
) -> Result<(), Vec<DoctorFinding>> {
    let journal_path = local_transaction_journal_path(context, journal.issue);
    if journal_path.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_already_pending",
            "a lifecycle mutation journal must be recovered before another mutation starts",
        )]);
    }
    fs::create_dir_all(
        journal_path
            .parent()
            .expect("transaction journal has a parent"),
    )
    .map_err(io_finding("local_transaction_root_create_failed"))?;
    let value = serde_json::to_value(&journal).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "local_transaction_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write_json(&journal_path, &value)
}
pub(super) fn read_pending_local_transaction(
    context: &OperationalLocalContext,
    issue: u64,
) -> Result<Option<LocalMutationJournal>, Vec<DoctorFinding>> {
    let journal_path = local_transaction_journal_path(context, issue);
    if !journal_path.exists() {
        return Ok(None);
    }
    if journal_path
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
        &fs::read(&journal_path).map_err(io_finding("local_transaction_journal_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_invalid",
            &error.to_string(),
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
            "local_transaction_journal_mismatch",
            "lifecycle mutation journal identity is invalid",
        )]);
    }
    Ok(Some(journal))
}
pub(super) fn recover_pending_local_transaction(
    context: &OperationalLocalContext,
    issue: u64,
) -> Result<(), Vec<DoctorFinding>> {
    let Some(journal) = read_pending_local_transaction(context, issue)? else {
        return Ok(());
    };
    if journal.route == "bind" {
        let branch = journal.bind_branch.as_deref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_transaction_bind_identity_missing",
                "bind recovery requires its exact branch identity",
            )]
        })?;
        let target = journal.bind_worktree.as_deref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_transaction_bind_identity_missing",
                "bind recovery requires its exact worktree identity",
            )]
        })?;
        ensure_bind_registration(
            &context.repository_root,
            branch,
            target,
            &context.expected_head_sha,
            false,
        )?;
    }
    if journal.route == "bind" {
        let target = journal.bind_worktree.as_deref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_transaction_bind_identity_missing",
                "bind recovery requires its exact worktree identity",
            )]
        })?;
        commit_bind_local_transaction(context, &journal, target)
    } else {
        commit_local_transaction(context, &journal)
    }
}
pub(super) fn commit_pending_local_transaction(
    context: &OperationalLocalContext,
    issue: u64,
) -> Result<(), Vec<DoctorFinding>> {
    recover_pending_local_transaction(context, issue)
}
pub(super) fn commit_local_transaction(
    context: &OperationalLocalContext,
    journal: &LocalMutationJournal,
) -> Result<(), Vec<DoctorFinding>> {
    let issue_root = context.state_root.join(format!("issues/{}", journal.issue));
    let journal_path = local_transaction_journal_path(context, journal.issue);
    let (stage, backup, completion) = local_transaction_paths(
        context,
        journal.issue,
        &journal.route,
        &journal.request_digest,
    );
    if issue_root.exists() && stage.exists() && !backup.exists() {
        fs::rename(&issue_root, &backup)
            .map_err(io_finding("local_transaction_backup_commit_failed"))?;
        local_transaction_failpoint("after_backup_rename");
    }
    if !issue_root.exists() && stage.exists() && backup.exists() {
        fs::rename(&stage, &issue_root)
            .map_err(io_finding("local_transaction_stage_commit_failed"))?;
        local_transaction_failpoint("after_stage_rename");
    }
    if !issue_root.exists() || stage.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_state_ambiguous",
            "lifecycle transaction cannot reconcile its issue, stage, and backup state",
        )]);
    }
    let completed = LocalMutationCompletion {
        schema: "csdlc.v3.local_mutation_completion.v1".into(),
        issue: journal.issue,
        route: journal.route.clone(),
        request_digest: journal.request_digest.clone(),
        result: journal.result.clone(),
    };
    fs::create_dir_all(completion.parent().expect("completion has a parent"))
        .map_err(io_finding("local_transaction_completion_parent_failed"))?;
    let value = serde_json::to_value(completed).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "local_transaction_completion_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write_json(&completion, &value)?;
    if backup.exists() {
        fs::remove_dir_all(&backup)
            .map_err(io_finding("local_transaction_backup_cleanup_failed"))?;
    }
    fs::remove_file(&journal_path).map_err(io_finding("local_transaction_journal_cleanup_failed"))
}
pub(super) fn commit_bind_local_transaction(
    context: &OperationalLocalContext,
    journal: &LocalMutationJournal,
    target: &Path,
) -> Result<(), Vec<DoctorFinding>> {
    let source_issue_root = context.state_root.join(format!("issues/{}", journal.issue));
    let source_journal_path = local_transaction_journal_path(context, journal.issue);
    let (source_stage, source_backup, _) = local_transaction_paths(
        context,
        journal.issue,
        &journal.route,
        &journal.request_digest,
    );
    let target_state_root = target.join(".csdlc");
    if target_state_root
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bind_target_state_root_symlink_denied",
            "bound lifecycle state root must not be a symlink",
        )]);
    }
    let target_context = OperationalLocalContext {
        repository_root: target.to_path_buf(),
        state_root: target_state_root,
        allowed_worktree_parent: context.allowed_worktree_parent.clone(),
        expected_authority_selector_digest: context.expected_authority_selector_digest.clone(),
        cutover_approval_path: context.cutover_approval_path.clone(),
        expected_cutover_approval_digest: context.expected_cutover_approval_digest.clone(),
        expected_head_sha: context.expected_head_sha.clone(),
        expected_lifecycle_digest: context.expected_lifecycle_digest.clone(),
    };
    let target_issue_root = target_context
        .state_root
        .join(format!("issues/{}", journal.issue));
    let (target_stage, target_backup, target_completion) = local_transaction_paths(
        &target_context,
        journal.issue,
        &journal.route,
        &journal.request_digest,
    );
    for path in [
        &target_issue_root,
        &target_stage,
        &target_backup,
        &target_completion,
    ] {
        if !has_canonical_existing_ancestor(path) {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_storage_symlink_denied",
                "bind target paths must not redirect writes through symlinks",
            )]);
        }
    }
    if target_backup.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_orphaned_backup",
            "an unjournaled lifecycle backup requires operator recovery",
        )]);
    }
    if target_issue_root.exists() && target_stage.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_state_ambiguous",
            "bind transaction cannot reconcile target issue and stage state",
        )]);
    }
    if source_issue_root.exists() && source_stage.exists() && !source_backup.exists() {
        fs::rename(&source_issue_root, &source_backup)
            .map_err(io_finding("local_transaction_backup_commit_failed"))?;
        local_transaction_failpoint("after_backup_rename");
    }
    if !target_issue_root.exists() {
        fs::create_dir_all(
            target_stage
                .parent()
                .expect("target bind stage has a parent"),
        )
        .map_err(io_finding("local_transaction_target_parent_failed"))?;
        if !target_stage.exists() {
            copy_local_issue_tree(&source_stage, &target_stage)?;
            local_transaction_failpoint("bind_after_target_stage_copy");
        }
        fs::rename(&target_stage, &target_issue_root)
            .map_err(io_finding("local_transaction_target_stage_commit_failed"))?;
        local_transaction_failpoint("bind_after_target_stage_rename");
    }
    if !target_issue_root.exists() || target_stage.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_state_ambiguous",
            "bind transaction cannot reconcile source and target lifecycle state",
        )]);
    }
    let completed = LocalMutationCompletion {
        schema: "csdlc.v3.local_mutation_completion.v1".into(),
        issue: journal.issue,
        route: journal.route.clone(),
        request_digest: journal.request_digest.clone(),
        result: journal.result.clone(),
    };
    fs::create_dir_all(target_completion.parent().expect("completion has a parent"))
        .map_err(io_finding("local_transaction_completion_parent_failed"))?;
    let value = serde_json::to_value(completed).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "local_transaction_completion_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write_json(&target_completion, &value)?;
    // Retain the handoff identity outside the working tree so a fresh issue
    // request cannot accidentally recreate preparation after binding.
    let binding_path = context
        .state_root
        .join(format!("bindings/{}.json", journal.issue));
    fs::create_dir_all(binding_path.parent().expect("binding parent"))
        .map_err(io_finding("bind_identity_parent_failed"))?;
    atomic_write_json(
        &binding_path,
        &serde_json::json!({
            "schema": "csdlc.v3.binding.v1", "issue": journal.issue,
            "branch": journal.bind_branch, "worktree": target
        }),
    )?;
    if source_stage.exists() {
        fs::remove_dir_all(&source_stage)
            .map_err(io_finding("local_transaction_stage_cleanup_failed"))?;
    }
    if source_backup.exists() {
        fs::remove_dir_all(&source_backup)
            .map_err(io_finding("local_transaction_backup_cleanup_failed"))?;
    }
    fs::remove_file(&source_journal_path)
        .map_err(io_finding("local_transaction_journal_cleanup_failed"))
}
pub(super) fn load_local_completion(
    context: &OperationalLocalContext,
    request: &LocalPreparationRequest,
    route: &str,
    request_digest: &str,
) -> Result<Option<OperationalLocalResult>, Vec<DoctorFinding>> {
    if !matches!(route, "bind" | "edit") {
        return Ok(None);
    }
    let (_, _, path) = local_transaction_paths(context, request.issue, route, request_digest);
    let (completion_path, completion_state_root) = if path.exists() {
        (path, context.state_root.clone())
    } else if route == "bind" {
        let target_context = OperationalLocalContext {
            repository_root: PathBuf::from(&request.worktree),
            state_root: PathBuf::from(&request.worktree).join(".csdlc"),
            allowed_worktree_parent: context.allowed_worktree_parent.clone(),
            expected_authority_selector_digest: context.expected_authority_selector_digest.clone(),
            cutover_approval_path: context.cutover_approval_path.clone(),
            expected_cutover_approval_digest: context.expected_cutover_approval_digest.clone(),
            expected_head_sha: context.expected_head_sha.clone(),
            expected_lifecycle_digest: context.expected_lifecycle_digest.clone(),
        };
        let (_, _, target_path) =
            local_transaction_paths(&target_context, request.issue, route, request_digest);
        if target_path.exists() {
            (target_path, target_context.state_root)
        } else {
            return Ok(None);
        }
    } else {
        return Ok(None);
    };
    let completion: LocalMutationCompletion = serde_json::from_slice(
        &fs::read(completion_path)
            .map_err(io_finding("local_transaction_completion_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_invalid",
            &error.to_string(),
        )]
    })?;
    if completion.schema != "csdlc.v3.local_mutation_completion.v1"
        || completion.issue != request.issue
        || completion.route != route
        || completion.request_digest != request_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_mismatch",
            "lifecycle mutation completion does not match the exact request",
        )]);
    }
    let issue_root = completion_state_root.join(format!("issues/{}", request.issue));
    let observed = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
    if completion.result.route != route
        || completion.result.issue != request.issue
        || !completion.result.mutated
        || completion.result.phase != observed.phase
        || completion.result.generation != observed.generation
        || completion.result.digest != observed.digest
        || !observed.ready_to_execute
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_state_mismatch",
            "lifecycle mutation completion does not reconcile to the live issue state",
        )]);
    }
    if route == "bind" {
        if !git_worktree_registration(
            &context.repository_root,
            &request.branch,
            Path::new(&request.worktree),
        )? {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_completion_bind_mismatch",
                "bind completion does not reconcile to the exact Git worktree registration",
            )]);
        }
        verify_bound_worktree(
            &request.branch,
            Path::new(&request.worktree),
            &context.expected_head_sha,
            false,
        )?;
    }
    Ok(Some(completion.result))
}
