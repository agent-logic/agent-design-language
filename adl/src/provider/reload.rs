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
    let sidecar: ProviderReloadSidecar = serde_yaml::from_str(raw).map_err(|_| {
        record_diagnostic(
            diagnostic,
            generation.load(Ordering::SeqCst),
            "parse_error",
            "provider sidecar parse rejected; input details <redacted>",
        );
        ConfigReloadError::parse("provider sidecar parse rejected; input details <redacted>")
    })?;
    materialize_provider_reload_snapshot(source, sidecar, active_document, diagnostic, generation)
        .map_err(|_| {
            record_diagnostic(
                diagnostic,
                generation.load(Ordering::SeqCst),
                "validation_error",
                "provider sidecar validation rejected; input details <redacted>",
            );
            ConfigReloadError::validation(
                "provider sidecar validation rejected; input details <redacted>",
            )
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
    validate_provider_specs(&activation.document.providers)?;
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
        // Constructors validate adapter configuration without performing inference,
        // resolving credential values, launching processes or creating resources.
        // Keep this before promotion so dispatch cannot discover an invalid endpoint
        // only after the last-known-good definition has already been replaced.
        let _ = super::build_provider_for_id(provider_id, spec, None)?;
    }
    Ok(())
}

fn reject_credential_values(providers: &HashMap<String, adl::ProviderSpec>) -> Result<()> {
    for spec in providers.values() {
        // These declared strings select transport, identity or credential references.
        // Adapter cfg_str helpers treat malformed values as absent; admission must
        // reject them before that fallback can silently change dispatch behavior.
        for key in [
            "endpoint",
            "provider_model_id",
            "model",
            "vendor",
            "local_shadow_model",
            "local_shadow_provider_kind",
            "local_shadow_rule_set",
            "local_shadow_evidence_path",
            "api_key_env",
            "auth_env",
            "token_env",
        ] {
            if spec.config.get(key).is_some_and(|value| !value.is_string()) {
                return Err(anyhow!(
                    "provider reload sidecar has invalid declared string field"
                ));
            }
        }
        // Typed identity fields may be long model/profile names, but not raw keys.
        for value in [
            spec.id.as_deref(),
            spec.profile.as_deref(),
            spec.base_url.as_deref(),
            spec.default_model.as_deref(),
            Some(spec.kind.as_str()),
        ]
        .into_iter()
        .flatten()
        {
            if has_credential_marker(value) {
                return Err(anyhow!("provider reload sidecar contains credential value"));
            }
        }
        let value =
            serde_json::to_value(&spec.config).context("serialize provider reload config")?;
        reject_credential_value_at(&[], &value)?;
    }
    Ok(())
}

