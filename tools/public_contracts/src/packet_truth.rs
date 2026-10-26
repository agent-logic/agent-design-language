//! Historical packet truth boundaries, not proof of fresh execution.
use anyhow::{ensure, Context, Result};
use serde_json::Value;
use std::{fs, path::Path};
pub const CONSOLE_REQUIRED_FILES: &[&str] = &[
    "README.md",
    "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
    "ct_demo_003_sprint_console_storyboard.md",
    "ct_demo_003_sprint_console_proof_report.md",
];
pub const CONSOLE_REQUIRED_SNIPPETS: &[(&str, &[&str])] = &[
    (
        "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md",
        &[
            "# Five-Minute Sprint Console Packet v0.91.3",
            "## Demo Identity",
            "## Claims",
            "## Non-Claims",
            "## Run Path",
            "## Timebox Truth",
            "## Result Classification",
        ],
    ),
    (
        "ct_demo_003_sprint_console_storyboard.md",
        &[
            "# Sprint Console Storyboard",
            "Compressed Replay Clock",
            "Role Rail",
            "Review And Friction",
        ],
    ),
    (
        "ct_demo_003_sprint_console_proof_report.md",
        &[
            "# Sprint Console Proof Report",
            "What the Demo Proves",
            "What the Demo Suggests",
            "What the Demo Does Not Prove",
        ],
    ),
];
pub const READINESS_REQUIRED_FILES: &[&str] = &[
    "README.md",
    "FIRST_PROOF_READINESS_PACKET_v0.91.3.md",
    "ct_demo_001_first_proof_readiness.md",
];
pub const READINESS_REQUIRED_SECTIONS: &[&str] = &[
    "## Readiness Identity",
    "## Upstream Proof Inputs",
    "## Combined-Lane Readiness Checks",
    "## Closeout-Truth Lessons",
    "## Readiness Decision",
    "## Deferred / Non-Claims",
];
pub const READINESS_REQUIRED_PROOF_PATHS: &[&str] = &["docs/milestones/v0.91.3/review/transition_manifest/fixtures/valid_cognitive_transition_manifest_v1.json", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/README.md", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sip.md", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/stp.md", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/spp.md", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/srp.md", "docs/milestones/v0.91.3/review/evidence/csdlc/issues/issue-3201-card-lifecycle-demo/cards/sor.md", "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_transition_dag.md", "docs/milestones/v0.91.3/review/transition_dag/ct_demo_001_shard_plan.md", "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md", "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_review_synthesis.md", "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md", "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json", "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.md"];
pub const READINESS_REQUIRED_SNIPPETS: &[&str] = &[
    "[#3203]",
    "[#3204]",
    "[#3205]",
    "[#3243]",
    "[#3244]",
    "[#3247]",
    "combined-lane validation",
    "closeout-truth",
    "ready_for_wp09",
];
pub const READINESS_EXPECTED_MANIFEST_PATHS: &[(&str, &str)] = &[
    (
        "evidence_bundle_rel_path",
        "docs/milestones/v0.91.3/review/evidence_bundle/ct_demo_001_evidence_bundle.md",
    ),
    (
        "merge_readiness_gate_rel_path",
        "docs/milestones/v0.91.3/review/merge_readiness/ct_demo_001_merge_gate.md",
    ),
    (
        "obsmem_handoff_rel_path",
        "docs/milestones/v0.91.3/review/obsmem_handoff/ct_demo_001_obsmem_handoff.json",
    ),
];
pub const MEMORY_REQUIRED_FILES: &[(&str, &[&str])] = &[
    (
        "README.md",
        &[
            "# ObsMem Handoff Review Packet",
            "## Primary Proof Surfaces",
        ],
    ),
    (
        "OBSMEM_HANDOFF_PROOF_PACKET_v0.91.3.md",
        &[
            "# v0.91.3 ObsMem Handoff Proof Packet",
            "## Focused Validation",
        ],
    ),
    (
        "ct_demo_001_obsmem_handoff.md",
        &[
            "# CT Demo 001 ObsMem Handoff",
            "## Source Truth Boundary",
            "## SRP Review Learning Memory",
            "## SOR Outcome Truth Memory",
            "## Deferred / Outside Memory",
        ],
    ),
];
fn read(root: &Path, name: &str) -> Result<String> {
    fs::read_to_string(root.join(name))
        .with_context(|| format!("read required packet file: {name}"))
}
fn files(root: &Path, names: &[&str]) -> Result<()> {
    for name in names {
        ensure!(root.join(name).is_file(), "missing required file: {name}");
    }
    Ok(())
}
fn snippets(text: &str, required: &[&str], label: &str) -> Result<()> {
    for snippet in required {
        ensure!(text.contains(snippet), "{label}: missing {snippet}");
    }
    Ok(())
}
fn documents(root: &Path, required: &[(&str, &[&str])]) -> Result<()> {
    for (name, markers) in required {
        snippets(&read(root, name)?, markers, name)?;
    }
    Ok(())
}
fn unquote(text: &str) -> &str {
    text.strip_prefix('`')
        .and_then(|s| s.strip_suffix('`'))
        .unwrap_or(text)
}
fn field<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    text.lines()
        .find_map(|s| s.trim().strip_prefix(prefix).map(|s| unquote(s.trim())))
}
pub fn console(root: &Path) -> Result<()> {
    files(root, CONSOLE_REQUIRED_FILES)?;
    documents(root, CONSOLE_REQUIRED_SNIPPETS)?;
    let packet = read(root, "FIVE_MINUTE_SPRINT_CONSOLE_PACKET_v0.91.3.md")?;
    let report = read(root, "ct_demo_003_sprint_console_proof_report.md")?;
    let mut lines = report.lines();
    let result = if lines.any(|s| s.trim() == "## Result") {
        lines.map(str::trim).find(|s| !s.is_empty()).map(unquote)
    } else {
        None
    };
    ensure!(
        result == Some("passed"),
        "sprint console proof report must keep top-level result `passed`"
    );
    ensure!(
        field(&packet, "- run status:") == Some("passed"),
        "packet run status must stay `passed`"
    );
    ensure!(
        field(&packet, "- evidence type:") == Some("estimated"),
        "timebox truth must stay `estimated`"
    );
    ensure!(
        packet.contains("`partial`") && report.contains("`partial`"),
        "literal five-minute claim must remain explicitly partial"
    );
    Ok(())
}
pub fn readiness(root: &Path, repo: &Path) -> Result<()> {
    files(root, READINESS_REQUIRED_FILES)?;
    let record = read(root, "ct_demo_001_first_proof_readiness.md")?;
    snippets(
        &record,
        READINESS_REQUIRED_SECTIONS,
        "readiness record missing required sections",
    )?;
    snippets(
        &record,
        READINESS_REQUIRED_SNIPPETS,
        "readiness record missing required snippets",
    )?;
    ensure!(
        !record.contains(".adl/docs/TBD"),
        "readiness record must not rely on local-only .adl/docs/TBD notes"
    );
    files(repo, READINESS_REQUIRED_PROOF_PATHS)?;
    let manifest: Value = serde_json::from_str(&read(repo, READINESS_REQUIRED_PROOF_PATHS[0])?)?;
    for (key, expected) in READINESS_EXPECTED_MANIFEST_PATHS {
        ensure!(
            manifest[key].as_str() == Some(expected),
            "manifest field {key} mismatch"
        );
    }
    Ok(())
}
fn relative(value: &str) -> Result<()> {
    ensure!(!value.trim().is_empty(), "path must be non-empty");
    ensure!(
        !value.starts_with('/') && !value.contains([':', '\\']) && !value.contains(".."),
        "path must be repo-relative without traversal"
    );
    ensure!(
        !value.starts_with(".adl/"),
        "must cite tracked packet artifacts, not local-only .adl paths"
    );
    Ok(())
}
pub fn memory(root: &Path, repo: &Path) -> Result<()> {
    documents(root, MEMORY_REQUIRED_FILES)?;
    let data: Value = serde_json::from_str(&read(root, "ct_demo_001_obsmem_handoff.json")?)?;
    ensure!(
        data["schema_version"] == "srp_sor_obsmem_handoff.v1",
        "unexpected schema_version"
    );
    ensure!(
        data["transition_id"] == "cts.v0_91_3.issue_3200.ct_demo_001",
        "unexpected transition_id"
    );
    for key in [
        "evidence_bundle_rel_path",
        "review_synthesis_rel_path",
        "merge_readiness_gate_rel_path",
    ] {
        relative(
            data["tracked_supporting_artifacts"][key]
                .as_str()
                .context("missing tracked supporting artifact")?,
        )?;
    }
    for (name, kind, truth, suffix) in [
        (
            "srp_memory_entry",
            "srp_review_learning",
            "derived_from_final_srp",
            "cards/srp.md",
        ),
        (
            "sor_memory_entry",
            "sor_outcome_truth",
            "derived_from_final_sor",
            "cards/sor.md",
        ),
    ] {
        let entry = &data[name];
        ensure!(entry["entry_kind"] == kind, "unexpected entry_kind");
        ensure!(entry["source_truth"] == truth, "unexpected source_truth");
        let source = entry["source_record_rel_path"]
            .as_str()
            .context("must declare source_record_rel_path")?;
        relative(source)?;
        ensure!(
            source.ends_with(suffix),
            "source_record_rel_path must end with {suffix}"
        );
        ensure!(
            repo.join(source).is_file(),
            "source_record_rel_path does not exist in the repo"
        );
        let citations = entry["citations"]
            .as_array()
            .context("must have citations")?;
        ensure!(
            citations.iter().any(|v| v.as_str() == Some(source)),
            "citations must include source_record_rel_path"
        );
        for citation in citations {
            relative(citation.as_str().context("citation must be a string")?)?;
        }
    }
    ensure!(
        data["outside_memory"]
            .as_array()
            .is_some_and(|a| !a.is_empty()),
        "outside_memory must be a non-empty list"
    );
    Ok(())
}
pub fn cli(kind: &str, args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("usage: {kind} <packet-root>");
        return if kind == "sprint-console" { 2 } else { 1 };
    }
    let root = Path::new(&args[0]);
    let result = match kind {
        "sprint-console" => console(root),
        "first-proof-readiness" => readiness(root, &crate::root()),
        "obsmem-handoff" => memory(root, &crate::root()),
        _ => Err(anyhow::anyhow!("unknown packet kind")),
    };
    match result {
        Err(e) => {
            eprintln!("{kind}: FAIL {e:#}");
            1
        }
        Ok(()) => {
            match kind {
                "sprint-console" => println!("five-minute-sprint-console-packet: PASS"),
                "first-proof-readiness" => {
                    println!("first_proof_readiness_packet: PASS root={}", root.display())
                }
                _ => println!("obsmem_handoff_packet: PASS root={}", root.display()),
            }
            0
        }
    }
}

