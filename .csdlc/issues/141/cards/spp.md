# Structured Planning Prompt

Template: 1.0.0

Issue: 141

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Strengthen the receipt contract and regression first, then derive and commit #5909 terminal truth, validate, review, and publish.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Require exact structured strict-Clippy command proof and add focused regressions.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Reconcile #5909 records with merged PR #120 and closed issue truth.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused validation, independent review, and publish.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Opaque artifacts never imply command success
- Terminal truth comes from live GitHub state
- No Runtime product paths change

## Risks

- Legacy receipt compatibility
- Stale lifecycle projection
- Overbroad proof acceptance

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/141/design.md

Digest: b7605425cc7ba607d04228ff87b30dc51caa82d9bc71db7ce0d850b0ec30ce29

## Diagram

.csdlc/prepared/issues/141/diagram.mmd

Digest: a7370ff66e7b198a86345ce4516f241919cdbe5068294885201410b4fa226790

## Stop Conditions

- Runtime product changes become necessary
- The validator would need to weaken existing receipt checks
- Live GitHub terminal state is ambiguous

## Handoff

Proceed only after doctor readiness.
