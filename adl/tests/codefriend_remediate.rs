//! PVF lane: runtime. Deterministic local CPU/filesystem proof for CodeFriend
//! remediation planning. No provider calls, source mutation, network, or issue
//! publication.

use adl::codefriend::{
    actions::remediation::{
        plan, read_plan_from_file, validate_plan, RemediationManifest, RemediationPlan,
    },
    evidence::{
        contracts::{Completion, ReviewRecord, Run, Severity},
        hash, Admission,
    },
    ingestion::digest,
    review::synthesis::{synthesize, ReviewSynthesis},
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/codefriend-remediate-tests/{name}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn completed_synthesis_bundle() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".csdlc/evidence/892/predecessor-openai-r5-synthesis")
}

fn run_remediation(input: &Path, out_dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "remediation", "--input"])
        .arg(input)
        .arg("--out")
        .arg(out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap()
}

fn completed_review_record() -> ReviewRecord {
    serde_json::from_slice(
        &fs::read(completed_synthesis_bundle().join("review-record.json")).unwrap(),
    )
    .unwrap()
}

fn synthesis_case(anchors: &[&str]) -> (ReviewSynthesis, ReviewRecord) {
    let mut record = completed_review_record();
    let base = record.findings[0].clone();
    record.findings = anchors
        .iter()
        .enumerate()
        .map(|(index, anchor)| {
            let mut finding = base.clone();
            finding.perspective = if index % 2 == 0 {
                "correctness".to_string()
            } else {
                "security".to_string()
            };
            finding.rule = format!("bounded-remediation-test-{index}");
            finding.semantic_anchor = (*anchor).to_string();
            finding.title = format!("bounded remediation finding {index}");
            finding.severity = Severity::High;
            finding.id = finding.identity().unwrap();
            finding
        })
        .collect();
    record.validate().unwrap();
    let synthesis = synthesize(&record).unwrap();
    (synthesis, record)
}

fn completed_review_record_for_paths(paths: &[&str]) -> ReviewRecord {
    let source = completed_review_record();
    let mut packet = source.admission.packet.clone();
    let base_object = packet
        .objects
        .iter()
        .find(|object| object.content.is_some())
        .unwrap()
        .clone();
    let mut paths = paths
        .iter()
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    packet.scope.analysis = paths.clone();
    packet.scope.context.clear();
    packet.objects = paths
        .iter()
        .map(|path| {
            let mut object = base_object.clone();
            object.path = path.clone();
            object.analysis_support = if path.ends_with(".rs") {
                "rust_source_not_yet_analyzed".to_string()
            } else {
                "context_or_unsupported_analysis".to_string()
            };
            object
        })
        .collect();
    packet.scope_digest = digest(&serde_json::to_vec(&packet.scope).unwrap());
    packet.packet_id.clear();
    packet.packet_id = digest(&serde_json::to_vec(&packet).unwrap());
    packet.validate().unwrap();

    let admission = Admission::new(
        packet,
        source.admission.retention.clone(),
        source.admission.admitted_at,
    )
    .unwrap();
    let run = Run::new(
        &admission,
        source.run.lane_versions.clone(),
        source.run.provider_route.clone(),
        Completion::Complete,
        vec![],
    )
    .unwrap();
    let base_finding = source.findings[0].clone();
    let findings = admission
        .evidence
        .iter()
        .enumerate()
        .map(|(index, evidence)| {
            let mut finding = base_finding.clone();
            finding.repository = run.repository.clone();
            finding.perspective = if index % 2 == 0 {
                "correctness".to_string()
            } else {
                "security".to_string()
            };
            finding.rule = format!("bounded-path-contract-{index}");
            finding.semantic_anchor = evidence.path.clone();
            finding.title = format!("path contract finding {index}");
            finding.scope_digest = run.scope_digest.clone();
            finding.evidence = vec![evidence.id.clone()];
            finding.id = finding.identity().unwrap();
            finding
        })
        .collect();
    let record = ReviewRecord {
        admission,
        run,
        findings,
    };
    record.validate().unwrap();
    record
}

fn synthesis() -> (ReviewSynthesis, ReviewRecord) {
    synthesis_case(&[
        "adl/src/codefriend/review/runner.rs:85",
        "adl/src/codefriend/review/runner.rs:501",
    ])
}

