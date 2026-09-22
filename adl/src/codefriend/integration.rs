//! Native artifact challenges for the website integration boundary.
//!
//! A challenge is inspectable binding data, never authentication or permission to
//! append a decision. The eventual authenticated service must retain it and check
//! its current decision head under the approval owner's append lock.
pub mod journey;
use crate::codefriend::{
    evidence::{
        contracts::{Publication, ReviewRecord},
        hash, valid_digest,
    },
    publication::verify_artifacts,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Owner-retained snapshot. Deserialization provides data, never authority;
/// service callers must load it only from their own operation store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationChallenge {
    schema: String,
    subject: String,
    operation: String,
    candidate_revision: String,
    publication: Publication,
    expected_head: Option<String>,
    issued_at: u64,
    expires_at: u64,
    digest: String,
}

impl PublicationChallenge {
    /// Prepare from owner-resolved subject, operation, review, artifacts and head.
    /// These arguments are NOT a substitute for service authentication.
    // Keep independently resolved identity, artifact and lifetime inputs explicit.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        subject: &str,
        operation: &str,
        candidate_revision: &str,
        review: &ReviewRecord,
        publication: &Publication,
        artifact_root: &Path,
        expected_head: Option<&str>,
        issued_at: u64,
        expires_at: u64,
    ) -> Result<Self> {
        ensure!(
            candidate_revision.len() == 40
                && candidate_revision.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid_challenge_candidate"
        );
        ensure!(
            identifier(subject) && identifier(operation),
            "invalid_challenge_identity"
        );
        ensure!(
            issued_at > 0 && expires_at > issued_at && expires_at - issued_at <= 300,
            "invalid_challenge_lifetime"
        );
        ensure!(
            expected_head.is_none_or(valid_digest),
            "invalid_challenge_head"
        );
        publication.validate(review)?;
        ensure!(
            review.successful_execution().is_ok(),
            "challenge_requires_complete_run"
        );
        verify_artifacts(artifact_root, &publication.artifact_manifest)?;
        let mut challenge = Self {
            schema: "codefriend.publication_challenge.v1".into(),
            subject: subject.into(),
            operation: operation.into(),
            candidate_revision: candidate_revision.into(),
            publication: publication.clone(),
            expected_head: expected_head.map(str::to_owned),
            issued_at,
            expires_at,
            digest: String::new(),
        };
        challenge.digest = hash(&challenge)?;
        Ok(challenge)
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
    pub fn publication(&self) -> &Publication {
        &self.publication
    }

    /// Recheck a response against the retained owner snapshot. Success is only a
    /// binding check; it does not mint an authenticated capability or decision.
    // Separate caller observations from the retained snapshot at this trust boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn verify_response(
        &self,
        subject: &str,
        operation: &str,
        candidate_revision: &str,
        challenge_digest: &str,
        binding_digest: &str,
        current_head: Option<&str>,
        review: &ReviewRecord,
        current_publication: &Publication,
        artifact_root: &Path,
        now: u64,
    ) -> Result<()> {
        let mut expected = self.clone();
        expected.digest.clear();
        ensure!(
            self.schema == "codefriend.publication_challenge.v1" && self.digest == hash(&expected)?,
            "challenge_integrity_mismatch"
        );
        ensure!(
            subject == self.subject
                && operation == self.operation
                && candidate_revision == self.candidate_revision,
            "challenge_identity_mismatch"
        );
        ensure!(
            now >= self.issued_at && now < self.expires_at,
            "challenge_expired_or_clock_reversed"
        );
        ensure!(challenge_digest == self.digest, "challenge_digest_mismatch");
        ensure!(
            current_head == self.expected_head.as_deref(),
            "challenge_head_changed"
        );
        current_publication.validate(review)?;
        ensure!(
            review.successful_execution().is_ok(),
            "challenge_requires_complete_run"
        );
        let expected = self.publication.binding_digest()?;
        ensure!(
            binding_digest == expected && current_publication.binding_digest()? == expected,
            "challenge_binding_changed"
        );
        verify_artifacts(artifact_root, &current_publication.artifact_manifest)
    }
}
fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationFormat {
    #[default]
    Markdown,
    Html,
    Pdf,
}
impl PublicationFormat {
    pub fn key(self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Html => "html",
            Self::Pdf => "pdf",
        }
    }
    pub fn renderer_version(self) -> &'static str {
        match self {
            Self::Markdown => crate::codefriend::publication::MARKDOWN_RENDERER_VERSION,
            Self::Html => crate::codefriend::publication::HTML_RENDERER_VERSION,
            Self::Pdf => crate::codefriend::publication::PDF_RENDERER_VERSION,
        }
    }
    fn target(self) -> &'static str {
        match self {
            Self::Markdown => "report-md",
            Self::Html => "report-html",
            Self::Pdf => "report-pdf",
        }
    }
}

