use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    net::{IpAddr, TcpListener},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use adl_runtime_kernel::{
    LocalShepherdConfig, LocalShepherdExecutor, OperationExecutor, OperationRequest,
    ShepherdExecutionClass, ShepherdModelIdentity, ShepherdProvenance, ShepherdResponse,
    OPERATION_REQUEST_SCHEMA, SHEPHERD_REQUEST_SCHEMA,
};
use serde_json::json;
use sha2::{Digest, Sha256};

fn validate_loopback_ollama_origin(value: &str) -> Result<(), &'static str> {
    if value.is_empty() || value.trim() != value {
        return Err("origin must be nonempty and contain no surrounding whitespace");
    }
    let authority = value
        .strip_prefix("http://")
        .ok_or("origin must use plain HTTP on the local loopback boundary")?;
    if authority.contains(['/', '?', '#', '@', '\\']) {
        return Err("origin must contain only a loopback authority and explicit port");
    }
    let (host, port) = if let Some(bracketed) = authority.strip_prefix('[') {
        let (host, port) = bracketed
            .split_once("]:")
            .ok_or("bracketed loopback origin must include an explicit port")?;
        (host, port)
    } else {
        authority
            .rsplit_once(':')
            .ok_or("loopback origin must include an explicit port")?
    };
    let port = port
        .parse::<u16>()
        .map_err(|_| "loopback origin port is invalid")?;
    if port == 0 {
        return Err("loopback origin port must be nonzero");
    }
    let loopback = host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<IpAddr>()
            .is_ok_and(|address| address.is_loopback());
    if !loopback {
        return Err("origin host must be localhost or a loopback IP address");
    }
    Ok(())
}

const OLLAMA_ATTESTED_RUNNER: &str = r#"#!/usr/bin/python3
import json, os, sys, urllib.request

request = json.loads(sys.stdin.readline())
host = os.environ["ADL_OLLAMA_HOST"].rstrip("/")

class RejectRedirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, file_pointer, code, message, headers, new_url):
        raise RuntimeError("Ollama endpoint redirects are denied")

opener = urllib.request.build_opener(RejectRedirects)

def get(path):
    with opener.open(host + path, timeout=10) as response:
        return json.load(response)

def post(path, payload):
    encoded = json.dumps(payload).encode("utf-8")
    command = urllib.request.Request(
        host + path,
        data=encoded,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with opener.open(command, timeout=240) as response:
        return json.load(response)

models = get("/api/tags").get("models", [])
matching = [entry for entry in models if entry.get("name") == request["model_identity"]]
installed_digest = matching[0].get("digest", "") if len(matching) == 1 else ""
if installed_digest.removeprefix("sha256:") != request["model_artifact_sha256"]:
    raise RuntimeError("configured model identity or digest is not locally installed")

generated = post("/api/generate", {
    "model": request["model_identity"],
    "prompt": request["prompt"],
    "stream": False,
    "options": {"temperature": 0.0, "top_p": 0.1},
})
loaded = get("/api/ps").get("models", [])
resident = [entry for entry in loaded if entry.get("name") == request["model_identity"]]
if len(resident) != 1 or int(resident[0].get("size_vram", 0)) <= 0:
    raise RuntimeError("configured model is not resident on the local GPU backend")

response = {
    key: request[key]
    for key in [
        "correlation_id", "runtime_id", "nonce", "backend_identity",
        "model_identity", "model_artifact_sha256"
    ]
}
response["schema"] = "adl.runtime.shepherd_runner_response.v1"
response["response"] = generated["response"]
print(json.dumps(response, sort_keys=True))
"#;

#[test]
fn local_model_origin_accepts_only_structural_loopback_http_origins() {
    for accepted in [
        "http://localhost:11434",
        "http://LOCALHOST:11434",
        "http://127.0.0.1:11434",
        "http://127.255.0.1:11434",
        "http://[::1]:11434",
    ] {
        assert_eq!(
            validate_loopback_ollama_origin(accepted),
            Ok(()),
            "{accepted}"
        );
    }

    for rejected in [
        "https://localhost:11434",
        "http://localhost",
        "http://localhost:0",
        "http://localhost:70000",
        "http://localhost:11434/",
        "http://localhost:11434/api",
        "http://localhost:11434?target=evil.example",
        "http://localhost:11434#evil.example",
        "http://localhost@evil.example:11434",
        "http://localhost:11434@evil.example:80",
        "http://localhost.evil.example:11434",
        "http://127.0.0.1.evil.example:11434",
        "http://2130706433:11434",
        "http://%31%32%37.0.0.1:11434",
        "http://[::2]:11434",
        " http://127.0.0.1:11434",
        "http://127.0.0.1:11434\n",
    ] {
        assert!(
            validate_loopback_ollama_origin(rejected).is_err(),
            "unsafe origin accepted: {rejected:?}"
        );
    }
}

#[test]
fn local_model_runner_denies_redirects() {
    let redirect_target = TcpListener::bind("127.0.0.1:0").unwrap();
    redirect_target.set_nonblocking(true).unwrap();
    let target_address = redirect_target.local_addr().unwrap();
    let target_reached = Arc::new(AtomicBool::new(false));
    let target_reached_worker = Arc::clone(&target_reached);
    let target = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match redirect_target.accept() {
                Ok(_) => {
                    target_reached_worker.store(true, Ordering::SeqCst);
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("redirect target accept failed: {error}"),
            }
        }
    });

    let source = TcpListener::bind("127.0.0.1:0").unwrap();
    let source_address = source.local_addr().unwrap();
    let redirect = thread::spawn(move || {
        let (mut stream, _) = source.accept().unwrap();
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request).unwrap();
        write!(
            stream,
            "HTTP/1.1 302 Found\r\nLocation: http://{target_address}/api/tags\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
    });

    let mut child = Command::new("python3")
        .arg("-c")
        .arg(OLLAMA_ATTESTED_RUNNER)
        .env("ADL_OLLAMA_HOST", format!("http://{source_address}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 must be available for the attested local-model runner");
    child.stdin.take().unwrap().write_all(b"{}\n").unwrap();
    let output = child.wait_with_output().unwrap();
    redirect.join().unwrap();
    target.join().unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Ollama endpoint redirects are denied")
    );
    assert!(!target_reached.load(Ordering::SeqCst));
}

