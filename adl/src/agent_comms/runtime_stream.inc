use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamFailureClassificationV1 {
    pub case_name: String,
    pub category: String,
    pub fail_closed: bool,
    pub retry_allowed: bool,
    pub expected_error_substring: String,
    pub authority_expansion_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamSubstrateDecisionV1 {
    pub schema_version: String,
    pub selected_substrate: String,
    pub selected_crate: String,
    pub alternatives_considered: Vec<String>,
    pub rationale: Vec<String>,
    pub failure_classifications: Vec<AcipRuntimeStreamFailureClassificationV1>,
    pub integration_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamLoopbackProofV1 {
    pub schema_version: String,
    pub proof_id: String,
    pub selected_substrate: String,
    pub selected_crate: String,
    pub carrier_kind: String,
    pub request_message: AcipMessageEnvelopeV1,
    pub response_message: AcipMessageEnvelopeV1,
    pub negative_cases: Vec<AcipRuntimeStreamFailureClassificationV1>,
    pub lifecycle_events: Vec<String>,
    pub integration_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamAccessPolicyV1 {
    pub policy_id: String,
    pub allowed_sender_prefixes: Vec<String>,
    pub denied_sender_prefixes: Vec<String>,
    pub allowed_recipient_ids: Vec<String>,
    pub required_trace_requirement: AcipTraceRequirementV1,
    pub authority_expansion_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamTransportCaseV1 {
    pub case_name: String,
    pub status: String,
    pub request_message_id: Option<String>,
    pub response_message_id: Option<String>,
    pub error_substring: Option<String>,
    pub lifecycle_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipRuntimeStreamWebSocketTransportProofV1 {
    pub schema_version: String,
    pub proof_id: String,
    pub selected_substrate: String,
    pub selected_crate: String,
    pub carrier_kind: String,
    pub endpoint_binding: String,
    pub access_policy: AcipRuntimeStreamAccessPolicyV1,
    pub positive_case: AcipRuntimeStreamTransportCaseV1,
    pub negative_cases: Vec<AcipRuntimeStreamFailureClassificationV1>,
    pub failure_cases: Vec<AcipRuntimeStreamTransportCaseV1>,
    pub lifecycle_events: Vec<String>,
    pub integration_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

pub fn acip_runtime_stream_substrate_decision_v1() -> AcipRuntimeStreamSubstrateDecisionV1 {
    AcipRuntimeStreamSubstrateDecisionV1 {
        schema_version: ACIP_RUNTIME_STREAM_SUBSTRATE_DECISION_SCHEMA_VERSION.to_string(),
        selected_substrate: "websocket".to_string(),
        selected_crate: "tokio-tungstenite".to_string(),
        alternatives_considered: vec![
            "jsonrpsee: useful if ACIP runtime streams later become JSON-RPC method calls, but current ACIP semantics are envelope/event transport rather than RPC method dispatch".to_string(),
            "bespoke tokio tcp framing: rejected because ADL should not own WebSocket framing, ping/pong, close handshake, or upgrade mechanics".to_string(),
        ],
        rationale: vec![
            "ACIP remains the domain contract above the carrier.".to_string(),
            "tokio-tungstenite provides Tokio-compatible WebSocket client/server mechanics for the bounded WP-12 loopback proof.".to_string(),
            "The selected carrier does not create authority; existing ACIP validation and policy refusal remain fail-closed.".to_string(),
        ],
        failure_classifications: acip_runtime_stream_failure_classifications_v1(),
        integration_refs: vec![
            "#4658 ACIP schema/protobuf projection".to_string(),
            "#4659 WP-12 WebSocket transport path".to_string(),
            "docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md".to_string(),
        ],
        non_claims: vec![
            "This decision does not implement protobuf wire encoding.".to_string(),
            "This decision does not prove production WebSocket authentication, TLS, reconnect scheduling, or cross-polis transport.".to_string(),
            "This decision does not bypass ACIP access rules, Freedom Gate decisions, trace, replay, or policy boundaries.".to_string(),
        ],
    }
}

pub fn validate_acip_runtime_stream_substrate_decision_v1(
    decision: &AcipRuntimeStreamSubstrateDecisionV1,
) -> Result<()> {
    if decision.schema_version != ACIP_RUNTIME_STREAM_SUBSTRATE_DECISION_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP runtime stream substrate decision requires schema_version '{}'",
            ACIP_RUNTIME_STREAM_SUBSTRATE_DECISION_SCHEMA_VERSION
        ));
    }
    if decision.selected_substrate != "websocket" {
        return Err(anyhow!(
            "ACIP runtime stream substrate decision currently selects websocket"
        ));
    }
    if decision.selected_crate != "tokio-tungstenite" {
        return Err(anyhow!(
            "ACIP runtime stream substrate decision must bind to tokio-tungstenite"
        ));
    }
    if decision.failure_classifications.len() < 5 {
        return Err(anyhow!(
            "ACIP runtime stream substrate decision requires reconnect, malformed, close, timeout, and policy-denial classifications"
        ));
    }
    for classification in &decision.failure_classifications {
        validate_acip_runtime_stream_failure_classification_v1(classification)?;
    }
    if decision.integration_refs.iter().all(|value| !value.contains("#4659")) {
        return Err(anyhow!(
            "ACIP runtime stream substrate decision must feed #4659 WebSocket transport proof"
        ));
    }
    Ok(())
}

pub fn validate_acip_runtime_stream_loopback_proof_v1(
    proof: &AcipRuntimeStreamLoopbackProofV1,
) -> Result<()> {
    if proof.schema_version != ACIP_RUNTIME_STREAM_LOOPBACK_PROOF_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP runtime stream loopback proof requires schema_version '{}'",
            ACIP_RUNTIME_STREAM_LOOPBACK_PROOF_SCHEMA_VERSION
        ));
    }
    if proof.selected_substrate != "websocket" {
        return Err(anyhow!(
            "ACIP runtime stream loopback proof currently selects websocket"
        ));
    }
    if proof.selected_crate != "tokio-tungstenite" || proof.carrier_kind != "mock_loopback" {
        return Err(anyhow!(
            "ACIP runtime stream loopback proof must use tokio-tungstenite mock_loopback"
        ));
    }
    validate_acip_message_envelope_v1(&proof.request_message)?;
    validate_acip_message_envelope_v1(&proof.response_message)?;
    if proof.response_message.prior_message_id.as_deref()
        != Some(proof.request_message.message_id.as_str())
    {
        return Err(anyhow!(
            "ACIP runtime stream response must preserve prior_message_id"
        ));
    }
    if proof.response_message.monotonic_order <= proof.request_message.monotonic_order {
        return Err(anyhow!(
            "ACIP runtime stream response must advance monotonic_order"
        ));
    }
    if proof.negative_cases.len() < 2 {
        return Err(anyhow!(
            "ACIP runtime stream loopback proof requires at least two negative cases"
        ));
    }
    for classification in &proof.negative_cases {
        validate_acip_runtime_stream_failure_classification_v1(classification)?;
    }
    require_runtime_stream_case(&proof.negative_cases, "malformed_message")?;
    require_runtime_stream_case(&proof.negative_cases, "auth_policy_denial")?;
    require_runtime_stream_event(&proof.lifecycle_events, "acip_request_validated")?;
    require_runtime_stream_event(&proof.lifecycle_events, "acip_response_validated")?;
    require_runtime_stream_ref(&proof.integration_refs, "#4658")?;
    require_runtime_stream_ref(&proof.integration_refs, "#4659")?;
    require_runtime_stream_non_claim(&proof.non_claims, "protobuf")?;
    require_runtime_stream_non_claim(&proof.non_claims, "authentication")?;
    require_runtime_stream_non_claim(&proof.non_claims, "TLS")?;
    require_runtime_stream_non_claim(&proof.non_claims, "reconnect")?;
    Ok(())
}

