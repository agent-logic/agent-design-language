# Structured Planning Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare #260 now as execution-ready initialized/ready state, hold before bind until #259 is terminal, then bind to a dedicated FastWork worktree and migrate only non-transport distributed Runtime callers.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify #258 terminal/ancestral evidence and #259 gate status immediately before bind.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Inventory non-transport distributed Runtime caller sites that still need governed adapter migration.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Migrate only #260-owned caller sites to the governed adapter facade.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused Runtime caller validation and record proof without claiming parent #203 integration.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- No #260 bind or source implementation occurs before #259 is terminal, reconciled, and ancestral.
- #260 consumes but does not weaken the #258 authority-store boundary.
- #260 consumes but does not replace the #259 transport authority result.
- Parent #203 final integration remains a later exact-revision proof after child terminal closure.

## Risks

- #259 may alter adapter names or transport-facing seams; #260 bind must resync to exact post-#259 main.
- Distributed Runtime callers may share helper utilities with #259; post-#259 inventory must separate transport from non-transport ownership.
- Focused validation selector names may require adjustment after #259 lands.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/260/design.md

Digest: 268386db6870d5e1a1b1895d982c18d936ca2dc41f958f6592f4b170d6f07dda

## Diagram

.csdlc/prepared/issues/260/diagram.mmd

Digest: eac1d44f45d2f05d26ce8e15561981a4e618d4cbdab648eab3a417750761d0c1

## Stop Conditions

- #259 is not terminal, reconciled, and ancestral when bind is requested.
- A candidate change would modify #258 authority-store boundary behavior.
- A candidate change would modify #259 governed transport behavior.
- A candidate change would require parent #203 integration scope.
- Focused Runtime validation fails or is non-proving without explicit rationale.
- Fresh exact-head review reports actionable findings.

## Handoff

Proceed only after doctor readiness.
