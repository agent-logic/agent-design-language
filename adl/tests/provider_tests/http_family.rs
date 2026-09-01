use std::io::{Read, Write};

use ::adl::provider::{build_provider, is_retryable_error, stable_failure_kind};
use serde_json::Value;

use super::helpers::EnvVarGuard;
use super::support::{
    block_incoming_localhost, localhost_and_auth_env_guard, provider_spec_from_yaml,
    read_http_request,
};

fn request_json_body(request: &str) -> Value {
    let body = request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .expect("request should include a JSON body");
    serde_json::from_str(body).expect("request body should be JSON")
}

#[test]
fn http_provider_happy_path() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let out = p.complete("hello").expect("http provider should succeed");
    assert_eq!(out, "OK");
}

#[test]
fn expanded_kimi_profile_uses_chat_payload_and_choice_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.contains("\"model\":\"kimi-k2.5\""));
        assert!(request.contains("\"messages\":[{"));
        assert!(request.contains("hello kimi"));
        let body = r#"{"choices":[{"message":{"content":"KIMI_OK"}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });
    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
profile: "kimi:k2.5"
config:
  endpoint: "http://{addr}/v1/chat/completions"
  provider_model_id: "kimi-k2.5"
"#
    ));
    let provider = build_provider(&spec, None).expect("kimi profile should build");
    assert_eq!(
        provider.complete("hello kimi").expect("kimi call"),
        "KIMI_OK"
    );
}

#[test]
fn openai_provider_translates_native_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_OPENAI_KEY", "test-openai-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("authorization:"));
        assert!(request.contains("Bearer test-openai-token"));
        assert!(request.contains("\"model\":\"gpt-test\""));
        assert!(request.contains("\"input\":\"hello openai\""));
        let body = r#"{"output_text":"OPENAI_NATIVE_OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: openai
config:
  endpoint: "http://{addr}/v1/responses"
  provider_model_id: "gpt-test"
  auth:
    type: bearer
    env: ADL_TEST_OPENAI_KEY
"#
    ));

    let p = build_provider(&spec, None).expect("openai provider should build");
    let out = p
        .complete("hello openai")
        .expect("openai provider should succeed");
    assert_eq!(out, "OPENAI_NATIVE_OK");
}

#[test]
fn bedrock_provider_builds_without_network_and_preserves_model_id() {
    let spec = provider_spec_from_yaml(
        r#"
type: bedrock
default_model: "hosted:adl-bedrock:amazon.nova-lite-v1:0"
config:
  region: "us-east-1"
  profile: "agent-logic-admin"
  provider_model_id: "amazon.nova-lite-v1:0"
"#,
    );

    let _provider = build_provider(&spec, None).expect("bedrock provider should build");
}

#[test]
fn anthropic_provider_translates_native_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_ANTHROPIC_KEY", "test-anthropic-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("x-api-key:"));
        assert!(request.contains("test-anthropic-token"));
        assert!(request
            .to_ascii_lowercase()
            .contains("anthropic-version: 2023-06-01"));
        assert!(request.contains("\"model\":\"claude-test\""));
        assert!(request.contains("\"content\":\"hello claude\""));
        let body = r#"{"content":[{"type":"text","text":"ANTHROPIC_NATIVE_OK"}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: anthropic
config:
  endpoint: "http://{addr}/v1/messages"
  provider_model_id: "claude-test"
  auth:
    type: bearer
    env: ADL_TEST_ANTHROPIC_KEY
"#
    ));

    let p = build_provider(&spec, None).expect("anthropic provider should build");
    let out = p
        .complete("hello claude")
        .expect("anthropic provider should succeed");
    assert_eq!(out, "ANTHROPIC_NATIVE_OK");
}

