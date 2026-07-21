# Structured Planning Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add --no-report to partitioned profile collection, lock the command contract, validate locally, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add no-report profile collection to every authoritative coverage partition",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Strengthen focused regression contracts and validate on /Volumes/FastWork",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Review, publish, and integrate the bounded repair",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Coverage thresholds are unchanged
- Test selectors and partition count are unchanged
- Explicit combined reports remain required
- No AWS route is used

## Risks

- A command path could omit no-report
- Explicit final reporting could accidentally be weakened
- A test could assert text without exercising failure semantics

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5602/design.md

Digest: f45a2baf128a937eb1fb7bfd0e1d39db33ac3c2efb243efc8b965c7ea0304f82

## Diagram

.csdlc/prepared/issues/5602/diagram.mmd

Digest: d7109fa116d0e69fa1da25577a5282e45586699370b8386eba414f20ae6c24d3

## Stop Conditions

- The repair requires product/runtime changes
- The repair requires lowering coverage or skipping tests
- Validation requires AWS

## Handoff

Proceed only after doctor readiness.
