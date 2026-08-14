# Structured Planning Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Separate admission acknowledgement from bounded execution timing, prove the cleanup race repeatedly, run the required Runtime lane, and obtain exact-head review before publication.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Diagnose and correct the admission versus execution deadline boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Strengthen and repeatedly run the cleanup-race regression proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run the required Runtime lane and exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Authentication remains required and server frame order is preserved.
- Production admission and execution deadline behavior is unchanged.
- Each active turn produces at most one terminal result.
- Active turns are not evicted to admit new work.
- Explicit cancellation and timeout remain fail-closed.

## Risks

- Moving acknowledgement could overstate execution success.
- Resetting deadlines incorrectly could weaken bounded execution.
- A timing-only test change could conceal a production defect.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/244/design.md

Digest: 032b8441dc168cd6953d93b0bffc43e4089a2b70ea2bf92b6263bbd8e96a326d

## Diagram

.csdlc/prepared/issues/244/diagram.mmd

Digest: aa4841f6144e20ef15b0769c04e9af3008e48c1cff066c57d4f8aa14acd3b2e3

## Stop Conditions

- The repair requires any #237 or PR #242 change.
- The focused proof contradicts established cancellation or timeout semantics.
- The required Runtime lane exposes a distinct unrelated blocker.

## Handoff

Proceed only after doctor readiness.
