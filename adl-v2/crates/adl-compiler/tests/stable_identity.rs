mod common;

use adl_compiler::compile;
use adl_language::WorkflowKind;

#[test]
fn stable_ids_ignore_unrelated_document_values() {
    let original = common::document(WorkflowKind::Sequential);
    let mut changed = original.clone();
    changed.providers.get_mut("local").unwrap().profile = Some("unrelated".into());
    changed
        .run
        .inputs
        .insert("extra".into(), serde_json::json!(true));
    let left: Vec<_> = compile(&original)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    let right: Vec<_> = compile(&changed)
        .unwrap()
        .nodes
        .into_iter()
        .map(|node| node.id)
        .collect();
    assert_eq!(left, right);
}

#[test]
fn stable_ids_change_with_semantic_step_identity() {
    let original = common::document(WorkflowKind::Sequential);
    let mut changed = original.clone();
    changed.workflows.get_mut("flow").unwrap().steps[1].id = "renamed".into();
    let left = compile(&original).unwrap().nodes;
    let right = compile(&changed).unwrap().nodes;
    assert_eq!(left[0].id, right[0].id);
    assert_ne!(left[1].id, right[1].id);
}

#[test]
fn stable_identity_golden_vector_is_versioned() {
    let plan = compile(&common::document(WorkflowKind::Sequential)).unwrap();
    assert_eq!(
        plan.nodes[0].id,
        "node_v1_3e3c5d4a53bf98a70af30baa2d7e41b31c1fac90774f7b799d4df987b3a11172"
    );
}
