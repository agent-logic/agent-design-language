//! Isolated, explicit conversion owner for copied lifecycle records.

use crate::commands::local::{
    discover_operational_local_context, execute_operational_local_route,
    inspect_local_lifecycle_state, inspect_v3_local_state, required_local_commands,
    LocalPreparationRequest, PlanStatus, PromptRegistry,
};
use crate::lifecycle::LifecycleState;
use crate::storage::semantic::{
    AcceptedIntentPlan, Admission, Binding, CardProjectionArtifact, CardProjectionBundle,
    CardProjectionObservation, CommitOutcome, CopiedRecordConversion, Digest, IssueInputs,
    IssueKey, LocalChange, Observation, PlanStep, Publication, SemanticRoot, Snapshot, Validator,
    SEMANTIC_CARD_KINDS,
};
use crate::storage::DurableTransactionStore;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversionRequest {
    pub schema: String,
    pub repository: String,
    #[serde(default)]
    pub operation_id: String,
    pub git_common: PathBuf,
    pub linked_worktree: PathBuf,
    pub linked_branch: String,
    pub linked_head: String,
    pub registry_path: PathBuf,
    pub authority_bytes_path: PathBuf,
    pub records: Vec<CopiedRecord>,
    #[serde(default)]
    pub fault_injection: Option<FaultInjection>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaultInjection {
    pub point: String,
    pub boundary: String,
    pub mode: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CopiedRecord {
    pub role: String,
    pub issue: u64,
    pub source: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConvertedRecord {
    pub role: String,
    pub issue: u64,
    pub generation: u64,
    pub digest: String,
    pub projection_digest: String,
    pub disposition: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CurrentObservationRelocationRequest {
    pub schema: String,
    pub operation_id: String,
    pub repository: String,
    pub issue: u64,
    pub source_local_issue: PathBuf,
    pub source_semantic_current: PathBuf,
    pub source_projection_state: PathBuf,
    pub target_repository_root: PathBuf,
    pub target_git_common: PathBuf,
    pub target_worktree: PathBuf,
    pub target_branch: String,
    pub target_head: String,
    pub target_worktree_role: String,
    pub registry_path: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub struct CurrentObservationRelocationResult {
    pub schema: String,
    pub status: String,
    pub operation_id: String,
    pub issue: u64,
    pub generation: u64,
    pub digest: String,
    pub source_hashes: BTreeMap<String, String>,
    pub relocated_hashes: BTreeMap<String, String>,
    pub journal_path: PathBuf,
    pub provenance_path: PathBuf,
}

const FAULT_POINTS: [&str; 16] = [
    "operation_journal_creation",
    "conversion_intent_durability",
    "per_issue_staging_write",
    "whole_census_staging_complete",
    "semantic_state_activation",
    "per_issue_conversion_receipt_persistence",
    "projection_data_completion",
    "projection_publication",
    "candidate_executable_activation",
    "fake_remote_request_dispatch",
    "fake_remote_success_readback",
    "local_reconciled_success_persistence",
    "restore_intent_durability",
    "source_record_restoration",
    "prior_executable_restoration",
    "restore_receipt_persistence_and_fence_release",
];

struct Operation<'a> {
    request: &'a ConversionRequest,
    operation_id: String,
    root: PathBuf,
    journal: PathBuf,
}

impl<'a> Operation<'a> {
    fn open(
        request: &'a ConversionRequest,
        request_digest: &Digest,
        canonical_git_common: &Path,
    ) -> Result<Self, String> {
        let operation_id = if request.operation_id.is_empty() {
            let repository = request
                .repository
                .bytes()
                .map(|byte| {
                    if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.') {
                        byte as char
                    } else {
                        '-'
                    }
                })
                .collect::<String>();
            let first_issue = request.records.first().map_or(0, |record| record.issue);
            format!("compat-{repository}-{first_issue}")
        } else {
            request.operation_id.clone()
        };
        if !operation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(
                "operation_id must contain only ASCII letters, digits, '-', '_' or '.'".to_owned(),
            );
        }
        if let Some(fault) = &request.fault_injection {
            if !FAULT_POINTS.contains(&fault.point.as_str()) {
                return Err(format!("unsupported fault point {}", fault.point));
            }
            if !matches!(fault.boundary.as_str(), "before" | "after") {
                return Err(format!("unsupported fault boundary {}", fault.boundary));
            }
            if fault.mode != "once" {
                return Err(format!("unsupported fault mode {}", fault.mode));
            }
        }
        let csdlc = create_durable_child(canonical_git_common, "csdlc-v3")?;
        let local = create_durable_child(&csdlc, "local")?;
        let rehearsals = create_durable_child(&local, "conversion-rehearsals")?;
        let root = create_durable_child(&rehearsals, &operation_id)?;
        let journal = root.join("journal.jsonl");
        let operation = Self {
            request,
            operation_id,
            root,
            journal,
        };
        if operation.journal.is_file() {
            let bytes = fs::read(&operation.journal)
                .map_err(|error| format!("{}: {error}", operation.journal.display()))?;
            let retained = bytes
                .split(|byte| *byte == b'\n')
                .filter(|line| !line.is_empty())
                .find_map(|line| serde_json::from_slice::<Value>(line).ok())
                .filter(|event| {
                    event.get("event").and_then(Value::as_str) == Some("operation_journal_created")
                })
                .and_then(|event| {
                    event
                        .pointer("/detail/request_digest")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                });
            if retained.as_deref() != Some(request_digest.as_str()) {
                return Err(format!(
                    "operation identity mismatch for {}: retained request digest does not match",
                    operation.id()
                ));
            }
        } else {
            operation.append(
                "operation_journal_created",
                json!({"request_digest": request_digest.as_str()}),
            )?;
        }
        operation.fault("operation_journal_creation", "after")?;
        Ok(operation)
    }

    fn id(&self) -> &str {
        &self.operation_id
    }

    fn append(&self, event: &str, detail: Value) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.journal)
            .map_err(|error| format!("{}: {error}", self.journal.display()))?;
        let line = json!({
            "schema": "csdlc.v3.copied_record_conversion_journal_event.v1",
            "operation_id": self.operation_id,
            "event": event,
            "detail": detail,
        });
        serde_json::to_writer(&mut file, &line).map_err(|error| error.to_string())?;
        file.write_all(b"\n").map_err(|error| error.to_string())?;
        file.sync_all().map_err(|error| error.to_string())?;
        sync_dir(self.journal.parent().expect("operation journal has parent"))?;
        Ok(())
    }

    fn has_event(&self, event: &str) -> Result<bool, String> {
        let bytes = fs::read(&self.journal)
            .map_err(|error| format!("{}: {error}", self.journal.display()))?;
        for line in bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            let value: Value = serde_json::from_slice(line)
                .map_err(|error| format!("{}: {error}", self.journal.display()))?;
            if value.get("event").and_then(Value::as_str) == Some(event) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn fault(&self, point: &str, boundary: &str) -> Result<(), String> {
        let Some(fault) = &self.request.fault_injection else {
            return Ok(());
        };
        if fault.point != point || fault.boundary != boundary {
            return Ok(());
        }
        let event = format!("fault_injected:{point}:{boundary}");
        if self.has_event(&event)? {
            return Ok(());
        }
        self.append(
            &event,
            json!({"point": point, "boundary": boundary, "mode": "once"}),
        )?;
        std::process::abort()
    }

    fn marker(&self, relative: &str, event: &str, detail: Value) -> Result<(), String> {
        let path = self.root.join(relative);
        let existed = path.is_file();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        let bytes = serde_json::to_vec_pretty(&json!({
            "schema": "csdlc.v3.copied_record_conversion_checkpoint.v1",
            "operation_id": self.operation_id,
            "event": event,
            "detail": detail,
        }))
        .map_err(|error| error.to_string())?;
        write_create_once(&path, &bytes)?;
        if existed {
            Ok(())
        } else {
            self.append(event, json!({"path": path}))
        }
    }
}

fn sync_dir(path: &Path) -> Result<(), String> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// Create one directory entry and request that the parent persist that entry before
/// returning. Repeating this one component at a time closes the first-use ancestry
/// gap left by `create_dir_all`. Unit tests can prove layout and restart behavior;
/// physical power-loss persistence still requires a filesystem crash harness.
fn create_durable_child(parent: &Path, name: &str) -> Result<PathBuf, String> {
    let child = parent.join(name);
    match fs::create_dir(&child) {
        Ok(()) => sync_dir(parent)?,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let metadata = fs::symlink_metadata(&child)
                .map_err(|metadata_error| format!("{}: {metadata_error}", child.display()))?;
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err(format!("{} is not a safe directory", child.display()));
            }
        }
        Err(error) => return Err(format!("{}: {error}", child.display())),
    }
    Ok(child)
}

