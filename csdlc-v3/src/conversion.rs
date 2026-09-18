//! Isolated, explicit conversion owner for copied lifecycle records.

use crate::commands::local::{
    discover_operational_local_context, execute_operational_local_route,
    inspect_local_lifecycle_state, inspect_v3_local_state, required_local_commands,
    LocalPreparationRequest, PlanStatus, PromptRegistry,
};
use crate::lifecycle::LifecycleState;
use crate::storage::semantic::{
    AcceptedIntentPlan, Binding, CardProjectionArtifact, CardProjectionBundle,
    CardProjectionObservation, CommitOutcome, CopiedRecordConversion, Digest, IssueInputs,
    IssueKey, NativeWriterFenceGuard, Observation, PlanStep, Publication, SemanticRoot, Snapshot,
    Validator, SEMANTIC_CARD_KINDS,
};
use crate::storage::DurableTransactionStore;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static CREATE_ONCE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

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
    pub prior_executable_path: PathBuf,
    pub prior_executable_blake3: String,
    pub writer_fence_issues: Vec<u64>,
    pub writer_probe_issue: u64,
    pub records: Vec<CopiedRecord>,
    #[serde(default)]
    pub writer_fence_probe: bool,
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

#[derive(Clone, Debug, Serialize)]
pub struct ConversionOperationEvidence {
    pub schema: String,
    pub operation_id: String,
    pub request_digest: String,
    pub outcome: String,
    pub abrupt_fault_point: Option<String>,
    pub abrupt_fault_boundary: Option<String>,
    pub journal_path: PathBuf,
    pub journal_event_count: usize,
    pub semantic_effect_count: usize,
    pub remote_effect_count: usize,
    pub remote_operation_identity: Option<String>,
    pub remote_state: String,
    pub remote_reconcile_count: usize,
    pub effect_paths: Vec<PathBuf>,
    pub readback_paths: Vec<PathBuf>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ConversionRestoreResult {
    pub schema: String,
    pub operation_id: String,
    pub status: String,
    pub allowed: bool,
    pub request_digest: String,
    pub source_record_count: usize,
    pub source_hashes_before: Vec<String>,
    pub source_hashes_after: Vec<String>,
    pub effect_count: usize,
    pub receipt_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WriterFenceGuardianRequest {
    schema: String,
    operation_id: String,
    request_digest: String,
    git_common: PathBuf,
    issues: Vec<u64>,
    fence_path: PathBuf,
    ready_path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WriterFenceGuardianReady {
    schema: String,
    operation_id: String,
    request_digest: String,
    issues: Vec<u64>,
    challenge_port: u16,
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
        validate_operation_id(&operation_id)?;
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
        if root.parent() != Some(rehearsals.as_path())
            || root.file_name() != Some(operation_id.as_ref())
        {
            return Err("conversion operation root escaped its canonical parent".to_owned());
        }
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
    if path.is_file() {
        return compare_create_once_bytes(path, bytes);
    }

    let parent = path.parent().expect("created file has parent");
    let file_name = path
        .file_name()
        .expect("created file has file name")
        .to_string_lossy();
    let sequence = CREATE_ONCE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temporary = parent.join(format!(
        ".{file_name}.{}.{}.create-once",
        std::process::id(),
        sequence
    ));
    let publish = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        file.write_all(bytes)
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        file.sync_all()
            .map_err(|error| format!("{}: {error}", temporary.display()))?;
        drop(file);

        match fs::hard_link(&temporary, path) {
            Ok(()) => sync_dir(parent),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                compare_create_once_bytes(path, bytes)
            }
            Err(error) => Err(format!(
                "publish {} as {}: {error}",
                temporary.display(),
                path.display()
            )),
        }
    })();
    let cleanup = match fs::remove_file(&temporary) {
        Ok(()) => sync_dir(parent),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("{}: {error}", temporary.display())),
    };
    publish.and(cleanup)
}

fn compare_create_once_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let existing = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    if existing == bytes {
        Ok(())
    } else {
        Err(format!(
            "{} already exists with different bytes",
            path.display()
        ))
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

fn guardian_ready(
    path: &Path,
    request: &WriterFenceGuardianRequest,
) -> Option<WriterFenceGuardianReady> {
    let Ok(bytes) = fs::read(path) else {
        return None;
    };
    let Ok(value) = serde_json::from_slice::<WriterFenceGuardianReady>(&bytes) else {
        return None;
    };
    (value.schema == "csdlc.v3.copied_record_writer_fence_guardian_ready.v1"
        && value.operation_id == request.operation_id
        && value.request_digest == request.request_digest
        && value.issues == request.issues
        && value.challenge_port != 0)
        .then_some(value)
}

fn guardian_challenge(request: &WriterFenceGuardianRequest) -> Vec<u8> {
    format!(
        "csdlc.v3.writer_fence_guardian.challenge.v1\n{}\n{}\n",
        request.operation_id, request.request_digest
    )
    .into_bytes()
}

fn guardian_response(challenge: &[u8]) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc.v3.writer_fence_guardian.response.v1\0");
    hasher.update(challenge);
    format!("{}\n", hasher.finalize().to_hex()).into_bytes()
}

fn guardian_channel_authenticates(
    ready: &WriterFenceGuardianReady,
    request: &WriterFenceGuardianRequest,
) -> bool {
    let address = SocketAddr::from(([127, 0, 0, 1], ready.challenge_port));
    let Ok(socket) = UdpSocket::bind(("127.0.0.1", 0)) else {
        return false;
    };
    let _ = socket.set_read_timeout(Some(Duration::from_millis(250)));
    let _ = socket.set_write_timeout(Some(Duration::from_millis(250)));
    if socket.connect(address).is_err() {
        return false;
    }
    let challenge = guardian_challenge(request);
    if socket.send(&challenge).ok() != Some(challenge.len()) {
        return false;
    }
    let mut response = [0_u8; 128];
    let Ok(size) = socket.recv(&mut response) else {
        return false;
    };
    response[..size] == guardian_response(&challenge)
}

