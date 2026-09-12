//! Shared versioned semantic checks, not review, matching, or publication algorithms.
use super::{hash, valid_digest, Admission};
use crate::codefriend::ingestion::unsafe_content;
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const CONTRACT: &str = "codefriend.contracts.v1";
fn text(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty() && value.len() <= 8192 && !unsafe_content("", value),
        "unsafe_or_empty_contract_text"
    );
    Ok(())
}
fn version(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"._-/:".contains(&c)),
        "invalid_contract_identifier"
    );
    text(value)
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Completion {
    Complete,
    Incomplete,
    Failed,
    Cancelled,
    Withheld,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub schema: String,
    pub id: String,
    pub repository: String,
    pub revision: String,
    pub scope_digest: String,
    pub included: Vec<String>,
    pub excluded: Vec<String>,
    pub packet_id: String,
    pub admission_digest: String,
    pub lane_versions: BTreeMap<String, String>,
    pub provider_route: String,
    pub completion: Completion,
    pub failures: Vec<String>,
}
impl Run {
    pub fn new(
        a: &Admission,
        lane_versions: BTreeMap<String, String>,
        provider_route: String,
        completion: Completion,
        failures: Vec<String>,
    ) -> Result<Self> {
        a.validate()?;
        let mut r = Self {
            schema: CONTRACT.into(),
            id: String::new(),
            repository: a.packet.repository.clone(),
            revision: a.packet.revision.clone(),
            scope_digest: a.packet.scope_digest.clone(),
            included: a.evidence.iter().map(|e| e.path.clone()).collect(),
            excluded: a
                .packet
                .objects
                .iter()
                .filter(|o| o.content.is_none())
                .map(|o| o.path.clone())
                .chain(std::iter::once(a.packet.excluded_surfaces.clone()))
                .collect(),
            packet_id: a.packet.packet_id.clone(),
            admission_digest: a.digest.clone(),
            lane_versions,
            provider_route,
            completion,
            failures,
        };
        r.excluded.sort();
        r.excluded.dedup();
        r.id = r.identity()?;
        r.validate(a)?;
        Ok(r)
    }
    fn identity(&self) -> Result<String> {
        let mut c = self.clone();
        c.id.clear();
        hash(&c)
    }
    pub fn validate(&self, a: &Admission) -> Result<()> {
        a.validate()?;
        ensure!(
            self.schema == CONTRACT && self.id == self.identity()?,
            "invalid_run_identity_or_version"
        );
        ensure!(
            self.repository == a.packet.repository
                && self.revision == a.packet.revision
                && self.scope_digest == a.packet.scope_digest
                && self.packet_id == a.packet.packet_id
                && self.admission_digest == a.digest,
            "run_provenance_mismatch"
        );
        let included: Vec<_> = a.evidence.iter().map(|e| e.path.clone()).collect();
        let mut excluded: Vec<_> = a
            .packet
            .objects
            .iter()
            .filter(|o| o.content.is_none())
            .map(|o| o.path.clone())
            .chain(std::iter::once(a.packet.excluded_surfaces.clone()))
            .collect();
        excluded.sort();
        excluded.dedup();
        ensure!(
            self.included == included && self.excluded == excluded,
            "run_coverage_mismatch"
        );
        ensure!(
            !self.lane_versions.is_empty() && self.lane_versions.len() <= 32,
            "invalid_lane_versions"
        );
        for (k, v) in &self.lane_versions {
            version(k)?;
            version(v)?;
        }
        version(&self.provider_route)?;
        ensure!(self.failures.len() <= 100, "too_many_failures");
        for f in &self.failures {
            text(f)?;
        }
        if self.completion == Completion::Complete {
            ensure!(
                self.failures.is_empty() && a.packet.completeness == "complete_scoped_acquisition",
                "false_complete_run"
            );
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "state", content = "percent", rename_all = "snake_case")]
pub enum Confidence {
    Known(u8),
    Unknown,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub schema: String,
    pub id: String,
    pub repository: String,
    pub perspective: String,
    pub rule: String,
    pub semantic_anchor: String,
    pub title: String,
    pub severity: Severity,
    pub rationale: String,
    pub confidence: Confidence,
    pub evidence: Vec<String>,
    pub inference: String,
    pub scope_digest: String,
    pub limitations: Vec<String>,
}
impl Finding {
    pub fn identity(&self) -> Result<String> {
        hash(&(
            "codefriend.finding_identity.v1",
            &self.repository,
            &self.perspective,
            &self.rule,
            &self.semantic_anchor,
        ))
    }
    pub fn validate(&self, run: &Run, a: &Admission) -> Result<()> {
        ensure!(
            self.schema == CONTRACT
                && self.id == self.identity()?
                && self.repository == run.repository
                && self.scope_digest == run.scope_digest,
            "invalid_finding_identity_or_scope"
        );
        version(&self.perspective)?;
        version(&self.rule)?;
        text(&self.semantic_anchor)?;
        text(&self.title)?;
        text(&self.rationale)?;
        text(&self.inference)?;
        ensure!(
            run.lane_versions.contains_key(&self.perspective),
            "unselected_finding_perspective"
        );
        if let Confidence::Known(p) = self.confidence {
            ensure!(p <= 100, "invalid_confidence");
        }
        ensure!(
            !self.evidence.is_empty() && self.evidence.windows(2).all(|w| w[0] < w[1]),
            "invalid_finding_evidence_set"
        );
        for id in &self.evidence {
            ensure!(
                a.evidence
                    .iter()
                    .any(|e| &e.id == id && run.included.contains(&e.path)),
                "missing_finding_evidence"
            );
        }
        ensure!(self.limitations.len() <= 100, "too_many_limitations");
        for l in &self.limitations {
            text(l)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewRecord {
    pub admission: Admission,
    pub run: Run,
    pub findings: Vec<Finding>,
}
impl ReviewRecord {
    pub fn validate(&self) -> Result<()> {
        self.run.validate(&self.admission)?;
        ensure!(self.findings.len() <= 1000, "too_many_findings");
        let mut seen = BTreeSet::new();
        for f in &self.findings {
            f.validate(&self.run, &self.admission)?;
            ensure!(seen.insert(&f.id), "finding_identity_collision");
        }
        Ok(())
    }
    pub fn finding_digest(&self) -> Result<String> {
        let mut sorted: Vec<_> = self.findings.iter().collect();
        sorted.sort_by(|a, b| a.id.cmp(&b.id));
        hash(&sorted)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Delta {
    Added,
    Resolved,
    Changed,
    Unchanged,
    NotComparable,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Comparison {
    pub schema: String,
    pub baseline_run: String,
    pub current_run: String,
    pub baseline_version: String,
    pub current_version: String,
    pub finding_id: Option<String>,
    pub outcome: Delta,
    pub reason: String,
}
impl Comparison {
    pub fn validate(&self, b: &ReviewRecord, c: &ReviewRecord) -> Result<()> {
        b.validate()?;
        c.validate()?;
        text(&self.reason)?;
        ensure!(
            self.schema == CONTRACT
                && self.baseline_run == b.run.id
                && self.current_run == c.run.id
                && self.baseline_version == b.run.schema
                && self.current_version == c.run.schema,
            "comparison_identity_mismatch"
        );
        let compatible = b.run.repository == c.run.repository
            && b.run.scope_digest == c.run.scope_digest
            && b.run.lane_versions == c.run.lane_versions
            && b.run.provider_route == c.run.provider_route
            && b.run.completion == Completion::Complete
            && c.run.completion == Completion::Complete;
        if self.outcome == Delta::NotComparable {
            return Ok(());
        }
        ensure!(
            compatible,
            "comparison_requires_compatible_completed_coverage"
        );
        let id = self
            .finding_id
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("missing_comparison_finding"))?;
        let old = b.findings.iter().find(|f| &f.id == id);
        let new = c.findings.iter().find(|f| &f.id == id);
        let valid = match self.outcome {
            Delta::Added => old.is_none() && new.is_some(),
            Delta::Resolved => old.is_some() && new.is_none(),
            Delta::Changed => old.is_some() && new.is_some() && old != new,
            Delta::Unchanged => old.is_some() && old == new,
            Delta::NotComparable => true,
        };
        ensure!(valid, "unsupported_comparison_delta");
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub path: String,
    pub digest: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    Withheld,
    Approved,
    Invalidated,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Approval {
    pub binding_digest: String,
    pub actor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub schema: String,
    pub run_digest: String,
    pub finding_set_digest: String,
    pub artifact_manifest: Vec<Artifact>,
    pub manifest_digest: String,
    pub renderer_versions: BTreeMap<String, String>,
    pub scope_digest: String,
    pub target: String,
    pub claims: Vec<String>,
    pub nonclaims: Vec<String>,
    pub state: PublicationState,
    pub approval: Option<Approval>,
}
impl Publication {
    pub fn binding_digest(&self) -> Result<String> {
        hash(&(
            CONTRACT,
            &self.run_digest,
            &self.finding_set_digest,
            &self.manifest_digest,
            &self.renderer_versions,
            &self.scope_digest,
            &self.target,
            &self.claims,
            &self.nonclaims,
        ))
    }
    pub fn validate(&self, r: &ReviewRecord) -> Result<()> {
        r.validate()?;
        ensure!(
            self.schema == CONTRACT
                && self.run_digest == hash(&r.run)?
                && self.finding_set_digest == r.finding_digest()?
                && self.scope_digest == r.run.scope_digest,
            "publication_source_mismatch"
        );
        ensure!(
            !self.artifact_manifest.is_empty()
                && self.artifact_manifest.len() <= 1000
                && self
                    .artifact_manifest
                    .windows(2)
                    .all(|w| w[0].path < w[1].path)
                && self.manifest_digest == hash(&self.artifact_manifest)?,
            "invalid_artifact_manifest"
        );
        for a in &self.artifact_manifest {
            crate::codefriend::ingestion::validate_path(&a.path)?;
            ensure!(valid_digest(&a.digest), "invalid_artifact_digest");
        }
        ensure!(
            !self.renderer_versions.is_empty(),
            "missing_renderer_version"
        );
        for (k, v) in &self.renderer_versions {
            version(k)?;
            version(v)?;
        }
        text(&self.target)?;
        ensure!(
            self.claims.len() <= 100 && self.nonclaims.len() <= 100,
            "too_many_publication_claims"
        );
        for t in self.claims.iter().chain(&self.nonclaims) {
            text(t)?;
        }
        if let Some(a) = &self.approval {
            text(&a.actor)?;
            ensure!(valid_digest(&a.binding_digest), "invalid_approval_digest");
        }
        if self.state == PublicationState::Approved {
            let a = self
                .approval
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("approval_missing"))?;
            ensure!(
                a.binding_digest == self.binding_digest()?,
                "approval_invalidated"
            );
        }
        Ok(())
    }
}
/// All future consumers must call this shared admission check and use the same fixtures.
#[derive(Debug, Clone, Copy)]
pub enum Consumer {
    Review,
    Memory,
    Renderer,
}
pub fn consume(_consumer: Consumer, record: &ReviewRecord) -> Result<()> {
    record.validate()
}
