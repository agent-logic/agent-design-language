//! Composition of existing owners. A prepared journey is not a completed Beta review.
//! Handles are created from local acquisition, never deserialized from an asserted manifest.
use super::PublicationFormat;
use crate::codefriend::{
    architecture::{drift, impact, rationale, structure},
    evidence::{contracts::Completion, hash, store::Store, Admission, Retention},
    governance::local as fitness,
    ingestion::{local, Scope},
    memory::{baseline::AdmittedBaselines, palace},
    publication::write_json_create_only,
    review::runner::{self, FourPerspectiveReviewRun, ReviewRunOptions},
};
use crate::provider_communication::ProviderInvocationRequestV1;
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Pending,
    Failed,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    pub status: StageStatus,
    /// Fixed reason code, never provider diagnostics or local path/secret content.
    pub reason: Option<String>,
    pub artifact: Option<String>,
    /// Canonical semantic digest from the shared evidence hash contract.
    pub digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JourneyManifest {
    pub schema: String,
    pub candidate_revision: String,
    pub candidate_clean: bool,
    pub repository: String,
    pub revision: String,
    pub packet_id: String,
    pub admission_digest: String,
    pub scope_digest: String,
    pub stages: BTreeMap<String, Stage>,
    pub status: StageStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalJourneyOptions {
    pub checkout: PathBuf,
    pub repository: String,
    pub revision: String,
    pub scope: Scope,
    pub store: PathBuf,
    /// New private output directory whose existing parent is outside the source checkout.
    pub output: PathBuf,
    pub retention: Retention,
    pub boundary_policy: structure::BoundaryPolicy,
    pub fitness_policy: fitness::Policy,
}

pub struct Journey {
    store: Store,
    store_path: PathBuf,
    output: PathBuf,
    source: PathBuf,
    manifest: JourneyManifest,
    graph: Option<structure::StructureReport>,
    review: Option<FourPerspectiveReviewRun>,
    sequence: usize,
    persistence_failed: bool,
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Reject aliases and parent traversal before any source-adjacent write.
fn outside_source(path: &Path, source: &Path) -> Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut cursor = PathBuf::new();
    for component in absolute.components() {
        ensure!(
            !matches!(component, Component::ParentDir),
            "journey_parent_path_rejected"
        );
        cursor.push(component);
        if let Ok(metadata) = fs::symlink_metadata(&cursor) {
            ensure!(
                !metadata.file_type().is_symlink(),
                "journey_symlink_rejected"
            );
        }
    }
    ensure!(
        !absolute.starts_with(source) && !source.starts_with(&absolute),
        "journey_output_overlaps_source"
    );
    Ok(absolute)
}

/// Acquisition and policy validation happen before a model executor is reachable.
/// Invalid input returns an error; execution failures after admission remain in the manifest.
pub fn prepare_local(options: LocalJourneyOptions) -> Result<Journey> {
    let source = fs::canonicalize(&options.checkout)?;
    let output = outside_source(&options.output, &source)?;
    let store_path = outside_source(&options.store, &source)?;
    ensure!(
        !output.starts_with(&store_path) && !store_path.starts_with(&output),
        "journey_store_output_overlap"
    );
    ensure!(
        output.parent().is_some_and(Path::is_dir) && !output.exists(),
        "journey_output_unavailable"
    );
    let packet = local::acquire(
        &source,
        &options.repository,
        &options.revision,
        options.scope,
    )?;
    let checked = Admission::new(packet.clone(), options.retention.clone(), now())?;
    checked.validate()?;
    options.boundary_policy.validate(&checked)?;
    options.fitness_policy.validate()?;
    let store = Store::open(&store_path, now)?;
    let admission = store.admit(packet, options.retention)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new().mode(0o700).create(&output)?;
    }
    #[cfg(not(unix))]
    fs::create_dir(&output)?;
    let mut manifest = JourneyManifest {
        schema: "codefriend.journey.v1".into(),
        candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
        candidate_clean: env!("CODEFRIEND_BUILD_CLEAN") == "true",
        repository: admission.packet.repository.clone(),
        revision: admission.packet.revision.clone(),
        packet_id: admission.packet.packet_id.clone(),
        admission_digest: admission.digest.clone(),
        scope_digest: admission.packet.scope_digest.clone(),
        status: StageStatus::Pending,
        stages: BTreeMap::new(),
    };
    for name in [
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
    ] {
        manifest.stages.insert(
            name.into(),
            Stage {
                status: StageStatus::Pending,
                reason: None,
                artifact: None,
                digest: None,
            },
        );
    }
    let mut journey = Journey {
        store,
        store_path,
        output,
        source,
        manifest,
        graph: None,
        review: None,
        sequence: 0,
        persistence_failed: false,
    };
    journey.record("acquisition", &admission.packet, true)?;
    journey.record("admission", &admission, true)?;
    match structure::repository_structure_reporter(
        &journey.store,
        &journey.manifest.packet_id,
        options.boundary_policy,
    ) {
        Ok(graph) => {
            let complete = graph.record.run.completion == Completion::Complete;
            journey.record("structure", &graph, complete)?;
            journey.graph = Some(graph);
        }
        Err(_) => journey.failed("structure", "structure_execution_failed")?,
    }
    match fitness::local_fitness_runner(
        &journey.store,
        &journey.manifest.packet_id,
        options.fitness_policy,
    ) {
        Ok(report) => {
            let passed = report.status == fitness::Status::Pass;
            journey.record("fitness", &report, passed)?;
        }
        Err(_) => journey.failed("fitness", "fitness_execution_failed")?,
    }
    Ok(journey)
}

