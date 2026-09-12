//! PVF: installed integration, deterministic local Git/synthetic GitHub transport.
//! #870 terminal/cleanup semantic-owner gate; local construction proof, no live delivery claim.
use csdlc_v3::storage::{
    semantic::{IssueKey, Observation, SemanticRoot},
    DurableTransactionStore,
};
use serde_json::{json, Value};
use std::{fs, process::Output};
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use intent_fixture::Fixture;

fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "installed terminal command failed: {output:?}"
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn no_pr_terminal_and_cleanup_keep_semantic_outcome_after_checkout_removal() {
    let mut fixture = Fixture::new("semantic-terminal-cleanup");
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    let plan = fixture.write_json("semantic-plan.json", &json!({
        "schema":"csdlc.v3.intent_plan.v1","slug":"semantic-terminal-cleanup",
        "cards":{"sip":{},"stp":{},"spp":{},"vpp":{},"srp":{},"sor":{}},
        "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],
        "publication":{"base":"main","title":"Semantic terminal fixture","body":"Closes #505","draft":true}
    }));
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", plan.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/505.json")).unwrap(),
    )
    .unwrap();
    let linked = std::path::PathBuf::from(binding["worktree"].as_str().unwrap());
    let disposition=fixture.write_json("semantic-disposition.json",&json!({"disposition":"retired_without_execution","operator":"synthetic-fixture","rationale":"Explicit administrative retirement; no implementation delivery","evidence_refs":["fixture:retirement"]}));
    let mut remote = fixture.remote_issue();
    remote["state"] = json!("closed");
    remote["updated_at"] = json!("2026-09-11T12:00:00Z");
    remote["closed_at"] = json!("2026-09-11T12:00:00Z");
    fs::write(
        primary.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&remote).unwrap(),
    )
    .unwrap();
    let before = intent_fixture::inventory(&primary);
    success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
            "--preview",
            "plan",
        ],
    ));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "finish preview mutated state"
    );
    success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    // Use authenticated fixture repository identity from the native binding/card.
    let index: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/issues/505/index.json")).unwrap())
            .unwrap();
    let repository = index["repository"].as_str().unwrap();
    let root = SemanticRoot::from_git_common(primary.join(".git"), repository).unwrap();
    let key = IssueKey::new(repository, 505).unwrap();
    let snapshot = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(s) | Observation::ProjectionRepairRequired(s) => s,
        other => panic!("{other:?}"),
    };
    assert!(!snapshot.completed().is_empty());
    assert_eq!(
        snapshot.completed().last().unwrap().truth(),
        csdlc_v3::storage::semantic::protocol::EffectTruth::Performed
    );
    let replay = success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    assert_eq!(
        replay["performed_mutation"], false,
        "replay dispatched terminal writer"
    );
    assert_eq!(
        replay["semantic"]["effect_truth"], "performed",
        "replay erased historical write"
    );
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&linked, &["clean", "505", "--preview", "plan"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "cleanup preview mutated state"
    );
    let token = preview["result"]["preview_token"]
        .as_str()
        .or_else(|| preview["preview_token"].as_str())
        .expect("cleanup token");
    let interrupted = fixture.interrupt_clean_after_index_removal(
        &primary,
        &["clean", "505", "--execute", "--preview", token],
    );
    assert!(!interrupted.status.success());
    assert!(linked.exists());
    assert!(!linked.join(".csdlc/issues/505/index.json").exists());
    let pending = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(s) | Observation::ProjectionRepairRequired(s) => s,
        other => panic!("{other:?}"),
    };
    let operation = pending
        .pending()
        .expect("cleanup reservation must survive interrupted archive")
        .id()
        .clone();
    let before = intent_fixture::inventory(&primary);
    let resumed = success(fixture.run(&primary, &["clean", "505"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "recovery preview mutated state"
    );
    let resumed_token = resumed["preview_token"].as_str().expect("recovery token");
    success(fixture.run(
        &primary,
        &["clean", "505", "--execute", "--preview", resumed_token],
    ));
    assert!(!linked.exists());
    let after = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(s) | Observation::ProjectionRepairRequired(s) => s,
        other => panic!("{other:?}"),
    };
    assert!(after.pending().is_none());
    assert_eq!(
        after
            .completed()
            .iter()
            .find(|done| done.id() == &operation)
            .unwrap()
            .truth(),
        csdlc_v3::storage::semantic::protocol::EffectTruth::Performed,
        "archive/removal history was replaced by reconciliation no-op truth"
    );
    assert!(
        after.completed().iter().any(|done| done.id() == &operation),
        "recovery replaced original operation identity"
    );
    assert!(
        after.completed().len() > snapshot.completed().len(),
        "removal lost semantic outcome"
    );
    let before = intent_fixture::inventory(&primary);
    success(fixture.run(&primary, &["clean", "505"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "completed cleanup replay wrote state"
    );
}
