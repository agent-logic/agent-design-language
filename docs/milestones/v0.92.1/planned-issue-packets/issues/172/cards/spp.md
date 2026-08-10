# Structured Planning Prompt

Template: 1.0.0

Issue: 172

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Deliver capability-matrix-driven semantic card operations and a strictly read-only doctor that diagnoses distinct blockers, enforces phase optionality, and always names a real next operation without treating Markdown as authority.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Generate or mechanically check card commands and semantic operations against the V3-01 capability matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement card show/edit over canonical semantic state with deterministic regeneration of every affected projection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement a typed read-only doctor finding registry for topology, schema, import, projection, lifecycle, and stranded-correction states.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement capability-derived next-operation selection and dedicated failure when no authorized correction path exists.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Exercise all six cards across phases, optional placeholders, projection drift/repair, unsupported imports, distinct blockers, and no-mutation doctor fixtures.",
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
    "id": "S6",
    "action": "Retain finding/repair proof and stop on direct Markdown edits, invented authority, mutation, or blocker collapse.",
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
    "id": "S7",
    "action": "Remove projection-repair scratch output and prove doctor remained read-only while semantic edits left every regenerated card digest-consistent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-10B owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/172/design.md

Digest: bb3c894b929857fe9db6ac66925dfe527f1b04d3978a8f2dd086e13bc13e254d

## Diagram

.csdlc/prepared/issues/172/diagram.mmd

Digest: a25d559891b2fe68fa9dc2bad26ea7ac166cb8c51f25c9e75c0bcaf138686c6e

## Stop Conditions

- Commands hand-edit rendered files, doctor mutates state, repair invents missing authority, or findings collapse distinct blockers.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