impl Journey {
    pub fn manifest(&self) -> &JourneyManifest {
        &self.manifest
    }
    pub fn output(&self) -> &Path {
        &self.output
    }
    pub fn graph(&self) -> Option<&structure::StructureReport> {
        self.graph.as_ref()
    }

    fn pending(&self, name: &str) -> Result<()> {
        ensure!(
            !self.persistence_failed,
            "journey_manifest_persistence_failed"
        );
        ensure!(
            self.manifest.stages[name].status == StageStatus::Pending,
            "journey_stage_already_attempted"
        );
        ensure!(
            self.manifest.status != StageStatus::Failed,
            "journey_failed"
        );
        self.store.get(&self.manifest.packet_id)?;
        Ok(())
    }
    fn persist(&mut self) -> Result<()> {
        self.persistence_failed = true;
        self.manifest.status = if self
            .manifest
            .stages
            .values()
            .any(|s| s.status == StageStatus::Failed)
        {
            StageStatus::Failed
        } else if self
            .manifest
            .stages
            .values()
            .all(|s| s.status == StageStatus::Complete)
        {
            StageStatus::Complete
        } else {
            StageStatus::Pending
        };
        let path = self
            .output
            .join(format!("journey-{:04}.json", self.sequence));
        write_json_create_only(&path, &self.manifest)?;
        self.sequence += 1;
        self.persistence_failed = false;
        Ok(())
    }
    fn failed(&mut self, name: &str, reason: &str) -> Result<()> {
        let stage = self.manifest.stages.get_mut(name).unwrap();
        stage.status = StageStatus::Failed;
        stage.reason = Some(reason.into());
        self.persist()
    }
    fn record<T: Serialize>(&mut self, name: &str, value: &T, complete: bool) -> Result<()> {
        let artifact = format!("{name}.json");
        write_json_create_only(&self.output.join(&artifact), value)?;
        let stage = self.manifest.stages.get_mut(name).unwrap();
        stage.status = if complete {
            StageStatus::Complete
        } else {
            StageStatus::Failed
        };
        stage.reason = (!complete).then(|| "stage_incomplete_or_failed".into());
        stage.artifact = Some(artifact);
        stage.digest = Some(hash(value)?);
        self.persist()
    }
    pub fn analyze_impact(&mut self, changes: impact::ChangeSet) -> Result<()> {
        self.pending("impact")?;
        let graph = self
            .graph
            .clone()
            .ok_or_else(|| anyhow::anyhow!("journey_structure_missing"))?;
        match impact::change_impact_reporter(&self.store, graph, changes) {
            Ok(report) => self.record(
                "impact",
                &report,
                report.record.run.completion == Completion::Complete,
            ),
            Err(_) => self.failed("impact", "impact_execution_failed"),
        }
    }
    pub fn analyze_rationale(&mut self, selection: rationale::RationaleSelection) -> Result<()> {
        self.pending("rationale")?;
        let graph = self
            .graph
            .clone()
            .ok_or_else(|| anyhow::anyhow!("journey_structure_missing"))?;
        match rationale::architecture_rationale_reporter(&self.store, graph, selection) {
            Ok(report) => self.record(
                "rationale",
                &report,
                report.record.run.completion == Completion::Complete,
            ),
            Err(_) => self.failed("rationale", "rationale_execution_failed"),
        }
    }
    /// Explicit paid dispatch boundary. The caller must authorize this invocation separately.
    /// No retry is allowed on this handle, including after an uncertain provider result.
    pub fn run_review(
        &mut self,
        request: ProviderInvocationRequestV1,
        run_id: String,
        cancel_file: Option<PathBuf>,
    ) -> Result<()> {
        self.pending("review")?;
        ensure!(
            self.manifest.candidate_clean,
            "journey_dispatch_requires_clean_candidate"
        );
        let admission = self.store.get(&self.manifest.packet_id)?;
        // A durable reservation is deliberately non-successful. If dispatch or output
        // recording fails, this handle cannot issue another possibly paid request.
        self.failed("review", "review_dispatch_reserved_outcome_unknown")?;
        match runner::run(
            ReviewRunOptions {
                store: self.store_path.clone(),
                packet_id: self.manifest.packet_id.clone(),
                provider_request: request,
                out: self.output.join("review"),
                run_id,
                cancel_file,
            },
            admission,
        ) {
            Ok(review) => {
                self.record("review", &review, review.completion == Completion::Complete)?;
                self.review = Some(review);
                Ok(())
            }
            Err(_) => self.failed("review", "review_execution_failed_or_uncertain"),
        }
    }
    pub fn prepare_publication(
        &mut self,
        destination: &Path,
        format: PublicationFormat,
    ) -> Result<()> {
        let stage = format!("publication_{}", format.key());
        self.pending(&stage)?;
        ensure!(
            self.review
                .as_ref()
                .is_some_and(|r| r.completion == Completion::Complete),
            "journey_complete_review_required"
        );
        outside_source(destination, &self.source)?;
        ensure!(
            crate::codefriend::publication::read_review(
                &self.output.join("review/review-record.json")
            )? == self.review.as_ref().unwrap().review_record,
            "journey_review_artifact_changed"
        );
        match super::prepare_publication_bundle_for_format(
            &self.output.join("review/review-record.json"),
            &self.output.join(&stage),
            destination,
            format,
        ) {
            Ok(publication) => self.record(&stage, &publication, true),
            Err(_) => self.failed(&stage, "publication_preparation_failed"),
        }
    }

