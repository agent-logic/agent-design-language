//! PVF deterministic local integration: real Git acquisition and stage owners.
//! No provider/network, installed candidate, website, or complete Beta acceptance claim.
use adl::codefriend::{
    architecture::{
        impact::{ChangeSet, ChangeTarget},
        structure::{self, BoundaryPolicy},
    },
    evidence::Retention,
    governance::local::{self as fitness, Policy, Rule},
    ingestion::Scope,
    integration::{
        journey::{prepare_local, LocalJourneyOptions, StageStatus},
        PublicationFormat,
    },
};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> String {
    let result = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).unwrap().trim().into()
}
struct Fixture {
    dir: tempfile::TempDir,
    source: PathBuf,
    revision: String,
}
impl Fixture {
    fn new(source_text: &str) -> Self {
        let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = dir.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        fs::write(source.join("lib.rs"), source_text).unwrap();
        git(&source, &["add", "lib.rs"]);
        git(
            &source,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-m",
                "synthetic fixture",
            ],
        );
        let revision = git(&source, &["rev-parse", "HEAD"]);
        Self {
            dir,
            source,
            revision,
        }
    }
    fn options(&self) -> LocalJourneyOptions {
        LocalJourneyOptions {
            checkout: self.source.clone(),
            repository: "https://example.com/owner/repo".into(),
            revision: self.revision.clone(),
            scope: Scope {
                analysis: vec!["lib.rs".into()],
                context: vec![],
                max_files: 1,
                max_bytes: 4096,
                max_file_bytes: 4096,
            },
            store: self.dir.path().join("store"),
            output: self.dir.path().join("journey"),
            retention: Retention { seconds: 3600 },
            boundary_policy: BoundaryPolicy {
                schema: structure::VERSION.into(),
                crate_root: "lib.rs".into(),
                manifest_path: None,
                layers: [("lib.rs".into(), "core".into())].into(),
                allowed: BTreeSet::new(),
                coupling_threshold: 2,
            },
            fitness_policy: Policy {
                schema: fitness::VERSION.into(),
                rules: vec![Rule {
                    id: "no_network".into(),
                    kind: "forbidden_declared_use".into(),
                    source_path: "lib.rs".into(),
                    forbidden_prefix: "reqwest".into(),
                }],
            },
        }
    }
}

#[test]
fn actual_offline_stages_preserve_source_and_never_claim_full_journey() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    // The capture must read committed Git objects, not replace the dirty working file.
    fs::write(
        fixture.source.join("lib.rs"),
        "pub fn answer() -> u8 { 99 }\n",
    )
    .unwrap();
    let before = git(&fixture.source, &["status", "--porcelain=v1"]);
    let bytes = fs::read(fixture.source.join("lib.rs")).unwrap();
    let journey = prepare_local(fixture.options()).unwrap();
    assert_eq!(journey.manifest().revision, fixture.revision);
    assert_eq!(journey.manifest().status, StageStatus::Pending);
    for stage in ["acquisition", "admission", "structure", "fitness"] {
        assert_eq!(
            journey.manifest().stages[stage].status,
            StageStatus::Complete,
            "{stage}"
        );
        assert!(journey.manifest().stages[stage].digest.is_some());
    }
    for stage in [
        "review",
        "approval_markdown",
        "markdown",
        "html",
        "pdf",
        "palace_comparison",
    ] {
        assert_eq!(
            journey.manifest().stages[stage].status,
            StageStatus::Pending
        );
    }
    assert_eq!(before, git(&fixture.source, &["status", "--porcelain=v1"]));
    assert_eq!(bytes, fs::read(fixture.source.join("lib.rs")).unwrap());
    assert!(!journey.output().join("review").exists());
    assert!(!journey.output().join("publication").exists());
    let captured: serde_json::Value =
        serde_json::from_slice(&fs::read(journey.output().join("acquisition.json")).unwrap())
            .unwrap();
    assert_eq!(
        captured["objects"][0]["content"],
        "pub fn answer() -> u8 { 42 }\n"
    );
}

