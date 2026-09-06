# Structured Planning Prompt

Template: 1.0.0

Issue: 538

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile live and canonical membership, write the sequential execution packet, preflight all child prompt surfaces, and emit the first truthful child handoff.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile live #538 membership version 7 and the exact #516 through #526 dependency chain with canonical sprint planning.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Create the versioned sequential Sprint Execution Packet, watcher plan, issue-goal handoffs, activity log path, and closeout bar.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Mechanically preflight and route repairs for the eleven child prompt bundles without executing child work.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Recheck live prerequisite truth and emit the first-child handoff or exact blocker list.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Eleven child issues remain distinct
- Execution stays strictly sequential
- Preparation never satisfies a dependency
- Every live child has a watcher during execution
- Every implementation session creates its own issue-bound goal
- The umbrella never fabricates child completion

## Risks

- Canonical planning may still describe the superseded Sprint 9 through Sprint 11 split
- Generic or absent child prompt bundles could make execution ambiguous
- Open #516 prerequisites could be mistaken for readiness
- Stale preparation worktrees could be confused with current authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/538/design.md

Digest: 9723dc8ebdcc85fd322f896e34aaa383b8a83424f62687f279f8487c4f81b399

## Diagram

.csdlc/prepared/issues/538/diagram.mmd

Digest: 1840b9e98a56856f14afaf85a4939d426c8405560747e84dc38d306b8ecd8ae5

## Stop Conditions

- Live #538 membership differs from #516 through #526
- A child dependency differs from the strict sequential chain
- A child prompt surface cannot be repaired through its typed editor route
- An open #516 prerequisite is omitted from the handoff
- Concurrent work owns a surface required for repair

## Handoff

Proceed only after doctor readiness.
