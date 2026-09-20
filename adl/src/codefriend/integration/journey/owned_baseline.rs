//! Borrowed authenticated hosted owners; saved identities cannot restore authority.
use super::*;
use crate::codefriend::memory::baseline::{BaselineAccess, BaselineRef};

const VERSION: &str = "codefriend.owned_drift_intent.v1";

pub(crate) struct OwnedBaseline<'a> {
    pub operation: &'a str,
    pub store: &'a Store,
    pub graph: &'a structure::StructureReport,
    pub review: &'a crate::codefriend::evidence::contracts::ReviewRecord,
    pub baseline_root: &'a Path,
    pub expires_at: u64,
}

impl OwnedBaseline<'_> {
    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            !self.operation.is_empty() && self.operation.len() <= 256,
            "journey_baseline_operation_invalid"
        );
        self.graph.validate(self.store)?;
        self.review.validate()?;
        ensure!(
            self.review.admission == self.graph.record.admission,
            "journey_baseline_review_changed"
        );
        ensure!(
            now() < self.expires_at && self.expires_at <= self.graph.record.admission.expires_at,
            "journey_baseline_expired"
        );
        Ok(())
    }

    fn intent(&self) -> Result<OwnedDriftIntent> {
        self.validate()?;
        Ok(OwnedDriftIntent {
            schema: VERSION.into(),
            operation: self.operation.into(),
            baseline: BaselineRef::from_record(&self.graph.record)?,
            admission_digest: self.graph.record.admission.digest.clone(),
            expires_at: self.expires_at,
        })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct OwnedDriftIntent {
    schema: String,
    operation: String,
    baseline: BaselineRef,
    admission_digest: String,
    expires_at: u64,
}

/// Retain only comparison references and locations in the current operation.
/// Full original graphs contain admitted source and remain with their owners.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct OwnedDriftReport {
    schema: String,
    original_report_digest: String,
    expires_at: u64,
    graph_comparison: crate::codefriend::memory::comparison::DeltaReport,
    structural_comparison: crate::codefriend::memory::comparison::DeltaReport,
    current_traces: Vec<drift::FactTrace>,
}
impl OwnedDriftReport {
    fn from_report(report: drift::DriftReport, deadline: u64) -> Self {
        Self {
            schema: "codefriend.owned_drift.v1".into(),
            original_report_digest: report.digest,
            expires_at: deadline
                .min(report.baseline.record.admission.expires_at)
                .min(report.current.record.admission.expires_at),
            graph_comparison: report.graph_comparison,
            structural_comparison: report.structural_comparison,
            current_traces: report.current_traces,
        }
    }
}

fn saved(output: &Path) -> Result<Option<OwnedDriftIntent>> {
    let path = output.join("intent-drift.json");
    if !path.exists() {
        return Ok(None);
    }
    let value: serde_json::Value = read_typed(&path)?;
    if value.get("schema").is_none() {
        return Ok(None); // Existing checkout Continuation::Drift format.
    }
    let intent: OwnedDriftIntent = serde_json::from_value(value)?;
    ensure!(intent.schema == VERSION, "journey_baseline_intent_version");
    intent.baseline.validate()?;
    Ok(Some(intent))
}

pub(crate) fn operation(output: &Path) -> Result<Option<String>> {
    private_journey(output)?;
    Ok(saved(output)?.map(|i| i.operation))
}

