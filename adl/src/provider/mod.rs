//! Provider selection and execution entrypoints for ADL.
//!
//! This module selects among mock/HTTP/CLI provider implementations and exposes
//! the minimal abstraction layer used by scheduler and remote-exec paths.
use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::adl;
use crate::provider_substrate::{self, ProviderInvocationTargetV1};

mod deepgram;
mod http_family;
mod local;
mod profiles;

pub use deepgram::{
    build_speech_provider, AudioContainer, AudioEncoding, DeepgramSpeechProvider, SpeechErrorKind,
    SpeechProvenance, SpeechProvider, SpeechProviderError, SynthesisRequest, SynthesisResult,
    TranscriptWord, TranscriptionRequest, TranscriptionResult,
};
pub use http_family::{
    AnthropicProvider, AwsBedrockProvider, HttpProvider, OllamaHttpProvider, OpenAiProvider,
    VertexAiGeminiProvider, ZAiProvider,
};
pub use http_family::{DeepSeekProvider, OpenRouterProvider};
pub use local::{MockProvider, OllamaProvider};
pub use profiles::{
    activate_provider_profile_candidate, expand_provider_profiles,
    provider_profile_materialization_projection, provider_profile_names,
    redacted_provider_profile_projection, ProviderProfileActivation,
};

pub(crate) use profiles::{
    is_allowed_ollama_endpoint, is_allowed_remote_endpoint, ANTHROPIC_MESSAGES_ENDPOINT,
    ANTHROPIC_VERSION, DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT, OPENAI_RESPONSES_ENDPOINT,
    OPENROUTER_CHAT_COMPLETIONS_ENDPOINT, Z_AI_GLM_5_3_FLASH_CHAT_COMPLETIONS_ENDPOINT,
    Z_AI_LEGACY_CHAT_COMPLETIONS_ENDPOINT,
};

/// A minimal blocking provider abstraction used by runtime execution paths.
pub trait Provider: Send + Sync {
    /// Run a single completion call and return output text.
    fn complete(&self, prompt: &str) -> Result<String>;

    /// Optional streaming callback form; default implementation buffers internally
    /// and emits progress via callback before returning the full output.
    fn complete_stream(&self, prompt: &str, on_chunk: &mut dyn FnMut(&str)) -> Result<String> {
        let out = self.complete(prompt)?;
        on_chunk(&out);
        Ok(out)
    }
}

/// Execution channel for provider results that must not be conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderExecutionChannel {
    Authoritative,
    Shadow,
}

/// Exact local-model shadow input shared by authority and shadow paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderShadowInput {
    prompt: String,
    comparison_rule_set: String,
}

impl ProviderShadowInput {
    /// Build a shadow input from one exact prompt and a deterministic rule-set id.
    pub fn new(prompt: impl Into<String>, comparison_rule_set: impl Into<String>) -> Result<Self> {
        let comparison_rule_set = comparison_rule_set.into();
        if comparison_rule_set.trim().is_empty() {
            return Err(anyhow!(
                "provider shadow comparison rule set must be non-empty"
            ));
        }
        Ok(Self {
            prompt: prompt.into(),
            comparison_rule_set,
        })
    }

    /// The exact prompt used for both authoritative and shadow observation paths.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Stable rule-set id recorded in redacted comparison evidence.
    pub fn comparison_rule_set(&self) -> &str {
        &self.comparison_rule_set
    }
}

/// Authoritative provider output. This is the only output channel callers may accept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritativeProviderCompletion {
    channel: ProviderExecutionChannel,
    pub output: String,
}

impl AuthoritativeProviderCompletion {
    /// Constructor for the authority-only completion channel.
    fn new(output: String) -> Self {
        Self {
            channel: ProviderExecutionChannel::Authoritative,
            output,
        }
    }

    /// Read-only execution channel marker.
    pub fn channel(&self) -> ProviderExecutionChannel {
        self.channel
    }
}

/// Redacted shadow observation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderShadowObservationClass {
    Completed,
    Failed,
    NotConfigured,
}

/// Non-authoritative shadow observation. Raw shadow output is intentionally omitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderShadowObservation {
    channel: ProviderExecutionChannel,
    pub observation_class: ProviderShadowObservationClass,
    pub output_digest: Option<String>,
    pub failure_kind: Option<String>,
}

impl ProviderShadowObservation {
    fn completed(output: &str) -> Self {
        Self {
            channel: ProviderExecutionChannel::Shadow,
            observation_class: ProviderShadowObservationClass::Completed,
            output_digest: Some(sha256_text(output)),
            failure_kind: None,
        }
    }

    fn failed(err: &anyhow::Error) -> Self {
        Self {
            channel: ProviderExecutionChannel::Shadow,
            observation_class: ProviderShadowObservationClass::Failed,
            output_digest: None,
            failure_kind: Some(
                stable_failure_kind(err)
                    .unwrap_or("provider_error")
                    .to_string(),
            ),
        }
    }

    fn not_configured() -> Self {
        Self {
            channel: ProviderExecutionChannel::Shadow,
            observation_class: ProviderShadowObservationClass::NotConfigured,
            output_digest: None,
            failure_kind: None,
        }
    }

    /// Read-only execution channel marker.
    pub fn channel(&self) -> ProviderExecutionChannel {
        self.channel
    }
}

