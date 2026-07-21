# Structured Planning Prompt

Template: 1.0.0

Issue: 5346

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six cards; freeze exact terminal dependency and disjoint-manifest gates, eligibility schema, ownership, COTS, budgets, PVF, no-deferral, review, and serialized integration rules; obtain bounded preparation review and fix findings; commit and push preparation only; remain fail-closed until every dependency is terminal and ancestral; then execute only the exact reviewed manifest through the full typed lifecycle.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete all six typed cards, design, diagram, protected paths, COTS, budgets, PVF, executable preparation validation, and bounded review/fixes without deletion",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Wait read-only until #5344/#5343/#5358/#5361 are terminal and ancestral and #5346/#5347 reviewed manifests are disjoint",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Recompute the denominator, run csdlc-eligibility, amend the typed claim to exact eligible paths, and delete only approved manifest rows",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and complete post-deletion proof, budgets, consumer/link/workspace checks, and exact bounded review; fix every finding",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Publish through typed v2, shepherd green required CI, serialize merge with #5347, run post-merge proof, close out, retain the receipt, and release the claim",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- No product path is protected or mutated during preparation
- No deletion begins before all four terminal dependency receipts and ancestry predicates pass
- #5346 and #5347 manifests remain disjoint and every path has exactly one owner/disposition
- Existing csdlc-eligibility is the sole eligibility authority and broad glob deletion is forbidden
- Deleted, retained, and new LoC remain separate; below 80 percent deletion is never completion
- Every retained path has a named owner and justification
- Runtime v2 is categorically outside #5346 ownership and may not be edited or deleted by this issue
- Deletion merges and post-merge validation are serialized

## Risks

- A broad directory or glob can delete an unclassified or peer-owned path
- Manifest aliasing, symlinks, generated files, or Cargo membership can hide overlap between #5346 and #5347
- A stale receipt or non-ancestral merge can make deletion depend on evidence absent from the execution tree
- Denominator drift or mixed deleted/new LoC can overstate the reduction percentage
- Compatibility and rollback surfaces can be removed before the reviewed window expires
- Retained files can become ownerless duplicate authority
- Validation can pass locally while consumers, docs, demos, install, or selector paths still reference deleted code

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/5346/design.md

Digest: 14dfc6b19de9acb122b4054f1629ccbfc9fc685e74b83d9fe63cf990027ad402

## Diagram

.csdlc/prepared/issues/5346/diagram.mmd

Digest: 45d18f9c0ff8e43be2d9788cf90560f6658fa028d71cf514e0abdbe3fd18e0da

## Stop Conditions

- Any #5344/#5343/#5358/#5361 terminal receipt, claim release, merge state, or ancestry predicate is absent or contradictory
- The #5346 and #5347 manifests overlap or any path lacks exactly one disposition and owner
- csdlc-eligibility rejects the exact manifest or revision
- The rollback window or selector cutover evidence is incomplete
- The denominator cannot be reproduced or deletion would fall below 80 percent
- A retained path lacks an owner/justification or replacement proof is absent
- A product path cannot be claimed without collision or requires broad/glob deletion
- Any required validation would be deferred, skipped, host-bound, secret-bearing, stale, or replaced by metadata

## Handoff

Proceed only after doctor readiness.
