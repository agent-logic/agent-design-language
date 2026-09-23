//! Deterministic, local PDF export for an approved CodeFriend review.

use super::{
    markdown::{prepare_report, publish_with_attachments},
    MarkdownRenderOptions,
};
use crate::codefriend::{evidence::hash, ingestion::digest};
use anyhow::{ensure, Context, Result};
use lopdf::{Document as LopdfDocument, Object as LopdfObject};
use printpdf::{
    Mm, Op, ParsedFont, PdfDocument, PdfFontHandle, PdfPage, PdfSaveOptions, Point, Pt, TextItem,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::PathBuf};

pub const PDF_RENDERER_VERSION: &str = "v2-printpdf-0.12.8";
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
const PDF_LINE_WRAP_TAG: &str = "ADLVisualWrap";
const PDF_LINE_END_TAG: &str = "ADLLogicalEnd";
const PDF_EMPTY_LINE_TAG: &str = "ADLLogicalEmpty";

#[derive(Debug, Clone)]
struct PdfLayoutLine {
    text: String,
    tag: &'static str,
}

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
    let semantic_text = markdown_semantic_text(&prepared.text)?;
    ensure!(
        !crate::codefriend::ingestion::unsafe_content("report.pdf", &semantic_text),
        "pdf_redaction_recheck_failed"
    );

    let mut parse_warnings = Vec::new();
    let font = ParsedFont::from_bytes(&font_bytes, 0, &mut parse_warnings)
        .ok_or_else(|| anyhow::anyhow!("pdf_font_parse_failed"))?;
    ensure!(parse_warnings.is_empty(), "pdf_font_parse_warning");
    for character in semantic_text.chars().filter(|character| {
        !character.is_whitespace() && !matches!(character, '\u{00ad}' | '\u{feff}')
    }) {
        ensure!(
            font.lookup_glyph_index(character as u32).is_some(),
            "pdf_font_missing_glyph_u{:04x}",
            character as u32
        );
    }

    let lines = layout_text(&semantic_text, &font, PRINTABLE_WIDTH_MM)?;
    ensure!(!lines.is_empty(), "pdf_empty_semantic_report");
    let maximum_line_width_mm = lines
        .iter()
        .map(|line| text_width_mm(&line.text, &font))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(0.0_f32, f32::max);
    ensure!(
        maximum_line_width_mm <= PRINTABLE_WIDTH_MM,
        "pdf_line_exceeds_printable_width"
    );
    let page_count = lines.len().div_ceil(LINES_PER_PAGE)
        + prepared
            .architecture
            .keys()
            .filter(|name| name.ends_with(".svg"))
            .count();
    ensure!(page_count <= MAX_PAGES, "pdf_page_limit_exceeded");
    let pdf_bytes = build_pdf(&lines, font, &prepared.architecture)?;
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
        renderer_engine: "printpdf@0.12.8; layout=codefriend-pdf-v2".to_string(),
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
        semantic_digest: digest(semantic_text.as_bytes()),
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

    let (actual_pdf, actual_manifest) = publish_with_attachments(
        &options.destination_root,
        std::path::Path::new(&prepared.publication.target),
        "report.pdf",
        &pdf_bytes,
        &manifest_bytes,
        MAX_PDF_BYTES,
        "pdf",
        &super::architecture::attachments(&prepared.architecture)?,
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

fn build_pdf(
    lines: &[PdfLayoutLine],
    font: ParsedFont,
    architecture: &std::collections::BTreeMap<String, Vec<u8>>,
) -> Result<Vec<u8>> {
    let mut document = PdfDocument::new("CodeFriend approved review");
    let font_id = document.add_font(&font);
    let mut pages = lines
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
                operations.push(Op::BeginMarkedContent {
                    tag: line.tag.to_string(),
                });
                operations.push(Op::ShowText {
                    items: vec![TextItem::Text(if line.text.is_empty() {
                        " ".to_string()
                    } else {
                        line.text.clone()
                    })],
                });
                operations.push(Op::EndMarkedContent);
            }
            operations.push(Op::EndTextSection);
            PdfPage::new(Mm(210.0), Mm(297.0), operations)
        })
        .collect::<Vec<_>>();
    if let Some(bytes) = architecture.get("package.json") {
        use crate::codefriend::architecture::four_plus_one::Package;
        let package: Package = serde_json::from_slice(bytes)?;
        let mut rendered = std::collections::BTreeSet::new();
        for (view_name, view) in &package.views {
            let key = serde_json::to_value(view_name)?
                .as_str()
                .unwrap()
                .to_owned();
            let entity = |id: &str| {
                package
                    .entities
                    .iter()
                    .find(|e| e.id == id)
                    .expect("validated entity")
            };
            for (i, relationship) in view.relationships.iter().enumerate() {
                let name = format!("{key}-{i:03}.svg");
                let labels = [
                    &entity(&relationship.from).name[..],
                    &entity(&relationship.to).name[..],
                ];
                pages.push(diagram_page(&name, &labels, &font, &font_id)?);
                rendered.insert(name);
            }
            for (i, id) in view.entities.iter().enumerate() {
                if view
                    .relationships
                    .iter()
                    .any(|r| &r.from == id || &r.to == id)
                {
                    continue;
                }
                let name = format!("{key}-entity-{i:03}.svg");
                pages.push(diagram_page(&name, &[&entity(id).name], &font, &font_id)?);
                rendered.insert(name);
            }
        }
        ensure!(
            rendered
                == architecture
                    .keys()
                    .filter(|n| n.ends_with(".svg"))
                    .cloned()
                    .collect(),
            "architecture_pdf_diagram_parity"
        );
    }
    let mut warnings = Vec::new();
    let bytes = document
        .with_pages(pages)
        .save(&PdfSaveOptions::default(), &mut warnings);
    ensure!(warnings.is_empty(), "pdf_render_warning");
    Ok(bytes)
}