#[test]
fn cli_executes_native_offline_journey_and_reports_pending_not_complete() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let request = fixture.dir.path().join("request.json");
    fs::write(&request, serde_json::to_vec(&fixture.options()).unwrap()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "journey", "local", "--request"])
        .arg(&request)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let manifest: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(manifest["status"], "pending");
    assert_eq!(manifest["stages"]["structure"]["status"], "complete");
    assert_eq!(manifest["stages"]["review"]["status"], "pending");
    assert_eq!(manifest["stages"]["approval_markdown"]["status"], "pending");
    assert_eq!(git(&fixture.source, &["status", "--porcelain=v1"]), "");
}

#[test]
fn cli_rejects_unknown_authority_fields_before_creating_outputs() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let request = fixture.dir.path().join("request.json");
    let mut value = serde_json::to_value(fixture.options()).unwrap();
    value["approved"] = serde_json::json!(true);
    fs::write(&request, serde_json::to_vec(&value).unwrap()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "journey", "local", "--request"])
        .arg(&request)
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!fixture.dir.path().join("journey").exists());
    assert!(!fixture.dir.path().join("store").exists());
}

#[test]
fn malformed_scope_and_wrong_provenance_cannot_reach_dispatch_boundary() {
    for mode in 0..3 {
        let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let mut options = fixture.options();
        match mode {
            0 => options.scope.analysis = vec!["../lib.rs".into()],
            1 => options.repository = "https://example.com/other/repo".into(),
            _ => options.revision = "0".repeat(40),
        }
        let mut downstream_calls = 0;
        let result = prepare_local(options).map(|_| {
            downstream_calls += 1;
        });
        assert!(result.is_err());
        assert_eq!(downstream_calls, 0);
        assert!(!fixture.dir.path().join("journey").exists());
        assert!(!fixture.dir.path().join("store").exists());
    }
}

#[test]
fn failed_fitness_is_terminal_and_source_is_unchanged() {
    let fixture = Fixture::new("use reqwest::Client;\npub fn answer() -> u8 { 42 }\n");
    let mut journey = prepare_local(fixture.options()).unwrap();
    assert_eq!(
        journey.manifest().stages["fitness"].status,
        StageStatus::Failed
    );
    assert_eq!(journey.manifest().status, StageStatus::Failed);
    assert!(journey
        .prepare_publication(
            &fixture.dir.path().join("destination"),
            PublicationFormat::Markdown
        )
        .is_err());
    assert_eq!(git(&fixture.source, &["status", "--porcelain=v1"]), "");
    assert!(!journey.output().join("review").exists());
}

#[test]
fn foreign_impact_graph_fails_explicitly_and_cannot_be_retried() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let mut journey = prepare_local(fixture.options()).unwrap();
    let changes = ChangeSet {
        schema: "codefriend.impact.v1".into(),
        repository: journey.manifest().repository.clone(),
        revision: fixture.revision.clone(),
        graph_digest: "0".repeat(64),
        targets: vec![ChangeTarget::Module("crate".into())],
    };
    journey.analyze_impact(changes.clone()).unwrap();
    assert_eq!(
        journey.manifest().stages["impact"].status,
        StageStatus::Failed
    );
    assert!(journey.analyze_impact(changes).is_err());
    assert_eq!(
        journey.manifest().stages["review"].status,
        StageStatus::Pending
    );
}