fn write_create_once(path: &Path, bytes: &[u8]) -> Result<(), String> {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => {
            file.write_all(bytes)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            file.sync_all()
                .map_err(|error| format!("{}: {error}", path.display()))?;
            sync_dir(path.parent().expect("created file has parent"))
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing =
                fs::read(path).map_err(|read_error| format!("{}: {read_error}", path.display()))?;
            if existing == bytes {
                Ok(())
            } else {
                Err(format!(
                    "{} already exists with different bytes",
                    path.display()
                ))
            }
        }
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

fn replace_durable(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if path.is_file()
        && fs::read(path).map_err(|error| format!("{}: {error}", path.display()))? == bytes
    {
        return Ok(());
    }
    let parent = path.parent().expect("replacement target has parent");
    fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    let temporary = parent.join(format!(
        ".{}.next",
        path.file_name()
            .expect("replacement target has file name")
            .to_string_lossy()
    ));
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temporary)
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        file.write_all(bytes)
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        file.sync_all()
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
    }
    fs::rename(&temporary, path).map_err(|error| {
        format!(
            "rename {} to {}: {error}",
            temporary.display(),
            path.display()
        )
    })?;
    sync_dir(parent)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("{}: {error}", destination.display()))?;
    let mut entries = fs::read_dir(source)
        .map_err(|error| format!("{}: {error}", source.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        if kind.is_dir() {
            copy_tree(&source_path, &destination_path)?;
        } else if kind.is_file() {
            if destination_path.is_file() {
                if fs::read(&source_path).map_err(|error| error.to_string())?
                    != fs::read(&destination_path).map_err(|error| error.to_string())?
                {
                    return Err(format!(
                        "staged file {} differs from source",
                        destination_path.display()
                    ));
                }
            } else {
                fs::copy(&source_path, &destination_path)
                    .map_err(|error| format!("{}: {error}", destination_path.display()))?;
                fs::File::open(&destination_path)
                    .and_then(|file| file.sync_all())
                    .map_err(|error| format!("{}: {error}", destination_path.display()))?;
            }
        } else {
            return Err(format!(
                "unsupported source entry {}",
                source_path.display()
            ));
        }
    }
    sync_dir(destination)
}

struct ConversionPreflight {
    canonical_git_common: PathBuf,
    canonical_linked_worktree: PathBuf,
    source_digests: Vec<Digest>,
    authority_bytes: Vec<u8>,
    registry_bytes: Vec<u8>,
}

