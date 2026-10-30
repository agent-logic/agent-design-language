# v0.91.8 Milestone README

## Metadata

- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Status: WP-23 release ceremony active after all preceding release-tail gates
- Active release-tail issue: WP-23 release ceremony `#5348`
- Milestone sprint umbrella: `#5595`
- Historical planning sources: `#5335` and `#5383`
- Restored source issue: `#4641` remains `v0.91.7` WP-14
- v0.91.8 platform acceptance parent: `#5384` / WP-14A
- Downstream milestone: `v0.92`

## Purpose

`v0.91.8` is a bridge milestone between the `v0.91.7` implementation tranche
and the `v0.92` birthday milestone. It exists to finish the ADL Core
Rearchitecture platform prerequisite: ADL v2, Runtime v3, and C-SDLC v2 must be
accepted at exact revisions, installed on stable operational paths, exercised
through declared lifecycle contracts, and handed off to `v0.92` without
overstating birthday readiness.

The milestone is sourced from live issues and retained evidence. WP-16
established that every completed predecessor produced working code or a useful
durable result, except the explicitly partial/ambiguous rows retained in the
audit. That quality gate is not release approval by itself.

## Current Routing Truth

- `#4641` is restored to `[v0.91.7][WP-14] Launch and v0.92 birthday handoff`.
- `#5384` preserves the v0.91.8 integrated platform acceptance/deployment
  content that temporarily overwrote `#4641`.
- `#5383` is the historical v0.91.7 setup authority for this package and is
  closed; do not describe setup as currently in progress.
- `#5335` is retained as historical setup context and no longer carries active
  WP-01 ownership.
- `#5594` completed the opening readiness/canonical reconciliation gate.
- `#5595` is the single milestone sprint umbrella. Nested umbrellas `#5497`,
  `#5361`, and `#5384` own bounded multi-agent, Runtime v3, and integrated
  acceptance child sets without duplicating implementation ownership.
- WP-14A accepts only the platform revisions. Unity proof is owned by WP-15.
  Both WP-18 reviews `#5356` and `#5791`, WP-19 external review, WP-20
  remediation, WP-21 handoff, WP-21A closeout planning, and WP-22 planning
  review are closed. WP-23 owns the final release ceremony.

## Status

The milestone is ceremony-ready. WP-16 passed the integrated quality gate
at `2e9d2dd7c`, with 67 audited issue outcomes, 0 unacceptable outcomes, and
passing ADL v2, Runtime v3, and C-SDLC v2 validation lanes. WP-17 `#5360`
closed the documentation-alignment gate. WP-18 `#5356` and `#5791` closed both
internal review passes. WP-19 external review findings were remediated by
WP-20; WP-21, WP-21A, and WP-22 completed the v0.92 handoff and planning review.
Only the WP-23 tag, GitHub release, and umbrella closure remain.

Every work package must exit as one of:

- implemented and proven at an exact revision;
- already closed with current evidence;
- blocked with evidence and operator approval;
- explicitly deferred with downstream consumption truth.

## Document Map

