//! Hosted Palace effects retain original review references and operator authority.
use super::owned_baseline::{OwnedBaseline, Pair};
use super::*;
use crate::codefriend::memory::{baseline::BaselineRef, palace_authority};

const VERSION: &str = "codefriend.owned_palace_intent.v1";

/// Constructed only by the authenticated server owner, never from a browser path.
pub(crate) struct AuthorityContext {
    pub root: PathBuf,
    pub palace_root: PathBuf,
}
#[derive(Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct AuthorityPin {
    trust: String,
    evidence: String,
    identity: String,
    continuity: String,
}
impl AuthorityContext {
    fn load(
        &self,
    ) -> Result<(
        adl_runtime_kernel::VerifiedMemoryPalaceAuthority,
        AuthorityPin,
    )> {
        let trust_path = self.root.join("trust.json");
        let evidence_path = self.root.join("authority-evidence.json");
        safe_absolute(&trust_path)?;
        safe_absolute(&evidence_path)?;
        ensure!(
            fs::metadata(&trust_path)?.len() <= 16 * 1024
                && fs::metadata(&evidence_path)?.len() <= 2 * 1024 * 1024,
            "journey_palace_authority_bounds"
        );
        let trust: palace_authority::Trust = read_typed(&trust_path)?;
        let evidence: palace_authority::Evidence = read_typed(&evidence_path)?;
        let authority = palace_authority::provision(&trust_path, &evidence_path)?;
        let pin = AuthorityPin {
            trust: hash(&trust)?,
            evidence: hash(&evidence)?,
            identity: authority.identity().identity_root.clone(),
            continuity: authority.continuity().record().continuity_head.clone(),
        };
        // Detect input replacement during Runtime admission.
        ensure!(
            hash(&read_typed::<palace_authority::Trust>(&trust_path)?)? == pin.trust
                && hash(&read_typed::<palace_authority::Evidence>(&evidence_path)?)?
                    == pin.evidence,
            "journey_palace_authority_changed"
        );
        Ok((authority, pin))
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Intent {
    schema: String,
    operation: String,
    baseline: BaselineRef,
    current: BaselineRef,
    authority: AuthorityPin,
    observed_epoch_ms: u64,
    expires_at: u64,
}
fn saved(output: &Path) -> Result<Option<Intent>> {
    let path = output.join("intent-palace_comparison.json");
    if !path.exists() {
        return Ok(None);
    }
    let value: serde_json::Value = read_typed(&path)?;
    if value.get("schema").is_none() {
        return Ok(None);
    }
    let intent: Intent = serde_json::from_value(value)?;
    ensure!(intent.schema == VERSION, "journey_palace_intent_version");
    intent.baseline.validate()?;
    intent.current.validate()?;
    Ok(Some(intent))
}
pub(crate) fn operation(output: &Path) -> Result<Option<String>> {
    private_journey(output)?;
    Ok(saved(output)?.map(|v| v.operation))
}
fn request(intent: &Intent) -> Result<palace::RetrieveRequest> {
    let observed = now().saturating_mul(1000);
    ensure!(
        now() < intent.expires_at && intent.observed_epoch_ms <= observed,
        "journey_palace_expired"
    );
    let stale = intent
        .expires_at
        .saturating_mul(1000)
        .checked_sub(intent.observed_epoch_ms)
        .filter(|v| *v > 0)
        .ok_or_else(|| anyhow::anyhow!("journey_palace_expired"))?;
    Ok(palace::RetrieveRequest {
        schema: palace::VERSION.into(),
        baseline: intent.baseline.clone(),
        current: intent.current.clone(),
        expected_identity_root: intent.authority.identity.clone(),
        expected_continuity_head: intent.authority.continuity.clone(),
        packet_observed_epoch_ms: intent.observed_epoch_ms,
        observed_epoch_ms: observed,
        stale_after_ms: stale,
        max_working_set_items: 2,
    })
}

pub(super) fn validate_saved(
    source: &PathBoundary,
    output: &Path,
    store: &Store,
    baseline: Option<&OwnedBaseline<'_>>,
    authority: Option<&AuthorityContext>,
    review: Option<&FourPerspectiveReviewRun>,
) -> Result<bool> {
    let Some(intent) = saved(output)? else {
        return Ok(false);
    };
    ensure!(
        matches!(source, PathBoundary::Owned { .. }),
        "journey_owned_baseline_required"
    );
    let baseline =
        baseline.ok_or_else(|| anyhow::anyhow!("journey_baseline_authority_required"))?;
    let authority =
        authority.ok_or_else(|| anyhow::anyhow!("journey_palace_authority_required"))?;
    baseline.validate()?;
    source.check(&authority.palace_root)?;
    let (_, pin) = authority.load()?;
    let review = review.ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
    ensure!(
        intent.operation == baseline.operation
            && intent.authority == pin
            && intent.baseline == BaselineRef::from_record(baseline.review)?
            && intent.current == BaselineRef::from_record(&review.review_record)?
            && intent.expires_at <= baseline.expires_at
            && intent.expires_at <= review.review_record.admission.expires_at,
        "journey_palace_binding_changed"
    );
    let req = request(&intent)?;
    // An interrupted reservation is observable but never indexed again.
    if !output.join("palace_comparison.json").exists() {
        return Ok(true);
    }
    let before = AdmittedBaselines::open(baseline.store, baseline.baseline_root, false)?;
    let after = AdmittedBaselines::open(store, &output.join("hosted-palace-baselines"), false)?;
    let pair = Pair {
        owners: [
            (&before, intent.baseline.clone()),
            (&after, intent.current.clone()),
        ],
    };
    let retained: palace::RetrievedComparison = read_typed(&output.join("palace_comparison.json"))?;
    let current = palace::retrieve_with_baselines(&pair, &authority.palace_root, &req)?;
    ensure!(
        current.schema == retained.schema
            && current.provenance == retained.provenance
            && current.selected_references == retained.selected_references
            && current.delta == retained.delta
            && retained.observed_epoch_ms == intent.observed_epoch_ms,
        "journey_palace_changed"
    );
    baseline.validate()?;
    ensure!(
        authority.load()?.1 == intent.authority,
        "journey_palace_authority_changed"
    );
    request(&intent)?;
    Ok(true)
}

impl Journey {
    pub(crate) fn validate_owned_palace(
        &self,
        baseline: Option<&OwnedBaseline<'_>>,
        authority: &AuthorityContext,
    ) -> Result<()> {
        validate_saved(
            &self.source,
            &self.output,
            &self.store,
            baseline,
            Some(authority),
            self.review.as_ref(),
        )?;
        self.live_admission()?;
        Ok(())
    }

    pub(crate) fn continue_owned_palace(
        &mut self,
        baseline: &OwnedBaseline<'_>,
        authority: &AuthorityContext,
    ) -> Result<()> {
        ensure!(
            matches!(self.source, PathBoundary::Owned { .. }),
            "journey_owned_baseline_required"
        );
        self.pending("palace_comparison")?;
        baseline.validate()?;
        self.source.check(&authority.palace_root)?;
        let (verified, pin) = authority.load()?;
        let review = self
            .review
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
        let intent = Intent {
            schema: VERSION.into(),
            operation: baseline.operation.into(),
            baseline: BaselineRef::from_record(baseline.review)?,
            current: BaselineRef::from_record(&review.review_record)?,
            authority: pin,
            observed_epoch_ms: now().saturating_mul(1000),
            expires_at: baseline
                .expires_at
                .min(review.review_record.admission.expires_at)
                .min(self.deadline.unwrap_or(u64::MAX)),
        };
        let mut req = request(&intent)?;
        req.observed_epoch_ms = intent.observed_epoch_ms;
        let review_record = review.review_record.clone();
        write_json_create_only(&self.output.join("intent-palace_comparison.json"), &intent)?;
        self.persist()?;
        let result: Result<palace::RetrievedComparison> = (|| {
            let before = AdmittedBaselines::open(baseline.store, baseline.baseline_root, true)?;
            let after = AdmittedBaselines::open(
                &self.store,
                &self.output.join("hosted-palace-baselines"),
                true,
            )?;
            ensure!(
                before.retain(baseline.review)? == intent.baseline
                    && after.retain(&review_record)? == intent.current,
                "journey_palace_review_changed"
            );
            let pair = Pair {
                owners: [
                    (&before, intent.baseline.clone()),
                    (&after, intent.current.clone()),
                ],
            };
            let mut references = vec![intent.baseline.clone()];
            if intent.current != intent.baseline {
                references.push(intent.current.clone());
            }
            palace::index_with_baselines(
                &pair,
                &authority.palace_root,
                &verified,
                &palace::IndexRequest {
                    schema: palace::VERSION.into(),
                    references,
                    observed_epoch_ms: intent.observed_epoch_ms,
                    stale_after_ms: req.stale_after_ms,
                    max_working_set_items: 2,
                },
            )?;
            let report = palace::retrieve_with_baselines(&pair, &authority.palace_root, &req)?;
            baseline.validate()?;
            ensure!(
                authority.load()?.1 == intent.authority,
                "journey_palace_authority_changed"
            );
            request(&intent)?;
            Ok(report)
        })();
        match result {
            Ok(report) => self.record("palace_comparison", &report, report.delta.comparable),
            Err(_) => self.failed("palace_comparison", "palace_comparison_failed_or_uncertain"),
        }
    }
}
