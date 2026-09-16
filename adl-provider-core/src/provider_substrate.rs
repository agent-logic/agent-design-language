use anyhow::{anyhow, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

use crate as adl;
use crate::model_identity::{observed_at_now_v1, ModelIdentityStrengthV1, ModelIdentityV1};

pub const PROVIDER_SUBSTRATE_MANIFEST_SCHEMA: &str = "provider_substrate_manifest.v1";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityModeV1 {
    Native,
    PromptBased,
    SemanticFallback,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct CapabilitySupportV1 {
    pub supported: bool,
    pub mode: CapabilityModeV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderCapabilitiesV1 {
    pub tool_calling: CapabilitySupportV1,
    pub structured_json: CapabilitySupportV1,
    pub semantic_tool_fallback: CapabilitySupportV1,
    #[serde(default = "unsupported_capability")]
    pub speech_synthesis: CapabilitySupportV1,
    #[serde(default = "unsupported_capability")]
    pub speech_transcription: CapabilitySupportV1,
}

fn unsupported_capability() -> CapabilitySupportV1 {
    CapabilitySupportV1 {
        supported: false,
        mode: CapabilityModeV1::None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderTransportV1 {
    Http,
    LocalCli,
    InProcess,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum InferenceControlV1 {
    ContextWindowTokens,
    MaxOutputTokens,
    Temperature,
    TopP,
    DeterministicSeed,
    TimeoutSecs,
    ReasoningEffort,
    ThinkingBudget,
    ThinkingLevel,
    IncludeThoughts,
    ClearThinking,
    Think,
    LocalKeepAlive,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum OllamaThinkV1 {
    Enabled(bool),
    Level(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum LocalKeepAliveV1 {
    Seconds(i64),
    Duration(String),
}

/// Canonical, validated inference controls carried from AProvider data to a
/// trusted built-in codec. None means the operator did not declare a value.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct EffectiveInferenceConfigV1 {
    #[serde(default)]
    pub context_window_tokens: Option<u64>,
    #[serde(default)]
    pub max_output_tokens: Option<u64>,
    #[serde(default)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub top_p: Option<f64>,
    #[serde(default)]
    pub deterministic_seed: Option<u64>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default)]
    pub reasoning_effort: Option<String>,
    #[serde(default)]
    pub thinking_budget: Option<u64>,
    #[serde(default)]
    pub thinking_level: Option<String>,
    #[serde(default)]
    pub include_thoughts: Option<bool>,
    #[serde(default)]
    pub clear_thinking: Option<bool>,
    #[serde(default)]
    pub think: Option<OllamaThinkV1>,
    #[serde(default)]
    pub local_keep_alive: Option<LocalKeepAliveV1>,
}

impl EffectiveInferenceConfigV1 {
    fn supplied_controls(&self) -> Vec<InferenceControlV1> {
        let entries = [
            (
                self.context_window_tokens.is_some(),
                InferenceControlV1::ContextWindowTokens,
            ),
            (
                self.max_output_tokens.is_some(),
                InferenceControlV1::MaxOutputTokens,
            ),
            (self.temperature.is_some(), InferenceControlV1::Temperature),
            (self.top_p.is_some(), InferenceControlV1::TopP),
            (
                self.deterministic_seed.is_some(),
                InferenceControlV1::DeterministicSeed,
            ),
            (self.timeout_secs.is_some(), InferenceControlV1::TimeoutSecs),
            (
                self.reasoning_effort.is_some(),
                InferenceControlV1::ReasoningEffort,
            ),
            (
                self.thinking_budget.is_some(),
                InferenceControlV1::ThinkingBudget,
            ),
            (
                self.thinking_level.is_some(),
                InferenceControlV1::ThinkingLevel,
            ),
            (
                self.include_thoughts.is_some(),
                InferenceControlV1::IncludeThoughts,
            ),
            (
                self.clear_thinking.is_some(),
                InferenceControlV1::ClearThinking,
            ),
            (self.think.is_some(), InferenceControlV1::Think),
            (
                self.local_keep_alive.is_some(),
                InferenceControlV1::LocalKeepAlive,
            ),
        ];
        entries
            .into_iter()
            .filter_map(|(present, control)| present.then_some(control))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderCodecControlsV1 {
    pub codec: String,
    pub consumes: Vec<InferenceControlV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ProviderCredentialReferenceV1 {
    pub strategy: String,
    pub environment: String,
    #[serde(default)]
    pub file_environment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct RedactedEffectiveInferenceProjectionV1 {
    pub schema: String,
    pub provider_id: String,
    pub provider_kind: String,
    pub transport: ProviderTransportV1,
    pub codec: String,
    pub consumes: Vec<InferenceControlV1>,
    pub effective: EffectiveInferenceConfigV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ProviderSubstrateV1 {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: ProviderTransportV1,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub default_model_ref: Option<String>,
    #[serde(default)]
    pub provider_default_model_id: Option<String>,
    pub capabilities: ProviderCapabilitiesV1,
    #[serde(default)]
    pub credential_reference: Option<ProviderCredentialReferenceV1>,
    pub codec_controls: ProviderCodecControlsV1,
    pub effective_inference: EffectiveInferenceConfigV1,
    pub inference_parameter_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct ProviderInvocationTargetV1 {
    pub provider_id: String,
    pub provider_kind: String,
    pub vendor: String,
    pub transport: ProviderTransportV1,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    pub model_ref: String,
    pub provider_model_id: String,
    pub model_identity: ModelIdentityV1,
    pub capabilities: ProviderCapabilitiesV1,
    #[serde(default)]
    pub credential_reference: Option<ProviderCredentialReferenceV1>,
    pub codec_controls: ProviderCodecControlsV1,
    pub effective_inference: EffectiveInferenceConfigV1,
}

fn cfg_str<'a>(cfg: &'a HashMap<String, Value>, key: &str) -> Option<&'a str> {
    cfg.get(key).and_then(|v| v.as_str()).map(str::trim)
}

fn invalid_control(key: &str, detail: &str) -> anyhow::Error {
    anyhow!("invalid_aprovider_inference_control: config.{key} {detail}")
}

fn optional_u64(
    cfg: &HashMap<String, Value>,
    key: &str,
    minimum: u64,
    maximum: u64,
) -> Result<Option<u64>> {
    let Some(value) = cfg.get(key) else {
        return Ok(None);
    };
    let value = value
        .as_u64()
        .filter(|value| *value >= minimum && *value <= maximum)
        .ok_or_else(|| {
            invalid_control(key, &format!("must be an integer in {minimum}..={maximum}"))
        })?;
    Ok(Some(value))
}

fn optional_f64(
    cfg: &HashMap<String, Value>,
    key: &str,
    minimum: f64,
    maximum: f64,
) -> Result<Option<f64>> {
    let Some(value) = cfg.get(key) else {
        return Ok(None);
    };
    let value = value
        .as_f64()
        .filter(|value| value.is_finite() && *value >= minimum && *value <= maximum)
        .ok_or_else(|| {
            invalid_control(
                key,
                &format!("must be a finite number in {minimum}..={maximum}"),
            )
        })?;
    Ok(Some(value))
}

fn reject_executable_aprovider_fields(value: &Value) -> Result<()> {
    const FORBIDDEN: &[&str] = &[
        "executable",
        "executable_path",
        "command",
        "commands",
        "script",
        "script_path",
        "dynamic_library",
        "dynamic_library_path",
        "plugin",
        "plugin_path",
        "embedded_code",
        "code",
        "workflow",
        "workflow_authority",
        "lifecycle",
        "lifecycle_authority",
    ];
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let normalized = key.trim().to_ascii_lowercase().replace('-', "_");
                let forbidden_authority_token = normalized.split('_').any(|token| {
                    matches!(
                        token,
                        "command"
                            | "commands"
                            | "script"
                            | "scripts"
                            | "executable"
                            | "binary"
                            | "plugin"
                            | "library"
                            | "workflow"
                            | "lifecycle"
                    )
                });
                if FORBIDDEN.contains(&normalized.as_str()) || forbidden_authority_token {
                    return Err(anyhow!(
                        "aprovider_executable_authority_forbidden: config.{normalized}"
                    ));
                }
                reject_executable_aprovider_fields(value)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                reject_executable_aprovider_fields(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn effective_inference_config_v1(spec: &adl::ProviderSpec) -> Result<EffectiveInferenceConfigV1> {
    reject_executable_aprovider_fields(&serde_json::to_value(&spec.config)?)?;
    let max_output_tokens = optional_u64(&spec.config, "max_output_tokens", 1, 131_072)?;
    let max_tokens = optional_u64(&spec.config, "max_tokens", 1, 131_072)?;
    if max_output_tokens.is_some() && max_tokens.is_some() && max_output_tokens != max_tokens {
        return Err(anyhow!(
            "conflicting_aprovider_inference_controls: config.max_output_tokens and config.max_tokens differ"
        ));
    }
    let runtime_cap = optional_u64(&spec.config, "runtime_max_output_tokens", 1, 32_768)?;
    let declared_output = max_output_tokens.or(max_tokens);
    let max_output_tokens = match (declared_output, runtime_cap) {
        (Some(declared), Some(cap)) => Some(declared.min(cap)),
        (declared, cap) => declared.or(cap),
    };
    let reasoning_effort = match spec.config.get("reasoning_effort") {
        None => None,
        Some(Value::String(value)) => {
            let value = value.trim().to_ascii_lowercase();
            if !matches!(value.as_str(), "low" | "medium" | "high" | "max") {
                return Err(invalid_control(
                    "reasoning_effort",
                    "must be one of low, medium, high, max",
                ));
            }
            Some(value)
        }
        Some(_) => return Err(invalid_control("reasoning_effort", "must be a string")),
    };
    let clear_thinking = match spec.config.get("clear_thinking") {
        None => None,
        Some(Value::Bool(value)) => Some(*value),
        Some(_) => return Err(invalid_control("clear_thinking", "must be a boolean")),
    };
    let thinking_budget =
        match spec.config.get("thinking_budget") {
            None => None,
            Some(value) => Some(value.as_u64().ok_or_else(|| {
                invalid_control("thinking_budget", "must be a non-negative integer")
            })?),
        };
    let thinking_level = match spec.config.get("thinking_level") {
        None => None,
        Some(Value::String(value)) => {
            let value = value.trim().to_ascii_lowercase();
            if !matches!(value.as_str(), "minimal" | "low" | "medium" | "high") {
                return Err(invalid_control(
                    "thinking_level",
                    "must be one of minimal, low, medium, high",
                ));
            }
            Some(value)
        }
        Some(_) => return Err(invalid_control("thinking_level", "must be a string")),
    };
    if thinking_budget.is_some() && thinking_level.is_some() {
        return Err(anyhow!(
            "conflicting_aprovider_inference_controls: config.thinking_budget and config.thinking_level are mutually exclusive"
        ));
    }
    let include_thoughts = match spec.config.get("include_thoughts") {
        None => None,
        Some(Value::Bool(value)) => Some(*value),
        Some(_) => return Err(invalid_control("include_thoughts", "must be a boolean")),
    };
    let think = match spec.config.get("think") {
        None => None,
        Some(Value::Bool(value)) => Some(OllamaThinkV1::Enabled(*value)),
        Some(Value::String(value)) => {
            let level = value.trim().to_ascii_lowercase();
            if !matches!(level.as_str(), "low" | "medium" | "high") {
                return Err(invalid_control(
                    "think",
                    "must be a boolean or low, medium, high",
                ));
            }
            Some(OllamaThinkV1::Level(level))
        }
        Some(_) => return Err(invalid_control("think", "must be a boolean or string")),
    };
    let local_keep_alive = match spec.config.get("local_keep_alive") {
        None => None,
        Some(Value::Number(value)) => value
            .as_i64()
            .map(LocalKeepAliveV1::Seconds)
            .ok_or_else(|| {
                invalid_control("local_keep_alive", "must be an integer or duration string")
            })
            .map(Some)?,
        Some(Value::String(value)) => {
            let value = value.trim();
            if value.is_empty() || value.len() > 64 || value.chars().any(char::is_control) {
                return Err(invalid_control(
                    "local_keep_alive",
                    "must be a non-empty bounded duration string",
                ));
            }
            Some(match value.parse::<i64>() {
                Ok(seconds) => LocalKeepAliveV1::Seconds(seconds),
                Err(_) => LocalKeepAliveV1::Duration(value.to_string()),
            })
        }
        Some(_) => {
            return Err(invalid_control(
                "local_keep_alive",
                "must be an integer or string",
            ))
        }
    };
    Ok(EffectiveInferenceConfigV1 {
        context_window_tokens: optional_u64(&spec.config, "context_window_tokens", 1, 1_048_576)?,
        max_output_tokens,
        temperature: optional_f64(&spec.config, "temperature", 0.0, 2.0)?,
        top_p: optional_f64(&spec.config, "top_p", 0.0, 1.0)?,
        deterministic_seed: optional_u64(&spec.config, "deterministic_seed", 0, u32::MAX as u64)?,
        timeout_secs: optional_u64(&spec.config, "timeout_secs", 1, 86_400)?,
        reasoning_effort,
        thinking_budget,
        thinking_level,
        include_thoughts,
        clear_thinking,
        think,
        local_keep_alive,
    })
}

fn credential_reference_v1(
    spec: &adl::ProviderSpec,
    codec: &ProviderCodecControlsV1,
) -> Result<Option<ProviderCredentialReferenceV1>> {
    let nested = match spec.config.get("auth") {
        None => None,
        Some(Value::Object(auth)) => Some(auth),
        Some(_) => {
            return Err(anyhow!(
                "invalid_aprovider_credential_reference: auth must be an object"
            ))
        }
    };
    if nested.is_some_and(|auth| {
        ["type", "env", "file_env"]
            .into_iter()
            .any(|key| auth.get(key).is_some_and(|value| !value.is_string()))
    }) || ["api_key_env", "auth_env", "token_env"]
        .into_iter()
        .any(|key| spec.config.get(key).is_some_and(|value| !value.is_string()))
    {
        return Err(anyhow!(
            "invalid_aprovider_credential_reference: references must be strings"
        ));
    }
    let strategy = nested
        .and_then(|auth| auth.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("bearer")
        .trim();
    let environment = nested
        .and_then(|auth| auth.get("env"))
        .and_then(Value::as_str)
        .or_else(|| cfg_str(&spec.config, "api_key_env"))
        .or_else(|| cfg_str(&spec.config, "auth_env"))
        .or_else(|| cfg_str(&spec.config, "token_env"))
        .or_else(|| {
            (codec.codec == "vertex_gemini_v1" && matches!(strategy, "adc" | "workload_identity"))
                .then_some("ADL_VERTEX_AI_ACCESS_TOKEN")
        });
    let file_environment = nested
        .and_then(|auth| auth.get("file_env"))
        .and_then(Value::as_str);
    let Some(environment) = environment else {
        if file_environment.is_some() {
            return Err(anyhow!(
                "invalid_aprovider_credential_reference: file_env requires env"
            ));
        }
        return Ok(None);
    };
    let valid_env = |value: &str| {
        !value.is_empty()
            && value.bytes().enumerate().all(|(index, byte)| {
                byte == b'_' || byte.is_ascii_alphabetic() || (index > 0 && byte.is_ascii_digit())
            })
    };
    let strategy_supported = strategy == "bearer"
        || (codec.codec == "vertex_gemini_v1" && matches!(strategy, "adc" | "workload_identity"));
    if !strategy_supported
        || !valid_env(environment)
        || file_environment.is_some_and(|value| !valid_env(value))
        || (strategy != "bearer" && file_environment.is_some())
    {
        return Err(anyhow!(
            "invalid_aprovider_credential_reference: strategy is not supported by the selected codec"
        ));
    }
    Ok(Some(ProviderCredentialReferenceV1 {
        strategy: strategy.to_string(),
        environment: environment.to_string(),
        file_environment: file_environment.map(ToString::to_string),
    }))
}

fn normalize_vendor_token(raw: &str) -> Option<String> {
    let token = raw.trim().to_lowercase();
    if token.is_empty() {
        return None;
    }
    let valid = token
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-');
    valid.then_some(token)
}

fn parse_capability_mode(raw: &str) -> Option<CapabilityModeV1> {
    match raw.trim().to_lowercase().as_str() {
        "native" => Some(CapabilityModeV1::Native),
        "prompt_based" | "prompt-based" => Some(CapabilityModeV1::PromptBased),
        "semantic_fallback" | "semantic-fallback" => Some(CapabilityModeV1::SemanticFallback),
        "none" => Some(CapabilityModeV1::None),
        _ => None,
    }
}

fn capability_override(cfg: &HashMap<String, Value>, key: &str) -> Option<CapabilitySupportV1> {
    let caps = cfg.get("capabilities")?.as_object()?;
    let entry = caps.get(key)?.as_object()?;
    let supported = entry.get("supported")?.as_bool()?;
    let mode = entry
        .get("mode")
        .and_then(|v| v.as_str())
        .and_then(parse_capability_mode)?;
    Some(CapabilitySupportV1 { supported, mode })
}

fn infer_vendor(spec: &adl::ProviderSpec) -> String {
    if let Some(explicit) = cfg_str(&spec.config, "vendor").and_then(normalize_vendor_token) {
        return explicit;
    }

    if let Some(profile) = spec.profile.as_deref() {
        if let Some((family, _)) = profile.split_once(':') {
            match family {
                "ollama" => return "ollama".to_string(),
                "mlx" => return "mlx".to_string(),
                "mock" => return "mock".to_string(),
                "chatgpt" => return "openai".to_string(),
                "claude" => return "anthropic".to_string(),
                "bedrock" => return "aws_bedrock".to_string(),
                "openrouter" => return "openrouter".to_string(),
                "z_ai" | "zai" | "zhipu" => return "z_ai".to_string(),
                "kimi" | "moonshot" => return "kimi".to_string(),
                "minimax" => return "minimax".to_string(),
                "qwen" => return "qwen".to_string(),
                "xai" => return "xai".to_string(),
                "mistral" => return "mistral".to_string(),
                "cohere" => return "cohere".to_string(),
                "gemini" => return "google".to_string(),
                "vertex" | "vertex_ai" | "vertex_ai_gemini" => {
                    return "google_vertex_ai".to_string();
                }
                "http" => return "generic_http".to_string(),
                "deepgram" => return "deepgram".to_string(),
                _ => {}
            }
        }
    }

    let endpoint = spec
        .base_url
        .as_deref()
        .or_else(|| cfg_str(&spec.config, "endpoint"));
    if let Some(endpoint) = endpoint {
        let lower = endpoint.to_lowercase();
        if lower.contains("openai") {
            return "openai".to_string();
        }
        if lower.contains("anthropic") {
            return "anthropic".to_string();
        }
        if lower.contains("googleapis.com")
            || lower.contains("generativelanguage")
            || lower.contains("gemini")
        {
            if lower.contains("aiplatform.googleapis.com") {
                return "google_vertex_ai".to_string();
            }
            return "google".to_string();
        }
        if lower.contains("deepseek") {
            return "deepseek".to_string();
        }
        if lower.contains("deepgram") {
            return "deepgram".to_string();
        }
        if lower.contains("openrouter") {
            return "openrouter".to_string();
        }
        if lower.contains("bedrock-runtime") || lower.contains("bedrock") {
            return "aws_bedrock".to_string();
        }
        if lower.contains("bigmodel.cn") || lower.contains("z.ai") || lower.contains("zhipu") {
            return "z_ai".to_string();
        }
        if lower.contains("ollama") || lower.contains("11434") {
            return "ollama".to_string();
        }
    }

    match spec.kind.trim() {
        "ollama" | "local_ollama" => "ollama".to_string(),
        "mock" => "mock".to_string(),
        "openai" => "openai".to_string(),
        "anthropic" => "anthropic".to_string(),
        "deepseek" => "deepseek".to_string(),
        "kimi" | "moonshot" => "kimi".to_string(),
        "bedrock" | "aws_bedrock" => "aws_bedrock".to_string(),
        "openrouter" => "openrouter".to_string(),
        "vertex_ai_gemini" | "vertex_ai" | "vertex" => "google_vertex_ai".to_string(),
        "z_ai" | "zai" | "zhipu" => "z_ai".to_string(),
        "http" | "http_remote" => "generic_http".to_string(),
        "deepgram" => "deepgram".to_string(),
        other if !other.is_empty() => other.to_lowercase(),
        _ => "unknown".to_string(),
    }
}

fn infer_transport(spec: &adl::ProviderSpec) -> Result<ProviderTransportV1> {
    match spec.kind.trim() {
        "ollama" => {
            if spec.base_url.is_some() || cfg_str(&spec.config, "endpoint").is_some() {
                Ok(ProviderTransportV1::Http)
            } else {
                Ok(ProviderTransportV1::LocalCli)
            }
        }
        "mlx" | "http" | "http_remote" | "openai" | "anthropic" | "deepseek" | "kimi"
        | "moonshot" | "openrouter" | "bedrock" | "aws_bedrock" | "z_ai" | "zai" | "zhipu"
        | "deepgram" | "vertex_ai_gemini" | "vertex_ai" | "vertex" => Ok(ProviderTransportV1::Http),
        "local_ollama" => Ok(ProviderTransportV1::LocalCli),
        "mock" => Ok(ProviderTransportV1::InProcess),
        other => Err(anyhow!(
            "unsupported provider kind '{other}' for provider substrate v1"
        )),
    }
}

fn normalized_provider_kind(kind: &str) -> String {
    match kind.trim() {
        "moonshot" => "kimi".to_string(),
        other => other.to_string(),
    }
}

fn generic_http_chat_mode(spec: &adl::ProviderSpec) -> bool {
    spec.config.get("api_format").and_then(Value::as_str) == Some("openai_chat_completions")
        || spec
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
            })
}

fn codec_controls(
    provider_kind: &str,
    transport: &ProviderTransportV1,
    spec: &adl::ProviderSpec,
) -> ProviderCodecControlsV1 {
    use InferenceControlV1::*;
    let common = vec![MaxOutputTokens, Temperature, TopP, TimeoutSecs];
    let (codec, consumes) = match (provider_kind, transport) {
        ("ollama", ProviderTransportV1::Http) => (
            "ollama_generate_v1",
            vec![
                ContextWindowTokens,
                MaxOutputTokens,
                Temperature,
                TopP,
                DeterministicSeed,
                TimeoutSecs,
                Think,
                LocalKeepAlive,
            ],
        ),
        ("mlx", ProviderTransportV1::Http) => (
            "mlx_openai_chat_v1",
            vec![
                MaxOutputTokens,
                Temperature,
                TopP,
                DeterministicSeed,
                TimeoutSecs,
            ],
        ),
        ("kimi", ProviderTransportV1::Http) => (
            "kimi_chat_v1",
            vec![
                MaxOutputTokens,
                Temperature,
                TopP,
                TimeoutSecs,
                ReasoningEffort,
            ],
        ),
        ("z_ai", ProviderTransportV1::Http)
        | ("zai", ProviderTransportV1::Http)
        | ("zhipu", ProviderTransportV1::Http) => (
            "z_ai_chat_v1",
            vec![
                MaxOutputTokens,
                Temperature,
                TopP,
                TimeoutSecs,
                ReasoningEffort,
                ClearThinking,
            ],
        ),
        ("ollama", ProviderTransportV1::LocalCli)
        | ("local_ollama", ProviderTransportV1::LocalCli) => ("ollama_cli_v1", vec![TimeoutSecs]),
        ("mock", ProviderTransportV1::InProcess) => ("mock_v1", vec![]),
        ("deepgram", ProviderTransportV1::Http) => ("deepgram_speech_v1", vec![TimeoutSecs]),
        ("openai", ProviderTransportV1::Http) => ("openai_responses_v1", common.clone()),
        ("anthropic", ProviderTransportV1::Http) => ("anthropic_messages_v1", common.clone()),
        ("deepseek", ProviderTransportV1::Http) => ("deepseek_chat_v1", common.clone()),
        ("openrouter", ProviderTransportV1::Http) => ("openrouter_chat_v1", common.clone()),
        ("bedrock", ProviderTransportV1::Http) | ("aws_bedrock", ProviderTransportV1::Http) => {
            ("aws_bedrock_invoke_v1", common.clone())
        }
        ("vertex_ai_gemini", ProviderTransportV1::Http)
        | ("vertex_ai", ProviderTransportV1::Http)
        | ("vertex", ProviderTransportV1::Http) => (
            "vertex_gemini_v1",
            vec![
                MaxOutputTokens,
                Temperature,
                TopP,
                TimeoutSecs,
                ThinkingBudget,
                ThinkingLevel,
                IncludeThoughts,
            ],
        ),
        ("http", ProviderTransportV1::Http) | ("http_remote", ProviderTransportV1::Http) => {
            if generic_http_chat_mode(spec) {
                ("generic_http_chat_v1", common)
            } else {
                ("generic_http_legacy_v1", vec![TimeoutSecs])
            }
        }
        _ => ("unsupported_builtin_codec", vec![]),
    };
    ProviderCodecControlsV1 {
        codec: codec.to_string(),
        consumes,
    }
}

fn validate_codec_controls(
    effective: &EffectiveInferenceConfigV1,
    codec: &ProviderCodecControlsV1,
) -> Result<()> {
    let unsupported = effective
        .supplied_controls()
        .into_iter()
        .filter(|control| !codec.consumes.contains(control))
        .collect::<Vec<_>>();
    if !unsupported.is_empty() {
        let names = unsupported
            .iter()
            .filter_map(|control| serde_json::to_value(control).ok())
            .filter_map(|value| value.as_str().map(ToString::to_string))
            .collect::<Vec<_>>()
            .join(",");
        return Err(crate::provider::unsupported_capability_error(
            &codec.codec,
            format!("unsupported_aprovider_inference_control: does not consume {names}"),
        ));
    }
    if codec.codec == "mlx_openai_chat_v1" {
        if effective.max_output_tokens.is_some_and(|value| value > 512) {
            return Err(invalid_control(
                "max_output_tokens",
                "must be no greater than 512 for mlx_openai_chat_v1",
            ));
        }
        if effective.timeout_secs.is_some_and(|value| value > 120) {
            return Err(invalid_control(
                "timeout_secs",
                "must be no greater than 120 for mlx_openai_chat_v1",
            ));
        }
    }
    if matches!(
        codec.codec.as_str(),
        "anthropic_messages_v1" | "aws_bedrock_invoke_v1"
    ) && effective.temperature.is_some_and(|value| value > 1.0)
    {
        return Err(invalid_control(
            "temperature",
            "must be no greater than 1 for the selected codec",
        ));
    }
    Ok(())
}

fn apply_codec_defaults(
    effective: &mut EffectiveInferenceConfigV1,
    codec: &ProviderCodecControlsV1,
) {
    effective.max_output_tokens = effective.max_output_tokens.or(match codec.codec.as_str() {
        "openai_responses_v1"
        | "anthropic_messages_v1"
        | "deepseek_chat_v1"
        | "kimi_chat_v1"
        | "openrouter_chat_v1"
        | "aws_bedrock_invoke_v1"
        | "z_ai_chat_v1" => Some(220),
        "vertex_gemini_v1" => Some(1024),
        _ => None,
    });
}

fn apply_model_defaults(
    effective: &mut EffectiveInferenceConfigV1,
    codec: &ProviderCodecControlsV1,
    provider_model_id: Option<&str>,
) -> Result<()> {
    if codec.codec == "kimi_chat_v1" && provider_model_id == Some("kimi-k3") {
        match effective.reasoning_effort.as_deref() {
            None => effective.reasoning_effort = Some("max".to_string()),
            Some("low" | "high" | "max") => {}
            Some(_) => {
                return Err(invalid_control(
                    "reasoning_effort",
                    "must be one of low, high, max for kimi-k3",
                ))
            }
        }
    }
    Ok(())
}

fn normalized_effective_inference(
    spec: &adl::ProviderSpec,
    codec: &ProviderCodecControlsV1,
    provider_model_id: Option<&str>,
) -> Result<EffectiveInferenceConfigV1> {
    let mut effective = effective_inference_config_v1(spec)?;
    validate_codec_controls(&effective, codec)?;
    apply_codec_defaults(&mut effective, codec);
    apply_model_defaults(&mut effective, codec, provider_model_id)?;
    Ok(effective)
}

fn redacted_effective_projection(
    provider_id: &str,
    provider_kind: &str,
    transport: &ProviderTransportV1,
    codec_controls: &ProviderCodecControlsV1,
    effective: &EffectiveInferenceConfigV1,
) -> RedactedEffectiveInferenceProjectionV1 {
    RedactedEffectiveInferenceProjectionV1 {
        schema: "adl.aprovider_effective_inference.v1".to_string(),
        provider_id: provider_id.to_string(),
        provider_kind: provider_kind.to_string(),
        transport: transport.clone(),
        codec: codec_controls.codec.clone(),
        consumes: codec_controls.consumes.clone(),
        effective: effective.clone(),
    }
}

fn inference_fingerprint(projection: &RedactedEffectiveInferenceProjectionV1) -> Result<String> {
    let bytes = serde_json::to_vec(projection)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn transport_surface_label(vendor: &str, transport: &ProviderTransportV1) -> &'static str {
    match (vendor, transport) {
        ("ollama", ProviderTransportV1::Http) => "ollama_http",
        ("mlx", ProviderTransportV1::Http) => "mlx_http",
        (_, ProviderTransportV1::Http) => "hosted_http",
        (_, ProviderTransportV1::LocalCli) => "local_cli",
        (_, ProviderTransportV1::InProcess) => "in_process",
    }
}

fn model_identity_strength_for_target(
    vendor: &str,
    transport: &ProviderTransportV1,
) -> ModelIdentityStrengthV1 {
    if vendor == "ollama" || matches!(transport, ProviderTransportV1::LocalCli) {
        ModelIdentityStrengthV1::TagOnly
    } else {
        ModelIdentityStrengthV1::ProviderAsserted
    }
}

fn infer_capability_defaults(
    transport: &ProviderTransportV1,
    vendor: &str,
    model_ref: Option<&str>,
) -> ProviderCapabilitiesV1 {
    let model = model_ref.unwrap_or("").trim().to_lowercase();
    if vendor == "deepgram" {
        return ProviderCapabilitiesV1 {
            tool_calling: unsupported_capability(),
            structured_json: unsupported_capability(),
            semantic_tool_fallback: unsupported_capability(),
            speech_synthesis: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            },
            speech_transcription: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            },
        };
    }
    if vendor == "ollama" {
        let native_supported = model.contains("gpt-oss")
            || model.contains("qwen3-coder")
            || model.contains("qwen2.5-coder");
        let tool_calling = if native_supported {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            }
        } else {
            CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            }
        };
        let structured_json = if tool_calling.supported {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::Native,
            }
        } else {
            CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::PromptBased,
            }
        };
        return ProviderCapabilitiesV1 {
            tool_calling,
            structured_json,
            semantic_tool_fallback: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::SemanticFallback,
            },
            speech_synthesis: unsupported_capability(),
            speech_transcription: unsupported_capability(),
        };
    }

    if matches!(transport, ProviderTransportV1::Http) && matches!(vendor, "generic_http" | "mlx") {
        return ProviderCapabilitiesV1 {
            tool_calling: CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            },
            structured_json: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::PromptBased,
            },
            semantic_tool_fallback: CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            },
            speech_synthesis: unsupported_capability(),
            speech_transcription: unsupported_capability(),
        };
    }

    if matches!(transport, ProviderTransportV1::Http)
        && (vendor == "deepseek"
            || vendor == "kimi"
            || vendor == "openrouter"
            || vendor == "aws_bedrock"
            || vendor == "z_ai")
    {
        return ProviderCapabilitiesV1 {
            tool_calling: CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            },
            structured_json: CapabilitySupportV1 {
                supported: true,
                mode: CapabilityModeV1::PromptBased,
            },
            semantic_tool_fallback: CapabilitySupportV1 {
                supported: false,
                mode: CapabilityModeV1::None,
            },
            speech_synthesis: unsupported_capability(),
            speech_transcription: unsupported_capability(),
        };
    }

    let native_tool_calling = match transport {
        ProviderTransportV1::Http | ProviderTransportV1::InProcess => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        ProviderTransportV1::LocalCli => {
            let native_supported = model.contains("gpt-oss")
                || model.contains("qwen3-coder")
                || model.contains("qwen2.5-coder");
            if native_supported {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::Native,
                }
            } else {
                CapabilitySupportV1 {
                    supported: false,
                    mode: CapabilityModeV1::None,
                }
            }
        }
    };

    let structured_json = match transport {
        ProviderTransportV1::Http | ProviderTransportV1::InProcess => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::Native,
        },
        ProviderTransportV1::LocalCli => {
            if native_tool_calling.supported {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::Native,
                }
            } else {
                CapabilitySupportV1 {
                    supported: true,
                    mode: CapabilityModeV1::PromptBased,
                }
            }
        }
    };

    let semantic_tool_fallback = match transport {
        ProviderTransportV1::LocalCli if vendor == "ollama" => CapabilitySupportV1 {
            supported: true,
            mode: CapabilityModeV1::SemanticFallback,
        },
        _ => CapabilitySupportV1 {
            supported: false,
            mode: CapabilityModeV1::None,
        },
    };

    ProviderCapabilitiesV1 {
        tool_calling: native_tool_calling,
        structured_json,
        semantic_tool_fallback,
        speech_synthesis: unsupported_capability(),
        speech_transcription: unsupported_capability(),
    }
}

