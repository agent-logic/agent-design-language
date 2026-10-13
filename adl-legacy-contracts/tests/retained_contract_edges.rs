//! PVF: public-contract regression; deterministic, small CPU/memory, no I/O.
//! Release proof role: portable validation/refusal and schema diagnostics.
//! Release gate: required public source validation; packaged-consumer proof is separate.
use adl_legacy_contracts::{adl::AdlDoc, schema, validate_run_id_path_segment};
use serde_json::{json, Value};

fn document() -> Value {
    json!({"version":"0.5", "providers":{}, "tools":{}, "agents":{},
        "tasks":{}, "workflows":{},
        "run":{"workflow":{"steps":[{"id":"one"}]}}})
}

fn validate(value: Value) -> anyhow::Result<()> {
    serde_json::from_value::<AdlDoc>(value)?.validate()
}

fn refuses(value: Value, expected: &str) {
    let error = validate(value)
        .expect_err("invalid declaration must refuse")
        .to_string();
    assert!(
        error.contains(expected),
        "expected {expected:?}, got {error:?}"
    );
}

#[test]
fn provider_declarations_preserve_profiles_and_transport_boundaries() {
    for provider in [
        json!({"type":"mock"}),
        json!({"profile":"local-review"}),
        json!({"type":"ollama","config":{"endpoint":"http://localhost:11434"}}),
        json!({"type":"http_remote","config":{"endpoint":"https://example.invalid"}}),
        json!({"type":"http","base_url":"http://[::1]:8080"}),
    ] {
        let mut doc = document();
        doc["providers"]["p"] = provider;
        validate(doc).unwrap();
    }
    for (provider, error) in [
        (json!({"profile":" "}), "profile must not be empty"),
        (
            json!({"profile":"local-review","type":"mock"}),
            "uses profile and explicit",
        ),
        (
            json!({"type":"ollama","base_url":"file:///provider"}),
            "requires an http://",
        ),
        (
            json!({"type":"local_ollama","config":{"endpoint":"http://localhost"}}),
            "CLI-backed",
        ),
        (
            json!({"type":"http_remote"}),
            "requires base_url or config.endpoint",
        ),
        (
            json!({"type":"http","base_url":"http://example.invalid"}),
            "plaintext http://",
        ),
        (json!({"type":"unknown"}), "unsupported kind"),
    ] {
        let mut doc = document();
        doc["providers"]["p"] = provider;
        refuses(doc, error);
    }
}

#[test]
fn references_and_explicit_ids_must_match_their_declared_targets() {
    for (section, entry, expected) in [
        (
            "providers",
            json!({"type":"mock","id":"other"}),
            "providers.item.id must match",
        ),
        (
            "tools",
            json!({"type":"builtin","id":"other"}),
            "tools.item.id must match",
        ),
        (
            "agents",
            json!({"provider":"p","model":"m","id":"other"}),
            "agents.item.id must match",
        ),
        (
            "tasks",
            json!({"prompt":{},"id":"other"}),
            "tasks.item.id must match",
        ),
        (
            "workflows",
            json!({"steps":[],"id":"other"}),
            "workflows.item.id must match",
        ),
        (
            "agents",
            json!({"provider":"p","model":"m","tools":["missing"]}),
            "references unknown tool",
        ),
        (
            "tasks",
            json!({"prompt":{},"agent_ref":"missing"}),
            "agent_ref references unknown agent",
        ),
        (
            "tasks",
            json!({"prompt":{},"tool_allowlist":["missing"]}),
            "tool_allowlist references unknown tool",
        ),
        ("tools", json!({"type":"unknown"}), "unsupported kind"),
    ] {
        let mut doc = document();
        doc[section]["item"] = entry;
        refuses(doc, expected);
    }
    let mut doc = document();
    doc["providers"]["p"] = json!({"type":"mock"});
    doc["agents"]["a"] = json!({"provider":"missing","model":"m"});
    refuses(doc, "references unknown provider");
}

