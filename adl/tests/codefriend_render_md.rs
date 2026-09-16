//! PVF lane: deterministic local contract and installed-CLI proof for the
//! approved Markdown renderer. No provider, network, browser, or publication.

use adl::codefriend::{
    actions::{
        remediation::{plan_from_file as remediation_from_file, RemediationOptions},
        test_plan::{plan_from_file as test_plan_from_file, TestPlanOptions},
    },
    evidence::{contracts::ReviewRecord, hash},
    ingestion::digest,
    publication::{
        append_decision, render_markdown, DecisionKind, ManifestInput, MarkdownManifest,
        MarkdownRenderOptions, MARKDOWN_RENDERER_VERSION,
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
        let test_plan = artifact_root.join("tests");
        test_plan_from_file(TestPlanOptions {
            input: synthesis.join("synthesis.json"),
            out: test_plan.clone(),
        })
        .unwrap();

        let artifacts = artifact_inventory(&artifact_root);
        let destination_root = root.join("destination");
        fs::create_dir(&destination_root).unwrap();
        let input: ManifestInput = serde_json::from_value(json!({
            "schema":"codefriend.publication_manifest_input.v1",
            "artifact_manifest": artifacts,
            "renderer_versions":{"markdown":renderer},
            "target":"approved-report",
            "claims":claims,
            "nonclaims":["No HTML, PDF, remote, or customer publication"]
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
            "Exact semantic inputs approved for deterministic Markdown rendering",
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

    fn options(&self) -> MarkdownRenderOptions {
        MarkdownRenderOptions {
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
}

fn predecessor_review() -> ReviewRecord {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.csdlc/evidence/892/predecessor-openai-r5-synthesis/review-record.json");
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
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
        .args(["codefriend", "export", "markdown", "--review-record"])
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
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap()
}

#[test]
fn installed_renderer_emits_complete_bound_report_and_manifest() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["Approved exact review semantics".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    let output = cli(&fixture);
    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["schema"], "codefriend.markdown_render_result.v1");
    assert_eq!(result["finding_count"], 1);
    let report = fs::read_to_string(fixture.out.join("report.md")).unwrap();
    let manifest: MarkdownManifest =
        serde_json::from_slice(&fs::read(fixture.out.join("manifest.json")).unwrap()).unwrap();
    assert!(report.contains("# CodeFriend Review Report"));
    assert!(report.contains("410da89a0ed42c523143da89fffeb7f6402833e0"));
    assert!(report.contains("lib/dnsmsg-parser/src/dns_message.rs"));
    assert!(report.contains("**Remediation plan**"));
    assert!(report.contains("**Test plan**"));
    assert_eq!(manifest.finding_ids.len(), 1);
    assert_eq!(manifest.claims, ["Approved exact review semantics"]);
    assert_eq!(
        manifest.report_digest,
        digest(&fs::read(fixture.out.join("report.md")).unwrap())
    );
    assert!(!cli(&fixture).status.success(), "fresh target is mandatory");
}

#[test]
fn renderer_refuses_missing_or_withheld_approval_and_identity_drift() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["Approved review".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
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
    let error = render_markdown(fixture.options()).unwrap_err().to_string();
    assert!(
        error.contains("markdown_requires_current_approval"),
        "{error}"
    );

    let wrong_renderer = Fixture::new(
        predecessor_review(),
        vec!["Approved review".to_string()],
        "v2",
    );
    let error = render_markdown(wrong_renderer.options())
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("markdown_renderer_identity_mismatch"),
        "{error}"
    );

    let stale_target = Fixture::new(
        predecessor_review(),
        vec!["Approved review".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    let mut options = stale_target.options();
    options.out = stale_target.destination_root.join("different-target");
    let error = render_markdown(options).unwrap_err().to_string();
    assert!(
        error.contains("markdown_target_identity_mismatch"),
        "{error}"
    );
}

#[test]
fn renderer_refuses_changed_approved_artifacts_and_missing_provenance() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["Approved review".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    fs::write(fixture.artifact_root.join(&fixture.synthesis_rel), b"{}\n").unwrap();
    let error = render_markdown(fixture.options()).unwrap_err().to_string();
    assert!(error.contains("artifact_digest_mismatch"), "{error}");

    let missing = Fixture::new(
        predecessor_review(),
        vec!["Approved review".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    let mut options = missing.options();
    options.test_plan = "tests/not-approved.json".into();
    let error = render_markdown(options).unwrap_err().to_string();
    assert!(
        error.contains("markdown_input_not_in_approved_manifest"),
        "{error}"
    );
}

#[test]
fn renderer_handles_empty_and_long_partial_evidence_without_claim_drift() {
    let mut empty = predecessor_review();
    empty.findings.clear();
    empty.validate().unwrap();
    let empty_fixture = Fixture::new(
        empty,
        vec!["Empty finding set retained exactly".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    render_markdown(empty_fixture.options()).unwrap();
    let report = fs::read_to_string(empty_fixture.out.join("report.md")).unwrap();
    assert!(report.contains("No findings were reported"));
    let manifest: MarkdownManifest =
        serde_json::from_slice(&fs::read(empty_fixture.out.join("manifest.json")).unwrap())
            .unwrap();
    assert!(manifest.finding_ids.is_empty());

    let mut long = predecessor_review();
    long.findings[0].rationale = format!(
        "{} partial evidence; comparison not comparable",
        "bounded evidence ".repeat(300)
    );
    long.findings[0].limitations =
        vec!["partial evidence retained; historical comparison is not comparable".to_string()];
    long.validate().unwrap();
    let long_fixture = Fixture::new(
        long,
        vec!["Long and partial evidence retained".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    render_markdown(long_fixture.options()).unwrap();
    let report = fs::read_to_string(long_fixture.out.join("report.md")).unwrap();
    assert!(report.contains("comparison not comparable"));
    assert!(report.len() > 5_000);
}

#[test]
fn renderer_escapes_links_and_rechecks_secret_like_claims() {
    let fixture = Fixture::new(
        predecessor_review(),
        vec!["See [operator guide](https://example.com/guide)".to_string()],
        MARKDOWN_RENDERER_VERSION,
    );
    render_markdown(fixture.options()).unwrap();
    let report = fs::read_to_string(fixture.out.join("report.md")).unwrap();
    assert!(!report.contains("[operator guide](https://example.com/guide)"));
    assert!(report.contains("\\[operator guide\\]\\(https://example\\.com/guide\\)"));

    let review = predecessor_review();
    let destination = Path::new(env!("CARGO_TARGET_TMPDIR"));
    let input: Result<ManifestInput, _> = serde_json::from_value(json!({
        "schema":"codefriend.publication_manifest_input.v1",
        "artifact_manifest":[{"path":"synthesis.json","digest":"a".repeat(64)}],
        "renderer_versions":{"markdown":"v1"},
        "target":"report",
        "claims":["ghp_example_secret_must_not_render"],
        "nonclaims":["No external publication"]
    }));
    let error = input
        .unwrap()
        .publication(&review, destination)
        .unwrap_err()
        .to_string();
    assert!(error.contains("unsafe_or_empty_contract_text"), "{error}");
    assert!(hash(&review).unwrap().len() == 64);
}
