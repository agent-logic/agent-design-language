//! PVF: deterministic local adapter negatives, CPU/loopback only, bounded one
//! socket and one worker per case, required provider-platform gate. No Metal proof.
use super::*;
use std::net::TcpListener;

fn spec(endpoint: &str) -> adl::ProviderSpec {
    serde_json::from_value(
        json!({"type":"mlx", "default_model":"fixture-model", "config":{
            "endpoint":endpoint,"timeout_secs":1,"max_output_tokens":16
        }}),
    )
    .unwrap()
}
fn adapter(spec: &adl::ProviderSpec) -> Result<MlxProvider> {
    let target = provider_substrate::provider_invocation_target_v1("mlx", spec, None)?;
    MlxProvider::validated(spec, &target)
}
fn server(status: u16, body: String, delay: Duration) -> (String, std::thread::JoinHandle<Value>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!(
        "http://{}/v1/chat/completions",
        listener.local_addr().unwrap()
    );
    let worker = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut bytes = Vec::new();
        let (start, size) = loop {
            let mut buf = [0; 2048];
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            bytes.extend_from_slice(&buf[..n]);
            assert!(bytes.len() < 65536);
            if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                let header = std::str::from_utf8(&bytes[..end]).unwrap();
                assert!(header.starts_with("POST /v1/chat/completions HTTP/1.1"));
                assert!(!header.to_ascii_lowercase().contains("authorization:"));
                let length = header
                    .lines()
                    .find_map(|line| {
                        let (k, v) = line.split_once(':')?;
                        k.eq_ignore_ascii_case("content-length")
                            .then(|| v.trim().parse::<usize>().unwrap())
                    })
                    .unwrap();
                break (end + 4, length);
            }
        };
        while bytes.len() < start + size {
            let mut buf = [0; 2048];
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            bytes.extend_from_slice(&buf[..n]);
        }
        let request = serde_json::from_slice(&bytes[start..start + size]).unwrap();
        std::thread::sleep(delay);
        let _=write!(socket,"HTTP/1.1 {status} Fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len());
        request
    });
    (endpoint, worker)
}
fn response() -> Value {
    json!({"model":"fixture-model","choices":[{"message":{"content":"bounded answer"},"finish_reason":"stop"}]})
}

