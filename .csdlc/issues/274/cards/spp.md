# Structured Planning Prompt

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Remediate PR362's exact coverage-impact failure after typed publication/review recovery: preserve historical STP product scope, authorize only four coverage mapping/runner contract paths, add meaningful Observatory behavioral coverage to the unchanged 80 percent floor, then revalidate, freshly review, republish, and terminally finish before #275.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Preserve PR362 producer evidence, recover publication/review through typed authority, and retain the exact 186/360 (51.67%) unmapped coverage finding.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Obtain fresh design authority for the exact four coverage mapping/runner contract paths and typed SPP/VPP parity while preserving historical STP product scope.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add only the exact Observatory mapping and feature-bearing runner route, their contract regressions, and meaningful behavior tests until measured module coverage is at least 80 percent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused/unit/Clippy/mapping/runner/coverage/preflight/scope/diff proof, record truthful evidence, obtain a fresh immutable review, republish PR362, require all CI green, and typed-finish before releasing #275.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- At most one exact committed Observatory quorum lease is eligible for a lineage/generation
- No stale, revoked, expired, minority, partitioned, superseded, or conflicting authority can regain eligibility
- Transfer cannot expose overlapping incompatible quorums
- Retry and restart return the exact prior committed result or fail closed
- Receipts and projections are deterministic, bounded, and redacted
- #272, #273, #203, and #205 owned paths remain outside #274

## Risks

- Shared distributed/mod.rs registration could collide with #273 if implementation is not serialized
- Caller-supplied quorum evidence could be mistaken for committed authority
- Transfer could briefly overlap predecessor and successor eligibility
- Revoked or expired authority could revive after replay or restart
- Projection or receipt could leak quorum membership or authority material

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/274/design.md

Digest: e967c5eedc2e256a624abe3b8087fc4233e05f15b69bc7b8407109b88a5d2d65

## Diagram

.csdlc/prepared/issues/274/diagram.mmd

Digest: c03f4ac687b7786ea70f3a825d876658e65b564cddd1634eb9d37ed688141d84

## Stop Conditions

- Any required terminal cache is noncanonical or nonancestral
- Any design or card claims ownership of serving_authority.rs, a Shepherd path, authority_store_adapters.rs, or #205
- Any bind or implementation is attempted before fresh design PASS and separate approval authority
- distributed/mod.rs is required while #273 is not terminal and ancestral
- Implementation requires UI, listener, transport, cloud, provider, or any undeclared product path
- Any validation, review, CI, terminal-cache, or ancestry gate fails

## Handoff

Proceed only after doctor readiness.
