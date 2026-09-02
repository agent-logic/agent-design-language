use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    future::Future,
    io::Write,
    path::{Path, PathBuf},
    pin::Pin,
    process::Stdio,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sysinfo::{Pid, ProcessesToUpdate, System};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, Command},
    sync::Semaphore,
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use crate::operations::{ExecutorError, FailureClass, OperationExecutor, OperationRequest};

type TrackedProcesses = BTreeMap<Pid, u64>;

pub const SHEPHERD_REQUEST_SCHEMA: &str = "adl.runtime.shepherd_request.v1";
pub const SHEPHERD_RESPONSE_SCHEMA: &str = "adl.runtime.shepherd_response.v1";
pub const SHEPHERD_FAILURE_SCHEMA: &str = "adl.runtime.shepherd_failure.v1";
pub const SHEPHERD_RUNNER_REQUEST_SCHEMA: &str = "adl.runtime.shepherd_runner_request.v1";
pub const SHEPHERD_RUNNER_RESPONSE_SCHEMA: &str = "adl.runtime.shepherd_runner_response.v1";

const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_ARGUMENTS: usize = 32;
const MAX_ARGUMENT_BYTES: usize = 512;
const MAX_ENVIRONMENT_ENTRIES: usize = 32;
const MAX_ENVIRONMENT_VALUE_BYTES: usize = 4096;
const MAX_STDERR_BYTES: usize = 4096;
const MAX_CONFIGURED_PROMPT_BYTES: usize = 1024 * 1024;
const MAX_CONFIGURED_OUTPUT_BYTES: usize = 1024 * 1024;
const MAX_CONFIGURED_IN_FLIGHT: usize = 4;
const MAX_CONFIGURED_MEMORY_BYTES: u64 = 128 * 1024 * 1024 * 1024;
const MAX_CONFIGURED_CPU_SECONDS: u64 = 3600;
const MAX_CONFIGURED_OPEN_FILES: u64 = 1024;
const MAX_CONFIGURED_PROCESSES: u64 = 4096;
const MAX_REQUEST_ENVELOPE_OVERHEAD: usize = 1024;
const MAX_RUNNER_ENVELOPE_OVERHEAD: usize = 4096;
const PIPE_DRAIN_TIMEOUT: Duration = Duration::from_millis(500);
const CHILD_REAP_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShepherdExecutionClass {
    Unavailable,
    DeterministicTestDouble,
    RealLocalModel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShepherdProvenance {
    LiveExecution,
    IdempotencyReplay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShepherdModelIdentity {
    pub runner_program_sha256: String,
    pub backend_identity: String,
    pub model_identity: String,
    pub model_artifact_sha256: String,
}

impl ShepherdModelIdentity {
    pub fn new(
        runner_program_sha256: impl Into<String>,
        backend_identity: impl Into<String>,
        model_identity: impl Into<String>,
        model_artifact_sha256: impl Into<String>,
    ) -> Self {
        Self {
            runner_program_sha256: runner_program_sha256.into(),
            backend_identity: backend_identity.into(),
            model_identity: model_identity.into(),
            model_artifact_sha256: model_artifact_sha256.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalShepherdConfig {
    pub runtime_id: String,
    pub program: PathBuf,
    pub arguments: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub expected_runner_program_sha256: Option<String>,
    pub backend_identity: Option<String>,
    pub model_identity: String,
    pub model_artifact_sha256: Option<String>,
    pub timeout: Duration,
    pub max_prompt_bytes: usize,
    pub max_output_bytes: usize,
    pub max_in_flight: usize,
    pub max_memory_bytes: u64,
    pub max_cpu_seconds: u64,
    pub max_open_files: u64,
    pub max_processes: u64,
    execution_class: ShepherdExecutionClass,
}

impl LocalShepherdConfig {
    pub fn real_local_model(
        runtime_id: impl Into<String>,
        program: impl Into<PathBuf>,
        arguments: Vec<String>,
        environment: BTreeMap<String, String>,
        identity: ShepherdModelIdentity,
    ) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            program: program.into(),
            arguments,
            environment,
            expected_runner_program_sha256: Some(identity.runner_program_sha256),
            backend_identity: Some(identity.backend_identity),
            model_identity: identity.model_identity,
            model_artifact_sha256: Some(identity.model_artifact_sha256),
            timeout: Duration::from_secs(120),
            max_prompt_bytes: 16 * 1024,
            max_output_bytes: 64 * 1024,
            max_in_flight: 1,
            max_memory_bytes: 32 * 1024 * 1024 * 1024,
            max_cpu_seconds: 180,
            max_open_files: 128,
            max_processes: 1024,
            execution_class: ShepherdExecutionClass::RealLocalModel,
        }
    }

    /// Constructs an explicitly non-production executor used by deterministic proofs.
    /// Its output can never be classified as a real local model execution.
    pub fn deterministic_test_double(
        runtime_id: impl Into<String>,
        program: impl Into<PathBuf>,
        arguments: Vec<String>,
    ) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            program: program.into(),
            arguments,
            environment: BTreeMap::new(),
            expected_runner_program_sha256: None,
            backend_identity: None,
            model_identity: "deterministic-test-double".to_owned(),
            model_artifact_sha256: None,
            timeout: Duration::from_secs(120),
            max_prompt_bytes: 16 * 1024,
            max_output_bytes: 64 * 1024,
            max_in_flight: 1,
            max_memory_bytes: 512 * 1024 * 1024,
            max_cpu_seconds: 10,
            max_open_files: 64,
            max_processes: 1024,
            execution_class: ShepherdExecutionClass::DeterministicTestDouble,
        }
    }

    pub fn validate(&self) -> Result<(), ShepherdError> {
        validate_identifier(&self.runtime_id).map_err(|_| ShepherdError::InvalidConfiguration)?;
        validate_identifier(&self.model_identity)
            .map_err(|_| ShepherdError::InvalidConfiguration)?;
        match self.execution_class {
            ShepherdExecutionClass::RealLocalModel => {
                if !self
                    .expected_runner_program_sha256
                    .as_deref()
                    .is_some_and(valid_sha256)
                {
                    return Err(ShepherdError::InvalidConfiguration);
                }
                validate_identifier(
                    self.backend_identity
                        .as_deref()
                        .ok_or(ShepherdError::InvalidConfiguration)?,
                )
                .map_err(|_| ShepherdError::InvalidConfiguration)?;
                if !self
                    .model_artifact_sha256
                    .as_deref()
                    .is_some_and(valid_sha256)
                {
                    return Err(ShepherdError::InvalidConfiguration);
                }
            }
            ShepherdExecutionClass::DeterministicTestDouble => {
                if self.expected_runner_program_sha256.is_some()
                    || self.backend_identity.is_some()
                    || self.model_artifact_sha256.is_some()
                {
                    return Err(ShepherdError::InvalidConfiguration);
                }
            }
            ShepherdExecutionClass::Unavailable => return Err(ShepherdError::InvalidConfiguration),
        }
        if !self.program.is_absolute() || !is_executable_file(&self.program) {
            return Err(ShepherdError::Unavailable);
        }
        if self.arguments.len() > MAX_ARGUMENTS
            || self.arguments.iter().any(|value| {
                value.is_empty()
                    || value.len() > MAX_ARGUMENT_BYTES
                    || value
                        .bytes()
                        .any(|byte| byte == 0 || byte == b'\n' || byte == b'\r')
            })
        {
            return Err(ShepherdError::InvalidConfiguration);
        }
        if self.environment.len() > MAX_ENVIRONMENT_ENTRIES
            || self.environment.iter().any(|(key, value)| {
                !valid_environment_key(key)
                    || value.len() > MAX_ENVIRONMENT_VALUE_BYTES
                    || value
                        .bytes()
                        .any(|byte| byte == 0 || byte == b'\n' || byte == b'\r')
            })
        {
            return Err(ShepherdError::InvalidConfiguration);
        }
        if self.timeout.is_zero()
            || self.max_prompt_bytes == 0
            || self.max_prompt_bytes > MAX_CONFIGURED_PROMPT_BYTES
            || self.max_output_bytes == 0
            || self.max_output_bytes > MAX_CONFIGURED_OUTPUT_BYTES
            || self.max_in_flight == 0
            || self.max_in_flight > MAX_CONFIGURED_IN_FLIGHT
            || self.max_memory_bytes == 0
            || self.max_memory_bytes > MAX_CONFIGURED_MEMORY_BYTES
            || self.max_cpu_seconds == 0
            || self.max_cpu_seconds > MAX_CONFIGURED_CPU_SECONDS
            || self.max_open_files < 16
            || self.max_open_files > MAX_CONFIGURED_OPEN_FILES
            || self.max_processes == 0
            || self.max_processes > MAX_CONFIGURED_PROCESSES
        {
            return Err(ShepherdError::InvalidConfiguration);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdRequest {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shepherd_name: Option<String>,
    pub prompt: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdRunnerRequest {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    pub nonce: String,
    pub backend_identity: String,
    pub model_identity: String,
    pub model_artifact_sha256: String,
    pub prompt: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdRunnerResponse {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    pub nonce: String,
    pub backend_identity: String,
    pub model_identity: String,
    pub model_artifact_sha256: String,
    pub response: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdResponse {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    pub execution_class: ShepherdExecutionClass,
    pub provenance: ShepherdProvenance,
    pub retained: bool,
    pub backend_identity_sha256: Option<String>,
    pub model_identity_sha256: String,
    pub model_artifact_sha256: Option<String>,
    pub runner_program_sha256: String,
    pub runner_launch_sha256: String,
    pub runner_nonce_sha256: Option<String>,
    pub elapsed_millis: u64,
    pub response: String,
    pub response_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdFailureResponse {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    pub execution_class: ShepherdExecutionClass,
    pub provenance: ShepherdProvenance,
    pub retained: bool,
    pub reason_code: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShepherdError {
    InvalidConfiguration,
    InvalidRequest,
    WrongRuntime,
    Unavailable,
    Saturated,
    TimedOut,
    Cancelled,
    ProcessFailed,
    OutputTooLarge,
    MalformedOutput,
    AttestationFailed,
    ResourceLimitExceeded,
}

impl ShepherdError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidConfiguration => "shepherd_invalid_configuration",
            Self::InvalidRequest => "shepherd_invalid_request",
            Self::WrongRuntime => "shepherd_wrong_runtime",
            Self::Unavailable => "shepherd_unavailable",
            Self::Saturated => "shepherd_saturated",
            Self::TimedOut => "shepherd_timeout",
            Self::Cancelled => "shepherd_cancelled",
            Self::ProcessFailed => "shepherd_process_failed",
            Self::OutputTooLarge => "shepherd_output_too_large",
            Self::MalformedOutput => "shepherd_malformed_output",
            Self::AttestationFailed => "shepherd_runner_attestation_failed",
            Self::ResourceLimitExceeded => "shepherd_resource_limit_exceeded",
        }
    }

    fn failure_class(self) -> FailureClass {
        match self {
            Self::Unavailable
            | Self::Saturated
            | Self::TimedOut
            | Self::ProcessFailed
            | Self::ResourceLimitExceeded => FailureClass::Degraded,
            Self::InvalidConfiguration
            | Self::InvalidRequest
            | Self::WrongRuntime
            | Self::Cancelled
            | Self::OutputTooLarge
            | Self::MalformedOutput
            | Self::AttestationFailed => FailureClass::Fatal,
        }
    }
}

#[derive(Clone)]
pub struct LocalShepherdExecutor {
    config: Option<Arc<LocalShepherdConfig>>,
    runner_program: Option<Arc<ImmutableRunnerProgram>>,
    permits: Arc<Semaphore>,
    runner_program_sha256: Option<String>,
    runner_launch_sha256: Option<String>,
}

struct ImmutableRunnerProgram {
    _file: File,
    launch: ImmutableRunnerLaunch,
    retained_path: Option<PathBuf>,
}

enum ImmutableRunnerLaunch {
    #[cfg(target_os = "linux")]
    Descriptor,
    Path(PathBuf),
}

impl ImmutableRunnerProgram {
    fn capture(bytes: &[u8]) -> Result<Self, ShepherdError> {
        let path = std::env::temp_dir().join(format!(
            "adl-shepherd-runner-{}",
            uuid::Uuid::new_v4().simple()
        ));
        let mut options = OpenOptions::new();
        options.read(true).write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o700);
        }
        let mut file = options
            .open(&path)
            .map_err(|_| ShepherdError::Unavailable)?;
        if let Err(error) = file.write_all(bytes).and_then(|_| file.sync_all()) {
            let _ = std::fs::remove_file(&path);
            return Err(if error.kind() == std::io::ErrorKind::WriteZero {
                ShepherdError::InvalidConfiguration
            } else {
                ShepherdError::Unavailable
            });
        }
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;
        #[cfg(unix)]
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o500))
            .map_err(|_| ShepherdError::Unavailable)?;
        drop(file);
        let file = File::open(&path).map_err(|_| ShepherdError::Unavailable)?;

        let launch = immutable_runner_launch(bytes, &path)?;
        let retain_path = matches!(launch, ImmutableRunnerLaunch::Path(_));
        if !retain_path {
            std::fs::remove_file(&path).map_err(|_| ShepherdError::Unavailable)?;
        }
        Ok(Self {
            _file: file,
            launch,
            retained_path: retain_path.then_some(path),
        })
    }

    fn command(&self) -> Command {
        match &self.launch {
            #[cfg(target_os = "linux")]
            ImmutableRunnerLaunch::Descriptor => Command::new(self.descriptor_path()),
            ImmutableRunnerLaunch::Path(path) => Command::new(path),
        }
    }

    #[cfg(target_os = "linux")]
    fn descriptor_path(&self) -> PathBuf {
        use std::os::fd::AsRawFd;
        let fd = self._file.as_raw_fd();
        #[cfg(target_os = "linux")]
        return PathBuf::from(format!("/proc/self/fd/{fd}"));
        #[cfg(not(target_os = "linux"))]
        PathBuf::from(format!("/dev/fd/{fd}"))
    }
}

impl Drop for ImmutableRunnerProgram {
    fn drop(&mut self) {
        if let Some(path) = self.retained_path.as_ref() {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn immutable_runner_launch(
    _bytes: &[u8],
    _path: &Path,
) -> Result<ImmutableRunnerLaunch, ShepherdError> {
    #[cfg(target_os = "linux")]
    return Ok(ImmutableRunnerLaunch::Descriptor);

    #[allow(unreachable_code)]
    Ok(ImmutableRunnerLaunch::Path(_path.to_path_buf()))
}

impl LocalShepherdExecutor {
    pub fn unavailable() -> Self {
        Self {
            config: None,
            runner_program: None,
            permits: Arc::new(Semaphore::new(1)),
            runner_program_sha256: None,
            runner_launch_sha256: None,
        }
    }

    pub fn configured(config: LocalShepherdConfig) -> Result<Self, ShepherdError> {
        config.validate()?;
        let program = std::fs::read(&config.program).map_err(|_| ShepherdError::Unavailable)?;
        let runner_program_sha256 = sha256(&program);
        if config.execution_class == ShepherdExecutionClass::RealLocalModel
            && config.expected_runner_program_sha256.as_deref()
                != Some(runner_program_sha256.as_str())
        {
            return Err(ShepherdError::AttestationFailed);
        }
        let runner_program = Arc::new(ImmutableRunnerProgram::capture(&program)?);
        let runner_launch_sha256 = launch_digest(&config, &runner_program_sha256)?;
        let max_in_flight = config.max_in_flight;
        Ok(Self {
            config: Some(Arc::new(config)),
            runner_program: Some(runner_program),
            permits: Arc::new(Semaphore::new(max_in_flight)),
            runner_program_sha256: Some(runner_program_sha256),
            runner_launch_sha256: Some(runner_launch_sha256),
        })
    }

    async fn invoke(
        &self,
        operation: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ShepherdError> {
        let config = self.config.as_ref().ok_or(ShepherdError::Unavailable)?;
        let request = decode_request(operation, config.max_prompt_bytes)?;
        if request.runtime_id != config.runtime_id {
            return Err(ShepherdError::WrongRuntime);
        }
        if cancellation.is_cancelled() {
            return Err(ShepherdError::Cancelled);
        }
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| ShepherdError::Saturated)?;
        let started = Instant::now();
        let nonce = uuid::Uuid::new_v4().simple().to_string();
        let input = runner_input(config, &request, &nonce)?;
        let runner_program = self
            .runner_program
            .as_ref()
            .ok_or(ShepherdError::InvalidConfiguration)?;
        let (mut child, mut process_group) = spawn(config, runner_program)?;
        let stdin = child.stdin.take().ok_or(ShepherdError::ProcessFailed)?;
        let stdout = child.stdout.take().ok_or(ShepherdError::ProcessFailed)?;
        let stderr = child.stderr.take().ok_or(ShepherdError::ProcessFailed)?;
        let stdout_limit = config
            .max_output_bytes
            .saturating_add(MAX_RUNNER_ENVELOPE_OVERHEAD);
        let stdout = tokio::spawn(read_bounded(stdout, stdout_limit));
        let stderr = tokio::spawn(read_bounded(stderr, MAX_STDERR_BYTES));
        let mut memory_monitor = ProcessTreeMemoryMonitor::start(
            process_group.process_id,
            config.max_memory_bytes,
            process_group.tracked.clone(),
        );
        let timeout = tokio::time::sleep(config.timeout);
        tokio::pin!(timeout);
        let write_result = tokio::select! {
            _ = cancellation.cancelled() => Err(ShepherdError::Cancelled),
            _ = &mut timeout => Err(ShepherdError::TimedOut),
            _ = memory_monitor.exceeded() => Err(ShepherdError::ResourceLimitExceeded),
            result = write_input(stdin, &input) => result,
        };
        if let Err(error) = write_result {
            terminate_and_reap(&mut child, &mut process_group, stdout, stderr).await;
            memory_monitor.stop().await;
            return Err(error);
        }
        let status = tokio::select! {
            _ = cancellation.cancelled() => {
                terminate_and_reap(&mut child, &mut process_group, stdout, stderr).await;
                memory_monitor.stop().await;
                return Err(ShepherdError::Cancelled);
            }
            _ = &mut timeout => {
                terminate_and_reap(&mut child, &mut process_group, stdout, stderr).await;
                memory_monitor.stop().await;
                return Err(ShepherdError::TimedOut);
            }
            _ = memory_monitor.exceeded() => {
                terminate_and_reap(&mut child, &mut process_group, stdout, stderr).await;
                memory_monitor.stop().await;
                return Err(ShepherdError::ResourceLimitExceeded);
            }
            status = child.wait() => status.map_err(|_| ShepherdError::ProcessFailed)?,
        };
        // The direct child may exit while descendants retain its pipes. Terminate the
        // isolated process group and any separately observed descendants before drain.
        process_group.terminate();
        memory_monitor.stop().await;
        let stdout = join_reader_bounded(stdout).await?;
        let _stderr = join_reader_bounded(stderr).await?;
        drop(permit);

        if !status.success() {
            return Err(ShepherdError::ProcessFailed);
        }
        let (response, nonce_hash) = decode_runner_output(config, &request, &nonce, stdout)?;
        let response_sha256 = sha256(response.as_bytes());
        let response = ShepherdResponse {
            schema: SHEPHERD_RESPONSE_SCHEMA.to_owned(),
            correlation_id: request.correlation_id,
            runtime_id: request.runtime_id,
            execution_class: config.execution_class,
            provenance: ShepherdProvenance::LiveExecution,
            retained: false,
            backend_identity_sha256: config
                .backend_identity
                .as_ref()
                .map(|value| sha256(value.as_bytes())),
            model_identity_sha256: sha256(config.model_identity.as_bytes()),
            model_artifact_sha256: config.model_artifact_sha256.clone(),
            runner_program_sha256: self
                .runner_program_sha256
                .clone()
                .ok_or(ShepherdError::InvalidConfiguration)?,
            runner_launch_sha256: self
                .runner_launch_sha256
                .clone()
                .ok_or(ShepherdError::InvalidConfiguration)?,
            runner_nonce_sha256: nonce_hash,
            elapsed_millis: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
            response,
            response_sha256,
        };
        serde_json::to_vec(&response).map_err(|_| ShepherdError::MalformedOutput)
    }

    fn executor_error(&self, operation: &OperationRequest, error: ShepherdError) -> ExecutorError {
        let message = decode_request(operation, MAX_CONFIGURED_PROMPT_BYTES)
            .ok()
            .and_then(|request| {
                let runtime_id = self
                    .config
                    .as_ref()
                    .map(|config| config.runtime_id.clone())
                    .unwrap_or(request.runtime_id);
                serde_json::to_string(&ShepherdFailureResponse {
                    schema: SHEPHERD_FAILURE_SCHEMA.to_owned(),
                    correlation_id: request.correlation_id,
                    runtime_id,
                    execution_class: ShepherdExecutionClass::Unavailable,
                    provenance: ShepherdProvenance::LiveExecution,
                    retained: false,
                    reason_code: error.code().to_owned(),
                })
                .ok()
            })
            .unwrap_or_else(|| error.code().to_owned());
        ExecutorError {
            class: error.failure_class(),
            message,
        }
    }
}

impl OperationExecutor for LocalShepherdExecutor {
    fn execute<'life0, 'life1, 'async_trait>(
        &'life0 self,
        request: &'life1 OperationRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, ExecutorError>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.execute_with_cancellation(request, &CancellationToken::new())
                .await
        })
    }

    fn replay_payload(&self, payload: &[u8]) -> Result<Vec<u8>, ExecutorError> {
        let mut response: ShepherdResponse = serde_json::from_slice(payload)
            .map_err(|_| self.executor_error_from_code(ShepherdError::MalformedOutput))?;
        if response.schema != SHEPHERD_RESPONSE_SCHEMA {
            return Err(self.executor_error_from_code(ShepherdError::MalformedOutput));
        }
        response.provenance = ShepherdProvenance::IdempotencyReplay;
        response.retained = true;
        serde_json::to_vec(&response)
            .map_err(|_| self.executor_error_from_code(ShepherdError::MalformedOutput))
    }

    fn replay_error_message(&self, message: &str) -> Result<String, ExecutorError> {
        let Ok(mut response) = serde_json::from_str::<ShepherdFailureResponse>(message) else {
            return Ok(message.to_owned());
        };
        if response.schema != SHEPHERD_FAILURE_SCHEMA {
            return Ok(message.to_owned());
        }
        response.provenance = ShepherdProvenance::IdempotencyReplay;
        response.retained = true;
        serde_json::to_string(&response)
            .map_err(|_| self.executor_error_from_code(ShepherdError::MalformedOutput))
    }

    fn execute_with_cancellation<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        request: &'life1 OperationRequest,
        cancellation: &'life2 CancellationToken,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, ExecutorError>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.invoke(request, cancellation)
                .await
                .map_err(|error| self.executor_error(request, error))
        })
    }
}

