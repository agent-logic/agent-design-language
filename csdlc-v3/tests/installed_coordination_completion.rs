//! PVF: installed integration, deterministic synthetic authenticated GitHub;
//! required #1006 proof, no live writes or providers, bounded local Git fixtures.
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Output,
};
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use intent_fixture::{git, Fixture};
const REPO: &str = "agent-logic/agent-design-language";
const HEAD: &str = "1111111111111111111111111111111111111111";
const PROSE: &str = "Preserve this coordinator prose exactly.";

fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "installed command failed: {output:?}"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(value["envelope"].is_object());
    value
}
fn write(path: &Path, value: &Value) {
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
}
fn base(fixture: &Fixture) -> PathBuf {
    fixture.root.join(".git/installed-candidate")
}
fn setup(label: &str) -> (Fixture, PathBuf, String) {
    let mut fixture = Fixture::new(label);
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    let plan=fixture.write_json("plan.json",&json!({"schema":"csdlc.v3.intent_plan.v1","slug":"installed-coordination-fixture","cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Declared children complete","repo_inputs_inline":"Synthetic authenticated observations","target_files_surfaces_inline":"Installed coordination command","deliverables_inline":"Verify guarded completion","validation_plan_inline":"Installed synthetic transport","acceptance_criteria_inline":"Completion and finish reconcile","notes_risks_inline":"No live mutations"},"vpp":{},"srp":{},"sor":{}},"validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],"publication":{"base":"main","title":"Installed coordination fixture","body":"Closes #505","draft":true}}));
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", plan.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = git(&primary, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .find(|path| path != &primary)
        .unwrap();
    let contract = json!({"repository":REPO,"issue":505,"kind":"coordination_only","children":[{"issue":887,"pull_request":989,"head_sha":HEAD}]});
    let body = format!("{PROSE}\n\n<!-- csdlc-coordination:v1 {contract} -->");
    let mut remote = fixture.remote_issue();
    remote["body"] = json!(body);
    remote["html_url"] = json!(format!("https://github.com/{REPO}/issues/505"));
    remote["updated_at"] = json!("2026-09-16T00:00:00Z");
    remote["closed_at"] = json!("2026-09-16T00:00:00Z");
    write(&base(&fixture).join("remote-issue.json"), &remote);
    write(
        &base(&fixture).join("child.json"),
        &json!({"number":887,"html_url":format!("https://github.com/{REPO}/issues/887"),"state":"closed","state_reason":"completed"}),
    );
    write(
        &base(&fixture).join("child-merge.json"),
        &json!({"data":{"repository":{"nameWithOwner":REPO,"pullRequest":{"number":989,"url":format!("https://github.com/{REPO}/pull/989"),"state":"MERGED","headRefOid":HEAD,"merged":true,"mergeCommit":{"oid":"2222222222222222222222222222222222222222"},"body":"Closes #887","closingIssuesReferences":{"nodes":[{"number":887,"url":format!("https://github.com/{REPO}/issues/887"),"repository":{"nameWithOwner":REPO}}],"pageInfo":{"hasNextPage":false}}}},"linkedRepository":{"nameWithOwner":REPO,"issue":{"number":887,"url":format!("https://github.com/{REPO}/issues/887"),"state":"CLOSED"}}}}),
    );
    let script = base(&fixture).join("fake-bin/curl");
    let original = fs::read_to_string(&script).unwrap();
    let routes=" GET:https://api.github.com/repos/agent-logic/agent-design-language/issues/887) cat \"$base/child.json\" ;;\n POST:https://api.github.com/graphql) cat \"$base/child-merge.json\" ;;\n";
    fs::write(
        script,
        original.replacen(
            "case \"$method:$url\" in\n",
            &format!("case \"$method:$url\" in\n{routes}"),
            1,
        ),
    )
    .unwrap();
    fs::create_dir_all(linked.join(".csdlc/evidence/505")).unwrap();
    fs::write(
        linked.join(".csdlc/evidence/505/coordination.json"),
        b"synthetic child delivery proof\n",
    )
    .unwrap();
    (fixture, linked, body)
}
fn operation(body: &str) -> Value {
    json!({"action":"issue_complete_coordination","operator_approval":"Synthetic operator approves this exact completion","completion":{"current_body":body,"expected_updated_at":"2026-09-16T00:00:00Z","rationale":"All declared children have authenticated delivery evidence","evidence":[{"path":".csdlc/evidence/505/coordination.json","digest":blake3::hash(b"synthetic child delivery proof\n").to_hex().to_string()}]}})
}
fn installation_operation(body: &str) -> Value {
    let mut value = operation(body);
    value["completion"]["install_contract"] = json!({
        "repository": REPO,
        "issue": 505,
        "kind": "coordination_only",
        "children": [{"issue":887,"pull_request":989,"head_sha":HEAD}]
    });
    value
}
fn execute(fixture: &mut Fixture, linked: &Path, path: &Path) -> Output {
    execute_family(fixture, linked, "github-issue", path)
}
fn execute_family(fixture: &mut Fixture, linked: &Path, family: &str, path: &Path) -> Output {
    fixture.run(
        linked,
        &[
            family,
            "505",
            "--operation",
            path.to_str().unwrap(),
            "--execute",
        ],
    )
}
fn retain_legacy_only(fixture: &Fixture) {
    fs::remove_dir_all(fixture.root.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
}

#[test]
fn installed_coordination_completion_preserves_prose_reconciles_once_and_finishes() {
    let (mut fixture, linked, body) = setup("coordination-success");
    // Establish classification through the ordinary native edit, not a fabricated completion request.
    let mut remote = fixture.remote_issue();
    remote["body"] = json!(PROSE);
    write(&base(&fixture).join("remote-issue.json"), &remote);
    let edit = fixture.write_json(
        "establish-contract.json",
        &json!({"action":"issue_edit","title":null,"body":body}),
    );
    success(execute(&mut fixture, &linked, &edit));
    assert_eq!(fixture.remote_effects(), 1);
    let approved_body = fixture.remote_issue()["body"].as_str().unwrap().to_owned();
    assert!(approved_body.starts_with(&body));
    assert!(approved_body.contains("<!-- csdlc-v3-operation:"));
    let op = fixture.write_json("completion.json", &operation(&approved_body));
    success(execute(&mut fixture, &linked, &op));
    let closed = fixture.remote_issue();
    assert_eq!(closed["state"], "closed");
    assert_eq!(closed["state_reason"], "completed");
    assert!(closed["body"].as_str().unwrap().starts_with(&approved_body));
    assert_eq!(
        closed["body"]
            .as_str()
            .unwrap()
            .matches("<!-- csdlc-v3-operation:")
            .count(),
        2
    );
    assert_eq!(fixture.remote_effects(), 2);
    success(execute(&mut fixture, &linked, &op));
    assert_eq!(fixture.remote_effects(), 2, "replay repeated PATCH");
    let disposition=fixture.write_json("disposition.json",&json!({"disposition":"coordination_completed","operator":"synthetic-fixture-operator","rationale":"Verified coordination delivery","evidence_refs":[".csdlc/evidence/505/coordination.json"]}));
    success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    let receipt: Value = serde_json::from_slice(
        &fs::read(
            fixture
                .root
                .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert!(receipt["pull_request"].is_null());
    assert_eq!(
        receipt["no_pr_closeout"]["disposition"],
        "coordination_completed"
    );
    assert_eq!(fixture.remote_effects(), 2);
}

#[test]
fn installed_coordination_completion_denials_have_no_remote_effects() {
    for case in [
        "ordinary_body",
        "no_approval",
        "stale_evidence",
        "open_child",
        "bad_head",
    ] {
        let (mut fixture, linked, body) = setup(case);
        let mut op = operation(&body);
        match case {
            "ordinary_body" => {
                op["completion"]["current_body"] = json!("Ordinary implementation issue")
            }
            "no_approval" => {
                op.as_object_mut().unwrap().remove("operator_approval");
            }
            "stale_evidence" => fs::write(
                linked.join(".csdlc/evidence/505/coordination.json"),
                b"changed",
            )
            .unwrap(),
            "open_child" => {
                let mut child: Value =
                    serde_json::from_slice(&fs::read(base(&fixture).join("child.json")).unwrap())
                        .unwrap();
                child["state"] = json!("open");
                write(&base(&fixture).join("child.json"), &child);
            }
            "bad_head" => {
                let mut child: Value = serde_json::from_slice(
                    &fs::read(base(&fixture).join("child-merge.json")).unwrap(),
                )
                .unwrap();
                child["data"]["repository"]["pullRequest"]["headRefOid"] =
                    json!("3333333333333333333333333333333333333333");
                write(&base(&fixture).join("child-merge.json"), &child);
            }
            _ => unreachable!(),
        }
        let path = fixture.write_json("completion.json", &op);
        let output = execute(&mut fixture, &linked, &path);
        assert!(!output.status.success(), "denial accepted: {case}");
        let expected = if case == "no_approval" {
            "intent_remote_operation_invalid"
        } else {
            "github_coordination_completion_denied"
        };
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(expected),
            "wrong denial for {case}: {output:?}"
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
        assert_eq!(
            fixture.remote_effects(),
            0,
            "denied operation mutated remote: {case}"
        );
    }
}

#[test]
fn installed_coordination_absence_retry_rechecks_changed_child_and_evidence() {
    for case in ["unchanged", "changed_child", "changed_evidence"] {
        let (mut fixture, linked, body) = setup(case);
        let script = base(&fixture).join("fake-bin/curl");
        let original = fs::read_to_string(&script).unwrap();
        let patch =
            " PATCH:https://api.github.com/repos/agent-logic/agent-design-language/issues/505)\n";
        assert!(original.contains(patch));
        // First PATCH has an uncertain transport outcome and no remote effect.
        // A second dispatch would succeed: the guard must prevent that attempt.
        let replacement = format!("{patch}  printf 'attempt\\n' >> \"$base/completion-attempts\"\n  if test -f \"$base/drop-completion\"; then exit 9; fi\n");
        fs::write(&script, original.replacen(patch, &replacement, 1)).unwrap();
        fs::write(base(&fixture).join("drop-completion"), b"synthetic").unwrap();
        let completion = operation(&body);
        let initial = fixture.write_json("completion.json", &completion);
        let failed = execute(&mut fixture, &linked, &initial);
        assert!(!failed.status.success());
        assert_eq!(fixture.remote_effects(), 0);
        assert_eq!(
            fs::read_to_string(base(&fixture).join("completion-attempts"))
                .unwrap()
                .lines()
                .count(),
            1
        );
        fs::remove_file(base(&fixture).join("drop-completion")).unwrap();
        if case == "changed_child" {
            let path = base(&fixture).join("child.json");
            let mut child: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            child["state"] = json!("open");
            write(&path, &child);
        } else if case == "changed_evidence" {
            fs::write(
                linked.join(".csdlc/evidence/505/coordination.json"),
                b"changed after initial attempt",
            )
            .unwrap();
        }
        let retry = fixture.write_json(
            "retry.json",
            &json!({"operation":completion,"recovery":"retry_after_authenticated_absence"}),
        );
        let denied = execute(&mut fixture, &linked, &retry);
        if case == "unchanged" {
            success(denied);
            assert_eq!(fixture.remote_issue()["state"], "closed");
            assert_eq!(fixture.remote_effects(), 1);
            assert_eq!(
                fs::read_to_string(base(&fixture).join("completion-attempts"))
                    .unwrap()
                    .lines()
                    .count(),
                2
            );
            continue;
        }

        assert!(
            !denied.status.success(),
            "changed readiness accepted on retry: {case}"
        );
        assert_eq!(fixture.remote_issue()["state"], "open");
        assert_eq!(fixture.remote_effects(), 0);
        assert_eq!(
            fs::read_to_string(base(&fixture).join("completion-attempts"))
                .unwrap()
                .lines()
                .count(),
            1,
            "retry dispatched despite changed readiness: {case}"
        );
    }
}

// PVF #1006 review regression: parent references do not change child delivery.
#[test]
fn installed_coordination_accepts_child_closing_link_with_parent_reference() {
    let (mut fixture, linked, body) = setup("coordination-child-parent-reference");
    let path = base(&fixture).join("child-merge.json");
    let mut child: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    child["data"]["repository"]["pullRequest"]["body"] = json!("Closes #887\n\nPart of #505");
    write(&path, &child);
    let op = fixture.write_json("completion.json", &operation(&body));
    success(execute(&mut fixture, &linked, &op));
    assert_eq!(fixture.remote_issue()["state"], "closed");
    assert_eq!(fixture.remote_issue()["state_reason"], "completed");
    assert_eq!(fixture.remote_effects(), 1);
    success(execute(&mut fixture, &linked, &op));
    assert_eq!(fixture.remote_effects(), 1, "replay repeated PATCH");
}

#[test]
fn installed_legacy_coordination_completion_uses_native_guards_and_replays_once() {
    let (mut fixture, linked, body) = setup("legacy-coordination-success");
    retain_legacy_only(&fixture);
    let op = fixture.write_json("legacy-completion.json", &operation(&body));
    let first = success(execute(&mut fixture, &linked, &op));
    assert_eq!(first["compatibility"], "legacy_coordination_only");
    assert_eq!(first["performed_mutation"], true);
    assert_eq!(fixture.remote_issue()["state"], "closed");
    assert_eq!(fixture.remote_issue()["state_reason"], "completed");
    assert_eq!(fixture.remote_effects(), 1);
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/semantic/issues/505")
        .exists());

    let replay = success(execute(&mut fixture, &linked, &op));
    assert_eq!(replay["compatibility"], "legacy_coordination_only");
    assert_eq!(replay["performed_mutation"], false);
    assert_eq!(fixture.remote_effects(), 1, "legacy replay repeated PATCH");
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/semantic/issues/505")
        .exists());

    let disposition=fixture.write_json("legacy-disposition.json",&json!({"disposition":"coordination_completed","operator":"synthetic-fixture-operator","rationale":"Verified legacy coordination delivery","evidence_refs":[".csdlc/evidence/505/coordination.json"]}));
    let finish = success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    assert_eq!(finish["status"], "completed");
    assert_eq!(finish["compatibility"], "legacy_coordination_only");
    let receipt: Value = serde_json::from_slice(
        &fs::read(
            fixture
                .root
                .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        receipt["no_pr_closeout"]["disposition"],
        "coordination_completed"
    );
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/semantic/issues/505")
        .exists());
    let finish_replay = success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    assert_eq!(finish_replay["status"], "expected_noop");
    assert_eq!(finish_replay["performed_mutation"], false);
    assert_eq!(finish_replay["compatibility"], "legacy_coordination_only");

    let cleanup_preview = success(fixture.run(&linked, &["clean", "505", "--preview", "plan"]));
    let cleanup_token = cleanup_preview["preview_token"]
        .as_str()
        .expect("legacy cleanup preview token");
    let cleanup = success(fixture.run(
        &linked,
        &["clean", "505", "--execute", "--preview", cleanup_token],
    ));
    assert_eq!(cleanup["status"], "completed");
    assert_eq!(cleanup["compatibility"], "legacy_coordination_only");
    assert!(!linked.exists());
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/semantic/issues/505")
        .exists());
    let primary = fixture.root.clone();
    let cleanup_replay = success(fixture.run(&primary, &["clean", "505"]));
    assert_eq!(cleanup_replay["status"], "expected_noop");
    assert_eq!(cleanup_replay["performed_mutation"], false);
    assert_eq!(cleanup_replay["compatibility"], "legacy_coordination_only");
}

#[test]
fn installed_legacy_completion_installs_contract_atomically_and_replays_once() {
    let (mut fixture, linked, _) = setup("legacy-contract-installation");
    retain_legacy_only(&fixture);
    let mut remote = fixture.remote_issue();
    remote["body"] = json!(PROSE);
    write(&base(&fixture).join("remote-issue.json"), &remote);

    let operation = installation_operation(PROSE);
    let path = fixture.write_json("legacy-install-and-complete.json", &operation);
    let first = success(execute(&mut fixture, &linked, &path));
    assert_eq!(first["compatibility"], "legacy_coordination_only");
    assert_eq!(first["performed_mutation"], true);
    let closed = fixture.remote_issue();
    assert_eq!(closed["state"], "closed");
    assert_eq!(closed["state_reason"], "completed");
    let body = closed["body"].as_str().unwrap();
    assert!(body.starts_with(PROSE));
    assert_eq!(body.matches("<!-- csdlc-coordination:v1 ").count(), 1);
    assert_eq!(body.matches("<!-- csdlc-v3-operation:").count(), 1);
    assert_eq!(fixture.remote_effects(), 1);

    let replay = success(execute(&mut fixture, &linked, &path));
    assert_eq!(replay["performed_mutation"], false);
    assert_eq!(fixture.remote_effects(), 1, "replay repeated PATCH");
}

#[test]
fn installed_legacy_contract_installation_denies_invalid_inputs_without_effects() {
    for case in [
        "stale_body",
        "stale_timestamp",
        "wrong_repository",
        "wrong_issue",
        "wrong_children",
        "malformed_contract",
        "existing_malformed_marker",
        "missing_approval",
    ] {
        let (mut fixture, linked, _) = setup(&format!("legacy-install-{case}"));
        retain_legacy_only(&fixture);
        let mut remote = fixture.remote_issue();
        remote["body"] = json!(PROSE);
        write(&base(&fixture).join("remote-issue.json"), &remote);
        let mut operation = installation_operation(PROSE);
        match case {
            "stale_body" => operation["completion"]["current_body"] = json!("stale prose"),
            "stale_timestamp" => {
                operation["completion"]["expected_updated_at"] = json!("2026-09-16T00:00:01Z")
            }
            "wrong_repository" => {
                operation["completion"]["install_contract"]["repository"] =
                    json!("other/repository")
            }
            "wrong_issue" => operation["completion"]["install_contract"]["issue"] = json!(506),
            "wrong_children" => {
                operation["completion"]["install_contract"]["children"][0]["issue"] = json!(888)
            }
            "malformed_contract" => {
                operation["completion"]["install_contract"]["unexpected"] = json!(true)
            }
            "existing_malformed_marker" => {
                let malformed = format!("{PROSE}\n\n<!-- csdlc-coordination:v1 invalid -->");
                operation["completion"]["current_body"] = json!(malformed);
                let mut remote = fixture.remote_issue();
                remote["body"] = operation["completion"]["current_body"].clone();
                write(&base(&fixture).join("remote-issue.json"), &remote);
            }
            "missing_approval" => {
                operation
                    .as_object_mut()
                    .unwrap()
                    .remove("operator_approval");
            }
            _ => unreachable!(),
        }
        let path = fixture.write_json("legacy-install-denied.json", &operation);
        let denied = execute(&mut fixture, &linked, &path);
        assert!(
            !denied.status.success(),
            "invalid installation accepted: {case}"
        );
        assert_eq!(fixture.remote_effects(), 0, "denial mutated remote: {case}");
    }
}

#[test]
fn installed_semantic_completion_cannot_use_legacy_contract_installation() {
    let (mut fixture, linked, _) = setup("semantic-install-denied");
    let mut remote = fixture.remote_issue();
    remote["body"] = json!(PROSE);
    write(&base(&fixture).join("remote-issue.json"), &remote);
    let operation = fixture.write_json(
        "semantic-install-denied.json",
        &installation_operation(PROSE),
    );
    let denied = execute(&mut fixture, &linked, &operation);
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stdout)
        .contains("intent_coordination_contract_installation_legacy_only"));
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn installed_legacy_contract_installation_replay_rejects_changed_contract() {
    let (mut fixture, linked, _) = setup("legacy-install-replay-mismatch");
    retain_legacy_only(&fixture);
    let mut remote = fixture.remote_issue();
    remote["body"] = json!(PROSE);
    write(&base(&fixture).join("remote-issue.json"), &remote);
    let first = fixture.write_json("legacy-install-first.json", &installation_operation(PROSE));
    success(execute(&mut fixture, &linked, &first));
    assert_eq!(fixture.remote_effects(), 1);

    let mut changed = installation_operation(PROSE);
    changed["completion"]["install_contract"]["children"][0]["head_sha"] =
        json!("3333333333333333333333333333333333333333");
    let changed = fixture.write_json("legacy-install-changed.json", &changed);
    let denied = execute(&mut fixture, &linked, &changed);
    assert!(!denied.status.success(), "changed replay was accepted");
    assert_eq!(fixture.remote_effects(), 1, "changed replay repeated PATCH");
}

