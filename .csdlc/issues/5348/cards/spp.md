# Structured Planning Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5359 is live-merged and ancestral.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify WP-22 merge ancestry, remaining v0.91.8 open issues, and pre-ceremony tag/release absence.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Finalize release documentation, ceremony packet, and #5809 supplemental evidence without product changes.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused docs/evidence validation and release-script check-only preflight.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain one exact-head bounded review, fix findings, and publish a PR closing #5348 and #5809.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "After merge, execute and verify the repository release script, then close sprint umbrella #5595 with exact release identity.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
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

Digest: ab8d9286241ed70d72e00f4a3132052b2738174b61fa7db2cea7557aeb6b7f97

## Diagram

.csdlc/prepared/issues/5348/diagram.mmd

Digest: 762a615dcdae2affac2d7c0e5bf51ede96c495ae463c326d133d59847cfb9d0a

## Stop Conditions

- #5359 not live-merged
- #5359 merge not ancestral
- release evidence incomplete
- ceremony would require repair work

## Handoff

Proceed only after doctor readiness.
