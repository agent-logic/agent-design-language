use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use tokio_util::sync::CancellationToken;

use crate::{
    invoke_resident_shepherd_provider, shepherd::decode_request, ExecutorError, FailureClass,
    InferenceReadinessState, OperationExecutor, OperationRequest, ResidentShepherdInitConfig,
    ShepherdExecutionClass, ShepherdProvenance, ShepherdRequest, ShepherdResponse,
    SHEPHERD_RESPONSE_SCHEMA,
};

/// Provider adapters that are compiled into this Runtime build. Configuration
/// remains provider-shaped, but startup must reject profiles that have no
/// executable adapter instead of admitting a permanently degraded resident.
pub fn resident_shepherd_provider_is_available(provider: &str) -> bool {
    matches!(provider, "ollama" | "openai-compatible")
}

/// Provider-backed production Shepherd executor. The native executor remains
/// responsible for Runtime admission records; reasoning requests are routed to
/// the configured provider without granting it lifecycle authority.
pub struct ResidentShepherdExecutor {
    runtime_id: String,
    primary_name: String,
    configs: BTreeMap<String, ResidentShepherdInitConfig>,
    ready: ResidentShepherdReadiness,
    admission: Arc<dyn OperationExecutor>,
}

/// Internal-only executor used by the Runtime's governed bootstrap probe. It
/// shares the production provider path but deliberately bypasses only the
/// public readiness gate; it never mutates that gate itself.
pub struct ResidentShepherdProbeExecutor {
    inner: Arc<ResidentShepherdExecutor>,
}

#[derive(Clone, Default)]
pub struct ResidentShepherdReadiness(Arc<RwLock<BTreeSet<String>>>);

impl ResidentShepherdReadiness {
    pub fn mark_ready(&self, name: &str) {
        self.0
            .write()
            .expect("resident Shepherd readiness lock poisoned")
            .insert(name.to_owned());
    }

    pub fn mark_unready(&self, name: &str) {
        self.0
            .write()
            .expect("resident Shepherd readiness lock poisoned")
            .remove(name);
    }

