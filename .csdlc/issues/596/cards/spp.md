# Structured Planning Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Create the typed lifecycle record, repair the idempotency and durability defects found by review, rerun focused proof, push the remediation branch, and re-check PR #597 without touching main.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Create canonical typed six-card lifecycle state for #596.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify and preserve PR #597 linkage so it closes #596 only and marks #505/#534 as Part-Of.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Make typed PR update idempotency operation-key bound and add regression proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Close the v3 durable projection crash window and add regression proof.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused validation, push, and refresh PR #597 readback.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Primary checkout remains clean on main
- V2 remains the only live lifecycle authority
- PR #597 closes only #596
- Durable recovery must fail closed after committed state but missing projections
- Operation keys must be replay-safe and conflict-detecting

## Risks

- Accidentally closing #505
- Leaving PR transport as caller-forgeable or replay-unsafe
- Claiming v3 cutover before independent #505 review
- Recording local canaries as stronger evidence than they are

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/596/design.md

Digest: 49da6d996cd3ad753d225fc7301126f732ce2dd815a7fc5378642174997301bb

## Diagram

.csdlc/prepared/issues/596/diagram.mmd

Digest: e1ab7ebed0bad7574ff6ba639fdeccba760dc9fd07a54c65c3f59779a3920ba9

## Stop Conditions

- Primary checkout is no longer clean on main
- PR #597 body would close #505
- Typed C-SDLC v2 owner refuses required lifecycle mutation
- Fix requires widening beyond sprint remediation

## Handoff

Proceed only after doctor readiness.
