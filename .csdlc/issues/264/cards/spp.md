# Structured Planning Prompt

Template: 1.0.0

Issue: 264

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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/264/design.md

Digest: 65d38b667312f95317d70b59f5beafeb959d4623dd78cbc2abf91b7c34c1446b

## Diagram

.csdlc/prepared/issues/264/diagram.mmd

Digest: 36f47ae06a242da01eaa04edda8e44a77a3c68a9b30276e6f01b6b14b22e12ac

## Stop Conditions

- Any dependency is nonterminal or ambiguous
- Any owned-path collision
- Any missing proof target
- Any secret or private material would enter evidence
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
