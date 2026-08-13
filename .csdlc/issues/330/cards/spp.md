# Structured Planning Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap, approve design, bind #330, repair only the production bridge-fed invariant, prove focused regressions and #300 integration, obtain fresh exact-head review, publish, watch CI, and finish when gates are green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap, approve, doctor, and bind #330 from the #300 RED evidence.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the narrow recovery-validation and cleanup-race production repair.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused #330 and parent #300 bridge-fed validation, strict Clippy, fmt/diff, validate, and doctor.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, publish, shepherd CI, and finish terminal if gates are green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Recovery validation stays fail-closed unless exact cleanup authority proves the post-cleanup state
- Cleanup final receipt races must be zero-mutation on rejection
- #300 remains frozen until #330 terminal and ancestral
- Primary main remains tracked clean except unrelated pre-existing staging

## Risks

- The narrow repair may require coordinating recovery validator and cleanup receipt semantics
- A too-broad bypass could mask corrupt retained recovery attempts
- Hosted CI may surface unrelated baseline failures

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/330/design.md

Digest: 3a35b38d67f8c24930eb971ceffb16bf72fa4265a143a810d952c98bafe6e9b9

## Diagram

.csdlc/prepared/issues/330/diagram.mmd

Digest: fe93ca7d039f229429e9e9a0794cbd9e49956bb7d00173c6aa2341bbfca7a63d

## Stop Conditions

- Owned path collision with another active issue owner
- Need to widen beyond projection_recovery/projection_cleanup and focused tests
- Focused #300 bridge-fed integration remains red after repair
- Fresh review reports actionable finding
- Required hosted checks fail after publication

## Handoff

Proceed only after doctor readiness.
