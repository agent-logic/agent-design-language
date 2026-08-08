# Structured Planning Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Model the two repository authorities explicitly, teach doctor a three-way fail-closed decision, add focused fixtures for all required cases, update active guidance, validate, and obtain exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Trace the current doctor, record, publication, and Git-remote identity flow and select the smallest typed split-authority representation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement same-repository and explicit split-repository validation with specific fail-closed drift diagnostics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add deterministic fixtures and tests for same repository, valid split authority, and invalid drift.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Update every active skill and runbook affected by repository identity, then run focused validation and exact-head review.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue and code repository identities never collapse implicitly
- Explicit consistent split authority is required when identities differ
- Effective Git topology cannot substitute for issue authority
- Same-repository compatibility remains deterministic and fail-closed
- No claims, leases, v1 wrappers, or migration assumptions return

## Risks

- A permissive fallback accidentally accepts arbitrary origin drift
- Identity normalization treats distinct repositories as equal
- The record schema duplicates publication identity inconsistently
- Skills or runbooks continue teaching the obsolete single-repository assumption

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/45/design.md

Digest: 3ba9b8f651c2fba7b1e2248b446c6ac9596ec381118a45a0f4be0cc726d765a2

## Diagram

.csdlc/prepared/issues/45/diagram.mmd

Digest: ad3e20a829f8fb2c9010355be03c7bd14246833c7b3b9e0af0bcf257febd73d2

## Stop Conditions

- The implementation requires guessing either repository identity
- Same-repository compatibility would be weakened
- A split route cannot be represented without ambiguity
- Scope expands into repository migration or unrelated lifecycle repair

## Handoff

Proceed only after doctor readiness.
