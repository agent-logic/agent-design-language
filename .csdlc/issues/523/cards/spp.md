# Structured Planning Prompt

Template: 1.0.0

Issue: 523

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the existing successor package with delivered v0.92.1 truth, repair only planning inconsistencies, validate the complete denominator, and publish one reviewed planning PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the exact delivered/deferred/open planning denominator at the post-#522 main revision.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Refresh the v0.92.2 package and feature routing without creating successor issues.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused consistency and denominator validation, obtain exact-head independent review, and publish one closing PR.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Planning claims are source-grounded
- Deferred work is never silently promoted
- One planned item owns one concrete result
- No successor issue creation

## Risks

- The existing package may be stale relative to late v0.92.1 merges
- Residual work may be duplicated or silently omitted
- Planning prose may overstate implementation

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/523/design.md

Digest: 41bf25be8f50e48deacef0581ef18014eded9d61a421decfe0c4ccc135427c42

## Diagram

.csdlc/prepared/issues/523/diagram.mmd

Digest: a3c4ef86476692975539357c4920e8b7795089ecd3296378f213d5aea41e89af

## Stop Conditions

- #522 lacks a reviewed merge
- Successor ownership is ambiguous
- A requested change would create or execute v0.92.2 work
- The denominator cannot be reconciled from tracked and live authority

## Handoff

Proceed only after doctor readiness.
