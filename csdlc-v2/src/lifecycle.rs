use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::cards::validate_initial_input;
use crate::doctor::DoctorStatus;
use crate::error::{ErrorCode, Result, V2Error};
use crate::git;
use crate::model::AuditEvent;
use crate::store::{
    bootstrap_issue, read_regular_authored_artifact, require_canonical_parent_beneath,
    require_regular_or_absent_beneath, validate_bootstrap_request, BootstrapRequest, Store,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BindRequest {
    pub issue: u64,
    pub base_branch: String,
    pub branch: String,
    pub worktree: String,
    #[serde(default)]
    pub code_repository: Option<String>,
    #[serde(default)]
    pub expected_repository: Option<String>,
    #[serde(default)]
    pub adopt_existing: bool,
    #[serde(default)]
    pub expected_head: Option<String>,
    #[serde(default)]
    pub expected_generation: Option<u64>,
    #[serde(default)]
    pub expected_digest: Option<String>,
    #[serde(default)]
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BindResult {
    pub created: bool,
    pub branch: String,
    pub worktree: String,
    #[serde(default)]
    pub adopted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resulting_digest: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorktreePolicy {
    schema: String,
    required_parent: String,
}

fn requires_worktree_policy(repository: &str) -> bool {
    repository.eq_ignore_ascii_case("agent-logic/agent-design-language")
}

fn enforce_worktree_policy(root: &Path, requested: &Path, required: bool) -> Result<()> {
    let policy_path = root.join(".adl/worktree-policy.json");
    if !policy_path.is_file() {
        return if required {
            Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "canonical ADL repository is missing .adl/worktree-policy.json",
            ))
        } else {
            Ok(())
        };
    }
    let policy: WorktreePolicy = serde_json::from_slice(&fs::read(&policy_path)?)?;
    if policy.schema != "adl.worktree_policy.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "unsupported ADL worktree policy schema",
        ));
    }
    let parent = requested_worktree(root, &policy.required_parent)?;
    if requested == parent || !requested.starts_with(&parent) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "worktree must be a child of the required parent {}",
                parent.display()
            ),
        ));
    }
    Ok(())
}

fn existing_bound_issue_local(
    phase: crate::LifecyclePhase,
    stored_branch: Option<&str>,
    requested: &Path,
    current_root: &Path,
    current_branch: &str,
    requested_branch: &str,
    stored_worktree: Option<&Path>,
) -> bool {
    requested == current_root
        && current_branch == requested_branch
        && matches!(phase, crate::LifecyclePhase::Bound)
        && stored_branch == Some(requested_branch)
        && stored_worktree == Some(requested)
}

fn clean_relative(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn valid_repository_identity(value: &str) -> bool {
    let mut parts = value.split('/');
    matches!((parts.next(), parts.next(), parts.next()), (Some(owner), Some(repo), None) if !owner.is_empty() && !repo.is_empty())
}

fn requested_worktree(root: &Path, value: &str) -> Result<PathBuf> {
    if value == "." {
        return Ok(root.canonicalize()?);
    }
    let path = Path::new(value);
    if !path.is_absolute() && !clean_relative(value) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "worktree must be an absolute path or a clean repository-relative path",
        ));
    }
    let requested = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    if requested.exists() {
        return Ok(requested.canonicalize()?);
    }
    let mut ancestor = requested.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(ancestor.file_name().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "worktree path has no existing ancestor",
            )
        })?);
        ancestor = ancestor.parent().ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "worktree path has no existing ancestor",
            )
        })?;
    }
    let mut normalized = ancestor.canonicalize()?;
    for component in suffix.into_iter().rev() {
        normalized.push(component);
    }
    Ok(normalized)
}

fn valid_branch(root: &Path, branch: &str) -> bool {
    !branch.trim().is_empty()
        && Command::new("git")
            .current_dir(root)
            .args(["check-ref-format", "--branch", branch])
            .output()
            .is_ok_and(|output| output.status.success())
}

fn branch_exists(root: &Path, branch: &str) -> bool {
    Command::new("git")
        .current_dir(root)
        .args([
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .status()
        .is_ok_and(|status| status.success())
}

fn require_bind_adoption_text(value: &Option<String>, field: &str) -> Result<String> {
    let Some(value) = value.as_ref() else {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("adoption requires {field}"),
        ));
    };
    if value.trim().is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("adoption requires non-empty {field}"),
        ));
    }
    Ok(value.clone())
}

fn require_bind_adoption_u64(value: Option<u64>, field: &str) -> Result<u64> {
    value.ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            format!("adoption requires {field}"),
        )
    })
}