/// Redaction facts for the comparison record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderShadowRedaction {
    pub policy: String,
    pub prompt_redacted: bool,
    pub output_redacted: bool,
    pub credential_material_redacted: bool,
    pub host_paths_redacted: bool,
}

impl Default for ProviderShadowRedaction {
    fn default() -> Self {
        Self {
            policy: "provider_shadow_redaction_v1".to_string(),
            prompt_redacted: true,
            output_redacted: true,
            credential_material_redacted: true,
            host_paths_redacted: true,
        }
    }
}

/// Redacted comparison evidence. It records digests/classes, never prompts or output payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderShadowComparisonRecord {
    pub schema: String,
    pub authority_input_digest: String,
    pub shadow_input_digest: String,
    pub comparison_rule_set: String,
    pub authority_channel: ProviderExecutionChannel,
    pub shadow_channel: ProviderExecutionChannel,
    pub authority_outcome_class: String,
    pub shadow_observation_class: ProviderShadowObservationClass,
    pub redaction: ProviderShadowRedaction,
}

/// Result for one authority execution plus optional non-authoritative local-model shadow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderShadowExecution {
    pub authoritative: AuthoritativeProviderCompletion,
    pub shadow: ProviderShadowObservation,
    pub comparison: ProviderShadowComparisonRecord,
}

impl ProviderShadowExecution {
    /// Redacted, reviewable evidence projection for PROV-B receipts.
    pub fn redacted_evidence(&self) -> Result<Value> {
        serde_json::to_value(&self.comparison).context("serialize provider shadow evidence")
    }
}

/// Run the authoritative provider first, then observe an optional local shadow provider.
///
/// Shadow output never replaces the authoritative output, and shadow failures are
/// captured only as redacted observation metadata.
pub fn complete_with_local_model_shadow(
    authoritative_provider: &dyn Provider,
    shadow_provider: Option<&dyn Provider>,
    input: ProviderShadowInput,
) -> Result<ProviderShadowExecution> {
    let authority_output = authoritative_provider.complete(input.prompt())?;
    let authority_input_digest = sha256_text(input.prompt());

    let shadow = match shadow_provider {
        Some(provider) => observe_shadow_provider(provider, input.prompt()),
        None => ProviderShadowObservation::not_configured(),
    };

    let comparison = ProviderShadowComparisonRecord {
        schema: "adl.provider.local_model_shadow_comparison.v1".to_string(),
        authority_input_digest: authority_input_digest.clone(),
        shadow_input_digest: authority_input_digest,
        comparison_rule_set: input.comparison_rule_set().to_string(),
        authority_channel: ProviderExecutionChannel::Authoritative,
        shadow_channel: ProviderExecutionChannel::Shadow,
        authority_outcome_class: "completed".to_string(),
        shadow_observation_class: shadow.observation_class,
        redaction: ProviderShadowRedaction::default(),
    };

    Ok(ProviderShadowExecution {
        authoritative: AuthoritativeProviderCompletion::new(authority_output),
        shadow,
        comparison,
    })
}

fn sha256_text(value: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(value.as_bytes()))
}

fn observe_shadow_provider(provider: &dyn Provider, prompt: &str) -> ProviderShadowObservation {
    thread_local! {
        static SUPPRESS_SHADOW_PANIC_HOOK: Cell<bool> = const { Cell::new(false) };
    }
    static SHADOW_PANIC_HOOK_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    let _hook_guard = SHADOW_PANIC_HOOK_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous_hook = std::panic::take_hook();
    let previous_hook = Arc::new(Mutex::new(Some(previous_hook)));
    let previous_hook_for_delegate = Arc::clone(&previous_hook);
    std::panic::set_hook(Box::new(move |info| {
        if SUPPRESS_SHADOW_PANIC_HOOK.with(Cell::get) {
            return;
        }
        let guard = previous_hook_for_delegate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(delegate) = guard.as_ref() {
            delegate(info);
        }
    }));
    SUPPRESS_SHADOW_PANIC_HOOK.with(|suppressed| suppressed.set(true));
    let result = catch_unwind(AssertUnwindSafe(|| provider.complete(prompt)));
    SUPPRESS_SHADOW_PANIC_HOOK.with(|suppressed| suppressed.set(false));
    let _shadow_hook = std::panic::take_hook();
    if let Some(previous_hook) = previous_hook
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take()
    {
        std::panic::set_hook(previous_hook);
    }

    match result {
        Ok(Ok(output)) => ProviderShadowObservation::completed(&output),
        Ok(Err(err)) => ProviderShadowObservation::failed(&err),
        Err(_) => ProviderShadowObservation::failed(&panic_error(
            "local-model-shadow",
            "shadow provider panicked",
        )),
    }
}

#[derive(Debug, Clone, Copy)]
enum ProviderErrorKind {
    UnknownKind,
    InvalidConfig,
    Timeout,
    Panic,
    RuntimeRetryable,
    RuntimeNonRetryable,
}

#[derive(Debug)]
struct ProviderError {
    kind: ProviderErrorKind,
    provider: Option<String>,
    message: String,
}

impl ProviderError {
    fn unknown_kind(kind: &str) -> Self {
        Self {
            kind: ProviderErrorKind::UnknownKind,
            provider: None,
            message: format!(
            "provider kind '{kind}' is not supported (supported: ollama, local_ollama, mock, http, http_remote, openai, anthropic, deepseek, openrouter, bedrock, aws_bedrock, z_ai). \
Set providers.<id>.type to one of: ollama, local_ollama, mock, http, http_remote, openai, anthropic, deepseek, openrouter, bedrock, aws_bedrock, z_ai, vertex_ai_gemini. The remote provider surfaces are HTTPS-only."
            ),
        }
    }

