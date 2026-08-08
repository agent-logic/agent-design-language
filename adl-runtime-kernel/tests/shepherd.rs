use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

use adl_runtime_kernel::{
    AdapterKind, AdapterPolicy, AuthorityMode, ExecutionPermit, OperationError, OperationExecutor,
    OperationRequest, OperationalAdapter, OPERATION_REQUEST_SCHEMA,
};
use ed25519_dalek::SigningKey;
use serde_json::json;
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

mod operations {
    pub use adl_runtime_kernel::{
        ExecutorError, FailureClass, OperationExecutor, OperationRequest,
    };
}

#[path = "../src/shepherd.rs"]
mod shepherd;

use shepherd::{
    LocalShepherdConfig, LocalShepherdExecutor, ShepherdError, ShepherdExecutionClass,
    ShepherdResponse, SHEPHERD_REQUEST_SCHEMA, SHEPHERD_RESPONSE_SCHEMA,
};

fn write_script(temp: &TempDir, name: &str, body: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let path = temp.path().join(name);
    fs::write(&path, format!("#!/bin/sh\nset -eu\n{body}\n")).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    path
}

fn config(program: &Path, arguments: Vec<String>) -> LocalShepherdConfig {
    let mut config = LocalShepherdConfig::deterministic_test_double(
        "runtime-test",
        program.to_path_buf(),
        arguments,
    );
    config.timeout = Duration::from_millis(300);
    config.max_prompt_bytes = 128;
    config.max_output_bytes = 256;
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

#[tokio::test]
async fn governed_request_invokes_local_process_and_classifies_test_evidence() {
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "respond",
        "read prompt\nprintf 'answer:%s' \"$prompt\"",
    );
    let executor = Arc::new(LocalShepherdExecutor::configured(config(&script, vec![])).unwrap());
    let key = SigningKey::from_bytes(&[41; 32]);
    let adapter = OperationalAdapter::with_permit_keys(
        AdapterKind::Shepherd,
        policy(),
        executor,
        BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
    )
    .unwrap();
    let mut request = request("hello");
    request.permit = Some(signed_permit(&request, &key));

    let result = adapter.invoke(request).await.unwrap();
    let response: ShepherdResponse = serde_json::from_slice(&result.payload).unwrap();
    assert_eq!(response.schema, SHEPHERD_RESPONSE_SCHEMA);
    assert_eq!(
        response.execution_class,
        ShepherdExecutionClass::DeterministicTestDouble
    );
    assert!(!response.retained);
    assert_eq!(response.response, "answer:hello");
    assert_eq!(response.model_identity_sha256.len(), 64);
    assert_eq!(response.response_sha256.len(), 64);
}

#[tokio::test]
async fn provider_environment_is_cleared_and_allow_listed() {
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(
        &temp,
        "environment",
        "printf '%s:%s' \"${HOME-unset}\" \"${ADL_ALLOWED-unset}\"",
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

#[tokio::test]
async fn missing_authority_fails_before_process_invocation() {
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("invoked");
    let script = write_script(
        &temp,
        "mark",
        &format!("touch '{}'\nprintf answer", marker.display()),
    );
    let key = SigningKey::from_bytes(&[42; 32]);
    let adapter = OperationalAdapter::with_permit_keys(
        AdapterKind::Shepherd,
        policy(),
        Arc::new(LocalShepherdExecutor::configured(config(&script, vec![])).unwrap()),
        BTreeMap::from([("operator".to_owned(), key.verifying_key())]),
    )
    .unwrap();

    assert_eq!(
        adapter.invoke(request("hello")).await.unwrap_err(),
        OperationError::MissingAuthority
    );
    assert!(!marker.exists());
}

#[tokio::test]
async fn malformed_oversized_and_wrong_runtime_requests_fail_before_invocation() {
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
        executor.execute(&malformed).await.unwrap_err().message,
        "shepherd_invalid_request"
    );

    let oversized = request(&"x".repeat(129));
    assert_eq!(
        executor.execute(&oversized).await.unwrap_err().message,
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
        executor.execute(&wrong_runtime).await.unwrap_err().message,
        "shepherd_wrong_runtime"
    );
    assert!(!marker.exists());
}

#[tokio::test]
async fn timeout_releases_capacity_and_runtime_remains_usable() {
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("first");
    let script = write_script(
        &temp,
        "recover",
        &format!(
            "if [ ! -e '{}' ]; then touch '{}'; exec sleep 5; fi\nread prompt\nprintf 'recovered:%s' \"$prompt\"",
            marker.display(),
            marker.display()
        ),
    );
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();

    assert_eq!(
        executor
            .execute(&request("first"))
            .await
            .unwrap_err()
            .message,
        "shepherd_timeout"
    );
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("second")).await.unwrap()).unwrap();
    assert_eq!(response.response, "recovered:second");
}

