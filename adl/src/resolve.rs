//! Runtime admission/expansion adapter over the public deterministic resolver.
pub use adl_legacy_contracts::resolve::{print_resolved_plan, AdlResolved, ResolvedStep};
use anyhow::{Context, Result};

pub fn resolve_run(doc: &crate::adl::AdlDoc) -> Result<AdlResolved> {
    let expanded = crate::provider::expand_provider_profiles(doc)
        .context("failed to expand provider profiles")?;
    adl_legacy_contracts::resolve::resolve_run(&expanded)
}

#[cfg(test)]
mod tests {
    // PVF: deterministic Runtime adapter contract, CPU-only, required RD03 gate.
    #[test]
    fn expanded_runtime_profile_resolves_without_losing_provenance() {
        let doc: crate::adl::AdlDoc = serde_yaml::from_str(
            r#"
version: "0.5"
providers:
  local:
    profile: "ollama:phi4-mini"
agents:
  worker:
    provider: local
    model: phi4-mini
run:
  name: profile-adapter
  workflow:
    kind: sequential
    steps:
      - id: first
        agent: worker
        prompt:
          user: hello
"#,
        )
        .unwrap();
        let resolved = super::resolve_run(&doc).unwrap();
        assert_eq!(resolved.steps.len(), 1);
        assert_eq!(
            resolved.doc.providers["local"].profile.as_deref(),
            Some("ollama:phi4-mini")
        );
        assert!(!resolved.doc.providers["local"].kind.is_empty());
    }
}
