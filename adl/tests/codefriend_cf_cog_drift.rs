//! PVF runtime: deterministic admitted structural drift and installed consumer contracts.
//! Controlled inert Git revisions, CPU/filesystem, required issue acceptance; no provider.
use adl::codefriend::{
    architecture::{
        drift,
        structure::{self, BoundaryPolicy, StructureReport},
    },
    evidence::{contracts::Delta, store::Store, Retention},
    ingestion::{local, Scope},
    memory::baseline::{AdmittedBaselines, BaselineRef},
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
static FIXTURES: Mutex<()> = Mutex::new(());
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
    _dir: tempfile::TempDir,
    source: PathBuf,
    store: Store,
    policy: BoundaryPolicy,
    scope: Scope,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = dir.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        for (p, c) in [
            ("lib.rs", "mod a; mod b; mod c;"),
            ("a.rs", "use crate::b::B;"),
            ("b.rs", "pub struct B;"),
            ("c.rs", "pub struct C;"),
            ("LICENSE", "MIT fixture license"),
        ] {
            fs::write(source.join(p), c).unwrap();
        }
        let clock = Arc::new(AtomicU64::new(100));
        let c = clock.clone();
        let store =
            Store::open(&dir.path().join("store"), move || c.load(Ordering::SeqCst)).unwrap();
        Self {
            _dir: dir,
            source,
            store,
            clock,
            scope: Scope {
                analysis: vec!["a.rs".into(), "b.rs".into(), "c.rs".into(), "lib.rs".into()],
                context: vec!["LICENSE".into()],
                max_files: 5,
                max_bytes: 65536,
                max_file_bytes: 32768,
            },
            policy: BoundaryPolicy {
                schema: structure::VERSION.into(),
                crate_root: "lib.rs".into(),
                manifest_path: None,
                layers: [
                    ("lib.rs".into(), "core".into()),
                    ("a.rs".into(), "core".into()),
                    ("b.rs".into(), "core".into()),
                    ("c.rs".into(), "api".into()),
                ]
                .into(),
                allowed: BTreeSet::new(),
                coupling_threshold: 2,
            },
        }
    }
    fn graph(&self) -> StructureReport {
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
                "controlled revision",
            ],
        );
        let rev = git(&self.source, &["rev-parse", "HEAD"]);
        let p = local::acquire(
            &self.source,
            "https://example.com/owner/repo",
            &rev,
            self.scope.clone(),
        )
        .unwrap();
        let a = self.store.admit(p, Retention { seconds: 1000 }).unwrap();
        structure::repository_structure_reporter(
            &self.store,
            &a.packet.packet_id,
            self.policy.clone(),
        )
        .unwrap()
    }
    fn baselines(&self) -> AdmittedBaselines<'_> {
        AdmittedBaselines::open(&self.store, &self._dir.path().join("baselines"), true).unwrap()
    }
}
fn run(f: &Fixture, b: StructureReport, c: StructureReport) -> drift::DriftReport {
    let db = f.baselines();
    db.retain(&b.record).unwrap();
    db.retain(&c.record).unwrap();
    drift::architecture_drift_reporter(&f.store, &db, b, c).unwrap()
}
#[test]
fn introduced_removed_edges_and_cross_layer_explanations() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    fs::write(f.source.join("a.rs"), "use crate::c::C;").unwrap();
    let c = f.graph();
    assert_ne!(b.record.run.revision, c.record.run.revision);
    let r = run(&f, b, c);
    assert!(r.structural_comparison.comparable);
    let changes = &r.structural_comparison.changes;
    assert_eq!(
        changes
            .iter()
            .filter(|x| x.comparison.outcome == Delta::Added)
            .count(),
        1
    );
    assert_eq!(
        changes
            .iter()
            .filter(|x| x.comparison.outcome == Delta::Resolved)
            .count(),
        1
    );
    assert_eq!(
        changes
            .iter()
            .filter(|x| x.comparison.outcome == Delta::Unchanged)
            .count(),
        4
    );
    let added = r
        .current_facts
        .findings
        .iter()
        .find(|x| x.rule == "edge")
        .unwrap();
    assert!(added.rationale.contains("cross_layer=true"));
    assert!(added.rationale.contains("crate::c"));
    let removed = r
        .baseline_facts
        .findings
        .iter()
        .find(|x| x.rule == "edge")
        .unwrap();
    assert!(removed.rationale.contains("cross_layer=false"));
    for ch in changes {
        if let Some(ref x) = ch.before {
            assert_eq!(x.revision, r.baseline.record.run.revision);
        }
        if let Some(ref x) = ch.after {
            assert_eq!(x.revision, r.current.record.run.revision);
        }
    }
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}
#[test]
fn unchanged_and_line_relocation_have_stable_shared_identity() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    fs::write(f.source.join("a.rs"), "\n\nuse crate::b::B;").unwrap();
    let c = f.graph();
    let r = run(&f, b.clone(), c.clone());
    assert!(r
        .structural_comparison
        .changes
        .iter()
        .all(|x| x.comparison.outcome == Delta::Unchanged));
    let edge = r
        .current_facts
        .findings
        .iter()
        .find(|x| x.rule == "edge")
        .unwrap();
    let old = r
        .baseline_traces
        .iter()
        .find(|x| x.finding_id == edge.id)
        .unwrap();
    let new = r
        .current_traces
        .iter()
        .find(|x| x.finding_id == edge.id)
        .unwrap();
    assert_ne!(old.locations, new.locations);
    assert_eq!(r, run(&f, b, c));
}
#[test]
fn repeated_edge_locations_do_not_create_identity_collisions() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    fs::write(f.source.join("a.rs"), "use crate::b::B;\nuse crate::b::B;").unwrap();
    let c = f.graph();
    let r = run(&f, b, c);
    assert!(r
        .structural_comparison
        .changes
        .iter()
        .all(|x| x.comparison.outcome == Delta::Unchanged));
    assert_eq!(
        r.current_traces
            .iter()
            .filter(|t| t.locations.len() == 2)
            .count(),
        1
    );
}
#[test]
fn partial_narrower_and_policy_changes_never_resolve() {
    let _g = FIXTURES.lock().unwrap();
    for variant in ["partial", "narrower", "policy"] {
        let mut f = Fixture::new();
        let b = f.graph();
        match variant {
            "partial" => fs::write(f.source.join("a.rs"), "opaque!();").unwrap(),
            "narrower" => {
                f.scope.analysis.retain(|p| p != "c.rs");
                f.policy.layers.remove("c.rs");
            }
            _ => {
                f.policy.layers.insert("a.rs".into(), "api".into());
            }
        }
        let c = f.graph();
        let r = run(&f, b, c);
        assert!(!r.structural_comparison.comparable);
        assert_eq!(r.structural_comparison.changes.len(), 1);
        assert_eq!(
            r.structural_comparison.changes[0].comparison.outcome,
            Delta::NotComparable
        );
        assert!(!r.structural_comparison.reasons.is_empty());
    }
}
#[test]
fn actual_baseline_storage_is_required_and_deletion_revokes() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    let c = f.graph();
    let db = f.baselines();
    assert!(drift::architecture_drift_reporter(&f.store, &db, b.clone(), c.clone()).is_err());
    let br = db.retain(&b.record).unwrap();
    db.retain(&c.record).unwrap();
    let r = drift::architecture_drift_reporter(&f.store, &db, b, c).unwrap();
    db.delete(&br).unwrap();
    assert!(r.validate(&f.store, &db).is_err());
}
#[test]
fn artifacts_reject_tamper_expiry_and_deleted_evidence() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    let c = f.graph();
    let db = f.baselines();
    db.retain(&b.record).unwrap();
    db.retain(&c.record).unwrap();
    let r = drift::architecture_drift_reporter(&f.store, &db, b, c).unwrap();
    let p = f._dir.path().join("drift.json");
    drift::write_report(&r, &f.store, &db, &p).unwrap();
    assert_eq!(r, drift::read_report(&f.store, &db, &p).unwrap());
    assert!(drift::write_report(&r, &f.store, &db, &p).is_err());
    let mut bad = r.clone();
    bad.structural_comparison.changes.clear();
    assert_eq!(
        bad.validate(&f.store, &db).unwrap_err().to_string(),
        "drift_artifact_mismatch"
    );
    f.clock.store(1200, Ordering::SeqCst);
    assert!(r.validate(&f.store, &db).is_err());
    f.clock.store(100, Ordering::SeqCst);
    f.store.delete(&r.baseline.record.run.packet_id).unwrap();
    assert!(drift::read_report(&f.store, &db, &p).is_err());
}
#[test]
fn malformed_graph_version_and_ambiguous_identity_reject() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    let db = f.baselines();
    db.retain(&b.record).unwrap();
    for variant in ["schema", "identity", "evidence"] {
        let mut bad = b.clone();
        match variant {
            "schema" => bad.schema = "unsupported".into(),
            "identity" => bad.nodes.push(bad.nodes[0].clone()),
            _ => bad.record.admission.evidence.clear(),
        };
        assert!(drift::architecture_drift_reporter(&f.store, &db, b.clone(), bad).is_err());
    }
    assert!(BaselineRef::from_record(&b.record).is_ok());
}
#[test]
fn bad_artifacts_reject_without_partial_success() {
    let _g = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let db = f.baselines();
    let p = f._dir.path().join("bad.json");
    fs::write(&p, "{}").unwrap();
    assert_eq!(
        drift::read_report(&f.store, &db, &p)
            .unwrap_err()
            .to_string(),
        "invalid_drift_artifact"
    );
    fs::File::create(&p)
        .unwrap()
        .set_len(32 * 1024 * 1024 + 1)
        .unwrap();
    assert_eq!(
        drift::read_report(&f.store, &db, &p)
            .unwrap_err()
            .to_string(),
        "drift_artifact_too_large"
    );
    assert!(drift::read_report(&f.store, &db, &f._dir.path().join("../escape")).is_err());
}
#[test]
fn installed_dispatch_retains_real_graphs_and_reads_drift() {
    let _guard = FIXTURES.lock().unwrap();
    let f = Fixture::new();
    let b = f.graph();
    fs::write(f.source.join("a.rs"), "use crate::c::C;").unwrap();
    let c = f.graph();
    let live = f._dir.path().join("live");
    let db = f._dir.path().join("live-baselines");
    let paths = [
        f._dir.path().join("before.json"),
        f._dir.path().join("after.json"),
    ];
    let store = Store::open(&live, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })
    .unwrap();
    for (g, p) in [b, c].into_iter().zip(&paths) {
        let a = store
            .admit(g.record.admission.packet, Retention { seconds: 3600 })
            .unwrap();
        let graph =
            structure::repository_structure_reporter(&store, &a.packet.packet_id, f.policy.clone())
                .unwrap();
        structure::write_report(&graph, &store, p).unwrap();
    }
    drop(store);
    let output = f._dir.path().join("cli-drift.json");
    let args = vec![
        "architecture".into(),
        "drift".into(),
        "--store".into(),
        live.display().to_string(),
        "--baselines".into(),
        db.display().to_string(),
        "--baseline".into(),
        paths[0].display().to_string(),
        "--current".into(),
        paths[1].display().to_string(),
        "--out".into(),
        output.display().to_string(),
    ];
    let cli = |args: Vec<String>, ok: bool| {
        let r = Command::new(env!("CARGO_BIN_EXE_adl"))
            .arg("codefriend")
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            r.status.success(),
            ok,
            "{}",
            String::from_utf8_lossy(&r.stderr)
        );
        r
    };
    let first = cli(args.clone(), true);
    let summary: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(summary["comparable"], true);
    assert!(String::from_utf8_lossy(&first.stderr).contains("adl_event kind=codefriend_drift"));
    let read = cli(
        vec![
            "architecture".into(),
            "drift-read".into(),
            "--store".into(),
            live.display().to_string(),
            "--baselines".into(),
            db.display().to_string(),
            "--input".into(),
            output.display().to_string(),
        ],
        true,
    );
    assert_eq!(
        summary,
        serde_json::from_slice::<serde_json::Value>(&read.stdout).unwrap()
    );
    assert!(String::from_utf8_lossy(&cli(args.clone(), false).stderr)
        .contains("drift_output_unavailable"));
    let mut bad = args;
    *bad.last_mut().unwrap() = db.join("injected.json").display().to_string();
    assert!(String::from_utf8_lossy(&cli(bad, false).stderr)
        .contains("drift_output_inside_managed_store"));
}