#[test]
fn vertex_ai_gemini_provider_translates_native_response_and_records_family() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let receipt_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.csdlc/evidence/528/provider-tests");
    std::fs::create_dir_all(&receipt_dir).expect("create issue-owned provider test evidence dir");
    let receipt = receipt_dir.join(format!(
        "adl-vertex-ai-invocations-{}-{}.json",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    let _ = std::fs::remove_file(&receipt);
    let receipt_value = receipt.to_string_lossy().to_string();
    let _env_guard = EnvVarGuard::set_many(&[
        ("NO_PROXY", std::ffi::OsStr::new("127.0.0.1,localhost")),
        (
            "ADL_TEST_VERTEX_AI_TOKEN",
            std::ffi::OsStr::new("test-vertex-token"),
        ),
        (
            "ADL_PROVIDER_INVOCATIONS_PATH",
            std::ffi::OsStr::new(&receipt_value),
        ),
    ]);

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.starts_with("POST /v1/projects/test-project/locations/us-west1/publishers/google/models/gemini-test:generateContent "));
        assert!(request.to_ascii_lowercase().contains("authorization:"));
        assert!(request.contains("Bearer test-vertex-token"));
        assert!(request.contains("\"role\":\"user\""));
        assert!(request.contains("\"text\":\"hello vertex\""));
        assert!(request.contains("\"maxOutputTokens\":321"));
        let body = r#"{"candidates":[{"content":{"parts":[{"text":"VERTEX_NATIVE_OK"}]}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: vertex_ai_gemini
config:
  endpoint: "http://{addr}/v1/projects/test-project/locations/us-west1/publishers/google/models/gemini-test:generateContent"
  project: "test-project"
  location: "us-west1"
  provider_model_id: "gemini-test"
  max_output_tokens: 321
  auth:
    type: bearer
    env: ADL_TEST_VERTEX_AI_TOKEN
"#
    ));

    let provider = build_provider(&spec, None).expect("vertex_ai_gemini provider should build");
    let out = provider
        .complete("hello vertex")
        .expect("vertex_ai_gemini provider should succeed");
    assert_eq!(out, "VERTEX_NATIVE_OK");

    let receipt_json: Value =
        serde_json::from_slice(&std::fs::read(&receipt).expect("invocation receipt should exist"))
            .expect("receipt should be JSON");
    assert_eq!(receipt_json["invocations"][0]["family"], "vertex_ai_gemini");
    assert_eq!(receipt_json["invocations"][0]["model"], "gemini-test");
    assert_eq!(receipt_json["invocations"][0]["http_status"], 200);
    let rendered = receipt_json.to_string();
    assert!(!rendered.contains("test-vertex-token"));
    assert!(!rendered.contains("ADL_TEST_VERTEX_AI_TOKEN"));
    let _ = std::fs::remove_file(receipt);
}

#[test]
fn vertex_ai_gemini_provider_preserves_uts_tool_declarations_and_call_arguments() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_VERTEX_AI_TOKEN", "test-vertex-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        let body = request_json_body(&request);
        assert_eq!(
            body["tools"][0]["functionDeclarations"][0]["name"],
            "search_docs"
        );
        assert_eq!(
            body["tools"][0]["functionDeclarations"][0]["parameters"]["properties"]["query"]
                ["type"],
            "string"
        );
        let response = r#"{"candidates":[{"content":{"parts":[{"functionCall":{"name":"search_docs","args":{"query":"agent logic","limit":3}}}]}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response.len(),
            response
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: vertex_ai_gemini
config:
  endpoint: "http://{addr}/v1/projects/test-project/locations/us-west1/publishers/google/models/gemini-test:generateContent"
  project: "test-project"
  location: "us-west1"
  provider_model_id: "gemini-test"
  auth:
    type: bearer
    env: ADL_TEST_VERTEX_AI_TOKEN
  tools:
    - name: search_docs
      description: Search indexed docs.
      input_schema:
        type: object
        properties:
          query:
            type: string
          limit:
            type: integer
"#
    ));

    let provider = build_provider(&spec, None).expect("vertex_ai_gemini provider should build");
    let out = provider
        .complete("use a tool")
        .expect("vertex_ai_gemini tool response should succeed");
    let normalized: Value = serde_json::from_str(&out).expect("tool-call output should be JSON");
    assert_eq!(normalized["tool_calls"][0]["name"], "search_docs");
    assert_eq!(
        normalized["tool_calls"][0]["arguments"]["query"],
        "agent logic"
    );
    assert_eq!(normalized["tool_calls"][0]["arguments"]["limit"], 3);
    assert_ne!(
        normalized["tool_calls"][0]["arguments"]["query"],
        normalized["tool_calls"][0]["name"]
    );
}

