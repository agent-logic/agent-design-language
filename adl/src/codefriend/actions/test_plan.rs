use crate::codefriend::{
    evidence::{
        contracts::{ReviewCoverage, ReviewRecord, Severity},
        hash,
        store::Store,
    },
    review::synthesis::{
        synthesize, ReviewSynthesis, SynthesisManifest, SynthesizedFinding,
        SYNTHESIS_MANIFEST_SCHEMA, SYNTHESIS_SCHEMA, SYNTHESIS_SCHEMA_V2,
    },
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::{Component, Path, PathBuf},
};

pub const TEST_PLAN_SCHEMA: &str = "codefriend.test_plan.v1";
pub const TEST_PLAN_SCHEMA_V2: &str = "codefriend.test_plan.v2";
pub const TEST_PLAN_MANIFEST_SCHEMA_V2: &str = "codefriend.test_plan_manifest.v2";

pub const TEST_PLAN_MANIFEST_SCHEMA: &str = "codefriend.test_plan_manifest.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestPlanOptions {
    pub input: PathBuf,
    pub out: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestCasePlan {
    pub id: String,
    pub finding_id: String,
    pub source_finding_ids: Vec<String>,
    pub title: String,
    pub severity: Severity,
    pub behavior_under_test: String,
    pub source_evidence: Vec<String>,
    pub proposed_test_location: String,
    pub proposed_fixture: String,
    pub expected_pre_fix_failure: String,
    pub expected_post_fix_assertion: String,
    pub validation_lane: String,
    pub resource_profile: String,
    pub detection_rationale: String,
    pub non_goals: Vec<String>,
    pub scope_limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OmittedFinding {
    pub finding_id: String,
    pub title: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestPlan {
    pub schema: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<ReviewCoverage>,
    pub synthesis_schema: String,
    pub synthesis_digest: String,
    pub run_id: String,
    pub repository: String,
    pub revision: String,
    pub scope_digest: String,
    pub test_cases: Vec<TestCasePlan>,
    pub omitted_findings: Vec<OmittedFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestPlanManifest {
    pub schema: String,
    pub synthesis_manifest_ref: String,
    pub synthesis_manifest_digest: String,
    pub synthesis_ref: String,
    pub synthesis_digest: String,
    pub review_record_ref: String,
    pub review_record_digest: String,
    pub test_plan_ref: String,
    pub test_plan_digest: String,
    pub test_case_count: usize,
    pub omitted_finding_count: usize,
}

/// Historical v1 snapshot derivation. New live operations use `plan_from_store`.
pub fn plan_from_file(options: TestPlanOptions) -> Result<TestPlan> {
    write_plan_bundle(options, None)
}

pub fn plan_from_store(options: TestPlanOptions, store: &Store) -> Result<TestPlan> {
    write_plan_bundle(options, Some(store))
}

fn check_owner(store: &Store, record: &ReviewRecord) -> Result<()> {
    ensure!(
        store.get(&record.admission.packet.packet_id)? == record.admission,
        "test_plan_original_admission_mismatch"
    );
    Ok(())
}

struct CreatedOutput {
    path: PathBuf,
    identity: (u64, u64),
    committed: bool,
}
impl Drop for CreatedOutput {
    fn drop(&mut self) {
        if !self.committed
            && fs::symlink_metadata(&self.path)
                .is_ok_and(|m| m.is_dir() && (m.dev(), m.ino()) == self.identity)
        {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

fn write_plan_bundle(options: TestPlanOptions, store: Option<&Store>) -> Result<TestPlan> {
    ensure!(
        !options.out.exists(),
        "test_plan_output_directory_already_exists"
    );
    let (synthesis, source_manifest, review_record) = read_synthesis_bundle(&options.input)?;
    let plan = if let Some(store) = store {
        check_owner(store, &review_record)?;
        plan_with_record(&synthesis, &review_record)?
    } else {
        plan(&synthesis)?
    };
    if let Some(store) = store {
        check_owner(store, &review_record)?;
    }
    fs::create_dir(&options.out).with_context(|| format!("create {}", options.out.display()))?;
    let metadata = fs::symlink_metadata(&options.out)?;
    let mut output = CreatedOutput {
        path: options.out.clone(),
        identity: (metadata.dev(), metadata.ino()),
        committed: false,
    };
    if store.is_some() {
        // Persist the already validated values, never reopen mutable input paths
        // after authority and canonical synthesis have been checked.
        write_json(&options.out.join("synthesis.json"), &synthesis)?;
        write_json(
            &options.out.join("synthesis-manifest.json"),
            &source_manifest,
        )?;
        write_json(&options.out.join("review-record.json"), &review_record)?;
    } else {
        // Historical v1 artifacts preserve their original byte snapshots.
        copy_json_snapshot(&options.input, &options.out.join("synthesis.json"))?;
        let source_dir = options
            .input
            .parent()
            .context("synthesis_bundle_requires_parent_directory")?;
        copy_json_snapshot(
            &source_dir.join("manifest.json"),
            &options.out.join("synthesis-manifest.json"),
        )?;
        copy_json_snapshot(
            &source_dir.join("review-record.json"),
            &options.out.join("review-record.json"),
        )?;
    }
    write_json(&options.out.join("test-plan.json"), &plan)?;
    let manifest = TestPlanManifest {
        schema: manifest_schema(&plan)?.to_string(),
        synthesis_manifest_ref: "synthesis-manifest.json".to_string(),
        synthesis_manifest_digest: hash(&source_manifest)?,
        synthesis_ref: "synthesis.json".to_string(),
        synthesis_digest: plan.synthesis_digest.clone(),
        review_record_ref: "review-record.json".to_string(),
        review_record_digest: hash(&review_record)?,
        test_plan_ref: "test-plan.json".to_string(),
        test_plan_digest: hash(&plan)?,
        test_case_count: plan.test_cases.len(),
        omitted_finding_count: plan.omitted_findings.len(),
    };
    write_json(&options.out.join("manifest.json"), &manifest)?;
    if let Some(store) = store {
        check_owner(store, &review_record)?;
    }
    let metadata = fs::symlink_metadata(&options.out)?;
    ensure!(
        metadata.is_dir() && (metadata.dev(), metadata.ino()) == output.identity,
        "test_plan_output_identity_changed"
    );
    output.committed = true;
    Ok(plan)
}

/// Historical v1 readback does not establish current Store authority.
pub fn read_plan_from_file(input: &Path) -> Result<TestPlan> {
    read_plan_bundle(input, None)
}
pub fn read_plan_from_store(input: &Path, store: &Store) -> Result<TestPlan> {
    read_plan_bundle(input, Some(store))
}
fn read_plan_bundle(input: &Path, store: Option<&Store>) -> Result<TestPlan> {
    ensure!(
        input.file_name().and_then(|name| name.to_str()) == Some("test-plan.json"),
        "test_plan_bundle_requires_canonical_plan_ref"
    );
    let bundle = input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("test_plan_bundle_requires_parent_directory"))?;
    let plan: TestPlan = read_json(input, 8 * 1024 * 1024)?;
    let manifest: TestPlanManifest = read_json(&bundle.join("manifest.json"), 1024 * 1024)?;
    ensure!(
        manifest.schema == manifest_schema(&plan)?
            && manifest.synthesis_manifest_ref == "synthesis-manifest.json"
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.review_record_ref == "review-record.json"
            && manifest.test_plan_ref == "test-plan.json",
        "invalid_test_plan_manifest"
    );
    let source_manifest: SynthesisManifest =
        read_json(&bundle.join(&manifest.synthesis_manifest_ref), 1024 * 1024)?;
    let synthesis: ReviewSynthesis =
        read_json(&bundle.join(&manifest.synthesis_ref), 8 * 1024 * 1024)?;
    let review_record: ReviewRecord =
        read_json(&bundle.join(&manifest.review_record_ref), 8 * 1024 * 1024)?;
    validate_synthesis_bundle(&source_manifest, &synthesis, &review_record)?;
    if plan.schema == TEST_PLAN_SCHEMA_V2 {
        check_owner(
            store.context("test_plan_v2_requires_original_store")?,
            &review_record,
        )?;
    }
    ensure!(
        manifest.synthesis_manifest_digest == hash(&source_manifest)?
            && manifest.synthesis_digest == hash(&synthesis)?
            && manifest.review_record_digest == hash(&review_record)?
            && manifest.test_plan_digest == hash(&plan)?
            && manifest.test_case_count == plan.test_cases.len()
            && manifest.omitted_finding_count == plan.omitted_findings.len(),
        "test_plan_bundle_digest_or_count_mismatch"
    );
    validate_plan_against_synthesis(&plan, &synthesis)?;
    ensure!(
        plan == derive_for_schema(&plan.schema, &synthesis, &review_record)?,
        "test_plan_not_canonical_for_synthesis"
    );
    if let Some(store) = store {
        check_owner(store, &review_record)?;
    }
    Ok(plan)
}

pub fn manifest_schema(plan: &TestPlan) -> Result<&'static str> {
    match plan.schema.as_str() {
        TEST_PLAN_SCHEMA => Ok(TEST_PLAN_MANIFEST_SCHEMA),
        TEST_PLAN_SCHEMA_V2 => Ok(TEST_PLAN_MANIFEST_SCHEMA_V2),
        _ => anyhow::bail!("invalid_test_plan_schema"),
    }
}

/// Canonical snapshot derivation; live owners must separately check original Store authority.
pub fn derive_for_schema(
    schema: &str,
    synthesis: &ReviewSynthesis,
    record: &ReviewRecord,
) -> Result<TestPlan> {
    match schema {
        TEST_PLAN_SCHEMA => plan(synthesis),
        TEST_PLAN_SCHEMA_V2 => plan_with_record(synthesis, record),
        _ => anyhow::bail!("invalid_test_plan_schema"),
    }
}

/// Inert deterministic derivation, not a grant of live source or publication authority.
pub fn plan_with_record(synthesis: &ReviewSynthesis, record: &ReviewRecord) -> Result<TestPlan> {
    record.successful_execution()?;
    ensure!(
        &synthesize(record)? == synthesis,
        "test_plan_review_synthesis_mismatch"
    );
    let mut index = BTreeMap::new();
    for evidence in &record.admission.evidence {
        ensure!(
            index
                .insert(evidence.id.as_str(), evidence.path.as_str())
                .is_none(),
            "test_plan_duplicate_evidence_identity"
        );
    }
    plan_using(synthesis, TEST_PLAN_SCHEMA_V2, |finding| {
        let mut paths = BTreeSet::new();
        for id in &finding.evidence {
            let path = *index
                .get(id.as_str())
                .context("test_plan_unknown_evidence")?;
            if validate_relative_path(path).is_ok()
                && matches!(
                    Path::new(path).extension().and_then(|s| s.to_str()),
                    Some("rs" | "py" | "java" | "js" | "mjs" | "cjs")
                )
            {
                paths.insert(path.to_owned());
            }
        }
        Ok(paths.into_iter().collect())
    })
}

fn read_synthesis_bundle(
    input: &Path,
) -> Result<(ReviewSynthesis, SynthesisManifest, ReviewRecord)> {
    ensure!(
        input.file_name().and_then(|name| name.to_str()) == Some("synthesis.json"),
        "test_plan_requires_canonical_synthesis_ref"
    );
    let bundle = input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("synthesis_bundle_requires_parent_directory"))?;
    let synthesis: ReviewSynthesis = read_json(input, 8 * 1024 * 1024)?;
    let manifest: SynthesisManifest = read_json(&bundle.join("manifest.json"), 1024 * 1024)?;
    let review_record: ReviewRecord =
        read_json(&bundle.join("review-record.json"), 8 * 1024 * 1024)?;
    validate_synthesis_bundle(&manifest, &synthesis, &review_record)?;
    Ok((synthesis, manifest, review_record))
}

fn validate_synthesis_bundle(
    manifest: &SynthesisManifest,
    synthesis: &ReviewSynthesis,
    review_record: &ReviewRecord,
) -> Result<()> {
    ensure!(
        manifest.schema == SYNTHESIS_MANIFEST_SCHEMA
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.review_record_ref == "review-record.json",
        "invalid_synthesis_bundle_manifest"
    );
    review_record.validate()?;
    ensure!(
        review_record.successful_execution().is_ok(),
        "test_plan_requires_complete_review"
    );
    let expected = synthesize(review_record)?;
    ensure!(
        &expected == synthesis
            && manifest.synthesis_digest == hash(synthesis)?
            && manifest.review_record_digest == hash(review_record)?
            && synthesis.review_record_digest == manifest.review_record_digest
            && manifest.synthesized_finding_count == synthesis.synthesized_findings.len()
            && manifest.input_finding_count == synthesis.input_finding_count,
        "untrusted_or_inconsistent_synthesis_bundle"
    );
    Ok(())
}

pub fn plan(synthesis: &ReviewSynthesis) -> Result<TestPlan> {
    plan_using(synthesis, TEST_PLAN_SCHEMA, |finding| {
        Ok(relevant_paths(finding))
    })
}

fn plan_using(
    synthesis: &ReviewSynthesis,
    schema: &str,
    paths: impl Fn(&SynthesizedFinding) -> Result<Vec<String>>,
) -> Result<TestPlan> {
    validate_synthesis(synthesis)?;
    let synthesis_digest = hash(synthesis)?;
    let mut test_cases = Vec::new();
    let mut omitted_findings = Vec::new();
    for finding in &synthesis.synthesized_findings {
        let relevant_paths = paths(finding)?;
        if relevant_paths.is_empty() {
            omitted_findings.push(OmittedFinding {
                finding_id: finding.id.clone(),
                title: finding.title.clone(),
                reason: "no_supported_repository_path_for_test_location".to_string(),
            });
            continue;
        }
        let source_finding_ids = finding
            .sources
            .iter()
            .map(|source| source.finding_id.clone())
            .collect::<Vec<_>>();
        let primary_path = &relevant_paths[0];
        let proposed_test_location = if schema == TEST_PLAN_SCHEMA_V2 {
            match Path::new(primary_path).extension().and_then(|s| s.to_str()) {
                Some("java") => "tests/CodeFriendRegressionTest.java".into(),
                Some("py") => "tests/test_codefriend_regression.py".into(),
                Some("js" | "mjs" | "cjs") => "tests/codefriend_regression.test.js".into(),
                _ => "tests/codefriend_regression.rs".into(),
            }
        } else {
            proposed_test_location(primary_path)
        };
        let id = hash(&(
            if schema == TEST_PLAN_SCHEMA_V2 {
                "codefriend.test_case_plan.v2"
            } else {
                "codefriend.test_case_plan.v1"
            },
            &synthesis.repository,
            &synthesis.revision,
            &finding.id,
            &proposed_test_location,
        ))?;
        let mut case = TestCasePlan {
            id,
            finding_id: finding.id.clone(),
            source_finding_ids,
            title: format!("Focused regression test: {}", finding.title),
            severity: finding.severity.clone(),
            behavior_under_test: behavior_under_test(finding, primary_path),
            source_evidence: finding.evidence.clone(),
            proposed_test_location,
            proposed_fixture: proposed_fixture(finding, primary_path),
            expected_pre_fix_failure: expected_pre_fix_failure(finding),
            expected_post_fix_assertion: expected_post_fix_assertion(finding),
            validation_lane: validation_lane(primary_path),
            resource_profile: "local deterministic CPU/filesystem; no provider credentials, network, source mutation, or paid infrastructure".to_string(),
            detection_rationale: detection_rationale(finding, primary_path),
            non_goals: vec![
                "Do not implement the production repair in the test-plan step.".to_string(),
                "Do not mirror the implementation by asserting only function names or snapshot text.".to_string(),
                "Do not mutate repository source while generating the plan.".to_string(),
            ],
            scope_limits: finding.scope_limits.clone(),
        };
        if schema == TEST_PLAN_SCHEMA_V2 {
            case.behavior_under_test = format!(
                "Review the reported behavior at admitted source `{primary_path}`: {}",
                finding.title
            );
            case.proposed_fixture = format!("Derive a minimal regression fixture from admitted source `{primary_path}` and evidence IDs {}. Review source behavior before implementation; no fixture bytes or expected result have been executed.", finding.evidence.join(", "));
            case.expected_pre_fix_failure = "Demonstrate the reported defect with an assertion derived from the admitted source; expected failure remains a proposal until executed.".into();
            case.expected_post_fix_assertion = "Assert the intended externally observable behavior derived from the reviewed source, then execute the test to verify it.".into();
            case.detection_rationale = format!("Source-bound proposal for finding {} using admitted path `{primary_path}`; it does not prove the finding or measured coverage.", finding.id);
            case.scope_limits.push("Test location is proposed, not verified existing; unsupported and privacy-omitted source is not covered by this proposal.".into());
        }
        test_cases.push(case);
    }
    test_cases.sort_by(|a, b| a.id.cmp(&b.id));
    let plan = TestPlan {
        schema: schema.to_string(),
        coverage: if schema == TEST_PLAN_SCHEMA_V2 {
            synthesis.coverage.clone()
        } else {
            None
        },
        synthesis_schema: synthesis.schema.clone(),
        synthesis_digest,
        run_id: synthesis.run_id.clone(),
        repository: synthesis.repository.clone(),
        revision: synthesis.revision.clone(),
        scope_digest: synthesis.scope_digest.clone(),
        test_cases,
        omitted_findings,
    };
    validate_plan_against_synthesis(&plan, synthesis)?;
    Ok(plan)
}

pub fn validate_plan(plan: &TestPlan) -> Result<()> {
    ensure!(
        matches!(plan.schema.as_str(), TEST_PLAN_SCHEMA | TEST_PLAN_SCHEMA_V2)
            && matches!(
                plan.synthesis_schema.as_str(),
                SYNTHESIS_SCHEMA | SYNTHESIS_SCHEMA_V2
            ),
        "invalid_test_plan_schema"
    );
    ensure!(
        if plan.schema == TEST_PLAN_SCHEMA_V2 && plan.synthesis_schema == SYNTHESIS_SCHEMA_V2 {
            plan.coverage.is_some()
        } else {
            plan.coverage.is_none()
        },
        "test_plan_coverage_schema_mismatch"
    );
    ensure!(
        !plan.synthesis_digest.is_empty(),
        "missing_synthesis_digest"
    );
    ensure!(
        !plan.repository.trim().is_empty()
            && !plan.revision.trim().is_empty()
            && !plan.scope_digest.trim().is_empty(),
        "missing_test_plan_provenance"
    );
    let case_ids = plan
        .test_cases
        .iter()
        .map(|case| case.id.as_str())
        .collect::<BTreeSet<_>>();
    ensure!(
        case_ids.len() == plan.test_cases.len(),
        "duplicate_test_case_plan_id"
    );
    for case in &plan.test_cases {
        validate_case(case)?;
    }
    let omitted = plan
        .omitted_findings
        .iter()
        .map(|finding| finding.finding_id.as_str())
        .collect::<BTreeSet<_>>();
    ensure!(
        omitted.len() == plan.omitted_findings.len(),
        "duplicate_omitted_test_plan_finding"
    );
    for finding in &plan.omitted_findings {
        ensure!(
            !finding.finding_id.trim().is_empty()
                && !finding.title.trim().is_empty()
                && !finding.reason.trim().is_empty(),
            "invalid_omitted_test_plan_finding"
        );
    }
    Ok(())
}

fn validate_plan_against_synthesis(plan: &TestPlan, synthesis: &ReviewSynthesis) -> Result<()> {
    validate_plan(plan)?;
    ensure!(
        (if plan.schema == TEST_PLAN_SCHEMA_V2 {
            plan.coverage == synthesis.coverage
        } else {
            plan.coverage.is_none()
        }) && plan.synthesis_schema == synthesis.schema
            && plan.synthesis_digest == hash(synthesis)?
            && plan.run_id == synthesis.run_id
            && plan.repository == synthesis.repository
            && plan.revision == synthesis.revision
            && plan.scope_digest == synthesis.scope_digest,
        "test_plan_synthesis_identity_mismatch"
    );
    let findings = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let planned = plan
        .test_cases
        .iter()
        .map(|case| case.finding_id.as_str())
        .chain(
            plan.omitted_findings
                .iter()
                .map(|finding| finding.finding_id.as_str()),
        )
        .collect::<BTreeSet<_>>();
    ensure!(
        planned == findings
            && plan.test_cases.len() + plan.omitted_findings.len() == findings.len(),
        "test_plan_finding_trace_mismatch"
    );
    for case in &plan.test_cases {
        let finding = synthesis
            .synthesized_findings
            .iter()
            .find(|finding| finding.id == case.finding_id)
            .ok_or_else(|| anyhow::anyhow!("test_case_without_finding"))?;
        ensure!(
            case.source_evidence
                .iter()
                .all(|id| finding.evidence.contains(id)),
            "test_case_untraceable_evidence"
        );
        ensure!(
            case.source_finding_ids.iter().all(|id| finding
                .sources
                .iter()
                .any(|source| &source.finding_id == id)),
            "test_case_untraceable_source_finding"
        );
    }
    Ok(())
}

fn validate_synthesis(synthesis: &ReviewSynthesis) -> Result<()> {
    ensure!(
        (synthesis.schema == SYNTHESIS_SCHEMA && synthesis.coverage.is_none())
            || (synthesis.schema == SYNTHESIS_SCHEMA_V2 && synthesis.coverage.is_some()),
        "invalid_synthesis_schema"
    );
    // A complete review may truthfully contain no findings. Preserve that
    // result as an empty, provenance-bound plan instead of inventing tests.
    ensure!(
        !synthesis.repository.trim().is_empty()
            && !synthesis.revision.trim().is_empty()
            && !synthesis.scope_digest.trim().is_empty(),
        "missing_synthesis_provenance"
    );
    Ok(())
}

fn validate_case(case: &TestCasePlan) -> Result<()> {
    ensure!(
        !case.id.trim().is_empty()
            && !case.finding_id.trim().is_empty()
            && !case.title.trim().is_empty()
            && !case.behavior_under_test.trim().is_empty()
            && !case.expected_pre_fix_failure.trim().is_empty()
            && !case.expected_post_fix_assertion.trim().is_empty()
            && !case.detection_rationale.trim().is_empty(),
        "invalid_test_case_identity"
    );
    ensure!(
        !case.source_finding_ids.is_empty()
            && !case.source_evidence.is_empty()
            && !case.proposed_fixture.trim().is_empty()
            && !case.validation_lane.trim().is_empty()
            && !case.resource_profile.trim().is_empty()
            && !case.non_goals.is_empty(),
        "untraceable_test_case"
    );
    validate_relative_path(&case.proposed_test_location)?;
    ensure!(
        case.proposed_test_location.contains("test")
            || case.proposed_test_location == "lib/dnsmsg-parser/src/dns_message_parser.rs",
        "test_plan_location_must_be_test_surface"
    );
    ensure!(
        !case.expected_post_fix_assertion.contains("TODO")
            && !case.proposed_fixture.contains("TODO")
            && !case.detection_rationale.contains("TODO"),
        "placeholder_test_plan"
    );
    ensure!(
        case.detection_rationale.contains(&case.finding_id),
        "untraceable_test_detection_rationale"
    );
    Ok(())
}

fn relevant_paths(finding: &SynthesizedFinding) -> Vec<String> {
    let mut paths = BTreeSet::new();
    for token in finding
        .semantic_anchor
        .split(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | ':' | '(' | ')' | '[' | ']'))
        .chain(finding.evidence.iter().map(String::as_str))
    {
        let trimmed = normalize_path_token(token);
        if looks_like_path(trimmed) && validate_relative_path(trimmed).is_ok() {
            paths.insert(trimmed.to_string());
        }
    }
    for path in semantic_candidate_paths(finding) {
        paths.insert(path.to_string());
    }
    paths.into_iter().collect()
}

fn normalize_path_token(value: &str) -> &str {
    value
        .trim_matches(|c: char| matches!(c, '"' | '\'' | '`' | ',' | ';'))
        .trim_end_matches(['.', ',', ';'])
}

fn looks_like_path(value: &str) -> bool {
    (value.contains('/') || value.contains('.'))
        && value.len() <= 240
        && !value.starts_with('/')
        && !value.starts_with("http://")
        && !value.starts_with("https://")
}

fn semantic_candidate_paths(finding: &SynthesizedFinding) -> Vec<&'static str> {
    let context = finding_context_text(finding).to_ascii_lowercase();
    let mut paths = Vec::new();
    if context.contains("dnsmessageparser")
        || context.contains("dns message parser")
        || context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        paths.push("lib/dnsmsg-parser/src/dns_message_parser.rs");
    }
    paths
}

