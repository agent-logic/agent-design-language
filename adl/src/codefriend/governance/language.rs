//! Versioned source-only fitness rules. V1 policy/report bytes are unchanged.
use crate::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        hash,
        store::Store,
    },
    language::{owner, AnalysisPolicy, Coverage, ImportForm, ImportSpec, Language, Span},
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const VERSION: &str = "codefriend.fitness.v2";
const MAX_REPORT: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "language_form", rename_all = "snake_case", deny_unknown_fields)]
pub enum StaticSelector {
    RustUse {
        prefix: Vec<String>,
    },
    JavaTypeImport {
        prefix: Vec<String>,
    },
    JavaStaticImport {
        prefix: Vec<String>,
    },
    PythonModule {
        prefix: Vec<String>,
    },
    PythonFrom {
        module_prefix: Vec<String>,
        member: Option<String>,
    },
    JavaScriptModule {
        specifier: String,
    },
}
impl StaticSelector {
    fn language(&self) -> Language {
        match self {
            Self::RustUse { .. } => Language::Rust,
            Self::JavaTypeImport { .. } | Self::JavaStaticImport { .. } => Language::Java,
            Self::PythonModule { .. } | Self::PythonFrom { .. } => Language::Python,
            Self::JavaScriptModule { .. } => Language::JavaScript,
        }
    }
    fn validate(&self) -> Result<()> {
        match self {
            Self::RustUse { prefix }
            | Self::JavaTypeImport { prefix }
            | Self::JavaStaticImport { prefix }
            | Self::PythonModule { prefix } => parts(prefix),
            Self::PythonFrom {
                module_prefix,
                member,
            } => {
                parts(module_prefix)?;
                if let Some(m) = member {
                    token(m)?;
                }
                Ok(())
            }
            Self::JavaScriptModule { specifier } => {
                ensure!(
                    !specifier.is_empty()
                        && specifier.len() <= 4096
                        && !specifier.chars().any(char::is_control),
                    "invalid_fitness_module_selector"
                );
                Ok(())
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Rule {
    ForbiddenStaticImport {
        id: String,
        source_path: String,
        selector: StaticSelector,
    },
    ForbiddenResolvedEdge {
        id: String,
        source_path: String,
        language: Language,
        forbidden_targets: BTreeSet<String>,
    },
}
impl Rule {
    fn id(&self) -> &str {
        match self {
            Self::ForbiddenStaticImport { id, .. } | Self::ForbiddenResolvedEdge { id, .. } => id,
        }
    }
    fn source(&self) -> &str {
        match self {
            Self::ForbiddenStaticImport { source_path, .. }
            | Self::ForbiddenResolvedEdge { source_path, .. } => source_path,
        }
    }
    fn language(&self) -> Language {
        match self {
            Self::ForbiddenStaticImport { selector, .. } => selector.language(),
            Self::ForbiddenResolvedEdge { language, .. } => *language,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema: String,
    pub analysis: AnalysisPolicy,
    pub rules: Vec<Rule>,
}
impl Policy {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.schema == VERSION && !self.rules.is_empty() && self.rules.len() <= 32,
            "invalid_language_fitness_policy"
        );
        self.analysis.limits.validate()?;
        let mut ids = BTreeSet::new();
        for r in &self.rules {
            token(r.id())?;
            ensure!(
                r.id().len() <= 64 && ids.insert(r.id()),
                "duplicate_fitness_rule"
            );
            ensure!(
                self.analysis.files.get(r.source()) == Some(&r.language()),
                "fitness_rule_source_language_mismatch"
            );
            match r {
                Rule::ForbiddenStaticImport { selector, .. } => selector.validate()?,
                Rule::ForbiddenResolvedEdge {
                    forbidden_targets, ..
                } => {
                    ensure!(
                        !forbidden_targets.is_empty() && forbidden_targets.len() <= 1000,
                        "invalid_fitness_edge_targets"
                    );
                    for p in forbidden_targets {
                        ensure!(
                            self.analysis.files.get(p) == Some(&r.language()),
                            "fitness_edge_target_language_mismatch"
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pass,
    Fail,
    Unknown,
}
impl Status {
    pub fn exit_code(self) -> i32 {
        match self {
            Self::Pass => 0,
            Self::Fail => 1,
            Self::Unknown => 2,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Witness {
    pub span: Span,
    pub evidence_id: String,
    pub reference: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuleResult {
    pub rule_id: String,
    pub source_path: String,
    pub status: Status,
    pub violations: Vec<Witness>,
    pub unknowns: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub schema: String,
    pub policy: Policy,
    pub policy_digest: String,
    pub analysis_digest: String,
    pub status: Status,
    pub results: Vec<RuleResult>,
    pub record: ReviewRecord,
    pub digest: String,
}
fn token(s: &str) -> Result<()> {
    ensure!(
        !s.is_empty() && s.len() <= 256 && !s.chars().any(|c| c.is_control() || c.is_whitespace()),
        "invalid_fitness_selector_component"
    );
    Ok(())
}
fn parts(p: &[String]) -> Result<()> {
    ensure!(
        !p.is_empty() && p.len() <= 32,
        "invalid_fitness_selector_components"
    );
    for s in p {
        token(s)?;
        ensure!(
            !s.contains(['.', '/', ':', '\\']),
            "invalid_fitness_selector_component"
        );
    }
    Ok(())
}
fn identity(s: &ImportSpec) -> bool {
    matches!(
        s.form,
        ImportForm::JavaPackage | ImportForm::JavaType | ImportForm::RustInlineModule
    )
}
fn matching(selector: &StaticSelector, s: &ImportSpec) -> Option<bool> {
    let (prefix, expected) = match selector {
        StaticSelector::RustUse { prefix } => (prefix, ImportForm::RustUse),
        StaticSelector::JavaTypeImport { prefix } => (prefix, ImportForm::JavaImport),
        StaticSelector::JavaStaticImport { prefix } => (prefix, ImportForm::JavaStaticImport),
        StaticSelector::PythonModule { prefix } => (prefix, ImportForm::PythonImport),
        StaticSelector::PythonFrom {
            module_prefix,
            member,
        } => {
            if s.form != ImportForm::PythonFrom {
                return Some(false);
            }
            if s.relative_depth > 0 {
                return None;
            }
            if !s.components.starts_with(module_prefix) {
                return if s.is_glob && module_prefix.starts_with(&s.components) {
                    None
                } else {
                    Some(false)
                };
            }
            return match member {
                None => Some(true),
                Some(m) => {
                    if s.is_glob {
                        None
                    } else {
                        Some(s.imported.as_ref() == Some(m))
                    }
                }
            };
        }
        StaticSelector::JavaScriptModule { specifier } => {
            return if s.form == ImportForm::JavaScriptModule {
                s.specifier.as_ref().map(|v| v == specifier)
            } else {
                Some(false)
            }
        }
    };
    if s.form != expected {
        return Some(false);
    }
    if s.relative_depth > 0
        || s.components
            .first()
            .is_some_and(|p| matches!(p.as_str(), "self" | "super"))
    {
        return None;
    }
    if s.components.starts_with(prefix) {
        Some(true)
    } else if s.is_glob && prefix.starts_with(&s.components) {
        None
    } else {
        Some(false)
    }
}
struct LimitedWriter {
    bytes: Vec<u8>,
    count: usize,
    retain: bool,
}
impl std::io::Write for LimitedWriter {
    fn write(&mut self, input: &[u8]) -> std::io::Result<usize> {
        let next = self
            .count
            .checked_add(input.len())
            .filter(|n| *n <= MAX_REPORT)
            .ok_or_else(|| std::io::Error::other("fitness_output_limit"))?;
        if self.retain {
            self.bytes.extend_from_slice(input);
        }
        self.count = next;
        Ok(input.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn bounded<T: Serialize>(value: &T, retain: bool) -> Result<LimitedWriter> {
    let mut writer = LimitedWriter {
        bytes: Vec::new(),
        count: 0,
        retain,
    };
    serde_json::to_writer(&mut writer, value)
        .map_err(|_| anyhow::anyhow!("fitness_output_limit"))?;
    Ok(writer)
}
fn charge<T: Serialize>(value: &T, used: &mut usize) -> Result<()> {
    *used = used
        .checked_add(bounded(value, false)?.count + 1)
        .ok_or_else(|| anyhow::anyhow!("fitness_output_limit"))?;
    ensure!(*used <= MAX_REPORT, "fitness_output_limit");
    Ok(())
}
fn seal(report: &mut Report) -> Result<()> {
    report.digest.clear();
    let bytes = bounded(report, true)?.bytes;
    report.digest = crate::codefriend::ingestion::digest(&bytes);
    bounded(report, false)?;
    Ok(())
}
fn push_witness(result: &mut RuleResult, witness: Witness, bytes: &mut usize) -> Result<()> {
    ensure!(witness.reference.len() <= 4096, "fitness_reference_limit");
    *bytes = bytes
        .checked_add(serde_json::to_vec(&witness)?.len())
        .ok_or_else(|| anyhow::anyhow!("fitness_output_limit"))?;
    ensure!(*bytes <= MAX_REPORT, "fitness_output_limit");
    result.violations.push(witness);
    Ok(())
}
/// Errors are authority/policy/resource failures, never a successful fitness report.
pub fn evaluate(store: &Store, packet_id: &str, policy: &Policy, now: u64) -> Result<Report> {
    bounded(policy, false)?;
    policy.validate()?;
    let admission = store.get(packet_id)?;
    crate::codefriend::evidence::contracts::reviewable_acquisition(&admission)?;
    policy.analysis.validate(&admission)?;
    let outcome = owner::analyze_outcome(store, packet_id, &policy.analysis, now)?;
    ensure!(
        outcome.report.admission_digest == admission.digest,
        "fitness_admission_changed"
    );
    let mut results = Vec::new();
    // Charge immutable source/policy before materializing results. Fixed reserve
    // covers field names, digest strings and vector punctuation; exact final
    // bounded serialization remains authoritative.
    let mut output_bytes = 1024usize;
    charge(policy, &mut output_bytes)?;
    charge(&admission, &mut output_bytes)?;
    for rule in &policy.rules {
        let before_witnesses = output_bytes;
        let file = outcome
            .report
            .files
            .iter()
            .find(|f| f.coverage.path == rule.source())
            .ok_or_else(|| anyhow::anyhow!("fitness_analysis_source_missing"))?;
        if file.coverage.evidence_id.is_none() {
            ensure!(
                admission
                    .packet
                    .objects
                    .iter()
                    .any(|object| object.path == rule.source()
                        && object.disposition == "omitted_unsafe"
                        && object.content.is_none()),
                "fitness_source_unavailable"
            );
            let result = RuleResult {
                rule_id: rule.id().into(),
                source_path: rule.source().into(),
                status: Status::Unknown,
                violations: Vec::new(),
                unknowns: vec!["source_excluded_by_privacy_filter".into()],
            };
            charge(&result, &mut output_bytes)?;
            results.push(result);
            continue;
        }
        ensure!(
            file.coverage.syntax != Coverage::Failed,
            "fitness_analysis_failed"
        );
        let mut result = RuleResult {
            rule_id: rule.id().into(),
            source_path: rule.source().into(),
            status: Status::Pass,
            violations: Vec::new(),
            unknowns: Vec::new(),
        };
        if file.coverage.syntax != Coverage::Complete {
            result.unknowns.push("syntax_not_complete".into());
        }
        for d in &file.coverage.diagnostics {
            if !matches!(
                d.code.as_str(),
                "project_resolution_not_performed"
                    | "semantic_resolution_not_performed"
                    | "module_resolution_unresolved"
                    | "module_resolution_ambiguous"
                    | "module_resolution_admitted_only"
                    | "admitted_module_resolution_only"
            ) {
                result.unknowns.push(d.code.clone());
            }
        }
        match rule {
            Rule::ForbiddenStaticImport { selector, .. } => {
                for spec in outcome
                    .normalized_imports
                    .iter()
                    .filter(|s| s.file == rule.source() && !identity(s))
                {
                    if spec.unsupported
                        || (rule.language() == Language::Rust
                            && spec.components.iter().any(|c| c.starts_with("r#")))
                    {
                        result.unknowns.push("unsupported_import_semantics".into());
                        continue;
                    }
                    match matching(selector, spec) {
                        Some(true) => {
                            let source = admission
                                .packet
                                .objects
                                .iter()
                                .find(|o| o.path == rule.source())
                                .and_then(|o| o.content.as_deref())
                                .ok_or_else(|| anyhow::anyhow!("fitness_source_unavailable"))?;
                            spec.span.validate(source)?;
                            push_witness(
                                &mut result,
                                Witness {
                                    span: spec.span,
                                    evidence_id: file.coverage.evidence_id.clone().unwrap(),
                                    reference: source[spec.span.start_byte..spec.span.end_byte]
                                        .into(),
                                },
                                &mut output_bytes,
                            )?;
                        }
                        None => result.unknowns.push("import_match_unresolved".into()),
                        Some(false) => {}
                    }
                }
            }
            Rule::ForbiddenResolvedEdge {
                forbidden_targets, ..
            } => {
                if file.coverage.structure != Coverage::Complete {
                    result
                        .unknowns
                        .push("structural_edge_coverage_incomplete".into());
                }
                for import in &file.facts.imports {
                    // Identity-only syntax nodes do not assert a dependency edge.
                    let specs: Vec<_> = outcome
                        .normalized_imports
                        .iter()
                        .filter(|s| s.file == rule.source() && s.span == import.span)
                        .collect();
                    if !specs.is_empty() && specs.iter().all(|s| identity(s)) {
                        continue;
                    }
                    match &import.target_path {
                        Some(target) if forbidden_targets.contains(target) => push_witness(
                            &mut result,
                            Witness {
                                span: import.span,
                                evidence_id: file.coverage.evidence_id.clone().unwrap(),
                                reference: import.spelling.clone(),
                            },
                            &mut output_bytes,
                        )?,
                        Some(_) => {}
                        None => result.unknowns.push("module_edge_unresolved".into()),
                    }
                }
                if outcome
                    .normalized_imports
                    .iter()
                    .any(|s| s.file == rule.source() && s.unsupported)
                {
                    result.unknowns.push("unsupported_import_semantics".into());
                }
            }
        }
        result.unknowns.sort();
        result.unknowns.dedup();
        result.status = if !result.violations.is_empty() {
            Status::Fail
        } else if !result.unknowns.is_empty() {
            Status::Unknown
        } else {
            Status::Pass
        };
        // Replace the incremental witness charge with the entire result charge.
        output_bytes = before_witnesses;
        charge(&result, &mut output_bytes)?;
        results.push(result);
    }
    let status = if results.iter().any(|r| r.status == Status::Fail) {
        Status::Fail
    } else if results.iter().any(|r| r.status == Status::Unknown) {
        Status::Unknown
    } else {
        Status::Pass
    };
    let policy_digest = hash(policy)?;
    let mut failures: Vec<_> = results
        .iter()
        .filter(|r| !r.unknowns.is_empty())
        .map(|r| format!("fitness_unassessed:{}", r.rule_id))
        .collect();
    if admission.packet.completeness != "complete_scoped_acquisition" {
        failures.push("source_coverage_incomplete_privacy_omissions".into());
    }
    let complete =
        failures.is_empty() && admission.packet.completeness == "complete_scoped_acquisition";
    let run = Run::new(
        &admission,
        BTreeMap::from([
            ("fitness".into(), VERSION.into()),
            ("fitness-policy".into(), policy_digest.clone()),
            ("language-analysis".into(), outcome.report.digest.clone()),
        ]),
        "local".into(),
        if complete {
            Completion::Complete
        } else {
            Completion::Incomplete
        },
        failures,
    )?;
    charge(&run, &mut output_bytes)?;
    let mut findings = BTreeMap::new();
    for result in &results {
        for witness in &result.violations {
            let mut f=Finding{schema:CONTRACT.into(),id:String::new(),repository:run.repository.clone(),perspective:"fitness".into(),rule:"explicit_language_fitness".into(),semantic_anchor:format!("{}:{}:{}",result.rule_id,result.source_path,witness.span.start_byte),title:"Source violates explicit fitness policy".into(),severity:Severity::High,rationale:format!("Rule {} has an observed violation in {}.",result.rule_id,result.source_path),confidence:Confidence::Known(100),evidence:vec![witness.evidence_id.clone()],inference:"Observed admitted static import or resolved admitted module edge; no runtime behavior inference".into(),scope_digest:run.scope_digest.clone(),limitations:vec!["No inspected-code execution, dynamic loading or general type resolution".into()]};
            f.id = f.identity()?;
            if !findings.contains_key(&f.id) {
                charge(&f, &mut output_bytes)?;
                findings.insert(f.id.clone(), f);
            }
        }
    }
    let record = ReviewRecord {
        admission,
        run,
        findings: findings.into_values().collect(),
    };
    record.validate()?;
    let mut report = Report {
        schema: VERSION.into(),
        policy: policy.clone(),
        policy_digest,
        analysis_digest: outcome.report.digest,
        status,
        results,
        record,
        digest: String::new(),
    };
    seal(&mut report)?;
    ensure!(
        store.get(packet_id)?.digest == report.record.admission.digest,
        "fitness_admission_changed"
    );
    Ok(report)
}
impl Report {
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        bounded(self, false)?;
        ensure!(
            self == &evaluate(store, &self.record.run.packet_id, &self.policy, now)?,
            "language_fitness_artifact_mismatch"
        );
        Ok(())
    }
}
