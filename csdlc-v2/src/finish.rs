use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use octocrab::params::pulls::MergeMethod as OctoMergeMethod;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::cards::{CardContent, CardKind};
use crate::error::{ErrorCode, Result, V2Error};
use crate::estimation::{
    artifact_reference, canonical_digest, load_observation_manifest, load_verified_json,
    terminal_outcome, validate_accepted_estimate, verified_calibration, ArtifactReference,
    Availability, EstimateDisposition, EstimateMethod, Forecast, MetricObservation, Observation,
    ObservationSource, Provenance, TerminalOutcome, OBSERVATION_SCHEMA, OUTCOME_SCHEMA,
};
use crate::git::{self, clean_commit_revision};
use crate::github::{
    collect_issue_closing_pull_requests, collect_pr_state, execute_github_action,
    ClosingPullRequestIdentity, GithubAction, GithubActionRequest, GithubIssuePacket,
    PrStatePacket, PrStateRequest,
};
use crate::github_token;
use crate::model::{IssueRecord, LifecyclePhase};
use crate::store::Store;

pub const NO_PR_APPROVAL_LABEL: &str = "closeout:no-pr-approved";
const MUTABLE_TERMINAL_FRESHNESS_SECONDS: u64 = 300;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinishDisposition {
    Merged,
    ClosedUnmerged,
    ClosedNoPr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordlessCloseoutMode {
    ClassifyOnly,
    RetainReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordlessCloseoutTarget {
    pub issue: u64,
    pub issue_repository: String,
    pub pr_repository: String,
    pub pull_request: u64,
    pub expected_head_sha: String,
    pub expected_merge_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordlessCloseoutRequest {
    pub schema: String,
    pub actor: String,
    pub approved_reason: String,
    pub mode: RecordlessCloseoutMode,
    pub targets: Vec<RecordlessCloseoutTarget>,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordlessTerminalReceipt {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub receipt_ref: String,
    pub terminal: DerivedTerminalEnvelope,
    pub actor: String,
    pub approved_reason: String,
    pub source_projection_at_pr_head: bool,
    pub local_projection_present: bool,
    pub existing_closeout_receipt_present: bool,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordlessCloseoutTargetResult {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub pull_request: u64,
    pub classification: String,
    pub receipt_ref: Option<String>,
    pub terminal: Option<DerivedTerminalEnvelope>,
    pub blocker: Option<String>,
    pub source_projection_at_pr_head: bool,
    pub local_projection_present: bool,
    pub existing_closeout_receipt_present: bool,
    pub retained: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RecordlessCloseoutResult {
    pub schema: String,
    pub actor: String,
    pub mode: RecordlessCloseoutMode,
    pub results: Vec<RecordlessCloseoutTargetResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub repository: String,
    pub pull_request: Option<u64>,
    pub base: Option<String>,
    pub head: Option<String>,
    pub expected_head_sha: Option<String>,
    pub merge_method: MergeMethod,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub approved_no_pr_reason: Option<String>,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HistoricalFinishRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    /// Exact GitHub repository in `owner/name` form.
    #[schemars(regex(pattern = "^[^/]+/[^/]+$"))]
    pub issue_repository: String,
    pub disposition: FinishDisposition,
    /// Exact PR repository in `owner/name` form when the disposition uses a PR.
    #[schemars(regex(pattern = "^[^/]+/[^/]+$"))]
    pub pr_repository: Option<String>,
    pub pull_request: Option<u64>,
    /// Exact 40- or 64-character hexadecimal Git object ID.
    #[schemars(regex(pattern = "^(?:[0-9A-Fa-f]{40}|[0-9A-Fa-f]{64})$"))]
    pub expected_head_sha: Option<String>,
    /// Exact 40- or 64-character hexadecimal Git object ID for merged dispositions only.
    #[schemars(regex(pattern = "^(?:[0-9A-Fa-f]{40}|[0-9A-Fa-f]{64})$"))]
    pub expected_merge_sha: Option<String>,
    pub approved_reason: Option<String>,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DerivedTerminalEnvelope {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub initialization_digest: String,
    pub canonical_generation: u64,
    pub canonical_digest: String,
    pub pull_request: Option<u64>,
    pub disposition: FinishDisposition,
    pub head_sha: Option<String>,
    pub merge_sha: Option<String>,
    pub issue_state: String,
    pub pr_state: Option<String>,
    pub approved_reason: Option<String>,
    pub observed_unix_seconds: u64,
    pub mutable_fresh_until_unix_seconds: Option<u64>,
    pub source: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IssueTerminalObservation {
    pub state: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub observed_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishResult {
    pub schema: String,
    pub terminal: DerivedTerminalEnvelope,
    pub already_terminal: bool,
    pub estimation: TerminalEstimationResult,
}

pub async fn execute_recordless_closeout(
    root: &Path,
    request: &RecordlessCloseoutRequest,
) -> Result<RecordlessCloseoutResult> {
    validate_recordless_request(request)?;
    let mut results = Vec::new();
    for target in &request.targets {
        let issue = read_issue_in_repository(
            &target.issue_repository,
            target.issue,
            request.token_file.clone(),
        )
        .await?;
        let observation = issue_observation(issue, now_unix_seconds()?);
        let candidates = collect_issue_closing_pull_requests(
            &target.issue_repository,
            target.issue,
            request.token_file.clone(),
        )
        .await?;
        let packet = collect_pr_state(&PrStateRequest {
            repository: target.pr_repository.clone(),
            pull_request: target.pull_request,
            required_checks: Vec::new(),
            require_review: false,
            token_file: request.token_file.clone(),
            linked_issue: Some(target.issue),
            linked_issue_repository: Some(target.issue_repository.clone()),
        })
        .await?;
        let mut result = classify_recordless_closeout_target(
            root,
            request,
            target,
            &observation,
            &packet,
            &candidates,
        )?;
        if request.mode == RecordlessCloseoutMode::RetainReceipt
            && result.classification == "recordless_terminal_eligible"
        {
            let terminal = result.terminal.clone().ok_or_else(|| {
                V2Error::new(
                    ErrorCode::CorruptRecord,
                    "recordless eligible result is missing terminal envelope",
                )
            })?;
            let receipt = build_recordless_terminal_receipt(
                root,
                request,
                target,
                terminal,
                result.source_projection_at_pr_head,
                result.local_projection_present,
                result.existing_closeout_receipt_present,
            )?;
            retain_recordless_terminal_receipt(root, &receipt)?;
            result.receipt_ref = Some(receipt.receipt_ref);
            result.retained = true;
        }
        results.push(result);
    }
    Ok(RecordlessCloseoutResult {
        schema: "csdlc.recordless_closeout_result.v1".into(),
        actor: request.actor.clone(),
        mode: request.mode,
        results,
    })
}

pub fn classify_recordless_closeout_target(
    root: &Path,
    request: &RecordlessCloseoutRequest,
    target: &RecordlessCloseoutTarget,
    issue: &IssueTerminalObservation,
    packet: &PrStatePacket,
    candidates: &[ClosingPullRequestIdentity],
) -> Result<RecordlessCloseoutTargetResult> {
    validate_recordless_request(request)?;
    validate_recordless_target(target)?;
    let source_projection_at_pr_head =
        source_projection_at_revision(root, &target.expected_head_sha, target.issue)?;
    let local_projection_present = root
        .join(".csdlc/issues")
        .join(target.issue.to_string())
        .join("index.json")
        .exists();
    let existing_closeout_receipt_present = Store::new(root)
        .terminal_receipt_path(target.issue)?
        .exists();
    let blocker = recordless_blocker(RecordlessBlockerContext {
        root,
        request,
        target,
        issue,
        packet,
        candidates,
        source_projection_at_pr_head,
        local_projection_present,
        existing_closeout_receipt_present,
    })?;
    if let Some(blocker) = blocker {
        return Ok(RecordlessCloseoutTargetResult {
            schema: "csdlc.recordless_closeout_target_result.v1".into(),
            issue: target.issue,
            repository: target.issue_repository.clone(),
            pull_request: target.pull_request,
            classification: blocker.clone(),
            receipt_ref: None,
            terminal: None,
            blocker: Some(blocker),
            source_projection_at_pr_head,
            local_projection_present,
            existing_closeout_receipt_present,
            retained: false,
        });
    }
    let terminal = derive_recordless_terminal(request, target, issue, packet)?;
    Ok(RecordlessCloseoutTargetResult {
        schema: "csdlc.recordless_closeout_target_result.v1".into(),
        issue: target.issue,
        repository: target.issue_repository.clone(),
        pull_request: target.pull_request,
        classification: "recordless_terminal_eligible".into(),
        receipt_ref: (request.mode == RecordlessCloseoutMode::RetainReceipt)
            .then(|| format!("csdlc-v2/closeout/{}.json", target.issue)),
        terminal: Some(terminal),
        blocker: None,
        source_projection_at_pr_head,
        local_projection_present,
        existing_closeout_receipt_present,
        retained: false,
    })
}

struct RecordlessBlockerContext<'a> {
    root: &'a Path,
    request: &'a RecordlessCloseoutRequest,
    target: &'a RecordlessCloseoutTarget,
    issue: &'a IssueTerminalObservation,
    packet: &'a PrStatePacket,
    candidates: &'a [ClosingPullRequestIdentity],
    source_projection_at_pr_head: bool,
    local_projection_present: bool,
    existing_closeout_receipt_present: bool,
}

fn recordless_blocker(context: RecordlessBlockerContext<'_>) -> Result<Option<String>> {
    let RecordlessBlockerContext {
        root,
        request,
        target,
        issue,
        packet,
        candidates,
        source_projection_at_pr_head,
        local_projection_present,
        existing_closeout_receipt_present,
    } = context;
    if issue.state != "closed" {
        return Ok(Some("live_issue_not_closed".into()));
    }
    let historical = HistoricalFinishRequest {
        schema: "csdlc.historical_finish_request.v1".into(),
        issue: target.issue,
        expected_generation: 0,
        expected_digest: "recordless".into(),
        actor: request.actor.clone(),
        issue_repository: target.issue_repository.clone(),
        disposition: FinishDisposition::Merged,
        pr_repository: Some(target.pr_repository.clone()),
        pull_request: Some(target.pull_request),
        expected_head_sha: Some(target.expected_head_sha.clone()),
        expected_merge_sha: Some(target.expected_merge_sha.clone()),
        approved_reason: Some(request.approved_reason.clone()),
        token_file: request.token_file.clone(),
    };
    validate_historical_candidates(&historical, candidates)?;
    if packet.repository != target.pr_repository
        || packet.pull_request != target.pull_request
        || packet.linked_issue != Some(target.issue)
        || packet.linkage_source.as_deref() != Some("github_closing_issues_references")
        || packet.state != "closed"
        || !packet.merged
        || packet.head_sha != target.expected_head_sha
        || packet.merge_commit_sha.as_deref() != Some(target.expected_merge_sha.as_str())
    {
        return Ok(Some("live_pr_identity_mismatch".into()));
    }
    if existing_closeout_receipt_present {
        return Ok(Some("existing_closeout_receipt_present".into()));
    }
    if local_projection_present {
        return Ok(Some("local_projection_present_use_normal_finish".into()));
    }
    if source_projection_at_pr_head {
        return Ok(Some(
            "source_projection_at_pr_head_use_normal_finish".into(),
        ));
    }
    if historical_publication_conflicts(root, target.issue, target.pull_request)? {
        return Ok(Some("conflicting_historical_publication".into()));
    }
    Ok(None)
}

fn derive_recordless_terminal(
    request: &RecordlessCloseoutRequest,
    target: &RecordlessCloseoutTarget,
    issue: &IssueTerminalObservation,
    packet: &PrStatePacket,
) -> Result<DerivedTerminalEnvelope> {
    let mut envelope = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: target.issue,
        repository: target.issue_repository.clone(),
        initialization_digest: "recordless".into(),
        canonical_generation: 0,
        canonical_digest: "recordless".into(),
        pull_request: Some(target.pull_request),
        disposition: FinishDisposition::Merged,
        head_sha: Some(packet.head_sha.clone()),
        merge_sha: packet.merge_commit_sha.clone(),
        issue_state: "closed_by_merged_pr".into(),
        pr_state: Some(packet.state.clone()),
        approved_reason: Some(request.approved_reason.clone()),
        observed_unix_seconds: issue.observed_unix_seconds,
        mutable_fresh_until_unix_seconds: None,
        source: "live_github_recordless_closeout".into(),
        digest: String::new(),
    };
    envelope.digest = envelope_digest(&envelope)?;
    validate_envelope(&envelope)?;
    Ok(envelope)
}

fn build_recordless_terminal_receipt(
    root: &Path,
    request: &RecordlessCloseoutRequest,
    target: &RecordlessCloseoutTarget,
    terminal: DerivedTerminalEnvelope,
    source_projection_at_pr_head: bool,
    local_projection_present: bool,
    existing_closeout_receipt_present: bool,
) -> Result<RecordlessTerminalReceipt> {
    let receipt_ref = format!("csdlc-v2/closeout/{}.json", target.issue);
    let mut receipt = RecordlessTerminalReceipt {
        schema: "csdlc.recordless_terminal_receipt.v1".into(),
        issue: target.issue,
        repository: target.issue_repository.clone(),
        receipt_ref,
        terminal,
        actor: request.actor.clone(),
        approved_reason: request.approved_reason.clone(),
        source_projection_at_pr_head,
        local_projection_present,
        existing_closeout_receipt_present,
        digest: String::new(),
    };
    receipt.digest = recordless_terminal_receipt_digest(&receipt)?;
    validate_recordless_terminal_receipt(&receipt)?;
    let path = Store::new(root).terminal_receipt_path(target.issue)?;
    let common = PathBuf::from(
        git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    if !path.starts_with(common) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "recordless terminal receipt escapes Git-common root",
        ));
    }
    Ok(receipt)
}

fn retain_recordless_terminal_receipt(
    root: &Path,
    receipt: &RecordlessTerminalReceipt,
) -> Result<PathBuf> {
    validate_recordless_terminal_receipt(receipt)?;
    let path = Store::new(root).terminal_receipt_path(receipt.issue)?;
    if path.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recordless closeout refuses to overwrite an existing terminal receipt",
        ));
    }
    let parent = path.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "recordless terminal receipt has no parent directory",
        )
    })?;
    fs::create_dir_all(parent)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(receipt)?)?;
    fs::rename(&tmp, &path)?;
    Ok(path)
}

