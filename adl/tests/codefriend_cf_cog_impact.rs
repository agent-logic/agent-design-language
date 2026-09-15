//! PVF runtime: deterministic production impact semantics, required issue acceptance.
//! CPU/filesystem only; isolated admitted fixtures, no providers or source execution.
use adl::codefriend::{
    architecture::{
        impact::{self, ChangeSet, ChangeTarget},
        structure::{self, BoundaryPolicy, VERSION},
    },
    evidence::{store::Store, Retention},
    ingestion::{local, Scope},
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
static FIXTURE_LOCK: Mutex<()> = Mutex::new(());
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
    _temp: tempfile::TempDir,
    source: PathBuf,
    store: Store,
    packet: String,
    policy: BoundaryPolicy,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new(files: &[(&str, &str)]) -> Self {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = temp.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        for (p, c) in files {
            let p = source.join(p);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, c).unwrap();
        }
        fs::write(source.join("LICENSE"), "MIT fixture license\n").unwrap();
        git(&source, &["add", "."]);
        git(
            &source,
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
        let revision = git(&source, &["rev-parse", "HEAD"]);
        let mut analysis: Vec<_> = files.iter().map(|(p, _)| p.to_string()).collect();
        analysis.sort();
        let scope = Scope {
            analysis: analysis.clone(),
            context: vec!["LICENSE".into()],
            max_files: files.len() + 1,
            max_bytes: 1024 * 1024,
            max_file_bytes: 512 * 1024,
        };
        let packet =
            local::acquire(&source, "https://example.com/owner/repo", &revision, scope).unwrap();
        let clock = Arc::new(AtomicU64::new(100));
        let clock2 = clock.clone();
        let root = temp.path().join("store");
        let store = Store::open(&root, move || clock2.load(Ordering::SeqCst)).unwrap();
        let admission = store.admit(packet, Retention { seconds: 1000 }).unwrap();
        let policy = BoundaryPolicy {
            schema: VERSION.into(),
            crate_root: "src/lib.rs".into(),
            manifest_path: None,
            layers: analysis
                .iter()
                .filter(|p| p.ends_with(".rs"))
                .map(|p| (p.clone(), "core".into()))
                .collect(),
            allowed: BTreeSet::new(),
            coupling_threshold: 2,
        };
        Self {
            _temp: temp,
            source,
            store,
            packet: admission.packet.packet_id,
            policy,
            clock,
        }
    }
    fn store(&self) -> &Store {
        &self.store
    }
    fn report(&self) -> structure::StructureReport {
        structure::repository_structure_reporter(self.store(), &self.packet, self.policy.clone())
            .unwrap()
    }
    fn output(&self) -> PathBuf {
        self._temp.path().join("report.json")
    }
}
fn known() -> Fixture {
    let mut f = Fixture::new(&[
        (
            "src/lib.rs",
            include_str!("fixtures/codefriend/impact/lib.rs"),
        ),
        (
            "src/api.rs",
            include_str!("fixtures/codefriend/impact/api.rs"),
        ),
        (
            "src/domain.rs",
            include_str!("fixtures/codefriend/impact/domain.rs"),
        ),
        (
            "src/storage.rs",
            include_str!("fixtures/codefriend/impact/storage.rs"),
        ),
        (
            "src/unused.rs",
            include_str!("fixtures/codefriend/impact/unused.rs"),
        ),
    ]);
    f.policy.layers.insert("src/api.rs".into(), "api".into());
    f
}
fn changes(g: &structure::StructureReport, modules: &[&str]) -> ChangeSet {
    ChangeSet {
        schema: impact::VERSION.into(),
        repository: g.record.run.repository.clone(),
        revision: g.record.run.revision.clone(),
        graph_digest: g.digest.clone(),
        targets: modules
            .iter()
            .map(|m| ChangeTarget::Module((*m).into()))
            .collect(),
    }
}
fn node<'a>(g: &'a structure::StructureReport, module: &str) -> &'a str {
    &g.nodes.iter().find(|n| n.module == module).unwrap().id
}
#[test]
fn direct_transitive_cross_boundary_and_unaffected_have_exact_witnesses() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let g = f.report();
    assert!(g.analysis_complete);
    let r = impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &["crate::storage"]))
        .unwrap();
    assert!(r.analysis_complete);
    assert_eq!(r.impacts.len(), 2);
    let api = r
        .impacts
        .iter()
        .find(|i| i.dependent_node == node(&g, "crate::api"))
        .unwrap();
    assert_eq!(
        api.path
            .iter()
            .map(|e| (e.from.as_str(), e.to.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (node(&g, "crate::api"), node(&g, "crate::domain")),
            (node(&g, "crate::domain"), node(&g, "crate::storage"))
        ]
    );
    assert_eq!(
        api.risks,
        vec!["transitive_dependency", "cross_layer_dependency"]
    );
    let domain = r
        .impacts
        .iter()
        .find(|i| i.dependent_node == node(&g, "crate::domain"))
        .unwrap();
    assert_eq!(domain.path.len(), 1);
    assert!(r
        .scoped_unimpacted_nodes
        .contains(&node(&g, "crate::unused").to_string()));
    assert_eq!(r.scoped_unimpacted_nodes.len(), 2);
    for i in &r.impacts {
        assert_eq!(i.change_digest, r.change_digest);
        assert_eq!(i.graph_revision, g.record.run.revision);
        assert_eq!(i.graph_digest, g.digest);
        assert!(!i.inference.is_empty());
    }
    assert_eq!(r.record.findings.len(), 2);
    r.record.validate().unwrap();
    assert_eq!(
        r,
        impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &["crate::storage"]))
            .unwrap()
    );
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}
#[test]
fn multiple_roots_keep_distinct_paths_and_normalize_order() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let g = f.report();
    let a = impact::change_impact_reporter(
        f.store(),
        g.clone(),
        changes(&g, &["crate::storage", "crate::domain"]),
    )
    .unwrap();
    let b = impact::change_impact_reporter(
        f.store(),
        g.clone(),
        changes(&g, &["crate::domain", "crate::storage"]),
    )
    .unwrap();
    assert_eq!(a, b);
    assert_eq!(a.impacts.len(), 3);
    assert_eq!(a.changed_nodes.len(), 2);
    assert_eq!(
        a.impacts
            .iter()
            .filter(|i| i.dependent_node == node(&g, "crate::api"))
            .count(),
        2
    );
}
#[test]
fn cycle_terminates_and_explains_each_affected_node() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = Fixture::new(&[
        ("src/lib.rs", "mod a; mod b; mod c;"),
        ("src/a.rs", "use crate::b::B;"),
        ("src/b.rs", "use crate::a::A;"),
        ("src/c.rs", "use crate::b::B;"),
    ]);
    let g = f.report();
    let r =
        impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &["crate::a"])).unwrap();
    assert_eq!(r.impacts.len(), 2);
    let b = r
        .impacts
        .iter()
        .find(|i| i.dependent_node == node(&g, "crate::b"))
        .unwrap();
    assert!(b.risks.contains(&"cycle_with_changed_root".into()));
    assert_eq!(b.path.len(), 1);
    let c = r
        .impacts
        .iter()
        .find(|i| i.dependent_node == node(&g, "crate::c"))
        .unwrap();
    assert!(!c.risks.contains(&"cycle_with_changed_root".into()));
    assert_eq!(c.path.len(), 2);
}
#[test]
fn unsupported_symbols_and_outside_modules_are_partial_not_safe() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let g = f.report();
    let mut c = changes(&g, &["crate::missing", "crate::storage"]);
    c.targets
        .push(ChangeTarget::Symbol("crate::storage::Record".into()));
    let r = impact::change_impact_reporter(f.store(), g, c).unwrap();
    assert!(!r.analysis_complete);
    assert!(r.scoped_unimpacted_nodes.is_empty());
    assert_eq!(r.impacts.len(), 2);
    assert_eq!(
        r.unknowns,
        vec![
            "module_outside_graph:crate::missing",
            "symbol_resolution_unsupported:crate::storage::Record"
        ]
    );
}
#[test]
fn unknown_graph_edges_never_become_no_impact() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = Fixture::new(&[
        ("src/lib.rs", "mod a; unknown_macro!();"),
        ("src/a.rs", "pub struct A;"),
    ]);
    let g = f.report();
    assert!(!g.analysis_complete);
    let r =
        impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &["crate::a"])).unwrap();
    assert!(!r.analysis_complete);
    assert!(r.scoped_unimpacted_nodes.is_empty());
    assert!(!r.graph.unknowns.is_empty());
    assert!(!r.unknowns.is_empty());
}
#[test]
fn stale_malformed_duplicate_and_bounded_changes_fail_precisely() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let g = f.report();
    let base = changes(&g, &["crate::storage"]);
    let mut cases = Vec::new();
    let mut c = base.clone();
    c.schema = "bad".into();
    cases.push((c, "unsupported_change_version"));
    let mut c = base.clone();
    c.repository = "wrong".into();
    cases.push((c, "change_repository_mismatch"));
    let mut c = base.clone();
    c.revision = "0".repeat(40);
    cases.push((c, "stale_change_revision"));
    let mut c = base.clone();
    c.graph_digest = "0".repeat(64);
    cases.push((c, "stale_change_graph_digest"));
    let mut c = base.clone();
    c.targets.clear();
    cases.push((c, "change_target_bounds_exceeded"));
    let mut c = base.clone();
    c.targets = vec![ChangeTarget::Module("crate".into()); 65];
    cases.push((c, "change_target_bounds_exceeded"));
    let mut c = base.clone();
    c.targets.push(c.targets[0].clone());
    cases.push((c, "duplicate_change_target"));
    let mut c = base.clone();
    c.targets = vec![ChangeTarget::Module("/private/source".into())];
    cases.push((c, "invalid_change_target"));
    for (c, message) in cases {
        assert_eq!(
            impact::change_impact_reporter(f.store(), g.clone(), c)
                .unwrap_err()
                .to_string(),
            message
        );
    }
    let mut g2 = g.clone();
    g2.edges.clear();
    assert_eq!(
        impact::change_impact_reporter(f.store(), g2, base)
            .unwrap_err()
            .to_string(),
        "structure_artifact_mismatch"
    );
}
#[test]
fn artifact_roundtrip_tamper_overwrite_deletion_and_expiry() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let g = f.report();
    let r = impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &["crate::storage"]))
        .unwrap();
    let path = f.output();
    impact::write_report(&r, f.store(), &path).unwrap();
    assert_eq!(r, impact::read_report(f.store(), &path).unwrap());
    assert!(impact::write_report(&r, f.store(), &path).is_err());
    let mut bad = r.clone();
    bad.impacts.clear();
    assert_eq!(
        bad.validate(f.store()).unwrap_err().to_string(),
        "impact_artifact_mismatch"
    );
    let mut bad = r.clone();
    bad.scoped_unimpacted_nodes
        .push(node(&g, "crate::api").into());
    assert_eq!(
        bad.validate(f.store()).unwrap_err().to_string(),
        "impact_artifact_mismatch"
    );
    f.clock.store(1200, Ordering::SeqCst);
    assert!(impact::read_report(f.store(), &path).is_err());
    f.clock.store(100, Ordering::SeqCst);
    f.store.delete(&f.packet).unwrap();
    assert!(impact::read_report(f.store(), &path).is_err());
}
#[test]
fn pair_bound_rejects_dense_cycle_without_truncating_claims() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let mut files = vec![(
        "src/lib.rs".to_string(),
        (0..33).map(|i| format!("mod m{i}; ")).collect::<String>(),
    )];
    for i in 0..33 {
        files.push((
            format!("src/m{i}.rs"),
            format!("use crate::m{}::Item;", (i + 1) % 33),
        ));
    }
    let refs: Vec<_> = files
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    let f = Fixture::new(&refs);
    let g = f.report();
    assert!(g.analysis_complete);
    let modules: Vec<_> = (0..33).map(|i| format!("crate::m{i}")).collect();
    let names: Vec<_> = modules.iter().map(String::as_str).collect();
    assert_eq!(
        impact::change_impact_reporter(f.store(), g.clone(), changes(&g, &names))
            .unwrap_err()
            .to_string(),
        "impact_pair_bounds_exceeded"
    );
}
#[test]
fn artifact_input_guards_reject_malformed_oversize_and_unsafe_paths() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let path = f.output();
    fs::write(&path, "{}").unwrap();
    assert_eq!(
        impact::read_report(f.store(), &path)
            .unwrap_err()
            .to_string(),
        "invalid_impact_artifact"
    );
    let file = fs::File::create(&path).unwrap();
    file.set_len(32 * 1024 * 1024 + 1).unwrap();
    assert_eq!(
        impact::read_report(f.store(), &path)
            .unwrap_err()
            .to_string(),
        "impact_artifact_too_large"
    );
    assert_eq!(
        impact::read_report(f.store(), f._temp.path())
            .unwrap_err()
            .to_string(),
        "impact_input_not_regular"
    );
    assert_eq!(
        impact::read_report(f.store(), &f._temp.path().join("../escape"))
            .unwrap_err()
            .to_string(),
        "artifact_parent_traversal_rejected"
    );
    #[cfg(unix)]
    {
        let link = f._temp.path().join("link.json");
        std::os::unix::fs::symlink(&path, &link).unwrap();
        assert_eq!(
            impact::read_report(f.store(), &link)
                .unwrap_err()
                .to_string(),
            "artifact_symlink_rejected"
        );
    }
}
#[test]
fn product_cli_creates_and_consumes_impact_with_strict_inputs() {
    let _guard = FIXTURE_LOCK.lock().unwrap();
    let f = known();
    let dir = f._temp.path();
    let live = dir.join("live-store");
    let graph = f.report();
    let scope = dir.join("scope.json");
    let policy = dir.join("policy.json");
    fs::write(
        &scope,
        serde_json::to_vec(&graph.record.admission.packet.scope).unwrap(),
    )
    .unwrap();
    fs::write(&policy, serde_json::to_vec(&f.policy).unwrap()).unwrap();
    let cli = |args: Vec<String>, success: bool| {
        let out = Command::new(env!("CARGO_BIN_EXE_adl"))
            .arg("codefriend")
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            out.status.success(),
            success,
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        out
    };
    let admission = cli(
        vec![
            "evidence".into(),
            "admit-local".into(),
            "--checkout".into(),
            f.source.display().to_string(),
            "--repository".into(),
            graph.record.run.repository.clone(),
            "--revision".into(),
            graph.record.run.revision.clone(),
            "--scope".into(),
            scope.display().to_string(),
            "--store".into(),
            live.display().to_string(),
            "--retention-seconds".into(),
            "3600".into(),
        ],
        true,
    );
    let packet: serde_json::Value = serde_json::from_slice(&admission.stdout).unwrap();
    let graph_path = dir.join("graph.json");
    cli(
        vec![
            "architecture".into(),
            "report".into(),
            "--store".into(),
            live.display().to_string(),
            "--packet-id".into(),
            packet["packet_id"].as_str().unwrap().into(),
            "--policy".into(),
            policy.display().to_string(),
            "--out".into(),
            graph_path.display().to_string(),
        ],
        true,
    );
    let g: structure::StructureReport =
        serde_json::from_slice(&fs::read(&graph_path).unwrap()).unwrap();
    let change_path = dir.join("changes.json");
    fs::write(
        &change_path,
        serde_json::to_vec(&changes(&g, &["crate::storage"])).unwrap(),
    )
    .unwrap();
    let output = dir.join("impact.json");
    let args = vec![
        "architecture".into(),
        "impact".into(),
        "--store".into(),
        live.display().to_string(),
        "--graph".into(),
        graph_path.display().to_string(),
        "--changes".into(),
        change_path.display().to_string(),
        "--out".into(),
        output.display().to_string(),
    ];
    let result = cli(args.clone(), true);
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["impacts"], 2);
    assert_eq!(summary["analysis_complete"], true);
    assert!(String::from_utf8_lossy(&result.stderr).contains("adl_event kind=codefriend_impact"));
    let read = cli(
        vec![
            "architecture".into(),
            "impact-read".into(),
            "--store".into(),
            live.display().to_string(),
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
        .contains("impact_output_unavailable"));
    fs::write(&change_path, "{}").unwrap();
    assert!(
        String::from_utf8_lossy(&cli(args.clone(), false).stderr).contains("invalid_change_input")
    );
    fs::File::create(&change_path)
        .unwrap()
        .set_len(128 * 1024 + 1)
        .unwrap();
    assert!(String::from_utf8_lossy(&cli(args, false).stderr).contains("change_input_too_large"));
}
