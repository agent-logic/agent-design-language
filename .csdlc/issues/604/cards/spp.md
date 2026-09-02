# Structured Planning Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and bind #604, restore typed csdlc-publish ready and reconcile-ready commands, prove exact remote identity/readback and recovery behavior, update skill/inventory docs, run focused validation and review, then publish a PR with Closes #604.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #604 in the FastWork issue worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement typed ready and reconcile-ready publication command surfaces.",
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
    "action": "Add focused tests for success, uncertain response recovery, stale CAS, identity drift, pre-state rejection, and zero-write failures.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Update publication skill and operator inventory guidance.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused validation, independent review, and publish the PR with Closes #604.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Publication-ready mutation requires exact live PR identity before and after mutation.
- Lifecycle truth is not advanced on stale generation, stale digest, identity drift, or uncertain remote state.
- Reconciliation observes remote state; it does not trust caller claims.
- Raw GitHub mutation remains prohibited for covered lifecycle writes.

## Risks

- GitHub ready-for-review transport can return uncertain responses.
- Existing publication state may encode draft/non-draft status inconsistently.
- Tests can become synthetic if they do not distinguish pre-state and post-state readback.
- Skill text can overclaim if the binary command surface is not aligned.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/604/design.md

Digest: b8848e01d7c9010be517c43cedc50c002d795dfcf4f48ebab7d4648c7615459b

## Diagram

.csdlc/prepared/issues/604/diagram.mmd

Digest: 40cef7b693acc39fcacfc3a6c8bb3edbf35ea1535b9cc3663dca2424e4c52d5e

## Stop Conditions

- The issue branch or worktree identity drifts.
- Typed GitHub transport cannot represent ready-for-review without raw gh.
- Ready truth would be recorded before exact post-mutation readback.
- Fresh review finds unresolved actionable issues.
- Required checks are red.

## Handoff

Proceed only after doctor readiness.
