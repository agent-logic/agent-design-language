# Structured Planning Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Stabilize workspace ci-coverage with an exact seven-test Runtime v2 timeout override, repair the context-mirror fixture and explicit active-milestone authority exposed by hosted coverage, retain focused proof, obtain exact-head review, republish once, and require green hosted coverage before merge.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and bind #560 to its dedicated FastWork worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Apply and prove the exact seven-test ci-coverage timeout override without changing Runtime v2 semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Repair context-mirror canonical input resolution and strict active-milestone detection; retain exact binary and negative proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, republish the reviewed head, and shepherd required hosted checks to green before merge.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "in_progress"
  }
]

## Invariants

- All seven unified-kernel tests keep their existing semantic assertions.
- Coverage instrumentation gets an exact bounded timeout allowance only for the named test module.
- Context-mirror milestone truth requires exactly one explicit active-status marker.
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
