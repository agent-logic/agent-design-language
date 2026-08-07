use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const REQUEST_SCHEMA: &str = "adl.remote_validation.request.v1";
pub const RESULT_SCHEMA: &str = "adl.remote_validation.result.v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AdapterKind {
    Local,
    Nessus,
    Aws,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FallbackPolicy {
    Disabled,
    OfferLocal,
    RunLocal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandProfile {
    pub argv: Vec<String>,
    pub working_directory: String,
    pub environment_allowlist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceBudget {
    pub cpu_cores: u16,
    pub memory_mib: u64,
    pub timeout_seconds: u64,
    pub estimated_max_cost_microusd: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactPolicy {
    pub paths: Vec<String>,
    pub required: bool,
    pub max_total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortableRequest {
    pub schema: String,
    pub request_id: String,
    pub checkout: String,
    pub revision: String,
    #[serde(default)]
    pub source_ref: Option<String>,
    pub command_profile: CommandProfile,
    pub command_profile_digest: String,
    pub adapter: AdapterKind,
    pub requested_platform: String,
    pub resource_budget: ResourceBudget,
    pub artifact_policy: ArtifactPolicy,
    pub cancellation_file: Option<String>,
    pub fallback: FallbackPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactDigest {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformRecord {
    pub os: String,
    pub architecture: String,
    pub native: bool,
    pub qualification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunOutcome {
    Passed,
    Failed,
    TimedOut,
    Cancelled,
    ProviderUnavailable,
    MalformedResult,
    StaleRevision,
    CleanupIncomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CleanupStatus {
    pub attempted: bool,
    pub complete: bool,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FallbackStatus {
    pub policy: FallbackPolicy,
    pub offered: bool,
    pub ran: bool,
    pub local_profile_digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortableResult {
    pub schema: String,
    pub request_id: String,
    pub adapter: AdapterKind,
    pub platform: PlatformRecord,
    pub revision: String,
    pub command_profile_digest: String,
    pub resource_budget: ResourceBudget,
    pub artifact_policy: ArtifactPolicy,
    pub cancellation_file: Option<String>,
    pub started_unix_ms: u128,
    pub finished_unix_ms: u128,
    pub exit_code: Option<i32>,
    pub outcome: RunOutcome,
    pub artifact_digests: Vec<ArtifactDigest>,
    pub redaction_passed: bool,
    pub cleanup: CleanupStatus,
    pub fallback: FallbackStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterExecutionReceipt {
    pub schema: String,
    pub adapter: AdapterKind,
    pub platform: PlatformRecord,
    pub revision: String,
    pub started_unix_ms: u128,
    pub finished_unix_ms: u128,
    pub exit_code: Option<i32>,
    pub outcome: RunOutcome,
    pub redaction_passed: bool,
    pub cleanup: CleanupStatus,
    pub fallback: FallbackStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterPlan {
    pub adapter: AdapterKind,
    pub revision: String,
    pub source_ref: Option<String>,
    pub command_profile_digest: String,
    pub shell_command: String,
    pub resource_budget: ResourceBudget,
    pub artifact_policy: ArtifactPolicy,
    pub cancellation_file: Option<String>,
    pub fallback: FallbackPolicy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFailure {
    Unreachable,
    Authentication,
    Capacity,
    MalformedResult,
    StaleRevision,
    CleanupIncomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FallbackDecision {
    pub allowed: bool,
    pub run_local: bool,
    pub reason: String,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn command_profile_digest(profile: &CommandProfile) -> Result<String, String> {
    serde_json::to_vec(profile)
        .map(|bytes| sha256_hex(&bytes))
        .map_err(|error| format!("command profile serialization failed: {error}"))
}

fn valid_revision(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn valid_source_ref(value: &str) -> bool {
    (value.starts_with("refs/heads/") || value.starts_with("refs/tags/"))
        && !value.contains("..")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
}

fn repo_relative(value: &str, allow_dot: bool) -> bool {
    if value.is_empty() || Path::new(value).is_absolute() {
        return false;
    }
    if allow_dot && value == "." {
        return true;
    }
    Path::new(value)
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
}

fn safe_environment_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
        && ![
            "TOKEN",
            "SECRET",
            "PASSWORD",
            "PRIVATE",
            "CREDENTIAL",
            "ACCESS_KEY",
        ]
        .iter()
        .any(|marker| value.contains(marker))
}

pub fn validate_request(request: &PortableRequest) -> Result<(), String> {
    if request.schema != REQUEST_SCHEMA {
        return Err("unsupported request schema".into());
    }
    if request.request_id.is_empty()
        || !request
            .request_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err("request_id must be non-empty and portable".into());
    }
    if !repo_relative(&request.checkout, true)
        || !repo_relative(&request.command_profile.working_directory, true)
    {
        return Err("checkout and working_directory must be repository-relative".into());
    }
    if !valid_revision(&request.revision) {
        return Err("revision must be an exact lowercase 40-hex commit".into());
    }
    match (request.adapter, request.source_ref.as_deref()) {
        (AdapterKind::Local, None) => {}
        (AdapterKind::Local, Some(_)) => {
            return Err("local execution must not declare a remote source ref".into())
        }
        (_, Some(value)) if valid_source_ref(value) => {}
        _ => return Err("remote execution requires a safe advertised source ref".into()),
    }
    if request.command_profile.argv.is_empty()
        || request
            .command_profile
            .argv
            .iter()
            .any(|value| value.is_empty())
    {
        return Err("command profile argv must be non-empty".into());
    }
    let unique_env: BTreeSet<_> = request
        .command_profile
        .environment_allowlist
        .iter()
        .collect();
    if unique_env.len() != request.command_profile.environment_allowlist.len()
        || request
            .command_profile
            .environment_allowlist
            .iter()
            .any(|value| !safe_environment_name(value))
    {
        return Err(
            "environment allowlist contains duplicate, invalid, or secret-bearing names".into(),
        );
    }
    if command_profile_digest(&request.command_profile)? != request.command_profile_digest {
        return Err("command profile digest mismatch".into());
    }
    if !matches!(
        request.requested_platform.as_str(),
        "linux" | "macos" | "windows"
    ) || request.resource_budget.cpu_cores == 0
        || request.resource_budget.memory_mib == 0
        || request.resource_budget.timeout_seconds == 0
        || request.artifact_policy.max_total_bytes == 0
    {
        return Err("platform and resource/artifact budgets must be bounded".into());
    }
    if request
        .artifact_policy
        .paths
        .iter()
        .any(|path| !repo_relative(path, false))
        || request
            .cancellation_file
            .as_deref()
            .is_some_and(|path| !repo_relative(path, false))
    {
        return Err("artifact and cancellation paths must be repository-relative".into());
    }
    if request.adapter == AdapterKind::Local && request.fallback != FallbackPolicy::Disabled {
        return Err("local execution cannot declare a second local fallback".into());
    }
    if request.adapter == AdapterKind::Aws
        && !matches!(
            request.resource_budget.estimated_max_cost_microusd,
            Some(value) if value > 0
        )
    {
        return Err("AWS execution requires a nonzero estimated cost ceiling".into());
    }
    Ok(())
}

pub fn select_adapter(
    request: &PortableRequest,
    available: &[AdapterKind],
) -> Result<AdapterKind, String> {
    validate_request(request)?;
    let unique: BTreeSet<_> = available.iter().copied().collect();
    if unique.len() != available.len() {
        return Err("adapter availability is ambiguous".into());
    }
    if unique.contains(&request.adapter) {
        return Ok(request.adapter);
    }
    Err(format!(
        "requested adapter {:?} is unavailable",
        request.adapter
    ))
}

pub fn shell_join(argv: &[String]) -> Result<String, String> {
    if argv.is_empty()
        || argv
            .iter()
            .any(|value| value.contains('\0') || value.contains('\n'))
    {
        return Err("command argv cannot be empty or contain control separators".into());
    }
    Ok(argv
        .iter()
        .map(|value| format!("'{}'", value.replace('\'', "'\"'\"'")))
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn adapter_plan(
    request: &PortableRequest,
    adapter: AdapterKind,
) -> Result<AdapterPlan, String> {
    validate_request(request)?;
    if request.adapter != adapter {
        return Err("adapter plan does not match the selected request adapter".into());
    }
    Ok(AdapterPlan {
        adapter,
        revision: request.revision.clone(),
        source_ref: request.source_ref.clone(),
        command_profile_digest: request.command_profile_digest.clone(),
        shell_command: shell_join(&request.command_profile.argv)?,
        resource_budget: request.resource_budget.clone(),
        artifact_policy: request.artifact_policy.clone(),
        cancellation_file: request.cancellation_file.clone(),
        fallback: request.fallback,
    })
}

pub fn fallback_decision(
    request: &PortableRequest,
    failure: ProviderFailure,
    local_profile_digest: &str,
) -> FallbackDecision {
    let safe_failure = matches!(
        failure,
        ProviderFailure::Unreachable | ProviderFailure::Capacity | ProviderFailure::Authentication
    );
    let profile_matches = local_profile_digest == request.command_profile_digest;
    let allowed = request.adapter != AdapterKind::Local
        && request.fallback != FallbackPolicy::Disabled
        && safe_failure
        && profile_matches;
    FallbackDecision {
        allowed,
        run_local: allowed && request.fallback == FallbackPolicy::RunLocal,
        reason: if !safe_failure {
            "fallback cannot hide malformed, stale, or incomplete-cleanup remote proof".into()
        } else if !profile_matches {
            "local fallback command profile digest differs from the remote request".into()
        } else if allowed {
            "same-profile local fallback is permitted without claiming remote proof".into()
        } else {
            "local fallback is disabled".into()
        },
    }
}

pub fn validate_result(request: &PortableRequest, result: &PortableResult) -> Result<(), String> {
    validate_request(request)?;
    if result.schema != RESULT_SCHEMA
        || result.request_id != request.request_id
        || result.adapter != request.adapter
    {
        return Err("result identity does not match request".into());
    }
    if result.revision != request.revision
        || result.command_profile_digest != request.command_profile_digest
        || result.resource_budget != request.resource_budget
        || result.artifact_policy != request.artifact_policy
        || result.cancellation_file != request.cancellation_file
    {
        return Err("result provenance or execution contract does not match request".into());
    }
    if result.platform.os != request.requested_platform
        || result.platform.architecture.trim().is_empty()
        || !matches!(result.platform.qualification.as_str(), "live" | "fixture")
        || (result.platform.qualification == "live" && !result.platform.native)
        || (result.platform.qualification == "fixture" && result.platform.native)
    {
        return Err("result platform does not match the requested qualification".into());
    }
    if result.finished_unix_ms < result.started_unix_ms {
        return Err("result timing is invalid".into());
    }
    if result.finished_unix_ms - result.started_unix_ms
        > u128::from(request.resource_budget.timeout_seconds) * 1_000
    {
        return Err("result exceeded the declared timeout".into());
    }
    if !result.redaction_passed {
        return Err("result failed redaction".into());
    }
    if !result.cleanup.complete {
        return Err("result cleanup is incomplete".into());
    }
    if result
        .artifact_digests
        .iter()
        .any(|artifact| !repo_relative(&artifact.path, false) || !valid_digest(&artifact.sha256))
    {
        return Err("result contains invalid artifact provenance".into());
    }
    let declared_artifacts: BTreeSet<_> = request.artifact_policy.paths.iter().collect();
    let result_artifacts: BTreeSet<_> = result
        .artifact_digests
        .iter()
        .map(|artifact| &artifact.path)
        .collect();
    let result_bytes = result
        .artifact_digests
        .iter()
        .try_fold(0_u64, |total, artifact| total.checked_add(artifact.bytes))
        .ok_or_else(|| "result artifact byte count overflowed".to_string())?;
    if result_artifacts.len() != result.artifact_digests.len()
        || !result_artifacts.is_subset(&declared_artifacts)
        || result_bytes > request.artifact_policy.max_total_bytes
    {
        return Err("result artifacts exceed the declared policy".into());
    }
    if request.artifact_policy.required
        && request.artifact_policy.paths.iter().any(|path| {
            !result
                .artifact_digests
                .iter()
                .any(|artifact| &artifact.path == path)
        })
    {
        return Err("required artifact is missing".into());
    }
    if result.outcome == RunOutcome::Passed && result.exit_code != Some(0) {
        return Err("passed result must have exit code zero".into());
    }
    if result.fallback.policy != request.fallback
        || (result.fallback.ran && !result.fallback.offered)
        || (result.fallback.ran
            && result.fallback.local_profile_digest.as_deref()
                != Some(&request.command_profile_digest))
    {
        return Err("result fallback does not match the request".into());
    }
    Ok(())
}

fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn current_revision(checkout: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(checkout)
        .output()
        .map_err(|error| format!("git revision check failed: {error}"))?;
    if !output.status.success() {
        return Err("git revision check failed".into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn verify_checkout_revision(checkout: &Path, revision: &str) -> Result<(), String> {
    if current_revision(checkout)? != revision {
        return Err("stale revision: local checkout does not match request".into());
    }
    let output = Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=all"])
        .current_dir(checkout)
        .output()
        .map_err(|error| format!("git cleanliness check failed: {error}"))?;
    if !output.status.success() {
        return Err("git cleanliness check failed".into());
    }
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let untracked_evidence = line
            .strip_prefix("?? ")
            .is_some_and(|path| path.starts_with(".csdlc/evidence/"));
        if !untracked_evidence {
            return Err("checkout has changes outside untracked evidence".into());
        }
    }
    Ok(())
}

fn artifact_digests(
    checkout: &Path,
    policy: &ArtifactPolicy,
) -> Result<Vec<ArtifactDigest>, String> {
    let mut total = 0_u64;
    let mut artifacts = Vec::new();
    for relative in &policy.paths {
        let path = checkout.join(relative);
        if !path.is_file() {
            if policy.required {
                return Err(format!("required artifact missing: {relative}"));
            }
            continue;
        }
        let bytes = fs::read(&path).map_err(|error| format!("artifact read failed: {error}"))?;
        total = total.saturating_add(bytes.len() as u64);
        if total > policy.max_total_bytes {
            return Err("artifact budget exceeded".into());
        }
        artifacts.push(ArtifactDigest {
            path: relative.clone(),
            sha256: sha256_hex(&bytes),
            bytes: bytes.len() as u64,
        });
    }
    Ok(artifacts)
}

pub fn canonicalize_adapter_result(
    request: &PortableRequest,
    receipt: &AdapterExecutionReceipt,
    artifact_root: &Path,
) -> Result<PortableResult, String> {
    validate_request(request)?;
    if receipt.schema != "adl.remote_validation.adapter_execution.v1"
        || receipt.adapter != request.adapter
    {
        return Err("adapter execution receipt identity does not match request".into());
    }
    let result = PortableResult {
        schema: RESULT_SCHEMA.into(),
        request_id: request.request_id.clone(),
        adapter: receipt.adapter,
        platform: receipt.platform.clone(),
        revision: receipt.revision.clone(),
        command_profile_digest: request.command_profile_digest.clone(),
        resource_budget: request.resource_budget.clone(),
        artifact_policy: request.artifact_policy.clone(),
        cancellation_file: request.cancellation_file.clone(),
        started_unix_ms: receipt.started_unix_ms,
        finished_unix_ms: receipt.finished_unix_ms,
        exit_code: receipt.exit_code,
        outcome: receipt.outcome.clone(),
        artifact_digests: artifact_digests(artifact_root, &request.artifact_policy)?,
        redaction_passed: receipt.redaction_passed,
        cleanup: receipt.cleanup.clone(),
        fallback: receipt.fallback.clone(),
    };
    validate_result(request, &result)?;
    Ok(result)
}

pub fn run_local(
    request: &PortableRequest,
    repository_root: &Path,
) -> Result<PortableResult, String> {
    validate_request(request)?;
    if request.adapter != AdapterKind::Local {
        return Err("run_local requires the local adapter".into());
    }
    let checkout = repository_root.join(&request.checkout);
    verify_checkout_revision(&checkout, &request.revision)?;
    let command_dir = checkout.join(&request.command_profile.working_directory);
    let started = unix_ms();
    let mut command = Command::new(&request.command_profile.argv[0]);
    command
        .args(&request.command_profile.argv[1..])
        .current_dir(&command_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .env_clear();
    for name in &request.command_profile.environment_allowlist {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("local command failed to start: {error}"))?;
    let timeout = Duration::from_secs(request.resource_budget.timeout_seconds);
    let start = Instant::now();
    let cancellation = request
        .cancellation_file
        .as_ref()
        .map(|path| checkout.join(path));
    let (outcome, exit_code, cleanup) = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("local wait failed: {error}"))?
        {
            break (
                if status.success() {
                    RunOutcome::Passed
                } else {
                    RunOutcome::Failed
                },
                status.code(),
                CleanupStatus {
                    attempted: false,
                    complete: true,
                    detail: None,
                },
            );
        }
        let cancelled = cancellation.as_ref().is_some_and(|path| path.exists());
        if cancelled || start.elapsed() >= timeout {
            let kill = child.kill();
            let _ = child.wait();
            break (
                if cancelled {
                    RunOutcome::Cancelled
                } else {
                    RunOutcome::TimedOut
                },
                None,
                CleanupStatus {
                    attempted: true,
                    complete: kill.is_ok(),
                    detail: kill.err().map(|error| error.to_string()),
                },
            );
        }
        thread::sleep(Duration::from_millis(20));
    };
    let artifacts = artifact_digests(&checkout, &request.artifact_policy)?;
    Ok(PortableResult {
        schema: RESULT_SCHEMA.into(),
        request_id: request.request_id.clone(),
        adapter: AdapterKind::Local,
        platform: PlatformRecord {
            os: std::env::consts::OS.into(),
            architecture: std::env::consts::ARCH.into(),
            native: true,
            qualification: "live".into(),
        },
        revision: request.revision.clone(),
        command_profile_digest: request.command_profile_digest.clone(),
        resource_budget: request.resource_budget.clone(),
        artifact_policy: request.artifact_policy.clone(),
        cancellation_file: request.cancellation_file.clone(),
        started_unix_ms: started,
        finished_unix_ms: unix_ms(),
        exit_code,
        outcome,
        artifact_digests: artifacts,
        redaction_passed: true,
        cleanup,
        fallback: FallbackStatus {
            policy: FallbackPolicy::Disabled,
            offered: false,
            ran: false,
            local_profile_digest: None,
        },
    })
}

pub fn read_request(path: &Path) -> Result<PortableRequest, String> {
    let bytes = fs::read(path).map_err(|error| format!("request read failed: {error}"))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("malformed request: {error}"))
}

pub fn read_result(path: &Path) -> Result<PortableResult, String> {
    let bytes = fs::read(path).map_err(|error| format!("result read failed: {error}"))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("malformed result: {error}"))
}

pub fn resolve_repository_root(start: &Path) -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(start)
        .output()
        .map_err(|error| format!("repository root lookup failed: {error}"))?;
    if !output.status.success() {
        return Err("repository root lookup failed".into());
    }
    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}