fn validate_bind_adoption_request(
    record: &crate::IssueRecord,
    request: &BindRequest,
) -> Result<()> {
    if !request.adopt_existing {
        return Ok(());
    }
    let expected_generation =
        require_bind_adoption_u64(request.expected_generation, "expected_generation")?;
    let expected_digest = require_bind_adoption_text(&request.expected_digest, "expected_digest")?;
    let expected_repository =
        require_bind_adoption_text(&request.expected_repository, "expected_repository")?;
    let expected_head = require_bind_adoption_text(&request.expected_head, "expected_head")?;
    let actor = require_bind_adoption_text(&request.actor, "actor")?;
    if expected_head.len() != 40 || !expected_head.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "adoption expected_head must be a full 40-character hex commit SHA",
        ));
    }
    if actor == "csdlc-bind" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "adoption actor must identify the requesting operator or session",
        ));
    }
    if record.phase != crate::LifecyclePhase::Ready {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "adoption requires a ready unbound issue",
        ));
    }
    if record.generation != expected_generation {
        return Err(V2Error::new(
            ErrorCode::StaleGeneration,
            "adoption expected_generation is stale",
        ));
    }
    if record.digest != expected_digest {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "adoption expected_digest is stale",
        ));
    }
    if !record.repository.eq_ignore_ascii_case(&expected_repository) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption expected_repository does not match issue authority",
        ));
    }
    if record.branch.is_some() || record.worktree.is_some() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption refuses an issue that already has typed Git topology",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone)]
struct AdoptionObservation {
    actor: String,
    expected_repository: String,
    expected_generation: u64,
    expected_digest: String,
    expected_head: String,
    observed_head: String,
    base_branch: String,
    branch: String,
    worktree: String,
}

fn verify_bind_adoption_topology(
    request: &BindRequest,
    listed: &[(String, String)],
    wanted: &Path,
    wanted_text: &str,
) -> Result<Option<AdoptionObservation>> {
    if !request.adopt_existing {
        return Ok(None);
    }
    let expected_generation =
        require_bind_adoption_u64(request.expected_generation, "expected_generation")?;
    let expected_digest = require_bind_adoption_text(&request.expected_digest, "expected_digest")?;
    let expected_repository =
        require_bind_adoption_text(&request.expected_repository, "expected_repository")?;
    let expected_head = require_bind_adoption_text(&request.expected_head, "expected_head")?;
    let actor = require_bind_adoption_text(&request.actor, "actor")?;
    if !wanted.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption requires a pre-existing Git worktree",
        ));
    }
    let matching_branch = listed
        .iter()
        .filter(|(branch, _)| branch == &request.branch)
        .count();
    let matching_worktree = listed
        .iter()
        .filter(|(_, path)| fs::canonicalize(path).is_ok_and(|candidate| candidate == wanted))
        .count();
    let exact_match = listed.iter().any(|(branch, path)| {
        branch == &request.branch
            && fs::canonicalize(path).is_ok_and(|candidate| candidate == wanted)
    });
    if matching_branch != 1 || matching_worktree != 1 || !exact_match {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption requires exactly one registered worktree for the requested branch and path",
        ));
    }
    if git::current_branch(wanted)? != request.branch {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption worktree is not on the requested branch",
        ));
    }
    let observed_head = git::run(wanted, &["rev-parse", "HEAD"])?.stdout;
    if observed_head != expected_head {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption expected_head does not match the worktree HEAD",
        ));
    }
    git::run(
        wanted,
        &[
            "merge-base",
            "--is-ancestor",
            &request.base_branch,
            &observed_head,
        ],
    )
    .map_err(|_| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption HEAD is not descended from the requested base branch",
        )
    })?;
    if !git::worktree_is_clean(wanted)? {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "adoption requires a clean worktree before lifecycle materialization",
        ));
    }
    Ok(Some(AdoptionObservation {
        actor,
        expected_repository,
        expected_generation,
        expected_digest,
        expected_head,
        observed_head,
        base_branch: request.base_branch.clone(),
        branch: request.branch.clone(),
        worktree: wanted_text.to_owned(),
    }))
}

#[derive(Debug)]
struct TopologyRecordMatch {
    record: crate::IssueRecord,
    same_issue: bool,
    same_stored_branch: bool,
    same_canonical_worktree: bool,
}

fn topology_error(issue: u64, path: &Path, context: &str, error: V2Error) -> V2Error {
    V2Error::new(
        error.code,
        format!(
            "issue {issue} topology {context} at {}: {}",
            path.display(),
            error.message
        ),
    )
}

