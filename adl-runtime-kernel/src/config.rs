use std::{
    collections::{BTreeMap, BTreeSet},
    net::{SocketAddr, ToSocketAddrs},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    candidate_digest, BirthWitnessAttestation, BirthWitnessError, BirthWitnessPacket,
    BirthWitnessRole, BirthdayCandidate, BirthdayDecision, ComponentId,
    RuntimeBirthWitnessAuthority, RuntimeBirthWitnessService, VerifiedBirthWitnessBinding,
};

pub const RUNTIME_CONFIG_SCHEMA: &str = "adl.runtime.config.v1";
pub const RUNTIME_INIT_SCHEMA: &str = "adl.runtime_v3.init.v1";
const MAX_RUNTIME_INIT_MILLIS: u64 = 600_000;
const MAX_RUNTIME_INIT_CAPACITY: usize = 1_000_000;
const MAX_GUARDIAN_RESTART_BUDGET: u32 = 10_000;
const MAX_OBSERVABILITY_FILE_BYTES: u64 = 1024 * 1024 * 1024;
const MAX_OBSERVABILITY_RETAINED_FILES: usize = 128;
const MAX_GUARDIAN_CONFIGURATION_EXIT_CODES: usize = 16;
const MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS: u32 = 32;

const BIRTH_WITNESS_TRUST_SCHEMA: &str = "adl.runtime.birth_witness_trust.v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeBirthWitnessTrustManifest {
    schema: String,
    authority_context: String,
    authorities: Vec<RuntimeBirthWitnessTrustAuthority>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RuntimeBirthWitnessTrustAuthority {
    witness_id: String,
    role: BirthWitnessRole,
    signing_key_id: String,
    verifying_key: String,
}

#[derive(Clone, Debug)]
struct RuntimeBirthWitnessTrust {
    authority_context: String,
    authorities: Vec<RuntimeBirthWitnessAuthority>,
}

impl RuntimeBirthWitnessTrust {
    fn provision(
        &self,
        candidate_sha256: impl Into<String>,
        current_generation: u64,
    ) -> Result<RuntimeBirthWitnessService, BirthWitnessError> {
        RuntimeBirthWitnessService::provision(
            self.authority_context.clone(),
            candidate_sha256,
            current_generation,
            self.authorities.clone(),
        )
    }
}

/// Runtime-owned operator for governed birth-witness receipt production.
///
/// Trust roots are sealed inside validated boot configuration. Callers supply
/// only the candidate, decision, attestations, generation, and receipt sink.
#[derive(Clone, Debug)]
pub struct RuntimeBirthWitnessOwner {
    trust: RuntimeBirthWitnessTrust,
}

impl RuntimeBirthWitnessOwner {
    pub fn roster_sha256(&self) -> Result<String, BirthWitnessError> {
        Ok(self
            .trust
            .provision("0".repeat(64), 1)?
            .roster_sha256()
            .to_owned())
    }

    pub fn build_validate_and_emit<F, C>(
        &self,
        candidate: &BirthdayCandidate,
        decision: &BirthdayDecision,
        current_generation: u64,
        attestations: &[BirthWitnessAttestation],
        prepare_receipt: F,
    ) -> Result<BirthWitnessPacket, BirthWitnessError>
    where
        F: FnOnce(&[u8]) -> Result<C, ()>,
        C: FnOnce(),
    {
        let candidate_sha256 =
            candidate_digest(candidate).map_err(|_| BirthWitnessError::Encoding)?;
        self.trust
            .provision(candidate_sha256, current_generation)?
            .build_validate_and_emit(candidate, decision, attestations, prepare_receipt)
    }

    pub fn build_validate_and_emit_verified<F, C>(
        &self,
        candidate: &BirthdayCandidate,
        decision: &BirthdayDecision,
        current_generation: u64,
        attestations: &[BirthWitnessAttestation],
        prepare_receipt: F,
    ) -> Result<VerifiedBirthWitnessBinding, BirthWitnessError>
    where
        F: FnOnce(&[u8]) -> Result<C, ()>,
        C: FnOnce(),
    {
        self.build_validate_and_emit(
            candidate,
            decision,
            current_generation,
            attestations,
            prepare_receipt,
        )
        .map(|packet| VerifiedBirthWitnessBinding {
            packet,
            observed_generation: current_generation,
        })
    }
}

fn load_runtime_birth_witness_trust(
    trusted_manifest_path: &Path,
) -> Result<RuntimeBirthWitnessTrust, RuntimeInitError> {
    let bytes = std::fs::read(trusted_manifest_path).map_err(|error| {
        RuntimeInitError::Read(trusted_manifest_path.to_path_buf(), error.to_string())
    })?;
    let manifest: RuntimeBirthWitnessTrustManifest = serde_json::from_slice(&bytes)
        .map_err(|error| RuntimeInitError::Policy(error.to_string()))?;
    if manifest.schema != BIRTH_WITNESS_TRUST_SCHEMA {
        return Err(RuntimeInitError::Policy(
            "unsupported birth-witness trust manifest schema".to_owned(),
        ));
    }
    let authorities = manifest
        .authorities
        .into_iter()
        .map(|authority| {
            let key = hex::decode(authority.verifying_key)
                .map_err(|_| RuntimeInitError::Policy("invalid birth-witness key".to_owned()))?;
            let verifying_key: [u8; 32] = key.try_into().map_err(|_| {
                RuntimeInitError::Policy("invalid birth-witness key length".to_owned())
            })?;
            Ok(RuntimeBirthWitnessAuthority {
                witness_id: authority.witness_id,
                role: authority.role,
                signing_key_id: authority.signing_key_id,
                verifying_key,
            })
        })
        .collect::<Result<Vec<_>, RuntimeInitError>>()?;
    let trust = RuntimeBirthWitnessTrust {
        authority_context: manifest.authority_context,
        authorities,
    };
    trust
        .provision("0".repeat(64), 1)
        .map_err(|error| RuntimeInitError::Policy(format!("birth-witness trust: {error}")))?;
    Ok(trust)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CanonicalValue {
    Bool(bool),
    Integer(i64),
    Text(String),
    List(Vec<CanonicalValue>),
    Map(BTreeMap<String, CanonicalValue>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentConfig {
    pub id: ComponentId,
    pub factory: String,
    #[serde(default)]
    pub dependencies: Vec<ComponentId>,
    #[serde(default)]
    pub parameters: BTreeMap<String, CanonicalValue>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WeatherConfig {
    pub sample_millis: u64,
    pub history_capacity: usize,
    pub disk_warning_free_bytes: u64,
    pub disk_stop_free_bytes: u64,
    pub disk_recover_free_bytes: u64,
    pub memory_warning_used_basis_points: u16,
    pub memory_stop_used_basis_points: u16,
    pub memory_recover_used_basis_points: u16,
    pub cpu_warning_basis_points: u16,
    pub cpu_stop_basis_points: u16,
    pub cpu_recover_basis_points: u16,
    pub checkpoint_deadline_millis: u64,
    pub snapshot_concurrency: usize,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            sample_millis: 1_000,
            history_capacity: 60,
            disk_warning_free_bytes: 5 * 1024 * 1024 * 1024,
            disk_stop_free_bytes: 2 * 1024 * 1024 * 1024,
            disk_recover_free_bytes: 8 * 1024 * 1024 * 1024,
            memory_warning_used_basis_points: 8_500,
            memory_stop_used_basis_points: 9_500,
            memory_recover_used_basis_points: 7_500,
            cpu_warning_basis_points: 9_000,
            cpu_stop_basis_points: 9_800,
            cpu_recover_basis_points: 8_000,
            checkpoint_deadline_millis: 5_000,
            snapshot_concurrency: 4,
        }
    }
}

impl WeatherConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.sample_millis == 0
            || self.history_capacity == 0
            || self.checkpoint_deadline_millis == 0
            || self.snapshot_concurrency == 0
        {
            return Err(ConfigError::ZeroBound);
        }
        if !(self.disk_stop_free_bytes < self.disk_warning_free_bytes
            && self.disk_warning_free_bytes < self.disk_recover_free_bytes)
        {
            return Err(ConfigError::ThresholdOrder("disk"));
        }
        validate_high_thresholds(
            "memory",
            self.memory_recover_used_basis_points,
            self.memory_warning_used_basis_points,
            self.memory_stop_used_basis_points,
        )?;
        validate_high_thresholds(
            "cpu",
            self.cpu_recover_basis_points,
            self.cpu_warning_basis_points,
            self.cpu_stop_basis_points,
        )
    }
}

