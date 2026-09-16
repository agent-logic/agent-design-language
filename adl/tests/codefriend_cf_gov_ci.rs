//! PVF runtime: deterministic local contract/security proof, small resource profile,
//! required release gate; no provider or network execution. See fixtures/codefriend/fitness-ci/PVF.json.
use adl::codefriend::{
    evidence::{store::Store, Retention},
    governance::local::*,
    ingestion::{local, Scope},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
// Each fixture launches Git/CLI subprocesses while using exclusive store locks.
// Isolate their lifetimes so a concurrent fork cannot retain another fixture lock.
static FIXTURE_PROCESSES: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap().trim().into()
}
struct Fixture {
    dir: tempfile::TempDir,
    packet: String,
    root: PathBuf,
}
impl Fixture {
    fn new(source: &str) -> Self {
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let root = dir.path().join("source");
        fs::create_dir(&root).unwrap();
        git(&root, &["init", "-b", "main"]);
        git(
            &root,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        fs::write(root.join("lib.rs"), source).unwrap();
        git(&root, &["add", "."]);
        git(
            &root,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "-m",
                "fixture",
            ],
        );
        let revision = git(&root, &["rev-parse", "HEAD"]);
        let packet = local::acquire(
            &root,
            "https://example.com/owner/repo",
            &revision,
            Scope {
                analysis: vec!["lib.rs".into()],
                context: vec![],
                max_files: 4,
                max_bytes: 100000,
                max_file_bytes: 100000,
            },
        )
        .unwrap();
        let store = Store::open(&dir.path().join("store"), || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap();
        let packet = store
            .admit(packet, Retention { seconds: 3600 })
            .unwrap()
            .packet
            .packet_id;
        Self { dir, packet, root }
    }
    fn store(&self) -> Store {
        Store::open(&self.dir.path().join("store"), || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap()
    }
    fn report(&self) -> Report {
        local_fitness_runner(&self.store(), &self.packet, policy()).unwrap()
    }
    fn cli(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "fitness"])
            .args(args)
            .output()
            .unwrap()
    }
}
fn policy() -> Policy {
    Policy {
        schema: VERSION.into(),
        rules: vec![Rule {
            id: "boundary".into(),
            kind: "forbidden_declared_use".into(),
            source_path: "lib.rs".into(),
            forbidden_prefix: "crate::forbidden".into(),
        }],
    }
}