fn source_projection_at_revision(root: &Path, revision: &str, issue: u64) -> Result<bool> {
    validate_sha(revision, "expected head SHA")?;
    let commit_spec = format!("{revision}^{{commit}}");
    let commit = Command::new("git")
        .current_dir(root)
        .args(["cat-file", "-e", &commit_spec])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !commit.status.success() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recordless closeout expected head SHA is not available as a local commit object",
        ));
    }
    let spec = format!("{revision}:.csdlc/issues/{issue}/index.json");
    let output = Command::new("git")
        .current_dir(root)
        .args(["cat-file", "-e", &spec])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    Ok(output.status.success())
}

fn historical_publication_conflicts(root: &Path, issue: u64, pull_request: u64) -> Result<bool> {
    let path = format!(".csdlc/issues/{issue}/index.json");
    let output = Command::new("git")
        .current_dir(root)
        .args(["log", "--all", "--format=%H", "--", &path])
        .output()
        .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
    if !output.status.success() {
        return Err(V2Error::new(
            ErrorCode::GitFailure,
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    for revision in String::from_utf8_lossy(&output.stdout).lines() {
        let spec = format!("{revision}:{path}");
        let show = Command::new("git")
            .current_dir(root)
            .args(["show", &spec])
            .output()
            .map_err(|error| V2Error::new(ErrorCode::GitFailure, error.to_string()))?;
        if !show.status.success() {
            continue;
        }
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&show.stdout) else {
            continue;
        };
        let observed = value
            .get("publication")
            .and_then(|publication| publication.get("pull_request"))
            .and_then(serde_json::Value::as_u64);
        if observed.is_some_and(|observed| observed != pull_request) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn validate_recordless_request(request: &RecordlessCloseoutRequest) -> Result<()> {
    if request.schema != "csdlc.recordless_closeout_request.v1"
        || request.actor.trim().is_empty()
        || request.approved_reason.trim().len() < 12
        || request.targets.is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "recordless closeout request is incomplete",
        ));
    }
    for target in &request.targets {
        validate_recordless_target(target)?;
    }
    Ok(())
}

fn validate_recordless_target(target: &RecordlessCloseoutTarget) -> Result<()> {
    if target.issue == 0
        || target.issue_repository.split_once('/').is_none()
        || target.pr_repository.split_once('/').is_none()
        || target.pull_request == 0
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "recordless closeout target identity is incomplete",
        ));
    }
    validate_sha(&target.expected_head_sha, "expected head SHA")?;
    validate_sha(&target.expected_merge_sha, "expected merge SHA")?;
    Ok(())
}

