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

fn normalize_observation_seed(root: &Path, issue: u64) {
    let local = root.join("local");
    let projection: Value =
        serde_json::from_slice(&fs::read(root.join("projection/state.json")).unwrap()).unwrap();
    for (kind, card) in projection["inputs"]["intent_plan"]["cards"]
        .as_object()
        .unwrap()
    {
        write_json(
            &local.join("cards").join(format!("{kind}.values.json")),
            card,
        );
    }
    let index_path = local.join("index.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index["phase"] = projection["phase"].clone();
    index.as_object_mut().unwrap().remove("digest");
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serde_json::to_vec(&index).unwrap());
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        for suffix in ["values.json", "md"] {
            hasher.update(&fs::read(local.join(format!("cards/{kind}.{suffix}"))).unwrap());
        }
    }
    let binding = local.join("binding.json");
    if binding.is_file() {
        hasher.update(&fs::read(binding).unwrap());
    }
    let intent_plan = local.join("intent-plan.json");
    if intent_plan.is_file() {
        hasher.update(b"csdlc.v3.intent_plan.v1\0");
        hasher.update(&fs::read(intent_plan).unwrap());
    }
    index["digest"] = json!(hasher.finalize().to_hex().to_string());
    write_json(&index_path, &index);
    write_json(
        &root.join("generation-seed.json"),
        &json!({
            "schema":"csdlc.v3.current_observation_generation_seed.v1",
            "issue":issue,
            "purpose":"input only for native local and semantic relocation owners",
            "positive_observation":false,
            "normalized_phase":projection["phase"],
            "normalized_cards_from":"projection/state.json#inputs.intent_plan.cards"
        }),
    );
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
        &primary.join(".adl/worktree-policy.json"),
        &json!({"schema":"adl.worktree_policy.v1","required_parent":fixture.0}),
    );
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
    let bound_binding_path = source.join("517/binding.json");
    let original_bound_index_sha256 = sha256(&bound_index_path);
    let original_bound_binding_sha256 = sha256(&bound_binding_path);
    let mut bound_index: Value =
        serde_json::from_slice(&fs::read(&bound_index_path).unwrap()).unwrap();
    bound_index["branch"] = json!("codex/fixture-linked");
    bound_index["worktree"] = json!(linked);
    write_json(&bound_index_path, &bound_index);
    let mut bound_binding: Value =
        serde_json::from_slice(&fs::read(&bound_binding_path).unwrap()).unwrap();
    bound_binding["branch"] = json!("codex/fixture-linked");
    bound_binding["worktree"] = json!(linked);
    write_json(&bound_binding_path, &bound_binding);
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
            "original_binding_sha256":original_bound_binding_sha256,
            "enriched_binding_sha256":sha256(&bound_binding_path),
            "branch":"codex/fixture-linked",
            "worktree":linked,
            "applied_before_source_census":true
        }),
    );
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
    command(&linked, "git", &["merge", "--ff-only", "main"]);
    command(&observation_linked, "git", &["merge", "--ff-only", "main"]);
    assert_eq!(
        command_output(&observation_linked, "git", &["status", "--porcelain=v1"]),
        ""
    );
    let registry = linked.join("docs/templates/prompts/current.json");
    let authority = linked.join("csdlc-v3/operator/authority-selector.json");
    let historical_observation_source = primary.join("historical-current-observations");
    copy_tree(
        &repository_root().join("csdlc-v3/tests/fixtures/issue872-current-observations"),
        &historical_observation_source,
    );
    let observation_source = primary.join("current-observations");
    let primary_observation_issue = 970_u64;
    let primary_observation_seed = observation_source.join(primary_observation_issue.to_string());
    copy_tree(
        &historical_observation_source.join(primary_observation_issue.to_string()),
        &primary_observation_seed,
    );
    normalize_observation_seed(&primary_observation_seed, primary_observation_issue);
    let installed_candidate = primary.join(".adl/bin/native-v3/csdlc");
    fs::create_dir_all(installed_candidate.parent().unwrap()).unwrap();
    fs::copy(&candidate, &installed_candidate).unwrap();
    let fake_bin = fixture.0.join("synthetic-readback-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let fake_curl = fake_bin.join("curl");
    fs::write(
        &fake_curl,
        "#!/bin/sh\ncase \"$*\" in *'--config -'*) cat >/dev/null;; esac\ncase \"$*\" in *issues/981*) printf '%s' '{\"number\":981,\"title\":\"Generated linked observation 981\",\"body\":\"Isolated fixture issue\",\"state\":\"open\",\"labels\":[],\"assignees\":[],\"milestone\":null}' ;; *) exit 9 ;; esac\n",
    )
    .unwrap();
    let token = fixture.0.join("synthetic-readback-token");
    fs::write(&token, "ISSUE872_SYNTHETIC_READBACK_TOKEN").unwrap();
    command(
        &fixture.0,
        "chmod",
        &[
            "+x",
            installed_candidate.to_str().unwrap(),
            fake_curl.to_str().unwrap(),
        ],
    );
    let generation = observation_source.join("981/generation");
    fs::create_dir_all(&generation).unwrap();
    let plan_path = generation.join("prepare-plan.json");
    write_json(
        &plan_path,
        &json!({
            "schema":"csdlc.v3.intent_plan.v1",
            "slug":"issue872-generated-linked-observation-981",
            "cards":{
                "sip":{},"stp":{},
                "spp":{
                    "dependencies_inline":"Isolated dependencies ready",
                    "repo_inputs_inline":"Authenticated isolated fixture",
                    "target_files_surfaces_inline":"Auxiliary current observation",
                    "deliverables_inline":"Native local and semantic observation",
                    "validation_plan_inline":"Installed status and validate",
                    "acceptance_criteria_inline":"Linked readbacks succeed",
                    "notes_risks_inline":"Synthetic read-only issue input; no live mutation"
                },
                "vpp":{},"srp":{},"sor":{}
            },
            "validators":[{"id":"issue872-observation","program":"cargo","args":["test","--manifest-path","csdlc-v3/Cargo.toml","--test","copied_current_observation_relocation"],"success_marker":"test result: ok.","timeout_seconds":60}],
            "publication":{"base":"main","title":"Generated linked observation 981","body":"Closes #981","draft":true}
        }),
    );
    let run_installed = |args: &[&str]| {
        Command::new(&installed_candidate)
            .args(args)
            .current_dir(&primary)
            .env(
                "PATH",
                format!("{}:{}", fake_bin.display(), std::env::var("PATH").unwrap()),
            )
            .env("ADL_GITHUB_TOKEN_FILE", &token)
            .env_remove("GH_TOKEN")
            .output()
            .unwrap()
    };
    let prepared = run_installed(&["prepare", "981", "--plan", plan_path.to_str().unwrap()]);
    assert!(
        prepared.status.success(),
        "generated prepare failed: {} {}",
        String::from_utf8_lossy(&prepared.stdout),
        String::from_utf8_lossy(&prepared.stderr)
    );
    write_json(
        &generation.join("prepare-receipt.json"),
        &serde_json::from_slice::<Value>(&prepared.stdout).unwrap(),
    );
    let bound = run_installed(&["bind", "981"]);
    assert!(
        bound.status.success(),
        "generated bind failed: {} {}",
        String::from_utf8_lossy(&bound.stdout),
        String::from_utf8_lossy(&bound.stderr)
    );
    write_json(
        &generation.join("bind-receipt.json"),
        &serde_json::from_slice::<Value>(&bound.stdout).unwrap(),
    );
    let generated_binding: Value = serde_json::from_slice(
        &fs::read(fixture_common.join("csdlc-v3/local/bindings/981.json")).unwrap(),
    )
    .unwrap();
    let generated_linked = PathBuf::from(generated_binding["worktree"].as_str().unwrap());
    copy_tree(
        &generated_linked.join(".csdlc/issues/981"),
        &observation_source.join("981/local"),
    );
    copy_tree(
        &fixture_common.join("csdlc-v3/semantic/issues/981"),
        &observation_source.join("981/git-common/csdlc-v3/semantic/issues/981"),
    );
    copy_tree(
        &fixture_common.join("csdlc-v3/local/projections/981"),
        &observation_source.join("981/projection"),
    );
    write_json(
        &generation.join("generated-observation.json"),
        &json!({"schema":"csdlc.v3.generated_current_observation_fixture.v1","issue":981,
            "generator":"installed prepare plus native bind","fixture_class":"generated_current_observation",
            "historical_or_converted_role":false,"request_ref":"generation/prepare-plan.json",
            "prepare_receipt_ref":"generation/prepare-receipt.json","bind_receipt_ref":"generation/bind-receipt.json"}),
    );
    let observation_values = vec![
        json!({"issue":970,"source":observation_source.join("970"),"historical_source":historical_observation_source.join("970"),"checkout":"primary","expected_registry_version":"1.0.5","fixture_class":"native_generation_seed"}),
        json!({"issue":981,"source":observation_source.join("981"),"historical_source":historical_observation_source.join("981"),"checkout":"linked","target_worktree":generated_linked,"expected_registry_version":"1.0.5","fixture_class":"generated_current_observation"}),
    ];
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
            "old_executable_blake3":blake3::hash(&fs::read(&old).unwrap()).to_hex().to_string(),
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