// Draw the same approved entities and directed relationships with the existing
// PDF vector primitives. No SVG parser, external resources or font lookup.
fn diagram_page(
    name: &str,
    labels: &[&str],
    font: &ParsedFont,
    font_id: &printpdf::FontId,
) -> Result<PdfPage> {
    let mut operations = Vec::new();
    let mut text_at = |label: &str, x: f32, y: f32| {
        operations.extend([
            Op::StartTextSection,
            Op::SetTextCursor {
                pos: Point::new(Mm(x), Mm(y)),
            },
            Op::SetFont {
                font: PdfFontHandle::External(font_id.clone()),
                size: Pt(FONT_SIZE_PT),
            },
            Op::ShowText {
                items: vec![TextItem::Text(label.into())],
            },
            Op::EndTextSection,
        ]);
    };
    text_at(&format!("Architecture diagram: {name}"), 18.0, 280.0);
    let wrapped = labels
        .iter()
        .map(|label| wrap_text(label, font, 66.0))
        .collect::<Result<Vec<_>>>()?;
    let height = 12.0 + 5.0 * wrapped.iter().map(Vec::len).max().unwrap_or(0) as f32;
    ensure!(height <= 240.0, "architecture_pdf_diagram_height");
    for (i, lines) in wrapped.iter().enumerate() {
        let x = if i == 0 { 18.0 } else { 120.0 };
        for (line, text) in lines.iter().enumerate() {
            text_at(text, x + 3.0, 260.0 - 5.0 * line as f32);
        }
    }
    let mut line = |points: &[(f32, f32)], closed: bool| {
        operations.push(Op::DrawLine {
            line: printpdf::Line {
                points: points
                    .iter()
                    .map(|(x, y)| printpdf::LinePoint {
                        p: Point::new(Mm(*x), Mm(*y)),
                        bezier: false,
                    })
                    .collect(),
                is_closed: closed,
            },
        });
    };
    for i in 0..labels.len() {
        let x = if i == 0 { 18.0 } else { 120.0 };
        line(
            &[
                (x, 268.0),
                (x + 72.0, 268.0),
                (x + 72.0, 268.0 - height),
                (x, 268.0 - height),
            ],
            true,
        );
    }
    if labels.len() == 2 {
        line(&[(90.0, 258.0), (118.0, 258.0)], false);
        line(&[(115.0, 260.0), (118.0, 258.0), (115.0, 256.0)], false);
    }
    Ok(PdfPage::new(Mm(210.0), Mm(297.0), operations))
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
    wrap_text_with_width(text, max_width_mm, |value| text_width_mm(value, font))
}

