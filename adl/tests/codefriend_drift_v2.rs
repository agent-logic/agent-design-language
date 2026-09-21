//! PVF: deterministic both-original-owner drift; no complete comparison claim.
use adl::codefriend::{
    architecture::structure_v2::*,
    evidence::{contracts::Completion, store::Store, Retention},
    ingestion::{local, Scope},
    language::*,
};
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
fn git(path: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().into()
}
static GRAPH_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());
struct Fixture {
    store: Store,
    id: String,
    policy: BoundaryPolicyV2,
    clock: Arc<AtomicU64>,
    calls: Arc<AtomicU64>,
    expire_on_call: Arc<AtomicU64>,
    temp: tempfile::TempDir,
}
fn fixture(files: &[(&str, &str, Language)]) -> Fixture {
    fixture_at(files, 100)
}
fn fixture_at(files: &[(&str, &str, Language)], admitted_at: u64) -> Fixture {
    let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    let root = temp.path();
    git(root, &["init", "-b", "main"]);
    git(
        root,
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    for (path, source, _) in files {
        let p = root.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, source).unwrap();
    }
    fs::write(root.join("LICENSE"), "MIT fixture\n").unwrap();
    git(root, &["add", "."]);
    git(
        root,
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
    let revision = git(root, &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        root,
        "https://example.com/owner/repo",
        &revision,
        Scope {
            analysis: files
                .iter()
                .map(|v| v.0.to_string())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            context: vec!["LICENSE".into()],
            max_files: files.len() + 1,
            max_bytes: 1024 * 1024,
            max_file_bytes: 512 * 1024,
        },
    )
    .unwrap();
    let clock = Arc::new(AtomicU64::new(admitted_at));
    let c = clock.clone();
    let calls = Arc::new(AtomicU64::new(0));
    let expire_on_call = Arc::new(AtomicU64::new(u64::MAX));
    let count = calls.clone();
    let expire = expire_on_call.clone();
    let store = Store::open(&root.join("store"), move || {
        if count.fetch_add(1, Ordering::SeqCst) + 1 >= expire.load(Ordering::SeqCst) {
            admitted_at + 100
        } else {
            c.load(Ordering::SeqCst)
        }
    })
    .unwrap();
    let a = store.admit(packet, Retention { seconds: 100 }).unwrap();
    let languages: BTreeSet<_> = files.iter().map(|v| v.2).collect();
    let policy = BoundaryPolicyV2 {
        schema: adl::codefriend::architecture::structure_v2::VERSION.into(),
        coupling_threshold: 1,
        analysis: AnalysisPolicy {
            schema: adl::codefriend::language::VERSION.into(),
            files: files.iter().map(|v| (v.0.into(), v.2)).collect(),
            roots: languages
                .into_iter()
                .map(|language| ProjectRoot {
                    language,
                    root: ".".into(),
                    manifest: None,
                })
                .collect(),
            layers: files
                .iter()
                .enumerate()
                .map(|(i, v)| (v.0.into(), format!("layer{i}")))
                .collect(),
            allowed: BTreeSet::new(),
            limits: Limits {
                max_nodes: 10000,
                max_depth: 64,
                max_facts: 1000,
                max_output_bytes: 4 * 1024 * 1024,
            },
        },
    };
    Fixture {
        store,
        id: a.packet.packet_id,
        policy,
        clock,
        calls,
        expire_on_call,
        temp,
    }
}

use adl::codefriend::{
    architecture::drift_v2,
    evidence::contracts::ReviewRecord,
    memory::baseline::{AdmittedBaselines, BaselineAccess, BaselineRef},
};
struct Pair<'a> {
    before: AdmittedBaselines<'a>,
    after: AdmittedBaselines<'a>,
    left: BaselineRef,
    right: BaselineRef,
}
impl BaselineAccess for Pair<'_> {
    fn load(&self, r: &BaselineRef) -> anyhow::Result<ReviewRecord> {
        if r == &self.left {
            self.before.load(r)
        } else if r == &self.right {
            self.after.load(r)
        } else {
            anyhow::bail!("reference_missing")
        }
    }
}
fn graph(f: &Fixture) -> StructureReportV2 {
    repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap()
}
fn pair<'a>(
    a: &'a Fixture,
    b: &'a Fixture,
    ga: &StructureReportV2,
    gb: &StructureReportV2,
) -> Pair<'a> {
    let before = AdmittedBaselines::open(&a.store, &a.temp.path().join("baseline"), true).unwrap();
    let after = AdmittedBaselines::open(&b.store, &b.temp.path().join("baseline"), true).unwrap();
    let left = before.retain(&ga.record).unwrap();
    let right = after.retain(&gb.record).unwrap();
    Pair {
        before,
        after,
        left,
        right,
    }
}
fn fixtures() -> (Fixture, Fixture) {
    (
        fixture(&[
            ("a.js", "import './b.js';", Language::JavaScript),
            ("b.js", "export const x=1;", Language::JavaScript),
        ]),
        fixture(&[
            ("a.js", "export const y=2;", Language::JavaScript),
            ("b.js", "export const x=1;", Language::JavaScript),
        ]),
    )
}
#[test]
fn incomplete_graphs_retain_original_refs_and_observed_fact_traces() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let (a, b) = fixtures();
    let ga = graph(&a);
    let gb = graph(&b);
    let access = pair(&a, &b, &ga, &gb);
    let r = drift_v2::architecture_drift_reporter_v2_pair(
        &a.store,
        &b.store,
        &access,
        ga.clone(),
        gb.clone(),
        100,
    )
    .unwrap();
    assert_eq!(r.graph_comparison.baseline, access.left);
    assert_eq!(r.graph_comparison.current, access.right);
    assert!(!r.graph_comparison.comparable);
    assert!(!r.structural_comparison.comparable);
    assert!(r
        .graph_comparison
        .reasons
        .contains(&"incomplete_review_coverage".into()));
    assert!(!r.baseline_traces.is_empty());
    assert!(!r.current_traces.is_empty());
    assert_eq!(r.baseline_facts.admission, ga.record.admission);
    assert_eq!(r.current_facts.admission, gb.record.admission);
    assert_eq!(r.baseline_facts.run.completion, Completion::Incomplete);
    assert_eq!(
        r,
        drift_v2::architecture_drift_reporter_v2_pair(&a.store, &b.store, &access, ga, gb, 100)
            .unwrap()
    );
    r.validate_pair(&a.store, &b.store, &access, 100).unwrap();
    let path = a.temp.path().join("drift.json");
    drift_v2::write_report_v2_pair(&r, &a.store, &b.store, &access, &path, 100).unwrap();
    assert_eq!(
        drift_v2::read_report_v2_pair(&a.store, &b.store, &access, &path, 100).unwrap(),
        r
    );
}
#[test]
fn both_original_owners_and_retained_records_remain_required() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let (a, b) = fixtures();
    let ga = graph(&a);
    let gb = graph(&b);
    let access = pair(&a, &b, &ga, &gb);
    let r = drift_v2::architecture_drift_reporter_v2_pair(&a.store, &b.store, &access, ga, gb, 100)
        .unwrap();
    a.clock.store(200, Ordering::SeqCst);
    assert!(r.validate_pair(&a.store, &b.store, &access, 100).is_err());
    let (a, b) = fixtures();
    let ga = graph(&a);
    let gb = graph(&b);
    let access = pair(&a, &b, &ga, &gb);
    let r = drift_v2::architecture_drift_reporter_v2_pair(&a.store, &b.store, &access, ga, gb, 100)
        .unwrap();
    b.store.delete(&b.id).unwrap();
    assert!(r.validate_pair(&a.store, &b.store, &access, 100).is_err());
}
#[test]
fn tampered_traces_backend_references_and_aggregate_output_fail_closed() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let (a, b) = fixtures();
    let ga = graph(&a);
    let gb = graph(&b);
    let access = pair(&a, &b, &ga, &gb);
    let r = drift_v2::architecture_drift_reporter_v2_pair(
        &a.store,
        &b.store,
        &access,
        ga.clone(),
        gb.clone(),
        100,
    )
    .unwrap();
    let mut bad = r.clone();
    bad.baseline_traces.clear();
    assert!(bad.validate_pair(&a.store, &b.store, &access, 100).is_err());
    struct Wrong(ReviewRecord);
    impl BaselineAccess for Wrong {
        fn load(&self, _: &BaselineRef) -> anyhow::Result<ReviewRecord> {
            Ok(self.0.clone())
        }
    }
    assert!(drift_v2::architecture_drift_reporter_v2_pair(
        &a.store,
        &b.store,
        &Wrong(gb.record.clone()),
        ga,
        gb,
        100
    )
    .is_err());
    let mut huge = r.clone();
    huge.digest = "x".repeat(4 * 1024 * 1024);
    let path = a.temp.path().join("too-large.json");
    assert!(
        drift_v2::write_report_v2_pair(&huge, &a.store, &b.store, &access, &path, 100).is_err()
    );
    assert!(!path.exists());
}
#[cfg(unix)]
#[test]
fn postwrite_original_baseline_expiry_removes_new_artifact() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let (a, b) = fixtures();
    let ga = graph(&a);
    let gb = graph(&b);
    let access = pair(&a, &b, &ga, &gb);
    let r = drift_v2::architecture_drift_reporter_v2_pair(&a.store, &b.store, &access, ga, gb, 100)
        .unwrap();
    let before = a.calls.load(Ordering::SeqCst);
    r.validate_pair(&a.store, &b.store, &access, 100).unwrap();
    let count = a.calls.load(Ordering::SeqCst) - before;
    // After validation: prewrite access.load(left)+original Store, followed by
    // postwrite access.load(left). Expire at that first postwrite owner read.
    a.expire_on_call
        .store(a.calls.load(Ordering::SeqCst) + count + 3, Ordering::SeqCst);
    let path = a.temp.path().join("expired.json");
    assert!(drift_v2::write_report_v2_pair(&r, &a.store, &b.store, &access, &path, 100).is_err());
    assert!(!path.exists());
}