fn provider_capabilities_v1(
    spec: &adl::ProviderSpec,
    transport: &ProviderTransportV1,
    vendor: &str,
    model_ref: Option<&str>,
) -> ProviderCapabilitiesV1 {
    let mut caps = infer_capability_defaults(transport, vendor, model_ref);
    if let Some(v) = capability_override(&spec.config, "tool_calling") {
        caps.tool_calling = v;
    }
    if let Some(v) = capability_override(&spec.config, "structured_json") {
        caps.structured_json = v;
    }
    if let Some(v) = capability_override(&spec.config, "semantic_tool_fallback") {
        caps.semantic_tool_fallback = v;
    }
    if let Some(v) = capability_override(&spec.config, "speech_synthesis") {
        caps.speech_synthesis = v;
    }
    if let Some(v) = capability_override(&spec.config, "speech_transcription") {
        caps.speech_transcription = v;
    }
    caps
}

fn default_model_ref(spec: &adl::ProviderSpec) -> Option<String> {
    cfg_str(&spec.config, "model_ref")
        .map(ToString::to_string)
        .or_else(|| spec.default_model.clone())
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
}

fn default_provider_model_id(spec: &adl::ProviderSpec) -> Option<String> {
    cfg_str(&spec.config, "provider_model_id")
        .map(ToString::to_string)
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
        .or_else(|| spec.default_model.clone())
}

