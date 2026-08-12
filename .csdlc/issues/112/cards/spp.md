# Structured Planning Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reach typed pre-bind readiness now; after #111 merges ancestrally, reconcile its concrete conversation contract, bind separately, implement the Runtime-owned authority and audit module plus narrow API invocation and truthful Observatory presentation, run the exact nonzero Rust, API, and real-browser/UI product targets, and obtain exact-head review before publication.

## Plan

Revision 10

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile merged #111 and #113 contracts and the live #112 signed-message ownership without widening into durability, rooms, or attention workflows.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement principal authority, policy intersection, replay defense, bounded refusal, redacted audit, one signed ACIP identity-message contract, and recipient-signed acknowledgement verification.",
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
    "id": "S3",
    "action": "Integrate authority before conversation reservation and provider delivery while preserving the merged Runtime ingress and externally held private-key boundary.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run all exact issue-owned validation targets, resolve independent exact-head review findings, and publish only current reviewed truth.",
    "acceptance_ids": [
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Authentication precedes authorization; authorization and durable redacted audit precede sequence reservation, provider execution, and delivery
- Capability, agent policy, and Polis policy intersect and none may widen another
- Recipient sets, actions, attachments, and conversation scope are exact and cannot be substituted or widened
- Replay, revocation, expiry, rotation, policy uncertainty, and audit uncertainty fail closed
- Preparation evidence is not product proof and cannot satisfy either deferred product validation target

## Risks

- Treating Runtime API authentication as blanket conversation authority could bypass action and recipient policy
- A continuation or single-recipient capability could widen into new contact, attachment, room, broadcast, or cross-Polis authority
- Audit or refusal text could leak private recipient policy, content, credentials, provider payload, or private cognition
- Restart or concurrent requests could admit replay before replay and audit state are restored
- A merged gated contract could change identifiers or integration paths and require typed replanning before bind

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/112/design.md

Digest: 98a528db6b03233108b7780ccb4d1561e49f6aaf95a25d6b45ddb7582f218558

## Diagram

.csdlc/prepared/issues/112/diagram.mmd

Digest: 8123e40f11131f1474ed236413da4886bf8cdd0022a8148005cfff1302897151

## Stop Conditions

- The sole #111 serial gate is open, unmerged, non-ancestral, or lacks the concrete producer contract required by issue #112
- The intended write set collides with ownership established by the merged gated contract or another issue
- Authorization cannot be enforced before sequence reservation, provider execution, and delivery without widening scope
- Public-safe refusal or redacted audit cannot be proven without retaining forbidden content or private policy inputs
- Any requested action would bind, mutate another issue, publish, merge, or treat preparation evidence as product proof

## Handoff

Proceed only after doctor readiness.
