# Structured Planning Prompt

Template: 1.0.0

Issue: 146

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the source plans, define three independent execution lanes, author the complete milestone package and machine-readable issue graph, validate focused planning contracts, obtain bounded review, and publish without starting child implementation.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile v0.92.5, C-SDLC v3, active Runtime work, and legal ownership gaps into one v0.92.1 scope contract.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author the complete canonical v0.92.1 milestone and feature-document package.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Define the machine-readable issue wave, dependencies, sprint topology, proof matrix, and release gates.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation, bounded independent review, repair findings, and publish the setup PR.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- Independent lane execution
- No hidden legal completion claims
- Reviewed v3 architecture preserved
- Real distributed proof rather than synthetic success counts
- Evidence-bound release gates
- No tracked work on main

## Risks

- The combined milestone can become too large unless lanes retain independent sprint and release-gate ownership.
- Legal transfer scope may omit assets or rely on non-counsel language.
- C-SDLC v3 estimates may change materially after the construction spike.
- Runtime tests may overclaim distribution if they use in-process services or fabricated receipts.
- Existing v0.92 work may drift while the planning package is under review.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/146/design.md

Digest: 3ba7394e3178f4de033d1bdffc16a9df948026065fcd7881bee84cb91060429f

## Diagram

.csdlc/prepared/issues/146/diagram.mmd

Digest: fb565622bffab6bb219e9fe5c6b20767164aa2d5fd49056f25f7f1a36c6d0bf2

## Stop Conditions

- A source plan or active issue creates contradictory ownership that cannot be reconciled without operator judgment.
- The issue wave introduces a cross-lane implementation dependency not required by the product contract.
- The Runtime proof plan cannot distinguish real production paths from in-process or synthetic substitutes.
- The package implies legal sufficiency without counsel review.
- Tracked edits would occur on main.

## Handoff

Proceed only after doctor readiness.
