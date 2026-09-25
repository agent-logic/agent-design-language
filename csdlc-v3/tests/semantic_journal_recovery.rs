//! PVF: required deterministic installed recovery regression, local Git/files only,
//! synthetic authenticated transport; no network or paid provider activity.
use serde_json::{json, Value};
use std::{fs, process::Output};
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod fixture;
use fixture::Fixture;
fn success(o: Output) -> Value {
    assert!(o.status.success(), "{o:?}");
    serde_json::from_slice(&o.stdout).unwrap()
}
fn plan() -> Value {
    json!({"schema":"csdlc.v3.intent_plan.v1", "slug":"installed-intent-fixture",
      "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Fixture dependencies ready","repo_inputs_inline":"Tracked fixture inputs","target_files_surfaces_inline":"installed intent commands","deliverables_inline":"Run installed lifecycle commands","validation_plan_inline":"Declared Cargo validator","acceptance_criteria_inline":"Installed command behavior is proven","notes_risks_inline":"Synthetic transport and isolated repository"},"vpp":{},"srp":{},"sor":{}},
      "validators":[{"id":"fixture-proof", "program":"cargo",
        "args":["test", "--manifest-path", "fixture-proof/Cargo.toml", "--offline"], "success_marker":"test result: ok."}],
      "publication":{"base":"main","title":"Installed intent fixture", "body":"Closes #505", "draft":true}})
}
fn prepared(name: &str) -> Fixture {
    let mut f = Fixture::new(name);
    let root = f.root.clone();
    let p = f.write_json("plan.json", &plan());
    success(f.run(&root, &["prepare", "505", "--plan", p.to_str().unwrap()]));
    f
}
#[test]
fn installed_recovers_retained_issue_commit_without_dispatch() {
    for point in [
        "journal_after_retained_commit",
        "journal_after_next_pointer",
    ] {
        let mut f = Fixture::new(point);
        let root = f.root.clone();
        let p = f.write_json("plan.json", &plan());
        let o = f.run_with_env(
            &root,
            &["prepare", "505", "--plan", p.to_str().unwrap()],
            &[("CSDLC_V3_TEST_CRASH_POINT", point)],
        );
        assert_eq!(o.status.code(), Some(91));
        let before = fixture::inventory(&root);
        let preview = success(f.run(&root, &["recover", "505"]));
        assert_eq!(preview["action"], "activate_retained_issue_commit");
        assert_eq!(before, fixture::inventory(&root));
        assert!(!f
            .run(
                &root,
                &["recover", "505", "--execute", "--preview", "stale"]
            )
            .status
            .success());
        assert_eq!(before, fixture::inventory(&root));
        let token = preview["preview_digest"].as_str().unwrap();
        let result = success(f.run(&root, &["recover", "505", "--execute", "--preview", token]));
        assert_eq!(result["business_effect_replayed"], false);
        assert_eq!(f.remote_effects(), 0);
    }
}
fn interrupted_finish(point: &str) -> Fixture {
    let mut f = prepared(point);
    f.enable_issue_transport();
    let root = f.root.clone();
    success(f.run(&root, &["bind", "505"]));
    let mut remote = f.remote_issue();
    remote["state"] = json!("closed");
    remote["updated_at"] = json!("2026-09-11T12:00:00Z");
    remote["closed_at"] = json!("2026-09-11T12:00:00Z");
    fs::write(
        root.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&remote).unwrap(),
    )
    .unwrap();
    let disposition=f.write_json("disposition.json",&json!({"disposition":"retired_without_execution","operator":"fixture","rationale":"Synthetic retirement","evidence_refs":["fixture:decision"]}));
    let o = f.run_with_env(
        &root,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
        &[("CSDLC_V3_TEST_CRASH_POINT", point)],
    );
    assert_eq!(o.status.code(), Some(91), "{o:?}");
    f
}
#[test]
fn installed_finish_recovers_reservation_and_state_only_crashes() {
    for point in ["finish_after_reservation", "finish_after_state_write"] {
        let mut f = interrupted_finish(point);
        let root = f.root.clone();
        let before = fixture::inventory(&root);
        let preview = success(f.run(&root, &["recover", "505"]));
        assert_eq!(preview["action"], "reconcile_native_finish");
        assert_eq!(before, fixture::inventory(&root));
        assert!(!f
            .run(
                &root,
                &["recover", "505", "--execute", "--preview", "stale"]
            )
            .status
            .success());
        let token = preview["preview_digest"].as_str().unwrap();
        success(f.run(&root, &["recover", "505", "--execute", "--preview", token]));
        assert!(root
            .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
            .exists());
        assert_eq!(f.remote_effects(), 0);
    }
}
#[test]
fn installed_finish_recovery_preserves_conflicting_state() {
    let mut f = interrupted_finish("finish_after_reservation");
    let root = f.root.clone();
    let original_preview = success(f.run(&root, &["recover", "505"]));
    let path = root.join(".git/csdlc-v3/local/v3/issues/505/terminal.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, b"conflicting state").unwrap();
    let stale = f.run(
        &root,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            original_preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stdout).contains("intent_finish_recovery_preview_stale"));
    let preview = success(f.run(&root, &["recover", "505"]));
    let token = preview["preview_digest"].as_str().unwrap();
    assert!(!f
        .run(&root, &["recover", "505", "--execute", "--preview", token])
        .status
        .success());
    assert_eq!(fs::read(path).unwrap(), b"conflicting state");
    assert_eq!(f.remote_effects(), 0);
}

