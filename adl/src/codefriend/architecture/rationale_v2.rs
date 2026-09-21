//! Admitted deployment/ADR evidence over v2 source modules; no approval authority.
pub use super::rationale::{BoundaryExplanation, BoundarySelection, Decision, RationaleSource};
use super::{structure::Location, structure_v2::StructureReportV2};
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
use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    path::Path,
};
pub const VERSION: &str = "codefriend.rationale.v2";
const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationaleSelectionV2 {
    pub schema: String,
    pub graph_digest: String,
    pub revision: String,
    pub boundaries: Vec<BoundarySelection>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RationaleReportV2 {
    pub schema: String,
    pub selection: RationaleSelectionV2,
    pub graph: StructureReportV2,
    pub boundaries: Vec<BoundaryExplanation>,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
struct Bounded(Vec<u8>);
impl Write for Bounded {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if b.len() > MAX_BYTES.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("rationale_artifact_too_large"));
        }
        self.0.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn bounded<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    let mut b = Bounded(vec![]);
    serde_json::to_writer(&mut b, v)?;
    Ok(b.0)
}
fn live(store: &Store, g: &StructureReportV2) -> Result<()> {
    ensure!(
        store.get(&g.record.admission.packet.packet_id)? == g.record.admission,
        "rationale_admission_changed"
    );
    Ok(())
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
fn source<'a>(g: &'a StructureReportV2, path: &str) -> Result<Option<(&'a str, Location)>> {
    validate_path(path)?;
    let a = &g.record.admission;
    if !a.packet.scope.context.iter().any(|p| p == path) {
        return Ok(None);
    }
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
    let v = serde_json::from_str::<crate::codefriend::schema::UniqueValue>(content)
        .map_err(|_| anyhow::anyhow!("invalid_deployment_json"))?
        .0;
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
pub fn architecture_rationale_reporter_v2(
    store: &Store,
    graph: StructureReportV2,
    mut selection: RationaleSelectionV2,
    now: u64,
) -> Result<RationaleReportV2> {
    graph.validate(store, now)?;
    live(store, &graph)?;
    let mut materialized = bounded(&graph)?.len()
        + bounded(&graph.record.admission)?.len()
        + bounded(&selection)?.len();
    ensure!(materialized <= MAX_BYTES, "rationale_artifact_too_large");
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
            .filter(|n| n.layer == selected.boundary)
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
            materialized = materialized
                .checked_add(explanation.len() + 2048)
                .ok_or_else(|| anyhow::anyhow!("rationale_artifact_too_large"))?;
            ensure!(materialized <= MAX_BYTES, "rationale_artifact_too_large");
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
        materialized = materialized
            .checked_add(bounded(&b)?.len())
            .ok_or_else(|| anyhow::anyhow!("rationale_artifact_too_large"))?;
        ensure!(materialized <= MAX_BYTES, "rationale_artifact_too_large");
        explanations.push(b);
    }
    let complete = false; // Source-module coverage remains partial; never promote ADR evidence to authority.
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
    let mut report = RationaleReportV2 {
        schema: VERSION.into(),
        selection,
        graph,
        boundaries: explanations,
        analysis_complete: complete,
        record,
        digest: String::new(),
    };
    bounded(&report)?;
    report.digest = hash(&report)?;
    bounded(&report)?;
    live(store, &report.graph)?;
    Ok(report)
}
impl RationaleReportV2 {
    pub fn validate(&self, store: &Store, now: u64) -> Result<()> {
        bounded(self)?;
        let expected = architecture_rationale_reporter_v2(
            store,
            self.graph.clone(),
            self.selection.clone(),
            now,
        )?;
        ensure!(*self == expected, "rationale_artifact_mismatch");
        live(store, &self.graph)
    }
}
pub fn write_report_v2(
    report: &RationaleReportV2,
    store: &Store,
    path: &Path,
    now: u64,
) -> Result<()> {
    report.validate(store, now)?;
    super::structure::safe_artifact_path(path)?;
    let data = bounded(report)?;
    live(store, &report.graph)?;
    let mut options = std::fs::OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    let result = (|| -> Result<()> {
        file.write_all(&data)?;
        file.sync_all()?;
        live(store, &report.graph)
    })();
    if result.is_err() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if let (Ok(a), Ok(b)) = (file.metadata(), std::fs::symlink_metadata(path)) {
                if b.file_type().is_file() && a.dev() == b.dev() && a.ino() == b.ino() {
                    std::fs::remove_file(path)?;
                }
            }
        }
    }
    result
}
pub fn read_report_v2(store: &Store, path: &Path, now: u64) -> Result<RationaleReportV2> {
    super::structure::safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "rationale_input_not_regular"
    );
    let mut data = vec![];
    std::fs::File::open(path)?
        .take(MAX_BYTES as u64 + 1)
        .read_to_end(&mut data)?;
    ensure!(data.len() <= MAX_BYTES, "rationale_artifact_too_large");
    let report: RationaleReportV2 = serde_json::from_slice(&data)?;
    report.validate(store, now)?;
    live(store, &report.graph)?;
    Ok(report)
}
