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
- PR #591 has consumed the current V3-H evidence, has been reconciled with
  current `main`, and has current non-closing typed publication evidence;
- the #505 validation DAG is current at the published head and must remain
  current if PR #591 changes again;
- rollback exercise evidence is now recorded in
  `.csdlc/evidence/505/pre-cutover-rollback-exercise.json`: v3 terminal,
  cleanup, and cutover routes stayed non-authoritative, pre-cutover cleanup
  removal was denied, and typed v2 validation still passed afterward;
- terminal v3 finish/cleanup canary evidence is now recorded for merged PR
  #641 and closed issue #629: v3 observed live GitHub terminal truth, previewed
  cleanup against a registered exact-head worktree, and refused removal before
  #505 cutover authority;
- #179/#180 and parity dispositions are evidence-backed in
  `docs/csdlc-v3/authority-transition-disposition.json`;
- explicit operator approval for #505 authority cutover remains absent and is
  the remaining pre-merge approval gate.

## Operator guidance

Until #505 closes, use typed C-SDLC v2 for live issue work. Treat v3 output as
evidence to review, not as authority to act. If a v3 canary discovers a defect,
record it against #632 and either fix it before cutover or mark it as an
explicit #505 blocker.
