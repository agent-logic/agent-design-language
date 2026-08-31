use ::adl::provider::{build_provider, expand_provider_profiles, provider_profile_names};

use super::support::adl_doc_from_yaml;

#[test]
fn provider_profiles_registry_is_deterministic_and_has_at_least_twelve_profiles() {
    let names = provider_profile_names();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(
        names, sorted,
        "profile names must be sorted deterministically"
    );
    assert!(
        names.len() >= 12,
        "expected at least 12 profiles, got {}",
        names.len()
    );
}

#[test]
fn expand_provider_profiles_rejects_unknown_profile() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "unknown:profile"
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let err = expand_provider_profiles(&doc).expect_err("unknown profile should fail");
    let msg = err.to_string();
    assert!(
        msg.contains("unknown:profile") && msg.contains("available:"),
        "unexpected error: {msg}"
    );
}

#[test]
fn expand_provider_profiles_rejects_profile_with_explicit_fields() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "ollama:phi4-mini"
    type: "ollama"
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let err = expand_provider_profiles(&doc).expect_err("profile + explicit fields must fail");
    assert!(
        err.to_string()
            .contains("profile and explicit provider identity fields together"),
        "{err:#}"
    );
}

#[test]
fn expand_provider_profiles_rejects_vendor_identity_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  kimi_primary:
    profile: "kimi:k2.5"
    config:
      vendor: "openai"
agents:
  a1:
    provider: "kimi_primary"
    model: "kimi-k2.5"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let err = expand_provider_profiles(&doc).expect_err("profile vendor override must fail");
    assert!(
        err.to_string().contains("conflicts with profile vendor"),
        "{err:#}"
    );
}

#[test]
fn expand_provider_profiles_is_byte_stable_across_runs() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  a_mock:
    profile: "mock:echo-v1"
  z_ollama:
    profile: "ollama:phi4-mini"
agents:
  a1:
    provider: "z_ollama"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded1 = expand_provider_profiles(&doc).expect("expand run 1");
    let expanded2 = expand_provider_profiles(&doc).expect("expand run 2");

    let json1 = serde_json::to_string(&expanded1.providers).expect("serialize providers");
    let json2 = serde_json::to_string(&expanded2.providers).expect("serialize providers");
    assert_eq!(json1, json2, "profile expansion must be byte-stable");

    assert_eq!(
        expanded1.providers["z_ollama"].kind, "ollama",
        "ollama profile should expand to kind=ollama"
    );
    assert_eq!(
        expanded1.providers["a_mock"].kind, "mock",
        "mock profile should expand to kind=mock"
    );
}

#[test]
fn expand_provider_profiles_accepts_zai_glm5_profile() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  zai_primary:
    profile: "z_ai:glm-5"
agents:
  a1:
    provider: "zai_primary"
    model: "hosted:adl-z-ai:glm-5"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("expand z_ai profile");
    let provider = &expanded.providers["zai_primary"];
    assert_eq!(provider.kind, "z_ai");
    assert_eq!(
        provider.default_model.as_deref(),
        Some("hosted:adl-z-ai:glm-5")
    );
    assert_eq!(
        provider
            .config
            .get("provider_model_id")
            .and_then(|value| value.as_str()),
        Some("glm-5")
    );
    assert_eq!(
        provider
            .config
            .get("endpoint")
            .and_then(|value| value.as_str()),
        Some("https://open.bigmodel.cn/api/paas/v4/chat/completions")
    );
}

#[test]
fn z_ai_glm_5_3_flash_profile_expands_for_reviewer_agent_selection() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  glm53_flash:
    profile: "z_ai:glm-5.3-flash"
agents:
  fresh_reviewer:
    provider: "glm53_flash"
    model: "hosted:adl-z-ai:glm-5.3-flash"
tasks:
  review:
    prompt:
      user: "review candidate"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "fresh_reviewer"
        task: "review"
"#,
    );

    let expanded = expand_provider_profiles(&doc).expect("expand GLM-5.3-Flash profile");
    let provider = &expanded.providers["glm53_flash"];
    assert_eq!(provider.kind, "z_ai");
    assert_eq!(provider.profile.as_deref(), Some("z_ai:glm-5.3-flash"));
    assert_eq!(
        provider.default_model.as_deref(),
        Some("hosted:adl-z-ai:glm-5.3-flash")
    );
    assert_eq!(
        provider
            .config
            .get("provider_model_id")
            .and_then(|value| value.as_str()),
        Some("glm-5.3-flash")
    );
    assert_eq!(
        provider
            .config
            .get("endpoint")
            .and_then(|value| value.as_str()),
        Some("https://api.z.ai/api/paas/v4/chat/completions")
    );
    assert_eq!(
        provider
            .config
            .get("reasoning_effort")
            .and_then(|value| value.as_str()),
        Some("low")
    );
    assert_eq!(
        provider
            .config
            .get("clear_thinking")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        provider
            .config
            .get("temperature")
            .and_then(|value| value.as_f64()),
        Some(1.0)
    );
    assert_eq!(
        provider
            .config
            .get("top_p")
            .and_then(|value| value.as_f64()),
        Some(0.95)
    );
    assert_eq!(
        provider
            .config
            .get("max_output_tokens")
            .and_then(|value| value.as_u64()),
        Some(4096)
    );
    assert_eq!(
        provider
            .config
            .get("timeout_secs")
            .and_then(|value| value.as_u64()),
        Some(120)
    );
    let reviewer = &expanded.agents["fresh_reviewer"];
    assert_eq!(reviewer.provider, "glm53_flash");
    assert_eq!(reviewer.model, "hosted:adl-z-ai:glm-5.3-flash");
    build_provider(provider, None).expect("expanded GLM-5.3-Flash profile should build");
}

