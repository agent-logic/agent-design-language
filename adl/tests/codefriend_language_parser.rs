//! PVF component: actual pinned parsers over genuine local Git admissions.
//! Source/coverage/resource evidence only, not full semantic or installed review qualification.
use adl::codefriend::{
    evidence::{store::Store, Admission, Retention},
    ingestion::{local, Scope},
    language::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
static PARSER_TEST: std::sync::Mutex<()> = std::sync::Mutex::new(());
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
    admission: Admission,
    store: Store,
    clock: std::sync::Arc<std::sync::atomic::AtomicU64>,
    _temp: tempfile::TempDir,
}
impl std::ops::Deref for Fixture {
    type Target = Admission;
    fn deref(&self) -> &Admission {
        &self.admission
    }
}
impl Fixture {
    fn analyze(&self, policy: &AnalysisPolicy, now: u64) -> anyhow::Result<AnalysisReport> {
        owner::analyze(&self.store, &self.admission.packet.packet_id, policy, now)
    }
}
fn fixture(inputs: &[(&str, &str, Language)]) -> (Fixture, AnalysisPolicy) {
    let t = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    git(t.path(), &["init", "-b", "main"]);
    git(
        t.path(),
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    for (p, s, _) in inputs {
        fs::create_dir_all(t.path().join(p).parent().unwrap()).unwrap();
        fs::write(t.path().join(p), s).unwrap();
    }
    fs::write(t.path().join("LICENSE"), "MIT fixture\n").unwrap();
    git(t.path(), &["add", "."]);
    git(
        t.path(),
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
    let rev = git(t.path(), &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        t.path(),
        "https://example.com/owner/repo",
        &rev,
        Scope {
            analysis: inputs
                .iter()
                .map(|(p, _, _)| p.to_string())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            context: vec!["LICENSE".into()],
            max_files: inputs.len() + 1,
            max_bytes: 1024 * 1024,
            max_file_bytes: 512 * 1024,
        },
    )
    .unwrap();
    let clock = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(100));
    let clock2 = clock.clone();
    let store = Store::open(&t.path().join("evidence-store"), move || {
        clock2.load(std::sync::atomic::Ordering::SeqCst)
    })
    .unwrap();
    let a = store.admit(packet, Retention { seconds: 100 }).unwrap();
    let p = AnalysisPolicy {
        schema: VERSION.into(),
        files: inputs.iter().map(|(p, _, l)| (p.to_string(), *l)).collect(),
        roots: inputs
            .iter()
            .map(|(_, _, l)| *l)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|l| ProjectRoot {
                language: l,
                root: ".".into(),
                manifest: None,
            })
            .collect(),
        layers: BTreeMap::new(),
        allowed: BTreeSet::new(),
        limits: Limits {
            max_nodes: 1000,
            max_depth: 32,
            max_facts: 100,
            max_output_bytes: 100000,
        },
    };
    (
        Fixture {
            admission: a,
            store,
            clock,
            _temp: t,
        },
        p,
    )
}

