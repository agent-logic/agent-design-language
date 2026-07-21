mod common;

use adl_compiler::{canonical_plan_bytes, compile, PlanEdgeKind};
use adl_language::{parse_and_validate_json, parse_and_validate_yaml, WorkflowKind};

#[test]
fn repeated_compilation_is_byte_identical() {
    let document = common::document(WorkflowKind::Concurrent);
    let expected = canonical_plan_bytes(&compile(&document).unwrap()).unwrap();
    for _ in 0..100 {
        assert_eq!(
            canonical_plan_bytes(&compile(&document).unwrap()).unwrap(),
            expected
        );
    }
}

#[test]
fn json_and_yaml_key_permutations_compile_identically() {
    let json = serde_json::to_string(&common::document(WorkflowKind::Sequential)).unwrap();
    let yaml = r#"
run: {workflow_ref: flow, name: example, id: run-1, inputs: {request: hello}}
workflows:
  flow:
    steps:
      - {save_as: result, inputs: {value: {a: 1, b: 2}}, task: produce, id: first}
      - {inputs: {source: "@state:result"}, id: second, task: consume}
    kind: sequential
tasks:
  consume: {tool_allowlist: [alpha], inputs: [source], agent_ref: worker, prompt: {user: consume}}
  produce: {agent_ref: worker, prompt: {user: produce}}
agents: {worker: {tools: [zeta, alpha], provider: local}}
tools: {zeta: {type: test}, alpha: {type: test}}
providers: {local: {default_model: model-a, type: test}}
version: "0.5"
"#;
    let from_json = compile(&parse_and_validate_json(&json).unwrap()).unwrap();
    let from_yaml = compile(&parse_and_validate_yaml(yaml).unwrap()).unwrap();
    assert_eq!(
        canonical_plan_bytes(&from_json).unwrap(),
        canonical_plan_bytes(&from_yaml).unwrap()
    );
}

#[test]
fn concurrent_workflow_only_orders_explicit_state_dependencies() {
    let plan = compile(&common::document(WorkflowKind::Concurrent)).unwrap();
    assert_eq!(plan.edges.len(), 1);
    assert_eq!(plan.edges[0].kind, PlanEdgeKind::StateDependency);
    assert_eq!(plan.edges[0].state.as_deref(), Some("result"));
    assert_eq!(plan.nodes[1].tools, vec!["alpha"]);
    assert_eq!(plan.nodes[0].model.as_deref(), Some("model-a"));
    assert_eq!(plan.nodes[1].ports.inputs, vec!["source"]);
    assert!(plan.nodes[1].ports.outputs.is_empty());
    assert_eq!(plan.nodes[1].prompt.user, "consume");
    assert_eq!(
        plan.nodes[1].provenance.semantic_path,
        "$.run.workflow.steps.second"
    );
}

#[test]
fn sequential_and_state_edges_are_both_explicit() {
    let plan = compile(&common::document(WorkflowKind::Sequential)).unwrap();
    assert_eq!(plan.edges.len(), 2);
    assert!(plan
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::Sequential));
    assert!(plan
        .edges
        .iter()
        .any(|edge| edge.kind == PlanEdgeKind::StateDependency));
}