fn validate_sha(value: &str, label: &str) -> Result<()> {
    if value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::InvalidInput,
            format!("recordless closeout {label} is not a full 40-character SHA"),
        ))
    }
}

fn recordless_terminal_receipt_digest(receipt: &RecordlessTerminalReceipt) -> Result<String> {
    let mut canonical = receipt.clone();
    canonical.digest.clear();
    Ok(blake3::hash(&serde_json::to_vec(&canonical)?)
        .to_hex()
        .to_string())
}

fn validate_recordless_terminal_receipt(receipt: &RecordlessTerminalReceipt) -> Result<()> {
    if receipt.schema != "csdlc.recordless_terminal_receipt.v1"
        || receipt.issue == 0
        || receipt.repository.split_once('/').is_none()
        || receipt.receipt_ref != format!("csdlc-v2/closeout/{}.json", receipt.issue)
        || receipt.source_projection_at_pr_head
        || receipt.local_projection_present
        || receipt.existing_closeout_receipt_present
        || receipt.digest != recordless_terminal_receipt_digest(receipt)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recordless terminal receipt identity is invalid",
        ));
    }
    validate_envelope(&receipt.terminal)?;
    if receipt.terminal.issue != receipt.issue || receipt.terminal.repository != receipt.repository
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "recordless terminal receipt envelope identity is invalid",
        ));
    }
    Ok(())
}

pub async fn execute_historical_finish(
    root: &Path,
    request: &HistoricalFinishRequest,
) -> Result<FinishResult> {
    let store = Store::new(root);
    let _authority_lock = store.authority_projection_lock(request.issue)?;
    let record = store.load_record(request.issue)?;
    validate_historical_request(&record, request)?;

    let issue = read_issue_in_repository(
        &request.issue_repository,
        request.issue,
        request.token_file.clone(),
    )
    .await?;
    let observation = issue_observation(issue, now_unix_seconds()?);
    let candidates = collect_issue_closing_pull_requests(
        &request.issue_repository,
        request.issue,
        request.token_file.clone(),
    )
    .await?;
    validate_historical_candidates(request, &candidates)?;
    let packet = match (request.pr_repository.as_ref(), request.pull_request) {
        (Some(repository), Some(pull_request)) => Some(
            collect_pr_state(&PrStateRequest {
                repository: repository.clone(),
                pull_request,
                required_checks: Vec::new(),
                require_review: false,
                token_file: request.token_file.clone(),
                linked_issue: Some(request.issue),
                linked_issue_repository: Some(request.issue_repository.clone()),
            })
            .await?,
        ),
        (None, None) => None,
        _ => {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "historical finish PR identity is incomplete",
            ))
        }
    };
    let observed = derive_historical_terminal(&record, request, &observation, packet.as_ref())?;
    let (terminal, already_terminal) =
        select_historical_terminal(load_cached_terminal(store.root(), request.issue)?, observed)?;
    retain_cached_terminal(store.root(), &terminal)?;
    let estimation = retain_terminal_estimation_outcome(&store, &terminal);
    Ok(FinishResult {
        schema: "csdlc.finish_result.v1".into(),
        terminal,
        already_terminal,
        estimation,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TerminalEstimationStatus {
    NotPlanned,
    Deferred,
    Recorded,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalEstimationResult {
    pub schema: String,
    pub issue: u64,
    pub status: TerminalEstimationStatus,
    pub outcome_ref: Option<String>,
    pub outcome_digest: Option<String>,
    pub detail: String,
}

/// Execute the complete terminal operation. The remote merge primitive is kept
/// private so no caller can merge without exact-head validation, live terminal
/// re-observation, and derived-envelope retention.
pub async fn execute_finish(root: &Path, request: &FinishRequest) -> Result<FinishResult> {
    let store = Store::new(root);
    let _authority_lock = store.authority_projection_lock(request.issue)?;
    let record = store.load_record(request.issue)?;
    validate_canonical_identity(&record, request)?;
    validate_publication_head_in_repo(store.root(), &record, request)?;
    let pr_repository = publication_repository(&record, request)?;

    let issue = read_issue(request).await?;
    let observation = issue_observation(issue, now_unix_seconds()?);
    let packet = match request.pull_request {
        Some(pull_request) => Some(
            collect_pr_state(&PrStateRequest {
                repository: pr_repository.to_string(),
                pull_request,
                required_checks: request.required_checks.clone(),
                require_review: request.require_review,
                token_file: request.token_file.clone(),
                linked_issue: Some(request.issue),
                linked_issue_repository: Some(record.repository.clone()),
            })
            .await?,
        ),
        None => None,
    };

    if let Some(terminal) = derive_terminal(&record, request, &observation, packet.as_ref())? {
        retain_cached_terminal(store.root(), &terminal)?;
        let estimation = retain_terminal_estimation_outcome(&store, &terminal);
        return Ok(FinishResult {
            schema: "csdlc.finish_result.v1".into(),
            terminal,
            already_terminal: true,
            estimation,
        });
    }

    let state = packet.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "issue is open and has no PR terminal authority",
        )
    })?;
    validate_finish_merge_authority(store.root(), &record, request, now_unix_seconds()?)?;
    validate_remote_merge(state, request, pr_repository)?;
    let token = github_token::resolve(request.token_file.as_deref())?;
    execute_remote_merge(request, pr_repository, token).await?;

    let observed_issue = issue_observation(read_issue(request).await?, now_unix_seconds()?);
    let observed_pr = collect_pr_state(&PrStateRequest {
        repository: pr_repository.to_string(),
        pull_request: request.pull_request.expect("validated PR finish request"),
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: Some(request.issue),
        linked_issue_repository: Some(record.repository.clone()),
    })
    .await?;
    let terminal = derive_terminal(&record, request, &observed_issue, Some(&observed_pr))?
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "merge returned success but GitHub did not re-observe terminal PR state",
            )
        })?;
    retain_cached_terminal(store.root(), &terminal)?;
    let estimation = retain_terminal_estimation_outcome(&store, &terminal);
    Ok(FinishResult {
        schema: "csdlc.finish_result.v1".into(),
        terminal,
        already_terminal: false,
        estimation,
    })
}

pub fn validate_historical_request(
    record: &IssueRecord,
    request: &HistoricalFinishRequest,
) -> Result<()> {
    if request.schema != "csdlc.historical_finish_request.v1"
        || request.issue == 0
        || request.actor.trim().is_empty()
        || !valid_repository(&request.issue_repository)
        || request.issue != record.issue
        || request.issue_repository != record.repository
        || request.expected_generation != record.generation
        || request.expected_digest != record.digest
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "historical finish request does not match canonical issue identity",
        ));
    }
    let reason_present = request
        .approved_reason
        .as_deref()
        .is_some_and(|reason| !reason.trim().is_empty());
    let pr_repository_present = request
        .pr_repository
        .as_deref()
        .is_some_and(valid_repository);
    let pr_present = request.pull_request.is_some_and(|number| number > 0);
    let head_present = request
        .expected_head_sha
        .as_deref()
        .is_some_and(valid_git_oid);
    let merge_present = request
        .expected_merge_sha
        .as_deref()
        .is_some_and(valid_git_oid);
    let valid = match request.disposition {
        FinishDisposition::Merged => {
            pr_repository_present
                && pr_present
                && head_present
                && merge_present
                && request.approved_reason.is_none()
        }
        FinishDisposition::ClosedUnmerged => {
            pr_repository_present
                && pr_present
                && head_present
                && request.expected_merge_sha.is_none()
                && reason_present
        }
        FinishDisposition::ClosedNoPr => {
            request.pr_repository.is_none()
                && request.pull_request.is_none()
                && request.expected_head_sha.is_none()
                && request.expected_merge_sha.is_none()
                && reason_present
        }
    };
    if !valid {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "historical finish fields contradict the requested disposition",
        ));
    }
    Ok(())
}

