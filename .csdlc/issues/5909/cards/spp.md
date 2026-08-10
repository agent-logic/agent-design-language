# Structured Planning Prompt

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bind the corrective issue, apply the already-proven bounded product diff to merged main, regenerate issue-owned exact proof, obtain fresh independent exact-head review, and publish a green ready PR without merging.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind current merged main and apply only the two-path corrective product diff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Generate exact machine-derived proof and run focused validation.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Resolve fresh independent exact-head review and publish a green ready PR.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Mutation authority comes from ledger-owned committed state, never caller assertions
- All capacity failures are fail-closed and atomic
- Proof is machine-derived and exact-revision digest-bound
- Product scope remains exactly two files

## Risks

- A stale branch could omit the externally merged base
- Capacity validation could partially mutate state
- Evidence could drift from executed case names or source revision

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5869/design.md

Digest: 132f5437e67cb71961df3c6cb1b88fed79e68d88ac39b44d8f421f630d468125

## Diagram

.csdlc/prepared/issues/5869/diagram.mmd

Digest: b6214f3f6d8281d9eabcf54b42092e69517d8f3cbb616f27313f0b7265a97d07

## Stop Conditions

- Current base does not contain merge 081988dfe4632e27062f3acc72b7c5d226cd0802
- Any product path outside the two declared files is required
- Focused test selects zero tests
- Fresh review finds unresolved actionable issues

## Handoff

Proceed only after doctor readiness.
