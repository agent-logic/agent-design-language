use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
    time::Instant,
};

use tokio_util::sync::CancellationToken;

use crate::{
    invoke_ollama_model, shepherd::decode_request, ExecutorError, FailureClass, OperationExecutor,
    OperationRequest, ResidentShepherdInitConfig, ShepherdExecutionClass, ShepherdProvenance,
    ShepherdRequest, ShepherdResponse, SHEPHERD_RESPONSE_SCHEMA,
};

/// Provider adapters that are compiled into this Runtime build. Configuration
/// remains provider-shaped, but startup must reject profiles that have no
/// executable adapter instead of admitting a permanently degraded resident.
pub fn resident_shepherd_provider_is_available(provider: &str) -> bool {
    matches!(provider, "ollama")
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
        Box::pin(async move {
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
            if !self.ready.is_ready(shepherd_name) {
                return Err(ExecutorError {
                    class: FailureClass::Retryable,
                    message: "shepherd_model_not_ready".to_owned(),
                });
            }
            if config.provider != "ollama" {
                return Err(ExecutorError {
                    class: FailureClass::Retryable,
                    message: "resident_shepherd_provider_unsupported".to_owned(),
                });
            }
            let started = Instant::now();
            let response = invoke_ollama_model(
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
        })
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

fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
}
