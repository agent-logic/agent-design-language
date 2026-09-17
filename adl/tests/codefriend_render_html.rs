//! PVF lane: deterministic local contract and installed-CLI proof for the
//! approved HTML renderer. Browser behavior is restricted to script-free,
//! same-document navigation and is separately observed in retained evidence.

use adl::codefriend::{
    actions::{
        remediation::{plan_from_file as remediation_from_file, RemediationOptions},
        test_plan::{plan_from_file as test_plan_from_file, TestPlanOptions},
    },
    evidence::{
        contracts::{Confidence, ReviewRecord, Severity},
        hash,
    },
    ingestion::digest,
    publication::{
        append_decision, render_html, render_markdown, DecisionKind, HtmlManifest,
        HtmlRenderOptions, ManifestInput, MarkdownRenderOptions, HTML_RENDERER_VERSION,
        MARKDOWN_RENDERER_VERSION,
    },
    review::synthesis::{synthesize_from_file, SynthesisOptions},
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

struct Fixture {
    _temp: tempfile::TempDir,
    review_path: PathBuf,
    artifact_root: PathBuf,
    publication_path: PathBuf,
    approval_store: PathBuf,
    destination_root: PathBuf,
    synthesis_rel: PathBuf,
    remediation_rel: PathBuf,
    test_plan_rel: PathBuf,
    out: PathBuf,
}

impl Fixture {
    fn new(review: ReviewRecord, renderer: &str) -> Self {
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
            out: tests,
        })
        .unwrap();
        let destination_root = root.join("destination");
        fs::create_dir(&destination_root).unwrap();
        let input: ManifestInput = serde_json::from_value(json!({
            "schema":"codefriend.publication_manifest_input.v1",
            "artifact_manifest":artifact_inventory(&artifact_root),
            "renderer_versions":{"html":renderer,"markdown":MARKDOWN_RENDERER_VERSION},
            "target":"approved-report",
            "claims":["Approved exact review semantics"],
            "nonclaims":["No PDF, remote, or customer publication"]
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
            "Exact semantic inputs approved for deterministic HTML rendering",
            1_700_000_000,
        )
        .unwrap();
        let out = destination_root.join("approved-report");
        Self {
            _temp: temp,
            review_path,
            artifact_root,
            publication_path,
            approval_store,
            destination_root,
            synthesis_rel: "synthesis/synthesis.json".into(),
            remediation_rel: "remediation/remediation-plan.json".into(),
            test_plan_rel: "tests/test-plan.json".into(),
            out,
        }
    }

    fn options(&self) -> HtmlRenderOptions {
        HtmlRenderOptions {
            review_record: self.review_path.clone(),
            publication: self.publication_path.clone(),
            approval_store: self.approval_store.clone(),
            artifact_root: self.artifact_root.clone(),
            synthesis: self.synthesis_rel.clone(),
            remediation_plan: self.remediation_rel.clone(),
            test_plan: self.test_plan_rel.clone(),
            destination_root: self.destination_root.clone(),
            out: self.out.clone(),
        }
    }

    fn cli(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "export", "html", "--review-record"])
            .arg(&self.review_path)
            .arg("--publication")
            .arg(&self.publication_path)
            .arg("--approval-store")
            .arg(&self.approval_store)
            .arg("--artifact-root")
            .arg(&self.artifact_root)
            .arg("--synthesis")
            .arg(&self.synthesis_rel)
            .arg("--remediation-plan")
            .arg(&self.remediation_rel)
            .arg("--test-plan")
            .arg(&self.test_plan_rel)
            .arg("--destination-root")
            .arg(&self.destination_root)
            .arg("--out")
            .arg(&self.out)
            .env("ADL_OBSERVABILITY_OTEL", "0")
            .output()
            .unwrap()
    }
}

fn recompute_finding_id(finding: &mut adl::codefriend::evidence::contracts::Finding) {
    finding.id = hash(&(
        "codefriend.finding_identity.v1",
        &finding.repository,
        &finding.perspective,
        &finding.rule,
        &finding.semantic_anchor,
    ))
    .unwrap();
}