fn canonical_topology_root(invocation_root: &Path, listed: &[(String, String)]) -> Result<PathBuf> {
    let primary_text = listed.first().map(|(_, path)| path).ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "Git worktree topology has no primary checkout",
        )
    })?;
    let primary = Path::new(primary_text).canonicalize().map_err(|error| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("canonicalize primary Git worktree {primary_text}: {error}"),
        )
    })?;
    if !primary.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!(
                "primary Git worktree is not a directory: {}",
                primary.display()
            ),
        ));
    }
    let invocation_common = PathBuf::from(
        git::run(
            invocation_root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    )
    .canonicalize()
    .map_err(|error| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("canonicalize invocation Git common directory: {error}"),
        )
    })?;
    let primary_common = PathBuf::from(
        git::run(
            &primary,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    )
    .canonicalize()
    .map_err(|error| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            format!("canonicalize primary Git common directory: {error}"),
        )
    })?;
    if invocation_common != primary_common {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "primary and invocation worktrees do not share one Git common directory",
        ));
    }
    Ok(primary)
}

fn unsafe_checkout(context: impl Into<String>) -> V2Error {
    V2Error::new(ErrorCode::UnsafeCheckout, context.into())
}

fn reject_primary_checkout_bootstrap(root: &Path, repository: &str) -> Result<()> {
    crate::operator::verify_installed_owner_preflight(root)?;
    if !requires_worktree_policy(repository) {
        return Ok(());
    }
    let invocation = git::run(
        root,
        &["rev-parse", "--path-format=absolute", "--show-toplevel"],
    )
    .map_err(|error| {
        unsafe_checkout(format!(
            "cannot determine bootstrap checkout top-level: {}",
            error.message
        ))
    })?
    .stdout;
    let invocation = PathBuf::from(invocation).canonicalize().map_err(|error| {
        unsafe_checkout(format!(
            "canonicalize bootstrap checkout top-level: {error}"
        ))
    })?;
    let listed = git::worktrees(root).map_err(|error| {
        unsafe_checkout(format!(
            "cannot determine Git worktree topology before bootstrap: {}",
            error.message
        ))
    })?;
    let primary = canonical_topology_root(root, &listed).map_err(|error| {
        unsafe_checkout(format!(
            "cannot prove non-primary bootstrap checkout: {}",
            error.message
        ))
    })?;
    if invocation == primary {
        return Err(unsafe_checkout(format!(
            "csdlc-issue create must run from an isolated non-primary checkout; primary checkout {} is inspection-only",
            primary.display()
        )));
    }
    Ok(())
}

fn issue_records(
    store: &Store,
    topology_root: &Path,
    requested_issue: u64,
    requested_branch: &str,
    requested_path: &Path,
) -> Result<Vec<TopologyRecordMatch>> {
    let issues = store.root().join(".csdlc/issues");
    if !issues.exists() {
        return Ok(Vec::new());
    }
    let mut records = Vec::new();
    for entry in fs::read_dir(issues)? {
        let entry = entry?;
        let Some(issue) = entry
            .file_name()
            .to_str()
            .and_then(|value| value.parse().ok())
        else {
            continue;
        };
        if entry.path().join("index.json").exists() {
            let index_path = entry.path().join("index.json");
            let bytes = fs::read(&index_path).map_err(|error| {
                topology_error(issue, &index_path, "metadata read", error.into())
            })?;
            let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| {
                topology_error(issue, &index_path, "metadata parse", error.into())
            })?;
            let branch_value = value.get("branch");
            let worktree_value = value.get("worktree");
            if branch_value.is_none_or(serde_json::Value::is_null)
                && worktree_value.is_none_or(serde_json::Value::is_null)
            {
                continue;
            }
            let branch = branch_value.and_then(serde_json::Value::as_str);
            let worktree = worktree_value.and_then(serde_json::Value::as_str);
            let same_issue = issue == requested_issue;
            let same_stored_branch = branch == Some(requested_branch);
            let same_canonical_worktree = worktree
                .map(|value| requested_worktree(topology_root, value))
                .transpose()
                .map_err(|error| {
                    topology_error(issue, &index_path, "worktree normalization", error)
                })?
                .is_some_and(|path| path == requested_path);
            if !(same_issue || same_stored_branch || same_canonical_worktree) {
                continue;
            }
            let record = store
                .load_record_for_topology_scan(issue)
                .map_err(|error| topology_error(issue, &index_path, "record read", error))?;
            let cards_path = store.issue_dir(issue).join("cards");
            let cards = store
                .load_cards(issue)
                .map_err(|error| topology_error(issue, &cards_path, "card read", error))?;
            crate::store::verify_cards(store, &record, &cards).map_err(|error| {
                let artifact_context = store.root().join(&record.design_path);
                topology_error(
                    issue,
                    &artifact_context,
                    &format!(
                        "card/artifact verification (diagram {})",
                        store.root().join(&record.diagram_path).display()
                    ),
                    error,
                )
            })?;
            records.push(TopologyRecordMatch {
                record,
                same_issue,
                same_stored_branch,
                same_canonical_worktree,
            });
        }
    }
    Ok(records)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
    let metadata = fs::symlink_metadata(source)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "issue projection source must be a real directory",
        ));
    }
    if fs::symlink_metadata(destination).is_ok_and(|value| value.file_type().is_symlink()) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "issue projection target cannot be a symlink",
        ));
    }
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&from)?;
        if metadata.file_type().is_symlink() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "issue projection cannot contain symlinks",
            ));
        }
        if metadata.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(from, to)?;
        }
    }
    Ok(())
}

