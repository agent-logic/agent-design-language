//! Structural drift uses shared Finding identities and CF-MEMORY compatibility without weakening graph guards.
use super::structure::{Location, StructureReport};
use crate::codefriend::{
    evidence::{
        contracts::{Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        hash,
        store::Store,
    },
    memory::{
        baseline::{AdmittedBaselines, BaselineAccess, BaselineRef},
        comparison::{self, DeltaReport},
    },
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
const LIMIT: u64 = 32 * 1024 * 1024;
pub const VERSION: &str = "codefriend.drift.v1";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FactTrace {
    pub finding_id: String,
    pub locations: Vec<Location>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DriftReport {
    pub schema: String,
    pub baseline: StructureReport,
    pub current: StructureReport,
    /// Original producer records compared through the persistent CF-MEMORY adapter.
    pub graph_comparison: DeltaReport,
    pub baseline_facts: ReviewRecord,
    pub current_facts: ReviewRecord,
    pub baseline_traces: Vec<FactTrace>,
    pub current_traces: Vec<FactTrace>,
    /// Added/resolved refer to structural declarations, never corrected defects or runtime behavior.
    pub structural_comparison: DeltaReport,
    pub digest: String,
}
fn facts(g: &StructureReport) -> Result<(ReviewRecord, Vec<FactTrace>)> {
    let mut lanes = g.record.run.lane_versions.clone();
    lanes.insert("architecture-drift".into(), VERSION.into());
    let run = Run::new(
        &g.record.admission,
        lanes,
        g.record.run.provider_route.clone(),
        g.record.run.completion.clone(),
        g.record.run.failures.clone(),
    )?;
    let mut record = ReviewRecord {
        admission: g.record.admission.clone(),
        run,
        findings: vec![],
    };
    // Multiple occurrences of a reference share one structural identity and retain every location.
    let mut values: BTreeMap<(String, String), (String, BTreeSet<Location>)> = BTreeMap::new();
    for node in &g.nodes {
        let rationale = format!(
            "Observed {} node {} in declared layer {}.",
            node.kind,
            node.module,
            node.layer.as_deref().unwrap_or("unknown")
        );
        ensure!(
            values
                .insert(
                    ("node".into(), node.module.clone()),
                    (rationale, BTreeSet::from([node.location.clone()]))
                )
                .is_none(),
            "ambiguous_drift_node_identity"
        );
    }
    let nodes: BTreeMap<_, _> = g.nodes.iter().map(|n| (&n.id, n)).collect();
    for edge in &g.edges {
        let from = nodes[&edge.from];
        let to = nodes[&edge.to];
        let anchor =
            serde_json::to_string(&(&from.module, &to.module, &edge.reference, &edge.kind))?;
        let cross = from.layer != to.layer;
        let rationale=format!("Observed {} from {} to {} referencing {}; declared layers {:?} -> {:?}; cross_layer={cross}. This is a syntactic/declaration relationship, not runtime coupling.",edge.kind,from.module,to.module,edge.reference,from.layer,to.layer);
        values
            .entry(("edge".into(), anchor))
            .or_insert_with(|| (rationale, BTreeSet::new()))
            .1
            .insert(edge.location.clone());
    }
    ensure!(values.len() <= 1000, "drift_fact_bounds_exceeded");
    let mut traces = Vec::new();
    for ((rule, anchor), (rationale, locations)) in values {
        let evidence = locations
            .iter()
            .map(|l| l.evidence_id.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut f=Finding{schema:CONTRACT.into(),id:String::new(),repository:record.run.repository.clone(),perspective:"architecture-drift".into(),rule,semantic_anchor:anchor,title:"Observed structural declaration".into(),severity:Severity::Info,rationale,confidence:Confidence::Unknown,evidence,inference:"Matched by shared Finding identity from scoped module names or directed reference endpoints, spelling and kind. No rename guessing or runtime independence claim.".into(),scope_digest:record.run.scope_digest.clone(),limitations:vec!["Only complete compatible scoped syntax graphs establish comparable declaration changes; evidence line moves do not change identity.".into()]};
        f.id = f.identity()?;
        traces.push(FactTrace {
            finding_id: f.id.clone(),
            locations: locations.into_iter().collect(),
        });
        record.findings.push(f);
    }
    record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    traces.sort_by(|a, b| a.finding_id.cmp(&b.finding_id));
    record.validate()?;
    Ok((record, traces))
}
/// Recomputed facts are not a compatibility bypass: original graph comparison is mandatory,
/// and all inherited lane/policy, coverage, completion and admission identities stay intact.
struct FactAccess<'a> {
    graphs: [(&'a Store, &'a StructureReport); 2],
}
impl BaselineAccess for FactAccess<'_> {
    fn load(&self, reference: &BaselineRef) -> Result<ReviewRecord> {
        reference.validate()?;
        for (store, graph) in self.graphs {
            graph.validate(store)?;
            let (record, _) = facts(graph)?;
            if &BaselineRef::from_record(&record)? == reference {
                return Ok(record);
            }
        }
        anyhow::bail!("drift_fact_reference_missing")
    }
}
pub fn architecture_drift_reporter(
    store: &Store,
    baselines: &AdmittedBaselines<'_>,
    baseline: StructureReport,
    current: StructureReport,
) -> Result<DriftReport> {
    architecture_drift_reporter_pair(store, store, baselines, baseline, current)
}

/// Each graph remains bound to its original live admission owner. The caller
/// supplies a trusted, exact-reference backend; no admission is copied here.
pub fn architecture_drift_reporter_pair(
    baseline_store: &Store,
    current_store: &Store,
    baselines: &impl BaselineAccess,
    baseline: StructureReport,
    current: StructureReport,
) -> Result<DriftReport> {
    baseline.validate(baseline_store)?;
    current.validate(current_store)?;
    // Caller explicitly retains these actual producer records; missing/deleted baselines fail.
    let before = BaselineRef::from_record(&baseline.record)?;
    let after = BaselineRef::from_record(&current.record)?;
    let graph_comparison = comparison::compare(baselines, &before, &after)?;
    let (baseline_facts, baseline_traces) = facts(&baseline)?;
    let (current_facts, current_traces) = facts(&current)?;
    let structural_comparison = comparison::compare(
        &FactAccess {
            graphs: [(baseline_store, &baseline), (current_store, &current)],
        },
        &BaselineRef::from_record(&baseline_facts)?,
        &BaselineRef::from_record(&current_facts)?,
    )?;
    ensure!(
        graph_comparison.comparable == structural_comparison.comparable
            && graph_comparison.reasons == structural_comparison.reasons,
        "drift_compatibility_guard_mismatch"
    );
    let mut r = DriftReport {
        schema: VERSION.into(),
        baseline,
        current,
        graph_comparison,
        baseline_facts,
        current_facts,
        baseline_traces,
        current_traces,
        structural_comparison,
        digest: String::new(),
    };
    r.digest = hash(&r)?;
    // Calculation does not extend either admission's lifetime. Recheck the
    // original owners and retained references before exposing the result.
    for reference in [&before, &after] {
        ensure!(
            BaselineRef::from_record(&baselines.load(reference)?)? == *reference,
            "drift_baseline_reference_changed"
        );
    }
    r.baseline.validate(baseline_store)?;
    r.current.validate(current_store)?;
    Ok(r)
}
impl DriftReport {
    pub fn validate(&self, store: &Store, baselines: &AdmittedBaselines<'_>) -> Result<()> {
        self.validate_pair(store, store, baselines)
    }

    pub fn validate_pair(
        &self,
        baseline_store: &Store,
        current_store: &Store,
        baselines: &impl BaselineAccess,
    ) -> Result<()> {
        ensure!(
            *self
                == architecture_drift_reporter_pair(
                    baseline_store,
                    current_store,
                    baselines,
                    self.baseline.clone(),
                    self.current.clone()
                )?,
            "drift_artifact_mismatch"
        );
        Ok(())
    }
}
pub fn write_report(
    r: &DriftReport,
    store: &Store,
    baselines: &AdmittedBaselines<'_>,
    path: &std::path::Path,
) -> Result<()> {
    write_report_pair(r, store, store, baselines, path)
}

pub fn write_report_pair(
    r: &DriftReport,
    baseline_store: &Store,
    current_store: &Store,
    baselines: &impl BaselineAccess,
    path: &std::path::Path,
) -> Result<()> {
    use std::io::Write;
    r.validate_pair(baseline_store, current_store, baselines)?;
    super::structure::safe_artifact_path(path)?;
    let bytes = serde_json::to_vec(r)?;
    ensure!(bytes.len() as u64 <= LIMIT, "drift_artifact_too_large");
    let mut o = std::fs::OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    let mut f = o
        .open(path)
        .map_err(|_| anyhow::anyhow!("drift_output_unavailable"))?;
    f.write_all(&bytes)?;
    f.sync_all()?;
    Ok(())
}
pub fn read_report(
    store: &Store,
    baselines: &AdmittedBaselines<'_>,
    path: &std::path::Path,
) -> Result<DriftReport> {
    read_report_pair(store, store, baselines, path)
}

pub fn read_report_pair(
    baseline_store: &Store,
    current_store: &Store,
    baselines: &impl BaselineAccess,
    path: &std::path::Path,
) -> Result<DriftReport> {
    use std::io::Read;
    super::structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "drift_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(LIMIT + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= LIMIT, "drift_artifact_too_large");
    let r: DriftReport =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_drift_artifact"))?;
    r.validate_pair(baseline_store, current_store, baselines)?;
    Ok(r)
}
