# Structured Planning Prompt

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Derive the exact v0.92 issue universe, reconcile live immutable merge/review/check truth, construct an acyclic merge-gated action graph with asynchronous bookkeeping, exercise focused negatives, obtain exact-head review, and publish without merging.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify #316 merge ancestry and derive the canonical v0.92 denominator plus exact legacy-to-canonical provenance mapping.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Acquire retained live observations, validate every row deterministically, and construct the merge-gated asynchronous closeout DAG.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused negatives, hygiene, exact-head review, and closing-linked publication without merge or closure.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Reviewed green merge ancestry gates successor execution; closeout and cleanup do not.
- Every required issue appears exactly once with one owner and next action.
- The action graph remains acyclic.
- #317 performs no remote release or terminal mutation.

## Risks

- Historical issue mappings can duplicate current canonical issues.
- Stale GitHub or lifecycle observations can misclassify a row.
- Legacy closeout sequencing can accidentally reintroduce serialization.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/317/design.md

Digest: 3425e72e0b9744514acee71c52db381991ccd40ec156a06fa30859753cf43069

## Diagram

.csdlc/prepared/issues/317/diagram.mmd

Digest: 0607301dc50f658de41f7956070326444c9e76f6cce1e5d7cff98b518d701572

## Stop Conditions

- The canonical denominator cannot be derived exactly.
- Any row is duplicate, unknown, unowned, or has contradictory authority.
- The graph contains a cycle or makes closeout a successor gate.
- Exact-head review finds an actionable defect.

## Handoff

Proceed only after doctor readiness.
