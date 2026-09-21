//! Potential source-module impact over original admitted v2 edges, never an
//! absence, symbol-resolution, or runtime safety proof. V1 remains unchanged.
use super::structure_v2::{EdgeV2, StructureReportV2};
use crate::codefriend::evidence::{
    contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
    hash,
    store::Store,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{Read, Write},
    path::Path,
};
pub const VERSION: &str = "codefriend.impact.v2";
const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ChangeTargetV2 {
    NodeId(String),
    Path(String),
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ChangeSetV2 {
    pub schema: String,
    pub repository: String,
    pub revision: String,
    pub graph_digest: String,
    pub targets: Vec<ChangeTargetV2>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ImpactV2 {
    pub changed_node: String,
    pub dependent_node: String,
    pub path: Vec<EdgeV2>,
    pub change_digest: String,
    pub graph_digest: String,
    pub graph_revision: String,
    pub inference: String,
    pub risks: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ImpactReportV2 {
    pub schema: String,
    pub changes: ChangeSetV2,
    pub change_digest: String,
    pub graph: StructureReportV2,
    pub changed_nodes: Vec<String>,
    pub impacts: Vec<ImpactV2>,
    pub scoped_unimpacted_nodes: Vec<String>,
    pub unknowns: Vec<String>,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
struct Bounded(Vec<u8>);
impl Write for Bounded {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if b.len() > MAX_BYTES.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("impact_artifact_too_large"));
        }
        self.0.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn bytes(r: &ImpactReportV2) -> Result<Vec<u8>> {
    let mut b = Bounded(vec![]);
    serde_json::to_writer(&mut b, r)?;
    Ok(b.0)
}
fn live(store: &Store, g: &StructureReportV2) -> Result<()> {
    ensure!(
        store.get(&g.record.admission.packet.packet_id)? == g.record.admission,
        "impact_admission_changed"
    );
    Ok(())
}
const INFERENCE:&str="Potential impact inferred from known admitted import paths. Declared changes are operator input, not observed diffs; no runtime or semantic safety proof.";
pub fn change_impact_reporter_v2(
    store: &Store,
    graph: StructureReportV2,
    mut changes: ChangeSetV2,
    now: u64,
) -> Result<ImpactReportV2> {
    graph.validate(store, now)?;

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
    changes.targets.sort();
    changes.targets.dedup();
    let by_id: BTreeMap<_, _> = graph.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut selected = BTreeSet::new();
    for target in &changes.targets {
        let id = match target {
            ChangeTargetV2::NodeId(id) => {
                ensure!(by_id.contains_key(id), "change_node_missing");
                id.clone()
            }
            ChangeTargetV2::Path(path) => {
                crate::codefriend::ingestion::validate_path(path)?;
                let nodes: Vec<_> = graph.nodes.iter().filter(|n| &n.path == path).collect();
                ensure!(nodes.len() == 1, "change_path_missing_or_ambiguous");
                nodes[0].id.clone()
            }
        };
        selected.insert(id);
    }
    let change_digest = hash(&changes)?;
    let mut incoming: BTreeMap<String, Vec<&EdgeV2>> = BTreeMap::new();
    for edge in &graph.edges {
        if edge.from != edge.to {
            incoming.entry(edge.to.clone()).or_default().push(edge);
        }
    }
    for edges in incoming.values_mut() {
        edges.sort_by(|a, b| {
            (
                &a.from,
                &a.to,
                a.span.start_byte,
                a.span.end_byte,
                &a.spelling,
            )
                .cmp(&(
                    &b.from,
                    &b.to,
                    b.span.start_byte,
                    b.span.end_byte,
                    &b.spelling,
                ))
        });
    }
    let mut impacts = Vec::new();
    let mut witness_bytes = 0usize;
    for changed in &selected {
        let mut queue = VecDeque::from([changed.clone()]);
        let mut seen = BTreeSet::from([changed.clone()]);
        // At most one next edge per admitted node. Reconstruct witnesses only
        // when emitting; queue storage never duplicates whole source-edge paths.
        let mut next: BTreeMap<String, &EdgeV2> = BTreeMap::new();
        while let Some(node) = queue.pop_front() {
            for edge in incoming.get(&node).into_iter().flatten() {
                if !seen.insert(edge.from.clone()) {
                    continue;
                }
                next.insert(edge.from.clone(), edge);
                queue.push_back(edge.from.clone());
                let mut path = Vec::new();
                let mut cursor = edge.from.as_str();
                while cursor != changed {
                    let step = next
                        .get(cursor)
                        .ok_or_else(|| anyhow::anyhow!("impact_witness_missing"))?;
                    witness_bytes = witness_bytes
                        .checked_add(
                            step.spelling.len()
                                + step.from.len()
                                + step.to.len()
                                + step.location.path.len()
                                + step.location.evidence_id.len()
                                + 256,
                        )
                        .ok_or_else(|| anyhow::anyhow!("impact_artifact_too_large"))?;
                    ensure!(witness_bytes <= MAX_BYTES, "impact_artifact_too_large");
                    path.push((**step).clone());
                    cursor = &step.to;
                    ensure!(path.len() <= graph.nodes.len(), "impact_witness_cycle");
                }
                ensure!(impacts.len() < 512, "impact_result_limit");
                impacts.push(ImpactV2{changed_node:changed.clone(),dependent_node:edge.from.clone(),path,change_digest:change_digest.clone(),graph_digest:graph.digest.clone(),graph_revision:graph.record.run.revision.clone(),inference:INFERENCE.into(),risks:vec!["Unknown imports may introduce additional impacts outside this witness set.".into()]});
            }
        }
    }
    impacts.sort_by(|a, b| {
        (&a.changed_node, &a.dependent_node).cmp(&(&b.changed_node, &b.dependent_node))
    });
    let run = Run::new(
        &graph.record.admission,
        BTreeMap::from([
            ("architecture".into(), VERSION.into()),
            ("architecture-graph".into(), graph.digest.clone()),
            ("declared-change".into(), change_digest.clone()),
        ]),
        "none-local-syntax".into(),
        Completion::Incomplete,
        vec!["partial_admitted_module_impact".into()],
    )?;
    let mut record = ReviewRecord {
        admission: graph.record.admission.clone(),
        run,
        findings: vec![],
    };
    for impact in &impacts {
        let changed = by_id[&impact.changed_node];
        let dependent = by_id[&impact.dependent_node];
        let evidence = impact
            .path
            .iter()
            .map(|e| e.location.evidence_id.clone())
            .chain([changed.location.evidence_id.clone()])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut finding = Finding {
            schema: CONTRACT.into(),
            id: String::new(),
            repository: record.run.repository.clone(),
            perspective: "architecture".into(),
            rule: "admitted_change_impact".into(),
            semantic_anchor: hash(&(VERSION, &impact.changed_node, &impact.dependent_node))?,
            title: "Declared change has an admitted import dependent".into(),
            severity: Severity::Medium,
            rationale: format!(
                "{} has a {}-edge admitted import witness to declared changed source {}.",
                dependent.path,
                impact.path.len(),
                changed.path
            ),
            confidence: Confidence::Unknown,
            evidence,
            inference: INFERENCE.into(),
            scope_digest: record.run.scope_digest.clone(),
            limitations: impact.risks.clone(),
        };
        finding.id = finding.identity()?;
        record.findings.push(finding);
    }
    record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    record.validate()?;
    let mut unknowns = graph
        .unknowns
        .iter()
        .map(|u| format!("{}: {}", u.path, u.reason))
        .collect::<BTreeSet<_>>();
    unknowns.insert("Incomplete admitted graph cannot establish scoped-unimpacted nodes.".into());
    let mut report = ImpactReportV2 {
        schema: VERSION.into(),
        changes,
        change_digest,
        graph,
        changed_nodes: selected.into_iter().collect(),
        impacts,
        scoped_unimpacted_nodes: vec![],
        unknowns: unknowns.into_iter().collect(),
        analysis_complete: false,
        record,
        digest: String::new(),
    };
    bytes(&report)?;
    report.digest = hash(&report)?;
    bytes(&report)?;
    live(store, &report.graph)?;
    Ok(report)
}
impl ImpactReportV2 {
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        bytes(self)?;
        let expected =
            change_impact_reporter_v2(store, self.graph.clone(), self.changes.clone(), now)?;
        ensure!(*self == expected, "impact_artifact_mismatch");
        live(store, &self.graph)
    }
}
pub fn write_report_v2(
    report: &ImpactReportV2,
    store: &Store,
    path: &Path,
    now: u64,
) -> Result<()> {
    live(store, &report.graph)?;
    report.validate(store, now)?;
    super::structure::safe_artifact_path(path)?;
    let data = bytes(report)?;
    live(store, &report.graph)?;
    let mut options = std::fs::OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    let result = (|| -> Result<()> {
        file.write_all(&data)?;
        file.sync_all()?;
        live(store, &report.graph)
    })();
    if result.is_err() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let (Ok(a), Ok(b)) = (file.metadata(), std::fs::symlink_metadata(path)) {
                if b.file_type().is_file() && a.dev() == b.dev() && a.ino() == b.ino() {
                    std::fs::remove_file(path)?;
                }
            }
        }
    }
    result
}
pub fn read_report_v2(store: &Store, path: &Path, now: u64) -> Result<ImpactReportV2> {
    super::structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "impact_input_not_regular"
    );
    let mut data = vec![];
    std::fs::File::open(path)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    ensure!(data.len() <= MAX_BYTES, "impact_artifact_too_large");
    let report: ImpactReportV2 = serde_json::from_slice(&data)?;
    report.validate(store, now)?;
    live(store, &report.graph)?;
    Ok(report)
}
