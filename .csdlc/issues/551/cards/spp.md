# Structured Planning Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the required validated Polis init contract, project it through the Runtime feed, support display-name-only atomic reload, update HTML rendering, and retain focused review evidence.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Implement and test the validated Runtime Polis identity configuration contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Project the redacted identity through the Observatory feed and production startup/reload path.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Replace HTML deployment constants with feed-owned rendering and focused tests.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused proof and obtain independent exact-head review before publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Continuity identity is never derived from display name
- Only display name may hot reload
- Identity projection is redacted
- Unity remains deferred

## Risks

- Feed schema compatibility drift
- Reload accidentally admits restart-gated identity changes
- HTML retains a hidden deployment fallback

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/551/design.md

Digest: b2c91d643963cd508d4ab398ca6d170640fad16af9ac2210c69293d12eefcf74

## Diagram

.csdlc/prepared/issues/551/diagram.mmd

Digest: 83190a476d94924a172ff44fc87d5917f53751466c314f60ff6dbebfc2f8fce7

## Stop Conditions

- Continuity identity would change
- Unity paths enter the diff
- External infrastructure mutation is required
- Validation selects zero tests

## Handoff

Proceed only after doctor readiness.