#[tokio::test]
async fn stdin_backpressure_is_covered_by_timeout() {
    let temp = tempfile::tempdir().unwrap();
    let script = write_script(&temp, "never-read", "exec sleep 5");
    let mut blocked = config(&script, vec![]);
    blocked.max_prompt_bytes = 128 * 1024;
    let executor = LocalShepherdExecutor::configured(blocked).unwrap();
    let started = Instant::now();

    assert_eq!(
        executor
            .execute(&request(&"x".repeat(128 * 1024)))
            .await
            .unwrap_err()
            .message,
        "shepherd_timeout"
    );
    assert!(started.elapsed() < Duration::from_secs(1));
}

#[tokio::test]
async fn cancellation_is_bounded_and_releases_capacity() {
    let temp = tempfile::tempdir().unwrap();
    let marker = temp.path().join("first");
    let script = write_script(
        &temp,
        "pending",
        &format!(
            "if [ ! -e '{}' ]; then touch '{}'; exec sleep 5; fi\nread prompt\nprintf 'recovered:%s' \"$prompt\"",
            marker.display(),
            marker.display()
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
    assert_eq!(error.message, "shepherd_cancelled");
    let response: ShepherdResponse =
        serde_json::from_slice(&executor.execute(&request("second")).await.unwrap()).unwrap();
    assert_eq!(response.response, "recovered:second");
}

#[tokio::test]
async fn pre_cancelled_request_never_invokes_provider() {
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

    assert_eq!(
        executor
            .execute_with_cancellation(&request("cancelled"), &cancellation)
            .await
            .unwrap_err()
            .message,
        "shepherd_cancelled"
    );
    assert!(!marker.exists());
}

#[tokio::test]
async fn concurrent_request_is_rejected_without_exceeding_capacity() {
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
    let executor = LocalShepherdExecutor::configured(config(&script, vec![])).unwrap();
    let first_executor = executor.clone();
    let first = tokio::spawn(async move { first_executor.execute(&request("first")).await });
    tokio::time::timeout(Duration::from_millis(100), async {
        while !marker.exists() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();

    assert_eq!(
        executor
            .execute(&request("second"))
            .await
            .unwrap_err()
            .message,
        "shepherd_saturated"
    );
    assert_eq!(
        first.await.unwrap().unwrap_err().message,
        "shepherd_timeout"
    );
}

#[tokio::test]
async fn provider_failure_redacts_prompt_and_stderr() {
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

    assert_eq!(error.message, "shepherd_process_failed");
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
        "model",
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
    assert!(matches!(
        LocalShepherdExecutor::configured(unbounded),
        Err(ShepherdError::InvalidConfiguration)
    ));
}

#[tokio::test]
async fn unavailable_and_excess_output_are_truthful() {
    assert_eq!(
        LocalShepherdExecutor::unavailable()
            .execute(&request("hello"))
            .await
            .unwrap_err()
            .message,
        "shepherd_unavailable"
    );

    let temp = tempfile::tempdir().unwrap();
    let script = write_script(&temp, "large", "head -c 300 /dev/zero | tr '\\0' x");
    assert_eq!(
        LocalShepherdExecutor::configured(config(&script, vec![]))
            .unwrap()
            .execute(&request("hello"))
            .await
            .unwrap_err()
            .message,
        "shepherd_output_too_large"
    );
}
