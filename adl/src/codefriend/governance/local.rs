use crate::codefriend::evidence::{
    contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
    hash,
    store::Store,
    Admission,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use syn::{ext::IdentExt, spanned::Spanned, visit::Visit};
pub const VERSION: &str = "codefriend.fitness.v1";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub id: String,
    pub kind: String,
    pub source_path: String,
    pub forbidden_prefix: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema: String,
    pub rules: Vec<Rule>,
}
impl Policy {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.schema == VERSION && !self.rules.is_empty() && self.rules.len() <= 32,
            "invalid_fitness_policy"
        );
        let mut ids = BTreeSet::new();
        for r in &self.rules {
            ensure!(
                r.kind == "forbidden_declared_use",
                "unsupported_fitness_rule"
            );
            ensure!(
                !r.id.is_empty()
                    && r.id.len() <= 64
                    && r.id
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
                    && ids.insert(&r.id),
                "invalid_or_duplicate_fitness_rule_id"
            );
            ensure!(
                r.source_path.len() <= 512
                    && r.source_path.ends_with(".rs")
                    && !r.source_path.starts_with('/')
                    && r.source_path.split('/').all(|s| !s.is_empty()
                        && s != "."
                        && s != ".."
                        && s.bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))),
                "invalid_fitness_source_path"
            );
            let parts: Vec<_> = r.forbidden_prefix.split("::").collect();
            ensure!(
                !parts.is_empty()
                    && parts.len() <= 16
                    && r.forbidden_prefix.len() <= 256
                    && parts.iter().all(|p| !p.is_empty()
                        && p.bytes().enumerate().all(|(i, b)| b.is_ascii_alphabetic()
                            || b == b'_'
                            || (i > 0 && b.is_ascii_digit())))
                    && !matches!(parts[0], "self" | "super"),
                "invalid_forbidden_prefix"
            );
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pass,
    Fail,
    Error,
}
impl Status {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Pass => 0,
            Self::Fail => 1,
            Self::Error => 2,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub rule_id: String,
    pub path: String,
    pub line: usize,
    pub evidence_id: String,
    pub reference: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub schema: String,
    pub policy: Policy,
    pub policy_digest: String,
    pub status: Status,
    pub violations: Vec<Location>,
    pub errors: Vec<String>,
    pub unassessed: Vec<String>,
    pub record: ReviewRecord,
    pub digest: String,
}
#[derive(Default)]
struct Imports {
    paths: Vec<(Vec<String>, usize, bool)>,
    unsupported: bool,
}
impl Imports {
    fn tree(&mut self, prefix: Vec<String>, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Path(p) => {
                let mut parts = prefix;
                parts.push(p.ident.unraw().to_string());
                self.tree(parts, &p.tree);
            }
            syn::UseTree::Name(n) => {
                let mut p = prefix;
                p.push(n.ident.unraw().to_string());
                self.paths.push((p, n.span().start().line, false));
            }
            syn::UseTree::Rename(n) => {
                let mut p = prefix;
                p.push(n.ident.unraw().to_string());
                self.paths.push((p, n.span().start().line, false));
            }
            syn::UseTree::Group(g) => {
                for item in &g.items {
                    self.tree(prefix.clone(), item);
                }
            }
            syn::UseTree::Glob(g) => self.paths.push((prefix, g.span().start().line, true)),
        }
    }
}
impl<'ast> Visit<'ast> for Imports {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        self.tree(Vec::new(), &item.tree);
    }
    fn visit_macro(&mut self, _: &'ast syn::Macro) {
        self.unsupported = true;
    }
}
/// The policy is data; no source script, compiler, macro or provider is executed.
pub fn local_fitness_runner(store: &Store, packet_id: &str, policy: Policy) -> Result<Report> {
    policy.validate()?;
    evaluate(store.get(packet_id)?, policy)
}
fn evaluate(admission: Admission, policy: Policy) -> Result<Report> {
    admission.validate()?;
    policy.validate()?;
    let mut errors = Vec::new();
    let mut violations = Vec::new();
    if admission.packet.completeness != "complete_scoped_acquisition" {
        errors.push("incomplete_admitted_evidence".into());
    }
    for rule in &policy.rules {
        if !admission.packet.scope.analysis.contains(&rule.source_path) {
            errors.push(format!("required_analysis_missing:{}", rule.id));
            continue;
        }
        let source = admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == rule.source_path)
            .and_then(|o| o.content.as_deref());
        let Some(source) = source else {
            errors.push(format!("required_evidence_omitted:{}", rule.id));
            continue;
        };
        let imports = match crate::codefriend::rust_parse::inspect(source, |file| {
            let mut imports = Imports::default();
            imports.visit_file(file);
            imports
        }) {
            Ok(imports) => imports,
            Err(crate::codefriend::rust_parse::Error::Resource) => {
                errors.push(format!("rust_complexity_limit:{}", rule.id));
                continue;
            }
            Err(crate::codefriend::rust_parse::Error::Syntax) => {
                errors.push(format!("rust_parse_failed:{}", rule.id));
                continue;
            }
        };
        if imports.unsupported {
            errors.push(format!("macro_expansion_unassessed:{}", rule.id));
        }
        let forbidden: Vec<_> = rule.forbidden_prefix.split("::").collect();
        for (parts, line, glob) in imports.paths {
            if parts.first().is_some_and(|p| p == "self" || p == "super") {
                errors.push(format!("relative_use_unresolved:{}", rule.id));
                continue;
            }
            let p: Vec<_> = parts.iter().map(String::as_str).collect();
            if p.starts_with(&forbidden) {
                let evidence = admission
                    .evidence
                    .iter()
                    .find(|e| e.path == rule.source_path)
                    .ok_or_else(|| anyhow::anyhow!("required_evidence_identity_missing"))?;
                violations.push(Location {
                    rule_id: rule.id.clone(),
                    path: rule.source_path.clone(),
                    line,
                    evidence_id: evidence.id.clone(),
                    reference: parts.join("::"),
                });
            } else if glob && forbidden.starts_with(&p) {
                errors.push(format!("glob_use_unresolved:{}", rule.id));
            }
        }
    }
    errors.sort();
    errors.dedup();
    violations.sort();
    violations.dedup();
    let status = if !errors.is_empty() {
        Status::Error
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Fail
    };
    let policy_digest = hash(&policy)?;
    let run = Run::new(
        &admission,
        BTreeMap::from([
            ("fitness".into(), VERSION.into()),
            ("fitness-policy".into(), policy_digest.clone()),
        ]),
        "local".into(),
        if status == Status::Error {
            Completion::Incomplete
        } else {
            Completion::Complete
        },
        errors.clone(),
    )?;
    let mut findings = BTreeMap::new();
    for v in &violations {
        let mut finding=Finding{schema:CONTRACT.into(),id:String::new(),repository:run.repository.clone(),perspective:"fitness".into(),rule:"forbidden_declared_use".into(),semantic_anchor:format!("{}:{}:{}",v.rule_id,v.path,v.reference),title:"Declared import violates explicit fitness policy".into(),severity:Severity::High,rationale:format!("Remove or revise the declared import {} in {} under rule {}; exact violating lines are retained in the result.",v.reference,v.path,v.rule_id),confidence:Confidence::Known(100),evidence:vec![v.evidence_id.clone()],inference:"Observed literal use declaration; no runtime dependency or architecture quality inference".into(),scope_digest:run.scope_digest.clone(),limitations:vec!["No macro expansion or semantic Rust resolution".into()]};
        finding.id = finding.identity()?;
        findings.insert(finding.id.clone(), finding);
    }
    let record = ReviewRecord {
        admission,
        run,
        findings: findings.into_values().collect(),
    };
    record.validate()?;
    let mut r = Report {
        schema: VERSION.into(),
        policy,
        policy_digest,
        status,
        violations,
        errors,
        unassessed: vec![
            "architecture_quality".into(),
            "runtime_effects".into(),
            "macro_expansion".into(),
        ],
        record,
        digest: String::new(),
    };
    r.digest = hash(&r)?;
    Ok(r)
}
impl Report {
    pub fn validate(&self, store: &Store) -> Result<()> {
        ensure!(
            self == &local_fitness_runner(store, &self.record.run.packet_id, self.policy.clone())?,
            "fitness_artifact_mismatch"
        );
        Ok(())
    }
}
