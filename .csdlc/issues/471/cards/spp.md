# Structured Planning Prompt

Template: 1.0.0

Issue: 471

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement five coherent kernel slices: authoritative contract wiring, determinism enforcement, layered lifecycle, resilient supervision, and truthful health/metrics; prove each with focused negative and integration tests.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Version contracts and add kernel-owned typed port registry with explicit backpressure.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Enforce determinism and lifecycle declarations at component boundaries.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement layered startup and staged reverse-dependency shutdown.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement restart windows, readiness policy, supervision scopes, and degradation propagation.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Add aggregate health and poison-free metrics, then run full proof and review.",
    "acceptance_ids": [
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  }
]

## Invariants

- Invalid assemblies fail before component spawn
- All lifecycle waits and restart loops are bounded
- Every degraded or terminal component state is observable
- No telemetry failure poisons the data path
- Valid public Runtime behavior remains compatible

## Risks

- Contract migration may touch many assembly fixtures
- Concurrent startup may expose hidden ordering dependencies
- Supervision changes may create restart or shutdown races
- Health projection may drift from actual task state

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/471/design.md

Digest: e76587e9ef3715f10a06f01864232c1aef20bcdce560de0b43886a5d0963f307

## Diagram

.csdlc/prepared/issues/471/diagram.mmd

Digest: 89bfa76f743d9eba8f8d1b99f14edbe0b05aa53185427cbe7b4c72a9f890b8aa

## Stop Conditions

- A change requires WP-27 or cloud scope
- A valid existing assembly cannot be migrated explicitly
- A lifecycle path becomes unbounded
- Any focused or regression test remains failing
- Exact-head review finds an actionable defect

## Handoff

Proceed only after doctor readiness.
