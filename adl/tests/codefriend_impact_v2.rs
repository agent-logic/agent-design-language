//! PVF: deterministic source-backed impact v2 contract; no runtime/Journey claim.
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

use adl::codefriend::architecture::impact_v2::{self, ChangeSetV2, ChangeTargetV2};
fn graph(f: &Fixture) -> StructureReportV2 {
    repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap()
}
fn changes(g: &StructureReportV2, path: &str) -> ChangeSetV2 {
    ChangeSetV2 {
        schema: impact_v2::VERSION.into(),
        repository: g.record.run.repository.clone(),
        revision: g.record.run.revision.clone(),
        graph_digest: g.digest.clone(),
        targets: vec![ChangeTargetV2::Path(path.into())],
    }
}
#[test]
fn source_paths_have_deterministic_import_witnesses_without_absence_claims() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[
        ("a.js", "import './b.js';", Language::JavaScript),
        ("b.js", "import './c.js';", Language::JavaScript),
        ("c.js", "export const c = 1;", Language::JavaScript),
        (
            "unseen.js",
            "import unknown from 'external';",
            Language::JavaScript,
        ),
    ]);
    let g = graph(&f);
    let c = changes(&g, "c.js");
    let r = impact_v2::change_impact_reporter_v2(&f.store, g.clone(), c.clone(), 100).unwrap();
    assert_eq!(r.impacts.len(), 2);
    assert!(r.impacts.iter().any(|i| i.path.len() == 2));
    assert!(!r.analysis_complete);
    assert!(r.scoped_unimpacted_nodes.is_empty());
    assert!(!r.unknowns.is_empty());
    assert_eq!(r.record.run.completion, Completion::Incomplete);
    for impact in &r.impacts {
        assert_eq!(impact.path.first().unwrap().from, impact.dependent_node);
        assert_eq!(impact.path.last().unwrap().to, impact.changed_node);
        for pair in impact.path.windows(2) {
            assert_eq!(pair[0].to, pair[1].from);
        }
    }
    assert_eq!(
        r,
        impact_v2::change_impact_reporter_v2(&f.store, g, c, 100).unwrap()
    );
    r.validate(&f.store, 100).unwrap();
}
#[test]
fn selection_binds_exact_graph_revision_and_admitted_identity() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[
        ("main.py", "import dep\n", Language::Python),
        ("dep.py", "x=1\n", Language::Python),
    ]);
    let g = graph(&f);
    let mut c = changes(&g, "dep.py");
    c.revision = "0".repeat(40);
    assert!(impact_v2::change_impact_reporter_v2(&f.store, g.clone(), c, 100).is_err());
    let mut c = changes(&g, "dep.py");
    c.graph_digest = "0".repeat(64);
    assert!(impact_v2::change_impact_reporter_v2(&f.store, g.clone(), c, 100).is_err());
    assert!(impact_v2::change_impact_reporter_v2(
        &f.store,
        g.clone(),
        changes(&g, "../dep.py"),
        100
    )
    .is_err());
    let mut c = changes(&g, "dep.py");
    let id = g
        .nodes
        .iter()
        .find(|n| n.path == "dep.py")
        .unwrap()
        .id
        .clone();
    c.targets = vec![ChangeTargetV2::NodeId(id.clone())];
    let r = impact_v2::change_impact_reporter_v2(&f.store, g, c, 100).unwrap();
    assert_eq!(r.changed_nodes, vec![id]);
}
#[test]
fn original_owner_tampering_retention_and_aggregate_output_are_enforced() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[
        ("a.js", "import './b.js';", Language::JavaScript),
        ("b.js", "export const x=1;", Language::JavaScript),
    ]);
    let g = graph(&f);
    let c = changes(&g, "b.js");
    let r = impact_v2::change_impact_reporter_v2(&f.store, g, c, 100).unwrap();
    let path = f.temp.path().join("impact.json");
    impact_v2::write_report_v2(&r, &f.store, &path, 100).unwrap();
    assert_eq!(impact_v2::read_report_v2(&f.store, &path, 100).unwrap(), r);
    let mut bad = r.clone();
    bad.impacts[0].path.clear();
    assert!(bad.validate(&f.store, 100).is_err());
    let mut huge = r.clone();
    huge.unknowns.push("x".repeat(4 * 1024 * 1024));
    let rejected = f.temp.path().join("too-large.json");
    assert!(impact_v2::write_report_v2(&huge, &f.store, &rejected, 100).is_err());
    assert!(!rejected.exists());
    f.clock.store(200, Ordering::SeqCst);
    assert!(r.validate(&f.store, 100).is_err());
    let f = fixture(&[("a.py", "x=1\n", Language::Python)]);
    let g = graph(&f);
    let r = impact_v2::change_impact_reporter_v2(&f.store, g.clone(), changes(&g, "a.py"), 100)
        .unwrap();
    f.store.delete(&f.id).unwrap();
    assert!(r.validate(&f.store, 100).is_err());
}
#[cfg(unix)]
#[test]
fn expiry_after_impact_write_removes_new_output() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("a.py", "x=1\n", Language::Python)]);
    let g = graph(&f);
    let r = impact_v2::change_impact_reporter_v2(&f.store, g.clone(), changes(&g, "a.py"), 100)
        .unwrap();
    let before = f.calls.load(Ordering::SeqCst);
    r.validate(&f.store, 100).unwrap();
    let validation_calls = f.calls.load(Ordering::SeqCst) - before;
    f.expire_on_call.store(
        f.calls.load(Ordering::SeqCst) + validation_calls + 2,
        Ordering::SeqCst,
    );
    let path = f.temp.path().join("expired-impact.json");
    assert!(impact_v2::write_report_v2(&r, &f.store, &path, 100).is_err());
    assert!(!path.exists());
}
