# Structured Planning Prompt

Template: 1.0.0

Issue: 331

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #330 clears root staging, bootstrap and approve #331, bind a FastWork worktree, implement the narrow initialized code_repository recovery route, prove focused regressions/doctor behavior, obtain fresh exact-head review, publish, shepherd CI, and finish if gates are green.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap, design-review, doctor, and bind #331 after #330 root projection clearance.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the narrow initialized code_repository declaration/recovery route.",
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
    "action": "Run focused migration and doctor regressions plus strict local hygiene.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, publish, shepherd CI, and finish terminal if gates are green.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue repository authority and code repository authority remain explicitly distinct
- Initialized recovery keeps branch and worktree null
- Audit history is append-only
- Existing non-initialized migration behavior remains compatible

## Risks

- A too-broad operation could become an unsafe lifecycle identity rewrite
- Doctor/readiness may need exact handling for initialized records with explicit code repository
- Regression fixtures must avoid mutating live #5837/#5838 state

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/331/design.md

Digest: 55f1ca7bc01337f2b4f62e3e4b34891200d21c4b8b23c8585126bd1d17d106dd

## Diagram

.csdlc/prepared/issues/331/diagram.mmd

Digest: 25289d31b866a3f09cc50370502dcfc230789bab74c38e26448449efc81b9a32

## Stop Conditions

- #330 root projection has not cleared
- Owned path collision with another active tooling issue
- Need to widen into GitHub issue transfer or publication semantics
- Fresh review reports actionable finding
- Required hosted checks fail after publication

## Handoff

Proceed only after doctor readiness.