use adl::codefriend::{
    evidence::hash,
    governance::ci::{verify, Expected, Receipt},
};
fn expected(f: &Fixture) -> Expected {
    Expected {
        candidate: git(&f.root, &["rev-parse", "HEAD"]),
        packet_id: f.packet.clone(),
        policy_digest: hash(&policy()).unwrap(),
    }
}
fn ci_run(f: &Fixture, output: &Path, policy_path: &Path) -> Output {
    let e = expected(f);
    Command::new("bash")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tools/codefriend_fitness_ci.sh"
        ))
        .args([
            "--binary",
            env!("CARGO_BIN_EXE_adl"),
            "--store",
            f.dir.path().join("store").to_str().unwrap(),
            "--packet-id",
            &e.packet_id,
            "--policy",
            policy_path.to_str().unwrap(),
            "--candidate",
            &e.candidate,
            "--policy-digest",
            &e.policy_digest,
            "--out",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap()
}
#[test]
fn installed_process_contract_preserves_pass_fail_error_and_local_parity() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    for (source, code) in [
        ("pub fn safe() {}", 0),
        ("use crate::forbidden::Thing;", 1),
        ("include!(\"undeclared.rs\");", 2),
    ] {
        let f = Fixture::new(source);
        let p = f.dir.path().join("policy.json");
        fs::write(&p, serde_json::to_vec(&policy()).unwrap()).unwrap();
        let output = f.dir.path().join("ci");
        let result = ci_run(&f, &output, &p);
        assert_eq!(
            result.status.code(),
            Some(code),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let receipt: Receipt = serde_json::from_slice(&result.stdout).unwrap();
        assert!(receipt.artifact_valid);
        assert_eq!(receipt.original_exit, code);
        assert_eq!(receipt.exit_code, code);
        let report: Report =
            serde_json::from_slice(&fs::read(output.join("report.json")).unwrap()).unwrap();
        assert_eq!(report, f.report());
        assert_eq!(
            receipt,
            serde_json::from_slice::<Receipt>(&fs::read(output.join("receipt.json")).unwrap())
                .unwrap()
        );
        assert_eq!(
            fs::read_to_string(output.join("runner-exit.txt")).unwrap(),
            format!("{code}\n")
        );
        assert!(String::from_utf8_lossy(&result.stderr).contains("adl_event"));
        assert!(!String::from_utf8_lossy(&result.stderr).contains(f.dir.path().to_str().unwrap()));
        let again = ci_run(&f, &output, &p);
        assert_eq!(again.status.code(), Some(2));
        assert_eq!(
            report,
            serde_json::from_slice::<Report>(&fs::read(output.join("report.json")).unwrap())
                .unwrap()
        );
    }
}
#[test]
fn independently_pinned_identities_and_original_status_are_required() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    let report = f.report();
    let expected = expected(&f);
    let store = f.store();
    assert!(verify(&store, &report, &expected, 0).is_ok());
    for which in 0..6 {
        let mut altered = expected.clone();
        match which {
            0 => altered.packet_id = "0".repeat(64),
            1 => altered.candidate = "0".repeat(40),
            2 => altered.policy_digest = "0".repeat(64),
            3 => altered.packet_id = "bad".into(),
            4 => altered.candidate = "branch".into(),
            _ => altered.policy_digest = "bad".into(),
        };
        assert!(verify(&store, &report, &altered, 0).is_err());
    }
    for code in [-1, 1, 2, 99] {
        assert!(verify(&store, &report, &expected, code).is_err());
    }
}
#[test]
fn missing_truncated_tampered_and_stale_artifacts_fail_closed() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    let e = expected(&f);
    let report = f.report();
    for (case, bytes) in [
        None,
        Some(Vec::new()),
        Some(b"{\"schema\":".to_vec()),
        Some({
            let mut r = report.clone();
            r.policy_digest = "0".repeat(64);
            serde_json::to_vec(&r).unwrap()
        }),
        Some(serde_json::to_vec(&report).unwrap()),
    ]
    .into_iter()
    .enumerate()
    {
        let input = f.dir.path().join(format!("input-{case}.json"));
        if let Some(bytes) = bytes {
            fs::write(&input, bytes).unwrap();
        }
        let output = f.dir.path().join(format!("receipt-{case}.json"));
        let code = if case == 4 { "1" } else { "0" };
        let result = f.cli(&[
            "ci-verify",
            "--store",
            f.dir.path().join("store").to_str().unwrap(),
            "--input",
            input.to_str().unwrap(),
            "--candidate",
            &e.candidate,
            "--packet-id",
            &e.packet_id,
            "--policy-digest",
            &e.policy_digest,
            "--runner-exit",
            code,
            "--out",
            output.to_str().unwrap(),
        ]);
        assert_eq!(result.status.code(), Some(2));
        let receipt: Receipt = serde_json::from_slice(&result.stdout).unwrap();
        assert!(!receipt.artifact_valid);
        assert_eq!(receipt.original_exit, code.parse::<i32>().unwrap());
        assert_eq!(
            receipt.error.as_deref(),
            Some("fitness_ci_contract_rejected")
        );
    }
}
#[test]
fn command_failure_without_report_retains_original_error() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    let out = f.dir.path().join("ci");
    let result = ci_run(&f, &out, &f.dir.path().join("missing-policy.json"));
    assert_eq!(result.status.code(), Some(2));
    let r: Receipt = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(r.original_exit, 2);
    assert!(!r.artifact_valid);
    assert!(!out.join("report.json").exists());
    let declared: Expected =
        serde_json::from_slice(&fs::read(out.join("expectations.json")).unwrap()).unwrap();
    assert_eq!(declared.candidate, expected(&f).candidate);
    assert_eq!(declared.packet_id, f.packet);
    assert_eq!(declared.policy_digest, expected(&f).policy_digest);
    assert_eq!(
        fs::read_to_string(out.join("runner-exit.txt")).unwrap(),
        "2\n"
    );
}
#[test]
fn malformed_cli_arguments_and_managed_output_are_denied() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    for args in [
        vec!["ci-run"],
        vec!["ci-verify", "--unknown", "x"],
        vec!["ci-run", "--store", "x", "--store", "y"],
        vec!["ci-verify", "--store"],
    ] {
        assert_eq!(f.cli(&args).status.code(), Some(2));
    }
    let p = f.dir.path().join("policy.json");
    fs::write(&p, serde_json::to_vec(&policy()).unwrap()).unwrap();
    let out = f.dir.path().join("store").join("bad-output");
    assert_eq!(ci_run(&f, &out, &p).status.code(), Some(2));
    assert!(!out.exists());
}

#[test]
fn unrelated_executable_zero_without_artifacts_is_not_success() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    let e = expected(&f);
    let out = f.dir.path().join("missing-output");
    let result = Command::new("bash")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tools/codefriend_fitness_ci.sh"
        ))
        .args([
            "--binary",
            "/usr/bin/true",
            "--store",
            f.dir.path().join("store").to_str().unwrap(),
            "--packet-id",
            &e.packet_id,
            "--policy",
            "unused.json",
            "--candidate",
            &e.candidate,
            "--policy-digest",
            &e.policy_digest,
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    assert!(!out.exists());
    assert!(String::from_utf8_lossy(&result.stderr).contains("codefriend_fitness_ci_transport"));
}

#[test]
fn optimized_python_rejects_mismatched_receipt_identity() {
    let _guard = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("pub fn ok() {}");
    let e = expected(&f);
    let out = f.dir.path().join("ci");
    let p = f.dir.path().join("policy.json");
    fs::write(&p, serde_json::to_vec(&policy()).unwrap()).unwrap();
    assert_eq!(ci_run(&f, &out, &p).status.code(), Some(0));
    let receipt_path = out.join("receipt.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["candidate"] = serde_json::json!("f".repeat(40));
    fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    for optimization in ["0", "1", "2"] {
        let result = Command::new("python3")
            .env("PYTHONOPTIMIZE", optimization)
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tools/codefriend_fitness_ci_artifacts.py"
            ))
            .args([
                "after",
                "0",
                "--store",
                f.dir.path().join("store").to_str().unwrap(),
                "--packet-id",
                &e.packet_id,
                "--policy",
                p.to_str().unwrap(),
                "--candidate",
                &e.candidate,
                "--policy-digest",
                &e.policy_digest,
                "--out",
                out.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(2), "optimization={optimization}");
        assert!(String::from_utf8_lossy(&result.stderr).contains("codefriend_fitness_ci_transport"));
    }
}
