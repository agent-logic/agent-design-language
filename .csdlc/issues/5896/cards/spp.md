# Structured Planning Prompt

Template: 1.0.0

Issue: 5896

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Define typed migration contracts, classify the full cohort before writes, atomically reset only unambiguous open never-bound records, prove idempotence and negative cases, then validate the real cohort and WP-24 bindability.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement typed request, disposition, report, and dry-run classification for the exact legacy shape.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply atomic record migration only after full classification succeeds, preserving cards and audit truth.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused tests for all positive, negative, and idempotent cases.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the real cohort migration, retain dispositions, and prove current doctor plus WP-24 bindability.",
    "acceptance_ids": [
      "AC-1",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Git branch/worktree topology remains the sole binding authority
- No record changes before complete classification succeeds
- No branch or worktree value is invented
- Terminal truth and retained evidence are immutable
- A second migration pass makes no changes

## Risks

- A historical branch name may resemble issue topology without being authoritative
- A partial migration could strand records unless classification is transaction-wide
- Closed records may require preservation rather than phase reset

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/5896/design.md

Digest: 3630249209061b26e687230fc580c0eceb71026ecc1a6daff839970b83d01d12

## Diagram

.csdlc/prepared/issues/5896/diagram.mmd

Digest: aebaba3073eadae57755ce3ce6f0200a24c6f552d4730d1d555bcaf3ec7201b8

## Stop Conditions

- Any record or card digest fails verification
- Live topology is ambiguous or only partially matches
- Live issue state is missing for a cohort member
- The migration would alter cards, authored evidence, or terminal truth

## Handoff

Proceed only after doctor readiness.
