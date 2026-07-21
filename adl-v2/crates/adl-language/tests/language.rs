use adl_language::{
    canonical_bytes, json_schema, parse_and_validate_json, parse_and_validate_yaml, DiagnosticCode,
};
use std::{fs, path::PathBuf};

const COMPLETE: &str = r#"
version: "0.5"
providers:
  local: {id: local, type: local_ollama, config: {model: test}}
tools:
  docs: {id: docs, type: mcp, config: {server: docs}}
agents:
  planner: {id: planner, provider: local, model: test, tools: [docs]}
tasks:
  summarize:
    id: summarize
    agent_ref: planner
    inputs: [topic]
    tool_allowlist: [docs]
    prompt: {system: stable, user: "Summarize {{topic}}"}
workflows:
  main:
    id: main
    kind: sequential
    steps: [{id: summarize.topic, task: summarize, inputs: {topic: test}, save_as: summary}]
run: {id: run-main, name: complete, workflow_ref: main, inputs: {topic: test}, placement: {target: local}}
"#;

#[test]
fn parses_and_validates_all_six_primitives() {
    let document = parse_and_validate_yaml(COMPLETE).expect("valid complete document");
    assert_eq!(document.providers.len(), 1);
    assert_eq!(document.tools.len(), 1);
    assert_eq!(document.agents.len(), 1);
    assert_eq!(document.tasks.len(), 1);
    assert_eq!(document.workflows.len(), 1);
    assert_eq!(document.run.name, "complete");
}

#[test]
fn yaml_json_and_map_order_canonicalize_identically() {
    let yaml = parse_and_validate_yaml(COMPLETE).unwrap();
    let json = serde_json::to_string(&yaml).unwrap();
    let reparsed = parse_and_validate_json(&json).unwrap();
    assert_eq!(
        canonical_bytes(&yaml).unwrap(),
        canonical_bytes(&reparsed).unwrap()
    );

    let reordered = COMPLETE.replace(
        "providers:\n  local: {id: local, type: local_ollama, config: {model: test}}\ntools:\n  docs: {id: docs, type: mcp, config: {server: docs}}",
        "tools:\n  docs: {id: docs, type: mcp, config: {server: docs}}\nproviders:\n  local: {id: local, type: local_ollama, config: {model: test}}",
    );
    let reordered = parse_and_validate_yaml(&reordered).unwrap();
    assert_eq!(
        canonical_bytes(&yaml).unwrap(),
        canonical_bytes(&reordered).unwrap()
    );
}

#[test]
fn preserves_declared_step_order() {
    let a = COMPLETE.replace(
        "steps: [{id: summarize.topic, task: summarize, inputs: {topic: test}, save_as: summary}]",
        "steps: [{id: first, task: summarize}, {id: second, task: summarize}]",
    );
    let b = COMPLETE.replace(
        "steps: [{id: summarize.topic, task: summarize, inputs: {topic: test}, save_as: summary}]",
        "steps: [{id: second, task: summarize}, {id: first, task: summarize}]",
    );
    assert_ne!(
        canonical_bytes(&parse_and_validate_yaml(&a).unwrap()).unwrap(),
        canonical_bytes(&parse_and_validate_yaml(&b).unwrap()).unwrap()
    );
}

#[test]
fn rejects_unknown_and_duplicate_fields() {
    let unknown = COMPLETE.replace("version: \"0.5\"", "version: \"0.5\"\nunknown: true");
    assert_eq!(
        parse_and_validate_yaml(&unknown).unwrap_err()[0].code,
        DiagnosticCode::UnknownField
    );
    let duplicate = COMPLETE.replace("version: \"0.5\"", "version: \"0.5\"\nversion: \"0.5\"");
    assert_eq!(
        parse_and_validate_yaml(&duplicate).unwrap_err()[0].code,
        DiagnosticCode::DuplicateKey
    );
    let duplicate_json = r#"{"version":"0.5","version":"0.5","run":{"name":"x","workflow":{"kind":"sequential","steps":[]}}}"#;
    assert_eq!(
        parse_and_validate_json(duplicate_json).unwrap_err()[0].code,
        DiagnosticCode::DuplicateKey
    );
}

#[test]
fn rejects_bad_version_references_and_cycles() {
    let bad_version = COMPLETE.replace("version: \"0.5\"", "version: \"9\"");
    assert!(parse_and_validate_yaml(&bad_version)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::InvalidVersion));
    let bad_provider = COMPLETE.replace("provider: local", "provider: missing");
    assert!(parse_and_validate_yaml(&bad_provider)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::UnknownProvider));
    let bad_tool = COMPLETE.replace("tools: [docs]", "tools: [missing]");
    assert!(parse_and_validate_yaml(&bad_tool)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::UnknownTool));
    let bad_task = COMPLETE.replace("task: summarize", "task: missing");
    assert!(parse_and_validate_yaml(&bad_task)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::UnknownTask));
    let bad_workflow = COMPLETE.replace("workflow_ref: main", "workflow_ref: missing");
    assert!(parse_and_validate_yaml(&bad_workflow)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::UnknownWorkflow));

    let cycle = COMPLETE.replace(
        "steps: [{id: summarize.topic, task: summarize, inputs: {topic: test}, save_as: summary}]",
        "steps: [{id: a, task: summarize, save_as: a_out, inputs: {peer: '@state:b_out'}}, {id: b, task: summarize, save_as: b_out, inputs: {peer: '@state:a_out'}}]",
    );
    assert!(parse_and_validate_yaml(&cycle)
        .unwrap_err()
        .iter()
        .any(|error| error.code == DiagnosticCode::DependencyCycle));
}

#[test]
fn generated_schema_matches_parser_contract() {
    let schema = json_schema();
    let checked_schema = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schema/adl-document.schema.json"),
    )
    .expect("checked generated schema");
    let checked_schema: serde_json::Value =
        serde_json::from_str(&checked_schema).expect("valid checked schema");
    assert_eq!(schema, checked_schema, "regenerate the checked schema");
    let validator = jsonschema::validator_for(&schema).expect("valid generated schema");
    let document = parse_and_validate_yaml(COMPLETE).unwrap();
    let value = serde_json::to_value(document).unwrap();
    assert!(validator.is_valid(&value));
    let mut invalid = value;
    invalid
        .as_object_mut()
        .unwrap()
        .insert("unknown".into(), true.into());
    assert!(!validator.is_valid(&invalid));
}
