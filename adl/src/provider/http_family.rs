//! HTTP-based provider implementations and request transport helpers.
//!
//! Supports OpenAI, Anthropic, DeepSeek, OpenRouter, Z.ai, generic HTTP, and Ollama-HTTP style backends.
use super::*;
use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_bedrockruntime as bedrockruntime;
use aws_sdk_sts as sts;
use fs2::FileExt;
use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::process::Command;
use std::thread;
use std::time::Duration;

mod config;

use config::{
    auth_env_for, cfg_bool_opt, cfg_f64_strict, cfg_u64_strict, endpoint_host,
    is_loopback_endpoint, ollama_generate_endpoint, validate_http_credential_endpoint,
    validate_vendor_credential_endpoint, vendor_endpoint, HttpAuth,
};
pub(crate) use config::{cfg_u64, timeout_secs};

struct InvocationArtifactLock {
    _file: File,
}

const INVOCATION_ARTIFACT_LOCK_TIMEOUT: Duration = Duration::from_secs(2);
const INVOCATION_ARTIFACT_LOCK_TIMEOUT_ENV: &str = "ADL_INVOCATION_LOCK_TIMEOUT_MS";

fn invocation_lock_path(path: &Path) -> PathBuf {
    let mut os = path.as_os_str().to_os_string();
    os.push(".lock");
    PathBuf::from(os)
}

fn acquire_invocation_artifact_lock(path: &Path) -> std::io::Result<InvocationArtifactLock> {
    let lock_path = invocation_lock_path(path);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    let started = Instant::now();
    let timeout = invocation_artifact_lock_timeout();
    loop {
        match file.try_lock_exclusive() {
            Ok(()) => return Ok(InvocationArtifactLock { _file: file }),
            Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if started.elapsed() > timeout {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "timed out waiting for invocation artifact lock",
                    ));
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(err) => return Err(err),
        }
    }
}

fn invocation_artifact_lock_timeout() -> Duration {
    env::var(INVOCATION_ARTIFACT_LOCK_TIMEOUT_ENV)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(INVOCATION_ARTIFACT_LOCK_TIMEOUT)
}

/// Maximum number of provider error-body characters kept for inline request-failure messages.
const MAX_PROVIDER_ERROR_BODY_BYTES: usize = 200;

fn truncate_provider_body(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.len() <= MAX_PROVIDER_ERROR_BODY_BYTES {
        return trimmed.to_string();
    }

    let end = trimmed
        .char_indices()
        .map(|(idx, _)| idx)
        .chain(std::iter::once(trimmed.len()))
        .take_while(|idx| *idx <= MAX_PROVIDER_ERROR_BODY_BYTES)
        .last()
        .unwrap_or(0);
    trimmed[..end].to_string()
}

fn provider_http_json(
    provider_label: &str,
    req: reqwest::blocking::RequestBuilder,
) -> Result<(Value, u16)> {
    let resp = provider_http_response(provider_label, req)?;

    let http_status = resp.status().as_u16();
    let json = resp
        .json()
        .context("native provider response was not valid JSON")
        .map_err(|err| runtime_error_non_retryable(provider_label, err.to_string()))?;
    Ok((json, http_status))
}

fn provider_http_text(
    provider_label: &str,
    req: reqwest::blocking::RequestBuilder,
) -> Result<(String, u16)> {
    let resp = provider_http_response(provider_label, req)?;
    let http_status = resp.status().as_u16();
    let text = resp
        .text()
        .context("native provider response body could not be read")
        .map_err(|err| runtime_error(provider_label, err.to_string()))?;
    Ok((text, http_status))
}

fn provider_http_response(
    provider_label: &str,
    req: reqwest::blocking::RequestBuilder,
) -> Result<reqwest::blocking::Response> {
    let resp = match req.send() {
        Ok(resp) => resp,
        Err(err) => {
            if err.is_timeout() {
                return Err(timeout_error(
                    provider_label,
                    "kind=timeout native provider request timed out",
                ));
            }
            return Err(runtime_error(
                provider_label,
                format!("kind=request_failed native provider request failed: {err}"),
            ));
        }
    };

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        let class = if status.is_client_error() {
            "client_error"
        } else if status.is_server_error() {
            "server_error"
        } else {
            "http_error"
        };
        let msg = format!(
            "kind={class} status={status} body={}",
            truncate_provider_body(&text)
        );
        if status.is_client_error() {
            return Err(runtime_error_non_retryable(provider_label, msg));
        }
        return Err(runtime_error(provider_label, msg));
    }
    Ok(resp)
}

fn write_native_invocation_record(
    family: &str,
    model: &str,
    prompt: &str,
    output: &str,
    http_status: u16,
) -> Result<()> {
    let Some(path) = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH") else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            post_success_invocation_artifact_io_error(
                family,
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let _artifact_lock = acquire_invocation_artifact_lock(&path).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("partial_success_unknown_invocation_record_lock_unavailable: provider call completed but invocation artifact lock could not be acquired without risking duplicate retry: {err}"),
        )
    })?;
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            post_success_invocation_artifact_io_error(
                family,
                format!("failed to read provider invocation artifact: {err}"),
            )
        })?)
        .map_err(|err| {
            runtime_error_non_retryable(
                family,
                format!("provider invocation artifact is invalid JSON: {err}"),
            )
        })?
    } else {
        serde_json::json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_only_no_secret_material_recorded",
            "invocations": []
        })
    };

    let Some(invocations) = payload
        .get_mut("invocations")
        .and_then(|v| v.as_array_mut())
    else {
        return Err(runtime_error_non_retryable(
            family,
            "provider invocation artifact missing invocations array",
        ));
    };
    let timestamp_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    invocations.push(serde_json::json!({
        "family": family,
        "model": model,
        "http_status": http_status,
        "timestamp_unix_ms": timestamp_unix_ms,
        "prompt_chars": prompt.chars().count(),
        "output_chars": output.chars().count()
    }));
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            family,
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        post_success_invocation_artifact_io_error(
            family,
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

struct BedrockInvocationRecord<'a> {
    model: &'a str,
    prompt: &'a str,
    output: &'a str,
    http_status: u16,
    profile: &'a str,
    region: &'a str,
    account_id_sha256: Option<&'a str>,
    account_profile_validation_status: &'a str,
}

