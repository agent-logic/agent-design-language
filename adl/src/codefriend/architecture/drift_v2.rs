//! Both-owner drift over admitted v2 facts, retaining incomplete comparison truth.
use super::{structure::Location, structure_v2::StructureReportV2};
use crate::codefriend::{
    evidence::{
        contracts::{Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        hash,
        store::Store,
    },
    memory::{
        baseline::{BaselineAccess, BaselineRef},
        comparison::{self, DeltaReport},
    },
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    path::Path,
};
pub const VERSION: &str = "codefriend.drift.v2";
const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FactTraceV2 {
    pub finding_id: String,
    pub locations: Vec<Location>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DriftReportV2 {
    pub schema: String,
    pub baseline: StructureReportV2,
    pub current: StructureReportV2,
    pub graph_comparison: DeltaReport,
    pub baseline_facts: ReviewRecord,
    pub current_facts: ReviewRecord,
    pub baseline_traces: Vec<FactTraceV2>,
    pub current_traces: Vec<FactTraceV2>,
    pub structural_comparison: DeltaReport,
    pub digest: String,
}
struct Bounded(Vec<u8>);
impl Write for Bounded {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if b.len() > MAX_BYTES.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("drift_artifact_too_large"));
        }
        self.0.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn bounded<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut b = Bounded(vec![]);
    serde_json::to_writer(&mut b, v)?;
    Ok(b.0)
}
fn live(store: &Store, g: &StructureReportV2) -> Result<()> {
    ensure!(
        store.get(&g.record.admission.packet.packet_id)? == g.record.admission,
        "drift_admission_changed"
    );
    Ok(())
}
fn facts(g: &StructureReportV2) -> Result<(ReviewRecord, Vec<FactTraceV2>)> {
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
            node.kind, node.path, node.layer
        );
        ensure!(
            values
                .insert(
                    (
                        "node".into(),
                        hash(&(VERSION, node.language, &node.project_root, &node.path))?
                    ),
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
        let anchor = serde_json::to_string(&(
            VERSION,
            from.language,
            &from.project_root,
            &from.path,
            to.language,
            &to.project_root,
            &to.path,
            &edge.spelling,
            &edge.kind,
        ))?;
        let cross = from.layer != to.layer;
        let rationale=format!("Observed {} from {} to {} referencing {}; declared layers {:?} -> {:?}; cross_layer={cross}. This is a syntactic/declaration relationship, not runtime coupling.",edge.kind,from.path,to.path,edge.spelling,from.layer,to.layer);
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
        let mut f=Finding{schema:CONTRACT.into(),id:String::new(),repository:record.run.repository.clone(),perspective:"architecture-drift".into(),rule,semantic_anchor:anchor,title:"Observed structural declaration".into(),severity:Severity::Info,rationale,confidence:Confidence::Unknown,evidence,inference:"Matched by shared Finding identity from versioned language/project/source identities or directed reference endpoints, spelling and kind. No rename guessing or runtime independence claim.".into(),scope_digest:record.run.scope_digest.clone(),limitations:vec!["Only complete compatible scoped syntax graphs establish comparable declaration changes; evidence line moves do not change identity.".into()]};
        f.id = f.identity()?;
        traces.push(FactTraceV2 {
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
// Internal projected records inherit the original admission, coverage, tools and
// policy. No retain/admit operation, freshness reset, or completion promotion.
struct FactAccess<'a> {
    graphs: [(&'a Store, &'a StructureReportV2); 2],
}
impl BaselineAccess for FactAccess<'_> {
    fn load(&self, reference: &BaselineRef) -> Result<ReviewRecord> {
        reference.validate()?;
        for (store, g) in self.graphs {
            live(store, g)?;
            let (record, _) = facts(g)?;
            if BaselineRef::from_record(&record)? == *reference {
                return Ok(record);
            }
        }
        anyhow::bail!("drift_fact_reference_missing")
    }
}
fn final_check(
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    r: &DriftReportV2,
) -> Result<()> {
    for (g, reference) in [
        (&r.baseline, &r.graph_comparison.baseline),
        (&r.current, &r.graph_comparison.current),
    ] {
        let loaded = access.load(reference)?;
        ensure!(
            BaselineRef::from_record(&loaded)? == *reference && loaded == g.record,
            "drift_original_record_mismatch"
        );
    }
    // Backend callbacks may have changed either owner; check both afterward.
    live(bs, &r.baseline)?;
    live(cs, &r.current)?;
    Ok(())
}
pub fn architecture_drift_reporter_v2_pair(
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    baseline: StructureReportV2,
    current: StructureReportV2,
    now: u64,
) -> Result<DriftReportV2> {
    baseline.validate(bs, now)?;
    current.validate(cs, now)?;
    // Enforce aggregate graph/source footprint before cloning derived records.
    bounded(&(
        &baseline,
        &current,
        &baseline.record.admission,
        &current.record.admission,
    ))?;
    let before = BaselineRef::from_record(&baseline.record)?;
    let after = BaselineRef::from_record(&current.record)?;
    let graph_comparison = comparison::compare(access, &before, &after)?;
    let (baseline_facts, baseline_traces) = facts(&baseline)?;
    let (current_facts, current_traces) = facts(&current)?;
    let structural_comparison = comparison::compare(
        &FactAccess {
            graphs: [(bs, &baseline), (cs, &current)],
        },
        &BaselineRef::from_record(&baseline_facts)?,
        &BaselineRef::from_record(&current_facts)?,
    )?;
    ensure!(
        graph_comparison.comparable == structural_comparison.comparable
            && graph_comparison.reasons == structural_comparison.reasons,
        "drift_compatibility_guard_mismatch"
    );
    let mut r = DriftReportV2 {
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
    bounded(&r)?;
    r.digest = hash(&r)?;
    bounded(&r)?;
    final_check(bs, cs, access, &r)?;
    Ok(r)
}
impl DriftReportV2 {
    pub fn validate_pair(
        &self,
        bs: &Store,
        cs: &Store,
        access: &impl BaselineAccess,
        now: u64,
    ) -> Result<()> {
        bounded(self)?;
        let expected = architecture_drift_reporter_v2_pair(
            bs,
            cs,
            access,
            self.baseline.clone(),
            self.current.clone(),
            now,
        )?;
        ensure!(*self == expected, "drift_artifact_mismatch");
        final_check(bs, cs, access, self)
    }
}
pub fn write_report_v2_pair(
    r: &DriftReportV2,
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    path: &Path,
    now: u64,
) -> Result<()> {
    r.validate_pair(bs, cs, access, now)?;
    super::structure::safe_artifact_path(path)?;
    let data = bounded(r)?;
    final_check(bs, cs, access, r)?;
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
        final_check(bs, cs, access, r)
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
pub fn read_report_v2_pair(
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    path: &Path,
    now: u64,
) -> Result<DriftReportV2> {
    super::structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "drift_input_not_regular"
    );
    let mut data = vec![];
    std::fs::File::open(path)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    ensure!(data.len() <= MAX_BYTES, "drift_artifact_too_large");
    let r: DriftReportV2 = serde_json::from_slice(&data)?;
    r.validate_pair(bs, cs, access, now)?;
    final_check(bs, cs, access, &r)?;
    Ok(r)
}