fn validate_high_thresholds(
    resource: &'static str,
    recover: u16,
    warning: u16,
    stop: u16,
) -> Result<(), ConfigError> {
    if stop > 10_000 || !(recover < warning && warning < stop) {
        return Err(ConfigError::ThresholdOrder(resource));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfig {
    pub schema: String,
    #[serde(default)]
    pub weather: WeatherConfig,
    pub components: Vec<ComponentConfig>,
}

impl RuntimeConfig {
    pub fn from_json(bytes: &[u8]) -> Result<Self, ConfigError> {
        let value: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|error| ConfigError::Json(error.to_string()))?;
        let schema = value
            .get("schema")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        if schema != RUNTIME_CONFIG_SCHEMA {
            return Err(ConfigError::UnsupportedSchema(schema.to_owned()));
        }
        let config: Self =
            serde_json::from_value(value).map_err(|error| ConfigError::Json(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema != RUNTIME_CONFIG_SCHEMA {
            return Err(ConfigError::UnsupportedSchema(self.schema.clone()));
        }
        self.weather.validate()?;
        let mut ids = BTreeSet::new();
        for component in &self.components {
            if component.id.as_str().trim().is_empty() || component.factory.trim().is_empty() {
                return Err(ConfigError::EmptyIdentity);
            }
            if !ids.insert(component.id.clone()) {
                return Err(ConfigError::DuplicateComponent(component.id.clone()));
            }
            let dependencies = component.dependencies.iter().collect::<BTreeSet<_>>();
            if dependencies.len() != component.dependencies.len()
                || dependencies.contains(&component.id)
            {
                return Err(ConfigError::InvalidDependencies(component.id.clone()));
            }
            for key in component.parameters.keys() {
                let normalized = key.to_ascii_lowercase();
                if ["secret", "password", "token", "credential", "api_key"]
                    .iter()
                    .any(|term| normalized.contains(term))
                {
                    return Err(ConfigError::SecretInCanonicalConfig {
                        component: component.id.clone(),
                        key: key.clone(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn canonical_json(&self) -> Result<String, ConfigError> {
        self.validate()?;
        let mut effective = self.clone();
        effective
            .components
            .sort_by(|left, right| left.id.cmp(&right.id));
        for component in &mut effective.components {
            component.dependencies.sort();
        }
        serde_json::to_string(&effective).map_err(|error| ConfigError::Json(error.to_string()))
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ConfigError {
    #[error("unsupported runtime configuration schema: {0}")]
    UnsupportedSchema(String),
    #[error("invalid runtime configuration JSON: {0}")]
    Json(String),
    #[error("component identity and factory names must be non-empty")]
    EmptyIdentity,
    #[error("duplicate configured component: {0}")]
    DuplicateComponent(ComponentId),
    #[error("component has duplicate or self dependencies: {0}")]
    InvalidDependencies(ComponentId),
    #[error("canonical configuration cannot contain secret field {component}.{key}")]
    SecretInCanonicalConfig { component: ComponentId, key: String },
    #[error("resource thresholds are not ordered for {0}")]
    ThresholdOrder(&'static str),
    #[error("sampling, history, checkpoint, and concurrency bounds must be non-zero")]
    ZeroBound,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInitConfig {
    pub schema: String,
    pub state_root: PathBuf,
    pub binaries: RuntimeBinariesInitConfig,
    pub paths: RuntimePathsInitConfig,
    pub api: RuntimeApiInitConfig,
    pub polis: PolisInitConfig,
    pub resident_shepherd: ResidentShepherdSetInitConfig,
    pub kernel: RuntimeKernelInitConfig,
    #[serde(default)]
    pub continuity_control: Option<crate::ContinuityControlInitConfig>,
    pub credentials: RuntimeCredentialInitConfig,
    pub shutdown: RuntimeShutdownInitConfig,
    #[serde(default)]
    pub service_convergence: RuntimeServiceConvergenceInitConfig,
    pub guardian: RuntimeGuardianInitConfig,
    pub qualification: RuntimeQualificationInitConfig,
    pub observatory: ObservatoryInitConfig,
    pub observability_pipeline: RuntimeObservabilityInitConfig,
    #[serde(default)]
    pub agent_partial_checkpoints: AgentPartialCheckpointInitConfig,
    pub weather: WeatherConfig,
}

impl RuntimeInitConfig {
    pub fn load(path: Option<PathBuf>) -> Result<Self, RuntimeInitError> {
        let path = path.ok_or(RuntimeInitError::MissingInitFile)?;
        Self::from_path(path)
    }

    pub fn from_path(path: PathBuf) -> Result<Self, RuntimeInitError> {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| RuntimeInitError::Read(path.clone(), error.to_string()))?;
        Self::from_toml_str(&text)
    }

    pub fn from_toml_str(text: &str) -> Result<Self, RuntimeInitError> {
        let config: Self =
            toml::from_str(text).map_err(|error| RuntimeInitError::Toml(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn birth_witness_owner(&self) -> Result<RuntimeBirthWitnessOwner, RuntimeInitError> {
        self.validate()?;
        let trust =
            load_runtime_birth_witness_trust(&self.credentials.birth_witness_trust_manifest_path)?;
        Ok(RuntimeBirthWitnessOwner { trust })
    }

    /// Rebase the sealed birth-witness manifest to its fixed filename under
    /// the currently configured Runtime credential root.
    pub fn rebase_birth_witness_trust_manifest(&mut self) {
        self.credentials.birth_witness_trust_manifest_path = self
            .paths
            .credentials_root(&self.state_root)
            .join("birth-witness-trust.json");
    }

    pub fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.schema != RUNTIME_INIT_SCHEMA {
            return Err(RuntimeInitError::UnsupportedSchema(self.schema.clone()));
        }
        validate_absolute_path("state_root", &self.state_root)?;
        self.binaries.validate()?;
        self.paths.validate()?;
        self.kernel.validate()?;
        let tls_root = self.paths.tls_root(&self.state_root);
        let credential_root = self.paths.credentials_root(&self.state_root);
        validate_non_empty_trimmed("api.address", &self.api.address)?;
        if self.socket_addrs()?.iter().any(SocketAddr::is_ipv6) {
            return Err(RuntimeInitError::Policy(
                "api.address must resolve only to IPv4".to_owned(),
            ));
        }
        if let Some(continuity_control) = &self.continuity_control {
            continuity_control
                .validate(&self.state_root, &self.socket_addrs()?)
                .map_err(|error| RuntimeInitError::Policy(error.to_string()))?;
        }
        validate_https_base_url("api.public_base_url", &self.api.public_base_url)?;
        let public_uri = parse_http_uri(&self.api.public_base_url)?;
        let public_host =
            public_uri
                .host()
                .ok_or_else(|| RuntimeInitError::InvalidHttpsBaseUrl {
                    field: "api.public_base_url",
                    value: self.api.public_base_url.clone(),
                })?;
        if self.api.tls.server_name != public_host {
            return Err(RuntimeInitError::TlsServerNameMismatch {
                configured: self.api.tls.server_name.clone(),
                public_host: public_host.to_owned(),
            });
        }
        self.polis.validate(public_host)?;
        self.resident_shepherd.validate()?;
        self.service_convergence.validate()?;
        if self.api.bind_attempts == 0 || self.api.bind_attempts > 100 {
            return Err(RuntimeInitError::Policy(
                "api.bind_attempts must be between 1 and 100".to_owned(),
            ));
        }
        for (field, value) in [
            ("api.bind_retry_millis", self.api.bind_retry_millis),
            (
                "api.websocket_auth_timeout_millis",
                self.api.websocket_auth_timeout_millis,
            ),
            (
                "api.websocket_refresh_millis",
                self.api.websocket_refresh_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        validate_bounded_capacity(
            "api.websocket_max_frame_bytes",
            self.api.websocket_max_frame_bytes,
        )?;
        validate_distinct_paths(
            "api.tls.certificate_chain_path",
            &self.api.tls.certificate_chain_path,
            "api.tls.private_key_path",
            &self.api.tls.private_key_path,
        )?;
        validate_distinct_paths(
            "api.tls.certificate_chain_path",
            &self.api.tls.certificate_chain_path,
            "api.tls.trust_roots_path",
            &self.api.tls.trust_roots_path,
        )?;
        validate_distinct_paths(
            "api.tls.private_key_path",
            &self.api.tls.private_key_path,
            "api.tls.trust_roots_path",
            &self.api.tls.trust_roots_path,
        )?;
        validate_child_path(
            "api.tls.certificate_chain_path",
            &tls_root,
            &self.api.tls.certificate_chain_path,
        )?;
        validate_child_path(
            "api.tls.trust_roots_path",
            &tls_root,
            &self.api.tls.trust_roots_path,
        )?;
        validate_child_path(
            "api.tls.private_key_path",
            &tls_root,
            &self.api.tls.private_key_path,
        )?;
        for (field, value) in [
            (
                "credentials.control_key_id",
                &self.credentials.control_key_id,
            ),
            (
                "credentials.control_principal",
                &self.credentials.control_principal,
            ),
            (
                "credentials.operation_key_id",
                &self.credentials.operation_key_id,
            ),
            (
                "credentials.migration_decision_key_id",
                &self.credentials.migration_decision_key_id,
            ),
            (
                "credentials.continuity_key_id",
                &self.credentials.continuity_key_id,
            ),
        ] {
            validate_non_empty_trimmed(field, value)?;
        }
        if self.credentials.migration_decision_key_generation == 0
            || self.credentials.migration_decision_key_id == self.credentials.operation_key_id
            || self.credentials.migration_decision_public_key_path
                == self.credentials.operation_public_key_path
        {
            return Err(RuntimeInitError::Policy(
                "migration decision authority must be nonzero and distinct from the operation permit authority"
                    .to_owned(),
            ));
        }
        validate_non_empty_trimmed("credentials.sntp_server", &self.credentials.sntp_server)?;
        for (field, path) in [
            (
                "credentials.control_public_key_path",
                &self.credentials.control_public_key_path,
            ),
            (
                "credentials.operation_public_key_path",
                &self.credentials.operation_public_key_path,
            ),
            (
                "credentials.migration_decision_public_key_path",
                &self.credentials.migration_decision_public_key_path,
            ),
            (
                "credentials.continuity_signing_key_path",
                &self.credentials.continuity_signing_key_path,
            ),
            (
                "credentials.observatory_token_path",
                &self.credentials.observatory_token_path,
            ),
            (
                "credentials.acip_write_token_path",
                &self.credentials.acip_write_token_path,
            ),
            (
                "credentials.birth_witness_trust_manifest_path",
                &self.credentials.birth_witness_trust_manifest_path,
            ),
        ] {
            validate_child_path(field, &credential_root, path)?;
        }
        self.shutdown.validate()?;
        self.guardian.validate()?;
        self.qualification.validate()?;
        validate_origin_list(
            "observatory.allowed_origins",
            &self.observatory.allowed_origins,
        )?;
        validate_additional_origin_list(&self.observatory.additional_allowed_origins)?;
        validate_combined_origin_list(
            &self.observatory.allowed_origins,
            &self.observatory.additional_allowed_origins,
        )?;
        if !self
            .observatory
            .allowed_origins
            .iter()
            .chain(self.observatory.additional_allowed_origins.iter())
            .any(|origin| origin == &self.polis.observatory_public_origin)
        {
            return Err(RuntimeInitError::Policy(
                "polis.observatory_public_origin must be present in the Observatory allowed-origin set"
                    .to_owned(),
            ));
        }
        self.observability_pipeline.validate()?;
        self.agent_partial_checkpoints.validate()?;
        self.weather
            .validate()
            .map_err(|error| RuntimeInitError::Weather(error.to_string()))?;
        Ok(())
    }

    pub fn socket_addrs(&self) -> Result<Vec<SocketAddr>, RuntimeInitError> {
        self.api
            .address
            .to_socket_addrs()
            .map(|addrs| addrs.collect::<Vec<_>>())
            .map_err(|error| RuntimeInitError::BindAddress(error.to_string()))
            .and_then(|addrs| {
                if addrs.is_empty() {
                    Err(RuntimeInitError::BindAddress(
                        "no socket addresses resolved".to_owned(),
                    ))
                } else {
                    Ok(addrs)
                }
            })
    }

    pub fn observatory_allowed_origins(&self) -> Vec<String> {
        self.observatory
            .allowed_origins
            .iter()
            .chain(self.observatory.additional_allowed_origins.iter())
            .cloned()
            .collect()
    }

    pub fn runtime_observability(&self) -> &RuntimeObservabilityInitConfig {
        &self.observability_pipeline
    }

    pub fn guardian_shutdown_grace_millis(&self) -> u64 {
        self.shutdown
            .checkpoint_deadline_millis
            .saturating_add(self.shutdown.kernel_grace_millis)
            .saturating_add(self.shutdown.api_drain_millis)
            .saturating_add(self.shutdown.guardian_margin_millis)
    }

    pub fn state_root(&self) -> &PathBuf {
        &self.state_root
    }

    pub fn continuity_root(&self) -> PathBuf {
        self.paths.continuity_root(&self.state_root)
    }

    pub fn continuity_identity_projection(&self) -> Result<serde_json::Value, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;
        if let Some(credentials) = value
            .get_mut("credentials")
            .and_then(serde_json::Value::as_object_mut)
        {
            credentials.remove("continuity_min_generation");
        }
        if let Some(observability) = value
            .get_mut("observability_pipeline")
            .and_then(serde_json::Value::as_object_mut)
        {
            observability.remove("lifecycle_run");
            observability.remove("lifecycle_cycle");
        }
        if let Some(observatory) = value
            .get_mut("observatory")
            .and_then(serde_json::Value::as_object_mut)
        {
            observatory.insert(
                "additional_allowed_origins".to_owned(),
                serde_json::Value::Array(Vec::new()),
            );
        }
        if let Some(resident_shepherd) = value.get_mut("resident_shepherd") {
            match resident_shepherd {
                serde_json::Value::Object(shepherd) => {
                    shepherd.remove("display_name");
                }
                serde_json::Value::Array(shepherds) => {
                    for shepherd in shepherds {
                        if let Some(shepherd) = shepherd.as_object_mut() {
                            shepherd.remove("display_name");
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AgentPartialCheckpointInitConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
    pub snapshot_concurrency: usize,
    pub max_partial_bytes: u64,
    pub local_max_bytes: u64,
    pub local_max_files: usize,
    pub retained_partials_per_agent: usize,
    pub spool_max_bytes: u64,
    pub spool_max_files: usize,
    pub s3_archive: Option<AgentPartialS3ArchiveInitConfig>,
}

impl Default for AgentPartialCheckpointInitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 300,
            snapshot_concurrency: 4,
            max_partial_bytes: 16 * 1024 * 1024,
            local_max_bytes: 2 * 1024 * 1024 * 1024,
            local_max_files: 8_192,
            retained_partials_per_agent: 12,
            spool_max_bytes: 512 * 1024 * 1024,
            spool_max_files: 4_096,
            s3_archive: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentPartialS3ArchiveInitConfig {
    pub region: String,
    pub bucket: String,
    pub kms_key_arn: String,
}

impl AgentPartialCheckpointInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if !(60..=86_400).contains(&self.interval_seconds) {
            return Err(RuntimeInitError::Policy(
                "agent_partial_checkpoints.interval_seconds must be between 60 and 86400"
                    .to_owned(),
            ));
        }
        if self.snapshot_concurrency == 0 || self.snapshot_concurrency > 64 {
            return Err(RuntimeInitError::Policy(
                "agent_partial_checkpoints.snapshot_concurrency must be between 1 and 64"
                    .to_owned(),
            ));
        }
        if self.max_partial_bytes == 0
            || self.max_partial_bytes > 16 * 1024 * 1024
            || self.local_max_bytes < self.max_partial_bytes
            || self.local_max_bytes > 2 * 1024 * 1024 * 1024
            || self.local_max_files == 0
            || self.local_max_files > 8_192
            || self.retained_partials_per_agent == 0
            || self.retained_partials_per_agent > 12
            || self.spool_max_bytes < self.max_partial_bytes
            || self.spool_max_bytes > 512 * 1024 * 1024
            || self.spool_max_files == 0
            || self.spool_max_files > 4_096
        {
            return Err(RuntimeInitError::Policy(
                "agent_partial_checkpoints storage bounds exceed the governed limits".to_owned(),
            ));
        }
        if let Some(archive) = &self.s3_archive {
            validate_s3_bucket_name(
                "agent_partial_checkpoints.s3_archive.bucket",
                &archive.bucket,
            )?;
            validate_non_empty_trimmed(
                "agent_partial_checkpoints.s3_archive.region",
                &archive.region,
            )?;
            validate_non_empty_trimmed(
                "agent_partial_checkpoints.s3_archive.kms_key_arn",
                &archive.kms_key_arn,
            )?;
            if !archive
                .region
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
                || !archive.kms_key_arn.starts_with("arn:")
            {
                return Err(RuntimeInitError::Policy(
                    "agent_partial_checkpoints S3 region or KMS key ARN is invalid".to_owned(),
                ));
            }
        }
        Ok(())
    }
}

pub const MIN_SERVICE_CONVERGENCE_MILLIS: u64 = 1_000;
pub const MAX_SERVICE_CONVERGENCE_MILLIS: u64 = 3_600_000;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RuntimeServiceConvergenceInitConfig {
    pub stop_timeout_millis: u64,
    pub unload_timeout_millis: u64,
    pub listener_timeout_millis: u64,
    pub readiness_timeout_millis: u64,
}

impl Default for RuntimeServiceConvergenceInitConfig {
    fn default() -> Self {
        Self {
            stop_timeout_millis: 300_000,
            unload_timeout_millis: 300_000,
            listener_timeout_millis: 300_000,
            readiness_timeout_millis: 900_000,
        }
    }
}

impl RuntimeServiceConvergenceInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            (
                "service_convergence.stop_timeout_millis",
                self.stop_timeout_millis,
            ),
            (
                "service_convergence.unload_timeout_millis",
                self.unload_timeout_millis,
            ),
            (
                "service_convergence.listener_timeout_millis",
                self.listener_timeout_millis,
            ),
            (
                "service_convergence.readiness_timeout_millis",
                self.readiness_timeout_millis,
            ),
        ] {
            if !(MIN_SERVICE_CONVERGENCE_MILLIS..=MAX_SERVICE_CONVERGENCE_MILLIS).contains(&value) {
                return Err(RuntimeInitError::Policy(format!(
                    "{field} must be between {MIN_SERVICE_CONVERGENCE_MILLIS} and {MAX_SERVICE_CONVERGENCE_MILLIS}"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBinariesInitConfig {
    pub kernel_path: PathBuf,
}

impl RuntimeBinariesInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_absolute_path("binaries.kernel_path", &self.kernel_path)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimePathsInitConfig {
    pub continuity_dir: PathBuf,
    pub tls_dir: PathBuf,
    pub credentials_dir: PathBuf,
    pub observability_dir: PathBuf,
}

impl RuntimePathsInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_relative_runtime_path("paths.continuity_dir", &self.continuity_dir)?;
        validate_relative_runtime_path("paths.tls_dir", &self.tls_dir)?;
        validate_relative_runtime_path("paths.credentials_dir", &self.credentials_dir)?;
        validate_relative_runtime_path("paths.observability_dir", &self.observability_dir)?;
        Ok(())
    }

    pub fn continuity_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.continuity_dir)
    }

    pub fn tls_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.tls_dir)
    }

    pub fn credentials_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.credentials_dir)
    }

    pub fn observability_root(&self, state_root: &Path) -> PathBuf {
        state_root.join(&self.observability_dir)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeKernelInitConfig {
    pub recorder_capacity: usize,
    pub control_history_capacity: usize,
    pub checkpoint_channel_capacity: usize,
    #[serde(default = "default_canonical_ingress_capacity")]
    pub canonical_ingress_capacity: usize,
    pub component_readiness_timeout_millis: u64,
    pub observability_poll_millis: u64,
    pub weather_stale_after_millis: u64,
    pub guardian_lease_connect_millis: u64,
    pub guardian_lease_auth_millis: u64,
    pub trusted_time_sample_timeout_millis: u64,
    pub trusted_time_max_offset_millis: u64,
    pub trusted_time_max_round_trip_millis: u64,
    pub trusted_time_retry_millis: u64,
    pub trusted_time_refresh_millis: u64,
}

impl RuntimeKernelInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            ("kernel.recorder_capacity", self.recorder_capacity),
            (
                "kernel.control_history_capacity",
                self.control_history_capacity,
            ),
            (
                "kernel.checkpoint_channel_capacity",
                self.checkpoint_channel_capacity,
            ),
            (
                "kernel.canonical_ingress_capacity",
                self.canonical_ingress_capacity,
            ),
        ] {
            validate_bounded_capacity(field, value)?;
        }
        for (field, value) in [
            (
                "kernel.component_readiness_timeout_millis",
                self.component_readiness_timeout_millis,
            ),
            (
                "kernel.observability_poll_millis",
                self.observability_poll_millis,
            ),
            (
                "kernel.weather_stale_after_millis",
                self.weather_stale_after_millis,
            ),
            (
                "kernel.guardian_lease_connect_millis",
                self.guardian_lease_connect_millis,
            ),
            (
                "kernel.guardian_lease_auth_millis",
                self.guardian_lease_auth_millis,
            ),
            (
                "kernel.trusted_time_sample_timeout_millis",
                self.trusted_time_sample_timeout_millis,
            ),
            (
                "kernel.trusted_time_max_offset_millis",
                self.trusted_time_max_offset_millis,
            ),
            (
                "kernel.trusted_time_max_round_trip_millis",
                self.trusted_time_max_round_trip_millis,
            ),
            (
                "kernel.trusted_time_retry_millis",
                self.trusted_time_retry_millis,
            ),
            (
                "kernel.trusted_time_refresh_millis",
                self.trusted_time_refresh_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        Ok(())
    }
}

fn default_canonical_ingress_capacity() -> usize {
    64
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeApiInitConfig {
    pub address: String,
    pub public_base_url: String,
    pub bind_attempts: u32,
    pub bind_retry_millis: u64,
    pub websocket_auth_timeout_millis: u64,
    pub websocket_refresh_millis: u64,
    pub websocket_max_frame_bytes: usize,
    pub tls: RuntimeTlsInitConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTlsInitConfig {
    pub certificate_chain_path: PathBuf,
    pub private_key_path: PathBuf,
    pub trust_roots_path: PathBuf,
    pub server_name: String,
}

impl RuntimeTlsInitConfig {
    pub fn identity_paths(&self) -> crate::tls::TlsIdentityPaths {
        crate::tls::TlsIdentityPaths {
            certificate_chain_path: self.certificate_chain_path.clone(),
            private_key_path: self.private_key_path.clone(),
        }
    }

    pub fn server_validation(&self) -> crate::tls::TlsServerValidation {
        crate::tls::TlsServerValidation {
            trust_roots_path: self.trust_roots_path.clone(),
            server_name: self.server_name.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCredentialInitConfig {
    pub control_public_key_path: PathBuf,
    pub control_key_id: String,
    pub control_principal: String,
    pub operation_public_key_path: PathBuf,
    pub operation_key_id: String,
    /// Boot-trusted #204 decision authority. This key is deliberately
    /// independent from the generic governed-operation permit authority.
    pub migration_decision_public_key_path: PathBuf,
    pub migration_decision_key_id: String,
    pub migration_decision_key_generation: u64,
    pub continuity_signing_key_path: PathBuf,
    pub continuity_key_id: String,
    pub observatory_token_path: PathBuf,
    pub acip_write_token_path: PathBuf,
    birth_witness_trust_manifest_path: PathBuf,
    pub continuity_min_generation: u64,
    pub sntp_server: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeShutdownInitConfig {
    pub checkpoint_deadline_millis: u64,
    pub kernel_grace_millis: u64,
    pub api_drain_millis: u64,
    pub guardian_margin_millis: u64,
}

impl RuntimeShutdownInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        self.guardian_budgets().map(|_| ())
    }

    pub fn guardian_budgets(&self) -> Result<(u64, u64), RuntimeInitError> {
        let child_budget = self
            .checkpoint_deadline_millis
            .checked_add(self.kernel_grace_millis)
            .and_then(|total| total.checked_add(self.api_drain_millis))
            .ok_or_else(|| {
                RuntimeInitError::Policy("shutdown child budget overflows u64".to_owned())
            })?;
        let total_budget = child_budget
            .checked_add(self.guardian_margin_millis)
            .ok_or_else(|| {
                RuntimeInitError::Policy("guardian shutdown budget overflows u64".to_owned())
            })?;
        for (field, value) in [
            (
                "shutdown.checkpoint_deadline_millis",
                self.checkpoint_deadline_millis,
            ),
            ("shutdown.kernel_grace_millis", self.kernel_grace_millis),
            ("shutdown.api_drain_millis", self.api_drain_millis),
            (
                "shutdown.guardian_margin_millis",
                self.guardian_margin_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        if child_budget > MAX_RUNTIME_INIT_MILLIS {
            return Err(RuntimeInitError::Policy(format!(
                "shutdown child budget must not exceed {MAX_RUNTIME_INIT_MILLIS}"
            )));
        }
        if total_budget > MAX_RUNTIME_INIT_MILLIS {
            return Err(RuntimeInitError::Policy(format!(
                "guardian shutdown budget must not exceed {MAX_RUNTIME_INIT_MILLIS}"
            )));
        }
        Ok((child_budget, total_budget))
    }
}

#[cfg(test)]
mod shutdown_policy_tests {
    use super::*;

    fn shutdown(
        checkpoint_deadline_millis: u64,
        kernel_grace_millis: u64,
        api_drain_millis: u64,
        guardian_margin_millis: u64,
    ) -> RuntimeShutdownInitConfig {
        RuntimeShutdownInitConfig {
            checkpoint_deadline_millis,
            kernel_grace_millis,
            api_drain_millis,
            guardian_margin_millis,
        }
    }

    #[test]
    fn aggregate_shutdown_policy_enforces_boundary_and_overflow() {
        assert_eq!(
            shutdown(599_997, 1, 1, 1).guardian_budgets(),
            Ok((599_999, 600_000))
        );

        for (policy, expected) in [
            (
                shutdown(599_998, 1, 1, 1),
                "guardian shutdown budget must not exceed 600000",
            ),
            (
                shutdown(599_999, 1, 1, 1),
                "shutdown child budget must not exceed 600000",
            ),
            (
                shutdown(0, 1, 1, 1),
                "shutdown.checkpoint_deadline_millis must be between 1 and 600000",
            ),
            (
                shutdown(u64::MAX, 1, 1, 1),
                "shutdown child budget overflows u64",
            ),
            (
                shutdown(1, 1, 1, u64::MAX),
                "guardian shutdown budget overflows u64",
            ),
        ] {
            assert_eq!(
                policy.guardian_budgets(),
                Err(RuntimeInitError::Policy(expected.to_owned()))
            );
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeGuardianInitConfig {
    pub restart_budget: u32,
    pub backoff_base_millis: u64,
    pub backoff_cap_millis: u64,
    pub healthy_window_millis: u64,
    pub lease_auth_timeout_millis: u64,
    pub lease_auth_attempts: u32,
    pub capture_max_bytes: u64,
    pub capture_drain_grace_millis: u64,
    pub configuration_exit_codes: Vec<i32>,
}

impl RuntimeGuardianInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.restart_budget > MAX_GUARDIAN_RESTART_BUDGET {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.restart_budget exceeds {MAX_GUARDIAN_RESTART_BUDGET}"
            )));
        }
        validate_bounded_millis("guardian.backoff_base_millis", self.backoff_base_millis)?;
        validate_bounded_millis("guardian.backoff_cap_millis", self.backoff_cap_millis)?;
        validate_bounded_millis("guardian.healthy_window_millis", self.healthy_window_millis)?;
        validate_bounded_millis(
            "guardian.lease_auth_timeout_millis",
            self.lease_auth_timeout_millis,
        )?;
        validate_bounded_millis(
            "guardian.capture_drain_grace_millis",
            self.capture_drain_grace_millis,
        )?;
        if self.lease_auth_attempts == 0
            || self.lease_auth_attempts > MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS
        {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.lease_auth_attempts must be in 1..={MAX_GUARDIAN_LEASE_AUTH_ATTEMPTS}"
            )));
        }
        if self.capture_max_bytes == 0 || self.capture_max_bytes > MAX_OBSERVABILITY_FILE_BYTES {
            return Err(RuntimeInitError::Policy(format!(
                "guardian.capture_max_bytes must be in 1..={MAX_OBSERVABILITY_FILE_BYTES}"
            )));
        }
        if self.backoff_cap_millis < self.backoff_base_millis {
            return Err(RuntimeInitError::Policy(
                "guardian.backoff_cap_millis must be >= backoff_base_millis".to_owned(),
            ));
        }
        if self.configuration_exit_codes.is_empty()
            || self.configuration_exit_codes.len() > MAX_GUARDIAN_CONFIGURATION_EXIT_CODES
            || self.configuration_exit_codes.iter().any(|code| *code <= 0)
        {
            return Err(RuntimeInitError::Policy(
                "guardian.configuration_exit_codes must be a non-empty bounded list of positive exit codes".to_owned(),
            ));
        }
        let unique = self
            .configuration_exit_codes
            .iter()
            .collect::<BTreeSet<_>>();
        if unique.len() != self.configuration_exit_codes.len() {
            return Err(RuntimeInitError::Policy(
                "guardian.configuration_exit_codes must be unique".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeQualificationInitConfig {
    pub readiness_timeout_millis: u64,
    pub readiness_poll_millis: u64,
    pub shutdown_wait_millis: u64,
}

impl RuntimeQualificationInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        for (field, value) in [
            (
                "qualification.readiness_timeout_millis",
                self.readiness_timeout_millis,
            ),
            (
                "qualification.readiness_poll_millis",
                self.readiness_poll_millis,
            ),
            (
                "qualification.shutdown_wait_millis",
                self.shutdown_wait_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        if self.readiness_poll_millis >= self.readiness_timeout_millis {
            return Err(RuntimeInitError::Policy(
                "qualification.readiness_poll_millis must be less than readiness_timeout_millis"
                    .to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservatoryInitConfig {
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub additional_allowed_origins: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolisInitConfig {
    pub id: String,
    pub display_name: String,
    pub public_domain: String,
    pub observatory_public_origin: String,
    #[serde(default)]
    pub vertex_ai: Option<PolisVertexAiInitConfig>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolisVertexAiInitConfig {
    pub provider: String,
    pub gcp_project: String,
    pub vertex_location: String,
    pub model: String,
    pub credential_source: VertexAiCredentialSource,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VertexAiCredentialSource {
    ApplicationDefaultCredentials,
    ServiceAccountFile { path: PathBuf },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VertexAiProviderFailure {
    MissingCredentials,
    DisabledApi,
    ProjectLocationMismatch,
    QuotaOrAuth,
    ModelOrRequest,
    Transport,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentShepherdInitConfig {
    pub name: String,
    pub display_name: String,
    pub office: String,
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    #[serde(default)]
    pub preload: ResidentShepherdPreloadConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResidentShepherdSetInitConfig {
    One(ResidentShepherdInitConfig),
    Many(Vec<ResidentShepherdInitConfig>),
}

impl ResidentShepherdSetInitConfig {
    pub fn iter(&self) -> std::slice::Iter<'_, ResidentShepherdInitConfig> {
        match self {
            Self::One(config) => std::slice::from_ref(config).iter(),
            Self::Many(configs) => configs.iter(),
        }
    }

    pub fn primary(&self) -> &ResidentShepherdInitConfig {
        self.iter()
            .next()
            .expect("validated Shepherd set is non-empty")
    }

    fn validate(&self) -> Result<(), RuntimeInitError> {
        let configs = self.iter().collect::<Vec<_>>();
        if configs.is_empty() {
            return Err(RuntimeInitError::Policy(
                "resident_shepherd must contain at least one configured Shepherd".to_owned(),
            ));
        }
        let mut names = std::collections::BTreeSet::new();
        for config in configs {
            config.validate()?;
            if !names.insert(config.name.as_str()) {
                return Err(RuntimeInitError::Policy(format!(
                    "duplicate resident_shepherd.name: {}",
                    config.name
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentShepherdPreloadConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_shepherd_preload_timeout_millis")]
    pub timeout_millis: u64,
    #[serde(default = "default_shepherd_retry_initial_millis")]
    pub retry_initial_millis: u64,
    #[serde(default = "default_shepherd_retry_max_millis")]
    pub retry_max_millis: u64,
}

impl Default for ResidentShepherdPreloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_millis: default_shepherd_preload_timeout_millis(),
            retry_initial_millis: default_shepherd_retry_initial_millis(),
            retry_max_millis: default_shepherd_retry_max_millis(),
        }
    }
}

const fn default_true() -> bool {
    true
}
const fn default_shepherd_preload_timeout_millis() -> u64 {
    15 * 60 * 1_000
}
const fn default_shepherd_retry_initial_millis() -> u64 {
    5_000
}
const fn default_shepherd_retry_max_millis() -> u64 {
    60_000
}

impl ResidentShepherdInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if !crate::is_canonical_agent_name(&self.name) {
            return Err(RuntimeInitError::Policy(
                "resident_shepherd.name must be a canonical two-part agent name".to_owned(),
            ));
        }
        validate_non_empty_trimmed("resident_shepherd.display_name", &self.display_name)?;
        validate_non_empty_trimmed("resident_shepherd.office", &self.office)?;
        validate_non_empty_trimmed("resident_shepherd.provider", &self.provider)?;
        validate_non_empty_trimmed("resident_shepherd.model", &self.model)?;
        validate_non_empty_trimmed("resident_shepherd.endpoint", &self.endpoint)?;
        if self.provider.len() > 64
            || !self
                .provider
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(RuntimeInitError::Policy(
                "resident_shepherd.provider must be a lowercase provider identifier".to_owned(),
            ));
        }
        if !crate::resident_shepherd_provider_is_available(&self.provider) {
            return Err(RuntimeInitError::Policy(format!(
                "resident_shepherd.provider '{}' has no executable adapter in this Runtime build",
                self.provider
            )));
        }
        if crate::control::validate_private_provider_binding(&self.model, &self.endpoint).is_err() {
            return Err(RuntimeInitError::Policy(
                "resident_shepherd model and endpoint must form a valid private provider binding"
                    .to_owned(),
            ));
        }
        if self.preload.timeout_millis < 60_000
            || self.preload.retry_initial_millis == 0
            || self.preload.retry_max_millis < self.preload.retry_initial_millis
        {
            return Err(RuntimeInitError::Policy(
                "resident_shepherd preload and retry budgets are invalid".to_owned(),
            ));
        }
        Ok(())
    }
}

impl PolisInitConfig {
    fn validate(&self, public_host: &str) -> Result<(), RuntimeInitError> {
        if self.id.is_empty()
            || self.id.len() > 128
            || !self.id.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-')
            })
        {
            return Err(RuntimeInitError::Policy(
                "polis.id must be a bounded safe identifier".to_owned(),
            ));
        }
        validate_non_empty_trimmed("polis.display_name", &self.display_name)?;
        if self.display_name.len() > 128 || self.display_name.bytes().any(|b| b.is_ascii_control())
        {
            return Err(RuntimeInitError::Policy(
                "polis.display_name must be at most 128 display characters".to_owned(),
            ));
        }
        if self.public_domain != self.public_domain.to_ascii_lowercase()
            || self.public_domain != public_host
            || self.public_domain.len() > 253
        {
            return Err(RuntimeInitError::Policy(
                "polis.public_domain must equal the canonical Runtime API and TLS host".to_owned(),
            ));
        }
        validate_origin(&self.observatory_public_origin)?;
        if let Some(vertex_ai) = &self.vertex_ai {
            vertex_ai.validate()?;
        }
        Ok(())
    }
}

impl PolisVertexAiInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        if self.provider != "vertex_ai" {
            return Err(RuntimeInitError::Policy(
                "polis.vertex_ai.provider must be vertex_ai".to_owned(),
            ));
        }
        validate_gcp_project("polis.vertex_ai.gcp_project", &self.gcp_project)?;
        validate_safe_label("polis.vertex_ai.vertex_location", &self.vertex_location)?;
        validate_safe_label("polis.vertex_ai.model", &self.model)?;
        match &self.credential_source {
            VertexAiCredentialSource::ApplicationDefaultCredentials => {}
            VertexAiCredentialSource::ServiceAccountFile { path } => {
                validate_absolute_path("polis.vertex_ai.credential_source.path", path)?;
            }
        }
        Ok(())
    }
}

pub fn classify_vertex_ai_provider_failure(
    status: Option<u16>,
    body: &str,
) -> VertexAiProviderFailure {
    let normalized = body.to_ascii_lowercase();
    if normalized.contains("application default credentials")
        || normalized.contains("could not load the default credentials")
        || normalized.contains("missing credentials")
        || normalized.contains("credential")
            && (normalized.contains("not found") || normalized.contains("unavailable"))
    {
        return VertexAiProviderFailure::MissingCredentials;
    }
    if normalized.contains("api has not been used")
        || normalized.contains("service disabled")
        || normalized.contains("enable it by visiting")
        || normalized.contains("aiplatform.googleapis.com") && normalized.contains("disabled")
    {
        return VertexAiProviderFailure::DisabledApi;
    }
    if normalized.contains("location")
        && (normalized.contains("project") || normalized.contains("region"))
        || normalized.contains("not found in location")
        || normalized.contains("publisher model")
            && (normalized.contains("not found") || normalized.contains("location"))
    {
        return VertexAiProviderFailure::ProjectLocationMismatch;
    }
    if matches!(status, Some(401 | 403 | 429))
        || normalized.contains("permission")
        || normalized.contains("quota")
        || normalized.contains("rate limit")
        || normalized.contains("unauthorized")
    {
        return VertexAiProviderFailure::QuotaOrAuth;
    }
    if matches!(status, Some(400 | 404))
        || normalized.contains("invalid argument")
        || normalized.contains("model")
        || normalized.contains("request")
    {
        return VertexAiProviderFailure::ModelOrRequest;
    }
    VertexAiProviderFailure::Transport
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeObservabilityInitConfig {
    pub vector_binary_path: PathBuf,
    pub service_name: String,
    pub revision: String,
    pub guardian_id: String,
    pub lifecycle_suite: String,
    pub lifecycle_run: String,
    pub lifecycle_cycle: String,
    pub trace_filter: String,
    pub otlp_endpoint: Option<String>,
    pub otlp_timeout_millis: u64,
    pub vector_startup_attempts: u32,
    pub vector_startup_backoff_millis: u64,
    pub vector_shutdown_limit_millis: u64,
    pub drain_timeout_millis: u64,
    pub vector_config_path: PathBuf,
    pub ingress_spool_path: PathBuf,
    pub master_log_path: PathBuf,
    pub audit_path: PathBuf,
    pub sequence_checkpoint_path: PathBuf,
    pub vector_data_dir: PathBuf,
    pub spool_max_bytes: u64,
    pub spool_retained_files: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloudwatch: Option<RuntimeCloudWatchInitConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s3_archive: Option<RuntimeS3LogArchiveInitConfig>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCloudWatchInitConfig {
    pub region: String,
    pub log_group: String,
    pub log_stream: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeS3LogArchiveInitConfig {
    pub region: String,
    pub bucket: String,
    pub environment: String,
    pub polis_id: String,
    pub runtime_id: String,
}

impl RuntimeObservabilityInitConfig {
    fn validate(&self) -> Result<(), RuntimeInitError> {
        validate_absolute_path(
            "observability_pipeline.vector_binary_path",
            &self.vector_binary_path,
        )?;
        for (field, value) in [
            ("observability_pipeline.service_name", &self.service_name),
            ("observability_pipeline.revision", &self.revision),
            ("observability_pipeline.guardian_id", &self.guardian_id),
            (
                "observability_pipeline.lifecycle_suite",
                &self.lifecycle_suite,
            ),
            ("observability_pipeline.lifecycle_run", &self.lifecycle_run),
            (
                "observability_pipeline.lifecycle_cycle",
                &self.lifecycle_cycle,
            ),
            ("observability_pipeline.trace_filter", &self.trace_filter),
        ] {
            validate_non_empty_trimmed(field, value)?;
        }
        for (field, value) in [
            (
                "observability_pipeline.otlp_timeout_millis",
                self.otlp_timeout_millis,
            ),
            (
                "observability_pipeline.vector_startup_backoff_millis",
                self.vector_startup_backoff_millis,
            ),
            (
                "observability_pipeline.drain_timeout_millis",
                self.drain_timeout_millis,
            ),
            (
                "observability_pipeline.vector_shutdown_limit_millis",
                self.vector_shutdown_limit_millis,
            ),
        ] {
            validate_bounded_millis(field, value)?;
        }
        if self.vector_startup_attempts == 0 || self.vector_startup_attempts > 10 {
            return Err(RuntimeInitError::Policy(
                "observability_pipeline.vector_startup_attempts must be between 1 and 10"
                    .to_owned(),
            ));
        }
        if self.vector_shutdown_limit_millis >= self.drain_timeout_millis {
            return Err(RuntimeInitError::Policy(
                "observability_pipeline.vector_shutdown_limit_millis must be less than drain_timeout_millis"
                    .to_owned(),
            ));
        }
        if self.spool_max_bytes == 0 || self.spool_max_bytes > MAX_OBSERVABILITY_FILE_BYTES {
            return Err(RuntimeInitError::Policy(format!(
                "observability_pipeline.spool_max_bytes must be between 1 and {MAX_OBSERVABILITY_FILE_BYTES}"
            )));
        }
        if self.spool_retained_files == 0
            || self.spool_retained_files > MAX_OBSERVABILITY_RETAINED_FILES
        {
            return Err(RuntimeInitError::Policy(format!(
                "observability_pipeline.spool_retained_files must be between 1 and {MAX_OBSERVABILITY_RETAINED_FILES}"
            )));
        }
        if let Some(endpoint) = self.otlp_endpoint.as_deref() {
            validate_observability_otlp_endpoint(endpoint)?;
        }
        if let Some(cloudwatch) = self.cloudwatch.as_ref() {
            for (field, value) in [
                (
                    "observability_pipeline.cloudwatch.region",
                    &cloudwatch.region,
                ),
                (
                    "observability_pipeline.cloudwatch.log_group",
                    &cloudwatch.log_group,
                ),
                (
                    "observability_pipeline.cloudwatch.log_stream",
                    &cloudwatch.log_stream,
                ),
            ] {
                validate_non_empty_trimmed(field, value)?;
            }
            if !cloudwatch.log_group.starts_with('/')
                || cloudwatch
                    .log_group
                    .bytes()
                    .any(|byte| byte.is_ascii_control())
                || cloudwatch
                    .log_stream
                    .bytes()
                    .any(|byte| byte.is_ascii_control())
            {
                return Err(RuntimeInitError::Policy(
                    "observability_pipeline.cloudwatch names are invalid".to_owned(),
                ));
            }
        }
        if let Some(archive) = self.s3_archive.as_ref() {
            for (field, value) in [
                ("observability_pipeline.s3_archive.region", &archive.region),
                ("observability_pipeline.s3_archive.bucket", &archive.bucket),
                (
                    "observability_pipeline.s3_archive.environment",
                    &archive.environment,
                ),
                (
                    "observability_pipeline.s3_archive.polis_id",
                    &archive.polis_id,
                ),
                (
                    "observability_pipeline.s3_archive.runtime_id",
                    &archive.runtime_id,
                ),
            ] {
                validate_non_empty_trimmed(field, value)?;
            }
            for (field, value) in [
                (
                    "observability_pipeline.s3_archive.environment",
                    archive.environment.as_str(),
                ),
                (
                    "observability_pipeline.s3_archive.polis_id",
                    archive.polis_id.as_str(),
                ),
                (
                    "observability_pipeline.s3_archive.runtime_id",
                    archive.runtime_id.as_str(),
                ),
            ] {
                validate_dns_safe_label(field, value)?;
            }
            validate_s3_bucket_name("observability_pipeline.s3_archive.bucket", &archive.bucket)?;
            if !archive
                .region
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            {
                return Err(RuntimeInitError::Policy(
                    "observability_pipeline.s3_archive.region must be lowercase AWS region syntax"
                        .to_owned(),
                ));
            }
        }
        for (field, path) in [
            ("vector_config_path", &self.vector_config_path),
            ("ingress_spool_path", &self.ingress_spool_path),
            ("master_log_path", &self.master_log_path),
            ("audit_path", &self.audit_path),
            ("sequence_checkpoint_path", &self.sequence_checkpoint_path),
            ("vector_data_dir", &self.vector_data_dir),
        ] {
            validate_relative_runtime_path(field, path)?;
        }
        Ok(())
    }
}

fn validate_dns_safe_label(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    if value.len() > 63
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        || value.starts_with('-')
        || value.ends_with('-')
    {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be a lowercase DNS-safe label"
        )));
    }
    Ok(())
}

fn validate_s3_bucket_name(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    if value.len() < 3
        || value.len() > 63
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-' || byte == b'.'
        })
        || value.starts_with(['-', '.'])
        || value.ends_with(['-', '.'])
        || value.contains("..")
        || value.contains(".-")
        || value.contains("-.")
    {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be a DNS-compatible S3 bucket name"
        )));
    }
    Ok(())
}

fn validate_non_empty_trimmed(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    if value.trim().is_empty() || value != value.trim() {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be non-empty without surrounding whitespace"
        )));
    }
    Ok(())
}

fn validate_safe_label(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    validate_non_empty_trimmed(field, value)?;
    if value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be a bounded provider label"
        )));
    }
    Ok(())
}

fn validate_gcp_project(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    validate_non_empty_trimmed(field, value)?;
    if value.len() < 6
        || value.len() > 30
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        || !value
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_lowercase())
        || !value
            .bytes()
            .last()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be an explicit GCP project id, not an ambient default"
        )));
    }
    Ok(())
}

fn validate_bounded_millis(field: &'static str, value: u64) -> Result<(), RuntimeInitError> {
    if value == 0 || value > MAX_RUNTIME_INIT_MILLIS {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be between 1 and {MAX_RUNTIME_INIT_MILLIS}"
        )));
    }
    Ok(())
}

fn validate_bounded_capacity(field: &'static str, value: usize) -> Result<(), RuntimeInitError> {
    if value == 0 || value > MAX_RUNTIME_INIT_CAPACITY {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be between 1 and {MAX_RUNTIME_INIT_CAPACITY}"
        )));
    }
    Ok(())
}

fn validate_relative_runtime_path(
    field: &'static str,
    path: &Path,
) -> Result<(), RuntimeInitError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must be a non-empty path relative to state_root"
        )));
    }
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return Err(RuntimeInitError::Policy(format!(
            "{field} must not escape state_root"
        )));
    }
    Ok(())
}

