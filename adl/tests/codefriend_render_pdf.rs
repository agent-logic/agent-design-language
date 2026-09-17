//! PVF lane: deterministic local contract and installed-CLI proof for the
//! approved PDF renderer. No provider, network, browser, or remote publication.

use adl::codefriend::{
    actions::{
        remediation::{plan_from_file as remediation_from_file, RemediationOptions},
        test_plan::{
            plan_from_file as test_plan_from_file, read_plan_from_file as read_test_plan,
            TestPlanOptions,
        },
    },
    evidence::contracts::ReviewRecord,
    ingestion::digest,
    publication::{
        append_decision, render_pdf, DecisionKind, ManifestInput, PdfManifest, PdfRenderOptions,
        PDF_RENDERER_VERSION,
    },
    review::synthesis::{read_synthesis_from_file, synthesize_from_file, SynthesisOptions},
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

struct Fixture {
    _temp: tempfile::TempDir,
    review: ReviewRecord,
    review_path: PathBuf,
    artifact_root: PathBuf,
    publication_path: PathBuf,
    approval_store: PathBuf,
    destination_root: PathBuf,
    synthesis_rel: PathBuf,
    remediation_rel: PathBuf,
    test_plan_rel: PathBuf,
    out: PathBuf,
    font: PathBuf,
}

impl Fixture {
    fn new(review: ReviewRecord, claims: Vec<String>, renderer: &str) -> Self {
        review.validate().unwrap();
        let target_tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
        fs::create_dir_all(target_tmp).unwrap();
        let temp = tempfile::tempdir_in(target_tmp).unwrap();
        let root = temp.path().to_path_buf();
        let review_path = root.join("review-record.json");
        write_json(&review_path, &review);
        let artifact_root = root.join("artifacts");
        fs::create_dir(&artifact_root).unwrap();
        let synthesis = artifact_root.join("synthesis");
        synthesize_from_file(SynthesisOptions {
            input: review_path.clone(),
            out: synthesis.clone(),
        })
        .unwrap();
        let remediation = artifact_root.join("remediation");
        remediation_from_file(RemediationOptions {
            input: synthesis.join("synthesis.json"),
            out: remediation.clone(),
        })
        .unwrap();
        let tests = artifact_root.join("tests");
        test_plan_from_file(TestPlanOptions {
            input: synthesis.join("synthesis.json"),
            out: tests.clone(),
        })
        .unwrap();

        let destination_root = root.join("destination");
        fs::create_dir(&destination_root).unwrap();
        let input: ManifestInput = serde_json::from_value(json!({
            "schema":"codefriend.publication_manifest_input.v1",
            "artifact_manifest":artifact_inventory(&artifact_root),
            "renderer_versions":{"pdf":renderer},
            "target":"approved-pdf-report",
            "claims":claims,
            "nonclaims":["No HTML, Markdown, remote, or customer publication"]
        }))
        .unwrap();
        let publication = input.publication(&review, &destination_root).unwrap();
        let publication_path = root.join("publication.json");
        write_json(&publication_path, &publication);
        let approval_store = root.join("approval-store");
        append_decision(
            &approval_store,
            &review,
            &publication,
            DecisionKind::Approved,
            "operator-fixture",
            "Exact semantic inputs approved for deterministic PDF rendering",
            1_700_000_000,
        )
        .unwrap();
        Self {
            _temp: temp,
            review,
            review_path,
            artifact_root,
            publication_path,
            approval_store,
            destination_root: destination_root.clone(),
            synthesis_rel: "synthesis/synthesis.json".into(),
            remediation_rel: "remediation/remediation-plan.json".into(),
            test_plan_rel: "tests/test-plan.json".into(),
            out: destination_root.join("approved-pdf-report"),
            font: qualification_font(),
        }
    }

    fn options(&self) -> PdfRenderOptions {
        PdfRenderOptions {
            review_record: self.review_path.clone(),
            publication: self.publication_path.clone(),
            approval_store: self.approval_store.clone(),
            artifact_root: self.artifact_root.clone(),
            synthesis: self.synthesis_rel.clone(),
            remediation_plan: self.remediation_rel.clone(),
            test_plan: self.test_plan_rel.clone(),
            destination_root: self.destination_root.clone(),
            out: self.out.clone(),
            font: self.font.clone(),
        }
    }
}

fn predecessor_review() -> ReviewRecord {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.csdlc/evidence/892/predecessor-openai-r5-synthesis/review-record.json");
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn qualification_font() -> PathBuf {
    [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
        "/Library/Fonts/Arial Unicode.ttf",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|path| path.is_file())
    .expect("PDF qualification requires an installed Unicode TrueType font")
}

fn artifact_inventory(root: &Path) -> Vec<serde_json::Value> {
    fn visit(current: &Path, files: &mut Vec<PathBuf>) {
        let mut entries = fs::read_dir(current)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry.path().is_dir() {
                visit(&entry.path(), files);
            } else {
                files.push(entry.path());
            }
        }
    }
    let mut files = Vec::new();
    visit(root, &mut files);
    let mut artifacts = files
        .into_iter()
        .map(|path| {
            json!({
                "path":path.strip_prefix(root).unwrap().to_string_lossy(),
                "digest":digest(&fs::read(path).unwrap())
            })
        })
        .collect::<Vec<_>>();
    artifacts.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    artifacts
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn cli(fixture: &Fixture) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "export", "pdf", "--review-record"])
        .arg(&fixture.review_path)
        .arg("--publication")
        .arg(&fixture.publication_path)
        .arg("--approval-store")
        .arg(&fixture.approval_store)
        .arg("--artifact-root")
        .arg(&fixture.artifact_root)
        .arg("--synthesis")
        .arg(&fixture.synthesis_rel)
        .arg("--remediation-plan")
        .arg(&fixture.remediation_rel)
        .arg("--test-plan")
        .arg(&fixture.test_plan_rel)
        .arg("--destination-root")
        .arg(&fixture.destination_root)
        .arg("--out")
        .arg(&fixture.out)
        .arg("--font")
        .arg(&fixture.font)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap()
}