fn write_bedrock_invocation_record(record: BedrockInvocationRecord<'_>) -> Result<()> {
    let Some(path) = env::var_os("ADL_PROVIDER_INVOCATIONS_PATH") else {
        return Ok(());
    };
    let path = PathBuf::from(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            post_success_invocation_artifact_io_error(
                "bedrock",
                format!("failed to create provider invocation artifact directory: {err}"),
            )
        })?;
    }
    let _artifact_lock = acquire_invocation_artifact_lock(&path).map_err(|err| {
        runtime_error_non_retryable(
            "bedrock",
            format!("partial_success_unknown_invocation_record_lock_unavailable: Bedrock call completed but invocation artifact lock could not be acquired without risking duplicate retry: {err}"),
        )
    })?;
    let mut payload = if path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&path).map_err(|err| {
            post_success_invocation_artifact_io_error(
                "bedrock",
                format!("failed to read provider invocation artifact: {err}"),
            )
        })?)
        .map_err(|err| {
            runtime_error_non_retryable(
                "bedrock",
                format!("provider invocation artifact is invalid JSON: {err}"),
            )
        })?
    } else {
        serde_json::json!({
            "schema_version": "adl.native_provider_invocations.v1",
            "credential_policy": "operator_env_or_aws_profile_only_no_secret_material_recorded",
            "invocations": []
        })
    };

    let Some(invocations) = payload
        .get_mut("invocations")
        .and_then(|v| v.as_array_mut())
    else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "provider invocation artifact missing invocations array",
        ));
    };
    let timestamp_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    invocations.push(serde_json::json!({
        "family": "bedrock",
        "model": record.model,
        "http_status": record.http_status,
        "timestamp_unix_ms": timestamp_unix_ms,
        "prompt_chars": record.prompt.chars().count(),
        "output_chars": record.output.chars().count(),
        "aws_profile": record.profile,
        "aws_region": record.region,
        "account_id_sha256": record.account_id_sha256,
        "account_profile_validation_status": record.account_profile_validation_status
    }));
    let bytes = serde_json::to_vec_pretty(&payload).map_err(|err| {
        runtime_error_non_retryable(
            "bedrock",
            format!("failed to serialize provider invocation artifact: {err}"),
        )
    })?;
    write_file_atomic(&path, &bytes).map_err(|err| {
        post_success_invocation_artifact_io_error(
            "bedrock",
            format!("failed to write invocation artifact: {err}"),
        )
    })
}

fn post_success_invocation_artifact_io_error(
    provider: &str,
    message: impl Into<String>,
) -> anyhow::Error {
    runtime_error_non_retryable(
        provider,
        format!(
            "partial_success_unknown_invocation_record_io_failure: provider call completed but invocation artifact I/O failed without a safe retry boundary: {}",
            message.into()
        ),
    )
}

