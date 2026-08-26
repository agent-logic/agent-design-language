//! Provider profile presets and expansion helpers.
//!
//! This module maps profile names to deterministic provider defaults and expands
//! ADL documents into explicit provider specs before execution.
use super::*;
use reqwest::Url;
use serde_json::{json, Map};

/// Profile payload used by `provider_profile_registry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderProfilePreset {
    pub(crate) kind: &'static str,
    pub(crate) default_model: Option<&'static str>,
    pub(crate) provider_model_id: Option<&'static str>,
    pub(crate) endpoint: Option<&'static str>,
}

/// Shared bounded inference defaults applied to materialized provider profiles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ProviderInferenceProfilePreset {
    pub(crate) temperature: f64,
    pub(crate) top_p: f64,
    pub(crate) max_output_tokens: u64,
    pub(crate) timeout_secs: u64,
    pub(crate) deterministic_seed: Option<u64>,
}

const DEFAULT_INFERENCE_PROFILE: ProviderInferenceProfilePreset = ProviderInferenceProfilePreset {
    temperature: 0.2,
    top_p: 0.95,
    max_output_tokens: 1024,
    timeout_secs: 120,
    deterministic_seed: None,
};

const DETERMINISTIC_OLLAMA_INFERENCE_PROFILE: ProviderInferenceProfilePreset =
    ProviderInferenceProfilePreset {
        temperature: 0.0,
        top_p: 1.0,
        max_output_tokens: 512,
        timeout_secs: 120,
        deterministic_seed: Some(0),
    };

const PROFILE_STATE_SCHEMA: &str = "adl.provider_profile_state.v1";
const PROFILE_MATERIALIZATION_STATE_SCHEMA: &str = "adl.provider_profile_materialization_state.v1";
const PROFILE_MATERIALIZATION_SCHEMA: &str = "adl.provider_profile_materialization_projection.v1";
const PROFILE_REDACTION_SCHEMA: &str = "adl.provider_profile_redacted_projection.v1";
const MAX_PROFILE_OUTPUT_TOKENS: u64 = 32_768;
const MAX_PROFILE_TIMEOUT_SECS: u64 = 600;

const HTTP_PROFILE_PLACEHOLDER_ENDPOINT: &str = "https://api.example.invalid/v1/complete";
const INVALID_ENDPOINT_HOST_MARKER: &str = "example.invalid";

fn profile_vendor(profile: &str) -> Option<&'static str> {
    match profile.split_once(':').map(|(family, _)| family) {
        Some("kimi") => Some("kimi"),
        Some("minimax") => Some("minimax"),
        Some("qwen") => Some("qwen"),
        Some("xai") => Some("xai"),
        Some("mistral") => Some("mistral"),
        Some("cohere") => Some("cohere"),
        Some("deepseek") => Some("deepseek"),
        Some("z_ai" | "zai" | "zhipu") => Some("z_ai"),
        Some("gemini") => Some("google"),
        Some("chatgpt") => Some("openai"),
        Some("claude") => Some("anthropic"),
        Some("deepgram") => Some("deepgram"),
        _ => None,
    }
}

/// Validate that a profile-provided endpoint is usable and non-placeholder.
pub(crate) fn validate_profile_endpoint(
    provider_id: &str,
    profile_name: &str,
    endpoint: &str,
) -> Result<()> {
    let trimmed = endpoint.trim();
    if trimmed.is_empty()
        || trimmed == HTTP_PROFILE_PLACEHOLDER_ENDPOINT
        || trimmed.contains(INVALID_ENDPOINT_HOST_MARKER)
    {
        return Err(anyhow!(
            "providers.{provider_id}.profile '{}' has placeholder or invalid endpoint; configure providers.{provider_id}.config.endpoint with a real endpoint",
            profile_name
        ));
    }
    if !is_allowed_remote_endpoint(trimmed) {
        return Err(anyhow!(
            "providers.{provider_id}.profile '{}' must use an https:// endpoint; plaintext http:// is only allowed for localhost/loopback test endpoints",
            profile_name
        ));
    }
    Ok(())
}

pub(crate) fn is_allowed_remote_endpoint(endpoint: &str) -> bool {
    let Ok(url) = Url::parse(endpoint.trim()) else {
        return false;
    };
    match url.scheme() {
        "https" => url.host_str().is_some_and(|host| !host.is_empty()),
        "http" => matches!(
            url.host_str(),
            Some("localhost") | Some("127.0.0.1") | Some("[::1]") | Some("::1")
        ),
        _ => false,
    }
}

