# Issue #307 Design — Quality and Release Tail Coordination

## Purpose

#307 coordinates the v0.92 quality and release-tail wave. It owns dependency
truth, child merge/readiness reconciliation, sprint-level review, and final
umbrella closeout. Each child retains its own implementation, review,
publication, merge, and terminal authority.

## Entry Gate

#343 must be terminal, canonical, and ancestral before #308 begins. #343 in
turn waits for terminal #256 and #341. Historical WP-19 evidence is a read-only
input to the #343/#308 boundary.

## Current Child Allocation

- #308 — WP-20 demo matrix, AEE proof, and proof coverage.
- #309 — WP-21 repository-wide deletion/reduction cleanup.
- #310 — WP-21A bounded Rust refactoring and maintainability.
- #311 — WP-22 quality gate.
- #312 — WP-23 documentation and release-truth pass.
- #313 — WP-25 internal review.
- #314 — WP-26 external review.
- #315 — WP-27 review-finding remediation.
- #316 — WP-28 next-milestone planning.
- #317 — WP-28A closeout plan.
- #318 — WP-29 next-milestone review.
- #319 — WP-30 release ceremony.

## Canonical Child Sequence

#309 remains active v0.92 WP-21 work. The exact execution graph is
`#308 -> #309 -> #310 -> #311 -> #312 -> #313 -> #314 -> #315 -> #316 ->
#317 -> #318 -> #319`. #310 consumes the post-deletion #309 head and inventory,
and every later child waits for the predecessor state required by its own issue
contract. Ordinary successors depend on reviewed/green/merged predecessor truth,
not individual issue closeout. Closeout receipts and registered-worktree cleanup
remain asynchronous bookkeeping and gate only final #307 closeout.

## Current Release-Tail Routing

- #268 is closed successfully and is no longer an AWS qualification
  blocker for Sprint 6.
- #314 / WP-26 owns external-review intake and retained review reports.
- #315 / WP-27 owns external/internal review finding dispositions and
  remediation. #471 is a WP-27 remediation subissue, not an independent
  release-tail lane.
- #316 / WP-28 owns v0.92.1 and v0.92.2 planning publication and may proceed
  from documented WP-27 finding state without waiting for individual closeout
  paperwork.
- #317 / WP-28A owns the final closeout plan after #316 is published.

## Owned Surfaces

- `.csdlc/issues/307/**`
- `.csdlc/prepared/issues/307/**`
- `.csdlc/evidence/307/**`
- `docs/milestones/v0.92/review/sprint_307/**`

All child implementation and release surfaces remain read-only.

## Coordination Contract

For every child in the operator-approved graph, require exact issue and PR
identity, reviewed head, green required checks, merge state, merge ancestry,
residual risk, and handoff state before advancing to a dependent child. Typed
terminal cache, canonical match, and registered worktree cleanup are required
for final #307 closeout, but they are not dependencies for unrelated successor
implementation once the predecessor merge/readiness contract is satisfied.

The umbrella cannot convert GitHub closure, passing local tests, a prepared
packet, or a green PR into terminal child authority.

## Exit Contract

1. The operator-approved child sequence is explicit and acyclic.
2. Every included child is merged or explicitly routed, and final #307 closeout
   reconciles terminal/canonical/ancestral/cleanup truth without blocking
   already-unblocked successor work.
3. Review findings are resolved or explicitly routed with truthful ownership.
4. Release evidence and claims agree with landed exact revisions.
5. WP-30 completes its separately authorized ceremony and v0.93 receives the
   accepted handoff without implicit activation.
6. One exact-head sprint review passes before #307 terminal closeout.

## Resolved #268 Carryover

#268 is outside the Sprint 6 child sequence and is now closed successfully. Final
release and sprint evidence must preserve that observed result. #268 does not
block #308 through #319 or #307 closeout.

## Non-Goals

- Implementing any child issue.
- Running AWS, provider, demo, review, tag, release, or deployment work during
  preparation.
- Treating #342 as terminal WP-24A unless its separate v0.92.1 authority is
  explicitly required by an included child.
- Activating v0.93.
- Reopening or reclassifying #268 qualification inside #307.

## Failure Policy

Fail closed on graph drift, an unmerged or unrouted child required by a
successor, nonancestral merge, red required check, stale review, unsupported
release claim, partial tag/release identity, missing final closeout
reconciliation, or cyclic handoff.
