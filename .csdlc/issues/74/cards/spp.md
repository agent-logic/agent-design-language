# Structured Planning Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Run the exact current-main canary first, retain a focused regression, and touch production only if the relevance-first scan still fails.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reproduce real csdlc-bind with a stale same-issue projection whose retired claim field is present but whose branch and worktree ownership fields are absent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Skip strict decoding only for projections with no branch or worktree topology authority while preserving strict verification for live same-issue, branch, and worktree collisions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run the focused Gate 2 real-binary regression and strict clippy validation from the issue worktree.",
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
    "id": "S4",
    "action": "Obtain bounded exact-head review, publish the repair PR, and shepherd required checks without merging.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Canonical IssueRecord remains claim-free and strict
- Unrelated records are not mutated
- Real collisions fail closed
- No broad validation

## Risks

- Mistaking existing #61 behavior for exact claim-field proof
- Accidentally tolerating malformed relevant records
- Expanding into historical data migration

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/74/design.md

Digest: 8db4b8fe501d7ec47cdcac6df0453e3d21c508e2e2f575dc5a68552d10ab9206

## Diagram

.csdlc/prepared/issues/74/diagram.mmd

Digest: 4a09e26fc26523ef5dbc6039d707d2175433db6efaa2d25aca1edb21ae107037

## Stop Conditions

- Fix would require restoring claims
- Historical records would need mutation
- Collision safety would be weakened

## Handoff

Proceed only after doctor readiness.
