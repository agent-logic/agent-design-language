use adl::provider::{
    complete_with_local_model_shadow, Provider, ProviderShadowInput, ProviderShadowObservationClass,
};
use anyhow::{anyhow, Result};

struct OkProvider(&'static str);

impl Provider for OkProvider {
    fn complete(&self, _prompt: &str) -> Result<String> {
        Ok(self.0.to_string())
    }
}

struct FailingProvider(&'static str);

impl Provider for FailingProvider {
    fn complete(&self, _prompt: &str) -> Result<String> {
        Err(anyhow!(self.0))
    }
}

struct PanickingProvider;

impl Provider for PanickingProvider {
    fn complete(&self, _prompt: &str) -> Result<String> {
        panic!("shadow panic payload must not escape");
    }
}

#[test]
fn local_model_shadow_failure_preserves_authoritative_result() {
    let authority = OkProvider("AUTHORITATIVE_RESULT_SURVIVES");
    let shadow = FailingProvider("shadow fixture failed");
    let input = ProviderShadowInput::new("fallback input", "prov_b_shadow_failure_v1")
        .expect("valid shadow input");

    let result = complete_with_local_model_shadow(&authority, Some(&shadow), input)
        .expect("shadow failure must not fail authoritative completion");

    assert_eq!(result.authoritative.output, "AUTHORITATIVE_RESULT_SURVIVES");
    assert_eq!(
        result.shadow.observation_class,
        ProviderShadowObservationClass::Failed
    );
    assert_eq!(result.shadow.output_digest, None);
    assert_eq!(
        result.shadow.failure_kind.as_deref(),
        Some("provider_error")
    );
    assert_eq!(result.comparison.authority_outcome_class, "completed");
}

#[test]
fn authoritative_failure_is_not_masked_by_shadow_success() {
    let authority = FailingProvider("authoritative failure");
    let shadow = OkProvider("SHADOW_SUCCESS_IS_NOT_AUTHORITY");
    let input = ProviderShadowInput::new("authority failure input", "prov_b_authority_first_v1")
        .expect("valid shadow input");

    let err = complete_with_local_model_shadow(&authority, Some(&shadow), input)
        .expect_err("authoritative failure must stay authoritative");

    assert!(
        format!("{err:#}").contains("authoritative failure"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn local_model_shadow_panic_preserves_authoritative_result() {
    let authority = OkProvider("AUTHORITATIVE_RESULT_SURVIVES_PANIC");
    let shadow = PanickingProvider;
    let input = ProviderShadowInput::new("panic fallback input", "prov_b_shadow_panic_failure_v1")
        .expect("valid shadow input");

    let result = complete_with_local_model_shadow(&authority, Some(&shadow), input)
        .expect("shadow panic must not fail authoritative completion");

    assert_eq!(
        result.authoritative.output,
        "AUTHORITATIVE_RESULT_SURVIVES_PANIC"
    );
    assert_eq!(
        result.shadow.observation_class,
        ProviderShadowObservationClass::Failed
    );
    assert_eq!(result.shadow.output_digest, None);
    assert_eq!(result.shadow.failure_kind.as_deref(), Some("panic"));
    assert_eq!(result.comparison.authority_outcome_class, "completed");
}