fn validate_observability_otlp_endpoint(value: &str) -> Result<(), RuntimeInitError> {
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() == Some("https") {
        return Ok(());
    }
    let host = uri.host().unwrap_or_default();
    if uri.scheme_str() == Some("http")
        && matches!(host, "localhost" | "127.0.0.1" | "::1" | "[::1]")
    {
        return Ok(());
    }
    Err(RuntimeInitError::Observability(
        "otlp_endpoint must be HTTPS or loopback HTTP".to_owned(),
    ))
}

fn validate_absolute_path(field: &'static str, path: &Path) -> Result<(), RuntimeInitError> {
    if path.as_os_str().is_empty() || !path.is_absolute() {
        return Err(RuntimeInitError::RelativePath(field));
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(RuntimeInitError::RelativePath(field));
    }
    Ok(())
}

fn validate_child_path(
    field: &'static str,
    root: &Path,
    path: &Path,
) -> Result<(), RuntimeInitError> {
    validate_absolute_path(field, path)?;
    if root.exists() && path.exists() {
        let canonical_root = root
            .canonicalize()
            .map_err(|_| RuntimeInitError::PathOutsideStateRoot(field))?;
        let canonical_path = path
            .canonicalize()
            .map_err(|_| RuntimeInitError::PathOutsideStateRoot(field))?;
        if !canonical_path.starts_with(canonical_root) {
            return Err(RuntimeInitError::PathOutsideStateRoot(field));
        }
    } else if !lexically_contains(root, path) {
        return Err(RuntimeInitError::PathOutsideStateRoot(field));
    }
    Ok(())
}

