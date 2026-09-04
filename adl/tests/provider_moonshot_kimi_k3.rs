use std::io::Write;
use std::time::Duration;

use ::adl::adl;
use ::adl::model_identity::{observed_at_now_v1, ModelIdentityStrengthV1, ModelIdentityV1};
use ::adl::provider::{build_provider, expand_provider_profiles, provider_profile_names};
use ::adl::provider_adapter::execute_provider_invocation;
use ::adl::provider_communication::{
    ProviderAttemptPolicyV1, ProviderInvocationFinalStatusV1, ProviderInvocationRequestV1,
    ProviderKindV1, ProviderRouteV1, ProviderRunLoggerV1, RuntimeSurfaceV1,
};
use ::adl::provider_substrate::{provider_invocation_target_v1, provider_substrate_v1};
use serde_json::Value;
use tempfile::TempDir;

fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("provider spec should parse")
}

fn adl_doc_from_yaml(yaml: &str) -> adl::AdlDoc {
    serde_yaml::from_str::<adl::AdlDoc>(yaml).expect("ADL doc should parse")
}

fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set read timeout");
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1024];
    let header_end = loop {
        let n = std::io::Read::read(stream, &mut buf).expect("read request chunk");
        assert!(n > 0, "client closed before sending complete headers");
        bytes.extend_from_slice(&buf[..n]);
        if let Some(pos) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            break pos + 4;
        }
    };

    let headers = String::from_utf8_lossy(&bytes[..header_end]).to_string();
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().expect("valid content length"))
        })
        .unwrap_or(0);
    while bytes.len() < header_end + content_length {
        let n = std::io::Read::read(stream, &mut buf).expect("read request body");
        assert!(n > 0, "client closed before sending complete body");
        bytes.extend_from_slice(&buf[..n]);
    }
    String::from_utf8_lossy(&bytes).to_string()
}

fn one_request_server(response_body: &'static str) -> (String, std::sync::mpsc::Receiver<String>) {
    let server = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback server");
    let addr = server.local_addr().expect("local addr");
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().expect("accept provider request");
        let request = read_http_request(&mut stream);
        tx.send(request).expect("send captured request");
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        stream.write_all(resp.as_bytes()).expect("write response");
    });
    (format!("http://{addr}/v1/chat/completions"), rx)
}

fn request_json_body(request: &str) -> Value {
    let body = request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .expect("request should include body");
    serde_json::from_str(body).expect("request body should be JSON")
}

fn kimi_model_identity() -> ModelIdentityV1 {
    ModelIdentityV1 {
        provider_kind: "kimi".to_string(),
        provider: "kimi".to_string(),
        provider_model_id: "kimi-k3".to_string(),
        runtime_surface: "hosted_http".to_string(),
        model_ref: "hosted:adl-kimi:kimi-k3".to_string(),
        identity_strength: ModelIdentityStrengthV1::TagOnly,
        observed_at: observed_at_now_v1(),
        resolved_digest: None,
        source_registry: Some("provider-profile-registry".to_string()),
        runtime_fingerprint: None,
        inference_parameter_fingerprint: None,
        tool_surface: None,
        governance_surface: None,
        evaluator_ref: None,
        lane_ref: Some("issue-680-focused".to_string()),
        benchmark_ref: None,
    }
}

fn kimi_invocation_request(
    endpoint: String,
    credential_ref: String,
) -> ProviderInvocationRequestV1 {
    ProviderInvocationRequestV1 {
        route: ProviderRouteV1 {
            provider_kind: ProviderKindV1::Hosted,
            provider: "kimi".to_string(),
            runtime_surface: RuntimeSurfaceV1::HostedApi,
            provider_model_id: "kimi-k3".to_string(),
            endpoint_ref: Some(endpoint),
            credential_ref: Some(credential_ref),
            source_registry: Some("provider-profile-registry".to_string()),
        },
        model_identity: kimi_model_identity(),
        prompt_contract_ref: "issue-680-kimi-k3-contract".to_string(),
        lane_ref: "issue-680-focused".to_string(),
        run_id: Some("issue-680-kimi-k3".to_string()),
        request_id: Some("issue-680-kimi-k3-001".to_string()),
        attempt_policy: ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 2_000,
            retry_backoff_ms: None,
        },
        input_text: Some("hello kimi k3".to_string()),
        max_output_tokens: Some(1_024),
        context_window_tokens: None,
        reasoning_effort: Some("high".to_string()),
        clear_thinking: None,
        temperature: None,
        top_p: None,
        local_keep_alive: None,
        inference_parameter_fingerprint: None,
        tool_surface: None,
        governance_surface: None,
        evaluator_ref: None,
        benchmark_ref: None,
    }
}

