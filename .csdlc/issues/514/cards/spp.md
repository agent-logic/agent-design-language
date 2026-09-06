# Structured Planning Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the issue boundary, produce the single PROV-A deliverable, validate it, and obtain exact-head review.

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
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run every planned PVF lane and retain bounded redacted evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head review and prepare a truthful publication handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Issue completion is exactly one shared provider-profile contract; provider-specific checks are evidence inputs.
- Schema, materialization, invalid-profile, last-known-good, and redaction checks pass.
- No secret or private material in Git

## Risks

- Tools require incompatible provider configuration
- Private material would enter a profile

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/514/design.md

Digest: 1ed0957e58b6ce64aeaee2096c6a9798aa51b5c9c4f3bf14bd0a8115bf64ef77

## Diagram

.csdlc/prepared/issues/514/diagram.mmd

Digest: 38c362aef42ad49d59732236139c96a15952738ff2c24c22745632eb34ef310a

## Stop Conditions

- Tools require incompatible provider configuration
- Private material would enter a profile

## Handoff

Proceed only after doctor readiness.
