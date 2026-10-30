use adl::provider::{
    complete_with_local_model_shadow, Provider, ProviderExecutionChannel, ProviderShadowInput,
    ProviderShadowObservationClass,
};
use anyhow::Result;

struct EchoProvider(&'static str);

impl Provider for EchoProvider {
    fn complete(&self, prompt: &str) -> Result<String> {
        Ok(format!("{}:{prompt}", self.0))
    }
}

#[test]
fn local_model_shadow_records_exact_shared_input_digest_and_rule_set() {
    let authority = EchoProvider("authority");
    let shadow = EchoProvider("shadow");
    let input = ProviderShadowInput::new(
        "same declared input for both provider paths",
        "prov_b_exact_input_digest_v1",
    )
    .expect("valid shadow input");

    let result = complete_with_local_model_shadow(&authority, Some(&shadow), input)
        .expect("shadowed completion should succeed");

    assert_eq!(
        result.comparison.authority_input_digest, result.comparison.shadow_input_digest,
        "authority and shadow must compare the same declared input"
    );
    assert!(result
        .comparison
        .authority_input_digest
        .starts_with("sha256:"));
    assert_eq!(
        result.comparison.comparison_rule_set,
        "prov_b_exact_input_digest_v1"
    );
    assert_eq!(
        result.comparison.authority_channel,
        ProviderExecutionChannel::Authoritative
    );
    assert_eq!(
        result.comparison.shadow_channel,
        ProviderExecutionChannel::Shadow
    );
    assert_eq!(
        result.comparison.shadow_observation_class,
        ProviderShadowObservationClass::Completed
    );
    assert!(result.comparison.redaction.prompt_redacted);
    assert!(result.comparison.redaction.output_redacted);
    assert!(result.comparison.redaction.credential_material_redacted);
    assert!(result.comparison.redaction.host_paths_redacted);
}
