# Structured Planning Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #388, add the narrow implemented-phase card-truth repair operations and refusal tests, validate, obtain fresh exact-head review, publish, CI, and finish so #114 can consume the new typed route.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap, review, approve, doctor, and bind #388.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement bounded SPP/VPP/SOR card-truth repair operations and audit records, including SOR empty-vector removal and blank-entry refusal.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused tests, strict hygiene, fresh review, publish, CI, and finish.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Active review/publication/readiness/terminal truth invalidates card repair authority
- Audit remains append-only
- Repairs are exact-card and exact-field only
- Issue topology and product source are unchanged by card repair operations

## Risks

- Too broad a repair operation could become generic implemented-phase card rewriting
- Recovery epoch detection could accidentally span unrelated review cycles
- Existing #363 tests could regress if SPP summary guard is widened incorrectly

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/388/design.md

Digest: d2135ce8ea6c3c8b2a90a58f7e23f703e8752b5f74c11f4bc96ed792bf08a5b7

## Diagram

.csdlc/prepared/issues/388/diagram.mmd

Digest: 922883f7f2bcdf47badeeb23c839ce40770906175ff648bf956f94cf7c35e818

## Stop Conditions

- Need to authorize generic implemented-phase set_field
- Need to mutate #114 product/proof code
- Fresh review reports actionable finding
- Required validation or hosted CI fails

## Handoff

Proceed only after doctor readiness.
