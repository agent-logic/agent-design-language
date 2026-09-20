//! PVF runtime: deterministic local CPU/filesystem production graph and CLI integration.
//! Required acceptance gate; no provider, network, source builds or repository script execution.
use adl::codefriend::{
    architecture::structure::{self, BoundaryPolicy, VERSION},
    evidence::{store::Store, Retention},
    ingestion::{local, Scope},
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
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
    Fixture::new(&[
        (
            "src/lib.rs",
            include_str!("fixtures/codefriend/architecture/allowed/lib.rs"),
        ),
        (
            "src/api.rs",
            include_str!("fixtures/codefriend/architecture/allowed/api.rs"),
        ),
        (
            "src/domain.rs",
            include_str!("fixtures/codefriend/architecture/allowed/domain.rs"),
        ),
        (
            "src/helper.rs",
            include_str!("fixtures/codefriend/architecture/allowed/helper.rs"),
        ),
    ])
}
#[test]
fn allowed_graph_is_deterministic_and_evidence_linked() {
    let f = known();
    let r = f.report();
    assert!(r.analysis_complete);
    assert_eq!(r, f.report());
    assert_eq!(r.nodes.len(), 4);
    assert_eq!(r.edges.len(), 2);
    assert_eq!(r.record.findings.len(), 1);
    assert_eq!(r.record.findings[0].rule, "name_connascence");
    for edge in &r.edges {
        assert_eq!(edge.location.line, 1);
        assert!(r
            .record
            .admission
            .evidence
            .iter()
            .any(|e| e.id == edge.location.evidence_id && e.path == edge.location.path));
    }
    r.record.validate().unwrap();
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}
#[test]
fn explicit_boundary_policy_distinguishes_allowed_and_forbidden() {
    let mut f = known();
    f.policy.layers.insert("src/api.rs".into(), "api".into());
    let denied = f.report();
    assert_eq!(
        denied
            .record
            .findings
            .iter()
            .filter(|x| x.rule == "forbidden_boundary")
            .count(),
        1
    );
    f.policy.allowed.insert(("api".into(), "core".into()));
    assert!(!f
        .report()
        .record
        .findings
        .iter()
        .any(|x| x.rule == "forbidden_boundary"));
}
#[test]
fn cycle_and_coupling_report_real_directed_edges() {
    let mut f = Fixture::new(&[
        ("src/lib.rs", "pub mod a; pub mod b; pub mod c;"),
        (
            "src/a.rs",
            "use crate::b::B; use crate::c::C; pub struct A;",
        ),
        ("src/b.rs", "use crate::a::A; pub struct B;"),
        ("src/c.rs", "pub struct C;"),
    ]);
    f.policy.coupling_threshold = 1;
    let r = f.report();
    assert_eq!(
        r.record
            .findings
            .iter()
            .filter(|x| x.rule == "dependency_cycle")
            .count(),
        2
    );
    assert_eq!(
        r.record
            .findings
            .iter()
            .filter(|x| x.rule == "module_fanout")
            .count(),
        1
    );
    assert_eq!(r.edges.len(), 3);
}
#[test]
fn unsupported_constructs_and_external_edges_cannot_claim_complete() {
    for source in [
        "use external::Thing;",
        "use crate::*;",
        "macro_rules! hidden { () => {} }",
        "#[cfg(feature=\"x\")] pub mod absent;",
        "pub mod inner { pub fn f(){} }",
        "fn f(){ dynamic(); }",
        "fn f(x: X){ x.dispatch(); }",
    ] {
        let f = Fixture::new(&[("src/lib.rs", source)]);
        let r = f.report();
        assert!(!r.analysis_complete, "{source}");
        assert!(!r.unknowns.is_empty());
        assert_eq!(
            r.record.run.completion,
            adl::codefriend::evidence::contracts::Completion::Incomplete
        );
    }
}
#[test]
fn syntax_uncertainty_preserves_references_and_exact_reasons() {
    for (source, reasons, references) in [
        (
            "#[cfg(feature = \"optional\")] use crate::dep::{Thing, Other as Alias};",
            &[
                "alias_uses_not_resolved",
                "attribute_semantics_not_analyzed",
            ][..],
            2,
        ),
        (
            "fn f<T: external::Trait>(x: T) { (x)(); }",
            &[
                "trait_resolution_not_performed",
                "unqualified_or_external_path_not_resolved",
                "type_resolution_not_performed",
                "dynamic_call_not_resolved",
            ][..],
            0,
        ),
        (
            "type Item = <crate::dep::Thing as external::Trait>::Item;",
            &[
                "type_resolution_not_performed",
                "unqualified_or_external_path_not_resolved",
            ][..],
            1,
        ),
    ] {
        let root = format!("mod dep;\n{source}\n");
        let f = Fixture::new(&[("src/lib.rs", &root), ("src/dep.rs", "")]);
        let r = f.report();
        assert!(!r.analysis_complete, "{source}");
        assert_eq!(r.edges.len(), references, "{source}");
        let observed: BTreeSet<_> = r.unknowns.iter().map(|u| u.reason.as_str()).collect();
        assert_eq!(observed, reasons.iter().copied().collect(), "{source}");
        for unknown in &r.unknowns {
            assert_eq!(unknown.location.path, "src/lib.rs");
            assert_eq!(unknown.location.line, 2);
            assert!(r
                .record
                .admission
                .evidence
                .iter()
                .any(|e| e.id == unknown.location.evidence_id && e.path == unknown.location.path));
        }
    }
    let f = Fixture::new(&[(
        "src/lib.rs",
        "/// Ordinary documentation\nfn f(x: u32) -> u32 { x }",
    )]);
    let r = f.report();
    assert!(r.analysis_complete);
    assert!(r.unknowns.is_empty());
    assert!(r.edges.is_empty());
}
#[test]
fn missing_module_parse_error_and_unsupported_language_are_partial() {
    for files in [
        vec![("src/lib.rs", "mod missing;")],
        vec![("src/lib.rs", "not valid rust { !")],
        vec![("src/lib.rs", ""), ("script.py", "print('hello')")],
    ] {
        let r = Fixture::new(&files).report();
        assert!(!r.analysis_complete);
        assert!(!r.unknowns.is_empty());
    }
}
#[test]
fn manifest_dependencies_are_declared_not_invented_resolved_code() {
    let mut f = Fixture::new(&[
        ("src/lib.rs", ""),
        (
            "Cargo.toml",
            "[package]\nname='fixture'\nversion='0.1.0'\n[dependencies]\nserde='1'\n",
        ),
    ]);
    f.policy.manifest_path = Some("Cargo.toml".into());
    let r = f.report();
    assert!(r
        .nodes
        .iter()
        .any(|n| n.module == "external::dependencies::serde"));
    assert!(r.edges.iter().any(|e| e.kind == "manifest_declaration"));
    assert!(!r.analysis_complete);
}
#[test]
fn root_super_paths_and_missing_boundaries_do_not_invent_edges() {
    let f = Fixture::new(&[
        ("src/lib.rs", "use super::outside; mod a;"),
        ("src/a.rs", "use crate::missing::Thing;"),
    ]);
    let mut policy = f.policy.clone();
    policy.layers.remove("src/a.rs");
    let r = structure::repository_structure_reporter(f.store(), &f.packet, policy).unwrap();
    assert!(r.edges.is_empty());
    for reason in [
        "path_escapes_crate",
        "reference_target_not_in_graph",
        "missing_boundary_assignment",
    ] {
        assert!(r.unknowns.iter().any(|u| u.reason == reason));
    }
}
#[test]
fn output_readback_rejects_tampering_and_never_overwrites() {
    let f = known();
    let r = f.report();
    let store = f.store();
    let path = f.output();
    structure::write_report(&r, store, &path).unwrap();
    assert_eq!(structure::read_report(store, &path).unwrap(), r);
    assert!(structure::write_report(&r, store, &path).is_err());
    let mut wrong = r.clone();
    wrong.edges.clear();
    fs::write(&path, serde_json::to_vec(&wrong).unwrap()).unwrap();
    assert!(structure::read_report(store, &path).is_err());
    wrong = r;
    wrong.record.findings[0].evidence = vec!["0".repeat(64)];
    fs::write(&path, serde_json::to_vec(&wrong).unwrap()).unwrap();
    assert!(structure::read_report(store, &path).is_err());
}
#[test]
fn deletion_and_expiry_deny_saved_report_retrieval() {
    for delete in [false, true] {
        let f = known();
        let r = f.report();
        let path = f.output();
        let store = f.store();
        structure::write_report(&r, store, &path).unwrap();
        if delete {
            store.delete(&f.packet).unwrap();
        } else {
            f.clock.store(1101, Ordering::SeqCst);
        }
        assert!(structure::read_report(store, &path).is_err());
        assert!(
            structure::repository_structure_reporter(store, &f.packet, f.policy.clone()).is_err()
        );
    }
}
#[test]
fn invalid_policy_and_bounds_fail_closed() {
    let f = known();
    let mut p = f.policy.clone();
    p.coupling_threshold = 0;
    assert!(structure::repository_structure_reporter(f.store(), &f.packet, p).is_err());
    let mut p = f.policy.clone();
    p.layers.insert("outside.rs".into(), "core".into());
    assert!(structure::repository_structure_reporter(f.store(), &f.packet, p).is_err());
    let mut p = f.policy.clone();
    p.schema = "other".into();
    assert!(structure::repository_structure_reporter(f.store(), &f.packet, p).is_err());
}
#[cfg(unix)]
#[test]
fn output_symlinks_are_rejected() {
    let f = known();
    let r = f.report();
    let target = f._temp.path().join("target");
    fs::write(&target, "keep").unwrap();
    std::os::unix::fs::symlink(&target, f.output()).unwrap();
    assert!(structure::write_report(&r, f.store(), &f.output()).is_err());
    assert_eq!(fs::read_to_string(target).unwrap(), "keep");
}
fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn real_cli_admits_reports_and_reads_persisted_artifact() {
    let f = known();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let admission = f.store().get(&f.packet).unwrap();
    let packet_file = f._temp.path().join("packet.json");
    fs::write(&packet_file, serde_json::to_vec(&admission.packet).unwrap()).unwrap();
    let root = f._temp.path().join("cli-store");
    let policy = f._temp.path().join("policy.json");
    fs::write(&policy, serde_json::to_vec(&f.policy).unwrap()).unwrap();
    let admitted = cli(&[
        "codefriend",
        "evidence",
        "admit-packet",
        "--input",
        packet_file.to_str().unwrap(),
        "--store",
        root.to_str().unwrap(),
        "--retention-seconds",
        "1000",
    ]);
    assert!(
        admitted.status.success(),
        "{}",
        String::from_utf8_lossy(&admitted.stderr)
    );
    let a: serde_json::Value = serde_json::from_slice(&admitted.stdout).unwrap();
    assert!(a["expires_at"].as_u64().unwrap() > now);
    let out = f.output();
    let args = [
        "codefriend",
        "architecture",
        "report",
        "--store",
        root.to_str().unwrap(),
        "--packet-id",
        &f.packet,
        "--policy",
        policy.to_str().unwrap(),
        "--out",
        out.to_str().unwrap(),
    ];
    let result = cli(&args);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["edges"], 2);
    assert_eq!(summary["analysis_complete"], true);
    assert!(String::from_utf8_lossy(&result.stderr).contains("adl_event"));
    let read = cli(&[
        "codefriend",
        "architecture",
        "read",
        "--store",
        root.to_str().unwrap(),
        "--input",
        out.to_str().unwrap(),
    ]);
    assert!(read.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&read.stdout).unwrap(),
        summary
    );
    assert!(!cli(&args).status.success());
    assert_eq!(git(&f.source, &["status", "--porcelain"]), "");
}