/// A baseline graph has its own original policy/admission binding. Its prior
/// comparisons are not authority for this graph and are not recursively loaded.
pub(crate) fn validate_graph_source(
    output: &Path,
    graph: &structure::StructureReport,
) -> Result<()> {
    private_journey(output)?;
    let session: PersistedSession = read_typed(&output.join("session.json"))?;
    let binding = session.resume_binding(output)?;
    ensure!(
        matches!(binding.source, PathBoundary::Owned { .. }),
        "journey_owned_baseline_required"
    );
    session.validate_admission(&graph.record.admission)?;
    ensure!(
        hash(&binding.boundary_policy)? == hash(&graph.policy)?,
        "journey_baseline_policy_changed"
    );
    // Validate the original graph's immutable checkpoint owner without treating
    // saved comparison dependencies as authority for this producer output.
    let all = inventory(output)?;
    let session_digest = hash(&session)?;
    let mut previous = None;
    let mut last = None;
    let mut count = 0;
    for index in 0..MAX_CHECKPOINTS {
        let path = output.join(format!("checkpoint-{index:04}.json"));
        if !path.exists() {
            break;
        }
        let cp: Checkpoint = read_typed(&path)?;
        let mut unsigned = cp.clone();
        unsigned.digest.clear();
        ensure!(
            cp.schema == "codefriend.journey_checkpoint.v1"
                && cp.sequence == index
                && cp.previous == previous
                && cp.session_digest == session_digest
                && cp.digest == hash(&unsigned)?,
            "journey_baseline_checkpoint_changed"
        );
        for (name, digest) in &cp.files {
            ensure!(
                all.get(name) == Some(digest),
                "journey_baseline_artifact_changed"
            );
        }
        previous = Some(cp.digest.clone());
        last = Some(cp);
        count += 1;
    }
    let cp = last.ok_or_else(|| anyhow::anyhow!("journey_baseline_checkpoint_missing"))?;
    let mut expected = cp.files;
    let latest = format!("checkpoint-{:04}.json", count - 1);
    expected.insert(latest.clone(), all[&latest].clone());
    ensure!(all == expected, "journey_baseline_uncheckpointed_state");
    let manifest: JourneyManifest =
        read_typed(&output.join(format!("journey-{:04}.json", count - 1)))?;
    let stage = manifest
        .stages
        .get("structure")
        .ok_or_else(|| anyhow::anyhow!("journey_baseline_structure_missing"))?;
    ensure!(
        manifest.candidate_revision == binding.candidate_revision
            && manifest.candidate_clean == (env!("CODEFRIEND_BUILD_CLEAN") == "true")
            && manifest.admission_digest == graph.record.admission.digest
            && stage.status == StageStatus::Complete
            && stage.artifact.as_deref() == Some("structure.json")
            && stage.digest.as_deref() == Some(hash(graph)?.as_str()),
        "journey_baseline_structure_binding_changed"
    );
    let retained: FourPerspectiveReviewRun = read_typed(&binding.review_root.join("run.json"))?;
    let original = crate::codefriend::publication::read_review(
        &binding.review_root.join("review-record.json"),
    )?;
    ensure!(
        retained.review_record == original && original.admission == graph.record.admission,
        "journey_baseline_review_changed"
    );
    Ok(())
}

pub(crate) fn original_review(
    output: &Path,
) -> Result<crate::codefriend::evidence::contracts::ReviewRecord> {
    let session: PersistedSession = read_typed(&output.join("session.json"))?;
    let binding = session.resume_binding(output)?;
    crate::codefriend::publication::read_review(&binding.review_root.join("review-record.json"))
}

pub(super) struct Pair<'a> {
    pub(super) owners: [(&'a AdmittedBaselines<'a>, BaselineRef); 2],
}
impl BaselineAccess for Pair<'_> {
    fn load(
        &self,
        reference: &BaselineRef,
    ) -> Result<crate::codefriend::evidence::contracts::ReviewRecord> {
        let mut matches = self.owners.iter().filter(|(_, r)| r == reference);
        let (owner, _) = matches
            .next()
            .ok_or_else(|| anyhow::anyhow!("journey_baseline_unknown"))?;
        let record = owner.load(reference)?;
        // Independently admitted identical packets can have the same full
        // reference. Both explicitly selected owners must remain live and agree.
        for (other, _) in matches {
            ensure!(
                other.load(reference)? == record,
                "journey_baseline_ambiguous"
            );
        }
        Ok(record)
    }
}

pub(super) fn validate_saved(
    source: &PathBoundary,
    output: &Path,
    store: &Store,
    deadline: Option<u64>,
    context: Option<&OwnedBaseline<'_>>,
) -> Result<bool> {
    let Some(intent) = saved(output)? else {
        return Ok(false);
    };
    ensure!(
        matches!(source, PathBoundary::Owned { .. }),
        "journey_owned_baseline_required"
    );
    let context = context.ok_or_else(|| anyhow::anyhow!("journey_baseline_authority_required"))?;
    ensure!(
        intent == context.intent()?,
        "journey_baseline_binding_changed"
    );
    if !output.join("drift.json").exists() {
        return Ok(true); // A failed reservation has no comparison payload.
    }
    let report: OwnedDriftReport = read_typed(&output.join("drift.json"))?;
    let current: structure::StructureReport = read_typed(&output.join("structure.json"))?;
    let before = AdmittedBaselines::open(context.store, context.baseline_root, false)?;
    let after = AdmittedBaselines::open(store, &output.join("hosted-baselines"), false)?;
    let pair = Pair {
        owners: [
            (&before, intent.baseline),
            (&after, BaselineRef::from_record(&current.record)?),
        ],
    };
    let original = drift::architecture_drift_reporter_pair(
        context.store,
        store,
        &pair,
        context.graph.clone(),
        current,
    )?;
    ensure!(
        report
            == OwnedDriftReport::from_report(
                original,
                deadline.unwrap_or(u64::MAX).min(context.expires_at)
            ),
        "journey_owned_drift_changed"
    );
    context.validate()?;
    Ok(true)
}