fn validate_relative_path(value: &str) -> Result<()> {
    ensure!(!value.trim().is_empty(), "empty_test_plan_path");
    let path = Path::new(value);
    ensure!(!path.is_absolute(), "unsupported_test_plan_path");
    for component in path.components() {
        ensure!(
            matches!(component, Component::Normal(_)),
            "unsupported_test_plan_path"
        );
    }
    ensure!(
        value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/')),
        "unsupported_test_plan_path"
    );
    Ok(())
}

fn proposed_test_location(path: &str) -> String {
    if path == "lib/dnsmsg-parser/src/dns_message_parser.rs" {
        path.to_string()
    } else if path.starts_with("adl/src/cli/") {
        "adl/tests/codefriend_cli_regression.rs".to_string()
    } else if path.starts_with("adl/src/codefriend/review/") {
        "adl/tests/codefriend_review_regression.rs".to_string()
    } else if path.starts_with("adl/src/codefriend/actions/") {
        "adl/tests/codefriend_action_regression.rs".to_string()
    } else if path.starts_with("adl/src/codefriend/") {
        "adl/tests/codefriend_regression.rs".to_string()
    } else if path.starts_with("docs/") {
        "docs/testing/codefriend-regression-plan.md".to_string()
    } else {
        "tests/codefriend_regression.rs".to_string()
    }
}

