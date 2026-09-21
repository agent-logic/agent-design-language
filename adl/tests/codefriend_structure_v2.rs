//! PVF: deterministic source-owner graph contract; no provider or Journey claim.
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
#[test]
fn four_languages_produce_source_bound_edges_without_complete_claim() {
    let _guard = GRAPH_TEST.lock().unwrap();
    for (files, from, to) in [
        (
            vec![
                ("lib.rs", "mod dep; use crate::dep;", Language::Rust),
                ("dep.rs", "pub fn f() {}", Language::Rust),
            ],
            "lib.rs",
            "dep.rs",
        ),
        (
            vec![
                ("Main.java", "import pkg.Dep; class Main {}", Language::Java),
                (
                    "Dep.java",
                    "package pkg; public class Dep {}",
                    Language::Java,
                ),
            ],
            "Main.java",
            "Dep.java",
        ),
        (
            vec![
                ("main.py", "import dep\n", Language::Python),
                ("dep.py", "x = 1\n", Language::Python),
            ],
            "main.py",
            "dep.py",
        ),
        (
            vec![
                ("main.js", "import './dep.js';", Language::JavaScript),
                ("dep.js", "export const x = 1;", Language::JavaScript),
            ],
            "main.js",
            "dep.js",
        ),
    ] {
        let f = fixture(&files);
        let original = f.store.get(&f.id).unwrap();
        let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
        assert!(!r.analysis_complete);
        assert_eq!(r.record.run.completion, Completion::Incomplete);
        assert_eq!(r.record.admission, original);
        assert!(r
            .analysis
            .files
            .iter()
            .all(|f| f.coverage.semantics == Coverage::Unsupported));
        assert!(r
            .edges
            .iter()
            .any(|e| r.nodes.iter().any(|n| n.id == e.from && n.path == from)
                && r.nodes.iter().any(|n| n.id == e.to && n.path == to)));
        assert!(r
            .record
            .findings
            .iter()
            .any(|v| v.rule == "admitted_boundary_violation"));
        r.validate(&f.store, 100).unwrap();
        let encoded = serde_json::to_vec(&r).unwrap();
        let dispatched: adl::codefriend::architecture::artifact::StructureArtifact =
            serde_json::from_slice(&encoded).unwrap();
        assert_eq!(serde_json::to_vec(&dispatched).unwrap(), encoded);
        assert_eq!(dispatched.digest(), r.digest);
        dispatched.validate(&f.store, 100).unwrap();
        let policy_bytes = serde_json::to_vec(&f.policy).unwrap();
        let policy_dispatch: adl::codefriend::architecture::artifact::BoundaryPolicyArtifact =
            serde_json::from_slice(&policy_bytes).unwrap();
        assert_eq!(serde_json::to_vec(&policy_dispatch).unwrap(), policy_bytes);
        assert_eq!(
            r,
            repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap()
        );
        for e in &r.edges {
            let source = original
                .packet
                .objects
                .iter()
                .find(|o| o.path == e.location.path)
                .unwrap()
                .content
                .as_ref()
                .unwrap();
            assert_eq!(&source[e.span.start_byte..e.span.end_byte], e.spelling);
        }
        assert_eq!(f.store.get(&f.id).unwrap(), original);
    }
}
#[test]
fn graph_findings_keep_cycle_and_coupling_witnesses_bounded() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[
        (
            "a.js",
            "import './b.js'; import './c.js';",
            Language::JavaScript,
        ),
        ("b.js", "import './a.js';", Language::JavaScript),
        ("c.js", "export const c = 1;", Language::JavaScript),
    ]);
    let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    assert!(r
        .record
        .findings
        .iter()
        .any(|f| f.rule == "admitted_import_cycle"));
    assert!(r
        .record
        .findings
        .iter()
        .any(|f| f.rule == "admitted_coupling_threshold"));
    r.record.validate().unwrap();
}
#[test]
fn tampering_and_original_owner_expiry_deletion_are_rejected() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("a.py", "import missing\n", Language::Python)]);
    let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    assert!(r.edges.is_empty());
    let mut oversized = r.clone();
    oversized.coverage_scope = "x".repeat(4 * 1024 * 1024);
    let oversized_path = f.temp.path().join("oversized.json");
    assert!(write_report_v2(&oversized, &f.store, &oversized_path, 100).is_err());
    assert!(!oversized_path.exists());
    let mut bad = r.clone();
    bad.analysis_complete = true;
    assert!(bad.validate(&f.store, 100).is_err());
    let path = f.temp.path().join("graph.json");
    write_report_v2(&r, &f.store, &path, 100).unwrap();
    assert_eq!(read_report_v2(&f.store, &path, 100).unwrap(), r);
    assert!(write_report_v2(&r, &f.store, &path, 100).is_err());
    f.clock.store(200, Ordering::SeqCst);
    assert!(r.validate(&f.store, 100).is_err());
    assert!(read_report_v2(&f.store, &path, 100).is_err());
    let f = fixture(&[("a.py", "x = 1\n", Language::Python)]);
    let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    f.store.delete(&f.id).unwrap();
    assert!(r.validate(&f.store, 100).is_err());
}
#[test]
fn v2_requires_explicit_layers_and_does_not_accept_fake_rust_policy() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("a.java", "class A {}", Language::Java)]);
    let mut p = f.policy.clone();
    p.analysis.layers.clear();
    assert!(repository_structure_reporter_v2(&f.store, &f.id, p, 100).is_err());
    let mut p = f.policy.clone();
    p.schema = "codefriend.structure.v1".into();
    assert!(repository_structure_reporter_v2(&f.store, &f.id, p, 100).is_err());
    let mut p = f.policy.clone();
    p.coupling_threshold = 513;
    assert!(repository_structure_reporter_v2(&f.store, &f.id, p, 100).is_err());
}

