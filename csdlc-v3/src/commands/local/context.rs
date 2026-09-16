//! Context for the native local owner.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde_json::Value;

use super::filesystem::io_finding;
use super::transactions::{
    local_request_digest, local_transaction_paths, read_pending_local_transaction,
};
use super::worktree::{
    canonical_existing_ancestor_local, git_worktree_registration, has_canonical_existing_ancestor,
};
use super::{
    finding, is_observation_route, DoctorFinding, LocalLifecycleStateObservation,
    LocalPreparationRequest, OperationalLocalContext, PlanStatus,
};

pub fn discover_operational_local_context(
    repository_root: &Path,
    request: &LocalPreparationRequest,
) -> Result<Option<OperationalLocalContext>, Vec<DoctorFinding>> {
    let repository_root = repository_root.canonicalize().map_err(|_| {
        vec![finding(
            PlanStatus::Failed,
            "repository_root_unavailable",
            "operational repository root must be an existing canonical directory",
        )]
    })?;
    let selector_path = repository_root.join(crate::authority::SELECTOR_PATH);
    let selector_bytes = match fs::read(&selector_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(vec![finding(
                PlanStatus::Failed,
                "canonical_authority_selector_unreadable",
                &error.to_string(),
            )])
        }
    };
    let selector: Value = serde_json::from_slice(&selector_bytes).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_invalid",
            "canonical generation selector must be typed JSON",
        )]
    })?;
    if selector.get("schema").and_then(Value::as_str) != Some("csdlc.v3.authority_selector.v1")
        || selector.get("generation").and_then(Value::as_str) != Some("v3")
        || selector
            .get("operational_authority")
            .and_then(Value::as_str)
            != Some("csdlc-v3")
    {
        return Ok(None);
    }
    if crate::authority::canonical_v3_authority(&repository_root)
        .map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_invalid",
                &error,
            )]
        })?
        .is_none()
    {
        return Ok(None);
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(&repository_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(io_finding("repository_head_unreadable"))?;
    if !head.status.success() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "repository_head_unreadable",
            "operational context requires an exact checkout head",
        )]);
    }
    let expected_head_sha = String::from_utf8_lossy(&head.stdout).trim().to_owned();
    let approval_path = PathBuf::from(crate::authority::SELECTOR_PATH);
    let approval_digest = blake3::hash(&selector_bytes).to_hex().to_string();
    let worktree_policy: Value = serde_json::from_slice(
        &fs::read(repository_root.join(".adl/worktree-policy.json"))
            .map_err(io_finding("worktree_policy_unreadable"))?,
    )
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "worktree_policy_invalid",
            "tracked worktree policy must be typed JSON",
        )]
    })?;
    if worktree_policy["schema"] != "adl.worktree_policy.v1" {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "worktree_policy_invalid",
            "tracked worktree policy schema is not supported",
        )]);
    }
    let allowed_worktree_parent = PathBuf::from(
        worktree_policy["required_parent"]
            .as_str()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| {
                vec![finding(
                    PlanStatus::Blocked,
                    "worktree_policy_parent_missing",
                    "tracked worktree policy must declare required_parent",
                )]
            })?,
    )
    .canonicalize()
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "worktree_parent_unavailable",
            "operational worktree parent must already exist",
        )]
    })?;
    Ok(Some(OperationalLocalContext {
        state_root: operational_state_root(&repository_root)?,
        allowed_worktree_parent,
        expected_authority_selector_digest: blake3::hash(&selector_bytes).to_hex().to_string(),
        cutover_approval_path: approval_path,
        expected_cutover_approval_digest: approval_digest,
        expected_head_sha,
        expected_lifecycle_digest: request.expected_lifecycle_digest.clone(),
        repository_root,
    }))
}
/// Preparation belongs to Git metadata. Only a linked checkout materializes cards.
pub fn operational_state_root(repository_root: &Path) -> Result<PathBuf, Vec<DoctorFinding>> {
    let git_path = |flag: &str| -> Result<PathBuf, Vec<DoctorFinding>> {
        let output = Command::new("git")
            .arg("-C")
            .arg(repository_root)
            .args(["rev-parse", "--path-format=absolute", flag])
            .output()
            .map_err(io_finding("git_metadata_unavailable"))?;
        if !output.status.success() {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "git_metadata_unavailable",
                "native lifecycle storage requires resolved Git metadata",
            )]);
        }
        PathBuf::from(String::from_utf8_lossy(&output.stdout).trim())
            .canonicalize()
            .map_err(io_finding("git_metadata_unavailable"))
    };
    let git_dir = git_path("--git-dir")?;
    let common_dir = git_path("--git-common-dir")?;
    Ok(if git_dir == common_dir {
        git_dir.join("csdlc-v3/local")
    } else {
        repository_root.join(".csdlc")
    })
}
pub(super) fn validate_context(
    route: &str,
    request: &LocalPreparationRequest,
    context: &OperationalLocalContext,
) -> Result<(), Vec<DoctorFinding>> {
    let repository_root = context.repository_root.canonicalize().map_err(|_| {
        vec![finding(
            PlanStatus::Failed,
            "repository_root_unavailable",
            "operational repository root must be an existing canonical directory",
        )]
    })?;
    let state_root = &context.state_root;
    let expected_state_root = operational_state_root(&repository_root)?;
    let is_primary = expected_state_root != repository_root.join(".csdlc");
    if state_root != &expected_state_root
        || state_root.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir | std::path::Component::Prefix(_)
            )
        })
        || !has_canonical_existing_ancestor(state_root)
    {
        return Err(vec![finding(
            PlanStatus::Failed,
            "state_root_outside_repository",
            "operational state root must match Git metadata preparation or the exact linked checkout",
        )]);
    }
    for child in [
        "issues".to_owned(),
        "transactions".to_owned(),
        "locks".to_owned(),
        "bindings".to_owned(),
        format!("issues/{}", request.issue),
        format!("transactions/completed/{}", request.issue),
        format!("transactions/{}.json", request.issue),
        format!("bindings/{}.json", request.issue),
    ] {
        if !has_canonical_existing_ancestor(&state_root.join(child)) {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_storage_symlink_denied",
                "lifecycle storage directories must not redirect writes through symlinks",
            )]);
        }
    }
    if is_primary {
        if route != "bind"
            && state_root
                .join(format!("bindings/{}.json", request.issue))
                .symlink_metadata()
                .is_ok()
        {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "issue_already_bound",
                "the bound checkout owns this issue; primary preparation and edits are denied",
            )]);
        }
        for legacy in [
            format!(".csdlc/issues/{}", request.issue),
            format!(".csdlc/prepared/issues/{}", request.issue),
            format!(".csdlc/transactions/{}.json", request.issue),
            format!(".csdlc/transactions/completed/{}", request.issue),
        ] {
            if repository_root.join(legacy).symlink_metadata().is_ok() {
                return Err(vec![finding(PlanStatus::Blocked, "legacy_primary_state_requires_recovery",
                    "preserve legacy primary state into a reviewed recovery location before metadata-backed preparation; no automatic migration is permitted")]);
            }
        }
        if route == "issue"
            && git_worktree_registration(
                &repository_root,
                &request.branch,
                Path::new(&request.worktree),
            )?
        {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "issue_already_bound",
                "an existing bound checkout owns this issue; initialization on primary is denied",
            )]);
        }
    } else {
        let matches_binding = |path: &Path| {
            if !canonical_existing_ancestor_local(path)
                .is_some_and(|ancestor| ancestor.starts_with(state_root))
                || !has_canonical_existing_ancestor(path)
            {
                return false;
            }
            let index: Option<Value> = fs::read(path)
                .ok()
                .and_then(|bytes| serde_json::from_slice(&bytes).ok());
            index.is_some_and(|value| {
                value["schema"] == "csdlc.v3.local_state.v1"
                    && value["issue"] == request.issue
                    && value["phase"] == "bound"
                    && value["repository"] == request.repository
                    && value["branch"] == request.branch
                    && value["worktree"] == request.worktree
            })
        };
        let binding_matches =
            matches_binding(&state_root.join(format!("issues/{}/index.json", request.issue)));
        // Directory-swap recovery still requires the exact binding retained
        // in the journal's backup or stage. Only observation or an identical
        // edit retry may use it; a different mutation cannot recover it first.
        let interrupted_binding_matches = if binding_matches {
            false
        } else if let Some(journal) = read_pending_local_transaction(context, request.issue)? {
            let (stage, backup, _) = local_transaction_paths(
                context,
                request.issue,
                &journal.route,
                &journal.request_digest,
            );
            journal.route == "edit"
                && (is_observation_route(route)
                    || (route == "edit"
                        && local_request_digest(request)? == journal.request_digest))
                && [backup, stage]
                    .iter()
                    .any(|path| matches_binding(&path.join("index.json")))
        } else {
            false
        };
        let bound_checkout = !matches!(route, "issue" | "bind")
            && (binding_matches || interrupted_binding_matches)
            && repository_root.starts_with(&context.allowed_worktree_parent)
            && repository_root != context.allowed_worktree_parent
            && repository_root.join(".git").is_file()
            && Path::new(&request.worktree)
                .canonicalize()
                .is_ok_and(|path| path == repository_root)
            && git_worktree_registration(&repository_root, &request.branch, &repository_root)?;
        if !bound_checkout {
            return Err(vec![finding(
                PlanStatus::Failed,
                "invalid_operational_roots",
                "a root under the worktree parent must be the exact registered issue checkout",
            )]);
        }
    }
    if request.expected_lifecycle_digest != context.expected_lifecycle_digest {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "operational_lifecycle_digest_mismatch",
            "serialized operational context and typed request must bind the same lifecycle digest",
        )]);
    }

    let selector_path = context
        .repository_root
        .join(crate::authority::SELECTOR_PATH);
    let selector_bytes = fs::read(&selector_path).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_v3_authority_missing",
            "canonical generation selector is unavailable; v3 local mutations remain denied",
        )]
    })?;
    let selector_digest = blake3::hash(&selector_bytes).to_hex().to_string();
    if context.expected_authority_selector_digest.trim().is_empty()
        || context.expected_authority_selector_digest != selector_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_digest_mismatch",
            "serialized authority digest does not match the canonical generation selector",
        )]);
    }
    let selector: Value = serde_json::from_slice(&selector_bytes).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_invalid",
            "canonical generation selector must be typed JSON",
        )]
    })?;
    if selector.get("schema").and_then(Value::as_str) != Some("csdlc.v3.authority_selector.v1")
        || selector.get("generation").and_then(Value::as_str) != Some("v3")
        || selector
            .get("operational_authority")
            .and_then(Value::as_str)
            != Some("csdlc-v3")
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "canonical_v3_authority_inactive",
            "canonical generation selector does not grant v3 operational authority",
        )]);
    }

    let _authority = crate::authority::canonical_v3_authority(&repository_root)
        .map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_invalid",
                &error,
            )]
        })?
        .ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_inactive",
                "tracked v3 selector is not active on origin/main",
            )]
        })?;
    if context.cutover_approval_path != Path::new(crate::authority::SELECTOR_PATH)
        || context.expected_cutover_approval_digest != context.expected_authority_selector_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_authority_mismatch",
            "operational context does not bind the canonical merge-derived selector authority",
        )]);
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(&repository_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(io_finding("git_head_observation_failed"))?;
    if !head.status.success()
        || String::from_utf8_lossy(&head.stdout).trim() != context.expected_head_sha
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "operational_exact_head_mismatch",
            "live repository HEAD does not match digest-bound #505 approval evidence",
        )]);
    }

    if is_observation_route(route) {
        return Ok(());
    }
    fs::create_dir_all(&context.state_root).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "state_root_create_failed",
            &error.to_string(),
        )]
    })
}
pub(super) fn require_operational_cas(
    route: &str,
    request: &LocalPreparationRequest,
    observed: &LocalLifecycleStateObservation,
) -> Result<(), Vec<DoctorFinding>> {
    if observed.code == "missing_local_lifecycle_state" {
        if route == "issue" && request.expected_lifecycle_digest.is_none() {
            return Ok(());
        }
        return Err(vec![finding(
            PlanStatus::Blocked,
            "missing_local_lifecycle_state",
            "only issue initialization may operate without existing lifecycle state",
        )]);
    }
    let expected = request
        .expected_lifecycle_digest
        .as_deref()
        .ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_lifecycle_digest_required",
                "an existing lifecycle record requires expected_lifecycle_digest",
            )]
        })?;
    if observed.digest.as_deref() != Some(expected) {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "stale_local_lifecycle_digest",
            "observed local lifecycle digest does not match the typed request",
        )]);
    }
    Ok(())
}
