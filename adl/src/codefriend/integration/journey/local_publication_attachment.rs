//! Attach an already committed paired-agent export without dispatching effects.
use super::*;
use crate::codefriend::{
    agent::{publication as local, RunReport},
    evidence::contracts::Publication,
    publication::{self, DecisionKind},
};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attachment {
    binding: local::Binding,
    decision: local::Decision,
    prepared_digest: String,
    terminal_digest: String,
    publication_digest: String,
    decision_digest: String,
    result_digest: String,
    artifact_files: BTreeMap<String, String>,
    export_files: BTreeMap<String, String>,
}
struct Observed {
    marker: Attachment,
    publication: Publication,
    result: Value,
}

fn observe(
    source: &PathBoundary,
    review: &FourPerspectiveReviewRun,
    binding: &local::Binding,
    requested: &local::Decision,
) -> Result<Observed> {
    let PathBoundary::Owned { root, .. } = source else {
        anyhow::bail!("journey_local_owner_required")
    };
    ensure!(
        !binding.job_id.is_empty()
            && binding.job_id.len() <= 80
            && binding
                .job_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
        "journey_local_job_identity"
    );
    let report: RunReport = read_typed(&source.check(&root.join("report.json"))?)?;
    ensure!(
        report.result.as_ref() == Some(review),
        "journey_local_report_changed"
    );
    let job = source.check(&root.join("work/publications").join(&binding.job_id))?;
    let prepared: local::Stage = read_typed(&job.join("prepared-stage.json"))?;
    let terminal: local::Stage = read_typed(&job.join("terminal-stage.json"))?;
    let observed_at = now();
    let context = local::VerificationContext {
        schema: "codefriend.agent_publication_verifier_context.v1".into(),
        binding: binding.clone(),
        report,
        decision: Some(requested.clone()),
        prepared: Some(prepared.clone()),
        now: observed_at,
    };
    local::verify_stage(&terminal, &context, observed_at)?;
    ensure!(
        terminal.stage == "terminal"
            && terminal.agent_candidate_revision == env!("CODEFRIEND_BUILD_REVISION"),
        "journey_local_candidate_changed"
    );
    let payload: local::Terminal = serde_json::from_value(terminal.payload.clone())?;
    ensure!(
        payload.status == "complete",
        "journey_local_export_not_complete"
    );
    let format = binding.format;
    let bundle = job.join("bundle");
    let publication = publication::read_publication(&bundle.join("publication.json"))?;
    publication.validate(&review.review_record)?;
    ensure!(
        publication.target == format.target()
            && publication.renderer_versions
                == BTreeMap::from([(format.key().into(), format.renderer_version().into())]),
        "journey_export_format_changed"
    );
    publication::verify_artifacts(&bundle.join("artifacts"), &publication.artifact_manifest)?;
    let head = publication::read_decision_head(
        &job.join("approvals"),
        &review.review_record,
        &publication,
    )?
    .ok_or_else(|| anyhow::anyhow!("journey_local_approval_missing"))?;
    let native = payload
        .native
        .decision
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("journey_local_approval_missing"))?;
    ensure!(
        head.digest == native.digest
            && hash(&head)? == hash(native)?
            && head.decision == DecisionKind::Approved
            && head.channel == "authenticated_local_agent",
        "journey_local_approval_changed"
    );
    let target = source.check(&job.join("exports").join(&publication.target))?;
    let manifest_bytes = bounded_read(&target.join("manifest.json"))?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let name = match format {
        PublicationFormat::Markdown => "report.md",
        PublicationFormat::Html => "report.html",
        PublicationFormat::Pdf => "report.pdf",
    };
    let bytes = bounded_read(&target.join(name))?;
    let result = publication::relay::result_from_manifest(format, &manifest)?;
    ensure!(
        payload.native.render.as_ref() == Some(&result)
            && payload.native.manifest.as_ref() == Some(&manifest)
            && result["manifest_digest"].as_str()
                == Some(crate::codefriend::ingestion::digest(&manifest_bytes).as_str())
            && terminal.exports[0].digest == crate::codefriend::ingestion::digest(&bytes),
        "journey_local_export_changed"
    );
    publication::relay::verify_rendered(
        &review.review_record,
        &head,
        format,
        &result,
        &manifest,
        &bytes,
    )?;
    let artifact_files = inventory(&bundle.join("artifacts"))?;
    let export_files = inventory(&target)?;
    // Recheck retention after all bounded byte validation and inventory reads.
    local::verify_stage(&terminal, &context, now())?;
    Ok(Observed {
        marker: Attachment {
            binding: binding.clone(),
            decision: requested.clone(),
            prepared_digest: prepared.digest,
            terminal_digest: terminal.digest,
            publication_digest: hash(&publication)?,
            decision_digest: head.digest,
            result_digest: hash(&result)?,
            artifact_files,
            export_files,
        },
        publication,
        result,
    })
}