fn validate_distinct_paths(
    left_field: &'static str,
    left: &Path,
    right_field: &'static str,
    right: &Path,
) -> Result<(), RuntimeInitError> {
    if left == right {
        return Err(RuntimeInitError::InvalidTlsPaths);
    }
    validate_absolute_path(left_field, left)?;
    validate_absolute_path(right_field, right)?;
    Ok(())
}

fn lexically_contains(root: &Path, path: &Path) -> bool {
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::CurDir | std::path::Component::ParentDir
        )
    }) {
        return false;
    }
    path.starts_with(root)
}

fn validate_https_base_url(field: &'static str, value: &str) -> Result<(), RuntimeInitError> {
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() != Some("https") {
        return Err(RuntimeInitError::InvalidHttpsBaseUrl {
            field,
            value: value.to_owned(),
        });
    }
    if uri.query().is_some() {
        return Err(RuntimeInitError::InvalidHttpsBaseUrl {
            field,
            value: value.to_owned(),
        });
    }
    Ok(())
}

fn validate_origin_list(field: &'static str, origins: &[String]) -> Result<(), RuntimeInitError> {
    if origins.is_empty() {
        return Err(RuntimeInitError::NoAllowedOrigins(field));
    }
    let mut seen = BTreeSet::new();
    for origin in origins {
        if !seen.insert(origin.clone()) {
            return Err(RuntimeInitError::DuplicateOrigin(origin.clone()));
        }
        validate_origin(origin)?;
    }
    Ok(())
}

