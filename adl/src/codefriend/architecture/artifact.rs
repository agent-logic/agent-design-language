//! Version dispatch preserves the payload of each original architecture owner.
//! Deserialization selects a contract; it does not establish source authority.
use super::{drift, drift_v2, impact, impact_v2, rationale, rationale_v2, structure, structure_v2};
use crate::codefriend::evidence::{contracts::ReviewRecord, store::Store};
use crate::codefriend::memory::baseline::BaselineAccess;
use anyhow::Result;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum BoundaryPolicyArtifact {
    V1(structure::BoundaryPolicy),
    V2(structure_v2::BoundaryPolicyV2),
}

#[derive(Debug, Clone)]
pub enum StructureArtifact {
    V1(structure::StructureReport),
    V2(structure_v2::StructureReportV2),
}
#[derive(Debug, Clone)]
pub enum ChangeSetArtifact {
    V1(impact::ChangeSet),
    V2(impact_v2::ChangeSetV2),
}
#[derive(Debug, Clone)]
pub enum ImpactArtifact {
    V1(impact::ImpactReport),
    V2(impact_v2::ImpactReportV2),
}

macro_rules! payload_dispatch {
    ($name:ident, $old:ty, $new:ty, $v1:path, $v2:path) => {
        impl Serialize for $name {
            fn serialize<S: Serializer>(
                &self,
                serializer: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                match self {
                    Self::V1(value) => value.serialize(serializer),
                    Self::V2(value) => value.serialize(serializer),
                }
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Self, D::Error> {
                let value = crate::codefriend::schema::UniqueValue::deserialize(deserializer)?.0;
                match value.get("schema").and_then(serde_json::Value::as_str) {
                    Some($v1) => serde_json::from_value::<$old>(value)
                        .map(Self::V1)
                        .map_err(D::Error::custom),
                    Some($v2) => serde_json::from_value::<$new>(value)
                        .map(Self::V2)
                        .map_err(D::Error::custom),
                    _ => Err(D::Error::custom("unsupported_structure_schema")),
                }
            }
        }
    };
}
payload_dispatch!(
    BoundaryPolicyArtifact,
    structure::BoundaryPolicy,
    structure_v2::BoundaryPolicyV2,
    structure::VERSION,
    structure_v2::VERSION
);
payload_dispatch!(
    StructureArtifact,
    structure::StructureReport,
    structure_v2::StructureReportV2,
    structure::VERSION,
    structure_v2::VERSION
);
payload_dispatch!(
    ChangeSetArtifact,
    impact::ChangeSet,
    impact_v2::ChangeSetV2,
    impact::VERSION,
    impact_v2::VERSION
);
payload_dispatch!(
    ImpactArtifact,
    impact::ImpactReport,
    impact_v2::ImpactReportV2,
    impact::VERSION,
    impact_v2::VERSION
);

impl ImpactArtifact {
    pub fn summary(&self) -> serde_json::Value {
        match self {
            Self::V1(v) => {
                serde_json::json!({"schema":impact::VERSION,"digest":v.digest,"run_id":v.record.run.id,"analysis_complete":v.analysis_complete,"impacts":v.impacts.len(),"unknowns":v.unknowns.len()})
            }
            Self::V2(v) => {
                serde_json::json!({"schema":impact_v2::VERSION,"digest":v.digest,"run_id":v.record.run.id,"analysis_complete":v.analysis_complete,"impacts":v.impacts.len(),"unknowns":v.unknowns.len()})
            }
        }
    }
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        match self {
            Self::V1(v) => v.validate(store),
            Self::V2(v) => v.validate(store, now),
        }
    }
}
pub fn impact_report(
    store: &Store,
    graph: StructureArtifact,
    changes: ChangeSetArtifact,
    now: u64,
) -> Result<ImpactArtifact> {
    match (graph, changes) {
        (StructureArtifact::V1(g), ChangeSetArtifact::V1(c)) => {
            impact::change_impact_reporter(store, g, c).map(ImpactArtifact::V1)
        }
        (StructureArtifact::V2(g), ChangeSetArtifact::V2(c)) => {
            impact_v2::change_impact_reporter_v2(store, g, c, now).map(ImpactArtifact::V2)
        }
        _ => anyhow::bail!("incompatible_impact_graph_schema"),
    }
}
pub fn write_impact(
    value: &ImpactArtifact,
    store: &Store,
    path: &std::path::Path,
    now: u64,
) -> Result<()> {
    match value {
        ImpactArtifact::V1(v) => impact::write_report(v, store, path),
        ImpactArtifact::V2(v) => impact_v2::write_report_v2(v, store, path, now),
    }
}
pub fn read_impact(store: &Store, path: &std::path::Path, now: u64) -> Result<ImpactArtifact> {
    use std::io::Read;
    structure::safe_artifact_path(path)?;
    anyhow::ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "impact_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    anyhow::ensure!(bytes.len() <= 32 * 1024 * 1024, "impact_artifact_too_large");
    let value: ImpactArtifact =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_impact_artifact"))?;
    if let ImpactArtifact::V2(_) = &value {
        anyhow::ensure!(bytes.len() <= 4 * 1024 * 1024, "impact_artifact_too_large");
    }
    value.validate(store, now)?;
    Ok(value)
}

