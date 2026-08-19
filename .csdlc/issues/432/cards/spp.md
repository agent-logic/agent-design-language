# Structured Planning Prompt

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inventory, relocate authority, remove tracked residue, add guards, and prove fresh-checkout behavior.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Retain exact tracked-path and active-reference inventories with dispositions.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Relocate worktree policy and update every active consumer and focused test.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Remove every tracked .adl path and add deterministic reintroduction guards.",
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
    "action": "Run focused and fresh-checkout proof, then obtain exact-head review.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- The FastWork parent remains unchanged
- .csdlc remains canonical
- No .adl fallback authority exists
- Operator-local state is preserved

## Risks

- Historical mentions may be confused with active dependencies
- Removing local logs from the index must not promote them elsewhere
- Fresh-checkout bind behavior may drift if policy resolution is incomplete

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/432/design.md

Digest: 71b7847480be3b2afee82ddabc3e15450761b5b7a6d7a1dc2b8b5bb36232933d

## Diagram

.csdlc/prepared/issues/432/diagram.mmd

Digest: 8ee0ac2ef0807b81b378a9bc239fb7374a61fc84eb7bd1c47725ac3bdb27e85a

## Stop Conditions

- Any active consumer still resolves .adl authority
- The tracked denominator is nonzero
- A replacement would expose local or sensitive data
- Fresh-checkout positive or negative policy proof fails

## Handoff

Proceed only after doctor readiness.