pub const HTML_FILES: &[&str] = &[
    "README.md",
    "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md",
    "ct_demo_002_starharvest_design_note.md",
    "ct_demo_002_starharvest_implementation_summary.md",
    "ct_demo_002_starharvest_qa_checklist.md",
    "ct_demo_002_starharvest_proof_report.md",
];
pub const HTML_SNIPPETS: &[(&str, &[&str])] = &[
    (
        "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md",
        &[
            "# Five-Minute HTML Game Packet v0.91.3",
            "Result Vocabulary",
            "What This Demo Proves",
            "What This Demo Suggests",
            "What This Demo Does Not Prove",
            "Primary Demo Command",
        ],
    ),
    (
        "ct_demo_002_starharvest_design_note.md",
        &[
            "# Starharvest Design Note",
            "Visual Thesis",
            "Gameplay Loop",
            "Scope Cuts",
        ],
    ),
    (
        "ct_demo_002_starharvest_qa_checklist.md",
        &[
            "# Starharvest QA Checklist",
            "Observed Evidence Status",
            "Keyboard Control",
            "Win/loss",
        ],
    ),
    (
        "ct_demo_002_starharvest_proof_report.md",
        &[
            "# Starharvest Proof Report",
            "`partial`",
            "What the Demo Proves",
            "What the Demo Does Not Prove",
        ],
    ),
];
fn html_field(text: &str, prefix: &str, expected: &str) -> bool {
    let value = text
        .lines()
        .find_map(|s| s.trim().strip_prefix(prefix))
        .map(str::trim);
    value.is_some_and(|v| {
        let v = v
            .strip_prefix('`')
            .and_then(|rest| rest.split_once('`').map(|(s, _)| s))
            .unwrap_or(v);
        v == expected || v.starts_with(&format!("{expected},"))
    })
}
pub fn html_game(root: &Path) -> Result<()> {
    files(root, HTML_FILES)?;
    documents(root, HTML_SNIPPETS)?;
    let packet = read(root, "FIVE_MINUTE_HTML_GAME_PACKET_v0.91.3.md")?;
    let proof = read(root, "ct_demo_002_starharvest_proof_report.md")?;
    let qa = read(root, "ct_demo_002_starharvest_qa_checklist.md")?;
    let mut lines = proof.lines();
    let result = if lines.any(|s| s.trim() == "## Result") {
        lines.map(str::trim).find(|s| !s.is_empty()).map(unquote)
    } else {
        None
    };
    ensure!(
        result == Some("partial"),
        "proof report result must stay `partial` until browser/gameplay proof is fully captured"
    );
    for (text, prefix, expected) in [
        (
            &packet,
            "- browser/gameplay proof in captured environment:",
            "partial",
        ),
        (
            &proof,
            "- full browser/gameplay proof in captured environment:",
            "partial",
        ),
        (
            &qa,
            "- browser load and interactive gameplay behavior in captured environment:",
            "not run",
        ),
        (&qa, "- helper command prints a valid local URL:", "passed"),
    ] {
        ensure!(
            html_field(text, prefix, expected),
            "{prefix} must record `{expected}`"
        );
    }
    for claim in [
        "a bounded C-SDLC mini-sprint can produce a real playable browser artifact",
        "the mini-sprint produced a real playable browser artifact",
    ] {
        ensure!(
            !packet.contains(claim) && !proof.contains(claim),
            "overclaim retained after downgrade"
        );
    }
    Ok(())
}
