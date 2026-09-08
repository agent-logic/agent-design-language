# Structured Planning Prompt

Template: 1.0.0

Issue: 431

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inventory and validate the existing package, allocate the six authorized lanes plus #432 prerequisite, remove .adl dependencies, record Runtime v4 as risk, refresh the exact planning surfaces, review them, and hand the result to unchanged WP-28 plus v0.92.2 CodeFriend Beta 1.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory existing v0.92.1 documents and live tracked inputs; classify truth and readiness gaps.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-11"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Refresh the complete milestone package and global feature list for exact six-lane, #432, dependency, proof, and routing parity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-8",
      "AC-10",
      "AC-11"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused planning, no-.adl, scope, YAML, link, and review validation; resolve or route findings.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-11"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Produce explicit handoffs for unchanged WP-28 #316 and v0.92.2 CodeFriend Beta 1 through the v0.95 beta gate.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- WP-28 #316 remains unchanged
- Planning status never implies implementation or release approval
- Backlog candidates remain non-executable without operator promotion
- Unfinished v0.92 evidence remains explicitly pending

## Risks

- Existing package surfaces may disagree on old issue identifiers or dependency order
- Backlog candidates could be mistaken for approved execution
- Late v0.92 work may invalidate early assumptions
- The sidecar could accidentally duplicate WP-28 authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/431/design.md

Digest: 043d86ae2db56fd31bf3715eb7403c85ef25b9bec049884cee795ed9ed174b27

## Diagram

.csdlc/prepared/issues/431/diagram.mmd

Digest: eb2ffdd41815afe996828e271f23be1fc0970dcc549e995585d0c5caf930573e

## Stop Conditions

- Any edit requires changing WP-28 #316
- Any issue move or backlog promotion lacks explicit authority
- The package cannot be reconciled without unresolved architecture or release decisions
- Execution would require non-planning source changes

## Handoff

Proceed only after doctor readiness.
