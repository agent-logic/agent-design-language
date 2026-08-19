# Structured Planning Prompt

Template: 1.0.0

Issue: 327

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and approve the bounded design, bind #327, remove the unreachable helper, run focused CLI and strict-Clippy proof, obtain fresh exact-head review, publish, and finish only after required checks are green.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap, independently review, approve, doctor, and bind issue #327.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Remove only the unreachable real_tooling helper in the bound worktree.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused CLI tests and strict all-target Clippy through typed validation evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, publish a ready PR, observe required CI, and finish terminal if all gates permit.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Unsupported v1 tooling remains fail closed
- Supported CLI routes are unchanged
- Primary main remains tracked clean
- Only the bound #327 worktree receives source edits

## Risks

- A hidden caller could make removal behaviorally significant
- Hosted CI may expose unrelated baseline failures

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/327/design.md

Digest: 6e02af29cf146f3e06f54a0c50c81a2520efcbdd8e97fcc7cde145c840654fdf

## Diagram

.csdlc/prepared/issues/327/diagram.mmd

Digest: 0e7d3b087decd37f05dcd556179f88694b5eef157bef933d8f51c06f6ad167f9

## Stop Conditions

- Any live caller of real_tooling is found
- Owned path collision
- Primary main gains tracked changes
- Focused test or strict Clippy fails after the bounded correction
- Fresh review reports an actionable finding

## Handoff

Proceed only after doctor readiness.
