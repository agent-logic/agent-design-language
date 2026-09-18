//! Narrow intent bridges into the existing local owner and transaction journal.

use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
    net::{SocketAddr, UdpSocket},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::cards::{initial_card_values, merge_json_object, render_template, structure_valid};
use super::context::{require_operational_cas, validate_context};
use super::filesystem::io_finding;
use super::issue::initialize_operational_issue_with_plan;
use super::lifecycle::inspect_lifecycle_issue_root;
use super::planning::{plan_cards, validate_contract};
use super::storage::{lifecycle_digest, read_index_value};
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

pub(crate) struct SemanticPrepareTarget<'a> {
    pub(crate) root: &'a crate::storage::semantic::SemanticRoot,
    pub(crate) key: crate::storage::semantic::IssueKey,
    pub(crate) authority: crate::storage::semantic::Digest,
    pub(crate) legacy_binding: Option<crate::storage::semantic::Binding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdoptionFenceRequest {
    schema: String,
    issue: u64,
    request_digest: String,
    git_common: PathBuf,
    release_path: PathBuf,
    ready_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdoptionFenceReady {
    schema: String,
    issue: u64,
    request_digest: String,
    challenge_port: u16,
}

pub(crate) struct AdoptionFenceLease {
    request: AdoptionFenceRequest,
    guard: crate::storage::semantic::NativeWriterFenceGuard,
    // Stable per-issue client lock survives guardian journal cleanup. Keep it
    // through activation, projection, and release so another caller cannot
    // acquire or abort this caller's guardian.
    _client: File,
}

pub(crate) struct SemanticPrepareResult {
    pub(crate) snapshot: crate::storage::semantic::Snapshot,
    pub(crate) adoption_fence: Option<AdoptionFenceLease>,
}

fn adoption_fence_challenge(request: &AdoptionFenceRequest) -> Vec<u8> {
    format!(
        "csdlc.v3.bound_legacy_adoption_fence.challenge.v1\n{}\n{}\n",
        request.issue, request.request_digest
    )
    .into_bytes()
}

fn adoption_fence_response(challenge: &[u8]) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc.v3.bound_legacy_adoption_fence.response.v1\0");
    hasher.update(challenge);
    format!("{}\n", hasher.finalize().to_hex()).into_bytes()
}

fn adoption_guardian_ready(request: &AdoptionFenceRequest) -> Option<AdoptionFenceReady> {
    let value: AdoptionFenceReady =
        serde_json::from_slice(&fs::read(&request.ready_path).ok()?).ok()?;
    (value.schema == "csdlc.v3.bound_legacy_adoption_fence_ready.v1"
        && value.issue == request.issue
        && value.request_digest == request.request_digest
        && value.challenge_port != 0)
        .then_some(value)
}

fn adoption_guardian_authenticates(
    request: &AdoptionFenceRequest,
    ready: &AdoptionFenceReady,
) -> bool {
    let Ok(socket) = UdpSocket::bind(("127.0.0.1", 0)) else {
        return false;
    };
    let _ = socket.set_read_timeout(Some(Duration::from_millis(250)));
    let _ = socket.set_write_timeout(Some(Duration::from_millis(250)));
    if socket
        .connect(SocketAddr::from(([127, 0, 0, 1], ready.challenge_port)))
        .is_err()
    {
        return false;
    }
    let challenge = adoption_fence_challenge(request);
    if socket.send(&challenge).ok() != Some(challenge.len()) {
        return false;
    }
    let mut response = [0_u8; 128];
    let Ok(size) = socket.recv(&mut response) else {
        return false;
    };
    response[..size] == adoption_fence_response(&challenge)
}

fn exact_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if path.exists() {
        return (fs::read(path).map_err(|error| error.to_string())? == bytes)
            .then_some(())
            .ok_or_else(|| format!("{} has conflicting retained bytes", path.display()));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    File::open(path.parent().ok_or("adoption fence parent missing")?)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| error.to_string())
}