fn valid_repository(repository: &str) -> bool {
    let mut parts = repository.split('/');
    matches!(
        (parts.next(), parts.next(), parts.next()),
        (Some(owner), Some(repo), None) if !owner.is_empty() && !repo.is_empty()
    )
}

fn valid_git_oid(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn validate_historical_candidates(
    request: &HistoricalFinishRequest,
    candidates: &[ClosingPullRequestIdentity],
) -> Result<()> {
    let expected = match (&request.pr_repository, request.pull_request) {
        (Some(repository), Some(pull_request)) => Some((repository.as_str(), pull_request)),
        (None, None) => None,
        _ => {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "historical finish PR identity is incomplete",
            ))
        }
    };
    let identity_matches = |candidate: &ClosingPullRequestIdentity, expected: (&str, u64)| {
        candidate.repository == expected.0 && candidate.pull_request == expected.1
    };
    let merged = candidates
        .iter()
        .filter(|candidate| candidate.merged || candidate.state == "MERGED")
        .collect::<Vec<_>>();
    let valid = match (request.disposition, expected) {
        (FinishDisposition::Merged, Some(expected)) => match merged.as_slice() {
            [candidate] => identity_matches(candidate, expected),
            candidates => unique_latest_merged_candidate(candidates)
                .is_some_and(|candidate| identity_matches(candidate, expected)),
        },
        (FinishDisposition::ClosedUnmerged, Some(expected)) => {
            merged.is_empty()
                && candidates.len() == 1
                && candidates[0].state == "CLOSED"
                && !candidates[0].merged
                && identity_matches(&candidates[0], expected)
        }
        (FinishDisposition::ClosedNoPr, None) => candidates.is_empty(),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "historical finish has no unique terminal-precedence closing PR identity",
        ))
    }
}

fn unique_latest_merged_candidate<'a>(
    candidates: &[&'a ClosingPullRequestIdentity],
) -> Option<&'a ClosingPullRequestIdentity> {
    let parsed = candidates
        .iter()
        .map(|candidate| {
            let merged_at = candidate.merged_at.as_deref()?;
            let instant = OffsetDateTime::parse(merged_at, &Rfc3339).ok()?;
            Some((*candidate, instant))
        })
        .collect::<Option<Vec<_>>>()?;
    let latest = parsed.iter().map(|(_, instant)| *instant).max()?;
    let mut latest_candidates = parsed
        .iter()
        .filter(|(_, instant)| *instant == latest)
        .map(|(candidate, _)| *candidate);
    let candidate = latest_candidates.next()?;
    latest_candidates.next().is_none().then_some(candidate)
}

pub fn select_historical_terminal(
    existing: Option<DerivedTerminalEnvelope>,
    observed: DerivedTerminalEnvelope,
) -> Result<(DerivedTerminalEnvelope, bool)> {
    let Some(existing) = existing else {
        return Ok((observed, false));
    };
    validate_envelope(&existing)?;
    validate_envelope(&observed)?;
    let same_authority = existing.issue == observed.issue
        && existing.repository == observed.repository
        && existing.initialization_digest == observed.initialization_digest
        && existing.canonical_generation == observed.canonical_generation
        && existing.canonical_digest == observed.canonical_digest
        && existing.pull_request == observed.pull_request
        && existing.disposition == observed.disposition
        && existing.head_sha == observed.head_sha
        && existing.merge_sha == observed.merge_sha
        && existing.issue_state == observed.issue_state
        && existing.pr_state == observed.pr_state
        && existing.approved_reason == observed.approved_reason
        && existing.source == observed.source;
    if same_authority {
        Ok((existing, true))
    } else {
        Ok((observed, false))
    }
}

pub fn derive_historical_terminal(
    record: &IssueRecord,
    request: &HistoricalFinishRequest,
    issue: &IssueTerminalObservation,
    packet: Option<&PrStatePacket>,
) -> Result<DerivedTerminalEnvelope> {
    validate_historical_request(record, request)?;
    if issue.state != "closed" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "historical finish requires a closed live issue",
        ));
    }
    let (pull_request, head_sha, merge_sha, pr_state) = match request.disposition {
        FinishDisposition::Merged => {
            let packet = validate_historical_packet(request, packet)?;
            let expected_merge = request.expected_merge_sha.as_deref();
            if !packet.merged
                || packet.state != "closed"
                || packet.merge_commit_sha.as_deref() != expected_merge
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "historical merged PR state or merge SHA does not match",
                ));
            }
            (
                Some(packet.pull_request),
                Some(packet.head_sha.clone()),
                packet.merge_commit_sha.clone(),
                Some(packet.state.clone()),
            )
        }
        FinishDisposition::ClosedUnmerged => {
            let packet = validate_historical_packet(request, packet)?;
            if packet.merged || packet.state != "closed" {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "historical closed-unmerged PR is not exactly closed and unmerged",
                ));
            }
            (
                Some(packet.pull_request),
                Some(packet.head_sha.clone()),
                None,
                Some(packet.state.clone()),
            )
        }
        FinishDisposition::ClosedNoPr => {
            if packet.is_some()
                || !issue
                    .labels
                    .iter()
                    .any(|label| label == NO_PR_APPROVAL_LABEL)
            {
                return Err(V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!(
                        "historical no-PR closure requires GitHub label {NO_PR_APPROVAL_LABEL}"
                    ),
                ));
            }
            (None, None, None, None)
        }
    };
    let mut envelope = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: record.issue,
        repository: record.repository.clone(),
        initialization_digest: record.initialization_digest.clone(),
        canonical_generation: record.generation,
        canonical_digest: record.digest.clone(),
        pull_request,
        disposition: request.disposition,
        head_sha,
        merge_sha,
        issue_state: if request.disposition == FinishDisposition::Merged {
            "closed_by_merged_pr".into()
        } else {
            issue.state.clone()
        },
        pr_state,
        approved_reason: request.approved_reason.clone(),
        observed_unix_seconds: issue.observed_unix_seconds,
        mutable_fresh_until_unix_seconds: (request.disposition != FinishDisposition::Merged).then(
            || {
                issue
                    .observed_unix_seconds
                    .saturating_add(MUTABLE_TERMINAL_FRESHNESS_SECONDS)
            },
        ),
        source: "live_github_historical_reconciliation".into(),
        digest: String::new(),
    };
    envelope.digest = envelope_digest(&envelope)?;
    validate_envelope(&envelope)?;
    Ok(envelope)
}

fn validate_historical_packet<'a>(
    request: &HistoricalFinishRequest,
    packet: Option<&'a PrStatePacket>,
) -> Result<&'a PrStatePacket> {
    let packet = packet.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "historical PR disposition requires exact live PR state",
        )
    })?;
    if Some(packet.repository.as_str()) != request.pr_repository.as_deref()
        || Some(packet.pull_request) != request.pull_request
        || packet.linked_issue != Some(request.issue)
        || packet.linkage_source.as_deref() != Some("github_closing_issues_references")
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "historical PR does not match exact repository, issue, PR, linkage, or head identity",
        ));
    }
    Ok(packet)
}

/// Retain advisory forecast-versus-actual truth after terminal authority is
/// durable. Estimation is deliberately non-enforcing: a malformed or missing
/// advisory artifact is reported as `invalid` and never reverses closeout.
pub fn retain_terminal_estimation_outcome(
    store: &Store,
    terminal: &DerivedTerminalEnvelope,
) -> TerminalEstimationResult {
    match try_retain_terminal_estimation_outcome(store, terminal) {
        Ok(result) => result,
        Err(error) => TerminalEstimationResult {
            schema: "csdlc.terminal_estimation_result.v1".into(),
            issue: terminal.issue,
            status: TerminalEstimationStatus::Invalid,
            outcome_ref: None,
            outcome_digest: None,
            detail: error.to_string(),
        },
    }
}