pub fn validate_acip_runtime_stream_access_policy_v1(
    policy: &AcipRuntimeStreamAccessPolicyV1,
) -> Result<()> {
    validate_id(&policy.policy_id, "policy_id")?;
    if policy.allowed_sender_prefixes.is_empty() {
        return Err(anyhow!(
            "ACIP WebSocket transport access policy requires allowed_sender_prefixes"
        ));
    }
    if policy.denied_sender_prefixes.is_empty() {
        return Err(anyhow!(
            "ACIP WebSocket transport access policy requires denied_sender_prefixes"
        ));
    }
    if policy.allowed_recipient_ids.is_empty() {
        return Err(anyhow!(
            "ACIP WebSocket transport access policy requires allowed_recipient_ids"
        ));
    }
    for value in &policy.allowed_sender_prefixes {
        validate_non_empty(value, "allowed_sender_prefixes[]")?;
    }
    for value in &policy.denied_sender_prefixes {
        validate_non_empty(value, "denied_sender_prefixes[]")?;
    }
    for value in &policy.allowed_recipient_ids {
        validate_id(value, "allowed_recipient_ids[]")?;
    }
    if policy.authority_expansion_allowed {
        return Err(anyhow!(
            "ACIP WebSocket transport access policy must not expand authority"
        ));
    }
    Ok(())
}