fn write_file_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut os = path.as_os_str().to_os_string();
    os.push(format!(
        ".tmp-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let tmp = PathBuf::from(os);
    fs::write(&tmp, bytes)?;
    fs::rename(tmp, path)
}

fn extract_openai_output_text(json: &Value) -> Option<String> {
    if let Some(text) = json.get("output_text").and_then(|v| v.as_str()) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    let mut chunks = Vec::new();
    for item in json.get("output")?.as_array()? {
        for content in item.get("content").and_then(|v| v.as_array())? {
            if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_anthropic_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    if let Some(contents) = json.get("content").and_then(|v| v.as_array()) {
        for content in contents {
            let content_type = content.get("type").and_then(|v| v.as_str());
            if content_type == Some("text") {
                if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                    chunks.push(text);
                }
            }
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    if joined.is_empty() && json.get("stop_reason").and_then(|v| v.as_str()) == Some("refusal") {
        return Some(r#"{"refusal":"provider refused the request"}"#.to_string());
    }
    (!joined.is_empty()).then_some(joined)
}

fn extract_deepseek_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    for choice in json.get("choices")?.as_array()? {
        if let Some(text) = choice
            .get("message")
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str())
        {
            chunks.push(text);
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_openrouter_output_text(json: &Value) -> Option<String> {
    extract_deepseek_output_text(json)
}

fn extract_gemini_output_text(json: &Value) -> Option<String> {
    let parts = json
        .pointer("/candidates/0/content/parts")
        .and_then(|v| v.as_array())?;
    let mut text_chunks = Vec::new();
    let mut tool_calls = Vec::new();
    for part in parts {
        if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
            let text = text.trim();
            if !text.is_empty() {
                text_chunks.push(text.to_string());
            }
        }
        if let Some(function_call) = part.get("functionCall").and_then(|v| v.as_object()) {
            if let Some(name) = function_call.get("name").and_then(|v| v.as_str()) {
                let args = function_call
                    .get("args")
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!({}));
                tool_calls.push(serde_json::json!({
                    "name": name,
                    "arguments": args,
                }));
            }
        }
    }
    if !tool_calls.is_empty() {
        let mut envelope = serde_json::json!({ "tool_calls": tool_calls });
        let text = text_chunks.join("\n");
        if !text.is_empty() {
            envelope["text"] = serde_json::json!(text);
        }
        return serde_json::to_string(&envelope).ok();
    }
    let joined = text_chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

fn extract_bedrock_nova_output_text(json: &Value) -> Option<String> {
    let mut chunks = Vec::new();
    if let Some(content) = json
        .pointer("/output/message/content")
        .and_then(|v| v.as_array())
    {
        for part in content {
            if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
                chunks.push(text);
            }
        }
    }
    if chunks.is_empty() {
        if let Some(text) = json.get("outputText").and_then(|v| v.as_str()) {
            chunks.push(text);
        }
    }
    let joined = chunks.join("\n").trim().to_string();
    (!joined.is_empty()).then_some(joined)
}

#[derive(Debug, Clone)]
/// OpenAI-compatible provider backed by HTTP/requests API.
pub struct OpenAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_output_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenAiProvider {
    /// Build an OpenAI provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(spec, target, OPENAI_RESPONSES_ENDPOINT, "openai")?;
        let auth_env = auth_env_for(spec, "OPENAI_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "openai",
            &endpoint,
            &auth_env,
            "OPENAI_API_KEY",
            &["api.openai.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_output_tokens: cfg_u64(&spec.config, "max_output_tokens").unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OpenAiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "openai",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build OpenAI client")
            .map_err(|err| runtime_error("openai", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "input": prompt,
                "max_output_tokens": self.max_output_tokens,
            }));
        let (json, http_status) = provider_http_json("openai", req)?;
        let output = extract_openai_output_text(&json)
            .ok_or_else(|| runtime_error_non_retryable("openai", "response missing text output"))?;
        write_native_invocation_record("openai", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// Anthropic-compatible provider using the messages API format.
pub struct AnthropicProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AnthropicProvider {
    /// Build an Anthropic provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(spec, target, ANTHROPIC_MESSAGES_ENDPOINT, "anthropic")?;
        let auth_env = auth_env_for(spec, "ANTHROPIC_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "anthropic",
            &endpoint,
            &auth_env,
            "ANTHROPIC_API_KEY",
            &["api.anthropic.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for AnthropicProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "anthropic",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Anthropic client")
            .map_err(|err| runtime_error("anthropic", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .header("x-api-key", token)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(&serde_json::json!({
                "model": self.model,
                "max_tokens": self.max_tokens,
                "messages": [{"role": "user", "content": prompt}],
            }));
        let (json, http_status) = provider_http_json("anthropic", req)?;
        let output = extract_anthropic_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("anthropic", "response missing text output")
        })?;
        write_native_invocation_record("anthropic", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// DeepSeek native provider using the chat completions API format.
pub struct DeepSeekProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl DeepSeekProvider {
    /// Build a DeepSeek provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint =
            vendor_endpoint(spec, target, DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT, "deepseek")?;
        let auth_env = auth_env_for(spec, "DEEPSEEK_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "deepseek",
            &endpoint,
            &auth_env,
            "DEEPSEEK_API_KEY",
            &["api.deepseek.com"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for DeepSeekProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "deepseek",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build DeepSeek client")
            .map_err(|err| runtime_error("deepseek", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": self.max_tokens,
                "stream": false,
            }));
        let (json, http_status) = provider_http_json("deepseek", req)?;
        let output = extract_deepseek_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("deepseek", "response missing message content")
        })?;
        write_native_invocation_record("deepseek", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// OpenRouter native provider using the OpenAI-compatible chat completions format.
pub struct OpenRouterProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl OpenRouterProvider {
    /// Build an OpenRouter provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let endpoint = vendor_endpoint(
            spec,
            target,
            OPENROUTER_CHAT_COMPLETIONS_ENDPOINT,
            "openrouter",
        )?;
        let auth_env = auth_env_for(spec, "OPENROUTER_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "openrouter",
            &endpoint,
            &auth_env,
            "OPENROUTER_API_KEY",
            &["openrouter.ai"],
        )?;
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for OpenRouterProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "openrouter",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build OpenRouter client")
            .map_err(|err| runtime_error("openrouter", err.to_string()))?;
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": self.max_tokens,
                "stream": false,
            }));
        let (json, http_status) = provider_http_json("openrouter", req)?;
        let output = extract_openrouter_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("openrouter", "response missing message content")
        })?;
        write_native_invocation_record("openrouter", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

const DEFAULT_BEDROCK_PROFILE: &str = "agent-logic-admin";
const DEFAULT_BEDROCK_REGION: &str = "us-west-2";
const BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV: &str = "ADL_AWS_BEDROCK_ACCOUNT_SHA256";

#[derive(Debug, Clone)]
/// AWS Bedrock native provider using Bedrock Runtime InvokeModel.
pub struct AwsBedrockProvider {
    model: String,
    region: String,
    profile: String,
    expected_account_sha256: Option<String>,
    max_tokens: u64,
    timeout_secs: Option<u64>,
}

impl AwsBedrockProvider {
    /// Build an AWS Bedrock provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let region = cfg_string(&spec.config, "region")
            .or_else(|| env::var("AWS_REGION").ok())
            .or_else(|| env::var("AWS_DEFAULT_REGION").ok())
            .unwrap_or_else(|| DEFAULT_BEDROCK_REGION.to_string());
        let profile = cfg_string(&spec.config, "profile")
            .or_else(|| env::var("ADL_AWS_PROFILE").ok())
            .or_else(|| env::var("AWS_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_BEDROCK_PROFILE.to_string());
        if profile != DEFAULT_BEDROCK_PROFILE {
            return Err(invalid_config(
                "bedrock",
                format!(
                    "AWS Bedrock provider requires Agent Logic AWS profile '{DEFAULT_BEDROCK_PROFILE}' (got '{profile}')"
                ),
            ));
        }
        let config_expected_account_sha256 = cfg_string(&spec.config, "expected_account_sha256")
            .or_else(|| cfg_string(&spec.config, "expected-account-sha256"));
        let env_expected_account_sha256 = env::var(BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV).ok();
        let expected_account_sha256 = match (
            env_expected_account_sha256.as_deref(),
            config_expected_account_sha256.as_deref(),
        ) {
            (Some(env_expected), Some(config_expected)) => {
                let env_expected = normalize_sha256_hex(env_expected)
                    .map_err(|err| invalid_config("bedrock", err))?;
                validate_sha256_hex(config_expected)
                    .map_err(|err| invalid_config("bedrock", err))?;
                let config_expected = config_expected.to_ascii_lowercase();
                if env_expected != config_expected {
                    return Err(invalid_config(
                        "bedrock",
                        format!(
                            "{BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV} is authoritative and conflicts with config.expected_account_sha256"
                        ),
                    ));
                }
                Some(env_expected)
            }
            (Some(env_expected), None) => Some(
                normalize_sha256_hex(env_expected).map_err(|err| invalid_config("bedrock", err))?,
            ),
            (None, Some(config_expected)) => Some(
                normalize_sha256_hex(config_expected)
                    .map_err(|err| invalid_config("bedrock", err))?,
            ),
            (None, None) => None,
        };
        if let Some(expected) = expected_account_sha256.as_deref() {
            validate_sha256_hex(expected).map_err(|err| invalid_config("bedrock", err))?;
        }
        Ok(Self {
            model: target.provider_model_id.clone(),
            region,
            profile,
            expected_account_sha256,
            max_tokens: cfg_u64(&spec.config, "max_tokens")
                .or_else(|| cfg_u64(&spec.config, "max_output_tokens"))
                .unwrap_or(220),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }

    async fn complete_async(&self, prompt: &str) -> Result<String> {
        let region_provider =
            RegionProviderChain::first_try(Some(aws_config::Region::new(self.region.clone())));
        let mut timeout_config = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(5))
            .operation_timeout(Duration::from_secs(self.timeout_secs.unwrap_or(45)));
        if let Some(secs) = self.timeout_secs {
            timeout_config = timeout_config.operation_attempt_timeout(Duration::from_secs(secs));
        }
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .profile_name(&self.profile)
            .timeout_config(timeout_config.build())
            .load()
            .await;
        let identity = sts::Client::new(&shared_config)
            .get_caller_identity()
            .send()
            .await
            .map_err(|err| bedrock_sdk_error(format!("{err:?}")))?;
        let account_id_sha256 = identity.account().map(sha256_hex);
        verify_bedrock_account_identity(
            account_id_sha256.as_deref(),
            self.expected_account_sha256.as_deref(),
        )?;
        let body = bedrock_nova_request_body(prompt, self.max_tokens);
        let response = bedrockruntime::Client::new(&shared_config)
            .invoke_model()
            .model_id(&self.model)
            .content_type("application/json")
            .accept("application/json")
            .body(bedrockruntime::primitives::Blob::new(
                serde_json::to_vec(&body).map_err(|err| {
                    runtime_error_non_retryable(
                        "bedrock",
                        format!("failed to serialize Bedrock request: {err}"),
                    )
                })?,
            ))
            .send()
            .await
            .map_err(|err| bedrock_sdk_error(format!("{err:?}")))?;
        let json: Value = serde_json::from_slice(response.body().as_ref()).map_err(|err| {
            runtime_error_non_retryable("bedrock", format!("invalid Bedrock JSON: {err}"))
        })?;
        let output = extract_bedrock_nova_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("bedrock", "response missing Bedrock output text")
        })?;
        write_bedrock_invocation_record(BedrockInvocationRecord {
            model: &self.model,
            prompt,
            output: &output,
            http_status: 200,
            profile: &self.profile,
            region: &self.region,
            account_id_sha256: account_id_sha256.as_deref(),
            account_profile_validation_status: "account_hash_verified",
        })?;
        Ok(output)
    }
}

fn validate_sha256_hex(value: &str) -> std::result::Result<(), String> {
    if value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("expected account hash must be a 64-character SHA-256 hex digest".to_string())
    }
}

fn normalize_sha256_hex(value: &str) -> std::result::Result<String, String> {
    validate_sha256_hex(value)?;
    Ok(value.to_ascii_lowercase())
}

fn verify_bedrock_account_identity(
    account_id_sha256: Option<&str>,
    expected_account_sha256: Option<&str>,
) -> Result<()> {
    let Some(expected) = expected_account_sha256 else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            format!(
                "AWS Bedrock provider requires operator-approved expected account hash; set {BEDROCK_EXPECTED_ACCOUNT_SHA256_ENV} or config.expected_account_sha256"
            ),
        ));
    };
    let expected = normalize_sha256_hex(expected).map_err(|err| invalid_config("bedrock", err))?;
    let Some(observed) = account_id_sha256 else {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "AWS Bedrock STS identity did not include an account id",
        ));
    };
    if observed != expected {
        return Err(runtime_error_non_retryable(
            "bedrock",
            "AWS Bedrock profile account hash does not match expected Agent Logic account hash",
        ));
    }
    Ok(())
}

