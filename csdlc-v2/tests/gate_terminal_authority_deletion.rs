use std::fs;
use std::path::Path;

use csdlc_v2::LifecyclePhase;

fn repo() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
}

#[test]
fn competing_closeout_binary_and_skill_are_absent() {
    let root = repo();
    assert!(!root.join("csdlc-v2/src/bin/csdlc-closeout.rs").exists());
    assert!(!root
        .join("csdlc-v2/operator/skills/csdlc-v2-closeout/SKILL.md")
        .exists());

    let cargo = fs::read_to_string(root.join("csdlc-v2/Cargo.toml")).unwrap();
    let skills = fs::read_to_string(root.join("csdlc-v2/operator/skills.json")).unwrap();
    let coexistence = fs::read_to_string(root.join("csdlc-v2/operator/coexistence.json")).unwrap();
    for surface in [cargo, skills, coexistence] {
        assert!(!surface.contains("csdlc-closeout"));
    }
}

#[test]
fn publication_and_store_expose_no_terminal_mutation_route() {
    let root = repo();
    let publish = fs::read_to_string(root.join("csdlc-v2/src/bin/csdlc-publish.rs")).unwrap();
    let store = fs::read_to_string(root.join("csdlc-v2/src/store.rs")).unwrap();
    let model = fs::read_to_string(root.join("csdlc-v2/src/model.rs")).unwrap();

    for removed in [
        "ReconcileMerged",
        "ReconcileReady",
        "record_readiness",
        "commit_terminal",
        "retain_terminal_receipt",
        "reconcile_terminal(",
        "repair_terminal_",
        "TerminalReceiptTransportRequest",
        "ReconcileTerminalRequest",
    ] {
        assert!(
            !publish.contains(removed) && !store.contains(removed) && !model.contains(removed),
            "removed terminal writer remains reachable: {removed}"
        );
    }
}

#[test]
fn historical_phase_and_receipt_shapes_remain_readable_only() {
    for (encoded, expected) in [
        ("\"merge_ready\"", LifecyclePhase::MergeReady),
        ("\"merged\"", LifecyclePhase::Merged),
        ("\"closed_out\"", LifecyclePhase::ClosedOut),
    ] {
        assert_eq!(
            serde_json::from_str::<LifecyclePhase>(encoded).unwrap(),
            expected
        );
    }

    assert!(!LifecyclePhase::Published.allows(LifecyclePhase::MergeReady));
    assert!(!LifecyclePhase::MergeReady.allows(LifecyclePhase::Merged));
    assert!(!LifecyclePhase::Merged.allows(LifecyclePhase::ClosedOut));

    let schemas = csdlc_v2::public_schema_bundle();
    assert!(schemas.get("terminal_receipt").is_some());
    assert!(schemas.get("finish_request").is_some());
    assert!(schemas.get("derived_terminal_envelope").is_some());
    assert!(schemas.get("terminal_reconciliation_request").is_none());
    assert!(schemas.get("terminal_receipt_transport_request").is_none());
}