fn parity_review() -> ReviewRecord {
    let mut review = predecessor_review();

    let mut corroborating = review.findings[0].clone();
    corroborating.perspective = "security".to_string();
    corroborating.rule = "security.raw_message_reuse_boundary".to_string();
    corroborating.severity = Severity::Medium;
    corroborating.rationale =
        "Security lane retains a distinct bounded rationale token".to_string();
    corroborating.confidence = Confidence::Known(61);
    corroborating.inference = "Security lane retains a distinct inference token".to_string();
    corroborating.limitations = vec!["Security lane uncertainty token".to_string()];
    recompute_finding_id(&mut corroborating);
    review.findings.push(corroborating);

    let mut omitted = review.findings[0].clone();
    omitted.perspective = "constitutional".to_string();
    omitted.rule = "constitutional.no_repository_path".to_string();
    omitted.semantic_anchor = "Behavioral contract without a repository location".to_string();
    omitted.title = "Omitted action test parity token".to_string();
    omitted.severity = Severity::Info;
    omitted.rationale = "Omission rationale parity token".to_string();
    omitted.confidence = Confidence::Unknown;
    omitted.inference = "Omission inference parity token".to_string();
    omitted.limitations = vec!["Omission uncertainty parity token".to_string()];
    recompute_finding_id(&mut omitted);
    review.findings.push(omitted);
    review
        .findings
        .sort_by(|left, right| left.id.cmp(&right.id));
    review.validate().unwrap();
    review
}

fn normalized_semantics(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn assert_json_strings_present(value: &serde_json::Value, html: &str, markdown: &str) {
    match value {
        serde_json::Value::String(expected) if !expected.is_empty() => {
            let expected = normalized_semantics(expected);
            assert!(
                normalized_semantics(html).contains(&expected),
                "HTML omitted semantic value: {expected}"
            );
            assert!(
                normalized_semantics(markdown).contains(&expected),
                "Markdown omitted semantic value: {expected}"
            );
        }
        serde_json::Value::Array(values) => {
            for value in values {
                assert_json_strings_present(value, html, markdown);
            }
        }
        serde_json::Value::Object(values) => {
            for value in values.values() {
                assert_json_strings_present(value, html, markdown);
            }
        }
        _ => {}
    }
}

fn html_finding_section<'a>(report: &'a str, finding_id: &str) -> &'a str {
    let marker = format!("<dt>Finding ID</dt><dd>{finding_id}</dd>");
    let marker_start = report.find(&marker).unwrap();
    let start = report[..marker_start]
        .rfind("<article class=\"finding\"")
        .unwrap();
    let end = marker_start + report[marker_start..].find("</article>").unwrap();
    &report[start..end]
}

fn markdown_finding_section<'a>(report: &'a str, finding_id: &str) -> &'a str {
    let marker = format!("**Finding ID:** {finding_id}");
    let marker_start = report.find(&marker).unwrap();
    let start = report[..marker_start].rfind("\n### ").unwrap();
    let end = marker_start
        + report[marker_start..]
            .find("\n### ")
            .or_else(|| report[marker_start..].find("\n## Output boundary"))
            .unwrap();
    &report[start..end]
}

fn matching_by_finding_id<'a>(
    values: &'a [serde_json::Value],
    finding_id: &str,
) -> Vec<&'a serde_json::Value> {
    values
        .iter()
        .filter(|value| value["finding_id"].as_str() == Some(finding_id))
        .collect()
}