fn replace_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("adoption fence parent missing")?;
    let next = parent.join(format!(
        ".{}.next",
        path.file_name()
            .ok_or("adoption fence file name missing")?
            .to_string_lossy()
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&next)
        .map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    fs::rename(next, path).map_err(|error| error.to_string())?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| error.to_string())
}

fn native_lock_is_held(common: &Path, issue: u64) -> bool {
    let path = common
        .join("csdlc-v3/local/locks")
        .join(format!("{issue}.lock"));
    let Ok(file) = OpenOptions::new().read(true).write(true).open(path) else {
        return false;
    };
    file.try_lock_exclusive().is_err()
}

fn adoption_fence_request(
    common: &Path,
    issue: u64,
    request_digest: &str,
) -> Result<AdoptionFenceRequest, String> {
    if request_digest.len() != 64 || !request_digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid bound legacy adoption digest".into());
    }
    let common = fs::canonicalize(common).map_err(|error| error.to_string())?;
    let root = common
        .join("csdlc-v3/semantic/adoption-fences")
        .join(issue.to_string())
        .join(request_digest);
    if !root.starts_with(&common) {
        return Err("adoption fence path escaped Git common directory".into());
    }
    for ancestor in root.ancestors().take_while(|path| *path != common) {
        if fs::symlink_metadata(ancestor).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return Err("adoption fence path contains a symlink".into());
        }
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(AdoptionFenceRequest {
        schema: "csdlc.v3.bound_legacy_adoption_fence_request.v1".into(),
        issue,
        request_digest: request_digest.into(),
        git_common: common,
        release_path: root.join("fence.state"),
        ready_path: root.join("guardian-ready.json"),
    })
}

