# Structured Planning Prompt

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Approve the narrow owner design, bind, implement marker-aware title-only update and exact readback, validate, obtain fresh review, and stop before merge.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Finalize the title-only provenance contract: durable operation-key-to-request fingerprint receipts, same-key idempotent retry, conflicting key reuse fail-closed, and compatibility with legacy body-bearing markers.",
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
    "action": "Implement only csdlc-v2/src/github.rs plus focused gate_github_actions coverage for body preservation, provenance readback, retry, conflicting reuse, partial-operation recovery, and deterministic concurrent body drift boundaries.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused validation and strict Clippy, obtain fresh exact-head review with no unresolved findings, publish ready, and stop before merge.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No existing issue body bytes are removed
- No duplicate marker is appended
- Title equality alone is insufficient reconciliation proof

## Risks

- Unnecessary body update may affect formatting
- Retry logic may duplicate markers
- Readback may overclaim reconciliation

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/301/design.md

Digest: 131c08ddd36d3441808326298bd5b342cf347da248c85000c38e1e28c5a7c254

## Diagram

.csdlc/prepared/issues/301/diagram.mmd

Digest: e8858a02167e85c90e9486d328827bc4a7fc697684efbe785d2e10600a4f5f41

## Stop Conditions

- Any store/card/recovery/coverage path would change
- Typed lifecycle reports collision
- Focused proof or exact-head review fails

## Handoff

Proceed only after doctor readiness.
