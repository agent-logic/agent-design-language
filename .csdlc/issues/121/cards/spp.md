# Structured Planning Prompt

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind the two-path repair on the exact PR 120 parent, implement operation-sensitive quorum fencing and durable restart floors, run focused proof, resolve exact-head review, and publish a green stacked PR without merging.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind exact PR 120 ancestry and implement only the two-path lease authority repair.",
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
    "action": "Run exact tests, strict Clippy, and machine-derived receipt validation.",
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
    "id": "S3",
    "action": "Resolve fresh independent exact-head review and publish a green stacked PR without merging.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Only committed majority authority advances fence epochs
- Unavailable old-holder keys cannot prevent a quorum fence
- Restart never trusts foreign elapsed time or erases an unresolved portable floor
- All failure paths are atomic and fail closed
- Product scope remains exactly two files

## Risks

- Operation-sensitive proof changes could weaken holder-authorized transitions
- Fence epoch semantics could conflict with activation sequencing
- Recovery-floor cleanup could happen too early
- Snapshot truth could drift from current membership applied index

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/121/design.md

Digest: 478101411c25e58ae2d5cb3e4ab058cbc4cc777a39d4be6745ac2b4a973811dd

## Diagram

.csdlc/prepared/issues/121/diagram.mmd

Digest: 63fb7758b7d4c2447ef019eb70e50012e7b06ff2eab7a953be3bf7b9b886a1ac

## Stop Conditions

- The exact base is not PR 120 reviewed head or an approved descendant
- Any product path outside the two declared files is required
- Fence/revoke safety cannot be proved without weakening majority or activation authority
- Exact tests select zero cases, strict Clippy fails, or machine evidence is self-attested
- Independent review has unresolved actionable findings

## Handoff

Proceed only after doctor readiness.