fn acquire_durable_adoption_fence(
    common: &Path,
    issue: u64,
    request_digest: &str,
    durable_required: bool,
) -> Result<Option<AdoptionFenceLease>, String> {
    let client_path = common
        .join("csdlc-v3/local/locks")
        .join(format!("{issue}.adoption.lock"));
    if fs::symlink_metadata(&client_path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("adoption client lock must not be a symlink".into());
    }
    let client = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(client_path)
        .map_err(|error| error.to_string())?;
    client
        .try_lock_exclusive()
        .map_err(|_| "bound legacy adoption is already in progress".to_owned())?;
    let request = adoption_fence_request(common, issue, request_digest)?;
    let active = fs::read(&request.release_path).ok().as_deref() == Some(b"active\n");
    if !durable_required && !active {
        return Ok(None);
    }
    let request_path = request
        .release_path
        .parent()
        .ok_or("adoption fence root missing")?
        .join("guardian-request.json");
    exact_file(
        &request_path,
        &serde_json::to_vec_pretty(&request).map_err(|error| error.to_string())?,
    )?;
    let bootstrap_path = request
        .release_path
        .parent()
        .ok_or("adoption fence root missing")?
        .join("guardian-start.lock");
    let bootstrap = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(bootstrap_path)
        .map_err(|error| error.to_string())?;
    bootstrap
        .lock_exclusive()
        .map_err(|error| error.to_string())?;
    let retained = adoption_guardian_ready(&request);
    if !retained.as_ref().is_some_and(|ready| {
        adoption_guardian_authenticates(&request, ready)
            && native_lock_is_held(&request.git_common, issue)
    }) {
        if active && native_lock_is_held(&request.git_common, issue) {
            return Err("active adoption fence has no authenticated guardian".into());
        }
        Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
            .args(["__bound-legacy-adoption-guardian", "--request"])
            .arg(&request_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("start adoption fence guardian: {error}"))?;
    }
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if adoption_guardian_ready(&request)
            .as_ref()
            .is_some_and(|ready| {
                adoption_guardian_authenticates(&request, ready)
                    && native_lock_is_held(&request.git_common, issue)
            })
        {
            let guard = crate::storage::semantic::NativeWriterFenceGuard::authenticated_guardian(
                &request.git_common,
                [issue],
            )
            .map_err(|error| format!("authenticate adoption guardian: {error:?}"))?;
            return Ok(Some(AdoptionFenceLease {
                request,
                guard,
                _client: client,
            }));
        }
        if Instant::now() >= deadline {
            return Err("adoption fence guardian did not authenticate held native lock".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

impl AdoptionFenceLease {
    pub(crate) fn release(self) -> Result<(), String> {
        self.release_guardian()
    }

    fn release_guardian(&self) -> Result<(), String> {
        replace_file(&self.request.release_path, b"released\n")?;
        let deadline = Instant::now() + Duration::from_secs(15);
        while native_lock_is_held(&self.request.git_common, self.request.issue) {
            if Instant::now() >= deadline {
                return Err("adoption fence guardian did not release native lock".into());
            }
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    fn abort(self) -> Result<(), String> {
        let root = self
            .request
            .release_path
            .parent()
            .ok_or("adoption fence root missing")?
            .to_owned();
        self.release_guardian()?;
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        if let Some(issue_root) = root.parent() {
            let _ = fs::remove_dir(issue_root);
            if let Some(fences_root) = issue_root.parent() {
                let _ = fs::remove_dir(fences_root);
            }
        }
        Ok(())
    }
}

pub fn run_bound_legacy_adoption_guardian(request_path: &Path) -> Result<(), String> {
    let request: AdoptionFenceRequest =
        serde_json::from_slice(&fs::read(request_path).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?;
    if request.schema != "csdlc.v3.bound_legacy_adoption_fence_request.v1" {
        return Err("unsupported adoption fence guardian request".into());
    }
    let expected =
        adoption_fence_request(&request.git_common, request.issue, &request.request_digest)?;
    let expected_path = expected
        .release_path
        .parent()
        .ok_or("adoption fence root missing")?
        .join("guardian-request.json");
    if request_path != expected_path
        || request.release_path != expected.release_path
        || request.ready_path != expected.ready_path
    {
        return Err("adoption fence guardian paths escaped authenticated root".into());
    }
    let _guard = crate::storage::semantic::NativeWriterFenceGuard::acquire(
        &request.git_common,
        [request.issue],
    )
    .map_err(|error| format!("acquire adoption writer fence: {error:?}"))?;
    let retained_port = adoption_guardian_ready(&request)
        .map(|ready| ready.challenge_port)
        .unwrap_or(0);
    let listener =
        UdpSocket::bind(("127.0.0.1", retained_port)).map_err(|error| error.to_string())?;
    listener
        .set_nonblocking(true)
        .map_err(|error| error.to_string())?;
    replace_file(&request.release_path, b"active\n")?;
    replace_file(
        &request.ready_path,
        &serde_json::to_vec_pretty(&AdoptionFenceReady {
            schema: "csdlc.v3.bound_legacy_adoption_fence_ready.v1".into(),
            issue: request.issue,
            request_digest: request.request_digest.clone(),
            challenge_port: listener
                .local_addr()
                .map_err(|error| error.to_string())?
                .port(),
        })
        .map_err(|error| error.to_string())?,
    )?;
    let challenge = adoption_fence_challenge(&request);
    let response = adoption_fence_response(&challenge);
    loop {
        if fs::read(&request.release_path).ok().as_deref() == Some(b"released\n") {
            return Ok(());
        }
        let mut received = [0_u8; 512];
        match listener.recv_from(&mut received) {
            Ok((size, peer)) if received[..size] == challenge => {
                let _ = listener.send_to(&response, peer);
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => return Err(error.to_string()),
        }
    }
}

fn semantic_git_common(context: &OperationalLocalContext) -> Result<PathBuf, Vec<DoctorFinding>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(&context.repository_root)
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output()
        .map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "semantic_prepare_git_common_missing",
                "native state root has no Git common parent",
            )]
        })?;
    if !output.status.success() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_git_common_missing",
            "native state root has no Git common parent",
        )]);
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}

fn adoption_request_digest(
    request: &LocalPreparationRequest,
    inputs: &crate::storage::semantic::IssueInputs,
) -> Result<String, Vec<DoctorFinding>> {
    serde_json::to_vec(&serde_json::json!({
        "schema":"csdlc.v3.bound_legacy_adoption_identity.v1",
        "repository":request.repository,
        "issue":request.issue,
        "retained_digest":request.expected_lifecycle_digest,
        "binding":inputs.binding(),
    }))
    .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_adoption_identity_invalid",
            "bound legacy adoption identity could not be encoded",
        )]
    })
}

