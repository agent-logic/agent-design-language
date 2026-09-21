//! Deterministic, script-free HTML export for an approved CodeFriend review.

use super::{
    approval::{read_decision_head, DecisionKind, DecisionRecord},
    manifest::{
        destination_digest, read_publication, read_review, reject_symlink_components,
        snapshot_artifacts,
    },
    markdown::{
        approved_input_path, normalized_path, publish_with_attachments,
        read_remediation_from_snapshot, read_synthesis_from_snapshot, read_test_plan_from_snapshot,
        require_bundle_files, validate_plan_parity, validate_source_identity,
    },
};
use crate::codefriend::{
    actions::{remediation, test_plan},
    evidence::{contracts::ReviewRecord, hash, valid_digest},
    ingestion::{digest, unsafe_content},
    review::synthesis::{ReviewSynthesis, SynthesizedFinding},
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};

pub const HTML_RENDERER_VERSION: &str = "v1";
pub const HTML_MANIFEST_SCHEMA: &str = "codefriend.html_report_manifest.v1";
pub const HTML_RESULT_SCHEMA: &str = "codefriend.html_render_result.v1";
const MAX_RENDERED_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlRenderOptions {
    pub review_record: PathBuf,
    pub publication: PathBuf,
    pub approval_store: PathBuf,
    pub artifact_root: PathBuf,
    pub synthesis: PathBuf,
    pub remediation_plan: PathBuf,
    pub test_plan: PathBuf,
    pub destination_root: PathBuf,
    pub out: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HtmlManifest {
    pub schema: String,
    pub renderer_version: String,
    pub review_record_digest: String,
    pub run_digest: String,
    pub finding_set_digest: String,
    pub synthesis_digest: String,
    pub remediation_plan_digest: String,
    pub test_plan_digest: String,
    pub publication_binding_digest: String,
    pub approval_decision_digest: String,
    pub repository: String,
    pub revision: String,
    pub scope_digest: String,
    pub target: String,
    pub report_path: String,
    pub report_digest: String,
    pub finding_ids: Vec<String>,
    pub claims: Vec<String>,
    pub nonclaims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HtmlRenderResult {
    pub schema: String,
    pub target: String,
    pub renderer_version: String,
    pub report_digest: String,
    pub manifest_digest: String,
    pub finding_count: usize,
    pub approval_decision_digest: String,
}

pub fn render_html(options: HtmlRenderOptions) -> Result<HtmlRenderResult> {
    ensure!(!options.out.exists(), "html_output_target_already_exists");
    ensure!(
        options.destination_root.exists()
            && fs::symlink_metadata(&options.destination_root)?.is_dir(),
        "html_destination_root_missing_or_invalid"
    );
    reject_symlink_components(&options.destination_root)?;
    reject_symlink_components(&options.out)?;

    let review = read_review(&options.review_record)?;
    let publication = read_publication(&options.publication)?;
    publication.validate(&review)?;
    let artifacts = snapshot_artifacts(&options.artifact_root, &publication.artifact_manifest)?;
    let decision = read_decision_head(&options.approval_store, &review, &publication)?
        .ok_or_else(|| anyhow::anyhow!("html_requires_publication_decision"))?;
    ensure!(
        decision.decision == DecisionKind::Approved,
        "html_requires_current_approval"
    );
    ensure!(
        decision.publication.binding_digest()? == publication.binding_digest()?,
        "html_approval_binding_mismatch"
    );
    ensure!(
        publication
            .renderer_versions
            .get("html")
            .is_some_and(|version| version == HTML_RENDERER_VERSION),
        "html_renderer_identity_mismatch"
    );
    ensure!(
        destination_digest(&options.destination_root)? == publication.destination_digest,
        "html_destination_identity_mismatch"
    );
    ensure!(
        normalized_path(&options.destination_root.join(&publication.target))?
            == normalized_path(&options.out)?,
        "html_target_identity_mismatch"
    );

    let synthesis_path = approved_input_path(&options.synthesis, &publication.artifact_manifest)?;
    let remediation_path =
        approved_input_path(&options.remediation_plan, &publication.artifact_manifest)?;
    let test_path = approved_input_path(&options.test_plan, &publication.artifact_manifest)?;
    require_bundle_files(
        &options.synthesis,
        &["synthesis.json", "manifest.json", "review-record.json"],
        &artifacts,
    )?;
    require_bundle_files(
        &options.remediation_plan,
        &[
            "remediation-plan.json",
            "manifest.json",
            "synthesis.json",
            "synthesis-manifest.json",
            "review-record.json",
        ],
        &artifacts,
    )?;
    require_bundle_files(
        &options.test_plan,
        &[
            "test-plan.json",
            "manifest.json",
            "synthesis.json",
            "synthesis-manifest.json",
            "review-record.json",
        ],
        &artifacts,
    )?;
    let synthesis = read_synthesis_from_snapshot(&artifacts, &synthesis_path)?;
    let remediation = read_remediation_from_snapshot(&artifacts, &remediation_path)?;
    let tests = read_test_plan_from_snapshot(&artifacts, &test_path)?;
    validate_source_identity(&review, &synthesis, &remediation, &tests)?;
    validate_plan_parity(&synthesis, &remediation, &tests)?;

    let mut report = render_report(&review, &synthesis, &remediation, &tests, &decision)?;
    let architecture = super::architecture::from_snapshot(&artifacts, &review)?;
    if !architecture.is_empty() {
        report = report.replace(
            "</body>",
            &format!("{}</body>", super::architecture::html(&architecture)?),
        );
    }
    ensure!(
        report.len() <= MAX_RENDERED_BYTES,
        "html_report_byte_limit_exceeded"
    );
    ensure!(
        !unsafe_content("report.html", &report),
        "html_redaction_recheck_failed"
    );
    let report_digest = digest(report.as_bytes());
    let finding_ids = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    let manifest = HtmlManifest {
        schema: HTML_MANIFEST_SCHEMA.to_string(),
        renderer_version: HTML_RENDERER_VERSION.to_string(),
        review_record_digest: hash(&review)?,
        run_digest: hash(&review.run)?,
        finding_set_digest: review.finding_digest()?,
        synthesis_digest: hash(&synthesis)?,
        remediation_plan_digest: hash(&remediation)?,
        test_plan_digest: hash(&tests)?,
        publication_binding_digest: publication.binding_digest()?,
        approval_decision_digest: decision.digest.clone(),
        repository: review.run.repository.clone(),
        revision: review.run.revision.clone(),
        scope_digest: review.run.scope_digest.clone(),
        target: publication.target.clone(),
        report_path: "report.html".to_string(),
        report_digest: report_digest.clone(),
        finding_ids,
        claims: publication.claims.clone(),
        nonclaims: publication.nonclaims.clone(),
    };
    validate_manifest(
        &manifest,
        &review,
        &synthesis,
        &remediation,
        &tests,
        &publication,
        &decision.digest,
    )?;
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    ensure!(
        !unsafe_content("manifest.json", std::str::from_utf8(&manifest_bytes)?),
        "html_manifest_redaction_recheck_failed"
    );
    let (actual_report, actual_manifest) = publish_with_attachments(
        &options.destination_root,
        std::path::Path::new(&publication.target),
        "report.html",
        report.as_bytes(),
        &manifest_bytes,
        MAX_RENDERED_BYTES as u64,
        "html",
        &super::architecture::attachments(&architecture)?,
    )?;
    ensure!(
        actual_report == report.as_bytes(),
        "html_report_readback_mismatch"
    );
    let read_manifest: HtmlManifest =
        serde_json::from_slice(&actual_manifest).context("html_manifest_readback_invalid")?;
    ensure!(read_manifest == manifest, "html_manifest_readback_mismatch");
    ensure!(
        digest(&actual_report) == manifest.report_digest,
        "html_report_digest_mismatch"
    );
    Ok(HtmlRenderResult {
        schema: HTML_RESULT_SCHEMA.to_string(),
        target: publication.target,
        renderer_version: HTML_RENDERER_VERSION.to_string(),
        report_digest,
        manifest_digest: digest(&actual_manifest),
        finding_count: synthesis.synthesized_findings.len(),
        approval_decision_digest: decision.digest,
    })
}

pub(crate) fn render_report(
    review: &ReviewRecord,
    synthesis: &ReviewSynthesis,
    remediation: &remediation::RemediationPlan,
    tests: &test_plan::TestPlan,
    decision: &DecisionRecord,
) -> Result<String> {
    let evidence = review
        .admission
        .evidence
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let remediation_by_finding = remediation
        .actions
        .iter()
        .map(|item| (item.finding_id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let tests_by_finding = tests
        .test_cases
        .iter()
        .map(|item| (item.finding_id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let remediation_omissions = remediation
        .omitted_findings
        .iter()
        .map(|item| (item.finding_id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let test_omissions = tests
        .omitted_findings
        .iter()
        .map(|item| (item.finding_id.as_str(), item))
        .collect::<BTreeMap<_, _>>();

    let mut out = String::from("<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>CodeFriend Review Report</title><style>body{font:16px/1.55 system-ui,sans-serif;max-width:78rem;margin:auto;padding:2rem;color:#18202a;background:#fff}nav,section,article{margin:2rem 0}nav ul{columns:2}a{color:#0757a5}a:focus{outline:3px solid #f5a623;outline-offset:3px}code{overflow-wrap:anywhere}dt{font-weight:700;margin-top:.65rem}dd{margin-left:0}.finding{border-top:2px solid #ccd4dd;padding-top:1.5rem}.meta{background:#f4f6f8;padding:1rem}h1,h2,h3{line-height:1.2}.architecture table{width:100%;border-collapse:collapse;table-layout:fixed;font-size:.9rem}.architecture th,.architecture td{text-align:left;vertical-align:top;padding:.6rem;border:1px solid #ccd4dd;overflow-wrap:anywhere}.architecture th{background:#f4f6f8}.architecture img{max-width:100%;height:auto}</style></head><body>\n<a href=\"#report\">Skip to report</a><main id=\"report\"><h1>CodeFriend Review Report</h1>\n");
    out.push_str("<nav aria-label=\"Report contents\"><h2>Contents</h2><ul><li><a href=\"#source\">Source and scope</a></li><li><a href=\"#approval\">Publication approval</a></li><li><a href=\"#findings\">Findings</a></li><li><a href=\"#evidence\">Evidence index</a></li><li><a href=\"#boundary\">Output boundary</a></li></ul><h3>Finding index</h3><ul>");
    for finding in &synthesis.synthesized_findings {
        write!(
            out,
            "<li><a href=\"#{}\">{}</a></li>",
            finding_anchor(&finding.id),
            html_text(&finding.title)?
        )?;
    }
    out.push_str("</ul></nav><section id=\"source\"><h2>Source and scope</h2><dl class=\"meta\">");
    field(&mut out, "Repository", &review.run.repository)?;
    field(&mut out, "Revision", &review.run.revision)?;
    field(&mut out, "Scope digest", &review.run.scope_digest)?;
    field(&mut out, "Run", &review.run.id)?;
    field(
        &mut out,
        "Completion",
        &format!("{:?}", review.run.completion).to_ascii_lowercase(),
    )?;
    list(&mut out, "Included paths", &review.run.included)?;
    list(&mut out, "Excluded paths", &review.run.excluded)?;
    if let Some(coverage) = &review.run.coverage {
        field(&mut out, "Review execution", "complete")?;
        field(
            &mut out,
            "Source coverage",
            "incomplete: privacy-filtered files were not reviewed",
        )?;
        let omissions: Vec<String> = coverage
            .omissions
            .iter()
            .map(|omission| {
                format!(
                    "{}: excluded by privacy filtering; contents were not reviewed",
                    omission.path
                )
            })
            .collect();
        list(&mut out, "Privacy exclusions", &omissions)?;
        field(&mut out, "Coverage limitation", "Findings cover retained source only. Excluded files may contain additional problems and affect dependencies or runtime behavior.")?;
    }
    list(&mut out, "Run failures", &review.run.failures)?;
    out.push_str(
        "</dl></section><section id=\"approval\"><h2>Publication approval</h2><dl class=\"meta\">",
    );
    field(&mut out, "Decision", "approved")?;
    field(&mut out, "Decision digest", &decision.digest)?;
    field(&mut out, "Approved by", &decision.actor)?;
    field(&mut out, "Reason", &decision.reason)?;
    field(&mut out, "Approved at", &decision.decided_at.to_string())?;
    field(
        &mut out,
        "Publication binding",
        &decision.publication.binding_digest()?,
    )?;
    field(&mut out, "Target", &decision.publication.target)?;
    field(&mut out, "Renderer", HTML_RENDERER_VERSION)?;
    list(&mut out, "Claims", &decision.publication.claims)?;
    list(&mut out, "Nonclaims", &decision.publication.nonclaims)?;
    out.push_str("</dl></section><section id=\"findings\"><h2>Findings</h2>");
    if synthesis.synthesized_findings.is_empty() {
        out.push_str("<p>No findings were reported by the completed four-perspective review.</p>");
    }
    for finding in &synthesis.synthesized_findings {
        render_finding(
            &mut out,
            finding,
            &evidence,
            remediation_by_finding.get(finding.id.as_str()).copied(),
            remediation_omissions.get(finding.id.as_str()).copied(),
            tests_by_finding.get(finding.id.as_str()).copied(),
            test_omissions.get(finding.id.as_str()).copied(),
        )?;
    }
    out.push_str("</section><section id=\"evidence\"><h2>Evidence index</h2><ul>");
    for item in &review.admission.evidence {
        write!(out, "<li id=\"{}\"><code>{}</code> — <code>{}</code>; source object <code>{}</code>; content <code>{}</code></li>", evidence_anchor(&item.id), html_text(&item.id)?, html_text(&item.path)?, html_text(&item.source_object)?, html_text(&item.content_digest)?)?;
    }
    out.push_str("</ul></section><section id=\"boundary\"><h2>Output boundary</h2><p>This is the canonical local HTML rendering of the approved review. It does not claim PDF, remote, or customer publication.</p></section></main></body></html>\n");
    Ok(out)
}

fn render_finding(
    out: &mut String,
    finding: &SynthesizedFinding,
    evidence: &BTreeMap<&str, &crate::codefriend::evidence::Evidence>,
    remediation: Option<&remediation::RemediationAction>,
    remediation_omission: Option<&remediation::OmittedFinding>,
    test: Option<&test_plan::TestCasePlan>,
    test_omission: Option<&test_plan::OmittedFinding>,
) -> Result<()> {
    write!(
        out,
        "<article class=\"finding\" id=\"{}\"><h3>{}</h3><dl>",
        finding_anchor(&finding.id),
        html_text(&finding.title)?
    )?;
    field(out, "Finding ID", &finding.id)?;
    field(
        out,
        "Severity",
        &format!("{:?}", finding.severity).to_ascii_lowercase(),
    )?;
    field(out, "Anchor", &finding.semantic_anchor)?;
    field(out, "Severity rationale", &finding.severity_rationale)?;
    field(
        out,
        "Disagreement",
        finding.disagreement.as_deref().unwrap_or("none recorded"),
    )?;
    list(out, "Scope limits and uncertainty", &finding.scope_limits)?;
    out.push_str("</dl><h4>Citations</h4><ul>");
    for id in &finding.evidence {
        let item = evidence
            .get(id.as_str())
            .ok_or_else(|| anyhow::anyhow!("html_missing_evidence_provenance"))?;
        write!(out, "<li><a href=\"#{}\"><code>{}</code></a> at <code>{}</code>; source object <code>{}</code>; content <code>{}</code></li>", evidence_anchor(&item.id), html_text(&item.id)?, html_text(&item.path)?, html_text(&item.source_object)?, html_text(&item.content_digest)?)?;
    }
    out.push_str("</ul><h4>Attributed source assessments</h4><ul>");
    for source in &finding.sources {
        write!(out, "<li><strong>{} / {}</strong> — {} Finding: {}. Severity: {}. Confidence: {:?}. Inference: {}.", html_text(&source.perspective)?, html_text(&source.rule)?, html_text(&source.rationale)?, html_text(&source.finding_id)?, html_text(&format!("{:?}", source.severity).to_ascii_lowercase())?, source.confidence, html_text(&source.inference)?)?;
        if !source.evidence.is_empty() {
            write!(
                out,
                " Evidence: {}.",
                html_text(&source.evidence.join("; "))?
            )?;
        }
        if !source.limitations.is_empty() {
            write!(
                out,
                " Limitations: {}.",
                html_text(&source.limitations.join("; "))?
            )?;
        }
        out.push_str("</li>");
    }
    out.push_str("</ul><h4>Remediation plan</h4><dl>");
    if let Some(action) = remediation {
        field(out, "Action", &action.title)?;
        field(out, "Action ID", &action.id)?;
        field(out, "Finding ID", &action.finding_id)?;
        field(
            out,
            "Severity",
            &format!("{:?}", action.severity).to_ascii_lowercase(),
        )?;
        list(out, "Source finding IDs", &action.source_finding_ids)?;
        list(out, "Evidence IDs", &action.evidence_ids)?;
        list(out, "Dependencies", &action.dependencies)?;
        field(out, "Owner role", &action.owner_role)?;
        field(out, "Assignment", &action.assignment_status)?;
        list(out, "Relevant paths", &action.relevant_paths)?;
        list(out, "Acceptance criteria", &action.acceptance_criteria)?;
        list(out, "Validation", &action.validation)?;
        list(
            out,
            "Risk, non-goals and uncertainty",
            &action.risk_and_non_goals,
        )?;
        list(out, "Additional uncertainty", &action.uncertainty)?;
    } else if let Some(omission) = remediation_omission {
        field(out, "Omitted finding ID", &omission.finding_id)?;
        field(out, "Omitted finding", &omission.title)?;
        field(out, "Remediation omitted", &omission.reason)?;
    }
    out.push_str("</dl><h4>Test plan</h4><dl>");
    if let Some(case) = test {
        field(out, "Test", &case.title)?;
        field(out, "Test ID", &case.id)?;
        field(out, "Finding ID", &case.finding_id)?;
        field(
            out,
            "Severity",
            &format!("{:?}", case.severity).to_ascii_lowercase(),
        )?;
        list(out, "Source finding IDs", &case.source_finding_ids)?;
        list(out, "Source evidence", &case.source_evidence)?;
        field(out, "Behavior under test", &case.behavior_under_test)?;
        field(out, "Proposed location", &case.proposed_test_location)?;
        field(out, "Fixture", &case.proposed_fixture)?;
        field(
            out,
            "Expected pre-fix failure",
            &case.expected_pre_fix_failure,
        )?;
        field(
            out,
            "Expected post-fix assertion",
            &case.expected_post_fix_assertion,
        )?;
        field(out, "Validation lane", &case.validation_lane)?;
        field(out, "Resource profile", &case.resource_profile)?;
        field(out, "Detection rationale", &case.detection_rationale)?;
        list(out, "Test non-goals", &case.non_goals)?;
        list(out, "Test scope limits", &case.scope_limits)?;
    } else if let Some(omission) = test_omission {
        field(out, "Omitted finding ID", &omission.finding_id)?;
        field(out, "Omitted finding", &omission.title)?;
        field(out, "Test omitted", &omission.reason)?;
    }
    out.push_str("</dl></article>");
    Ok(())
}

fn field(out: &mut String, label: &str, value: &str) -> Result<()> {
    write!(
        out,
        "<dt>{}</dt><dd>{}</dd>",
        html_text(label)?,
        html_text(value)?
    )?;
    Ok(())
}

fn list(out: &mut String, label: &str, values: &[String]) -> Result<()> {
    write!(out, "<dt>{}</dt><dd>", html_text(label)?)?;
    if values.is_empty() {
        out.push_str("none");
    } else {
        out.push_str("<ul>");
        for value in values {
            write!(out, "<li>{}</li>", html_text(value)?)?;
        }
        out.push_str("</ul>");
    }
    out.push_str("</dd>");
    Ok(())
}

fn html_text(value: &str) -> Result<String> {
    ensure!(
        !value.is_empty() && value.len() <= 64 * 1024,
        "html_text_empty_or_too_large"
    );
    ensure!(
        !unsafe_content("", value),
        "html_text_failed_redaction_recheck"
    );
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        ensure!(
            !character.is_control() || matches!(character, '\n' | '\r' | '\t'),
            "html_text_contains_control_character"
        );
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            '\n' | '\r' | '\t' => output.push(' '),
            _ => output.push(character),
        }
    }
    Ok(output)
}

fn finding_anchor(id: &str) -> String {
    format!("finding-{}", &digest(id.as_bytes())[..16])
}
fn evidence_anchor(id: &str) -> String {
    format!("evidence-{}", &digest(id.as_bytes())[..16])
}

pub(crate) fn validate_manifest(
    manifest: &HtmlManifest,
    review: &ReviewRecord,
    synthesis: &ReviewSynthesis,
    remediation: &remediation::RemediationPlan,
    tests: &test_plan::TestPlan,
    publication: &crate::codefriend::evidence::contracts::Publication,
    decision_digest: &str,
) -> Result<()> {
    ensure!(
        manifest.schema == HTML_MANIFEST_SCHEMA
            && manifest.renderer_version == HTML_RENDERER_VERSION
            && manifest.review_record_digest == hash(review)?
            && manifest.run_digest == hash(&review.run)?
            && manifest.finding_set_digest == review.finding_digest()?
            && manifest.synthesis_digest == hash(synthesis)?
            && manifest.remediation_plan_digest == hash(remediation)?
            && manifest.test_plan_digest == hash(tests)?
            && manifest.publication_binding_digest == publication.binding_digest()?
            && manifest.approval_decision_digest == decision_digest
            && manifest.repository == review.run.repository
            && manifest.revision == review.run.revision
            && manifest.scope_digest == review.run.scope_digest
            && manifest.target == publication.target
            && manifest.report_path == "report.html"
            && valid_digest(&manifest.report_digest)
            && manifest.claims == publication.claims
            && manifest.nonclaims == publication.nonclaims,
        "invalid_html_manifest_binding"
    );
    let expected = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    ensure!(
        manifest.finding_ids == expected,
        "html_manifest_finding_parity_mismatch"
    );
    Ok(())
}

use std::fmt::Write as _;
