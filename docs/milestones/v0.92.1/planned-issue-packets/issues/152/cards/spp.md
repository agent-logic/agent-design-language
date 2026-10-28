# Structured Planning Prompt

Template: 1.0.0

Issue: 152

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Coordinate integrated review and release only after CORP-08, V3-16, and DRT-07 are terminal, routing all findings before release-candidate and final evidence work. Fail closed on missing terminal, producer, ancestry, dependency, cleanup, or path-ownership proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify exact terminal revisions, ancestry, reviews, and proof inventories for CORP-08, V3-16, and DRT-07; confirm all three INT child packets and owned paths before releasing INT-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run INT-01 independent integrated review, route every actionable finding to an owning lane, and block INT-02 until all required dispositions and residual risks are recorded at exact revisions.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run INT-02 release-candidate inventory and rollback rehearsal, then release INT-03 only after terminal proof; recompute artifact digests, ancestry, review authority, and rollback evidence rather than trusting summary flags.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Verify all three INT children terminal, prove the umbrella touched coordination surfaces only, retain the final cross-lane evidence index, obtain exact-head review, and publish without asserting release authority beyond the issue contract.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The umbrella may coordinate and synthesize but cannot modify child-owned product paths.
- Children retain exclusive implementation and review ownership.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- Umbrella scope could absorb child work
- A stale status could start a child early

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/152/design.md

Digest: 2a9b03e60dfd150de8fd7b4ad8a7ddb906a3f443d587d3f843bb3447b9103f43

## Diagram

.csdlc/prepared/issues/152/diagram.mmd

Digest: 437c752b8ff80c3fcdfc37236dd6dee3ba2e6e7c8ae0e8620fef37461d7199ba

## Stop Conditions

- A child lacks complete readiness
- A dependency or serialization gate is ambiguous
- Coordination would require a product-path edit
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