fn retained_guardian_authenticates(
    ready_path: &Path,
    request: &WriterFenceGuardianRequest,
    common: &Path,
    issues: &[u64],
) -> Result<bool, String> {
    let retained_ready = guardian_ready(ready_path, request);
    if retained_ready.as_ref().is_some_and(|ready| {
        guardian_channel_authenticates(ready, request) && native_writer_locks_held(common, issues)
    }) {
        return Ok(true);
    }
    if retained_ready.is_some() && native_writer_locks_held(common, issues) {
        return Err(
            "stale writer-fence guardian readiness cannot authenticate busy native locks"
                .to_owned(),
        );
    }
    Ok(false)
}

fn native_writer_locks_held(common: &Path, issues: &[u64]) -> bool {
    issues.iter().all(|issue| {
        let path = common
            .join("csdlc-v3/local/locks")
            .join(format!("{issue}.lock"));
        let Ok(file) = OpenOptions::new().read(true).write(true).open(path) else {
            return false;
        };
        file.try_lock_exclusive().is_err()
    })
}

fn native_writer_locks_available(common: &Path, issues: &[u64]) -> bool {
    issues.iter().all(|issue| {
        let path = common
            .join("csdlc-v3/local/locks")
            .join(format!("{issue}.lock"));
        let Ok(file) = OpenOptions::new().read(true).write(true).open(path) else {
            return false;
        };
        file.try_lock_exclusive().is_ok()
    })
}

fn acquire_guarded_writer_fence(
    operation: &Operation<'_>,
    request_digest: &Digest,
    common: &Path,
    issues: &[u64],
) -> Result<NativeWriterFenceGuard, String> {
    let fence_path = operation.root.join("conversion.fence");
    let ready_path = operation.root.join("writer-fence-guardian-ready.json");
    let guardian_request = WriterFenceGuardianRequest {
        schema: "csdlc.v3.copied_record_writer_fence_guardian_request.v1".to_owned(),
        operation_id: operation.id().to_owned(),
        request_digest: request_digest.as_str().to_owned(),
        git_common: common.to_path_buf(),
        issues: issues.to_vec(),
        fence_path,
        ready_path: ready_path.clone(),
    };
    let request_path = operation.root.join("writer-fence-guardian-request.json");
    let request_bytes = serde_json::to_vec_pretty(&guardian_request).map_err(|e| e.to_string())?;
    write_create_once(&request_path, &request_bytes)?;

    if !retained_guardian_authenticates(&ready_path, &guardian_request, common, issues)? {
        let executable = std::env::current_exe().map_err(|error| error.to_string())?;
        Command::new(executable)
            .args(["writer-fence-guardian", "--request"])
            .arg(&request_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("start writer-fence guardian: {error}"))?;
    }

    let deadline = Instant::now() + Duration::from_secs(15);
    while !guardian_ready(&ready_path, &guardian_request).is_some_and(|ready| {
        guardian_channel_authenticates(&ready, &guardian_request)
            && native_writer_locks_held(common, issues)
    }) {
        if Instant::now() >= deadline {
            return Err("writer-fence guardian did not authenticate held native locks".to_owned());
        }
        thread::sleep(Duration::from_millis(10));
    }
    NativeWriterFenceGuard::authenticated_guardian(common, issues.iter().copied())
        .map_err(|error| format!("authenticate writer-fence guardian: {error:?}"))
}

