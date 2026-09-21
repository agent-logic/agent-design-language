//! PVF component: real admitted syntax/resolution and v2 fitness policy results.
//! Local temporary Git/Store only; no provider or inspected-code execution.
use adl::codefriend::{
    evidence::{store::Store, Retention},
    governance::language::{self, Policy, Rule, StaticSelector, Status},
    ingestion::{local, Scope},
    language::{AnalysisPolicy, Language, Limits, ProjectRoot},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
static SERIAL: Mutex<()> = Mutex::new(());
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().into()
}
struct Fixture {
    _temp: tempfile::TempDir,
    store: Store,
    packet: String,
    analysis: AnalysisPolicy,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new(files: &[(&str, &str, Language)]) -> Self {
        Self::new_at(files, 100)
    }
    fn new_at(files: &[(&str, &str, Language)], now: u64) -> Self {
        let t = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = t.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        for (p, s, _) in files {
            fs::write(source.join(p), s).unwrap();
        }
        fs::write(source.join("LICENSE"), "MIT fixture\n").unwrap();
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
        let rev = git(&source, &["rev-parse", "HEAD"]);
        let paths: BTreeMap<_, _> = files.iter().map(|(p, _, l)| (p.to_string(), *l)).collect();
        let packet = local::acquire(
            &source,
            "https://example.com/owner/repo",
            &rev,
            Scope {
                analysis: paths.keys().cloned().collect(),
                context: vec!["LICENSE".into()],
                max_files: files.len() + 1,
                max_bytes: 1024 * 1024,
                max_file_bytes: 512 * 1024,
            },
        )
        .unwrap();
        let clock = Arc::new(AtomicU64::new(now));
        let c = clock.clone();
        let store = Store::open(&t.path().join("store"), move || c.load(Ordering::SeqCst)).unwrap();
        let a = store.admit(packet, Retention { seconds: 100 }).unwrap();
        let roots = paths
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|language| ProjectRoot {
                language,
                root: ".".into(),
                manifest: None,
            })
            .collect();
        let analysis = AnalysisPolicy {
            schema: adl::codefriend::language::VERSION.into(),
            files: paths,
            roots,
            layers: BTreeMap::new(),
            allowed: BTreeSet::new(),
            limits: Limits {
                max_nodes: 10000,
                max_depth: 128,
                max_facts: 1000,
                max_output_bytes: 1024 * 1024,
            },
        };
        Self {
            _temp: t,
            store,
            packet: a.packet.packet_id,
            analysis,
            clock,
        }
    }
    fn policy(&self, rule: Rule) -> Policy {
        Policy {
            schema: language::VERSION.into(),
            analysis: self.analysis.clone(),
            rules: vec![rule],
        }
    }
    fn run(&self, p: &Policy) -> language::Report {
        language::evaluate(&self.store, &self.packet, p, 101).unwrap()
    }
}
fn static_rule(path: &str, selector: StaticSelector) -> Rule {
    Rule::ForbiddenStaticImport {
        id: "rule".into(),
        source_path: path.into(),
        selector,
    }
}
#[test]
fn four_languages_literal_violations_and_supported_absence() {
    let _g = SERIAL.lock().unwrap();
    for (path, source, lang, bad, good) in [
        (
            "lib.rs",
            "use std::fs;",
            Language::Rust,
            StaticSelector::RustUse {
                prefix: vec!["std".into(), "fs".into()],
            },
            StaticSelector::RustUse {
                prefix: vec!["std".into(), "net".into()],
            },
        ),
        (
            "Main.java",
            "import java.util.List; class Main {}",
            Language::Java,
            StaticSelector::JavaTypeImport {
                prefix: vec!["java".into(), "util".into()],
            },
            StaticSelector::JavaTypeImport {
                prefix: vec!["java".into(), "net".into()],
            },
        ),
        (
            "main.py",
            "import os as operating\n",
            Language::Python,
            StaticSelector::PythonModule {
                prefix: vec!["os".into()],
            },
            StaticSelector::PythonModule {
                prefix: vec!["socket".into()],
            },
        ),
        (
            "main.js",
            "import thing from 'package';",
            Language::JavaScript,
            StaticSelector::JavaScriptModule {
                specifier: "package".into(),
            },
            StaticSelector::JavaScriptModule {
                specifier: "different".into(),
            },
        ),
    ] {
        let f = Fixture::new(&[(path, source, lang)]);
        let r = f.run(&f.policy(static_rule(path, bad)));
        assert_eq!(r.status, Status::Fail, "{path}: {r:?}");
        assert!(!r.results[0].violations.is_empty());
        let r = f.run(&f.policy(static_rule(path, good)));
        assert_eq!(r.status, Status::Pass, "{path}: {r:?}");
    }
}
#[test]
fn unresolved_and_conditional_imports_never_pass() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[("lib.rs", "use super::hidden;", Language::Rust)]);
    let r = f.run(&f.policy(static_rule(
        "lib.rs",
        StaticSelector::RustUse {
            prefix: vec!["hidden".into()],
        },
    )));
    assert_eq!(r.status, Status::Unknown);
    assert!(!r.results[0].unknowns.is_empty());
}
#[test]
fn conclusive_violation_retains_other_unknowns() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[(
        "lib.rs",
        "use std::fs; #[cfg(feature=\"x\")] mod hidden { use std::net; }",
        Language::Rust,
    )]);
    let r = f.run(&f.policy(static_rule(
        "lib.rs",
        StaticSelector::RustUse {
            prefix: vec!["std".into(), "fs".into()],
        },
    )));
    assert_eq!(r.status, Status::Fail);
    assert!(!r.results[0].unknowns.is_empty());
    assert_eq!(
        r.record.run.completion,
        adl::codefriend::evidence::contracts::Completion::Incomplete
    );
}
#[test]
fn resolved_edges_fail_but_external_edges_are_unknown() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[
        ("dep.js", "export const value=1;", Language::JavaScript),
        (
            "main.js",
            "import {value} from './dep.js';",
            Language::JavaScript,
        ),
    ]);
    let p = f.policy(Rule::ForbiddenResolvedEdge {
        id: "edge".into(),
        source_path: "main.js".into(),
        language: Language::JavaScript,
        forbidden_targets: BTreeSet::from(["dep.js".into()]),
    });
    assert_eq!(f.run(&p).status, Status::Fail);
    let external = Fixture::new(&[
        ("dep.js", "export const value=1;", Language::JavaScript),
        (
            "main.js",
            "import value from 'external';",
            Language::JavaScript,
        ),
    ]);
    let p = external.policy(Rule::ForbiddenResolvedEdge {
        id: "edge".into(),
        source_path: "main.js".into(),
        language: Language::JavaScript,
        forbidden_targets: BTreeSet::from(["dep.js".into()]),
    });
    assert_eq!(external.run(&p).status, Status::Unknown);
}
#[test]
fn recomputation_retention_and_resource_failures_are_errors() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[("main.py", "import os", Language::Python)]);
    let p = f.policy(static_rule(
        "main.py",
        StaticSelector::PythonModule {
            prefix: vec!["os".into()],
        },
    ));
    let mut r = f.run(&p);
    r.status = Status::Pass;
    r.digest.clear();
    r.digest = adl::codefriend::evidence::hash(&r).unwrap();
    assert!(r.validate(&f.store, 101).is_err());
    let mut bounded = p.clone();
    bounded.analysis.limits.max_nodes = 1;
    assert!(language::evaluate(&f.store, &f.packet, &bounded, 101).is_err());
    f.clock.store(200, Ordering::SeqCst);
    assert!(language::evaluate(&f.store, &f.packet, &p, 101).is_err());
}