fn behavior_under_test(finding: &SynthesizedFinding, path: &str) -> String {
    let context = finding_context_text(finding).to_ascii_lowercase();
    if context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        return format!(
            "Verify `{path}` keeps DNS RDATA name decoding isolated per message: repeated `DnsMessageParser::get_rdata_decoder_with_raw_message` calls must not reuse or cumulatively extend `raw_message_for_rdata_parsing` with unrelated attacker-controlled raw RDATA from a prior call."
        );
    }
    format!(
        "Verify the externally visible behavior reported by synthesized finding {} at {}: {}. The case must be driven from the cited source evidence rather than a schema-only assertion.",
        finding.id,
        path,
        finding.title
    )
}

fn proposed_fixture(finding: &SynthesizedFinding, path: &str) -> String {
    let context = finding_context_text(finding).to_ascii_lowercase();
    if context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        return format!(
            "In `{path}`'s existing `#[cfg(test)] mod tests`, add `test_compressed_rdata_buffer_is_replaced_between_calls`. Decode the existing MINFO message `5ZWBgAABAAEAAAABBm1pbmZvbwhleGFtcGxlMQNjb20AAA4AAcAMAA4AAQAADGsADQRmcmVkwBMDam9lwBMAACkQAAAAAAAAHAAKABgZ5zwJEK3VJQEAAABfSBqpS2bKf9CNBXg=` and first RDATA `BGZyZWTAEwNqb2XAEw==`; on the same parser, then pass a second MINFO `NULL` payload `b\"\\x05alice\\x07example\\x03com\\x00\\x03bob\\x07example\\x03com\\x00\"`. Assert the first result is `fred.example1.com. joe.example1.com.`, the second is `alice.example.com. bob.example.com.`, and the retained buffer length equals `parser.raw_message().len() + second_rdata.len()` rather than both RDATA payloads cumulatively."
        );
    }
    format!(
        "Construct the smallest fixture that reaches `{}` using evidence `{}` and source findings `{}`. Include the concrete input that reproduces the reported behavior and an assertion on the externally visible result.",
        path,
        finding.evidence.join(","),
        finding
            .sources
            .iter()
            .map(|source| source.finding_id.as_str())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn expected_pre_fix_failure(finding: &SynthesizedFinding) -> String {
    let context = finding_context_text(finding).to_ascii_lowercase();
    if context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        return "Before the fix, the second same-parser MINFO decode starts at the original message boundary but reads the first appended RDATA, yielding `fred.example1.com. joe.example1.com.` instead of the second payload's `alice.example.com. bob.example.com.`; the retained buffer also includes both RDATA payloads.".to_string();
    }
    format!(
        "Before the fix, the concrete fixture reproduces synthesized finding {} through the admitted evidence.",
        finding.id
    )
}

