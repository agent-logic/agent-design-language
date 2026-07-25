# Structured Planning Prompt

Template: 1.0.0

Issue: 5666

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add a small policy plus contract test that turns the operator's ten speed improvements into enforceable ADL guidance for small fixes.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Write the proportional fast-lane policy",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Link routing docs, reference the selector policy, and add focused contract test",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate, review, publish, and close out",
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

- small fixes do not bypass typed lifecycle
- focused proof must be selected and truthful
- runtime/product changes remain full proof
- no waiting on unchanged GitHub states

## Risks

- fast lane could be mistaken for weaker proof
- policy-only work might not change behavior unless linked to contract tests
- local disk fallback could reappear if not explicitly forbidden

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5666/design.md

Digest: 17e9330bc8371335f7a79046ba2b93a9832c9bef55da76e3e1844f62d7610e3a

## Diagram

.csdlc/prepared/issues/5666/diagram.mmd

Digest: 436c059b2c761ddbf14fbd15316810433a64125802cd93ae85f734f58e3d592a

## Stop Conditions

- FastWork is unavailable
- scope expands into CI workflow rewrite or runtime/product code
- typed lifecycle state cannot be initialized or validated
- review finds unresolved actionables

## Handoff

Proceed only after doctor readiness.
