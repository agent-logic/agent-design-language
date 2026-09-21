//! Language-neutral facts derived only from immutable admitted source.
//! Parser success is distinct from structural and semantic completeness.
mod imports;
pub mod owner;
pub mod parser;
mod resolver;
use crate::codefriend::{
    evidence::{hash, valid_digest, Admission},
    ingestion::validate_path,
};
use anyhow::{ensure, Result};
pub(crate) use resolver::{ImportForm, ImportSpec};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const VERSION: &str = "codefriend.language_analysis.v1";
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Rust,
    Java,
    Python,
    JavaScript,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
}
impl Span {
    pub fn validate(&self, source: &str) -> Result<()> {
        ensure!(
            self.start_byte <= self.end_byte
                && self.end_byte <= source.len()
                && source.is_char_boundary(self.start_byte)
                && source.is_char_boundary(self.end_byte),
            "language_span_outside_source"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub max_nodes: usize,
    pub max_depth: usize,
    pub max_facts: usize,
    pub max_output_bytes: usize,
}
impl Limits {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            (1..=1_000_000).contains(&self.max_nodes)
                && (1..=512).contains(&self.max_depth)
                && (1..=100_000).contains(&self.max_facts)
                && (1..=4 * 1024 * 1024).contains(&self.max_output_bytes),
            "invalid_language_resource_limits"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProjectRoot {
    pub language: Language,
    pub root: String,
    pub manifest: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AnalysisPolicy {
    pub schema: String,
    pub files: BTreeMap<String, Language>,
    pub roots: Vec<ProjectRoot>,
    pub layers: BTreeMap<String, String>,
    pub allowed: BTreeSet<(String, String)>,
    pub limits: Limits,
}
impl AnalysisPolicy {
    pub fn validate(&self, admission: &Admission) -> Result<()> {
        admission.validate()?;
        self.limits.validate()?;
        ensure!(
            self.schema == VERSION && !self.files.is_empty() && self.files.len() <= 1000,
            "invalid_language_policy"
        );
        // Cover the entire requested analysis scope, including omitted/missing objects.
        ensure!(
            self.files.keys().eq(admission.packet.scope.analysis.iter()),
            "language_policy_scope_mismatch"
        );
        ensure!(
            !self.roots.is_empty() && self.roots.len() <= 1000,
            "invalid_language_roots"
        );
        let mut roots = BTreeSet::new();
        for root in &self.roots {
            if root.root != "." {
                validate_path(&root.root)?;
            }
            ensure!(
                roots.insert((root.language, &root.root)),
                "duplicate_language_root"
            );
            if let Some(path) = &root.manifest {
                validate_path(path)?;
                ensure!(
                    admission
                        .packet
                        .objects
                        .iter()
                        .any(|o| &o.path == path && o.content.is_some()),
                    "language_manifest_not_admitted"
                );
            }
        }
        for (path, language) in &self.files {
            validate_path(path)?;
            ensure!(
                self.roots.iter().any(|r| &r.language == language
                    && (r.root == "." || path.starts_with(&format!("{}/", r.root)))),
                "language_file_without_root"
            );
        }
        ensure!(
            self.layers.len() <= self.files.len() && self.allowed.len() <= 1000,
            "language_layer_limit"
        );
        for (path, layer) in &self.layers {
            ensure!(
                self.files.contains_key(path)
                    && !layer.is_empty()
                    && layer.len() <= 64
                    && layer
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c)),
                "invalid_language_layer"
            );
        }
        let layers: BTreeSet<_> = self.layers.values().collect();
        ensure!(
            self.allowed
                .iter()
                .all(|(a, b)| layers.contains(a) && layers.contains(b)),
            "unknown_language_layer"
        );
        Ok(())
    }
    pub fn digest(&self) -> Result<String> {
        hash(self)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ToolIdentity {
    pub name: String,
    pub version: String,
    pub grammar_revision: String,
    pub query_digest: String,
}
impl ToolIdentity {
    fn validate(&self) -> Result<()> {
        ensure!(
            [&self.name, &self.version, &self.grammar_revision]
                .iter()
                .all(|v| !v.is_empty() && v.len() <= 128 && !v.chars().any(char::is_control))
                && valid_digest(&self.query_digest),
            "invalid_language_tool_identity"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Coverage {
    Complete,
    Partial,
    Unsupported,
    Failed,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub code: String,
    pub span: Option<Span>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FileCoverage {
    pub path: String,
    pub evidence_id: Option<String>,
    pub content_digest: Option<String>,
    pub language: Language,
    pub syntax: Coverage,
    pub structure: Coverage,
    pub semantics: Coverage,
    pub diagnostics: Vec<Diagnostic>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Declaration {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub span: Span,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Import {
    pub spelling: String,
    pub kind: String,
    pub span: Span,
    pub target_path: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub spelling: String,
    pub span: Span,
    pub target_declaration: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceFacts {
    pub declarations: Vec<Declaration>,
    pub imports: Vec<Import>,
    pub references: Vec<Reference>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FileAnalysis {
    pub coverage: FileCoverage,
    pub facts: SourceFacts,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AnalysisReport {
    pub schema: String,
    pub admission_digest: String,
    pub policy_digest: String,
    pub toolchain: BTreeMap<Language, ToolIdentity>,
    pub files: Vec<FileAnalysis>,
    pub digest: String,
}
/// Source-backed identity; never use parser-local node pointers as durable IDs.
pub fn declaration_id(
    evidence_id: &str,
    language: Language,
    span: Span,
    name: &str,
    kind: &str,
) -> Result<String> {
    hash(&(VERSION, evidence_id, language, span, name, kind))
}
impl AnalysisReport {
    // These checks establish internal consistency, not trusted producer provenance.
    // External consumers must authenticate the installed owner/candidate and original
    // source, or recompute analysis; an untrusted self-hash is never execution proof.
    pub fn expected_digest(&self) -> Result<String> {
        let mut value = self.clone();
        value.digest.clear();
        hash(&value)
    }
    pub fn validate(&self, admission: &Admission, policy: &AnalysisPolicy, now: u64) -> Result<()> {
        policy.validate(admission)?;
        ensure!(
            now >= admission.admitted_at && now < admission.expires_at,
            "language_admission_not_live"
        );
        ensure!(
            self.schema == VERSION
                && self.admission_digest == admission.digest
                && self.policy_digest == policy.digest()?,
            "language_report_identity_mismatch"
        );
        ensure!(
            self.files
                .iter()
                .map(|f| &f.coverage.path)
                .eq(policy.files.keys()),
            "language_report_scope_mismatch"
        );
        let languages: BTreeSet<_> = policy.files.values().copied().collect();
        ensure!(
            self.toolchain.keys().copied().collect::<BTreeSet<_>>() == languages,
            "language_toolchain_mismatch"
        );
        for tool in self.toolchain.values() {
            tool.validate()?;
        }
        let mut declarations = BTreeMap::new();
        let mut count = 0usize;
        for file in &self.files {
            let c = &file.coverage;
            ensure!(
                policy.files.get(&c.path) == Some(&c.language),
                "language_identity_mismatch"
            );
            let object = admission
                .packet
                .objects
                .iter()
                .find(|o| o.path == c.path)
                .ok_or_else(|| anyhow::anyhow!("language_source_missing"))?;
            let evidence = admission.evidence.iter().find(|e| e.path == c.path);
            ensure!(
                c.evidence_id.as_deref() == evidence.map(|e| e.id.as_str())
                    && c.content_digest == object.content_digest,
                "language_source_identity_mismatch"
            );
            let n = file.facts.declarations.len()
                + file.facts.imports.len()
                + file.facts.references.len()
                + c.diagnostics.len();
            count = count
                .checked_add(n)
                .ok_or_else(|| anyhow::anyhow!("language_fact_limit"))?;
            ensure!(count <= policy.limits.max_facts, "language_fact_limit");
            ensure!(
                [c.syntax, c.structure, c.semantics]
                    .iter()
                    .all(|s| *s == Coverage::Complete)
                    || !c.diagnostics.is_empty(),
                "language_incomplete_reason_missing"
            );
            ensure!(
                c.syntax == Coverage::Complete
                    || (c.structure != Coverage::Complete && c.semantics != Coverage::Complete),
                "language_coverage_overclaim"
            );
            if object.content.is_none() {
                ensure!(
                    c.syntax != Coverage::Complete
                        && c.structure != Coverage::Complete
                        && c.semantics != Coverage::Complete
                        && file.facts == SourceFacts::default(),
                    "language_missing_source_overclaim"
                );
            }
            let source = object.content.as_deref().unwrap_or("");
            for d in &c.diagnostics {
                label(&d.code)?;
                if matches!(d.code.as_str(), "parse_error" | "missing_syntax") {
                    ensure!(
                        c.syntax != Coverage::Complete,
                        "language_syntax_error_overclaim"
                    );
                }
                if let Some(s) = d.span {
                    s.validate(source)?;
                }
            }
            for d in &file.facts.declarations {
                label(&d.kind)?;
                text(&d.name)?;
                d.span.validate(source)?;
                ensure!(
                    &source[d.span.start_byte..d.span.end_byte] == d.name,
                    "language_declaration_text_mismatch"
                );
                ensure!(
                    Some(&d.id)
                        == c.evidence_id
                            .as_ref()
                            .map(|e| declaration_id(e, c.language, d.span, &d.name, &d.kind))
                            .transpose()?
                            .as_ref()
                        && declarations.insert(d.id.clone(), c.language).is_none(),
                    "language_declaration_identity_mismatch"
                );
            }
            for i in &file.facts.imports {
                text(&i.spelling)?;
                label(&i.kind)?;
                i.span.validate(source)?;
                ensure!(
                    &source[i.span.start_byte..i.span.end_byte] == i.spelling,
                    "language_import_text_mismatch"
                );
                if let Some(p) = &i.target_path {
                    ensure!(
                        policy.files.get(p) == Some(&c.language)
                            && admission
                                .packet
                                .objects
                                .iter()
                                .any(|o| &o.path == p && o.content.is_some())
                            && admission.evidence.iter().any(|e| &e.path == p),
                        "language_import_target_unavailable_or_cross_language"
                    );
                }
            }
            for r in &file.facts.references {
                text(&r.spelling)?;
                r.span.validate(source)?;
                ensure!(
                    &source[r.span.start_byte..r.span.end_byte] == r.spelling,
                    "language_reference_text_mismatch"
                );
            }
        }
        for file in &self.files {
            for r in &file.facts.references {
                if let Some(id) = &r.target_declaration {
                    ensure!(
                        declarations.get(id) == Some(&file.coverage.language),
                        "language_reference_target_missing"
                    );
                }
            }
        }
        ensure!(
            self.digest == self.expected_digest()?,
            "language_report_digest_mismatch"
        );
        ensure!(
            serde_json::to_vec(self)?.len() <= policy.limits.max_output_bytes,
            "language_output_limit"
        );
        Ok(())
    }
}
fn text(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty() && value.len() <= 4096 && !value.contains('\0'),
        "invalid_language_fact_text"
    );
    Ok(())
}
fn label(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
        "invalid_language_diagnostic_or_kind"
    );
    Ok(())
}