pub(crate) fn is_allowed_ollama_endpoint(endpoint: &str) -> bool {
    let normalized = endpoint.trim().to_ascii_lowercase();
    normalized.starts_with("https://") || normalized.starts_with("http://")
}

pub(crate) const OPENAI_RESPONSES_ENDPOINT: &str = "https://api.openai.com/v1/responses";
pub(crate) const ANTHROPIC_MESSAGES_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
pub(crate) const DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.deepseek.com/chat/completions";
pub(crate) const OPENROUTER_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://openrouter.ai/api/v1/chat/completions";
pub(crate) const Z_AI_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://open.bigmodel.cn/api/paas/v4/chat/completions";
pub(crate) const KIMI_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.moonshot.ai/v1/chat/completions";
pub(crate) const MINIMAX_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.minimax.io/v1/chat/completions";
pub(crate) const QWEN_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";
pub(crate) const XAI_CHAT_COMPLETIONS_ENDPOINT: &str = "https://api.x.ai/v1/chat/completions";
pub(crate) const MISTRAL_CHAT_COMPLETIONS_ENDPOINT: &str =
    "https://api.mistral.ai/v1/chat/completions";
pub(crate) const COHERE_CHAT_ENDPOINT: &str = "https://api.cohere.com/v2/chat";
pub(crate) const DEEPGRAM_API_ENDPOINT: &str = "https://api.deepgram.com";
/// Canonical Anthropic API version used by the HTTP adapter.
pub(crate) const ANTHROPIC_VERSION: &str = "2023-06-01";

fn inference_profile_for(preset: ProviderProfilePreset) -> ProviderInferenceProfilePreset {
    match preset.kind {
        "ollama" => DETERMINISTIC_OLLAMA_INFERENCE_PROFILE,
        _ => DEFAULT_INFERENCE_PROFILE,
    }
}

fn config_f64(
    provider_id: &str,
    config: &BTreeMap<String, Value>,
    key: &str,
) -> Result<Option<f64>> {
    let Some(value) = config.get(key) else {
        return Ok(None);
    };
    match value {
        Value::Number(number) => number
            .as_f64()
            .map(Some)
            .ok_or_else(|| anyhow!("providers.{provider_id}.config.{key} must be a finite number")),
        _ => Err(anyhow!(
            "providers.{provider_id}.config.{key} must be a finite number"
        )),
    }
}

fn config_u64(
    provider_id: &str,
    config: &BTreeMap<String, Value>,
    key: &str,
) -> Result<Option<u64>> {
    let Some(value) = config.get(key) else {
        return Ok(None);
    };
    match value {
        Value::Number(number) => number.as_u64().map(Some).ok_or_else(|| {
            anyhow!("providers.{provider_id}.config.{key} must be a positive integer")
        }),
        _ => Err(anyhow!(
            "providers.{provider_id}.config.{key} must be a positive integer"
        )),
    }
}

fn validate_bounded_f64(
    provider_id: &str,
    key: &str,
    value: f64,
    min: f64,
    max: f64,
) -> Result<()> {
    if value.is_finite() && value >= min && value <= max {
        Ok(())
    } else {
        Err(anyhow!(
            "providers.{provider_id}.config.{key} must be a finite number in [{min}, {max}]"
        ))
    }
}

fn validate_bounded_u64(provider_id: &str, key: &str, value: u64, max: u64) -> Result<()> {
    if value > 0 && value <= max {
        Ok(())
    } else {
        Err(anyhow!(
            "providers.{provider_id}.config.{key} must be a positive integer no greater than {max}"
        ))
    }
}

