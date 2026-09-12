//! PVF: deterministic installed integration; isolated Git and synthetic transport.
//! Required SIM-03 regression: issue-zero operations never acquire another issue's scope.
#![cfg(unix)]
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Output};
#[path = "support/intent_fixture.rs"]
#[allow(dead_code)]
mod intent_fixture;
use intent_fixture::{git, inventory, Fixture};

fn success(output: Output) -> Value {
    assert!(output.status.success(), "{output:?}");
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn installed_unattributed_create_cannot_be_recovered_under_another_issue() {
    let mut fixture = Fixture::new("issue-create-recovery-scope");
    let root = fixture.root.clone();
    let script = root.join(".git/installed-candidate/fake-bin/curl");
    let original = fs::read_to_string(&script).unwrap();
    let second = " *api.github.com/repos/agent-logic/agent-design-language/issues/506*) printf '%s' '{\"number\":506,\"title\":\"Second issue\",\"body\":\"Second fixture\",\"state\":\"open\",\"labels\":[],\"assignees\":[],\"milestone\":null}' ;;\n";
    fs::write(
        &script,
        original.replace(
            " *api.github.com/repos",
            &format!("{second} *api.github.com/repos"),
        ),
    )
    .unwrap();
    let mut worktrees = Vec::new();
    for issue in ["505", "506"] {
        let plan = fixture.write_json(&format!("plan-{issue}.json"), &json!({
            "schema":"csdlc.v3.intent_plan.v1", "slug":format!("scope-{issue}"),
            "cards":{"sip":{},"stp":{},"spp":{},"vpp":{},"srp":{},"sor":{}},
            "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],
            "publication":{"base":"main","title":"Scope fixture","body":format!("Closes #{issue}"),"draft":true}
        }));
        success(fixture.run(&root, &["prepare", issue, "--plan", plan.to_str().unwrap()]));
        success(fixture.run(&root, &["bind", issue]));
        let binding: Value = serde_json::from_slice(
            &fs::read(root.join(format!(".git/csdlc-v3/local/bindings/{issue}.json"))).unwrap(),
        )
        .unwrap();
        worktrees.push(PathBuf::from(binding["worktree"].as_str().unwrap()));
    }
    assert_eq!(
        git(&worktrees[0], &["rev-parse", "HEAD"]),
        git(&worktrees[1], &["rev-parse", "HEAD"])
    );
    fixture.enable_issue_transport();
    let transport = fs::read_to_string(&script).unwrap();
    fs::write(
        &script,
        transport
            .replace("cat \"$base/remote-created.json\" ;;", "exit 9 ;;")
            .replace(
                "GET:https://api.github.com/search/issues*)",
                "GET:https://api.github.com/search/issues*) exit 9;",
            ),
    )
    .unwrap();
    let operation = fixture.write_json("create-operation.json", &json!({"action":"issue_create","title":"Uncertain synthetic creation","body":"Retained creation","labels":[],"assignees":[],"milestone":null}));
    let args = [
        "github-issue",
        "505",
        "--operation",
        operation.to_str().unwrap(),
        "--execute",
    ];
    let uncertain = fixture.run(&worktrees[0], &args);
    assert!(!uncertain.status.success());
    assert_eq!(fixture.remote_effects(), 1);
    // Both equal and distinct checkout heads must exclude the unattributable
    // operation before performing issue-local head or recovery admission.
    for advanced in [false, true] {
        if advanced {
            git(
                &worktrees[1],
                &[
                    "commit",
                    "--quiet",
                    "--allow-empty",
                    "-m",
                    "advance unrelated issue",
                ],
            );
        }
        for (issue, cwd) in [("505", &worktrees[0]), ("506", &worktrees[1])] {
            for command in ["status", "recover"] {
                let before = inventory(&root);
                let result = success(fixture.run(cwd, &[command, issue]));
                assert_eq!(result["envelope"]["effects"]["outcome"], "none");
                if command == "recover" {
                    assert_eq!(result["schema"], "csdlc.v3.intent_recovery.v1");
                    assert_eq!(result["status"], "expected_noop");
                    assert!(result["transaction"].is_null());
                } else {
                    assert_eq!(result["pending_remote"], json!([]));
                    assert_ne!(result["allowed_next"], json!(["recover"]));
                }
                assert_eq!(inventory(&root), before);
            }
        }
        assert_eq!(fixture.remote_effects(), 1);
    }
    // Explicit original-operation replay still reconciles the uncertain create
    // against the synthetic remote object without a second POST.
    fs::write(&script, transport).unwrap();
    success(fixture.run(&worktrees[0], &args));
    assert_eq!(fixture.remote_effects(), 1);
}
