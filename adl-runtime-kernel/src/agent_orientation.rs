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
const CAPABILITY_MARKER_PREFIX: &str = "<!-- polis-capability:";
const CAPABILITY_MARKER_SUFFIX: &str = " -->";
const ORIENTATION_CAPABILITY_FAMILIES: [&str; 6] = [
    "identity_office",
    "governed_tools",
    "freedom_gate",
    "adaptive_execution",
    "memory_observability",
    "operator_escalation",
];

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
        let source_path = resolve_orientation_source_path(&config.source_path);
        let content = std::fs::read_to_string(&source_path)
            .or_else(|error| {
                if config.version == DEFAULT_AGENT_ORIENTATION_VERSION
                    && config.source_path == Path::new(DEFAULT_AGENT_ORIENTATION_SOURCE_PATH)
                {
                    Ok(DEFAULT_AGENT_ORIENTATION_BODY.to_owned())
                } else {
                    Err(error)
                }
            })
            .map_err(|error| AgentOrientationError::Read(error.to_string()))?;
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

    pub fn validate_persisted(&self) -> Result<(), AgentOrientationError> {
        validate_resource_shape(self)?;
        let digest = blake3::hash(self.content.as_bytes()).to_hex().to_string();
        if digest == self.digest {
            Ok(())
        } else {
            Err(AgentOrientationError::InvalidContent)
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

fn resolve_orientation_source_path(source_path: &Path) -> PathBuf {
    if source_path.is_absolute() {
        return source_path.to_path_buf();
    }
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .unwrap_or(manifest_dir)
        .join(source_path)
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

fn validate_resource_shape(
    resource: &AgentOrientationResource,
) -> Result<(), AgentOrientationError> {
    if resource.schema != AGENT_ORIENTATION_RESOURCE_SCHEMA
        || resource.digest_algorithm != AGENT_ORIENTATION_DIGEST_ALGORITHM
        || resource.projection != "full"
    {
        return Err(AgentOrientationError::InvalidContent);
    }
    validate_version(&resource.version)?;
    if resource.source_path.trim().is_empty() || resource.source_path.len() > 512 {
        return Err(AgentOrientationError::InvalidSource);
    }
    let digest_is_hex =
        resource.digest.len() == 64 && resource.digest.bytes().all(|byte| byte.is_ascii_hexdigit());
    if !digest_is_hex {
        return Err(AgentOrientationError::InvalidContent);
    }
    validate_package_envelope(&resource.content)
}

fn validate_package_content(content: &str) -> Result<(), AgentOrientationError> {
    validate_package_envelope(content)?;
    validate_capability_inventory(content)?;
    Ok(())
}

// Persisted deliveries predate the machine-readable capability inventory. Their
// schema, version, projection, source and content digest remain authoritative
// provenance, but upgrading the Runtime must not retroactively require markers
// that did not exist when those deliveries were admitted. Candidate and active
// packages still pass `validate_package_content`, including exact inventory
// validation, before they can be delivered to a newly admitted agent.
fn validate_package_envelope(content: &str) -> Result<(), AgentOrientationError> {
    if content.trim().is_empty()
        || content.len() > 128 * 1024
        || !content.contains("Axioma Polis Welcome Package")
        || !content.to_ascii_lowercase().contains("grants no authority")
    {
        return Err(AgentOrientationError::InvalidContent);
    }
    Ok(())
}

fn canonical_orientation_capability_ids() -> Vec<String> {
    let mut ids = crate::REQUIRED_OPERATIONAL_ADAPTERS
        .iter()
        .map(|kind| kind.service_name().to_owned())
        .chain(
            ORIENTATION_CAPABILITY_FAMILIES
                .iter()
                .map(|id| (*id).to_owned()),
        )
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

fn declared_orientation_capability_ids(
    content: &str,
) -> Result<Vec<String>, AgentOrientationError> {
    let mut ids = Vec::new();
    for line in content.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix(CAPABILITY_MARKER_PREFIX) {
            let Some(id) = rest.strip_suffix(CAPABILITY_MARKER_SUFFIX) else {
                return Err(AgentOrientationError::InvalidContent);
            };
            if id.is_empty()
                || !id
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
            {
                return Err(AgentOrientationError::InvalidContent);
            }
            ids.push(id.to_owned());
        }
    }
    ids.sort();
    if ids.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(AgentOrientationError::InvalidContent);
    }
    Ok(ids)
}

fn validate_capability_inventory(content: &str) -> Result<(), AgentOrientationError> {
    if declared_orientation_capability_ids(content)? == canonical_orientation_capability_ids() {
        Ok(())
    } else {
        Err(AgentOrientationError::InvalidContent)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn default_orientation_uses_configured_repo_path_not_cwd_shadow_source() {
        let _guard = CURRENT_DIR_LOCK
            .lock()
            .expect("current directory test lock poisoned");
        let original_dir = std::env::current_dir().expect("current dir resolves");
        let root = tempfile::tempdir().expect("test tempdir");
        let shadow_path = root.path().join(DEFAULT_AGENT_ORIENTATION_SOURCE_PATH);
        std::fs::create_dir_all(shadow_path.parent().expect("shadow parent"))
            .expect("shadow parent directory writes");
        std::fs::write(
            &shadow_path,
            custom_package("Shadow orientation should not load."),
        )
        .expect("shadow source writes");

        std::env::set_current_dir(root.path()).expect("test cwd changes");
        let loaded = AgentOrientationResource::load_from_config(&AgentOrientationConfig::default());
        std::env::set_current_dir(original_dir).expect("test cwd restores");
        let loaded = loaded.expect("default orientation loads");

        assert!(loaded.content.contains("Axioma Polis Welcome Package"));
        assert!(!loaded
            .content
            .contains("Shadow orientation should not load."));
        assert_eq!(loaded, AgentOrientationResource::bundled_default());
    }

    #[test]
    fn configured_default_source_path_loads_the_configured_file() {
        let root = tempfile::tempdir().expect("test tempdir");
        let source_path = root.path().join(DEFAULT_AGENT_ORIENTATION_SOURCE_PATH);
        std::fs::create_dir_all(source_path.parent().expect("source parent"))
            .expect("source parent directory writes");
        std::fs::write(
            &source_path,
            custom_package("Configured default-path orientation."),
        )
        .expect("source writes");

        let loaded = AgentOrientationResource::load_from_config(&AgentOrientationConfig {
            enabled: true,
            version: "v2".to_owned(),
            source_path,
        })
        .expect("configured default-path orientation loads from disk");

        assert_eq!(loaded.version, "v2");
        assert!(loaded
            .content
            .contains("Configured default-path orientation."));
    }

    #[test]
    fn custom_orientation_source_path_still_loads_explicit_package() {
        let root = tempfile::tempdir().expect("test tempdir");
        let source_path = root.path().join("custom-welcome.md");
        std::fs::write(&source_path, custom_package("Explicit custom orientation."))
            .expect("custom source writes");

        let loaded = AgentOrientationResource::load_from_config(&AgentOrientationConfig {
            enabled: true,
            version: "custom-v1".to_owned(),
            source_path,
        })
        .expect("custom orientation loads");

        assert!(loaded.content.contains("Explicit custom orientation."));
        assert_eq!(loaded.version, "custom-v1");
    }

    fn custom_package(body: &str) -> String {
        let inventory = canonical_orientation_capability_ids()
            .into_iter()
            .map(|id| format!("{CAPABILITY_MARKER_PREFIX}{id}{CAPABILITY_MARKER_SUFFIX}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "# Axioma Polis Welcome Package custom\n\nThis package grants no authority by itself.\n\n{inventory}\n\n{body}"
        )
    }

    #[test]
    fn bundled_orientation_matches_canonical_runtime_capability_inventory() {
        let declared = declared_orientation_capability_ids(DEFAULT_AGENT_ORIENTATION_BODY)
            .expect("bundled inventory parses");
        assert_eq!(declared, canonical_orientation_capability_ids());
    }

    #[test]
    fn bundled_orientation_teaches_governed_use_and_deployment_aware_authority() {
        let content = AgentOrientationResource::bundled_default().content;
        for required in [
            "internal Rust modules directly",
            "platform",
            "deployment",
            "admitted for your identity",
            "authorization for the particular action",
            "Freedom Gate",
            "allow, constrain, defer, or refuse",
            "UTS and ACC",
            "ACIP, A2A, and Layer 8",
            "Adaptive Execution Engine",
            "Checkpoints, partials, and continuity",
            "Memory, lifelog, and observability",
            "Scheduling and time",
            "Operator escalation",
        ] {
            assert!(
                content.contains(required),
                "missing orientation: {required}"
            );
        }
        assert!(content.contains("grants no authority"));
        assert!(content.contains("cannot override Runtime policy"));
    }

    #[test]
    fn orientation_rejects_missing_stale_duplicate_and_invented_capabilities() {
        let valid = custom_package("Capability inventory test.");
        let missing = valid.replacen("<!-- polis-capability:freedom_gate -->\n", "", 1);
        assert_eq!(
            AgentOrientationResource::from_content("v1", "welcome.md", missing),
            Err(AgentOrientationError::InvalidContent)
        );

        let invented = valid.replace(
            "<!-- polis-capability:freedom_gate -->",
            "<!-- polis-capability:freedom_gate -->\n<!-- polis-capability:telepathy -->",
        );
        assert_eq!(
            AgentOrientationResource::from_content("v1", "welcome.md", invented),
            Err(AgentOrientationError::InvalidContent)
        );

        let duplicate = valid.replace(
            "<!-- polis-capability:freedom_gate -->",
            "<!-- polis-capability:freedom_gate -->\n<!-- polis-capability:freedom_gate -->",
        );
        assert_eq!(
            AgentOrientationResource::from_content("v1", "welcome.md", duplicate),
            Err(AgentOrientationError::InvalidContent)
        );

        let stale = valid.replace(
            "<!-- polis-capability:freedom_gate -->",
            "<!-- polis-capability:freedom_gate_v0 -->",
        );
        assert_eq!(
            AgentOrientationResource::from_content("v1", "welcome.md", stale),
            Err(AgentOrientationError::InvalidContent)
        );
    }

    #[test]
    fn persisted_pre_inventory_orientation_retains_digest_provenance() {
        let content = "Axioma Polis agent orientation package\n\
Version: v1\n\
Source: docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md\n\
Authority: non-authoritative orientation only.\n\n\
# Axioma Polis Welcome Package v1\n\n\
This package grants no authority by itself.\n";
        let digest = blake3::hash(content.as_bytes()).to_hex().to_string();
        let historical = AgentOrientationResource {
            schema: AGENT_ORIENTATION_RESOURCE_SCHEMA.to_owned(),
            version: "v1".to_owned(),
            digest_algorithm: AGENT_ORIENTATION_DIGEST_ALGORITHM.to_owned(),
            digest,
            source_path: DEFAULT_AGENT_ORIENTATION_SOURCE_PATH.to_owned(),
            projection: "full".to_owned(),
            content: content.to_owned(),
        };

        historical
            .validate_persisted()
            .expect("authenticated pre-inventory delivery remains upgrade-compatible");
        assert_eq!(
            AgentOrientationResource::from_content(
                "v1",
                DEFAULT_AGENT_ORIENTATION_SOURCE_PATH,
                content,
            ),
            Err(AgentOrientationError::InvalidContent),
            "the same legacy content cannot become a new candidate package"
        );

        let mut tampered = historical;
        tampered.content.push_str("tampered");
        assert_eq!(
            tampered.validate_persisted(),
            Err(AgentOrientationError::InvalidContent)
        );
    }
}
