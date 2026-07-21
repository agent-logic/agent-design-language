# Structured Planning Prompt

Template: 1.0.0

Issue: 5499

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and review the pure conductor contract, hold product scope behind exact retained closeout and ancestry gates, then later implement a small COTS-backed deterministic planner for #5498 consumption.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Render, validate, and independently review all six cards, design, diagram, dependencies, protected paths, COTS, budgets, and PVF lanes",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed for #5340, #5341, #5342, and final gate #5349 merged typed closeout and ancestry before amending product scope",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the pure conductor component with COTS graph traversal, typed records, canonical ordering, and refusal contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and full proof, exact-revision review, typed publication, serialized merge, post-merge validation, and closeout",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked work on main
- no product implementation before #5349 and direct dependencies are merged typed closed_out
- preparation claim protects only issue-local lifecycle paths
- no Runtime v2 edits or dependencies
- no AWS, network, provider, or GitHub mutation
- no autonomous merge or closeout authority
- canonical deterministic output ordering

## Risks

- planning logic could accidentally become a second scheduler
- naive path-prefix comparison could admit overlapping writes
- unordered maps or wall-clock ids could break deterministic replay
- stale claim or card snapshots could produce unsafe assignments
- parallel work could bypass serialized review and integration gates
- the new component could regrow before its interface stabilizes

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5499/design.md

Digest: 808a27103da870697efd0b927a2dabc699c5bccc42a58f85d3bbf287743b86f4

## Diagram

.csdlc/prepared/issues/5499/diagram.mmd

Digest: f271df2d7413ad573a07bea66b13ffcbe5151f8c273d26aa9c64dc0ef426849e

## Stop Conditions

- a required retained closeout receipt is absent, non-merged, or not ancestral
- a planned product path overlaps another active typed claim
- input state is stale, incomplete, cyclic, ambiguous, or contains an unknown validation lane
- implementation requires task operations or lifecycle mutation
- the LoC, test, time, or direct-dependency budget is exceeded without reviewed typed exception

## Handoff

Proceed only after doctor readiness.
