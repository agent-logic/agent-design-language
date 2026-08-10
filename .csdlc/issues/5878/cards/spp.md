# Structured Planning Prompt

Template: 1.0.0

Issue: 5878

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
    "action": "Implement the bounded WP-04.16 outcome in the exclusive paths.",
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

- Only adl-runtime/src/distributed/mod.rs, adl-runtime/src/lib.rs, adl-runtime/tests/distributed_guardian.rs, adl/tools/validate_v092_distributed_guardian.sh, and adl/tools/validate_v092_distributed_native_receipts.rb are mutable
- Issue #5878 solely owns production distributed module and route registration; all #5863-#5877 product paths remain read-only terminal inputs
- Registration exposes exactly the terminal reviewed sibling contracts and never substitutes, forks, or reimplements their authority logic
- Guardian remains process 0 and every distributed API, WSS, transport, enrollment, lease, fencing, migration, recovery, and projection path remains authenticated and fail closed
- Integrated topology, authority, failure, lease, placement, migration, recovery, and projection state is one deterministic coherent cut with stable ordering and exact OpenAPI behavior parity
- Queues, frames, nodes, peers, candidates, histories, snapshots, retries, waits, timeouts, strings, logs, artifacts, and total evidence bytes have explicit hard bounds with checked arithmetic and finite cancellation
- Secrets, signatures, private keys, bearer material, credentials, internal paths, and unauthorized diagnostic detail are redacted from API, WSS, logs, artifacts, and receipts
- The exact distributed_guardian target selects nonzero tests and proves production registration, authenticated API and WSS continuity, partitions, fencing, migration, recovery, shutdown, disable, and rollback behavior
- Native receipts prove macOS, Linux, and Windows exactly once each from actual production commands, distinct run identifiers and runner identities, exact protected source revision, retained logs and artifacts, and recomputed digests; self-attestation is rejected
- Rollback removes or disables distributed registration and launch integration, keeps remote ownership fenced, and preserves the unchanged durable single-node Guardian path
- Execution evidence and independent review remain digest-bound to all five exact protected paths and the complete fifteen-child denominator

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

.csdlc/prepared/issues/5878/design.md

Digest: 509558ed6cfbd8dd18ae62b4d197e18490063cbd9d93840b9b25f40e303313ec

## Diagram

.csdlc/prepared/issues/5878/diagram.mmd

Digest: 6c8b714c4987361c7d51ad9a3fffcdf0ff3f94a7dd8b7e6ac7d1e392a173caf0

## Stop Conditions

- Stop before binding unless #5909 PR #120 and exactly every child #5863 through #5877 are merged, closed, and ancestral with #5909 preceding #5870 and all downstream serial gates satisfied
- Stop if any sibling input path, terminal receipt, reviewed source revision, ownership mapping, or fifteen-child denominator entry is missing, ambiguous, stale, or inconsistent
- Stop on any active path collision or any requested mutation outside the five canonical owned paths
- Stop if production registration would require reimplementing sibling logic, inventing interfaces, weakening authentication, or diverging from the issue-owned OpenAPI contract
- Stop if coherent integration, deterministic ordering, redaction, hard bounds, finite cancellation, rollback or disable safety, or fenced remote ownership cannot be proved
- After all five owned paths are implemented, stop if distributed_guardian or either owned validator target is absent, the exact integration target selects zero tests, or any integration, native, receipt, or diff validation fails
- Stop if any macOS, Linux, or Windows receipt is missing, duplicated, self-attested, shares a run identity, lacks production command logs or artifacts, or is not bound to the exact protected source revision
- Stop if scope, registration, interface, platform, authority, proof, or rollback behavior must widen

## Handoff

Proceed only after doctor readiness.
