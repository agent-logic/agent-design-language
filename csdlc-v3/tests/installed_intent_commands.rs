//! PVF: installed integration proof, deterministic local Git and synthetic transport.
//! Required #869/SIM-07 lane; fixture bootstrap is not issue execution evidence.
use serde_json::{json, Value};
use std::{fs, path::Path, process::Output};
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use intent_fixture::{git, Fixture};

macro_rules! assert_same_inventory {
    ($before:expr, $after:expr $(, $reason:expr)? $(,)?) => {{
        let before=&$before; let after=$after;
        let changed=before.keys().chain(after.keys()).filter(|path|before.get(*path)!=after.get(*path)).collect::<std::collections::BTreeSet<_>>();
        assert!(changed.is_empty(),"fixture bytes changed at {changed:?} {}",stringify!($($reason)?));
    }};
}

fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "installed intent failed: {output:?}"
    );
    let result: Value = serde_json::from_slice(&output.stdout).expect("machine JSON stdout");
    assert!(
        result["envelope"].is_object(),
        "missing common envelope: {result}"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
    result
}

fn plan() -> Value {
    json!({"schema":"csdlc.v3.intent_plan.v1", "slug":"installed-intent-fixture",
      "cards":{"sip":{},"stp":{},"spp":{},"vpp":{},"srp":{},"sor":{}},
      "validators":[{"id":"fixture-proof", "program":"cargo",
        "args":["test", "--manifest-path", "fixture-proof/Cargo.toml", "--offline"], "success_marker":"test result: ok."}],
      "publication":{"base":"main","title":"Installed intent fixture", "body":"Closes #505", "draft":true}})
}