fn layout_text(text: &str, font: &ParsedFont, max_width_mm: f32) -> Result<Vec<PdfLayoutLine>> {
    let mut layout = Vec::new();
    for logical in expected_logical_lines(text) {
        let physical =
            wrap_text_with_width(&logical, max_width_mm, |value| text_width_mm(value, font))?;
        let last = physical.len().saturating_sub(1);
        for (index, line) in physical.into_iter().enumerate() {
            layout.push(PdfLayoutLine {
                tag: if line.is_empty() {
                    PDF_EMPTY_LINE_TAG
                } else if index == last {
                    PDF_LINE_END_TAG
                } else {
                    PDF_LINE_WRAP_TAG
                },
                text: line,
            });
        }
    }
    Ok(layout)
}

fn markdown_semantic_text(source: &str) -> Result<String> {
    let tree = markdown::to_mdast(source, &markdown::ParseOptions::default())
        .map_err(|error| anyhow::anyhow!("pdf_markdown_parse_failed: {error}"))?;
    let mut output = String::with_capacity(source.len());
    append_semantic_node(&tree, &mut output);
    let normalized = output
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    ensure!(!normalized.trim().is_empty(), "pdf_empty_semantic_report");
    Ok(normalized)
}

pub(crate) fn semantic_text(source: &str) -> Result<String> {
    markdown_semantic_text(source)
}

pub(crate) fn validate_rendered_content(
    bytes: &[u8],
    expected_semantic: &str,
    expected_page_count: usize,
) -> Result<()> {
    ensure!(
        bytes.starts_with(b"%PDF-") && bytes.len() as u64 <= MAX_PDF_BYTES,
        "pdf_content_invalid_or_oversized"
    );
    let document = LopdfDocument::load_mem(bytes).context("pdf_content_parse_failed")?;
    ensure!(
        !document.trailer.has(b"Encrypt"),
        "pdf_content_encryption_forbidden"
    );
    ensure!(
        document
            .objects
            .values()
            .all(|object| !pdf_object_has_active_content(object)),
        "pdf_content_active_or_external_resource_forbidden"
    );
    ensure!(
        !pdf_object_has_active_content(&LopdfObject::Dictionary(document.trailer.clone())),
        "pdf_content_active_or_external_resource_forbidden"
    );
    let pages = document.get_pages().into_keys().collect::<Vec<_>>();
    ensure!(
        pages.len() == expected_page_count,
        "pdf_content_page_count_mismatch"
    );
    ensure!(
        extracted_logical_lines(&document, &pages)? == expected_logical_lines(expected_semantic),
        "pdf_content_semantic_mismatch"
    );
    Ok(())
}

fn expected_logical_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(|line| line.replace('\t', "    "))
        .collect()
}

