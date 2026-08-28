# Structured Planning Prompt

Template: 1.0.0

Issue: 51

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate dependencies, implement the smallest owned result, run focused proof, obtain one exact-head review, publish, shepherd, finish, and clean up.

## Plan

Revision 2

## Steps

[
  {
    "id": "dependency-gate",
    "action": "Verify all issue-specific dependencies and operator gates",
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
    "action": "Run focused proof and one exact-head review before publication",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No ownership widening
- No action before dependencies
- No secret retention
- Review precedes publication
- Failure preserves prior valid state and evidence

## Risks

- Dependency truth could drift
- A child could cross another issue's write boundary
- Evidence could overclaim external or Runtime state

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/51/design.md

Digest: 9331fb164fc17391a0f8ac52d316ba328b215098a0ed2b2593e4a471349319f4

## Diagram

.csdlc/prepared/issues/51/diagram.mmd

Digest: 86c4420d3e9831bee4f96f9c970fc50134face75d387e87b51a5db7cd3c60df3

## Stop Conditions

- Any dependency is nonterminal or ambiguous
- Any owned-path collision
- Any missing proof target
- Any secret or private material would enter evidence
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
