//! Relate observed graph/deployment declarations to human ADR records, without granting authority.
use super::structure::{self, Location, StructureReport};
use crate::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
        hash,
        store::Store,
    },
    ingestion::validate_path,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const VERSION: &str = "codefriend.rationale.v1";
const LIMIT: u64 = 32 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoundarySelection {
    pub boundary: String,
    pub deployment_path: String,
    pub service: String,
    pub rationale_paths: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationaleSelection {
    pub schema: String,
    pub graph_digest: String,
    pub revision: String,
    pub boundaries: Vec<BoundarySelection>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub status: String,
    pub boundary: String,
    pub service: String,
    pub decision_key: String,
    pub choice: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationaleSource {
    pub location: Location,
    pub revision: String,
    pub decision: Decision,
    pub explanation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoundaryExplanation {
    pub boundary: String,
    pub nodes: Vec<String>,
    pub graph_digest: String,
    pub revision: String,
    pub boundary_evidence: Vec<Location>,
    pub deployment_evidence: Option<Location>,
    pub deployment_observation: String,
    pub quantum_inference: String,
    pub rationale: Vec<RationaleSource>,
    pub conflicting_decision_keys: Vec<String>,
    pub unknowns: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationaleReport {
    pub schema: String,
    pub selection: RationaleSelection,
    pub graph: StructureReport,
    pub boundaries: Vec<BoundaryExplanation>,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
fn identifier(value: &str) -> Result<()> {
    ensure!(
        !value.is_empty()
            && value.len() <= 128
            && value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
        "invalid_rationale_identifier"
    );
    Ok(())
}
fn source<'a>(g: &'a StructureReport, path: &str) -> Result<Option<(&'a str, Location)>> {
    validate_path(path)?;
    let a = &g.record.admission;
    let Some(object) = a.packet.objects.iter().find(|o| o.path == path) else {
        return Ok(None);
    };
    let Some(content) = object.content.as_deref() else {
        return Ok(None);
    };
    let Some(e) = a.evidence.iter().find(|e| e.path == path) else {
        return Ok(None);
    };
    ensure!(
        content.len() <= 64 * 1024,
        "rationale_source_bounds_exceeded"
    );
    Ok(Some((
        content,
        Location {
            path: path.into(),
            line: 1,
            evidence_id: e.id.clone(),
        },
    )))
}
fn parse_adr(content: &str) -> Option<(Decision, String)> {
    let normalized = content.replace("\r\n", "\n");
    let rest = normalized.strip_prefix("+++\n")?;
    let (header, body) = rest.split_once("\n+++\n")?;
    if header.len() > 8192 || body.trim().is_empty() || body.len() > 8192 {
        return None;
    }
    let d: Decision = toml::from_str(header).ok()?;
    for value in [
        &d.status,
        &d.boundary,
        &d.service,
        &d.decision_key,
        &d.choice,
    ] {
        identifier(value).ok()?;
    }
    Some((d, body.trim().into()))
}
fn deployment(content: &str, selected: &BoundarySelection) -> Result<bool> {
    let v: serde_json::Value =
        serde_json::from_str(content).map_err(|_| anyhow::anyhow!("invalid_deployment_json"))?;
    let Some(object) = v.as_object() else {
        return Ok(false);
    };
    // Unsupported Compose semantics cannot silently become a deployability claim.
    if object
        .keys()
        .any(|k| !["services", "name", "version"].contains(&k.as_str()))
    {
        return Ok(false);
    }
    let Some(service) = v
        .get("services")
        .and_then(|s| s.get(&selected.service))
        .and_then(|s| s.as_object())
    else {
        return Ok(false);
    };
    if service
        .keys()
        .any(|k| !["image", "labels"].contains(&k.as_str()))
    {
        return Ok(false);
    }
    let Some(image) = service.get("image").and_then(|s| s.as_str()) else {
        return Ok(false);
    };
    if image.is_empty()
        || image.len() > 512
        || !image
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/._-:@".contains(&b))
    {
        return Ok(false);
    }
    Ok(service
        .get("labels")
        .and_then(|l| l.get("codefriend.boundary"))
        .and_then(|s| s.as_str())
        == Some(selected.boundary.as_str()))
}
pub fn architecture_rationale_reporter(
    store: &Store,
    graph: StructureReport,
    mut selection: RationaleSelection,
) -> Result<RationaleReport> {
    graph.validate(store)?;
    ensure!(selection.schema == VERSION, "unsupported_rationale_version");
    ensure!(
        selection.graph_digest == graph.digest && selection.revision == graph.record.run.revision,
        "stale_rationale_graph"
    );
    ensure!(
        !selection.boundaries.is_empty() && selection.boundaries.len() <= 32,
        "rationale_boundary_bounds_exceeded"
    );
    selection
        .boundaries
        .sort_by(|a, b| a.boundary.cmp(&b.boundary));
    ensure!(
        selection
            .boundaries
            .windows(2)
            .all(|w| w[0].boundary != w[1].boundary),
        "duplicate_rationale_boundary"
    );
    let mut explanations = Vec::new();
    for selected in &mut selection.boundaries {
        identifier(&selected.boundary)?;
        identifier(&selected.service)?;
        validate_path(&selected.deployment_path)?;
        ensure!(
            selected.rationale_paths.len() <= 64,
            "rationale_document_bounds_exceeded"
        );
        selected.rationale_paths.sort();
        ensure!(
            selected.rationale_paths.windows(2).all(|w| w[0] != w[1]),
            "duplicate_rationale_document"
        );
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.layer.as_deref() == Some(&selected.boundary))
            .collect();
        ensure!(!nodes.is_empty(), "rationale_boundary_not_in_graph");
        let mut b = BoundaryExplanation {
            boundary: selected.boundary.clone(),
            nodes: nodes.iter().map(|n| n.id.clone()).collect(),
            graph_digest: graph.digest.clone(),
            revision: graph.record.run.revision.clone(),
            boundary_evidence: nodes.iter().map(|n| n.location.clone()).collect(),
            deployment_evidence: None,
            deployment_observation: "unavailable_or_unsupported".into(),
            quantum_inference: "unknown".into(),
            rationale: vec![],
            conflicting_decision_keys: vec![],
            unknowns: vec![],
        };
        if !graph.analysis_complete {
            b.unknowns.push("partial_structure_graph".into());
        }
        if let Some((content, location)) = source(&graph, &selected.deployment_path)? {
            b.deployment_evidence = Some(location);
            if deployment(content, selected)? {
                b.deployment_observation =
                    "separate_compose_service_declared_with_matching_boundary_label".into();
                b.quantum_inference =
                    "candidate_quantum_from_declaration_only_runtime_independence_unverified"
                        .into();
            } else {
                b.unknowns
                    .push("unsupported_or_unmatched_deployment_relationship".into());
            }
        } else {
            b.unknowns.push("deployment_evidence_unavailable".into());
        }
        for path in &selected.rationale_paths {
            let Some((content, location)) = source(&graph, path)? else {
                b.unknowns
                    .push(format!("rationale_evidence_unavailable:{path}"));
                continue;
            };
            let Some((decision, explanation)) = parse_adr(content) else {
                b.unknowns
                    .push(format!("unsupported_rationale_document:{path}"));
                continue;
            };
            ensure!(
                decision.boundary == selected.boundary && decision.service == selected.service,
                "rationale_reference_mismatch"
            );
            if !["accepted", "candidate", "superseded"].contains(&decision.status.as_str()) {
                b.unknowns
                    .push(format!("unsupported_rationale_status:{path}"));
            }
            b.rationale.push(RationaleSource {
                location,
                revision: graph.record.run.revision.clone(),
                decision,
                explanation,
            });
        }
        let mut choices: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for r in &b.rationale {
            if r.decision.status == "accepted" {
                choices
                    .entry(r.decision.decision_key.clone())
                    .or_default()
                    .insert(r.decision.choice.clone());
            }
        }
        if choices.is_empty() {
            b.unknowns.push("no_accepted_rationale".into());
        }
        b.conflicting_decision_keys = choices
            .into_iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(k, _)| k)
            .collect();
        b.unknowns.sort();
        b.boundary_evidence.sort();
        b.boundary_evidence.dedup();
        explanations.push(b);
    }
    let complete = explanations
        .iter()
        .all(|b| b.unknowns.is_empty() && b.conflicting_decision_keys.is_empty());
    let run = Run::new(
        &graph.record.admission,
        BTreeMap::from([
            ("architecture-rationale".into(), VERSION.into()),
            ("graph-digest".into(), graph.digest.clone()),
            ("selection-digest".into(), hash(&selection)?),
        ]),
        "none-local-evidence".into(),
        if complete {
            Completion::Complete
        } else {
            Completion::Incomplete
        },
        if complete {
            vec![]
        } else {
            vec!["rationale_unknown_or_conflicting".into()]
        },
    )?;
    let mut record = ReviewRecord {
        admission: graph.record.admission.clone(),
        run,
        findings: vec![],
    };
    for b in &explanations {
        let mut evidence: BTreeSet<_> = b
            .boundary_evidence
            .iter()
            .map(|l| l.evidence_id.clone())
            .collect();
        evidence.extend(b.deployment_evidence.iter().map(|l| l.evidence_id.clone()));
        evidence.extend(b.rationale.iter().map(|r| r.location.evidence_id.clone()));
        let (rule, title) = if !b.conflicting_decision_keys.is_empty() {
            ("conflicting_rationale", "Accepted ADR choices conflict")
        } else if !b.unknowns.is_empty() {
            (
                "unknown_rationale",
                "Boundary rationale has unresolved evidence",
            )
        } else {
            (
                "recorded_rationale",
                "Boundary has recorded accepted rationale",
            )
        };
        let mut finding=Finding {schema:CONTRACT.into(),id:String::new(),repository:record.run.repository.clone(),perspective:"architecture-rationale".into(),rule:rule.into(),semantic_anchor:b.boundary.clone(),title:title.into(),severity:if rule=="conflicting_rationale"{Severity::Medium}else{Severity::Info},rationale:format!("Boundary {}: {}. {} retained ADR records; {} unresolved evidence conditions; {} conflicting decision keys. Inspect the retained explanation and source references.",b.boundary,b.deployment_observation,b.rationale.len(),b.unknowns.len(),b.conflicting_decision_keys.len()),confidence:Confidence::Unknown,evidence:evidence.into_iter().collect(),inference:"Graph layers and deployment configuration are observed declarations; a candidate quantum is inferred, not proof of runtime independence. ADR status/choices are human-recorded evidence, never execution permission or agent approval.".into(),scope_digest:record.run.scope_digest.clone(),limitations:vec!["Only selected admitted Compose JSON and Markdown/TOML ADR documents are analyzed. Other deployment forms or semantic contradictions require human review.".into()]};
        finding.id = finding.identity()?;
        record.findings.push(finding);
    }
    record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    record.validate()?;
    let mut report = RationaleReport {
        schema: VERSION.into(),
        selection,
        graph,
        boundaries: explanations,
        analysis_complete: complete,
        record,
        digest: String::new(),
    };
    report.digest = hash(&report)?;
    Ok(report)
}
impl RationaleReport {
    pub fn validate(&self, store: &Store) -> Result<()> {
        ensure!(
            *self
                == architecture_rationale_reporter(
                    store,
                    self.graph.clone(),
                    self.selection.clone()
                )?,
            "rationale_artifact_mismatch"
        );
        Ok(())
    }
}
pub fn write_report(report: &RationaleReport, store: &Store, path: &std::path::Path) -> Result<()> {
    use std::io::Write;
    report.validate(store)?;
    structure::safe_artifact_path(path)?;
    let bytes = serde_json::to_vec(report)?;
    ensure!(bytes.len() as u64 <= LIMIT, "rationale_artifact_too_large");
    let mut o = std::fs::OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    let mut f = o
        .open(path)
        .map_err(|_| anyhow::anyhow!("rationale_output_unavailable"))?;
    f.write_all(&bytes)?;
    f.sync_all()?;
    Ok(())
}
pub fn read_report(store: &Store, path: &std::path::Path) -> Result<RationaleReport> {
    use std::io::Read;
    structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "rationale_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(LIMIT + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= LIMIT, "rationale_artifact_too_large");
    let r: RationaleReport = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow::anyhow!("invalid_rationale_artifact"))?;
    r.validate(store)?;
    Ok(r)
}
