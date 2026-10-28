# Structured Planning Prompt

Template: 1.0.0

Issue: 150

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Coordinate the twenty-package C-SDLC v3 construction graph from contract freeze through writer-fenced cutover, enforcing Decision 11 before transaction work and keeping V3-R01 explicitly deferred. Fail closed on missing terminal, producer, ancestry, dependency, cleanup, or path-ownership proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the exact V3-01 through V3-16 plus V3-R01 ledger, verify six-card and design readiness, preserve all eleven architecture decisions, and record V3-R01 as deferred rather than runnable.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run V3-01 then V3-02; obtain V3-D11 before V3-08; release foundation, repository, state, lifecycle, adapter, command, PVF, review, GitHub, finish, and cutover packages only when every declared predecessor is terminal.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Recompute each child terminal revision, card/index digest, dependency ancestry, and issue-specific producer proof; stop on parity loss, unsupported platform behavior, stale review, failed recovery, or any attempt to start deferred V3-R01.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Verify V3-16 writer-fenced cutover and rollback proof, retain V3-R01 as deferred until rollback expiry, prove no umbrella-owned product edits, and publish independently reviewed lane synthesis.",
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

.csdlc/prepared/issues/150/design.md

Digest: 1ce6e5bf6f772cfa7bbe9162b0603298c3f361305c6bb05e288b1447c382d90a

## Diagram

.csdlc/prepared/issues/150/diagram.mmd

Digest: b90008f5e6594b1c77da5ab54d5aebc619b3bd52d017ae1d4864beff0d1e4bcc

## Stop Conditions

- A child lacks complete readiness
- A dependency or serialization gate is ambiguous
- Coordination would require a product-path edit
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