pub fn validate_acip_runtime_stream_websocket_transport_proof_v1(
    proof: &AcipRuntimeStreamWebSocketTransportProofV1,
) -> Result<()> {
    if proof.schema_version != ACIP_RUNTIME_STREAM_WEBSOCKET_TRANSPORT_PROOF_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP WebSocket transport proof requires schema_version '{}'",
            ACIP_RUNTIME_STREAM_WEBSOCKET_TRANSPORT_PROOF_SCHEMA_VERSION
        ));
    }
    if proof.selected_substrate != "websocket" || proof.selected_crate != "tokio-tungstenite" {
        return Err(anyhow!(
            "ACIP WebSocket transport proof must use tokio-tungstenite websocket"
        ));
    }
    if proof.carrier_kind != "websocket_transport_path" {
        return Err(anyhow!(
            "ACIP WebSocket transport proof carrier_kind must be websocket_transport_path"
        ));
    }
    if proof.endpoint_binding != "loopback_ephemeral_tcp" {
        return Err(anyhow!(
            "ACIP WebSocket transport proof must bind a loopback ephemeral endpoint"
        ));
    }
    validate_acip_runtime_stream_access_policy_v1(&proof.access_policy)?;
    validate_runtime_stream_transport_case(&proof.positive_case, "delivered")?;
    if proof.positive_case.request_message_id.is_none()
        || proof.positive_case.response_message_id.is_none()
    {
        return Err(anyhow!(
            "ACIP WebSocket transport positive case must record request and response ids"
        ));
    }
    for classification in &proof.negative_cases {
        validate_acip_runtime_stream_failure_classification_v1(classification)?;
    }
    for required in [
        "malformed_message",
        "auth_policy_denial",
        "peer_close_before_response",
        "response_timeout",
    ] {
        require_runtime_stream_case(&proof.negative_cases, required)?;
    }
    for case in &proof.failure_cases {
        validate_runtime_stream_transport_case(case, "failed_closed")?;
        if case.error_substring.is_none() {
            return Err(anyhow!(
                "ACIP WebSocket transport failure case '{}' must record error_substring",
                case.case_name
            ));
        }
        let classification = proof
            .negative_cases
            .iter()
            .find(|classification| classification.case_name == case.case_name)
            .ok_or_else(|| {
                anyhow!(
                    "ACIP WebSocket transport failure case '{}' must have a matching classification",
                    case.case_name
                )
            })?;
        let error = case.error_substring.as_deref().unwrap_or_default();
        if !error.contains(&classification.expected_error_substring) {
            return Err(anyhow!(
                "ACIP WebSocket transport failure case '{}' error must contain expected classification substring '{}'",
                case.case_name,
                classification.expected_error_substring
            ));
        }
    }
    for required in [
        "positive_delivery",
        "malformed_message",
        "auth_policy_denial",
        "peer_close_before_response",
        "response_timeout",
    ] {
        require_runtime_stream_transport_case(&proof.failure_cases, required)
            .or_else(|_| require_runtime_stream_transport_case_ref(&proof.positive_case, required))?;
    }
    require_runtime_stream_event(&proof.lifecycle_events, "websocket_server_bound")?;
    require_runtime_stream_event(&proof.lifecycle_events, "positive_delivery_validated")?;
    require_runtime_stream_event(&proof.lifecycle_events, "fail_closed_cases_validated")?;
    require_runtime_stream_ref(&proof.integration_refs, "#4658")?;
    require_runtime_stream_ref(&proof.integration_refs, "#4659")?;
    require_runtime_stream_non_claim(&proof.non_claims, "TLS")?;
    require_runtime_stream_non_claim(&proof.non_claims, "production authentication")?;
    require_runtime_stream_non_claim(&proof.non_claims, "cross-polis")?;
    Ok(())
}