impl LocalShepherdExecutor {
    fn executor_error_from_code(&self, error: ShepherdError) -> ExecutorError {
        ExecutorError {
            class: error.failure_class(),
            message: error.code().to_owned(),
        }
    }
}

pub(crate) fn decode_request(
    operation: &OperationRequest,
    max_prompt_bytes: usize,
) -> Result<ShepherdRequest, ShepherdError> {
    if operation.payload.is_empty()
        || operation.payload.len() > max_prompt_bytes.saturating_add(MAX_REQUEST_ENVELOPE_OVERHEAD)
    {
        return Err(ShepherdError::InvalidRequest);
    }
    let request: ShepherdRequest =
        serde_json::from_slice(&operation.payload).map_err(|_| ShepherdError::InvalidRequest)?;
    if request.schema != SHEPHERD_REQUEST_SCHEMA
        || validate_identifier(&request.correlation_id).is_err()
        || validate_identifier(&request.runtime_id).is_err()
        || request
            .shepherd_name
            .as_deref()
            .is_some_and(|name| !crate::is_canonical_agent_name(name))
        || request.prompt.trim().is_empty()
        || request.prompt.len() > max_prompt_bytes
        || request.prompt.bytes().any(|byte| byte == 0)
    {
        return Err(ShepherdError::InvalidRequest);
    }
    Ok(request)
}