#[test]
fn installed_creation_journal_activation_never_dispatches_creation() {
    let mut f = prepared("creation-journal");
    let root = f.root.clone();
    f.enable_issue_transport();
    success(f.run(&root, &["bind", "505"]));
    let op=f.write_json("create.json",&json!({"action":"issue_create","title":"Synthetic creation","body":"Exact request","labels":[],"assignees":[],"milestone":null}));
    let o = f.run_with_env(
        &root,
        &[
            "github-issue",
            "505",
            "--operation",
            op.to_str().unwrap(),
            "--execute",
        ],
        &[("CSDLC_V3_TEST_CRASH_POINT", "creation_journal_after_commit")],
    );
    assert_eq!(o.status.code(), Some(91), "{o:?}");
    // The immutable record supplies the explicit operation identity; no fabricated capability.
    fn retained_creation(path: &std::path::Path) -> Option<Value> {
        for entry in fs::read_dir(path).ok()? {
            let p = entry.ok()?.path();
            if p.is_dir() {
                if let Some(v) = retained_creation(&p) {
                    return Some(v);
                }
            } else if let Ok(v) = serde_json::from_slice::<Value>(&fs::read(p).unwrap_or_default())
            {
                if v["payload"]["id"].is_string() && v["payload"]["native"].is_object() {
                    return Some(v);
                }
            }
        }
        None
    }
    let record =
        retained_creation(&root.join(".git/csdlc-v3/semantic")).expect("retained creation record");
    let d=f.write_json("recover-creation.json",&json!({"schema":"csdlc.v3.creation_journal_recovery_disposition.v1","operation_id":record["payload"]["id"]}));
    let preview = success(f.run(
        &root,
        &["recover", "505", "--disposition", d.to_str().unwrap()],
    ));
    assert!(!f
        .run(
            &root,
            &[
                "recover",
                "505",
                "--disposition",
                d.to_str().unwrap(),
                "--execute",
                "--preview",
                "stale"
            ]
        )
        .status
        .success());
    let token = preview["preview_digest"].as_str().unwrap();
    success(f.run(
        &root,
        &[
            "recover",
            "505",
            "--disposition",
            d.to_str().unwrap(),
            "--execute",
            "--preview",
            token,
        ],
    ));
    assert_eq!(f.remote_effects(), 0);
}

#[test]
fn installed_journal_refuses_corrupted_commit_without_deleting_evidence() {
    let mut f = Fixture::new("journal-corruption");
    let root = f.root.clone();
    let p = f.write_json("plan.json", &plan());
    let o = f.run_with_env(
        &root,
        &["prepare", "505", "--plan", p.to_str().unwrap()],
        &[("CSDLC_V3_TEST_CRASH_POINT", "journal_after_retained_commit")],
    );
    assert_eq!(o.status.code(), Some(91));
    let commits = root.join(".git/csdlc-v3/semantic/issues/505/commits");
    let path = fs::read_dir(commits)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(path, b"corrupt retained immutable commit").unwrap();
    let before = fixture::inventory(&root);
    assert!(!f.run(&root, &["recover", "505"]).status.success());
    assert_eq!(before, fixture::inventory(&root));
    assert_eq!(f.remote_effects(), 0);
}

#[test]
fn installed_finish_recovery_refuses_changed_remote_closeout() {
    let mut f = interrupted_finish("finish_after_reservation");
    let root = f.root.clone();
    let mut remote = f.remote_issue();
    remote["state"] = json!("open");
    remote["closed_at"] = Value::Null;
    fs::write(
        root.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&remote).unwrap(),
    )
    .unwrap();
    let before = fixture::inventory(&root);
    assert!(!f.run(&root, &["recover", "505"]).status.success());
    assert_eq!(before, fixture::inventory(&root));
    assert!(!root
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .exists());
}

#[test]
fn installed_merged_finish_replays_retained_request_after_crash() {
    for point in ["finish_after_reservation", "finish_after_state_write"] {
        let mut f = prepared(point);
        let root = f.root.clone();
        f.enable_issue_transport();
        success(f.run(&root, &["bind", "505"]));
        let binding: Value = serde_json::from_slice(
            &fs::read(root.join(".git/csdlc-v3/local/bindings/505.json")).unwrap(),
        )
        .unwrap();
        let linked = std::path::PathBuf::from(binding["worktree"].as_str().unwrap());
        f.enable_pr_transport(&linked);
        let head = fixture::git(&linked, &["rev-parse", "HEAD"]);
        f.set_remote_pr(&json!({"number":639,"head":{"sha":head},"merged":true,"state":"closed","body":"Closes #505"}));
        let mut issue = f.remote_issue();
        issue["state"] = json!("closed");
        fs::write(
            root.join(".git/installed-candidate/remote-issue.json"),
            serde_json::to_vec(&issue).unwrap(),
        )
        .unwrap();
        let args = ["finish", "505", "--pull-request", "639"];
        let crash = f.run_with_env(&linked, &args, &[("CSDLC_V3_TEST_CRASH_POINT", point)]);
        assert_eq!(crash.status.code(), Some(91), "{crash:?}");
        success(f.run(&linked, &args));
        let receipt: Value = serde_json::from_slice(
            &fs::read(root.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["pull_request"], 639);
        assert_eq!(receipt["head_sha"], head);
        let before = fixture::inventory(&root);
        success(f.run(&linked, &args));
        assert_eq!(before, fixture::inventory(&root));
        assert_eq!(f.remote_effects(), 0);
    }
}