#[test]
fn issue_680_kimi_k3_profile_expands_to_moonshot_native_identity() {
    let names = provider_profile_names();
    assert!(names.contains(&"kimi:k3".to_string()));

    let doc = adl_doc_from_yaml(
        r#"
version: "0.5"
providers:
  kimi_primary:
    profile: "kimi:k3"
    config:
      auth:
        type: bearer
        env: MOONSHOT_API_KEY
agents:
  kimi_agent:
    provider: "kimi_primary"
    model: "hosted:adl-kimi:kimi-k3"
tasks:
  t1:
    prompt:
      user: "hello"
run:
  workflow:
    kind: sequential
    steps:
      - agent: "kimi_agent"
        task: "t1"
"#,
    );
    let expanded = expand_provider_profiles(&doc).expect("profile should expand");
    let spec = expanded
        .providers
        .get("kimi_primary")
        .expect("expanded provider");
    let target =
        provider_invocation_target_v1("kimi_primary", spec, None).expect("target should build");
    assert_eq!(target.vendor, "kimi");
    assert_eq!(target.provider_kind, "kimi");
    assert_eq!(target.model_ref, "kimi-k3");
    assert_eq!(target.provider_model_id, "kimi-k3");
    assert_eq!(
        target.endpoint.as_deref(),
        Some("https://api.moonshot.ai/v1/chat/completions")
    );
    assert_eq!(target.model_identity.provider_kind, "kimi");
    assert_eq!(target.model_identity.provider_model_id, "kimi-k3");
}

#[test]
fn issue_680_native_kimi_provider_accepts_moonshot_alias_and_preserves_capabilities() {
    for kind in ["kimi", "moonshot"] {
        let spec = provider_spec_from_yaml(&format!(
            r#"
type: "{kind}"
default_model: "hosted:adl-kimi:kimi-k3"
config:
  provider_model_id: "kimi-k3"
  auth:
    type: bearer
    env: MOONSHOT_API_KEY
"#
        ));
        let substrate = provider_substrate_v1("kimi_primary", &spec).expect("substrate");
        assert_eq!(substrate.vendor, "kimi");
        assert_eq!(substrate.provider_kind, "kimi");
        assert!(!substrate.capabilities.tool_calling.supported);
        assert!(substrate.capabilities.structured_json.supported);
        build_provider(&spec, None).expect("native Kimi provider should build");
    }
}

#[test]
fn issue_680_native_kimi_provider_sends_k3_reasoning_effort_and_redacts_secret() {
    let _no_proxy = std::env::var("NO_PROXY").ok();
    std::env::set_var("NO_PROXY", "127.0.0.1,localhost");
    std::env::set_var("ADL_ISSUE_680_MOONSHOT_KEY", "moonshot-test-key");
    let (endpoint, rx) = one_request_server(
        r#"{"choices":[{"message":{"content":"KIMI_K3_OK","reasoning_content":"hidden chain"}}]}"#,
    );
    let spec = provider_spec_from_yaml(&format!(
        r#"
type: "kimi"
config:
  endpoint: "{endpoint}"
  provider_model_id: "kimi-k3"
  reasoning_effort: "high"
  max_output_tokens: 1024
  auth:
    type: bearer
    env: ADL_ISSUE_680_MOONSHOT_KEY
"#
    ));

    let provider = build_provider(&spec, None).expect("native Kimi provider");
    assert_eq!(
        provider.complete("hello kimi k3").expect("Kimi response"),
        "KIMI_K3_OK"
    );
    let received = rx.recv_timeout(Duration::from_secs(2)).expect("request");
    assert!(received.contains("authorization: Bearer moonshot-test-key"));
    let body = request_json_body(&received);
    assert_eq!(body["model"], "kimi-k3");
    assert_eq!(body["reasoning_effort"], "high");
    assert_eq!(body["max_tokens"], 1024);
    assert_eq!(body["messages"][0]["content"], "hello kimi k3");

    std::env::remove_var("ADL_ISSUE_680_MOONSHOT_KEY");
}