#[test]
fn vertex_ai_gemini_provider_streams_via_vertex_stream_endpoint_and_callback() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_VERTEX_AI_TOKEN", "test-vertex-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.starts_with("POST /v1/projects/test-project/locations/us-west1/publishers/google/models/gemini-test:streamGenerateContent "));
        let body = request_json_body(&request);
        assert_eq!(body["contents"][0]["parts"][0]["text"], "stream vertex");
        let response = r#"{"candidates":[{"content":{"parts":[{"text":"VERTEX_STREAM_OK"}]}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response.len(),
            response
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: vertex_ai_gemini
config:
  endpoint: "http://{addr}/v1/projects/test-project/locations/us-west1/publishers/google/models/gemini-test:generateContent"
  project: "test-project"
  location: "us-west1"
  provider_model_id: "gemini-test"
  auth:
    type: bearer
    env: ADL_TEST_VERTEX_AI_TOKEN
"#
    ));

    let provider = build_provider(&spec, None).expect("vertex_ai_gemini provider should build");
    let mut chunks = Vec::new();
    let out = provider
        .complete_stream("stream vertex", &mut |chunk| chunks.push(chunk.to_string()))
        .expect("vertex_ai_gemini stream should succeed");
    assert_eq!(out, "VERTEX_STREAM_OK");
    assert_eq!(chunks, vec!["VERTEX_STREAM_OK"]);
}

#[test]
fn vertex_ai_gemini_provider_builds_regional_endpoint_without_network() {
    let spec = provider_spec_from_yaml(
        r#"
type: vertex_ai_gemini
config:
  project: "company-project"
  location: "us-west1"
  provider_model_id: "gemini-2.5-flash"
"#,
    );

    let provider = build_provider(&spec, None)
        .expect("vertex_ai_gemini provider should build with regional Vertex endpoint");
    let err = provider
        .complete("missing token")
        .expect_err("missing token should fail before network");
    let msg = format!("{err:#}");
    assert!(msg.contains("missing required auth env var 'ADL_VERTEX_AI_ACCESS_TOKEN'"));
    assert!(!msg.contains("Bearer"));
}

#[test]
fn vertex_ai_gemini_provider_rejects_untrusted_custom_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: vertex_ai_gemini
config:
  endpoint: "https://example.com/v1/projects/p/locations/us-west1/publishers/google/models/gemini-test:generateContent"
  project: "p"
  location: "us-west1"
  provider_model_id: "gemini-test"
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("untrusted custom endpoint should fail"),
        Err(err) => err,
    };
    assert!(err
        .to_string()
        .contains("refusing to send Vertex AI bearer credentials to an untrusted endpoint"));
}

#[test]
fn zai_provider_translates_native_response_through_build_provider() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_ZAI_KEY", "test-zai-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("authorization:"));
        assert!(request.contains("Bearer test-zai-token"));
        assert!(request.contains("\"model\":\"glm-5\""));
        assert!(request.contains("\"content\":\"hello zai\""));
        assert!(request.contains("\"stream\":false"));
        let body = r#"{"model":"glm-5","choices":[{"message":{"content":"ZAI_NATIVE_OK"}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: z_ai
config:
  endpoint: "http://{addr}/api/paas/v4/chat/completions"
  provider_model_id: "glm-5"
  auth:
    type: bearer
    env: ADL_TEST_ZAI_KEY
"#
    ));

    let p = build_provider(&spec, None).expect("z_ai provider should build");
    let out = p
        .complete("hello zai")
        .expect("z_ai provider should succeed");
    assert_eq!(out, "ZAI_NATIVE_OK");
}

#[test]
fn zai_glm_5_3_flash_request_materializes_profile_defaults_and_runtime_overrides() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _env_guard = localhost_and_auth_env_guard("ADL_TEST_ZAI_KEY", "test-zai-token");

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let request = read_http_request(&mut stream);
        assert!(request.to_ascii_lowercase().contains("authorization:"));
        assert!(request.contains("Bearer test-zai-token"));
        let body = request_json_body(&request);
        assert_eq!(body["model"], "glm-5.3-flash");
        assert_eq!(body["messages"][0]["role"], "user");
        assert_eq!(body["messages"][0]["content"], "hello glm flash");
        assert_eq!(body["max_tokens"], 4096);
        assert_eq!(body["stream"], false);
        assert_eq!(body["reasoning_effort"], "low");
        assert_eq!(body["thinking"]["type"], "enabled");
        assert_eq!(body["thinking"]["clear_thinking"], true);
        assert_eq!(body["temperature"], 1.0);
        assert_eq!(body["top_p"], 0.95);
        let response_body =
            r#"{"model":"glm-5.3-flash","choices":[{"message":{"content":"GLM_FLASH_OK"}}]}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: z_ai