/// Assemble renderer inputs from a retained review using the existing component
/// owners. The caller supplies an owner-controlled, create-only output directory;
/// no browser-selected path belongs at this boundary. A failure leaves no final
/// publication manifest, and the partial directory must not be treated as ready.
pub fn prepare_publication_bundle(
    review_path: &Path,
    output: &Path,
    destination: &Path,
) -> Result<Publication> {
    prepare_publication_bundle_for_format(
        review_path,
        output,
        destination,
        PublicationFormat::Markdown,
    )
}

/// Atomically commit only a complete format-specific bundle. Interrupted stages
/// remain separate from the final path and are removed by operation retention.
pub fn prepare_publication_bundle_for_format(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
) -> Result<Publication> {
    prepare_publication_bundle_for_options(
        review_path,
        output,
        destination,
        format,
        crate::codefriend::actions::test_plan::TEST_PLAN_SCHEMA,
        None,
    )
}

/// Current live owners use admitted evidence mappings. The caller retains its
/// original Store authorization before and after this deterministic preparation.
pub fn prepare_publication_bundle_for_format_v2(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
) -> Result<Publication> {
    prepare_publication_bundle_for_options(
        review_path,
        output,
        destination,
        format,
        crate::codefriend::actions::test_plan::TEST_PLAN_SCHEMA_V2,
        None,
    )
}

/// Prepare a bundle with a validated architecture package and explicit planner generation.
pub fn prepare_publication_bundle_with_architecture(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
    architecture: Option<&std::collections::BTreeMap<String, Vec<u8>>>,
) -> Result<Publication> {
    prepare_publication_bundle_for_options(
        review_path,
        output,
        destination,
        format,
        crate::codefriend::actions::test_plan::TEST_PLAN_SCHEMA,
        architecture,
    )
}

/// Prepare a bundle with a validated architecture package and explicit planner generation.
pub fn prepare_publication_bundle_with_architecture_v2(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
    architecture: Option<&std::collections::BTreeMap<String, Vec<u8>>>,
) -> Result<Publication> {
    prepare_publication_bundle_for_options(
        review_path,
        output,
        destination,
        format,
        crate::codefriend::actions::test_plan::TEST_PLAN_SCHEMA_V2,
        architecture,
    )
}