#[test]
fn raw_rust_identifier_is_unknown_and_v2_dispatch_preserves_payload() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[("lib.rs", "use r#module::item;", Language::Rust)]);
    let p = f.policy(static_rule(
        "lib.rs",
        StaticSelector::RustUse {
            prefix: vec!["module".into()],
        },
    ));
    let r = f.run(&p);
    assert_eq!(r.status, Status::Unknown);
    use adl::codefriend::governance::artifact::{FitnessArtifact, PolicyArtifact};
    let bytes = serde_json::to_vec(&r).unwrap();
    let dispatch: FitnessArtifact = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(bytes, serde_json::to_vec(&dispatch).unwrap());
    assert!(!dispatch.passes());
    dispatch.validate(&f.store, 101).unwrap();
    let pbytes = serde_json::to_vec(&p).unwrap();
    let pd: PolicyArtifact = serde_json::from_slice(&pbytes).unwrap();
    assert_eq!(pbytes, serde_json::to_vec(&pd).unwrap());
    pd.validate().unwrap();
    let malformed = format!(
        "{{\"schema\":\"codefriend.fitness.v1\",{}",
        &String::from_utf8(bytes).unwrap()[1..]
    );
    assert!(serde_json::from_str::<FitnessArtifact>(&malformed).is_err());
}
#[test]
fn no_proven_edge_does_not_claim_complete_dependency_coverage() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[
        ("dep.js", "export const value=1;", Language::JavaScript),
        (
            "main.js",
            "const loaded = require(name);",
            Language::JavaScript,
        ),
    ]);
    let p = f.policy(Rule::ForbiddenResolvedEdge {
        id: "edge".into(),
        source_path: "main.js".into(),
        language: Language::JavaScript,
        forbidden_targets: BTreeSet::from(["dep.js".into()]),
    });
    let r = f.run(&p);
    assert_eq!(r.status, Status::Unknown);
    assert!(r.results[0]
        .unknowns
        .iter()
        .any(|r| r == "structural_edge_coverage_incomplete"));
}

