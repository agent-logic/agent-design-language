# Structured Planning Prompt

Template: 1.0.0

Issue: 5905

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add the narrow compatibility contract, prove it with focused tests, review exact head, publish, then reconcile #5800 and the remaining closed v0.92 inventory.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Implement the disposition-conditional typed historical reconciliation contract inside csdlc-finish only",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused success, idempotency, mismatch, ambiguity, provenance, and routine-gate non-regression tests",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Complete exact-head review, publish the implementation PR, shepherd it green, and merge it",
    "acceptance_ids": [
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Using the merged installed binary, reconcile and validate #5800 as a hard canary, then independently reconcile and validate the frozen remaining inventory",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Sole terminal writer
- Exact live identity
- No invented historical review
- No routine gate weakening

## Risks

- A compatibility path could accidentally bypass routine finish gates
- Ambiguous historical PR attribution could be accepted
- Repository migration identity could be misclassified

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5905/design.md

Digest: 414e5560355972a0aa26ff1d315ff309560c5435181732e60d814e4e35a1f470

## Diagram

.csdlc/prepared/issues/5905/diagram.mmd

Digest: 0dd484b4cf6d3f941c7add8505ed941aa54c4d5d93d6e8e68a97642ce86a06e5

## Stop Conditions

- Live GitHub state is ambiguous
- The change requires a competing terminal writer
- Routine finish tests regress

## Handoff

Proceed only after doctor readiness.
