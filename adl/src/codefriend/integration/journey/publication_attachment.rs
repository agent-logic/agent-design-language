//! Observe existing hosted exports through their original owner. No renderer dispatch.
use super::*;
use crate::codefriend::{
    evidence::contracts::{Publication, ReviewRecord},
    publication::{self, DecisionKind},
};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attachment {
    subject: String,
    operation: String,
    publication_digest: String,
    decision_digest: String,
    result_digest: String,
    artifact_files: BTreeMap<String, String>,
    export_files: BTreeMap<String, String>,
}

struct Observed {
    publication: Publication,
    decision_digest: String,
    result: Value,
    artifact_files: BTreeMap<String, String>,
    export_files: BTreeMap<String, String>,
}

fn observe(
    source: &PathBoundary,
    review: &ReviewRecord,
    format: PublicationFormat,
    subject: &str,
    operation: &str,
) -> Result<Observed> {
    let PathBoundary::Owned { root, .. } = source else {
        anyhow::bail!("journey_hosted_owner_required")
    };
    let bundle = source.check(&root.join("publications").join(format.key()))?;
    let publication = publication::read_publication(&bundle.join("publication.json"))?;
    publication.validate(review)?;
    ensure!(
        publication.target == format.target()
            && publication.renderer_versions
                == BTreeMap::from([(format.key().into(), format.renderer_version().into())]),
        "journey_export_format_changed"
    );
    publication::verify_artifacts(&bundle.join("artifacts"), &publication.artifact_manifest)?;
    let decision = publication::read_decision_head(&bundle.join("approval"), review, &publication)?
        .ok_or_else(|| anyhow::anyhow!("journey_export_approval_missing"))?;
    let website = decision
        .website
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("journey_export_website_approval_required"))?;
    ensure!(
        decision.decision == DecisionKind::Approved
            && decision.channel == "authenticated_website"
            && decision.actor == subject
            && website.subject == subject
            && website.operation == operation
            && website.candidate_revision == env!("CODEFRIEND_BUILD_REVISION")
            && website.binding_digest == publication.binding_digest()?,
        "journey_export_approval_changed"
    );
    // Observe only the fixed native output beneath this operation's owner.
    let target = source.check(&root.join("exports").join(&publication.target))?;
    let manifest_bytes = bounded_read(&target.join("manifest.json"))?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let name = match format {
        PublicationFormat::Markdown => "report.md",
        PublicationFormat::Html => "report.html",
        PublicationFormat::Pdf => "report.pdf",
    };
    let bytes = bounded_read(&target.join(name))?;
    let result = publication::relay::result_from_manifest(format, &manifest)?;
    publication::relay::verify_rendered(review, &decision, format, &result, &manifest, &bytes)?;
    ensure!(
        result["manifest_digest"].as_str()
            == Some(crate::codefriend::ingestion::digest(&manifest_bytes).as_str()),
        "journey_export_manifest_changed"
    );
    Ok(Observed {
        publication,
        decision_digest: decision.digest,
        result,
        artifact_files: inventory(&bundle.join("artifacts"))?,
        export_files: inventory(&target)?,
    })
}

/// True means the hosted attachment was found and all three stages revalidated.
pub(super) fn validate(
    source: &PathBoundary,
    output: &Path,
    manifest: &JourneyManifest,
    review: Option<&FourPerspectiveReviewRun>,
    format: PublicationFormat,
) -> Result<bool> {
    let key = format.key();
    let marker = output.join(format!("hosted-publication-{key}.json"));
    if !marker.exists() {
        return Ok(false);
    }
    let saved: Attachment = read_typed(&marker)?;
    let review = review.ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
    ensure!(
        review.run_id == saved.operation,
        "journey_export_operation_changed"
    );
    let observed = observe(
        source,
        &review.review_record,
        format,
        &saved.subject,
        &saved.operation,
    )?;
    ensure!(
        saved.publication_digest == hash(&observed.publication)?
            && saved.decision_digest == observed.decision_digest
            && saved.result_digest == hash(&observed.result)?
            && saved.artifact_files == observed.artifact_files
            && saved.export_files == observed.export_files,
        "journey_hosted_export_changed"
    );
    for (name, digest) in [
        (format!("publication_{key}"), saved.publication_digest),
        (format!("approval_{key}"), hash(&saved.decision_digest)?),
        (key.into(), saved.result_digest),
    ] {
        let stage = &manifest.stages[&name];
        let path = output.join(format!("{name}.json"));
        let recorded_digest = if name.starts_with("publication_") {
            hash(&read_typed::<Publication>(&path)?)?
        } else if name.starts_with("approval_") {
            hash(&read_typed::<String>(&path)?)?
        } else {
            hash(&read_typed::<Value>(&path)?)?
        };
        ensure!(
            stage.status == StageStatus::Complete
                && stage.digest.as_deref() == Some(digest.as_str())
                && recorded_digest == digest,
            "journey_hosted_export_stage_changed"
        );
    }
    Ok(true)
}

impl Journey {
    /// Called only after the server authenticates the owner and observes a committed export.
    pub(crate) fn attach_hosted_publication(
        &mut self,
        format: PublicationFormat,
        subject: &str,
        operation: &str,
    ) -> Result<()> {
        self.live_admission()?;
        if validate(
            &self.source,
            &self.output,
            &self.manifest,
            self.review.as_ref(),
            format,
        )? {
            return Ok(());
        }
        let key = format.key();
        for name in [
            format!("publication_{key}"),
            format!("approval_{key}"),
            key.into(),
        ] {
            self.pending(&name)?;
        }
        let review = self
            .review
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
        ensure!(
            review.run_id == operation,
            "journey_export_operation_changed"
        );
        let observed = observe(
            &self.source,
            &review.review_record,
            format,
            subject,
            operation,
        )?;
        let marker = Attachment {
            subject: subject.into(),
            operation: operation.into(),
            publication_digest: hash(&observed.publication)?,
            decision_digest: observed.decision_digest.clone(),
            result_digest: hash(&observed.result)?,
            artifact_files: observed.artifact_files,
            export_files: observed.export_files,
        };
        self.live_admission()?;
        write_json_create_only(
            &self.output.join(format!("hosted-publication-{key}.json")),
            &marker,
        )?;
        // All three stages become visible in the same checkpoint. Never retain
        // a valid checkpoint that claims only part of this one observed effect.
        let publication_stage = format!("publication_{key}");
        let approval_stage = format!("approval_{key}");
        write_json_create_only(
            &self.output.join(format!("{publication_stage}.json")),
            &observed.publication,
        )?;
        write_json_create_only(
            &self.output.join(format!("{approval_stage}.json")),
            &observed.decision_digest,
        )?;
        write_json_create_only(&self.output.join(format!("{key}.json")), &observed.result)?;
        self.live_admission()?;
        for (name, digest) in [
            (publication_stage, hash(&observed.publication)?),
            (approval_stage, hash(&observed.decision_digest)?),
            (key.into(), hash(&observed.result)?),
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