#[test]
fn issue_680_runtime_adapter_routes_moonshot_kimi_k3_with_reasoning_effort() {
    std::env::set_var("NO_PROXY", "127.0.0.1,localhost");
    std::env::set_var("ADL_ISSUE_680_RUNTIME_MOONSHOT_KEY", "runtime-moonshot-key");
    let (endpoint, rx) = one_request_server(
        r#"{"model":"kimi-k3","choices":[{"message":{"content":"RUNTIME_KIMI_OK"}}]}"#,
    );
    let temp = TempDir::new().expect("temp dir");
    let log_path = temp.path().join("provider-run.jsonl");
    let mut logger = ProviderRunLoggerV1::create(&log_path, "issue-680-run").expect("logger");
    let req = kimi_invocation_request(
        endpoint,
        "env:ADL_ISSUE_680_RUNTIME_MOONSHOT_KEY".to_string(),
    );

    let result = execute_provider_invocation(req, &mut logger);
    drop(logger);

    assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Ok);
    assert_eq!(result.output_text.as_deref(), Some("RUNTIME_KIMI_OK"));
    let received = rx.recv_timeout(Duration::from_secs(2)).expect("request");
    assert!(received.contains("authorization: Bearer runtime-moonshot-key"));
    let body = request_json_body(&received);
    assert_eq!(body["model"], "kimi-k3");
    assert_eq!(body["reasoning_effort"], "high");
    assert_eq!(body["max_tokens"], 1024);
    let log = std::fs::read_to_string(log_path).expect("log");
    assert!(!log.contains("runtime-moonshot-key"));
    assert!(!log.contains("RUNTIME_KIMI_OK"));

    std::env::remove_var("ADL_ISSUE_680_RUNTIME_MOONSHOT_KEY");
}

#[test]
fn issue_680_kimi_k3_reasoning_effort_rejects_unknown_value() {
    let spec = provider_spec_from_yaml(
        r#"
type: "kimi"
config:
  provider_model_id: "kimi-k3"
  reasoning_effort: "medium"
  auth:
    type: bearer
    env: MOONSHOT_API_KEY
"#,
    );
    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("invalid reasoning effort should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("reasoning_effort must be one of low, high, max"),
        "{err:#}"
    );

    let mut req = kimi_invocation_request(
        "http://127.0.0.1:9/v1/chat/completions".to_string(),
        "env:MOONSHOT_API_KEY".to_string(),
    );
    req.reasoning_effort = Some("medium".to_string());
    let mut logger =
        ProviderRunLoggerV1::create(TempDir::new().unwrap().path().join("run.jsonl"), "bad")
            .expect("logger");
    let result = execute_provider_invocation(req, &mut logger);
    assert_eq!(result.final_status, ProviderInvocationFinalStatusV1::Failed);
    assert!(result
        .attempts
        .first()
        .and_then(|attempt| attempt.failure.as_ref())
        .map(|failure| failure.message.contains("reasoning_effort"))
        .unwrap_or(false));
}

#[test]
fn issue_680_provider_setup_cli_documents_kimi_family() {
    let temp = TempDir::new().expect("temp dir");
    let bin = env!("CARGO_BIN_EXE_adl");
    let output = std::process::Command::new(bin)
        .args([
            "provider",
            "setup",
            "moonshot",
            "--out",
            temp.path().join("kimi-setup").to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run provider setup");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let provider_yaml =
        std::fs::read_to_string(temp.path().join("kimi-setup/provider.adl.yaml")).expect("yaml");
    let env_example =
        std::fs::read_to_string(temp.path().join("kimi-setup/env.example")).expect("env");
    let readme = std::fs::read_to_string(temp.path().join("kimi-setup/README.md")).expect("readme");
    assert!(provider_yaml.contains("type: \"kimi\""));
    assert!(provider_yaml.contains("provider_model_id: \"kimi-k3\""));
    assert!(provider_yaml.contains("env: MOONSHOT_API_KEY"));
    assert!(env_example.contains("MOONSHOT_API_KEY=replace-me"));
    assert!(readme.contains("Kimi K3 supports reasoning_effort"));
}
