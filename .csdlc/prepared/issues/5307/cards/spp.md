# Structured Planning Prompt

Template: 1.0.0

Issue: 5307

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate time, approval, migration, active contracts, and v2 health; remove only importer; prove and review.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Evaluate time, approval, migration completeness, active contracts, and v2 health",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Remove only the exact importer surface while preserving durable evidence",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run exact-revision review and required proof",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Zero mutation before the date
- Migration evidence remains durable
- No active contract loses a required path

## Risks

- Hidden importer consumer
- Incomplete migration evidence
- Clock or approval ambiguity

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs/architecture/csdlc-v2/gate10d4/DESIGN.md

Digest: f1d43b37ae3eadac6ab7dce6354c8caaaa83e557314e46a7608c66f4d0bc93bd

## Diagram

docs/architecture/csdlc-v2/gate10d4/DIAGRAM.mmd

Digest: 1e7e318b78a895c77cd997d86c84f67d9bf3ebe6e03b55e63e8f1c34014d8caa

## Stop Conditions

- Current time is early or untrusted
- Approval is absent
- Migration is incomplete
- An active contract names importer
- v2 proof is red

## Handoff

Proceed only after doctor readiness.