#[test]
fn remediation_plan_orders_traceable_bounded_actions() {
    let (synthesis, record) = synthesis();
    let plan = plan(&synthesis, &record).unwrap();
    assert_eq!(plan.schema, "codefriend.remediation_plan.v1");
    assert_eq!(plan.actions.len(), 2);
    assert!(plan.omitted_findings.is_empty());
    assert_eq!(plan.action_order.len(), 2);
    assert!(plan
        .actions
        .iter()
        .all(|action| action.assignment_status == "unassigned"
            && action.owner_role == "repository-owner"
            && action
                .acceptance_criteria
                .iter()
                .any(|criterion| criterion.contains(&action.finding_id))));
    assert!(plan
        .actions
        .iter()
        .any(|action| !action.dependencies.is_empty()));
}

#[test]
fn remediation_plan_resolves_evidence_ids_to_review_record_paths() {
    let record = completed_review_record();
    let synthesis = synthesize(&record).unwrap();
    let plan = plan(&synthesis, &record).unwrap();
    assert_eq!(plan.actions.len(), 1);
    assert!(plan.omitted_findings.is_empty());
    assert_eq!(
        plan.actions[0].relevant_paths,
        ["lib/dnsmsg-parser/src/dns_message.rs"]
    );
}

#[test]
fn remediation_plan_preserves_dot_directories_and_root_files() {
    let record = completed_review_record_for_paths(&[".github/workflows/ci.yml", "Cargo.toml"]);
    let synthesis = synthesize(&record).unwrap();

    let plan = plan(&synthesis, &record).unwrap();
    let paths = plan
        .actions
        .iter()
        .flat_map(|action| action.relevant_paths.iter().map(String::as_str))
        .collect::<Vec<_>>();
    assert!(paths.contains(&".github/workflows/ci.yml"));
    assert!(paths.contains(&"Cargo.toml"));
    assert!(!paths.contains(&"github/workflows/ci.yml"));
    assert!(plan.omitted_findings.is_empty());
}

#[test]
fn remediation_plan_preserves_ingestion_valid_paths_with_spaces() {
    let record = completed_review_record_for_paths(&["docs/My File.md"]);
    let synthesis = synthesize(&record).unwrap();
    let plan = plan(&synthesis, &record).unwrap();
    assert_eq!(plan.actions.len(), 1);
    assert_eq!(plan.actions[0].relevant_paths, ["docs/My File.md"]);
    assert_eq!(plan.actions[0].owner_role, "documentation-owner");
}

#[test]
fn remediation_reader_rejects_tampered_paths_acceptance_and_cycles() {
    let (synthesis, record) = synthesis();
    let plan = plan(&synthesis, &record).unwrap();

    let mut bad_path = plan.clone();
    bad_path.actions[0].relevant_paths = vec!["../secret".to_string()];
    let err = validate_plan(&bad_path).unwrap_err().to_string();
    assert!(err.contains("unsupported_remediation_path"));

    let mut bad_acceptance = plan.clone();
    bad_acceptance.actions[0].acceptance_criteria = vec!["fix something".to_string()];
    let err = validate_plan(&bad_acceptance).unwrap_err().to_string();
    assert!(err.contains("untraceable_remediation_acceptance"));

    let mut bad_order = plan.clone();
    bad_order.action_order.reverse();
    let err = validate_plan(&bad_order).unwrap_err().to_string();
    assert!(err.contains("remediation_action_order_not_topological"));

    let mut cycle = plan;
    let first = cycle.actions[0].id.clone();
    let second = cycle.actions[1].id.clone();
    cycle.actions[0].dependencies = vec![second];
    cycle.actions[1].dependencies = vec![first];
    let err = validate_plan(&cycle).unwrap_err().to_string();
    assert!(err.contains("remediation_dependency_cycle"));
}