fn release_guarded_writer_fence(
    operation: &Operation<'_>,
    common: &Path,
    issues: &[u64],
) -> Result<(), String> {
    replace_durable(&operation.root.join("conversion.fence"), b"released\n")?;
    let deadline = Instant::now() + Duration::from_secs(15);
    while !native_writer_locks_available(common, issues) {
        if Instant::now() >= deadline {
            return Err("writer-fence guardian did not release native locks".to_owned());
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

pub fn run_writer_fence_guardian(request_path: &Path) -> Result<(), String> {
    let request: WriterFenceGuardianRequest = serde_json::from_slice(
        &fs::read(request_path).map_err(|error| format!("{}: {error}", request_path.display()))?,
    )
    .map_err(|error| error.to_string())?;
    if request.schema != "csdlc.v3.copied_record_writer_fence_guardian_request.v1" {
        return Err("unsupported writer-fence guardian request schema".to_owned());
    }
    validate_operation_id(&request.operation_id)?;
    let common = fs::canonicalize(&request.git_common)
        .map_err(|error| format!("{}: {error}", request.git_common.display()))?;
    let operation_root = common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&request.operation_id);
    if request_path != operation_root.join("writer-fence-guardian-request.json")
        || request.fence_path != operation_root.join("conversion.fence")
        || request.ready_path != operation_root.join("writer-fence-guardian-ready.json")
    {
        return Err("writer-fence guardian paths escaped the authenticated operation".to_owned());
    }
    let guard = NativeWriterFenceGuard::acquire(&common, request.issues.iter().copied())
        .map_err(|error| format!("guardian acquire native issue writer fences: {error:?}"))?;
    let retained_port = guardian_ready(&request.ready_path, &request)
        .map(|ready| ready.challenge_port)
        .unwrap_or(0);
    let listener = UdpSocket::bind(("127.0.0.1", retained_port))
        .map_err(|error| format!("bind writer-fence guardian challenge channel: {error}"))?;
    listener
        .set_nonblocking(true)
        .map_err(|error| format!("configure writer-fence guardian challenge channel: {error}"))?;
    let challenge_port = listener
        .local_addr()
        .map_err(|error| format!("inspect writer-fence guardian challenge channel: {error}"))?
        .port();
    replace_durable(&request.fence_path, b"active\n")?;
    write_create_once(
        &request.ready_path,
        &serde_json::to_vec_pretty(&WriterFenceGuardianReady {
            schema: "csdlc.v3.copied_record_writer_fence_guardian_ready.v1".to_owned(),
            operation_id: request.operation_id.clone(),
            request_digest: request.request_digest.clone(),
            issues: request.issues.clone(),
            challenge_port,
        })
        .map_err(|error| error.to_string())?,
    )?;
    let challenge = guardian_challenge(&request);
    let response = guardian_response(&challenge);
    loop {
        match fs::read(&request.fence_path) {
            Ok(bytes) if bytes == b"released\n" => break,
            _ => {}
        }
        let mut received = [0_u8; 512];
        match listener.recv_from(&mut received) {
            Ok((size, peer)) => {
                if received[..size] == challenge {
                    let _ = listener.send_to(&response, peer);
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => {
                return Err(format!(
                    "accept writer-fence guardian challenge channel: {error}"
                ));
            }
        }
    }
    drop(guard);
    Ok(())
}

#[cfg(test)]
mod writer_fence_guardian_tests {
    use super::*;

    #[test]
    fn stale_ready_and_foreign_eight_lock_holder_fail_closed() {
        let nonce = CREATE_ONCE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let common = std::env::temp_dir().join(format!(
            "csdlc-stale-guardian-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&common).unwrap();
        let issues = vec![511, 517, 497, 3, 505, 122, 113, 868];
        let operation_id = "stale-guardian-negative";
        let operation_root = common
            .join("csdlc-v3/local/conversion-rehearsals")
            .join(operation_id);
        fs::create_dir_all(&operation_root).unwrap();
        let ready_path = operation_root.join("writer-fence-guardian-ready.json");
        let request = WriterFenceGuardianRequest {
            schema: "csdlc.v3.copied_record_writer_fence_guardian_request.v1".to_owned(),
            operation_id: operation_id.to_owned(),
            request_digest: "semantic-projection-v1:stale-guardian".to_owned(),
            git_common: common.clone(),
            issues: issues.clone(),
            fence_path: operation_root.join("conversion.fence"),
            ready_path: ready_path.clone(),
        };
        let stale_listener = UdpSocket::bind(("127.0.0.1", 0)).unwrap();
        let stale_port = stale_listener.local_addr().unwrap().port();
        drop(stale_listener);
        fs::write(
            &ready_path,
            serde_json::to_vec_pretty(&WriterFenceGuardianReady {
                schema: "csdlc.v3.copied_record_writer_fence_guardian_ready.v1".to_owned(),
                operation_id: operation_id.to_owned(),
                request_digest: request.request_digest.clone(),
                issues: issues.clone(),
                challenge_port: stale_port,
            })
            .unwrap(),
        )
        .unwrap();
        let foreign_holder = NativeWriterFenceGuard::acquire(&common, issues.iter().copied())
            .expect("foreign process fixture must hold all eight native locks");

        let rejected = retained_guardian_authenticates(&ready_path, &request, &common, &issues)
            .expect_err("stale readiness plus foreign locks must not mint admission");
        assert!(rejected.contains("stale writer-fence guardian readiness"));

        drop(foreign_holder);
        fs::remove_dir_all(common).unwrap();
    }
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
    prior_executable_path: PathBuf,
    prior_executable_bytes: Vec<u8>,
    semantic_mappings: Vec<Value>,
}

fn validate_operation_id(operation_id: &str) -> Result<(), String> {
    if operation_id.is_empty()
        || matches!(operation_id, "." | "..")
        || !operation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(
            "operation_id must be a non-dot path segment containing only ASCII letters, digits, '-', '_' or '.'"
                .to_owned(),
        );
    }
    Ok(())
}

fn canonical_topology_file(
    supplied: &Path,
    worktree: &Path,
    relative: &Path,
    label: &str,
) -> Result<PathBuf, String> {
    let expected = fs::canonicalize(worktree.join(relative))
        .map_err(|error| format!("canonical {label}: {error}"))?;
    let observed =
        fs::canonicalize(supplied).map_err(|error| format!("{}: {error}", supplied.display()))?;
    if observed != expected {
        return Err(format!(
            "{label} must authenticate the canonical registered-worktree path {}",
            expected.display()
        ));
    }
    Ok(expected)
}

fn card_value<'a>(card: &'a Value, pointer: &str) -> Option<&'a Value> {
    card.pointer(pointer)
        .or_else(|| pointer.strip_prefix('/').and_then(|field| card.get(field)))
}

fn required_card_string(card: &Value, pointer: &str, label: &str) -> Result<String, String> {
    card_value(card, pointer)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("source card lacks unambiguous {label}"))
}

fn source_card_identity(
    record: &CopiedRecord,
    cards: &BTreeMap<String, Value>,
    index_generation: u64,
    expected_repository: &str,
) -> Result<(String, String, u64), String> {
    let mut identities = BTreeSet::new();
    for kind in SEMANTIC_CARD_KINDS {
        let card = cards
            .get(kind)
            .ok_or_else(|| format!("source card denominator lacks {kind}"))?;
        let wrapped_identity = card.get("identity").filter(|value| value.is_object());
        let identity = wrapped_identity.unwrap_or(card);
        let issue = identity.get("issue").and_then(Value::as_u64);
        let repository = identity.get("repository").and_then(Value::as_str);
        let slug = identity.get("slug").and_then(Value::as_str);
        let title = identity.get("title").and_then(Value::as_str);
        let generation = wrapped_identity
            .map(|identity| identity.get("generation").and_then(Value::as_u64))
            .unwrap_or(Some(index_generation));
        if issue != Some(record.issue)
            || repository != Some(expected_repository)
            || slug.is_none_or(str::is_empty)
            || title.is_none_or(str::is_empty)
            || generation.is_none_or(|value| value == 0)
        {
            return Err(format!("{kind} card has incomplete or mismatched identity"));
        }
        identities.insert((
            repository.unwrap().to_owned(),
            slug.unwrap().to_owned(),
            title.unwrap().to_owned(),
            generation.unwrap(),
        ));
    }
    if identities.len() != 1 {
        return Err("six-card identity is ambiguous".to_owned());
    }
    let (_, slug, title, generation) = identities.into_iter().next().unwrap();
    Ok((slug, title, generation))
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
    let authority_path = canonical_topology_file(
        &request.authority_bytes_path,
        &canonical_linked_worktree,
        Path::new(crate::authority::SELECTOR_PATH),
        "authority_bytes_path",
    )?;
    let registry_path = canonical_topology_file(
        &request.registry_path,
        &canonical_linked_worktree,
        Path::new("docs/templates/prompts/current.json"),
        "registry_path",
    )?;
    let authority_bytes = fs::read(&authority_path)
        .map_err(|error| format!("{}: {error}", authority_path.display()))?;
    let authority = Digest::authority(&authority_bytes);
    let registry_bytes = fs::read(&registry_path)
        .map_err(|error| format!("{}: {error}", registry_path.display()))?;
    PromptRegistry::from_current_json(&registry_bytes)
        .map_err(|findings| format!("invalid prompt registry: {findings:?}"))?;
    let prior_executable_path = fs::canonicalize(&request.prior_executable_path)
        .map_err(|error| format!("{}: {error}", request.prior_executable_path.display()))?;
    if !prior_executable_path.is_file() {
        return Err("prior executable path is not a regular file".to_owned());
    }
    let prior_executable_bytes = fs::read(&prior_executable_path)
        .map_err(|error| format!("{}: {error}", prior_executable_path.display()))?;
    let observed_prior_digest = blake3::hash(&prior_executable_bytes).to_hex().to_string();
    if request.prior_executable_blake3 != observed_prior_digest {
        return Err("prior executable bytes do not match declared blake3 provenance".to_owned());
    }
    SemanticRoot::from_git_common(&canonical_git_common, request.repository.clone())
        .map_err(|error| format!("invalid semantic root: {error:?}"))?;
    let writer_fence_issues = request
        .writer_fence_issues
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let mut expected_writer_fence_issues = request
        .records
        .iter()
        .map(|record| record.issue)
        .collect::<BTreeSet<_>>();
    expected_writer_fence_issues.insert(request.writer_probe_issue);
    if request.writer_probe_issue == 0
        || writer_fence_issues.is_empty()
        || writer_fence_issues.len() != request.writer_fence_issues.len()
        || writer_fence_issues.contains(&0)
        || writer_fence_issues != expected_writer_fence_issues
    {
        return Err("writer_fence_issues must exactly equal the seven-record census plus writer_probe_issue".to_owned());
    }
    let mut source_digests = Vec::with_capacity(request.records.len());
    let mut semantic_mappings = Vec::with_capacity(request.records.len());
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
        if index.get("issue").and_then(Value::as_u64) != Some(record.issue)
            || index.get("repository").and_then(Value::as_str) != Some(request.repository.as_str())
        {
            return Err(format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: source index identity mismatch",
                record.issue
            ));
        }
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
        let (inputs, mut mapping) = conversion_inputs(
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
        let card_generation = mapping
            .pointer("/source/card_generation")
            .and_then(Value::as_u64);
        if card_generation != Some(generation) {
            return Err(format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: card generation does not match index generation",
                record.issue
            ));
        }
        if record.role == "pending_recovery" && !record.source.join("recovery-evidence").is_dir() {
            return Err(format!(
                "issue {} disposition=unsupported_ambiguous_incomplete_source: pending-recovery record lacks recovery evidence",
                record.issue
            ));
        }
        mapping["source"]["source_digest"] = json!(digest.as_str());
        mapping["destination"]["inputs_digest"] =
            json!(Digest::semantic_projection(&canonical_json_bytes(
                &serde_json::to_value(&inputs).map_err(|error| error.to_string())?
            )?)
            .as_str());
        semantic_mappings.push(mapping);
        source_digests.push(digest);
    }
    Ok(ConversionPreflight {
        canonical_git_common,
        canonical_linked_worktree,
        source_digests,
        authority_bytes,
        registry_bytes,
        prior_executable_bytes,
        prior_executable_path,
        semantic_mappings,
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
        "prior_executable_blake3": blake3::hash(&preflight.prior_executable_bytes).to_hex().to_string(),
        "prior_executable_path": preflight.prior_executable_path,
        "semantic_mappings": preflight.semantic_mappings,
        "writer_fence_issues": request.writer_fence_issues,
        "writer_probe_issue": request.writer_probe_issue,
    });
    let bytes = serde_json::to_vec(&identity).map_err(|error| error.to_string())?;
    Ok(Digest::semantic_projection(&bytes))
}

