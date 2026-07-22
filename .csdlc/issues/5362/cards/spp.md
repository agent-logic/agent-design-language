# Structured Planning Prompt

Template: 1.0.0

Issue: 5362

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare now; execute only after #5363 is live-merged and ancestral.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify typed preparation packet and #5363 live merge plus ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Consume accepted preflight truth",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Align feature-list and v0.92 planning inputs",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Preserve blockers or release WP-21A without preparation-scope mutations",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
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

- feature-list rows may lack evidence-bound owners
- v0.92 inputs may overclaim birthday readiness
- preflight blockers may be stale

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5362/design.md

Digest: e36e285953ebe1532719cc11fcb614abc5be0a2ade5539eafa8ab0c10a6ebab2

## Diagram

.csdlc/prepared/issues/5362/diagram.mmd

Digest: 6557a5d066ef3c14765e3ed132d6a1ba7cf98c4fea2cfb1a7e8813943720abe8

## Stop Conditions

- #5363 not live-merged
- #5363 merge not ancestral
- feature disposition lacks evidence
- v0.92 wording would overclaim

## Handoff

Proceed only after doctor readiness.
