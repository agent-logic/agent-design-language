//! Shared complete-map sidecar validation; no watcher or lifecycle state.
use crate::{provider_substrate, ProviderMap, ProviderSpec};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
pub const PROVIDER_RELOAD_SIDECAR_SCHEMA: &str = "adl.provider_reload_sidecar.v1";
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderReloadSidecar {
    #[serde(default)]
    pub schema: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub providers: HashMap<String, ProviderSpec>,
}

#[derive(Debug, Clone)]
pub struct ValidatedProviderCandidate {
    pub version: Option<String>,
    pub providers: ProviderMap,
    pub digest: String,
}
/// Parse and validate an entire replacement map without exposing input in errors.
pub fn parse_validated_provider_sidecar(raw: &str) -> Result<ValidatedProviderCandidate> {
    let sidecar: ProviderReloadSidecar = serde_yaml::from_str(raw)
        .map_err(|_| anyhow!("provider sidecar parse rejected; input details <redacted>"))?;
    validate_provider_sidecar(sidecar)
        .map_err(|_| anyhow!("provider sidecar validation rejected; input details <redacted>"))
}
pub fn validate_provider_sidecar(
    sidecar: ProviderReloadSidecar,
) -> Result<ValidatedProviderCandidate> {
    if sidecar
        .schema
        .as_deref()
        .is_some_and(|s| s != PROVIDER_RELOAD_SIDECAR_SCHEMA)
    {
        return Err(anyhow!("provider reload sidecar schema is unsupported"));
    }
    let providers = validate_provider_candidate(&sidecar.providers)?;
    let digest = redacted_provider_digest(&providers)?;
    Ok(ValidatedProviderCandidate {
        version: sidecar.version,
        providers,
        digest,
    })
}
/// Reject secrets before expansion and construct every adapter before promotion.
pub fn validate_provider_candidate(providers: &ProviderMap) -> Result<ProviderMap> {
    if providers.is_empty() {
        return Err(anyhow!("provider reload sidecar must declare providers"));
    }
    reject_credential_values(providers)?;
    let activation =
        crate::profiles::activate_provider_profile_candidate(&ProviderMap::new(), providers)?;
    if !activation.accepted {
        return Err(anyhow!("provider profile candidate rejected"));
    }
    reject_credential_values(&activation.document)?;
    validate_provider_specs(&activation.document)?;
    Ok(activation.document)
}
pub fn validate_provider_specs(providers: &HashMap<String, ProviderSpec>) -> Result<()> {
    for (provider_id, spec) in providers {
        provider_substrate::provider_substrate_v1(provider_id, spec)
            .with_context(|| format!("validate provider reload spec '{provider_id}'"))?;
        // Constructors validate adapter configuration without performing inference,
        // resolving credential values, launching processes or creating resources.
        // Keep this before promotion so dispatch cannot discover an invalid endpoint
        // only after the last-known-good definition has already been replaced.
        let _ = crate::build_provider_for_id(provider_id, spec, None)?;
    }
    Ok(())
}

pub fn reject_credential_values(providers: &HashMap<String, ProviderSpec>) -> Result<()> {
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

pub fn redacted_provider_digest(providers: &HashMap<String, ProviderSpec>) -> Result<String> {
    let ordered = providers
        .iter()
        .map(|(provider_id, spec)| (provider_id.clone(), spec.clone()))
        .collect::<BTreeMap<_, _>>();
    let bytes = serde_json::to_vec(&ordered).context("serialize redacted provider digest input")?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    // PVF: deterministic local contract proof, bounded CPU, no hosted calls;
    // release gate tracked in issue 855's provider extraction proof inventory.
    #[test]
    fn complete_map_validation_rejects_entire_candidate_and_redacts_input() {
        let good = r#"providers:
  good:
    type: mock
    default_model: local
"#;
        let active = parse_validated_provider_sidecar(good).unwrap();
        assert_eq!(active.providers.len(), 1);
        for invalid in [
            "  invalid: {type: not-a-provider}",
            "  invalid: {type: http, base_url: 'https://user:password@api.example.com', default_model: x}",
            "  invalid: {type: mock, config: {api_key: secret-do-not-echo}}",
            "  invalid: {type: openai, config: {endpoint: 123}}",
        ] {
            let error = parse_validated_provider_sidecar(&format!("{good}{invalid}\n")).unwrap_err();
            assert_eq!(error.to_string(), "provider sidecar validation rejected; input details <redacted>");
            assert_eq!(active.providers.len(), 1);
        }
        let replacement =
            parse_validated_provider_sidecar("providers: {replacement: {type: mock}}").unwrap();
        assert!(!replacement.providers.contains_key("good"));
    }
}
