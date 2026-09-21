//! Hosted review and local-agent model gateway. Provider routing is operator-owned.
//! No client-selected paths, identities, arbitrary prompts, endpoints or credentials.
#[cfg(unix)]
pub mod control;
mod journey;
mod journey_publication;
mod publication_export;
use super::{
    activities::{self, Activity, ProviderOutput, UpdateCyclePlan},
    evidence::{store::Store, Admission, Retention},
    ingestion::Packet,
    review::{
        lanes::ReviewLane,
        runner::{self, ReviewRunOptions},
    },
};
use crate::{
    provider_adapter::execute_codefriend_invocation,
    provider_communication::{
        ProviderInvocationFinalStatusV1, ProviderInvocationRequestV1, ProviderRunLoggerV1,
    },
};
use anyhow::{ensure, Result};
use axum::{
    extract::{DefaultBodyLimit, Path as HttpPath, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Semaphore;

mod build {
    pub const REVISION: &str = env!("CODEFRIEND_BUILD_REVISION");
    pub const SOURCE_CLEAN: &str = env!("CODEFRIEND_BUILD_CLEAN");
}
/// Immutable source identity of this compiled artifact, never operator-supplied.
pub fn build_revision() -> &'static str {
    build::REVISION
}

pub const MAX_BODY: usize = 2 * 1024 * 1024;
const MAX_RESULT: usize = 4 * 1024 * 1024;
const MAX_LEGACY_PROMPT_BYTES: usize = 128 * 1024;
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 80
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Hosted,
    LocalModel,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credential {
    pub token_hash: String,
    pub subject: String,
    /// Local-agent credentials have only local_model scope; website tokens only hosted.
    pub mode: Mode,
    pub expires_at: u64,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub root: PathBuf,
    pub credentials_file: PathBuf,
    pub provider: ProviderInvocationRequestV1,
    pub candidate_revision: String,
    pub max_concurrent: usize,
    /// Persistent total per subject in this service store, including failed attempts.
    pub max_operations_per_subject: usize,
    pub retention_seconds: u64,
}
/// Explicit model-lane generation. Omission preserves legacy request digests.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewGeneration {
    Assessments,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Submit {
    pub operation_id: String,
    pub packet: Packet,
    pub mode: Mode,
    pub lane: Option<ReviewLane>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_generation: Option<ReviewGeneration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cycle: Option<UpdateCyclePlan>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Running,
    CancelRequested,
    Complete,
    Failed,
    Cancelled,
    Interrupted,
    Expired,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub operation_id: String,
    pub subject: String,
    pub mode: Mode,
    pub request_digest: String,
    pub packet_id: String,
    pub source_revision: String,
    pub candidate_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_identity: Option<crate::model_identity::ModelIdentityV1>,
    pub expires_at: u64,
    pub status: Status,
}

/// The production implementation below uses the existing review and provider owners.
/// Injected backends support deterministic protocol tests, not product acceptance.
pub trait Backend: Send + Sync + 'static {
    fn execute(
        &self,
        config: &Config,
        request: &Submit,
        admission: Admission,
        dir: &Path,
    ) -> Result<Value>;
}
pub struct ProductionBackend;
impl Backend for ProductionBackend {
    fn execute(
        &self,
        config: &Config,
        request: &Submit,
        admission: Admission,
        dir: &Path,
    ) -> Result<Value> {
        let work = dir.join("work");
        fs::create_dir_all(&work)?;
        let assessments = (request.mode == Mode::Hosted && request.cycle.is_none())
            || request.review_generation == Some(ReviewGeneration::Assessments);
        // Bound model input before the first provider effect, including hosted lanes.
        for lane in ReviewLane::ALL.into_iter().filter(|_| {
            request
                .cycle
                .as_ref()
                .is_none_or(|plan| plan.activities.contains(&Activity::Review))
        }) {
            let (_, prompt) = if assessments {
                runner::assessment_lane_input_manifest(&request.operation_id, lane, &admission)?
            } else {
                runner::lane_input_manifest(&request.operation_id, lane, &admission)?
            };
            let limit = if assessments {
                runner::MAX_ASSESSMENT_PROMPT_BYTES
            } else {
                MAX_LEGACY_PROMPT_BYTES
            };
            ensure!(prompt.len() <= limit, "model_prompt_byte_limit");
        }
        match request.mode {
            Mode::Hosted => {
                if request.cycle.is_none() {
                    // Preserve the original review-only protocol byte shape.
                    let run = runner::run(
                        ReviewRunOptions {
                            store: work.join("evidence"),
                            packet_id: admission.packet.packet_id.clone(),
                            provider_request: config.provider.clone(),
                            out: work.join("review"),
                            run_id: request.operation_id.clone(),
                            cancel_file: Some(dir.join("cancel")),
                        },
                        admission,
                    )?;
                    ensure!(
                        run.successful_execution().is_ok(),
                        "hosted_review_incomplete"
                    );
                    return Ok(serde_json::to_value(run)?);
                }
                let (result, _) = execute_cycle(config, request, admission, dir, &work)?;
                Ok(serde_json::to_value(result)?)
            }
            Mode::LocalModel => {
                if request.cycle.is_some() {
                    let gateway_admission = admission.clone();
                    let (result, model_identity) =
                        execute_cycle(config, request, admission, dir, &work)?;
                    return Ok(json!({
                        "schema":"codefriend.local_cycle_result.v1",
                        "execution_location":"local_agent",
                        "model_execution_location":"agent_logic_provider",
                        "candidate_revision":build::REVISION,
                        "model_identity":model_identity,
                        "admission":gateway_admission,
                        "cycle_result":result
                    }));
                }
                let lane = request
                    .lane
                    .ok_or_else(|| anyhow::anyhow!("lane_required"))?;
                let (manifest, prompt) = if assessments {
                    runner::assessment_lane_input_manifest(&request.operation_id, lane, &admission)?
                } else {
                    runner::lane_input_manifest(&request.operation_id, lane, &admission)?
                };
                let mut provider = config.provider.clone();
                provider.input_text = Some(prompt);
                provider.run_id = Some(request.operation_id.clone());
                provider.request_id = Some(format!("{}-{}", request.operation_id, lane.id()));
                provider.lane_ref = lane.id().into();
                provider.prompt_contract_ref = manifest.prompt_contract.clone();
                let mut logger = ProviderRunLoggerV1::create(
                    work.join("provider.jsonl"),
                    &request.operation_id,
                )?;
                let result = execute_codefriend_invocation(provider, &mut logger);
                crate::provider_adapter::retain_codefriend_provider_outcome(&result, || {
                    write_json(&work.join("provider-result.json"), &result)
                })?;
                ensure!(
                    result.final_status == ProviderInvocationFinalStatusV1::Ok,
                    "model_request_failed"
                );
                let output = result
                    .output_text
                    .ok_or_else(|| anyhow::anyhow!("model_output_missing"))?;
                ensure!(output.len() <= MAX_RESULT, "model_output_too_large");
                let parsed = if assessments {
                    super::evidence::assessments::parse_lane(lane.id(), &output, &admission)?;
                    serde_json::to_value(serde_json::from_str::<
                        super::evidence::assessments::ProviderAssessmentOutput,
                    >(&output)?)?
                } else {
                    let parsed = runner::parse_lane_output(lane, &output, &admission)?;
                    for finding in &parsed.findings {
                        runner::finding_from_lane(lane, &admission, finding.clone())?;
                    }
                    serde_json::to_value(parsed)?
                };
                Ok(
                    json!({"schema":if assessments { "codefriend.local_model_result.v2" } else { "codefriend.local_model_result.v1" }, "execution_location":"local_agent",
                    "model_execution_location":"agent_logic_provider", "candidate_revision":build::REVISION,
                    "model_identity":result.model_identity, "input_manifest":manifest,"output":parsed}),
                )
            }
        }
    }
}

