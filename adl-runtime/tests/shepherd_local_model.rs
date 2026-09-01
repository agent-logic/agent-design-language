use std::{collections::BTreeMap, fs, time::Duration};

use adl_runtime_kernel::{
    LocalShepherdConfig, LocalShepherdExecutor, OperationExecutor, OperationRequest,
    ShepherdExecutionClass, ShepherdModelIdentity, ShepherdProvenance, ShepherdResponse,
    OPERATION_REQUEST_SCHEMA, SHEPHERD_REQUEST_SCHEMA,
};
use serde_json::json;
use sha2::{Digest, Sha256};

#[tokio::test]
#[ignore = "requires an explicitly configured Ollama Gemma GPU runtime and model"]
async fn real_local_model_smoke() {
    use std::os::unix::fs::PermissionsExt;

    let ollama_host = std::env::var("ADL_SHEPHERD_OLLAMA_HOST")
        .unwrap_or_else(|_| "http://127.0.0.1:11434".to_owned());
    assert!(
        ollama_host.starts_with("http://127.0.0.1:")
            || ollama_host.starts_with("http://localhost:"),
        "ADL_SHEPHERD_OLLAMA_HOST must be loopback HTTP"
    );
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
    fs::write(
        &runner,
        r#"#!/usr/bin/python3
import json, os, sys, urllib.request

request = json.loads(sys.stdin.readline())
host = os.environ["ADL_OLLAMA_HOST"].rstrip("/")

def get(path):
    with urllib.request.urlopen(host + path, timeout=10) as response:
        return json.load(response)

def post(path, payload):
    encoded = json.dumps(payload).encode("utf-8")
    command = urllib.request.Request(
        host + path,
        data=encoded,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(command, timeout=240) as response:
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
"#,
    )
    .unwrap();
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
