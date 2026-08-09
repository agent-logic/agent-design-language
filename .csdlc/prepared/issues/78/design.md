# Issue 78 Design: Typed STP Deliverable Correction After Review Recovery

## Problem

`csdlc-review recover` clears stale review and publication authority and returns
an issue to `implemented`. The current `csdlc-edit` authorization table then
permits only STP acceptance-criteria correction. A contradictory STP
deliverable list therefore cannot be repaired through any supported typed
route, even when an exact review finding requires that correction.

## Required Behavior

Add `correct_stp_deliverables_after_recovery` as a narrow semantic operation.
It accepts only a replacement list and is authorized only when all of the
following are true:

1. the requested card is `stp`;
2. the issue phase is `implemented`;
3. the durable audit proves a preceding typed `recover_review` operation;
4. review assignment, review result, publication, readiness, and terminal
   authority are absent;
5. every replacement is nonblank and no normalized replacement is duplicated;
6. generation, digest, and card projections match current canonical truth.

The operation changes only `StpValues.deliverables`. It retains both the prior
and replacement collections in the audit operation description and uses the
existing atomic store commit to regenerate values, Markdown, AST digests, and
the issue digest together.

## Authorization Model

The recovery proof must be structural rather than inferred from empty review
fields. An ordinary implemented issue with no review history must be rejected.
The latest relevant lifecycle repair evidence must include `recover_review`,
and no later operation may re-establish review, publication, readiness, or
terminal authority.

This is not a general administrative edit mode and does not permit phase
rollback.

## Validation

- Positive end-to-end correction after typed review recovery.
- Rejection before recovery and in every non-implemented phase.
- Rejection for a non-STP card, stale generation, stale digest, projection
  drift, blank replacements, and duplicate replacements.
- Proof that all unrelated STP fields remain identical.
- Proof that audit evidence contains actor, reason, prior deliverables, and
  replacement deliverables.
- Focused tests, formatting, and strict Clippy using build output under
  `/Volumes/FastWork`.

## V3 Carry-Forward

This repair is intentionally narrow. C-SDLC v3 must replace scattered
phase-specific mutation allowlists with a complete command capability matrix,
explicit recovery provenance, invariant checks, and generated transition and
negative-test coverage so valid recovery paths cannot terminate in an
unrepairable state.
