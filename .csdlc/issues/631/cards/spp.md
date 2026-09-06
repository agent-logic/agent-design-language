# Structured Planning Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add typed non-authoritative v3 proof, shadow, soak, and install behavior; prove positive and denial cases; keep install/cutover boundaries explicit.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current v3 one-binary command manifest and route modules from the #627 dependency head.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement proof route manifest validation and stale/missing evidence denial behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement shadow and soak route classification with bounded observations and no hidden authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Implement install route planning with stable artifact provenance and #505 cutover gating.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
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
    "status": "completed"
  }
]

## Invariants

- v2 remains live authority
- v3 is fail-closed before cutover
- Proof evidence must be durable and fresh
- Shadow parity must be exact and bounded
- Install must not depend on disposable Cargo targets
- No v2 source changes

## Risks

- Claiming broad v2/v3 parity from narrow shadow samples
- Treating missing proof evidence as a soft warning
- Hiding live provider or long-running soak side effects in tests
- Letting install imply selector or cutover authority before #505
- Changing v2 source while wiring v3 proof tests

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/631/design.md

Digest: 83d5daad4ba4ca16c67674e2a3b3dc2424ed3622093d01580921a89451b9505a

## Diagram

.csdlc/prepared/issues/631/diagram.mmd

Digest: 71fdfc0eea9950fcdf32585a685dc07197f125b1ed4953721969968bcc317e14

## Stop Conditions

- The #627 command denominator branch is not available
- v3 route implementation would require live provider execution
- v3 route implementation would require selector mutation before #505
- Any proposed change touches csdlc-v2 source

## Handoff

Proceed only after doctor readiness.