    pub fn is_ready(&self, name: &str) -> bool {
        self.0
            .read()
            .expect("resident Shepherd readiness lock poisoned")
            .contains(name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidentShepherdRecoveryState {
    Unimplemented,
    Unavailable,
    ModelLoading,
    Failed,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResidentShepherdRecoveryPolicy {
    pub timeout: Duration,
    pub retry_initial: Duration,
    pub retry_max: Duration,
}

impl ResidentShepherdRecoveryState {
    pub fn inference_readiness(self) -> InferenceReadinessState {
        match self {
            Self::Unimplemented => InferenceReadinessState::Unimplemented,
            Self::Unavailable => InferenceReadinessState::Unavailable,
            Self::ModelLoading => InferenceReadinessState::ModelLoading,
            Self::Failed => InferenceReadinessState::Failed,
            Self::Ready => InferenceReadinessState::Ready,
        }
    }

    pub fn health(self) -> (&'static str, &'static str) {
        match self {
            Self::Unimplemented => (
                "unimplemented",
                "Configured provider has no production resident Shepherd adapter",
            ),
            Self::Unavailable => (
                "unavailable",
                "Provider model unavailable; recovery retry scheduled",
            ),
            Self::ModelLoading => ("model_loading", "Provider model preload in progress"),
            Self::Failed => (
                "failed",
                "Governed inference probe failed; recovery retry scheduled",
            ),
            Self::Ready => (
                "ready",
                "Configured provider model loaded; governed inference probe passed",
            ),
        }
    }

    fn from_attempt_error(error: &'static str) -> Self {
        match error {
            "resident_shepherd_provider_unsupported" => Self::Unimplemented,
            "provider_unreachable"
            | "provider_temporarily_unavailable"
            | "model_not_installed"
            | "operation cancelled" => Self::Unavailable,
            "provider_response_invalid"
            | "agent_provider_failed"
            | "resident_shepherd_governed_probe_failed" => Self::Failed,
            _ => Self::Failed,
        }
    }
}

/// Runs the resident Shepherd's complete lifetime health cycle. An attempt is
/// successful only after both provider preload and governed inference succeed.
/// The same controller is used by production and focused recovery tests.
pub async fn run_resident_shepherd_recovery<Attempt, AttemptFuture, Observe>(
    name: &str,
    policy: ResidentShepherdRecoveryPolicy,
    readiness: ResidentShepherdReadiness,
    shutdown: CancellationToken,
    mut attempt: Attempt,
    mut observe: Observe,
) where
    Attempt: FnMut() -> AttemptFuture + Send,
    AttemptFuture: Future<Output = Result<(), &'static str>> + Send,
    Observe: FnMut(ResidentShepherdRecoveryState) + Send,
{
    let mut retry = policy.retry_initial;
    let mut was_ready = false;
    loop {
        if !was_ready {
            readiness.mark_unready(name);
            observe(ResidentShepherdRecoveryState::ModelLoading);
        }
        let result = tokio::time::timeout(policy.timeout, attempt()).await;
        if matches!(result, Ok(Ok(()))) {
            readiness.mark_ready(name);
            observe(ResidentShepherdRecoveryState::Ready);
            was_ready = true;
            retry = policy.retry_initial;
            tokio::select! {
                _ = shutdown.cancelled() => break,
                _ = tokio::time::sleep(policy.retry_max) => continue,
            }
        }

        readiness.mark_unready(name);
        let state = match result {
            Ok(Err(error)) => ResidentShepherdRecoveryState::from_attempt_error(error),
            Err(_) => ResidentShepherdRecoveryState::Unavailable,
            Ok(Ok(())) => unreachable!("successful attempt handled above"),
        };
        observe(state);
        was_ready = false;
        tokio::select! {
            _ = shutdown.cancelled() => break,
            _ = tokio::time::sleep(retry) => {}
        }
        retry = retry.saturating_mul(2).min(policy.retry_max);
    }
}

impl ResidentShepherdExecutor {
    pub fn new(
        runtime_id: impl Into<String>,
        configs: impl IntoIterator<Item = ResidentShepherdInitConfig>,
        admission: Arc<dyn OperationExecutor>,
    ) -> Self {
        let configs = configs.into_iter().collect::<Vec<_>>();
        let primary_name = configs
            .first()
            .expect("validated Shepherd set is non-empty")
            .name
            .clone();
        let configs = configs
            .into_iter()
            .map(|config| (config.name.clone(), config))
            .collect::<BTreeMap<_, _>>();
        Self {
            runtime_id: runtime_id.into(),
            primary_name,
            configs,
            ready: ResidentShepherdReadiness::default(),
            admission,
        }
    }

    pub fn readiness(&self) -> ResidentShepherdReadiness {
        self.ready.clone()
    }

    fn invalid(message: &'static str) -> ExecutorError {
        ExecutorError {
            class: FailureClass::Fatal,
            message: message.to_owned(),
        }
    }

    async fn execute_provider_request(
        &self,
        request: &OperationRequest,
        cancellation: &CancellationToken,
        require_ready: bool,
    ) -> Result<Vec<u8>, ExecutorError> {
        let parsed = serde_json::from_slice::<ShepherdRequest>(&request.payload);
        if parsed.is_err() {
            return self
                .admission
                .execute_with_cancellation(request, cancellation)
                .await;
        }
        let shepherd_request = decode_request(request, 16 * 1024)
            .map_err(|_| Self::invalid("shepherd_invalid_request"))?;
        if shepherd_request.runtime_id != self.runtime_id {
            return Err(Self::invalid("shepherd_invalid_request"));
        }
        let shepherd_name = shepherd_request
            .shepherd_name
            .as_deref()
            .unwrap_or(&self.primary_name);
        let config = self
            .configs
            .get(shepherd_name)
            .ok_or_else(|| Self::invalid("shepherd_unknown_resident"))?;
        if require_ready && !self.ready.is_ready(shepherd_name) {
            return Err(ExecutorError {
                class: FailureClass::Retryable,
                message: "shepherd_model_not_ready".to_owned(),
            });
        }
        let started = Instant::now();
        let response = invoke_resident_shepherd_provider(
            &config.provider,
            &config.endpoint,
            &config.model,
            &shepherd_request.prompt,
            cancellation,
        )
        .await
        .map_err(|message| ExecutorError {
            class: if message == "operation cancelled" {
                FailureClass::Degraded
            } else {
                FailureClass::Retryable
            },
            message: message.to_owned(),
        })?;
        let response_sha256 = sha256(response.as_bytes());
        serde_json::to_vec(&ShepherdResponse {
            schema: SHEPHERD_RESPONSE_SCHEMA.to_owned(),
            correlation_id: shepherd_request.correlation_id,
            runtime_id: self.runtime_id.clone(),
            execution_class: ShepherdExecutionClass::RealLocalModel,
            provenance: ShepherdProvenance::LiveExecution,
            retained: false,
            backend_identity_sha256: Some(sha256(config.provider.as_bytes())),
            model_identity_sha256: sha256(config.model.as_bytes()),
            model_artifact_sha256: None,
            runner_program_sha256: sha256(config.endpoint.as_bytes()),
            runner_launch_sha256: sha256(
                format!("{}:{}", config.provider, config.model).as_bytes(),
            ),
            runner_nonce_sha256: None,
            elapsed_millis: started.elapsed().as_millis().try_into().unwrap_or(u64::MAX),
            response: response.clone(),
            response_sha256,
        })
        .map_err(|_| Self::invalid("shepherd_response_encoding_failed"))
    }
}

impl ResidentShepherdProbeExecutor {
    pub fn new(inner: Arc<ResidentShepherdExecutor>) -> Self {
        Self { inner }
    }
}

impl OperationExecutor for ResidentShepherdExecutor {
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
            let cancellation = CancellationToken::new();
            self.execute_with_cancellation(request, &cancellation).await
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
        Box::pin(self.execute_provider_request(request, cancellation, true))
    }

    fn replay_payload(&self, payload: &[u8]) -> Result<Vec<u8>, ExecutorError> {
        let mut response: ShepherdResponse = serde_json::from_slice(payload)
            .map_err(|_| Self::invalid("shepherd_response_invalid"))?;
        response.provenance = ShepherdProvenance::IdempotencyReplay;
        response.retained = true;
        serde_json::to_vec(&response)
            .map_err(|_| Self::invalid("shepherd_response_encoding_failed"))
    }
}

impl OperationExecutor for ResidentShepherdProbeExecutor {
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
            let cancellation = CancellationToken::new();
            self.inner
                .execute_provider_request(request, &cancellation, false)
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
        Box::pin(
            self.inner
                .execute_provider_request(request, cancellation, false),
        )
    }

    fn replay_payload(&self, payload: &[u8]) -> Result<Vec<u8>, ExecutorError> {
        self.inner.replay_payload(payload)
    }
}

fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
}
