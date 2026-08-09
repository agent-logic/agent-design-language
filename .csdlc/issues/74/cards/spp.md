# Structured Planning Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Run the exact current-main canary first, retain a focused regression, and touch production only if the relevance-first scan still fails.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Construct an unrelated legacy projection with a retired claim field and run real csdlc-bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add negative cases making the malformed record relevant and creating real ownership collisions.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "If current behavior fails, apply the smallest relevance-first correction; otherwise record a regression-only outcome.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-head review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Canonical IssueRecord remains claim-free and strict
- Unrelated records are not mutated
- Real collisions fail closed
- No broad validation

## Risks

- Mistaking existing #61 behavior for exact claim-field proof
- Accidentally tolerating malformed relevant records
- Expanding into historical data migration

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/74/design.md

Digest: 8db4b8fe501d7ec47cdcac6df0453e3d21c508e2e2f575dc5a68552d10ab9206

## Diagram

.csdlc/prepared/issues/74/diagram.mmd

Digest: 4a09e26fc26523ef5dbc6039d707d2175433db6efaa2d25aca1edb21ae107037

## Stop Conditions

- Fix would require restoring claims
- Historical records would need mutation
- Collision safety would be weakened

## Handoff

Proceed only after doctor readiness.
