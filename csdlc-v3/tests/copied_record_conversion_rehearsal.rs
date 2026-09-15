use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ROLES: [&str; 7] = [
    "prepared",
    "bound_dirty",
    "implemented",
    "reviewed",
    "published",
    "terminal",
    "pending_recovery",
];

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = Command::new("chmod")
            .args(["-R", "u+w"])
            .arg(&self.0)
            .status();
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn command(cwd: &Path, program: &str, args: &[&str]) {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{} {:?}: {}",
        program,
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_output(cwd: &Path, program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn sha256(path: &Path) -> String {
    command_output(
        path.parent().unwrap(),
        "shasum",
        &["-a", "256", path.file_name().unwrap().to_str().unwrap()],
    )
    .split_whitespace()
    .next()
    .unwrap()
    .to_owned()
}

fn write_json(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(value).unwrap()),
    )
    .unwrap();
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn copied_record_conversion_rehearsal_cli_compatibility_smoke_is_non_proving() {
    let smoke = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .arg("--help")
        .output()
        .unwrap();
    assert_eq!(smoke.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&smoke.stdout).contains("copied_record_conversion_failure")
            && String::from_utf8_lossy(&smoke.stdout).contains("usage:"),
        "non-proving usage rejection did not return the public failure envelope"
    );
}

