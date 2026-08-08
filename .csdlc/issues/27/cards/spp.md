# Structured Planning Prompt

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Strengthen the validator's explicit role-set and post-proof path policies, add focused negative and positive regression checks, then review the exact delta.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Implement explicit canonical role-set and verifier-only Git path policy helpers",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Prove positive and negative policy cases in a temporary Git repository",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate the final exact-head WP-03 native receipt packet without rerunning product soak from this issue",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Digest recomputation remains mandatory
- Platform coverage remains exact and unique
- No runtime or product path is verifier-only

## Risks

- An overbroad allowlist could accept stale product proof
- Set canonicalization could accidentally hide duplicate roles

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/27/design.md

Digest: 8eefdd3176645f328745d212bdc70acefde07362ebbee9dc67b6638c14e5579e

## Diagram

.csdlc/prepared/issues/27/diagram.mmd

Digest: 6936a258fc1326850d138cc746ee7fa167478e225a389fb30a1abc4e6599e630

## Stop Conditions

- The repair requires any Runtime v3 product change
- The WP-03 committed validator baseline changes during implementation

## Handoff

Proceed only after doctor readiness.
