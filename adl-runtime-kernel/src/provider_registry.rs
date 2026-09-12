//! Async execution boundary for the process-owned canonical provider registry.
use adl_provider_core::registry::{
    ProviderBinding, ProviderFailure, ProviderProjection, ProviderRegistry,
};
use std::sync::{Arc, OnceLock};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

fn calls() -> Arc<Semaphore> {
    static CALLS: OnceLock<Arc<Semaphore>> = OnceLock::new();
    CALLS.get_or_init(|| Arc::new(Semaphore::new(8))).clone()
}
pub async fn validate(
    registry: Arc<ProviderRegistry>,
    binding: ProviderBinding,
) -> Result<ProviderProjection, ProviderFailure> {
    let permit = calls()
        .acquire_owned()
        .await
        .map_err(|_| ProviderFailure::Transport)?;
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        registry.prepare(&binding).map(|p| p.projection)
    })
    .await
    .map_err(|_| ProviderFailure::InvalidConfiguration)?
}
pub async fn complete(
    registry: Arc<ProviderRegistry>,
    binding: ProviderBinding,
    prompt: String,
    cancellation: &CancellationToken,
) -> Result<String, ProviderFailure> {
    let semaphore = calls();
    let permit = tokio::select! {
        _=cancellation.cancelled()=>return Err(ProviderFailure::Cancelled),
        permit=semaphore.acquire_owned()=>permit.map_err(|_|ProviderFailure::Transport)?,
    };
    let cancel = cancellation.clone();
    let mut work = tokio::task::spawn_blocking(move || {
        // Held until the transport finishes even if the caller cancels its wait.
        let _permit = permit;
        if cancel.is_cancelled() {
            return Err(ProviderFailure::Cancelled);
        }
        let prepared = registry.prepare(&binding)?;
        if cancel.is_cancelled() {
            return Err(ProviderFailure::Cancelled);
        }
        let out = prepared.executor.complete(&prompt).map_err(|error| {
            if let Some(failure) = error.downcast_ref::<ProviderFailure>() {
                *failure
            } else {
                match adl_provider_core::failure_category(&error) {
                    "credentials" => ProviderFailure::Credentials,
                    "quota" => ProviderFailure::Quota,
                    "unsupported_capability" => ProviderFailure::UnsupportedCapability,
                    "model_unavailable" => ProviderFailure::ModelUnavailable,
                    "timeout" => ProviderFailure::Timeout,
                    "invalid_response" => ProviderFailure::InvalidResponse,
                    "invalid_configuration" => ProviderFailure::InvalidConfiguration,
                    _ => ProviderFailure::Transport,
                }
            }
        })?;
        if out.trim().is_empty() || out.len() > 4_194_304 {
            return Err(ProviderFailure::InvalidResponse);
        }
        Ok(out)
    });
    tokio::select! {
        _=cancellation.cancelled()=>Err(ProviderFailure::Cancelled),
        result=&mut work=>result.map_err(|_|ProviderFailure::InvalidResponse)?,
    }
}