fn git_output(worktree: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(worktree)
        .args(args)
        .output()
        .map_err(|error| format!("git {:?}: {error}", args))?;
    if !output.status.success() {
        return Err(format!(
            "git {:?} failed for {}: {}",
            args,
            worktree.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|error| format!("git {:?} returned non-UTF-8 output: {error}", args))
}

fn authenticate_linked_worktree(request: &ConversionRequest) -> Result<(PathBuf, PathBuf), String> {
    if !request.linked_worktree.is_absolute()
        || request
            .linked_worktree
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err("linked_worktree must be absolute without traversal components".to_owned());
    }
    let canonical_git_common = fs::canonicalize(&request.git_common)
        .map_err(|error| format!("{}: {error}", request.git_common.display()))?;
    let canonical_linked_worktree = fs::canonicalize(&request.linked_worktree)
        .map_err(|error| format!("{}: {error}", request.linked_worktree.display()))?;
    let observed_root = fs::canonicalize(git_output(
        &canonical_linked_worktree,
        &["rev-parse", "--show-toplevel"],
    )?)
    .map_err(|error| format!("linked worktree root: {error}"))?;
    if observed_root != canonical_linked_worktree {
        return Err("linked_worktree does not name the exact Git worktree root".to_owned());
    }
    let observed_common = fs::canonicalize(git_output(
        &canonical_linked_worktree,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?)
    .map_err(|error| format!("linked worktree common directory: {error}"))?;
    if observed_common != canonical_git_common {
        return Err("linked_worktree belongs to a different Git common directory".to_owned());
    }
    let observed_branch = git_output(
        &canonical_linked_worktree,
        &["symbolic-ref", "--quiet", "--short", "HEAD"],
    )?;
    if observed_branch != request.linked_branch {
        return Err(format!(
            "linked_worktree branch mismatch: expected {}, observed {}",
            request.linked_branch, observed_branch
        ));
    }
    let observed_head = git_output(&canonical_linked_worktree, &["rev-parse", "HEAD"])?;
    if observed_head != request.linked_head {
        return Err(format!(
            "linked_worktree HEAD mismatch: expected {}, observed {}",
            request.linked_head, observed_head
        ));
    }

    let listing = git_output(
        &canonical_linked_worktree,
        &["worktree", "list", "--porcelain"],
    )?;
    let mut registered = Vec::new();
    for block in listing.split("\n\n") {
        let mut path = None;
        let mut head = None;
        let mut branch = None;
        for line in block.lines() {
            if let Some(value) = line.strip_prefix("worktree ") {
                path = fs::canonicalize(value).ok();
            } else if let Some(value) = line.strip_prefix("HEAD ") {
                head = Some(value.to_owned());
            } else if let Some(value) = line.strip_prefix("branch ") {
                branch = Some(value.to_owned());
            }
        }
        if let (Some(path), Some(head), Some(branch)) = (path, head, branch) {
            registered.push((path, head, branch));
        }
    }
    let expected_ref = format!("refs/heads/{}", request.linked_branch);
    let Some(position) = registered.iter().position(|(path, head, branch)| {
        path == &canonical_linked_worktree
            && head == &request.linked_head
            && branch == &expected_ref
    }) else {
        return Err("linked_worktree is not registered with the exact branch and HEAD".to_owned());
    };
    if position == 0 {
        return Err("linked_worktree must be a registered non-primary worktree".to_owned());
    }
    Ok((canonical_git_common, canonical_linked_worktree))
}

fn authenticate_relocation_target(
    request: &CurrentObservationRelocationRequest,
) -> Result<(PathBuf, PathBuf, PathBuf), String> {
    if !matches!(request.target_worktree_role.as_str(), "primary" | "linked") {
        return Err("target_worktree_role must be primary or linked".to_owned());
    }
    for (name, path) in [
        ("target_repository_root", &request.target_repository_root),
        ("target_git_common", &request.target_git_common),
        ("target_worktree", &request.target_worktree),
    ] {
        if !path.is_absolute()
            || path
                .components()
                .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
        {
            return Err(format!(
                "{name} must be absolute without traversal components"
            ));
        }
    }
    let worktree = fs::canonicalize(&request.target_worktree)
        .map_err(|error| format!("{}: {error}", request.target_worktree.display()))?;
    let repository_root = fs::canonicalize(&request.target_repository_root)
        .map_err(|error| format!("{}: {error}", request.target_repository_root.display()))?;
    if repository_root != worktree {
        return Err("target_repository_root must be the exact target worktree".to_owned());
    }
    let git_common = fs::canonicalize(&request.target_git_common)
        .map_err(|error| format!("{}: {error}", request.target_git_common.display()))?;
    let observed_common = fs::canonicalize(git_output(
        &worktree,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?)
    .map_err(|error| format!("target Git common directory: {error}"))?;
    if observed_common != git_common {
        return Err("target worktree belongs to a different Git common directory".to_owned());
    }
    if git_output(&worktree, &["rev-parse", "--show-toplevel"])? != worktree.to_string_lossy()
        || git_output(&worktree, &["symbolic-ref", "--quiet", "--short", "HEAD"])?
            != request.target_branch
        || git_output(&worktree, &["rev-parse", "HEAD"])? != request.target_head
    {
        return Err("target worktree branch, root, or HEAD mismatch".to_owned());
    }
    let listing = git_output(&worktree, &["worktree", "list", "--porcelain"])?;
    let mut registrations = Vec::new();
    for block in listing.split("\n\n") {
        let path = block
            .lines()
            .find_map(|line| line.strip_prefix("worktree "))
            .and_then(|path| fs::canonicalize(path).ok());
        let head = block.lines().find_map(|line| line.strip_prefix("HEAD "));
        let branch = block.lines().find_map(|line| line.strip_prefix("branch "));
        if let (Some(path), Some(head), Some(branch)) = (path, head, branch) {
            registrations.push((path, head.to_owned(), branch.to_owned()));
        }
    }
    let expected_branch = format!("refs/heads/{}", request.target_branch);
    let position = registrations
        .iter()
        .position(|(path, head, branch)| {
            path == &worktree && head == &request.target_head && branch == &expected_branch
        })
        .ok_or_else(|| "target worktree is not registered with exact branch and HEAD".to_owned())?;
    if (request.target_worktree_role == "primary") != (position == 0) {
        return Err("target worktree role does not match Git registration".to_owned());
    }
    let primary = registrations
        .first()
        .map(|registration| registration.0.clone())
        .ok_or_else(|| "target Git topology has no primary registration".to_owned())?;
    Ok((git_common, worktree, primary))
}

fn preflight_conversion(request: &ConversionRequest) -> Result<ConversionPreflight, String> {
    let (canonical_git_common, canonical_linked_worktree) = authenticate_linked_worktree(request)?;
    let authority_bytes = fs::read(&request.authority_bytes_path)
        .map_err(|error| format!("{}: {error}", request.authority_bytes_path.display()))?;
    let authority = Digest::authority(&authority_bytes);
    let registry_bytes = fs::read(&request.registry_path)
        .map_err(|error| format!("{}: {error}", request.registry_path.display()))?;
    PromptRegistry::from_current_json(&registry_bytes)
        .map_err(|findings| format!("invalid prompt registry: {findings:?}"))?;
    SemanticRoot::from_git_common(&canonical_git_common, request.repository.clone())
        .map_err(|error| format!("invalid semantic root: {error:?}"))?;
    let mut source_digests = Vec::with_capacity(request.records.len());
    let mut issues = BTreeSet::new();
    for record in &request.records {
        if !issues.insert(record.issue) {
            return Err(format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: duplicate issue identity",
                record.issue
            ));
        }
        let digest = source_digest(&record.source).map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: {error}",
                record.issue
            )
        })?;
        let index = read_json(&record.source.join("index.json")).map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: {error}",
                record.issue
            )
        })?;
        let cards = load_cards(&record.source).map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: {error}",
                record.issue
            )
        })?;
        source_phase(&record.role, &index).map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: {error}",
                record.issue
            )
        })?;
        let generation = index
            .get("generation")
            .and_then(Value::as_u64)
            .filter(|generation| *generation > 0)
            .ok_or_else(|| {
                format!(
                    "issue {} disposition=unsupported_ambiguous_incomplete_source: invalid numeric generation",
                    record.issue
                )
            })?;
        IssueKey::new(request.repository.clone(), record.issue).map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: {error:?}",
                record.issue
            )
        })?;
        conversion_inputs(
            request,
            record,
            &index,
            cards,
            &canonical_linked_worktree,
            authority.clone(),
        )
        .map_err(|error| {
            format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: activation prerequisite failed at generation {generation}: {error}",
                record.issue
            )
        })?;
        source_digests.push(digest);
    }
    Ok(ConversionPreflight {
        canonical_git_common,
        canonical_linked_worktree,
        source_digests,
        authority_bytes,
        registry_bytes,
    })
}

