# Structured Planning Prompt

Template: 1.0.0

Issue: 516

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

.csdlc/prepared/issues/516/design.md

Digest: eb11d4473d4e8f17270ab1b3c7c3009d894dc9ca3f3491dd8601291fc04294ea

## Diagram

.csdlc/prepared/issues/516/diagram.mmd

Digest: b17628d28cc9a213a14a19ec34b52a3c1ec6575d67e971837d89a5f822515540

## Stop Conditions

- Any predecessor is nonterminal or ambiguous
- Any owned-path collision
- Any missing or zero proof target
- Any unsupported completion claim
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
