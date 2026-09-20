//! Composition of existing owners. A prepared journey is not a completed Beta review.
//! Handles derive from acquisition or an authenticated owner attachment, never an asserted manifest.
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

pub(crate) mod owned_baseline;
mod publication_attachment;

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
    source: PathBoundary,
    review_root: PathBuf,
    deadline: Option<u64>,
    external_review: Option<ExternalReviewBinding>,
    manifest: JourneyManifest,
    graph: Option<structure::StructureReport>,
    review: Option<FourPerspectiveReviewRun>,
    sequence: usize,
    persistence_failed: bool,
    session_digest: String,
    checkpoint_digest: Option<String>,
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Reject aliases and parent traversal before any source-adjacent write.
fn safe_absolute(path: &Path) -> Result<PathBuf> {
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
    Ok(absolute)
}
// Compare open filesystem identities, not canonical path spelling: filesystem
// casing aliases can survive canonicalization. Missing leaves still have existing
// ancestors; permission and other lookup failures must not become non-overlap.
fn contains_identity(root: &Path, path: &Path) -> Result<bool> {
    let root = match same_file::Handle::from_path(root) {
        Ok(handle) => handle,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    for ancestor in path.ancestors() {
        match same_file::Handle::from_path(ancestor) {
            Ok(handle) if handle == root => return Ok(true),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(false)
}
fn paths_overlap(left: &Path, right: &Path) -> Result<bool> {
    Ok(left.starts_with(right)
        || right.starts_with(left)
        || contains_identity(left, right)?
        || contains_identity(right, left)?)
}
fn outside_source(path: &Path, source: &Path) -> Result<PathBuf> {
    let absolute = safe_absolute(path)?;
    ensure!(
        !paths_overlap(&absolute, source)?,
        "journey_output_overlaps_source"
    );
    Ok(absolute)
}

/// Acquisition and policy validation happen before a model executor is reachable.
/// Invalid input returns an error; execution failures after admission remain in the manifest.
pub fn prepare_local(options: LocalJourneyOptions) -> Result<Journey> {
    prepare_source(options, AcquisitionSource::Local)
}

pub fn prepare_source(
    mut options: LocalJourneyOptions,
    mut acquisition: AcquisitionSource,
) -> Result<Journey> {
    let source = fs::canonicalize(&options.checkout)?;
    let output = outside_source(&options.output, &source)?;
    let store_path = outside_source(&options.store, &source)?;
    ensure!(
        !paths_overlap(&output, &store_path)?,
        "journey_store_output_overlap"
    );
    ensure!(
        output.parent().is_some_and(Path::is_dir) && !output.exists(),
        "journey_output_unavailable"
    );
    options.checkout = source.clone();
    options.output = output.clone();
    options.store = store_path.clone();
    if let AcquisitionSource::Ci { receipt } = &mut acquisition {
        *receipt = std::path::absolute(&*receipt)?;
    }
    let session = SessionRecord {
        schema: "codefriend.journey_session.v1".into(),
        candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
        options: options.clone(),
        provenance_digest: provenance_digest(&options, &acquisition)?,
        acquisition,
    };
    let packet = acquire_source(&options, &session.acquisition)?;
    let checked = Admission::new(packet.clone(), options.retention.clone(), now())?;
    checked.validate()?;
    options.boundary_policy.validate(&checked)?;
    options.fitness_policy.validate()?;
    let store = Store::open(&store_path, now)?;
    let admission = store.admit(packet, options.retention)?;
    initialize(
        store,
        store_path,
        output.clone(),
        PathBoundary::Checkout(source),
        output.join("review"),
        None,
        None,
        &session,
        admission,
        options.boundary_policy,
        options.fitness_policy,
    )
}

#[allow(clippy::too_many_arguments)] // Explicit owner inputs preserve source and retention boundaries.
fn initialize<S: Serialize>(
    store: Store,
    store_path: PathBuf,
    output: PathBuf,
    source: PathBoundary,
    review_root: PathBuf,
    deadline: Option<u64>,
    external_review: Option<ExternalReviewBinding>,
    session: &S,
    admission: Admission,
    boundary_policy: structure::BoundaryPolicy,
    fitness_policy: fitness::Policy,
) -> Result<Journey> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new().mode(0o700).create(&output)?;
    }
    #[cfg(not(unix))]
    fs::create_dir(&output)?;
    write_json_create_only(&output.join("session.json"), session)?;
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
        review_root,
        deadline,
        external_review,
        manifest,
        graph: None,
        review: None,
        sequence: 0,
        persistence_failed: false,
        session_digest: hash(session)?,
        checkpoint_digest: None,
    };
    journey.record("acquisition", &admission.packet, true)?;
    journey.record("admission", &admission, true)?;
    match structure::repository_structure_reporter(
        &journey.store,
        &journey.manifest.packet_id,
        boundary_policy,
    ) {
        Ok(graph) => {
            let complete = graph.record.run.completion == Completion::Complete;
            journey.record("structure", &graph, complete)?;
            journey.graph = Some(graph);
        }
        Err(_) => journey.failed("structure", "structure_execution_failed")?,
    }
    match fitness::local_fitness_runner(&journey.store, &journey.manifest.packet_id, fitness_policy)
    {
        Ok(report) => {
            let passed = report.status == fitness::Status::Pass;
            journey.record("fitness", &report, passed)?;
        }
        Err(_) => journey.failed("fitness", "fitness_execution_failed")?,
    }
    Ok(journey)
}

