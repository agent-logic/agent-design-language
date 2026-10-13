#!/usr/bin/env python3
from __future__ import annotations

import re
from collections import Counter
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
TOOLS_DIR = REPO_ROOT / "adl" / "tools"
SKILLS_DIR = TOOLS_DIR / "skills"
CARGO_TOML = REPO_ROOT / "adl" / "Cargo.toml"
OUTPUT_PATH = REPO_ROOT / "docs" / "tooling" / "TOOL_SURFACE_REGISTRY.md"


PRIMARY_BINARIES = {
    "adl": "Public ADL planning and installed-product CLI declared by this repository.",
    "adl-process": "Public permission-safe process-status helper declared by this repository.",
}


SUPPORTED_SHIMS = {
}

REMOVE_CANDIDATES = {
    "codex_pr.sh": "Retired fail-closed wrapper kept only for migration guidance until final deletion.",
    "codexw.sh": "Retired fail-closed wrapper kept only for migration guidance until final deletion.",
}

PRIMARY_TOOL_SCRIPTS = {
    "batched_checks.sh": "Local validation entrypoint; use the current declared public validation contract.",
    "run_pr_fast_test_lane.sh": "Primary focused PR-fast local test runner.",
    "check_coverage_impact.sh": "Primary local coverage-impact gate for risky changed files.",
}

HISTORICAL_TOOL_SCRIPTS = {
    'run_v0917_csm_continuity_capsule_4910_proof.sh': 'Infrastructure-owned manual proof; retained ADL copy is historical source evidence. Use agent-logic-infrastructure owner delivery; resolver proof does not qualify full behavior.',
    'run_v0917_csm_otlp_4904_proof.sh': 'Infrastructure-owned manual proof; retained ADL copy is historical source evidence. Use agent-logic-infrastructure owner delivery; resolver proof does not qualify full behavior.',
    'run_v0917_no_sparrow_4909_proof.sh': 'Infrastructure-owned manual proof; retained ADL copy is historical source evidence. Use agent-logic-infrastructure owner delivery; resolver proof does not qualify full behavior.',

    "run_wp5795_aws_gpu_proof.sh": "Historical single-node SSM GPU proof with pinned artifacts and owner-bound cleanup; no new execution qualified, and the two-node issue345 proof is not an equivalent replacement.",
    "run_v0914_multi_agent_workcell_proof.sh": "Historical model-backed v0.91.4 workcell rehearsal; retained manifest and review artifacts do not qualify a new execution, including its provider-active dry-run path.",
    "demo_d11_signed_remote.sh": "Historical signed-remote contract; current public ADL does not supply its Runtime binary or test interfaces.",
    "demo_v087_operational_skills.sh": "Historical bundle-install and inventory contract; original recursive acceptance remains separate from owner resource proof.",
    "test_demo_v0871_real_multi_agent_discussion.sh": "Historical credentialed provider acceptance contract; five real calls remain unqualified.",
    "test_v0916_unity_observatory_unity65_smoke.sh": "Historical Unity editor smoke contract; retained timeout is not a successful scene-validation result.",
    "validate_issue256_birthday_after_observatory.py": "Historical issue256 acceptance validator tied to frozen issue414/424 evidence; not a current workflow entrypoint.",
}

HISTORICAL_EVIDENCE = [
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
]

OWNER_NAVIGATION = [
    (
        "https://github.com/agent-logic/agent-logic-infrastructure",
        "owner documentation",
        "primary",
        "Infrastructure manual proof owner; retained source and resolver checks do not qualify live behavior.",
    ),
    (
        "https://github.com/agent-logic/cognitive-sdlc",
        "owner documentation",
        "primary",
        "C-SDLC v3 lifecycle owner. Use its installed operator manual and authenticated authority; source presence grants no authority.",
    ),
    (
        "https://github.com/agent-logic/agent-logic-runtime",
        "owner documentation",
        "primary",
        "Runtime product owner; installation and product qualification are separate from this source inventory.",
    ),
    (
        "https://github.com/agent-logic/codefriend",
        "owner documentation",
        "primary",
        "CodeFriend review product owner; this registry does not invoke or qualify it.",
    ),
]


