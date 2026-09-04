# C-SDLC v3 cutover readiness notice

Status: advance notice only. C-SDLC v3 is not the live authority yet.

## What is changing

The intended post-cutover operator surface is one Rust binary named `csdlc`.
It is meant to replace the current C-SDLC v2 binary family with a simpler,
typed command surface that behaves like a small governed GitHub/lifecycle CLI:
issue setup, local lifecycle state, review, publication, PR and issue
readback, finish, cleanup, proof, shadow, soak, and install routes live under
one binary.

## What remains true before #505

- Root `AGENTS.md`, `csdlc-v2/AGENTS.md`, and typed v2 owner binaries remain
  authoritative.
- v3 may be used only as construction, fixture, and canary evidence.
- v3 must not bind live worktrees, mutate GitHub, publish PRs, finish issues,
  clean worktrees, or retire v2 before #505.
- Raw `gh` lifecycle writes remain prohibited.
- Every PR intended to close an issue still needs a visible and live-verified
  `Closes #<issue>` relationship.

## Current readiness state

The v3 replacement is not cutover-ready. The current #632 evidence records:

- the current `csdlc-v3` CLI advertises the planned one-binary surface for all
  21 v2 entrypoints, but those routes remain read-only or fail-closed
  construction evidence before #505;
- green construction/readiness PRs for most current V3-H lanes, with #629/#641
  refreshed after title-readback hardening and still requiring terminal CI
  confirmation at its latest head;
- a fail-closed #631 stacked PR closing-linkage defect;
- a fresh-worktree install/startup defect that affects the three-minute issue
  startup target;
- a #644-derived PR title-readback defect that #629 now fixes by requiring
  non-empty observed PR titles in authenticated readback receipts;
- terminal finish and cleanup canary proof still pending an authorized canary
  merge.

## Operator guidance

Until #505 closes, use typed C-SDLC v2 for live issue work. Treat v3 output as
evidence to review, not as authority to act. If a v3 canary discovers a defect,
record it against #632 and either fix it before cutover or mark it as an
explicit #505 blocker.