fn wait_for_writer_fence_probe(
    operation: &Operation<'_>,
    request_digest: &Digest,
    acknowledgement_name: &str,
    checkpoint: &str,
    completed_event: &str,
) -> Result<(), String> {
    let acknowledgement = operation.root.join(acknowledgement_name);
    let deadline = Instant::now() + Duration::from_secs(30);
    let retained = loop {
        while !acknowledgement.is_file() {
            if Instant::now() >= deadline {
                return Err(format!(
                    "writer fence probe timed out waiting for {}",
                    acknowledgement.display()
                ));
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let bytes = fs::read(&acknowledgement)
            .map_err(|error| format!("{}: {error}", acknowledgement.display()))?;
        match serde_json::from_slice::<Value>(&bytes) {
            Ok(value) => break value,
            Err(error) if error.is_eof() && Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) if error.is_eof() => {
                return Err(format!(
                    "writer fence probe timed out waiting for complete acknowledgement {}",
                    acknowledgement.display()
                ));
            }
            Err(error) => return Err(format!("{}: {error}", acknowledgement.display())),
        }
    };
    if retained.get("schema").and_then(Value::as_str)
        != Some("csdlc.v3.copied_record_writer_fence_probe_ack.v1")
        || retained.get("operation_id").and_then(Value::as_str) != Some(operation.id())
        || retained.get("request_digest").and_then(Value::as_str) != Some(request_digest.as_str())
        || retained.get("checkpoint").and_then(Value::as_str) != Some(checkpoint)
    {
        return Err(format!(
            "writer fence probe acknowledgement {} does not authenticate the current operation/request checkpoint",
            acknowledgement.display()
        ));
    }
    operation.append(
        completed_event,
        json!({
            "acknowledgement": acknowledgement,
            "checkpoint": checkpoint,
            "request_digest": request_digest.as_str(),
        }),
    )
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
        let card = read_json(&values)?;
        let wrapped = card.pointer("/content/values").is_some();
        let declared_kind = if wrapped {
            card.pointer("/content/card_kind")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    format!("wrapped {kind} card lacks its required content.card_kind declaration")
                })?
        } else {
            card.get("card")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| format!("flat {kind} card lacks its required card declaration"))?
        };
        if declared_kind != kind {
            return Err(format!(
                "declared card kind {declared_kind} does not match {kind} values filename"
            ));
        }
        cards.insert(kind.to_owned(), card);
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

pub(crate) fn copied_card_projection(
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

pub(crate) fn source_digest(root: &Path) -> Result<Digest, String> {
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
        ("bound_dirty", "bound") => Ok(LifecycleState::Bound),
        ("implemented", "implemented") => Ok(LifecycleState::Implemented),
        ("reviewed", "reviewed") => Ok(LifecycleState::Reviewed),
        ("published", "published") => Ok(LifecycleState::Published),
        ("terminal", "closed_out" | "terminal") => Ok(LifecycleState::ClosedOut),
        ("pending_recovery", "bound") => Ok(LifecycleState::Bound),
        ("pending_recovery", "implemented") => Ok(LifecycleState::Implemented),
        ("pending_recovery", "reviewed") => Ok(LifecycleState::Reviewed),
        ("pending_recovery", "published") => Ok(LifecycleState::Published),
        _ => Err(format!("unsupported source role/phase {role}/{declared}")),
    }
}

