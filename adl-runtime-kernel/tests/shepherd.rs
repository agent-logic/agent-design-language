use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use adl_runtime_kernel::{
    AdapterKind, AdapterPolicy, AuthorityMode, ExecutionPermit, ExecutorError, LocalShepherdConfig,
    LocalShepherdExecutor, OperationError, OperationExecutor, OperationRequest, OperationalAdapter,
    ShepherdError, ShepherdExecutionClass, ShepherdFailureResponse, ShepherdModelIdentity,
    ShepherdProvenance, ShepherdResponse, OPERATION_REQUEST_SCHEMA, SHEPHERD_FAILURE_SCHEMA,
    SHEPHERD_REQUEST_SCHEMA, SHEPHERD_RESPONSE_SCHEMA, SHEPHERD_RUNNER_RESPONSE_SCHEMA,
};
use ed25519_dalek::SigningKey;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

static SHEPHERD_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn serialize_shepherd_process_test() -> tokio::sync::MutexGuard<'static, ()> {
    SHEPHERD_TEST_LOCK.lock().await
}

fn write_script(temp: &TempDir, name: &str, body: &str) -> PathBuf {
    write_executable(temp, name, &format!("#!/bin/sh\nset -eu\n{body}\n"))
}

fn write_python(temp: &TempDir, name: &str, body: &str) -> PathBuf {
    write_executable(temp, name, &format!("#!/usr/bin/python3\n{body}\n"))
}

fn write_executable(temp: &TempDir, name: &str, body: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let path = temp.path().join(name);
    fs::write(&path, body).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    path
}

fn config(program: &Path, arguments: Vec<String>) -> LocalShepherdConfig {
    let mut config = LocalShepherdConfig::deterministic_test_double(
        "runtime-test",
        program.to_path_buf(),
        arguments,
    );
    config.timeout = Duration::from_secs(2);
    config.max_prompt_bytes = 128;
    config.max_output_bytes = 1024;
    config
}

fn short_timeout_config(program: &Path, arguments: Vec<String>) -> LocalShepherdConfig {
    let mut config = config(program, arguments);
    config.timeout = Duration::from_millis(300);
    config
}

fn real_config(program: &Path) -> LocalShepherdConfig {
    let runner_digest = hex::encode(Sha256::digest(fs::read(program).unwrap()));
    let mut config = LocalShepherdConfig::real_local_model(
        "runtime-test",
        program.to_path_buf(),
        vec![],
        BTreeMap::new(),
        ShepherdModelIdentity::new(
            runner_digest,
            "ollama_metal_local",
            "gemma4:12b-mlx",
            "a".repeat(64),
        ),
    );
    config.timeout = Duration::from_secs(2);
    config.max_output_bytes = 1024;
    config
}

fn request(prompt: &str) -> OperationRequest {
    let payload = serde_json::to_vec(&json!({
        "schema": SHEPHERD_REQUEST_SCHEMA,
        "correlation_id": "corr-1",
        "runtime_id": "runtime-test",
        "prompt": prompt,
    }))
    .unwrap();
    OperationRequest {
        schema: OPERATION_REQUEST_SCHEMA.to_owned(),
        request_id: "request-1".to_owned(),
        idempotency_key: "idempotency-1".to_owned(),
        principal: "operator".to_owned(),
        payload,
        permit: None,
    }
}

fn policy() -> AdapterPolicy {
    AdapterPolicy {
        capacity: 4,
        max_in_flight: 1,
        shutdown_grace_millis: 100,
        max_attempts: 1,
        idempotency_entries: 8,
        authority: AuthorityMode::Governed,
    }
}