impl Provider for AwsBedrockProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| runtime_error("bedrock", format!("failed to build runtime: {err}")))?
            .block_on(self.complete_async(prompt))
    }
}

#[derive(Debug, Clone)]
/// Google Vertex AI Gemini provider using the generateContent API format.
pub struct VertexAiGeminiProvider {
    endpoint: String,
    auth: VertexAiAuth,
    model: String,
    max_output_tokens: u64,
    timeout_secs: Option<u64>,
    tools: Option<Value>,
    thinking_config: Option<Value>,
}

impl VertexAiGeminiProvider {
    /// Build a Vertex AI Gemini provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let project = required_cfg_string(&spec.config, "project", "vertex_ai_gemini")?;
        let location = required_cfg_string(&spec.config, "location", "vertex_ai_gemini")?;
        let endpoint = cfg_string(&spec.config, "endpoint").unwrap_or_else(|| {
            vertex_ai_gemini_endpoint(&project, &location, &target.provider_model_id)
        });
        validate_vertex_ai_endpoint(spec, &endpoint)?;
        Ok(Self {
            endpoint,
            auth: vertex_ai_auth_from_config(spec)?,
            model: target.provider_model_id.clone(),
            max_output_tokens: cfg_u64(&spec.config, "max_output_tokens").unwrap_or(1024),
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
            tools: vertex_ai_tools_from_config(&spec.config)?,
            thinking_config: vertex_ai_thinking_config_from_config(&spec.config)?,
        })
    }

    fn complete_with_mode(
        &self,
        prompt: &str,
        streaming: bool,
        mut on_chunk: Option<&mut dyn FnMut(&str)>,
    ) -> Result<String> {
        let token = self.auth.resolve_token()?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Vertex AI Gemini client")
            .map_err(|err| runtime_error("vertex_ai_gemini", err.to_string()))?;
        let endpoint = if streaming {
            vertex_ai_gemini_stream_endpoint(&self.endpoint)
        } else {
            self.endpoint.clone()
        };
        let req = client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&vertex_ai_gemini_request_body(
                prompt,
                self.max_output_tokens,
                self.thinking_config.as_ref(),
                self.tools.as_ref(),
            ));
        let (output, http_status) = if streaming {
            let (body, http_status) = provider_http_text("vertex_ai_gemini", req)?;
            let output = if let Some(callback) = on_chunk.as_mut() {
                extract_vertex_ai_stream_output(&body, Some(&mut **callback))?
            } else {
                extract_vertex_ai_stream_output(&body, None)?
            };
            (output, http_status)
        } else {
            let (json, http_status) = provider_http_json("vertex_ai_gemini", req)?;
            let output = extract_gemini_output_text(&json).ok_or_else(|| {
                runtime_error_non_retryable(
                    "vertex_ai_gemini",
                    "response missing Gemini text output",
                )
            })?;
            if let Some(callback) = on_chunk.as_mut() {
                callback(&output);
            }
            (output, http_status)
        };
        write_native_invocation_record(
            "vertex_ai_gemini",
            &self.model,
            prompt,
            &output,
            http_status,
        )?;
        Ok(output)
    }
}

impl Provider for VertexAiGeminiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        self.complete_with_mode(prompt, false, None)
    }

    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        self.complete_with_mode(prompt, true, Some(on_chunk))
    }
}

#[derive(Debug, Clone)]
enum VertexAiAuth {
    BearerEnv { env: String },
    Adc { env_override: String },
    WorkloadIdentity { env_override: String },
}

impl VertexAiAuth {
    fn resolve_token(&self) -> Result<String> {
        match self {
            Self::BearerEnv { env } => vertex_ai_env_token(env),
            Self::Adc { env_override } => {
                vertex_ai_env_token(env_override).or_else(|_| vertex_ai_gcloud_adc_token())
            }
            Self::WorkloadIdentity { env_override } => vertex_ai_env_token(env_override)
                .or_else(|_| vertex_ai_metadata_workload_identity_token()),
        }
    }
}

fn vertex_ai_auth_from_config(spec: &adl::ProviderSpec) -> Result<VertexAiAuth> {
    let Some(auth_val) = spec.config.get("auth") else {
        return Ok(VertexAiAuth::Adc {
            env_override: "ADL_VERTEX_AI_ACCESS_TOKEN".to_string(),
        });
    };
    let obj = auth_val
        .as_object()
        .ok_or_else(|| invalid_config("vertex_ai_gemini", "config.auth must be an object"))?;
    let auth_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| invalid_config("vertex_ai_gemini", "config.auth.type is required"))?;
    match auth_type {
        "bearer" => Ok(VertexAiAuth::BearerEnv {
            env: auth_env_for(spec, "ADL_VERTEX_AI_ACCESS_TOKEN")?,
        }),
        "adc" => Ok(VertexAiAuth::Adc {
            env_override: vertex_ai_auth_env_override(obj)?,
        }),
        "workload_identity" => Ok(VertexAiAuth::WorkloadIdentity {
            env_override: vertex_ai_auth_env_override(obj)?,
        }),
        other => Err(invalid_config(
            "vertex_ai_gemini",
            format!(
                "config.auth.type must be 'bearer', 'adc', or 'workload_identity' (got '{other}')"
            ),
        )),
    }
}