fn try_retain_terminal_estimation_outcome(
    store: &Store,
    terminal: &DerivedTerminalEnvelope,
) -> Result<TerminalEstimationResult> {
    let cards = store.load_cards(terminal.issue)?;
    let spp = cards
        .get(&CardKind::Spp)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "SPP card is missing"))?;
    let CardContent::Spp(spp) = &spp.content else {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "SPP projection has the wrong type",
        ));
    };
    let Some(accepted) = &spp.execution_estimates.advisory else {
        return Ok(estimation_result(
            terminal.issue,
            TerminalEstimationStatus::NotPlanned,
            "no advisory forecast was dispositioned",
        ));
    };
    validate_accepted_estimate(accepted)?;
    let forecast_artifact = ArtifactReference {
        reference: accepted.forecast_ref.clone(),
        digest: accepted.forecast_digest.clone(),
    };
    let forecast: Forecast = load_verified_json(store.root(), &forecast_artifact)?;
    if forecast.target_issue != terminal.issue {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "accepted forecast target does not match terminal issue",
        ));
    }
    if let Some(calibration) = forecast.calibration.clone() {
        let calibration = verified_calibration(store.root(), calibration)?;
        if forecast.method == EstimateMethod::ComparableMedian && !calibration.report().calibrated {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "accepted forecast references failed calibration",
            ));
        }
    }
    let terminal_ref = format!(
        "git-common:csdlc-v2/derived-terminal/{}.json",
        terminal.issue
    );
    let unknown = |name: &str| MetricObservation {
        availability: Availability::Unknown,
        value: None,
        provenance: vec![Provenance {
            source: ObservationSource::Lifecycle,
            reference: format!("{terminal_ref}#{name}"),
        }],
    };
    let fallback_actual = Observation {
        schema: OBSERVATION_SCHEMA.into(),
        issue: terminal.issue,
        key: forecast.key.clone(),
        elapsed_seconds: unknown("elapsed_seconds_unavailable"),
        active_work_seconds: unknown("active_work_seconds_unavailable"),
        validation_seconds: unknown("validation_seconds_unavailable"),
        pr_wait_seconds: unknown("pr_wait_seconds_unavailable"),
        ci_wait_seconds: unknown("ci_wait_seconds_unavailable"),
        operator_wait_seconds: unknown("operator_wait_seconds_unavailable"),
        reconnect_actions: unknown("reconnect_actions_unavailable"),
        total_tokens: unknown("total_tokens_unavailable"),
    };
    let manifest_ref = format!(
        ".csdlc/evidence/{}/terminal-observation-manifest.json",
        terminal.issue
    );
    let actual = if store.root().join(&manifest_ref).is_file() {
        let artifact = artifact_reference(store.root(), manifest_ref)?;
        load_observation_manifest(store.root(), &artifact)?
    } else {
        fallback_actual
    };
    let outcome = terminal_outcome(&forecast, forecast_artifact, &actual)?;
    let digest = canonical_digest(&outcome)?;
    let path = terminal_estimation_path(store.root(), terminal.issue)?;
    retain_terminal_estimation_file(store.root(), &path, &outcome)?;
    let retained: TerminalOutcome = serde_json::from_slice(&fs::read(&path)?)?;
    if retained != outcome || canonical_digest(&retained)? != digest {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "retained terminal estimation outcome failed reread validation",
        ));
    }
    Ok(TerminalEstimationResult {
        schema: "csdlc.terminal_estimation_result.v1".into(),
        issue: terminal.issue,
        status: if matches!(
            accepted.disposition,
            EstimateDisposition::Rejected | EstimateDisposition::Deferred
        ) {
            TerminalEstimationStatus::Deferred
        } else {
            TerminalEstimationStatus::Recorded
        },
        outcome_ref: Some(format!(
            "git-common:csdlc-v2/derived-terminal/{}.estimation.json",
            terminal.issue
        )),
        outcome_digest: Some(digest),
        detail: if matches!(
            accepted.disposition,
            EstimateDisposition::Rejected | EstimateDisposition::Deferred
        ) {
            "verified deferred advisory forecast and retained all available terminal observations"
                .into()
        } else {
            "verified advisory forecast and retained terminal outcome".into()
        },
    })
}

fn estimation_result(
    issue: u64,
    status: TerminalEstimationStatus,
    detail: impl Into<String>,
) -> TerminalEstimationResult {
    TerminalEstimationResult {
        schema: "csdlc.terminal_estimation_result.v1".into(),
        issue,
        status,
        outcome_ref: None,
        outcome_digest: None,
        detail: detail.into(),
    }
}

/// Validate the derived terminal forecast-versus-actual evidence retained by
/// `execute_finish`. This check is explicit and may fail, while finish itself
/// reports estimation failure without weakening terminal lifecycle authority.
pub fn validate_terminal_estimation_evidence(store: &Store, issue: u64) -> Result<()> {
    let cards = store.load_cards(issue)?;
    let spp = cards
        .get(&CardKind::Spp)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "SPP card is missing"))?;
    let CardContent::Spp(spp) = &spp.content else {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "SPP projection has the wrong type",
        ));
    };
    let Some(accepted) = &spp.execution_estimates.advisory else {
        return Ok(());
    };
    validate_accepted_estimate(accepted)?;
    let forecast_artifact = ArtifactReference {
        reference: accepted.forecast_ref.clone(),
        digest: accepted.forecast_digest.clone(),
    };
    let forecast: Forecast = load_verified_json(store.root(), &forecast_artifact)?;
    if let Some(calibration) = forecast.calibration.clone() {
        let calibration = verified_calibration(store.root(), calibration)?;
        if forecast.method == EstimateMethod::ComparableMedian && !calibration.report().calibrated {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "terminal forecast references failed calibration",
            ));
        }
    }
    if matches!(
        accepted.disposition,
        EstimateDisposition::Rejected | EstimateDisposition::Deferred
    ) {
        return Ok(());
    }
    let path = terminal_estimation_path(store.root(), issue)?;
    let bytes = fs::read(&path).map_err(|error| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            format!(
                "accepted advisory estimate requires derived outcome {}: {error}",
                path.display()
            ),
        )
    })?;
    let outcome: TerminalOutcome = serde_json::from_slice(&bytes).map_err(|error| {
        V2Error::new(
            ErrorCode::InvalidInput,
            format!("invalid estimation outcome: {error}"),
        )
    })?;
    if outcome.schema != OUTCOME_SCHEMA
        || outcome.issue != issue
        || outcome.forecast_ref != accepted.forecast_ref
        || outcome.forecast_digest != accepted.forecast_digest
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "terminal estimation outcome does not match the accepted advisory forecast",
        ));
    }
    Ok(())
}

fn validate_remote_merge(
    packet: &PrStatePacket,
    request: &FinishRequest,
    pr_repository: &str,
) -> Result<()> {
    if packet.repository != pr_repository
        || Some(packet.pull_request) != request.pull_request
        || packet.draft
        || !matches!(packet.merge_state.as_str(), "clean" | "unstable")
        || packet.base_ref.as_deref() != request.base.as_deref()
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
        || packet.classification != "ready"
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR is not the exact clean finish target",
        ));
    }
    for required in &request.required_checks {
        let check = packet
            .checks
            .iter()
            .find(|check| &check.name == required)
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    format!("required check {required} is missing"),
                )
            })?;
        if check.conclusion != "success" {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("required check {required} is {}", check.conclusion),
            ));
        }
    }
    if request.require_review && packet.review_decision != "approved" {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "required review approval is missing",
        ));
    }
    Ok(())
}

async fn execute_remote_merge(
    request: &FinishRequest,
    pr_repository: &str,
    token: String,
) -> Result<()> {
    let (owner, repo) = pr_repository
        .split_once('/')
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))?;
    let pull_request = request.pull_request.expect("validated PR finish request");
    let expected_head_sha = request
        .expected_head_sha
        .as_deref()
        .expect("validated PR finish request");
    let client = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(remote_merge_error)?;
    let pr = client
        .pulls(owner, repo)
        .get(pull_request)
        .await
        .map_err(remote_merge_error)?;
    if pr.head.as_ref().map(|head| head.sha.as_str()) != Some(expected_head_sha) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "PR head changed before finish merge",
        ));
    }
    if pr.merged == Some(true) || pr.merged_at.is_some() {
        return Ok(());
    }
    let response = client
        .pulls(owner, repo)
        .merge(pull_request)
        .sha(expected_head_sha)
        .method(match request.merge_method {
            MergeMethod::Merge => OctoMergeMethod::Merge,
            MergeMethod::Squash => OctoMergeMethod::Squash,
            MergeMethod::Rebase => OctoMergeMethod::Rebase,
        })
        .send()
        .await
        .map_err(remote_merge_error)?;
    if !response.merged {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "GitHub did not merge the pull request",
        ));
    }
    Ok(())
}

