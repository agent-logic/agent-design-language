# Structured Planning Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Keep #203 preparation-only while #202 remains under review and #199 remains prepared behind it. After #202 and then #199 are independently reviewed, merged, and ancestral, resync to the resulting exact origin/main, rerun typed validation and doctor, and only then bind #203 to close all enumerated raw store bypasses using the already merged #201 artifact interface and #200 barrier while preserving #208, #205, and #204 authority boundaries.

## Plan

Revision 9

## Steps

[
  {
    "id": "S1",
    "action": "Remain preparation-only until #202 and then #199 are independently reviewed, merged, and ancestral; resync #203 to the resulting exact origin/main and rerun typed validation and doctor before any bind.",
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
    "action": "After the serial gate passes, bind #203 and implement the sealed adapter registry, authority-bound handles, raw-bypass closure, deterministic lease state, local anchors, and exact ordered operations in owned paths; explicitly migrate adl-runtime/src/distributed/polis_runtime.rs and adl-runtime/tests/distributed_runtime_transport.rs off raw Arc<DistributedCertificateStore> ownership without changing #208, #205, or #204 authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove exact forty-four-case behavior, every crash/retry/bounds window, strict Clippy, and merge-safe receipt truth; resolve fresh exact-head review, publish a ready PR closing #203, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No normal-build raw store constructor, mutation, authorization, grant, receipt, or local history creates authority
- Every store use observes the live #200 barrier and a newer Pending generation invalidates retained grants
- Exact private #201 signed artifact bytes and store-native verification remain the sole concrete authority inputs
- Canonical replicated store state, step receipts, results, result digests, and published receipt views contain no node-local wall time, monotonic time, boot-local safety-anchor bytes, or safety-anchor digest
- Each local safety anchor is bound only by a node-local checkpoint and audit record; local NotReady or Unsafe cannot create canonical progress
- AuthorityStoreAdapterRegistry exposes #205 only a read-only opaque PublishedStoreAuthorityReceiptView for an exactly Published OwnerCommit or Fence result and exposes no raw handle or serving decision
- Partial multi-store progress remains denied and is never described as atomic
- Migration and recovery source edits in #203 are signature/handle compatibility only; #204 retains workflow, failure-policy, and execution ownership

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

Digest: 821593327fcf90839f9ed455ae467af9a234eba05f58564de36b029c73349b49

## Diagram

.csdlc/prepared/issues/203/diagram.mmd

Digest: dc93d5b1073997215f11c82fa195a1207ac78d6aae8378a5f09dddd163389ebc

## Stop Conditions

- Issue #202 is not independently reviewed, merged, and ancestral
- Issue #199 is not independently reviewed, merged, and ancestral after #202
- After both prerequisite merges, #203 has not been resynchronized onto the resulting exact origin/main and passed typed csdlc-validate issue plus csdlc-doctor
- Merged #201 lacks the exact sealed store-native artifact view or merged #200 lacks live per-use grant validation expected by the approved #203 design
- Closing raw bypasses requires undeclared production consumer edits that cannot be handled by store-bound validation
- A local-clock refusal would create a durable canonical effect or result
- Implementation expands into #208 Guardian-kernel continuity effects, #205 serving authority, #204 migration/recovery policy, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
