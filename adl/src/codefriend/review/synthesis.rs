use crate::codefriend::evidence::{
    contracts::{Completion, Finding, ReviewRecord, Severity},
    hash,
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

pub const SYNTHESIS_SCHEMA: &str = "codefriend.review_synthesis.v1";
pub const SYNTHESIS_MANIFEST_SCHEMA: &str = "codefriend.review_synthesis_manifest.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SynthesisOptions {
    pub input: std::path::PathBuf,
    pub out: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SynthesisSource {
    pub finding_id: String,
    pub perspective: String,
    pub rule: String,
    pub severity: Severity,
    pub evidence: Vec<String>,
    pub rationale: String,
    pub confidence: crate::codefriend::evidence::contracts::Confidence,
    pub inference: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SynthesizedFinding {
    pub id: String,
    pub semantic_anchor: String,
    pub title: String,
    pub severity: Severity,
    pub severity_rationale: String,
    pub evidence: Vec<String>,
    pub sources: Vec<SynthesisSource>,
    pub disagreement: Option<String>,
    pub scope_limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewSynthesis {
    pub schema: String,
    pub review_record_digest: String,
    pub run_id: String,
    pub repository: String,
    pub revision: String,
    pub scope_digest: String,
    pub lane_count: usize,
    pub input_finding_count: usize,
    pub synthesized_findings: Vec<SynthesizedFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SynthesisManifest {
    pub schema: String,
    pub synthesis_ref: String,
    pub synthesis_digest: String,
    pub review_record_ref: String,
    pub review_record_digest: String,
    pub synthesized_finding_count: usize,
    pub input_finding_count: usize,
}

pub fn synthesize_from_file(options: SynthesisOptions) -> Result<ReviewSynthesis> {
    ensure!(
        !options.out.exists(),
        "synthesis_output_directory_already_exists"
    );
    let record: ReviewRecord = read_json(&options.input, 8 * 1024 * 1024)?;
    let synthesis = synthesize(&record)?;
    fs::create_dir_all(&options.out)?;
    let synthesis_path = options.out.join("synthesis.json");
    write_json(&synthesis_path, &synthesis)?;
    let synthesis_digest = hash(&synthesis)?;
    let review_record_digest = hash(&record)?;
    let manifest = SynthesisManifest {
        schema: SYNTHESIS_MANIFEST_SCHEMA.to_string(),
        synthesis_ref: "synthesis.json".to_string(),
        synthesis_digest,
        review_record_ref: options
            .input
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("review-record.json")
            .to_string(),
        review_record_digest,
        synthesized_finding_count: synthesis.synthesized_findings.len(),
        input_finding_count: synthesis.input_finding_count,
    };
    write_json(&options.out.join("manifest.json"), &manifest)?;
    Ok(synthesis)
}

pub fn synthesize(record: &ReviewRecord) -> Result<ReviewSynthesis> {
    record.validate()?;
    ensure!(
        record.run.completion == Completion::Complete,
        "synthesis_requires_complete_review"
    );
    let required = ["adversarial", "constitutional", "correctness", "security"];
    let lanes: BTreeSet<_> = record
        .run
        .lane_versions
        .keys()
        .map(String::as_str)
        .collect();
    ensure!(
        required.iter().all(|lane| lanes.contains(lane)) && lanes.len() == required.len(),
        "synthesis_requires_complete_lane_set"
    );
    ensure!(
        record.findings.iter().all(|finding| {
            record
                .run
                .lane_versions
                .contains_key(finding.perspective.as_str())
        }),
        "finding_without_selected_lane"
    );

    let mut groups: BTreeMap<(String, String), Vec<&Finding>> = BTreeMap::new();
    for finding in &record.findings {
        groups
            .entry((finding.semantic_anchor.clone(), finding.title.clone()))
            .or_default()
            .push(finding);
    }

    let mut synthesized_findings = Vec::new();
    for ((semantic_anchor, title), findings) in groups {
        let severity = strongest_severity(&findings);
        let mut evidence = BTreeSet::new();
        let mut perspectives = BTreeSet::new();
        let mut severities = BTreeSet::new();
        let mut rationales = Vec::new();
        let mut scope_limits = BTreeSet::new();
        let mut sources = Vec::new();
        for finding in findings {
            perspectives.insert(finding.perspective.clone());
            severities.insert(format!("{:?}", finding.severity));
            rationales.push(format!("{}: {}", finding.perspective, finding.rationale));
            for id in &finding.evidence {
                evidence.insert(id.clone());
            }
            for limit in &finding.limitations {
                scope_limits.insert(format!("{}: {}", finding.perspective, limit));
            }
            sources.push(SynthesisSource {
                finding_id: finding.id.clone(),
                perspective: finding.perspective.clone(),
                rule: finding.rule.clone(),
                severity: finding.severity.clone(),
                evidence: finding.evidence.clone(),
                rationale: finding.rationale.clone(),
                confidence: finding.confidence.clone(),
                inference: finding.inference.clone(),
                limitations: finding.limitations.clone(),
            });
        }
        sources.sort_by(|a, b| a.finding_id.cmp(&b.finding_id));
        let evidence = evidence.into_iter().collect::<Vec<_>>();
        let scope_limits = scope_limits.into_iter().collect::<Vec<_>>();
        let disagreement = if perspectives.len() > 1 && severities.len() > 1 {
            Some(format!(
                "severity disagreement retained across {} perspectives: {}",
                perspectives.len(),
                severities.into_iter().collect::<Vec<_>>().join(",")
            ))
        } else if perspectives.len() > 1 {
            Some(format!(
                "multi-perspective attribution retained: {}",
                perspectives.into_iter().collect::<Vec<_>>().join(",")
            ))
        } else {
            None
        };
        let mut synthesized = SynthesizedFinding {
            id: String::new(),
            semantic_anchor,
            title,
            severity,
            severity_rationale: rationales.join(" | "),
            evidence,
            sources,
            disagreement,
            scope_limits,
        };
        synthesized.id = hash(&(
            "codefriend.synthesized_finding.v1",
            &record.run.repository,
            &record.run.revision,
            &record.run.scope_digest,
            &synthesized.semantic_anchor,
            &synthesized.title,
        ))?;
        synthesized_findings.push(synthesized);
    }
    synthesized_findings.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(ReviewSynthesis {
        schema: SYNTHESIS_SCHEMA.to_string(),
        review_record_digest: hash(record)?,
        run_id: record.run.id.clone(),
        repository: record.run.repository.clone(),
        revision: record.run.revision.clone(),
        scope_digest: record.run.scope_digest.clone(),
        lane_count: record.run.lane_versions.len(),
        input_finding_count: record.findings.len(),
        synthesized_findings,
    })
}

fn strongest_severity(findings: &[&Finding]) -> Severity {
    findings
        .iter()
        .map(|finding| finding.severity.clone())
        .max_by_key(severity_rank)
        .unwrap_or(Severity::Info)
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
    ensure!(bytes.len() as u64 <= limit, "synthesis_input_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_synthesis_input_json"))
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