fn validate_additional_origin_list(origins: &[String]) -> Result<(), RuntimeInitError> {
    let mut seen = BTreeSet::new();
    for origin in origins {
        if !seen.insert(origin.clone()) {
            return Err(RuntimeInitError::DuplicateOrigin(origin.clone()));
        }
        validate_additional_origin(origin)?;
    }
    Ok(())
}

fn validate_combined_origin_list(
    allowed_origins: &[String],
    additional_allowed_origins: &[String],
) -> Result<(), RuntimeInitError> {
    let mut seen = BTreeSet::new();
    for origin in allowed_origins
        .iter()
        .chain(additional_allowed_origins.iter())
    {
        if !seen.insert(origin.clone()) {
            return Err(RuntimeInitError::DuplicateOrigin(origin.clone()));
        }
    }
    Ok(())
}

fn validate_origin(value: &str) -> Result<(), RuntimeInitError> {
    if value == "*" || value.len() > 512 || value.bytes().any(|byte| byte.is_ascii_control()) {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    let uri = parse_http_uri(value)?;
    if uri.scheme_str() != Some("https") || uri.path() != "/" || uri.query().is_some() {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    Ok(())
}

fn validate_additional_origin(value: &str) -> Result<(), RuntimeInitError> {
    if value == "http://localhost:8000" {
        return Ok(());
    }
    validate_origin(value)
}

fn parse_http_uri(value: &str) -> Result<axum::http::Uri, RuntimeInitError> {
    let uri = value
        .parse::<axum::http::Uri>()
        .map_err(|_| RuntimeInitError::InvalidOrigin(value.to_owned()))?;
    let Some(scheme) = uri.scheme_str() else {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    };
    if scheme != "http" && scheme != "https" {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    if uri.authority().is_none() {
        return Err(RuntimeInitError::InvalidOrigin(value.to_owned()));
    }
    Ok(uri)
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuntimeInitError {
    #[error("runtime serve requires an explicit init file")]
    MissingInitFile,
    #[error("runtime init file could not be read at {0}: {1}")]
    Read(PathBuf, String),
    #[error("invalid runtime init TOML: {0}")]
    Toml(String),
    #[error("unsupported runtime init schema: {0}")]
    UnsupportedSchema(String),
    #[error("runtime init {0} must not be empty")]
    NoAllowedOrigins(&'static str),
    #[error("runtime init {field} must be an HTTPS origin/base URL: {value}")]
    InvalidHttpsBaseUrl { field: &'static str, value: String },
    #[error("runtime init contains a duplicate observatory origin: {0}")]
    DuplicateOrigin(String),
    #[error("runtime init observatory origin is invalid: {0}")]
    InvalidOrigin(String),
    #[error("runtime init bind address did not resolve: {0}")]
    BindAddress(String),
    #[error("runtime init TLS certificate and private-key paths must be non-empty and distinct")]
    InvalidTlsPaths,
    #[error(
        "runtime init TLS server name {configured} does not match public API host {public_host}"
    )]
    TlsServerNameMismatch {
        configured: String,
        public_host: String,
    },
    #[error("runtime init {0} must be an absolute path without parent traversal")]
    RelativePath(&'static str),
    #[error("runtime init {0} must stay inside state_root")]
    PathOutsideStateRoot(&'static str),
    #[error("runtime init weather configuration is invalid: {0}")]
    Weather(String),
    #[error("runtime init observability pipeline configuration is invalid: {0}")]
    Observability(String),
    #[error("runtime init policy is invalid: {0}")]
    Policy(String),
}
