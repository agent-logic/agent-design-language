# Issue 524 design — successor closeout contract

## Result

One reviewed closeout contract shared by the v0.92.2 release plan and milestone
checklist. It defines the denominator and gates but performs no release action.

## Authority and ordering

The reviewed #523 merge supplies the successor denominator. TAIL-01 through
TAIL-10 remain explicit and ordered. Downstream execution gates on reviewed
merge truth; typed finish and worktree cleanup remain asynchronous bookkeeping.
Tagging, release publication, and other operator actions require explicit
authorization at execution time.

## Execution shape

1. Freeze the reviewed #523 planning revision.
2. Compare release plan and checklist for denominator and order parity.
3. Repair only those two closeout surfaces.
4. Validate, obtain independent exact-head review, and publish one PR.

## Guardrails

- No v0.92.2 execution or issue creation.
- No current milestone ceremony.
- No implicit operator authority.
- Stop on denominator disagreement or absent reviewed #523 merge.