fn validate_legacy_under_fence(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    inputs: &crate::storage::semantic::IssueInputs,
) -> Result<(), Vec<DoctorFinding>> {
    let legacy_issue_root = context
        .state_root
        .join("issues")
        .join(request.issue.to_string());
    // Read and authenticate the retained source only after the old-writer lock
    // is held. A pre-fence observation is never the adoption CAS.
    let legacy_index = read_index_value(&legacy_issue_root)?;
    verify_integrity(&legacy_issue_root, &legacy_index)?;
    if legacy_index["digest"].as_str() != request.expected_lifecycle_digest.as_deref() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_lifecycle_changed",
            "retained lifecycle digest changed before writer-fenced adoption",
        )]);
    }
    if inputs.binding().is_some() {
        super::worktree::verify_bound_worktree(
            &request.branch,
            Path::new(&request.worktree),
            &context.expected_head_sha,
            false,
        )?;
    }
    validate_context(
        if inputs.binding().is_some() {
            "prepare"
        } else {
            "issue"
        },
        request,
        context,
    )?;
    let diagnosis = super::execute_operational_local_route("doctor", request, registry, context)?;
    if diagnosis
        .findings
        .iter()
        .any(|finding| matches!(finding.status, PlanStatus::Blocked | PlanStatus::Failed))
    {
        return Err(diagnosis.findings);
    }
    Ok(())
}

fn prepare_legacy_under_fence(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    root: &crate::storage::semantic::SemanticRoot,
    key: crate::storage::semantic::IssueKey,
    inputs: crate::storage::semantic::IssueInputs,
    fence: &crate::storage::semantic::NativeWriterFenceGuard,
) -> Result<crate::storage::semantic::CommitOutcome, Vec<DoctorFinding>> {
    validate_legacy_under_fence(request, registry, context, &inputs)?;
    crate::storage::DurableTransactionStore::prepare_legacy_native_issue_under_writer_fence(
        root, key, inputs, fence,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_refused",
            &format!("{error:?}"),
        )]
    })
}

