//! Exact-artifact publication approval and fail-closed local admission.
pub mod approval;
pub mod manifest;
pub mod markdown;
pub mod pdf;

pub use approval::{
    admit_local, append_decision, read_decision_head, write_json_create_only, AdmissionReceipt,
    DecisionKind, DecisionRecord,
};
pub use manifest::{read_publication, read_review, verify_artifacts, ManifestInput};
pub use markdown::{
    render_markdown, MarkdownManifest, MarkdownRenderOptions, MarkdownRenderResult,
    MARKDOWN_MANIFEST_SCHEMA, MARKDOWN_RENDERER_VERSION, MARKDOWN_RESULT_SCHEMA,
};
pub use pdf::{
    render_pdf, PdfManifest, PdfRenderOptions, PdfRenderResult, PDF_MANIFEST_SCHEMA,
    PDF_RENDERER_VERSION, PDF_RESULT_SCHEMA,
};
