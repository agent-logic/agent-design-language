# Structured Planning Prompt

Template: 1.0.0

Issue: 512

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

.csdlc/prepared/issues/512/design.md

Digest: a431b7ceaad708f00cb9e5b9357319202af9e5201397b7e9e64aa5bb9b8448f4

## Diagram

.csdlc/prepared/issues/512/diagram.mmd

Digest: 64cea4146caba55d888a81af764f8cc8aa2837392546c97711acea3e6c354bb8

## Stop Conditions

- Any dependency is nonterminal or ambiguous
- Any owned-path collision
- Any missing proof target
- Any secret or private material would enter evidence
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