- Vision: [VISION_v0.91.8.md](VISION_v0.91.8.md)
- Design: [DESIGN_v0.91.8.md](DESIGN_v0.91.8.md)
- Decisions: [DECISIONS_v0.91.8.md](DECISIONS_v0.91.8.md)
- Work breakdown: [WBS_v0.91.8.md](WBS_v0.91.8.md)
- Sprint plan: [SPRINT_PLAN_v0.91.8.md](SPRINT_PLAN_v0.91.8.md)
- Parallel execution plan: [PARALLEL_EXECUTION_PLAN_v0.91.8.md](PARALLEL_EXECUTION_PLAN_v0.91.8.md)
- WP-01 readiness report: [review/V0918_WP01_EXECUTION_READINESS_5594.md](review/V0918_WP01_EXECUTION_READINESS_5594.md)
- WP-16 quality evidence: [evidence/wp16/QUALITY_GATE.md](evidence/wp16/QUALITY_GATE.md)
- WP-16 issue outcome audit: [evidence/wp16/ISSUE_OUTCOME_AUDIT.md](evidence/wp16/ISSUE_OUTCOME_AUDIT.md)
- Issue wave: [WP_ISSUE_WAVE_v0.91.8.yaml](WP_ISSUE_WAVE_v0.91.8.yaml)
- Canonical document inventory: [CANONICAL_DOC_INVENTORY_v0.91.8.md](CANONICAL_DOC_INVENTORY_v0.91.8.md)
- Demo matrix: [DEMO_MATRIX_v0.91.8.md](DEMO_MATRIX_v0.91.8.md)
- Checklist: [MILESTONE_CHECKLIST_v0.91.8.md](MILESTONE_CHECKLIST_v0.91.8.md)
- Release plan: [RELEASE_PLAN_v0.91.8.md](RELEASE_PLAN_v0.91.8.md)
- Release notes: [RELEASE_NOTES_v0.91.8.md](RELEASE_NOTES_v0.91.8.md)
- WP-23 ceremony: [V0918_WP23_RELEASE_CEREMONY_5348.md](V0918_WP23_RELEASE_CEREMONY_5348.md)
- Quality gate: [QUALITY_GATE_v0.91.8.md](QUALITY_GATE_v0.91.8.md)
- Feature proof coverage: [FEATURE_PROOF_COVERAGE_v0.91.8.md](FEATURE_PROOF_COVERAGE_v0.91.8.md)
- Feature preservation crosswalk: [FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md](FEATURE_PRESERVATION_CROSSWALK_v0.91.8.md)
- Execution readiness: [WP_EXECUTION_READINESS_v0.91.8.md](WP_EXECUTION_READINESS_v0.91.8.md)
- ADR plan: [ADR_PLAN_v0.91.8.md](ADR_PLAN_v0.91.8.md)
- v0.92 handoff: [NEXT_MILESTONE_HANDOFF_v0.91.8.md](NEXT_MILESTONE_HANDOFF_v0.91.8.md)
- v0.92 closeout planning packet: [NEXT_MILESTONE_CLOSEOUT_PLAN_v0.91.8.md](NEXT_MILESTONE_CLOSEOUT_PLAN_v0.91.8.md)
- v0.92 activation test map: [V092_ACTIVATION_TEST_MAP_v0.91.8.md](V092_ACTIVATION_TEST_MAP_v0.91.8.md)
- Feature index: [features/README.md](features/README.md)
- Review index: [review/README.md](review/README.md)
- Third-party review handoff: [review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md](review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md)

## Non-Goals

- Do not implement v0.91.8 product changes in the setup issue.
- Do not delete incumbent ADL code from planning text.
- Do not claim `v0.92` birthday readiness before review and proof converge.
- Do not pull Runtime v3 or C-SDLC v2 ownership back into ADL core.

## Documentation Responsibility

WP-17 `#5360` closed documentation alignment. WP-18 `#5356` and `#5791`
closed both internal review passes. WP-19 findings are retained under
[review/external_review_5357/](review/external_review_5357/), and WP-20 closed
their accepted remediation before the handoff and ceremony stages.
Closed v0.91.7 WP-21A
`#5489` is historical preparation evidence. Current v0.91.8 WP-21A `#5355`
prepares
[NEXT_MILESTONE_CLOSEOUT_PLAN_v0.91.8.md](NEXT_MILESTONE_CLOSEOUT_PLAN_v0.91.8.md)
for WP-22 review. WP-21 `#5362` closed after PR #5807 merged exact head
`f1ddeacf5e91a1c8da690b2940e4125937aa57a3` as squash merge
`eaa62d3d2c0241bc07ce827fedef0e42389d0491`; WP-21A consumers must still verify
that merge commit is contained in current `origin/main`. Later v0.91.8 release-tail
documentation/review gates revalidate the canonical packet and fail closed if
any surface named in
[CANONICAL_DOC_INVENTORY_v0.91.8.md](CANONICAL_DOC_INVENTORY_v0.91.8.md) is
missing, contradictory, stale against live issue truth, or presents planned
work as proven.
