//! Deterministic source inventory, not installed owner admission.
use anyhow::{Context, Result};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const PRIMARY_BINARIES: &[(&str, &str)] = &[
    (
        "adl",
        "Public ADL planning and installed-product CLI declared by this repository.",
    ),
    (
        "adl-process",
        "Public permission-safe process-status helper declared by this repository.",
    ),
];
const PRIMARY_TOOL_SCRIPTS: &[(&str, &str)] = &[
    (
        "batched_checks.sh",
        "Local validation entrypoint; use the current declared public validation contract.",
    ),
    (
        "run_pr_fast_test_lane.sh",
        "Primary focused PR-fast local test runner.",
    ),
];
const HISTORICAL_TOOL_SCRIPTS: &[(&str, &str)] = &[
    ("run_aws_spot_remote_validation_lane.sh", "Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route."),
    ("test_run_aws_spot_ci_profile.sh", "Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route."),
    ("run_issue509_gcp_drt_d_qualification.sh", "Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route."),
    ("run_pr_fast_test_lane.sh", "Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing."),
    ("test_validation_inventory.sh", "Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing."),
    ("validate_v092_runtime_guardian_lifecycle.sh", "Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing."),
    ("run_v0914_multi_agent_workcell_proof.sh", "Historical model-backed v0.91.4 workcell rehearsal; retained manifest and review artifacts do not qualify a new execution, including its provider-active dry-run path."),
    ("demo_d11_signed_remote.sh", "Historical signed-remote contract; current public ADL does not supply its Runtime binary or test interfaces."),
    ("demo_v087_operational_skills.sh", "Historical bundle-install and inventory contract; original recursive acceptance remains separate from owner resource proof."),
];
const HISTORICAL_EVIDENCE: &[(&str, &str)] = &[
    (
        "docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md",
        "Historical command-inventory evidence and prior ownership table.",
    ),
    (
        "docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md",
        "Historical migration contract for wrapper/public workflow spine truth.",
    ),
    (
        "docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md",
        "Historical compatibility-sunset policy and active-bundle scan gate.",
    ),
    (
        "docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md",
        "Historical proof packet for the validator/editor small-binary slice.",
    ),
];
const OWNER_NAVIGATION: &[(&str, &str, &str, &str)] = &[
    ("https://github.com/agent-logic/agent-logic-infrastructure", "owner documentation", "primary", "Infrastructure manual proof owner; retained source and resolver checks do not qualify live behavior."),
    ("https://github.com/agent-logic/cognitive-sdlc", "owner documentation", "primary", "C-SDLC v3 lifecycle owner. Use its installed operator manual and authenticated authority; source presence grants no authority."),
    ("https://github.com/agent-logic/agent-logic-runtime", "owner documentation", "primary", "Runtime product owner; installation and product qualification are separate from this source inventory."),
    ("https://github.com/agent-logic/codefriend", "owner documentation", "primary", "CodeFriend review product owner; this registry does not invoke or qualify it."),
];
const HEADER: &str = "# ADL Source Tool Surface Registry\n\nHistorical baseline: issue #3734; its v0.91.5 snapshot remains unchanged.\nStatus: generated_live_registry\n\n## Summary\n\nThis registry records the current visible ADL toolkit surface for the\nselected source checkout. It is a live source inventory intended to\ndrive later simplification cuts without confusing primary workflow\nentrypoints, supported compatibility shims, internal helpers, and\nhistorical evidence artifacts.\n\nThis registry distinguishes active workflow spines from already-retired\nwrapper residue so later cuts can stay evidence-bound.\n\n## Generation Command\n\n```bash\nbash adl/tools/generate_tool_surface_registry.sh\n```\n\n## Freshness Check\n\n```bash\nbash adl/tools/generate_tool_surface_registry.sh --check\n```\n\n";
const FOOTER: &str = "## Non-Claims\n\n- This registry does not approve final deletion of retired wrappers.\n- This registry does not prove active-reference absence for any shim.\n- This registry does not replace the later simplification review and scan issues.\n";
const OUTPUT: &str = "docs/tooling/TOOL_SURFACE_REGISTRY.md";
type Row = (String, String, String, String);
fn lookup<'a>(items: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    items.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}
fn tool(name: &str) -> (&str, &str) {
    if let Some(notes) = lookup(HISTORICAL_TOOL_SCRIPTS, name) {
        return ("historical evidence", notes);
    }
    if let Some(notes) = lookup(PRIMARY_TOOL_SCRIPTS, name) {
        return ("primary", notes);
    }
    let notes = if name.starts_with("test_") {
        "Focused regression or contract test helper."
    } else if name.starts_with("demo_") {
        "Demo/proof helper rather than a primary workflow entrypoint."
    } else if name.starts_with("mock_") {
        "Mock helper used for focused testing or proof."
    } else if ["check_", "validate_", "verify_", "run_"]
        .iter()
        .any(|p| name.starts_with(p))
    {
        "Bounded validation/generation helper used by primary workflow entrypoints."
    } else if name.ends_with(".py") {
        "Python helper or generator used by a bounded tooling surface."
    } else if name.ends_with(".sh") {
        "Shell helper retained for bounded workflow support."
    } else {
        "Visible tool surface retained for bounded support work."
    };
    ("internal helper", notes)
}
fn skill(name: &str) -> (&str, &str) {
    if name == "pr-run" {
        return ("historical evidence", "Partial historical ADL compatibility reference; use the selected native owner for current lifecycle commands.");
    }
    if [
        "csdlc-doctor",
        "pr-init",
        "pr-ready",
        "pr-finish",
        "pr-closeout",
        "pr-janitor",
        "sprint-conductor",
    ]
    .contains(&name)
    {
        return (
            "primary",
            "Primary skill in the governed issue/sprint lifecycle.",
        );
    }
    if name.ends_with("-editor") {
        return (
            "supported shim",
            "Lifecycle editor skill used to normalize durable card truth.",
        );
    }
    ("internal helper", "Specialized skill available for bounded tasks but not part of the primary issue workflow spine.")
}
fn children(path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(path)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();
    Ok(paths)
}
fn has_file(path: &Path) -> Result<bool> {
    for child in children(path)? {
        if child.is_file() {
            return Ok(true);
        }
        // Match non-following recursive glob behavior for directory symlinks.
        if child.is_dir() && !child.is_symlink() && has_file(&child)? {
            return Ok(true);
        }
    }
    Ok(false)
}
fn row(surface: &str, kind: &str, classification: &str, notes: &str) -> Row {
    (
        surface.into(),
        kind.into(),
        classification.into(),
        notes.into(),
    )
}
fn table(rows: &[Row]) -> String {
    let mut text =
        "| Surface | Kind | Classification | Notes |\n| --- | --- | --- | --- |\n".to_owned();
    for (surface, kind, class, notes) in rows {
        text += &format!("| `{surface}` | {kind} | {class} | {notes} |\n");
    }
    text
}
pub fn build(root: &Path) -> Result<String> {
    let manifest: toml::Value = toml::from_str(&fs::read_to_string(root.join("adl/Cargo.toml"))?)?;
    let mut bins = Vec::new();
    if let Some(entries) = manifest.get("bin").and_then(toml::Value::as_array) {
        for entry in entries {
            let name = entry
                .get("name")
                .and_then(toml::Value::as_str)
                .context("explicit binary lacks name")?;
            bins.push(row(name, "owner binary", "primary", lookup(PRIMARY_BINARIES, name).unwrap_or("Local binary declared by this Cargo manifest; presence is not installed acceptance.")));
        }
    }
    let tools = root.join("adl/tools");
    let skills = tools.join("skills");
    let mut tool_rows = Vec::new();
    for path in children(&tools)? {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("non UTF-8 tool name")?;
        if ["skills", "__pycache__"].contains(&name) {
            continue;
        }
        let (class, notes) = if path.is_dir() {
            if !has_file(&path)? {
                continue;
            }
            (
                "internal helper",
                "Directory containing bounded helper assets, scripts, or proof inputs.",
            )
        } else {
            tool(name)
        };
        tool_rows.push(row(
            &format!("adl/tools/{name}"),
            "tool surface",
            class,
            notes,
        ));
    }
    let mut skill_rows = Vec::new();
    if skills.is_dir() {
        for path in children(&skills)? {
            if !path.is_dir() || !path.join("SKILL.md").exists() {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .context("non UTF-8 skill name")?;
            let (class, notes) = skill(name);
            skill_rows.push(row(
                &format!("adl/tools/skills/{name}"),
                "skill",
                class,
                notes,
            ));
        }
    }
    let historical: Vec<_> = HISTORICAL_EVIDENCE
        .iter()
        .map(|(path, notes)| row(path, "historical evidence", "historical evidence", notes))
        .collect();
    let owners: Vec<_> = OWNER_NAVIGATION
        .iter()
        .map(|(p, k, c, n)| row(p, k, c, n))
        .collect();
    let mut counts = BTreeMap::<&str, usize>::new();
    for rows in [&bins, &tool_rows, &skill_rows, &historical, &owners] {
        for entry in rows {
            *counts.entry(&entry.2).or_default() += 1;
        }
    }
    let mut text = format!("{HEADER}## Classification Counts\n\n");
    for class in [
        "primary",
        "supported shim",
        "internal helper",
        "historical evidence",
        "archive candidate",
        "remove candidate",
    ] {
        text += &format!("- `{class}`: {}\n", counts.get(class).copied().unwrap_or(0));
    }
    for (heading, rows) in [
        ("Locally Declared Binaries", &bins),
        ("Owner Navigation", &owners),
        ("Tool Surfaces Under `adl/tools`", &tool_rows),
    ] {
        text += &format!("\n## {heading}\n\n{}", table(rows));
    }
    let skill_note = if skills.is_dir() {
        "Source skill directories only; installed skill admission is not inventoried here."
    } else {
        "Unavailable: the source skill directory is absent. Installed skills are not inventoried."
    };
    text += &format!(
        "\n## ADL Skills\n\n{skill_note}\n\n{}\n## Historical Evidence Artifacts\n\n{}\n{FOOTER}",
        table(&skill_rows),
        table(&historical)
    );
    Ok(text)
}
pub fn execute(root: &Path, check: bool) -> Result<()> {
    let rendered = build(root)?;
    if check {
        anyhow::ensure!(
            fs::read_to_string(root.join(OUTPUT))? == rendered,
            "FAIL: {OUTPUT} is stale; rerun generator"
        );
        println!("PASS: {OUTPUT} is current");
    } else {
        fs::write(root.join(OUTPUT), rendered)?;
        println!("WROTE {OUTPUT}");
    }
    Ok(())
}
