//! Production provider/profile hot-reload owner.
//!
//! The reload owner watches a provider-only sidecar, validates a complete
//! candidate document against the existing profile/materialization path, and
//! publishes immutable last-known-good snapshots for subsequent provider
//! resolution. Credential values and executable workflow/authority surfaces are
//! rejected before activation.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use adl_runtime_kernel::config_reload::{
    start_config_reload, ConfigReloadController, ConfigReloadError, ConfigReloadOptions,
    HotReloadHandle,
};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::adl;
use crate::provider_substrate;

use super::activate_provider_profile_candidate;

const PROVIDER_RELOAD_SIDECAR_SCHEMA: &str = "adl.provider_reload_sidecar.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReloadDiagnostic {
    pub generation: u64,
    pub code: String,
    pub redacted_message: String,
    pub observed_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct ProviderReloadSnapshot {
    pub schema: String,
    pub generation: u64,
    pub source: PathBuf,
    pub digest: String,
    pub document: Arc<adl::AdlDoc>,
}

#[derive(Debug, Clone)]
pub struct ProviderReloadHandle {
    handle: HotReloadHandle<ProviderReloadSnapshot>,
    diagnostic: Arc<Mutex<Option<ProviderReloadDiagnostic>>>,
}

impl ProviderReloadHandle {
    pub fn current_snapshot(&self) -> Arc<ProviderReloadSnapshot> {
        Arc::new(self.handle.current().value().clone())
    }

    pub fn current_document(&self) -> Arc<adl::AdlDoc> {
        Arc::clone(&self.current_snapshot().document)
    }

    pub async fn changed(&mut self) -> Result<Arc<ProviderReloadSnapshot>, ConfigReloadError> {
        self.handle
            .changed()
            .await
            .map(|snapshot| Arc::new(snapshot.value().clone()))
    }

    pub fn last_diagnostic(&self) -> Option<ProviderReloadDiagnostic> {
        self.diagnostic.lock().ok().and_then(|guard| guard.clone())
    }
}

#[derive(Debug)]
pub struct ProviderReloadOwner {
    controller: ConfigReloadController<ProviderReloadSnapshot>,
    diagnostic: Arc<Mutex<Option<ProviderReloadDiagnostic>>>,
}

impl ProviderReloadOwner {
    pub async fn start(
        provider_config_path: impl Into<PathBuf>,
        base_document: adl::AdlDoc,
        options: ConfigReloadOptions,
    ) -> Result<Self, ConfigReloadError> {
        let provider_config_path = provider_config_path.into();
        let active_document = Arc::new(Mutex::new(base_document));
        let diagnostic = Arc::new(Mutex::new(None));
        let generation = Arc::new(AtomicU64::new(0));
        let parser_source = provider_config_path.clone();
        let parser_active = Arc::clone(&active_document);
        let parser_diagnostic = Arc::clone(&diagnostic);
        let parser_generation = Arc::clone(&generation);
        let parser = Arc::new(move |raw: &str| {
            parse_provider_reload_snapshot(
                &parser_source,
                raw,
                &parser_active,
                &parser_diagnostic,
                &parser_generation,
            )
        });
        let controller = start_config_reload(provider_config_path, parser, options).await?;
        Ok(Self {
            controller,
            diagnostic,
        })
    }

    pub fn handle(&self) -> ProviderReloadHandle {
        ProviderReloadHandle {
            handle: self.controller.handle(),
            diagnostic: Arc::clone(&self.diagnostic),
        }
    }

