//! PVF lane: runtime. Deterministic local CPU/filesystem proof for CodeFriend
//! test planning. No provider calls, source mutation, network, or issue
//! publication.

use adl::codefriend::{
    actions::test_plan::{
        plan, plan_from_file, plan_from_store, plan_with_record, read_plan_from_file,
        read_plan_from_store, validate_plan, TestPlan, TestPlanOptions, TEST_PLAN_SCHEMA_V2,
    },
    evidence::contracts::{Confidence, ReviewRecord, Severity},
    review::synthesis::{ReviewSynthesis, SynthesisSource, SynthesizedFinding, SYNTHESIS_SCHEMA},
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn temp_dir(name: &str) -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "target/codefriend-testplan-tests/{name}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn predecessor_bundle() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../.csdlc/evidence/892/predecessor-openai-r5-synthesis")
}

fn copy_bundle(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for name in ["synthesis.json", "manifest.json", "review-record.json"] {
        fs::copy(source.join(name), destination.join(name)).unwrap();
    }
}

fn tree_inventory(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        let mut entries = fs::read_dir(current)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

fn source(id: &str, evidence: &[&str]) -> SynthesisSource {
    SynthesisSource {
        finding_id: id.to_string(),
        perspective: "correctness".to_string(),
        rule: "bounded-rule".to_string(),
        severity: Severity::High,
        evidence: evidence.iter().map(|value| value.to_string()).collect(),
        rationale: "rationale retained from lane".to_string(),
        confidence: Confidence::Known(90),
        inference: "observed from admitted evidence".to_string(),
        limitations: vec![],
    }
}

fn finding(id: &str, anchor: &str, evidence: &[&str]) -> SynthesizedFinding {
    SynthesizedFinding {
        id: id.to_string(),
        semantic_anchor: anchor.to_string(),
        title: format!("finding {id}"),
        severity: Severity::High,
        severity_rationale: "correctness: rationale retained".to_string(),
        evidence: evidence.iter().map(|value| value.to_string()).collect(),
        sources: vec![source(&format!("source-{id}"), evidence)],
        disagreement: None,
        scope_limits: vec!["correctness: bounded fixture only".to_string()],
    }
}

fn synthesis() -> ReviewSynthesis {
    ReviewSynthesis {
        schema: SYNTHESIS_SCHEMA.to_string(),
        coverage: None,
        assessment_coverage: None,
        assessment_counts: None,
        observations: None,
        review_record_digest: "review-digest".to_string(),
        run_id: "test-plan-run".to_string(),
        repository: "https://example.com/team/test-plan-target".to_string(),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        scope_digest: "scope-digest".to_string(),
        lane_count: 4,
        input_finding_count: 2,
        synthesized_findings: vec![
            finding(
                "finding-a",
                "adl/src/codefriend/review/runner.rs:85",
                &["adl/src/codefriend/review/runner.rs:85", "evidence-a"],
            ),
            finding(
                "finding-b",
                "adl/src/cli/codefriend_cmd.rs:112",
                &["adl/src/cli/codefriend_cmd.rs:112", "evidence-b"],
            ),
        ],
    }
}

#[test]
fn test_plan_maps_every_traceable_finding_to_executable_case() {
    let plan = plan(&synthesis()).unwrap();
    assert_eq!(plan.schema, "codefriend.test_plan.v1");
    assert_eq!(plan.test_cases.len(), 2);
    assert!(plan.omitted_findings.is_empty());
    assert!(plan.test_cases.iter().all(|case| {
        case.proposed_test_location.contains("test")
            && case.validation_lane.contains("PVF")
            && case.resource_profile.contains("no provider credentials")
            && case.detection_rationale.contains(&case.finding_id)
            && !case.proposed_fixture.contains("TODO")
    }));
    assert!(plan
        .test_cases
        .iter()
        .any(|case| case.proposed_test_location == "adl/tests/codefriend_cli_regression.rs"));
}

#[test]
fn test_plan_omits_findings_without_repository_path() {
    let mut synthesis = synthesis();
    synthesis.synthesized_findings.push(finding(
        "finding-c",
        "repository-wide concern without path",
        &["evidence-c"],
    ));
    let plan = plan(&synthesis).unwrap();
    assert_eq!(plan.test_cases.len(), 2);
    assert_eq!(plan.omitted_findings.len(), 1);
    assert_eq!(
        plan.omitted_findings[0].reason,
        "no_supported_repository_path_for_test_location"
    );
}

#[test]
fn test_plan_preserves_dot_directories_and_root_files() {
    let synthesis = ReviewSynthesis {
        synthesized_findings: vec![
            finding(
                "finding-ci",
                ".github/workflows/ci.yml:42",
                &[".github/workflows/ci.yml:42"],
            ),
            finding("finding-root", "Cargo.toml", &["Cargo.toml"]),
        ],
        ..synthesis()
    };

    let plan = plan(&synthesis).unwrap();
    let source_paths = plan
        .test_cases
        .iter()
        .flat_map(|case| case.source_evidence.iter().map(String::as_str))
        .collect::<Vec<_>>();
    assert!(source_paths.contains(&".github/workflows/ci.yml:42"));
    assert!(source_paths.contains(&"Cargo.toml"));
    assert!(!source_paths.contains(&"github/workflows/ci.yml"));
    assert!(plan.omitted_findings.is_empty());
}

#[test]
fn test_plan_consumes_tracked_predecessor_synthesis_with_concrete_mapping() {
    let fixture = predecessor_bundle().join("synthesis.json");
    let synthesis: ReviewSynthesis =
        serde_json::from_slice(&fs::read(&fixture).expect("tracked predecessor synthesis"))
            .expect("valid predecessor synthesis");

    let plan = plan(&synthesis).unwrap();

    assert_eq!(plan.test_cases.len(), 1);
    assert!(plan.omitted_findings.is_empty());
    let case = &plan.test_cases[0];
    assert_eq!(
        case.proposed_test_location,
        "lib/dnsmsg-parser/src/dns_message_parser.rs"
    );
    assert!(case
        .behavior_under_test
        .contains("get_rdata_decoder_with_raw_message"));
    assert!(case
        .behavior_under_test
        .contains("raw_message_for_rdata_parsing"));
    assert!(case
        .proposed_fixture
        .contains("test_compressed_rdata_buffer_is_replaced_between_calls"));
    assert!(case.proposed_fixture.contains("BGZyZWTAEwNqb2XAEw=="));
    assert!(case.proposed_fixture.contains("alice\\x07example"));
    assert!(case
        .expected_pre_fix_failure
        .contains("alice.example.com. bob.example.com."));
    assert!(case
        .expected_post_fix_assertion
        .contains("second_rdata.len()"));
    assert!(case
        .detection_rationale
        .contains("same parser, multiple decode calls"));
    assert!(!case
        .proposed_fixture
        .contains("Construct the smallest fixture"));
}

#[test]
fn test_plan_reader_rejects_placeholder_untraceable_or_non_test_cases() {
    let plan = plan(&synthesis()).unwrap();

    let mut bad_location = plan.clone();
    bad_location.test_cases[0].proposed_test_location =
        "adl/src/codefriend/review/runner.rs".to_string();
    let err = validate_plan(&bad_location).unwrap_err().to_string();
    assert!(err.contains("test_plan_location_must_be_test_surface"));

    let mut bad_rationale = plan.clone();
    bad_rationale.test_cases[0].detection_rationale = "detects behavior".to_string();
    let err = validate_plan(&bad_rationale).unwrap_err().to_string();
    assert!(err.contains("untraceable_test_detection_rationale"));

    let mut placeholder = plan;
    placeholder.test_cases[0].proposed_fixture = "TODO".to_string();
    let err = validate_plan(&placeholder).unwrap_err().to_string();
    assert!(err.contains("placeholder_test_plan"));
}

#[test]
fn installed_cli_generates_and_reads_test_plan_without_source_mutation() {
    let root = temp_dir("installed-cli");
    let (record, store, _clock, bundle) = admitted_fixture(&root);
    let store_path = store.root_path().to_path_buf();
    drop(store);
    let input = bundle.join("synthesis.json");
    let out_dir = root.join("test-plan-out");
    let source_root = root.join("inspected-repository");
    for object in &record.admission.packet.objects {
        if let Some(content) = &object.content {
            let path = source_root.join(&object.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, content).unwrap();
        }
    }
    fs::create_dir_all(source_root.join(".git-marker")).unwrap();
    fs::write(source_root.join(".git-marker/HEAD"), b"fixture-head\n").unwrap();
    let before = tree_inventory(&source_root);

    let output = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "--store"])
        .arg(&store_path)
        .arg("--input")
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["schema"], TEST_PLAN_SCHEMA_V2);
    assert_eq!(summary["test_case_count"], 1);
    assert!(out_dir.join("synthesis.json").exists());
    assert!(out_dir.join("synthesis-manifest.json").exists());
    assert!(out_dir.join("review-record.json").exists());
    assert!(out_dir.join("manifest.json").exists());
    let plan_path = out_dir.join("test-plan.json");
    assert!(plan_path.exists());

    let read = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "read", "--store"])
        .arg(&store_path)
        .arg("--input")
        .arg(&plan_path)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(
        read.status.success(),
        "{}",
        String::from_utf8_lossy(&read.stderr)
    );
    let read_plan: TestPlan = serde_json::from_slice(&read.stdout).unwrap();
    assert_eq!(read_plan.test_cases.len(), 1);
    let store = adl::codefriend::evidence::store::Store::open(&store_path, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })
    .unwrap();
    assert_eq!(read_plan_from_store(&plan_path, &store).unwrap(), read_plan);
    assert!(read_plan_from_file(&plan_path).is_err());
    drop(store);
    assert_eq!(tree_inventory(&source_root), before);

    let duplicate = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(["codefriend", "plan", "tests", "--store"])
        .arg(&store_path)
        .arg("--input")
        .arg(&input)
        .arg("--out")
        .arg(&out_dir)
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .output()
        .unwrap();
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr)
        .contains("test_plan_output_directory_already_exists"));
}

