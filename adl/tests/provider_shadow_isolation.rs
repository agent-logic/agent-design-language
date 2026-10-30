use adl::provider::{
    complete_with_local_model_shadow, Provider, ProviderExecutionChannel, ProviderShadowInput,
    ProviderShadowObservationClass,
};
use anyhow::Result;

struct StaticProvider(&'static str);

impl Provider for StaticProvider {
    fn complete(&self, _prompt: &str) -> Result<String> {
        Ok(self.0.to_string())
    }
}

#[test]
fn local_model_shadow_output_cannot_replace_authoritative_output() {
    let authority = StaticProvider("AUTHORITATIVE_RESULT");
    let shadow = StaticProvider("SHADOW_RESULT_MUST_NOT_ESCAPE");
    let input = ProviderShadowInput::new(
        "private prompt must be represented only by digest",
        "prov_b_exact_input_digest_v1",
    )
    .expect("valid shadow input");

    let result = complete_with_local_model_shadow(&authority, Some(&shadow), input)
        .expect("shadowed completion should preserve authority");

    assert_eq!(
        result.authoritative.channel(),
        ProviderExecutionChannel::Authoritative
    );
    assert_eq!(result.authoritative.output, "AUTHORITATIVE_RESULT");
    assert_eq!(result.shadow.channel(), ProviderExecutionChannel::Shadow);
    assert_eq!(
        result.shadow.observation_class,
        ProviderShadowObservationClass::Completed
    );
    assert_ne!(
        result.authoritative.channel(),
        result.shadow.channel(),
        "authority and shadow channels must stay distinguishable"
    );

    let evidence = result.redacted_evidence().expect("redacted evidence");
    let evidence_text = serde_json::to_string(&evidence).expect("evidence serializes");
    assert!(!evidence_text.contains("AUTHORITATIVE_RESULT"));
    assert!(!evidence_text.contains("SHADOW_RESULT_MUST_NOT_ESCAPE"));
    assert!(!evidence_text.contains("private prompt"));
}
