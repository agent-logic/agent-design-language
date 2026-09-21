//! Deterministic Markdown export for an approved CodeFriend review.

pub(crate) const MARKDOWN_OUTPUT_BOUNDARY: &str = "This is the canonical local Markdown rendering of the approved review. It does not claim HTML, PDF, remote, or customer publication.";
use super::{
    approval::{read_decision_head, DecisionKind},
    manifest::{
        destination_digest, read_publication, read_review, reject_symlink_components,
        snapshot_artifacts, VerifiedArtifact,
    },
};
use crate::codefriend::{
    actions::{
        remediation::{self, RemediationManifest, RemediationPlan, REMEDIATION_MANIFEST_SCHEMA},
        test_plan::{self, TestPlan, TestPlanManifest, TEST_PLAN_MANIFEST_SCHEMA},
    },
    evidence::{contracts::ReviewRecord, hash, valid_digest},
    ingestion::{digest, unsafe_content, validate_path},
    review::synthesis::{
        self, ReviewSynthesis, SynthesisManifest, SynthesizedFinding, SYNTHESIS_MANIFEST_SCHEMA,
    },
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::CString,
    fs::{self, File},
    io::{Read, Write},
    os::unix::{
        ffi::OsStrExt,
        fs::MetadataExt,
        io::{AsRawFd, FromRawFd},
    },
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

pub(crate) struct PreparedReport {
    pub review: ReviewRecord,
    pub publication: crate::codefriend::evidence::contracts::Publication,
    pub decision: super::approval::DecisionRecord,
    pub synthesis: ReviewSynthesis,
    pub remediation: RemediationPlan,
    pub tests: TestPlan,
    pub text: String,
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

    let prepared = prepare_report(
        &options,
        "markdown",
        MARKDOWN_RENDERER_VERSION,
        MARKDOWN_OUTPUT_BOUNDARY,
    )?;
    let PreparedReport {
        review,
        publication,
        decision,
        synthesis,
        remediation,
        tests,
        text: report,
    } = prepared;
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

    let (actual_report, actual_manifest) = publish_create_only_anchored(
        &options.destination_root,
        Path::new(&publication.target),
        "report.md",
        report.as_bytes(),
        &manifest_bytes,
        MAX_RENDERED_BYTES as u64,
        "markdown",
    )?;
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

pub(crate) fn prepare_report(
    options: &MarkdownRenderOptions,
    renderer_key: &str,
    renderer_version: &str,
    output_boundary: &str,
) -> Result<PreparedReport> {
    let review = read_review(&options.review_record)?;
    let publication = read_publication(&options.publication)?;
    publication.validate(&review)?;
    let artifacts = snapshot_artifacts(&options.artifact_root, &publication.artifact_manifest)?;
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
            .get(renderer_key)
            .is_some_and(|version| version == renderer_version),
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
    let text = render_report(
        &review,
        &synthesis,
        &remediation,
        &tests,
        &decision,
        renderer_version,
        output_boundary,
    )?;
    Ok(PreparedReport {
        review,
        publication,
        decision,
        synthesis,
        remediation,
        tests,
        text,
    })
}

pub(super) fn approved_input_path(
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
    Ok(PathBuf::from(relative))
}

pub(super) fn require_bundle_files(
    selected: &Path,
    required_names: &[&str],
    artifacts: &[VerifiedArtifact],
) -> Result<()> {
    let parent = selected
        .parent()
        .ok_or_else(|| anyhow::anyhow!("markdown_bundle_parent_missing"))?;
    for name in required_names {
        snapshot_bytes(artifacts, &parent.join(name))?;
    }
    Ok(())
}

fn snapshot_bytes<'a>(artifacts: &'a [VerifiedArtifact], relative: &Path) -> Result<&'a [u8]> {
    let relative = relative
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("markdown_input_path_not_utf8"))?;
    validate_path(relative)?;
    artifacts
        .iter()
        .find(|artifact| artifact.path == relative)
        .map(|artifact| artifact.bytes.as_slice())
        .ok_or_else(|| anyhow::anyhow!("markdown_input_not_in_verified_snapshot"))
}

