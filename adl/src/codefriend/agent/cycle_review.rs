//! Reuse the exact review in a validated cycle; never invoke a provider here.
use super::*;
use crate::codefriend::{
    activities::UpdateCycleResult,
    evidence::{store::Store, Admission},
    review::runner::FourPerspectiveReviewRun,
};

impl RunReport {
    /// Callers validate the whole report first, preserving the outer agent/run
    /// identity and the cycle's distinct gateway operation identity.
    pub(crate) fn selected_review(&self) -> Result<&FourPerspectiveReviewRun> {
        ensure!(self.status == "complete", "agent_review_not_complete");
        let review = match (&self.result, &self.cycle_result) {
            (Some(review), None) => review,
            (None, Some(cycle))
                if cycle.completion
                    == crate::codefriend::evidence::contracts::Completion::Complete
                    && cycle.failures.is_empty() =>
            {
                cycle
                    .review
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("agent_cycle_review_missing"))?
            }
            _ => anyhow::bail!("agent_review_unavailable"),
        };
        review.successful_execution()?;
        Ok(review)
    }
    pub(super) fn check_original_cycle(&self, root: &Path, now: u64) -> Result<()> {
        if let Some(cycle) = &self.cycle_result {
            let original: Admission =
                publication::read(&root.join("admission.json"), MAX_RESPONSE as usize)?;
            ensure!(
                original.packet == cycle.admission.packet,
                "agent_cycle_original_admission_changed"
            );
            let store = Store::open(&root.join("evidence"), move || now)?;
            ensure!(
                store.get(&original.packet.packet_id)? == original,
                "agent_cycle_original_admission_changed"
            );
        }
        Ok(())
    }
    pub(super) fn review_store(&self, root: &Path) -> PathBuf {
        root.join(if self.cycle_result.is_some() {
            "cycle-evidence"
        } else {
            "evidence"
        })
    }
}

pub(super) fn retain(
    root: &Path,
    original: &Admission,
    cycle: &UpdateCycleResult,
    now: u64,
) -> Result<()> {
    let Some(review) = &cycle.review else {
        return Ok(());
    };
    ensure!(
        original.packet == cycle.admission.packet
            && now < original.expires_at
            && now < cycle.admission.expires_at,
        "agent_cycle_original_admission_changed"
    );
    let execution = cycle
        .execution
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("agent_cycle_execution_missing"))?;
    cycle.validate(
        &crate::codefriend::review::runner::provider_route_identity_from_model(
            &execution.model_identity,
        ),
    )?;
    review.successful_execution()?;
    let store = Store::open(&root.join("cycle-evidence"), move || now)?;
    store.retain_original(&cycle.admission)?;
    let output = root.join("work/review");
    fs::create_dir_all(&output)?;
    fs::set_permissions(&output, fs::Permissions::from_mode(0o700))?;
    // These are a local projection of the original validated response, not a
    // new review. The report and immutable Journey inventory bind their bytes.
    save_private(&output.join("run.json"), review)?;
    save_private(&output.join("review-record.json"), &review.review_record)?;
    Ok(())
}
