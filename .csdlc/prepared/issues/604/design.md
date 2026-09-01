# Issue #604 design: governed draft-to-ready publication reconciliation

## Goal

Restore the authoritative C-SDLC v2 publication owner so an already governed
draft PR can be marked ready through typed identity-checked tooling instead of
raw GitHub mutation.

## Current failure

The installed v2 publication binary exposes `publish`, `status`, and `schema`.
The live issue and skill text require bounded `ready` and `reconcile-ready`
publication operations. That gap strands otherwise governed draft PRs because
raw `gh` ready mutation is prohibited except under break-glass.

## Design

Add a typed publication-ready request and result under `csdlc-publish`.

The ready operation must:

- load the current issue record and require reviewed or published issue truth;
- require exact repository, issue, PR number, branch, head SHA, generation, and
  digest inputs;
- observe the live PR before mutation and reject repository, PR, head, state, or
  draft mismatches;
- use the existing typed GitHub transport owner to mark the PR ready;
- re-observe the PR after mutation and record ready publication truth only after
  the remote readback proves the expected state;
- retain an idempotent operation key so an uncertain response can be recovered
  through `reconcile-ready`;
- fail before lifecycle mutation on stale records, non-draft PRs, closed PRs,
  wrong head SHA, or transport/readback mismatch.

The reconcile operation must:

- consume the same exact identity request;
- re-observe the target PR;
- complete the ready publication truth only if the remote already reflects the
  expected non-draft open PR at the expected head;
- reject contradictory state without mutating lifecycle truth.

## Non-goals

- No merge, issue closeout, cleanup, or terminal delivery.
- No Runtime v3 behavior change.
- No raw GitHub fallback.
- No weakening of reviewed/publication prerequisites.

## Validation

Focused validation covers request parsing, stale CAS rejection, mismatched PR
identity rejection, pre-state rejection, uncertain-response reconciliation, and
zero-write failure paths. The issue also runs the C-SDLC v2 GitHub route tests
and exact diff hygiene before publication.