fn snapshot_json<T: serde::de::DeserializeOwned>(
    artifacts: &[VerifiedArtifact],
    relative: &Path,
    context: &'static str,
) -> Result<T> {
    serde_json::from_slice(snapshot_bytes(artifacts, relative)?).context(context)
}

fn bundle_path(selected: &Path, name: &str) -> Result<PathBuf> {
    Ok(selected
        .parent()
        .ok_or_else(|| anyhow::anyhow!("markdown_bundle_parent_missing"))?
        .join(name))
}

pub(super) fn read_synthesis_from_snapshot(
    artifacts: &[VerifiedArtifact],
    selected: &Path,
) -> Result<ReviewSynthesis> {
    ensure!(
        selected.file_name().and_then(|name| name.to_str()) == Some("synthesis.json"),
        "synthesis_bundle_requires_canonical_synthesis_ref"
    );
    let synthesis: ReviewSynthesis =
        snapshot_json(artifacts, selected, "synthesis_snapshot_invalid")?;
    let manifest: SynthesisManifest = snapshot_json(
        artifacts,
        &bundle_path(selected, "manifest.json")?,
        "synthesis_manifest_snapshot_invalid",
    )?;
    let review: ReviewRecord = snapshot_json(
        artifacts,
        &bundle_path(selected, "review-record.json")?,
        "synthesis_review_snapshot_invalid",
    )?;
    validate_synthesis_snapshot(&manifest, &synthesis, &review)?;
    Ok(synthesis)
}

fn validate_synthesis_snapshot(
    manifest: &SynthesisManifest,
    synthesis: &ReviewSynthesis,
    review: &ReviewRecord,
) -> Result<()> {
    review.validate()?;
    ensure!(
        manifest.schema == SYNTHESIS_MANIFEST_SCHEMA
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.review_record_ref == "review-record.json",
        "invalid_synthesis_manifest"
    );
    ensure!(
        manifest.synthesis_digest == hash(synthesis)?
            && manifest.review_record_digest == hash(review)?
            && manifest.synthesized_finding_count == synthesis.synthesized_findings.len()
            && manifest.input_finding_count == synthesis.input_finding_count
            && synthesis.review_record_digest == hash(review)?
            && synthesis::synthesize(review)? == *synthesis,
        "synthesis_snapshot_digest_or_canonicality_mismatch"
    );
    Ok(())
}

pub(super) fn read_remediation_from_snapshot(
    artifacts: &[VerifiedArtifact],
    selected: &Path,
) -> Result<RemediationPlan> {
    ensure!(
        selected.file_name().and_then(|name| name.to_str()) == Some("remediation-plan.json"),
        "remediation_plan_reference_mismatch"
    );
    let plan: RemediationPlan =
        snapshot_json(artifacts, selected, "remediation_plan_snapshot_invalid")?;
    let manifest: RemediationManifest = snapshot_json(
        artifacts,
        &bundle_path(selected, "manifest.json")?,
        "remediation_manifest_snapshot_invalid",
    )?;
    let synthesis: ReviewSynthesis = snapshot_json(
        artifacts,
        &bundle_path(selected, "synthesis.json")?,
        "remediation_synthesis_snapshot_invalid",
    )?;
    let synthesis_manifest: SynthesisManifest = snapshot_json(
        artifacts,
        &bundle_path(selected, "synthesis-manifest.json")?,
        "remediation_synthesis_manifest_snapshot_invalid",
    )?;
    let review: ReviewRecord = snapshot_json(
        artifacts,
        &bundle_path(selected, "review-record.json")?,
        "remediation_review_snapshot_invalid",
    )?;
    validate_synthesis_snapshot(&synthesis_manifest, &synthesis, &review)?;
    ensure!(
        manifest.schema == REMEDIATION_MANIFEST_SCHEMA
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.synthesis_manifest_ref == "synthesis-manifest.json"
            && manifest.review_record_ref == "review-record.json"
            && manifest.remediation_plan_ref == "remediation-plan.json"
            && manifest.synthesis_digest == plan.synthesis_digest
            && manifest.synthesis_manifest_digest == hash(&synthesis_manifest)?
            && manifest.review_record_digest == hash(&review)?
            && manifest.remediation_plan_digest == hash(&plan)?
            && manifest.action_count == plan.actions.len()
            && manifest.omitted_finding_count == plan.omitted_findings.len()
            && remediation::plan(&synthesis, &review)? == plan,
        "remediation_snapshot_digest_or_canonicality_mismatch"
    );
    Ok(plan)
}

