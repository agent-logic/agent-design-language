//! Operational analysis reads the original retained admission before and after extraction.
use super::{parser, resolver, AnalysisPolicy, AnalysisReport};
use crate::codefriend::evidence::store::Store;
use anyhow::{ensure, Result};

pub fn analyze(
    store: &Store,
    packet_id: &str,
    policy: &AnalysisPolicy,
    now: u64,
) -> Result<AnalysisReport> {
    Ok(analyze_outcome(store, packet_id, policy, now)?.report)
}

/// Internal evidence from the same extraction and original Store lifetime.
/// This is never deserialized from a caller-supplied report.
pub(crate) struct AnalysisOutcome {
    pub report: AnalysisReport,
    pub normalized_imports: Vec<super::ImportSpec>,
}

pub(crate) fn analyze_outcome(
    store: &Store,
    packet_id: &str,
    policy: &AnalysisPolicy,
    now: u64,
) -> Result<AnalysisOutcome> {
    policy.limits.validate()?;
    let admission = store.get(packet_id)?;
    let (syntax, specs) = parser::analyze(&admission, policy, now)?;
    let report = resolver::resolve(&admission, policy, &syntax, &specs, now)?;
    // The Store clock and tombstone remain authoritative even if caller time is stale.
    let current = store.get(packet_id)?;
    ensure!(
        current.digest == admission.digest,
        "language_admission_changed"
    );
    Ok(AnalysisOutcome {
        report,
        normalized_imports: specs,
    })
}
