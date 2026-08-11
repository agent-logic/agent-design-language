# Structured Planning Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After every dependency merges, bind #203, close raw store bypasses, implement sealed deterministic adapters and local safety anchoring, prove all forty-four cases, independently review, and publish a ready unmerged PR before #205/#204.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After all five dependencies merge ancestrally, bind #203 and freeze exact #201 artifact views, #200 grant semantics, store ordering, and canonical/local time boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the sealed adapter registry, authority-bound handles, raw-bypass closure, deterministic lease state, local anchors, and exact ordered operations in owned paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove exact forty-four-case behavior, every crash/retry/bounds window, strict Clippy, and merge-safe receipt truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a ready PR closing #203, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No normal-build raw store constructor, mutation, authorization, grant, receipt, or local history creates authority
- Every store use observes the live #200 barrier and a newer Pending generation invalidates retained grants
- Exact signed artifacts and store-native verification remain the sole concrete authority inputs
- Canonical replicated results never depend on a replica-local wall or monotonic clock
- Partial multi-store progress remains denied and is never described as atomic

## Risks

- Compatibility code could retain a same-crate raw authority bypass
- Changing store constructors could ripple beyond the declared paths
- Replica-local elapsed values could leak into canonical state or receipts
- A crash between floor and ledger effects could be mistaken for completed revocation
- The issue could drift into new serving or migration authority already split to #205/#204

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/203/design.md

Digest: 230e1ca4ff6bfe8364d5b76c20765083a2cb58b3ac0ec8689c2233c377e55544

## Diagram

.csdlc/prepared/issues/203/diagram.mmd

Digest: c2c1ceb90f12b08e8c6df1995969067f810fcc49b248b989ca93da13d6665f4e

## Stop Conditions

- Any dependency is not externally reviewed, merged, and ancestral
- Merged #201 lacks a private exact store-native artifact view or merged #200 lacks live per-use grant validation
- Closing raw bypasses requires undeclared production consumer edits that cannot be handled by store-bound validation
- A local-clock refusal would create a durable canonical effect or result
- Implementation expands into #205, #204, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
