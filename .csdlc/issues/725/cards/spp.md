# Structured Planning Prompt

Template: 1.0.0

Issue: 725

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inventory v2 coupling, introduce native v3 authority evidence, migrate readers and tests, then prove from a no-v2 worktree.

## Plan

Revision 2

## Steps

[
  {
    "id": "inventory",
    "action": "Inventory v2 source selector and build dependencies in v3",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "migrate",
    "action": "Implement native v3 selector receipt and migrate operational readers",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "proof",
    "action": "Update proof install shadow tests and migration documentation",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "canary",
    "action": "Run focused validation and fresh no-v2 worktree canary",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- exact HEAD is immutable review authority
- lifecycle digest uses CAS
- uncertain remote mutation fails closed until authenticated readback reconciles
- authority is not caller controlled

## Risks

- cutover evidence migration could accept stale authority
- test fixtures could retain hidden v2 paths
- Cargo graph could retain an indirect v2 dependency

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.adl/requests/725-design.md

Digest: e9541aa96765297962e87486eac521bfb12c82e68c701640b32210ec6fb641cd

## Diagram

.adl/requests/725-diagram.md

Digest: 85cb22afe120bcad0e24b0aaf5beda848f19832e29524509315ef5632e6e212b

## Stop Conditions

- root main becomes dirty
- issue ownership ambiguity
- exact-head or digest guard would need weakening

## Handoff

Proceed only after doctor readiness.