fn runner_input(
    config: &LocalShepherdConfig,
    request: &ShepherdRequest,
    nonce: &str,
) -> Result<Vec<u8>, ShepherdError> {
    if config.execution_class == ShepherdExecutionClass::DeterministicTestDouble {
        return Ok(request.prompt.as_bytes().to_vec());
    }
    serde_json::to_vec(&ShepherdRunnerRequest {
        schema: SHEPHERD_RUNNER_REQUEST_SCHEMA.to_owned(),
        correlation_id: request.correlation_id.clone(),
        runtime_id: request.runtime_id.clone(),
        nonce: nonce.to_owned(),
        backend_identity: config
            .backend_identity
            .clone()
            .ok_or(ShepherdError::InvalidConfiguration)?,
        model_identity: config.model_identity.clone(),
        model_artifact_sha256: config
            .model_artifact_sha256
            .clone()
            .ok_or(ShepherdError::InvalidConfiguration)?,
        prompt: request.prompt.clone(),
    })
    .map_err(|_| ShepherdError::InvalidRequest)
}

fn decode_runner_output(
    config: &LocalShepherdConfig,
    request: &ShepherdRequest,
    nonce: &str,
    stdout: Vec<u8>,
) -> Result<(String, Option<String>), ShepherdError> {
    if config.execution_class == ShepherdExecutionClass::DeterministicTestDouble {
        let response = String::from_utf8(stdout)
            .map_err(|_| ShepherdError::MalformedOutput)?
            .trim()
            .to_owned();
        if response.is_empty() {
            return Err(ShepherdError::MalformedOutput);
        }
        if response.len() > config.max_output_bytes {
            return Err(ShepherdError::OutputTooLarge);
        }
        return Ok((response, None));
    }
    let response: ShepherdRunnerResponse =
        serde_json::from_slice(&stdout).map_err(|_| ShepherdError::AttestationFailed)?;
    let expected_backend = config
        .backend_identity
        .as_deref()
        .ok_or(ShepherdError::InvalidConfiguration)?;
    let expected_digest = config
        .model_artifact_sha256
        .as_deref()
        .ok_or(ShepherdError::InvalidConfiguration)?;
    if response.schema != SHEPHERD_RUNNER_RESPONSE_SCHEMA
        || response.correlation_id != request.correlation_id
        || response.runtime_id != request.runtime_id
        || response.nonce != nonce
        || response.backend_identity != expected_backend
        || response.model_identity != config.model_identity
        || response.model_artifact_sha256 != expected_digest
        || response.response.trim().is_empty()
        || response.response.len() > config.max_output_bytes
    {
        return Err(ShepherdError::AttestationFailed);
    }
    Ok((response.response, Some(sha256(nonce.as_bytes()))))
}