    pub fn analyze_drift(
        &mut self,
        baseline_root: &Path,
        baseline: structure::StructureReport,
    ) -> Result<()> {
        self.pending("drift")?;
        outside_source(baseline_root, &self.source)?;
        let result = (|| {
            let current = self
                .graph
                .clone()
                .ok_or_else(|| anyhow::anyhow!("journey_structure_missing"))?;
            let backend = AdmittedBaselines::open(&self.store, baseline_root, true)?;
            backend.retain(&baseline.record)?;
            backend.retain(&current.record)?;
            drift::architecture_drift_reporter(&self.store, &backend, baseline, current)
        })();
        match result {
            Ok(report) => self.record(
                "drift",
                &report,
                report.graph_comparison.comparable && report.structural_comparison.comparable,
            ),
            Err(_) => self.failed("drift", "drift_baseline_unavailable_or_incompatible"),
        }
    }

    /// Caller supplies authenticated Runtime identity and explicit compatible references.
    /// Retention/deletion and compatibility are rechecked through the native owners.
    pub fn compare_palace(
        &mut self,
        baseline_root: &Path,
        palace_root: &Path,
        authority: &adl_runtime_kernel::VerifiedMemoryPalaceAuthority,
        index: palace::IndexRequest,
        retrieve: palace::RetrieveRequest,
    ) -> Result<()> {
        self.pending("palace_comparison")?;
        outside_source(baseline_root, &self.source)?;
        outside_source(palace_root, &self.source)?;
        let result = (|| {
            let review = self
                .review
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
            let backend = AdmittedBaselines::open(&self.store, baseline_root, true)?;
            let current = backend.retain(&review.review_record)?;
            ensure!(
                retrieve.current == current && index.references.contains(&current),
                "journey_palace_current_mismatch"
            );
            palace::index(&backend, palace_root, authority, &index)?;
            palace::retrieve(&backend, palace_root, &retrieve)
        })();
        match result {
            Ok(report) => self.record("palace_comparison", &report, report.delta.comparable),
            Err(_) => self.failed("palace_comparison", "palace_comparison_failed"),
        }
    }

