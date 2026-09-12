//! PVF tooling / required #870 consumer proof. Isolated installed candidate,
//! deterministic Git + synthetic transport, local CPU/disk only. Fixture setup
//! establishes authority; every claimed issue transition executes through the CLI.
//! No live GitHub, active installation, or caller-authored semantic capability.
use csdlc_v3::storage::{
    semantic::{IssueKey, Observation, SemanticRoot, Snapshot},
    DurableTransactionStore,
};
use serde_json::{json, Value};
use std::{fs, path::Path, process::Output};
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
        "publication":{"base":"main","title":"Semantic fixture","body":"Closes #870","draft":true}})
}
fn snapshot(primary: &Path) -> Snapshot {
    let root =
        SemanticRoot::from_git_common(primary.join(".git"), "agent-logic/agent-design-language")
            .unwrap();
    let key = IssueKey::new("agent-logic/agent-design-language", 870).unwrap();
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
        &["prepare", "870", "--plan", input.to_str().unwrap()],
    ));
    let prepared = snapshot(&primary);
    assert_eq!(prepared.phase(), csdlc_v3::lifecycle::LifecycleState::Ready);
    assert!(prepared.inputs().binding().is_none());
    success(fixture.run(&primary, &["bind", "870"]));
    let bound = snapshot(&primary);
    assert_eq!(bound.phase(), csdlc_v3::lifecycle::LifecycleState::Bound);
    assert!(bound.version().generation() > prepared.version().generation());
    assert_ne!(bound.inputs_version(), prepared.inputs_version());
    let linked = bound.inputs().binding().unwrap().worktree.clone();
    for command in ["status", "validate"] {
        let before = fixture::inventory(&primary);
        success(fixture.run(&linked, &[command, "870"]));
        assert_eq!(
            before,
            fixture::inventory(&primary),
            "observation wrote fixture state"
        );
    }
    let before = snapshot(&primary);
    let proof = success(fixture.run(&linked, &["proof", "870"]));
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
    assert!(!Path::new(&linked)
        .join(".csdlc/evidence/870/intent-proof.json")
        .exists());
    let projected: Value = serde_json::from_slice(
        &fs::read(Path::new(&linked).join(".csdlc/v3/issues/870/proof.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(projected, proof["proof"]);
    let status = success(fixture.run(&linked, &["status", "870"]));
    assert_eq!(status["evidence"]["proof_current"], true);
    assert_eq!(
        inventory,
        fixture::inventory(&primary),
        "semantic proof status wrote state"
    );
    let replay = success(fixture.run(&linked, &["proof", "870"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_eq!(snapshot(&primary).version(), proven.version());
    assert_eq!(
        inventory,
        fixture::inventory(&primary),
        "proof replay ran an effect"
    );

    let proof_path = Path::new(&linked).join(".csdlc/v3/issues/870/proof.json");
    fs::remove_file(&proof_path).unwrap();
    let retained = snapshot(&primary);
    let preview = success(fixture.run(&linked, &["recover", "870"]));
    assert_eq!(preview["status"], "recovery_required");
    assert!(!proof_path.exists(), "recovery preview repaired projection");
    success(fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(snapshot(&primary).version(), retained.version());
    let repaired: Value = serde_json::from_slice(&fs::read(&proof_path).unwrap()).unwrap();
    assert_eq!(repaired, proof["proof"]);
}

#[test]
fn installed_bind_recovers_after_target_activation_without_ambiguous_topology() {
    let mut fixture = Fixture::new("semantic-bind-target-activation-recovery");
    let primary = fixture.root.clone();
    let input = fixture.write_json("semantic-plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "870", "--plan", input.to_str().unwrap()],
    ));
    let crash = fixture.run_with_env(
        &primary,
        &["bind", "870"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "bind_after_target_stage_rename",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    let interrupted = snapshot(&primary);
    assert!(interrupted.pending().is_some());
    let journal = primary.join(".git/csdlc-v3/local/transactions/870.json");
    assert!(journal.is_file());
    assert!(!primary
        .join(".git/csdlc-v3/local/bindings/870.json")
        .exists());
    let interrupted_completed = interrupted.completed().len();
    let before = fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "870"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(before, fixture::inventory(&primary));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "870",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    let bound = snapshot(&primary);
    assert_eq!(bound.phase(), csdlc_v3::lifecycle::LifecycleState::Bound);
    assert!(bound.pending().is_none());
    assert_eq!(bound.completed().len(), interrupted_completed + 1);
    let binding = bound.inputs().binding().expect("recovered exact binding");
    assert!(binding
        .worktree
        .join(".csdlc/issues/870/index.json")
        .is_file());
    assert_eq!(
        fixture::git(&primary, &["worktree", "list", "--porcelain"])
            .lines()
            .filter(|line| line.strip_prefix("worktree ") == binding.worktree.to_str())
            .count(),
        1
    );
    assert!(!journal.exists());
    assert!(primary
        .join(".git/csdlc-v3/local/bindings/870.json")
        .is_file());
}

#[test]
fn installed_edit_recovers_one_retained_semantic_amendment_after_native_crash() {
    let mut fixture = Fixture::new("semantic-edit-native-recovery");
    let primary = fixture.root.clone();
    let input = fixture.write_json("semantic-plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "870", "--plan", input.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "870"]));
    let linked = snapshot(&primary)
        .inputs()
        .binding()
        .unwrap()
        .worktree
        .clone();
    let changes = fixture.write_json(
        "semantic-changes.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","cards":{"sip":{"title":"Recovered semantic edit"}}}),
    );
    let crash = fixture.run_with_env(
        &linked,
        &["edit", "870", "--changes", changes.to_str().unwrap()],
        &[("CSDLC_V3_TEST_CRASH_POINT", "after_backup_rename")],
    );
    assert_eq!(crash.status.code(), Some(91));
    let preview = success(fixture.run(&linked, &["recover", "870"]));
    assert_eq!(preview["status"], "recovery_required");
    success(fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    let amended = snapshot(&primary);
    assert_eq!(
        amended.inputs().cards()["sip"]["title"],
        "Recovered semantic edit"
    );
    assert!(amended.pending().is_none());
}

#[test]
fn installed_prepare_refuses_retained_native_journal_without_creating_semantic_state() {
    let mut fixture = Fixture::new("semantic-legacy-collision");
    let primary = fixture.root.clone();
    let journal = primary.join(".git/csdlc-v3/local/transactions/870.json");
    std::fs::create_dir_all(journal.parent().unwrap()).unwrap();
    std::fs::write(&journal, b"retained interrupted native journal").unwrap();
    let input = fixture.write_json("semantic-plan.json", &plan());
    let before = fixture::inventory(&primary);
    let output = fixture.run(
        &primary,
        &["prepare", "870", "--plan", input.to_str().unwrap()],
    );
    assert!(!output.status.success());
    assert_eq!(before, fixture::inventory(&primary));
    assert!(!primary.join(".git/csdlc-v3/semantic/issues/870").exists());
}

/// PVF #870: required deterministic installed topology/confinement negatives;
/// local CPU/Git/disk only. The Cargo tripwire distinguishes validator launch
/// from read-only metadata admission, and every fixture inventory is preserved.
#[cfg(unix)]
#[test]
fn installed_semantic_proof_rejects_live_topology_and_output_escape_before_validator() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    for case in [
        "corrupt-registration",
        "detached-head",
        "symlink-proof-output",
    ] {
        let mut fixture = Fixture::new(&format!("semantic-proof-{case}"));
        let primary = fixture.root.clone();
        let input = fixture.write_json("plan.json", &plan());
        success(fixture.run(
            &primary,
            &["prepare", "870", "--plan", input.to_str().unwrap()],
        ));
        success(fixture.run(&primary, &["bind", "870"]));
        let linked = snapshot(&primary)
            .inputs()
            .binding()
            .unwrap()
            .worktree
            .clone();
        let external = primary.parent().unwrap().join(format!(
            "{}-external",
            primary.file_name().unwrap().to_string_lossy()
        ));
        fs::create_dir_all(&external).unwrap();
        let sentinel = external.join("sentinel.json");
        fs::write(&sentinel, b"foreign protected bytes").unwrap();
        let marker = external.join("validator-launched");
        let real_cargo = std::env::split_paths(&std::env::var_os("PATH").unwrap())
            .map(|p| p.join("cargo"))
            .find(|p| p.is_file())
            .unwrap();
        let quote = |value: &str| format!("'{}'", value.replace('\'', "'\\''"));
        let wrapper = primary.join(".git/installed-candidate/fake-bin/cargo");
        fs::write(
            &wrapper,
            format!(
                "#!/bin/sh\nif [ \"$1\" = test ]; then touch {}; exit 97; fi\nexec {} \"$@\"\n",
                quote(marker.to_str().unwrap()),
                quote(real_cargo.to_str().unwrap())
            ),
        )
        .unwrap();
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
        match case {
            "corrupt-registration" => {
                let metadata = fixture::git(&linked, &["rev-parse", "--absolute-git-dir"]);
                fs::write(
                    Path::new(&metadata).join("gitdir"),
                    format!("{}/missing/.git\n", external.display()),
                )
                .unwrap();
            }
            "detached-head" => {
                fixture::git(&linked, &["checkout", "--detach"]);
            }
            "symlink-proof-output" => {
                let output = linked.join(".csdlc/v3/issues/870/proof.json");
                fs::create_dir_all(output.parent().unwrap()).unwrap();
                symlink(&sentinel, &output).unwrap();
            }
            _ => unreachable!(),
        }
        let primary_before = fixture::inventory(&primary);
        let linked_before = fixture::inventory(&linked);
        let external_before = fixture::inventory(&external);
        let output = fixture.run(&linked, &["proof", "870"]);
        assert!(!output.status.success(), "{case}: {output:?}");
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(!marker.exists(), "{case}: validator launched: {report}");
        assert_eq!(
            primary_before,
            fixture::inventory(&primary),
            "{case}: primary changed: {report}"
        );
        assert_eq!(
            linked_before,
            fixture::inventory(&linked),
            "{case}: linked changed: {report}"
        );
        assert_eq!(
            external_before,
            fixture::inventory(&external),
            "{case}: external changed: {report}"
        );
        assert_eq!(fixture.remote_effects(), 0);
    }
}