impl Journey {
    fn live_admission(&self) -> Result<Admission> {
        ensure!(
            self.deadline.is_none_or(|d| now() < d),
            "journey_operation_expired"
        );
        let admission = self.store.get(&self.manifest.packet_id)?;
        ensure!(
            admission.digest == self.manifest.admission_digest,
            "journey_admission_changed"
        );
        if let Some(binding) = &self.external_review {
            ensure!(
                inventory(&binding.root)? == binding.files,
                "journey_external_review_changed"
            );
        }
        ensure!(
            self.deadline.is_none_or(|d| now() < d),
            "journey_operation_expired"
        );
        self.store.get(&self.manifest.packet_id)?;
        Ok(admission)
    }
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
        self.live_admission()?;
        Ok(())
    }
    fn persist(&mut self) -> Result<()> {
        ensure!(self.sequence < MAX_CHECKPOINTS, "journey_checkpoint_limit");
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
        let mut checkpoint = Checkpoint {
            schema: "codefriend.journey_checkpoint.v1".into(),
            sequence: self.sequence,
            previous: self.checkpoint_digest.clone(),
            session_digest: self.session_digest.clone(),
            files: inventory(&self.output)?,
            digest: String::new(),
        };
        checkpoint.digest = hash(&checkpoint)?;
        write_json_create_only(
            &self
                .output
                .join(format!("checkpoint-{:04}.json", self.sequence)),
            &checkpoint,
        )?;
        self.checkpoint_digest = Some(checkpoint.digest);
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
        // An owner operation may have crossed the retention deadline.
        self.live_admission()?;
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
            self.external_review.is_none(),
            "journey_owned_review_cannot_dispatch"
        );
        ensure!(
            self.manifest.candidate_clean,
            "journey_dispatch_requires_clean_candidate"
        );
        let admission = self.live_admission()?;
        // A durable reservation is deliberately non-successful. If dispatch or output
        // recording fails, this handle cannot issue another possibly paid request.
        self.failed("review", "review_dispatch_reserved_outcome_unknown")?;
        match runner::run(
            ReviewRunOptions {
                store: self.store_path.clone(),
                packet_id: self.manifest.packet_id.clone(),
                provider_request: request,
                out: self.review_root.clone(),
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
        self.source.check(destination)?;
        ensure!(
            crate::codefriend::publication::read_review(
                &self.review_root.join("review-record.json")
            )? == self.review.as_ref().unwrap().review_record,
            "journey_review_artifact_changed"
        );
        match super::prepare_publication_bundle_for_format(
            &self.review_root.join("review-record.json"),
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
        self.source.check(baseline_root)?;
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
        mut index: palace::IndexRequest,
        mut retrieve: palace::RetrieveRequest,
    ) -> Result<()> {
        self.pending("palace_comparison")?;
        self.source.check(baseline_root)?;
        self.source.check(palace_root)?;
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
            let observed = now().saturating_mul(1000);
            index.observed_epoch_ms = observed;
            retrieve.packet_observed_epoch_ms = observed;
            retrieve.observed_epoch_ms = observed;
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
        let destination = self.source.check(destination)?;
        self.source.check(approval_store)?;
        let review_record = self.review_root.join("review-record.json");
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
                write_json_create_only(
                    &self.output.join(format!("external-{key}.json")),
                    &ExternalOutput {
                        root: destination.join(&bound_publication.target),
                        approval_store: approval_store.into(),
                        decision_digest: approval.clone(),
                        files: inventory(&destination.join(&bound_publication.target))?,
                    },
                )?;
                self.record(&format!("approval_{key}"), &approval, true)?;
                self.record(key, &result, true)
            }
            Err(_) => self.failed(key, "approval_or_renderer_failed"),
        }
    }
}

