use std::fs;
use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

use crate::cards::{digest, execution_readiness_findings_for_cards};
use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{DesignReview, LifecyclePhase};
use crate::review::evaluate_publication_review_in_repo;
use crate::store::{read_regular_authored_artifact, verify_cards, verify_record, Store};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DoctorStatus {
    Pass,
    Block,
    Corrupt,
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Finding {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DoctorReport {
    pub schema: String,
    pub issue: u64,
    pub status: DoctorStatus,
    pub phase: Option<LifecyclePhase>,
    pub generation: Option<u64>,
    pub ready: bool,
    pub findings: Vec<Finding>,
    pub next_operation: Option<String>,
}

pub fn diagnose(store: &Store, issue: u64) -> DoctorReport {
    diagnose_with_code_repository(store, issue, None)
}

pub fn diagnose_with_code_repository(
    store: &Store,
    issue: u64,
    requested_code_repository: Option<&str>,
) -> DoctorReport {
    let mut report = DoctorReport {
        schema: "csdlc.doctor.report.v1".into(),
        issue,
        status: DoctorStatus::Pass,
        phase: None,
        generation: None,
        ready: false,
        findings: Vec::new(),
        next_operation: None,
    };
    if !store.issue_dir(issue).exists() && store.interrupted_backup(issue).exists() {
        report.status = DoctorStatus::Interrupted;
        report.findings.push(Finding {
            code: "interrupted_transaction".into(),
            message: "complete prior generation is recoverable by the next writer".into(),
        });
        report.next_operation = Some("recover_then_retry".into());
        return report;
    }
    let record = match store.load_record(issue) {
        Ok(record) => record,
        Err(error) => {
            report.status = DoctorStatus::Corrupt;
            report.findings.push(finding(error));
            return report;
        }
    };
    report.phase = Some(record.phase);
    report.generation = Some(record.generation);
    if let Err(error) = verify_record(&record) {
        report.status = DoctorStatus::Corrupt;
        report.findings.push(finding(error));
        return report;
    }
    let recorded_code_repository = record.code_repository.as_deref();
    let requested_identity_conflicts = requested_code_repository
        .zip(recorded_code_repository)
        .is_some_and(|(requested, recorded)| !requested.eq_ignore_ascii_case(recorded));
    if requested_identity_conflicts {
        report.findings.push(Finding {
            code: "repository_identity_drift".into(),
            message: format!(
                "requested code repository {} does not match recorded code repository {} for issue repository {}",
                requested_code_repository.unwrap_or_default(),
                recorded_code_repository.unwrap_or_default(),
                record.repository,
            ),
        });
    }
    let explicit_code_repository = recorded_code_repository.or(requested_code_repository);
    let code_repository = explicit_code_repository.unwrap_or(&record.repository);
    match crate::git::github_remote_repository(store.root(), "origin") {
        Ok(Some(repository))
            if explicit_code_repository.is_none()
                && !repository.eq_ignore_ascii_case(&record.repository) =>
        {
            report.findings.push(Finding {
                code: "repository_identity_drift".into(),
                message: format!(
                    "issue repository {} differs from origin repository {repository}, but no explicit code repository was declared",
                    record.repository,
                ),
            });
        }
        Ok(Some(repository)) if !repository.eq_ignore_ascii_case(code_repository) => {
            report.findings.push(Finding {
                code: "repository_identity_drift".into(),
                message: format!(
                    "declared code repository {code_repository} does not match origin repository {repository}; issue repository is {}",
                    record.repository,
                ),
            });
        }
        Ok(None) if explicit_code_repository.is_some() => report.findings.push(Finding {
            code: "repository_identity_drift".into(),
            message: format!(
                "declared code repository {code_repository} requires an exact effective GitHub origin repository, but none is available; issue repository is {}",
                record.repository,
            ),
        }),
        Ok(_) => {}
        Err(error) => report.findings.push(Finding {
            code: "repository_identity_unavailable".into(),
            message: error.message,
        }),
    }
    for (code, path) in [
        ("design_missing", &record.design_path),
        ("diagram_missing", &record.diagram_path),
    ] {
        match read_regular_authored_artifact(store.root(), Path::new(path)) {
            Ok(Some(_)) => {}
            Ok(None) => report.findings.push(Finding {
                code: code.into(),
                message: format!("required path is missing: {path}"),
            }),
            Err(error) => {
                report.status = DoctorStatus::Corrupt;
                report.findings.push(finding(error));
                return report;
            }
        }
    }
    if report
        .findings
        .iter()
        .any(|finding| matches!(finding.code.as_str(), "design_missing" | "diagram_missing"))
    {
        report.status = DoctorStatus::Block;
        report.next_operation = Some("repair_design_readiness".into());
        return report;
    }
    let cards = match store.load_cards(issue) {
        Ok(cards) => cards,
        Err(error) => {
            report.status = DoctorStatus::Corrupt;
            report.findings.push(finding(error));
            return report;
        }
    };
    if let Err(error) = verify_cards(store, &record, &cards) {
        report.status = DoctorStatus::Corrupt;
        report.findings.push(finding(error));
        return report;
    }
    match execution_readiness_findings_for_cards(store.root(), &cards, record.phase) {
        Ok(findings) => report
            .findings
            .extend(findings.into_iter().map(|finding| Finding {
                code: finding.code.into(),
                message: finding.message,
            })),
        Err(error) => report.findings.push(Finding {
            code: "execution_readiness_invalid".into(),
            message: error.message,
        }),
    }
    let diagram = fs::read_to_string(store.root().join(&record.diagram_path)).unwrap_or_default();
    let first = diagram
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim();
    if !(first.starts_with("flowchart ")
        || first == "stateDiagram-v2"
        || first.starts_with("sequenceDiagram"))
        || diagram.lines().count() < 2
    {
        report.findings.push(Finding {
            code: "diagram_invalid".into(),
            message: "diagram is not recognized Mermaid source".into(),
        });
    }
    let design_bytes = fs::read(store.root().join(&record.design_path)).unwrap_or_default();
    match &record.design_review {
        DesignReview::Approved { reviewer, revision }
            if !reviewer.trim().is_empty() && revision == &digest(&design_bytes) => {}
        _ => report.findings.push(Finding {
            code: "design_review_missing_or_stale".into(),
            message: "design review does not cover the current design digest".into(),
        }),
    }
    if matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) {
        if let Some(review) = record.review.as_ref() {
            let current = crate::git::substantive_revision(store.root(), &review.scope);
            let stale = current.as_ref().is_ok_and(|current| {
                evaluate_publication_review_in_repo(store.root(), Some(review), current)
                    .blocker_codes
                    .iter()
                    .any(|code| code == "review_stale")
            });
            if stale {
                report.findings.push(Finding {
                    code: "review_publication_dead_end".into(),
                    message: "reviewed evidence does not match a clean current substantive commit; recover_review is required before publication".into(),
                });
            }
        }
    }
    if report.findings.is_empty() {
        report.ready = record.phase == LifecyclePhase::Initialized;
        report.next_operation = Some(
            if report.ready {
                "advance_ready"
            } else {
                "inspect_phase"
            }
            .into(),
        );
    } else {
        report.status = DoctorStatus::Block;
        report.next_operation = Some(
            if report
                .findings
                .iter()
                .any(|finding| finding.code == "review_publication_dead_end")
            {
                "recover_review"
            } else {
                "repair_design_readiness"
            }
            .into(),
        );
    }
    report
}

fn finding(error: V2Error) -> Finding {
    Finding {
        code: match error.code {
            ErrorCode::CorruptRecord => "corrupt_record",
            ErrorCode::InterruptedTransaction => "interrupted_transaction",
            _ => "doctor_error",
        }
        .into(),
        message: error.message,
    }
}

pub fn diagnose_result(store: &Store, issue: u64) -> Result<DoctorReport> {
    if issue == 0 {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "issue must be non-zero",
        ));
    }
    Ok(diagnose(store, issue))
}
