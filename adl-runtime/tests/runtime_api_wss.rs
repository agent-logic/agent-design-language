use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

use adl_runtime::{
    acip::{
        deterministic_json_to_protobuf, encode_semantic_envelope, negotiate_version,
        protobuf_to_deterministic_json, AcipEnvelopeInput, AcipNegotiationOffer,
        CSM_ACIP_MAX_REQUIRED_FEATURES, CSM_ACIP_PROTOCOL_FAMILY, CSM_ACIP_SUPPORTED_FEATURES,
        CSM_ACIP_VERSION_MAJOR, CSM_ACIP_VERSION_MINOR,
    },
    runtime_api::{
        runtime_api_health_report, runtime_api_telemetry_event, serve_runtime_api_listener_until,
        RuntimeApiCapabilityHealth, RuntimeApiFeatureMatrix, RuntimeApiHealthState,
        RuntimeApiService, RuntimeApiTelemetryConfig, RuntimeApiTelemetrySink,
        CSM_RUNTIME_API_DEFAULT_PORT, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA,
        CSM_RUNTIME_API_WSS_SESSION_SCHEMA,
    },
    runtime_api_auth::{
        RuntimeApiCredentialStore, RuntimeApiWssAdmissionPolicy,
        CSM_RUNTIME_API_WSS_MAX_FRAME_BYTES, CSM_RUNTIME_API_WSS_MAX_REPLAY_ENTRIES,
    },
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use futures::{SinkExt, StreamExt};
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        http::{
            header::{AUTHORIZATION, ORIGIN},
            HeaderValue, Request,
        },
        Message,
    },
    Connector, MaybeTlsStream, WebSocketStream,
};

#[path = "support/tls.rs"]
mod tls_support;
use tls_support::{TestIdentity, TestPki};

fn health() -> adl_runtime::runtime_api::RuntimeApiHealthReport {
    runtime_api_health_report(vec![
        RuntimeApiCapabilityHealth {
            capability: "authenticated_wss".into(),
            state: RuntimeApiHealthState::Healthy,
            reason_code: "loopback_tls_wss_exchange_passed".into(),
            evidence_ref: "adl-runtime/tests/runtime_api_wss.rs".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "html_observatory_ui".into(),
            state: RuntimeApiHealthState::Unimplemented,
            reason_code: "separate_client_boundary".into(),
            evidence_ref: "demos/html-observatory/README.md".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "cloud_sink".into(),
            state: RuntimeApiHealthState::Unavailable,
            reason_code: "no_configured_sink".into(),
            evidence_ref: "local_no_aws".into(),
        },
        RuntimeApiCapabilityHealth {
            capability: "adapter_probe_negative_case".into(),
            state: RuntimeApiHealthState::Failed,
            reason_code: "negative_case_retained_for_observatory".into(),
            evidence_ref: "docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json"
                .into(),
        },
    ])
}

fn telemetry() -> RuntimeApiTelemetryConfig {
    RuntimeApiTelemetryConfig {
        schema: "adl.csm.runtime_api.telemetry_config.v1".into(),
        sinks: vec![RuntimeApiTelemetrySink {
            sink: "local_jsonl".into(),
            supported_fields: BTreeSet::from([
                "runtime_instance_id".into(),
                "event".into(),
                "health_state".into(),
            ]),
        }],
    }
}

fn matrix() -> RuntimeApiFeatureMatrix {
    serde_json::from_str(include_str!(
        "../../docs/milestones/v0.91.8/review/runtime/5665_feature_adapter_matrix.json"
    ))
    .unwrap()
}

async fn server(
    store: RuntimeApiCredentialStore,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<Result<(), String>>,
) {
    let pki = TestPki::new("runtime api wss");
    let identity = pki.server(&["localhost"]);
    server_with_tls(store, identity, pki.roots(), true).await
}

