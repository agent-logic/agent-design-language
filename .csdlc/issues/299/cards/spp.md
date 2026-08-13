# Structured Planning Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #298 is terminal and ancestral, bootstrap and bind #299, implement an exact-authority cleanup command that consumes completed recovery receipts, captures archived projection nodes through exact exchanges, removes only receipt-owned inodes with type-correct operations, persists immutable cleanup receipts, proves restart/idempotence/sentinel preservation, obtains exact-head review, and stops before merge.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #298 terminal+ancestral gate, bootstrap/design-approve/bind #299 using the prepared design and diagram.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement cleanup request/result schema, terminal+ancestral gate, and receipt/canonical/archive binding validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
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
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement deterministic restart/adoption and idempotent repeat behavior across every cleanup boundary.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused cleanup validation, strict checks, assign exact-head review, fix findings, and publish only after review PASS.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Every destructive action is receipt-owned and exact-identity-bound
- No cleanup action follows symlinks or trusts path/digest alone
- Unrecorded, replaced, unsupported, or ambiguous state is preserved
- Cleanup receipts are immutable and survive successful cleanup
- A completed cleanup is idempotent

## Risks

- #298 PR #305 is currently open/conflicting and not terminal
- Cleanup may require integration with surfaces frozen by #298 ownership
- Filesystem exchange and mount identity behavior must remain portable across macOS/Linux
- Over-broad cleanup could delete unrelated state unless every operation is exact receipt-bound

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/299/design.md

Digest: 3e97c9192dd6fc95fef49a103d9399a13e5999956ee72aff4c81a5636c5d9d6b

## Diagram

.csdlc/prepared/issues/299/diagram.mmd

Digest: b50298f4d85aac55fd158bc66039d9add2a12ff720209de69378c59d57c5ab45

## Stop Conditions

- #298 is not terminal and ancestral
- Main is not clean before bootstrap/bind
- Any required implementation path collides with active #298 freeze
- Receipt/canonical/archive evidence does not agree exactly
- A test requires recursive deletion or path-authoritative cleanup semantics
- Exact-head review reports an actionable finding

## Handoff

Proceed only after doctor readiness.