fn vertex_ai_auth_env_override(obj: &serde_json::Map<String, Value>) -> Result<String> {
    let env_key = obj
        .get("env")
        .and_then(|v| v.as_str())
        .unwrap_or("ADL_VERTEX_AI_ACCESS_TOKEN")
        .trim();
    if env_key.is_empty() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "config.auth.env must not be empty",
        ));
    }
    Ok(env_key.to_string())
}

fn vertex_ai_env_token(env_key: &str) -> Result<String> {
    let token = env::var(env_key).map_err(|_| {
        invalid_config(
            "vertex_ai_gemini",
            format!(
                "missing Vertex AI bearer token env var '{env_key}' and no ADC/workload identity token was available"
            ),
        )
    })?;
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            format!("Vertex AI bearer token env var '{env_key}' must not be empty"),
        ));
    }
    Ok(trimmed.to_string())
}

fn vertex_ai_gcloud_adc_token() -> Result<String> {
    let output = Command::new("gcloud")
        .args(["auth", "print-access-token", "--quiet"])
        .output()
        .map_err(|err| {
            invalid_config(
                "vertex_ai_gemini",
                format!("ADC token acquisition failed: gcloud auth print-access-token unavailable: {err}"),
            )
        })?;
    if !output.status.success() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "ADC token acquisition failed: gcloud auth print-access-token returned a non-zero status",
        ));
    }
    let token = String::from_utf8(output.stdout).map_err(|err| {
        invalid_config(
            "vertex_ai_gemini",
            format!("ADC token acquisition produced non-UTF-8 output: {err}"),
        )
    })?;
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "ADC token acquisition produced an empty token",
        ));
    }
    Ok(trimmed.to_string())
}

fn vertex_ai_metadata_workload_identity_token() -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .context("failed to build Vertex AI workload identity metadata client")
        .map_err(|err| runtime_error("vertex_ai_gemini", err.to_string()))?;
    let (json, _) = provider_http_json(
        "vertex_ai_gemini",
        client
            .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
            .header("Metadata-Flavor", "Google"),
    )?;
    let token = json
        .get("access_token")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            invalid_config(
                "vertex_ai_gemini",
                "workload identity metadata response missing access_token",
            )
        })?;
    Ok(token.to_string())
}

fn vertex_ai_gemini_endpoint(project: &str, location: &str, model: &str) -> String {
    let host = if location == "global" {
        "aiplatform.googleapis.com".to_string()
    } else {
        format!("{location}-aiplatform.googleapis.com")
    };
    format!(
        "https://{host}/v1/projects/{project}/locations/{location}/publishers/google/models/{model}:generateContent"
    )
}

fn vertex_ai_gemini_stream_endpoint(endpoint: &str) -> String {
    endpoint
        .strip_suffix(":generateContent")
        .map(|prefix| format!("{prefix}:streamGenerateContent"))
        .unwrap_or_else(|| endpoint.to_string())
}

fn vertex_ai_gemini_request_body(
    prompt: &str,
    max_output_tokens: u64,
    thinking_config: Option<&Value>,
    tools: Option<&Value>,
) -> Value {
    let mut body = serde_json::json!({
        "contents": [{"role": "user", "parts": [{"text": prompt}]}],
        "generationConfig": {
            "maxOutputTokens": max_output_tokens,
        },
    });
    if let Some(thinking_config) = thinking_config {
        body["generationConfig"]["thinkingConfig"] = thinking_config.clone();
    }
    if let Some(tools) = tools {
        body["tools"] = tools.clone();
    }
    body
}

fn extract_vertex_ai_stream_output(
    body: &str,
    mut on_chunk: Option<&mut dyn FnMut(&str)>,
) -> Result<String> {
    let frames = parse_vertex_ai_stream_frames(body)?;
    let mut output = String::new();
    for frame in frames {
        let Some(chunk) = extract_gemini_output_text(&frame) else {
            continue;
        };
        if let Some(callback) = on_chunk.as_deref_mut() {
            callback(&chunk);
        }
        output.push_str(&chunk);
    }
    if output.is_empty() {
        return Err(runtime_error_non_retryable(
            "vertex_ai_gemini",
            "streaming response missing Gemini text output",
        ));
    }
    Ok(output)
}

fn parse_vertex_ai_stream_frames(body: &str) -> Result<Vec<Value>> {
    if let Ok(value) = serde_json::from_str::<Value>(body) {
        return match value {
            Value::Array(items) => Ok(items),
            other => Ok(vec![other]),
        };
    }

    let mut frames = Vec::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let payload = line.strip_prefix("data:").map(str::trim).unwrap_or(line);
        if payload.is_empty() || payload == "[DONE]" {
            continue;
        }
        let frame = serde_json::from_str::<Value>(payload).map_err(|err| {
            runtime_error_non_retryable(
                "vertex_ai_gemini",
                format!("streaming response frame was not valid JSON: {err}"),
            )
        })?;
        frames.push(frame);
    }
    if frames.is_empty() {
        return Err(runtime_error_non_retryable(
            "vertex_ai_gemini",
            "streaming response did not contain JSON frames",
        ));
    }
    Ok(frames)
}

fn vertex_ai_tools_from_config(cfg: &HashMap<String, Value>) -> Result<Option<Value>> {
    if let Some(raw) = cfg.get("vertex_tools") {
        if raw.as_array().is_none() {
            return Err(invalid_config(
                "vertex_ai_gemini",
                "config.vertex_tools must be a Vertex tools array",
            ));
        }
        return Ok(Some(raw.clone()));
    }
    let Some(raw) = cfg.get("tools") else {
        return Ok(None);
    };
    let declarations = raw
        .as_array()
        .ok_or_else(|| invalid_config("vertex_ai_gemini", "config.tools must be a UTS array"))?
        .iter()
        .map(vertex_ai_function_declaration_from_uts)
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(
        serde_json::json!([{ "functionDeclarations": declarations }]),
    ))
}

