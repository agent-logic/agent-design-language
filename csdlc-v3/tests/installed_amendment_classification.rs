#![allow(dead_code)]

#[path = "support/intent_fixture.rs"]
mod intent_fixture;

use intent_fixture::Fixture;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn result(output: &std::process::Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("installed command must emit JSON")
}

fn state_inventory(primary: &Path, linked: &Path) -> Vec<(PathBuf, BTreeMap<PathBuf, String>)> {
    [
        primary.join(".git/csdlc-v3/semantic/issues/505"),
        primary.join(".git/csdlc-v3/local/transactions"),
        linked.join(".csdlc/v3/issues/505"),
        linked.join(".csdlc/issues/505"),
        linked.join(".csdlc/transactions"),
    ]
    .into_iter()
    .map(|path| {
        let bytes = intent_fixture::inventory(&path);
        (path, bytes)
    })
    .collect()
}

#[test]
fn installed_edit_rejects_false_display_only_and_mismatched_classes_without_mutation() {
    let mut fixture = Fixture::new("amendment-classification");
    let primary = fixture.root.clone();
    let plan = fixture.write_json(
        "plan.json",
        &json!({
            "schema": "csdlc.v3.intent_plan.v1",
            "slug": "installed-intent-fixture",
            "cards": {
                "sip": {},
                "stp": {},
                "spp": {
                    "dependencies_inline": "Fixture dependencies ready",
                    "repo_inputs_inline": "Tracked fixture inputs",
                    "target_files_surfaces_inline": "installed intent commands",
                    "deliverables_inline": "Run installed lifecycle commands",
                    "validation_plan_inline": "Declared Cargo validator",
                    "acceptance_criteria_inline": "Installed command behavior is proven",
                    "notes_risks_inline": "Synthetic transport and isolated repository"
                },
                "vpp": {},
                "srp": {},
                "sor": {}
            },
            "validators": [{
                "id": "fixture-proof",
                "program": "cargo",
                "args": ["test", "--manifest-path", "fixture-proof/Cargo.toml", "--offline"],
                "success_marker": "test result: ok."
            }],
            "publication": {
                "base": "main",
                "title": "Installed intent fixture",
                "body": "Closes #505",
                "draft": true
            }
        }),
    );
    let plan = plan.to_string_lossy().into_owned();
    assert!(fixture
        .run(&primary, &["prepare", "505", "--plan", &plan])
        .status
        .success());
    assert!(fixture.run(&primary, &["bind", "505"]).status.success());
    let linked = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    assert!(fixture.run(&linked, &["rebuild", "505"]).status.success());

    let cases = [
        (
            "false-display-only.json",
            json!({
                "schema": "csdlc.v3.intent_changes.v1",
                "cards": {"sip": {"title": "Semantic title falsely called display"}},
                "amendment": {"class": "display_only", "transition_approved": true}
            }),
        ),
        (
            "mismatched-review.json",
            json!({
                "schema": "csdlc.v3.intent_changes.v1",
                "cards": {"spp": {"plan_summary": "Changed execution plan"}},
                "amendment": {"class": "review", "transition_approved": true}
            }),
        ),
        (
            "mismatched-proof-validator.json",
            json!({
                "schema": "csdlc.v3.intent_changes.v1",
                "cards": {"sip": {"acceptance_criteria_inline": "Changed acceptance"}},
                "amendment": {"class": "proof_validator", "transition_approved": true}
            }),
        ),
    ];

    for (name, changes) in cases {
        let changes = fixture.write_json(name, &changes);
        let changes = changes.to_string_lossy().into_owned();
        let before = state_inventory(&primary, &linked);
        let output = fixture.run(&linked, &["edit", "505", "--changes", &changes]);
        assert!(
            !output.status.success(),
            "mismatched amendment was admitted"
        );
        let result = result(&output);
        assert_eq!(result["status"], "failed");
        assert!(result["findings"]
            .as_array()
            .is_some_and(|findings| findings
                .iter()
                .any(|finding| { finding["code"] == "intent_amendment_class_mismatch" })));
        assert_eq!(
            state_inventory(&primary, &linked),
            before,
            "refused amendment changed semantic, projection, or transaction state"
        );
    }
}
