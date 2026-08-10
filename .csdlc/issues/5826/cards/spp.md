# Structured Planning Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-09 canonical identity records with stable labels, immutable root authority, provenance, and redaction-safe negative handling.

## Plan

Revision 22

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5825 terminal proof and inspect adl-runtime-kernel identity_memory.rs and private_state.rs before claiming the exact birthday_identity.rs, lib.rs, tests, fixture, feature, and evidence paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the versioned identity record, deterministic derivation, canonical serialization, and valid/negative fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused replay, alias/provenance negatives, privacy-redaction, and path-portability lanes and retain exact-revision proof.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5826 linkage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Stable name never substitutes for identity root authority.
- Canonical identity derivation and serialization are deterministic.
- Raw private state is unnecessary for review and cannot enter retained projections.

## Risks

- Alias updates could silently replace root identity.
- Continuity references could be accepted without binding prior evidence.
- Fixtures or reports could leak private or host-specific paths.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5826/design.md

Digest: 9c143b55f2e75010e7f934806c64fb50b3b46fc7456253871c91947bcbfb71d2

## Diagram

.csdlc/prepared/issues/5826/diagram.mmd

Digest: 0c8418cba179260289efc43a6ec61eb785f3973ba701053a09c7066f5e59adfd

## Stop Conditions

- #5825 lacks terminal receipt-backed proof.
- Exact identity paths collide with another live claim.
- Identity requires raw private state or an unversioned shared schema change.

## Handoff

Proceed only after doctor readiness.
