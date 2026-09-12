//! PVF #869 manual artifact-input measurement; required acceptance evidence.
//! Run explicitly with --ignored and ADL_SIM03_BASELINE_BINARY. Ordinary CI's
//! ignored result is not measurement proof. No OS cache reset or live effects.
#![cfg(unix)]
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

const BASELINE_DIGEST: &str = "e7fa9aedc8f992151dcfc82ff9bd0da7f5415d69798ac67956f2977f88416f64";
fn read(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn write(path: &Path, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn plan() -> Value {
    json!({"schema":"csdlc.v3.intent_plan.v1","slug":"prepared-start-measurement",
    "cards":{"sip":{},"stp":{},"spp":{},"vpp":{},"srp":{},"sor":{}},
    "validators":[{"id":"fixture-proof","program":"cargo","args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline"],"success_marker":"test result: ok."}],
    "publication":{"base":"main","title":"Installed intent fixture","body":"Closes #505","draft":true}})
}
fn git_log(f: &intent_fixture::Fixture) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/sim03-intent-corpus")
        .join(f.root.file_name().unwrap())
        .with_extension("git.jsonl")
}
fn events(f: &intent_fixture::Fixture) -> Vec<Value> {
    fs::read_to_string(git_log(f))
        .unwrap_or_default()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn environment(command: &mut Command, f: &intent_fixture::Fixture) {
    command
        .env_clear()
        .env(
            "PATH",
            format!(
                "{}:{}",
                f.root.join(".git/installed-candidate/fake-bin").display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env("HOME", f.root.join(".git/installed-candidate"))
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env(
            "ADL_GITHUB_TOKEN_FILE",
            f.root.join(".git/installed-candidate/token"),
        )
        .env("GITHUB_TOKEN", "SIM03_SYNTHETIC_TOKEN");
}
fn run(f: &intent_fixture::Fixture, binary: &Path, cwd: &Path, args: &[String]) -> Value {
    let before = events(f).len();
    let started = Instant::now();
    let mut command = Command::new(binary);
    command.current_dir(cwd).args(args);
    environment(&mut command, f);
    let output = command.output().unwrap();
    let elapsed = started.elapsed().as_millis();
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        output.status.success(),
        "measurement invocation {args:?}: {report}; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!String::from_utf8_lossy(&output.stdout).contains("SIM03_SYNTHETIC_TOKEN"));
    json!({"argv":args,"process_elapsed_ms":elapsed,"git_events":events(f).into_iter().skip(before).collect::<Vec<_>>(),"result":report})
}
fn registrations(f: &intent_fixture::Fixture) -> Value {
    let mut command = Command::new(f.root.join(".git/installed-candidate/fake-bin/git"));
    command
        .arg("-C")
        .arg(&f.root)
        .args(["worktree", "list", "--porcelain"]);
    environment(&mut command, f);
    let output = command.output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    Value::Array(text.split("\n\n").filter_map(|block|{
        let worktree=block.lines().find_map(|line|line.strip_prefix("worktree "))?;
        let branch=block.lines().find_map(|line|line.strip_prefix("branch refs/heads/"))?;
        Some(json!({"worktree":worktree,"branch":branch,"primary":Path::new(worktree)==f.root}))
    }).collect())
}
fn leaves(value: &Value) -> usize {
    match value {
        Value::Object(v) => v.values().map(leaves).sum(),
        Value::Array(v) => v.iter().map(leaves).sum(),
        _ => 1,
    }
}

fn measure(baseline: Option<&Path>, repetition: usize) -> Value {
    let variant = if baseline.is_some() {
        "predecessor"
    } else {
        "candidate"
    };
    let f = intent_fixture::Fixture::new(&format!("measurement-{variant}-{repetition}"));
    let binary = if let Some(source) = baseline {
        let p = f.root.join(".git/installed-candidate/accepted-predecessor");
        fs::copy(source, &p).unwrap();
        p
    } else {
        f.binary.clone()
    };
    let binary_digest = blake3::hash(&fs::read(&binary).unwrap())
        .to_hex()
        .to_string();
    let registry = f.root.join("docs/templates/prompts/current.json");
    let bound = f
        .root
        .join("worktrees/adl-issue-505-prepared-start-measurement");
    let input = plan();
    let plan_path = f.write_json("measurement-plan.json", &input);
    let request_path = f
        .root
        .join(".git/installed-candidate/measurement-request.json");
    let registration_path = f
        .root
        .join(".git/installed-candidate/measurement-registrations.json");
    let mut request = json!({"issue":505,"title":"Installed intent fixture","repository":"agent-logic/agent-design-language",
        "branch":"codex/505-prepared-start-measurement","worktree":bound,
        "registry_version":read(&registry)["csdlc_prompt_template_set"],"expected_lifecycle_digest":null,
        "commands":csdlc_v3::commands::local::required_local_commands(),"card_updates":input["cards"],
        "schedule_readiness":null,"shepherd_routing":null});
    let old_args = |route: &str, root: &Path| {
        vec![
            route.into(),
            "--request".into(),
            request_path.to_str().unwrap().into(),
            "--registry".into(),
            registry.to_str().unwrap().into(),
            "--registrations".into(),
            registration_path.to_str().unwrap().into(),
            "--repo-root".into(),
            root.to_str().unwrap().into(),
        ]
    };
    let preparation = if baseline.is_some() {
        write(&request_path, &request);
        write(&registration_path, &registrations(&f));
        run(&f, &binary, &f.root, &old_args("issue", &f.root))
    } else {
        run(
            &f,
            &binary,
            &f.root,
            &[
                "prepare".into(),
                "505".into(),
                "--plan".into(),
                plan_path.to_str().unwrap().into(),
            ],
        )
    };
    let prepared_index = f.root.join(".git/csdlc-v3/local/issues/505/index.json");
    assert_eq!(read(&prepared_index)["phase"], "ready");
    assert!(!bound.exists());
    let first_event = events(&f).len();
    let start = Instant::now();
    let mut invocations = Vec::new();
    let mut materializations = Vec::new();
    if baseline.is_some() {
        // Canonical index and actual Git topology, never caller-invented state.
        let index = read(&prepared_index);
        request["expected_lifecycle_digest"] = index["digest"].clone();
        let regs = registrations(&f);
        write(&request_path, &request);
        write(&registration_path, &regs);
        materializations.push(
            json!({"phase":"prepared","request":request,"registrations":regs,"file_writes":2}),
        );
        invocations.push(run(&f, &binary, &f.root, &old_args("doctor", &f.root)));
        invocations.push(run(&f, &binary, &f.root, &old_args("bind", &f.root)));
        let index = read(&bound.join(".csdlc/issues/505/index.json"));
        request["expected_lifecycle_digest"] = index["digest"].clone();
        let regs = registrations(&f);
        write(&request_path, &request);
        write(&registration_path, &regs);
        materializations
            .push(json!({"phase":"bound","request":request,"registrations":regs,"file_writes":2}));
        invocations.push(run(&f, &binary, &f.root, &old_args("validate", &bound)));
    } else {
        for route in ["status", "bind", "validate"] {
            invocations.push(run(&f, &binary, &f.root, &[route.into(), "505".into()]));
        }
    }
    let total_elapsed_ms = start.elapsed().as_millis();
    let observed = events(&f).into_iter().skip(first_event).collect::<Vec<_>>();
    assert_eq!(
        read(&bound.join(".csdlc/issues/505/index.json"))["phase"],
        "bound"
    );
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        assert!(bound
            .join(format!(".csdlc/issues/505/cards/{card}.md"))
            .is_file());
    }
    let mut files = BTreeSet::new();
    let mut argv_tokens = 0;
    for call in &invocations {
        let args = call["argv"].as_array().unwrap();
        argv_tokens += args.len();
        for pair in args.windows(2) {
            if matches!(
                pair[0].as_str(),
                Some("--request" | "--registry" | "--registrations")
            ) {
                files.insert(pair[1].as_str().unwrap().to_owned());
            }
        }
    }
    let report = json!({"variant":variant,"repetition":repetition,"host_condition":if repetition==0{"first measured fixture/process on already warm host"}else{"later fresh fixture/process on warmed host"},
        "binary_blake3":binary_digest,"preparation_outside_interval":preparation,
        "total_prepared_start_elapsed_ms":total_elapsed_ms,"three_minute_fixture_target_met":total_elapsed_ms<=180_000,
        "interval":"Canonical prepared-index input refresh through successful final native validate; includes orchestration, JSON materialization and instrumentation",
        "invocations":invocations,"caller_materializations":materializations,"supplied_unique_files_in_interval":files,
        "argv_tokens_in_interval":argv_tokens,"semantic_plan_scalar_leaves":leaves(&input),"semantic_plan_blake3":blake3::hash(&serde_json::to_vec(&input).unwrap()).to_hex().to_string(),
        "actual_git_events_in_interval":observed,"git_invocations_in_interval":observed.len(),
        "authority_git_blob_reads":observed.iter().filter(|v|v["verb"]=="show"&&v["subcommand"].as_str().is_some_and(|s|s.contains("authority-selector")||s.contains("authority-receipt")||s.contains("authority-pr-observation"))).count(),
        "baseline_in_process_authority_reads":"not instrumented in accepted predecessor binary; unavailable, not zero",
        "bound_cards_present":6,"final_native_phase":"bound","human_authoring_effort_measured":false});
    serde_json::from_str(
        &serde_json::to_string(&report)
            .unwrap()
            .replace(f.root.to_str().unwrap(), "$FIXTURE_ROOT"),
    )
    .unwrap()
}

#[test]
#[ignore = "Manual required measurement: supply verified ADL_SIM03_BASELINE_BINARY and run --ignored; CI ignore is not acceptance proof"]
fn accepted_predecessor_and_candidate_prepared_start_measurements() {
    let baseline = PathBuf::from(
        std::env::var_os("ADL_SIM03_BASELINE_BINARY")
            .expect("explicit accepted predecessor binary required"),
    );
    assert_eq!(
        blake3::hash(&fs::read(&baseline).unwrap())
            .to_hex()
            .as_str(),
        BASELINE_DIGEST,
        "unverified predecessor binary"
    );
    let mut samples = Vec::new();
    let destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/sim03-prepared-start-measurement")
        .join(format!("{}.json", std::process::id()));
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    for repetition in 0..3 {
        for predecessor_first in [true, false] {
            let predecessor = if repetition % 2 == 0 {
                predecessor_first
            } else {
                !predecessor_first
            };
            samples.push(measure(
                predecessor.then_some(baseline.as_path()),
                repetition,
            ));
            let report = json!({"schema":"csdlc.v3.prepared_start_measurement.v1","issue":869,
            "source_head":intent_fixture::git(&intent_fixture::source_root(),&["rev-parse","HEAD"]),
            "source_has_uncommitted_changes":!intent_fixture::git(&intent_fixture::source_root(),&["status","--porcelain"]).is_empty(),
            "baseline_revision":"6425ba9bbce4cc1f46789c2e3ac019adf86238b8","baseline_blake3":BASELINE_DIGEST,
            "conditions":{"os_cold_cache_measured":false,"cache_reset_performed":false,"host":"same warm host; first and later repetitions separately labeled",
            "excluded":"fixture Git/template/authority bootstrap, binary installation and plan authoring; preparation is outside prepared-start interval and retained separately",
            "remote":"synthetic local transport; no live network timing","scope":"small isolated fixture, not a live repository or human workflow benchmark"},
            "samples":samples});
            write(&destination, &report);
        }
    }
    assert_eq!(samples.len(), 6);
    assert!(samples
        .iter()
        .all(|s| s["three_minute_fixture_target_met"] == true));
    println!(
        "prepared-start measurement retained at {}",
        destination.display()
    );
}
