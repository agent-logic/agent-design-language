# Structured Planning Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reach typed pre-bind readiness now; after both exact serial gates merge ancestrally, reconcile their concrete contracts, bind separately, implement one Runtime-owned authority and audit module plus narrow API invocation, run the exact nonzero product targets, and obtain exact-head review before publication.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Hold at pre-bind readiness until #83 and #111 are both terminal, merged, ancestral, and ownership-compatible; then reconcile their exact contracts and replan through typed edits on drift.",
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
    "id": "S2",
    "action": "Implement the typed Layer 8 principal, action-specific capability, policy intersection, replay guard, bounded refusal, and hash-chained redacted audit contracts in the issue-owned Runtime module.",
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
    "id": "S3",
    "action": "Integrate the authority decision before sequence reservation, provider execution, and delivery while preserving the merged ingress and session ownership boundaries.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the exact issue-owned authority-contract and Runtime API integration targets with nonzero selection, resolve all actionable exact-head review findings, and only then hand off for publication.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
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

Digest: b72f31882a4d915230c29accc68f7feced2a5dbe442a6768abdefda2d389ff70

## Diagram

.csdlc/prepared/issues/112/diagram.mmd

Digest: 8123e40f11131f1474ed236413da4886bf8cdd0022a8148005cfff1302897151

## Stop Conditions

- Either serial gate is open, unmerged, non-ancestral, or lacks the concrete producer contract required by issue #112
- The intended write set collides with ownership established by either merged gated contract or another issue
- Authorization cannot be enforced before sequence reservation, provider execution, and delivery without widening scope
- Public-safe refusal or redacted audit cannot be proven without retaining forbidden content or private policy inputs
- Any requested action would bind, mutate another issue, publish, merge, or treat preparation evidence as product proof

## Handoff

Proceed only after doctor readiness.