#[test]
fn generator_rejects_standalone_or_tampered_synthesis_bundle() {
    let root = temp_dir("synthesis-authority");
    let standalone = root.join("standalone");
    fs::create_dir_all(&standalone).unwrap();
    fs::copy(
        predecessor_bundle().join("synthesis.json"),
        standalone.join("synthesis.json"),
    )
    .unwrap();
    let error = plan_from_file(TestPlanOptions {
        input: standalone.join("synthesis.json"),
        out: root.join("standalone-out"),
    })
    .unwrap_err()
    .to_string();
    assert!(error.contains("manifest.json"));

    let tampered = root.join("tampered");
    copy_bundle(&predecessor_bundle(), &tampered);
    let synthesis_path = tampered.join("synthesis.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&synthesis_path).unwrap()).unwrap();
    value["repository"] = serde_json::Value::String("https://example.com/forged".to_string());
    fs::write(&synthesis_path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let error = plan_from_file(TestPlanOptions {
        input: synthesis_path,
        out: root.join("tampered-out"),
    })
    .unwrap_err()
    .to_string();
    assert!(error.contains("untrusted_or_inconsistent_synthesis_bundle"));
}

#[test]
fn reader_rejects_plan_manifest_synthesis_and_partition_tampering() {
    let root = temp_dir("reader-integrity");
    let good = root.join("good");
    plan_from_file(TestPlanOptions {
        input: predecessor_bundle().join("synthesis.json"),
        out: good.clone(),
    })
    .unwrap();
    assert!(read_plan_from_file(&good.join("test-plan.json")).is_ok());

    let plan_tamper = root.join("plan-tamper");
    copy_dir(&good, &plan_tamper);
    let plan_path = plan_tamper.join("test-plan.json");
    let mut plan: TestPlan = serde_json::from_slice(&fs::read(&plan_path).unwrap()).unwrap();
    plan.test_cases.clear();
    fs::write(&plan_path, serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
    let error = read_plan_from_file(&plan_path).unwrap_err().to_string();
    assert!(error.contains("test_plan_bundle_digest_or_count_mismatch"));

    let manifest_tamper = root.join("manifest-tamper");
    copy_dir(&good, &manifest_tamper);
    let manifest_path = manifest_tamper.join("manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
    manifest["test_case_count"] = serde_json::json!(99);
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    let error = read_plan_from_file(&manifest_tamper.join("test-plan.json"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("test_plan_bundle_digest_or_count_mismatch"));

    let synthesis_tamper = root.join("synthesis-tamper");
    copy_dir(&good, &synthesis_tamper);
    let synthesis_path = synthesis_tamper.join("synthesis.json");
    let mut synthesis: ReviewSynthesis =
        serde_json::from_slice(&fs::read(&synthesis_path).unwrap()).unwrap();
    synthesis.synthesized_findings.clear();
    fs::write(
        &synthesis_path,
        serde_json::to_vec_pretty(&synthesis).unwrap(),
    )
    .unwrap();
    let error = read_plan_from_file(&synthesis_tamper.join("test-plan.json"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("untrusted_or_inconsistent_synthesis_bundle"));
}

fn copy_dir(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
    }
}

// PVF runtime: deterministic Admission/Store mapping regression, local filesystem,
// no models, network or source execution; required pre-publication proof.
fn admitted_fixture(
    root: &Path,
) -> (
    ReviewRecord,
    adl::codefriend::evidence::store::Store,
    std::rc::Rc<std::cell::Cell<u64>>,
    PathBuf,
) {
    use adl::codefriend::{
        evidence::{contracts::Run, store::Store, Retention},
        review::synthesis::{synthesize_from_file, SynthesisOptions},
    };
    let mut record: ReviewRecord =
        serde_json::from_slice(&fs::read(predecessor_bundle().join("review-record.json")).unwrap())
            .unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let clock = std::rc::Rc::new(std::cell::Cell::new(now));
    let c = clock.clone();
    let store = Store::open(&root.join("original-store"), move || c.get()).unwrap();
    record.admission = store
        .admit(record.admission.packet.clone(), Retention { seconds: 3600 })
        .unwrap();
    record.run = Run::new(
        &record.admission,
        record.run.lane_versions.clone(),
        record.run.provider_route.clone(),
        record.run.completion.clone(),
        record.run.failures.clone(),
    )
    .unwrap();
    for (i, finding) in record.findings.iter_mut().enumerate() {
        finding.semantic_anchor = format!("opaque-behavior-{i}");
        finding.id = finding.identity().unwrap();
    }
    record.validate().unwrap();
    let input = root.join("review-record.json");
    fs::write(&input, serde_json::to_vec(&record).unwrap()).unwrap();
    let bundle = root.join("synthesis");
    synthesize_from_file(SynthesisOptions {
        input,
        out: bundle.clone(),
    })
    .unwrap();
    (record, store, clock, bundle)
}

#[test]
fn admitted_ids_with_opaque_anchors_produce_source_bound_v2_without_fixture_guessing() {
    let root = temp_dir("admitted-id-only");
    let (record, _store, _clock, _) = admitted_fixture(&root);
    let synthesis = adl::codefriend::review::synthesis::synthesize(&record).unwrap();
    let result = plan_with_record(&synthesis, &record).unwrap();
    assert_eq!(result.schema, TEST_PLAN_SCHEMA_V2);
    assert!(!result.test_cases.is_empty());
    for case in &result.test_cases {
        assert!(case.source_evidence.iter().all(|id| record
            .admission
            .evidence
            .iter()
            .any(|e| &e.id == id)));
        assert!(record
            .admission
            .evidence
            .iter()
            .any(|e| case.behavior_under_test.contains(&e.path)));
        assert!(!case.proposed_fixture.contains("BGZyZWTAEwNqb2XAEw=="));
        assert!(case
            .scope_limits
            .iter()
            .any(|s| s.contains("privacy-omitted")));
    }
    assert_eq!(result, plan_with_record(&synthesis, &record).unwrap());
}

#[test]
fn admitted_mapping_rejects_unknown_duplicate_or_mismatched_identity() {
    let root = temp_dir("admitted-invalid-ids");
    let (record, _store, _clock, _) = admitted_fixture(&root);
    let synthesis = adl::codefriend::review::synthesis::synthesize(&record).unwrap();
    let mut unknown = record.clone();
    unknown.findings[0].evidence = vec!["0".repeat(64)];
    assert!(plan_with_record(&synthesis, &unknown).is_err());
    let mut duplicate = record.clone();
    duplicate
        .admission
        .evidence
        .push(duplicate.admission.evidence[0].clone());
    assert!(plan_with_record(&synthesis, &duplicate).is_err());
    let mut duplicate_ref = record.clone();
    let repeated = duplicate_ref.findings[0].evidence[0].clone();
    duplicate_ref.findings[0].evidence.push(repeated);
    assert!(plan_with_record(&synthesis, &duplicate_ref).is_err());
    let mut wrong = synthesis.clone();
    wrong.revision = "0".repeat(40);
    assert!(plan_with_record(&wrong, &record).is_err());
}

#[test]
fn admitted_v2_file_owner_requires_original_store_and_rejects_deletion_expiry_and_tamper() {
    let root = temp_dir("admitted-owner");
    let (record, store, clock, bundle) = admitted_fixture(&root);
    let out = root.join("tests");
    let result = plan_from_store(
        TestPlanOptions {
            input: bundle.join("synthesis.json"),
            out: out.clone(),
        },
        &store,
    )
    .unwrap();
    let path = out.join("test-plan.json");
    assert_eq!(read_plan_from_store(&path, &store).unwrap(), result);
    assert!(read_plan_from_file(&path).is_err());
    let mut tampered = result.clone();
    tampered.test_cases[0].proposed_test_location = "tests/invented.rs".into();
    fs::write(&path, serde_json::to_vec(&tampered).unwrap()).unwrap();
    assert!(read_plan_from_store(&path, &store).is_err());
    fs::write(&path, serde_json::to_vec(&result).unwrap()).unwrap();
    clock.set(record.admission.expires_at);
    assert!(read_plan_from_store(&path, &store).is_err());
    let rejected = root.join("expired-output");
    assert!(plan_from_store(
        TestPlanOptions {
            input: bundle.join("synthesis.json"),
            out: rejected.clone()
        },
        &store
    )
    .is_err());
    assert!(!rejected.exists());
    let root2 = temp_dir("admitted-deleted");
    let (r2, s2, _, b2) = admitted_fixture(&root2);
    s2.delete(&r2.admission.packet.packet_id).unwrap();
    assert!(plan_from_store(
        TestPlanOptions {
            input: b2.join("synthesis.json"),
            out: root2.join("tests")
        },
        &s2
    )
    .is_err());
}

#[test]
fn admitted_mixed_privacy_and_unsupported_sources_preserve_real_coverage() {
    use adl::codefriend::{
        evidence::{
            contracts::{Completion, ReviewCoverage, Run},
            Admission, Retention,
        },
        ingestion::{local, Scope},
        review::{lanes::LANE_CONTRACT_VERSION, synthesis::synthesize},
    };
    let root = temp_dir("mixed-privacy");
    let checkout = root.join("repo");
    fs::create_dir(&checkout).unwrap();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&checkout)
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    };
    git(&["init"]);
    git(&[
        "remote",
        "add",
        "origin",
        "https://example.com/test/planner",
    ]);
    fs::write(
        checkout.join("safe.rs"),
        "pub fn divide(a: u32, b: u32) -> u32 { a / b }\n",
    )
    .unwrap();
    fs::write(checkout.join("README.md"), "Source documentation\n").unwrap();
    fs::write(
        checkout.join("private.txt"),
        "password = \"private-fixture-value\"",
    )
    .unwrap();
    git(&["add", "."]);
    git(&[
        "-c",
        "user.name=fixture",
        "-c",
        "user.email=fixture@example.com",
        "commit",
        "-m",
        "fixture",
    ]);
    let revision = git(&["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &checkout,
        "https://example.com/test/planner",
        &revision,
        Scope {
            analysis: vec!["safe.rs".into()],
            context: vec!["README.md".into(), "private.txt".into()],
            max_files: 3,
            max_bytes: 4096,
            max_file_bytes: 4096,
        },
    )
    .unwrap();
    let admission = Admission::new(packet, Retention { seconds: 100 }, 100).unwrap();
    assert!(admission.evidence.iter().all(|e| e.path != "private.txt"));
    let lanes: BTreeMap<String, String> =
        ["adversarial", "constitutional", "correctness", "security"]
            .into_iter()
            .map(|s| (s.into(), LANE_CONTRACT_VERSION.into()))
            .collect();
    let coverage = ReviewCoverage::new(
        &admission,
        lanes.keys().map(|s| (s.clone(), "a".repeat(64))).collect(),
    )
    .unwrap();
    let run = Run::new(
        &admission,
        lanes,
        "fixture:mock:reviewer".into(),
        Completion::Incomplete,
        vec![],
    )
    .unwrap()
    .with_review_coverage(&admission, coverage.clone())
    .unwrap();
    let predecessor: ReviewRecord =
        serde_json::from_slice(&fs::read(predecessor_bundle().join("review-record.json")).unwrap())
            .unwrap();
    let mut supported = predecessor.findings[0].clone();
    supported.repository = run.repository.clone();
    supported.scope_digest = run.scope_digest.clone();
    supported.perspective = "correctness".into();
    supported.semantic_anchor = "opaque-supported".into();
    supported.evidence = admission.evidence.iter().map(|e| e.id.clone()).collect();
    supported.evidence.sort();
    supported.id = supported.identity().unwrap();
    let mut unsupported = supported.clone();
    unsupported.semantic_anchor = "opaque-docs-only".into();
    unsupported.evidence = vec![admission
        .evidence
        .iter()
        .find(|e| e.path == "README.md")
        .unwrap()
        .id
        .clone()];
    unsupported.id = unsupported.identity().unwrap();
    let record = ReviewRecord {
        admission,
        run,
        findings: vec![supported.clone(), unsupported],
    };
    let synthesis = synthesize(&record).unwrap();
    let result = plan_with_record(&synthesis, &record).unwrap();
    assert_eq!(result.test_cases.len(), 1);
    assert_eq!(result.omitted_findings.len(), 1);
    assert_eq!(result.coverage, Some(coverage));
    assert!(result.test_cases[0].behavior_under_test.contains("safe.rs"));
    let encoded = serde_json::to_string(&result).unwrap();
    assert!(encoded.contains("privacy_filter"));
    assert!(!encoded.contains("private-fixture-value"));
    let mut repeated = record.clone();
    supported.perspective = "security".into();
    supported.id = supported.identity().unwrap();
    repeated.findings.push(supported);
    let joined = plan_with_record(&synthesize(&repeated).unwrap(), &repeated).unwrap();
    assert!(joined
        .test_cases
        .iter()
        .all(|c| c.source_evidence.windows(2).all(|w| w[0] < w[1])));
    let mut erased = result.clone();
    erased.coverage = None;
    assert_ne!(erased, plan_with_record(&synthesis, &record).unwrap());
}