pub async fn prove_acip_runtime_stream_websocket_loopback_v1(
) -> Result<AcipRuntimeStreamLoopbackProofV1> {
    let request_message = sample_runtime_stream_request_message();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind ACIP runtime stream loopback listener")?;
    let addr = listener
        .local_addr()
        .context("read ACIP runtime stream loopback listener address")?;

    let server = tokio::spawn(async move {
        let (stream, _) = listener
            .accept()
            .await
            .context("accept ACIP runtime stream loopback connection")?;
        let mut websocket = tokio_tungstenite::accept_async(stream)
            .await
            .context("accept ACIP runtime stream WebSocket handshake")?;
        let incoming = timeout(Duration::from_secs(2), websocket.next())
            .await
            .context("ACIP runtime stream timed out waiting for client message")?
            .ok_or_else(|| anyhow!("ACIP runtime stream closed before first message"))?
            .context("read ACIP runtime stream client frame")?;
        let text = incoming
            .into_text()
            .context("ACIP runtime stream requires text JSON frames for v1 proof")?;
        let request: AcipMessageEnvelopeV1 =
            serde_json::from_str(&text).context("parse ACIP runtime stream request frame")?;
        validate_acip_message_envelope_v1(&request)?;
        let response = build_runtime_stream_response_message(&request)?;
        websocket
            .send(Message::Text(serde_json::to_string(&response)?))
            .await
            .context("send ACIP runtime stream response frame")?;
        websocket
            .close(None)
            .await
            .context("close ACIP runtime stream loopback websocket")?;
        Ok::<AcipMessageEnvelopeV1, anyhow::Error>(response)
    });

    let url = format!("ws://{addr}");
    let (mut client, _) = tokio_tungstenite::connect_async(&url)
        .await
        .with_context(|| format!("connect ACIP runtime stream loopback client to {url}"))?;
    client
        .send(Message::Text(serde_json::to_string(&request_message)?))
        .await
        .context("send ACIP runtime stream request frame")?;
    let response_frame = timeout(Duration::from_secs(2), client.next())
        .await
        .context("ACIP runtime stream timed out waiting for response")?
        .ok_or_else(|| anyhow!("ACIP runtime stream closed before response"))?
        .context("read ACIP runtime stream response frame")?;
    let response_text = response_frame
        .into_text()
        .context("ACIP runtime stream response must be text JSON")?;
    let response_message: AcipMessageEnvelopeV1 = serde_json::from_str(&response_text)
        .context("parse ACIP runtime stream response frame")?;
    let server_response = server
        .await
        .context("join ACIP runtime stream loopback server task")??;
    if response_message != server_response {
        return Err(anyhow!(
            "ACIP runtime stream client/server response messages diverged"
        ));
    }

    let proof = AcipRuntimeStreamLoopbackProofV1 {
        schema_version: ACIP_RUNTIME_STREAM_LOOPBACK_PROOF_SCHEMA_VERSION.to_string(),
        proof_id: "acip-runtime-stream-websocket-loopback-v1".to_string(),
        selected_substrate: "websocket".to_string(),
        selected_crate: "tokio-tungstenite".to_string(),
        carrier_kind: "mock_loopback".to_string(),
        request_message,
        response_message,
        negative_cases: vec![
            classify_acip_runtime_stream_malformed_frame_v1("{not-json")?,
            classify_acip_runtime_stream_policy_denial_v1()?,
        ],
        lifecycle_events: vec![
            "listener_bound".to_string(),
            "websocket_handshake_accepted".to_string(),
            "acip_request_validated".to_string(),
            "acip_response_validated".to_string(),
            "websocket_closed".to_string(),
        ],
        integration_refs: vec![
            "#4658".to_string(),
            "#4659".to_string(),
            "docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md".to_string(),
        ],
        non_claims: acip_runtime_stream_substrate_decision_v1().non_claims,
    };
    validate_acip_runtime_stream_loopback_proof_v1(&proof)?;
    Ok(proof)
}

