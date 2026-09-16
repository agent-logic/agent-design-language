//! Candidate-bound verification of the local runner; this module defines no predicates.
use super::local::{Report, Status};
use crate::codefriend::evidence::{store::Store, valid_digest};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

pub const VERSION: &str = "codefriend.fitness.ci.v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Expected {
    pub candidate: String,
    pub packet_id: String,
    pub policy_digest: String,
}

impl Expected {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            (self.candidate.len() == 40 || self.candidate.len() == 64)
                && self.candidate.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid_candidate"
        );
        ensure!(
            valid_digest(&self.packet_id) && valid_digest(&self.policy_digest),
            "invalid_expected_identity"
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema: String,
    pub original_exit: i32,
    pub exit_code: i32,
    pub artifact_valid: bool,
    pub assessment: Option<Status>,
    pub candidate: Option<String>,
    pub packet_id: Option<String>,
    pub policy_digest: Option<String>,
    pub report_digest: Option<String>,
    pub error: Option<String>,
}

impl Receipt {
    pub fn rejected(original_exit: i32) -> Self {
        Self {
            schema: VERSION.into(),
            original_exit,
            exit_code: 2,
            artifact_valid: false,
            assessment: None,
            candidate: None,
            packet_id: None,
            policy_digest: None,
            report_digest: None,
            error: Some("fitness_ci_contract_rejected".into()),
        }
    }
}

pub fn verify(
    store: &Store,
    report: &Report,
    expected: &Expected,
    original_exit: i32,
) -> Result<Receipt> {
    expected.validate()?;
    ensure!((0..=2).contains(&original_exit), "unsupported_runner_exit");
    report.validate(store)?;
    ensure!(
        report.record.run.packet_id == expected.packet_id,
        "ci_packet_mismatch"
    );
    ensure!(
        report.record.run.revision == expected.candidate,
        "ci_candidate_mismatch"
    );
    ensure!(
        report.policy_digest == expected.policy_digest,
        "ci_policy_mismatch"
    );
    ensure!(
        report.status.exit_code() == original_exit,
        "ci_exit_mismatch"
    );
    Ok(Receipt {
        schema: VERSION.into(),
        original_exit,
        exit_code: original_exit,
        artifact_valid: true,
        assessment: Some(report.status.clone()),
        candidate: Some(expected.candidate.clone()),
        packet_id: Some(expected.packet_id.clone()),
        policy_digest: Some(expected.policy_digest.clone()),
        report_digest: Some(report.digest.clone()),
        error: None,
    })
}