fn ensure_inference_profile_config(
    provider_id: &str,
    profile_name: &str,
    preset: ProviderProfilePreset,
    config: &mut BTreeMap<String, Value>,
) -> Result<()> {
    let inference = inference_profile_for(preset);

    let temperature =
        config_f64(provider_id, config, "temperature")?.unwrap_or(inference.temperature);
    validate_bounded_f64(provider_id, "temperature", temperature, 0.0, 2.0)?;
    config
        .entry("temperature".to_string())
        .or_insert_with(|| json!(inference.temperature));

    let top_p = config_f64(provider_id, config, "top_p")?.unwrap_or(inference.top_p);
    validate_bounded_f64(provider_id, "top_p", top_p, 0.0, 1.0)?;
    config
        .entry("top_p".to_string())
        .or_insert_with(|| json!(inference.top_p));

    let max_output_tokens = config_u64(provider_id, config, "max_output_tokens")?
        .unwrap_or(inference.max_output_tokens);
    validate_bounded_u64(
        provider_id,
        "max_output_tokens",
        max_output_tokens,
        MAX_PROFILE_OUTPUT_TOKENS,
    )?;
    config
        .entry("max_output_tokens".to_string())
        .or_insert_with(|| json!(inference.max_output_tokens));

    let timeout_secs =
        config_u64(provider_id, config, "timeout_secs")?.unwrap_or(inference.timeout_secs);
    validate_bounded_u64(
        provider_id,
        "timeout_secs",
        timeout_secs,
        MAX_PROFILE_TIMEOUT_SECS,
    )?;
    config
        .entry("timeout_secs".to_string())
        .or_insert_with(|| json!(inference.timeout_secs));

    if let Some(seed) = inference.deterministic_seed {
        let explicit_seed = config_u64(provider_id, config, "deterministic_seed")?.unwrap_or(seed);
        config
            .entry("deterministic_seed".to_string())
            .or_insert_with(|| json!(seed));
        if preset.kind == "ollama" && explicit_seed != seed {
            return Err(anyhow!(
                "providers.{provider_id}.config.deterministic_seed must remain {seed} for deterministic Ollama profile '{}'",
                profile_name
            ));
        }
    }

    if preset.kind == "ollama" {
        config.insert(
            "materialization_policy".to_string(),
            json!("deterministic_ollama_v1"),
        );
        config.insert(
            "activation_policy".to_string(),
            json!("validate_before_activation"),
        );
    }

    let state = retained_profile_state(profile_name, config.get("profile_state"), &config)?;
    config.insert("profile_state".to_string(), state);
    Ok(())
}

fn retained_profile_state(
    profile_name: &str,
    previous_state: Option<&Value>,
    config: &BTreeMap<String, Value>,
) -> Result<Value> {
    if let Some(previous_state) = previous_state {
        let schema = previous_state
            .get("schema")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if schema != PROFILE_STATE_SCHEMA {
            return Err(anyhow!(
                "providers profile_state must use schema {PROFILE_STATE_SCHEMA}"
            ));
        }
    }
    let previous_lkg = previous_state
        .and_then(Value::as_object)
        .and_then(|state| state.get("last_known_good_profile"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|profile| !profile.is_empty())
        .unwrap_or(profile_name);
    if !provider_profile_registry().contains_key(previous_lkg) {
        return Err(anyhow!(
            "providers profile_state.last_known_good_profile '{}' must name a known profile",
            previous_lkg
        ));
    }
    let previous_materialization = previous_state
        .and_then(Value::as_object)
        .and_then(|state| state.get("last_known_good_materialization"));
    let last_known_good_materialization = match previous_materialization {
        Some(materialization) => {
            validate_materialization_state(previous_lkg, materialization)?;
            materialization.clone()
        }
        None => {
            let retained_preset = provider_profile_registry()
                .get(previous_lkg)
                .copied()
                .ok_or_else(|| {
                    anyhow!(
                        "providers profile_state.last_known_good_profile '{}' must name a known profile",
                        previous_lkg
                    )
                })?;
            materialization_state(previous_lkg, retained_preset, config)
        }
    };

    Ok(json!({
        "schema": PROFILE_STATE_SCHEMA,
        "profile": profile_name,
        "last_known_good_profile": previous_lkg,
        "last_known_good_materialization": last_known_good_materialization,
        "retention": "retain_last_valid_materialization",
        "activation": "validate_before_activation"
    }))
}

fn validate_materialization_state(profile_name: &str, materialization: &Value) -> Result<()> {
    let schema = materialization
        .get("schema")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let materialized_profile = materialization
        .get("profile")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if schema != PROFILE_MATERIALIZATION_STATE_SCHEMA || materialized_profile != profile_name {
        return Err(anyhow!(
            "providers profile_state.last_known_good_materialization must use schema {PROFILE_MATERIALIZATION_STATE_SCHEMA} for last_known_good_profile '{}'",
            profile_name
        ));
    }
    Ok(())
}

fn materialization_state(
    profile_name: &str,
    preset: ProviderProfilePreset,
    config: &BTreeMap<String, Value>,
) -> Value {
    let mut redacted_config = Map::new();
    for (key, value) in config {
        if key == "profile_state" {
            continue;
        }
        redacted_config.insert(key.clone(), redacted_value_for_key(key, value));
    }
    json!({
        "schema": PROFILE_MATERIALIZATION_STATE_SCHEMA,
        "profile": profile_name,
        "type": preset.kind,
        "default_model": preset.default_model,
        "base_url_present": false,
        "config": redacted_config
    })
}

fn materialized_config(
    mut target: HashMap<String, Value>,
    config: BTreeMap<String, Value>,
) -> HashMap<String, Value> {
    target.clear();
    for (key, value) in config {
        target.insert(key, value);
    }
    target
}

fn redacted_value_for_key(key: &str, value: &Value) -> Value {
    if is_private_config_key(key) {
        return Value::String("<redacted>".to_string());
    }
    match value {
        Value::Object(object) => {
            let mut redacted = Map::new();
            for (key, value) in object {
                redacted.insert(key.clone(), redacted_value_for_key(key, value));
            }
            Value::Object(redacted)
        }
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|value| redacted_value_for_key(key, value))
                .collect(),
        ),
        Value::String(_) => Value::String("<redacted>".to_string()),
        Value::Number(_) | Value::Bool(_) | Value::Null => value.clone(),
    }
}

