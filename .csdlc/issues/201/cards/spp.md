# Structured Planning Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After WP-04.16a merges, bind and implement the bounded committed authority protocol, retain a private byte-identical operation-specific store-native signed artifact view in each opaque finalized token for sealed #199/#200/#203 consumers, and expose to sealed #210 only a read-only ContinuityTransferGrantProjection bound to the exact lineage, SourceCheckpointHandle identity, and byte-identical bundle-handle identity; prove deterministic endorsement/time/retry/checkpoint/artifact, projection-confusion, wrong-lineage, wrong-checkpoint-handle, wrong-bundle-handle, and legacy-closure behavior, retain merge-safe evidence, resolve exact-head review, and publish a ready unmerged PR before releasing downstream integrations.

## Plan

Revision 13

## Steps

[
  {
    "id": "S1",
    "action": "Bind production PrepareAuthority and FinalizeAuthority commands to the actual OpenRaft applied log IDs and an already-approved current authority cut bootstrapped from the verified route cut; reject caller-selected authority, caller indices, legacy replay, relabeling, and stale custody.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Keep opaque signer custody runtime-local and non-serialized, cryptographically bind guardian, certificate generation, boot generation, membership index, intent, committed prepare, finalize time, and actual finalize apply index, and deny every non-replicated route to a sealed artifact or projection.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Withhold authority publication while replicated finalize remains pending, then reconcile each voter's exact result, retry record, journal, and node-local fresh CAS before returning an opaque PublishedAuthorityResult; prove the fixed 47-case contract and a separate real three-voter production OpenRaft path including wrong self-consistent authority-cut denial.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Retain one immutable v6 proof, validate it in full-history and depth-one squash-like history, obtain fresh independent exact-head review, and stop before publication or merge.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No uncommitted proposal, single voter, leader, caller, harness, Shepherd, model, local history, or local clock can mint a verified authority-operation token
- Every finalized token binds exact polis, domain, membership cut, committed index, operation, bounded store-native signed artifact bytes and digest, time token, and distinct quorum endorsements
- The existing private artifact view returns byte-identical retained committed bytes only to sealed #199/#200/#203 consumers; it cannot accept replacement bytes or reconstruct authority from a digest
- The separate #210 projection is constructible only by #201 from an exact finalized continuity-transfer variant and exposes a borrowed read-only bounded transfer grant; every other operation variant and consumer identity is rejected
- The #210 projection binds the exact lineage, SourceCheckpointHandle identity, byte-identical bundle-handle identity, signed bundle/catalog bytes and digests, source, target, route, membership, certificate, boot, entry, chunk, range, bounds, deadline, and cleanup identity without granting transport, filesystem, migration, fence, activation, serving, or store-effect authority
- Wrong lineage, wrong SourceCheckpointHandle, or wrong bundle-handle identity is rejected before #208 source access or #210 transfer-session establishment
- Each of the three voters owns an independent node-local checkpoint authority, records exact expected old and new checkpoints, performs only a fresh compare-and-swap against its own expected old checkpoint, and after a proved partial commit may complete only the exact same operation; no voter or shared checkpoint can publish for another
- No token, artifact view, projection, canonical response, or Raft-apply acknowledgement is published until that voter's canonical result, retry record, and node-local external checkpoint are durable and agree
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

Digest: 673631c5387f72f4d5da4b11b2a467360a565ee306efd9229fa0fa36f409c5bf

## Diagram

.csdlc/prepared/issues/201/diagram.mmd

Digest: 40f6332420e6f7a4301b821cf802ecc3f4bee646a0a9c4c8d6f5da86447dfbf8

## Stop Conditions

- PR #197 is not externally reviewed, merged, and ancestral
- The signer or verified token would be publicly constructible or accept caller-produced authority
- Replicated apply would branch on replica-local time or local history
- Protocol publication cannot reconcile initialization, exact retry, and rollback through an external checkpoint
- Implementation expands into #199, #200, kernel continuity, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
