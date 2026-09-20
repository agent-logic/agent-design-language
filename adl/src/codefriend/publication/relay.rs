//! Native local publication receipt verification. No provider/renderer execution.
use super::*;
use crate::codefriend::{
    agent::publication::{Binding, Decision, Prepared},
    evidence::{contracts::ReviewRecord, hash, valid_digest},
    integration::PublicationFormat,
};
use anyhow::{ensure, Result};
use serde::Serialize;
use serde_json::Value;
fn target(f: PublicationFormat) -> &'static str {
    match f {
        PublicationFormat::Markdown => "report-md",
        PublicationFormat::Html => "report-html",
        PublicationFormat::Pdf => "report-pdf",
    }
}

pub(crate) fn verify_prepared(
    p: &Prepared,
    b: &Binding,
    candidate: &str,
    review: &ReviewRecord,
    now: u64,
) -> Result<()> {
    let c = serde_json::to_value(&p.native.challenge)?;
    ensure!(
        c["schema"] == "codefriend.publication_challenge.v1"
            && c["subject"] == b.subject
            && c["operation"] == b.job_id
            && c["candidate_revision"] == candidate
            && c["digest"] == p.challenge_digest
            && c["expected_head"] == serde_json::to_value(&p.expected_decision_digest)?
            && c["issued_at"] == p.issued_at
            && c["expires_at"] == p.expires_at,
        "relay_challenge_identity"
    );
    ensure!(
        p.issued_at > 0
            && p.issued_at <= now
            && now < p.expires_at
            && p.expires_at <= b.expires_at
            && p.expires_at - p.issued_at <= 300,
        "relay_challenge_time"
    );
    let mut unsigned = c;
    unsigned["digest"] = Value::String(String::new());
    let unsigned: crate::codefriend::integration::PublicationChallenge =
        serde_json::from_value(unsigned)?;
    ensure!(
        hash(&unsigned)? == p.challenge_digest,
        "relay_challenge_digest"
    );
    let publication = p.native.challenge.publication();
    publication.validate(review)?;
    ensure!(
        publication.binding_digest()? == p.binding_digest
            && publication.approval.is_none()
            && publication.target == target(b.format)
            && publication.renderer_versions
                == std::collections::BTreeMap::from([(
                    b.format.key().into(),
                    b.format.renderer_version().into()
                )]),
        "relay_publication_binding"
    );
    ensure!(
        publication.state == crate::codefriend::evidence::contracts::PublicationState::Withheld
            && publication.claims == vec!["Bounded CodeFriend review and action plans".to_string()]
            && publication.nonclaims
                == vec!["No source changes or external publication".to_string()]
            && p.expected_decision_digest.is_none(),
        "relay_prepared_semantics"
    );
    // Rebuild exact artifact bytes through the shared native builder without
    // filesystem destination, approval, or renderer effects.
    let expected = crate::codefriend::integration::publication_artifacts(
        review,
        &serde_json::to_vec(review)?,
    )?;
    ensure!(
        expected.inventory() == publication.artifact_manifest,
        "relay_artifact_inventory"
    );
    Ok(())
}
pub(crate) fn verify_local_decision(
    record: &DecisionRecord,
    d: &Decision,
    p: &Prepared,
    b: &Binding,
    candidate: &str,
    review: &ReviewRecord,
) -> Result<()> {
    record.validate(review)?;
    ensure!(
        record.schema == "codefriend.publication_decision.v3"
            && record.channel == "authenticated_local_agent"
            && record.website.is_none()
            && record.actor == b.subject
            && record.reason == "Authenticated local agent artifact decision"
            && record.decision == d.decision
            && record.previous_decision_digest == d.expected_decision_digest
            && d.expected_decision_digest == p.expected_decision_digest
            && d.challenge_digest == p.challenge_digest
            && d.binding_digest == p.binding_digest
            && record.publication.binding_digest()? == p.binding_digest
            && record.decided_at >= p.issued_at
            && record.decided_at < p.expires_at,
        "relay_local_decision"
    );
    let local = record
        .local_agent
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("relay_local_provenance"))?;
    ensure!(
        local.schema == "codefriend.local_agent_decision.v1"
            && local.subject == b.subject
            && local.agent_id == b.agent_id
            && local.run_id == b.run_id
            && local.job_digest == hash(b)?
            && local.report_digest == b.report_digest
            && local.received_digest == b.received_digest
            && local.consent_digest == b.consent_digest
            && local.format == b.format
            && local.agent_candidate_revision == candidate
            && local.challenge_digest == p.challenge_digest
            && local.publication_binding_digest == p.binding_digest,
        "relay_local_provenance_binding"
    );
    Ok(())
}
fn common<R: Serialize, M: Serialize>(
    result: &R,
    manifest: &M,
    bytes: &[u8],
    decision: &DecisionRecord,
) -> Result<()> {
    let r = serde_json::to_value(result)?;
    let m = serde_json::to_value(manifest)?;
    ensure!(
        r["report_digest"] == crate::codefriend::ingestion::digest(bytes)
            && r["report_digest"] == m["report_digest"]
            && r["manifest_digest"]
                == crate::codefriend::ingestion::digest(&serde_json::to_vec_pretty(manifest)?)
            && r["target"] == m["target"]
            && r["renderer_version"] == m["renderer_version"]
            && r["approval_decision_digest"] == decision.digest,
        "relay_renderer_result_binding"
    );
    Ok(())
}
pub(crate) fn verify_rendered(
    review: &ReviewRecord,
    decision: &DecisionRecord,
    format: PublicationFormat,
    result: &Value,
    manifest: &Value,
    bytes: &[u8],
) -> Result<()> {
    use crate::codefriend::{
        actions::{remediation, test_plan},
        review::synthesis,
    };
    let synthesis = synthesis::synthesize(review)?;
    let remediation = remediation::plan(&synthesis, review)?;
    let tests = test_plan::plan(&synthesis)?;
    let publication = &decision.publication;
    match format {
        PublicationFormat::Markdown => {
            let r: MarkdownRenderResult = serde_json::from_value(result.clone())?;
            let m: MarkdownManifest = serde_json::from_value(manifest.clone())?;
            ensure!(
                r.schema == MARKDOWN_RESULT_SCHEMA
                    && r.finding_count == synthesis.synthesized_findings.len(),
                "relay_markdown_result"
            );
            let expected = markdown::render_report(
                review,
                &synthesis,
                &remediation,
                &tests,
                decision,
                MARKDOWN_RENDERER_VERSION,
                markdown::MARKDOWN_OUTPUT_BOUNDARY,
            )?;
            ensure!(
                bytes == expected.as_bytes(),
                "relay_markdown_content_mismatch"
            );
            markdown::validate_manifest(&m, review, &synthesis, publication, &decision.digest)?;
            ensure!(
                m.remediation_plan_digest == hash(&remediation)?
                    && m.test_plan_digest == hash(&tests)?,
                "relay_markdown_actions"
            );
            common(&r, &m, bytes, decision)?;
        }
        PublicationFormat::Html => {
            let r: HtmlRenderResult = serde_json::from_value(result.clone())?;
            let m: HtmlManifest = serde_json::from_value(manifest.clone())?;
            ensure!(
                r.schema == HTML_RESULT_SCHEMA
                    && r.finding_count == synthesis.synthesized_findings.len(),
                "relay_html_result"
            );
            let expected = html::render_report(review, &synthesis, &remediation, &tests, decision)?;
            ensure!(bytes == expected.as_bytes(), "relay_html_content_mismatch");
            html::validate_manifest(
                &m,
                review,
                &synthesis,
                &remediation,
                &tests,
                publication,
                &decision.digest,
            )?;
            common(&r, &m, bytes, decision)?;
        }
        PublicationFormat::Pdf => {
            let r: PdfRenderResult = serde_json::from_value(result.clone())?;
            let m: PdfManifest = serde_json::from_value(manifest.clone())?;
            ensure!(
                r.schema == PDF_RESULT_SCHEMA
                    && r.finding_count == synthesis.synthesized_findings.len()
                    && r.page_count == m.page_count
                    && bytes.starts_with(b"%PDF-")
                    && valid_digest(&m.font_digest)
                    && valid_digest(&m.semantic_digest),
                "relay_pdf_result"
            );
            let prepared = markdown::PreparedReport {
                review: review.clone(),
                publication: publication.clone(),
                decision: decision.clone(),
                synthesis,
                remediation,
                tests,
                text: String::new(),
            };
            pdf::validate_manifest(&m, &prepared)?;
            common(&r, &m, bytes, decision)?;
        }
    }
    Ok(())
}

