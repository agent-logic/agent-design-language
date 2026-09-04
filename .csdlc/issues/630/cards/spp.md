# Structured Planning Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add route-specific v3 finish, clean, and cutover behavior; make every authority boundary explicit; prove positive and denial cases with focused tests and issue-owned validation.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current v3 command routing and route modules from the #629 dependency head.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement finish route as non-authoritative typed terminal derivation with fail-closed denial cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement clean route as Git-registration-derived classification/removal planning with distinct outcomes.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement cutover route as approval/provenance/rollback decision packet that cannot execute cutover.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused tests, issue-owned validator, typed validation, diff hygiene, and pre-PR review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- v2 remains live authority
- v3 is fail-closed before cutover
- Terminal authority must be derived, not caller-forged
- Cleanup authority must come from Git worktree registration
- Cutover requires explicit operator approval and rollback evidence
- No v2 source changes

## Risks

- Accidentally modeling caller-provided terminal truth as authority
- Treating filesystem existence as worktree registration
- Collapsing cleanup outcomes into a vague blocked state
- Letting cutover route imply live authority before #505
- Changing v2 source while wiring v3 tests

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/630/design.md

Digest: 952ae80c35f499b7d0fc883a143c05f09f155f816ffed49508b423d9e540e301

## Diagram

.csdlc/prepared/issues/630/diagram.mmd

Digest: 61b71889349ac761f54724c973a1b1e2e48b55771f75043116f692da985c20a8

## Stop Conditions

- A required predecessor branch cannot be represented without conflicts
- v3 route implementation would require live GitHub mutation
- v3 route implementation would require operational cleanup before cutover
- Any proposed change touches csdlc-v2 source

## Handoff

Proceed only after doctor readiness.
