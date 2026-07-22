# Structured Planning Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5359 is live-merged and ancestral.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify typed preparation packet and #5359 live merge plus ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify accepted release evidence and blockers",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Reconcile tag, notes, issue, PR, card, milestone, and handoff truth",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Perform authorized release closeout or preserve blockers without preparation-scope mutations",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is the dependency gate
- receipts audit-only
- no preparation review churn
- no implementation in preparation

## Risks

- ceremony could hide implementation work
- release notes could overclaim
- GitHub, card, milestone, and handoff state may disagree

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5348/design.md

Digest: 42d446da6fa9a799916632221c98363aa110577a72b8d384fde26d815fbdcf4f

## Diagram

.csdlc/prepared/issues/5348/diagram.mmd

Digest: 6bf90d440819d3f76adb93d445146829144efd9e7f870d1b1d84e82c14b98990

## Stop Conditions

- #5359 not live-merged
- #5359 merge not ancestral
- release evidence incomplete
- ceremony would require repair work

## Handoff

Proceed only after doctor readiness.