fn reject_credential_value_at(path: &[&str], value: &Value) -> Result<()> {
    if matches!(
        path,
        ["expected_account_sha256"] | ["expected-account-sha256"]
    ) {
        let valid = value
            .as_str()
            .is_some_and(|raw| raw.len() == 64 && raw.bytes().all(|b| b.is_ascii_hexdigit()));
        if !valid {
            return Err(anyhow!(
                "provider reload sidecar has invalid public account digest"
            ));
        }
        return Ok(());
    }
    if path == ["auth", "env"] && !value.is_string() {
        return Err(anyhow!(
            "provider reload sidecar has invalid credential reference"
        ));
    }
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if credential_value_key(key) {
                    return Err(anyhow!("provider reload sidecar contains credential value"));
                }
                let mut next = path.to_vec();
                next.push(key);
                reject_credential_value_at(&next, value)?;
            }
        }
        Value::Array(values) => {
            let mut next = path.to_vec();
            next.push("[]");
            for value in values {
                reject_credential_value_at(&next, value)?;
            }
        }
        Value::String(raw) => {
            let reference = matches!(
                path,
                ["auth", "env"] | ["api_key_env"] | ["auth_env"] | ["token_env"]
            );
            if reference {
                let valid = !raw.is_empty()
                    && raw.bytes().enumerate().all(|(i, b)| {
                        b == b'_' || b.is_ascii_alphabetic() || (i > 0 && b.is_ascii_digit())
                    });
                if !valid || has_credential_marker(raw) {
                    return Err(anyhow!(
                        "provider reload sidecar has invalid credential reference"
                    ));
                }
            } else {
                let declared_data = matches!(
                    path,
                    ["provider_model_id"]
                        | ["model"]
                        | ["local_shadow_model"]
                        | ["local_shadow_evidence_path"]
                        | ["local_shadow_rule_set"]
                );
                if has_credential_marker(raw) || (!declared_data && looks_like_raw_credential(raw))
                {
                    return Err(anyhow!("provider reload sidecar contains credential value"));
                }
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

fn looks_like_raw_credential(raw: &str) -> bool {
    let trimmed = raw.trim();
    has_credential_marker(raw)
        || (trimmed.len() >= 32
            && trimmed
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.')))
}

fn has_credential_marker(raw: &str) -> bool {
    url_contains_credentials(raw) || explicit_credential_marker(raw)
}

fn explicit_credential_marker(raw: &str) -> bool {
    let lower = raw.trim().to_ascii_lowercase();
    lower.starts_with("sk-")
        || lower.starts_with("bearer ")
        || lower.contains("-----begin private key-----")
        || lower.contains("-----begin rsa private key-----")
        || lower.contains("-----begin ec private key-----")
}

// URL parsing decodes query names before policy matching. Ordinary paths and
// query parameters remain instance data; credentials belong in references.
fn url_contains_credentials(raw: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(raw.trim()) else {
        return false;
    };
    !url.username().is_empty()
        || url.password().is_some()
        || url.query_pairs().any(|(name, value)| {
            let normalized = name.to_ascii_lowercase().replace('-', "_");
            explicit_credential_marker(&value)
                || credential_value_key(&normalized)
                || matches!(
                    normalized.as_str(),
                    "key" | "auth" | "authorization" | "bearer" | "auth_token" | "passwd"
                )
        })
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
    use std::time::UNIX_EPOCH;

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

    fn provider_doc(default_model: &str) -> adl::AdlDoc {
        let mut providers = HashMap::new();
        let mut provider = provider("mock");
        provider.default_model = Some(default_model.to_string());
        providers.insert("primary".to_string(), provider);

        adl::AdlDoc {
            version: "0.5".to_string(),
            providers,
            tools: HashMap::new(),
            agents: HashMap::new(),
            tasks: HashMap::new(),
            workflows: HashMap::new(),
            patterns: vec![],
            signature: None,
            run: adl::RunSpec {
                id: None,
                name: None,
                created_at: None,
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: Some(adl::WorkflowSpec {
                    id: None,
                    kind: adl::WorkflowKind::Sequential,
                    max_concurrency: None,
                    steps: vec![],
                }),
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        }
    }

    fn provider_reload_sidecar(default_model: &str) -> String {
        format!(
            r#"schema: adl.provider_reload_sidecar.v1
version: "0.5"
providers:
  primary:
    type: mock
    default_model: {default_model}
"#
        )
    }

    fn unique_temp_path(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
    }

    fn current_global_provider_model() -> Option<String> {
        current_provider_reload_document().and_then(|doc| {
            doc.providers
                .get("primary")
                .and_then(|provider| provider.default_model.clone())
        })
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

    // #876 PVF: deterministic loader contract; bounded filesystem/CPU; required
    // provider-platform security and last-known-good gate, no network or secrets.
    #[test]
    fn provider_definitions_reject_nested_credentials_and_redact_loader_errors() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let base = unique_temp_path("adl-provider-definitions-negatives");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("providers.yaml");
        let secret = "sk-fixture-only-012345678901234567890123456789";
        let cases = [
            format!("providers: {{primary: {{type: mock, config: {{neutral: [{{nested: '{secret}'}}]}}}}}}"),
            format!("providers: {{primary: {{type: mock, config: {{neutral: {{config: {{model: '{secret}'}}}}}}}}}}"),
            "providers: {primary: {type: mock, config: {neutral: [ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890]}}}".to_string(),
            format!("providers: {{primary: {{type: mock, config: {{auth: {{env: '{secret}'}}}}}}}}"),
            format!("providers: {{primary: {{type: mock, unknown: '{secret}'}}}}"),
            format!("providers: {{primary: {{type: ['{secret}']}}}}"),
            "schema: unsupported\nproviders: {}".to_string(),
            "providers: {primary: {type: unsupported}}".to_string(),
            "providers: {primary: {}}".to_string(),
            "providers: {primary: {type: http}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {endpoint: 123}}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {endpoint: null}}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {endpoint: false}}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {endpoint: []}}}".to_string(),
            "providers: {primary: {type: mock, config: {model: 123}}}".to_string(),
            "providers: {primary: {type: mock, config: {provider_model_id: false}}}".to_string(),
            "providers: {primary: {type: mock, config: {auth: {env: 123}}}}".to_string(),
            "providers: {primary: {type: mock, config: {local_shadow_model: 123}}}".to_string(),
            "providers: {primary: {type: mock, config: {neutral: {model: ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890}}}}".to_string(),
            "providers: {primary: {type: mock, config: {local_shadow_model: echo, local_shadow_evidence_path: /absolute/forbidden.jsonl}}}".to_string(),
            "providers: {primary: {type: mock, config: {local_shadow_model: echo, local_shadow_evidence_path: ../forbidden.jsonl}}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {endpoint: invalid}}}".to_string(),
            "providers: {primary: {type: http, config: {endpoint: 'http://127.0.0.1:1', auth: {type: unsupported, env: ADL_FIXTURE_TOKEN}}}}".to_string(),
            "providers: {primary: {profile: 'unknown:profile'}}".to_string(),
            "providers: {primary: {profile: 'ollama:phi4-mini', config: {temperature: 9}}}".to_string(),
        ];
        for candidate in cases {
            std::fs::write(&path, &candidate).unwrap();
            let error = match runtime.block_on(ProviderReloadOwner::start(
                path.clone(),
                provider_doc("base"),
                ConfigReloadOptions::default(),
            )) {
                Err(error) => error.to_string(),
                Ok(_) => panic!("invalid initial definition accepted"),
            };
            assert!(!error.contains(secret));
            assert!(error.len() < 256);
            // The identical parser used by the watcher must preserve its whole
            // active document and generation, including on deserialization errors.
            let active = Arc::new(Mutex::new(provider_doc("retained")));
            let diagnostic = Arc::new(Mutex::new(None));
            let generation = Arc::new(AtomicU64::new(7));
            assert!(parse_provider_reload_snapshot(
                &path,
                &candidate,
                &active,
                &diagnostic,
                &generation
            )
            .is_err());
            assert_eq!(generation.load(Ordering::SeqCst), 7);
            assert_eq!(
                active.lock().unwrap().providers["primary"]
                    .default_model
                    .as_deref(),
                Some("retained")
            );
            let diagnostic = diagnostic.lock().unwrap().clone().unwrap();
            assert!(!diagnostic.redacted_message.contains(secret));
            assert!(diagnostic.redacted_message.len() < 128);
        }
        let mut compatible = provider("mock");
        compatible.default_model =
            Some("legitimate-long-model-identifier-version-20260912".to_string());
        compatible.config.insert("auth".to_string(), serde_json::json!({"type":"bearer", "env":"ADL_VERY_LONG_APPROVED_PROVIDER_TOKEN_ENVIRONMENT"}));
        reject_credential_values(&HashMap::from([("primary".to_string(), compatible)])).unwrap();
        std::fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn provider_definitions_validate_references_without_resolving_or_writing_them() {
        // #876 PVF: deterministic admission compatibility, bounded filesystem;
        // no dispatch, secret resolution, shadow evidence, or network request.
        let base = unique_temp_path("adl-provider-definitions-references");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("providers.yaml");
        std::fs::write(
            &path,
            r#"providers:
  primary:
    type: http
    default_model: legitimate-long-model-identifier-version-20260912
    config:
      endpoint: http://127.0.0.1:1/unused
      auth:
        type: bearer
        env: ADL_876_UNSET_PROVIDER_REFERENCE_FOR_ADMISSION_TEST
      local_shadow_model: echo
      local_shadow_provider_kind: mock
      local_shadow_evidence_path: never-written-876-admission.jsonl
"#,
        )
        .unwrap();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let owner = runtime
            .block_on(ProviderReloadOwner::start(
                path,
                provider_doc("base"),
                ConfigReloadOptions::default(),
            ))
            .expect("construction must not resolve auth or contact endpoint");
        assert!(!Path::new("never-written-876-admission.jsonl").exists());
        runtime.block_on(owner.shutdown()).unwrap();
        let mut spec = provider("mock");
        spec.config.insert(
            "expected_account_sha256".to_string(),
            Value::String("a".repeat(64)),
        );
        reject_credential_values(&HashMap::from([("primary".to_string(), spec)])).unwrap();
        std::fs::remove_dir_all(base).unwrap();
    }

    // #876 PVF: local admission contract; no inference, AWS calls or credentials.
    #[test]
    fn provider_definitions_account_aliases_and_embedded_url_credentials() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let base = unique_temp_path("adl-provider-url-alias");
        std::fs::create_dir_all(&base).unwrap();
        let path = base.join("providers.yaml");
        let hash = "a".repeat(64);
        for key in ["expected_account_sha256", "expected-account-sha256"] {
            let mut spec = provider("bedrock");
            spec.config
                .insert("region".into(), Value::String("us-west-2".into()));
            spec.config.insert(key.into(), Value::String(hash.clone()));
            // Prove the existing Bedrock constructor consumes each public alias.
            reject_credential_values(&HashMap::from([("primary".into(), spec.clone())])).unwrap();
            super::super::build_provider_for_id("primary", &spec, None).unwrap();
            std::fs::write(&path, format!("providers: {{primary: {{profile: 'bedrock:nova-lite-v1', config: {{region: us-west-2, {key}: '{hash}'}}}}}}" )).unwrap();
            let owner = runtime
                .block_on(ProviderReloadOwner::start(
                    path.clone(),
                    provider_doc("base"),
                    ConfigReloadOptions::default(),
                ))
                .unwrap();
            assert_eq!(
                owner.handle().current_document().providers["primary"].config[key],
                hash
            );
            runtime.block_on(owner.shutdown()).unwrap();

            for value in [
                Value::Null,
                serde_json::json!(123),
                serde_json::json!(false),
                serde_json::json!([]),
                serde_json::json!("bad"),
                Value::String("g".repeat(64)),
            ] {
                spec.config.insert(key.into(), value);
                assert!(reject_credential_values(&HashMap::from([(
                    "primary".into(),
                    spec.clone()
                )]))
                .is_err());
            }
        }
        let urls = [
            "http://user:password@127.0.0.1:1/api/generate",
            "http://127.0.0.1:1/api/generate?api_key=sk-fixture",
            "http://127.0.0.1:1/api/generate?api%5Fkey=fixture",
            "https://example.invalid/v1?%61uthorization=fixture",
            "https://example.invalid/v1?key=fixture",
            "https://example.invalid/v1?route=%73k-fixture",
        ];
        for url in urls {
            for field in [
                format!("base_url: '{url}'"),
                format!("config: {{endpoint: '{url}'}}"),
            ] {
                std::fs::write(
                    &path,
                    format!("providers: {{primary: {{type: http, {field}}}}}"),
                )
                .unwrap();
                let error = runtime
                    .block_on(ProviderReloadOwner::start(
                        path.clone(),
                        provider_doc("base"),
                        ConfigReloadOptions::default(),
                    ))
                    .unwrap_err();
                let message = error.to_string();
                assert!(!message.contains(url));
                assert!(!message.contains("sk-fixture"));
            }
        }
        for url in [
            "http://127.0.0.1:1/api/generate?version=1&model=ordinary",
            "https://example.invalid/a%20b?route=chat&api-version=2026-01",
        ] {
            std::fs::write(
                &path,
                format!("providers: {{primary: {{type: http, config: {{endpoint: '{url}'}}}}}}"),
            )
            .unwrap();
            let owner = runtime
                .block_on(ProviderReloadOwner::start(
                    path.clone(),
                    provider_doc("base"),
                    ConfigReloadOptions::default(),
                ))
                .unwrap();
            runtime.block_on(owner.shutdown()).unwrap();
        }
        std::fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn provider_reload_sidecar_digest_is_stable_and_redacted() {
        let mut providers = HashMap::new();
        providers.insert("primary".to_string(), provider("mock"));
        let first = redacted_provider_digest(&providers).expect("digest");
        let second = redacted_provider_digest(&providers).expect("digest");
        assert_eq!(first, second);
    }

    #[test]
    fn global_provider_reload_guard_only_clears_owned_registration() {
        let base = unique_temp_path("adl-provider-reload-global-guard");
        std::fs::create_dir_all(&base).expect("create base");
        let sidecar_a = base.join("providers-a.yaml");
        let sidecar_b = base.join("providers-b.yaml");
        std::fs::write(&sidecar_a, provider_reload_sidecar("workflow-a-model"))
            .expect("write sidecar a");
        std::fs::write(&sidecar_b, provider_reload_sidecar("workflow-b-model"))
            .expect("write sidecar b");

        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        let owner_a = runtime
            .block_on(ProviderReloadOwner::start(
                sidecar_a,
                provider_doc("base-a-model"),
                ConfigReloadOptions::default(),
            ))
            .expect("start owner a");
        let owner_b = runtime
            .block_on(ProviderReloadOwner::start(
                sidecar_b,
                provider_doc("base-b-model"),
                ConfigReloadOptions::default(),
            ))
            .expect("start owner b");

        let guard_a = set_global_provider_reload_handle(owner_a.handle());
        assert_eq!(
            current_global_provider_model().as_deref(),
            Some("workflow-a-model")
        );

        let guard_b = set_global_provider_reload_handle(owner_b.handle());
        assert_eq!(
            current_global_provider_model().as_deref(),
            Some("workflow-b-model")
        );

        drop(guard_a);
        assert_eq!(
            current_global_provider_model().as_deref(),
            Some("workflow-b-model"),
            "dropping an older guard must not clear a newer global registration"
        );

        drop(guard_b);
        assert_eq!(current_global_provider_model(), None);

        let outcome_b = runtime.block_on(owner_b.shutdown()).expect("shutdown b");
        assert!(outcome_b.shutdown_requested);
        let outcome_a = runtime.block_on(owner_a.shutdown()).expect("shutdown a");
        assert!(outcome_a.shutdown_requested);
    }
}
