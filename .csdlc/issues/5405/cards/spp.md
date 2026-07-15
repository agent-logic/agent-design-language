# Structured Planning Prompt

Template: 1.0.0

Issue: 5405

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inspect WP-13 claim records, correct overclaims, add duplicate semantic-policy validation, then run focused scheduler and doc validation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map #5403 WP-13 findings to records and scheduler validation",
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
    "action": "Repair docs/records and duplicate semantic-policy validation",
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
    "action": "Run focused regression proof and update retained evidence",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- WP-13 claim status remains evidence-bound
- Admission readiness is not equivalent to live provider invocation
- Duplicate semantic policy entries fail validation

## Risks

- Closeout records may need multiple claim-status updates
- Scheduler economics duplicate semantics may require preserving legacy fixtures

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5405/design.md

Digest: 41fa4be23fead9c7a1305031eab718184dd27dbce1b95b8b265e135d9163c092

## Diagram

.csdlc/prepared/issues/5405/diagram.mmd

Digest: d8ea209969c64b4eab1849ade9c605b6295f8f6b6c06445fd2b52ed1bef5ae56

## Stop Conditions

- Real guild integration is required but outside the approved issue scope
- Economics duplicate rejection breaks existing valid scheduler fixtures
- Parent closeout truth needs operator decision rather than mechanical repair

## Handoff

Proceed only after doctor readiness.
