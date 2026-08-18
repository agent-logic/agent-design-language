# Structured Planning Prompt

Template: 1.0.0

Issue: 363

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review bind implement the narrow recovery-epoch predicate and regression then publish and terminally finish before #274 resumes.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Review exact recovery-epoch design.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement bounded predicate and focused regression.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate review publish CI and finish.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Review and publication truth always invalidate correction authority
- Unknown intervening operations fail closed
- Audit remains append-only and CAS guarded

## Risks

- Allowlist could admit an authority-changing operation
- Recovery epoch could cross an unrelated lifecycle transition

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/363/design.md

Digest: ffc518fee91e4d11bfa2a64f647f2016a56576f23eec9e1543cb05239831b399

## Diagram

.csdlc/prepared/issues/363/diagram.mmd

Digest: a76f8408e50e481eb39be17e58470f56c0c18327e19cc79b4ee2eca5cdb5ba48

## Stop Conditions

- Any #274 mutation
- Any generic set_field authorization
- Any review publication or terminal weakening

## Handoff

Proceed only after doctor readiness.