impl StructureArtifact {
    pub fn summary(&self) -> serde_json::Value {
        let (schema, complete, nodes, edges, unknowns) = match self {
            Self::V1(v) => (
                structure::VERSION,
                v.analysis_complete,
                v.nodes.len(),
                v.edges.len(),
                v.unknowns.len(),
            ),
            Self::V2(v) => (
                structure_v2::VERSION,
                v.analysis_complete,
                v.nodes.len(),
                v.edges.len(),
                v.unknowns.len(),
            ),
        };
        serde_json::json!({"schema":schema,"digest":self.digest(),"run_id":self.record().run.id,"analysis_complete":complete,"nodes":nodes,"edges":edges,"findings":self.record().findings.len(),"unknowns":unknowns})
    }
    pub fn record(&self) -> &ReviewRecord {
        match self {
            Self::V1(value) => &value.record,
            Self::V2(value) => &value.record,
        }
    }
    pub fn digest(&self) -> &str {
        match self {
            Self::V1(value) => &value.digest,
            Self::V2(value) => &value.digest,
        }
    }
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        match self {
            Self::V1(value) => value.validate(store),
            Self::V2(value) => value.validate(store, now),
        }
    }
}

pub fn write_report(
    value: &StructureArtifact,
    store: &Store,
    path: &std::path::Path,
    now: u64,
) -> Result<()> {
    match value {
        StructureArtifact::V1(v) => structure::write_report(v, store, path),
        StructureArtifact::V2(v) => structure_v2::write_report_v2(v, store, path, now),
    }
}

pub fn read_report(store: &Store, path: &std::path::Path, now: u64) -> Result<StructureArtifact> {
    use std::io::Read;
    structure::safe_artifact_path(path)?;
    anyhow::ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "structure_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    anyhow::ensure!(
        bytes.len() <= 16 * 1024 * 1024,
        "structure_artifact_too_large"
    );
    let value: StructureArtifact = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow::anyhow!("invalid_structure_artifact"))?;
    if let StructureArtifact::V2(_) = &value {
        anyhow::ensure!(
            bytes.len() <= 4 * 1024 * 1024,
            "structure_artifact_too_large"
        );
    }
    value.validate(store, now)?;
    Ok(value)
}

/// Reconstruct the selected report through its original source owner.
pub fn report(
    store: &Store,
    packet_id: &str,
    policy: BoundaryPolicyArtifact,
    now: u64,
) -> Result<StructureArtifact> {
    match policy {
        BoundaryPolicyArtifact::V1(value) => {
            structure::repository_structure_reporter(store, packet_id, value)
                .map(StructureArtifact::V1)
        }
        BoundaryPolicyArtifact::V2(value) => {
            structure_v2::repository_structure_reporter_v2(store, packet_id, value, now)
                .map(StructureArtifact::V2)
        }
    }
}