#[test]
fn four_modern_language_sources_emit_actual_named_facts() {
    let _guard = PARSER_TEST.lock().unwrap();
    let(a,p)=fixture(&[("Main.java","package demo; import java.util.List; public record Main(int value) {}",Language::Java),("lib.rs","use std::fmt; pub struct Item {} pub async fn run() {}",Language::Rust),("main.js","import {x} from './dep.js'; export class Box { value() { return x?.value ?? 0; } }",Language::JavaScript),("main.py","from typing import TypeVar\nclass Box[T]:\n    def value(self):\n        return f'value={1}'\n",Language::Python)]);
    let r = a.analyze(&p, 101).unwrap();
    r.validate(&a, &p, 101).unwrap();
    for f in &r.files {
        assert_eq!(f.coverage.syntax, Coverage::Complete, "{:?}", f.coverage);
        assert_eq!(f.coverage.semantics, Coverage::Unsupported);
        assert!(!f.facts.declarations.is_empty(), "{}", f.coverage.path);
        assert!(!f.facts.imports.is_empty(), "{}", f.coverage.path);
    }
    assert_eq!(r.toolchain.len(), 4);
}
#[test]
fn malformed_source_retains_error_spans_and_partial_coverage() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[(
        "main.js",
        "function broken( { return ;",
        Language::JavaScript,
    )]);
    let r = a.analyze(&p, 101).unwrap();
    let c = &r.files[0].coverage;
    assert_eq!(c.syntax, Coverage::Partial);
    for error in c
        .diagnostics
        .iter()
        .filter(|d| d.code == "parse_error")
        .filter_map(|d| d.span)
    {
        assert!(
            r.files[0]
                .facts
                .declarations
                .iter()
                .all(|d| !(error.start_byte <= d.span.start_byte
                    && d.span.end_byte <= error.end_byte))
        );
        assert!(
            r.files[0]
                .facts
                .references
                .iter()
                .all(|d| !(error.start_byte <= d.span.start_byte
                    && d.span.end_byte <= error.end_byte))
        );
    }
    assert_ne!(c.structure, Coverage::Complete);
    assert!(c
        .diagnostics
        .iter()
        .any(|d| matches!(d.code.as_str(), "parse_error" | "missing_syntax") && d.span.is_some()));
}
#[test]
fn actual_parser_obeys_node_depth_fact_and_output_limits() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[(
        "main.js",
        "const a = (((((1))))); const b = 2;",
        Language::JavaScript,
    )]);
    for (kind, expected) in [
        (0, "traversal_limit"),
        (1, "traversal_limit"),
        (2, "fact_limit"),
        (3, "output_limit"),
    ] {
        let mut bounded = p.clone();
        match kind {
            0 => bounded.limits.max_nodes = 1,
            1 => bounded.limits.max_depth = 1,
            2 => bounded.limits.max_facts = 1,
            _ => bounded.limits.max_output_bytes = 1,
        };
        let err = a.analyze(&bounded, 101).unwrap_err().to_string();
        assert!(err.contains(expected), "{err}");
    }
}
#[test]
fn extension_mismatch_is_explicit_not_false_syntax_success() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[("main.py", "value = 1", Language::Java)]);
    let r = a.analyze(&p, 101).unwrap();
    assert_eq!(r.files[0].coverage.syntax, Coverage::Unsupported);
    assert_eq!(r.files[0].facts, SourceFacts::default());
    assert!(r.files[0]
        .coverage
        .diagnostics
        .iter()
        .any(|d| d.code == "unsupported_source_kind"));
}
#[test]
fn parser_results_are_deterministic_and_unicode_spans_are_source_backed() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[(
        "main.js",
        "const café = 1; console.log(café);",
        Language::JavaScript,
    )]);
    let first = a.analyze(&p, 101).unwrap();
    let second = a.analyze(&p, 101).unwrap();
    assert_eq!(first, second);
    let d = first.files[0]
        .facts
        .declarations
        .iter()
        .find(|d| d.name == "café")
        .unwrap();
    assert_eq!(
        d.span,
        Span {
            start_byte: 6,
            end_byte: 11
        }
    );
}

#[test]
fn deleted_original_store_entry_cannot_be_replaced_by_retained_snapshot() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[("main.py", "value = 1", Language::Python)]);
    let original = a.admission.clone();
    a.analyze(&p, 101).unwrap();
    a.store.delete(&a.packet.packet_id).unwrap();
    assert!(a.analyze(&p, 101).is_err());
    original.validate().unwrap();
}
#[test]
fn store_clock_expiry_rejects_old_caller_time_and_preserves_original_deadline() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[("main.py", "value = 1", Language::Python)]);
    let digest = a.digest.clone();
    let expires = a.expires_at;
    a.analyze(&p, 101).unwrap();
    assert_eq!(a.store.get(&a.packet.packet_id).unwrap().digest, digest);
    a.clock.store(expires, std::sync::atomic::Ordering::SeqCst);
    assert!(a.analyze(&p, 101).is_err());
    assert_eq!(a.expires_at, expires);
}

