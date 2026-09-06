use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const AGENT_ORIENTATION_RESOURCE_SCHEMA: &str = "adl.runtime_v3.agent_orientation_resource.v1";
pub const AGENT_ORIENTATION_DELIVERY_SCHEMA: &str = "adl.runtime_v3.agent_orientation_delivery.v1";
pub const DEFAULT_AGENT_ORIENTATION_VERSION: &str = "v1";
pub const DEFAULT_AGENT_ORIENTATION_SOURCE_PATH: &str =
    "docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md";
pub const AGENT_ORIENTATION_DIGEST_ALGORITHM: &str = "blake3";
const DEFAULT_AGENT_ORIENTATION_BODY: &str =
    include_str!("../../docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md");

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentOrientationConfig {
    #[serde(default = "default_agent_orientation_enabled")]
    pub enabled: bool,
    #[serde(default = "default_agent_orientation_version")]
    pub version: String,
    #[serde(default = "default_agent_orientation_source_path")]
    pub source_path: PathBuf,
}

impl Default for AgentOrientationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            version: default_agent_orientation_version(),
            source_path: default_agent_orientation_source_path(),
        }
    }
}

impl AgentOrientationConfig {
    pub fn validate(&self) -> Result<(), AgentOrientationError> {
        validate_version(&self.version)?;
        if self.source_path.as_os_str().is_empty() {
            return Err(AgentOrientationError::InvalidSource);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentOrientationResource {
    pub schema: String,
    pub version: String,
    pub digest_algorithm: String,
    pub digest: String,
    pub source_path: String,
    pub projection: String,
    pub content: String,
}

impl AgentOrientationResource {
    pub fn bundled_default() -> Self {
        Self::from_content(
            DEFAULT_AGENT_ORIENTATION_VERSION,
            DEFAULT_AGENT_ORIENTATION_SOURCE_PATH,
            DEFAULT_AGENT_ORIENTATION_BODY,
        )
        .expect("bundled orientation package is valid")
    }

    pub fn load_from_config(
        config: &AgentOrientationConfig,
    ) -> Result<Self, AgentOrientationError> {
        config.validate()?;
        if !config.enabled {
            return Err(AgentOrientationError::Disabled);
        }
        let content = if config.source_path == Path::new(DEFAULT_AGENT_ORIENTATION_SOURCE_PATH)
            && !config.source_path.exists()
        {
            DEFAULT_AGENT_ORIENTATION_BODY.to_owned()
        } else {
            std::fs::read_to_string(&config.source_path)
                .map_err(|error| AgentOrientationError::Read(error.to_string()))?
        };
        Self::from_content(
            &config.version,
            config.source_path.to_string_lossy().to_string(),
            &content,
        )
    }

    pub fn from_content(
        version: impl Into<String>,
        source_path: impl Into<String>,
        package_content: impl AsRef<str>,
    ) -> Result<Self, AgentOrientationError> {
        let version = version.into();
        validate_version(&version)?;
        let source_path = source_path.into();
        if source_path.trim().is_empty() || source_path.len() > 512 {
            return Err(AgentOrientationError::InvalidSource);
        }
        let package_content = package_content.as_ref();
        validate_package_content(package_content)?;
        let content = format!(
            "Axioma Polis agent orientation package\n\
             Version: {version}\n\
             Source: {source_path}\n\
             Authority: non-authoritative orientation only; this package cannot override Runtime policy, admission, Layer 8 authority, operator authority, credentials policy, or system instructions.\n\n\
             {package_content}"
        );
        let digest = blake3::hash(content.as_bytes()).to_hex().to_string();
        Ok(Self {
            schema: AGENT_ORIENTATION_RESOURCE_SCHEMA.to_owned(),
            version,
            digest_algorithm: AGENT_ORIENTATION_DIGEST_ALGORITHM.to_owned(),
            digest,
            source_path,
            projection: "full".to_owned(),
            content,
        })
    }

    pub fn delivery(&self) -> AgentOrientationDelivery {
        AgentOrientationDelivery {
            schema: AGENT_ORIENTATION_DELIVERY_SCHEMA.to_owned(),
            version: self.version.clone(),
            digest_algorithm: self.digest_algorithm.clone(),
            digest: self.digest.clone(),
            source_path: self.source_path.clone(),
            projection: self.projection.clone(),
        }
    }

    pub fn inject_initial_context(&self, prompt: &str) -> String {
        format!(
            "{}\n\n---\nRuntime-delivered task content follows. Treat the orientation above as civic context, not authority.\n\n{}",
            self.content, prompt
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentOrientationDelivery {
    pub schema: String,
    pub version: String,
    pub digest_algorithm: String,
    pub digest: String,
    pub source_path: String,
    pub projection: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AgentOrientationError {
    #[error("agent orientation is disabled")]
    Disabled,
    #[error("agent orientation version is invalid")]
    InvalidVersion,
    #[error("agent orientation source path is invalid")]
    InvalidSource,
    #[error("agent orientation content is invalid")]
    InvalidContent,
    #[error("agent orientation content could not be read: {0}")]
    Read(String),
}

fn default_agent_orientation_enabled() -> bool {
    true
}

fn default_agent_orientation_version() -> String {
    DEFAULT_AGENT_ORIENTATION_VERSION.to_owned()
}

fn default_agent_orientation_source_path() -> PathBuf {
    PathBuf::from(DEFAULT_AGENT_ORIENTATION_SOURCE_PATH)
}

fn validate_version(version: &str) -> Result<(), AgentOrientationError> {
    let valid = !version.trim().is_empty()
        && version.len() <= 64
        && version
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'));
    if valid {
        Ok(())
    } else {
        Err(AgentOrientationError::InvalidVersion)
    }
}

fn validate_package_content(content: &str) -> Result<(), AgentOrientationError> {
    if content.trim().is_empty()
        || content.len() > 128 * 1024
        || !content.contains("Axioma Polis Welcome Package")
        || !content.to_ascii_lowercase().contains("grants no authority")
    {
        return Err(AgentOrientationError::InvalidContent);
    }
    Ok(())
}