fn extracted_logical_lines(document: &LopdfDocument, pages: &[u32]) -> Result<Vec<String>> {
    let page_objects = document.get_pages();
    let mut logical = Vec::new();
    let mut current = String::new();
    for page_number in pages {
        let page_id = page_objects
            .get(page_number)
            .ok_or_else(|| anyhow::anyhow!("pdf_content_page_object_missing"))?;
        let content = lopdf::content::Content::decode(
            &document
                .get_page_content_with_limit(*page_id, MAX_PDF_BYTES as usize)
                .context("pdf_content_stream_decode_failed")?,
        )
        .context("pdf_content_operations_invalid")?;
        let tags = content
            .operations
            .iter()
            .filter_map(|operation| {
                if operation.operator != "BMC" {
                    return None;
                }
                match operation.operands.first() {
                    Some(LopdfObject::Name(tag))
                        if matches!(
                            tag.as_slice(),
                            b"ADLVisualWrap" | b"ADLLogicalEnd" | b"ADLLogicalEmpty"
                        ) =>
                    {
                        Some(tag.as_slice())
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        if tags.is_empty() {
            continue;
        }
        let extracted = document
            .extract_text(&[*page_number])
            .context("pdf_content_text_extraction_failed")?;
        let physical = extracted.lines().collect::<Vec<_>>();
        ensure!(
            physical.len() == tags.len(),
            "pdf_content_line_structure_mismatch"
        );
        for (line, tag) in physical.into_iter().zip(tags) {
            match tag {
                b"ADLVisualWrap" => current.push_str(line),
                b"ADLLogicalEnd" => {
                    current.push_str(line);
                    logical.push(std::mem::take(&mut current));
                }
                b"ADLLogicalEmpty" => {
                    ensure!(line == " ", "pdf_content_empty_line_marker_mismatch");
                    ensure!(current.is_empty(), "pdf_content_line_structure_mismatch");
                    logical.push(String::new());
                }
                _ => unreachable!(),
            }
        }
    }
    ensure!(current.is_empty(), "pdf_content_unterminated_logical_line");
    Ok(logical)
}

fn pdf_object_has_active_content(object: &LopdfObject) -> bool {
    pdf_object_has_active_content_at(object, None)
}

fn pdf_object_has_active_content_at(object: &LopdfObject, parent_key: Option<&[u8]>) -> bool {
    match object {
        LopdfObject::Name(name) => matches!(
            name.as_slice(),
            b"Action"
                | b"JavaScript"
                | b"Launch"
                | b"SubmitForm"
                | b"ImportData"
                | b"GoToE"
                | b"GoToR"
                | b"URI"
                | b"Filespec"
                | b"EmbeddedFile"
                | b"RichMedia"
                | b"Movie"
                | b"Sound"
                | b"Rendition"
        ),
        LopdfObject::Array(values) => values
            .iter()
            .any(|value| pdf_object_has_active_content_at(value, parent_key)),
        LopdfObject::Dictionary(dictionary) => dictionary.iter().any(|(key, value)| {
            // printpdf uses /F as an internal font-resource handle. Everywhere
            // else /F is an external file selector and is forbidden.
            (key == b"F" && parent_key != Some(b"Font"))
                || (key == b"Annots"
                    && !matches!(value, LopdfObject::Array(values) if values.is_empty()))
                || pdf_key_is_active(key)
                || pdf_object_has_active_content_at(value, Some(key))
        }),
        LopdfObject::Stream(stream) => stream.dict.iter().any(|(key, value)| {
            key == b"F"
                || pdf_key_is_active(key)
                || pdf_object_has_active_content_at(value, Some(key))
        }),
        _ => false,
    }
}

fn pdf_key_is_active(key: &[u8]) -> bool {
    matches!(
        key,
        b"A" | b"AA"
            | b"OpenAction"
            | b"AcroForm"
            | b"JavaScript"
            | b"JS"
            | b"URI"
            | b"Launch"
            | b"SubmitForm"
            | b"ImportData"
            | b"GoToE"
            | b"GoToR"
            | b"AF"
            | b"EF"
            | b"RF"
            | b"FS"
            | b"UF"
            | b"DOS"
            | b"Mac"
            | b"Unix"
            | b"FFilter"
            | b"FDecodeParms"
            | b"EmbeddedFiles"
            | b"EmbeddedFile"
            | b"RichMedia"
            | b"XFA"
    )
}

fn append_semantic_node(node: &markdown::mdast::Node, output: &mut String) {
    use markdown::mdast::Node;

    match node {
        Node::Root(value) => append_semantic_children(&value.children, output, "\n"),
        Node::Blockquote(value) => append_semantic_children(&value.children, output, "\n"),
        Node::FootnoteDefinition(value) => append_semantic_children(&value.children, output, "\n"),
        Node::List(value) => append_semantic_children(&value.children, output, "\n"),
        Node::Delete(value) => append_semantic_children(&value.children, output, ""),
        Node::Emphasis(value) => append_semantic_children(&value.children, output, ""),
        Node::Link(value) => append_semantic_children(&value.children, output, ""),
        Node::LinkReference(value) => append_semantic_children(&value.children, output, ""),
        Node::Strong(value) => append_semantic_children(&value.children, output, ""),
        Node::Heading(value) => append_semantic_children(&value.children, output, ""),
        Node::Table(value) => append_semantic_children(&value.children, output, "\n"),
        Node::TableRow(value) => append_semantic_children(&value.children, output, " | "),
        Node::TableCell(value) => append_semantic_children(&value.children, output, ""),
        Node::ListItem(value) => {
            output.push_str("- ");
            append_semantic_children(&value.children, output, "\n");
        }
        Node::Paragraph(value) => append_semantic_children(&value.children, output, ""),
        Node::Break(_) => output.push('\n'),
        Node::InlineCode(value) => output.push_str(&value.value),
        Node::InlineMath(value) => output.push_str(&value.value),
        Node::Text(value) => output.push_str(&value.value),
        Node::Code(value) => output.push_str(&value.value),
        Node::Math(value) => output.push_str(&value.value),
        Node::Image(value) => output.push_str(&value.alt),
        Node::ImageReference(value) => output.push_str(&value.alt),
        Node::FootnoteReference(value) => output.push_str(&value.identifier),
        Node::ThematicBreak(_) => output.push_str("---"),
        Node::Definition(_)
        | Node::Html(_)
        | Node::MdxjsEsm(_)
        | Node::MdxFlowExpression(_)
        | Node::MdxTextExpression(_)
        | Node::MdxJsxFlowElement(_)
        | Node::MdxJsxTextElement(_)
        | Node::Toml(_)
        | Node::Yaml(_) => {}
    }
}

fn append_semantic_children(
    children: &[markdown::mdast::Node],
    output: &mut String,
    separator: &str,
) {
    for (index, child) in children.iter().enumerate() {
        if index > 0 {
            output.push_str(separator);
        }
        append_semantic_node(child, output);
    }
}

fn wrap_text_with_width(
    text: &str,
    max_width: f32,
    measure: impl Fn(&str) -> Result<f32>,
) -> Result<Vec<String>> {
    ensure!(max_width > 0.0, "pdf_printable_width_invalid");
    let mut lines = Vec::new();
    for source_line in text.lines() {
        let source_line = source_line.replace('\t', "    ");
        if source_line.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        for character in source_line.chars() {
            let mut candidate = current.clone();
            candidate.push(character);
            if !current.is_empty() && measure(&candidate)? > max_width {
                lines.push(std::mem::take(&mut current));
                candidate = character.to_string();
            }
            ensure!(
                measure(&candidate)? <= max_width,
                "pdf_glyph_exceeds_printable_width_u{:04x}",
                character as u32
            );
            current = candidate;
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    Ok(lines)
}

pub(crate) fn validate_manifest(
    manifest: &PdfManifest,
    prepared: &super::markdown::PreparedReport,
) -> Result<()> {
    ensure!(
        manifest.schema == PDF_MANIFEST_SCHEMA
            && manifest.renderer_version == PDF_RENDERER_VERSION
            && manifest.renderer_engine == "printpdf@0.12.8; layout=codefriend-pdf-v2"
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
            && manifest.semantic_digest
                == digest(markdown_semantic_text(&prepared.text)?.as_bytes())
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
    use super::{
        build_pdf, layout_text, markdown_semantic_text, validate_rendered_content,
        wrap_text_with_width, PRINTABLE_WIDTH_MM,
    };
    use printpdf::ParsedFont;

    fn qualification_font() -> Vec<u8> {
        [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/System/Library/Fonts/Supplemental/Arial.ttf",
            "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
            "/Library/Fonts/Arial Unicode.ttf",
        ]
        .into_iter()
        .find_map(|path| std::fs::read(path).ok())
        .expect("PDF qualification requires an installed Unicode TrueType font")
    }

    #[test]
    fn semantic_text_keeps_preformatted_excerpt_line_and_tab_boundaries() {
        let text = markdown_semantic_text(
            "- **Exact source excerpt:**\n\n```\nif authorized:\r\n\tdelete_records()\r\nreturn ok\n```\n",
        )
        .unwrap();
        assert!(
            text.contains("if authorized:\n\tdelete_records()\nreturn ok"),
            "{text:?}"
        );
    }

    #[test]
    fn rendered_pdf_extraction_keeps_code_block_indentation_and_double_spaces() {
        let semantic = markdown_semantic_text(
            "```\nif ready:\n    let  result = verify();\n    publish(result);\n```\n",
        )
        .unwrap();
        let mut warnings = Vec::new();
        let font = ParsedFont::from_bytes(&qualification_font(), 0, &mut warnings).unwrap();
        assert!(warnings.is_empty());
        let lines = layout_text(&semantic, &font, PRINTABLE_WIDTH_MM).unwrap();
        let bytes = build_pdf(&lines, font, &Default::default()).unwrap();
        let document = lopdf::Document::load_mem(&bytes).unwrap();
        let pages = document.get_pages().into_keys().collect::<Vec<_>>();
        let extracted = document.extract_text(&pages).unwrap();
        assert!(
            extracted.contains("if ready:\n    let  result = verify();\n    publish(result);"),
            "rendered extraction lost significant code whitespace: {extracted:?}"
        );
        validate_rendered_content(&bytes, &semantic, 1).unwrap();
    }

    #[test]
    fn verifier_rejects_whitespace_moved_across_a_logical_newline() {
        let semantic = "alpha\n beta";
        let mut warnings = Vec::new();
        let font = ParsedFont::from_bytes(&qualification_font(), 0, &mut warnings).unwrap();
        assert!(warnings.is_empty());
        let lines = layout_text(semantic, &font, PRINTABLE_WIDTH_MM).unwrap();
        let original = build_pdf(&lines, font, &Default::default()).unwrap();
        validate_rendered_content(&original, semantic, 1).unwrap();

        let mut document = lopdf::Document::load_mem(&original).unwrap();
        let first_page = *document.get_pages().values().next().unwrap();
        let mut content =
            lopdf::content::Content::decode(&document.get_page_content(first_page)).unwrap();
        let boundary = content
            .operations
            .iter_mut()
            .find(|operation| {
                operation.operator == "BMC"
                    && matches!(
                        operation.operands.first(),
                        Some(lopdf::Object::Name(tag)) if tag == b"ADLLogicalEnd"
                    )
            })
            .expect("rendered PDF must expose its first logical boundary");
        boundary.operands[0] = lopdf::Object::Name(b"ADLVisualWrap".to_vec());
        document
            .change_page_content(first_page, content.encode().unwrap())
            .unwrap();
        let original_text = lopdf::Document::load_mem(&original)
            .unwrap()
            .extract_text(&[1])
            .unwrap();
        assert_eq!(document.extract_text(&[1]).unwrap(), original_text);
        let mut substituted = Vec::new();
        document.save_to(&mut substituted).unwrap();
        let error = validate_rendered_content(&substituted, semantic, 1).unwrap_err();
        assert!(error.to_string().contains("pdf_content_semantic_mismatch"));
    }

    #[test]
    fn verifier_rejects_active_and_external_pdf_structures_with_same_text() {
        let semantic = "approved report text";
        let mut warnings = Vec::new();
        let font = ParsedFont::from_bytes(&qualification_font(), 0, &mut warnings).unwrap();
        assert!(warnings.is_empty());
        let lines = layout_text(semantic, &font, PRINTABLE_WIDTH_MM).unwrap();
        let original = build_pdf(&lines, font, &Default::default()).unwrap();
        validate_rendered_content(&original, semantic, 1).unwrap();

        let mut file_spec = lopdf::Dictionary::new();
        file_spec.set("Type", lopdf::Object::Name(b"Filespec".to_vec()));
        file_spec.set("F", lopdf::Object::string_literal("external.bin"));

        let reject = |mut document: lopdf::Document| {
            assert_eq!(
                document.extract_text(&[1]).unwrap(),
                lopdf::Document::load_mem(&original)
                    .unwrap()
                    .extract_text(&[1])
                    .unwrap()
            );
            let mut substituted = Vec::new();
            document.save_to(&mut substituted).unwrap();
            let error = validate_rendered_content(&substituted, semantic, 1).unwrap_err();
            assert!(error
                .to_string()
                .contains("pdf_content_active_or_external_resource_forbidden"));
        };

        let mut document = lopdf::Document::load_mem(&original).unwrap();
        let first_page = *document.get_pages().values().next().unwrap();
        let mut action = lopdf::Dictionary::new();
        action.set("S", lopdf::Object::Name(b"GoToE".to_vec()));
        action.set("F", lopdf::Object::Dictionary(file_spec.clone()));
        document
            .get_object_mut(first_page)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("A", lopdf::Object::Dictionary(action));
        reject(document);

        let mut document = lopdf::Document::load_mem(&original).unwrap();
        let first_page = *document.get_pages().values().next().unwrap();
        document
            .get_object_mut(first_page)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set(
                "Annots",
                lopdf::Object::Array(vec![lopdf::Object::Dictionary(lopdf::Dictionary::new())]),
            );
        reject(document);

        let mut document = lopdf::Document::load_mem(&original).unwrap();
        let first_page = *document.get_pages().values().next().unwrap();
        document
            .get_object_mut(first_page)
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set(
                "AF",
                lopdf::Object::Array(vec![lopdf::Object::Dictionary(file_spec)]),
            );
        reject(document);

        let mut document = lopdf::Document::load_mem(&original).unwrap();
        let mut external_stream = lopdf::Dictionary::new();
        external_stream.set("Type", lopdf::Object::Name(b"EmbeddedFile".to_vec()));
        external_stream.set("F", lopdf::Object::string_literal("external.bin"));
        document.add_object(lopdf::Stream::new(external_stream, Vec::new()));
        reject(document);
    }

    #[test]
    fn semantic_text_decodes_markdown_escapes_but_preserves_real_backslashes() {
        let text = markdown_semantic_text(
            r"Path `lib/dnsmsg-parser/src/dns_message.rs` and escaped lib/dnsmsg\-parser/src/dns\_message\.rs; Windows C:\temp\file.txt.",
        )
        .unwrap();
        assert!(text.contains("lib/dnsmsg-parser/src/dns_message.rs"));
        assert!(text.contains(r"C:\temp\file.txt"));
        assert!(!text.contains(r"dnsmsg\-parser"));
        assert!(!text.contains(r"dns\_message"));
    }

    #[test]
    fn wrapping_preserves_all_unicode_and_splits_long_tokens() {
        let input = "Résumé café π\nhttps://example.invalid/abcdefghijklmnopqrstuvwxyz";
        let lines =
            wrap_text_with_width(input, 12.0, |value| Ok(value.chars().count() as f32)).unwrap();
        assert!(lines.iter().all(|line| line.chars().count() <= 12));
        assert_eq!(
            lines.concat().replace(' ', ""),
            input.replace([' ', '\n'], "")
        );
    }
}