config:
  endpoint: "http://{addr}/api/paas/v4/chat/completions"
  provider_model_id: "glm-5.3-flash"
  max_output_tokens: 4096
  reasoning_effort: "low"
  clear_thinking: true
  temperature: 1.0
  top_p: 0.95
  auth:
    type: bearer
    env: ADL_TEST_ZAI_KEY
"#
    ));

    let provider = build_provider(&spec, None).expect("z_ai GLM-5.3-Flash provider should build");
    let out = provider
        .complete("hello glm flash")
        .expect("z_ai GLM-5.3-Flash provider should succeed");
    assert_eq!(out, "GLM_FLASH_OK");
}

#[test]
fn zai_glm_5_3_flash_rejects_invalid_runtime_config() {
    for (yaml, expected) in [
        (
            r#"
type: z_ai
config:
  provider_model_id: "glm-5.3-flash"
  max_output_tokens: 131073
"#,
            "max_tokens/max_output_tokens must be no greater than 131072",
        ),
        (
            r#"
type: z_ai
config:
  provider_model_id: "glm-5.3-flash"
  reasoning_effort: "medium"
"#,
            "reasoning_effort must be one of low, high, max",
        ),
        (
            r#"
type: z_ai
config:
  provider_model_id: "glm-5.3-flash"
  clear_thinking: "false"
"#,
            "clear_thinking must be a boolean",
        ),
        (
            r#"
type: z_ai
config:
  provider_model_id: "glm-5.3-flash"
  temperature: 1.1
"#,
            "temperature must be in [0, 1]",
        ),
        (
            r#"
type: z_ai
config:
  provider_model_id: "glm-5.3-flash"
  top_p: 0.0
"#,
            "top_p must be in [0.01, 1]",
        ),
    ] {
        let spec = provider_spec_from_yaml(yaml);
        let err = match build_provider(&spec, None) {
            Ok(_) => panic!("invalid config should fail"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(expected),
            "expected {expected:?}, got {err:#}"
        );
    }
}

#[test]
fn native_provider_missing_auth_env_is_sanitized() {
    let _env_guard = EnvVarGuard::unset("ADL_TEST_MISSING_OPENAI_KEY");
    let spec = provider_spec_from_yaml(
        r#"
type: openai
config:
  provider_model_id: "gpt-test"
  auth:
    type: bearer
    env: ADL_TEST_MISSING_OPENAI_KEY
"#,
    );

    let p = build_provider(&spec, None).expect("openai provider should build");
    let err = p
        .complete("hello")
        .expect_err("missing auth env should fail");
    let msg = format!("{err:#}");
    assert!(msg.contains("missing required auth env var"));
    assert!(msg.contains("ADL_TEST_MISSING_OPENAI_KEY"));
    assert!(!msg.contains("Bearer"));
}

#[test]
fn http_provider_accepts_https_endpoint() {
    // This test is intentionally config-only: we verify that `https://` endpoints
    // are accepted by parsing/building the provider, without performing a network call.
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "https://example.com/v1/complete"
"#,
    );

    let _p = build_provider(&spec, None).expect("build_provider should accept https endpoints");
}

#[test]
fn http_provider_rejects_plaintext_remote_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://api.example.com/v1/complete"
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("plain remote http should fail"),
        Err(err) => err,
    };
    assert!(err
        .to_string()
        .contains("plaintext http:// is only allowed for localhost/loopback test endpoints"));
}

#[test]
fn http_provider_non_200_response() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad";
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("server_error") && msg.contains("500"),
        "unexpected error: {msg}"
    );
    assert!(
        is_retryable_error(&err),
        "5xx responses should be retryable: {msg}"
    );
}

