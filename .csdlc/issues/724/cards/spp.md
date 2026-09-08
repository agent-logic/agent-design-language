# Structured Planning Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Stack on #721, add a narrow flag parser and typed request builder that reuses the existing issue-create dispatch, prove fail-closed and receipt behavior, document both interfaces, and publish a reviewed stacked PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the exact #721 dependency and shared dispatch invariants.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement strict simple-form parsing and typed dispatch projection without an alternate transport.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused positive and adversarial command tests, including inactive-authority refusal and reconciliation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Document the simple form first and request-file form as advanced/audit, then obtain exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Both CLI forms reach the same operational dispatch
- No remote side effect occurs before authority and input validation
- Receipts and readback remain mandatory
- No credential is accepted in ordinary issue-content flags

## Risks

- Convenience parsing could create a second weaker execution path
- Ambiguous body flags could yield unintended content
- Authority metadata could be guessed instead of resolved
- Tests could prove parsing without proving shared dispatch

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/724/design.md

Digest: 9f70832f8e8ebe716f4762a37a2dbd06fed82a0904a9ada33818166a8991f7d2

## Diagram

.csdlc/prepared/issues/724/diagram.mmd

Digest: 0de98c5c80c32970c592ce2b55e761fb9b1a148e25027858ce833f7ff46bb3ee

## Stop Conditions

- #721 semantics are unavailable or change incompatibly
- The implementation requires raw gh or bypassing typed dispatch
- Authority binding cannot be resolved exactly
- Focused proof cannot exercise the shared dispatch

## Handoff

Proceed only after doctor readiness.