fn execute_cycle(
    config: &Config,
    request: &Submit,
    admission: Admission,
    dir: &Path,
    work: &Path,
) -> Result<(
    activities::UpdateCycleResult,
    crate::model_identity::ModelIdentityV1,
)> {
    let plan = request
        .cycle
        .clone()
        .ok_or_else(|| anyhow::anyhow!("activity_plan_required"))?;
    plan.validate(&admission)?;
    let route = runner::provider_route_identity(&config.provider);
    let cancel = dir.join("cancel");
    let mut identities = Vec::new();
    let review = if plan.activities.contains(&Activity::Review) {
        let assessments = request.review_generation == Some(ReviewGeneration::Assessments);
        let run = if assessments {
            runner::run_assessments_with_executor
        } else {
            runner::run_with_executor
        };
        match run(
            runner::ExecutionOptions {
                out: work.join("review"),
                run_id: request.operation_id.clone(),
                cancel_file: Some(dir.join("cancel")),
            },
            admission.clone(),
            route.clone(),
            |lane: ReviewLane, prompt: String, lane_dir: &Path| {
                ensure!(!cancel.exists(), "cancelled");
                let mut provider = config.provider.clone();
                provider.input_text = Some(prompt);
                provider.run_id = Some(request.operation_id.clone());
                provider.request_id = Some(format!("{}-{}", request.operation_id, lane.id()));
                provider.lane_ref = lane.id().into();
                provider.prompt_contract_ref = format!(
                    "{}:{}",
                    if assessments {
                        runner::PROMPT_CONTRACT_V2
                    } else {
                        runner::PROMPT_CONTRACT
                    },
                    lane.id()
                );
                let mut logger = ProviderRunLoggerV1::create_with_context(
                    lane_dir.join("provider.log.jsonl"),
                    &request.operation_id,
                    provider.request_id.clone(),
                    Some(format!("lanes/{}/provider.log.jsonl", lane.id())),
                )?;
                let output = execute_codefriend_invocation(provider, &mut logger);
                crate::provider_adapter::retain_codefriend_provider_outcome(&output, || {
                    write_json(&lane_dir.join("provider-result.json"), &output)
                })?;
                identities.push(output.model_identity.clone());
                Ok(runner::LaneExecution {
                    final_status: output.final_status,
                    output_text: output.output_text,
                })
            },
        ) {
            Ok(review) => Some(review),
            Err(error) if error.is::<crate::provider_adapter::CodeFriendProviderInterrupted>() => {
                return Err(error);
            }
            Err(_) => None,
        }
    } else {
        None
    };
    let mut result = activities::run_with_executor(
        plan,
        admission,
        request.operation_id.clone(),
        route,
        review,
        |activity, prompt, _manifest| {
            ensure!(!cancel.exists(), "cancelled");
            let mut provider = config.provider.clone();
            provider.input_text = Some(prompt);
            provider.run_id = Some(request.operation_id.clone());
            provider.request_id = Some(format!("{}-{}", request.operation_id, activity.id()));
            provider.lane_ref = format!("activity:{}", activity.id());
            provider.prompt_contract_ref =
                format!("{}:{}", activities::PROMPT_CONTRACT, activity.id());
            let mut logger = ProviderRunLoggerV1::create_with_context(
                work.join(format!("activity-{}.jsonl", activity.id())),
                &request.operation_id,
                provider.request_id.clone(),
                Some(format!("activity-{}.jsonl", activity.id())),
            )?;
            let output = execute_codefriend_invocation(provider, &mut logger);
            crate::provider_adapter::retain_codefriend_provider_outcome(&output, || {
                write_json(
                    &work.join(format!("activity-{}-provider-result.json", activity.id())),
                    &output,
                )
            })?;
            identities.push(output.model_identity.clone());
            Ok(ProviderOutput {
                final_status: output.final_status,
                output_text: output.output_text,
            })
        },
    )?;
    let first = identities
        .first()
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("activity_model_identity_missing"))?;
    ensure!(
        identities
            .iter()
            .all(|identity| same_model_execution(&first, identity)),
        "activity_model_identity_changed"
    );
    let observed_route = runner::provider_route_identity_from_model(&first);
    for activity in &mut result.activities {
        activity.provider_route.clone_from(&observed_route);
    }
    if let Some(review) = &mut result.review {
        review.rebind_provider_route(&observed_route)?;
        if let Some(activity) = result
            .activities
            .iter_mut()
            .find(|activity| activity.activity == Activity::Review)
        {
            activity.review_result_digest = Some(super::evidence::hash(review)?);
        }
    }
    result.execution = Some(activities::CycleExecutionBinding {
        candidate_revision: build::REVISION.into(),
        request_digest: super::evidence::hash(request)?,
        model_identity: first.clone(),
    });
    result.validate(&observed_route)?;
    Ok((result, first))
}

