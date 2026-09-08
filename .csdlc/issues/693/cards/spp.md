# Structured Planning Prompt

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Introduce a Runtime-owned typed A2A action decision boundary compatible with ordinary model output, route accepted actions through the existing governed primitive, then prove the real production conversation ingress through recipient execution and feed observation on isolated local resources.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Trace the exact production prompt/output/dispatch path and define the smallest deterministic first-class A2A action boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement Runtime-owned action selection and governed dispatch while preserving existing admission replay cancellation correlation and failure semantics.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and repeatedly run an isolated production-ingress live-style acceptance with ordinary model output plus focused compatibility tests.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused quality checks obtain exact-head independent review and publish a non-draft PR without merging.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- Natural-language claims never count as dispatch
- Runtime retains all admission and authority decisions
- A2A work is distinct from operator-facing replies
- Replay and cancellation stay governed
- Tests do not touch the permanent Runtime

## Risks

- Natural-language parsing may recreate brittleness
- Action dispatch could bypass existing authority
- A2A results could be confused with user replies
- Fixtures may only prove perfect structured output

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/693/design.md

Digest: 3d6e1f22f907c3dd4a5cdb9cd79f7501e9898ea069400f940bed0de1b12d740d

## Diagram

.csdlc/prepared/issues/693/diagram.mmd

Digest: 258c0894881958fb171a77bd3e94eb4c64b8e3bc18fe553a69567fd872f7e7e3

## Stop Conditions

- Implementation would bypass Layer8 or admission authority
- Validation would require live Runtime mutation or cloud spend
- Scope expands into transcript history or UI redesign
- Primary main becomes tracked dirty

## Handoff

Proceed only after doctor readiness.