fn source_values(card: &Value) -> &Value {
    card.pointer("/content/values").unwrap_or(card)
}

pub(crate) fn source_plan(cards: &BTreeMap<String, Value>) -> Result<Vec<PlanStep>, String> {
    let spp = source_values(
        cards
            .get("spp")
            .ok_or_else(|| "source lacks SPP values".to_owned())?,
    );
    if let Some(steps) = spp.get("steps").and_then(Value::as_array) {
        if steps.is_empty() {
            return Err("source SPP has an empty plan".to_owned());
        }
        return steps
            .iter()
            .map(|step| {
                Ok(PlanStep {
                    id: step
                        .get("id")
                        .and_then(Value::as_str)
                        .filter(|value| !value.trim().is_empty())
                        .ok_or_else(|| "source SPP step lacks id".to_owned())?
                        .to_owned(),
                    acceptance: step
                        .get("action")
                        .and_then(Value::as_str)
                        .filter(|value| !value.trim().is_empty())
                        .ok_or_else(|| "source SPP step lacks action".to_owned())?
                        .to_owned(),
                })
            })
            .collect();
    }
    [
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
    ]
    .into_iter()
    .map(|(id, fields)| {
        let acceptance = fields
            .into_iter()
            .map(|field| required_card_string(spp, &format!("/{field}"), field))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n");
        Ok(PlanStep {
            id: id.to_owned(),
            acceptance,
        })
    })
    .collect()
}

fn source_validators(cards: &BTreeMap<String, Value>) -> Result<Vec<Validator>, String> {
    let vpp = source_values(
        cards
            .get("vpp")
            .ok_or_else(|| "source lacks VPP values".to_owned())?,
    );
    let Some(lanes) = vpp.get("lanes").and_then(Value::as_array) else {
        // Some retained 1.0.4 cards contain only human validation text. Preserve
        // that text in the card bytes; do not invent an executable validator.
        return Ok(Vec::new());
    };
    lanes
        .iter()
        .map(|lane| {
            let argv = lane
                .get("argv")
                .and_then(Value::as_array)
                .filter(|values| !values.is_empty())
                .ok_or_else(|| "source validation lane lacks argv".to_owned())?;
            let command = argv
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .filter(|value| !value.is_empty())
                        .map(str::to_owned)
                        .ok_or_else(|| "source validation argv is ambiguous".to_owned())
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Validator {
                id: lane
                    .get("lane")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| "source validation lane lacks identity".to_owned())?
                    .to_owned(),
                program: command[0].clone(),
                args: command[1..].to_vec(),
                success_marker: String::new(),
                timeout_seconds: lane
                    .get("budget_seconds")
                    .and_then(Value::as_u64)
                    .filter(|value| *value > 0)
                    .ok_or_else(|| "source validation lane lacks positive budget".to_owned())?,
            })
        })
        .collect()
}