#[test]
fn impact_and_compatible_second_revision_drift_use_retained_native_records() {
    let mut fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let first = prepare_local(fixture.options()).unwrap();
    let baseline = first.graph().unwrap().clone();
    drop(first);
    fs::write(
        fixture.source.join("lib.rs"),
        "pub fn answer() -> u8 { 42 }\npub fn another() -> u8 { 7 }\n",
    )
    .unwrap();
    git(&fixture.source, &["add", "lib.rs"]);
    git(
        &fixture.source,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "second synthetic revision",
        ],
    );
    fixture.revision = git(&fixture.source, &["rev-parse", "HEAD"]);
    let mut options = fixture.options();
    options.output = fixture.dir.path().join("second-journey");
    let mut second = prepare_local(options).unwrap();
    let changes = ChangeSet {
        schema: "codefriend.impact.v1".into(),
        repository: second.manifest().repository.clone(),
        revision: fixture.revision.clone(),
        graph_digest: second.graph().unwrap().digest.clone(),
        targets: vec![ChangeTarget::Module("crate".into())],
    };
    second.analyze_impact(changes).unwrap();
    assert_eq!(
        second.manifest().stages["impact"].status,
        StageStatus::Complete
    );
    second
        .analyze_drift(&fixture.dir.path().join("baselines"), baseline)
        .unwrap();
    assert_eq!(
        second.manifest().stages["drift"].status,
        StageStatus::Complete
    );
    assert_eq!(second.manifest().status, StageStatus::Pending);
    assert_eq!(git(&fixture.source, &["status", "--porcelain=v1"]), "");
}

#[test]
fn existing_output_and_source_nested_outputs_are_rejected_without_mutation() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let mut options = fixture.options();
    options.output = fixture.source.join("results");
    assert!(prepare_local(options).is_err());
    assert!(!fixture.source.join("results").exists());
    let options = fixture.options();
    fs::create_dir(&options.output).unwrap();
    fs::write(options.output.join("keep"), "existing").unwrap();
    assert!(prepare_local(options).is_err());
    assert_eq!(
        fs::read_to_string(fixture.dir.path().join("journey/keep")).unwrap(),
        "existing"
    );
}

#[cfg(unix)]
#[test]
fn symlink_output_alias_cannot_write_inside_source() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let alias = fixture.dir.path().join("alias");
    std::os::unix::fs::symlink(&fixture.source, &alias).unwrap();
    let mut options = fixture.options();
    options.output = alias.join("results");
    assert!(prepare_local(options).is_err());
    assert!(!fixture.source.join("results").exists());
}