#[test]
fn installed_legacy_finish_requires_exact_native_coordination_completion() {
    let (mut fixture, linked, _) = setup("legacy-finish-without-completion");
    retain_legacy_only(&fixture);
    let mut remote = fixture.remote_issue();
    remote["state"] = json!("closed");
    remote["state_reason"] = json!("completed");
    write(&base(&fixture).join("remote-issue.json"), &remote);
    let disposition=fixture.write_json("legacy-direct-disposition.json",&json!({"disposition":"coordination_completed","operator":"synthetic-fixture-operator","rationale":"Attempt direct terminal admission","evidence_refs":[".csdlc/evidence/505/coordination.json"]}));

    let denied = fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    );
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stdout)
        .contains("intent_legacy_coordination_completion_receipt_required"));
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .exists());
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/local/v3/issues/505/terminal.json")
        .exists());

    let clean = fixture.run(&linked, &["clean", "505", "--preview", "plan"]);
    assert!(!clean.status.success());
    assert!(linked.exists());
    assert!(!fixture.root.join(".git/csdlc-v3/local/archives").exists());
}

#[test]
fn installed_legacy_cleanup_replay_requires_native_cleanup_archive() {
    let (mut fixture, linked, body) = setup("legacy-manual-removal");
    retain_legacy_only(&fixture);
    let op = fixture.write_json("legacy-completion.json", &operation(&body));
    success(execute(&mut fixture, &linked, &op));
    let disposition=fixture.write_json("legacy-disposition.json",&json!({"disposition":"coordination_completed","operator":"synthetic-fixture-operator","rationale":"Verified legacy coordination delivery","evidence_refs":[".csdlc/evidence/505/coordination.json"]}));
    success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));

    let removed = std::process::Command::new("git")
        .arg("-C")
        .arg(&fixture.root)
        .args(["worktree", "remove", "--force", "--"])
        .arg(&linked)
        .output()
        .unwrap();
    assert!(
        removed.status.success(),
        "manual removal failed: {removed:?}"
    );
    assert!(!linked.exists());
    assert!(!fixture.root.join(".git/csdlc-v3/local/archives").exists());

    let denied = fixture.run(&fixture.root.clone(), &["clean", "505"]);
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stdout).contains("cleanup_archive_recovery_required"));
}

