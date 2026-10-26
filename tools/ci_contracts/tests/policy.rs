//! PVF local_contract: deterministic policy and aggregation fixtures; bounded
//! local CPU/disk, no provider/network; required focused prepublication gate.
use adl_ci_contracts::{aggregation_contract, workflow_policy};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn source() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
struct Fixture {
    dir: tempfile::TempDir,
    ci: String,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let source = source();
        fs::create_dir_all(root.join(".github/workflows")).unwrap();
        fs::create_dir_all(root.join("tools/public_adl")).unwrap();
        for e in fs::read_dir(source.join(".github/workflows")).unwrap() {
            let p = e.unwrap().path();
            if p.is_file() {
                fs::copy(
                    &p,
                    root.join(".github/workflows").join(p.file_name().unwrap()),
                )
                .unwrap();
            }
        }
        for p in [
            "rust-toolchain.toml",
            "tools/public_adl/run_ci.sh",
            "tools/public_adl/run_package_checks.sh",
        ] {
            fs::copy(source.join(p), root.join(p)).unwrap();
        }
        let ci = fs::read_to_string(root.join(".github/workflows/ci.yaml")).unwrap();
        Self { dir, ci }
    }
    fn root(&self) -> &Path {
        self.dir.path()
    }
    fn change(&self, text: &str) {
        fs::write(self.root().join(".github/workflows/ci.yaml"), text).unwrap();
    }
    fn rejects(&self, reason: &str) {
        let v = workflow_policy(self.root()).unwrap();
        assert_eq!(v["status"], "fail", "{v}");
        assert!(
            v["errors"]
                .as_array()
                .unwrap()
                .iter()
                .any(|e| e.as_str().unwrap().contains(reason)),
            "{v}"
        );
    }
}
#[test]
fn valid_complete_workflow() {
    let f = Fixture::new();
    assert_eq!(workflow_policy(f.root()).unwrap()["status"], "pass");
}
macro_rules! deadline {
    ($name:ident,$addition:expr) => {
        #[test]
        fn $name() {
            let f = Fixture::new();
            f.change(&f.ci.replacen(
                "  public_adl_coverage:\n",
                concat!("  public_adl_coverage:\n", $addition),
                1,
            ));
            f.rejects("deadline");
        }
    };
}
deadline!(original_35_minute_deadline, "    timeout-minutes: 35\n");
deadline!(quoted_deadline_key, "    'timeout-minutes': 35\n");
deadline!(
    expression_deadline,
    "    timeout-minutes: ${{ vars.COVERAGE_TIMEOUT }}\n"
);
deadline!(null_deadline, "    timeout-minutes:\n");
#[test]
fn malformed_yaml_deadline_refusal() {
    let f = Fixture::new();
    f.change(&(f.ci.clone() + "\nbroken: [\n"));
    f.rejects("deadline");
}
#[test]
fn step_deadline() {
    let f = Fixture::new();
    let (a, b) = f.ci.split_once("  public_adl_coverage:\n").unwrap();
    f.change(&format!("{a}  public_adl_coverage:\n{}",b.replacen("    steps:\n","    steps:\n      - name: shorter coverage deadline\n        timeout-minutes: 35\n        run: echo fixture\n",1)));
    f.rejects("deadline");
}
#[test]
fn automatic_fanout_refused() {
    let f = Fixture::new();
    let p = f
        .root()
        .join(".github/workflows/nightly-coverage-ratchet.yaml");
    let s = fs::read_to_string(&p).unwrap();
    fs::write(p, s.replacen("on:\n", "on:\n  workflow_run:\n", 1)).unwrap();
    f.rejects("not allowed");
}
#[test]
fn floating_toolchain_refused() {
    let f = Fixture::new();
    f.change(&f.ci.replacen("toolchain: 1.92.0", "toolchain: stable", 1));
    f.rejects("must pin");
}
#[test]
fn missing_toolchain_refused() {
    let f = Fixture::new();
    f.change(&f.ci.replacen("          toolchain: 1.92.0\n", "", 1));
    f.rejects("must pin");
}
#[test]
fn classifier_runner_bypass_refused() {
    let f = Fixture::new();
    f.change(&f.ci.replacen(
        "runs-on: ubuntu-latest",
        "runs-on: ${{ vars.ADL_HEAVY_RUNNER || 'adl-ubuntu-24.04-16core' }}",
        1,
    ));
    f.rejects("must use");
}
fn public_runner(runner: &str) {
    let f = Fixture::new();
    let (a, b) = f.ci.split_once("  public_adl_validation:\n").unwrap();
    f.change(&format!(
        "{a}  public_adl_validation:\n{}",
        b.replacen("runs-on: macos-latest", &format!("runs-on: {runner}"), 1)
    ));
    f.rejects("macos-latest");
}
#[test]
fn sandbox_producer_ubuntu_refused() {
    public_runner("ubuntu-latest");
}
#[test]
fn sandbox_producer_self_hosted_refused() {
    public_runner("self-hosted");
}
#[test]
fn six_aggregation_outcomes_and_declaration_scope() {
    aggregation_contract(Fixture::new().root()).unwrap();
}
#[test]
fn disconnected_uts_producer_refused() {
    let f = Fixture::new();
    let p = f.root().join("tools/public_adl/run_package_checks.sh");
    let s = fs::read_to_string(&p).unwrap();
    fs::write(p, s.replace("  adl-uts\n", "")).unwrap();
    assert_eq!(
        aggregation_contract(f.root()).unwrap_err(),
        "UTS package dropped"
    );
}
#[test]
fn wrong_aggregate_binding_refused() {
    let f = Fixture::new();
    f.change(&f.ci.replace(
        "PUBLIC_RESULT: ${{ needs.public_adl_validation.result }}",
        "PUBLIC_RESULT: success",
    ));
    assert_eq!(
        aggregation_contract(f.root()).unwrap_err(),
        "wrong binding PUBLIC_RESULT"
    );
}
#[test]
fn stable_status_names_and_no_aws_in_public_producers() {
    let f = Fixture::new();
    let v: serde_yaml::Value = serde_yaml::from_str(&f.ci).unwrap();
    let jobs = v["jobs"].as_mapping().unwrap();
    for name in ["adl-ci", "adl-coverage"] {
        assert_eq!(
            jobs.values()
                .filter(|j| j["name"].as_str() == Some(name))
                .count(),
            1
        );
    }
    let aws = regex::Regex::new(r"(?i)(?:^|\s)aws(?:\s|$)").unwrap();
    for name in ["public_adl_validation", "public_adl_coverage"] {
        for step in v["jobs"][name]["steps"].as_sequence().unwrap() {
            assert!(!aws.is_match(step["run"].as_str().unwrap_or("")));
        }
    }
}
#[cfg(unix)]
#[test]
fn policy_and_aggregation_do_not_execute_interpreters() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let bin = f.root().join("bin");
    fs::create_dir(&bin).unwrap();
    let marker = f.root().join("interpreter-called");
    for program in ["ruby", "python", "python3"] {
        let trap = bin.join(program);
        fs::write(
            &trap,
            "#!/bin/sh\nprintf called > \"$INTERPRETER_CALLED\"\nexit 99\n",
        )
        .unwrap();
        fs::set_permissions(&trap, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap());
    for action in ["workflow-policy", "aggregation-contract"] {
        let o = Command::new(env!("CARGO_BIN_EXE_adl-ci-contracts"))
            .arg(action)
            .arg(f.root())
            .env("PATH", &path)
            .env("INTERPRETER_CALLED", &marker)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    }
    assert!(!marker.exists());
}