#[test]
fn embedded_record_overhead_cannot_escape_the_total_report_cap() {
    let _g = SERIAL.lock().unwrap();
    let f = Fixture::new(&[("main.py", "import os", Language::Python)]);
    let p = f.policy(static_rule(
        "main.py",
        StaticSelector::PythonModule {
            prefix: vec!["os".into()],
        },
    ));
    let mut r = f.run(&p);
    let witness = r.results[0].violations[0].clone();
    let cap = 4 * 1024 * 1024;
    let per = serde_json::to_vec(&witness).unwrap().len() + 1;
    // The witnesses alone fit; the actual policy/admission/run/findings envelope does not.
    r.results[0].violations = vec![witness; (cap - 1) / per];
    assert!(serde_json::to_vec(&r.results[0].violations).unwrap().len() <= cap);
    assert!(serde_json::to_vec(&r).unwrap().len() > cap);
    let err = r.validate(&f.store, 101).unwrap_err().to_string();
    assert!(err.contains("fitness_output_limit"), "{err}");
}

#[test]
fn real_cli_v2_run_read_preserves_status_and_raw_input_limit() {
    let _g = SERIAL.lock().unwrap();
    for (source, selector, expected) in [
        (
            "import os",
            StaticSelector::PythonModule {
                prefix: vec!["os".into()],
            },
            1,
        ),
        (
            "import os",
            StaticSelector::PythonModule {
                prefix: vec!["socket".into()],
            },
            0,
        ),
        (
            "from .pkg import name",
            StaticSelector::PythonFrom {
                module_prefix: vec!["pkg".into()],
                member: None,
            },
            2,
        ),
    ] {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let f = Fixture::new_at(&[("main.py", source, Language::Python)], now);
        let policy = f.policy(static_rule("main.py", selector));
        let policy_path = f._temp.path().join("policy.json");
        fs::write(&policy_path, serde_json::to_vec(&policy).unwrap()).unwrap();
        let out_path = f._temp.path().join("fitness.json");
        let store_path = f._temp.path().join("store");
        let packet = f.packet.clone();
        drop(f.store);
        let run = Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "fitness", "run", "--store"])
            .arg(&store_path)
            .args(["--packet-id", &packet, "--policy"])
            .arg(&policy_path)
            .arg("--out")
            .arg(&out_path)
            .output()
            .unwrap();
        assert_eq!(
            run.status.code(),
            Some(expected),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        let report: language::Report = serde_json::from_slice(&run.stdout).unwrap();
        assert_eq!(report.status.exit_code(), expected);
        assert!(out_path.is_file());
        let read = Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "fitness", "read", "--store"])
            .arg(&store_path)
            .arg("--input")
            .arg(&out_path)
            .output()
            .unwrap();
        assert_eq!(read.status.code(), Some(expected));
        assert_eq!(run.stdout, read.stdout);
        // Whitespace counts toward raw bytes even when semantic JSON remains small.
        let mut oversized = fs::read(&out_path).unwrap();
        oversized.resize(4 * 1024 * 1024 + 1, b' ');
        let over = f._temp.path().join("oversized.json");
        fs::write(&over, oversized).unwrap();
        let denied = Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "fitness", "read", "--store"])
            .arg(&store_path)
            .arg("--input")
            .arg(&over)
            .output()
            .unwrap();
        assert_eq!(denied.status.code(), Some(2));
        let body: serde_json::Value = serde_json::from_slice(&denied.stdout).unwrap();
        assert_eq!(body["schema"], language::VERSION);
        assert_eq!(body["report_available"], false);
    }
}

