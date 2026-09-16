//! Construction of native local route results.

use super::{DoctorFinding, OperationalLocalResult};

#[allow(clippy::too_many_arguments)]
pub(super) fn operational_result(
    route: &str,
    issue: u64,
    mutated: bool,
    phase: &str,
    generation: u64,
    digest: String,
    next_route: Option<&str>,
    findings: Vec<DoctorFinding>,
) -> OperationalLocalResult {
    OperationalLocalResult {
        route: route.into(),
        issue,
        mutated,
        phase: Some(phase.into()),
        generation: Some(generation),
        digest: Some(digest),
        next_route: next_route.map(str::to_string),
        routing: None,
        findings,
    }
}
