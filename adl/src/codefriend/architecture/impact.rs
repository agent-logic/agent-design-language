//! Potential module impact from the admitted syntactic graph, never runtime safety proof.
use super::structure::{self, Edge, StructureReport};
use crate::codefriend::evidence::{
    contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
    hash,
    store::Store,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const VERSION: &str = "codefriend.impact.v1";
const INFERENCE: &str = "Potential impact inferred from incoming syntactic dependency paths. Declared changes are operator input, not an observed diff; paths do not prove runtime effects or safety.";
const ARTIFACT_LIMIT: u64 = 32 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(
    tag = "kind",
    content = "name",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ChangeTarget {
    Module(String),
    Symbol(String),
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChangeSet {
    pub schema: String,
    pub repository: String,
    pub revision: String,
    pub graph_digest: String,
    pub targets: Vec<ChangeTarget>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Impact {
    pub changed_node: String,
    pub dependent_node: String,
    /// Ordered from dependent to changed node. One deterministic shortest witness per pair.
    pub path: Vec<Edge>,
    pub change_digest: String,
    pub graph_digest: String,
    pub graph_revision: String,
    pub inference: String,
    pub risks: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ImpactReport {
    pub schema: String,
    pub changes: ChangeSet,
    pub change_digest: String,
    pub graph: StructureReport,
    pub changed_nodes: Vec<String>,
    pub impacts: Vec<Impact>,
    /// Only scoped nodes without a known dependency path, and only when analysis is complete.
    /// This is not an assertion of semantic independence or runtime safety.
    pub scoped_unimpacted_nodes: Vec<String>,
    pub unknowns: Vec<String>,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
/// Live graph validation enforces admission deletion/expiry and detects topology tampering.
pub fn change_impact_reporter(
    store: &Store,
    graph: StructureReport,
    mut changes: ChangeSet,
) -> Result<ImpactReport> {
    graph.validate(store)?;
    ensure!(changes.schema == VERSION, "unsupported_change_version");
    ensure!(
        changes.repository == graph.record.run.repository,
        "change_repository_mismatch"
    );
    ensure!(
        changes.revision == graph.record.run.revision,
        "stale_change_revision"
    );
    ensure!(
        changes.graph_digest == graph.digest,
        "stale_change_graph_digest"
    );
    ensure!(
        !changes.targets.is_empty() && changes.targets.len() <= 64,
        "change_target_bounds_exceeded"
    );
    for target in &changes.targets {
        let (ChangeTarget::Module(name) | ChangeTarget::Symbol(name)) = target;
        ensure!(
            !name.is_empty()
                && name.len() <= 512
                && name.split("::").all(|p| !p.is_empty()
                    && p.bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_-#".contains(&b))),
            "invalid_change_target"
        );
    }
    changes.targets.sort();
    ensure!(
        changes.targets.windows(2).all(|w| w[0] != w[1]),
        "duplicate_change_target"
    );
    let change_digest = hash(&changes)?;
    let mut unknowns = Vec::new();
    if !graph.analysis_complete {
        unknowns.push("partial_structure_graph: inspect graph.unknowns and admission completeness; absent paths cannot prove no impact".into());
    }
    let nodes: BTreeMap<_, _> = graph.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut roots = BTreeSet::new();
    for target in &changes.targets {
        match target {
            ChangeTarget::Module(name) => match graph.nodes.iter().find(|n| &n.module == name) {
                Some(node) => {
                    roots.insert(node.id.clone());
                }
                None => unknowns.push(format!("module_outside_graph:{name}")),
            },
            ChangeTarget::Symbol(name) => {
                unknowns.push(format!("symbol_resolution_unsupported:{name}"))
            }
        }
    }
    let mut incoming: BTreeMap<String, Vec<&Edge>> = BTreeMap::new();
    let mut outgoing: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.to.clone()).or_default().push(edge);
        outgoing
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
    }
    let mut impacts = Vec::new();
    for root in &roots {
        // Nodes reachable in both directions participate in the changed root's cycle.
        let mut forward = BTreeSet::new();
        let mut todo = outgoing.get(root).cloned().unwrap_or_default();
        while let Some(next) = todo.pop() {
            if forward.insert(next.clone()) {
                todo.extend(outgoing.get(&next).cloned().unwrap_or_default());
            }
        }
        let mut paths = BTreeMap::from([(root.clone(), Vec::<Edge>::new())]);
        let mut queue = VecDeque::from([root.clone()]);
        while let Some(next) = queue.pop_front() {
            for edge in incoming.get(&next).into_iter().flatten() {
                if paths.contains_key(&edge.from) {
                    continue;
                }
                let mut path = vec![(*edge).clone()];
                path.extend(paths[&next].iter().cloned());
                ensure!(path.len() <= 512, "impact_path_bounds_exceeded");
                paths.insert(edge.from.clone(), path);
                queue.push_back(edge.from.clone());
            }
        }
        for (dependent, path) in paths {
            if &dependent == root {
                continue;
            }
            ensure!(impacts.len() < 1000, "impact_pair_bounds_exceeded");
            let mut risks = Vec::new();
            if path.len() > 1 {
                risks.push("transitive_dependency".into());
            }
            if path.iter().any(|e| matches!((&nodes[&e.from].layer, &nodes[&e.to].layer), (Some(a),Some(b)) if a != b)) { risks.push("cross_layer_dependency".into()); }
            if forward.contains(&dependent) {
                risks.push("cycle_with_changed_root".into());
            }
            impacts.push(Impact {
                changed_node: root.clone(),
                dependent_node: dependent,
                path,
                change_digest: change_digest.clone(),
                graph_digest: graph.digest.clone(),
                graph_revision: graph.record.run.revision.clone(),
                inference: INFERENCE.into(),
                risks,
            });
        }
    }
    unknowns.sort();
    let complete = unknowns.is_empty();
    let affected: BTreeSet<_> = roots
        .iter()
        .chain(impacts.iter().map(|i| &i.dependent_node))
        .collect();
    let unimpacted = if complete {
        nodes
            .keys()
            .filter(|id| !affected.contains(id))
            .cloned()
            .collect()
    } else {
        vec![]
    };
    let run = Run::new(
        &graph.record.admission,
        BTreeMap::from([
            ("architecture-impact".into(), VERSION.into()),
            ("structure".into(), structure::VERSION.into()),
            ("graph-digest".into(), graph.digest.clone()),
            ("change-digest".into(), change_digest.clone()),
        ]),
        "none-local-syntax".into(),
        if complete {
            Completion::Complete
        } else {
            Completion::Incomplete
        },
        if complete {
            vec![]
        } else {
            vec!["partial_impact_analysis".into()]
        },
    )?;
    let mut record = ReviewRecord {
        admission: graph.record.admission.clone(),
        run,
        findings: vec![],
    };
    for impact in &impacts {
        let dependent = nodes[&impact.dependent_node];
        let root = nodes[&impact.changed_node];
        let evidence = impact
            .path
            .iter()
            .map(|e| e.location.evidence_id.clone())
            .chain(std::iter::once(root.location.evidence_id.clone()))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut finding = Finding {
            schema: CONTRACT.into(), id: String::new(), repository: record.run.repository.clone(), perspective: "architecture-impact".into(), rule: "potential_module_impact".into(), semantic_anchor: format!("{}->{}",dependent.module, root.module), title: "Module may be affected by declared change".into(), severity: Severity::Info,
            rationale: format!("{} reaches changed {} through {} retained dependency edges. Change {} at graph revision {}; inspect the corresponding impact.path witness.",dependent.module,root.module,impact.path.len(),change_digest,impact.graph_revision), confidence: Confidence::Unknown, evidence, inference: INFERENCE.into(), scope_digest: record.run.scope_digest.clone(), limitations: vec!["Bounded module-level syntactic reachability only; symbols, dynamic dispatch and macros are not resolved. One shortest witness is retained, not every possible path.".into()],
        };
        finding.id = finding.identity()?;
        record.findings.push(finding);
    }
    record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    record.validate()?;
    let mut report = ImpactReport {
        schema: VERSION.into(),
        changes,
        change_digest,
        graph,
        changed_nodes: roots.into_iter().collect(),
        impacts,
        scoped_unimpacted_nodes: unimpacted,
        unknowns,
        analysis_complete: complete,
        record,
        digest: String::new(),
    };
    report.digest = hash(&report)?;
    Ok(report)
}
impl ImpactReport {
    pub fn validate(&self, store: &Store) -> Result<()> {
        let expected = change_impact_reporter(store, self.graph.clone(), self.changes.clone())?;
        ensure!(*self == expected, "impact_artifact_mismatch");
        Ok(())
    }
}
pub fn write_report(report: &ImpactReport, store: &Store, path: &std::path::Path) -> Result<()> {
    use std::io::Write;
    report.validate(store)?;
    structure::safe_artifact_path(path)?;
    let bytes = serde_json::to_vec(report)?;
    ensure!(
        bytes.len() as u64 <= ARTIFACT_LIMIT,
        "impact_artifact_too_large"
    );
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|_| anyhow::anyhow!("impact_output_unavailable"))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}
pub fn read_report(store: &Store, path: &std::path::Path) -> Result<ImpactReport> {
    use std::io::Read;
    structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "impact_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(ARTIFACT_LIMIT + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 <= ARTIFACT_LIMIT,
        "impact_artifact_too_large"
    );
    let report: ImpactReport =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_impact_artifact"))?;
    report.validate(store)?;
    Ok(report)
}
