# Structured Planning Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the required validated Polis init contract, project it through the Runtime feed, atomically hot-load every Polis parameter without restart, update HTML rendering, and retain focused review evidence.

## Plan

Revision 9

## Steps

[
  {
    "id": "S1",
    "action": "Implement and test the validated Runtime Polis identity configuration contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Project the redacted identity through the Observatory feed and production startup/reload path.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Replace HTML deployment constants with feed-owned rendering and focused tests.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
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
    "status": "in_progress"
  }
]

## Invariants

- Every Polis parameter may hot-load without restarting the Runtime
- A reload publishes one complete validated snapshot or no change
- Invalid reloads preserve the complete last-known-good snapshot
- Identity projection and diagnostics remain redacted
- Unity remains deferred

## Risks

- Feed schema compatibility drift
- A partial reload exposes mixed old and new Polis values
- Runtime and HTML consumers do not converge on the newly loaded snapshot
- HTML retains a hidden deployment fallback

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/551/design.md

Digest: e502e926314c0472b3d8df68fa358bca8de7dc30cd72617e6278e14d69de0710

## Diagram

.csdlc/prepared/issues/551/diagram.mmd

Digest: 87391f624101b24aa3024a1756fa2e7ba2e9fff3ca293424e36b2eb717a4d31e

## Stop Conditions

- Continuity state would be mutated
- A reload requires Runtime restart or exposes mixed Polis values
- Unity or issue #84 paths enter the diff
- External infrastructure mutation is required
- Validation selects zero tests

## Handoff

Proceed only after doctor readiness.
