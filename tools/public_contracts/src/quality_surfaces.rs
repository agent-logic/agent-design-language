//! Retained review surface consistency; command strings are never executed.
use anyhow::{ensure, Context, Result};
use std::{fs, path::Path};
pub type Surface = (
    &'static str,
    &'static str,
    &'static str,
    &'static [&'static str],
);
pub const SURFACES: &[Surface] = &[
    (
        "quality_gate_doc",
        "QUALITY_GATE_v0.91.3.md",
        "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md",
        &[
            "## Primary Run Path",
            "bash adl/tools/demo_v0913_quality_gate.sh",
            "## Current Gate Dimensions",
            "## Review Gate",
            "## Blockers",
            "## Non-Claims",
            "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md",
        ],
    ),
    (
        "quality_gate_packet",
        "review/quality_gate/QUALITY_GATE_PACKET_v0.91.3.md",
        "review/quality_gate/README.md",
        &[
            "## Scope",
            "## Packet Contents",
            "## Demo Command",
            "## Focused Validation",
            "## Current Gate Dimensions",
            "## Boundaries",
            "README.md",
            "bash adl/tools/demo_v0913_quality_gate.sh",
            "bash adl/tools/test_demo_v0913_quality_gate.sh",
        ],
    ),
    (
        "demo_coverage",
        "review/demo_coverage/DEMO_COVERAGE_PACKET_v0.91.3.md",
        "review/demo_coverage/ct_demo_006_feature_demo_map.md",
        &[
            "## Claim Boundary",
            "## Primary Artifact",
            "## Review Use",
            "## Validation",
            "every current `v0.91.3` feature has a bounded reviewer-facing demo or proof path",
            "ct_demo_006_feature_demo_map.md",
        ],
    ),
];
pub const ROUTES: &[(&str,&str)] = &[("Cognitive Transition Manifest","cargo test --manifest-path adl/Cargo.toml cognitive_transition_schema -- --nocapture"), ("Card Lifecycle Integration",".adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 3201"), ("Transition DAG And Shard Coordination","python3 adl/tools/validate_transition_dag_packet.py docs/milestones/v0.91.3/review/transition_dag"), ("Evidence Bundle And Review Synthesis","python3 adl/tools/validate_evidence_bundle_packet.py docs/milestones/v0.91.3/review/evidence_bundle"), ("Governed Merge-Readiness Gate","python3 adl/tools/validate_merge_readiness_packet.py docs/milestones/v0.91.3/review/merge_readiness"), ("SRP/SOR ObsMem Handoff","python3 adl/tools/validate_obsmem_handoff_packet.py docs/milestones/v0.91.3/review/obsmem_handoff"), ("Integrated Process Lessons And Proof Readiness","python3 adl/tools/validate_first_proof_readiness_packet.py docs/milestones/v0.91.3/review/first_proof_readiness"), ("Five-Minute Sprint First Proof","python3 adl/tools/demo_v0913_first_proof_demo.py"), ("C-SDLC Demo Proof Contract","python3 adl/tools/validate_csdlc_demo_proof_contract_packet.py docs/milestones/v0.91.3/review/csdlc_demo_proof_contract")];
pub fn check(repo: &Path, surface: &str) -> Result<()> {
    let root = repo.join("docs/milestones/v0.91.3");
    let (_, path, linked, markers) = SURFACES
        .iter()
        .find(|row| row.0 == surface)
        .context("unknown review surface")?;
    let text = fs::read_to_string(root.join(path))?;
    for marker in *markers {
        ensure!(text.contains(marker), "missing required snippet: {marker}");
    }
    ensure!(root.join(linked).is_file(), "missing linked packet surface");
    if surface == "demo_coverage" {
        let text = fs::read_to_string(root.join(linked))?;
        for (feature, route) in ROUTES {
            ensure!(
                text.contains(feature) && text.contains(route),
                "demo coverage map route mismatch: {feature} -> {route}"
            );
        }
        ensure!(
            text.contains("Missing feature with no truthful demo/proof route: none found"),
            "demo coverage map must record the missing-feature verdict"
        );
    }
    Ok(())
}
