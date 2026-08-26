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

Digest: f7d8f2374406bb3db50d20589b07bfbe66a108728eb34307bc80826b7ea2af8d

## Diagram

.csdlc/prepared/issues/560/diagram.mmd

Digest: 1ef366e0f765b353b3dcc5b600674f2b6495d0965818b45ba9bd14b66fd01bc2

## Stop Conditions

- A required fix would alter Runtime v2 product semantics.
- The focused coverage proof still times out after bounded adjustment.
- Typed lifecycle or exact-head review cannot be completed.

## Handoff

Proceed only after doctor readiness.
