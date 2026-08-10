# Structured Planning Prompt

Template: 1.0.0

Issue: 5874

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify gates, implement the exclusive slice, run exact proving tests and negatives, validate rollback, resolve review, and close through child authority.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5821 terminal ancestry, dependency receipts, exact paths, and source contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the bounded WP-04.12 outcome in the exclusive paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run exact positive, negative, failure, recovery, and receipt validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve independent review and complete child-owned publication and closeout.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Exclusive paths remain disjoint
- Guardian stays process 0
- No insecure or Runtime v2 fallback
- Queues and waits remain bounded
- Evidence is exact-revision and digest bound

## Risks

- Dependency contract drift
- Cross-child path overlap
- False-green zero-test selection
- Self-attested platform or recovery evidence

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5874/design.md

Digest: 7b05fbe4cea6d63828f6ad01c998b0df252e03013b24a7228f658cd3288db99a

## Diagram

.csdlc/prepared/issues/5874/diagram.mmd

Digest: 507b3c8821e811de22357e79f5bc8834f772ca1c01d57a12872ebd4093be2e88

## Stop Conditions

- Corrective issue #5909 PR #120 is not merged or its exact merge revision is not ancestral to the selected execution base
- Issue #5870 is not closed through a merged PR or its exact merge revision is not ancestral to the selected execution base
- The merged certificate or fencing contract cannot support authenticated digest-bound redacted snapshot catalogs and transfer manifests without widening the two owned paths
- Either exact owned path is already present in or collides with another live issue worktree
- After implementation the distributed_snapshot_catalog target is absent, selects zero tests, or any required positive, negative, receipt, or review proof fails
- Module registration, product scope, dependency ownership, transfer authority, or rollback authority must widen beyond issue #5874

## Handoff

Proceed only after doctor readiness.