fn copy_authored_file(source: &Store, target: &Store, relative: &str) -> Result<()> {
    let relative = Path::new(relative);
    let to = target.root().join(relative);
    if to.starts_with(target.issue_dir(0).parent().expect("issue root")) {
        return Ok(());
    }
    let bytes = read_regular_authored_artifact(source.root(), relative)?.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!("source authored file is absent: {}", relative.display()),
        )
    })?;
    if let Some(existing) = read_regular_authored_artifact(target.root(), relative)? {
        if existing == bytes {
            return Ok(());
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "bound worktree has different authored file {}",
                relative.display()
            ),
        ));
    }
    require_canonical_parent_beneath(target.root(), relative)?;
    require_regular_or_absent_beneath(target.root(), relative)?;
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    require_canonical_parent_beneath(target.root(), relative)?;
    require_regular_or_absent_beneath(target.root(), relative)?;
    fs::write(to, bytes)?;
    Ok(())
}

struct MaterializedIssue {
    store: Store,
    issue: u64,
    source_is_target: bool,
    created_issue: bool,
    created_design: bool,
    created_diagram: bool,
    design_path: String,
    diagram_path: String,
}

impl MaterializedIssue {
    fn rollback(&self) {
        if self.created_issue {
            let _ = fs::remove_dir_all(self.store.issue_dir(self.issue));
        }
        if self.created_design {
            let _ = fs::remove_file(self.store.root().join(&self.design_path));
        }
        if self.created_diagram {
            let _ = fs::remove_file(self.store.root().join(&self.diagram_path));
        }
    }
}

fn materialize_issue(source: &Store, target_root: &Path, issue: u64) -> Result<MaterializedIssue> {
    let target = Store::new(target_root.to_path_buf());
    let source_record = source.load_record(issue)?;
    let source_is_target = source.root().canonicalize()? == target.root().canonicalize()?;
    if source_is_target {
        return Ok(MaterializedIssue {
            store: target,
            issue,
            source_is_target,
            created_issue: false,
            created_design: false,
            created_diagram: false,
            design_path: source_record.design_path,
            diagram_path: source_record.diagram_path,
        });
    }
    let mut created_issue = false;
    let mut created_design = false;
    let mut created_diagram = false;
    if target.issue_dir(issue).exists() {
        let target_record = target.load_record(issue)?;
        if target_record.issue != source_record.issue
            || target_record.repository != source_record.repository
            || target_record.initialization_digest != source_record.initialization_digest
            || target_record.digest != source_record.digest
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "bound worktree contains different or stale issue truth",
            ));
        }
        crate::store::verify_cards(&target, &target_record, &target.load_cards(issue)?)?;
    } else {
        for relative in [&source_record.design_path, &source_record.diagram_path] {
            let from = source.root().join(relative);
            let to = target.root().join(relative);
            if let Ok(existing) = fs::read(&to) {
                if existing != fs::read(&from)? {
                    return Err(V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        format!("bound worktree has different authored file {relative}"),
                    ));
                }
            }
        }
        created_design = !target.root().join(&source_record.design_path).exists();
        created_diagram = !target.root().join(&source_record.diagram_path).exists();
        let result = (|| {
            copy_tree(&source.issue_dir(issue), &target.issue_dir(issue))?;
            created_issue = true;
            copy_authored_file(source, &target, &source_record.design_path)?;
            copy_authored_file(source, &target, &source_record.diagram_path)?;
            Ok(())
        })();
        if let Err(error) = result {
            if created_issue {
                let _ = fs::remove_dir_all(target.issue_dir(issue));
            }
            if created_design {
                let _ = fs::remove_file(target.root().join(&source_record.design_path));
            }
            if created_diagram {
                let _ = fs::remove_file(target.root().join(&source_record.diagram_path));
            }
            return Err(error);
        }
    }
    Ok(MaterializedIssue {
        store: target,
        issue,
        source_is_target,
        created_issue,
        created_design,
        created_diagram,
        design_path: source_record.design_path,
        diagram_path: source_record.diagram_path,
    })
}

fn reject_bind_unsafe_authored_path(relative: &str) -> Result<()> {
    let path = Path::new(relative);
    if path
        .components()
        .next()
        .is_some_and(|component| component == Component::Normal(".git".as_ref()))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "binding cannot materialize authored design/diagram artifacts from .git; run initialized design-envelope recovery first",
        ));
    }
    Ok(())
}

