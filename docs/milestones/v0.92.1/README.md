# ADL v0.92.1 — Integration and Product Readiness

v0.92.1 converts the remaining v0.92 planning residue into six independent, reviewable execution lanes. It is a focused integration milestone, not a container for every deferred idea.

## Opening gate

Issue #432 removes tracked dependencies on local untracked paths. It is the repository-authority prerequisite for #431 and every execution lane. WP-28 #316 and WP-28A #317 remain unchanged and outside this package.

## Execution lanes

1. **Corporate and IP transfer** — complete the bounded corporate, ownership, licensing, and operational handoff records.
2. **C-SDLC v3** — convert the reviewed GitHub-inspired architecture into a typed implementation program using the tracked source in `sources/`.
3. **Distributed multi-agent Runtime qualification** — qualify governed multi-agent work, using UTS as a workload rather than inventing a separate UTS architecture program.
4. **Podcast publication and Studio** — finish the #51 / #261–#264 / #342 product chain with release evidence and operator-owned external decisions.
5. **Axum configuration hot reload** — deliver validated last-known-good configuration replacement, beginning with stateless strings and flags.
6. **Observatory redesign** — redesign the product around authentic Runtime authority, accessibility, and explicit empty/degraded states; invented data is prohibited.

All six roots depend on #431 after #432. They may execute independently after those gates; no lane silently absorbs another.

The existing executable graph is #432, #431, podcast #51/#261-#264/#342, Observatory prerequisites #251/#122/#84, and GPU Shepherd #345. Every other execution and release-tail issue in this package is number-free planning truth; WP-01 creates those issues later in the exact catalog and dependency order. Premature placeholders #433-#438 are closed and must not be reused as execution authority. #431 owns the planning-time v0.92.2 handoff.

Closed issues #149–#190 were prematurely retired planning packets, not delivered execution. Their requirements remain in the routing denominator: corporate #153–#160 is consolidated into CORP-A through CORP-D; C-SDLC v3 #161–#180 into V3-A through V3-F; Runtime #181–#187 into DRT-A through DRT-C; and integration #188–#190 into INT-01 plus the canonical release tail. They are not reopened. The v0.92 tooling defect #387 is not part of the active milestone plan even if legacy labeling includes v0.92.1. #433–#438 were closed as premature placeholders and #439 was closed as redundant with #431.

Existing issues #251, #122, #84, and #345 are active v0.92.1 execution scope. #251 TLS 1.2, #122 Route53/ACM exposure, and #345 AWS GPU Shepherd hardening may execute in parallel; #84 Unity Observatory preparation may overlap them, while its final proof consumes #251 and #122. The distributed Runtime production qualification consumes #345 where GPU evidence is required.

## Canonical release tail

After all six execution roots are terminal, the planned integration conductor starts the same ten-step serial tail used by the preceding milestone standard: quality gate; docs and release-truth pass; publication finalization; internal review; external review; remediation/preflight; next-milestone planning; next-milestone closeout planning; next-milestone planning review; and release ceremony.

## Boundaries

- Runtime v4 is a named rebaseline risk, not implicit v0.92.1 scope. Any incompatible authority change must trigger explicit replanning.
- Observatory implementation depends on stable Runtime authority APIs; its design work can proceed earlier.
- Paid infrastructure, publication, legal decisions, and external credentials remain operator-controlled.
- CodeFriend implementation is not part of v0.92.1.

## Successor

v0.92.2 is the **CodeFriend Beta 1** milestone. The release train must make CodeFriend available as an integrated beta by v0.95.

## Package map

- [Vision](VISION_v0.92.1.md)
- [Design](DESIGN_v0.92.1.md)
- [Decisions](DECISIONS_v0.92.1.md)
- [Work breakdown](WBS_v0.92.1.md)
- [Sprint plan](SPRINT_v0.92.1.md)
- [Issue wave](WP_ISSUE_WAVE_v0.92.1.yaml)
- [Planned issue catalog](PLANNED_ISSUE_CATALOG_v0.92.1.md)
- [Canonical document inventory](CANONICAL_DOC_INVENTORY_v0.92.1.md)
- [Execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml)
- [Execution readiness](WP_EXECUTION_READINESS_v0.92.1.md)
- [Retirement ledger](WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml)
- [Feature plans](features/README.md)
- [Feature proof coverage](FEATURE_PROOF_COVERAGE_v0.92.1.md)
- [Quality gate](QUALITY_GATE_v0.92.1.md)
- [Demo matrix](DEMO_MATRIX_v0.92.1.md)
- [Milestone checklist](MILESTONE_CHECKLIST_v0.92.1.md)
- [ADR plan](ADR_PLAN_v0.92.1.md)
- [Release plan](RELEASE_PLAN_v0.92.1.md)
- [Release notes](RELEASE_NOTES_v0.92.1.md)
- [Next milestone handoff](NEXT_MILESTONE_HANDOFF_v0.92.1.md)
