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

The v3 replacement command denominator is implemented, but authority cutover is
deferred. The current sprint evidence records:

- live GitHub readback shows V3-H umbrella #625 and all six child issues
  #627-#632 closed;
- the current `csdlc-v3` CLI exposes one `csdlc` command surface covering all
  21 required v2 entrypoints plus helper/construction routes;
- those routes remain construction evidence before #505 and do not make v3
  live lifecycle authority;
- PR #591 must still consume the current V3-H evidence, merge current `main`,
  rerun the #505 validation DAG, and refresh exact-head review/publication
  evidence;
- rollback exercise evidence, terminal v3 finish/cleanup canary evidence or an
  explicit operator waiver, and evidence-backed #179/#180 and parity
  dispositions remain required before approval.

## Operator guidance

Until #505 closes, use typed C-SDLC v2 for live issue work. Treat v3 output as
evidence to review, not as authority to act. If a v3 canary discovers a defect,
record it against #632 and either fix it before cutover or mark it as an
explicit #505 blocker.
