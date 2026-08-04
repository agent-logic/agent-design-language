# Structured Planning Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5359 is live-merged and ancestral.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Keep #5348 bound to the existing FastWork preparation branch and recover the typed claim if stale.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Encode the #5359 live-merge plus exact-base ancestry gate in the cards, design, and diagram.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused preparation validation and retain the validation request/evidence in #5348 issue-local paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Leave future release ceremony execution blocked unless the live #5359 merge and ancestry checks pass at the exact execution base.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is the dependency gate
- receipts audit-only
- no preparation review churn
- no implementation in preparation

## Risks

- ceremony could hide implementation work
- release notes could overclaim
- GitHub, card, milestone, and handoff state may disagree

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5348/design.md

Digest: 72a882a9a1d63899b5a9acafcb3f87a21ef059e8e273a8bae33afd0813b8accd

## Diagram

.csdlc/prepared/issues/5348/diagram.mmd

Digest: cdbf16aaf9e251c3b06cec9c70c508ece932eee913f65f9ba46391139276861f

## Stop Conditions

- #5359 not live-merged
- #5359 merge not ancestral
- release evidence incomplete
- ceremony would require repair work

## Handoff

Proceed only after doctor readiness.
