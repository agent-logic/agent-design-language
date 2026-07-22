# Structured Planning Prompt

Template: 1.0.0

Issue: 4759

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the issue-local packet now; later execution re-checks #5384 live merge and ancestry, then implements and validates the integrated activation map.

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
    "action": "Re-check live #5384 merge plus ancestry before future implementation",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the integrated activation map only after #5384 releases execution",
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

- live #5384 merge plus ancestry is required before execution
- #5335 and receipts are audit-only
- preparation does not advance implementation state
- activation surfaces must point to implemented evidence

## Risks

- open #5384 could block later execution
- routing context could be mistaken for implementation evidence
- activation-map text could accidentally imply v0.92 implementation readiness

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/4759/design.md

Digest: 0301800d9f120cd84ac46ae920c0e1102779abf79d4869117b0ab0daaad8ffca

## Diagram

.csdlc/prepared/issues/4759/diagram.mmd

Digest: 24b8889f9279611c5cd348227adf8345db93f9cb8138ee4e016b194d99a4d5c7

## Stop Conditions

- #5384 remains open without an operator-approved evidence blocker
- #5384 merge is absent from current origin/main ancestry
- accepted deployed-product evidence cannot be mapped exactly
- scope pressure asks preparation to implement activation now

## Handoff

Proceed only after doctor readiness.