fn canonical_request_digest(
    request: &ConversionRequest,
    preflight: &ConversionPreflight,
) -> Result<Digest, String> {
    let records = request
        .records
        .iter()
        .zip(&preflight.source_digests)
        .map(|(record, digest)| {
            json!({
                "issue": record.issue,
                "role": record.role,
                "source_digest": digest.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let identity = json!({
        "schema": "csdlc.v3.copied_record_conversion_request_identity.v1",
        "repository": request.repository,
        "records": records,
        "binding": {
            "git_common": preflight.canonical_git_common,
            "linked_worktree": preflight.canonical_linked_worktree,
            "linked_branch": request.linked_branch,
            "linked_head": request.linked_head,
        },
        "authority_digest": Digest::authority(&preflight.authority_bytes).as_str(),
        "registry_digest": Digest::projection(&preflight.registry_bytes).as_str(),
    });
    let bytes = serde_json::to_vec(&identity).map_err(|error| error.to_string())?;
    Ok(Digest::semantic_projection(&bytes))
}

fn read_json(path: &Path) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn load_cards(source: &Path) -> Result<BTreeMap<String, Value>, String> {
    let mut cards = BTreeMap::new();
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        let values = source.join("cards").join(format!("{kind}.values.json"));
        let rendered = source.join("cards").join(format!("{kind}.md"));
        if !rendered.is_file() {
            return Err(format!("missing rendered card {}", rendered.display()));
        }
        cards.insert(kind.to_owned(), read_json(&values)?);
    }
    Ok(cards)
}

fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, String> {
    fn ordered(value: &Value) -> Value {
        match value {
            Value::Object(object) => {
                let mut keys = object.keys().collect::<Vec<_>>();
                keys.sort();
                let mut result = serde_json::Map::new();
                for key in keys {
                    result.insert(key.clone(), ordered(&object[key]));
                }
                Value::Object(result)
            }
            Value::Array(values) => values.iter().map(ordered).collect::<Vec<_>>().into(),
            value => value.clone(),
        }
    }
    serde_json::to_vec(&ordered(value)).map_err(|error| error.to_string())
}

fn copied_card_projection(
    snapshot: &Snapshot,
    registry: &PromptRegistry,
    source: &Path,
) -> Result<CardProjectionBundle, String> {
    let mut artifacts = BTreeMap::new();
    for kind in SEMANTIC_CARD_KINDS {
        if !registry.card_kinds.contains(kind) {
            return Err(format!("active registry is missing {kind}"));
        }
        let template_ref = registry
            .template_paths
            .get(kind)
            .ok_or_else(|| format!("active registry is missing the {kind} template path"))?;
        let values = canonical_json_bytes(
            snapshot
                .inputs()
                .cards()
                .get(kind)
                .ok_or_else(|| format!("accepted semantic input is missing {kind} values"))?,
        )?;
        let rendered_path = source.join("cards").join(format!("{kind}.md"));
        let rendered = fs::read(&rendered_path)
            .map_err(|error| format!("{}: {error}", rendered_path.display()))?;
        artifacts.insert(
            kind.to_owned(),
            CardProjectionArtifact::new(kind, template_ref.clone(), values, rendered)
                .map_err(|error| format!("{error:?}"))?,
        );
    }
    CardProjectionBundle::new(snapshot, registry.version.clone(), artifacts)
        .map_err(|error| format!("{error:?}"))
}

fn snapshot(outcome: CommitOutcome) -> Box<Snapshot> {
    match outcome {
        CommitOutcome::Committed(snapshot) | CommitOutcome::Unchanged(snapshot) => snapshot,
    }
}

fn source_digest(root: &Path) -> Result<Digest, String> {
    fn collect(root: &Path, current: &Path, bytes: &mut Vec<u8>) -> Result<(), String> {
        let mut entries = fs::read_dir(current)
            .map_err(|error| format!("{}: {error}", current.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("{}: {error}", current.display()))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() {
                return Err(format!("source census contains symlink {}", path.display()));
            }
            if metadata.is_dir() {
                collect(root, &path, bytes)?;
            } else if metadata.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_string_lossy();
                bytes.extend_from_slice(relative.as_bytes());
                bytes.push(0);
                let content =
                    fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
                bytes.extend_from_slice(content.len().to_string().as_bytes());
                bytes.push(0);
                bytes.extend_from_slice(&content);
                bytes.push(0);
            }
        }
        Ok(())
    }
    let mut bytes = Vec::new();
    collect(root, root, &mut bytes)?;
    Ok(Digest::semantic_projection(&bytes))
}

fn source_phase(role: &str, index: &Value) -> Result<LifecycleState, String> {
    let declared = index.get("phase").and_then(Value::as_str).unwrap_or("");
    match (role, declared) {
        ("prepared", "ready" | "prepared") => Ok(LifecycleState::Ready),
        ("bound_dirty" | "pending_recovery", _) => Ok(LifecycleState::Bound),
        ("implemented", _) => Ok(LifecycleState::Implemented),
        ("reviewed", _) => Ok(LifecycleState::Reviewed),
        ("published", _) => Ok(LifecycleState::Published),
        ("terminal", _) => Ok(LifecycleState::ClosedOut),
        _ => Err(format!("unsupported source role/phase {role}/{declared}")),
    }
}

fn conversion_inputs(
    request: &ConversionRequest,
    record: &CopiedRecord,
    index: &Value,
    cards: BTreeMap<String, Value>,
    canonical_linked_worktree: &Path,
    authority: Digest,
) -> Result<IssueInputs, String> {
    let slug = index
        .get("slug")
        .and_then(Value::as_str)
        .unwrap_or(record.role.as_str())
        .replace('_', "-");
    let publication = Publication {
        base: index
            .get("base_branch")
            .and_then(Value::as_str)
            .unwrap_or("main")
            .to_owned(),
        title: index
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or(record.role.as_str())
            .to_owned(),
        body: format!("Copied conversion rehearsal for issue {}", record.issue),
        draft: true,
    };
    let accepted = AcceptedIntentPlan {
        schema: "csdlc.v3.intent_plan.v1".to_owned(),
        slug,
        cards,
        validators: vec![Validator {
            id: "conversion-equivalence".to_owned(),
            program: "/usr/bin/true".to_owned(),
            args: Vec::new(),
            success_marker: String::new(),
            timeout_seconds: 30,
        }],
        publication,
    };
    let binding = if record.role == "prepared" {
        None
    } else {
        Some(Binding {
            branch: request.linked_branch.clone(),
            head: request.linked_head.clone(),
            worktree: canonical_linked_worktree.to_owned(),
            registration: "git-worktree-list".to_owned(),
        })
    };
    IssueInputs::new(
        format!("copied-record-conversion:{}", record.role),
        accepted,
        vec![PlanStep {
            id: "convert".to_owned(),
            acceptance: "lossless copied-record mapping".to_owned(),
        }],
        binding,
        authority,
    )
    .map_err(|error| format!("{error:?}"))
}

pub fn convert(request: &ConversionRequest) -> Result<Vec<ConvertedRecord>, String> {
    if request.schema != "csdlc.v3.copied_record_conversion.v1" {
        return Err("unsupported conversion request schema".to_owned());
    }
    let roles: Vec<&str> = request
        .records
        .iter()
        .map(|record| record.role.as_str())
        .collect();
    if roles
        != [
            "prepared",
            "bound_dirty",
            "implemented",
            "reviewed",
            "published",
            "terminal",
            "pending_recovery",
        ]
    {
        return Err("conversion requires the exact ordered seven-role census".to_owned());
    }

    let preflight = preflight_conversion(request)?;
    let request_digest = canonical_request_digest(request, &preflight)?;
    let operation = Operation::open(request, &request_digest, &preflight.canonical_git_common)?;
    operation.fault("conversion_intent_durability", "before")?;
    operation.marker(
        "checkpoints/conversion-intent.json",
        "conversion_intent_durable",
        json!({"repository": request.repository, "record_count": request.records.len()}),
    )?;
    let fence = operation.root.join("conversion.fence");
    if !fence.is_file() {
        write_create_once(&fence, b"active\n")?;
    }
    operation.fault("conversion_intent_durability", "after")?;

    let staging = operation.root.join("staging");
    for (record_index, record) in request.records.iter().enumerate() {
        operation.fault("per_issue_staging_write", "before")?;
        let destination = staging.join(record.issue.to_string());
        copy_tree(&record.source, &destination)?;
        let staged_digest = source_digest(&destination)?;
        if staged_digest != preflight.source_digests[record_index] {
            return Err(format!(
                "issue {} source changed after request identity was retained",
                record.issue
            ));
        }
        operation.marker(
            &format!("checkpoints/staged-{}.json", record.issue),
            "per_issue_staging_write_completed",
            json!({"issue": record.issue, "source_digest": staged_digest.as_str()}),
        )?;
        operation.fault("per_issue_staging_write", "after")?;
    }
    operation.fault("whole_census_staging_complete", "before")?;
    operation.marker(
        "checkpoints/whole-census-staging-complete.json",
        "whole_census_staging_completed",
        json!({"record_count": request.records.len()}),
    )?;
    operation.fault("whole_census_staging_complete", "after")?;

    let (activation_git_common, activation_linked_worktree) =
        authenticate_linked_worktree(request)?;
    if activation_git_common != preflight.canonical_git_common
        || activation_linked_worktree != preflight.canonical_linked_worktree
    {
        return Err("linked_worktree identity changed after conversion preflight".to_owned());
    }

    let authority_bytes = preflight.authority_bytes;
    let authority = Digest::authority(&authority_bytes);
    let registry_bytes = preflight.registry_bytes;
    let registry = PromptRegistry::from_current_json(&registry_bytes)
        .map_err(|findings| format!("invalid prompt registry: {findings:?}"))?;
    let canonical_git_common = preflight.canonical_git_common;
    let canonical_linked_worktree = preflight.canonical_linked_worktree;
    let root = SemanticRoot::from_git_common(&canonical_git_common, request.repository.clone())
        .map_err(|error| format!("{error:?}"))?;
    let mut converted = Vec::new();
    for record in &request.records {
        let staged_source = staging.join(record.issue.to_string());
        let index = read_json(&staged_source.join("index.json"))?;
        let cards = load_cards(&staged_source)?;
        let inputs = conversion_inputs(
            request,
            record,
            &index,
            cards,
            &canonical_linked_worktree,
            authority.clone(),
        )?;
        let source_generation = index
            .get("generation")
            .and_then(Value::as_u64)
            .filter(|generation| *generation > 0)
            .ok_or_else(|| format!("{} lacks a valid generation", record.source.display()))?;
        let key = IssueKey::new(request.repository.clone(), record.issue)
            .map_err(|error| format!("{error:?}"))?;
        let phase = source_phase(&record.role, &index)?;
        operation.fault("semantic_state_activation", "before")?;
        let mut current = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(|error| format!("observe {} before activation: {error:?}", record.role))?
        {
            Observation::Current(current) | Observation::ProjectionRepairRequired(current) => {
                if current.inputs() != &inputs || current.phase() != phase {
                    return Err(format!(
                        "existing semantic state for {} does not match the retained conversion operation",
                        record.role
                    ));
                }
                current
            }
            Observation::Absent => snapshot(
                DurableTransactionStore::convert_copied_issue(
                    &root,
                    CopiedRecordConversion {
                        key: key.clone(),
                        inputs,
                        phase,
                        source_generation,
                        source_digest: source_digest(&staged_source)?,
                    },
                )
                .map_err(|error| format!("activate {}: {error:?}", record.role))?,
            ),
            Observation::RecoveryRequired => {
                return Err(format!(
                    "semantic activation for {} requires guarded recovery",
                    record.role
                ));
            }
            Observation::LegacyMigrationRequired => {
                return Err(format!(
                    "semantic activation for {} encountered legacy state",
                    record.role
                ));
            }
        };
        let activation_checkpoint = operation.root.join(format!(
            "checkpoints/semantic-activation-{}.json",
            record.issue
        ));
        let activation_digest = if activation_checkpoint.is_file() {
            let retained = read_json(&activation_checkpoint)?;
            if retained.get("operation_id").and_then(Value::as_str) != Some(operation.id())
                || retained.pointer("/detail/issue").and_then(Value::as_u64) != Some(record.issue)
            {
                return Err(format!(
                    "retained semantic activation for {} does not match the operation",
                    record.role
                ));
            }
            retained
                .pointer("/detail/digest")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    format!(
                        "retained semantic activation for {} lacks its digest",
                        record.role
                    )
                })?
                .to_owned()
        } else {
            let digest = current.version().digest().as_str().to_owned();
            operation.marker(
                &format!("checkpoints/semantic-activation-{}.json", record.issue),
                "semantic_state_activated",
                json!({
                    "issue": record.issue,
                    "generation": current.version().generation(),
                    "digest": digest,
                }),
            )?;
            digest
        };
        operation.fault("semantic_state_activation", "after")?;

        operation.fault("per_issue_conversion_receipt_persistence", "before")?;
        operation.marker(
            &format!("receipts/{}.json", record.issue),
            "per_issue_conversion_receipt_persisted",
            json!({
                "issue": record.issue,
                "source_digest": source_digest(&staged_source)?.as_str(),
                "semantic_digest": activation_digest,
            }),
        )?;
        operation.fault("per_issue_conversion_receipt_persistence", "after")?;

        operation.fault("projection_data_completion", "before")?;
        DurableTransactionStore::write_issue_projection(&root, &current)
            .map_err(|error| format!("issue projection {}: {error:?}", record.role))?;
        let projection_digest = Digest::projection(
            &current
                .projection_bytes()
                .map_err(|error| format!("{error:?}"))?,
        );
        operation.marker(
            &format!("checkpoints/projection-data-{}.json", record.issue),
            "projection_data_completed",
            json!({"issue": record.issue, "projection_digest": projection_digest.as_str()}),
        )?;
        operation.fault("projection_data_completion", "after")?;

        let cards = copied_card_projection(&current, &registry, &staged_source)
            .map_err(|error| format!("card derivation {}: {error}", record.role))?;
        let expected_card_projection = cards.projection_digest().clone();
        operation.fault("projection_publication", "before")?;
        let rehearsal_projection = canonical_git_common
            .join("csdlc-v3/local/projections")
            .join(record.issue.to_string());
        let projection_checkpoint = operation.root.join(format!(
            "checkpoints/projection-published-{}.json",
            record.issue
        ));
        let retained_projection = projection_checkpoint.is_file();
        if retained_projection {
            let checkpoint = read_json(&projection_checkpoint)?;
            let manifest = read_json(&rehearsal_projection.join("cards/manifest.json"))?;
            if checkpoint.get("operation_id").and_then(Value::as_str) != Some(operation.id())
                || checkpoint.pointer("/detail/issue").and_then(Value::as_u64) != Some(record.issue)
                || !rehearsal_projection.join("state.json").is_file()
                || manifest.get("schema").and_then(Value::as_str)
                    != Some("csdlc.v3.semantic_card_projection_manifest.v1")
            {
                return Err(format!(
                    "retained projection checkpoint for {} does not authenticate a complete operation output",
                    record.role
                ));
            }
        }
        let projection_healthy = current.acknowledged_card_projection()
            == Some(&expected_card_projection)
            && !current.projection_required()
            && DurableTransactionStore::observe_card_projection(&root, &current, &cards)
                .is_ok_and(|observation| observation == CardProjectionObservation::Healthy);
        if !projection_healthy {
            let proof = DurableTransactionStore::write_card_projection(&root, &current, cards)
                .map_err(|error| format!("card projection {}: {error:?}", record.role))?;
            current = snapshot(
                DurableTransactionStore::commit_issue_local(
                    &root,
                    Admission::new(key.clone(), current.version().clone(), authority.clone()),
                    LocalChange::AcknowledgeProjection(proof),
                )
                .map_err(|error| {
                    format!("projection acknowledgement {}: {error:?}", record.role)
                })?,
            );
            let acknowledged_bundle = copied_card_projection(&current, &registry, &staged_source)
                .map_err(|error| {
                format!("acknowledged card derivation {}: {error}", record.role)
            })?;
            let acknowledged_observation = DurableTransactionStore::observe_card_projection(
                &root,
                &current,
                &acknowledged_bundle,
            )
            .map_err(|error| {
                format!(
                    "acknowledged projection observation {}: {error:?}",
                    record.role
                )
            })?;
            if current.acknowledged_card_projection() != Some(&expected_card_projection)
                || current.projection_required()
                || acknowledged_bundle.projection_digest() != &expected_card_projection
                || acknowledged_observation != CardProjectionObservation::Healthy
            {
                return Err(format!(
                    "projection acknowledgement for {} failed exact readback: acknowledged={:?} required={} expected={} regenerated={} observation={:?}",
                    record.role,
                    current.acknowledged_card_projection().map(Digest::as_str),
                    current.projection_required(),
                    expected_card_projection.as_str(),
                    acknowledged_bundle.projection_digest().as_str(),
                    acknowledged_observation,
                ));
            }
        }
        if !retained_projection {
            if record.role != "prepared" {
                let bound_projection = canonical_linked_worktree
                    .join(".csdlc/v3/issues")
                    .join(record.issue.to_string());
                copy_tree(&bound_projection, &rehearsal_projection)?;
            }
            operation.marker(
                &format!("checkpoints/projection-published-{}.json", record.issue),
                "projection_published",
                json!({"issue": record.issue}),
            )?;
        }
        operation.fault("projection_publication", "after")?;
        converted.push(ConvertedRecord {
            role: record.role.clone(),
            issue: record.issue,
            generation: current.version().generation(),
            digest: current.version().digest().as_str().to_owned(),
            projection_digest: projection_digest.as_str().to_owned(),
            disposition: "converted".to_owned(),
        });
    }

    operation.fault("candidate_executable_activation", "before")?;
    let executable_slot = operation.root.join("executable-slot");
    fs::create_dir_all(&executable_slot)
        .map_err(|error| format!("{}: {error}", executable_slot.display()))?;
    let prior_executable = executable_slot.join("prior");
    if !prior_executable.is_file() {
        write_create_once(&prior_executable, &authority_bytes)?;
    }
    let candidate_bytes = fs::read(std::env::current_exe().map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;
    replace_durable(&executable_slot.join("active"), &candidate_bytes)?;
    operation.marker(
        "checkpoints/candidate-executable-activated.json",
        "candidate_executable_activated",
        json!({"candidate_size": candidate_bytes.len()}),
    )?;
    operation.fault("candidate_executable_activation", "after")?;

    operation.fault("fake_remote_request_dispatch", "before")?;
    let ledger = operation.root.join("fake-transport-ledger.jsonl");
    let ledger_line = serde_json::to_vec(&json!({
        "schema": "csdlc.v3.copied_record_conversion_fake_transport.v1",
        "operation_id": operation.id(),
        "effect": "synthetic_remote_conversion_acknowledgement",
    }))
    .map_err(|error| error.to_string())?;
    let mut ledger_bytes = ledger_line;
    ledger_bytes.push(b'\n');
    write_create_once(&ledger, &ledger_bytes)?;
    operation.fault("fake_remote_request_dispatch", "after")?;

    operation.fault("fake_remote_success_readback", "before")?;
    operation.marker(
        "remote/readback.json",
        "fake_remote_success_read_back",
        json!({"ledger": ledger, "effect_count": 1}),
    )?;
    operation.fault("fake_remote_success_readback", "after")?;

    operation.fault("local_reconciled_success_persistence", "before")?;
    operation.marker(
        "checkpoints/local-reconciled-success.json",
        "local_reconciled_success_persisted",
        json!({"converted_count": converted.iter().filter(|record| record.disposition == "converted").count()}),
    )?;
    operation.fault("local_reconciled_success_persistence", "after")?;

    operation.fault("restore_intent_durability", "before")?;
    operation.marker(
        "restore/intent.json",
        "restore_intent_durable",
        json!({"source": "staged_census", "target": "copied_sources"}),
    )?;
    operation.fault("restore_intent_durability", "after")?;

    operation.fault("source_record_restoration", "before")?;
    let restored = operation.root.join("restore/source");
    for record in &request.records {
        copy_tree(
            &staging.join(record.issue.to_string()),
            &restored.join(record.issue.to_string()),
        )?;
    }
    operation.marker(
        "restore/source-restored.json",
        "source_records_restored",
        json!({"record_count": request.records.len()}),
    )?;
    operation.fault("source_record_restoration", "after")?;

    operation.fault("prior_executable_restoration", "before")?;
    replace_durable(&executable_slot.join("active"), &authority_bytes)?;
    operation.marker(
        "restore/prior-executable-restored.json",
        "prior_executable_restored",
        json!({"restored_size": authority_bytes.len()}),
    )?;
    operation.fault("prior_executable_restoration", "after")?;

    operation.fault("restore_receipt_persistence_and_fence_release", "before")?;
    operation.marker(
        "restore/receipt.json",
        "restore_receipt_persisted",
        json!({"status": "restored"}),
    )?;
    replace_durable(&fence, b"released\n")?;
    operation.append("conversion_fence_released", json!({}))?;
    operation.fault("restore_receipt_persistence_and_fence_release", "after")?;
    operation.append(
        "operation_completed",
        json!({"record_count": converted.len()}),
    )?;
    Ok(converted)
}

pub fn observe(git_common: &Path, repository: &str, issue: u64) -> Result<Value, String> {
    let root = SemanticRoot::from_git_common(git_common, repository.to_owned())
        .map_err(|error| format!("{error:?}"))?;
    let key = IssueKey::new(repository.to_owned(), issue).map_err(|error| format!("{error:?}"))?;
    match DurableTransactionStore::observe_issue(&root, &key)
        .map_err(|error| format!("{error:?}"))?
    {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            Ok(json!({
                "schema":"csdlc.v3.copied_record_conversion_observation.v1",
                "status":"passed", "issue":issue,
                "generation":snapshot.version().generation(),
                "digest":snapshot.version().digest().as_str(),
                "projection_required":snapshot.projection_required(),
            }))
        }
        Observation::LegacyMigrationRequired => {
            Err("intent_semantic_migration_required".to_owned())
        }
        Observation::RecoveryRequired => Err("conversion_recovery_required".to_owned()),
        Observation::Absent => Err("converted_issue_absent".to_owned()),
    }
}

