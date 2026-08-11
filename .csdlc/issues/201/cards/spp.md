# Structured Planning Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After WP-04.16a merges, bind and implement the bounded committed authority protocol, retain a private byte-identical operation-specific store-native signed artifact view in each opaque finalized token for sealed #199/#200/#203 consumers, prove deterministic endorsement/time/retry/checkpoint/artifact and legacy-closure behavior, retain merge-safe evidence, resolve exact-head review, and publish a ready unmerged PR before releasing downstream integrations.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "After PR #197 merges ancestrally, bind #201 and freeze canonical committed intent, quorum-attested time, opaque local endorsement, durable result-cache, checkpoint, and legacy-log compatibility contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement only the core authority protocol and retire or fail-close legacy direct authority command application; emit opaque verified operation tokens for #199 and #200 without executing their side effects.",
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
    "action": "Prove quorum endorsement, deterministic time boundaries, exact retry/conflict behavior, init and checkpoint crash windows, rollback, corruption, capacity, path safety, legacy-command rejection, and opaque-token construction.",
    "acceptance_ids": [
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
    "action": "Resolve fresh exact-head review, publish a ready PR closing #201, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No uncommitted proposal, single voter, leader, caller, harness, Shepherd, model, local history, or local clock can mint a verified authority-operation token
- Every finalized token binds exact polis, domain, membership cut, committed index, operation, bounded store-native signed artifact bytes and digest, time token, and distinct quorum endorsements
- The private artifact view returns byte-identical retained committed bytes only to sealed #199/#200/#203 consumers; it cannot accept replacement bytes or reconstruct authority from a digest
- No token, artifact view, or canonical response is published before protocol result, retry record, and external checkpoint reconcile
- Legacy direct authority commands cannot mint or restore authority
- One-of-three always halts new authority token finalization

## Risks

- A public signer or token constructor could restore caller self-attestation
- Replica-local clock checks inside apply could diverge at expiry boundaries
- Initialization or checkpoint crash ordering could publish a result without monotonic authority
- Exact OpenRaft retries could conflict with overly strict sequence rejection
- Legacy log or snapshot replay could silently retain a direct authority bypass
- Scope could drift back into #199 membership or #200 concrete-store behavior

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/201/design.md

Digest: e9a9cf340baeab70b7c80defe670336fc4bb1efbaaae3aac83fbeb224dc914c8

## Diagram

.csdlc/prepared/issues/201/diagram.mmd

Digest: df8a270f7b9e19ae6218701fc9e3d54f3f4c7d8bb9c555139b2914931e625d3f

## Stop Conditions

- PR #197 is not externally reviewed, merged, and ancestral
- The signer or verified token would be publicly constructible or accept caller-produced authority
- Replicated apply would branch on replica-local time or local history
- Protocol publication cannot reconcile initialization, exact retry, and rollback through an external checkpoint
- Implementation expands into #199, #200, kernel continuity, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
