//! PVF runtime, deterministic CPU/filesystem fixtures and production CLI. Required acceptance gate.
use adl::codefriend::{
    evidence::{
        contracts::{
            Completion, Confidence, Delta, Finding, ReviewRecord, Run, Severity, CONTRACT,
        },
        store::Store,
        Retention,
    },
    ingestion::{local, Scope},
    memory::{
        baseline::{self, AdmittedBaselines, BaselineAccess, BaselineRef},
        comparison,
    },
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
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
    temp: tempfile::TempDir,
    source: PathBuf,
    store: Option<Store>,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = temp.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        fs::write(source.join("LICENSE"), "MIT fixture\n").unwrap();
        fs::write(source.join("helper.rs"), "pub fn helper() {}\n").unwrap();
        let clock = Arc::new(AtomicU64::new(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ));
        let c = clock.clone();
        let store =
            Store::open(&temp.path().join("store"), move || c.load(Ordering::SeqCst)).unwrap();
        Self {
            temp,
            source,
            store: Some(store),
            clock,
        }
    }
    fn store(&self) -> &Store {
        self.store.as_ref().unwrap()
    }
    fn baselines(&self) -> PathBuf {
        self.temp.path().join("baselines")
    }
    fn record(
        &self,
        source: &str,
        names: &[(&str, &str)],
        version: &str,
        completion: Completion,
        narrow: bool,
    ) -> ReviewRecord {
        fs::write(self.source.join("lib.rs"), source).unwrap();
        git(&self.source, &["add", "."]);
        git(
            &self.source,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "--allow-empty",
                "-m",
                source,
            ],
        );
        let revision = git(&self.source, &["rev-parse", "HEAD"]);
        let scope = Scope {
            analysis: if narrow {
                vec!["lib.rs".into()]
            } else {
                vec!["helper.rs".into(), "lib.rs".into()]
            },
            context: vec!["LICENSE".into()],
            max_files: 3,
            max_bytes: 65536,
            max_file_bytes: 32768,
        };
        let packet = local::acquire(
            &self.source,
            "https://example.com/owner/repo",
            &revision,
            scope,
        )
        .unwrap();
        let admission = self
            .store()
            .admit(packet, Retention { seconds: 1000 })
            .unwrap();
        let run = Run::new(
            &admission,
            BTreeMap::from([("fixture".into(), version.into())]),
            "local".into(),
            completion,
            vec![],
        )
        .unwrap();
        let findings = names
            .iter()
            .map(|(name, title)| {
                let mut f = Finding {
                    schema: CONTRACT.into(),
                    id: String::new(),
                    repository: run.repository.clone(),
                    perspective: "fixture".into(),
                    rule: "boundary".into(),
                    semantic_anchor: (*name).into(),
                    title: (*title).into(),
                    severity: Severity::Medium,
                    rationale: "Review the referenced dependency".into(),
                    confidence: Confidence::Unknown,
                    evidence: vec![admission
                        .evidence
                        .iter()
                        .find(|e| e.path == "lib.rs")
                        .unwrap()
                        .id
                        .clone()],
                    inference: "Fixture assessment".into(),
                    scope_digest: run.scope_digest.clone(),
                    limitations: vec![],
                };
                f.id = f.identity().unwrap();
                f
            })
            .collect();
        let record = ReviewRecord {
            admission,
            run,
            findings,
        };
        record.validate().unwrap();
        record
    }
}
#[test]
fn four_classifications_preserve_stable_identity_and_revision_references() {
    let f = Fixture::new();
    let old = f.record(
        "pub fn old() {}",
        &[
            ("same", "Same"),
            ("changed", "Old prose"),
            ("resolved", "Resolved"),
        ],
        "1",
        Completion::Complete,
        false,
    );
    let new = f.record(
        "\n\npub fn moved() {}",
        &[
            ("same", "Same"),
            ("changed", "New prose"),
            ("added", "Added"),
        ],
        "1",
        Completion::Complete,
        false,
    );
    let access = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = access.retain(&old).unwrap();
    let c = access.retain(&new).unwrap();
    let d = comparison::compare(&access, &b, &c).unwrap();
    assert!(d.comparable);
    assert_eq!(d, comparison::compare(&access, &b, &c).unwrap());
    assert_eq!(d.changes.len(), 4);
    for outcome in [
        Delta::Added,
        Delta::Resolved,
        Delta::Changed,
        Delta::Unchanged,
    ] {
        assert_eq!(
            d.changes
                .iter()
                .filter(|x| x.comparison.outcome == outcome)
                .count(),
            1
        );
    }
    let same = d
        .changes
        .iter()
        .find(|x| x.comparison.outcome == Delta::Unchanged)
        .unwrap();
    assert_eq!(
        same.before.as_ref().unwrap().finding_id,
        same.after.as_ref().unwrap().finding_id
    );
    assert_ne!(
        same.before.as_ref().unwrap().evidence,
        same.after.as_ref().unwrap().evidence
    );
    assert_ne!(
        same.before.as_ref().unwrap().revision,
        same.after.as_ref().unwrap().revision
    );
    assert!(d
        .changes
        .windows(2)
        .all(|x| x[0].comparison.finding_id < x[1].comparison.finding_id));
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}
#[test]
fn incompatible_and_partial_coverage_never_resolves_findings() {
    for (version, completion, narrow, reason) in [
        (
            "2",
            Completion::Complete,
            false,
            "lane_or_policy_version_mismatch",
        ),
        (
            "1",
            Completion::Incomplete,
            false,
            "incomplete_review_coverage",
        ),
        ("1", Completion::Complete, true, "coverage_scope_mismatch"),
    ] {
        let f = Fixture::new();
        let old = f.record(
            "pub fn old() {}",
            &[("old", "Old")],
            "1",
            Completion::Complete,
            false,
        );
        let new = f.record("pub fn new() {}", &[], version, completion, narrow);
        let a = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
        let d =
            comparison::compare(&a, &a.retain(&old).unwrap(), &a.retain(&new).unwrap()).unwrap();
        assert!(!d.comparable);
        assert!(d.reasons.iter().any(|r| r == reason));
        assert_eq!(d.changes.len(), 1);
        assert_eq!(d.changes[0].comparison.outcome, Delta::NotComparable);
    }
}
#[test]
fn missing_tampered_and_colliding_baselines_fail_closed() {
    let f = Fixture::new();
    let old = f.record(
        "pub fn old() {}",
        &[("old", "Old")],
        "1",
        Completion::Complete,
        false,
    );
    let a = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let r = a.retain(&old).unwrap();
    assert_eq!(r, a.retain(&old).unwrap());
    let mut bad = r.clone();
    bad.run_id = "0".repeat(64);
    assert!(a.load(&bad).unwrap_err().to_string().contains("missing"));
    bad = r.clone();
    bad.record_digest = "0".repeat(64);
    assert!(a.load(&bad).is_err());
    let mut changed = old.clone();
    changed.findings[0].title = "Changed".into();
    assert!(a.retain(&changed).is_err());
    let mut duplicate = old.clone();
    duplicate.findings.push(duplicate.findings[0].clone());
    assert!(a
        .retain(&duplicate)
        .unwrap_err()
        .to_string()
        .contains("collision"));
    let mut forged = old.clone();
    forged.run.revision = "0".repeat(40);
    assert!(a.retain(&forged).is_err());
    let p = f.baselines().join(format!("{}.json", r.run_id));
    let mut stored: serde_json::Value = baseline::read_json(&p).unwrap();
    stored["findings"] = serde_json::json!([]);
    fs::write(&p, serde_json::to_vec(&stored).unwrap()).unwrap();
    assert!(a.load(&r).is_err());
}
#[test]
fn deletion_expiry_and_saved_delta_readback_require_live_baselines() {
    for mode in ["baseline", "admission", "expiry"] {
        let f = Fixture::new();
        let old = f.record(
            "pub fn old() {}",
            &[("old", "Old")],
            "1",
            Completion::Complete,
            false,
        );
        let a = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
        let r = a.retain(&old).unwrap();
        let d = comparison::compare(&a, &r, &r).unwrap();
        d.validate(&a).unwrap();
        match mode {
            "baseline" => {
                a.delete(&r).unwrap();
                a.delete(&r).unwrap();
                assert!(a.retain(&old).is_err());
            }
            "admission" => f.store().delete(&r.packet_id).unwrap(),
            _ => {
                f.clock.fetch_add(1001, Ordering::SeqCst);
            }
        }
        assert!(a.load(&r).is_err());
        assert!(d.validate(&a).is_err());
        if mode != "baseline" {
            a.delete(&r).unwrap();
        }
    }
}
#[test]
fn backend_substitution_cannot_bypass_reference_identity() {
    let f = Fixture::new();
    let record = f.record("pub fn old() {}", &[], "1", Completion::Complete, false);
    struct Substitute(ReviewRecord);
    impl BaselineAccess for Substitute {
        fn load(&self, _: &BaselineRef) -> anyhow::Result<ReviewRecord> {
            Ok(self.0.clone())
        }
    }
    let mut reference = BaselineRef::from_record(&record).unwrap();
    reference.record_digest = "0".repeat(64);
    assert!(
        comparison::compare(&Substitute(record), &reference, &reference)
            .unwrap_err()
            .to_string()
            .contains("backend_reference")
    );
}
#[test]
fn persisted_delta_tampering_and_unsafe_paths_are_rejected() {
    let f = Fixture::new();
    let record = f.record(
        "pub fn old() {}",
        &[("old", "Old")],
        "1",
        Completion::Complete,
        false,
    );
    let a = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let r = a.retain(&record).unwrap();
    let mut d = comparison::compare(&a, &r, &r).unwrap();
    let p = f.temp.path().join("delta.json");
    baseline::write_json(&p, &d).unwrap();
    assert!(baseline::write_json(&p, &d).is_err());
    d.changes.clear();
    assert!(d.validate(&a).is_err());
    assert!(baseline::safe_path(&f.temp.path().join("../escape")).is_err());
    #[cfg(unix)]
    {
        let link = f.temp.path().join("link");
        std::os::unix::fs::symlink(&p, &link).unwrap();
        assert!(baseline::read_json::<serde_json::Value>(&link).is_err());
    }
    let bytes = fs::read(f.baselines().join(format!("{}.json", r.run_id))).unwrap();
    assert!(!String::from_utf8(bytes).unwrap().contains("pub fn old"));
}
fn cli(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .arg("codefriend")
        .arg("memory")
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn actual_cli_retains_compares_reads_and_deletes() {
    let mut f = Fixture::new();
    let old = f.record(
        "pub fn old() {}",
        &[("same", "Same"), ("resolved", "Resolved")],
        "1",
        Completion::Complete,
        false,
    );
    let new = f.record(
        "pub fn new() {}",
        &[("same", "Same"), ("added", "Added")],
        "1",
        Completion::Complete,
        false,
    );
    let store = f.temp.path().join("store");
    let baselines = f.baselines();
    let oldfile = f.temp.path().join("old.json");
    let newfile = f.temp.path().join("new.json");
    baseline::write_json(&oldfile, &old).unwrap();
    baseline::write_json(&newfile, &new).unwrap();
    drop(f.store.take());
    let store = store.to_str().unwrap();
    let baselines = baselines.to_str().unwrap();
    let mut refs = Vec::new();
    for (i, input) in [oldfile, newfile].iter().enumerate() {
        let o = cli(&[
            "retain",
            "--store",
            store,
            "--baselines",
            baselines,
            "--input",
            input.to_str().unwrap(),
        ]);
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        assert!(String::from_utf8_lossy(&o.stderr).contains("adl_event"));
        let r: BaselineRef = serde_json::from_slice(&o.stdout).unwrap();
        let p = f.temp.path().join(format!("ref{i}.json"));
        baseline::write_json(&p, &r).unwrap();
        refs.push(p);
    }
    let out = f.temp.path().join("delta.json");
    let args = [
        "compare",
        "--store",
        store,
        "--baselines",
        baselines,
        "--baseline",
        refs[0].to_str().unwrap(),
        "--current",
        refs[1].to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ];
    let result = cli(&args);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let delta: comparison::DeltaReport = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(delta.changes.len(), 3);
    assert!(delta.comparable);
    assert!(!cli(&args).status.success());
    let read = cli(&[
        "delta-read",
        "--store",
        store,
        "--baselines",
        baselines,
        "--input",
        out.to_str().unwrap(),
    ]);
    assert!(read.status.success());
    assert_eq!(
        serde_json::from_slice::<comparison::DeltaReport>(&read.stdout).unwrap(),
        delta
    );
    assert!(cli(&[
        "read",
        "--store",
        store,
        "--baselines",
        baselines,
        "--reference",
        refs[0].to_str().unwrap()
    ])
    .status
    .success());
    assert!(cli(&[
        "delete",
        "--store",
        store,
        "--baselines",
        baselines,
        "--reference",
        refs[0].to_str().unwrap()
    ])
    .status
    .success());
    assert!(!cli(&[
        "delta-read",
        "--store",
        store,
        "--baselines",
        baselines,
        "--input",
        out.to_str().unwrap()
    ])
    .status
    .success());
    assert!(!cli(&[]).status.success());
    assert!(!cli(&["unknown"]).status.success());
    assert!(!cli(&["retain", "--store", store]).status.success());
    assert!(!cli(&["read", "--store", store, "--store", store])
        .status
        .success());
    assert!(!cli(&["read", "--unexpected", "x"]).status.success());
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}
#[test]
fn invalid_roots_and_malformed_artifacts_leave_existing_content() {
    let f = Fixture::new();
    let nested = f.store().root_path().join("baselines");
    assert!(AdmittedBaselines::open(f.store(), &nested, true).is_err());
    assert!(!nested.exists());
    let existing = f.temp.path().join("existing");
    fs::create_dir(&existing).unwrap();
    fs::write(existing.join("keep"), "unchanged").unwrap();
    assert!(AdmittedBaselines::open(f.store(), &existing, true).is_err());
    assert_eq!(
        fs::read_to_string(existing.join("keep")).unwrap(),
        "unchanged"
    );
    assert!(AdmittedBaselines::open(f.store(), &f.baselines(), false).is_err());
    let p = f.temp.path().join("malformed.json");
    fs::write(&p, b"{").unwrap();
    assert!(baseline::read_json::<BaselineRef>(&p).is_err());
    assert!(baseline::read_json::<BaselineRef>(f.temp.path()).is_err());
    let r = BaselineRef {
        run_id: "../x".into(),
        packet_id: "0".repeat(64),
        record_digest: "0".repeat(64),
    };
    assert!(r.validate().is_err());
}
