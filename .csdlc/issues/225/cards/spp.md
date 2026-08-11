# Structured Planning Prompt

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review and approve the two-operation design, bind #225, implement exact authorization and audit semantics, prove both accepted paths and adjacent rejection boundaries, obtain exact-head review, and publish a ready unmerged PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Review and approve exact operation, phase, card, topology, recovery, and audit contracts, then bind #225.",
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
    "action": "Implement both semantic operations through the existing store transaction and renderer.",
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
    "action": "Add focused Gate 2 and Gate 5 accepted and fail-closed regression sequences.",
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
    "id": "S4",
    "action": "Run focused validation, resolve independent exact-head review, and publish a ready PR closing #225.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- No correction grants Git topology or execution authority
- Only the two named card fields are mutable through the new operations
- CAS, renderer, AST, cross-card validation, audit, and transaction atomicity remain fail closed
- Pre-bind SIP correction invalidates stale design approval
- Post-recovery SPP correction requires cleared review/publication/readiness truth

## Risks

- Over-broad matching could authorize adjacent fields or phases
- Recovery checks could rely only on phase and ignore retained truth
- Audit serialization could omit the previous value
- Tests could bypass the real editor/store/render path

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/225/design.md

Digest: c6e2eb19dc80c076d34116fba976730dea4aeb23d87ae58c4a3fecb53afac72e

## Diagram

.csdlc/prepared/issues/225/diagram.mmd

Digest: c827e554a7aa14540af5a4a0885a041b8d86a4750b0a653fae4da2e1768dd0ae

## Stop Conditions

- A direct card/state mutation is required
- Either operation cannot remain field- and phase-specific
- Review recovery or binding authority must move into csdlc-edit
- Atomic rendering, audit preservation, or negative proof fails

## Handoff

Proceed only after doctor readiness.