fn vertex_ai_thinking_config_from_config(cfg: &HashMap<String, Value>) -> Result<Option<Value>> {
    let thinking_level = cfg_string(cfg, "thinking_level");
    let thinking_budget = cfg_u64_strict(cfg, "thinking_budget", "vertex_ai_gemini")?;
    let include_thoughts = cfg_bool_opt(cfg, "include_thoughts", "vertex_ai_gemini")?;

    if thinking_level.is_some() && thinking_budget.is_some() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "config.thinking_level and config.thinking_budget are mutually exclusive",
        ));
    }

    if thinking_level.is_none() && thinking_budget.is_none() && include_thoughts.is_none() {
        return Ok(None);
    }

    let mut thinking = serde_json::Map::new();
    if let Some(level) = thinking_level {
        let normalized = level.trim().to_ascii_uppercase();
        let allowed = ["MINIMAL", "LOW", "MEDIUM", "HIGH"];
        if !allowed.contains(&normalized.as_str()) {
            return Err(invalid_config(
                "vertex_ai_gemini",
                "config.thinking_level must be one of MINIMAL, LOW, MEDIUM, or HIGH",
            ));
        }
        thinking.insert("thinkingLevel".to_string(), Value::String(normalized));
    }
    if let Some(budget) = thinking_budget {
        thinking.insert("thinkingBudget".to_string(), serde_json::json!(budget));
    }
    if let Some(include) = include_thoughts {
        thinking.insert("includeThoughts".to_string(), Value::Bool(include));
    }

    Ok(Some(Value::Object(thinking)))
}

fn vertex_ai_function_declaration_from_uts(tool: &Value) -> Result<Value> {
    let name = tool.get("name").and_then(|v| v.as_str()).ok_or_else(|| {
        invalid_config(
            "vertex_ai_gemini",
            "each config.tools entry must include a string name",
        )
    })?;
    let description = tool
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let parameters = tool
        .get("input_schema")
        .or_else(|| tool.get("parameters"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({ "type": "object", "properties": {} }));
    if !parameters.is_object() {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "UTS tool input_schema must be a JSON object",
        ));
    }
    Ok(serde_json::json!({
        "name": name,
        "description": description,
        "parameters": parameters,
    }))
}

fn required_cfg_string(
    cfg: &HashMap<String, Value>,
    key: &str,
    provider_label: &str,
) -> Result<String> {
    cfg_string(cfg, key).ok_or_else(|| {
        invalid_config(
            provider_label,
            format!("config.{key} is required for Vertex AI Gemini"),
        )
    })
}

fn validate_vertex_ai_endpoint(spec: &adl::ProviderSpec, endpoint: &str) -> Result<()> {
    if !is_allowed_remote_endpoint(endpoint) {
        return Err(invalid_config(
            "vertex_ai_gemini",
            "endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
        ));
    }
    let trusted_vertex_endpoint = match (
        endpoint_host(endpoint),
        required_cfg_string(&spec.config, "location", "vertex_ai_gemini"),
    ) {
        (Some(host), Ok(location)) if location == "global" => host == "aiplatform.googleapis.com",
        (Some(host), Ok(location)) => host == format!("{location}-aiplatform.googleapis.com"),
        _ => false,
    };
    if is_loopback_endpoint(endpoint)
        || trusted_vertex_endpoint
        || cfg_bool_opt(&spec.config, "trust_custom_endpoint", "vertex_ai_gemini")?.unwrap_or(false)
    {
        return Ok(());
    }
    Err(invalid_config(
        "vertex_ai_gemini",
        "refusing to send Vertex AI bearer credentials to an untrusted endpoint; use a regional aiplatform.googleapis.com endpoint, loopback, or config.trust_custom_endpoint: true",
    ))
}

fn cfg_string(cfg: &HashMap<String, Value>, key: &str) -> Option<String> {
    cfg.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
}

fn bedrock_nova_request_body(prompt: &str, max_tokens: u64) -> Value {
    serde_json::json!({
        "schemaVersion": "messages-v1",
        "messages": [{
            "role": "user",
            "content": [{"text": prompt}],
        }],
        "inferenceConfig": {
            "maxTokens": max_tokens,
        },
    })
}

fn bedrock_sdk_error(message: String) -> anyhow::Error {
    let sanitized = sanitize_bedrock_error(&message);
    let retryable = sanitized.contains("Throttling")
        || sanitized.contains("TooManyRequests")
        || sanitized.contains("timeout")
        || sanitized.contains("Timeout")
        || sanitized.contains("ServiceUnavailable")
        || sanitized.contains("InternalServer");
    if retryable {
        runtime_error("bedrock", sanitized)
    } else {
        runtime_error_non_retryable("bedrock", sanitized)
    }
}

fn sanitize_bedrock_error(message: &str) -> String {
    let mut out = message.replace('\n', " ");
    for marker in [
        "Authorization: ",
        "Authorization=",
        "Credential=",
        "X-Amz-Signature=",
        "SecretAccessKey=",
    ] {
        out = redact_aws_error_value(&out, marker);
    }
    out = redact_aws_arns(&out);
    out = redact_aws_account_ids(&out);
    truncate_provider_body(&out)
}

fn redact_aws_arns(input: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find("arn:aws") {
        let arn_start = cursor + relative_start;
        let Some(next) = input[arn_start + "arn:aws".len()..].chars().next() else {
            redacted.push_str(&input[cursor..]);
            return redacted;
        };
        if next != ':' && next != '-' {
            let prefix_end = arn_start + "arn:aws".len();
            redacted.push_str(&input[cursor..prefix_end]);
            cursor = prefix_end;
            continue;
        }
        redacted.push_str(&input[cursor..arn_start]);
        redacted.push_str("<redacted-aws-arn>");

        let arn_end = input[arn_start..]
            .char_indices()
            .find_map(|(idx, ch)| {
                matches!(ch, ' ' | ',' | ';' | '"' | '\'' | ')' | '}' | ']')
                    .then_some(arn_start + idx)
            })
            .unwrap_or(input.len());
        cursor = arn_end;
    }
    redacted.push_str(&input[cursor..]);
    redacted
}

fn redact_aws_account_ids(input: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut digit_start = None;
    let mut digit_count = 0usize;
    let mut last_end = 0usize;

    for (idx, ch) in input.char_indices() {
        if ch.is_ascii_digit() {
            if digit_start.is_none() {
                digit_start = Some(idx);
            }
            digit_count += 1;
            continue;
        }

        if let Some(start) = digit_start {
            if digit_count == 12 {
                redacted.push_str(&input[last_end..start]);
                redacted.push_str("<redacted-aws-account-id>");
                last_end = idx;
            }
        }
        digit_start = None;
        digit_count = 0;
    }

    if let Some(start) = digit_start {
        if digit_count == 12 {
            redacted.push_str(&input[last_end..start]);
            redacted.push_str("<redacted-aws-account-id>");
            last_end = input.len();
        }
    }

    redacted.push_str(&input[last_end..]);
    redacted
}

