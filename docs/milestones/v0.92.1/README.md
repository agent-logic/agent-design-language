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

The active roots are #433 Corporate and IP, #434 C-SDLC v3, #435 Distributed multi-agent Runtime, the existing #51 podcast graph, #436 Axum configuration hot reload, and #437 Observatory redesign. #438 owns integration and release. #431 itself owns the v0.92.2 handoff.

Closed issues #149–#190 are historical planning and evidence packets. They are inputs for reconciliation, not active v0.92.1 execution issues. The v0.92 tooling defect #387 is likewise not part of the active milestone plan even if legacy labeling includes v0.92.1. #439 was closed as redundant with #431.

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
- [Execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml)
- [Feature plans](features/README.md)
- [Next milestone handoff](NEXT_MILESTONE_HANDOFF_v0.92.1.md)
