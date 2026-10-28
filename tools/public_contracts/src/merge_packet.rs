//! Retained merge packet consistency, never live merge authority.
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use std::{fs, path::Path};
pub const REQUIRED_FILES: &[&str] = &[
    "README.md",
    "MERGE_READINESS_PROOF_PACKET_v0.91.3.md",
    "ct_demo_001_merge_gate.md",
    "ct_demo_001_merge_gate_snapshot.json",
];
pub const REQUIRED_GATE_SNIPPETS: &[&str] = &[
    "## Gate Identity",
    "## Issue / Branch / Worktree Truth",
    "## PR / CI Truth",
    "## Review Truth",
    "## Evidence Bundle Link",
    "## Structured Snapshot",
    "## Blocked Conditions",
    "## Decision",
    "human merge review remains required",
    "`merge_ready`",
];
pub const SNAPSHOT: &str = r#"{"transition_id": "cts.v0_91_3.issue_3200.ct_demo_001", "gate_kind": "governed_merge_readiness_gate.v1", "decision_mode": "reviewable_record", "outcome": "merge_ready", "issue_number": 3203, "issue_state": "CLOSED", "pr_number": 3243, "pr_state": "MERGED", "base_branch": "main", "adl_ci": "SUCCESS", "adl_coverage": "SUCCESS", "bounded_review_outcome": "PASS", "open_bounded_review_findings": 0, "human_merge_review_required": true, "decision": "merge_ready"}"#;
fn relative(path: &str) -> Result<()> {
    ensure!(
        !path.starts_with('/') && !path.starts_with("../") && !path.contains("/../"),
        "not a safe repo-relative path"
    );
    Ok(())
}
pub fn check(root: &Path, repo: &Path) -> Result<()> {
    for name in REQUIRED_FILES {
        ensure!(root.join(name).is_file(), "missing required file: {name}");
    }
    let text = fs::read_to_string(root.join("ct_demo_001_merge_gate.md"))?;
    for snippet in REQUIRED_GATE_SNIPPETS {
        ensure!(
            text.contains(snippet),
            "gate record missing required snippets: {snippet}"
        );
    }
    let snapshot: Value = serde_json::from_str(&fs::read_to_string(
        root.join("ct_demo_001_merge_gate_snapshot.json"),
    )?)
    .context("snapshot is not valid JSON")?;
    let expected: Value = serde_json::from_str(SNAPSHOT)?;
    for (key, value) in expected.as_object().unwrap() {
        ensure!(
            &snapshot[key] == value,
            "snapshot field '{key}' mismatch: expected {value}, found {}",
            snapshot[key]
        );
    }
    let mut paths = Vec::new();
    for key in [
        "evidence_bundle_rel_path",
        "review_synthesis_rel_path",
        "sor_rel_path",
    ] {
        let path = snapshot[key]
            .as_str()
            .context("path missing required string value")?;
        relative(path)?;
        if key != "sor_rel_path" {
            ensure!(
                repo.join(path).is_file(),
                "missing required tracked file: {path}"
            );
        }
        paths.push(path);
    }
    // Identity fields are fixed by SNAPSHOT above. SOR existence was never a
    // requirement of this historical record; do not claim fresh owner evidence.
    let mut markers = vec![
        "cts.v0_91_3.issue_3200.ct_demo_001",
        "[#3203]",
        "[#3243]",
        "PR state: `MERGED`",
        "base branch: `main`",
        "`adl-ci`: `SUCCESS`",
        "`adl-coverage`: `SUCCESS`",
        "decision: `merge_ready`",
        "outcome: `merge_ready`",
    ];
    markers.extend(paths);
    for marker in markers {
        ensure!(
            text.contains(marker),
            "gate record drifted from structured snapshot: {marker}"
        );
    }
    Ok(())
}
