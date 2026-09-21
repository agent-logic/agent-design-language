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
    fixture_context(files, &[])
}
fn fixture_context(files: &[(&str, &str, Language)], contexts: &[(&str, &str)]) -> Fixture {
    fixture_context_at(files, contexts, 100)
}
fn fixture_context_at(
    files: &[(&str, &str, Language)],
    contexts: &[(&str, &str)],
    admitted_at: u64,
) -> Fixture {
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
    for (path, source) in contexts {
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
            context: std::iter::once("LICENSE".to_string())
                .chain(contexts.iter().map(|(p, _)| p.to_string()))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            max_files: files.len() + contexts.len() + 1,
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

use adl::codefriend::architecture::rationale_v2::{self, BoundarySelection, RationaleSelectionV2};
const COMPOSE: &str =
    r#"{"services":{"api":{"image":"example/api:v1","labels":{"codefriend.boundary":"layer0"}}}}"#;
const ADR:&str="+++\nstatus = 'accepted'\nboundary = 'layer0'\nservice = 'api'\ndecision_key = 'storage'\nchoice = 'local'\n+++\nThe operator recorded this decision.\n";
fn graph(f: &Fixture) -> StructureReportV2 {
    repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), 100).unwrap()
}
fn selection(g: &StructureReportV2, paths: &[&str]) -> RationaleSelectionV2 {
    RationaleSelectionV2 {
        schema: rationale_v2::VERSION.into(),
        graph_digest: g.digest.clone(),
        revision: g.record.run.revision.clone(),
        boundaries: vec![BoundarySelection {
            boundary: "layer0".into(),
            deployment_path: "compose.json".into(),
            service: "api".into(),
            rationale_paths: paths.iter().map(|p| p.to_string()).collect(),
        }],
    }
}
#[test]
fn admitted_context_adr_and_deployment_are_evidence_not_authority() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture_context(
        &[("main.py", "x = 1\n", Language::Python)],
        &[("compose.json", COMPOSE), ("adr.md", ADR)],
    );
    let g = graph(&f);
    let selected = selection(&g, &["adr.md"]);
    let old_selection = adl::codefriend::architecture::rationale::RationaleSelection {
        schema: adl::codefriend::architecture::rationale::VERSION.into(),
        graph_digest: g.digest.clone(),
        revision: g.record.run.revision.clone(),
        boundaries: selected.boundaries.clone(),
    };
    assert!(adl::codefriend::architecture::artifact::rationale_report(
        &f.store,
        adl::codefriend::architecture::artifact::StructureArtifact::V2(g.clone()),
        adl::codefriend::architecture::artifact::RationaleSelectionArtifact::V1(old_selection),
        100
    )
    .is_err());
    let r = rationale_v2::architecture_rationale_reporter_v2(
        &f.store,
        g.clone(),
        selected.clone(),
        100,
    )
    .unwrap();
    assert!(!r.analysis_complete);
    assert_eq!(r.record.run.completion, Completion::Incomplete);
    assert_eq!(r.boundaries[0].rationale.len(), 1);
    assert_eq!(r.boundaries[0].rationale[0].decision.status, "accepted");
    assert!(r.boundaries[0]
        .deployment_observation
        .starts_with("separate_compose"));
    assert!(r.boundaries[0]
        .unknowns
        .contains(&"partial_structure_graph".into()));
    assert!(r
        .record
        .findings
        .iter()
        .all(|v| v.inference.contains("never execution permission")));
    assert_eq!(
        r,
        rationale_v2::architecture_rationale_reporter_v2(&f.store, g, selected, 100).unwrap()
    );
    r.validate(&f.store, 100).unwrap();
    let path = f.temp.path().join("rationale.json");
    rationale_v2::write_report_v2(&r, &f.store, &path, 100).unwrap();
    assert_eq!(
        rationale_v2::read_report_v2(&f.store, &path, 100).unwrap(),
        r
    );
}
#[test]
fn conflicting_and_absent_decisions_remain_explicit_unknowns() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let contrary = ADR.replace("'local'", "'remote'");
    let f = fixture_context(
        &[("Main.java", "class Main {}", Language::Java)],
        &[
            ("compose.json", COMPOSE),
            ("a.md", ADR),
            ("b.md", &contrary),
        ],
    );
    let g = graph(&f);
    let r = rationale_v2::architecture_rationale_reporter_v2(
        &f.store,
        g.clone(),
        selection(&g, &["a.md", "b.md", "missing.md"]),
        100,
    )
    .unwrap();
    assert_eq!(r.boundaries[0].conflicting_decision_keys, vec!["storage"]);
    assert!(r.boundaries[0]
        .unknowns
        .iter()
        .any(|u| u.contains("missing.md")));
    assert!(r
        .record
        .findings
        .iter()
        .any(|f| f.rule == "conflicting_rationale"));
    let mut stale = selection(&g, &[]);
    stale.graph_digest = "0".repeat(64);
    assert!(rationale_v2::architecture_rationale_reporter_v2(&f.store, g, stale, 100).is_err());
}
#[test]
fn original_owner_retention_tamper_and_aggregate_bounds() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("main.js", "export const x = 1;", Language::JavaScript)]);
    let g = graph(&f);
    let r = rationale_v2::architecture_rationale_reporter_v2(
        &f.store,
        g.clone(),
        selection(&g, &[]),
        100,
    )
    .unwrap();
    assert!(r.boundaries[0]
        .unknowns
        .contains(&"deployment_evidence_unavailable".into()));
    let mut bad = r.clone();
    bad.boundaries[0].unknowns.clear();
    assert!(bad.validate(&f.store, 100).is_err());
    let mut huge = r.clone();
    huge.boundaries[0]
        .unknowns
        .push("x".repeat(4 * 1024 * 1024));
    let path = f.temp.path().join("oversized.json");
    assert!(rationale_v2::write_report_v2(&huge, &f.store, &path, 100).is_err());
    assert!(!path.exists());
    f.clock.store(200, Ordering::SeqCst);
    assert!(r.validate(&f.store, 100).is_err());
    let f = fixture(&[("lib.rs", "pub fn f() {}", Language::Rust)]);
    let g = graph(&f);
    let r = rationale_v2::architecture_rationale_reporter_v2(
        &f.store,
        g.clone(),
        selection(&g, &[]),
        100,
    )
    .unwrap();
    f.store.delete(&f.id).unwrap();
    assert!(r.validate(&f.store, 100).is_err());
}
#[cfg(unix)]
#[test]
fn postwrite_expiry_removes_new_rationale_artifact() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let f = fixture(&[("main.py", "x=1\n", Language::Python)]);
    let g = graph(&f);
    let r = rationale_v2::architecture_rationale_reporter_v2(
        &f.store,
        g.clone(),
        selection(&g, &[]),
        100,
    )
    .unwrap();
    let before = f.calls.load(Ordering::SeqCst);
    r.validate(&f.store, 100).unwrap();
    let count = f.calls.load(Ordering::SeqCst) - before;
    f.expire_on_call
        .store(f.calls.load(Ordering::SeqCst) + count + 2, Ordering::SeqCst);
    let path = f.temp.path().join("expired.json");
    assert!(rationale_v2::write_report_v2(&r, &f.store, &path, 100).is_err());
    assert!(!path.exists());
}

