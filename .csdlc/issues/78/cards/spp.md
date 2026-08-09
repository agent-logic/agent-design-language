# Structured Planning Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the narrow semantic operation and authorization predicate, prove its atomic and fail-closed behavior, install the reviewed binary, and verify issue #73 can consume it.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Add the typed semantic operation, strict replacement validation, and recovery-sensitive authorization.",
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
    "action": "Retain previous and replacement deliverables in audit evidence while preserving unrelated STP fields and atomic projections.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused positive and negative tests and run formatting and strict Clippy under /Volumes/FastWork.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run independent pre-PR review, install exact reviewed binaries, and prove issue #73 can consume the operation.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Only STP deliverables change
- Review recovery provenance is explicit and durable
- Canonical state and all card projections commit atomically
- Stale or drifted truth fails closed
- No mutation weakens exact review or publication authority

## Risks

- Empty review fields could be mistaken for recovery provenance
- Audit text could omit the previous collection
- A broad authorization branch could enable unrelated implemented-phase edits
- Tests could prove only the happy path and miss phase or drift rejection

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/78/design.md

Digest: c3ffd664872a50829810df8c97d34785c04b3bb27acc93e50b5f857c25f56fea

## Diagram

.csdlc/prepared/issues/78/diagram.mmd

Digest: f2a4d69914c4b78a3e2737bb2b1e33da77e7263f2ba63ddfb10e02b6bf206970

## Stop Conditions

- The operation requires a general phase rollback or unrestricted administrative edit surface
- Recovery provenance cannot be proven from durable typed state
- The implementation changes unrelated lifecycle semantics
- Any tracked edit appears on primary main

## Handoff

Proceed only after doctor readiness.
