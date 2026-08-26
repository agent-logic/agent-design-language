# Structured Planning Prompt

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and approve a safe design, bind #294, implement the narrow recovery contract and bootstrap guard, validate focused behavior, obtain independent exact-head review, and publish ready for CI.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Define typed initialized recovery request, validation, atomic state update, and audit.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Reject unsafe authored paths during bootstrap.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused unit and linked-worktree regression fixtures.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate, independently review, and publish without merge; keep #292 blocked pending terminal ancestry observation.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Initialized recovery never creates bound topology
- Recovery is atomic and CAS guarded
- Lifecycle audit remains append-only
- Unsafe artifact paths never reach bind materialization

## Risks

- Recovery could accidentally preserve stale approval
- Path validation could diverge between bootstrap and recovery
- Linked-worktree behavior can differ from primary checkout

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/294/design.md

Digest: aa21ee83435381695a1c97ee82b4e04540c7ee71f8eb5623c2b11dfa7a322309

## Diagram

.csdlc/prepared/issues/294/diagram.mmd

Digest: beac856b22f305cbf45a3fcd48de11fc8b92637cef77c24fa81c22f834709450

## Stop Conditions

- #292 or unrelated root state would be mutated
- Typed lifecycle reports stale or unsafe topology
- Focused validation or independent review fails

## Handoff

Proceed only after doctor readiness.