pub(super) fn read_test_plan_from_snapshot(
    artifacts: &[VerifiedArtifact],
    selected: &Path,
) -> Result<TestPlan> {
    ensure!(
        selected.file_name().and_then(|name| name.to_str()) == Some("test-plan.json"),
        "test_plan_bundle_requires_canonical_plan_ref"
    );
    let plan: TestPlan = snapshot_json(artifacts, selected, "test_plan_snapshot_invalid")?;
    let manifest: TestPlanManifest = snapshot_json(
        artifacts,
        &bundle_path(selected, "manifest.json")?,
        "test_plan_manifest_snapshot_invalid",
    )?;
    let synthesis: ReviewSynthesis = snapshot_json(
        artifacts,
        &bundle_path(selected, "synthesis.json")?,
        "test_plan_synthesis_snapshot_invalid",
    )?;
    let synthesis_manifest: SynthesisManifest = snapshot_json(
        artifacts,
        &bundle_path(selected, "synthesis-manifest.json")?,
        "test_plan_synthesis_manifest_snapshot_invalid",
    )?;
    let review: ReviewRecord = snapshot_json(
        artifacts,
        &bundle_path(selected, "review-record.json")?,
        "test_plan_review_snapshot_invalid",
    )?;
    validate_synthesis_snapshot(&synthesis_manifest, &synthesis, &review)?;
    ensure!(
        manifest.schema == TEST_PLAN_MANIFEST_SCHEMA
            && manifest.synthesis_manifest_ref == "synthesis-manifest.json"
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.review_record_ref == "review-record.json"
            && manifest.test_plan_ref == "test-plan.json"
            && manifest.synthesis_manifest_digest == hash(&synthesis_manifest)?
            && manifest.synthesis_digest == hash(&synthesis)?
            && manifest.review_record_digest == hash(&review)?
            && manifest.test_plan_digest == hash(&plan)?
            && manifest.test_case_count == plan.test_cases.len()
            && manifest.omitted_finding_count == plan.omitted_findings.len()
            && test_plan::plan(&synthesis)? == plan,
        "test_plan_snapshot_digest_or_canonicality_mismatch"
    );
    Ok(plan)
}