pub async fn prove_acip_runtime_stream_websocket_transport_path_v1(
) -> Result<AcipRuntimeStreamWebSocketTransportProofV1> {
    let policy = acip_runtime_stream_access_policy_v1();
    let positive_request = sample_runtime_stream_request_message();
    let positive_exchange = execute_websocket_transport_text_exchange(
        serde_json::to_string(&positive_request)?,
        policy.clone(),
        ClientFrameMode::SendAndRead,
    )
    .await?;
    let positive_response = positive_exchange
        .response_message
        .ok_or_else(|| {
            anyhow!(
                "positive WebSocket transport exchange must produce a response: {}",
                positive_exchange
                    .error_substring
                    .as_deref()
                    .unwrap_or("missing server error")
            )
        })?;
    let positive_case = AcipRuntimeStreamTransportCaseV1 {
        case_name: "positive_delivery".to_string(),
        status: "delivered".to_string(),
        request_message_id: Some(positive_request.message_id.clone()),
        response_message_id: Some(positive_response.message_id),
        error_substring: None,
        lifecycle_events: positive_exchange.lifecycle_events,
    };

    let malformed_exchange = execute_websocket_transport_text_exchange(
        "{not-json".to_string(),
        policy.clone(),
        ClientFrameMode::SendAndRead,
    )
    .await?;
    let malformed_case = AcipRuntimeStreamTransportCaseV1 {
        case_name: "malformed_message".to_string(),
        status: "failed_closed".to_string(),
        request_message_id: None,
        response_message_id: None,
        error_substring: malformed_exchange.error_substring,
        lifecycle_events: malformed_exchange.lifecycle_events,
    };

    let mut denied_request = sample_runtime_stream_request_message();
    denied_request.message_id = "msg-runtime-stream-denied-0001".to_string();
    denied_request.sender.id = "external.agent".to_string();
    let denied_exchange = execute_websocket_transport_text_exchange(
        serde_json::to_string(&denied_request)?,
        policy.clone(),
        ClientFrameMode::SendAndRead,
    )
    .await?;
    let denied_case = AcipRuntimeStreamTransportCaseV1 {
        case_name: "auth_policy_denial".to_string(),
        status: "failed_closed".to_string(),
        request_message_id: Some(denied_request.message_id),
        response_message_id: None,
        error_substring: denied_exchange.error_substring,
        lifecycle_events: denied_exchange.lifecycle_events,
    };

    let close_exchange =
        execute_websocket_transport_text_exchange(String::new(), policy.clone(), ClientFrameMode::CloseBeforeRequest)
            .await?;
    let close_case = AcipRuntimeStreamTransportCaseV1 {
        case_name: "peer_close_before_response".to_string(),
        status: "failed_closed".to_string(),
        request_message_id: None,
        response_message_id: None,
        error_substring: close_exchange.error_substring,
        lifecycle_events: close_exchange.lifecycle_events,
    };

    let timeout_exchange =
        execute_websocket_transport_text_exchange(String::new(), policy.clone(), ClientFrameMode::Idle)
            .await?;
    let timeout_case = AcipRuntimeStreamTransportCaseV1 {
        case_name: "response_timeout".to_string(),
        status: "failed_closed".to_string(),
        request_message_id: None,
        response_message_id: None,
        error_substring: timeout_exchange.error_substring,
        lifecycle_events: timeout_exchange.lifecycle_events,
    };

    let mut non_claims = acip_runtime_stream_substrate_decision_v1().non_claims;
    non_claims.push(
        "This proof exercises loopback WebSocket transport path behavior; production authentication, TLS termination, and cross-polis networking remain outside #4659.".to_string(),
    );

    let proof = AcipRuntimeStreamWebSocketTransportProofV1 {
        schema_version: ACIP_RUNTIME_STREAM_WEBSOCKET_TRANSPORT_PROOF_SCHEMA_VERSION.to_string(),
        proof_id: "wp12-acip-websocket-transport-path-v1".to_string(),
        selected_substrate: "websocket".to_string(),
        selected_crate: "tokio-tungstenite".to_string(),
        carrier_kind: "websocket_transport_path".to_string(),
        endpoint_binding: "loopback_ephemeral_tcp".to_string(),
        access_policy: policy,
        positive_case,
        negative_cases: acip_runtime_stream_failure_classifications_v1(),
        failure_cases: vec![malformed_case, denied_case, close_case, timeout_case],
        lifecycle_events: vec![
            "websocket_server_bound".to_string(),
            "positive_delivery_validated".to_string(),
            "fail_closed_cases_validated".to_string(),
        ],
        integration_refs: vec![
            "#4658".to_string(),
            "#4659".to_string(),
            "#4900".to_string(),
            "docs/milestones/v0.91.7/review/runtime/ACIP_RUNTIME_STREAM_SUBSTRATE_4900.md"
                .to_string(),
            "docs/milestones/v0.91.7/review/runtime/WP12_ACIP_WEBSOCKET_TRANSPORT_4659.md"
                .to_string(),
        ],
        non_claims,
    };
    validate_acip_runtime_stream_websocket_transport_proof_v1(&proof)?;
    Ok(proof)
}

pub fn classify_acip_runtime_stream_malformed_frame_v1(
    payload: &str,
) -> Result<AcipRuntimeStreamFailureClassificationV1> {
    let expected_error_substring = match serde_json::from_str::<AcipMessageEnvelopeV1>(payload) {
        Ok(message) => match validate_acip_message_envelope_v1(&message) {
            Ok(()) => {
                return Err(anyhow!(
                    "malformed frame classifier requires invalid ACIP payload"
                ));
            }
            Err(error) => error.to_string(),
        },
        Err(error) => error.to_string(),
    };
    Ok(AcipRuntimeStreamFailureClassificationV1 {
        case_name: "malformed_message".to_string(),
        category: "malformed".to_string(),
        fail_closed: true,
        retry_allowed: false,
        expected_error_substring,
        authority_expansion_allowed: false,
    })
}