#[test]
fn installed_cli_generates_and_reads_remediation_plan() {
    let root = temp_dir("installed-cli");
    let bundle = completed_synthesis_bundle();
    let input = bundle.join("synthesis.json");
    let source_before = ["synthesis.json", "manifest.json", "review-record.json"]
        .map(|name| fs::read(bundle.join(name)).unwrap());
    let out_dir = root.join("remediation-out");

    let output = run_remediation(&input, &out_dir);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["schema"], "codefriend.remediation_plan.v1");
    assert_eq!(summary["action_count"], 1);
    assert!(out_dir.join("synthesis.json").exists());
    let manifest: RemediationManifest =
        serde_json::from_slice(&fs::read(out_dir.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest.action_count, 1);
    assert_eq!(manifest.synthesis_manifest_ref, "synthesis-manifest.json");
    assert_eq!(manifest.review_record_ref, "review-record.json");
    let plan_path = out_dir.join("remediation-plan.json");
    assert!(plan_path.exists());

    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "remediation", "read", "--input"])
        .arg(&plan_path)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        read.status.success(),
        "{}",
        String::from_utf8_lossy(&read.stderr)
    );
    let read_plan: RemediationPlan = serde_json::from_slice(&read.stdout).unwrap();
    assert_eq!(read_plan.actions.len(), 1);
    assert_eq!(
        read_plan.actions[0].relevant_paths,
        ["lib/dnsmsg-parser/src/dns_message.rs"]
    );
    assert_eq!(read_plan_from_file(&plan_path).unwrap(), read_plan);
    let aliased_plan_path = out_dir.join("aliased-plan.json");
    fs::copy(&plan_path, &aliased_plan_path).unwrap();
    let alias_error = read_plan_from_file(&aliased_plan_path)
        .unwrap_err()
        .to_string();
    assert!(alias_error.contains("remediation_plan_reference_mismatch"));
    let source_after = ["synthesis.json", "manifest.json", "review-record.json"]
        .map(|name| fs::read(bundle.join(name)).unwrap());
    assert_eq!(source_before, source_after);

    let duplicate = run_remediation(&input, &out_dir);
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr)
        .contains("remediation_output_directory_already_exists"));
}

#[test]
fn installed_cli_rejects_bare_synthetic_synthesis() {
    let root = temp_dir("bare-synthesis");
    let input = root.join("synthesis.json");
    let (synthesis, _) = synthesis();
    fs::write(&input, serde_json::to_vec_pretty(&synthesis).unwrap()).unwrap();

    let output = run_remediation(&input, &root.join("out"));
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("synthesis_manifest_missing_or_invalid")
    );
}

#[test]
fn remediation_reader_rejects_unrelated_action_substitution() {
    let root = temp_dir("unrelated-action");
    let input = completed_synthesis_bundle().join("synthesis.json");
    let out_dir = root.join("out");
    let output = run_remediation(&input, &out_dir);
    assert!(output.status.success());

    let plan_path = out_dir.join("remediation-plan.json");
    let mut plan: RemediationPlan = serde_json::from_slice(&fs::read(&plan_path).unwrap()).unwrap();
    plan.actions[0].relevant_paths = vec!["Cargo.toml".to_string()];
    fs::write(&plan_path, serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
    let manifest_path = out_dir.join("manifest.json");
    let mut manifest: RemediationManifest =
        serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    manifest.remediation_plan_digest = hash(&plan).unwrap();
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();

    let error = read_plan_from_file(&plan_path).unwrap_err().to_string();
    assert!(error.contains("remediation_plan_not_canonical_for_synthesis"));
}

#[test]
fn remediation_reader_rejects_missing_evidence_and_tampered_bundle() {
    let root = temp_dir("tampered-bundle");
    let input = completed_synthesis_bundle().join("synthesis.json");
    let out_dir = root.join("out");
    let output = run_remediation(&input, &out_dir);
    assert!(output.status.success());

    let plan_path = out_dir.join("remediation-plan.json");
    let mut plan: RemediationPlan = serde_json::from_slice(&fs::read(&plan_path).unwrap()).unwrap();
    plan.actions[0].evidence_ids.clear();
    fs::write(&plan_path, serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
    let manifest_path = out_dir.join("manifest.json");
    let mut manifest: RemediationManifest =
        serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    manifest.remediation_plan_digest = hash(&plan).unwrap();
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    let error = read_plan_from_file(&plan_path).unwrap_err().to_string();
    assert!(error.contains("untraceable_remediation_action"));

    let output_dir = root.join("fresh-out");
    let copied_bundle = root.join("bundle");
    fs::create_dir(&copied_bundle).unwrap();
    for name in ["synthesis.json", "manifest.json", "review-record.json"] {
        fs::copy(
            completed_synthesis_bundle().join(name),
            copied_bundle.join(name),
        )
        .unwrap();
    }
    let synthesis_path = copied_bundle.join("synthesis.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&synthesis_path).unwrap()).unwrap();
    value["revision"] = serde_json::Value::String("tampered".to_string());
    fs::write(&synthesis_path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let output = run_remediation(&synthesis_path, &output_dir);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("synthesis_bundle_digest_or_count_mismatch"));
}
