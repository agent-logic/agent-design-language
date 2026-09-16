//! PVF: installed integration, deterministic local Git/synthetic GitHub transport.
//! #870 terminal/cleanup semantic-owner gate; local construction proof, no live delivery claim.
use csdlc_v3::storage::{
    semantic::{IssueKey, Observation, SemanticRoot},
    DurableTransactionStore,
};
use serde_json::{json, Value};
use std::{fs, process::Output};
#[path = "support/intent_fixture.rs"]
#[allow(dead_code)]
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
        "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Fixture dependencies ready","repo_inputs_inline":"Tracked fixture inputs","target_files_surfaces_inline":"terminal cleanup","deliverables_inline":"Record terminal state and remove the checkout","validation_plan_inline":"Installed terminal recovery journey","acceptance_criteria_inline":"Exact interrupted cleanup resumes once","notes_risks_inline":"Synthetic transport and isolated repository"},"vpp":{},"srp":{},"sor":{}},
        "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],
        "publication":{"base":"main","title":"Semantic terminal fixture","body":"Closes #870","draft":true}
    }));
    success(fixture.run(
        &primary,
        &["prepare", "870", "--plan", plan.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "870"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/870.json")).unwrap(),
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
            "870",
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
            "870",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    // Use authenticated fixture repository identity from the native binding/card.
    let index: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/issues/870/index.json")).unwrap())
            .unwrap();
    let repository = index["repository"].as_str().unwrap();
    let root = SemanticRoot::from_git_common(primary.join(".git"), repository).unwrap();
    let key = IssueKey::new(repository, 870).unwrap();
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
            "870",
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
    let preview = success(fixture.run(&linked, &["clean", "870", "--preview", "plan"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "cleanup preview mutated state"
    );
    let token = preview["result"]["preview_token"]
        .as_str()
        .or_else(|| preview["preview_token"].as_str())
        .expect("cleanup token");
    let interrupted = fixture.run_with_env(
        &primary,
        &["clean", "870", "--execute", "--preview", token],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "cleanup_after_first_source_removal",
        )],
    );
    assert_eq!(interrupted.status.code(), Some(91));
    assert!(linked.exists());
    assert!(linked.join(".csdlc/issues/870/index.json").exists());
    assert!(primary
        .join(".git/csdlc-v3/local/archives")
        .read_dir()
        .unwrap()
        .any(|entry| entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("870-intent-")));
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
    let resumed = success(fixture.run(&primary, &["clean", "870"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "recovery preview mutated state"
    );
    let resumed_token = resumed["preview_token"].as_str().expect("recovery token");
    success(fixture.run(
        &primary,
        &["clean", "870", "--execute", "--preview", resumed_token],
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
    success(fixture.run(&primary, &["clean", "870"]));
    assert_eq!(
        before,
        intent_fixture::inventory(&primary),
        "completed cleanup replay wrote state"
    );
}

// PVF #1039: installed deterministic local Git fixture; required cleanup
// preview/execute parity regression; no network, provider, or paid resources.
#[test]
fn cleanup_preview_execute_accepts_stale_generated_projection() {
    let mut fixture = Fixture::new("cleanup-stale-tracked-projection");
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    let plan = fixture.write_json(
        "cleanup-projection-plan.json",
        &json!({
            "schema":"csdlc.v3.intent_plan.v1","slug":"cleanup-projection-parity",
            "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Fixture dependencies ready","repo_inputs_inline":"Tracked fixture inputs","target_files_surfaces_inline":"terminal cleanup projection admission","deliverables_inline":"Remove the exact terminal checkout","validation_plan_inline":"Installed cleanup parity journey","acceptance_criteria_inline":"The accepted preview token executes once","notes_risks_inline":"Cleanup-only projection exception"},"vpp":{},"srp":{},"sor":{}},
            "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],
            "publication":{"base":"main","title":"Cleanup projection parity","body":"Closes #870","draft":true}
        }),
    );
    success(fixture.run(
        &primary,
        &["prepare", "870", "--plan", plan.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "870"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/870.json")).unwrap(),
    )
    .unwrap();
    let linked = std::path::PathBuf::from(binding["worktree"].as_str().unwrap());

    intent_fixture::git(&linked, &["add", ".csdlc/v3/issues/870"]);
    intent_fixture::git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track generated lifecycle projections",
        ],
    );
    let tracked_projection_head = intent_fixture::git(&linked, &["rev-parse", "HEAD"]);
    intent_fixture::git(
        &primary,
        &["merge", "--quiet", "--ff-only", &tracked_projection_head],
    );
    intent_fixture::git(
        &primary,
        &[
            "update-ref",
            "refs/remotes/origin/main",
            &tracked_projection_head,
        ],
    );
    success(fixture.run(&linked, &["bind", "870"]));

    let disposition = fixture.write_json(
        "cleanup-projection-disposition.json",
        &json!({"disposition":"retired_without_execution","operator":"synthetic-fixture","rationale":"Explicit administrative retirement; no implementation delivery","evidence_refs":["fixture:cleanup-projection-parity"]}),
    );
    let mut remote = fixture.remote_issue();
    remote["state"] = json!("closed");
    remote["updated_at"] = json!("2026-09-16T12:00:00Z");
    remote["closed_at"] = json!("2026-09-16T12:00:00Z");
    fs::write(
        primary.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&remote).unwrap(),
    )
    .unwrap();
    success(fixture.run(
        &linked,
        &[
            "finish",
            "870",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));

    intent_fixture::git(&linked, &["restore", ".csdlc/v3/issues/870"]);
    assert!(
        intent_fixture::git(&linked, &["status", "--porcelain", "--untracked-files=no"]).is_empty(),
        "fixture must reproduce no tracked dirt with stale generated projections"
    );

    let repository = "agent-logic/agent-design-language";
    let root = SemanticRoot::from_git_common(primary.join(".git"), repository).unwrap();
    let key = IssueKey::new(repository, 870).unwrap();
    assert!(matches!(
        DurableTransactionStore::observe_issue(&root, &key).unwrap(),
        Observation::ProjectionRepairRequired(_)
    ));

    let preview = success(fixture.run(&primary, &["clean", "870"]));
    let token = preview["preview_token"].as_str().expect("cleanup token");
    let removed =
        success(fixture.run(&primary, &["clean", "870", "--execute", "--preview", token]));
    assert_eq!(removed["status"], "completed");
    assert!(!linked.exists());

    let after = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            snapshot
        }
        other => panic!("{other:?}"),
    };
    assert_eq!(
        after.phase(),
        csdlc_v3::lifecycle::LifecycleState::ClosedOut
    );
    assert!(after.pending().is_none());
    assert_eq!(
        after.completed().last().unwrap().truth(),
        csdlc_v3::storage::semantic::protocol::EffectTruth::Performed
    );
}
