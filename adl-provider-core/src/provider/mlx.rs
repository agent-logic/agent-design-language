//! Bounded local MLX-LM transport. The operator owns the model server and its
//! Metal memory ceiling. It must run offline with a preloaded model; a model
//! request to an online MLX-LM server can otherwise trigger a server-side download.
use super::*;
use reqwest::{blocking::Client, redirect::Policy, Url};
use serde_json::json;

/// Apple-silicon provider using MLX-LM's local chat-completion protocol.
pub struct MlxProvider {
    endpoint: Url,
    model: String,
    timeout: Duration,
    max_tokens: u64,
    temperature: f64,
    top_p: f64,
    seed: Option<u32>,
}

const MAX_PROMPT_BYTES: usize = 16_384;
const MAX_RESPONSE_BYTES: u64 = 1_048_576;

impl MlxProvider {
    /// Consume the shared canonical invocation target; no private registry or
    /// fallback model is maintained here.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            return Err(invalid_config(
                "mlx",
                "unsupported_platform: requires macOS Apple silicon",
            ));
        }
        Self::validated(spec, target)
    }

    fn validated(_spec: &adl::ProviderSpec, target: &ProviderInvocationTargetV1) -> Result<Self> {
        if target.provider_kind != "mlx"
            || target.capabilities.tool_calling.supported
            || target.capabilities.semantic_tool_fallback.supported
            || target.capabilities.speech_synthesis.supported
            || target.capabilities.speech_transcription.supported
            || target.capabilities.structured_json.mode
                == provider_substrate::CapabilityModeV1::Native
        {
            return Err(invalid_config(
                "mlx",
                "MLX adapter supports text completion only; incompatible capability declaration",
            ));
        }
        let endpoint = target
            .endpoint
            .as_deref()
            .or(target.base_url.as_deref())
            .ok_or_else(|| invalid_config("mlx", "endpoint is required"))?;
        let endpoint =
            Url::parse(endpoint).map_err(|_| invalid_config("mlx", "invalid local endpoint"))?;
        let host = endpoint.host_str().unwrap_or_default();
        if endpoint.scheme() != "http"
            || !matches!(host, "127.0.0.1" | "[::1]")
            || endpoint.path() != "/v1/chat/completions"
            || !endpoint.username().is_empty()
            || endpoint.password().is_some()
            || endpoint.query().is_some()
            || endpoint.fragment().is_some()
        {
            return Err(invalid_config("mlx", "endpoint must be literal loopback HTTP /v1/chat/completions without credentials, query or fragment"));
        }
        // The substrate rejects its generic fallback for MLX before target creation.
        if target.provider_model_id == "default_model"
            || target.provider_model_id.trim().is_empty()
            || target.provider_model_id.len() > 512
            || target.provider_model_id.chars().any(char::is_control)
        {
            return Err(invalid_config(
                "mlx",
                "explicit bounded model identity is required",
            ));
        }
        let effective = &target.effective_inference;
        Ok(Self {
            endpoint,
            model: target.provider_model_id.clone(),
            timeout: Duration::from_secs(effective.timeout_secs.ok_or_else(|| {
                invalid_config("mlx", "timeout_secs must be an integer in 1..=120")
            })?),
            max_tokens: effective.max_output_tokens.ok_or_else(|| {
                invalid_config("mlx", "max_output_tokens must be an integer in 1..=512")
            })?,
            temperature: effective.temperature.unwrap_or(0.0),
            top_p: effective.top_p.unwrap_or(1.0),
            seed: effective
                .deterministic_seed
                .map(u32::try_from)
                .transpose()
                .map_err(|_| {
                    invalid_config(
                        "mlx",
                        "deterministic_seed must be an integer in 0..=4294967295",
                    )
                })?,
        })
    }
}

impl Provider for MlxProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        if prompt.trim().is_empty() || prompt.len() > MAX_PROMPT_BYTES {
            return Err(invalid_config(
                "mlx",
                "prompt must be nonempty and at most 16384 bytes",
            ));
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .timeout(self.timeout)
            .build()
            .map_err(|_| runtime_error_non_retryable("mlx", "cannot construct local client"))?;
        let mut body = json!({
            "model": self.model, "messages": [{"role": "user", "content": prompt}],
            "max_tokens": self.max_tokens, "temperature": self.temperature, "top_p": self.top_p, "stream": false
        });
        if let Some(seed) = self.seed {
            body["seed"] = json!(seed);
        }
        let mut response = client
            .post(self.endpoint.clone())
            .json(&body)
            .send()
            .map_err(|error| {
                if error.is_timeout() {
                    timeout_error("mlx", "local request deadline exceeded")
                } else {
                    runtime_error(
                        "mlx",
                        "local service unavailable; check server and selected model",
                    )
                }
            })?;
        if !response.status().is_success() {
            return Err(runtime_error_non_retryable(
                "mlx",
                format!(
                    "local service returned HTTP {}; check selected model/service",
                    response.status().as_u16()
                ),
            ));
        }
        let mut bytes = Vec::new();
        response
            .by_ref()
            .take(MAX_RESPONSE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| timeout_error("mlx", "local response incomplete or deadline exceeded"))?;
        if bytes.len() as u64 > MAX_RESPONSE_BYTES {
            return Err(runtime_error_non_retryable(
                "mlx",
                "local response exceeds byte limit",
            ));
        }
        let payload: Value = serde_json::from_slice(&bytes)
            .map_err(|_| runtime_error_non_retryable("mlx", "malformed local JSON response"))?;
        if payload.get("model").and_then(Value::as_str) != Some(self.model.as_str()) {
            return Err(runtime_error_non_retryable(
                "mlx",
                "response model does not match selected model",
            ));
        }
        let choices = payload
            .get("choices")
            .and_then(Value::as_array)
            .filter(|choices| choices.len() == 1)
            .ok_or_else(|| runtime_error_non_retryable("mlx", "expected one completion choice"))?;
        let text = choices[0]
            .pointer("/message/content")
            .and_then(Value::as_str)
            .filter(|text| !text.trim().is_empty())
            .ok_or_else(|| {
                runtime_error_non_retryable("mlx", "response missing nonempty message content")
            })?;
        Ok(text.to_owned())
    }
}

#[cfg(test)]
#[path = "mlx/tests.rs"]
mod tests;