fn conversion_inputs(
    request: &ConversionRequest,
    record: &CopiedRecord,
    index: &Value,
    cards: BTreeMap<String, Value>,
    canonical_linked_worktree: &Path,
    authority: Digest,
) -> Result<(IssueInputs, Value), String> {
    let index_generation = index
        .get("generation")
        .and_then(Value::as_u64)
        .filter(|value| *value > 0)
        .ok_or_else(|| "source index lacks positive generation".to_owned())?;
    let (slug, title, card_generation) =
        source_card_identity(record, &cards, index_generation, &request.repository)?;
    let publication_source = index.get("publication").cloned().unwrap_or(Value::Null);
    let publication = Publication {
        base: publication_source
            .get("base")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        title: title.clone(),
        body: if publication_source.is_null() {
            String::new()
        } else {
            String::from_utf8(canonical_json_bytes(&publication_source)?)
                .map_err(|error| error.to_string())?
        },
        draft: publication_source
            .get("draft")
            .and_then(Value::as_bool)
            .unwrap_or(true),
    };
    let plan = source_plan(&cards)?;
    let validators = source_validators(&cards)?;
    let accepted = AcceptedIntentPlan {
        schema: "csdlc.v3.intent_plan.v1".to_owned(),
        slug,
        cards,
        validators,
        publication,
    };
    let binding = if record.role == "prepared" {
        None
    } else {
        let source_branch = index
            .get("branch")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "source index lacks branch identity".to_owned())?;
        let source_worktree = index
            .get("worktree")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| "source index lacks worktree identity".to_owned())?;
        let binding_path = record.source.join("binding.json");
        if binding_path.is_file() {
            let retained = read_json(&binding_path)?;
            if retained.get("branch").and_then(Value::as_str) != Some(source_branch)
                || retained.get("worktree").and_then(Value::as_str) != Some(source_worktree)
            {
                return Err("source binding record disagrees with source index".to_owned());
            }
        }
        Some(Binding {
            branch: request.linked_branch.clone(),
            head: request.linked_head.clone(),
            worktree: canonical_linked_worktree.to_owned(),
            registration: "git-worktree-list".to_owned(),
        })
    };
    let inputs = IssueInputs::new(title, accepted, plan, binding, authority)
        .map_err(|error| format!("{error:?}"))?;
    let mapping = json!({
        "schema":"csdlc.v3.copied_record_semantic_equivalence.v1",
        "issue":record.issue,
        "role":record.role,
        "source":{
            "index_generation":index.get("generation"),
            "card_generation":card_generation,
            "phase":index.get("phase"),
            "branch":index.get("branch"),
            "worktree":index.get("worktree"),
            "publication":publication_source,
            "review":index.get("review"),
            "terminal":index.get("terminal"),
            "migration":index.get("migration"),
            "cards_digest":Digest::semantic_projection(&canonical_json_bytes(
                &serde_json::to_value(inputs.cards()).map_err(|error| error.to_string())?
            )?).as_str(),
        },
        "destination":{
            "intent":inputs.intent(),
            "slug":inputs.slug(),
            "plan":inputs.plan(),
            "validators":inputs.validation(),
            "publication":inputs.publication(),
            "binding":inputs.binding(),
        }
    });
    Ok((inputs, mapping))
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
    let native_issue_writer_fences = acquire_guarded_writer_fence(
        &operation,
        &request_digest,
        &preflight.canonical_git_common,
        &request.writer_fence_issues,
    )?;
    let fence_paths = native_issue_writer_fences.paths().to_vec();
    let mut native_issue_writer_fences = Some(native_issue_writer_fences);
    operation.marker(
        "writer-fence-held.json",
        "native_issue_writer_fence_held",
        json!({
            "lock_contract":"native-local-state-root/locks/<issue>.lock",
            "issues":request.writer_fence_issues,
            "paths":&fence_paths,
            "checkpoint":"during_conversion",
            "request_digest":request_digest.as_str(),
        }),
    )?;
    if request.writer_fence_probe {
        wait_for_writer_fence_probe(
            &operation,
            &request_digest,
            "writer-fence-probe-complete",
            "during_conversion",
            "writer_fence_probe_completed",
        )?;
    }
    operation.fault("conversion_intent_durability", "before")?;
    operation.marker(
        "checkpoints/conversion-intent.json",
        "conversion_intent_durable",
        json!({"repository": request.repository, "record_count": request.records.len()}),
    )?;
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
    let prior_executable_bytes = preflight.prior_executable_bytes;
    let semantic_mappings = preflight.semantic_mappings;
    let registry = PromptRegistry::from_current_json(&registry_bytes)
        .map_err(|findings| format!("invalid prompt registry: {findings:?}"))?;
    let canonical_git_common = preflight.canonical_git_common;
    let canonical_linked_worktree = preflight.canonical_linked_worktree;
    let root = SemanticRoot::from_git_common(&canonical_git_common, request.repository.clone())
        .map_err(|error| format!("{error:?}"))?;
    let mut converted = Vec::new();
    for (record_index, record) in request.records.iter().enumerate() {
        let staged_source = staging.join(record.issue.to_string());
        let index = read_json(&staged_source.join("index.json"))?;
        let cards = load_cards(&staged_source)?;
        let (inputs, observed_mapping) = conversion_inputs(
            request,
            record,
            &index,
            cards,
            &canonical_linked_worktree,
            authority.clone(),
        )?;
        let mut observed_mapping = observed_mapping;
        observed_mapping["source"]["source_digest"] =
            json!(source_digest(&staged_source)?.as_str());
        observed_mapping["destination"]["inputs_digest"] =
            json!(Digest::semantic_projection(&canonical_json_bytes(
                &serde_json::to_value(&inputs).map_err(|error| error.to_string())?
            )?)
            .as_str());
        if observed_mapping != semantic_mappings[record_index] {
            return Err(format!(
                "issue {} staged semantic mapping differs from admitted source mapping",
                record.issue
            ));
        }
        let source_generation = index
            .get("generation")
            .and_then(Value::as_u64)
            .filter(|generation| *generation > 0)
            .ok_or_else(|| format!("{} lacks a valid generation", record.source.display()))?;
        let key = IssueKey::new(request.repository.clone(), record.issue)
            .map_err(|error| format!("{error:?}"))?;
        let phase = source_phase(&record.role, &index)?;
        operation.fault("semantic_state_activation", "before")?;
        let current = match DurableTransactionStore::observe_issue_under_native_writer_fence(
            &root,
            &key,
            native_issue_writer_fences
                .as_ref()
                .expect("writer fences held through semantic observation"),
        )
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
                DurableTransactionStore::convert_copied_issue_under_native_writer_fence(
                    &root,
                    CopiedRecordConversion {
                        key: key.clone(),
                        inputs,
                        phase,
                        source_generation,
                        source_digest: source_digest(&staged_source)?,
                    },
                    native_issue_writer_fences
                        .as_ref()
                        .expect("writer fences held through semantic activation"),
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
                "semantic_equivalence": semantic_mappings[record_index],
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
        let projection_healthy =
            crate::storage::semantic::ProjectionWriteProof::verify(&root, &current).is_ok()
                && DurableTransactionStore::observe_card_projection(&root, &current, &cards)
                    .is_ok_and(|observation| observation == CardProjectionObservation::Healthy);
        if !projection_healthy {
            DurableTransactionStore::write_card_projection(&root, &current, cards)
                .map_err(|error| format!("card projection {}: {error:?}", record.role))?;
            let regenerated = copied_card_projection(&current, &registry, &staged_source)?;
            if regenerated.projection_digest() != &expected_card_projection
                || DurableTransactionStore::observe_card_projection(&root, &current, &regenerated)
                    .map_err(|error| format!("projection readback: {error:?}"))?
                    != CardProjectionObservation::Healthy
            {
                return Err(format!(
                    "projection for {} failed exact readback",
                    record.role
                ));
            }
        }
        if !retained_projection {
            if current.inputs().binding().is_some() {
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
        write_create_once(&prior_executable, &prior_executable_bytes)?;
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
    replace_durable(&executable_slot.join("active"), &prior_executable_bytes)?;
    operation.marker(
        "restore/prior-executable-restored.json",
        "prior_executable_restored",
        json!({
            "restored_size": prior_executable_bytes.len(),
            "restored_blake3": blake3::hash(&prior_executable_bytes).to_hex().to_string(),
        }),
    )?;
    operation.fault("prior_executable_restoration", "after")?;

    operation.fault("restore_receipt_persistence_and_fence_release", "before")?;
    operation.marker(
        "restore/receipt.json",
        "restore_receipt_persisted",
        json!({"status": "restored"}),
    )?;
    if request.writer_fence_probe {
        operation.marker(
            "writer-fence-post-activation-held.json",
            "native_issue_writer_fence_post_activation_held",
            json!({
                "lock_contract":"native-local-state-root/locks/<issue>.lock",
                "issues":request.writer_fence_issues,
                "paths":&fence_paths,
                "checkpoint":"post_activation",
                "request_digest":request_digest.as_str(),
            }),
        )?;
        wait_for_writer_fence_probe(
            &operation,
            &request_digest,
            "writer-fence-post-activation-probe-complete",
            "post_activation",
            "writer_fence_post_activation_probe_completed",
        )?;
    }
    drop(native_issue_writer_fences.take());
    release_guarded_writer_fence(
        &operation,
        &canonical_git_common,
        &request.writer_fence_issues,
    )?;
    operation.append(
        "conversion_fence_released",
        json!({"native_issue_writer_fences_released":true}),
    )?;
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
                "projection_required":crate::storage::semantic::ProjectionWriteProof::verify(&root, &snapshot).is_err(),
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
    if request.issue == 0 {
        return Err("invalid relocation issue or operation_id".to_owned());
    }
    validate_operation_id(&request.operation_id)?;

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
    let source_index = read_json(&source_local.join("index.json"))?;
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
    if source_snapshot.key().issue() != request.issue
        || source_snapshot.key().repository() != request.repository
        || source_index.get("issue").and_then(Value::as_u64) != Some(request.issue)
        || source_index.get("repository").and_then(Value::as_str)
            != Some(request.repository.as_str())
    {
        return Err("source local and semantic issue identities disagree".to_owned());
    }
    if source_local_cards != source_cards {
        return Err("source local card values disagree with semantic snapshot cards".to_owned());
    }
    let semantic_phase = match source_snapshot.phase() {
        LifecycleState::Ready => "ready",
        LifecycleState::Bound => "bound",
        LifecycleState::Implemented => "implemented",
        LifecycleState::Reviewed => "reviewed",
        LifecycleState::Published => "published",
        LifecycleState::ClosedOut => "closed_out",
        other => return Err(format!("unsupported source semantic phase {other:?}")),
    };
    if source_index.get("phase").and_then(Value::as_str) != Some(semantic_phase) {
        return Err(
            "source local lifecycle phase disagrees with semantic snapshot state".to_owned(),
        );
    }
    match source_snapshot.inputs().binding() {
        None if source_local.join("binding.json").exists() => {
            return Err("source local binding exists but semantic binding is absent".to_owned())
        }
        Some(binding) => {
            let local_binding = read_json(&source_local.join("binding.json"))?;
            if local_binding.get("issue").and_then(Value::as_u64) != Some(request.issue)
                || local_binding.get("branch").and_then(Value::as_str)
                    != Some(binding.branch.as_str())
                || local_binding.get("worktree").and_then(Value::as_str)
                    != Some(binding.worktree.to_string_lossy().as_ref())
                || source_index.get("branch").and_then(Value::as_str)
                    != Some(binding.branch.as_str())
                || source_index.get("worktree").and_then(Value::as_str)
                    != Some(binding.worktree.to_string_lossy().as_ref())
            {
                return Err(
                    "source local binding disagrees with semantic snapshot binding".to_owned(),
                );
            }
        }
        None => {}
    }
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
    let canonical_registry = canonical_topology_file(
        &request.registry_path,
        &target_worktree,
        Path::new("docs/templates/prompts/current.json"),
        "registry_path",
    )?;
    let (registry, registry_bytes) = absolute_registry(&target_worktree, &canonical_registry)?;
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
    if operation_root.parent() != Some(relocations.as_path())
        || operation_root.file_name() != Some(request.operation_id.as_ref())
    {
        return Err("relocation operation root escaped its canonical parent".to_owned());
    }
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
    let current = match DurableTransactionStore::observe_issue(&target_root, &key)
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
    let projection_healthy =
        DurableTransactionStore::observe_card_projection(&target_root, &current, &bundle)
            .is_ok_and(|observation| observation == CardProjectionObservation::Healthy);
    if !projection_healthy {
        DurableTransactionStore::write_card_projection(&target_root, &current, bundle)
            .map_err(|error| format!("target card projection: {error:?}"))?;
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

fn retained_operation(
    request: &ConversionRequest,
) -> Result<(ConversionPreflight, Digest, PathBuf, PathBuf), String> {
    if request.schema != "csdlc.v3.copied_record_conversion.v1" {
        return Err("unsupported conversion request schema".to_owned());
    }
    if request.operation_id.is_empty()
        || !request
            .operation_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err("operation evidence requires an explicit valid operation_id".to_owned());
    }
    let preflight = preflight_conversion(request)?;
    let digest = canonical_request_digest(request, &preflight)?;
    let root = preflight
        .canonical_git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&request.operation_id);
    let journal = root.join("journal.jsonl");
    let bytes = fs::read(&journal).map_err(|error| format!("{}: {error}", journal.display()))?;
    let retained = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .find_map(|line| serde_json::from_slice::<Value>(line).ok())
        .and_then(|event| {
            event
                .pointer("/detail/request_digest")
                .and_then(Value::as_str)
                .map(str::to_owned)
        });
    if retained.as_deref() != Some(digest.as_str()) {
        return Err("retained operation request digest mismatch".to_owned());
    }
    Ok((preflight, digest, root, journal))
}

pub fn inspect_conversion_operation(
    request: &ConversionRequest,
) -> Result<ConversionOperationEvidence, String> {
    let (preflight, request_digest, root, journal_path) = retained_operation(request)?;
    let journal_bytes =
        fs::read(&journal_path).map_err(|error| format!("{}: {error}", journal_path.display()))?;
    let events = journal_bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice::<Value>(line).map_err(|error| error.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let fault = events.iter().find_map(|event| {
        let name = event.get("event").and_then(Value::as_str)?;
        let suffix = name.strip_prefix("fault_injected:")?;
        let (point, boundary) = suffix.rsplit_once(':')?;
        Some((point.to_owned(), boundary.to_owned()))
    });
    let completed = events
        .iter()
        .any(|event| event.get("event").and_then(Value::as_str) == Some("operation_completed"));

    let checkpoints = root.join("checkpoints");
    let mut semantic_checkpoints = Vec::new();
    if checkpoints.is_dir() {
        let mut entries = fs::read_dir(&checkpoints)
            .map_err(|error| format!("{}: {error}", checkpoints.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with("semantic-activation-")
            {
                semantic_checkpoints.push(entry.path());
            }
        }
    }
    // Detect an activation that reached durable semantic state before its checkpoint.
    let mut semantic_effects = Vec::new();
    for record in &request.records {
        let path = preflight
            .canonical_git_common
            .join("csdlc-v3/semantic/issues")
            .join(record.issue.to_string());
        if path.exists() {
            semantic_effects.push(path);
        }
    }
    if semantic_effects.is_empty() {
        semantic_effects = semantic_checkpoints;
    }

    let ledger = root.join("fake-transport-ledger.jsonl");
    let remote_lines = if ledger.is_file() {
        fs::read(&ledger)
            .map_err(|error| format!("{}: {error}", ledger.display()))?
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).map_err(|error| error.to_string()))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    let remote_operation_identity = remote_lines
        .first()
        .and_then(|line| line.get("operation_id"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    if remote_operation_identity
        .as_deref()
        .is_some_and(|identity| identity != request.operation_id)
    {
        return Err("remote effect operation identity mismatch".to_owned());
    }
    let remote_readback = root.join("remote/readback.json");
    let remote_reconcile_count = usize::from(remote_readback.is_file());
    let remote_state = match (remote_lines.is_empty(), remote_readback.is_file()) {
        (true, false) => "not_dispatched",
        (false, false) => "uncertain",
        (false, true) => "reconciled",
        (true, true) => "invalid_readback_without_effect",
    }
    .to_owned();
    if remote_state == "invalid_readback_without_effect" {
        return Err("remote readback exists without an authenticated effect".to_owned());
    }
    let mut effect_paths = semantic_effects.clone();
    if ledger.is_file() {
        effect_paths.push(ledger.clone());
    }
    let mut readback_paths = Vec::new();
    if remote_readback.is_file() {
        readback_paths.push(remote_readback);
    }
    Ok(ConversionOperationEvidence {
        schema: "csdlc.v3.copied_record_conversion_operation_evidence.v1".to_owned(),
        operation_id: request.operation_id.clone(),
        request_digest: request_digest.as_str().to_owned(),
        outcome: if completed {
            "completed"
        } else {
            "interrupted"
        }
        .to_owned(),
        abrupt_fault_point: fault.as_ref().map(|value| value.0.clone()),
        abrupt_fault_boundary: fault.map(|value| value.1),
        journal_path,
        journal_event_count: events.len(),
        semantic_effect_count: semantic_effects.len(),
        remote_effect_count: remote_lines.len(),
        remote_operation_identity,
        remote_state,
        remote_reconcile_count,
        effect_paths,
        readback_paths,
    })
}

pub fn restore_conversion_pre_effect(
    request: &ConversionRequest,
) -> Result<ConversionRestoreResult, String> {
    let (preflight, request_digest, _, _) = retained_operation(request)?;
    let evidence = inspect_conversion_operation(request)?;
    let effect_count = evidence.semantic_effect_count + evidence.remote_effect_count;
    if effect_count > 0 {
        return Ok(ConversionRestoreResult {
            schema: "csdlc.v3.copied_record_conversion_restore_result.v1".to_owned(),
            operation_id: request.operation_id.clone(),
            status: "refused_post_effect".to_owned(),
            allowed: false,
            request_digest: request_digest.as_str().to_owned(),
            source_record_count: request.records.len(),
            source_hashes_before: preflight
                .source_digests
                .iter()
                .map(|digest| digest.as_str().to_owned())
                .collect(),
            source_hashes_after: Vec::new(),
            effect_count,
            receipt_path: None,
        });
    }
    let after = request
        .records
        .iter()
        .map(|record| source_digest(&record.source))
        .collect::<Result<Vec<_>, _>>()?;
    if after != preflight.source_digests {
        return Err("source census changed before pre-effect restore".to_owned());
    }
    let operation = Operation::open(request, &request_digest, &preflight.canonical_git_common)?;
    let fence = operation.root.join("conversion.fence");
    if fence.is_file() {
        release_guarded_writer_fence(
            &operation,
            &preflight.canonical_git_common,
            &request.writer_fence_issues,
        )?;
    }
    let receipt_path = operation.root.join("restore/pre-effect-receipt.json");
    operation.marker(
        "restore/pre-effect-receipt.json",
        "pre_effect_restore_completed",
        json!({
            "source_record_count": request.records.len(),
            "source_hashes": after.iter().map(Digest::as_str).collect::<Vec<_>>(),
        }),
    )?;
    Ok(ConversionRestoreResult {
        schema: "csdlc.v3.copied_record_conversion_restore_result.v1".to_owned(),
        operation_id: request.operation_id.clone(),
        status: "restored_pre_effect".to_owned(),
        allowed: true,
        request_digest: request_digest.as_str().to_owned(),
        source_record_count: request.records.len(),
        source_hashes_before: preflight
            .source_digests
            .iter()
            .map(|digest| digest.as_str().to_owned())
            .collect(),
        source_hashes_after: after
            .iter()
            .map(|digest| digest.as_str().to_owned())
            .collect(),
        effect_count,
        receipt_path: Some(receipt_path),
    })
}

#[cfg(test)]
mod tests {
    use super::write_create_once;
    use std::fs;
    use std::sync::{Arc, Barrier};
    use std::thread;

    fn scratch(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "csdlc-conversion-{name}-{}-{}",
            std::process::id(),
            super::CREATE_ONCE_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        root
    }

    #[test]
    fn interrupted_create_once_staging_never_publishes_partial_final_bytes() {
        let root = scratch("interrupted-checkpoint");
        let final_path = root.join("checkpoint.json");
        let orphan = root.join(".checkpoint.json.interrupted.create-once");
        fs::write(&orphan, br#"{"schema":"partial"#).unwrap();

        assert!(!final_path.exists());
        let complete = br#"{"schema":"complete","status":"ready"}"#;
        write_create_once(&final_path, complete).unwrap();

        assert_eq!(fs::read(&final_path).unwrap(), complete);
        assert_eq!(fs::read(&orphan).unwrap(), br#"{"schema":"partial"#);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn concurrent_create_once_publishes_one_complete_candidate() {
        let root = scratch("concurrent-checkpoint");
        let final_path = root.join("checkpoint.json");
        let first = br#"{"writer":"first","padding":"aaaaaaaaaaaaaaaa"}"#.to_vec();
        let second = br#"{"writer":"second","padding":"bbbbbbbbbbbbbbbb"}"#.to_vec();
        let barrier = Arc::new(Barrier::new(3));

        let handles = [first.clone(), second.clone()].map(|candidate| {
            let path = final_path.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                write_create_once(&path, &candidate)
            })
        });
        barrier.wait();
        let results = handles.map(|handle| handle.join().unwrap());

        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        let retained = fs::read(&final_path).unwrap();
        assert!(retained == first || retained == second);
        fs::remove_dir_all(root).unwrap();
    }
}
