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
