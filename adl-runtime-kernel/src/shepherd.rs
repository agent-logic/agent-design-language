use std::{
    collections::BTreeMap,
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, Command},
    sync::Semaphore,
};
use tokio_util::sync::CancellationToken;

use crate::operations::{ExecutorError, FailureClass, OperationExecutor, OperationRequest};

pub const SHEPHERD_REQUEST_SCHEMA: &str = "adl.runtime.shepherd_request.v1";
pub const SHEPHERD_RESPONSE_SCHEMA: &str = "adl.runtime.shepherd_response.v1";

const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_ARGUMENTS: usize = 32;
const MAX_ARGUMENT_BYTES: usize = 512;
const MAX_ENVIRONMENT_ENTRIES: usize = 32;
const MAX_ENVIRONMENT_VALUE_BYTES: usize = 4096;
const MAX_STDERR_BYTES: usize = 4096;
const MAX_CONFIGURED_PROMPT_BYTES: usize = 1024 * 1024;
const MAX_CONFIGURED_OUTPUT_BYTES: usize = 1024 * 1024;
const MAX_CONFIGURED_IN_FLIGHT: usize = 64;
const MAX_REQUEST_ENVELOPE_OVERHEAD: usize = 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShepherdExecutionClass {
    Unavailable,
    DeterministicTestDouble,
    RealLocalModel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalShepherdConfig {
    pub runtime_id: String,
    pub program: PathBuf,
    pub arguments: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub model_identity: String,
    pub timeout: Duration,
    pub max_prompt_bytes: usize,
    pub max_output_bytes: usize,
    pub max_in_flight: usize,
    execution_class: ShepherdExecutionClass,
}

impl LocalShepherdConfig {
    pub fn real_local_model(
        runtime_id: impl Into<String>,
        program: impl Into<PathBuf>,
        arguments: Vec<String>,
        environment: BTreeMap<String, String>,
        model_identity: impl Into<String>,
    ) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            program: program.into(),
            arguments,
            environment,
            model_identity: model_identity.into(),
            timeout: Duration::from_secs(120),
            max_prompt_bytes: 16 * 1024,
            max_output_bytes: 64 * 1024,
            max_in_flight: 1,
            execution_class: ShepherdExecutionClass::RealLocalModel,
        }
    }

    #[cfg(test)]
    pub fn deterministic_test_double(
        runtime_id: impl Into<String>,
        program: impl Into<PathBuf>,
        arguments: Vec<String>,
    ) -> Self {
        let mut config = Self::real_local_model(
            runtime_id,
            program,
            arguments,
            BTreeMap::new(),
            "deterministic-test-double",
        );
        config.execution_class = ShepherdExecutionClass::DeterministicTestDouble;
        config
    }

    pub fn validate(&self) -> Result<(), ShepherdError> {
        validate_identifier(&self.runtime_id).map_err(|_| ShepherdError::InvalidConfiguration)?;
        validate_identifier(&self.model_identity)
            .map_err(|_| ShepherdError::InvalidConfiguration)?;
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
    pub prompt: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShepherdResponse {
    pub schema: String,
    pub correlation_id: String,
    pub runtime_id: String,
    pub execution_class: ShepherdExecutionClass,
    pub retained: bool,
    pub model_identity_sha256: String,
    pub elapsed_millis: u64,
    pub response: String,
    pub response_sha256: String,
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
        }
    }

    fn failure_class(self) -> FailureClass {
        match self {
            Self::Unavailable | Self::Saturated | Self::TimedOut | Self::ProcessFailed => {
                FailureClass::Degraded
            }
            Self::InvalidConfiguration
            | Self::InvalidRequest
            | Self::WrongRuntime
            | Self::Cancelled
            | Self::OutputTooLarge
            | Self::MalformedOutput => FailureClass::Fatal,
        }
    }
}

#[derive(Clone)]
pub struct LocalShepherdExecutor {
    config: Option<Arc<LocalShepherdConfig>>,
    permits: Arc<Semaphore>,
}

impl LocalShepherdExecutor {
    pub fn unavailable() -> Self {
        Self {
            config: None,
            permits: Arc::new(Semaphore::new(1)),
        }
    }

    pub fn configured(config: LocalShepherdConfig) -> Result<Self, ShepherdError> {
        config.validate()?;
        let max_in_flight = config.max_in_flight;
        Ok(Self {
            config: Some(Arc::new(config)),
            permits: Arc::new(Semaphore::new(max_in_flight)),
        })
    }

