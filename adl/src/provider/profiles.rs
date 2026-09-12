use crate::adl;
use anyhow::Result;
use serde_json::Value;
#[derive(Debug, Clone, PartialEq)]
pub struct ProviderProfileActivation {
    pub document: adl::AdlDoc,
    pub accepted: bool,
    pub rejection: Option<String>,
}
pub fn expand_provider_profiles(doc: &adl::AdlDoc) -> Result<adl::AdlDoc> {
    let mut expanded = doc.clone();
    expanded.providers = adl_provider_core::profiles::expand_provider_profiles(&doc.providers)?;
    Ok(expanded)
}
pub fn provider_profile_materialization_projection(doc: &adl::AdlDoc) -> Result<Value> {
    adl_provider_core::profiles::provider_profile_materialization_projection(&doc.providers)
}
pub fn activate_provider_profile_candidate(
    active: &adl::AdlDoc,
    candidate: &adl::AdlDoc,
) -> Result<ProviderProfileActivation> {
    let activation = adl_provider_core::profiles::activate_provider_profile_candidate(
        &active.providers,
        &candidate.providers,
    )?;
    let mut document = if activation.accepted {
        candidate.clone()
    } else {
        active.clone()
    };
    document.providers = activation.document;
    Ok(ProviderProfileActivation {
        document,
        accepted: activation.accepted,
        rejection: activation.rejection,
    })
}
