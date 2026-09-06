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

Digest: a0f25231074724d0711d0603f2b1c95e493832b21001a4d29ea820917bf54bb3

## Diagram

.csdlc/prepared/issues/713/diagram.mmd

Digest: 0130ada9fadf6f4b748d59a8040eb2c97ed4496fff77c960f7c709fed1808c64

## Stop Conditions

- Implementation requires weakening ACIP or API authentication
- Scope collides with unmerged #707 files without a safe base
- History cannot be recovered without inventing missing content

## Handoff

Proceed only after doctor readiness.