#[test]
fn expiry_between_extraction_and_return_discards_the_report() {
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    };
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, p) = fixture(&[("main.py", "value = 1", Language::Python)]);
    let directory = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    let armed = Arc::new(AtomicBool::new(false));
    let reads = Arc::new(AtomicUsize::new(0));
    let arm_clock = armed.clone();
    let clock_reads = reads.clone();
    let store = Store::open(&directory.path().join("store"), move || {
        if arm_clock.load(Ordering::SeqCst) && clock_reads.fetch_add(1, Ordering::SeqCst) > 0 {
            200
        } else {
            100
        }
    })
    .unwrap();
    let original = store
        .admit(a.packet.clone(), Retention { seconds: 100 })
        .unwrap();
    assert_eq!(original.digest, a.digest);
    armed.store(true, Ordering::SeqCst);
    let error = owner::analyze(&store, &original.packet.packet_id, &p, 101).unwrap_err();
    assert_eq!(error.to_string(), "evidence_expired");
    assert!(reads.load(Ordering::SeqCst) >= 2);
    assert_eq!(
        store
            .get(&original.packet.packet_id)
            .unwrap_err()
            .to_string(),
        "evidence_deleted"
    );
}

#[test]
fn four_languages_resolve_actual_imports_to_original_admitted_sources() {
    let _guard = PARSER_TEST.lock().unwrap();
    let cases = [
        (
            Language::Rust,
            "lib.rs",
            "mod other; use crate::other;",
            "other.rs",
            "pub fn value() {}",
        ),
        (
            Language::Java,
            "Main.java",
            "package demo; import demo.Other; public class Main {}",
            "Other.java",
            "package demo; public class Other {}",
        ),
        (
            Language::Python,
            "main.py",
            "import other\n",
            "other.py",
            "value = 1\n",
        ),
        (
            Language::JavaScript,
            "main.js",
            "import {value} from './oth\\u0065r.js';",
            "other.js",
            "export const value = 1;",
        ),
    ];
    for (language, path, source, target, target_source) in cases {
        let (a, policy) = fixture(&[(path, source, language), (target, target_source, language)]);
        let report = a.analyze(&policy, 101).unwrap();
        let file = report
            .files
            .iter()
            .find(|f| f.coverage.path == path)
            .unwrap();
        assert!(
            file.facts
                .imports
                .iter()
                .any(|i| i.target_path.as_deref() == Some(target)),
            "{language:?}: {:?}",
            file
        );
        assert_eq!(file.coverage.semantics, Coverage::Unsupported);
        assert_eq!(report.admission_digest, a.digest);
        report.validate(&a, &policy, 101).unwrap();
    }
}

#[test]
fn conditional_rust_ancestors_cannot_resolve_descendant_imports() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, policy) = fixture(&[
        (
            "lib.rs",
            "#[cfg(any())] mod hidden { mod child; } use crate::hidden::child;",
            Language::Rust,
        ),
        ("hidden/child.rs", "pub fn value() {}", Language::Rust),
    ]);
    let report = a.analyze(&policy, 101).unwrap();
    let root = report
        .files
        .iter()
        .find(|f| f.coverage.path == "lib.rs")
        .unwrap();
    assert!(
        root.facts.imports.iter().all(|i| i.target_path.is_none()),
        "{:?}",
        root.facts.imports
    );
    assert_eq!(root.coverage.structure, Coverage::Partial);
}

#[test]
fn javascript_does_not_guess_a_loader_extension_policy() {
    let _guard = PARSER_TEST.lock().unwrap();
    let (a, policy) = fixture(&[
        (
            "main.mjs",
            "import './dep'; import './dep.js';",
            Language::JavaScript,
        ),
        ("dep.js", "export const value = 1;", Language::JavaScript),
    ]);
    let report = a.analyze(&policy, 101).unwrap();
    let file = report
        .files
        .iter()
        .find(|f| f.coverage.path == "main.mjs")
        .unwrap();
    assert_eq!(file.facts.imports.len(), 2);
    assert!(file.facts.imports[0].target_path.is_none());
    assert_eq!(file.facts.imports[1].target_path.as_deref(), Some("dep.js"));
}
