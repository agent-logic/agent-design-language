//! PVF runtime: deterministic local contract/security proof, small resource profile,
//! required release gate; no provider or network execution. See LOCAL_FITNESS_PROOF_INVENTORY.json.
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
    fn run(&self, output: &Path) -> Output {
        let p = self.dir.path().join("policy.json");
        fs::write(&p, serde_json::to_vec(&policy()).unwrap()).unwrap();
        self.cli(&[
            "run",
            "--store",
            self.dir.path().join("store").to_str().unwrap(),
            "--packet-id",
            &self.packet,
            "--policy",
            p.to_str().unwrap(),
            "--out",
            output.to_str().unwrap(),
        ])
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
#[test]
fn literal_imports_pass_fail_and_stable_evidence() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    let good = Fixture::new("use crate::allowed::Thing;\n");
    assert_eq!(good.report().status, Status::Pass);
    let bad = Fixture::new("use crate::{forbidden::{Thing, Other as Renamed}, allowed::Fine};\n");
    let report = bad.report();
    assert_eq!(report.status, Status::Fail);
    assert_eq!(report.violations.len(), 2);
    assert_eq!(report.record.findings.len(), 2);
    assert!(report
        .violations
        .iter()
        .all(|v| v.line == 1 && !v.evidence_id.is_empty()));
    assert_eq!(report, bad.report());
    report.validate(&bad.store()).unwrap();
    assert_eq!(
        fs::read_to_string(bad.root.join("lib.rs")).unwrap(),
        "use crate::{forbidden::{Thing, Other as Renamed}, allowed::Fine};\n"
    );
}
#[test]
fn raw_identifiers_do_not_bypass_boundary() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    for source in [
        "use crate::r#forbidden::Thing;",
        "use crate::{r#forbidden::Thing as Alias};",
        "use crate::r#forbidden::*;",
    ] {
        let report = Fixture::new(source).report();
        assert_eq!(report.status, Status::Fail, "{source}");
        assert!(report.violations[0]
            .reference
            .starts_with("crate::forbidden"));
    }
}
#[test]
fn unprovable_inputs_are_errors_not_passes() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    for source in [
        "use super::Thing;",
        "use self::Thing;",
        "use crate::*;",
        "generate!();",
        "this is not rust",
        &"!".repeat(12000),
        &" ".repeat(33000),
    ] {
        let report = Fixture::new(source).report();
        assert_eq!(report.status, Status::Error, "{source:.50}");
        assert!(!report.errors.is_empty());
        assert_eq!(report.status.exit_code(), 2);
    }
    let f = Fixture::new("use crate::allowed::Thing;");
    let revision = git(&f.root, &["rev-parse", "HEAD"]);
    let partial = local::acquire(
        &f.root,
        "https://example.com/owner/repo",
        &revision,
        Scope {
            analysis: vec!["lib.rs".into(), "missing.rs".into()],
            context: vec![],
            max_files: 4,
            max_bytes: 100,
            max_file_bytes: 100,
        },
    )
    .unwrap();
    let store = f.store();
    let admitted = store.admit(partial, Retention { seconds: 3600 }).unwrap();
    let result = local_fitness_runner(&store, &admitted.packet.packet_id, policy()).unwrap();
    assert_eq!(result.status, Status::Error);
    assert!(result
        .errors
        .iter()
        .any(|e| e == "incomplete_admitted_evidence"));
    drop(store);
    let mut p = policy();
    p.rules[0].source_path = "missing.rs".into();
    assert_eq!(
        local_fitness_runner(&f.store(), &f.packet, p)
            .unwrap()
            .status,
        Status::Error
    );
}
#[test]
fn invalid_policy_is_rejected_and_policy_is_run_identity() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("use crate::allowed::Thing;");
    let original = f.report();
    let mut p = policy();
    p.rules[0].forbidden_prefix = "crate::other".into();
    let changed = local_fitness_runner(&f.store(), &f.packet, p).unwrap();
    assert_ne!(original.record.run.id, changed.record.run.id);
    for (field, value) in [
        ("kind", "shell"),
        ("id", ""),
        ("source_path", "../lib.rs"),
        ("forbidden_prefix", "super::bad"),
        ("forbidden_prefix", "crate::::bad"),
    ] {
        let mut v = serde_json::to_value(policy()).unwrap();
        v["rules"][0][field] = value.into();
        assert!(serde_json::from_value::<Policy>(v)
            .unwrap()
            .validate()
            .is_err());
    }
    let mut p = policy();
    p.rules.push(p.rules[0].clone());
    assert!(p.validate().is_err());
    p.rules.clear();
    assert!(p.validate().is_err());
    let mut p = policy();
    p.schema = "future".into();
    assert!(p.validate().is_err());
    let mut v = serde_json::to_value(policy()).unwrap();
    v["execute"] = "hidden command".into();
    assert!(serde_json::from_value::<Policy>(v).is_err());
}
#[test]
fn tamper_and_deleted_admission_fail_readback() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("use crate::forbidden::Thing;");
    let mut r = f.report();
    r.status = Status::Pass;
    assert!(r.validate(&f.store()).is_err());
    let r = f.report();
    f.store().delete(&f.packet).unwrap();
    assert!(r.validate(&f.store()).is_err());
}
#[test]
fn cli_reports_distinct_outcomes_and_create_only_artifacts() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    for (source, code) in [
        ("use crate::allowed::Thing;", 0),
        ("use crate::forbidden::Thing;", 1),
        ("generate!();", 2),
    ] {
        let f = Fixture::new(source);
        let path = f.dir.path().join("report.json");
        let out = f.run(&path);
        assert_eq!(
            out.status.code(),
            Some(code),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let report: Report = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(report.status.exit_code(), code);
        assert!(String::from_utf8_lossy(&out.stderr).starts_with("adl_event"));
        let read = f.cli(&[
            "read",
            "--store",
            f.dir.path().join("store").to_str().unwrap(),
            "--input",
            path.to_str().unwrap(),
        ]);
        assert_eq!(read.status.code(), Some(code));
        assert_eq!(read.stdout, out.stdout);
        let bytes = fs::read(&path).unwrap();
        assert_eq!(f.run(&path).status.code(), Some(2));
        assert_eq!(bytes, fs::read(&path).unwrap());
        assert_eq!(
            f.run(&f.dir.path().join("store/forbidden.json"))
                .status
                .code(),
            Some(2)
        );
        assert!(!f.dir.path().join("store/forbidden.json").exists());
    }
}
#[test]
fn cli_invalid_arguments_and_policy_are_machine_readable_errors() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("");
    let invalid = f.dir.path().join("invalid-policy.json");
    for bytes in [
        b"{invalid".as_slice(),
        br#"{"schema":"codefriend.fitness.v1","rules":[],"execute":"forbidden"}"#.as_slice(),
    ] {
        fs::write(&invalid, bytes).unwrap();
        let out = f.cli(&[
            "run",
            "--store",
            f.dir.path().join("store").to_str().unwrap(),
            "--packet-id",
            &f.packet,
            "--policy",
            invalid.to_str().unwrap(),
            "--out",
            f.dir.path().join("absent.json").to_str().unwrap(),
        ]);
        assert_eq!(out.status.code(), Some(2));
        assert!(!f.dir.path().join("absent.json").exists());
    }
    for args in [
        vec![],
        vec!["unknown"],
        vec!["run", "--bad", "value"],
        vec!["read", "--store", "x", "--store", "y"],
    ] {
        let out = f.cli(&args);
        assert_eq!(out.status.code(), Some(2));
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["status"], "error");
        assert!(!String::from_utf8_lossy(&out.stderr).contains(f.dir.path().to_str().unwrap()));
    }
}

#[cfg(unix)]
#[test]
fn cli_rejects_symlink_and_oversized_input() {
    let _isolation = FIXTURE_PROCESSES.lock().unwrap();
    let f = Fixture::new("");
    let destination = f.dir.path().join("outside.json");
    fs::write(&destination, b"preserve").unwrap();
    let alias = f.dir.path().join("alias.json");
    std::os::unix::fs::symlink(&destination, &alias).unwrap();
    assert_eq!(f.run(&alias).status.code(), Some(2));
    assert_eq!(fs::read(&destination).unwrap(), b"preserve");
    let oversized = f.dir.path().join("oversized.json");
    fs::write(&oversized, vec![b' '; 128 * 1024 + 1]).unwrap();
    let out = f.cli(&[
        "run",
        "--store",
        f.dir.path().join("store").to_str().unwrap(),
        "--packet-id",
        &f.packet,
        "--policy",
        oversized.to_str().unwrap(),
        "--out",
        f.dir.path().join("absent.json").to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(!f.dir.path().join("absent.json").exists());
}
