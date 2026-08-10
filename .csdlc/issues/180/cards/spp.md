# Structured Planning Prompt

Template: 1.0.0

Issue: 180

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After the approved rollback window, classify and remove every remaining v2 operational authority while preserving immutable historical evidence and proving a fresh v3-only install-to-clean lifecycle.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify the rollback window has expired, every retained record has a terminal disposition, and explicit operator removal approval exists.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory every v2 executable, skill, selector route, writable-state path, historical evidence path, and deletion candidate with one disposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Protect historical Gate/migration evidence and apply only the independently reviewed deletion manifest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Verify clean installation inventory contains no v2 executable, operator skill, selector route, or writable authority.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "From a fresh checkout, install v3 and complete validate, review, publish, finish, and clean without any v2 artifact.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain the final authority audit and stop on active rollback, stale eligibility, required v2 write, historical-evidence deletion, or absent approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Remove only classified v2 operational targets, preserve immutable historical evidence, clear retirement scratch output, and prove the fresh v3-only lifecycle leaves no v2 authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-R01 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/180/design.md

Digest: 117b63a309c397887ff7129b449cb367044813688db37b5a64eaf014dd5aee36

## Diagram

.csdlc/prepared/issues/180/diagram.mmd

Digest: bd2d2245ee90406daa4755f036adce2a7bbf3c6af1d0ed3878098fa763278a1f

## Stop Conditions

- The rollback window is active, any issue still requires v2 writes, eligibility evidence is stale, deletion touches historical evidence, or the operator has not explicitly approved removal.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
