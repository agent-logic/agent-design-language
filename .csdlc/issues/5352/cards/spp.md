# Structured Planning Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the issue-local packet now; later execution re-checks live merge and ancestry, then writes and validates the exact-revision handoff ledger.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Generate issue-local C-SDLC v2 cards, design, and diagram",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Re-check live dependency merge plus ancestry before future implementation",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the exact-revision handoff ledger only after dependencies release execution",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact pre-PR review during later execution",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is required before execution
- receipts are audit-only
- preparation does not advance implementation state
- all claims remain exact-revision and evidence-bound

## Risks

- open #5361 or #5384 could block later execution
- historical receipts could be mistaken for current ancestry
- handoff text could accidentally imply v0.92 implementation readiness

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5352/design.md

Digest: 2a5d7686733d7b8d77c0dfd437f17216e3a900c7dd9e01db3cf1be8d8fb9723e

## Diagram

.csdlc/prepared/issues/5352/diagram.mmd

Digest: 6b4e05cd0ff006a75623c4406ad4655a413ef756af56d0734b01503d4e1ef924

## Stop Conditions

- #5361 or #5384 remains open without an operator-approved evidence blocker
- required dependency merge is absent from current origin/main ancestry
- handoff evidence lacks exact revision or rollback truth
- scope pressure asks preparation to implement v0.92

## Handoff

Proceed only after doctor readiness.
