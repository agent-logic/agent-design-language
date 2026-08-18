# Structured Planning Prompt

Template: 1.0.0

Issue: 367

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate and review the exact same-lineage design, bind only after approval and doctor, implement four-path redacted lineage binding and opaque verification, prove authentic A/A and A/B restart behavior, then review publish finish and release #275.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Validate exact terminal ancestry and obtain fresh design review approval for verifier-derived lineage and the opaque pair adapter.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "In the bound exact four paths, implement Shepherd lineage binding and verifier-only construction of a borrowed VerifiedCommittedChildLineagePair over the exact sealed children.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused matrices strict Clippy diff review CI finish cache and ancestry before #275 consumes the opaque adapter.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- Only a verified cut derives the redacted lineage reference
- Only store-owned sealed values enter pair verification
- Only the verifier constructs the borrowed pair adapter over the exact two sealed inputs
- The pair adapter exposes no raw lineage boolean constructor deserializer or mutable state
- Authentic different-lineage stores fail on first use and restart
- Legacy missing lineage is never synthesized
- Existing eligibility policy is unchanged

## Risks

- A raw lineage value could accidentally become integration authority
- Shepherd normalized state could omit lineage
- Two authentic stores could bypass pairing on initial commit
- Legacy compatibility could synthesize false provenance

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/367/design.md

Digest: ddca0a5ce38a8dfed01754222b7b1e790d8c3faba374a6db1e0e6ad3b2475d4b

## Diagram

.csdlc/prepared/issues/367/diagram.mmd

Digest: 43f7f1c44c3d8e8fd8969ffbab8421b8721f65fc61fbca84bd0c58398709c463

## Stop Conditions

- Any required edit outside exact four product paths
- Any new raw lineage or caller pairing authority
- Any eligibility policy change or synthetic legacy migration
- Any noncanonical dependency scope drift zero-test review finding CI failure or terminal mismatch

## Handoff

Proceed only after doctor readiness.
