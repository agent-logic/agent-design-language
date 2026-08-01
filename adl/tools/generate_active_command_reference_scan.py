#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from collections import Counter
from dataclasses import dataclass
from datetime import date
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
OUTPUT_PATH = (
    REPO_ROOT
    / "docs"
    / "milestones"
    / "v0.91.5"
    / "ACTIVE_COMMAND_REFERENCE_SCAN_3735.md"
)


ACTIVE_PATH_PREFIXES = (
    "AGENTS.md",
    "CONTRIBUTING.md",
    "adl/src/cli/",
    "docs/tooling/",
    "docs/templates/",
    "adl/tools/skills/",
    "adl/tools/",
    "docs/planning/",
    ".adl/v0.91.5/tasks/",
    ".adl/v0.91.5/bodies/",
)

HISTORICAL_PATH_PREFIXES = (
    "docs/milestones/",
    "docs/planning/PR_CONTROL_PLANE_DECRUFT_COMPATIBILITY_CUT_PLAN.md",
    "docs/milestones/v0.91.5/review/",
    "docs/milestones/v0.91.5/release/",
    "docs/milestones/v0.91.4/",
    "docs/milestones/v0.91.3/",
    "docs/milestones/v0.91.2/",
    "docs/milestones/v0.91.1/",
    "docs/milestones/v0.91.0/",
    "docs/milestones/v0.90",
)

UNKNOWN_PATH_PREFIXES = (
    ".adl/cards/",
    "docs/milestones/v0.91.5/",
    "adl/tools/",
)

INCLUDE_ROOTS = (
    REPO_ROOT / "AGENTS.md",
    REPO_ROOT / "CONTRIBUTING.md",
    REPO_ROOT / "adl" / "src" / "cli",
    REPO_ROOT / "docs" / "tooling",
    REPO_ROOT / "docs" / "templates",
    REPO_ROOT / "adl" / "tools" / "skills",
    REPO_ROOT / "docs" / "planning",
    REPO_ROOT / "docs" / "milestones" / "v0.91.5",
    REPO_ROOT / ".adl" / "v0.91.5" / "tasks",
    REPO_ROOT / ".adl" / "v0.91.5" / "bodies",
    REPO_ROOT / ".adl" / "cards",
    REPO_ROOT / "adl" / "tools",
)

SKIP_PATH_SUBSTRINGS = (
    "/target/",
    "/.git/",
    "/node_modules/",
)

EXCLUDED_REL_PATHS = {
    "adl/tools/generate_active_command_reference_scan.py",
    "adl/tools/test_generate_active_command_reference_scan.sh",
    # These wrappers are executable fail-closed tombstones. Their only legacy
    # strings are prohibition/migration text, not runnable workflow routes.
    "adl/tools/codex_pr.sh",
    "adl/tools/codexw.sh",
    "docs/milestones/v0.91.5/ACTIVE_COMMAND_REFERENCE_SCAN_3735.md",
}

HISTORICAL_EXACT_PATHS = {
    # Immutable migration/review evidence and explicitly retired command docs.
    "adl/tools/build_v0916_workflow_metric_backfill_inventory.py",
    "docs/tooling/ADL_OCTOCRAB_MIGRATION_REVIEW.md",
    "docs/tooling/BUILD_ACTION_LOGS.md",
    "docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md",
    "docs/tooling/PR_INVENTORY_COMMAND.md",
    "docs/tooling/WP_ISSUE_WAVE_GENERATION.md",
    "docs/tooling/active-card-lifecycle-migration-readiness-v0.91.2.md",
    "docs/tooling/csdlc-prompt-editor/editor_model.js",
    "docs/tooling/prompt-spec.md",
    "docs/tooling/review-surface-format.md",
    "docs/tooling/reviewer-provenance.md",
    "docs/tooling/reviewer-surface.md",
    "docs/tooling/structured-prompt-contracts.md",
}


@dataclass(frozen=True)
class CommandFamily:
    key: str
    label: str
    preferred_owner: str
    required_action: str
    pattern: str