fn predecessor_review() -> ReviewRecord {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.csdlc/evidence/892/predecessor-openai-r5-synthesis/review-record.json");
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn write_json(path: &Path, value: &impl serde::Serialize) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
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

#[test]
fn installed_renderer_emits_navigable_bound_report_and_manifest() {
    let fixture = Fixture::new(predecessor_review(), HTML_RENDERER_VERSION);
    let output = fixture.cli();
    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["schema"], "codefriend.html_render_result.v1");
    assert_eq!(result["finding_count"], 1);
    let report = fs::read_to_string(fixture.out.join("report.html")).unwrap();
    let manifest: HtmlManifest =
        serde_json::from_slice(&fs::read(fixture.out.join("manifest.json")).unwrap()).unwrap();
    for required in [
        "<!doctype html>",
        "aria-label=\"Report contents\"",
        "href=\"#source\"",
        "href=\"#findings\"",
        "href=\"#evidence\"",
        "Remediation plan",
        "Test plan",
        "Scope limits and uncertainty",
        "Attributed source assessments",
        "410da89a0ed42c523143da89fffeb7f6402833e0",
        "lib/dnsmsg-parser/src/dns_message.rs",
    ] {
        assert!(report.contains(required), "missing {required}");
    }
    assert!(!report.contains("<script"));
    assert!(!report.contains("href=\"http://"));
    assert!(!report.contains("href=\"https://"));
    assert_eq!(manifest.finding_ids.len(), 1);
    assert_eq!(manifest.claims, ["Approved exact review semantics"]);
    assert_eq!(manifest.report_digest, digest(report.as_bytes()));
    if let Some(retain) = std::env::var_os("CODEFRIEND_HTML_RETAIN_DIR") {
        let retain = PathBuf::from(retain);
        fs::create_dir_all(&retain).unwrap();
        fs::copy(fixture.out.join("report.html"), retain.join("report.html")).unwrap();
        fs::copy(
            fixture.out.join("manifest.json"),
            retain.join("manifest.json"),
        )
        .unwrap();
        fs::write(retain.join("render-result.json"), &output.stdout).unwrap();
    }
    assert!(!fixture.cli().status.success(), "fresh target is mandatory");
}

#[test]
fn renderer_escapes_hostile_markup_and_preserves_long_content() {
    let mut review = predecessor_review();
    review.findings[0].title = "<img src=x onerror=alert(1)> & \"quoted\"".to_string();
    review.findings[0].rationale = format!(
        "{}<script>alert(1)</script>",
        "bounded evidence ".repeat(300)
    );
    review.validate().unwrap();
    let fixture = Fixture::new(review, HTML_RENDERER_VERSION);
    render_html(fixture.options()).unwrap();
    let report = fs::read_to_string(fixture.out.join("report.html")).unwrap();
    assert!(report.contains("&lt;img src=x onerror=alert(1)&gt;"));
    assert!(report.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!report.contains("<img"));
    assert!(!report.contains("<script"));
    assert!(report.len() > 5_000);
}

#[test]
fn renderer_refuses_withheld_approval_wrong_identity_and_tampering() {
    let fixture = Fixture::new(predecessor_review(), HTML_RENDERER_VERSION);
    let publication: adl::codefriend::evidence::contracts::Publication =
        serde_json::from_slice(&fs::read(&fixture.publication_path).unwrap()).unwrap();
    append_decision(
        &fixture.approval_store,
        &predecessor_review(),
        &publication,
        DecisionKind::Withheld,
        "operator-fixture",
        "Current decision withholds rendering",
        1_700_000_001,
    )
    .unwrap();
    assert!(render_html(fixture.options())
        .unwrap_err()
        .to_string()
        .contains("html_requires_current_approval"));

    let wrong = Fixture::new(predecessor_review(), "v2");
    assert!(render_html(wrong.options())
        .unwrap_err()
        .to_string()
        .contains("html_renderer_identity_mismatch"));

    let tampered = Fixture::new(predecessor_review(), HTML_RENDERER_VERSION);
    fs::write(
        tampered.artifact_root.join(&tampered.synthesis_rel),
        b"{}\n",
    )
    .unwrap();
    assert!(render_html(tampered.options())
        .unwrap_err()
        .to_string()
        .contains("artifact_digest_mismatch"));
}

#[test]
fn empty_findings_have_explicit_complete_output() {
    let mut review = predecessor_review();
    review.findings.clear();
    review.validate().unwrap();
    let fixture = Fixture::new(review, HTML_RENDERER_VERSION);
    render_html(fixture.options()).unwrap();
    let report = fs::read_to_string(fixture.out.join("report.html")).unwrap();
    assert!(report.contains("No findings were reported"));
    let manifest: HtmlManifest =
        serde_json::from_slice(&fs::read(fixture.out.join("manifest.json")).unwrap()).unwrap();
    assert!(manifest.finding_ids.is_empty());
}

