use crate::codefriend::{
    evidence::{
        contracts::{ReviewRecord, Severity},
        hash,
    },
    review::synthesis::{
        synthesize, ReviewSynthesis, SynthesisManifest, SynthesizedFinding,
        SYNTHESIS_MANIFEST_SCHEMA, SYNTHESIS_SCHEMA,
    },
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

pub const REMEDIATION_PLAN_SCHEMA: &str = "codefriend.remediation_plan.v1";
pub const REMEDIATION_MANIFEST_SCHEMA: &str = "codefriend.remediation_manifest.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemediationOptions {
    pub input: PathBuf,
    pub out: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemediationAction {
    pub id: String,
    pub finding_id: String,
    pub source_finding_ids: Vec<String>,
    pub title: String,
    pub severity: Severity,
    pub owner_role: String,
    pub assignment_status: String,
    pub relevant_paths: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub dependencies: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub validation: Vec<String>,
    pub risk_and_non_goals: Vec<String>,
    pub uncertainty: Vec<String>,
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
pub struct RemediationPlan {
    pub schema: String,
    pub synthesis_schema: String,
    pub synthesis_digest: String,
    pub run_id: String,
    pub repository: String,
    pub revision: String,
    pub scope_digest: String,
    pub action_order: Vec<String>,
    pub actions: Vec<RemediationAction>,
    pub omitted_findings: Vec<OmittedFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemediationManifest {
    pub schema: String,
    pub synthesis_ref: String,
    pub synthesis_digest: String,
    pub synthesis_manifest_ref: String,
    pub synthesis_manifest_digest: String,
    pub review_record_ref: String,
    pub review_record_digest: String,
    pub remediation_plan_ref: String,
    pub remediation_plan_digest: String,
    pub action_count: usize,
    pub omitted_finding_count: usize,
}

pub fn plan_from_file(options: RemediationOptions) -> Result<RemediationPlan> {
    ensure!(
        !options.out.exists(),
        "remediation_output_directory_already_exists"
    );
    let (synthesis, synthesis_manifest, review_record) =
        read_completed_synthesis_bundle(&options.input)?;
    let plan = plan(&synthesis, &review_record)?;
    fs::create_dir(&options.out).with_context(|| format!("create {}", options.out.display()))?;
    copy_json_snapshot(&options.input, &options.out.join("synthesis.json"))?;
    write_json(
        &options.out.join("synthesis-manifest.json"),
        &synthesis_manifest,
    )?;
    write_json(&options.out.join("review-record.json"), &review_record)?;
    write_json(&options.out.join("remediation-plan.json"), &plan)?;
    let manifest = RemediationManifest {
        schema: REMEDIATION_MANIFEST_SCHEMA.to_string(),
        synthesis_ref: "synthesis.json".to_string(),
        synthesis_digest: plan.synthesis_digest.clone(),
        synthesis_manifest_ref: "synthesis-manifest.json".to_string(),
        synthesis_manifest_digest: hash(&synthesis_manifest)?,
        review_record_ref: "review-record.json".to_string(),
        review_record_digest: hash(&review_record)?,
        remediation_plan_ref: "remediation-plan.json".to_string(),
        remediation_plan_digest: hash(&plan)?,
        action_count: plan.actions.len(),
        omitted_finding_count: plan.omitted_findings.len(),
    };
    write_json(&options.out.join("manifest.json"), &manifest)?;
    Ok(plan)
}

pub fn read_plan_from_file(input: &Path) -> Result<RemediationPlan> {
    let remediation_plan: RemediationPlan = read_json(input, 8 * 1024 * 1024)?;
    let directory = input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("remediation_plan_parent_missing"))?;
    let manifest: RemediationManifest = read_json(&directory.join("manifest.json"), 1024 * 1024)
        .context("remediation_manifest_missing_or_invalid")?;
    validate_remediation_manifest(&manifest, &remediation_plan)?;
    ensure!(
        input.file_name().and_then(|name| name.to_str())
            == Some(manifest.remediation_plan_ref.as_str()),
        "remediation_plan_reference_mismatch"
    );
    let synthesis_path = resolve_bundle_ref(directory, &manifest.synthesis_ref)?;
    let synthesis_manifest_path = resolve_bundle_ref(directory, &manifest.synthesis_manifest_ref)?;
    let review_record_path = resolve_bundle_ref(directory, &manifest.review_record_ref)?;
    let (synthesis, synthesis_manifest, review_record) = read_completed_synthesis_bundle_paths(
        &synthesis_path,
        &synthesis_manifest_path,
        &review_record_path,
    )?;
    ensure!(
        hash(&synthesis_manifest)? == manifest.synthesis_manifest_digest
            && hash(&review_record)? == manifest.review_record_digest,
        "remediation_source_bundle_digest_mismatch"
    );
    validate_plan_against_synthesis(&remediation_plan, &synthesis)?;
    ensure!(
        plan(&synthesis, &review_record)? == remediation_plan,
        "remediation_plan_not_canonical_for_synthesis"
    );
    Ok(remediation_plan)
}

fn validate_remediation_manifest(
    manifest: &RemediationManifest,
    plan: &RemediationPlan,
) -> Result<()> {
    ensure!(
        manifest.schema == REMEDIATION_MANIFEST_SCHEMA
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.synthesis_manifest_ref == "synthesis-manifest.json"
            && manifest.review_record_ref == "review-record.json"
            && manifest.remediation_plan_ref == "remediation-plan.json",
        "invalid_remediation_manifest"
    );
    ensure!(
        manifest.synthesis_digest == plan.synthesis_digest
            && manifest.remediation_plan_digest == hash(plan)?
            && manifest.action_count == plan.actions.len()
            && manifest.omitted_finding_count == plan.omitted_findings.len(),
        "remediation_manifest_digest_or_count_mismatch"
    );
    Ok(())
}

fn read_completed_synthesis_bundle(
    synthesis_path: &Path,
) -> Result<(ReviewSynthesis, SynthesisManifest, ReviewRecord)> {
    let directory = synthesis_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("synthesis_parent_missing"))?;
    read_completed_synthesis_bundle_paths(
        synthesis_path,
        &directory.join("manifest.json"),
        &directory.join("review-record.json"),
    )
}

fn read_completed_synthesis_bundle_paths(
    synthesis_path: &Path,
    manifest_path: &Path,
    review_record_path: &Path,
) -> Result<(ReviewSynthesis, SynthesisManifest, ReviewRecord)> {
    let synthesis: ReviewSynthesis = read_json(synthesis_path, 8 * 1024 * 1024)?;
    let manifest: SynthesisManifest =
        read_json(manifest_path, 1024 * 1024).context("synthesis_manifest_missing_or_invalid")?;
    let review_record: ReviewRecord = read_json(review_record_path, 8 * 1024 * 1024)
        .context("synthesis_review_record_missing_or_invalid")?;
    ensure!(
        manifest.schema == SYNTHESIS_MANIFEST_SCHEMA
            && manifest.synthesis_ref == "synthesis.json"
            && manifest.review_record_ref == "review-record.json",
        "invalid_synthesis_manifest"
    );
    ensure!(
        synthesis_path.file_name().and_then(|name| name.to_str())
            == Some(manifest.synthesis_ref.as_str())
            && review_record_path
                .file_name()
                .and_then(|name| name.to_str())
                == Some(manifest.review_record_ref.as_str()),
        "synthesis_manifest_reference_mismatch"
    );
    ensure!(
        hash(&synthesis)? == manifest.synthesis_digest
            && hash(&review_record)? == manifest.review_record_digest
            && synthesis.review_record_digest == manifest.review_record_digest
            && synthesis.synthesized_findings.len() == manifest.synthesized_finding_count
            && synthesis.input_finding_count == manifest.input_finding_count,
        "synthesis_bundle_digest_or_count_mismatch"
    );
    ensure!(
        synthesize(&review_record)? == synthesis,
        "synthesis_not_canonical_for_completed_review"
    );
    Ok((synthesis, manifest, review_record))
}

fn resolve_bundle_ref(directory: &Path, reference: &str) -> Result<PathBuf> {
    validate_relative_path(reference)?;
    ensure!(
        Path::new(reference).components().count() == 1,
        "nested_remediation_bundle_ref_denied"
    );
    Ok(directory.join(reference))
}

pub fn plan(synthesis: &ReviewSynthesis, review_record: &ReviewRecord) -> Result<RemediationPlan> {
    ensure!(
        synthesize(review_record)? == *synthesis,
        "remediation_requires_canonical_completed_synthesis"
    );
    let evidence_paths = review_record
        .admission
        .evidence
        .iter()
        .map(|evidence| (evidence.id.as_str(), evidence.path.as_str()))
        .collect::<BTreeMap<_, _>>();
    plan_with_evidence_paths(synthesis, &evidence_paths)
}

fn plan_with_evidence_paths(
    synthesis: &ReviewSynthesis,
    evidence_paths: &BTreeMap<&str, &str>,
) -> Result<RemediationPlan> {
    validate_synthesis(synthesis)?;
    let synthesis_digest = hash(synthesis)?;
    let mut actions = Vec::new();
    let mut omitted_findings = Vec::new();
    let mut prior_for_path: BTreeMap<String, String> = BTreeMap::new();
    for finding in &synthesis.synthesized_findings {
        let relevant_paths = relevant_paths(finding, evidence_paths);
        if relevant_paths.is_empty() {
            omitted_findings.push(OmittedFinding {
                finding_id: finding.id.clone(),
                title: finding.title.clone(),
                reason: "no_supported_repository_path_in_synthesized_finding".to_string(),
            });
            continue;
        }
        let mut evidence_ids = finding.evidence.clone();
        evidence_ids.sort();
        evidence_ids.dedup();
        let owner_role = owner_role(&relevant_paths);
        let source_finding_ids = finding
            .sources
            .iter()
            .map(|source| source.finding_id.clone())
            .collect::<Vec<_>>();
        let id = hash(&(
            "codefriend.remediation_action.v1",
            &synthesis.repository,
            &synthesis.revision,
            &finding.id,
            &relevant_paths,
        ))?;
        let mut dependencies = Vec::new();
        for path in &relevant_paths {
            if let Some(previous) = prior_for_path.get(path) {
                dependencies.push(previous.clone());
            }
        }
        dependencies.sort();
        dependencies.dedup();
        for path in &relevant_paths {
            prior_for_path.insert(path.clone(), id.clone());
        }
        actions.push(RemediationAction {
            id,
            finding_id: finding.id.clone(),
            source_finding_ids,
            title: format!("Bounded repair: {}", finding.title),
            severity: finding.severity.clone(),
            owner_role,
            assignment_status: "unassigned".to_string(),
            relevant_paths,
            evidence_ids,
            dependencies,
            acceptance_criteria: acceptance_criteria(finding),
            validation: validation_plan(finding),
            risk_and_non_goals: risk_and_non_goals(finding),
            uncertainty: finding.scope_limits.clone(),
        });
    }
    actions.sort_by(|a, b| {
        severity_rank(&b.severity)
            .cmp(&severity_rank(&a.severity))
            .then(a.id.cmp(&b.id))
    });
    let action_order = topological_order(&actions)?;
    let plan = RemediationPlan {
        schema: REMEDIATION_PLAN_SCHEMA.to_string(),
        synthesis_schema: synthesis.schema.clone(),
        synthesis_digest,
        run_id: synthesis.run_id.clone(),
        repository: synthesis.repository.clone(),
        revision: synthesis.revision.clone(),
        scope_digest: synthesis.scope_digest.clone(),
        action_order,
        actions,
        omitted_findings,
    };
    validate_plan_against_synthesis(&plan, synthesis)?;
    Ok(plan)
}

pub fn validate_plan(plan: &RemediationPlan) -> Result<()> {
    ensure!(
        plan.schema == REMEDIATION_PLAN_SCHEMA && plan.synthesis_schema == SYNTHESIS_SCHEMA,
        "invalid_remediation_plan_schema"
    );
    ensure!(
        !plan.synthesis_digest.is_empty(),
        "missing_synthesis_digest"
    );
    ensure!(
        !plan.repository.trim().is_empty()
            && !plan.revision.trim().is_empty()
            && !plan.scope_digest.trim().is_empty(),
        "missing_remediation_plan_provenance"
    );
    let action_ids = plan
        .actions
        .iter()
        .map(|action| action.id.as_str())
        .collect::<BTreeSet<_>>();
    ensure!(
        action_ids.len() == plan.actions.len(),
        "duplicate_remediation_action_id"
    );
    ensure!(
        plan.action_order.len() == plan.actions.len()
            && plan
                .action_order
                .iter()
                .all(|id| action_ids.contains(id.as_str())),
        "remediation_action_order_mismatch"
    );
    for action in &plan.actions {
        validate_action(action, &action_ids)?;
    }
    ensure!(
        topological_order(&plan.actions)? == plan.action_order,
        "remediation_action_order_not_topological"
    );
    let omitted = plan
        .omitted_findings
        .iter()
        .map(|finding| finding.finding_id.as_str())
        .collect::<BTreeSet<_>>();
    ensure!(
        omitted.len() == plan.omitted_findings.len(),
        "duplicate_omitted_finding"
    );
    for finding in &plan.omitted_findings {
        ensure!(
            !finding.finding_id.trim().is_empty()
                && !finding.title.trim().is_empty()
                && !finding.reason.trim().is_empty(),
            "invalid_omitted_finding"
        );
    }
    Ok(())
}

fn validate_plan_against_synthesis(
    plan: &RemediationPlan,
    synthesis: &ReviewSynthesis,
) -> Result<()> {
    validate_plan(plan)?;
    ensure!(
        plan.synthesis_digest == hash(synthesis)?,
        "remediation_synthesis_digest_mismatch"
    );
    let findings = synthesis
        .synthesized_findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let planned = plan
        .actions
        .iter()
        .map(|action| action.finding_id.as_str())
        .chain(
            plan.omitted_findings
                .iter()
                .map(|finding| finding.finding_id.as_str()),
        )
        .collect::<BTreeSet<_>>();
    ensure!(planned == findings, "remediation_finding_trace_mismatch");
    for action in &plan.actions {
        let finding = synthesis
            .synthesized_findings
            .iter()
            .find(|finding| finding.id == action.finding_id)
            .ok_or_else(|| anyhow::anyhow!("remediation_action_without_finding"))?;
        ensure!(
            action
                .evidence_ids
                .iter()
                .all(|id| finding.evidence.contains(id)),
            "remediation_action_untraceable_evidence"
        );
        ensure!(
            action.source_finding_ids.iter().all(|id| finding
                .sources
                .iter()
                .any(|source| &source.finding_id == id)),
            "remediation_action_untraceable_source_finding"
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
        "remediation_requires_synthesized_findings"
    );
    ensure!(
        !synthesis.repository.trim().is_empty()
            && !synthesis.revision.trim().is_empty()
            && !synthesis.scope_digest.trim().is_empty(),
        "missing_synthesis_provenance"
    );
    Ok(())
}

fn validate_action(action: &RemediationAction, action_ids: &BTreeSet<&str>) -> Result<()> {
    ensure!(
        !action.id.trim().is_empty()
            && !action.finding_id.trim().is_empty()
            && !action.title.trim().is_empty()
            && !action.owner_role.trim().is_empty()
            && action.assignment_status == "unassigned",
        "invalid_remediation_action_identity"
    );
    ensure!(
        !action.source_finding_ids.is_empty()
            && !action.relevant_paths.is_empty()
            && !action.evidence_ids.is_empty()
            && !action.acceptance_criteria.is_empty()
            && !action.validation.is_empty()
            && !action.risk_and_non_goals.is_empty(),
        "untraceable_remediation_action"
    );
    for path in &action.relevant_paths {
        validate_relative_path(path)?;
    }
    ensure!(
        action
            .dependencies
            .iter()
            .all(|id| action_ids.contains(id.as_str()) && id != &action.id),
        "invalid_remediation_dependency"
    );
    ensure!(
        action
            .acceptance_criteria
            .iter()
            .any(|criterion| criterion.contains(&action.finding_id)),
        "untraceable_remediation_acceptance"
    );
    Ok(())
}

fn topological_order(actions: &[RemediationAction]) -> Result<Vec<String>> {
    let by_id = actions
        .iter()
        .map(|action| (action.id.as_str(), action))
        .collect::<BTreeMap<_, _>>();
    let mut temporary = BTreeSet::new();
    let mut permanent = BTreeSet::new();
    let mut order = Vec::new();
    for action in actions {
        visit(
            action.id.as_str(),
            &by_id,
            &mut temporary,
            &mut permanent,
            &mut order,
        )?;
    }
    Ok(order)
}

fn visit<'a>(
    id: &'a str,
    by_id: &BTreeMap<&'a str, &'a RemediationAction>,
    temporary: &mut BTreeSet<&'a str>,
    permanent: &mut BTreeSet<&'a str>,
    order: &mut Vec<String>,
) -> Result<()> {
    if permanent.contains(id) {
        return Ok(());
    }
    ensure!(temporary.insert(id), "remediation_dependency_cycle");
    let action = by_id
        .get(id)
        .ok_or_else(|| anyhow::anyhow!("invalid_remediation_dependency"))?;
    for dependency in &action.dependencies {
        visit(dependency, by_id, temporary, permanent, order)?;
    }
    temporary.remove(id);
    permanent.insert(id);
    order.push(id.to_string());
    Ok(())
}

fn relevant_paths(
    finding: &SynthesizedFinding,
    evidence_paths: &BTreeMap<&str, &str>,
) -> Vec<String> {
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
    for evidence_id in &finding.evidence {
        if let Some(path) = evidence_paths.get(evidence_id.as_str()) {
            if validate_relative_path(path).is_ok() {
                paths.insert((*path).to_string());
            }
        }
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

fn validate_relative_path(value: &str) -> Result<()> {
    ensure!(!value.trim().is_empty(), "empty_remediation_path");
    let path = Path::new(value);
    ensure!(!path.is_absolute(), "unsupported_remediation_path");
    for component in path.components() {
        ensure!(
            matches!(component, Component::Normal(_)),
            "unsupported_remediation_path"
        );
    }
    ensure!(
        value
            .bytes()
            .all(|b| { b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/') }),
        "unsupported_remediation_path"
    );
    Ok(())
}

fn owner_role(paths: &[String]) -> String {
    if paths.iter().any(|path| path.starts_with("docs/")) {
        "documentation-owner".to_string()
    } else if paths.iter().any(|path| path.contains("test")) {
        "test-owner".to_string()
    } else if paths.iter().any(|path| path.starts_with("adl/src/cli/")) {
        "cli-owner".to_string()
    } else if paths
        .iter()
        .any(|path| path.starts_with("adl/src/codefriend/"))
    {
        "codefriend-owner".to_string()
    } else {
        "repository-owner".to_string()
    }
}

fn acceptance_criteria(finding: &SynthesizedFinding) -> Vec<String> {
    vec![
        format!(
            "Resolve synthesized finding {} without changing unrelated scope.",
            finding.id
        ),
        format!(
            "Retain traceability to evidence ids: {}.",
            finding.evidence.join(",")
        ),
        "Add or update focused validation proving the bounded repair and negative case."
            .to_string(),
    ]
}

fn validation_plan(finding: &SynthesizedFinding) -> Vec<String> {
    vec![
        "Run the smallest focused test covering the repaired path.".to_string(),
        "Run git diff --check over the exact candidate.".to_string(),
        format!(
            "Independent reviewer must verify finding {} is resolved and no unrelated finding was bundled.",
            finding.id
        ),
    ]
}

fn risk_and_non_goals(finding: &SynthesizedFinding) -> Vec<String> {
    let mut values = vec![
        "Do not create issues, assign humans, publish PRs, or mutate source as part of planning."
            .to_string(),
        "Do not combine unrelated findings merely because they share severity.".to_string(),
    ];
    if !finding.scope_limits.is_empty() {
        values.push(format!(
            "Preserve scope limits: {}.",
            finding.scope_limits.join(" | ")
        ));
    }
    values
}

fn severity_rank(severity: &Severity) -> u8 {
    match severity {
        Severity::Info => 0,
        Severity::Low => 1,
        Severity::Medium => 2,
        Severity::High => 3,
        Severity::Critical => 4,
    }
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path, limit: u64) -> Result<T> {
    let mut bytes = Vec::new();
    File::open(path)
        .with_context(|| format!("open {}", path.display()))?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "remediation_input_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_remediation_input_json"))
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
    ensure!(
        bytes.len() <= 8 * 1024 * 1024,
        "remediation_input_too_large"
    );
    let mut file = File::options()
        .write(true)
        .create_new(true)
        .open(output)
        .with_context(|| format!("create {}", output.display()))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}