fn signed_permit(request: &OperationRequest, key: &SigningKey) -> ExecutionPermit {
    ExecutionPermit {
        permit_id: "permit-1".to_owned(),
        request_hash: "1".repeat(64),
        request_id: request.request_id.clone(),
        principal: request.principal.clone(),
        action: "shepherd.invoke".to_owned(),
        resource: "shepherd".to_owned(),
        units: 1,
        payload_hash: blake3::hash(&request.payload).to_hex().to_string(),
        policy_hash: "2".repeat(64),
        evidence_hash: "3".repeat(64),
        signing_key_id: "operator".to_owned(),
        signature: String::new(),
    }
    .sign(key)
    .unwrap()
}

fn reason(error: &ExecutorError) -> String {
    serde_json::from_str::<ShepherdFailureResponse>(&error.message)
        .map(|failure| failure.reason_code)
        .unwrap_or_else(|_| error.message.clone())
}

async fn wait_for_marker(path: &Path) {
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if path.exists() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("runner readiness marker was not observed within five seconds");
}

fn governed_adapter(executor: Arc<dyn OperationExecutor>, key: &SigningKey) -> OperationalAdapter {
    OperationalAdapter::with_permit_keys(
        AdapterKind::Shepherd,
        policy(),
        executor,
        BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
    )
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn governed_request_invokes_local_process_and_classifies_test_evidence() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "respond",
        "read prompt\nprintf 'answer:%s' \"$prompt\"",
    );
    let executor = Arc::new(LocalShepherdExecutor::configured(config(&script, vec![])).unwrap());
    let key = SigningKey::from_bytes(&[41; 32]);
    let adapter = governed_adapter(executor, &key);
    let mut request = request("hello");
    request.permit = Some(signed_permit(&request, &key));

    let result = adapter.invoke(request).await.unwrap();
    let response: ShepherdResponse = serde_json::from_slice(&result.payload).unwrap();
    assert_eq!(response.schema, SHEPHERD_RESPONSE_SCHEMA);
    assert_eq!(
        response.execution_class,
        ShepherdExecutionClass::DeterministicTestDouble
    );
    assert_eq!(response.provenance, ShepherdProvenance::LiveExecution);
    assert!(!response.retained);
    assert_eq!(response.response, "answer:hello");
    assert_eq!(response.model_identity_sha256.len(), 64);
    assert_eq!(response.runner_program_sha256.len(), 64);
    assert_eq!(response.runner_launch_sha256.len(), 64);
    assert!(response.runner_nonce_sha256.is_none());
    assert_eq!(response.response_sha256.len(), 64);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn completed_idempotency_replay_is_retained_and_does_not_reinvoke_provider() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("invocations");
    let script = write_script(
        &temp,
        "count",
        &format!(
            "printf x >> '{}'\nread prompt\nprintf 'answer:%s' \"$prompt\"",
            marker.display()
        ),
    );
    let key = SigningKey::from_bytes(&[40; 32]);
    let adapter = governed_adapter(
        Arc::new(LocalShepherdExecutor::configured(config(&script, vec![])).unwrap()),
        &key,
    );
    let mut request = request("hello");
    request.permit = Some(signed_permit(&request, &key));

    let first: ShepherdResponse =
        serde_json::from_slice(&adapter.invoke(request.clone()).await.unwrap().payload).unwrap();
    let replay: ShepherdResponse =
        serde_json::from_slice(&adapter.invoke(request).await.unwrap().payload).unwrap();

    assert_eq!(first.provenance, ShepherdProvenance::LiveExecution);
    assert!(!first.retained);
    assert_eq!(replay.provenance, ShepherdProvenance::IdempotencyReplay);
    assert!(replay.retained);
    assert_eq!(first.response_sha256, replay.response_sha256);
    assert_eq!(fs::read(marker).unwrap(), b"x");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn correlated_failure_and_replay_retention_are_truthful() {
    let _test_guard = serialize_shepherd_process_test().await;
    let key = SigningKey::from_bytes(&[39; 32]);
    let adapter = governed_adapter(Arc::new(LocalShepherdExecutor::unavailable()), &key);
    let mut request = request("hello");
    request.permit = Some(signed_permit(&request, &key));

    let first = adapter.invoke(request.clone()).await.unwrap_err();
    let replay = adapter.invoke(request).await.unwrap_err();
    let OperationError::Degraded(first) = first else {
        panic!("expected degraded failure")
    };
    let OperationError::Degraded(replay) = replay else {
        panic!("expected replayed degraded failure")
    };
    let first: ShepherdFailureResponse = serde_json::from_str(&first).unwrap();
    let replay: ShepherdFailureResponse = serde_json::from_str(&replay).unwrap();
    assert_eq!(first.schema, SHEPHERD_FAILURE_SCHEMA);
    assert_eq!(first.correlation_id, "corr-1");
    assert_eq!(first.runtime_id, "runtime-test");
    assert_eq!(first.execution_class, ShepherdExecutionClass::Unavailable);
    assert_eq!(first.provenance, ShepherdProvenance::LiveExecution);
    assert!(!first.retained);
    assert_eq!(first.reason_code, "shepherd_unavailable");
    assert_eq!(replay.provenance, ShepherdProvenance::IdempotencyReplay);
    assert!(replay.retained);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn real_classification_requires_exact_nonce_bound_runner_attestation() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let runner = write_python(
        &temp,
        "attested-runner",
        &format!(
            r#"import json, sys
request = json.loads(sys.stdin.readline())
response = {{key: request[key] for key in ["correlation_id", "runtime_id", "nonce", "backend_identity", "model_identity", "model_artifact_sha256"]}}
response["schema"] = "{}"
response["response"] = "attested-local-response"
print(json.dumps(response, sort_keys=True))"#,
            SHEPHERD_RUNNER_RESPONSE_SCHEMA
        ),
    );
    let executor = LocalShepherdExecutor::configured(real_config(&runner)).unwrap();
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("hello")).await.unwrap()).unwrap();

    assert_eq!(
        response.execution_class,
        ShepherdExecutionClass::RealLocalModel
    );
    assert_eq!(response.provenance, ShepherdProvenance::LiveExecution);
    assert_eq!(response.response, "attested-local-response");
    assert_eq!(response.backend_identity_sha256.unwrap().len(), 64);
    assert_eq!(response.model_artifact_sha256.unwrap(), "a".repeat(64));
    assert_eq!(response.runner_nonce_sha256.unwrap().len(), 64);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn forged_real_runner_binding_is_rejected() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let runner = write_python(
        &temp,
        "forged-runner",
        &format!(
            r#"import json, sys
request = json.loads(sys.stdin.readline())
request["schema"] = "{}"
request["model_identity"] = "different-model"
request["response"] = "not-attested"
request.pop("prompt")
print(json.dumps(request, sort_keys=True))"#,
            SHEPHERD_RUNNER_RESPONSE_SCHEMA
        ),
    );
    let executor = LocalShepherdExecutor::configured(real_config(&runner)).unwrap();
    let error = executor.execute(&request("hello")).await.unwrap_err();

    assert_eq!(reason(&error), "shepherd_runner_attestation_failed");
}