fn spawn(
    config: &LocalShepherdConfig,
    runner_program: &ImmutableRunnerProgram,
) -> Result<(Child, ProcessGroupGuard), ShepherdError> {
    let mut command = runner_program.command();
    command
        .args(&config.arguments)
        .env_clear()
        .envs(&config.environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(unix)]
    {
        #[cfg(target_os = "linux")]
        use std::os::fd::AsRawFd;
        use std::os::unix::process::CommandExt;
        #[cfg(target_os = "linux")]
        let runner_fd = runner_program._file.as_raw_fd();
        command.as_std_mut().process_group(0);
        #[cfg(not(target_os = "macos"))]
        let memory = config.max_memory_bytes as libc::rlim_t;
        let cpu = config.max_cpu_seconds as libc::rlim_t;
        let open_files = config.max_open_files as libc::rlim_t;
        let processes = config.max_processes as libc::rlim_t;
        unsafe {
            command.as_std_mut().pre_exec(move || {
                #[cfg(target_os = "linux")]
                {
                    if libc::fcntl(runner_fd, libc::F_SETFD, 0) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let mut inherited = libc::rlimit {
                        rlim_cur: 0,
                        rlim_max: 0,
                    };
                    if libc::getrlimit(libc::RLIMIT_AS, &mut inherited) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    inherited.rlim_cur = inherited.rlim_max.min(memory);
                    if libc::setrlimit(libc::RLIMIT_AS, &inherited) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                for (resource, value) in [
                    (libc::RLIMIT_CPU, cpu),
                    (libc::RLIMIT_NOFILE, open_files),
                    (libc::RLIMIT_NPROC, processes),
                ] {
                    let mut inherited = libc::rlimit {
                        rlim_cur: 0,
                        rlim_max: 0,
                    };
                    if libc::getrlimit(resource, &mut inherited) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    inherited.rlim_cur = inherited.rlim_max.min(value);
                    if libc::setrlimit(resource, &inherited) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                Ok(())
            });
        }
    }
    let child = command.spawn().map_err(|_| ShepherdError::Unavailable)?;
    let process_id = child.id().ok_or(ShepherdError::Unavailable)?;
    Ok((child, ProcessGroupGuard::new(process_id)))
}

struct ProcessTreeMemoryMonitor {
    cancellation: CancellationToken,
    task: Option<JoinHandle<()>>,
}

impl ProcessTreeMemoryMonitor {
    fn start(
        process_id: u32,
        max_memory_bytes: u64,
        tracked: Arc<Mutex<TrackedProcesses>>,
    ) -> Self {
        let cancellation = CancellationToken::new();
        let monitor_cancellation = cancellation.clone();
        let task = tokio::task::spawn_blocking(move || {
            monitor_process_tree_memory_blocking(
                process_id,
                max_memory_bytes,
                &tracked,
                &monitor_cancellation,
            );
        });
        Self {
            cancellation,
            task: Some(task),
        }
    }

    async fn exceeded(&mut self) {
        if let Some(task) = self.task.as_mut() {
            let _ = task.await;
            self.task = None;
        }
    }

    async fn stop(&mut self) {
        self.cancellation.cancel();
        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }
}

impl Drop for ProcessTreeMemoryMonitor {
    fn drop(&mut self) {
        self.cancellation.cancel();
    }
}

fn monitor_process_tree_memory_blocking(
    process_id: u32,
    max_memory_bytes: u64,
    tracked: &Mutex<TrackedProcesses>,
    cancellation: &CancellationToken,
) {
    let root = Pid::from(process_id as usize);
    let mut system = System::new_all();
    while !cancellation.is_cancelled() {
        system.refresh_processes(ProcessesToUpdate::All, true);
        let mut members = tracked
            .lock()
            .map(|members| members.clone())
            .unwrap_or_else(|_| BTreeMap::new());
        if let Some(process) = system.process(root) {
            members.insert(root, process.start_time());
        }
        extend_process_tree(&system, &mut members);
        if let Ok(mut retained) = tracked.lock() {
            retained.extend(members.iter().map(|(pid, started)| (*pid, *started)));
        }
        let used = members.iter().fold(0_u64, |total, (pid, started)| {
            total.saturating_add(system.process(*pid).map_or(0, |process| {
                if process.start_time() == *started {
                    process.memory()
                } else {
                    0
                }
            }))
        });
        if used > max_memory_bytes {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn extend_process_tree(system: &System, members: &mut TrackedProcesses) {
    loop {
        let previous = members.len();
        for (pid, process) in system.processes() {
            if process.parent().is_some_and(|parent| {
                members.get(&parent).is_some_and(|started| {
                    system
                        .process(parent)
                        .is_some_and(|parent_process| parent_process.start_time() == *started)
                })
            }) {
                members.entry(*pid).or_insert_with(|| process.start_time());
            }
        }
        if members.len() == previous {
            break;
        }
    }
}

async fn write_input(mut stdin: ChildStdin, input: &[u8]) -> Result<(), ShepherdError> {
    stdin
        .write_all(input)
        .await
        .map_err(|_| ShepherdError::ProcessFailed)?;
    stdin
        .write_all(b"\n")
        .await
        .map_err(|_| ShepherdError::ProcessFailed)?;
    stdin
        .shutdown()
        .await
        .map_err(|_| ShepherdError::ProcessFailed)?;
    Ok(())
}

async fn terminate_and_reap(
    child: &mut Child,
    process_group: &mut ProcessGroupGuard,
    stdout: JoinHandle<Result<Vec<u8>, ShepherdError>>,
    stderr: JoinHandle<Result<Vec<u8>, ShepherdError>>,
) {
    process_group.terminate();
    let _ = child.kill().await;
    let _ = tokio::time::timeout(CHILD_REAP_TIMEOUT, child.wait()).await;
    abort_reader(stdout).await;
    abort_reader(stderr).await;
}

async fn abort_reader(reader: JoinHandle<Result<Vec<u8>, ShepherdError>>) {
    reader.abort();
    let _ = reader.await;
}

async fn join_reader_bounded(
    mut reader: JoinHandle<Result<Vec<u8>, ShepherdError>>,
) -> Result<Vec<u8>, ShepherdError> {
    match tokio::time::timeout(PIPE_DRAIN_TIMEOUT, &mut reader).await {
        Ok(result) => result.map_err(|_| ShepherdError::ProcessFailed)?,
        Err(_) => {
            reader.abort();
            let _ = reader.await;
            Err(ShepherdError::ProcessFailed)
        }
    }
}

async fn read_bounded(
    reader: impl AsyncRead + Unpin + Send + 'static,
    max_bytes: usize,
) -> Result<Vec<u8>, ShepherdError> {
    let limit = u64::try_from(max_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::with_capacity(max_bytes.min(8192));
    reader
        .take(limit)
        .read_to_end(&mut bytes)
        .await
        .map_err(|_| ShepherdError::ProcessFailed)?;
    if bytes.len() > max_bytes {
        return Err(ShepherdError::OutputTooLarge);
    }
    Ok(bytes)
}

#[cfg(unix)]
fn terminate_observed_process_tree(process_id: u32, tracked: &Mutex<TrackedProcesses>) {
    if let Ok(group) = i32::try_from(process_id) {
        unsafe {
            libc::kill(-group, libc::SIGSTOP);
        }
        let root = Pid::from(process_id as usize);
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::All, true);
        let mut members = tracked
            .lock()
            .map(|members| members.clone())
            .unwrap_or_else(|_| BTreeMap::new());
        if let Some(process) = system.process(root) {
            members.insert(root, process.start_time());
        }
        extend_process_tree(&system, &mut members);
        for (pid, started) in members.iter().filter(|(pid, _)| **pid != root) {
            let identity_matches = system
                .process(*pid)
                .is_some_and(|process| process.start_time() == *started);
            if identity_matches {
                if let Ok(pid) = i32::try_from(pid.as_u32()) {
                    unsafe {
                        libc::kill(pid, libc::SIGSTOP);
                    }
                }
            }
        }
        system.refresh_processes(ProcessesToUpdate::All, true);
        extend_process_tree(&system, &mut members);
        for (pid, started) in members.into_iter().rev().filter(|(pid, _)| *pid != root) {
            if system
                .process(pid)
                .is_some_and(|process| process.start_time() == started)
            {
                if let Ok(pid) = i32::try_from(pid.as_u32()) {
                    unsafe {
                        libc::kill(pid, libc::SIGKILL);
                    }
                }
            }
        }
        unsafe {
            libc::kill(-group, libc::SIGKILL);
        }
    }
}

#[cfg(not(unix))]
fn terminate_observed_process_tree(_process_id: u32, _tracked: &Mutex<TrackedProcesses>) {}

struct ProcessGroupGuard {
    process_id: u32,
    tracked: Arc<Mutex<TrackedProcesses>>,
    armed: bool,
}

impl ProcessGroupGuard {
    fn new(process_id: u32) -> Self {
        Self {
            process_id,
            tracked: Arc::new(Mutex::new(BTreeMap::new())),
            armed: true,
        }
    }

    fn terminate(&mut self) {
        if self.armed {
            terminate_observed_process_tree(self.process_id, &self.tracked);
            self.armed = false;
        }
    }
}

impl Drop for ProcessGroupGuard {
    fn drop(&mut self) {
        self.terminate();
    }
}

fn validate_identifier(value: &str) -> Result<(), ShepherdError> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(ShepherdError::InvalidRequest);
    }
    Ok(())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn valid_environment_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_IDENTIFIER_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_executable_file(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[derive(Serialize)]
struct LaunchIdentity<'a> {
    schema: &'static str,
    runner_program_sha256: &'a str,
    arguments: &'a [String],
    environment: &'a BTreeMap<String, String>,
}

fn launch_digest(
    config: &LocalShepherdConfig,
    runner_program_sha256: &str,
) -> Result<String, ShepherdError> {
    let bytes = serde_json::to_vec(&LaunchIdentity {
        schema: "adl.runtime.shepherd_launch_identity.v1",
        runner_program_sha256,
        arguments: &config.arguments,
        environment: &config.environment,
    })
    .map_err(|_| ShepherdError::InvalidConfiguration)?;
    Ok(sha256(&bytes))
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
