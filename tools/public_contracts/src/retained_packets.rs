//! Retained packet structure checks; no execution or lifecycle acceptance.
use anyhow::{ensure, Context, Result};
use std::{fs, path::Path};
pub type Requirements = &'static [(&'static str, &'static [&'static str])];
pub const EVIDENCE_BUNDLE: Requirements = &[
    (
        "README.md",
        &[
            "# Evidence Bundle Review Packet",
            "## Primary Proof Surfaces",
        ],
    ),
    (
        "EVIDENCE_BUNDLE_PROOF_PACKET_v0.91.3.md",
        &[
            "# Evidence Bundle Proof Packet v0.91.3",
            "## Focused Validation",
        ],
    ),
    (
        "ct_demo_001_evidence_bundle.md",
        &[
            "# CT Demo 001 Evidence Bundle",
            "## Transition Identity",
            "## Changed Artifact Inventory",
            "## Validation Record",
            "## Validation Not Run",
            "## Review Inputs",
            "## Review Findings",
            "## Finding Dispositions",
            "## Trace / Proof References",
            "## Residual Risks",
        ],
    ),
    (
        "ct_demo_001_review_synthesis.md",
        &[
            "# CT Demo 001 Review Synthesis",
            "## Conclusion",
            "## Confirmed Findings",
            "## Deferred Findings",
            "## Recommended Outcome",
        ],
    ),
];
pub const TRANSITION_DAG: Requirements = &[
    (
        "README.md",
        &[
            "# Transition DAG Review Packet",
            "## Primary Proof Surfaces",
        ],
    ),
    (
        "TRANSITION_DAG_PROOF_PACKET_v0.91.3.md",
        &[
            "# Transition DAG Proof Packet v0.91.3",
            "## Focused Validation",
        ],
    ),
    (
        "ct_demo_001_transition_dag.md",
        &[
            "# CT Demo 001 Transition DAG",
            "## Serial Nodes",
            "## Shard Nodes",
            "## Barrier Nodes",
            "barrier.review_barrier",
            "barrier.merge_readiness_barrier",
            "barrier.closeout_barrier",
            "coordination latency",
            "implementation time",
        ],
    ),
    (
        "ct_demo_001_shard_plan.md",
        &[
            "# CT Demo 001 Shard Plan",
            "## Shards",
            "## Interface Freeze Rules",
            "## Handoff Contracts",
            "## Barrier Contracts",
            "## Coordination Metrics Split",
        ],
    ),
];
pub const CSDLC_DEMO_PROOF_CONTRACT: Requirements = &[
    (
        "README.md",
        &[
            "## Focused Validation",
            "This packet defines the shared evidence contract",
        ],
    ),
    (
        "C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md",
        &[
            "## Required Packet Sections",
            "## Claim Ledger Rules",
            "## Result Classification",
            "## Timebox Truth Rules",
            "## Validation Minimums",
            "## Review Minimums",
        ],
    ),
    (
        "C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md",
        &[
            "## Demo Identity",
            "## Claims",
            "## Non-Claims",
            "## Run Path",
            "## Timebox Truth",
            "## Validation Evidence",
            "## Review Evidence",
            "## Result Classification",
        ],
    ),
];
pub fn requirements(kind: &str) -> Result<Requirements> {
    match kind {
        "evidence-bundle" => Ok(EVIDENCE_BUNDLE),
        "transition-dag" => Ok(TRANSITION_DAG),
        "csdlc-demo-proof-contract" => Ok(CSDLC_DEMO_PROOF_CONTRACT),
        _ => anyhow::bail!("unknown retained packet kind"),
    }
}
pub fn check(kind: &str, root: &Path) -> Result<()> {
    let required = requirements(kind)?;
    ensure!(
        root.is_dir(),
        "packet root does not exist: {}",
        root.display()
    );
    let mut failures = Vec::new();
    for (relative, snippets) in required {
        let path = root.join(relative);
        if !path.is_file() {
            failures.push(format!("missing file: {relative}"));
            continue;
        }
        let text = fs::read_to_string(&path).with_context(|| format!("read {relative}"))?;
        for snippet in *snippets {
            if !text.contains(snippet) {
                failures.push(format!("{relative} missing snippet: {snippet}"));
            }
        }
    }
    ensure!(failures.is_empty(), "{}", failures.join("\n"));
    Ok(())
}
pub fn cli(kind: &str, args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("usage: {kind} <packet-root>");
        return if kind == "csdlc-demo-proof-contract" {
            2
        } else {
            1
        };
    }
    match check(kind, Path::new(&args[0])) {
        Err(error) => {
            eprintln!("{error:#}");
            1
        }
        Ok(()) => {
            match kind {
                "evidence-bundle" => println!("evidence_bundle_packet: PASS root={}", args[0]),
                "transition-dag" => println!("transition_dag_packet: PASS root={}", args[0]),
                _ => println!("packet ok: {}", args[0]),
            }
            0
        }
    }
}