    pub async fn shutdown(
        self,
    ) -> Result<adl_runtime_kernel::config_reload::ConfigReloadOutcome, ConfigReloadError> {
        self.controller.shutdown().await
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderReloadSidecar {
    #[serde(default)]
    schema: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    providers: HashMap<String, adl::ProviderSpec>,
}

fn parse_provider_reload_snapshot(
    source: &Path,
    raw: &str,
    active_document: &Arc<Mutex<adl::AdlDoc>>,
    diagnostic: &Arc<Mutex<Option<ProviderReloadDiagnostic>>>,
    generation: &Arc<AtomicU64>,
) -> std::result::Result<ProviderReloadSnapshot, ConfigReloadError> {
    let sidecar: ProviderReloadSidecar = serde_yaml::from_str(raw).map_err(|error| {
        record_diagnostic(
            diagnostic,
            generation.load(Ordering::SeqCst),
            "parse_error",
            error.to_string(),
        );
        ConfigReloadError::parse(error.to_string())
    })?;
    materialize_provider_reload_snapshot(source, sidecar, active_document, diagnostic, generation)
        .map_err(|error| {
            record_diagnostic(
                diagnostic,
                generation.load(Ordering::SeqCst),
                "validation_error",
                error.to_string(),
            );
            ConfigReloadError::validation(error.to_string())
        })
}

fn materialize_provider_reload_snapshot(
    source: &Path,
    sidecar: ProviderReloadSidecar,
    active_document: &Arc<Mutex<adl::AdlDoc>>,
    diagnostic: &Arc<Mutex<Option<ProviderReloadDiagnostic>>>,
    generation: &Arc<AtomicU64>,
) -> Result<ProviderReloadSnapshot> {
    if let Some(schema) = sidecar.schema.as_deref() {
        if schema != PROVIDER_RELOAD_SIDECAR_SCHEMA {
            return Err(anyhow!("provider reload sidecar schema is unsupported"));
        }
    }
    if sidecar.providers.is_empty() {
        return Err(anyhow!("provider reload sidecar must declare providers"));
    }
    reject_credential_values(&sidecar.providers)?;
    validate_provider_specs(&sidecar.providers)?;

    let mut active = active_document
        .lock()
        .map_err(|_| anyhow!("provider reload active snapshot lock poisoned"))?;
    let mut candidate = active.clone();
    if let Some(version) = sidecar.version {
        candidate.version = version;
    }
    candidate.providers = sidecar.providers;
    candidate
        .validate()
        .context("validate provider reload candidate document")?;
    let activation = activate_provider_profile_candidate(&active, &candidate)?;
    if !activation.accepted {
        return Err(anyhow!(
            "{}",
            activation
                .rejection
                .unwrap_or_else(|| "provider reload candidate rejected".to_string())
        ));
    }
    *active = activation.document;
    if let Ok(mut slot) = diagnostic.lock() {
        *slot = None;
    }
    let digest = redacted_provider_digest(&active.providers)?;
    let snapshot_generation = generation.fetch_add(1, Ordering::SeqCst);
    Ok(ProviderReloadSnapshot {
        schema: "adl.provider_reload_snapshot.v1".to_string(),
        generation: snapshot_generation,
        source: source.to_path_buf(),
        digest,
        document: Arc::new(active.clone()),
    })
}

fn validate_provider_specs(providers: &HashMap<String, adl::ProviderSpec>) -> Result<()> {
    for (provider_id, spec) in providers {
        provider_substrate::provider_substrate_v1(provider_id, spec)
            .with_context(|| format!("validate provider reload spec '{provider_id}'"))?;
    }
    Ok(())
}

fn reject_credential_values(providers: &HashMap<String, adl::ProviderSpec>) -> Result<()> {
    let value = serde_json::to_value(providers).context("serialize provider reload sidecar")?;
    reject_credential_value_at("$", &value)
}

fn reject_credential_value_at(path: &str, value: &Value) -> Result<()> {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let next = format!("{path}.{key}");
                if credential_value_key(key) {
                    return Err(anyhow!(
                        "provider reload sidecar contains credential value at {next}"
                    ));
                }
                if credential_value_container_key(key) {
                    reject_raw_credential_scalar(&next, value)?;
                }
                reject_credential_value_at(&next, value)?;
            }
        }
        Value::Array(values) => {
            for (idx, value) in values.iter().enumerate() {
                reject_credential_value_at(&format!("{path}[{idx}]"), value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn credential_value_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "api_key"
            | "apikey"
            | "token"
            | "secret"
            | "credential"
            | "credentials"
            | "password"
            | "client_secret"
            | "private_key"
            | "access_token"
            | "refresh_token"
    )
}

fn credential_value_container_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "value" | "raw" | "header" | "authorization" | "bearer"
    )
}

fn reject_raw_credential_scalar(path: &str, value: &Value) -> Result<()> {
    if let Some(raw) = value.as_str() {
        if looks_like_raw_credential(raw) {
            return Err(anyhow!(
                "provider reload sidecar contains credential value at {path}"
            ));
        }
    }
    Ok(())
}

