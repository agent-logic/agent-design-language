# Structured Planning Prompt

Template: 1.0.0

Issue: 712

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Delete receipt choreography, make direct config validation authoritative, simplify Guardian launch, implement atomic reload, prove behavior, deploy, and publish.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Map and remove the prepared receipt handshake while preserving real security boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement direct Kernel validation/hash readiness and one-child Guardian supervision.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement atomic candidate validation and reload/rollback.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and live regression proof for retained Runtime behavior.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Complete independent review and typed publication.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Invalid config never serves
- Only one Guardian child owns Runtime listeners
- Failed reload preserves prior working state
- Actual security boundaries remain fail closed
- Runtime features remain available

## Risks

- Persisted service arguments may still reference retired receipt files
- Reload crash recovery may select stale backup
- Over-broad removal could weaken executable ownership checks

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/712/design.md

Digest: 6060eff7a14ca1fb7b22a3a117bc1c235c03bd41ec89de981f50869dc9b321fb

## Diagram

.csdlc/prepared/issues/712/diagram.mmd

Digest: 7b242f4cae90cad75edea452eb7acdd4070485a162bb0de32271b173f86a8712

## Stop Conditions

- Simplification requires weakening API, ACIP, identity, replay, or audit security
- A competing Runtime owns the listener
- Existing persisted config cannot be migrated without operator choice
- Review finds unresolved P1 or P2 defects

## Handoff

Proceed only after doctor readiness.
