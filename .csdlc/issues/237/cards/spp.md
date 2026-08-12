# Structured Planning Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Introduce the smallest typed continuity-record binding at the capability boundary, update callers, and prove real composition plus substitution and retained negative behavior.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Confirm and independently review the canonical continuity-record design.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the narrow continuity binding and update existing callers.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add real signed composition and substitution regression proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run exact proof, fresh independent review, required CI, merge, and typed finish.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Continuity identity bindings are verified
- Authority and privacy checks are not weakened
- Substitution fails closed
- Positive proof is Runtime-produced

## Risks

- API churn across retained tests
- Accidental acceptance of a digest-only unverified record
- A compatibility shortcut could weaken substitution resistance

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/237/design.md

Digest: 6e6e122d1b8ed8399e47793e9aab12131f107bb66ebba8a573482bc750992c9d

## Diagram

.csdlc/prepared/issues/237/diagram.mmd

Digest: 5cfcdecfc7cf13d3a56ebf2d905ea63c4a59781cbff3c3b9b839e8c78780c9b4

## Stop Conditions

- The fix requires public trust constructors
- The fix requires broader Runtime redesign
- A real signed continuity record cannot be verified without fixture authority
- Any source edit begins before design approval

## Handoff

Proceed only after doctor readiness.
