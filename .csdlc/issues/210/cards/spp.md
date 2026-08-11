# Structured Planning Prompt

Template: 1.0.0

Issue: 210

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #191/#201/#203/#208 merge ancestrally, bind #210; implement the sealed transfer session, signed expected entry/chunk/range verification, crash-reconcilable bytes/verifier/prefix advancement, bounded resources, VerifiedTransferPossession, and #208-owned cleanup requests; prove exactly 45 cases plus the SHA-256-bound 8-acceptance/80-subassertion map serially through tests, Clippy, recorded-base-to-source diff hygiene, producer, fresh independent exact-head review, and distinct validator; then publish a ready unmerged PR before #204.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "After dependencies merge ancestrally, bind #210 and freeze exact token, route, bundle-handle, frame, prefix, verifier and cleanup interfaces.",
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
    "action": "Implement the typed session, bounded stream, durable prefix/retry, incremental verification, #208 handle integration and exact abort cleanup.",
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
    "action": "Prove forty-five exact cases, crash/restart/reply-loss/bounds/cleanup/redaction, strict Clippy and merge-safe receipt truth.",
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
    "action": "Resolve fresh exact-head review through a subagent, publish a ready PR closing #210, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Only #201's exact private ContinuityTransferGrantProjection may establish the exact source-to-target session; wrong variant, consumer, source, target, route, membership, certificate, boot, lineage, domain, polis, deadline, or cleanup identity is denied
- Every frame boundary revalidates the live #191/#203 route and identity cut; drift after an accepted prefix stops further I/O and preserves reconcilable stage plus cleanup authority
- #210 and #208 independently verify the retained signed manifest/catalog key generation and exact entry order/schema/range/length/digest plus chunk index/range/digest/predecessor before any target write
- Durable bytes, incremental verifier state, and accepted prefix advance only as one journaled crash-reconcilable effect after signed expectation verification; acknowledgment follows the reconciled prefix
- Frame bytes, frame/entry/chunk/service/file counts, total bytes, queued frames, in-flight requests, open source readers, open target stages, concurrent transfers, journals, caches, and diagnostics are bounded with N+1 denial before effect
- Exact duplicate frames and results are cache-first; conflict, gap, reorder, overlap, final-frame mismatch, range overflow, rollback, corruption, or ambiguous recovery never advances state
- VerifiedTransferPossession exists only after full signed catalog, every entry/chunk, total length, and whole digest verify, and contains only exact #208 TargetStageHandle plus TargetPossessionEvidence
- TargetCleanupPermit is separate from transfer authority, remains valid after transfer expiry/cancellation/restart, permits only #208-owned exact-stage discard, and terminates only with TargetDiscardReceipt or TargetActivationReceipt
- #210 never deletes, activates, resumes a source, fences, owns, serves, or decides migration; #208 owns effects and cleanup while #204 owns migration/control decisions
- No caller path, mock, synthetic snapshot, retained boolean, local absence claim, transfer receipt, possession evidence, or redacted proof creates ownership or serving authority

## Risks

- Generic transport dispatch could bypass typed session authority
- Whole-bundle buffering or frame queues could exceed memory bounds
- Acknowledgment before durable prefix could corrupt resume
- Cancellation or reply loss could duplicate accepted bytes or false-complete
- The issue could drift into migration/fence/serving/cloud policy

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/210/design.md

Digest: bfe137499e1fec51c6eff2c65773f51a10d9488b5ad94ea365fd381d354b0adc

## Diagram

.csdlc/prepared/issues/210/diagram.mmd

Digest: fca1602e1956dbdc13cf1a9dd1bda620246bd281ff007e8aab503165ddaa1923

## Stop Conditions

- Any dependency is not externally reviewed, merged, and ancestral
- Merged #191 cannot host a closed typed non-Raft service without reopening generic message authority
- Merged #208 does not expose opaque bounded bundle reader and isolated-stage writer handles
- Incremental verification cannot bind the exact signed catalog without weakening snapshot authority
- Implementation expands into #204 policy, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
