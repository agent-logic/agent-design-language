# Structured Planning Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Build a small read-only Rust preflight on existing COTS GitHub surfaces, wire a fast workflow gate, prove policy/capacity classification, review exact head, and publish a live canary PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Define the typed runner policy and capacity contract.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement Octocrab-backed runner and group inspection with stale-ref diagnostics.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused deterministic tests and CI contract routing.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Review exact head, publish, and retain the live runner-assignment canary.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Repository scope retained
- Branch-independent workflow eligibility
- Capacity-policy-dispatch distinction
- Ready is not dispatch proof
- Read-only diagnostics
- No credential output
- No AWS

## Risks

- GitHub API response shape may drift
- Token permissions may make state indeterminate
- A Ready runner may have transient capacity delay

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/32/design.md

Digest: 1ce39cb5d02984ca57d3191178834e6a107774a8ca6513d581e6c060bc4456de

## Diagram

.csdlc/prepared/issues/32/diagram.mmd

Digest: 706a56f11a18e0b180a4bd40e23b9ae5a9adc18d28254dbfadc4d6f01acd22d4

## Stop Conditions

- Repository selection cannot be verified
- GitHub authorization prevents fail-closed classification
- The fix would require broadening repository access or using AWS

## Handoff

Proceed only after doctor readiness.
