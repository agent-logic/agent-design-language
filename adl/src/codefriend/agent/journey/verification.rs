//! Pure native contract verification. Expected authority is supplied by the website;
//! absent artifact bytes and live remote authority are not proved by this verifier.
use super::*;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationContext {
    pub schema: String,
    pub job: Job,
    pub report: RunReport,
    pub receipt: ForwardReceipt,
    pub previous: Option<StageResult>,
    pub now: u64,
}
const STAGES: [&str; 18] = [
    "acquisition",
    "admission",
    "structure",
    "fitness",
    "impact",
    "rationale",
    "drift",
    "review",
    "publication_markdown",
    "publication_html",
    "publication_pdf",
    "palace_comparison",
    "approval_markdown",
    "approval_html",
    "approval_pdf",
    "markdown",
    "html",
    "pdf",
];

fn payload_outcome(stage: &native::Stage, success: bool, gap: bool) -> Result<()> {
    ensure!(
        stage.status
            == if success || gap {
                native::StageStatus::Complete
            } else {
                native::StageStatus::Failed
            }
            && stage.reason.as_deref()
                == if gap {
                    Some("analysis_gaps_reported")
                } else if success {
                    None
                } else {
                    Some("stage_incomplete_or_failed")
                },
        "agent_journey_payload_outcome"
    );
    Ok(())
}

fn analysis_outcome(
    stage: &native::Stage,
    completion: &crate::codefriend::evidence::contracts::Completion,
    analysis_complete: Option<bool>,
) -> Result<()> {
    use crate::codefriend::evidence::contracts::Completion;
    let complete = *completion == Completion::Complete;
    if let Some(claim) = analysis_complete {
        ensure!(claim == complete, "agent_journey_payload_completeness");
    }
    payload_outcome(
        stage,
        complete,
        analysis_complete.is_some() && *completion == Completion::Incomplete,
    )
}

