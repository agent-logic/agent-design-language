//! PVF: public-contract regression; deterministic, small CPU/memory, no network.
//! One test captures stdout from one child of this same test executable; no file writes.
//! Release proof role: portable validation/refusal and schema diagnostics.
//! Release gate: required public source validation; packaged-consumer proof is separate.
use adl_legacy_contracts::yaml as serde_yaml;
use adl_legacy_contracts::{adl::AdlDoc, prompt, resolve, schema, validate_run_id_path_segment};
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

#[test]
fn structured_prompt_preserves_role_order_unicode_and_unknown_placeholders() {
    let spec = serde_json::from_value(json!({
        "system":"Policy {{name}}", "developer":"Keep {{unknown}} literal",
        "user":"Résumé {{name}}", "context":"Context {{name}}", "output":"JSON only"
    }))
    .unwrap();
    let inputs = [("name".to_string(), "Ada".to_string())]
        .into_iter()
        .collect();
    let rendered = prompt::trace_prompt_assembly(&spec, &inputs);
    assert_eq!(rendered,
        "SYSTEM:\nPolicy Ada\n\nDEVELOPER:\nKeep {{unknown}} literal\n\nUSER:\nRésumé Ada\n\nCONTEXT:\nContext Ada\n\nOUTPUT:\nJSON only");
    assert_eq!(
        prompt::trace_prompt_assembly(&Default::default(), &inputs),
        ""
    );
    assert_eq!(
        prompt::hash_prompt(&rendered),
        prompt::hash_prompt(&rendered)
    );
    assert_ne!(
        prompt::hash_prompt(&rendered),
        prompt::hash_prompt("different role content")
    );
}

#[test]
fn portable_resolution_preserves_prompt_precedence_and_provider_ambiguity() {
    let mut value = document();
    value["providers"] = json!({"p":{"type":"mock"},"q":{"type":"mock"}});
    value["agents"] = json!({"a":{"provider":"p","model":"m","prompt":{"user":"agent"}}});
    value["tasks"] = json!({"task":{"agent_ref":"a","prompt":{"user":"task"}}});
    value["run"]["defaults"] = json!({"system":"default policy"});
    value["run"]["workflow"]["steps"] = json!([
        {"id":"override","task":"task","prompt":{"system":"step policy","user":"step"}},
        {"id":"task","task":"task"}, {"id":"agent","agent":"a"}, {"id":"unbound"}
    ]);
    let doc: AdlDoc = serde_json::from_value(value).unwrap();
    doc.validate().unwrap();
    let resolved = resolve::resolve_run(&doc).unwrap();
    for (index, expected_user, expected_system) in [
        (0, "step", "step policy"),
        (1, "task", "default policy"),
        (2, "agent", "default policy"),
    ] {
        let step = &resolved.steps[index];
        assert_eq!(step.provider.as_deref(), Some("p"));
        assert_eq!(step.agent.as_deref(), Some("a"));
        let effective = step.effective_prompt_with_defaults(&resolved).unwrap();
        assert_eq!(effective.user.as_deref(), Some(expected_user));
        assert_eq!(effective.system.as_deref(), Some(expected_system));
    }
    assert!(
        resolved.steps[3].provider.is_none(),
        "two providers must not be chosen arbitrarily"
    );
    assert!(resolved.steps[3]
        .effective_prompt_with_defaults(&resolved)
        .is_none());
}

#[test]
fn portable_resolution_rejects_unmaterialized_authority_and_unsafe_run_names() {
    for version in ["0.1", "0.2", "0.3", "0.4", "0.5"] {
        let mut value = document();
        value["version"] = json!(version);
        let doc: AdlDoc = serde_json::from_value(value).unwrap();
        assert!(resolve::resolve_run(&doc).is_ok(), "{version}");
    }
    for (version, expected) in [
        ("", "missing required field: version"),
        ("9.0", "unsupported ADL version"),
    ] {
        let mut value = document();
        value["version"] = json!(version);
        let doc: AdlDoc = serde_json::from_value(value).unwrap();
        assert!(resolve::resolve_run(&doc)
            .unwrap_err()
            .to_string()
            .contains(expected));
    }
    let mut value = document();
    value["providers"]["p"] = json!({"profile":"configured-on-runtime"});
    let doc: AdlDoc = serde_json::from_value(value).unwrap();
    assert!(resolve::resolve_run(&doc)
        .unwrap_err()
        .to_string()
        .contains("Runtime expansion"));
    let mut value = document();
    value["run"]["name"] = json!("../escape");
    let doc: AdlDoc = serde_json::from_value(value).unwrap();
    assert!(resolve::resolve_run(&doc)
        .unwrap_err()
        .to_string()
        .contains("safe artifact path segment"));
}

#[test]
fn plan_and_prompt_stdout_preserve_order_and_missing_prompt_diagnostics() {
    const CHILD: &str = "ADL_RETAINED_CONTRACT_STDOUT_CHILD";
    if std::env::var(CHILD).as_deref() == Ok("capture") {
        let mut value = document();
        value["run"]["name"] = json!("review-run");
        value["run"]["defaults"] = json!({"system":"base policy"});
        value["run"]["workflow"] = json!({"id":"review-flow","steps":[
            {"id":"first","prompt":{"user":"Hello {{name}}"},"inputs":{"name":"Ada"}},
            {"id":"second"}, {"id":"nested","call":"child","as":"ns"}
        ]});
        let doc: AdlDoc = serde_json::from_value(value).unwrap();
        let resolved = resolve::resolve_run(&doc).unwrap();
        println!("BEGIN_RETAINED_OUTPUT");
        resolve::print_resolved_plan(&resolved);
        prompt::print_prompts(&resolved).unwrap();
        println!("END_RETAINED_OUTPUT");
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "plan_and_prompt_stdout_preserve_order_and_missing_prompt_diagnostics",
            "--nocapture",
        ])
        .env(CHILD, "capture")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let text = stdout
        .split_once("BEGIN_RETAINED_OUTPUT\n")
        .unwrap()
        .1
        .split_once("END_RETAINED_OUTPUT")
        .unwrap()
        .0;
    assert!(
        text.starts_with("Resolved run: review-run\nWorkflow:     review-flow\nSteps:        3\n"),
        "{text}"
    );
    for line in [
        "  0. first  agent=<unresolved-agent> provider=<unresolved-provider> task=<unresolved-task>\n",
        "  1. second  agent=<unresolved-agent> provider=<unresolved-provider> task=<unresolved-task>\n",
        "  2. nested  call=child as=ns\n",
        "--- step 0: first ---\nSYSTEM:\nbase policy\n\nUSER:\nHello Ada\n\n",
        "Step 1 (second) has no prompt.\n", "Step 2 (nested) has no prompt.\n"
    ] { assert!(text.contains(line), "missing {line:?} in {text:?}"); }
    assert!(text.find("  0. first").unwrap() < text.find("  1. second").unwrap());
    assert!(text.find("  1. second").unwrap() < text.find("  2. nested").unwrap());
}
