# Structured Planning Prompt

Template: 1.0.0

Issue: 482

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the issue boundary, produce the single CORP-A deliverable, validate it, and obtain exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile dependencies and freeze the exact issue-local denominator.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Produce the bounded primary deliverable without widening authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run every planned PVF lane and retain bounded redacted evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head review and prepare a truthful publication handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- Issue completion is exactly acceptance of the one critical-asset schedule; source-specific checks are evidence inputs, not separately closeable results.
- The asset validator proves every critical asset appears exactly once with an accepted disposition and redacted receipt.
- No secret or private material in Git

## Risks

- Unknown critical-asset ownership
- Counsel or corporate authority is missing
- Private material would enter Git

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/482/design.md

Digest: e52268ade257af75c462783492e3afbb11fc5eec7eff7b0357d093c9f0882ac3

## Diagram

.csdlc/prepared/issues/482/diagram.mmd

Digest: 38c362aef42ad49d59732236139c96a15952738ff2c24c22745632eb34ef310a

## Stop Conditions

- Unknown critical-asset ownership
- Counsel or corporate authority is missing
- Private material would enter Git

## Handoff

Proceed only after doctor readiness.
