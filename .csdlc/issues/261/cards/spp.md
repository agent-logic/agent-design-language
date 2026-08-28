# Structured Planning Prompt

Template: 1.0.0

Issue: 261

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

.csdlc/prepared/issues/261/design.md

Digest: b3c0230d09eb8afe14672d252e912c8ba3f950022cae5b9a9dbcd98f40c68c90

## Diagram

.csdlc/prepared/issues/261/diagram.mmd

Digest: 1a670f37bfe27744ab2847e73187b7e7e195c0f138e23ff21aedc1fc441705ee

## Stop Conditions

- Any dependency is nonterminal or ambiguous
- Any owned-path collision
- Any missing proof target
- Any secret or private material would enter evidence
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