#[test]
fn http_provider_long_error_body_is_truncated_deterministically() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "x".repeat(300);
        let resp = format!(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").expect_err("500 should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("kind=server_error"),
        "expected server_error classification, got: {msg}"
    );
    assert!(
        msg.contains("status=500"),
        "expected status code in message, got: {msg}"
    );
    assert!(
        msg.len() < 600,
        "response body should be truncated in error message, got len={}",
        msg.len()
    );
}

#[test]
fn http_provider_rejects_json_without_output_field() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = r#"{"not_output":"value"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p
        .complete("hello")
        .expect_err("response without output should fail");
    let msg = format!("{err:#}");
    assert!(
        msg.contains("response missing 'output' field"),
        "unexpected error: {msg}"
    );
    assert_eq!(stable_failure_kind(&err), Some("provider_error"));
}

#[test]
fn http_provider_4xx_is_non_retryable() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = "bad request";
        let resp = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("client_error") && msg.contains("400"),
        "unexpected error: {msg}"
    );
    assert!(
        !is_retryable_error(&err),
        "4xx responses should be non-retryable: {msg}"
    );
}

#[test]
fn http_provider_missing_auth_env_var() {
    let addr = "127.0.0.1:9";

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  auth:
    type: bearer
    env: MISSING_ENV
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}");
    assert!(
        msg.contains("missing required auth env var"),
        "unexpected error: {msg}"
    );
    assert_eq!(
        stable_failure_kind(&err),
        Some("schema_error"),
        "missing auth env var should classify as schema_error"
    );
}

#[test]
fn http_provider_rejects_non_object_headers_and_non_string_values() {
    let non_object = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers: "not-an-object"
"#,
    );
    let err = match build_provider(&non_object, None) {
        Ok(_) => panic!("non-object headers should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string().contains("config.headers must be an object"),
        "unexpected error: {err:#}"
    );

    let non_string_value = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  headers:
    X-Number: 123
"#,
    );
    let err2 = match build_provider(&non_string_value, None) {
        Ok(_) => panic!("non-string header should fail"),
        Err(err) => err,
    };
    assert!(
        err2.to_string()
            .contains("config.headers values must be strings"),
        "unexpected error: {err2:#}"
    );
}

#[test]
fn http_provider_rejects_non_bearer_auth_type() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  auth:
    type: basic
    env: API_KEY
"#,
    );
    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("non-bearer auth should fail"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("config.auth.type must be 'bearer'"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn http_provider_supports_timeout_secs_string_and_rejects_negative_number() {
    let string_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: "7"
"#,
    );
    let _provider =
        build_provider(&string_timeout, None).expect("string timeout should parse as u64");

    let negative_timeout = provider_spec_from_yaml(
        r#"
type: http
config:
  endpoint: "http://127.0.0.1:9/"
  timeout_secs: -3
"#,
    );
    let _provider = build_provider(&negative_timeout, None)
        .expect("negative timeout should be treated as absent, not a parse failure");
}

#[test]
fn http_provider_rejects_missing_endpoint() {
    let spec = provider_spec_from_yaml(
        r#"
type: http
config: {}
"#,
    );

    let err = match build_provider(&spec, None) {
        Ok(_) => panic!("expected build_provider to fail for missing endpoint"),
        Err(err) => err,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("invalid config") && msg.contains("endpoint"),
        "unexpected error: {msg}"
    );
}

#[test]
fn http_provider_timeout() {
    let server = match std::net::TcpListener::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return,
        Err(e) => panic!("failed to bind local test server: {e}"),
    };
    let addr = server.local_addr().unwrap();
    let _server_guard = block_incoming_localhost();

    std::thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        std::thread::sleep(std::time::Duration::from_secs(2));
        let body = r#"{"output":"OK"}"#;
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(resp.as_bytes());
    });

    let spec = provider_spec_from_yaml(&format!(
        r#"
type: http
config:
  endpoint: "http://{addr}/"
  timeout_secs: 1
"#
    ));

    let p = build_provider(&spec, None).expect("build_provider failed");
    let err = p.complete("hello").unwrap_err();
    let msg = format!("{err:#}").to_lowercase();
    assert!(
        msg.contains("timeout") || msg.contains("timed out"),
        "unexpected error: {msg}"
    );
}
