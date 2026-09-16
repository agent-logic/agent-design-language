//! Deterministic Markdown export for an approved CodeFriend review.

use super::{
    approval::{read_decision_head, DecisionKind},
    manifest::{
        destination_digest, read_publication, read_review, reject_symlink_components,
        verify_artifacts,
    },
};
use crate::codefriend::{
    actions::{
        remediation::{self, RemediationPlan},
        test_plan::{self, TestPlan},
    },
    evidence::{hash, valid_digest},
    ingestion::{digest, unsafe_content, validate_path},
    review::synthesis::{read_synthesis_from_file, ReviewSynthesis, SynthesizedFinding},
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const MARKDOWN_RENDERER_VERSION: &str = "v1";
pub const MARKDOWN_MANIFEST_SCHEMA: &str = "codefriend.markdown_report_manifest.v1";
pub const MARKDOWN_RESULT_SCHEMA: &str = "codefriend.markdown_render_result.v1";
const MAX_RENDERED_BYTES: usize = 16 * 1024 * 1024;
static NEXT_STAGE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownRenderOptions {
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
pub struct MarkdownManifest {
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
pub struct MarkdownRenderResult {
    pub schema: String,
    pub target: String,
    pub renderer_version: String,
    pub report_digest: String,
    pub manifest_digest: String,
    pub finding_count: usize,
    pub approval_decision_digest: String,
}

pub fn render_markdown(options: MarkdownRenderOptions) -> Result<MarkdownRenderResult> {
    ensure!(
        !options.out.exists(),
        "markdown_output_target_already_exists"
    );
    ensure!(
        options.destination_root.exists()
            && fs::symlink_metadata(&options.destination_root)?.is_dir(),
        "markdown_destination_root_missing_or_invalid"
    );
    reject_symlink_components(&options.destination_root)?;
    reject_symlink_components(&options.out)?;

    let review = read_review(&options.review_record)?;
    let publication = read_publication(&options.publication)?;
    publication.validate(&review)?;
    verify_artifacts(&options.artifact_root, &publication.artifact_manifest)?;
    let decision = read_decision_head(&options.approval_store, &review, &publication)?
        .ok_or_else(|| anyhow::anyhow!("markdown_requires_publication_decision"))?;
    ensure!(
        decision.decision == DecisionKind::Approved,
        "markdown_requires_current_approval"
    );
    ensure!(
        decision.publication.binding_digest()? == publication.binding_digest()?,
        "markdown_approval_binding_mismatch"
    );
    ensure!(
        publication
            .renderer_versions
            .get("markdown")
            .is_some_and(|version| version == MARKDOWN_RENDERER_VERSION),
        "markdown_renderer_identity_mismatch"
    );
    ensure!(
        destination_digest(&options.destination_root)? == publication.destination_digest,
        "markdown_destination_identity_mismatch"
    );
    let expected_out = options.destination_root.join(&publication.target);
    ensure!(
        normalized_path(&expected_out)? == normalized_path(&options.out)?,
        "markdown_target_identity_mismatch"
    );

    let synthesis_path = approved_input_path(
        &options.artifact_root,
        &options.synthesis,
        &publication.artifact_manifest,
    )?;
    let remediation_path = approved_input_path(
        &options.artifact_root,
        &options.remediation_plan,
        &publication.artifact_manifest,
    )?;
    let test_path = approved_input_path(
        &options.artifact_root,
        &options.test_plan,
        &publication.artifact_manifest,
    )?;
    require_bundle_files(
        &options.artifact_root,
        &options.synthesis,
        &["synthesis.json", "manifest.json", "review-record.json"],
        &publication.artifact_manifest,
    )?;
    require_bundle_files(
        &options.artifact_root,
        &options.remediation_plan,
        &[
            "remediation-plan.json",
            "manifest.json",
            "synthesis.json",
            "synthesis-manifest.json",
            "review-record.json",
        ],
        &publication.artifact_manifest,
    )?;
    require_bundle_files(
        &options.artifact_root,
        &options.test_plan,
        &[
            "test-plan.json",
            "manifest.json",
            "synthesis.json",
            "synthesis-manifest.json",
            "review-record.json",
        ],
        &publication.artifact_manifest,
    )?;

    let synthesis = read_synthesis_from_file(&synthesis_path)?;
    let remediation = remediation::read_plan_from_file(&remediation_path)?;
    let tests = test_plan::read_plan_from_file(&test_path)?;
    validate_source_identity(&review, &synthesis, &remediation, &tests)?;
    validate_plan_parity(&synthesis, &remediation, &tests)?;

    let report = render_report(&review, &synthesis, &remediation, &tests, &decision)?;
    ensure!(
        report.len() <= MAX_RENDERED_BYTES,
        "markdown_report_byte_limit_exceeded"
    );
    ensure!(
        !unsafe_content("report.md", &report),
        "markdown_redaction_recheck_failed"
    );
    let report_digest = digest(report.as_bytes());
    let finding_ids = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    let manifest = MarkdownManifest {
        schema: MARKDOWN_MANIFEST_SCHEMA.to_string(),
        renderer_version: MARKDOWN_RENDERER_VERSION.to_string(),
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
        report_path: "report.md".to_string(),
        report_digest: report_digest.clone(),
        finding_ids,
        claims: publication.claims.clone(),
        nonclaims: publication.nonclaims.clone(),
    };
    validate_manifest(
        &manifest,
        &review,
        &synthesis,
        &publication,
        &decision.digest,
    )?;
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    ensure!(
        !unsafe_content("manifest.json", std::str::from_utf8(&manifest_bytes)?),
        "markdown_manifest_redaction_recheck_failed"
    );

    let target_parent = options
        .out
        .parent()
        .ok_or_else(|| anyhow::anyhow!("markdown_target_parent_missing"))?;
    fs::create_dir_all(target_parent)?;
    reject_symlink_components(target_parent)?;
    ensure!(
        normalized_path(target_parent)?.starts_with(&normalized_path(&options.destination_root)?),
        "markdown_target_outside_destination"
    );
    let stage = create_stage(target_parent)?;
    let write_result = (|| -> Result<()> {
        write_create_only(&stage.join("report.md"), report.as_bytes())?;
        write_create_only(&stage.join("manifest.json"), &manifest_bytes)?;
        File::open(&stage)?.sync_all()?;
        fs::rename(&stage, &options.out)?;
        File::open(target_parent)?.sync_all()?;
        Ok(())
    })();
    if let Err(error) = write_result {
        let _ = fs::remove_dir_all(&stage);
        return Err(error);
    }

    let actual_report = read_limited(&options.out.join("report.md"), MAX_RENDERED_BYTES as u64)?;
    let actual_manifest = read_limited(&options.out.join("manifest.json"), 2 * 1024 * 1024)?;
    ensure!(
        actual_report == report.as_bytes(),
        "markdown_report_readback_mismatch"
    );
    let read_manifest: MarkdownManifest =
        serde_json::from_slice(&actual_manifest).context("markdown_manifest_readback_invalid")?;
    ensure!(
        read_manifest == manifest,
        "markdown_manifest_readback_mismatch"
    );
    ensure!(
        digest(&actual_report) == manifest.report_digest,
        "markdown_report_digest_mismatch"
    );
    let manifest_digest = digest(&actual_manifest);
    Ok(MarkdownRenderResult {
        schema: MARKDOWN_RESULT_SCHEMA.to_string(),
        target: publication.target,
        renderer_version: MARKDOWN_RENDERER_VERSION.to_string(),
        report_digest,
        manifest_digest,
        finding_count: synthesis.synthesized_findings.len(),
        approval_decision_digest: decision.digest,
    })
}

fn approved_input_path(
    artifact_root: &Path,
    relative: &Path,
    artifacts: &[crate::codefriend::evidence::contracts::Artifact],
) -> Result<PathBuf> {
    let relative = relative
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("markdown_input_path_not_utf8"))?;
    validate_path(relative)?;
    let artifact = artifacts
        .iter()
        .find(|artifact| artifact.path == relative)
        .ok_or_else(|| anyhow::anyhow!("markdown_input_not_in_approved_manifest"))?;
    ensure!(
        valid_digest(&artifact.digest),
        "invalid_markdown_input_digest"
    );
    let path = artifact_root.join(relative);
    ensure!(
        path.starts_with(artifact_root),
        "markdown_input_outside_artifact_root"
    );
    Ok(path)
}

fn require_bundle_files(
    artifact_root: &Path,
    selected: &Path,
    required_names: &[&str],
    artifacts: &[crate::codefriend::evidence::contracts::Artifact],
) -> Result<()> {
    let parent = selected
        .parent()
        .ok_or_else(|| anyhow::anyhow!("markdown_bundle_parent_missing"))?;
    for name in required_names {
        approved_input_path(artifact_root, &parent.join(name), artifacts)?;
    }
    Ok(())
}

fn validate_source_identity(
    review: &crate::codefriend::evidence::contracts::ReviewRecord,
    synthesis: &ReviewSynthesis,
    remediation: &RemediationPlan,
    tests: &TestPlan,
) -> Result<()> {
    let synthesis_digest = hash(synthesis)?;
    ensure!(
        synthesis.review_record_digest == hash(review)?
            && synthesis.run_id == review.run.id
            && synthesis.repository == review.run.repository
            && synthesis.revision == review.run.revision
            && synthesis.scope_digest == review.run.scope_digest,
        "markdown_synthesis_source_identity_mismatch"
    );
    for (kind, digest_value, run_id, repository, revision, scope_digest) in [
        (
            "remediation",
            remediation.synthesis_digest.as_str(),
            remediation.run_id.as_str(),
            remediation.repository.as_str(),
            remediation.revision.as_str(),
            remediation.scope_digest.as_str(),
        ),
        (
            "test_plan",
            tests.synthesis_digest.as_str(),
            tests.run_id.as_str(),
            tests.repository.as_str(),
            tests.revision.as_str(),
            tests.scope_digest.as_str(),
        ),
    ] {
        ensure!(
            digest_value == synthesis_digest
                && run_id == synthesis.run_id
                && repository == synthesis.repository
                && revision == synthesis.revision
                && scope_digest == synthesis.scope_digest,
            "markdown_{kind}_source_identity_mismatch"
        );
    }
    Ok(())
}

fn validate_plan_parity(
    synthesis: &ReviewSynthesis,
    remediation: &RemediationPlan,
    tests: &TestPlan,
) -> Result<()> {
    let expected = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let remediation_ids = remediation
        .actions
        .iter()
        .map(|action| action.finding_id.as_str())
        .chain(
            remediation
                .omitted_findings
                .iter()
                .map(|finding| finding.finding_id.as_str()),
        )
        .collect::<Vec<_>>();
    let test_ids = tests
        .test_cases
        .iter()
        .map(|case| case.finding_id.as_str())
        .chain(
            tests
                .omitted_findings
                .iter()
                .map(|finding| finding.finding_id.as_str()),
        )
        .collect::<Vec<_>>();
    ensure!(
        remediation_ids.len() == remediation_ids.iter().collect::<BTreeSet<_>>().len()
            && remediation_ids.iter().copied().collect::<BTreeSet<_>>() == expected,
        "markdown_remediation_finding_parity_mismatch"
    );
    ensure!(
        test_ids.len() == test_ids.iter().collect::<BTreeSet<_>>().len()
            && test_ids.iter().copied().collect::<BTreeSet<_>>() == expected,
        "markdown_test_plan_finding_parity_mismatch"
    );
    Ok(())
}

fn render_report(
    review: &crate::codefriend::evidence::contracts::ReviewRecord,
    synthesis: &ReviewSynthesis,
    remediation: &RemediationPlan,
    tests: &TestPlan,
    decision: &super::approval::DecisionRecord,
) -> Result<String> {
    let publication = &decision.publication;
    let evidence = review
        .admission
        .evidence
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let remediation_by_finding = remediation
        .actions
        .iter()
        .map(|action| (action.finding_id.as_str(), action))
        .collect::<BTreeMap<_, _>>();
    let tests_by_finding = tests
        .test_cases
        .iter()
        .map(|case| (case.finding_id.as_str(), case))
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

    let mut out = String::new();
    out.push_str("# CodeFriend Review Report\n\n");
    out.push_str("## Source and scope\n\n");
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
    list(&mut out, "Run failures", &review.run.failures)?;

    out.push_str("\n## Publication approval\n\n");
    field(&mut out, "Decision", "approved")?;
    field(&mut out, "Decision digest", &decision.digest)?;
    field(&mut out, "Approved by", &decision.actor)?;
    field(&mut out, "Reason", &decision.reason)?;
    field(&mut out, "Approved at", &decision.decided_at.to_string())?;
    field(
        &mut out,
        "Publication binding",
        &publication.binding_digest()?,
    )?;
    field(&mut out, "Target", &publication.target)?;
    field(&mut out, "Renderer", MARKDOWN_RENDERER_VERSION)?;
    list(&mut out, "Claims", &publication.claims)?;
    list(&mut out, "Nonclaims", &publication.nonclaims)?;

    out.push_str("\n## Findings\n\n");
    if synthesis.synthesized_findings.is_empty() {
        out.push_str("No findings were reported by the completed four-perspective review.\n");
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
    out.push_str("\n## Output boundary\n\n");
    out.push_str("This is the canonical local Markdown rendering of the approved review. It does not claim HTML, PDF, remote, or customer publication.\n");
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
    out.push_str("\n### ");
    out.push_str(&markdown_text(&finding.title)?);
    out.push_str("\n\n");
    field(out, "Finding ID", &finding.id)?;
    field(
        out,
        "Severity",
        &format!("{:?}", finding.severity).to_ascii_lowercase(),
    )?;
    field(out, "Anchor", &finding.semantic_anchor)?;
    field(out, "Severity rationale", &finding.severity_rationale)?;
    if let Some(disagreement) = &finding.disagreement {
        field(out, "Disagreement", disagreement)?;
    } else {
        field(out, "Disagreement", "none recorded")?;
    }
    list(out, "Scope limits and uncertainty", &finding.scope_limits)?;
    out.push_str("\n**Citations**\n\n");
    for id in &finding.evidence {
        let item = evidence
            .get(id.as_str())
            .ok_or_else(|| anyhow::anyhow!("markdown_missing_evidence_provenance"))?;
        out.push_str("- `");
        out.push_str(&markdown_code(&item.path));
        out.push_str("` at source object `");
        out.push_str(&markdown_code(&item.source_object));
        out.push_str("`; evidence `");
        out.push_str(&markdown_code(&item.id));
        out.push_str("`; content `");
        out.push_str(&markdown_code(&item.content_digest));
        out.push_str("`\n");
    }
    out.push_str("\n**Attributed source assessments**\n\n");
    for source in &finding.sources {
        out.push_str("- **");
        out.push_str(&markdown_text(&source.perspective)?);
        out.push_str(" / ");
        out.push_str(&markdown_text(&source.rule)?);
        out.push_str("** — ");
        out.push_str(&markdown_text(&source.rationale)?);
        out.push_str(" Confidence: ");
        out.push_str(&markdown_text(&format!("{:?}", source.confidence))?);
        out.push_str(". Inference: ");
        out.push_str(&markdown_text(&source.inference)?);
        if !source.limitations.is_empty() {
            out.push_str(". Limitations: ");
            out.push_str(&markdown_text(&source.limitations.join("; "))?);
        }
        out.push_str(".\n");
    }
    out.push_str("\n**Remediation plan**\n\n");
    if let Some(action) = remediation {
        field(out, "Action", &action.title)?;
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
        field(out, "Remediation omitted", &omission.reason)?;
    }
    out.push_str("\n**Test plan**\n\n");
    if let Some(case) = test {
        field(out, "Test", &case.title)?;
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
        list(out, "Test non-goals", &case.non_goals)?;
        list(out, "Test scope limits", &case.scope_limits)?;
    } else if let Some(omission) = test_omission {
        field(out, "Test omitted", &omission.reason)?;
    }
    Ok(())
}

fn field(out: &mut String, label: &str, value: &str) -> Result<()> {
    out.push_str("- **");
    out.push_str(label);
    out.push_str(":** ");
    out.push_str(&markdown_text(value)?);
    out.push('\n');
    Ok(())
}

fn list(out: &mut String, label: &str, values: &[String]) -> Result<()> {
    out.push_str("- **");
    out.push_str(label);
    out.push_str(":**");
    if values.is_empty() {
        out.push_str(" none\n");
    } else {
        out.push('\n');
        for value in values {
            out.push_str("  - ");
            out.push_str(&markdown_text(value)?);
            out.push('\n');
        }
    }
    Ok(())
}

fn markdown_text(value: &str) -> Result<String> {
    ensure!(
        !value.is_empty() && value.len() <= 64 * 1024,
        "markdown_text_empty_or_too_large"
    );
    ensure!(
        !unsafe_content("", value),
        "markdown_text_failed_redaction_recheck"
    );
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        ensure!(
            !character.is_control() || matches!(character, '\n' | '\r' | '\t'),
            "markdown_text_contains_control_character"
        );
        match character {
            '\n' | '\r' | '\t' => output.push(' '),
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '<' | '>' | '(' | ')' | '#' | '+'
            | '-' | '.' | '!' | '|' => {
                output.push('\\');
                output.push(character);
            }
            _ => output.push(character),
        }
    }
    Ok(output)
}

fn markdown_code(value: &str) -> String {
    value.replace('`', "\\`").replace(['\n', '\r'], " ")
}

fn validate_manifest(
    manifest: &MarkdownManifest,
    review: &crate::codefriend::evidence::contracts::ReviewRecord,
    synthesis: &ReviewSynthesis,
    publication: &crate::codefriend::evidence::contracts::Publication,
    decision_digest: &str,
) -> Result<()> {
    ensure!(
        manifest.schema == MARKDOWN_MANIFEST_SCHEMA
            && manifest.renderer_version == MARKDOWN_RENDERER_VERSION
            && manifest.review_record_digest == hash(review)?
            && manifest.run_digest == hash(&review.run)?
            && manifest.finding_set_digest == review.finding_digest()?
            && manifest.synthesis_digest == hash(synthesis)?
            && manifest.publication_binding_digest == publication.binding_digest()?
            && manifest.approval_decision_digest == decision_digest
            && manifest.repository == review.run.repository
            && manifest.revision == review.run.revision
            && manifest.scope_digest == review.run.scope_digest
            && manifest.target == publication.target
            && manifest.report_path == "report.md"
            && valid_digest(&manifest.report_digest)
            && manifest.claims == publication.claims
            && manifest.nonclaims == publication.nonclaims,
        "invalid_markdown_manifest_binding"
    );
    let expected_ids = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    ensure!(
        manifest.finding_ids == expected_ids,
        "markdown_manifest_finding_parity_mismatch"
    );
    Ok(())
}

fn normalized_path(path: &Path) -> Result<PathBuf> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                ensure!(normalized.pop(), "markdown_path_escapes_root");
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    Ok(normalized)
}

fn create_stage(parent: &Path) -> Result<PathBuf> {
    loop {
        let serial = NEXT_STAGE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(
            ".codefriend-markdown-stage-{}-{serial}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }
}

fn write_create_only(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn read_limited(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?.take(limit + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "markdown_readback_too_large");
    Ok(bytes)
}
