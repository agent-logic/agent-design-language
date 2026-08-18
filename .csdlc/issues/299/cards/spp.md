# Structured Planning Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #298 is terminal and ancestral, bootstrap and bind #299, implement an exact-authority cleanup command that consumes completed recovery receipts, captures archived projection nodes through exact exchanges, removes only receipt-owned inodes with type-correct operations, persists immutable cleanup receipts, proves restart/idempotence/sentinel preservation, obtains exact-head review, and stops before merge.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "After #298 terminal+ancestral gate, bootstrap/design-approve/bind #299 using the prepared design and diagram.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement cleanup request/result schema, terminal+ancestral gate, and receipt/canonical/archive binding validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement immutable cleanup ledger, private namespace, type-matched placeholders, exact capture exchange, and type-correct removal.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Implement deterministic restart/adoption and idempotent repeat behavior across every cleanup boundary.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused cleanup validation, strict checks, assign exact-head review, fix findings, and publish only after review PASS.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- Every destructive action is receipt-owned and exact-identity-bound
- No cleanup action follows symlinks or trusts path/digest alone
- Unrecorded, replaced, unsupported, or ambiguous state is preserved
- Cleanup receipts are immutable and survive successful cleanup
- A completed cleanup is idempotent

## Risks

- #298 PR #305 is terminal at merge 5a1d3bfda7108bede1572cbd9dc9e2af19d9eedb; #299 must continue to verify cached terminal evidence and ancestry before destructive cleanup.
- Cleanup may require integration with nearby lifecycle/store surfaces, so #299 must preserve unrelated root staging and non-#299 owners.
- Filesystem exchange and mount identity behavior must remain portable across macOS/Linux.
- Over-broad cleanup could delete unrelated state unless every operation is exact receipt-bound.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/299/design.md

Digest: 33db4cc9f41106f4ad0ae5b002a3fb6befc8454ca5f156e6dd0e6f7b62219f32

## Diagram

.csdlc/prepared/issues/299/diagram.mmd

Digest: af7a53b44961b3010199ac739e88d0ea14d8e0526e5523c5a3681a51bdf80d77

## Stop Conditions

- #298 is not terminal and ancestral
- Main is not clean before bootstrap/bind
- Any required implementation path collides with active #298 freeze
- Receipt/canonical/archive evidence does not agree exactly
- A test requires recursive deletion or path-authoritative cleanup semantics
- Exact-head review reports an actionable finding

## Handoff

Proceed only after doctor readiness.
