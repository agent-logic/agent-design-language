# Structured Planning Prompt

Template: 1.0.0

Issue: 5906

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add mergedAt evidence, implement unique-latest precedence, run focused tests and strict Clippy, obtain exact-head review, publish, merge when green, then reconcile 5818 and 5861.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add merged-at candidate evidence and unique-latest validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused tests and strict Clippy",
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
    "action": "Resolve exact-head review and publish",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Reconcile issues 5818 and 5861",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No tracked work on main
- No invented historical evidence
- Exact repository, issue, PR, head, and merge SHA remain required

## Risks

- GitHub closing references omit mergedAt
- Timestamp tie prevents deterministic precedence

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5906/design.md

Digest: aab00dd908f0fec461c2d95d9585b62a7ea8d8c1027796fbac2d2f7bb43308e7

## Diagram

.csdlc/prepared/issues/5906/diagram.mmd

Digest: 2013e400e7e2f7f652ea382806ebcf1976ca8e19bbc34d9ff5ae8f88ed2f6cc7

## Stop Conditions

- Missing or tied latest mergedAt evidence
- Exact requested identity does not match latest candidate
- Focused regression or review failure

## Handoff

Proceed only after doctor readiness.
