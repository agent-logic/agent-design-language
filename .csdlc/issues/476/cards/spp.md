# Structured Planning Prompt

Template: 1.0.0

Issue: 476

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #476, transplant only ed454a246, validate the narrow truth repair, obtain a fresh pre-assigned exact-head review, publish and merge when green, then reconcile #315.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #476 and transplant only ed454a246 onto current main.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Run focused validator and diff hygiene.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Obtain fresh assigned exact-head review, publish, shepherd green CI, merge, and reconcile terminal truth.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No runtime behavior changes
- No false terminal claims
- Typed lifecycle remains authoritative

## Risks

- Concurrent remote state drift
- Accidental inclusion of unrelated #315 branch history

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/476/design.md

Digest: c5b758c1b387574d13d230dbc193a73e8916d636c563057f2d31922687690c7f

## Diagram

.csdlc/prepared/issues/476/diagram.mmd

Digest: 0c46999c96328b16c27e31a66ebb7104c3f998eff6d19eabd650e77100d4c965

## Stop Conditions

- Cherry-pick includes paths outside the declared repair
- Typed lifecycle or review authority cannot be established
- Required CI fails for a causal reason outside issue scope

## Handoff

Proceed only after doctor readiness.
