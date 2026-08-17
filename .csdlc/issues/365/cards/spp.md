# Structured Planning Prompt

Template: 1.0.0

Issue: 365

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate and review the exact opaque-provenance design, bind only after approval/doctor, implement four-path store-derived seams, prove authenticity/restart/corruption/redaction/no-policy drift, then review publish finish and release #275.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Validate exact terminal ancestry and obtain fresh design review approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind and implement private store-derived opaque committed projections in the exact four paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused matrices strict Clippy diff review CI finish cache and ancestry before #275.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Only owning durable store constructs sealed value
- Every exposed field is redacted and cross-bound
- Restart reproduces exact bytes
- Corruption or substitution returns no sealed value
- Existing eligibility policy is unchanged

## Risks

- Opaque type could accidentally gain a public constructor
- Provenance could bind formatted digests without recomputing store truth
- A/B components could be combined after construction
- Test seam could weaken production boundary

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/365/design.md

Digest: 70b8dc394f65d1941b050383f5159e636b1b35a51e15bf13e83975995494f4ba

## Diagram

.csdlc/prepared/issues/365/diagram.mmd

Digest: f324d21b5cbf854485758c05014b3d4a99488b4ac38d2a389af457943cd584c7

## Stop Conditions

- Any required edit outside exact four product paths
- Any caller constructor conversion or raw authority exposure
- Any eligibility policy or transition behavior change
- Any noncanonical dependency scope drift zero-test review finding CI failure or terminal mismatch

## Handoff

Proceed only after doctor readiness.