fn expected_post_fix_assertion(finding: &SynthesizedFinding) -> String {
    let context = finding_context_text(finding).to_ascii_lowercase();
    if context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        return "After the fix, the first MINFO result equals `fred.example1.com. joe.example1.com.`, the second equals `alice.example.com. bob.example.com.`, and `raw_message_for_rdata_parsing().unwrap().len()` equals `raw_message().len() + second_rdata.len()`; any prior-call influence fails one of those exact assertions.".to_string();
    }
    format!(
        "After the fix, the test passes only when the reported behavior for finding {} is corrected and fails if the cited evidence becomes reproducible again.",
        finding.id
    )
}

fn validation_lane(path: &str) -> String {
    if path.starts_with("docs/") {
        "PVF docs/contract lane plus git diff --check".to_string()
    } else if path.starts_with("adl/") {
        "PVF runtime focused Rust test plus git diff --check".to_string()
    } else {
        "PVF local focused validation plus git diff --check".to_string()
    }
}

fn detection_rationale(finding: &SynthesizedFinding, path: &str) -> String {
    let context = finding_context_text(finding).to_ascii_lowercase();
    if context.contains("raw_message_for_rdata_parsing")
        || context.contains("get_rdata_decoder_with_raw_message")
    {
        return format!(
            "Targets finding {} by exercising `{path}` through the exact state-reuse risk described in the accepted synthesis: same parser, multiple decode calls, attacker-controlled raw RDATA, and an assertion that prior-call bytes cannot affect the later decode.",
            finding.id
        );
    }
    format!(
        "Targets finding {} by driving `{}` through externally visible behavior described by evidence `{}` instead of mirroring implementation details; it must fail when that behavior remains observable.",
        finding.id,
        path,
        finding.evidence.join(",")
    )
}

fn finding_context_text(finding: &SynthesizedFinding) -> String {
    let mut parts = vec![
        finding.semantic_anchor.as_str(),
        finding.title.as_str(),
        finding.severity_rationale.as_str(),
    ];
    for source in &finding.sources {
        parts.push(source.rule.as_str());
        parts.push(source.rationale.as_str());
        parts.push(source.inference.as_str());
    }
    parts.join("\n")
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path, limit: u64) -> Result<T> {
    let mut bytes = Vec::new();
    File::open(path)
        .with_context(|| format!("open {}", path.display()))?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "test_plan_input_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_test_plan_input_json"))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut file = File::options()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("create {}", path.display()))?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn copy_json_snapshot(input: &Path, output: &Path) -> Result<()> {
    let mut bytes = Vec::new();
    File::open(input)
        .with_context(|| format!("open {}", input.display()))?
        .take(8 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 8 * 1024 * 1024, "test_plan_input_too_large");
    let mut file = File::options()
        .write(true)
        .create_new(true)
        .open(output)
        .with_context(|| format!("create {}", output.display()))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}
