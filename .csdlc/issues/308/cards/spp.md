# Structured Planning Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reverify the predecessor gate, freeze the exact-revision denominator, inventory child proof read-only, reconcile owned proof surfaces, implement fail-closed coverage checks, validate, and complete exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reverify #256, #340, #341, and legacy #5839 terminal reconciliation and ancestry, then freeze the exact revision denominator",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory child-owned demo, AEE, activation, positive, and negative proof artifacts read-only",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Reconcile the matrix, coverage, activation ledger, and artifact index at the frozen revision",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement and exercise the fail-closed coverage validator and focused negative cases",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Retain exact-revision validation evidence and complete independent exact-head review",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Missing evidence remains missing evidence
- Unaccepted statuses never become accepted proof
- Child proof retains its producing owner and exact revision
- WP-20 does not absorb feature, reduction, refactoring, or release authority

## Risks

- Predecessor terminal truth may be stale, unreconciled, or non-ancestral
- Existing proof documents may disagree on owner, status, command, or revision
- Platform or credential limitations may make a claimed demo non-portable
- A proof row may cite synthetic, missing, or insufficient negative evidence

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/308/design.md

Digest: be86c5e4be6fb6a41ea1de95ee310062a9b437e1918db6c5dd097a7d717ecfa1

## Diagram

.csdlc/prepared/issues/308/diagram.mmd

Digest: 7548efae5b547b96c3ba721da45cf698601d9146c67e997a3bfa0257fef1c27c

## Stop Conditions

- Any predecessor is not terminal, reconciled, and ancestral
- The exact revision denominator cannot be frozen consistently
- Required positive or negative proof is missing or synthetic
- Ownership correction would require modifying child or WP-21/WP-21A surfaces
- Exact-head review reports an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
