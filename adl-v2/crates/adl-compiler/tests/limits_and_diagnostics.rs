mod common;

use adl_compiler::{compile, compile_with_limits, CompilerDiagnosticCode, CompilerLimits};
use adl_language::{Workflow, WorkflowKind, WorkflowStep};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[test]
fn node_limit_fails_closed_with_stable_diagnostic() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_nodes: 1,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert_eq!(error[0].path, "$.run.workflow");
}

#[test]
fn edge_limit_is_enforced_during_construction() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_edges: 1,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert!(error[0].message.contains("edge limit 1"));
}

#[test]
fn input_value_limit_is_aggregate_across_run_and_steps() {
    let error = compile_with_limits(
        &common::document(WorkflowKind::Sequential),
        CompilerLimits {
            max_input_values: 4,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
}

#[test]
fn deep_typed_input_is_rejected_before_semantic_validation() {
    let mut document = common::document(WorkflowKind::Sequential);
    let mut value = json!("@state:missing");
    for _ in 0..32 {
        value = Value::Array(vec![value]);
    }
    document.workflows.get_mut("flow").unwrap().steps[0]
        .inputs
        .insert("deep".into(), value);

    let error = compile_with_limits(
        &document,
        CompilerLimits {
            max_input_depth: 8,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();

    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert!(error[0].path.contains("steps[0].inputs"));
}

#[test]
fn wide_typed_input_is_rejected_before_copying_child_fanout() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.run.inputs.insert(
        "wide".into(),
        Value::Array((0..1_000).map(|index| json!(index)).collect()),
    );

    let error = compile_with_limits(
        &document,
        CompilerLimits {
            max_input_values: 2,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();

    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert_eq!(error[0].path, "$.run.inputs");
}

#[test]
fn limits_cover_unselected_workflow_declarations() {
    let mut document = common::document(WorkflowKind::Sequential);
    let mut value = json!("leaf");
    for _ in 0..16 {
        value = Value::Object(serde_json::Map::from_iter([("next".into(), value)]));
    }
    let mut unselected = document.workflows["flow"].clone();
    unselected.steps[0].inputs.insert("deep".into(), value);
    document.workflows.insert("unselected".into(), unselected);

    let error = compile_with_limits(
        &document,
        CompilerLimits {
            max_input_depth: 8,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();

    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert!(error[0].path.starts_with("$.workflows.unselected"));
}

#[test]
fn dependency_chain_over_node_limit_fails_before_validation() {
    let mut document = common::document(WorkflowKind::Concurrent);
    document
        .workflows
        .insert("flow".into(), dependency_chain(128));

    let error = compile_with_limits(
        &document,
        CompilerLimits {
            max_nodes: 16,
            ..CompilerLimits::default()
        },
    )
    .unwrap_err();

    assert_eq!(error[0].code, CompilerDiagnosticCode::LimitExceeded);
    assert_eq!(error[0].path, "$.run.workflow");
}

#[test]
fn dependency_chain_within_default_limits_validates_without_recursion() {
    let mut document = common::document(WorkflowKind::Concurrent);
    document
        .workflows
        .insert("flow".into(), dependency_chain(2_048));

    let plan = compile(&document).unwrap();

    assert_eq!(plan.nodes.len(), 2_048);
    assert_eq!(plan.edges.len(), 2_047);
}

fn dependency_chain(length: usize) -> Workflow {
    let steps = (0..length)
        .map(|index| WorkflowStep {
            id: format!("step-{index}"),
            agent: None,
            task: "produce".into(),
            inputs: if index == 0 {
                BTreeMap::new()
            } else {
                BTreeMap::from([(
                    "source".into(),
                    json!(format!("@state:state-{}", index - 1)),
                )])
            },
            save_as: Some(format!("state-{index}")),
        })
        .collect();
    Workflow {
        id: None,
        kind: WorkflowKind::Concurrent,
        steps,
    }
}

#[test]
fn invalid_language_diagnostics_are_sorted_and_preserved() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.agents.get_mut("worker").unwrap().provider = "missing".into();
    document.workflows.get_mut("flow").unwrap().steps[0].task = "missing".into();
    let errors = compile(&document).unwrap_err();
    assert!(errors.windows(2).all(|pair| pair[0] <= pair[1]));
    assert!(errors
        .iter()
        .all(|error| error.code == CompilerDiagnosticCode::InvalidDocument));
}

#[test]
fn missing_effective_agent_fails_closed() {
    let mut document = common::document(WorkflowKind::Sequential);
    document.tasks.get_mut("produce").unwrap().agent_ref = None;
    let error = compile(&document).unwrap_err();
    assert_eq!(error[0].code, CompilerDiagnosticCode::InvalidDocument);
    assert!(error[0].message.contains("do not resolve an agent"));
}