#[test]
fn concurrency_and_artifact_paths_refuse_invalid_bounds() {
    let mut doc = document();
    doc["run"]["defaults"] = json!({"max_concurrency":0});
    refuses(doc, "run.defaults.max_concurrency must be >= 1");
    let mut doc = document();
    doc["run"]["workflow"]["max_concurrency"] = json!(0);
    refuses(doc, "run.workflow.max_concurrency must be >= 1");
    let mut doc = document();
    doc["workflows"]["stored"] = json!({"steps":[],"max_concurrency":0});
    refuses(doc, "workflows.stored.max_concurrency must be >= 1");
    for (step, expected) in [
        (json!({"id":"p::reserved"}), "reserved compiler prefix"),
        (json!({"write_to":"out.txt"}), "missing save_as"),
        (json!({"write_to":" ","save_as":"out"}), "empty write_to"),
        (
            json!({"write_to":"../escape","save_as":"out"}),
            "relative path without '..'",
        ),
        (
            json!({"write_to":"/absolute","save_as":"out"}),
            "relative path without '..'",
        ),
        (
            json!({"retry":{"max_attempts":0}}),
            "invalid retry.max_attempts=0",
        ),
    ] {
        let mut doc = document();
        doc["run"]["workflow"]["steps"] = json!([step]);
        refuses(doc, expected);
    }
    let mut doc = document();
    doc["run"]["workflow"]["steps"] =
        json!([{"save_as":"out","write_to":"reports/out.txt","retry":{"max_attempts":1}}]);
    validate(doc).unwrap();
}

#[test]
fn remote_verification_does_not_accept_unsigned_key_id_requirements() {
    for (remote, expected) in [
        (json!({"endpoint":" "}), "endpoint must not be empty"),
        (
            json!({"endpoint":"https://example.invalid","require_key_id":true,"require_signed_requests":false}),
            "requires run.remote.require_signed_requests=true",
        ),
        (
            json!({"endpoint":"https://example.invalid","verify_allowed_key_sources":["remote"]}),
            "unsupported source",
        ),
    ] {
        let mut doc = document();
        doc["run"]["remote"] = remote;
        refuses(doc, expected);
    }
    let mut doc = document();
    doc["run"]["remote"] = json!({"endpoint":"https://example.invalid","require_key_id":true,
        "require_signed_requests":true,"verify_allowed_key_sources":[" EMBEDDED ","explicit_key"]});
    validate(doc).unwrap();
}

#[test]
fn conversation_turns_require_identity_order_and_unique_ids() {
    for (turn, expected) in [
        (
            json!({"id":" ","speaker":"agent"}),
            "conversation.id must not be empty",
        ),
        (
            json!({"id":"turn","speaker":" "}),
            "conversation.speaker must not be empty",
        ),
        (
            json!({"id":"turn","speaker":"agent","sequence":0}),
            "conversation.sequence must be >= 1",
        ),
        (
            json!({"id":"turn","speaker":"agent","responds_to":"turn"}),
            "must not reference the same turn id",
        ),
    ] {
        let mut doc = document();
        doc["run"]["workflow"]["steps"][0]["conversation"] = turn;
        refuses(doc, expected);
    }
    let turn = json!({"conversation":{"id":"turn","speaker":"agent","sequence":1}});
    let mut doc = document();
    doc["run"]["workflow"]["steps"] = json!([turn.clone(), turn]);
    refuses(doc, "duplicate conversation turn id");
    let mut doc = document();
    doc["run"]["workflow"]["steps"] = json!([
        {"conversation":{"id":"first","speaker":"agent","sequence":1}},
        {"conversation":{"id":"second","speaker":"reviewer","sequence":2,"responds_to":"first"}}]);
    validate(doc).unwrap();
}

#[test]
fn artifact_run_ids_cannot_escape_their_path_segment() {
    assert_eq!(
        validate_run_id_path_segment("  run-42  ").unwrap(),
        "run-42"
    );
    for id in ["", " ", ".", "..", "a/b", "a\\b", "C:run"] {
        assert!(validate_run_id_path_segment(id).is_err(), "{id}");
    }
}

#[test]
fn schema_errors_reject_unknown_root_keys_and_remain_bounded() {
    let valid = serde_yaml::to_string(&document()).unwrap();
    schema::validate_adl_yaml(&valid).unwrap();
    assert!(schema::compiled_schema_loose().is_valid(&document()));
    assert!(schema::generated_schema_json()["properties"]["run"].is_object());
    assert!(schema::committed_schema_json().is_object());
    for yaml in [
        "run: {}",
        "version: '0.5'",
        "version: '0.5'\nrun: {}\nworkflw: {}",
    ] {
        let error = schema::validate_adl_yaml(yaml).unwrap_err().to_string();
        assert!(error.contains("ADL schema validation failed"), "{error}");
        assert!(error.contains("at /"), "{error}");
    }
    let error = schema::validate_adl_yaml("run: [").unwrap_err().to_string();
    assert!(error.contains("parse yaml into Value"), "{error}");
    let mut many = document();
    for n in 0..15 {
        many["providers"][format!("invalid_{n}")] = json!(false);
    }
    let error = schema::validate_adl_yaml(&serde_yaml::to_string(&many).unwrap())
        .unwrap_err()
        .to_string();
    assert!(
        error.lines().count() <= 12,
        "unbounded schema diagnostics: {error}"
    );
    assert!(error.contains("more schema errors omitted"), "{error}");
}
