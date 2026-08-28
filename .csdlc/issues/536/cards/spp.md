# Structured Planning Prompt

Template: 1.0.0

Issue: 536

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize the exact child batch, validate design-time cards, then hand dependency-safe lanes to separate issue sessions and retain integrated closeout truth.

## Plan

Revision 3

## Steps

[
  {
    "id": "readiness",
    "action": "Initialize and validate the Sprint Execution Packet and all child design-time card bundles",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "coordinate",
    "action": "Route child sessions according to declared safe lanes and serial gates",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "review-close",
    "action": "Review integrated outcomes and close the umbrella only after terminal child truth",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Child issues retain all implementation and proof authority
- No child starts before declared dependencies
- No umbrella closeout substitutes for child closeout
- Parallel lanes use distinct child worktrees and goals
- Operator-controlled external actions remain blocked until explicitly authorized

## Risks

- Podcast publication work could cross child ownership boundaries
- Observatory implementation could begin before reviewed terminal #511
- Provider submission could occur without explicit operator authorization
- The umbrella could overstate completion while a child is nonterminal
- Observatory implementation could invent Runtime fields or use non-authentic routes

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/536/design.md

Digest: 59ac3fa2ba8d6bc8a2ab110577d928b452fe262db310c98fe2c6d80ae29e2ab9

## Diagram

.csdlc/prepared/issues/536/diagram.mmd

Digest: 8079306bf86fe585ac4b92901f08f9bc24ed7a3c84303f5eafa3f2c06664bb1a

## Stop Conditions

- Any overlapping child write ownership
- Any missing or generic child design-time card bundle
- Any required dependency absent from the packet
- Any request for credentials or irreversible provider action without explicit operator authority
- Any attempt to use the umbrella to implement child work

## Handoff

Proceed only after doctor readiness.
