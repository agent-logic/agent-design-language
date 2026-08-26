# Structured Planning Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #560 in a new FastWork worktree, add an exact ci-coverage nextest override for the three timed-out runtime_v2 unified-kernel tests, run focused llvm-cov nextest proof, obtain exact-head API review, publish ready, shepherd green checks, and finish merge.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and bind #560 to a dedicated FastWork worktree from current origin/main.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply the smallest instrumentation-aware ci-coverage timeout/profile adjustment for the three exact tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused local coverage proof and retain evidence.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head OpenAI Responses API review, publish ready, shepherd required checks, and typed finish merge.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- The three affected tests keep their existing semantic assertions.
- Coverage instrumentation gets an explicit bounded timeout allowance.
- The default ci-coverage timeout remains fail-closed for unrelated tests.

## Risks

- A too-broad timeout increase could hide unrelated hangs.
- Local focused proof may be shorter than hosted workspace coverage; hosted `adl-coverage` remains final proof.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/560/design.md

Digest: e20d5a3b15b9ff7b83a325f051d43f5e970d696cc191e2579cdd49f89d5545b3

## Diagram

.csdlc/prepared/issues/560/diagram.mmd

Digest: ab1176ec352e1b0f78d942d6d82491b26acc0c23116b30721c40b265c959cf05

## Stop Conditions

- A required fix would alter Runtime v2 product semantics.
- The focused coverage proof still times out after bounded adjustment.
- Typed lifecycle or exact-head review cannot be completed.

## Handoff

Proceed only after doctor readiness.
