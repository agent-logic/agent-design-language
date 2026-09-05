# Structured Planning Prompt

Template: 1.0.0

Issue: 262

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
    "status": "completed"
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
    "status": "completed"
  },
  {
    "id": "validate-review",
    "action": "Run focused proof and one exact-head review before publication",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
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

.csdlc/prepared/issues/262/design.md

Digest: 2ad6b02587c25d6dc494d36dcb29994ab945efd160f460b704c9a2180a6441ad

## Diagram

.csdlc/prepared/issues/262/diagram.mmd

Digest: f2f481181c6f373c16ea49990527b9abbe24ce7852ffbeb7ef4231e87f944394

## Stop Conditions

- Any dependency is nonterminal or ambiguous
- Any owned-path collision
- Any missing proof target
- Any secret or private material would enter evidence
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