#[test]
fn inline_rust_import_is_not_a_file_cycle() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("lib.rs", "mod inner {} use crate::inner;", Language::Rust)]);
    let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    assert!(r.edges.iter().any(|e| e.from == e.to));
    assert!(!r.record.findings.iter().any(|f| matches!(
        f.rule.as_str(),
        "admitted_import_cycle" | "admitted_coupling_threshold"
    )));
}

#[cfg(unix)]
#[test]
fn expiry_after_artifact_write_removes_only_new_artifact() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("a.py", "x = 1\n", Language::Python)]);
    let r = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    let before = f.calls.load(Ordering::SeqCst);
    r.validate(&f.store, 100).unwrap();
    let validation_calls = f.calls.load(Ordering::SeqCst) - before;
    // validate's owner reads, then pre-write live, then post-fsync live.
    f.expire_on_call.store(
        f.calls.load(Ordering::SeqCst) + validation_calls + 2,
        Ordering::SeqCst,
    );
    let path = f.temp.path().join("expired-output.json");
    assert!(write_report_v2(&r, &f.store, &path, 100).is_err());
    assert!(!path.exists());
}

#[test]
fn existing_architecture_cli_reports_and_reads_java_v2() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let f = fixture_at(
        &[
            (
                "Main.java",
                "package demo; import demo.Other; public class Main {}",
                Language::Java,
            ),
            (
                "Other.java",
                "package demo; public class Other {}",
                Language::Java,
            ),
        ],
        now,
    );
    let policy = f.temp.path().join("policy.json");
    let output = f.temp.path().join("cli-report.json");
    let store_path = f.temp.path().join("store");
    fs::write(&policy, serde_json::to_vec(&f.policy).unwrap()).unwrap();
    drop(f.store);
    let run = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "report",
            "--store",
            store_path.to_str().unwrap(),
            "--packet-id",
            &f.id,
            "--policy",
            policy.to_str().unwrap(),
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
    let summary: serde_json::Value = serde_json::from_slice(&run.stdout).unwrap();
    assert_eq!(summary["schema"], "codefriend.structure.v2");
    assert_eq!(summary["analysis_complete"], false);
    assert_eq!(summary["edges"], 1);
    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "read",
            "--store",
            store_path.to_str().unwrap(),
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
    let read_summary: serde_json::Value = serde_json::from_slice(&read.stdout).unwrap();
    assert_eq!(read_summary, summary);
    let graph: adl::codefriend::architecture::structure_v2::StructureReportV2 =
        serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    let changes = f.temp.path().join("changes.json");
    fs::write(&changes, serde_json::to_vec(&serde_json::json!({"schema":"codefriend.impact.v2", "repository":graph.record.admission.packet.repository,"revision":graph.record.admission.packet.revision,"graph_digest":graph.digest,"targets":[{"kind":"path","value":"Other.java"}]})).unwrap()).unwrap();
    let impact = f.temp.path().join("impact.json");
    let affected = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "impact",
            "--store",
            store_path.to_str().unwrap(),
            "--graph",
            output.to_str().unwrap(),
            "--changes",
            changes.to_str().unwrap(),
            "--out",
            impact.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        affected.status.success(),
        "{}",
        String::from_utf8_lossy(&affected.stderr)
    );
    let affected_summary: serde_json::Value = serde_json::from_slice(&affected.stdout).unwrap();
    assert_eq!(affected_summary["schema"], "codefriend.impact.v2");
    assert_eq!(affected_summary["impacts"], 1);
    assert_eq!(affected_summary["analysis_complete"], false);
    let impact_read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "impact-read",
            "--store",
            store_path.to_str().unwrap(),
            "--input",
            impact.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        impact_read.status.success(),
        "{}",
        String::from_utf8_lossy(&impact_read.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&impact_read.stdout).unwrap(),
        affected_summary
    );
    let padded_impact = f.temp.path().join("padded-impact.json");
    let mut impact_bytes = fs::read(&impact).unwrap();
    impact_bytes.resize(4 * 1024 * 1024 + 1, b' ');
    fs::write(&padded_impact, impact_bytes).unwrap();
    let reject_impact = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "impact-read",
            "--store",
            store_path.to_str().unwrap(),
            "--input",
            padded_impact.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!reject_impact.status.success());

    let padded = f.temp.path().join("padded-report.json");
    let mut padded_bytes = fs::read(&output).unwrap();
    padded_bytes.resize(4 * 1024 * 1024 + 1, b' ');
    fs::write(&padded, padded_bytes).unwrap();
    let rejected = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "read",
            "--store",
            store_path.to_str().unwrap(),
            "--input",
            padded.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!rejected.status.success());

    assert!(String::from_utf8_lossy(&run.stderr).contains("adl_event kind=codefriend_architecture"));
}

#[test]
fn architecture_dispatch_retains_original_admission_authority() {
    use adl::codefriend::architecture::artifact::{self, StructureArtifact};
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("main.py", "value = 1", Language::Python)]);
    let report = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap();
    let output = f.temp.path().join("report.json");
    artifact::write_report(&StructureArtifact::V2(report), &f.store, &output, 100).unwrap();
    assert!(artifact::read_report(&f.store, &output, 100).is_ok());
    f.clock.store(200, Ordering::SeqCst);
    assert!(artifact::read_report(&f.store, &output, 100).is_err());
}