    fn invalid_config(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::InvalidConfig,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn runtime_non_retryable(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::RuntimeNonRetryable,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn timeout(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Timeout,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }

    fn panic(provider: &str, message: impl Into<String>) -> Self {
        Self {
            kind: ProviderErrorKind::Panic,
            provider: Some(provider.to_string()),
            message: message.into(),
        }
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ProviderErrorKind::UnknownKind => write!(f, "{}", self.message),
            ProviderErrorKind::InvalidConfig => write!(
                f,
                "provider {} invalid config: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Timeout => write!(
                f,
                "provider {} timeout: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::Panic => write!(
                f,
                "provider {} panic: {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeRetryable => write!(
                f,
                "provider {} runtime error (retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
            ProviderErrorKind::RuntimeNonRetryable => write!(
                f,
                "provider {} runtime error (non-retryable): {}",
                self.provider.as_deref().unwrap_or("<unknown>"),
                self.message
            ),
        }
    }
}

impl StdError for ProviderError {}

/// Validate and normalize an unsupported provider kind into a typed error.
fn unknown_kind(kind: &str) -> anyhow::Error {
    ProviderError::unknown_kind(kind).into()
}

fn invalid_config(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::invalid_config(provider, message).into()
}

fn runtime_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime(provider, message).into()
}

fn runtime_error_non_retryable(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::runtime_non_retryable(provider, message).into()
}

fn timeout_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::timeout(provider, message).into()
}

fn panic_error(provider: &str, message: impl Into<String>) -> anyhow::Error {
    ProviderError::panic(provider, message).into()
}

pub fn is_retryable_error(err: &anyhow::Error) -> bool {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return matches!(
                p.kind,
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::Timeout
            );
        }
    }
    if let Some(retryable) = crate::remote_exec::retryability(err) {
        return retryable;
    }
    true
}

pub fn stable_failure_kind(err: &anyhow::Error) -> Option<&'static str> {
    for cause in err.chain() {
        if let Some(p) = cause.downcast_ref::<ProviderError>() {
            return Some(match p.kind {
                ProviderErrorKind::Timeout => "timeout",
                ProviderErrorKind::Panic => "panic",
                ProviderErrorKind::UnknownKind | ProviderErrorKind::InvalidConfig => "schema_error",
                ProviderErrorKind::RuntimeRetryable | ProviderErrorKind::RuntimeNonRetryable => {
                    "provider_error"
                }
            });
        }
    }
    None
}

/// Construct a provider implementation from a `ProviderSpec`.
///
/// Expected schema (based on your compiler errors):
/// ProviderSpec { kind, base_url, config }
pub fn build_provider(
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    build_provider_for_id(
        spec.id.as_deref().unwrap_or("<anonymous-provider>"),
        spec,
        model_override,
    )
}