// PVF component; deterministic local Git/Store. Proves privacy omission coverage
// and witnessed failure precedence; no provider or release qualification.
#[test]
fn privacy_omissions_are_unknown_while_available_violations_still_fail() {
    let _g = SERIAL.lock().unwrap();
    let fixture = Fixture::new(&[
        ("safe.js", "import fs from 'node:fs';", Language::JavaScript),
        (
            "private.js",
            "const api_key = 'fixture-private-value';",
            Language::JavaScript,
        ),
    ]);
    let mut policy = fixture.policy(static_rule(
        "private.js",
        StaticSelector::JavaScriptModule {
            specifier: "node:fs".into(),
        },
    ));
    let report = fixture.run(&policy);
    assert_eq!(report.status, Status::Unknown);
    assert!(report.results[0].violations.is_empty());
    assert!(report.results[0]
        .unknowns
        .iter()
        .any(|s| s == "source_excluded_by_privacy_filter"));
    assert_eq!(
        report.record.run.completion,
        adl::codefriend::evidence::contracts::Completion::Incomplete
    );
    let serialized = serde_json::to_string(&report).unwrap();
    assert!(!serialized.contains("fixture-private-value"));
    policy.rules.push(Rule::ForbiddenStaticImport {
        id: "visible_violation".into(),
        source_path: "safe.js".into(),
        selector: StaticSelector::JavaScriptModule {
            specifier: "node:fs".into(),
        },
    });
    let report = fixture.run(&policy);
    assert_eq!(report.status, Status::Fail);
    assert_eq!(report.results[1].violations.len(), 1);
    report.validate(&fixture.store, 101).unwrap();
}

// PVF component: retained source with a genuinely filtered declared manifest.
#[test]
fn privacy_filtered_manifest_continues_with_explicit_dependency_gap() {
    let _g = SERIAL.lock().unwrap();
    let fixture = Fixture::new(&[
        ("main.js", "export const answer = 42;", Language::JavaScript),
        (
            "package.json",
            r#"{"api_key":"manifest-private-fixture"}"#,
            Language::JavaScript,
        ),
    ]);
    let mut policy = fixture.policy(static_rule(
        "main.js",
        StaticSelector::JavaScriptModule {
            specifier: "node:fs".into(),
        },
    ));
    policy.analysis.roots[0].manifest = Some("package.json".into());
    let report = fixture.run(&policy);
    assert_eq!(report.status, Status::Unknown);
    assert!(report.results[0]
        .unknowns
        .iter()
        .any(|s| s == "project_manifest_excluded_by_privacy_filter"));
    assert!(!serde_json::to_string(&report)
        .unwrap()
        .contains("manifest-private-fixture"));
    report.validate(&fixture.store, 101).unwrap();
    policy.analysis.roots[0].manifest = Some("missing.json".into());
    assert!(language::evaluate(&fixture.store, &fixture.packet, &policy, 101).is_err());
}
