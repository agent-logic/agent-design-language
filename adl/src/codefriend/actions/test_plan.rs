use crate::codefriend::{
    evidence::{contracts::Severity, hash},
    review::synthesis::{ReviewSynthesis, SynthesizedFinding, SYNTHESIS_SCHEMA},
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

pub const TEST_PLAN_SCHEMA: &str = "codefriend.test_plan.v1";
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
    pub synthesis_ref: String,
    pub synthesis_digest: String,
    pub test_plan_ref: String,
    pub test_plan_digest: String,
    pub test_case_count: usize,
    pub omitted_finding_count: usize,
}

pub fn plan_from_file(options: TestPlanOptions) -> Result<TestPlan> {
    ensure!(
        !options.out.exists(),
        "test_plan_output_directory_already_exists"
    );
    let synthesis: ReviewSynthesis = read_json(&options.input, 8 * 1024 * 1024)?;
    let plan = plan(&synthesis)?;
    fs::create_dir(&options.out).with_context(|| format!("create {}", options.out.display()))?;
    copy_json_snapshot(&options.input, &options.out.join("synthesis.json"))?;
    write_json(&options.out.join("test-plan.json"), &plan)?;
    let manifest = TestPlanManifest {
        schema: TEST_PLAN_MANIFEST_SCHEMA.to_string(),
        synthesis_ref: "synthesis.json".to_string(),
        synthesis_digest: plan.synthesis_digest.clone(),
        test_plan_ref: "test-plan.json".to_string(),
        test_plan_digest: hash(&plan)?,
        test_case_count: plan.test_cases.len(),
        omitted_finding_count: plan.omitted_findings.len(),
    };
    write_json(&options.out.join("manifest.json"), &manifest)?;
    Ok(plan)
}

pub fn read_plan_from_file(input: &Path) -> Result<TestPlan> {
    let plan: TestPlan = read_json(input, 8 * 1024 * 1024)?;
    validate_plan(&plan)?;
    Ok(plan)
}

pub fn plan(synthesis: &ReviewSynthesis) -> Result<TestPlan> {
    validate_synthesis(synthesis)?;
    let synthesis_digest = hash(synthesis)?;
    let mut test_cases = Vec::new();
    let mut omitted_findings = Vec::new();
    for finding in &synthesis.synthesized_findings {
        let relevant_paths = relevant_paths(finding);
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
        let proposed_test_location = proposed_test_location(primary_path);
        let id = hash(&(
            "codefriend.test_case_plan.v1",
            &synthesis.repository,
            &synthesis.revision,
            &finding.id,
            &proposed_test_location,
        ))?;
        test_cases.push(TestCasePlan {
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
        });
    }
    test_cases.sort_by(|a, b| a.id.cmp(&b.id));
    let plan = TestPlan {
        schema: TEST_PLAN_SCHEMA.to_string(),
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
        plan.schema == TEST_PLAN_SCHEMA && plan.synthesis_schema == SYNTHESIS_SCHEMA,
        "invalid_test_plan_schema"
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
        plan.synthesis_digest == hash(synthesis)?,
        "test_plan_synthesis_digest_mismatch"
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
    ensure!(planned == findings, "test_plan_finding_trace_mismatch");
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
        synthesis.schema == SYNTHESIS_SCHEMA,
        "invalid_synthesis_schema"
    );
    ensure!(
        !synthesis.synthesized_findings.is_empty(),
        "test_plan_requires_synthesized_findings"
    );
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
        case.proposed_test_location.contains("test"),
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
    if path.starts_with("adl/src/cli/") {
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
            "Add a focused dnsmsg-parser regression near `{path}` that constructs one parser, decodes a first raw message/RDATA pair containing a compression target, then decodes a second unrelated raw message/RDATA pair on the same parser. The fixture should assert the second decode cannot resolve names or bytes from the first RDATA append and should fail if `raw_message_for_rdata_parsing` grows cumulatively across calls."
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
        return "Before the fix, the second decode can observe state created by the first raw RDATA append, so the regression should fail by detecting cross-call buffer reuse or cumulative growth.".to_string();
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
        return "After the fix, each decode uses only the current message/RDATA bytes; the assertion fails closed if unrelated prior RDATA can influence compressed-name decoding or if the reusable buffer length carries across calls.".to_string();
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