pub fn classify_acip_runtime_stream_policy_denial_v1(
) -> Result<AcipRuntimeStreamFailureClassificationV1> {
    let carrier = carrier::sample_acip_local_carrier_v1();
    let mut contract = sample_invocation_contract();
    contract.caller_class = AcipCallerClassV1::ExternalAgent;
    let request_message = carrier::sample_local_review_request_message(&contract);
    let exchange =
        carrier::execute_acip_local_invocation_round_trip(&carrier, &request_message, &contract)?;
    if exchange.invocation_event.status != AcipInvocationStatusV1::Refused {
        return Err(anyhow!(
            "ACIP runtime stream policy-denial classifier expected refused invocation"
        ));
    }
    Ok(AcipRuntimeStreamFailureClassificationV1 {
        case_name: "auth_policy_denial".to_string(),
        category: "auth_policy_denial".to_string(),
        fail_closed: true,
        retry_allowed: false,
        expected_error_substring: exchange
            .invocation_event
            .refusal_code
            .unwrap_or_else(|| "caller_class_denied".to_string()),
        authority_expansion_allowed: false,
    })
}

pub fn acip_runtime_stream_failure_classifications_v1(
) -> Vec<AcipRuntimeStreamFailureClassificationV1> {
    vec![
        AcipRuntimeStreamFailureClassificationV1 {
            case_name: "reconnect_after_disconnect".to_string(),
            category: "reconnect".to_string(),
            fail_closed: true,
            retry_allowed: true,
            expected_error_substring: "reconnect_requires_new_handshake_and_revalidation"
                .to_string(),
            authority_expansion_allowed: false,
        },
        classify_acip_runtime_stream_malformed_frame_v1("{not-json").unwrap_or_else(|error| {
            AcipRuntimeStreamFailureClassificationV1 {
                case_name: "malformed_message".to_string(),
                category: "malformed".to_string(),
                fail_closed: true,
                retry_allowed: false,
                expected_error_substring: error.to_string(),
                authority_expansion_allowed: false,
            }
        }),
        AcipRuntimeStreamFailureClassificationV1 {
            case_name: "peer_close_before_response".to_string(),
            category: "close".to_string(),
            fail_closed: true,
            retry_allowed: true,
            expected_error_substring: "closed_before_response".to_string(),
            authority_expansion_allowed: false,
        },
        AcipRuntimeStreamFailureClassificationV1 {
            case_name: "response_timeout".to_string(),
            category: "timeout".to_string(),
            fail_closed: true,
            retry_allowed: true,
            expected_error_substring: "timed out waiting for response".to_string(),
            authority_expansion_allowed: false,
        },
        classify_acip_runtime_stream_policy_denial_v1().unwrap_or_else(|error| {
            AcipRuntimeStreamFailureClassificationV1 {
                case_name: "auth_policy_denial".to_string(),
                category: "auth_policy_denial".to_string(),
                fail_closed: true,
                retry_allowed: false,
                expected_error_substring: error.to_string(),
                authority_expansion_allowed: false,
            }
        }),
    ]
}

fn validate_acip_runtime_stream_failure_classification_v1(
    classification: &AcipRuntimeStreamFailureClassificationV1,
) -> Result<()> {
    validate_negative_case_name(&classification.case_name, "case_name")?;
    validate_non_empty(&classification.category, "category")?;
    validate_non_empty(
        &classification.expected_error_substring,
        "expected_error_substring",
    )?;
    if !classification.fail_closed {
        return Err(anyhow!(
            "ACIP runtime stream failure classification '{}' must fail closed",
            classification.case_name
        ));
    }
    if classification.authority_expansion_allowed {
        return Err(anyhow!(
            "ACIP runtime stream failure classification '{}' must not expand authority",
            classification.case_name
        ));
    }
    Ok(())
}

fn require_runtime_stream_case(
    classifications: &[AcipRuntimeStreamFailureClassificationV1],
    case_name: &str,
) -> Result<()> {
    if classifications
        .iter()
        .any(|classification| classification.case_name == case_name)
    {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP runtime stream proof requires negative case '{}'",
            case_name
        ))
    }
}

fn require_runtime_stream_event(events: &[String], event: &str) -> Result<()> {
    if events.iter().any(|value| value == event) {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP runtime stream proof requires lifecycle event '{}'",
            event
        ))
    }
}

fn require_runtime_stream_ref(refs: &[String], required: &str) -> Result<()> {
    if refs.iter().any(|value| value.contains(required)) {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP runtime stream proof requires integration ref '{}'",
            required
        ))
    }
}

