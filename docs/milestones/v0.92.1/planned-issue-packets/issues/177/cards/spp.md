# Structured Planning Prompt

Template: 1.0.0

Issue: 177

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement idempotent intent-first PR mutation and bounded foreground watch with explicit Closing/PartOf linkage, exact readback, fixed deadlines, cancellation, no persistent jobs, and merge only under explicit policy plus operator authority.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define typed PR mutation, linkage selection, watch bounds, idempotency keys, durable intent, and mode-bound publication evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Commit and sync the remote intent before each mutation, execute through the GitHub adapter, and reconcile exact qualified readback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Enforce Closing and PartOf relation grammar, same-repository normalization, split-repository qualification, and open-parent readback for checkpoints.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement foreground watch with 30-minute default, 24-hour maximum, 15-second poll, fixed deadline, retry-after clipping, stderr progress, and no persistence.",
    "acceptance_ids": [
      "AC-5",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Integrate root cancellation so every sleep/network await exits, drains, and joins before 130; gate merge on both policy and operator authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-7",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run idempotency, ambiguous readback, linkage, stale/closed/missing parent, deadline, retry-after, cancellation, and bounded live canary fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Reconcile every durable mutation intent, cancel and join foreground watchers, remove no remote resource speculatively, and prove no persistent job or unjoined task remains.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-14 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/177/design.md

Digest: 023624dea22d5b1408addd507964386c089c852666c57b232dc9deb81ac4ba47

## Diagram

.csdlc/prepared/issues/177/diagram.mmd

Digest: f090319c72f5e41f3b028ae26f409f2257eba23c100067fa6c5338fef954d4bd

## Stop Conditions

- Mutation lacks a resumable intent, linkage mode or target is not durable, readback can conflate `part_of` with closing, watch detaches, exact readback is unavailable, merge becomes implicit, or cancellation leaves work running.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
