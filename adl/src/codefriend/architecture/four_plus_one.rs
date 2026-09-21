//! Evidence-bound 4+1 package contracts. A source declaration is not observed topology.
use crate::codefriend::{
    evidence::{hash, Admission},
    ingestion::unsafe_content,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SCHEMA: &str = "codefriend.four_plus_one.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum View {
    Logical,
    Development,
    Process,
    Deployment,
}
impl View {
    pub const ALL: [Self; 4] = [
        Self::Logical,
        Self::Development,
        Self::Process,
        Self::Deployment,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Basis {
    SourceDeclaration,
    Inference,
    Assumption,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    pub evidence_id: String,
    pub path: String,
    pub first_line: usize,
    pub last_line: usize,
    /// Exact admitted text. Its presence supports provenance, not semantic entailment.
    pub excerpt: String,
}
impl Citation {
    pub fn validate(&self, admission: &Admission) -> Result<()> {
        ensure!(
            admission
                .evidence
                .iter()
                .any(|e| e.id == self.evidence_id && e.path == self.path),
            "four_plus_one_evidence_missing"
        );
        let content = admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == self.path)
            .and_then(|o| o.content.as_ref())
            .ok_or_else(|| anyhow::anyhow!("four_plus_one_source_missing"))?;
        let lines: Vec<_> = content.lines().collect();
        ensure!(
            self.first_line > 0
                && self.last_line >= self.first_line
                && self.last_line <= lines.len(),
            "four_plus_one_invalid_span"
        );
        ensure!(
            lines[self.first_line - 1..self.last_line].join("\n") == self.excerpt,
            "four_plus_one_excerpt_mismatch"
        );
        text(&self.excerpt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub responsibility: String,
    pub basis: Basis,
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub description: String,
    pub basis: Basis,
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchitectureView {
    pub entities: Vec<String>,
    pub relationships: Vec<Relationship>,
    pub missing_inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub description: String,
    pub failure_recovery: bool,
    pub basis: Basis,
    pub citations: Vec<Citation>,
    /// Ordered entity IDs encountered by the scenario in each view.
    pub trace: BTreeMap<View, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Conflict {
    pub description: String,
    pub entities: Vec<String>,
    pub citations: Vec<Citation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    pub schema: String,
    pub repository: String,
    pub revision: String,
    pub packet_id: String,
    pub admission_digest: String,
    pub entities: Vec<Entity>,
    pub views: BTreeMap<View, ArchitectureView>,
    pub scenarios: Vec<Scenario>,
    pub conflicts: Vec<Conflict>,
    pub missing_inputs: Vec<String>,
    /// Coverage only; never a claim of verified running topology or semantic correctness.
    pub complete: bool,
    pub digest: String,
}

fn text(value: &str) -> Result<()> {
    ensure!(
        !value.trim().is_empty() && value.len() <= 8192 && !unsafe_content("", value),
        "four_plus_one_unsafe_text"
    );
    Ok(())
}
fn identifier(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty()
            && value.len() <= 96
            && value
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-".contains(&b)),
        "four_plus_one_invalid_id"
    );
    Ok(())
}
fn citations(values: &[Citation], admission: &Admission) -> Result<()> {
    ensure!(
        !values.is_empty() && values.len() <= 32,
        "four_plus_one_citations_required"
    );
    for value in values {
        value.validate(admission)?;
    }
    Ok(())
}
impl Package {
    pub fn expected_digest(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.digest.clear();
        hash(&copy)
    }
    pub fn coverage_complete(&self) -> bool {
        self.conflicts.is_empty()
            && self.missing_inputs.is_empty()
            && !self.scenarios.is_empty()
            && View::ALL.iter().all(|v| {
                self.views.get(v).is_some_and(|view| {
                    !view.entities.is_empty()
                        && !view.relationships.is_empty()
                        && view.missing_inputs.is_empty()
                })
            })
            && self.scenarios.iter().all(|s| {
                View::ALL
                    .iter()
                    .all(|v| s.trace.get(v).is_some_and(|t| !t.is_empty()))
            })
            && self.entities.iter().all(|e| e.basis != Basis::Assumption)
            && self
                .views
                .values()
                .flat_map(|v| &v.relationships)
                .all(|r| r.basis != Basis::Assumption)
            && self.scenarios.iter().all(|s| s.basis != Basis::Assumption)
    }
    pub fn validate(&self, admission: &Admission, now: u64) -> Result<()> {
        admission.validate()?;
        ensure!(
            now < admission.expires_at,
            "four_plus_one_admission_expired"
        );
        ensure!(
            self.schema == SCHEMA
                && self.repository == admission.packet.repository
                && self.revision == admission.packet.revision
                && self.packet_id == admission.packet.packet_id
                && self.admission_digest == admission.digest,
            "four_plus_one_source_binding_mismatch"
        );
        ensure!(
            self.entities.len() <= 256
                && self.scenarios.len() <= 32
                && self.conflicts.len() <= 64
                && self.missing_inputs.len() <= 64
                && self.views.len() == 4,
            "four_plus_one_bounds"
        );
        ensure!(
            View::ALL.iter().all(|v| self.views.contains_key(v)),
            "four_plus_one_missing_view"
        );
        let mut ids = BTreeSet::new();
        for e in &self.entities {
            identifier(&e.id)?;
            ensure!(ids.insert(&e.id), "four_plus_one_duplicate_entity");
            ensure!(e.name.chars().count() <= 120, "four_plus_one_name_too_long");
            text(&e.name)?;
            text(&e.responsibility)?;
            citations(&e.citations, admission)?;
        }
        for view in self.views.values() {
            ensure!(
                (!view.entities.is_empty() && !view.relationships.is_empty())
                    || !view.missing_inputs.is_empty(),
                "four_plus_one_missing_view_explanation"
            );
            ensure!(
                view.entities.len() <= 256
                    && view.relationships.len() <= 512
                    && view.missing_inputs.len() <= 64,
                "four_plus_one_view_bounds"
            );
            let members: BTreeSet<_> = view.entities.iter().collect();
            ensure!(
                members.len() == view.entities.len() && members.iter().all(|id| ids.contains(*id)),
                "four_plus_one_unknown_or_duplicate_entity"
            );
            for r in &view.relationships {
                ensure!(
                    members.contains(&r.from) && members.contains(&r.to),
                    "four_plus_one_unknown_endpoint"
                );
                text(&r.description)?;
                citations(&r.citations, admission)?;
            }
            for missing in &view.missing_inputs {
                text(missing)?;
            }
        }
        ensure!(
            !self.scenarios.is_empty() || !self.missing_inputs.is_empty(),
            "four_plus_one_missing_scenario_explanation"
        );
        let mut scenarios = BTreeSet::new();
        for s in &self.scenarios {
            identifier(&s.id)?;
            ensure!(scenarios.insert(&s.id), "four_plus_one_duplicate_scenario");
            text(&s.description)?;
            citations(&s.citations, admission)?;
            for v in View::ALL {
                ensure!(
                    s.trace.get(&v).is_some_and(|t| !t.is_empty())
                        || !self.views[&v].missing_inputs.is_empty()
                        || !self.missing_inputs.is_empty(),
                    "four_plus_one_missing_trace_explanation"
                );
            }
            for (v, trace) in &s.trace {
                ensure!(
                    trace.len() <= 256
                        && trace.iter().all(|id| self.views[v].entities.contains(id)),
                    "four_plus_one_unknown_trace_entity"
                );
            }
        }
        for conflict in &self.conflicts {
            text(&conflict.description)?;
            citations(&conflict.citations, admission)?;
            ensure!(
                !conflict.entities.is_empty()
                    && conflict.entities.len() <= 256
                    && conflict.entities.iter().all(|id| ids.contains(id)),
                "four_plus_one_conflict_entity"
            );
        }
        for missing in &self.missing_inputs {
            text(missing)?;
        }
        ensure!(
            self.complete == self.coverage_complete(),
            "four_plus_one_false_completeness"
        );
        ensure!(
            self.digest == self.expected_digest()?,
            "four_plus_one_digest_mismatch"
        );
        Ok(())
    }
}

/// Generate the structural portion through the existing graph owner. Runtime,
/// deployment and scenario gaps are explicit rather than guessed from imports.
pub fn from_structure(
    store: &crate::codefriend::evidence::store::Store,
    graph: &super::artifact::StructureArtifact,
    now: u64,
) -> Result<Package> {
    use super::artifact::StructureArtifact;
    graph.validate(store, now)?;
    let admission = store.get(&graph.record().run.packet_id)?;
    let source_nodes: Vec<(&str, &str, &super::structure::Location)> = match graph {
        StructureArtifact::V1(g) => g
            .nodes
            .iter()
            .map(|n| (n.id.as_str(), n.module.as_str(), &n.location))
            .collect(),
        StructureArtifact::V2(g) => g
            .nodes
            .iter()
            .map(|n| (n.id.as_str(), n.path.as_str(), &n.location))
            .collect(),
    };
    ensure!(source_nodes.len() <= 256, "four_plus_one_bounds");
    let cite = |location: &super::structure::Location| -> Result<Citation> {
        let content = admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == location.path)
            .and_then(|o| o.content.as_ref())
            .ok_or_else(|| anyhow::anyhow!("four_plus_one_source_missing"))?;
        let excerpt = content
            .lines()
            .nth(location.line.saturating_sub(1))
            .ok_or_else(|| anyhow::anyhow!("four_plus_one_invalid_span"))?;
        Ok(Citation {
            evidence_id: location.evidence_id.clone(),
            path: location.path.clone(),
            first_line: location.line,
            last_line: location.line,
            excerpt: excerpt.into(),
        })
    };
    let mut mapping = BTreeMap::new();
    let mut entities = Vec::new();
    for (index, (original, name, location)) in source_nodes.iter().enumerate() {
        let id = format!("module_{index}");
        ensure!(
            mapping.insert(*original, id.clone()).is_none(),
            "four_plus_one_duplicate_entity"
        );
        entities.push(Entity {
            id,
            name: (*name).into(),
            responsibility:
                "Source module; domain responsibility requires additional admitted evidence".into(),
            basis: Basis::SourceDeclaration,
            citations: vec![cite(location)?],
        });
    }
    let source_edges: Vec<(&str, &str, &str, &super::structure::Location)> = match graph {
        StructureArtifact::V1(g) => g
            .edges
            .iter()
            .map(|e| (e.from.as_str(), e.to.as_str(), e.kind.as_str(), &e.location))
            .collect(),
        StructureArtifact::V2(g) => g
            .edges
            .iter()
            .map(|e| (e.from.as_str(), e.to.as_str(), e.kind.as_str(), &e.location))
            .collect(),
    };
    ensure!(source_edges.len() <= 512, "four_plus_one_view_bounds");
    let source_edges_were_empty = source_edges.is_empty();
    let mut relationships = Vec::new();
    for (from, to, kind, location) in source_edges {
        relationships.push(Relationship {
            from: mapping
                .get(from)
                .ok_or_else(|| anyhow::anyhow!("four_plus_one_unknown_endpoint"))?
                .clone(),
            to: mapping
                .get(to)
                .ok_or_else(|| anyhow::anyhow!("four_plus_one_unknown_endpoint"))?
                .clone(),
            description: kind.into(),
            basis: Basis::SourceDeclaration,
            citations: vec![cite(location)?],
        });
    }
    let mut views: BTreeMap<_,_> = View::ALL.into_iter().map(|v| (v,ArchitectureView {
        entities: vec![], relationships: vec![], missing_inputs: vec![match v {
            View::Logical => "Admit domain responsibilities and component boundaries; module imports alone do not establish a logical view.",
            View::Development => "Inspect graph coverage and unresolved source relationships.",
            View::Process => "Admit runtime interaction, concurrency and failure/recovery evidence.",
            View::Deployment => "Admit deployment declarations mapping services to nodes; declarations are not verified running topology.",
        }.into()],
    })).collect();
    views.insert(View::Development, ArchitectureView {
        entities: entities.iter().map(|e| e.id.clone()).collect(), relationships,
        missing_inputs: if !entities.is_empty() && !source_edges_were_empty && graph.record().run.completion == crate::codefriend::evidence::contracts::Completion::Complete {
            vec![]
        } else { vec!["Source graph has incomplete coverage; inspect the retained structure report's unknowns.".into()] },
    });
    let mut package = Package {
        schema: SCHEMA.into(), repository: admission.packet.repository.clone(), revision: admission.packet.revision.clone(),
        packet_id: admission.packet.packet_id.clone(),
        admission_digest: admission.digest.clone(), entities, views, scenarios: vec![], conflicts: vec![],
        missing_inputs: vec!["Admit representative scenarios and evidence linking their steps across all four views, including failure/recovery where applicable.".into()],
        complete: false, digest: String::new(),
    };
    package.digest = package.expected_digest()?;
    package.validate(&admission, now)?;
    ensure!(
        store.get(&admission.packet.packet_id)? == admission,
        "four_plus_one_admission_changed"
    );
    Ok(package)
}

pub mod generation;

pub mod render;
