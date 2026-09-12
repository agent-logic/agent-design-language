//! ADL document projection over canonical provider substrate types.
use crate::{adl, provider};
pub use adl_provider_core::provider_substrate::*;
use anyhow::Result;
use serde_json::{json, Value};
pub fn provider_substrate_manifest_v1(doc: &adl::AdlDoc) -> Result<Value> {
    let expanded = provider::expand_provider_profiles(doc)?;
    let mut ids: Vec<&String> = expanded.providers.keys().collect();
    ids.sort();
    let providers = ids
        .into_iter()
        .map(|provider_id| provider_substrate_v1(provider_id, &expanded.providers[provider_id]))
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({
        "schema_name": PROVIDER_SUBSTRATE_MANIFEST_SCHEMA,
        "schema_version": 1,
        "providers": providers,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    fn provider_spec(kind: &str) -> adl::ProviderSpec {
        adl::ProviderSpec {
            id: None,
            profile: None,
            kind: kind.into(),
            base_url: None,
            default_model: None,
            config: HashMap::new(),
        }
    }
    #[test]
    fn provider_substrate_manifest_is_sorted_and_stable() {
        let mut providers = HashMap::new();
        providers.insert("b".to_string(), provider_spec("mock"));
        providers.insert("a".to_string(), provider_spec("mock"));
        let doc = adl::AdlDoc {
            version: "0.5".to_string(),
            providers,
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
                defaults: adl::RunDefaults::default(),
                workflow_ref: None,
                workflow: None,
                pattern_ref: None,
                inputs: HashMap::new(),
                placement: None,
                remote: None,
                delegation_policy: None,
            },
        };

        let manifest = provider_substrate_manifest_v1(&doc).expect("manifest");
        let providers = manifest
            .get("providers")
            .and_then(|v| v.as_array())
            .expect("providers array");
        assert_eq!(
            providers[0].get("provider_id").and_then(|v| v.as_str()),
            Some("a")
        );
        assert_eq!(
            providers[1].get("provider_id").and_then(|v| v.as_str()),
            Some("b")
        );
    }
}
