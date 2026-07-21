# Structured Planning Prompt

Template: 1.0.0

Issue: 5361

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and review the Runtime v3 acceptance contract, wait for architecture and parity dependencies, then synthesize exact-revision operational, consumer, rollback, and workcell proof for #5384.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Prepare and review all six acceptance cards, dependency graph, protected paths, and validation lanes",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Consume reviewed Parity-A through Parity-D and Runtime v3 consumer evidence at exact revisions",
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
    "action": "Run guardian, secure access, Observatory, telemetry, pressure, rollback, and recovery acceptance",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Synthesize live workcell and consumer proof, review the exact revision, and publish truthful acceptance or blocker state",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked work on main
- no acceptance execution before dependencies integrate
- no Runtime v2 implementation files
- no hard-coded IP addresses
- HTTPS only for network access
- no AWS
- one serialized integration queue

## Risks

- fixture-only parity could be mistaken for integrated behavior
- parallel parity children may collide on shared Runtime kernel paths
- Runtime v2 imports could silently defeat cutover independence
- network configuration could regress to loopback-only or hard-coded addresses
- unsupported remote or GPU evidence could be overstated

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5361/design.md

Digest: 3dea593c006065eb2ddc44090d2b02ee2342dacaa1c12b2f39be905f8bcbc70a

## Diagram

.csdlc/prepared/issues/5361/diagram.mmd

Digest: 6042adc38f598dcd2021d4d976966cba0d5bd598546bd9e1bfc2ac38b061ca75

## Stop Conditions

- #5336 architecture authority is not integrated
- any required parity or consumer dependency remains open or unreviewed
- protected paths overlap without explicit serialization
- acceptance requires Runtime v2 implementation reuse
- a required proof can only be represented as an unsupported claim

## Handoff

Proceed only after doctor readiness.