COMMAND_FAMILIES = (
    CommandFamily(
        key="sunset_pr_sh",
        label="sunset `adl/tools/pr.sh` wrapper",
        preferred_owner="csdlc-install resolve + typed csdlc-* binaries",
        required_action="forbid if active or unknown; preserve only if historical",
        pattern=r"(?<![\w/-])(?:bash\s+)?(?:\./)?adl/tools/pr\.sh\s+(?:create|init|start|run|doctor|ready|preflight|finish|closeout|issue|shepherd|janitor|pr-inventory|project-doctor)\b|(?<![\w/-])bash\s+\"?\$ROOT_DIR/adl/tools/pr\.sh\"?\s+(?:create|init|start|run|doctor|ready|preflight|finish|closeout|issue|shepherd|janitor|pr-inventory|project-doctor)\b",
    ),
    CommandFamily(
        key="direct_adl_pr",
        label="direct `adl pr ...` issue-mode commands",
        preferred_owner="csdlc-install resolve + typed csdlc-* binaries",
        required_action="forbid if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl pr (?:create|init|doctor|ready|preflight|run|finish|closeout)\b",
    ),
    CommandFamily(
        key="runtime_through_pr",
        label="`adl pr run <adl.yaml>` runtime-through-PR",
        preferred_owner="adl-runtime run <adl.yaml> ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl pr run\s+[^`\s]+\.adl\.ya?ml\b|(?<![\w/-])adl pr run\s+[^`\s]+\.ya?ml\b",
    ),
    CommandFamily(
        key="legacy_prompt_template",
        label="`adl tooling prompt-template ...`",
        preferred_owner="adl-csdlc tooling prompt-template ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl tooling prompt-template\b",
    ),
    CommandFamily(
        key="legacy_review_tooling",
        label="legacy `adl tooling ...` helper/review commands",
        preferred_owner="adl-review ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl tooling (?:code-review|review-card-surface|review-runtime-surface|verify-review-output-provenance|verify-repo-review-contract|validate-structured-prompt|lint-prompt-spec|card-prompt|public-prompt-packet|markdown-ast-edit|github-release|csdlc-prompt-editor|generate-wp-issue-wave)\b",
    ),
    CommandFamily(
        key="legacy_runtime_umbrella",
        label="legacy umbrella runtime forms",
        preferred_owner="adl-runtime ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl (?:demo|provider|agent|instrument|learn|artifact|runtime-v2|godel|identity|keygen|sign|verify)\b",
    ),
    CommandFamily(
        key="legacy_codex_pr",
        label="retired `codex_pr.sh` / `codexw.sh` wrappers",
        preferred_owner="adl/tools/pr.sh ...",
        required_action="migrate if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])(?:adl/tools/)?codex_pr\.sh\s+\S+|(?<![\w/-])(?:adl/tools/)?codexw\.sh\s+\S+",
    ),
    CommandFamily(
        key="unapproved_helper_binaries",
        label="unapproved helper binaries",
        preferred_owner="adl-runtime ...",
        required_action="forbid if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl-(?:crypto|godel|identity)\b|(?<![\w/-])adl (?:godel|identity)\b",
    ),
    CommandFamily(
        key="csdlc_issue_run",
        label="`adl-csdlc issue run <issue>`",
        preferred_owner="csdlc-bind --root <worktree> --request <bind-request.json>",
        required_action="forbid if active; preserve if historical; route if unknown",
        pattern=r"(?<![\w/-])adl-csdlc issue run\b",
    ),
)


