# Structured Planning Prompt

Template: 1.0.0

Issue: 175

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement authenticated independent exact-revision review, finding disposition, deterministic staleness, complete recovery invalidation, mode-bound publication authorization, and disjoint Closing/PartOf linkage gates.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define review/finding/principal/independence/staleness/linkage schemas and a typed override boundary without provider authority inflation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement exact-revision review recording, actionable finding disposition, substantive-head staleness, and publication fail-closed predicates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement review recover only from reviewed/published/merge_ready, with actor/reason/provenance and atomic invalidation before semantic correction.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Bind publication authorization to exact reviewed revision, normalized qualified target, and explicit Closing or PartOf mode; reject mixed/ambiguous relations.",
    "acceptance_ids": [
      "AC-5",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Exercise full review/publish/recover/correct/re-review/republish journeys for both linkage modes plus identity, stale head, missing review, and wrong-repository negatives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain exact fixtures and stop on stranded recovery, hidden findings, implicit linkage, same-principal bypass, or unknown revision approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Invalidate superseded review/publication artifacts, retain recovery provenance, remove scratch fixtures, and prove no stale authorization or unresolved actionable finding survives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-12 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/175/design.md

Digest: 8bc24bae8f58932e821e02b2832ced3ad708d345369fca5c4e78452f1eb99e6a

## Diagram

.csdlc/prepared/issues/175/diagram.mmd

Digest: 494f58b943a3eb6fdd062a1c893c937633ea6f09c30cf6a61f08e7f651b30684

## Stop Conditions

- Review can approve an unknown revision, actionable findings can be hidden, recovery can strand a record or leave dependent truth current, publication can bypass review, linkage mode is implicit or ambiguous, or provider identity is overstated.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
