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

#[test]
fn durable_resume_reconstructs_owners_and_keeps_pending_denominator() {
    use adl::codefriend::integration::journey::{resume, Continuation};
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let j = prepare_local(f.options()).unwrap();
    let output = j.output().to_path_buf();
    drop(j);
    let mut resumed = resume(&output).unwrap();
    assert!(resumed.graph().is_some());
    resumed.continue_with(Continuation::Status).unwrap();
    assert_eq!(resumed.manifest().status, StageStatus::Pending);
    assert!(
        resume(&output).is_err(),
        "the admission owner lock remains held during continuation"
    );
}
#[test]
fn resume_rejects_retained_artifact_tampering_and_uncheckpointed_outputs() {
    use adl::codefriend::integration::journey::resume;
    for name in ["structure.json", "orphan-effect.json", "session.json"] {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let j = prepare_local(f.options()).unwrap();
        let output = j.output().to_path_buf();
        drop(j);
        fs::write(output.join(name), "{}").unwrap();
        assert!(resume(&output).is_err(), "{name}");
    }
}
#[test]
fn resume_rejects_missing_checkpoint_or_manifest() {
    use adl::codefriend::integration::journey::resume;
    for altered in ["checkpoint-0001.json", "journey-0003.json"] {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let j = prepare_local(f.options()).unwrap();
        let output = j.output().to_path_buf();
        drop(j);
        fs::remove_file(output.join(altered)).unwrap();
        assert!(resume(&output).is_err());
    }
}
#[test]
fn compiled_cli_dispatch_can_observe_a_resumed_journey() {
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let request = f.dir.path().join("journey-request.json");
    fs::write(&request, serde_json::to_vec(&f.options()).unwrap()).unwrap();
    let exe = env!("CARGO_BIN_EXE_adl");
    assert!(Command::new(exe)
        .args(["codefriend", "journey", "local", "--request"])
        .arg(request)
        .status()
        .unwrap()
        .success());
    let step = f.dir.path().join("step.json");
    fs::write(&step, r#"{"stage":"status"}"#).unwrap();
    let result = Command::new(exe)
        .args(["codefriend", "journey", "resume", "--output"])
        .arg(f.options().output)
        .arg("--request")
        .arg(step)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let manifest: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(manifest["status"], "pending");
}

#[test]
fn failed_review_continuation_survives_restart_without_another_dispatch() {
    use adl::codefriend::integration::journey::{resume, Continuation};
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let j = prepare_local(f.options()).unwrap();
    let output = j.output().to_path_buf();
    drop(j);
    let request = || Continuation::Review {
        provider_request: f.dir.path().join("missing-provider-request.json"),
        run_id: "review1".into(),
        cancel_file: None,
    };
    let mut j = resume(&output).unwrap();
    assert!(j.continue_with(request()).is_err());
    assert_eq!(j.manifest().stages["review"].status, StageStatus::Failed);
    drop(j);
    let mut j = resume(&output).unwrap();
    assert!(j.continue_with(request()).is_err());
    assert_eq!(j.manifest().status, StageStatus::Failed);
    assert!(output.join("intent-review.json").is_file());
    assert!(!output.join("review").exists());
}

#[cfg(unix)]
#[test]
fn resume_rejects_broadened_private_record_permissions() {
    use adl::codefriend::integration::journey::resume;
    use std::os::unix::fs::PermissionsExt;
    for name in [
        "",
        "session.json",
        "checkpoint-0000.json",
        "journey-0000.json",
    ] {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let j = prepare_local(f.options()).unwrap();
        let output = j.output().to_path_buf();
        drop(j);
        fs::set_permissions(
            output.join(name),
            fs::Permissions::from_mode(if name.is_empty() { 0o755 } else { 0o644 }),
        )
        .unwrap();
        assert!(resume(&output).is_err(), "broadened permissions: {name}");
    }
}

#[test]
fn resume_rejects_source_changes_and_deleted_admission() {
    use adl::codefriend::integration::journey::resume;
    for deleted in [false, true] {
        let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
        let j = prepare_local(f.options()).unwrap();
        let output = j.output().to_path_buf();
        drop(j);
        if deleted {
            fs::remove_dir_all(f.options().store).unwrap();
        } else {
            fs::remove_dir_all(f.source.join(".git")).unwrap();
        }
        assert!(resume(&output).is_err());
    }
}

#[test]
fn ci_continuation_pins_original_provenance() {
    use adl::codefriend::{
        ingestion::ci,
        integration::journey::{prepare_source, resume, AcquisitionSource},
    };
    let f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let options = f.options();
    let (_, mut receipt) = ci::acquire(
        &f.source,
        &options.repository,
        &f.revision,
        options.scope.clone(),
        env!("CODEFRIEND_BUILD_REVISION"),
        [("run_id".into(), "123".into())].into(),
    )
    .unwrap();
    let path = f.dir.path().join("ci-receipt.json");
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let j = prepare_source(
        options,
        AcquisitionSource::Ci {
            receipt: path.clone(),
        },
    )
    .unwrap();
    let output = j.output().to_path_buf();
    drop(j);
    drop(resume(&output).unwrap());
    // This remains a structurally valid receipt for the same packet. The saved
    // original provenance must nevertheless reject replacement with another run.
    receipt.metadata.insert("run_id".into(), "456".into());
    fs::write(&path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    assert!(resume(&output).is_err());
}

fn journey_provider_request() -> adl::provider_communication::ProviderInvocationRequestV1 {
    use adl::{model_identity::ModelIdentityStrengthV1, provider_communication::*};
    let route = ProviderRouteV1 {
        provider_kind: ProviderKindV1::Hosted,
        provider: "openai".to_string(),
        runtime_surface: RuntimeSurfaceV1::HostedApi,
        provider_model_id: "codefriend-fixture-model".to_string(),
        endpoint_ref: Some("http://127.0.0.1:1".to_string()),
        credential_ref: Some("env:ADL_CODEFRIEND_REVIEW_FIXTURE_KEY".to_string()),
        source_registry: Some("codefriend-review-fixture".to_string()),
    };
    let mut model_identity = hosted_model_identity(
        "openai",
        "codefriend-fixture-model",
        "codefriend-fixture-model",
        Some("codefriend-review-fixture".to_string()),
    );
    model_identity.identity_strength = ModelIdentityStrengthV1::ProviderAsserted;
    ProviderInvocationRequestV1 {
        route,
        model_identity,
        prompt_contract_ref: "template.replaced.by.runner".to_string(),
        lane_ref: "template".to_string(),
        run_id: None,
        request_id: None,
        attempt_policy: ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 5_000,
            retry_backoff_ms: Some(1),
        },
        input_text: None,
        max_output_tokens: Some(512),
        context_window_tokens: None,
        reasoning_effort: None,
        clear_thinking: Some(true),
        temperature: Some(0.0),
        top_p: None,
        local_keep_alive: None,
        inference_parameter_fingerprint: Some("temperature=0,max_output_tokens=512".into()),
        tool_surface: Some("none".into()),
        governance_surface: Some("read_only_findings_only".into()),
        evaluator_ref: None,
        benchmark_ref: None,
    }
}

#[allow(dead_code)]
#[path = "../examples/codefriend_palace_fixture.rs"]
mod journey_authority_fixture;

// Actual provider adapter transport, but deterministic loopback output; no paid provider.
fn journey_responses_server() -> (String, std::thread::JoinHandle<usize>) {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        time::{Duration, Instant},
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let worker = std::thread::spawn(move || {
        for _ in 0..8 {
            let deadline = Instant::now() + Duration::from_secs(30);
            let mut stream = loop {
                match listener.accept() {
                    Ok((s, _)) => break s,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            Instant::now() < deadline,
                            "expected review request did not arrive"
                        );
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(e) => panic!("fixture accept failed: {e}"),
                }
            };
            stream.set_nonblocking(false).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            loop {
                let mut chunk = [0u8; 4096];
                let n = stream.read(&mut chunk).unwrap();
                assert!(n > 0);
                request.extend_from_slice(&chunk[..n]);
                assert!(request.len() < 1024 * 1024);
                if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let length: usize = headers
                        .lines()
                        .find_map(|line| {
                            let (k, v) = line.split_once(':')?;
                            (k.eq_ignore_ascii_case("content-length"))
                                .then(|| v.trim().parse().unwrap())
                        })
                        .unwrap();
                    if request.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            assert!(String::from_utf8_lossy(&request).contains("answer"));
            let body = r#"{"output_text":"{\"findings\":[]}"}"#;
            write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();
        }
        8
    });
    (endpoint, worker)
}
fn journey_step(
    output: &Path,
    step: adl::codefriend::integration::journey::Continuation,
) -> serde_json::Value {
    let input = output.parent().unwrap().join("continuation-request.json");
    fs::write(&input, serde_json::to_vec(&step).unwrap()).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "journey", "resume", "--output"])
        .arg(output)
        .arg("--request")
        .arg(input)
        .env(
            "ADL_CODEFRIEND_REVIEW_FIXTURE_KEY",
            "public-loopback-fixture",
        )
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}