async fn read_issue(request: &FinishRequest) -> Result<GithubIssuePacket> {
    read_issue_in_repository(
        &request.repository,
        request.issue,
        request.token_file.clone(),
    )
    .await
}

async fn read_issue_in_repository(
    repository: &str,
    issue: u64,
    token_file: Option<String>,
) -> Result<GithubIssuePacket> {
    execute_github_action(&GithubActionRequest {
        repository: repository.to_owned(),
        action: GithubAction::IssueRead,
        operation_key: None,
        token_file,
        issue: Some(issue),
        pull_request: None,
        title: None,
        body: None,
        base: None,
        head: None,
        labels: Vec::new(),
        assignees: Vec::new(),
        milestone: None,
        state: None,
        comment_body: None,
        required_checks: Vec::new(),
        require_review: false,
        linked_issue: None,
    })
    .await?
    .issue
    .ok_or_else(|| V2Error::new(ErrorCode::RemoteFailure, "issue read returned no issue"))
}

fn issue_observation(
    issue: GithubIssuePacket,
    observed_unix_seconds: u64,
) -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: issue.state,
        labels: issue.labels,
        observed_unix_seconds,
    }
}

fn now_unix_seconds() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))
}

fn remote_merge_error(error: octocrab::Error) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub finish merge failed: {error}"),
    )
}

pub fn validate_request(request: &FinishRequest) -> Result<()> {
    if request.schema != "csdlc.finish_request.v1"
        || request.issue == 0
        || request.repository.split_once('/').is_none()
        || request.expected_digest.trim().is_empty()
        || request.actor.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "finish request identity is incomplete",
        ));
    }
    match request.pull_request {
        Some(0) => Err(V2Error::new(
            ErrorCode::InvalidInput,
            "pull request must be nonzero",
        )),
        Some(_)
            if request.base.as_deref().is_none_or(str::is_empty)
                || request.head.as_deref().is_none_or(str::is_empty)
                || request
                    .expected_head_sha
                    .as_deref()
                    .is_none_or(str::is_empty) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "PR finish requires base, head, and expected head SHA",
            ))
        }
        None if request
            .approved_no_pr_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty()) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "no-PR finish requires an approved reason",
            ))
        }
        _ => Ok(()),
    }
}

pub fn validate_canonical_identity(record: &IssueRecord, request: &FinishRequest) -> Result<()> {
    validate_request(request)?;
    if record.issue != request.issue
        || record.repository != request.repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "finish request does not match canonical issue identity or digest",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "finish requires reviewed, published, or merge_ready pre-merge truth",
        ));
    }
    if let Some(number) = request.pull_request {
        let publication = record.publication.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "PR finish requires canonical publication evidence",
            )
        })?;
        if publication.linkage_mode.unwrap_or_default()
            != crate::publication::PublicationLinkageMode::Closing
        {
            return Err(V2Error::new(
                ErrorCode::InvalidTransition,
                "part_of publication evidence cannot authorize terminal issue closeout",
            ));
        }
        if publication.repository.split_once('/').is_none()
            || publication.issue != request.issue
            || publication.pull_request != number
            || publication.base != request.base.as_deref().unwrap_or_default()
            || publication.head != request.head.as_deref().unwrap_or_default()
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical publication does not match the exact finish request",
            ));
        }
    }
    Ok(())
}

pub fn validate_publication_head_in_repo(
    root: &Path,
    record: &IssueRecord,
    request: &FinishRequest,
) -> Result<()> {
    validate_canonical_identity(record, request)?;
    let Some(expected_head) = request.expected_head_sha.as_deref() else {
        return Ok(());
    };
    if git::run(root, &["rev-parse", "HEAD"])?.stdout != expected_head
        || !git::run(
            root,
            &[
                "status",
                "--porcelain",
                "--untracked-files=all",
                "--",
                ".",
                ":(exclude).csdlc/locks/*.lock",
            ],
        )?
        .stdout
        .is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish requires the exact clean local head",
        ));
    }
    validate_publication_head_lineage_in_repo(root, record, expected_head)
}

fn validate_publication_head_lineage_in_repo(
    root: &Path,
    record: &IssueRecord,
    expected_head: &str,
) -> Result<()> {
    let publication = record.publication.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication evidence is missing",
        )
    })?;
    if expected_head.len() != 40 || !expected_head.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication head cannot prove an exact commit identity",
        ));
    }
    let published_head = parse_clean_git_revision(&publication.revision).ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication revision cannot prove exact clean commit authority",
        )
    })?;
    if published_head == expected_head {
        return Ok(());
    }
    if !matches!(
        git::metadata_only_changed_paths(root, published_head, expected_head),
        Ok(paths) if !paths.is_empty()
    ) {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "forward publication drift is not governed metadata-only change",
        ));
    }
    let review = record
        .review
        .as_ref()
        .filter(|review| review.completed)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "metadata-only publication drift requires completed review evidence",
            )
        })?;
    // The publication revision remains the lower bound for governed metadata-only
    // drift. Canonical review authority, however, must be retained by the exact
    // live PR head supplied to finish after a supported recover/review/republish.
    let historical_path = format!("{expected_head}:.csdlc/issues/{}/index.json", record.issue);
    let historical: IssueRecord = serde_json::from_str(
        &git::run(root, &["show", &historical_path])?.stdout,
    )
    .map_err(|_| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "publication commit does not retain canonical review evidence",
        )
    })?;
    if historical.issue != record.issue
        || historical.repository != record.repository
        || historical.review.as_ref() != Some(review)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "review evidence changed after the publication commit",
        ));
    }
    let reviewed_commit = parse_clean_git_revision(&review.reviewed_revision).ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "review revision cannot prove exact clean commit authority",
        )
    })?;
    if !git::substantive_scope_matches_revisions(
        root,
        reviewed_commit,
        expected_head,
        &review.scope,
    )? || git::run(
        root,
        &[
            "merge-base",
            "--is-ancestor",
            reviewed_commit,
            expected_head,
        ],
    )
    .is_err()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "substantive revision changed after publication",
        ));
    }
    Ok(())
}

fn parse_clean_git_revision(value: &str) -> Option<&str> {
    let commit = value
        .strip_prefix("git-blake3:")
        .and_then(|value| value.split_once(':'))
        .map(|(commit, _)| commit)
        .filter(|commit| {
            commit.len() == 40 && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        })?;
    (value == clean_commit_revision(commit)).then_some(commit)
}

pub fn validate_finish_merge_authority(
    root: &Path,
    record: &IssueRecord,
    request: &FinishRequest,
    _now_unix_seconds: u64,
) -> Result<()> {
    validate_canonical_identity(record, request)?;
    let branch = record.branch.as_deref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish issue has no bound branch",
        )
    })?;
    if Some(branch) != request.head.as_deref()
        || git::current_branch(root)? != branch
        || !topology_worktree_matches_root(root, record.worktree.as_deref())?
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "finish checkout does not match the canonical issue topology",
        ));
    }
    Ok(())
}

