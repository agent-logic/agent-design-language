# Structured Planning Prompt

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Unify both halves of an A2A exchange into durable history, expose it through the authenticated API, restore it in Observatory, and prove recovery live.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Map existing A2A delivery, reply, history, checkpoint, and Observatory projections.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the durable symmetric causal transcript record and recovery.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Expose the authenticated API projection and Observatory restoration.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and live bidirectional recovery proof.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Complete independent review and typed publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime is transcript authority
- Agent identities and causal IDs never drift
- Replay cannot duplicate history
- Secrets and hidden prompts remain private

## Risks

- Existing events may lack one half of the exchange
- Restart projection may reorder turns
- UI may conflate operator and A2A replies

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/713/design.md

Digest: adad7d9a926a1644bdc559143556f5d9645f6ef943c819a7b6505195dee72286

## Diagram

.csdlc/prepared/issues/713/diagram.mmd

Digest: 11880d2638e636d21fbe24fcd43d3be5705abd7e03f72faec42c72f62967f657

## Stop Conditions

- Implementation requires weakening ACIP or API authentication
- Scope collides with unmerged #707 files without a safe base
- History cannot be recovered without inventing missing content

## Handoff

Proceed only after doctor readiness.