#[derive(Debug, Clone)]
pub enum RationaleSelectionArtifact {
    V1(rationale::RationaleSelection),
    V2(rationale_v2::RationaleSelectionV2),
}
#[derive(Debug, Clone)]
pub enum RationaleArtifact {
    V1(rationale::RationaleReport),
    V2(rationale_v2::RationaleReportV2),
}
#[derive(Debug, Clone)]
pub enum DriftArtifact {
    V1(drift::DriftReport),
    V2(drift_v2::DriftReportV2),
}
payload_dispatch!(
    RationaleSelectionArtifact,
    rationale::RationaleSelection,
    rationale_v2::RationaleSelectionV2,
    rationale::VERSION,
    rationale_v2::VERSION
);
payload_dispatch!(
    RationaleArtifact,
    rationale::RationaleReport,
    rationale_v2::RationaleReportV2,
    rationale::VERSION,
    rationale_v2::VERSION
);
payload_dispatch!(
    DriftArtifact,
    drift::DriftReport,
    drift_v2::DriftReportV2,
    drift::VERSION,
    drift_v2::VERSION
);
impl RationaleArtifact {
    pub fn record(&self) -> &ReviewRecord {
        match self {
            Self::V1(v) => &v.record,
            Self::V2(v) => &v.record,
        }
    }
    pub fn summary(&self) -> serde_json::Value {
        match self {
            Self::V1(v) => {
                serde_json::json!({"schema":rationale::VERSION,"digest":v.digest,"run_id":v.record.run.id,"analysis_complete":v.analysis_complete,"boundaries":v.boundaries.len(),"findings":v.record.findings.len()})
            }
            Self::V2(v) => {
                serde_json::json!({"schema":rationale_v2::VERSION,"digest":v.digest,"run_id":v.record.run.id,"analysis_complete":v.analysis_complete,"boundaries":v.boundaries.len(),"findings":v.record.findings.len()})
            }
        }
    }
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        match self {
            Self::V1(v) => v.validate(store),
            Self::V2(v) => v.validate(store, now),
        }
    }
}
pub fn rationale_report(
    store: &Store,
    graph: StructureArtifact,
    selection: RationaleSelectionArtifact,
    now: u64,
) -> Result<RationaleArtifact> {
    match (graph, selection) {
        (StructureArtifact::V1(g), RationaleSelectionArtifact::V1(s)) => {
            rationale::architecture_rationale_reporter(store, g, s).map(RationaleArtifact::V1)
        }
        (StructureArtifact::V2(g), RationaleSelectionArtifact::V2(s)) => {
            rationale_v2::architecture_rationale_reporter_v2(store, g, s, now)
                .map(RationaleArtifact::V2)
        }
        _ => anyhow::bail!("incompatible_rationale_graph_schema"),
    }
}
pub fn write_rationale(
    v: &RationaleArtifact,
    store: &Store,
    path: &std::path::Path,
    now: u64,
) -> Result<()> {
    match v {
        RationaleArtifact::V1(v) => rationale::write_report(v, store, path),
        RationaleArtifact::V2(v) => rationale_v2::write_report_v2(v, store, path, now),
    }
}
fn read_derived_bytes(path: &std::path::Path) -> Result<Vec<u8>> {
    use std::io::Read;
    structure::safe_artifact_path(path)?;
    anyhow::ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "architecture_input_not_regular"
    );
    let mut bytes = vec![];
    std::fs::File::open(path)?
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    anyhow::ensure!(
        bytes.len() <= 32 * 1024 * 1024,
        "architecture_artifact_too_large"
    );
    Ok(bytes)
}
pub fn read_rationale(
    store: &Store,
    path: &std::path::Path,
    now: u64,
) -> Result<RationaleArtifact> {
    let bytes = read_derived_bytes(path)?;
    let v: RationaleArtifact = serde_json::from_slice(&bytes)?;
    if matches!(v, RationaleArtifact::V2(_)) {
        anyhow::ensure!(
            bytes.len() <= 4 * 1024 * 1024,
            "rationale_artifact_too_large"
        );
    }
    v.validate(store, now)?;
    Ok(v)
}
impl DriftArtifact {
    pub fn summary(&self) -> serde_json::Value {
        match self {
            Self::V1(v) => {
                serde_json::json!({"schema":drift::VERSION,"digest":v.digest,"comparable":v.structural_comparison.comparable,"reasons":v.structural_comparison.reasons,"changes":v.structural_comparison.changes.len()})
            }
            Self::V2(v) => {
                serde_json::json!({"schema":drift_v2::VERSION,"digest":v.digest,"comparable":v.structural_comparison.comparable,"reasons":v.structural_comparison.reasons,"changes":v.structural_comparison.changes.len()})
            }
        }
    }
    pub fn validate_pair(
        &self,
        bs: &Store,
        cs: &Store,
        access: &impl BaselineAccess,
        now: u64,
    ) -> Result<()> {
        match self {
            Self::V1(v) => v.validate_pair(bs, cs, access),
            Self::V2(v) => v.validate_pair(bs, cs, access, now),
        }
    }
}
pub fn drift_report_pair(
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    before: StructureArtifact,
    current: StructureArtifact,
    now: u64,
) -> Result<DriftArtifact> {
    match (before, current) {
        (StructureArtifact::V1(b), StructureArtifact::V1(c)) => {
            drift::architecture_drift_reporter_pair(bs, cs, access, b, c).map(DriftArtifact::V1)
        }
        (StructureArtifact::V2(b), StructureArtifact::V2(c)) => {
            drift_v2::architecture_drift_reporter_v2_pair(bs, cs, access, b, c, now)
                .map(DriftArtifact::V2)
        }
        _ => anyhow::bail!("incompatible_drift_graph_schema"),
    }
}
pub fn write_drift_pair(
    v: &DriftArtifact,
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    path: &std::path::Path,
    now: u64,
) -> Result<()> {
    match v {
        DriftArtifact::V1(v) => drift::write_report_pair(v, bs, cs, access, path),
        DriftArtifact::V2(v) => drift_v2::write_report_v2_pair(v, bs, cs, access, path, now),
    }
}
pub fn read_drift_pair(
    bs: &Store,
    cs: &Store,
    access: &impl BaselineAccess,
    path: &std::path::Path,
    now: u64,
) -> Result<DriftArtifact> {
    let bytes = read_derived_bytes(path)?;
    let v: DriftArtifact = serde_json::from_slice(&bytes)?;
    if matches!(v, DriftArtifact::V2(_)) {
        anyhow::ensure!(bytes.len() <= 4 * 1024 * 1024, "drift_artifact_too_large");
    }
    v.validate_pair(bs, cs, access, now)?;
    Ok(v)
}

