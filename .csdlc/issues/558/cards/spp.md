# Structured Planning Prompt

Template: 1.0.0

Issue: 558

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #558 in a new FastWork worktree, inspect the learner replication wait/leader readiness flow, apply the smallest test-harness stabilization, run focused proof, record exact-head API review, publish ready, shepherd green checks, and finish merge.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and bind #558 to a dedicated FastWork worktree from current origin/main.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inspect the four-node learner replication harness and apply the smallest instrumentation-aware stabilization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused local proof and retain evidence.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head OpenAI Responses API review, publish ready, shepherd required checks, and typed finish merge.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- The test continues to exercise a real four-node learner replication path.
- A learner append is accepted only through existing governed authority.
- Timing adjustments are bounded and diagnostics remain test-local.

## Risks

- Coverage instrumentation can stretch async scheduling enough that an ordinary timeout remains insufficient.
- A too-broad fix could accidentally mask a real replication defect; review must verify semantics are unchanged.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/558/design.md

Digest: 5b4c7f2d6bdf6199844832ac14e83ed514183ea6e8bd799d6eaf81b330bdb2c7

## Diagram

.csdlc/prepared/issues/558/diagram.mmd

Digest: b51f18318d8bdc6cf6b92f791baf4fc69aeed4dcf5fcc006b07b8576d6cca1a3

## Stop Conditions

- A required fix would alter Runtime product semantics.
- The focused test still fails after bounded stabilization.
- Typed lifecycle or exact-head review cannot be completed.

## Handoff

Proceed only after doctor readiness.