/// Recovery metadata only: caller MUST subsequently verify_rendered against
/// owned report, exact decision, and actual export bytes before claiming success.
pub(crate) fn result_from_manifest(format: PublicationFormat, value: &Value) -> Result<Value> {
    Ok(match format {
        PublicationFormat::Markdown => {
            let m: MarkdownManifest = serde_json::from_value(value.clone())?;
            serde_json::to_value(MarkdownRenderResult {
                schema: MARKDOWN_RESULT_SCHEMA.into(),
                target: m.target.clone(),
                renderer_version: m.renderer_version.clone(),
                report_digest: m.report_digest.clone(),
                manifest_digest: crate::codefriend::ingestion::digest(&serde_json::to_vec_pretty(
                    &m,
                )?),
                finding_count: m.finding_ids.len(),
                approval_decision_digest: m.approval_decision_digest.clone(),
            })?
        }
        PublicationFormat::Html => {
            let m: HtmlManifest = serde_json::from_value(value.clone())?;
            serde_json::to_value(HtmlRenderResult {
                schema: HTML_RESULT_SCHEMA.into(),
                target: m.target.clone(),
                renderer_version: m.renderer_version.clone(),
                report_digest: m.report_digest.clone(),
                manifest_digest: crate::codefriend::ingestion::digest(&serde_json::to_vec_pretty(
                    &m,
                )?),
                finding_count: m.finding_ids.len(),
                approval_decision_digest: m.approval_decision_digest.clone(),
            })?
        }
        PublicationFormat::Pdf => {
            let m: PdfManifest = serde_json::from_value(value.clone())?;
            serde_json::to_value(PdfRenderResult {
                schema: PDF_RESULT_SCHEMA.into(),
                target: m.target.clone(),
                renderer_version: m.renderer_version.clone(),
                report_digest: m.report_digest.clone(),
                manifest_digest: crate::codefriend::ingestion::digest(&serde_json::to_vec_pretty(
                    &m,
                )?),
                finding_count: m.finding_ids.len(),
                page_count: m.page_count,
                approval_decision_digest: m.approval_decision_digest.clone(),
            })?
        }
    })
}