pub(super) fn validate_source_identity(
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

pub(super) fn validate_plan_parity(
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

pub(crate) fn render_report(
    review: &crate::codefriend::evidence::contracts::ReviewRecord,
    synthesis: &ReviewSynthesis,
    remediation: &RemediationPlan,
    tests: &TestPlan,
    decision: &super::approval::DecisionRecord,
    renderer_version: &str,
    output_boundary: &str,
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
    field(&mut out, "Renderer", renderer_version)?;
    list(&mut out, "Claims", &publication.claims)?;
    list(&mut out, "Nonclaims", &publication.nonclaims)?;

    if let Some(set) = &review.run.assessment_set {
        out.push_str("\n## Review assessments\n\n");
        let counts = set.counts();
        field(
            &mut out,
            "Defect candidates",
            &counts.defect_candidates.to_string(),
        )?;
        field(
            &mut out,
            "Positive observations",
            &counts.positive_observations.to_string(),
        )?;
        field(
            &mut out,
            "Unresolved questions",
            &counts.unresolved_questions.to_string(),
        )?;
        out.push_str("\nOnly defect candidates enter repair and test proposals. A completed review with no defect candidates does not establish that the source is defect-free. Exact citations establish source location, not semantic correctness.\n");
        for assessment in &set.assessments {
            out.push_str("\n### ");
            out.push_str(&markdown_text(&assessment.summary)?);
            out.push_str("\n\n");
            field(&mut out, "Assessment", &assessment.id)?;
            field(&mut out, "Classification", match assessment.kind {
                crate::codefriend::evidence::assessments::AssessmentKind::DefectCandidate => "Defect candidate",
                crate::codefriend::evidence::assessments::AssessmentKind::PositiveObservation => "Positive observation",
                crate::codefriend::evidence::assessments::AssessmentKind::UnresolvedQuestion => "Unresolved question",
            })?;
            field(&mut out, "Perspective", &assessment.lane)?;
            field(&mut out, "Explanation", &assessment.explanation)?;
            if let Some(defect) = &assessment.defect {
                field(&mut out, "Severity", &format!("{:?}", defect.severity))?;
                field(&mut out, "Observed behavior", &defect.observed_behavior)?;
                field(&mut out, "Expected behavior", &defect.expected_behavior)?;
                field(&mut out, "Concrete trigger", &defect.concrete_trigger)?;
                field(&mut out, "Impact", &defect.impact)?;
                field(
                    &mut out,
                    "Proposed remedy or verification",
                    &defect.proposed_remedy_or_verification,
                )?;
            }
            list(&mut out, "Limitations", &assessment.limitations)?;
            for citation in &assessment.citations {
                let item = evidence
                    .get(citation.evidence_id.as_str())
                    .ok_or_else(|| anyhow::anyhow!("assessment_evidence_missing"))?;
                field(
                    &mut out,
                    "Source location",
                    &format!(
                        "{} bytes [{}..{})",
                        item.path, citation.start_byte, citation.end_byte
                    ),
                )?;
                field(
                    &mut out,
                    "Exact source excerpt",
                    citation.quote(&review.admission)?,
                )?;
            }
        }
    }
    out.push_str("\n## Findings\n\n");
    if synthesis.synthesized_findings.is_empty() {
        out.push_str(if review.run.assessment_generation() { "No defect candidates were reported. Positive observations, unresolved questions and coverage gaps remain above; this is not a defect-free certification.\n" } else { "No findings were reported by the completed four-perspective review.\n" });
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
    out.push_str(output_boundary);
    out.push('\n');
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
        out.push_str("- ");
        out.push_str(&markdown_code_span(&item.path)?);
        out.push_str(" at source object ");
        out.push_str(&markdown_code_span(&item.source_object)?);
        out.push_str("; evidence ");
        out.push_str(&markdown_code_span(&item.id)?);
        out.push_str("; content ");
        out.push_str(&markdown_code_span(&item.content_digest)?);
        out.push('\n');
    }
    out.push_str("\n**Attributed source assessments**\n\n");
    for source in &finding.sources {
        out.push_str("- **");
        out.push_str(&markdown_text(&source.perspective)?);
        out.push_str(" / ");
        out.push_str(&markdown_text(&source.rule)?);
        out.push_str("** — ");
        out.push_str(&markdown_text(&source.rationale)?);
        out.push_str(" Finding: ");
        out.push_str(&markdown_text(&source.finding_id)?);
        out.push_str(". Severity: ");
        out.push_str(&markdown_text(
            &format!("{:?}", source.severity).to_ascii_lowercase(),
        )?);
        out.push_str(" Confidence: ");
        out.push_str(&markdown_text(&format!("{:?}", source.confidence))?);
        out.push_str(". Inference: ");
        out.push_str(&markdown_text(&source.inference)?);
        if !source.evidence.is_empty() {
            out.push_str(". Evidence: ");
            out.push_str(&markdown_text(&source.evidence.join("; "))?);
        }
        if !source.limitations.is_empty() {
            out.push_str(". Limitations: ");
            out.push_str(&markdown_text(&source.limitations.join("; "))?);
        }
        out.push_str(".\n");
    }
    out.push_str("\n**Remediation plan**\n\n");
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
    out.push_str("\n**Test plan**\n\n");
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

fn markdown_code_span(value: &str) -> Result<String> {
    ensure!(
        !value.is_empty() && value.len() <= 64 * 1024,
        "markdown_code_empty_or_too_large"
    );
    ensure!(
        !unsafe_content("", value),
        "markdown_code_failed_redaction_recheck"
    );
    let value = value.replace(['\n', '\r', '\t'], " ");
    ensure!(
        value.chars().all(|character| !character.is_control()),
        "markdown_code_contains_control_character"
    );
    let longest_run = value
        .split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or(0);
    let fence = "`".repeat(longest_run + 1);
    Ok(format!("{fence} {value} {fence}"))
}

pub(crate) fn validate_manifest(
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

pub(super) fn normalized_path(path: &Path) -> Result<PathBuf> {
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

pub(super) fn publish_create_only_anchored(
    destination_root: &Path,
    target: &Path,
    artifact_name: &str,
    report: &[u8],
    manifest: &[u8],
    artifact_limit: u64,
    stage_kind: &str,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let target_name = target
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("markdown_target_name_missing"))?;
    let relative_parent = target.parent().unwrap_or_else(|| Path::new(""));
    let parent = open_anchored_parent(destination_root, relative_parent)?;
    ensure!(
        same_open_directory(&parent, &destination_root.join(relative_parent))?,
        "markdown_target_parent_identity_changed"
    );

    let stage_name = create_stage_at(&parent, stage_kind)?;
    let stage = open_directory_at(&parent, &stage_name)?;
    let result = (|| -> Result<(Vec<u8>, Vec<u8>)> {
        write_create_only_at(&stage, artifact_name, report)?;
        write_create_only_at(&stage, "manifest.json", manifest)?;
        stage.sync_all()?;
        rename_create_only_at(&parent, &stage_name, target_name)?;
        parent.sync_all()?;
        ensure!(
            same_open_directory(&parent, &destination_root.join(relative_parent))?,
            "markdown_target_parent_identity_changed"
        );
        let committed = open_directory_at(&parent, target_name)?;
        let actual_report = read_limited_at(&committed, artifact_name, artifact_limit)?;
        let actual_manifest = read_limited_at(&committed, "manifest.json", 2 * 1024 * 1024)?;
        Ok((actual_report, actual_manifest))
    })();
    if result.is_err() {
        cleanup_stage_at(&parent, &stage_name, &stage, artifact_name);
    }
    result
}

fn open_anchored_parent(destination_root: &Path, relative_parent: &Path) -> Result<File> {
    let mut current = open_directory_path(destination_root)?;
    for component in relative_parent.components() {
        let std::path::Component::Normal(name) = component else {
            anyhow::bail!("markdown_target_parent_invalid");
        };
        match open_directory_at(&current, name) {
            Ok(next) => current = next,
            Err(error) if io_not_found(&error) => {
                mkdir_at(&current, name)?;
                current = open_directory_at(&current, name)?;
            }
            Err(error) => return Err(error),
        }
    }
    Ok(current)
}

fn open_directory_path(path: &Path) -> Result<File> {
    let path = c_path(path)?;
    // SAFETY: `path` is a live NUL-terminated CString and the returned fd is
    // immediately owned by `File` on success.
    let fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW,
        )
    };
    ensure!(fd >= 0, "markdown_secure_directory_open_failed");
    // SAFETY: `fd` was just returned by `open` and has a single owner.
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn open_directory_at(parent: &File, name: impl AsRef<std::ffi::OsStr>) -> Result<File> {
    let name = c_name(name.as_ref())?;
    // SAFETY: `name` is NUL terminated and `parent` remains open for the call.
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: `fd` was just returned by `openat` and has a single owner.
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn mkdir_at(parent: &File, name: &std::ffi::OsStr) -> Result<()> {
    let name = c_name(name)?;
    // SAFETY: `name` is NUL terminated and `parent` remains open for the call.
    let status = unsafe { libc::mkdirat(parent.as_raw_fd(), name.as_ptr(), 0o700) };
    if status == 0 {
        return Ok(());
    }
    let error = std::io::Error::last_os_error();
    if error.kind() == std::io::ErrorKind::AlreadyExists {
        return Ok(());
    }
    Err(error.into())
}

fn create_stage_at(parent: &File, stage_kind: &str) -> Result<std::ffi::OsString> {
    ensure!(
        !stage_kind.is_empty()
            && stage_kind
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b'-'),
        "markdown_stage_kind_invalid"
    );
    loop {
        let serial = NEXT_STAGE.fetch_add(1, Ordering::Relaxed);
        let name = std::ffi::OsString::from(format!(
            ".codefriend-{stage_kind}-stage-{}-{serial}",
            std::process::id(),
        ));
        let c_name = c_name(&name)?;
        // SAFETY: `c_name` is NUL terminated and `parent` remains open.
        let status = unsafe { libc::mkdirat(parent.as_raw_fd(), c_name.as_ptr(), 0o700) };
        if status == 0 {
            return Ok(name);
        }
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(error.into());
        }
    }
}

fn write_create_only_at(parent: &File, name: &str, bytes: &[u8]) -> Result<()> {
    let name = c_name(std::ffi::OsStr::new(name))?;
    // SAFETY: `name` is NUL terminated and `parent` remains open for the call.
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_WRONLY | libc::O_CLOEXEC | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW,
            0o600,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: `fd` was just returned by `openat` and has a single owner.
    let mut file = unsafe { File::from_raw_fd(fd) };
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

fn read_limited_at(parent: &File, name: &str, limit: u64) -> Result<Vec<u8>> {
    let name = c_name(std::ffi::OsStr::new(name))?;
    // SAFETY: `name` is NUL terminated and `parent` remains open for the call.
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    // SAFETY: `fd` was just returned by `openat` and has a single owner.
    let file = unsafe { File::from_raw_fd(fd) };
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "markdown_readback_too_large");
    Ok(bytes)
}

fn same_open_directory(open: &File, path: &Path) -> Result<bool> {
    let current = match open_directory_path(path) {
        Ok(file) => file,
        Err(_) => return Ok(false),
    };
    let left = open.metadata()?;
    let right = current.metadata()?;
    Ok(left.dev() == right.dev() && left.ino() == right.ino())
}

#[cfg(target_os = "linux")]
pub(crate) fn rename_create_only_at(
    parent: &File,
    from: &std::ffi::OsStr,
    to: &std::ffi::OsStr,
) -> Result<()> {
    let from = c_name(from)?;
    let to = c_name(to)?;
    // SAFETY: both names are NUL terminated and `parent` remains open.
    let status = unsafe {
        libc::renameat2(
            parent.as_raw_fd(),
            from.as_ptr(),
            parent.as_raw_fd(),
            to.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    rename_status(status)
}

#[cfg(target_os = "macos")]
pub(crate) fn rename_create_only_at(
    parent: &File,
    from: &std::ffi::OsStr,
    to: &std::ffi::OsStr,
) -> Result<()> {
    let from = c_name(from)?;
    let to = c_name(to)?;
    // SAFETY: both names are NUL terminated and `parent` remains open.
    let status = unsafe {
        libc::renameatx_np(
            parent.as_raw_fd(),
            from.as_ptr(),
            parent.as_raw_fd(),
            to.as_ptr(),
            libc::RENAME_EXCL,
        )
    };
    rename_status(status)
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub(crate) fn rename_create_only_at(
    _parent: &File,
    _from: &std::ffi::OsStr,
    _to: &std::ffi::OsStr,
) -> Result<()> {
    anyhow::bail!("markdown_secure_create_only_unsupported_platform")
}

fn rename_status(status: libc::c_int) -> Result<()> {
    if status == 0 {
        return Ok(());
    }
    let error = std::io::Error::last_os_error();
    if error.kind() == std::io::ErrorKind::AlreadyExists {
        anyhow::bail!("markdown_output_target_already_exists");
    }
    Err(error.into())
}

fn cleanup_stage_at(
    parent: &File,
    stage_name: &std::ffi::OsStr,
    stage: &File,
    artifact_name: &str,
) {
    for name in [artifact_name, "manifest.json"] {
        if let Ok(name) = c_name(std::ffi::OsStr::new(name)) {
            // SAFETY: `name` is NUL terminated and `stage` remains open.
            unsafe {
                libc::unlinkat(stage.as_raw_fd(), name.as_ptr(), 0);
            }
        }
    }
    if let Ok(stage_name) = c_name(stage_name) {
        // SAFETY: `stage_name` is NUL terminated and `parent` remains open.
        unsafe {
            libc::unlinkat(parent.as_raw_fd(), stage_name.as_ptr(), libc::AT_REMOVEDIR);
        }
    }
}

fn c_path(path: &Path) -> Result<CString> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|_| anyhow::anyhow!("markdown_path_contains_nul"))
}

fn c_name(name: &std::ffi::OsStr) -> Result<CString> {
    ensure!(
        !name.as_bytes().contains(&b'/'),
        "markdown_component_contains_separator"
    );
    CString::new(name.as_bytes()).map_err(|_| anyhow::anyhow!("markdown_path_contains_nul"))
}

fn io_not_found(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_spans_keep_backtick_paths_non_clickable() {
        let value = "src/odd` [click](https://example.invalid) <em>name</em>.rs";
        let span = markdown_code_span(value).unwrap();
        let html = markdown::to_html(&format!("- {span}\n"));
        assert!(html.contains("<code>"), "{html}");
        assert!(!html.contains("<a "), "{html}");
        assert!(!html.contains("<em>"), "{html}");
        assert!(html.contains("[click](https://example.invalid)"), "{html}");
    }

    #[test]
    fn verified_snapshot_parse_ignores_post_snapshot_path_swap() {
        let target_tmp = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tmp");
        fs::create_dir_all(&target_tmp).unwrap();
        let root = tempfile::tempdir_in(target_tmp).unwrap();
        let path = root.path().join("bundle.json");
        let approved = br#"{"value":"approved"}"#;
        fs::write(&path, approved).unwrap();
        let manifest = vec![crate::codefriend::evidence::contracts::Artifact {
            path: "bundle.json".to_string(),
            digest: digest(approved),
        }];

        let snapshot = snapshot_artifacts(root.path(), &manifest).unwrap();
        fs::write(&path, br#"{"value":"swapped"}"#).unwrap();

        let parsed: serde_json::Value =
            snapshot_json(&snapshot, Path::new("bundle.json"), "snapshot_json_invalid").unwrap();
        assert_eq!(parsed["value"], "approved");
        assert_eq!(fs::read_to_string(path).unwrap(), r#"{"value":"swapped"}"#);
    }

    #[test]
    fn anchored_commit_refuses_a_competing_target_without_replacement() {
        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("report");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("owner"), b"competitor").unwrap();

        let error = publish_create_only_anchored(
            root.path(),
            Path::new("report"),
            "report.md",
            b"review",
            br#"{"manifest":true}"#,
            MAX_RENDERED_BYTES as u64,
            "markdown",
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("markdown_output_target_already_exists"),
            "{error}"
        );
        assert_eq!(fs::read(target.join("owner")).unwrap(), b"competitor");
        assert!(!target.join("report.md").exists());
        assert!(fs::read_dir(root.path()).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".codefriend-markdown-stage-")));
    }

    #[test]
    fn anchored_parent_handle_cannot_be_redirected_by_path_swap() {
        let root = tempfile::tempdir().unwrap();
        let visible = root.path().join("visible");
        fs::create_dir(&visible).unwrap();
        let anchored = open_anchored_parent(root.path(), Path::new("visible")).unwrap();
        let stage_name = create_stage_at(&anchored, "markdown").unwrap();
        let stage = open_directory_at(&anchored, &stage_name).unwrap();
        write_create_only_at(&stage, "report.md", b"anchored").unwrap();

        let original = root.path().join("original");
        fs::rename(&visible, &original).unwrap();
        fs::create_dir(&visible).unwrap();
        assert!(!same_open_directory(&anchored, &visible).unwrap());

        rename_create_only_at(&anchored, &stage_name, std::ffi::OsStr::new("report")).unwrap();
        assert_eq!(
            fs::read(original.join("report/report.md")).unwrap(),
            b"anchored"
        );
        assert!(!visible.join("report").exists());
    }
}
