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

#[cfg(unix)]
#[test]
fn interrupted_proof_is_explicitly_abandoned_without_rerun_and_can_retry() {
    let mut fixture = Fixture::new("semantic-proof-indeterminate-abandonment");
    let primary = fixture.root.clone();
    let marker = primary.join(".git/proof-launches");
    fs::write(
        primary.join("fixture-proof/src/lib.rs"),
        format!(
            "#[test]\nfn records_launch() {{ use std::io::Write; let path = std::path::Path::new({marker:?}); let prior = std::fs::read_to_string(path).unwrap_or_default().lines().count(); let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).unwrap(); writeln!(f, \"launch\").unwrap(); assert_eq!(prior, 0, \"retry fixture failure\"); }}\n"
        ),
    )
    .unwrap();
    fixture::git(&primary, &["add", "fixture-proof/src/lib.rs"]);
    fixture::git(
        &primary,
        &["commit", "--quiet", "-m", "instrument proof launch"],
    );
    fixture::git(
        &primary,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );

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
    let before = snapshot(&primary);
    let crash = fixture.run_with_env(
        &linked,
        &["proof", "870"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_proof_after_execution",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
    let interrupted = snapshot(&primary);
    let pending = interrupted.pending().expect("retained proof reservation");
    let interrupted_id = pending.id().as_str().to_owned();
    assert_eq!(interrupted.inputs_version(), before.inputs_version());
    assert!(pending.observed_truth().is_none());

    let inventory = fixture::inventory(&primary);
    let preview = success(fixture.run(&linked, &["recover", "870"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(inventory, fixture::inventory(&primary));
    let disposition = fixture.write_json(
        "proof-abandonment.json",
        &json!({
            "schema":"csdlc.v3.semantic_proof_recovery_disposition.v1",
            "action":"abandon_indeterminate_proof",
            "operation_id":interrupted_id,
            "rationale":"validator outcome was not durably retained after interruption"
        }),
    );
    let recovered = success(fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--disposition",
            disposition.to_str().unwrap(),
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(recovered["action"], "abandoned_indeterminate_proof");
    assert_eq!(recovered["native_effect_truth"], "unknown");
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
    let abandoned = snapshot(&primary);
    assert!(abandoned.pending().is_none());
    assert_eq!(abandoned.inputs_version(), before.inputs_version());
    assert_eq!(abandoned.phase(), before.phase());
    let completed = abandoned.completed().last().unwrap();
    assert_eq!(completed.id().as_str(), interrupted_id);
    assert_eq!(
        completed.outcome(),
        csdlc_v3::storage::semantic::protocol::OutcomeKind::Failure
    );
    assert_eq!(
        completed.truth(),
        csdlc_v3::storage::semantic::protocol::EffectTruth::Unknown
    );
    assert!(!linked.join(".csdlc/v3/issues/870/proof.json").exists());

    let proof = fixture.run(&linked, &["proof", "870"]);
    assert!(!proof.status.success());
    let proof: Value = serde_json::from_slice(&proof.stdout).unwrap();
    assert_eq!(proof["status"], "failed");
    assert_ne!(proof["operation_id"], interrupted_id);
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 2);
    assert_eq!(
        success(fixture.run(&linked, &["status", "870"]))["evidence"]["proof_current"],
        false
    );
    let replay = fixture.run(&linked, &["proof", "870"]);
    assert!(!replay.status.success());
    let replay: Value = serde_json::from_slice(&replay.stdout).unwrap();
    assert_eq!(replay["status"], "failed");
    assert_eq!(replay["read_only"], true);
    assert_eq!(replay["operation_id"], proof["operation_id"]);
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 2);
}

#[cfg(unix)]
#[test]
fn prelaunch_proof_interruption_requires_exact_explicit_disposition() {
    let mut fixture = Fixture::new("semantic-proof-prelaunch-abandonment");
    let primary = fixture.root.clone();
    let marker = primary.join(".git/proof-launches");
    fs::write(
        primary.join("fixture-proof/src/lib.rs"),
        format!(
            "#[test]\nfn records_launch() {{ std::fs::write({marker:?}, \"launch\\n\").unwrap(); }}\n"
        ),
    )
    .unwrap();
    fixture::git(&primary, &["add", "fixture-proof/src/lib.rs"]);
    fixture::git(
        &primary,
        &["commit", "--quiet", "-m", "instrument prelaunch proof"],
    );
    fixture::git(
        &primary,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
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
    let crash = fixture.run_with_env(
        &linked,
        &["proof", "870"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_proof_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert!(
        !marker.exists(),
        "validator launched before recovery choice"
    );
    let interrupted = snapshot(&primary);
    let operation = interrupted.pending().unwrap().id().as_str().to_owned();
    let preview = success(fixture.run(&linked, &["recover", "870"]));
    let forged = fixture.write_json(
        "forged-proof-abandonment.json",
        &json!({"schema":"csdlc.v3.semantic_proof_recovery_disposition.v1",
            "action":"abandon_indeterminate_proof","operation_id":"wrong-operation",
            "rationale":"negative fixture"}),
    );
    let before_refusals = snapshot(&primary);
    let wrong_operation = fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--disposition",
            forged.to_str().unwrap(),
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!wrong_operation.status.success());
    assert_eq!(snapshot(&primary), before_refusals);
    let valid = fixture.write_json(
        "valid-proof-abandonment.json",
        &json!({"schema":"csdlc.v3.semantic_proof_recovery_disposition.v1",
            "action":"abandon_indeterminate_proof","operation_id":operation,
            "rationale":"prelaunch interruption left no durable outcome"}),
    );
    let stale = fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--disposition",
            valid.to_str().unwrap(),
            "--execute",
            "--preview",
            "wrong-preview",
        ],
    );
    assert!(!stale.status.success());
    assert_eq!(snapshot(&primary), before_refusals);
    success(fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--disposition",
            valid.to_str().unwrap(),
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert!(!marker.exists(), "recovery launched the validator");
    let repeated = success(fixture.run(
        &linked,
        &[
            "recover",
            "870",
            "--disposition",
            valid.to_str().unwrap(),
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(repeated["status"], "expected_noop");
    assert_eq!(repeated["action"], "abandoned_indeterminate_proof");
    assert!(
        !marker.exists(),
        "idempotent recovery launched the validator"
    );
    let proof = success(fixture.run(&linked, &["proof", "870"]));
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
    let replay = success(fixture.run(&linked, &["proof", "870"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_eq!(replay["operation_id"], proof["operation_id"]);
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
}

#[cfg(unix)]
#[test]
fn failed_proof_replay_retains_the_failed_attempt_without_rerun() {
    let mut fixture = Fixture::new("semantic-failed-proof-replay");
    let primary = fixture.root.clone();
    let marker = primary.join(".git/failed-proof-launches");
    fs::write(
        primary.join("fixture-proof/src/lib.rs"),
        format!(
            "#[test]\nfn records_failed_launch() {{ use std::io::Write; let mut f = std::fs::OpenOptions::new().create(true).append(true).open({marker:?}).unwrap(); writeln!(f, \"launch\").unwrap(); panic!(\"expected failure\"); }}\n"
        ),
    )
    .unwrap();
    fixture::git(&primary, &["add", "fixture-proof/src/lib.rs"]);
    fixture::git(
        &primary,
        &["commit", "--quiet", "-m", "install failing proof"],
    );
    fixture::git(
        &primary,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
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
    let failed = fixture.run(&linked, &["proof", "870"]);
    assert!(!failed.status.success());
    let failed: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(failed["status"], "failed");
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
    let replay = fixture.run(&linked, &["proof", "870"]);
    assert!(!replay.status.success());
    let replay: Value = serde_json::from_slice(&replay.stdout).unwrap();
    assert_eq!(replay["status"], "failed");
    assert_eq!(replay["read_only"], true);
    assert_eq!(replay["operation_id"], failed["operation_id"]);
    assert_eq!(fs::read_to_string(&marker).unwrap().lines().count(), 1);
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
    let inapplicable = fixture.write_json(
        "inapplicable-proof-disposition.json",
        &json!({"schema":"csdlc.v3.semantic_proof_recovery_disposition.v1",
            "action":"abandon_indeterminate_proof","operation_id":"not-the-bind-operation",
            "rationale":"negative fixture"}),
    );
    let before_inapplicable = snapshot(&primary);
    let rejected = fixture.run(
        &primary,
        &[
            "recover",
            "870",
            "--disposition",
            inapplicable.to_str().unwrap(),
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!rejected.status.success());
    assert_eq!(snapshot(&primary), before_inapplicable);
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
        &json!({"schema":"csdlc.v3.intent_changes.v1","amendment":{"class":"scope_acceptance","transition_approved":true},"cards":{"sip":{"title":"Recovered semantic edit"}}}),
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

// PVF #1003: deterministic installed regression, local Git/CPU only, required gate.
#[test]
fn scope_rewound_binding_can_rebind_at_the_same_head() {
    let mut fixture = Fixture::new("scope-rebind-same-head");
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
    let changes = fixture.write_json("scope.json", &json!({"schema":"csdlc.v3.intent_changes.v1", "amendment":{"class":"scope_acceptance","transition_approved":true}, "cards":{"sip":{"goal":"Amended accepted scope"}}}));
    success(fixture.run(
        &linked,
        &["edit", "870", "--changes", changes.to_str().unwrap()],
    ));
    let rewound = snapshot(&primary);
    assert_eq!(rewound.phase(), csdlc_v3::lifecycle::LifecycleState::Ready);
    success(fixture.run(&linked, &["bind", "870"]));
    let rebound = snapshot(&primary);
    assert_eq!(rebound.phase(), csdlc_v3::lifecycle::LifecycleState::Bound);
    assert_eq!(rebound.inputs().binding(), rewound.inputs().binding());
    assert!(rebound.version().generation() > rewound.version().generation());
    assert_eq!(
        success(fixture.run(&linked, &["status", "870"]))["evidence"]["proof_current"],
        false
    );
}

#[test]
fn scope_rebind_refreshes_head_before_replacing_an_inadmissible_validator() {
    let mut fixture = Fixture::new("scope-rebind-validator-repair");
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
    let changes = fixture.write_json("scope.json", &json!({"schema":"csdlc.v3.intent_changes.v1", "amendment":{"class":"scope_acceptance","transition_approved":true}, "cards":{"sip":{"goal":"Amended scope requiring a new validator"}}}));
    success(fixture.run(
        &linked,
        &["edit", "870", "--changes", changes.to_str().unwrap()],
    ));
    fs::create_dir_all(linked.join("fixture-proof/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/tests/unselected.rs"),
        "fn main() { panic!(\"inadmissible validator executed\"); }\n",
    )
    .unwrap();
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[test]]\nname = \"unselected\"\npath = \"tests/unselected.rs\"\nharness = false\n",
    );
    fs::write(manifest, content).unwrap();
    fixture::git(&linked, &["add", "fixture-proof"]);
    fixture::git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Change validator admission surface",
        ],
    );
    let proof = fixture.run(&linked, &["proof", "870"]);
    assert!(!proof.status.success());
    assert!(
        String::from_utf8_lossy(&proof.stderr)
            .contains("intent_validator_custom_harness_not_admitted"),
        "{proof:?}"
    );
    success(fixture.run(&linked, &["bind", "870"]));
    let rebound = snapshot(&primary);
    assert_eq!(rebound.phase(), csdlc_v3::lifecycle::LifecycleState::Bound);
    assert_eq!(
        rebound.inputs().binding().unwrap().head,
        fixture::git(&linked, &["rev-parse", "HEAD"])
    );
    let mut validators = plan()["validators"].clone();
    validators[0]["args"]
        .as_array_mut()
        .unwrap()
        .push(json!("--lib"));
    let changes = fixture.write_json(
        "validators.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","validators":validators}),
    );
    success(fixture.run(
        &linked,
        &["edit", "870", "--changes", changes.to_str().unwrap()],
    ));
    let proof = success(fixture.run(&linked, &["proof", "870"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
}

#[test]
fn rebind_rejects_stale_generated_request_and_wrong_branch_without_state_changes() {
    let mut fixture = Fixture::new("rebind-identity-guards");
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
    let generated = fixture.run(&linked, &["bind", "870", "--emit-request"]);
    assert!(generated.status.success());
    let generated: Value = serde_json::from_slice(&generated.stdout).unwrap();
    let request = fixture.write_json("stale-bind.json", &generated);
    fixture::git(
        &linked,
        &["commit", "--allow-empty", "--quiet", "-m", "New candidate"],
    );
    let before = snapshot(&primary);
    let denied = fixture.run(
        &linked,
        &["bind", "--intent-request", request.to_str().unwrap()],
    );
    assert!(!denied.status.success());
    let failure: Value = serde_json::from_slice(&denied.stdout).unwrap();
    assert_eq!(failure["findings"][0]["code"], "intent_snapshot_stale");
    assert_eq!(before.version(), snapshot(&primary).version());
    fixture::git(
        &linked,
        &["switch", "--quiet", "-c", "codex/870-wrong-owner"],
    );
    let denied = fixture.run(&linked, &["bind", "870"]);
    assert!(!denied.status.success());
    let failure: Value = serde_json::from_slice(&denied.stdout).unwrap();
    assert_eq!(
        failure["findings"][0]["code"],
        "intent_bound_checkout_mismatch"
    );
    assert_eq!(before.version(), snapshot(&primary).version());
}

#[test]
fn scope_rebind_does_not_advance_when_native_doctor_is_blocked() {
    let mut fixture = Fixture::new("rebind-blocked-doctor");
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
    let changes = fixture.write_json("scope.json", &json!({"schema":"csdlc.v3.intent_changes.v1", "amendment":{"class":"scope_acceptance","transition_approved":true}, "cards":{"sip":{"goal":"Scope changed"}}}));
    success(fixture.run(
        &linked,
        &["edit", "870", "--changes", changes.to_str().unwrap()],
    ));
    let before = snapshot(&primary);
    let template = linked.join("docs/templates/prompts/1.0.5/sip.md");
    let mut content = fs::read_to_string(&template).unwrap();
    content.push_str("\nUnexpected template drift\n");
    fs::write(template, content).unwrap();
    let denied = fixture.run(&linked, &["bind", "870"]);
    assert!(!denied.status.success(), "{denied:?}");
    assert!(
        String::from_utf8_lossy(&denied.stdout).contains("rendered_card_drift"),
        "{denied:?}"
    );
    assert_eq!(before.version(), snapshot(&primary).version());
    assert_eq!(
        snapshot(&primary).phase(),
        csdlc_v3::lifecycle::LifecycleState::Ready
    );
}
