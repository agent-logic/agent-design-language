# Structured Planning Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the required validated Polis init contract, project it through the Runtime feed, atomically hot-load every Polis parameter without restart, update HTML rendering, and retain focused review evidence.

## Plan

Revision 10

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
    "status": "completed"
  }
]

## Invariants

- The advertised Observatory public origin is an exact member of the combined allowed-origin set
- REST and WSS default to v2 and explicitly negotiate v1 or v3 without inventing compatibility from constants
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

Digest: f6eeb4c1e7d54dd46a4e16725b9f392370196892a74e0c70a0b5930764eb356d

## Diagram

.csdlc/prepared/issues/551/diagram.mmd

Digest: 047aed23fe007d7e8670c3a585c6a376f038cfabf1616980469bc1dde0654450

## Stop Conditions

- Continuity state would be mutated
- A reload requires Runtime restart or exposes mixed Polis values
- Unity or issue #84 paths enter the diff
- External infrastructure mutation is required
- Validation selects zero tests

## Handoff

Proceed only after doctor readiness.