#[test]
fn installed_renderer_emits_extractable_multipage_pdf_and_bound_manifest() {
    let mut review = predecessor_review();
    review.findings[0].rationale = format!(
        "Résumé café π with a long URL https://example.invalid/{} and table row | cell | value. {}",
        "abcdefghijklmnopqrstuvwxyz0123456789".repeat(4),
        "bounded evidence ".repeat(180)
    );
    review.validate().unwrap();
    let fixture = Fixture::new(
        review,
        vec!["Approved exact PDF review semantics".to_string()],
        PDF_RENDERER_VERSION,
    );
    let output = cli(&fixture);
    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["schema"], "codefriend.pdf_render_result.v1");
    assert_eq!(result["finding_count"], 1);
    let report = fixture.out.join("report.pdf");
    let manifest: PdfManifest =
        serde_json::from_slice(&fs::read(fixture.out.join("manifest.json")).unwrap()).unwrap();
    assert!(manifest.page_count >= 2);
    assert_eq!(manifest.report_digest, digest(&fs::read(&report).unwrap()));
    assert_eq!(manifest.claims, ["Approved exact PDF review semantics"]);
    assert!(!manifest.external_resources);

    let extracted = Command::new("pdftotext")
        .arg("-layout")
        .arg(&report)
        .arg("-")
        .output()
        .expect("pdftotext is required for PDF semantic qualification");
    assert!(
        extracted.status.success(),
        "{}",
        String::from_utf8_lossy(&extracted.stderr)
    );
    let text = String::from_utf8(extracted.stdout).unwrap();
    for semantic in [
        "CodeFriend Review Report",
        "Approved exact PDF review semantics",
        "410da89a0ed42c523143da89fffeb7f6402833e0",
        "lib/dnsmsg-parser/src/dns_message.rs",
        "Remediation plan",
        "Test plan",
        "Résumé café π",
        "table row",
    ] {
        assert!(
            text.contains(semantic),
            "missing extracted semantic: {semantic}"
        );
    }
    let normalized = normalized_text(&text);
    let synthesis =
        read_synthesis_from_file(&fixture.artifact_root.join(&fixture.synthesis_rel)).unwrap();
    let remediation: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.artifact_root.join(&fixture.remediation_rel)).unwrap(),
    )
    .unwrap();
    let test_plan = read_test_plan(&fixture.artifact_root.join(&fixture.test_plan_rel)).unwrap();
    for finding in &synthesis.synthesized_findings {
        for value in [
            finding.id.as_str(),
            finding.title.as_str(),
            finding.semantic_anchor.as_str(),
            finding.severity_rationale.as_str(),
        ] {
            assert_semantic(&normalized, value);
        }
        if let Some(disagreement) = &finding.disagreement {
            assert_semantic(&normalized, disagreement);
        }
        for value in &finding.scope_limits {
            assert_semantic(&normalized, value);
        }
        for source in &finding.sources {
            for value in [
                source.perspective.as_str(),
                source.rule.as_str(),
                source.rationale.as_str(),
                source.finding_id.as_str(),
                source.inference.as_str(),
            ] {
                assert_semantic(&normalized, value);
            }
            for value in source.evidence.iter().chain(source.limitations.iter()) {
                assert_semantic(&normalized, value);
            }
        }
    }
    assert_json_string_leaves(&normalized, &remediation["actions"]);
    assert_json_string_leaves(&normalized, &remediation["omitted_findings"]);
    for case in &test_plan.test_cases {
        let value = serde_json::to_value(case).unwrap();
        assert_json_string_leaves(&normalized, &value);
    }
    for omission in &test_plan.omitted_findings {
        let value = serde_json::to_value(omission).unwrap();
        assert_json_string_leaves(&normalized, &value);
    }
    if let Some(retain) = std::env::var_os("ADL_PDF_RETAIN_DIR") {
        let retain = PathBuf::from(retain);
        fs::create_dir(&retain).expect("retained PDF proof directory must be fresh");
        fs::copy(&report, retain.join("report.pdf")).unwrap();
        fs::copy(
            fixture.out.join("manifest.json"),
            retain.join("manifest.json"),
        )
        .unwrap();
        fs::write(retain.join("extracted.txt"), text.as_bytes()).unwrap();
    }
    assert!(!cli(&fixture).status.success(), "fresh target is mandatory");
}

