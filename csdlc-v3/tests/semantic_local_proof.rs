//! PVF tooling / required #870 consumer proof. Isolated installed candidate,
//! deterministic Git + synthetic transport, local CPU/disk only. Fixture setup
//! establishes authority; every claimed issue transition executes through the CLI.
//! No live GitHub, active installation, or caller-authored semantic capability.
use csdlc_v3::storage::{
    semantic::{IssueKey, Observation, SemanticRoot, Snapshot},
    DurableTransactionStore,
};
use serde_json::{json, Value};
use std::{path::Path, process::Output};
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod fixture;
use fixture::Fixture;

fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "installed command failed: {output:?}"
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("structured stdout");
    assert!(value["envelope"].is_object());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
    value
}
fn plan() -> Value {
    json!({"schema":"csdlc.v3.intent_plan.v1","slug":"semantic-local-proof",
        "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Fixture dependencies ready","repo_inputs_inline":"Tracked fixture inputs","target_files_surfaces_inline":"fixture-proof","deliverables_inline":"Run the fixture proof","validation_plan_inline":"Declared Cargo validator","acceptance_criteria_inline":"Real nonzero tests pass","notes_risks_inline":"Isolated local fixture only"},"vpp":{},"srp":{},"sor":{}},
        "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok.","timeout_seconds":60}],
        "publication":{"base":"main","title":"Semantic fixture","body":"Closes #505","draft":true}})
}
fn snapshot(primary: &Path) -> Snapshot {
    let root =
        SemanticRoot::from_git_common(primary.join(".git"), "agent-logic/agent-design-language")
            .unwrap();
    let key = IssueKey::new("agent-logic/agent-design-language", 505).unwrap();
    match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(value) | Observation::ProjectionRepairRequired(value) => *value,
        other => panic!("no canonical semantic record: {other:?}"),
    }
}

#[test]
fn installed_local_proof_uses_one_history_and_replays_without_validator_effects() {
    let mut fixture = Fixture::new("semantic-history");
    let primary = fixture.root.clone();
    let input = fixture.write_json("semantic-plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    let prepared = snapshot(&primary);
    assert_eq!(prepared.phase(), csdlc_v3::lifecycle::LifecycleState::Ready);
    assert!(prepared.inputs().binding().is_none());
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = snapshot(&primary);
    assert_eq!(bound.phase(), csdlc_v3::lifecycle::LifecycleState::Bound);
    assert!(bound.version().generation() > prepared.version().generation());
    assert_ne!(bound.inputs_version(), prepared.inputs_version());
    let linked = bound.inputs().binding().unwrap().worktree.clone();
    for command in ["status", "validate"] {
        let before = fixture::inventory(&primary);
        success(fixture.run(&linked, &[command, "505"]));
        assert_eq!(
            before,
            fixture::inventory(&primary),
            "observation wrote fixture state"
        );
    }
    let before = snapshot(&primary);
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["native_effect_truth"], "performed");
    let proven = snapshot(&primary);
    assert_eq!(
        proven.phase(),
        csdlc_v3::lifecycle::LifecycleState::Implemented
    );
    assert_eq!(proven.inputs_version(), before.inputs_version());
    assert!(proven.pending().is_none());
    assert!(proven.completed().len() > before.completed().len());
    let inventory = fixture::inventory(&primary);
    let replay = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_eq!(snapshot(&primary).version(), proven.version());
    assert_eq!(
        inventory,
        fixture::inventory(&primary),
        "proof replay ran an effect"
    );
}

#[test]
fn installed_prepare_refuses_retained_native_journal_without_creating_semantic_state() {
    let mut fixture = Fixture::new("semantic-legacy-collision");
    let primary = fixture.root.clone();
    let journal = primary.join(".git/csdlc-v3/local/transactions/505.json");
    std::fs::create_dir_all(journal.parent().unwrap()).unwrap();
    std::fs::write(&journal, b"retained interrupted native journal").unwrap();
    let input = fixture.write_json("semantic-plan.json", &plan());
    let before = fixture::inventory(&primary);
    let output = fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    );
    assert!(!output.status.success());
    assert_eq!(before, fixture::inventory(&primary));
    assert!(!primary.join(".git/csdlc-v3/semantic/issues/505").exists());
}