fn prepare(fixture: &mut Fixture) {
    let primary = fixture.root.clone();
    let input = fixture.write_json("plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
}

fn observation(fixture: &mut Fixture, cwd: &Path, route: &str) {
    let before = intent_fixture::inventory(&fixture.root);
    let result = success(fixture.run(cwd, &[route, "505"]));
    assert_eq!(result["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&fixture.root),
        "{route} changed fixture bytes"
    );
}

#[test]
fn installed_prepare_and_bind_from_unrelated_linked_checkout_resolve_primary_state() {
    let mut fixture = Fixture::new("linked-prepared-start");
    let primary = fixture.root.clone();
    let observer = primary.join("worktrees/observer");
    git(
        &primary,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "fixture-observer",
            observer.to_str().unwrap(),
        ],
    );
    let input = fixture.write_json("plan.json", &plan());
    success(fixture.run(
        &observer,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert!(primary
        .join(".git/csdlc-v3/local/issues/505/index.json")
        .is_file());
    assert!(!observer.join(".csdlc/issues/505/index.json").exists());
    success(fixture.run(&observer, &["bind", "505"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/505.json")).unwrap(),
    )
    .unwrap();
    assert!(Path::new(binding["worktree"].as_str().unwrap())
        .join(".git")
        .is_file());
    observation(&mut fixture, &observer, "status");
    observation(&mut fixture, &observer, "validate");
}

#[test]
fn installed_prepare_bind_edit_and_observations_use_canonical_context() {
    let mut fixture = Fixture::new("ordinary-local");
    let primary = fixture.root.clone();
    let input = fixture.write_json("plan.json", &plan());
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &primary,
            &[
                "prepare",
                "505",
                "--plan",
                input.to_str().unwrap(),
                "--preview",
                "plan"
            ]
        )
        .status
        .success());
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "forbidden prepare preview mutated state"
    );
    prepare(&mut fixture);
    observation(&mut fixture, &primary, "status");
    observation(&mut fixture, &primary, "validate");
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    assert!(linked.join(".git").is_file());
    for cwd in [&primary, &linked] {
        observation(&mut fixture, cwd, "status");
        observation(&mut fixture, cwd, "validate");
    }
    let changes = fixture.write_json("changes.json", &json!({"schema":"csdlc.v3.intent_changes.v1", "cards":{"sip":{"title":"Edited through ordinary intent"}}}));
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &linked,
            &[
                "edit",
                "505",
                "--changes",
                changes.to_str().unwrap(),
                "--preview",
                "plan"
            ]
        )
        .status
        .success());
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "forbidden edit preview mutated state"
    );
    let edited = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(edited["envelope"]["issue"]["number"], 505);
    assert_eq!(
        edited["envelope"]["issue_version"]["before"]["basis"],
        "observed_intent_snapshot"
    );
    assert_eq!(
        edited["envelope"]["issue_version"]["before"]["digest"],
        edited["intent_snapshot"]["version"]["digest"]
    );
    assert!(edited["envelope"]["issue_version"]["before"]["digest"]
        .as_str()
        .is_some_and(|digest| digest.len() == 64));
    assert_eq!(
        edited["envelope"]["issue_version"]["after"]["digest"],
        edited["result"]["digest"]
    );
    assert_ne!(
        edited["envelope"]["issue_version"]["after"]["digest"],
        edited["envelope"]["issue_version"]["before"]["digest"]
    );
    success(fixture.run(
        &primary,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert!(
        fs::read_to_string(linked.join(".csdlc/issues/505/cards/sip.md"))
            .unwrap()
            .contains("Edited through ordinary intent")
    );
    observation(&mut fixture, &linked, "validate");
}

#[test]
fn installed_intent_rejects_old_and_unsupported_content_without_preparation_effects() {
    for (label, input) in [
        ("old-schema", json!({"schema":"csdlc.v3.intent_plan.v0"})),
        ("unknown-fields", {
            let mut input = plan();
            input["authority_digest"] = json!("forged");
            input
        }),
    ] {
        let mut fixture = Fixture::new(label);
        let primary = fixture.root.clone();
        let path = fixture.write_json("plan.json", &input);
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(
            &primary,
            &["prepare", "505", "--plan", path.to_str().unwrap()],
        );
        assert!(
            !output.status.success(),
            "unsupported input executed: {output:?}"
        );
        let result: Value = serde_json::from_slice(&output.stdout).expect("typed JSON rejection");
        assert_ne!(result["envelope"]["status"], "completed");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        assert!(!primary
            .join(".git/csdlc-v3/local/issues/505/index.json")
            .exists());
    }
}

#[test]
fn installed_intent_rejects_foreign_repository_and_altered_authority_without_fallback() {
    for label in ["foreign-repository", "altered-authority"] {
        let mut fixture = Fixture::new(label);
        let primary = fixture.root.clone();
        if label == "foreign-repository" {
            prepare(&mut fixture);
            git(
                &primary,
                &[
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/other/repository.git",
                ],
            );
        } else {
            let selector = primary.join("csdlc-v3/operator/authority-selector.json");
            let mut value: Value = serde_json::from_slice(&fs::read(&selector).unwrap()).unwrap();
            value["receipt_digest"] = json!("0".repeat(64));
            fs::write(selector, serde_json::to_vec(&value).unwrap()).unwrap();
        }
        let input = fixture.write_json("plan.json", &plan());
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(
            &primary,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(
            !output.status.success(),
            "identity guard allowed execution: {output:?}"
        );
        let value: Value = serde_json::from_slice(&output.stdout).expect("machine-readable denial");
        assert!(
            value["envelope"]["findings"]
                .as_array()
                .is_some_and(|findings| !findings.is_empty()),
            "missing guard finding: {value}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        if label == "foreign-repository" {
            assert_eq!(
                value["envelope"]["reason_code"],
                "intent_issue_repository_mismatch"
            );
        } else {
            assert!(!primary
                .join(".git/csdlc-v3/local/issues/505/index.json")
                .exists());
        }
    }
}

#[test]
fn installed_unknown_intent_never_reports_success_or_creates_state() {
    let mut fixture = Fixture::new("unknown-intent");
    let primary = fixture.root.clone();
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&primary, &["unknown-intent", "505"]);
    assert!(!output.status.success());
    let value: Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable unknown command");
    assert_ne!(value["envelope"]["status"], "completed");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

fn linked_worktree(primary: &Path) -> std::path::PathBuf {
    git(primary, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(std::path::PathBuf::from)
        .find(|path| path != primary)
        .expect("registered linked worktree")
}

#[test]
fn installed_advanced_request_uses_same_writer_and_rejects_stale_snapshot() {
    let mut fixture = Fixture::new("advanced-snapshot");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let changes=fixture.write_json("changes.json",&json!({"schema":"csdlc.v3.intent_changes.v1","cards":{"sip":{"title":"Advanced intent execution"}}}));
    let before = intent_fixture::inventory(&primary);
    let emitted = success(fixture.run(
        &linked,
        &[
            "edit",
            "505",
            "--changes",
            changes.to_str().unwrap(),
            "--emit-request",
        ],
    ));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "request generation mutated issue state"
    );
    assert_eq!(emitted["request"]["schema"], "csdlc.v3.intent_request.v1");
    let mut foreign = emitted["request"].clone();
    foreign["snapshot"]["platform"] = json!("synthetic-foreign-platform");
    let foreign_path = fixture.write_json("foreign-platform.json", &foreign);
    let before = intent_fixture::inventory(&primary);
    let denied = fixture.run(
        &linked,
        &["edit", "--intent-request", foreign_path.to_str().unwrap()],
    );
    assert!(
        !denied.status.success(),
        "foreign-platform mutation snapshot admitted"
    );
    let denial: Value = serde_json::from_slice(&denied.stdout).unwrap();
    assert_eq!(
        denial["envelope"]["reason_code"],
        "intent_platform_not_admitted"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let request = fixture.write_json("advanced.json", &emitted["request"]);
    success(fixture.run(
        &linked,
        &["edit", "--intent-request", request.to_str().unwrap()],
    ));
    assert!(
        fs::read_to_string(linked.join(".csdlc/issues/505/cards/sip.md"))
            .unwrap()
            .contains("Advanced intent execution")
    );
    let before = intent_fixture::inventory(&primary);
    let stale = fixture.run(
        &linked,
        &["edit", "--intent-request", request.to_str().unwrap()],
    );
    assert!(
        !stale.status.success(),
        "stale serialized request silently refreshed: {stale:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_recovery_requires_fresh_preview_of_actual_interrupted_transaction() {
    let mut fixture = Fixture::new("recovery-preview");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let changes=fixture.write_json("changes.json",&json!({"schema":"csdlc.v3.intent_changes.v1","cards":{"sip":{"title":"Recovered ordinary edit"}}}));
    let crash = fixture.run_with_env(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[("CSDLC_V3_TEST_CRASH_POINT", "after_backup_rename")],
    );
    assert_eq!(
        crash.status.code(),
        Some(91),
        "did not reach actual transaction interruption: {crash:?}"
    );
    let before = intent_fixture::inventory(&primary);
    let missing = fixture.run(&linked, &["recover", "505", "--execute"]);
    assert!(!missing.status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "recovery preview performed repair"
    );
    let digest = preview["preview_digest"]
        .as_str()
        .expect("fresh preview digest");
    let wrong = fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", "stale-preview"],
    );
    assert!(!wrong.status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", digest],
    ));
    observation(&mut fixture, &linked, "validate");
    let before = intent_fixture::inventory(&primary);
    let consumed = fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", digest],
    );
    assert!(
        !consumed.status.success(),
        "consumed preview accepted for changed transaction state"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_proof_runs_real_validator_and_rejects_zero_test_success() {
    for zero_tests in [false, true] {
        let mut fixture = Fixture::new(if zero_tests {
            "zero-test-proof"
        } else {
            "real-validator-proof"
        });
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        if zero_tests {
            fs::write(
                linked.join("fixture-proof/src/lib.rs"),
                "pub const NO_TESTS: bool = true;\n",
            )
            .unwrap();
            git(&linked, &["add", "fixture-proof/src/lib.rs"]);
            git(
                &linked,
                &["commit", "--quiet", "-m", "fixture with no tests"],
            );
        }
        let target = linked.join("target/intent-validation");
        assert!(
            !target.exists(),
            "proof target already warm; measurement would be misleading"
        );
        observation(&mut fixture, &linked, "status");
        observation(&mut fixture, &linked, "validate");
        assert!(!target.exists(), "observation ran a validator");
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            target.exists(),
            "proof did not run the declared Cargo validator: {output:?}"
        );
        if zero_tests {
            assert!(
                !output.status.success(),
                "zero executed tests accepted as proof: {output:?}"
            );
            let value: Value =
                serde_json::from_slice(&output.stdout).expect("zero-test rejection result");
            assert_ne!(value["envelope"]["status"], "completed");
        } else {
            let proof = success(output);
            assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
            let repeated = success(fixture.run(&primary, &["proof", "505"]));
            assert_eq!(repeated["proof"]["validators"][0]["tests_passed"], 1);
            fs::write(linked.join("tracked"), "changed after proof\n").unwrap();
            let before = intent_fixture::inventory(&primary);
            let status = success(fixture.run(&linked, &["status", "505"]));
            assert_eq!(
                status["evidence"]["proof_current"], false,
                "dirty source retained current proof recommendation"
            );
            assert_same_inventory!(
                before,
                intent_fixture::inventory(&primary),
                "status repaired or reran stale proof"
            );
        }
    }
}

#[test]
fn installed_issue_mutations_execute_exact_targets_and_preserve_omitted_metadata() {
    for linked_invocation in [false, true] {
        for operation in [
            json!({"action":"issue_create","title":"New synthetic issue","body":"Synthetic new issue","labels":[],"assignees":[],"milestone":null}),
            json!({"action":"issue_comment","body":"Synthetic comment"}),
            json!({"action":"issue_edit","title":"Edited remote title","body":null}),
            json!({"action":"issue_close","rationale":"Duplicate fixture scope","current_body":"Fixture issue","disposition":"duplicate","duplicate_of":504,"github_state_reason":"not_planned"}),
        ] {
            let name = operation["action"].as_str().unwrap();
            let mut fixture = Fixture::new(&format!("{name}-{linked_invocation}"));
            fixture.enable_issue_transport();
            let primary = fixture.root.clone();
            prepare(&mut fixture);
            success(fixture.run(&primary, &["bind", "505"]));
            let linked = linked_worktree(&primary);
            let cwd = if linked_invocation { &linked } else { &primary };
            let path = fixture.write_json("operation.json", &operation);
            let before = intent_fixture::inventory(&primary);
            let preview = success(fixture.run(
                cwd,
                &["github-issue", "505", "--operation", path.to_str().unwrap()],
            ));
            assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let wrong_family = fixture.run(
                cwd,
                &[
                    "github-pr",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            );
            assert!(
                !wrong_family.status.success(),
                "wrong family accepted: {wrong_family:?}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            success(fixture.run(
                cwd,
                &[
                    "github-issue",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            ));
            assert_eq!(
                fixture.remote_effects(),
                1,
                "operation did not produce exactly one remote effect"
            );
            if name == "issue_edit" {
                let remote = fixture.remote_issue();
                assert_eq!(remote["title"], "Edited remote title");
                assert_eq!(remote["labels"], json!(["retained"]));
                assert_eq!(remote["assignees"], json!(["fixture-assignee"]));
                assert_eq!(remote["milestone"]["number"], 4);
                assert!(remote["body"]
                    .as_str()
                    .unwrap()
                    .starts_with("Fixture issue"));
            }
            if name == "issue_close" {
                let remote = fixture.remote_issue();
                assert_eq!(remote["state"], "closed");
                assert_eq!(remote["state_reason"], "not_planned");
                assert!(remote["body"].as_str().unwrap().contains("504"));
            }
        }
    }
}

fn external_review(linked: &Path) -> Value {
    use csdlc_v3::commands::remote::{typed_review_receipt_payload_digest, TypedReviewReceipt};
    let head = git(linked, &["rev-parse", "HEAD"]);
    let proof_path = ".csdlc/evidence/505/intent-proof.json";
    let proof = fs::read(linked.join(proof_path)).expect("real installed proof receipt");
    let receipt:TypedReviewReceipt=serde_json::from_value(json!({
        "schema":"csdlc.v3.typed_review_receipt.v1","repository":"agent-logic/agent-design-language","issue":505,
        "implementer":"synthetic-fixture-author","reviewer":"synthetic-independent-fixture-reviewer",
        "reviewed_revision":head,"expected_head_sha":head,
        "evidence_digest":blake3::hash(b"Synthetic external fixture review, not review of #869 source").to_hex().to_string(),
        "publication_linkage":{"repository":"agent-logic/agent-design-language","issue":505,"mode":"closing"}
    })).unwrap();
    json!({"receipt_digest":typed_review_receipt_payload_digest(&receipt),"receipt":receipt,"proof_path":proof_path,"proof_digest":blake3::hash(&proof).to_hex().to_string()})
}

#[test]
fn installed_review_consumes_external_exact_head_evidence_and_rejects_wrong_identity() {
    use csdlc_v3::commands::remote::{typed_review_receipt_payload_digest, TypedReviewReceipt};
    for linked_invocation in [false, true] {
        let mut fixture = Fixture::new(&format!("independent-review-{linked_invocation}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        success(fixture.run(&linked, &["proof", "505"]));
        let valid = external_review(&linked);
        let cwd = if linked_invocation { &linked } else { &primary };
        for defect in [
            "same-principal",
            "stale-head",
            "wrong-repository",
            "wrong-proof-digest",
        ] {
            let mut altered = valid.clone();
            match defect {
                "same-principal" => {
                    altered["receipt"]["reviewer"] = altered["receipt"]["implementer"].clone()
                }
                "stale-head" => altered["receipt"]["reviewed_revision"] = json!("0".repeat(40)),
                "wrong-repository" => altered["receipt"]["repository"] = json!("other/repository"),
                "wrong-proof-digest" => altered["proof_digest"] = json!("0".repeat(64)),
                _ => unreachable!(),
            }
            let receipt: TypedReviewReceipt =
                serde_json::from_value(altered["receipt"].clone()).unwrap();
            altered["receipt_digest"] = json!(typed_review_receipt_payload_digest(&receipt));
            let path = fixture.write_json("review.json", &altered);
            let before = intent_fixture::inventory(&primary);
            let output = fixture.run(
                cwd,
                &["review", "505", "--evidence", path.to_str().unwrap()],
            );
            assert!(
                !output.status.success(),
                "invalid review {defect} accepted: {output:?}"
            );
            assert_same_inventory!(
                before,
                intent_fixture::inventory(&primary),
                "rejected review persisted evidence"
            );
        }
        let path = fixture.write_json("review.json", &valid);
        let accepted = success(fixture.run(
            cwd,
            &["review", "505", "--evidence", path.to_str().unwrap()],
        ));
        let reference = accepted["review_receipt_path"]
            .as_str()
            .expect("persisted review reference");
        assert!(linked.join(reference).is_file());
        let before = intent_fixture::inventory(&primary);
        let replay = success(fixture.run(
            cwd,
            &["review", "505", "--evidence", path.to_str().unwrap()],
        ));
        assert_eq!(replay["envelope"]["status"], "expected_noop");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

fn reviewed_fixture(label: &str) -> (Fixture, std::path::PathBuf) {
    let mut fixture = Fixture::new(label);
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["proof", "505"]));
    let review = fixture.write_json("review.json", &external_review(&linked));
    success(fixture.run(
        &linked,
        &["review", "505", "--evidence", review.to_str().unwrap()],
    ));
    fixture.enable_pr_transport(&linked);
    (fixture, linked)
}

#[test]
fn installed_publication_pr_mutations_and_uncertain_ready_retry_use_native_receipts() {
    for ordinary_publication in [false, true] {
        let (mut fixture, linked) =
            reviewed_fixture(&format!("publication-{ordinary_publication}"));
        let primary = fixture.root.clone();
        let cwd = if ordinary_publication {
            &primary
        } else {
            &linked
        };
        for route in ["clean", "finish"] {
            let before = intent_fixture::inventory(&primary);
            let absent = fixture.run(cwd, &[route, "505"]);
            assert!(
                !absent.status.success(),
                "{route} succeeded without terminal target: {absent:?}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
        if ordinary_publication {
            fixture.remote_flag("drop-publication-readback", true);
            let failed_readback = fixture.run(cwd, &["publish", "505"]);
            assert!(!failed_readback.status.success());
            let payload: Value = serde_json::from_slice(&failed_readback.stdout).unwrap();
            assert_eq!(payload["status"], "recovery_required");
            assert!(
                payload["findings"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|finding| {
                        let code = finding["code"].as_str().unwrap();
                        !code.contains("recovery_required")
                            && !code.contains("reconciliation_required")
                            && !code.contains("reconciliation_pending")
                    }),
                "fixture must exercise explicit owner status rather than code-substring inference"
            );
            assert_eq!(payload["envelope"]["status"], "recovery_required");
            assert_eq!(payload["envelope"]["process_status"], "failed");
            assert_eq!(payload["envelope"]["effects"]["outcome"], "performed");
            assert_eq!(fixture.remote_effects(), 1);
            fixture.remote_flag("drop-publication-readback", false);
            fixture.remote_flag("drop-readback", false);
            observation(&mut fixture, cwd, "pr-state");
            assert_eq!(
                fixture.remote_effects(),
                1,
                "authenticated observation duplicated publication effect"
            );
        } else {
            let operation = json!({"action":"pull_request_create","base":"main","head":git(&linked,&["symbolic-ref","--short","HEAD"]),"title":"Installed intent fixture","body":"Closes #505","draft":true});
            let mut invalid = operation.clone();
            invalid["head"] = json!("codex/foreign");
            let path = fixture.write_json("operation.json", &invalid);
            let before = intent_fixture::inventory(&primary);
            assert!(!fixture
                .run(
                    cwd,
                    &[
                        "github-pr",
                        "505",
                        "--operation",
                        path.to_str().unwrap(),
                        "--execute"
                    ]
                )
                .status
                .success());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let path = fixture.write_json("operation.json", &operation);
            success(fixture.run(
                cwd,
                &[
                    "github-pr",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            ));
        }
        assert_eq!(fixture.remote_effects(), 1);
        observation(&mut fixture, cwd, "pr-state");
        let published = fixture.remote_pr();
        assert_eq!(published["base"]["ref"], "main");
        assert_eq!(
            published["head"]["sha"],
            git(&linked, &["rev-parse", "HEAD"])
        );
        let mut wrong_head = published.clone();
        wrong_head["head"]["sha"] = json!("0".repeat(40));
        fixture.set_remote_pr(&wrong_head);
        let before = intent_fixture::inventory(&primary);
        assert!(
            !fixture.run(cwd, &["publish", "505"]).status.success(),
            "publication admitted wrong remote HEAD"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        fixture.set_remote_pr(&published);
        let operation=fixture.write_json("operation.json",&json!({"action":"pull_request_update","title":"Updated fixture PR","body":"Closes #505"}));
        success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                operation.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(fixture.remote_effects(), 2);
        assert_eq!(fixture.remote_pr()["title"], "Updated fixture PR");
        let current_remote = fixture.remote_pr();
        for field in ["head", "base"] {
            let mut foreign = current_remote.clone();
            if field == "head" {
                foreign["head"]["sha"] = json!("0".repeat(40));
            } else {
                foreign["base"]["ref"] = json!("foreign-base");
            }
            fixture.set_remote_pr(&foreign);
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture
                    .run(
                        cwd,
                        &[
                            "github-pr",
                            "505",
                            "--operation",
                            operation.to_str().unwrap(),
                            "--execute"
                        ]
                    )
                    .status
                    .success(),
                "explicit PR update admitted foreign {field}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            assert_eq!(fixture.remote_effects(), 2);
        }
        fixture.set_remote_pr(&current_remote);
        let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
        fixture.remote_flag("uncertain-response", true);
        let uncertain = fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        );
        assert!(
            !uncertain.status.success(),
            "uncertain ready outcome reported completed"
        );
        let unknown: Value = serde_json::from_slice(&uncertain.stdout).unwrap();
        assert_eq!(unknown["envelope"]["status"], "recovery_required");
        assert_eq!(unknown["envelope"]["effects"]["outcome"], "unknown");
        assert_eq!(fixture.remote_effects(), 3);
        assert_eq!(fixture.remote_pr()["draft"], false);
        fixture.remote_flag("uncertain-response", false);
        fixture.remote_flag("drop-readback", false);
        if ordinary_publication {
            let before = intent_fixture::inventory(&primary);
            let pending = success(fixture.run(cwd, &["status", "505"]));
            assert_eq!(pending["allowed_next"], json!(["recover"]));
            assert!(!pending["pending_remote"].as_array().unwrap().is_empty());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let preview = success(fixture.run(cwd, &["recover", "505"]));
            let token = preview["preview_digest"].as_str().unwrap();
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            for extra in [vec!["--execute"], vec!["--execute", "--preview", "stale"]] {
                let mut args = vec!["recover", "505"];
                args.extend(extra);
                assert!(!fixture.run(cwd, &args).status.success());
                assert_same_inventory!(before, intent_fixture::inventory(&primary));
            }
            success(fixture.run(cwd, &["recover", "505", "--execute", "--preview", token]));
            assert_eq!(
                fixture.remote_effects(),
                3,
                "ordinary recovery duplicated observed ready mutation"
            );
        }
        success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(
            fixture.remote_effects(),
            3,
            "reconciliation duplicated ready mutation"
        );
        let before = intent_fixture::inventory(&primary);
        let noop = success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(noop["envelope"]["status"], "expected_noop");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

#[test]
fn installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue() {
    let (mut fixture, linked) = reviewed_fixture("merge-finish-clean");
    let primary = fixture.root.clone();
    success(fixture.run(&primary, &["publish", "505"]));
    let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let missing_approval = fixture.write_json(
        "merge.json",
        &json!({"action":"pull_request_merge","base":"main","method":"merge"}),
    );
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &primary,
            &[
                "github-pr",
                "505",
                "--operation",
                missing_approval.to_str().unwrap(),
                "--execute"
            ]
        )
        .status
        .success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let merge=fixture.write_json("merge.json",&json!({"action":"pull_request_merge","base":"main","method":"merge","operator_approval":"synthetic operator authorizes only fixture PR639 exact candidate merge"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(fixture.remote_effects(), 3);
    assert_eq!(fixture.remote_pr()["merged"], true);
    assert_eq!(fixture.remote_issue()["state"], "closed");
    let before = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(replay["envelope"]["status"], "expected_noop");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let preview = success(fixture.run(&primary, &["finish", "505", "--preview", "plan"]));
    assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(&linked, &["finish", "505"]));
    let terminal = primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json");
    assert!(terminal.is_file(), "native terminal evidence missing");
    for residue in ["foreign-note.txt", "tracked"] {
        let path = linked.join(residue);
        let previous = fs::read(&path).ok();
        fs::write(&path, b"retained dirty fixture bytes").unwrap();
        let before = intent_fixture::inventory(&primary);
        assert!(
            !fixture.run(&primary, &["clean", "505"]).status.success(),
            "unsafe residue permitted cleanup: {residue}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        if let Some(bytes) = previous {
            fs::write(path, bytes).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }
    let before = intent_fixture::inventory(&primary);
    let clean = success(fixture.run(&primary, &["clean", "505"]));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "cleanup preview archived or changed source files"
    );
    let token = clean["preview_token"]
        .as_str()
        .expect("cleanup preview token");
    let card = linked.join(".csdlc/issues/505/cards/sip.md");
    let original = fs::read(&card).unwrap();
    fs::write(
        &card,
        [original.as_slice(), b"\nchanged after preview\n"].concat(),
    )
    .unwrap();
    let changed = intent_fixture::inventory(&primary);
    assert!(
        !fixture
            .run(&primary, &["clean", "505", "--execute", "--preview", token])
            .status
            .success(),
        "stale generated-residue preview admitted"
    );
    assert_same_inventory!(changed, intent_fixture::inventory(&primary));
    fs::write(&card, &original).unwrap();
    let archive_parent = primary.join(".git/csdlc-v3/local/archives");
    fs::write(
        &archive_parent,
        b"fixture blocks archive directory creation",
    )
    .unwrap();
    let originals = intent_fixture::inventory(&linked);
    let archive_failure = fixture.run(&primary, &["clean", "505", "--execute", "--preview", token]);
    assert!(
        !archive_failure.status.success(),
        "archive destination failure reported cleanup success"
    );
    let failed_state = intent_fixture::inventory(&linked);
    for (path, value) in &originals {
        assert_eq!(
            failed_state.get(path),
            Some(value),
            "archive failure changed original {path:?}"
        );
    }
    for added in failed_state
        .keys()
        .filter(|path| !originals.contains_key(*path))
    {
        assert!(
            matches!(
                added.to_str(),
                Some(".csdlc/locks" | ".csdlc/locks/505.lock")
            ),
            "unexpected archive-failure write {added:?}"
        );
    }
    assert!(fs::read(linked.join(".csdlc/locks/505.lock"))
        .unwrap()
        .is_empty());
    let failure: Value = serde_json::from_slice(&archive_failure.stdout).unwrap();
    assert_ne!(
        failure["envelope"]["effects"]["outcome"], "none",
        "lock-creating failure reported no effect"
    );
    let originals = failed_state;
    fs::remove_file(&archive_parent).unwrap();
    let interrupted = fixture.interrupt_clean_after_archive(
        &primary,
        &["clean", "505", "--execute", "--preview", token],
    );
    assert!(!interrupted.status.success());
    assert_same_inventory!(
        originals,
        intent_fixture::inventory(&linked),
        "interrupted archive deleted original bytes before durable boundary"
    );
    assert!(fs::read_dir(&archive_parent).unwrap().any(|entry| entry
        .unwrap()
        .path()
        .join("manifest.json")
        .is_file()));

    let interrupted = fixture.interrupt_clean_after_index_removal(
        &primary,
        &["clean", "505", "--execute", "--preview", token],
    );
    assert!(!interrupted.status.success());
    assert!(
        linked.exists(),
        "interruption occurred after full removal rather than before it"
    );
    assert!(
        !linked.join(".csdlc/issues/505/index.json").exists(),
        "interruption occurred before source index removal"
    );
    assert!(git(&primary, &["worktree", "list", "--porcelain"]).contains(linked.to_str().unwrap()));
    let partial = intent_fixture::inventory(&primary);
    assert!(
        !fixture.run(&primary, &["status", "505"]).status.success(),
        "partial cleanup evidence authorized ordinary issue work"
    );
    assert_same_inventory!(partial, intent_fixture::inventory(&primary));
    let continuation = success(fixture.run(&primary, &["clean", "505"]));
    assert_ne!(
        continuation["envelope"]["status"], "expected_noop",
        "registered partial removal misreported as complete"
    );
    assert_same_inventory!(partial, intent_fixture::inventory(&primary));
    let continued_token = continuation["preview_token"]
        .as_str()
        .expect("fresh continuation preview");
    success(fixture.run(
        &linked,
        &["clean", "505", "--execute", "--preview", continued_token],
    ));
    assert!(!linked.exists(), "exact bound worktree not removed");
    assert_eq!(
        git(&primary, &["worktree", "list", "--porcelain"])
            .lines()
            .filter(|line| line.starts_with("worktree "))
            .count(),
        1
    );
    assert!(terminal.is_file(), "cleanup lost terminal authority");
    let archived = fs::read_dir(primary.join(".git/csdlc-v3/local/archives"))
        .expect("durable cleanup archive")
        .collect::<Vec<_>>();
    assert!(
        !archived.is_empty(),
        "native cleanup omitted generated residue archive"
    );
    let before = intent_fixture::inventory(&primary);
    let noop = success(fixture.run(&primary, &["clean", "505"]));
    assert_eq!(noop["envelope"]["status"], "expected_noop");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_generated_request_roundtrips_exact_stdout_without_manual_extraction() {
    let mut fixture = Fixture::new("generated-wrapper");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let emitted = fixture.run(&primary, &["bind", "505", "--emit-request"]);
    let bytes = emitted.stdout.clone();
    success(emitted);
    let path = primary.join(".git/installed-candidate/generated-request.json");
    fs::write(&path, &bytes).unwrap();
    success(fixture.run(
        &primary,
        &["bind", "--intent-request", path.to_str().unwrap()],
    ));
    assert!(linked_worktree(&primary)
        .join(".csdlc/issues/505/index.json")
        .is_file());
}

#[test]
fn installed_issue_edit_metadata_operations_are_complete_and_explicit() {
    for (name, fields, expected_labels, expected_assignees, expected_milestone) in [
        (
            "add",
            json!({"labels":{"operation":"add","names":["new"]}}),
            json!(["retained", "new"]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "remove",
            json!({"labels":{"operation":"remove","names":["retained"]}}),
            json!([]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "replace",
            json!({"labels":{"operation":"replace","names":["replacement"]}}),
            json!(["replacement"]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "assignees",
            json!({"assignees":["new-assignee"]}),
            json!(["retained"]),
            json!(["new-assignee"]),
            json!({"number":4}),
        ),
        (
            "milestone-set",
            json!({"milestone":{"operation":"set","number":7}}),
            json!(["retained"]),
            json!(["fixture-assignee"]),
            json!({"number":7}),
        ),
        (
            "milestone-clear",
            json!({"milestone":{"operation":"clear"}}),
            json!(["retained"]),
            json!(["fixture-assignee"]),
            Value::Null,
        ),
    ] {
        let mut fixture = Fixture::new(&format!("metadata-{name}"));
        fixture.enable_issue_transport();
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        let mut operation = json!({"action":"issue_edit","title":null,"body":null});
        operation
            .as_object_mut()
            .unwrap()
            .extend(fields.as_object().unwrap().clone());
        let path = fixture.write_json("operation.json", &operation);
        success(fixture.run(
            &linked,
            &[
                "github-issue",
                "505",
                "--operation",
                path.to_str().unwrap(),
                "--execute",
            ],
        ));
        let remote = fixture.remote_issue();
        assert_eq!(remote["labels"], expected_labels);
        assert_eq!(remote["assignees"], expected_assignees);
        assert_eq!(remote["milestone"], expected_milestone);
        assert_eq!(remote["title"], "Installed intent fixture");
        assert_eq!(fixture.remote_effects(), 1);
        for field in ["labels", "assignees", "milestone"] {
            let mut invalid = json!({"action":"issue_edit","title":"invalid metadata","body":null});
            invalid[field] = Value::Null;
            let path = fixture.write_json("operation.json", &invalid);
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture
                    .run(
                        &linked,
                        &[
                            "github-issue",
                            "505",
                            "--operation",
                            path.to_str().unwrap(),
                            "--execute"
                        ]
                    )
                    .status
                    .success(),
                "null {field} admitted as omitted metadata"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
    }
}

#[test]
fn installed_status_preserves_native_recommendations_and_explicit_human_decisions() {
    let mut fixture = Fixture::new("status-decisions");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let before = intent_fixture::inventory(&primary);
    let unknown = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(unknown["operator_decisions"]["required"], true);
    assert_eq!(unknown["shepherd"]["routing"]["state"], "operator_required");
    assert_eq!(unknown["evidence"]["validators_run"], false);
    assert!(unknown["eligibility"].is_object());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    for (dependencies, retry, schedule, shepherd) in [
        (true, false, "ready", "waiting"),
        (false, false, "blocked", "waiting"),
        (true, true, "ready", "retryable"),
    ] {
        let decisions = fixture.write_json(
            "decisions.json",
            &json!({
                "design_ready":true,"dependencies_ready":dependencies,"budget_available":true,
                "retryable_failure":retry
            }),
        );
        let before = intent_fixture::inventory(&primary);
        let output = success(fixture.run(
            &primary,
            &["status", "505", "--decisions", decisions.to_str().unwrap()],
        ));
        assert_eq!(output["operator_decisions"]["required"], false);
        assert_eq!(output["scheduling"]["routing"]["state"], schedule);
        assert_eq!(output["shepherd"]["routing"]["state"], shepherd);
        assert_eq!(output["evidence"]["validators_run"], false);
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

#[test]
fn installed_no_pr_finish_requires_closed_authenticated_issue_and_retains_disposition() {
    let mut fixture = Fixture::new("no-pr-disposition");
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let disposition = fixture.write_json(
        "disposition.json",
        &json!({
            "disposition":"retired_without_execution", "operator":"synthetic-fixture-operator",
            "rationale":"Synthetic administrative retirement, not implementation delivery",
            "evidence_refs":["fixture:operator-retirement-decision"]
        }),
    );
    let args = [
        "finish",
        "505",
        "--disposition",
        disposition.to_str().unwrap(),
    ];
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture.run(&primary, &args).status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
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
    let preview = success(fixture.run(
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
    assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(&primary, &args));
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert!(receipt["pull_request"].is_null());
    assert_eq!(
        receipt["no_pr_closeout"]["disposition"],
        "retired_without_execution"
    );
    assert_eq!(
        receipt["no_pr_closeout"]["expected_issue_closed_at"],
        remote["closed_at"]
    );
    let clean = success(fixture.run(&primary, &["clean", "505"]));
    let token = clean["preview_token"].as_str().unwrap();
    success(fixture.run(&primary, &["clean", "505", "--execute", "--preview", token]));
    assert!(!linked.exists());
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn installed_remote_recover_retries_once_only_after_authenticated_absence() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-absence");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
    fixture.remote_flag("reject-ready-before-effect", true);
    assert!(!fixture
        .run(
            &linked,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute"
            ]
        )
        .status
        .success());
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["draft"], true);
    fixture.remote_flag("reject-ready-before-effect", false);
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 2);
    assert_eq!(fixture.remote_pr()["draft"], false);
    let before = intent_fixture::inventory(&primary);
    let settled = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(settled["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert_eq!(fixture.remote_effects(), 2);
}

#[test]
fn installed_proof_refuses_ignored_configuration_outside_declared_caches() {
    let mut fixture = Fixture::new("ignored-configuration");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let ignore = linked.join(".gitignore");
    let mut rules = fs::read_to_string(&ignore).unwrap();
    rules.push_str("\n/undeclared-config.json\n");
    fs::write(ignore, rules).unwrap();
    git(&linked, &["add", ".gitignore"]);
    git(
        &linked,
        &["commit", "-m", "Declare ignored fixture configuration"],
    );
    fs::write(
        linked.join("undeclared-config.json"),
        b"{\"undeclared_input\":true}",
    )
    .unwrap();
    let before = intent_fixture::inventory(&primary);
    let refused = fixture.run(&linked, &["proof", "505"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stdout).contains("intent_candidate_ignored_source"));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
}

#[test]
fn installed_proof_retains_actual_validator_outcome_when_source_changes_during_execution() {
    let mut fixture = Fixture::new("validator-mutates-source");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn actual_test_changes_declared_source() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tracked");
    assert_eq!(std::fs::read_to_string(&source).unwrap(), "fixture\n");
    std::fs::write(source, "changed by actual validator\n").unwrap();
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &["commit", "-m", "Install source-mutating fixture validator"],
    );
    let failed = fixture.run(&linked, &["proof", "505"]);
    assert!(!failed.status.success());
    let result: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(result["proof"]["status"], "failed");
    assert_eq!(result["proof"]["inputs_unchanged"], false);
    assert_eq!(result["proof"]["validators"][0]["tests_passed"], 1);
    assert_eq!(
        fs::read_to_string(linked.join("tracked")).unwrap(),
        "changed by actual validator\n"
    );
    assert!(result["proof"]["input_revalidation"].is_object());
    assert_eq!(result["evidence_persisted"], true);
    let retained: Value = serde_json::from_slice(
        &fs::read(linked.join(".csdlc/evidence/505/intent-proof.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(retained["validators"], result["proof"]["validators"]);
    assert_eq!(retained["status"], "failed");
}

#[test]
fn installed_bind_and_validate_refuse_malformed_bound_state_without_repair() {
    let mut fixture = Fixture::new("malformed-bound-observation");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join(".csdlc/issues/505/index.json"),
        b"{malformed native state",
    )
    .unwrap();
    for cwd in [&primary, &linked] {
        for route in ["bind", "validate"] {
            let before = intent_fixture::inventory(&primary);
            assert!(!fixture.run(cwd, &[route, "505"]).status.success());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
    }
}

#[test]
fn installed_publication_plan_tampering_invalidates_ordinary_and_emitted_requests() {
    for (field, value) in [
        ("title", json!("Unreviewed title")),
        ("body", json!("Unreviewed body\n\nCloses #505")),
        ("base", json!("unreviewed-base")),
        ("draft", json!(false)),
    ] {
        let (mut fixture, linked) = reviewed_fixture(&format!("plan-tampering-{field}"));
        let primary = fixture.root.clone();
        let emitted = fixture.run(&linked, &["publish", "505", "--emit-request"]);
        assert!(emitted.status.success());
        let request = fixture.write_json(
            "emitted-publish.json",
            &serde_json::from_slice::<Value>(&emitted.stdout).unwrap(),
        );
        let plan_path = linked.join(".csdlc/issues/505/intent-plan.json");
        let index = fs::read(linked.join(".csdlc/issues/505/index.json")).unwrap();
        let mut changed: Value = serde_json::from_slice(&fs::read(&plan_path).unwrap()).unwrap();
        changed["publication"][field] = value;
        fs::write(plan_path, serde_json::to_vec(&changed).unwrap()).unwrap();
        for args in [
            vec!["publish", "505"],
            vec!["publish", "--intent-request", request.to_str().unwrap()],
        ] {
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture.run(&primary, &args).status.success(),
                "unreviewed publication {field} admitted"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            assert_eq!(fixture.remote_effects(), 0);
            assert_eq!(
                fs::read(linked.join(".csdlc/issues/505/index.json")).unwrap(),
                index
            );
        }
    }
}

#[test]
fn installed_proof_refuses_manifest_and_source_hidden_inside_excluded_evidence() {
    for hidden_manifest in [true, false] {
        let mut fixture = Fixture::new(if hidden_manifest {
            "excluded-manifest"
        } else {
            "excluded-source"
        });
        let primary = fixture.root.clone();
        let mut requested = plan();
        if hidden_manifest {
            requested["validators"][0]["args"][2] = json!(".csdlc/probe/Cargo.toml");
        }
        let plan_file = fixture.write_json("plan.json", &requested);
        success(fixture.run(
            &primary,
            &["prepare", "505", "--plan", plan_file.to_str().unwrap()],
        ));
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
        fs::write(
            linked.join(".csdlc/probe/lib.rs"),
            "#[test] fn hidden_test() { assert_eq!(2 + 2, 4); }\n",
        )
        .unwrap();
        let manifest = if hidden_manifest {
            linked.join(".csdlc/probe/Cargo.toml")
        } else {
            linked.join("fixture-proof/Cargo.toml")
        };
        let libpath = if hidden_manifest {
            "lib.rs"
        } else {
            "../.csdlc/probe/lib.rs"
        };
        fs::write(manifest, format!("[package]\nname=\"fixture-proof\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[lib]\npath=\"{libpath}\"\n")).unwrap();
        if !hidden_manifest {
            git(&linked, &["add", "fixture-proof/Cargo.toml"]);
            git(
                &linked,
                &[
                    "commit",
                    "-m",
                    "Track manifest pointing to excluded fixture source",
                ],
            );
        }
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            !output.status.success(),
            "excluded validator input yielded accepted proof"
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("intent_validator_input_not_tracked"),
            "unexpected validator admission refusal: {output:?}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        assert!(!linked.join("target/intent-validation").exists());
    }
}

#[test]
fn installed_proof_retains_failure_for_compiler_discovered_excluded_input() {
    let mut fixture = Fixture::new("compiler-excluded-input");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
    fs::write(
        linked.join(".csdlc/probe/input.txt"),
        "undeclared compile input\n",
    )
    .unwrap();
    fs::write(linked.join("fixture-proof/src/lib.rs"), "#[test]\nfn reads_excluded_compile_input() { assert_eq!(include_str!(\"../../.csdlc/probe/input.txt\"), \"undeclared compile input\\n\"); }\n").unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &[
            "commit",
            "-m",
            "Track compiler-discovered fixture dependency",
        ],
    );
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "compiler-discovered excluded input yielded current proof"
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["proof"]["status"], "failed");
    assert_eq!(
        value["proof"]["input_revalidation"]["finding"],
        "intent_validator_compiler_input_not_tracked"
    );
    assert_eq!(value["proof"]["validators"][0]["tests_passed"], 1);
    assert_eq!(value["evidence_persisted"], true);
    let before = intent_fixture::inventory(&primary);
    let status = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(status["evidence"]["proof_current"], false);
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_proof_retains_useful_bounded_failure_diagnostics_without_sensitive_lines() {
    let mut fixture = Fixture::new("validator-diagnostics");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn diagnostic_failure() {
    eprintln!("authorization: synthetic-private-diagnostic");
    panic!("fixture assertion explains the failure");
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &["commit", "-m", "Track bounded failure diagnostic fixture"],
    );
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let validator = &value["proof"]["validators"][0];
    assert_eq!(validator["passed"], false);
    assert_eq!(validator["tests_failed"], 1);
    let diagnostic = &validator["stdout_evidence"];
    assert_eq!(diagnostic["lossless"], false);
    assert!(diagnostic["excerpt"]
        .as_str()
        .unwrap()
        .contains("fixture assertion explains the failure"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("synthetic-private-diagnostic"));
    assert!(diagnostic["redacted_lines"].as_u64().unwrap() > 0);
    assert_eq!(validator["stdout_digest"].as_str().unwrap().len(), 64);
}

#[test]
fn installed_proof_accounts_for_build_scripts_and_nested_automatic_targets() {
    for (name, relative, include_path, build_script) in [
        (
            "build-script",
            "fixture-proof/build.rs",
            "../.csdlc/probe/hidden.rs",
            true,
        ),
        (
            "nested-test",
            "fixture-proof/tests/fixture_nested/main.rs",
            "../../../.csdlc/probe/hidden.rs",
            false,
        ),
        (
            "nested-bin",
            "fixture-proof/src/bin/fixture_nested/main.rs",
            "../../../../.csdlc/probe/hidden.rs",
            false,
        ),
    ] {
        let mut fixture = Fixture::new(&format!("compiler-{name}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
        fs::write(
            linked.join(".csdlc/probe/hidden.rs"),
            "const EXCLUDED_INPUT: u64 = 4;\n",
        )
        .unwrap();
        let source = linked.join(relative);
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        let body = if build_script {
            format!(
                "include!(\"{include_path}\");\nfn main() {{ assert_eq!(EXCLUDED_INPUT, 4); }}\n"
            )
        } else {
            format!("include!(\"{include_path}\");\nfn main() {{}}\n#[test] fn nested_reads_excluded_input() {{ assert_eq!(EXCLUDED_INPUT, 4); }}\n")
        };
        fs::write(source, body).unwrap();
        git(&linked, &["add", relative]);
        git(
            &linked,
            &[
                "commit",
                "-m",
                "Track compiler target with excluded include",
            ],
        );
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            !output.status.success(),
            "{name} excluded dependency yielded accepted proof"
        );
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["proof"]["status"], "failed", "{name}: {value}");
        assert_eq!(
            value["proof"]["input_revalidation"]["finding"],
            "intent_validator_compiler_input_not_tracked",
            "{name}: {value}"
        );
        assert!(
            value["proof"]["validators"][0]["tests_passed"]
                .as_u64()
                .unwrap()
                >= 1,
            "normal tracked library must actually execute"
        );
        assert_eq!(value["evidence_persisted"], true);
        let retained: Value = serde_json::from_slice(
            &fs::read(linked.join(".csdlc/evidence/505/intent-proof.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(retained["validators"], value["proof"]["validators"]);
        assert_eq!(retained["status"], "failed");
    }
}

#[test]
fn installed_proof_refuses_external_workspace_inheritance_and_patch_inputs() {
    for patched in [false, true] {
        let mut fixture = Fixture::new(if patched {
            "external-patch"
        } else {
            "external-inherited"
        });
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        let external = linked.parent().unwrap().join("external-helper");
        fs::create_dir_all(external.join("src")).unwrap();
        fs::write(
            external.join("Cargo.toml"),
            "[package]\nname='external-helper'\nversion='0.1.0'\nedition='2021'\n",
        )
        .unwrap();
        fs::write(external.join("src/lib.rs"), "pub fn value() -> u8 { 1 }\n").unwrap();
        let workspace = if patched {
            "[workspace]\nmembers=['fixture-proof']\n[patch.crates-io]\nexternal-helper={path='../external-helper'}\n"
        } else {
            "[workspace]\nmembers=['fixture-proof']\n[workspace.dependencies]\nexternal-helper={path='../external-helper'}\n"
        };
        fs::write(linked.join("Cargo.toml"), workspace).unwrap();
        let manifest = linked.join("fixture-proof/Cargo.toml");
        let mut content = fs::read_to_string(&manifest).unwrap();
        content.push_str(if patched {
            "\n[dependencies]\nexternal-helper='0.1.0'\n"
        } else {
            "\n[dependencies]\nexternal-helper={workspace=true}\n"
        });
        fs::write(manifest, content).unwrap();
        git(&linked, &["add", "Cargo.toml", "fixture-proof/Cargo.toml"]);
        git(
            &linked,
            &["commit", "-m", "Declare external workspace validator input"],
        );
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            value
                .to_string()
                .contains("intent_validator_input_outside_repository"),
            "{value}"
        );
        assert!(
            !linked.join("target/intent-validation").exists(),
            "admission must reject before executing validator"
        );
    }
}