fn same_model_execution(
    left: &crate::model_identity::ModelIdentityV1,
    right: &crate::model_identity::ModelIdentityV1,
) -> bool {
    left.provider_kind == right.provider_kind
        && left.provider == right.provider
        && left.model_ref == right.model_ref
        && left.provider_model_id == right.provider_model_id
        && left.runtime_surface == right.runtime_surface
        && left.identity_strength == right.identity_strength
        && left.resolved_digest == right.resolved_digest
}
struct Inner {
    config: Config,
    backend: Arc<dyn Backend>,
    gate: Mutex<bool>,
    slots: Arc<Semaphore>,
    // Retained for service lifetime; rejects a second process using this store.
    _lock: File,
}
#[derive(Clone)]
pub struct Service(Arc<Inner>);
#[derive(Debug)]
struct ApiError(StatusCode, &'static str);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({"error":self.1}))).into_response()
    }
}
type ApiResult<T> = std::result::Result<T, ApiError>;
fn internal<T>(r: Result<T>) -> ApiResult<T> {
    r.map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_storage_failure"))
}
fn read_json<T: serde::de::DeserializeOwned>(path: &Path, max: usize) -> Result<T> {
    let mut b = Vec::new();
    File::open(path)?.take(max as u64 + 1).read_to_end(&mut b)?;
    ensure!(b.len() <= max, "file_too_large");
    Ok(serde_json::from_slice(&b)?)
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    let tmp = path.with_extension("tmp");
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&tmp)?;
    f.write_all(&bytes)?;
    f.sync_all()?;
    fs::rename(tmp, path)?;
    File::open(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("parent_missing"))?,
    )?
    .sync_all()?;
    Ok(())
}
impl Service {
    pub fn open(config: Config, backend: Arc<dyn Backend>) -> Result<Self> {
        ensure!(
            (1..=8).contains(&config.max_concurrent),
            "invalid_concurrency"
        );
        ensure!(
            (1..=1000).contains(&config.max_operations_per_subject),
            "invalid_operation_quota"
        );
        ensure!(
            (60..=86400).contains(&config.retention_seconds),
            "invalid_retention"
        );
        ensure!(
            build::SOURCE_CLEAN == "true" && !build::REVISION.is_empty(),
            "build_provenance_unavailable"
        );
        ensure!(
            config.candidate_revision == build::REVISION,
            "candidate_revision_mismatch"
        );
        ensure!(
            config
                .provider
                .input_text
                .as_deref()
                .unwrap_or_default()
                .is_empty(),
            "preloaded_prompt_forbidden"
        );
        ensure!(
            config.provider.attempt_policy.max_attempts == 1,
            "bounded_provider_attempt_required"
        );
        ensure!(
            config
                .provider
                .max_output_tokens
                .is_some_and(|n| (1..=4096).contains(&n)),
            "bounded_output_required"
        );
        ensure!(config.root.is_absolute(), "absolute_private_store_required");
        let parent = config
            .root
            .parent()
            .ok_or_else(|| anyhow::anyhow!("store_parent_required"))?;
        ensure!(parent.is_dir(), "existing_store_parent_required");
        if !config.root.exists() {
            fs::create_dir(&config.root)?;
        }
        File::open(parent)?.sync_all()?;
        fs::create_dir_all(config.root.join("operations"))?;
        File::open(&config.root)?.sync_all()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&config.root, fs::Permissions::from_mode(0o700))?;
        }
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(config.root.join("service.lock"))?;
        fs2::FileExt::try_lock_exclusive(&lock)?;
        let service = Self(Arc::new(Inner {
            slots: Arc::new(Semaphore::new(config.max_concurrent)),
            config,
            backend,
            gate: Mutex::new(false),
            _lock: lock,
        }));
        service.credentials()?;
        // Never infer that an interrupted provider operation had no effect.
        for user in fs::read_dir(service.0.config.root.join("operations"))? {
            for entry in fs::read_dir(user?.path())? {
                let dir = entry?.path();
                let path = dir.join("operation.json");
                if !path.exists() {
                    continue;
                } // An interrupted reservation stays consumed.
                let mut op: Operation = read_json(&path, 16384)?;
                if matches!(op.status, Status::Running | Status::CancelRequested) {
                    op.status = Status::Interrupted;
                    write_json(&path, &op)?;
                }
            }
        }
        service.expire()?;
        Ok(service)
    }
    /// Freeze new reservations under the same gate used by submit. Existing work
    /// and observations continue; the host must separately drain the website.
    pub fn begin_drain(&self) -> Result<()> {
        *self
            .0
            .gate
            .lock()
            .map_err(|_| anyhow::anyhow!("gate_poisoned"))? = true;
        Ok(())
    }
    /// Only the local operator control path may resume admissions after a failed stop.
    pub fn resume_admissions(&self) -> Result<()> {
        *self
            .0
            .gate
            .lock()
            .map_err(|_| anyhow::anyhow!("gate_poisoned"))? = false;
        Ok(())
    }
    /// A point-in-time local guard, not authorization to power off the host.
    /// Keep admissions frozen until service stop; website and host guards are
    /// still required. Errors and uncertain reservations never imply quiescence.
    pub fn drained_without_payloads(&self) -> Result<bool> {
        self.payloads_quiescent(true)
    }
    /// Advisory preflight only: avoids draining while known work or retained
    /// results remain. Admissions are still open and the frozen guard must be
    /// checked again before stopping the service.
    pub fn quiescent_without_payloads(&self) -> Result<bool> {
        self.payloads_quiescent(false)
    }
    fn payloads_quiescent(&self, require_drain: bool) -> Result<bool> {
        self.expire()?;
        let draining = self
            .0
            .gate
            .lock()
            .map_err(|_| anyhow::anyhow!("gate_poisoned"))?;
        if (require_drain && !*draining)
            || self.0.slots.available_permits() != self.0.config.max_concurrent
        {
            return Ok(false);
        }
        for user in fs::read_dir(self.0.config.root.join("operations"))? {
            for entry in fs::read_dir(user?.path())? {
                let dir = entry?.path();
                let path = dir.join("operation.json");
                if !path.exists() {
                    return Ok(false);
                }
                let op: Operation = read_json(&path, 16384)?;
                if matches!(
                    op.status,
                    Status::Running | Status::CancelRequested | Status::Interrupted
                ) {
                    return Ok(false);
                }
                // Retain unexpired results until their existing deadline. expire()
                // removes expired payloads and keeps no-replay operation identities.
                if dir.join("work").exists()
                    || dir.join("result.json").exists()
                    || dir.join("result.tmp").exists()
                {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    fn credentials(&self) -> Result<Vec<Credential>> {
        let credentials: Vec<Credential> = read_json(&self.0.config.credentials_file, 128 * 1024)?;
        // Fresh website startup and revocation of the last credential publish an
        // empty registry. It is a valid deny-all state, not a startup failure.
        ensure!(credentials.len() <= 512, "invalid_credentials_registry");
        let mut hashes = BTreeSet::new();
        for c in &credentials {
            ensure!(
                id(&c.subject)
                    && c.token_hash.len() == 64
                    && c.token_hash.bytes().all(|b| b.is_ascii_hexdigit())
                    && hashes.insert(&c.token_hash),
                "invalid_credentials_registry"
            );
        }
        Ok(credentials)
    }
    fn auth(&self, headers: &HeaderMap) -> ApiResult<Credential> {
        let token = headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .filter(|s| (32..=256).contains(&s.len()))
            .ok_or(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"))?;
        let digest = blake3::hash(token.as_bytes()).to_hex().to_string();
        let credentials = internal(self.credentials())?;
        credentials
            .into_iter()
            .find(|c| c.token_hash == digest && c.expires_at > now())
            .ok_or(ApiError(StatusCode::UNAUTHORIZED, "unauthorized"))
    }
    fn dir(&self, subject: &str, operation: &str) -> PathBuf {
        self.0
            .config
            .root
            .join("operations")
            .join(subject)
            .join(operation)
    }
    fn operation(&self, credential: &Credential, operation: &str) -> ApiResult<Operation> {
        if !id(operation) {
            return Err(ApiError(StatusCode::NOT_FOUND, "operation_not_found"));
        }
        let dir = self.dir(&credential.subject, operation);
        let op: Operation = read_json(&dir.join("operation.json"), 16384)
            .map_err(|_| ApiError(StatusCode::NOT_FOUND, "operation_not_found"))?;
        if op.subject != credential.subject || op.mode != credential.mode {
            return Err(ApiError(StatusCode::NOT_FOUND, "operation_not_found"));
        }
        Ok(op)
    }
    /// Periodically removes source, provider logs and result bytes; identity tombstones remain.
    pub fn expire(&self) -> Result<()> {
        let _guard = self
            .0
            .gate
            .lock()
            .map_err(|_| anyhow::anyhow!("gate_poisoned"))?;
        for user in fs::read_dir(self.0.config.root.join("operations"))? {
            for entry in fs::read_dir(user?.path())? {
                let dir = entry?.path();
                let path = dir.join("operation.json");
                if !path.exists() {
                    continue;
                }
                let mut op: Operation = read_json(&path, 16384)?;
                if op.expires_at > now() {
                    continue;
                }
                if matches!(op.status, Status::Running | Status::CancelRequested) {
                    File::create(dir.join("cancel"))?.sync_all()?;
                    continue;
                }
                if dir.join("work").exists() {
                    fs::remove_dir_all(dir.join("work"))?;
                }
                for name in ["result.json", "result.tmp"] {
                    if dir.join(name).exists() {
                        fs::remove_file(dir.join(name))?;
                    }
                }
                // Expiry removes payloads, not uncertainty about an interrupted
                // provider dispatch. Preserve that tombstone across restart so
                // automatic host shutdown cannot infer a reconciled outcome.
                if op.status != Status::Interrupted {
                    op.status = Status::Expired;
                }
                write_json(&path, &op)?;
            }
        }
        Ok(())
    }
    pub fn router(self) -> Router {
        Router::new()
            .route("/v1/operations", post(submit))
            .route("/v1/operations/:operation", get(inspect))
            .route("/v1/operations/:operation/cancel", post(cancel))
            .route("/v1/operations/:operation/result", get(result))
            .route(
                "/v1/operations/:operation/publication/challenge",
                post(publication_challenge),
            )
            .route(
                "/v1/operations/:operation/publication/decision",
                post(publication_decision),
            )
            .route(
                "/v1/operations/:operation/publication/result",
                get(publication_result),
            )
            .route(
                "/v1/operations/:operation/publication/render",
                post(publication_export::render),
            )
            .route(
                "/v1/operations/:operation/journey",
                get(journey::status).post(journey::prepare),
            )
            .route(
                "/v1/operations/:operation/journey/step",
                post(journey::step),
            )
            .route(
                "/v1/operations/:operation/journey/graph",
                get(journey::graph),
            )
            .route(
                "/v1/operations/:operation/journey/publication",
                post(journey_publication::attach),
            )
            .route(
                "/v1/operations/:operation/journey/artifacts/:stage",
                get(journey::artifact),
            )
            .route_layer(axum::middleware::from_fn_with_state(
                self.clone(),
                authorize,
            ))
            .layer(axum::middleware::map_response(
                |mut response: Response| async move {
                    response.headers_mut().insert(
                        axum::http::header::CACHE_CONTROL,
                        axum::http::HeaderValue::from_static("no-store"),
                    );
                    response
                },
            ))
            .layer(DefaultBodyLimit::max(MAX_BODY))
            .with_state(self)
    }
}
async fn authorize(
    State(service): State<Service>,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    match service.auth(request.headers()) {
        Ok(credential) => {
            request.extensions_mut().insert(credential);
            next.run(request).await
        }
        Err(error) => error.into_response(),
    }
}
async fn submit(
    State(service): State<Service>,
    headers: HeaderMap,
    Json(request): Json<Submit>,
) -> ApiResult<(StatusCode, Json<Operation>)> {
    // Middleware rejects unauthenticated streams before polling their bodies.
    // Recheck after the body await: a slow upload must not retain revoked authority.
    let credential = service.auth(&headers)?;
    if credential.mode != request.mode {
        return Err(ApiError(StatusCode::FORBIDDEN, "scope_denied"));
    }
    let shape_valid = match request.mode {
        Mode::Hosted => request.lane.is_none(),
        Mode::LocalModel => {
            (request.lane.is_some() && request.cycle.is_none())
                || (request.lane.is_none() && request.cycle.is_some())
        }
    };
    if !id(&request.operation_id) || !shape_valid {
        return Err(ApiError(StatusCode::BAD_REQUEST, "invalid_operation"));
    }
    let admission = Admission::new(
        request.packet.clone(),
        Retention {
            seconds: service.0.config.retention_seconds,
        },
        now(),
    )
    .map_err(|_| ApiError(StatusCode::BAD_REQUEST, "invalid_evidence"))?;
    if let Some(plan) = &request.cycle {
        plan.validate(&admission)
            .map_err(|_| ApiError(StatusCode::BAD_REQUEST, "invalid_activity_plan"))?;
    }
    let permit = service
        .0
        .slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError(StatusCode::TOO_MANY_REQUESTS, "capacity_exhausted"))?;
    let guard = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    if *guard {
        return Err(ApiError(
            StatusCode::SERVICE_UNAVAILABLE,
            "service_draining",
        ));
    }
    let user = service
        .0
        .config
        .root
        .join("operations")
        .join(&credential.subject);
    internal(fs::create_dir_all(&user).map_err(Into::into))?;
    internal(
        File::open(service.0.config.root.join("operations"))
            .and_then(|f| f.sync_all())
            .map_err(Into::into),
    )?;
    let dir = service.dir(&credential.subject, &request.operation_id);
    if dir.exists() {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "operation_id_reserved_do_not_replay",
        ));
    }
    let used = internal(
        fs::read_dir(&user)
            .map(|entries| entries.count())
            .map_err(Into::into),
    )?;
    if used >= service.0.config.max_operations_per_subject {
        return Err(ApiError(
            StatusCode::TOO_MANY_REQUESTS,
            "subject_quota_exhausted",
        ));
    }
    // Durable reservation precedes any provider effect. Even an incomplete reservation is not replayable.
    internal(fs::create_dir(&dir).map_err(Into::into))?;
    internal(
        File::open(&user)
            .and_then(|f| f.sync_all())
            .map_err(Into::into),
    )?;
    let operation = Operation {
        operation_id: request.operation_id.clone(),
        subject: credential.subject,
        mode: request.mode.clone(),
        request_digest: internal(super::evidence::hash(&request))?,
        packet_id: request.packet.packet_id.clone(),
        source_revision: request.packet.revision.clone(),
        candidate_revision: build::REVISION.into(),
        model_identity: None,
        expires_at: admission.expires_at,
        status: Status::Running,
    };
    internal(write_json(&dir.join("operation.json"), &operation))?;
    if request.cycle.is_some() {
        internal(
            File::create(dir.join("cycle-operation"))
                .and_then(|file| file.sync_all())
                .and_then(|_| File::open(&dir)?.sync_all())
                .map_err(Into::into),
        )?;
    }
    drop(guard);
    let returned = operation.clone();
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        let work = dir.join("work");
        let outcome = (|| -> Result<Value> {
            if dir.join("cancel").exists() {
                anyhow::bail!("cancelled");
            }
            let store = Store::open(&work.join("evidence"), now)?;
            let admitted = store.admit(
                request.packet.clone(),
                Retention {
                    seconds: service.0.config.retention_seconds,
                },
            )?;
            service
                .0
                .backend
                .execute(&service.0.config, &request, admitted, &dir)
        })();
        if let Ok(_guard) = service.0.gate.lock() {
            let mut op = operation;
            op.status = if let Some(status) = stopped_provider_status(
                &outcome,
                dir.join("cancel").exists(),
                op.expires_at <= now(),
            ) {
                status
            } else if let Ok(value) = outcome {
                match serde_json::to_vec(&value) {
                    Ok(bytes) if bytes.len() <= MAX_RESULT => {
                        if write_json(&dir.join("result.json"), &value).is_ok() {
                            op.model_identity = value
                                .get("model_identity")
                                .or_else(|| value.get("execution")?.get("model_identity"))
                                .or_else(|| {
                                    value
                                        .get("cycle_result")?
                                        .get("execution")?
                                        .get("model_identity")
                                })
                                .and_then(|identity| serde_json::from_value(identity.clone()).ok());
                            let failed_cycle = request.cycle.is_some()
                                && (value
                                    .get("completion")
                                    .or_else(|| value.get("cycle_result")?.get("completion"))
                                    .and_then(Value::as_str)
                                    == Some("failed"));
                            if dir.join("cancel").exists() || op.expires_at <= now() {
                                Status::Cancelled
                            } else if failed_cycle {
                                Status::Failed
                            } else {
                                Status::Complete
                            }
                        } else {
                            Status::Failed
                        }
                    }
                    _ => Status::Failed,
                }
            } else if dir.join("cancel").exists() || op.expires_at <= now() {
                Status::Cancelled
            } else {
                Status::Failed
            };
            // Storage failure leaves Running durable, which restart classifies Interrupted.
            let _ = write_json(&dir.join("operation.json"), &op);
            eprintln!(
                "adl_event component=codefriend_server event=operation_terminal status={:?}",
                op.status
            );
        }
        let _ = service.expire();
    });
    Ok((StatusCode::ACCEPTED, Json(returned)))
}
async fn inspect(
    State(service): State<Service>,
    Extension(c): Extension<Credential>,
    HttpPath(operation): HttpPath<String>,
) -> ApiResult<Json<Operation>> {
    internal(service.expire())?;
    Ok(Json(service.operation(&c, &operation)?))
}
async fn cancel(
    State(service): State<Service>,
    Extension(c): Extension<Credential>,
    HttpPath(operation): HttpPath<String>,
) -> ApiResult<Json<Operation>> {
    let _guard = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    let mut op = service.operation(&c, &operation)?;
    if matches!(op.status, Status::Running | Status::CancelRequested) {
        let dir = service.dir(&c.subject, &operation);
        internal(
            File::create(dir.join("cancel"))
                .and_then(|f| f.sync_all())
                .map_err(Into::into),
        )?;
        op.status = Status::CancelRequested;
        internal(write_json(&dir.join("operation.json"), &op))?;
    }
    Ok(Json(op))
}
async fn result(
    State(service): State<Service>,
    Extension(c): Extension<Credential>,
    HttpPath(operation): HttpPath<String>,
) -> ApiResult<Json<Value>> {
    internal(service.expire())?;
    let op = service.operation(&c, &operation)?;
    let result_path = service.dir(&c.subject, &operation).join("result.json");
    let cycle_result = service
        .dir(&c.subject, &operation)
        .join("cycle-operation")
        .exists();
    if !(op.status == Status::Complete
        || (cycle_result && matches!(op.status, Status::Failed | Status::Cancelled)))
        || op.expires_at <= now()
        || !result_path.exists()
    {
        return Err(ApiError(StatusCode::CONFLICT, "result_not_complete"));
    }
    Ok(Json(internal(read_json(&result_path, MAX_RESULT))?))
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct PublicationSelection {
    #[serde(default)]
    format: super::integration::PublicationFormat,
}
fn publication_directory(
    service: &Service,
    subject: &str,
    operation: &str,
    format: super::integration::PublicationFormat,
) -> PathBuf {
    service
        .dir(subject, operation)
        .join("work/publications")
        .join(format.key())
}

/// Prepare only from this service's authenticated, complete hosted operation.
/// Local-agent publication authority remains at the paired local artifact owner.
async fn publication_challenge(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(selection): Json<PublicationSelection>,
) -> ApiResult<Json<Value>> {
    use super::{
        evidence::contracts::{Publication, ReviewRecord},
        integration::{prepare_publication_bundle_for_format_v2, PublicationChallenge},
        publication::{read_decision_head, verify_artifacts},
        review::runner::FourPerspectiveReviewRun,
    };
    let _guard = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    // Re-read the live registry inside the operation boundary; a middleware
    // credential snapshot must not survive revocation while waiting for this lock.
    let credential = service.auth(&headers)?;
    let op = service.operation(&credential, &operation)?;
    let issued_at = now();
    if credential.mode != Mode::Hosted
        || op.status != Status::Complete
        || op.expires_at <= issued_at
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_requires_complete_hosted_operation",
        ));
    }
    if op.candidate_revision != build::REVISION {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_candidate_changed",
        ));
    }
    let dir = service.dir(&credential.subject, &operation);
    let run: FourPerspectiveReviewRun = internal(read_json(&dir.join("result.json"), MAX_RESULT))?;
    let review_path = dir.join("work/review/review-record.json");
    let review: ReviewRecord = internal(read_json(&review_path, MAX_RESULT))?;
    if run.successful_execution().is_err()
        || run.run_id != operation
        || run.review_record != review
        || review.run.packet_id != op.packet_id
        || review.run.revision != op.source_revision
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_operation_identity_mismatch",
        ));
    }
    let destination = dir.join("work/exports");
    internal(fs::create_dir_all(&destination).map_err(Into::into))?;
    internal(fs::create_dir_all(dir.join("work/publications")).map_err(Into::into))?;
    let bundle = publication_directory(&service, &credential.subject, &operation, selection.format);
    let publication: Publication = if bundle.exists() {
        internal(read_json(&bundle.join("publication.json"), MAX_RESULT))?
    } else {
        internal(prepare_publication_bundle_for_format_v2(
            &review_path,
            &bundle,
            &destination,
            selection.format,
        ))?
    };
    internal(publication.validate(&review))?;
    internal(verify_artifacts(
        &bundle.join("artifacts"),
        &publication.artifact_manifest,
    ))?;
    let head = internal(read_decision_head(
        &bundle.join("approval"),
        &review,
        &publication,
    ))?;
    let expected = head.as_ref().map(|record| record.digest.as_str());
    let expires_at = op
        .expires_at
        .min(credential.expires_at)
        .min(issued_at + 300);
    let challenge = internal(PublicationChallenge::prepare(
        &credential.subject,
        &operation,
        build::REVISION,
        &review,
        &publication,
        &bundle.join("artifacts"),
        expected,
        issued_at,
        op.expires_at
            .min(credential.expires_at)
            .min(issued_at + 300),
    ))?;
    internal(write_json(&bundle.join("challenge.json"), &challenge))?;
    Ok(Json(json!({
        "schema":"codefriend.publication_challenge_response.v1",
        "format":selection.format,"expires_at":expires_at,
        "challenge_digest":challenge.digest(),
        "binding_digest":internal(publication.binding_digest())?,
        "expected_decision_digest":expected,
        "candidate_revision":build::REVISION,
        "challenge":challenge,
        "approval_status":"not_approved"
    })))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WebsiteDecisionRequest {
    #[serde(default)]
    format: super::integration::PublicationFormat,
    challenge_digest: String,
    binding_digest: String,
    expected_decision_digest: Option<String>,
    decision: super::publication::DecisionKind,
}