fn redact_aws_error_value(input: &str, marker: &str) -> String {
    let mut redacted = String::with_capacity(input.len());
    let mut cursor = 0;
    while let Some(relative_start) = input[cursor..].find(marker) {
        let marker_start = cursor + relative_start;
        let value_start = marker_start + marker.len();
        redacted.push_str(&input[cursor..value_start]);
        redacted.push_str("<redacted>");

        let value_end = input[value_start..]
            .char_indices()
            .find_map(|(idx, ch)| {
                matches!(ch, ' ' | ',' | '&' | ';' | '"' | '\'' | ')' | '}' | ']')
                    .then_some(value_start + idx)
            })
            .unwrap_or(input.len());
        cursor = value_end;
    }
    redacted.push_str(&input[cursor..]);
    redacted
}

fn sha256_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone)]
/// Z.ai native provider using the OpenAI-compatible chat completions API format.
pub struct ZAiProvider {
    endpoint: String,
    auth_env: String,
    model: String,
    max_tokens: u64,
    reasoning_effort: Option<String>,
    clear_thinking: Option<bool>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    timeout_secs: Option<u64>,
}

impl ZAiProvider {
    /// Build a Z.ai provider from normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let default_endpoint = if target.provider_model_id == "glm-5.3-flash" {
            Z_AI_GLM_5_3_FLASH_CHAT_COMPLETIONS_ENDPOINT
        } else {
            Z_AI_LEGACY_CHAT_COMPLETIONS_ENDPOINT
        };
        let endpoint = vendor_endpoint(spec, target, default_endpoint, "z_ai")?;
        let auth_env = auth_env_for(spec, "ZAI_API_KEY")?;
        validate_vendor_credential_endpoint(
            spec,
            "z_ai",
            &endpoint,
            &auth_env,
            "ZAI_API_KEY",
            &["open.bigmodel.cn", "api.z.ai"],
        )?;
        let max_tokens = match cfg_u64_strict(&spec.config, "max_tokens", "z_ai")? {
            Some(value) => value,
            None => cfg_u64_strict(&spec.config, "max_output_tokens", "z_ai")?.unwrap_or(220),
        };
        if target.provider_model_id == "glm-5.3-flash" && max_tokens > 131_072 {
            return Err(invalid_config(
                "z_ai",
                "config.max_tokens/max_output_tokens must be no greater than 131072 for glm-5.3-flash",
            ));
        }
        let reasoning_effort = match spec.config.get("reasoning_effort") {
            Some(Value::String(value)) => {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    return Err(invalid_config(
                        "z_ai",
                        "config.reasoning_effort must not be empty when provided",
                    ));
                }
                Some(trimmed.to_string())
            }
            Some(_) => {
                return Err(invalid_config(
                    "z_ai",
                    "config.reasoning_effort must be a string when provided",
                ));
            }
            None => None,
        };
        if target.provider_model_id == "glm-5.3-flash" {
            if let Some(value) = reasoning_effort.as_deref() {
                if !matches!(value, "low" | "high" | "max") {
                    return Err(invalid_config(
                        "z_ai",
                        "config.reasoning_effort must be one of low, high, max for glm-5.3-flash",
                    ));
                }
            }
        }
        let clear_thinking = cfg_bool_opt(&spec.config, "clear_thinking", "z_ai")?;
        let temperature = cfg_f64_strict(&spec.config, "temperature", "z_ai")?;
        if let Some(value) = temperature {
            let max = if target.provider_model_id == "glm-5.3-flash" {
                1.0
            } else {
                2.0
            };
            if value < 0.0 || value > max {
                return Err(invalid_config(
                    "z_ai",
                    format!("config.temperature must be in [0, {max}]"),
                ));
            }
        }
        let top_p = cfg_f64_strict(&spec.config, "top_p", "z_ai")?;
        if let Some(value) = top_p {
            let min = if target.provider_model_id == "glm-5.3-flash" {
                0.01
            } else {
                0.0
            };
            if value < min || value > 1.0 {
                return Err(invalid_config(
                    "z_ai",
                    format!("config.top_p must be in [{min}, 1]"),
                ));
            }
        }
        Ok(Self {
            endpoint,
            auth_env,
            model: target.provider_model_id.clone(),
            max_tokens,
            reasoning_effort,
            clear_thinking,
            temperature,
            top_p,
            timeout_secs: cfg_u64(&spec.config, "timeout_secs"),
        })
    }
}

impl Provider for ZAiProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let token = env::var(&self.auth_env).map_err(|_| {
            invalid_config(
                "z_ai",
                format!("missing required auth env var '{}'", self.auth_env),
            )
        })?;
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build Z.ai client")
            .map_err(|err| runtime_error("z_ai", err.to_string()))?;
        let mut body = serde_json::json!({
            "model": self.model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": self.max_tokens,
            "stream": false,
        });
        if let Some(reasoning_effort) = &self.reasoning_effort {
            body["reasoning_effort"] = serde_json::json!(reasoning_effort);
        }
        if let Some(clear_thinking) = self.clear_thinking {
            body["thinking"] =
                serde_json::json!({ "type": "enabled", "clear_thinking": clear_thinking });
        }
        if let Some(temperature) = self.temperature {
            body["temperature"] = serde_json::json!(temperature);
        }
        if let Some(top_p) = self.top_p {
            body["top_p"] = serde_json::json!(top_p);
        }
        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .json(&body);
        let (json, http_status) = provider_http_json("z_ai", req)?;
        let output = extract_deepseek_output_text(&json).ok_or_else(|| {
            runtime_error_non_retryable("z_ai", "response missing message content")
        })?;
        write_native_invocation_record("z_ai", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

#[derive(Debug, Clone)]
/// Generic HTTP provider for configurable endpoint + optional bearer auth.
pub struct HttpProvider {
    endpoint: String,
    auth: Option<HttpAuth>,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
    vendor: String,
    model: String,
    chat_mode: bool,
}

#[derive(Debug, Clone)]
/// Ollama-specific HTTP provider with prompt/model serialization.
pub struct OllamaHttpProvider {
    endpoint: String,
    model: String,
    temperature: Option<f32>,
    timeout_secs: Option<u64>,
}

impl OllamaHttpProvider {
    /// Build an Ollama HTTP provider from the normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let timeout_secs = match cfg_u64_strict(&spec.config, "timeout_secs", "ollama")? {
            Some(value) => value,
            None => timeout_secs().map_err(|err| invalid_config("ollama", err.to_string()))?,
        };
        Ok(Self {
            endpoint: ollama_generate_endpoint(spec)?,
            model: target.provider_model_id.clone(),
            temperature: super::local::cfg_f32(&spec.config, "temperature"),
            timeout_secs: Some(timeout_secs),
        })
    }
}

