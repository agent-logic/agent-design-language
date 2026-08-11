# Structured Planning Prompt

Template: 1.0.0

Issue: 5877

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
    "action": "Implement the bounded WP-04.15 outcome in the exclusive paths.",
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

- Only adl-runtime/src/distributed/projection.rs, adl-runtime/tests/distributed_projection.rs, and docs/api/runtime-v3/v1/distributed.openapi.json are mutable
- Integration issue #5878 alone owns route and production module registration
- Expose exactly one authenticated least-privilege redacted v1 view; current authentication has no scope model, so no privileged detail tier or scope-based authorization is claimed
- The projection is read-only and never grants, transfers, reconstructs, or mutates distributed authority
- Every response is one coherent cut whose topology, certificate, failure, lease, placement, migration, and recovery fields derive from the same declared version or revision boundary
- Canonical ordering, stable identifiers, version selection, serialization, and error results are deterministic for identical coherent input state
- Secrets, signatures, private keys, raw credentials, bearer material, internal paths, unbounded diagnostics, and unauthorized detail are always omitted or redacted
- Nodes, edges, certificates, failures, leases, placements, migrations, recoveries, strings, identifiers, response bytes, nesting, and serialization work have explicit hard bounds with checked arithmetic
- The OpenAPI document exactly matches implemented authentication, version, schema, required fields, bounds, redaction, status, and error behavior
- Missing, stale, malformed, incoherent, unauthorized, wrong-domain, oversized, or unsupported-version state fails closed without partial or mixed-cut output
- Execution evidence and independent review remain digest-bound to all three exact protected paths

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

.csdlc/prepared/issues/5877/design.md

Digest: 16575417d19cd1f6fd3a364040400e22de2eeb3ada77cbbc7c9e42c6f169126b

## Diagram

.csdlc/prepared/issues/5877/diagram.mmd

Digest: 205e7e83ffbf8812f537bacc9065673460c7a36bdcb9ca33e80fc0cabd99e5b4

## Stop Conditions

- Stop before binding unless #5909 PR #120, then #5870, then both #5873 and #5874, then #5875, then #5876 are merged and ancestral in that order
- Stop if merged future input paths do not expose enough stable behavior to project one coherent cut without inventing interfaces
- Stop if the design would require authorization scopes, a privileged detail tier, route registration, module registration, or mutation outside the three owned paths
- Stop on any active product-path collision or ownership ambiguity
- Stop if coherent-cut determinism, least-privilege redaction, hard resource bounds, fail-closed errors, or exact OpenAPI parity cannot be proved
- After the three issue-owned paths are implemented, stop if distributed_projection is absent, selects zero tests, or any focused, OpenAPI-parity, or receipt validation fails
- Stop if scope, interface, versioning, registration, authentication, or rollback authority must widen

## Handoff

Proceed only after doctor readiness.
