# Structured Planning Prompt

Template: 1.0.0

Issue: 449

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind a FastWork worktree, implement a production resident adaptive-learning module and actual resident-cycle integration proof, then publish only after focused code proof and fresh exact review pass.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Validate live issue scope, dependency gates, typed initialization, and design readiness.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind #449 in a FastWork worktree and implement resident-cycle production input mapping to execute_governed_adaptive_learning without fabricated capability/profile handles.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement MutationGate-only accepted adaptation application and non-mutating terminal evidence for rejection/cancellation/fail-closed cases.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement restart/rehydration continuity and tamper/rollback/gap/lineage fail-closed behavior.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused production-path proof through actual resident cycle accepted/rejected/restart/deterministic continuation and update feature/evidence truth only after proof passes.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No graph/state mutation occurs outside MutationGate.
- Dependency-gated capability/profile handles are not fabricated.
- Rejected/cancelled/invalid proposals retain terminal evidence and make no mutation.
- Restart/rehydration cannot replay or duplicate accepted mutations.
- Private profile/provider content is not leaked in evidence.
- #446 tool-actuation scope remains separate.

## Risks

- Sibling capability/profile production handles may not be terminal when #449 implementation reaches AC2.
- Resident-cycle test proof may expose broader runtime fixture coupling.
- Durable adaptive history may need bounded API changes to avoid replay/duplication.
- Feature/evidence docs must not overclaim production integration before exact proof.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/449/design.md

Digest: 8e784134583c99f0be41b3b5c9428b22ff97f938ef41d563d1f2a8ea2d5ecacd

## Diagram

.csdlc/prepared/issues/449/diagram.mmd

Digest: 47dbc64f3e11f0b6758591b6588b98db9ada2eaed61ec4d321f4fc6276d9ffc4

## Stop Conditions

- Capability/profile production handles are unavailable and no truthful dependency-gated implementation path remains.
- A proposed fix requires absorbing #446 ACC tool-actuation authority.
- Mutation would bypass MutationGate.
- Production-path proof collapses to fixture-only or benchmark-only evidence.
- Required source changes would touch unrelated #341/#343/#84/#122/#251 scope.

## Handoff

Proceed only after doctor readiness.
