# Structured Planning Prompt

Template: 1.0.0

Issue: 87

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind issue 87, simplify the mathematically redundant ACIP range predicate, add focused boundary cases, prove strict Clippy, review exact head, and publish a ready closing PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Simplify the ACIP compatible-minor predicate without changing the inclusive-range contract.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add focused exact, wider, future-only, and malformed range tests beside the predicate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused tests and both issue-named warning-denied Clippy targets.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head independent review and publish one ready PR closing issue 87.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Protocol major matching remains exact
- The offered minor interval remains inclusive
- Malformed intervals fail closed
- Unknown required features still fail closed
- Child-owned files remain untouched

## Risks

- Removing a redundant comparison accidentally accepts a range not containing the local minor
- Tests cover only the current exact range and miss wider or malformed offers
- Named child test targets are not yet present on canonical main

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/87/design.md

Digest: 9b43fca9e625ea50ae7022dc0f8fcf2569ca96b3373db88e3395c234e0a33046

## Diagram

.csdlc/prepared/issues/87/diagram.mmd

Digest: a365daa41e2b95dcf7516c276d67acb6ea2465b4a4720a6b1868bbdd315a745f

## Stop Conditions

- The fix requires changing protocol constants or child-owned modules
- A named strict-Clippy target cannot be materialized from the Sprint 4 integration surface
- Focused tests reveal different intended negotiation semantics

## Handoff

Proceed only after doctor readiness.