pub(crate) fn initialize_issue(
    store: &Store,
    mut request: BootstrapRequest,
) -> Result<crate::IssueRecord> {
    if !clean_relative(&request.design_path)
        || !clean_relative(&request.diagram_path)
        || request.design_path == request.diagram_path
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "design and diagram paths must be distinct and repository-relative",
        ));
    }
    validate_bootstrap_request(&request)?;
    validate_initial_input(&request.initial)?;
    reject_primary_checkout_bootstrap(store.root(), &request.repository)?;
    let _creation_lock = store.binding_lock()?;
    let issue_dir = store.issue_dir(request.issue);
    for authored_path in [&request.design_path, &request.diagram_path] {
        let path = store.root().join(authored_path);
        if path == issue_dir.join("index.json")
            || path == issue_dir.join("audit.jsonl")
            || path.starts_with(issue_dir.join("cards"))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "design and diagram paths cannot target issue control files",
            ));
        }
    }
    if issue_dir.join("index.json").exists() {
        return bootstrap_issue(store, request);
    }
    let design = store.root().join(&request.design_path);
    let diagram = store.root().join(&request.diagram_path);
    for relative in [&request.design_path, &request.diagram_path] {
        let relative = Path::new(relative);
        require_canonical_parent_beneath(store.root(), relative)?;
        require_regular_or_absent_beneath(store.root(), relative)?;
    }
    let created_design = !design.exists();
    let created_diagram = !diagram.exists();
    let result = (|| {
        if created_design {
            if let Some(parent) = design.parent() {
                fs::create_dir_all(parent)?;
            }
            require_canonical_parent_beneath(store.root(), Path::new(&request.design_path))?;
            require_regular_or_absent_beneath(store.root(), Path::new(&request.design_path))?;
            fs::write(
                &design,
                format!(
                    "# Issue {} design\n\nStatus: design required before Ready.\n",
                    request.issue
                ),
            )?;
            request.design_approved = false;
        }
        if created_diagram {
            if let Some(parent) = diagram.parent() {
                fs::create_dir_all(parent)?;
            }
            require_canonical_parent_beneath(store.root(), Path::new(&request.diagram_path))?;
            require_regular_or_absent_beneath(store.root(), Path::new(&request.diagram_path))?;
            fs::write(
                &diagram,
                format!(
                    "flowchart LR\n  I[\"Issue {}\"] --> D[\"Design required\"]\n",
                    request.issue
                ),
            )?;
        }
        bootstrap_issue(store, request)
    })();
    if result.is_err() {
        if created_diagram && diagram.exists() {
            fs::remove_file(&diagram)?;
        }
        if created_design && design.exists() {
            fs::remove_file(&design)?;
        }
    }
    result
}

pub fn initialize_native_json(store: &Store, bytes: &[u8]) -> Result<crate::IssueRecord> {
    let value: serde_json::Value = serde_json::from_slice(bytes)?;
    let initial = value
        .get("initial")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "native initial input is missing"))?;
    if !initial.contains_key("operator_constraints") || !initial.contains_key("review_scope") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "native bootstrap requires explicit operator_constraints and review_scope",
        ));
    }
    let request: BootstrapRequest = serde_json::from_value(value)?;
    reject_primary_checkout_bootstrap(store.root(), &request.repository)?;
    crate::registry::validate_native_registry(store.root())?;
    initialize_issue(store, request)
}

