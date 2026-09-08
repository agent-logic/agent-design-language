# Structured Planning Prompt

Template: 1.0.0

Issue: 526

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze and verify the candidate and ancestral tail census, finalize truthful notes, obtain exact operator authorization, perform the typed ceremony route, and retain live tag/release readback.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the candidate and verify every prior tail reviewed merge is ancestral.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Finalize release notes and obtain explicit operator authorization naming the exact candidate, tag, and release operation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Perform the authorized typed release operation and retain exact live readback receipt.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Release identity is exact and immutable
- Operator authorization precedes mutation
- All included work is reviewed and ancestral
- Closeout bookkeeping is asynchronous

## Risks

- Candidate advancement after approval
- Tag or release points to the wrong commit
- Notes overstate delivered behavior
- A missing prior merge is hidden by green current checks

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/526/design.md

Digest: 1ccc9b1dc06e67360fa0d89e2f50a2f21e8b0ba0f89cffe8cdda9bc8c73ab51e

## Diagram

.csdlc/prepared/issues/526/diagram.mmd

Digest: 44ab7e99cebcdbdeaacd2b7dda26885775d5b035f6065c84491f7c1b84d80cfe

## Stop Conditions

- #525 lacks a reviewed merge
- Operator authorization is absent or ambiguous
- Any prior tail issue lacks a reviewed-green ancestral merge
- Candidate, tag, notes, or release identity drift
- Typed release route cannot produce exact readback

## Handoff

Proceed only after doctor readiness.