#[test]
fn z_ai_glm_5_3_flash_profile_preserves_runtime_overrides() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  glm53_flash:
    profile: "z_ai:glm-5.3-flash"
    config:
      max_output_tokens: 131072
      reasoning_effort: " high "
      clear_thinking: true
      temperature: 0.85
      top_p: 0.9
agents:
  reviewer:
    provider: "glm53_flash"
    model: "hosted:adl-z-ai:glm-5.3-flash"
tasks:
  review:
    prompt:
      user: "review candidate"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "reviewer"
        task: "review"
"#,
    );

    let expanded = expand_provider_profiles(&doc).expect("profile overrides should expand");
    let provider = &expanded.providers["glm53_flash"];
    assert_eq!(
        provider
            .config
            .get("max_output_tokens")
            .and_then(|value| value.as_u64()),
        Some(131_072)
    );
    assert_eq!(
        provider
            .config
            .get("reasoning_effort")
            .and_then(|value| value.as_str()),
        Some("high")
    );
    assert_eq!(
        provider
            .config
            .get("clear_thinking")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        provider
            .config
            .get("temperature")
            .and_then(|value| value.as_f64()),
        Some(0.85)
    );
    assert_eq!(
        provider
            .config
            .get("top_p")
            .and_then(|value| value.as_f64()),
        Some(0.9)
    );
}

#[test]
fn z_ai_glm_5_3_flash_profile_rejects_invalid_runtime_overrides() {
    for (yaml, expected) in [
        (
            r#"
version: "0.5"
providers:
  p:
    profile: "z_ai:glm-5.3-flash"
    config:
      reasoning_effort: "medium"
"#,
            "reasoning_effort must be one of low, high, max",
        ),
        (
            r#"
version: "0.5"
providers:
  p:
    profile: "z_ai:glm-5.3-flash"
    config:
      clear_thinking: "false"
"#,
            "clear_thinking must be a boolean",
        ),
        (
            r#"
version: "0.5"
providers:
  p:
    profile: "z_ai:glm-5.3-flash"
    config:
      max_output_tokens: 131073
"#,
            "max_output_tokens must be a positive integer no greater than 131072",
        ),
    ] {
        let doc = adl_doc_from_yaml(&format!(
            r#"{yaml}
agents:
  reviewer:
    provider: "p"
    model: "hosted:adl-z-ai:glm-5.3-flash"
tasks:
  review:
    prompt:
      user: "review"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "reviewer"
        task: "review"
"#
        ));
        let err = expand_provider_profiles(&doc).expect_err("invalid override should fail");
        assert!(
            err.to_string().contains(expected),
            "expected {expected:?}, got {err:#}"
        );
    }
}

#[test]
fn expand_provider_profiles_accepts_bedrock_nova_pro_inference_profile() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  bedrock_primary:
    profile: "bedrock:nova-pro-v1"
agents:
  a1:
    provider: "bedrock_primary"
    model: "hosted:adl-bedrock:us.amazon.nova-pro-v1:0"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("expand bedrock nova pro profile");
    let provider = &expanded.providers["bedrock_primary"];
    assert_eq!(provider.kind, "bedrock");
    assert_eq!(
        provider.default_model.as_deref(),
        Some("hosted:adl-bedrock:us.amazon.nova-pro-v1:0")
    );
    assert_eq!(
        provider
            .config
            .get("provider_model_id")
            .and_then(|value| value.as_str()),
        Some("us.amazon.nova-pro-v1:0")
    );
}

#[test]
fn expand_provider_profiles_rejects_http_profile_without_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
agents:
  a1:
    provider: "p1"
    model: "m"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let err = expand_provider_profiles(&doc).expect_err("placeholder endpoint profile must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("providers.p1.profile 'http:gpt-4o-mini'")
            && msg.contains("placeholder or invalid endpoint")
            && msg.contains("configure providers.p1.config.endpoint"),
        "unexpected error: {msg}"
    );
}

#[test]
fn expand_provider_profiles_accepts_http_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
    config:
      endpoint: "https://api.openai.com/v1/complete"
      headers:
        X-Client: "adl-test"
      timeout_secs: 12
agents:
  a1:
    provider: "p1"
    model: "gpt-4o-mini"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("profile expansion should succeed");
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(provider.default_model.as_deref(), Some("gpt-4o-mini"));
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.openai.com/v1/complete")
    );
    assert_eq!(
        provider.config.get("timeout_secs").and_then(|v| v.as_u64()),
        Some(12)
    );
}

