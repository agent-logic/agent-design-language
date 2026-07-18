# Structured Planning Prompt

Template: 1.0.0

Issue: 5521

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Establish exact authority, repair #5518 S4, and validate terminal parity.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Establish and bind exact repair authority",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair #5518 S4 and validate terminal parity",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- No source files change
- #5518 remains closed-out and claim-free
- Only S4 advances
- Receipt and local record remain exact

## Risks

- Stale receipt CAS must fail closed
- Unexpected semantic drift must stop publication

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5521/design.md

Digest: 4e65a25513e39209bcb6d1ed624e015a1cc955db8bb05671ce313510d90b3485

## Diagram

.csdlc/prepared/issues/5521/diagram.mmd

Digest: 0132a57dbeabc6dfd96bce54d33b8ed2df792d32c6beeec31517e929f49472e8

## Stop Conditions

- Any source file changes
- Any #5518 field besides generation, digests, audit, and S4 changes
- Receipt parity or doctor fails

## Handoff

Proceed only after doctor readiness.