#[test]
fn sends_canonical_model_limits_and_prompt() {
    let (url, worker) = server(200, response().to_string(), Duration::ZERO);
    assert_eq!(
        adapter(&spec(&url))
            .unwrap()
            .complete("private prompt")
            .unwrap(),
        "bounded answer"
    );
    let request = worker.join().unwrap();
    assert_eq!(request["model"], "fixture-model");
    assert_eq!(request["max_tokens"], 16);
    assert_eq!(request["stream"], false);
    assert_eq!(request["messages"][0]["content"], "private prompt");
}
#[test]
fn rejects_remote_credentials_redirect_paths_and_query() {
    for url in [
        "https://example.com/v1/chat/completions",
        "http://localhost:8080/v1/chat/completions",
        "http://user:secret@127.0.0.1:1/v1/chat/completions",
        "http://127.0.0.1:1/other",
        "http://127.0.0.1:1/v1/chat/completions?key=secret",
    ] {
        let err = adapter(&spec(url)).err().unwrap().to_string();
        assert!(!err.contains("secret"));
    }
}
#[test]
fn requires_explicit_model_and_strict_limits() {
    let original = spec("http://127.0.0.1:1/v1/chat/completions");
    let mut missing = original.clone();
    missing.default_model = None;
    assert!(adapter(&missing).is_err());
    for key in ["timeout_secs", "max_output_tokens"] {
        for invalid in [
            Value::Null,
            json!("1"),
            json!(0),
            json!(-1),
            json!(1.5),
            json!(999999),
        ] {
            let mut s = original.clone();
            s.config.insert(key.into(), invalid);
            assert!(adapter(&s).is_err());
        }
    }
    let mut s = original;
    s.default_model = Some("default_model".into());
    assert!(adapter(&s).is_err());
}
#[test]
fn rejects_empty_and_oversized_prompt_before_network() {
    let p = adapter(&spec("http://127.0.0.1:1/v1/chat/completions")).unwrap();
    assert!(p.complete(" ").unwrap_err().to_string().contains("prompt"));
    assert!(p
        .complete(&"x".repeat(MAX_PROMPT_BYTES + 1))
        .unwrap_err()
        .to_string()
        .contains("prompt"));
}
#[test]
fn response_shape_model_and_nonempty_text_fail_closed() {
    for body in [
        "private garbage".into(),
        json!({}).to_string(),
        json!({"model":"other","choices":[{"message":{"content":"text"}}]}).to_string(),
        json!({"model":"fixture-model","choices":[{"message":{"content":" "}}]}).to_string(),
        json!({"model":"fixture-model","choices":[]}).to_string(),
    ] {
        let (url, worker) = server(200, body, Duration::ZERO);
        let err = adapter(&spec(&url))
            .unwrap()
            .complete("secret")
            .unwrap_err();
        assert!(!err.to_string().contains("private garbage"));
        assert!(!err.to_string().contains("secret"));
        worker.join().unwrap();
    }
}
#[test]
fn http_errors_are_redacted_and_no_redirect_followed() {
    for status in [302, 404, 500] {
        let (url, worker) = server(status, "secret server payload".into(), Duration::ZERO);
        let err = adapter(&spec(&url))
            .unwrap()
            .complete("secret prompt")
            .unwrap_err();
        assert!(err.to_string().contains(&status.to_string()));
        assert!(!err.to_string().contains("secret"));
        worker.join().unwrap();
    }
}
#[test]
fn absent_service_is_actionable() {
    let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!(
        "http://{}/v1/chat/completions",
        socket.local_addr().unwrap()
    );
    drop(socket);
    assert!(adapter(&spec(&url))
        .unwrap()
        .complete("hello")
        .unwrap_err()
        .to_string()
        .contains("service unavailable"));
}
#[test]
fn timeout_settles_client_call_without_claiming_server_cancellation() {
    let (url, worker) = server(200, response().to_string(), Duration::from_millis(1300));
    let start = Instant::now();
    let err = adapter(&spec(&url)).unwrap().complete("hello").unwrap_err();
    assert_eq!(stable_failure_kind(&err), Some("timeout"));
    assert!(start.elapsed() < Duration::from_secs(3));
    worker.join().unwrap();
}
#[test]
fn oversized_response_is_rejected() {
    let (url, worker) = server(
        200,
        "x".repeat(MAX_RESPONSE_BYTES as usize + 1),
        Duration::ZERO,
    );
    assert!(adapter(&spec(&url))
        .unwrap()
        .complete("hello")
        .unwrap_err()
        .to_string()
        .contains("byte limit"));
    worker.join().unwrap();
}

#[test]
fn mlx_capabilities_do_not_advertise_native_tools_or_json() {
    let s = spec("http://127.0.0.1:1/v1/chat/completions");
    let target = provider_substrate::provider_invocation_target_v1("mlx", &s, None).unwrap();
    assert!(!target.capabilities.tool_calling.supported);
    assert!(!target.capabilities.semantic_tool_fallback.supported);
    assert_eq!(
        target.capabilities.structured_json.mode,
        provider_substrate::CapabilityModeV1::PromptBased
    );
}

#[test]
fn canonical_seed_is_validated_and_transmitted() {
    let (url, worker) = server(200, response().to_string(), Duration::ZERO);
    let mut s = spec(&url);
    s.config.insert("deterministic_seed".into(), json!(42));
    adapter(&s).unwrap().complete("hello").unwrap();
    assert_eq!(worker.join().unwrap()["seed"], 42);
    for value in [
        json!(-1),
        json!(1.5),
        json!("42"),
        Value::Null,
        json!(4294967296u64),
    ] {
        s.config.insert("deterministic_seed".into(), value);
        assert!(adapter(&s).is_err());
    }
}

#[test]
fn unsupported_capability_override_is_rejected() {
    let mut s = spec("http://127.0.0.1:1/v1/chat/completions");
    s.config.insert(
        "capabilities".into(),
        json!({"tool_calling": {"supported": true, "mode": "native"}}),
    );
    assert!(adapter(&s).is_err());
}
