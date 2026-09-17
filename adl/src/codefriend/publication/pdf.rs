//! Deterministic, local PDF export for an approved CodeFriend review.

use super::{
    markdown::{prepare_report, publish_create_only_anchored},
    MarkdownRenderOptions,
};
use crate::codefriend::{evidence::hash, ingestion::digest};
use anyhow::{ensure, Context, Result};
use printpdf::{
    Mm, Op, ParsedFont, PdfDocument, PdfFontHandle, PdfPage, PdfSaveOptions, Point, Pt, TextItem,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::PathBuf};

pub const PDF_RENDERER_VERSION: &str = "v1-printpdf-0.12.8";
pub const PDF_MANIFEST_SCHEMA: &str = "codefriend.pdf_report_manifest.v1";
pub const PDF_RESULT_SCHEMA: &str = "codefriend.pdf_render_result.v1";
const MAX_FONT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_PDF_BYTES: u64 = 64 * 1024 * 1024;
const MAX_PAGES: usize = 2_048;
const LINES_PER_PAGE: usize = 50;
const FONT_SIZE_PT: f32 = 9.5;
const PAGE_WIDTH_MM: f32 = 210.0;
const HORIZONTAL_MARGIN_MM: f32 = 18.0;
const PRINTABLE_WIDTH_MM: f32 = PAGE_WIDTH_MM - (2.0 * HORIZONTAL_MARGIN_MM);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfRenderOptions {
    pub review_record: PathBuf,
    pub publication: PathBuf,
    pub approval_store: PathBuf,
    pub artifact_root: PathBuf,
    pub synthesis: PathBuf,
    pub remediation_plan: PathBuf,
    pub test_plan: PathBuf,
    pub destination_root: PathBuf,
    pub out: PathBuf,
    pub font: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PdfManifest {
    pub schema: String,
    pub renderer_version: String,
    pub renderer_engine: String,
    pub font_digest: String,
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
    pub semantic_digest: String,
    pub page_count: usize,
    pub line_count: usize,
    pub printable_width_micrometers: u32,
    pub maximum_line_width_micrometers: u32,
    pub finding_ids: Vec<String>,
    pub claims: Vec<String>,
    pub nonclaims: Vec<String>,
    pub external_resources: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PdfRenderResult {
    pub schema: String,
    pub target: String,
    pub renderer_version: String,
    pub report_digest: String,
    pub manifest_digest: String,
    pub finding_count: usize,
    pub page_count: usize,
    pub approval_decision_digest: String,
}

pub fn render_pdf(options: PdfRenderOptions) -> Result<PdfRenderResult> {
    ensure!(!options.out.exists(), "pdf_output_target_already_exists");
    ensure!(
        options.destination_root.exists()
            && fs::symlink_metadata(&options.destination_root)?.is_dir(),
        "pdf_destination_root_missing_or_invalid"
    );
    super::manifest::reject_symlink_components(&options.destination_root)?;
    super::manifest::reject_symlink_components(&options.out)?;
    super::manifest::reject_symlink_components(&options.font)?;

    let metadata = fs::symlink_metadata(&options.font).context("pdf_font_metadata_failed")?;
    ensure!(metadata.is_file(), "pdf_font_not_regular_file");
    ensure!(
        metadata.len() <= MAX_FONT_BYTES,
        "pdf_font_byte_limit_exceeded"
    );
    let mut font_bytes = Vec::with_capacity(metadata.len() as usize);
    fs::File::open(&options.font)
        .context("pdf_font_open_failed")?
        .take(MAX_FONT_BYTES + 1)
        .read_to_end(&mut font_bytes)
        .context("pdf_font_read_failed")?;
    ensure!(
        font_bytes.len() as u64 <= MAX_FONT_BYTES,
        "pdf_font_byte_limit_exceeded"
    );

    let shared = MarkdownRenderOptions {
        review_record: options.review_record.clone(),
        publication: options.publication.clone(),
        approval_store: options.approval_store.clone(),
        artifact_root: options.artifact_root.clone(),
        synthesis: options.synthesis.clone(),
        remediation_plan: options.remediation_plan.clone(),
        test_plan: options.test_plan.clone(),
        destination_root: options.destination_root.clone(),
        out: options.out.clone(),
    };
    let prepared = prepare_report(
        &shared,
        "pdf",
        PDF_RENDERER_VERSION,
        "This is the canonical local PDF rendering of the approved review. It does not claim HTML, Markdown, remote, or customer publication.",
    )?;
    ensure!(
        !crate::codefriend::ingestion::unsafe_content("report.pdf", &prepared.text),
        "pdf_redaction_recheck_failed"
    );

    let mut parse_warnings = Vec::new();
    let font = ParsedFont::from_bytes(&font_bytes, 0, &mut parse_warnings)
        .ok_or_else(|| anyhow::anyhow!("pdf_font_parse_failed"))?;
    ensure!(parse_warnings.is_empty(), "pdf_font_parse_warning");
    for character in prepared.text.chars().filter(|character| {
        !character.is_whitespace() && !matches!(character, '\u{00ad}' | '\u{feff}')
    }) {
        ensure!(
            font.lookup_glyph_index(character as u32).is_some(),
            "pdf_font_missing_glyph_u{:04x}",
            character as u32
        );
    }

    let lines = wrap_text(&prepared.text, &font, PRINTABLE_WIDTH_MM)?;
    ensure!(!lines.is_empty(), "pdf_empty_semantic_report");
    let maximum_line_width_mm = lines
        .iter()
        .map(|line| text_width_mm(line, &font))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(0.0_f32, f32::max);
    ensure!(
        maximum_line_width_mm <= PRINTABLE_WIDTH_MM,
        "pdf_line_exceeds_printable_width"
    );
    let page_count = lines.len().div_ceil(LINES_PER_PAGE);
    ensure!(page_count <= MAX_PAGES, "pdf_page_limit_exceeded");
    let pdf_bytes = build_pdf(&lines, font)?;
    ensure!(
        pdf_bytes.starts_with(b"%PDF-") && pdf_bytes.len() > 1_024,
        "pdf_output_invalid_or_empty"
    );
    ensure!(
        pdf_bytes.len() as u64 <= MAX_PDF_BYTES,
        "pdf_output_byte_limit_exceeded"
    );

    let report_digest = digest(&pdf_bytes);
    let font_digest = digest(&font_bytes);
    let finding_ids = prepared
        .synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    let manifest = PdfManifest {
        schema: PDF_MANIFEST_SCHEMA.to_string(),
        renderer_version: PDF_RENDERER_VERSION.to_string(),
        renderer_engine: "printpdf@0.12.8; layout=codefriend-pdf-v1".to_string(),
        font_digest,
        review_record_digest: hash(&prepared.review)?,
        run_digest: hash(&prepared.review.run)?,
        finding_set_digest: prepared.review.finding_digest()?,
        synthesis_digest: hash(&prepared.synthesis)?,
        remediation_plan_digest: hash(&prepared.remediation)?,
        test_plan_digest: hash(&prepared.tests)?,
        publication_binding_digest: prepared.publication.binding_digest()?,
        approval_decision_digest: prepared.decision.digest.clone(),
        repository: prepared.review.run.repository.clone(),
        revision: prepared.review.run.revision.clone(),
        scope_digest: prepared.review.run.scope_digest.clone(),
        target: prepared.publication.target.clone(),
        report_path: "report.pdf".to_string(),
        report_digest: report_digest.clone(),
        semantic_digest: digest(prepared.text.as_bytes()),
        page_count,
        line_count: lines.len(),
        printable_width_micrometers: (PRINTABLE_WIDTH_MM * 1_000.0).round() as u32,
        maximum_line_width_micrometers: (maximum_line_width_mm * 1_000.0).round() as u32,
        finding_ids,
        claims: prepared.publication.claims.clone(),
        nonclaims: prepared.publication.nonclaims.clone(),
        external_resources: false,
    };
    validate_manifest(&manifest, &prepared)?;
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
    ensure!(
        !crate::codefriend::ingestion::unsafe_content(
            "manifest.json",
            std::str::from_utf8(&manifest_bytes)?
        ),
        "pdf_manifest_redaction_recheck_failed"
    );

    let (actual_pdf, actual_manifest) = publish_create_only_anchored(
        &options.destination_root,
        std::path::Path::new(&prepared.publication.target),
        "report.pdf",
        &pdf_bytes,
        &manifest_bytes,
        MAX_PDF_BYTES,
        "pdf",
    )?;
    ensure!(actual_pdf == pdf_bytes, "pdf_report_readback_mismatch");
    ensure!(
        digest(&actual_pdf) == manifest.report_digest,
        "pdf_report_digest_mismatch"
    );
    let read_manifest: PdfManifest =
        serde_json::from_slice(&actual_manifest).context("pdf_manifest_readback_invalid")?;
    ensure!(read_manifest == manifest, "pdf_manifest_readback_mismatch");

    Ok(PdfRenderResult {
        schema: PDF_RESULT_SCHEMA.to_string(),
        target: prepared.publication.target,
        renderer_version: PDF_RENDERER_VERSION.to_string(),
        report_digest,
        manifest_digest: digest(&actual_manifest),
        finding_count: prepared.synthesis.synthesized_findings.len(),
        page_count,
        approval_decision_digest: prepared.decision.digest,
    })
}

fn build_pdf(lines: &[String], font: ParsedFont) -> Result<Vec<u8>> {
    let mut document = PdfDocument::new("CodeFriend approved review");
    let font_id = document.add_font(&font);
    let pages = lines
        .chunks(LINES_PER_PAGE)
        .map(|page_lines| {
            let mut operations = vec![
                Op::StartTextSection,
                Op::SetTextCursor {
                    pos: Point::new(Mm(18.0), Mm(280.0)),
                },
                Op::SetFont {
                    font: PdfFontHandle::External(font_id.clone()),
                    size: Pt(FONT_SIZE_PT),
                },
                Op::SetLineHeight { lh: Pt(14.0) },
            ];
            for (index, line) in page_lines.iter().enumerate() {
                if index > 0 {
                    operations.push(Op::AddLineBreak);
                }
                operations.push(Op::ShowText {
                    items: vec![TextItem::Text(if line.is_empty() {
                        " ".to_string()
                    } else {
                        line.clone()
                    })],
                });
            }
            operations.push(Op::EndTextSection);
            PdfPage::new(Mm(210.0), Mm(297.0), operations)
        })
        .collect::<Vec<_>>();
    let mut warnings = Vec::new();
    let bytes = document
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut warnings);
    ensure!(warnings.is_empty(), "pdf_render_warning");
    Ok(bytes)
}

fn glyph_width_mm(character: char, font: &ParsedFont) -> Result<f32> {
    let glyph = font
        .lookup_glyph_index(character as u32)
        .ok_or_else(|| anyhow::anyhow!("pdf_font_missing_glyph_u{:04x}", character as u32))?;
    let width = font
        .get_glyph_width(glyph)
        .ok_or_else(|| anyhow::anyhow!("pdf_font_missing_glyph_width_u{:04x}", character as u32))?;
    ensure!(font.units_per_em > 0, "pdf_font_invalid_units_per_em");
    Ok(width as f32 / font.units_per_em as f32 * FONT_SIZE_PT * 25.4 / 72.0)
}

fn text_width_mm(text: &str, font: &ParsedFont) -> Result<f32> {
    text.chars().try_fold(0.0_f32, |width, character| {
        Ok(width + glyph_width_mm(character, font)?)
    })
}

fn wrap_text(text: &str, font: &ParsedFont, max_width_mm: f32) -> Result<Vec<String>> {
    ensure!(max_width_mm > 0.0, "pdf_printable_width_invalid");
    let mut lines = Vec::new();
    for source_line in text.lines() {
        if source_line.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for word in source_line.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_string()
            } else {
                format!("{current} {word}")
            };
            if text_width_mm(&candidate, font)? <= max_width_mm {
                current = candidate;
                continue;
            }
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
            for character in word.chars() {
                let mut candidate = current.clone();
                candidate.push(character);
                if !current.is_empty() && text_width_mm(&candidate, font)? > max_width_mm {
                    lines.push(std::mem::take(&mut current));
                    candidate = character.to_string();
                }
                ensure!(
                    text_width_mm(&candidate, font)? <= max_width_mm,
                    "pdf_glyph_exceeds_printable_width_u{:04x}",
                    character as u32
                );
                current = candidate;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    Ok(lines)
}

fn validate_manifest(
    manifest: &PdfManifest,
    prepared: &super::markdown::PreparedReport,
) -> Result<()> {
    ensure!(
        manifest.schema == PDF_MANIFEST_SCHEMA
            && manifest.renderer_version == PDF_RENDERER_VERSION
            && manifest.renderer_engine == "printpdf@0.12.8; layout=codefriend-pdf-v1"
            && manifest.review_record_digest == hash(&prepared.review)?
            && manifest.run_digest == hash(&prepared.review.run)?
            && manifest.finding_set_digest == prepared.review.finding_digest()?
            && manifest.synthesis_digest == hash(&prepared.synthesis)?
            && manifest.remediation_plan_digest == hash(&prepared.remediation)?
            && manifest.test_plan_digest == hash(&prepared.tests)?
            && manifest.publication_binding_digest == prepared.publication.binding_digest()?
            && manifest.approval_decision_digest == prepared.decision.digest
            && manifest.repository == prepared.review.run.repository
            && manifest.revision == prepared.review.run.revision
            && manifest.scope_digest == prepared.review.run.scope_digest
            && manifest.target == prepared.publication.target
            && manifest.report_path == "report.pdf"
            && manifest.page_count > 0
            && manifest.line_count > 0
            && manifest.printable_width_micrometers
                == (PRINTABLE_WIDTH_MM * 1_000.0).round() as u32
            && manifest.maximum_line_width_micrometers <= manifest.printable_width_micrometers
            && manifest.claims == prepared.publication.claims
            && manifest.nonclaims == prepared.publication.nonclaims
            && !manifest.external_resources,
        "invalid_pdf_manifest_binding"
    );
    let expected_ids = prepared
        .synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    ensure!(
        manifest.finding_ids == expected_ids,
        "pdf_manifest_finding_parity_mismatch"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::wrap_text;

    #[test]
    fn wrapping_preserves_all_unicode_and_splits_long_tokens() {
        let input = "Résumé café π\nhttps://example.invalid/abcdefghijklmnopqrstuvwxyz";
        let lines = wrap_text(input, 12);
        assert!(lines.iter().all(|line| line.chars().count() <= 12));
        assert_eq!(
            lines.concat().replace(' ', ""),
            input.replace([' ', '\n'], "")
        );
    }
}