// Digest-linked local integrity records are not authentication or attestation.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExternalOutput {
    root: PathBuf,
    approval_store: PathBuf,
    decision_digest: String,
    files: BTreeMap<String, String>,
}

const MAX_CHECKPOINTS: usize = 64;
const MAX_FILE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_INVENTORY_BYTES: u64 = 256 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AcquisitionSource {
    Local,
    /// The checkout option is the immutable bundle emitted by `ingest github`.
    Github,
    /// The checkout remains the exact local source; receipt comes from `ingest ci`.
    Ci {
        receipt: PathBuf,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionRecord {
    schema: String,
    candidate_revision: String,
    options: LocalJourneyOptions,
    acquisition: AcquisitionSource,
    provenance_digest: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Checkpoint {
    schema: String,
    sequence: usize,
    previous: Option<String>,
    session_digest: String,
    files: BTreeMap<String, String>,
    digest: String,
}
fn bounded_read(path: &Path) -> Result<Vec<u8>> {
    use std::io::Read;
    let m = fs::symlink_metadata(path)?;
    ensure!(
        m.is_file() && !m.file_type().is_symlink() && m.len() <= MAX_FILE_BYTES,
        "journey_artifact_bounds"
    );
    let mut data = Vec::new();
    fs::File::open(path)?
        .take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut data)?;
    ensure!(
        data.len() as u64 <= MAX_FILE_BYTES,
        "journey_artifact_bounds"
    );
    Ok(data)
}
fn read_typed<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&bounded_read(path)?)?)
}
fn inventory(root: &Path) -> Result<BTreeMap<String, String>> {
    fn visit(
        root: &Path,
        dir: &Path,
        files: &mut BTreeMap<String, String>,
        count: &mut usize,
        total: &mut u64,
    ) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            *count += 1;
            ensure!(*count <= 2048, "journey_inventory_count");
            let m = fs::symlink_metadata(entry.path())?;
            ensure!(!m.file_type().is_symlink(), "journey_symlink_rejected");
            if m.is_dir() {
                visit(root, &entry.path(), files, count, total)?;
            } else {
                ensure!(m.is_file(), "journey_special_file_rejected");
                *total = total
                    .checked_add(m.len())
                    .ok_or_else(|| anyhow::anyhow!("journey_inventory_bounds"))?;
                ensure!(*total <= MAX_INVENTORY_BYTES, "journey_inventory_bounds");
                let bytes = bounded_read(&entry.path())?;
                let name = entry
                    .path()
                    .strip_prefix(root)?
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("journey_path_encoding"))?
                    .replace('\\', "/");
                files.insert(name, crate::codefriend::ingestion::digest(&bytes));
            }
        }
        Ok(())
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result, &mut 0, &mut 0)?;
    Ok(result)
}
fn provenance_digest(options: &LocalJourneyOptions, source: &AcquisitionSource) -> Result<String> {
    match source {
        AcquisitionSource::Local => hash(source),
        AcquisitionSource::Github => hash(
            &crate::codefriend::ingestion::github::Acquisition::read(&options.checkout)?.provenance,
        ),
        AcquisitionSource::Ci { receipt } => hash(&read_typed::<
            crate::codefriend::ingestion::ci::Receipt,
        >(receipt)?),
    }
}
fn private_journey(root: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = fs::symlink_metadata(root)?;
    ensure!(
        metadata.is_dir()
            && !metadata.file_type().is_symlink()
            && metadata.permissions().mode() & 0o077 == 0,
        "journey_private_directory_required"
    );
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == "session.json"
            || name.starts_with("checkpoint-")
            || name.starts_with("journey-")
            || name.starts_with("intent-")
        {
            let m = fs::symlink_metadata(entry.path())?;
            ensure!(
                m.is_file() && !m.file_type().is_symlink() && m.permissions().mode() & 0o077 == 0,
                "journey_private_record_required"
            );
        }
    }
    Ok(())
}
fn acquire_source(
    options: &LocalJourneyOptions,
    source: &AcquisitionSource,
) -> Result<crate::codefriend::ingestion::Packet> {
    use crate::codefriend::ingestion::{ci, github};
    let packet = match source {
        AcquisitionSource::Local => local::acquire(
            &options.checkout,
            &options.repository,
            &options.revision,
            options.scope.clone(),
        )?,
        AcquisitionSource::Github => github::Acquisition::read(&options.checkout)?.packet,
        AcquisitionSource::Ci { receipt } => {
            let packet = local::acquire(
                &options.checkout,
                &options.repository,
                &options.revision,
                options.scope.clone(),
            )?;
            let receipt: ci::Receipt = read_typed(receipt)?;
            receipt.validate(&packet)?;
            ensure!(
                receipt.candidate_revision == env!("CODEFRIEND_BUILD_REVISION"),
                "journey_ci_candidate_changed"
            );
            packet
        }
    };
    packet.validate()?;
    ensure!(
        packet.repository == options.repository
            && packet.revision == options.revision
            && packet.scope == options.scope,
        "journey_source_binding_changed"
    );
    Ok(packet)
}
/// Explicit continuation instructions; provider credential values are never retained.
// Processed one continuation at a time; keep the public construction API without
// boxing Palace request fields solely to shrink the less frequent variants.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "stage", rename_all = "snake_case", deny_unknown_fields)]
pub enum Continuation {
    Status,
    Impact {
        changes: impact::ChangeSet,
    },
    Rationale {
        selection: rationale::RationaleSelection,
    },
    Drift {
        baseline_root: PathBuf,
        baseline: PathBuf,
    },
    Review {
        provider_request: PathBuf,
        run_id: String,
        cancel_file: Option<PathBuf>,
    },
    PreparePublication {
        destination: PathBuf,
        format: PublicationFormat,
    },
    Palace {
        baseline_root: PathBuf,
        palace_root: PathBuf,
        trust: PathBuf,
        authority: PathBuf,
        index: palace::IndexRequest,
        retrieve: palace::RetrieveRequest,
    },
    Export {
        approval_store: PathBuf,
        destination: PathBuf,
        format: PublicationFormat,
        font: Option<PathBuf>,
    },
}
impl Continuation {
    fn normalize_paths(&mut self) -> Result<()> {
        let absolute = |p: &mut PathBuf| -> Result<()> {
            *p = std::path::absolute(&*p)?;
            Ok(())
        };
        match self {
            Self::Status | Self::Impact { .. } | Self::Rationale { .. } => {}
            Self::Drift {
                baseline_root,
                baseline,
            } => {
                absolute(baseline_root)?;
                absolute(baseline)?;
            }
            Self::Review {
                provider_request,
                cancel_file,
                ..
            } => {
                absolute(provider_request)?;
                if let Some(p) = cancel_file {
                    absolute(p)?;
                }
            }
            Self::PreparePublication { destination, .. } => absolute(destination)?,
            Self::Palace {
                baseline_root,
                palace_root,
                trust,
                authority,
                ..
            } => {
                absolute(baseline_root)?;
                absolute(palace_root)?;
                absolute(trust)?;
                absolute(authority)?;
            }
            Self::Export {
                approval_store,
                destination,
                font,
                ..
            } => {
                absolute(approval_store)?;
                absolute(destination)?;
                if let Some(p) = font {
                    absolute(p)?;
                }
            }
        }
        Ok(())
    }
    fn key(&self) -> String {
        match self {
            Self::Status => "status".into(),
            Self::Impact { .. } => "impact".into(),
            Self::Rationale { .. } => "rationale".into(),
            Self::Drift { .. } => "drift".into(),
            Self::Review { .. } => "review".into(),
            Self::Palace { .. } => "palace_comparison".into(),
            Self::PreparePublication { format, .. } => format!("publication_{}", format.key()),
            Self::Export { format, .. } => format.key().into(),
        }
    }
}
/// Resume holds the admission Store lock throughout validation and continuation.
/// No legacy snapshot is promoted by synthesizing missing original options.
pub fn resume(output: &Path) -> Result<Journey> {
    resume_with_baseline(output, None)
}