#[test]
fn installed_legacy_coordination_completion_preserves_all_denials() {
    for case in [
        "ordinary_body",
        "no_approval",
        "stale_parent",
        "stale_evidence",
        "open_child",
        "bad_head",
    ] {
        let (mut fixture, linked, body) = setup(&format!("legacy-{case}"));
        retain_legacy_only(&fixture);
        let mut op = operation(&body);
        match case {
            "ordinary_body" => {
                op["completion"]["current_body"] = json!("Ordinary implementation issue")
            }
            "no_approval" => {
                op.as_object_mut().unwrap().remove("operator_approval");
            }
            "stale_parent" => {
                op["completion"]["expected_updated_at"] = json!("2026-09-16T00:00:01Z")
            }
            "stale_evidence" => fs::write(
                linked.join(".csdlc/evidence/505/coordination.json"),
                b"changed",
            )
            .unwrap(),
            "open_child" => {
                let path = base(&fixture).join("child.json");
                let mut child: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                child["state"] = json!("open");
                write(&path, &child);
            }
            "bad_head" => {
                let path = base(&fixture).join("child-merge.json");
                let mut child: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                child["data"]["repository"]["pullRequest"]["headRefOid"] =
                    json!("3333333333333333333333333333333333333333");
                write(&path, &child);
            }
            _ => unreachable!(),
        }
        let path = fixture.write_json("legacy-denial.json", &op);
        let denied = execute(&mut fixture, &linked, &path);
        assert!(!denied.status.success(), "legacy denial accepted: {case}");
        let expected = if case == "no_approval" {
            "intent_remote_operation_invalid"
        } else {
            "github_coordination_completion_denied"
        };
        assert!(
            String::from_utf8_lossy(&denied.stdout).contains(expected),
            "wrong legacy denial for {case}: {denied:?}"
        );
        assert_eq!(fixture.remote_effects(), 0);
    }
}