#[test]
#[ignore = "release-gate proof requires ISSUE872_OLD_CSDLC and ISSUE872_OLD_STATE"]
fn copied_record_conversion_rehearsal_release_gate_executes_complete_isolated_denominator() {
    let proving_old_owner = PathBuf::from(
        std::env::var_os("ISSUE872_OLD_CSDLC")
            .expect("ISSUE872_OLD_CSDLC is required for the ignored release-gate proof"),
    );
    let proving_old_state = PathBuf::from(
        std::env::var_os("ISSUE872_OLD_STATE")
            .expect("ISSUE872_OLD_STATE is required with ISSUE872_OLD_CSDLC"),
    );
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let fixture = Fixture(
        PathBuf::from("/Volumes/FastWork/adl-worktrees")
            .join(format!(".csdlc-872-fixture-{nonce}")),
    );
    let primary = fixture.0.join("primary");
    let linked = fixture.0.join("linked");
    let observation_linked = fixture.0.join("observation-linked");
    fs::create_dir_all(&primary).unwrap();
    command(&primary, "git", &["init", "-q"]);
    command(&primary, "git", &["config", "user.name", "C-SDLC Fixture"]);
    command(
        &primary,
        "git",
        &["config", "user.email", "fixture@example.invalid"],
    );
    fs::write(primary.join("README.md"), "isolated issue 872 fixture\n").unwrap();
    fs::write(
        primary.join("issue872-dirty-tracked.txt"),
        "committed baseline byte\n",
    )
    .unwrap();
    command(
        &primary,
        "git",
        &["add", "README.md", "issue872-dirty-tracked.txt"],
    );
    command(&primary, "git", &["commit", "-qm", "fixture"]);
    command(&primary, "git", &["branch", "-M", "main"]);
    command(
        &primary,
        "git",
        &[
            "remote",
            "add",
            "origin",
            "git@github.com:agent-logic/agent-design-language.git",
        ],
    );
    command(
        &primary,
        "git",
        &[
            "worktree",
            "add",
            "-qb",
            "codex/fixture-observation",
            observation_linked.to_str().unwrap(),
        ],
    );
    command(
        &primary,
        "git",
        &[
            "worktree",
            "add",
            "-qb",
            "codex/fixture-linked",
            linked.to_str().unwrap(),
        ],
    );
    let fixture_common = primary.join(".git");
    fs::create_dir_all(fixture_common.join("objects/info")).unwrap();
    fs::write(
        fixture_common.join("objects/info/alternates"),
        format!(
            "{}/objects\n",
            command_output(
                &repository_root(),
                "git",
                &["rev-parse", "--path-format=absolute", "--git-common-dir"]
            )
        ),
    )
    .unwrap();
    command(
        &primary,
        "git",
        &[
            "update-ref",
            "refs/remotes/origin/main",
            &command_output(&repository_root(), "git", &["rev-parse", "origin/main"]),
        ],
    );
    copy_tree(
        &repository_root().join("csdlc-v3/operator"),
        &primary.join("csdlc-v3/operator"),
    );
    fs::create_dir_all(primary.join(".csdlc/evidence/505")).unwrap();
    fs::copy(
        repository_root().join(".csdlc/evidence/505/terminal-receipt.json"),
        primary.join(".csdlc/evidence/505/terminal-receipt.json"),
    )
    .unwrap();
    fs::create_dir_all(primary.join(".adl")).unwrap();
    fs::copy(
        repository_root().join(".adl/worktree-policy.json"),
        primary.join(".adl/worktree-policy.json"),
    )
    .unwrap();
    write_json(
        &fixture.0.join(".csdlc-conversion-rehearsal.json"),
        &json!({"isolated": true}),
    );

    let source = primary.join("copied-records");
    copy_tree(
        &repository_root().join("csdlc-v3/tests/fixtures/issue872-copied-records"),
        &source,
    );
    let mut role_values = Vec::new();
    let issues = [511_u64, 517, 497, 3, 505, 122, 113];
    for (role, issue) in ROLES.iter().zip(issues) {
        let role_root = source.join(issue.to_string());
        role_values.push(json!({"role": role, "issue": issue, "source": role_root}));
    }
    let prepared_sip_path = source.join("511/cards/sip.values.json");
    let prepared_index_path = source.join("511/index.json");
    let prepared_title_source_path = source.join("511/title-source.json");
    let prepared_title_source: Value =
        serde_json::from_slice(&fs::read(&prepared_title_source_path).unwrap()).unwrap();
    assert_eq!(
        sha256(&prepared_title_source_path),
        "b45b26d1fba0a5ef3f6538b366b65bfc4c41f228305f0f37ecc76c622fd93bcf"
    );
    let original_prepared_sip_sha256 = sha256(&prepared_sip_path);
    let original_prepared_index_sha256 = sha256(&prepared_index_path);
    let mut prepared_sip: Value =
        serde_json::from_slice(&fs::read(&prepared_sip_path).unwrap()).unwrap();
    prepared_sip["title"] = prepared_title_source["title"].clone();
    prepared_sip["issue"] = json!(511);
    prepared_sip["repository"] = prepared_title_source["repository"].clone();
    prepared_sip["branch"] = json!("main");
    prepared_sip["worktree"] = json!(primary);
    prepared_sip["registry_version"] = prepared_title_source["registry_version"].clone();
    prepared_sip["commands"] = prepared_title_source["commands"].clone();
    write_json(&prepared_sip_path, &prepared_sip);
    let mut prepared_index: Value =
        serde_json::from_slice(&fs::read(&prepared_index_path).unwrap()).unwrap();
    prepared_index["title"] = prepared_title_source["title"].clone();
    prepared_index["branch"] = json!("main");
    prepared_index["worktree"] = json!(primary);
    prepared_index["template_registry_version"] = prepared_title_source["registry_version"].clone();
    write_json(&prepared_index_path, &prepared_index);
    write_json(
        &source.join("511/source-enrichment.json"),
        &json!({
            "schema":"csdlc.v3.copied_record_source_enrichment.v1",
            "issue":511,
            "purpose":"normalize the copied historical record to the current candidate request context before conversion",
            "request_context_source":"title-source.json",
            "title_source_sha256":"b45b26d1fba0a5ef3f6538b366b65bfc4c41f228305f0f37ecc76c622fd93bcf",
            "title":prepared_title_source["title"],
            "topology_source":"authenticated isolated primary checkout",
            "branch":"main",
            "worktree":primary,
            "registry_version":prepared_title_source["registry_version"],
            "commands":prepared_title_source["commands"],
            "normalized_fields":["title","issue","repository","branch","worktree","registry_version","commands","template_registry_version"],
            "original_sip_sha256":original_prepared_sip_sha256,
            "enriched_sip_sha256":sha256(&prepared_sip_path),
            "original_index_sha256":original_prepared_index_sha256,
            "enriched_index_sha256":sha256(&prepared_index_path),
            "applied_before_source_census":true
        }),
    );
    let bound_index_path = source.join("517/index.json");
    let original_bound_index_sha256 = sha256(&bound_index_path);
    let mut bound_index: Value =
        serde_json::from_slice(&fs::read(&bound_index_path).unwrap()).unwrap();
    bound_index["branch"] = json!("codex/fixture-linked");
    bound_index["worktree"] = json!(linked);
    write_json(&bound_index_path, &bound_index);
    write_json(
        &source.join("517/source-enrichment.json"),
        &json!({
            "schema":"csdlc.v3.copied_record_source_enrichment.v1",
            "issue":517,
            "purpose":"bind copied historical record to authenticated isolated linked worktree before census",
            "title_source":"cards/sip.values.json#title",
            "title":"[v0.92.1][TAIL-01] Quality gate",
            "original_index_sha256":original_bound_index_sha256,
            "enriched_index_sha256":sha256(&bound_index_path),
            "branch":"codex/fixture-linked",
            "worktree":linked,
            "applied_before_source_census":true
        }),
    );
    let observation_source = primary.join("current-observations");
    copy_tree(
        &repository_root().join("csdlc-v3/tests/fixtures/issue872-current-observations"),
        &observation_source,
    );
    let observation_values = vec![
        json!({"issue":970,"source":observation_source.join("970"),"checkout":"primary","expected_registry_version":"1.0.5"}),
        json!({"issue":981,"source":observation_source.join("981"),"checkout":"linked","target_worktree":observation_linked,"expected_registry_version":"1.0.5"}),
    ];
    let dirty_tracked = source.join("517/dirty-tracked.txt");
    let dirty_untracked = source.join("517/untracked.txt");
    let linked_tracked = linked.join("issue872-dirty-tracked.txt");
    let linked_untracked = linked.join("issue872-dirty-untracked.txt");
    fs::write(&linked_tracked, "authentic modified tracked byte\n").unwrap();
    fs::write(&linked_untracked, "authentic untracked byte\n").unwrap();
    let dirty_status = command_output(
        &linked,
        "git",
        &[
            "status",
            "--porcelain=v1",
            "--",
            "issue872-dirty-tracked.txt",
            "issue872-dirty-untracked.txt",
        ],
    );
    assert!(dirty_status.contains("M issue872-dirty-tracked.txt"));
    assert!(dirty_status.contains("?? issue872-dirty-untracked.txt"));
    fs::copy(&linked_tracked, &dirty_tracked).unwrap();
    fs::copy(&linked_untracked, &dirty_untracked).unwrap();
    write_json(
        &source.join("517/dirty-inventory.json"),
        &json!({
            "schema":"csdlc.v3.copied_record_dirty_inventory.v1",
            "git_status_porcelain_v1":dirty_status,
            "linked_worktree":linked,
            "tracked":{"source_path":"issue872-dirty-tracked.txt","retained_path":"dirty-tracked.txt","sha256":sha256(&dirty_tracked)},
            "untracked":{"source_path":"issue872-dirty-untracked.txt","retained_path":"untracked.txt","sha256":sha256(&dirty_untracked)},
            "machine_derived":true
        }),
    );

    let old = fixture.0.join("old-csdlc");
    let candidate = fixture.0.join("candidate-csdlc");
    let production_owner = fixture.0.join("csdlc-conversion-rehearsal");
    fs::copy(&proving_old_owner, &old).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_csdlc"), &candidate).unwrap();
    fs::copy(
        env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"),
        &production_owner,
    )
    .unwrap();
    command(
        &fixture.0,
        "chmod",
        &[
            "+x",
            old.to_str().unwrap(),
            candidate.to_str().unwrap(),
            production_owner.to_str().unwrap(),
        ],
    );
    copy_tree(
        &repository_root().join("docs/templates/prompts"),
        &primary.join("docs/templates/prompts"),
    );
    copy_tree(
        &repository_root().join("docs/templates/prompts"),
        &linked.join("docs/templates/prompts"),
    );
    command(
        &primary,
        "git",
        &[
            "add",
            "docs/templates/prompts",
            "csdlc-v3/operator",
            ".csdlc/evidence/505",
            ".adl/worktree-policy.json",
        ],
    );
    command(
        &primary,
        "git",
        &["commit", "-qm", "fixture observation registry"],
    );
    command(&observation_linked, "git", &["merge", "--ff-only", "main"]);
    assert_eq!(
        command_output(&observation_linked, "git", &["status", "--porcelain=v1"]),
        ""
    );
    let registry = primary.join("docs/templates/prompts/current.json");
    let authority = primary.join("csdlc-v3/operator/authority-selector.json");
    let old_request = fixture.0.join("old-writer-request.json");
    let fixture_git_common = command_output(
        &primary,
        "git",
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    );
    let old_state = PathBuf::from(&fixture_git_common).join("csdlc-v3/local");
    copy_tree(&proving_old_state, &old_state.join("issues/868"));
    let old_index: Value =
        serde_json::from_slice(&fs::read(old_state.join("issues/868/index.json")).unwrap())
            .unwrap();
    let registrations = fixture.0.join("old-writer-registrations.json");
    write_json(
        &registrations,
        &json!([
            {"branch":"main","worktree":primary,"primary":true},
            {"branch":"codex/fixture-linked","worktree":linked,"primary":false}
        ]),
    );
    write_json(
        &old_request,
        &json!({
            "issue":868, "title":"[v0.92.2][SIM-02] One current installed command contract",
            "repository":"agent-logic/agent-design-language", "branch":"codex/fixture-linked",
            "worktree":linked, "registry_version":"1.0.5",
            "expected_lifecycle_digest":old_index["digest"],
            "commands":["prepare_issue","bind_worktree","edit_cards","plan_pvf","doctor","schedule","shepherd","eligibility"],
            "card_updates":{"sor":{"follow_up_2":"isolated issue 872 authenticated fence control"}},
            "schedule_readiness":{"phase_ready":true,"cards_ready":true,"design_ready":true,"dependencies_ready":true,"paths_clear":true,"budget_available":true}
        }),
    );

    let output = primary.join(".csdlc/evidence/872/conversion-rehearsal");
    let request_path = fixture.0.join("request.json");
    write_json(
        &request_path,
        &json!({
            "schema": "csdlc.v3.copied_record_conversion_rehearsal_request.v1",
            "repository":"agent-logic/agent-design-language",
            "fixture_root": fixture.0, "primary": primary, "linked_worktree": linked,
            "source_root": source, "output_root": output,
            "old_executable": old, "candidate_executable": candidate,
            "old_source_revision":"6425ba9bbce4cc1f46789c2e3ac019adf86238b8",
            "old_owner_proving":true,
            "candidate_source_revision":command_output(&repository_root(), "git", &["rev-parse", "HEAD"]),
            "source_revision":command_output(&repository_root(), "git", &["rev-parse", "HEAD"]),
            "repository_revision":command_output(&repository_root(), "git", &["rev-parse", "HEAD"]),
            "production_owner": production_owner, "registry_path": registry,
            "authority_bytes_path": authority,
            "old_writer_lock": old_state.join("locks/868.lock"),
            "old_writer_state_root": old_state.join("issues/868"),
            "old_writer_command": [old, "edit", "--request", old_request,
                "--registry", registry, "--registrations", registrations,
                "--repo-root", primary, "--v3-state-root", old_state], "roles": role_values,
            "observations":observation_values
        }),
    );

    let repo = repository_root();
    let run = Command::new(repo.join("adl/tools/csdlc-conversion-rehearsal"))
        .args(["--request", request_path.to_str().unwrap()])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "runner stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let report: Value = serde_json::from_slice(&run.stdout).unwrap();
    if std::env::var_os("ISSUE872_FAULT_FILTER").is_some() {
        assert_eq!(report["status"], "fault_probe_completed");
        println!("{report}");
        return;
    }
    assert_eq!(report["status"], "passed");
    assert_eq!(
        report["proof_denominator"],
        json!({"roles": 7, "scenarios": 12, "fault_cases": 30})
    );

    let validate = Command::new("python3")
        .args([
            repo.join("adl/tools/validate_issue872_conversion_rehearsal.py")
                .to_str()
                .unwrap(),
            "--evidence-root",
            output.to_str().unwrap(),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        validate.status.success(),
        "validator stdout={} stderr={}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr)
    );
    if let Some(destination) = std::env::var_os("ISSUE872_RETAIN_EVIDENCE") {
        let destination = PathBuf::from(destination);
        if destination.exists() {
            fs::remove_dir_all(&destination).unwrap();
        }
        copy_tree(&output, &destination);
    }
}
