//! Bounded import of retained gateway review bytes. Not a second review producer.
use super::{
    activities::UpdateCycleResult,
    evidence::{hash, Admission},
    review::{
        lanes::ReviewLane,
        runner::{self, FourPerspectiveReviewRun, LaneInputManifest, LaneResult},
    },
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::Read, path::Path};

pub(crate) const LIMIT: usize = 4 * 1024 * 1024;
const FILE_LIMIT: usize = 1024 * 1024;

pub(crate) fn review(cycle: &UpdateCycleResult) -> Result<&FourPerspectiveReviewRun> {
    let run = cycle
        .review
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("cycle_review_unavailable"))?;
    cycle.validate(&run.review_record.run.provider_route)?;
    ensure!(
        cycle.completion == super::evidence::contracts::Completion::Complete
            && cycle.failures.is_empty(),
        "cycle_review_unavailable"
    );
    runner::validate_complete_run(
        run,
        &cycle.run_id,
        &cycle.admission,
        &run.review_record.run.provider_route,
    )?;
    Ok(run)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Capsule {
    pub schema: String,
    pub operation_id: String,
    pub request_digest: String,
    pub candidate_revision: String,
    pub cycle_digest: String,
    pub admission_digest: String,
    pub expires_at: u64,
    /// Exact allowlisted producer JSON bytes; never provider logs or paths supplied by a client.
    pub files: BTreeMap<String, String>,
    pub digest: String,
}
impl Capsule {
    pub fn validate(&self, cycle: &UpdateCycleResult, now: u64) -> Result<()> {
        let run = review(cycle)?;
        let execution = cycle
            .execution
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("cycle_execution_missing"))?;
        let mut unsigned = self.clone();
        unsigned.digest.clear();
        ensure!(
            self.schema == "codefriend.cycle_review_evidence.v1"
                && self.digest == hash(&unsigned)?
                && self.operation_id == cycle.run_id
                && self.cycle_digest == hash(cycle)?
                && self.request_digest == execution.request_digest
                && self.candidate_revision == execution.candidate_revision
                && self.admission_digest == cycle.admission.digest
                && self.expires_at <= cycle.admission.expires_at
                && now < self.expires_at
                && serde_json::to_vec(self)?.len() <= LIMIT
                && self.files.len() == 10
                && self.files.values().all(|v| v.len() <= FILE_LIMIT),
            "cycle_capsule_binding"
        );
        let retained: FourPerspectiveReviewRun = self.read("run.json")?;
        ensure!(&retained == run, "cycle_capsule_review_changed");
        let record: super::evidence::contracts::ReviewRecord = self.read("review-record.json")?;
        ensure!(record == run.review_record, "cycle_capsule_record_changed");
        for lane in ReviewLane::ALL {
            let (expected, _) = if run.review_record.run.assessment_generation() {
                runner::assessment_lane_input_manifest_version(
                    &run.run_id,
                    lane,
                    &cycle.admission,
                    run.review_record
                        .run
                        .lane_versions
                        .get(lane.id())
                        .ok_or_else(|| anyhow::anyhow!("cycle_capsule_lane_contract"))?,
                )?
            } else {
                runner::lane_input_manifest(&run.run_id, lane, &cycle.admission)?
            };
            let input: LaneInputManifest = self.read(&format!("lanes/{}/input.json", lane.id()))?;
            ensure!(input == expected, "cycle_capsule_input_changed");
            let result: LaneResult = self.read(&format!("lanes/{}/result.json", lane.id()))?;
            ensure!(
                result.lane == lane.id() && run.lane_results.contains(&result),
                "cycle_capsule_lane_changed"
            );
        }
        Ok(())
    }
    fn read<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        Ok(serde_json::from_str(self.files.get(path).ok_or_else(
            || anyhow::anyhow!("cycle_capsule_file_missing"),
        )?)?)
    }
    pub fn capture(root: &Path, cycle: &UpdateCycleResult, now: u64) -> Result<Self> {
        review(cycle)?;
        let execution = cycle
            .execution
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("cycle_execution_missing"))?;
        let mut names = vec!["run.json".to_owned(), "review-record.json".to_owned()];
        for lane in ReviewLane::ALL {
            for name in ["input.json", "result.json"] {
                names.push(format!("lanes/{}/{name}", lane.id()));
            }
        }
        let mut files = BTreeMap::new();
        let mut total = 0usize;
        for name in names {
            let path = root.join(&name);
            let mut current = root.to_path_buf();
            ensure!(
                !fs::symlink_metadata(&current)?.file_type().is_symlink(),
                "cycle_capsule_symlink"
            );
            for component in Path::new(&name).components() {
                current.push(component);
                ensure!(
                    !fs::symlink_metadata(&current)?.file_type().is_symlink(),
                    "cycle_capsule_symlink"
                );
            }
            ensure!(
                fs::metadata(&path)?.is_file() && fs::metadata(&path)?.len() <= FILE_LIMIT as u64,
                "cycle_capsule_file_limit"
            );
            let mut bytes = String::new();
            fs::File::open(path)?
                .take(FILE_LIMIT as u64 + 1)
                .read_to_string(&mut bytes)?;
            ensure!(bytes.len() <= FILE_LIMIT, "cycle_capsule_file_limit");
            total += serde_json::to_vec(&bytes)?.len() + name.len() + 4;
            ensure!(total <= LIMIT - 4096, "cycle_capsule_total_limit");
            files.insert(name, bytes);
        }
        let mut capsule = Self {
            schema: "codefriend.cycle_review_evidence.v1".into(),
            operation_id: cycle.run_id.clone(),
            request_digest: execution.request_digest.clone(),
            candidate_revision: execution.candidate_revision.clone(),
            cycle_digest: hash(cycle)?,
            admission_digest: cycle.admission.digest.clone(),
            expires_at: cycle.admission.expires_at,
            files,
            digest: String::new(),
        };
        capsule.digest = hash(&capsule)?;
        capsule.validate(cycle, now)?;
        Ok(capsule)
    }
}

/// Both admissions remain intact. This receipt identifies imported gateway bytes,
/// not original local acquisition or independent provider verification.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ImportBinding {
    pub schema: String,
    pub website_run_id: String,
    pub agent_id: String,
    pub consent_digest: String,
    pub local_admission: Admission,
    pub capsule: Capsule,
}