#[test]
fn html_and_markdown_preserve_complete_governed_semantics() {
    let fixture = Fixture::new(parity_review(), HTML_RENDERER_VERSION);
    render_html(fixture.options()).unwrap();
    let html = fs::read_to_string(fixture.out.join("report.html")).unwrap();

    fs::remove_dir_all(&fixture.out).unwrap();
    render_markdown(MarkdownRenderOptions {
        review_record: fixture.review_path.clone(),
        publication: fixture.publication_path.clone(),
        approval_store: fixture.approval_store.clone(),
        artifact_root: fixture.artifact_root.clone(),
        synthesis: fixture.synthesis_rel.clone(),
        remediation_plan: fixture.remediation_rel.clone(),
        test_plan: fixture.test_plan_rel.clone(),
        destination_root: fixture.destination_root.clone(),
        out: fixture.out.clone(),
    })
    .unwrap();
    let markdown = fs::read_to_string(fixture.out.join("report.md")).unwrap();

    let synthesis: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.artifact_root.join(&fixture.synthesis_rel)).unwrap(),
    )
    .unwrap();
    let remediation: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.artifact_root.join(&fixture.remediation_rel)).unwrap(),
    )
    .unwrap();
    let test_plan: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.artifact_root.join(&fixture.test_plan_rel)).unwrap(),
    )
    .unwrap();

    let findings = synthesis["synthesized_findings"].as_array().unwrap();
    let actions = remediation["actions"].as_array().unwrap();
    let remediation_omissions = remediation["omitted_findings"].as_array().unwrap();
    let tests = test_plan["test_cases"].as_array().unwrap();
    let test_omissions = test_plan["omitted_findings"].as_array().unwrap();
    assert_eq!(
        html.matches("<article class=\"finding\"").count(),
        findings.len()
    );
    let markdown_findings = markdown
        .split_once("## Findings")
        .unwrap()
        .1
        .split_once("## Output boundary")
        .unwrap()
        .0;
    assert_eq!(markdown_findings.matches("\n### ").count(), findings.len());

    for finding in findings {
        let finding_id = finding["id"].as_str().unwrap();
        let html_section = html_finding_section(&html, finding_id);
        let markdown_section = markdown_finding_section(&markdown, finding_id);
        assert_json_strings_present(finding, html_section, markdown_section);
        assert_eq!(
            html_section.matches(" Finding: ").count(),
            finding["sources"].as_array().unwrap().len()
        );
        assert_eq!(
            markdown_section.matches(" Finding: ").count(),
            finding["sources"].as_array().unwrap().len()
        );
        for source in finding["sources"].as_array().unwrap() {
            let confidence = match source["confidence"]["state"].as_str().unwrap() {
                "known" => format!(
                    "Known({})",
                    source["confidence"]["percent"].as_u64().unwrap()
                ),
                "unknown" => "Unknown".to_string(),
                state => panic!("unexpected confidence state {state}"),
            };
            let confidence = normalized_semantics(&confidence);
            assert!(normalized_semantics(html_section).contains(&confidence));
            assert!(normalized_semantics(markdown_section).contains(&confidence));
        }

        let finding_actions = matching_by_finding_id(actions, finding_id);
        let finding_remediation_omissions =
            matching_by_finding_id(remediation_omissions, finding_id);
        let finding_tests = matching_by_finding_id(tests, finding_id);
        let finding_test_omissions = matching_by_finding_id(test_omissions, finding_id);
        assert_eq!(
            finding_actions.len() + finding_remediation_omissions.len(),
            1
        );
        assert_eq!(finding_tests.len() + finding_test_omissions.len(), 1);
        for associated in finding_actions
            .into_iter()
            .chain(finding_remediation_omissions)
            .chain(finding_tests)
            .chain(finding_test_omissions)
        {
            assert_json_strings_present(associated, html_section, markdown_section);
        }
    }
    assert_eq!(test_plan["omitted_findings"].as_array().unwrap().len(), 1);
    assert!(html.contains("severity disagreement retained"));
    assert!(markdown.contains("severity disagreement retained"));
    let omission_reason = normalized_semantics("no_supported_repository_path_for_test_location");
    assert!(normalized_semantics(&html).contains(&omission_reason));
    assert!(normalized_semantics(&markdown).contains(&omission_reason));
}
