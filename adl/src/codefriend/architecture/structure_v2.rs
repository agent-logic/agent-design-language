//! Four-language admitted module architecture. This additive schema does not
//! change v1 bytes or claim compiler, symbol, or runtime completeness.
use super::structure::Location;
use crate::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        hash,
        store::Store,
        Admission,
    },
    language::{self, AnalysisPolicy, AnalysisReport, Language, Span},
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{Read, Write},
    path::Path,
};
pub const VERSION: &str = "codefriend.structure.v2";
const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoundaryPolicyV2 {
    pub schema: String,
    pub analysis: AnalysisPolicy,
    pub coupling_threshold: usize,
}
impl BoundaryPolicyV2 {
    pub fn validate(&self, a: &Admission) -> Result<()> {
        ensure!(
            self.schema == VERSION,
            "unsupported_boundary_policy_version"
        );
        self.analysis.validate(a)?;
        ensure!(
            self.analysis.files.len() <= 512 && (1..=512).contains(&self.coupling_threshold),
            "invalid_graph_policy_bounds"
        );
        ensure!(
            self.analysis.layers.keys().eq(self.analysis.files.keys()),
            "architecture_layer_scope_incomplete"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NodeV2 {
    pub id: String,
    pub language: Language,
    pub project_root: String,
    pub path: String,
    pub kind: String,
    pub layer: String,
    pub location: Location,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EdgeV2 {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub spelling: String,
    pub span: Span,
    pub location: Location,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UnknownV2 {
    pub path: String,
    pub evidence_id: Option<String>,
    pub span: Option<Span>,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StructureReportV2 {
    pub schema: String,
    pub policy: BoundaryPolicyV2,
    pub policy_digest: String,
    pub analysis: AnalysisReport,
    pub nodes: Vec<NodeV2>,
    pub edges: Vec<EdgeV2>,
    pub unknowns: Vec<UnknownV2>,
    pub coverage_scope: String,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
struct BoundedBytes(Vec<u8>);
impl Write for BoundedBytes {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > MAX_BYTES.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("structure_artifact_too_large"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn bytes(value: &StructureReportV2) -> Result<Vec<u8>> {
    let mut out = BoundedBytes(Vec::new());
    serde_json::to_writer(&mut out, value)?;
    Ok(out.0)
}
fn live(store: &Store, original: &Admission) -> Result<()> {
    ensure!(
        store.get(&original.packet.packet_id)? == *original,
        "architecture_admission_changed"
    );
    Ok(())
}
fn location(a: &Admission, path: &str, span: Span) -> Result<Location> {
    let object = a
        .packet
        .objects
        .iter()
        .find(|o| o.path == path)
        .and_then(|o| o.content.as_ref())
        .ok_or_else(|| anyhow::anyhow!("architecture_source_missing"))?;
    span.validate(object)?;
    let e = a
        .evidence
        .iter()
        .find(|e| e.path == path)
        .ok_or_else(|| anyhow::anyhow!("architecture_evidence_missing"))?;
    Ok(Location {
        path: path.into(),
        line: object[..span.start_byte]
            .bytes()
            .filter(|b| *b == b'\n')
            .count()
            + 1,
        evidence_id: e.id.clone(),
    })
}
/// Reopens the original source owner; an externally supplied AnalysisReport is
/// deliberately not an accepted operational input.
pub fn repository_structure_reporter_v2(
    store: &Store,
    packet_id: &str,
    policy: BoundaryPolicyV2,
    now: u64,
) -> Result<StructureReportV2> {
    let a = store.get(packet_id)?;
    policy.validate(&a)?;
    let analysis = language::owner::analyze(store, packet_id, &policy.analysis, now)?;
    ensure!(
        analysis.admission_digest == a.digest,
        "architecture_admission_changed"
    );

    let mut nodes = Vec::new();
    let mut unknowns = Vec::new();
    for f in &analysis.files {
        let c = &f.coverage;
        for d in &c.diagnostics {
            unknowns.push(UnknownV2 {
                path: c.path.clone(),
                evidence_id: c.evidence_id.clone(),
                span: d.span,
                reason: d.code.clone(),
            });
        }
        if c.evidence_id.is_none() {
            continue;
        }
        let root = policy
            .analysis
            .roots
            .iter()
            .filter(|r| {
                r.language == c.language
                    && (r.root == "." || c.path.starts_with(&format!("{}/", r.root)))
            })
            .max_by_key(|r| r.root.len())
            .ok_or_else(|| anyhow::anyhow!("architecture_root_missing"))?;
        let id = hash(&(
            VERSION,
            &a.packet.repository,
            c.language,
            &root.root,
            &c.path,
        ))?;
        nodes.push(NodeV2 {
            id,
            language: c.language,
            project_root: root.root.clone(),
            path: c.path.clone(),
            kind: "admitted_source_module".into(),
            layer: policy.analysis.layers[&c.path].clone(),
            location: location(
                &a,
                &c.path,
                Span {
                    start_byte: 0,
                    end_byte: 0,
                },
            )?,
        });
    }
    ensure!(unknowns.len() <= 8192, "architecture_unknown_limit");
    let by_path: BTreeMap<_, _> = nodes.iter().map(|n| (n.path.as_str(), n)).collect();
    let mut edges = Vec::new();
    for f in &analysis.files {
        for import in &f.facts.imports {
            let Some(target) = &import.target_path else {
                continue;
            };
            let from = by_path
                .get(f.coverage.path.as_str())
                .ok_or_else(|| anyhow::anyhow!("architecture_import_source_missing"))?;
            let to = by_path
                .get(target.as_str())
                .ok_or_else(|| anyhow::anyhow!("architecture_import_target_missing"))?;
            ensure!(
                from.language == to.language && from.project_root == to.project_root,
                "architecture_import_scope_mismatch"
            );
            edges.push(EdgeV2 {
                from: from.id.clone(),
                to: to.id.clone(),
                kind: "admitted_import".into(),
                spelling: import.spelling.clone(),
                span: import.span,
                location: location(&a, &f.coverage.path, import.span)?,
            });
            ensure!(edges.len() <= 4096, "architecture_edge_limit");
        }
    }
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
    edges.dedup();
    let policy_digest = hash(&policy)?;
    let run = Run::new(
        &a,
        BTreeMap::from([
            ("architecture".into(), VERSION.into()),
            ("architecture-policy".into(), policy_digest.clone()),
            ("language-analysis-tools".into(), hash(&analysis.toolchain)?),
        ]),
        "none-local-syntax".into(),
        Completion::Incomplete,
        vec!["partial_admitted_module_architecture".into()],
    )?;
    let mut report = StructureReportV2 {
        schema: VERSION.into(),
        policy,
        policy_digest,
        analysis,
        nodes,
        edges,
        unknowns,
        coverage_scope: "admitted_module_edges_v1".into(),
        analysis_complete: false,
        record: ReviewRecord {
            admission: a,
            run,
            findings: vec![],
        },
        digest: String::new(),
    };
    findings(&mut report)?;
    report.record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    report.record.validate()?;
    // Bound materialization before hashing the complete record/source payload.
    bytes(&report)?;
    report.digest = hash(&report)?;
    bytes(&report)?;

    live(store, &report.record.admission)?;

    Ok(report)
}
fn add(
    report: &mut StructureReportV2,
    rule: &str,
    anchor: &str,
    title: &str,
    rationale: String,
    evidence: Vec<String>,
) -> Result<()> {
    ensure!(
        report.record.findings.len() < 1000,
        "architecture_finding_limit"
    );
    let mut finding=Finding {schema:CONTRACT.into(),id:String::new(),repository:report.record.run.repository.clone(),perspective:"architecture".into(),rule:rule.into(),semantic_anchor:anchor.into(),title:title.into(),severity:Severity::Medium,rationale,confidence:Confidence::Unknown,evidence:evidence.into_iter().collect::<BTreeSet<_>>().into_iter().collect(),inference:"Risk inferred from admitted static import edges and explicit boundary policy; not runtime or semantic dependency proof.".into(),scope_digest:report.record.run.scope_digest.clone(),limitations:vec!["Partial admitted module scope; unresolved imports and unsupported semantics remain unknown. No absence or safety claim.".into()]};
    finding.id = finding.identity()?;
    finding.validate(&report.record.run, &report.record.admission)?;
    report.record.findings.push(finding);
    Ok(())
}
fn findings(report: &mut StructureReportV2) -> Result<()> {
    let nodes: BTreeMap<_, _> = report
        .nodes
        .iter()
        .map(|n| (n.id.clone(), n.clone()))
        .collect();
    let mut adjacency: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for edge in report.edges.clone() {
        // File nodes collapse inline modules. Retain the original import edge,
        // but it cannot establish a cross-file cycle or fan-out target.
        if edge.from != edge.to {
            adjacency
                .entry(edge.from.clone())
                .or_default()
                .insert(edge.to.clone());
        }
        let from = &nodes[&edge.from];
        let to = &nodes[&edge.to];
        if from.layer != to.layer
            && !report
                .policy
                .analysis
                .allowed
                .contains(&(from.layer.clone(), to.layer.clone()))
        {
            let anchor = hash(&(VERSION, "boundary", &from.id, &to.id))?;
            // One finding per source-target boundary; repeated import spans remain in edges.
            if !report
                .record
                .findings
                .iter()
                .any(|f| f.semantic_anchor == anchor)
            {
                add(
                    report,
                    "admitted_boundary_violation",
                    &anchor,
                    "Admitted import crosses a forbidden layer boundary",
                    format!(
                        "{} imports {} across declared layers {} -> {}.",
                        from.path, to.path, from.layer, to.layer
                    ),
                    vec![edge.location.evidence_id, to.location.evidence_id.clone()],
                )?;
            }
        }
    }
    for (origin, targets) in &adjacency {
        if targets.len() > report.policy.coupling_threshold {
            let mut evidence = vec![nodes[origin].location.evidence_id.clone()];
            evidence.extend(
                targets
                    .iter()
                    .map(|t| nodes[t].location.evidence_id.clone()),
            );
            add(
                report,
                "admitted_coupling_threshold",
                origin,
                "Admitted import fan-out exceeds policy",
                format!(
                    "{} has {} distinct admitted import targets; policy threshold is {}.",
                    nodes[origin].path,
                    targets.len(),
                    report.policy.coupling_threshold
                ),
                evidence,
            )?;
        }
        // Deterministic breadth-first cycle witness, bounded by admitted nodes.
        let mut queue = VecDeque::from([(origin.clone(), vec![origin.clone()])]);
        let mut seen = BTreeSet::from([origin.clone()]);
        let mut witness = None;
        while let Some((node, path)) = queue.pop_front() {
            for next in adjacency.get(&node).into_iter().flatten() {
                if next == origin {
                    let mut cycle = path.clone();
                    cycle.push(origin.clone());
                    witness = Some(cycle);
                    break;
                }
                if seen.insert(next.clone()) {
                    let mut p = path.clone();
                    p.push(next.clone());
                    queue.push_back((next.clone(), p));
                }
            }
            if witness.is_some() {
                break;
            }
        }
        if let Some(path) = witness {
            let evidence = path
                .iter()
                .map(|id| nodes[id].location.evidence_id.clone())
                .collect();
            add(
                report,
                "admitted_import_cycle",
                origin,
                "Admitted imports contain a cycle",
                format!(
                    "Known import witness: {}.",
                    path.iter()
                        .map(|id| nodes[id].path.as_str())
                        .collect::<Vec<_>>()
                        .join(" -> ")
                ),
                evidence,
            )?;
        }
    }
    Ok(())
}
impl StructureReportV2 {
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        bytes(self)?;
        let expected = repository_structure_reporter_v2(
            store,
            &self.record.admission.packet.packet_id,
            self.policy.clone(),
            now,
        )?;
        ensure!(*self == expected, "structure_artifact_mismatch");

        live(store, &self.record.admission)?;
        Ok(())
    }
}
pub fn write_report_v2(
    report: &StructureReportV2,
    store: &Store,
    path: &Path,
    now: u64,
) -> Result<()> {
    report.validate(store, now)?;
    super::structure::safe_artifact_path(path)?;
    let data = bytes(report)?;

    live(store, &report.record.admission)?;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    let result = (|| -> Result<()> {
        file.write_all(&data)?;
        file.sync_all()?;
        live(store, &report.record.admission)?;
        Ok(())
    })();
    if result.is_err() {
        // create_new established ownership. Do not remove a replacement object.
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let (Ok(opened), Ok(current)) = (file.metadata(), std::fs::symlink_metadata(path)) {
                if current.file_type().is_file()
                    && opened.dev() == current.dev()
                    && opened.ino() == current.ino()
                {
                    std::fs::remove_file(path)?;
                }
            }
        }
    }
    result
}
pub fn read_report_v2(store: &Store, path: &Path, now: u64) -> Result<StructureReportV2> {
    super::structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "structure_input_not_regular"
    );
    let mut data = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    ensure!(data.len() <= MAX_BYTES, "structure_artifact_too_large");
    let report: StructureReportV2 = serde_json::from_slice(&data)?;
    report.validate(store, now)?;

    live(store, &report.record.admission)?;

    Ok(report)
}
