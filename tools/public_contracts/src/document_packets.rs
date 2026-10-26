//! Supported architecture/showcase packet checks; structural/demo claims only.
use anyhow::{ensure, Context, Result};
use regex::Regex;
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};
pub const REQUIRED_DOCS: &[&str] = &[
    "docs/architecture/README.md",
    "docs/architecture/ADL_ARCHITECTURE.md",
    "docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md",
    "docs/architecture/ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md",
    "docs/architecture/adr/README.md",
    "docs/architecture/adr/CANDIDATE_ADRS.md",
    "docs/architecture/diagrams/README.md",
    "docs/architecture/diagrams/DIAGRAM_PACKET.md",
];
pub const REQUIRED_DIAGRAMS: &[&str] = &[
    "system_context.mmd",
    "runtime_lifecycle.mmd",
    "control_plane_lifecycle.mmd",
    "task_bundle_state.mmd",
    "skill_orchestration.mmd",
    "artifact_data_flow.mmd",
    "trust_boundaries.mmd",
];
pub const ARCHITECTURE_SECTIONS: &[&str] = &[
    "# ADL Architecture",
    "## Scope And Evidence",
    "## System Context",
    "## Runtime Model",
    "## Authoring And Control Plane",
    "## Task Bundle Lifecycle",
    "## Provider And Tool Boundaries",
    "## Trace And Artifact Truth",
    "## Security And Trust Boundaries",
    "## Long-Lived Agent Layer",
    "## Operational Skills",
    "## Review And Release Surfaces",
    "## Architecture Invariants",
    "## Known Gaps",
    "## Diagram Index",
    "## ADR Candidate Provenance",
];
pub const AUTOMATION_SECTIONS: &[&str] = &[
    "# Architecture Review Automation",
    "## Review Pipeline",
    "## Machine-Checkable Invariants",
    "## Human-Judgment Gates",
    "## Specialist Roles",
    "## Missing Or Backlog Skills",
    "## Automation Boundaries",
    "## Suggested CI Gates",
];
pub const PLAN_SECTIONS: &[&str] = &[
    "# Architecture Document Generation Plan",
    "## Goal",
    "## Inputs",
    "## Skill Order",
    "## Output Template",
    "## Diagram Template",
    "## Review Template",
    "## Machine Validation",
    "## Deferred Dependencies",
    "## Non-Goals",
];
pub const REQUIRED_FILES: &[&str] = &[
    "run_manifest.json",
    "repo_scope.md",
    "repo_inventory.json",
    "specialist_reviews/code.md",
    "specialist_reviews/security.md",
    "specialist_reviews/tests.md",
    "specialist_reviews/docs.md",
    "specialist_reviews/architecture.md",
    "specialist_reviews/dependencies.md",
    "diagrams/system_map.mmd",
    "diagrams/diagram_manifest.md",
    "diagrams/diagram_review.md",
    "redaction_report.md",
    "test_recommendations/test_gap_report.md",
    "issue_planning/issue_candidates.md",
    "adr_candidates/adr_candidates.md",
    "fitness_functions/fitness_function_plan.md",
    "final_report.md",
    "quality_evaluation.md",
    "demo_operator_result.json",
    "README.md",
];
pub const FINAL_REPORT_SECTIONS: &[&str] = &[
    "# CodeBuddy Review Report",
    "## Executive Summary",
    "## Review Scope",
    "## Top Findings",
    "## Architecture Summary",
    "## Security And Privacy Notes",
    "## Test Recommendations",
    "## Documentation And Onboarding Notes",
    "## Remediation Sequence",
    "## Residual Risks",
    "## Caveats",
];
pub fn scan(text: &str, google: bool) -> Result<()> {
    let host = Regex::new(
        r"(?:^|[^A-Za-z0-9._-])/(?:Users|home)/[^/\s`]+(?:/[^\s`]*)*|(?:^|[^A-Za-z0-9._-])/(?:private|var|tmp|etc|opt|srv|root)/[^\s`]+|[A-Za-z]:\\[^\s`]+",
    )?;
    ensure!(!host.is_match(text), "absolute host path leaked");
    let secret = Regex::new(
        r"OPENAI_API_KEY|ANTHROPIC_API_KEY|Bearer\s+[A-Za-z0-9._-]+|sk-[A-Za-z0-9]{8,}",
    )?;
    ensure!(
        !(secret.is_match(text) || google && text.contains("GOOGLE_API_KEY")),
        "secret-like marker leaked"
    );
    Ok(())
}
fn text(root: &Path, relative: &str) -> Result<String> {
    fs::read_to_string(root.join(relative))
        .with_context(|| format!("missing or unreadable required file: {relative}"))
}
fn json(root: &Path, relative: &str) -> Result<Value> {
    serde_json::from_str(&text(root, relative)?)
        .with_context(|| format!("invalid JSON: {relative}"))
}
fn includes(text: &str, markers: &[&str]) -> Result<()> {
    for marker in markers {
        ensure!(text.contains(marker), "required marker missing: {marker}");
    }
    Ok(())
}
pub fn ordered(text: &str, sections: &[&str]) -> Result<()> {
    let mut last = None;
    for section in sections {
        let index = text
            .find(section)
            .with_context(|| format!("missing section {section}"))?;
        ensure!(
            last.is_none_or(|last| index > last),
            "section out of order: {section}"
        );
        last = Some(index);
    }
    Ok(())
}
pub fn architecture(root: &Path, artifacts: Option<&Path>) -> Result<()> {
    for rel in REQUIRED_DOCS {
        scan(&text(root, rel)?, true).with_context(|| rel.to_string())?;
    }
    let architecture = text(root, "docs/architecture/ADL_ARCHITECTURE.md")?;
    ordered(&architecture, ARCHITECTURE_SECTIONS)?;
    includes(
        &architecture,
        &[
            "adl/src/adl/types.rs",
            "adl/src/execution_plan.rs",
            "adl/src/trace.rs",
            "adl/src/control_plane.rs",
            "adl/src/long_lived_agent.rs",
            "docs/default_workflow.md",
        ],
    )?;
    let automation = text(root, "docs/architecture/ARCHITECTURE_REVIEW_AUTOMATION.md")?;
    ordered(&automation, AUTOMATION_SECTIONS)?;
    includes(&automation, &["Machine-Checkable", "Human-Judgment"])?;
    let plan = text(
        root,
        "docs/architecture/ARCHITECTURE_DOCUMENT_GENERATION_PLAN.md",
    )?;
    ordered(&plan, PLAN_SECTIONS)?;
    includes(&plan, &["#2042", "#2044"])?;
    let packet = text(root, "docs/architecture/diagrams/DIAGRAM_PACKET.md")?;
    for diagram in REQUIRED_DIAGRAMS {
        let rel = format!("docs/architecture/diagrams/{diagram}");
        let source = text(root, &rel)?;
        scan(&source, true).with_context(|| rel.clone())?;
        includes(&source, &["%% Evidence:", "%% Assumptions:"])?;
        ensure!(
            source.contains("flowchart") || source.contains("stateDiagram"),
            "not recognized Mermaid source: {rel}"
        );
        includes(&packet, &[diagram])?;
    }
    if let Some(root) = artifacts {
        ensure!(root.is_dir(), "artifact root not found");
        for rel in [
            "architecture_generation_manifest.json",
            "architecture_review_note.md",
            "diagram_review_note.md",
            "threat_boundary_note.md",
            "fitness_function_note.md",
        ] {
            scan(&text(root, rel)?, true).with_context(|| rel.to_string())?;
        }
        let manifest = json(root, "architecture_generation_manifest.json")?;
        ensure!(
            manifest["schema_version"] == "adl.v090.architecture_document_generation_demo.v1",
            "architecture generation manifest schema mismatch"
        );
        ensure!(
            manifest["classification"] == "proving",
            "demo classification must be proving"
        );
        ensure!(
            manifest["diagram_sources"]
                .as_array()
                .is_some_and(|v| v.len() >= 7),
            "manifest lacks diagram sources"
        );
        ensure!(
            manifest["skills_represented"]
                .as_array()
                .is_some_and(|v| v.iter().any(|v| v == "repo-architecture-review")),
            "manifest lacks architecture review skill representation"
        );
    }
    Ok(())
}
pub fn showcase(root: &Path) -> Result<()> {
    ensure!(root.is_dir(), "artifact root not found");
    for rel in REQUIRED_FILES {
        ensure!(root.join(rel).is_file(), "missing required artifact: {rel}");
    }
    let manifest = json(root, "run_manifest.json")?;
    ensure!(
        manifest["schema_version"] == "codebuddy.review_showcase.v1",
        "run manifest schema mismatch"
    );
    ensure!(
        manifest["classification"] == "non_proving",
        "demo must remain non_proving"
    );
    ensure!(
        manifest["publication_allowed"] == false,
        "publication_allowed must be false"
    );
    let lanes = manifest["skill_lanes"]
        .as_array()
        .context("skill lanes must be an array")?;
    ensure!(lanes.len() >= 17, "skill lane count too small");
    let mut by_skill = BTreeMap::new();
    for lane in lanes {
        let skill = lane["skill"].as_str().context("lane skill missing")?;
        ensure!(
            by_skill.insert(skill, lane).is_none(),
            "duplicate skill lane"
        );
    }
    for (skill, status) in [
        ("review-quality-evaluator", "staged_pending_2070"),
        ("product-report-writer", "represented"),
        ("redaction-and-evidence-auditor", "represented"),
    ] {
        ensure!(
            by_skill.get(skill).is_some_and(|v| v["status"] == status),
            "skill lane state mismatch: {skill}"
        );
    }
    let inventory = json(root, "repo_inventory.json")?;
    ensure!(
        inventory["review_packet_policy"]["redaction_required_before_report"] == true,
        "inventory must require redaction before report"
    );
    let result = json(root, "demo_operator_result.json")?;
    ensure!(
        result["classification"] == "non_proving",
        "demo operator classification mismatch"
    );
    ensure!(
        result["prerequisite_state"]["review_quality_evaluator"] == "staged_pending_2070",
        "demo operator staged-lane truth missing"
    );
    let report = text(root, "final_report.md")?;
    ordered(&report, FINAL_REPORT_SECTIONS)?;
    includes(
        &report,
        &["[P1]", "[P2]", "publication_allowed=false", "#2070"],
    )?;
    includes(
        &text(root, "redaction_report.md")?,
        &["Publication allowed: false"],
    )?;
    includes(
        &text(root, "quality_evaluation.md")?,
        &["Staged pending #2070"],
    )?;
    for rel in REQUIRED_FILES {
        scan(&text(root, rel)?, false).with_context(|| rel.to_string())?;
    }
    Ok(())
}
