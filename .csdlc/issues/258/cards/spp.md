# Structured Planning Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Publish the reviewed authority-store boundary as the first split from #203, then proceed to #259 and #260.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Seal raw store APIs and add authority-bound adapter/view.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Update compile-required raw-store test fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused validation and pre-PR review.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Raw authority stores are not a production serving surface.
- Test fixture access is explicit and visibly separated from production authority access.
- Transport/peripheral migrations remain separate follow-on scope.

## Risks

- Raw API signature changes require broad fixture token updates.
- Slice still touches several test fixtures because Rust compilation is crate-wide for changed raw signatures.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/258/design.md

Digest: 7111b0edd969a6bda2e853f665ba8a86d249ed43597b60a5ad4f2008f0285439

## Diagram

.csdlc/prepared/issues/258/diagram.mmd

Digest: b7b2752d8925ace806dbf18f75c75678dae6c72cc81e1262ef50a2706aaaa6e1

## Stop Conditions

- review finds actionable P1/P2
- focused validation fails
- typed publication cannot bind issue #258 truthfully

## Handoff

Proceed only after doctor readiness.