fn prepare_publication_bundle_for_options(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
    test_plan_schema: &str,
    architecture: Option<&std::collections::BTreeMap<String, Vec<u8>>>,
) -> Result<Publication> {
    use std::{
        fs::{self, File},
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    ensure!(!output.exists(), "publication_bundle_already_exists");
    crate::codefriend::publication::manifest::reject_symlink_components(output)?;
    let parent = output
        .parent()
        .ok_or_else(|| anyhow::anyhow!("publication_bundle_parent_missing"))?;
    let parent_file = File::open(parent)?;
    let stage = loop {
        let path = parent.join(format!(
            ".publication-stage-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match fs::create_dir(&path) {
            Ok(()) => break path,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    };
    let publication = build_publication_bundle(
        review_path,
        &stage,
        destination,
        format,
        test_plan_schema,
        architecture,
    )?;
    for artifact in &publication.artifact_manifest {
        File::open(stage.join("artifacts").join(&artifact.path))?.sync_all()?;
    }
    if architecture.is_some() {
        File::open(stage.join("artifacts/architecture"))?.sync_all()?;
    }
    for directory in [
        "artifacts/synthesis",
        "artifacts/remediation",
        "artifacts/tests",
        "artifacts",
        "",
    ] {
        File::open(stage.join(directory))?.sync_all()?;
    }
    crate::codefriend::publication::markdown::rename_create_only_at(
        &parent_file,
        stage
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("publication_stage_name_missing"))?,
        output
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("publication_output_name_missing"))?,
    )?;
    parent_file.sync_all()?;
    Ok(publication)
}

fn build_publication_bundle(
    review_path: &Path,
    output: &Path,
    destination: &Path,
    format: PublicationFormat,
    test_plan_schema: &str,
    architecture: Option<&std::collections::BTreeMap<String, Vec<u8>>>,
) -> Result<Publication> {
    use crate::codefriend::publication::{manifest::read_review_snapshot, ManifestInput};
    use std::{collections::BTreeMap, fs, io::Write};
    let (review, snapshot) = read_review_snapshot(review_path)?;
    ensure!(
        review.successful_execution().is_ok(),
        "publication_bundle_requires_complete_run"
    );
    let mut generated = publication_artifacts(&review, &snapshot, test_plan_schema)?;
    if let Some(files) = architecture {
        for (name, bytes) in files {
            ensure!(
                !name.contains('/') && !name.starts_with('.'),
                "architecture_artifact_name"
            );
            generated
                .files
                .insert(format!("architecture/{name}"), bytes.clone());
        }
        crate::codefriend::publication::architecture::validate_files(
            files,
            &review,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        )?;
    }
    let artifact_root = output.join("artifacts");
    fs::create_dir(&artifact_root)?;
    for name in ["synthesis", "remediation", "tests"] {
        fs::create_dir(artifact_root.join(name))?;
    }
    if architecture.is_some() {
        fs::create_dir(artifact_root.join("architecture"))?;
    }
    for (name, bytes) in &generated.files {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(artifact_root.join(name))?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    let artifacts = generated.inventory();
    let manifest = ManifestInput {
        schema: "codefriend.publication_manifest_input.v1".into(),
        artifact_manifest: artifacts,
        renderer_versions: BTreeMap::from([(
            format.key().into(),
            format.renderer_version().into(),
        )]),
        target: format.target().into(),
        claims: vec!["Bounded CodeFriend review and action plans".into()],
        nonclaims: vec!["No source changes or external publication".into()],
    };
    let publication = manifest.publication(&review, destination)?;
    verify_artifacts(&artifact_root, &publication.artifact_manifest)?;
    crate::codefriend::publication::write_json_create_only(
        &output.join("publication.json"),
        &publication,
    )?;
    Ok(publication)
}

pub(crate) struct PublicationArtifacts {
    pub files: std::collections::BTreeMap<String, Vec<u8>>,
}
impl PublicationArtifacts {
    pub(crate) fn inventory(&self) -> Vec<crate::codefriend::evidence::contracts::Artifact> {
        self.files
            .iter()
            .map(
                |(path, bytes)| crate::codefriend::evidence::contracts::Artifact {
                    path: path.clone(),
                    digest: crate::codefriend::ingestion::digest(bytes),
                },
            )
            .collect()
    }
}
/// Preserve the original snapshot bytes exactly; file owners intentionally retain
/// them in synthesis/tests, while remediation writes a typed pretty snapshot.
pub(crate) fn publication_artifacts(
    review: &crate::codefriend::evidence::contracts::ReviewRecord,
    review_bytes: &[u8],
    test_plan_schema: &str,
) -> anyhow::Result<PublicationArtifacts> {
    use crate::codefriend::{
        actions::{
            remediation::{self, RemediationManifest, REMEDIATION_MANIFEST_SCHEMA},
            test_plan::{self, TestPlanManifest},
        },
        evidence::hash,
        review::synthesis::{self, SynthesisManifest, SYNTHESIS_MANIFEST_SCHEMA},
    };
    let parsed: crate::codefriend::evidence::contracts::ReviewRecord =
        serde_json::from_slice(review_bytes)?;
    anyhow::ensure!(&parsed == review, "publication_snapshot_mismatch");
    review.validate()?;
    let synthesis = synthesis::synthesize(review)?;
    let remediation = remediation::plan(&synthesis, review)?;
    let tests = test_plan::derive_for_schema(test_plan_schema, &synthesis, review)?;
    let sm = SynthesisManifest {
        schema: SYNTHESIS_MANIFEST_SCHEMA.into(),
        synthesis_ref: "synthesis.json".into(),
        synthesis_digest: hash(&synthesis)?,
        review_record_ref: "review-record.json".into(),
        review_record_digest: hash(review)?,
        synthesized_finding_count: synthesis.synthesized_findings.len(),
        input_finding_count: synthesis.input_finding_count,
    };
    let rm = RemediationManifest {
        schema: REMEDIATION_MANIFEST_SCHEMA.into(),
        synthesis_ref: "synthesis.json".into(),
        synthesis_digest: remediation.synthesis_digest.clone(),
        synthesis_manifest_ref: "synthesis-manifest.json".into(),
        synthesis_manifest_digest: hash(&sm)?,
        review_record_ref: "review-record.json".into(),
        review_record_digest: hash(review)?,
        remediation_plan_ref: "remediation-plan.json".into(),
        remediation_plan_digest: hash(&remediation)?,
        action_count: remediation.actions.len(),
        omitted_finding_count: remediation.omitted_findings.len(),
    };
    let tm = TestPlanManifest {
        schema: test_plan::manifest_schema(&tests)?.into(),
        synthesis_manifest_ref: "synthesis-manifest.json".into(),
        synthesis_manifest_digest: hash(&sm)?,
        synthesis_ref: "synthesis.json".into(),
        synthesis_digest: tests.synthesis_digest.clone(),
        review_record_ref: "review-record.json".into(),
        review_record_digest: hash(review)?,
        test_plan_ref: "test-plan.json".into(),
        test_plan_digest: hash(&tests)?,
        test_case_count: tests.test_cases.len(),
        omitted_finding_count: tests.omitted_findings.len(),
    };
    fn bytes<T: serde::Serialize>(v: &T) -> anyhow::Result<Vec<u8>> {
        let mut b = serde_json::to_vec_pretty(v)?;
        b.push(b'\n');
        Ok(b)
    }
    let synthesis_bytes = bytes(&synthesis)?;
    let sm_bytes = bytes(&sm)?;
    Ok(PublicationArtifacts {
        files: std::collections::BTreeMap::from([
            ("synthesis/review-record.json".into(), review_bytes.to_vec()),
            ("synthesis/synthesis.json".into(), synthesis_bytes.clone()),
            ("synthesis/manifest.json".into(), sm_bytes.clone()),
            ("remediation/review-record.json".into(), bytes(review)?),
            ("remediation/synthesis.json".into(), synthesis_bytes.clone()),
            (
                "remediation/synthesis-manifest.json".into(),
                sm_bytes.clone(),
            ),
            (
                "remediation/remediation-plan.json".into(),
                bytes(&remediation)?,
            ),
            ("remediation/manifest.json".into(), bytes(&rm)?),
            (
                "tests/review-record.json".into(),
                if test_plan_schema == test_plan::TEST_PLAN_SCHEMA_V2 {
                    bytes(review)?
                } else {
                    review_bytes.to_vec()
                },
            ),
            ("tests/synthesis.json".into(), synthesis_bytes),
            ("tests/synthesis-manifest.json".into(), sm_bytes),
            ("tests/test-plan.json".into(), bytes(&tests)?),
            ("tests/manifest.json".into(), bytes(&tm)?),
        ]),
    })
}
