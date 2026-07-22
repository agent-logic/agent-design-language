# Structured Planning Prompt

Template: 1.0.0

Issue: 5355

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5362 is live-merged and ancestral.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify typed preparation packet and #5362 live merge plus ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Consume accepted WP-21 feature-list and v0.92 truth",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prepare closeout-planning packet and canonical doc checks",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Preserve blockers or release WP-22 without preparation-scope mutations",
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

- historical preparation evidence could be mistaken for current truth
- canonical document inventory may be incomplete
- handoff language may overclaim v0.92 readiness

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5355/design.md

Digest: c708bc570694fbd8d88095ef40569313c219bfb5d15465b93ef4b43806359ebe

## Diagram

.csdlc/prepared/issues/5355/diagram.mmd

Digest: 4dc2f21e876feeea6bf768a3815fc9afc3d0ab59f8f2e965042df91b3aa23a88

## Stop Conditions

- #5362 not live-merged
- #5362 merge not ancestral
- canonical document missing
- handoff would overclaim

## Handoff

Proceed only after doctor readiness.