impl Provider for OllamaHttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build ollama http client")
            .map_err(|err| runtime_error("ollama", err.to_string()))?;

        let mut body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
        });
        if let Some(temperature) = self.temperature {
            body["options"] = serde_json::json!({ "temperature": temperature });
        }

        let req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&body);
        let (json, http_status) = provider_http_json("ollama", req)?;
        let output = json
            .get("response")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .ok_or_else(|| {
                runtime_error_non_retryable("ollama", "response missing 'response' text field")
            })?
            .to_string();
        write_native_invocation_record("ollama", &self.model, prompt, &output, http_status)?;
        Ok(output)
    }
}

impl HttpProvider {
    /// Build an HTTP provider from an already-normalized invocation spec.
    pub fn from_spec(spec: &adl::ProviderSpec) -> Result<Self> {
        let target = provider_substrate::provider_invocation_target_v1(
            spec.id.as_deref().unwrap_or("<anonymous-provider>"),
            spec,
            None,
        )?;
        Self::from_target(spec, &target)
    }

    /// Build a generic HTTP provider from the normalized invocation target.
    pub fn from_target(
        spec: &adl::ProviderSpec,
        target: &ProviderInvocationTargetV1,
    ) -> Result<Self> {
        let cfg = &spec.config;
        let mut endpoint = target
            .endpoint
            .clone()
            .or_else(|| target.base_url.clone())
            .ok_or_else(|| {
                invalid_config(
                    "http",
                    "config.endpoint is required (set providers.<id>.config.endpoint)",
                )
            })?;
        if target.vendor == "google" && endpoint.ends_with("/models") {
            endpoint = format!(
                "{}/{}:generateContent",
                endpoint.trim_end_matches('/'),
                target.provider_model_id
            );
        }
        // Existing user-configured HTTP endpoints retain the legacy `{prompt}`
        // contract. Only profiles with a declared chat-completions contract
        // opt into vendor/model-aware payloads.
        let chat_mode = target
            .profile
            .as_deref()
            .and_then(|profile| profile.split(':').next())
            .is_some_and(|family| {
                matches!(
                    family,
                    "kimi"
                        | "minimax"
                        | "qwen"
                        | "xai"
                        | "mistral"
                        | "cohere"
                        | "deepseek"
                        | "gemini"
                )
            });
        if !is_allowed_remote_endpoint(&endpoint) {
            return Err(invalid_config(
                "http",
                "config.endpoint must use https://; plaintext http:// is only allowed for localhost/loopback test endpoints",
            ));
        }

        let timeout_secs = cfg_u64(cfg, "timeout_secs");

        let mut headers = HashMap::new();
        if let Some(h) = cfg.get("headers") {
            let obj = h.as_object().ok_or_else(|| {
                invalid_config("http", "config.headers must be an object of string values")
            })?;
            for (k, v) in obj {
                let v = v.as_str().ok_or_else(|| {
                    invalid_config("http", "config.headers values must be strings")
                })?;
                headers.insert(k.clone(), v.to_string());
            }
        }

        let auth = if let Some(auth_val) = cfg.get("auth") {
            let obj = auth_val
                .as_object()
                .ok_or_else(|| invalid_config("http", "config.auth must be an object"))?;
            let auth_type = obj
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.type is required"))?;
            if auth_type != "bearer" {
                return Err(invalid_config(
                    "http",
                    format!("config.auth.type must be 'bearer' (got '{auth_type}')"),
                ));
            }
            let env_key = obj
                .get("env")
                .and_then(|v| v.as_str())
                .ok_or_else(|| invalid_config("http", "config.auth.env is required"))?;
            Some(HttpAuth {
                env: env_key.to_string(),
            })
        } else {
            None
        };
        if auth.is_some() {
            validate_http_credential_endpoint(cfg, &endpoint)?;
        }

        Ok(Self {
            endpoint,
            auth,
            headers,
            timeout_secs,
            vendor: target.vendor.clone(),
            model: target.provider_model_id.clone(),
            chat_mode,
        })
    }
}

impl Provider for HttpProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(secs) = self.timeout_secs {
            client_builder = client_builder.timeout(Duration::from_secs(secs));
        }
        let client = client_builder
            .build()
            .context("failed to build http client")
            .map_err(|err| runtime_error("http", err.to_string()))?;

        let mut req = client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        for (k, v) in self.headers.iter() {
            req = req.header(k, v);
        }

        if let Some(auth) = &self.auth {
            let token = env::var(&auth.env).map_err(|_| {
                invalid_config(
                    "http",
                    format!(
                        "missing required auth env var '{}' (set it or remove config.auth)",
                        auth.env
                    ),
                )
            })?;
            req = req.bearer_auth(token);
        }

        let body = if !self.chat_mode {
            serde_json::json!({ "prompt": prompt })
        } else if self.vendor == "google" {
            serde_json::json!({
                "contents": [{"role": "user", "parts": [{"text": prompt}]}]
            })
        } else {
            serde_json::json!({
                "model": self.model,
                "messages": [{"role": "user", "content": prompt}]
            })
        };

        let resp = match req.json(&body).send() {
            Ok(resp) => resp,
            Err(err) => {
                if err.is_timeout() {
                    let msg = match self.timeout_secs {
                        Some(secs) => format!(
                            "kind=timeout timed out after {secs}s (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                        ),
                        None => {
                            "kind=timeout timed out (set providers.<id>.config.timeout_secs or ADL_TIMEOUT_SECS to override)"
                                .to_string()
                        }
                    };
                    return Err(timeout_error("http", msg));
                }

                return Err(runtime_error(
                    "http",
                    format!("kind=request_failed http provider request failed: {err}"),
                ));
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().unwrap_or_default();
            let class = if status.is_client_error() {
                "client_error"
            } else if status.is_server_error() {
                "server_error"
            } else {
                "http_error"
            };
            let msg = format!(
                "kind={class} status={status} body={}",
                truncate_provider_body(&text)
            );
            if status.is_client_error() {
                return Err(runtime_error_non_retryable("http", msg));
            }
            return Err(runtime_error("http", msg));
        }

        let json: serde_json::Value = resp
            .json()
            .context("http provider response was not valid JSON")
            .map_err(|err| runtime_error_non_retryable("http", err.to_string()))?;
        let out = json
            .get("output")
            .and_then(|v| v.as_str())
            .or_else(|| {
                json.pointer("/choices/0/message/content")
                    .and_then(|v| v.as_str())
            })
            .or_else(|| {
                json.pointer("/message/content/0/text")
                    .and_then(|v| v.as_str())
            })
            .or_else(|| {
                json.pointer("/candidates/0/content/parts/0/text")
                    .and_then(|v| v.as_str())
            })
            .ok_or_else(|| {
                if self.chat_mode {
                    runtime_error_non_retryable("http", "response missing supported text output")
                } else {
                    runtime_error_non_retryable("http", "response missing 'output' field")
                }
            })?;

        Ok(out.to_string())
    }
}

#[cfg(test)]
mod tests;
