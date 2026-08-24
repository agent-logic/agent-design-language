# v0.92 Canonical Document Inventory

Status: external-review input. This inventory defines the documentation corpus;
it does not claim release readiness or external-review approval.

## Standard denominator

The machine-readable authority is
[`docs/reviews/v0.92/docs-release-truth-312/inventory.json`](../../reviews/v0.92/docs-release-truth-312/inventory.json).
It contains exactly one row for:

1. root `README.md`, `CHANGELOG.md`, `AGENTS.md`, `REVIEW.md`,
   `docs/README.md`, `docs/planning/ADL_FEATURE_LIST.md`, and
   `csdlc-v2/AGENTS.md`;
2. every tracked regular file under this `docs/milestones/v0.92/` tree; and
3. every tracked operator `SKILL.md` under `csdlc-v2/operator/skills/`.

The #312 validator regenerates the set from Git and fails on missing, duplicate,
extra, stale-digest, machine-local, unparseable, out-of-scope, or tracked
`.adl`-dependent entries.

## Canonical entrypoints

- [Milestone README](README.md)
- [Vision](VISION_v0.92.md)
- [Design](DESIGN_v0.92.md)
- [Decisions](DECISIONS_v0.92.md)
- [WBS](WBS_v0.92.md)
- [Sprint plan](SPRINT_v0.92.md)
- [Issue wave](WP_ISSUE_WAVE_v0.92.yaml)
- [Execution readiness](WP_EXECUTION_READINESS_v0.92.md)
- [Feature index](features/README.md)
- [Feature/proof coverage](FEATURE_PROOF_COVERAGE_v0.92.md)
- [Quality gate](QUALITY_GATE_v0.92.md)
- [Demo matrix](DEMO_MATRIX_v0.92.md)
- [Milestone checklist](MILESTONE_CHECKLIST_v0.92.md)
- [ADR plan](ADR_PLAN_v0.92.md)
- [Release plan](RELEASE_PLAN_v0.92.md)
- [Release notes](RELEASE_NOTES_v0.92.md)
- [Next-milestone handoff](NEXT_MILESTONE_HANDOFF_v0.92.md)
- [External-review index](review/README.md)
- [Third-party review handoff](review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md)

## Current truth boundary

WP-22/#311 produced a structurally valid but blocked quality packet: 33 of 33
rows were blockers and downstream unlock was false. That result is an input to
review, not a passing release gate. #467 owns evidence-hydration repair in
parallel; its future merge may be incorporated by the mandatory publication-time
rescan, but its administrative closeout never gates this documentation pass.
