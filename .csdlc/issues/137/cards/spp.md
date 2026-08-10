# Structured Planning Prompt

Template: 1.0.0

Issue: 137

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Adapt the already-reviewed #5878 workflow bytes with a required exact-SHA manual input, validate its static contract and path-policy behavior, obtain exact-head review, then publish and merge immediately after hosted CI is green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Register the pinned three-platform producer and Ubuntu aggregation workflow with exact-SHA dispatch.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused workflow contract, path-policy, and diff-hygiene validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Obtain independent exact-head review, publish with closing linkage, shepherd hosted CI, and merge.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Only exact commit revisions are checked out
- Every platform writes into the same governed receipt directory but uploads a unique artifact name
- Aggregation depends on every platform producer
- Missing files or validator failures fail the workflow
- Repository permissions remain contents read only
- No #5878-owned source or evidence file is modified

## Risks

- Manual dispatch could accept a branch or tag instead of an immutable commit
- Artifact names could collide or aggregation could proceed with a missing platform
- Windows Bash path behavior could diverge from Linux and macOS
- Unpinned actions could weaken reproducibility

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/137/design.md

Digest: 9b186b59424e873be1c371c9c336dcfaf6c94b7c3ab8272a004eee3ec6b1c65f

## Diagram

.csdlc/prepared/issues/137/diagram.mmd

Digest: 93f505a975c21eeb52199e301b4ab9cb3f4ace91a1bf167c559b8926285acc89

## Stop Conditions

- The workflow requires modifying an existing #5878-owned producer or validator
- Exact commit checkout cannot be enforced for manual dispatch
- A platform or aggregate receipt can be skipped while the workflow remains green
- Tracked edits appear outside the workflow or issue #137 lifecycle/proof paths
- Exact-head review or hosted CI reports an actionable failure

## Handoff

Proceed only after doctor readiness.