/// Build a provider with explicit identity and optional model override.
pub fn build_provider_for_id(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<Box<dyn Provider>> {
    match spec.kind.trim() {
        "http" | "http_remote" | "ollama" | "local_ollama" | "mock" | "openai" | "anthropic"
        | "deepseek" | "openrouter" | "bedrock" | "aws_bedrock" | "z_ai" | "zai" | "zhipu"
        | "vertex_ai_gemini" | "vertex_ai" | "vertex" => {}
        other => return Err(unknown_kind(other)),
    }

    let target =
        provider_substrate::provider_invocation_target_v1(provider_id, spec, model_override)
            .with_context(|| format!("normalize provider substrate for '{provider_id}'"))?;
    match target.transport {
        provider_substrate::ProviderTransportV1::Http => match target.provider_kind.as_str() {
            "http" | "http_remote" => Ok(Box::new(HttpProvider::from_target(spec, &target)?)),
            "ollama" => Ok(Box::new(OllamaHttpProvider::from_target(spec, &target)?)),
            "openai" => Ok(Box::new(OpenAiProvider::from_target(spec, &target)?)),
            "anthropic" => Ok(Box::new(AnthropicProvider::from_target(spec, &target)?)),
            "deepseek" => Ok(Box::new(DeepSeekProvider::from_target(spec, &target)?)),
            "openrouter" => Ok(Box::new(OpenRouterProvider::from_target(spec, &target)?)),
            "bedrock" | "aws_bedrock" => {
                Ok(Box::new(AwsBedrockProvider::from_target(spec, &target)?))
            }
            "z_ai" | "zai" | "zhipu" => Ok(Box::new(ZAiProvider::from_target(spec, &target)?)),
            "vertex_ai_gemini" | "vertex_ai" | "vertex" => Ok(Box::new(
                VertexAiGeminiProvider::from_target(spec, &target)?,
            )),
            other => Err(unknown_kind(other)),
        },
        provider_substrate::ProviderTransportV1::LocalCli
        | provider_substrate::ProviderTransportV1::InProcess => match target.provider_kind.as_str()
        {
            "ollama" | "local_ollama" => Ok(Box::new(OllamaProvider::from_target(spec, &target)?)),
            "mock" => Ok(Box::new(MockProvider::from_target(&target))),
            other => Err(unknown_kind(other)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::http_family::{cfg_u64, timeout_secs};
    use super::local::cfg_f32;
    use super::profiles::{provider_profile_registry, validate_profile_endpoint};
    use super::*;
    use std::env;

    fn provider_spec(kind: &str, default_model: Option<&str>) -> adl::ProviderSpec {
        adl::ProviderSpec {
            id: Some(format!("{kind}_primary")),
            profile: None,
            kind: kind.to_string(),
            base_url: None,
            default_model: default_model.map(ToString::to_string),
            config: HashMap::new(),
        }
    }

    #[test]
    fn provider_mod_error_helpers_and_classification_are_stable() {
        let retryable = runtime_error("mock", "retryable");
        assert!(is_retryable_error(&retryable));
        assert_eq!(stable_failure_kind(&retryable), Some("provider_error"));

        let non_retryable = runtime_error_non_retryable("mock", "non-retryable");
        assert!(!is_retryable_error(&non_retryable));
        assert_eq!(stable_failure_kind(&non_retryable), Some("provider_error"));

        let timeout = timeout_error("mock", "timeout");
        assert!(is_retryable_error(&timeout));
        assert_eq!(stable_failure_kind(&timeout), Some("timeout"));

        let panic = panic_error("mock", "panic");
        assert!(!is_retryable_error(&panic));
        assert_eq!(stable_failure_kind(&panic), Some("panic"));
        assert!(format!("{panic:#}").contains("provider mock panic: panic"));

        let config = invalid_config("mock", "bad config");
        assert!(!is_retryable_error(&config));
        assert_eq!(stable_failure_kind(&config), Some("schema_error"));
        assert!(format!("{config:#}").contains("provider mock invalid config: bad config"));

        let unknown = unknown_kind("mystery");
        assert!(!is_retryable_error(&unknown));
        assert_eq!(stable_failure_kind(&unknown), Some("schema_error"));
        assert!(format!("{unknown:#}").contains("provider kind 'mystery' is not supported"));
    }

    #[test]
    fn provider_mod_complete_stream_default_buffers_mock_output() {
        let spec = provider_spec("mock", Some("mock-model"));
        let provider = build_provider_for_id("mock_primary", &spec, None).expect("mock provider");
        let mut chunks = Vec::new();
        let output = provider
            .complete_stream("hello mock", &mut |chunk| chunks.push(chunk.to_string()))
            .expect("mock stream completion");

        assert_eq!(output, "hello mock");
        assert_eq!(chunks, vec!["hello mock".to_string()]);
    }

    #[test]
    fn provider_mod_build_provider_dispatches_supported_native_and_compatibility_kinds() {
        let mock = provider_spec("mock", Some("mock-model"));
        build_provider_for_id("mock_primary", &mock, None).expect("mock provider");

        let local_ollama = provider_spec("ollama", Some("phi4-mini"));
        build_provider_for_id("ollama_primary", &local_ollama, Some("phi4-mini"))
            .expect("local ollama provider");

        let mut http_ollama = provider_spec("ollama", Some("phi4-mini"));
        http_ollama.base_url = Some("http://127.0.0.1:11434".to_string());
        build_provider_for_id("ollama_http_primary", &http_ollama, None)
            .expect("ollama http provider");

        let mut generic_http = provider_spec("http", Some("http-model"));
        generic_http.config.insert(
            "endpoint".to_string(),
            serde_json::json!("https://api.example.com/v1/complete"),
        );
        build_provider_for_id("http_primary", &generic_http, None).expect("generic http provider");

        let openai = provider_spec("openai", Some("gpt-test"));
        build_provider_for_id("openai_primary", &openai, None).expect("openai provider");

        let anthropic = provider_spec("anthropic", Some("claude-test"));
        build_provider_for_id("anthropic_primary", &anthropic, None).expect("anthropic provider");

        let deepseek = provider_spec("deepseek", Some("deepseek-chat"));
        build_provider_for_id("deepseek_primary", &deepseek, None).expect("deepseek provider");

        let openrouter = provider_spec("openrouter", Some("openai/gpt-4o-mini"));
        build_provider_for_id("openrouter_primary", &openrouter, None)
            .expect("openrouter provider");

        let mut vertex = provider_spec("vertex_ai_gemini", Some("gemini-2.5-flash"));
        vertex
            .config
            .insert("project".to_string(), serde_json::json!("company-project"));
        vertex
            .config
            .insert("location".to_string(), serde_json::json!("us-west1"));
        build_provider_for_id("vertex_primary", &vertex, None).expect("vertex provider");
    }

    #[test]
    fn provider_mod_build_provider_rejects_unknown_kind_and_invalid_native_endpoint() {
        let unknown = provider_spec("not-a-provider", Some("model"));
        let unknown_err = match build_provider_for_id("unknown_primary", &unknown, None) {
            Ok(_) => panic!("unknown kind should fail"),
            Err(err) => err,
        };
        assert!(unknown_err
            .to_string()
            .contains("provider kind 'not-a-provider' is not supported"));

        let mut unsafe_openai = provider_spec("openai", Some("gpt-test"));
        unsafe_openai.config.insert(
            "endpoint".to_string(),
            serde_json::json!("http://api.openai.com/v1/responses"),
        );
        let endpoint_err = match build_provider_for_id("openai_primary", &unsafe_openai, None) {
            Ok(_) => panic!("plain http hosted endpoint should fail"),
            Err(err) => err,
        };
        assert!(endpoint_err
            .to_string()
            .contains("endpoint must use https://"));
    }

    #[test]
    fn provider_mod_remote_retry_classification_distinguishes_deterministic_failures() {
        let schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::SchemaViolation,
            "REMOTE_SCHEMA_VIOLATION",
            "missing result on ok response",
        ));
        assert!(!is_retryable_error(&schema));

        let envelope = anyhow::Error::new(crate::remote_exec::SecurityEnvelopeError::MissingKeyId);
        assert!(!is_retryable_error(&envelope));

        let remote_schema = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::RemoteExecution,
            "REMOTE_SCHEMA_VIOLATION",
            "invalid provider config",
        ));
        assert!(!is_retryable_error(&remote_schema));

        let timeout = anyhow::Error::new(crate::remote_exec::RemoteExecuteClientError::new(
            crate::remote_exec::RemoteExecuteClientErrorKind::Timeout,
            "REMOTE_TIMEOUT",
            "timed out",
        ));
        assert!(is_retryable_error(&timeout));
    }

    #[test]
    fn provider_mod_profile_endpoint_validation_rejects_placeholder_and_invalid_hosts() {
        let empty =
            validate_profile_endpoint("p1", "http:gpt-4o-mini", " ").expect_err("empty endpoint");
        assert!(empty
            .to_string()
            .contains("placeholder or invalid endpoint"));

        let invalid_host = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "https://api.example.invalid/v1/complete",
        )
        .expect_err("placeholder host should fail");
        assert!(invalid_host
            .to_string()
            .contains("configure providers.p1.config.endpoint"));

        validate_profile_endpoint("p1", "custom", "https://api.openai.com/v1/complete")
            .expect("real endpoint should pass");
    }

    #[test]
    fn provider_mod_profile_endpoint_validation_rejects_plain_http() {
        let err = validate_profile_endpoint(
            "p1",
            "http:gpt-4o-mini",
            "http://api.example.com/v1/complete",
        )
        .expect_err("plain http should fail");
        assert!(err.to_string().contains("must use an https:// endpoint"));
    }

    #[test]
    fn provider_mod_profile_endpoint_validation_rejects_hostless_https() {
        let err = validate_profile_endpoint("p1", "custom", "https://")
            .expect_err("https endpoints must include a host");
        assert!(err.to_string().contains("must use an https:// endpoint"));
    }

    #[test]
    fn provider_mod_profile_endpoint_validation_allows_loopback_http_for_local_harnesses() {
        validate_profile_endpoint("p1", "http:gpt-4o-mini", "http://127.0.0.1:8787/complete")
            .expect("loopback http should remain allowed");
    }

    #[test]
    fn provider_mod_profile_endpoint_validation_rejects_loopback_prefix_confusion() {
        let err =
            validate_profile_endpoint("p1", "http:gpt-4o-mini", "http://localhost.evil/complete")
                .expect_err("host suffix must not be treated as loopback");
        assert!(err.to_string().contains("must use an https:// endpoint"));
    }

    #[test]
    fn provider_mod_provider_profile_registry_includes_first_class_claude_profiles() {
        let names = provider_profile_names();
        assert!(names.contains(&"claude:claude-3-7-sonnet".to_string()));
        assert!(names.contains(&"claude:claude-3-5-haiku".to_string()));
        assert!(names.contains(&"claude:claude-opus-5".to_string()));

        let preset = provider_profile_registry()
            .get("claude:claude-3-7-sonnet")
            .copied()
            .expect("claude sonnet preset");
        assert_eq!(preset.kind, "http");
        assert_eq!(preset.default_model, Some("claude-3-7-sonnet-latest"));

        let opus = provider_profile_registry()
            .get("claude:claude-opus-5")
            .copied()
            .expect("claude opus 5 preset");
        assert_eq!(opus.kind, "anthropic");
        assert_eq!(opus.default_model, Some("claude-opus-5"));
        assert_eq!(opus.provider_model_id, Some("claude-opus-5"));
        assert_eq!(opus.endpoint, Some(ANTHROPIC_MESSAGES_ENDPOINT));
    }

    #[test]
    fn provider_mod_registry_keeps_expanded_vendor_identities_distinct() {
        let names = provider_profile_names();
        for name in [
            "kimi:k2.5",
            "minimax:m2.5",
            "qwen:qwen3-max",
            "xai:grok-4.5",
            "mistral:medium-3.5",
            "cohere:command-a-plus",
            "deepseek:v4",
            "gemini:3.1-pro-preview",
            "vertex_ai:gemini-2.5-flash",
        ] {
            assert!(names.contains(&name.to_string()), "missing profile {name}");
        }
        assert_eq!(
            provider_profile_registry()["vertex_ai:gemini-2.5-flash"].kind,
            "vertex_ai_gemini"
        );
        assert_eq!(
            provider_profile_registry()["vertex_ai:gemini-2.5-flash"].provider_model_id,
            Some("gemini-2.5-flash")
        );
        assert_eq!(
            provider_profile_registry()["vertex_ai:gemini-2.5-flash"].endpoint,
            None
        );
        assert_ne!(
            provider_profile_registry()["kimi:k2.5"].provider_model_id,
            provider_profile_registry()["xai:grok-4.5"].provider_model_id
        );
    }

    #[test]
    fn provider_mod_profile_expansion_materializes_bounded_inference_defaults() {
        let mut doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let expanded = expand_provider_profiles(&doc).expect("profile expansion");
        let local = expanded.providers.get("local").expect("local provider");
        assert_eq!(local.kind, "ollama");
        assert_eq!(local.default_model.as_deref(), Some("phi4-mini"));
        assert_eq!(
            local.config["provider_model_id"],
            serde_json::json!("phi4-mini")
        );
        assert_eq!(local.config["temperature"], serde_json::json!(0.0));
        assert_eq!(local.config["top_p"], serde_json::json!(1.0));
        assert_eq!(local.config["max_output_tokens"], serde_json::json!(512));
        assert_eq!(local.config["timeout_secs"], serde_json::json!(120));
        assert_eq!(local.config["deterministic_seed"], serde_json::json!(0));
        assert_eq!(
            local.config["materialization_policy"],
            serde_json::json!("deterministic_ollama_v1")
        );
        assert_eq!(
            local.config["profile_state"]["retention"],
            serde_json::json!("retain_last_valid_materialization")
        );

        doc.providers
            .get_mut("local")
            .expect("local")
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        let err = expand_provider_profiles(&doc).expect_err("out-of-bounds profile should fail");
        assert!(err.to_string().contains("config.temperature"));
    }

    #[test]
    fn provider_mod_profile_materialization_projection_is_stable_and_redacted() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "metadata".to_string(),
                        serde_json::json!({
                            "safe_count": 2,
                            "api_key": "secret-key",
                            "password": 123456,
                            "pin": 654321,
                            "passphrase": {
                                "hint": "swordfish"
                            },
                            "private_payload": {
                                "prompt": "do not retain"
                            }
                        }),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let projection1 =
            provider_profile_materialization_projection(&doc).expect("projection run 1");
        let projection2 =
            provider_profile_materialization_projection(&doc).expect("projection run 2");
        let json1 = serde_json::to_string(&projection1).expect("serialize projection 1");
        let json2 = serde_json::to_string(&projection2).expect("serialize projection 2");
        assert_eq!(json1, json2, "canonical projection must be byte-stable");
        assert!(json1.contains("adl.provider_profile_materialization_projection.v1"));
        assert!(json1.contains("\"safe_count\":2"));
        assert!(json1.contains("<redacted>"));
        assert!(!json1.contains("secret-key"));
        assert!(!json1.contains("do not retain"));
        assert!(!json1.contains("123456"));
        assert!(!json1.contains("654321"));
        assert!(!json1.contains("swordfish"));

        let mixed = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "hosted".to_string(),
                adl::ProviderSpec {
                    id: Some("hosted".to_string()),
                    profile: None,
                    kind: "http".to_string(),
                    base_url: Some(
                        "https://user:token@example.invalid/v1?api_key=secret".to_string(),
                    ),
                    default_model: Some("gpt-test".to_string()),
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let mixed_projection =
            provider_profile_materialization_projection(&mixed).expect("mixed projection");
        let mixed_json = serde_json::to_string(&mixed_projection).expect("mixed json");
        assert_eq!(
            mixed_projection["providers"]["hosted"]["base_url_present"],
            serde_json::json!(true)
        );
        assert!(!mixed_json.contains("user:token"));
        assert!(!mixed_json.contains("api_key=secret"));
        assert!(!mixed_json.contains("https://user"));
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_non_deterministic_ollama_seed() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "deterministic_seed".to_string(),
                        serde_json::json!(7),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let err = expand_provider_profiles(&doc).expect_err("seed drift should fail");
        assert!(err.to_string().contains("deterministic_seed must remain 0"));
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_malformed_inference_values() {
        for (key, value) in [
            ("temperature", serde_json::json!("hot")),
            ("temperature", serde_json::json!("0.2")),
            ("top_p", serde_json::json!(true)),
            ("timeout_secs", serde_json::json!("later")),
            ("timeout_secs", serde_json::json!("120")),
            ("timeout_secs", serde_json::json!(601)),
            ("max_output_tokens", serde_json::json!(-1)),
            ("max_output_tokens", serde_json::json!(32_769)),
            ("deterministic_seed", serde_json::json!("seed")),
            ("deterministic_seed", serde_json::json!("0")),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some("ollama:phi4-mini".to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([(key.to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };
            let err = expand_provider_profiles(&doc).expect_err("malformed profile should fail");
            assert!(err.to_string().contains(key), "{key}: {err:#}");
        }
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_provider_model_id_conflicts() {
        for value in [
            serde_json::json!("llama3.1:8b"),
            serde_json::json!(123),
            serde_json::json!(true),
            serde_json::json!({ "model": "phi4-mini" }),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some("ollama:phi4-mini".to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([("provider_model_id".to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };

            let err = expand_provider_profiles(&doc).expect_err("model conflict should fail");
            assert!(err.to_string().contains("provider_model_id"));
        }
    }

    #[test]
    fn provider_mod_profile_expansion_rejects_malformed_identity_config() {
        for (profile, key, value) in [
            ("z_ai:glm-5", "endpoint", serde_json::json!(123)),
            (
                "z_ai:glm-5",
                "endpoint",
                serde_json::json!({ "url": "https://open.bigmodel.cn/api/paas/v4/chat/completions" }),
            ),
            ("ollama:phi4-mini", "vendor", serde_json::json!(true)),
        ] {
            let doc = adl::AdlDoc {
                version: "0.92".to_string(),
                providers: HashMap::from([(
                    "local".to_string(),
                    adl::ProviderSpec {
                        id: Some("local".to_string()),
                        profile: Some(profile.to_string()),
                        kind: String::new(),
                        base_url: None,
                        default_model: None,
                        config: HashMap::from([(key.to_string(), value)]),
                    },
                )]),
                tools: HashMap::new(),
                agents: HashMap::new(),
                tasks: HashMap::new(),
                workflows: HashMap::new(),
                patterns: Vec::new(),
                signature: None,
                run: adl::RunSpec {
                    id: None,
                    name: None,
                    created_at: None,
                    defaults: Default::default(),
                    workflow_ref: None,
                    workflow: None,
                    pattern_ref: None,
                    inputs: HashMap::new(),
                    placement: None,
                    remote: None,
                    delegation_policy: None,
                },
            };

            let err = expand_provider_profiles(&doc).expect_err("malformed config should fail");
            assert!(err.to_string().contains(key), "{key}: {err:#}");
        }
    }

    #[test]
    fn provider_mod_profile_state_retains_previous_last_known_good() {
        let active_doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::new(),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let active = expand_provider_profiles(&active_doc).expect("active profile expansion");
        let mut doc = active_doc.clone();
        let local = doc.providers.get_mut("local").expect("local provider");
        local.profile = Some("ollama:qwen2.5-7b".to_string());
        local.config.insert(
            "profile_state".to_string(),
            active.providers["local"].config["profile_state"].clone(),
        );

        let expanded = expand_provider_profiles(&doc).expect("profile expansion");
        let state = &expanded.providers["local"].config["profile_state"];
        assert_eq!(state["profile"], serde_json::json!("ollama:qwen2.5-7b"));
        assert_eq!(
            state["last_known_good_profile"],
            serde_json::json!("ollama:phi4-mini")
        );
        assert_eq!(
            state["last_known_good_materialization"]["schema"],
            serde_json::json!("adl.provider_profile_materialization_state.v1")
        );
        assert_eq!(
            state["last_known_good_materialization"]["profile"],
            serde_json::json!("ollama:phi4-mini")
        );
        assert_eq!(
            state["last_known_good_materialization"]["default_model"],
            serde_json::json!("phi4-mini")
        );
    }

    #[test]
    fn provider_mod_profile_activation_preserves_active_materialization_on_invalid_candidate() {
        let active_doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:phi4-mini".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "private_payload".to_string(),
                        serde_json::json!("do not leak active prompt"),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };
        let active = expand_provider_profiles(&active_doc).expect("active materialization");
        let active_provider = active.providers["local"].clone();
        let active_projection = redacted_provider_profile_projection("local", &active_provider);

        let mut candidate_doc = active_doc.clone();
        let candidate = candidate_doc.providers.get_mut("local").expect("candidate");
        candidate.profile = Some("ollama:qwen2.5-7b".to_string());
        candidate
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        candidate.config.insert(
            "profile_state".to_string(),
            active_provider.config["profile_state"].clone(),
        );

        let rejected = activate_provider_profile_candidate(&active_doc, &candidate_doc)
            .expect("invalid candidate should retain active state");
        assert!(!rejected.accepted);
        assert!(rejected
            .rejection
            .as_deref()
            .unwrap_or_default()
            .contains("config.temperature"));
        assert_eq!(
            rejected.document, active,
            "failed candidate activation must return the retained active materialization"
        );
        assert_eq!(
            redacted_provider_profile_projection("local", &rejected.document.providers["local"]),
            active_projection
        );

        let mut valid_candidate_doc = active_doc.clone();
        valid_candidate_doc
            .providers
            .get_mut("local")
            .expect("candidate")
            .profile = Some("ollama:qwen2.5-7b".to_string());
        let accepted = activate_provider_profile_candidate(&active_doc, &valid_candidate_doc)
            .expect("valid candidate should promote");
        assert!(accepted.accepted);
        assert!(accepted.rejection.is_none());
        let promoted_state = &accepted.document.providers["local"].config["profile_state"];
        assert_eq!(
            promoted_state["last_known_good_profile"],
            serde_json::json!("ollama:qwen2.5-7b")
        );
        assert_eq!(
            promoted_state["last_known_good_materialization"]["profile"],
            serde_json::json!("ollama:qwen2.5-7b")
        );

        let mut chained_invalid_doc = active_doc.clone();
        let chained_invalid = chained_invalid_doc
            .providers
            .get_mut("local")
            .expect("candidate");
        chained_invalid.profile = Some("ollama:phi4-mini".to_string());
        chained_invalid
            .config
            .insert("temperature".to_string(), serde_json::json!(3.0));
        let retained =
            activate_provider_profile_candidate(&accepted.document, &chained_invalid_doc)
                .expect("chained invalid candidate should retain materialized active state");
        assert!(!retained.accepted);
        assert_eq!(
            retained.document, accepted.document,
            "returned materialized activation document must be reusable as active state"
        );

        let accepted_again = activate_provider_profile_candidate(&retained.document, &active_doc)
            .expect("retained materialized active state should allow later valid activation");
        assert!(accepted_again.accepted);
        assert_eq!(
            accepted_again.document.providers["local"].config["profile_state"]
                ["last_known_good_profile"],
            serde_json::json!("ollama:phi4-mini")
        );
    }

    #[test]
    fn provider_mod_profile_state_rejects_unknown_last_known_good() {
        let doc = adl::AdlDoc {
            version: "0.92".to_string(),
            providers: HashMap::from([(
                "local".to_string(),
                adl::ProviderSpec {
                    id: Some("local".to_string()),
                    profile: Some("ollama:qwen2.5-7b".to_string()),
                    kind: String::new(),
                    base_url: None,
                    default_model: None,
                    config: HashMap::from([(
                        "profile_state".to_string(),
                        serde_json::json!({
                            "schema": "adl.provider_profile_state.v1",
                            "profile": "ollama:phi4-mini",
                            "last_known_good_profile": "ollama:unknown",
                            "retention": "retain_last_valid_materialization",
                            "activation": "validate_before_activation"
                        }),
                    )]),
                },
            )]),
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: Vec::new(),
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: Default::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let err = expand_provider_profiles(&doc).expect_err("unknown LKG should fail");
        assert!(err.to_string().contains("last_known_good_profile"));
    }

    #[test]
    fn provider_mod_redacted_profile_projection_excludes_private_payloads() {
        let spec = adl::ProviderSpec {
            id: Some("hosted".to_string()),
            profile: Some("chatgpt:gpt-5.4".to_string()),
            kind: "http".to_string(),
            base_url: None,
            default_model: Some("gpt-5.4".to_string()),
            config: HashMap::from([
                ("temperature".to_string(), serde_json::json!(0.2)),
                (
                    "auth".to_string(),
                    serde_json::json!({"env": "OPENAI_API_KEY"}),
                ),
                (
                    "private_payload".to_string(),
                    serde_json::json!("raw prompt"),
                ),
                (
                    "metadata".to_string(),
                    serde_json::json!({
                        "recovery_code": 123456,
                        "pin": 654321,
                        "password": {"value": 777777},
                        "safe_count": 2
                    }),
                ),
                ("passphrase".to_string(), serde_json::json!(987654)),
            ]),
        };

        let projection = redacted_provider_profile_projection("hosted", &spec);
        assert_eq!(
            projection["schema"],
            serde_json::json!("adl.provider_profile_redacted_projection.v1")
        );
        assert_eq!(projection["config"]["temperature"], serde_json::json!(0.2));
        assert_eq!(
            projection["config"]["auth"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["private_payload"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["metadata"]["recovery_code"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["metadata"]["pin"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["metadata"]["password"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["passphrase"],
            serde_json::json!("<redacted>")
        );
        assert_eq!(
            projection["config"]["metadata"]["safe_count"],
            serde_json::json!(2)
        );
        let rendered = serde_json::to_string(&projection).expect("json");
        assert!(!rendered.contains("OPENAI_API_KEY"));
        assert!(!rendered.contains("raw prompt"));
        assert!(!rendered.contains("123456"));
        assert!(!rendered.contains("654321"));
        assert!(!rendered.contains("777777"));
        assert!(!rendered.contains("987654"));
    }

    #[test]
    fn provider_mod_cfg_numeric_helpers_cover_all_supported_and_rejected_types() {
        let mut cfg = HashMap::new();
        cfg.insert("f64".to_string(), serde_json::json!(0.5));
        cfg.insert("i64".to_string(), serde_json::json!(2));
        cfg.insert("str".to_string(), serde_json::json!("3.25"));
        cfg.insert("bad_str".to_string(), serde_json::json!("not-a-number"));
        cfg.insert("bool".to_string(), serde_json::json!(true));
        cfg.insert("u64".to_string(), serde_json::json!(7));
        cfg.insert("neg_i64".to_string(), serde_json::json!(-1));

        assert_eq!(cfg_f32(&cfg, "f64"), Some(0.5_f32));
        assert_eq!(cfg_f32(&cfg, "i64"), Some(2.0_f32));
        assert_eq!(cfg_f32(&cfg, "str"), Some(3.25_f32));
        assert_eq!(cfg_f32(&cfg, "bad_str"), None);
        assert_eq!(cfg_f32(&cfg, "bool"), None);
        assert_eq!(cfg_f32(&cfg, "missing"), None);

        assert_eq!(cfg_u64(&cfg, "u64"), Some(7_u64));
        assert_eq!(cfg_u64(&cfg, "i64"), Some(2_u64));
        assert_eq!(cfg_u64(&cfg, "str"), None);
        assert_eq!(cfg_u64(&cfg, "neg_i64"), None);
        assert_eq!(cfg_u64(&cfg, "bad_str"), None);
        assert_eq!(cfg_u64(&cfg, "bool"), None);
    }

    #[test]
    fn provider_mod_timeout_secs_rejects_zero_and_uses_default_without_env() {
        let prev_adl = env::var_os("ADL_TIMEOUT_SECS");

        env::set_var("ADL_TIMEOUT_SECS", "0");
        let err = timeout_secs().expect_err("zero timeout env should fail");
        assert!(err.to_string().contains("invalid ADL_TIMEOUT_SECS"));

        env::remove_var("ADL_TIMEOUT_SECS");
        assert_eq!(timeout_secs().expect("default timeout"), 120);

        match prev_adl {
            Some(v) => env::set_var("ADL_TIMEOUT_SECS", v),
            None => env::remove_var("ADL_TIMEOUT_SECS"),
        }
    }
}