#[test]
fn actual_journey_continuations_complete_all_eighteen_stages_and_resume() {
    use adl::codefriend::{
        architecture::{
            rationale::{BoundarySelection, RationaleSelection},
            structure::StructureReport,
        },
        evidence::{contracts::ReviewRecord, store::Store},
        integration::journey::{resume, Continuation},
        memory::{
            baseline::{AdmittedBaselines, BaselineRef},
            palace::{self, IndexRequest, RetrieveRequest},
            palace_authority,
        },
        publication::{self, append_decision, DecisionKind},
    };
    let mut f = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    fs::write(
        f.source.join("compose.json"),
        r#"{"services":{"api":{"image":"fixture:v1","labels":{"codefriend.boundary":"core"}}}}"#,
    )
    .unwrap();
    fs::write(f.source.join("adr.md"),"+++\nstatus = \"accepted\"\nboundary = \"core\"\nservice = \"api\"\ndecision_key = \"core_service\"\nchoice = \"separate\"\n+++\nKeep the declared boundary explicit.\n").unwrap();
    git(&f.source, &["add", "."]);
    git(
        &f.source,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "context declarations",
        ],
    );
    f.revision = git(&f.source, &["rev-parse", "HEAD"]);
    let options = |fixture: &Fixture, name: &str| {
        let mut o = fixture.options();
        o.output = fixture.dir.path().join(name);
        o.scope.context = vec!["adr.md".into(), "compose.json".into()];
        o.scope.max_files = 3;
        o
    };
    let baseline_output = f.dir.path().join("baseline-journey");
    drop(prepare_local(options(&f, "baseline-journey")).unwrap());
    let (endpoint, server) = journey_responses_server();
    let mut provider = journey_provider_request();
    provider.route.endpoint_ref = Some(endpoint);
    let provider_path = f.dir.path().join("journey-provider.json");
    fs::write(&provider_path, serde_json::to_vec(&provider).unwrap()).unwrap();
    journey_step(
        &baseline_output,
        Continuation::Review {
            provider_request: provider_path.clone(),
            run_id: "baseline-provider-review".into(),
            cancel_file: None,
        },
    );
    let baseline_graph: StructureReport =
        serde_json::from_slice(&fs::read(baseline_output.join("structure.json")).unwrap()).unwrap();
    let baseline_review: ReviewRecord =
        publication::read_review(&baseline_output.join("review/review-record.json")).unwrap();
    let baseline_root = f.dir.path().join("baselines");
    {
        let store = Store::open(&f.options().store, || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
        .unwrap();
        let backend = AdmittedBaselines::open(&store, &baseline_root, true).unwrap();
        backend.retain(&baseline_review).unwrap();
    }
    fs::write(
        f.source.join("lib.rs"),
        "pub fn answer() -> u8 { 42 }\npub fn another() -> u8 { 7 }\n",
    )
    .unwrap();
    git(&f.source, &["add", "lib.rs"]);
    git(
        &f.source,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-m",
            "current revision",
        ],
    );
    f.revision = git(&f.source, &["rev-parse", "HEAD"]);
    let output = f.dir.path().join("complete-journey");
    let j = prepare_local(options(&f, "complete-journey")).unwrap();
    let graph = j.graph().unwrap().clone();
    drop(j);
    journey_step(
        &output,
        Continuation::Impact {
            changes: ChangeSet {
                schema: "codefriend.impact.v1".into(),
                repository: graph.record.run.repository.clone(),
                revision: f.revision.clone(),
                graph_digest: graph.digest.clone(),
                targets: vec![ChangeTarget::Module("crate".into())],
            },
        },
    );
    journey_step(
        &output,
        Continuation::Rationale {
            selection: RationaleSelection {
                schema: "codefriend.rationale.v1".into(),
                graph_digest: graph.digest,
                revision: f.revision.clone(),
                boundaries: vec![BoundarySelection {
                    boundary: "core".into(),
                    deployment_path: "compose.json".into(),
                    service: "api".into(),
                    rationale_paths: vec!["adr.md".into()],
                }],
            },
        },
    );
    journey_step(
        &output,
        Continuation::Drift {
            baseline_root: baseline_root.clone(),
            baseline: baseline_output.join("structure.json"),
        },
    );
    journey_step(
        &output,
        Continuation::Review {
            provider_request: provider_path,
            run_id: "current-provider-review".into(),
            cancel_file: None,
        },
    );
    assert_eq!(
        server.join().unwrap(),
        8,
        "exactly four loopback calls per review"
    );
    let current = publication::read_review(&output.join("review/review-record.json")).unwrap();
    assert_ne!(baseline_graph.record.run.revision, current.run.revision);
    let authority_root = f.dir.path().join("authority");
    journey_authority_fixture::generate(&authority_root).unwrap();
    let authority = palace_authority::provision(
        &authority_root.join("trust.json"),
        &authority_root.join("authority-evidence.json"),
    )
    .unwrap();
    let before = BaselineRef::from_record(&baseline_review).unwrap();
    let after = BaselineRef::from_record(&current).unwrap();
    journey_step(
        &output,
        Continuation::Palace {
            baseline_root,
            palace_root: f.dir.path().join("palace"),
            trust: authority_root.join("trust.json"),
            authority: authority_root.join("authority-evidence.json"),
            index: IndexRequest {
                schema: palace::VERSION.into(),
                references: vec![before.clone(), after.clone()],
                observed_epoch_ms: 0,
                stale_after_ms: 600_000,
                max_working_set_items: 8,
            },
            retrieve: RetrieveRequest {
                schema: palace::VERSION.into(),
                baseline: before,
                current: after,
                expected_identity_root: authority.identity().identity_root.clone(),
                expected_continuity_head: authority.continuity().record().continuity_head.clone(),
                packet_observed_epoch_ms: 0,
                observed_epoch_ms: 0,
                stale_after_ms: 600_000,
                max_working_set_items: 8,
            },
        },
    );
    let destination = f.dir.path().join("exports");
    fs::create_dir(&destination).unwrap();
    let approvals = f.dir.path().join("approvals");
    let font = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|p| p.is_file())
    .expect("native PDF fixture needs installed Unicode font");
    for (format, key, extension) in [
        (PublicationFormat::Markdown, "markdown", "md"),
        (PublicationFormat::Html, "html", "html"),
        (PublicationFormat::Pdf, "pdf", "pdf"),
    ] {
        journey_step(
            &output,
            Continuation::PreparePublication {
                destination: destination.clone(),
                format,
            },
        );
        let publication = publication::read_publication(
            &output.join(format!("publication_{key}/publication.json")),
        )
        .unwrap();
        append_decision(
            &approvals,
            &current,
            &publication,
            DecisionKind::Approved,
            "component-fixture",
            "Synthetic approval for exact native journey format",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
        .unwrap();
        journey_step(
            &output,
            Continuation::Export {
                approval_store: approvals.clone(),
                destination: destination.clone(),
                format,
                font: Some(font.clone()),
            },
        );
        assert!(destination
            .join(publication.target)
            .join(format!("report.{extension}"))
            .is_file());
    }
    let status = journey_step(&output, Continuation::Status);
    assert_eq!(status["status"], "complete");
    assert_eq!(status["stages"].as_object().unwrap().len(), 18);
    assert!(status["stages"]
        .as_object()
        .unwrap()
        .values()
        .all(|v| v["status"] == "complete"));
    let mut j = resume(&output).unwrap();
    assert!(j
        .continue_with(Continuation::Review {
            provider_request: f.dir.path().join("never-read"),
            run_id: "no-replay".into(),
            cancel_file: None
        })
        .is_err());
    drop(j);
    assert_eq!(git(&f.source, &["status", "--porcelain=v1"]), "");
    // A completed export cannot survive tampered bytes on a later restart.
    fs::write(destination.join("report-md/report.md"), "tampered").unwrap();
    assert!(resume(&output).is_err());
}

// This lane explicitly requires a case-insensitive macOS temporary filesystem;
// an unsupported filesystem fails setup rather than silently claiming proof.
#[cfg(target_os = "macos")]
#[test]
fn case_alias_source_output_and_store_are_rejected_before_writes() {
    let fixture = Fixture::new("pub fn answer() -> u8 { 42 }\n");
    let alias = fixture.dir.path().join("Source");
    assert!(
        same_file::is_same_file(&fixture.source, &alias).unwrap(),
        "case-insensitive filesystem required for this platform regression"
    );
    let before = git(&fixture.source, &["status", "--porcelain=v1"]);
    for store in [false, true] {
        let mut options = fixture.options();
        if store {
            options.store = alias.join("retained");
        } else {
            options.output = alias.join("journey");
        }
        let error = prepare_local(options)
            .err()
            .expect("source alias must reject");
        assert!(error.to_string().contains("journey_output_overlaps_source"));
        assert!(!fixture.source.join("journey").exists());
        assert!(!fixture.source.join("retained").exists());
        assert!(!fixture.options().output.exists());
        assert!(!fixture.options().store.exists());
    }
    assert_eq!(before, git(&fixture.source, &["status", "--porcelain=v1"]));
}