impl BoundaryPolicyArtifact {
    pub fn validate(&self, admission: &crate::codefriend::evidence::Admission) -> Result<()> {
        match self {
            Self::V1(v) => v.validate(admission),
            Self::V2(v) => v.validate(admission),
        }
    }
    pub fn is_v2(&self) -> bool {
        matches!(self, Self::V2(_))
    }
}
impl StructureArtifact {
    pub fn policy(&self) -> BoundaryPolicyArtifact {
        match self {
            Self::V1(v) => BoundaryPolicyArtifact::V1(v.policy.clone()),
            Self::V2(v) => BoundaryPolicyArtifact::V2(v.policy.clone()),
        }
    }
    pub fn is_v2(&self) -> bool {
        matches!(self, Self::V2(_))
    }
}
impl ImpactArtifact {
    pub fn record(&self) -> &ReviewRecord {
        match self {
            Self::V1(v) => &v.record,
            Self::V2(v) => &v.record,
        }
    }
}
macro_rules! from_payload {
    ($artifact:ident,$v1:ty,$v2:ty) => {
        impl From<$v1> for $artifact {
            fn from(value: $v1) -> Self {
                Self::V1(value)
            }
        }
        impl From<$v2> for $artifact {
            fn from(value: $v2) -> Self {
                Self::V2(value)
            }
        }
    };
}
from_payload!(
    BoundaryPolicyArtifact,
    structure::BoundaryPolicy,
    structure_v2::BoundaryPolicyV2
);
from_payload!(
    StructureArtifact,
    structure::StructureReport,
    structure_v2::StructureReportV2
);
from_payload!(ChangeSetArtifact, impact::ChangeSet, impact_v2::ChangeSetV2);
from_payload!(
    RationaleSelectionArtifact,
    rationale::RationaleSelection,
    rationale_v2::RationaleSelectionV2
);
