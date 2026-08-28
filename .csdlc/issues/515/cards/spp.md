# Structured Planning Prompt

Template: 1.0.0

Issue: 515

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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/515/design.md

Digest: 539bd6fbf2fffb0fb19b489cb597a72bdcbd3bc26472a07300cea9dd0c10e6c2

## Diagram

.csdlc/prepared/issues/515/diagram.mmd

Digest: 62dd8b9b7dfba9b51b70c72721ea7c2072d38f7b7e14b92720b32c6052e08c67

## Stop Conditions

- Any predecessor is nonterminal or ambiguous
- Any owned-path collision
- Any missing or zero proof target
- Any unsupported completion claim
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
