# Structured Planning Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inventory every workflow, make ci.yaml the sole automatic PR dispatcher, gate required heavy work onto the 16-core runner, isolate optional proof and soaks behind explicit dispatch, coalesce duplicate heads, prove routing locally, review once, and publish once.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Inventory all CI, coverage, proof, soak, demo, and provider workflow triggers and classify required versus explicit lanes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Centralize PR dispatch, add head-SHA concurrency, isolate optional workflows, and preserve required 16-core routing.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add deterministic whole-workflow and representative path-routing contracts plus concise operating procedure.",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused local proof, resolve bounded review findings, and publish one reviewed revision without optional hosted cycles.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Required validation is not weakened
- Required heavy work uses the configured 16-core runner
- Optional work cannot allocate a runner automatically
- One repository revision executes at most one required CI fleet
- Skipped lanes are explained without dispatching them

## Risks

- Removing standalone PR triggers could hide a formerly required proof unless central classification is complete
- Concurrency keyed incorrectly could cancel distinct revisions
- Broad fallback routing could recreate optional fanout
- Coverage selection could accidentally include long soaks
- Publishing parallel PRs for one branch could duplicate events before concurrency

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/234/design.md

Digest: e874e5f9268540952661f4e8cdf524146510d0c12ef75de26a6de2af426edf6e

## Diagram

.csdlc/prepared/issues/234/diagram.mmd

Digest: 3397f34d9afd2e1a6d57c8dadb380a64a5e242f52a0b9f577338060507d61f98

## Stop Conditions

- A required branch-protection check depends on a standalone workflow that cannot be represented in central CI
- The 16-core selector is unavailable or changed without operator approval
- A proposed fix requires GitHub organization or billing mutation
- Local deterministic contracts cannot distinguish required from optional lanes
- Review finds unresolved loss of required validation

## Handoff

Proceed only after doctor readiness.
