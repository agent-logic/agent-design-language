use std::{collections::BTreeSet, sync::Arc};

use adl_runtime::{
    runtime_api::{
        runtime_api_feature_matrix, runtime_api_health_report, runtime_api_router,
        RuntimeApiFeatureMatrixRow, RuntimeApiHealthState, RuntimeApiService,
        RuntimeApiTelemetryConfig, CSM_RUNTIME_API_DOCS_PATH,
        CSM_RUNTIME_API_OBSERVATORY_OPENAPI_PATH, CSM_RUNTIME_API_OPENAPI_PATH,
    },
    runtime_api_auth::RuntimeApiCredentialStore,
};
use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

const RUNTIME_OPENAPI: &str = include_str!("../../docs/api/runtime-v3/v1/openapi.json");
const OBSERVATORY_OPENAPI: &str =
    include_str!("../../docs/api/runtime-v3/v1/observatory.openapi.json");

fn service() -> Arc<RuntimeApiService> {
    let root = tempfile::tempdir().expect("temporary credential root");
    let store = RuntimeApiCredentialStore::for_state_root(root.path());
    store.ensure().expect("credential store");
    Arc::new(RuntimeApiService::new(
        store,
        runtime_api_health_report(Vec::new()),
        RuntimeApiTelemetryConfig {
            schema: "adl.csm.runtime_api.telemetry_config.v1".into(),
            sinks: Vec::new(),
        },
        runtime_api_feature_matrix(vec![RuntimeApiFeatureMatrixRow {
            feature: "openapi_docs".into(),
            adapter: "runtime_api_router".into(),
            claimed: true,
            health_state: RuntimeApiHealthState::Healthy,
            proof: "adl-runtime/tests/runtime_api_docs.rs".into(),
        }]),
    ))
}

#[tokio::test]
async fn runtime_and_observatory_specs_are_served_byte_exact_for_client_generation() {
    assert_spec_route(
        CSM_RUNTIME_API_OPENAPI_PATH,
        RUNTIME_OPENAPI,
        "ADL Runtime v3 Core API",
        "runtime-core",
        "/v1/docs/",
    )
    .await;
    assert_spec_route(
        CSM_RUNTIME_API_OBSERVATORY_OPENAPI_PATH,
        OBSERVATORY_OPENAPI,
        "ADL Observatory API",
        "observatory",
        "/v1/observatory",
    )
    .await;
}

#[tokio::test]
async fn swagger_docs_route_uses_embedded_ui_and_lists_both_openapi_specs() {
    let app = runtime_api_router(service());
    let redirect = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/docs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("docs redirect response");
    assert!(matches!(
        redirect.status(),
        StatusCode::MOVED_PERMANENTLY
            | StatusCode::FOUND
            | StatusCode::SEE_OTHER
            | StatusCode::TEMPORARY_REDIRECT
            | StatusCode::PERMANENT_REDIRECT
    ));
    assert_eq!(
        redirect
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some(CSM_RUNTIME_API_DOCS_PATH)
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri(CSM_RUNTIME_API_DOCS_PATH)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("docs response");
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("text/html"),
        "unexpected docs content-type {content_type}"
    );
    let body = String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("docs body bytes")
            .to_vec(),
    )
    .expect("docs body utf8");
    assert!(body.contains("swagger-ui"));

    let response = runtime_api_router(service())
        .oneshot(
            Request::builder()
                .uri("/v1/docs/swagger-initializer.js")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("docs initializer response");
    assert_eq!(response.status(), StatusCode::OK);
    let body = String::from_utf8(
        to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("docs initializer body bytes")
            .to_vec(),
    )
    .expect("docs initializer body utf8");
    assert!(body.contains("SwaggerUIBundle"));
    assert!(body.contains(CSM_RUNTIME_API_OPENAPI_PATH));
    assert!(body.contains(CSM_RUNTIME_API_OBSERVATORY_OPENAPI_PATH));
    assert!(body.contains("Runtime Core"));
    assert!(body.contains("Observatory"));
}

async fn assert_spec_route(
    route: &str,
    expected_bytes: &str,
    expected_title: &str,
    expected_identity: &str,
    expected_path: &str,
) {
    let response = runtime_api_router(service())
        .oneshot(Request::builder().uri(route).body(Body::empty()).unwrap())
        .await
        .expect("spec response");
    assert_eq!(response.status(), StatusCode::OK);
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("application/json"),
        "unexpected spec content-type {content_type}"
    );
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("spec body bytes");
    assert_eq!(&body[..], expected_bytes.as_bytes());

    let parsed: Value = serde_json::from_slice(&body).expect("served OpenAPI parses");
    assert_eq!(parsed["openapi"], "3.1.0");
    assert_eq!(parsed["info"]["title"], expected_title);
    assert_eq!(parsed["info"]["version"], "1.0.0");
    assert_eq!(parsed["x-adl-api-identity"], expected_identity);
    let paths = parsed["paths"].as_object().expect("paths object");
    assert!(paths.contains_key(expected_path));

    let operation_ids = paths
        .values()
        .flat_map(|methods| methods.as_object().into_iter().flatten())
        .filter_map(|(_, operation)| operation["operationId"].as_str())
        .collect::<BTreeSet<_>>();
    assert!(
        !operation_ids.is_empty(),
        "served spec must remain suitable for client generation"
    );
}