/// A live service authentication boundary, never deserialized or publicly
/// constructible. The approval owner rechecks it while holding its append lock.
pub(crate) struct WebsiteDecisionAuthority<'a> {
    service: &'a Service,
    headers: &'a HeaderMap,
    operation: &'a str,
    request: &'a WebsiteDecisionRequest,
    challenge: &'a super::integration::PublicationChallenge,
}
impl WebsiteDecisionAuthority<'_> {
    pub(crate) fn verify(
        &self,
        review: &super::evidence::contracts::ReviewRecord,
        publication: &super::evidence::contracts::Publication,
        current_head: Option<&str>,
    ) -> Result<(super::publication::approval::WebsiteDecisionProvenance, u64)> {
        let credential = self
            .service
            .auth(self.headers)
            .map_err(|_| anyhow::anyhow!("unauthorized"))?;
        let op = self
            .service
            .operation(&credential, self.operation)
            .map_err(|_| anyhow::anyhow!("operation_not_found"))?;
        let time = now();
        ensure!(
            credential.mode == Mode::Hosted
                && op.status == Status::Complete
                && op.expires_at > time
                && op.candidate_revision == build::REVISION,
            "publication_operation_not_current"
        );
        ensure!(
            review.run.packet_id == op.packet_id && review.run.revision == op.source_revision,
            "publication_operation_identity_mismatch"
        );
        ensure!(
            current_head == self.request.expected_decision_digest.as_deref(),
            "decision_head_changed"
        );
        self.challenge.verify_response(
            &credential.subject,
            self.operation,
            build::REVISION,
            &self.request.challenge_digest,
            &self.request.binding_digest,
            current_head,
            review,
            publication,
            &publication_directory(
                self.service,
                &credential.subject,
                self.operation,
                self.request.format,
            )
            .join("artifacts"),
            time,
        )?;
        Ok((
            super::publication::approval::WebsiteDecisionProvenance {
                subject: credential.subject,
                operation: self.operation.into(),
                candidate_revision: build::REVISION.into(),
                challenge_digest: self.request.challenge_digest.clone(),
                binding_digest: self.request.binding_digest.clone(),
            },
            time,
        ))
    }
}

