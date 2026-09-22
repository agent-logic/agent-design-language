//! PVF: deterministic runtime regression proof; local memory only; no new release gate.
use super::*;
use std::sync::{Arc, Mutex};

#[test]
fn response_auth_empty_action_is_inert_but_malformed_actions_stay_rejected() {
    let output = normalize_registered_conversation(
        serde_json::json!({
            "schema":"adl.runtime.provider_agent_action.v1",
            "message":"I am quill.axioma, ready for cooperation.", "action":{}
        })
        .to_string(),
    )
    .unwrap();
    assert_eq!(output.message, "I am quill.axioma, ready for cooperation.");
    assert!(output.agent_to_agent.is_none());
    for action in [
        serde_json::json!({"recipient_name":"beacon.axioma"}),
        serde_json::json!({"request_help":false}),
        serde_json::Value::Null,
    ] {
        assert!(normalize_registered_conversation(
            serde_json::json!({
                "schema":"adl.runtime.provider_agent_action.v1", "message":"hello", "action":action
            })
            .to_string()
        )
        .is_err());
    }
    for message in [
        serde_json::json!(" "),
        serde_json::json!(42),
        serde_json::json!("x".repeat(AGENT_CONVERSATION_MESSAGE_TOTAL_LIMIT_BYTES + 1)),
    ] {
        assert!(normalize_registered_conversation(
            serde_json::json!({
                "schema":"adl.runtime.provider_agent_action.v1", "message":message, "action":{}
            })
            .to_string()
        )
        .is_err());
    }
    assert!(normalize_registered_conversation(
        serde_json::json!({
            "schema":"adl.runtime.provider_agent_action.v1", "message":"hello", "action":{},
            "recipient_name":"beacon.axioma"
        })
        .to_string()
    )
    .is_err());
    let valid = normalize_registered_conversation(
        serde_json::json!({
            "schema":"adl.runtime.provider_agent_action.v1", "message":"asking Beacon",
            "action":{"recipient_name":"beacon.axioma","message":"hello","message_parts":[]}
        })
        .to_string(),
    )
    .unwrap();
    assert!(valid.agent_to_agent.is_some());
}

#[derive(Clone)]
struct AuditWriter(Arc<Mutex<Vec<u8>>>);
impl std::io::Write for AuditWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn response_auth_audit_records_both_outcomes_without_credentials() {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let writer = AuditWriter(bytes.clone());
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_writer(move || writer.clone())
        .finish();
    tracing::subscriber::with_default(subscriber, || {
        record_observatory_authentication(false);
        record_observatory_authentication(true);
    });
    let log = String::from_utf8(bytes.lock().unwrap().clone()).unwrap();
    assert_eq!(log.lines().count(), 2);
    for field in [
        "observatory_authentication",
        "authentication_failed",
        "authenticated",
        "accepted",
        "rejected",
        "/v1/observatory/ws",
    ] {
        assert!(log.contains(field), "{field}: {log}");
    }
    for forbidden in ["bearer", "token", "credential", "principal", "payload"] {
        assert!(!log.contains(forbidden));
    }
}

#[test]
fn response_auth_continuation_preserves_canonical_identity() {
    let binding: AgentAdmissionRequest = serde_json::from_value(serde_json::json!({
        "schema":"adl.runtime_v3.agent_admission.v1", "id":"qwen25-coder-32b-nessus",
        "name":"nova.axioma", "provider":"anthropic_claude_opus_5", "model":"claude-opus-5",
        "endpoint":"https://provider.invalid"
    }))
    .unwrap();
    let result = ObservatoryConversationResult::from_parts(ObservatoryConversationResultParts {
        status: "delivered",
        conversation_id: "internal-conversation".into(),
        turn_id: "internal-turn".into(),
        recipient_id: "deepseek-v4-flash-openrouter".into(),
        correlation_id: "internal-correlation".into(),
        reply: Some("Hello nova.axioma, I am delta.axioma.".into()),
        accepted_sequence: None,
        turn_sequence: None,
        error: None,
        initiation: None,
    });
    let prompt = named_agent_result_continuation_prompt(
        &binding,
        Some("delta.axioma"),
        None,
        "Greet your peers",
        &result,
    )
    .unwrap();
    assert!(prompt.contains("You are resident agent `nova.axioma`"));
    assert!(prompt.contains("\"recipient_name\": \"delta.axioma\""));
    for internal in [
        "qwen25-coder-32b-nessus",
        "deepseek-v4-flash-openrouter",
        "claude-opus-5",
        "internal-conversation",
    ] {
        assert!(!prompt.contains(internal));
    }
    let missing =
        named_agent_result_continuation_prompt(&binding, None, None, "Greet your peers", &result)
            .unwrap();
    assert!(missing.contains("\"recipient_name\": null"));
    assert!(!missing.contains("deepseek-v4-flash-openrouter"));
}

#[test]
fn response_auth_explicit_help_remains_an_explicit_action() {
    let response = normalize_registered_conversation(
        serde_json::json!({
            "schema":"adl.runtime.provider_agent_action.v1", "message":"Please help",
            "action":{"request_help":true}
        })
        .to_string(),
    )
    .unwrap();
    let help: serde_json::Value = serde_json::from_str(&response.message).unwrap();
    assert_eq!(help["request_help"], true);
    assert_eq!(help["schema"], "adl.runtime.agent_conversation_response.v1");
    assert!(response.agent_to_agent.is_none());
}
