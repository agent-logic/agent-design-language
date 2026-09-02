use std::{future::Future, pin::Pin, sync::Arc, time::Instant};

use tokio_util::sync::CancellationToken;

use crate::{
    invoke_ollama_model, ExecutorError, FailureClass, OperationExecutor, OperationRequest,
    ResidentShepherdInitConfig, ShepherdExecutionClass, ShepherdProvenance, ShepherdRequest,
    ShepherdResponse, SHEPHERD_REQUEST_SCHEMA, SHEPHERD_RESPONSE_SCHEMA,
};

/// Provider-backed production Shepherd executor. The native executor remains
/// responsible for Runtime admission records; reasoning requests are routed to
/// the configured provider without granting it lifecycle authority.
pub struct ResidentShepherdExecutor {
    runtime_id: String,
    config: ResidentShepherdInitConfig,
    admission: Arc<dyn OperationExecutor>,
}

impl ResidentShepherdExecutor {
    pub fn new(
        runtime_id: impl Into<String>,
        config: ResidentShepherdInitConfig,
        admission: Arc<dyn OperationExecutor>,
    ) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            config,
            admission,
        }
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
            let Ok(shepherd_request) = parsed else {
                return self
                    .admission
                    .execute_with_cancellation(request, cancellation)
                    .await;
            };
            if shepherd_request.schema != SHEPHERD_REQUEST_SCHEMA
                || shepherd_request.runtime_id != self.runtime_id
                || shepherd_request.prompt.trim().is_empty()
            {
                return Err(Self::invalid("shepherd_invalid_request"));
            }
            let started = Instant::now();
            let response = invoke_ollama_model(
                &self.config.endpoint,
                &self.config.model,
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
                backend_identity_sha256: Some(sha256(self.config.provider.as_bytes())),
                model_identity_sha256: sha256(self.config.model.as_bytes()),
                model_artifact_sha256: None,
                runner_program_sha256: sha256(self.config.endpoint.as_bytes()),
                runner_launch_sha256: sha256(
                    format!("{}:{}", self.config.provider, self.config.model).as_bytes(),
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
