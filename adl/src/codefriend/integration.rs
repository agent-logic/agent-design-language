//! Native artifact challenges for the website integration boundary.
//!
//! A challenge is inspectable binding data, never authentication or permission to
//! append a decision. The eventual authenticated service must retain it and check
//! its current decision head under the approval owner's append lock.
pub mod journey;
use crate::codefriend::{
    evidence::{
        contracts::{Completion, Publication, ReviewRecord},
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
            review.run.completion == Completion::Complete,
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
            review.run.completion == Completion::Complete,
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
    let publication = build_publication_bundle(review_path, &stage, destination, format)?;
    for artifact in &publication.artifact_manifest {
        File::open(stage.join("artifacts").join(&artifact.path))?.sync_all()?;
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
) -> Result<Publication> {
    use crate::codefriend::{
        actions::{
            remediation::{plan_from_file as remediate, RemediationOptions},
            test_plan::{plan_from_file as test_plan, TestPlanOptions},
        },
        evidence::contracts::Artifact,
        ingestion::digest,
        publication::{read_review, ManifestInput},
        review::synthesis::{synthesize_from_file, SynthesisOptions},
    };
    use std::{collections::BTreeMap, fs};
    let review = read_review(review_path)?;
    ensure!(
        review.run.completion == Completion::Complete,
        "publication_bundle_requires_complete_run"
    );
    let artifact_root = output.join("artifacts");
    fs::create_dir(&artifact_root)?;
    synthesize_from_file(SynthesisOptions {
        input: review_path.into(),
        out: artifact_root.join("synthesis"),
    })?;
    remediate(RemediationOptions {
        input: artifact_root.join("synthesis/synthesis.json"),
        out: artifact_root.join("remediation"),
    })?;
    test_plan(TestPlanOptions {
        input: artifact_root.join("synthesis/synthesis.json"),
        out: artifact_root.join("tests"),
    })?;
    let mut artifacts = Vec::new();
    for (directory, names) in [
        (
            "synthesis",
            &["manifest.json", "review-record.json", "synthesis.json"][..],
        ),
        (
            "remediation",
            &[
                "manifest.json",
                "remediation-plan.json",
                "review-record.json",
                "synthesis-manifest.json",
                "synthesis.json",
            ][..],
        ),
        (
            "tests",
            &[
                "manifest.json",
                "review-record.json",
                "synthesis-manifest.json",
                "synthesis.json",
                "test-plan.json",
            ][..],
        ),
    ] {
        for name in names {
            let path = format!("{directory}/{name}");
            artifacts.push(Artifact {
                path: path.clone(),
                digest: digest(&fs::read(artifact_root.join(path))?),
            });
        }
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
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
