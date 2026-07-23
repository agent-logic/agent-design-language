# Structured Planning Prompt

Template: 1.0.0

Issue: 5332

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Extract the ILPP classifier, prove it with deterministic fixtures, reproduce through #4741 safe staging, run a one-variable diagnostic matrix, apply only the isolated root-cause fix, and prove normal batch behavior.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Extract a focused progress-aware ILPP signature classifier and deterministic fixture harness",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-8",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Use #4741 safe staging to reproduce the loop and retain exact non-secret baseline evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run the smallest one-variable environment matrix needed to isolate wrapper, mutable state, host/domain identity, or Unity version",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Apply the minimal isolated root-cause fix or record the exact irreducible blocker and owner",
    "acceptance_ids": [
      "AC-7",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused fixtures and one safe normal-start regression, then record bounded review and WP-15 outcome truth",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Only complete repeating ILPP signatures classify the loop
- Semantic progress resets non-progress state
- One diagnostic variable changes per matrix cell
- The root-cause fix follows evidence rather than speculation
- Repository binaries and approved staging remain authoritative

## Risks

- Repeating ILPP logs can appear live while no semantic progress occurs
- Changing multiple environment variables at once could produce a false root-cause attribution
- A Unity version comparison can introduce unrelated import differences
- The general wrapper overlaps #4741 and must remain narrowly partitioned
- Host and domain evidence can leak machine-specific detail unless bounded

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5332/design.md

Digest: 236902bff7f84f7613a2ea354724c3834f6dec993e8a029b8b8b5c078d633c6e

## Diagram

.csdlc/prepared/issues/5332/diagram.mmd

Digest: d2774d7d3cc655ca1b4e0e1af44fa388b839152f89bff99978d985024c10cb28

## Stop Conditions

- #4741 cannot provide a safe staged-project mode
- The loop cannot be reproduced from exact retained evidence or an approved staged run
- Diagnosis requires broad host inspection or secret-bearing environment output
- More than one matrix variable must change in a cell
- The isolated fix belongs to #4739, #4741, Unity scene code, or another owner

## Handoff

Proceed only after doctor readiness.