#[tokio::test]
#[ignore = "requires an explicitly configured Ollama Gemma GPU runtime and model"]
async fn real_local_model_smoke() {
    use std::os::unix::fs::PermissionsExt;

    let ollama_host = std::env::var("ADL_SHEPHERD_OLLAMA_HOST")
        .unwrap_or_else(|_| "http://127.0.0.1:11434".to_owned());
    validate_loopback_ollama_origin(&ollama_host)
        .expect("ADL_SHEPHERD_OLLAMA_HOST must be a structural loopback HTTP origin");
    let model_identity = std::env::var("ADL_SHEPHERD_MODEL_IDENTITY")
        .expect("ADL_SHEPHERD_MODEL_IDENTITY is required");
    let backend_identity = std::env::var("ADL_SHEPHERD_BACKEND_IDENTITY")
        .unwrap_or_else(|_| "ollama_metal_local".to_owned());
    let model_artifact_sha256 = std::env::var("ADL_SHEPHERD_MODEL_DIGEST_SHA256")
        .expect("ADL_SHEPHERD_MODEL_DIGEST_SHA256 is required");
    assert_eq!(model_artifact_sha256.len(), 64);
    assert!(model_artifact_sha256
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')));

    let temp = tempfile::tempdir().unwrap();
    let runner = temp.path().join("ollama-attested-runner.py");
    fs::write(&runner, OLLAMA_ATTESTED_RUNNER).unwrap();
    fs::set_permissions(&runner, fs::Permissions::from_mode(0o700)).unwrap();
    let runner_program_sha256 = hex::encode(Sha256::digest(fs::read(&runner).unwrap()));

    let environment = BTreeMap::from([("ADL_OLLAMA_HOST".to_owned(), ollama_host)]);
    let mut config = LocalShepherdConfig::real_local_model(
        "runtime-v3-local",
        runner,
        vec![],
        environment,
        ShepherdModelIdentity::new(
            runner_program_sha256.clone(),
            backend_identity,
            model_identity,
            model_artifact_sha256.clone(),
        ),
    );
    config.timeout = Duration::from_secs(300);
    config.max_output_bytes = 128 * 1024;
    let executor = LocalShepherdExecutor::configured(config).unwrap();
    let payload = serde_json::to_vec(&json!({
        "schema": SHEPHERD_REQUEST_SCHEMA,
        "correlation_id": "wp-5795-real-local-smoke",
        "runtime_id": "runtime-v3-local",
        "prompt": "Reply with exactly: ADL local Shepherd is present",
    }))
    .unwrap();
    let result = executor
        .execute(&OperationRequest {
            schema: OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: "wp-5795-real-local-smoke".to_owned(),
            idempotency_key: "wp-5795-real-local-smoke".to_owned(),
            principal: "operator".to_owned(),
            payload,
            permit: None,
        })
        .await
        .unwrap();
    let response: ShepherdResponse = serde_json::from_slice(&result).unwrap();
    assert_eq!(
        response.execution_class,
        ShepherdExecutionClass::RealLocalModel
    );
    assert_eq!(response.provenance, ShepherdProvenance::LiveExecution);
    assert!(!response.retained);
    assert_eq!(response.correlation_id, "wp-5795-real-local-smoke");
    assert!(!response.response.trim().is_empty());
    assert_eq!(
        response.backend_identity_sha256.as_deref().map(str::len),
        Some(64)
    );
    assert_eq!(
        response.model_artifact_sha256.as_deref(),
        Some(model_artifact_sha256.as_str())
    );
    assert_eq!(response.model_identity_sha256.len(), 64);
    assert_eq!(response.runner_program_sha256, runner_program_sha256);
    assert_eq!(response.runner_launch_sha256.len(), 64);
    assert_eq!(
        response.runner_nonce_sha256.as_deref().map(str::len),
        Some(64)
    );
    assert_eq!(response.response_sha256.len(), 64);
    println!(
        "{}",
        serde_json::to_string(&json!({
            "schema": "adl.runtime.shepherd_local_model_smoke.v1",
            "execution_class": response.execution_class,
            "provenance": response.provenance,
            "retained": response.retained,
            "correlation_id": response.correlation_id,
            "backend_identity_sha256": response.backend_identity_sha256,
            "model_identity_sha256": response.model_identity_sha256,
            "model_artifact_sha256": response.model_artifact_sha256,
            "runner_program_sha256": response.runner_program_sha256,
            "runner_launch_sha256": response.runner_launch_sha256,
            "runner_nonce_sha256": response.runner_nonce_sha256,
            "response_sha256": response.response_sha256,
        }))
        .unwrap()
    );
}
