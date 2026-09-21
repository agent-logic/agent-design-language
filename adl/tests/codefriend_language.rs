//! PVF component: deterministic admitted-source contract regressions, local Git/CPU only.
//! No parser or installed-product acceptance is claimed by these validation fixtures.
use adl::codefriend::{
    evidence::{Admission, Retention},
    ingestion::{local, Scope},
    language::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
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
fn fixture() -> (Admission, AnalysisPolicy, AnalysisReport) {
    let t = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    git(t.path(), &["init", "-b", "main"]);
    git(
        t.path(),
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    let inputs = [
        ("Main.java", "class Main {}", Language::Java),
        ("lib.rs", "fn main() {}", Language::Rust),
        ("main.js", "const café = 1;", Language::JavaScript),
        ("main.py", "value = 1", Language::Python),
    ];
    for (p, s, _) in inputs {
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
            analysis: inputs.iter().map(|(p, _, _)| p.to_string()).collect(),
            context: vec!["LICENSE".into()],
            max_files: 5,
            max_bytes: 4096,
            max_file_bytes: 1024,
        },
    )
    .unwrap();
    let a = Admission::new(packet, Retention { seconds: 100 }, 100).unwrap();
    let p = AnalysisPolicy {
        schema: VERSION.into(),
        files: inputs.iter().map(|(p, _, l)| (p.to_string(), *l)).collect(),
        roots: inputs
            .iter()
            .map(|(_, _, l)| ProjectRoot {
                language: *l,
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
    let mut r = AnalysisReport {
        schema: VERSION.into(),
        admission_digest: a.digest.clone(),
        policy_digest: p.digest().unwrap(),
        toolchain: inputs
            .iter()
            .map(|(_, _, l)| {
                (
                    *l,
                    ToolIdentity {
                        name: "fixture_contract_only".into(),
                        version: "1".into(),
                        grammar_revision: "not_a_parser_execution".into(),
                        query_digest: "a".repeat(64),
                    },
                )
            })
            .collect(),
        files: a
            .evidence
            .iter()
            .filter(|e| p.files.contains_key(&e.path))
            .map(|e| FileAnalysis {
                coverage: FileCoverage {
                    path: e.path.clone(),
                    evidence_id: Some(e.id.clone()),
                    content_digest: Some(e.content_digest.clone()),
                    language: p.files[&e.path],
                    syntax: Coverage::Complete,
                    structure: Coverage::Partial,
                    semantics: Coverage::Unsupported,
                    diagnostics: vec![Diagnostic {
                        code: "fixture_no_semantic_analysis".into(),
                        span: None,
                    }],
                },
                facts: SourceFacts::default(),
            })
            .collect(),
        digest: String::new(),
    };
    r.files
        .sort_by(|a, b| a.coverage.path.cmp(&b.coverage.path));
    r.digest = r.expected_digest().unwrap();
    (a, p, r)
}
fn seal(r: &mut AnalysisReport) {
    r.digest = r.expected_digest().unwrap();
}
#[test]
fn four_languages_preserve_original_packet_validation() {
    let (a, p, r) = fixture();
    let before = serde_json::to_vec(&a).unwrap();
    r.validate(&a, &p, 101).unwrap();
    a.validate().unwrap();
    assert_eq!(before, serde_json::to_vec(&a).unwrap());
    assert!(a
        .packet
        .objects
        .iter()
        .any(|o| o.path == "main.py" && o.analysis_support.contains("unsupported")));
}
#[test]
fn resealed_wrong_source_and_expired_reports_fail() {
    let (a, p, mut r) = fixture();
    let original = r.clone();
    r.files[0].coverage.content_digest = Some("b".repeat(64));
    seal(&mut r);
    assert!(r.validate(&a, &p, 101).is_err());
    assert!(original
        .validate(&a, &p, 200)
        .unwrap_err()
        .to_string()
        .contains("not_live"));
}
#[test]
fn source_spans_and_reference_targets_are_checked() {
    let (a, p, mut r) = fixture();
    let f = r
        .files
        .iter_mut()
        .find(|f| f.coverage.path == "main.js")
        .unwrap();
    f.facts.references.push(Reference {
        spelling: "café".into(),
        span: Span {
            start_byte: 6,
            end_byte: 10,
        },
        target_declaration: None,
    });
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("span"));
    r.files
        .iter_mut()
        .find(|f| f.coverage.path == "main.js")
        .unwrap()
        .facts
        .references[0]
        .span
        .end_byte = 11;
    seal(&mut r);
    r.validate(&a, &p, 101).unwrap();
    r.files
        .iter_mut()
        .find(|f| f.coverage.path == "main.js")
        .unwrap()
        .facts
        .references[0]
        .target_declaration = Some("a".repeat(64));
    seal(&mut r);
    assert!(r.validate(&a, &p, 101).is_err());
}
#[test]
fn coverage_and_scope_cannot_be_silently_dropped() {
    let (a, p, mut r) = fixture();
    r.files[0].coverage.syntax = Coverage::Partial;
    r.files[0].coverage.structure = Coverage::Complete;
    seal(&mut r);
    assert!(r.validate(&a, &p, 101).is_err());
    r.files.remove(0);
    seal(&mut r);
    assert!(r.validate(&a, &p, 101).is_err());
    let mut p = p;
    p.files.remove("main.py");
    assert!(p.validate(&a).is_err());
}
#[test]
fn policy_and_aggregate_output_bounds_are_enforced() {
    let (a, mut p, mut r) = fixture();
    p.limits.max_output_bytes = 1;
    r.policy_digest = p.digest().unwrap();
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("output_limit"));
    p.roots[0].root = "../escape".into();
    assert!(p.validate(&a).is_err());
}

#[test]
fn fully_resealed_text_and_cross_language_claims_fail() {
    let (a, p, mut r) = fixture();
    let js = r
        .files
        .iter()
        .position(|f| f.coverage.path == "main.js")
        .unwrap();
    r.files[js].facts.references.push(Reference {
        spelling: "wrong".into(),
        span: Span {
            start_byte: 6,
            end_byte: 11,
        },
        target_declaration: None,
    });
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("text_mismatch"));
    r.files[js].facts.references[0].spelling = "café".into();
    let java = r
        .files
        .iter()
        .position(|f| f.coverage.path == "Main.java")
        .unwrap();
    let span = Span {
        start_byte: 6,
        end_byte: 10,
    };
    let id = declaration_id(
        r.files[java].coverage.evidence_id.as_ref().unwrap(),
        Language::Java,
        span,
        "Main",
        "class",
    )
    .unwrap();
    r.files[java].facts.declarations.push(Declaration {
        id: id.clone(),
        name: "Main".into(),
        kind: "class".into(),
        span,
    });
    r.files[js].facts.references[0].target_declaration = Some(id);
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("reference_target"));
    r.files[js].facts.references[0].target_declaration = None;
    r.files[js].coverage.diagnostics.push(Diagnostic {
        code: "parse_error".into(),
        span: Some(Span {
            start_byte: 0,
            end_byte: 1,
        }),
    });
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("syntax_error_overclaim"));
}

#[test]
fn fully_resealed_cross_language_import_target_is_rejected() {
    let (a, p, mut r) = fixture();
    let js = r
        .files
        .iter()
        .position(|f| f.coverage.path == "main.js")
        .unwrap();
    r.files[js].facts.imports.push(Import {
        spelling: "café".into(),
        kind: "fixture_import".into(),
        span: Span {
            start_byte: 6,
            end_byte: 11,
        },
        target_path: Some("Main.java".into()),
    });
    seal(&mut r);
    assert!(r
        .validate(&a, &p, 101)
        .unwrap_err()
        .to_string()
        .contains("import_target_unavailable_or_cross_language"));
    r.files[js].facts.imports[0].target_path = Some("main.js".into());
    seal(&mut r);
    r.validate(&a, &p, 101).unwrap();
}