pub fn bind_issue(store: &Store, request: BindRequest) -> Result<BindResult> {
    if request.issue == 0
        || request.branch == "main"
        || request.branch == request.base_branch
        || !valid_branch(store.root(), &request.branch)
        || !valid_branch(store.root(), &request.base_branch)
        || request
            .code_repository
            .as_deref()
            .is_some_and(|repository| !valid_repository_identity(repository))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue, distinct safe base/issue branches, and worktree are required",
        ));
    }
    let wanted = requested_worktree(store.root(), &request.worktree)?;
    let wanted_text = wanted.to_string_lossy().into_owned();
    let current_root = store.root().canonicalize()?;
    let current_record = store.load_record(request.issue)?;
    validate_bind_adoption_request(&current_record, &request)?;
    let stored_worktree = current_record
        .worktree
        .as_deref()
        .map(|value| requested_worktree(store.root(), value))
        .transpose()?;
    let issue_local = wanted.exists()
        && wanted.canonicalize().ok().as_ref() == Some(&current_root)
        && existing_bound_issue_local(
            current_record.phase,
            current_record.branch.as_deref(),
            &wanted,
            &current_root,
            &git::current_branch(store.root())?,
            &request.branch,
            stored_worktree.as_deref(),
        );
    let policy_required = requires_worktree_policy(&current_record.repository);
    if !issue_local {
        enforce_worktree_policy(store.root(), &wanted, policy_required)?;
    }

    let _lock = store.binding_lock()?;
    let _issue_lock = store.authority_projection_lock(request.issue)?;
    let current_record = store.load_record(request.issue)?;
    validate_bind_adoption_request(&current_record, &request)?;
    let source_diagnosis = crate::doctor::diagnose_with_code_repository(
        store,
        request.issue,
        request.code_repository.as_deref(),
    );
    let source_phase = source_diagnosis.phase;
    let source_is_bindable = source_diagnosis.status == DoctorStatus::Pass
        && matches!(
            source_phase,
            Some(
                crate::LifecyclePhase::Initialized
                    | crate::LifecyclePhase::Ready
                    | crate::LifecyclePhase::Bound
            )
        )
        && (source_phase != Some(crate::LifecyclePhase::Initialized) || source_diagnosis.ready);
    reject_bind_unsafe_authored_path(&current_record.design_path)?;
    reject_bind_unsafe_authored_path(&current_record.diagram_path)?;
    if !source_is_bindable {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "source issue is not execution-ready for binding",
        ));
    }

    if !issue_local && git::current_branch(store.root())? != request.base_branch {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "binding must start from the declared base branch or the exact issue worktree",
        ));
    }
    let listed = git::worktrees(store.root())?;
    let topology_root = canonical_topology_root(store.root(), &listed)?;
    let adoption = verify_bind_adoption_topology(&request, &listed, &wanted, &wanted_text)?;
    let mut idempotent_match = false;
    for (_, path) in &listed {
        let path = Path::new(path);
        if !path.exists() {
            continue;
        }
        for candidate in issue_records(
            &Store::new(path),
            &topology_root,
            request.issue,
            &request.branch,
            &wanted,
        )? {
            let record = candidate.record;
            if record.branch.is_none() && record.worktree.is_none() {
                continue;
            }
            if candidate.same_issue {
                if candidate.same_stored_branch
                    && candidate.same_canonical_worktree
                    && request.code_repository.as_deref().is_none_or(|requested| {
                        record
                            .code_repository
                            .as_deref()
                            .unwrap_or(&record.repository)
                            .eq_ignore_ascii_case(requested)
                    })
                    && record.phase == crate::LifecyclePhase::Bound
                {
                    idempotent_match = true;
                    continue;
                }
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "issue is already bound to different Git topology",
                ));
            }
            if candidate.same_stored_branch || candidate.same_canonical_worktree {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "requested Git topology is already bound to another issue",
                ));
            }
        }
    }
    if let Some((branch, _)) = listed.iter().find(|(_, path)| path == &wanted_text) {
        if branch != &request.branch {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "requested worktree belongs to a different branch",
            ));
        }
    } else if wanted.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "requested path exists but is not a Git worktree",
        ));
    }
    if let Some((_, path)) = listed.iter().find(|(branch, _)| branch == &request.branch) {
        if path != &wanted_text {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "requested branch belongs to a different worktree",
            ));
        }
    }
    if idempotent_match {
        let live_topology_matches = listed
            .iter()
            .any(|(branch, path)| branch == &request.branch && path == &wanted_text);
        if !live_topology_matches {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "stored bind is not registered at the requested Git topology",
            ));
        }
        return Ok(BindResult {
            created: false,
            branch: request.branch,
            worktree: wanted_text,
            adopted: false,
            evidence_ref: None,
            resulting_digest: None,
        });
    }

    let new_branch = !branch_exists(store.root(), &request.branch);
    let created = !wanted.exists();
    if !request.adopt_existing && !created && !issue_local {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "pre-existing worktree adoption requires explicit adoption authority",
        ));
    }
    if request.adopt_existing && created {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "adoption cannot create a missing worktree",
        ));
    }
    if created {
        if new_branch {
            git::run(
                store.root(),
                &[
                    "worktree",
                    "add",
                    "-b",
                    &request.branch,
                    &wanted_text,
                    &request.base_branch,
                ],
            )?;
        } else {
            git::run(
                store.root(),
                &["worktree", "add", &wanted_text, &request.branch],
            )?;
        }
    }

    let materialized = match materialize_issue(store, &wanted, request.issue) {
        Ok(materialized) => materialized,
        Err(error) => {
            if created {
                let _ = git::run(
                    store.root(),
                    &["worktree", "remove", "--force", &wanted_text],
                );
                if new_branch {
                    let _ = git::run(store.root(), &["branch", "-D", &request.branch]);
                }
            }
            return Err(error);
        }
    };
    let target_lock = wanted.join(format!(".csdlc/locks/{}.lock", request.issue));
    let target_lock_created = !target_lock.exists();
    let commit = (|| {
        let target = &materialized.store;
        let mut record = target.load_record(request.issue)?;
        validate_bind_adoption_request(&record, &request)?;
        let expected_digest = record.digest.clone();
        if record.phase == crate::LifecyclePhase::Initialized {
            record.advance(
                crate::LifecyclePhase::Ready,
                "csdlc-bind".into(),
                "validated issue readiness".into(),
            )?;
        }
        record.branch = Some(request.branch.clone());
        record.worktree = Some(wanted_text.clone());
        record.code_repository = request.code_repository.clone();
        if record.phase == crate::LifecyclePhase::Ready {
            let reason = adoption
                .as_ref()
                .map(|observation| {
                    serde_json::json!({
                        "operation": "adopt_existing_bind",
                        "actor": observation.actor,
                        "issue": request.issue,
                        "repository": record.repository,
                        "expected_repository": observation.expected_repository,
                        "code_repository": request.code_repository,
                        "expected_generation": observation.expected_generation,
                        "expected_digest": observation.expected_digest,
                        "expected_head": observation.expected_head,
                        "observed_head": observation.observed_head,
                        "base_branch": observation.base_branch,
                        "branch": observation.branch,
                        "worktree": observation.worktree,
                        "pre_state": "ready",
                        "result_state": "bound"
                    })
                    .to_string()
                })
                .unwrap_or_else(|| "bound issue branch and worktree".into());
            record.advance(
                crate::LifecyclePhase::Bound,
                adoption
                    .as_ref()
                    .map(|observation| observation.actor.clone())
                    .unwrap_or_else(|| "csdlc-bind".into()),
                reason,
            )?;
        } else if record.phase != crate::LifecyclePhase::Bound {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "issue phase cannot be bound",
            ));
        }
        record.audit.push(AuditEvent {
            sequence: record.audit.len() as u64 + 1,
            generation: record.generation,
            actor: adoption
                .as_ref()
                .map(|observation| observation.actor.clone())
                .unwrap_or_else(|| "csdlc-bind".into()),
            reason: adoption
                .as_ref()
                .map(|observation| {
                    serde_json::json!({
                        "operation": "adopt_existing_bind",
                        "expected_generation": observation.expected_generation,
                        "expected_repository": observation.expected_repository,
                        "expected_digest": observation.expected_digest,
                        "expected_head": observation.expected_head,
                        "observed_head": observation.observed_head,
                        "base_branch": observation.base_branch,
                        "branch": observation.branch,
                        "worktree": observation.worktree
                    })
                    .to_string()
                })
                .unwrap_or_else(|| "record Git branch/worktree topology".into()),
            operation: adoption
                .as_ref()
                .map(|_| "adopt_existing_bind".into())
                .unwrap_or_else(|| "bind".into()),
        });
        record.digest = crate::store::record_digest(&record)?;
        let mut evidence_ref = None;
        let mut resulting_digest = None;
        let mut issue_file = None;
        if let Some(observation) = &adoption {
            let ref_text = format!(".csdlc/issues/{}/adoption.v1.json", request.issue);
            let evidence = serde_json::json!({
                "schema": "csdlc.bind_adoption_evidence.v1",
                "issue": request.issue,
                "repository": record.repository.clone(),
                "expected_repository": observation.expected_repository.clone(),
                "code_repository": record.code_repository.clone(),
                "actor": observation.actor.clone(),
                "base_branch": observation.base_branch.clone(),
                "branch": observation.branch.clone(),
                "worktree": observation.worktree.clone(),
                "expected_head": observation.expected_head.clone(),
                "observed_head": observation.observed_head.clone(),
                "pre_state": "ready",
                "result_state": "bound",
                "expected_generation": observation.expected_generation,
                "resulting_generation": record.generation,
                "expected_digest": observation.expected_digest.clone(),
                "resulting_digest": record.digest.clone()
            });
            let mut bytes = serde_json::to_vec_pretty(&evidence)?;
            bytes.push(b'\n');
            issue_file = Some(("adoption.v1.json".to_owned(), bytes));
            evidence_ref = Some(ref_text);
            resulting_digest = Some(record.digest.clone());
        }
        if let Some((relative_path, bytes)) = issue_file {
            if materialized.source_is_target {
                target.replace_record_locked_with_issue_file(
                    request.issue,
                    &expected_digest,
                    &record,
                    &relative_path,
                    &bytes,
                )?;
            } else {
                target.replace_record_with_issue_file(
                    request.issue,
                    &expected_digest,
                    &record,
                    &relative_path,
                    &bytes,
                )?;
            }
        } else if materialized.source_is_target {
            target.replace_record_locked(request.issue, &expected_digest, &record)?;
        } else {
            target.replace_record(request.issue, &expected_digest, &record)?;
        }
        Ok((evidence_ref, resulting_digest))
    })();

    let (evidence_ref, resulting_digest) = match commit {
        Ok(result) => result,
        Err(error) => {
            if adoption.is_some() {
                let _ = fs::remove_file(
                    materialized
                        .store
                        .root()
                        .join(format!(".csdlc/issues/{}/adoption.v1.json", request.issue)),
                );
            }
            materialized.rollback();
            if target_lock_created {
                let _ = fs::remove_file(target_lock);
            }
            if created {
                let _ = git::run(
                    store.root(),
                    &["worktree", "remove", "--force", &wanted_text],
                );
                if new_branch {
                    let _ = git::run(store.root(), &["branch", "-D", &request.branch]);
                }
            }
            return Err(error);
        }
    };

    Ok(BindResult {
        created,
        branch: request.branch,
        worktree: wanted_text,
        adopted: adoption.is_some(),
        evidence_ref,
        resulting_digest,
    })
}