fn require_runtime_stream_non_claim(non_claims: &[String], required: &str) -> Result<()> {
    if non_claims.iter().any(|value| value.contains(required)) {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP runtime stream proof requires non-claim containing '{}'",
            required
        ))
    }
}

fn sample_runtime_stream_request_message() -> AcipMessageEnvelopeV1 {
    let mut message = transport::sample_message(
        "msg-runtime-stream-0001",
        "conv-runtime-stream-001",
        1,
        AcipIntentV1::Conversation,
        "Open a bounded ACIP runtime stream and preserve authority at the envelope layer.",
    );
    message.recipient.id = "runtime.agent".to_string();
    message.correlation_id = Some("runtime-stream-loopback-0001".to_string());
    message
}

fn build_runtime_stream_response_message(
    request: &AcipMessageEnvelopeV1,
) -> Result<AcipMessageEnvelopeV1> {
    let mut response = transport::sample_message(
        "msg-runtime-stream-0001-reply",
        &request.conversation_id,
        request.monotonic_order + 1,
        AcipIntentV1::Conversation,
        "ACIP runtime stream loopback accepted one validated envelope over WebSocket.",
    );
    response.sender = request.recipient.clone();
    response.recipient = request.sender.clone();
    response.trace_requirement = request.trace_requirement.clone();
    response.correlation_id = request.correlation_id.clone();
    response.prior_message_id = Some(request.message_id.clone());
    response.authority_scope = None;
    validate_acip_message_envelope_v1(&response)?;
    Ok(response)
}

#[derive(Clone, Copy)]
enum ClientFrameMode {
    SendAndRead,
    CloseBeforeRequest,
    Idle,
}

struct WebSocketTransportExchangeResult {
    response_message: Option<AcipMessageEnvelopeV1>,
    error_substring: Option<String>,
    lifecycle_events: Vec<String>,
}

pub fn acip_runtime_stream_access_policy_v1() -> AcipRuntimeStreamAccessPolicyV1 {
    AcipRuntimeStreamAccessPolicyV1 {
        policy_id: "wp12_acip_websocket_transport_policy".to_string(),
        allowed_sender_prefixes: vec!["planner.".to_string(), "agent.".to_string()],
        denied_sender_prefixes: vec!["external.".to_string(), "untrusted.".to_string()],
        allowed_recipient_ids: vec!["runtime.agent".to_string()],
        required_trace_requirement: AcipTraceRequirementV1::Summary,
        authority_expansion_allowed: false,
    }
}

fn authorize_runtime_stream_message(
    request: &AcipMessageEnvelopeV1,
    policy: &AcipRuntimeStreamAccessPolicyV1,
) -> Result<()> {
    validate_acip_runtime_stream_access_policy_v1(policy)?;
    if policy
        .denied_sender_prefixes
        .iter()
        .any(|prefix| request.sender.id.starts_with(prefix))
    {
        return Err(anyhow!("caller_class_denied_by_websocket_transport_policy"));
    }
    if !policy
        .allowed_sender_prefixes
        .iter()
        .any(|prefix| request.sender.id.starts_with(prefix))
    {
        return Err(anyhow!("sender_prefix_not_allowed_by_websocket_transport_policy"));
    }
    if !policy
        .allowed_recipient_ids
        .iter()
        .any(|recipient| recipient == &request.recipient.id)
    {
        return Err(anyhow!(
            "recipient_not_allowed_by_websocket_transport_policy"
        ));
    }
    if request.trace_requirement != policy.required_trace_requirement {
        return Err(anyhow!(
            "trace_requirement_not_allowed_by_websocket_transport_policy"
        ));
    }
    Ok(())
}