async fn publication_decision(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Json(request): Json<WebsiteDecisionRequest>,
) -> ApiResult<Json<Value>> {
    use super::publication::approval::{append_authenticated_website_decision, DecisionKind};
    if !matches!(
        request.decision,
        DecisionKind::Approved | DecisionKind::Withheld
    ) {
        return Err(ApiError(
            StatusCode::BAD_REQUEST,
            "invalid_website_decision",
        ));
    }
    let _guard = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    let credential = service.auth(&headers)?;
    let op = service.operation(&credential, &operation)?;
    if credential.mode != Mode::Hosted || op.status != Status::Complete || op.expires_at <= now() {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_operation_not_current",
        ));
    }
    let dir = service.dir(&credential.subject, &operation);
    let bundle = publication_directory(&service, &credential.subject, &operation, request.format);
    let review = internal(read_json(
        &dir.join("work/review/review-record.json"),
        MAX_RESULT,
    ))?;
    let publication = internal(read_json(&bundle.join("publication.json"), MAX_RESULT))?;
    let challenge = internal(read_json(&bundle.join("challenge.json"), MAX_RESULT))?;
    let authority = WebsiteDecisionAuthority {
        service: &service,
        headers: &headers,
        operation: &operation,
        request: &request,
        challenge: &challenge,
    };
    let record = append_authenticated_website_decision(
        &bundle.join("approval"),
        &review,
        &publication,
        request.decision.clone(),
        &authority,
    )
    .map_err(|_| ApiError(StatusCode::CONFLICT, "publication_decision_rejected"))?;
    Ok(Json(
        json!({"schema":"codefriend.publication_decision_response.v1",
        "candidate_revision":build::REVISION,"format":request.format, "decision":record}),
    ))
}