fn looks_like_raw_credential(raw: &str) -> bool {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    trimmed.starts_with("sk-")
        || lower.starts_with("bearer ")
        || lower.contains("-----begin private key-----")
        || lower.contains("-----begin rsa private key-----")
        || lower.contains("-----begin ec private key-----")
        || (trimmed.len() >= 32
            && trimmed
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.')))
}

fn redacted_provider_digest(providers: &HashMap<String, adl::ProviderSpec>) -> Result<String> {
    let ordered = providers
        .iter()
        .map(|(provider_id, spec)| (provider_id.clone(), spec.clone()))
        .collect::<BTreeMap<_, _>>();
    let bytes = serde_json::to_vec(&ordered).context("serialize redacted provider digest input")?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn record_diagnostic(
    diagnostic: &Arc<Mutex<Option<ProviderReloadDiagnostic>>>,
    generation: u64,
    code: impl Into<String>,
    message: impl Into<String>,
) {
    if let Ok(mut slot) = diagnostic.lock() {
        *slot = Some(ProviderReloadDiagnostic {
            generation,
            code: code.into(),
            redacted_message: redact_diagnostic(message.into()),
            observed_at: SystemTime::now(),
        });
    }
}

fn redact_diagnostic(message: String) -> String {
    let mut redacted = Vec::new();
    for token in message.split_whitespace() {
        let lower = token.to_ascii_lowercase();
        if lower.contains("token")
            || lower.contains("secret")
            || lower.contains("api_key")
            || lower.contains("credential")
        {
            redacted.push("<redacted>");
        } else {
            redacted.push(token);
        }
    }
    redacted.join(" ")
}

#[derive(Clone)]
struct GlobalProviderReloadRegistration {
    token: u64,
    handle: ProviderReloadHandle,
}

static GLOBAL_PROVIDER_RELOAD: OnceLock<Mutex<Option<GlobalProviderReloadRegistration>>> =
    OnceLock::new();
static NEXT_GLOBAL_PROVIDER_RELOAD_TOKEN: AtomicU64 = AtomicU64::new(1);

fn global_provider_reload() -> &'static Mutex<Option<GlobalProviderReloadRegistration>> {
    GLOBAL_PROVIDER_RELOAD.get_or_init(|| Mutex::new(None))
}

pub struct ProviderReloadGlobalGuard {
    token: u64,
}

pub fn set_global_provider_reload_handle(
    handle: ProviderReloadHandle,
) -> ProviderReloadGlobalGuard {
    let token = NEXT_GLOBAL_PROVIDER_RELOAD_TOKEN.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut slot) = global_provider_reload().lock() {
        *slot = Some(GlobalProviderReloadRegistration { token, handle });
    }
    ProviderReloadGlobalGuard { token }
}

impl Drop for ProviderReloadGlobalGuard {
    fn drop(&mut self) {
        if let Ok(mut slot) = global_provider_reload().lock() {
            if slot
                .as_ref()
                .is_some_and(|registration| registration.token == self.token)
            {
                *slot = None;
            }
        }
    }
}

pub fn current_provider_reload_document() -> Option<Arc<adl::AdlDoc>> {
    global_provider_reload().lock().ok().and_then(|slot| {
        slot.as_ref()
            .map(|registration| registration.handle.current_document())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(kind: &str) -> adl::ProviderSpec {
        adl::ProviderSpec {
            id: Some("primary".to_string()),
            profile: None,
            kind: kind.to_string(),
            base_url: None,
            default_model: Some("mock-model".to_string()),
            config: HashMap::new(),
        }
    }

    #[test]
    fn provider_reload_sidecar_rejects_credential_values() {
        let mut provider = provider("mock");
        provider.config.insert(
            "api_key".to_string(),
            Value::String("ADL_PROVIDER_TOKEN".to_string()),
        );
        let mut providers = HashMap::new();
        providers.insert("primary".to_string(), provider);

        let err = reject_credential_values(&providers).expect_err("credential value rejected");
        assert!(err.to_string().contains("credential value"));
    }

    #[test]
    fn provider_reload_sidecar_rejects_raw_credential_values_under_neutral_keys() {
        let mut provider = provider("mock");
        provider.config.insert(
            "auth".to_string(),
            serde_json::json!({
                "type": "bearer",
                "value": "sk-test-012345678901234567890123456789"
            }),
        );
        let mut providers = HashMap::new();
        providers.insert("primary".to_string(), provider);

        let err = reject_credential_values(&providers).expect_err("raw credential rejected");
        assert!(err.to_string().contains("credential value"));
    }

    #[test]
    fn provider_reload_sidecar_allows_env_reference_fields() {
        let mut provider = provider("mock");
        provider.config.insert(
            "auth".to_string(),
            serde_json::json!({
                "type": "bearer_env",
                "env": "ADL_PROVIDER_TOKEN"
            }),
        );
        let mut providers = HashMap::new();
        providers.insert("primary".to_string(), provider);

        reject_credential_values(&providers).expect("env reference field accepted");
    }

    #[test]
    fn provider_reload_sidecar_digest_is_stable_and_redacted() {
        let mut providers = HashMap::new();
        providers.insert("primary".to_string(), provider("mock"));
        let first = redacted_provider_digest(&providers).expect("digest");
        let second = redacted_provider_digest(&providers).expect("digest");
        assert_eq!(first, second);
    }
}