pub fn provider_substrate_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
) -> Result<ProviderSubstrateV1> {
    let transport = infer_transport(spec)?;
    let vendor = infer_vendor(spec);
    let provider_kind = normalized_provider_kind(&spec.kind);
    let codec_controls = codec_controls(&provider_kind, &transport, spec);
    let credential_reference = credential_reference_v1(spec, &codec_controls)?;
    let provider_default_model_id = default_provider_model_id(spec);
    let effective_inference = normalized_effective_inference(
        spec,
        &codec_controls,
        provider_default_model_id.as_deref(),
    )?;
    let projection = redacted_effective_projection(
        provider_id,
        &provider_kind,
        &transport,
        &codec_controls,
        &effective_inference,
    );
    let inference_parameter_fingerprint = inference_fingerprint(&projection)?;
    let default_model_ref = default_model_ref(spec);
    Ok(ProviderSubstrateV1 {
        provider_id: provider_id.to_string(),
        provider_kind,
        vendor: vendor.clone(),
        transport: transport.clone(),
        profile: spec.profile.clone(),
        endpoint: cfg_str(&spec.config, "endpoint").map(ToString::to_string),
        base_url: spec.base_url.clone(),
        default_model_ref: default_model_ref.clone(),
        provider_default_model_id,
        capabilities: provider_capabilities_v1(
            spec,
            &transport,
            &vendor,
            default_model_ref.as_deref(),
        ),
        credential_reference,
        codec_controls,
        effective_inference,
        inference_parameter_fingerprint,
    })
}