async fn publication_result(
    State(service): State<Service>,
    headers: HeaderMap,
    HttpPath(operation): HttpPath<String>,
    Query(selection): Query<PublicationSelection>,
) -> ApiResult<Json<Value>> {
    use super::evidence::contracts::{Publication, ReviewRecord};
    let _guard = service
        .0
        .gate
        .lock()
        .map_err(|_| ApiError(StatusCode::INTERNAL_SERVER_ERROR, "service_unavailable"))?;
    let credential = service.auth(&headers)?;
    let op = service.operation(&credential, &operation)?;
    if credential.mode != Mode::Hosted
        || op.status != Status::Complete
        || op.expires_at <= now()
        || op.candidate_revision != build::REVISION
    {
        return Err(ApiError(
            StatusCode::CONFLICT,
            "publication_operation_not_current",
        ));
    }
    let dir = service.dir(&credential.subject, &operation);
    let bundle = publication_directory(&service, &credential.subject, &operation, selection.format);
    if !bundle.join("publication.json").exists() {
        return Err(ApiError(StatusCode::NOT_FOUND, "publication_not_prepared"));
    }
    let review: ReviewRecord = internal(read_json(
        &dir.join("work/review/review-record.json"),
        MAX_RESULT,
    ))?;
    let publication: Publication =
        internal(read_json(&bundle.join("publication.json"), MAX_RESULT))?;
    internal(publication.validate(&review))?;
    internal(super::publication::verify_artifacts(
        &bundle.join("artifacts"),
        &publication.artifact_manifest,
    ))?;
    let decision = internal(super::publication::read_decision_head(
        &bundle.join("approval"),
        &review,
        &publication,
    ))?;
    let value = internal(publication_export::observe(
        &service,
        &credential.subject,
        &operation,
        selection.format,
        &review,
        &publication,
        decision.as_ref(),
    ))?;
    publication_export::checked_response(&service, &headers, &operation, &credential.subject, value)
}

// Unknown provider effect survives cancellation/expiry; neither proves non-effect.
fn stopped_provider_status(
    outcome: &Result<Value>,
    cancelled: bool,
    expired: bool,
) -> Option<Status> {
    if outcome
        .as_ref()
        .err()
        .is_some_and(|error| error.is::<crate::provider_adapter::CodeFriendProviderInterrupted>())
    {
        Some(Status::Interrupted)
    } else if cancelled || expired {
        Some(Status::Cancelled)
    } else {
        None
    }
}
#[cfg(test)]
mod provider_outcome_tests {
    use super::*;
    #[test]
    fn unknown_effect_survives_cancellation_and_expiry() {
        for (cancelled, expired) in [(false, false), (true, false), (false, true), (true, true)] {
            let result: Result<Value> =
                Err(crate::provider_adapter::CodeFriendProviderInterrupted.into());
            assert_eq!(
                stopped_provider_status(&result, cancelled, expired),
                Some(Status::Interrupted)
            );
        }
        assert_eq!(
            stopped_provider_status(&Ok(Value::Null), true, false),
            Some(Status::Cancelled)
        );
        assert_eq!(
            stopped_provider_status(&Ok(Value::Null), false, true),
            Some(Status::Cancelled)
        );
    }
}