#[test]
fn expand_provider_profiles_accepts_bedrock_nova_lite_profile() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  bedrock_primary:
    profile: "bedrock:nova-lite-v1"
agents:
  a1:
    provider: "bedrock_primary"
    model: "hosted:adl-bedrock:amazon.nova-lite-v1:0"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("expand bedrock profile");
    let provider = &expanded.providers["bedrock_primary"];
    assert_eq!(provider.kind, "bedrock");
    assert_eq!(
        provider.default_model.as_deref(),
        Some("hosted:adl-bedrock:amazon.nova-lite-v1:0")
    );
    assert_eq!(
        provider
            .config
            .get("provider_model_id")
            .and_then(|v| v.as_str()),
        Some("amazon.nova-lite-v1:0")
    );
}

#[test]
fn expand_provider_profiles_accepts_chatgpt_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "chatgpt:gpt-5.4"
    config:
      endpoint: "https://api.openai.com/v1/complete"
      auth:
        type: "bearer"
        env: "OPENAI_API_KEY"
      timeout_secs: 20
agents:
  a1:
    provider: "p1"
    model: "gpt-5.4"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("profile expansion should succeed");
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(provider.profile.as_deref(), Some("chatgpt:gpt-5.4"));
    assert_eq!(provider.default_model.as_deref(), Some("gpt-5.4"));
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.openai.com/v1/complete")
    );
    assert_eq!(
        provider
            .config
            .get("auth")
            .and_then(|v| v.get("env"))
            .and_then(|v| v.as_str()),
        Some("OPENAI_API_KEY")
    );
}

#[test]
fn expand_provider_profiles_accepts_claude_profile_with_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "claude:claude-3-7-sonnet"
    config:
      endpoint: "https://api.anthropic.com/v1/complete"
      auth:
        type: "bearer"
        env: "ANTHROPIC_API_KEY"
      timeout_secs: 20
agents:
  a1:
    provider: "p1"
    model: "claude-3-7-sonnet"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("profile expansion should succeed");
    let provider = &expanded.providers["p1"];
    assert_eq!(provider.kind, "http");
    assert_eq!(
        provider.profile.as_deref(),
        Some("claude:claude-3-7-sonnet")
    );
    assert_eq!(
        provider.default_model.as_deref(),
        Some("claude-3-7-sonnet-latest")
    );
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.anthropic.com/v1/complete")
    );
    assert_eq!(
        provider
            .config
            .get("auth")
            .and_then(|v| v.get("env"))
            .and_then(|v| v.as_str()),
        Some("ANTHROPIC_API_KEY")
    );
}

#[test]
fn expand_provider_profiles_builds_claude_opus_5_anthropic_provider() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  opus:
    profile: "claude:claude-opus-5"
agents:
  reviewer:
    provider: "opus"
    model: "claude-opus-5"
tasks:
  review:
    prompt:
      user: "review"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "reviewer"
        task: "review"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("profile expansion should succeed");
    let provider = &expanded.providers["opus"];
    assert_eq!(provider.kind, "anthropic");
    assert_eq!(provider.default_model.as_deref(), Some("claude-opus-5"));
    assert_eq!(
        provider
            .config
            .get("provider_model_id")
            .and_then(|v| v.as_str()),
        Some("claude-opus-5")
    );
    assert_eq!(
        provider.config.get("endpoint").and_then(|v| v.as_str()),
        Some("https://api.anthropic.com/v1/messages")
    );
    build_provider(provider, None).expect("expanded profile should build Anthropic provider");
}

#[test]
fn provider_profile_names_include_chatgpt_family() {
    let names = provider_profile_names();
    for required in [
        "chatgpt:gpt-5.4",
        "chatgpt:gpt-5.4-mini",
        "chatgpt:gpt-5.3-codex",
        "chatgpt:gpt-5.2",
    ] {
        assert!(
            names.iter().any(|name| name == required),
            "missing provider profile {required}"
        );
    }
}

#[test]
fn provider_profile_names_include_claude_family() {
    let names = provider_profile_names();
    for required in ["claude:claude-3-7-sonnet", "claude:claude-3-5-haiku"] {
        assert!(
            names.iter().any(|name| name == required),
            "missing provider profile {required}"
        );
    }
}

#[test]
fn resolve_run_accepts_http_profile_with_valid_endpoint_override() {
    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  p1:
    profile: "http:gpt-4o-mini"
    config:
      endpoint: "https://api.openai.com/v1/complete"
agents:
  a1:
    provider: "p1"
    model: "reasoning/default"
tasks:
  t1:
    prompt:
      user: "u"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "a1"
        task: "t1"
"#,
    );
    let resolved = ::adl::resolve::resolve_run(&doc).expect("valid endpoint should pass resolve");
    assert_eq!(
        resolved.steps.len(),
        1,
        "expected exactly one resolved step"
    );
    assert_eq!(resolved.doc.providers["p1"].kind, "http");
}