#[test]
fn installed_legacy_non_coordination_mutations_remain_denied() {
    let cases = [
        (
            "issue-create",
            "github-issue",
            json!({"action":"issue_create","title":"denied","body":"denied"}),
        ),
        (
            "issue-comment",
            "github-issue",
            json!({"action":"issue_comment","body":"denied"}),
        ),
        (
            "issue-edit",
            "github-issue",
            json!({"action":"issue_edit","title":"denied","body":null}),
        ),
        (
            "issue-close",
            "github-issue",
            json!({"action":"issue_close","rationale":"denied","current_body":"denied","disposition":"no_op","github_state_reason":"not_planned"}),
        ),
        (
            "pr-create",
            "github-pr",
            json!({"action":"pull_request_create","base":"main","head":"codex/denied","title":"denied","body":"Closes #505","draft":true}),
        ),
        (
            "pr-update",
            "github-pr",
            json!({"action":"pull_request_update","title":"denied","body":null}),
        ),
        (
            "pr-ready",
            "github-pr",
            json!({"action":"pull_request_ready"}),
        ),
        (
            "pr-merge",
            "github-pr",
            json!({"action":"pull_request_merge","base":"main","method":"merge","operator_approval":"denied on legacy state"}),
        ),
    ];
    for (label, family, operation) in cases {
        let (mut fixture, linked, _) = setup(&format!("legacy-{label}-denied"));
        retain_legacy_only(&fixture);
        let path = fixture.write_json(&format!("legacy-{label}.json"), &operation);
        let denied = execute_family(&mut fixture, &linked, family, &path);
        assert!(
            !denied.status.success(),
            "legacy mutation accepted: {label}"
        );
        assert!(
            String::from_utf8_lossy(&denied.stdout).contains("intent_semantic_migration_required"),
            "wrong non-coordination denial for {label}: {denied:?}"
        );
        assert_eq!(
            fixture.remote_effects(),
            0,
            "legacy mutation escaped: {label}"
        );
    }
}