    /// Consume one existing, format-specific exact approval. No approval is created.
    pub fn export_approved(
        &mut self,
        approval_store: &Path,
        destination: &Path,
        format: PublicationFormat,
        font: Option<&Path>,
    ) -> Result<()> {
        use crate::codefriend::publication::{
            self, HtmlRenderOptions, MarkdownRenderOptions, PdfRenderOptions,
        };
        let key = format.key();
        self.pending(key)?;
        let bundle = format!("publication_{key}");
        ensure!(
            self.manifest.stages[&bundle].status == StageStatus::Complete,
            "journey_publication_missing"
        );
        let destination = outside_source(destination, &self.source)?;
        outside_source(approval_store, &self.source)?;
        let review_record = self.output.join("review/review-record.json");
        let publication = self.output.join(&bundle).join("publication.json");
        let bound_publication = publication::read_publication(&publication)?;
        let target = destination.join(&bound_publication.target);
        ensure!(
            self.manifest.stages[&bundle].digest.as_deref()
                == Some(hash(&bound_publication)?.as_str()),
            "journey_publication_artifact_changed"
        );
        ensure!(
            publication::read_review(&review_record)?
                == self
                    .review
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?
                    .review_record,
            "journey_review_artifact_changed"
        );
        let artifact_root = self.output.join(&bundle).join("artifacts");
        let synthesis = PathBuf::from("synthesis/synthesis.json");
        let remediation_plan = PathBuf::from("remediation/remediation-plan.json");
        let test_plan = PathBuf::from("tests/test-plan.json");
        // The native renderer revalidates current approval, destination, input hashes,
        // format and version. Each output owns a distinct approved target.
        let result: Result<(serde_json::Value, String)> = match format {
            PublicationFormat::Markdown => publication::render_markdown(MarkdownRenderOptions {
                review_record,
                publication,
                approval_store: approval_store.into(),
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root: destination.clone(),
                out: target,
            })
            .and_then(|r| Ok((serde_json::to_value(&r)?, r.approval_decision_digest))),
            PublicationFormat::Html => publication::render_html(HtmlRenderOptions {
                review_record,
                publication,
                approval_store: approval_store.into(),
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root: destination.clone(),
                out: target,
            })
            .and_then(|r| Ok((serde_json::to_value(&r)?, r.approval_decision_digest))),
            PublicationFormat::Pdf => {
                let font = font.ok_or_else(|| anyhow::anyhow!("journey_pdf_font_required"))?;
                publication::render_pdf(PdfRenderOptions {
                    review_record,
                    publication,
                    approval_store: approval_store.into(),
                    artifact_root,
                    synthesis,
                    remediation_plan,
                    test_plan,
                    destination_root: destination.clone(),
                    out: target,
                    font: font.into(),
                })
                .and_then(|r| Ok((serde_json::to_value(&r)?, r.approval_decision_digest)))
            }
        };
        match result {
            Ok((result, approval)) => {
                self.record(&format!("approval_{key}"), &approval, true)?;
                self.record(key, &result, true)
            }
            Err(_) => self.failed(key, "approval_or_renderer_failed"),
        }
    }
}
