# Structured Planning Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize from isolated staging, design-review and approve, bind to the canonical FastWork issue worktree, add a pre-write primary checkout guard with focused tests and docs, validate, review, and publish.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the issue from an isolated staging checkout",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind to the canonical FastWork issue worktree and implement the lifecycle guard",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused tests and operator docs",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate, independently review, fix actionables, and publish a reviewed PR",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked issue mutation on root main
- primary-checkout rejection must happen before initialization writes
- ambiguous Git topology is a typed fail-closed error before initialization writes
- non-primary staging bootstrap remains supported
- bind policy remains FastWork-bound for ADL
- issue lifecycle writes use typed C-SDLC v2 routes

## Risks

- placing the guard after design or lock creation would leave residue
- letting ambiguous Git topology proceed would recreate primary-checkout pollution under uncertainty
- branch-name heuristics could reject legitimate staging checkouts or miss dirty primary checkouts
- over-broad rejection could break idempotent isolated initialization
- tests that use the real checkout could pollute shared state

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/544/design.md

Digest: f1074cf92f8c511d39cea40810d45a82d5e29d98f1ccaf8d9a211815dc939ddb

## Diagram

.csdlc/prepared/issues/544/diagram.mmd

Digest: 1023787c89cbe48b86c0831d36cfff3d0e7bd8548f97a8e0f4e1db91bf548448

## Stop Conditions

- typed initialization cannot run from isolated staging without polluting primary
- design review finds unresolved actionables
- bind cannot create the canonical FastWork issue worktree
- focused validation fails
- independent exact-head review has unresolved findings

## Handoff

Proceed only after doctor readiness.