#[test]
fn malformed_root_with_manifest_returns_partial_without_dangling_edges() {
    let mut f = Fixture::new(&[
        ("src/lib.rs", "invalid rust { !"),
        ("Cargo.toml", "[dependencies]\nserde='1'\n"),
    ]);
    f.policy.manifest_path = Some("Cargo.toml".into());
    let r = f.report();
    assert!(!r.analysis_complete);
    assert!(r
        .edges
        .iter()
        .all(|e| r.nodes.iter().any(|n| n.id == e.from) && r.nodes.iter().any(|n| n.id == e.to)));
}
#[test]
fn policy_changes_invalidate_run_compatibility() {
    let mut f = known();
    let old = f.report();
    f.policy.coupling_threshold = 1;
    let new = f.report();
    assert_ne!(old.record.run.id, new.record.run.id);
    assert_ne!(old.record.run.lane_versions, new.record.run.lane_versions);
}
#[test]
fn extern_crate_and_unresolved_types_are_explicit_unknowns() {
    for source in ["extern crate external;", "pub fn f(_:External) {}"] {
        let r = Fixture::new(&[("src/lib.rs", source)]).report();
        assert!(!r.analysis_complete);
        assert!(!r.unknowns.is_empty());
    }
}

#[test]
fn pathological_rust_is_partial_before_recursive_parsing() {
    let sources = [
        format!("pub fn f() {{ {}true }}", "return ".repeat(4_000)),
        format!("pub fn f() {{ let _ = {}true; }}", "!".repeat(12_000)),
        format!(
            "pub fn f() {{ let _ = {}true{}; }}",
            "(".repeat(12_000),
            ")".repeat(12_000)
        ),
        format!(
            "type T = {}bool{};",
            "Vec<".repeat(6_000),
            ">".repeat(6_000)
        ),
        format!("// {}\npub fn f() {{}}", "a".repeat(400 * 1024)),
    ];
    for source in &sources {
        let f = Fixture::new(&[("src/lib.rs", source)]);
        let report = f.report();
        assert!(!report.analysis_complete);
        assert!(report.nodes.is_empty());
        assert!(report.edges.is_empty());
        assert!(serde_json::to_string(&report.unknowns)
            .unwrap()
            .contains("rust_syntax_complexity_limit"));
    }
    let normal = Fixture::new(&[(
        "src/lib.rs",
        "pub fn f() -> bool { let x = !!!true; x && (false || true) }",
    )]);
    let report = normal.report();
    assert!(report.analysis_complete);
    assert_eq!(report.nodes.len(), 1);
}