fn topology_worktree_matches_root(root: &Path, worktree: Option<&str>) -> Result<bool> {
    let Some(worktree) = worktree else {
        return Ok(false);
    };
    if worktree == "." {
        return Ok(true);
    }
    let expected = PathBuf::from(worktree);
    if expected.is_absolute() {
        return Ok(expected
            .canonicalize()
            .ok()
            .zip(root.canonicalize().ok())
            .is_some_and(|(expected, current)| expected == current));
    }
    let common_dir = PathBuf::from(
        git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    Ok(common_dir
        .parent()
        .map(|primary| primary.join(worktree))
        .and_then(|expected| expected.canonicalize().ok())
        .zip(root.canonicalize().ok())
        .is_some_and(|(expected, current)| expected == current))
}

pub fn derive_terminal(
    record: &IssueRecord,
    request: &FinishRequest,
    issue: &IssueTerminalObservation,
    packet: Option<&PrStatePacket>,
) -> Result<Option<DerivedTerminalEnvelope>> {
    validate_canonical_identity(record, request)?;
    let (disposition, pull_request, head_sha, merge_sha, pr_state) = match packet {
        Some(packet) => {
            validate_packet_identity(record, request, packet)?;
            if packet.merged && issue.state == "closed" {
                let merge_sha = packet.merge_commit_sha.clone().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "merged PR has no merge commit SHA",
                    )
                })?;
                (
                    FinishDisposition::Merged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    Some(merge_sha),
                    Some(packet.state.clone()),
                )
            } else if packet.merged {
                return Ok(None);
            } else if packet.state == "closed" && issue.state == "closed" {
                (
                    FinishDisposition::ClosedUnmerged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    None,
                    Some(packet.state.clone()),
                )
            } else {
                return Ok(None);
            }
        }
        None if issue.state == "closed"
            && issue
                .labels
                .iter()
                .any(|label| label == NO_PR_APPROVAL_LABEL) =>
        {
            (FinishDisposition::ClosedNoPr, None, None, None, None)
        }
        None if issue.state == "closed" => {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                format!("no-PR closure requires GitHub label {NO_PR_APPROVAL_LABEL}"),
            ));
        }
        None => return Ok(None),
    };
    let mut envelope = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: record.issue,
        repository: record.repository.clone(),
        initialization_digest: record.initialization_digest.clone(),
        canonical_generation: record.generation,
        canonical_digest: record.digest.clone(),
        pull_request,
        disposition,
        head_sha,
        merge_sha,
        issue_state: if disposition == FinishDisposition::Merged {
            "closed_by_merged_pr".into()
        } else {
            issue.state.clone()
        },
        pr_state,
        approved_reason: request.approved_no_pr_reason.clone(),
        observed_unix_seconds: issue.observed_unix_seconds,
        mutable_fresh_until_unix_seconds: (disposition != FinishDisposition::Merged).then(|| {
            issue
                .observed_unix_seconds
                .saturating_add(MUTABLE_TERMINAL_FRESHNESS_SECONDS)
        }),
        source: "live_github".into(),
        digest: String::new(),
    };
    envelope.digest = envelope_digest(&envelope)?;
    Ok(Some(envelope))
}

fn validate_packet_identity(
    record: &IssueRecord,
    request: &FinishRequest,
    packet: &PrStatePacket,
) -> Result<()> {
    let pr_repository = publication_repository(record, request)?;
    if packet.repository != pr_repository
        || Some(packet.pull_request) != request.pull_request
        || packet.linked_issue != Some(request.issue)
        || packet.base_ref.as_deref() != request.base.as_deref()
        || packet.head_ref.as_deref() != request.head.as_deref()
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR does not match the exact finish identity",
        ));
    }
    Ok(())
}

fn publication_repository<'a>(record: &'a IssueRecord, request: &FinishRequest) -> Result<&'a str> {
    if request.pull_request.is_none() {
        return Ok(record.repository.as_str());
    }
    record
        .publication
        .as_ref()
        .map(|publication| publication.repository.as_str())
        .filter(|repository| repository.split_once('/').is_some())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "PR finish requires a valid canonical publication repository",
            )
        })
}

pub fn validate_envelope(envelope: &DerivedTerminalEnvelope) -> Result<()> {
    if envelope.schema != "csdlc.derived_terminal.v1"
        || envelope.issue == 0
        || envelope.repository.split_once('/').is_none()
        || envelope.initialization_digest.trim().is_empty()
        || envelope.canonical_digest.trim().is_empty()
        || envelope.observed_unix_seconds == 0
        || !matches!(
            envelope.source.as_str(),
            "live_github"
                | "live_github_historical_reconciliation"
                | "live_github_recordless_closeout"
        )
        || envelope.digest != envelope_digest(envelope)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal envelope is invalid",
        ));
    }
    if (envelope.disposition == FinishDisposition::Merged
        && envelope.mutable_fresh_until_unix_seconds.is_some())
        || (envelope.disposition != FinishDisposition::Merged
            && envelope
                .mutable_fresh_until_unix_seconds
                .is_none_or(|until| until < envelope.observed_unix_seconds))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal freshness contract is invalid",
        ));
    }
    if envelope.disposition == FinishDisposition::Merged
        && (envelope.pull_request.is_none()
            || envelope.head_sha.as_deref().is_none_or(str::is_empty)
            || envelope.merge_sha.as_deref().is_none_or(str::is_empty))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "merged terminal envelope is incomplete",
        ));
    }
    if envelope.disposition == FinishDisposition::ClosedNoPr
        && envelope
            .approved_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "closed-no-PR terminal envelope has no approved reason",
        ));
    }
    Ok(())
}

pub fn envelope_matches_record(
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> Result<bool> {
    validate_envelope(envelope)?;
    Ok(envelope_matches_record_identity(envelope, record)
        && (envelope.source == "live_github_historical_reconciliation"
            || match envelope.pull_request {
                Some(_) => record.publication.as_ref().is_some_and(|publication| {
                    envelope
                        .head_sha
                        .as_deref()
                        .is_some_and(|head| publication.revision == clean_commit_revision(head))
                }),
                None => true,
            }))
}

pub fn envelope_matches_record_in_repo(
    root: &Path,
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> Result<bool> {
    validate_envelope(envelope)?;
    if !envelope_matches_record_identity(envelope, record) {
        return Ok(false);
    }
    if envelope.source == "live_github_historical_reconciliation" || envelope.pull_request.is_none()
    {
        return Ok(true);
    }
    let Some(head) = envelope.head_sha.as_deref() else {
        return Ok(false);
    };
    Ok(validate_publication_head_lineage_in_repo(root, record, head).is_ok())
}

fn envelope_matches_record_identity(
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> bool {
    envelope.issue == record.issue
        && envelope.repository == record.repository
        && envelope.initialization_digest == record.initialization_digest
        && envelope.canonical_generation == record.generation
        && envelope.canonical_digest == record.digest
        && (envelope.source == "live_github_historical_reconciliation"
            || match envelope.pull_request {
                Some(pull_request) => record
                    .publication
                    .as_ref()
                    .is_some_and(|publication| publication.pull_request == pull_request),
                None => true,
            })
}

pub fn terminal_cache_path(root: &Path, issue: u64) -> Result<PathBuf> {
    let common = crate::git::run(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .stdout;
    Ok(PathBuf::from(common)
        .join("csdlc-v2/derived-terminal")
        .join(format!("{issue}.json")))
}

fn terminal_estimation_path(root: &Path, issue: u64) -> Result<PathBuf> {
    Ok(validate_cache_parent(root, true)?.join(format!("{issue}.estimation.json")))
}

fn retain_terminal_estimation_file(
    root: &Path,
    path: &Path,
    outcome: &TerminalOutcome,
) -> Result<()> {
    if path.parent() != Some(validate_cache_parent(root, true)?.as_path()) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "terminal estimation path is outside the derived terminal cache",
        ));
    }
    if path.exists() {
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "terminal estimation outcome is not a regular file",
            ));
        }
        let existing: TerminalOutcome = serde_json::from_slice(&fs::read(path)?)?;
        if existing == *outcome {
            return Ok(());
        }
    }
    let parent = path.parent().expect("validated terminal estimation parent");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?
        .as_nanos();
    let temp = parent.join(format!(
        ".{}.{}.{}.estimation.tmp",
        outcome.issue,
        std::process::id(),
        nonce
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    serde_json::to_writer_pretty(&mut file, outcome)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(&temp, path)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

pub fn load_cached_terminal(root: &Path, issue: u64) -> Result<Option<DerivedTerminalEnvelope>> {
    let path = terminal_cache_path(root, issue)?;
    if !path.exists() {
        return Ok(None);
    }
    validate_cache_parent(root, false)?;
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache is not a regular file",
        ));
    }
    let envelope: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(path)?)?;
    validate_envelope(&envelope)?;
    if envelope.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal cache namespace mismatch",
        ));
    }
    Ok(Some(envelope))
}