impl Journey {
    pub(crate) fn continue_owned_drift(&mut self, baseline: &OwnedBaseline<'_>) -> Result<()> {
        ensure!(
            matches!(self.source, PathBoundary::Owned { .. }),
            "journey_owned_baseline_required"
        );
        self.pending("drift")?;
        let intent = baseline.intent()?;
        let current = self
            .graph
            .clone()
            .ok_or_else(|| anyhow::anyhow!("journey_structure_missing"))?;
        write_json_create_only(&self.output.join("intent-drift.json"), &intent)?;
        self.persist()?;
        let result: Result<drift::DriftReport> = (|| {
            let before = AdmittedBaselines::open(baseline.store, baseline.baseline_root, true)?;
            let after =
                AdmittedBaselines::open(&self.store, &self.output.join("hosted-baselines"), true)?;
            let pair = Pair {
                owners: [
                    (&before, before.retain(&baseline.graph.record)?),
                    (&after, after.retain(&current.record)?),
                ],
            };
            let report = drift::architecture_drift_reporter_pair(
                baseline.store,
                &self.store,
                &pair,
                baseline.graph.clone(),
                current,
            )?;
            baseline.validate()?;
            Ok(report)
        })();
        match result {
            Ok(report) => {
                let report = OwnedDriftReport::from_report(
                    report,
                    self.deadline.unwrap_or(u64::MAX).min(baseline.expires_at),
                );
                self.record(
                    "drift",
                    &report,
                    report.graph_comparison.comparable && report.structural_comparison.comparable,
                )
            }
            Err(_) => self.failed("drift", "drift_baseline_unavailable_or_incompatible"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codefriend::evidence::contracts::{ReviewRecord, Run};
    use std::sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    };

    // PVF: deterministic original Store admissions at one fixed timestamp;
    // synthetic empty review findings, no Runtime/provider acceptance claim.
    #[test]
    fn identical_reference_requires_both_selected_original_owners_live() {
        let fixture: ReviewRecord = serde_json::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/codefriend/evidence/review-v1.json"
        )))
        .unwrap();
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let second_clock = Arc::new(AtomicU64::new(100));
        let clock = second_clock.clone();
        let first = Store::open(&dir.path().join("first"), || 100).unwrap();
        let second = Store::open(&dir.path().join("second"), move || {
            clock.load(Ordering::SeqCst)
        })
        .unwrap();
        let admit = |store: &Store| {
            let admission = store
                .admit(
                    fixture.admission.packet.clone(),
                    Retention { seconds: 1000 },
                )
                .unwrap();
            let run = Run::new(
                &admission,
                fixture.run.lane_versions.clone(),
                "fixture:no-provider".into(),
                Completion::Complete,
                vec![],
            )
            .unwrap();
            ReviewRecord {
                admission,
                run,
                findings: vec![],
            }
        };
        let first_record = admit(&first);
        let second_record = admit(&second);
        assert_eq!(first_record, second_record);
        let first_owner =
            AdmittedBaselines::open(&first, &dir.path().join("first-refs"), true).unwrap();
        let second_owner =
            AdmittedBaselines::open(&second, &dir.path().join("second-refs"), true).unwrap();
        let reference = first_owner.retain(&first_record).unwrap();
        assert_eq!(reference, second_owner.retain(&second_record).unwrap());
        let pair = Pair {
            owners: [
                (&first_owner, reference.clone()),
                (&second_owner, reference.clone()),
            ],
        };
        assert_eq!(pair.load(&reference).unwrap(), first_record);
        second_clock.store(1200, Ordering::SeqCst);
        assert!(pair.load(&reference).is_err());
        second_clock.store(100, Ordering::SeqCst);
        second_owner.delete(&reference).unwrap();
        assert!(pair.load(&reference).is_err());
        assert!(first_owner.load(&reference).is_ok());
    }
}