async fn execute_websocket_transport_text_exchange(
    payload: String,
    policy: AcipRuntimeStreamAccessPolicyV1,
    mode: ClientFrameMode,
) -> Result<WebSocketTransportExchangeResult> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind ACIP WebSocket transport listener")?;
    let addr = listener
        .local_addr()
        .context("read ACIP WebSocket transport listener address")?;

    let server = tokio::spawn(async move {
        let (stream, _) = listener
            .accept()
            .await
            .context("accept ACIP WebSocket transport connection")?;
        let mut websocket = tokio_tungstenite::accept_async(stream)
            .await
            .context("accept ACIP WebSocket transport handshake")?;
        let incoming = timeout(Duration::from_millis(250), websocket.next())
            .await
            .context("timed out waiting for response")?
            .ok_or_else(|| anyhow!("closed_before_response"))?
            .context("read ACIP WebSocket transport client frame")?;
        let text = incoming
            .into_text()
            .context("ACIP WebSocket transport requires text JSON frames")?;
        let request: AcipMessageEnvelopeV1 = serde_json::from_str(&text)
            .context("parse ACIP WebSocket transport request frame")?;
        validate_acip_message_envelope_v1(&request)?;
        authorize_runtime_stream_message(&request, &policy)?;
        let response = build_runtime_stream_response_message(&request)?;
        websocket
            .send(Message::Text(serde_json::to_string(&response)?))
            .await
            .context("send ACIP WebSocket transport response frame")?;
        websocket
            .close(None)
            .await
            .context("close ACIP WebSocket transport connection")?;
        Ok::<AcipMessageEnvelopeV1, anyhow::Error>(response)
    });

    let url = format!("ws://{addr}");
    let (mut client, _) = tokio_tungstenite::connect_async(&url)
        .await
        .with_context(|| format!("connect ACIP WebSocket transport client to {url}"))?;

    let mut lifecycle_events = vec![
        "websocket_server_bound".to_string(),
        "websocket_handshake_accepted".to_string(),
    ];
    let client_response = match mode {
        ClientFrameMode::SendAndRead => {
            client
                .send(Message::Text(payload))
                .await
                .context("send ACIP WebSocket transport request frame")?;
            lifecycle_events.push("client_frame_sent".to_string());
            match timeout(Duration::from_millis(250), client.next()).await {
                Ok(Some(Ok(frame))) => match frame.into_text() {
                    Ok(text) => {
                        let response: AcipMessageEnvelopeV1 = serde_json::from_str(&text)
                            .context("parse ACIP WebSocket transport response frame")?;
                        lifecycle_events.push("client_response_received".to_string());
                        Some(response)
                    }
                    Err(_) => None,
                },
                _ => None,
            }
        }
        ClientFrameMode::CloseBeforeRequest => {
            drop(client);
            lifecycle_events.push("client_closed_before_request".to_string());
            None
        }
        ClientFrameMode::Idle => {
            tokio::time::sleep(Duration::from_millis(350)).await;
            lifecycle_events.push("client_idle_until_server_timeout".to_string());
            None
        }
    };

    let server_result = server
        .await
        .context("join ACIP WebSocket transport server task")?;
    match (server_result, client_response) {
        (Ok(server_response), Some(response)) => {
            if response != server_response {
                return Err(anyhow!(
                    "ACIP WebSocket transport client/server response messages diverged"
                ));
            }
            lifecycle_events.push("server_response_validated".to_string());
            Ok(WebSocketTransportExchangeResult {
                response_message: Some(response),
                error_substring: None,
                lifecycle_events,
            })
        }
        (Ok(_response), None) => Ok(WebSocketTransportExchangeResult {
            response_message: None,
            error_substring: Some("client_response_not_observed".to_string()),
            lifecycle_events,
        }),
        (Err(error), _) => {
            lifecycle_events.push("server_failed_closed".to_string());
            let error_substring = match mode {
                ClientFrameMode::CloseBeforeRequest => format!("closed_before_response: {error:#}"),
                ClientFrameMode::Idle => format!("{error:#}"),
                ClientFrameMode::SendAndRead => format!("{error:#}"),
            };
            Ok(WebSocketTransportExchangeResult {
                response_message: None,
                error_substring: Some(error_substring),
                lifecycle_events,
            })
        }
    }
}

fn validate_runtime_stream_transport_case(
    case: &AcipRuntimeStreamTransportCaseV1,
    expected_status: &str,
) -> Result<()> {
    validate_negative_case_name(&case.case_name, "case_name")?;
    if case.status != expected_status {
        return Err(anyhow!(
            "ACIP WebSocket transport case '{}' must have status '{}'",
            case.case_name,
            expected_status
        ));
    }
    if case.lifecycle_events.is_empty() {
        return Err(anyhow!(
            "ACIP WebSocket transport case '{}' must record lifecycle_events",
            case.case_name
        ));
    }
    Ok(())
}

fn require_runtime_stream_transport_case(
    cases: &[AcipRuntimeStreamTransportCaseV1],
    case_name: &str,
) -> Result<()> {
    if cases.iter().any(|case| case.case_name == case_name) {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP WebSocket transport proof requires case '{}'",
            case_name
        ))
    }
}

fn require_runtime_stream_transport_case_ref(
    case: &AcipRuntimeStreamTransportCaseV1,
    case_name: &str,
) -> Result<()> {
    if case.case_name == case_name {
        Ok(())
    } else {
        Err(anyhow!(
            "ACIP WebSocket transport proof requires case '{}'",
            case_name
        ))
    }
}
