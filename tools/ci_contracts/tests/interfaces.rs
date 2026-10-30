//! PVF local_contract: public validation CLI contracts, bounded Git/filesystem fixtures.
//! Production selector/manager/inventory are #1351 subjects, temporarily Python;
//! this native test contains no Python program or source-language wrapper logic.
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let f = Self {
            root: tmp.path().to_path_buf(),
            _temp: tmp,
        };
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for name in [
            "select_validation_lanes",
            "validation_manager",
            "validation_inventory",
        ] {
            for suffix in ["py", "sh"] {
                let p = format!("adl/tools/{name}.{suffix}");
                f.write(&p, &fs::read_to_string(source.join(&p)).unwrap());
            }
        }
        for name in [
            "validation_lane_selector.public.v1.json",
            "slow_proof_families.public.v1.json",
        ] {
            let p = format!("adl/config/{name}");
            f.write(&p, &fs::read_to_string(source.join(&p)).unwrap());
        }
        for package in [
            "adl",
            "adl-schema",
            "adl-uts",
            "adl-v2/crates/adl-language",
            "adl-v2/crates/adl-compiler",
            "adl-legacy-contracts",
        ] {
            f.write(
                &format!("{package}/src/lib.rs"),
                "#[test]\nfn passes() {}\n",
            );
        }
        f.write(
            "adl/Cargo.toml",
            "[package]\nname=\"adl\"\nversion=\"0.1.0\"\n",
        );
        f.write("docs/check.md", "clean\n");
        f.write("old.rs", "old\n");
        f.git(&["init", "-q"]);
        f.git(&["add", "."]);
        f.git(&[
            "-c",
            "user.name=Public fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "-qm",
            "fixture",
        ]);
        f.write("changed.txt", "adl/src/main.rs\n");
        f
    }
    fn write(&self, path: &str, text: &str) {
        let p = self.root.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, text).unwrap();
    }
    fn git(&self, args: &[&str]) -> Output {
        let o = Command::new("git")
            .args(args)
            .current_dir(&self.root)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        o
    }
    fn tool(&self, name: &str, args: &[&str]) -> Output {
        Command::new("bash")
            .arg(self.root.join(format!("adl/tools/{name}.sh")))
            .args(args)
            .current_dir(&self.root)
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .output()
            .unwrap()
    }
    fn profile(&self, args: &[&str]) -> Output {
        let mut all = vec!["--changed-files", "changed.txt", "--json"];
        all.extend(args);
        self.tool("validation_manager", &all)
    }
    fn manifest(&self) -> Value {
        serde_json::from_str(
            &fs::read_to_string(
                self.root
                    .join("adl/config/validation_lane_selector.public.v1.json"),
            )
            .unwrap(),
        )
        .unwrap()
    }
    fn save(&self, v: &Value) {
        self.write(
            "adl/config/validation_lane_selector.public.v1.json",
            &v.to_string(),
        );
    }
}
fn parsed(o: Output) -> Value {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}
#[test]
fn source_and_docs_real_entrypoints() {
    let f = Fixture::new();
    let p = parsed(f.tool(
        "select_validation_lanes",
        &["--changed-files", "changed.txt", "--json"],
    ));
    assert_eq!(
        p["lanes"].as_object().unwrap().keys().collect::<Vec<_>>(),
        vec!["public_source"]
    );
    let p = parsed(f.profile(&[]));
    assert_eq!(p["status"], "ready_to_run");
    assert_eq!(p["run"][0]["command"], "bash adl/tools/batched_checks.sh");
    let mut deferred = p["not_run"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["surface"].as_str().unwrap())
        .collect::<Vec<_>>();
    deferred.sort();
    assert_eq!(
        deferred,
        vec!["coverage", "installed_public", "owner_contracts"]
    );
    f.write("changed.txt", "docs/check.md\n");
    assert_eq!(parsed(f.profile(&["--run"]))["run_status"], "passed");
}
#[test]
fn workspace_only_selects_public() {
    let f = Fixture::new();
    for path in ["adl-v2/Cargo.toml", "adl-v2/Cargo.lock"] {
        f.write("changed.txt", path);
        let p = parsed(f.profile(&[]));
        assert_eq!(p["status"], "ready_to_run");
        assert_eq!(p["run"].as_array().unwrap().len(), 1);
        assert_eq!(p["run"][0]["command"], "bash adl/tools/batched_checks.sh");
    }
}
#[test]
fn unknown_and_private_refuse_execution() {
    let f = Fixture::new();
    for paths in [
        "unmapped.bin\n",
        "adl/src/runtime_v2/mod.rs\n",
        "adl/src/main.rs\nunmapped.bin\n",
    ] {
        f.write("changed.txt", paths);
        let out = f.profile(&["--run"]);
        assert!(!out.status.success());
        let p: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(p["status"], "escalation_required");
    }
}
#[test]
fn failure_report_and_stdout_separation() {
    let f = Fixture::new();
    let mut m = f.manifest();
    m["special_surfaces"]["rust_pr_fast"]["run_command"] = json!("exit 7");
    f.save(&m);
    let out = f.profile(&[
        "--run",
        "--report-out",
        "report.json",
        "--build-action-log-dir",
        "logs",
    ]);
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["run_status"],
        "failed"
    );
    let report: Value =
        serde_json::from_str(&fs::read_to_string(f.root.join("report.json")).unwrap()).unwrap();
    assert_eq!(report["run_status"], "failed");
    m["special_surfaces"]["rust_pr_fast"]["run_command"] = json!("printf '%s\\n' child-output");
    f.save(&m);
    let out = f.tool(
        "select_validation_lanes",
        &["--changed-files", "changed.txt", "--json", "--run"],
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("child-output"));
    assert_eq!(parsed(out)["run_status"], "passed");
}
#[test]
fn manifest_metadata_and_vpp_fail_closed() {
    let f = Fixture::new();
    let baseline = f.manifest();
    let mut invalid = vec![];
    for key in [
        "surface_defaults",
        "lanes",
        "release_gate_hints",
        "rust_path_hints",
    ] {
        let mut v = baseline.clone();
        v.as_object_mut().unwrap().remove(key);
        invalid.push(v);
    }
    for key in [
        "owner",
        "resource_class",
        "determinism_posture",
        "proof_role",
        "risk_class",
        "escalation_rule",
    ] {
        let mut v = baseline.clone();
        v["surface_defaults"]["public"][key] = json!("");
        invalid.push(v);
    }
    for record in [json!({}), json!({"contract_version":"v1","artifacts":[]})] {
        let mut v = baseline.clone();
        v["lanes"][0]["vpp_record"] = record;
        invalid.push(v);
    }
    let mut v = baseline.clone();
    v["special_surfaces"]["rust_pr_fast"]["planner"] = json!("unknown");
    invalid.push(v);
    for v in invalid {
        f.save(&v);
        assert!(!f
            .tool(
                "select_validation_lanes",
                &["--changed-files", "changed.txt", "--json"]
            )
            .status
            .success());
    }
}
#[test]
fn valid_metadata_and_vpp_preserved() {
    let f = Fixture::new();
    let mut m = f.manifest();
    let record = json!({"contract_version":"v1","artifacts":["report.json"],"expected_runtime_class":"small","parallel_group":"public","cache_equivalence_group":"public","failure_semantics":"fail_closed"});
    m["special_surfaces"]["rust_pr_fast"]["vpp_record"] = record.clone();
    f.save(&m);
    let p = parsed(f.tool(
        "select_validation_lanes",
        &["--changed-files", "changed.txt", "--json"],
    ));
    assert_eq!(p["lanes"]["public_source"]["vpp_record"], record);
    assert_eq!(p["lanes"]["public_source"]["owner"], "adl");
}
#[test]
fn git_deletion_rename_and_untracked() {
    let f = Fixture::new();
    f.git(&["mv", "old.rs", "renamed.rs"]);
    fs::remove_file(f.root.join("adl/src/lib.rs")).unwrap();
    f.write("docs/untracked.md", "new\n");
    let p = parsed(f.tool(
        "select_validation_lanes",
        &[
            "--base",
            "HEAD",
            "--head",
            "HEAD",
            "--include-working-tree",
            "--json",
        ],
    ));
    for path in [
        "old.rs",
        "renamed.rs",
        "adl/src/lib.rs",
        "docs/untracked.md",
    ] {
        assert!(p["changed_paths"]
            .as_array()
            .unwrap()
            .contains(&json!(path)));
    }
    assert!(!f
        .tool(
            "select_validation_lanes",
            &["--base", "not-a-revision", "--head", "HEAD", "--json"]
        )
        .status
        .success());
}
#[test]
fn public_fixtures_select_and_private_escalates() {
    let f = Fixture::new();
    for path in [
        "adl/tests/fixtures/cli_smoke.adl.yaml",
        "adl/examples/adl-0.1.yaml",
        "adl/examples/v0-2-multi-step-basic.adl.yaml",
        "adl/examples/v0-2-multi-step-file-input.adl.yaml",
        "adl/examples/v0-2-coordinator-agents-sdk.adl.yaml",
    ] {
        f.write("changed.txt", path);
        let p = parsed(f.profile(&[]));
        assert_eq!(p["status"], "ready_to_run");
        assert_eq!(p["run"][0]["lane_id"], "public_source");
    }
    f.write("changed.txt", "adl/tests/fixtures/private_runtime.adl.yaml");
    assert_eq!(parsed(f.profile(&[]))["status"], "escalation_required");
}
#[test]
fn six_package_inventory_and_missing_inputs() {
    let f = Fixture::new();
    let p = parsed(f.tool("validation_inventory", &["--format", "json"]));
    assert_eq!(p["rust"]["rust_lib_unit_tests"]["test_entry_count"], 6);
    assert_eq!(p["rust"]["owner_summary"], json!({"adl":6}));
    assert_eq!(p["slow_proof_family_inventory"], json!([]));
    assert!(!f
        .tool(
            "validation_inventory",
            &["--slow-proof-families", "missing.json", "--format", "json"]
        )
        .status
        .success());
    assert!(!f
        .profile(&["--slow-proof-families", "missing.json"])
        .status
        .success());
}
#[test]
fn historical_disposition_cannot_bypass_public_escalation() {
    let f = Fixture::new();
    f.write(
        "changed.txt",
        ".github/workflows/ci.yaml\nadl/tools/test_ci_runtime_contracts.sh\n",
    );
    let p = parsed(f.profile(&[]));
    assert_eq!(p["status"], "escalation_required");
    assert_eq!(p["pr_publication_sufficient"], false);
    assert!(p["run"].as_array().unwrap().is_empty());
    f.write("historical.json",&json!({"schema_version":"adl.slow_proof_families.v1","families":[{"id":"private-runtime"}]}).to_string());
    let out = f.profile(&["--slow-proof-families", "historical.json"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("retired private slow-proof families"));
    let mut m = f.manifest();
    m["special_surfaces"]["rust_pr_fast"]["planner"] = json!("pr_fast");
    f.save(&m);
    assert!(!f.profile(&[]).status.success());
}
#[test]
fn shell_and_javascript_syntax_failure() {
    let f = Fixture::new();
    for (program, name, valid, invalid) in [
        ("bash -n", "syntax.sh", "true\n", "if then\n"),
        (
            "node --check",
            "syntax.mjs",
            "export const valid = true;\n",
            "const = ;\n",
        ),
    ] {
        let mut m = f.manifest();
        m["special_surfaces"]["rust_pr_fast"]["run_command"] = json!(format!("{program} {name}"));
        f.save(&m);
        f.write(name, valid);
        assert!(f.profile(&["--run"]).status.success());
        f.write(name, invalid);
        assert!(!f.profile(&["--run"]).status.success());
    }
}
#[cfg(unix)]
#[test]
fn aws_spot_requires_executable_owner_runner() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let runner = f.root.join("owner runner");
    f.write("owner runner", "#!/bin/sh\nexit 99\n");
    let directory = f.root.to_string_lossy().into_owned();
    let missing = f.root.join("missing").to_string_lossy().into_owned();
    let runner = runner.to_string_lossy().into_owned();
    for candidate in [
        "relative/runner",
        missing.as_str(),
        directory.as_str(),
        runner.as_str(),
    ] {
        let p = parsed(f.profile(&[
            "--validation-platform",
            "aws_spot",
            "--aws-spot-owner-runner",
            candidate,
        ]));
        let candidates = p["platform_routing"]["candidates"].as_array().unwrap();
        let aws = candidates
            .iter()
            .find(|v| v["platform"] == "aws_spot")
            .unwrap();
        assert_eq!(aws["decision"], "rejected");
        assert!(aws["command"].is_null());
    }
    fs::set_permissions(&runner, fs::Permissions::from_mode(0o755)).unwrap();
    let p = parsed(f.profile(&[
        "--validation-platform",
        "aws_spot",
        "--aws-spot-owner-runner",
        &runner,
    ]));
    let candidates = p["platform_routing"]["candidates"].as_array().unwrap();
    let aws = candidates
        .iter()
        .find(|v| v["platform"] == "aws_spot")
        .unwrap();
    assert_eq!(aws["decision"], "eligible");
    let argv = shell_words::split(aws["command"].as_str().unwrap()).unwrap();
    assert_eq!(argv[0], runner);
    assert!(argv.contains(&"--print-command".into()));
    f.write("changed.txt", "unknown.bin");
    let p = parsed(f.profile(&[
        "--validation-platform",
        "aws_spot",
        "--aws-spot-owner-runner",
        &runner,
    ]));
    assert_eq!(
        p["platform_routing"]["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["platform"] == "aws_spot")
            .unwrap()["decision"],
        "rejected"
    );
}