pub fn retain_cached_terminal(root: &Path, envelope: &DerivedTerminalEnvelope) -> Result<PathBuf> {
    validate_envelope(envelope)?;
    let path = terminal_cache_path(root, envelope.issue)?;
    let parent = validate_cache_parent(root, true)?;
    let lock_path = parent.join(format!(".{}.cache.lock", envelope.issue));
    if fs::symlink_metadata(&lock_path)
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.file_type().is_file())
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache lock is not a regular file",
        ));
    }
    let mut lock_options = OpenOptions::new();
    lock_options
        .create(true)
        .truncate(false)
        .read(true)
        .write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        lock_options.custom_flags(libc::O_NOFOLLOW);
    }
    let lock = lock_options.open(&lock_path)?;
    lock.lock_exclusive()?;
    if path.exists() {
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(V2Error::new(
                ErrorCode::UnsafeCheckout,
                "derived terminal cache is not a regular file",
            ));
        }
        let existing: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(&path)?)?;
        validate_envelope(&existing)?;
        if existing == *envelope {
            FileExt::unlock(&lock)?;
            return Ok(path);
        }
        let historical_refresh = existing.source == "live_github_historical_reconciliation"
            && envelope.source == "live_github_historical_reconciliation"
            && existing.issue == envelope.issue
            && existing.repository == envelope.repository
            && existing.initialization_digest == envelope.initialization_digest
            && existing.pull_request == envelope.pull_request
            && existing.disposition == envelope.disposition
            && existing.head_sha == envelope.head_sha
            && existing.merge_sha == envelope.merge_sha
            && existing.issue_state == envelope.issue_state
            && existing.pr_state == envelope.pr_state
            && existing.approved_reason == envelope.approved_reason;
        if !historical_refresh
            && (existing.issue != envelope.issue
                || existing.repository != envelope.repository
                || existing.initialization_digest != envelope.initialization_digest
                || existing.canonical_generation != envelope.canonical_generation
                || existing.canonical_digest != envelope.canonical_digest
                || (existing.disposition == FinishDisposition::Merged
                    && envelope.disposition != FinishDisposition::Merged)
                || (existing.disposition == FinishDisposition::Merged
                    && existing.merge_sha != envelope.merge_sha))
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal cache conflicts with retained immutable authority",
            ));
        }
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?
        .as_nanos();
    let temp = parent.join(format!(
        ".{}.{}.{}.tmp",
        envelope.issue,
        std::process::id(),
        nonce
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    serde_json::to_writer_pretty(&mut file, envelope)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(&temp, &path)?;
    File::open(parent)?.sync_all()?;
    FileExt::unlock(&lock)?;
    Ok(path)
}

fn validate_cache_parent(root: &Path, create: bool) -> Result<PathBuf> {
    let common = PathBuf::from(
        crate::git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    let mut current = common.clone();
    for component in ["csdlc-v2", "derived-terminal"] {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            }
            Ok(_) => {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "derived terminal cache directory is not a real directory",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && create => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current)?;
                        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                            return Err(V2Error::new(
                                ErrorCode::UnsafeCheckout,
                                "concurrently created terminal cache path is unsafe",
                            ));
                        }
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(current),
            Err(error) => return Err(error.into()),
        }
    }
    let canonical_common = fs::canonicalize(&common)?;
    let canonical_parent = fs::canonicalize(&current)?;
    if !canonical_parent.starts_with(canonical_common) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache escapes the Git common directory",
        ));
    }
    Ok(current)
}

fn envelope_digest(envelope: &DerivedTerminalEnvelope) -> Result<String> {
    let mut canonical = envelope.clone();
    canonical.digest.clear();
    Ok(blake3::hash(&serde_json::to_vec(&canonical)?)
        .to_hex()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::PrCheck;
    use crate::model::{DesignReview, IssueRecord};
    use std::collections::BTreeMap;

    fn record() -> IssueRecord {
        IssueRecord {
            schema: "csdlc.issue.v2".into(),
            issue: 7,
            repository: "owner/repo".into(),
            code_repository: None,
            initialization_digest: "init".into(),
            phase: LifecyclePhase::MergeReady,
            generation: 3,
            digest: "digest".into(),
            branch: Some("codex/7".into()),
            worktree: Some(".".into()),
            review_assignment: None,
            review: None,
            publication: Some(crate::model::PublicationEvidence {
                repository: "owner/repo".into(),
                issue: 7,
                pull_request: 9,
                url: "https://example.test/pr/9".into(),
                base: "main".into(),
                head: "codex/7".into(),
                revision: clean_commit_revision("abc"),
                linkage_mode: Some(crate::publication::PublicationLinkageMode::Closing),
                draft: false,
                observed_state: "open".into(),
            }),
            readiness: None,
            terminal: None,
            migration: None,
            design_path: "design.md".into(),
            diagram_path: "diagram.mmd".into(),
            design_review: DesignReview::Approved {
                reviewer: "reviewer".into(),
                revision: "abc".into(),
            },
            cards: BTreeMap::new(),
            transitions: vec![],
            audit: vec![],
        }
    }

    fn request() -> FinishRequest {
        FinishRequest {
            schema: "csdlc.finish_request.v1".into(),
            issue: 7,
            expected_generation: 3,
            expected_digest: "digest".into(),
            actor: "agent".into(),
            repository: "owner/repo".into(),
            pull_request: Some(9),
            base: Some("main".into()),
            head: Some("codex/7".into()),
            expected_head_sha: Some("abc".into()),
            merge_method: MergeMethod::Squash,
            required_checks: vec!["ci".into()],
            require_review: true,
            approved_no_pr_reason: None,
            token_file: None,
        }
    }

    fn packet() -> PrStatePacket {
        PrStatePacket {
            schema: "csdlc.github_pr_state.v1".into(),
            repository: "owner/repo".into(),
            pull_request: 9,
            linked_issue: Some(7),
            linkage_source: Some("github".into()),
            state: "closed".into(),
            draft: false,
            merge_state: "unknown".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_ref: Some("codex/7".into()),
            head_sha: "abc".into(),
            url: None,
            body: None,
            merged: true,
            merge_commit_sha: Some("def".into()),
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: "success".into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: "merged".into(),
        }
    }

    fn issue(state: &str) -> IssueTerminalObservation {
        IssueTerminalObservation {
            state: state.into(),
            labels: Vec::new(),
            observed_unix_seconds: 100,
        }
    }

    #[test]
    fn merged_pr_derives_terminal_without_mutating_record() {
        let terminal = derive_terminal(&record(), &request(), &issue("closed"), Some(&packet()))
            .expect("derive")
            .expect("terminal");
        assert_eq!(terminal.disposition, FinishDisposition::Merged);
        assert!(envelope_matches_record(&terminal, &record()).unwrap());
    }

    #[test]
    fn open_pr_is_not_terminal() {
        let mut packet = packet();
        packet.state = "open".into();
        packet.merged = false;
        packet.merge_commit_sha = None;
        assert!(
            derive_terminal(&record(), &request(), &issue("open"), Some(&packet))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn finish_accepts_unstable_when_declared_required_checks_are_green() {
        let mut packet = packet();
        packet.state = "open".into();
        packet.merged = false;
        packet.merge_commit_sha = None;
        packet.merge_state = "unstable".into();
        packet.classification = "ready".into();
        validate_remote_merge(&packet, &request(), "owner/repo").expect("ready target");
    }

    #[test]
    fn finish_rejects_conflicts_and_exact_target_drift() {
        let mut conflicted = packet();
        conflicted.merge_state = "dirty".into();
        conflicted.classification = "conflicted".into();
        assert!(validate_remote_merge(&conflicted, &request(), "owner/repo").is_err());

        let mut draft = packet();
        draft.draft = true;
        draft.merge_state = "unstable".into();
        draft.classification = "waiting".into();
        assert!(validate_remote_merge(&draft, &request(), "owner/repo").is_err());

        let mut drifted = packet();
        drifted.head_sha = "different".into();
        drifted.merge_state = "unstable".into();
        drifted.classification = "ready".into();
        assert!(validate_remote_merge(&drifted, &request(), "owner/repo").is_err());
    }
}