pub(crate) fn resume_with_baseline(
    output: &Path,
    baseline: Option<&owned_baseline::OwnedBaseline<'_>>,
) -> Result<Journey> {
    private_journey(output)?;
    let session: PersistedSession = read_typed(&output.join("session.json"))?;
    let session_digest = hash(&session)?;
    let binding = session.resume_binding(output)?;
    let source = binding.source;
    let output = binding.output;
    let store_path = binding.store;
    let store = Store::open(&store_path, now)?;
    let all = inventory(&output)?;
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
            "journey_checkpoint_changed"
        );
        for (name, digest) in &cp.files {
            ensure!(all.get(name) == Some(digest), "journey_artifact_changed");
        }
        previous = Some(cp.digest.clone());
        last = Some(cp);
        count += 1;
    }
    let cp = last.ok_or_else(|| anyhow::anyhow!("journey_checkpoint_missing"))?;
    let mut expected = cp.files.clone();
    let latest = format!("checkpoint-{:04}.json", count - 1);
    expected.insert(latest.clone(), all[&latest].clone());
    ensure!(all == expected, "journey_uncheckpointed_or_gapped_state");
    let manifest: JourneyManifest =
        read_typed(&output.join(format!("journey-{:04}.json", count - 1)))?;
    ensure!(
        manifest.candidate_revision == binding.candidate_revision
            && manifest.candidate_clean == (env!("CODEFRIEND_BUILD_CLEAN") == "true"),
        "journey_candidate_changed"
    );
    let admission = store.get(&manifest.packet_id)?;
    admission.validate()?;
    ensure!(
        admission.digest == manifest.admission_digest
            && admission.packet.repository == manifest.repository
            && admission.packet.revision == manifest.revision
            && admission.packet.scope_digest == manifest.scope_digest,
        "journey_admission_or_source_changed"
    );
    session.validate_admission(&admission)?;
    binding.boundary_policy.validate(&admission)?;
    binding.fitness_policy.validate()?;
    let expected_stages: std::collections::BTreeSet<_> = [
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
    ]
    .into_iter()
    .collect();
    ensure!(
        manifest
            .stages
            .keys()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
            == expected_stages,
        "journey_stage_set_changed"
    );
    let computed = if manifest
        .stages
        .values()
        .any(|s| s.status == StageStatus::Failed)
    {
        StageStatus::Failed
    } else if manifest
        .stages
        .values()
        .all(|s| s.status == StageStatus::Complete)
    {
        StageStatus::Complete
    } else {
        StageStatus::Pending
    };
    ensure!(
        computed == manifest.status && manifest.stages.len() == 18,
        "journey_status_changed"
    );
    ensure!(
        binding.external_review.is_none()
            || manifest.stages["review"].status == StageStatus::Complete,
        "journey_owned_attachment_incomplete"
    );
    let mut graph = None;
    let mut review = None;
    for (name, stage) in &manifest.stages {
        if let Some(artifact) = &stage.artifact {
            ensure!(
                *artifact == format!("{name}.json"),
                "journey_artifact_path_changed"
            );
            // The byte inventory binds every typed output to its recorded checkpoint.
            bounded_read(&output.join(artifact))?;
        } else {
            ensure!(
                stage.status != StageStatus::Complete && stage.digest.is_none(),
                "journey_missing_stage_artifact"
            );
        }
        if output.join(format!("intent-{name}.json")).exists()
            && stage.status == StageStatus::Pending
        {
            anyhow::bail!("journey_unresolved_step_reservation");
        }
    }
    if manifest.stages["structure"].status == StageStatus::Complete {
        let value: structure::StructureReport = read_typed(&output.join("structure.json"))?;
        value.validate(&store)?;
        ensure!(
            value.record.admission == admission
                && hash(&value.policy)? == hash(&binding.boundary_policy)?
                && manifest.stages["structure"].digest.as_deref() == Some(hash(&value)?.as_str()),
            "journey_structure_changed"
        );
        graph = Some(value);
    }
    if manifest.stages["review"].status == StageStatus::Complete {
        let value: FourPerspectiveReviewRun = read_typed(&output.join("review.json"))?;
        value.review_record.validate()?;
        ensure!(
            value.completion == Completion::Complete
                && value.review_record.admission == admission
                && manifest.stages["review"].digest.as_deref() == Some(hash(&value)?.as_str())
                && crate::codefriend::publication::read_review(
                    &binding.review_root.join("review-record.json")
                )? == value.review_record,
            "journey_review_changed"
        );
        if let Some(external) = &binding.external_review {
            ensure!(
                hash(&value)? == external.run_digest,
                "journey_owned_review_changed"
            );
        }
        review = Some(value);
    }
    if manifest.stages["fitness"].status == StageStatus::Complete {
        let value: fitness::Report = read_typed(&output.join("fitness.json"))?;
        value.validate(&store)?;
        ensure!(
            hash(&value.policy)? == hash(&binding.fitness_policy)?,
            "journey_fitness_policy_changed"
        );
    }
    if manifest.stages["impact"].status == StageStatus::Complete {
        let value: impact::ImpactReport = read_typed(&output.join("impact.json"))?;
        value.validate(&store)?;
    }
    if manifest.stages["rationale"].status == StageStatus::Complete {
        let value: rationale::RationaleReport = read_typed(&output.join("rationale.json"))?;
        value.validate(&store)?;
    }
    let owned_drift =
        owned_baseline::validate_saved(&source, &output, &store, binding.deadline, baseline)?;
    if manifest.stages["drift"].status == StageStatus::Complete && !owned_drift {
        let step: Continuation = read_typed(&output.join("intent-drift.json"))?;
        let Continuation::Drift { baseline_root, .. } = step else {
            anyhow::bail!("journey_drift_intent_missing")
        };
        source.check(&baseline_root)?;
        let backend = AdmittedBaselines::open(&store, &baseline_root, false)?;
        let value: drift::DriftReport = read_typed(&output.join("drift.json"))?;
        value.validate(&store, &backend)?;
    }
    if manifest.stages["palace_comparison"].status == StageStatus::Complete {
        let step: Continuation = read_typed(&output.join("intent-palace_comparison.json"))?;
        let Continuation::Palace {
            baseline_root,
            palace_root,
            trust,
            authority,
            retrieve,
            ..
        } = step
        else {
            anyhow::bail!("journey_palace_intent_missing")
        };
        source.check(&baseline_root)?;
        source.check(&palace_root)?;
        let _authority =
            crate::codefriend::memory::palace_authority::provision(&trust, &authority)?;
        let retained: palace::RetrievedComparison =
            read_typed(&output.join("palace_comparison.json"))?;
        let mut current_request = retrieve;
        // Re-evaluate freshness against the clock, while keeping the packet's
        // original observation pinned to the retained owner result.
        current_request.packet_observed_epoch_ms = retained.observed_epoch_ms;
        current_request.observed_epoch_ms = now().saturating_mul(1000);
        let backend = AdmittedBaselines::open(&store, &baseline_root, false)?;
        let current = palace::retrieve(&backend, &palace_root, &current_request)?;
        ensure!(
            current.schema == retained.schema
                && current.provenance == retained.provenance
                && current.selected_references == retained.selected_references
                && current.delta == retained.delta,
            "journey_palace_changed"
        );
    }
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        if publication_attachment::validate(&source, &output, &manifest, review.as_ref(), format)? {
            continue;
        }
        let key = format.key();
        if manifest.stages[key].status == StageStatus::Complete {
            let external: ExternalOutput =
                read_typed(&output.join(format!("external-{key}.json")))?;
            source.check(&external.root)?;
            source.check(&external.approval_store)?;
            ensure!(
                inventory(&external.root)? == external.files,
                "journey_export_changed"
            );
            let publication = crate::codefriend::publication::read_publication(
                &output.join(format!("publication_{key}/publication.json")),
            )?;
            let review = review
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("journey_review_missing"))?;
            let head = crate::codefriend::publication::read_decision_head(
                &external.approval_store,
                &review.review_record,
                &publication,
            )?;
            ensure!(
                head.is_some_and(|h| h.digest == external.decision_digest
                    && h.decision == crate::codefriend::publication::DecisionKind::Approved),
                "journey_approval_changed"
            );
        }
    }
    Ok(Journey {
        store,
        store_path,
        output,
        source,
        review_root: binding.review_root,
        deadline: binding.deadline,
        external_review: binding.external_review,
        manifest,
        graph,
        review,
        sequence: count,
        persistence_failed: false,
        session_digest,
        checkpoint_digest: previous,
    })
}
impl Journey {
    pub fn continue_with(&mut self, mut step: Continuation) -> Result<()> {
        step.normalize_paths()?;
        if matches!(step, Continuation::Status) {
            self.live_admission()?;
            return Ok(());
        }
        let key = step.key();
        self.pending(&key)?;
        // Create-only reservation survives a crash before any potentially costly effect.
        write_json_create_only(&self.output.join(format!("intent-{key}.json")), &step)?;
        self.persist()?;
        let result = (|| match step {
            Continuation::Status => unreachable!(),
            Continuation::Impact { changes } => self.analyze_impact(changes),
            Continuation::Rationale { selection } => self.analyze_rationale(selection),
            Continuation::Drift {
                baseline_root,
                baseline,
            } => self.analyze_drift(&baseline_root, read_typed(&baseline)?),
            Continuation::Review {
                provider_request,
                run_id,
                cancel_file,
            } => self.run_review(
                runner::read_provider_request(&provider_request)?,
                run_id,
                cancel_file,
            ),
            Continuation::PreparePublication {
                destination,
                format,
            } => self.prepare_publication(&destination, format),
            Continuation::Palace {
                baseline_root,
                palace_root,
                trust,
                authority,
                index,
                retrieve,
            } => {
                let authority =
                    crate::codefriend::memory::palace_authority::provision(&trust, &authority)?;
                self.compare_palace(&baseline_root, &palace_root, &authority, index, retrieve)
            }
            Continuation::Export {
                approval_store,
                destination,
                format,
                font,
            } => self.export_approved(&approval_store, &destination, format, font.as_deref()),
        })();
        if result.is_err()
            && !self.persistence_failed
            && self.manifest.stages[&key].status == StageStatus::Pending
        {
            self.failed(&key, "continuation_failed_or_uncertain")?;
        }
        result
    }
}

