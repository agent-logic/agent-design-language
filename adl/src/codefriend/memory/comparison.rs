//! Matching semantics shared by current and future baseline backends.
use super::baseline::{BaselineAccess, BaselineRef};
use crate::codefriend::evidence::{
    contracts::{Comparison, Completion, Delta, Finding, ReviewRecord, CONTRACT},
    hash,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const VERSION: &str = "codefriend.comparison.v1";
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FindingRef {
    pub run_id: String,
    pub finding_id: String,
    pub revision: String,
    pub evidence: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Change {
    pub comparison: Comparison,
    pub before: Option<FindingRef>,
    pub after: Option<FindingRef>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeltaReport {
    pub schema: String,
    pub baseline: BaselineRef,
    pub current: BaselineRef,
    pub comparable: bool,
    pub reasons: Vec<String>,
    pub changes: Vec<Change>,
    pub digest: String,
}
fn reference(record: &ReviewRecord, finding: &Finding) -> FindingRef {
    FindingRef {
        run_id: record.run.id.clone(),
        finding_id: finding.id.clone(),
        revision: record.run.revision.clone(),
        evidence: finding.evidence.clone(),
    }
}
/// Loading is mandatory even for identical references, so retention cannot be bypassed.
pub fn compare(
    access: &impl BaselineAccess,
    baseline: &BaselineRef,
    current: &BaselineRef,
) -> Result<DeltaReport> {
    let b = access.load(baseline)?;
    let c = access.load(current)?;
    b.validate()?;
    c.validate()?;
    ensure!(
        &BaselineRef::from_record(&b)? == baseline && &BaselineRef::from_record(&c)? == current,
        "baseline_backend_reference_mismatch"
    );
    let mut reasons = Vec::new();
    for (equal, reason) in [
        (b.run.repository == c.run.repository, "repository_mismatch"),
        (b.run.schema == c.run.schema, "schema_mismatch"),
        (
            b.run.scope_digest == c.run.scope_digest
                && b.run.included == c.run.included
                && b.run.excluded == c.run.excluded,
            "coverage_scope_mismatch",
        ),
        (
            b.run.lane_versions == c.run.lane_versions,
            "lane_or_policy_version_mismatch",
        ),
        (
            b.run.provider_route == c.run.provider_route,
            "provider_route_mismatch",
        ),
        (
            b.run.completion == Completion::Complete && c.run.completion == Completion::Complete,
            "incomplete_review_coverage",
        ),
    ] {
        if !equal {
            reasons.push(reason.into());
        }
    }
    let comparable = reasons.is_empty();
    let mut changes = Vec::new();
    let row = |id: Option<String>, outcome, reason: String| Comparison {
        schema: CONTRACT.into(),
        baseline_run: b.run.id.clone(),
        current_run: c.run.id.clone(),
        baseline_version: b.run.schema.clone(),
        current_version: c.run.schema.clone(),
        finding_id: id,
        outcome,
        reason,
    };
    if comparable {
        let mut ids = BTreeMap::new();
        for f in &b.findings {
            ids.entry(f.id.clone()).or_insert((None, None)).0 = Some(f);
        }
        for f in &c.findings {
            ids.entry(f.id.clone()).or_insert((None, None)).1 = Some(f);
        }
        for (id, (old, new)) in ids {
            let (outcome, reason) = match (old, new) {
                (None, Some(_)) => (
                    Delta::Added,
                    "stable_identity_absent_from_complete_baseline",
                ),
                (Some(_), None) => (
                    Delta::Resolved,
                    "stable_identity_absent_from_complete_current",
                ),
                (Some(old), Some(new)) if old.same_assessment(new) => (
                    Delta::Unchanged,
                    "stable_identity_and_assessment_equal_revision_evidence_may_differ",
                ),
                (Some(_), Some(_)) => (
                    Delta::Changed,
                    "stable_identity_equal_assessment_fields_changed",
                ),
                _ => unreachable!(),
            };
            let comparison = row(Some(id), outcome, reason.into());
            comparison.validate(&b, &c)?;
            changes.push(Change {
                comparison,
                before: old.map(|f| reference(&b, f)),
                after: new.map(|f| reference(&c, f)),
            });
        }
    } else {
        let comparison = row(None, Delta::NotComparable, reasons.join(","));
        comparison.validate(&b, &c)?;
        changes.push(Change {
            comparison,
            before: None,
            after: None,
        });
    }
    let mut result = DeltaReport {
        schema: VERSION.into(),
        baseline: baseline.clone(),
        current: current.clone(),
        comparable,
        reasons,
        changes,
        digest: String::new(),
    };
    result.digest = hash(&result)?;
    Ok(result)
}
impl DeltaReport {
    pub fn validate(&self, access: &impl BaselineAccess) -> Result<()> {
        ensure!(
            self == &compare(access, &self.baseline, &self.current)?,
            "delta_artifact_mismatch"
        );
        Ok(())
    }
}