pub fn provider_invocation_target_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
    model_override: Option<&str>,
) -> Result<ProviderInvocationTargetV1> {
    let substrate = provider_substrate_v1(provider_id, spec)?;
    let transport = substrate.transport.clone();
    let vendor = substrate.vendor.clone();
    if spec.kind.trim() == "mlx"
        && model_override
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .is_none()
        && default_model_ref(spec)
            .filter(|s| !s.trim().is_empty())
            .is_none()
        && default_provider_model_id(spec)
            .filter(|s| !s.trim().is_empty())
            .is_none()
    {
        return Err(anyhow!("mlx requires an explicit model identity"));
    }
    let model_ref = model_override
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToString::to_string)
        .or_else(|| default_model_ref(spec))
        .or_else(|| default_provider_model_id(spec))
        .unwrap_or_else(|| "llama3.1:8b".to_string());

    let provider_model_id = cfg_str(&spec.config, "provider_model_id")
        .map(ToString::to_string)
        .or_else(|| cfg_str(&spec.config, "model").map(ToString::to_string))
        .unwrap_or_else(|| model_ref.clone());
    let effective_inference =
        normalized_effective_inference(spec, &substrate.codec_controls, Some(&provider_model_id))?;
    let target_projection = redacted_effective_projection(
        &substrate.provider_id,
        &substrate.provider_kind,
        &substrate.transport,
        &substrate.codec_controls,
        &effective_inference,
    );
    let target_inference_fingerprint = inference_fingerprint(&target_projection)?;
    let capabilities = provider_capabilities_v1(spec, &transport, &vendor, Some(&model_ref));
    let provider_id = substrate.provider_id.clone();
    let provider_kind = substrate.provider_kind.clone();
    let model_identity = ModelIdentityV1 {
        provider_kind: provider_kind.clone(),
        provider: provider_id.clone(),
        model_ref: model_ref.clone(),
        provider_model_id: provider_model_id.clone(),
        runtime_surface: transport_surface_label(&vendor, &transport).to_string(),
        identity_strength: model_identity_strength_for_target(&vendor, &transport),
        observed_at: observed_at_now_v1(),
        resolved_digest: None,
        source_registry: substrate
            .endpoint
            .clone()
            .or_else(|| substrate.base_url.clone())
            .or_else(|| substrate.profile.clone()),
        runtime_fingerprint: None,
        inference_parameter_fingerprint: Some(target_inference_fingerprint),
        tool_surface: None,
        governance_surface: None,
        evaluator_ref: None,
        lane_ref: None,
        benchmark_ref: None,
    };

    Ok(ProviderInvocationTargetV1 {
        provider_id,
        provider_kind,
        vendor: substrate.vendor,
        transport: substrate.transport,
        profile: substrate.profile,
        endpoint: substrate.endpoint,
        base_url: substrate.base_url,
        model_ref,
        provider_model_id,
        model_identity,
        capabilities,
        credential_reference: substrate.credential_reference,
        codec_controls: substrate.codec_controls,
        effective_inference,
    })
}