#[test]
fn installed_legacy_coordination_uncertainty_requires_guarded_exact_retry() {
    let (mut fixture, linked, body) = setup("legacy-coordination-uncertain");
    retain_legacy_only(&fixture);
    let script = base(&fixture).join("fake-bin/curl");
    let original = fs::read_to_string(&script).unwrap();
    let patch =
        " PATCH:https://api.github.com/repos/agent-logic/agent-design-language/issues/505)\n";
    assert!(original.contains(patch));
    let replacement = format!(
        "{patch}  printf 'attempt\\n' >> \"$base/completion-attempts\"\n  if test -f \"$base/drop-completion\"; then exit 9; fi\n"
    );
    fs::write(&script, original.replacen(patch, &replacement, 1)).unwrap();
    fs::write(base(&fixture).join("drop-completion"), b"synthetic").unwrap();

    let completion = operation(&body);
    let initial = fixture.write_json("legacy-uncertain.json", &completion);
    let failed = execute(&mut fixture, &linked, &initial);
    assert!(!failed.status.success());
    let report: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(report["status"], "recovery_required");
    assert_eq!(report["effects_unknown"], true);
    assert_eq!(report["compatibility"], "legacy_coordination_only");
    assert_eq!(fixture.remote_effects(), 0);
    assert_eq!(
        fs::read_to_string(base(&fixture).join("completion-attempts"))
            .unwrap()
            .lines()
            .count(),
        1
    );

    fs::remove_file(base(&fixture).join("drop-completion")).unwrap();
    fs::write(
        linked.join(".csdlc/evidence/505/coordination.json"),
        b"changed before retry",
    )
    .unwrap();
    let retry = fixture.write_json(
        "legacy-uncertain-retry.json",
        &json!({"operation":completion,"recovery":"retry_after_authenticated_absence"}),
    );
    let denied = execute(&mut fixture, &linked, &retry);
    assert!(!denied.status.success());
    assert!(
        String::from_utf8_lossy(&denied.stdout).contains("github_coordination_completion_denied")
    );
    assert_eq!(fixture.remote_effects(), 0);
    assert_eq!(
        fs::read_to_string(base(&fixture).join("completion-attempts"))
            .unwrap()
            .lines()
            .count(),
        1,
        "guard failure dispatched the retained completion"
    );
}
