# Structured Planning Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the issue-local packet now against `origin/main` `51bc5ae51b57c19dbab693af1c5a45142995f4e5`; later execution re-checks live merge and ancestry, then writes and validates the exact-revision handoff ledger.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Generate issue-local C-SDLC v2 cards, design, diagram, exact dependency register, COTS/tool boundary, budgets, PVF lanes, rollback criteria, and no-deferral rules",
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
    "id": "S2",
    "action": "Re-check live dependency merge plus ancestry before future implementation, using #5384, #5358, and #5361 accepted merges as current preparation inputs",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the exact-revision handoff ledger at docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md only after dependencies release execution",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact pre-PR review during later execution; keep preparation review/fix truth separate from publication authority",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- live merge plus ancestry is required before execution
- receipts are audit-only
- active claim reacquisition is not required for preparation and is deferred to execution
- preparation does not advance implementation state
- all claims remain exact-revision and evidence-bound

## Risks

- a currently closed dependency could regress or be superseded before later execution
- historical receipts could be mistaken for current ancestry
- handoff text could accidentally imply v0.92 implementation readiness
- a requested external gpt-5.5 lane could be unavailable; absence must be recorded rather than invented

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1800,
  "preparation_nonblank_line_budget": 350,
  "future_ledger_nonblank_line_budget": 300,
  "future_product_code_loc_budget": 0
}

## Design

.csdlc/prepared/issues/5352/design.md

Digest: af365531793d1cd3580017d467c2411dee21710b611baaf90034f5b513795570

## Diagram

.csdlc/prepared/issues/5352/diagram.mmd

Digest: 3f84cac3043146e0572c82b70c7ca49809181d9d9b1ccccb10bb4e3a5c7b62a0

## Stop Conditions

- #5384, #5358, or #5361 is not closed and ancestral on the execution-time origin/main
- required dependency merge is absent from current origin/main ancestry
- handoff evidence lacks exact revision or rollback truth
- scope pressure asks preparation to implement v0.92
- claim reacquisition, typed closeout receipt reconciliation, or stale lifecycle projection is treated as implementation authority

## Handoff

Proceed only after doctor readiness.