fn snapshot(result: &StageResult, context: &VerificationContext, now: u64) -> Result<()> {
    let job = &context.job;
    let binding = &job.binding;
    let report = &context.report;
    report.validate(now)?;
    let receipt = &context.receipt;
    ensure!(
        matches!(binding.schema.as_str(), JOB_SCHEMA | JOB_SCHEMA_V2)
            && job.request.matches_schema(&binding.schema)
            && identifier(&binding.job_id)
            && binding.subject == report.subject
            && binding.agent_id == report.agent_id
            && binding.run_id == report.run_id
            && binding.consent_digest == report.consent_digest
            && binding.report_digest == report.digest
            && binding.expires_at == report.expires_at
            && binding.request_digest == job.request.digest()?
            && valid_digest(&binding.received_digest)
            && report.status == "complete",
        "agent_journey_verifier_binding"
    );
    ensure!(
        receipt.schema == "codefriend.agent_report_receipt.v1"
            && receipt.subject == binding.subject
            && receipt.agent_id == binding.agent_id
            && receipt.run_id == binding.run_id
            && receipt.report_digest == binding.report_digest
            && receipt.received_digest == binding.received_digest
            && receipt.consent_digest == binding.consent_digest
            && receipt.expires_at == binding.expires_at,
        "agent_journey_verifier_receipt"
    );
    ensure!(
        job.permitted_agent_candidate == env!("CODEFRIEND_BUILD_REVISION")
            && env!("CODEFRIEND_BUILD_CLEAN") == "true"
            && result.agent_candidate_revision == job.permitted_agent_candidate,
        "agent_journey_verifier_candidate"
    );
    if let Some(baseline) = job.request.baseline() {
        ensure!(
            identifier(baseline) && baseline != binding.run_id,
            "agent_journey_baseline_identity"
        );
    }
    if let Request::AttachPublication {
        publication_job, ..
    } = &job.request
    {
        ensure!(
            identifier(publication_job),
            "agent_journey_publication_identity"
        );
    }
    ensure!(
        result.schema
            == if binding.schema == JOB_SCHEMA_V2 {
                RESULT_SCHEMA_V2
            } else {
                RESULT_SCHEMA
            }
            && result.binding == *binding
            && (5..=64).contains(&result.checkpoint_sequence)
            && serde_json::to_vec(result)?.len() as u64 <= MAX_RESPONSE,
        "agent_journey_verifier_result"
    );
    let mut unsigned = result.clone();
    unsigned.digest.clear();
    ensure!(
        valid_digest(&result.digest) && hash(&unsigned)? == result.digest,
        "agent_journey_verifier_digest"
    );
    let run = report.completed_review()?;
    let admission = &run.review_record.admission;
    let manifest = &result.manifest;
    ensure!(
        manifest.schema
            == if binding.schema == JOB_SCHEMA_V2 {
                "codefriend.journey.v2"
            } else {
                "codefriend.journey.v1"
            }
            && manifest.candidate_revision == job.permitted_agent_candidate
            && manifest.candidate_clean
            && manifest.repository == admission.packet.repository
            && manifest.revision == admission.packet.revision
            && manifest.packet_id == admission.packet.packet_id
            && manifest.admission_digest == admission.digest
            && manifest.scope_digest == admission.packet.scope_digest
            && manifest.stages.len() == STAGES.len()
            && STAGES.iter().all(|key| manifest.stages.contains_key(*key)),
        "agent_journey_verifier_manifest"
    );
    for (name, stage) in &manifest.stages {
        let paired = stage.artifact.is_some() == stage.digest.is_some();
        ensure!(
            paired
                && stage
                    .artifact
                    .as_ref()
                    .is_none_or(|p| p == &format!("{name}.json"))
                && stage.digest.as_ref().is_none_or(|d| valid_digest(d)),
            "agent_journey_verifier_stage_artifact"
        );
        match stage.status {
            native::StageStatus::Pending => ensure!(
                stage.reason.is_none() && stage.artifact.is_none(),
                "agent_journey_verifier_pending"
            ),
            native::StageStatus::Complete => ensure!(
                stage.artifact.is_some()
                    && (stage.reason.is_none()
                        || (binding.schema == JOB_SCHEMA_V2
                            && matches!(
                                name.as_str(),
                                "structure"
                                    | "fitness"
                                    | "impact"
                                    | "rationale"
                                    | "drift"
                                    | "palace_comparison"
                            )
                            && stage.reason.as_deref() == Some("analysis_gaps_reported"))),
                "agent_journey_verifier_complete"
            ),
            native::StageStatus::Failed => ensure!(
                stage
                    .reason
                    .as_ref()
                    .is_some_and(|reason| !reason.is_empty()
                        && reason.len() <= 80
                        && reason.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')),
                "agent_journey_verifier_failed"
            ),
        }
    }
    let status = if manifest
        .stages
        .values()
        .any(|s| s.status == native::StageStatus::Failed)
    {
        native::StageStatus::Failed
    } else if manifest
        .stages
        .values()
        .all(|s| s.status == native::StageStatus::Complete)
    {
        native::StageStatus::Complete
    } else {
        native::StageStatus::Pending
    };
    ensure!(manifest.status == status, "agent_journey_verifier_status");
    for (name, digest) in [
        ("acquisition", hash(&admission.packet)?),
        ("admission", hash(admission)?),
        ("review", hash(run)?),
    ] {
        let stage = &manifest.stages[name];
        ensure!(
            stage.status == native::StageStatus::Complete
                && stage.digest.as_deref() == Some(digest.as_str()),
            "agent_journey_verifier_original_evidence"
        );
    }
    ensure!(
        manifest.stages["structure"].status != native::StageStatus::Pending
            && manifest.stages["fitness"].status != native::StageStatus::Pending,
        "agent_journey_verifier_preparation"
    );
    let required = match &job.request {
        Request::Impact { .. } => Some("impact"),
        Request::Rationale { .. } => Some("rationale"),
        Request::Drift { .. } => Some("drift"),
        Request::PalaceComparison { .. } => Some("palace_comparison"),
        Request::AttachPublication { format, .. } => Some(format.key()),
        _ => None,
    };
    if let Some(key) = required {
        ensure!(
            manifest.stages[key].status != native::StageStatus::Pending,
            "agent_journey_verifier_effect_unresolved"
        );
    }
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        let status = manifest.stages[format.key()].status.clone();
        ensure!(
            manifest.stages[&format!("publication_{}", format.key())].status == status
                && manifest.stages[&format!("approval_{}", format.key())].status == status,
            "agent_journey_verifier_partial_attachment"
        );
    }
    let requested = match &job.request {
        Request::Graph => Some(Artifact::Structure),
        Request::Artifact { artifact } => Some(artifact.clone()),
        _ => None,
    };
    match (requested, &result.payload) {
        (None, None) => {}
        (Some(artifact), Some(value)) => {
            let stage = &manifest.stages[artifact.key()];
            ensure!(
                stage.status != native::StageStatus::Pending
                    && stage.digest.as_deref() == Some(artifact_digest(&artifact, value)?.as_str()),
                "agent_journey_verifier_payload"
            );
            // Source-bearing native records must retain the original admission.
            let v2 = binding.schema == JOB_SCHEMA_V2;
            let record = match artifact {
                Artifact::Structure => {
                    let a: StructureArtifact = serde_json::from_value(value.clone())?;
                    ensure!(
                        matches!(&a, StructureArtifact::V2(_)) == v2,
                        "agent_journey_payload_version"
                    );
                    let analysis_complete = match &a {
                        StructureArtifact::V1(_) => None,
                        StructureArtifact::V2(value) => Some(value.analysis_complete),
                    };
                    analysis_outcome(stage, &a.record().run.completion, analysis_complete)?;
                    Some(a.record().clone())
                }
                Artifact::Fitness => {
                    let a: FitnessArtifact = serde_json::from_value(value.clone())?;
                    ensure!(
                        matches!(&a, FitnessArtifact::V2(_)) == v2,
                        "agent_journey_payload_version"
                    );
                    payload_outcome(stage, a.passes(), v2 && a.exit_code() == 2)?;
                    Some(a.record().clone())
                }
                Artifact::Impact => {
                    let a: ImpactArtifact = serde_json::from_value(value.clone())?;
                    ensure!(
                        matches!(&a, ImpactArtifact::V2(_)) == v2,
                        "agent_journey_payload_version"
                    );
                    let analysis_complete = match &a {
                        ImpactArtifact::V1(_) => None,
                        ImpactArtifact::V2(value) => Some(value.analysis_complete),
                    };
                    analysis_outcome(stage, &a.record().run.completion, analysis_complete)?;
                    Some(a.record().clone())
                }
                Artifact::Rationale => {
                    let a: RationaleArtifact = serde_json::from_value(value.clone())?;
                    ensure!(
                        matches!(&a, RationaleArtifact::V2(_)) == v2,
                        "agent_journey_payload_version"
                    );
                    let analysis_complete = match &a {
                        RationaleArtifact::V1(_) => None,
                        RationaleArtifact::V2(value) => Some(value.analysis_complete),
                    };
                    analysis_outcome(stage, &a.record().run.completion, analysis_complete)?;
                    Some(a.record().clone())
                }
                Artifact::Drift => {
                    let a: native::owned_baseline::OwnedDriftReport =
                        serde_json::from_value(value.clone())?;
                    ensure!(
                        a.schema()
                            == if v2 {
                                "codefriend.owned_drift.v2"
                            } else {
                                "codefriend.owned_drift.v1"
                            },
                        "agent_journey_payload_version"
                    );
                    payload_outcome(stage, a.comparable(), v2 && !a.comparable())?;
                    None
                }
                Artifact::PalaceComparison => {
                    let a: crate::codefriend::memory::palace::RetrievedComparison =
                        serde_json::from_value(value.clone())?;
                    payload_outcome(stage, a.delta.comparable, v2 && !a.delta.comparable)?;
                    None
                }
            };
            if let Some(record) = record {
                record.validate()?;
                ensure!(
                    record.admission == *admission,
                    "agent_journey_verifier_payload_admission"
                );
            }
        }
        _ => anyhow::bail!("agent_journey_verifier_payload_shape"),
    }
    if let Request::Prepare {
        boundary_policy,
        fitness_policy,
    } = &job.request
    {
        boundary_policy.validate(admission)?;
        fitness_policy.validate()?;
    }
    ensure!(now < binding.expires_at, "agent_journey_verifier_expired");
    Ok(())
}