#[test]
fn real_runner_bytes_must_match_the_operator_pinned_digest() {
    let temp = tempfile::tempdir().unwrap();
    let runner = write_python(&temp, "runner", "print('unreachable')");
    let mut configured = real_config(&runner);
    configured.expected_runner_program_sha256 = Some("f".repeat(64));

    assert!(matches!(
        LocalShepherdExecutor::configured(configured),
        Err(ShepherdError::AttestationFailed)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn configured_runner_executes_captured_bytes_after_source_replacement() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("replacement-executed");
    let runner = write_script(
        &temp,
        "runner",
        "read prompt\nprintf 'captured:%s' \"$prompt\"",
    );
    let executor = LocalShepherdExecutor::configured(config(&runner, vec![])).unwrap();
    write_executable(
        &temp,
        "runner",
        &format!(
            "#!/bin/sh\ntouch '{}'\nprintf replacement\n",
            marker.display()
        ),
    );

    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("hello")).await.unwrap()).unwrap();
    assert_eq!(response.response, "captured:hello");
    assert!(!marker.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn provider_environment_is_cleared_and_allow_listed() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "environment",
        "read prompt\nprintf '%s:%s' \"${HOME-unset}\" \"${ADL_ALLOWED-unset}\"",
    );
    let mut isolated = config(&script, vec![]);
    isolated
        .environment
        .insert("ADL_ALLOWED".to_owned(), "present".to_owned());
    let executor = LocalShepherdExecutor::configured(isolated).unwrap();

    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("hello")).await.unwrap()).unwrap();
    assert_eq!(response.response, "unset:present");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn every_authority_mutation_fails_before_process_invocation() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("invoked");
    let script = write_script(
        &temp,
        "mark",
        &format!("touch '{}'\nprintf answer", marker.display()),
    );
    let key = SigningKey::from_bytes(&[42; 32]);
    let adapter = governed_adapter(
        Arc::new(LocalShepherdExecutor::configured(config(&script, vec![])).unwrap()),
        &key,
    );
    let mut baseline = request("hello");
    baseline.permit = Some(signed_permit(&baseline, &key));

    let mut mutations = Vec::new();

    let mut invalid_signature = baseline.clone();
    invalid_signature.permit.as_mut().unwrap().signature = "00".repeat(64);
    mutations.push(invalid_signature);

    let mut payload = baseline.clone();
    payload.payload = serde_json::to_vec(&json!({
        "schema": SHEPHERD_REQUEST_SCHEMA,
        "correlation_id": "corr-1",
        "runtime_id": "runtime-test",
        "prompt": "changed-after-signing",
    }))
    .unwrap();
    mutations.push(payload);

    let mut principal = baseline.clone();
    principal.principal = "different-principal".to_owned();
    mutations.push(principal);

    let mut action = baseline.clone();
    action.permit.as_mut().unwrap().action = "provider.invoke".to_owned();
    mutations.push(action);

    let mut resource = baseline.clone();
    resource.permit.as_mut().unwrap().resource = "provider".to_owned();
    mutations.push(resource);

    let mut runtime = baseline;
    let mut body: Value = serde_json::from_slice(&runtime.payload).unwrap();
    body["runtime_id"] = Value::String("runtime-other".to_owned());
    runtime.payload = serde_json::to_vec(&body).unwrap();
    mutations.push(runtime);

    for mutation in mutations {
        assert_eq!(
            adapter.invoke(mutation).await.unwrap_err(),
            OperationError::MissingAuthority
        );
    }
    assert!(!marker.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn malformed_oversized_and_wrong_runtime_requests_fail_before_invocation() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("invoked");
    let script = write_script(
        &temp,
        "mark",
        &format!("touch '{}'\nprintf answer", marker.display()),
    );
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();

    let mut malformed = request("hello");
    malformed.payload = b"not-json".to_vec();
    assert_eq!(
        reason(&executor.execute(&malformed).await.unwrap_err()),
        "shepherd_invalid_request"
    );

    let oversized = request(&"x".repeat(129));
    assert_eq!(
        reason(&executor.execute(&oversized).await.unwrap_err()),
        "shepherd_invalid_request"
    );

    let mut wrong_runtime = request("hello");
    wrong_runtime.payload = serde_json::to_vec(&json!({
        "schema": SHEPHERD_REQUEST_SCHEMA,
        "correlation_id": "corr-1",
        "runtime_id": "runtime-other",
        "prompt": "hello",
    }))
    .unwrap();
    assert_eq!(
        reason(&executor.execute(&wrong_runtime).await.unwrap_err()),
        "shepherd_wrong_runtime"
    );
    assert!(!marker.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn timeout_releases_capacity_and_runtime_remains_usable() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("first");
    let script = write_script(
        &temp,
        "recover",
        &format!(
            "if [ ! -e '{}' ]; then touch '{}'; exec sleep 5; fi\nread prompt\nprintf 'recovered:%s' \"$prompt\"",
            marker.display(), marker.display()
        ),
    );
    let executor =
        LocalShepherdExecutor::configured(short_timeout_config(&script, vec![])).unwrap();

    assert_eq!(
        reason(&executor.execute(&request("first")).await.unwrap_err()),
        "shepherd_timeout"
    );
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("second")).await.unwrap()).unwrap();
    assert_eq!(response.response, "recovered:second");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn normal_parent_exit_kills_observed_descendants_and_bounds_pipe_drain() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("descendant-survived");
    let runner = write_python(
        &temp,
        "fork-and-exit",
        &format!(
            r#"import os, subprocess, sys, time
subprocess.Popen(["/bin/sh", "-c", "sleep 0.5; touch '{}'"], preexec_fn=os.setsid)
time.sleep(0.1)
sys.stdout.write("answer")
sys.stdout.flush()
os._exit(0)"#,
            marker.display()
        ),
    );
    let executor = LocalShepherdExecutor::configured(config(&runner, vec![])).unwrap();
    let started = Instant::now();
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("hello")).await.unwrap()).unwrap();
    assert_eq!(response.response, "answer");
    assert!(started.elapsed() < Duration::from_secs(1));
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert!(!marker.exists());
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn timeout_kills_observed_descendants() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("descendant-survived");
    let runner = write_python(
        &temp,
        "fork-and-wait",
        &format!(
            r#"import os, subprocess, time
subprocess.Popen(["/bin/sh", "-c", "sleep 0.5; touch '{}'"], preexec_fn=os.setsid)
time.sleep(5)"#,
            marker.display()
        ),
    );
    let executor =
        LocalShepherdExecutor::configured(short_timeout_config(&runner, vec![])).unwrap();
    assert_eq!(
        reason(&executor.execute(&request("hello")).await.unwrap_err()),
        "shepherd_timeout"
    );
    tokio::time::sleep(Duration::from_millis(700)).await;
    assert!(!marker.exists());
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropped_execution_future_kills_observed_descendants() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let started = temp.path().join("started");
    let escaped = temp.path().join("descendant-survived");
    let runner = write_python(
        &temp,
        "fork-for-drop",
        &format!(
            r#"import os, pathlib, subprocess, time
subprocess.Popen(["/bin/sh", "-c", "sleep 0.5; touch '{}'"], preexec_fn=os.setsid)
time.sleep(0.1)
pathlib.Path("{}").touch()
time.sleep(5)"#,
            escaped.display(),
            started.display()
        ),
    );
    let mut lifecycle = config(&runner, vec![]);
    lifecycle.timeout = Duration::from_secs(10);
    let executor = LocalShepherdExecutor::configured(lifecycle).unwrap();
    let task = tokio::spawn(async move { executor.execute(&request("hello")).await });
    wait_for_marker(&started).await;
    task.abort();
    let _ = task.await;
    tokio::time::sleep(Duration::from_millis(700)).await;
    assert!(!escaped.exists());
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn runner_inherits_declared_resource_limits() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let runner = write_python(
        &temp,
        "limits",
        r#"import json, resource, sys
memory_resource = resource.RLIMIT_DATA if sys.platform == "darwin" else resource.RLIMIT_AS
print(json.dumps({
  "memory": resource.getrlimit(memory_resource)[0],
  "cpu": resource.getrlimit(resource.RLIMIT_CPU)[0],
  "nofile": resource.getrlimit(resource.RLIMIT_NOFILE)[0],
  "nproc": resource.getrlimit(resource.RLIMIT_NPROC)[0]
}, sort_keys=True))"#,
    );
    let mut bounded = config(&runner, vec![]);
    bounded.max_memory_bytes = 768 * 1024 * 1024;
    bounded.max_cpu_seconds = 7;
    bounded.max_open_files = 48;
    bounded.max_processes = 1024;
    let executor = LocalShepherdExecutor::configured(bounded.clone()).unwrap();
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("hello")).await.unwrap()).unwrap();
    let limits: Value = serde_json::from_str(&response.response).unwrap();
    #[cfg(not(target_os = "macos"))]
    assert_eq!(limits["memory"].as_u64(), Some(bounded.max_memory_bytes));
    assert_eq!(limits["cpu"].as_u64(), Some(bounded.max_cpu_seconds));
    assert_eq!(limits["nofile"].as_u64(), Some(bounded.max_open_files));
    assert_eq!(limits["nproc"].as_u64(), Some(bounded.max_processes));
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn process_tree_memory_pressure_is_terminated_and_capacity_recovers() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("pressure-started");
    let runner = write_python(
        &temp,
        "memory-pressure",
        &format!(
            r#"import pathlib, subprocess, sys, time
marker = pathlib.Path("{}")
if marker.exists():
    print("recovered")
else:
    marker.touch()
    children = [
        subprocess.Popen([
            sys.executable,
            "-c",
            "import time; allocation = bytearray(8 * 1024 * 1024); time.sleep(5)",
        ])
        for _ in range(8)
    ]
    time.sleep(5)"#,
            marker.display()
        ),
    );
    let mut bounded = config(&runner, vec![]);
    // Leave enough per-process headroom for Python itself while the aggregate
    // process tree deterministically crosses the configured limit.
    bounded.max_memory_bytes = 64 * 1024 * 1024;
    bounded.timeout = Duration::from_secs(3);
    let executor = LocalShepherdExecutor::configured(bounded).unwrap();

    let error = executor.execute(&request("pressure")).await.unwrap_err();
    assert_eq!(reason(&error), "shepherd_resource_limit_exceeded");
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("recover")).await.unwrap()).unwrap();
    assert_eq!(response.response, "recovered");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stdin_backpressure_is_covered_by_timeout() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(&temp, "never-read", "exec sleep 5");
    let mut blocked = short_timeout_config(&script, vec![]);
    blocked.max_prompt_bytes = 128 * 1024;
    let executor = LocalShepherdExecutor::configured(blocked).unwrap();
    let started = Instant::now();

    assert_eq!(
        reason(
            &executor
                .execute(&request(&"x".repeat(128 * 1024)))
                .await
                .unwrap_err()
        ),
        "shepherd_timeout"
    );
    assert!(started.elapsed() < Duration::from_secs(1));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_is_bounded_and_releases_capacity() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("first");
    let script = write_script(
        &temp,
        "pending",
        &format!(
            "if [ ! -e '{}' ]; then touch '{}'; exec sleep 5; fi\nread prompt\nprintf 'recovered:%s' \"$prompt\"",
            marker.display(), marker.display()
        ),
    );
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();
    let cancellation = CancellationToken::new();
    let cancel = cancellation.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(25)).await;
        cancel.cancel();
    });

    let error = executor
        .execute_with_cancellation(&request("cancel"), &cancellation)
        .await
        .unwrap_err();
    assert_eq!(reason(&error), "shepherd_cancelled");
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("second")).await.unwrap()).unwrap();
    assert_eq!(response.response, "recovered:second");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_kills_observed_descendant_that_escaped_the_process_group() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let started = temp.path().join("started");
    let escaped = temp.path().join("descendant-survived");
    let runner = write_python(
        &temp,
        "setsid-for-cancellation",
        &format!(
            r#"import os, pathlib, subprocess, time
subprocess.Popen(["/bin/sh", "-c", "sleep 0.5; touch '{}'"], preexec_fn=os.setsid)
time.sleep(0.1)
pathlib.Path("{}").touch()
time.sleep(5)"#,
            escaped.display(),
            started.display()
        ),
    );
    let mut lifecycle = config(&runner, vec![]);
    lifecycle.timeout = Duration::from_secs(10);
    let executor = LocalShepherdExecutor::configured(lifecycle).unwrap();
    let cancellation = CancellationToken::new();
    let cancel = cancellation.clone();
    let execution = tokio::spawn(async move {
        executor
            .execute_with_cancellation(&request("hello"), &cancellation)
            .await
    });
    wait_for_marker(&started).await;
    cancel.cancel();
    assert_eq!(
        reason(&execution.await.unwrap().unwrap_err()),
        "shepherd_cancelled"
    );
    tokio::time::sleep(Duration::from_millis(700)).await;
    assert!(!escaped.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pre_cancelled_request_never_invokes_provider() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("invoked");
    let script = write_script(
        &temp,
        "mark-cancelled",
        &format!("touch '{}'\nprintf answer", marker.display()),
    );
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();
    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let error = executor
        .execute_with_cancellation(&request("cancelled"), &cancellation)
        .await
        .unwrap_err();
    assert_eq!(reason(&error), "shepherd_cancelled");
    assert!(!marker.exists());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_request_is_rejected_without_exceeding_capacity() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("started");
    let script = write_script(
        &temp,
        "busy",
        &format!(
            "touch '{}'\nsleep 1\nread prompt\nprintf 'answer:%s' \"$prompt\"",
            marker.display()
        ),
    );
    let executor =
        LocalShepherdExecutor::configured(short_timeout_config(&script, vec![])).unwrap();
    let first_executor = executor.clone();
    let first = tokio::spawn(async move { first_executor.execute(&request("first")).await });
    wait_for_marker(&marker).await;

    assert_eq!(
        reason(&executor.execute(&request("second")).await.unwrap_err()),
        "shepherd_saturated"
    );
    assert_eq!(
        reason(&first.await.unwrap().unwrap_err()),
        "shepherd_timeout"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn provider_failure_redacts_prompt_and_stderr() {
    let _test_guard = serialize_shepherd_process_test().await;
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "fail",
        "read prompt\nprintf 'provider leaked: %s' \"$prompt\" >&2\nexit 7",
    );
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();
    let error = executor
        .execute(&request("secret-prompt-value"))
        .await
        .unwrap_err();

    assert_eq!(reason(&error), "shepherd_process_failed");
    assert!(!error.message.contains("secret"));
    assert!(!error.message.contains("provider leaked"));
}

#[test]
fn invalid_configuration_is_rejected_before_execution() {
    let mut invalid = LocalShepherdConfig::real_local_model(
        "runtime with spaces",
        PathBuf::from("relative-program"),
        vec![],
        BTreeMap::new(),
        ShepherdModelIdentity::new("b".repeat(64), "backend", "model", "not-a-digest"),
    );
    invalid.max_in_flight = 0;
    assert!(matches!(
        LocalShepherdExecutor::configured(invalid),
        Err(ShepherdError::InvalidConfiguration)
    ));

    let temp = tempfile::tempdir().unwrap();
    let script = write_script(&temp, "valid", "printf answer");
    let mut unbounded = config(&script, vec![]);
    unbounded.max_prompt_bytes = usize::MAX;
    unbounded.max_output_bytes = usize::MAX;
    unbounded.max_in_flight = usize::MAX;
    unbounded.max_memory_bytes = u64::MAX;
    assert!(matches!(
        LocalShepherdExecutor::configured(unbounded),
        Err(ShepherdError::InvalidConfiguration)
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn unavailable_and_excess_output_are_truthful() {
    let _test_guard = serialize_shepherd_process_test().await;
    let unavailable = LocalShepherdExecutor::unavailable()
        .execute(&request("hello"))
        .await
        .unwrap_err();
    assert_eq!(reason(&unavailable), "shepherd_unavailable");

    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "large",
        "read prompt\n/usr/bin/head -c 1100 /dev/zero | /usr/bin/tr '\\0' x",
    );
    let error = LocalShepherdExecutor::configured(config(&script, vec![]))
        .unwrap()
        .execute(&request("hello"))
        .await
        .unwrap_err();
    assert_eq!(reason(&error), "shepherd_output_too_large");
}