async fn server_with_tls(
    store: RuntimeApiCredentialStore,
    identity: TestIdentity,
    roots: RootCertStore,
    complete_chain: bool,
) -> (
    std::net::SocketAddr,
    Connector,
    tokio::sync::oneshot::Sender<()>,
    tokio::task::JoinHandle<Result<(), String>>,
) {
    let certificate = if complete_chain {
        identity.certificate_pem()
    } else {
        identity.leaf_only_pem()
    };
    let tls =
        axum_server::tls_rustls::RustlsConfig::from_pem(certificate, identity.private_key_pem())
            .await
            .unwrap();
    let connector = Connector::Rustls(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
        .await
        .unwrap();
    let address = listener.local_addr().unwrap();
    let service = Arc::new(RuntimeApiService::new(
        store,
        health(),
        telemetry(),
        matrix(),
        RuntimeApiWssAdmissionPolicy::new(
            "runtime-a",
            ["https://observatory.local".to_string()],
            ["runtime.inspect".to_string()],
            ["permit-1".to_string()],
        )
        .unwrap(),
    ));
    let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(serve_runtime_api_listener_until(
        service,
        listener,
        tls,
        async move {
            let _ = stop_rx.await;
        },
    ));
    (address, connector, stop_tx, task)
}

async fn assert_tls_connection_denied(
    identity: TestIdentity,
    roots: RootCertStore,
    complete_chain: bool,
) {
    let root = tempfile::tempdir().unwrap();
    let store = RuntimeApiCredentialStore::for_state_root(root.path());
    store.ensure().unwrap();
    let token = store.with_bearer_token(str::to_owned).unwrap();
    let (address, connector, stop, task) =
        server_with_tls(store, identity, roots, complete_chain).await;
    let error =
        connect_async_tls_with_config(request(address, &token), None, false, Some(connector))
            .await
            .unwrap_err();
    assert!(
        error.to_string().contains("certificate")
            || error.to_string().contains("peer")
            || error.to_string().contains("issuer"),
        "unexpected TLS rejection: {error}"
    );
    let _ = stop.send(());
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn runtime_wss_rejects_self_signed_wrong_san_unknown_ca_and_incomplete_chain() {
    let pki = TestPki::new("runtime api negative TLS");
    assert_tls_connection_denied(pki.self_signed_server(), pki.roots(), true).await;
    assert_tls_connection_denied(pki.wrong_san_server(), pki.roots(), true).await;
    assert_tls_connection_denied(pki.server(&["localhost"]), pki.wrong_roots(), true).await;
    assert_tls_connection_denied(pki.server(&["localhost"]), pki.roots(), false).await;
}

fn request(address: std::net::SocketAddr, token: &str) -> Request<()> {
    request_with_origin(address, token, Some("https://observatory.local"))
}

fn request_with_origin(
    address: std::net::SocketAddr,
    token: &str,
    origin: Option<&str>,
) -> Request<()> {
    let mut request = format!("wss://localhost:{}/v1/acip/ws", address.port())
        .into_client_request()
        .unwrap();
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    if let Some(origin) = origin {
        request
            .headers_mut()
            .insert(ORIGIN, HeaderValue::from_str(origin).unwrap());
    }
    request
}

fn semantic_envelope(sequence: u64, replay_id: &str) -> Vec<u8> {
    semantic_envelope_for(
        sequence,
        replay_id,
        "runtime-a",
        "runtime.inspect",
        "permit-1",
    )
}

fn semantic_envelope_for(
    sequence: u64,
    replay_id: &str,
    runtime_id: &str,
    capability: &str,
    authority: &str,
) -> Vec<u8> {
    encode_semantic_envelope(
        AcipEnvelopeInput {
            message_id: "message-1",
            source: "observatory",
            target: "guardian",
            route: "runtime.inspect",
            runtime_id,
            correlation_id: "correlation-1",
            causation_id: "causation-1",
            trace_id: "trace-1",
            replay_id,
            capability,
            authority,
            payload_type: "application/json",
            monotonic_sequence: sequence,
            acknowledgement_requested: true,
            error_code: None,
            required_features: &["correlation", "deterministic-json", "trace-context"],
        },
        &serde_json::json!({"command":"inspect","limit":8}),
    )
    .unwrap()
}

fn carrier_frame(bytes: &[u8], signature: Option<&str>, encoding: &str) -> String {
    let envelope = adl_runtime::acip::decode_protobuf_envelope(bytes).unwrap();
    let body = if encoding == "protobuf-base64" {
        serde_json::Value::String(URL_SAFE_NO_PAD.encode(bytes))
    } else {
        serde_json::from_str(&protobuf_to_deterministic_json(bytes).unwrap()).unwrap()
    };
    serde_json::json!({
        "type": "acip",
        "encoding": encoding,
        "correlation_id": envelope.correlation_id,
        "body": body,
        "control_signature": signature
    })
    .to_string()
}

async fn expect_policy_close(
    socket: &mut WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    reason: &str,
) {
    match socket.next().await.unwrap().unwrap() {
        Message::Close(Some(frame)) => assert_eq!(frame.reason, reason),
        other => panic!("expected policy close {reason}, got {other:?}"),
    }
}

async fn negotiate_session(socket: &mut WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>) {
    socket
        .send(Message::Text(
            serde_json::json!({
                "type": "negotiate",
                "body": {
                    "protocol_family": CSM_ACIP_PROTOCOL_FAMILY,
                    "supported_major": 1,
                    "minimum_minor": 0,
                    "maximum_minor": 0,
                    "required_features": ["correlation", "deterministic-json"]
                }
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    let response: serde_json::Value =
        serde_json::from_str(&socket.next().await.unwrap().unwrap().into_text().unwrap()).unwrap();
    assert_eq!(response["type"], "negotiated");
    assert_eq!(response["body"]["version_major"], 1);
}

fn protobuf_message_fields(proto: &str) -> BTreeMap<String, Vec<(String, String, u32, String)>> {
    let mut messages = BTreeMap::new();
    let mut current = None;
    for raw_line in proto.lines() {
        let line = raw_line.split("//").next().unwrap().trim();
        if let Some(name) = line
            .strip_prefix("message ")
            .and_then(|line| line.strip_suffix(" {"))
        {
            current = Some(name.to_string());
            messages.insert(name.to_string(), Vec::new());
            continue;
        }
        if line == "}" {
            current = None;
            continue;
        }
        let Some(message) = current.as_ref() else {
            continue;
        };
        let Some(field) = line.strip_suffix(';') else {
            continue;
        };
        let tokens = field.split_whitespace().collect::<Vec<_>>();
        let (cardinality, offset) = match tokens.first().copied() {
            Some("optional") | Some("repeated") => (tokens[0], 1),
            _ => ("singular", 0),
        };
        assert_eq!(
            tokens.get(offset + 2),
            Some(&"="),
            "invalid proto field: {line}"
        );
        messages.get_mut(message).unwrap().push((
            tokens[offset].to_string(),
            tokens[offset + 1].to_string(),
            tokens[offset + 3].parse().unwrap(),
            cardinality.to_string(),
        ));
    }
    messages
}

#[test]
fn acip_schema_roundtrip_negatives() {
    let proto = include_str!("../schemas/acip/v1/acip.proto");
    let catalog: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/acip/v1/catalog.json")).unwrap();
    let openapi: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/api/runtime-v3/v1/acip.openapi.json"
    ))
    .unwrap();
    assert!(proto.contains(&format!(
        "package agentlogic.acip.v{};",
        CSM_ACIP_VERSION_MAJOR
    )));
    assert_eq!(catalog["protocol_family"], CSM_ACIP_PROTOCOL_FAMILY);
    assert_eq!(
        catalog["version"],
        serde_json::json!({"major": CSM_ACIP_VERSION_MAJOR, "minor": CSM_ACIP_VERSION_MINOR})
    );
    assert_eq!(
        catalog["limits"],
        serde_json::json!({
            "maximum_frame_bytes": CSM_RUNTIME_API_WSS_MAX_FRAME_BYTES,
            "maximum_required_features": CSM_ACIP_MAX_REQUIRED_FEATURES,
            "maximum_replay_entries": CSM_RUNTIME_API_WSS_MAX_REPLAY_ENTRIES
        })
    );
    assert_eq!(
        catalog["json_projection"],
        serde_json::json!({
            "bytes": "base64url-no-padding",
            "uint64": "unsigned-decimal-string",
            "omission": "only-optional-null-fields",
            "ordering": "RFC8785-JCS",
            "payload_json": "embedded-RFC8785-JCS-string"
        })
    );
    assert_eq!(
        catalog["payload_contract"],
        serde_json::json!({
            "payload_type": "non-empty-string",
            "payload_json": "embedded-RFC8785-JCS-string",
            "protobuf_projection": "deterministic-field-number-order",
            "json_projection": "RFC8785-JCS"
        })
    );
    assert_eq!(
        catalog["supported_features"],
        serde_json::json!(CSM_ACIP_SUPPORTED_FEATURES)
    );
    let websocket = &openapi["paths"]["/v1/acip/ws"]["get"]["x-adl-websocket"];
    assert_eq!(
        websocket["clientFrames"],
        serde_json::json!([
            {
                "format": "binary",
                "schema": "adl.csm.acip_carrier.protobuf_envelope.v1"
            }
        ])
    );
    assert_eq!(
        websocket["serverFrames"],
        serde_json::json!([
            {"$ref": "#/components/schemas/DispatchResult"}
        ])
    );

    let fields = protobuf_message_fields(proto);
    let expected_fields = BTreeMap::from([
        (
            "Envelope".to_string(),
            vec![
                ("string", "schema", 1, "singular"),
                ("string", "message_id", 2, "singular"),
                ("string", "source", 3, "singular"),
                ("string", "target", 4, "singular"),
                ("string", "route", 5, "singular"),
                ("string", "payload_json", 6, "singular"),
                ("uint64", "monotonic_sequence", 7, "singular"),
                ("string", "protocol_family", 8, "singular"),
                ("uint32", "version_major", 9, "singular"),
                ("uint32", "version_minor", 10, "singular"),
                ("string", "runtime_id", 11, "singular"),
                ("string", "correlation_id", 12, "singular"),
                ("string", "causation_id", 13, "singular"),
                ("string", "trace_id", 14, "singular"),
                ("string", "replay_id", 15, "singular"),
                ("string", "capability", 16, "singular"),
                ("string", "authority", 17, "singular"),
                ("string", "payload_type", 18, "singular"),
                ("bool", "acknowledgement_requested", 19, "singular"),
                ("string", "error_code", 20, "optional"),
                ("string", "required_features", 21, "repeated"),
            ],
        ),
        (
            "NegotiationOffer".to_string(),
            vec![
                ("string", "protocol_family", 1, "singular"),
                ("uint32", "supported_major", 2, "singular"),
                ("uint32", "minimum_minor", 3, "singular"),
                ("uint32", "maximum_minor", 4, "singular"),
                ("string", "required_features", 5, "repeated"),
            ],
        ),
        (
            "NegotiatedVersion".to_string(),
            vec![
                ("string", "protocol_family", 1, "singular"),
                ("uint32", "version_major", 2, "singular"),
                ("uint32", "version_minor", 3, "singular"),
                ("string", "features", 4, "repeated"),
            ],
        ),
        (
            "Error".to_string(),
            vec![
                ("string", "code", 1, "singular"),
                ("string", "message", 2, "singular"),
                ("string", "correlation_id", 3, "singular"),
                ("bool", "retryable", 4, "singular"),
            ],
        ),
    ])
    .into_iter()
    .map(|(name, fields)| {
        (
            name,
            fields
                .into_iter()
                .map(|(kind, name, tag, cardinality)| {
                    (
                        kind.to_string(),
                        name.to_string(),
                        tag,
                        cardinality.to_string(),
                    )
                })
                .collect(),
        )
    })
    .collect::<BTreeMap<_, _>>();
    assert_eq!(fields, expected_fields);

    let expected_message_contracts = BTreeMap::from([
        (
            "Envelope",
            (
                "bidirectional",
                "runtime-api-bearer-plus-signed-control",
                true,
            ),
        ),
        (
            "NegotiationOffer",
            ("client-to-runtime", "runtime-api-bearer", false),
        ),
        (
            "NegotiatedVersion",
            ("runtime-to-client", "authenticated-session", false),
        ),
        (
            "Error",
            ("runtime-to-client", "authenticated-session", false),
        ),
    ]);
    for message in catalog["messages"].as_array().unwrap() {
        let name = message["name"].as_str().unwrap();
        let (direction, authentication, capability_required) = expected_message_contracts[name];
        assert_eq!(message["direction"], direction);
        assert_eq!(message["authentication"], authentication);
        assert_eq!(message["capability_required"], capability_required);
        let schema_fields = fields[name]
            .iter()
            .map(|(_, field, _, _)| field.as_str())
            .collect::<BTreeSet<_>>();
        let required_semantics = message["required_semantics"]
            .as_array()
            .unwrap()
            .iter()
            .map(|field| field.as_str().unwrap())
            .collect::<BTreeSet<_>>();
        assert!(required_semantics.is_subset(&schema_fields));
        assert_eq!(
            required_semantics,
            schema_fields
                .into_iter()
                .filter(|field| *field != "error_code")
                .collect()
        );
    }
    assert_eq!(
        catalog["messages"].as_array().unwrap().len(),
        expected_message_contracts.len()
    );
    assert_eq!(
        openapi["paths"]["/v1/acip/ws"]["get"]["x-acip-protocol"]["catalog"],
        "adl-runtime/schemas/acip/v1/catalog.json"
    );
    let protocol = &openapi["paths"]["/v1/acip/ws"]["get"]["x-acip-protocol"];
    assert_eq!(protocol["family"], CSM_ACIP_PROTOCOL_FAMILY);
    assert_eq!(protocol["major"], CSM_ACIP_VERSION_MAJOR);
    assert_eq!(protocol["minor"], CSM_ACIP_VERSION_MINOR);
    assert_eq!(
        protocol["maximumFrameBytes"],
        CSM_RUNTIME_API_WSS_MAX_FRAME_BYTES
    );
    assert_eq!(
        protocol["admission"],
        serde_json::json!({
            "session": ["server-tls", "runtime-api-bearer", "exact-origin"],
            "dispatch": [
                "negotiated-v1", "runtime-id", "signed-control", "capability",
                "authority", "replay-id", "monotonic-sequence", "frame-size"
            ]
        })
    );
    let bearer = &openapi["components"]["securitySchemes"]["runtimeApiBearer"];
    assert_eq!(bearer["type"], "http");
    assert_eq!(bearer["scheme"], "bearer");
    let bytes = semantic_envelope(u64::MAX, "replay-roundtrip");
    let json = protobuf_to_deterministic_json(&bytes).unwrap();
    assert_eq!(deterministic_json_to_protobuf(&json).unwrap(), bytes);
    assert!(json.contains(&format!(r#""monotonic_sequence":"{}""#, u64::MAX)));
    assert!(deterministic_json_to_protobuf("{\"unknown\":true}").is_err());

    let runtime_default = adl_runtime::acip::encode_protobuf_envelope(
        "message-parity",
        "agent-a",
        "agent-b",
        "invoke",
        &serde_json::json!({"a":1,"z":2}),
        7,
    )
    .unwrap();
    let kernel_default = adl_runtime_kernel::acip::encode_acip_envelope(
        "message-parity",
        "agent-a",
        "agent-b",
        "invoke",
        &serde_json::json!({"z":2,"a":1}),
        7,
    )
    .unwrap();
    assert_eq!(runtime_default, kernel_default);

    assert!(negotiate_version(&AcipNegotiationOffer {
        protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
        supported_major: 1,
        minimum_minor: 0,
        maximum_minor: 0,
        required_features: vec!["correlation".into()],
    })
    .is_ok());
    assert!(negotiate_version(&AcipNegotiationOffer {
        protocol_family: CSM_ACIP_PROTOCOL_FAMILY.to_string(),
        supported_major: 2,
        minimum_minor: 0,
        maximum_minor: 0,
        required_features: vec![],
    })
    .unwrap_err()
    .contains("unsupported protocol major"));
}

#[tokio::test]
async fn production_acip_wss() {
    let root = tempfile::tempdir().unwrap();
    let store = RuntimeApiCredentialStore::for_state_root(root.path());
    store.ensure().unwrap();
    let token = store.with_bearer_token(str::to_owned).unwrap();
    let (address, connector, stop, task) = server(store.clone()).await;
    let mut successful_exchanges = 0_u64;
    let mut negative_cases = 0_u64;

    let missing_origin = connect_async_tls_with_config(
        request_with_origin(address, &token, None),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(missing_origin.to_string().contains("403"));
    negative_cases += 1;
    let invalid_origin = connect_async_tls_with_config(
        request_with_origin(address, &token, Some("https://attacker.invalid")),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(invalid_origin.to_string().contains("403"));
    negative_cases += 1;

    for oversized in [
        Message::Text("x".repeat(CSM_RUNTIME_API_WSS_MAX_FRAME_BYTES + 1).into()),
        Message::Binary(vec![0; CSM_RUNTIME_API_WSS_MAX_FRAME_BYTES + 1].into()),
    ] {
        let (mut denied, _) = connect_async_tls_with_config(
            request(address, &token),
            None,
            false,
            Some(connector.clone()),
        )
        .await
        .unwrap();
        denied.next().await.unwrap().unwrap();
        denied.send(oversized).await.unwrap();
        expect_policy_close(&mut denied, "frame_size_refused").await;
        negative_cases += 1;
    }

    let (mut incompatible, _) = connect_async_tls_with_config(
        request(address, &token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    incompatible.next().await.unwrap().unwrap();
    incompatible
        .send(Message::Text(
            serde_json::json!({
                "type": "negotiate",
                "body": {
                    "protocol_family": CSM_ACIP_PROTOCOL_FAMILY,
                    "supported_major": 2,
                    "minimum_minor": 0,
                    "maximum_minor": 0,
                    "required_features": []
                }
            })
            .to_string()
            .into(),
        ))
        .await
        .unwrap();
    expect_policy_close(&mut incompatible, "negotiation_refused").await;
    negative_cases += 1;

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, &token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    socket.next().await.unwrap().unwrap();
    negotiate_session(&mut socket).await;

    for sequence in 1..=8 {
        let replay_id = format!("replay-{sequence}");
        let protobuf = semantic_envelope(sequence, &replay_id);
        let encoding = if sequence % 2 == 0 {
            "protobuf-base64"
        } else {
            "deterministic-json"
        };
        let signature = store.sign_wss_control(&protobuf).unwrap();
        socket
            .send(Message::Text(
                carrier_frame(&protobuf, Some(&signature), encoding).into(),
            ))
            .await
            .unwrap();
        let ack: serde_json::Value =
            serde_json::from_str(&socket.next().await.unwrap().unwrap().into_text().unwrap())
                .unwrap();
        assert_eq!(ack["type"], "ack");
        assert_eq!(ack["body"]["correlation_id"], "correlation-1");
        assert_eq!(ack["body"]["encoding"], encoding);
        let echoed = &ack["body"]["body"];
        let round_trip = if encoding == "protobuf-base64" {
            URL_SAFE_NO_PAD.decode(echoed.as_str().unwrap()).unwrap()
        } else {
            let canonical = serde_jcs::to_string(echoed).unwrap();
            deterministic_json_to_protobuf(&canonical).unwrap()
        };
        assert_eq!(round_trip, protobuf);
        successful_exchanges += 1;
    }
    socket.close(None).await.unwrap();

    let reconnect_bytes = semantic_envelope(10, "replay-reconnect");
    let reconnect_signature = store.sign_wss_control(&reconnect_bytes).unwrap();
    let reconnect_frame = carrier_frame(
        &reconnect_bytes,
        Some(&reconnect_signature),
        "protobuf-base64",
    );

    let (mut unnegotiated, _) = connect_async_tls_with_config(
        request(address, &token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    unnegotiated.next().await.unwrap().unwrap();
    unnegotiated
        .send(Message::Text(reconnect_frame.clone().into()))
        .await
        .unwrap();
    expect_policy_close(&mut unnegotiated, "negotiation_required").await;
    negative_cases += 1;

    let (mut reconnected, _) = connect_async_tls_with_config(
        request(address, &token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    reconnected.next().await.unwrap().unwrap();
    negotiate_session(&mut reconnected).await;
    reconnected
        .send(Message::Text(reconnect_frame.clone().into()))
        .await
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(
            &reconnected
                .next()
                .await
                .unwrap()
                .unwrap()
                .into_text()
                .unwrap()
        )
        .unwrap()["type"],
        "ack"
    );
    successful_exchanges += 1;
    reconnected
        .send(Message::Text(reconnect_frame.into()))
        .await
        .unwrap();
    expect_policy_close(&mut reconnected, "replay_refused").await;
    negative_cases += 1;

    for (bytes, signature, expected_reason) in [
        (
            semantic_envelope_for(
                11,
                "wrong-runtime",
                "runtime-b",
                "runtime.inspect",
                "permit-1",
            ),
            true,
            "wrong_runtime",
        ),
        (
            semantic_envelope_for(12, "unsigned", "runtime-a", "runtime.inspect", "permit-1"),
            false,
            "unsigned_control_refused",
        ),
        (
            semantic_envelope_for(
                13,
                "denied-capability",
                "runtime-a",
                "runtime.admin",
                "permit-1",
            ),
            true,
            "capability_denied",
        ),
        (
            semantic_envelope_for(
                14,
                "denied-authority",
                "runtime-a",
                "runtime.inspect",
                "forged-permit",
            ),
            true,
            "authority_denied",
        ),
    ] {
        let (mut denied, _) = connect_async_tls_with_config(
            request(address, &token),
            None,
            false,
            Some(connector.clone()),
        )
        .await
        .unwrap();
        denied.next().await.unwrap().unwrap();
        negotiate_session(&mut denied).await;
        let control_signature = signature.then(|| store.sign_wss_control(&bytes).unwrap());
        denied
            .send(Message::Text(
                carrier_frame(&bytes, control_signature.as_deref(), "protobuf-base64").into(),
            ))
            .await
            .unwrap();
        expect_policy_close(&mut denied, expected_reason).await;
        negative_cases += 1;
    }

    let _ = stop.send(());
    task.await.unwrap().unwrap();

    if let Some(path) = std::env::var_os("ADL_ACIP_PROOF_OUTPUT") {
        let proof = serde_json::json!({
            "schema": "adl.acip_native_platform_proof.v2",
            "producer": "adl-runtime/tests/runtime_api_wss.rs::production_acip_wss",
            "subject": "runtime_api_listener",
            "platform": std::env::var("ADL_ACIP_PLATFORM").unwrap_or_else(|_| std::env::consts::OS.to_owned()),
            "successful_exchanges": successful_exchanges,
            "negative_cases": negative_cases,
            "assertions": [
                {"name": "runtime_api_listener", "result": "passed", "evidence": "real rustls listener served RuntimeApiService over wss"},
                {"name": "rustls_wss", "result": "passed", "evidence": "native TLS WebSocket sessions completed"},
                {"name": "authenticated_bidirectional", "result": "passed", "evidence": "bearer and origin admission preceded request and acknowledgement frames"},
                {"name": "protobuf_json_parity", "result": "passed", "evidence": "eight alternating protobuf-base64 and deterministic-json envelopes round-tripped byte-identically"},
                {"name": "reconnect_backpressure", "result": "passed", "evidence": "reconnect required renegotiation and oversized frames closed fail-closed"},
                {"name": "replay_denied", "result": "passed", "evidence": "duplicate replay identity closed with replay_refused"},
                {"name": "denied_access", "result": "passed", "evidence": "origin runtime signature capability and authority denials closed fail-closed"}
            ]
        });
        std::fs::write(
            path,
            format!("{}\n", serde_json::to_string_pretty(&proof).unwrap()),
        )
        .unwrap();
    }
}

#[tokio::test]
async fn wss_auth_rotation_revocation_and_shutdown_are_real_tls_frames() {
    let root = tempfile::tempdir().unwrap();
    let store = RuntimeApiCredentialStore::for_state_root(root.path());
    store.ensure().unwrap();
    let first_token = store.with_bearer_token(str::to_owned).unwrap();
    let (address, connector, stop, task) = server(store.clone()).await;

    let denied = connect_async_tls_with_config(
        request(address, "wrong-token"),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap_err();
    assert!(denied.to_string().contains("401"));

    let (mut socket, _) = connect_async_tls_with_config(
        request(address, &first_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    let hello = socket.next().await.unwrap().unwrap().into_text().unwrap();
    assert!(hello.contains(CSM_RUNTIME_API_WSS_SESSION_SCHEMA));
    socket
        .send(Message::Text(
            serde_json::json!({"type":"ping","body":{"n":1}})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    let pong = socket.next().await.unwrap().unwrap().into_text().unwrap();
    assert!(pong.contains("\"type\":\"pong\""));
    assert!(pong.contains("\"n\":1"));
    socket
        .send(Message::Text(
            serde_json::json!({"type":"feature_matrix"})
                .to_string()
                .into(),
        ))
        .await
        .unwrap();
    let matrix_frame: RuntimeApiFeatureMatrix =
        serde_json::from_str(&socket.next().await.unwrap().unwrap().into_text().unwrap()).unwrap();
    assert_eq!(matrix_frame, matrix());

    store.rotate().unwrap();
    let (mut old_overlap_socket, _) = connect_async_tls_with_config(
        request(address, &first_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    assert!(old_overlap_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .contains("authenticated"));
    let second_token = store.with_bearer_token(str::to_owned).unwrap();
    let (mut rotated_socket, _) = connect_async_tls_with_config(
        request(address, &second_token),
        None,
        false,
        Some(connector.clone()),
    )
    .await
    .unwrap();
    assert!(rotated_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap()
        .contains("authenticated"));
    rotated_socket
        .send(Message::Text(
            serde_json::json!({"type":"shutdown"}).to_string().into(),
        ))
        .await
        .unwrap();
    let shutdown = rotated_socket
        .next()
        .await
        .unwrap()
        .unwrap()
        .into_text()
        .unwrap();
    assert!(shutdown.contains("shutdown_ack"));

    store.revoke().unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(2), socket.next())
            .await
            .unwrap(),
        Some(Ok(Message::Close(_)))
    ));
    let _ = stop.send(());
    let _ = task.await.unwrap();
}

#[test]
fn health_telemetry_matrix_and_init_file_are_truthful() {
    let health = health();
    let states = health
        .capabilities
        .iter()
        .map(|capability| capability.state)
        .collect::<BTreeSet<_>>();
    assert!(states.contains(&RuntimeApiHealthState::Unimplemented));
    assert!(states.contains(&RuntimeApiHealthState::Unavailable));
    assert!(states.contains(&RuntimeApiHealthState::Failed));
    assert!(states.contains(&RuntimeApiHealthState::Healthy));

    let telemetry = telemetry();
    let event = runtime_api_telemetry_event(
        &telemetry,
        "local_jsonl",
        &serde_json::json!({
            "runtime_instance_id": "runtime-1",
            "event": "health",
            "health_state": "healthy",
            "unsupported_cloud_field": "must_drop"
        }),
    )
    .unwrap();
    assert_eq!(event["payload"]["runtime_instance_id"], "runtime-1");
    assert!(event["payload"].get("unsupported_cloud_field").is_none());
    assert_eq!(event["dropped_unsupported_fields"], 1);

    let matrix = matrix();
    assert_eq!(matrix.schema, CSM_RUNTIME_API_FEATURE_MATRIX_SCHEMA);
    assert!(matrix.unresolved_claimed_features.is_empty());
    let features = matrix
        .rows
        .iter()
        .map(|row| row.feature.as_str())
        .collect::<BTreeSet<_>>();
    assert!(features.contains("wss_authenticated_bidirectional_exchange"));
    assert!(features.contains("observatory_health_distinctions"));
    assert!(features.contains("sink_bounded_telemetry"));
    assert!(features.contains("html_observatory_ui_redesign"));

    let init: toml::Value =
        toml::from_str(include_str!("../../infra/runtime-v3/runtime-api-5665.toml")).unwrap();
    assert_eq!(init["runtime_api"]["mode"].as_str(), Some("api_only"));
    assert_eq!(init["runtime_api"]["port"].as_integer(), Some(20_997));
    assert_eq!(
        init["runtime_api"]["wss_path"].as_str(),
        Some("/v1/acip/ws")
    );
    assert_eq!(
        init["runtime_api"]["auth"].as_str(),
        Some("runtime_api_bearer")
    );
    assert_eq!(CSM_RUNTIME_API_DEFAULT_PORT, 20_997);
}