/// Return the endpoint-free effective inference projection used for audit
/// output and request fingerprinting.
pub fn redacted_effective_inference_projection_v1(
    provider_id: &str,
    spec: &adl::ProviderSpec,
) -> Result<RedactedEffectiveInferenceProjectionV1> {
    let target = provider_invocation_target_v1(provider_id, spec, None)?;
    Ok(redacted_effective_projection(
        provider_id,
        &target.provider_kind,
        &target.transport,
        &target.codec_controls,
        &target.effective_inference,
    ))
}

pub fn provider_substrate_schema_v1_json() -> Result<String> {
    let schema = schema_for!(ProviderSubstrateV1);
    Ok(serde_json::to_string_pretty(&schema)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn provider_spec(kind: &str) -> adl::ProviderSpec {
        adl::ProviderSpec {
            id: None,
            profile: None,
            kind: kind.to_string(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        }
    }

    #[test]
    fn provider_substrate_separates_http_vendor_and_transport() {
        let mut spec = provider_spec("http");
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.openai.com/v1/complete"),
        );
        spec.default_model = Some("gpt-4.1-mini".to_string());

        let substrate = provider_substrate_v1("openai_primary", &spec).expect("substrate");
        assert_eq!(substrate.provider_id, "openai_primary");
        assert_eq!(substrate.vendor, "openai");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert_eq!(substrate.default_model_ref.as_deref(), Some("gpt-4.1-mini"));
    }

    #[test]
    fn provider_substrate_infers_first_class_claude_profile_vendor() {
        let mut spec = provider_spec("http");
        spec.profile = Some("claude:claude-3-7-sonnet".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("http://127.0.0.1:8787/complete"),
        );
        spec.default_model = Some("claude-3-7-sonnet-latest".to_string());

        let substrate = provider_substrate_v1("claude_primary", &spec).expect("substrate");
        assert_eq!(substrate.provider_id, "claude_primary");
        assert_eq!(substrate.vendor, "anthropic");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert_eq!(
            substrate.default_model_ref.as_deref(),
            Some("claude-3-7-sonnet-latest")
        );
    }

    #[test]
    fn provider_substrate_preserves_expanded_vendor_and_model_identity() {
        for (profile, vendor, model) in [
            ("kimi:k2.5", "kimi", "kimi-k2.5"),
            ("minimax:m2.5", "minimax", "MiniMax-M2.5"),
            ("qwen:qwen3-max", "qwen", "qwen3-max"),
            ("xai:grok-4.5", "xai", "grok-4.5"),
            ("mistral:small-4", "mistral", "mistral-small-4"),
            ("cohere:command-a-plus", "cohere", "command-a-plus"),
        ] {
            let mut spec = provider_spec("http");
            spec.profile = Some(profile.to_string());
            spec.config
                .insert("provider_model_id".to_string(), json!(model));
            let substrate = provider_substrate_v1(profile, &spec).expect("substrate");
            assert_eq!(substrate.vendor, vendor);
            assert_eq!(substrate.provider_default_model_id.as_deref(), Some(model));
            assert_eq!(substrate.transport, ProviderTransportV1::Http);
        }
    }

    #[test]
    fn provider_substrate_accepts_native_openai_anthropic_deepseek_openrouter_and_zai_kinds() {
        let mut openai = provider_spec("openai");
        openai.default_model = Some("gpt-test".to_string());
        let openai_substrate =
            provider_substrate_v1("openai_primary", &openai).expect("openai substrate");
        assert_eq!(openai_substrate.vendor, "openai");
        assert_eq!(openai_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(openai_substrate.provider_kind, "openai");

        let mut anthropic = provider_spec("anthropic");
        anthropic.default_model = Some("claude-test".to_string());
        let anthropic_substrate =
            provider_substrate_v1("anthropic_primary", &anthropic).expect("anthropic substrate");
        assert_eq!(anthropic_substrate.vendor, "anthropic");
        assert_eq!(anthropic_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(anthropic_substrate.provider_kind, "anthropic");

        let mut deepseek = provider_spec("deepseek");
        deepseek.default_model = Some("deepseek-chat".to_string());
        let deepseek_substrate =
            provider_substrate_v1("deepseek_primary", &deepseek).expect("deepseek substrate");
        assert_eq!(deepseek_substrate.vendor, "deepseek");
        assert_eq!(deepseek_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(deepseek_substrate.provider_kind, "deepseek");
        assert!(!deepseek_substrate.capabilities.tool_calling.supported);
        assert_eq!(
            deepseek_substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert_eq!(
            deepseek_substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );

        let mut openrouter = provider_spec("openrouter");
        openrouter.default_model = Some("openai/gpt-4o-mini".to_string());
        let openrouter_substrate =
            provider_substrate_v1("openrouter_primary", &openrouter).expect("openrouter substrate");
        assert_eq!(openrouter_substrate.vendor, "openrouter");
        assert_eq!(openrouter_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(openrouter_substrate.provider_kind, "openrouter");
        assert!(!openrouter_substrate.capabilities.tool_calling.supported);
        assert_eq!(
            openrouter_substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert_eq!(
            openrouter_substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );

        let mut bedrock = provider_spec("bedrock");
        bedrock.default_model = Some("hosted:adl-bedrock:amazon.nova-lite-v1:0".to_string());
        bedrock.config.insert(
            "provider_model_id".to_string(),
            json!("amazon.nova-lite-v1:0"),
        );
        let bedrock_substrate =
            provider_substrate_v1("bedrock_primary", &bedrock).expect("bedrock substrate");
        assert_eq!(bedrock_substrate.vendor, "aws_bedrock");
        assert_eq!(bedrock_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(bedrock_substrate.provider_kind, "bedrock");
        assert!(!bedrock_substrate.capabilities.tool_calling.supported);
        assert_eq!(
            bedrock_substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );

        let bedrock_target =
            provider_invocation_target_v1("bedrock_primary", &bedrock, None).expect("target");
        assert_eq!(
            bedrock_target.model_ref,
            "hosted:adl-bedrock:amazon.nova-lite-v1:0"
        );
        assert_eq!(bedrock_target.provider_model_id, "amazon.nova-lite-v1:0");

        let mut z_ai = provider_spec("z_ai");
        z_ai.default_model = Some("glm-5".to_string());
        let z_ai_substrate = provider_substrate_v1("z_ai_primary", &z_ai).expect("z_ai substrate");
        assert_eq!(z_ai_substrate.vendor, "z_ai");
        assert_eq!(z_ai_substrate.transport, ProviderTransportV1::Http);
        assert_eq!(z_ai_substrate.provider_kind, "z_ai");
        assert!(!z_ai_substrate.capabilities.tool_calling.supported);
        assert_eq!(
            z_ai_substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert_eq!(
            z_ai_substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
    }

    #[test]
    fn provider_substrate_accepts_zai_profile_and_distinct_provider_model_id() {
        let mut spec = provider_spec("z_ai");
        spec.profile = Some("z_ai:glm-5".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://open.bigmodel.cn/api/paas/v4/chat/completions"),
        );
        spec.default_model = Some("hosted:adl-z-ai:glm-5".to_string());
        spec.config
            .insert("provider_model_id".to_string(), json!("glm-5"));

        let substrate = provider_substrate_v1("z_ai_primary", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "z_ai");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert_eq!(
            substrate.provider_default_model_id.as_deref(),
            Some("glm-5")
        );

        let target = provider_invocation_target_v1("z_ai_primary", &spec, None).expect("target");
        assert_eq!(target.model_ref, "hosted:adl-z-ai:glm-5");
        assert_eq!(target.provider_model_id, "glm-5");
        assert_eq!(target.model_identity.provider_kind, "z_ai");
        assert_eq!(target.model_identity.provider_model_id, "glm-5");
        assert_eq!(target.model_identity.runtime_surface, "hosted_http");
    }

    #[test]
    fn provider_substrate_infers_openrouter_endpoint_vendor_and_preserves_model_id() {
        let mut spec = provider_spec("openrouter");
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://openrouter.ai/api/v1/chat/completions"),
        );
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "provider_model_id".to_string(),
            json!("anthropic/claude-3.5-haiku"),
        );

        let substrate = provider_substrate_v1("openrouter_primary", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "openrouter");
        assert_eq!(
            substrate.provider_default_model_id.as_deref(),
            Some("anthropic/claude-3.5-haiku")
        );

        let target =
            provider_invocation_target_v1("openrouter_primary", &spec, None).expect("target");
        assert_eq!(target.model_ref, "reasoning/default");
        assert_eq!(target.provider_model_id, "anthropic/claude-3.5-haiku");
        assert_eq!(target.model_identity.provider_kind, "openrouter");
        assert_eq!(
            target.model_identity.provider_model_id,
            "anthropic/claude-3.5-haiku"
        );
        assert_eq!(target.model_identity.runtime_surface, "hosted_http");
    }

    #[test]
    fn invocation_target_keeps_model_ref_distinct_from_provider_model_id() {
        let mut spec = provider_spec("http");
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "provider_model_id".to_string(),
            json!("gpt-4.1-mini-2026-03-01"),
        );
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.openai.com/v1/complete"),
        );

        let target = provider_invocation_target_v1("p1", &spec, None).expect("target");
        assert_eq!(target.model_ref, "reasoning/default");
        assert_eq!(target.provider_model_id, "gpt-4.1-mini-2026-03-01");
        assert_eq!(target.model_identity.model_ref, "reasoning/default");
        assert_eq!(
            target.model_identity.provider_model_id,
            "gpt-4.1-mini-2026-03-01"
        );
    }

    #[test]
    fn invocation_target_uses_model_override_as_stable_model_ref() {
        let mut spec = provider_spec("ollama");
        spec.config
            .insert("model".to_string(), json!("phi4-mini-provider-native"));

        let target =
            provider_invocation_target_v1("local", &spec, Some("phi4-mini")).expect("target");
        assert_eq!(target.vendor, "ollama");
        assert_eq!(target.transport, ProviderTransportV1::LocalCli);
        assert_eq!(target.model_ref, "phi4-mini");
        assert_eq!(target.provider_model_id, "phi4-mini-provider-native");
    }

    #[test]
    fn provider_substrate_uses_http_transport_for_ollama_with_endpoint() {
        let mut spec = provider_spec("ollama");
        spec.base_url = Some("http://remote_ollama_private_lan:11434".to_string());
        spec.default_model = Some("phi4-mini".to_string());

        let substrate = provider_substrate_v1("remote_ollama", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );

        let target = provider_invocation_target_v1("remote_ollama", &spec, None).expect("target");
        assert_eq!(target.model_identity.provider_kind, "ollama");
        assert_eq!(target.model_identity.runtime_surface, "ollama_http");
        assert_eq!(
            target.model_identity.identity_strength,
            ModelIdentityStrengthV1::TagOnly
        );
    }

    #[test]
    fn provider_substrate_keeps_local_ollama_cli_transport() {
        let mut spec = provider_spec("local_ollama");
        spec.default_model = Some("phi4-mini".to_string());

        let substrate = provider_substrate_v1("local_ollama", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert_eq!(substrate.transport, ProviderTransportV1::LocalCli);
    }

    #[test]
    fn provider_substrate_marks_gpt_oss_ollama_as_tool_capable() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("gpt-oss:latest".to_string());

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert!(substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::Native
        );
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
    }

    #[test]
    fn provider_substrate_marks_deepseek_ollama_for_semantic_fallback() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("deepseek-r1:latest".to_string());

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "ollama");
        assert!(!substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert!(substrate.capabilities.structured_json.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
        assert!(substrate.capabilities.semantic_tool_fallback.supported);
        assert_eq!(
            substrate.capabilities.semantic_tool_fallback.mode,
            CapabilityModeV1::SemanticFallback
        );
    }

    #[test]
    fn provider_substrate_honors_explicit_capability_overrides() {
        let mut spec = provider_spec("ollama");
        spec.default_model = Some("deepseek-r1:latest".to_string());
        spec.config.insert(
            "capabilities".to_string(),
            json!({
                "tool_calling": { "supported": true, "mode": "native" },
                "structured_json": { "supported": true, "mode": "native" },
                "semantic_tool_fallback": { "supported": false, "mode": "none" }
            }),
        );

        let substrate = provider_substrate_v1("local", &spec).expect("substrate");
        assert!(substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::Native
        );
        assert!(!substrate.capabilities.semantic_tool_fallback.supported);
    }

    #[test]
    fn provider_substrate_marks_generic_http_profiles_as_compatibility_by_default() {
        let mut spec = provider_spec("http");
        spec.profile = Some("http:gemini-2.0-flash".to_string());
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.example.invalid/v1/complete"),
        );
        spec.config
            .insert("provider_model_id".to_string(), json!("gemini-2.0-flash"));

        let substrate = provider_substrate_v1("generic_http_profile", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "generic_http");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert!(!substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::None
        );
        assert!(substrate.capabilities.structured_json.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
    }

    #[test]
    fn provider_substrate_allows_explicit_native_override_for_generic_http_profiles() {
        let mut spec = provider_spec("http");
        spec.profile = Some("http:gemini-2.0-flash".to_string());
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://api.example.invalid/v1/complete"),
        );
        spec.config
            .insert("provider_model_id".to_string(), json!("gemini-2.0-flash"));
        spec.config.insert(
            "capabilities".to_string(),
            json!({
                "tool_calling": { "supported": true, "mode": "native" },
                "structured_json": { "supported": true, "mode": "native" }
            }),
        );

        let substrate = provider_substrate_v1("generic_http_profile", &spec).expect("substrate");
        assert!(substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.tool_calling.mode,
            CapabilityModeV1::Native
        );
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::Native
        );
    }

    #[test]
    fn provider_substrate_keeps_http_deepseek_profile_as_compatibility_lane() {
        let mut spec = provider_spec("http");
        spec.profile = Some("http:deepseek-chat".to_string());
        spec.default_model = Some("reasoning/default".to_string());
        spec.config.insert(
            "endpoint".to_string(),
            json!("https://proxy.deepseek.example.com/v1/complete"),
        );
        spec.config
            .insert("provider_model_id".to_string(), json!("deepseek-chat"));

        let substrate = provider_substrate_v1("deepseek_compat", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "generic_http");
        assert_eq!(substrate.provider_kind, "http");
        assert_eq!(substrate.transport, ProviderTransportV1::Http);
        assert!(!substrate.capabilities.tool_calling.supported);
        assert_eq!(
            substrate.capabilities.structured_json.mode,
            CapabilityModeV1::PromptBased
        );
    }

    #[test]
    fn canonical_effective_configuration_is_redacted_stable_and_runtime_capped() {
        let mut spec = provider_spec("ollama");
        spec.base_url = Some("http://127.0.0.1:11434".to_string());
        spec.default_model = Some("qwen3:8b".to_string());
        for (key, value) in [
            ("context_window_tokens", json!(32_768)),
            ("max_output_tokens", json!(1024)),
            ("runtime_max_output_tokens", json!(256)),
            ("temperature", json!(0.2)),
            ("top_p", json!(0.95)),
            ("deterministic_seed", json!(7)),
            ("timeout_secs", json!(30)),
            ("think", json!(true)),
            ("local_keep_alive", json!("-1")),
        ] {
            spec.config.insert(key.to_string(), value);
        }
        let first = provider_invocation_target_v1("resident", &spec, None).unwrap();
        let second = provider_invocation_target_v1("resident", &spec, None).unwrap();
        assert_eq!(first.effective_inference.max_output_tokens, Some(256));
        assert_eq!(
            first.effective_inference.context_window_tokens,
            Some(32_768)
        );
        assert_eq!(
            first.model_identity.inference_parameter_fingerprint,
            second.model_identity.inference_parameter_fingerprint
        );
        let projection = redacted_effective_inference_projection_v1("resident", &spec).unwrap();
        let projection = serde_json::to_value(projection).unwrap();
        assert!(projection.get("endpoint").is_none());
        assert!(projection.get("base_url").is_none());
        assert_eq!(projection["effective"]["local_keep_alive"], json!(-1));
    }

    #[test]
    fn unsupported_invalid_conflicting_and_executable_controls_fail_before_dispatch() {
        let mut local = provider_spec("local_ollama");
        local.default_model = Some("fixture".to_string());
        local.config.insert("temperature".to_string(), json!(0.2));
        let error = provider_invocation_target_v1("local", &local, None).unwrap_err();
        assert!(error
            .to_string()
            .contains("unsupported_aprovider_inference_control"));

        let mut invalid = provider_spec("ollama");
        invalid.base_url = Some("http://127.0.0.1:11434".to_string());
        invalid
            .config
            .insert("context_window_tokens".to_string(), json!(0));
        assert!(provider_invocation_target_v1("invalid", &invalid, None)
            .unwrap_err()
            .to_string()
            .contains("invalid_aprovider_inference_control"));

        let mut conflicting = provider_spec("ollama");
        conflicting.base_url = Some("http://127.0.0.1:11434".to_string());
        conflicting
            .config
            .insert("max_output_tokens".to_string(), json!(100));
        conflicting
            .config
            .insert("max_tokens".to_string(), json!(101));
        assert!(
            provider_invocation_target_v1("conflicting", &conflicting, None)
                .unwrap_err()
                .to_string()
                .contains("conflicting_aprovider_inference_controls")
        );

        for key in [
            "command",
            "script_path",
            "dynamic_library",
            "plugin",
            "plugin_ref",
            "plugin_uri",
            "binary_path",
            "executable_file",
            "library_path",
            "command_path",
            "command_ref",
            "script_ref",
            "dynamic_library_ref",
            "workflow_ref",
            "lifecycle_ref",
            "embedded_code",
            "workflow_authority",
            "lifecycle_authority",
        ] {
            let mut executable = provider_spec("mock");
            executable
                .config
                .insert(key.to_string(), json!("forbidden"));
            assert!(
                provider_invocation_target_v1("executable", &executable, None)
                    .unwrap_err()
                    .to_string()
                    .contains("aprovider_executable_authority_forbidden"),
                "{key}"
            );
        }

        let mut legacy_http = provider_spec("http");
        legacy_http.config.insert(
            "endpoint".to_string(),
            json!("http://127.0.0.1:8765/complete"),
        );
        legacy_http
            .config
            .insert("temperature".to_string(), json!(0.2));
        let error = provider_invocation_target_v1("legacy", &legacy_http, None).unwrap_err();
        assert_eq!(
            crate::provider::failure_category(&error),
            "unsupported_capability"
        );
        assert!(error
            .to_string()
            .contains("unsupported_aprovider_inference_control"));

        legacy_http
            .config
            .insert("api_format".to_string(), json!("openai_chat_completions"));
        let chat_target = provider_invocation_target_v1("chat", &legacy_http, None).unwrap();
        assert_eq!(chat_target.codec_controls.codec, "generic_http_chat_v1");
    }

    #[test]
    fn ollama_profile_and_explicit_definition_share_effective_configuration() {
        let profile = adl::ProviderSpec {
            id: Some("resident".to_string()),
            profile: Some("ollama:phi4-mini".to_string()),
            kind: String::new(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        };
        let expanded = crate::candidate::validate_provider_candidate(&HashMap::from([(
            "resident".to_string(),
            profile,
        )]))
        .unwrap();
        let profile_spec = &expanded["resident"];
        assert_eq!(
            profile_spec.config["endpoint"],
            json!("http://127.0.0.1:11434")
        );
        let profile_target = provider_invocation_target_v1("resident", profile_spec, None).unwrap();

        let mut explicit = provider_spec("ollama");
        explicit.default_model = Some("phi4-mini".to_string());
        for (key, value) in [
            ("endpoint", json!("http://127.0.0.1:11434")),
            ("temperature", json!(0.0)),
            ("top_p", json!(1.0)),
            ("max_output_tokens", json!(512)),
            ("timeout_secs", json!(120)),
            ("deterministic_seed", json!(0)),
        ] {
            explicit.config.insert(key.to_string(), value);
        }
        let explicit_target = provider_invocation_target_v1("resident", &explicit, None).unwrap();
        assert_eq!(
            profile_target.effective_inference,
            explicit_target.effective_inference
        );
        assert_eq!(
            profile_target
                .model_identity
                .inference_parameter_fingerprint,
            explicit_target
                .model_identity
                .inference_parameter_fingerprint
        );
    }

    #[test]
    fn credential_references_are_typed_without_entering_inference_projection() {
        let mut spec = provider_spec("openai");
        spec.default_model = Some("gpt-test".to_string());
        spec.config.insert(
            "auth".to_string(),
            json!({
                "type": "bearer",
                "env": "OPENAI_API_KEY",
                "file_env": "OPENAI_API_KEY_FILE"
            }),
        );
        let target = provider_invocation_target_v1("hosted", &spec, None).unwrap();
        let credential = target.credential_reference.unwrap();
        assert_eq!(credential.strategy, "bearer");
        assert_eq!(credential.environment, "OPENAI_API_KEY");
        assert_eq!(
            credential.file_environment.as_deref(),
            Some("OPENAI_API_KEY_FILE")
        );
        let projection = serde_json::to_string(
            &redacted_effective_inference_projection_v1("hosted", &spec).unwrap(),
        )
        .unwrap();
        assert!(!projection.contains("OPENAI_API_KEY"));
    }

    #[test]
    fn vertex_adc_and_workload_identity_environment_overrides_are_supported() {
        for strategy in ["adc", "workload_identity"] {
            let mut spec = provider_spec("vertex_ai_gemini");
            spec.default_model = Some("gemini-2.5-flash".to_string());
            spec.config.insert(
                "auth".to_string(),
                json!({"type": strategy, "env": "FIXTURE_ACCESS_TOKEN"}),
            );

            let target = provider_invocation_target_v1("vertex", &spec, None).unwrap();
            let credential = target.credential_reference.unwrap();
            assert_eq!(credential.strategy, strategy);
            assert_eq!(credential.environment, "FIXTURE_ACCESS_TOKEN");
            assert_eq!(credential.file_environment, None);
        }
    }

    #[test]
    fn vertex_thinking_controls_are_bound_into_inference_identity() {
        let mut disabled = provider_spec("vertex_ai_gemini");
        disabled.default_model = Some("gemini-2.5-flash".to_string());
        disabled
            .config
            .insert("thinking_budget".to_string(), json!(0));
        disabled
            .config
            .insert("include_thoughts".to_string(), json!(false));
        let mut enabled = disabled.clone();
        enabled
            .config
            .insert("thinking_budget".to_string(), json!(4096));

        let disabled_target = provider_invocation_target_v1("vertex", &disabled, None).unwrap();
        let enabled_target = provider_invocation_target_v1("vertex", &enabled, None).unwrap();
        assert_eq!(disabled_target.effective_inference.thinking_budget, Some(0));
        assert_eq!(
            enabled_target.effective_inference.thinking_budget,
            Some(4096)
        );
        assert_eq!(
            disabled_target.effective_inference.include_thoughts,
            Some(false)
        );
        assert_ne!(
            disabled_target
                .model_identity
                .inference_parameter_fingerprint,
            enabled_target
                .model_identity
                .inference_parameter_fingerprint
        );
    }

    #[test]
    fn built_in_mock_profile_validates_without_phantom_inference_controls() {
        let profile = adl::ProviderSpec {
            id: Some("echo".to_string()),
            profile: Some("mock:echo-v1".to_string()),
            kind: String::new(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        };
        let expanded = crate::candidate::validate_provider_candidate(&HashMap::from([(
            "echo".to_string(),
            profile,
        )]))
        .unwrap();
        let target = provider_invocation_target_v1("echo", &expanded["echo"], None).unwrap();

        assert_eq!(target.codec_controls.codec, "mock_v1");
        assert!(target.codec_controls.consumes.is_empty());
        assert_eq!(
            target.effective_inference,
            EffectiveInferenceConfigV1::default()
        );
    }

    #[test]
    fn deepgram_profile_materializes_only_its_consumed_timeout_control() {
        let profile = adl::ProviderSpec {
            id: Some("speech".to_string()),
            profile: Some("deepgram:nova-3".to_string()),
            kind: String::new(),
            base_url: None,
            default_model: None,
            config: HashMap::from([("timeout_secs".to_string(), json!(45))]),
        };
        let expanded = crate::profiles::expand_provider_profiles(&HashMap::from([(
            "speech".to_string(),
            profile,
        )]))
        .unwrap();
        let target = provider_invocation_target_v1("speech", &expanded["speech"], None).unwrap();

        assert_eq!(target.codec_controls.codec, "deepgram_speech_v1");
        assert_eq!(
            target.codec_controls.consumes,
            vec![InferenceControlV1::TimeoutSecs]
        );
        assert_eq!(target.effective_inference.timeout_secs, Some(45));
        assert_eq!(target.effective_inference.max_output_tokens, None);
        assert_eq!(target.effective_inference.temperature, None);
        assert_eq!(target.effective_inference.top_p, None);
    }

    #[test]
    fn redacted_projection_includes_model_specific_effective_defaults() {
        let mut spec = provider_spec("kimi");
        spec.default_model = Some("kimi-k3".to_string());
        spec.config
            .insert("provider_model_id".to_string(), json!("kimi-k3"));

        let target = provider_invocation_target_v1("reasoner", &spec, None).unwrap();
        let substrate = provider_substrate_v1("reasoner", &spec).unwrap();
        let projection = redacted_effective_inference_projection_v1("reasoner", &spec).unwrap();
        let projection_fingerprint = inference_fingerprint(&projection).unwrap();

        assert_eq!(
            projection.effective.reasoning_effort.as_deref(),
            Some("max")
        );
        assert_eq!(
            target
                .model_identity
                .inference_parameter_fingerprint
                .as_deref(),
            Some(projection_fingerprint.as_str())
        );
        assert_eq!(substrate.effective_inference, target.effective_inference);
        assert_eq!(
            substrate.inference_parameter_fingerprint,
            projection_fingerprint
        );
    }
}