pub(super) fn validate(
    source: &PathBoundary,
    output: &Path,
    manifest: &JourneyManifest,
    review: Option<&FourPerspectiveReviewRun>,
    format: PublicationFormat,
) -> Result<bool> {
    let marker = output.join(format!("local-publication-{}.json", format.key()));
    if !marker.exists() {
        return Ok(false);
    }
    let saved: Attachment = read_typed(&marker)?;
    ensure!(
        saved.binding.format == format,
        "journey_local_format_changed"
    );
    let observed = observe(
        source,
        review.ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?,
        &saved.binding,
        &saved.decision,
    )?;
    ensure!(
        hash(&saved)? == hash(&observed.marker)?,
        "journey_local_attachment_changed"
    );
    for (name, digest) in [
        (
            format!("publication_{}", format.key()),
            saved.publication_digest,
        ),
        (
            format!("approval_{}", format.key()),
            hash(&saved.decision_digest)?,
        ),
        (format.key().into(), saved.result_digest),
    ] {
        let stage = &manifest.stages[&name];
        let path = output.join(format!("{name}.json"));
        let actual = if name.starts_with("publication_") {
            hash(&read_typed::<Publication>(&path)?)?
        } else if name.starts_with("approval_") {
            hash(&read_typed::<String>(&path)?)?
        } else {
            hash(&read_typed::<Value>(&path)?)?
        };
        ensure!(
            stage.status == StageStatus::Complete
                && stage.digest.as_deref() == Some(digest.as_str())
                && actual == digest,
            "journey_local_export_stage_changed"
        );
    }
    ensure!(
        now() < saved.binding.expires_at,
        "journey_local_export_expired"
    );
    Ok(true)
}

impl Journey {
    /// Caller retains live paired-job authority before and after this observation.
    pub(crate) fn attach_local_publication(
        &mut self,
        context: &local::VerificationContext,
    ) -> Result<()> {
        self.live_admission()?;
        context.report.validate(now())?;
        let format = context.binding.format;
        let review = self
            .review
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
        ensure!(
            context.report.result.as_ref() == Some(review) && context.now <= now(),
            "journey_local_context_changed"
        );
        let requested = context
            .decision
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("journey_local_decision_missing"))?;
        let observed = observe(&self.source, review, &context.binding, requested)?;
        ensure!(
            context.schema == "codefriend.agent_publication_verifier_context.v1"
                && context.report.digest == context.binding.report_digest
                && context
                    .prepared
                    .as_ref()
                    .is_some_and(|p| p.digest == observed.marker.prepared_digest),
            "journey_local_context_changed"
        );
        let key = format.key();
        let marker_path = self.output.join(format!("local-publication-{key}.json"));
        if marker_path.exists() {
            let saved: Attachment = read_typed(&marker_path)?;
            ensure!(
                hash(&saved)? == hash(&observed.marker)?,
                "journey_local_attachment_changed"
            );
            validate(
                &self.source,
                &self.output,
                &self.manifest,
                self.review.as_ref(),
                format,
            )?;
            return Ok(());
        }
        let publication_stage = format!("publication_{key}");
        let approval_stage = format!("approval_{key}");
        for name in [&publication_stage, &approval_stage, key] {
            self.pending(name)?;
        }
        self.live_admission()?;
        ensure!(
            now() < context.binding.expires_at,
            "journey_local_export_expired"
        );
        write_json_create_only(&marker_path, &observed.marker)?;
        write_json_create_only(
            &self.output.join(format!("{publication_stage}.json")),
            &observed.publication,
        )?;
        write_json_create_only(
            &self.output.join(format!("{approval_stage}.json")),
            &observed.marker.decision_digest,
        )?;
        write_json_create_only(&self.output.join(format!("{key}.json")), &observed.result)?;
        self.live_admission()?;
        ensure!(
            now() < context.binding.expires_at,
            "journey_local_export_expired"
        );
        for (name, digest) in [
            (publication_stage, observed.marker.publication_digest),
            (approval_stage, hash(&observed.marker.decision_digest)?),
            (key.into(), observed.marker.result_digest),
        ] {
            self.manifest.stages.insert(
                name.clone(),
                Stage {
                    status: StageStatus::Complete,
                    reason: None,
                    artifact: Some(format!("{name}.json")),
                    digest: Some(digest),
                },
            );
        }
        self.persist()
    }
}

#[cfg(all(test, unix))]
#[path = "../../../../tests/support/codefriend_local_attachment_tests.rs"]
mod tests;
