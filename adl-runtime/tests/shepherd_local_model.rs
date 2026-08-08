use std::{collections::BTreeMap, path::PathBuf, time::Duration};

use adl_runtime_kernel::{OperationExecutor, OperationRequest, OPERATION_REQUEST_SCHEMA};
use serde_json::json;

mod operations {
    pub use adl_runtime_kernel::{ExecutorError, FailureClass, OperationExecutor, OperationRequest};
}

#[path = "../../adl-runtime-kernel/src/shepherd.rs"]
#[allow(dead_code)]
mod shepherd;

use shepherd::{
    LocalShepherdConfig, LocalShepherdExecutor, ShepherdExecutionClass, ShepherdResponse,
    SHEPHERD_REQUEST_SCHEMA,
};

#[tokio::test]
#[ignore = "requires explicitly configured local MLX/Gemma runtime and model"]
async fn real_local_model_smoke() {
    let program = PathBuf::from(
        std::env::var("ADL_SHEPHERD_PROGRAM")
            .expect("ADL_SHEPHERD_PROGRAM must name an absolute local executable"),
    );
    let arguments = serde_json::from_str::<Vec<String>>(
        &std::env::var("ADL_SHEPHERD_ARGUMENTS_JSON")
            .expect("ADL_SHEPHERD_ARGUMENTS_JSON must be a JSON string array"),
    )
    .expect("ADL_SHEPHERD_ARGUMENTS_JSON must be valid JSON");
    let environment = std::env::var("ADL_SHEPHERD_ENVIRONMENT_JSON")
        .ok()
        .map(|value| serde_json::from_str::<BTreeMap<String, String>>(&value).unwrap())
        .unwrap_or_default();
    let model_identity = std::env::var("ADL_SHEPHERD_MODEL_IDENTITY")
        .expect("ADL_SHEPHERD_MODEL_IDENTITY is required");
    let mut config = LocalShepherdConfig::real_local_model(
        "runtime-v3-local",
        program,
        arguments,
        environment,
        model_identity,
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
    assert_eq!(response.execution_class, ShepherdExecutionClass::RealLocalModel);
    assert!(!response.retained);
    assert_eq!(response.correlation_id, "wp-5795-real-local-smoke");
    assert!(!response.response.trim().is_empty());
    assert_eq!(response.model_identity_sha256.len(), 64);
    assert_eq!(response.response_sha256.len(), 64);
}
