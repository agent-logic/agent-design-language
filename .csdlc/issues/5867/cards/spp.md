# Structured Planning Prompt

Template: 1.0.0

Issue: 5867

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify gates, implement the exclusive slice, run exact proving tests and negatives, validate rollback, resolve review, and close through child authority.

## Plan

Revision 4

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
    "action": "Implement the bounded WP-04.05 outcome in the exclusive paths.",
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

- Exclusive paths remain disjoint; Guardian stays process 0; queues and waits remain bounded; no insecure fallback is permitted
- Evidence is exact-revision and digest bound
- Voter promotion rejects a candidate whose effective Guardian control public key belongs to another active voter
- Committed membership snapshots and replay preserve one effective control key per active voter identity
- Membership epochs and convergence remain deterministic and bounded

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

.csdlc/prepared/issues/5867/design.md

Digest: c60e74502ea664909f76263e93e463c3d5ac0449e2b68550ea662a7d4cf0ff43

## Diagram

.csdlc/prepared/issues/5867/diagram.mmd

Digest: 583069f1e5e23de29e40c6d6a2c6f6f2c39c31907ef9d63198e80ae2ae0a8f33

## Stop Conditions

- Issue #5866 is not closed through merged PR #88 or its exact merge revision is not ancestral to the selected base
- The merged discovery, certificate, identity, or transport contract cannot support authenticated membership without widening the two owned paths
- Either exact owned path is already present in or collides with another live issue worktree
- After implementation the distributed_membership target is absent, selects zero tests, or any required proof fails
- Module registration, product scope, dependency ownership, or rollback authority must widen beyond issue #5867

## Handoff

Proceed only after doctor readiness.
