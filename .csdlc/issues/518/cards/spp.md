# Structured Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Verify the predecessor gate, implement the smallest owned result, run focused proof, obtain one exact-head review, publish, shepherd, finish, and clean up.

## Plan

Revision 1

## Steps

[
  {
    "id": "dependency-gate",
    "action": "Verify every declared predecessor and authority gate",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "implement",
    "action": "Implement the bounded owned result",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "validate-review",
    "action": "Run focused proof and one independent exact-head review",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No ownership widening
- No action before dependencies
- No authority substitution
- Review precedes publication
- Failure preserves prior valid state and evidence

## Risks

- Dependency truth could drift
- A child could cross another issue's write boundary
- Evidence could overclaim exact candidate or terminal state

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/518/design.md

Digest: 4c5484baef85bd46f4901f1ab37dc9c01a0461648447850a76e14175ecc8346d

## Diagram

.csdlc/prepared/issues/518/diagram.mmd

Digest: 96a83b9f087d1d15702472c407c660766ff27f3bd4f6d6d2d827dfd84163ad70

## Stop Conditions

- Any predecessor is nonterminal or ambiguous
- Any owned-path collision
- Any missing or zero proof target
- Any unsupported completion claim
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