fn normalized_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace() && *character != '\\')
        .collect()
}

fn assert_semantic(normalized_report: &str, value: &str) {
    let expected = normalized_text(value);
    assert!(
        expected.is_empty() || normalized_report.contains(&expected),
        "missing PDF semantic: {value}"
    );
}

fn assert_json_string_leaves(normalized_report: &str, value: &serde_json::Value) {
    match value {
        serde_json::Value::String(value) => assert_semantic(normalized_report, value),
        serde_json::Value::Array(values) => {
            for value in values {
                assert_json_string_leaves(normalized_report, value);
            }
        }
        serde_json::Value::Object(values) => {
            for (key, value) in values {
                if !matches!(
                    key.as_str(),
                    "schema"
                        | "synthesis_digest"
                        | "run_id"
                        | "repository"
                        | "revision"
                        | "scope_digest"
                ) {
                    assert_json_string_leaves(normalized_report, value);
                }
            }
        }
        _ => {}
    }
}

#[test]
fn renderer_refuses_withheld_approval_renderer_drift_and_tampering() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["Approved PDF review".to_string()],
        PDF_RENDERER_VERSION,
    );
    let publication: adl::codefriend::evidence::contracts::Publication =
        serde_json::from_slice(&fs::read(&fixture.publication_path).unwrap()).unwrap();
    append_decision(
        &fixture.approval_store,
        &fixture.review,
        &publication,
        DecisionKind::Withheld,
        "operator-fixture",
        "Current decision withholds rendering",
        1_700_000_001,
    )
    .unwrap();
    let error = render_pdf(fixture.options()).unwrap_err().to_string();
    assert!(
        error.contains("markdown_requires_current_approval"),
        "{error}"
    );

    let wrong = Fixture::new(
        predecessor_review(),
        vec!["Approved PDF review".to_string()],
        "v999",
    );
    let error = render_pdf(wrong.options()).unwrap_err().to_string();
    assert!(
        error.contains("markdown_renderer_identity_mismatch"),
        "{error}"
    );

    let tampered = Fixture::new(
        predecessor_review(),
        vec!["Approved PDF review".to_string()],
        PDF_RENDERER_VERSION,
    );
    fs::write(
        tampered.artifact_root.join(&tampered.synthesis_rel),
        b"{}\n",
    )
    .unwrap();
    let error = render_pdf(tampered.options()).unwrap_err().to_string();
    assert!(error.contains("artifact_digest_mismatch"), "{error}");
    assert!(
        !tampered.out.exists(),
        "failure must not leave success output"
    );
}

#[test]
fn renderer_rejects_invalid_font() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["Approved PDF review".to_string()],
        PDF_RENDERER_VERSION,
    );
    fs::write(fixture._temp.path().join("not-a-font.ttf"), b"not a font").unwrap();
    let mut options = fixture.options();
    options.font = fixture._temp.path().join("not-a-font.ttf");
    let error = render_pdf(options).unwrap_err().to_string();
    assert!(error.contains("pdf_font_parse_failed"), "{error}");
    assert!(!fixture.out.exists());
}

#[test]
fn renderer_rejects_font_without_complete_glyph_coverage() {
    let mut review = predecessor_review();
    review.findings[0].rationale = "Unsupported glyph must fail: 🧪".to_string();
    review.validate().unwrap();
    let fixture = Fixture::new(
        review,
        vec!["Approved PDF review".to_string()],
        PDF_RENDERER_VERSION,
    );
    let error = render_pdf(fixture.options()).unwrap_err().to_string();
    assert!(error.contains("pdf_font_missing_glyph"), "{error}");
    assert!(!fixture.out.exists());
}
