//! Binding for the native local owner.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use super::failpoints::local_transaction_failpoint;
use super::filesystem::atomic_write_json;
use super::lifecycle::inspect_lifecycle_issue_root;
use super::results::operational_result;
use super::storage::{persist_index, read_index_value};
use super::transactions::{
    begin_local_transaction, commit_pending_local_transaction, local_request_digest,
    prepare_local_transaction_stage,
};
use super::worktree::{
    canonical_existing_ancestor_local, ensure_bind_registration, git_worktree_registration,
};
use super::{
    finding, DoctorFinding, LocalMutationJournal, LocalPreparationRequest, OperationalLocalContext,
    OperationalLocalResult, PlanStatus, PromptRegistry,
};

pub(super) fn bind_operational_issue(
    request: &LocalPreparationRequest,
    context: &OperationalLocalContext,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    if observed.phase.as_deref() != Some("ready") {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bind_phase_invalid",
            "bind requires ready lifecycle state",
        )]);
    }
    let target = PathBuf::from(&request.worktree);
    let allowed_parent = context
        .allowed_worktree_parent
        .canonicalize()
        .map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "worktree_parent_unavailable",
                "allowed worktree parent must be an existing canonical directory",
            )]
        })?;
    let target_parent_inside_policy = target
        .parent()
        .and_then(canonical_existing_ancestor_local)
        .is_some_and(|ancestor| ancestor.starts_with(&allowed_parent));
    if !target.is_absolute()
        || target.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir | std::path::Component::Prefix(_)
            )
        })
        || !target.starts_with(&context.allowed_worktree_parent)
        || !target_parent_inside_policy
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "worktree_outside_policy",
            "operational bind requires an absolute path below the allowed worktree parent",
        )]);
    }
    if target == context.repository_root {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "primary_worktree_denied",
            "bind cannot target the primary checkout",
        )]);
    }
    if !git_worktree_registration(&context.repository_root, &request.branch, &target)?
        && target.exists()
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "unregistered_worktree_exists",
            "target path exists but is not registered for the requested branch",
        )]);
    }
    let request_digest = local_request_digest(request)?;
    let stage = prepare_local_transaction_stage(context, request.issue, "bind", &request_digest)?;
    atomic_write_json(
        &stage.join("binding.json"),
        &serde_json::json!({
            "schema": "csdlc.v3.binding.v1",
            "issue": request.issue,
            "branch": request.branch,
            "worktree": request.worktree
        }),
    )?;
    let index = read_index_value(&stage)?;
    let registry_version = index["template_registry_version"]
        .as_str()
        .unwrap_or(&request.registry_version);
    let registry = PromptRegistry {
        version: registry_version.to_owned(),
        card_kinds: BTreeSet::new(),
        template_paths: BTreeMap::new(),
    };
    let generation = observed.generation.unwrap_or(1) + 1;
    let (generation, digest) = persist_index(&stage, request, &registry, "bound", generation)?;
    let result = operational_result(
        "bind",
        request.issue,
        true,
        "bound",
        generation,
        digest,
        Some("edit"),
        vec![],
    );
    begin_local_transaction(
        context,
        LocalMutationJournal {
            schema: "csdlc.v3.local_mutation_journal.v1".into(),
            issue: request.issue,
            route: "bind".into(),
            request_digest,
            result: result.clone(),
            bind_branch: Some(request.branch.clone()),
            bind_worktree: Some(target.clone()),
        },
    )?;
    ensure_bind_registration(
        &context.repository_root,
        &request.branch,
        &target,
        &context.expected_head_sha,
        true,
    )?;
    local_transaction_failpoint("bind_after_git");
    commit_pending_local_transaction(context, request.issue)?;
    Ok(result)
}
