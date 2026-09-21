//! Real Git acquisition and retained native v2 owners; no provider or source execution.
use adl::codefriend::{
    architecture::structure_v2::BoundaryPolicyV2,
    evidence::{hash, Retention},
    governance::{
        artifact::FitnessArtifact,
        language::{Policy, Rule},
    },
    ingestion::{digest, Scope},
    integration::journey::{
        prepare_local, resume, JourneyManifest, StageStatus, VersionedLocalJourneyOptions,
    },
    language::{AnalysisPolicy, Language, Limits, ProjectRoot},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
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
// Mirrors only the checkpoint envelope to reseal transport hashes in the negative case.
#[derive(Serialize, Deserialize)]
struct Checkpoint {
    schema: String,
    sequence: usize,
    previous: Option<String>,
    session_digest: String,
    files: BTreeMap<String, String>,
    digest: String,
}
static OWNER_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn exercise(path: &str, text: &str, language: Language, erase_gap: bool) {
    let _guard = OWNER_TESTS.lock().unwrap_or_else(|e| e.into_inner());
    let dir = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    let source = dir.path().join("source");
    fs::create_dir(&source).unwrap();
    git(&source, &["init", "-b", "main"]);
    git(
        &source,
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    fs::write(source.join(path), text).unwrap();
    git(&source, &["add", "."]);
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
            "language fixture",
        ],
    );
    let revision = git(&source, &["rev-parse", "HEAD"]);
    let analysis = AnalysisPolicy {
        schema: adl::codefriend::language::VERSION.into(),
        files: [(path.into(), language)].into(),
        roots: vec![ProjectRoot {
            language,
            root: ".".into(),
            manifest: None,
        }],
        layers: [(path.into(), "core".into())].into(),
        allowed: Default::default(),
        limits: Limits {
            max_nodes: 10000,
            max_depth: 128,
            max_facts: 1000,
            max_output_bytes: 1024 * 1024,
        },
    };
    let output = dir.path().join("journey");
    let options = VersionedLocalJourneyOptions {
        checkout: source,
        repository: "https://example.com/owner/repo".into(),
        revision: revision.clone(),
        scope: Scope {
            analysis: vec![path.into()],
            context: vec![],
            max_files: 1,
            max_bytes: 4096,
            max_file_bytes: 4096,
        },
        store: dir.path().join("store"),
        output: output.clone(),
        retention: Retention { seconds: 3600 },
        boundary_policy: BoundaryPolicyV2 {
            schema: "codefriend.structure.v2".into(),
            analysis: analysis.clone(),
            coupling_threshold: 2,
        }
        .into(),
        fitness_policy: Policy {
            schema: "codefriend.fitness.v2".into(),
            analysis,
            rules: vec![Rule::ForbiddenResolvedEdge {
                id: "boundary".into(),
                source_path: path.into(),
                language,
                forbidden_targets: [path.into()].into(),
            }],
        }
        .into(),
    };
    let journey = prepare_local(options).unwrap();
    let original = serde_json::to_value(journey.manifest()).unwrap();
    assert_eq!(journey.manifest().schema, "codefriend.journey.v2");
    assert_eq!(journey.manifest().revision, revision);
    for name in ["structure", "fitness"] {
        assert_eq!(
            journey.manifest().stages[name].status,
            StageStatus::Complete
        );
        assert_eq!(
            journey.manifest().stages[name].reason.as_deref(),
            Some("analysis_gaps_reported")
        );
    }
    assert_eq!(journey.manifest().status, StageStatus::Pending);
    let graph = journey.graph_artifact().unwrap();
    assert!(!serde_json::to_value(graph).unwrap()["analysis_complete"]
        .as_bool()
        .unwrap());
    let fitness: FitnessArtifact =
        serde_json::from_slice(&fs::read(output.join("fitness.json")).unwrap()).unwrap();
    assert_eq!(fitness.exit_code(), 2);
    assert!(!fitness.passes());
    let sequence = journey.checkpoint_sequence();
    drop(journey);
    let mut restarted = resume(&output).unwrap();
    assert_eq!(
        serde_json::to_value(restarted.manifest()).unwrap(),
        original
    );
    assert_eq!(restarted.checkpoint_sequence(), sequence);
    let changes = adl::codefriend::architecture::impact_v2::ChangeSetV2 {
        schema: adl::codefriend::architecture::impact_v2::VERSION.into(),
        repository: restarted.manifest().repository.clone(),
        revision: revision.clone(),
        graph_digest: restarted.graph_artifact().unwrap().digest().into(),
        targets: vec![adl::codefriend::architecture::impact_v2::ChangeTargetV2::Path(path.into())],
    };
    restarted.analyze_impact(changes.clone()).unwrap();
    let impact = &restarted.manifest().stages["impact"];
    assert_eq!(impact.status, StageStatus::Complete);
    assert_eq!(impact.reason.as_deref(), Some("analysis_gaps_reported"));
    let payload: adl::codefriend::architecture::impact_v2::ImpactReportV2 =
        serde_json::from_slice(&fs::read(output.join("impact.json")).unwrap()).unwrap();
    assert!(!payload.analysis_complete);
    assert!(!payload.unknowns.is_empty());
    assert_eq!(
        payload.record.admission.digest,
        restarted.manifest().admission_digest
    );
    let continued = serde_json::to_value(restarted.manifest()).unwrap();
    let sequence = restarted.checkpoint_sequence();
    let checkpoint_path = output.join(format!("checkpoint-{:04}.json", sequence - 1));
    let checkpoint_bytes = fs::read(&checkpoint_path).unwrap();
    drop(restarted);
    let mut reopened = resume(&output).unwrap();
    assert_eq!(
        serde_json::to_value(reopened.manifest()).unwrap(),
        continued
    );
    assert!(reopened.analyze_impact(changes).is_err());
    assert_eq!(reopened.checkpoint_sequence(), sequence);
    assert_eq!(fs::read(&checkpoint_path).unwrap(), checkpoint_bytes);
    assert_eq!(
        serde_json::to_value(reopened.manifest()).unwrap(),
        continued
    );
    drop(reopened);
    if erase_gap {
        let name = format!("journey-{:04}.json", sequence - 1);
        let file = output.join(&name);
        let mut manifest: JourneyManifest =
            serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
        manifest.stages.get_mut("structure").unwrap().reason = None;
        let bytes = serde_json::to_vec_pretty(&manifest).unwrap();
        fs::write(&file, &bytes).unwrap();
        let checkpoint = output.join(format!("checkpoint-{:04}.json", sequence - 1));
        let mut cp: Checkpoint = serde_json::from_slice(&fs::read(&checkpoint).unwrap()).unwrap();
        cp.files.insert(name, digest(&bytes));
        cp.digest.clear();
        cp.digest = hash(&cp).unwrap();
        fs::write(checkpoint, serde_json::to_vec_pretty(&cp).unwrap()).unwrap();
        let error = resume(&output).err().expect("erased native gap must fail");
        assert!(
            error.to_string().contains("journey_analysis_stage_changed"),
            "{error:#}"
        );
    }
}
#[test]
fn rust_journey_retains_gaps_and_rejects_resealed_erasure() {
    exercise(
        "lib.rs",
        "use absent::Thing;\npub fn f() {}\n",
        Language::Rust,
        true,
    );
}
#[test]
fn java_journey_retains_original_admission_and_gaps() {
    exercise(
        "App.java",
        "import absent.Thing;\nclass App {}\n",
        Language::Java,
        false,
    );
}
#[test]
fn python_journey_retains_original_admission_and_gaps() {
    exercise(
        "app.py",
        "import absent\ndef f(): pass\n",
        Language::Python,
        false,
    );
}
#[test]
fn javascript_journey_retains_original_admission_and_gaps() {
    exercise(
        "app.js",
        "import x from 'absent';\nexport function f() {}\n",
        Language::JavaScript,
        false,
    );
}