def repo_rel(path: Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def should_skip(path: Path) -> bool:
    rel = repo_rel(path)
    if rel in EXCLUDED_REL_PATHS:
        return True
    if rel.startswith("adl/tools/test_"):
        return True
    return any(token in f"/{rel}" for token in SKIP_PATH_SUBSTRINGS)


def iter_paths() -> list[Path]:
    paths: list[Path] = []
    for root in INCLUDE_ROOTS:
        if not root.exists():
            continue
        if root.is_file():
            paths.append(root)
            continue
        for path in sorted(root.rglob("*")):
            if not path.is_file():
                continue
            if should_skip(path):
                continue
            paths.append(path)
    return paths


def classify_path(rel: str) -> str:
    if rel in HISTORICAL_EXACT_PATHS:
        return "historical"
    if rel.startswith(HISTORICAL_PATH_PREFIXES):
        return "historical"
    if rel.startswith(ACTIVE_PATH_PREFIXES):
        return "active"
    if rel.startswith(UNKNOWN_PATH_PREFIXES):
        return "unknown"
    return "unknown"


def load_text(path: Path) -> str | None:
    try:
        return path.read_text()
    except UnicodeDecodeError:
        return None


def evidence_pointer(text: str, index: int) -> tuple[int, str]:
    line = text.count("\n", 0, index) + 1
    excerpt = text[index : index + 120].splitlines()[0].strip()
    return line, excerpt


def build_rows() -> tuple[list[tuple[str, str, int, str, str, str, str]], Counter]:
    rows: list[tuple[str, str, int, str, str, str, str]] = []
    counts: Counter = Counter()
    seen: set[tuple[str, str, int]] = set()
    for path in iter_paths():
        text = load_text(path)
        if text is None:
            continue
        rel = repo_rel(path)
        path_class = classify_path(rel)
        for family in COMMAND_FAMILIES:
            for match in re.finditer(family.pattern, text):
                line, excerpt = evidence_pointer(text, match.start())
                dedupe_key = (family.key, rel, line)
                if dedupe_key in seen:
                    continue
                seen.add(dedupe_key)
                counts[(family.key, path_class)] += 1
                rows.append(
                    (
                        family.label,
                        rel,
                        line,
                        path_class,
                        family.required_action,
                        family.preferred_owner,
                        excerpt,
                    )
                )
    rows.sort(key=lambda row: (row[3], row[0], row[1], row[2]))
    return rows, counts


def summarize_family_counts(counts: Counter) -> list[tuple[str, int, int, int]]:
    summary: list[tuple[str, int, int, int]] = []
    for family in COMMAND_FAMILIES:
        summary.append(
            (
                family.label,
                counts[(family.key, "active")],
                counts[(family.key, "historical")],
                counts[(family.key, "unknown")],
            )
        )
    return summary


def summarize_unique_class_counts(
    rows: list[tuple[str, str, int, str, str, str, str]]
) -> Counter:
    counts: Counter = Counter()
    seen: set[tuple[str, int, str]] = set()
    for _, rel, line, class_name, _, _, _ in rows:
        key = (rel, line, class_name)
        if key in seen:
            continue
        seen.add(key)
        counts[class_name] += 1
    return counts


def render(rows: list[tuple[str, str, int, str, str, str, str]], counts: Counter) -> str:
    by_class = summarize_unique_class_counts(rows)
    summary_rows = summarize_family_counts(counts)
    lines = [
        "# Active Command Reference Scan 3735",
        "",
        "Issue: #3735",
        "Parent sprint: #3732",
        f"Captured: {date.today().isoformat()}",
        "Status: generated_scan_report",
        "",
        "## Summary",
        "",
        "This report implements the `#3628` active-bundle scan gate for the",
        "toolkit-simplification sprint. It scans the current repo surfaces for",
        "legacy command-family references and classifies matching command-family",
        "hits as `active`, `historical`, or `unknown` using the path classes",
        "defined for shim-cut review.",
        "",
        "Deletion or fail-closed shim cuts remain blocked while any relevant",
        "`active` or `unknown` references remain unrouted.",
        "",
        "## Generation Command",
        "",
        "```bash",
        "python3 adl/tools/generate_active_command_reference_scan.py",
        "python3 adl/tools/generate_active_command_reference_scan.py --check",
        "```",
        "",
        "## Scan Inputs",
        "",
        "- `AGENTS.md`",
        "- `docs/templates/`",
        "- `adl/tools/skills/`",
        "- `docs/planning/`",
        "- `docs/milestones/v0.91.5/`",
        "- `.adl/v0.91.5/tasks/`",
        "- `.adl/v0.91.5/bodies/`",
        "- `.adl/cards/`",
        "- `adl/tools/`",
        "",
        "## Unique Evidence Totals",
        "",
        f"- `active`: {by_class['active']}",
        f"- `historical`: {by_class['historical']}",
        f"- `unknown`: {by_class['unknown']}",
        "",
        "## Command Family Hit Totals",
        "",
        "| Command family | Active | Historical | Unknown |",
        "| --- | --- | --- | --- |",
    ]
    for label, active, historical, unknown in summary_rows:
        lines.append(f"| {label} | {active} | {historical} | {unknown} |")

    lines.extend(
        [
            "",
            "## Findings",
            "",
            "| Command reference | Path | Line | Class | Required action before deletion | Preferred owner | Evidence excerpt |",
            "| --- | --- | --- | --- | --- | --- | --- |",
        ]
    )
    for label, rel, line, class_name, action, owner, excerpt in rows:
        safe_excerpt = excerpt.replace("|", "\\|").replace("`", "\\`")
        lines.append(
            f"| {label} | `{rel}` | {line} | `{class_name}` | {action} | `{owner}` | `{safe_excerpt}` |"
        )

    lines.extend(
        [
            "",
            "## Deletion Recommendation",
            "",
            "- Do not delete or fail-close any scanned command family while `active` findings remain.",
            "- Route every `unknown` finding through a bounded follow-on before compatibility removal.",
            "- Treat `historical` findings as readability evidence, not as executable dependencies.",
            "",
            "## Known Classification Rules",
            "",
            "- `AGENTS.md`, prompt templates, skills, planning docs, open issue bodies, and task bundles classify as `active`.",
            "- `adl/tools/` scripts and active validation/demo helpers classify as `active` unless a later issue carves out an explicit historical-fixture rule.",
            "- `docs/milestones/v0.91.5/review/` and older closed-milestone evidence classify as `historical`.",
            "- `docs/milestones/v0.91.5/` outside review packets and `.adl/cards/` classify as `unknown` until a later issue narrows them further.",
            "- Unique evidence totals count one row per path/line/class; command-family hit totals may overlap when one source line names multiple legacy families.",
            "",
            "## Non-Claims",
            "",
            "- This issue does not delete any compatibility shim.",
            "- This issue does not rewrite historical records solely to remove old command strings.",
            "- This issue does not claim every `unknown` reference is unsafe; it only routes them for future review.",
        ]
    )
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    rows, counts = build_rows()
    rendered = render(rows, counts)

    if args.check:
        non_csdlc_families = {
            "legacy umbrella runtime forms",
            "unapproved helper binaries",
        }
        blocking = [
            row
            for row in rows
            if row[3] in {"active", "unknown"} and row[0] not in non_csdlc_families
        ]
        if blocking:
            details = "\n".join(
                f"{row[1]}:{row[2]}: {row[0]}" for row in blocking[:50]
            )
            raise SystemExit(
                f"active command reference scan found {len(blocking)} blocking legacy references:\n{details}"
            )
        return 0

    OUTPUT_PATH.write_text(rendered)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