#[test]
fn existing_drift_cli_uses_two_original_admissions_in_same_store() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let f = fixture_at(&[("a.py", "x=1\n", Language::Python)], now);
    let before = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), now).unwrap();
    fs::write(f.temp.path().join("a.py"), "x=2\n").unwrap();
    git(f.temp.path(), &["add", "a.py"]);
    git(
        f.temp.path(),
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "second",
        ],
    );
    let revision = git(f.temp.path(), &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        f.temp.path(),
        "https://example.com/owner/repo",
        &revision,
        before.record.admission.packet.scope.clone(),
    )
    .unwrap();
    let second = f.store.admit(packet, Retention { seconds: 100 }).unwrap();
    let after =
        repository_structure_reporter_v2(&f.store, &second.packet.packet_id, f.policy.clone(), now)
            .unwrap();
    let bp = f.temp.path().join("before.json");
    let cp = f.temp.path().join("current.json");
    write_report_v2(&before, &f.store, &bp, now).unwrap();
    write_report_v2(&after, &f.store, &cp, now).unwrap();
    let output = f.temp.path().join("drift.json");
    let baselines = f.temp.path().join("retained-baselines");
    let store_path = f.temp.path().join("store");
    drop(f.store);
    let run = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "drift",
            "--store",
            store_path.to_str().unwrap(),
            "--baselines",
            baselines.to_str().unwrap(),
            "--baseline",
            bp.to_str().unwrap(),
            "--current",
            cp.to_str().unwrap(),
            "--out",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let raw = fs::read(&output).unwrap();
    let value: adl::codefriend::architecture::artifact::DriftArtifact =
        serde_json::from_slice(&raw).unwrap();
    assert_eq!(raw, serde_json::to_vec(&value).unwrap());
    assert_eq!(value.summary()["comparable"], false);
    let typed: drift_v2::DriftReportV2 = serde_json::from_slice(&raw).unwrap();
    assert_eq!(typed.baseline.record.admission, before.record.admission);
    assert_eq!(typed.current.record.admission, after.record.admission);
    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "drift-read",
            "--store",
            store_path.to_str().unwrap(),
            "--baselines",
            baselines.to_str().unwrap(),
            "--input",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        read.status.success(),
        "{}",
        String::from_utf8_lossy(&read.stderr)
    );
    let padded = f.temp.path().join("oversized-drift.json");
    let mut data = raw;
    data.resize(4 * 1024 * 1024 + 1, b' ');
    fs::write(&padded, data).unwrap();
    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "drift-read",
            "--store",
            store_path.to_str().unwrap(),
            "--baselines",
            baselines.to_str().unwrap(),
            "--input",
            padded.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!read.status.success());
}

#[test]
fn cross_version_drift_rejects_before_baseline_lookup() {
    use adl::codefriend::architecture::{artifact, structure};
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("lib.rs", "pub fn f() {}", Language::Rust)]);
    let before = structure::repository_structure_reporter(
        &f.store,
        &f.id,
        structure::BoundaryPolicy {
            schema: structure::VERSION.into(),
            crate_root: "lib.rs".into(),
            manifest_path: None,
            layers: f.policy.analysis.layers.clone(),
            allowed: BTreeSet::new(),
            coupling_threshold: 1,
        },
    )
    .unwrap();
    let current = graph(&f);
    struct NoLookup;
    impl BaselineAccess for NoLookup {
        fn load(&self, _: &BaselineRef) -> anyhow::Result<ReviewRecord> {
            panic!("incompatible graph must not load baseline")
        }
    }
    let result = artifact::drift_report_pair(
        &f.store,
        &f.store,
        &NoLookup,
        artifact::StructureArtifact::V1(before),
        artifact::StructureArtifact::V2(current),
        100,
    );
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("incompatible_drift_graph_schema"));
}
