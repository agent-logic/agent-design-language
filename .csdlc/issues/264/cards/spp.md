# Structured Planning Prompt

Template: 1.0.0

Issue: 264

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize #264, validate terminal podcast prerequisites, implement the non-submission gate packet, run focused proof, obtain exact-head review, publish, finish if the blocked-disposition PR is accepted, and leave external provider work gated.

## Plan

Revision 1

## Steps

[
  {
    "id": "dependency-gate",
    "action": "Verify #261, #262, and #263 terminal truth.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "implement-gate-packet",
    "action": "Create authorization, ledger, monitoring, rollback, and parent-handoff materials without provider mutation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "validate-review",
    "action": "Run focused proof and exact-head review before publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- No provider account action without future explicit operator authorization
- No secret retention
- No unsupported provider acceptance or public-launch claim
- Destination links activate only after live canonical provider URLs are verified
- Parent #51 truth remains a coordination view, not child implementation

## Risks

- A future operator may need to choose per-provider visibility and terms decisions.
- Provider UI behavior can change after the #263 sampled official instructions.
- A blocked disposition can be overread as submission completion unless the packet is explicit.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/264/design.md

Digest: 1dca3f7e0d976a470f5932e16c41c5848f8339b275c5fe44db2442e5dc688a64

## Diagram

.csdlc/prepared/issues/264/diagram.mmd

Digest: 7cff9f020a0de84494367ffda2d2ac744d6c8fd94a45db18c676f37bc7c436be

## Stop Conditions

- Any terminal dependency validation fails
- Any provider account action would be required
- Any credential, verification code, mailbox content, or private provider data would be retained
- Any claim implies a directory listing is live or accepted
- Any unresolved actionable review finding

## Handoff

Proceed only after doctor readiness.