pub(crate) fn prepare_semantic(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    plan: &Value,
    target: SemanticPrepareTarget<'_>,
) -> Result<SemanticPrepareResult, Vec<DoctorFinding>> {
    let SemanticPrepareTarget {
        root,
        key,
        authority,
        legacy_binding,
    } = target;
    validate_context(
        if legacy_binding.is_some() {
            "prepare"
        } else {
            "issue"
        },
        request,
        context,
    )?;
    if key.issue() != request.issue || key.repository() != request.repository {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "semantic_prepare_identity_mismatch",
            "native and semantic issue identity must match",
        )]);
    }
    let inputs = prepared_inputs(request, registry, plan, authority)?;
    let inputs = if let Some(binding) = legacy_binding {
        inputs.with_binding(binding).map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "semantic_bound_legacy_inputs_invalid",
                &format!("{error:?}"),
            )]
        })?
    } else {
        inputs
    };
    validate_context(
        if inputs.binding().is_some() {
            "prepare"
        } else {
            "issue"
        },
        request,
        context,
    )?;
    let observed =
        crate::storage::DurableTransactionStore::observe_issue(root, &key).map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "semantic_prepare_observation_failed",
                &format!("{error:?}"),
            )]
        })?;
    let mut adoption_fence = None;
    let prepared = if let crate::storage::semantic::Observation::Current(current)
    | crate::storage::semantic::Observation::ProjectionRepairRequired(current) =
        &observed
    {
        if current.phase() != crate::lifecycle::LifecycleState::Bound
            || current.inputs() != &inputs
            || current.pending().is_some()
            || !current.completed().is_empty()
        {
            Err(vec![finding(
                PlanStatus::Blocked,
                "semantic_prepare_refused",
                "AlreadyExists",
            )])
        } else if inputs.binding().is_none() {
            Err(vec![finding(
                PlanStatus::Blocked,
                "semantic_prepare_refused",
                "semantic prepare replay is not a bound legacy adoption",
            )])
        } else {
            let common = semantic_git_common(context)?;
            let identity = adoption_request_digest(request, &inputs)?;
            if let Some(lease) = acquire_durable_adoption_fence(
                &common,
                request.issue,
                &identity,
                crate::storage::semantic::ProjectionWriteProof::verify(root, current).is_err(),
            )
            .map_err(|error| {
                vec![finding(
                    PlanStatus::Blocked,
                    "semantic_prepare_writer_fence_failed",
                    &error,
                )]
            })? {
                let result = prepare_legacy_under_fence(
                    request,
                    registry,
                    context,
                    root,
                    key,
                    inputs,
                    &lease.guard,
                );
                adoption_fence = Some(lease);
                result
            } else {
                let _fence = crate::storage::semantic::NativeWriterFenceGuard::acquire(
                    &common,
                    [request.issue],
                )
                .map_err(|error| {
                    vec![finding(
                        PlanStatus::Blocked,
                        "semantic_prepare_writer_fence_failed",
                        &format!("{error:?}"),
                    )]
                })?;
                validate_legacy_under_fence(request, registry, context, &inputs)?;
                Ok(crate::storage::semantic::CommitOutcome::Unchanged(
                    current.clone(),
                ))
            }
        }
    } else if observed == crate::storage::semantic::Observation::LegacyMigrationRequired {
        let common = semantic_git_common(context)?;
        if inputs.binding().is_some() {
            let identity = adoption_request_digest(request, &inputs)?;
            let lease = acquire_durable_adoption_fence(&common, request.issue, &identity, true)
                .map_err(|error| {
                    vec![finding(
                        PlanStatus::Blocked,
                        "semantic_prepare_writer_fence_failed",
                        &error,
                    )]
                })?
                .ok_or_else(|| {
                    vec![finding(
                        PlanStatus::Blocked,
                        "semantic_prepare_writer_fence_failed",
                        "durable adoption fence was not established",
                    )]
                })?;
            #[cfg(debug_assertions)]
            if let Some(barrier) = std::env::var_os("CSDLC_V3_TEST_ADOPTION_BARRIER") {
                let barrier = PathBuf::from(barrier);
                fs::write(barrier.with_extension("ready"), b"fenced\n")
                    .expect("write adoption test barrier");
                let deadline = Instant::now() + Duration::from_secs(30);
                while !barrier.exists() {
                    assert!(Instant::now() < deadline, "adoption test barrier timed out");
                    thread::sleep(Duration::from_millis(10));
                }
            }
            let result = prepare_legacy_under_fence(
                request,
                registry,
                context,
                root,
                key,
                inputs,
                &lease.guard,
            );
            match result {
                Ok(outcome) => {
                    adoption_fence = Some(lease);
                    Ok(outcome)
                }
                Err(mut findings) => {
                    if let Err(error) = lease.abort() {
                        findings.push(finding(
                            PlanStatus::Blocked,
                            "semantic_prepare_writer_fence_cleanup_failed",
                            &error,
                        ));
                    }
                    Err(findings)
                }
            }
        } else {
            let fence =
                crate::storage::semantic::NativeWriterFenceGuard::acquire(&common, [request.issue])
                    .map_err(|error| {
                        vec![finding(
                            PlanStatus::Blocked,
                            "semantic_prepare_writer_fence_failed",
                            &format!("{error:?}"),
                        )]
                    })?;
            prepare_legacy_under_fence(request, registry, context, root, key, inputs, &fence)
        }
    } else {
        crate::storage::DurableTransactionStore::prepare_issue(root, key, inputs).map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "semantic_prepare_refused",
                &format!("{error:?}"),
            )]
        })
    };
    match prepared {
        Ok(crate::storage::semantic::CommitOutcome::Committed(snapshot))
        | Ok(crate::storage::semantic::CommitOutcome::Unchanged(snapshot)) => {
            Ok(SemanticPrepareResult {
                snapshot: *snapshot,
                adoption_fence,
            })
        }
        Err(findings) => Err(findings),
    }
}