    async fn invoke(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
    ) -> Result<Vec<u8>, ShepherdError> {
        let config = self.config.as_ref().ok_or(ShepherdError::Unavailable)?;
        let request = decode_request(request, config)?;
        if cancellation.is_cancelled() {
            return Err(ShepherdError::Cancelled);
        }
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| ShepherdError::Saturated)?;
        let started = Instant::now();
        let mut child = spawn(config)?;
        let stdin = child.stdin.take().ok_or(ShepherdError::ProcessFailed)?;
        let stdout = child.stdout.take().ok_or(ShepherdError::ProcessFailed)?;
        let stderr = child.stderr.take().ok_or(ShepherdError::ProcessFailed)?;
        let stdout = tokio::spawn(read_bounded(stdout, config.max_output_bytes));
        let stderr = tokio::spawn(read_bounded(stderr, MAX_STDERR_BYTES));
        let timeout = tokio::time::sleep(config.timeout);
        tokio::pin!(timeout);
        let write_result = tokio::select! {
            _ = cancellation.cancelled() => Err(ShepherdError::Cancelled),
            _ = &mut timeout => Err(ShepherdError::TimedOut),
            result = write_prompt(stdin, request.prompt.as_bytes()) => result,
        };
        if let Err(error) = write_result {
            terminate_and_reap(&mut child, stdout, stderr).await;
            return Err(error);
        }
        let status = tokio::select! {
            _ = cancellation.cancelled() => {
                terminate_and_reap(&mut child, stdout, stderr).await;
                return Err(ShepherdError::Cancelled);
            }
            _ = &mut timeout => {
                terminate_and_reap(&mut child, stdout, stderr).await;
                return Err(ShepherdError::TimedOut);
            }
            status = child.wait() => {
                status.map_err(|_| ShepherdError::ProcessFailed)?
            }
        };
        let stdout = join_reader(stdout).await?;
        let _stderr = join_reader(stderr).await?;
        drop(permit);

        if !status.success() {
            return Err(ShepherdError::ProcessFailed);
        }
        let response = String::from_utf8(stdout)
            .map_err(|_| ShepherdError::MalformedOutput)?
            .trim()
            .to_owned();
        if response.is_empty() {
            return Err(ShepherdError::MalformedOutput);
        }
        let response_sha256 = sha256(response.as_bytes());
        let response = ShepherdResponse {
            schema: SHEPHERD_RESPONSE_SCHEMA.to_owned(),
            correlation_id: request.correlation_id,
            runtime_id: request.runtime_id,
            execution_class: config.execution_class,
            retained: false,
            model_identity_sha256: sha256(config.model_identity.as_bytes()),
            elapsed_millis: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
            response,
            response_sha256,
        };
        serde_json::to_vec(&response).map_err(|_| ShepherdError::MalformedOutput)
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
                .map_err(|error| ExecutorError {
                    class: error.failure_class(),
                    message: error.code().to_owned(),
                })
        })
    }
}

fn decode_request(
    operation: &OperationRequest,
    config: &LocalShepherdConfig,
) -> Result<ShepherdRequest, ShepherdError> {
    if operation.payload.is_empty()
        || operation.payload.len()
            > config
                .max_prompt_bytes
                .saturating_add(MAX_REQUEST_ENVELOPE_OVERHEAD)
    {
        return Err(ShepherdError::InvalidRequest);
    }
    let request: ShepherdRequest =
        serde_json::from_slice(&operation.payload).map_err(|_| ShepherdError::InvalidRequest)?;
    if request.schema != SHEPHERD_REQUEST_SCHEMA
        || validate_identifier(&request.correlation_id).is_err()
        || validate_identifier(&request.runtime_id).is_err()
        || request.prompt.trim().is_empty()
        || request.prompt.len() > config.max_prompt_bytes
        || request.prompt.bytes().any(|byte| byte == 0)
    {
        return Err(ShepherdError::InvalidRequest);
    }
    if request.runtime_id != config.runtime_id {
        return Err(ShepherdError::WrongRuntime);
    }
    Ok(request)
}

fn spawn(config: &LocalShepherdConfig) -> Result<Child, ShepherdError> {
    let mut command = Command::new(&config.program);
    command
        .args(&config.arguments)
        .env_clear()
        .envs(&config.environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    command.spawn().map_err(|_| ShepherdError::Unavailable)
}

async fn write_prompt(mut stdin: ChildStdin, prompt: &[u8]) -> Result<(), ShepherdError> {
    stdin
        .write_all(prompt)
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
    stdout: tokio::task::JoinHandle<Result<Vec<u8>, ShepherdError>>,
    stderr: tokio::task::JoinHandle<Result<Vec<u8>, ShepherdError>>,
) {
    let _ = child.kill().await;
    stdout.abort();
    stderr.abort();
    let _ = stdout.await;
    let _ = stderr.await;
}

async fn join_reader(
    reader: tokio::task::JoinHandle<Result<Vec<u8>, ShepherdError>>,
) -> Result<Vec<u8>, ShepherdError> {
    reader.await.map_err(|_| ShepherdError::ProcessFailed)?
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

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