def repo_rel(path: Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def parse_bin_names() -> list[str]:
    # Read explicit [[bin]] tables, excluding the package/library names.
    # This inventory reports explicit declarations, not installed product admission.
    names = []
    in_bin = False
    for raw in CARGO_TOML.read_text().splitlines():
        line = raw.split("#", 1)[0].strip()
        if line.startswith("["):
            in_bin = line == "[[bin]]"
        elif in_bin:
            match = re.fullmatch(r'name\s*=\s*"([^\"]+)"', line)
            if match:
                names.append(match.group(1))
    return names


def classify_tool_script(path: Path) -> tuple[str, str]:
    name = path.name
    if name in HISTORICAL_TOOL_SCRIPTS:
        return "historical evidence", HISTORICAL_TOOL_SCRIPTS[name]
    if name in REMOVE_CANDIDATES:
        return "remove candidate", REMOVE_CANDIDATES[name]
    if name in SUPPORTED_SHIMS:
        return "supported shim", SUPPORTED_SHIMS[name]
    if name == "run_owner_validation_lane.sh":
        return "internal helper", "Mixed-owner legacy validation runner; successor test obligations remain separately tracked."
    if name in PRIMARY_TOOL_SCRIPTS:
        return "primary", PRIMARY_TOOL_SCRIPTS[name]
    if name.startswith("test_"):
        return "internal helper", "Focused regression or contract test helper."
    if name.startswith("demo_"):
        return "internal helper", "Demo/proof helper rather than a primary workflow entrypoint."
    if name.startswith("mock_"):
        return "internal helper", "Mock helper used for focused testing or proof."
    if name.startswith("check_") or name.startswith("validate_") or name.startswith("verify_") or name.startswith("run_"):
        return "internal helper", "Bounded validation/generation helper used by primary workflow entrypoints."
    if name.endswith(".py"):
        return "internal helper", "Python helper or generator used by a bounded tooling surface."
    if name.endswith(".sh"):
        return "internal helper", "Shell helper retained for bounded workflow support."
    return "internal helper", "Visible tool surface retained for bounded support work."


def classify_skill_dir(path: Path) -> tuple[str, str]:
    name = path.name
    if name in {
        "csdlc-doctor",
        "pr-init",
        "pr-ready",
        "pr-run",
        "pr-finish",
        "pr-closeout",
        "pr-janitor",
        "sprint-conductor",
    }:
        return "primary", "Primary skill in the governed issue/sprint lifecycle."
    if name.endswith("-editor") or name in {"sip-editor", "stp-editor", "spp-editor", "srp-editor", "sor-editor"}:
        return "supported shim", "Lifecycle editor skill used to normalize durable card truth."
    return "internal helper", "Specialized skill available for bounded tasks but not part of the primary issue workflow spine."


def render_table(rows: list[tuple[str, str, str, str]]) -> list[str]:
    lines = ["| Surface | Kind | Classification | Notes |", "| --- | --- | --- | --- |"]
    for surface, kind, classification, notes in rows:
        lines.append(f"| `{surface}` | {kind} | {classification} | {notes} |")
    return lines


def build_document() -> str:
    bin_rows: list[tuple[str, str, str, str]] = []
    counts = Counter()

    for name in parse_bin_names():
        notes = PRIMARY_BINARIES.get(name, "Local binary declared by this Cargo manifest; presence is not installed acceptance.")
        classification = "primary"
        counts[classification] += 1
        bin_rows.append((name, "owner binary", classification, notes))

    tool_rows: list[tuple[str, str, str, str]] = []
    for path in sorted(TOOLS_DIR.iterdir()):
        if path.name in {"skills", "__pycache__"}:
            continue
        if path.is_dir():
            classification = "internal helper"
            notes = "Directory containing bounded helper assets, scripts, or proof inputs."
        else:
            classification, notes = classify_tool_script(path)
        counts[classification] += 1
        tool_rows.append((repo_rel(path), "tool surface", classification, notes))

    skill_rows: list[tuple[str, str, str, str]] = []
    for path in sorted(
        p for p in (SKILLS_DIR.iterdir() if SKILLS_DIR.is_dir() else [])
        if p.is_dir() and (p / "SKILL.md").exists()
    ):
        classification, notes = classify_skill_dir(path)
        counts[classification] += 1
        skill_rows.append((repo_rel(path), "skill", classification, notes))

    historical_rows: list[tuple[str, str, str, str]] = []
    for rel, notes in HISTORICAL_EVIDENCE:
        counts["historical evidence"] += 1
        historical_rows.append((rel, "historical evidence", "historical evidence", notes))

    command_rows: list[tuple[str, str, str, str]] = []
    for row in OWNER_NAVIGATION:
        counts[row[2]] += 1
        command_rows.append(row)

    lines = [
        "# ADL Source Tool Surface Registry",
        "",
        "Historical baseline: issue #3734; its v0.91.5 snapshot remains unchanged.",
        "Status: generated_live_registry",
        "",
        "## Summary",
        "",
        "This registry records the current visible ADL toolkit surface for the",
        "selected source checkout. It is a live source inventory intended to",
        "drive later simplification cuts without confusing primary workflow",
        "entrypoints, supported compatibility shims, internal helpers, and",
        "historical evidence artifacts.",
        "",
        "This registry distinguishes active workflow spines from already-retired",
        "wrapper residue so later cuts can stay evidence-bound.",
        "",
        "## Generation Command",
        "",
        "```bash",
        "python3 adl/tools/generate_tool_surface_registry.py",
        "```",
        "",
        "## Freshness Check",
        "",
        "```bash",
        "python3 adl/tools/generate_tool_surface_registry.py --check",
        "```",
        "",
        "## Classification Counts",
        "",
        f"- `primary`: {counts['primary']}",
        f"- `supported shim`: {counts['supported shim']}",
        f"- `internal helper`: {counts['internal helper']}",
        f"- `historical evidence`: {counts['historical evidence']}",
        f"- `archive candidate`: {counts['archive candidate']}",
        f"- `remove candidate`: {counts['remove candidate']}",
        "",
        "## Locally Declared Binaries",
        "",
        *render_table(bin_rows),
        "",
        "## Owner Navigation",
        "",
        *render_table(command_rows),
        "",
        "## Tool Surfaces Under `adl/tools`",
        "",
        *render_table(tool_rows),
        "",
        "## ADL Skills",
        "",
        ("Source skill directories only; installed skill admission is not inventoried here."
         if SKILLS_DIR.is_dir() else
         "Unavailable: the source skill directory is absent. Installed skills are not inventoried."),
        "",
        *render_table(skill_rows),
        "",
        "## Historical Evidence Artifacts",
        "",
        *render_table(historical_rows),
        "",
        "## Non-Claims",
        "",
        "- This registry does not approve final deletion of retired wrappers.",
        "- This registry does not prove active-reference absence for any shim.",
        "- This registry does not replace the later simplification review and scan issues.",
    ]
    return "\n".join(lines) + "\n"


def main() -> int:
    import sys

    check_mode = len(sys.argv) > 1 and sys.argv[1] == "--check"
    rendered = build_document()

    if check_mode:
        current = OUTPUT_PATH.read_text()
        if current != rendered:
            print(f"FAIL: {repo_rel(OUTPUT_PATH)} is stale; rerun generator")
            return 1
        print(f"PASS: {repo_rel(OUTPUT_PATH)} is current")
        return 0

    OUTPUT_PATH.write_text(rendered)
    print(f"WROTE {repo_rel(OUTPUT_PATH)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
