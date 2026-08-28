# Structured Planning Prompt

Template: 1.0.0

Issue: 537

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize the five-child batch, validate design-time cards, then hand each child to a separate issue session only after its exact predecessor gates pass and retain integrated closeout truth.

## Plan

Revision 2

## Steps

[
  {
    "id": "readiness",
    "action": "Initialize and validate the Sprint Execution Packet and all child design-time card bundles",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "coordinate",
    "action": "Route child sessions sequentially according to declared dependency gates",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "review-close",
    "action": "Review integrated outcomes and close the umbrella only after terminal child truth",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Child issues retain all implementation and proof authority
- No child starts before declared dependencies
- No umbrella closeout substitutes for child closeout
- Release-tail evidence remains exact-revision, redacted, and fail-closed
- Preparation performs no publication or release mutation

## Risks

- #515 could begin before terminal #514
- #516 could admit an incomplete or non-ancestral milestone root
- Quality or documentation checks could accept skipped or stale evidence
- Publication-candidate preparation could be confused with merge, tag, or release authority
- The umbrella could overstate completion while a child is nonterminal

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/537/design.md

Digest: b68c06cfc5bcae5f0c82ee7083266943b05d3261d7b59731597af35af082ebe6

## Diagram

.csdlc/prepared/issues/537/diagram.mmd

Digest: b1953fb23e32752d0e599d0d26bb54862289061c94aaa49110ef02a4e74568ee

## Stop Conditions

- Any missing or generic child design-time card bundle
- Any required predecessor or milestone root is nonterminal or ambiguous
- Any evidence denominator is incomplete, stale, skipped, or non-ancestral
- Any request to merge, tag, release, or publish during preparation
- Any attempt to use the umbrella to implement child work

## Handoff

Proceed only after doctor readiness.