fn findings(label: &str, values: Vec<crate::commands::local::DoctorFinding>) -> String {
    format!("{label}: {values:?}")
}

fn relocation_source_git_common(path: &Path, issue: u64) -> Result<PathBuf, String> {
    let canonical =
        fs::canonicalize(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut common = canonical.as_path();
    for _ in 0..5 {
        common = common.parent().ok_or_else(|| {
            "source_semantic_current is outside the native semantic layout".to_owned()
        })?;
    }
    let common = fs::canonicalize(common).map_err(|error| error.to_string())?;
    let expected = common
        .join("csdlc-v3/semantic/issues")
        .join(issue.to_string())
        .join("current.json");
    if canonical != expected {
        return Err(
            "source_semantic_current does not name the exact native current pointer".to_owned(),
        );
    }
    Ok(common)
}

fn relocation_hash_file(path: &Path) -> Result<Digest, String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(Digest::semantic_projection(&bytes))
}

fn absolute_registry(
    repository_root: &Path,
    registry_path: &Path,
) -> Result<(PromptRegistry, Vec<u8>), String> {
    let bytes =
        fs::read(registry_path).map_err(|error| format!("{}: {error}", registry_path.display()))?;
    let mut registry = PromptRegistry::from_current_json(&bytes)
        .map_err(|values| findings("invalid target registry", values))?;
    for path in registry.template_paths.values_mut() {
        let candidate = Path::new(path);
        if !candidate.is_absolute() {
            *path = repository_root
                .join(candidate)
                .to_string_lossy()
                .into_owned();
        }
    }
    Ok((registry, bytes))
}