/// Composition proof uses generated native review output and explicitly synthetic
/// lane responses/approval. Journey's provider-only review wrapper is not executed.
#[test]
fn generated_four_lane_review_flows_through_three_separately_approved_exports() {
    use adl::codefriend::{
        evidence::{contracts::Completion, Admission},
        integration::prepare_publication_bundle_for_format,
        publication::{
            self, append_decision, DecisionKind, HtmlRenderOptions, MarkdownRenderOptions,
            PdfRenderOptions,
        },
        review::runner::{run_with_executor, ExecutionOptions, LaneExecution},
    };
    use adl::provider_communication::ProviderInvocationFinalStatusV1;
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let journey = prepare_local(fixture.options()).unwrap();
    let admission: Admission =
        serde_json::from_slice(&fs::read(journey.output().join("admission.json")).unwrap())
            .unwrap();
    let review_dir = fixture.dir.path().join("generated-review");
    let mut lanes = BTreeSet::new();
    let run = run_with_executor(
        ExecutionOptions {
            out: review_dir.clone(),
            run_id: "generated-component-review".into(),
            cancel_file: None,
        },
        admission,
        "fixture:deterministic:no-provider".into(),
        |lane, prompt, _| {
            assert!(prompt.contains("answer"));
            assert!(lanes.insert(lane.id().to_owned()));
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(r#"{"findings":[]}"#.into()),
            })
        },
    )
    .unwrap();
    assert_eq!(lanes.len(), 4);
    assert_eq!(run.completion, Completion::Complete);
    run.review_record.validate().unwrap();
    assert_eq!(
        run.review_record.admission.packet.revision,
        fixture.revision
    );
    let review_record = review_dir.join("review-record.json");
    assert_eq!(
        publication::read_review(&review_record).unwrap(),
        run.review_record
    );
    let destination = fixture.dir.path().join("exports");
    fs::create_dir(&destination).unwrap();
    let approval_store = fixture.dir.path().join("approvals");
    let font = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|p| p.is_file())
    .expect("PDF component proof requires an installed Unicode TrueType font");
    let mut decisions = BTreeSet::new();
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        let key = format.key();
        let bundle = fixture.dir.path().join(format!("bundle-{key}"));
        let bound =
            prepare_publication_bundle_for_format(&review_record, &bundle, &destination, format)
                .unwrap();
        assert_eq!(bound.artifact_manifest.len(), 13);
        publication::verify_artifacts(&bundle.join("artifacts"), &bound.artifact_manifest).unwrap();
        assert!(bound.approval.is_none());
        let out = destination.join(&bound.target);
        let render = || -> anyhow::Result<serde_json::Value> {
            let review_record = review_record.clone();
            let publication = bundle.join("publication.json");
            let approval_store = approval_store.clone();
            let artifact_root = bundle.join("artifacts");
            let synthesis = "synthesis/synthesis.json".into();
            let remediation_plan = "remediation/remediation-plan.json".into();
            let test_plan = "tests/test-plan.json".into();
            let destination_root = destination.clone();
            let out = out.clone();
            Ok(match format {
                PublicationFormat::Markdown => {
                    serde_json::to_value(publication::render_markdown(MarkdownRenderOptions {
                        review_record,
                        publication,
                        approval_store,
                        artifact_root,
                        synthesis,
                        remediation_plan,
                        test_plan,
                        destination_root,
                        out,
                    })?)?
                }
                PublicationFormat::Html => {
                    serde_json::to_value(publication::render_html(HtmlRenderOptions {
                        review_record,
                        publication,
                        approval_store,
                        artifact_root,
                        synthesis,
                        remediation_plan,
                        test_plan,
                        destination_root,
                        out,
                    })?)?
                }
                PublicationFormat::Pdf => {
                    serde_json::to_value(publication::render_pdf(PdfRenderOptions {
                        review_record,
                        publication,
                        approval_store,
                        artifact_root,
                        synthesis,
                        remediation_plan,
                        test_plan,
                        destination_root,
                        out,
                        font: font.clone(),
                    })?)?
                }
            })
        };
        // Another format's approval must never authorize this format's output.
        assert!(render().is_err());
        assert!(!out.exists());
        append_decision(
            &approval_store,
            &run.review_record,
            &bound,
            DecisionKind::Withheld,
            "component-fixture",
            "Withheld for authorization regression",
            1_700_000_000,
        )
        .unwrap();
        assert!(render().is_err());
        assert!(!out.exists());
        let approved = append_decision(
            &approval_store,
            &run.review_record,
            &bound,
            DecisionKind::Approved,
            "component-fixture",
            "Synthetic component approval for this exact format",
            1_700_000_001,
        )
        .unwrap();
        assert!(decisions.insert(approved.digest.clone()));
        let result = render().unwrap();
        assert_eq!(result["approval_decision_digest"], approved.digest);
        let extension = match format {
            PublicationFormat::Markdown => "md",
            PublicationFormat::Html => "html",
            PublicationFormat::Pdf => "pdf",
        };
        let bytes = fs::read(out.join(format!("report.{extension}"))).unwrap();
        assert_eq!(
            result["report_digest"],
            adl::codefriend::ingestion::digest(&bytes)
        );
        assert!(!bytes.is_empty());
        if extension == "pdf" {
            assert!(bytes.starts_with(b"%PDF-"));
        } else {
            assert!(String::from_utf8(bytes)
                .unwrap()
                .contains(&fixture.revision));
        }
    }
    assert_eq!(decisions.len(), 3);
    assert_eq!(
        journey.manifest().stages["review"].status,
        StageStatus::Pending
    );
    assert_eq!(journey.manifest().status, StageStatus::Pending);
    assert_eq!(git(&fixture.source, &["status", "--porcelain=v1"]), "");
}
