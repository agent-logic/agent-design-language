# Structured Planning Prompt

Template: 1.0.0

Issue: 629

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement #629-owned remote/publication command dispatch and typed evidence contracts, prove rejection of forged/stale authority, then publish only after exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "629-1",
    "action": "Verify #627 command manifest availability and #629 route ownership.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "629-2",
    "action": "Implement #629 one-binary route dispatch and typed remote/publication reports.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "629-3",
    "action": "Add focused remote/publication tests and real issue canary.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "629-4",
    "action": "Run typed validation, exact-head review, and publication readiness.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- One binary remains named csdlc.
- C-SDLC v3 remains non-authoritative before #505.
- No csdlc-v2 source changes.
- Remote authority must come from authenticated readback, not caller strings.

## Risks

- GitHub publication behavior can accidentally become caller-forgeable.
- Stacked PR dependencies can hide closing-linkage behavior.
- Credentials can leak through argv/debug output if not redacted.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/629/design.md

Digest: 7fb92a9d1a316579eecfd3770f73944b5e0de8ad994a2c6a9b570b1b31252881

## Diagram

.csdlc/prepared/issues/629/diagram.mmd

Digest: f90355de9124a86eeb577d5cfdf7a028d0e2d6b4ff26de06e04352aa2bff98f1

## Stop Conditions

- Need to edit csdlc-v2 source.
- Need merge/finish/cleanup/cutover authority.
- Need raw gh lifecycle write.
- Cannot prove live PR readback without operator-approved typed route.

## Handoff

Proceed only after doctor readiness.
