# Structured Planning Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement a focused read-only Rust preflight on the existing csdlc-github and Octocrab surfaces; prove complete API pagination, policy/capacity/dispatch classification, exact canary identity, stale-reference uncertainty, CLI redaction, and live larger-runner dispatch; then review the exact head and publish without changing workflow routing.

## Plan

Revision 7

## Steps

[
  {
    "id": "S1",
    "action": "Define the typed runner policy, capacity, dispatch, pagination, and canary-identity contract.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement read-only Octocrab runner and group inspection with fail-closed stale-ref diagnostics.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Prove classifiers, schemas, API pagination, CLI redaction, and exact live larger-runner dispatch.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve exact-head review, publish the reviewed change, and retain terminal canary evidence.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
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

Digest: 9de2ed1bc225fb8e8e6fac8edd938f655ff77987dd574b9043a37a17463caf84

## Diagram

.csdlc/prepared/issues/32/diagram.mmd

Digest: 706a56f11a18e0b180a4bd40e23b9ae5a9adc18d28254dbfadc4d6f01acd22d4

## Stop Conditions

- Repository selection cannot be verified
- GitHub authorization prevents fail-closed classification
- The fix would require broadening repository access or using AWS

## Handoff

Proceed only after doctor readiness.