#[cfg(test)]
mod fastwork_policy_tests {
    use super::{
        canonical_topology_root, enforce_worktree_policy, existing_bound_issue_local,
        reject_primary_checkout_bootstrap, requires_worktree_policy,
    };
    use crate::{ErrorCode, LifecyclePhase};
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    fn git(root: &Path, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(root)
            .args(args)
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn policy_root() -> tempfile::TempDir {
        let root = tempfile::tempdir().expect("policy root");
        fs::create_dir_all(root.path().join(".adl")).expect("policy directory");
        fs::write(
            root.path().join(".adl/worktree-policy.json"),
            r#"{"schema":"adl.worktree_policy.v1","required_parent":"/Volumes/FastWork/adl-worktrees"}"#,
        )
        .expect("policy file");
        root
    }

    #[test]
    fn fastwork_policy_accepts_child_of_required_parent() {
        let root = policy_root();
        enforce_worktree_policy(
            root.path(),
            Path::new("/Volumes/FastWork/adl-worktrees/adl-issue-5911"),
            true,
        )
        .expect("FastWork child should pass");
    }

    #[test]
    fn fastwork_policy_refuses_other_parent() {
        let root = policy_root();
        let error = enforce_worktree_policy(
            root.path(),
            Path::new("/Users/example/git/agent-design-language/.worktrees/adl-issue-5911"),
            true,
        )
        .expect_err("local-disk worktree should fail");
        assert!(error.message.contains("/Volumes/FastWork/adl-worktrees"));
    }

    #[test]
    fn fastwork_policy_fails_closed_when_required_policy_is_missing() {
        let root = tempfile::tempdir().expect("policy root");
        let error = enforce_worktree_policy(
            root.path(),
            Path::new("/Volumes/FastWork/adl-worktrees/adl-issue-5911"),
            true,
        )
        .expect_err("missing mandatory policy should fail");
        assert!(error.message.contains("missing .adl/worktree-policy.json"));
    }

    #[test]
    fn fastwork_policy_is_required_for_github_equivalent_repository_casing() {
        assert!(requires_worktree_policy(
            "Agent-Logic/Agent-Design-Language"
        ));
    }

    #[test]
    fn ready_issue_local_checkout_cannot_bypass_policy() {
        let path = Path::new("/Users/example/adl-5911");
        assert!(!existing_bound_issue_local(
            LifecyclePhase::Ready,
            Some("codex/5911"),
            path,
            path,
            "codex/5911",
            "codex/5911",
            Some(path),
        ));
    }

    #[test]
    fn matching_legacy_bound_topology_is_idempotent() {
        let path = Path::new("/Users/example/adl-5911");
        assert!(existing_bound_issue_local(
            LifecyclePhase::Bound,
            Some("codex/5911"),
            path,
            path,
            "codex/5911",
            "codex/5911",
            Some(path),
        ));
    }

    #[test]
    fn bootstrap_guard_wraps_topology_probe_failure_as_unsafe_checkout() {
        let root = tempfile::tempdir().expect("repo root");
        let error =
            reject_primary_checkout_bootstrap(root.path(), "agent-logic/agent-design-language")
                .expect_err("non-git ADL bootstrap must fail closed");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        assert!(error
            .message
            .contains("cannot determine bootstrap checkout top-level"));
    }

    #[test]
    fn topology_root_requires_a_primary_worktree_entry() {
        let root = tempfile::tempdir().expect("repo root");
        let error =
            canonical_topology_root(root.path(), &[]).expect_err("missing primary should fail");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        assert!(error.message.contains("no primary checkout"));
    }

    #[test]
    fn topology_root_fails_closed_on_common_dir_mismatch() {
        let invocation = tempfile::tempdir().expect("invocation repo");
        let primary = tempfile::tempdir().expect("different repo");
        git(invocation.path(), &["init", "-b", "main"]);
        git(primary.path(), &["init", "-b", "main"]);
        let listed = vec![("main".into(), primary.path().display().to_string())];

        let error = canonical_topology_root(invocation.path(), &listed)
            .expect_err("unrelated common dirs should fail");
        assert_eq!(error.code, ErrorCode::UnsafeCheckout);
        assert!(error
            .message
            .contains("do not share one Git common directory"));
    }
}