#[test]
fn duplicate_deployment_keys_are_rejected_instead_of_selecting_last_value() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let duplicate = r#"{"services":{"api":{"image":"example/api:v1","labels":{"codefriend.boundary":"wrong","codefriend.boundary":"layer0"}}}}"#;
    let f = fixture_context(
        &[("main.py", "x=1\n", Language::Python)],
        &[("compose.json", duplicate), ("adr.md", ADR)],
    );
    let g = graph(&f);
    let selected = selection(&g, &["adr.md"]);
    let result = rationale_v2::architecture_rationale_reporter_v2(&f.store, g, selected, 100);
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("invalid_deployment_json"));
}

#[test]
fn existing_rationale_cli_dispatches_v2_without_re_admission() {
    let _guard = GRAPH_TEST.lock().unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let f = fixture_context_at(
        &[("main.py", "x=1\n", Language::Python)],
        &[("compose.json", COMPOSE), ("adr.md", ADR)],
        now,
    );
    let g = repository_structure_reporter_v2(&f.store, &f.id, f.policy.clone(), now).unwrap();
    let graph_path = f.temp.path().join("graph.json");
    write_report_v2(&g, &f.store, &graph_path, now).unwrap();
    let selection_path = f.temp.path().join("selection.json");
    fs::write(
        &selection_path,
        serde_json::to_vec(&selection(&g, &["adr.md"])).unwrap(),
    )
    .unwrap();
    let output = f.temp.path().join("rationale.json");
    let store_path = f.temp.path().join("store");
    drop(f.store);
    let run = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "rationale",
            "--store",
            store_path.to_str().unwrap(),
            "--graph",
            graph_path.to_str().unwrap(),
            "--selection",
            selection_path.to_str().unwrap(),
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
    let dispatch: adl::codefriend::architecture::artifact::RationaleArtifact =
        serde_json::from_slice(&raw).unwrap();
    assert_eq!(raw, serde_json::to_vec(&dispatch).unwrap());
    assert_eq!(dispatch.record().admission, g.record.admission);
    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "rationale-read",
            "--store",
            store_path.to_str().unwrap(),
            "--input",
            output.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(read.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&read.stdout).unwrap()["schema"],
        rationale_v2::VERSION
    );
    let padded = f.temp.path().join("oversized-rationale.json");
    let mut data = raw;
    data.resize(4 * 1024 * 1024 + 1, b' ');
    fs::write(&padded, data).unwrap();
    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "architecture",
            "rationale-read",
            "--store",
            store_path.to_str().unwrap(),
            "--input",
            padded.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!read.status.success());
}