/// Expected job, report, receipt and optional prior snapshot must come from the
/// authenticated website owner. This proves consistency, not absent source artifacts.
pub fn verify_stage(result: &StageResult, context: &VerificationContext, now: u64) -> Result<()> {
    ensure!(
        context.schema == "codefriend.agent_journey_verifier_context.v1"
            && context.now <= now
            && serde_json::to_vec(context)?.len() <= 9 * 1024 * 1024,
        "agent_journey_verifier_context"
    );
    snapshot(result, context, now)?;
    if let Some(previous) = &context.previous {
        snapshot(previous, context, now)?;
        ensure!(
            result.checkpoint_sequence >= previous.checkpoint_sequence,
            "agent_journey_verifier_regression"
        );
        if result.checkpoint_sequence == previous.checkpoint_sequence {
            ensure!(
                result.digest == previous.digest,
                "agent_journey_verifier_sequence_changed"
            );
        }
        for (name, stage) in &previous.manifest.stages {
            if stage.status != native::StageStatus::Pending {
                ensure!(
                    hash(stage)? == hash(&result.manifest.stages[name])?,
                    "agent_journey_verifier_committed_stage_changed"
                );
            }
        }
    }
    Ok(())
}

#[cfg(all(test, unix))]
#[path = "../../../../tests/support/codefriend_agent_journey_verification.rs"]
mod tests;
