use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
/// Provider spec: local Ollama, OpenAI, etc.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProviderSpec {
    #[serde(default)]
    pub id: Option<String>,

    /// Optional provider profile id (deterministic in-code preset).
    #[serde(default)]
    pub profile: Option<String>,

    /// Provider type (e.g. "ollama", "openai").
    #[serde(default, rename = "type", alias = "kind")]
    pub kind: String,

    /// Optional base URL.
    #[serde(default)]
    pub base_url: Option<String>,

    /// Optional default model name (provider-specific).
    #[serde(default)]
    pub default_model: Option<String>,

    /// Arbitrary provider config.
    #[serde(default)]
    pub config: HashMap<String, JsonValue>,
}

pub type ProviderMap = HashMap<String, ProviderSpec>;
