use adl::provider::{
    complete_with_local_model_shadow, OllamaProvider, Provider, ProviderExecutionChannel,
    ProviderShadowInput, ProviderShadowObservationClass,
};
use anyhow::Result;

struct AuthoritativeReviewProvider {
    accepted_review: &'static str,
}

impl Provider for AuthoritativeReviewProvider {
    fn complete(&self, _prompt: &str) -> Result<String> {
        Ok(self.accepted_review.to_string())
    }
}

#[test]
#[ignore = "requires a local Ollama daemon and an explicitly selected local model"]
fn local_ollama_shadow_reviews_open_pr_prompts_without_authority() {
    let shadow_model = std::env::var("ADL_PROVIDER_SHADOW_OPEN_PR_MODEL")
        .expect("set ADL_PROVIDER_SHADOW_OPEN_PR_MODEL to a local Ollama model");
    let shadow = OllamaProvider {
        model: shadow_model,
        temperature: Some(0.0),
    };

    for (pr, prompt, accepted_review) in open_pr_review_prompts() {
        let authority = AuthoritativeReviewProvider { accepted_review };
        let input = ProviderShadowInput::new(prompt, format!("open_pr_{pr}_shadow_review_v1"))
            .expect("valid open PR shadow-review input");

        let result = complete_with_local_model_shadow(&authority, Some(&shadow), input)
            .unwrap_or_else(|err| panic!("PR #{pr} shadow review execution failed: {err:#}"));

        assert_eq!(
            result.authoritative.channel(),
            ProviderExecutionChannel::Authoritative
        );
        assert_eq!(result.authoritative.output, accepted_review);
        assert_eq!(result.shadow.channel(), ProviderExecutionChannel::Shadow);
        assert_eq!(
            result.shadow.observation_class,
            ProviderShadowObservationClass::Completed,
            "PR #{pr} local shadow review did not complete"
        );
        assert!(
            result.shadow.output_digest.is_some(),
            "PR #{pr} shadow output digest was not recorded"
        );
        assert_eq!(result.shadow.failure_kind, None);
        assert_eq!(result.comparison.authority_outcome_class, "completed");

        let evidence = result
            .redacted_evidence()
            .expect("serialize redacted shadow evidence");
        let evidence_json =
            serde_json::to_string(&evidence).expect("redacted evidence serializes to JSON");
        assert!(
            !evidence_json.contains("The Cognitive Stack")
                && !evidence_json.contains("csmctl")
                && !evidence_json.contains("audio")
                && !evidence_json.contains("AUTHORITATIVE_REVIEW"),
            "PR #{pr} evidence leaked prompt or authority payload: {evidence_json}"
        );
        println!("PR #{pr} redacted provider-shadow evidence: {evidence_json}");
    }
}

fn open_pr_review_prompts() -> Vec<(u64, &'static str, &'static str)> {
    vec![
        (
            618,
            "Review open PR #618, '[v0.92][podcast] Publish The Cognitive Stack hosting bundle'. Changed surfaces include C-SDLC issue #262 evidence/cards, podcast HTTP playback evidence, podcast launch packet validators, demo podcast HTML/RSS/audio manifests, episode script/show notes/transcript, and storage/browser playback proof. Return concise findings-first review observations. This is a non-authoritative local shadow review; do not claim merge authority.",
            "AUTHORITATIVE_REVIEW_FOR_PR_618_REMAINS_ACCEPTED",
        ),
        (
            614,
            "Review open PR #614, '[v0.92.1][Runtime] Add config-driven agent lifecycle to csmctl'. Changed surfaces include C-SDLC issue #602 evidence/cards, csmctl command code, runtime-kernel assembly/control/feed paths, runtime init config, OpenAPI observatory contract, and focused/live-Wuji validation records. Return concise findings-first review observations. This is a non-authoritative local shadow review; do not claim merge authority.",
            "AUTHORITATIVE_REVIEW_FOR_PR_614_REMAINS_ACCEPTED",
        ),
    ]
}