fn is_private_config_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    normalized.contains("auth")
        || normalized.contains("credential")
        || normalized.contains("secret")
        || normalized.contains("token")
        || normalized.contains("key")
        || normalized.contains("recovery")
        || normalized.contains("code")
        || normalized.contains("prompt")
        || normalized.contains("private_payload")
}

/// Build a canonical, redacted materialization projection for provider-profile
/// evidence. Raw ADL provider maps retain their public `HashMap` shape; this is
/// the stable byte boundary for deterministic profile materialization proof.
pub fn provider_profile_materialization_projection(doc: &adl::AdlDoc) -> Result<Value> {
    let expanded = expand_provider_profiles(doc)?;
    let mut providers = Map::new();
    let mut provider_ids: Vec<_> = expanded.providers.keys().cloned().collect();
    provider_ids.sort();

    for provider_id in provider_ids {
        let spec = &expanded.providers[&provider_id];
        let mut config = Map::new();
        let mut config_keys: Vec<_> = spec.config.keys().cloned().collect();
        config_keys.sort();
        for key in config_keys {
            let value = spec
                .config
                .get(&key)
                .expect("sorted key came from materialized config");
            config.insert(key.clone(), redacted_value_for_key(&key, value));
        }
        providers.insert(
            provider_id,
            json!({
                "id": spec.id,
                "profile": spec.profile,
                "type": spec.kind,
                "base_url_present": spec.base_url.is_some(),
                "default_model": spec.default_model,
                "config": config
            }),
        );
    }

    Ok(json!({
        "schema": PROFILE_MATERIALIZATION_SCHEMA,
        "providers": providers
    }))
}

/// Redacted projection suitable for profile evidence and review packets.
pub fn redacted_provider_profile_projection(provider_id: &str, spec: &adl::ProviderSpec) -> Value {
    let mut config = Map::new();
    for (key, value) in &spec.config {
        config.insert(key.clone(), redacted_value_for_key(key, value));
    }

    json!({
        "schema": PROFILE_REDACTION_SCHEMA,
        "provider_id": provider_id,
        "profile": spec.profile,
        "type": spec.kind,
        "default_model": spec.default_model,
        "base_url_present": spec.base_url.is_some(),
        "config": config
    })
}