/// Relocate one complete, currently observable copied record into an authenticated
/// isolated checkout. This owner is deliberately separate from the seven-role
/// legacy conversion denominator and never repairs or normalizes the source.
pub fn relocate_current_observation_copy(
    request: &CurrentObservationRelocationRequest,
) -> Result<CurrentObservationRelocationResult, String> {
    if request.schema != "csdlc.v3.current_observation_relocation.v1" {
        return Err("unsupported current-observation relocation schema".to_owned());
    }
    if request.issue == 0
        || request.operation_id.is_empty()
        || !request
            .operation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err("invalid relocation issue or operation_id".to_owned());
    }

    // Authenticate and fully observe the copied source before the target obtains
    // any local, semantic, projection, journal, or provenance entry.
    let source_local = fs::canonicalize(&request.source_local_issue)
        .map_err(|error| format!("{}: {error}", request.source_local_issue.display()))?;
    if source_local.file_name().and_then(|value| value.to_str())
        != Some(request.issue.to_string().as_str())
        || source_local
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            != Some("issues")
    {
        return Err("source_local_issue does not name the exact issue directory".to_owned());
    }
    let local_state_root = source_local
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "source local lifecycle layout is incomplete".to_owned())?;
    let local_observation = inspect_v3_local_state(local_state_root, request.issue);
    if local_observation.status != PlanStatus::Ready
        || local_observation.digest.is_none()
        || local_observation.generation.is_none()
    {
        return Err(format!(
            "source local lifecycle is not a complete authentic record: {}",
            local_observation.code
        ));
    }
    let source_local_cards = load_cards(&source_local)?;
    let source_common =
        relocation_source_git_common(&request.source_semantic_current, request.issue)?;
    let source_root = SemanticRoot::from_git_common(&source_common, request.repository.clone())
        .map_err(|error| format!("source semantic root: {error:?}"))?;
    let key = IssueKey::new(request.repository.clone(), request.issue)
        .map_err(|error| format!("{error:?}"))?;
    let source_snapshot = match DurableTransactionStore::observe_issue(&source_root, &key)
        .map_err(|error| format!("source semantic observation: {error:?}"))?
    {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            snapshot
        }
        Observation::Absent => return Err("source semantic observation is absent".to_owned()),
        Observation::RecoveryRequired => {
            return Err("source semantic observation requires recovery".to_owned())
        }
        Observation::LegacyMigrationRequired => {
            return Err("source semantic observation requires migration".to_owned())
        }
    };
    if source_snapshot.phase() == LifecycleState::Bound {
        return Err(
            "source semantic observation is incomplete/recovery-required at bound phase".to_owned(),
        );
    }
    if source_local_cards.len() != SEMANTIC_CARD_KINDS.len() {
        return Err("source local lifecycle lacks the six-card denominator".to_owned());
    }
    let source_cards = source_snapshot.inputs().cards().clone();
    let source_projection = fs::read(&request.source_projection_state)
        .map_err(|error| format!("{}: {error}", request.source_projection_state.display()))?;
    if source_projection
        != source_snapshot
            .projection_bytes()
            .map_err(|error| format!("source projection: {error:?}"))?
    {
        return Err(
            "source projection bytes do not match the authenticated semantic record".to_owned(),
        );
    }

    let source_local_hash = source_digest(&source_local)?;
    let source_semantic_hash = relocation_hash_file(&request.source_semantic_current)?;
    let source_projection_hash = Digest::semantic_projection(&source_projection);
    let (target_common, target_worktree, target_primary) = authenticate_relocation_target(request)?;
    if (request.target_worktree_role == "linked") != source_snapshot.inputs().binding().is_some() {
        return Err(
            "source semantic binding does not match the authenticated target role".to_owned(),
        );
    }
    let (registry, registry_bytes) = absolute_registry(&target_worktree, &request.registry_path)?;
    let selector_bytes = fs::read(target_primary.join(crate::authority::SELECTOR_PATH))
        .map_err(|error| format!("target authority selector: {error}"))?;
    let target_authority = Digest::authority(&selector_bytes);
    if source_snapshot.inputs().authority() != &target_authority {
        return Err("source and target authority identities differ".to_owned());
    }

    let identity = json!({
        "schema": request.schema,
        "repository": request.repository,
        "issue": request.issue,
        "source": {
            "local": source_local_hash.as_str(),
            "semantic": source_semantic_hash.as_str(),
            "projection": source_projection_hash.as_str(),
        },
        "target": {
            "git_common": target_common,
            "worktree": target_worktree,
            "branch": request.target_branch,
            "head": request.target_head,
            "role": request.target_worktree_role,
        },
        "authority": target_authority.as_str(),
        "registry": Digest::projection(&registry_bytes).as_str(),
    });
    let request_digest = Digest::semantic_projection(&canonical_json_bytes(&identity)?);
    let csdlc = create_durable_child(&target_common, "csdlc-v3")?;
    let local = create_durable_child(&csdlc, "local")?;
    let relocations = create_durable_child(&local, "current-observation-relocations")?;
    let operation_root = create_durable_child(&relocations, &request.operation_id)?;
    let journal_path = operation_root.join("journal.json");
    write_create_once(
        &journal_path,
        &serde_json::to_vec_pretty(&json!({
            "schema":"csdlc.v3.current_observation_relocation_journal.v1",
            "operation_id":request.operation_id,
            "request_digest":request_digest.as_str(),
            "status":"admitted"
        }))
        .map_err(|error| error.to_string())?,
    )?;

    let source_index = read_json(&source_local.join("index.json"))?;
    let title = source_index
        .get("title")
        .and_then(Value::as_str)
        .or_else(|| {
            source_cards
                .get("sip")
                .and_then(|card| card.get("title"))
                .and_then(Value::as_str)
        })
        .unwrap_or("Relocated current observation")
        .to_owned();
    let primary_state_root = target_common.join("csdlc-v3/local");
    let target_issue_root = if request.target_worktree_role == "primary" {
        primary_state_root
            .join("issues")
            .join(request.issue.to_string())
    } else {
        target_worktree
            .join(".csdlc/issues")
            .join(request.issue.to_string())
    };
    let mut expected_target_cards = source_cards.clone();
    for (kind, card) in &mut expected_target_cards {
        let object = card
            .as_object_mut()
            .ok_or_else(|| format!("source {kind} values are not an object"))?;
        object.insert("issue".to_owned(), json!(request.issue));
        object.insert(
            "issue_padded".to_owned(),
            json!(format!("{:04}", request.issue)),
        );
        object.insert(
            "issue_url".to_owned(),
            json!(format!(
                "https://github.com/{}/issues/{}",
                request.repository, request.issue
            )),
        );
        object.insert("repository".to_owned(), json!(request.repository));
        object.insert("branch".to_owned(), json!(request.target_branch));
        object.insert("worktree".to_owned(), json!(target_worktree));
        object.insert("title".to_owned(), json!(title));
        object.insert("version".to_owned(), json!(registry.version));
        object.insert("card".to_owned(), json!(kind));
        object.insert("card_status".to_owned(), json!("ready"));
    }
    let card_updates = expected_target_cards.clone();
    let mut accepted = source_snapshot.inputs().accepted_plan().clone();
    accepted.cards = expected_target_cards.clone();
    let binding = source_snapshot.inputs().binding().map(|_| Binding {
        branch: request.target_branch.clone(),
        head: request.target_head.clone(),
        worktree: target_worktree.clone(),
        registration: "git-worktree-list".to_owned(),
    });
    let target_inputs = IssueInputs::new(
        source_snapshot.inputs().intent().to_owned(),
        accepted,
        source_snapshot.inputs().plan().to_vec(),
        binding,
        target_authority.clone(),
    )
    .map_err(|error| format!("relocated semantic input: {error:?}"))?;
    let target_root = SemanticRoot::from_git_common(&target_common, request.repository.clone())
        .map_err(|error| format!("target semantic root: {error:?}"))?;
    let mut current = match DurableTransactionStore::observe_issue(&target_root, &key)
        .map_err(|error| format!("target semantic observation: {error:?}"))?
    {
        Observation::Absent => snapshot(
            DurableTransactionStore::convert_copied_issue(
                &target_root,
                CopiedRecordConversion {
                    key: key.clone(),
                    inputs: target_inputs.clone(),
                    phase: source_snapshot.phase(),
                    source_generation: source_snapshot.version().generation(),
                    source_digest: source_local_hash.clone(),
                },
            )
            .map_err(|error| format!("target semantic activation: {error:?}"))?,
        ),
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot)
            if snapshot.inputs() == &target_inputs
                && snapshot.phase() == source_snapshot.phase() =>
        {
            snapshot
        }
        _ => return Err("target semantic state conflicts with relocation operation".to_owned()),
    };
    let final_request = |expected_lifecycle_digest: Option<String>| LocalPreparationRequest {
        issue: request.issue,
        title: title.clone(),
        repository: request.repository.clone(),
        branch: request.target_branch.clone(),
        worktree: target_worktree.to_string_lossy().into_owned(),
        registry_version: registry.version.clone(),
        expected_lifecycle_digest,
        commands: required_local_commands().to_vec(),
        card_updates: card_updates.clone(),
        schedule_readiness: None,
        shepherd_routing: None,
    };
    let target_observation = || {
        if request.target_worktree_role == "primary" {
            inspect_v3_local_state(&primary_state_root, request.issue)
        } else {
            inspect_local_lifecycle_state(&target_worktree, request.issue)
        }
    };
    let primary_observation = || inspect_v3_local_state(&primary_state_root, request.issue);
    let observed = target_observation();
    if observed.code == "missing_local_lifecycle_state" {
        let primary_observed = primary_observation();
        if primary_observed.code != "missing_local_lifecycle_state" {
            return Err("target primary contains conflicting lifecycle state".to_owned());
        }
        let bootstrap_branch = format!("codex/relocate-{}", request.operation_id);
        let bootstrap_worktree = target_primary
            .parent()
            .unwrap_or(&target_primary)
            .join(format!(".relocate-{}", request.operation_id));
        let bootstrap = LocalPreparationRequest {
            branch: bootstrap_branch,
            worktree: bootstrap_worktree.to_string_lossy().into_owned(),
            card_updates: BTreeMap::new(),
            ..final_request(None)
        };
        let context = discover_operational_local_context(&target_primary, &bootstrap)
            .map_err(|values| findings("target local context", values))?
            .ok_or_else(|| "target native v3 authority is inactive".to_owned())?;
        execute_operational_local_route("issue", &bootstrap, &registry, &context)
            .map_err(|values| findings("target local initialization", values))?;
    }
    let observed = target_observation();
    if observed.code == "missing_local_lifecycle_state" {
        let primary_observed = primary_observation();
        let local_request = final_request(primary_observed.digest.clone());
        let primary_issue_root = primary_state_root
            .join("issues")
            .join(request.issue.to_string());
        let primary_cards_before = load_cards(&primary_issue_root)?;
        if primary_cards_before != expected_target_cards
            || read_json(&primary_issue_root.join("index.json"))?
                .get("branch")
                .and_then(Value::as_str)
                != Some(request.target_branch.as_str())
        {
            let context = discover_operational_local_context(&target_primary, &local_request)
                .map_err(|values| findings("target local edit context", values))?
                .ok_or_else(|| "target native v3 authority is inactive".to_owned())?;
            execute_operational_local_route("edit", &local_request, &registry, &context)
                .map_err(|values| findings("target local relocation", values))?;
        }
        if request.target_worktree_role == "linked" {
            let edited = primary_observation();
            let bind_request = final_request(edited.digest.clone());
            let context = discover_operational_local_context(&target_primary, &bind_request)
                .map_err(|values| findings("target local bind context", values))?
                .ok_or_else(|| "target native v3 authority is inactive".to_owned())?;
            execute_operational_local_route("bind", &bind_request, &registry, &context)
                .map_err(|values| findings("target local binding", values))?;
        }
    } else {
        let local_request = final_request(observed.digest.clone());
        let target_cards_before = load_cards(&target_issue_root)?;
        if target_cards_before != expected_target_cards
            || read_json(&target_issue_root.join("index.json"))?
                .get("branch")
                .and_then(Value::as_str)
                != Some(request.target_branch.as_str())
        {
            let context = discover_operational_local_context(&target_worktree, &local_request)
                .map_err(|values| findings("target local edit context", values))?
                .ok_or_else(|| "target native v3 authority is inactive".to_owned())?;
            execute_operational_local_route("edit", &local_request, &registry, &context)
                .map_err(|values| findings("target local relocation", values))?;
        }
    }
    let relocated_local = target_observation();
    let relocated_cards = load_cards(&target_issue_root)?;
    if relocated_local.status != PlanStatus::Ready || relocated_cards != expected_target_cards {
        return Err(format!(
            "relocated local lifecycle failed exact native readback: code={} cards_match={}",
            relocated_local.code,
            relocated_cards == expected_target_cards,
        ));
    }

    DurableTransactionStore::write_issue_projection(&target_root, &current)
        .map_err(|error| format!("target issue projection: {error:?}"))?;
    let bundle = copied_card_projection(&current, &registry, &target_issue_root)?;
    let projection_healthy = !current.projection_required()
        && current.acknowledged_card_projection() == Some(bundle.projection_digest())
        && DurableTransactionStore::observe_card_projection(&target_root, &current, &bundle)
            .is_ok_and(|observation| observation == CardProjectionObservation::Healthy);
    if !projection_healthy {
        let proof = DurableTransactionStore::write_card_projection(&target_root, &current, bundle)
            .map_err(|error| format!("target card projection: {error:?}"))?;
        current = snapshot(
            DurableTransactionStore::commit_issue_local(
                &target_root,
                Admission::new(key, current.version().clone(), target_authority),
                LocalChange::AcknowledgeProjection(proof),
            )
            .map_err(|error| format!("target projection acknowledgement: {error:?}"))?,
        );
    }

    let target_semantic_current = target_common
        .join("csdlc-v3/semantic/issues")
        .join(request.issue.to_string())
        .join("current.json");
    let target_projection_state = if request.target_worktree_role == "primary" {
        target_common
            .join("csdlc-v3/local/projections")
            .join(request.issue.to_string())
            .join("state.json")
    } else {
        target_worktree
            .join(".csdlc/v3/issues")
            .join(request.issue.to_string())
            .join("state.json")
    };
    let mut source_hashes = BTreeMap::new();
    source_hashes.insert(
        "local_lifecycle".to_owned(),
        source_local_hash.as_str().to_owned(),
    );
    source_hashes.insert(
        "semantic_current".to_owned(),
        source_semantic_hash.as_str().to_owned(),
    );
    source_hashes.insert(
        "projection_state".to_owned(),
        source_projection_hash.as_str().to_owned(),
    );
    let mut relocated_hashes = BTreeMap::new();
    relocated_hashes.insert(
        "local_lifecycle".to_owned(),
        source_digest(&target_issue_root)?.as_str().to_owned(),
    );
    relocated_hashes.insert(
        "semantic_current".to_owned(),
        relocation_hash_file(&target_semantic_current)?
            .as_str()
            .to_owned(),
    );
    relocated_hashes.insert(
        "projection_state".to_owned(),
        relocation_hash_file(&target_projection_state)?
            .as_str()
            .to_owned(),
    );
    if source_digest(&source_local)? != source_local_hash
        || relocation_hash_file(&request.source_semantic_current)? != source_semantic_hash
        || relocation_hash_file(&request.source_projection_state)? != source_projection_hash
    {
        return Err("source bytes changed during relocation".to_owned());
    }
    let provenance_path = operation_root.join("provenance.json");
    let provenance = json!({
        "schema":"csdlc.v3.current_observation_relocation_provenance.v1",
        "operation_id":request.operation_id,
        "request_digest":request_digest.as_str(),
        "issue":request.issue,
        "source_hashes":source_hashes,
        "relocated_hashes":relocated_hashes,
        "generation":current.version().generation(),
        "digest":current.version().digest().as_str(),
    });
    write_create_once(
        &provenance_path,
        &serde_json::to_vec_pretty(&provenance).map_err(|error| error.to_string())?,
    )?;
    Ok(CurrentObservationRelocationResult {
        schema: "csdlc.v3.current_observation_relocation_result.v1".to_owned(),
        status: "completed".to_owned(),
        operation_id: request.operation_id.clone(),
        issue: request.issue,
        generation: current.version().generation(),
        digest: current.version().digest().as_str().to_owned(),
        source_hashes,
        relocated_hashes,
        journal_path,
        provenance_path,
    })
}