// Server-only attachment of an existing admission; callers supply owner-resolved paths.
pub(crate) struct OwnedAdmissionJourneyOptions {
    pub store: PathBuf,
    pub output: PathBuf,
    pub owner_root: PathBuf,
    pub review_root: PathBuf,
    pub packet_id: String,
    pub admission_digest: String,
    pub operation_id: String,
    pub candidate_revision: String,
    pub expires_at: u64,
    pub completed_run: FourPerspectiveReviewRun,
    pub boundary_policy: structure::BoundaryPolicy,
    pub fitness_policy: fitness::Policy,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExternalReviewBinding {
    root: PathBuf,
    files: BTreeMap<String, String>,
    run_digest: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OwnedSession {
    schema: String,
    candidate_revision: String,
    owner_root: PathBuf,
    store: PathBuf,
    output: PathBuf,
    packet_id: String,
    admission_digest: String,
    operation_id: String,
    expires_at: u64,
    external_review: ExternalReviewBinding,
    boundary_policy: structure::BoundaryPolicy,
    fitness_policy: fitness::Policy,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum PersistedSession {
    Checkout(SessionRecord),
    Owned(OwnedSession),
}
enum PathBoundary {
    Checkout(PathBuf),
    Owned {
        root: PathBuf,
        store: PathBuf,
        review: PathBuf,
    },
}
impl PathBoundary {
    fn check(&self, path: &Path) -> Result<PathBuf> {
        match self {
            Self::Checkout(source) => outside_source(path, source),
            Self::Owned {
                root,
                store,
                review,
            } => {
                let path = safe_absolute(path)?;
                ensure!(
                    contains_identity(root, &path)? && !contains_identity(&path, root)?,
                    "journey_owned_path_escape"
                );
                ensure!(
                    !paths_overlap(&path, store)? && !paths_overlap(&path, review)?,
                    "journey_owned_path_overlap"
                );
                Ok(path)
            }
        }
    }
}
struct ResumeBinding {
    source: PathBoundary,
    output: PathBuf,
    store: PathBuf,
    review_root: PathBuf,
    candidate_revision: String,
    boundary_policy: structure::BoundaryPolicy,
    fitness_policy: fitness::Policy,
    deadline: Option<u64>,
    external_review: Option<ExternalReviewBinding>,
}
impl PersistedSession {
    fn resume_binding(&self, output: &Path) -> Result<ResumeBinding> {
        match self {
            Self::Checkout(record) => {
                ensure!(
                    record.schema == "codefriend.journey_session.v1"
                        && record.candidate_revision == env!("CODEFRIEND_BUILD_REVISION"),
                    "journey_candidate_changed"
                );
                let source = fs::canonicalize(&record.options.checkout)?;
                let output = outside_source(output, &source)?;
                ensure!(
                    output == record.options.output,
                    "journey_output_binding_changed"
                );
                Ok(ResumeBinding {
                    store: outside_source(&record.options.store, &source)?,
                    review_root: output.join("review"),
                    output,
                    source: PathBoundary::Checkout(source),
                    candidate_revision: record.candidate_revision.clone(),
                    boundary_policy: record.options.boundary_policy.clone(),
                    fitness_policy: record.options.fitness_policy.clone(),
                    deadline: None,
                    external_review: None,
                })
            }
            Self::Owned(record) => {
                ensure!(
                    record.schema == "codefriend.journey_owned_session.v1"
                        && record.candidate_revision == env!("CODEFRIEND_BUILD_REVISION"),
                    "journey_candidate_changed"
                );
                ensure!(now() < record.expires_at, "journey_operation_expired");
                private_journey(&record.owner_root)?;
                let root = safe_absolute(&record.owner_root)?;
                let store = safe_absolute(&record.store)?;
                let review = safe_absolute(&record.external_review.root)?;
                ensure!(
                    store.starts_with(&root)
                        && review.starts_with(&root)
                        && store != root
                        && review != root
                        && !store.starts_with(&review)
                        && !review.starts_with(&store),
                    "journey_owned_path_overlap"
                );
                let source = PathBoundary::Owned {
                    root,
                    store: store.clone(),
                    review: review.clone(),
                };
                let output = source.check(output)?;
                ensure!(output == record.output, "journey_output_binding_changed");
                ensure!(
                    inventory(&review)? == record.external_review.files,
                    "journey_external_review_changed"
                );
                Ok(ResumeBinding {
                    source,
                    output,
                    store,
                    review_root: review,
                    candidate_revision: record.candidate_revision.clone(),
                    boundary_policy: record.boundary_policy.clone(),
                    fitness_policy: record.fitness_policy.clone(),
                    deadline: Some(record.expires_at),
                    external_review: Some(record.external_review.clone()),
                })
            }
        }
    }
    fn validate_admission(&self, admission: &Admission) -> Result<()> {
        match self {
            Self::Checkout(record) => {
                ensure!(
                    admission.packet == acquire_source(&record.options, &record.acquisition)?,
                    "journey_admission_or_source_changed"
                );
                ensure!(
                    record.provenance_digest
                        == provenance_digest(&record.options, &record.acquisition)?,
                    "journey_acquisition_provenance_changed"
                );
                ensure!(
                    hash(&admission.retention)? == hash(&record.options.retention)?,
                    "journey_retention_changed"
                );
            }
            Self::Owned(record) => {
                ensure!(
                    admission.packet.packet_id == record.packet_id
                        && admission.digest == record.admission_digest
                        && record.expires_at <= admission.expires_at
                        && now() < record.expires_at,
                    "journey_owned_admission_changed"
                );
                let run: FourPerspectiveReviewRun =
                    read_typed(&record.external_review.root.join("run.json"))?;
                ensure!(
                    run.run_id == record.operation_id
                        && hash(&run)? == record.external_review.run_digest,
                    "journey_owned_run_changed"
                );
                let review = crate::codefriend::publication::read_review(
                    &record.external_review.root.join("review-record.json"),
                )?;
                ensure!(
                    review.admission == *admission,
                    "journey_owned_review_changed"
                );
            }
        }
        Ok(())
    }
}
pub(crate) fn prepare_owned_admission(options: OwnedAdmissionJourneyOptions) -> Result<Journey> {
    ensure!(
        options.candidate_revision == env!("CODEFRIEND_BUILD_REVISION"),
        "journey_candidate_changed"
    );
    ensure!(now() < options.expires_at, "journey_operation_expired");
    let store_path = safe_absolute(&options.store)?;
    let store = Store::open(&store_path, now)?;
    let admission = store.get(&options.packet_id)?;
    ensure!(
        admission.digest == options.admission_digest,
        "journey_admission_changed"
    );
    let run = options.completed_run;
    run.review_record.validate()?;
    ensure!(
        run.schema == runner::REVIEW_RUN_SCHEMA
            && run.completion == Completion::Complete
            && run.run_id == options.operation_id
            && run.review_record.admission == admission,
        "journey_owned_review_changed"
    );
    let review_root = safe_absolute(&options.review_root)?;
    let retained_run: FourPerspectiveReviewRun = read_typed(&review_root.join("run.json"))?;
    ensure!(retained_run == run, "journey_owned_run_changed");
    ensure!(
        crate::codefriend::publication::read_review(&review_root.join("review-record.json"))?
            == run.review_record,
        "journey_owned_review_changed"
    );
    options.boundary_policy.validate(&admission)?;
    options.fitness_policy.validate()?;
    let session = OwnedSession {
        schema: "codefriend.journey_owned_session.v1".into(),
        candidate_revision: options.candidate_revision,
        owner_root: safe_absolute(&options.owner_root)?,
        store: store_path.clone(),
        output: safe_absolute(&options.output)?,
        packet_id: options.packet_id,
        admission_digest: admission.digest.clone(),
        operation_id: options.operation_id,
        expires_at: options.expires_at.min(admission.expires_at),
        external_review: ExternalReviewBinding {
            root: review_root,
            files: inventory(&options.review_root)?,
            run_digest: hash(&run)?,
        },
        boundary_policy: options.boundary_policy,
        fitness_policy: options.fitness_policy,
    };
    let persisted = PersistedSession::Owned(session.clone());
    let binding = persisted.resume_binding(&session.output)?;
    persisted.validate_admission(&admission)?;
    ensure!(
        session.output.parent().is_some_and(Path::is_dir) && !session.output.exists(),
        "journey_output_unavailable"
    );
    let mut journey = initialize(
        store,
        store_path,
        session.output.clone(),
        binding.source,
        binding.review_root,
        binding.deadline,
        binding.external_review,
        &persisted,
        admission,
        session.boundary_policy,
        session.fitness_policy,
    )?;
    journey.record("review", &run, true)?;
    journey.review = Some(run);
    Ok(journey)
}

#[cfg(test)]
#[path = "../../../tests/support/codefriend_owned_journey.rs"]
mod owned_tests;