pub(crate) fn provider_profile_registry() -> BTreeMap<&'static str, ProviderProfilePreset> {
    let mut m = BTreeMap::new();
    // Ollama / local presets
    m.insert(
        "ollama:phi4-mini",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("phi4-mini"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:qwen2.5-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("qwen2.5:7b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:llama3.1-8b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("llama3.1:8b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    m.insert(
        "ollama:mistral-7b",
        ProviderProfilePreset {
            kind: "ollama",
            default_model: Some("mistral:7b"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    // Mock/testing preset
    m.insert(
        "mock:echo-v1",
        ProviderProfilePreset {
            kind: "mock",
            default_model: Some("echo-v1"),
            provider_model_id: None,
            endpoint: None,
        },
    );
    // AWS Bedrock hosted presets.
    for (name, stable_ref, provider_model_id) in [
        (
            "bedrock:nova-lite-v1",
            "hosted:adl-bedrock:amazon.nova-lite-v1:0",
            "amazon.nova-lite-v1:0",
        ),
        (
            "bedrock:nova-pro-v1",
            "hosted:adl-bedrock:us.amazon.nova-pro-v1:0",
            "us.amazon.nova-pro-v1:0",
        ),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "bedrock",
                default_model: Some(stable_ref),
                provider_model_id: Some(provider_model_id),
                endpoint: None,
            },
        );
    }
    m.insert(
        "z_ai:glm-5",
        ProviderProfilePreset {
            kind: "z_ai",
            default_model: Some("hosted:adl-z-ai:glm-5"),
            provider_model_id: Some("glm-5"),
            endpoint: Some(Z_AI_CHAT_COMPLETIONS_ENDPOINT),
        },
    );
    for (name, model) in [
        ("deepgram:aura-2-pluto-en", "aura-2-pluto-en"),
        ("deepgram:nova-3", "nova-3"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "deepgram",
                default_model: Some(model),
                provider_model_id: Some(model),
                endpoint: Some(DEEPGRAM_API_ENDPOINT),
            },
        );
    }
    // First-class hosted provider identities. These profiles intentionally
    // share the bounded HTTP transport while retaining vendor/model identity.
    for (name, model, endpoint) in [
        ("kimi:k2.5", "kimi-k2.5", KIMI_CHAT_COMPLETIONS_ENDPOINT),
        (
            "minimax:m2.5",
            "MiniMax-M2.5",
            MINIMAX_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "qwen:qwen3-max",
            "qwen3-max",
            QWEN_CHAT_COMPLETIONS_ENDPOINT,
        ),
        ("xai:grok-4.5", "grok-4.5", XAI_CHAT_COMPLETIONS_ENDPOINT),
        (
            "mistral:medium-3.5",
            "mistral-medium-3.5",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "mistral:small-4",
            "mistral-small-4",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "mistral:devstral-2",
            "devstral-2",
            MISTRAL_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "cohere:command-a-plus",
            "command-a-plus",
            COHERE_CHAT_ENDPOINT,
        ),
        (
            "cohere:north-mini-code",
            "north-mini-code",
            COHERE_CHAT_ENDPOINT,
        ),
        (
            "deepseek:v4",
            "deepseek-v4",
            DEEPSEEK_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "z_ai:glm-5-current",
            "glm-5",
            Z_AI_CHAT_COMPLETIONS_ENDPOINT,
        ),
        (
            "gemini:3.1-pro-preview",
            "gemini-3.1-pro-preview",
            "https://generativelanguage.googleapis.com/v1beta/models",
        ),
        (
            "gemini:3.1-flash-lite",
            "gemini-3.1-flash-lite",
            "https://generativelanguage.googleapis.com/v1beta/models",
        ),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: Some(model),
                endpoint: Some(endpoint),
            },
        );
    }
    // HTTP presets (explicit fixed endpoint placeholders; no secrets)
    for (name, model) in [
        ("http:gpt-4o-mini", "gpt-4o-mini"),
        ("http:gpt-4.1-mini", "gpt-4.1-mini"),
        ("http:claude-3-5-haiku", "claude-3-5-haiku-latest"),
        ("http:claude-3-7-sonnet", "claude-3-7-sonnet-latest"),
        ("http:gemini-2.0-flash", "gemini-2.0-flash"),
        ("http:gemini-2.5-flash", "gemini-2.5-flash"),
        ("http:deepseek-chat", "deepseek-chat"),
        ("http:llama-3.3-70b", "llama-3.3-70b-instruct"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    // ChatGPT-facing presets (same bounded HTTP substrate, distinct profile family)
    for (name, model) in [
        ("chatgpt:gpt-5.4", "gpt-5.4"),
        ("chatgpt:gpt-5.4-mini", "gpt-5.4-mini"),
        ("chatgpt:gpt-5.3-codex", "gpt-5.3-codex"),
        ("chatgpt:gpt-5.2", "gpt-5.2"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    // Claude-facing presets (same bounded HTTP substrate, distinct profile family)
    m.insert(
        "claude:claude-opus-5",
        ProviderProfilePreset {
            kind: "anthropic",
            default_model: Some("claude-opus-5"),
            provider_model_id: Some("claude-opus-5"),
            endpoint: Some(ANTHROPIC_MESSAGES_ENDPOINT),
        },
    );
    for (name, model) in [
        ("claude:claude-3-7-sonnet", "claude-3-7-sonnet-latest"),
        ("claude:claude-3-5-haiku", "claude-3-5-haiku-latest"),
    ] {
        m.insert(
            name,
            ProviderProfilePreset {
                kind: "http",
                default_model: Some(model),
                provider_model_id: None,
                endpoint: Some(HTTP_PROFILE_PLACEHOLDER_ENDPOINT),
            },
        );
    }
    m
}

/// Return available profile names for validation and command completions.
pub fn provider_profile_names() -> Vec<String> {
    provider_profile_registry()
        .keys()
        .map(|name| (*name).to_string())
        .collect()
}

/// Expand provider profiles in an ADL document into explicit concrete specs.
///
/// This is a bounded transform: it expands profile-only providers while keeping
/// explicit `kind`/`base_url`/`default_model` usage unchanged.
pub fn expand_provider_profiles(doc: &adl::AdlDoc) -> Result<adl::AdlDoc> {
    let registry = provider_profile_registry();
    let available = provider_profile_names().join(", ");
    let mut expanded = doc.clone();
    let mut provider_ids: Vec<String> = expanded.providers.keys().cloned().collect();
    provider_ids.sort();

    for provider_id in provider_ids {
        let Some(spec) = expanded.providers.get(&provider_id).cloned() else {
            continue;
        };
        let Some(profile_name_raw) = spec.profile.as_deref() else {
            continue;
        };

        if !spec.kind.trim().is_empty() || spec.base_url.is_some() || spec.default_model.is_some() {
            return Err(anyhow!(
                "providers.{provider_id} uses profile and explicit provider identity fields together (remove type/base_url/default_model when profile is set; config remains available for bounded compatibility overrides)"
            ));
        }

        let profile_name = profile_name_raw.trim();
        let Some(preset) = registry.get(profile_name) else {
            return Err(anyhow!(
                "providers.{provider_id}.profile '{}' is unknown (available: {})",
                profile_name,
                available
            ));
        };

        let mut config: BTreeMap<String, Value> = spec.config.clone().into_iter().collect();
        if let Some(explicit) = config.get("vendor") {
            let Some(explicit) = explicit.as_str() else {
                return Err(anyhow!(
                    "providers.{provider_id}.config.vendor must be a string"
                ));
            };
            if let Some(expected) = profile_vendor(profile_name) {
                let normalized = explicit.trim().to_ascii_lowercase();
                if normalized != expected {
                    return Err(anyhow!(
                        "providers.{provider_id}.config.vendor '{}' conflicts with profile vendor '{}'",
                        explicit.trim(),
                        expected
                    ));
                }
            }
        }

        ensure_inference_profile_config(&provider_id, profile_name, *preset, &mut config)?;
        if let Some(provider_model_id) = preset.provider_model_id.or(preset.default_model) {
            if let Some(explicit) = config.get("provider_model_id") {
                let Some(explicit) = explicit.as_str() else {
                    return Err(anyhow!(
                        "providers.{provider_id}.config.provider_model_id must be a string"
                    ));
                };
                if explicit.trim() != provider_model_id {
                    return Err(anyhow!(
                        "providers.{provider_id}.config.provider_model_id '{}' conflicts with profile model '{}'",
                        explicit.trim(),
                        provider_model_id
                    ));
                }
            }
            config.insert(
                "provider_model_id".to_string(),
                Value::String(provider_model_id.to_string()),
            );
        }
        if let Some(endpoint) = preset.endpoint {
            match config.get("endpoint") {
                Some(explicit) => {
                    let Some(explicit) = explicit.as_str() else {
                        return Err(anyhow!(
                            "providers.{provider_id}.config.endpoint must be a string"
                        ));
                    };
                    validate_profile_endpoint(&provider_id, profile_name, explicit)?;
                }
                None => {
                    validate_profile_endpoint(&provider_id, profile_name, endpoint)?;
                    config.insert("endpoint".to_string(), Value::String(endpoint.to_string()));
                }
            }
        }
        expanded.providers.insert(
            provider_id,
            adl::ProviderSpec {
                id: spec.id.clone(),
                profile: Some(profile_name.to_string()),
                kind: preset.kind.to_string(),
                base_url: None,
                default_model: preset.default_model.map(|m| m.to_string()),
                config: materialized_config(spec.config.clone(), config),
            },
        );
    }
    Ok(expanded)
}
